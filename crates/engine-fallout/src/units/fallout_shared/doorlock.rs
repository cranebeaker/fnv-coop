//! `fallout shared/doorlock.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `DoorLock` is the pathfinder's copy of what a door reference says
//! about getting through it (its cell, the cell it leads to, its lock, its
//! owner); an `ActorLockData` is what an actor brings (its base object,
//! its inventory, whether it is alarmed).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::extradataobjects::RefrLock;

layout! {
    /// `DoorLock` (Xbox PDB), 0x18 bytes (the PC offsets agree).
    pub struct DoorLock: 0x18 {
        /// `pDoorCell` (Xbox PDB): `TESObjectCELL*`.
        0x00 pDoorCell: u32,
        /// `pTeleportDoorCell` (Xbox PDB): `TESObjectCELL*`.
        0x04 pTeleportDoorCell: u32,
        /// `pLockData` (Xbox PDB): `REFR_LOCK*`, an owned copy.
        0x08 pLockData: Ptr<RefrLock>,
        /// `pOwnershipForm` (Xbox PDB): `TESForm*`.
        0x0C pOwnershipForm: u32,
        /// `iOwnershipRank` (Xbox PDB).
        0x10 iOwnershipRank: i32,
        /// `pOwnershipGlobal` (Xbox PDB): `TESGlobal*`.
        0x14 pOwnershipGlobal: u32,
    }

    /// `ActorLockData` (Xbox PDB), 0xC bytes (the PC offsets agree).
    pub struct ActorLockData: 0x0C {
        /// `pActorBase` (Xbox PDB): `TESActorBase*`.
        0x00 pActorBase: u32,
        /// `pInventory` (Xbox PDB): `InventoryChanges*`.
        0x04 pInventory: u32,
        /// `bIsAlarmed` (Xbox PDB).
        0x08 bIsAlarmed: u8,
    }
}

/// A reference's parent cell (`008d6f30`: the word at `+0x40`).
const PARENT_CELL: u32 = 0x008d_6f30;
/// A reference's door teleport data (`00568e50`).
const TELEPORT_DATA: u32 = 0x0056_8e50;
/// `DoorTeleportData::GetTeleportCell` (Xbox PDB, `0043a2b0`).
const GET_TELEPORT_CELL: u32 = 0x0043_a2b0;
/// `TESObjectREFR::GetLock` (Xbox PDB, `00569160`).
const GET_LOCK: u32 = 0x0056_9160;
/// The `MemoryManager` allocate wrapper (`00401000`, cdecl).
const ALLOCATE: u32 = 0x0040_1000;
/// The `MemoryManager` free wrapper (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;
/// Zeroes a `REFR_LOCK` (`00411b00`).
const LOCK_CONSTRUCT: u32 = 0x0041_1b00;
/// `TESObjectREFR::GetOwner`, `GetOwnershipRank`, `GetOwnershipGlobal`
/// (Xbox PDB).
const GET_OWNER: u32 = 0x0056_7790;
const GET_OWNERSHIP_RANK: u32 = 0x0056_79f0;
const GET_OWNERSHIP_GLOBAL: u32 = 0x0056_7960;
/// The player pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// `TESObjectCELL::GetOwner` (Xbox PDB, `00546a40`).
const CELL_GET_OWNER: u32 = 0x0054_6a40;
/// A cell's ownership test for an actor (`00546ca0`, `RET 4`).
const CELL_OWNED_BY_ACTOR: u32 = 0x0054_6ca0;
/// A cell's ownership test for an actor base (`00546ee0`, `RET 4`).
const CELL_OWNED_BY_BASE: u32 = 0x0054_6ee0;
/// A lock's `cFlags` bit 0 (`005021a0`).
const LOCK_FLAG_0: u32 = 0x0050_21a0;
/// `Actor::IsAlarmed` (Xbox PDB, `008a61b0`).
const IS_ALARMED: u32 = 0x008a_61b0;
/// Bit 0 of a cell's flag byte at `+0x24` (`00425fd0`).
const CELL_FLAG_0: u32 = 0x0042_5fd0;
/// Bit 0x20 of a cell's flag byte at `+0x24` (`00502180`).
const CELL_FLAG_20: u32 = 0x0050_2180;
/// The form type byte of a form (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `BGSSaveFormBuffer::GetForm` (Xbox PDB name of `007af430`).
const SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `.?AVTESBoundObject@@`, `.?AVTESActorBase@@`.
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
/// `TESActorBaseData::GetFactionRank` (Xbox PDB, `0047d680`, `RET 8`).
const GET_FACTION_RANK: u32 = 0x0047_d680;
/// The value of a `TESGlobal` (`00526ac0`, `ST0`).
const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// The `double` the global's value is compared with (`01012060`, 0.0).
const ZERO: u32 = 0x0101_2060;
/// `InventoryChanges::ContainerHasObjects` (Xbox PDB, `004cffe0`, six
/// words).
const CONTAINER_HAS_OBJECTS: u32 = 0x004c_ffe0;
/// `REFR_LOCK::GetLockLevel` (Xbox PDB, `004309e0`, `RET 4`).
const GET_LOCK_LEVEL: u32 = 0x0043_09e0;
/// The form `CanActorUnlockDoorLock` looks for in the inventory when the
/// key is missing (`011ca268`; not identified here).
const UNLOCK_ITEM: u32 = 0x011c_a268;

/// The door's cell, the teleport cell, the owner fields (the parts the
/// constructor and `SetFromDoorRef` share, in their order, with the lock
/// handled between by `lock`).
fn read_door(
    e: &mut Engine,
    this: Ptr<DoorLock>,
    door: Ptr,
    lock: impl FnOnce(&mut Engine, Ptr<DoorLock>, Ptr<RefrLock>),
) {
    let cell = e.call(PARENT_CELL, &args![door]).u32();
    e.set(this, DoorLock::pDoorCell, cell);
    let teleport = e.call(TELEPORT_DATA, &args![door]).u32();
    let teleport_cell = if teleport != 0 {
        e.call(GET_TELEPORT_CELL, &args![teleport]).u32()
    } else {
        0
    };
    e.set(this, DoorLock::pTeleportDoorCell, teleport_cell);
    let door_lock = e.call(GET_LOCK, &args![door]).ptr::<RefrLock>();
    lock(e, this, door_lock);
    let owner = e.call(GET_OWNER, &args![door]).u32();
    e.set(this, DoorLock::pOwnershipForm, owner);
    let rank = e.call(GET_OWNERSHIP_RANK, &args![door]).i32();
    e.set(this, DoorLock::iOwnershipRank, rank);
    let global = e.call(GET_OWNERSHIP_GLOBAL, &args![door]).u32();
    e.set(this, DoorLock::pOwnershipGlobal, global);
}

/// A new zeroed `REFR_LOCK` (`00401000` then `00411b00`), null when the
/// allocation fails.
fn new_lock(e: &mut Engine) -> Ptr<RefrLock> {
    let memory = e.call(ALLOCATE, &args![0x14u32]).u32();
    if memory == 0 {
        Ptr::new(0)
    } else {
        e.call(LOCK_CONSTRUCT, &args![memory]).ptr()
    }
}

/// Copies the key, the base level and the flags of `source` into the
/// owned lock copy (in that order, through `pLockData` each time).
fn copy_lock(e: &mut Engine, this: Ptr<DoorLock>, source: Ptr<RefrLock>) {
    let key = e.get(source, RefrLock::pKey);
    let copy = e.get(this, DoorLock::pLockData);
    e.set(copy, RefrLock::pKey, key);
    let level = e.get(source, RefrLock::cBaseLevel);
    let copy = e.get(this, DoorLock::pLockData);
    e.set(copy, RefrLock::cBaseLevel, level);
    let flags = e.get(source, RefrLock::cFlags);
    let copy = e.get(this, DoorLock::pLockData);
    e.set(copy, RefrLock::cFlags, flags);
}

// Translated from 00501d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00501d20 - the exception-unwinding frame (FS:[0]) around the lock allocation and its security-cookie word are not translated.
/// `DoorLock::DoorLock` (Xbox PDB): fills the lock from a door reference:
/// its parent cell (`008d6f30`), the teleport cell of its teleport data
/// (`00568e50`, `DoorTeleportData::GetTeleportCell`; null without data), a
/// new copy of its lock's key, base level and flags (null without a lock)
/// and its owner, ownership rank and ownership global. Returns `this`.
pub fn door_lock_door_lock(e: &mut Engine, this: Ptr<DoorLock>, door: Ptr) -> Ptr<DoorLock> {
    read_door(e, this, door, |e, this, door_lock| {
        if door_lock.is_null() {
            e.set(this, DoorLock::pLockData, Ptr::new(0));
        } else {
            let copy = new_lock(e);
            e.set(this, DoorLock::pLockData, copy);
            copy_lock(e, this, door_lock);
        }
    });
    this
}

// Translated from 00501e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorLock::~DoorLock` (Xbox PDB; Ghidra's library match
/// `TaskStack::~TaskStack` is the same code): frees the lock copy
/// (`00401030`) when there is one.
pub fn door_lock_destructor(e: &mut Engine, this: Ptr<DoorLock>) {
    let copy = e.get(this, DoorLock::pLockData);
    if !copy.is_null() {
        e.call(FREE, &args![copy]);
    }
}

// Translated from 00501e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00501e80 - the exception-unwinding frame (FS:[0]) around the lock allocation and its security-cookie word are not translated.
/// `DoorLock::SetFromDoorRef` (Xbox PDB): as the constructor, but the lock
/// copy is kept: made only when missing and the door has a lock, freed
/// (`00401030`) and cleared when the door has none.
pub fn door_lock_set_from_door_ref(e: &mut Engine, this: Ptr<DoorLock>, door: Ptr) {
    read_door(e, this, door, |e, this, door_lock| {
        if door_lock.is_null() {
            let copy = e.get(this, DoorLock::pLockData);
            if !copy.is_null() {
                e.call(FREE, &args![copy]);
                e.set(this, DoorLock::pLockData, Ptr::new(0));
            }
        } else {
            if e.get(this, DoorLock::pLockData).is_null() {
                let copy = new_lock(e);
                e.set(this, DoorLock::pLockData, copy);
            }
            copy_lock(e, this, door_lock);
        }
    });
}

// Translated from 00501fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorLock::CanActorIgnoreDoorLock` (Xbox PDB): whether `actor` may go
/// through the door regardless of the lock. False for no actor. True when
/// `allow_unlocked` is set and the door has no lock or a lock without
/// `cFlags` bit 0. For an actor other than the player in an owned cell
/// (`TESObjectCELL::GetOwner`) that `00546ca0` accepts for it, the answer
/// is decided there: true unless the teleport cell is owned and
/// `00546ca0` refuses the actor for it. Otherwise true when, for an actor
/// other than the player, `005021c0` accepts it (with true); when vtable
/// slot 0x304 and `Actor::IsAlarmed` both hold; or, for the player, when
/// slot 0x448 holds, the door leads somewhere, has a lock whose base level
/// is not 100 (0x64), and the teleport cell lacks flag bit 0 or has flag
/// 0x20 (`00425fd0`, `00502180`). The second word is never read.
pub fn door_lock_can_actor_ignore_door_lock(
    e: &mut Engine,
    this: Ptr<DoorLock>,
    actor: u32,
    _unused_1: u32,
    allow_unlocked: bool,
) -> bool {
    if actor == 0 {
        return false;
    }
    let lock = e.get(this, DoorLock::pLockData);
    if lock.is_null() && allow_unlocked {
        return true;
    }
    if !lock.is_null() && !e.call(LOCK_FLAG_0, &args![lock]).bool() && allow_unlocked {
        return true;
    }
    let player = e.global::<u32>(PLAYER);
    if actor != player && e.call(PARENT_CELL, &args![actor]).u32() != 0 {
        let cell = e.call(PARENT_CELL, &args![actor]).u32();
        if e.call(CELL_GET_OWNER, &args![cell]).u32() != 0 {
            let cell = e.call(PARENT_CELL, &args![actor]).u32();
            if e.call(CELL_OWNED_BY_ACTOR, &args![cell, actor]).bool() {
                let teleport_cell = e.get(this, DoorLock::pTeleportDoorCell);
                return teleport_cell == 0
                    || e.call(CELL_GET_OWNER, &args![teleport_cell]).u32() == 0
                    || e.call(CELL_OWNED_BY_ACTOR, &args![teleport_cell, actor])
                        .bool();
            }
        }
    }
    if actor != e.global::<u32>(PLAYER) && fn_005021c0(e, this, actor, true) {
        return true;
    }
    if e.vcall(actor, 0x304, &args![]).bool() && e.call(IS_ALARMED, &args![actor]).bool() {
        return true;
    }
    if actor != e.global::<u32>(PLAYER) {
        return false;
    }
    let player = e.global::<u32>(PLAYER);
    if !e.vcall(player, 0x448, &args![]).bool() {
        return false;
    }
    let teleport_cell = e.get(this, DoorLock::pTeleportDoorCell);
    if teleport_cell == 0 {
        return false;
    }
    let lock = e.get(this, DoorLock::pLockData);
    if lock.is_null() || e.get(lock, RefrLock::cBaseLevel) == 0x64 {
        return false;
    }
    let teleport_cell = e.get(this, DoorLock::pTeleportDoorCell);
    if !e.call(CELL_FLAG_0, &args![teleport_cell]).bool() {
        return true;
    }
    let teleport_cell = e.get(this, DoorLock::pTeleportDoorCell);
    e.call(CELL_FLAG_20, &args![teleport_cell]).bool()
}

/// The ownership-global part of `005021c0` and `00502550`: false when
/// there is no global or its value equals the `double` at `01012060`
/// (`FCOMP`, `TEST AH,0x44`, `JP`: a NaN keeps the answer).
fn global_refuses(e: &mut Engine, this: Ptr<DoorLock>) -> bool {
    let global = e.get(this, DoorLock::pOwnershipGlobal);
    if global == 0 {
        return true;
    }
    let value = e.call(GLOBAL_VALUE, &args![global]).f32();
    value as f64 == e.global::<f64>(ZERO)
}

/// The owner when it is of form type 8 (the faction case), else 0.
fn owner_faction(e: &mut Engine, this: Ptr<DoorLock>) -> u32 {
    let owner = e.get(this, DoorLock::pOwnershipForm);
    if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == 8 {
        e.get(this, DoorLock::pOwnershipForm)
    } else {
        0
    }
}

// Translated from 005021c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the door's owner lets `actor` through. False without an owner
/// or an actor. The actor's base form is vtable slot 0x1A4, or
/// `BGSSaveFormBuffer::GetForm` (Xbox PDB name of `007af430`) when that
/// is null; when its form type is 0x2A or 0x2B it is cast to
/// `TESActorBase` (`__RTDynamicCast(form, 0, TESBoundObject,
/// TESActorBase, 0)`). The owner being that form answers true. Otherwise
/// the answer is true unless (the owner is not of form type 8 or
/// `use_faction` is false) and the ownership global is missing or zero,
/// and unless the owner is of type 8, the cast succeeded and the actor's
/// rank in it (`TESActorBaseData::GetFactionRank` on the cast form +0x30,
/// with whether the actor is the player) is below `iOwnershipRank`.
pub fn fn_005021c0(e: &mut Engine, this: Ptr<DoorLock>, actor: u32, use_faction: bool) -> bool {
    if e.get(this, DoorLock::pOwnershipForm) == 0 || actor == 0 {
        return false;
    }
    let mut answer = true;
    let mut base = e.vcall(actor, 0x1a4, &args![]).u32();
    if base == 0 {
        base = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![actor]).u32();
    }
    let mut actor_base = 0;
    let kind = e.call(FORM_TYPE, &args![base]).i32();
    if (0x2a..=0x2b).contains(&kind) {
        actor_base = e
            .call(
                RT_DYNAMIC_CAST,
                &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_ACTOR_BASE, 0u32],
            )
            .u32();
    }
    if e.get(this, DoorLock::pOwnershipForm) == base {
        return answer;
    }
    let faction = owner_faction(e, this);
    if (faction == 0 || !use_faction) && global_refuses(e, this) {
        answer = false;
    }
    if faction != 0 && actor_base != 0 {
        let is_player = actor == e.global::<u32>(PLAYER);
        let rank = e
            .call(
                GET_FACTION_RANK,
                &args![actor_base + 0x30, faction, is_player as u32],
            )
            .i32();
        if rank < e.get(this, DoorLock::iOwnershipRank) {
            answer = false;
        }
    }
    answer
}

// Translated from 00502310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor described by `data` may go through regardless of the
/// lock. False without data. True when the door has no lock, or (with
/// `allow_keyless`) its lock has no key. When the door's cell is owned by
/// the actor's base (`00546ee0`), true unless the teleport cell is owned
/// (`TESObjectCELL::GetOwner`) and `00546ee0` refuses the base for it.
/// Otherwise true when `00502550` accepts (with true), or the base is of
/// form type 0x2A, its class (`00502430`) is a guard
/// (`TESClass::IsGuard`, Xbox PDB) and `bIsAlarmed` is set.
pub fn fn_00502310(
    e: &mut Engine,
    this: Ptr<DoorLock>,
    data: Ptr<ActorLockData>,
    allow_keyless: bool,
) -> bool {
    if data.is_null() {
        return false;
    }
    let lock = e.get(this, DoorLock::pLockData);
    if lock.is_null() {
        return true;
    }
    if e.get(lock, RefrLock::pKey).is_null() && allow_keyless {
        return true;
    }
    let door_cell = e.get(this, DoorLock::pDoorCell);
    if door_cell != 0 {
        let base = e.get(data, ActorLockData::pActorBase);
        if e.call(CELL_OWNED_BY_BASE, &args![door_cell, base]).bool() {
            let teleport_cell = e.get(this, DoorLock::pTeleportDoorCell);
            return teleport_cell == 0
                || e.call(CELL_GET_OWNER, &args![teleport_cell]).u32() == 0
                || {
                    let base = e.get(data, ActorLockData::pActorBase);
                    e.call(CELL_OWNED_BY_BASE, &args![teleport_cell, base])
                        .bool()
                };
        }
    }
    if fn_00502550(e, this, data, true) {
        return true;
    }
    let base = e.get(data, ActorLockData::pActorBase);
    if base == 0 || e.call(FORM_TYPE, &args![base]).u32() != 0x2a {
        return false;
    }
    let base = e.get(data, ActorLockData::pActorBase);
    let class = if base != 0 {
        e.call(0x0050_2430, &args![base]).u32()
    } else {
        0
    };
    class != 0
        && e.call(0x005f_6e60, &args![class]).bool()
        && e.get(data, ActorLockData::bIsAlarmed) != 0
}

// Translated from 00502450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorLock::CanActorUnlockDoorLock` (Xbox PDB): clears `*used_item`;
/// false without data. True when `00502310` lets the actor through
/// (keyless not allowed) or the lock lacks `cFlags` bit 0. True when the
/// lock has a key, the actor an inventory and the inventory holds the key
/// (`InventoryChanges::ContainerHasObjects` with the key, 0, 1, the address
/// of a stack word starting 0, 0, 0). Otherwise, when the lock level
/// (`REFR_LOCK::GetLockLevel` with `level_argument`) is not 5 and the
/// inventory holds the form at `011ca268` (same call), true with
/// `*used_item` set to 1.
pub fn door_lock_can_actor_unlock_door_lock(
    e: &mut Engine,
    this: Ptr<DoorLock>,
    data: Ptr<ActorLockData>,
    level_argument: u32,
    used_item: Ptr,
) -> bool {
    e.mem.set_u8(used_item.addr(), 0);
    if data.is_null() {
        return false;
    }
    if fn_00502310(e, this, data, false) {
        return true;
    }
    let lock = e.get(this, DoorLock::pLockData);
    if !e.call(LOCK_FLAG_0, &args![lock]).bool() {
        return true;
    }
    e.with_stack(4, |e, count| {
        let lock = e.get(this, DoorLock::pLockData);
        let key = e.get(lock, RefrLock::pKey);
        if !key.is_null() && e.get(data, ActorLockData::pInventory) != 0 {
            let inventory = e.get(data, ActorLockData::pInventory);
            if e.call(
                CONTAINER_HAS_OBJECTS,
                &args![inventory, key, 0u32, 1u32, count, 0u32, 0u32],
            )
            .bool()
            {
                return true;
            }
        }
        let lock = e.get(this, DoorLock::pLockData);
        if e.call(GET_LOCK_LEVEL, &args![lock, level_argument]).i32() == 5
            || e.get(data, ActorLockData::pInventory) == 0
        {
            return false;
        }
        let item = e.global::<u32>(UNLOCK_ITEM);
        let inventory = e.get(data, ActorLockData::pInventory);
        if e.call(
            CONTAINER_HAS_OBJECTS,
            &args![inventory, item, 0u32, 1u32, count, 0u32, 0u32],
        )
        .bool()
        {
            e.mem.set_u8(used_item.addr(), 1);
            return true;
        }
        false
    })
}

// Translated from 00502550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The owner check of `00502310`, on an `ActorLockData`: false without an
/// owner or data; true when the owner is the actor's base. Otherwise true
/// unless (the owner is not of form type 8 or `use_faction` is false) and
/// the ownership global is missing or zero, and unless the owner is of
/// type 8, the data has a base and the base's rank in it
/// (`TESActorBaseData::GetFactionRank` on base +0x30, with 0) is below
/// `iOwnershipRank`.
pub fn fn_00502550(
    e: &mut Engine,
    this: Ptr<DoorLock>,
    data: Ptr<ActorLockData>,
    use_faction: bool,
) -> bool {
    if e.get(this, DoorLock::pOwnershipForm) == 0 || data.is_null() {
        return false;
    }
    let mut answer = true;
    if e.get(this, DoorLock::pOwnershipForm) == e.get(data, ActorLockData::pActorBase) {
        return answer;
    }
    let faction = owner_faction(e, this);
    if (faction == 0 || !use_faction) && global_refuses(e, this) {
        answer = false;
    }
    let base = e.get(data, ActorLockData::pActorBase);
    if faction != 0 && base != 0 {
        let rank = e
            .call(GET_FACTION_RANK, &args![base + 0x30, faction, 0u32])
            .i32();
        if rank < e.get(this, DoorLock::iOwnershipRank) {
            answer = false;
        }
    }
    answer
}

// Translated from 00502630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorLockData`'s assignment: copies the base, the inventory and the
/// alarmed byte from `source`; returns `this`.
pub fn fn_00502630(
    e: &mut Engine,
    this: Ptr<ActorLockData>,
    source: Ptr<ActorLockData>,
) -> Ptr<ActorLockData> {
    let base = e.get(source, ActorLockData::pActorBase);
    e.set(this, ActorLockData::pActorBase, base);
    let inventory = e.get(source, ActorLockData::pInventory);
    e.set(this, ActorLockData::pInventory, inventory);
    let alarmed = e.get(source, ActorLockData::bIsAlarmed);
    e.set(this, ActorLockData::bIsAlarmed, alarmed);
    this
}

// Translated from 00502670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorLockData::ActorLockData` (Xbox PDB): from an actor, its base
/// object (`004181e0`), the container changes of its extra data list
/// (`005d43c0`, then `ExtraDataList::GetContainerChanges`, Xbox PDB,
/// `00418520`) and `Actor::IsAlarmed`. Returns `this`.
pub fn actor_lock_data_actor_lock_data(
    e: &mut Engine,
    this: Ptr<ActorLockData>,
    actor: u32,
) -> Ptr<ActorLockData> {
    let base = e.call(0x0041_81e0, &args![actor]).u32();
    e.set(this, ActorLockData::pActorBase, base);
    let extra = e.call(0x005d_43c0, &args![actor]).u32();
    let inventory = e.call(0x0041_8520, &args![extra]).u32();
    e.set(this, ActorLockData::pInventory, inventory);
    let alarmed = e.call(IS_ALARMED, &args![actor]).u8();
    e.set(this, ActorLockData::bIsAlarmed, alarmed);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0050_1d20,
            door_lock_door_lock(Ptr<DoorLock>, Ptr) -> Ptr<DoorLock>
        ),
        entry!(0x0050_1e50, door_lock_destructor(Ptr<DoorLock>)),
        entry!(0x0050_1e80, door_lock_set_from_door_ref(Ptr<DoorLock>, Ptr)),
        entry!(
            0x0050_1fd0,
            door_lock_can_actor_ignore_door_lock(Ptr<DoorLock>, u32, u32, bool) -> bool
        ),
        entry!(0x0050_21c0, fn_005021c0(Ptr<DoorLock>, u32, bool) -> bool),
        entry!(
            0x0050_2310,
            fn_00502310(Ptr<DoorLock>, Ptr<ActorLockData>, bool) -> bool
        ),
        entry!(
            0x0050_2450,
            door_lock_can_actor_unlock_door_lock(
                Ptr<DoorLock>,
                Ptr<ActorLockData>,
                u32,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x0050_2550,
            fn_00502550(Ptr<DoorLock>, Ptr<ActorLockData>, bool) -> bool
        ),
        entry!(
            0x0050_2630,
            fn_00502630(Ptr<ActorLockData>, Ptr<ActorLockData>) -> Ptr<ActorLockData>
        ),
        entry!(
            0x0050_2670,
            actor_lock_data_actor_lock_data(Ptr<ActorLockData>, u32) -> Ptr<ActorLockData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn recorder(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::default();
        let l = log.clone();
        e.register_double(addr, move |_, a| {
            l.borrow_mut().push(a.to_vec());
            ret(answer)
        });
        log
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const VTABLE: u32 = 0x0600_0000;
    const SLOT_1A4: u32 = 0x7000_0000;
    const SLOT_304: u32 = 0x7000_0004;
    const SLOT_448: u32 = 0x7000_0008;

    /// Doubles reading test objects: an actor keeps its parent cell at
    /// +0x40 (as `008d6f30` does), its alarmed byte at +0x50, slot 0x304's
    /// and 0x448's answers at +0x54 and +0x58, its base at +0x5C; a cell
    /// keeps its owner at +0x10, its ownership answer at +0x14 and its
    /// flags at +0x24; a form its type at +4.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(CELL_GET_OWNER, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(
            CELL_OWNED_BY_ACTOR,
            |e, a| ret(e.mem.u8(a[0] + 0x14) as u32),
        );
        e.register(CELL_OWNED_BY_BASE, |e, a| ret(e.mem.u8(a[0] + 0x14) as u32));
        e.register(LOCK_FLAG_0, |e, a| ret((e.mem.u8(a[0] + 8) & 1) as u32));
        e.register(IS_ALARMED, |e, a| ret(e.mem.u8(a[0] + 0x50) as u32));
        e.register(SLOT_304, |e, a| ret(e.mem.u8(a[0] + 0x54) as u32));
        e.register(SLOT_448, |e, a| ret(e.mem.u8(a[0] + 0x58) as u32));
        e.register(SLOT_1A4, |e, a| ret(e.mem.u32(a[0] + 0x5c)));
        e.register(CELL_FLAG_0, |e, a| ret((e.mem.u8(a[0] + 0x24) & 1) as u32));
        e.register(CELL_FLAG_20, |e, a| {
            ret((e.mem.u8(a[0] + 0x24) & 0x20) as u32)
        });
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        let mut slots = vec![0u32; 0x44c / 4];
        slots[0x1a4 / 4] = SLOT_1A4;
        slots[0x304 / 4] = SLOT_304;
        slots[0x448 / 4] = SLOT_448;
        e.put_vtable(VTABLE, &slots);
        e.map(PLAYER, 4);
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0x60);
        e.mem.set_u32(o, VTABLE);
        o
    }

    fn lock(e: &mut Engine, level: u8, key: u32, flags: u8) -> u32 {
        let l = e.mem.alloc(0x14);
        e.mem.set_u8(l, level);
        e.mem.set_u32(l + 4, key);
        e.mem.set_u8(l + 8, flags);
        l
    }

    fn door_doubles(e: &mut Engine, door_lock: u32, teleport: u32) {
        e.register(PARENT_CELL, |_, _| ret(0x0c00));
        e.register_double(TELEPORT_DATA, move |_, _| ret(teleport));
        e.register(GET_TELEPORT_CELL, |_, a| ret(a[0] + 1));
        e.register_double(GET_LOCK, move |_, _| ret(door_lock));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LOCK_CONSTRUCT, |_, a| ret(a[0]));
        e.register(GET_OWNER, |_, _| ret(0x0e00));
        e.register(GET_OWNERSHIP_RANK, |_, _| ret(u32::MAX));
        e.register(GET_OWNERSHIP_GLOBAL, |_, _| ret(0x0f00));
    }

    #[test]
    fn constructor_copies_the_door() {
        let mut e = Engine::new();
        let source = lock(&mut e, 50, 0x0777, 5);
        door_doubles(&mut e, source, 0x0d00);
        let this = e.mem.alloc(0x18);
        let door = 0x0500_0000;
        assert_eq!(e.call(0x0050_1d20, &[this, door]).u32(), this);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words[0], 0x0c00);
        assert_eq!(words[1], 0x0d01);
        assert_eq!(&words[3..], &[0x0e00, u32::MAX, 0x0f00]);
        let copy = words[2];
        assert_ne!(copy, source);
        assert_eq!(
            (e.mem.u8(copy), e.mem.u32(copy + 4), e.mem.u8(copy + 8)),
            (50, 0x0777, 5)
        );

        // No teleport data, no lock.
        door_doubles(&mut e, 0, 0);
        e.call(0x0050_1d20, &[this, door]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (0, 0));
    }

    #[test]
    fn destructor_frees_the_lock_copy() {
        let mut e = Engine::new();
        let freed = recorder(&mut e, FREE, 0);
        let this = e.mem.alloc(0x18);
        e.call(0x0050_1e50, &[this]);
        assert!(freed.borrow().is_empty());
        e.mem.set_u32(this + 8, 0x1234);
        e.call(0x0050_1e50, &[this]);
        assert_eq!(*freed.borrow(), vec![vec![0x1234]]);
    }

    #[test]
    fn set_from_door_ref_keeps_or_drops_the_copy() {
        let mut e = Engine::new();
        let source = lock(&mut e, 75, 0x0888, 1);
        door_doubles(&mut e, source, 0x0d00);
        let freed = recorder(&mut e, FREE, 0);
        let this = e.mem.alloc(0x18);
        let existing = lock(&mut e, 0, 0, 0);
        e.mem.set_u32(this + 8, existing);
        e.call_log = Some(vec![]);
        e.call(0x0050_1e80, &[this, 0x0500_0000]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this + 8), existing);
        assert_eq!(
            (
                e.mem.u8(existing),
                e.mem.u32(existing + 4),
                e.mem.u8(existing + 8)
            ),
            (75, 0x0888, 1)
        );
        assert_eq!(e.mem.u32(this + 4), 0x0d01);

        // A missing copy is made.
        e.mem.set_u32(this + 8, 0);
        e.call(0x0050_1e80, &[this, 0x0500_0000]);
        let made = e.mem.u32(this + 8);
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made + 4), 0x0888);

        // The door lost its lock: the copy is freed.
        door_doubles(&mut e, 0, 0);
        e.call(0x0050_1e80, &[this, 0x0500_0000]);
        assert_eq!(*freed.borrow(), vec![vec![made]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        e.call(0x0050_1e80, &[this, 0x0500_0000]);
        assert_eq!(freed.borrow().len(), 1);
    }

    #[test]
    fn ignore_door_lock_cases() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let actor = object(&mut e);
        let player = object(&mut e);
        e.set_global(PLAYER, player);
        let ignore = |e: &mut Engine, actor: u32, allow: u32| {
            e.call(0x0050_1fd0, &[this, actor, 0xdead, allow]).bool()
        };
        assert!(!ignore(&mut e, 0, 1));
        // No lock.
        assert!(ignore(&mut e, actor, 1));
        assert!(!ignore(&mut e, actor, 0));
        // An unlocked lock.
        let l = lock(&mut e, 50, 0, 0);
        e.mem.set_u32(this + 8, l);
        assert!(ignore(&mut e, actor, 1));
        e.mem.set_u8(l + 8, 1);
        assert!(!ignore(&mut e, actor, 1));
        // The actor's owned cell accepts it: the teleport cell decides.
        let cell = object(&mut e);
        e.mem.set_u32(cell + 0x10, 0x0abc);
        e.mem.set_u8(cell + 0x14, 1);
        e.mem.set_u32(actor + 0x40, cell);
        assert!(ignore(&mut e, actor, 0));
        let teleport = object(&mut e);
        e.mem.set_u32(this + 4, teleport);
        assert!(ignore(&mut e, actor, 0));
        e.mem.set_u32(teleport + 0x10, 0x0abc);
        assert!(!ignore(&mut e, actor, 0));
        e.mem.set_u8(teleport + 0x14, 1);
        assert!(ignore(&mut e, actor, 0));
        // A cell that refuses the actor leaves the decision to the rest.
        e.mem.set_u8(cell + 0x14, 0);
        assert!(!ignore(&mut e, actor, 0));
        e.mem.set_u8(actor + 0x54, 1);
        assert!(!ignore(&mut e, actor, 0));
        e.mem.set_u8(actor + 0x50, 1);
        assert!(ignore(&mut e, actor, 0));
        // The player: slot 0x448, a lock below 100, the teleport cell flags.
        e.mem.set_u8(player + 0x58, 1);
        e.mem.set_u8(teleport + 0x24, 0);
        assert!(ignore(&mut e, player, 0));
        e.mem.set_u8(l, 100);
        assert!(!ignore(&mut e, player, 0));
        e.mem.set_u8(l, 99);
        e.mem.set_u8(teleport + 0x24, 1);
        assert!(!ignore(&mut e, player, 0));
        e.mem.set_u8(teleport + 0x24, 0x21);
        assert!(ignore(&mut e, player, 0));
        e.mem.set_u8(player + 0x58, 0);
        assert!(!ignore(&mut e, player, 0));
    }

    fn owner_setup(e: &mut Engine, owner_type: u8) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x18);
        let owner = object(e);
        e.mem.set_u8(owner + 4, owner_type);
        e.mem.set_u32(this + 0xc, owner);
        e.mem.set_i32(this + 0x10, 3);
        let global = e.mem.alloc(4);
        e.mem.set_u32(this + 0x14, global);
        e.register(GLOBAL_VALUE, |e, a| Ret {
            st0: e.mem.f32(a[0]) as f64,
            ..Ret::default()
        });
        (this, owner, global)
    }

    #[test]
    fn owner_check_for_an_actor() {
        let mut e = engine();
        let (this, owner, global) = owner_setup(&mut e, 0x2a);
        let actor = object(&mut e);
        let base = object(&mut e);
        e.mem.set_u8(base + 4, 0x2a);
        e.mem.set_u32(actor + 0x5c, base);
        let casts = recorder(&mut e, RT_DYNAMIC_CAST, 0x0300_0000);
        let check = |e: &mut Engine, flag: u32| e.call(0x0050_21c0, &[this, actor, flag]).bool();
        assert!(!e.call(0x0050_21c0, &[this, 0, 1]).bool());
        // A global holding 0 refuses; any other value (NaN too) accepts.
        assert!(!check(&mut e, 1));
        assert_eq!(
            casts.borrow()[0],
            vec![base, 0, TYPE_TES_BOUND_OBJECT, TYPE_TES_ACTOR_BASE, 0]
        );
        e.mem.set_f32(global, 1.0);
        assert!(check(&mut e, 1));
        e.mem.set_f32(global, f32::NAN);
        assert!(check(&mut e, 1));
        e.mem.set_u32(this + 0x14, 0);
        assert!(!check(&mut e, 1));
        // The owner is the actor's base.
        e.mem.set_u32(actor + 0x5c, owner);
        assert!(check(&mut e, 0));
        // A base from 007af430 when slot 0x1A4 has none.
        e.mem.set_u32(actor + 0x5c, 0);
        let got = recorder(&mut e, SAVE_FORM_BUFFER_GET_FORM, owner);
        assert!(check(&mut e, 0));
        assert_eq!(*got.borrow(), vec![vec![actor]]);
        // No owner.
        e.mem.set_u32(this + 0xc, 0);
        assert!(!check(&mut e, 1));
    }

    #[test]
    fn owner_check_by_faction_rank() {
        let mut e = engine();
        let (this, faction, _) = owner_setup(&mut e, 8);
        let player = object(&mut e);
        e.set_global(PLAYER, player);
        let base = object(&mut e);
        e.mem.set_u8(base + 4, 0x2b);
        e.mem.set_u32(player + 0x5c, base);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0x0300_0000));
        let ranks = recorder(&mut e, GET_FACTION_RANK, 2);
        // Rank 2 below 3 refuses even with a faction.
        assert!(!e.call(0x0050_21c0, &[this, player, 1]).bool());
        assert_eq!(*ranks.borrow(), vec![vec![0x0300_0030, faction, 1]]);
        e.register(GET_FACTION_RANK, |_, _| ret(3));
        assert!(e.call(0x0050_21c0, &[this, player, 1]).bool());
        // Without the faction flag the zero global refuses.
        assert!(!e.call(0x0050_21c0, &[this, player, 0]).bool());
        // A base of another type is not cast: no rank check.
        e.mem.set_u8(base + 4, 0x2c);
        e.register(GET_FACTION_RANK, |_, _| ret(0));
        assert!(e.call(0x0050_21c0, &[this, player, 1]).bool());
    }

    fn data(e: &mut Engine, base: u32, inventory: u32, alarmed: u8) -> u32 {
        let d = e.mem.alloc(0xc);
        e.mem.set_u32(d, base);
        e.mem.set_u32(d + 4, inventory);
        e.mem.set_u8(d + 8, alarmed);
        d
    }

    #[test]
    fn owner_check_for_lock_data() {
        let mut e = engine();
        let (this, faction, global) = owner_setup(&mut e, 8);
        let base = object(&mut e);
        let d = data(&mut e, base, 0, 0);
        let ranks = recorder(&mut e, GET_FACTION_RANK, 1);
        assert!(!e.call(0x0050_2550, &[this, 0, 1]).bool());
        assert!(!e.call(0x0050_2550, &[this, d, 1]).bool());
        assert_eq!(*ranks.borrow(), vec![vec![base + 0x30, faction, 0]]);
        e.register(GET_FACTION_RANK, |_, _| ret(5));
        assert!(e.call(0x0050_2550, &[this, d, 1]).bool());
        assert!(!e.call(0x0050_2550, &[this, d, 0]).bool());
        e.mem.set_f32(global, -2.0);
        assert!(e.call(0x0050_2550, &[this, d, 0]).bool());
        e.mem.set_u32(d, faction);
        e.mem.set_f32(global, 0.0);
        assert!(e.call(0x0050_2550, &[this, d, 0]).bool());
        e.mem.set_u32(this + 0xc, 0);
        assert!(!e.call(0x0050_2550, &[this, d, 1]).bool());
    }

    #[test]
    fn lock_data_may_pass_cases() {
        let mut e = engine();
        e.register(0x0050_2430, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(0x005f_6e60, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        let this = e.mem.alloc(0x18);
        let base = object(&mut e);
        let d = data(&mut e, base, 0, 0);
        let pass = |e: &mut Engine, d: u32, flag: u32| e.call(0x0050_2310, &[this, d, flag]).bool();
        assert!(!pass(&mut e, 0, 1));
        assert!(pass(&mut e, d, 0));
        let l = lock(&mut e, 0, 0, 1);
        e.mem.set_u32(this + 8, l);
        assert!(pass(&mut e, d, 1));
        assert!(!pass(&mut e, d, 0));
        // The door's cell owned by the base: the teleport cell decides.
        let cell = object(&mut e);
        e.mem.set_u8(cell + 0x14, 1);
        e.mem.set_u32(this, cell);
        assert!(pass(&mut e, d, 0));
        let teleport = object(&mut e);
        e.mem.set_u32(teleport + 0x10, 0x0abc);
        e.mem.set_u32(this + 4, teleport);
        assert!(!pass(&mut e, d, 0));
        e.mem.set_u8(teleport + 0x14, 1);
        assert!(pass(&mut e, d, 0));
        // An alarmed guard.
        e.mem.set_u8(cell + 0x14, 0);
        e.mem.set_u8(base + 4, 0x2a);
        let class = object(&mut e);
        e.mem.set_u32(base + 0x40, class);
        e.mem.set_u8(class + 4, 1);
        assert!(!pass(&mut e, d, 0));
        e.mem.set_u8(d + 8, 1);
        assert!(pass(&mut e, d, 0));
        e.mem.set_u8(class + 4, 0);
        assert!(!pass(&mut e, d, 0));
        // The owner check passes.
        let owner = object(&mut e);
        e.mem.set_u32(this + 0xc, owner);
        e.mem.set_u32(d, owner);
        assert!(pass(&mut e, d, 0));
    }

    #[test]
    fn unlock_with_the_key_or_the_item() {
        let mut e = engine();
        e.map(UNLOCK_ITEM, 4);
        e.set_global(UNLOCK_ITEM, 0x0999u32);
        e.register(GET_LOCK_LEVEL, |e, a| ret(e.mem.u8(a[0]) as u32 + a[1]));
        // The inventory holds what its words at +0 and +4 name.
        e.register(CONTAINER_HAS_OBJECTS, |e, a| {
            assert_eq!(e.mem.u32(a[4]), 0);
            assert_eq!((a[2], a[3], a[5], a[6]), (0, 1, 0, 0));
            ret((e.mem.u32(a[0]) == a[1] || e.mem.u32(a[0] + 4) == a[1]) as u32)
        });
        let this = e.mem.alloc(0x18);
        let base = object(&mut e);
        let inventory = e.mem.alloc(8);
        let d = data(&mut e, base, inventory, 0);
        let used = e.mem.alloc(4);
        let unlock = |e: &mut Engine, d: u32, level: u32| {
            e.mem.set_u8(used, 7);
            let r = e.call(0x0050_2450, &[this, d, level, used]).bool();
            (r, e.mem.u8(used))
        };
        assert_eq!(unlock(&mut e, 0, 0), (false, 0));
        // No lock: 00502310 lets it through.
        assert_eq!(unlock(&mut e, d, 0), (true, 0));
        let l = lock(&mut e, 1, 0x0777, 0);
        e.mem.set_u32(this + 8, l);
        // Not locked.
        assert_eq!(unlock(&mut e, d, 0), (true, 0));
        e.mem.set_u8(l + 8, 1);
        assert_eq!(unlock(&mut e, d, 0), (false, 0));
        e.mem.set_u32(inventory, 0x0777);
        assert_eq!(unlock(&mut e, d, 0), (true, 0));
        e.mem.set_u32(inventory, 0);
        e.mem.set_u32(inventory + 4, 0x0999);
        assert_eq!(unlock(&mut e, d, 0), (true, 1));
        // Lock level 5 (1 + 4) refuses the item.
        assert_eq!(unlock(&mut e, d, 4), (false, 0));
        // No inventory.
        e.mem.set_u32(d + 4, 0);
        assert_eq!(unlock(&mut e, d, 0), (false, 0));
    }

    #[test]
    fn lock_data_assignment_copies_three_fields() {
        let mut e = Engine::new();
        let source = data(&mut e, 0x11, 0x22, 1);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xffff_ff00);
        assert_eq!(e.call(0x0050_2630, &[this, source]).u32(), this);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)),
            (0x11, 0x22, 0xffff_ff01)
        );
    }

    #[test]
    fn lock_data_from_an_actor() {
        let mut e = Engine::new();
        e.register(0x0041_81e0, |_, a| ret(a[0] + 1));
        e.register(0x005d_43c0, |_, a| ret(a[0] + 2));
        e.register(0x0041_8520, |_, a| ret(a[0] + 3));
        e.register(IS_ALARMED, |_, _| ret(0xffff_ff01));
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0050_2670, &[this, 0x100]).u32(), this);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 4), e.mem.u8(this + 8)),
            (0x101, 0x105, 1)
        );
    }
}
