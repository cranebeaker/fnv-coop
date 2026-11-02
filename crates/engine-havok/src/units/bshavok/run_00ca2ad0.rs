//! `BSHavok/run_00ca2ad0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00ca2ad0` is a method of `bhkRagdollPenetrationUtil` (its neighbours in
//! `bhkragdollpenetrationutil.rs`; the Xbox PDB gives it no name, so none is claimed). It
//! rebuilds the util's array of `ConstrainedSystem` records (0x24 bytes each: root node,
//! `hkaRagdollInstance`, `hkaSkeleton`, bounding phantom, penetration detector, output
//! array) for a scene-graph root: systems that still describe the same ragdoll are kept,
//! the others are destroyed and new ones are built.
//!
//! The function's frame holds four small arrays that the callees fill through their
//! addresses, so the translation keeps them in a scratch block at the exe's own offsets:
//! the two local `hkArray`s `bodies` (`+0x28`) and `parents` (`+0x38`; data, size,
//! capacity-and-flags, and the start of the 0x100 bytes they take from the `hkMemoryRouter`
//! stack), the `BSSimpleArray`s `kept` (`+0x48`) and `nodes` (`+0x58`) and the `extras`
//! list (`+0x68`; vtable `0104e190`). Floats: the bounding box is accumulated with the
//! `MAXPS`/`MINPS` lane rules; the timer limit is computed on the x87 stack like
//! `fn_00ca0cd0`'s. The `FS:[0]` exception frame and the `/GS` cookie check (`00ec408c`)
//! are not modeled, so the function is marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, maxps, minps, store_vec, zip_with, Vec4};

use super::bhkentity::{HEAP_ALLOCATE, MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, MEMORY_ROUTER};

/// Methods of the util used here (all thiscall on the util unless stated): `00ca0f00`
/// `(node, &list)` collects the nodes, `00ca1f00` `(node, &bodies, &parents)` gathers the
/// rigid bodies, `00ca1080` `(&bodies, &extras)` collects the extra shapes, `00ca21c0`
/// `(&bodies, &parents, &extras)` is `CreateSkeletonForRagdoll`, `00ca2020` destroys a
/// system (`this` = the system) and `00ca0e60` (`this` = the source array, `(target)`)
/// copies a list into the util's `ActiveSystems`.
const COLLECT_NODES: u32 = 0x00ca_0f00;
const GATHER_BODIES: u32 = 0x00ca_1f00;
const COLLECT_EXTRAS: u32 = 0x00ca_1080;
const CREATE_SKELETON: u32 = 0x00ca_21c0;
const DESTROY_SYSTEM: u32 = 0x00ca_2020;
const COPY_SYSTEMS: u32 = 0x00ca_0e60;
/// `hkaRagdollUtils::reorderForRagdoll` (Xbox PDB, cdecl `(&bodies, &parents)`, 1 means it
/// failed) and `hkaRagdollUtils::createRagdollInstanceFromSkeleton` (Xbox PDB, cdecl
/// `(skeleton, &bodies, &parents)`).
const REORDER_FOR_RAGDOLL: u32 = 0x00ce_a030;
const CREATE_RAGDOLL_INSTANCE: u32 = 0x00ce_a810;
/// `bhkAabbPhantom::bhkAabbPhantom` (Xbox PDB, `(cinfo)`), the detector's constructor
/// (`00d1fa50`, `(&info)`) and its reset (`00d1fa20`).
const PHANTOM_CONSTRUCT: u32 = 0x0056_e550;
const DETECTOR_CONSTRUCT: u32 = 0x00d1_fa50;
const DETECTOR_RESET: u32 = 0x00d1_fa20;
/// The array step that makes room for one more element and answers its index
/// (`00b5c360`, `this` = the array), the reallocating helper (`00cb9ce0`, `(capacity,
/// size)`) and a bare `RET` called around the collision-filter stores (`00a29680`).
const ARRAY_APPEND: u32 = 0x00b5_c360;
const ARRAY_REALLOC: u32 = 0x00cb_9ce0;
const NO_OP: u32 = 0x00a2_9680;
/// `BSPrecisionTimer::GetTimer` (Xbox PDB), the `ST0` to 64-bit conversion and
/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the manager, `(block)`).
const GET_TIMER: u32 = 0x00aa_4d80;
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The vtables of the three list objects in the frame; the slot at `+4` allocates `(count)`
/// and the one at `+8` frees `(block)`.
const NODES_VTABLE: u32 = 0x0107_1aec;
const KEPT_VTABLE: u32 = 0x010c_7310;
const EXTRAS_VTABLE: u32 = 0x0104_e190;
/// The bounding box accumulator starts: the lower bound with the float at `0104da48` in all
/// four lanes, the upper bound with the one at `0104e148`.
const LOWER_START: u32 = 0x0104_da48;
const UPPER_START: u32 = 0x0104_e148;
/// The detection time budget: seconds (`012682b4`) times the double 1000.0 (`01017b70`),
/// scaled to ticks by `011ac39c`.
const BUDGET_SECONDS: u32 = 0x0126_82b4;
const MILLISECONDS: u32 = 0x0101_7b70;
const TICK_SCALE: u32 = 0x011a_c39c;

/// Scratch offsets, the exe's own.
const BODIES: u32 = 0x28;
const PARENTS: u32 = 0x38;
const KEPT: u32 = 0x48;
const NODES: u32 = 0x58;
const EXTRAS: u32 = 0x68;
const DETECTOR_INFO: u32 = 0x78;
const PHANTOM_INFO: u32 = 0x94;
const BOUNDS: u32 = 0xe4;
const SCRATCH_SIZE: u32 = 0x110;

/// Opens a local `hkArray` of the frame: no data, size 0, capacity word `0x80000000`, then
/// takes 0x100 bytes from the `hkMemoryRouter` stack (the router's first word is the
/// cursor) as its buffer for 0x40 elements, and remembers where they start (`+0xC`).
fn open_frame_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let start = e.mem.u32(router);
    e.mem.set_u32(router, start.wrapping_add(0x100));
    e.mem.set_u32(array, start);
    e.mem.set_u32(array + 8, 0x8000_0040);
    e.mem.set_u32(array + 0xc, start);
}

/// Closes a local `hkArray`: the size is cleared when the data is still the router stack
/// buffer, the router cursor goes back to the start, a buffer the array owns (capacity
/// word not negative) is freed through the allocator's slot `0x10` as
/// `(data, 4 * capacity)`, and the array is left empty.
fn close_frame_array(e: &mut Engine, array: u32) {
    let start = e.mem.u32(array + 0xc);
    if start == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, start);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Sets a `BSSimpleArray`-like list of the frame (vtable, data, size, capacity) to empty.
fn open_list(e: &mut Engine, list: u32, vtable: u32) {
    e.mem.set_u32(list, vtable);
    e.mem.set_u32(list + 4, 0);
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list + 0xc, 0);
}

/// Destroys a list of the frame: stores the vtable and, when there is a buffer, frees it
/// through the vtable's slot `8` (called at its static address) and, with `clear`, resets
/// the buffer, size and capacity.
fn close_list(e: &mut Engine, list: u32, vtable: u32, clear: bool) {
    e.mem.set_u32(list, vtable);
    let data = e.mem.u32(list + 4);
    if data != 0 {
        let free = e.mem.u32(vtable + 8);
        e.call(free, &args![list, data]);
        if clear {
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 0xc, 0);
            e.mem.set_u32(list + 8, 0);
        }
    }
}

/// Appends `value` to the `kept` list the way the exe does inline: when the list is full
/// an empty one allocates 4 items (slot `4`) and a used one grows by `00cb9ce0` (doubling,
/// plus 0x400 above 0x400).
fn append_kept(e: &mut Engine, list: u32, value: u32) {
    let size = e.mem.u32(list + 8);
    let capacity = e.mem.u32(list + 0xc);
    if size == capacity {
        if capacity == 0 {
            let data = e.vcall(list, 4, &args![4u32]).u32();
            e.mem.set_u32(list + 4, data);
            e.mem.set_u32(list + 0xc, 4);
        } else {
            let grown = if capacity > 0x400 {
                capacity.wrapping_add(0x400)
            } else {
                capacity.wrapping_mul(2)
            };
            e.call(ARRAY_REALLOC, &args![list, grown, size]);
            e.mem.set_u32(list + 0xc, grown);
        }
    }
    let size = e.mem.u32(list + 8).wrapping_add(1);
    e.mem.set_u32(list + 8, size);
    let data = e.mem.u32(list + 4);
    e.mem.set_u32(
        data.wrapping_add(4u32.wrapping_mul(size.wrapping_sub(1))),
        value,
    );
}

/// Destroys one `ConstrainedSystem` and gives its block back to the memory manager.
fn discard_system(e: &mut Engine, system: u32) {
    e.call(DESTROY_SYSTEM, &args![system]);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, system]);
}

/// The lanes of one float in all four.
fn splat(value: f32) -> Vec4 {
    [value; 4]
}

/// The second half of building a system whose `hkaRagdollInstance` exists: the bounding
/// box of the bodies' shapes, the `bhkAabbPhantom` over it (filter info `group << 16 |
/// 0x1E`, its Havok world set from the first body's wrapper through virtual slot `0x9C`),
/// the penetration detector (a 0x28-byte block from the allocator's slot `4`, built from
/// the skeleton, the util and the phantom's word at `+8`, then reset) and the util's
/// detection timer (start now, limit `seconds * 1000.0` scaled to ticks).
fn attach_phantom_and_detector(e: &mut Engine, this: u32, frame: u32, system: u32) {
    let bodies = frame + BODIES;
    let instance = e.mem.u32(system + 4);
    let first_list = e.mem.u32(instance + 8);
    let first_body = e.mem.u32(first_list);
    let owner = if first_body != 0 {
        e.mem.u32(first_body + 8)
    } else {
        0
    };
    let mut upper = splat(e.global::<f32>(UPPER_START));
    let mut lower = splat(e.global::<f32>(LOWER_START));
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(bodies + 4) {
        let body = e.mem.u32(e.mem.u32(bodies) + 4 * index);
        if e.mem.u32(body + 0x10) != 0 {
            let shape_owner = e.mem.u32(owner + 0x58);
            e.vcall(shape_owner, 0x3c, &args![body + 0x24, frame + BOUNDS]);
            let box_lower = load_vec(e, frame + BOUNDS);
            let box_upper = load_vec(e, frame + BOUNDS + 0x10);
            upper = zip_with(upper, box_upper, maxps);
            lower = zip_with(lower, box_lower, minps);
        }
        index += 1;
    }
    store_vec(e, frame + BOUNDS, lower);
    store_vec(e, frame + BOUNDS + 0x10, upper);
    let info = frame + PHANTOM_INFO;
    e.mem.set_u32(info + 0xc, 0);
    e.mem.set_u32(info + 0x10, 0);
    e.mem.set_u32(info + 0x14, 0x8000_0000);
    e.mem.set_u32(info + 4, 0);
    e.mem.set_u8(info + 8, 2);
    let group = e.mem.u32(this + 0x14);
    e.mem.set_u32(info, (group << 16) | 0x1e);
    store_vec(e, info + 0x20, lower);
    store_vec(e, info + 0x30, upper);
    let block = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
    let phantom = if block != 0 {
        e.call(PHANTOM_CONSTRUCT, &args![block, info]).u32()
    } else {
        0
    };
    e.mem.set_u32(system + 0xc, phantom);
    let first_list = e.mem.u32(e.mem.u32(system + 4) + 8);
    let first_body = e.mem.u32(first_list);
    if first_body != 0 && e.mem.u32(first_body + 8) != 0 {
        let wrapper = e.mem.u32(first_body + 8);
        let world = e.mem.u32(wrapper + 0x350);
        if phantom != 0 {
            e.vcall(phantom, 0x9c, &args![world]);
        }
    }
    let skeleton = e.mem.u32(e.mem.u32(system + 4) + 0x2c);
    let phantom = e.mem.u32(system + 0xc);
    let detector_info = frame + DETECTOR_INFO;
    e.mem.set_u32(detector_info, skeleton);
    e.mem.set_u32(detector_info + 4, this);
    let phantom_word = e.mem.u32(phantom + 8);
    e.mem.set_u32(detector_info + 8, phantom_word);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let detector_block = e.vcall(allocator, 4, &args![0x28u32]).u32();
    e.mem.set_u16(detector_block + 4, 0x28);
    let detector = e
        .call(DETECTOR_CONSTRUCT, &args![detector_block, detector_info])
        .u32();
    e.mem.set_u32(system + 0x10, detector);
    e.call(DETECTOR_RESET, &args![detector]);
    // 00ca30cd: `FLD float [012682b4]`, `FMUL double [01017b70]`, `FSTP float` to a local:
    // both x87 products round to the control word's precision (24-bit on the main thread
    // once the renderer's device exists), as `x87_mul` does.
    let seconds = x87_load32(e.global::<u32>(BUDGET_SECONDS));
    let milliseconds = e.global::<f64>(MILLISECONDS);
    let budget = x87_store32(x87_mul(seconds, milliseconds));
    let now = e.call(GET_TIMER, &args![]).u64();
    e.mem.set_u32(this + 0x20, now as u32);
    e.mem.set_u32(this + 0x24, (now >> 32) as u32);
    // 00ca30e2: `FLD` the local again, `FMUL float [011ac39c]`, then `00ec62f6` on ST0.
    let scale = x87_m32(e.global::<u32>(TICK_SCALE));
    let ticks = e
        .call(FLOAT_TO_INT64, &args![x87_mul(x87_load32(budget), scale)])
        .u64();
    e.mem.set_u32(this + 0x28, ticks as u32);
    e.mem.set_u32(this + 0x2c, (ticks >> 32) as u32);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(info + 0x14);
    let allocator = e.mem.u32(router + 0x10);
    e.mem.set_u32(info + 0x10, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(info + 0xc);
        // 00ca3127: `LEA ECX, [EDX*8]` wraps.
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(8);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Builds a new system for the node of index `index` of the `nodes` list: the extras list
/// is collected, a 0x24-byte record is allocated (all zero, the penetration array empty,
/// `bPenetrating` set), the bodies are reordered (a failure of the reorder sets
/// `failed`), and the skeleton, the ragdoll instance, the phantom and the detector are
/// made. A system that ends up with all of its five pointers is appended to `kept`
/// (`ARRAY_APPEND`), any other is destroyed.
fn build_system(e: &mut Engine, this: u32, frame: u32, index: u32, failed: &mut bool) {
    let bodies = frame + BODIES;
    let parents = frame + PARENTS;
    let kept = frame + KEPT;
    let extras = frame + EXTRAS;
    open_list(e, extras, EXTRAS_VTABLE);
    e.call(COLLECT_EXTRAS, &args![this, bodies, extras]);
    let system = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x24u32])
        .u32();
    if system != 0 {
        for word in 0..7 {
            e.mem.set_u32(system + 4 * word, 0);
        }
        e.mem.set_u32(system + 0x1c, 0x8000_0000);
        e.mem.set_u8(system + 0x20, 1);
    }
    let reordered = e.call(REORDER_FOR_RAGDOLL, &args![bodies, parents]).i32();
    if reordered == 1 {
        *failed = true;
    } else {
        let nodes = e.mem.u32(frame + NODES + 4);
        let root = e.mem.u32(nodes + 4 * index);
        e.mem.set_u32(system, root);
        let skeleton = e
            .call(CREATE_SKELETON, &args![this, bodies, parents, extras])
            .u32();
        e.mem.set_u32(system + 8, skeleton);
        if skeleton != 0 {
            let instance = e
                .call(CREATE_RAGDOLL_INSTANCE, &args![skeleton, bodies, parents])
                .u32();
            e.mem.set_u32(system + 4, instance);
        }
        if e.mem.u32(system + 4) != 0 {
            attach_phantom_and_detector(e, this, frame, system);
        }
    }
    let complete = [0xc, 0, 0x10, 4, 8]
        .into_iter()
        .all(|offset| e.mem.u32(system + offset) != 0);
    if complete {
        let slot = e.call(ARRAY_APPEND, &args![kept]).u32();
        let data = e.mem.u32(kept + 4);
        e.mem.set_u32(data + 4 * slot, system);
    } else {
        discard_system(e, system);
    }
    close_list(e, extras, EXTRAS_VTABLE, true);
}

/// Handles the node of index `index` of the `nodes` list. Its bodies and parents are
/// gathered (`GATHER_BODIES`); with nothing gathered there is nothing to do. Otherwise
/// the first existing system whose root is the node and whose ragdoll has as many rigid
/// bodies as were gathered is appended to `kept`; without one a new system is built.
fn handle_node(e: &mut Engine, this: u32, frame: u32, index: u32, failed: &mut bool) {
    let bodies = frame + BODIES;
    let parents = frame + PARENTS;
    open_frame_array(e, bodies);
    open_frame_array(e, parents);
    let nodes = e.mem.u32(frame + NODES + 4);
    let root = e.mem.u32(nodes + 4 * index);
    e.call(GATHER_BODIES, &args![this, root, bodies, parents]);
    if e.mem.u32(parents + 4) != 0 {
        let mut found = false;
        if e.mem.u32(this + 0xc) != 0 {
            let mut position = 0u32;
            loop {
                if found {
                    break;
                }
                let slot = e.mem.u32(this + 8) + 4 * position;
                let system = e.mem.u32(slot);
                if root == e.mem.u32(system) {
                    let instance = e.mem.u32(system + 4);
                    if e.mem.u32(bodies + 4) == e.mem.u32(instance + 0xc) {
                        let value = e.mem.u32(slot);
                        append_kept(e, frame + KEPT, value);
                        found = true;
                    }
                }
                position += 1;
                if position >= e.mem.u32(this + 0xc) {
                    break;
                }
            }
        }
        if !found {
            build_system(e, this, frame, index, failed);
        }
    }
    close_frame_array(e, parents);
    close_frame_array(e, bodies);
}

/// Clears bit 14 of the collision filter word of the rigid body number `bone` of the
/// ragdoll instance of `system` (the bodies a penetration check had marked) and tells the
/// body's shape about it through virtual slot `0xC4`.
fn clear_penetration_mark(e: &mut Engine, system: u32, bone: u32) {
    let instance = e.mem.u32(system + 4);
    let map = e.mem.u32(instance + 0x20);
    let rigid_index = e.mem.i32(map + 4 * bone);
    if rigid_index < 0 {
        return;
    }
    let rigid_bodies = e.mem.u32(instance + 8);
    let body = e.mem.u32(rigid_bodies + 4 * rigid_index as u32);
    if body == 0 {
        return;
    }
    let shape = e.mem.u32(body + 0xc);
    if shape == 0 {
        return;
    }
    let owner = e.mem.u32(shape + 8);
    let filter = if owner != 0 && owner.wrapping_add(0x10) != 0 {
        e.mem.u32(owner.wrapping_add(0x10).wrapping_add(0x1c))
    } else {
        0
    };
    if (filter >> 14) & 1 == 0 {
        return;
    }
    let cleared = filter & 0xffff_bfff;
    if owner != 0 {
        e.call(NO_OP, &args![shape]);
        e.call(NO_OP, &args![shape]);
        let target = owner.wrapping_add(0x10);
        if target != 0 {
            e.mem.set_u32(target.wrapping_add(0x1c), cleared);
        }
    }
    e.vcall(shape, 0xc4, &args![]);
}

// Translated from 00ca2ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca2ad0 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not modeled.
/// `bhkRagdollPenetrationUtil` method with no Xbox PDB name (`this` in ECX, stack
/// arguments `node` and `group`; answers whether the util now has systems, in `AL`).
/// Stores `group` as the util's collision group (`+0x14`), collects the nodes under `node`
/// (`00ca0f00`) and for each handles its bodies (see `handle_node`), keeping the existing
/// systems that still match and building the missing ones. Systems that were not kept are
/// destroyed, the kept list replaces `ActiveSystems` (`00ca0e60`), and the retry state is
/// updated: the byte at `+0x18` (an initialization attempt is due) is set when there are
/// no systems and no reorder failed, in which case the attempt counter (`+0x1C`) is
/// raised and, past 5, both are cleared; with systems or a failure the counter is
/// cleared. Finally the penetration mark (bit 14 of the collision filter word) of every
/// rigid body of every system is cleared.
pub fn fn_00ca2ad0(e: &mut Engine, this: u32, node: u32, group: u32) -> bool {
    e.with_stack(SCRATCH_SIZE, |e, scratch| {
        rebuild_systems(e, this, node, group, scratch.addr())
    })
}

fn rebuild_systems(e: &mut Engine, this: u32, node: u32, group: u32, frame: u32) -> bool {
    let nodes = frame + NODES;
    let kept = frame + KEPT;
    e.mem.set_u32(this + 0x14, group);
    open_list(e, nodes, NODES_VTABLE);
    e.call(COLLECT_NODES, &args![this, node, nodes]);
    open_list(e, kept, KEPT_VTABLE);
    let mut failed = false;
    let mut index = 0u32;
    while index < e.mem.u32(nodes + 8) {
        handle_node(e, this, frame, index, &mut failed);
        index += 1;
    }
    // Systems that were not kept are destroyed.
    if e.mem.u32(this + 0xc) != 0 {
        let mut position = 0u32;
        loop {
            let slot = e.mem.u32(this + 8) + 4 * position;
            let kept_size = e.mem.u32(kept + 8);
            let mut still_wanted = false;
            let mut candidate = 0u32;
            if kept_size != 0 {
                loop {
                    let data = e.mem.u32(kept + 4);
                    if data != 0 && e.mem.u32(data + 4 * candidate) == e.mem.u32(slot) {
                        still_wanted = candidate != u32::MAX;
                        break;
                    }
                    candidate = candidate.wrapping_add(1);
                    if candidate >= kept_size {
                        break;
                    }
                }
            }
            if !still_wanted {
                let system = e.mem.u32(slot);
                if system != 0 {
                    discard_system(e, system);
                }
            }
            position += 1;
            if position >= e.mem.u32(this + 0xc) {
                break;
            }
        }
    }
    e.call(COPY_SYSTEMS, &args![kept, this + 4]);
    let count = e.mem.u32(this + 0xc);
    let has_systems = count > 0;
    let due = !has_systems && !failed;
    e.mem.set_u8(this + 0x18, u8::from(due));
    let mut reset = true;
    if due {
        let attempts = e.mem.u32(this + 0x1c).wrapping_add(1);
        e.mem.set_u32(this + 0x1c, attempts);
        if attempts <= 5 {
            reset = false;
        } else {
            e.mem.set_u8(this + 0x18, 0);
        }
    }
    if reset {
        e.mem.set_u32(this + 0x1c, 0);
    }
    if count != 0 {
        let mut system_index = 0u32;
        loop {
            let system = e.mem.u32(e.mem.u32(this + 8) + 4 * system_index);
            let skeleton = e.mem.u32(e.mem.u32(system + 4) + 0x2c);
            if e.mem.i32(skeleton + 0x1c) > 0 {
                let mut bone = 0u32;
                loop {
                    let system = e.mem.u32(e.mem.u32(this + 8) + 4 * system_index);
                    clear_penetration_mark(e, system, bone);
                    bone += 1;
                    let system = e.mem.u32(e.mem.u32(this + 8) + 4 * system_index);
                    let skeleton = e.mem.u32(e.mem.u32(system + 4) + 0x2c);
                    if (bone as i32) >= e.mem.i32(skeleton + 0x1c) {
                        break;
                    }
                }
            }
            system_index += 1;
            if system_index >= e.mem.u32(this + 0xc) {
                break;
            }
        }
    }
    close_list(e, kept, KEPT_VTABLE, true);
    close_list(e, nodes, NODES_VTABLE, false);
    has_systems
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca2ad0, fn_00ca2ad0(u32, u32, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    /// Cells the doubles read and write.
    const RECORD: u32 = 0x0900_1000;
    const REORDER_RESULT: u32 = RECORD;
    const SKELETON_RESULT: u32 = RECORD + 4;
    const INSTANCE_RESULT: u32 = RECORD + 8;
    const ROOT_COUNT: u32 = RECORD + 0x10;
    const PHANTOM_RECORD: u32 = RECORD + 0x40;
    const DETECTOR_RECORD: u32 = RECORD + 0x80;
    /// The memory router, its allocator and the value its cursor starts with.
    const ROUTER_OBJECT: u32 = 0x0900_3000;
    const ALLOCATOR_OBJECT: u32 = 0x0900_3100;
    const ALLOCATOR_VTABLE: u32 = 0x0900_3200;
    const ROUTER_CURSOR: u32 = 0x0800_0000;
    /// The vtable the phantom double gets; its slot 0x9c is `PHANTOM_SLOT`.
    const PHANTOM_VTABLE: u32 = 0x0900_5000;
    /// The doubles in the vtable slots.
    const ALLOCATE_SLOT: u32 = 0x0900_0004;
    const FREE_SLOT: u32 = 0x0900_0010;
    const SHAPE_SLOT: u32 = 0x0900_003c;
    const PHANTOM_SLOT: u32 = 0x0900_009c;
    const MARK_SLOT: u32 = 0x0900_00c4;
    const NODES_FREE: u32 = 0x0900_0a08;
    const KEPT_ALLOCATE: u32 = 0x0900_0b04;
    const KEPT_FREE: u32 = 0x0900_0b08;
    const EXTRAS_FREE: u32 = 0x0900_0c08;
    /// The world the first body wrapper holds.
    const WORLD: u32 = 0x7777;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn word(e: &Engine, address: u32) -> Ret {
        ret(e.mem.u32(address))
    }

    fn engine() -> Engine {
        let mut e = logging_engine();
        for (address, len) in [
            (RECORD, 0x100),
            (ROUTER_OBJECT, 0x40),
            (ALLOCATOR_OBJECT, 0x20),
            (ALLOCATOR_VTABLE, 0x40),
            (PHANTOM_VTABLE, 0x100),
            (BUDGET_SECONDS, 4),
            (MILLISECONDS, 8),
            (TICK_SCALE, 4),
            (LOWER_START, 4),
            (UPPER_START, 4),
        ] {
            e.map(address, len);
        }
        e.mem.set_u32(ROUTER_OBJECT, ROUTER_CURSOR);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_VTABLE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 4, ALLOCATE_SLOT);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0x10, FREE_SLOT);
        e.mem.set_u32(PHANTOM_VTABLE + 0x9c, PHANTOM_SLOT);
        e.mem.set_f32(BUDGET_SECONDS, 0.5);
        e.mem.set_f64(MILLISECONDS, 1000.0);
        e.mem.set_f32(TICK_SCALE, 2.0);
        e.mem.set_f32(LOWER_START, 1.0e30);
        e.mem.set_f32(UPPER_START, -1.0e30);
        e.put_vtable(NODES_VTABLE, &[0, 0, NODES_FREE]);
        e.put_vtable(KEPT_VTABLE, &[0, KEPT_ALLOCATE, KEPT_FREE]);
        e.put_vtable(EXTRAS_VTABLE, &[0, 0, EXTRAS_FREE]);
        e.register(MEMORY_ROUTER, |_, _| ret(ROUTER_OBJECT));
        e.register(ALLOCATE_SLOT, |e, a| {
            let block = e.mem.alloc(a[1]);
            ret(block)
        });
        e.register(KEPT_ALLOCATE, |e, a| {
            let block = e.mem.alloc(4 * a[1]);
            ret(block)
        });
        for slot in [
            FREE_SLOT,
            PHANTOM_SLOT,
            MARK_SLOT,
            NODES_FREE,
            KEPT_FREE,
            EXTRAS_FREE,
        ] {
            e.register(slot, nothing);
        }
        e.register(SHAPE_SLOT, |e, a| {
            for lane in 0..8 {
                let value = e.mem.u32(a[1] + 4 * lane);
                e.mem.set_u32(a[2] + 4 * lane, value);
            }
            Ret::default()
        });
        e.register(COLLECT_NODES, |e, a| {
            let count = e.mem.u32(ROOT_COUNT);
            if count == 0 {
                return Ret::default();
            }
            let data = e.mem.alloc(4 * count.max(1));
            for index in 0..count {
                let root = e.mem.u32(ROOT_COUNT + 4 + 4 * index);
                e.mem.set_u32(data + 4 * index, root);
            }
            e.mem.set_u32(a[2] + 4, data);
            e.mem.set_u32(a[2] + 8, count);
            e.mem.set_u32(a[2] + 0xc, count);
            Ret::default()
        });
        e.register(GATHER_BODIES, |e, a| {
            // The test keeps the gathered bodies of a node at +0x30 (data) and +0x34
            // (count) and the number of gathered parents at +0x38.
            let data = e.mem.u32(a[1] + 0x30);
            let count = e.mem.u32(a[1] + 0x34);
            let parents = e.mem.u32(a[1] + 0x38);
            e.mem.set_u32(a[2], data);
            e.mem.set_u32(a[2] + 4, count);
            e.mem.set_u32(a[3] + 4, parents);
            Ret::default()
        });
        e.register(COLLECT_EXTRAS, nothing);
        e.register(REORDER_FOR_RAGDOLL, |e, _| word(e, REORDER_RESULT));
        e.register(CREATE_SKELETON, |e, _| word(e, SKELETON_RESULT));
        e.register(CREATE_RAGDOLL_INSTANCE, |e, _| word(e, INSTANCE_RESULT));
        e.register(PHANTOM_CONSTRUCT, |e, a| {
            let filter = e.mem.u32(a[1]);
            e.mem.set_u32(PHANTOM_RECORD, filter);
            for lane in 0..8 {
                let value = e.mem.u32(a[1] + 0x20 + 4 * lane);
                e.mem.set_u32(PHANTOM_RECORD + 4 + 4 * lane, value);
            }
            e.mem.set_u32(a[0], PHANTOM_VTABLE);
            ret(a[0])
        });
        e.register(DETECTOR_CONSTRUCT, |e, a| {
            for index in 0..3 {
                let value = e.mem.u32(a[1] + 4 * index);
                e.mem.set_u32(DETECTOR_RECORD + 4 * index, value);
            }
            ret(a[0])
        });
        e.register(DETECTOR_RESET, nothing);
        e.register(ARRAY_APPEND, |e, a| {
            if e.mem.u32(a[0] + 4) == 0 {
                let data = e.mem.alloc(64);
                e.mem.set_u32(a[0] + 4, data);
                e.mem.set_u32(a[0] + 0xc, 16);
            }
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.register(COPY_SYSTEMS, |e, a| {
            let data = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[1] + 4, data);
            e.mem.set_u32(a[1] + 8, size);
            Ret::default()
        });
        e.register(DESTROY_SYSTEM, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        e.register(NO_OP, nothing);
        e.register(GET_TIMER, |_, _| Ret {
            eax: 100,
            edx: 1,
            ..Ret::default()
        });
        e.register(FLOAT_TO_INT64, |_, _| Ret {
            eax: 0x55,
            edx: 0x66,
            ..Ret::default()
        });
        e.register(HEAP_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            ret(block)
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            ret(block)
        });
        e
    }

    /// A ragdoll instance with `rigid_count` rigid bodies, one bone and a first body
    /// whose wrapper holds `WORLD` and a shape owner answering the bounds through virtual
    /// slot 0x3c; the body shape and its filter owner (word `0x4100` at `+0x2c`, bit 14
    /// set) are answered too. Answers the instance, the shape and the filter owner.
    fn ragdoll_instance(e: &mut Engine, rigid_count: u32) -> (u32, u32, u32) {
        let instance = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let map = e.mem.alloc(0x10);
        let rigid_list = e.mem.alloc(0x10);
        let body = e.mem.alloc(0x20);
        let wrapper = e.mem.alloc(0x400);
        let shape_owner = e.mem.alloc(0x20);
        let shape_vtable = e.mem.alloc(0x100);
        let shape = e.mem.alloc(0x40);
        let filter_owner = e.mem.alloc(0x40);
        let owner_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(owner_vtable + 0x3c, SHAPE_SLOT);
        e.mem.set_u32(shape_owner, owner_vtable);
        e.mem.set_u32(shape_vtable + 0xc4, MARK_SLOT);
        e.mem.set_u32(shape, shape_vtable);
        e.mem.set_u32(shape + 8, filter_owner);
        e.mem.set_u32(filter_owner + 0x2c, 0x4100);
        e.mem.set_u32(wrapper + 0x58, shape_owner);
        e.mem.set_u32(wrapper + 0x350, WORLD);
        e.mem.set_u32(body + 8, wrapper);
        e.mem.set_u32(body + 0xc, shape);
        e.mem.set_u32(rigid_list, body);
        e.mem.set_u32(skeleton + 0x1c, 1);
        e.mem.set_u32(instance + 8, rigid_list);
        e.mem.set_u32(instance + 0xc, rigid_count);
        e.mem.set_u32(instance + 0x20, map);
        e.mem.set_u32(instance + 0x2c, skeleton);
        (instance, shape, filter_owner)
    }

    fn system(e: &mut Engine, root: u32, instance: u32) -> u32 {
        let system = e.mem.alloc(0x24);
        e.mem.set_u32(system, root);
        e.mem.set_u32(system + 4, instance);
        system
    }

    fn util(e: &mut Engine, systems: &[u32]) -> u32 {
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(4 * systems.len().max(1) as u32);
        for (index, system) in systems.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *system);
        }
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, systems.len() as u32);
        this
    }

    /// A node the gather double answers with `bodies` and `parents` parents.
    fn node(e: &mut Engine, bodies: &[u32], parents: u32) -> u32 {
        let node = e.mem.alloc(0x40);
        let data = e.mem.alloc(4 * bodies.len().max(1) as u32);
        for (index, body) in bodies.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *body);
        }
        e.mem.set_u32(node + 0x30, data);
        e.mem.set_u32(node + 0x34, bodies.len() as u32);
        e.mem.set_u32(node + 0x38, parents);
        node
    }

    /// Makes `nodes` the nodes `COLLECT_NODES` answers.
    fn roots(e: &mut Engine, nodes: &[u32]) {
        e.mem.set_u32(ROOT_COUNT, nodes.len() as u32);
        for (index, root) in nodes.iter().enumerate() {
            e.mem.set_u32(ROOT_COUNT + 4 + 4 * index as u32, *root);
        }
    }

    /// A body whose shape owner answers `lower` and `upper` as its bounds.
    fn bounded_body(e: &mut Engine, lower: Vec4, upper: Vec4) -> u32 {
        let body = e.mem.alloc(0x50);
        e.mem.set_u32(body + 0x10, 1);
        store_vec(e, body + 0x24, lower);
        store_vec(e, body + 0x34, upper);
        body
    }

    #[test]
    fn without_nodes_the_attempt_flag_is_raised_and_given_up_after_five() {
        let mut e = engine();
        roots(&mut e, &[]);
        let this = util(&mut e, &[]);
        assert!(!e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]).bool());
        assert_eq!(e.mem.u32(this + 0x14), 3);
        let collect = calls_to(&e, COLLECT_NODES);
        assert_eq!(collect.len(), 1);
        assert_eq!(collect[0][..2], [this, 0x77]);
        let copy = calls_to(&e, COPY_SYSTEMS);
        assert_eq!(copy[0][1], this + 4);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        for attempt in 2..=5 {
            e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]);
            assert_eq!(e.mem.u8(this + 0x18), 1);
            assert_eq!(e.mem.u32(this + 0x1c), attempt);
        }
        e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert!(calls_to(&e, GATHER_BODIES).is_empty());
        assert!(calls_to(&e, KEPT_FREE).is_empty());
        assert!(calls_to(&e, NODES_FREE).is_empty());
    }

    #[test]
    fn a_node_without_parents_gathered_makes_no_system() {
        let mut e = engine();
        let gathered = node(&mut e, &[1, 2], 0);
        roots(&mut e, &[gathered]);
        let this = util(&mut e, &[]);
        assert!(!e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]).bool());
        let gather = calls_to(&e, GATHER_BODIES);
        assert_eq!(gather.len(), 1);
        assert_eq!(gather[0][..2], [this, gathered]);
        assert_eq!(gather[0][3] - gather[0][2], 0x10);
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(ROUTER_OBJECT), ROUTER_CURSOR);
        assert_eq!(calls_to(&e, NODES_FREE).len(), 1);
        assert_eq!(e.mem.u8(this + 0x18), 1);
    }

    #[test]
    fn a_matching_system_is_kept_and_the_others_are_destroyed() {
        let mut e = engine();
        let (instance, shape, owner) = ragdoll_instance(&mut e, 3);
        let (other_instance, _, _) = ragdoll_instance(&mut e, 3);
        let gathered = node(&mut e, &[0, 0, 0], 2);
        let kept = system(&mut e, gathered, instance);
        let stale = system(&mut e, 0x99, other_instance);
        roots(&mut e, &[gathered]);
        let this = util(&mut e, &[stale, kept]);
        assert!(e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]).bool());
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, DESTROY_SYSTEM), [[stale]]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE), [[MEMORY_MANAGER, stale]]);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this + 8)), kept);
        assert_eq!(calls_to(&e, KEPT_ALLOCATE).len(), 1);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        // The penetration mark of the kept ragdoll body is cleared.
        assert_eq!(e.mem.u32(owner + 0x2c), 0x100);
        assert_eq!(calls_to(&e, MARK_SLOT), [[shape]]);
        assert_eq!(calls_to(&e, NO_OP), [[shape], [shape]]);
        assert_eq!(calls_to(&e, KEPT_FREE).len(), 1);
        assert_eq!(e.mem.u32(ROUTER_OBJECT), ROUTER_CURSOR);
    }

    #[test]
    fn a_ragdoll_with_another_body_count_gets_a_new_system() {
        let mut e = engine();
        let (old_instance, _, _) = ragdoll_instance(&mut e, 5);
        let (new_instance, shape, owner) = ragdoll_instance(&mut e, 2);
        let first = bounded_body(&mut e, [1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]);
        let second = bounded_body(&mut e, [0.0, 3.0, 2.0, 5.0], [4.0, 9.0, 6.0, 7.0]);
        let gathered = node(&mut e, &[first, second], 2);
        let old = system(&mut e, gathered, old_instance);
        roots(&mut e, &[gathered]);
        e.mem.set_u32(SKELETON_RESULT, 0x5150);
        e.mem.set_u32(INSTANCE_RESULT, new_instance);
        let this = util(&mut e, &[old]);
        assert!(e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]).bool());
        let gather = calls_to(&e, GATHER_BODIES);
        let (bodies, parents) = (gather[0][2], gather[0][3]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            [[MEMORY_MANAGER, 0x24]]
        );
        assert_eq!(calls_to(&e, REORDER_FOR_RAGDOLL), [[bodies, parents]]);
        let skeleton = calls_to(&e, CREATE_SKELETON);
        assert_eq!(skeleton[0][..3], [this, bodies, parents]);
        assert_eq!(
            calls_to(&e, CREATE_RAGDOLL_INSTANCE),
            [[0x5150, bodies, parents]]
        );
        // The phantom: filter info with the group, bounds over both bodies.
        assert_eq!(e.mem.u32(PHANTOM_RECORD), (3 << 16) | 0x1e);
        assert_eq!(load_vec(&e, PHANTOM_RECORD + 4), [0.0, 2.0, 2.0, 4.0]);
        assert_eq!(load_vec(&e, PHANTOM_RECORD + 0x14), [5.0, 9.0, 7.0, 8.0]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), [[0x18]]);
        let phantom = calls_to(&e, PHANTOM_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, PHANTOM_SLOT), [[phantom, WORLD]]);
        // The detector: a 0x28-byte block built from the skeleton, the util and the
        // word at +8 of the phantom, then reset.
        assert_eq!(calls_to(&e, ALLOCATE_SLOT), [[ALLOCATOR_OBJECT, 0x28]]);
        let detector = calls_to(&e, DETECTOR_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u16(detector + 4), 0x28);
        assert_eq!(calls_to(&e, DETECTOR_RESET), [[detector]]);
        let new_skeleton = e.mem.u32(new_instance + 0x2c);
        assert_eq!(e.mem.u32(DETECTOR_RECORD), new_skeleton);
        assert_eq!(e.mem.u32(DETECTOR_RECORD + 4), this);
        assert_eq!(e.mem.u32(DETECTOR_RECORD + 8), 0);
        // The timer: now, and the limit from 0.5 * 1000.0 scaled by 2.0.
        assert_eq!(e.mem.u32(this + 0x20), 100);
        assert_eq!(e.mem.u32(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x28), 0x55);
        assert_eq!(e.mem.u32(this + 0x2c), 0x66);
        let scaled = calls_to(&e, FLOAT_TO_INT64);
        let bits = u64::from(scaled[0][0]) | (u64::from(scaled[0][1]) << 32);
        assert_eq!(f64::from_bits(bits), 1000.0);
        // The new system replaces the old one.
        assert_eq!(e.mem.u32(this + 0xc), 1);
        let built = e.mem.u32(e.mem.u32(this + 8));
        assert_eq!(e.mem.u32(built), gathered);
        assert_eq!(e.mem.u32(built + 4), new_instance);
        assert_eq!(e.mem.u32(built + 8), 0x5150);
        assert_eq!(e.mem.u32(built + 0xc), phantom);
        assert_eq!(e.mem.u32(built + 0x10), detector);
        assert_eq!(e.mem.u8(built + 0x20), 1);
        assert_eq!(e.mem.u32(built + 0x1c), 0x8000_0000);
        assert_eq!(calls_to(&e, DESTROY_SYSTEM), [[old]]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE), [[MEMORY_MANAGER, old]]);
        assert_eq!(e.mem.u32(owner + 0x2c), 0x100);
        assert_eq!(calls_to(&e, MARK_SLOT), [[shape]]);
        assert_eq!(e.mem.u32(ROUTER_OBJECT), ROUTER_CURSOR);
    }

    /// 00ca2ad0 (00ca30cd, 00ca3127): under 24-bit precision the tick product of the float
    /// budget and the float scale reaches `00ec62f6` rounded to 24 bits, and the phantom
    /// info's own buffer is freed with `8 * capacity` wrapped to 32 bits.
    #[test]
    fn the_tick_product_follows_the_control_word_and_the_free_size_wraps() {
        let mut e = engine();
        crate::x87::set_control_word(0x007f);
        e.mem.set_u32(TICK_SCALE, 0x3f80_0001);
        e.register(PHANTOM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[1] + 0xc, 0x0900_1234);
            e.mem.set_u32(a[1] + 0x14, 0x2000_0001);
            e.mem.set_u32(a[0], PHANTOM_VTABLE);
            ret(a[0])
        });
        let (instance, _, _) = ragdoll_instance(&mut e, 1);
        let body = bounded_body(&mut e, [1.0; 4], [2.0; 4]);
        let gathered = node(&mut e, &[body], 1);
        roots(&mut e, &[gathered]);
        e.mem.set_u32(SKELETON_RESULT, 0x5150);
        e.mem.set_u32(INSTANCE_RESULT, instance);
        let this = util(&mut e, &[]);
        e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]);
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        // 500.0 * (1 + 2^-23) is 500 plus 1.95 units of the 24th bit: 500 + 2^-14.
        let scaled = calls_to(&e, FLOAT_TO_INT64);
        let bits = u64::from(scaled[0][0]) | (u64::from(scaled[0][1]) << 32);
        assert_eq!(f64::from_bits(bits), 500.0 + 2f64.powi(-14));
        assert_eq!(
            calls_to(&e, FREE_SLOT),
            [[ALLOCATOR_OBJECT, 0x0900_1234, 8]]
        );
    }

    #[test]
    fn a_failed_reorder_discards_the_new_system_and_ends_the_retries() {
        let mut e = engine();
        let gathered = node(&mut e, &[1], 1);
        roots(&mut e, &[gathered]);
        e.mem.set_u32(REORDER_RESULT, 1);
        let this = util(&mut e, &[]);
        e.mem.set_u32(this + 0x1c, 3);
        assert!(!e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]).bool());
        assert!(calls_to(&e, CREATE_SKELETON).is_empty());
        let discarded = calls_to(&e, DESTROY_SYSTEM);
        assert_eq!(discarded.len(), 1);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            [[MEMORY_MANAGER, discarded[0][0]]]
        );
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn a_system_without_a_skeleton_is_discarded_but_the_retries_go_on() {
        let mut e = engine();
        let gathered = node(&mut e, &[1], 1);
        roots(&mut e, &[gathered]);
        let this = util(&mut e, &[]);
        e.mem.set_u32(this + 0x1c, 3);
        assert!(!e.call(0x00ca_2ad0, &args![this, 0x77u32, 3u32]).bool());
        assert_eq!(calls_to(&e, CREATE_SKELETON).len(), 1);
        assert!(calls_to(&e, CREATE_RAGDOLL_INSTANCE).is_empty());
        assert!(calls_to(&e, ARRAY_APPEND).is_empty());
        assert_eq!(calls_to(&e, DESTROY_SYSTEM).len(), 1);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 4);
    }
}
