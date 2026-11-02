//! `Havok SDK/hkpaabbtreeworldmanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpAabbTreeWorldManager` (named in the engine map; the Xbox PDB has no such class) keeps
//! the world's collidables in two trees: a kd-tree of the active ones and a dynamic AABB tree
//! of the inactive ones. Offsets, as read from the code:
//!
//! - `+0`: vtable; `+0x14`, `+0x18`, `+0x1c`, `+0x20`: four listener sub-objects (their vtables
//!   `010cd05c`, `010cd044`, `010cd034`, `010cd024`) registered with the world;
//! - `+0x8`: byte, `+0xc`: the inactive AABB tree, `+0x10`: the kd-tree;
//! - `+0x24`: byte mode flag (1: both trees, 0: the kd-tree holds everything), `+0x28`: the
//!   world, `+0x2c`: byte "built once", `+0x30`/`+0x34`: the projected-primitive array words
//!   the kd-tree build fills, `+0x3c`: a 16-byte block whose first word is set to 4 for the
//!   multi-threaded build, `+0x40`: float margin;
//! - `+0x44`, `+0x48`, `+0x4c`: the active collidable list (`hkArray`: data, size, capacity word
//!   whose low 30 bits are the capacity and whose top bit means "storage not owned");
//! - `+0x50`, `+0x54`, `+0x58`: an `hkMap` of pending collidables (slots of 8 bytes: key,
//!   value; an empty slot's key is `-1`; `+0x58` is the highest slot index);
//! - `+0x5c`: byte, `+0x60`: scan cursor, `+0x64`: scan budget (30), `+0x68`/`+0x69`: bytes
//!   ("updated", "inactive list valid"), `+0x6c`: an `hkSemaphore` pointer.
//!
//! The collidables the lists hold are `hkpEntity + 0x10` (`hkpCollidable`): word `+0` its
//! shape, `+8` its transform, byte `+0x10` the offset back to the owner, byte `+0x18` a
//! flag byte, and from `+0x20` an integer AABB.
//!
//! The functions write Havok's profiling records (`RDTSC` time stamps appended to the thread's
//! monitor stream, found with `TlsGetValue` of the index at `0126835c`); the engine has no
//! model of `RDTSC`, so the host clock stands in for the time stamps. SSE code computes lane
//! by lane in `f32`; `RCPPS` is the host's estimate.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its word `+0` is the stack allocator's top and its
/// field `+0x10` the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;

/// Profiling labels, with the strings they point at: `"LtCastRay"`, `"StActiveTree"`,
/// `"StInactiveTree"` and the shared end-of-section label `"lt"`.
const LABEL_CAST_RAY: u32 = 0x010c_d014;
const LABEL_ACTIVE_TREE_SPLIT: u32 = 0x010c_d004;
const LABEL_INACTIVE_TREE_SPLIT: u32 = 0x010c_cff4;
const LABEL_SPLIT_END: u32 = 0x0107_6f38;
/// `"LtBuildKdTreeMt"`, `"StPrepare"`, `"StBuild"`, `"LtbuildTreeDistributed"`, `"Stqueue"`,
/// `"Ststart"`, `"StwaitForCompletion"`, `"StwaitSemaphore"`, `"StfinishPpu"`, `"StFinalize"`.
const LABEL_BUILD_KD_TREE_MT: u32 = 0x010c_d130;
const LABEL_PREPARE: u32 = 0x010c_d124;
const LABEL_BUILD: u32 = 0x010c_d11c;
const LABEL_BUILD_TREE_DISTRIBUTED: u32 = 0x010c_d104;
const LABEL_QUEUE: u32 = 0x010c_d0fc;
const LABEL_START: u32 = 0x010c_d0f4;
const LABEL_WAIT_FOR_COMPLETION: u32 = 0x010c_d0e0;
const LABEL_WAIT_SEMAPHORE: u32 = 0x010c_d0d0;
const LABEL_FINISH_PPU: u32 = 0x010c_d0c4;
const LABEL_FINALIZE: u32 = 0x010c_d0b8;
/// `"Ttprepare"` and `"Ttbuild"`.
const LABEL_TREE_PREPARE: u32 = 0x010c_d0ac;
const LABEL_TREE_BUILD: u32 = 0x010c_d0a4;
/// `"LtIncrementalBuild"`, `"StClear Tree"`, `"StFind Inactivated"`,
/// `"StUpdate Inactive Tree"`, `"MiNum of Added Nodes"`, `"StFind Activated"`,
/// `"MiNum of Removed Nodes"`, `"StUpdate Active"`.
const LABEL_INCREMENTAL_BUILD: u32 = 0x010c_d200;
const LABEL_CLEAR_TREE: u32 = 0x010c_d1f0;
const LABEL_FIND_INACTIVATED: u32 = 0x010c_d1dc;
const LABEL_UPDATE_INACTIVE_TREE: u32 = 0x010c_d1c4;
const LABEL_ADDED_NODES: u32 = 0x010c_d1ac;
const LABEL_FIND_ACTIVATED: u32 = 0x010c_d198;
const LABEL_REMOVED_NODES: u32 = 0x010c_d180;
const LABEL_UPDATE_ACTIVE: u32 = 0x010c_d170;
/// `"LtDynamicBuild"`, `"StGather"`, `"StClear"`, `"StRebuild"`.
const LABEL_DYNAMIC_BUILD: u32 = 0x010c_d160;
const LABEL_GATHER: u32 = 0x010c_d154;
const LABEL_CLEAR: u32 = 0x010c_d14c;
const LABEL_REBUILD: u32 = 0x010c_d140;
/// `"LtActiveTree"`, `"LtInactiveTree"`, `"TtTreeBuild"` and the end label `"Et"`.
const LABEL_ACTIVE_TREE: u32 = 0x010c_d224;
const LABEL_INACTIVE_TREE: u32 = 0x010c_d214;
const LABEL_TREE_BUILD_TOTAL: u32 = 0x010c_d234;
const LABEL_END: u32 = 0x010c_41f8;

/// `free` (`0x00401030`, cdecl, `(block)`), used by the deleting destructors.
const FREE: u32 = 0x0040_1030;
/// The accessors of a collidable's broad-phase handle: `00d083e0` (`(collidable)`, returns
/// the 16-bit value) and `hkpWorldObject::setTreeData`-like `00d08400` (cdecl, `(collidable,
/// value)`).
const GET_TREE_DATA: u32 = 0x00d0_83e0;
const SET_TREE_DATA: u32 = 0x00d0_8400;
/// `hkMapBase::get` (`00cb1980`, `this` = map, `(key, out value)`, returns 0 when found),
/// `insert` (`00cb1870`, `this` = map, `(allocator, key, value)`), the slot clear
/// (`00cb1a60`, `this` = map), `clearAndDeallocate` (`00cb1a90`, `this` = map,
/// `(allocator)`), the map's destructor (`00a29680`, `this` = map) and `reserve`
/// (`00cb1fe0`, `this` = map, `(allocator, element count)`).
const MAP_GET: u32 = 0x00cb_1980;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_CLEAR: u32 = 0x00cb_1a60;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_DESTRUCT: u32 = 0x00a2_9680;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`) and
/// `_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `00c9bff0` (`this` = owner of the collidable, `(out byte)`): writes the owner's two-bit
/// activity state into the byte and returns the byte's address.
const OWNER_ACTIVITY_STATE: u32 = 0x00c9_bff0;
/// The world's four listener registrations (`this` = world, `(listener)`) and removals.
const WORLD_ADD_LISTENERS: [u32; 4] = [0x00c9_3820, 0x00c9_3860, 0x00c9_38a0, 0x00c9_3920];
const WORLD_REMOVE_LISTENERS: [u32; 4] = [0x00c9_17a0, 0x00c9_17f0, 0x00c9_1840, 0x00c9_18e0];
/// The semaphore's constructor (`00d04350`, `this` = block, `(initial, maximum)`), destructor
/// (`00d04370`) and the wait (`00d04380`); all take `this` only apart from the constructor.
const SEMAPHORE_CONSTRUCT: u32 = 0x00d0_4350;
const SEMAPHORE_DESTRUCT: u32 = 0x00d0_4370;
const SEMAPHORE_WAIT: u32 = 0x00d0_4380;
/// The AABB tree's constructor (`00d9c180`, `this` = block, `(min, max, scale)` of the world's
/// broad-phase bounds), the insertion of an AABB (`00d9c610`, `this` = tree, `(aabb,
/// collidable)`, returns the 16-bit handle), the removal of a handle (`00d9c7d0`, `this` =
/// tree, `(handle)`), a whole-tree call (`00d9bed0`, `this` = tree) and the bulk insertion
/// (`00d9ce50`, `this` = tree, `(aabbs, collidables, count, out handles array)`).
const AABB_TREE_CONSTRUCT: u32 = 0x00d9_c180;
const AABB_TREE_INSERT: u32 = 0x00d9_c610;
const AABB_TREE_REMOVE: u32 = 0x00d9_c7d0;
const AABB_TREE_CALL: u32 = 0x00d9_bed0;
const AABB_TREE_INSERT_ALL: u32 = 0x00d9_ce50;
/// The kd-tree's destructor (`00d97850`), constructor (`00d97970`, `this` = block, `(info)`),
/// rebuild (`00d978d0`, `this` = tree, `(info)`) and the construction-info constructor
/// (`00d976d0`, `this` = info).
const KD_TREE_DESTRUCT: u32 = 0x00d9_7850;
const KD_TREE_CONSTRUCT: u32 = 0x00d9_7970;
const KD_TREE_RESET: u32 = 0x00d9_78d0;
const KD_INFO_CONSTRUCT: u32 = 0x00d9_76d0;
/// The projected-primitive source (`00d9df50`, `this` = object, `(array, world)`; `00d9e000`
/// returns the count) and the two cdecl build steps `00d9dab0` and `00d9db30`.
const SOURCE_CONSTRUCT: u32 = 0x00d9_df50;
const SOURCE_COUNT: u32 = 0x00d9_e000;
const PROJECT_PRIMITIVES: u32 = 0x00d9_dab0;
const BUILD_TREE: u32 = 0x00d9_db30;
/// The multi-threaded build's finish step (`00d9d950`, cdecl `(tree, count, block, block)`).
const FINISH_TREE: u32 = 0x00d9_d950;
/// The job queue's `00d04b50` (`this` = queue, `(job, 0)`) and `00d051b0` (`this` = queue).
const QUEUE_ADD_JOB: u32 = 0x00d0_4b50;
const QUEUE_FINISH: u32 = 0x00d0_51b0;
/// The stack-allocated query objects: `00d984b0` (kd-tree query, `this` = the query object,
/// `(tree, 7 words)`), `00d9a2a0` (the same for the AABB tree), the ray collector's
/// constructor `00d9a040`, the kd-tree ray cast `00d98ca0` (cdecl `(out, tree, from, to,
/// collector)`) and the AABB-tree ray cast `00d1bd00` (cdecl `(tree, from, to, collector,
/// fraction)`).
const KD_TREE_QUERY: u32 = 0x00d9_84b0;
const AABB_TREE_QUERY: u32 = 0x00d9_a2a0;
const RAY_COLLECTOR_CONSTRUCT: u32 = 0x00d9_a040;
const KD_TREE_CAST_RAY: u32 = 0x00d9_8ca0;
const AABB_TREE_CAST_RAY: u32 = 0x00d1_bd00;
/// The float the queries are given in the last word of their stack object (`0104da4c`), the
/// float `2^32` added to unsigned integers (`01099378`), the lane-3 constant (`01011d78`) and
/// the vector whose lane 3 completes the reciprocal (`010c7170`).
const QUERY_MARGIN: u32 = 0x0104_da4c;
const UNSIGNED_BIAS: u32 = 0x0109_9378;
const BOUNDS_W: u32 = 0x0101_1d78;
const RECIPROCAL_FILL: u32 = 0x010c_7170;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

/// The stand-in for `RDTSC`'s low word: the host clock.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// The thread's monitor stream (`TlsGetValue` of the index) and its cursor, when the stream
/// has room (cursor below the limit at `+0xc`).
fn monitor_cursor(e: &mut Engine) -> (u32, Option<u32>) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    let room = cursor < e.mem.u32(stream + 0xc);
    (stream, room.then_some(cursor))
}

/// Appends a 12-byte record `(label, time stamp)` to the monitor stream while its cursor (+4)
/// is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let (stream, cursor) = monitor_cursor(e);
    if let Some(cursor) = cursor {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// The 16-byte record `(first label, time stamp, -, second label)`. Returns the time stamp
/// when a record was written.
fn profile_pair_stamp(e: &mut Engine, first: u32, second: u32) -> Option<u32> {
    let (stream, cursor) = monitor_cursor(e);
    let cursor = cursor?;
    let stamp = time_stamp_low();
    e.mem.set_u32(cursor, first);
    e.mem.set_u32(cursor + 0xc, second);
    e.mem.set_u32(cursor + 4, stamp);
    e.mem.set_u32(stream + 4, cursor + 0x10);
    Some(stamp)
}

fn profile_pair(e: &mut Engine, first: u32, second: u32) {
    profile_pair_stamp(e, first, second);
}

/// The 8-byte counter record `(label, 1.0f)`, without a time stamp.
fn profile_counter(e: &mut Engine, label: u32) {
    let (stream, cursor) = monitor_cursor(e);
    if let Some(cursor) = cursor {
        e.mem.set_u32(cursor, label);
        e.mem.set_f32(cursor + 4, 1.0);
        e.mem.set_u32(stream + 4, cursor + 8);
    }
}

/// The heap allocator: field `+0x10` of the memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Appends `value` to the word `hkArray` at `array`: the memory router is asked first, and
/// when the size equals the capacity (low 30 bits of the word at `+8`) `_reserveMore` grows
/// the array by one element of 4 bytes.
fn array_push(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// A collidable's owner, the address `00c9bff0` is called on: the signed byte at `+0x10` added
/// to the collidable's address.
fn collidable_owner(e: &Engine, collidable: u32) -> u32 {
    let offset = e.mem.u8(collidable.wrapping_add(0x10)) as i8;
    collidable.wrapping_add(offset as i32 as u32)
}

/// A collidable's 16-bit tree data (`00d083e0`).
fn tree_data(e: &mut Engine, collidable: u32) -> u16 {
    e.call(GET_TREE_DATA, &args![collidable]).u32() as u16
}

/// Frees an `hkArray`'s storage through slot `0x10` of the heap allocator when the capacity
/// word's top bit says it is owned: `(data, (capacity & 0x3fffffff) * element size)`.
fn release_array(e: &mut Engine, allocator: u32, data: u32, capacity_word: u32, element: u32) {
    if capacity_word & 0x8000_0000 == 0 {
        let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// The scalar deleting destructor shape: writes `vtable`, frees `this` with `free` when bit 0
/// of `flags` is set, returns `this`.
fn deleting_with_free(e: &mut Engine, this: Ptr, flags: u8, vtable: u32) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00d08330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor (the engine map names the class `hkpWorldPostSimulationListener`;
/// unconfirmed): writes the vtable `010ccf7c` and frees `this` with `free` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn deleting_destructor_00d08330(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_with_free(e, this, flags, 0x010c_cf7c)
}

// Translated from 00d08350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor (named `hkKdTreeCastCollector` in the engine map; unconfirmed): writes
/// the vtable `010ccf8c` and frees `this` with `free` when bit 0 of `flags` is set.
pub fn deleting_destructor_00d08350(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_with_free(e, this, flags, 0x010c_cf8c)
}

// Translated from 00d0a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor (named `hkAabbTreeQueryUtils<...hkAabbTreeAabb...>::Collector` in the
/// engine map; unconfirmed): writes the vtable `010ccfa0` and frees `this` with `free` when
/// bit 0 of `flags` is set.
pub fn deleting_destructor_00d0a130(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_with_free(e, this, flags, 0x010c_cfa0)
}

// Translated from 00d0a150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor (the sibling collector of `00d0a130`, with the 16-bit AABB; unconfirmed):
/// writes the vtable `010ccfac` and frees `this` with `free` when bit 0 of `flags` is set.
pub fn deleting_destructor_00d0a150(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_with_free(e, this, flags, 0x010c_cfac)
}

// Translated from 00d0a170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor (`hkpKdTreeCollidableRaycaster` in the engine map; unconfirmed): writes
/// the vtable `010ccf8c`; when bit 0 of `flags` is set the `0x18`-byte block is returned to
/// the memory router's allocator (slot 8). Returns `this`.
pub fn deleting_destructor_00d0a170(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_cf8c);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00d08550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the "inactive list valid" byte (`+0x69`) and empties the pending map (`+0x50`) by
/// tail-calling the map's slot clear (`00cb1a60`); returns its result.
pub fn fn_00d08550(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u8(this.addr() + 0x69, 0);
    e.call(MAP_CLEAR, &args![this.addr() + 0x50]).u32()
}

// Translated from 00d08560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the active list's size (`+0x48`) and the bytes `+0x2c` and `+0x69`.
pub fn fn_00d08560(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x48, 0);
    e.mem.set_u8(this.addr() + 0x2c, 0);
    e.mem.set_u8(this.addr() + 0x69, 0);
}

// Translated from 00d08570 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the mode flag (`+0x24`) set: takes the collidable `object + 0x10` out of the tree
/// bookkeeping. Its tree data is read (`00d083e0`); when the collidable is in the active list
/// (`+0x44`, size `+0x48`) it is removed by moving the last element into its place;
/// otherwise a tree data other than `0xffff` is removed from the inactive AABB tree
/// (`00d9c7d0`, `this` = the tree at `+0xc`, `(handle)`). (The exe's extra test of the found
/// index against -1 cannot fire.)
pub fn fn_00d08570(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    let collidable = object.wrapping_add(0x10);
    if e.mem.u8(base + 0x24) == 0 {
        return;
    }
    let handle = tree_data(e, collidable);
    let count = e.mem.i32(base + 0x48);
    let mut found = None;
    if count > 0 {
        let data = e.mem.u32(base + 0x44);
        let mut index = 0i32;
        while index < count {
            if e.mem.u32(data.wrapping_add(4 * index as u32)) == collidable {
                found = Some(index);
                break;
            }
            index += 1;
        }
    }
    match found {
        Some(index) => {
            let size = e.mem.i32(base + 0x48).wrapping_sub(1);
            e.mem.set_i32(base + 0x48, size);
            if size != index {
                let data = e.mem.u32(base + 0x44);
                let last = e.mem.u32(data.wrapping_add(4 * size as u32));
                e.mem.set_u32(data.wrapping_add(4 * index as u32), last);
            }
        }
        None => {
            if handle != 0xffff {
                let tree = e.mem.u32(base + 0xc);
                e.call(AABB_TREE_REMOVE, &args![tree, u32::from(handle)]);
            }
        }
    }
}

// Translated from 00d089c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Listener thunk (`this` is the sub-object at `+0x14`): `00d08570` on the manager.
pub fn fn_00d089c0(e: &mut Engine, this: Ptr, object: u32) {
    fn_00d08570(e, Ptr::new(this.addr().wrapping_sub(0x14)), object);
}

// Translated from 00d089d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Listener thunk (`this` is the sub-object at `+0x14`): `00d08570` on the manager. Identical
/// to `00d089c0`.
pub fn fn_00d089d0(e: &mut Engine, this: Ptr, object: u32) {
    fn_00d08570(e, Ptr::new(this.addr().wrapping_sub(0x14)), object);
}

// Translated from 00d089e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Listener thunk (`this` is the sub-object at `+0x18`): `00d08570` on the manager.
pub fn fn_00d089e0(e: &mut Engine, this: Ptr, object: u32) {
    fn_00d08570(e, Ptr::new(this.addr().wrapping_sub(0x18)), object);
}

/// Puts `key` into the pending map (`+0x50`) with `value` when the map does not have it:
/// `hkMap::get` is asked with the key's own stack slot as the output, and a non-zero answer
/// (not found) is followed by `insert(allocator, key, value)`.
fn pending_insert_if_absent(e: &mut Engine, manager: u32, key: u32, value: u32) {
    let map = manager + 0x50;
    let missing = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), key);
        e.call(MAP_GET, &args![map, key, slot]).u32()
    });
    if missing != 0 {
        let allocator = router_allocator(e);
        e.call(MAP_INSERT, &args![map, allocator, key, value]);
    }
}

// Translated from 00d08940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records `key` in the pending map with the value 1 unless it is already there.
pub fn fn_00d08940(e: &mut Engine, this: Ptr, key: u32) {
    pending_insert_if_absent(e, this.addr(), key, 1);
}

// Translated from 00d08980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records `key` in the pending map with the value -1 unless it is already there.
pub fn fn_00d08980(e: &mut Engine, this: Ptr, key: u32) {
    pending_insert_if_absent(e, this.addr(), key, 0xffff_ffff);
}

/// The island loop of `00d089f0` / `00d08a40`: with the mode flag (`this + 8`, the sub-object
/// sits at manager `+0x1c`) set, marks the bytes `+0x68` / `+0x69` of the manager and records
/// every entity of the island (list `+0x48`, size `+0x4c`, re-read each round) whose
/// collidable has the flag byte `+0x18` equal to 1, through `record`.
fn record_island_collidables(
    e: &mut Engine,
    this: Ptr,
    island: u32,
    record: fn(&mut Engine, Ptr, u32),
) {
    let base = this.addr();
    if e.mem.u8(base + 8) == 0 {
        return;
    }
    e.mem.set_u8(base + 0x4d, 1);
    e.mem.set_u8(base + 0x4c, 0);
    let manager = Ptr::new(base.wrapping_sub(0x1c));
    let mut index = 0i32;
    while index < e.mem.i32(island + 0x4c) {
        let list = e.mem.u32(island + 0x48);
        let collidable = e
            .mem
            .u32(list.wrapping_add(4 * index as u32))
            .wrapping_add(0x10);
        if e.mem.u8(collidable + 0x18) == 1 {
            record(e, manager, collidable);
        }
        index += 1;
    }
}

// Translated from 00d089f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Island listener (`this` is the sub-object at `+0x1c`): see [`record_island_collidables`],
/// recording with the value -1 (`00d08980`).
pub fn fn_00d089f0(e: &mut Engine, this: Ptr, island: u32) {
    record_island_collidables(e, this, island, fn_00d08980);
}

// Translated from 00d08a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Island listener (`this` is the sub-object at `+0x1c`): see [`record_island_collidables`],
/// recording with the value 1 (`00d08940`).
pub fn fn_00d08a40(e: &mut Engine, this: Ptr, island: u32) {
    record_island_collidables(e, this, island, fn_00d08940);
}

// Translated from 00d08a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entity listener (`this` is the sub-object at `+0x20`): with the mode flag (`this + 4`)
/// set, marks the manager's bytes `+0x69` = 1 and `+0x68` = 0 and records the collidable
/// `entity + 0x10` with the value 1 (`00d08940`).
pub fn fn_00d08a90(e: &mut Engine, this: Ptr, entity: u32) {
    let base = this.addr();
    if e.mem.u8(base + 4) == 0 {
        return;
    }
    e.mem.set_u8(base + 0x49, 1);
    e.mem.set_u8(base + 0x48, 0);
    fn_00d08940(
        e,
        Ptr::new(base.wrapping_sub(0x20)),
        entity.wrapping_add(0x10),
    );
}

// Translated from 00d08c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::pushBackToActiveList` (named in the engine map): with the mode
/// flag (`+0x24`) set, clears the collidable's tree data (`00d08400(object, 0xffff)`) and
/// appends it to the active list at `+0x44`.
pub fn push_back_to_active_list(e: &mut Engine, this: Ptr, object: u32) {
    if e.mem.u8(this.addr() + 0x24) == 0 {
        return;
    }
    e.call(SET_TREE_DATA, &args![object, 0xffffu32]);
    array_push(e, this.addr() + 0x44, object);
}

// Translated from 00d08fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Listener thunk (`this` is the sub-object at `+0x14`): marks the manager's bytes `+0x69` = 1
/// and `+0x68` = 0, then `pushBackToActiveList(manager, object + 0x10)`.
pub fn fn_00d08fb0(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 0x55, 1);
    e.mem.set_u8(base + 0x54, 0);
    push_back_to_active_list(
        e,
        Ptr::new(base.wrapping_sub(0x14)),
        object.wrapping_add(0x10),
    );
}

// Translated from 00d08fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Listener thunk (`this` is the sub-object at `+0x18`): marks the manager's bytes `+0x69` = 1
/// and `+0x68` = 0, then `pushBackToActiveList(manager, object + 0x10)`.
pub fn fn_00d08fd0(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 0x51, 1);
    e.mem.set_u8(base + 0x50, 0);
    push_back_to_active_list(
        e,
        Ptr::new(base.wrapping_sub(0x18)),
        object.wrapping_add(0x10),
    );
}

// Translated from 00d08ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gathers the collidables of the world's active islands into the word array (cdecl `(array,
/// world)`): every entity of every island (islands list `world + 0x28`, count `+0x2c`; each
/// island's entities list `+0x48`, count `+0x4c`; island and counts are re-read each round)
/// contributes `entity + 0x10`; then every entry of the list at `world + 0x110` (count
/// `+0x114`) whose shape word (`entity + 0x10`) is not null.
pub fn fn_00d08ac0(e: &mut Engine, array: u32, world: u32) {
    let island_at = |e: &Engine, index: i32| {
        let islands = e.mem.u32(world + 0x28);
        e.mem.u32(islands.wrapping_add(4 * index as u32))
    };
    let mut island_index = 0i32;
    while island_index < e.mem.i32(world + 0x2c) {
        let mut island = island_at(e, island_index);
        let mut index = 0i32;
        while index < e.mem.i32(island + 0x4c) {
            let list = e.mem.u32(island + 0x48);
            let collidable = e
                .mem
                .u32(list.wrapping_add(4 * index as u32))
                .wrapping_add(0x10);
            array_push(e, array, collidable);
            island = island_at(e, island_index);
            index += 1;
        }
        island_index += 1;
    }
    let mut index = 0i32;
    while index < e.mem.i32(world + 0x114) {
        let list = e.mem.u32(world + 0x110);
        let entity = e.mem.u32(list.wrapping_add(4 * index as u32));
        let collidable = entity.wrapping_add(0x10);
        if e.mem.u32(collidable) != 0 {
            array_push(e, array, collidable);
        }
        index += 1;
    }
}

// Translated from 00d08b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gathers the collidables of the world's inactive islands into the word array (cdecl `(array,
/// world)`): every entity of every island of the list at `world + 0x34` (count `+0x38`), then
/// every entity of the fixed island (`world + 0x20`) except the one at `world + 0x24`;
/// each contributes `entity + 0x10`.
pub fn fn_00d08b90(e: &mut Engine, array: u32, world: u32) {
    let island_at = |e: &Engine, index: i32| {
        let islands = e.mem.u32(world + 0x34);
        e.mem.u32(islands.wrapping_add(4 * index as u32))
    };
    let mut island_index = 0i32;
    while island_index < e.mem.i32(world + 0x38) {
        let mut island = island_at(e, island_index);
        let mut index = 0i32;
        while index < e.mem.i32(island + 0x4c) {
            let list = e.mem.u32(island + 0x48);
            let collidable = e
                .mem
                .u32(list.wrapping_add(4 * index as u32))
                .wrapping_add(0x10);
            array_push(e, array, collidable);
            island = island_at(e, island_index);
            index += 1;
        }
        island_index += 1;
    }
    let fixed_island = e.mem.u32(world + 0x20);
    let skipped = e.mem.u32(world + 0x24);
    let mut index = 0i32;
    while index < e.mem.i32(fixed_island + 0x4c) {
        let list = e.mem.u32(fixed_island + 0x48);
        let entity = e.mem.u32(list.wrapping_add(4 * index as u32));
        if entity != skipped {
            array_push(e, array, entity.wrapping_add(0x10));
        }
        index += 1;
    }
}

// Translated from 00d08ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gathers every collidable of the world into the word array (cdecl `(array, world)`): the
/// size is reset to 0, then `00d08ac0` and `00d08b90`.
pub fn fn_00d08ff0(e: &mut Engine, array: u32, world: u32) {
    e.mem.set_u32(array + 4, 0);
    fn_00d08ac0(e, array, world);
    fn_00d08b90(e, array, world);
}

/// Whether `collidable` goes inactive: its flag byte `+0x18` is 1, its owner address is not
/// zero, and `00c9bff0` (given the stack byte) reports the state 0.
fn is_inactivated(e: &mut Engine, collidable: u32) -> bool {
    if e.mem.u8(collidable.wrapping_add(0x18)) != 1 {
        return false;
    }
    let owner = collidable_owner(e, collidable);
    if owner == 0 {
        return false;
    }
    e.with_stack(4, |e, slot| {
        let answer = e.call(OWNER_ACTIVITY_STATE, &args![owner, slot]).u32();
        e.mem.u8(answer) == 0
    })
}

/// Moves the active-list entry at `index` into the output array (`out`: data, size; no
/// growth), shrinks the active list and fills the hole with the last element.
fn move_to_inactivated(e: &mut Engine, base: u32, out: u32, index: i32) {
    let list = e.mem.u32(base + 0x44);
    let value = e.mem.u32(list.wrapping_add(4 * index as u32));
    let out_size = e.mem.u32(out + 4);
    let out_data = e.mem.u32(out);
    e.mem
        .set_u32(out_data.wrapping_add(out_size.wrapping_mul(4)), value);
    e.mem.set_u32(out + 4, out_size.wrapping_add(1));
    let size = e.mem.i32(base + 0x48).wrapping_sub(1);
    e.mem.set_i32(base + 0x48, size);
    if size != index {
        let list = e.mem.u32(base + 0x44);
        let last = e.mem.u32(list.wrapping_add(4 * size as u32));
        e.mem.set_u32(list.wrapping_add(4 * index as u32), last);
    }
}

// Translated from 00d08800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::scanActiveListAndGetInactivated` (named in the engine map): moves
/// the collidables of the active list (`+0x44`) that have gone inactive into `out` (data at
/// `+0`, size at `+4`, written without growing). With the byte `+0x5c` set the whole list is
/// scanned from the back and the answer is true. Otherwise at most `+0x64` entries are
/// scanned downward from the cursor `+0x60` (clamped to the last index; when it runs below 0
/// it restarts at the list size), and the answer is whether the number of entries scanned
/// equals the list size afterwards.
pub fn scan_active_list_and_get_inactivated(e: &mut Engine, this: Ptr, out: u32) -> bool {
    let base = this.addr();
    if e.mem.u8(base + 0x5c) != 0 {
        let mut index = e.mem.i32(base + 0x48).wrapping_sub(1);
        while index >= 0 {
            let list = e.mem.u32(base + 0x44);
            let collidable = e.mem.u32(list.wrapping_add(4 * index as u32));
            if is_inactivated(e, collidable) {
                move_to_inactivated(e, base, out, index);
            }
            index = index.wrapping_sub(1);
        }
        return true;
    }
    let last = e.mem.i32(base + 0x48).wrapping_sub(1);
    let cursor = e.mem.i32(base + 0x60);
    e.mem
        .set_i32(base + 0x60, if cursor >= last { last } else { cursor });
    let mut scanned = 0i32;
    if e.mem.i32(base + 0x64) > 0 {
        let mut round = 0i32;
        loop {
            let index = e.mem.i32(base + 0x60);
            e.mem.set_i32(base + 0x60, index.wrapping_sub(1));
            if index < 0 {
                let size = e.mem.u32(base + 0x48);
                e.mem.set_u32(base + 0x60, size);
                break;
            }
            let list = e.mem.u32(base + 0x44);
            let collidable = e.mem.u32(list.wrapping_add(4 * index as u32));
            if is_inactivated(e, collidable) {
                move_to_inactivated(e, base, out, index);
            }
            scanned += 1;
            round += 1;
            if round >= e.mem.i32(base + 0x64) {
                break;
            }
        }
    }
    scanned == e.mem.i32(base + 0x48)
}

// Translated from 00d08740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a query on both trees with the same seven words: with a kd-tree (`+0x10`) a `0x90`-byte
/// stack object (vtable `010ccfcc`, word `+0x24` = 0, float `+0x80` = the quieted float at
/// `0104da4c`) is the `this` of `00d984b0(tree, words...)`; with the mode flag (`+0x24`) set
/// the same kind of object (vtables `010ccfec` and `010ccfe0` at `+0` and `+4`) is the
/// `this` of `00d9a2a0(inactive tree at +0xc, words...)`. The two objects share the
/// stack block. The words' meaning is unconfirmed.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d08740(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
) {
    let base = this.addr();
    e.with_stack(0x90, |e, object| {
        let q = object.addr();
        let tree = e.mem.u32(base + 0x10);
        if tree != 0 {
            let margin = x87_quiet_bits(e.mem.u32(QUERY_MARGIN));
            e.mem.set_u32(q, 0x010c_cfcc);
            e.mem.set_u32(q + 0x24, 0);
            e.mem.set_u32(q + 0x80, margin);
            e.call(
                KD_TREE_QUERY,
                &args![q, tree, word_1, word_2, word_3, word_4, word_5, word_6, word_7],
            );
        }
        if e.mem.u8(base + 0x24) != 0 {
            let margin = x87_quiet_bits(e.mem.u32(QUERY_MARGIN));
            e.mem.set_u32(q + 0x80, margin);
            let inactive_tree = e.mem.u32(base + 0xc);
            e.mem.set_u32(q, 0x010c_cfec);
            e.mem.set_u32(q + 4, 0x010c_cfe0);
            e.mem.set_u32(q + 0x24, 0);
            e.call(
                AABB_TREE_QUERY,
                &args![
                    q,
                    inactive_tree,
                    word_1,
                    word_2,
                    word_3,
                    word_4,
                    word_5,
                    word_6,
                    word_7
                ],
            );
        }
    });
}

// Translated from 00d085e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::castRay` (named in the engine map): `ray` has the start at `+0`
/// and the end at `+0x10`; `filter_source` (may be 0) has the two pointers the collectors
/// take (`+0x14` and `+0x10`); `collector` receives the hits. A ray collector is built on the
/// stack (`00d9a040`; `+8` = ray, `+0xc` = `filter_source + 0x14` or 0, `+0x10` =
/// `filter_source + 0x10` or 0, `+0x14` = `collector`). Records `LtCastRay` / `StActiveTree`;
/// with a kd-tree (`+0x10`) an object (vtable `010ccfb8`, `+4` ray, `+8`/`+0xc` as the
/// collector's, `+0x10` collector) is given to the kd-tree ray cast
/// (`00d98ca0(out, tree, ray, ray + 0x10, object)`); the `out` word is the exe's stack slot of
/// the `collector` argument, which the first record overwrote with its time stamp when it
/// was written. Then `StInactiveTree` and, with the mode flag (`+0x24`) set, the AABB-tree cast
/// (`00d1bd00(tree at +0xc, ray, ray + 0x10, collector + 4, 1.0f)`), and the end record.
pub fn cast_ray(e: &mut Engine, this: Ptr, ray: u32, filter_source: u32, collector: u32) {
    let base = this.addr();
    e.with_stack(0x44, |e, frame| {
        let object = frame.addr();
        let ray_collector = object + 0x20;
        let out = object + 0x40;
        e.mem.set_u32(out, collector);
        e.call(RAY_COLLECTOR_CONSTRUCT, &args![ray_collector]);
        let (second, first) = if filter_source != 0 {
            (filter_source + 0x14, filter_source + 0x10)
        } else {
            (0, 0)
        };
        e.mem.set_u32(ray_collector + 0x14, collector);
        e.mem.set_u32(ray_collector + 8, ray);
        e.mem.set_u32(ray_collector + 0xc, second);
        e.mem.set_u32(ray_collector + 0x10, first);
        if let Some(stamp) = profile_pair_stamp(e, LABEL_CAST_RAY, LABEL_ACTIVE_TREE_SPLIT) {
            e.mem.set_u32(out, stamp);
        }
        let tree = e.mem.u32(base + 0x10);
        if tree != 0 {
            e.mem.set_u32(object, 0x010c_cfb8);
            e.mem.set_u32(object + 0x14, 0);
            e.mem.set_u32(object + 0x10, collector);
            e.mem.set_u32(object + 4, ray);
            e.mem.set_u32(object + 8, second);
            e.mem.set_u32(object + 0xc, first);
            e.call(KD_TREE_CAST_RAY, &args![out, tree, ray, ray + 0x10, object]);
        }
        profile_record(e, LABEL_INACTIVE_TREE_SPLIT);
        if e.mem.u8(base + 0x24) != 0 {
            let inactive_tree = e.mem.u32(base + 0xc);
            e.call(
                AABB_TREE_CAST_RAY,
                &args![
                    inactive_tree,
                    ray,
                    ray + 0x10,
                    ray_collector + 4,
                    1.0f32.to_bits()
                ],
            );
        }
        profile_record(e, LABEL_SPLIT_END);
    });
}

// Translated from 00d08cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::hkpAabbTreeWorldManager` (named in the engine map): `(world,
/// mode, margin)`. Sets the vtables (first the base ones, then `010cd078` and the four
/// listeners'), clears the lists and the map (reserved with `00cb1fe0(allocator, 0)`), takes
/// a 16-byte block (`+0x3c`) and the 4-byte semaphore block (constructed with `(0, 1000)`;
/// `+0x6c`), stores the margin through the x87 (`+0x40`), the mode flag (`+0x24`: 1 for mode
/// 0, 0 for mode 1, untouched otherwise), registers the four listeners with the world and,
/// with the flag set, builds the `0x80`-byte inactive AABB tree from the broad-phase bounds
/// (`world + 0x78`, `+0x20`, `+0x30`, `+0x40`). Finally the scan state (`+0x5c`, `+0x68`,
/// `+0x60`, `+0x64` = 30) and `00d08560`. Returns `this`.
pub fn hkp_aabb_tree_world_manager_construct(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    mode: u32,
    margin: f32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 0x14, 0x0102_f13c);
    e.mem.set_u32(base + 0x18, 0x0104_e3d0);
    e.mem.set_u32(base + 0x1c, 0x0104_de10);
    e.mem.set_u32(base + 0x20, 0x010c_cf7c);
    e.mem.set_u32(base, 0x010c_d078);
    e.mem.set_u32(base + 0x14, 0x010c_d05c);
    e.mem.set_u32(base + 0x18, 0x010c_d044);
    e.mem.set_u32(base + 0x1c, 0x010c_d034);
    e.mem.set_u32(base + 0x20, 0x010c_d024);
    e.mem.set_u32(base + 0x28, world);
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 0x8000_0000);
    e.mem.set_u32(base + 0x50, 0);
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x58, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(MAP_RESERVE, &args![base + 0x50, allocator, 0u32]);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x34, 0);
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x10u32]).u32();
    e.mem.set_u32(base + 0x3c, block);
    let allocator = router_allocator(e);
    let semaphore_block = e.vcall(allocator, 4, &args![4u32]).u32();
    let semaphore = if semaphore_block != 0 {
        e.call(SEMAPHORE_CONSTRUCT, &args![semaphore_block, 0u32, 1000u32])
            .u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x6c, semaphore);
    e.mem.set_u32(base + 0x40, x87_quiet_bits(margin.to_bits()));
    e.mem.set_u32(base + 8, 1);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0xc, 0);
    if mode == 0 {
        e.mem.set_u8(base + 0x24, 1);
    } else if mode == 1 {
        e.mem.set_u8(base + 0x24, 0);
    }
    for (index, add) in WORLD_ADD_LISTENERS.into_iter().enumerate() {
        let world = e.mem.u32(base + 0x28);
        e.call(add, &args![world, base + 0x14 + 4 * index as u32]);
    }
    if e.mem.u8(base + 0x24) != 0 {
        let world = e.mem.u32(base + 0x28);
        let bounds = e.mem.u32(world + 0x78).wrapping_add(0x20);
        let allocator = router_allocator(e);
        let block = e.vcall(allocator, 4, &args![0x80u32]).u32();
        let tree = if block != 0 {
            e.call(
                AABB_TREE_CONSTRUCT,
                &args![block, bounds, bounds + 0x10, bounds + 0x20],
            )
            .u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0xc, tree);
    }
    e.mem.set_u8(base + 0x5c, 0);
    e.mem.set_u8(base + 0x68, 0);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 0x1e);
    fn_00d08560(e, this);
    this
}

// Translated from 00d08e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::~hkpAabbTreeWorldManager` (named in the engine map): writes the
/// vtables back, removes the four listeners from the world, destroys the inactive tree
/// (virtual slot 0 with 1), the kd-tree (destructor, then its `0x50` bytes are freed through
/// slot 8), frees the 16-byte block, the semaphore (destructor, then 4 bytes), clears and
/// deallocates the map and frees the active list's storage when owned (slot `0x10`), and
/// writes the base vtables.
pub fn hkp_aabb_tree_world_manager_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_d078);
    e.mem.set_u32(base + 0x14, 0x010c_d05c);
    e.mem.set_u32(base + 0x18, 0x010c_d044);
    e.mem.set_u32(base + 0x1c, 0x010c_d034);
    e.mem.set_u32(base + 0x20, 0x010c_d024);
    for (index, remove) in WORLD_REMOVE_LISTENERS.into_iter().enumerate() {
        let world = e.mem.u32(base + 0x28);
        e.call(remove, &args![world, base + 0x14 + 4 * index as u32]);
    }
    let inactive_tree = e.mem.u32(base + 0xc);
    if inactive_tree != 0 {
        e.vcall(inactive_tree, 0, &args![1u32]);
    }
    let kd_tree = e.mem.u32(base + 0x10);
    if kd_tree != 0 {
        e.call(KD_TREE_DESTRUCT, &args![kd_tree]);
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![kd_tree, 0x50u32]);
    }
    let block = e.mem.u32(base + 0x3c);
    let allocator = router_allocator(e);
    e.vcall(allocator, 8, &args![block, 0x10u32]);
    let semaphore = e.mem.u32(base + 0x6c);
    if semaphore != 0 {
        e.call(SEMAPHORE_DESTRUCT, &args![semaphore]);
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![semaphore, 4u32]);
    }
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![base + 0x50, allocator]);
    e.call(MAP_DESTRUCT, &args![base + 0x50]);
    let allocator = router_allocator(e);
    let capacity_word = e.mem.u32(base + 0x4c);
    e.mem.set_u32(base + 0x48, 0);
    let data = e.mem.u32(base + 0x44);
    release_array(e, allocator, data, capacity_word, 4);
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x4c, 0x8000_0000);
    e.mem.set_u32(base + 0x20, 0x010c_cf7c);
    e.mem.set_u32(base + 0x1c, 0x0104_de10);
    e.mem.set_u32(base, 0x0102_e388);
    e.mem.set_u32(base + 0x18, 0x0104_e3d0);
    e.mem.set_u32(base + 0x14, 0x0102_f13c);
}

/// Creates the kd-tree in `slot` from the construction info, or resets the existing one:
/// `00d978d0(tree, info)`; otherwise a `0x50`-byte block from the heap allocator (slot 4)
/// constructed with `00d97970(block, info)` (0 when the allocation failed) is stored.
fn create_or_reset_kd_tree(e: &mut Engine, slot: u32, info: u32) {
    let tree = e.mem.u32(slot);
    if tree != 0 {
        e.call(KD_TREE_RESET, &args![tree, info]);
    } else {
        let allocator = router_allocator(e);
        let block = e.vcall(allocator, 4, &args![0x50u32]).u32();
        let tree = if block != 0 {
            e.call(KD_TREE_CONSTRUCT, &args![block, info]).u32()
        } else {
            0
        };
        e.mem.set_u32(slot, tree);
    }
}

/// Takes `0x10` bytes from the heap allocator with the size-returning slot (`0xc`, the byte
/// count in a stack word the allocator updates to the granted size) and returns the block
/// and the capacity word `granted / 4` (truncating toward zero; 0 becomes `0x80000000`).
fn take_scratch_block(e: &mut Engine, allocator: u32, size_word: u32) -> (u32, u32) {
    e.mem.set_u32(size_word, 0x10);
    let block = e.vcall(allocator, 0xc, &args![size_word]).u32();
    let granted = e.mem.i32(size_word);
    let capacity = granted / 4;
    (
        block,
        if capacity == 0 {
            0x8000_0000
        } else {
            capacity as u32
        },
    )
}

// Translated from 00d094a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::buildKdTree` (named in the engine map): builds the kd-tree stored
/// in `kd_slot` from the collidable array (`array`: data, size) of `world`. Without a job
/// queue or thread pool, or with fewer than 32 collidables, the build runs on this thread: a
/// source object over the array and world (`00d9df50`, count from `00d9e000`), the
/// construction info (`00d976d0`; `+0` count, byte `+5` = 0; the world's object at `+0x58`
/// fills the info's vectors through its slot `0x40`), the tree created or reset, then the
/// cdecl `00d9dab0(source, 0, count, this[+0x30])` and `00d9db30(this + 0x30, tree)`.
/// Otherwise it runs distributed: the same info (byte `+5` = 1), the block at `this[+0x3c]`
/// has its first word set to 4, two scratch blocks are taken from the allocator, a job
/// record (`0x60` bytes: bytes 6, 3, 1, `0x60` and `0xffff` as words at `+4` / `+6`, then the
/// semaphore, the block, the world's data at `world + 0x78`, the primitive words, the
/// tree's fields and the scratch blocks) is added to the queue (`00d04b50(queue, job, 0)`),
/// the thread pool's slot `0xc` is called with `(queue, 3)`, the queue finished
/// (`00d051b0`), the pool's slot `0x10` called, the semaphore waited on (`00d04380`), the
/// tree finished (cdecl `00d9d950(tree, count, block 1, block 2)`, read back from the job
/// record) and the scratch blocks freed. Each phase writes a profiling record. The job
/// record's words the exe leaves unwritten stay zero here.
pub fn build_kd_tree(
    e: &mut Engine,
    this: Ptr,
    kd_slot: u32,
    array: u32,
    world: u32,
    job_queue: u32,
    thread_pool: u32,
) {
    let base = this.addr();
    let distributed = job_queue != 0 && thread_pool != 0 && e.mem.i32(array + 4) >= 0x20;
    e.with_stack(0xd0, |e, frame| {
        let b = frame.addr();
        if !distributed {
            e.call(SOURCE_CONSTRUCT, &args![b + 0xc4, array, world]);
            let count = e.call(SOURCE_COUNT, &args![b + 0xc4]).u32();
            e.call(KD_INFO_CONSTRUCT, &args![b + 0x20]);
            let world_object = e.mem.u32(world + 0x58);
            e.mem.set_u32(b + 0x20, count);
            e.mem.set_u8(b + 0x25, 0);
            e.vcall(world_object, 0x40, &args![b + 0x30, b + 0x40]);
            create_or_reset_kd_tree(e, kd_slot, b + 0x20);
            let words = e.mem.u32(base + 0x30);
            e.call(PROJECT_PRIMITIVES, &args![b + 0xc4, 0u32, count, words]);
            let tree = e.mem.u32(kd_slot);
            e.call(BUILD_TREE, &args![base + 0x30, tree]);
            return;
        }
        profile_pair(e, LABEL_BUILD_KD_TREE_MT, LABEL_PREPARE);
        let count = e.mem.u32(array + 4);
        e.mem.set_u32(b + 0x54, count);
        e.call(KD_INFO_CONSTRUCT, &args![b + 0x20]);
        let world_object = e.mem.u32(world + 0x58);
        e.mem.set_u32(b + 0x20, count);
        e.mem.set_u8(b + 0x25, 1);
        e.vcall(world_object, 0x40, &args![b + 0x30, b + 0x40]);
        create_or_reset_kd_tree(e, kd_slot, b + 0x20);
        let sync_block = e.mem.u32(base + 0x3c);
        e.mem.set_u32(sync_block, 4);
        let allocator = router_allocator(e);
        let (block_1, capacity_1) = take_scratch_block(e, allocator, b + 0x10);
        e.mem.set_u32(b + 0x58, block_1);
        e.mem.set_u32(b + 0x14, capacity_1);
        let allocator = router_allocator(e);
        let (block_2, capacity_2) = take_scratch_block(e, allocator, b + 0x1c);
        e.mem.set_u32(b + 0x5c, block_2);
        e.mem.set_u32(b + 0x18, capacity_2);
        let job = b + 0x60;
        let tree = e.mem.u32(kd_slot);
        e.mem.set_u32(job + 0x14, sync_block);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u16(job + 4, 0x60);
        let world_data = e.mem.u32(world + 0x78);
        e.mem.set_u32(job + 0x1c, world_data);
        let semaphore = e.mem.u32(base + 0x6c);
        e.mem.set_u32(job + 0x10, semaphore);
        let words_0 = e.mem.u32(base + 0x30);
        e.mem.set_u32(job + 0x20, words_0);
        let array_data = e.mem.u32(array);
        e.mem.set_u32(job + 0x50, array_data);
        let words_1 = e.mem.u32(base + 0x34);
        e.mem.set_u32(job + 0x34, block_2);
        e.mem.set_u8(job + 1, 3);
        e.mem.set_u32(job + 0x18, 0);
        e.mem.set_u8(job + 2, 1);
        e.mem.set_u8(job, 6);
        e.mem.set_u32(job + 0x24, words_1);
        e.mem.set_u32(job + 0x30, block_1);
        let tree_field_10 = e.mem.u32(tree + 0x10);
        e.mem.set_u32(job + 0x38, tree_field_10);
        let tree_field_0 = e.mem.u32(tree);
        e.mem.set_u32(job + 0x28, tree_field_0);
        let tree_sum = e.mem.u32(tree + 0x18).wrapping_add(e.mem.u32(tree + 0x14));
        e.mem.set_u32(job + 0x3c, tree_sum);
        let tree_field_1c = e.mem.u32(tree + 0x1c);
        e.mem.set_u32(job + 0x2c, tree_field_1c);
        let tree_field_18 = e.mem.u32(tree + 0x18);
        e.mem.set_u32(job + 0x40, tree_field_18);
        profile_record(e, LABEL_BUILD);
        profile_pair(e, LABEL_BUILD_TREE_DISTRIBUTED, LABEL_QUEUE);
        e.call(QUEUE_ADD_JOB, &args![job_queue, job, 0u32]);
        profile_record(e, LABEL_START);
        e.vcall(thread_pool, 0xc, &args![job_queue, 3u32]);
        e.call(QUEUE_FINISH, &args![job_queue]);
        profile_record(e, LABEL_WAIT_FOR_COMPLETION);
        e.vcall(thread_pool, 0x10, &args![]);
        profile_record(e, LABEL_WAIT_SEMAPHORE);
        let semaphore = e.mem.u32(base + 0x6c);
        e.call(SEMAPHORE_WAIT, &args![semaphore]);
        profile_record(e, LABEL_FINISH_PPU);
        let block_2_now = e.mem.u32(job + 0x34);
        let block_1_now = e.mem.u32(job + 0x30);
        let count_now = e.mem.u32(b + 0x54);
        let tree = e.mem.u32(kd_slot);
        e.call(
            FINISH_TREE,
            &args![tree, count_now, block_1_now, block_2_now],
        );
        profile_record(e, LABEL_SPLIT_END);
        profile_record(e, LABEL_FINALIZE);
        profile_record(e, LABEL_SPLIT_END);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(b + 0x18);
        let data = e.mem.u32(b + 0x5c);
        release_array(e, allocator, data, capacity_word, 4);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(b + 0x14);
        let data = e.mem.u32(b + 0x58);
        release_array(e, allocator, data, capacity_word, 4);
    });
}

// Translated from 00d099a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a non-empty active list (`+0x48`): `buildKdTree(this, kd-tree slot this + 0x10, array,
/// world at +0x28, job_queue, thread_pool)`.
pub fn fn_00d099a0(e: &mut Engine, this: Ptr, array: u32, job_queue: u32, thread_pool: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0x48) == 0 {
        return;
    }
    let world = e.mem.u32(base + 0x28);
    build_kd_tree(e, this, base + 0x10, array, world, job_queue, thread_pool);
}

/// The integer-to-float conversion of the AABB code: `FILD` of the 32-bit value (signed),
/// plus `2^32` (the float at `01099378`) when it is negative, stored as a `float`.
fn unsigned_to_float(e: &Engine, value: u32) -> f32 {
    let signed = value as i32;
    let bias = if signed < 0 {
        f64::from(e.mem.f32(UNSIGNED_BIAS))
    } else {
        0.0
    };
    (f64::from(signed) + bias) as f32
}

/// The three integer coordinates at `at`, as the vector `(x, y, z, w)` with `w` the float
/// at `01011d78`.
fn integer_box_corner(e: &Engine, at: u32, w: f32) -> Vec4 {
    [
        unsigned_to_float(e, e.mem.u32(at)),
        unsigned_to_float(e, e.mem.u32(at + 4)),
        unsigned_to_float(e, e.mem.u32(at + 8)),
        w,
    ]
}

// Translated from 00d09020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::buildInactiveTree` (named in the engine map): `array` holds the
/// collidables (data, size `n`); the two other words are unused. Two arrays are taken from the
/// memory router's stack allocator (the top at the router's word `+0`, advanced by
/// `(size + 0x7f) & ~0x7f` bytes, with the old top as data and base): 32-byte AABB slots
/// and the 4-byte collidable pointers, each with capacity `n` (flagged as not owned; the exe's
/// grow branch through `00c908c0` is kept, it needs a capacity below `n`). Records
/// `Ttprepare`. Per collidable: when its integer AABB is ordered (word `+0x20` not above
/// word `+0x30`, unsigned) both corners are converted to floats (see [`unsigned_to_float`];
/// lane 3 the float at `01011d78`), multiplied by the refined reciprocal of the world's scale
/// (`world + 0x78` data `+0x40`, lane 3 from the vector at `010c7170`) and offset by the
/// data's `+0x20` base (vector at `+0`); otherwise the shape's slot `0x1c` is called with
/// `(transform, 0.0, slot)`. Then `Ttbuild`, `00d9ce50` on the tree at `this[+0xc]` with
/// `(AABB slots, collidables, n, handles array)` when `n > 0`, and every collidable gets its
/// 16-bit handle (`00d08400`). Finally the handle array is freed when owned, and the router's
/// stack top is restored to each array's base (array 2, then array 1; the arrays are freed
/// when owned).
pub fn build_inactive_tree(e: &mut Engine, this: Ptr, array: u32, _unused_2: u32, _unused_3: u32) {
    let base = this.addr();
    let tree = e.mem.u32(base + 0xc);
    let count = e.mem.i32(array + 4);
    e.with_stack(0x50, |e, frame| {
        let b = frame.addr();
        // The AABB slots (elements of 0x20 bytes) at b + 0x40, the pointers at b + 0x30,
        // the handles at b + 0x24: data, size, capacity word (and the base at +0xc).
        e.mem.set_u32(b + 0x40, 0);
        e.mem.set_u32(b + 0x44, 0);
        e.mem.set_u32(b + 0x48, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        let advance = (((count as u32) << 5).wrapping_add(0x7f)) & 0xffff_ff80;
        e.mem.set_u32(router, top.wrapping_add(advance));
        let flagged = (count as u32) | 0x8000_0000;
        e.mem.set_u32(b + 0x40, top);
        e.mem.set_u32(b + 0x48, flagged);
        e.mem.set_u32(b + 0x4c, top);
        let allocator = router_allocator(e);
        let capacity = (e.mem.u32(b + 0x48) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, b + 0x40, wanted as u32, 0x20u32],
            );
        }
        e.mem.set_u32(b + 0x44, count as u32);
        e.mem.set_u32(b + 0x30, 0);
        e.mem.set_u32(b + 0x34, 0);
        e.mem.set_u32(b + 0x38, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        let advance = ((count as u32).wrapping_mul(4).wrapping_add(0x7f)) & 0xffff_ff80;
        e.mem.set_u32(router, top.wrapping_add(advance));
        e.mem.set_u32(b + 0x30, top);
        e.mem.set_u32(b + 0x38, flagged);
        e.mem.set_u32(b + 0x3c, top);
        let allocator = router_allocator(e);
        let capacity = (e.mem.u32(b + 0x38) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, b + 0x30, wanted as u32, 4u32],
            );
        }
        e.mem.set_u32(b + 0x34, count as u32);
        profile_record(e, LABEL_TREE_PREPARE);
        let world = e.mem.u32(base + 0x28);
        let bounds = e.mem.u32(world + 0x78).wrapping_add(0x20);
        let mut index = 0i32;
        let mut offset = 0u32;
        while index < count {
            let w = e.mem.f32(BOUNDS_W);
            let collidables = e.mem.u32(array);
            let collidable = e.mem.u32(collidables.wrapping_add(4 * index as u32));
            let slots = e.mem.u32(b + 0x40);
            let slot = slots.wrapping_add(offset);
            let box_at = collidable.wrapping_add(0x20);
            if e.mem.u32(box_at) <= e.mem.u32(box_at + 0x10) {
                let low = integer_box_corner(e, box_at, w);
                let high = integer_box_corner(e, box_at + 0x10, w);
                let scale = load_vec(e, bounds + 0x20);
                let fill = load_vec(e, RECIPROCAL_FILL);
                let scale = [scale[0], scale[1], scale[2], fill[3]];
                let estimate = scale.map(rcp_estimate);
                let refined: Vec4 = std::array::from_fn(|lane| {
                    (estimate[lane] + estimate[lane])
                        - (estimate[lane] * scale[lane]) * estimate[lane]
                });
                let origin = load_vec(e, bounds);
                let low: Vec4 = std::array::from_fn(|lane| refined[lane] * low[lane]);
                let high: Vec4 = std::array::from_fn(|lane| high[lane] * refined[lane]);
                let low: Vec4 = std::array::from_fn(|lane| low[lane] - origin[lane]);
                let high: Vec4 = std::array::from_fn(|lane| high[lane] - origin[lane]);
                store_vec(e, slot, low);
                store_vec(e, slot + 0x10, high);
            } else {
                let shape = e.mem.u32(collidable);
                let transform = e.mem.u32(collidable + 8);
                e.vcall(shape, 0x1c, &args![transform, 0u32, slot]);
            }
            let collidables = e.mem.u32(array);
            let value = e.mem.u32(collidables.wrapping_add(4 * index as u32));
            let pointers = e.mem.u32(b + 0x30);
            e.mem
                .set_u32(pointers.wrapping_add(4 * index as u32), value);
            offset = offset.wrapping_add(0x20);
            index += 1;
        }
        profile_record(e, LABEL_END);
        e.mem.set_u32(b + 0x24, 0);
        e.mem.set_u32(b + 0x28, 0);
        e.mem.set_u32(b + 0x2c, 0x8000_0000);
        profile_record(e, LABEL_TREE_BUILD);
        if count > 0 {
            let slots = e.mem.u32(b + 0x40);
            let pointers = e.mem.u32(b + 0x30);
            e.call(
                AABB_TREE_INSERT_ALL,
                &args![tree, slots, pointers, count as u32, b + 0x24],
            );
        }
        profile_record(e, LABEL_END);
        let mut index = 0i32;
        while index < count {
            let handles = e.mem.u32(b + 0x24);
            let handle = e.mem.u16(handles.wrapping_add(2 * index as u32));
            let collidables = e.mem.u32(array);
            let collidable = e.mem.u32(collidables.wrapping_add(4 * index as u32));
            e.call(SET_TREE_DATA, &args![collidable, u32::from(handle)]);
            index += 1;
        }
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(b + 0x2c);
        e.mem.set_u32(b + 0x28, 0);
        let handles = e.mem.u32(b + 0x24);
        release_array(e, allocator, handles, capacity_word, 2);
        let pointer_base = e.mem.u32(b + 0x3c);
        e.mem.set_u32(b + 0x24, 0);
        e.mem.set_u32(b + 0x2c, 0x8000_0000);
        if pointer_base == e.mem.u32(b + 0x30) {
            e.mem.set_u32(b + 0x34, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, pointer_base);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(b + 0x38);
        e.mem.set_u32(b + 0x34, 0);
        let pointers = e.mem.u32(b + 0x30);
        release_array(e, allocator, pointers, capacity_word, 4);
        let slot_base = e.mem.u32(b + 0x4c);
        e.mem.set_u32(b + 0x30, 0);
        e.mem.set_u32(b + 0x38, 0x8000_0000);
        if slot_base == e.mem.u32(b + 0x40) {
            e.mem.set_u32(b + 0x44, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, slot_base);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(b + 0x48);
        e.mem.set_u32(b + 0x44, 0);
        let slots = e.mem.u32(b + 0x40);
        release_array(e, allocator, slots, capacity_word, 0x20);
    });
}

/// The next occupied slot of the pending map from `start`: the first index up to the highest
/// slot index (`+0x58`) whose key is not -1, or one past the end.
fn next_occupied_slot(e: &Engine, base: u32, start: i32) -> i32 {
    let last = e.mem.i32(base + 0x58);
    let mut index = start;
    if index <= last {
        let slots = e.mem.u32(base + 0x50);
        while index <= last && e.mem.u32(slots.wrapping_add(8 * index as u32)) == 0xffff_ffff {
            index += 1;
        }
    }
    index
}

// Translated from 00d099d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::incrementalBuild` (named in the engine map): `(job_queue,
/// thread_pool)`. A word array for the collidables found inactive is taken from the router's
/// stack allocator (`size + 0x7f & ~0x7f` bytes of `size * 4`, size = the active list's
/// size; capacity word `size | 0x80000000`). With the mode flag (`+0x24`) set (`LtIncrementalBuild`
/// / `StClear Tree`, `StFind Inactivated`): when the "inactive list valid" byte (`+0x69`) is
/// set, `00d08800` finds the newly inactive collidables (its answer is kept as the "all
/// scanned" byte), each is added to the inactive AABB tree (`+0xc`; shape slot `0x1c`
/// `(transform, 0.0, aabb)`, counter `MiNum of Added Nodes`, `00d9c610(aabb, collidable)`,
/// handle into `00d08400`) and the byte is cleared. Then `StFind Activated`: every pending
/// map entry with value 1 whose collidable has a tree handle other than `0xffff` and flag
/// byte 1 is removed from the inactive tree (counter `MiNum of Removed Nodes`,
/// `00d9c7d0`) and pushed back on the active list; `StUpdate Active` and `00d099a0`. With
/// the flag clear: `LtDynamicBuild` / `StGather`, size 0, `00d08ff0`, `StClear`, `StRebuild`,
/// `buildKdTree`. In both cases a final `00d08550`; the "inactive list valid" byte is set when the
/// scan answered true; the router's stack top is restored and the array freed when owned.
pub fn incremental_build(e: &mut Engine, this: Ptr, job_queue: u32, thread_pool: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 0x48);
    e.with_stack(0x50, |e, frame| {
        let b = frame.addr();
        e.mem.set_u32(b + 0x24, 0);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        let advance = (size.wrapping_mul(4).wrapping_add(0x7f)) & 0xffff_ff80;
        e.mem.set_u32(router, top.wrapping_add(advance));
        e.mem.set_u32(b + 0x20, top);
        e.mem.set_u32(b + 0x28, size | 0x8000_0000);
        e.mem.set_u32(b + 0x2c, top);
        e.mem.set_u8(b + 0x17, 0);
        if e.mem.u8(base + 0x24) != 0 {
            profile_pair(e, LABEL_INCREMENTAL_BUILD, LABEL_CLEAR_TREE);
            profile_record(e, LABEL_FIND_INACTIVATED);
            if e.mem.u8(base + 0x69) != 0 {
                let all_scanned = scan_active_list_and_get_inactivated(e, this, b + 0x20);
                profile_record(e, LABEL_UPDATE_INACTIVE_TREE);
                e.mem.set_u8(b + 0x17, u8::from(all_scanned));
                let tree = e.mem.u32(base + 0xc);
                let mut index = 0i32;
                while index < e.mem.i32(b + 0x24) {
                    let list = e.mem.u32(b + 0x20);
                    let collidable = e.mem.u32(list.wrapping_add(4 * index as u32));
                    let shape = e.mem.u32(collidable);
                    let transform = e.mem.u32(collidable + 8);
                    e.vcall(shape, 0x1c, &args![transform, 0u32, b + 0x30]);
                    profile_counter(e, LABEL_ADDED_NODES);
                    let handle = e
                        .call(AABB_TREE_INSERT, &args![tree, b + 0x30, collidable])
                        .u32();
                    let list = e.mem.u32(b + 0x20);
                    let collidable = e.mem.u32(list.wrapping_add(4 * index as u32));
                    e.call(SET_TREE_DATA, &args![collidable, handle & 0xffff]);
                    index += 1;
                }
                e.mem.set_u8(base + 0x69, 0);
            }
            profile_record(e, LABEL_FIND_ACTIVATED);
            let tree = e.mem.u32(base + 0xc);
            let mut slot = next_occupied_slot(e, base, 0);
            while slot <= e.mem.i32(base + 0x58) {
                let slots = e.mem.u32(base + 0x50);
                let key = e.mem.u32(slots.wrapping_add(8 * slot as u32));
                let value = e.mem.u32(slots.wrapping_add(8 * slot as u32 + 4));
                let handle = tree_data(e, key);
                if value == 1 && handle != 0xffff && e.mem.u8(key + 0x18) == 1 {
                    profile_counter(e, LABEL_REMOVED_NODES);
                    let handle = tree_data(e, key);
                    e.call(AABB_TREE_REMOVE, &args![tree, u32::from(handle)]);
                    push_back_to_active_list(e, this, key);
                }
                slot = next_occupied_slot(e, base, slot + 1);
            }
            profile_record(e, LABEL_UPDATE_ACTIVE);
            fn_00d099a0(e, this, base + 0x44, job_queue, thread_pool);
            profile_record(e, LABEL_SPLIT_END);
        } else {
            profile_pair(e, LABEL_DYNAMIC_BUILD, LABEL_GATHER);
            let world = e.mem.u32(base + 0x28);
            e.mem.set_u32(base + 0x48, 0);
            fn_00d08ff0(e, base + 0x44, world);
            profile_record(e, LABEL_CLEAR);
            profile_record(e, LABEL_REBUILD);
            let world = e.mem.u32(base + 0x28);
            build_kd_tree(
                e,
                this,
                base + 0x10,
                base + 0x44,
                world,
                job_queue,
                thread_pool,
            );
            profile_record(e, LABEL_SPLIT_END);
        }
        fn_00d08550(e, this);
        if e.mem.u8(b + 0x17) != 0 {
            e.mem.set_u8(base + 0x69, 1);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let restored = e.mem.u32(b + 0x2c);
        e.mem.set_u32(router, restored);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(b + 0x28);
        let data = e.mem.u32(b + 0x20);
        release_array(e, allocator, data, capacity_word, 4);
    });
}

// Translated from 00d09df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::fullBuild` (named in the engine map): `(job_queue,
/// thread_pool)`. Records `LtActiveTree` / `StGather`, empties the active list, gathers the
/// active islands' collidables (`00d08ac0`) and, in the single-tree mode (`+0x24` clear),
/// the inactive ones too (`00d08b90`); every active collidable's tree data is set to `0xffff`
/// (`00d08400`); `StClear`, `StRebuild`, `00d099a0` (the kd-tree). With the flag set, a word
/// array (empty, grown on the heap) gathers the inactive islands' collidables (`LtInactiveTree`
/// / `StGather`, `00d08b90`), `StClear`, the inactive tree's `00d9bed0`, `StRebuild`,
/// `buildInactiveTree` and the end record. Finally `00d08550` and the array is freed when
/// owned.
pub fn full_build(e: &mut Engine, this: Ptr, job_queue: u32, thread_pool: u32) {
    let base = this.addr();
    e.with_stack(0xc, |e, local| {
        let array = local.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        profile_pair(e, LABEL_ACTIVE_TREE, LABEL_GATHER);
        let world = e.mem.u32(base + 0x28);
        e.mem.set_u32(base + 0x48, 0);
        fn_00d08ac0(e, base + 0x44, world);
        if e.mem.u8(base + 0x24) == 0 {
            let world = e.mem.u32(base + 0x28);
            fn_00d08b90(e, base + 0x44, world);
        }
        let mut index = 0i32;
        while index < e.mem.i32(base + 0x48) {
            let list = e.mem.u32(base + 0x44);
            let collidable = e.mem.u32(list.wrapping_add(4 * index as u32));
            e.call(SET_TREE_DATA, &args![collidable, 0xffffu32]);
            index += 1;
        }
        profile_record(e, LABEL_CLEAR);
        profile_record(e, LABEL_REBUILD);
        fn_00d099a0(e, this, base + 0x44, job_queue, thread_pool);
        profile_record(e, LABEL_SPLIT_END);
        if e.mem.u8(base + 0x24) != 0 {
            profile_pair(e, LABEL_INACTIVE_TREE, LABEL_GATHER);
            let world = e.mem.u32(base + 0x28);
            e.mem.set_u32(array + 4, 0);
            fn_00d08b90(e, array, world);
            profile_record(e, LABEL_CLEAR);
            let inactive_tree = e.mem.u32(base + 0xc);
            e.call(AABB_TREE_CALL, &args![inactive_tree]);
            profile_record(e, LABEL_REBUILD);
            build_inactive_tree(e, this, array, job_queue, thread_pool);
            profile_record(e, LABEL_SPLIT_END);
        }
        fn_00d08550(e, this);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        let data = e.mem.u32(array);
        release_array(e, allocator, data, capacity_word, 4);
    });
}

// Translated from 00d0a060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbTreeWorldManager::updateFromWorld` (named in the engine map): `(job_queue,
/// thread_pool)`. Records `TtTreeBuild`, sets the byte `+0x68`; the first time (byte `+0x2c`
/// clear) `fullBuild` runs and the byte is set, afterwards `incrementalBuild`; the end record
/// closes the section.
pub fn update_from_world(e: &mut Engine, this: Ptr, job_queue: u32, thread_pool: u32) {
    let base = this.addr();
    profile_record(e, LABEL_TREE_BUILD_TOTAL);
    let built = e.mem.u8(base + 0x2c) != 0;
    e.mem.set_u8(base + 0x68, 1);
    if !built {
        full_build(e, this, job_queue, thread_pool);
        e.mem.set_u8(base + 0x2c, 1);
    } else {
        incremental_build(e, this, job_queue, thread_pool);
    }
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_8330, deleting_destructor_00d08330(Ptr, u8) -> Ptr),
        entry!(0x00d0_8350, deleting_destructor_00d08350(Ptr, u8) -> Ptr),
        entry!(0x00d0_8550, fn_00d08550(Ptr) -> u32),
        entry!(0x00d0_8560, fn_00d08560(Ptr)),
        entry!(0x00d0_8570, fn_00d08570(Ptr, u32)),
        entry!(0x00d0_85e0, cast_ray(Ptr, u32, u32, u32)),
        entry!(
            0x00d0_8740,
            fn_00d08740(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d0_8800,
            scan_active_list_and_get_inactivated(Ptr, u32) -> bool
        ),
        entry!(0x00d0_8940, fn_00d08940(Ptr, u32)),
        entry!(0x00d0_8980, fn_00d08980(Ptr, u32)),
        entry!(0x00d0_89c0, fn_00d089c0(Ptr, u32)),
        entry!(0x00d0_89d0, fn_00d089d0(Ptr, u32)),
        entry!(0x00d0_89e0, fn_00d089e0(Ptr, u32)),
        entry!(0x00d0_89f0, fn_00d089f0(Ptr, u32)),
        entry!(0x00d0_8a40, fn_00d08a40(Ptr, u32)),
        entry!(0x00d0_8a90, fn_00d08a90(Ptr, u32)),
        entry!(0x00d0_8ac0, fn_00d08ac0(u32, u32)),
        entry!(0x00d0_8b90, fn_00d08b90(u32, u32)),
        entry!(0x00d0_8c90, push_back_to_active_list(Ptr, u32)),
        entry!(
            0x00d0_8cf0,
            hkp_aabb_tree_world_manager_construct(Ptr, u32, u32, f32) -> Ptr
        ),
        entry!(0x00d0_8e80, hkp_aabb_tree_world_manager_destruct(Ptr)),
        entry!(0x00d0_8fb0, fn_00d08fb0(Ptr, u32)),
        entry!(0x00d0_8fd0, fn_00d08fd0(Ptr, u32)),
        entry!(0x00d0_8ff0, fn_00d08ff0(u32, u32)),
        entry!(0x00d0_9020, build_inactive_tree(Ptr, u32, u32, u32)),
        entry!(0x00d0_94a0, build_kd_tree(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00d0_99a0, fn_00d099a0(Ptr, u32, u32, u32)),
        entry!(0x00d0_99d0, incremental_build(Ptr, u32, u32)),
        entry!(0x00d0_9df0, full_build(Ptr, u32, u32)),
        entry!(0x00d0_a060, update_from_world(Ptr, u32, u32)),
        entry!(0x00d0_a130, deleting_destructor_00d0a130(Ptr, u8) -> Ptr),
        entry!(0x00d0_a150, deleting_destructor_00d0a150(Ptr, u8) -> Ptr),
        entry!(0x00d0_a170, deleting_destructor_00d0a170(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Doubles for the heap allocator's slots 4 (take), 8 (free block), 0xc (take with size
    /// word) and 0x10 (free data).
    const TAKE: u32 = 0x0900_0001;
    const FREE_BLOCK: u32 = 0x0900_0002;
    const TAKE_SIZED: u32 = 0x0900_0003;
    const FREE_DATA: u32 = 0x0900_0004;
    /// Doubles for the shape's slot 0x1c, the world object's slot 0x40, the thread pool's
    /// slots 0xc and 0x10 and the inactive tree's slot 0.
    const SHAPE_AABB: u32 = 0x0900_0010;
    const WORLD_OBJECT_FILL: u32 = 0x0900_0011;
    const POOL_PROCESS: u32 = 0x0900_0012;
    const POOL_WAIT: u32 = 0x0900_0013;
    const TREE_DELETE: u32 = 0x0900_0014;

    type Words = Rc<RefCell<Vec<u32>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn nothing(e: &mut Engine, addr: u32) {
        e.register_double(addr, |_, _| Ret::default());
    }

    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that returns its first argument (a constructor's `this`).
    fn returning_this(e: &mut Engine, addr: u32) {
        e.register_double(addr, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
    }

    /// Allocates the words (plus one spare) and returns their address.
    fn put_words(e: &mut Engine, words: &[u32]) -> u32 {
        let at = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *word);
        }
        at
    }

    fn read_words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    struct Router {
        router: u32,
        allocator: u32,
    }

    /// The memory router and the heap allocator behind it. `granted` is what the size-returning
    /// slot grants; a take of `fail_size` bytes fails (returns 0).
    fn install_router_with(e: &mut Engine, granted: u32, fail_size: u32) -> Router {
        let table = e.mem.alloc(0x20);
        for (slot, double) in [
            (4, TAKE),
            (8, FREE_BLOCK),
            (0xc, TAKE_SIZED),
            (0x10, FREE_DATA),
        ] {
            e.mem.set_u32(table + slot, double);
        }
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(TAKE, move |e, a| Ret {
            eax: if a[1] == fail_size {
                0
            } else {
                e.mem.alloc(a[1])
            },
            ..Ret::default()
        });
        e.register_double(TAKE_SIZED, move |e, a| {
            let wanted = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], granted);
            Ret {
                eax: e.mem.alloc(wanted),
                ..Ret::default()
            }
        });
        nothing(e, FREE_BLOCK);
        nothing(e, FREE_DATA);
        Router { router, allocator }
    }

    fn install_router(e: &mut Engine) -> Router {
        install_router_with(e, 0x16, 0)
    }

    /// `_reserveMore`: doubles the capacity (at least 4), copying the elements.
    fn install_reserve_more(e: &mut Engine) {
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let wanted = (capacity * 2).max(4);
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let block = e.mem.alloc(wanted * element);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, wanted);
            Ret::default()
        });
    }

    /// The monitor stream (records from `+0x40`; with `room` false the cursor is at the
    /// limit) and the TLS slot.
    fn monitor_stream(e: &mut Engine, room: bool) -> u32 {
        let stream = e.mem.alloc(0x400);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem
            .set_u32(stream + 0xc, stream + if room { 0x3f0 } else { 0x40 });
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    fn record_size(label: u32) -> u32 {
        match label {
            LABEL_CAST_RAY
            | LABEL_BUILD_KD_TREE_MT
            | LABEL_BUILD_TREE_DISTRIBUTED
            | LABEL_INCREMENTAL_BUILD
            | LABEL_DYNAMIC_BUILD
            | LABEL_ACTIVE_TREE
            | LABEL_INACTIVE_TREE => 16,
            LABEL_ADDED_NODES | LABEL_REMOVED_NODES => 8,
            _ => 12,
        }
    }

    /// The labels of the profiling records written since the stream was made.
    fn record_labels(e: &Engine, stream: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        let mut at = stream + 0x40;
        let mut labels = Vec::new();
        while at < end {
            let label = e.mem.u32(at);
            labels.push(label);
            at += record_size(label);
        }
        labels
    }

    fn manager(e: &mut Engine) -> u32 {
        e.mem.alloc(0x80)
    }

    fn entity(e: &mut Engine) -> u32 {
        e.mem.alloc(0x60)
    }

    /// Gives the manager an active list holding `items` with the capacity word.
    fn set_active_list(e: &mut Engine, mgr: u32, items: &[u32], capacity_word: u32) {
        let data = put_words(e, items);
        e.mem.set_u32(mgr + 0x44, data);
        e.mem.set_u32(mgr + 0x48, items.len() as u32);
        e.mem.set_u32(mgr + 0x4c, capacity_word);
    }

    /// A manager with the mode flag set and an active list holding the entities' collidables.
    fn manager_with_entities(e: &mut Engine, entities: &[u32]) -> u32 {
        let mgr = manager(e);
        e.mem.set_u8(mgr + 0x24, 1);
        let items: Vec<u32> = entities.iter().map(|entity| entity + 0x10).collect();
        set_active_list(e, mgr, &items, items.len() as u32);
        mgr
    }

    /// An island whose entity list is `entities`.
    fn island(e: &mut Engine, entities: &[u32]) -> u32 {
        let island = e.mem.alloc(0x60);
        let list = put_words(e, entities);
        e.mem.set_u32(island + 0x48, list);
        e.mem.set_u32(island + 0x4c, entities.len() as u32);
        island
    }

    /// Gives the world the object at `+0x58` (the double `WORLD_OBJECT_FILL` in its slot
    /// `0x40`) and the block at `+0x78`.
    fn give_world_object(e: &mut Engine, world: u32) {
        let table = e.mem.alloc(0x50);
        e.mem.set_u32(table + 0x40, WORLD_OBJECT_FILL);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        e.mem.set_u32(world + 0x58, object);
        let data = e.mem.alloc(0x80);
        e.mem.set_u32(world + 0x78, data);
        nothing(e, WORLD_OBJECT_FILL);
    }

    /// A world with no islands, no entities and an empty fixed island, with its object.
    fn empty_world(e: &mut Engine) -> u32 {
        let world = e.mem.alloc(0x120);
        let fixed = island(e, &[]);
        e.mem.set_u32(world + 0x20, fixed);
        give_world_object(e, world);
        world
    }

    /// Doubles for the single-threaded kd-tree build (the source's count is 5).
    fn install_build_doubles(e: &mut Engine) {
        nothing(e, SOURCE_CONSTRUCT);
        returning(e, SOURCE_COUNT, 5);
        nothing(e, KD_INFO_CONSTRUCT);
        nothing(e, KD_TREE_RESET);
        returning_this(e, KD_TREE_CONSTRUCT);
        nothing(e, PROJECT_PRIMITIVES);
        nothing(e, BUILD_TREE);
    }

    /// Doubles for the tree-data setter and the pending map clear.
    fn install_tree_data(e: &mut Engine) {
        nothing(e, SET_TREE_DATA);
        nothing(e, MAP_CLEAR);
    }

    // --- deleting destructors -------------------------------------------------------------

    /// Writes the vtable; frees through `free` only when bit 0 of the flags is set.
    fn check_deleting(address: u32, vtable: u32) {
        let mut e = engine();
        nothing(&mut e, FREE);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(address, &args![this, 0xffff_fffeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), vtable);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(e.call(address, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn deleting_destructor_00d08330_writes_its_vtable_and_frees() {
        check_deleting(0x00d0_8330, 0x010c_cf7c);
    }

    #[test]
    fn deleting_destructor_00d08350_writes_its_vtable_and_frees() {
        check_deleting(0x00d0_8350, 0x010c_cf8c);
    }

    #[test]
    fn deleting_destructor_00d0a130_writes_its_vtable_and_frees() {
        check_deleting(0x00d0_a130, 0x010c_cfa0);
    }

    #[test]
    fn deleting_destructor_00d0a150_writes_its_vtable_and_frees() {
        check_deleting(0x00d0_a150, 0x010c_cfac);
    }

    #[test]
    fn deleting_destructor_00d0a170_frees_through_the_allocator() {
        let mut e = engine();
        let router = install_router(&mut e);
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00d0_a170, &args![this, 0xffff_fffeu32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0x010c_cf8c);
        assert!(calls(&e, FREE_BLOCK).is_empty());
        assert_eq!(e.call(0x00d0_a170, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, FREE_BLOCK),
            vec![vec![router.allocator, this, 0x18]]
        );
    }

    // --- small state functions ------------------------------------------------------------

    #[test]
    fn fn_00d08550_clears_the_flag_and_tail_calls_the_map_clear() {
        let mut e = engine();
        returning(&mut e, MAP_CLEAR, 77);
        let mgr = manager(&mut e);
        e.mem.set_u8(mgr + 0x69, 1);
        assert_eq!(e.call(0x00d0_8550, &args![mgr]).u32(), 77);
        assert_eq!(e.mem.u8(mgr + 0x69), 0);
        assert_eq!(calls(&e, MAP_CLEAR), vec![vec![mgr + 0x50]]);
    }

    #[test]
    fn fn_00d08560_resets_the_list_size_and_two_bytes() {
        let mut e = engine();
        let mgr = manager(&mut e);
        e.mem.set_u32(mgr + 0x48, 5);
        e.mem.set_u8(mgr + 0x2c, 1);
        e.mem.set_u8(mgr + 0x68, 1);
        e.mem.set_u8(mgr + 0x69, 1);
        e.call(0x00d0_8560, &args![mgr]);
        assert_eq!(e.mem.u32(mgr + 0x48), 0);
        assert_eq!(e.mem.u8(mgr + 0x2c), 0);
        assert_eq!(e.mem.u8(mgr + 0x69), 0);
        assert_eq!(e.mem.u8(mgr + 0x68), 1);
    }

    // --- removal from the active list or the inactive tree --------------------------------

    /// Three entities, a flagged manager listing their collidables, the tree data double
    /// answering `handle`.
    fn removal_setup(handle: u32) -> (Engine, Vec<u32>, u32) {
        let mut e = engine();
        returning(&mut e, GET_TREE_DATA, handle);
        nothing(&mut e, AABB_TREE_REMOVE);
        let entities: Vec<u32> = (0..3).map(|_| entity(&mut e)).collect();
        let mgr = manager_with_entities(&mut e, &entities);
        e.mem.set_u32(mgr + 0xc, 0x1234);
        (e, entities, mgr)
    }

    #[test]
    fn fn_00d08570_removes_a_listed_collidable_by_moving_the_last_one_in() {
        let (mut e, ents, mgr) = removal_setup(7);
        e.call(0x00d0_8570, &args![mgr, ents[0]]);
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 2);
        assert_eq!(
            read_words(&e, data, 2),
            vec![ents[2] + 0x10, ents[1] + 0x10]
        );
        // The last element just shrinks the list.
        e.call(0x00d0_8570, &args![mgr, ents[1]]);
        assert_eq!(e.mem.u32(mgr + 0x48), 1);
        assert_eq!(read_words(&e, data, 1), vec![ents[2] + 0x10]);
        assert!(calls(&e, AABB_TREE_REMOVE).is_empty());
        assert_eq!(
            calls(&e, GET_TREE_DATA),
            vec![vec![ents[0] + 0x10], vec![ents[1] + 0x10]]
        );
    }

    #[test]
    fn fn_00d08570_removes_an_unlisted_collidable_from_the_inactive_tree() {
        let (mut e, _, mgr) = removal_setup(7);
        let stranger = entity(&mut e);
        e.call(0x00d0_8570, &args![mgr, stranger]);
        assert_eq!(calls(&e, AABB_TREE_REMOVE), vec![vec![0x1234, 7]]);
        assert_eq!(e.mem.u32(mgr + 0x48), 3);
    }

    #[test]
    fn fn_00d08570_leaves_an_unlisted_collidable_without_tree_data_alone() {
        let (mut e, _, mgr) = removal_setup(0xffff);
        let stranger = entity(&mut e);
        e.call(0x00d0_8570, &args![mgr, stranger]);
        assert!(calls(&e, AABB_TREE_REMOVE).is_empty());
    }

    #[test]
    fn fn_00d08570_does_nothing_without_the_mode_flag() {
        let (mut e, ents, mgr) = removal_setup(7);
        e.mem.set_u8(mgr + 0x24, 0);
        e.call(0x00d0_8570, &args![mgr, ents[0]]);
        assert!(calls(&e, GET_TREE_DATA).is_empty());
        assert_eq!(e.mem.u32(mgr + 0x48), 3);
    }

    /// The thunk adjusts `this` by `offset` and hands the entity to `00d08570`.
    fn check_removal_thunk(address: u32, offset: u32) {
        let (mut e, ents, mgr) = removal_setup(7);
        e.call(address, &args![mgr + offset, ents[2]]);
        assert_eq!(e.mem.u32(mgr + 0x48), 2);
        assert_eq!(calls(&e, GET_TREE_DATA), vec![vec![ents[2] + 0x10]]);
    }

    #[test]
    fn fn_00d089c0_forwards_to_the_removal_from_the_first_listener() {
        check_removal_thunk(0x00d0_89c0, 0x14);
    }

    #[test]
    fn fn_00d089d0_forwards_to_the_removal_from_the_first_listener() {
        check_removal_thunk(0x00d0_89d0, 0x14);
    }

    #[test]
    fn fn_00d089e0_forwards_to_the_removal_from_the_second_listener() {
        check_removal_thunk(0x00d0_89e0, 0x18);
    }

    // --- the pending map ------------------------------------------------------------------

    /// The map `get` double answers "found" (0) for the keys in `present` and checks that its
    /// output word holds the key; `insert` is logged.
    fn install_map(e: &mut Engine, present: Vec<u32>) {
        e.register_double(MAP_GET, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), a[1]);
            Ret {
                eax: u32::from(!present.contains(&a[1])),
                ..Ret::default()
            }
        });
        nothing(e, MAP_INSERT);
    }

    fn check_pending_insert(address: u32, value: u32) {
        let mut e = engine();
        let router = install_router(&mut e);
        install_map(&mut e, vec![0x5555]);
        let mgr = manager(&mut e);
        e.call(address, &args![mgr, 0x4444u32]);
        e.call(address, &args![mgr, 0x5555u32]);
        assert_eq!(calls(&e, MAP_GET).len(), 2);
        assert_eq!(
            calls(&e, MAP_INSERT),
            vec![vec![mgr + 0x50, router.allocator, 0x4444, value]]
        );
    }

    #[test]
    fn fn_00d08940_inserts_an_absent_key_with_the_value_one() {
        check_pending_insert(0x00d0_8940, 1);
    }

    #[test]
    fn fn_00d08980_inserts_an_absent_key_with_the_value_minus_one() {
        check_pending_insert(0x00d0_8980, 0xffff_ffff);
    }

    /// An island whose first and third entities have the collidable flag byte 1, given to the
    /// listener sub-object at `+0x1c` of a manager.
    fn check_island_listener(address: u32, value: u32) {
        let mut e = engine();
        let router = install_router(&mut e);
        install_map(&mut e, vec![]);
        let mgr = manager(&mut e);
        let ents: Vec<u32> = (0..3).map(|_| entity(&mut e)).collect();
        for (ent, flag) in ents.iter().zip([1u8, 0, 1]) {
            e.mem.set_u8(ent + 0x10 + 0x18, flag);
        }
        let isl = island(&mut e, &ents);
        e.mem.set_u8(mgr + 0x68, 5);
        // Mode flag off: nothing happens.
        e.call(address, &args![mgr + 0x1c, isl]);
        assert!(calls(&e, MAP_GET).is_empty());
        assert_eq!(e.mem.u8(mgr + 0x68), 5);
        e.mem.set_u8(mgr + 0x24, 1);
        e.call(address, &args![mgr + 0x1c, isl]);
        assert_eq!(e.mem.u8(mgr + 0x69), 1);
        assert_eq!(e.mem.u8(mgr + 0x68), 0);
        assert_eq!(
            calls(&e, MAP_INSERT),
            vec![
                vec![mgr + 0x50, router.allocator, ents[0] + 0x10, value],
                vec![mgr + 0x50, router.allocator, ents[2] + 0x10, value],
            ]
        );
    }

    #[test]
    fn fn_00d089f0_records_the_flagged_collidables_with_minus_one() {
        check_island_listener(0x00d0_89f0, 0xffff_ffff);
    }

    #[test]
    fn fn_00d08a40_records_the_flagged_collidables_with_one() {
        check_island_listener(0x00d0_8a40, 1);
    }

    #[test]
    fn fn_00d08a90_marks_the_manager_and_records_the_collidable() {
        let mut e = engine();
        let router = install_router(&mut e);
        install_map(&mut e, vec![]);
        let mgr = manager(&mut e);
        let ent = entity(&mut e);
        e.mem.set_u8(mgr + 0x68, 5);
        e.call(0x00d0_8a90, &args![mgr + 0x20, ent]);
        assert!(calls(&e, MAP_GET).is_empty());
        assert_eq!(e.mem.u8(mgr + 0x68), 5);
        e.mem.set_u8(mgr + 0x24, 1);
        e.call(0x00d0_8a90, &args![mgr + 0x20, ent]);
        assert_eq!(e.mem.u8(mgr + 0x69), 1);
        assert_eq!(e.mem.u8(mgr + 0x68), 0);
        assert_eq!(
            calls(&e, MAP_INSERT),
            vec![vec![mgr + 0x50, router.allocator, ent + 0x10, 1]]
        );
    }

    // --- the active list ------------------------------------------------------------------

    #[test]
    fn push_back_to_active_list_clears_the_tree_data_and_appends() {
        let mut e = engine();
        let router = install_router(&mut e);
        install_reserve_more(&mut e);
        install_tree_data(&mut e);
        let mgr = manager(&mut e);
        set_active_list(&mut e, mgr, &[0x111], 1);
        // Without the mode flag nothing happens.
        e.call(0x00d0_8c90, &args![mgr, 0x222u32]);
        assert!(calls(&e, SET_TREE_DATA).is_empty());
        e.mem.set_u8(mgr + 0x24, 1);
        e.call(0x00d0_8c90, &args![mgr, 0x222u32]);
        e.call(0x00d0_8c90, &args![mgr, 0x333u32]);
        assert_eq!(
            calls(&e, SET_TREE_DATA),
            vec![vec![0x222, 0xffff], vec![0x333, 0xffff]]
        );
        // The array was full once (capacity 1), then had room (capacity 4).
        assert_eq!(
            calls(&e, ARRAY_RESERVE_MORE),
            vec![vec![router.allocator, mgr + 0x44, 4]]
        );
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 3);
        assert_eq!(read_words(&e, data, 3), vec![0x111, 0x222, 0x333]);
    }

    /// The listener thunks mark the manager's bytes and push the entity's collidable.
    fn check_push_thunk(address: u32, offset: u32) {
        let mut e = engine();
        install_router(&mut e);
        install_reserve_more(&mut e);
        install_tree_data(&mut e);
        let mgr = manager(&mut e);
        e.mem.set_u8(mgr + 0x24, 1);
        e.mem.set_u8(mgr + 0x68, 5);
        let ent = entity(&mut e);
        e.call(address, &args![mgr + offset, ent]);
        assert_eq!(e.mem.u8(mgr + 0x69), 1);
        assert_eq!(e.mem.u8(mgr + 0x68), 0);
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 1);
        assert_eq!(read_words(&e, data, 1), vec![ent + 0x10]);
    }

    #[test]
    fn fn_00d08fb0_marks_the_manager_and_pushes_from_the_first_listener() {
        check_push_thunk(0x00d0_8fb0, 0x14);
    }

    #[test]
    fn fn_00d08fd0_marks_the_manager_and_pushes_from_the_second_listener() {
        check_push_thunk(0x00d0_8fd0, 0x18);
    }

    /// A world with two active islands (entities 0, 1 and 2), three entries in the world's
    /// entity list (entities 3, 4, 5; the shape word of the middle one is 0), two inactive
    /// islands (entities 6 and 7) and a fixed island of entities 8, 9, 10 whose middle
    /// entity is the skipped one.
    fn gathering_world(e: &mut Engine) -> (u32, Vec<u32>) {
        let ents: Vec<u32> = (0..11).map(|_| entity(e)).collect();
        let world = e.mem.alloc(0x120);
        let active = [island(e, &ents[0..2]), island(e, &ents[2..3])];
        let list = put_words(e, &active);
        e.mem.set_u32(world + 0x28, list);
        e.mem.set_u32(world + 0x2c, 2);
        let listed = [ents[3], ents[4], ents[5]];
        for (ent, shape) in listed.iter().zip([0x11u32, 0, 0x33]) {
            e.mem.set_u32(ent + 0x10, shape);
        }
        let entity_list = put_words(e, &listed);
        e.mem.set_u32(world + 0x110, entity_list);
        e.mem.set_u32(world + 0x114, 3);
        let inactive = [island(e, &ents[6..7]), island(e, &ents[7..8])];
        let list = put_words(e, &inactive);
        e.mem.set_u32(world + 0x34, list);
        e.mem.set_u32(world + 0x38, 2);
        let fixed = island(e, &ents[8..11]);
        e.mem.set_u32(world + 0x20, fixed);
        e.mem.set_u32(world + 0x24, ents[9]);
        (world, ents)
    }

    fn empty_word_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, 0x8000_0000);
        array
    }

    #[test]
    fn fn_00d08ac0_gathers_the_active_islands_and_the_world_entities() {
        let mut e = engine();
        install_router(&mut e);
        install_reserve_more(&mut e);
        let (world, ents) = gathering_world(&mut e);
        let array = empty_word_array(&mut e);
        e.call(0x00d0_8ac0, &args![array, world]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 5);
        assert_eq!(
            read_words(&e, data, 5),
            [0, 1, 2, 3, 5].map(|i| ents[i] + 0x10).to_vec()
        );
    }

    #[test]
    fn fn_00d08b90_gathers_the_inactive_islands_and_the_fixed_island() {
        let mut e = engine();
        install_router(&mut e);
        install_reserve_more(&mut e);
        let (world, ents) = gathering_world(&mut e);
        let array = empty_word_array(&mut e);
        e.call(0x00d0_8b90, &args![array, world]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 4);
        assert_eq!(
            read_words(&e, data, 4),
            [6, 7, 8, 10].map(|i| ents[i] + 0x10).to_vec()
        );
    }

    #[test]
    fn fn_00d08ff0_resets_the_array_then_gathers_everything() {
        let mut e = engine();
        install_router(&mut e);
        install_reserve_more(&mut e);
        let (world, ents) = gathering_world(&mut e);
        let array = empty_word_array(&mut e);
        e.mem.set_u32(array + 4, 3);
        e.call(0x00d0_8ff0, &args![array, world]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 9);
        assert_eq!(
            read_words(&e, data, 9),
            [0, 1, 2, 3, 5, 6, 7, 8, 10]
                .map(|i| ents[i] + 0x10)
                .to_vec()
        );
    }

    // --- scanning the active list ---------------------------------------------------------

    /// Collidables with the flag byte `+0x18` as given (owner offset byte 0, so the owner is
    /// the collidable itself) and the activity double reporting state 0 (inactive) for the
    /// collidables listed in `inactive`.
    fn scan_setup(e: &mut Engine, flags: &[u8], inactive: &[usize]) -> Vec<u32> {
        let items: Vec<u32> = flags
            .iter()
            .map(|flag| {
                let item = e.mem.alloc(0x40);
                e.mem.set_u8(item + 0x18, *flag);
                item
            })
            .collect();
        let down: Vec<u32> = inactive.iter().map(|i| items[*i]).collect();
        e.register_double(OWNER_ACTIVITY_STATE, move |e, a| {
            e.mem.set_u8(a[1], if down.contains(&a[0]) { 0 } else { 2 });
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        items
    }

    #[test]
    fn scan_active_list_moves_every_inactivated_entry_when_the_byte_is_set() {
        let mut e = engine();
        let items = scan_setup(&mut e, &[1, 1, 0, 1, 1], &[1, 3]);
        let mgr = manager(&mut e);
        set_active_list(&mut e, mgr, &items, 5);
        e.mem.set_u8(mgr + 0x5c, 1);
        let out = e.mem.alloc(0x10);
        let out_data = e.mem.alloc(0x20);
        e.mem.set_u32(out, out_data);
        assert_eq!(e.call(0x00d0_8800, &args![mgr, out]).u32() & 0xff, 1);
        assert_eq!(e.mem.u32(out + 4), 2);
        assert_eq!(read_words(&e, out_data, 2), vec![items[3], items[1]]);
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 3);
        assert_eq!(read_words(&e, data, 3), vec![items[0], items[4], items[2]]);
        // The owner is asked for the entries whose flag byte is 1 only.
        assert_eq!(calls(&e, OWNER_ACTIVITY_STATE).len(), 4);
    }

    #[test]
    fn scan_active_list_scans_a_budget_downward_from_the_cursor() {
        let mut e = engine();
        let items = scan_setup(&mut e, &[1, 1, 1, 1, 1], &[3]);
        let mgr = manager(&mut e);
        set_active_list(&mut e, mgr, &items, 5);
        e.mem.set_u32(mgr + 0x60, 10);
        e.mem.set_u32(mgr + 0x64, 2);
        let out = e.mem.alloc(0x10);
        let out_data = e.mem.alloc(0x20);
        e.mem.set_u32(out, out_data);
        // The cursor is clamped to the last index (4); entries 4 and 3 are scanned.
        assert_eq!(e.call(0x00d0_8800, &args![mgr, out]).u32() & 0xff, 0);
        assert_eq!(e.mem.u32(mgr + 0x60), 2);
        assert_eq!(e.mem.u32(out + 4), 1);
        assert_eq!(read_words(&e, out_data, 1), vec![items[3]]);
        assert_eq!(e.mem.u32(mgr + 0x48), 4);
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(read_words(&e, data, 4)[3], items[4]);
    }

    #[test]
    fn scan_active_list_restarts_at_the_list_size_after_the_cursor_runs_out() {
        let mut e = engine();
        let items = scan_setup(&mut e, &[1, 1], &[]);
        let mgr = manager(&mut e);
        set_active_list(&mut e, mgr, &items, 2);
        e.mem.set_u32(mgr + 0x60, 1);
        e.mem.set_u32(mgr + 0x64, 30);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d0_8800, &args![mgr, out]).u32() & 0xff, 1);
        assert_eq!(e.mem.u32(mgr + 0x60), 2);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(calls(&e, OWNER_ACTIVITY_STATE).len(), 2);
    }

    // --- queries and ray casts ------------------------------------------------------------

    const SIGNALLING_NAN: u32 = 0x7fa0_0000;
    const QUIETED_NAN: u32 = 0x7fe0_0000;

    #[test]
    fn fn_00d08740_queries_both_trees_with_stack_objects() {
        let mut e = engine();
        e.mem.set_u32(QUERY_MARGIN, SIGNALLING_NAN);
        // Each double reads its stack object when it is called.
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        for (address, tag) in [(KD_TREE_QUERY, 1), (AABB_TREE_QUERY, 2)] {
            let seen = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                let q = a[0];
                seen.borrow_mut().extend([
                    tag,
                    e.mem.u32(q),
                    e.mem.u32(q + 4),
                    e.mem.u32(q + 0x24),
                ]);
                seen.borrow_mut().push(e.mem.u32(q + 0x80));
                Ret::default()
            });
        }
        let mgr = manager(&mut e);
        e.mem.set_u32(mgr + 0x10, 0x7000);
        e.mem.set_u32(mgr + 0xc, 0x8000);
        e.mem.set_u8(mgr + 0x24, 1);
        let words: Vec<u32> = (1..=7).collect();
        e.call(
            0x00d0_8740,
            &args![mgr, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(
            *seen.borrow(),
            vec![
                1,
                0x010c_cfcc,
                0,
                0,
                QUIETED_NAN,
                2,
                0x010c_cfec,
                0x010c_cfe0,
                0,
                QUIETED_NAN
            ]
        );
        let kd = calls(&e, KD_TREE_QUERY).remove(0);
        assert_eq!(kd[1..], [vec![0x7000], words.clone()].concat());
        let aabb = calls(&e, AABB_TREE_QUERY).remove(0);
        assert_eq!(aabb[1..], [vec![0x8000], words].concat());
        assert_eq!(aabb[0], kd[0]);
    }

    #[test]
    fn fn_00d08740_skips_the_missing_trees() {
        let mut e = engine();
        nothing(&mut e, KD_TREE_QUERY);
        nothing(&mut e, AABB_TREE_QUERY);
        let mgr = manager(&mut e);
        let call_args = args![mgr, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        e.call(0x00d0_8740, &call_args);
        assert!(calls(&e, KD_TREE_QUERY).is_empty());
        assert!(calls(&e, AABB_TREE_QUERY).is_empty());
    }

    /// A ray (from at `+0`, to at `+0x10`) and a manager with both trees.
    fn ray_setup(e: &mut Engine) -> (u32, u32) {
        let ray = e.mem.alloc(0x20);
        let mgr = manager(e);
        e.mem.set_u32(mgr + 0x10, 0x7000);
        e.mem.set_u32(mgr + 0xc, 0x8000);
        e.mem.set_u8(mgr + 0x24, 1);
        (mgr, ray)
    }

    #[test]
    fn cast_ray_builds_the_collectors_and_casts_through_both_trees() {
        let mut e = engine();
        let stream = monitor_stream(&mut e, true);
        let (mgr, ray) = ray_setup(&mut e);
        let filter_source = e.mem.alloc(0x20);
        // The doubles copy the stack objects when called.
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        nothing(&mut e, RAY_COLLECTOR_CONSTRUCT);
        let kd_seen = Rc::clone(&seen);
        e.register_double(KD_TREE_CAST_RAY, move |e, a| {
            let slot_value = e.mem.u32(a[0]);
            let object = a[4];
            kd_seen.borrow_mut().push(slot_value);
            for field in [0, 4, 8, 0xc, 0x10, 0x14] {
                kd_seen.borrow_mut().push(e.mem.u32(object + field));
            }
            Ret::default()
        });
        nothing(&mut e, AABB_TREE_CAST_RAY);
        e.call(0x00d0_85e0, &args![mgr, ray, filter_source, 0x9999u32]);
        // The first record overwrote the collector argument slot with its time stamp.
        let stamp = e.mem.u32(stream + 0x40 + 4);
        assert_eq!(
            *seen.borrow(),
            vec![
                stamp,
                0x010c_cfb8,
                ray,
                filter_source + 0x14,
                filter_source + 0x10,
                0x9999,
                0
            ]
        );
        assert_eq!(calls(&e, RAY_COLLECTOR_CONSTRUCT).len(), 1);
        let collector_object = calls(&e, RAY_COLLECTOR_CONSTRUCT)[0][0];
        let kd = calls(&e, KD_TREE_CAST_RAY).remove(0);
        assert_eq!(kd[1..4], [0x7000, ray, ray + 0x10]);
        assert_eq!(
            calls(&e, AABB_TREE_CAST_RAY),
            vec![vec![
                0x8000,
                ray,
                ray + 0x10,
                collector_object + 4,
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(
            record_labels(&e, stream),
            vec![LABEL_CAST_RAY, LABEL_INACTIVE_TREE_SPLIT, LABEL_SPLIT_END]
        );
        assert_eq!(e.mem.u32(stream + 0x40 + 0xc), LABEL_ACTIVE_TREE_SPLIT);
    }

    #[test]
    fn cast_ray_without_a_stream_slot_passes_the_collector_and_no_filter_pointers() {
        let mut e = engine();
        let stream = monitor_stream(&mut e, false);
        let (mgr, ray) = ray_setup(&mut e);
        e.mem.set_u8(mgr + 0x24, 0);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        nothing(&mut e, RAY_COLLECTOR_CONSTRUCT);
        let kd_seen = Rc::clone(&seen);
        e.register_double(KD_TREE_CAST_RAY, move |e, a| {
            kd_seen.borrow_mut().push(e.mem.u32(a[0]));
            kd_seen.borrow_mut().push(e.mem.u32(a[4] + 8));
            kd_seen.borrow_mut().push(e.mem.u32(a[4] + 0xc));
            Ret::default()
        });
        e.call(0x00d0_85e0, &args![mgr, ray, 0u32, 0x9999u32]);
        assert_eq!(*seen.borrow(), vec![0x9999, 0, 0]);
        assert!(record_labels(&e, stream).is_empty());
        assert!(calls(&e, AABB_TREE_CAST_RAY).is_empty());
    }

    #[test]
    fn cast_ray_skips_the_kd_tree_when_there_is_none() {
        let mut e = engine();
        let stream = monitor_stream(&mut e, true);
        let (mgr, ray) = ray_setup(&mut e);
        e.mem.set_u32(mgr + 0x10, 0);
        nothing(&mut e, RAY_COLLECTOR_CONSTRUCT);
        nothing(&mut e, AABB_TREE_CAST_RAY);
        e.call(0x00d0_85e0, &args![mgr, ray, 0u32, 0x9999u32]);
        assert!(calls(&e, KD_TREE_CAST_RAY).is_empty());
        assert_eq!(calls(&e, AABB_TREE_CAST_RAY).len(), 1);
        assert_eq!(record_labels(&e, stream).len(), 3);
    }

    // --- construction and destruction -----------------------------------------------------

    /// Doubles for the constructor's callees; the world has the block at `+0x78`.
    fn construction_setup(e: &mut Engine) -> (u32, u32) {
        nothing(e, MAP_RESERVE);
        returning_this(e, SEMAPHORE_CONSTRUCT);
        returning_this(e, AABB_TREE_CONSTRUCT);
        for add in WORLD_ADD_LISTENERS {
            nothing(e, add);
        }
        let world = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x80);
        e.mem.set_u32(world + 0x78, data);
        (world, data)
    }

    #[test]
    fn construct_sets_up_the_manager_and_the_inactive_tree() {
        let mut e = engine();
        let router = install_router(&mut e);
        let (world, data) = construction_setup(&mut e);
        let mgr = manager(&mut e);
        e.mem.set_u8(mgr + 0x5c, 9);
        e.mem.set_u8(mgr + 0x68, 9);
        e.mem.set_u32(mgr + 0x48, 9);
        let got = e
            .call(
                0x00d0_8cf0,
                &args![mgr, world, 0u32, f32::from_bits(SIGNALLING_NAN)],
            )
            .u32();
        assert_eq!(got, mgr);
        assert_eq!(e.mem.u32(mgr), 0x010c_d078);
        assert_eq!(
            read_words(&e, mgr + 0x14, 4),
            vec![0x010c_d05c, 0x010c_d044, 0x010c_d034, 0x010c_d024]
        );
        assert_eq!(e.mem.u16(mgr + 6), 1);
        assert_eq!(e.mem.u32(mgr + 8), 1);
        assert_eq!(e.mem.u32(mgr + 0x28), world);
        assert_eq!(read_words(&e, mgr + 0x44, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(read_words(&e, mgr + 0x50, 3), vec![0, 0, 0xffff_ffff]);
        assert_eq!(
            calls(&e, MAP_RESERVE),
            vec![vec![mgr + 0x50, router.allocator, 0]]
        );
        assert_eq!(
            calls(&e, TAKE),
            vec![
                vec![router.allocator, 0x10],
                vec![router.allocator, 4],
                vec![router.allocator, 0x80]
            ]
        );
        let sync_block = e.mem.u32(mgr + 0x3c);
        assert_ne!(sync_block, 0);
        let semaphore = e.mem.u32(mgr + 0x6c);
        assert_eq!(
            calls(&e, SEMAPHORE_CONSTRUCT),
            vec![vec![semaphore, 0, 1000]]
        );
        assert_eq!(e.mem.u32(mgr + 0x40), QUIETED_NAN);
        assert_eq!(e.mem.u8(mgr + 0x24), 1);
        for (index, add) in WORLD_ADD_LISTENERS.into_iter().enumerate() {
            assert_eq!(
                calls(&e, add),
                vec![vec![world, mgr + 0x14 + 4 * index as u32]]
            );
        }
        let tree = e.mem.u32(mgr + 0xc);
        assert_ne!(tree, 0);
        assert_eq!(
            calls(&e, AABB_TREE_CONSTRUCT),
            vec![vec![tree, data + 0x20, data + 0x30, data + 0x40]]
        );
        assert_eq!(e.mem.u32(mgr + 0x10), 0);
        assert_eq!(e.mem.u8(mgr + 0x5c), 0);
        assert_eq!(e.mem.u8(mgr + 0x68), 0);
        assert_eq!(e.mem.u32(mgr + 0x60), 0);
        assert_eq!(e.mem.u32(mgr + 0x64), 0x1e);
        assert_eq!(e.mem.u32(mgr + 0x48), 0);
        assert_eq!(read_words(&e, mgr + 0x30, 2), vec![0, 0]);
    }

    #[test]
    fn construct_in_mode_one_has_no_inactive_tree() {
        let mut e = engine();
        install_router(&mut e);
        let (world, _) = construction_setup(&mut e);
        let mgr = manager(&mut e);
        e.mem.set_u8(mgr + 0x24, 9);
        e.call(0x00d0_8cf0, &args![mgr, world, 1u32, 0.5f32]);
        assert_eq!(e.mem.u8(mgr + 0x24), 0);
        assert_eq!(e.mem.u32(mgr + 0xc), 0);
        assert_eq!(e.mem.f32(mgr + 0x40), 0.5);
        assert!(calls(&e, AABB_TREE_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, TAKE).len(), 2);
    }

    #[test]
    fn construct_leaves_the_flag_alone_for_other_modes() {
        let mut e = engine();
        install_router(&mut e);
        let (world, _) = construction_setup(&mut e);
        let mgr = manager(&mut e);
        e.call(0x00d0_8cf0, &args![mgr, world, 2u32, 0.5f32]);
        assert_eq!(e.mem.u8(mgr + 0x24), 0);
        assert_eq!(calls(&e, TAKE).len(), 2);
    }

    #[test]
    fn construct_survives_a_failed_semaphore_allocation() {
        let mut e = engine();
        install_router_with(&mut e, 0x16, 4);
        let (world, _) = construction_setup(&mut e);
        let mgr = manager(&mut e);
        e.mem.set_u32(mgr + 0x6c, 9);
        e.call(0x00d0_8cf0, &args![mgr, world, 1u32, 0.5f32]);
        assert_eq!(e.mem.u32(mgr + 0x6c), 0);
        assert!(calls(&e, SEMAPHORE_CONSTRUCT).is_empty());
    }

    /// A manager as the constructor leaves it, with every owned part present.
    fn destruction_setup(e: &mut Engine) -> u32 {
        for remove in WORLD_REMOVE_LISTENERS {
            nothing(e, remove);
        }
        nothing(e, KD_TREE_DESTRUCT);
        nothing(e, SEMAPHORE_DESTRUCT);
        nothing(e, MAP_CLEAR_AND_DEALLOCATE);
        nothing(e, MAP_DESTRUCT);
        nothing(e, TREE_DELETE);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, TREE_DELETE);
        let inactive = e.mem.alloc(0x10);
        e.mem.set_u32(inactive, table);
        let mgr = manager(e);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(mgr + 0x28, world);
        e.mem.set_u32(mgr + 0xc, inactive);
        e.mem.set_u32(mgr + 0x10, 0x7000);
        e.mem.set_u32(mgr + 0x3c, 0x7100);
        e.mem.set_u32(mgr + 0x6c, 0x7200);
        set_active_list(e, mgr, &[1, 2, 3], 3);
        mgr
    }

    #[test]
    fn destruct_tears_down_everything_the_manager_owns() {
        let mut e = engine();
        let router = install_router(&mut e);
        let mgr = destruction_setup(&mut e);
        let world = e.mem.u32(mgr + 0x28);
        let inactive = e.mem.u32(mgr + 0xc);
        let data = e.mem.u32(mgr + 0x44);
        e.call(0x00d0_8e80, &args![mgr]);
        for (index, remove) in WORLD_REMOVE_LISTENERS.into_iter().enumerate() {
            assert_eq!(
                calls(&e, remove),
                vec![vec![world, mgr + 0x14 + 4 * index as u32]]
            );
        }
        assert_eq!(calls(&e, TREE_DELETE), vec![vec![inactive, 1]]);
        assert_eq!(calls(&e, KD_TREE_DESTRUCT), vec![vec![0x7000]]);
        assert_eq!(calls(&e, SEMAPHORE_DESTRUCT), vec![vec![0x7200]]);
        assert_eq!(
            calls(&e, FREE_BLOCK),
            vec![
                vec![router.allocator, 0x7000, 0x50],
                vec![router.allocator, 0x7100, 0x10],
                vec![router.allocator, 0x7200, 4],
            ]
        );
        assert_eq!(
            calls(&e, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![mgr + 0x50, router.allocator]]
        );
        assert_eq!(calls(&e, MAP_DESTRUCT), vec![vec![mgr + 0x50]]);
        assert_eq!(calls(&e, FREE_DATA), vec![vec![router.allocator, data, 12]]);
        assert_eq!(read_words(&e, mgr + 0x44, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u32(mgr), 0x0102_e388);
        assert_eq!(
            read_words(&e, mgr + 0x14, 4),
            vec![0x0102_f13c, 0x0104_e3d0, 0x0104_de10, 0x010c_cf7c]
        );
    }

    #[test]
    fn destruct_skips_the_parts_that_are_missing_and_unowned_storage() {
        let mut e = engine();
        install_router(&mut e);
        let mgr = destruction_setup(&mut e);
        for offset in [0xc, 0x10, 0x6c] {
            e.mem.set_u32(mgr + offset, 0);
        }
        e.mem.set_u32(mgr + 0x4c, 0x8000_0003);
        e.call(0x00d0_8e80, &args![mgr]);
        assert!(calls(&e, TREE_DELETE).is_empty());
        assert!(calls(&e, KD_TREE_DESTRUCT).is_empty());
        assert!(calls(&e, SEMAPHORE_DESTRUCT).is_empty());
        assert_eq!(calls(&e, FREE_BLOCK).len(), 1);
        assert!(calls(&e, FREE_DATA).is_empty());
    }

    // --- building the kd-tree -------------------------------------------------------------

    /// Records `(first word, byte +5)` of the construction info the tree functions are given
    /// (the info lives on the exe's stack, so it has to be read inside the double).
    fn capture_info(e: &mut Engine, address: u32, info_index: usize, seen: &Words) {
        let seen = Rc::clone(seen);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().extend([
                e.mem.u32(a[info_index]),
                u32::from(e.mem.u8(a[info_index] + 5)),
            ]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    /// A manager, a world and an array of `size` collidables for the build tests.
    fn build_setup(e: &mut Engine, size: u32) -> (u32, u32, u32) {
        let world = empty_world(e);
        let mgr = manager(e);
        e.mem.set_u32(mgr + 0x30, 0x7000);
        e.mem.set_u32(mgr + 0x28, world);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, size);
        (mgr, world, array)
    }

    #[test]
    fn build_kd_tree_builds_a_new_tree_on_this_thread_without_a_pool() {
        let mut e = engine();
        let router = install_router(&mut e);
        install_build_doubles(&mut e);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        capture_info(&mut e, KD_TREE_CONSTRUCT, 1, &seen);
        let (mgr, world, array) = build_setup(&mut e, 3);
        let slot = e.mem.alloc(4);
        e.call(0x00d0_94a0, &args![mgr, slot, array, world, 0u32, 0u32]);
        let source = calls(&e, SOURCE_CONSTRUCT);
        assert_eq!(source.len(), 1);
        assert_eq!(source[0][1..], [array, world]);
        let source_object = source[0][0];
        assert_eq!(calls(&e, SOURCE_COUNT), vec![vec![source_object]]);
        let info = calls(&e, KD_INFO_CONSTRUCT)[0][0];
        // The world object fills the two vectors of the info.
        assert_eq!(
            calls(&e, WORLD_OBJECT_FILL)[0][1..],
            [info + 0x10, info + 0x20]
        );
        assert_eq!(calls(&e, TAKE), vec![vec![router.allocator, 0x50]]);
        let tree = e.mem.u32(slot);
        assert_ne!(tree, 0);
        assert_eq!(calls(&e, KD_TREE_CONSTRUCT), vec![vec![tree, info]]);
        // The count of the source and the "multi-threaded" byte 0.
        assert_eq!(*seen.borrow(), vec![5, 0]);
        assert_eq!(
            calls(&e, PROJECT_PRIMITIVES),
            vec![vec![source_object, 0, 5, 0x7000]]
        );
        assert_eq!(calls(&e, BUILD_TREE), vec![vec![mgr + 0x30, tree]]);
    }

    #[test]
    fn build_kd_tree_resets_an_existing_tree() {
        let mut e = engine();
        install_router(&mut e);
        install_build_doubles(&mut e);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        capture_info(&mut e, KD_TREE_RESET, 1, &seen);
        let (mgr, world, array) = build_setup(&mut e, 3);
        let slot = put_words(&mut e, &[0x6000]);
        e.call(0x00d0_94a0, &args![mgr, slot, array, world, 0u32, 0u32]);
        let info = calls(&e, KD_INFO_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, KD_TREE_RESET), vec![vec![0x6000, info]]);
        assert!(calls(&e, TAKE).is_empty());
        assert_eq!(*seen.borrow(), vec![5, 0]);
        assert_eq!(e.mem.u32(slot), 0x6000);
        assert_eq!(calls(&e, BUILD_TREE), vec![vec![mgr + 0x30, 0x6000]]);
    }

    #[test]
    fn build_kd_tree_stores_null_when_the_tree_block_cannot_be_taken() {
        let mut e = engine();
        install_router_with(&mut e, 0x16, 0x50);
        install_build_doubles(&mut e);
        let (mgr, world, array) = build_setup(&mut e, 3);
        let slot = e.mem.alloc(4);
        e.call(0x00d0_94a0, &args![mgr, slot, array, world, 0u32, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls(&e, KD_TREE_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, BUILD_TREE), vec![vec![mgr + 0x30, 0]]);
    }

    #[test]
    fn build_kd_tree_stays_on_this_thread_for_few_collidables_or_a_missing_queue() {
        for (queue, pool, size) in [
            (0x5000u32, 0x5100u32, 0x1fu32),
            (0, 0x5100, 0x40),
            (0x5000, 0, 0x40),
        ] {
            let mut e = engine();
            install_router(&mut e);
            install_build_doubles(&mut e);
            let (mgr, world, array) = build_setup(&mut e, size);
            let slot = put_words(&mut e, &[0x6000]);
            e.call(0x00d0_94a0, &args![mgr, slot, array, world, queue, pool]);
            assert_eq!(calls(&e, SOURCE_CONSTRUCT).len(), 1);
            assert_eq!(calls(&e, PROJECT_PRIMITIVES).len(), 1);
        }
    }

    /// Everything the distributed build leaves to inspect.
    struct Distributed {
        e: Engine,
        router: Router,
        stream: u32,
        tree: u32,
        sync_block: u32,
        job: Vec<u8>,
        info_seen: Words,
        array_data: u32,
    }

    impl Distributed {
        fn word(&self, offset: usize) -> u32 {
            u32::from_le_bytes(self.job[offset..offset + 4].try_into().unwrap())
        }
    }

    const QUEUE: u32 = 0x0000_5000;
    const POOL_SLOT_PROCESS: u32 = 0xc;
    const POOL_SLOT_WAIT: u32 = 0x10;

    /// Runs the distributed build on an existing tree with the scratch allocator granting
    /// `granted` bytes.
    fn run_distributed(granted: u32) -> Distributed {
        let mut e = engine();
        let router = install_router_with(&mut e, granted, 0);
        let stream = monitor_stream(&mut e, true);
        let info_seen: Words = Rc::new(RefCell::new(Vec::new()));
        capture_info(&mut e, KD_TREE_RESET, 1, &info_seen);
        nothing(&mut e, KD_INFO_CONSTRUCT);
        let (mgr, world, array) = build_setup(&mut e, 0x20);
        let array_data = e.mem.alloc(0x100);
        e.mem.set_u32(array, array_data);
        e.mem.set_u32(world + 0x78, 0x444);
        let sync_block = e.mem.alloc(0x10);
        e.mem.set_u32(mgr + 0x3c, sync_block);
        e.mem.set_u32(mgr + 0x34, 0x222);
        e.mem.set_u32(mgr + 0x6c, 0x333);
        let tree = e.mem.alloc(0x40);
        for (offset, value) in [
            (0, 0xa0),
            (0x10, 0xa1),
            (0x14, 0xa2),
            (0x18, 0xa3),
            (0x1c, 0xa4),
        ] {
            e.mem.set_u32(tree + offset, value);
        }
        let slot = put_words(&mut e, &[tree]);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + POOL_SLOT_PROCESS, POOL_PROCESS);
        e.mem.set_u32(table + POOL_SLOT_WAIT, POOL_WAIT);
        let pool = e.mem.alloc(0x10);
        e.mem.set_u32(pool, table);
        let job: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let job_copy = Rc::clone(&job);
        e.register_double(QUEUE_ADD_JOB, move |e, a| {
            *job_copy.borrow_mut() = e.mem.bytes(a[1], 0x60);
            Ret::default()
        });
        for address in [
            QUEUE_FINISH,
            SEMAPHORE_WAIT,
            FINISH_TREE,
            POOL_PROCESS,
            POOL_WAIT,
        ] {
            nothing(&mut e, address);
        }
        e.call(0x00d0_94a0, &args![mgr, slot, array, world, QUEUE, pool]);
        let job = job.borrow().clone();
        Distributed {
            e,
            router,
            stream,
            tree,
            sync_block,
            job,
            info_seen,
            array_data,
        }
    }

    #[test]
    fn build_kd_tree_distributes_the_build_over_the_job_queue() {
        let run = run_distributed(0x16);
        let e = &run.e;
        // The info: the collidable count and the multi-threaded byte 1; the tree is reused.
        assert_eq!(*run.info_seen.borrow(), vec![0x20, 1]);
        let info = calls(e, KD_INFO_CONSTRUCT)[0][0];
        assert_eq!(calls(e, KD_TREE_RESET), vec![vec![run.tree, info]]);
        assert_eq!(e.mem.u32(run.sync_block), 4);
        // The job record.
        assert_eq!(run.job[..3], [6, 3, 1]);
        assert_eq!(&run.job[4..8], &[0x60, 0, 0xff, 0xff]);
        let (block_1, block_2) = (run.word(0x30), run.word(0x34));
        assert_ne!(block_1, 0);
        assert_ne!(block_2, 0);
        for (offset, expected) in [
            (0x10, 0x333),
            (0x14, run.sync_block),
            (0x18, 0),
            (0x1c, 0x444),
            (0x20, 0x7000),
            (0x24, 0x222),
            (0x28, 0xa0),
            (0x2c, 0xa4),
            (0x38, 0xa1),
            (0x3c, 0xa2 + 0xa3),
            (0x40, 0xa3),
            (0x50, run.array_data),
        ] {
            assert_eq!(run.word(offset), expected, "job word {offset:#x}");
        }
        let queue_calls = calls(e, QUEUE_ADD_JOB);
        assert_eq!(queue_calls.len(), 1);
        assert_eq!((queue_calls[0][0], queue_calls[0][2]), (QUEUE, 0));
        let pool = calls(e, POOL_PROCESS)[0][0];
        assert_eq!(calls(e, POOL_PROCESS), vec![vec![pool, QUEUE, 3]]);
        assert_eq!(calls(e, POOL_WAIT), vec![vec![pool]]);
        assert_eq!(calls(e, QUEUE_FINISH), vec![vec![QUEUE]]);
        assert_eq!(calls(e, SEMAPHORE_WAIT), vec![vec![0x333]]);
        assert_eq!(
            calls(e, FINISH_TREE),
            vec![vec![run.tree, 0x20, block_1, block_2]]
        );
        // Two scratch blocks of 0x10 bytes, granted 0x16: capacity 5, freed (second first).
        assert_eq!(calls(e, TAKE_SIZED).len(), 2);
        assert_eq!(
            calls(e, FREE_DATA),
            vec![
                vec![run.router.allocator, block_2, 20],
                vec![run.router.allocator, block_1, 20]
            ]
        );
        assert_eq!(
            record_labels(e, run.stream),
            vec![
                LABEL_BUILD_KD_TREE_MT,
                LABEL_BUILD,
                LABEL_BUILD_TREE_DISTRIBUTED,
                LABEL_START,
                LABEL_WAIT_FOR_COMPLETION,
                LABEL_WAIT_SEMAPHORE,
                LABEL_FINISH_PPU,
                LABEL_SPLIT_END,
                LABEL_FINALIZE,
                LABEL_SPLIT_END
            ]
        );
        assert_eq!(e.mem.u32(run.stream + 0x40 + 0xc), LABEL_PREPARE);
        assert_eq!(e.mem.u32(run.stream + 0x40 + 0x10), LABEL_BUILD);
    }

    #[test]
    fn build_kd_tree_does_not_free_scratch_blocks_with_no_capacity() {
        let run = run_distributed(3);
        assert!(calls(&run.e, FREE_DATA).is_empty());
    }

    #[test]
    fn fn_00d099a0_builds_only_for_a_non_empty_active_list() {
        let mut e = engine();
        install_router(&mut e);
        install_build_doubles(&mut e);
        let (mgr, world, array) = build_setup(&mut e, 1);
        e.call(0x00d0_99a0, &args![mgr, array, 0u32, 0u32]);
        assert!(calls(&e, SOURCE_CONSTRUCT).is_empty());
        e.mem.set_u32(mgr + 0x48, 2);
        e.call(0x00d0_99a0, &args![mgr, array, 0u32, 0u32]);
        let source = calls(&e, SOURCE_CONSTRUCT);
        assert_eq!(source.len(), 1);
        assert_eq!(source[0][1..], [array, world]);
        // The tree is created in the manager's kd-tree slot.
        assert_ne!(e.mem.u32(mgr + 0x10), 0);
    }

    // --- building the inactive tree -------------------------------------------------------

    /// A collidable with the integer AABB `min` at `+0x20` and `max` at `+0x30`.
    fn box_collidable(e: &mut Engine, min: [u32; 3], max: [u32; 3]) -> u32 {
        let collidable = e.mem.alloc(0x60);
        for (i, value) in min.into_iter().enumerate() {
            e.mem.set_u32(collidable + 0x20 + 4 * i as u32, value);
        }
        for (i, value) in max.into_iter().enumerate() {
            e.mem.set_u32(collidable + 0x30 + 4 * i as u32, value);
        }
        collidable
    }

    /// The float constants of the conversion: `2^32`, lane 3 of the bounds (2.5) and lane 3
    /// of the reciprocal fill (1.0).
    fn set_bound_constants(e: &mut Engine) {
        e.mem.set_f32(UNSIGNED_BIAS, 4_294_967_296.0);
        e.mem.set_f32(BOUNDS_W, 2.5);
        e.mem.set_f32(RECIPROCAL_FILL + 12, 1.0);
    }

    /// Gives the world the bounds data (at `+0x78`) with the origin `(1, 2, 3, 0.5)` at `+0x20`
    /// and the scale `(2, 4, 8, 99)` at `+0x40`.
    fn set_bounds(e: &mut Engine, world: u32) {
        let data = e.mem.alloc(0x80);
        store_vec(e, data + 0x20, [1.0, 2.0, 3.0, 0.5]);
        store_vec(e, data + 0x40, [2.0, 4.0, 8.0, 99.0]);
        e.mem.set_u32(world + 0x78, data);
    }

    /// The bulk insertion double: records the AABB slots (`8 * n` words) and the pointers
    /// (`n` words) it is given, answers the handles `0x11, 0x22, ...` in a freshly taken
    /// array (owned, capacity 4) and returns the block's address through `block`.
    fn install_insert_all(e: &mut Engine, seen: &Words, block: &Words) {
        let seen = Rc::clone(seen);
        let block = Rc::clone(block);
        e.register_double(AABB_TREE_INSERT_ALL, move |e, a| {
            let count = a[3];
            seen.borrow_mut().extend(read_words(e, a[1], 8 * count));
            seen.borrow_mut().extend(read_words(e, a[2], count));
            let handles = e.mem.alloc(2 * count + 2);
            for i in 0..count {
                e.mem.set_u16(handles + 2 * i, 0x11 * (i as u16 + 1));
            }
            block.borrow_mut().push(handles);
            e.mem.set_u32(a[4], handles);
            e.mem.set_u32(a[4] + 4, count);
            e.mem.set_u32(a[4] + 8, 4);
            Ret::default()
        });
    }

    fn assert_close(actual: &[u32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        for (i, (bits, want)) in actual.iter().zip(expected).enumerate() {
            let got = f32::from_bits(*bits);
            assert!(
                (got - want).abs() <= 1e-4 * want.abs().max(1.0),
                "lane {i}: {got} vs {want}"
            );
        }
    }

    #[test]
    fn build_inactive_tree_converts_boxes_asks_shapes_and_stores_the_handles() {
        let mut e = engine();
        let router = install_router(&mut e);
        let stream = monitor_stream(&mut e, true);
        nothing(&mut e, SET_TREE_DATA);
        set_bound_constants(&mut e);
        let mgr = manager(&mut e);
        let world = e.mem.alloc(0x100);
        set_bounds(&mut e, world);
        e.mem.set_u32(mgr + 0x28, world);
        e.mem.set_u32(mgr + 0xc, 0x5000);
        let region = e.mem.alloc(0x400);
        e.mem.set_u32(router.router, region);
        let ordered = box_collidable(&mut e, [10, 20, 30], [40, 60, 80]);
        let unordered = box_collidable(&mut e, [100, 0, 0], [50, 0, 0]);
        let negative = box_collidable(&mut e, [0xffff_fff0, 0, 0], [0xffff_fff8, 0, 0]);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, SHAPE_AABB);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, table);
        e.mem.set_u32(unordered, shape);
        e.mem.set_u32(unordered + 8, 0x777);
        e.register_double(SHAPE_AABB, |e, a| {
            for i in 0..8 {
                e.mem.set_u32(a[3] + 4 * i, 0xa000_0000 + i);
            }
            Ret::default()
        });
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let handles: Words = Rc::new(RefCell::new(Vec::new()));
        install_insert_all(&mut e, &seen, &handles);
        let array = e.mem.alloc(0x10);
        let data = put_words(&mut e, &[ordered, unordered, negative]);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.call(0x00d0_9020, &args![mgr, array, 0u32, 0u32]);
        let seen = seen.borrow();
        // The first slot: min * (1 / scale) - origin, then max likewise.
        assert_close(&seen[0..8], &[4.0, 3.0, 0.75, 2.0, 19.0, 13.0, 7.0, 2.0]);
        // The second slot is whatever the shape wrote.
        assert_eq!(
            seen[8..16],
            (0..8).map(|i| 0xa000_0000 + i).collect::<Vec<u32>>()
        );
        // The third slot: the unsigned minimum 0xfffffff0 becomes 2^32 as a float.
        assert_close(
            &seen[16..24],
            &[
                2_147_483_648.0,
                -2.0,
                -3.0,
                2.0,
                2_147_483_648.0,
                -2.0,
                -3.0,
                2.0,
            ],
        );
        // The pointers array holds the collidables.
        assert_eq!(seen[24..27], [ordered, unordered, negative]);
        // The AABB slots start at the old stack top and the pointers 0x80 bytes later.
        let insert = calls(&e, AABB_TREE_INSERT_ALL).remove(0);
        assert_eq!(insert[..4], [0x5000, region, region + 0x80, 3]);
        assert_eq!(
            calls(&e, SHAPE_AABB),
            vec![vec![shape, 0x777, 0, region + 0x20]]
        );
        let handle_block = handles.borrow()[0];
        assert_eq!(
            calls(&e, SET_TREE_DATA),
            vec![
                vec![ordered, 0x11],
                vec![unordered, 0x22],
                vec![negative, 0x33]
            ]
        );
        // The handle array (capacity 4 owned) is freed; the stack arrays only give back the top.
        assert_eq!(
            calls(&e, FREE_DATA),
            vec![vec![router.allocator, handle_block, 8]]
        );
        assert_eq!(e.mem.u32(router.router), region);
        assert_eq!(
            record_labels(&e, stream),
            vec![LABEL_TREE_PREPARE, LABEL_END, LABEL_TREE_BUILD, LABEL_END]
        );
    }

    #[test]
    fn build_inactive_tree_with_nothing_to_insert_only_records_and_restores_the_top() {
        let mut e = engine();
        let router = install_router(&mut e);
        let stream = monitor_stream(&mut e, true);
        let mgr = manager(&mut e);
        let world = e.mem.alloc(0x100);
        set_bounds(&mut e, world);
        e.mem.set_u32(mgr + 0x28, world);
        let region = e.mem.alloc(0x100);
        e.mem.set_u32(router.router, region);
        let array = e.mem.alloc(0x10);
        e.call(0x00d0_9020, &args![mgr, array, 0u32, 0u32]);
        assert_eq!(e.mem.u32(router.router), region);
        assert!(calls(&e, FREE_DATA).is_empty());
        assert_eq!(record_labels(&e, stream).len(), 4);
    }

    // --- incremental and full builds ------------------------------------------------------

    /// What the incremental build's tests share: stream, router with a stack region,
    /// doubles, a world and a flagged manager whose active list is `[a, b, c]` (flag bytes
    /// 1, 1, 0; `b` has gone inactive).
    struct Incremental {
        e: Engine,
        router: Router,
        stream: u32,
        region: u32,
        mgr: u32,
        world: u32,
        items: Vec<u32>,
    }

    fn incremental_setup() -> Incremental {
        let mut e = engine();
        let router = install_router(&mut e);
        let stream = monitor_stream(&mut e, true);
        install_build_doubles(&mut e);
        install_tree_data(&mut e);
        let world = empty_world(&mut e);
        let region = e.mem.alloc(0x400);
        e.mem.set_u32(router.router, region);
        let items = scan_setup(&mut e, &[1, 1, 0], &[1]);
        let mgr = manager(&mut e);
        e.mem.set_u8(mgr + 0x24, 1);
        e.mem.set_u32(mgr + 0x28, world);
        e.mem.set_u32(mgr + 0xc, 0x5000);
        set_active_list(&mut e, mgr, &items, 3);
        Incremental {
            e,
            router,
            stream,
            region,
            mgr,
            world,
            items,
        }
    }

    #[test]
    fn incremental_build_moves_inactivated_collidables_to_the_inactive_tree() {
        let mut run = incremental_setup();
        let e = &mut run.e;
        let mgr = run.mgr;
        e.mem.set_u8(mgr + 0x69, 1);
        e.mem.set_u8(mgr + 0x5c, 1);
        // The collidable that went inactive has a shape.
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, SHAPE_AABB);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, table);
        e.mem.set_u32(run.items[1], shape);
        e.mem.set_u32(run.items[1] + 8, 0x777);
        nothing(e, SHAPE_AABB);
        returning(e, AABB_TREE_INSERT, 0xabcd_1234);
        nothing(e, AABB_TREE_REMOVE);
        // The pending map: slot 1 holds a key with value 1 that has tree data, slot 3 a key
        // with value -1.
        let key_1 = e.mem.alloc(0x40);
        e.mem.set_u8(key_1 + 0x18, 1);
        let key_3 = e.mem.alloc(0x40);
        let slots = put_words(
            e,
            &[0xffff_ffff, 0, key_1, 1, 0xffff_ffff, 0, key_3, 0xffff_ffff],
        );
        e.mem.set_u32(mgr + 0x50, slots);
        e.mem.set_u32(mgr + 0x58, 3);
        e.register_double(GET_TREE_DATA, move |_, a| Ret {
            eax: if a[0] == key_1 { 5 } else { 0xffff },
            ..Ret::default()
        });
        e.call(0x00d0_99d0, &args![mgr, 0u32, 0u32]);
        assert_eq!(calls(e, OWNER_ACTIVITY_STATE).len(), 2);
        let shape_calls = calls(e, SHAPE_AABB);
        assert_eq!(shape_calls.len(), 1);
        assert_eq!(shape_calls[0][..3], [shape, 0x777, 0]);
        assert_eq!(
            calls(e, AABB_TREE_INSERT),
            vec![vec![0x5000, shape_calls[0][3], run.items[1]]]
        );
        assert_eq!(
            calls(e, SET_TREE_DATA),
            vec![vec![run.items[1], 0x1234], vec![key_1, 0xffff]]
        );
        assert_eq!(
            calls(e, GET_TREE_DATA),
            vec![vec![key_1], vec![key_1], vec![key_3]]
        );
        assert_eq!(calls(e, AABB_TREE_REMOVE), vec![vec![0x5000, 5]]);
        // The active list: the inactive entry gone, the re-activated key appended.
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 3);
        assert_eq!(
            read_words(e, data, 3),
            vec![run.items[0], run.items[2], key_1]
        );
        // The kd-tree is rebuilt from the list, the map cleared, the flag set again.
        let source = calls(e, SOURCE_CONSTRUCT);
        assert_eq!(source.len(), 1);
        assert_eq!(source[0][1..], [mgr + 0x44, run.world]);
        assert_eq!(calls(e, MAP_CLEAR), vec![vec![mgr + 0x50]]);
        assert_eq!(e.mem.u8(mgr + 0x69), 1);
        assert_eq!(e.mem.u32(run.router.router), run.region);
        assert_eq!(
            record_labels(e, run.stream),
            vec![
                LABEL_INCREMENTAL_BUILD,
                LABEL_FIND_INACTIVATED,
                LABEL_UPDATE_INACTIVE_TREE,
                LABEL_ADDED_NODES,
                LABEL_FIND_ACTIVATED,
                LABEL_REMOVED_NODES,
                LABEL_UPDATE_ACTIVE,
                LABEL_SPLIT_END
            ]
        );
    }

    #[test]
    fn incremental_build_without_the_valid_flag_skips_the_inactive_scan() {
        let mut run = incremental_setup();
        let e = &mut run.e;
        let mgr = run.mgr;
        e.mem.set_u32(mgr + 0x58, 0xffff_ffff);
        e.call(0x00d0_99d0, &args![mgr, 0u32, 0u32]);
        assert!(calls(e, OWNER_ACTIVITY_STATE).is_empty());
        assert_eq!(e.mem.u8(mgr + 0x69), 0);
        assert_eq!(e.mem.u32(mgr + 0x48), 3);
        assert_eq!(calls(e, SOURCE_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(run.router.router), run.region);
        assert_eq!(
            record_labels(e, run.stream),
            vec![
                LABEL_INCREMENTAL_BUILD,
                LABEL_FIND_INACTIVATED,
                LABEL_FIND_ACTIVATED,
                LABEL_UPDATE_ACTIVE,
                LABEL_SPLIT_END
            ]
        );
    }

    #[test]
    fn incremental_build_in_single_tree_mode_regathers_and_rebuilds_the_kd_tree() {
        let mut run = incremental_setup();
        let e = &mut run.e;
        let mgr = run.mgr;
        e.mem.set_u8(mgr + 0x24, 0);
        e.mem.set_u8(mgr + 0x69, 1);
        e.call(0x00d0_99d0, &args![mgr, 0u32, 0u32]);
        // The world is empty: the list is cleared and gathered again (nothing), the tree is
        // built from it, and the "valid" byte is cleared by the map clear.
        assert_eq!(e.mem.u32(mgr + 0x48), 0);
        let source = calls(e, SOURCE_CONSTRUCT);
        assert_eq!(source.len(), 1);
        assert_eq!(source[0][1..], [mgr + 0x44, run.world]);
        assert_eq!(e.mem.u8(mgr + 0x69), 0);
        assert_eq!(e.mem.u32(run.router.router), run.region);
        assert_eq!(
            record_labels(e, run.stream),
            vec![
                LABEL_DYNAMIC_BUILD,
                LABEL_CLEAR,
                LABEL_REBUILD,
                LABEL_SPLIT_END
            ]
        );
    }

    #[test]
    fn full_build_gathers_everything_into_the_kd_tree_in_single_tree_mode() {
        let mut e = engine();
        install_router(&mut e);
        let stream = monitor_stream(&mut e, true);
        install_reserve_more(&mut e);
        install_build_doubles(&mut e);
        install_tree_data(&mut e);
        let (world, ents) = gathering_world(&mut e);
        give_world_object(&mut e, world);
        let mgr = manager(&mut e);
        e.mem.set_u32(mgr + 0x28, world);
        e.mem.set_u32(mgr + 0x48, 7);
        e.call(0x00d0_9df0, &args![mgr, 0u32, 0u32]);
        let expected: Vec<u32> = [0, 1, 2, 3, 5, 6, 7, 8, 10]
            .map(|i| ents[i] + 0x10)
            .to_vec();
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 9);
        assert_eq!(read_words(&e, data, 9), expected);
        let cleared: Vec<Vec<u32>> = expected.iter().map(|c| vec![*c, 0xffff]).collect();
        assert_eq!(calls(&e, SET_TREE_DATA), cleared);
        let source = calls(&e, SOURCE_CONSTRUCT);
        assert_eq!(source.len(), 1);
        assert_eq!(source[0][1..], [mgr + 0x44, world]);
        assert_eq!(calls(&e, MAP_CLEAR), vec![vec![mgr + 0x50]]);
        assert_eq!(
            record_labels(&e, stream),
            vec![
                LABEL_ACTIVE_TREE,
                LABEL_CLEAR,
                LABEL_REBUILD,
                LABEL_SPLIT_END
            ]
        );
        assert_eq!(e.mem.u32(stream + 0x40 + 0xc), LABEL_GATHER);
    }

    #[test]
    fn full_build_with_an_inactive_tree_gathers_the_inactive_collidables_separately() {
        let mut e = engine();
        let router = install_router(&mut e);
        let stream = monitor_stream(&mut e, true);
        install_reserve_more(&mut e);
        install_build_doubles(&mut e);
        install_tree_data(&mut e);
        set_bound_constants(&mut e);
        nothing(&mut e, AABB_TREE_CALL);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let handles: Words = Rc::new(RefCell::new(Vec::new()));
        install_insert_all(&mut e, &seen, &handles);
        let (world, ents) = gathering_world(&mut e);
        give_world_object(&mut e, world);
        let region = e.mem.alloc(0x1000);
        e.mem.set_u32(router.router, region);
        let mgr = manager(&mut e);
        set_bounds(&mut e, world);
        e.mem.set_u32(mgr + 0x28, world);
        e.mem.set_u8(mgr + 0x24, 1);
        e.mem.set_u32(mgr + 0xc, 0x5000);
        e.call(0x00d0_9df0, &args![mgr, 0u32, 0u32]);
        let inactive: Vec<u32> = [6, 7, 8, 10].map(|i| ents[i] + 0x10).to_vec();
        let active: Vec<u32> = [0, 1, 2, 3, 5].map(|i| ents[i] + 0x10).to_vec();
        let data = e.mem.u32(mgr + 0x44);
        assert_eq!(e.mem.u32(mgr + 0x48), 5);
        assert_eq!(read_words(&e, data, 5), active);
        assert_eq!(calls(&e, AABB_TREE_CALL), vec![vec![0x5000]]);
        // The inserted pointers are the inactive collidables.
        assert_eq!(seen.borrow()[32..], inactive[..]);
        let handle_block = handles.borrow()[0];
        let mut expected_sets: Vec<Vec<u32>> = active.iter().map(|c| vec![*c, 0xffff]).collect();
        expected_sets.extend(
            inactive
                .iter()
                .zip([0x11, 0x22, 0x33, 0x44])
                .map(|(c, h)| vec![*c, h]),
        );
        assert_eq!(calls(&e, SET_TREE_DATA), expected_sets);
        // The handle array and the gathering array (capacity 4, owned) are freed.
        let frees = calls(&e, FREE_DATA);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0], vec![router.allocator, handle_block, 8]);
        assert_eq!((frees[1][0], frees[1][2]), (router.allocator, 16));
        assert_eq!(e.mem.u32(router.router), region);
        assert_eq!(calls(&e, SOURCE_CONSTRUCT).len(), 1);
        assert_eq!(
            record_labels(&e, stream),
            vec![
                LABEL_ACTIVE_TREE,
                LABEL_CLEAR,
                LABEL_REBUILD,
                LABEL_SPLIT_END,
                LABEL_INACTIVE_TREE,
                LABEL_CLEAR,
                LABEL_REBUILD,
                LABEL_TREE_PREPARE,
                LABEL_END,
                LABEL_TREE_BUILD,
                LABEL_END,
                LABEL_SPLIT_END
            ]
        );
    }

    #[test]
    fn update_from_world_does_a_full_build_first_and_incremental_builds_after() {
        let mut e = engine();
        let router = install_router(&mut e);
        let stream = monitor_stream(&mut e, true);
        install_build_doubles(&mut e);
        install_tree_data(&mut e);
        let world = empty_world(&mut e);
        let region = e.mem.alloc(0x400);
        e.mem.set_u32(router.router, region);
        let mgr = manager(&mut e);
        e.mem.set_u32(mgr + 0x28, world);
        e.call(0x00d0_a060, &args![mgr, 0u32, 0u32]);
        assert_eq!(e.mem.u8(mgr + 0x2c), 1);
        assert_eq!(e.mem.u8(mgr + 0x68), 1);
        assert_eq!(
            record_labels(&e, stream),
            vec![
                LABEL_TREE_BUILD_TOTAL,
                LABEL_ACTIVE_TREE,
                LABEL_CLEAR,
                LABEL_REBUILD,
                LABEL_SPLIT_END,
                LABEL_END
            ]
        );
        e.call(0x00d0_a060, &args![mgr, 0u32, 0u32]);
        assert_eq!(
            record_labels(&e, stream)[6..],
            [
                LABEL_TREE_BUILD_TOTAL,
                LABEL_DYNAMIC_BUILD,
                LABEL_CLEAR,
                LABEL_REBUILD,
                LABEL_SPLIT_END,
                LABEL_END
            ]
        );
        assert_eq!(e.mem.u32(router.router), region);
    }
}
