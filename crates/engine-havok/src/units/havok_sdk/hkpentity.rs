//! `Havok SDK/hkpentity.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c9d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d1_dff0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9d160(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d1_dff0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c9d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_d3b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9d460(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_d3b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c9d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_cf60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9d490(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_cf60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkpEntity` fields the translations of this file read, by their offsets (Xbox PDB names;
/// the PC offsets are the ones the code uses).
const ENTITY_WORLD: u32 = 0x08;
/// `m_collidable.m_shape` (the collidable starts at +0x10).
const ENTITY_SHAPE: u32 = 0x10;
/// `m_collidable.m_boundingVolumeData` (+0x30) and its child data pointer (+0x54).
const ENTITY_BOUNDING_DATA: u32 = 0x30;
const ENTITY_BOUNDING_CHILD_DATA: u32 = 0x54;
/// `m_constraintsSlave`: data pointer and size.
const ENTITY_SLAVES_DATA: u32 = 0xb4;
const ENTITY_SLAVES_SIZE: u32 = 0xb8;
/// `m_simulationIsland`.
const ENTITY_ISLAND: u32 = 0xcc;
/// `m_motion` (+0xe0) and the motion type byte (+0xe8).
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_MOTION_TYPE: u32 = 0xe8;
/// `m_extendedListeners`.
const ENTITY_EXTENDED_LISTENERS: u32 = 0x214;
/// The `float` (`0x7f7fffee`) `deactivate` stores in the two motion counters.
const DEACTIVATION_COUNTER_LIMIT: u32 = 0x0104_da48;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (Xbox PDB, cdecl `(collision input, entities,
/// count)`).
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkpWorldOperationUtil::splitSimulationIsland_ov2` (Xbox PDB, cdecl `(world, island)`).
const SPLIT_SIMULATION_ISLAND: u32 = 0x00cb_6e70;
/// `hkpWorldOperationUtil::markIslandInactive` (Xbox PDB, cdecl `(world, island)`).
const MARK_ISLAND_INACTIVE: u32 = 0x00cb_5420;
/// Island activation of the world operation util (`hkpworldoperationutil.cpp`, cdecl `(world,
/// island)`; no name in the engine map).
const ACTIVATE_ISLAND: u32 = 0x00cb_5500;
/// A member function of the motion (`this` = `entity + 0xe0`, three byte-sized arguments read
/// from the world at +0x305, +0x306 and +0x307; the callee cleans the stack).
const MOTION_SET_DEACTIVATION_FLAGS: u32 = 0x00c9_0ed0;
/// `hkpCollidable::BoundingVolumeData::deallocate` and `::allocate` (Xbox PDB; `this` = the
/// entity + 0x30; `allocate` takes the size).
const BOUNDING_DATA_DEALLOCATE: u32 = 0x00d0_1a80;
const BOUNDING_DATA_ALLOCATE: u32 = 0x00d0_1ae0;
/// The small-array growth helper (cdecl `(array, element size)`; no name in the engine map).
const SMALL_ARRAY_GROW: u32 = 0x00d0_6cb0;

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The extended listeners block of the entity (`+0x214`), made on first use: 0x10 bytes from
/// the allocator (slot 4), holding two empty small arrays (capacity word `0x8000`). A failed
/// allocation stores a null pointer, which the code then dereferences.
fn extended_listeners(e: &mut Engine, entity: u32) -> u32 {
    if e.mem.u32(entity + ENTITY_EXTENDED_LISTENERS) == 0 {
        let allocator = router_allocator(e);
        let block = e.vcall(allocator, 4, &args![0x10u32]).u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u16(block + 4, 0);
            e.mem.set_u16(block + 6, 0x8000);
            e.mem.set_u32(block + 8, 0);
            e.mem.set_u16(block + 0xc, 0);
            e.mem.set_u16(block + 0xe, 0x8000);
        }
        e.mem.set_u32(entity + ENTITY_EXTENDED_LISTENERS, block);
    }
    e.mem.u32(entity + ENTITY_EXTENDED_LISTENERS)
}

// Translated from 00c9bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::updateCachedAabb` (Xbox PDB): with a world and a shape, recalculates the AABB of
/// this one entity through the world's collision input (`world + 0x78`).
pub fn hkp_entity_update_cached_aabb(e: &mut Engine, this: Ptr) {
    let world = e.mem.u32(this.addr() + ENTITY_WORLD);
    if world != 0 && e.mem.u32(this.addr() + ENTITY_SHAPE) != 0 {
        let input = e.mem.u32(world + 0x78);
        e.with_stack(4, |e, list| {
            e.mem.set_u32(list.addr(), this.addr());
            e.call(ENTITY_BATCH_RECALC_AABB, &args![input, list, 1u32]);
        });
    }
}

// Translated from 00c9bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::isActive` (Xbox PDB): writes the island's activation state (bits 2 and 3 of the
/// island byte at +0x26) to the byte at `result`, 0 for an entity without an island, and answers
/// `result` (`MOV EAX,[ESP+4]` on both paths: the callers read the flag through `EAX`, 0062435f).
pub fn hkp_entity_is_active(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let island = e.mem.u32(this.addr() + ENTITY_ISLAND);
    let state = if island == 0 {
        0
    } else {
        (e.mem.u8(island + 0x26) >> 2) & 3
    };
    e.mem.set_u8(result.addr(), state);
    result
}

// Translated from 00c9c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::deactivate` (Xbox PDB): for an entity whose island is active, first (when the
/// island byte at +0x25 has any of its low four bits set) splits the island with the world's
/// word at +0xb4 cleared meanwhile, then sets both motion counters (+0x1dc, +0x1ec) of every
/// entity of the island (list at island +0x48, count at +0x4c) to the large `float` at
/// `0104da48`, and marks the island inactive.
pub fn hkp_entity_deactivate(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let island = e.mem.u32(me + ENTITY_ISLAND);
    if island == 0 || (e.mem.u8(island + 0x26) >> 2) & 3 == 0 {
        return;
    }
    if e.mem.u8(island + 0x25) & 0xf != 0 {
        let world = e.mem.u32(me + ENTITY_WORLD);
        let saved = e.mem.u32(world + 0xb4);
        e.mem.set_u32(world + 0xb4, 0);
        let island = e.mem.u32(me + ENTITY_ISLAND);
        let flags = e.mem.u8(island + 0x25);
        e.mem.set_u8(island + 0x25, flags & 0xf3);
        let island = e.mem.u32(me + ENTITY_ISLAND);
        let flags = e.mem.u8(island + 0x25);
        e.mem.set_u8(island + 0x25, (flags & 0xfd) | 1);
        let island = e.mem.u32(me + ENTITY_ISLAND);
        let world = e.mem.u32(me + ENTITY_WORLD);
        e.call(SPLIT_SIMULATION_ISLAND, &args![world, island]);
        let world = e.mem.u32(me + ENTITY_WORLD);
        e.mem.set_u32(world + 0xb4, saved);
    }
    let island = e.mem.u32(me + ENTITY_ISLAND);
    if e.mem.i32(island + 0x4c) > 0 {
        let limit = x87_quiet(e.mem.f32(DEACTIVATION_COUNTER_LIMIT));
        let mut slot = e.mem.u32(island + 0x48);
        let mut index = 0;
        loop {
            let entity = e.mem.u32(slot);
            e.mem.set_f32(entity + 0x1dc, limit);
            index += 1;
            e.mem.set_f32(entity + 0x1ec, limit);
            slot += 4;
            if index >= e.mem.i32(island + 0x4c) {
                break;
            }
        }
    }
    let world = e.mem.u32(me + ENTITY_WORLD);
    e.call(MARK_ISLAND_INACTIVE, &args![world, island]);
}

/// `cmpLessConstraint` of `hkpentity.cpp`'s anonymous namespace: constraint `a` sorts before
/// `b` when its priority word (+0x34, unsigned) is lower, or equal and the `m_uid` (+0xd4) of
/// its other entity (the xor of the two entity pointers at +0x14 and +0x18 with `owner`) is
/// lower.
fn constraint_less(e: &mut Engine, a: u32, b: u32, owner: u32) -> bool {
    let key_a = e.mem.u32(a + 0x34);
    let key_b = e.mem.u32(b + 0x34);
    if key_a != key_b {
        return key_a < key_b;
    }
    let other_a = e.mem.u32(a + 0x18) ^ e.mem.u32(a + 0x14) ^ owner;
    let other_b = e.mem.u32(b + 0x18) ^ e.mem.u32(b + 0x14) ^ owner;
    e.mem.u32(other_a + 0xd4) < e.mem.u32(other_b + 0xd4)
}

fn array_slot(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(4))
}

// Translated from 00c9d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpConstraintInstance, cmpLessConstraint>` (Xbox PDB):
/// sorts the constraint pointers `array[low..=high]` with `constraint_less` (middle pivot,
/// Hoare partition, recursion on the lower part, loop on the upper); `owner` is the entity
/// whose slave constraints these are.
pub fn hk_algorithm_quick_sort_recursive_cmp_less_constraint(
    e: &mut Engine,
    array: u32,
    low: i32,
    high: i32,
    owner: u32,
) {
    let mut low = low;
    loop {
        let pivot = e.mem.u32(array_slot(array, (low + high) >> 1));
        let mut left = low;
        let mut right = high;
        loop {
            loop {
                let item = e.mem.u32(array_slot(array, left));
                if !constraint_less(e, item, pivot, owner) {
                    break;
                }
                left += 1;
            }
            loop {
                let item = e.mem.u32(array_slot(array, right));
                if !constraint_less(e, pivot, item, owner) {
                    break;
                }
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                let from_right = e.mem.u32(array_slot(array, right));
                let from_left = e.mem.u32(array_slot(array, left));
                e.mem.set_u32(array_slot(array, right), from_left);
                e.mem.set_u32(array_slot(array, left), from_right);
            }
            left += 1;
            right -= 1;
            if left > right {
                break;
            }
        }
        if low < right {
            hk_algorithm_quick_sort_recursive_cmp_less_constraint(e, array, low, right, owner);
        }
        low = left;
        if low >= high {
            return;
        }
    }
}

// Translated from 00c9c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::sortConstraintsSlavesDeterministically` (Xbox PDB): for a motion of type 5 sorts
/// the slave constraint list (at +0xb4, count +0xb8) when it has more than one entry, then
/// writes each constraint's new position into the 16-bit field at +0x22 of its entry in the
/// other entity's master list (data at +0xac, count 16-bit at +0xb0, 0x30 bytes per entry, the
/// constraint pointer first).
pub fn hkp_entity_sort_constraints_slaves_deterministically(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if e.mem.u8(me + ENTITY_MOTION_TYPE) != 5 {
        return;
    }
    let count = e.mem.i32(me + ENTITY_SLAVES_SIZE);
    if count > 1 {
        let data = e.mem.u32(me + ENTITY_SLAVES_DATA);
        hk_algorithm_quick_sort_recursive_cmp_less_constraint(e, data, 0, count - 1, me);
    }
    let mut position = 0i32;
    while position < e.mem.i32(me + ENTITY_SLAVES_SIZE) {
        let data = e.mem.u32(me + ENTITY_SLAVES_DATA);
        let constraint = e.mem.u32(array_slot(data, position));
        let other = e.mem.u32(constraint + 0x18) ^ e.mem.u32(constraint + 0x14) ^ me;
        let entries = u32::from(e.mem.u16(other + 0xb0));
        let masters = e.mem.u32(other + 0xac);
        for entry in 0..entries {
            if e.mem.u32(masters + entry * 0x30) == constraint {
                e.mem
                    .set_u16(masters + entry * 0x30 + 0x22, position as u16);
                break;
            }
        }
        position += 1;
    }
}

// Translated from 00c9c040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::setCachedShapeData` (Xbox PDB): releases the entity's bounding-volume data when
/// it has child data (+0x54), then, for a `world` and `shape` (both required), looks up the
/// shape type (`shape + 0xc`) in the world's dispatcher table (`world + 0x80`, entries at
/// +0x110). A negative low byte takes the shape's slot `0x10` result; otherwise bit 8 must be
/// set (and then the entity must not be of motion type 4 or 5). Allocates bounding-volume data
/// of the size that result's slot `4` answers, sets `+0x30 = 1`, `+0x40 = 0` and, for motion
/// type 5, recalculates the AABB.
pub fn hkp_entity_set_cached_shape_data(e: &mut Engine, this: Ptr, world: Ptr, shape: Ptr) {
    let me = this.addr();
    if e.mem.u32(me + ENTITY_BOUNDING_CHILD_DATA) != 0 {
        e.call(BOUNDING_DATA_DEALLOCATE, &args![me + ENTITY_BOUNDING_DATA]);
    }
    if world.is_null() || shape.is_null() {
        return;
    }
    let shape_type = e.mem.u32(shape.addr() + 0xc);
    let dispatcher = e.mem.u32(world.addr() + 0x80);
    let flags = e.mem.u32(dispatcher + 0x110 + shape_type * 4);
    let second_form = (flags as u8) & 0x80 == 0;
    if second_form && flags & 0x100 == 0 {
        return;
    }
    let inner = e.vcall(shape.addr(), 0x10, &args![]).u32();
    let motion_type = e.mem.u8(me + ENTITY_MOTION_TYPE);
    if (motion_type == 5 || motion_type == 4) && second_form {
        return;
    }
    let size = e.vcall(inner, 4, &args![]).u32();
    e.call(
        BOUNDING_DATA_ALLOCATE,
        &args![me + ENTITY_BOUNDING_DATA, size],
    );
    e.mem.set_u32(me + ENTITY_BOUNDING_DATA, 1);
    e.mem.set_u32(me + 0x40, 0);
    if e.mem.u8(me + ENTITY_MOTION_TYPE) == 5 {
        hkp_entity_update_cached_aabb(e, this);
    }
}

// Translated from 00c9c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::activate` (Xbox PDB): for an entity without an active island, whose motion type
/// is not 5 and which has a world, clears the two words at +0xea and +0xec, activates the
/// island through the world operation util and passes the world's three flag bytes
/// (+0x305, +0x306, +0x307) to the motion (`this + 0xe0`).
pub fn hkp_entity_activate(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let island = e.mem.u32(me + ENTITY_ISLAND);
    if island != 0 && (e.mem.u8(island + 0x26) >> 2) & 3 != 0 {
        return;
    }
    if e.mem.u8(me + ENTITY_MOTION_TYPE) == 5 {
        return;
    }
    let world = e.mem.u32(me + ENTITY_WORLD);
    if world == 0 {
        return;
    }
    e.mem.set_u16(me + 0xea, 0);
    e.mem.set_u16(me + 0xec, 0);
    e.call(ACTIVATE_ISLAND, &args![world, island]);
    let world = e.mem.u32(me + ENTITY_WORLD);
    let first = u32::from(e.mem.u8(world + 0x305));
    let second = u32::from(e.mem.u8(world + 0x306));
    let third = u32::from(e.mem.u8(world + 0x307));
    e.call(
        MOTION_SET_DEACTIVATION_FLAGS,
        &args![me + ENTITY_MOTION, first, second, third],
    );
}

// Translated from 00c9c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): adds `listener` to the second small array of the entity's
/// extended listeners block (`+0x214`, array at +8 of the block; made on first use). Reuses the
/// first null entry; otherwise grows the array (`00d06cb0`) when it is full and appends.
pub fn fn_00c9c5b0(e: &mut Engine, this: Ptr, listener: u32) {
    let block = extended_listeners(e, this.addr());
    let array = block + 8;
    let size = u32::from(e.mem.u16(block + 0xc));
    if size > 0 {
        let data = e.mem.u32(array);
        let mut index = 0u32;
        loop {
            if e.mem.u32(data + index * 4) == 0 {
                e.mem.set_u32(data + index * 4, listener);
                return;
            }
            index += 1;
            if index >= u32::from(e.mem.u16(array + 4)) {
                break;
            }
        }
    }
    let capacity = e.mem.u16(array + 6) & 0x3fff;
    if size as u16 == capacity {
        e.call(SMALL_ARRAY_GROW, &args![array, 4u32]);
    }
    let count = u32::from(e.mem.u16(array + 4));
    let data = e.mem.u32(array);
    e.mem.set_u32(data + count * 4, listener);
    e.mem.set_u16(array + 4, (count as u16).wrapping_add(1));
}

// Translated from 00c9c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): in the second small array of the entity's extended listeners
/// block (made on first use) sets the first entry equal to `listener` to null; when there is
/// none (or the array is empty) writes the null to the element before the data, as the code
/// does.
pub fn fn_00c9c670(e: &mut Engine, this: Ptr, listener: u32) {
    let block = extended_listeners(e, this.addr());
    let array = block + 8;
    let count = u32::from(e.mem.u16(block + 0xc));
    let data = e.mem.u32(array);
    for index in 0..count {
        if e.mem.u32(data + index * 4) == listener {
            e.mem.set_u32(data + index * 4, 0);
            return;
        }
    }
    e.mem.set_u32(data.wrapping_sub(4), 0);
}

/// The reference-count release of `hkReferencedObject` (`hkreferencedobject.cpp`, no map name;
/// `this` = the object).
const REFERENCED_OBJECT_RELEASE: u32 = 0x00c9_05b0;
/// `hkArrayUtil::_reserve` (Xbox PDB, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_UTIL_RESERVE: u32 = 0x00c9_08c0;
/// `hkStatisticsCollectorUtil::addClassContents` (Xbox PDB, cdecl `(object, class record,
/// collector)`).
const ADD_CLASS_CONTENTS: u32 = 0x00cf_6580;
/// `hkpWorldObject::calcContentStatistics` (Xbox PDB; `this`, the collector and the class record).
const WORLD_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00cd_2b00;
/// `hkpWorldObject::hkpWorldObject` (Xbox PDB; `this`, the construction argument and `1`).
const WORLD_OBJECT_CONSTRUCT: u32 = 0x00cd_2ce0;
/// The no-name `hkpWorldObject` constructor variant that `0x00c9ce00` uses (`this`, one word).
const WORLD_OBJECT_CONSTRUCT_COPY: u32 = 0x00cd_2c50;
/// `hkpKeyframedRigidMotion::~hkpKeyframedRigidMotion` (Xbox PDB; `this` = the motion).
const MOTION_DESTRUCT: u32 = 0x00d1_dff0;
/// `hkpEntityCallbackUtil::fireEntityDeleted` (Xbox PDB, cdecl `(entity)`).
const FIRE_ENTITY_DELETED: u32 = 0x00cf_fe50;
/// `hkStringPtr::~hkStringPtr` (Xbox PDB; `this` = the string field).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// The statistics collector's slot `0x14` is told `(kind, name, data, bytes used, bytes
/// allocated)`; kind `3` is the one every array of this class uses.
const STATISTICS_ARRAY_KIND: u32 = 3;
/// Names of the reported blocks (strings in the exe).
const NAME_CONSTRAINTS_SLAVE: u32 = 0x010c_6f4c;
const NAME_COLLISION_LISTENERS: u32 = 0x010c_6f3c;
const NAME_ENTITY_LISTENERS: u32 = 0x010c_6f2c;
const NAME_ACTIVATION_LISTENERS: u32 = 0x010c_6f18;
const NAME_SAVED_MOTION: u32 = 0x010c_6f0c;
/// The class record `addClassContents` is given for the entity.
const ENTITY_CLASS_RECORD: u32 = 0x0126_88c8;
/// The class record `hkpWorldObject::calcContentStatistics` is given for the base.
const WORLD_OBJECT_CLASS_RECORD: u32 = 0x0126_8808;
/// The vtable of `hkpEntity` (RTTI `.?AVhkpEntity@@`).
const ENTITY_VTABLE: u32 = 0x010c_6f7c;
/// The vtable of `hkpWorldObject` (RTTI `.?AVhkpWorldObject@@`) and of its base `hkBaseObject`.
const WORLD_OBJECT_VTABLE: u32 = 0x010c_6f64;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The vtables of `hkpMotion`, `hkpMaxSizeMotion` and the six motion classes (RTTI).
const MOTION_VTABLE: u32 = 0x010c_59f4;
const MAX_SIZE_MOTION_VTABLE: u32 = 0x010c_6c64;
/// Constants the `hkpEntity` constructor copies through the x87 stack.
const ENTITY_DEFAULT_RESTITUTION: u32 = 0x0101_6248;
const ENTITY_DEFAULT_FRICTION: u32 = 0x0102_31a8;

/// Removes the first entry equal to `listener` from the small array at `array` (data pointer
/// at +0, 16-bit size at +4) by writing a null over it; when there is none (or the array is
/// empty) the null goes to the element before the data, as the code does.
fn small_array_remove(e: &mut Engine, array: u32, listener: u32) {
    let count = u32::from(e.mem.u16(array + 4));
    let data = e.mem.u32(array);
    if count != 0 {
        for index in 0..count {
            if e.mem.u32(data + index * 4) == listener {
                e.mem.set_u32(data + index * 4, 0);
                return;
            }
        }
    }
    e.mem.set_u32(data.wrapping_sub(4), 0);
}

/// Adds `value` to the small array at `array` (data +0, 16-bit size +4, 16-bit capacity word
/// +6): the first null entry is reused; otherwise the array is grown (`00d06cb0`, cdecl
/// `(array, element size)`) when its size equals the capacity (low 14 bits) and the value is
/// appended.
fn small_array_add(e: &mut Engine, array: u32, value: u32) {
    let size = u32::from(e.mem.u16(array + 4));
    if size != 0 {
        let data = e.mem.u32(array);
        for index in 0..size {
            if e.mem.u32(data + index * 4) == 0 {
                e.mem.set_u32(data + index * 4, value);
                return;
            }
        }
    }
    if e.mem.u16(array + 4) == e.mem.u16(array + 6) & 0x3fff {
        e.call(SMALL_ARRAY_GROW, &args![array, 4u32]);
    }
    let size = u32::from(e.mem.u16(array + 4));
    let data = e.mem.u32(array);
    e.mem.set_u32(data + size * 4, value);
    let size = e.mem.u16(array + 4);
    e.mem.set_u16(array + 4, size.wrapping_add(1));
}

/// True when some entry of the small array at `array` (data +0, 16-bit size +4) is not null.
fn small_array_has_entry(e: &mut Engine, array: u32) -> bool {
    let size = u32::from(e.mem.u16(array + 4));
    if size != 0 {
        let data = e.mem.u32(array);
        for index in 0..size {
            if e.mem.u32(data + index * 4) != 0 {
                return true;
            }
        }
    }
    false
}

/// Returns the heap block of the small array at `array` to the allocator (slot 8: pointer,
/// `capacity * element_size` bytes) when its capacity word (+6) does not have bit 15 set.
fn small_array_free_block(e: &mut Engine, array: u32, element_size: u32) {
    let capacity = u32::from(e.mem.u16(array + 6));
    if capacity & 0x8000 == 0 {
        let data = e.mem.u32(array);
        let allocator = router_allocator(e);
        e.vcall(
            allocator,
            8,
            &args![data, (capacity & 0x3fff) * element_size],
        );
    }
}

/// Empties the small array at `array`: frees its block when it owns it, sets the size and the
/// data pointer to zero and the capacity word to `(capacity & 0xc000) | 0x8000`.
fn small_array_release(e: &mut Engine, array: u32, element_size: u32) {
    small_array_free_block(e, array, element_size);
    e.mem.set_u16(array + 4, 0);
    let capacity = e.mem.u16(array + 6);
    e.mem.set_u32(array, 0);
    e.mem.set_u16(array + 6, (capacity & 0xc000) | 0x8000);
}

/// Releases an `hkArray` with 32-bit size and capacity words (data +0, size +4, capacity +8)
/// the way the entity's destructor and `deallocateInternalArrays` do: the router is asked
/// first, the size is cleared, the block is freed through slot `0x10` (pointer, bytes) when the
/// capacity word's top bit is clear, and data and capacity are reset.
fn big_array_release(e: &mut Engine, array: u32, element_shift: u32) {
    let allocator = router_allocator(e);
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.u32(array + 8);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff) << element_shift],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00c9c0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (identity unconfirmed). In the first small array of the extended listeners block
/// (`+0x214`, data +0, size +4; the block must exist) sets the first entry equal to `listener`
/// to null; when there is none writes the null to the element before the data.
pub fn fn_00c9c0f0(e: &mut Engine, this: Ptr, listener: u32) {
    let block = e.mem.u32(this.addr() + ENTITY_EXTENDED_LISTENERS);
    small_array_remove(e, block, listener);
}

// Translated from 00c9c140 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (identity unconfirmed). In the small array at `+0x200` (data +0x200, size
/// +0x204) sets the first entry equal to `listener` to null; when there is none writes the null
/// to the element before the data.
pub fn fn_00c9c140(e: &mut Engine, this: Ptr, listener: u32) {
    small_array_remove(e, this.addr() + 0x200, listener);
}

// Translated from 00c9c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The number of constraints of the entity: the 16-bit count of the master list
/// (+0xb0) plus the size of the slave list (+0xb8).
pub fn fn_00c9c380(e: &mut Engine, this: Ptr) -> i32 {
    let masters = i32::from(e.mem.u16(this.addr() + 0xb0));
    masters.wrapping_add(e.mem.i32(this.addr() + ENTITY_SLAVES_SIZE))
}

/// The constraint at `index` of the combined list: the first `+0xb0` entries come from the
/// master list (data +0xac, 0x30 bytes per entry, the constraint pointer first), the rest from
/// the slave list (data +0xb4).
fn constraint_at(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let me = this.addr();
    let masters = i32::from(e.mem.u16(me + 0xb0));
    if index < masters {
        let data = e.mem.u32(me + 0xac);
        e.mem
            .u32((index as u32).wrapping_mul(0x30).wrapping_add(data))
    } else {
        let data = e.mem.u32(me + ENTITY_SLAVES_DATA);
        e.mem
            .u32(data.wrapping_add(((index - masters) as u32).wrapping_mul(4)))
    }
}

// Translated from 00c9c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The constraint pointer at `index` of the combined master-then-slave list.
pub fn fn_00c9c390(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    constraint_at(e, this, index)
}

// Translated from 00c9c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name; the same body as `00c9c390` (the constraint pointer at `index` of the combined
/// master-then-slave list).
pub fn fn_00c9c3c0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    constraint_at(e, this, index)
}

/// Reports one array to the collector (slot `0x14`: kind, name, data, bytes used, bytes
/// allocated).
fn report_array(e: &mut Engine, collector: u32, name: u32, data: u32, used: u32, allocated: u32) {
    e.vcall(
        collector,
        0x14,
        &args![STATISTICS_ARRAY_KIND, name, data, used, allocated],
    );
}

// Translated from 00c9c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::calcContentStatistics` (Xbox PDB): reports the slave constraint array, the
/// entity's class contents, the collision listener array and (when the extended block exists)
/// its two listener arrays to the collector (slot `0x14`), an array without owned memory (top
/// bit of its capacity word set) only when the collector's byte at +0x10 is set; then the saved
/// motion (slot `0xc`: name, pointer, 0) and the `hkpWorldObject` contents. The second stack
/// word is not read.
pub fn hkp_entity_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused_1: u32,
) {
    let me = this.addr();
    let used = e.mem.u32(me + ENTITY_SLAVES_SIZE) << 2;
    let allocated = (e.mem.u32(me + 0xbc) & 0x3fff_ffff) << 2;
    let data = e.mem.u32(me + ENTITY_SLAVES_DATA);
    report_array(e, collector, NAME_CONSTRAINTS_SLAVE, data, used, allocated);
    e.call(
        ADD_CLASS_CONTENTS,
        &args![me, ENTITY_CLASS_RECORD, collector],
    );
    let capacity = u32::from(e.mem.u16(me + 0x206));
    if capacity & 0x8000 == 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(me + 0x200);
        let used = u32::from(e.mem.u16(me + 0x204)) << 2;
        report_array(
            e,
            collector,
            NAME_COLLISION_LISTENERS,
            data,
            used,
            (capacity & 0x3fff) << 2,
        );
    }
    let block = e.mem.u32(me + ENTITY_EXTENDED_LISTENERS);
    if block != 0 {
        let capacity = u32::from(e.mem.u16(block + 0xe));
        if capacity & 0x8000 == 0 || e.mem.u8(collector + 0x10) != 0 {
            let data = e.mem.u32(block + 8);
            let used = u32::from(e.mem.u16(block + 0xc)) << 2;
            report_array(
                e,
                collector,
                NAME_ENTITY_LISTENERS,
                data,
                used,
                (capacity & 0x3fff) << 2,
            );
        }
        let block = e.mem.u32(me + ENTITY_EXTENDED_LISTENERS);
        let capacity = u32::from(e.mem.u16(block + 6));
        if capacity & 0x8000 == 0 || e.mem.u8(collector + 0x10) != 0 {
            let data = e.mem.u32(block);
            let used = u32::from(e.mem.u16(block + 4)) << 2;
            report_array(
                e,
                collector,
                NAME_ACTIVATION_LISTENERS,
                data,
                used,
                (capacity & 0x3fff) << 2,
            );
        }
    }
    let saved_motion = e.mem.u32(me + 0x1f8);
    e.vcall(
        collector,
        0xc,
        &args![NAME_SAVED_MOTION, saved_motion, 0u32],
    );
    e.call(
        WORLD_OBJECT_CALC_CONTENT_STATISTICS,
        &args![me, collector, WORLD_OBJECT_CLASS_RECORD],
    );
}

// Translated from 00c9c540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (identity unconfirmed). Adds `listener` to the small array at `+0x200` (data
/// +0x200, size +0x204, capacity word +0x206): reuses the first null entry, otherwise grows
/// the array when it is full and appends.
pub fn fn_00c9c540(e: &mut Engine, this: Ptr, listener: u32) {
    small_array_add(e, this.addr() + 0x200, listener);
}

// Translated from 00c9c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (identity unconfirmed). Makes the extended listeners block (`+0x214`) on first
/// use, then adds `listener` to its first small array (data +0, size +4, capacity word +6) the
/// way `00c9c540` does.
pub fn fn_00c9c700(e: &mut Engine, this: Ptr, listener: u32) {
    let block = extended_listeners(e, this.addr());
    small_array_add(e, block, listener);
}

// Translated from 00c9c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Copies every constraint of the entity into the `hkArray` at `array` (data +0,
/// size +4, capacity +8): the pointers of the master list entries (0x30 bytes apart), then the
/// slave list. The array is reserved (`hkArrayUtil::_reserve`, to twice its capacity or the
/// count, whichever is larger) when it is too small.
pub fn fn_00c9c7c0(e: &mut Engine, this: Ptr, array: u32) {
    let me = this.addr();
    let count = fn_00c9c380(e, this);
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let mut wanted = capacity.wrapping_mul(2);
        if count >= wanted {
            wanted = count;
        }
        e.call(ARRAY_UTIL_RESERVE, &args![allocator, array, wanted, 4u32]);
    }
    e.mem.set_i32(array + 4, count);
    let mut out = 0u32;
    let mut entry = 0u32;
    if e.mem.u16(me + 0xb0) != 0 {
        let mut taken = 0i32;
        loop {
            let masters = e.mem.u32(me + 0xac);
            let constraint = e.mem.u32(masters.wrapping_add(entry));
            let data = e.mem.u32(array);
            e.mem
                .set_u32(data.wrapping_add(out.wrapping_mul(4)), constraint);
            taken += 1;
            out += 1;
            entry = entry.wrapping_add(0x30);
            if taken >= i32::from(e.mem.u16(me + 0xb0)) {
                break;
            }
        }
    }
    let mut slave = 0i32;
    if e.mem.i32(me + ENTITY_SLAVES_SIZE) > 0 {
        let mut offset = out.wrapping_mul(4);
        loop {
            let slaves = e.mem.u32(me + ENTITY_SLAVES_DATA);
            let constraint = e
                .mem
                .u32(slaves.wrapping_add((slave as u32).wrapping_mul(4)));
            let data = e.mem.u32(array);
            e.mem.set_u32(offset.wrapping_add(data), constraint);
            slave += 1;
            offset = offset.wrapping_add(4);
            if slave >= e.mem.i32(me + ENTITY_SLAVES_SIZE) {
                break;
            }
        }
    }
}

// Translated from 00c9c8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::deallocateInternalArrays` (Xbox PDB): destroys and clears the listener object at
/// `+0x98` (removing its entry from the collision listener array first), then releases every
/// array that has become empty: the blocks at `+0x60` and `+0xb4`/`+0xc0` (32-bit size and
/// capacity words), the master list (`+0xac`, 0x30 bytes per entry) when its count is zero, the
/// collision listener array (`+0x200`) and the two arrays of the extended block (`+0x214`)
/// when they hold only nulls (the block itself is freed when neither holds an entry), and the
/// array at `+0x208` when its size is zero.
pub fn hkp_entity_deallocate_internal_arrays(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let listener_object = e.mem.u32(me + 0x98);
    if listener_object != 0 {
        fn_00c9c140(e, this, listener_object + 8);
        e.vcall(listener_object, 0, &args![1u32]);
        e.mem.set_u32(me + 0x98, 0);
    }
    if e.mem.u32(me + 0x64) == 0 {
        big_array_release(e, me + 0x60, 3);
    }
    if e.mem.u16(me + 0xb0) == 0 {
        small_array_release(e, me + 0xac, 0x30);
    }
    if e.mem.u32(me + ENTITY_SLAVES_SIZE) == 0 {
        big_array_release(e, me + ENTITY_SLAVES_DATA, 2);
    }
    if e.mem.u32(me + 0xc4) == 0 {
        big_array_release(e, me + 0xc0, 0);
    }
    if !small_array_has_entry(e, me + 0x200) {
        small_array_release(e, me + 0x200, 4);
    }
    let mut first_in_use = false;
    let block = e.mem.u32(me + ENTITY_EXTENDED_LISTENERS);
    if block != 0 {
        if small_array_has_entry(e, block) {
            first_in_use = true;
        } else {
            small_array_release(e, block, 4);
        }
    }
    let mut second_in_use = false;
    let block = e.mem.u32(me + ENTITY_EXTENDED_LISTENERS);
    if block != 0 {
        if small_array_has_entry(e, block + 8) {
            second_in_use = true;
        } else {
            small_array_release(e, block + 8, 4);
        }
    }
    if !first_in_use && !second_in_use {
        let block = e.mem.u32(me + ENTITY_EXTENDED_LISTENERS);
        if block != 0 {
            small_array_free_block(e, block + 8, 4);
            small_array_free_block(e, block, 4);
            let allocator = router_allocator(e);
            e.vcall(allocator, 8, &args![block, 0x10u32]);
        }
        e.mem.set_u32(me + ENTITY_EXTENDED_LISTENERS, 0);
    }
    if e.mem.u16(me + 0x20c) == 0 {
        small_array_release(e, me + 0x208, 4);
    }
}

// Translated from 00c9cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::hkpEntity` (Xbox PDB): runs the `hkpWorldObject` constructor with `info` and `1`,
/// installs the `hkpEntity` vtable (`010c6f7c`) and the defaults: `0x90` and `0x94` copied from
/// the floats at `01016248` and `010231a8`, `0x9c` = 1.0, empty arrays (capacity words
/// `0x8000` / `0x80000000`) at `+0xac`, `+0xb4`, `+0xc0`, `+0x200`, `+0x208`, `+0xe0` = the
/// `hkpMaxSizeMotion` vtable, `+0xd4` = -1 and the other fields zero. Returns `this`.
pub fn hkp_entity_construct(e: &mut Engine, this: Ptr, info: u32) -> Ptr {
    let me = this.addr();
    e.call(WORLD_OBJECT_CONSTRUCT, &args![me, info, 1u32]);
    e.mem.set_u32(me, ENTITY_VTABLE);
    let restitution = x87_quiet(e.mem.f32(ENTITY_DEFAULT_RESTITUTION));
    e.mem.set_f32(me + 0x90, restitution);
    let friction = x87_quiet(e.mem.f32(ENTITY_DEFAULT_FRICTION));
    e.mem.set_f32(me + 0x94, friction);
    e.mem.set_u32(me + 0xac, 0);
    e.mem.set_u16(me + 0xb0, 0);
    e.mem.set_u16(me + 0xb2, 0x8000);
    e.mem.set_u32(me + 0xb4, 0);
    e.mem.set_u32(me + 0xb8, 0);
    e.mem.set_u32(me + 0xbc, 0x8000_0000);
    e.mem.set_u32(me + 0xc8, 0x8000_0000);
    e.mem.set_u32(me + 0xc0, 0);
    e.mem.set_u32(me + 0xc4, 0);
    e.mem.set_u32(me + 0xd8, 0);
    e.mem.set_u8(me + 0xde, 3);
    e.mem.set_u8(me + 0xdf, 1);
    e.mem.set_u16(me + 0xdc, 0);
    e.mem.set_u32(me + 0x1f8, 0);
    e.mem.set_u32(me + 0xe0, MAX_SIZE_MOTION_VTABLE);
    e.mem.set_u32(me + 0x200, 0);
    e.mem.set_u16(me + 0x204, 0);
    e.mem.set_u16(me + 0x206, 0x8000);
    e.mem.set_u32(me + 0x208, 0);
    e.mem.set_u16(me + 0x20c, 0);
    e.mem.set_u16(me + 0x20e, 0x8000);
    e.mem.set_u32(me + 0x210, 0);
    e.mem.set_f32(me + 0x9c, 1.0);
    e.mem.set_u32(me + 0xcc, 0);
    e.mem.set_u32(me + ENTITY_EXTENDED_LISTENERS, 0);
    e.mem.set_u32(me + 0x98, 0);
    e.mem.set_u32(me + 0xa4, 0);
    e.mem.set_u8(me + 0xd1, 0);
    e.mem.set_u32(me + 0xa0, 0);
    e.mem.set_u16(me + 0xa8, 0xffff);
    e.mem.set_u32(me + 0xd4, 0xffff_ffff);
    this
}

// Translated from 00c9ce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a second `hkpEntity` constructor. Runs the no-name `hkpWorldObject`
/// constructor variant (`00cd2c50`) with `other`, installs the `hkpEntity` vtable and empties
/// the master list (`+0xac`), the slave list (`+0xb4`), the block at `+0xc0` and the arrays at
/// `+0x200` and `+0x208`. The motion at `+0xe0` gets the `hkpMotion` vtable and, when `other`
/// is not null, the 16-bit word `0x3f80` (the high half of 1.0f) at `+0x1fe`, then the
/// `hkpMaxSizeMotion` vtable; for a non-null `other` the motion type byte (`+0xe8`) 2 to 7 then
/// picks one of the motion constructor tails (`00c9d190`, `00c9d1c0`, `00c9d100`, `00c9d130`,
/// `00c9d1f0`, `00c9d220`, called with `this + 0xe0` and `other`). Returns `this`.
pub fn fn_00c9ce00(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    let me = this.addr();
    e.call(WORLD_OBJECT_CONSTRUCT_COPY, &args![me, other]);
    e.mem.set_u32(me, ENTITY_VTABLE);
    e.mem.set_u16(me + 0xb0, 0);
    e.mem.set_u32(me + 0xac, 0);
    e.mem.set_u16(me + 0xb2, 0x8000);
    e.mem.set_u32(me + 0xbc, 0x8000_0000);
    e.mem.set_u32(me + 0xb4, 0);
    e.mem.set_u32(me + 0xb8, 0);
    e.mem.set_u32(me + 0xc8, 0x8000_0000);
    let motion = me.wrapping_add(ENTITY_MOTION);
    e.mem.set_u32(me + 0xc0, 0);
    e.mem.set_u32(me + 0xc4, 0);
    e.mem.set_u32(motion, MOTION_VTABLE);
    if other != 0 {
        e.mem.set_u16(motion.wrapping_add(0x11e), 0x3f80);
    }
    e.mem.set_u32(motion, MAX_SIZE_MOTION_VTABLE);
    e.mem.set_u16(me + 0x204, 0);
    e.mem.set_u16(me + 0x206, 0x8000);
    e.mem.set_u32(me + 0x200, 0);
    e.mem.set_u16(me + 0x20c, 0);
    e.mem.set_u32(me + 0x208, 0);
    e.mem.set_u16(me + 0x20e, 0x8000);
    if other != 0 && motion != 0 {
        match e.mem.u8(me + ENTITY_MOTION_TYPE) {
            2 => fn_00c9d190(e, Ptr::new(motion), other),
            3 => fn_00c9d1c0(e, Ptr::new(motion), other),
            4 => fn_00c9d100(e, Ptr::new(motion), other),
            5 => fn_00c9d130(e, Ptr::new(motion), other),
            6 => fn_00c9d1f0(e, Ptr::new(motion), other),
            7 => fn_00c9d220(e, Ptr::new(motion), other),
            _ => {}
        }
    }
    this
}

// Translated from 00c9cf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the `hkpEntity` destructor body (the vtable `010c6f7c` is `hkpEntity`'s by
/// RTTI). Installs that vtable, destroys the listener object at `+0x98` as
/// `deallocateInternalArrays` does, fires `fireEntityDeleted`, destroys the extended block
/// (`+0x214`, freeing it with its 0x10 bytes), releases the object at `+0x210`, frees the
/// array blocks at `+0x208` and `+0x200`, destroys the motion at `+0xe0`, frees the blocks at
/// `+0xc0`, `+0xb4` and `+0xac`, and finishes in the `hkpWorldObject` destructor
/// (`00c9d3b0`).
pub fn fn_00c9cf60(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let listener_object = e.mem.u32(me + 0x98);
    e.mem.set_u32(me, ENTITY_VTABLE);
    if listener_object != 0 {
        fn_00c9c140(e, this, listener_object + 8);
        e.vcall(listener_object, 0, &args![1u32]);
        e.mem.set_u32(me + 0x98, 0);
    }
    e.call(FIRE_ENTITY_DELETED, &args![me]);
    let block = e.mem.u32(me + ENTITY_EXTENDED_LISTENERS);
    if block != 0 {
        fn_00c9d340(e, Ptr::new(block));
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![block, 0x10u32]);
    }
    let object = e.mem.u32(me + 0x210);
    if object != 0 {
        e.call(REFERENCED_OBJECT_RELEASE, &args![object]);
    }
    e.mem.set_u32(me + 0x210, 0);
    small_array_free_block(e, me + 0x208, 4);
    small_array_free_block(e, me + 0x200, 4);
    e.call(MOTION_DESTRUCT, &args![me + ENTITY_MOTION]);
    big_array_release(e, me + 0xc0, 0);
    big_array_release(e, me + ENTITY_SLAVES_DATA, 2);
    small_array_free_block(e, me + 0xac, 0x30);
    hkp_world_object_destruct(e, this);
}

/// The common head of the six motion constructor tails: the `hkpMotion` vtable, the 16-bit
/// word `0x3f80` at `+0x11e` when `flag` is not zero, then `final_vtable`.
fn motion_tail(e: &mut Engine, this: Ptr, flag: u32, final_vtable: u32) {
    let me = this.addr();
    e.mem.set_u32(me, MOTION_VTABLE);
    if flag != 0 {
        e.mem.set_u16(me.wrapping_add(0x11e), 0x3f80);
    }
    e.mem.set_u32(me, final_vtable);
}

// Translated from 00c9d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a motion constructor tail (`this` = the motion). Stores the `hkpMotion` vtable,
/// `0x3f80` at `+0x11e` when `flag` is not zero (the high half of 1.0f), then the vtable of
/// `hkpKeyframedRigidMotion` (`010c6bec`, by RTTI).
pub fn fn_00c9d100(e: &mut Engine, this: Ptr, flag: u32) {
    motion_tail(e, this, flag, 0x010c_6bec);
}

// Translated from 00c9d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a motion constructor tail like `00c9d100` ending with the vtable of
/// `hkpFixedRigidMotion` (`010c6cdc`, by RTTI).
pub fn fn_00c9d130(e: &mut Engine, this: Ptr, flag: u32) {
    motion_tail(e, this, flag, 0x010c_6cdc);
}

// Translated from 00c9d190 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a motion constructor tail like `00c9d100` ending with the vtable of
/// `hkpSphereMotion` (`010c6d54`, by RTTI).
pub fn fn_00c9d190(e: &mut Engine, this: Ptr, flag: u32) {
    motion_tail(e, this, flag, 0x010c_6d54);
}

// Translated from 00c9d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a motion constructor tail like `00c9d100` ending with the vtable of
/// `hkpBoxMotion` (`010c6dc4`, by RTTI).
pub fn fn_00c9d1c0(e: &mut Engine, this: Ptr, flag: u32) {
    motion_tail(e, this, flag, 0x010c_6dc4);
}

// Translated from 00c9d1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a motion constructor tail like `00c9d100` ending with the vtable of
/// `hkpThinBoxMotion` (`010c6e34`, by RTTI).
pub fn fn_00c9d1f0(e: &mut Engine, this: Ptr, flag: u32) {
    motion_tail(e, this, flag, 0x010c_6e34);
}

// Translated from 00c9d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a motion constructor tail like `00c9d100` ending with the vtable of
/// `hkpCharacterMotion` (`010c6ea4`, by RTTI).
pub fn fn_00c9d220(e: &mut Engine, this: Ptr, flag: u32) {
    motion_tail(e, this, flag, 0x010c_6ea4);
}

// Translated from 00c9d340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: destructor of the extended listeners block (`this` = the block, `__fastcall`).
/// Frees the heap block of the second small array (data +8, capacity word +0xe) and then of
/// the first (data +0, capacity word +6) through the allocator (slot 8) when each owns its
/// memory.
pub fn fn_00c9d340(e: &mut Engine, this: Ptr) {
    small_array_free_block(e, this.addr() + 8, 4);
    small_array_free_block(e, this.addr(), 4);
}

// Translated from 00c9d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::~hkpWorldObject` (Xbox PDB): installs the `hkpWorldObject` vtable
/// (`010c6f64`), releases the objects at `+0x10` and `+0x84` when set, frees the blocks at
/// `+0x78` (16 bytes per entry) and `+0x60` (8 bytes per entry), runs the `hkStringPtr`
/// destructor on `+0x74` between them, and installs the base object's vtable (`0102e388`).
pub fn hkp_world_object_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, WORLD_OBJECT_VTABLE);
    let first = e.mem.u32(me + 0x10);
    if first != 0 {
        e.call(REFERENCED_OBJECT_RELEASE, &args![first]);
    }
    let second = e.mem.u32(me + 0x84);
    if second != 0 {
        e.call(REFERENCED_OBJECT_RELEASE, &args![second]);
    }
    big_array_release(e, me + 0x78, 4);
    e.call(STRING_PTR_DESTRUCT, &args![me + 0x74]);
    big_array_release(e, me + 0x60, 3);
    e.mem.set_u32(me, BASE_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_c0f0, fn_00c9c0f0(Ptr, u32)),
        entry!(0x00c9_c140, fn_00c9c140(Ptr, u32)),
        entry!(0x00c9_c380, fn_00c9c380(Ptr) -> i32),
        entry!(0x00c9_c390, fn_00c9c390(Ptr, i32) -> u32),
        entry!(0x00c9_c3c0, fn_00c9c3c0(Ptr, i32) -> u32),
        entry!(
            0x00c9_c400,
            hkp_entity_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(0x00c9_c540, fn_00c9c540(Ptr, u32)),
        entry!(0x00c9_c700, fn_00c9c700(Ptr, u32)),
        entry!(0x00c9_c7c0, fn_00c9c7c0(Ptr, u32)),
        entry!(0x00c9_c8f0, hkp_entity_deallocate_internal_arrays(Ptr)),
        entry!(0x00c9_cce0, hkp_entity_construct(Ptr, u32) -> Ptr),
        entry!(0x00c9_ce00, fn_00c9ce00(Ptr, u32) -> Ptr),
        entry!(0x00c9_cf60, fn_00c9cf60(Ptr)),
        entry!(0x00c9_d100, fn_00c9d100(Ptr, u32)),
        entry!(0x00c9_d130, fn_00c9d130(Ptr, u32)),
        entry!(0x00c9_d190, fn_00c9d190(Ptr, u32)),
        entry!(0x00c9_d1c0, fn_00c9d1c0(Ptr, u32)),
        entry!(0x00c9_d1f0, fn_00c9d1f0(Ptr, u32)),
        entry!(0x00c9_d220, fn_00c9d220(Ptr, u32)),
        entry!(0x00c9_d340, fn_00c9d340(Ptr)),
        entry!(0x00c9_d3b0, hkp_world_object_destruct(Ptr)),
        entry!(0x00c9_d160, deleting_destructor_00c9d160(Ptr, u8) -> Ptr),
        entry!(0x00c9_d460, deleting_destructor_00c9d460(Ptr, u8) -> Ptr),
        entry!(0x00c9_d490, deleting_destructor_00c9d490(Ptr, u8) -> Ptr),
        entry!(0x00c9_bfa0, hkp_entity_update_cached_aabb(Ptr)),
        entry!(0x00c9_bff0, hkp_entity_is_active(Ptr, Ptr) -> Ptr),
        entry!(0x00c9_c240, hkp_entity_deactivate(Ptr)),
        entry!(
            0x00c9_d250,
            hk_algorithm_quick_sort_recursive_cmp_less_constraint(u32, i32, i32, u32)
        ),
        entry!(
            0x00c9_c860,
            hkp_entity_sort_constraints_slaves_deterministically(Ptr)
        ),
        entry!(0x00c9_c040, hkp_entity_set_cached_shape_data(Ptr, Ptr, Ptr)),
        entry!(0x00c9_c1d0, hkp_entity_activate(Ptr)),
        entry!(0x00c9_c5b0, fn_00c9c5b0(Ptr, u32)),
        entry!(0x00c9_c670, fn_00c9c670(Ptr, u32)),
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
    fn deleting_destructor_00c9d160_frees_only_when_bit_zero_is_set() {
        check(0x00c9_d160, 0x00d1_dff0);
    }

    #[test]
    fn deleting_destructor_00c9d460_frees_only_when_bit_zero_is_set() {
        check(0x00c9_d460, 0x00c9_d3b0);
    }

    #[test]
    fn deleting_destructor_00c9d490_frees_only_when_bit_zero_is_set() {
        check(0x00c9_d490, 0x00c9_cf60);
    }

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn new_log() -> CallLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// Registers a double at `address` that records its argument words and answers `result`.
    fn record(e: &mut Engine, log: &CallLog, address: u32, result: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn update_cached_aabb_needs_a_world_and_a_shape() {
        let mut e = engine();
        let log = new_log();
        let seen = Rc::clone(&log);
        e.register_double(ENTITY_BATCH_RECALC_AABB, move |e, a| {
            let first = e.mem.u32(a[1]);
            seen.borrow_mut()
                .push((ENTITY_BATCH_RECALC_AABB, vec![a[0], first, a[2]]));
            Ret::default()
        });
        let entity = e.mem.alloc(0x220);
        e.call(0x00c9_bfa0, &args![entity]);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x78, 0x1234);
        e.mem.set_u32(entity + 8, world);
        e.call(0x00c9_bfa0, &args![entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(entity + 0x10, 0x5555);
        e.call(0x00c9_bfa0, &args![entity]);
        assert_eq!(
            calls_to(&log, ENTITY_BATCH_RECALC_AABB),
            vec![vec![0x1234, entity, 1]]
        );
    }

    #[test]
    fn is_active_reads_the_island_state_bits() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0x77);
        e.call(0x00c9_bff0, &args![entity, out]);
        assert_eq!(e.mem.u8(out), 0);
        let island = e.mem.alloc(0x80);
        e.mem.set_u8(island + 0x26, 0b1011_0100);
        e.mem.set_u32(entity + 0xcc, island);
        e.call(0x00c9_bff0, &args![entity, out]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u8(island + 0x26, 0b1111_1100);
        e.call(0x00c9_bff0, &args![entity, out]);
        assert_eq!(e.mem.u8(out), 3);
    }

    struct DeactivateScene {
        entity: u32,
        world: u32,
        island: u32,
        members: [u32; 2],
    }

    fn deactivate_scene(e: &mut Engine, island_flags: u8) -> DeactivateScene {
        e.mem.set_u32(0x0104_da48, 0x7f7f_ffee);
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        let island = e.mem.alloc(0x80);
        let members = [e.mem.alloc(0x220), e.mem.alloc(0x220)];
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, members[0]);
        e.mem.set_u32(list + 4, members[1]);
        e.mem.set_u32(island + 0x48, list);
        e.mem.set_u32(island + 0x4c, 2);
        e.mem.set_u8(island + 0x25, island_flags);
        e.mem.set_u8(island + 0x26, 0b0000_0100);
        e.mem.set_u32(entity + 8, world);
        e.mem.set_u32(entity + 0xcc, island);
        e.mem.set_u32(world + 0xb4, 0xabcd);
        DeactivateScene {
            entity,
            world,
            island,
            members,
        }
    }

    #[test]
    fn deactivate_splits_the_island_and_marks_it_inactive() {
        let mut e = engine();
        let scene = deactivate_scene(&mut e, 0x02);
        let log = new_log();
        let seen = Rc::clone(&log);
        e.register_double(SPLIT_SIMULATION_ISLAND, move |e, a| {
            // The world's word is cleared and the island's flags rewritten at the call.
            seen.borrow_mut().push((
                SPLIT_SIMULATION_ISLAND,
                vec![
                    a[0],
                    a[1],
                    e.mem.u32(a[0] + 0xb4),
                    u32::from(e.mem.u8(a[1] + 0x25)),
                ],
            ));
            Ret::default()
        });
        record(&mut e, &log, MARK_ISLAND_INACTIVE, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        assert_eq!(
            calls_to(&log, SPLIT_SIMULATION_ISLAND),
            vec![vec![scene.world, scene.island, 0, 0x01]]
        );
        assert_eq!(e.mem.u32(scene.world + 0xb4), 0xabcd);
        for member in scene.members {
            assert_eq!(e.mem.u32(member + 0x1dc), 0x7f7f_ffee);
            assert_eq!(e.mem.u32(member + 0x1ec), 0x7f7f_ffee);
        }
        assert_eq!(
            calls_to(&log, MARK_ISLAND_INACTIVE),
            vec![vec![scene.world, scene.island]]
        );
        let order: Vec<u32> = log.borrow().iter().map(|(at, _)| *at).collect();
        assert_eq!(order, vec![SPLIT_SIMULATION_ISLAND, MARK_ISLAND_INACTIVE]);
    }

    #[test]
    fn deactivate_without_split_flags_only_resets_counters() {
        let mut e = engine();
        let scene = deactivate_scene(&mut e, 0xf0);
        let log = new_log();
        record(&mut e, &log, SPLIT_SIMULATION_ISLAND, 0);
        record(&mut e, &log, MARK_ISLAND_INACTIVE, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        assert!(calls_to(&log, SPLIT_SIMULATION_ISLAND).is_empty());
        assert_eq!(calls_to(&log, MARK_ISLAND_INACTIVE).len(), 1);
        assert_eq!(e.mem.u32(scene.members[1] + 0x1ec), 0x7f7f_ffee);
    }

    #[test]
    fn deactivate_ignores_an_entity_without_an_active_island() {
        let mut e = engine();
        let scene = deactivate_scene(&mut e, 0x02);
        let log = new_log();
        record(&mut e, &log, MARK_ISLAND_INACTIVE, 0);
        e.mem.set_u8(scene.island + 0x26, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        e.mem.set_u32(scene.entity + 0xcc, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(scene.members[0] + 0x1dc), 0);
    }

    /// A constraint with priority `key` whose other entity (reached through the xor of the
    /// two entity words with `owner`) has the uid `uid`.
    fn make_constraint(e: &mut Engine, owner: u32, key: u32, uid: u32) -> (u32, u32) {
        let other = e.mem.alloc(0x220);
        e.mem.set_u32(other + 0xd4, uid);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0x34, key);
        e.mem.set_u32(constraint + 0x14, owner);
        e.mem.set_u32(constraint + 0x18, other);
        (constraint, other)
    }

    #[test]
    fn quick_sort_orders_by_priority_then_uid() {
        let mut e = engine();
        let owner = 0x1000;
        let specs = [(3, 50), (1, 40), (2, 10), (1, 5), (3, 7), (2, 99), (0, 60)];
        let mut items = Vec::new();
        for (key, uid) in specs {
            items.push((make_constraint(&mut e, owner, key, uid).0, (key, uid)));
        }
        let array = e.mem.alloc(4 * items.len() as u32);
        for (index, (constraint, _)) in items.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *constraint);
        }
        e.call(
            0x00c9_d250,
            &args![array, 0u32, items.len() as u32 - 1, owner],
        );
        let mut expected = items.clone();
        expected.sort_by_key(|(_, order)| *order);
        for (index, (constraint, _)) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(array + 4 * index as u32), *constraint);
        }
    }

    #[test]
    fn quick_sort_leaves_a_single_element_range_alone() {
        let mut e = engine();
        let array = e.mem.alloc(8);
        let item = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(array, item);
        e.mem.set_u32(array + 4, 0x2222);
        e.call(0x00c9_d250, &args![array, 0u32, 0u32, owner]);
        assert_eq!(e.mem.u32(array), item);
        assert_eq!(e.mem.u32(array + 4), 0x2222);
    }

    #[test]
    fn sort_slaves_sorts_and_numbers_the_masters_entries() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let specs = [(2, 30), (1, 20), (1, 10)];
        let mut made = Vec::new();
        for (key, uid) in specs {
            let (constraint, other) = make_constraint(&mut e, entity, key, uid);
            // The other entity's master list: a decoy first, then the constraint.
            let masters = e.mem.alloc(0x60);
            e.mem.set_u32(masters, 0xdead);
            e.mem.set_u32(masters + 0x30, constraint);
            e.mem.set_u32(other + 0xac, masters);
            e.mem.set_u16(other + 0xb0, 2);
            made.push((constraint, masters, (key, uid)));
        }
        let slaves = e.mem.alloc(12);
        for (index, (constraint, _, _)) in made.iter().enumerate() {
            e.mem.set_u32(slaves + 4 * index as u32, *constraint);
        }
        e.mem.set_u32(entity + 0xb4, slaves);
        e.mem.set_i32(entity + 0xb8, 3);
        // Motion type 4: untouched.
        e.mem.set_u8(entity + 0xe8, 4);
        e.call(0x00c9_c860, &args![entity]);
        assert_eq!(e.mem.u32(slaves), made[0].0);
        e.mem.set_u8(entity + 0xe8, 5);
        e.call(0x00c9_c860, &args![entity]);
        let mut expected = made.clone();
        expected.sort_by_key(|(_, _, order)| *order);
        for (position, (constraint, masters, _)) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(slaves + 4 * position as u32), *constraint);
            assert_eq!(e.mem.u16(masters + 0x30 + 0x22), position as u16);
            assert_eq!(e.mem.u16(masters + 0x22), 0);
        }
    }

    const SHAPE_VTABLE_SLOT_DOUBLE: u32 = 0x0900_0020;
    const INNER_SIZE_SLOT_DOUBLE: u32 = 0x0900_0021;

    struct ShapeScene {
        entity: u32,
        world: u32,
        shape: u32,
        inner: u32,
    }

    fn shape_scene(e: &mut Engine, flags: u32) -> ShapeScene {
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        let dispatcher = e.mem.alloc(0x200);
        e.mem.set_u32(world + 0x80, dispatcher);
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, 5);
        e.mem.set_u32(dispatcher + 0x110 + 5 * 4, flags);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, SHAPE_VTABLE_SLOT_DOUBLE);
        e.mem.set_u32(shape, vtable);
        let inner = e.mem.alloc(0x40);
        let inner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(inner_vtable + 4, INNER_SIZE_SLOT_DOUBLE);
        e.mem.set_u32(inner, inner_vtable);
        e.register_double(SHAPE_VTABLE_SLOT_DOUBLE, move |_, _| Ret {
            eax: inner,
            ..Ret::default()
        });
        e.register_double(INNER_SIZE_SLOT_DOUBLE, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        ShapeScene {
            entity,
            world,
            shape,
            inner,
        }
    }

    #[test]
    fn set_cached_shape_data_releases_old_data_first() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x80);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_DEALLOCATE, 0);
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        e.mem.set_u32(scene.entity + 0x54, 0x4444);
        e.call(0x00c9_c040, &args![scene.entity, 0u32, scene.shape]);
        assert_eq!(
            calls_to(&log, BOUNDING_DATA_DEALLOCATE),
            vec![vec![scene.entity + 0x30]]
        );
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
        e.call(0x00c9_c040, &args![scene.entity, scene.world, 0u32]);
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
    }

    #[test]
    fn set_cached_shape_data_allocates_for_a_flagged_shape() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x80);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        record(&mut e, &log, ENTITY_BATCH_RECALC_AABB, 0);
        e.mem.set_u32(scene.entity + 0x40, 9);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert_eq!(
            calls_to(&log, BOUNDING_DATA_ALLOCATE),
            vec![vec![scene.entity + 0x30, 0x77]]
        );
        assert_eq!(e.mem.u32(scene.entity + 0x30), 1);
        assert_eq!(e.mem.u32(scene.entity + 0x40), 0);
        assert!(calls_to(&log, ENTITY_BATCH_RECALC_AABB).is_empty());
        assert_ne!(scene.inner, 0);
    }

    #[test]
    fn set_cached_shape_data_second_form_needs_bit_eight_and_a_dynamic_motion() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x100);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        e.mem.set_u8(scene.entity + 0xe8, 5);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
        e.mem.set_u8(scene.entity + 0xe8, 4);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
        e.mem.set_u8(scene.entity + 0xe8, 1);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert_eq!(calls_to(&log, BOUNDING_DATA_ALLOCATE).len(), 1);
        let none = shape_scene(&mut e, 0x01);
        e.call(0x00c9_c040, &args![none.entity, none.world, none.shape]);
        assert_eq!(calls_to(&log, BOUNDING_DATA_ALLOCATE).len(), 1);
    }

    #[test]
    fn set_cached_shape_data_recalculates_the_aabb_for_motion_type_five() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x80);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        record(&mut e, &log, ENTITY_BATCH_RECALC_AABB, 0);
        e.mem.set_u8(scene.entity + 0xe8, 5);
        e.mem.set_u32(scene.entity + 8, scene.world);
        e.mem.set_u32(scene.entity + 0x10, scene.shape);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert_eq!(calls_to(&log, ENTITY_BATCH_RECALC_AABB).len(), 1);
    }

    #[test]
    fn activate_runs_for_an_inactive_non_fixed_entity_with_a_world() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        e.mem.set_u8(world + 0x305, 1);
        e.mem.set_u8(world + 0x306, 2);
        e.mem.set_u8(world + 0x307, 3);
        let log = new_log();
        record(&mut e, &log, ACTIVATE_ISLAND, 0);
        record(&mut e, &log, MOTION_SET_DEACTIVATION_FLAGS, 0);
        // No world: nothing happens.
        e.call(0x00c9_c1d0, &args![entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(entity + 8, world);
        e.mem.set_u16(entity + 0xea, 5);
        e.mem.set_u16(entity + 0xec, 6);
        e.call(0x00c9_c1d0, &args![entity]);
        assert_eq!(e.mem.u16(entity + 0xea), 0);
        assert_eq!(e.mem.u16(entity + 0xec), 0);
        assert_eq!(calls_to(&log, ACTIVATE_ISLAND), vec![vec![world, 0]]);
        assert_eq!(
            calls_to(&log, MOTION_SET_DEACTIVATION_FLAGS),
            vec![vec![entity + 0xe0, 1, 2, 3]]
        );
    }

    #[test]
    fn activate_skips_active_islands_and_motion_type_five() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(entity + 8, world);
        let island = e.mem.alloc(0x80);
        e.mem.set_u8(island + 0x26, 0b0000_1000);
        e.mem.set_u32(entity + 0xcc, island);
        let log = new_log();
        record(&mut e, &log, ACTIVATE_ISLAND, 0);
        record(&mut e, &log, MOTION_SET_DEACTIVATION_FLAGS, 0);
        e.call(0x00c9_c1d0, &args![entity]);
        assert!(log.borrow().is_empty());
        // An inactive island is passed on.
        e.mem.set_u8(island + 0x26, 0);
        e.mem.set_u8(entity + 0xe8, 5);
        e.call(0x00c9_c1d0, &args![entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(entity + 0xe8, 2);
        e.call(0x00c9_c1d0, &args![entity]);
        assert_eq!(calls_to(&log, ACTIVATE_ISLAND), vec![vec![world, island]]);
    }

    const ALLOCATOR_SLOT_DOUBLE: u32 = 0x0900_0030;

    /// A memory router whose allocator hands out heap blocks, and a double for the small
    /// array growth helper that gives the array room for four entries.
    fn listener_scene(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATOR_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATOR_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(SMALL_ARRAY_GROW, |e, a| {
            let room = e.mem.alloc(16);
            e.mem.set_u32(a[0], room);
            e.mem.set_u16(a[0] + 6, 4);
            Ret::default()
        });
        e.mem.alloc(0x220)
    }

    #[test]
    fn listener_add_makes_the_block_and_reuses_null_entries() {
        let mut e = engine();
        let entity = listener_scene(&mut e);
        e.call(0x00c9_c5b0, &args![entity, 7u32]);
        let block = e.mem.u32(entity + 0x214);
        assert_ne!(block, 0);
        assert_eq!(e.mem.u16(block + 0xc), 1);
        assert_eq!(e.mem.u16(block + 0xe), 4);
        let data = e.mem.u32(block + 8);
        assert_eq!(e.mem.u32(data), 7);
        e.call(0x00c9_c5b0, &args![entity, 8u32]);
        assert_eq!(e.mem.u32(data + 4), 8);
        assert_eq!(e.mem.u16(block + 0xc), 2);
        // Remove the first, then a new listener takes its place instead of growing the array.
        e.call(0x00c9_c670, &args![entity, 7u32]);
        assert_eq!(e.mem.u32(data), 0);
        e.call(0x00c9_c5b0, &args![entity, 9u32]);
        assert_eq!(e.mem.u32(data), 9);
        assert_eq!(e.mem.u16(block + 0xc), 2);
        // A listener in the first array (+0) is not touched.
        assert_eq!(e.mem.u16(block + 4), 0);
    }

    #[test]
    fn listener_remove_writes_before_the_data_when_absent() {
        let mut e = engine();
        let entity = listener_scene(&mut e);
        let block = e.mem.alloc(0x10);
        let region = e.mem.alloc(0x10);
        e.mem.set_u32(region, 0xaaaa);
        e.mem.set_u32(region + 4, 5);
        e.mem.set_u32(block + 8, region + 4);
        e.mem.set_u16(block + 0xc, 1);
        e.mem.set_u32(entity + 0x214, block);
        e.call(0x00c9_c670, &args![entity, 6u32]);
        assert_eq!(e.mem.u32(region), 0);
        assert_eq!(e.mem.u32(region + 4), 5);
    }

    const SMALL_FREE_DOUBLE: u32 = 0x0900_0040;
    const BIG_FREE_DOUBLE: u32 = 0x0900_0050;
    /// `(allocator slot, pointer, bytes)` of every free the allocator saw.
    type FreeLog = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// A memory router whose allocator answers slot 4 with heap blocks and records the frees
    /// of slots 8 and 0x10.
    fn router_with_frees(e: &mut Engine) -> FreeLog {
        let frees: FreeLog = Rc::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATOR_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 8, SMALL_FREE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, BIG_FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATOR_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        for (address, slot) in [(SMALL_FREE_DOUBLE, 8), (BIG_FREE_DOUBLE, 0x10)] {
            let sink = Rc::clone(&frees);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((slot, a[1], a[2]));
                Ret::default()
            });
        }
        frees
    }

    /// A small array at `array`: data pointer, 16-bit size and capacity word, holding `items`.
    fn make_small_array(e: &mut Engine, array: u32, items: &[u32], capacity: u16) {
        let data = e.mem.alloc(0x40);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(data + index as u32 * 4, *item);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u16(array + 4, items.len() as u16);
        e.mem.set_u16(array + 6, capacity);
    }

    #[test]
    fn listener_removal_nulls_the_first_match_or_the_element_before_the_data() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        make_small_array(&mut e, entity + 0x200, &[5, 6, 6], 4);
        e.call(0x00c9_c140, &args![entity, 6u32]);
        let data = e.mem.u32(entity + 0x200);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [5, 0, 6]
        );
        e.mem.set_u32(data - 4, 0x77);
        e.call(0x00c9_c140, &args![entity, 9u32]);
        assert_eq!(e.mem.u32(data - 4), 0);
        let block = e.mem.alloc(0x10);
        make_small_array(&mut e, block, &[1, 2], 4);
        e.mem.set_u32(entity + 0x214, block);
        e.call(0x00c9_c0f0, &args![entity, 1u32]);
        let data = e.mem.u32(block);
        assert_eq!([e.mem.u32(data), e.mem.u32(data + 4)], [0, 2]);
        e.mem.set_u32(data - 4, 0x77);
        e.call(0x00c9_c0f0, &args![entity, 9u32]);
        assert_eq!(e.mem.u32(data - 4), 0);
    }

    #[test]
    fn constraint_count_and_lookup_span_masters_then_slaves() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let masters = e.mem.alloc(0x100);
        e.mem.set_u32(masters, 0x1000);
        e.mem.set_u32(masters + 0x30, 0x2000);
        let slaves = e.mem.alloc(0x10);
        e.mem.set_u32(slaves, 0x3000);
        e.mem.set_u32(slaves + 4, 0x4000);
        e.mem.set_u32(entity + 0xac, masters);
        e.mem.set_u16(entity + 0xb0, 2);
        e.mem.set_u32(entity + 0xb4, slaves);
        e.mem.set_u32(entity + 0xb8, 2);
        assert_eq!(e.call(0x00c9_c380, &args![entity]).u32(), 4);
        for address in [0x00c9_c390, 0x00c9_c3c0] {
            let got: Vec<u32> = (0..4i32)
                .map(|index| e.call(address, &args![entity, index]).u32())
                .collect();
            assert_eq!(got, vec![0x1000, 0x2000, 0x3000, 0x4000]);
        }
    }

    /// A statistics collector whose slots `0x14` and `0xc` record their words.
    fn collector_scene(e: &mut Engine, log: &CallLog) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_0060);
        e.mem.set_u32(vtable + 0x14, 0x0900_0061);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, vtable);
        record(e, log, 0x0900_0060, 0);
        record(e, log, 0x0900_0061, 0);
        record(e, log, ADD_CLASS_CONTENTS, 0);
        record(e, log, WORLD_OBJECT_CALC_CONTENT_STATISTICS, 0);
        collector
    }

    #[test]
    fn calc_content_statistics_reports_arrays_that_own_memory() {
        let mut e = engine();
        let log = new_log();
        let collector = collector_scene(&mut e, &log);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + 0xb4, 0x1111);
        e.mem.set_u32(entity + 0xb8, 3);
        e.mem.set_u32(entity + 0xbc, 0xc000_0005);
        e.mem.set_u32(entity + 0x200, 0x2222);
        e.mem.set_u16(entity + 0x204, 2);
        e.mem.set_u16(entity + 0x206, 0x4006);
        e.mem.set_u32(entity + 0x1f8, 0x5555);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 8, 0x3333);
        e.mem.set_u16(block + 0xc, 1);
        e.mem.set_u16(block + 0xe, 0x0007);
        e.mem.set_u32(block, 0x4444);
        e.mem.set_u16(block + 4, 4);
        e.mem.set_u16(block + 6, 0x0008);
        e.mem.set_u32(entity + 0x214, block);
        e.call(0x00c9_c400, &args![entity, collector, 0u32]);
        let kinds = calls_to(&log, 0x0900_0061);
        assert_eq!(
            kinds,
            vec![
                vec![collector, 3, 0x010c_6f4c, 0x1111, 12, 20],
                vec![collector, 3, 0x010c_6f3c, 0x2222, 8, 24],
                vec![collector, 3, 0x010c_6f2c, 0x3333, 4, 28],
                vec![collector, 3, 0x010c_6f18, 0x4444, 16, 32],
            ]
        );
        assert_eq!(
            calls_to(&log, 0x0900_0060),
            vec![vec![collector, 0x010c_6f0c, 0x5555, 0]]
        );
        assert_eq!(
            calls_to(&log, ADD_CLASS_CONTENTS),
            vec![vec![entity, 0x0126_88c8, collector]]
        );
        assert_eq!(
            calls_to(&log, WORLD_OBJECT_CALC_CONTENT_STATISTICS),
            vec![vec![entity, collector, 0x0126_8808]]
        );
    }

    #[test]
    fn calc_content_statistics_skips_borrowed_arrays_unless_asked() {
        let mut e = engine();
        let log = new_log();
        let collector = collector_scene(&mut e, &log);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u16(entity + 0x206, 0x8003);
        let block = e.mem.alloc(0x10);
        e.mem.set_u16(block + 6, 0x8001);
        e.mem.set_u16(block + 0xe, 0x8002);
        e.mem.set_u32(entity + 0x214, block);
        e.call(0x00c9_c400, &args![entity, collector, 0u32]);
        assert_eq!(calls_to(&log, 0x0900_0061).len(), 1);
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00c9_c400, &args![entity, collector, 0u32]);
        let reports = calls_to(&log, 0x0900_0061);
        assert_eq!(reports.len(), 4);
        assert_eq!(reports[1][5], 12);
        assert_eq!(reports[2][5], 8);
        assert_eq!(reports[3][5], 4);
    }

    #[test]
    fn listener_arrays_reuse_null_entries_and_grow_when_full() {
        let mut e = engine();
        let _frees = router_with_frees(&mut e);
        e.register_double(SMALL_ARRAY_GROW, |e, a| {
            let room = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], room);
            e.mem.set_u16(a[0] + 6, 8);
            Ret::default()
        });
        let entity = e.mem.alloc(0x220);
        make_small_array(&mut e, entity + 0x200, &[7, 0, 9], 3);
        e.call(0x00c9_c540, &args![entity, 0x55u32]);
        let data = e.mem.u32(entity + 0x200);
        assert_eq!(e.mem.u32(data + 4), 0x55);
        assert_eq!(e.mem.u16(entity + 0x204), 3);
        // Full: the array grows (the double replaces the block) and the value is appended.
        e.mem.set_u32(data + 4, 8);
        e.call(0x00c9_c540, &args![entity, 0x66u32]);
        let grown = e.mem.u32(entity + 0x200);
        assert_ne!(grown, data);
        assert_eq!(e.mem.u32(grown), 0);
        assert_eq!(e.mem.u32(grown + 12), 0x66);
        assert_eq!(e.mem.u16(entity + 0x204), 4);
        // The extended block is made on first use and its first array receives the value.
        e.call(0x00c9_c700, &args![entity, 0x77u32]);
        let block = e.mem.u32(entity + 0x214);
        assert_ne!(block, 0);
        assert_eq!(e.mem.u16(block + 4), 1);
        assert_eq!(e.mem.u16(block + 6), 8);
        let first = e.mem.u32(block);
        assert_eq!(e.mem.u32(first), 0x77);
        assert_eq!(e.mem.u16(block + 0xc), 0);
        // A null entry is reused before the array grows.
        e.mem.set_u32(first, 0);
        e.call(0x00c9_c700, &args![entity, 0x88u32]);
        assert_eq!(e.mem.u32(first), 0x88);
        assert_eq!(e.mem.u16(block + 4), 1);
    }

    #[test]
    fn constraints_are_copied_into_the_array_masters_first() {
        let mut e = engine();
        let _frees = router_with_frees(&mut e);
        let reserve_log = new_log();
        let sink = Rc::clone(&reserve_log);
        e.register_double(ARRAY_UTIL_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_UTIL_RESERVE, a.to_vec()));
            let room = e.mem.alloc(a[2] * 4);
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let entity = e.mem.alloc(0x220);
        let masters = e.mem.alloc(0x100);
        e.mem.set_u32(masters, 0x1000);
        e.mem.set_u32(masters + 0x30, 0x2000);
        let slaves = e.mem.alloc(0x10);
        e.mem.set_u32(slaves, 0x3000);
        e.mem.set_u32(entity + 0xac, masters);
        e.mem.set_u16(entity + 0xb0, 2);
        e.mem.set_u32(entity + 0xb4, slaves);
        e.mem.set_u32(entity + 0xb8, 1);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, 0x4000_0001);
        e.call(0x00c9_c7c0, &args![entity, array]);
        let calls = calls_to(&reserve_log, ARRAY_UTIL_RESERVE);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..], [array, 3, 4]);
        assert_eq!(e.mem.u32(array + 4), 3);
        let data = e.mem.u32(array);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [0x1000, 0x2000, 0x3000]
        );
        // Enough room already: no reserve.
        e.call(0x00c9_c7c0, &args![entity, array]);
        assert_eq!(calls_to(&reserve_log, ARRAY_UTIL_RESERVE).len(), 1);
    }

    #[test]
    fn deallocate_internal_arrays_frees_empty_arrays_and_the_unused_block() {
        let mut e = engine();
        let frees = router_with_frees(&mut e);
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0070);
        e.mem.set_u32(object, vtable);
        let object_calls = new_log();
        record(&mut e, &object_calls, 0x0900_0070, 0);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + 0x98, object);
        make_small_array(&mut e, entity + 0x200, &[object + 8, 0], 4);
        e.mem.set_u32(entity + 0x60, 0x6000);
        e.mem.set_u32(entity + 0x68, 2);
        make_small_array(&mut e, entity + 0xac, &[], 2);
        e.mem.set_u32(entity + 0xb4, 0xb400);
        e.mem.set_u32(entity + 0xbc, 3);
        e.mem.set_u32(entity + 0xc0, 0xc000);
        e.mem.set_u32(entity + 0xc8, 0x4000_0005);
        make_small_array(&mut e, entity + 0x208, &[], 5);
        let block = e.mem.alloc(0x10);
        make_small_array(&mut e, block, &[0], 6);
        make_small_array(&mut e, block + 8, &[], 7);
        e.mem.set_u32(entity + 0x214, block);
        let first_data = e.mem.u32(block);
        let slot_data = e.mem.u32(entity + 0x200);
        let master_data = e.mem.u32(entity + 0xac);
        let extra_data = e.mem.u32(entity + 0x208);
        let second_data = e.mem.u32(block + 8);
        e.call(0x00c9_c8f0, &args![entity]);
        assert_eq!(calls_to(&object_calls, 0x0900_0070), vec![vec![object, 1]]);
        assert_eq!(e.mem.u32(entity + 0x98), 0);
        // The listener object's entry was removed first, leaving only nulls: freed too.
        assert_eq!(
            *frees.borrow(),
            vec![
                (0x10, 0x6000, 16),
                (8, master_data, 0x60),
                (0x10, 0xb400, 12),
                (0x10, 0xc000, 5),
                (8, slot_data, 16),
                (8, first_data, 24),
                (8, second_data, 28),
                (8, block, 0x10),
                (8, extra_data, 20),
            ]
        );
        assert_eq!(e.mem.u32(entity + 0x214), 0);
        assert_eq!(e.mem.u32(entity + 0x60), 0);
        assert_eq!(e.mem.u32(entity + 0x68), 0x8000_0000);
        assert_eq!(e.mem.u16(entity + 0x206), 0x8000);
    }

    #[test]
    fn construct_installs_the_vtable_and_the_empty_state() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, WORLD_OBJECT_CONSTRUCT, 0);
        e.mem.set_u32(ENTITY_DEFAULT_RESTITUTION, 0.5f32.to_bits());
        // A signalling NaN is quieted by the x87 copy.
        e.mem.set_u32(ENTITY_DEFAULT_FRICTION, 0x7fa0_0000);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + 0x214, 0xdead);
        let got = e.call(0x00c9_cce0, &args![entity, 0x1234u32]).u32();
        assert_eq!(got, entity);
        assert_eq!(
            calls_to(&log, WORLD_OBJECT_CONSTRUCT),
            vec![vec![entity, 0x1234, 1]]
        );
        assert_eq!(e.mem.u32(entity), ENTITY_VTABLE);
        assert_eq!(e.mem.f32(entity + 0x90), 0.5);
        assert_eq!(e.mem.u32(entity + 0x94), 0x7fe0_0000);
        assert_eq!(e.mem.f32(entity + 0x9c), 1.0);
        assert_eq!(e.mem.u16(entity + 0xb2), 0x8000);
        assert_eq!(e.mem.u32(entity + 0xbc), 0x8000_0000);
        assert_eq!(e.mem.u32(entity + 0xc8), 0x8000_0000);
        assert_eq!(e.mem.u8(entity + 0xde), 3);
        assert_eq!(e.mem.u8(entity + 0xdf), 1);
        assert_eq!(e.mem.u32(entity + 0xe0), MAX_SIZE_MOTION_VTABLE);
        assert_eq!(e.mem.u16(entity + 0x206), 0x8000);
        assert_eq!(e.mem.u16(entity + 0x20e), 0x8000);
        assert_eq!(e.mem.u16(entity + 0xa8), 0xffff);
        assert_eq!(e.mem.u32(entity + 0xd4), 0xffff_ffff);
        assert_eq!(e.mem.u32(entity + 0x214), 0);
    }

    #[test]
    fn second_constructor_picks_the_motion_tail_by_motion_type() {
        let tails = [
            (2u8, 0x010c_6d54u32),
            (3, 0x010c_6dc4),
            (4, 0x010c_6bec),
            (5, 0x010c_6cdc),
            (6, 0x010c_6e34),
            (7, 0x010c_6ea4),
        ];
        for (motion_type, vtable) in tails {
            let mut e = engine();
            let log = new_log();
            record(&mut e, &log, WORLD_OBJECT_CONSTRUCT_COPY, 0);
            let entity = e.mem.alloc(0x220);
            e.mem.set_u8(entity + 0xe8, motion_type);
            let got = e.call(0x00c9_ce00, &args![entity, 0x4321u32]).u32();
            assert_eq!(got, entity);
            assert_eq!(
                calls_to(&log, WORLD_OBJECT_CONSTRUCT_COPY),
                vec![vec![entity, 0x4321]]
            );
            assert_eq!(e.mem.u32(entity), ENTITY_VTABLE);
            assert_eq!(e.mem.u32(entity + 0xe0), vtable);
            assert_eq!(e.mem.u16(entity + 0x1fe), 0x3f80);
            assert_eq!(e.mem.u16(entity + 0x206), 0x8000);
            assert_eq!(e.mem.u32(entity + 0xbc), 0x8000_0000);
        }
    }

    #[test]
    fn second_constructor_without_an_argument_keeps_the_max_size_vtable() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, WORLD_OBJECT_CONSTRUCT_COPY, 0);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + 0xe8, 4);
        e.call(0x00c9_ce00, &args![entity, 0u32]);
        assert_eq!(e.mem.u32(entity + 0xe0), MAX_SIZE_MOTION_VTABLE);
        assert_eq!(e.mem.u16(entity + 0x1fe), 0);
        // A motion type outside 2..=7 leaves the max-size vtable too.
        e.mem.set_u8(entity + 0xe8, 8);
        e.call(0x00c9_ce00, &args![entity, 1u32]);
        assert_eq!(e.mem.u32(entity + 0xe0), MAX_SIZE_MOTION_VTABLE);
        assert_eq!(e.mem.u16(entity + 0x1fe), 0x3f80);
    }

    #[test]
    fn motion_tails_store_the_class_vtable_and_the_half_word() {
        let tails = [
            (0x00c9_d100, 0x010c_6bec),
            (0x00c9_d130, 0x010c_6cdc),
            (0x00c9_d190, 0x010c_6d54),
            (0x00c9_d1c0, 0x010c_6dc4),
            (0x00c9_d1f0, 0x010c_6e34),
            (0x00c9_d220, 0x010c_6ea4),
        ];
        for (address, vtable) in tails {
            let mut e = engine();
            let motion = e.mem.alloc(0x130);
            e.call(address, &args![motion, 0u32]);
            assert_eq!(e.mem.u32(motion), vtable);
            assert_eq!(e.mem.u16(motion + 0x11e), 0);
            e.call(address, &args![motion, 5u32]);
            assert_eq!(e.mem.u32(motion), vtable);
            assert_eq!(e.mem.u16(motion + 0x11e), 0x3f80);
        }
    }

    #[test]
    fn listener_block_destructor_frees_the_second_array_then_the_first() {
        let mut e = engine();
        let frees = router_with_frees(&mut e);
        let block = e.mem.alloc(0x10);
        make_small_array(&mut e, block, &[1], 6);
        make_small_array(&mut e, block + 8, &[1], 0x8007);
        let first = e.mem.u32(block);
        e.call(0x00c9_d340, &args![block]);
        assert_eq!(*frees.borrow(), vec![(8, first, 24)]);
        frees.borrow_mut().clear();
        e.mem.set_u16(block + 0xe, 3);
        let second = e.mem.u32(block + 8);
        e.call(0x00c9_d340, &args![block]);
        assert_eq!(*frees.borrow(), vec![(8, second, 12), (8, first, 24)]);
    }

    #[test]
    fn world_object_destructor_releases_and_frees_in_order() {
        let mut e = engine();
        let frees = router_with_frees(&mut e);
        let log = new_log();
        record(&mut e, &log, REFERENCED_OBJECT_RELEASE, 0);
        record(&mut e, &log, STRING_PTR_DESTRUCT, 0);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x10, 0xaa);
        e.mem.set_u32(object + 0x84, 0xbb);
        e.mem.set_u32(object + 0x78, 0x7800);
        e.mem.set_u32(object + 0x7c, 9);
        e.mem.set_u32(object + 0x80, 0x4000_0003);
        e.mem.set_u32(object + 0x60, 0x6000);
        e.mem.set_u32(object + 0x68, 2);
        e.call(0x00c9_d3b0, &args![object]);
        assert_eq!(
            calls_to(&log, REFERENCED_OBJECT_RELEASE),
            vec![vec![0xaa], vec![0xbb]]
        );
        assert_eq!(
            calls_to(&log, STRING_PTR_DESTRUCT),
            vec![vec![object + 0x74]]
        );
        assert_eq!(
            *frees.borrow(),
            vec![(0x10, 0x7800, 48), (0x10, 0x6000, 16)]
        );
        assert_eq!(e.mem.u32(object + 0x7c), 0);
        assert_eq!(e.mem.u32(object + 0x80), 0x8000_0000);
        assert_eq!(e.mem.u32(object + 0x68), 0x8000_0000);
        assert_eq!(e.mem.u32(object), BASE_OBJECT_VTABLE);
        // A negative capacity word means the block is not owned: no free.
        frees.borrow_mut().clear();
        e.mem.set_u32(object + 0x80, 0x8000_0003);
        e.call(0x00c9_d3b0, &args![object]);
        assert!(frees.borrow().is_empty());
    }

    #[test]
    fn entity_destructor_runs_each_step_in_order() {
        let mut e = engine();
        let frees = router_with_frees(&mut e);
        let log = new_log();
        for address in [
            FIRE_ENTITY_DELETED,
            REFERENCED_OBJECT_RELEASE,
            MOTION_DESTRUCT,
            STRING_PTR_DESTRUCT,
        ] {
            record(&mut e, &log, address, 0);
        }
        let entity = e.mem.alloc(0x220);
        let block = e.mem.alloc(0x10);
        make_small_array(&mut e, block, &[1], 6);
        make_small_array(&mut e, block + 8, &[1], 7);
        e.mem.set_u32(entity + 0x214, block);
        e.mem.set_u32(entity + 0x210, 0xcc);
        make_small_array(&mut e, entity + 0x208, &[], 5);
        make_small_array(&mut e, entity + 0x200, &[], 0x8004);
        make_small_array(&mut e, entity + 0xac, &[], 2);
        e.mem.set_u32(entity + 0xc0, 0xc000);
        e.mem.set_u32(entity + 0xc8, 7);
        e.mem.set_u32(entity + 0xb4, 0xb400);
        e.mem.set_u32(entity + 0xbc, 3);
        e.mem.set_u32(entity + 0x68, 0x8000_0000);
        e.mem.set_u32(entity + 0x80, 0x8000_0000);
        let block_first = e.mem.u32(block);
        let block_second = e.mem.u32(block + 8);
        let extra = e.mem.u32(entity + 0x208);
        let masters = e.mem.u32(entity + 0xac);
        e.call(0x00c9_cf60, &args![entity]);
        let order: Vec<u32> = log.borrow().iter().map(|(at, _)| *at).collect();
        assert_eq!(
            order,
            vec![
                FIRE_ENTITY_DELETED,
                REFERENCED_OBJECT_RELEASE,
                MOTION_DESTRUCT,
                STRING_PTR_DESTRUCT,
            ]
        );
        assert_eq!(
            calls_to(&log, MOTION_DESTRUCT),
            vec![vec![entity + ENTITY_MOTION]]
        );
        assert_eq!(
            *frees.borrow(),
            vec![
                (8, block_second, 28),
                (8, block_first, 24),
                (8, block, 0x10),
                (8, extra, 20),
                (0x10, 0xc000, 7),
                (0x10, 0xb400, 12),
                (8, masters, 0x60),
            ]
        );
        assert_eq!(e.mem.u32(entity), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(entity + 0x210), 0);
    }
}
