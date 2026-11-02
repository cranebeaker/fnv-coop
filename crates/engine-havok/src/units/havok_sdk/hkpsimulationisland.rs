//! `Havok SDK/hkpsimulationisland.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpSimulationIsland` (0x6c bytes on the Xbox, offsets checked against the
//! PC constructor `00d07830`) keeps its actions in an `hkArray` at +0x38 and
//! its entities in an `hkInplaceArray` at +0x48; an `hkArray` is the data
//! pointer, the size and a capacity word whose low 30 bits are the capacity
//! (bit 31 says the data is not owned). A full array is grown through
//! `hkArrayUtil::_reserveMore` (`00c90940`) with the memory router's
//! allocator (`00c85750`, field +0x10).
//!
//! The island's flag byte at +0x25 holds four 2-bit fields (Xbox PDB:
//! `m_splitCheckRequested`, `m_sparseEnabled`, `m_actionListCleanupNeeded`,
//! `m_allowIslandLocking`); the PDB gives their bit positions for the Xbox
//! (counting from the high end), and the PC code's masks are consistent with
//! the fields counting from bit 0 in declaration order, which is how the
//! translations below name them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (cdecl: allocator, the `hkArray`, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpSimulationIsland`'s vtable (the constructor stores it).
const ISLAND_VTABLE: u32 = 0x010c_cf28;
/// `float` stored in `m_timeOfDeactivation` by the constructor.
const TIME_OF_DEACTIVATION_INITIAL: u32 = 0x0104_eb20;

layout! {
    /// `hkpSimulationIsland` (Xbox PDB), 0x6c bytes.
    pub struct HkpSimulationIsland: 0x6c {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_constraintInfo.m_maxSizeOfSchema` (Xbox PDB).
        0x08 m_maxSizeOfSchema: i32,
        /// `m_constraintInfo.m_sizeOfSchemas` (Xbox PDB).
        0x0C m_sizeOfSchemas: i32,
        /// `m_constraintInfo.m_numSolverResults` (Xbox PDB).
        0x10 m_numSolverResults: i32,
        /// `m_constraintInfo.m_numSolverElemTemps` (Xbox PDB).
        0x14 m_numSolverElemTemps: i32,
        /// `m_world` (Xbox PDB): `hkpWorld*`.
        0x18 m_world: Ptr,
        /// `m_numConstraints` (Xbox PDB).
        0x1C m_numConstraints: i32,
        /// `m_storageIndex` (Xbox PDB).
        0x20 m_storageIndex: u16,
        /// `m_dirtyListIndex` (Xbox PDB).
        0x22 m_dirtyListIndex: u16,
        /// `m_splitCheckFrameCounter` (Xbox PDB).
        0x24 m_splitCheckFrameCounter: u8,
        /// The first flag byte: four 2-bit fields (see the module notes).
        0x25 flags: u8,
        /// The second flag byte (the constructor stores 5).
        0x26 flags2: u8,
        /// `m_multiThreadCheck.m_threadId` (Xbox PDB).
        0x28 m_threadId: u32,
        /// `m_multiThreadCheck.m_markCount` (Xbox PDB).
        0x2C m_markCount: u16,
        /// `m_timeSinceLastHighFrequencyCheck` (Xbox PDB).
        0x30 m_timeSinceLastHighFrequencyCheck: f32,
        /// `m_timeSinceLastLowFrequencyCheck` (Xbox PDB).
        0x34 m_timeSinceLastLowFrequencyCheck: f32,
        /// `m_actions` (Xbox PDB): the `hkArray` data pointer.
        0x38 m_actions: Ptr,
        /// Size of `m_actions`.
        0x3C m_actionsSize: i32,
        /// Capacity word of `m_actions`.
        0x40 m_actionsCapacity: u32,
        /// `m_timeOfDeactivation` (Xbox PDB).
        0x44 m_timeOfDeactivation: f32,
        /// `m_entities` (Xbox PDB): the `hkInplaceArray` data pointer.
        0x48 m_entities: Ptr,
        /// Size of `m_entities`.
        0x4C m_entitiesSize: i32,
        /// Capacity word of `m_entities`.
        0x50 m_entitiesCapacity: u32,
        // +0x54: the inplace storage of `m_entities` (one pointer).
        /// `m_agentTrack.m_bytesUsedInLastSector` (Xbox PDB).
        0x58 m_bytesUsedInLastSector: u32,
        /// `m_agentTrack.m_sectors` (Xbox PDB): the `hkInplaceArray` data pointer.
        0x5C m_sectors: Ptr,
        /// Size of `m_sectors`.
        0x60 m_sectorsSize: i32,
        /// Capacity word of `m_sectors`.
        0x64 m_sectorsCapacity: u32,
        // +0x68: the inplace storage of `m_sectors` (one pointer).
    }
}

/// `hkpEntity::m_storageIndex` (Xbox PDB), the entity's index in its island's
/// entity array (u16).
const ENTITY_STORAGE_INDEX: u32 = 0xa8;
/// `hkpEntity::m_simulationIsland` (Xbox PDB).
const ENTITY_ISLAND: u32 = 0xcc;
/// `hkpAction::m_island` (Xbox PDB).
const ACTION_ISLAND: u32 = 0x0c;

/// Appends `item` to the `hkArray` (data, size, capacity word) at `array`,
/// growing it through `_reserveMore` when the size equals the capacity.
fn array_push(e: &mut Engine, array: u32, item: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), item);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d07120 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. An empty function (a single `RET`); callers pass an
/// island or a world in `ECX` and it does nothing with it.
pub fn fn_00d07120(_e: &mut Engine, _this: Ptr) {}

// Translated from 00d07130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Detaches `action` from the island: the first slot of
/// `m_actions` that holds it is set to null (the array keeps its size; a
/// missing action gives index -1, whose slot lies before the data and is
/// written all the same), `action.m_island` is cleared, and the flag byte
/// gets bits 0 and 4 set and bits 1 and 5 cleared (the first bit of the
/// `m_splitCheckRequested` and `m_actionListCleanupNeeded` fields, by the
/// module notes).
pub fn fn_00d07130(e: &mut Engine, this: Ptr<HkpSimulationIsland>, action: Ptr) {
    let size = e.get(this, HkpSimulationIsland::m_actionsSize);
    let data = e.get(this, HkpSimulationIsland::m_actions).addr();
    let mut index: i32 = -1;
    let mut candidate = 0;
    while candidate < size {
        if e.mem
            .u32(data.wrapping_add((candidate as u32).wrapping_mul(4)))
            == action.addr()
        {
            index = candidate;
            break;
        }
        candidate += 1;
    }
    let data = e.get(this, HkpSimulationIsland::m_actions).addr();
    e.mem
        .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), 0);
    e.mem.set_u32(action.addr() + ACTION_ISLAND, 0);
    let flags = e.get(this, HkpSimulationIsland::flags);
    e.set(this, HkpSimulationIsland::flags, (flags & 0xdd) | 0x11);
}

// Translated from 00d07180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Merges the constraint info (`hkpConstraintInfo` at +8)
/// of `other` into this island's: the larger `m_maxSizeOfSchema` (signed),
/// and the sums of `m_sizeOfSchemas`, `m_numSolverResults` and
/// `m_numSolverElemTemps`.
pub fn fn_00d07180(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    other: Ptr<HkpSimulationIsland>,
) {
    let other_max = e.get(other, HkpSimulationIsland::m_maxSizeOfSchema);
    let own_max = e.get(this, HkpSimulationIsland::m_maxSizeOfSchema);
    e.set(
        this,
        HkpSimulationIsland::m_maxSizeOfSchema,
        if own_max > other_max {
            own_max
        } else {
            other_max
        },
    );
    for field in [
        HkpSimulationIsland::m_sizeOfSchemas,
        HkpSimulationIsland::m_numSolverResults,
        HkpSimulationIsland::m_numSolverElemTemps,
    ] {
        let sum = e.get(this, field).wrapping_add(e.get(other, field));
        e.set(this, field, sum);
    }
}

// Translated from 00d071b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Takes `entity` out of the island's entity array by
/// swap-remove: the last entity moves into the slot at `entity.m_storageIndex`
/// and takes that index, the size drops by one, the entity's island is
/// cleared and its storage index becomes 0xffff. The flag byte gets bit 0 set
/// and bit 1 cleared.
pub fn fn_00d071b0(e: &mut Engine, this: Ptr<HkpSimulationIsland>, entity: Ptr) {
    let data = e.get(this, HkpSimulationIsland::m_entities).addr();
    let size = e.get(this, HkpSimulationIsland::m_entitiesSize);
    let last = e.mem.u32(
        data.wrapping_add((size as u32).wrapping_mul(4))
            .wrapping_sub(4),
    );
    let index = u32::from(e.mem.u16(entity.addr() + ENTITY_STORAGE_INDEX));
    e.mem.set_u32(data.wrapping_add(index * 4), last);
    let index = e.mem.u16(entity.addr() + ENTITY_STORAGE_INDEX);
    let data = e.get(this, HkpSimulationIsland::m_entities).addr();
    let moved = e.mem.u32(data.wrapping_add(u32::from(index) * 4));
    e.mem.set_u16(moved + ENTITY_STORAGE_INDEX, index);
    e.set(
        this,
        HkpSimulationIsland::m_entitiesSize,
        size.wrapping_sub(1),
    );
    e.mem.set_u32(entity.addr() + ENTITY_ISLAND, 0);
    e.mem.set_u16(entity.addr() + ENTITY_STORAGE_INDEX, 0xffff);
    let flags = e.get(this, HkpSimulationIsland::flags);
    e.set(this, HkpSimulationIsland::flags, (flags & 0xfd) | 0x01);
}

// Translated from 00d07420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::internalAddEntity` (Xbox PDB): appends `entity` to
/// the entity array (growing it when full) and records the island in
/// `entity.m_simulationIsland` and the index (low 16 bits of the old size) in
/// `entity.m_storageIndex`.
pub fn hkp_simulation_island_internal_add_entity(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    entity: Ptr,
) {
    e.mem.set_u32(entity.addr() + ENTITY_ISLAND, this.addr());
    let index = e.get(this, HkpSimulationIsland::m_entitiesSize) as u16;
    e.mem.set_u16(entity.addr() + ENTITY_STORAGE_INDEX, index);
    array_push(e, this.addr() + 0x48, entity.addr());
}

// Translated from 00d07470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::addAction` (Xbox PDB): appends `action` to the action
/// array (growing it when full) and records the island in `action.m_island`.
pub fn hkp_simulation_island_add_action(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    action: Ptr,
) {
    array_push(e, this.addr() + 0x38, action.addr());
    e.mem.set_u32(action.addr() + ACTION_ISLAND, this.addr());
}

// Translated from 00d07830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::hkpSimulationIsland` (Xbox PDB): the constructor.
/// Stores the vtable and `world`, an empty (inplace, one-element) entity
/// array and sector array, an empty action array, zeroed constraint info,
/// reference count 1, storage index 0xffff in `m_dirtyListIndex`, the thread
/// check (`m_threadId` 0xfffffff1, `m_markCount` 0x8000), zero check timers,
/// `m_timeOfDeactivation` from the global at `0104eb20`, and clears the flag
/// byte. Returns `this`.
pub fn hkp_simulation_island_hkp_simulation_island(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    world: Ptr,
) -> Ptr<HkpSimulationIsland> {
    use HkpSimulationIsland as I;
    let flags = e.get(this, I::flags);
    e.set(this, I::flags, flags & 0xcc);
    e.set(this, I::m_world, world);
    e.set(this, I::vtable, ISLAND_VTABLE);
    e.set(this, I::flags2, 5);
    e.set(this, I::m_referenceCount, 1);
    e.set(this, I::m_dirtyListIndex, 0xffff);
    e.set(this, I::m_threadId, 0xffff_fff1);
    e.set(this, I::m_markCount, 0x8000);
    e.set(this, I::m_timeSinceLastHighFrequencyCheck, 0.0);
    e.set(this, I::m_timeSinceLastLowFrequencyCheck, 0.0);
    e.set(this, I::m_actions, Ptr::NULL);
    e.set(this, I::m_actionsSize, 0);
    e.set(this, I::m_actionsCapacity, 0x8000_0000);
    let initial = x87_quiet(e.global::<f32>(TIME_OF_DEACTIVATION_INITIAL));
    e.set(this, I::m_timeOfDeactivation, initial);
    e.set(this, I::m_entitiesSize, 0);
    e.set(this, I::m_entities, Ptr::new(this.addr() + 0x54));
    e.set(this, I::m_entitiesCapacity, 0x8000_0001);
    e.set(this, I::m_sectorsSize, 0);
    e.set(this, I::m_sectorsCapacity, 0x8000_0001);
    e.set(this, I::m_sectors, Ptr::new(this.addr() + 0x68));
    e.set(this, I::m_bytesUsedInLastSector, 0x200);
    let flags = e.get(this, I::flags);
    e.set(this, I::flags, flags & 0x3f);
    e.set(this, I::m_maxSizeOfSchema, 0);
    e.set(this, I::m_sizeOfSchemas, 0);
    e.set(this, I::m_numSolverResults, 0);
    e.set(this, I::m_numSolverElemTemps, 0);
    let flags = e.get(this, I::flags);
    e.set(this, I::flags, flags & 0xf3);
    e.set(this, I::m_numConstraints, 0);
    this
}

// Translated from 00d078e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_7720`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d078e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_7720, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d078d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_87cc.
pub fn hkp_simulation_island_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_87cc
}

/// `hkpWorldOperationUtil::addConstraintToCriticalLockedIsland` (`00cb3c80`) and
/// `removeConstraintFromCriticalLockedIsland` (`00cb3da0`).
const ADD_CONSTRAINT_TO_CRITICAL_LOCKED_ISLAND: u32 = 0x00cb_3c80;
const REMOVE_CONSTRAINT_FROM_CRITICAL_LOCKED_ISLAND: u32 = 0x00cb_3da0;
/// The constructor of the local entity-pointer array (`00d27f70`, no unit; `this` is the
/// array object, then the stack-allocator state and the count) and the island function
/// `00d074c0` that takes it.
const LOCAL_ARRAY_CONSTRUCT: u32 = 0x00d2_7f70;
const ISLAND_WITH_ENTITY_ARRAY: u32 = 0x00d0_74c0;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkpAgentMachineUtil::calcNnStatisticsContactMgrsOnly` (`00d97360`) and `calcNnStatistics`
/// (`00d97130`).
const CALC_NN_STATISTICS_CONTACT_MGRS_ONLY: u32 = 0x00d9_7360;
const CALC_NN_STATISTICS: u32 = 0x00d9_7130;

// Translated from 00d070c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::addConstraintToCriticalLockedIsland` (Xbox PDB). Register value first:
/// ECX (`this`) is never read; the constraint instance is the stack word. Calls
/// `hkpWorldOperationUtil::addConstraintToCriticalLockedIsland(world, constraint)` with the
/// world at +8 of the object at `constraint + 0x14`.
pub fn hkp_simulation_island_add_constraint_to_critical_locked_island(
    e: &mut Engine,
    _this: Ptr,
    constraint: Ptr,
) {
    let owner = e.mem.u32(constraint.addr() + 0x14);
    let world = e.mem.u32(owner + 8);
    e.call(
        ADD_CONSTRAINT_TO_CRITICAL_LOCKED_ISLAND,
        &args![world, constraint],
    );
}

// Translated from 00d070e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::removeConstraintFromCriticalLockedIsland` (Xbox PDB). Register value
/// first: ECX (`this`) is never read; the constraint instance is the stack word. Calls
/// `hkpWorldOperationUtil::removeConstraintFromCriticalLockedIsland(world, constraint)` with
/// the world at +8 of the object at `constraint + 0x14`.
pub fn hkp_simulation_island_remove_constraint_from_critical_locked_island(
    e: &mut Engine,
    _this: Ptr,
    constraint: Ptr,
) {
    let owner = e.mem.u32(constraint.addr() + 0x14);
    let world = e.mem.u32(owner + 8);
    e.call(
        REMOVE_CONSTRAINT_FROM_CRITICAL_LOCKED_ISLAND,
        &args![world, constraint],
    );
}

// Translated from 00d07100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::addCallbackRequest` (Xbox PDB). Register value first: ECX (`this`) is
/// never read; the first stack word is the constraint instance, whose internal record
/// (pointer at +0x30) gets `request` OR-ed into its byte at +0x12.
pub fn hkp_simulation_island_add_callback_request(
    e: &mut Engine,
    _this: Ptr,
    constraint: Ptr,
    request: u8,
) {
    let internal = e.mem.u32(constraint.addr() + 0x30);
    let old = e.mem.u8(internal + 0x12);
    e.mem.set_u8(internal + 0x12, old | request);
}

// Translated from 00d07210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::calcContentStatistics` (Xbox PDB), `(collector, _unused_2)`. Adds the
/// chunk `"EntityPtrs"` (when the entity array's capacity word is not negative or the
/// collector's byte at +0x10 is set), then for each entity opens `"Entity"` and `"Constraints"`,
/// adds `"ConMstPtr"` (entity + 0xac, 0x30 bytes per `u16` count at +0xb0, capacity from the
/// `u16` at +0xb2; skipped when its bit 15 is set and the collector's byte is clear) and
/// reports each constraint's instance (`"ConInstance"`) and data (`"ConData"`) when their
/// `u16` at +4 has bits 0 to 14 set, then closes; then `"ActionPtrs"` and each
/// action (`"Actions"`), the agent track's statistics (`00d97360`), `"CollAgents"` with
/// `00d97130(track, world[0x78], collector)`. The collector's slots: `0xc` object, `0x14`
/// chunk, `0x18` begin, `0x1c` end.
pub fn hkp_simulation_island_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    collector: Ptr,
    _unused_2: u32,
) {
    let island = this.addr();
    let target = collector.addr();
    let capacity = e.mem.u32(island + 0x50);
    if (capacity as i32) >= 0 || e.mem.u8(target + 0x10) != 0 {
        let pointers = e.mem.u32(island + 0x48);
        let size = e
            .call(
                GET_ALLOCATED_SIZE,
                &args![pointers, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            )
            .u32();
        let pointers = e.mem.u32(island + 0x48);
        let count = e.mem.u32(island + 0x4c).wrapping_mul(4);
        e.vcall(
            target,
            0x14,
            &args![3u32, 0x010c_cf18u32, pointers, count, size],
        );
    }
    let mut index: i32 = 0;
    if e.mem.i32(island + 0x4c) > 0 {
        loop {
            let entities = e.mem.u32(island + 0x48);
            let entity = e
                .mem
                .u32(entities.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(target, 0xc, &args![0x010c_cf10u32, entity, 0u32]);
            e.vcall(target, 0x18, &args![0x010c_b200u32]);
            let flags = u32::from(e.mem.u16(entity + 0xb2));
            if flags & 0x8000 == 0 || e.mem.u8(target + 0x10) != 0 {
                let master_count = u32::from(e.mem.u16(entity + 0xb0)).wrapping_mul(0x30);
                let masters = e.mem.u32(entity + 0xac);
                let master_capacity = (flags & 0x3fff).wrapping_mul(0x30);
                e.vcall(
                    target,
                    0x14,
                    &args![3u32, 0x010c_cf04u32, masters, master_count, master_capacity],
                );
            }
            if e.mem.u16(entity + 0xb0) != 0 {
                let mut constraint_index: i32 = 0;
                let mut offset: u32 = 0;
                loop {
                    let masters = e.mem.u32(entity + 0xac);
                    let instance = e.mem.u32(masters.wrapping_add(offset));
                    if e.mem.u16(instance + 4) & 0x7fff != 0 {
                        e.vcall(target, 0xc, &args![0x010c_cef8u32, instance, 0u32]);
                    }
                    let masters = e.mem.u32(entity + 0xac);
                    let instance = e.mem.u32(masters.wrapping_add(offset));
                    let data = e.mem.u32(instance + 0xc);
                    if e.mem.u16(data + 4) & 0x7fff != 0 {
                        e.vcall(target, 0xc, &args![0x010c_cef0u32, data, 0u32]);
                    }
                    let total = i32::from(e.mem.u16(entity + 0xb0));
                    constraint_index += 1;
                    offset = offset.wrapping_add(0x30);
                    if constraint_index >= total {
                        break;
                    }
                }
            }
            e.vcall(target, 0x1c, &args![]);
            index += 1;
            if index >= e.mem.i32(island + 0x4c) {
                break;
            }
        }
    }
    let capacity = e.mem.u32(island + 0x40);
    if (capacity as i32) >= 0 || e.mem.u8(target + 0x10) != 0 {
        let actions = e.mem.u32(island + 0x38);
        let size = e
            .call(
                GET_ALLOCATED_SIZE,
                &args![actions, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            )
            .u32();
        let actions = e.mem.u32(island + 0x38);
        let count = e.mem.u32(island + 0x3c).wrapping_mul(4);
        e.vcall(
            target,
            0x14,
            &args![3u32, 0x010c_cee4u32, actions, count, size],
        );
    }
    let mut index: i32 = 0;
    if e.mem.i32(island + 0x3c) > 0 {
        loop {
            let actions = e.mem.u32(island + 0x38);
            let action = e
                .mem
                .u32(actions.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(target, 0xc, &args![0x010c_41dcu32, action, 0u32]);
            index += 1;
            if index >= e.mem.i32(island + 0x3c) {
                break;
            }
        }
    }
    let track = island + 0x58;
    e.call(
        CALC_NN_STATISTICS_CONTACT_MGRS_ONLY,
        &args![track, collector],
    );
    e.vcall(target, 0x18, &args![0x010c_ced8u32]);
    let world = e.mem.u32(island + 0x18);
    let dispatcher = e.mem.u32(world + 0x78);
    e.call(CALC_NN_STATISTICS, &args![track, dispatcher, collector]);
    e.vcall(target, 0x1c, &args![]);
}

// Translated from 00d07720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the island (`hkpSimulationIsland::~hkpSimulationIsland` by its vtable,
/// `0x010c_cf28`; the map has no name): sets that vtable, then frees the three `hkArray`s
/// of 4-byte elements (sectors at +0x5c, entities at +0x48, actions at +0x38) in that order
/// through the allocator (slot `0x10`) when the capacity word is not negative, with size
/// `(capacity & 0x3fffffff) * 4`, clears each array's size and data and sets its capacity
/// word to `0x80000000`, and ends with the vtable `0x0102_e388`.
pub fn fn_00d07720(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, ISLAND_VTABLE);
    for (data, size, capacity) in [(0x5c, 0x60, 0x64), (0x48, 0x4c, 0x50), (0x38, 0x3c, 0x40)] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity_word = e.mem.u32(base + capacity);
        e.mem.set_u32(base + size, 0);
        if (capacity_word as i32) >= 0 {
            let block = e.mem.u32(base + data);
            e.vcall(
                allocator,
                0x10,
                &args![block, (capacity_word & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        e.mem.set_u32(base + data, 0);
        e.mem.set_u32(base + capacity, 0x8000_0000);
    }
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00d077d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An island function (no map name) that builds a temporary array of `-1` words, one per
/// entity (`m_entitiesSize`), on the memory router's stack allocator and passes it to
/// `00d074c0`. The router's first word (the allocator's top) is advanced by `count * 4 + 0x7f`
/// rounded down to a multiple of `0x80`; `00d27f70` constructs the array object (8 bytes:
/// the state pointer and the count; it writes `0xffffffff` through the state's word);
/// the allocator's top is restored afterwards. Returns the answer of `00d074c0` (`AL`).
pub fn fn_00d077d0(e: &mut Engine, this: Ptr<HkpSimulationIsland>) -> bool {
    let island = this.addr();
    let count = e.mem.u32(island + 0x4c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let advanced = (count.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top);
    e.mem.set_u32(router, advanced);
    let count = e.mem.u32(island + 0x4c);
    let answer = e.with_stack(0x10, |e, frame| {
        let state = frame.addr();
        let array = frame.addr() + 8;
        e.mem.set_u32(state, top);
        e.call(LOCAL_ARRAY_CONSTRUCT, &args![array, state, count]);
        e.call(ISLAND_WITH_ENTITY_ARRAY, &args![this, array]).u8()
    });
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
    answer != 0
}

// Translated from 00d074c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the unit is `hkpSimulationIsland.cpp`; the class is unconfirmed): `this`
/// in `ECX` and one stack word, `context` (`RET 4`). The island holds an array of entity pointers
/// (data at +0x48, signed count at +0x4c) and an array of object pointers (data at +0x38, signed
/// count at +0x3c). `context` is an object for which `00d28030` (`this` = `context`, two words) is
/// called with a pair of numbers; the walk stops with `true` as soon as the word reached through three
/// pointers from `context` (`[[[context]]]`) equals the negated word at `context + 4`. Three passes:
///
/// 1. For each entity `i` and each 8-byte record `j` of its array at `entity + 0x60` (data, signed
///    count): the record's word at +4, moved by the signed byte at its +0x10, gives an object; unless
///    the byte at +0xe8 of it is 5, `00d28030(context, i, word at +0xa8)` is called.
/// 2. For each entity and each of the `n` (16-bit count at `entity + 0xb0`) 0x30-byte entries of the
///    table at `entity + 0xac`: with `a` the word at +4 and `b` the word at +8 of the entry, unless
///    the byte at +0xe8 of either is 5, `00d28030(context, a's word at +0xa8, b's word at +0xa8)`.
/// 3. For each non-null object of the second array: a local array (10 words of storage on the stack,
///    size 0, not owned) is filled by the virtual function at +0x10 of the object; the first entry
///    whose byte at +0xe8 is not 5 is paired with every later such entry:
///    `00d28030(context, first's word at +0xa8, the later one's word at +0xa8)`.
///
/// The local array's storage is freed before either answer when it is owned (capacity word not
/// negative: the memory router's allocator, virtual function at +0x10, with the data pointer and
/// `(capacity & 0x3fffffff) * 4` bytes). Answers `true` as soon as the stop condition holds after a
/// call, otherwise whether it holds after all passes. (The upper bytes of `EAX` differ between the
/// exe's paths; only `AL` is the answer.)
pub fn fn_00d074c0(e: &mut Engine, this: u32, context: u32) -> bool {
    let entity_count = e.mem.i32(this + 0x4c);
    // Pass 1.
    let mut i = 0i32;
    if entity_count > 0 {
        loop {
            let entities = e.mem.u32(this + 0x48);
            let entity = e.mem.u32(entities.wrapping_add(4 * i as u32));
            let list = entity.wrapping_add(0x60);
            let mut j = 0i32;
            if e.mem.i32(list + 4) > j {
                loop {
                    let records = e.mem.u32(list);
                    let record = e.mem.u32(records.wrapping_add(8 * j as u32 + 4));
                    let object = record.wrapping_add(e.mem.i8(record + 0x10) as u32);
                    if e.mem.u8(object + 0xe8) != 5 {
                        let id = u32::from(e.mem.u16(object + 0xa8));
                        e.call(0x00d2_8030, &args![context, i as u32, id]);
                        if stop_reached_00d074c0(e, context) {
                            return true;
                        }
                    }
                    j += 1;
                    if j >= e.mem.i32(list + 4) {
                        break;
                    }
                }
            }
            i += 1;
            if i >= e.mem.i32(this + 0x4c) {
                break;
            }
        }
    }
    // Pass 2.
    let mut index = 0i32;
    if e.mem.i32(this + 0x4c) > 0 {
        loop {
            let entities = e.mem.u32(this + 0x48);
            let entity = e.mem.u32(entities.wrapping_add(4 * index as u32));
            if e.mem.u16(entity + 0xb0) != 0 {
                let mut n = 0i32;
                let mut offset = 0u32;
                loop {
                    let table = e.mem.u32(entity + 0xac);
                    let first = e.mem.u32(table.wrapping_add(offset + 4));
                    if e.mem.u8(first + 0xe8) != 5 {
                        let second = e.mem.u32(table.wrapping_add(offset + 8));
                        if e.mem.u8(second + 0xe8) != 5 {
                            let first_id = u32::from(e.mem.u16(first + 0xa8));
                            let second_id = u32::from(e.mem.u16(second + 0xa8));
                            e.call(0x00d2_8030, &args![context, first_id, second_id]);
                            if stop_reached_00d074c0(e, context) {
                                return true;
                            }
                        }
                    }
                    n += 1;
                    offset = offset.wrapping_add(0x30);
                    if n >= i32::from(e.mem.u16(entity + 0xb0)) {
                        break;
                    }
                }
            }
            index += 1;
            if index >= e.mem.i32(this + 0x4c) {
                break;
            }
        }
    }
    // Pass 3, with the exe's local array: header (data, size, capacity word) and 10 words of storage.
    e.with_stack(0x34, |e, frame| {
        let header = frame.addr();
        e.mem.set_u32(header, header + 0xc);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_000a);
        let mut slot_index = 0i32;
        if e.mem.i32(this + 0x3c) > 0 {
            loop {
                let slots = e.mem.u32(this + 0x38);
                let slot = slots.wrapping_add(4 * slot_index as u32);
                if e.mem.u32(slot) != 0 {
                    e.mem.set_u32(header + 4, 0);
                    let object = e.mem.u32(slot);
                    e.vcall(object, 0x10, &args![header]);
                    let mut size = e.mem.i32(header + 4);
                    let mut data = e.mem.u32(header);
                    let mut scan = 0i32;
                    let mut first = -1i32;
                    loop {
                        if scan >= size {
                            break;
                        }
                        let candidate = e.mem.u32(data.wrapping_add(4 * scan as u32));
                        if e.mem.u8(candidate + 0xe8) != 5 {
                            first = scan;
                        }
                        scan += 1;
                        if first == -1 {
                            continue;
                        }
                        if scan >= size {
                            break;
                        }
                        loop {
                            let current = e.mem.u32(data.wrapping_add(4 * scan as u32));
                            if e.mem.u8(current + 0xe8) != 5 {
                                let current_id = u32::from(e.mem.u16(current + 0xa8));
                                let first_object = e.mem.u32(data.wrapping_add(4 * first as u32));
                                let first_id = u32::from(e.mem.u16(first_object + 0xa8));
                                e.call(0x00d2_8030, &args![context, first_id, current_id]);
                                if stop_reached_00d074c0(e, context) {
                                    release_frame_array_00d074c0(e, header);
                                    return true;
                                }
                                size = e.mem.i32(header + 4);
                                data = e.mem.u32(header);
                            }
                            scan += 1;
                            if scan >= size {
                                break;
                            }
                        }
                        break;
                    }
                }
                slot_index += 1;
                if slot_index >= e.mem.i32(this + 0x3c) {
                    break;
                }
            }
        }
        release_frame_array_00d074c0(e, header);
        stop_reached_00d074c0(e, context)
    })
}

/// Whether the word reached through three pointers from `context` equals the negated word at
/// `context + 4`.
fn stop_reached_00d074c0(e: &Engine, context: u32) -> bool {
    let first = e.mem.u32(context);
    let second = e.mem.u32(first);
    e.mem.u32(second) == e.mem.u32(context + 4).wrapping_neg()
}

/// Frees the local array's storage when it is owned, after setting its size to 0.
fn release_frame_array_00d074c0(e: &mut Engine, header: u32) {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(header + 8);
    e.mem.set_u32(header + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(header);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d078d0, hkp_simulation_island_get_class_type() -> u32),
        entry!(0x00d0_7120, fn_00d07120(Ptr)),
        entry!(0x00d0_7130, fn_00d07130(Ptr<HkpSimulationIsland>, Ptr)),
        entry!(
            0x00d0_7180,
            fn_00d07180(Ptr<HkpSimulationIsland>, Ptr<HkpSimulationIsland>)
        ),
        entry!(0x00d0_71b0, fn_00d071b0(Ptr<HkpSimulationIsland>, Ptr)),
        entry!(
            0x00d0_7420,
            hkp_simulation_island_internal_add_entity(Ptr<HkpSimulationIsland>, Ptr)
        ),
        entry!(
            0x00d0_7470,
            hkp_simulation_island_add_action(Ptr<HkpSimulationIsland>, Ptr)
        ),
        entry!(
            0x00d0_7830,
            hkp_simulation_island_hkp_simulation_island(
                Ptr<HkpSimulationIsland>,
                Ptr,
            ) -> Ptr<HkpSimulationIsland>
        ),
        entry!(
            0x00d0_70c0,
            hkp_simulation_island_add_constraint_to_critical_locked_island(Ptr, Ptr)
        ),
        entry!(
            0x00d0_70e0,
            hkp_simulation_island_remove_constraint_from_critical_locked_island(Ptr, Ptr)
        ),
        entry!(
            0x00d0_7100,
            hkp_simulation_island_add_callback_request(Ptr, Ptr, u8)
        ),
        entry!(
            0x00d0_7210,
            hkp_simulation_island_calc_content_statistics(Ptr<HkpSimulationIsland>, Ptr, u32)
        ),
        entry!(0x00d0_7720, fn_00d07720(Ptr)),
        entry!(0x00d0_77d0, fn_00d077d0(Ptr<HkpSimulationIsland>) -> bool),
        entry!(0x00d0_78e0, deleting_destructor_00d078e0(Ptr, u8) -> Ptr),
        entry!(0x00d0_74c0, fn_00d074c0(u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d078e0_frees_only_when_bit_zero_is_set() {
        check(0x00d0_78e0, 0x00d0_7720);
    }

    // ---- the island's own functions

    fn island(e: &mut Engine) -> Ptr<HkpSimulationIsland> {
        e.new_object::<HkpSimulationIsland>()
    }

    /// An `hkArray` header at `array` over a fresh block of `capacity` words
    /// holding `items`; returns the data pointer.
    fn make_array(e: &mut Engine, array: u32, capacity: u32, items: &[u32]) -> u32 {
        // One spare word in front so that index -1 stays inside the block.
        let words = (capacity & 0x3fff_ffff).max(items.len() as u32);
        let block = e.mem.alloc(4 * words + 4) + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *item);
        }
        e.mem.set_u32(array, block);
        e.mem.set_u32(array + 4, items.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        block
    }

    #[test]
    fn empty_function_does_nothing() {
        let mut e = engine();
        e.call(0x00d0_7120, &args![0x1234u32]);
    }

    #[test]
    fn remove_action_nulls_its_slot_and_flags_the_island() {
        let mut e = engine();
        let this = island(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second + 0xc, this.addr());
        let block = make_array(&mut e, this.addr() + 0x38, 4, &[first, second]);
        e.mem.set_u8(this.addr() + 0x25, 0xee);
        e.call(0x00d0_7130, &args![this, second]);
        assert_eq!(e.mem.u32(block), first);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.get(this, HkpSimulationIsland::m_actionsSize), 2);
        assert_eq!(e.mem.u32(second + 0xc), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x25), (0xee & 0xdd) | 0x11);
    }

    #[test]
    fn remove_action_that_is_not_there_writes_before_the_data() {
        let mut e = engine();
        let this = island(&mut e);
        let first = e.mem.alloc(0x20);
        let stranger = e.mem.alloc(0x20);
        let block = make_array(&mut e, this.addr() + 0x38, 4, &[first]);
        e.mem.set_u32(block - 4, 0x5555);
        e.call(0x00d0_7130, &args![this, stranger]);
        assert_eq!(e.mem.u32(block - 4), 0);
        assert_eq!(e.mem.u32(block), first);
        // An empty array gives index -1 as well.
        e.mem.set_u32(this.addr() + 0x3c, 0);
        e.mem.set_u32(block - 4, 0x5555);
        e.call(0x00d0_7130, &args![this, stranger]);
        assert_eq!(e.mem.u32(block - 4), 0);
    }

    #[test]
    fn merge_constraint_info_takes_the_larger_schema_and_adds_the_rest() {
        let mut e = engine();
        let this = island(&mut e);
        let other = island(&mut e);
        for (target, values) in [(this, [10, 1, 2, 3]), (other, [4, 10, 20, 30])] {
            for (i, value) in values.iter().enumerate() {
                e.mem.set_i32(target.addr() + 8 + 4 * i as u32, *value);
            }
        }
        e.call(0x00d0_7180, &args![this, other]);
        let merged: Vec<i32> = (0..4).map(|i| e.mem.i32(this.addr() + 8 + 4 * i)).collect();
        assert_eq!(merged, vec![10, 11, 22, 33]);
        // The maximum is signed: a larger other wins.
        e.mem.set_i32(this.addr() + 8, -5);
        e.mem.set_i32(other.addr() + 8, 7);
        e.call(0x00d0_7180, &args![this, other]);
        assert_eq!(e.mem.i32(this.addr() + 8), 7);
    }

    #[test]
    fn remove_entity_moves_the_last_one_into_its_slot() {
        let mut e = engine();
        let this = island(&mut e);
        let entities: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xe0)).collect();
        for (i, entity) in entities.iter().enumerate() {
            e.mem.set_u16(entity + 0xa8, i as u16);
            e.mem.set_u32(entity + 0xcc, this.addr());
        }
        let block = make_array(&mut e, this.addr() + 0x48, 4, &entities);
        e.mem.set_u8(this.addr() + 0x25, 0xfe);
        e.call(0x00d0_71b0, &args![this, entities[0]]);
        assert_eq!(e.mem.u32(block), entities[2]);
        assert_eq!(e.mem.u16(entities[2] + 0xa8), 0);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 2);
        assert_eq!(e.mem.u32(entities[0] + 0xcc), 0);
        assert_eq!(e.mem.u16(entities[0] + 0xa8), 0xffff);
        assert_eq!(e.mem.u8(this.addr() + 0x25), (0xfe & 0xfd) | 1);
        // Removing the last entity leaves it with the marker only.
        e.call(0x00d0_71b0, &args![this, entities[2]]);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 1);
        assert_eq!(e.mem.u16(entities[2] + 0xa8), 0xffff);
    }

    type GrowthLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Doubles for the memory router and `_reserveMore`: the latter replaces
    /// the array's data with a block of twice the capacity (at least 4) and
    /// logs its arguments.
    fn install_growth(e: &mut Engine) -> GrowthLog {
        let log: GrowthLog = Rc::new(RefCell::new(Vec::new()));
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let grow_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push(a.to_vec());
            let array = a[1];
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff).max(2) * 2;
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let block = e.mem.alloc(4 * capacity);
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(block + 4 * i, word);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        log
    }

    #[test]
    fn add_entity_appends_and_records_island_and_index() {
        let mut e = engine();
        let log = install_growth(&mut e);
        let this = island(&mut e);
        let existing = e.mem.alloc(0xe0);
        let block = make_array(&mut e, this.addr() + 0x48, 4, &[existing]);
        let entity = e.mem.alloc(0xe0);
        e.call(0x00d0_7420, &args![this, entity]);
        assert_eq!(e.mem.u32(block + 4), entity);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 2);
        assert_eq!(e.mem.u32(entity + 0xcc), this.addr());
        assert_eq!(e.mem.u16(entity + 0xa8), 1);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn add_entity_grows_a_full_array_first() {
        let mut e = engine();
        let log = install_growth(&mut e);
        let this = island(&mut e);
        let existing = e.mem.alloc(0xe0);
        // Capacity 1 with the not-owned flag in bit 31 (as the inplace array has).
        make_array(&mut e, this.addr() + 0x48, 0x8000_0001, &[existing]);
        let entity = e.mem.alloc(0xe0);
        e.call(0x00d0_7420, &args![this, entity]);
        let calls = log.borrow().clone();
        assert_eq!(calls.len(), 1);
        // The array and the element size follow the allocator.
        assert_eq!(calls[0][1], this.addr() + 0x48);
        assert_eq!(calls[0][2], 4);
        let data = e.get(this, HkpSimulationIsland::m_entities).addr();
        assert_eq!(e.mem.u32(data), existing);
        assert_eq!(e.mem.u32(data + 4), entity);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 2);
    }

    #[test]
    fn add_action_appends_and_records_the_island() {
        let mut e = engine();
        let log = install_growth(&mut e);
        let this = island(&mut e);
        let block = make_array(&mut e, this.addr() + 0x38, 2, &[]);
        let action = e.mem.alloc(0x20);
        e.call(0x00d0_7470, &args![this, action]);
        assert_eq!(e.mem.u32(block), action);
        assert_eq!(e.get(this, HkpSimulationIsland::m_actionsSize), 1);
        assert_eq!(e.mem.u32(action + 0xc), this.addr());
        assert!(log.borrow().is_empty());
        // A full array grows through the helper.
        let second = e.mem.alloc(0x20);
        let third = e.mem.alloc(0x20);
        e.call(0x00d0_7470, &args![this, second]);
        e.call(0x00d0_7470, &args![this, third]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0][1], this.addr() + 0x38);
        let data = e.get(this, HkpSimulationIsland::m_actions).addr();
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [action, second, third]
        );
    }

    #[test]
    fn constructor_sets_the_empty_island_up() {
        let mut e = engine();
        e.set_global(TIME_OF_DEACTIVATION_INITIAL, -10.0f32);
        let this = island(&mut e);
        // Fill with a pattern to see what the constructor leaves alone.
        for offset in (0..0x6c).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xcdcd_cdcd);
        }
        let world = e.mem.alloc(0x10);
        let back = e.call(0x00d0_7830, &args![this, world]).u32();
        assert_eq!(back, this.addr());
        let word = |e: &Engine, offset: u32| e.mem.u32(this.addr() + offset);
        assert_eq!(word(&e, 0), ISLAND_VTABLE);
        assert_eq!(e.mem.u16(this.addr() + 6), 1);
        assert_eq!(word(&e, 0x18), world);
        assert_eq!(word(&e, 0x1c), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x22), 0xffff);
        assert_eq!(e.mem.u8(this.addr() + 0x25), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x26), 5);
        assert_eq!(word(&e, 0x28), 0xffff_fff1);
        assert_eq!(e.mem.u16(this.addr() + 0x2c), 0x8000);
        assert_eq!(word(&e, 0x30), 0);
        assert_eq!(word(&e, 0x34), 0);
        assert_eq!(
            [word(&e, 0x38), word(&e, 0x3c), word(&e, 0x40)],
            [0, 0, 0x8000_0000]
        );
        assert_eq!(e.mem.f32(this.addr() + 0x44), -10.0);
        assert_eq!(
            [word(&e, 0x48), word(&e, 0x4c), word(&e, 0x50)],
            [this.addr() + 0x54, 0, 0x8000_0001]
        );
        assert_eq!(
            [
                word(&e, 0x58),
                word(&e, 0x5c),
                word(&e, 0x60),
                word(&e, 0x64)
            ],
            [0x200, this.addr() + 0x68, 0, 0x8000_0001]
        );
        for offset in [0x8, 0xc, 0x10, 0x14] {
            assert_eq!(word(&e, offset), 0);
        }
        // Not touched: the flags at +4, the storage index, the split counter
        // and the inplace storage.
        assert_eq!(e.mem.u16(this.addr() + 4), 0xcdcd);
        assert_eq!(e.mem.u16(this.addr() + 0x20), 0xcdcd);
        assert_eq!(e.mem.u8(this.addr() + 0x24), 0xcd);
        assert_eq!(word(&e, 0x54), 0xcdcd_cdcd);
        assert_eq!(word(&e, 0x68), 0xcdcd_cdcd);
    }

    #[test]
    fn hkp_simulation_island_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d0_78d0, &args![]).u32(), 0x0126_87cc);
    }

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn new_log() -> EventLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double at `address` that logs `[address, arguments...]`.
    fn log_calls(e: &mut Engine, address: u32, log: &EventLog) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            let mut entry = vec![address];
            entry.extend_from_slice(a);
            log.borrow_mut().push(entry);
            Ret::default()
        });
    }

    #[test]
    fn critical_locked_island_calls_go_to_the_constraints_world() {
        let mut e = Engine::new();
        let log = new_log();
        log_calls(&mut e, ADD_CONSTRAINT_TO_CRITICAL_LOCKED_ISLAND, &log);
        log_calls(&mut e, REMOVE_CONSTRAINT_FROM_CRITICAL_LOCKED_ISLAND, &log);
        let constraint = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(constraint + 0x14, owner);
        e.mem.set_u32(owner + 8, 0x7700);
        e.call(0x00d0_70c0, &args![0x1234u32, constraint]);
        e.call(0x00d0_70e0, &args![0x1234u32, constraint]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![ADD_CONSTRAINT_TO_CRITICAL_LOCKED_ISLAND, 0x7700, constraint],
                vec![
                    REMOVE_CONSTRAINT_FROM_CRITICAL_LOCKED_ISLAND,
                    0x7700,
                    constraint
                ],
            ]
        );
    }

    #[test]
    fn callback_requests_are_or_ed_into_the_internal_record() {
        let mut e = Engine::new();
        let constraint = e.mem.alloc(0x40);
        let internal = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0x30, internal);
        e.mem.set_u8(internal + 0x12, 0x05);
        e.call(0x00d0_7100, &args![0x1234u32, constraint, 0x1au32]);
        assert_eq!(e.mem.u8(internal + 0x12), 0x1f);
        // Only the low byte of the stack word is the request.
        e.call(0x00d0_7100, &args![0x1234u32, constraint, 0xff00_0020u32]);
        assert_eq!(e.mem.u8(internal + 0x12), 0x3f);
    }

    const COLLECTOR_VTABLE: u32 = 0x0900_3000;

    /// A collector whose slots `0xc`, `0x14`, `0x18` and `0x1c` log `[slot, arguments...]`.
    fn collector(e: &mut Engine, log: &EventLog) -> u32 {
        let mut slots = vec![0u32; 8];
        for slot in [0xc, 0x14, 0x18, 0x1c] {
            slots[slot / 4] = 0x0900_0000 + slot as u32;
            let slot_log = Rc::clone(log);
            e.register_double(0x0900_0000 + slot as u32, move |_, a| {
                let mut entry = vec![slot as u32];
                entry.extend_from_slice(a);
                slot_log.borrow_mut().push(entry);
                Ret::default()
            });
        }
        e.put_vtable(COLLECTOR_VTABLE, &slots);
        let collector = e.mem.alloc(0x40);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        collector
    }

    #[test]
    fn the_island_statistics_walk_entities_constraints_and_actions() {
        let mut e = Engine::new();
        let log = new_log();
        let collector = collector(&mut e, &log);
        log_calls(&mut e, CALC_NN_STATISTICS_CONTACT_MGRS_ONLY, &log);
        log_calls(&mut e, CALC_NN_STATISTICS, &log);
        e.register_double(GET_ALLOCATED_SIZE, |_, a| Ret {
            eax: 0x1000 + a[1],
            ..Ret::default()
        });
        let island = e.mem.alloc(0x80);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, 0x7800);
        e.mem.set_u32(island + 0x18, world);
        // Two entities; the first has two constraints, the second none.
        let entities = e.mem.alloc(0x10);
        let entity_0 = e.mem.alloc(0x100);
        let entity_1 = e.mem.alloc(0x100);
        e.mem.set_u32(entities, entity_0);
        e.mem.set_u32(entities + 4, entity_1);
        e.mem.set_u32(island + 0x48, entities);
        e.mem.set_u32(island + 0x4c, 2);
        e.mem.set_u32(island + 0x50, 0x4000_0002);
        let masters = e.mem.alloc(0x60);
        e.mem.set_u32(entity_0 + 0xac, masters);
        e.mem.set_u16(entity_0 + 0xb0, 2);
        e.mem.set_u16(entity_0 + 0xb2, 3);
        e.mem.set_u16(entity_1 + 0xb2, 0x8001);
        let instances = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        let data = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        for index in 0..2 {
            e.mem
                .set_u32(masters + 0x30 * index, instances[index as usize]);
            e.mem
                .set_u32(instances[index as usize] + 0xc, data[index as usize]);
        }
        e.mem.set_u16(instances[0] + 4, 0x0001);
        e.mem.set_u16(instances[1] + 4, 0x8000);
        e.mem.set_u16(data[1] + 4, 0x0002);
        // One action; the action array has a negative capacity word.
        let actions = e.mem.alloc(0x10);
        e.mem.set_u32(actions, 0x7900);
        e.mem.set_u32(island + 0x38, actions);
        e.mem.set_u32(island + 0x3c, 1);
        e.mem.set_u32(island + 0x40, 0x8000_0001);
        e.call(0x00d0_7210, &args![island, collector, 0u32]);
        let track = island + 0x58;
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x14, collector, 3, 0x010c_cf18, entities, 8, 0x1008],
                vec![0xc, collector, 0x010c_cf10, entity_0, 0],
                vec![0x18, collector, 0x010c_b200],
                vec![0x14, collector, 3, 0x010c_cf04, masters, 0x60, 0x90],
                vec![0xc, collector, 0x010c_cef8, instances[0], 0],
                vec![0xc, collector, 0x010c_cef0, data[1], 0],
                vec![0x1c, collector],
                vec![0xc, collector, 0x010c_cf10, entity_1, 0],
                vec![0x18, collector, 0x010c_b200],
                vec![0x1c, collector],
                vec![0xc, collector, 0x010c_41dc, 0x7900, 0],
                vec![CALC_NN_STATISTICS_CONTACT_MGRS_ONLY, track, collector],
                vec![0x18, collector, 0x010c_ced8],
                vec![CALC_NN_STATISTICS, track, 0x7800, collector],
                vec![0x1c, collector],
            ]
        );
        // A set collector flag adds the skipped chunks.
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d0_7210, &args![island, collector, 0u32]);
        let log = log.borrow();
        assert!(log.contains(&vec![0x14, collector, 3, 0x010c_cee4, actions, 4, 0x1004]));
        assert!(log.contains(&vec![0x14, collector, 3, 0x010c_cf04, 0, 0, 0x30]));
    }

    const ISLAND_FREE_DOUBLE: u32 = 0x0900_0041;

    #[test]
    fn the_island_destructor_frees_its_three_arrays_in_order() {
        let mut e = Engine::new();
        let log = new_log();
        let mut slots = vec![0u32; 8];
        slots[4] = ISLAND_FREE_DOUBLE;
        e.put_vtable(0x0900_0300, &slots);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, 0x0900_0300);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(ISLAND_FREE_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        for (data, size, capacity, data_value, capacity_value) in [
            (0x5c, 0x60, 0x64, 0x5000, 0x4000_0003),
            (0x48, 0x4c, 0x50, 0x6000, 0x8000_0001),
            (0x38, 0x3c, 0x40, 0x7000, 5),
        ] {
            e.mem.set_u32(this + data, data_value);
            e.mem.set_u32(this + size, 9);
            e.mem.set_u32(this + capacity, capacity_value);
        }
        e.call(0x00d0_7720, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![0x5000, 12], vec![0x7000, 20]]);
        for (data, size, capacity) in [(0x5c, 0x60, 0x64), (0x48, 0x4c, 0x50), (0x38, 0x3c, 0x40)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }

    #[test]
    fn a_temporary_entity_array_is_made_on_the_stack_allocator() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, 0x0a00_0000);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log = new_log();
        let construct_log = Rc::clone(&log);
        e.register_double(LOCAL_ARRAY_CONSTRUCT, move |e, a| {
            construct_log
                .borrow_mut()
                .push(vec![a[0], e.mem.u32(a[1]), a[2], e.mem.u32(router)]);
            Ret::default()
        });
        let island_log = Rc::clone(&log);
        e.register_double(ISLAND_WITH_ENTITY_ARRAY, move |e, a| {
            island_log
                .borrow_mut()
                .push(vec![a[0], a[1], e.mem.u32(router)]);
            Ret {
                eax: 0x1234_5601,
                ..Ret::default()
            }
        });
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + 0x4c, 3);
        let answer = e.call(0x00d0_77d0, &args![island]).u32();
        assert_eq!(answer, 1);
        let log = log.borrow();
        // 3 * 4 + 0x7f rounded down to 0x80.
        assert_eq!(log[0][1..], [0x0a00_0000, 3, 0x0a00_0080]);
        assert_eq!(log[1][0], island);
        assert_eq!(log[1][1], log[0][0]);
        assert_eq!(log[1][2], 0x0a00_0080);
        assert_eq!(e.mem.u32(router), 0x0a00_0000);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type PairLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A stop context: `[[[context]]]` is 1 against `-[context + 4]` = 0 (not reached) until the
    /// double for `00d28030` has run `trigger` times (0: never), when it sets the word to 0. The
    /// double logs the words it receives.
    fn stop_context(e: &mut Engine, trigger: u32) -> (u32, PairLog) {
        let context = e.mem.alloc(8);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(context, first);
        e.mem.set_u32(first, second);
        e.mem.set_u32(second, 1);
        let log: PairLog = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(0x00d2_8030, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            if sink.borrow().len() as u32 == trigger {
                e.mem.set_u32(second, 0);
            }
            Ret::default()
        });
        (context, log)
    }

    /// An object whose byte at +0xe8 is `kind` and whose word at +0xa8 is `id`.
    fn object(e: &mut Engine, kind: u8, id: u16) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u8(object + 0xe8, kind);
        e.mem.set_u16(object + 0xa8, id);
        object
    }

    /// An island with `entities` (pointers) and `slots` (object pointers).
    fn island(e: &mut Engine, entities: &[u32], slots: &[u32]) -> u32 {
        let this = e.mem.alloc(0x60);
        let entity_data = e.mem.alloc(4 * entities.len().max(1) as u32);
        for (i, entity) in entities.iter().enumerate() {
            e.mem.set_u32(entity_data + 4 * i as u32, *entity);
        }
        e.mem.set_u32(this + 0x48, entity_data);
        e.mem.set_u32(this + 0x4c, entities.len() as u32);
        let slot_data = e.mem.alloc(4 * slots.len().max(1) as u32);
        for (i, slot) in slots.iter().enumerate() {
            e.mem.set_u32(slot_data + 4 * i as u32, *slot);
        }
        e.mem.set_u32(this + 0x38, slot_data);
        e.mem.set_u32(this + 0x3c, slots.len() as u32);
        // The memory router the local array's cleanup asks for (nothing is freed unless a test says so).
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        this
    }

    /// An entity whose list at +0x60 holds one record per `(kind, id, adjust)`: the record's byte at
    /// +0x10 is `adjust`, so the object it leads to lies `adjust` bytes from the record.
    fn entity_with_records(e: &mut Engine, objects: &[(u8, u16, i8)]) -> u32 {
        let entity = e.mem.alloc(0x100);
        let records = e.mem.alloc(8 * objects.len().max(1) as u32);
        for (i, (kind, id, adjust)) in objects.iter().enumerate() {
            let block = e.mem.alloc(0x400);
            let record = block + 0x100;
            e.mem.set_u8(record + 0x10, *adjust as u8);
            let target = record.wrapping_add(*adjust as u32);
            e.mem.set_u8(target + 0xe8, *kind);
            e.mem.set_u16(target + 0xa8, *id);
            e.mem.set_u32(records + 8 * i as u32 + 4, record);
        }
        e.mem.set_u32(entity + 0x60, records);
        e.mem.set_u32(entity + 0x64, objects.len() as u32);
        entity
    }
    /// The first pass pairs the entity index with each object's id and skips objects of kind 5; a
    /// positive and a negative adjustment both find the object.
    #[test]
    fn fn_00d074c0_pairs_entity_indexes_with_ids_in_the_first_pass() {
        let mut e = Engine::new();
        let entity_zero = entity_with_records(&mut e, &[(1, 11, 0x20), (5, 12, 0)]);
        let entity_one = entity_with_records(&mut e, &[(2, 21, -0x20)]);
        let this = island(&mut e, &[entity_zero, entity_one], &[]);
        let (context, log) = stop_context(&mut e, 0);
        assert!(!e.call(0x00d0_74c0, &args![this, context]).bool());
        assert_eq!(
            *log.borrow(),
            vec![vec![context, 0, 11], vec![context, 1, 21]]
        );
    }

    /// The walk stops with true at the first call that makes the condition hold.
    #[test]
    fn fn_00d074c0_stops_as_soon_as_the_condition_holds() {
        let mut e = Engine::new();
        let entity = entity_with_records(&mut e, &[(1, 11, 0), (1, 12, 0)]);
        let this = island(&mut e, &[entity], &[]);
        let (context, log) = stop_context(&mut e, 1);
        assert!(e.call(0x00d0_74c0, &args![this, context]).bool());
        assert_eq!(*log.borrow(), vec![vec![context, 0, 11]]);
    }
    /// The second pass pairs the two objects of each 0x30-byte table entry unless either is kind 5.
    #[test]
    fn fn_00d074c0_pairs_the_objects_of_each_table_entry_in_the_second_pass() {
        let mut e = Engine::new();
        let entity = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x30 * 4);
        let pairs = [
            (object(&mut e, 1, 31), object(&mut e, 1, 32)),
            (object(&mut e, 5, 33), object(&mut e, 1, 34)),
            (object(&mut e, 3, 41), object(&mut e, 3, 42)),
            (object(&mut e, 2, 35), object(&mut e, 5, 36)),
        ];
        for (i, (first, second)) in pairs.iter().enumerate() {
            e.mem.set_u32(table + 0x30 * i as u32 + 4, *first);
            e.mem.set_u32(table + 0x30 * i as u32 + 8, *second);
        }
        e.mem.set_u32(entity + 0xac, table);
        e.mem.set_u16(entity + 0xb0, 4);
        let this = island(&mut e, &[entity], &[]);
        let (context, log) = stop_context(&mut e, 0);
        assert!(!e.call(0x00d0_74c0, &args![this, context]).bool());
        assert_eq!(
            *log.borrow(),
            vec![vec![context, 31, 32], vec![context, 41, 42]]
        );
    }
    /// An entity without table entries (count 0) is skipped.
    #[test]
    fn fn_00d074c0_skips_entities_without_table_entries() {
        let mut e = Engine::new();
        let entity = e.mem.alloc(0x100);
        let this = island(&mut e, &[entity], &[]);
        let (context, log) = stop_context(&mut e, 0);
        assert!(!e.call(0x00d0_74c0, &args![this, context]).bool());
        assert!(log.borrow().is_empty());
    }

    /// An object whose virtual function at +0x10 fills the array with the given objects.
    fn filler(e: &mut Engine, entries: &[u32]) -> u32 {
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x10, 0x0900_0020);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, table);
        let entries = entries.to_vec();
        e.register_double(0x0900_0020, move |e, a| {
            let data = e.mem.u32(a[1]);
            for (i, entry) in entries.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *entry);
            }
            e.mem.set_u32(a[1] + 4, entries.len() as u32);
            Ret::default()
        });
        owner
    }

    /// The third pass pairs the first entry that is not kind 5 with each later one that is not.
    #[test]
    fn fn_00d074c0_pairs_the_first_usable_entry_with_the_later_ones_in_the_third_pass() {
        let mut e = Engine::new();
        let entries = [
            object(&mut e, 5, 1),
            object(&mut e, 1, 2),
            object(&mut e, 5, 3),
            object(&mut e, 2, 4),
            object(&mut e, 3, 5),
        ];
        let owner = filler(&mut e, &entries);
        // Null slots are skipped.
        let this = island(&mut e, &[], &[0, owner]);
        let (context, log) = stop_context(&mut e, 0);
        assert!(!e.call(0x00d0_74c0, &args![this, context]).bool());
        assert_eq!(
            *log.borrow(),
            vec![vec![context, 2, 4], vec![context, 2, 5]]
        );
    }

    /// The third pass can stop too; and storage that became owned is freed with its size in bytes.
    #[test]
    fn fn_00d074c0_frees_owned_local_storage() {
        let mut e = Engine::new();
        // The island is built first: it installs a router that frees nothing, which this test replaces.
        let first = object(&mut e, 1, 7);
        let second = object(&mut e, 1, 8);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x10, 0x0900_0022);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, table);
        let this = island(&mut e, &[], &[owner]);
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0021);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&freed);
        e.register_double(0x0900_0021, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // The filler grows the array: its own storage, capacity 8 and no flag bits.
        let storage = e.mem.alloc(0x40);
        e.mem.set_u32(storage, first);
        e.mem.set_u32(storage + 4, second);
        e.register_double(0x0900_0022, move |e, a| {
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let (context, log) = stop_context(&mut e, 1);
        // The first pair stops the walk: true, after freeing 8 * 4 bytes.
        assert!(e.call(0x00d0_74c0, &args![this, context]).bool());
        assert_eq!(*log.borrow(), vec![vec![context, 7, 8]]);
        assert_eq!(*freed.borrow(), vec![vec![allocator, storage, 32]]);
    }
}
