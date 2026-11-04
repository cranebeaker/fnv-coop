//! `fallout shared/havok/teswaterlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

// `!(a < b)` mirrors the exe's x87 flag tests, which are true for NaN.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESWaterListener` (Xbox PDB), 0x40 bytes; the PC code uses the same
    /// offsets (constructor `0062e780`, `00630120`, `0062ff10`).
    pub struct TESWaterListener: 0x40 {
        /// `bhkEntityListener::bEnabled` (Xbox PDB).
        0x08 bEnabled: bool,
        /// `BGSZoneTargetListener::pZones` (Xbox PDB): first `WaterZone`.
        0x18 pZones: u32,
        /// `fSplashTimer` (Xbox PDB).
        0x1c fSplashTimer: f32,
        /// `pfWaterLevels` (Xbox PDB): `uiCellArrayWidth` squared floats.
        0x20 pfWaterLevels: u32,
        /// `fHighestWaterLevel` (Xbox PDB).
        0x24 fHighestWaterLevel: f32,
        /// `uiCellArrayWidth` (Xbox PDB).
        0x28 uiCellArrayWidth: u32,
        /// `iBaseCellNumberX` (Xbox PDB).
        0x2c iBaseCellNumberX: i32,
        /// `iBaseCellNumberY` (Xbox PDB).
        0x30 iBaseCellNumberY: i32,
    }

    /// `WaterZone` (Xbox PDB), 0x20 bytes: a `ZoneEntry` (vtable, then a
    /// `bhkTTNode` at +4 with its next pointer at +8) and the water data.
    pub struct WaterZone: 0x20 {
        /// `ZoneEntry::pTargets` (Xbox PDB): first `WaterTargetEntry`.
        0x14 pTargets: u32,
        /// `fWaterHeight` (Xbox PDB).
        0x18 fWaterHeight: f32,
        /// `pGroup` (Xbox PDB).
        0x1c pGroup: u32,
    }

    /// `WaterTargetEntry` (Xbox PDB), 0x60 bytes: a `TargetEntry` (its
    /// `bhkTTNode` at +0x10) and the collidables touching the water.
    pub struct WaterTargetEntry: 0x60 {
        /// `CollidableList` (Xbox PDB):
        /// `BSSimpleArray<hkpCollidable const *,1024>`.
        0x50 CollidableList: u32,
    }

    /// `QueuedRemoval` (Xbox PDB), 0x14 bytes, built by
    /// `contactPointRemovedCallback` (`0062f900`).
    pub struct QueuedRemoval: 0x14 {
        /// `pTarget` (Xbox PDB).
        0x00 pTarget: u32,
        /// `pRB` (Xbox PDB).
        0x04 pRB: u32,
        /// `pWaterRef` (Xbox PDB).
        0x08 pWaterRef: u32,
        /// `pWZone` (Xbox PDB).
        0x0c pWZone: u32,
        /// `pCollidable` (Xbox PDB).
        0x10 pCollidable: u32,
    }
}

/// Offset of `ZoneEntry`'s `BGSZoneTargetListener` base in the listener.
const ZONE_LISTENER: u32 = 0x14;
/// Offset of `QueuedRemovals` (`NiTList<QueuedRemoval *>`, Xbox PDB).
const QUEUED_REMOVALS: u32 = 0x34;
/// Offset of `CollidableList` in a `WaterTargetEntry`.
const COLLIDABLES: u32 = 0x50;
/// Offset of `TargetEntry`'s `bhkTTNode` in a target entry.
const TARGET_NODE: u32 = 0x10;

/// `TESWaterListener` vtable (RTTI `.?AVTESWaterListener@@`).
const LISTENER_VTABLE: u32 = 0x0104_e450;
/// The vtable of its `BGSZoneTargetListener` part at +0x14 (same RTTI).
const ZONE_LISTENER_VTABLE: u32 = 0x0104_e440;
/// `WaterTargetEntry` vtable (RTTI `.?AVWaterTargetEntry@@`).
const TARGET_ENTRY_VTABLE: u32 = 0x0104_e438;
/// `BSSimpleArray<hkpCollidable const *,1024>` vtable (RTTI).
const COLLIDABLE_ARRAY_VTABLE: u32 = 0x0104_e4bc;
/// `NiTMap<hkpRigidBody *,bool>` vtable (RTTI).
const BODY_MAP_VTABLE: u32 = 0x0104_e4f0;
/// `NiTMapBase<DFALL<NiTMapItem<hkpRigidBody *,bool>>,hkpRigidBody *,bool>`
/// vtable (RTTI).
const BODY_MAP_BASE_VTABLE: u32 = 0x0104_e510;

/// The `TES` pointer (other units read it so).
const TES_POINTER: u32 = 0x011d_ea10;
/// The global reference other units read as the player character.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The listener `0062f3d0` creates on first use.
const LISTENER_GLOBAL_011CC54C: u32 = 0x011c_c54c;
/// The `NiTMap<hkpRigidBody *,bool>` global this unit fills and empties
/// (the Xbox PDB's static `RigidBodyUnderwaterMap` has that type; that it
/// is this global is not confirmed).
const BODY_MAP_GLOBAL_011CC584: u32 = 0x011c_c584;
/// The object whose byte (`00408d60` gives its address) stops the entity
/// scan of `Update` and skips the cast checks.
const FLAG_HOLDER_011CC5A8: u32 = 0x011c_c5a8;
/// Set by `0062f310` (a water form, from `GetWaterType` or a virtual call).
const WATER_GLOBAL_011C7A3C: u32 = 0x011c_7a3c;
/// Set by `0062f8e0` (one byte).
const FLAG_GLOBAL_011C7A5A: u32 = 0x011c_7a5a;
/// Set by `0062f8f0` (a water height).
const HEIGHT_GLOBAL_011C7A5C: u32 = 0x011c_7a5c;
/// The setting `0043d4d0` reads for `uiCellArrayWidth`.
const CELL_ARRAY_WIDTH_SETTING: u32 = 0x011c_63cc;

/// `0.0` (double).
const ZERO: u32 = 0x0101_2060;
/// `0.001` (double): the least speed that splashes.
const SPLASH_SPEED_FLOOR: u32 = 0x0101_e788;
/// `0.01f`: the ripple size.
const RIPPLE_SIZE: u32 = 0x0101_3ea4;
/// `-FLT_MAX` (float): no water.
const NO_WATER: u32 = 0x0101_5f5c;
/// `-FLT_MAX` (double).
const NO_WATER_DOUBLE: u32 = 0x0102_41b0;
/// `0.25` (double).
const QUARTER: u32 = 0x0102_90b0;
/// `0.85f` widened (double).
const EIGHTY_FIVE_HUNDREDTHS: u32 = 0x0101_9de8;
/// `1.5` (double).
const ONE_AND_A_HALF: u32 = 0x0101_6ff0;
/// `0.1f` widened (double).
const ONE_TENTH: u32 = 0x0101_ffa0;
/// `-1.0` (double).
const MINUS_ONE: u32 = 0x0101_a6b0;
/// `-1.0f`.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `2.0f`.
const DEPTH_LIMIT_0119AF38: u32 = 0x0119_af38;
/// `-0.3f`.
const BUOYANCY_0119AF3C: u32 = 0x0119_af3c;
/// `0.5f`.
const RATIO_0119AF40: u32 = 0x0119_af40;
/// `0.3f`.
const BUOYANCY_0119AF44: u32 = 0x0119_af44;
/// `0.75f`.
const RATIO_0119AF48: u32 = 0x0119_af48;
/// `1.2f`.
const RATIO_0119AF4C: u32 = 0x0119_af4c;
/// A float in `.data` (0 in the file).
const RATIO_011CC5E0: u32 = 0x011c_c5e0;
/// A float in `.data` (0 in the file).
const BUOYANCY_011CC5E4: u32 = 0x011c_c5e4;
/// The constant vector `00630bb0` starts from (16 bytes).
const VECTOR_010C71E0: u32 = 0x010c_71e0;

/// Thiscall getter on a setting (`011dxxxx`): pointer to its float.
const SETTING_VALUE_POINTER: u32 = 0x0040_3e20;
/// Thiscall getter on a setting: its value as a word.
const SETTING_VALUE_WORD: u32 = 0x0040_3df0;
/// Thiscall on a vector: pointer to its data (`006815c0`).
const VECTOR_DATA: u32 = 0x0068_15c0;
/// Thiscall `(vector, index)`: pointer to one component (`00560d30`).
const COMPONENT: u32 = 0x0056_0d30;
/// Thiscall `(vector, source)`: copies a vector (`004a3c90`).
const VECTOR_COPY: u32 = 0x004a_3c90;
/// Thiscall `(vector, value)`: a vector from one float (`005dbf20`).
const VECTOR_FROM_FLOAT: u32 = 0x005d_bf20;
/// Thiscall on a vector: its length (`004586d0`, result in `ST0`).
const VECTOR_LENGTH: u32 = 0x0045_86d0;
/// Thiscall `(vector, out, n)` (`00458700`).
const VECTOR_TERM: u32 = 0x0045_8700;
/// Thiscall `(vector, other)`: multiplies in place (`00627920`).
const VECTOR_MULTIPLY: u32 = 0x0062_7920;
/// Destructor-like thiscall that every temporary here ends with (`00483710`).
const RELEASE: u32 = 0x0048_3710;
/// `TESHavokUtilities::FindCollidableRef` (Xbox PDB), cdecl.
const FIND_COLLIDABLE_REF: u32 = 0x0062_b4e0;
/// Thiscall getter on a reference whose result goes to `TESObjectCELL`
/// methods (`008d6f30`).
const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::GetWaterType` (Xbox PDB).
const GET_WATER_TYPE: u32 = 0x0054_7770;
/// `TESObjectCELL::GetWaterHeight` (Xbox PDB, result in `ST0`).
const GET_WATER_HEIGHT: u32 = 0x0054_71e0;
/// No-argument test that picks the task-queue path (`008c7aa0`).
const USE_TASK_QUEUE: u32 = 0x008c_7aa0;
/// No-argument getter whose result the queue calls take (`004537b0`).
const TASK_QUEUE: u32 = 0x0045_37b0;
/// Thiscall on `TES` whose result the `TESWaterSystem` calls take.
const WATER_SYSTEM: u32 = 0x0070_ec90;
/// `TESWaterSystem::AddRipple` (Xbox PDB).
const ADD_RIPPLE: u32 = 0x004e_8510;
/// `TaskQueueInterface::QueueAddRipple` (Xbox PDB).
const QUEUE_ADD_RIPPLE: u32 = 0x0087_b760;
/// Task-queue call taking the splash strength and a position (`0087b700`).
const QUEUE_SPLASH: u32 = 0x0087_b700;
/// `TESWaterSystem::AddTESObjectToWaterGroup` (Xbox PDB).
const ADD_TO_WATER_GROUP: u32 = 0x004e_5c80;
/// `TESWaterSystem::RemoveTESObjectFromWaterGroup` (Xbox PDB).
const REMOVE_FROM_WATER_GROUP: u32 = 0x004e_5df0;
/// Task-queue form of the add (`0087b4a0`).
const QUEUE_ADD_TO_WATER_GROUP: u32 = 0x0087_b4a0;
/// Task-queue form of the removal (`0087b4f0`).
const QUEUE_REMOVE_FROM_WATER_GROUP: u32 = 0x0087_b4f0;
/// Thiscall getter on a reference whose result is cast (`007af430`; the
/// engine map's `BGSSaveFormBuffer::GetForm` is a folded name).
const REFERENCE_GETTER: u32 = 0x007a_f430;
/// `__RTDynamicCast` (CRT).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Source and target type descriptors of the casts here.
const CAST_SOURCE_TYPE: u32 = 0x0118_3108;
const CAST_TARGET_TYPE: u32 = 0x0118_9d70;
/// Thiscall test on the cast form (`00452440`).
const CAST_FORM_TEST: u32 = 0x0045_2440;
/// Thiscall on the flag holder: address of its byte (`00408d60`).
const FLAG_POINTER: u32 = 0x0040_8d60;
/// `BSSimpleArray` search `(array, &item, start, compare)` (`00719b20`).
const ARRAY_FIND: u32 = 0x0071_9b20;
/// The compare function the searches pass (`009a3830`).
const ARRAY_EQUAL: u32 = 0x009a_3830;
/// `BSSimpleArray` add `(array, &item)` (`007cb2e0`).
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// `BSSimpleArray` size getter (`0044ddc0`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Zone lookup of a target by key (`0057b730`, thiscall, one word).
const ZONE_FIND_TARGET: u32 = 0x0057_b730;
/// Zone's first target (`00825c00`).
const ZONE_FIRST_TARGET: u32 = 0x0082_5c00;
/// Next of a `bhkTTNode` (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Map `RemoveAll` (`00438af0`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// Map `GetAt(key, &value)` (`0057c850`).
const MAP_GET_AT: u32 = 0x0057_c850;
/// Thiscall `(reference, zone, flag)` (`0057b0d0`).
const REFERENCE_SET_ZONE: u32 = 0x0057_b0d0;
/// Thiscall `(reference, flag)` (`0057b240`).
const REFERENCE_SET_IN_WATER: u32 = 0x0057_b240;
/// Thiscall test on a reference (`0057b200`).
const REFERENCE_IN_WATER: u32 = 0x0057_b200;
/// Thiscall `(object, layer)` test (`005631e0`).
const OBJECT_HAS_LAYER: u32 = 0x0056_31e0;
/// Thiscall `(object, layer, flag)` (`00628f30`).
const OBJECT_SET_LAYER: u32 = 0x0062_8f30;
/// Cdecl one float, result in `ST0` (`004a3e90`).
const SCALE_FLOAT: u32 = 0x004a_3e90;
/// Cdecl one float, result in `ST0` (`004587d0`).
const CONVERT_FLOAT: u32 = 0x0045_87d0;
/// Cdecl: the object of a rigid body (`004b5a80`).
const BODY_OBJECT: u32 = 0x004b_5a80;
/// Thiscall on an entity: its collidable (`00460140` / `0043b560` users).
const COLLIDABLE_OF: u32 = 0x0043_b560;
/// `bhkEntityListener`-side holder getter (`0061f6c0`, another unit).
const OWN_COLLIDABLE_HOLDER: u32 = 0x0061_f6c0;
/// Operator delete (`00401030`).
const FREE: u32 = 0x0040_1030;
/// Operator new (`00401000`).
const ALLOCATE: u32 = 0x0040_1000;

/// Reads a float as `FLD dword` does (a signalling NaN comes out quiet).
fn fld(e: &Engine, addr: u32) -> f32 {
    x87_quiet(e.mem.f32(addr))
}

/// The float of a setting, through `00403e20`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE_POINTER, &args![setting]).u32();
    fld(e, value)
}

/// One component of a vector, through `00560d30`, as `FLD` reads it.
fn component(e: &mut Engine, vector: u32, index: u32) -> f32 {
    let at = e.call(COMPONENT, &args![vector, index]).u32();
    fld(e, at)
}

/// `TESHavokUtilities::FindCollidableRef` of a word.
fn find_ref(e: &mut Engine, collidable: u32) -> u32 {
    e.call(FIND_COLLIDABLE_REF, &args![collidable]).u32()
}

/// `FindCollidableRef` of the word at `address` (an event field).
fn find_ref_at(e: &mut Engine, address: u32) -> u32 {
    let collidable = e.mem.u32(address);
    find_ref(e, collidable)
}

/// The cast the contact callbacks make of the water reference, and whether
/// they stop because of it (flag byte clear and `00452440` true).
fn cast_water(e: &mut Engine, water: u32) -> (u32, bool) {
    let form = e.call(REFERENCE_GETTER, &args![water]).u32();
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32();
    let flag = e.call(FLAG_POINTER, &args![FLAG_HOLDER_011CC5A8]).u32();
    let stop = e.mem.u8(flag) == 0 && e.call(CAST_FORM_TEST, &args![cast]).bool();
    (cast, stop)
}

/// The ripple both splash paths add: `AddRipple` on the water system, or
/// `QueueAddRipple` with the position's address.
fn add_ripple(e: &mut Engine, position: u32) {
    let size = x87_quiet_bits(e.global::<u32>(RIPPLE_SIZE));
    if !e.call(USE_TASK_QUEUE, &args![]).bool() {
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        let tes = e.global::<u32>(TES_POINTER);
        let water_system = e.call(WATER_SYSTEM, &args![tes]).u32();
        e.call(ADD_RIPPLE, &args![water_system, x, y, z, size]);
    } else {
        let queue = e.call(TASK_QUEUE, &args![]).u32();
        e.call(QUEUE_ADD_RIPPLE, &args![queue, size, position]);
    }
}

/// The water-group change both contact paths make: through the task queue
/// when it is in use and present, else on the water system.
fn water_group_call(e: &mut Engine, queued: u32, direct: u32, water: u32, target: u32, value: u32) {
    if e.call(USE_TASK_QUEUE, &args![]).bool() && e.call(TASK_QUEUE, &args![]).u32() != 0 {
        let queue = e.call(TASK_QUEUE, &args![]).u32();
        e.call(queued, &args![queue, water, target, value]);
    } else {
        let tes = e.global::<u32>(TES_POINTER);
        let water_system = e.call(WATER_SYSTEM, &args![tes]).u32();
        e.call(direct, &args![water_system, water, target, value]);
    }
}

/// A 16-byte vector as `MOVAPS` copies it.
type Vec4 = [f32; 4];

fn load4(e: &Engine, addr: u32) -> Vec4 {
    [
        e.mem.f32(addr),
        e.mem.f32(addr + 4),
        e.mem.f32(addr + 8),
        e.mem.f32(addr + 12),
    ]
}

fn store4(e: &mut Engine, addr: u32, v: Vec4) {
    for (i, lane) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * i as u32, *lane);
    }
}

/// `MULPS a, b`.
fn mulps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// `ADDPS a, b`.
fn addps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

/// `SUBPS a, b`.
fn subps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// `SHUFPS v, v, imm` (both operands the same vector).
fn shufps(v: Vec4, imm: u8) -> Vec4 {
    let lane = |shift: u8| v[((imm >> shift) & 3) as usize];
    [lane(0), lane(2), lane(4), lane(6)]
}

// Translation incomplete: 0062e3d0 - FS:[0] exception frame (unwind of the new target) not translated.
// Translated from 0062e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterZone::AddTarget` (Xbox PDB): finds the zone's target for `key`
/// (`0057b730`) and adds `collidable` to its collidable list when it is not
/// there yet; without a target, makes a `WaterTargetEntry` (0x60 bytes,
/// `00553300`), links it at the head of `pTargets` and passes `data` to
/// `00620760`. Returns the target. The exe passes the address of its own
/// `collidable` argument to the array calls. open: the exception frame.
pub fn water_zone_add_target(
    e: &mut Engine,
    this: Ptr<WaterZone>,
    key: u32,
    collidable: u32,
    data: u32,
) -> u32 {
    let found = if e.get(this, WaterZone::pTargets) == 0 {
        0
    } else {
        e.call(ZONE_FIND_TARGET, &args![this, key]).u32()
    };
    if found != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), collidable);
            let list = found + COLLIDABLES;
            let index = e
                .call(ARRAY_FIND, &args![list, slot, 0u32, ARRAY_EQUAL])
                .i32();
            if index == -1 {
                e.call(ARRAY_ADD, &args![list, slot]);
            }
        });
        return found;
    }
    let memory = e.call(0x0055_3300, &args![0x60u32]).u32();
    let target = if memory == 0 {
        0
    } else {
        water_target_entry_water_target_entry(e, Ptr::new(memory), key, collidable).addr()
    };
    let node = target.wrapping_add(TARGET_NODE);
    e.call(0x0062_0760, &args![node, data, 1u32]);
    let head = e.get(this, WaterZone::pTargets);
    let next = if head == 0 { 0 } else { head + TARGET_NODE };
    e.call(0x006e_cd40, &args![node, next]);
    e.set(this, WaterZone::pTargets, target);
    target
}

// Translated from 0062e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `collidable` from the zone's target for `key` (`009a4320` at the
/// index the search finds); when the target's list is then empty, removes
/// the target from the zone (`0061ff30`).
pub fn fn_0062e500(e: &mut Engine, this: Ptr<WaterZone>, key: u32, collidable: u32) {
    let target = if e.get(this, WaterZone::pTargets) == 0 {
        0
    } else {
        e.call(ZONE_FIND_TARGET, &args![this, key]).u32()
    };
    if target == 0 {
        return;
    }
    let list = target + COLLIDABLES;
    let index = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), collidable);
        e.call(ARRAY_FIND, &args![list, slot, 0u32, ARRAY_EQUAL])
            .i32()
    });
    if index != -1 {
        e.call(0x009a_4320, &args![list, index, 1u32]);
    }
    if e.call(ARRAY_SIZE, &args![list]).u32() == 0 {
        e.call(0x0061_ff30, &args![this, target]);
    }
}

// Translation incomplete: 0062e590 - FS:[0] exception frame (unwind of the collidable list) not translated.
// Translated from 0062e590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterTargetEntry::WaterTargetEntry` (Xbox PDB): the `TargetEntry` base
/// (`0061fba0` with `key`, 0), the vtable, an empty collidable list, then
/// `collidable` added to it unless already there. open: the exception frame.
pub fn water_target_entry_water_target_entry(
    e: &mut Engine,
    this: Ptr<WaterTargetEntry>,
    key: u32,
    collidable: u32,
) -> Ptr<WaterTargetEntry> {
    e.call(0x0061_fba0, &args![this, key, 0u32]);
    e.mem.set_u32(this.addr(), TARGET_ENTRY_VTABLE);
    let list = this.addr() + COLLIDABLES;
    fn_00631750(e, Ptr::new(list));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), collidable);
        let index = e
            .call(ARRAY_FIND, &args![list, slot, 0u32, ARRAY_EQUAL])
            .i32();
        if index == -1 {
            e.call(ARRAY_ADD, &args![list, slot]);
        }
    });
    this
}

// Translated from 0062e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener` scalar deleting destructor (vtable slot 0): the
/// destructor body (`0062e860`, another unit), then frees with bit 0.
pub fn tes_water_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    flags: u32,
) -> Ptr<TESWaterListener> {
    e.call(0x0062_e860, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translation incomplete: 0062e780 - FS:[0] exception frame (unwind of the bases) not translated.
// Translated from 0062e780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener` constructor: bases (`00cb75d0`, `BGSZoneTargetListener`
/// `0061ffb0` at +0x14), vtables, `fSplashTimer` 1, the removal list
/// (`0048f200` at +0x34), `uiCellArrayWidth` from the setting at `011c63cc`
/// and a level table of width squared floats (`MUL` by 4 saturating to
/// 0xffffffff on overflow, as the compiler's `new[]` does). open: the
/// exception frame.
pub fn tes_water_listener_tes_water_listener(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
) -> Ptr<TESWaterListener> {
    e.call(0x00cb_75d0, &args![this]);
    e.call(0x0061_ffb0, &args![this.addr() + ZONE_LISTENER]);
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    e.mem
        .set_u32(this.addr() + ZONE_LISTENER, ZONE_LISTENER_VTABLE);
    e.set(this, TESWaterListener::fSplashTimer, 1.0);
    e.call(0x0048_f200, &args![this.addr() + QUEUED_REMOVALS]);
    let width = e.call(0x0043_d4d0, &args![CELL_ARRAY_WIDTH_SETTING]).u32();
    let width = e.mem.u32(width);
    e.set(this, TESWaterListener::uiCellArrayWidth, width);
    let width = e.get(this, TESWaterListener::uiCellArrayWidth);
    let cells = width.wrapping_mul(width);
    let bytes = u32::try_from(u64::from(cells) * 4).unwrap_or(u32::MAX);
    let levels = e.call(ALLOCATE, &args![bytes]).u32();
    e.set(this, TESWaterListener::pfWaterLevels, levels);
    this
}

// Translated from 0062e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splash of `object` entering water at `water_height`: when the splash
/// timer has run out (not above 0), the object's model type (`0043b4d0`)
/// is not 8 and its speed (`00408820` of its velocity's z) is at least
/// 0.001, computes the strength (setting `011d00a8` times speed plus
/// `004b5400`, times setting `011d0ab4` when the z velocity is positive) and,
/// unless `TES` reports a surface (`005f36f0` zero, `004572e0` true) below the
/// water height, plays the splash (`PlaySplashEffects` or `0087b700`), adds a
/// ripple and restarts the timer from setting `011d135c`.
pub fn fn_0062e970(e: &mut Engine, this: Ptr<TESWaterListener>, object: u32, water_height: f32) {
    if object == 0 {
        return;
    }
    let timer = fld(e, this.addr() + 0x1c);
    if f64::from(timer) > e.global::<f64>(ZERO) {
        return;
    }
    e.with_stack(0x30, |e, frame| {
        let base = frame.addr() + 0x30;
        let at = move |offset: u32| base - offset;
        let model = e.call(0x0043_b4f0, &args![object, at(0x2c)]).u32();
        if e.call(0x0043_b4d0, &args![model]).u32() == 8 {
            return;
        }
        let mass = e.call(0x004b_5400, &args![object]).f32();
        e.call(VECTOR_DATA, &args![at(0x28)]);
        e.call(0x0056_0d50, &args![object, at(0x28)]);
        let velocity_z = fld(e, at(0x20));
        let speed = e.call(0x0040_8820, &args![velocity_z.to_bits()]).f32();
        if f64::from(speed) < e.global::<f64>(SPLASH_SPEED_FLOOR) {
            return;
        }
        let factor = setting_float(e, 0x011d_00a8);
        let mut strength = (f64::from(factor) * f64::from(speed) + f64::from(mass)) as f32;
        if f64::from(fld(e, at(0x20))) > e.global::<f64>(ZERO) {
            let scale = setting_float(e, 0x011d_0ab4);
            strength = (f64::from(strength) * f64::from(scale)) as f32;
        }
        let position = at(0x14);
        e.call(VECTOR_DATA, &args![position]);
        e.call(0x0056_0bc0, &args![object, position]);
        e.mem.set_f32(at(4), x87_quiet(water_height));
        let tes = e.global::<u32>(TES_POINTER);
        let mut splash = e.call(0x005f_36f0, &args![tes]).u32() != 0;
        if !splash {
            let tes = e.global::<u32>(TES_POINTER);
            if !e.call(0x0045_72e0, &args![tes, position, at(4)]).bool() {
                splash = true;
            } else {
                let surface = fld(e, at(4));
                splash = !(f64::from(x87_quiet(water_height)) < f64::from(surface));
            }
        }
        if !splash {
            return;
        }
        e.mem.set_f32(position + 8, x87_quiet(water_height));
        if !e.call(USE_TASK_QUEUE, &args![]).bool() {
            let x = e.mem.f32(position);
            let y = e.mem.f32(position + 4);
            let z = e.mem.f32(position + 8);
            tes_water_listener_play_splash_effects(e, strength, x, y, z);
        } else {
            let queue = e.call(TASK_QUEUE, &args![]).u32();
            e.call(QUEUE_SPLASH, &args![queue, strength.to_bits(), position]);
        }
        add_ripple(e, position);
        let timer = setting_float(e, 0x011d_135c);
        e.set(this, TESWaterListener::fSplashTimer, timer);
    });
}

/// The three splash sounds: speed threshold setting, sound name
/// (`CWaterLarge`, `CWaterMedium`, `CWaterSmall`), the temporary handle's
/// frame offset and the volume setting.
const SPLASH_SOUNDS: [(u32, u32, u32, u32); 3] = [
    (0x011d_0c78, 0x0104_e4a0, 0x4c, 0x011d_1458),
    (0x011d_07f0, 0x0104_e490, 0x58, 0x011d_012c),
    (0x011d_0310, 0x0104_e484, 0x64, 0x011d_091c),
];

// Translation incomplete: 0062eb50 - FS:[0] exception frame (unwind of the sound handles) not translated.
// Translated from 0062eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener::PlaySplashEffects` (Xbox PDB), cdecl `(strength,
/// position)`: takes the first of the large, medium and small splash sounds
/// whose threshold setting is below `strength` (`BSAudio::GetSoundHandleByName`
/// with flags 0x102) and its volume setting; plays it at the position; with a
/// positive volume and a player cell whose water type is not dangerous,
/// starts the splash effect (`00689210`, a rotation from `004a4240`); then
/// adds a ripple. open: the exception frame.
pub fn tes_water_listener_play_splash_effects(
    e: &mut Engine,
    strength: f32,
    x: f32,
    y: f32,
    z: f32,
) {
    e.with_stack(0x70, |e, frame| {
        let base = frame.addr() + 0x70;
        let at = move |offset: u32| base - offset;
        let handle = at(0x18);
        let mut volume = 1.0f32;
        e.call(0x0041_a250, &args![handle]);
        for (index, &(threshold, name, temporary, volume_setting)) in
            SPLASH_SOUNDS.iter().enumerate()
        {
            if index > 0 && e.call(0x00ad_8ce0, &args![handle]).bool() {
                continue;
            }
            let limit = setting_float(e, threshold);
            if !(f64::from(limit) < f64::from(x87_quiet(strength))) {
                continue;
            }
            let audio = e.call(0x0045_3a70, &args![]).u32();
            let sound = e
                .call(0x00ad_7550, &args![audio, at(temporary), name, 0x102u32])
                .u32();
            e.call(0x0041_8900, &args![handle, sound]);
            e.call(RELEASE, &args![at(temporary)]);
            volume = setting_float(e, volume_setting);
        }
        if e.call(0x00ad_8ce0, &args![handle]).bool() {
            e.call(
                0x00ad_8b60,
                &args![
                    handle,
                    x87_quiet_bits(x.to_bits()),
                    x87_quiet_bits(y.to_bits()),
                    x87_quiet_bits(z.to_bits())
                ],
            );
            e.call(0x00ad_8830, &args![handle, 0u32]);
            if f64::from(x87_quiet(volume)) > e.global::<f64>(ZERO) {
                splash_effect(e, at(0x40), volume, x, y, z);
            }
        }
        e.with_stack(12, |e, position| {
            e.mem.set_f32(position.addr(), x);
            e.mem.set_f32(position.addr() + 4, y);
            e.mem.set_f32(position.addr() + 8, z);
            add_ripple(e, position.addr());
        });
        e.call(RELEASE, &args![handle]);
    });
}

/// The particle part of `PlaySplashEffects`: the player's cell must have a
/// water type that is not dangerous.
fn splash_effect(e: &mut Engine, rotation: u32, volume: f32, x: f32, y: f32, z: f32) {
    let player = e.global::<u32>(PLAYER_POINTER);
    if e.call(REFERENCE_CELL, &args![player]).u32() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    let cell = e.call(REFERENCE_CELL, &args![player]).u32();
    if e.call(GET_WATER_TYPE, &args![cell]).u32() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    let cell = e.call(REFERENCE_CELL, &args![player]).u32();
    let water = e.call(GET_WATER_TYPE, &args![cell]).u32();
    if e.call(0x0058_0060, &args![water]).bool() {
        return;
    }
    e.call(VECTOR_DATA, &args![rotation]);
    let angle = e.call(0x004a_4240, &args![]).f32();
    e.call(0x004a_0c90, &args![rotation, angle.to_bits()]);
    let matrix: Vec<u32> = (0..9).map(|i| e.mem.u32(rotation + 4 * i)).collect();
    let setting = e.call(SETTING_VALUE_WORD, &args![0x011d_12c0u32]).u32();
    let player = e.global::<u32>(PLAYER_POINTER);
    let cell = e.call(REFERENCE_CELL, &args![player]).u32();
    let mut words = vec![cell, 1.0f32.to_bits(), setting];
    words.extend_from_slice(&matrix);
    words.extend_from_slice(&[
        x.to_bits(),
        y.to_bits(),
        z.to_bits(),
        x87_quiet(volume).to_bits(),
        1,
        0,
    ]);
    e.call(0x0068_9210, &words);
}

// Translation incomplete: 0062ee60 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 0062ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener::Update` (Xbox PDB, vtable slot 0x18): when enabled,
/// counts the splash timer down by `delta`, empties the body map at
/// `011cc584`, runs every listed entity's rigid body through the water test
/// (`0062ff10`) and buoyancy (`00630170`) with the cell water levels, then
/// every zone target's collidables with the zone's height, updating the
/// references' water state and water groups; finally `00631370`. In every
/// case, sets the water global when the player's head is below its cell's
/// water. open: the /GS cookie.
pub fn tes_water_listener_update(e: &mut Engine, this: Ptr<TESWaterListener>, delta: f32) {
    if fn_0062f2f0(e, this) {
        e.with_stack(0x80, |e, frame| {
            let base = frame.addr() + 0x80;
            let at = move |offset: u32| base - offset;
            update_bodies(e, this, delta, at(0x20), at(0x38), at(0x55));
        });
    }
    let actor = e.call(0x0052_4c90, &args![]).u32();
    if actor == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    if e.call(REFERENCE_CELL, &args![player]).u32() == 0 {
        return;
    }
    let actor = e.call(0x0052_4c90, &args![]).u32();
    let point = e.call(0x0045_bb80, &args![actor]).u32();
    let player = e.global::<u32>(PLAYER_POINTER);
    let cell = e.call(REFERENCE_CELL, &args![player]).u32();
    let height = e.call(GET_WATER_HEIGHT, &args![cell]).f64();
    if f64::from(fld(e, point + 8)) < height {
        let player = e.global::<u32>(PLAYER_POINTER);
        let cell = e.call(REFERENCE_CELL, &args![player]).u32();
        let water = e.call(GET_WATER_TYPE, &args![cell]).u32();
        fn_0062f310(e, water);
    }
}

/// The enabled part of `Update`; `point` is its vector at [EBP-0x20],
/// `level` its float at [EBP-0x38], `found` its byte at [EBP-0x55].
fn update_bodies(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    delta: f32,
    point: u32,
    level: u32,
    found: u32,
) {
    let timer = f64::from(fld(e, this.addr() + 0x1c)) - f64::from(x87_quiet(delta));
    e.set(this, TESWaterListener::fSplashTimer, timer as f32);
    let range = setting_float(e, 0x011d_05d4);
    let range = (f64::from(range) * f64::from(range)) as f32;
    let range = e.call(SCALE_FLOAT, &args![range.to_bits()]).f32();
    let margin = setting_float(e, 0x011d_1248);
    e.call(SCALE_FLOAT, &args![margin.to_bits()]);
    e.call(VECTOR_DATA, &args![point]);
    let player = e.global::<u32>(PLAYER_POINTER);
    let head = e.vcall(player, 0x1f4, &args![]).u32();
    e.call(0x004a_3e00, &args![point, head]);
    let scratch = point - 0x10;
    e.call(VECTOR_DATA, &args![scratch]);
    let mut node = this.addr() + 0xc;
    e.mem.set_f32(level, 0.0);
    e.call(MAP_REMOVE_ALL, &args![BODY_MAP_GLOBAL_011CC584]);
    loop {
        let flag = e.call(FLAG_POINTER, &args![FLAG_HOLDER_011CC5A8]).u32();
        if e.mem.u8(flag) != 0 || node == 0 {
            break;
        }
        if e.call(0x004a_4460, &args![node]).bool() {
            break;
        }
        let item = e.call(VECTOR_DATA, &args![node]).u32();
        let entity = e.call(0x0055_9450, &args![item]).u32();
        let body = e.call(0x0062_0b80, &args![entity]).u32();
        let mut status = 0;
        if fn_0062ff10(e, this, body, point, range, 0, level) {
            let height = fld(e, level);
            status = fn_00630170(e, this, body, height, delta, false);
        }
        if status != 0 {
            let collidable = e.call(COLLIDABLE_OF, &args![entity]).u32();
            let reference = find_ref(e, collidable);
            if reference != 0 {
                let height = fld(e, level);
                let height = e.call(CONVERT_FLOAT, &args![height.to_bits()]).f32();
                e.mem.set_f32(level, height);
                if status == 1 {
                    if !e.call(REFERENCE_IN_WATER, &args![reference]).bool() {
                        water_group_call(
                            e,
                            QUEUE_ADD_TO_WATER_GROUP,
                            ADD_TO_WATER_GROUP,
                            0,
                            reference,
                            fld(e, level).to_bits(),
                        );
                    }
                    e.call(REFERENCE_SET_IN_WATER, &args![reference, 1u32]);
                } else if status == 2 {
                    e.call(REFERENCE_SET_IN_WATER, &args![reference, 0u32]);
                    if !e.call(REFERENCE_IN_WATER, &args![reference]).bool() {
                        water_group_call(
                            e,
                            QUEUE_REMOVE_FROM_WATER_GROUP,
                            REMOVE_FROM_WATER_GROUP,
                            0,
                            reference,
                            fld(e, level).to_bits(),
                        );
                    }
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let mut zone = e.get(this, TESWaterListener::pZones);
    while zone != 0 {
        let mut target = e.call(ZONE_FIRST_TARGET, &args![zone]).u32();
        while target != 0 {
            e.mem.set_u8(found, 0);
            let list = target + COLLIDABLES;
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE, &args![list]).u32() {
                let slot = e.call(0x006a_7ad0, &args![list, index]).u32();
                let collidable = e.mem.u32(slot);
                let body = e.call(0x004b_59f0, &args![collidable]).u32();
                let mut status = 0;
                if !e
                    .call(MAP_GET_AT, &args![BODY_MAP_GLOBAL_011CC584, body, found])
                    .bool()
                    && fn_0062ff10(e, this, body, point, range, zone, level)
                {
                    let height = fld(e, level);
                    status = fn_00630170(e, this, body, height, delta, true);
                }
                if status == 1 {
                    let reference = e.call(0x0084_e3a0, &args![target + TARGET_NODE]).u32();
                    e.call(REFERENCE_SET_IN_WATER, &args![reference, 1u32]);
                    let reference = e.call(0x0084_e3a0, &args![target + TARGET_NODE]).u32();
                    e.call(REFERENCE_SET_ZONE, &args![reference, zone, 1u32]);
                }
                index += 1;
            }
            let next = e.call(NODE_NEXT, &args![target + TARGET_NODE]).u32();
            target = if next == 0 { 0 } else { next - TARGET_NODE };
        }
        let next = e.call(NODE_NEXT, &args![zone + 4]).u32();
        zone = if next == 0 { 0 } else { next - 4 };
    }
    e.call(0x0063_1370, &args![this, 0u32, 1u32, 0u32]);
}

// Translated from 0062f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bEnabled` of the listener.
pub fn fn_0062f2f0(e: &mut Engine, this: Ptr<TESWaterListener>) -> bool {
    e.get(this, TESWaterListener::bEnabled)
}

// Translated from 0062f310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word in the global at `011c7a3c` (cdecl).
pub fn fn_0062f310(e: &mut Engine, value: u32) {
    e.set_global(WATER_GLOBAL_011C7A3C, value);
}

// Translated from 0062f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot 0x24 (the Xbox PDB's `bhkEntityListener::GetRB` slot): the
/// object of `entity` from the map at `012681c0` (`00653270`), unless its
/// state (`00517630`) is 5 or its model type is not one the compiler's switch accepts (the
/// types 4, 5, 6, 8, 10, 12, 13, 14 and 29); else 0.
pub fn fn_0062f320(e: &mut Engine, _this: Ptr<TESWaterListener>, entity: u32) -> u32 {
    let body = e.call(BODY_OBJECT, &args![entity]).u32();
    let object = e.call(0x0065_3270, &args![0x0126_81c0u32, body]).u32();
    if e.call(0x0051_7630, &args![object]).u32() == 5 || object == 0 {
        return 0;
    }
    e.with_stack(4, |e, local| {
        let model = e.call(0x0043_b4f0, &args![object, local]).u32();
        let kind = e.call(0x0043_b4d0, &args![model]).u32();
        if matches!(kind, 4..=6 | 8 | 10 | 12..=14 | 29) {
            object
        } else {
            0
        }
    })
}

// Translation incomplete: 0062f3d0 - FS:[0] exception frame (unwind of the allocation) not translated.
// Translated from 0062f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener at `011cc54c`, created (0x40 bytes, constructor `0062e780`)
/// on first use. open: the exception frame.
pub fn fn_0062f3d0(e: &mut Engine) -> u32 {
    if e.global::<u32>(LISTENER_GLOBAL_011CC54C) == 0 {
        let memory = e.call(ALLOCATE, &args![0x40u32]).u32();
        let listener = if memory == 0 {
            0
        } else {
            tes_water_listener_tes_water_listener(e, Ptr::new(memory)).addr()
        };
        e.set_global(LISTENER_GLOBAL_011CC54C, listener);
    }
    e.global(LISTENER_GLOBAL_011CC54C)
}

// Translated from 0062f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the listener's water levels: for every cell of the `TES` grid
/// (`00457050`), its water height when `004518e0` says it has water, else
/// `-FLT_MAX`, stored through `0062e710` and kept as the highest; then the
/// base cell numbers from cell (0, 0); and when `004036b0` is 0 and no cell
/// has water, `00937090(listener, 0)`.
pub fn fn_0062f460(e: &mut Engine) {
    let listener = fn_0062f3d0(e);
    let this: Ptr<TESWaterListener> = Ptr::new(listener);
    let no_water = fld(e, NO_WATER);
    e.set(this, TESWaterListener::fHighestWaterLevel, no_water);
    let rows = e.get(this, TESWaterListener::uiCellArrayWidth) as i32;
    let columns = e.get(this, TESWaterListener::uiCellArrayWidth) as i32;
    let mut row = 0i32;
    while row < rows {
        let mut column = 0i32;
        while column < columns {
            let mut level = fld(e, NO_WATER);
            let tes = e.global::<u32>(TES_POINTER);
            let slot = e.call(0x0045_7050, &args![tes, row, column]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
                level = e.call(GET_WATER_HEIGHT, &args![cell]).f32();
            }
            e.call(0x0062_e710, &args![this, level.to_bits(), row, column]);
            let highest = fld(e, listener + 0x24);
            if f64::from(level) > f64::from(highest) {
                e.set(this, TESWaterListener::fHighestWaterLevel, x87_quiet(level));
            }
            column += 1;
        }
        row += 1;
    }
    let tes = e.global::<u32>(TES_POINTER);
    let slot = e.call(0x0045_7050, &args![tes, 0u32, 0u32]).u32();
    let cell = e.mem.u32(slot);
    if cell != 0 {
        let x = e.call(0x0054_4c30, &args![cell]).i32();
        e.set(this, TESWaterListener::iBaseCellNumberX, x);
        let y = e.call(0x0054_4c60, &args![cell]).i32();
        e.set(this, TESWaterListener::iBaseCellNumberY, y);
    }
    let tes = e.global::<u32>(TES_POINTER);
    if e.call(0x0040_36b0, &args![tes]).u32() == 0 {
        let highest = fld(e, listener + 0x24);
        if f64::from(highest) == e.global::<f64>(NO_WATER_DOUBLE) {
            e.call(0x0093_7090, &args![this, 0u32]);
        }
    }
}

// Translation incomplete: 0062f5b0 - FS:[0] exception frame (unwind of the entity pointer) not translated.
// Translated from 0062f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener::collidableEnterCallback` (vtable slot 0x2c): wraps the
/// entity of `collidable` (`004b59f0`) in a pointer (`00633c90`), passes it
/// to `00631540` on the entity list at +0xc and releases it (`0045cec0`).
/// Returns false. The second word (`RET 8`) is never read. open: the
/// exception frame.
pub fn fn_0062f5b0(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    collidable: u32,
    _unused_2: u32,
) -> bool {
    entity_list_call(e, this, collidable, 0x0063_1540);
    false
}

// Translation incomplete: 0062f630 - FS:[0] exception frame (unwind of the entity pointer) not translated.
// Translated from 0062f630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener::collidableLeaveCallback` (vtable slot 0x30): as
/// `0062f5b0` with `00631620` (`BSSimpleList<NiPointer<..>>::Remove` in the
/// engine map). Returns true. The second word is never read. open: the
/// exception frame.
pub fn fn_0062f630(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    collidable: u32,
    _unused_2: u32,
) -> bool {
    entity_list_call(e, this, collidable, 0x0063_1620);
    true
}

/// The shared body of the collidable callbacks.
fn entity_list_call(e: &mut Engine, this: Ptr<TESWaterListener>, collidable: u32, list_call: u32) {
    let entity = e.call(0x004b_59f0, &args![collidable]).u32();
    e.with_stack(4, |e, pointer| {
        e.call(0x0063_3c90, &args![pointer, entity]);
        e.call(list_call, &args![this.addr() + 0xc, pointer]);
        e.call(0x0045_cec0, &args![pointer]);
    });
}

// Translated from 0062f6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener::contactPointAddedCallback` (Xbox PDB, vtable slot
/// 0x1c): for a contact of a reference (event +0x48) with water (event
/// +0x20): when the reference is not found but the collidable is the
/// player's own, records the water height (`0062f8f0`), `00575c90(1)`,
/// `0062f8e0(1)` and the water's form (`0062f310`); else adds the reference
/// to the zone (`00620340`), its water group and `00631370`. False when no
/// water reference is found or the cast check stops it.
pub fn tes_water_listener_contact_point_added_callback(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    event: u32,
) -> bool {
    let target = find_ref_at(e, event + 0x48);
    if target == 0 {
        let player = e.global::<u32>(PLAYER_POINTER);
        let holder = e.call(OWN_COLLIDABLE_HOLDER, &args![player]).u32();
        let own = e.call(COLLIDABLE_OF, &args![holder]).u32();
        if e.mem.u32(event + 0x48) != own {
            return false;
        }
        let water = find_ref_at(e, event + 0x20);
        if water == 0 {
            return false;
        }
        let (cast, stop) = cast_water(e, water);
        if stop {
            return false;
        }
        let bound = e.vcall(water, 0x1f4, &args![]).u32();
        let height = fld(e, bound + 8);
        fn_0062f8f0(e, height);
        e.call(0x0057_5c90, &args![1u32]);
        fn_0062f8e0(e, true);
        let form = e.vcall(cast, 0x140, &args![]).u32();
        fn_0062f310(e, form);
        return true;
    }
    let water = find_ref_at(e, event + 0x20);
    if water == 0 {
        return false;
    }
    let (cast, stop) = cast_water(e, water);
    if stop {
        return false;
    }
    if target == e.global::<u32>(PLAYER_POINTER) {
        let form = e.vcall(cast, 0x140, &args![]).u32();
        fn_0062f310(e, form);
    }
    let zone = e
        .call(
            0x0062_0340,
            &args![
                this.addr() + ZONE_LISTENER,
                event,
                water,
                target,
                0u32,
                0u32
            ],
        )
        .u32();
    e.call(REFERENCE_SET_ZONE, &args![target, zone, 1u32]);
    water_group_call(
        e,
        QUEUE_ADD_TO_WATER_GROUP,
        ADD_TO_WATER_GROUP,
        water,
        target,
        0,
    );
    e.call(0x0063_1370, &args![this, target, 0u32, 1u32]);
    true
}

// Translated from 0062f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global at `011c7a5a` (cdecl).
pub fn fn_0062f8e0(e: &mut Engine, value: bool) {
    e.set_global(FLAG_GLOBAL_011C7A5A, value);
}

// Translated from 0062f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float in the global at `011c7a5c` (cdecl, `FLD`/`FSTP`).
pub fn fn_0062f8f0(e: &mut Engine, value: f32) {
    e.set_global(HEIGHT_GLOBAL_011C7A5C, x87_quiet(value));
}

// Translated from 0062f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterListener::contactPointRemovedCallback` (Xbox PDB, vtable slot
/// 0x20): for a contact ending between a reference (event +0x48) and water
/// (event +0x20): when the reference is not found, the player has a cell
/// and the collidable is the player's own, clears the player's water state
/// (`00575c90(0)`, the cell's water height, `0062f8e0(0)`, the water form);
/// else, unless `00440d80` says so, queues a `QueuedRemoval` (when the
/// reference's body chain gives a key not queued yet) or removes it at once
/// (`0062fc70`). False when nothing applies.
pub fn tes_water_listener_contact_point_removed_callback(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    event: u32,
) -> bool {
    let target = find_ref_at(e, event + 0x48);
    let water = find_ref_at(e, event + 0x20);
    if water == 0 {
        return false;
    }
    let (_, stop) = cast_water(e, water);
    if stop {
        return false;
    }
    let zones = e.get(this, TESWaterListener::pZones);
    let zone = if zones == 0 {
        0
    } else {
        e.call(0x0062_0300, &args![zones, e.mem.u32(event + 0x20)])
            .u32()
    };
    if zone != 0 {
        // Read into a local the function never uses again.
        e.mem.u32(zone + 0x1c);
    }
    if target == 0 {
        let player = e.global::<u32>(PLAYER_POINTER);
        if e.call(REFERENCE_CELL, &args![player]).u32() != 0 {
            let player = e.global::<u32>(PLAYER_POINTER);
            let holder = e.call(OWN_COLLIDABLE_HOLDER, &args![player]).u32();
            let own = e.call(COLLIDABLE_OF, &args![holder]).u32();
            if e.mem.u32(event + 0x48) != own {
                return false;
            }
            e.call(0x0057_5c90, &args![0u32]);
            let player = e.global::<u32>(PLAYER_POINTER);
            let cell = e.call(REFERENCE_CELL, &args![player]).u32();
            let height = e.call(GET_WATER_HEIGHT, &args![cell]).f32();
            fn_0062f8f0(e, height);
            fn_0062f8e0(e, false);
            let player = e.global::<u32>(PLAYER_POINTER);
            let extra = e.call(0x005d_43c0, &args![player]).u32();
            let zone_data = e.call(0x0042_f180, &args![extra]).u32();
            if zone_data != 0 {
                let item = e.call(0x0084_e3a0, &args![zone_data + 4]).u32();
                let form = e.call(REFERENCE_GETTER, &args![item]).u32();
                let water_form = e.vcall(form, 0x140, &args![]).u32();
                fn_0062f310(e, water_form);
            } else {
                let player = e.global::<u32>(PLAYER_POINTER);
                let cell = e.call(REFERENCE_CELL, &args![player]).u32();
                let water_form = e.call(GET_WATER_TYPE, &args![cell]).u32();
                fn_0062f310(e, water_form);
            }
            return true;
        }
        return false;
    }
    if e.call(0x0044_0d80, &args![target]).bool() {
        return false;
    }
    let mut key = 0;
    let model = e.vcall(target, 0x1d0, &args![]).u32();
    let node = e.call(0x0068_38b0, &args![model]).u32();
    if node != 0 {
        let entity = e.vcall(node, 0x38, &args![]).u32();
        if entity != 0 {
            let body = e.call(0x006f_a820, &args![entity]).u32();
            if body != 0 {
                key = e.vcall(body, 0x40, &args![]).u32();
            }
        }
    }
    let collidable = e.mem.u32(event + 0x48);
    if key == 0 {
        e.with_stack(0x14, |e, record| {
            let removal: Ptr<QueuedRemoval> = Ptr::new(record.addr());
            e.set(removal, QueuedRemoval::pTarget, target);
            e.set(removal, QueuedRemoval::pRB, key);
            e.set(removal, QueuedRemoval::pWaterRef, water);
            e.set(removal, QueuedRemoval::pWZone, zone);
            e.set(removal, QueuedRemoval::pCollidable, collidable);
            fn_0062fc70(e, this, removal);
        });
        return true;
    }
    let list = this.addr() + QUEUED_REMOVALS;
    let mut position = e.call(0x0055_9450, &args![list]).u32();
    let mut queued = false;
    while position != 0 {
        let slot = e.call(0x0063_17a0, &args![list, position]).u32();
        let item = e.mem.u32(slot);
        position = e.call(0x007b_52d0, &args![list, position]).u32();
        if e.mem.u32(item + 4) == key {
            queued = true;
            break;
        }
    }
    if !queued {
        let memory = e.call(ALLOCATE, &args![0x14u32]).u32();
        let record = if memory == 0 {
            0
        } else {
            e.call(0x00ec_61c0, &args![memory, 0u32, 0x14u32]);
            memory
        };
        let removal: Ptr<QueuedRemoval> = Ptr::new(record);
        e.set(removal, QueuedRemoval::pTarget, target);
        e.set(removal, QueuedRemoval::pRB, key);
        e.set(removal, QueuedRemoval::pWaterRef, water);
        e.set(removal, QueuedRemoval::pWZone, zone);
        let collidable = e.mem.u32(event + 0x48);
        e.set(removal, QueuedRemoval::pCollidable, collidable);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), record);
            e.call(0x0047_00d0, &args![list, local]);
        });
    }
    true
}

// Translated from 0062fc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Carries out a `QueuedRemoval`: takes the collidable off its zone's target
/// (`0062e500`), splashes the body out of the water (`0062e970`, then
/// `00628f30(body, 0x80, 0)`), clears the reference's zone and water state,
/// and removes it from its water group unless another water zone of the
/// reference (`ExtraDataList::QWaterZoneMap`) has the same group. A block
/// guarded by a local byte the function sets to 0 just before (the player's
/// water form) never runs and is left out.
pub fn fn_0062fc70(e: &mut Engine, this: Ptr<TESWaterListener>, removal: Ptr<QueuedRemoval>) {
    let zone = e.get(removal, QueuedRemoval::pWZone);
    let group = if zone == 0 { 0 } else { e.mem.u32(zone + 0x1c) };
    if zone != 0 {
        let target = e.get(removal, QueuedRemoval::pTarget);
        let collidable = e.get(removal, QueuedRemoval::pCollidable);
        fn_0062e500(e, Ptr::new(zone), target, collidable);
        e.call(ZONE_FIRST_TARGET, &args![zone]);
    }
    let body = e.get(removal, QueuedRemoval::pRB);
    if body != 0 {
        let water = e.get(removal, QueuedRemoval::pWaterRef);
        let bound = e.vcall(water, 0x1f4, &args![]).u32();
        let height = fld(e, bound + 8);
        fn_0062e970(e, this, body, height);
        e.call(OBJECT_SET_LAYER, &args![body, 0x80u32, 0u32]);
    }
    let target = e.get(removal, QueuedRemoval::pTarget);
    if zone != 0 {
        e.call(REFERENCE_SET_ZONE, &args![target, zone, 0u32]);
    }
    e.call(REFERENCE_SET_IN_WATER, &args![target, 0u32]);
    let mut remove = true;
    let extra = e.call(0x005d_43c0, &args![target]).u32();
    if e.call(0x0042_f180, &args![extra]).u32() == 0 {
        let extra = e.call(0x005d_43c0, &args![target]).u32();
        let map = e.call(0x0042_f1d0, &args![extra]).u32();
        if map != 0 {
            e.with_stack(12, |e, locals| {
                let position = locals.addr();
                let key = position + 4;
                let value = position + 8;
                let first = e.call(0x004b_9ba0, &args![map]).u32();
                e.mem.set_u32(position, first);
                while e.mem.u32(position) != 0 {
                    e.mem.set_u32(value, 0);
                    e.mem.set_u32(key, 0);
                    e.call(0x006b_7f20, &args![map, position, key, value]);
                    if group != 0 && e.mem.u32(e.mem.u32(key) + 0x1c) == group {
                        remove = false;
                        break;
                    }
                }
            });
        }
    }
    if remove {
        let water = e.get(removal, QueuedRemoval::pWaterRef);
        water_group_call(
            e,
            QUEUE_REMOVE_FROM_WATER_GROUP,
            REMOVE_FROM_WATER_GROUP,
            water,
            target,
            0,
        );
    }
}

// Translated from 0062fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00576830(holder + 0x14, reference)` for the holder `0062fec0` finds for
/// the reference, else 0 (cdecl).
pub fn fn_0062fe80(e: &mut Engine, reference: u32) -> u32 {
    let holder = fn_0062fec0(e, reference);
    if holder == 0 {
        return 0;
    }
    e.call(0x0057_6830, &args![holder + 0x14, reference]).u32()
}

// Translated from 0062fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The chain `008d6f30` (the reference's cell), `004543c0`, `0059bb30` from
/// a reference, 0 at the first null (cdecl).
pub fn fn_0062fec0(e: &mut Engine, reference: u32) -> u32 {
    if reference == 0 {
        return 0;
    }
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    if cell == 0 {
        return 0;
    }
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    if world == 0 {
        return 0;
    }
    e.call(0x0059_bb30, &args![world]).u32()
}

// Translation incomplete: 0062ff10 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 0062ff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Water test of a rigid body: out of range (the distance from `point`,
/// `004586d0`, above `range`) it deactivates the object when it has layer
/// 0x88 and is active (`00c9d600`) and returns false. In range, writes the
/// water level to `level` (the zone's height through `004a3e90`, or the cell
/// table at the body's cell, `00630120`) and returns whether the body's top
/// (`00458700` term 2) is not above level plus setting `011d1248`, else the
/// object's layer 0x88 test. False without a body or object. open: the /GS
/// cookie.
pub fn fn_0062ff10(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    body: u32,
    point: u32,
    range: f32,
    zone: u32,
    level: u32,
) -> bool {
    if body == 0 {
        return false;
    }
    let object = e.call(BODY_OBJECT, &args![body]).u32();
    if object == 0 {
        return false;
    }
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr() + 0x80;
        let at = move |offset: u32| base - offset;
        let source = e.call(0x004b_4ec0, &args![body]).u32();
        e.call(VECTOR_COPY, &args![at(0x20), source]);
        e.call(VECTOR_DATA, &args![at(0x30)]);
        e.call(0x0045_8a10, &args![at(0x30), at(0x20), point]);
        let difference = fn_00630100(e, at(0x30), at(0x40));
        let distance = e.call(VECTOR_LENGTH, &args![difference]).f64();
        if f64::from(x87_quiet(range)) < distance {
            if e.call(OBJECT_HAS_LAYER, &args![object, 0x88u32]).bool() {
                let active = e.call(0x00c9_bff0, &args![body, at(0x41)]).u32();
                if e.call(0x0056_09f0, &args![active]).bool() {
                    e.call(0x00c9_d600, &args![object]);
                }
            }
            return false;
        }
        let height = if zone != 0 {
            let height = fld(e, zone + 0x18);
            e.call(SCALE_FLOAT, &args![height.to_bits()]).f32()
        } else if e.get(this, TESWaterListener::uiCellArrayWidth) > 1 {
            e.call(VECTOR_DATA, &args![at(0x50)]);
            e.call(0x0045_8620, &args![at(0x50), at(0x20)]);
            let x = fld(e, at(0x50));
            let cell_x = e.call(0x0040_6d90, &args![x.to_bits()]).i32() >> 12;
            let y = fld(e, at(0x4c));
            let cell_y = e.call(0x0040_6d90, &args![y.to_bits()]).i32() >> 12;
            let cell_x = cell_x.wrapping_sub(e.get(this, TESWaterListener::iBaseCellNumberX));
            let cell_y = cell_y.wrapping_sub(e.get(this, TESWaterListener::iBaseCellNumberY));
            fn_00630120(e, this, cell_x, cell_y)
        } else {
            fn_00630120(e, this, 0, 0)
        };
        e.mem.set_f32(level, height);
        let margin = setting_float(e, 0x011d_1248);
        let margin = e.call(SCALE_FLOAT, &args![margin.to_bits()]).f32();
        let limit = f64::from(fld(e, level)) + f64::from(margin);
        let top = e.call(VECTOR_TERM, &args![at(0x20), at(0x70), 2u32]).u32();
        let top = e.call(VECTOR_LENGTH, &args![top]).f64();
        if !(top > limit) {
            true
        } else {
            e.call(OBJECT_HAS_LAYER, &args![object, 0x88u32]).bool()
        }
    })
}

// Translated from 00630100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0062b0a0(this, out, this)`; returns `out`.
pub fn fn_00630100(e: &mut Engine, this: u32, out: u32) -> u32 {
    e.call(0x0062_b0a0, &args![this, out, this]);
    out
}

// Translated from 00630120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The water level of cell (`x`, `y`) in the table (index `y * width + x`,
/// unsigned, below width squared), else `-FLT_MAX`.
pub fn fn_00630120(e: &mut Engine, this: Ptr<TESWaterListener>, x: i32, y: i32) -> f32 {
    let width = e.get(this, TESWaterListener::uiCellArrayWidth);
    let index = (y as u32).wrapping_mul(width).wrapping_add(x as u32);
    let width = e.get(this, TESWaterListener::uiCellArrayWidth);
    if index < width.wrapping_mul(width) {
        let levels = e.get(this, TESWaterListener::pfWaterLevels);
        fld(e, levels.wrapping_add(index.wrapping_mul(4)))
    } else {
        fld(e, NO_WATER)
    }
}

// Translation incomplete: 00630170 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Buoyancy of a rigid body at water `level` over `delta` seconds; `in_zone`
/// picks the layer (0x80 in a zone, else 8). When the level is below the
/// bottom of the object's bounds: if the object has the layer, splashes it
/// (`0062e970`) and returns 2; clears the layer. Else, for an active body
/// (`00c8cce0`) with a shape: the submerged ratio between a quarter and
/// 0.85 of the height (not positive: returns 0 at once), a buoyancy factor
/// (`0062b310`, replaced by the constants of the ratio bands), the upward
/// force (`00631140`, with a torque for long thin shapes, `00630bb0`), the
/// counter force on mass (`00631100`), damped linear velocity (settings
/// `011d0478`, `011d0cf0`, `011d0ee8`) and angular velocity (`011d0580`);
/// then, without the layer, sets it, splashes and returns 1; finally
/// `0084d310(body map, body, 1)`. Returns 0 otherwise. open: the /GS cookie.
pub fn fn_00630170(
    e: &mut Engine,
    this: Ptr<TESWaterListener>,
    body: u32,
    level: f32,
    delta: f32,
    in_zone: bool,
) -> u32 {
    e.with_stack(0x240, |e, frame| {
        let base = frame.addr() + 0x240;
        let at = move |offset: u32| base - offset;
        let mut result = 0;
        let source = e.call(0x004b_4ec0, &args![body]).u32();
        e.call(VECTOR_COPY, &args![at(0x20), source]);
        let object = e.call(BODY_OBJECT, &args![body]).u32();
        let bounds = at(0x50);
        e.call(0x0043_7c10, &args![bounds]);
        e.vcall(object, 0xf8, &args![bounds]);
        let bottom = component(e, bounds, 2);
        let layer = if in_zone { 0x80u32 } else { 8 };
        if !(f64::from(x87_quiet(level)) >= f64::from(bottom)) {
            if e.call(OBJECT_HAS_LAYER, &args![object, layer]).bool() {
                let height = e
                    .call(CONVERT_FLOAT, &args![x87_quiet(level).to_bits()])
                    .f32();
                fn_0062e970(e, this, object, height);
                result = 2;
            }
            e.call(OBJECT_SET_LAYER, &args![object, layer, 0u32]);
            return result;
        }
        let state = e.call(0x0051_7670, &args![body]).u32();
        e.mem.set_u32(at(0x54), state);
        let shape = if e.call(0x00c8_cce0, &args![at(0x54)]).bool() {
            // The getter the collidable lists use for their size (`0044ddc0`),
            // here on the body.
            e.call(ARRAY_SIZE, &args![body]).u32()
        } else {
            0
        };
        if shape != 0 && !buoyancy(e, body, object, shape, level, delta, base) {
            return result;
        }
        if !e.call(OBJECT_HAS_LAYER, &args![object, layer]).bool() {
            e.call(OBJECT_SET_LAYER, &args![object, layer, 1u32]);
            let height = e
                .call(CONVERT_FLOAT, &args![x87_quiet(level).to_bits()])
                .f32();
            fn_0062e970(e, this, object, height);
            result = 1;
        }
        e.call(0x0084_d310, &args![BODY_MAP_GLOBAL_011CC584, body, 1u32]);
        result
    })
}

/// `00630aa0` of `setting * -1 * delta`.
fn damping(e: &mut Engine, setting: u32, delta: f32) -> f32 {
    let value = setting_float(e, setting);
    let rate = (f64::from(value) * e.global::<f64>(MINUS_ONE) * f64::from(x87_quiet(delta))) as f32;
    fn_00630aa0(e, rate)
}

/// The index of the largest (`largest`) or smallest component of the
/// vector at `extent`, with the exe's comparison order and NaN handling
/// (every comparison reads both components again).
fn extreme_axis(e: &mut Engine, extent: u32, largest: bool) -> u32 {
    let x = f64::from(component(e, extent, 0));
    let y = f64::from(component(e, extent, 1));
    let first = if largest {
        u32::from(y > x)
    } else {
        u32::from(y < x)
    };
    let kept = f64::from(component(e, extent, first));
    let z = f64::from(component(e, extent, 2));
    let take_z = if largest { z > kept } else { z < kept };
    if take_z {
        2
    } else {
        first
    }
}

/// The active-body part of `00630170` (frame top `base`, offsets as the
/// exe's [EBP-n]); false when the submerged ratio is not positive (the exe
/// then returns 0 at once).
fn buoyancy(
    e: &mut Engine,
    body: u32,
    object: u32,
    shape: u32,
    level: f32,
    delta: f32,
    base: u32,
) -> bool {
    let at = move |offset: u32| base - offset;
    e.call(RELEASE, &args![shape]);
    let centre = e.call(0x0046_0140, &args![shape]).u32();
    e.call(VECTOR_COPY, &args![at(0x70), centre]);
    let velocity = e.call(0x0056_0dc0, &args![body]).u32();
    e.call(VECTOR_COPY, &args![at(0x80), velocity]);
    let mass = e.call(0x004b_4ea0, &args![body]).f32();
    let bounds = at(0x50);
    let top = component(e, at(0x40), 2);
    let bottom = component(e, bounds, 2);
    let height = (f64::from(top) - f64::from(bottom)) as f32;
    let quarter = f64::from(height) * e.global::<f64>(QUARTER);
    let low = (f64::from(component(e, bounds, 2)) + quarter) as f32;
    let most = f64::from(height) * e.global::<f64>(EIGHTY_FIVE_HUNDREDTHS);
    let high = (f64::from(component(e, bounds, 2)) + most) as f32;
    let level = f64::from(x87_quiet(level));
    let ratio = ((level - f64::from(low)) / (f64::from(high) - f64::from(low))) as f32;
    let zero = e.global::<f64>(ZERO);
    let ratio = f64::from(ratio);
    if ratio <= zero {
        e.call(RELEASE, &args![shape]);
        return false;
    }
    let mut factor = e.call(0x0062_b310, &args![object]).f32();
    let mut damp_velocity = true;
    if f64::from(fld(e, RATIO_0119AF4C)) > ratio && f64::from(factor) > zero {
        if f64::from(fld(e, RATIO_0119AF48)) < ratio {
            factor = fld(e, BUOYANCY_0119AF44);
        } else if f64::from(fld(e, RATIO_0119AF40)) < ratio {
            factor = fld(e, BUOYANCY_011CC5E4);
        } else if f64::from(fld(e, RATIO_011CC5E0)) < ratio {
            factor = fld(e, BUOYANCY_0119AF3C);
        }
        let term = e.call(VECTOR_TERM, &args![at(0x80), at(0xc0), 2u32]).u32();
        let speed = e.call(VECTOR_LENGTH, &args![term]).f32();
        let speed = e.call(0x0040_8840, &args![speed.to_bits()]).f64();
        if f64::from(fld(e, DEPTH_LIMIT_0119AF38)) > speed {
            damp_velocity = false;
        }
    }
    let factor = -x87_quiet(factor);
    e.call(VECTOR_DATA, &args![at(0xd0)]);
    let lift = (f64::from(factor) * f64::from(mass)) as f32;
    e.call(VECTOR_FROM_FLOAT, &args![at(0xf0), lift.to_bits()]);
    fn_00630ae0(e, at(0xd0), at(0xf0), at(0x70));
    e.call(VECTOR_DATA, &args![at(0x100)]);
    if e.call(0x00c9_c380, &args![body]).u32() == 0 {
        e.call(0x0043_7c10, &args![at(0x120)]);
        e.call(0x00c8_d390, &args![object, at(0x120)]);
        e.call(0x0055_32a0, &args![at(0x130), 0u32, 0u32, 0u32, 0u32]);
        e.call(VECTOR_DATA, &args![at(0x140)]);
        e.call(0x0045_8a10, &args![at(0x140), at(0x110), at(0x120)]);
        let longest = extreme_axis(e, at(0x140), true);
        let shortest = extreme_axis(e, at(0x140), false);
        let long = component(e, at(0x140), longest);
        let short = component(e, at(0x140), shortest);
        let short = f64::from(short) * e.global::<f64>(ONE_AND_A_HALF);
        if f64::from(long) > short {
            let long = component(e, at(0x140), longest);
            let short = component(e, at(0x140), shortest);
            let tilt = (f64::from(long) / f64::from(short)) * e.global::<f64>(ONE_TENTH);
            let sign = f64::from(fn_00631070(e, factor));
            let arm = sign * tilt;
            let slot = e.call(COMPONENT, &args![at(0x130), shortest]).u32();
            e.mem.set_f32(slot, arm as f32);
            e.call(0x0062_40d0, &args![at(0x180)]);
            e.vcall(object, 0xd8, &args![at(0x180)]);
            fn_00630bb0(e, at(0x100), at(0x180), at(0x130));
            let point = fn_006310c0(e, body);
            e.call(0x0055_3f70, &args![at(0x100), point]);
        } else {
            let point = fn_006310c0(e, body);
            e.call(0x004a_3f10, &args![at(0x100), point]);
        }
    } else {
        let point = fn_006310c0(e, body);
        e.call(0x004a_3f10, &args![at(0x100), point]);
    }
    e.call(RELEASE, &args![shape]);
    fn_00631140(e, body, delta, at(0xd0), at(0x100));
    e.call(VECTOR_DATA, &args![at(0x190)]);
    let weight = (f64::from(mass) * e.global::<f64>(MINUS_ONE)) as f32;
    e.call(VECTOR_FROM_FLOAT, &args![at(0x1b0), weight.to_bits()]);
    fn_00630ae0(e, at(0x190), at(0x1b0), at(0x70));
    hkp_rigid_body_apply_force(e, body, delta, at(0x190));
    e.call(RELEASE, &args![shape]);
    let linear = at(0x1c0);
    let velocity = e.call(0x0056_0dc0, &args![body]).u32();
    e.call(VECTOR_COPY, &args![linear, velocity]);
    let mut vertical = component(e, linear, 2);
    let slot = e.call(COMPONENT, &args![linear, 2u32]).u32();
    e.mem.set_f32(slot, 0.0);
    if damp_velocity {
        let scale = damping(e, 0x011d_0478, delta);
        vertical = (f64::from(scale) * f64::from(vertical)) as f32;
    }
    let length = fn_00630b40(e, linear, at(0x1e0));
    let length = e.call(VECTOR_LENGTH, &args![length]).f64();
    let threshold = setting_float(e, 0x011d_0cf0);
    if f64::from(threshold) < length {
        let scale = damping(e, 0x011d_0ee8, delta);
        e.call(VECTOR_FROM_FLOAT, &args![at(0x200), scale.to_bits()]);
        e.call(VECTOR_MULTIPLY, &args![linear, at(0x200)]);
    }
    let slot = e.call(COMPONENT, &args![linear, 2u32]).u32();
    e.mem.set_f32(slot, x87_quiet(vertical));
    e.call(0x0056_16d0, &args![body, linear]);
    let angular = at(0x210);
    let spin = e.call(0x0056_0e50, &args![body]).u32();
    e.call(VECTOR_COPY, &args![angular, spin]);
    let scale = damping(e, 0x011d_0580, delta);
    e.call(VECTOR_FROM_FLOAT, &args![at(0x230), scale.to_bits()]);
    e.call(VECTOR_MULTIPLY, &args![angular, at(0x230)]);
    e.call(0x0056_1800, &args![body, angular]);
    e.call(RELEASE, &args![shape]);
    true
}

// Translated from 00630aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00630ac0` of its argument (cdecl).
pub fn fn_00630aa0(e: &mut Engine, value: f32) -> f32 {
    fn_00630ac0(e, x87_quiet(value))
}

// Translated from 00630ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `exp` (`00ecae30`, a double argument) of a float, rounded to float
/// (cdecl).
pub fn fn_00630ac0(e: &mut Engine, value: f32) -> f32 {
    let wide = f64::from(x87_quiet(value));
    e.call(0x00ec_ae30, &args![wide]).f32()
}

// Translation incomplete: 00630ae0 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = *00561240(a, temp) * *b` (SSE, lane by lane). open: the /GS
/// cookie.
pub fn fn_00630ae0(e: &mut Engine, this: u32, a: u32, b: u32) {
    let factor = load4(e, b);
    e.with_stack(0x10, |e, temp| {
        let source = e.call(0x0056_1240, &args![a, temp]).u32();
        let value = load4(e, source);
        store4(e, this, mulps(value, factor));
    });
}

// Translation incomplete: 00630b40 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `out` = the vector `0062b0a0(this, temp, this)` gives (through
/// `006815c0`) with its first lane replaced by its square root (`SQRTSS`),
/// copied by `004a3c90`. Returns `out`. open: the /GS cookie.
pub fn fn_00630b40(e: &mut Engine, this: u32, out: u32) -> u32 {
    e.with_stack(0x20, |e, temp| {
        let product = e.call(0x0062_b0a0, &args![this, temp, this]).u32();
        let data = e.call(VECTOR_DATA, &args![product]).u32();
        let mut value = load4(e, data);
        value[0] = value[0].sqrt();
        let root = temp.addr() + 0x10;
        store4(e, root, value);
        e.call(VECTOR_COPY, &args![out, root]);
    });
    out
}

// Translation incomplete: 00630bb0 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a vector from `a` (lane 3 broadcast, combined with the constant
/// at `010c71e0`, `0062b0a0(a, temp, b)` and `a x b`) and `b`, through the
/// vector helpers of this unit, and writes it with `00458880(this, t, t)`.
/// open: the /GS cookie.
pub fn fn_00630bb0(e: &mut Engine, this: u32, a: u32, b: u32) {
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr() + 0x80;
        let at = move |offset: u32| base - offset;
        e.call(VECTOR_DATA, &args![at(0x20)]);
        let data = e.call(VECTOR_DATA, &args![at(0x20)]).u32();
        let constant = load4(e, VECTOR_010C71E0);
        store4(e, data, constant);
        e.call(VECTOR_DATA, &args![at(0x30)]);
        fn_00630fa0(e, at(0x30), a, 3);
        e.call(VECTOR_DATA, &args![at(0x40)]);
        fn_00630d90(e, at(0x40), at(0x20), at(0x30), at(0x30));
        e.call(VECTOR_DATA, &args![at(0x50)]);
        fn_00630ce0(e, at(0x50), b, at(0x40));
        e.call(VECTOR_DATA, &args![at(0x60)]);
        let rotation = e.call(VECTOR_DATA, &args![a]).u32();
        let product = e.call(0x0062_b0a0, &args![rotation, at(0x70), b]).u32();
        let source = e.call(VECTOR_DATA, &args![product]).u32();
        let target = e.call(VECTOR_DATA, &args![at(0x60)]).u32();
        let value = load4(e, source);
        store4(e, target, value);
        fn_00630ed0(e, at(0x60), 0);
        let rotation = e.call(VECTOR_DATA, &args![a]).u32();
        fn_00630d30(e, at(0x50), at(0x60), rotation);
        e.call(VECTOR_DATA, &args![at(0x80)]);
        let rotation = e.call(VECTOR_DATA, &args![a]).u32();
        fn_00630df0(e, at(0x80), rotation, b);
        fn_00630d30(e, at(0x50), at(0x30), at(0x80));
        e.call(0x0045_8880, &args![this, at(0x50), at(0x50)]);
    });
}

// Translation incomplete: 00630ce0 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = a * b` (SSE, lane by lane). open: the /GS cookie.
pub fn fn_00630ce0(e: &mut Engine, this: u32, a: u32, b: u32) {
    let value = mulps(load4(e, a), load4(e, b));
    store4(e, this, value);
}

// Translation incomplete: 00630d30 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this += a * b` (SSE, lane by lane). open: the /GS cookie.
pub fn fn_00630d30(e: &mut Engine, this: u32, a: u32, b: u32) {
    let product = mulps(load4(e, a), load4(e, b));
    let value = addps(load4(e, this), product);
    store4(e, this, value);
}

// Translation incomplete: 00630d90 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = a + b * c` (SSE, lane by lane). open: the /GS cookie.
pub fn fn_00630d90(e: &mut Engine, this: u32, a: u32, b: u32, c: u32) {
    let product = mulps(load4(e, b), load4(e, c));
    let value = addps(load4(e, a), product);
    store4(e, this, value);
}

// Translation incomplete: 00630df0 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = a x b` (SSE shuffles 0xc9 and 0xd2; lane 3 is `a3*b3 - a3*b3`).
/// open: the /GS cookie.
pub fn fn_00630df0(e: &mut Engine, this: u32, a: u32, b: u32) {
    let a = load4(e, a);
    let b = load4(e, b);
    let first = mulps(shufps(a, 0xc9), shufps(b, 0xd2));
    let second = mulps(shufps(a, 0xd2), shufps(b, 0xc9));
    store4(e, this, subps(first, second));
}

/// The `SHUFPS` immediates broadcasting lane 0, 1, 2 or 3.
const BROADCAST: [u8; 4] = [0x00, 0x55, 0xaa, 0xff];

// Translation incomplete: 00630ed0 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Broadcasts lane `lane` (0 to 3) of `this` to all four; other values
/// leave it. open: the /GS cookie.
pub fn fn_00630ed0(e: &mut Engine, this: u32, lane: u32) {
    if lane < 4 {
        let value = load4(e, this);
        store4(e, this, shufps(value, BROADCAST[lane as usize]));
    }
}

// Translation incomplete: 00630fa0 - /GS stack cookie check (00ec408c) not modelled.
// Translated from 00630fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` = lane `lane` (0 to 3) of `source` in all four lanes; other values
/// leave it. open: the /GS cookie.
pub fn fn_00630fa0(e: &mut Engine, this: u32, source: u32, lane: u32) {
    if lane < 4 {
        let value = load4(e, source);
        store4(e, this, shufps(value, BROADCAST[lane as usize]));
    }
}

// Translated from 00631070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sign of a float (cdecl): 1 above 0, the constant at `01012054` (-1)
/// below, else 0 (also for NaN).
pub fn fn_00631070(e: &mut Engine, value: f32) -> f32 {
    let value = f64::from(x87_quiet(value));
    let zero = e.global::<f64>(ZERO);
    if value > zero {
        1.0
    } else if value < zero {
        fld(e, MINUS_ONE_FLOAT)
    } else {
        0.0
    }
}

// Translated from 006310c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `006310e0` of `009d9f40(this)`.
pub fn fn_006310c0(e: &mut Engine, this: u32) -> u32 {
    let inner = e.call(0x009d_9f40, &args![this]).u32();
    fn_006310e0(e, inner)
}

// Translated from 006310e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0087eaa0(this + 0x10) + 0x10`.
pub fn fn_006310e0(e: &mut Engine, this: u32) -> u32 {
    e.call(0x0087_eaa0, &args![this.wrapping_add(0x10)])
        .u32()
        .wrapping_add(0x10)
}

// Translated from 00631100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::applyForce` (Xbox PDB): activates the body
/// (`hkpEntity::activate`), then virtual slot 0x5c of `009d9f40(this)` with
/// `(delta, force)`.
pub fn hkp_rigid_body_apply_force(e: &mut Engine, this: u32, delta: f32, force: u32) {
    e.call(0x00c9_c1d0, &args![this]);
    let motion = e.call(0x009d_9f40, &args![this]).u32();
    e.vcall(motion, 0x5c, &args![x87_quiet(delta).to_bits(), force]);
}

// Translated from 00631140 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `applyForce` with virtual slot 0x58 and `(delta, force, point)`.
pub fn fn_00631140(e: &mut Engine, this: u32, delta: f32, force: u32, point: u32) {
    e.call(0x00c9_c1d0, &args![this]);
    let motion = e.call(0x009d_9f40, &args![this]).u32();
    e.vcall(
        motion,
        0x58,
        &args![x87_quiet(delta).to_bits(), force, point],
    );
}

// Translated from 00631750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<hkpCollidable const *,1024>` constructor: vtable, then
/// `006b3eb0(this, 0, 0)`.
pub fn fn_00631750(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), COLLIDABLE_ARRAY_VTABLE);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00631780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<hkpCollidable const *,1024>` destructor body: vtable,
/// then `008454f0(this, 1)`.
pub fn fn_00631780(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COLLIDABLE_ARRAY_VTABLE);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 006317c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<hkpCollidable const *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB).
pub fn bs_simple_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00631780(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006317f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00559a40(this, source)`, then a zero word at +4; returns `this`.
pub fn fn_006317f0(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.call(0x0055_9a40, &args![this, source]);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 00631820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the first words of `this` and `other` differ (0 or 1 in `EAX`).
pub fn fn_00631820(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    u32::from(e.mem.u32(this.addr()) != e.mem.u32(other.addr()))
}

// Translated from 00631840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk in vtable slot 0 of the listener's `BGSZoneTargetListener` part
/// (+0x14): `this - 0x14` to `0062e750`.
pub fn fn_00631840(e: &mut Engine, this: u32, flags: u32) -> u32 {
    let listener = Ptr::new(this.wrapping_sub(ZONE_LISTENER));
    tes_water_listener_scalar_deleting_destructor(e, listener, flags).addr()
}

// Translated from 00631850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<hkpRigidBody *,bool>` constructor: base map with `hash_size`
/// buckets, then the `NiTMap` vtable.
pub fn fn_00631850(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006318b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), BODY_MAP_VTABLE);
    this
}

// Translated from 00631880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<hkpRigidBody *,bool>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00631920(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006318b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..hkpRigidBody *,bool>` constructor: stores the hash size,
/// zero count, allocates the bucket table and clears it.
pub fn fn_006318b0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), BODY_MAP_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, hash_size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let bytes = e.mem.u32(this.addr() + 4) << 2;
    let table = e.call(0x00aa_1070, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, table);
    // The size and the table are both reloaded after the allocation.
    let clear_bytes = e.mem.u32(this.addr() + 4) << 2;
    let table = e.mem.u32(this.addr() + 8);
    e.call(0x0040_3d30, &args![table, 0u32, clear_bytes]);
    this
}

// Translation incomplete: 00631920 - FS:[0] exception frame (unwind into the base destructor) not translated.
// Translated from 00631920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<hkpRigidBody *,bool>` destructor body: `RemoveAll`, then the base
/// destructor. open: the exception frame.
pub fn fn_00631920(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BODY_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00631980(e, this);
}

// Translated from 00631980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..hkpRigidBody *,bool>` destructor body: `RemoveAll`, then
/// frees the bucket table.
pub fn fn_00631980(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BODY_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(0x00aa_10f0, &args![e.mem.u32(this.addr() + 8)]);
}

// Translated from 006319b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<hkpRigidBody *,bool>>,hkpRigidBody *,bool>::_scalar_deleting_destructor_`
/// (Xbox PDB).
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00631980(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0062e3d0,
            water_zone_add_target(Ptr<WaterZone>, u32, u32, u32) -> u32
        ),
        entry!(0x0062e500, fn_0062e500(Ptr<WaterZone>, u32, u32)),
        entry!(
            0x0062e590,
            water_target_entry_water_target_entry(
                Ptr<WaterTargetEntry>,
                u32,
                u32,
            ) -> Ptr<WaterTargetEntry>
        ),
        entry!(
            0x0062e750,
            tes_water_listener_scalar_deleting_destructor(
                Ptr<TESWaterListener>,
                u32,
            ) -> Ptr<TESWaterListener>
        ),
        entry!(
            0x0062e780,
            tes_water_listener_tes_water_listener(Ptr<TESWaterListener>) -> Ptr<TESWaterListener>
        ),
        entry!(0x0062e970, fn_0062e970(Ptr<TESWaterListener>, u32, f32)),
        entry!(
            0x0062eb50,
            tes_water_listener_play_splash_effects(f32, f32, f32, f32)
        ),
        entry!(
            0x0062ee60,
            tes_water_listener_update(Ptr<TESWaterListener>, f32)
        ),
        entry!(0x0062f2f0, fn_0062f2f0(Ptr<TESWaterListener>) -> bool),
        entry!(0x0062f310, fn_0062f310(u32)),
        entry!(0x0062f320, fn_0062f320(Ptr<TESWaterListener>, u32) -> u32),
        entry!(0x0062f3d0, fn_0062f3d0() -> u32),
        entry!(0x0062f460, fn_0062f460()),
        entry!(
            0x0062f5b0,
            fn_0062f5b0(Ptr<TESWaterListener>, u32, u32) -> bool
        ),
        entry!(
            0x0062f630,
            fn_0062f630(Ptr<TESWaterListener>, u32, u32) -> bool
        ),
        entry!(
            0x0062f6b0,
            tes_water_listener_contact_point_added_callback(Ptr<TESWaterListener>, u32) -> bool
        ),
        entry!(0x0062f8e0, fn_0062f8e0(bool)),
        entry!(0x0062f8f0, fn_0062f8f0(f32)),
        entry!(
            0x0062f900,
            tes_water_listener_contact_point_removed_callback(Ptr<TESWaterListener>, u32) -> bool
        ),
        entry!(
            0x0062fc70,
            fn_0062fc70(Ptr<TESWaterListener>, Ptr<QueuedRemoval>)
        ),
        entry!(0x0062fe80, fn_0062fe80(u32) -> u32),
        entry!(0x0062fec0, fn_0062fec0(u32) -> u32),
        entry!(
            0x0062ff10,
            fn_0062ff10(Ptr<TESWaterListener>, u32, u32, f32, u32, u32) -> bool
        ),
        entry!(0x00630100, fn_00630100(u32, u32) -> u32),
        entry!(
            0x00630120,
            fn_00630120(Ptr<TESWaterListener>, i32, i32) -> f32
        ),
        entry!(
            0x00630170,
            fn_00630170(Ptr<TESWaterListener>, u32, f32, f32, bool) -> u32
        ),
        entry!(0x00630aa0, fn_00630aa0(f32) -> f32),
        entry!(0x00630ac0, fn_00630ac0(f32) -> f32),
        entry!(0x00630ae0, fn_00630ae0(u32, u32, u32)),
        entry!(0x00630b40, fn_00630b40(u32, u32) -> u32),
        entry!(0x00630bb0, fn_00630bb0(u32, u32, u32)),
        entry!(0x00630ce0, fn_00630ce0(u32, u32, u32)),
        entry!(0x00630d30, fn_00630d30(u32, u32, u32)),
        entry!(0x00630d90, fn_00630d90(u32, u32, u32, u32)),
        entry!(0x00630df0, fn_00630df0(u32, u32, u32)),
        entry!(0x00630ed0, fn_00630ed0(u32, u32)),
        entry!(0x00630fa0, fn_00630fa0(u32, u32, u32)),
        entry!(0x00631070, fn_00631070(f32) -> f32),
        entry!(0x006310c0, fn_006310c0(u32) -> u32),
        entry!(0x006310e0, fn_006310e0(u32) -> u32),
        entry!(0x00631100, hkp_rigid_body_apply_force(u32, f32, u32)),
        entry!(0x00631140, fn_00631140(u32, f32, u32, u32)),
        entry!(0x00631750, fn_00631750(Ptr) -> Ptr),
        entry!(0x00631780, fn_00631780(Ptr)),
        entry!(0x006317c0, bs_simple_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006317f0, fn_006317f0(Ptr, u32) -> Ptr),
        entry!(0x00631820, fn_00631820(Ptr, Ptr) -> u32),
        entry!(0x00631840, fn_00631840(u32, u32) -> u32),
        entry!(0x00631850, fn_00631850(Ptr, u32) -> Ptr),
        entry!(0x00631880, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006318b0, fn_006318b0(Ptr, u32) -> Ptr),
        entry!(0x00631920, fn_00631920(Ptr)),
        entry!(0x00631980, fn_00631980(Ptr)),
        entry!(0x006319b0, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// A mapped test area (objects with fixed addresses, flag bytes).
    const SCRATCH: u32 = 0x0300_0000;
    /// One vtable every test object here shares.
    const VTABLE: u32 = SCRATCH + 0x1000;
    /// Fake targets of its slots.
    const SLOT_38: u32 = 0x0700_0038;
    const SLOT_40: u32 = 0x0700_0040;
    const SLOT_58: u32 = 0x0700_0058;
    const SLOT_5C: u32 = 0x0700_005c;
    const SLOT_D8: u32 = 0x0700_00d8;
    const SLOT_F8: u32 = 0x0700_00f8;
    const SLOT_140: u32 = 0x0700_0140;
    const SLOT_1D0: u32 = 0x0700_01d0;
    const SLOT_1F4: u32 = 0x0700_01f4;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    fn setting(e: &mut Engine, address: u32, value: f32) {
        e.map(address, 4);
        e.set_global(address, value);
    }

    /// An engine with the constants, the settings getter, the vector
    /// accessors, the shared vtable and the two game pointers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let doubles = [
            (ZERO, 0.0),
            (SPLASH_SPEED_FLOOR, 0.001),
            (NO_WATER_DOUBLE, f64::from(f32::MIN)),
            (QUARTER, 0.25),
            (EIGHTY_FIVE_HUNDREDTHS, f64::from(0.85f32)),
            (ONE_AND_A_HALF, 1.5),
            (ONE_TENTH, f64::from(0.1f32)),
            (MINUS_ONE, -1.0),
        ];
        for (address, value) in doubles {
            e.map(address, 8);
            e.set_global(address, value);
        }
        let floats = [
            (RIPPLE_SIZE, 0.01f32),
            (NO_WATER, f32::MIN),
            (MINUS_ONE_FLOAT, -1.0),
            (DEPTH_LIMIT_0119AF38, 2.0),
            (BUOYANCY_0119AF3C, -0.3),
            (RATIO_0119AF40, 0.5),
            (BUOYANCY_0119AF44, 0.3),
            (RATIO_0119AF48, 0.75),
            (RATIO_0119AF4C, 1.2),
            (RATIO_011CC5E0, 0.0),
            (BUOYANCY_011CC5E4, 0.0),
        ];
        for (address, value) in floats {
            setting(&mut e, address, value);
        }
        e.register(SETTING_VALUE_POINTER, |_, a| ret(a[0]));
        e.register(VECTOR_DATA, |_, a| ret(a[0]));
        e.register(COMPONENT, |_, a| ret(a[0] + 4 * a[1]));
        e.map(SCRATCH, 0x2000);
        let mut slots = vec![0u32; 0x80];
        for slot in [
            SLOT_38, SLOT_40, SLOT_58, SLOT_5C, SLOT_D8, SLOT_F8, SLOT_140, SLOT_1D0, SLOT_1F4,
        ] {
            slots[((slot & 0xfff) / 4) as usize] = slot;
            e.register(slot, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e.map(TES_POINTER, 4);
        e.map(PLAYER_POINTER, 4);
        e.set_global(TES_POINTER, 0x0000_9000u32);
        e.map(LISTENER_GLOBAL_011CC54C, 4);
        e.map(WATER_GLOBAL_011C7A3C, 4);
        e.map(FLAG_GLOBAL_011C7A5A, 1);
        e.map(HEIGHT_GLOBAL_011C7A5C, 4);
        e.map(VECTOR_010C71E0, 16);
        e
    }

    /// A heap object with the shared vtable.
    fn vtable_object(e: &mut Engine, size: u32) -> u32 {
        let address = e.mem.alloc(size);
        e.mem.set_u32(address, VTABLE);
        address
    }

    fn listener(e: &mut Engine) -> Ptr<TESWaterListener> {
        let this: Ptr<TESWaterListener> = e.new_object();
        e.set(this, TESWaterListener::fSplashTimer, 1.0);
        this
    }

    #[test]
    fn add_target_creates_entry_or_extends_its_list() {
        let mut e = engine();
        let zone: Ptr<WaterZone> = e.new_object();
        e.register(0x0055_3300, |e, a| ret(e.mem.alloc(a[0])));
        e.register(ARRAY_FIND, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x77);
            ret(if a[0] == 0 { 0 } else { u32::MAX })
        });
        noop(
            &mut e,
            &[
                0x0061_fba0,
                0x006b_3eb0,
                ARRAY_ADD,
                0x0062_0760,
                0x006e_cd40,
            ],
        );
        e.call_log = Some(vec![]);
        let target = e
            .call(0x0062_e3d0, &args![zone, 0x10u32, 0x77u32, 0x99u32])
            .u32();
        assert_eq!(e.get(zone, WaterZone::pTargets), target);
        assert_eq!(e.mem.u32(target), TARGET_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(target + 0x50), COLLIDABLE_ARRAY_VTABLE);
        assert_eq!(calls(&e, 0x0061_fba0), vec![vec![target, 0x10, 0]]);
        assert_eq!(calls(&e, 0x0062_0760), vec![vec![target + 0x10, 0x99, 1]]);
        assert_eq!(calls(&e, 0x006e_cd40), vec![vec![target + 0x10, 0]]);
        assert_eq!(calls(&e, ARRAY_ADD).len(), 1);

        e.register(ZONE_FIND_TARGET, |_, a| ret(a[0] + 0x1000));
        e.register(ARRAY_FIND, |_, _| ret(2));
        e.call_log = Some(vec![]);
        let found = e
            .call(0x0062_e3d0, &args![zone, 0x10u32, 0x77u32, 0x99u32])
            .u32();
        assert_eq!(found, zone.addr() + 0x1000);
        assert!(calls(&e, ARRAY_ADD).is_empty());
        assert_eq!(calls(&e, ZONE_FIND_TARGET), vec![vec![zone.addr(), 0x10]]);
    }

    #[test]
    fn remove_collidable_drops_empty_target() {
        let mut e = engine();
        let zone: Ptr<WaterZone> = e.new_object();
        e.set(zone, WaterZone::pTargets, 0x5000);
        e.register(ZONE_FIND_TARGET, |_, _| ret(0x5000));
        e.register(ARRAY_FIND, |_, _| ret(1));
        e.register(ARRAY_SIZE, |_, _| ret(0));
        noop(&mut e, &[0x009a_4320, 0x0061_ff30]);
        e.call_log = Some(vec![]);
        e.call(0x0062_e500, &args![zone, 0x10u32, 0x77u32]);
        assert_eq!(calls(&e, 0x009a_4320), vec![vec![0x5050, 1, 1]]);
        assert_eq!(calls(&e, 0x0061_ff30), vec![vec![zone.addr(), 0x5000]]);

        e.register(ARRAY_FIND, |_, _| ret(u32::MAX));
        e.register(ARRAY_SIZE, |_, _| ret(3));
        e.call_log = Some(vec![]);
        e.call(0x0062_e500, &args![zone, 0x10u32, 0x77u32]);
        assert!(calls(&e, 0x009a_4320).is_empty());
        assert!(calls(&e, 0x0061_ff30).is_empty());

        e.set(zone, WaterZone::pTargets, 0);
        e.call_log = Some(vec![]);
        e.call(0x0062_e500, &args![zone, 0x10u32, 0x77u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn target_entry_constructor_builds_list_and_adds_collidable() {
        let mut e = engine();
        let entry: Ptr<WaterTargetEntry> = e.new_object();
        e.register(ARRAY_FIND, |_, _| ret(u32::MAX));
        e.register(ARRAY_ADD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x77);
            Ret::default()
        });
        noop(&mut e, &[0x0061_fba0, 0x006b_3eb0]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_e590, &args![entry, 0x10u32, 0x77u32]).u32();
        assert_eq!(result, entry.addr());
        assert_eq!(e.mem.u32(entry.addr()), TARGET_ENTRY_VTABLE);
        assert_eq!(
            calls(&e, 0x006b_3eb0),
            vec![vec![entry.addr() + 0x50, 0, 0]]
        );
        assert_eq!(calls(&e, ARRAY_ADD).len(), 1);
        e.register(ARRAY_FIND, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0062_e590, &args![entry, 0x10u32, 0x77u32]);
        assert!(calls(&e, ARRAY_ADD).is_empty());
    }

    #[test]
    fn listener_scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        let this = listener(&mut e);
        noop(&mut e, &[0x0062_e860, FREE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_e750, &args![this, 0u32]).u32(), this.addr());
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0062_e750, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, 0x0062_e860).len(), 2);
    }

    #[test]
    fn listener_constructor_sizes_level_table() {
        let mut e = engine();
        let this: Ptr<TESWaterListener> = e.new_object();
        let width = e.mem.alloc(4);
        e.mem.set_u32(width, 3);
        e.register_double(0x0043_d4d0, move |_, a| {
            assert_eq!(a, &[CELL_ARRAY_WIDTH_SETTING]);
            ret(width)
        });
        e.register(ALLOCATE, |_, a| ret(0x0010_0000u32.wrapping_add(a[0])));
        noop(&mut e, &[0x00cb_75d0, 0x0061_ffb0, 0x0048_f200]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_e780, &args![this]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(e.mem.u32(this.addr()), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x14), ZONE_LISTENER_VTABLE);
        assert_eq!(e.get(this, TESWaterListener::fSplashTimer), 1.0);
        assert_eq!(e.get(this, TESWaterListener::uiCellArrayWidth), 3);
        assert_eq!(
            e.get(this, TESWaterListener::pfWaterLevels),
            0x0010_0000 + 36
        );
        assert_eq!(calls(&e, 0x0061_ffb0), vec![vec![this.addr() + 0x14]]);
        assert_eq!(calls(&e, 0x0048_f200), vec![vec![this.addr() + 0x34]]);
        // 0x9000 squared times 4 overflows: the size saturates.
        e.mem.set_u32(width, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x0062_e780, &args![this]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![u32::MAX]]);
    }

    fn splash_doubles(e: &mut Engine) {
        e.register(0x0043_b4f0, |_, _| ret(0x10));
        e.register(0x0043_b4d0, |_, _| ret(3));
        e.register(0x004b_5400, |_, _| st0(2.0));
        e.register(0x0056_0d50, |e, a| {
            e.mem.set_f32(a[1] + 8, -3.0);
            Ret::default()
        });
        e.register(0x0040_8820, |_, a| {
            assert_eq!(a, &[(-3.0f32).to_bits()]);
            st0(3.0)
        });
        e.register(0x0056_0bc0, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            Ret::default()
        });
        e.register(0x005f_36f0, |_, _| ret(1));
        e.register(USE_TASK_QUEUE, |_, _| ret(1));
        e.register(TASK_QUEUE, |_, _| ret(0x4000));
        e.register(QUEUE_SPLASH, |e, a| {
            assert_eq!(a[1], 3.5f32.to_bits());
            assert_eq!(e.mem.f32(a[2]), 1.0);
            assert_eq!(e.mem.f32(a[2] + 8), 10.0);
            Ret::default()
        });
        noop(e, &[QUEUE_ADD_RIPPLE]);
        setting(e, 0x011d_00a8, 0.5);
        setting(e, 0x011d_0ab4, 2.0);
        setting(e, 0x011d_135c, 4.0);
    }

    #[test]
    fn splash_waits_for_timer_and_plays_when_surface_allows() {
        let mut e = engine();
        let this = listener(&mut e);
        splash_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0062_e970, &args![this, 0x500u32, 10.0f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        e.set(this, TESWaterListener::fSplashTimer, 0.0);
        e.call(0x0062_e970, &args![this, 0x500u32, 10.0f32]);
        assert_eq!(calls(&e, QUEUE_SPLASH).len(), 1);
        let ripple = calls(&e, QUEUE_ADD_RIPPLE);
        assert_eq!(ripple.len(), 1);
        assert_eq!(ripple[0][..2], [0x4000, 0.01f32.to_bits()]);
        assert_eq!(e.get(this, TESWaterListener::fSplashTimer), 4.0);

        // A surface above the water height (004572e0) stops the splash.
        e.set(this, TESWaterListener::fSplashTimer, 0.0);
        e.register(0x005f_36f0, |_, _| ret(0));
        e.register(0x0045_72e0, |e, a| {
            e.mem.set_f32(a[2], 20.0);
            ret(1)
        });
        e.call_log = Some(vec![]);
        e.call(0x0062_e970, &args![this, 0x500u32, 10.0f32]);
        assert!(calls(&e, QUEUE_SPLASH).is_empty());
        assert_eq!(e.get(this, TESWaterListener::fSplashTimer), 0.0);

        // Model type 8 stops before the speed.
        e.register(0x0043_b4d0, |_, _| ret(8));
        e.call_log = Some(vec![]);
        e.call(0x0062_e970, &args![this, 0x500u32, 10.0f32]);
        assert!(calls(&e, 0x004b_5400).is_empty());
    }

    fn sound_doubles(e: &mut Engine) {
        e.register(0x00ad_8ce0, |e, a| ret(u32::from(e.mem.u32(a[0]) != 0)));
        e.register(0x0041_8900, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0045_3a70, |_, _| ret(0x300));
        e.register(0x00ad_7550, |_, a| {
            assert_eq!(a[0], 0x300);
            assert_eq!(a[3], 0x102);
            ret(a[2])
        });
        noop(
            e,
            &[0x0041_a250, RELEASE, 0x00ad_8b60, 0x00ad_8830, ADD_RIPPLE],
        );
        e.register(USE_TASK_QUEUE, |_, _| ret(0));
        e.register(WATER_SYSTEM, |_, _| ret(0x900));
        for (threshold, _, _, volume) in SPLASH_SOUNDS {
            setting(e, threshold, 0.0);
            setting(e, volume, 0.0);
        }
        setting(e, 0x011d_0c78, 10.0);
        setting(e, 0x011d_07f0, 4.0);
        setting(e, 0x011d_0310, 1.0);
        setting(e, 0x011d_1458, 0.9);
        setting(e, 0x011d_012c, 0.5);
        setting(e, 0x011d_091c, 0.2);
    }

    #[test]
    fn play_splash_effects_picks_sound_and_starts_particles() {
        let mut e = engine();
        sound_doubles(&mut e);
        e.register(REFERENCE_CELL, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0062_eb50, &args![5.0f32, 1.0f32, 2.0f32, 3.0f32]);
        let names: Vec<u32> = calls(&e, 0x00ad_7550).iter().map(|w| w[2]).collect();
        assert_eq!(names, vec![0x0104_e490]);
        assert_eq!(
            calls(&e, 0x00ad_8b60)[0][1..],
            [bits(1.0), bits(2.0), bits(3.0)]
        );
        assert_eq!(calls(&e, 0x00ad_8830)[0][1], 0);
        assert!(calls(&e, 0x0068_9210).is_empty());
        assert_eq!(
            calls(&e, ADD_RIPPLE),
            vec![vec![0x900, bits(1.0), bits(2.0), bits(3.0), bits(0.01)]]
        );

        e.register(REFERENCE_CELL, |_, _| ret(0x600));
        e.register(GET_WATER_TYPE, |_, _| ret(0x700));
        e.register(0x0058_0060, |_, _| ret(0));
        e.register(0x004a_4240, |_, _| st0(0.5));
        e.register(SETTING_VALUE_WORD, |_, _| ret(0x42));
        noop(&mut e, &[0x004a_0c90, 0x0068_9210]);
        e.call_log = Some(vec![]);
        e.call(0x0062_eb50, &args![20.0f32, 1.0f32, 2.0f32, 3.0f32]);
        let names: Vec<u32> = calls(&e, 0x00ad_7550).iter().map(|w| w[2]).collect();
        assert_eq!(names, vec![0x0104_e4a0]);
        assert_eq!(calls(&e, 0x004a_0c90)[0][1], bits(0.5));
        let effect = calls(&e, 0x0068_9210);
        assert_eq!(effect.len(), 1);
        assert_eq!(effect[0].len(), 18);
        assert_eq!(effect[0][..3], [0x600, bits(1.0), 0x42]);
        assert_eq!(
            effect[0][12..],
            [bits(1.0), bits(2.0), bits(3.0), bits(0.9), 1, 0]
        );

        // Too slow for every sound: nothing plays, the ripple still comes.
        e.call_log = Some(vec![]);
        e.call(0x0062_eb50, &args![0.5f32, 1.0f32, 2.0f32, 3.0f32]);
        assert!(calls(&e, 0x00ad_8b60).is_empty());
        assert_eq!(calls(&e, ADD_RIPPLE).len(), 1);
    }

    fn update_doubles(e: &mut Engine, this: Ptr<TESWaterListener>) {
        let player = vtable_object(e, 0x10);
        e.set_global(PLAYER_POINTER, player);
        e.register(SLOT_1F4, |_, _| ret(0x88));
        setting(e, 0x011d_05d4, 3.0);
        setting(e, 0x011d_1248, 1.0);
        e.register(SCALE_FLOAT, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(FLAG_POINTER, |_, _| ret(SCRATCH));
        e.register(0x004a_4460, |_, _| ret(0));
        e.register(0x0055_9450, |_, _| ret(0x1234));
        e.register(0x0062_0b80, |_, _| ret(0));
        e.register(NODE_NEXT, |_, _| ret(0));
        e.register(ZONE_FIRST_TARGET, |_, _| ret(0x0006_0000));
        e.register(ARRAY_SIZE, |_, _| ret(1));
        e.register(0x006a_7ad0, |_, _| ret(SCRATCH + 0x10));
        e.mem.set_u32(SCRATCH + 0x10, 0x3333);
        e.register(0x004b_59f0, |_, _| ret(0x2222));
        e.register(MAP_GET_AT, |_, _| ret(1));
        noop(e, &[0x004a_3e00, MAP_REMOVE_ALL, 0x0063_1370]);
        e.register(0x0052_4c90, |_, _| ret(0x500));
        e.register(REFERENCE_CELL, |_, _| ret(0x600));
        e.register(0x0045_bb80, |_, _| ret(SCRATCH + 0x20));
        e.mem.set_f32(SCRATCH + 0x28, 1.0);
        e.register(GET_WATER_HEIGHT, |_, _| st0(5.0));
        e.register(GET_WATER_TYPE, |_, _| ret(0x777));
        let zone: Ptr<WaterZone> = e.new_object();
        e.set(this, TESWaterListener::pZones, zone.addr());
    }

    #[test]
    fn update_scans_entities_and_zones_then_checks_player_head() {
        let mut e = engine();
        let this = listener(&mut e);
        update_doubles(&mut e, this);
        e.call_log = Some(vec![]);
        e.call(0x0062_ee60, &args![this, 0.25f32]);
        // Disabled: only the head check.
        assert!(calls(&e, MAP_REMOVE_ALL).is_empty());
        assert_eq!(e.global::<u32>(WATER_GLOBAL_011C7A3C), 0x777);
        assert_eq!(e.get(this, TESWaterListener::fSplashTimer), 1.0);

        e.set(this, TESWaterListener::bEnabled, true);
        e.set_global(WATER_GLOBAL_011C7A3C, 0u32);
        e.mem.set_f32(SCRATCH + 0x28, 6.0);
        e.call_log = Some(vec![]);
        e.call(0x0062_ee60, &args![this, 0.25f32]);
        assert_eq!(e.get(this, TESWaterListener::fSplashTimer), 0.75);
        assert_eq!(calls(&e, SCALE_FLOAT)[0], vec![bits(9.0)]);
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL),
            vec![vec![BODY_MAP_GLOBAL_011CC584]]
        );
        assert_eq!(calls(&e, 0x0062_0b80), vec![vec![0x1234]]);
        assert_eq!(calls(&e, 0x004b_59f0), vec![vec![0x3333]]);
        let lookups = calls(&e, MAP_GET_AT);
        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0][..2], [BODY_MAP_GLOBAL_011CC584, 0x2222]);
        assert_eq!(calls(&e, 0x0063_1370), vec![vec![this.addr(), 0, 1, 0]]);
        // The head is above the water now: the global stays.
        assert_eq!(e.global::<u32>(WATER_GLOBAL_011C7A3C), 0);
    }

    #[test]
    fn enabled_flag_is_byte_eight() {
        let mut e = engine();
        let this = listener(&mut e);
        assert!(!e.call(0x0062_f2f0, &args![this]).bool());
        e.set(this, TESWaterListener::bEnabled, true);
        assert!(e.call(0x0062_f2f0, &args![this]).bool());
    }

    #[test]
    fn water_global_setter_stores_word() {
        let mut e = engine();
        e.call(0x0062_f310, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(WATER_GLOBAL_011C7A3C), 0x1234);
    }

    #[test]
    fn rigid_body_slot_accepts_listed_model_types() {
        let mut e = engine();
        let this = listener(&mut e);
        e.register(BODY_OBJECT, |_, _| ret(0x10));
        e.register(0x0065_3270, |_, a| {
            assert_eq!(a, &[0x0126_81c0, 0x10]);
            ret(0x20)
        });
        e.register(0x0051_7630, |_, _| ret(1));
        e.register(0x0043_b4f0, |_, _| ret(0x30));
        e.register(0x0043_b4d0, |e, _| ret(e.mem.u32(SCRATCH)));
        e.mem.set_u32(SCRATCH, 13);
        assert_eq!(e.call(0x0062_f320, &args![this, 0x40u32]).u32(), 0x20);
        e.mem.set_u32(SCRATCH, 7);
        assert_eq!(e.call(0x0062_f320, &args![this, 0x40u32]).u32(), 0);
        e.mem.set_u32(SCRATCH, 29);
        e.register(0x0051_7630, |_, _| ret(5));
        assert_eq!(e.call(0x0062_f320, &args![this, 0x40u32]).u32(), 0);
    }

    #[test]
    fn listener_singleton_is_created_once() {
        let mut e = engine();
        let width = SCRATCH + 0x40;
        e.register(0x0043_d4d0, |_, _| ret(SCRATCH + 0x40));
        e.mem.set_u32(width, 2);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        noop(&mut e, &[0x00cb_75d0, 0x0061_ffb0, 0x0048_f200]);
        e.call_log = Some(vec![]);
        let first = e.call(0x0062_f3d0, &args![]).u32();
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first), LISTENER_VTABLE);
        assert_eq!(e.global::<u32>(LISTENER_GLOBAL_011CC54C), first);
        assert_eq!(e.call(0x0062_f3d0, &args![]).u32(), first);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x40], vec![16]]);
    }

    #[test]
    fn water_levels_refresh_keeps_highest_and_base_cells() {
        let mut e = engine();
        let this = listener(&mut e);
        e.set(this, TESWaterListener::uiCellArrayWidth, 2);
        e.set_global(LISTENER_GLOBAL_011CC54C, this.addr());
        let slots = SCRATCH + 0x100;
        e.mem.set_u32(slots, 0xc000);
        e.register(0x0045_7050, |_, a| {
            ret(SCRATCH + 0x100 + 4 * (a[1] * 2 + a[2]))
        });
        e.register(0x0045_18e0, |_, _| ret(1));
        e.register(GET_WATER_HEIGHT, |_, _| st0(5.0));
        e.register(0x0054_4c30, |_, _| ret(-3i32 as u32));
        e.register(0x0054_4c60, |_, _| ret(4));
        e.register(0x0040_36b0, |_, _| ret(0));
        noop(&mut e, &[0x0062_e710, 0x0093_7090]);
        e.call_log = Some(vec![]);
        e.call(0x0062_f460, &args![]);
        assert_eq!(e.get(this, TESWaterListener::fHighestWaterLevel), 5.0);
        let stores = calls(&e, 0x0062_e710);
        assert_eq!(stores.len(), 4);
        assert_eq!(stores[0], vec![this.addr(), bits(5.0), 0, 0]);
        assert_eq!(stores[1], vec![this.addr(), bits(f32::MIN), 0, 1]);
        assert_eq!(stores[2], vec![this.addr(), bits(f32::MIN), 1, 0]);
        assert_eq!(e.get(this, TESWaterListener::iBaseCellNumberX), -3);
        assert_eq!(e.get(this, TESWaterListener::iBaseCellNumberY), 4);
        assert!(calls(&e, 0x0093_7090).is_empty());

        e.mem.set_u32(slots, 0);
        e.call_log = Some(vec![]);
        e.call(0x0062_f460, &args![]);
        assert_eq!(e.get(this, TESWaterListener::fHighestWaterLevel), f32::MIN);
        assert_eq!(calls(&e, 0x0093_7090), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn collidable_callbacks_pass_entity_pointer_to_list() {
        let mut e = engine();
        let this = listener(&mut e);
        e.register(0x004b_59f0, |_, _| ret(0x55));
        noop(
            &mut e,
            &[0x0063_3c90, 0x0063_1540, 0x0063_1620, 0x0045_cec0],
        );
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_f5b0, &args![this, 0x10u32, 0u32]).bool());
        let pointer = calls(&e, 0x0063_3c90)[0][0];
        assert_eq!(calls(&e, 0x0063_3c90), vec![vec![pointer, 0x55]]);
        assert_eq!(
            calls(&e, 0x0063_1540),
            vec![vec![this.addr() + 0xc, pointer]]
        );
        assert_eq!(calls(&e, 0x0045_cec0), vec![vec![pointer]]);
    }

    #[test]
    fn collidable_leave_removes_and_returns_true() {
        let mut e = engine();
        let this = listener(&mut e);
        e.register(0x004b_59f0, |_, _| ret(0x55));
        noop(&mut e, &[0x0063_3c90, 0x0063_1620, 0x0045_cec0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_f630, &args![this, 0x10u32, 0u32]).bool());
        assert_eq!(calls(&e, 0x0063_1620)[0][0], this.addr() + 0xc);
    }

    type RefTable = Rc<Cell<[u32; 2]>>;

    /// `FindCollidableRef`: 0x48 gives the first entry, 0x20 the second.
    fn find_doubles(e: &mut Engine, table: [u32; 2]) -> RefTable {
        let refs = Rc::new(Cell::new(table));
        let shared = refs.clone();
        e.register_double(FIND_COLLIDABLE_REF, move |_, a| {
            let [first, second] = shared.get();
            ret(match a[0] {
                0x48 => first,
                0x20 => second,
                _ => 0,
            })
        });
        refs
    }

    fn contact_event(e: &mut Engine) -> u32 {
        let event = e.mem.alloc(0x4c);
        e.mem.set_u32(event + 0x48, 0x48);
        e.mem.set_u32(event + 0x20, 0x20);
        event
    }

    /// The cast and its checks: the form is an object with the shared
    /// vtable (slot 0x140 gives 0x999).
    fn cast_doubles(e: &mut Engine) -> u32 {
        let form = vtable_object(e, 0x10);
        e.register_double(REFERENCE_GETTER, move |_, _| ret(form));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FLAG_POINTER, |_, _| ret(SCRATCH));
        e.register(CAST_FORM_TEST, |_, _| ret(0));
        e.register(SLOT_140, |_, _| ret(0x999));
        form
    }

    #[test]
    fn contact_added_adds_reference_or_records_player_water() {
        let mut e = engine();
        let this = listener(&mut e);
        let water = vtable_object(&mut e, 0x10);
        let target = vtable_object(&mut e, 0x10);
        e.set_global(PLAYER_POINTER, target);
        let refs = find_doubles(&mut e, [target, water]);
        let form = cast_doubles(&mut e);
        e.register(0x0062_0340, |_, _| ret(0x7777));
        e.register(USE_TASK_QUEUE, |_, _| ret(0));
        e.register(WATER_SYSTEM, |_, _| ret(0x900));
        noop(
            &mut e,
            &[
                REFERENCE_SET_ZONE,
                ADD_TO_WATER_GROUP,
                0x0063_1370,
                0x0057_5c90,
            ],
        );
        let event = contact_event(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_f6b0, &args![this, event]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![form, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert_eq!(e.global::<u32>(WATER_GLOBAL_011C7A3C), 0x999);
        assert_eq!(
            calls(&e, 0x0062_0340),
            vec![vec![this.addr() + 0x14, event, water, target, 0, 0]]
        );
        assert_eq!(calls(&e, REFERENCE_SET_ZONE), vec![vec![target, 0x7777, 1]]);
        assert_eq!(
            calls(&e, ADD_TO_WATER_GROUP),
            vec![vec![0x900, water, target, 0]]
        );
        assert_eq!(
            calls(&e, 0x0063_1370),
            vec![vec![this.addr(), target, 0, 1]]
        );

        // The player's own collidable without a reference.
        refs.set([0, water]);
        e.register(OWN_COLLIDABLE_HOLDER, |_, _| ret(0x8000));
        e.register(COLLIDABLE_OF, |_, _| ret(0x48));
        let bound = e.mem.alloc(16);
        e.mem.set_f32(bound + 8, 7.5);
        e.register_double(SLOT_1F4, move |_, _| ret(bound));
        e.set_global(WATER_GLOBAL_011C7A3C, 0u32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_f6b0, &args![this, event]).bool());
        assert_eq!(e.global::<f32>(HEIGHT_GLOBAL_011C7A5C), 7.5);
        assert!(e.global::<bool>(FLAG_GLOBAL_011C7A5A));
        assert_eq!(e.global::<u32>(WATER_GLOBAL_011C7A3C), 0x999);
        assert_eq!(calls(&e, 0x0057_5c90), vec![vec![1]]);
        assert!(calls(&e, 0x0062_0340).is_empty());

        // The cast check stops it.
        e.register(CAST_FORM_TEST, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_f6b0, &args![this, event]).bool());
        assert!(calls(&e, 0x0057_5c90).is_empty());
    }

    #[test]
    fn global_byte_and_height_setters() {
        let mut e = engine();
        e.call(0x0062_f8e0, &args![true]);
        assert!(e.global::<bool>(FLAG_GLOBAL_011C7A5A));
        e.call(0x0062_f8e0, &args![false]);
        assert!(!e.global::<bool>(FLAG_GLOBAL_011C7A5A));
    }

    #[test]
    fn height_setter_quiets_signalling_nan() {
        let mut e = engine();
        e.call(0x0062_f8f0, &args![2.5f32]);
        assert_eq!(e.global::<f32>(HEIGHT_GLOBAL_011C7A5C), 2.5);
        e.call(0x0062_f8f0, &args![0x7f80_0001u32]);
        assert_eq!(e.global::<u32>(HEIGHT_GLOBAL_011C7A5C), 0x7fc0_0001);
    }

    #[test]
    fn contact_removed_queues_once_or_clears_player_water() {
        let mut e = engine();
        let this = listener(&mut e);
        let water = vtable_object(&mut e, 0x10);
        let target = vtable_object(&mut e, 0x10);
        let refs = find_doubles(&mut e, [target, water]);
        cast_doubles(&mut e);
        e.register(0x0044_0d80, |_, _| ret(0));
        e.register(SLOT_1D0, |_, _| ret(0x10));
        e.register_double(0x0068_38b0, move |_, _| ret(target));
        e.register_double(SLOT_38, move |_, _| ret(target));
        e.register_double(0x006f_a820, move |_, _| ret(target));
        e.register(SLOT_40, |_, _| ret(0x44));
        e.register(0x0055_9450, |_, _| ret(0));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        noop(&mut e, &[0x00ec_61c0]);
        let record = Rc::new(Cell::new(0u32));
        let seen = record.clone();
        e.register_double(0x0047_00d0, move |e, a| {
            seen.set(e.mem.u32(a[1]));
            Ret::default()
        });
        let event = contact_event(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_f900, &args![this, event]).bool());
        let removal: Ptr<QueuedRemoval> = Ptr::new(record.get());
        assert_eq!(e.get(removal, QueuedRemoval::pTarget), target);
        assert_eq!(e.get(removal, QueuedRemoval::pRB), 0x44);
        assert_eq!(e.get(removal, QueuedRemoval::pWaterRef), water);
        assert_eq!(e.get(removal, QueuedRemoval::pWZone), 0);
        assert_eq!(e.get(removal, QueuedRemoval::pCollidable), 0x48);
        assert_eq!(calls(&e, 0x00ec_61c0), vec![vec![record.get(), 0, 0x14]]);
        assert_eq!(calls(&e, 0x0047_00d0)[0][0], this.addr() + 0x34);

        // Already queued under the same key: nothing new.
        let queued = e.mem.alloc(0x14);
        e.mem.set_u32(queued + 4, 0x44);
        e.register(0x0055_9450, |_, _| ret(1));
        e.register_double(0x0063_17a0, move |e, _| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, queued);
            ret(slot)
        });
        e.register(0x007b_52d0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_f900, &args![this, event]).bool());
        assert!(calls(&e, 0x0047_00d0).is_empty());

        // No reference: the player's own collidable clears its water state.
        refs.set([0, water]);
        e.register(REFERENCE_CELL, |_, _| ret(0x600));
        e.register(OWN_COLLIDABLE_HOLDER, |_, _| ret(0x8000));
        e.register(COLLIDABLE_OF, |_, _| ret(0x48));
        e.register(GET_WATER_HEIGHT, |_, _| st0(3.5));
        e.register(0x005d_43c0, |_, _| ret(0x650));
        e.register(0x0042_f180, |_, _| ret(0));
        e.register(GET_WATER_TYPE, |_, _| ret(0x888));
        noop(&mut e, &[0x0057_5c90]);
        e.set_global(FLAG_GLOBAL_011C7A5A, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_f900, &args![this, event]).bool());
        assert_eq!(e.global::<f32>(HEIGHT_GLOBAL_011C7A5C), 3.5);
        assert!(!e.global::<bool>(FLAG_GLOBAL_011C7A5A));
        assert_eq!(e.global::<u32>(WATER_GLOBAL_011C7A3C), 0x888);
        assert_eq!(calls(&e, 0x0057_5c90), vec![vec![0]]);

        // Not the player's collidable: false.
        e.register(COLLIDABLE_OF, |_, _| ret(0x49));
        assert!(!e.call(0x0062_f900, &args![this, event]).bool());
    }

    #[test]
    fn queued_removal_keeps_group_shared_by_another_zone() {
        let mut e = engine();
        let this = listener(&mut e);
        let water = vtable_object(&mut e, 0x10);
        let bound = e.mem.alloc(16);
        e.register_double(SLOT_1F4, move |_, _| ret(bound));
        let zone: Ptr<WaterZone> = e.new_object();
        e.set(zone, WaterZone::pGroup, 0x99);
        let other = SCRATCH + 0x500;
        e.mem.set_u32(other + 0x1c, 0x99);
        e.register(ZONE_FIRST_TARGET, |_, _| ret(0));
        e.register(0x005d_43c0, |_, _| ret(0x650));
        e.register(0x0042_f180, |_, _| ret(0));
        e.register(0x0042_f1d0, |_, _| ret(0x660));
        e.register(0x004b_9ba0, |_, _| ret(1));
        e.register(0x006b_7f20, |e, a| {
            e.mem.set_u32(a[2], SCRATCH + 0x500);
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.register(USE_TASK_QUEUE, |_, _| ret(1));
        e.register(TASK_QUEUE, |_, _| ret(0x4000));
        noop(
            &mut e,
            &[
                OBJECT_SET_LAYER,
                REFERENCE_SET_ZONE,
                REFERENCE_SET_IN_WATER,
                QUEUE_REMOVE_FROM_WATER_GROUP,
            ],
        );
        let removal: Ptr<QueuedRemoval> = e.new_object();
        e.set(removal, QueuedRemoval::pTarget, 0x5000);
        e.set(removal, QueuedRemoval::pRB, 0x44);
        e.set(removal, QueuedRemoval::pWaterRef, water);
        e.set(removal, QueuedRemoval::pWZone, zone.addr());
        e.set(removal, QueuedRemoval::pCollidable, 0x48);
        e.call_log = Some(vec![]);
        e.call(0x0062_fc70, &args![this, removal]);
        assert_eq!(calls(&e, OBJECT_SET_LAYER), vec![vec![0x44, 0x80, 0]]);
        assert_eq!(
            calls(&e, REFERENCE_SET_ZONE),
            vec![vec![0x5000, zone.addr(), 0]]
        );
        assert_eq!(calls(&e, REFERENCE_SET_IN_WATER), vec![vec![0x5000, 0]]);
        assert!(calls(&e, QUEUE_REMOVE_FROM_WATER_GROUP).is_empty());

        e.mem.set_u32(other + 0x1c, 0x98);
        e.call_log = Some(vec![]);
        e.call(0x0062_fc70, &args![this, removal]);
        assert_eq!(
            calls(&e, QUEUE_REMOVE_FROM_WATER_GROUP),
            vec![vec![0x4000, water, 0x5000, 0]]
        );
    }

    #[test]
    fn holder_lookup_feeds_reference_query() {
        let mut e = engine();
        e.register(REFERENCE_CELL, |_, a| ret(a[0] + 1));
        e.register(0x0045_43c0, |_, a| ret(a[0] + 1));
        e.register(0x0059_bb30, |_, a| ret(a[0] + 0x100));
        e.register(0x0057_6830, |_, a| ret(a[0] + a[1]));
        assert_eq!(
            e.call(0x0062_fe80, &args![0x10u32]).u32(),
            0x112 + 0x14 + 0x10
        );
        assert_eq!(e.call(0x0062_fe80, &args![0u32]).u32(), 0);
    }

    #[test]
    fn holder_chain_stops_at_first_null() {
        let mut e = engine();
        e.register(REFERENCE_CELL, |_, a| ret(a[0] + 1));
        e.register(0x0045_43c0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_fec0, &args![0x10u32]).u32(), 0);
        assert!(calls(&e, 0x0059_bb30).is_empty());
        e.register(0x0045_43c0, |_, _| ret(0x20));
        e.register(0x0059_bb30, |_, _| ret(0x30));
        assert_eq!(e.call(0x0062_fec0, &args![0x10u32]).u32(), 0x30);
    }

    fn water_test_doubles(e: &mut Engine) {
        e.register(BODY_OBJECT, |_, _| ret(0x10));
        e.register(0x004b_4ec0, |_, _| ret(0x20));
        e.register(SCALE_FLOAT, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(VECTOR_TERM, |_, _| ret(0x40));
        noop(e, &[VECTOR_COPY, 0x0045_8a10, 0x0062_b0a0, 0x00c9_d600]);
        setting(e, 0x011d_1248, 0.5);
    }

    fn lengths(e: &mut Engine, values: Vec<f64>) {
        let mut index = 0;
        e.register_double(VECTOR_LENGTH, move |_, _| {
            let value = values[index];
            index += 1;
            st0(value)
        });
    }

    #[test]
    fn water_test_checks_range_and_level() {
        let mut e = engine();
        let this = listener(&mut e);
        water_test_doubles(&mut e);
        let level = e.mem.alloc(4);
        assert!(!e
            .call(0x0062_ff10, &args![this, 0u32, 0u32, 5.0f32, 0u32, level])
            .bool());

        // Out of range: an active object with layer 0x88 is deactivated.
        lengths(&mut e, vec![10.0]);
        e.register(OBJECT_HAS_LAYER, |_, _| ret(1));
        e.register(0x00c9_bff0, |_, _| ret(0x30));
        e.register(0x0056_09f0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(
                0x0062_ff10,
                &args![this, 0x50u32, 0x60u32, 5.0f32, 0u32, level]
            )
            .bool());
        assert_eq!(calls(&e, 0x00c9_d600), vec![vec![0x10]]);
        assert_eq!(calls(&e, OBJECT_HAS_LAYER), vec![vec![0x10, 0x88]]);

        // In a zone: the zone's height; top above level + margin asks the layer.
        let zone: Ptr<WaterZone> = e.new_object();
        e.set(zone, WaterZone::fWaterHeight, 2.0);
        lengths(&mut e, vec![1.0, 3.0]);
        e.register(OBJECT_HAS_LAYER, |_, _| ret(0));
        assert!(!e
            .call(
                0x0062_ff10,
                &args![this, 0x50u32, 0x60u32, 5.0f32, zone, level]
            )
            .bool());
        assert_eq!(e.mem.f32(level), 2.0);
        lengths(&mut e, vec![1.0, 2.5]);
        assert!(e
            .call(
                0x0062_ff10,
                &args![this, 0x50u32, 0x60u32, 5.0f32, zone, level]
            )
            .bool());

        // Cell table: cell (2 - 1, 1 - 1) of width 2 is index 1.
        e.set(this, TESWaterListener::uiCellArrayWidth, 2);
        e.set(this, TESWaterListener::iBaseCellNumberX, 1);
        e.set(this, TESWaterListener::iBaseCellNumberY, 1);
        let table = e.mem.alloc(16);
        e.mem.set_f32(table + 4, 6.0);
        e.set(this, TESWaterListener::pfWaterLevels, table);
        e.register(0x0045_8620, |e, a| {
            e.mem.set_f32(a[0], 8192.0);
            e.mem.set_f32(a[0] + 4, 4096.0);
            Ret::default()
        });
        e.register(0x0040_6d90, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        lengths(&mut e, vec![1.0, 2.0]);
        assert!(e
            .call(
                0x0062_ff10,
                &args![this, 0x50u32, 0x60u32, 5.0f32, 0u32, level]
            )
            .bool());
        assert_eq!(e.mem.f32(level), 6.0);
    }

    #[test]
    fn difference_helper_returns_out() {
        let mut e = engine();
        noop(&mut e, &[0x0062_b0a0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_0100, &args![0x10u32, 0x20u32]).u32(), 0x20);
        assert_eq!(calls(&e, 0x0062_b0a0), vec![vec![0x10, 0x20, 0x10]]);
    }

    #[test]
    fn cell_level_lookup_bounds_index() {
        let mut e = engine();
        let this = listener(&mut e);
        e.set(this, TESWaterListener::uiCellArrayWidth, 2);
        let table = e.mem.alloc(16);
        e.mem.set_f32(table + 12, 9.0);
        e.set(this, TESWaterListener::pfWaterLevels, table);
        assert_eq!(e.call(0x0063_0120, &args![this, 1i32, 1i32]).f32(), 9.0);
        assert_eq!(
            e.call(0x0063_0120, &args![this, 0i32, 2i32]).f32(),
            f32::MIN
        );
        assert_eq!(
            e.call(0x0063_0120, &args![this, -1i32, 0i32]).f32(),
            f32::MIN
        );
    }

    fn buoyancy_doubles(e: &mut Engine) -> (u32, u32) {
        let object = vtable_object(e, 0x10);
        let motion = vtable_object(e, 0x10);
        e.register(0x004b_4ec0, |_, _| ret(0x20));
        e.register_double(BODY_OBJECT, move |_, _| ret(object));
        e.register(SLOT_F8, |e, a| {
            e.mem.set_f32(a[1] + 8, 0.0);
            e.mem.set_f32(a[1] + 0x18, 4.0);
            Ret::default()
        });
        e.register(CONVERT_FLOAT, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        noop(
            e,
            &[
                VECTOR_COPY,
                0x0043_7c10,
                OBJECT_SET_LAYER,
                0x0084_d310,
                RELEASE,
            ],
        );
        e.register(0x0051_7670, |_, _| ret(0));
        e.register(0x0046_0140, |_, _| ret(0x70));
        e.register(0x0056_0dc0, |_, _| ret(0x80));
        e.register(0x004b_4ea0, |_, _| st0(2.0));
        e.register(0x0062_b310, |_, _| st0(1.0));
        e.register(VECTOR_TERM, |_, _| ret(0x90));
        e.register(VECTOR_LENGTH, |_, _| st0(1.0));
        e.register(0x0040_8840, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(0x0056_1240, |_, a| ret(a[1]));
        e.register(0x00c9_c380, |_, _| ret(1));
        e.register_double(0x009d_9f40, move |_, _| ret(motion));
        e.register(0x0087_eaa0, |_, _| ret(0x100));
        e.register(0x0062_b0a0, |_, a| ret(a[1]));
        e.register(0x0056_0e50, |_, _| ret(0x200));
        e.register(0x00ec_ae30, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).exp())
        });
        noop(
            e,
            &[
                VECTOR_FROM_FLOAT,
                0x004a_3f10,
                0x00c9_c1d0,
                0x0056_16d0,
                VECTOR_MULTIPLY,
                0x0056_1800,
            ],
        );
        setting(e, 0x011d_0cf0, 5.0);
        setting(e, 0x011d_0580, 0.5);
        (object, motion)
    }

    #[test]
    fn buoyancy_paths_return_status() {
        let mut e = engine();
        let this = listener(&mut e);
        let (object, _) = buoyancy_doubles(&mut e);
        // Below the bounds: a splash out (layer present) gives 2.
        e.register(OBJECT_HAS_LAYER, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0063_0170, &args![this, 0x30u32, -1.0f32, 0.1f32, false])
                .u32(),
            2
        );
        assert_eq!(calls(&e, OBJECT_SET_LAYER), vec![vec![object, 8, 0]]);

        // In water without an active body: sets the layer, 1.
        e.register(OBJECT_HAS_LAYER, |_, _| ret(0));
        e.register(0x00c8_cce0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0063_0170, &args![this, 0x30u32, 2.0f32, 0.1f32, true])
                .u32(),
            1
        );
        assert_eq!(calls(&e, OBJECT_SET_LAYER), vec![vec![object, 0x80, 1]]);
        assert_eq!(
            calls(&e, 0x0084_d310),
            vec![vec![BODY_MAP_GLOBAL_011CC584, 0x30, 1]]
        );

        // Active body, below a quarter of its height: 0 at once.
        e.register(0x00c8_cce0, |_, _| ret(1));
        e.register(ARRAY_SIZE, |_, _| ret(0x58));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0063_0170, &args![this, 0x30u32, 0.5f32, 0.1f32, false])
                .u32(),
            0
        );
        assert!(calls(&e, 0x0084_d310).is_empty());
    }

    #[test]
    fn buoyancy_applies_lift_weight_and_damping() {
        let mut e = engine();
        let this = listener(&mut e);
        let (_, motion) = buoyancy_doubles(&mut e);
        e.register(OBJECT_HAS_LAYER, |_, _| ret(1));
        e.register(0x00c8_cce0, |_, _| ret(1));
        e.register(ARRAY_SIZE, |_, _| ret(0x58));
        e.call_log = Some(vec![]);
        // Ratio (2 - 1) / 2.4 lies in the lowest band: factor -0.3, negated.
        assert_eq!(
            e.call(0x0063_0170, &args![this, 0x30u32, 2.0f32, 0.1f32, false])
                .u32(),
            0
        );
        let floats: Vec<u32> = calls(&e, VECTOR_FROM_FLOAT).iter().map(|w| w[1]).collect();
        let lift = (f64::from(0.3f32) * 2.0) as f32;
        let rate = (-0.5 * f64::from(0.1f32)) as f32;
        let angular = f64::from(rate).exp() as f32;
        assert_eq!(floats, vec![bits(lift), bits(-2.0), bits(angular)]);
        assert_eq!(calls(&e, 0x004a_3f10).len(), 1);
        assert_eq!(calls(&e, SLOT_58)[0][..2], [motion, bits(0.1)]);
        assert_eq!(calls(&e, SLOT_5C)[0][..2], [motion, bits(0.1)]);
        assert_eq!(calls(&e, 0x0056_16d0).len(), 1);
        assert_eq!(calls(&e, 0x0056_1800).len(), 1);
        assert_eq!(calls(&e, 0x0084_d310).len(), 1);
        assert_eq!(calls(&e, RELEASE).len(), 4);
    }

    #[test]
    fn exp_wrapper_passes_float_through() {
        let mut e = engine();
        e.register(0x00ec_ae30, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) * 2.0)
        });
        assert_eq!(e.call(0x0063_0aa0, &args![1.25f32]).f32(), 2.5);
    }

    #[test]
    fn exp_call_takes_a_double_and_rounds_to_float() {
        let mut e = engine();
        e.register(0x00ec_ae30, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).exp())
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0063_0ac0, &args![1.0f32]).f32();
        assert_eq!(result, std::f64::consts::E as f32);
        assert_eq!(calls(&e, 0x00ec_ae30)[0].len(), 2);
    }

    fn put4(e: &mut Engine, value: Vec4) -> u32 {
        let address = e.mem.alloc(16);
        store4(e, address, value);
        address
    }

    #[test]
    fn scaled_vector_multiplies_lanes() {
        let mut e = engine();
        let source = put4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.register_double(0x0056_1240, move |_, _| ret(source));
        let b = put4(&mut e, [2.0, 3.0, 4.0, 5.0]);
        let out = put4(&mut e, [0.0; 4]);
        e.call(0x0063_0ae0, &args![out, 0x10u32, b]);
        assert_eq!(load4(&e, out), [2.0, 6.0, 12.0, 20.0]);
    }

    #[test]
    fn root_vector_takes_square_root_of_first_lane() {
        let mut e = engine();
        e.register(0x0062_b0a0, |e, a| {
            store4(e, a[1], [16.0, 2.0, 3.0, 4.0]);
            ret(a[1])
        });
        let copied = Rc::new(Cell::new([0f32; 4]));
        let seen = copied.clone();
        e.register_double(VECTOR_COPY, move |e, a| {
            seen.set(load4(e, a[1]));
            Ret::default()
        });
        assert_eq!(e.call(0x0063_0b40, &args![0x10u32, 0x20u32]).u32(), 0x20);
        assert_eq!(copied.get(), [4.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn torque_builder_combines_helpers() {
        let mut e = engine();
        store4(&mut e, VECTOR_010C71E0, [1.0, 1.0, 1.0, 1.0]);
        let a = put4(&mut e, [1.0, 0.0, 0.0, 2.0]);
        let b = put4(&mut e, [0.0, 1.0, 0.0, 0.0]);
        e.register(0x0062_b0a0, |e, a| {
            store4(e, a[1], [3.0, 0.0, 0.0, 0.0]);
            ret(a[1])
        });
        let written = Rc::new(Cell::new([0f32; 4]));
        let seen = written.clone();
        e.register_double(0x0045_8880, move |e, a| {
            assert_eq!(a[1], a[2]);
            seen.set(load4(e, a[1]));
            Ret::default()
        });
        e.call(0x0063_0bb0, &args![0x10u32, a, b]);
        // t = b * (1 + 2 * 2) + 3 (lane 0 broadcast) * a + 2 * (a x b).
        let cross = [0.0, 0.0, 1.0, 0.0];
        let expected = [
            5.0 * 0.0 + 3.0 * 1.0 + 2.0 * cross[0],
            5.0 * 1.0 + 3.0 * 0.0 + 2.0 * cross[1],
            5.0 * 0.0 + 3.0 * 0.0 + 2.0 * cross[2],
            5.0 * 0.0 + 3.0 * 2.0 + 2.0 * cross[3],
        ];
        assert_eq!(written.get(), expected);
    }

    #[test]
    fn lane_product() {
        let mut e = engine();
        let a = put4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = put4(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let out = put4(&mut e, [0.0; 4]);
        e.call(0x0063_0ce0, &args![out, a, b]);
        assert_eq!(load4(&e, out), [5.0, 12.0, 21.0, 32.0]);
    }

    #[test]
    fn lane_multiply_add_in_place() {
        let mut e = engine();
        let a = put4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = put4(&mut e, [2.0, 2.0, 2.0, 2.0]);
        let out = put4(&mut e, [1.0, 1.0, 1.0, 1.0]);
        e.call(0x0063_0d30, &args![out, a, b]);
        assert_eq!(load4(&e, out), [3.0, 5.0, 7.0, 9.0]);
    }

    #[test]
    fn lane_multiply_add() {
        let mut e = engine();
        let a = put4(&mut e, [1.0, 1.0, 1.0, 1.0]);
        let b = put4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let c = put4(&mut e, [3.0, 3.0, 3.0, 3.0]);
        let out = put4(&mut e, [0.0; 4]);
        e.call(0x0063_0d90, &args![out, a, b, c]);
        assert_eq!(load4(&e, out), [4.0, 7.0, 10.0, 13.0]);
    }

    #[test]
    fn cross_product_lanes() {
        let mut e = engine();
        let a = put4(&mut e, [1.0, 0.0, 0.0, 5.0]);
        let b = put4(&mut e, [0.0, 1.0, 0.0, 7.0]);
        let out = put4(&mut e, [9.0; 4]);
        e.call(0x0063_0df0, &args![out, a, b]);
        assert_eq!(load4(&e, out), [0.0, 0.0, 1.0, 0.0]);
        let c = put4(&mut e, [2.0, 3.0, 4.0, 0.0]);
        let d = put4(&mut e, [5.0, 6.0, 7.0, 0.0]);
        e.call(0x0063_0df0, &args![out, c, d]);
        assert_eq!(load4(&e, out), [-3.0, 6.0, -3.0, 0.0]);
    }

    #[test]
    fn broadcast_in_place() {
        let mut e = engine();
        let v = put4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x0063_0ed0, &args![v, 2u32]);
        assert_eq!(load4(&e, v), [3.0; 4]);
        e.call(0x0063_0ed0, &args![v, 4u32]);
        assert_eq!(load4(&e, v), [3.0; 4]);
    }

    #[test]
    fn broadcast_from_source() {
        let mut e = engine();
        let v = put4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let out = put4(&mut e, [0.0; 4]);
        e.call(0x0063_0fa0, &args![out, v, 3u32]);
        assert_eq!(load4(&e, out), [4.0; 4]);
        e.call(0x0063_0fa0, &args![out, v, 9u32]);
        assert_eq!(load4(&e, out), [4.0; 4]);
    }

    #[test]
    fn sign_of_float() {
        let mut e = engine();
        assert_eq!(e.call(0x0063_1070, &args![2.0f32]).f32(), 1.0);
        assert_eq!(e.call(0x0063_1070, &args![-3.0f32]).f32(), -1.0);
        assert_eq!(e.call(0x0063_1070, &args![0.0f32]).f32(), 0.0);
        assert_eq!(e.call(0x0063_1070, &args![f32::NAN]).f32(), 0.0);
    }

    #[test]
    fn motion_point_chain() {
        let mut e = engine();
        e.register(0x009d_9f40, |_, a| ret(a[0] + 1));
        e.register(0x0087_eaa0, |_, a| ret(a[0] * 2));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0063_10c0, &args![0x10u32]).u32(),
            (0x11 + 0x10) * 2 + 0x10
        );
        assert_eq!(calls(&e, 0x0087_eaa0), vec![vec![0x21]]);
    }

    #[test]
    fn motion_point_offsets() {
        let mut e = engine();
        e.register(0x0087_eaa0, |_, a| ret(a[0]));
        assert_eq!(e.call(0x0063_10e0, &args![0x100u32]).u32(), 0x120);
    }

    #[test]
    fn apply_force_activates_then_calls_motion() {
        let mut e = engine();
        let motion = vtable_object(&mut e, 0x10);
        e.register_double(0x009d_9f40, move |_, _| ret(motion));
        noop(&mut e, &[0x00c9_c1d0]);
        e.call_log = Some(vec![]);
        e.call(0x0063_1100, &args![0x30u32, 0.5f32, 0x40u32]);
        assert_eq!(calls(&e, 0x00c9_c1d0), vec![vec![0x30]]);
        assert_eq!(calls(&e, SLOT_5C), vec![vec![motion, bits(0.5), 0x40]]);
    }

    #[test]
    fn apply_force_at_point_uses_slot_58() {
        let mut e = engine();
        let motion = vtable_object(&mut e, 0x10);
        e.register_double(0x009d_9f40, move |_, _| ret(motion));
        noop(&mut e, &[0x00c9_c1d0]);
        e.call_log = Some(vec![]);
        e.call(0x0063_1140, &args![0x30u32, 0.5f32, 0x40u32, 0x50u32]);
        assert_eq!(
            calls(&e, SLOT_58),
            vec![vec![motion, bits(0.5), 0x40, 0x50]]
        );
    }

    #[test]
    fn collidable_array_constructor() {
        let mut e = engine();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x006b_3eb0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_1750, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), COLLIDABLE_ARRAY_VTABLE);
        assert_eq!(calls(&e, 0x006b_3eb0), vec![vec![array.addr(), 0, 0]]);
    }

    #[test]
    fn collidable_array_destructor() {
        let mut e = engine();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0084_54f0]);
        e.call_log = Some(vec![]);
        e.call(0x0063_1780, &args![array]);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn collidable_array_scalar_deleting_destructor() {
        let mut e = engine();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0084_54f0, FREE]);
        e.call_log = Some(vec![]);
        e.call(0x0063_17c0, &args![array, 0u32]);
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0063_17c0, &args![array, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![array.addr()]]);
    }

    #[test]
    fn iterator_constructor_clears_second_word() {
        let mut e = engine();
        let it: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(it.addr() + 4, 0x55);
        noop(&mut e, &[0x0055_9a40]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_17f0, &args![it, 0x77u32]).u32(), it.addr());
        assert_eq!(e.mem.u32(it.addr() + 4), 0);
        assert_eq!(calls(&e, 0x0055_9a40), vec![vec![it.addr(), 0x77]]);
    }

    #[test]
    fn first_words_compare() {
        let mut e = engine();
        let a: Ptr = Ptr::new(e.mem.alloc(4));
        let b: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(a.addr(), 3);
        e.mem.set_u32(b.addr(), 3);
        assert_eq!(e.call(0x0063_1820, &args![a, b]).u32(), 0);
        e.mem.set_u32(b.addr(), 4);
        assert_eq!(e.call(0x0063_1820, &args![a, b]).u32(), 1);
    }

    #[test]
    fn zone_listener_thunk_adjusts_this() {
        let mut e = engine();
        let this = listener(&mut e);
        noop(&mut e, &[0x0062_e860, FREE]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0063_1840, &args![this.addr() + 0x14, 1u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(calls(&e, 0x0062_e860), vec![vec![this.addr()]]);
    }

    #[test]
    fn body_map_constructor_sets_map_vtable_last() {
        let mut e = engine();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x00aa_1070, |e, a| ret(e.mem.alloc(a[0])));
        noop(&mut e, &[0x0040_3d30]);
        assert_eq!(e.call(0x0063_1850, &args![map, 37u32]).u32(), map.addr());
        assert_eq!(e.mem.u32(map.addr()), BODY_MAP_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 4), 37);
    }

    #[test]
    fn body_map_scalar_deleting_destructor() {
        let mut e = engine();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0, FREE]);
        e.call_log = Some(vec![]);
        e.call(0x0063_1880, &args![map, 0u32]);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(e.mem.u32(map.addr()), BODY_MAP_BASE_VTABLE);
        e.call(0x0063_1880, &args![map, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![map.addr()]]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 4);
    }

    #[test]
    fn body_map_base_constructor_reloads_after_allocation() {
        let mut e = engine();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.register_double(0x00aa_1070, move |e, a| {
            assert_eq!(a, &[8]);
            e.mem.set_u32(map.addr() + 4, 3);
            ret(0x1234)
        });
        noop(&mut e, &[0x0040_3d30]);
        e.call_log = Some(vec![]);
        e.call(0x0063_18b0, &args![map, 2u32]);
        assert_eq!(e.mem.u32(map.addr()), BODY_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 8), 0x1234);
        assert_eq!(calls(&e, 0x0040_3d30), vec![vec![0x1234, 0, 12]]);
    }

    #[test]
    fn body_map_destructor_runs_base() {
        let mut e = engine();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0063_1920, &args![map]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(e.mem.u32(map.addr()), BODY_MAP_BASE_VTABLE);
    }

    #[test]
    fn body_map_base_destructor_frees_table() {
        let mut e = engine();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(map.addr() + 8, 0x9999);
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0063_1980, &args![map]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![map.addr()]]);
        assert_eq!(calls(&e, 0x00aa_10f0), vec![vec![0x9999]]);
    }

    #[test]
    fn body_map_base_scalar_deleting_destructor() {
        let mut e = engine();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0, FREE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_19b0, &args![map, 2u32]).u32(), map.addr());
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0063_19b0, &args![map, 3u32]);
        assert_eq!(calls(&e, FREE), vec![vec![map.addr()]]);
    }
}
