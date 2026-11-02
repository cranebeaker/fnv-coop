//! `Havok SDK/hkpworldcallbackutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `fire...` functions walk one of the world's listener arrays (`hkArray`: data pointer,
//! `i32` size) from the last entry to the first, call a virtual method of every non-null
//! listener inside a pair of profiling records, and finally remove the null entries the
//! listeners left behind (`00d00370`, which takes the array in ESI: translated as a plain
//! Rust function and not registered, like the other register-argument pieces of the
//! `hkpworld.cpp` unit). The profiling records are those of `hkpworld.rs`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpworld::{profile_record, HkpWorld, PROFILE_END};

/// `hkpWorld::m_actionListeners`, `m_entityListeners`, `m_phantomListeners`,
/// `m_constraintListeners`, `m_islandActivationListeners` and `m_worldPostSimulationListeners`
/// (Xbox PDB): the offsets of the `hkArray`s inside the world.
const ACTION_LISTENERS: u32 = 0x11c;
const ENTITY_LISTENERS: u32 = 0x128;
const PHANTOM_LISTENERS: u32 = 0x134;
const CONSTRAINT_LISTENERS: u32 = 0x140;
const ISLAND_ACTIVATION_LISTENERS: u32 = 0x158;
const WORLD_POST_SIMULATION_LISTENERS: u32 = 0x164;

/// The labels of the profiling records the callbacks open (`"TtactAddCb"`, `"TtactRemCb"`,
/// ...); the matching end label is [`PROFILE_END`].
const LABEL_ACTION_ADDED: u32 = 0x010c_ca68;
const LABEL_ACTION_REMOVED: u32 = 0x010c_ca74;
const LABEL_ENTITY_ADDED: u32 = 0x010c_c9dc;
const LABEL_ENTITY_REMOVED: u32 = 0x010c_c9e8;
const LABEL_ENTITY_SET_MOTION_TYPE: u32 = 0x010c_ca04;
const LABEL_PHANTOM_ADDED: u32 = 0x010c_ca80;
const LABEL_PHANTOM_REMOVED: u32 = 0x010c_ca8c;
const LABEL_CONSTRAINT_ADDED: u32 = 0x010c_ca98;
const LABEL_CONSTRAINT_REMOVED: u32 = 0x010c_caa4;
const LABEL_INACTIVE_ENTITY_MOVED: u32 = 0x010c_cb54;
const LABEL_ISLAND_ACTIVATED: u32 = 0x010c_caf8;
const LABEL_ENTITY_ISLAND_ACTIVATED: u32 = 0x010c_caec;

/// `00c91120(world)` and `00c91140(world)`: the world's flushes of its pending-operation
/// queues, run when the critical-operations lock count drops back to 0.
const FLUSH_PENDING_OPERATIONS: u32 = 0x00c9_1120;
const FLUSH_PENDING_BODY_OPERATIONS: u32 = 0x00c9_1140;

// Translated from 00d00320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d00320 - takes EAX (the array) as a register input, which has no uniform-ABI form; translated as a Rust function with it as the parameter and not registered.
/// `worldCallbackUtil_cleanupNullPointers` (Xbox PDB): removes the null entries of the small
/// array at `array` (data pointer, then a `u16` size at +4), keeping the order of the others,
/// going from the last entry to the first.
pub fn world_callback_util_cleanup_null_pointers(e: &mut Engine, array: Ptr) {
    let mut index = i32::from(e.mem.u16(array.addr() + 4)) - 1;
    while index >= 0 {
        let data = e.mem.u32(array.addr());
        if e.mem.u32(data.wrapping_add(index as u32 * 4)) == 0 {
            let size = e.mem.u16(array.addr() + 4).wrapping_sub(1);
            e.mem.set_u16(array.addr() + 4, size);
            let mut from = index;
            if from < i32::from(size) {
                loop {
                    let data = e.mem.u32(array.addr());
                    let next = e.mem.u32(data.wrapping_add(from as u32 * 4 + 4));
                    e.mem.set_u32(data.wrapping_add(from as u32 * 4), next);
                    from += 1;
                    if from >= i32::from(e.mem.u16(array.addr() + 4)) {
                        break;
                    }
                }
            }
        }
        index -= 1;
    }
}

// Translated from 00d00370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d00370 - takes ESI (the array) as a register input, which has no uniform-ABI form; translated as a Rust function with it as the parameter and not registered.
/// `worldCallbackUtil_cleanupNullPointers_ov2` (Xbox PDB): removes the null entries of the
/// `hkArray` at `array` (data pointer, `i32` size at +4), keeping the order of the others,
/// going from the last entry to the first.
pub fn world_callback_util_cleanup_null_pointers_ov2(e: &mut Engine, array: Ptr) {
    let mut index = e.mem.i32(array.addr() + 4).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(array.addr());
        let entry = data.wrapping_add(index as u32 * 4);
        if e.mem.u32(entry) == 0 {
            let size = e.mem.i32(array.addr() + 4).wrapping_sub(1);
            e.mem.set_i32(array.addr() + 4, size);
            let bytes = size.wrapping_sub(index).wrapping_mul(4);
            if bytes > 0 {
                let count = ((bytes as u32 - 1) >> 2) + 1;
                for n in 0..count {
                    let next = e.mem.u32(entry.wrapping_add(4 * n + 4));
                    e.mem.set_u32(entry.wrapping_add(4 * n), next);
                }
            }
        }
        index -= 1;
    }
}

/// The loop every `fire...` function shares: for the listeners of the world's array at
/// `array_offset`, last to first, each non-null one gets its virtual method at `slot` called
/// with `argument` between a profiling record named `label` and the end record; then the null
/// entries are removed.
fn fire_listeners(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    array_offset: u32,
    label: u32,
    slot: u32,
    argument: u32,
) {
    let array = world.addr().wrapping_add(array_offset);
    let mut index = e.mem.i32(array + 4);
    loop {
        index = index.wrapping_sub(1);
        if index < 0 {
            break;
        }
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add(index as u32 * 4)) != 0 {
            profile_record(e, label);
            let data = e.mem.u32(array);
            let listener = e.mem.u32(data.wrapping_add(index as u32 * 4));
            e.vcall(listener, slot, &args![argument]);
            profile_record(e, PROFILE_END);
        }
    }
    world_callback_util_cleanup_null_pointers_ov2(e, Ptr::new(array));
}

// Translated from 00d003c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireActionAdded` (Xbox PDB): slot 4 of every action listener.
pub fn hkp_world_callback_util_fire_action_added(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    action: u32,
) {
    fire_listeners(e, world, ACTION_LISTENERS, LABEL_ACTION_ADDED, 4, action);
}

// Translated from 00d00460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireActionRemoved` (Xbox PDB): slot 8 of every action listener.
pub fn hkp_world_callback_util_fire_action_removed(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    action: u32,
) {
    fire_listeners(e, world, ACTION_LISTENERS, LABEL_ACTION_REMOVED, 8, action);
}

// Translated from 00d00500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireEntityAdded` (Xbox PDB): slot 4 of every entity listener.
pub fn hkp_world_callback_util_fire_entity_added(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    entity: u32,
) {
    fire_listeners(e, world, ENTITY_LISTENERS, LABEL_ENTITY_ADDED, 4, entity);
}

// Translated from 00d005a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireEntityRemoved` (Xbox PDB): slot 8 of every entity listener.
pub fn hkp_world_callback_util_fire_entity_removed(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    entity: u32,
) {
    fire_listeners(e, world, ENTITY_LISTENERS, LABEL_ENTITY_REMOVED, 8, entity);
}

// Translated from 00d006e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireEntitySetMotionType` (Xbox PDB): slot 0x10 of every entity
/// listener.
pub fn hkp_world_callback_util_fire_entity_set_motion_type(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    entity: u32,
) {
    fire_listeners(
        e,
        world,
        ENTITY_LISTENERS,
        LABEL_ENTITY_SET_MOTION_TYPE,
        0x10,
        entity,
    );
}

// Translated from 00d00780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::firePhantomAdded` (Xbox PDB): slot 4 of every phantom listener.
pub fn hkp_world_callback_util_fire_phantom_added(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    phantom: u32,
) {
    fire_listeners(e, world, PHANTOM_LISTENERS, LABEL_PHANTOM_ADDED, 4, phantom);
}

// Translated from 00d00820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::firePhantomRemoved` (Xbox PDB): slot 8 of every phantom listener.
pub fn hkp_world_callback_util_fire_phantom_removed(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    phantom: u32,
) {
    fire_listeners(
        e,
        world,
        PHANTOM_LISTENERS,
        LABEL_PHANTOM_REMOVED,
        8,
        phantom,
    );
}

// Translated from 00d00960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireConstraintAdded` (Xbox PDB): slot 4 of every constraint listener.
pub fn hkp_world_callback_util_fire_constraint_added(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    constraint: u32,
) {
    fire_listeners(
        e,
        world,
        CONSTRAINT_LISTENERS,
        LABEL_CONSTRAINT_ADDED,
        4,
        constraint,
    );
}

// Translated from 00d00a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireConstraintRemoved` (Xbox PDB): slot 8 of every constraint
/// listener.
pub fn hkp_world_callback_util_fire_constraint_removed(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    constraint: u32,
) {
    fire_listeners(
        e,
        world,
        CONSTRAINT_LISTENERS,
        LABEL_CONSTRAINT_REMOVED,
        8,
        constraint,
    );
}

// Translated from 00d017b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireInactiveEntityMoved` (Xbox PDB): slot 8 of every listener of the
/// array at +0x164 (`m_worldPostSimulationListeners` in the Xbox PDB).
pub fn hkp_world_callback_util_fire_inactive_entity_moved(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    entity: u32,
) {
    fire_listeners(
        e,
        world,
        WORLD_POST_SIMULATION_LISTENERS,
        LABEL_INACTIVE_ENTITY_MOVED,
        8,
        entity,
    );
}

// Translated from 00d00fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireIslandActivated` (Xbox PDB). With the critical-operations lock
/// count raised for the duration: slot 4 of every island-activation listener gets `island`;
/// then for every entity of the island (array at +0x48, count at +0x4c) that has a listener
/// list at +0x214 (a small array), slot 8 of each of its listeners gets the entity. When the
/// lock count falls back to 0 and operations are not blocked (`m_blockExecutingPendingOperations`),
/// the pending-operation queue is flushed if `m_pendingOperationsCount` is non-zero, and the
/// body-operation queue if `m_pendingOperationQueueCount` is 1 and `m_pendingBodyOperationsCount`
/// is non-zero.
pub fn hkp_world_callback_util_fire_island_activated(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    island: Ptr,
) {
    let lock_count = e.get(world, HkpWorld::m_criticalOperationsLockCount);
    e.set(
        world,
        HkpWorld::m_criticalOperationsLockCount,
        lock_count.wrapping_add(1),
    );
    fire_listeners(
        e,
        world,
        ISLAND_ACTIVATION_LISTENERS,
        LABEL_ISLAND_ACTIVATED,
        4,
        island.addr(),
    );

    let mut entity_index = 0i32;
    while entity_index < e.mem.i32(island.addr() + 0x4c) {
        let entities = e.mem.u32(island.addr() + 0x48);
        let entity = e.mem.u32(entities.wrapping_add(entity_index as u32 * 4));
        let listeners = e.mem.u32(entity.wrapping_add(0x214));
        if listeners != 0 {
            let mut index = i32::from(e.mem.u16(listeners + 4));
            loop {
                index -= 1;
                if index < 0 {
                    break;
                }
                let data = e.mem.u32(listeners);
                if e.mem.u32(data.wrapping_add(index as u32 * 4)) != 0 {
                    profile_record(e, LABEL_ENTITY_ISLAND_ACTIVATED);
                    let data = e.mem.u32(listeners);
                    let listener = e.mem.u32(data.wrapping_add(index as u32 * 4));
                    let entities = e.mem.u32(island.addr() + 0x48);
                    let entity = e.mem.u32(entities.wrapping_add(entity_index as u32 * 4));
                    e.vcall(listener, 8, &args![entity]);
                    profile_record(e, PROFILE_END);
                }
            }
            world_callback_util_cleanup_null_pointers(e, Ptr::new(listeners));
        }
        entity_index += 1;
    }

    let lock_count = e
        .get(world, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(world, HkpWorld::m_criticalOperationsLockCount, lock_count);
    if lock_count == 0 && !e.get(world, HkpWorld::m_blockExecutingPendingOperations) {
        if e.get(world, HkpWorld::m_pendingOperationsCount) != 0 {
            e.call(FLUSH_PENDING_OPERATIONS, &args![world]);
        }
        if e.get(world, HkpWorld::m_pendingOperationQueueCount) == 1
            && e.get(world, HkpWorld::m_pendingBodyOperationsCount) != 0
        {
            e.call(FLUSH_PENDING_BODY_OPERATIONS, &args![world]);
        }
    }
}

/// The other listener arrays of the world (data pointer at the offset, `i32` size at +4) and
/// the labels of the profiling records the further callbacks open (the strings at the label
/// addresses are `"TtsetShapeCb"`, `"TtconVioCb"`, `"TtconBrokenCb"`, `"TtconRepairedCb"`,
/// `"TtcpCb"`, `"TtNewCollCb"`, `"TtFinCollCb"`, `"TtworldDelCb"`, `"TtislDeacCb"`,
/// `"TtentDeacCb"`, `"TtpostSimCb"`, `"TtpostIntCb"`, `"TtpostCollCb"`, `"TtimpLimitCb"`,
/// `"TtcpRemCb"` and `"TtcpProcCb"`).
const CONTACT_LISTENERS: u32 = 0x1a0;
const WORLD_LISTENERS: u32 = 0x14c;
const POST_INTEGRATE_LISTENERS: u32 = 0x170;
const POST_COLLIDE_LISTENERS: u32 = 0x17c;
const ISLAND_POST_INTEGRATE_LISTENERS: u32 = 0x188;
const ISLAND_POST_COLLIDE_LISTENERS: u32 = 0x194;
const CONTACT_IMPULSE_LISTENERS: u32 = 0x1ac;
const LABEL_SHAPE_SET: u32 = 0x010c_c9f4;
const LABEL_CONSTRAINT_VIOLATED: u32 = 0x010c_cab0;
const LABEL_CONSTRAINT_BROKEN: u32 = 0x010c_cabc;
const LABEL_CONSTRAINT_REPAIRED: u32 = 0x010c_cacc;
const LABEL_CONTACT_POINT: u32 = 0x010c_ca24;
const LABEL_CONTACT_CONSTRAINT_ADDED: u32 = 0x010c_ca2c;
const LABEL_CONTACT_CONSTRAINT_REMOVED: u32 = 0x010c_ca38;
const LABEL_WORLD_DELETED: u32 = 0x010c_cadc;
const LABEL_ISLAND_DEACTIVATED: u32 = 0x010c_cb10;
const LABEL_ENTITY_ISLAND_DEACTIVATED: u32 = 0x010c_cb04;
const LABEL_POST_SIMULATION: u32 = 0x010c_cb1c;
const LABEL_POST_INTEGRATE: u32 = 0x010c_cb28;
const LABEL_POST_COLLIDE: u32 = 0x010c_cb34;
const LABEL_IMPULSE_LIMIT: u32 = 0x010c_cb44;
const LABEL_CONTACT_POINT_REMOVED: u32 = 0x010c_ca50;
const LABEL_CONTACT_PROCESS: u32 = 0x010c_cb60;

/// `fire_listeners` for a listener method that takes several words: the same walk from the last
/// listener to the first, the same records, the same removal of null entries afterwards.
fn fire_listeners_with_args(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    array_offset: u32,
    label: u32,
    slot: u32,
    listener_args: &[u32],
) {
    let array = world.addr().wrapping_add(array_offset);
    let mut index = e.mem.i32(array + 4);
    loop {
        index = index.wrapping_sub(1);
        if index < 0 {
            break;
        }
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) != 0 {
            profile_record(e, label);
            let data = e.mem.u32(array);
            let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(listener, slot, listener_args);
            profile_record(e, PROFILE_END);
        }
    }
    world_callback_util_cleanup_null_pointers_ov2(e, Ptr::new(array));
}

// Translated from 00d00640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireEntityShapeSet` (Xbox PDB; cdecl, two words): slot `0xc` of every
/// entity listener, with a `"TtsetShapeCb"` record around each call.
pub fn hkp_world_callback_util_fire_entity_shape_set(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    entity: u32,
) {
    fire_listeners(e, world, ENTITY_LISTENERS, LABEL_SHAPE_SET, 0xc, entity);
}

// Translated from 00d008c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::firePhantomShapeSet` (Xbox PDB; cdecl, two words): slot `0xc` of every
/// phantom listener, with a `"TtsetShapeCb"` record around each call.
pub fn hkp_world_callback_util_fire_phantom_shape_set(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    phantom: u32,
) {
    fire_listeners(e, world, PHANTOM_LISTENERS, LABEL_SHAPE_SET, 0xc, phantom);
}

// Translated from 00d00aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireConstraintViolated` (Xbox PDB; cdecl, two words): slot `0x10` of
/// every constraint listener.
pub fn hkp_world_callback_util_fire_constraint_violated(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    constraint: u32,
) {
    fire_listeners(
        e,
        world,
        CONSTRAINT_LISTENERS,
        LABEL_CONSTRAINT_VIOLATED,
        0x10,
        constraint,
    );
}

// Translated from 00d00b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constraint-broken callback (the map gives the function no name; cdecl, two words): slot
/// `0x14` of every constraint listener, with a `"TtconBrokenCb"` record around each call.
pub fn fn_00d00b40(e: &mut Engine, world: Ptr<HkpWorld>, constraint: u32) {
    fire_listeners(
        e,
        world,
        CONSTRAINT_LISTENERS,
        LABEL_CONSTRAINT_BROKEN,
        0x14,
        constraint,
    );
}

// Translated from 00d00bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constraint-repaired callback (the map gives the function no name; cdecl, two words): slot
/// `0x18` of every constraint listener, with a `"TtconRepairedCb"` record around each call.
pub fn fn_00d00bf0(e: &mut Engine, world: Ptr<HkpWorld>, constraint: u32) {
    fire_listeners(
        e,
        world,
        CONSTRAINT_LISTENERS,
        LABEL_CONSTRAINT_REPAIRED,
        0x18,
        constraint,
    );
}

// Translated from 00d00ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactPointCallback` (Xbox PDB; cdecl, two words): slot 0 of every
/// listener of the array at `+0x1a0` (the collision listeners).
pub fn hkp_world_callback_util_fire_contact_point_callback(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    event: u32,
) {
    fire_listeners(e, world, CONTACT_LISTENERS, LABEL_CONTACT_POINT, 0, event);
}

// Translated from 00d00d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactConstraintAddedCallback` (Xbox PDB; cdecl, two words): slot
/// 4 of every listener of the array at `+0x1a0`.
pub fn hkp_world_callback_util_fire_contact_constraint_added_callback(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    event: u32,
) {
    fire_listeners(
        e,
        world,
        CONTACT_LISTENERS,
        LABEL_CONTACT_CONSTRAINT_ADDED,
        4,
        event,
    );
}

// Translated from 00d00de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactConstraintRemovedCallback` (Xbox PDB; cdecl, two words):
/// slot 8 of every listener of the array at `+0x1a0`.
pub fn hkp_world_callback_util_fire_contact_constraint_removed_callback(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    event: u32,
) {
    fire_listeners(
        e,
        world,
        CONTACT_LISTENERS,
        LABEL_CONTACT_CONSTRAINT_REMOVED,
        8,
        event,
    );
}

// Translated from 00d00e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireWorldDeleted` (Xbox PDB; cdecl, one word): slot 4 of every
/// listener of the array at `+0x14c`, each called with the world itself.
pub fn hkp_world_callback_util_fire_world_deleted(e: &mut Engine, world: Ptr<HkpWorld>) {
    fire_listeners(
        e,
        world,
        WORLD_LISTENERS,
        LABEL_WORLD_DELETED,
        4,
        world.addr(),
    );
}

// Translated from 00d00f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireWorldRemoveAll` (Xbox PDB; cdecl, one word): slot 8 of every
/// listener of the array at `+0x14c`, each called with the world itself (same record label as
/// `fireWorldDeleted`).
pub fn hkp_world_callback_util_fire_world_remove_all(e: &mut Engine, world: Ptr<HkpWorld>) {
    fire_listeners(
        e,
        world,
        WORLD_LISTENERS,
        LABEL_WORLD_DELETED,
        8,
        world.addr(),
    );
}

// Translated from 00d011b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireIslandDeactivated` (Xbox PDB; cdecl, two words). As
/// `fireIslandActivated` (`00d00fe0`) with the other labels and methods: with the
/// critical-operations lock count raised for the duration, slot 8 of every island-activation
/// listener gets `island`; then for every entity of the island (array at +0x48, count at +0x4c)
/// that has a listener list at +0x214 (a small array), slot 4 of each of its listeners gets the
/// entity. When the lock count falls back to 0 and operations are not blocked, the pending
/// operation queue is flushed if `m_pendingOperationsCount` is non-zero, and the body-operation
/// queue if `m_pendingOperationQueueCount` is 1 and `m_pendingBodyOperationsCount` is non-zero
/// (the exe ends with a tail jump to the second flush).
pub fn hkp_world_callback_util_fire_island_deactivated(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    island: Ptr,
) {
    let lock_count = e.get(world, HkpWorld::m_criticalOperationsLockCount);
    e.set(
        world,
        HkpWorld::m_criticalOperationsLockCount,
        lock_count.wrapping_add(1),
    );
    fire_listeners(
        e,
        world,
        ISLAND_ACTIVATION_LISTENERS,
        LABEL_ISLAND_DEACTIVATED,
        8,
        island.addr(),
    );

    let mut entity_index = 0i32;
    while entity_index < e.mem.i32(island.addr() + 0x4c) {
        let entities = e.mem.u32(island.addr() + 0x48);
        let entity = e
            .mem
            .u32(entities.wrapping_add((entity_index as u32).wrapping_mul(4)));
        let listeners = e.mem.u32(entity.wrapping_add(0x214));
        if listeners != 0 {
            let mut index = i32::from(e.mem.u16(listeners + 4));
            loop {
                index -= 1;
                if index < 0 {
                    break;
                }
                let data = e.mem.u32(listeners);
                if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) != 0 {
                    profile_record(e, LABEL_ENTITY_ISLAND_DEACTIVATED);
                    let data = e.mem.u32(listeners);
                    let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                    let entities = e.mem.u32(island.addr() + 0x48);
                    let entity = e
                        .mem
                        .u32(entities.wrapping_add((entity_index as u32).wrapping_mul(4)));
                    e.vcall(listener, 4, &args![entity]);
                    profile_record(e, PROFILE_END);
                }
            }
            world_callback_util_cleanup_null_pointers(e, Ptr::new(listeners));
        }
        entity_index += 1;
    }

    let lock_count = e
        .get(world, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(world, HkpWorld::m_criticalOperationsLockCount, lock_count);
    if lock_count == 0 && !e.get(world, HkpWorld::m_blockExecutingPendingOperations) {
        if e.get(world, HkpWorld::m_pendingOperationsCount) != 0 {
            e.call(FLUSH_PENDING_OPERATIONS, &args![world]);
        }
        if e.get(world, HkpWorld::m_pendingOperationQueueCount) == 1
            && e.get(world, HkpWorld::m_pendingBodyOperationsCount) != 0
        {
            e.call(FLUSH_PENDING_BODY_OPERATIONS, &args![world]);
        }
    }
}

// Translated from 00d01380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::firePostSimulationCallback` (Xbox PDB; cdecl, one word): slot 4 of
/// every listener of the array at `+0x164`, each called with the world. Unlike the other
/// callbacks it re-reads the array's size on every pass: when a listener has shrunk the array
/// below the size the walk started from, the index is lowered by the same amount (and the
/// remembered size follows), so that the walk keeps to the same distance from the end.
pub fn hkp_world_callback_util_fire_post_simulation_callback(e: &mut Engine, world: Ptr<HkpWorld>) {
    let array = world.addr().wrapping_add(WORLD_POST_SIMULATION_LISTENERS);
    let mut remembered = e.mem.i32(array + 4);
    let mut index = remembered.wrapping_sub(1);
    if index >= 0 {
        loop {
            let size = e.mem.i32(array + 4);
            if remembered > size {
                index = index.wrapping_add(size.wrapping_sub(remembered));
                remembered = size;
            }
            let data = e.mem.u32(array);
            if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) != 0 {
                profile_record(e, LABEL_POST_SIMULATION);
                let data = e.mem.u32(array);
                let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                e.vcall(listener, 4, &args![world]);
                profile_record(e, PROFILE_END);
            }
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
        }
    }
    world_callback_util_cleanup_null_pointers_ov2(e, Ptr::new(array));
}

// Translated from 00d01440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The post-integrate callback (the map gives the function no name; cdecl, two words): slot 4 of
/// every listener of the array at `+0x170`, each called with `(world, argument)` inside a
/// `"TtpostIntCb"` record.
pub fn fn_00d01440(e: &mut Engine, world: Ptr<HkpWorld>, argument: u32) {
    fire_listeners_with_args(
        e,
        world,
        POST_INTEGRATE_LISTENERS,
        LABEL_POST_INTEGRATE,
        4,
        &args![world, argument],
    );
}

// Translated from 00d014f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::firePostCollideCallback` (Xbox PDB; cdecl, two words): slot 4 of every
/// listener of the array at `+0x17c`, each called with `(world, argument)` inside a
/// `"TtpostCollCb"` record.
pub fn hkp_world_callback_util_fire_post_collide_callback(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    argument: u32,
) {
    fire_listeners_with_args(
        e,
        world,
        POST_COLLIDE_LISTENERS,
        LABEL_POST_COLLIDE,
        4,
        &args![world, argument],
    );
}

// Translated from 00d015a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The island post-integrate callback (the map gives the function no name; cdecl, three words):
/// slot 4 of every listener of the array at `+0x188`, each called with the two words after the
/// world, inside a `"TtpostIntCb"` record.
pub fn fn_00d015a0(e: &mut Engine, world: Ptr<HkpWorld>, second: u32, third: u32) {
    fire_listeners_with_args(
        e,
        world,
        ISLAND_POST_INTEGRATE_LISTENERS,
        LABEL_POST_INTEGRATE,
        4,
        &args![second, third],
    );
}

// Translated from 00d01650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireIslandPostCollideCallback` (Xbox PDB; cdecl, three words): slot 4
/// of every listener of the array at `+0x194`, each called with the two words after the world,
/// inside a `"TtpostCollCb"` record.
pub fn hkp_world_callback_util_fire_island_post_collide_callback(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    second: u32,
    third: u32,
) {
    fire_listeners_with_args(
        e,
        world,
        ISLAND_POST_COLLIDE_LISTENERS,
        LABEL_POST_COLLIDE,
        4,
        &args![second, third],
    );
}

// Translated from 00d01700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactImpulseLimitBreached` (Xbox PDB; cdecl, three words): slot
/// 4 of every listener of the array at `+0x1ac`, each called with the two words after the
/// world, inside a `"TtimpLimitCb"` record.
pub fn hkp_world_callback_util_fire_contact_impulse_limit_breached(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    second: u32,
    third: u32,
) {
    fire_listeners_with_args(
        e,
        world,
        CONTACT_IMPULSE_LISTENERS,
        LABEL_IMPULSE_LIMIT,
        4,
        &args![second, third],
    );
}

// Translated from 00d01900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactPointRemoved` (Xbox PDB; cdecl, two words): clears the
/// word at `event + 0x10`, then calls slot `0x14` of every listener of the array at `+0x1a0`
/// with `event` inside a `"TtcpRemCb"` record.
pub fn hkp_world_callback_util_fire_contact_point_removed(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    event: Ptr,
) {
    e.mem.set_u32(event.addr() + 0x10, 0);
    fire_listeners(
        e,
        world,
        CONTACT_LISTENERS,
        LABEL_CONTACT_POINT_REMOVED,
        0x14,
        event.addr(),
    );
}

// Translated from 00d019b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactProcess` (Xbox PDB; cdecl, two words): clears the word at
/// `event + 8`, then calls slot `0x18` of every listener of the array at `+0x1a0` with `event`
/// inside a `"TtcpProcCb"` record.
pub fn hkp_world_callback_util_fire_contact_process(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    event: Ptr,
) {
    e.mem.set_u32(event.addr() + 8, 0);
    fire_listeners(
        e,
        world,
        CONTACT_LISTENERS,
        LABEL_CONTACT_PROCESS,
        0x18,
        event.addr(),
    );
}

// Translated from 00d01850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCallbackUtil::fireContactPointAdded` (Xbox PDB name from the engine map): cdecl, two stack
/// words (nothing popped): `world` and `event`. Clears the word at +0xc of `event`, then runs the
/// shared loop of the `fire...` functions over the world's array at +0x1a0 (the contact listeners):
/// slot `0x10` of every non-null listener, last to first, with `event`, between the profiling
/// records `010cca44` and the end record, and finally the removal of the null entries.
pub fn hkp_world_callback_util_fire_contact_point_added(
    e: &mut Engine,
    world: Ptr<HkpWorld>,
    event: u32,
) {
    e.mem.set_u32(event + 0xc, 0);
    fire_listeners(e, world, 0x1a0, 0x010c_ca44, 0x10, event);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d003c0,
            hkp_world_callback_util_fire_action_added(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00460,
            hkp_world_callback_util_fire_action_removed(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00500,
            hkp_world_callback_util_fire_entity_added(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d005a0,
            hkp_world_callback_util_fire_entity_removed(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d006e0,
            hkp_world_callback_util_fire_entity_set_motion_type(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00780,
            hkp_world_callback_util_fire_phantom_added(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00820,
            hkp_world_callback_util_fire_phantom_removed(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00960,
            hkp_world_callback_util_fire_constraint_added(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00a00,
            hkp_world_callback_util_fire_constraint_removed(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00fe0,
            hkp_world_callback_util_fire_island_activated(Ptr<HkpWorld>, Ptr)
        ),
        entry!(
            0x00d017b0,
            hkp_world_callback_util_fire_inactive_entity_moved(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00640,
            hkp_world_callback_util_fire_entity_shape_set(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d008c0,
            hkp_world_callback_util_fire_phantom_shape_set(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00aa0,
            hkp_world_callback_util_fire_constraint_violated(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00d00b40, fn_00d00b40(Ptr<HkpWorld>, u32)),
        entry!(0x00d00bf0, fn_00d00bf0(Ptr<HkpWorld>, u32)),
        entry!(
            0x00d00ca0,
            hkp_world_callback_util_fire_contact_point_callback(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00d40,
            hkp_world_callback_util_fire_contact_constraint_added_callback(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00de0,
            hkp_world_callback_util_fire_contact_constraint_removed_callback(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00d00e80,
            hkp_world_callback_util_fire_world_deleted(Ptr<HkpWorld>)
        ),
        entry!(
            0x00d00f30,
            hkp_world_callback_util_fire_world_remove_all(Ptr<HkpWorld>)
        ),
        entry!(
            0x00d011b0,
            hkp_world_callback_util_fire_island_deactivated(Ptr<HkpWorld>, Ptr)
        ),
        entry!(
            0x00d01380,
            hkp_world_callback_util_fire_post_simulation_callback(Ptr<HkpWorld>)
        ),
        entry!(0x00d01440, fn_00d01440(Ptr<HkpWorld>, u32)),
        entry!(
            0x00d014f0,
            hkp_world_callback_util_fire_post_collide_callback(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00d015a0, fn_00d015a0(Ptr<HkpWorld>, u32, u32)),
        entry!(
            0x00d01650,
            hkp_world_callback_util_fire_island_post_collide_callback(Ptr<HkpWorld>, u32, u32)
        ),
        entry!(
            0x00d01700,
            hkp_world_callback_util_fire_contact_impulse_limit_breached(Ptr<HkpWorld>, u32, u32)
        ),
        entry!(
            0x00d01900,
            hkp_world_callback_util_fire_contact_point_removed(Ptr<HkpWorld>, Ptr)
        ),
        entry!(
            0x00d019b0,
            hkp_world_callback_util_fire_contact_process(Ptr<HkpWorld>, Ptr)
        ),
        entry!(
            0x00d0_1850,
            hkp_world_callback_util_fire_contact_point_added(Ptr<HkpWorld>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::hkpworld::profiling_support::{labels, profiling_engine};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FAKE_SLOT_4: u32 = 0x0900_0004;
    const FAKE_SLOT_8: u32 = 0x0900_0008;
    const FAKE_SLOT_10: u32 = 0x0900_0010;

    type Events = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// Doubles for the three vtable slots the callbacks use; each logs
    /// `(slot, listener, argument)`.
    fn logging_slots(e: &mut Engine) -> Events {
        let events: Events = Rc::new(RefCell::new(vec![]));
        for (target, slot) in [(FAKE_SLOT_4, 4), (FAKE_SLOT_8, 8), (FAKE_SLOT_10, 0x10)] {
            let sink = events.clone();
            e.register_double(target, move |_, a| {
                sink.borrow_mut().push((slot, a[0], a[1]));
                Ret::default()
            });
        }
        events
    }

    fn listener(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 4, FAKE_SLOT_4);
        e.mem.set_u32(table + 8, FAKE_SLOT_8);
        e.mem.set_u32(table + 0x10, FAKE_SLOT_10);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, table);
        object
    }

    /// Writes an `hkArray` (data pointer, size) at `at` holding `values`.
    fn put_array(e: &mut Engine, at: u32, values: &[u32]) {
        let data = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *value);
        }
        e.mem.set_u32(at, data);
        e.mem.set_i32(at + 4, values.len() as i32);
    }

    fn array_values(e: &Engine, at: u32) -> Vec<u32> {
        let data = e.mem.u32(at);
        (0..e.mem.i32(at + 4) as u32)
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    /// Checks one `fire...` function: three listeners (the middle slot null) in the array at
    /// `array_offset`; the callback goes to the two real ones, last first, inside the
    /// profiling records of `label`, and the null entry is removed.
    fn check_fire(address: u32, array_offset: u32, slot: u32, label: u32) {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let first = listener(&mut e);
        let last = listener(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(&mut e, world.addr() + array_offset, &[first, 0, last]);
        e.call(address, &args![world, 0x7777u32]);
        assert_eq!(
            *events.borrow(),
            vec![(slot, last, 0x7777), (slot, first, 0x7777)]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![label, PROFILE_END, label, PROFILE_END]
        );
        assert_eq!(
            array_values(&e, world.addr() + array_offset),
            vec![first, last]
        );
    }

    #[test]
    fn fire_action_added_calls_slot_4_of_the_action_listeners() {
        check_fire(0x00d003c0, ACTION_LISTENERS, 4, LABEL_ACTION_ADDED);
    }

    #[test]
    fn fire_action_removed_calls_slot_8_of_the_action_listeners() {
        check_fire(0x00d00460, ACTION_LISTENERS, 8, LABEL_ACTION_REMOVED);
    }

    #[test]
    fn fire_entity_added_calls_slot_4_of_the_entity_listeners() {
        check_fire(0x00d00500, ENTITY_LISTENERS, 4, LABEL_ENTITY_ADDED);
    }

    #[test]
    fn fire_entity_removed_calls_slot_8_of_the_entity_listeners() {
        check_fire(0x00d005a0, ENTITY_LISTENERS, 8, LABEL_ENTITY_REMOVED);
    }

    #[test]
    fn fire_entity_set_motion_type_calls_slot_0x10_of_the_entity_listeners() {
        check_fire(
            0x00d006e0,
            ENTITY_LISTENERS,
            0x10,
            LABEL_ENTITY_SET_MOTION_TYPE,
        );
    }

    #[test]
    fn fire_phantom_added_calls_slot_4_of_the_phantom_listeners() {
        check_fire(0x00d00780, PHANTOM_LISTENERS, 4, LABEL_PHANTOM_ADDED);
    }

    #[test]
    fn fire_phantom_removed_calls_slot_8_of_the_phantom_listeners() {
        check_fire(0x00d00820, PHANTOM_LISTENERS, 8, LABEL_PHANTOM_REMOVED);
    }

    #[test]
    fn fire_constraint_added_calls_slot_4_of_the_constraint_listeners() {
        check_fire(0x00d00960, CONSTRAINT_LISTENERS, 4, LABEL_CONSTRAINT_ADDED);
    }

    #[test]
    fn fire_constraint_removed_calls_slot_8_of_the_constraint_listeners() {
        check_fire(
            0x00d00a00,
            CONSTRAINT_LISTENERS,
            8,
            LABEL_CONSTRAINT_REMOVED,
        );
    }

    #[test]
    fn fire_inactive_entity_moved_calls_slot_8_of_the_array_at_0x164() {
        check_fire(
            0x00d017b0,
            WORLD_POST_SIMULATION_LISTENERS,
            8,
            LABEL_INACTIVE_ENTITY_MOVED,
        );
    }

    #[test]
    fn firing_with_a_full_profiling_stream_still_calls_the_listeners() {
        let (mut e, stream, start) = profiling_engine(0);
        // The cursor is at the limit: no record fits.
        let events = logging_slots(&mut e);
        let one = listener(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(&mut e, world.addr() + ACTION_LISTENERS, &[one]);
        e.call(0x00d003c0, &args![world, 5u32]);
        assert_eq!(*events.borrow(), vec![(4, one, 5)]);
        assert!(labels(&e, stream, start).is_empty());
    }

    #[test]
    fn cleanup_removes_every_null_entry_and_keeps_the_order() {
        let mut e = Engine::new();
        let at = e.mem.alloc(8);
        put_array(&mut e, at, &[0, 11, 0, 0, 12, 13, 0]);
        world_callback_util_cleanup_null_pointers_ov2(&mut e, Ptr::new(at));
        assert_eq!(array_values(&e, at), vec![11, 12, 13]);
        // Empty and null-free arrays are left alone.
        put_array(&mut e, at, &[]);
        world_callback_util_cleanup_null_pointers_ov2(&mut e, Ptr::new(at));
        assert_eq!(array_values(&e, at), Vec::<u32>::new());
        put_array(&mut e, at, &[1, 2]);
        world_callback_util_cleanup_null_pointers_ov2(&mut e, Ptr::new(at));
        assert_eq!(array_values(&e, at), vec![1, 2]);
    }

    /// A small array (data pointer, `u16` size, `u16` capacity) at `at`.
    fn put_small_array(e: &mut Engine, at: u32, values: &[u32]) {
        put_array(e, at, values);
        e.mem.set_u32(at + 4, values.len() as u32);
    }

    #[test]
    fn small_array_cleanup_removes_every_null_entry_and_keeps_the_order() {
        let mut e = Engine::new();
        let at = e.mem.alloc(8);
        put_small_array(&mut e, at, &[0, 0, 21, 0, 22]);
        world_callback_util_cleanup_null_pointers(&mut e, Ptr::new(at));
        assert_eq!(e.mem.u16(at + 4), 2);
        assert_eq!(array_values(&e, at)[..2], [21, 22]);
        put_small_array(&mut e, at, &[]);
        world_callback_util_cleanup_null_pointers(&mut e, Ptr::new(at));
        assert_eq!(e.mem.u16(at + 4), 0);
    }

    /// A world with one island-activation listener, and an island with two entities; the
    /// first entity has two listeners (one null), the second has no list.
    struct IslandScene {
        world: Ptr<HkpWorld>,
        island: u32,
        entity_listener: u32,
        entities: [u32; 2],
        island_listener: u32,
    }

    fn island_scene(e: &mut Engine) -> IslandScene {
        let island_listener = listener(e);
        let entity_listener = listener(e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(
            e,
            world.addr() + ISLAND_ACTIVATION_LISTENERS,
            &[island_listener],
        );
        let entities = [e.mem.alloc(0x220), e.mem.alloc(0x220)];
        let list = e.mem.alloc(8);
        put_small_array(e, list, &[0, entity_listener]);
        e.mem.set_u32(entities[0] + 0x214, list);
        let island = e.mem.alloc(0x60);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, entities[0]);
        e.mem.set_u32(data + 4, entities[1]);
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_i32(island + 0x4c, 2);
        IslandScene {
            world,
            island,
            entity_listener,
            entities,
            island_listener,
        }
    }

    #[test]
    fn island_activated_tells_the_listeners_and_the_entities_listeners() {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let scene = island_scene(&mut e);
        e.set(scene.world, HkpWorld::m_criticalOperationsLockCount, 0);
        e.set(
            scene.world,
            HkpWorld::m_blockExecutingPendingOperations,
            true,
        );
        e.call(0x00d00fe0, &args![scene.world, scene.island]);
        assert_eq!(
            *events.borrow(),
            vec![
                (4, scene.island_listener, scene.island),
                (8, scene.entity_listener, scene.entities[0]),
            ]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![
                LABEL_ISLAND_ACTIVATED,
                PROFILE_END,
                LABEL_ENTITY_ISLAND_ACTIVATED,
                PROFILE_END
            ]
        );
        // The entity's null listener was removed.
        let list = e.mem.u32(scene.entities[0] + 0x214);
        assert_eq!(e.mem.u16(list + 4), 1);
        assert_eq!(
            e.get(scene.world, HkpWorld::m_criticalOperationsLockCount),
            0
        );
    }

    #[test]
    fn island_activated_flushes_the_queues_when_the_lock_falls_to_zero() {
        let (mut e, ..) = profiling_engine(8);
        logging_slots(&mut e);
        e.call_log = Some(vec![]);
        for flush in [FLUSH_PENDING_OPERATIONS, FLUSH_PENDING_BODY_OPERATIONS] {
            e.register(flush, |_, _| Ret::default());
        }
        let scene = island_scene(&mut e);
        e.set(scene.world, HkpWorld::m_pendingOperationsCount, 3);
        e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 1);
        e.set(scene.world, HkpWorld::m_pendingBodyOperationsCount, 2);
        e.call(0x00d00fe0, &args![scene.world, scene.island]);
        let flushed: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| {
                *address == FLUSH_PENDING_OPERATIONS || *address == FLUSH_PENDING_BODY_OPERATIONS
            })
            .cloned()
            .collect();
        assert_eq!(
            flushed,
            vec![
                (FLUSH_PENDING_OPERATIONS, vec![scene.world.addr()]),
                (FLUSH_PENDING_BODY_OPERATIONS, vec![scene.world.addr()]),
            ]
        );
    }

    #[test]
    fn island_activated_keeps_the_queues_while_still_locked_or_blocked() {
        for (initial_lock, blocked) in [(1, false), (0, true)] {
            let (mut e, ..) = profiling_engine(8);
            logging_slots(&mut e);
            e.call_log = Some(vec![]);
            let scene = island_scene(&mut e);
            e.set(
                scene.world,
                HkpWorld::m_criticalOperationsLockCount,
                initial_lock,
            );
            e.set(
                scene.world,
                HkpWorld::m_blockExecutingPendingOperations,
                blocked,
            );
            e.set(scene.world, HkpWorld::m_pendingOperationsCount, 3);
            e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 1);
            e.set(scene.world, HkpWorld::m_pendingBodyOperationsCount, 2);
            e.call(0x00d00fe0, &args![scene.world, scene.island]);
            let log = e.call_log.take().unwrap();
            assert!(!log.iter().any(|(address, _)| {
                *address == FLUSH_PENDING_OPERATIONS || *address == FLUSH_PENDING_BODY_OPERATIONS
            }));
            assert_eq!(
                e.get(scene.world, HkpWorld::m_criticalOperationsLockCount),
                initial_lock
            );
        }
    }

    #[test]
    fn island_activated_flushes_only_what_is_pending() {
        let (mut e, ..) = profiling_engine(8);
        logging_slots(&mut e);
        e.call_log = Some(vec![]);
        for flush in [FLUSH_PENDING_OPERATIONS, FLUSH_PENDING_BODY_OPERATIONS] {
            e.register(flush, |_, _| Ret::default());
        }
        let scene = island_scene(&mut e);
        // Operations pending, but the queue count is 2: only the first flush runs.
        e.set(scene.world, HkpWorld::m_pendingOperationsCount, 1);
        e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 2);
        e.set(scene.world, HkpWorld::m_pendingBodyOperationsCount, 2);
        e.call(0x00d00fe0, &args![scene.world, scene.island]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(address, _)| *address == FLUSH_PENDING_OPERATIONS));
        assert!(!log
            .iter()
            .any(|(address, _)| *address == FLUSH_PENDING_BODY_OPERATIONS));
    }
}

#[cfg(test)]
mod havok_fire_tests {
    use super::super::hkpworld::profiling_support::{labels, profiling_engine};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// `(slot, listener, words after the listener)` of every listener call.
    type Events = Rc<RefCell<Vec<(u32, u32, Vec<u32>)>>>;

    const SLOTS: [u32; 7] = [0, 4, 8, 0xc, 0x10, 0x14, 0x18];

    /// Doubles for every vtable slot the callbacks use; each logs its call.
    fn logging_slots(e: &mut Engine) -> Events {
        let events: Events = Rc::new(RefCell::new(vec![]));
        for slot in SLOTS {
            let sink = events.clone();
            e.register_double(0x0900_0100 + slot, move |_, a| {
                sink.borrow_mut().push((slot, a[0], a[1..].to_vec()));
                Ret::default()
            });
        }
        events
    }

    fn listener(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x20);
        for slot in SLOTS {
            e.mem.set_u32(table + slot, 0x0900_0100 + slot);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, table);
        object
    }

    /// Writes an `hkArray` (data pointer, size) at `at` holding `values`.
    fn put_array(e: &mut Engine, at: u32, values: &[u32]) {
        let data = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *value);
        }
        e.mem.set_u32(at, data);
        e.mem.set_i32(at + 4, values.len() as i32);
    }

    fn array_values(e: &Engine, at: u32) -> Vec<u32> {
        let data = e.mem.u32(at);
        (0..e.mem.i32(at + 4) as u32)
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    /// A small array (data pointer, `u16` size, `u16` capacity) at `at`.
    fn put_small_array(e: &mut Engine, at: u32, values: &[u32]) {
        put_array(e, at, values);
        e.mem.set_u32(at + 4, values.len() as u32);
    }

    /// Checks one `fire...` function: three listeners (the middle slot null) in the array at
    /// `array_offset`; the call is `(world, extra...)`, `slot` goes to the two real listeners,
    /// last first, with the words `expected(world)`, inside the records of `label`, and the
    /// null entry is removed.
    fn check_fire(
        address: u32,
        array_offset: u32,
        slot: u32,
        label: u32,
        extra: &[u32],
        expected: impl Fn(u32) -> Vec<u32>,
    ) {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let first = listener(&mut e);
        let last = listener(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(&mut e, world.addr() + array_offset, &[first, 0, last]);
        let mut words = vec![world.addr()];
        words.extend_from_slice(extra);
        e.call(address, &words);
        let listener_words = expected(world.addr());
        assert_eq!(
            *events.borrow(),
            vec![
                (slot, last, listener_words.clone()),
                (slot, first, listener_words)
            ]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![label, PROFILE_END, label, PROFILE_END]
        );
        assert_eq!(
            array_values(&e, world.addr() + array_offset),
            vec![first, last]
        );
    }

    #[test]
    fn fire_entity_shape_set_calls_slot_0xc_of_the_entity_listeners() {
        check_fire(
            0x00d00640,
            ENTITY_LISTENERS,
            0xc,
            LABEL_SHAPE_SET,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn fire_phantom_shape_set_calls_slot_0xc_of_the_phantom_listeners() {
        check_fire(
            0x00d008c0,
            PHANTOM_LISTENERS,
            0xc,
            LABEL_SHAPE_SET,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn fire_constraint_violated_calls_slot_0x10_of_the_constraint_listeners() {
        check_fire(
            0x00d00aa0,
            CONSTRAINT_LISTENERS,
            0x10,
            LABEL_CONSTRAINT_VIOLATED,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn the_constraint_broken_callback_calls_slot_0x14() {
        check_fire(
            0x00d00b40,
            CONSTRAINT_LISTENERS,
            0x14,
            LABEL_CONSTRAINT_BROKEN,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn the_constraint_repaired_callback_calls_slot_0x18() {
        check_fire(
            0x00d00bf0,
            CONSTRAINT_LISTENERS,
            0x18,
            LABEL_CONSTRAINT_REPAIRED,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn fire_contact_point_callback_calls_slot_0_of_the_contact_listeners() {
        check_fire(
            0x00d00ca0,
            CONTACT_LISTENERS,
            0,
            LABEL_CONTACT_POINT,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn fire_contact_constraint_added_calls_slot_4_of_the_contact_listeners() {
        check_fire(
            0x00d00d40,
            CONTACT_LISTENERS,
            4,
            LABEL_CONTACT_CONSTRAINT_ADDED,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn fire_contact_constraint_removed_calls_slot_8_of_the_contact_listeners() {
        check_fire(
            0x00d00de0,
            CONTACT_LISTENERS,
            8,
            LABEL_CONTACT_CONSTRAINT_REMOVED,
            &[0x7777],
            |_| vec![0x7777],
        );
    }

    #[test]
    fn fire_world_deleted_hands_the_world_to_slot_4() {
        check_fire(
            0x00d00e80,
            WORLD_LISTENERS,
            4,
            LABEL_WORLD_DELETED,
            &[],
            |world| vec![world],
        );
    }

    #[test]
    fn fire_world_remove_all_hands_the_world_to_slot_8() {
        check_fire(
            0x00d00f30,
            WORLD_LISTENERS,
            8,
            LABEL_WORLD_DELETED,
            &[],
            |world| vec![world],
        );
    }

    #[test]
    fn the_post_integrate_callback_passes_the_world_and_its_argument() {
        check_fire(
            0x00d01440,
            POST_INTEGRATE_LISTENERS,
            4,
            LABEL_POST_INTEGRATE,
            &[0x7777],
            |world| vec![world, 0x7777],
        );
    }

    #[test]
    fn fire_post_collide_passes_the_world_and_its_argument() {
        check_fire(
            0x00d014f0,
            POST_COLLIDE_LISTENERS,
            4,
            LABEL_POST_COLLIDE,
            &[0x7777],
            |world| vec![world, 0x7777],
        );
    }

    #[test]
    fn the_island_post_integrate_callback_passes_its_two_words() {
        check_fire(
            0x00d015a0,
            ISLAND_POST_INTEGRATE_LISTENERS,
            4,
            LABEL_POST_INTEGRATE,
            &[0x7777, 0x8888],
            |_| vec![0x7777, 0x8888],
        );
    }

    #[test]
    fn fire_island_post_collide_passes_its_two_words() {
        check_fire(
            0x00d01650,
            ISLAND_POST_COLLIDE_LISTENERS,
            4,
            LABEL_POST_COLLIDE,
            &[0x7777, 0x8888],
            |_| vec![0x7777, 0x8888],
        );
    }

    #[test]
    fn fire_contact_impulse_limit_breached_passes_its_two_words() {
        check_fire(
            0x00d01700,
            CONTACT_IMPULSE_LISTENERS,
            4,
            LABEL_IMPULSE_LIMIT,
            &[0x7777, 0x8888],
            |_| vec![0x7777, 0x8888],
        );
    }

    #[test]
    fn fire_post_simulation_hands_the_world_to_slot_4() {
        check_fire(
            0x00d01380,
            WORLD_POST_SIMULATION_LISTENERS,
            4,
            LABEL_POST_SIMULATION,
            &[],
            |world| vec![world],
        );
    }

    #[test]
    fn fire_post_simulation_follows_a_listener_that_shrinks_the_array() {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let a = listener(&mut e);
        let b = listener(&mut e);
        let c = listener(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        let array = world.addr() + WORLD_POST_SIMULATION_LISTENERS;
        put_array(&mut e, array, &[a, b, c]);
        // The listener called first (the last entry) removes the middle entry and shrinks the
        // array: the walk moves down by the same amount and so reaches the first entry next.
        let sink = events.clone();
        e.register_double(0x0900_0104, move |e, args| {
            sink.borrow_mut().push((4, args[0], args[1..].to_vec()));
            let data = e.mem.u32(array);
            e.mem.set_u32(data + 4, e.mem.u32(data + 8));
            e.mem.set_i32(array + 4, 2);
            Ret::default()
        });
        e.call(0x00d01380, &args![world]);
        assert_eq!(
            *events.borrow(),
            vec![(4, c, vec![world.addr()]), (4, a, vec![world.addr()])]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![
                LABEL_POST_SIMULATION,
                PROFILE_END,
                LABEL_POST_SIMULATION,
                PROFILE_END
            ]
        );
        assert_eq!(array_values(&e, array), vec![a, c]);
    }

    #[test]
    fn fire_post_simulation_with_an_empty_array_only_cleans_up() {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(&mut e, world.addr() + WORLD_POST_SIMULATION_LISTENERS, &[]);
        e.call(0x00d01380, &args![world]);
        assert!(events.borrow().is_empty());
        assert!(labels(&e, stream, start).is_empty());
    }

    #[test]
    fn contact_point_removed_clears_the_event_word_and_calls_slot_0x14() {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let one = listener(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(&mut e, world.addr() + CONTACT_LISTENERS, &[one]);
        let event = e.mem.alloc(0x20);
        e.mem.set_u32(event + 0x10, 0xffff);
        e.mem.set_u32(event + 8, 0xeeee);
        e.call(0x00d01900, &args![world, event]);
        assert_eq!(e.mem.u32(event + 0x10), 0);
        assert_eq!(e.mem.u32(event + 8), 0xeeee);
        assert_eq!(*events.borrow(), vec![(0x14, one, vec![event])]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_CONTACT_POINT_REMOVED, PROFILE_END]
        );
    }

    #[test]
    fn contact_process_clears_the_event_word_and_calls_slot_0x18() {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let one = listener(&mut e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(&mut e, world.addr() + CONTACT_LISTENERS, &[one]);
        let event = e.mem.alloc(0x20);
        e.mem.set_u32(event + 0x10, 0xffff);
        e.mem.set_u32(event + 8, 0xeeee);
        e.call(0x00d019b0, &args![world, event]);
        assert_eq!(e.mem.u32(event + 8), 0);
        assert_eq!(e.mem.u32(event + 0x10), 0xffff);
        assert_eq!(*events.borrow(), vec![(0x18, one, vec![event])]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_CONTACT_PROCESS, PROFILE_END]
        );
    }

    /// A world with one island-activation listener, and an island with two entities; the
    /// first entity has two listeners (one null), the second has no list.
    struct IslandScene {
        world: Ptr<HkpWorld>,
        island: u32,
        entity_listener: u32,
        entities: [u32; 2],
        island_listener: u32,
    }

    fn island_scene(e: &mut Engine) -> IslandScene {
        let island_listener = listener(e);
        let entity_listener = listener(e);
        let world: Ptr<HkpWorld> = e.new_object();
        put_array(
            e,
            world.addr() + ISLAND_ACTIVATION_LISTENERS,
            &[island_listener],
        );
        let entities = [e.mem.alloc(0x220), e.mem.alloc(0x220)];
        let list = e.mem.alloc(8);
        put_small_array(e, list, &[0, entity_listener]);
        e.mem.set_u32(entities[0] + 0x214, list);
        let island = e.mem.alloc(0x60);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, entities[0]);
        e.mem.set_u32(data + 4, entities[1]);
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_i32(island + 0x4c, 2);
        IslandScene {
            world,
            island,
            entity_listener,
            entities,
            island_listener,
        }
    }

    fn flushed(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| {
                *address == FLUSH_PENDING_OPERATIONS || *address == FLUSH_PENDING_BODY_OPERATIONS
            })
            .cloned()
            .collect()
    }

    #[test]
    fn island_deactivated_tells_the_listeners_and_the_entities_listeners() {
        let (mut e, stream, start) = profiling_engine(8);
        let events = logging_slots(&mut e);
        let scene = island_scene(&mut e);
        e.set(scene.world, HkpWorld::m_criticalOperationsLockCount, 0);
        e.set(
            scene.world,
            HkpWorld::m_blockExecutingPendingOperations,
            true,
        );
        e.call(0x00d011b0, &args![scene.world, scene.island]);
        assert_eq!(
            *events.borrow(),
            vec![
                (8, scene.island_listener, vec![scene.island]),
                (4, scene.entity_listener, vec![scene.entities[0]]),
            ]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![
                LABEL_ISLAND_DEACTIVATED,
                PROFILE_END,
                LABEL_ENTITY_ISLAND_DEACTIVATED,
                PROFILE_END
            ]
        );
        // The entity's null listener was removed.
        let list = e.mem.u32(scene.entities[0] + 0x214);
        assert_eq!(e.mem.u16(list + 4), 1);
        assert_eq!(
            e.get(scene.world, HkpWorld::m_criticalOperationsLockCount),
            0
        );
    }

    #[test]
    fn island_deactivated_raises_the_lock_while_the_listeners_run() {
        let (mut e, ..) = profiling_engine(8);
        let scene = island_scene(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        let world = scene.world;
        e.register_double(0x0900_0108, move |e, _| {
            sink.borrow_mut()
                .push(e.get(world, HkpWorld::m_criticalOperationsLockCount));
            Ret::default()
        });
        e.register_double(0x0900_0104, |_, _| Ret::default());
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 4);
        e.call(0x00d011b0, &args![world, scene.island]);
        assert_eq!(*seen.borrow(), vec![5]);
        assert_eq!(e.get(world, HkpWorld::m_criticalOperationsLockCount), 4);
    }

    #[test]
    fn island_deactivated_flushes_the_queues_when_the_lock_falls_to_zero() {
        let (mut e, ..) = profiling_engine(8);
        logging_slots(&mut e);
        e.call_log = Some(vec![]);
        for flush in [FLUSH_PENDING_OPERATIONS, FLUSH_PENDING_BODY_OPERATIONS] {
            e.register(flush, |_, _| Ret::default());
        }
        let scene = island_scene(&mut e);
        e.set(scene.world, HkpWorld::m_pendingOperationsCount, 3);
        e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 1);
        e.set(scene.world, HkpWorld::m_pendingBodyOperationsCount, 2);
        e.call(0x00d011b0, &args![scene.world, scene.island]);
        assert_eq!(
            flushed(&e),
            vec![
                (FLUSH_PENDING_OPERATIONS, vec![scene.world.addr()]),
                (FLUSH_PENDING_BODY_OPERATIONS, vec![scene.world.addr()]),
            ]
        );
    }

    #[test]
    fn island_deactivated_keeps_the_queues_while_still_locked_or_blocked() {
        for (initial_lock, blocked) in [(1, false), (0, true)] {
            let (mut e, ..) = profiling_engine(8);
            logging_slots(&mut e);
            e.call_log = Some(vec![]);
            let scene = island_scene(&mut e);
            e.set(
                scene.world,
                HkpWorld::m_criticalOperationsLockCount,
                initial_lock,
            );
            e.set(
                scene.world,
                HkpWorld::m_blockExecutingPendingOperations,
                blocked,
            );
            e.set(scene.world, HkpWorld::m_pendingOperationsCount, 3);
            e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 1);
            e.set(scene.world, HkpWorld::m_pendingBodyOperationsCount, 2);
            e.call(0x00d011b0, &args![scene.world, scene.island]);
            assert!(flushed(&e).is_empty());
            assert_eq!(
                e.get(scene.world, HkpWorld::m_criticalOperationsLockCount),
                initial_lock
            );
        }
    }

    #[test]
    fn island_deactivated_flushes_only_what_is_pending() {
        let (mut e, ..) = profiling_engine(8);
        logging_slots(&mut e);
        e.call_log = Some(vec![]);
        for flush in [FLUSH_PENDING_OPERATIONS, FLUSH_PENDING_BODY_OPERATIONS] {
            e.register(flush, |_, _| Ret::default());
        }
        let scene = island_scene(&mut e);
        // Operations pending, but the queue count is 2: only the first flush runs.
        e.set(scene.world, HkpWorld::m_pendingOperationsCount, 1);
        e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 2);
        e.set(scene.world, HkpWorld::m_pendingBodyOperationsCount, 2);
        e.call(0x00d011b0, &args![scene.world, scene.island]);
        let flushes = flushed(&e);
        assert_eq!(flushes.len(), 1);
        assert_eq!(flushes[0].0, FLUSH_PENDING_OPERATIONS);
        // The body queue alone, with no pending operations.
        e.call_log = Some(vec![]);
        e.set(scene.world, HkpWorld::m_pendingOperationsCount, 0);
        e.set(scene.world, HkpWorld::m_pendingOperationQueueCount, 1);
        e.call(0x00d011b0, &args![scene.world, scene.island]);
        let flushes = flushed(&e);
        assert_eq!(flushes.len(), 1);
        assert_eq!(flushes[0].0, FLUSH_PENDING_BODY_OPERATIONS);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::units::havok_sdk::hkpworld::profiling_support::{labels, profiling_engine};

    /// A listener whose slot 0x10 (the contact callback) is a double that logs (listener, event).
    #[test]
    fn hkp_world_callback_util_fire_contact_point_added_calls_slot_0x10_of_each_listener() {
        let (mut e, stream, area) = profiling_engine(8);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x10, 0x0900_0010);
        let make = |e: &mut Engine| {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, table);
            object
        };
        let first = make(&mut e);
        let last = make(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0010, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let world = e.mem.alloc(0x200);
        let data = e.mem.alloc(16);
        for (i, listener) in [first, 0, last].iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *listener);
        }
        e.mem.set_u32(world + 0x1a0, data);
        e.mem.set_i32(world + 0x1a4, 3);
        let event = e.mem.alloc(0x20);
        e.mem.set_u32(event + 0xc, 0x99);
        e.call(0x00d0_1850, &args![world, event]);
        assert_eq!(e.mem.u32(event + 0xc), 0);
        // Last to first; the null entry is skipped and then removed.
        assert_eq!(*seen.borrow(), vec![(last, event), (first, event)]);
        assert_eq!(e.mem.i32(world + 0x1a4), 2);
        assert_eq!(e.mem.u32(data), first);
        assert_eq!(e.mem.u32(data + 4), last);
        assert_eq!(
            labels(&e, stream, area),
            vec![0x010c_ca44, 0x010c_41f8, 0x010c_ca44, 0x010c_41f8]
        );
    }

    /// With no listeners only the event word is cleared.
    #[test]
    fn hkp_world_callback_util_fire_contact_point_added_with_no_listeners_clears_the_event_word() {
        let (mut e, stream, area) = profiling_engine(4);
        let world = e.mem.alloc(0x200);
        let event = e.mem.alloc(0x20);
        e.mem.set_u32(event + 0xc, 0x99);
        e.call(0x00d0_1850, &args![world, event]);
        assert_eq!(e.mem.u32(event + 0xc), 0);
        assert!(labels(&e, stream, area).is_empty());
    }
}
