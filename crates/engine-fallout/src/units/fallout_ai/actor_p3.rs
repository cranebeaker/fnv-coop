//! `fallout/ai/actor.cpp` (Xbox PDB source unit), part 3: its functions from `00891d70` up to
//! (not including) `008a50d0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::actor`]; anything public there may be used here.
//!
//! This part is a collection of `Actor` methods around inventory (adding
//! and removing items, picking objects up), attacking and animation groups.
//! The game code is unoptimized, so most of it is a chain of tiny accessor
//! calls (`005d43c0` returns `this + 0x44`, the `TESObjectREFR::m_Extra`
//! list; `007af430` returns the base form at `this + 0x20`; `0044ddc0`
//! returns the form at `this + 8` of an inventory entry; `00401170` returns
//! the form type byte at `this + 4`). They are called by address like every
//! other function outside this file.
//!
//! x87 note: floats are computed in `f64` and rounded to `f32` where the code
//! stores a `float`.

#[allow(unused_imports)]
use super::actor::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// The global `PlayerCharacter *` (`thePlayer`).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// The `ProcessLists` singleton (its address is the `this` of `0096f450`).
const PROCESS_LISTS: u32 = 0x011e_0e80;

/// `TESObjectREFR::m_Extra` accessor: returns `this + 0x44`.
const REFR_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// Form type byte accessor: `*(u8 *)(form + 4)`.
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESObjectREFR` base form accessor: `*(this + 0x20)`.
const REFR_BASE_FORM: u32 = 0x007a_f430;
/// Inventory entry form accessor: `*(this + 8)`.
const ENTRY_FORM: u32 = 0x0044_ddc0;
/// `ExtraDataList::GetContainerChanges`.
const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `InventoryChanges::GetGoldAmount`.
const INVENTORY_CHANGES_GET_GOLD_AMOUNT: u32 = 0x004c_b320;
/// `InventoryChanges::ContainerHasObjects`.
const CONTAINER_HAS_OBJECTS: u32 = 0x004c_ffe0;
/// `InventoryChanges::RemoveGold`.
const INVENTORY_CHANGES_REMOVE_GOLD: u32 = 0x004c_b4b0;
/// `InventoryChanges::GetInventoryItem`.
const GET_INVENTORY_ITEM: u32 = 0x004d_0650;
/// `bhkWorld::Activate` (cdecl).
const BHK_WORLD_ACTIVATE: u32 = 0x00c6_a270;
/// `GarbageCollector::Add` (cdecl).
const GARBAGE_COLLECTOR_ADD: u32 = 0x0086_7f90;
/// `TESObjectREFR` parent cell accessor: `*(this + 0x40)`.
const REFR_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::RemoveReference`.
const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
/// `TESObjectREFR::MarkAsPickedUp`.
const MARK_AS_PICKED_UP: u32 = 0x0057_2230;
/// `Actor::GetAnimAction`.
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `Actor::IsWeaponDrawn`.
const ACTOR_IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
/// `Actor::SetCurrentTarget`.
const ACTOR_SET_CURRENT_TARGET: u32 = 0x0088_1620;
/// `Actor::GetCurrentPackageTarget`.
const ACTOR_GET_CURRENT_PACKAGE_TARGET: u32 = 0x0088_1650;
/// `MiddleHighProcess::GetSavedAcquireObject` (reads the process-side object
/// the actor is acquiring an item with; the code calls virtual functions on
/// it).
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `Actor::QueueEquipObject` (6 words).
const ACTOR_QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
/// `Actor::EquipObject` (6 words).
const ACTOR_EQUIP_OBJECT: u32 = 0x0088_c830;
/// `Actor::StealAlarm`.
const ACTOR_STEAL_ALARM: u32 = 0x008b_fa40;
/// `TESObjectWEAP::GetCurrentAmmo`.
const WEAPON_GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// `TESObjectWEAP::GetAmmoRegenRate` (returns a `float` in ST0).
const WEAPON_GET_AMMO_REGEN_RATE: u32 = 0x0070_9430;
/// `ItemChange::HasModEffectActive_ov2` (engine map name; takes a mod effect
/// number).
const ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
/// `ItemChange::GetWorn`.
const ITEM_CHANGE_GET_WORN: u32 = 0x004b_ddd0;
/// Scalar deleting destructor of the inventory entry objects the code gets
/// from `Actor` virtual slot `0x3bc` and `GetInventoryItem` (flag 1 = delete).
const ENTRY_DELETE: u32 = 0x0044_59e0;
/// Next-node accessor of a list: `*(this + 4)`.
const LIST_NEXT: u32 = 0x0072_6070;
/// Returns its `this` (the address of a list node's item slot).
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// Returns the first extra data list of an inventory entry (`0` if none).
const ENTRY_FIRST_EXTRA_LIST: u32 = 0x0055_9450;
/// Clears the list returned by `ProcessLists` (the temporary list's
/// `RemoveAll`).
const LIST_CLEAR: u32 = 0x0047_0470;
/// Scalar deleting destructor of that temporary list (flag 1 = delete).
const LIST_DELETE: u32 = 0x0047_02f0;
/// `0096f450`: the list of actors the process lists find for a form id and
/// actor (`this` = [`PROCESS_LISTS`]).
const PROCESS_LISTS_FIND_ACTORS: u32 = 0x0096_f450;
/// `TESObjectREFR` form id accessor: `*(this + 0x0C)`.
const REFR_FORM_ID: u32 = 0x0084_e3a0;
/// `0041c8d0` (`ExtraDataList::GetReferencePointer`): the reference stored in
/// an extra data list, or 0.
const EXTRA_GET_REFERENCE_POINTER: u32 = 0x0041_c8d0;
/// `ExtraDataList::RemoveOwnership`.
const EXTRA_REMOVE_OWNERSHIP: u32 = 0x0041_aed0;
/// Sets the ownership extra data of a list (takes the owner form).
const EXTRA_SET_OWNERSHIP: u32 = 0x0041_9700;
/// `0041c7f0`: extra-list call that takes an actor (used on the reference of
/// an item that is being picked up).
const EXTRA_SET_ACTOR: u32 = 0x0041_c7f0;
/// `Actor` virtual slot `0x17c`: the item-adding worker (10 words) the
/// item functions end in.
const ACTOR_SLOT_ADD_ITEM: u32 = 0x17c;
/// `Actor` virtual slot `0x22c` (returns a bool, takes one word).
const ACTOR_SLOT_22C: u32 = 0x22c;
/// `Actor` virtual slot `0x3bc`: returns a freshly allocated inventory entry
/// (one word argument, `6`).
const ACTOR_SLOT_3BC: u32 = 0x3bc;
/// `Actor` virtual slot `0x218`.
const ACTOR_SLOT_218: u32 = 0x218;
/// `Actor` virtual slot `0x1e4` (`Actor::GetAnimation` in the `Character`
/// vtable).
const ACTOR_SLOT_GET_ANIMATION: u32 = 0x1e4;
/// `Actor` virtual slot `0x1e8`.
const ACTOR_SLOT_1E8: u32 = 0x1e8;
/// `Actor` virtual slot `0x3ec` (4 words).
const ACTOR_SLOT_3EC: u32 = 0x3ec;
/// `Actor` virtual slot `0x1f4`: returns a pointer to a position (z at +8).
const ACTOR_SLOT_POSITION: u32 = 0x1f4;
/// Process virtual slot `0x148` (`MiddleHighProcess::GetCurrentWeapon` in
/// the high process vtable).
const PROCESS_SLOT_CURRENT_WEAPON: u32 = 0x148;
/// Process virtual slot `0x14c` (`MiddleHighProcess::GetCurrentAmmo` in the
/// high process vtable).
const PROCESS_SLOT_CURRENT_AMMO: u32 = 0x14c;
/// Process virtual slot `0x1cc` (`MiddleHighProcess::ForceWeaponDrawnSheathed`
/// in the high process vtable; `drawn`, two animation objects, the actor).
const PROCESS_SLOT_FORCE_WEAPON_DRAWN: u32 = 0x1cc;

/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `FLT_MAX` (`float`).
const FLOAT_MAX: u32 = 0x0108_4100;

layout! {
    /// `Actor` (Xbox PDB). Only the fields this part uses, at their PC
    /// offsets (the Xbox PDB's offsets minus `0x10` from `pCurrentProcess`
    /// on, since `TESForm` is `0x18` bytes on PC). The size is the Xbox
    /// PDB's; the PC allocation size has not been checked, so do not
    /// allocate an `Actor` by it.
    pub struct Actor: 0x1c4 {
        /// `pCurrentProcess` (Xbox PDB, +0x78): `BaseProcess *`.
        0x68 pCurrentProcess: Ptr,
        /// `bInCombat` (Xbox PDB).
        /// `fUpdateTargetTimer` (Xbox PDB).
        0x74 fUpdateTargetTimer: f32,
        /// `pRagdollController` (Xbox PDB).
        0xAC pRagdollController: Ptr,
        0x104 bInCombat: bool,
        /// `eLifeState` (Xbox PDB).
        0x108 eLifeState: u32,
        /// `eCriticalStage` (Xbox PDB).
        0x10C eCriticalStage: u32,
        /// `eQueuedattack` (Xbox PDB, +0x120): `ANIM_GROUP_ENUM`.
        0x110 eQueuedattack: u32,
        /// `fHeadTrackTimer` (Xbox PDB).
        0x158 fHeadTrackTimer: f32,
        /// `pInitialPackage` (Xbox PDB, +0x1a8): `TESPackage *`. Also the
        /// package `Actor::InitPackageLocations` remembers.
        0x198 pInitialPackage: u32,
        /// `pMyKiller` (Xbox PDB, +0xd0).
        0xC0 pMyKiller: Ptr,
    }
}

fn player(e: &Engine) -> Ptr<Actor> {
    Ptr::new(e.global::<u32>(PLAYER_CHARACTER))
}

fn extra_data_list_of(e: &mut Engine, refr: Ptr) -> Ptr {
    e.call(REFR_EXTRA_DATA_LIST, &args![refr]).ptr()
}

fn form_type(e: &mut Engine, form: Ptr) -> u32 {
    e.call(FORM_TYPE, &args![form]).u32()
}

fn base_form(e: &mut Engine, refr: Ptr) -> Ptr {
    e.call(REFR_BASE_FORM, &args![refr]).ptr()
}

fn entry_form(e: &mut Engine, entry: Ptr) -> Ptr {
    e.call(ENTRY_FORM, &args![entry]).ptr()
}

/// The first extra data list of an inventory entry, `0` when it has none
/// (`if (list) first = *list->item_slot`).
fn first_extra_data_list(e: &mut Engine, entry: Ptr) -> Ptr {
    let mut first = Ptr::NULL;
    if e.call(ENTRY_FIRST_EXTRA_LIST, &args![entry]).u32() != 0 {
        let list = e.call(ENTRY_FIRST_EXTRA_LIST, &args![entry]).ptr::<()>();
        let slot = e.call(LIST_ITEM_SLOT, &args![list]).u32();
        first = Ptr::new(e.mem.u32(slot));
    }
    first
}

/// Deletes an inventory entry object, if there is one.
fn delete_entry(e: &mut Engine, entry: Ptr) {
    if !entry.is_null() {
        e.call(ENTRY_DELETE, &args![entry, 1u32]);
    }
}

/// Walks the actors found by `0096f450` and calls `visit` on each until a
/// null item, then clears and deletes the temporary list.
fn visit_found_actors(e: &mut Engine, list: Ptr, mut visit: impl FnMut(&mut Engine, Ptr)) {
    let mut node = list;
    while !node.is_null() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let actor = Ptr::new(e.mem.u32(slot));
        visit(e, actor);
        node = e.call(LIST_NEXT, &args![node]).ptr();
    }
    if !list.is_null() {
        e.call(LIST_CLEAR, &args![list]);
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

// Translated from 00891d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetGoldAmount` (Xbox PDB): the gold of the actor's inventory
/// changes, 0 when it has none.
pub fn actor_get_gold_amount(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mut gold = 0;
    let extra = extra_data_list_of(e, this.cast());
    let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).ptr::<()>();
    if !changes.is_null() {
        gold = e
            .call(INVENTORY_CHANGES_GET_GOLD_AMOUNT, &args![changes])
            .u32();
    }
    gold
}

// Translated from 00891db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::HasObjects` (Xbox PDB): asks the inventory changes
/// (`InventoryChanges::ContainerHasObjects`) whether the actor holds the
/// objects the five arguments describe; false when it has no inventory
/// changes. The arguments are passed on in the order the game swaps them
/// (the fourth and fifth change places).
pub fn actor_has_objects(
    e: &mut Engine,
    this: Ptr<Actor>,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    arg_5: u32,
) -> bool {
    let extra = extra_data_list_of(e, this.cast());
    let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).ptr::<()>();
    if changes.is_null() {
        return false;
    }
    e.call(
        CONTAINER_HAS_OBJECTS,
        &args![changes, arg_1, arg_2, arg_3, arg_5, arg_4, this],
    )
    .bool()
}

// Translated from 00892460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RemovePickedUpObjectFromWorld` (Xbox PDB), a `cdecl` function
/// without `this`: activates the Havok world of `refr` (virtual slot
/// `0x1d0`), then either marks the reference as picked up (`delete_it`
/// zero) or detaches it (virtual slot `0x1c0`), queues it for garbage
/// collection and removes it from its parent cell.
pub fn actor_remove_picked_up_object_from_world(e: &mut Engine, refr: Ptr, delete_it: u8) {
    let world = e.vcall(refr.addr(), 0x1d0, &args![]).u32();
    e.call(BHK_WORLD_ACTIVATE, &args![world, 1u32, 1u32, 0u32]);
    if delete_it == 0 {
        e.call(MARK_AS_PICKED_UP, &args![refr]);
    } else {
        let cell = e.call(REFR_PARENT_CELL, &args![refr]).ptr::<()>();
        e.vcall(refr.addr(), 0x1c0, &args![]);
        e.call(GARBAGE_COLLECTOR_ADD, &args![refr]);
        if !cell.is_null() {
            e.call(CELL_REMOVE_REFERENCE, &args![cell, refr]);
        }
    }
}

// Translated from 008924e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::PayGoldToActor` (Xbox PDB): removes gold from the inventory
/// changes with `InventoryChanges::RemoveGold`, passing this actor and the
/// two arguments swapped.
pub fn actor_pay_gold_to_actor(e: &mut Engine, this: Ptr<Actor>, arg_1: u32, arg_2: u32) {
    let extra = extra_data_list_of(e, this.cast());
    let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).ptr::<()>();
    e.call(
        INVENTORY_CHANGES_REMOVE_GOLD,
        &args![changes, this, arg_2, arg_1],
    );
}

// Translated from 00893500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFatigue` (Xbox PDB): the current value (`float`, in ST0) of
/// actor value `0x16` from the actor-value owner at `this + 0xa4`
/// (virtual slot `0x0c`).
pub fn actor_get_fatigue(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    e.vcall(this.addr() + 0xa4, 0x0c, &args![0x16u32]).f32()
}

/// `current / base` of an actor value of the owner at `this + 0xa4`
/// (slot `0x00` gives the base as an integer, slot `0x0c` the current value),
/// `1.0` when the base is 0.
fn actor_value_percentage(e: &mut Engine, this: Ptr<Actor>, actor_value: u32) -> f32 {
    let base = e.vcall(this.addr() + 0xa4, 0x00, &args![actor_value]).i32();
    let result = if base == 0 {
        1.0
    } else {
        let current = e.vcall(this.addr() + 0xa4, 0x0c, &args![actor_value]).f32();
        current as f64 / base as f64
    };
    result as f32
}

// Translated from 00893530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFatiguePercentage` (Xbox PDB): actor value `0x16` current over
/// base, `1.0` when the base is 0.
pub fn actor_get_fatigue_percentage(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    actor_value_percentage(e, this, 0x16)
}

// Translated from 00893590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetHealthPercentage` (Xbox PDB): actor value `0x10` current over
/// base, `1.0` when the base is 0.
pub fn actor_get_health_percentage(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    actor_value_percentage(e, this, 0x10)
}

/// `TESObjectREFR::GetOwner` (Ghidra name of `00567790`): the owner form of
/// a reference, or 0.
const REFR_GET_OWNER: u32 = 0x0056_7790;
/// `TESObjectREFR::IsAnOwner` (actor, flag).
const REFR_IS_AN_OWNER: u32 = 0x0057_85e0;
/// Accessor on the object `008d8520` returns: `*(this + 0x28)`.
const ACQUIRE_OBJECT_FIELD_28: u32 = 0x0045_cd60;
/// Actor combat flag accessor: `*(u8 *)(this + 0x104)` (`bInCombat`, Xbox PDB
/// +0x114).
const ACTOR_IN_COMBAT: u32 = 0x0049_3bb0;
/// `ExtraDataList::GetLevCreaOriginalBase`.
const EXTRA_GET_LEV_CREA_ORIGINAL_BASE: u32 = 0x0042_16f0;
/// Item count of an extra data list (`i16` in AX; 1 when it has none).
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
/// Weapon type accessor (`*(i8 *)(form + 0xf4)`, sign-extended).
const WEAPON_TYPE: u32 = 0x0044_6390;
/// Form flag accessor: `*(form + 8) & 1`.
const FORM_FLAG_BIT_0: u32 = 0x0046_0250;
/// `TESForm::GetFile`.
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESObjectREFR::GetRefPersists`.
const REFR_GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// Reference extra-flag test taking one word (`2`).
const REFR_TEST_EXTRA_FLAG: u32 = 0x0057_2d30;
/// The pickup worker (reference, count, flag, zero) the pickup functions call
/// on the actor.
const ACTOR_PICK_UP_WORKER: u32 = 0x0057_4b30;
/// `008c7aa0` (no arguments): whether the pickup is reported to the task
/// queue instead of being removed from the world right away.
const PICK_UP_GOES_TO_TASK_QUEUE: u32 = 0x008c_7aa0;
/// Returns the global at `0x011df1a8`, the `this` of [`TES_QUEUE_PICK_UP`].
const GET_TES: u32 = 0x0045_37b0;
/// `0087afa0` (task queue interface): takes the object [`GET_TES`] returns as
/// `this`, a reference and a flag (stack arguments, `RET 8`).
const TES_QUEUE_PICK_UP: u32 = 0x0087_afa0;
/// `008aded0`: actor call taking a form and two flags.
const ACTOR_FORM_WORKER: u32 = 0x008a_ded0;
/// `008bc9d0`: actor call taking a form and a flag; true when the form may be
/// equipped.
const ACTOR_CAN_EQUIP: u32 = 0x008b_c9d0;

/// `Actor` slots `0x1e4` and `0x1e8` give the two objects the process needs
/// to force the weapon drawn (`drawn` 1) or sheathed (0); the process is read
/// after those calls.
fn force_weapon_drawn(e: &mut Engine, this: Ptr<Actor>, drawn: u32) {
    let biped_a = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .u32();
    let biped_b = e.vcall(this.addr(), ACTOR_SLOT_1E8, &args![]).u32();
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(
        process.addr(),
        PROCESS_SLOT_FORCE_WEAPON_DRAWN,
        &args![drawn, biped_b, biped_a, this],
    );
}

/// Whether `ammo_form` is the ammunition of the process's current weapon
/// (`TESObjectWEAP::GetCurrentAmmo` of the current weapon's form).
fn current_weapon_takes_ammo(e: &mut Engine, this: Ptr<Actor>, ammo_form: Ptr) -> bool {
    let process = e.get(this, Actor::pCurrentProcess);
    let weapon = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .ptr::<()>();
    let weapon_form = entry_form(e, weapon);
    let ammo = e
        .call(WEAPON_GET_CURRENT_AMMO, &args![weapon_form, this])
        .ptr::<()>();
    ammo_form == ammo
}

/// The tail both pickup functions share once the picked-up form is the
/// current weapon's ammunition: when the weapon has no ammunition regeneration
/// (`TESObjectWEAP::GetAmmoRegenRate` of the weapon with mod effect 6, at
/// most 0.0), the weapon is equipped again through `Actor` slot `0x3ec`.
fn requip_weapon_if_no_ammo_regen(e: &mut Engine, this: Ptr<Actor>) {
    let process = e.get(this, Actor::pCurrentProcess);
    let item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .ptr::<()>();
    let mod_active = e
        .call(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, &args![item, 6u32])
        .u8();
    let item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .ptr::<()>();
    let weapon_form = entry_form(e, item);
    let rate = e
        .call(WEAPON_GET_AMMO_REGEN_RATE, &args![weapon_form, mod_active])
        .f32() as f64;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if rate <= zero {
        let item = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .ptr::<()>();
        let mod_active = e
            .call(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, &args![item, 2u32])
            .u8();
        let drawn = e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).u8();
        let item = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .ptr::<()>();
        let weapon_form = entry_form(e, item);
        e.vcall(
            this.addr(),
            ACTOR_SLOT_3EC,
            &args![weapon_form, drawn, mod_active, 0u32],
        );
    }
}

// Translated from 00891e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the reference `refr` up into the actor's inventory. Everyone who has
/// it as a target is retargeted, ownership is fixed (a steal alarm if the
/// actor is not an owner), a drawn weapon of the same kind is sheathed around
/// the pickup (`Actor` virtual slot `0x218` gates the ownership part), the
/// pickup worker `00574b30` runs, ammunition is re-equipped, and the
/// reference leaves the world (`Actor::RemovePickedUpObjectFromWorld`, or the
/// task queue).
///
/// `count` and `flag` are stack arguments the code reads and, for a form of
/// type `0x29`, overwrites. The compiler's exception-unwinding frame is not
/// translated.
pub fn fn_00891e00(e: &mut Engine, this: Ptr<Actor>, refr: Ptr, count: u32, flag: u8) {
    let mut count = count;
    let mut flag = flag;
    let player = player(e);

    e.vcall(refr.addr(), 0x4c, &args![0u32]);
    let base = if refr.is_null() {
        Ptr::NULL
    } else {
        base_form(e, refr)
    };
    let base_type = if base.is_null() {
        0
    } else {
        form_type(e, base)
    };

    // Everyone who has this reference as their package target stops
    // targeting it.
    let form_id = e.call(REFR_FORM_ID, &args![refr]).u32();
    let list = e
        .call(
            PROCESS_LISTS_FIND_ACTORS,
            &args![PROCESS_LISTS, form_id, this],
        )
        .ptr::<()>();
    e.call(ACTOR_FORM_WORKER, &args![this, base, 1u32, 0u32]);
    visit_found_actors(e, list, |e, target| {
        let on_this_reference = !target.is_null()
            && e.call(ACTOR_GET_CURRENT_PACKAGE_TARGET, &args![target])
                .ptr::<()>()
                == refr;
        if on_this_reference {
            e.call(ACTOR_SET_CURRENT_TARGET, &args![target, this]);
        } else {
            e.call(ACTOR_SET_CURRENT_TARGET, &args![target, 0u32]);
        }
    });

    // Ownership.
    if e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool() {
        let mut remove_ownership = true;
        let owner = e.call(REFR_GET_OWNER, &args![refr]).u32();
        if owner != 0 && !e.call(REFR_IS_AN_OWNER, &args![refr, this, 1u32]).bool() {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
            if e.call(ACQUIRE_OBJECT_FIELD_28, &args![acquire]).u32() == 0 {
                let owner = e.call(REFR_GET_OWNER, &args![refr]).u32();
                e.call(
                    ACTOR_STEAL_ALARM,
                    &args![this, refr, base, count, 0u32, owner],
                );
                if e.call(ACTOR_IN_COMBAT, &args![this]).bool() && this != player {
                    let extra = extra_data_list_of(e, refr);
                    e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
                } else {
                    let extra = extra_data_list_of(e, refr);
                    let mut owner_form = e
                        .call(EXTRA_GET_LEV_CREA_ORIGINAL_BASE, &args![extra])
                        .ptr::<()>();
                    if owner_form.is_null() {
                        owner_form = base_form(e, refr);
                    }
                    let extra = extra_data_list_of(e, refr);
                    e.call(EXTRA_SET_OWNERSHIP, &args![extra, owner_form]);
                }
                remove_ownership = false;
            }
        }
        if remove_ownership {
            let extra = extra_data_list_of(e, refr);
            e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
        }
    }

    // A drawn weapon is sheathed around the pickup.
    let mut redraw = false;
    let mut weapon_form = Ptr::NULL;
    let picked_base = base_form(e, refr);
    let picked_type = form_type(e, picked_base);
    if picked_type == 0x28 {
        weapon_form = base_form(e, refr);
        let weapon_type = e.call(WEAPON_TYPE, &args![weapon_form]).i32();
        if weapon_type == 10 || weapon_type == 11 {
            let process = e.get(this, Actor::pCurrentProcess);
            if e.vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
                .u32()
                != 0
            {
                let current = e
                    .vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
                    .ptr::<()>();
                let current_form = entry_form(e, current);
                if weapon_form == current_form && e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).bool()
                {
                    force_weapon_drawn(e, this, 0);
                    redraw = true;
                }
            }
        }
    } else if picked_type == 0x29 {
        flag = 0;
        let extra = extra_data_list_of(e, refr);
        let stack_count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
        if stack_count < 1 {
            let base = base_form(e, refr);
            count = e.call(FORM_TYPE, &args![base.byte_add(0x80)]).u32();
        }
    }

    // The pickup itself.
    let mut removed_from_world = false;
    if e.call(FORM_FLAG_BIT_0, &args![refr]).bool()
        || e.call(FORM_GET_FILE, &args![refr, 0xffff_ffffu32]).u32() != 0
        || e.call(REFR_GET_REF_PERSISTS, &args![refr]).bool()
    {
        e.call(ACTOR_PICK_UP_WORKER, &args![this, refr, count, flag, 0u32]);
    } else if e.call(REFR_TEST_EXTRA_FLAG, &args![refr, 2u32]).bool()
        || e.vcall(refr.addr(), 0x224, &args![]).bool()
    {
        e.call(ACTOR_PICK_UP_WORKER, &args![this, refr, count, 0u32, 0u32]);
    } else {
        e.call(ACTOR_PICK_UP_WORKER, &args![this, refr, count, 0u32, 0u32]);
        removed_from_world = true;
    }

    if redraw {
        e.vcall(
            this.addr(),
            ACTOR_SLOT_3EC,
            &args![weapon_form, 0u32, 0u32, 0u32],
        );
        force_weapon_drawn(e, this, 1);
    }

    // Ammunition regeneration: skipped when the current ammunition item has a
    // successor.
    let process = e.get(this, Actor::pCurrentProcess);
    let mut skip_ammo = false;
    if e.vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
        .u32()
        != 0
    {
        let ammo_item = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
            .ptr::<()>();
        skip_ammo = e.call(LIST_NEXT, &args![ammo_item]).u32() != 0;
    }
    if !skip_ammo {
        let weapon = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32();
        if weapon != 0 && current_weapon_takes_ammo(e, this, base) {
            requip_weapon_if_no_ammo_regen(e, this);
        }
    }

    // Equip the ammunition entry the actor now holds.
    if !e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
        let mut entry = Ptr::NULL;
        if base_type == 0x28 {
            entry = e.vcall(this.addr(), ACTOR_SLOT_3BC, &args![6u32]).ptr();
        }
        if !entry.is_null() {
            let form = entry_form(e, entry);
            if e.call(ACTOR_CAN_EQUIP, &args![this, form, 1u32]).bool() {
                let form = entry_form(e, entry);
                e.call(
                    ACTOR_QUEUE_EQUIP_OBJECT,
                    &args![this, form, 1u32, 0u32, 1u32, 0u32, 1u32],
                );
            }
            delete_entry(e, entry);
        }
    }

    e.vcall(refr.addr(), 0xc4, &args![1u32]);
    let removed = u32::from(removed_from_world);
    if e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
        let tes = e.call(GET_TES, &args![]).ptr::<()>();
        e.call(TES_QUEUE_PICK_UP, &args![tes, refr, removed]);
    } else {
        actor_remove_picked_up_object_from_world(e, refr, removed as u8);
    }
}

/// `PlayerCharacter::AddNote` (item, flag).
const PLAYER_ADD_NOTE: u32 = 0x0096_6a70;
/// Accessor of the ownership extra data (type `0x21`): the owner form, or 0.
const EXTRA_GET_OWNER_FORM: u32 = 0x0041_8660;
/// Interface test taking no arguments; non-zero while a blocking menu is up.
const INTERFACE_BLOCKING_MENU: u32 = 0x0070_5020;
/// Interface menu mode accessor (no arguments); `0` or `3` allow the
/// automatic equip.
const INTERFACE_MENU_MODE: u32 = 0x0070_5040;
/// `Interface::IsInMenuMode`.
const INTERFACE_IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `MiddleHighProcess::GetForceNextUpdate` (engine map name).
const PROCESS_GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `MobileObject::IsInDialoguewithPlayer`.
const IS_IN_DIALOGUE_WITH_PLAYER: u32 = 0x0093_3840;
/// `ExtraDataList::GetWorn` (flag).
const EXTRA_GET_WORN: u32 = 0x0041_8ab0;
/// The item-adding worker on the actor (item, extra data list, count).
const ACTOR_ADD_ITEM_WORKER: u32 = 0x0057_4fa0;
/// Accessor on an animation object: `*(this + 0xd0)` as a `float`.
const ANIMATION_FLOAT_D0: u32 = 0x0045_3700;
/// Animation lookup of the sequence for a slot (`4`), `0` when none.
const ANIMATION_GET_SEQUENCE: u32 = 0x0049_1040;
/// `BSAnimGroupSequence::GetScaledTime` (takes a `float`, returns one).
const SEQUENCE_GET_SCALED_TIME: u32 = 0x004e_ec60;
/// Accessor on a sequence: `*(this + 0x30)` as a `float`.
const SEQUENCE_FLOAT_30: u32 = 0x0050_8100;
/// Accessor on a sequence: `*(this + 0x48)` as a `float`.
const SEQUENCE_FLOAT_48: u32 = 0x0063_9aa0;
/// `Animation::ZeroGlobalTransform` (engine map name).
const ANIMATION_ZERO_GLOBAL_TRANSFORM: u32 = 0x0048_f7f0;
/// `005f2b60`: animation worker taking the actor, two `float`s and the
/// sequence (`RET 0x10`).
const ANIMATION_APPLY_TIME_RANGE: u32 = 0x005f_2b60;
/// Message text source (`this` = [`MESSAGE_SOURCE`], no arguments).
const MESSAGE_TEXT: u32 = 0x004c_69f0;
/// The `this` of [`MESSAGE_TEXT`].
const MESSAGE_SOURCE: u32 = 0x011d_3264;
/// `007052f0` (cdecl, 6 words): shows a message: text, 0, icon path, 0,
/// duration (`float`), 0.
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds"`.
const MESSAGE_ICON_PATH: u32 = 0x0102_08a0;
/// The message duration, `2.0f`.
const MESSAGE_DURATION: u32 = 0x0101_62c0;
/// `007043c0`: no arguments; true when the item may be added into a cell
/// directly.
const ADD_INTO_CELL_ENABLED: u32 = 0x0070_43c0;
/// `TESObjectCELL` call without argument that returns the cell to add into.
const CELL_GET_ADD_TARGET: u32 = 0x0055_1110;
/// The same with the actor as argument.
const CELL_FIND_ADD_TARGET: u32 = 0x0055_1180;
/// `TESObjectCELL::GetOwner`.
const CELL_GET_OWNER: u32 = 0x0054_6a40;
/// `TESValueForm::GetFormValue` (cdecl, one argument).
const VALUE_FORM_GET_FORM_VALUE: u32 = 0x0048_e8a0;
/// The setting-like object whose `float` the owner-value check compares.
const OWNER_VALUE_SETTING: u32 = 0x011d_0f78;
/// Returns a pointer to the float of the setting object (`this` =
/// [`OWNER_VALUE_SETTING`]).
const SETTING_GET_VALUE_POINTER: u32 = 0x0040_3e20;
/// `008a6840`: actor call taking a flag (here `0`).
const ACTOR_FLAG_WORKER: u32 = 0x008a_6840;
/// `TESObjectWEAP::GetFormClipRounds` (takes a flag).
const WEAPON_GET_FORM_CLIP_ROUNDS: u32 = 0x004f_e160;
/// `00944460` (cdecl, two arguments: 0 and the clip rounds): ammunition
/// count to give.
const AMMO_COUNT_FOR_CLIP: u32 = 0x0094_4460;
/// Sets the ammunition of an extra data list (ammo form, count).
const EXTRA_SET_AMMO: u32 = 0x0042_eb60;
/// `bhkNiCollisionObject::ResetSim` (cdecl, world and flag).
const RESET_SIM: u32 = 0x00c6_bd00;
/// `Actor` virtual slot `0x3c4` (no arguments).
const ACTOR_SLOT_3C4: u32 = 0x3c4;
/// `Actor` virtual slot `0x428` (returns an object that takes `1`).
const ACTOR_SLOT_428: u32 = 0x428;
/// `009819d0`: call on the object `Actor` slot `0x428` returns (combat
/// controller, by its unit).
const COMBAT_CONTROLLER_CALL: u32 = 0x0098_19d0;

// Translated from 00892520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `item` (with the extra data list `extra`) to the actor (this is the
/// function in `Actor` virtual slot `0x190` of the `Character` vtable). A note (form
/// type `0x31`) given to the player goes to `PlayerCharacter::AddNote` and
/// the extra data list is destroyed (virtual slot 0, flag 1). Otherwise:
/// ownership that is the player's is removed, everybody who had the
/// reference in the list as package target is retargeted, the item worker
/// `00574fa0` runs, a weapon is equipped for a non-player actor, ammunition is
/// re-equipped, and the combat controller (`Actor` slot `0x428`) is told.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_00892520(e: &mut Engine, this: Ptr<Actor>, item: Ptr, extra: Ptr, arg_3: u32) {
    let player = player(e);
    if form_type(e, item) == 0x31 && this == player {
        e.call(PLAYER_ADD_NOTE, &args![player, item, 1u32]);
        if !extra.is_null() {
            e.vcall(extra.addr(), 0, &args![1u32]);
        }
        return;
    }

    if !extra.is_null() && this == player {
        let owner = e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32();
        let player_base = base_form(e, player.cast());
        if owner == player_base.addr() {
            e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
        }
    }

    if !extra.is_null() {
        let refr = e
            .call(EXTRA_GET_REFERENCE_POINTER, &args![extra])
            .ptr::<()>();
        if !refr.is_null() {
            let form_id = e.call(REFR_FORM_ID, &args![refr]).u32();
            let reference_extra = extra_data_list_of(e, refr);
            e.call(EXTRA_SET_ACTOR, &args![reference_extra, this]);
            e.vcall(refr.addr(), 0x48, &args![0x400u32]);
            let list = e
                .call(
                    PROCESS_LISTS_FIND_ACTORS,
                    &args![PROCESS_LISTS, form_id, this],
                )
                .ptr::<()>();
            visit_found_actors(e, list, |e, target| {
                let has_package_target = !target.is_null()
                    && e.call(ACTOR_GET_CURRENT_PACKAGE_TARGET, &args![target])
                        .u32()
                        != 0;
                if has_package_target {
                    e.call(ACTOR_SET_CURRENT_TARGET, &args![target, this]);
                } else {
                    e.call(ACTOR_SET_CURRENT_TARGET, &args![target, 0u32]);
                }
            });
        }
    }

    e.call(ACTOR_ADD_ITEM_WORKER, &args![this, item, extra, arg_3]);

    // A non-player actor equips a weapon it was given.
    if e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool()
        && player != this
        && !e.call(INTERFACE_BLOCKING_MENU, &args![]).bool()
        && (e.call(INTERFACE_MENU_MODE, &args![]).u32() == 0
            || e.call(INTERFACE_MENU_MODE, &args![]).u32() == 3)
    {
        let mut entry = Ptr::NULL;
        if form_type(e, item) == 0x28 {
            entry = e.vcall(this.addr(), ACTOR_SLOT_3BC, &args![6u32]).ptr();
        }
        if !entry.is_null() {
            if !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
                && !e.call(ACTOR_IN_COMBAT, &args![this]).bool()
            {
                let form = entry_form(e, entry);
                let equip_args = args![this, form, 1u32, 0u32, 1u32, 0u32, 1u32];
                if e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).bool()
                    && e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool()
                {
                    e.call(ACTOR_EQUIP_OBJECT, &equip_args);
                } else {
                    e.call(ACTOR_QUEUE_EQUIP_OBJECT, &equip_args);
                }
            }
            delete_entry(e, entry);
        }
    }

    // Ammunition regeneration.
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null()
        && e.vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
            .u32()
            == 0
        && e.vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32()
            != 0
        && current_weapon_takes_ammo(e, this, item)
        && (this == player || !e.call(IS_IN_DIALOGUE_WITH_PLAYER, &args![this]).bool())
    {
        requip_weapon_if_no_ammo_regen(e, this);
    }

    let combat_controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).ptr::<()>();
    if !combat_controller.is_null() {
        e.call(COMBAT_CONTROLLER_CALL, &args![combat_controller, 1u32]);
    }
}

// Translated from 008929a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the actor `item` (extra data list `extra`, `count`, two more words
/// passed on) through `Actor` virtual slot `0x17c` and returns the
/// reference it made, or null. The player cannot put on worn clothes while
/// an animation action is running (a message is shown and nothing is
/// added). A worn weapon's animation time range is adjusted. Items for a
/// cell are added into it. Ownership of the item is cleared or set to the
/// player's depending on the cell's owner and the item's value, a non-player
/// weapon gets its ammunition, a non-player actor equips what it was given,
/// and the reference's Havok simulation is reset.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_008929a0(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    extra: Ptr,
    count: u32,
    arg_4: u32,
    arg_5: u32,
) -> Ptr {
    let player = player(e);
    let worn = !extra.is_null() && e.call(EXTRA_GET_WORN, &args![extra, 0u32]).bool();

    if this == player && e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() != -1 && worn {
        let text = e.call(MESSAGE_TEXT, &args![MESSAGE_SOURCE]).u32();
        let duration: f32 = e.global(MESSAGE_DURATION);
        e.call(
            SHOW_MESSAGE,
            &args![text, 0u32, MESSAGE_ICON_PATH, 0u32, duration, 0u32],
        );
        return Ptr::NULL;
    }

    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    if !acquire.is_null() {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
        e.vcall(acquire.addr(), 0x17c, &args![item, extra]);
    }
    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .ptr::<()>();

    // A worn weapon: adjust the time range of its animation.
    if worn
        && form_type(e, item) == 0x28
        && !animation.is_null()
        && e.call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
            .u32()
            != 0
    {
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
            .ptr::<()>();
        let animation_time = e.call(ANIMATION_FLOAT_D0, &args![animation]).f32();
        let scaled_time = e
            .call(SEQUENCE_GET_SCALED_TIME, &args![sequence, animation_time])
            .f32();
        let float_max: f32 = e.global(FLOAT_MAX);
        if scaled_time as f64 != -(float_max as f64) {
            let animation_time = e.call(ANIMATION_FLOAT_D0, &args![animation]).f32() as f64;
            let start = e.call(SEQUENCE_FLOAT_30, &args![sequence]).f32() as f64;
            let length = e.call(SEQUENCE_FLOAT_48, &args![sequence]).f32() as f64;
            let end = ((start + animation_time) + length) as f32;
            let transform = e
                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                .ptr::<()>();
            e.call(
                ANIMATION_APPLY_TIME_RANGE,
                &args![transform, this, scaled_time, end, sequence],
            );
        }
    }

    // Straight into a cell, when the game allows it.
    let cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
    if e.call(ADD_INTO_CELL_ENABLED, &args![]).bool() && !cell.is_null() {
        let mut target = e.call(CELL_GET_ADD_TARGET, &args![cell]).ptr::<()>();
        if target.is_null() {
            target = e.call(CELL_FIND_ADD_TARGET, &args![cell, this]).ptr();
        }
        if !target.is_null() {
            e.vcall(
                this.addr(),
                ACTOR_SLOT_ADD_ITEM,
                &args![item, extra, count, 0u32, 0u32, target, 0u32, 0u32, 1u32, 0u32],
            );
            return Ptr::NULL;
        }
    }

    // Ownership.
    let mut fix_ownership = true;
    if !extra.is_null()
        && e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32() != 0
        && (e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32() == 0
            || !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool())
    {
        fix_ownership = false;
    }
    if fix_ownership {
        if !extra.is_null() && e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
            e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
        }
        let cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
        if !cell.is_null() {
            let cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
            if e.call(CELL_GET_OWNER, &args![cell]).u32() != 0 {
                let value = e.call(VALUE_FORM_GET_FORM_VALUE, &args![item]).i32() as f64;
                let threshold_pointer = e
                    .call(SETTING_GET_VALUE_POINTER, &args![OWNER_VALUE_SETTING])
                    .u32();
                let threshold = e.mem.f32(threshold_pointer) as f64;
                if threshold >= value || e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
                    let player_base = base_form(e, player.cast());
                    e.call(EXTRA_SET_OWNERSHIP, &args![extra, player_base]);
                }
            }
        }
    }

    let result = e
        .vcall(
            this.addr(),
            ACTOR_SLOT_ADD_ITEM,
            &args![item, extra, count, 0u32, 1u32, 0u32, arg_4, arg_5, 1u32, 0u32],
        )
        .ptr::<()>();

    // A weapon given to an NPC gets its ammunition.
    if !result.is_null() && form_type(e, item) == 0x28 && this != player {
        e.call(ACTOR_FLAG_WORKER, &args![this, 0u32]);
        let ammo = e
            .call(WEAPON_GET_CURRENT_AMMO, &args![item, this])
            .ptr::<()>();
        if !ammo.is_null() {
            let clip_rounds = e
                .call(WEAPON_GET_FORM_CLIP_ROUNDS, &args![item, 0u32])
                .u32();
            let rounds = e.call(AMMO_COUNT_FOR_CLIP, &args![0u32, clip_rounds]).i32();
            if rounds > 0 {
                let ammo = e
                    .call(WEAPON_GET_CURRENT_AMMO, &args![item, this])
                    .ptr::<()>();
                let result_extra = extra_data_list_of(e, result);
                e.call(EXTRA_SET_AMMO, &args![result_extra, ammo, rounds]);
                e.vcall(result.addr(), 0x48, &args![0x800u32]);
            }
        }
    }

    // A non-player actor equips what it was given.
    let mut entry = Ptr::NULL;
    if this != player
        && e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool()
        && !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
    {
        let item_type = form_type(e, item);
        if item_type == 0x18 {
            e.vcall(this.addr(), ACTOR_SLOT_3C4, &args![]);
        } else if item_type == 0x28 && !e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
            entry = e.vcall(this.addr(), ACTOR_SLOT_3BC, &args![6u32]).ptr();
        }
    }
    if !entry.is_null() {
        let first = first_extra_data_list(e, entry);
        let form = entry_form(e, entry);
        e.call(
            ACTOR_QUEUE_EQUIP_OBJECT,
            &args![this, form, 1u32, first, 1u32, 0u32, 1u32],
        );
        delete_entry(e, entry);
    }

    if !result.is_null() {
        let world = e.vcall(result.addr(), 0x1d0, &args![]).u32();
        e.call(RESET_SIM, &args![world, 1u32]);
        if !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
            let result_base = base_form(e, result);
            e.call(ACTOR_FORM_WORKER, &args![this, result_base, 0u32, 0u32]);
        }
    }
    result
}

// Translated from 00892e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `count` copies of `item` (filter `filter`) out of the actor's
/// inventory changes one at a time and drops them with `Actor` slot `0x17c`
/// at the reference `target` (which also gets the actor as its extra-list
/// actor), then, for a non-player actor, re-equips ammunition and plays the
/// use-item idle on `target`: the idle manager is told the used item, that it
/// activates, and whether `target` is above the actor's eye level. Always
/// returns false. `_unused_2` is a word the code never reads.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_00892e90(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    _unused_2: u32,
    target: Ptr,
    count: u32,
    filter: u32,
) -> bool {
    let player = player(e);
    let extra = extra_data_list_of(e, this.cast());
    let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).ptr::<()>();
    let mut remaining = count;
    if remaining == 0 {
        remaining = 1;
    }
    if !changes.is_null() {
        while remaining != 0 {
            let entry = e
                .call(GET_INVENTORY_ITEM, &args![changes, item, filter])
                .ptr::<()>();
            if !entry.is_null() {
                let first = first_extra_data_list(e, entry);
                if !first.is_null() {
                    let refr = e
                        .call(EXTRA_GET_REFERENCE_POINTER, &args![first])
                        .ptr::<()>();
                    if !refr.is_null() {
                        let refr_extra = extra_data_list_of(e, refr);
                        e.call(EXTRA_SET_ACTOR, &args![refr_extra, target]);
                        e.vcall(refr.addr(), 0x48, &args![0x400u32]);
                    }
                }
                e.vcall(
                    this.addr(),
                    ACTOR_SLOT_ADD_ITEM,
                    &args![item, first, remaining, 0u32, 0u32, target, 0u32, 0u32, 1u32, 0u32],
                );
                remaining -= 1;
            } else {
                remaining = 0;
            }
            delete_entry(e, entry);
        }
        if e.call(CHANGES_IS_EMPTY, &args![changes]).bool() {
            let extra = extra_data_list_of(e, this.cast());
            e.call(EXTRA_REMOVE_CONTAINER_CHANGES, &args![extra]);
        }
    }

    if this == player {
        e.call(INTERFACE_REFRESH_INVENTORY, &args![]);
    } else {
        let entry = e
            .vcall(this.addr(), ACTOR_SLOT_3BC, &args![6u32])
            .ptr::<()>();
        if !entry.is_null() && !e.call(ITEM_CHANGE_GET_WORN, &args![entry, 0u32]).bool() {
            let first = first_extra_data_list(e, entry);
            let form = entry_form(e, entry);
            e.call(
                ACTOR_QUEUE_EQUIP_OBJECT,
                &args![this, form, 1u32, first, 1u32, 0u32, 1u32],
            );
            delete_entry(e, entry);
        }
    }

    if this != player {
        let target_base = base_form(e, target);
        e.call(IDLE_MANAGER_SET_USED_ITEM, &args![target_base]);
        e.call(IDLE_MANAGER_SET_USED_ITEM_ACTIVATE, &args![1u32]);
        let eye_level = e.call(ACTOR_GET_EYE_LEVEL, &args![this]).f32() as f64;
        let actor_position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
        let actor_top = (e.mem.f32(actor_position + 8) as f64 + eye_level) as f32;
        let target_position = e.vcall(target.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
        let target_z = e.mem.f32(target_position + 8);
        let target_is_above =
            e.vcall(target.addr(), ACTOR_SLOT_218, &args![]).bool() || actor_top < target_z;
        e.call(
            IDLE_MANAGER_SET_USED_ITEM_LEVEL,
            &args![u32::from(target_is_above)],
        );
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
        e.vcall(
            acquire.addr(),
            0x44,
            &args![this, 0u32, 2u32, 1u32, 0u32, 1u32],
        );
        e.call(IDLE_MANAGER_SET_USED_ITEM, &args![0u32]);
        e.call(IDLE_MANAGER_SET_USED_ITEM_ACTIVATE, &args![0u32]);
        e.call(IDLE_MANAGER_SET_USED_ITEM_LEVEL, &args![0xffff_ffffu32]);
    }
    false
}

/// `ExtraDataList` test: true when the inventory changes hold nothing.
const CHANGES_IS_EMPTY: u32 = 0x0042_cde0;
/// Removes the container changes from an extra data list.
const EXTRA_REMOVE_CONTAINER_CHANGES: u32 = 0x0041_aeb0;
/// Interface call without arguments the player branch of `fn_00892e90` ends
/// in (`00704af0`).
const INTERFACE_REFRESH_INVENTORY: u32 = 0x0070_4af0;
/// `TESIdleManager::SetUsedItem` (cdecl, one argument).
const IDLE_MANAGER_SET_USED_ITEM: u32 = 0x0060_0900;
/// `TESIdleManager::SetUsedItemActivate` (cdecl, one argument).
const IDLE_MANAGER_SET_USED_ITEM_ACTIVATE: u32 = 0x0060_0940;
/// `TESIdleManager::SetUsedItemLevel` (cdecl, one argument).
const IDLE_MANAGER_SET_USED_ITEM_LEVEL: u32 = 0x0060_0920;
/// `Actor::GetEyeLevel` (returns a `float` in ST0).
const ACTOR_GET_EYE_LEVEL: u32 = 0x008b_e940;

/// `Actor` virtual slot `0x234`.
const ACTOR_SLOT_234: u32 = 0x234;
/// `Actor` virtual slot `0x214`.
const ACTOR_SLOT_214: u32 = 0x214;
/// Process virtual slot `0x214`, as the actor slot above.
const PROCESS_SLOT_214: u32 = 0x214;
/// Reference test: `*(this + 0x1ac) == 9`.
const REFR_FIELD_1AC_IS_9: u32 = 0x0057_9670;
/// Extra-data-objects test (`00437bd0`).
const EXTRA_OBJECTS_TEST: u32 = 0x0043_7bd0;
/// Cell test: `*(u8 *)(this + 0x24) & 1`.
const CELL_FLAG_24_BIT_0: u32 = 0x0042_5fd0;
/// `fabs`-like float function (cdecl, one `float` argument, result in ST0).
const FLOAT_ABS: u32 = 0x0040_8840;
/// `90.0` (`double`).
const NINETY: u32 = 0x0106_6808;
/// `Actor::ShouldSkipFallOutBehavior`.
const ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR: u32 = 0x008a_78f0;
/// `MobileObject::GetCurrentPackage`.
const MOBILE_OBJECT_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `TESPackage::IsInterruptPackage`.
const PACKAGE_IS_INTERRUPT_PACKAGE: u32 = 0x0067_8610;
/// Process virtual slot `0x30c` (a bool).
const PROCESS_SLOT_30C: u32 = 0x30c;
/// Process virtual slot `0x22c` without arguments (the process's current
/// package in `Actor::InitPackageLocations`).
const PROCESS_SLOT_22C: u32 = 0x22c;
/// Process virtual slot `0x1e0` (returns a `float`).
const PROCESS_SLOT_1E0: u32 = 0x1e0;

// Translated from 00893190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ShouldTalkTo` (Xbox PDB): whether the actor may start a dialogue
/// with `other`. False when the process has no acquire object; when the
/// actor is in an interior cell (`00425fd0`) and `other` is more than `90.0`
/// higher or lower; when the process object holds something and the actor
/// should skip its fall-out behaviour; when the actor's state (`Actor` slot
/// `0x214`) is neither 4 nor 0; or when its package is an interrupt
/// package. Otherwise true if the process object's slot `0x1e0` `float` is
/// at most 0.
pub fn actor_should_talk_to(e: &mut Engine, this: Ptr<Actor>, other: Ptr) -> bool {
    let process_object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    if process_object.is_null() {
        return false;
    }

    let blocked = e.vcall(this.addr(), 0x234, &args![]).bool()
        || e.call(REFR_FIELD_1AC_IS_9, &args![this]).bool()
        || e.call(EXTRA_OBJECTS_TEST, &args![this]).bool();
    if !blocked {
        let process_object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
        if e.vcall(process_object.addr(), PROCESS_SLOT_30C, &args![])
            .bool()
        {
            return false;
        }
    }

    let cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
    if !cell.is_null() {
        let cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
        if e.call(CELL_FLAG_24_BIT_0, &args![cell]).bool() {
            let this_position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
            let other_position = e.vcall(other.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
            let height_difference =
                (e.mem.f32(this_position + 8) as f64 - e.mem.f32(other_position + 8) as f64) as f32;
            let distance = e.call(FLOAT_ABS, &args![height_difference]).f32() as f64;
            let limit: f64 = e.global(NINETY);
            if distance > limit {
                return false;
            }
        }
    }

    let process_object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    let flag = e
        .vcall(process_object.addr(), PROCESS_SLOT_22C, &args![])
        .u32();
    if flag != 0
        && e.call(ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR, &args![this, 1u32])
            .bool()
    {
        return false;
    }

    let state = e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32();
    if state != 4 && e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0 {
        return false;
    }

    let package = e
        .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![this])
        .ptr::<()>();
    if !package.is_null() {
        let package = e
            .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![this])
            .ptr::<()>();
        if e.call(PACKAGE_IS_INTERRUPT_PACKAGE, &args![package]).bool() {
            return false;
        }
    }

    let process_object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    let value = e
        .vcall(process_object.addr(), PROCESS_SLOT_1E0, &args![])
        .f32() as f64;
    let zero: f64 = e.global(ZERO_DOUBLE);
    value <= zero
}

/// `Actor` virtual slot `0x2a8` (takes a coordinate pointer).
const ACTOR_SLOT_2A8: u32 = 0x2a8;
/// Form flag test: `*(form + 8) & 0x800`.
const FORM_FLAG_800: u32 = 0x0044_0da0;
/// Process virtual slots used by `Actor::InitPackageLocations`.
const PROCESS_SLOT_28: u32 = 0x28;
const PROCESS_SLOT_14: u32 = 0x14;
const PROCESS_SLOT_24: u32 = 0x24;
/// Accessor on the global at `0x011dea0c`: `*(u8 *)(this + 4)`.
const GLOBAL_11DEA0C: u32 = 0x011d_ea0c;
const GLOBAL_11DEA0C_FLAG: u32 = 0x004f_1540;
/// `ExtraDataList`-style accessor of a package's type: returns a type number
/// (`0xd` is skipped by `Actor::InitPackageLocations`).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `TESPackage` virtual slot `0x13c` (4 words: the actor, 0, a `float`, 0).
const PACKAGE_SLOT_13C: u32 = 0x13c;
/// `TESPackage::GetLocationCell`.
const PACKAGE_GET_LOCATION_CELL: u32 = 0x0067_5c20;
/// `TESPackage::GetLocationWorld`.
const PACKAGE_GET_LOCATION_WORLD: u32 = 0x0067_5a50;
/// `TESPackage::GetLocationCoord` (out coordinate, actor).
const PACKAGE_GET_LOCATION_COORD: u32 = 0x0067_5de0;
/// Reference call taking a `float` (here `FLT_MAX`).
const REFR_FLOAT_CALL: u32 = 0x0057_5770;
/// Returns the package's location (`0` if none).
const PACKAGE_GET_LOCATION: u32 = 0x0055_b980;
/// `PackageLocation::GetLocType`.
const PACKAGE_LOCATION_GET_LOC_TYPE: u32 = 0x0067_8ca0;
/// `TESObjectREFR::MoveRefToNewSpace` (cdecl: reference, cell, world).
const MOVE_REF_TO_NEW_SPACE: u32 = 0x0057_3800;
/// `Actor::UnlockLockDoorsProcedure`.
const ACTOR_UNLOCK_LOCK_DOORS_PROCEDURE: u32 = 0x008a_6a40;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;

// Translated from 00893340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::InitPackageLocations` (Xbox PDB): unless the actor is flagged
/// `0x800`, (when `use_remembered` is 0) tells the process to reset itself
/// (slots `0x28`, `0x14` for a persistent reference, `0x24`), remembers the
/// process's current package (slot `0x22c`) unless the global at
/// `0x011dea0c` says otherwise, and, for a package of type above 2 that
/// accepts the actor (package slot `0x13c`), moves the actor to the package's
/// location; finally calls `Actor::UnlockLockDoorsProcedure`.
pub fn actor_init_package_locations(e: &mut Engine, this: Ptr<Actor>, use_remembered: u8) {
    if e.call(FORM_FLAG_800, &args![this]).bool() {
        return;
    }
    if use_remembered == 0 {
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), PROCESS_SLOT_28, &args![]);
        if e.call(REFR_GET_REF_PERSISTS, &args![this]).bool() {
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(process.addr(), PROCESS_SLOT_14, &args![this, 0u32]);
        }
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), PROCESS_SLOT_24, &args![this, 0u32]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    let mut package = e.vcall(process.addr(), PROCESS_SLOT_22C, &args![]).u32();
    if !e
        .call(GLOBAL_11DEA0C_FLAG, &args![e.global::<u32>(GLOBAL_11DEA0C)])
        .bool()
        && e.get(this, Actor::pInitialPackage) == 0
    {
        e.set(this, Actor::pInitialPackage, package);
    }
    if use_remembered != 0 {
        package = e.get(this, Actor::pInitialPackage);
    }
    let package = Ptr::<()>::new(package);
    if !package.is_null() && e.call(PACKAGE_TYPE, &args![package]).i32() > 2 {
        let minus_one: f32 = e.global(MINUS_ONE);
        if !e
            .vcall(
                package.addr(),
                PACKAGE_SLOT_13C,
                &args![this, 0u32, minus_one, 0u32],
            )
            .bool()
        {
            let cell = e
                .call(PACKAGE_GET_LOCATION_CELL, &args![package, this])
                .u32();
            let world = e
                .call(PACKAGE_GET_LOCATION_WORLD, &args![package, this])
                .u32();
            let float_max: f32 = e.global(LARGE_FLOAT);
            e.call(REFR_FLOAT_CALL, &args![this, float_max]);
            let location = e.call(PACKAGE_GET_LOCATION, &args![package]).ptr::<()>();
            let location_is_current = !location.is_null() && {
                let location = e.call(PACKAGE_GET_LOCATION, &args![package]).ptr::<()>();
                e.call(PACKAGE_LOCATION_GET_LOC_TYPE, &args![location])
                    .u32()
                    == 1
            };
            if !location_is_current && e.call(PACKAGE_TYPE, &args![package]).u32() != 0xd {
                e.with_stack(0x0c, |e, coordinate| {
                    let found = e
                        .call(
                            PACKAGE_GET_LOCATION_COORD,
                            &args![package, coordinate, this],
                        )
                        .u32();
                    e.vcall(this.addr(), ACTOR_SLOT_2A8, &args![found]);
                });
                e.call(MOVE_REF_TO_NEW_SPACE, &args![this, cell, world]);
            }
        }
    }
    e.call(ACTOR_UNLOCK_LOCK_DOORS_PROCEDURE, &args![this]);
}
/// `FLT_MAX` (`float`), as the second copy at `0x01016970`.
const LARGE_FLOAT: u32 = 0x0101_6970;

/// `GameSetting`-style global object whose value byte makes non-player
/// actors refuse to queue attacks (`Actor::QueueAttack`).
const NO_AI_ATTACKS_SETTING: u32 = 0x011d_f6c4;
/// Returns the address of the setting's value (`this + 4`).
const SETTING_VALUE_POINTER: u32 = 0x0040_8d60;
/// Weapon byte accessor: `*(u8 *)(form + 0x102)`.
const WEAPON_BYTE_102: u32 = 0x0052_4b60;
/// `TESAnimGroup::GetMove`.
const ANIM_GROUP_GET_MOVE: u32 = 0x005f_23a0;
/// Actor test (`004997b0`): non-zero while the actor moves.
const ACTOR_IS_MOVING: u32 = 0x0049_97b0;
/// `Animation::BlendOut` (slot, flag).
const ANIMATION_BLEND_OUT: u32 = 0x0049_94f0;
/// Process virtual slots used by `Actor::QueueAttack`.
const PROCESS_SLOT_27C: u32 = 0x27c;
const PROCESS_SLOT_368: u32 = 0x368;
const PROCESS_SLOT_3F8: u32 = 0x3f8;
const PROCESS_SLOT_6DC: u32 = 0x6dc;
/// `CombatFormulas::GetWeaponConditionJamMult` (cdecl, one word).
const GET_WEAPON_CONDITION_JAM_MULT: u32 = 0x0064_77b0;
/// `004dff00` (cdecl, one `float`): a chance roll, true when it succeeds.
const CHANCE_ROLL: u32 = 0x004d_ff00;
/// Weapon call giving the attack animation group base (flag argument).
const WEAPON_GET_ATTACK_ANIM: u32 = 0x0051_e2a0;
/// `TESAnimGroup::GetType` (cdecl, one word).
const ANIM_GROUP_GET_TYPE: u32 = 0x005f_2440;
/// `TESAnimGroup::IsAttackAction` (cdecl, one word).
const ANIM_GROUP_IS_ATTACK_ACTION: u32 = 0x005f_2540;
/// Form test used to choose between name sources (`00474cb0`).
const FORM_HAS_EDITOR_NAME: u32 = 0x0047_4cb0;
/// `MapMarkerData::GetLocationName` (engine map name).
const MAP_MARKER_DATA_GET_LOCATION_NAME: u32 = 0x0040_8da0;
/// Reference name accessor (`0055d520`).
const REFR_GET_NAME: u32 = 0x0055_d520;
/// Debug message printer (cdecl, format and arguments).
const DEBUG_PRINT: u32 = 0x005b_5e40;
/// Table of animation group descriptions, `0x24` bytes each, the first word
/// being the group's name.
const ANIM_GROUP_TABLE: u32 = 0x0119_77d8;
/// `"ANIM_GROUP_NONE"`.
const ANIM_GROUP_NONE_NAME: u32 = 0x0108_4a74;
/// `"hand-to-hand"`.
const HAND_TO_HAND_NAME: u32 = 0x0108_4a64;
/// `"COMBAT: '%s' is queuing an attack with weapon '%s' with attack '%s' that
/// has non-attack group '%s'..."`.
const QUEUE_NON_ATTACK_FORMAT: u32 = 0x0108_4a00;

/// Name of animation group `group` in the table, or `"ANIM_GROUP_NONE"` for
/// `0xff`.
fn anim_group_name(e: &Engine, group: u32) -> u32 {
    if group == 0xff {
        ANIM_GROUP_NONE_NAME
    } else {
        e.mem
            .u32(ANIM_GROUP_TABLE.wrapping_add(group.wrapping_mul(0x24)))
    }
}

// Translated from 008935f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::QueueAttack` (Xbox PDB): asks the actor to attack with animation
/// group `anim_group`. Returns true when the attack is started or queued for
/// later. The player starts it directly (`Actor::StartAttack`). Non-player
/// actors refuse while the setting at `0x011df6c4` is set. Fails when the
/// weapon needs ammunition and there is none, while an animation action of
/// `9` or `0x11` is running, with the weapon not drawn, and when the move
/// state of the animation disagrees with the actor's (the animation is then
/// blended out). A weapon that may jam queues the weapon's jam attack
/// instead (the weapon's attack animation plus `0x17`). A group that is not
/// an attack action is only reported with a debug message.
pub fn actor_queue_attack(e: &mut Engine, this: Ptr<Actor>, anim_group: u32) -> bool {
    let player = player(e);
    if this != player {
        let setting = e
            .call(SETTING_VALUE_POINTER, &args![NO_AI_ATTACKS_SETTING])
            .u32();
        if e.mem.u8(setting) != 0 {
            return false;
        }
    }

    let process = e.get(this, Actor::pCurrentProcess);
    let weapon_item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .ptr::<()>();
    let weapon_form = if weapon_item.is_null() {
        Ptr::NULL
    } else {
        let weapon_item = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .ptr::<()>();
        entry_form(e, weapon_item)
    };
    let ammo_item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
        .ptr::<()>();
    if !weapon_form.is_null() {
        // (the byte is read and not used)
        e.call(WEAPON_BYTE_102, &args![weapon_form]);
    }
    let weapon_uses_ammo = !weapon_form.is_null()
        && !e
            .call(WEAPON_GET_CURRENT_AMMO, &args![weapon_form, this])
            .ptr::<()>()
            .is_null();
    if weapon_uses_ammo {
        if ammo_item.is_null() {
            return false;
        }
        if e.call(LIST_NEXT, &args![ammo_item]).u32() == 0 {
            return false;
        }
    }

    if e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 9
        || e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 0x11
    {
        return false;
    }
    if !e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).bool() {
        return false;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !e.vcall(process.addr(), PROCESS_SLOT_3F8, &args![]).bool() {
        return false;
    }

    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .ptr::<()>();
    if animation.is_null() {
        return false;
    }
    if e.call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
        .u32()
        != 0
    {
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
            .ptr::<()>();
        let transform = e
            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
            .ptr::<()>();
        let group_moves = e.call(ANIM_GROUP_GET_MOVE, &args![transform]).u32() == 1;
        let actor_moves = e.call(ACTOR_IS_MOVING, &args![this]).u8() != 0;
        if group_moves != actor_moves {
            e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
            return false;
        }
    }

    // The object the process is acquiring with decides whether the attack may
    // go ahead (type 8 and 0x10 packages stop it).
    let mut package_allows = true;
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    let package = e
        .vcall(acquire.addr(), PROCESS_SLOT_27C, &args![])
        .ptr::<()>();
    if !package.is_null() {
        package_allows = e.call(PACKAGE_TYPE, &args![package]).i32() != 8
            && e.call(PACKAGE_TYPE, &args![package]).i32() != 0x10;
    }

    if package_allows
        && !weapon_form.is_null()
        && !e
            .call(WEAPON_GET_CURRENT_AMMO, &args![weapon_form, this])
            .ptr::<()>()
            .is_null()
    {
        let process = e.get(this, Actor::pCurrentProcess);
        let condition = e
            .vcall(process.addr(), PROCESS_SLOT_368, &args![this])
            .u32();
        if package_allows && !e.vcall(process.addr(), PROCESS_SLOT_6DC, &args![]).bool() {
            let jam_multiplier = e
                .call(GET_WEAPON_CONDITION_JAM_MULT, &args![condition])
                .f32() as f64;
            let zero: f64 = e.global(ZERO_DOUBLE);
            if jam_multiplier > zero {
                let jam_multiplier = e
                    .call(GET_WEAPON_CONDITION_JAM_MULT, &args![condition])
                    .f32();
                if e.call(CHANCE_ROLL, &args![jam_multiplier]).bool() {
                    let jam_group = e
                        .call(WEAPON_GET_ATTACK_ANIM, &args![weapon_form, 0u32])
                        .u32()
                        .wrapping_add(0x17);
                    let group = actor_get_anim_group(e, this, jam_group, 0, 0, Ptr::NULL);
                    if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == jam_group {
                        e.set(this, Actor::eQueuedattack, jam_group);
                    }
                    return false;
                }
            }
        }
    }

    let group = actor_get_anim_group(e, this, anim_group, 0, 0, Ptr::NULL);
    if this == player {
        actor_start_attack(e, this, anim_group);
        return true;
    }
    if e.call(ANIM_GROUP_IS_ATTACK_ACTION, &args![u32::from(group)])
        .bool()
    {
        e.set(this, Actor::eQueuedattack, anim_group);
        return true;
    }

    // Not an attack action: report it.
    report_group_mismatch(
        e,
        this,
        weapon_form,
        group,
        anim_group,
        QUEUE_NON_ATTACK_FORMAT,
    );
    false
}

/// `Actor::GetAnimation` (called directly by the animation helpers).
const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
/// `Actor::SetAnimAction` (action, sequence).
const ACTOR_SET_ANIM_ACTION: u32 = 0x008a_73e0;
/// `Animation::PlayGroup` (group, 1, -1, -1).
const ANIMATION_PLAY_GROUP: u32 = 0x0049_4740;
/// `Animation::ClearGroup` (slot, blend time `float`).
const ANIMATION_CLEAR_GROUP: u32 = 0x0049_6080;
/// Animation accessor: the animation group id of a slot (`u16`).
const ANIMATION_GROUP_OF_SLOT: u32 = 0x0043_01b0;
/// Animation accessor: the movement speed of a group (`i16`).
const ANIMATION_MOVEMENT_SPEED: u32 = 0x0049_4300;
/// `PlayerCharacter::GetAnimation` (flag).
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `Actor::GetWalkSpeed` (returns a `float` in ST0).
const ACTOR_GET_WALK_SPEED: u32 = 0x0088_4dc0;
/// `Actor` virtual slot `0x4b0` (group id, flag).
const ACTOR_SLOT_4B0: u32 = 0x4b0;
/// `008a0330`: the object the actor's process returns from its virtual slot
/// `0x22c` (the combat controller's `this` in the calls below).
const ACTOR_PROCESS_OBJECT: u32 = 0x008a_0330;
/// `009818e0`: combat controller call taking the blocked flag.
const COMBAT_CONTROLLER_SET_BLOCKED: u32 = 0x0098_18e0;
/// `TESAnimGroup::IsPowerAttackAction` (cdecl, one word).
const ANIM_GROUP_IS_POWER_ATTACK_ACTION: u32 = 0x005f_2670;
/// Animation call (slot): the slot's blend state (`0x0070f490`).
const ANIMATION_SLOT_STATE: u32 = 0x0070_f490;
/// `LowProcess::GetGenericLocation` (engine map name) on an animation
/// sequence.
const SEQUENCE_GET_GENERIC_LOCATION: u32 = 0x0080_41a0;
/// `005f2420` on the global transform: the animation group number.
const ANIM_GROUP_GET_NUMBER: u32 = 0x005f_2420;
/// Weapon form test (`006450c0`) the block code runs on the current weapon.
const WEAPON_FORM_ALLOWS_BLOCK: u32 = 0x0064_50c0;
/// Call (`004954e0`) on the global transform: returns a byte.
const TRANSFORM_BYTE: u32 = 0x0049_54e0;
/// Game setting object whose `float` is the default blend time.
const BLEND_SETTING: u32 = 0x011c_56fc;
/// `30.0` (`double`).
const THIRTY: u32 = 0x0101_db88;
/// Process virtual slot `0x614` (a flag word).
const PROCESS_SLOT_614: u32 = 0x614;

/// Tells the combat controller (`Actor` slot `0x428`), when there is one,
/// whether the actor is blocking: `use_blocked_state` passes
/// `Actor::GetBlocked`, otherwise the `+0xf0` byte.
fn notify_combat_controller(e: &mut Engine, this: Ptr<Actor>, use_blocked_state: bool) {
    if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() == 0 {
        return;
    }
    let flag = if use_blocked_state {
        u32::from(actor_get_blocked(e, this))
    } else {
        u32::from(e.mem.u8(this.addr() + 0xf0))
    };
    let controller = e.call(ACTOR_PROCESS_OBJECT, &args![this]).ptr::<()>();
    e.call(COMBAT_CONTROLLER_SET_BLOCKED, &args![controller, flag]);
}

/// Plays `group` in `animation` (`Animation::PlayGroup` with blend -1/-1),
/// sets the action from the sequence of `slot` and tells the actor
/// (`Actor` slot `0x4b0`).
fn play_group_and_set_action(
    e: &mut Engine,
    this: Ptr<Actor>,
    animation: Ptr,
    group: u32,
    slot: u32,
    action: u32,
) {
    e.call(
        ANIMATION_PLAY_GROUP,
        &args![animation, group, 1u32, 0xffff_ffffu32, 0xffff_ffffu32],
    );
    let sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
        .u32();
    e.call(ACTOR_SET_ANIM_ACTION, &args![this, action, sequence]);
    e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![group, 1u32]);
}

// Translated from 00894900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor's animation action (`Actor::GetAnimAction`) is from 2 to
/// 6.
pub fn fn_00894900(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let action = e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32();
    (2..=6).contains(&action)
}

// Translated from 00894d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetBlocked` (Xbox PDB): whether the animation action is 7.
pub fn actor_get_blocked(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 7
}

// Translated from 00894940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the actor's blocking animation (slot 2 of its animation) to match
/// the byte at `+0xf0` (`bBlockPostAnim`) and the current action. When the
/// block sequence is there but not "generic", only the combat controller is
/// told. Otherwise the block flag is recomputed from the action and the group
/// playing in slot 4; if the block group `0xaa` is not playing and the actor
/// should block, it is played (or only the action is set when it is already
/// the group in slot 2); if the block group is playing and the actor no
/// longer blocks, it is cleared (the player's own animation as well).
pub fn fn_00894940(e: &mut Engine, this: Ptr<Actor>) {
    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .ptr::<()>();
    let block_sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![animation, 2u32])
        .ptr::<()>();
    if !block_sequence.is_null()
        && e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![block_sequence])
            .u32()
            != 1
    {
        notify_combat_controller(e, this, true);
        return;
    }

    let mut should_block = e.mem.u8(this.addr() + 0xf0) != 0;
    let action = e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32();
    if action == -1 {
        should_block = true;
    } else if action == 4 {
        let attack_group = e
            .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
            .u16();
        if e.call(ANIM_GROUP_IS_ATTACK_ACTION, &args![u32::from(attack_group)])
            .bool()
        {
            let attack_group = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
                .u16();
            if e.call(
                ANIM_GROUP_IS_POWER_ATTACK_ACTION,
                &args![u32::from(attack_group)],
            )
            .bool()
                || e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32() < 3
            {
                should_block = false;
            }
        } else {
            let group = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 2u32])
                .u16();
            if e.call(ANIM_GROUP_IS_POWER_ATTACK_ACTION, &args![u32::from(group)])
                .bool()
            {
                should_block = false;
            }
        }
    }

    let mut block_group_playing = false;
    if !block_sequence.is_null() {
        let transform = e
            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![block_sequence])
            .ptr::<()>();
        block_group_playing = e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).u32() == 0xaa;
    }

    if !block_group_playing {
        if e.mem.u8(this.addr() + 0xf0) != 0 && should_block {
            let process = e.get(this, Actor::pCurrentProcess);
            let weapon = e
                .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
                .ptr::<()>();
            let weapon_allows_block = weapon.is_null() || {
                let weapon = e
                    .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
                    .ptr::<()>();
                let weapon_form = entry_form(e, weapon);
                e.call(WEAPON_FORM_ALLOWS_BLOCK, &args![weapon_form]).bool()
            };
            if weapon_allows_block {
                let action = e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32();
                if action != -1 && action != 4 {
                    should_block = false;
                }
            }
        }
        if e.mem.u8(this.addr() + 0xf0) != 0 && should_block {
            let group = actor_get_anim_group(e, this, 0xaa, 0, 0, Ptr::NULL);
            let playing = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 2u32])
                .u16();
            if playing == group {
                let sequence = e
                    .call(ANIMATION_GET_SEQUENCE, &args![animation, 2u32])
                    .u32();
                e.call(ACTOR_SET_ANIM_ACTION, &args![this, 7u32, sequence]);
            } else {
                play_group_and_set_action(e, this, animation, u32::from(group), 2, 7);
            }
            notify_combat_controller(e, this, false);
        }
        return;
    }

    // The block group is playing: stop it unless the actor still blocks.
    if e.mem.u8(this.addr() + 0xf0) != 0 {
        return;
    }
    let setting_pointer = e
        .call(SETTING_GET_VALUE_POINTER, &args![BLEND_SETTING])
        .u32();
    let mut blend_time = e.mem.f32(setting_pointer);
    let mut transform_byte = 0u8;
    if !block_sequence.is_null() {
        let transform = e
            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![block_sequence])
            .ptr::<()>();
        transform_byte = e.call(TRANSFORM_BYTE, &args![transform]).u8();
    }
    if transform_byte != 0 {
        let thirty: f64 = e.global(THIRTY);
        blend_time = (i32::from(transform_byte) as f64 / thirty) as f32;
    }
    e.call(ANIMATION_CLEAR_GROUP, &args![animation, 2u32, blend_time]);
    if this == player(e) {
        let the_player = player(e);
        let player_animation = e
            .call(PLAYER_GET_ANIMATION, &args![the_player, 1u32])
            .ptr::<()>();
        e.call(
            ANIMATION_CLEAR_GROUP,
            &args![player_animation, 2u32, blend_time],
        );
    }
    e.call(ACTOR_SET_ANIM_ACTION, &args![this, 0xffff_ffffu32, 0u32]);
    notify_combat_controller(e, this, false);
}

// Translated from 00894cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetBlock` (Xbox PDB): stores `blocking` in the `+0xf0` byte and
/// applies it (`fn_00894940` for the player, process slot `0x614` with
/// `0x8000` for others). Returns false, after telling the combat controller,
/// when the actor has no animation.
pub fn actor_set_block(e: &mut Engine, this: Ptr<Actor>, blocking: u8) -> bool {
    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .u32();
    if animation == 0 {
        notify_combat_controller(e, this, true);
        return false;
    }
    e.mem.set_u8(this.addr() + 0xf0, blocking);
    if this == player(e) {
        fn_00894940(e, this);
    } else {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
        e.vcall(acquire.addr(), PROCESS_SLOT_614, &args![0x8000u32]);
    }
    true
}

// Translated from 00894d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the idle group `0xab` (in the animation slot that the group table
/// names at +8 of its entry) when the actor has an animation and a process, is
/// not busy (`Actor::GetAnimAction` is not 10, slot `0x214` is 0) and its
/// group has type `0xab`. `_unused_1` is a word the code never reads.
pub fn fn_00894d90(e: &mut Engine, this: Ptr<Actor>, _unused_1: u32) {
    if e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 10 {
        return;
    }
    let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).ptr::<()>();
    let process = e.get(this, Actor::pCurrentProcess);
    if animation.is_null()
        || process.is_null()
        || e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0
    {
        return;
    }
    let mut target_type = 0xabu32;
    let group = actor_get_anim_group(e, this, target_type, 0, 0, Ptr::NULL);
    if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() != target_type
        && e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == 0xab
    {
        target_type = 0xab;
    }
    if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == target_type {
        e.call(
            ANIMATION_PLAY_GROUP,
            &args![
                animation,
                u32::from(group),
                1u32,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
        let slot = e.mem.u32(ANIM_GROUP_TABLE + 8 + target_type * 0x24);
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
            .u32();
        e.call(ACTOR_SET_ANIM_ACTION, &args![this, 7u32, sequence]);
        e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(group), 1u32]);
    }
}

// Translated from 00894e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the group `0xac` in animation slot 4 (action 8), or, when the
/// actor's group is not of type `0xac`, clears slot 4 (the player's animation
/// as well). Same guards as `fn_00894d90`.
pub fn fn_00894e90(e: &mut Engine, this: Ptr<Actor>) {
    if e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 10 {
        return;
    }
    let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).ptr::<()>();
    let process = e.get(this, Actor::pCurrentProcess);
    if animation.is_null()
        || process.is_null()
        || e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0
    {
        return;
    }
    let group = actor_get_anim_group(e, this, 0xac, 0, 0, Ptr::NULL);
    if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == 0xac {
        play_group_and_set_action(e, this, animation, u32::from(group), 4, 8);
    } else {
        e.call(ANIMATION_CLEAR_GROUP, &args![animation, 4u32, 0.0f32]);
        if this == player(e) {
            let the_player = player(e);
            let player_animation = e
                .call(PLAYER_GET_ANIMATION, &args![the_player, 1u32])
                .ptr::<()>();
            e.call(
                ANIMATION_CLEAR_GROUP,
                &args![player_animation, 4u32, 0.0f32],
            );
        }
    }
}

// Translated from 00894f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays a movement-direction group for `directions`: bit 0 gives group
/// `0xb`, bit 1 `0xc`, bit 2 `0xd`, bit 3 `0xe` (the first bit set wins). If
/// the actor's animation picks one of those groups it is played in slot 1
/// (action `0xb`), and the animation's speed multiplier (`fn_008950f0`) is
/// set to the actor's walk speed over the animation's own movement speed for
/// the matching movement group. Returns the group asked for, or `0xff`.
pub fn fn_00894f90(e: &mut Engine, this: Ptr<Actor>, directions: u8) -> u32 {
    let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).ptr::<()>();
    let process = e.get(this, Actor::pCurrentProcess);
    if animation.is_null() || process.is_null() {
        return 0xff;
    }
    let mut wanted = 0xffu32;
    if directions & 1 != 0 {
        wanted = 0xb;
    } else if directions & 2 != 0 {
        wanted = 0xc;
    } else if directions & 4 != 0 {
        wanted = 0xd;
    } else if directions & 8 != 0 {
        wanted = 0xe;
    }
    let group = actor_get_anim_group(e, this, wanted, 0, 0, Ptr::NULL);
    let group_number = u32::from(group);
    if !(0xb..=0xe).contains(&group_number) {
        return 0xff;
    }
    play_group_and_set_action(e, this, animation, group_number, 1, 0xb);
    let movement_group = u32::from((group & 0xff00) | 3);
    let mut speed_multiplier = 1.0f32;
    let walk_speed = e.call(ACTOR_GET_WALK_SPEED, &args![this]).f32();
    if e.call(ANIMATION_MOVEMENT_SPEED, &args![animation, movement_group])
        .u16() as i16
        != 0
    {
        let movement_speed = e
            .call(ANIMATION_MOVEMENT_SPEED, &args![animation, movement_group])
            .u16() as i16;
        speed_multiplier =
            ((walk_speed as f64 / f64::from(movement_speed)) * speed_multiplier as f64) as f32;
    }
    fn_008950f0(e, animation, speed_multiplier);
    wanted
}

// Translated from 008950f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `float` at `+0x10c` of an animation object (its speed
/// multiplier).
pub fn fn_008950f0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x10c, x87_quiet(value));
}

// Translated from 00897890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the dword at `+0x12c` is set.
pub fn fn_00897890(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x12c) & 4 != 0
}

// Translated from 008978b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `0x011c6230`.
pub fn fn_008978b0(e: &mut Engine) -> u32 {
    e.global(0x011c_6230)
}

// Translated from 008978c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `0x011c6234`.
pub fn fn_008978c0(e: &mut Engine) -> u32 {
    e.global(0x011c_6234)
}

// Translated from 008978d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `00448a60(this, 0x100)` is non-zero.
pub fn fn_008978d0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0044_8a60, &args![this, 0x100u32]).u32() != 0
}

// Translated from 008978f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `this + index * 0x30 + 0x1bd`.
pub fn fn_008978f0(e: &mut Engine, this: Ptr, index: u32, value: u8) {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(0x30))
        .wrapping_add(0x1bd);
    e.mem.set_u8(address, value);
}

// Translated from 00897b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x6a0`.
pub fn fn_00897b30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x6a0)
}

/// `TESAnimGroup::AnimGroup` (cdecl: selector a, selector b, group, flag).
const ANIM_GROUP_COMPOSE: u32 = 0x005f_2370;
/// `Animation::PickBestAnimation` (group, 0).
const ANIMATION_PICK_BEST_ANIMATION: u32 = 0x0049_5740;
/// Actor flags accessor (`008846e0`): the `u16` flags of the object at
/// `this + 0x190`, or 0 when there is none.
const ACTOR_ANIM_FLAGS: u32 = 0x0088_46e0;
/// Actor test (`008a6970`).
const ACTOR_TEST_8A6970: u32 = 0x008a_6970;
/// Actor tests that look at process slots `0x6f4` / `0x6ec`.
const ACTOR_TEST_8BA3E0: u32 = 0x008b_a3e0;
const ACTOR_TEST_8BA410: u32 = 0x008b_a410;
/// `Actor` virtual slot `0x390`.
const ACTOR_SLOT_390: u32 = 0x390;
/// Process virtual slot `0x454` (a bool).
const PROCESS_SLOT_454: u32 = 0x454;
/// Accessor at `+0x6a0` (see [`fn_00897b30`]).
const ACTOR_FIELD_6A0: u32 = 0x0089_7b30;
/// `Animation::GetTESAnimGroup` (group).
const ANIMATION_GET_TES_ANIM_GROUP: u32 = 0x0049_6500;
/// `TESAnimGroup::GetTime` (flag; returns a `float` in ST0).
const ANIM_GROUP_GET_TIME: u32 = 0x005f_3780;
/// Table of the animation selector words per weapon type (indexed by the
/// weapon type byte, 4 bytes each).
const WEAPON_TYPE_ANIM_TABLE: u32 = 0x0118_a838;

// Translated from 00897910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetAnimGroup` (Xbox PDB): the animation group (`u16`) to use for
/// the group number `group_id`. `animation` is the animation object (null:
/// the actor's own, `Actor` slot `0x1e4`); `weapon_arg` selects the weapon
/// (`0xffffffff`: none, `0`: the process's current weapon). The group is
/// composed by `TESAnimGroup::AnimGroup` from two selectors (one from the
/// actor's flags `0x800` / `0x2000` / `0x400`, one from the weapon type
/// through [`WEAPON_TYPE_ANIM_TABLE`], or 1) and then refined by
/// `Animation::PickBestAnimation`. Returns `0xff` when there is no animation.
pub fn actor_get_anim_group(
    e: &mut Engine,
    this: Ptr<Actor>,
    group_id: u32,
    weapon_arg: u32,
    flag: u8,
    animation: Ptr,
) -> u16 {
    let mut animation = animation;
    if animation.is_null() {
        animation = e
            .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
            .ptr();
    }
    if animation.is_null() {
        return 0xff;
    }
    let flags = e.call(ACTOR_ANIM_FLAGS, &args![this]).u16();
    let mut selector_a = 0u32;
    let mut selector_b = 0u32;
    let weapon: Ptr = if weapon_arg == 0xffff_ffff {
        Ptr::NULL
    } else if weapon_arg != 0 {
        Ptr::new(weapon_arg)
    } else {
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .ptr()
    };
    if flags & 0x800 != 0 {
        selector_a = 2;
    } else if flags & 0x2000 != 0 {
        selector_a = 3;
    } else if flags & 0x400 != 0 {
        selector_a = 1;
    }

    // Whether the weapon (or the default `1`) decides the second selector.
    let mut uses_weapon_selector = e.vcall(this.addr(), ACTOR_SLOT_390, &args![]).u32() == 0
        && (e.call(ACTOR_TEST_8A6970, &args![this]).bool()
            || e.call(ACTOR_IN_COMBAT, &args![this]).bool());
    if !uses_weapon_selector {
        let process = e.get(this, Actor::pCurrentProcess);
        uses_weapon_selector = e.vcall(process.addr(), PROCESS_SLOT_454, &args![]).bool()
            || (this == player(e) && {
                let the_player = player(e);
                e.call(ACTOR_FIELD_6A0, &args![the_player]).u32() != 0
            });
        if !uses_weapon_selector {
            if !weapon.is_null() {
                uses_weapon_selector = weapon_arg != 0 || {
                    let process = e.get(this, Actor::pCurrentProcess);
                    e.vcall(process.addr(), PROCESS_SLOT_454, &args![]).bool()
                        || (e.call(ACTOR_TEST_8A6970, &args![this]).bool() && group_id != 0xaa)
                };
            }
            uses_weapon_selector |= flag != 0;
        }
    }
    if uses_weapon_selector {
        if !weapon.is_null() && flag == 0 {
            let weapon_form = entry_form(e, weapon);
            let weapon_type = e.call(WEAPON_TYPE, &args![weapon_form]).u32();
            selector_b = e
                .mem
                .u32(WEAPON_TYPE_ANIM_TABLE.wrapping_add(weapon_type.wrapping_mul(4)));
        } else {
            selector_b = 1;
        }
    }

    let in_special_state = e.call(ACTOR_TEST_8BA3E0, &args![this]).bool()
        || e.call(ACTOR_TEST_8BA410, &args![this]).bool();
    let composed = e
        .call(
            ANIM_GROUP_COMPOSE,
            &args![
                selector_a,
                selector_b,
                group_id,
                u32::from(in_special_state)
            ],
        )
        .u16();
    e.call(
        ANIMATION_PICK_BEST_ANIMATION,
        &args![animation, u32::from(composed), 0u32],
    )
    .u16()
}

// Translated from 00897b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetAnimGroupDuration` (Xbox PDB): the duration (`float`) of the
/// animation group `group` of the actor's animation, 0.0 when there is none.
pub fn actor_get_anim_group_duration(e: &mut Engine, this: Ptr<Actor>, group: u32) -> f32 {
    let mut duration = 0.0f32;
    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .ptr::<()>();
    if !animation.is_null() {
        let picked = actor_get_anim_group(e, this, group, 0, 0, animation);
        if picked != 0 {
            let tes_group = e
                .call(
                    ANIMATION_GET_TES_ANIM_GROUP,
                    &args![animation, u32::from(picked)],
                )
                .ptr::<()>();
            if !tes_group.is_null() {
                duration = e.call(ANIM_GROUP_GET_TIME, &args![tes_group, 1u32]).f32();
            }
        }
    }
    duration
}

/// `TESPackage::CreatePackage` (cdecl, package type).
const PACKAGE_CREATE: u32 = 0x0067_0b90;
/// `TESPackage::SetPackType`.
const PACKAGE_SET_PACK_TYPE: u32 = 0x0067_0fc0;
/// `00826b40` / `00826b90`: package flag setters taking a byte.
const PACKAGE_SET_FLAG_A: u32 = 0x0082_6b40;
const PACKAGE_SET_FLAG_B: u32 = 0x0082_6b90;
/// `operator new` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `PackageLocation::PackageLocation`.
const PACKAGE_LOCATION_CONSTRUCT: u32 = 0x0067_f030;
/// `PackageLocation::SetLocReference`.
const PACKAGE_LOCATION_SET_REFERENCE: u32 = 0x0067_f3c0;
/// `PackageLocation::SetLocCell`.
const PACKAGE_LOCATION_SET_CELL: u32 = 0x0067_f410;
/// `0067f1f0`: sets a radius on a package location.
const PACKAGE_LOCATION_SET_RADIUS: u32 = 0x0067_f1f0;
/// `TESPackage::SetPackageLocation`.
const PACKAGE_SET_LOCATION: u32 = 0x0067_1d30;
/// `PackageLocation::~PackageLocation` (flag 1 deletes).
const PACKAGE_LOCATION_DESTROY: u32 = 0x0067_0b30;
/// `PackageTarget::PackageTarget`.
const PACKAGE_TARGET_CONSTRUCT: u32 = 0x0067_ff70;
/// `TESPackage::SetPackageTarget`.
const PACKAGE_SET_TARGET: u32 = 0x0067_2fc0;
/// Package accessor: the package's target (`+0x30`).
const PACKAGE_GET_TARGET: u32 = 0x0067_1d10;
/// `PackageTarget::SetTargType`.
const PACKAGE_TARGET_SET_TYPE: u32 = 0x0068_00b0;
/// `PackageTarget::SetTargReference`.
const PACKAGE_TARGET_SET_REFERENCE: u32 = 0x0068_0110;
/// Package setter: `*(u32 *)(this + 0x18) = value`.
const PACKAGE_SET_FIELD_18: u32 = 0x0098_4f60;
/// `PackageTarget` destructor with delete flag (`007b3fa0`).
const PACKAGE_TARGET_DESTROY: u32 = 0x007b_3fa0;
/// `0067a1b0`: copies the process's current package data into a package.
const PACKAGE_COPY_FROM: u32 = 0x0067_a1b0;
/// `ExtraDataList`-style accessor: `base form of this` (`004181e0` calls
/// `007af430` on its `this`).
const ACTOR_BASE_FORM: u32 = 0x0041_81e0;
/// Reads `*(this + 0x14)`.
const FORM_FIELD_14: u32 = 0x0082_5c00;
/// Sets `*(this + 8) = value` (a package target's count).
const TARGET_SET_COUNT: u32 = 0x0040_3550;
/// Process virtual slot `0x28` (no arguments).
const PROCESS_SLOT_SET_28: u32 = 0x28;
/// `Actor` virtual slot `0x2f4` (package, flag, flag): puts a package on the
/// actor.
const ACTOR_SLOT_ADD_PACKAGE: u32 = 0x2f4;
/// `_ftol2_sse` (`00ec62c0`), the float to integer helper (an `f64`).
const FTOL: u32 = 0x00ec_62c0;

/// Creates a package location for `actor`, hands it to the package and
/// releases it (what the three package starters do).
fn give_package_actor_location(e: &mut Engine, package: Ptr, actor: Ptr<Actor>) {
    let memory = e.call(OPERATOR_NEW, &args![0xcu32]).ptr::<()>();
    let location = if memory.is_null() {
        Ptr::NULL
    } else {
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![memory])
            .ptr::<()>()
    };
    e.call(PACKAGE_LOCATION_SET_REFERENCE, &args![location, actor]);
    e.call(PACKAGE_SET_LOCATION, &args![package, location]);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_DESTROY, &args![location, 1u32]);
    }
}

/// Allocates and constructs a `PackageTarget` (0x10 bytes).
fn new_package_target(e: &mut Engine) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x10u32]).ptr::<()>();
    if memory.is_null() {
        Ptr::NULL
    } else {
        e.call(PACKAGE_TARGET_CONSTRUCT, &args![memory]).ptr()
    }
}

// Translated from 00897bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the actor a package of type `0xe` (field `0x18` = `0x29`) that
/// targets the actor itself with a location at the actor, copies the
/// process's current package data into it (`0067a1b0`), sets the target's
/// count from the actor's base form (field `0x14` of the word at
/// base form `+0x90`), tells the process (slot `0x28`) and puts the package on
/// the actor (`Actor` slot `0x2f4`). `_unused_1` is a word the code never
/// reads.
pub fn fn_00897bd0(e: &mut Engine, this: Ptr<Actor>, _unused_1: u32) {
    let package = e.call(PACKAGE_CREATE, &args![0xeu32]).ptr::<()>();
    e.call(PACKAGE_SET_PACK_TYPE, &args![package, 0xeu32]);
    e.call(PACKAGE_SET_FLAG_A, &args![package, 0u32]);
    e.call(PACKAGE_SET_FLAG_B, &args![package, 0u32]);
    give_package_actor_location(e, package, this);

    let target = new_package_target(e);
    e.call(PACKAGE_SET_TARGET, &args![package, target]);
    e.call(PACKAGE_SET_FIELD_18, &args![package, 0x29u32]);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).ptr::<()>();
    e.call(PACKAGE_TARGET_SET_TYPE, &args![package_target, 0u32]);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).ptr::<()>();
    e.call(PACKAGE_TARGET_SET_REFERENCE, &args![package_target, this]);

    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    let current_package = e
        .vcall(acquire.addr(), PROCESS_SLOT_22C, &args![])
        .ptr::<()>();
    if !current_package.is_null() {
        e.call(PACKAGE_COPY_FROM, &args![package, current_package]);
    }
    let base = e.call(ACTOR_BASE_FORM, &args![this]).ptr::<()>();
    let value = e.call(FORM_FIELD_14, &args![base.byte_add(0x90)]).u32();
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).ptr::<()>();
    e.call(TARGET_SET_COUNT, &args![package_target, value]);

    if !target.is_null() {
        e.call(PACKAGE_TARGET_DESTROY, &args![target, 1u32]);
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    e.vcall(acquire.addr(), PROCESS_SLOT_SET_28, &args![]);
    e.vcall(
        this.addr(),
        ACTOR_SLOT_ADD_PACKAGE,
        &args![package, 1u32, 1u32],
    );
}

// Translated from 008982c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::InitiateAvoidPackage` (Xbox PDB): ends the actor's movement, then
/// gives it a package of type `0x1f` (both flags 1) with a location at the
/// actor and a target of type 0 on `target`, marks `target` as targeted,
/// computes the procedure type, copies the process's current package data,
/// passes `speed` to the process (slot `0x25c`) and puts the package on the
/// actor.
pub fn actor_initiate_avoid_package(e: &mut Engine, this: Ptr<Actor>, target: Ptr, speed: f32) {
    e.call(ACTOR_END_MOVEMENT, &args![this]);
    let package = e.call(PACKAGE_CREATE, &args![0x1fu32]).ptr::<()>();
    e.call(PACKAGE_SET_FLAG_A, &args![package, 1u32]);
    e.call(PACKAGE_SET_FLAG_B, &args![package, 1u32]);
    give_package_actor_location(e, package, this);

    let package_target = new_package_target(e);
    e.call(PACKAGE_SET_TARGET, &args![package, package_target]);
    let stored_target = e.call(PACKAGE_GET_TARGET, &args![package]).ptr::<()>();
    e.call(PACKAGE_TARGET_SET_TYPE, &args![stored_target, 0u32]);
    let stored_target = e.call(PACKAGE_GET_TARGET, &args![package]).ptr::<()>();
    e.call(PACKAGE_TARGET_SET_REFERENCE, &args![stored_target, target]);
    e.call(REFR_SET_TARGETED, &args![target, 1u32]);
    if !package_target.is_null() {
        e.call(PACKAGE_TARGET_DESTROY, &args![package_target, 1u32]);
    }
    e.call(PACKAGE_CALCULATE_PROCEDURE_TYPE, &args![package, 0u32]);

    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    let current_package = e
        .vcall(acquire.addr(), PROCESS_SLOT_22C, &args![])
        .ptr::<()>();
    if !current_package.is_null() {
        e.call(PACKAGE_COPY_FROM, &args![package, current_package]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_SLOT_25C, &args![speed]);
    e.vcall(
        this.addr(),
        ACTOR_SLOT_ADD_PACKAGE,
        &args![package, 1u32, 1u32],
    );
}

// Translated from 008984c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::InitiateSurfacePackage` (Xbox PDB): stops the actor
/// (`Actor::ForceStopMoving`), then gives it a package of type `0x1d` (both
/// flags 1) with a location at the actor and the procedure type computed.
pub fn actor_initiate_surface_package(e: &mut Engine, this: Ptr<Actor>) {
    e.call(ACTOR_FORCE_STOP_MOVING, &args![this]);
    let package = e.call(PACKAGE_CREATE, &args![0x1du32]).ptr::<()>();
    e.call(PACKAGE_SET_PACK_TYPE, &args![package, 0x1du32]);
    e.call(PACKAGE_SET_FLAG_A, &args![package, 1u32]);
    e.call(PACKAGE_SET_FLAG_B, &args![package, 1u32]);
    give_package_actor_location(e, package, this);
    e.call(PACKAGE_CALCULATE_PROCEDURE_TYPE, &args![package, 0u32]);
    e.vcall(
        this.addr(),
        ACTOR_SLOT_ADD_PACKAGE,
        &args![package, 1u32, 1u32],
    );
}

/// `Actor::EndMovement`.
const ACTOR_END_MOVEMENT: u32 = 0x0087_faa0;
/// `Actor::ForceStopMoving`.
const ACTOR_FORCE_STOP_MOVING: u32 = 0x008b_3ad0;
/// `TESObjectREFR::SetTargeted`.
const REFR_SET_TARGETED: u32 = 0x0056_4db0;
/// `TESPackage::CalculateProcedureType`.
const PACKAGE_CALCULATE_PROCEDURE_TYPE: u32 = 0x0067_77b0;
/// Process virtual slot `0x25c` (a `float`).
const PROCESS_SLOT_25C: u32 = 0x25c;

/// `FleePackage::FleePackage` (this, avoided reference, 0, 0).
const FLEE_PACKAGE_CONSTRUCT: u32 = 0x009f_0e00;
/// `FleePackage::AddAvoidedRef`.
const FLEE_PACKAGE_ADD_AVOIDED_REF: u32 = 0x009f_1310;
/// `FleePackage::FindTeleportDoor` (actor, avoided reference).
const FLEE_PACKAGE_FIND_TELEPORT_DOOR: u32 = 0x009f_1140;
/// Extra-data test (`00437bf0`): the object's extra type is 5.
const EXTRA_TYPE_IS_5: u32 = 0x0043_7bf0;
/// `MobileObject::GetCurrentPackage`.
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// Package setter (`005d0b80`): `*(u8 *)(this + 0x80) = value`.
const PACKAGE_SET_BYTE_80: u32 = 0x005d_0b80;
/// `00994ef0`: package call taking a reference.
const PACKAGE_SET_REFERENCE_994EF0: u32 = 0x0099_4ef0;
/// `005e3fc0`: package accessor `*(this + 0xa4)`.
const PACKAGE_FIELD_A4: u32 = 0x005e_3fc0;
/// `00810530`: package setter `*(u8 *)(this + 0x94) = value`.
const PACKAGE_SET_BYTE_94: u32 = 0x0081_0530;
/// `006ca4e0`: package accessor `*(f32 *)(this + 0x90)`.
const PACKAGE_FLOAT_90: u32 = 0x006c_a4e0;
/// `0067a720`: package call taking a flag.
const PACKAGE_CALL_67A720: u32 = 0x0067_a720;
/// Process virtual slots used by `fn_00897de0`: `0xe4` returns a `float`,
/// `0xe8` takes one, `0x710` takes the actor.
const PROCESS_SLOT_E4: u32 = 0xe4;
const PROCESS_SLOT_E8: u32 = 0xe8;
const PROCESS_SLOT_710: u32 = 0x710;
/// `20.0` (`float`).
const TWENTY: u32 = 0x0101_7868;
/// `-1.0` (`double`).
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;

/// What `FISTP` with truncation leaves in the low word of the 64-bit result
/// (the indefinite value, whose low word is 0, when out of range).
fn x87_truncate_low_word(value: f32) -> u32 {
    let truncated = (value as f64).trunc();
    let limit = 2f64.powi(63);
    if !(-limit..limit).contains(&truncated) {
        0
    } else {
        truncated as i64 as u32
    }
}

// Translated from 00897de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the actor flee from `avoided`: unless the actor is excluded (extra
/// type tests `00437bf0` / `00437bd0`, `Actor` slot `0x22c`, byte `+0x118`)
/// it reuses its current package when that is a flee package (type `0x16`) or
/// makes a new `FleePackage`, adds `avoided` to the avoided references and
/// to the package's target (with `arg_7` as the target's count when it is not
/// negative), and gives the package a location: `location_ref`, else `cell`,
/// else the process's own (and then `FleePackage::FindTeleportDoor` runs
/// when the package has no teleport door yet). Finally the package's flags
/// and distance are set from `arg_3`, `arg_4` and the package's own float
/// (negated), and it is put on the actor (`Actor` slot `0x2f4`).
///
/// `arg_2` is cleared when `Actor` slot `0x214` is not 0. The compiler's
/// exception-unwinding frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00897de0(
    e: &mut Engine,
    this: Ptr<Actor>,
    avoided: Ptr,
    arg_2: u8,
    arg_3: u8,
    arg_4: u8,
    cell: Ptr,
    location_ref: Ptr,
    arg_7: f32,
    arg_8: f32,
) {
    if e.call(EXTRA_TYPE_IS_5, &args![this]).bool()
        || e.call(EXTRA_OBJECTS_TEST, &args![this]).bool()
        || e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
        || e.mem.u8(this.addr() + 0x118) != 0
    {
        return;
    }
    let mut arg_2 = arg_2;
    if e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0 {
        arg_2 = 0;
    }
    let current = e.call(GET_CURRENT_PACKAGE, &args![this]).ptr::<()>();
    let mut flee = Ptr::NULL;
    if !current.is_null() && e.call(PACKAGE_TYPE, &args![current]).i32() == 0x16 {
        flee = current;
    }
    if flee.is_null() {
        let memory = e.call(OPERATOR_NEW, &args![0xacu32]).ptr::<()>();
        flee = if memory.is_null() {
            Ptr::NULL
        } else {
            e.call(FLEE_PACKAGE_CONSTRUCT, &args![memory, avoided, 0u32, 0u32])
                .ptr()
        };
    }
    e.call(FLEE_PACKAGE_ADD_AVOIDED_REF, &args![flee, avoided]);
    let mut target = e.call(PACKAGE_GET_TARGET, &args![flee]).ptr::<()>();
    if target.is_null() {
        target = new_package_target(e);
        e.call(PACKAGE_SET_TARGET, &args![flee, target]);
    }
    e.call(PACKAGE_TARGET_SET_REFERENCE, &args![target, avoided]);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if arg_7 as f64 >= zero {
        let count = e.call(FTOL, &args![arg_7 as f64]).u32();
        e.call(TARGET_SET_COUNT, &args![target, count]);
    }

    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    e.vcall(acquire.addr(), PROCESS_SLOT_SET_28, &args![]);
    let memory = e.call(OPERATOR_NEW, &args![0xcu32]).ptr::<()>();
    let location = if memory.is_null() {
        Ptr::NULL
    } else {
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![memory])
            .ptr::<()>()
    };
    if arg_8 as f64 >= zero {
        let radius = x87_truncate_low_word(arg_8);
        e.call(PACKAGE_LOCATION_SET_RADIUS, &args![location, radius]);
    }
    if !location_ref.is_null() {
        e.call(
            PACKAGE_LOCATION_SET_REFERENCE,
            &args![location, location_ref],
        );
        e.call(PACKAGE_SET_BYTE_80, &args![flee, 0u32]);
        e.call(PACKAGE_SET_LOCATION, &args![flee, location]);
        e.call(PACKAGE_SET_REFERENCE_994EF0, &args![flee, location_ref]);
    } else if !cell.is_null() {
        e.call(PACKAGE_LOCATION_SET_CELL, &args![location, cell]);
        e.call(PACKAGE_SET_BYTE_80, &args![flee, 0u32]);
        e.call(PACKAGE_SET_LOCATION, &args![flee, location]);
    } else {
        // (the interior test's result is not used)
        let actor_cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
        if !actor_cell.is_null() {
            let actor_cell = e.call(REFR_PARENT_CELL, &args![this]).ptr::<()>();
            e.call(CELL_FLAG_24_BIT_0, &args![actor_cell]);
        }
        if e.call(PACKAGE_FIELD_A4, &args![flee]).u32() == 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
            let value = e.vcall(acquire.addr(), PROCESS_SLOT_E4, &args![]).f32() as f64;
            if value <= zero {
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
                let twenty: f32 = e.global(TWENTY);
                e.vcall(acquire.addr(), PROCESS_SLOT_E8, &args![twenty]);
            }
            e.call(FLEE_PACKAGE_FIND_TELEPORT_DOOR, &args![flee, this, avoided]);
        }
    }
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_DESTROY, &args![location, 1u32]);
    }

    if arg_2 == 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
        e.vcall(acquire.addr(), PROCESS_SLOT_710, &args![this]);
    }
    fn_008982a0(e, flee, arg_3);
    e.call(PACKAGE_SET_FIELD_18, &args![flee, 0x13u32]);
    e.call(PACKAGE_SET_BYTE_94, &args![flee, 0u32]);
    let own_value = e.call(PACKAGE_FLOAT_90, &args![flee]).f32() as f64;
    let minus_one: f64 = e.global(MINUS_ONE_DOUBLE);
    fn_00898250(e, flee, (own_value * minus_one) as f32);
    fn_00898280(e, flee, arg_4);
    if arg_4 != 0 {
        e.call(PACKAGE_CALL_67A720, &args![flee, 0u32]);
        fn_00898200(e, flee, 0);
    }
    e.vcall(
        this.addr(),
        ACTOR_SLOT_ADD_PACKAGE,
        &args![flee, u32::from(arg_2), 1u32],
    );
}

// Translated from 00898200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit `0x2000000` of the package flags at
/// `+0x1c`.
pub fn fn_00898200(e: &mut Engine, this: Ptr, flag: u8) {
    let flags = e.mem.u32(this.addr() + 0x1c);
    let flags = if flag != 0 {
        flags | 0x0200_0000
    } else {
        flags & 0xfdff_ffff
    };
    e.mem.set_u32(this.addr() + 0x1c, flags);
}

// Translated from 00898250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `value` to the `float` at `+0x90`.
pub fn fn_00898250(e: &mut Engine, this: Ptr, value: f32) {
    let sum = e.mem.f32(this.addr() + 0x90) as f64 + value as f64;
    e.mem.set_f32(this.addr() + 0x90, sum as f32);
}

// Translated from 00898280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x81`.
pub fn fn_00898280(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x81, value);
}

// Translated from 008982a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0xa8`.
pub fn fn_008982a0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xa8, value);
}

/// `BGSAmmoForm::IsRockItLauncher` (this = the weapon's ammo component at
/// `weapon + 0xa4`).
const AMMO_FORM_IS_ROCK_IT_LAUNCHER: u32 = 0x0047_4a80;
/// `PlayerCharacter::GetNumRockItAmmo`.
const PLAYER_GET_NUM_ROCK_IT_AMMO: u32 = 0x0096_9040;
/// `Interface::QueueMenuCreate` (cdecl, 6 words).
const INTERFACE_QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// Ammo component accessor (`00474a40`, `this` = weapon + 0xa4): the list of
/// ammunition forms, or 0.
const AMMO_COMPONENT_LIST: u32 = 0x0047_4a40;
/// `00500940`: list accessor on the ammunition list.
const AMMO_LIST_FIRST_NODE: u32 = 0x0050_0940;
/// `__RTDynamicCast` (cdecl: pointer, vfdelta, source type, target type,
/// is-reference).
const RTTI_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Type descriptor of the cast source (`TESForm`).
const RTTI_TYPE_TES_FORM: u32 = 0x0118_3028;
/// Type descriptor the ammunition cast targets.
const RTTI_TYPE_AMMO_TARGET: u32 = 0x0118_40c4;
/// Type descriptor the idle-form cast targets.
const RTTI_TYPE_IDLE_TARGET: u32 = 0x0118_6a18;
/// `TESObjectREFR::GetInventoryItem` (form, 0).
const REFR_GET_INVENTORY_ITEM: u32 = 0x0057_6260;
/// Object the menu code asks about the weapon's sound (`this` = the global
/// pointed to by `0x011dea0c`).
const OBJECT_POINTER_011DEA0C: u32 = 0x011d_ea0c;
/// `00877720` on that object (no arguments).
const OBJECT_011DEA0C_GET: u32 = 0x0087_7720;
/// `00a24660` (object, 4, 1): non-zero when the player may play the
/// dry-fire feedback.
const DRY_FIRE_ALLOWED: u32 = 0x00a2_4660;
/// Weapon call (`005224c0`): the weapon's sound set object, or 0.
const WEAPON_SOUND_SET: u32 = 0x0052_24c0;
/// `00511840`: file name of a sound set object.
const SOUND_SET_FILE_NAME: u32 = 0x0051_1840;
/// `BSAudio::QInstance` (no arguments).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByFilename` (handle out, name, flags, sound set).
const AUDIO_GET_SOUND_HANDLE_BY_FILENAME: u32 = 0x00ad_7480;
/// `BSSoundHandle` constructor (`0041a250`).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// `BSSoundHandle` destructor (`00483710`).
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `BSSoundHandle::IsValid`.
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
/// `BSSoundHandle::Play` (flag).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// Assigns a sound handle (`00418900`: this = destination, source).
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// Sets a sound handle's position (`0068a7d0`).
const SOUND_HANDLE_SET_POSITION: u32 = 0x0068_a7d0;
/// `ExtraDataList::GetSound` (out handle).
const EXTRA_GET_SOUND: u32 = 0x0041_8890;
/// `ExtraDataList::SetSound` (handle).
const EXTRA_SET_SOUND: u32 = 0x0041_a800;
/// Player weapon test (`00524d10`, `this` = the player).
const PLAYER_WEAPON_TEST: u32 = 0x0052_4d10;
/// Process virtual slot `0x3f4` (a flag).
const PROCESS_SLOT_3F4: u32 = 0x3f4;
/// `BGSEntryPoint::HandleEntryPoint` (cdecl: entry point, actor, weapon,
/// pointer to the value).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// Weapon `float` accessor (`004e4620`).
const WEAPON_FLOAT: u32 = 0x004e_4620;
/// `TESObjectWEAP::GetAttackSpeed` (mod effect flag; `float` in ST0).
const WEAPON_GET_ATTACK_SPEED: u32 = 0x0064_6020;
/// Animation call taking a `float` speed (`004c0c90`).
const ANIMATION_SET_SPEED: u32 = 0x004c_0c90;
/// Global transform tests (`004937c0`, `004937e0`).
const TRANSFORM_TEST_4937C0: u32 = 0x0049_37c0;
const TRANSFORM_TEST_4937E0: u32 = 0x0049_37e0;
/// Sequence call (`00598040`), returning a `float`.
const SEQUENCE_VALUE_598040: u32 = 0x0059_8040;
/// Animation test (`00498290`) used by the special idle.
const ANIMATION_TEST_498290: u32 = 0x0049_8290;
/// `Animation::SpecialIdleLoaded`.
const ANIMATION_SPECIAL_IDLE_LOADED: u32 = 0x0049_81f0;
/// Call on the global object `0x011f2250` (`0059bb30`).
const OBJECT_011F2250_CALL: u32 = 0x0059_bb30;
/// Global object whose word at `+8` says whether a special idle is playing
/// (it is `4` then).
const SPECIAL_IDLE_STATE_OBJECT: u32 = 0x011f_2250;
/// `008b28c0`: actor call taking an animation group and the animation
/// (plays an attack through the animation).
const ACTOR_PLAY_GROUP_WORKER: u32 = 0x008b_28c0;
/// Third word of an animation group table entry (+0xc), indexed by the
/// global transform's group number.
const ANIM_GROUP_TABLE_KIND: u32 = 0x0119_77e4;
/// `RandomFloat` (cdecl: low, high `float`s; result in ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `100.0f`-style upper bound the random roll uses (`float`).
const RANDOM_ROLL_MAX: u32 = 0x0101_6410;
/// Game settings (objects whose `float` `00403e20` returns a pointer to).
const SETTING_THRESHOLD_A: u32 = 0x011d_1380;
const SETTING_FACTOR_A: u32 = 0x011d_10f8;
const SETTING_THRESHOLD_B: u32 = 0x011d_1500;
const SETTING_FACTOR_B: u32 = 0x011d_0a0c;
/// Weapon accessors (`0051f5f0`, `00524b40`).
const WEAPON_SKILL_NUMBER: u32 = 0x0051_f5f0;
const WEAPON_BYTE_FLAG: u32 = 0x0052_4b40;
/// Animation accessor (`0047d3b0`) on the global transform: returns a `u16`.
const TRANSFORM_U16: u32 = 0x0047_d3b0;
/// `TESForm::GetFormByEditorID` (cdecl, name).
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
/// The editor ID the 0xa9 group looks up.
const EDITOR_ID_STRING: u32 = 0x0108_4af0;
/// Process virtual slots used by `Actor::StartAttack`.
const PROCESS_SLOT_1B0: u32 = 0x1b0;
const PROCESS_SLOT_71C: u32 = 0x71c;
/// Format of the message `Actor::StartAttack` prints for a group that is not
/// an attack.
const START_NON_ATTACK_FORMAT: u32 = 0x0108_4a88;
/// `100.0` is not assumed: the roll maximum is read from memory. The actor
/// value owner's virtual slot `8` (int result).
const OWNER_SLOT_8: u32 = 0x08;

/// Prints the debug message for an animation group that is not an attack
/// (`Actor::QueueAttack` / `Actor::StartAttack`): actor name, weapon name,
/// the requested group's name and the picked group's type name.
fn report_group_mismatch(
    e: &mut Engine,
    this: Ptr<Actor>,
    weapon_form: Ptr,
    group: u16,
    requested: u32,
    format: u32,
) {
    let group_type = e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32();
    let group_type_name = anim_group_name(e, group_type);
    let requested_name = anim_group_name(e, requested);
    let weapon_name = if weapon_form.is_null() {
        HAND_TO_HAND_NAME
    } else if e.call(FORM_HAS_EDITOR_NAME, &args![weapon_form]).u32() != 0 {
        e.vcall(weapon_form.addr(), 0x130, &args![]).u32()
    } else {
        e.call(
            MAP_MARKER_DATA_GET_LOCATION_NAME,
            &args![weapon_form.byte_add(0x30)],
        )
        .u32()
    };
    let actor_name = if e.call(FORM_HAS_EDITOR_NAME, &args![this]).u32() != 0 {
        e.vcall(this.addr(), 0x130, &args![]).u32()
    } else {
        e.call(REFR_GET_NAME, &args![this]).u32()
    };
    e.call(
        DEBUG_PRINT,
        &args![
            format,
            actor_name,
            weapon_name,
            requested_name,
            group_type_name
        ],
    );
}

/// Plays the weapon's dry-fire sound on the player (`Actor::StartAttack`):
/// reuses the sound stored in the actor's extra data when it is valid,
/// otherwise looks one up in the weapon's sound set, positions and plays it,
/// and stores it back.
/// In `00893a40`'s frame (`EBP` frame, exception record, `SUB ESP,0xf8`, `ESI`, the cookie: 0x110
/// bytes): the sound handle at +0xc8 (`EBP-0x48`), the looked-up handle at +0x6c
/// (`EBP-0xa4`) for the positioned sound or +0x60 (`EBP-0xb0`) for the other.
fn play_dry_fire_sound(e: &mut Engine, this: Ptr<Actor>, weapon_form: Ptr, player: Ptr<Actor>) {
    e.with_stack(0x110, |e, frame| {
        let frame = frame.addr();
        let sound = Ptr::<()>::new(frame + 0xc8);
        e.call(SOUND_HANDLE_CONSTRUCT, &args![sound]);
        let extra = extra_data_list_of(e, this.cast());
        e.call(EXTRA_GET_SOUND, &args![extra, sound]);
        if !e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
            let flags_and_position = e.call(PLAYER_WEAPON_TEST, &args![player]).bool();
            let (sound_flags, with_position) = if flags_and_position {
                (0x102u32, true)
            } else {
                (0x20_0101u32, false)
            };
            let temp = Ptr::<()>::new(frame + if with_position { 0x6c } else { 0x60 });
            let sound_set = e.call(WEAPON_SOUND_SET, &args![weapon_form]).u32();
            let sound_set_again = e.call(WEAPON_SOUND_SET, &args![weapon_form]).ptr::<()>();
            let file_name = e.call(SOUND_SET_FILE_NAME, &args![sound_set_again]).u32();
            let audio = e.call(AUDIO_INSTANCE, &args![]).ptr::<()>();
            let found = e
                .call(
                    AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
                    &args![audio, temp, file_name, sound_flags, sound_set],
                )
                .ptr::<()>();
            e.call(SOUND_HANDLE_ASSIGN, &args![sound, found]);
            e.call(SOUND_HANDLE_DESTROY, &args![temp]);
            if with_position {
                let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
                e.call(SOUND_HANDLE_SET_POSITION, &args![sound, position]);
            }
            e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
            let extra = extra_data_list_of(e, this.cast());
            e.call(EXTRA_SET_SOUND, &args![extra, sound]);
        }
        e.call(SOUND_HANDLE_DESTROY, &args![sound]);
    });
}

// Translated from 00893a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::StartAttack` (Xbox PDB): starts an attack with the animation group
/// `anim_group`. Returns true when the attack animation was started.
///
/// With a weapon that needs ammunition and none available (a Rock-It
/// launcher counts the player's Rock-It ammunition and opens a menu; other
/// weapons need the ammunition item to hold at least the weapon's per-shot
/// count) the player gets the dry-fire sound, process slot `0x3f4` is
/// called, and the attack fails. Otherwise a number of states make it fail
/// (animation actions 9 and 0x11, the weapon not drawn, process slot `0x3f8`
/// false, no animation, a package of type 8 or 0x10). A weapon that may jam,
/// or a power attack group `200..=216`, takes the jam / power group instead
/// and plays it through `008b28c0`. A melee-capable actor outside special
/// idles may have the group replaced by `0x38` or `0x3e` by a random roll
/// against two settings. The attack animation is then played in slot 4 with
/// the weapon's attack speed (entry point `0x2b`), after the checks the
/// animation's global transform imposes. A group that is not an attack is
/// only reported with a debug message.
///
/// The compiler's exception-unwinding frame (the `BSSoundHandle` temporaries)
/// is not translated.
pub fn actor_start_attack(e: &mut Engine, this: Ptr<Actor>, anim_group: u32) -> bool {
    let mut anim_group = anim_group;
    let player = player(e);
    let process = e.get(this, Actor::pCurrentProcess);
    let weapon_item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .ptr::<()>();
    let weapon_form = if weapon_item.is_null() {
        Ptr::NULL
    } else {
        let weapon_item = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .ptr::<()>();
        entry_form(e, weapon_item)
    };
    let mut ammo_item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_AMMO, &args![])
        .ptr::<()>();
    let mut shot_count = 0u32;
    if !weapon_form.is_null() {
        shot_count = u32::from(e.call(WEAPON_BYTE_102, &args![weapon_form]).u8());
    }

    if !weapon_form.is_null() {
        let mut out_of_ammo = false;
        let ammo_component = weapon_form.byte_add(0xa4);
        if e.call(AMMO_FORM_IS_ROCK_IT_LAUNCHER, &args![ammo_component])
            .bool()
        {
            out_of_ammo = true;
            if this == player && e.call(PLAYER_GET_NUM_ROCK_IT_AMMO, &args![player]).u32() != 0 {
                out_of_ammo = false;
            } else {
                e.call(
                    INTERFACE_QUEUE_MENU_CREATE,
                    &args![1u32, 0u32, 0u32, 0u32, 4u32, 0u32],
                );
            }
        } else if !e
            .call(WEAPON_GET_CURRENT_AMMO, &args![weapon_form, this])
            .ptr::<()>()
            .is_null()
        {
            if ammo_item.is_null() || e.call(LIST_NEXT, &args![ammo_item]).i32() < shot_count as i32
            {
                out_of_ammo = true;
            }
        } else if e.call(AMMO_COMPONENT_LIST, &args![ammo_component]).u32() != 0 {
            let list = e
                .call(AMMO_COMPONENT_LIST, &args![ammo_component])
                .ptr::<()>();
            let node = e.call(AMMO_LIST_FIRST_NODE, &args![list]).ptr::<()>();
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let form = e.mem.u32(slot);
            let ammo_form = e
                .call(
                    RTTI_DYNAMIC_CAST,
                    &args![form, 0u32, RTTI_TYPE_TES_FORM, RTTI_TYPE_AMMO_TARGET, 0u32],
                )
                .ptr::<()>();
            if !ammo_form.is_null() {
                ammo_item = e
                    .call(REFR_GET_INVENTORY_ITEM, &args![this, ammo_form, 0u32])
                    .ptr();
                if ammo_item.is_null()
                    || e.call(LIST_NEXT, &args![ammo_item]).i32() < shot_count as i32
                {
                    out_of_ammo = true;
                }
            }
        }
        if out_of_ammo && !ammo_item.is_null() && e.call(LIST_NEXT, &args![ammo_item]).u32() != 0 {
            out_of_ammo = false;
        }

        if out_of_ammo {
            let object = e.global::<u32>(OBJECT_POINTER_011DEA0C);
            let sound_object = e.call(OBJECT_011DEA0C_GET, &args![object]).u32();
            if e.call(WEAPON_SOUND_SET, &args![weapon_form]).u32() != 0 {
                let is_player = this == player;
                if is_player
                    && e.call(DRY_FIRE_ALLOWED, &args![sound_object, 4u32, 1u32])
                        .u32()
                        == 0
                {
                    return false;
                }
                if is_player {
                    play_dry_fire_sound(e, this, weapon_form, player);
                }
            }
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(process.addr(), PROCESS_SLOT_3F4, &args![0u32]);
            return false;
        }
    }

    if e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 9
        || e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 0x11
    {
        return false;
    }
    if !e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).bool() {
        return false;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !e.vcall(process.addr(), PROCESS_SLOT_3F8, &args![]).bool() {
        return false;
    }
    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .ptr::<()>();
    if animation.is_null() {
        return false;
    }

    // The package of the process decides whether the weapon may be used
    // (types 8 and 0x10 stop it).
    let mut package_allows = true;
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
    let package = e
        .vcall(acquire.addr(), PROCESS_SLOT_27C, &args![])
        .ptr::<()>();
    if !package.is_null() {
        package_allows = e.call(PACKAGE_TYPE, &args![package]).i32() != 8
            && e.call(PACKAGE_TYPE, &args![package]).i32() != 0x10;
    }
    if package_allows
        && !weapon_form.is_null()
        && !e
            .call(WEAPON_GET_CURRENT_AMMO, &args![weapon_form, this])
            .ptr::<()>()
            .is_null()
    {
        let process = e.get(this, Actor::pCurrentProcess);
        let condition = e
            .vcall(process.addr(), PROCESS_SLOT_368, &args![this])
            .u32();
        let is_power_group = (200..=216).contains(&(anim_group as i32));
        let mut take_special_group = is_power_group;
        if !is_power_group {
            let process = e.get(this, Actor::pCurrentProcess);
            if !e.vcall(process.addr(), PROCESS_SLOT_6DC, &args![]).bool() {
                let jam_multiplier = e
                    .call(GET_WEAPON_CONDITION_JAM_MULT, &args![condition])
                    .f32() as f64;
                let zero: f64 = e.global(ZERO_DOUBLE);
                if jam_multiplier > zero {
                    let jam_multiplier = e
                        .call(GET_WEAPON_CONDITION_JAM_MULT, &args![condition])
                        .f32();
                    if e.call(CHANCE_ROLL, &args![jam_multiplier]).bool() {
                        take_special_group = true;
                    }
                }
            }
        }
        if take_special_group {
            let special_group = if is_power_group {
                anim_group
            } else {
                e.call(WEAPON_GET_ATTACK_ANIM, &args![weapon_form, 0u32])
                    .u32()
                    .wrapping_add(0x17)
            };
            let group = actor_get_anim_group(e, this, special_group, 0, 0, Ptr::NULL);
            if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == special_group {
                e.call(
                    ACTOR_PLAY_GROUP_WORKER,
                    &args![this, special_group, animation],
                );
                let sequence = e
                    .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
                    .u32();
                e.call(ACTOR_SET_ANIM_ACTION, &args![this, 9u32, sequence]);
                e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(group), 1u32]);
            }
            return false;
        }
    }

    // Melee roll: a strong enough actor value may turn the attack into group
    // 0x38 or 0x3e.
    let special_idle = e.call(ENTRY_FORM, &args![SPECIAL_IDLE_STATE_OBJECT]).u32() == 4;
    let moving = e.call(ACTOR_IS_MOVING, &args![this]).bool();
    if (!moving || special_idle)
        && (e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool() || this == player)
        && (weapon_form.is_null() || e.call(WEAPON_TYPE, &args![weapon_form]).i32() == 0)
    {
        let mut is_ranged_skill = false;
        if !weapon_form.is_null() {
            let skill = e.call(WEAPON_SKILL_NUMBER, &args![weapon_form]).u32();
            if (0x4a..=0x5b).contains(&skill) {
                is_ranged_skill = true;
            }
        }
        let owner_value = e
            .vcall(this.addr() + 0xa4, OWNER_SLOT_8, &args![0x2du32])
            .i32();
        let owner_value = owner_value as f32;
        let mut rolled = false;
        let mut first_chance = 0.0f32;
        let mut second_chance = 0.0f32;
        if !is_ranged_skill {
            let pointer = e
                .call(SETTING_GET_VALUE_POINTER, &args![SETTING_THRESHOLD_A])
                .u32();
            let threshold = e.mem.f32(pointer);
            if (threshold as f64) < owner_value as f64 {
                rolled = true;
                let pointer = e
                    .call(SETTING_GET_VALUE_POINTER, &args![SETTING_FACTOR_A])
                    .u32();
                let factor = e.mem.f32(pointer);
                first_chance = (owner_value as f64 * factor as f64) as f32;
            }
        }
        if !is_ranged_skill {
            let pointer = e
                .call(SETTING_GET_VALUE_POINTER, &args![SETTING_THRESHOLD_B])
                .u32();
            let threshold = e.mem.f32(pointer);
            if (threshold as f64) < owner_value as f64 {
                rolled = true;
                let pointer = e
                    .call(SETTING_GET_VALUE_POINTER, &args![SETTING_FACTOR_B])
                    .u32();
                let factor = e.mem.f32(pointer);
                second_chance = (owner_value as f64 * factor as f64) as f32;
            }
        }
        if rolled && !special_idle {
            let roll_max: f32 = e.global(RANDOM_ROLL_MAX);
            let roll = e.call(RANDOM_FLOAT, &args![0.0f32, roll_max]).f32();
            if first_chance as f64 >= roll as f64 {
                anim_group = 0x38;
            } else if (roll as f64) < first_chance as f64 + second_chance as f64 {
                anim_group = 0x3e;
            }
        }
    }

    let mut special_idle_ready = false;
    if special_idle
        && e.call(OBJECT_011F2250_CALL, &args![SPECIAL_IDLE_STATE_OBJECT])
            .u32()
            != 0
        && e.call(ANIMATION_SPECIAL_IDLE_LOADED, &args![animation])
            .bool()
    {
        special_idle_ready = true;
    }
    let group = actor_get_anim_group(e, this, anim_group, 0, 0, Ptr::NULL);
    if !e
        .call(ANIM_GROUP_IS_ATTACK_ACTION, &args![u32::from(group)])
        .bool()
        && !special_idle_ready
    {
        report_group_mismatch(
            e,
            this,
            weapon_form,
            group,
            anim_group,
            START_NON_ATTACK_FORMAT,
        );
        return false;
    }

    // The attack animation.
    let mut started = true;
    if !weapon_form.is_null() {
        let mut attack_speed = e.call(WEAPON_FLOAT, &args![weapon_form]).f32();
        // The attack speed lives at +0x94 (`EBP-0x7c`) of the exe's 0x110-byte frame.
        attack_speed = e.with_stack(0x110, |e, frame| {
            let speed_pointer = frame.addr() + 0x94;
            e.mem.set_f32(speed_pointer, attack_speed);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x2bu32, this, weapon_form, speed_pointer],
            );
            e.mem.f32(speed_pointer)
        });
        let process = e.get(this, Actor::pCurrentProcess);
        let current = e
            .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .ptr::<()>();
        let has_mod_effect = e
            .call(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, &args![current, 8u32])
            .u8();
        let base_speed = e
            .call(
                WEAPON_GET_ATTACK_SPEED,
                &args![weapon_form, u32::from(has_mod_effect)],
            )
            .f32() as f64;
        let speed = (base_speed * attack_speed as f64) as f32;
        e.call(ANIMATION_SET_SPEED, &args![animation, speed]);
    } else {
        e.call(ANIMATION_SET_SPEED, &args![animation, 1.0f32]);
    }

    let sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
        .ptr::<()>();
    if sequence.is_null() {
        return false;
    }
    let transform = e
        .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
        .ptr::<()>();
    if e.call(TRANSFORM_TEST_4937C0, &args![transform]).bool() {
        if e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
            .u32()
            != 1
        {
            return false;
        }
        for slot in [5u32, 6u32] {
            if e.call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
                .u32()
                != 0
            {
                let other = e
                    .call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
                    .ptr::<()>();
                if e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![other]).u32() != 1 {
                    return false;
                }
            }
        }
        if this != player && e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool() {
            let transform = e
                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                .ptr::<()>();
            let moves = e.call(ANIM_GROUP_GET_MOVE, &args![transform]).u32();
            if moves == 1 {
                if !e.call(ACTOR_IS_MOVING, &args![this]).bool() {
                    return false;
                }
            } else if e.call(ACTOR_IS_MOVING, &args![this]).bool() {
                return false;
            }
        }
    } else {
        let transform = e
            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
            .ptr::<()>();
        if e.call(TRANSFORM_TEST_4937E0, &args![transform]).bool() {
            let transform = e
                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                .ptr::<()>();
            let number = e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).u32();
            let kind = e
                .mem
                .u32(ANIM_GROUP_TABLE_KIND.wrapping_add(number.wrapping_mul(0x24)));
            match kind {
                1 | 9 => {}
                5 => {
                    let power_with_zero_time = e
                        .call(ANIM_GROUP_IS_POWER_ATTACK_ACTION, &args![u32::from(group)])
                        .bool()
                        && {
                            let time = e.call(SEQUENCE_VALUE_598040, &args![sequence]).f32() as f64;
                            time == 0.0
                        };
                    if !power_with_zero_time
                        && e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32() != 3
                    {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }

    // Play it, unless that very group is already playing in slot 4.
    let playing = e
        .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
        .u16();
    let already_playing =
        playing == group && [0x4a, 0x4d, 0x50, 0x53, 0x56, 0x59].contains(&(anim_group as i32));
    if !already_playing {
        if special_idle_ready && anim_group < 0x61 && anim_group > 0x64 {
            started = e.call(ANIMATION_TEST_498290, &args![animation]).u8() != 0;
        } else if e
            .call(ANIM_GROUP_IS_POWER_ATTACK_ACTION, &args![u32::from(group)])
            .bool()
        {
            let played = e
                .call(
                    ANIMATION_PLAY_GROUP,
                    &args![
                        animation,
                        u32::from(group),
                        1u32,
                        0xffff_ffffu32,
                        0xffff_ffffu32
                    ],
                )
                .u32();
            started = played != 0;
        } else {
            e.call(ACTOR_PLAY_GROUP_WORKER, &args![this, anim_group, animation]);
        }
        let attack_sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
            .ptr::<()>();
        if !attack_sequence.is_null() {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
            let action = if e.vcall(acquire.addr(), PROCESS_SLOT_1B0, &args![]).bool() {
                5u32
            } else {
                2u32
            };
            e.call(ACTOR_SET_ANIM_ACTION, &args![this, action, attack_sequence]);
            if anim_group != 0xa9 {
                let transform = e
                    .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![attack_sequence])
                    .ptr::<()>();
                let value = e.call(TRANSFORM_U16, &args![transform]).u16();
                e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(value), 1u32]);
            } else {
                let form = e
                    .call(GET_FORM_BY_EDITOR_ID, &args![EDITOR_ID_STRING])
                    .u32();
                let idle = e
                    .call(
                        RTTI_DYNAMIC_CAST,
                        &args![form, 0u32, RTTI_TYPE_TES_FORM, RTTI_TYPE_IDLE_TARGET, 0u32],
                    )
                    .u32();
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
                e.vcall(acquire.addr(), PROCESS_SLOT_71C, &args![idle]);
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
                e.vcall(acquire.addr(), PROCESS_SLOT_614, &args![0x80u32]);
            }
        }
    }

    if !weapon_form.is_null() && e.call(WEAPON_BYTE_FLAG, &args![weapon_form]).bool() {
        let slot_four_group = e
            .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
            .u16();
        let group_type = e
            .call(ANIM_GROUP_GET_TYPE, &args![u32::from(slot_four_group)])
            .u16();
        if (0x18..=0xa8).contains(&group_type) {
            let sequence = e
                .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
                .ptr::<()>();
            if e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
                .u32()
                == 1
            {
                let process = e.get(this, Actor::pCurrentProcess);
                e.vcall(process.addr(), PROCESS_SLOT_614, &args![1u32]);
                let process = e.get(this, Actor::pCurrentProcess);
                e.vcall(process.addr(), PROCESS_SLOT_3F4, &args![0u32]);
            }
        }
    }
    started
}

/// `Actor` first-person check (`004eaf60`, `this` = the player).
const PLAYER_FIRST_PERSON_CHECK: u32 = 0x004e_af60;
/// `"AI: Don't call Actor::PickAnimations on the 1st person pc."`.
const FIRST_PERSON_MESSAGE: u32 = 0x0108_4b18;
/// `MobileObject::GetCharController`.
const MOBILE_OBJECT_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `PlayerCharacter::GetBiped` (flag).
const PLAYER_GET_BIPED: u32 = 0x0095_0b00;
/// `TESAnimGroup::IsJumpingLoopAnim` (this = the animation; the sequence is
/// the argument; result is a `float` in ST0).
const ANIM_GROUP_IS_JUMPING_LOOP_ANIM: u32 = 0x0049_3800;
/// Animation accessor (`005585e0`) used before `ClearGunWobble`.
const ANIMATION_GUN_WOBBLE_SOURCE: u32 = 0x0055_85e0;
/// `ClearGunWobble` (cdecl: table value, the accessor's result).
const CLEAR_GUN_WOBBLE: u32 = 0x008d_6a80;
/// `Actor::SetIronSights` (three words).
const ACTOR_SET_IRON_SIGHTS: u32 = 0x008b_b650;
/// Process virtual slot `0x190` (`MiddleHighProcess::GetWeaponBone` in the
/// high process vtable; takes the biped).
const PROCESS_SLOT_WEAPON_BONE: u32 = 0x190;
/// `0045bc00` (this = the bone, one word argument).
const BONE_NODE_LOOKUP: u32 = 0x0045_bc00;
/// Returns the name `NiObjectNET::GetExtraData` is asked for (`00448a80`).
const EXTRA_DATA_NAME: u32 = 0x0044_8a80;
/// `NiObjectNET::GetExtraData` (name).
const NI_OBJECT_NET_GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `00c74890` (this = `*(actor + 0xac)`; index, value).
const SET_SHADER_VALUE: u32 = 0x00c7_4890;
/// Process virtual slots used by `Actor::PickAnimations`.
const PROCESS_SLOT_QUEUED_ACTION: u32 = 0x3e4;
const PROCESS_SLOT_ANIM_SEQUENCE: u32 = 0x3e8;
const PROCESS_SLOT_SIT_SLEEP_STATE: u32 = 0x4bc;
const PROCESS_SLOT_FURNITURE_MARKER: u32 = 0x4d4;
const PROCESS_SLOT_AUTOMATIC_SHOT_DELAY: u32 = 0x444;
const PROCESS_SLOT_WEAPON_DRAWN: u32 = 0x454;
const PROCESS_SLOT_SET_WEAPON_DRAWN: u32 = 0x458;
const PROCESS_SLOT_SHADER_EFFECTS: u32 = 0x580;
const PROCESS_SLOT_WEAPON_ENCHANTMENT_VISUALS: u32 = 0x574;
const PROCESS_SLOT_SAVE_WEAPON_LAST_POSITION: u32 = 0x45c;
/// Actor virtual slot used by `Actor::PickAnimations` (see also `ACTOR_SLOT_234`).
const ACTOR_SLOT_230: u32 = 0x230;
/// Actor virtual slot `0x390` (non-zero while running).
const ACTOR_SLOT_RUNNING: u32 = 0x390;
/// `PlayerCharacter` virtual slot `0x358` (no arguments).
const PLAYER_SLOT_358: u32 = 0x358;
/// `ProcessLists::FindAndCleanupWeaponShaderHitEffect` (actor, effect).
const FIND_AND_CLEANUP_WEAPON_SHADER_HIT_EFFECT: u32 = 0x0097_4db0;
/// `MagicShaderHitEffect::ResetAlphaTimer`.
const RESET_ALPHA_TIMER: u32 = 0x0082_16c0;
/// `TESEnchantableForm::GetFormEnchanting` (cdecl, the weapon form).
const GET_FORM_ENCHANTING: u32 = 0x004b_e330;
/// `ItemChange::GetPoison`.
const ITEM_CHANGE_GET_POISON: u32 = 0x004b_dcc0;
/// `MobileObject::PlaySoundByEditorName` (out handle, name, 0, flags, 1).
const PLAY_SOUND_BY_EDITOR_NAME: u32 = 0x0093_3270;
/// `"WPNBlade1HandEquipEnchanted"`.
const ENCHANTED_EQUIP_SOUND_NAME: u32 = 0x0108_4afc;
/// `Actor::GetRunSpeed` (returns a `float` in ST0).
const ACTOR_GET_RUN_SPEED: u32 = 0x0088_4eb0;
/// Controller call (`005c0880`): the jump / fall state, 0 to 2.
const CONTROLLER_STATE: u32 = 0x005c_0880;
/// Controller call (`0088b0f0`, a `float` in ST0).
const CONTROLLER_FALL_TIME: u32 = 0x0088_b0f0;
/// Sequence test (`005f4d60`) on the global transform.
const TRANSFORM_IS_LOOPING: u32 = 0x005f_4d60;
/// The game setting object compared against the fall time.
const FALL_TIME_SETTING: u32 = 0x011d_f72c;
/// `Animation` call without arguments (`004974a0`).
const ANIMATION_LAND_RESET: u32 = 0x0049_74a0;
/// Offset of the object `0059bb30` is called on in the char controller.
const CONTROLLER_LAND_SOUND_OFFSET: u32 = 0x410;
/// `ImpactMixer::PlayJumpLand` (cdecl: actor, sound).
const IMPACT_MIXER_PLAY_JUMP_LAND: u32 = 0x0083_91b0;
/// `Actor::CanMove`.
const ACTOR_CAN_MOVE: u32 = 0x0088_43a0;
/// `Animation::SpecialIdleDonePlaying`.
const ANIMATION_SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
/// `Animation::SpecialIdlePlaying`.
const ANIMATION_SPECIAL_IDLE_PLAYING: u32 = 0x0049_85b0;
/// `Animation::SpecialIdleFree` (two flags).
const ANIMATION_SPECIAL_IDLE_FREE: u32 = 0x0049_8910;
/// `Animation::GroupLoaded`.
const ANIMATION_GROUP_LOADED: u32 = 0x0049_4710;
/// `TESAnimGroup::GetMove_ov2` (cdecl, one word).
const ANIM_GROUP_GET_MOVE_OV2: u32 = 0x005f_23c0;
/// Group test (`005f4cc0`, cdecl, one word).
const ANIM_GROUP_TEST_4CC0: u32 = 0x005f_4cc0;
/// Group test (`005f2700`, cdecl, one word).
const ANIM_GROUP_TEST_2700: u32 = 0x005f_2700;
/// Animation accessor (`00498cf0`).
const ANIMATION_CURRENT_BLEND_SEQUENCE: u32 = 0x0049_8cf0;
/// `Actor::GetWeaponPosition` (out `NiPoint3`, flag).
const ACTOR_GET_WEAPON_POSITION: u32 = 0x008a_5340;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;
/// Third entry word of the group table: how the group's sequence is used
/// (`+8`), indexed by group number times `0x24`.
const ANIM_GROUP_TABLE_MODE: u32 = 0x0119_77e0;
/// `CombatFormulas` call (`006477e0`, cdecl, the weapon condition word;
/// result in ST0).
const WEAPON_CONDITION_MULT_2: u32 = 0x0064_77e0;

/// The "slot mode" word of an animation group number (which animation slot
/// the group plays in).
fn group_mode(e: &Engine, number: u32) -> u32 {
    e.mem
        .u32(ANIM_GROUP_TABLE_MODE.wrapping_add(number.wrapping_mul(0x24)))
}

/// The "kind" word of an animation group number.
fn group_kind(e: &Engine, number: u32) -> u32 {
    e.mem
        .u32(ANIM_GROUP_TABLE_KIND.wrapping_add(number.wrapping_mul(0x24)))
}

/// The animation group number of an animation sequence
/// (`005f2420(Animation::ZeroGlobalTransform(sequence))`).
fn sequence_group_number(e: &mut Engine, sequence: Ptr) -> u32 {
    let transform = e
        .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
        .ptr::<()>();
    e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).u32()
}

/// The sequence the process currently plays (`HighProcess::GetAnimActionAnimSeq`
/// through virtual slot `0x3e8`).
fn process_sequence(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_SLOT_ANIM_SEQUENCE, &args![])
        .ptr()
}

fn set_anim_action(e: &mut Engine, this: Ptr<Actor>, action: u32, sequence: Ptr) {
    e.call(ACTOR_SET_ANIM_ACTION, &args![this, action, sequence]);
}

/// `true` when either `Actor::Test8BA3E0` or `Actor::Test8BA410` is set.
fn in_special_state(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.call(ACTOR_TEST_8BA3E0, &args![this]).bool() || e.call(ACTOR_TEST_8BA410, &args![this]).bool()
}

/// `PickBestAnimation(AnimGroup(selector_a, selector_b, group_type, special), 0)`.
fn compose_group(
    e: &mut Engine,
    animation: Ptr,
    selector_a: u32,
    selector_b: u32,
    group_type: u32,
    special: bool,
) -> u16 {
    let composed = e
        .call(
            ANIM_GROUP_COMPOSE,
            &args![selector_a, selector_b, group_type, u32::from(special)],
        )
        .u16();
    e.call(
        ANIMATION_PICK_BEST_ANIMATION,
        &args![animation, u32::from(composed), 0u32],
    )
    .u16()
}

/// Group `(group & 0xff00) | low` (the movement group of another speed).
fn movement_group(group: u16, low: u16) -> u16 {
    (group & 0xff00) | low
}

/// For a running actor, a movement group is mapped to the run (`|7`) or walk
/// (`|3`) variant (tables at `0x897848` / `0x89787c`: types 4, 5, 6 and 11 to
/// 14 walk, 8 to 10 run, 7 unchanged).
fn running_variant(e: &mut Engine, group: u16) -> u16 {
    let group_type = e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32();
    match group_type {
        4..=6 | 0xb..=0xe => movement_group(group, 3),
        8..=0xa => movement_group(group, 7),
        _ => group,
    }
}

/// Plays the biped/animation pairs of the actor (and for the player its
/// first-person pair) through process slot `0x1cc`.
fn force_weapon_drawn_pairs(
    e: &mut Engine,
    this: Ptr<Actor>,
    animation: Ptr,
    biped: u32,
    drawn: u32,
) {
    let player = player(e);
    let mut current_animation = animation;
    let mut current_biped = biped;
    let mut count = if this == player { 2 } else { 1 };
    while count != 0 {
        if this == player && count == 1 {
            current_animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).ptr();
            current_biped = e.call(PLAYER_GET_BIPED, &args![player, 1u32]).u32();
        }
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(
            process.addr(),
            PROCESS_SLOT_FORCE_WEAPON_DRAWN,
            &args![drawn, current_biped, current_animation, this],
        );
        count -= 1;
    }
}

fn process_of(e: &Engine, this: Ptr<Actor>) -> Ptr {
    e.get(this, Actor::pCurrentProcess)
}

/// The check `Actor::PickAnimations` makes first on the actor's animation and
/// then, for the player, on the first-person animation: if the animation's
/// attack sequence (slot 4) disagrees with the actor about moving and its
/// group differs from the group `Actor::GetAnimGroup` would pick now, the
/// slot is blended out; or when that sequence plays a group of type `0x4a` or
/// `0x4d` with no automatic shot delay it is blended out.
/// `group_animation` is the animation `GetAnimGroup` gets (null for the
/// actor's own).
fn blend_out_mismatched_attack(
    e: &mut Engine,
    this: Ptr<Actor>,
    animation: Ptr,
    group_animation: Ptr,
) {
    let sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
        .ptr::<()>();
    let mut check_looping_group = true;
    if !sequence.is_null() {
        let transform = e
            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
            .ptr::<()>();
        let group_moves = e.call(ANIM_GROUP_GET_MOVE, &args![transform]).u32() == 1;
        let actor_moves = e.call(ACTOR_IS_MOVING, &args![this]).u8();
        if u8::from(group_moves) != actor_moves
            && e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
                .u32()
                == 1
        {
            let transform = e
                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                .ptr::<()>();
            if e.call(TRANSFORM_TEST_4937C0, &args![transform]).bool() {
                let playing = e
                    .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
                    .u16();
                let number = sequence_group_number(e, sequence);
                let picked = actor_get_anim_group(e, this, number, 0, 0, group_animation);
                if playing != picked {
                    e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
                }
                check_looping_group = false;
            }
        }
    }
    if check_looping_group {
        let first = e
            .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
            .u16();
        let mut matches = e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(first)]).u32() == 0x4a;
        if !matches {
            let second = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
                .u16();
            matches = e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(second)]).u32() == 0x4d;
        }
        if matches {
            let process = process_of(e, this);
            let shot_delay = e
                .vcall(process.addr(), PROCESS_SLOT_AUTOMATIC_SHOT_DELAY, &args![])
                .f32() as f64;
            if shot_delay == 0.0
                && e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
                    .u32()
                    == 1
            {
                e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
            }
        }
    }
}

// Translated from 00895110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::PickAnimations` (Xbox PDB): the actor's animation state machine.
/// It blends out stale attack animations, reads the process's queued action
/// (`HighProcess::GetAnimAction`, virtual slot `0x3e4`) and its sequence
/// (slot `0x3e8`), and when the sequence finished (`GetGenericLocation`
/// non-zero) advances the action (weapon drawing and sheathing, reloading,
/// equipping, aiming, jumping and landing, post-attack states: the arms of
/// the `switch` on the queued action). Otherwise, with nothing queued, it
/// picks a locomotion or idle group from the actor's movement flags, sit/sleep
/// state and weapon, composes it with `TESAnimGroup::AnimGroup` and
/// `Animation::PickBestAnimation`, scales the animation speed (`arg_2` for
/// locomotion; `arg_3` for the two turn-in-place groups `0xf` and `0x10`),
/// and plays it in the slot the group table names.
///
/// `arg_2` and `arg_3` are the two `float` stack arguments (`RET 8`). The
/// compiler's exception-unwinding frame is not translated.
pub fn actor_pick_animations(e: &mut Engine, this: Ptr<Actor>, arg_2: f32, arg_3: f32) {
    let player = player(e);
    if e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return;
    }
    if e.vcall(this.addr(), ACTOR_SLOT_230, &args![]).bool() {
        return;
    }
    if this == player && !e.call(PLAYER_FIRST_PERSON_CHECK, &args![player]).bool() {
        e.call(DEBUG_PRINT, &args![FIRST_PERSON_MESSAGE]);
        return;
    }
    let animation = e
        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
        .ptr::<()>();
    let controller = e
        .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
        .ptr::<()>();
    if animation.is_null() || process_of(e, this).is_null() || controller.is_null() {
        return;
    }
    let process = process_of(e, this);
    if e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0 {
        e.call(SOUND_HANDLE_DESTROY, &args![process]);
    }

    blend_out_mismatched_attack(e, this, animation, Ptr::NULL);
    if this == player {
        let player_animation = e
            .call(PLAYER_GET_ANIMATION, &args![player, 1u32])
            .ptr::<()>();
        blend_out_mismatched_attack(e, this, player_animation, player_animation);
    }

    let mut speed_scale = 1.0f32;
    let mut move_speed = 0.0f32;
    let is_special = e.call(ACTOR_TEST_8A6970, &args![this]).u8() != 0;
    let mut group_type = 0u32;
    let mut selector_a = 0u32;
    let mut selector_b = 0u32;
    let mut new_action: i32 = -1;
    let slot1_sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![animation, 1u32])
        .ptr::<()>();
    let biped = e.vcall(this.addr(), ACTOR_SLOT_1E8, &args![]).u32();
    let process = process_of(e, this);
    let weapon_item = e
        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .ptr::<()>();
    let weapon_form = if weapon_item.is_null() {
        Ptr::NULL
    } else {
        entry_form(e, weapon_item)
    };
    let flags = e.call(ACTOR_ANIM_FLAGS, &args![this]).u16();
    let process = process_of(e, this);
    let mut queued_action = e
        .vcall(process.addr(), PROCESS_SLOT_QUEUED_ACTION, &args![])
        .i32();
    let process = process_of(e, this);
    let sit_state = e
        .vcall(process.addr(), PROCESS_SLOT_SIT_SLEEP_STATE, &args![])
        .i32();
    let process = process_of(e, this);
    let furniture = e
        .vcall(process.addr(), PROCESS_SLOT_FURNITURE_MARKER, &args![])
        .ptr::<()>();

    if queued_action != -1 {
        let mut switch_taken = false;
        let process = process_of(e, this);
        if e.vcall(process.addr(), PROCESS_SLOT_ANIM_SEQUENCE, &args![])
            .u32()
            != 0
        {
            let sequence = process_sequence(e, this);
            if e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
                .u32()
                != 0
            {
                switch_taken = true;
                match queued_action as u32 {
                    0x11 => {
                        let sequence = process_sequence(e, this);
                        let number = sequence_group_number(e, sequence);
                        let process = process_of(e, this);
                        if is_special
                            && !e
                                .vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
                                .bool()
                            && !weapon_form.is_null()
                            && !weapon_item.is_null()
                        {
                            let mod_active = e
                                .call(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, &args![weapon_item, 2u32])
                                .u8();
                            e.vcall(
                                this.addr(),
                                ACTOR_SLOT_3EC,
                                &args![weapon_form, 0u32, u32::from(mod_active), 1u32],
                            );
                            new_action = -1;
                            set_anim_action(e, this, 0xffff_ffff, Ptr::NULL);
                        } else if number as i32 <= 0xb0 {
                            let loop_time = e
                                .call(ANIM_GROUP_IS_JUMPING_LOOP_ANIM, &args![animation, sequence])
                                .f32() as f64;
                            let transform = e
                                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                                .ptr::<()>();
                            let time =
                                e.call(ANIM_GROUP_GET_TIME, &args![transform, 1u32]).f32() as f64;
                            if time <= loop_time {
                                let mod_active = e
                                    .call(
                                        ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE,
                                        &args![weapon_item, 2u32],
                                    )
                                    .u8();
                                let land_group = e
                                    .call(
                                        WEAPON_GET_ATTACK_ANIM,
                                        &args![weapon_form, u32::from(mod_active)],
                                    )
                                    .u32();
                                let group =
                                    actor_get_anim_group(e, this, land_group, 0, 0, Ptr::NULL);
                                if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32()
                                    == land_group
                                {
                                    let own_animation = e
                                        .vcall(this.addr(), ACTOR_SLOT_GET_ANIMATION, &args![])
                                        .ptr::<()>();
                                    e.call(
                                        ANIMATION_PLAY_GROUP,
                                        &args![
                                            own_animation,
                                            u32::from(group),
                                            1u32,
                                            0xffff_ffffu32,
                                            0xffff_ffffu32
                                        ],
                                    );
                                    if this == player {
                                        e.vcall(
                                            player.addr(),
                                            ACTOR_SLOT_4B0,
                                            &args![u32::from(group), 1u32],
                                        );
                                    }
                                }
                                new_action = 0x11;
                                let attack_sequence = e
                                    .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
                                    .ptr::<()>();
                                set_anim_action(e, this, 0x11, attack_sequence);
                            }
                        } else if e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32() == 1 {
                            let process = process_of(e, this);
                            e.vcall(process.addr(), PROCESS_SLOT_614, &args![0x4_0000u32]);
                        }
                    }
                    5 => {
                        let mut wanted_state = 2;
                        let sequence = process_sequence(e, this);
                        let number = sequence_group_number(e, sequence);
                        let kind = group_kind(e, number);
                        let applies = match kind {
                            7 => true,
                            8 => {
                                wanted_state = 1;
                                true
                            }
                            _ => false,
                        };
                        if applies
                            && e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32()
                                == wanted_state
                        {
                            if !weapon_form.is_null()
                                && !e.call(WEAPON_FORM_ALLOWS_BLOCK, &args![weapon_form]).bool()
                            {
                                let process = process_of(e, this);
                                e.vcall(process.addr(), PROCESS_SLOT_614, &args![1u32]);
                            }
                            if fn_00894900(e, this) {
                                let sequence = process_sequence(e, this);
                                set_anim_action(e, this, 6, sequence);
                            }
                        }
                    }
                    6 => {
                        let mut wanted_state = 3;
                        let sequence = process_sequence(e, this);
                        let number = sequence_group_number(e, sequence);
                        let kind = group_kind(e, number);
                        let applies = match kind {
                            7 => true,
                            8 => {
                                wanted_state = 2;
                                true
                            }
                            _ => false,
                        };
                        if applies
                            && e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32()
                                == wanted_state
                        {
                            if !weapon_form.is_null()
                                && !e.call(WEAPON_FORM_ALLOWS_BLOCK, &args![weapon_form]).bool()
                            {
                                force_weapon_drawn_pairs(e, this, animation, biped, 1);
                            }
                            if fn_00894900(e, this) {
                                let sequence = process_sequence(e, this);
                                set_anim_action(e, this, 4, sequence);
                            }
                        }
                    }
                    2 => {
                        let sequence = process_sequence(e, this);
                        let number = sequence_group_number(e, sequence);
                        let mode = group_mode(e, number);
                        if mode == 2 {
                            if e.call(ANIMATION_SLOT_STATE, &args![animation, 2u32]).i32() == 1 {
                                let group = e
                                    .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 2u32])
                                    .u16();
                                let group_type_of =
                                    e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32();
                                let kind = group_kind(e, group_type_of) as i32;
                                if (5..=6).contains(&kind) {
                                    release_cast_if_casting(e, this);
                                    let sequence = process_sequence(e, this);
                                    set_anim_action(e, this, 4, sequence);
                                }
                            }
                        } else if mode == 4 {
                            queued_action_2_mode_4(e, this, animation, weapon_form);
                        }
                    }
                    3 => {
                        let sequence = process_sequence(e, this);
                        let number = sequence_group_number(e, sequence);
                        if group_mode(e, number) == 4 && !weapon_form.is_null() {
                            let sequence = process_sequence(e, this);
                            let number = sequence_group_number(e, sequence);
                            if group_kind(e, number) != 1
                                && e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32() == 2
                            {
                                let process = process_of(e, this);
                                e.vcall(process.addr(), PROCESS_SLOT_614, &args![0x2_0000u32]);
                                let sequence = process_sequence(e, this);
                                set_anim_action(e, this, 4, sequence);
                            }
                        }
                    }
                    0 | 1 => {
                        queued_action_draw_or_sheathe(
                            e,
                            this,
                            animation,
                            biped,
                            weapon_form,
                            weapon_item,
                            queued_action,
                        );
                    }
                    0xc => {
                        if sit_state == 0 {
                            let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
                            if state == 1 {
                                set_anim_action(e, this, 0xffff_ffff, Ptr::NULL);
                            } else if state == 2 && !slot1_sequence.is_null() {
                                let transform = e
                                    .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![slot1_sequence])
                                    .ptr::<()>();
                                if e.call(TRANSFORM_IS_LOOPING, &args![transform]).bool() {
                                    set_anim_action(e, this, 0xffff_ffff, Ptr::NULL);
                                }
                            }
                        }
                    }
                    0xb => {
                        move_speed = e.call(ACTOR_GET_WALK_SPEED, &args![this]).f32();
                    }
                    _ => {}
                }
            }
        }
        if !switch_taken {
            let slot2 = e
                .call(ANIMATION_GET_SEQUENCE, &args![animation, 2u32])
                .u32();
            if slot2 != 0 {
                let group = e
                    .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 2u32])
                    .u16();
                if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == 0xaa {
                    new_action = 7;
                    let sequence = e
                        .call(ANIMATION_GET_SEQUENCE, &args![animation, 2u32])
                        .ptr::<()>();
                    set_anim_action(e, this, new_action as u32, sequence);
                }
            }
            if !weapon_form.is_null() && !fn_00897890(e, weapon_form) && queued_action == 9 {
                let sequence = process_sequence(e, this);
                let number = sequence_group_number(e, sequence);
                if e.call(ANIM_GROUP_TEST_2700, &args![number]).bool() {
                    let process = process_of(e, this);
                    let condition = e
                        .vcall(process.addr(), PROCESS_SLOT_368, &args![this])
                        .u32();
                    let chance = e.call(WEAPON_CONDITION_MULT_2, &args![condition]).f32() as f64;
                    let zero: f64 = e.global(ZERO_DOUBLE);
                    if chance > zero {
                        let chance = e.call(WEAPON_CONDITION_MULT_2, &args![condition]).f32();
                        if e.call(CHANCE_ROLL, &args![chance]).bool() {
                            let attack_group = e
                                .call(WEAPON_GET_ATTACK_ANIM, &args![weapon_form, 0u32])
                                .u32()
                                .wrapping_add(0x17);
                            let group =
                                actor_get_anim_group(e, this, attack_group, 0, 0, Ptr::NULL);
                            if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32()
                                == attack_group
                            {
                                new_action = 9;
                                e.call(
                                    ACTOR_PLAY_GROUP_WORKER,
                                    &args![this, attack_group, animation],
                                );
                                let sequence = e
                                    .call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
                                    .ptr::<()>();
                                set_anim_action(e, this, 9, sequence);
                                e.vcall(
                                    this.addr(),
                                    ACTOR_SLOT_4B0,
                                    &args![u32::from(group), 1u32],
                                );
                            }
                        }
                    }
                }
            }
            if new_action == -1 {
                set_anim_action(e, this, 0xffff_ffff, Ptr::NULL);
            }
        }
    }

    // Nothing was started by the queued action.
    if new_action == -1 {
        if sit_state == 0 {
            if matches!(queued_action, -1 | 2 | 4 | 0xc) {
                let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
                if state == 0 {
                    if !slot1_sequence.is_null() {
                        let transform = e
                            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![slot1_sequence])
                            .ptr::<()>();
                        let looping = e.call(TRANSFORM_IS_LOOPING, &args![transform]).bool();
                        let landing = looping || {
                            let transform = e
                                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![slot1_sequence])
                                .ptr::<()>();
                            e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).u32() == 0xe3
                        };
                        if landing {
                            queued_action = -1;
                            new_action = 0xc;
                            group_type = if flags & 1 != 0 {
                                0xf1
                            } else if flags & 2 != 0 {
                                0xf2
                            } else if flags & 4 != 0 {
                                0xf3
                            } else if flags & 8 != 0 {
                                0xf4
                            } else {
                                0xe5
                            };
                            e.call(ANIMATION_LAND_RESET, &args![animation]);
                            let landing_sound = e
                                .call(
                                    LANDING_SOUND_LOOKUP,
                                    &args![controller.byte_add(CONTROLLER_LAND_SOUND_OFFSET)],
                                )
                                .u32();
                            e.call(IMPACT_MIXER_PLAY_JUMP_LAND, &args![this, landing_sound]);
                        }
                    }
                } else if state == 1 {
                    group_type = 0xe4;
                } else if state == 2 {
                    let fall_time = e.call(CONTROLLER_FALL_TIME, &args![controller]).f32() as f64;
                    let pointer = e
                        .call(SETTING_GET_VALUE_POINTER, &args![FALL_TIME_SETTING])
                        .u32();
                    let threshold = e.mem.f32(pointer) as f64;
                    if threshold < fall_time {
                        group_type = 0xe4;
                    } else if !slot1_sequence.is_null() {
                        let transform = e
                            .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![slot1_sequence])
                            .ptr::<()>();
                        if e.call(TRANSFORM_IS_LOOPING, &args![transform]).bool() {
                            group_type = 0xe4;
                        }
                    }
                }
            }
            if is_special {
                let process = process_of(e, this);
                if !e
                    .vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
                    .bool()
                    && queued_action == -1
                    && e.call(ACTOR_CAN_MOVE, &args![this]).bool()
                {
                    if !e
                        .call(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, &args![animation])
                        .bool()
                    {
                        e.call(ANIMATION_SPECIAL_IDLE_FREE, &args![animation, 1u32, 1u32]);
                    }
                    group_type = 0x18;
                    new_action = 0;
                }
            }
        }
        if !is_special {
            let process = process_of(e, this);
            if e.vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
                .bool()
                && queued_action == -1
                && !e.vcall(this.addr(), ACTOR_SLOT_234, &args![]).bool()
                && !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
                && !e.vcall(this.addr(), ACTOR_SLOT_230, &args![]).bool()
                && !e.call(EXTRA_TYPE_IS_5, &args![this]).bool()
                && !e.call(EXTRA_OBJECTS_TEST, &args![this]).bool()
            {
                group_type = 0x19;
                new_action = 1;
            }
        }
    }

    // The second selector (from the weapon).
    let running = e.vcall(this.addr(), ACTOR_SLOT_RUNNING, &args![]).u32() != 0;
    let process = process_of(e, this);
    let weapon_selector_applies = (!running
        && (is_special || e.call(ACTOR_IN_COMBAT, &args![this]).bool()))
        || e.vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
            .bool()
        || new_action == 0
        || new_action == 1;
    if weapon_selector_applies {
        if weapon_item.is_null() {
            selector_b = 1;
        } else {
            let form = entry_form(e, weapon_item);
            let weapon_type = e.call(WEAPON_TYPE, &args![form]).u32();
            selector_b = e
                .mem
                .u32(WEAPON_TYPE_ANIM_TABLE.wrapping_add(weapon_type.wrapping_mul(4)));
        }
    }
    if flags & 0x800 != 0 {
        selector_a = 2;
    } else if flags & 0x2000 != 0 {
        selector_a = 3;
    } else if flags & 0x400 != 0 {
        selector_a = 1;
    }

    if group_type == 0 {
        match sit_state {
            3 | 8 => {
                if e.call(ANIMATION_SPECIAL_IDLE_PLAYING, &args![animation])
                    .bool()
                    || e.call(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, &args![animation])
                        .bool()
                {
                    let small = furniture.is_null() || e.mem.u8(furniture.addr() + 0xe) <= 0x14;
                    if small {
                        group_type = 1;
                    }
                }
            }
            4 | 5 | 9 | 10 => {
                let mut skip = false;
                if this == player {
                    let player_animation = e
                        .call(PLAYER_GET_ANIMATION, &args![player, 1u32])
                        .ptr::<()>();
                    let list = player_animation.byte_add(0x128);
                    if e.call(ENTRY_FIRST_EXTRA_LIST, &args![list]).u32() != 0 {
                        skip = true;
                    }
                }
                if !skip {
                    let small = furniture.is_null() || e.mem.u8(furniture.addr() + 0xe) <= 0x14;
                    if small {
                        group_type = 1;
                    }
                }
            }
            _ => {}
        }
        speed_scale = arg_2;
        if flags & 0xf != 0 {
            if flags & 0x200 != 0 {
                if flags & 1 != 0 {
                    group_type = 7;
                } else if flags & 2 != 0 {
                    group_type = 8;
                } else if flags & 4 != 0 {
                    group_type = 9;
                } else if flags & 8 != 0 {
                    group_type = 0xa;
                }
                move_speed = if e.vcall(this.addr(), ACTOR_SLOT_RUNNING, &args![]).u32() == 0 {
                    e.call(ACTOR_GET_WALK_SPEED, &args![this]).f32()
                } else {
                    e.call(ACTOR_GET_RUN_SPEED, &args![this]).f32()
                };
            } else if flags & 0xff00 != 0 {
                if flags & 1 != 0 {
                    group_type = 3;
                } else if flags & 2 != 0 {
                    group_type = 4;
                } else if flags & 4 != 0 {
                    group_type = 5;
                } else if flags & 8 != 0 {
                    group_type = 6;
                }
                move_speed = e.call(ACTOR_GET_WALK_SPEED, &args![this]).f32();
            }
        } else if flags & 0x10 != 0 {
            group_type = 0xf;
        } else if flags & 0x20 != 0 {
            group_type = 0x10;
        }
    } else if group_type == 0xe4 && flags & 0xf != 0 {
        if flags & 1 != 0 {
            group_type = 0xec;
        } else if flags & 2 != 0 {
            group_type = 0xed;
        } else if flags & 4 != 0 {
            group_type = 0xee;
        } else if flags & 8 != 0 {
            group_type = 0xef;
        }
    }

    if new_action != -1 && queued_action != -1 {
        return;
    }
    let one: f64 = e.global(ONE_DOUBLE);
    if (move_speed as f64) < one
        && (3..=0x10).contains(&(group_type as i32))
        && group_type != 0xf
        && group_type != 0x10
    {
        if this == player {
            e.vcall(player.addr(), PLAYER_SLOT_358, &args![]);
        }
        group_type = 0;
    }

    let special = in_special_state(e, this);
    let mut picked = compose_group(e, animation, selector_a, selector_b, group_type, special);
    let picked_type = e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(picked)]).u32();
    if new_action != -1 && group_type != picked_type {
        if new_action == 1 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
            e.vcall(
                acquire.addr(),
                PROCESS_SLOT_SET_WEAPON_DRAWN,
                &args![this, 0u32],
            );
        }
        new_action = -1;
    }
    group_type = picked_type;

    let process = process_of(e, this);
    if queued_action != -1
        && e.vcall(process.addr(), PROCESS_SLOT_ANIM_SEQUENCE, &args![])
            .u32()
            != 0
    {
        let slot = group_mode(e, group_type);
        let slot_sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
            .ptr::<()>();
        let current = process_sequence(e, this);
        if slot_sequence == current {
            if queued_action != 0xe {
                return;
            }
            picked = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 1u32])
                .u16();
            if e.vcall(this.addr(), ACTOR_SLOT_RUNNING, &args![]).u32() == 0 {
                picked = movement_group(picked, 3);
            } else {
                picked = running_variant(e, picked);
            }
            let picked_kind = e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(picked)]).u32();
            match picked_kind {
                3..=6 | 0xb..=0xe => {
                    move_speed = e.call(ACTOR_GET_WALK_SPEED, &args![this]).f32();
                }
                7..=0xa => {
                    move_speed = e.call(ACTOR_GET_RUN_SPEED, &args![this]).f32();
                }
                0xf | 0x10 => {
                    fn_008950f0(e, animation, arg_3);
                    return;
                }
                _ => {}
            }
            let group_speed = e
                .call(
                    ANIMATION_MOVEMENT_SPEED,
                    &args![animation, u32::from(picked)],
                )
                .u16() as i16;
            let group_speed = f32::from(group_speed);
            if group_speed != 0.0 {
                speed_scale =
                    ((move_speed as f64 / group_speed as f64) * speed_scale as f64) as f32;
            }
            fn_008950f0(e, animation, speed_scale);
            return;
        }
    }

    if picked != 0xff {
        if group_type == 0xf || group_type == 0x10 {
            fn_008950f0(e, animation, arg_3);
        } else if (3..=0x10).contains(&(group_type as i32)) {
            let speed_group = if e.vcall(this.addr(), ACTOR_SLOT_RUNNING, &args![]).u32() == 0 {
                movement_group(picked, 3)
            } else {
                running_variant(e, picked)
            };
            let group_speed = e
                .call(
                    ANIMATION_MOVEMENT_SPEED,
                    &args![animation, u32::from(speed_group)],
                )
                .u16() as i16;
            let group_speed = f32::from(group_speed);
            if group_speed != 0.0 {
                speed_scale =
                    ((move_speed as f64 / group_speed as f64) * speed_scale as f64) as f32;
            }
            fn_008950f0(e, animation, speed_scale);
        } else if (0x18..=0xa8).contains(&(group_type as i32)) {
            if weapon_form.is_null() {
                e.call(ANIMATION_SET_SPEED, &args![animation, 1.0f32]);
            } else {
                let base_speed = e.call(WEAPON_FLOAT, &args![weapon_form]).f32();
                let speed_pointer = e.mem.alloc(4);
                e.mem.set_f32(speed_pointer, base_speed);
                e.call(
                    HANDLE_ENTRY_POINT,
                    &args![0x2bu32, this, weapon_form, speed_pointer],
                );
                let entry_speed = e.mem.f32(speed_pointer);
                e.mem.free(speed_pointer);
                if e.call(ANIM_GROUP_IS_ATTACK_ACTION, &args![group_type & 0xffff])
                    .bool()
                    && !e.call(WEAPON_FORM_ALLOWS_BLOCK, &args![weapon_form]).bool()
                {
                    let process = process_of(e, this);
                    let current = e
                        .vcall(process.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
                        .ptr::<()>();
                    let mod_active = e
                        .call(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, &args![current, 8u32])
                        .u8();
                    let attack_speed = e
                        .call(
                            WEAPON_GET_ATTACK_SPEED,
                            &args![weapon_form, u32::from(mod_active)],
                        )
                        .f32() as f64;
                    let speed = (attack_speed * entry_speed as f64) as f32;
                    e.call(ANIMATION_SET_SPEED, &args![animation, speed]);
                } else {
                    e.call(ANIMATION_SET_SPEED, &args![animation, entry_speed]);
                }
            }
        }
    }

    play_picked_group(
        e, this, animation, picked, group_type, new_action, flags, selector_a, selector_b,
    );

    // The movement slot is blended out when it no longer matches.
    let slot1 = e
        .call(ANIMATION_GET_SEQUENCE, &args![animation, 1u32])
        .u32();
    if slot1 != 0 && group_mode(e, group_type) != 1 {
        let mut blend = true;
        if queued_action != -1 {
            let current = process_sequence(e, this);
            let sequence = e
                .call(ANIMATION_GET_SEQUENCE, &args![animation, 1u32])
                .ptr::<()>();
            if current == sequence {
                blend = false;
            }
        }
        let blend_sequence = e
            .call(ANIMATION_CURRENT_BLEND_SEQUENCE, &args![animation])
            .ptr::<()>();
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, 1u32])
            .ptr::<()>();
        if blend_sequence == sequence {
            blend = false;
        }
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, 1u32])
            .ptr::<()>();
        if e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
            .u32()
            != 1
        {
            blend = false;
        }
        if blend {
            e.call(ANIMATION_BLEND_OUT, &args![animation, 1u32, 0u32]);
            if this == player {
                let player_animation = e
                    .call(PLAYER_GET_ANIMATION, &args![player, 1u32])
                    .ptr::<()>();
                e.call(ANIMATION_BLEND_OUT, &args![player_animation, 1u32, 0u32]);
            }
        }
    }

    // The weapon's position is saved while an attack animation plays.
    if e.call(ANIMATION_GET_SEQUENCE, &args![animation, 4u32])
        .u32()
        != 0
    {
        let attack_group = e
            .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
            .u16();
        if e.call(ANIM_GROUP_IS_ATTACK_ACTION, &args![u32::from(attack_group)])
            .bool()
        {
            let mut position = [0u32; 3];
            e.with_stack(12, |e, out| {
                let result = e
                    .call(ACTOR_GET_WEAPON_POSITION, &args![this, out, 0u32])
                    .u32();
                for (i, word) in position.iter_mut().enumerate() {
                    *word = e.mem.u32(result + 4 * i as u32);
                }
            });
            let process = process_of(e, this);
            e.vcall(
                process.addr(),
                PROCESS_SLOT_SAVE_WEAPON_LAST_POSITION,
                &args![position[0], position[1], position[2]],
            );
        }
    }
}

/// `MagicCaster::ReleaseCast` (this = the caster at `actor + 0x88`, flag).
const MAGIC_CASTER_RELEASE_CAST: u32 = 0x0081_5870;
/// Animation call (`00494300`) is [`ANIMATION_MOVEMENT_SPEED`]; the weapon
/// type table is [`WEAPON_TYPE_ANIM_TABLE`].
/// Weapon test (`004c0c30`).
const WEAPON_TEST_4C0C30: u32 = 0x004c_0c30;
/// `ClearGunWobble` argument table is the same as [`WEAPON_TYPE_ANIM_TABLE`].
/// Lookup of the sound the landing plays (`0059bb30` on the controller's
/// object at `+0x410`).
const LANDING_SOUND_LOOKUP: u32 = 0x0059_bb30;

/// Releases the spell the actor is casting (`MagicCaster::ReleaseCast` on the
/// caster at `+0x88`) when its virtual slot `0x34` says it is casting.
fn release_cast_if_casting(e: &mut Engine, this: Ptr<Actor>) {
    let caster = this.byte_add(0x88);
    if e.vcall(caster.addr(), 0x34, &args![]).u32() != 0 {
        e.call(MAGIC_CASTER_RELEASE_CAST, &args![caster, 0u32]);
    }
}

/// Sets the animation action and, for the arm of the `switch` in
/// `Actor::PickAnimations` that handles queued action 2 with slot mode 4
/// (the attack slot): depending on the kind of the group playing in slot 4
/// it releases a cast and queues a post-attack state.
fn queued_action_2_mode_4(e: &mut Engine, this: Ptr<Actor>, animation: Ptr, weapon_form: Ptr) {
    let player = player(e);
    let sequence = process_sequence(e, this);
    let number = sequence_group_number(e, sequence);
    if group_kind(e, number) == 1 {
        return;
    }
    if e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32() != 1 {
        return;
    }
    let slot_group = e
        .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
        .u16();
    let group_type = e
        .call(ANIM_GROUP_GET_TYPE, &args![u32::from(slot_group)])
        .u32();
    match group_kind(e, group_type) {
        9 => {
            let process = process_of(e, this);
            let delay = e
                .vcall(process.addr(), PROCESS_SLOT_AUTOMATIC_SHOT_DELAY, &args![])
                .f32() as f64;
            if delay == 0.0 {
                let sequence = process_sequence(e, this);
                set_anim_action(e, this, 4, sequence);
            }
        }
        6 => {
            if this == player || e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
                e.call(ACTOR_FN_00899200, &args![this, 1u32, 1u32]);
            } else {
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
                let package = e
                    .vcall(acquire.addr(), PROCESS_SLOT_27C, &args![])
                    .ptr::<()>();
                let mut threw_special = false;
                if !package.is_null()
                    && (e.call(PACKAGE_TYPE, &args![package]).i32() == 8
                        || e.call(PACKAGE_TYPE, &args![package]).i32() == 0x10)
                {
                    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
                    if !e.vcall(acquire.addr(), PROCESS_SLOT_1B0, &args![]).bool() {
                        e.call(ACTOR_FN_00899200, &args![this, 0u32, 1u32]);
                        threw_special = true;
                    }
                }
                if !threw_special {
                    e.call(ACTOR_FN_00899200, &args![this, 1u32, 0u32]);
                }
            }
            if fn_00894900(e, this) {
                let sequence = process_sequence(e, this);
                set_anim_action(e, this, 4, sequence);
            }
        }
        5 => {
            if e.vcall(this.addr() + 0x88, 0x34, &args![]).u32() != 0 {
                release_cast_if_casting(e, this);
            } else if weapon_form.is_null() || !e.call(WEAPON_BYTE_FLAG, &args![weapon_form]).bool()
            {
                if !weapon_form.is_null() && e.call(WEAPON_TEST_4C0C30, &args![weapon_form]).bool()
                {
                    let process = process_of(e, this);
                    e.vcall(process.addr(), PROCESS_SLOT_614, &args![1u32]);
                } else if this == player || e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
                    e.call(ACTOR_FN_00899200, &args![this, 0u32, 1u32]);
                } else {
                    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).ptr::<()>();
                    let package = e
                        .vcall(acquire.addr(), PROCESS_SLOT_27C, &args![])
                        .ptr::<()>();
                    let special_package = !package.is_null()
                        && (e.call(PACKAGE_TYPE, &args![package]).i32() == 8
                            || e.call(PACKAGE_TYPE, &args![package]).i32() == 0x10);
                    let flag = u32::from(special_package);
                    e.call(ACTOR_FN_00899200, &args![this, 0u32, flag]);
                }
            }
            if fn_00894900(e, this) {
                if !weapon_form.is_null()
                    && !e.call(WEAPON_BYTE_FLAG, &args![weapon_form]).bool()
                    && !e.call(WEAPON_FORM_ALLOWS_BLOCK, &args![weapon_form]).bool()
                {
                    let sequence = process_sequence(e, this);
                    set_anim_action(e, this, 3, sequence);
                } else {
                    let sequence = process_sequence(e, this);
                    set_anim_action(e, this, 4, sequence);
                }
            }
        }
        _ => {}
    }
}

/// `Actor::fn_00899200` (a later function of this part): called with the
/// actor and two flags.
const ACTOR_FN_00899200: u32 = 0x0089_9200;

/// The arm of the `switch` in `Actor::PickAnimations` for queued actions 0
/// (sheathe) and 1 (draw): if the weapon is not already in the wanted state
/// and the attack slot is busy, the process is told (`SetWeaponDrawn`), the
/// animation pairs are forced to match (clearing the gun wobble first), iron
/// sights are reset, the player's weapon shader values are set, and the
/// enchantment effect and sound of the weapon are refreshed.
fn queued_action_draw_or_sheathe(
    e: &mut Engine,
    this: Ptr<Actor>,
    animation: Ptr,
    biped: u32,
    weapon_form: Ptr,
    weapon_item: Ptr,
    queued_action: i32,
) {
    let player = player(e);
    let process = process_of(e, this);
    let drawn = e
        .vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
        .u8();
    let want_drawn = u32::from(queued_action == 0);
    if u32::from(drawn) == want_drawn {
        return;
    }
    if e.call(ANIMATION_SLOT_STATE, &args![animation, 4u32]).i32() < 1 {
        return;
    }
    let mut current_animation = animation;
    let mut current_biped = biped;
    let mut count = if this == player { 2 } else { 1 };
    let process = process_of(e, this);
    e.vcall(
        process.addr(),
        PROCESS_SLOT_SET_WEAPON_DRAWN,
        &args![this, want_drawn],
    );
    while count != 0 {
        if this == player && count == 1 {
            current_animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).ptr();
            current_biped = e.call(PLAYER_GET_BIPED, &args![player, 1u32]).u32();
        }
        let process = process_of(e, this);
        if !e
            .vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
            .bool()
            && !weapon_form.is_null()
            && !current_animation.is_null()
            && e.call(ANIMATION_GUN_WOBBLE_SOURCE, &args![current_animation])
                .u32()
                != 0
        {
            let source = e
                .call(ANIMATION_GUN_WOBBLE_SOURCE, &args![current_animation])
                .u32();
            let weapon_type = e.call(WEAPON_TYPE, &args![weapon_form]).u32();
            let table_value = e
                .mem
                .u32(WEAPON_TYPE_ANIM_TABLE.wrapping_add(weapon_type.wrapping_mul(4)));
            e.call(CLEAR_GUN_WOBBLE, &args![table_value, source]);
        }
        let process = process_of(e, this);
        e.vcall(
            process.addr(),
            PROCESS_SLOT_FORCE_WEAPON_DRAWN,
            &args![want_drawn, current_biped, current_animation, this],
        );
        count -= 1;
    }
    e.call(ACTOR_SET_IRON_SIGHTS, &args![this, 0u32, 0u32, 0u32]);

    if this == player {
        let holder = e.mem.u32(this.addr() + 0xac);
        if queued_action == 0 {
            let mut first_value = 0u32;
            let mut second_value = 0u32;
            let process = process_of(e, this);
            let bone = e
                .vcall(process.addr(), PROCESS_SLOT_WEAPON_BONE, &args![biped])
                .u32();
            let node = e.call(BONE_NODE_LOOKUP, &args![bone, 0u32]).ptr::<()>();
            if !node.is_null() {
                let name = e.call(EXTRA_DATA_NAME, &args![]).u32();
                let extra = e
                    .call(NI_OBJECT_NET_GET_EXTRA_DATA, &args![node, name])
                    .ptr::<()>();
                if !extra.is_null() && fn_008978d0(e, extra) {
                    let first_key = fn_008978b0(e);
                    first_value = e.vcall(node.addr(), 0x9c, &args![first_key]).u32();
                    let second_key = fn_008978c0(e);
                    second_value = e.vcall(node.addr(), 0x9c, &args![second_key]).u32();
                }
            }
            let holder = e.mem.u32(this.addr() + 0xac);
            e.call(SET_SHADER_VALUE, &args![holder, 0u32, first_value]);
            let holder = e.mem.u32(this.addr() + 0xac);
            e.call(SET_SHADER_VALUE, &args![holder, 1u32, second_value]);
        } else {
            e.call(SET_SHADER_VALUE, &args![holder, 0u32, 0u32]);
            let holder = e.mem.u32(this.addr() + 0xac);
            e.call(SET_SHADER_VALUE, &args![holder, 1u32, 0u32]);
            let holder = e.mem.u32(this.addr() + 0xac);
            fn_008978f0(e, Ptr::new(holder), 1, 1);
            let holder = e.mem.u32(this.addr() + 0xac);
            fn_008978f0(e, Ptr::new(holder), 1, 1);
        }
    }

    let process = process_of(e, this);
    e.vcall(
        process.addr(),
        PROCESS_SLOT_SHADER_EFFECTS,
        &args![1u32, 0u32, 0u32],
    );
    let process = process_of(e, this);
    if e.vcall(process.addr(), PROCESS_SLOT_WEAPON_DRAWN, &args![])
        .bool()
    {
        let process = process_of(e, this);
        if e.vcall(
            process.addr(),
            PROCESS_SLOT_WEAPON_ENCHANTMENT_VISUALS,
            &args![],
        )
        .u32()
            != 0
        {
            let visuals = e
                .vcall(
                    process.addr(),
                    PROCESS_SLOT_WEAPON_ENCHANTMENT_VISUALS,
                    &args![],
                )
                .u32();
            let effect = e
                .call(
                    FIND_AND_CLEANUP_WEAPON_SHADER_HIT_EFFECT,
                    &args![PROCESS_LISTS, this, visuals],
                )
                .ptr::<()>();
            if !effect.is_null() {
                e.call(RESET_ALPHA_TIMER, &args![effect]);
            }
            if !weapon_form.is_null() {
                let enchanting = e.call(GET_FORM_ENCHANTING, &args![weapon_form]).u32();
                let mut enchantment = if enchanting != 0 {
                    enchanting + 0x18
                } else {
                    0
                };
                if enchantment == 0 {
                    let poison = e.call(ITEM_CHANGE_GET_POISON, &args![weapon_item]).u32();
                    enchantment = if poison != 0 { poison + 0x30 } else { 0 };
                }
                if enchantment != 0 {
                    let handle = e.mem.alloc(12);
                    e.call(
                        PLAY_SOUND_BY_EDITOR_NAME,
                        &args![
                            this,
                            handle,
                            ENCHANTED_EQUIP_SOUND_NAME,
                            0u32,
                            0x4000_0102u32,
                            1u32
                        ],
                    );
                    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
                    e.mem.free(handle);
                }
            }
        }
    }
}

/// The last part of `Actor::PickAnimations`: plays the chosen group `picked`
/// in the slot the group table names unless that very group already plays
/// there (a finished sequence). For the movement slot it first plays the
/// group for the actor's current movement direction when that changed; after
/// the play it sets the action, and replays the landing / movement variants
/// for groups `0xe3` and `0xf1..=0xf4`.
#[allow(clippy::too_many_arguments)]
fn play_picked_group(
    e: &mut Engine,
    this: Ptr<Actor>,
    animation: Ptr,
    picked: u16,
    group_type: u32,
    new_action: i32,
    flags: u16,
    selector_a: u32,
    selector_b: u32,
) {
    let slot = group_mode(e, group_type);
    let current = e
        .call(ANIMATION_GROUP_OF_SLOT, &args![animation, slot])
        .u16();
    if u32::from(current) == u32::from(picked)
        && e.call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
            .u32()
            != 0
    {
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
            .ptr::<()>();
        if e.call(SEQUENCE_GET_GENERIC_LOCATION, &args![sequence])
            .u32()
            != 0
        {
            return;
        }
    }
    if !e
        .call(ANIMATION_GROUP_LOADED, &args![animation, u32::from(picked)])
        .bool()
    {
        return;
    }
    let mut picked = picked;
    if group_mode(e, group_type) == 1
        && e.call(ANIMATION_GET_SEQUENCE, &args![animation, 1u32])
            .u32()
            != 0
    {
        let slot_one_group = e
            .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 1u32])
            .u16();
        let movement = e
            .call(ANIM_GROUP_GET_MOVE_OV2, &args![u32::from(slot_one_group)])
            .u32();
        if selector_a != movement {
            let special = in_special_state(e, this);
            let other = compose_group(e, animation, selector_a, selector_b, 0, special);
            let slot_zero_group = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 0u32])
                .u16();
            if other != slot_zero_group {
                e.call(
                    ANIMATION_PLAY_GROUP,
                    &args![
                        animation,
                        u32::from(other),
                        1u32,
                        0xffff_ffffu32,
                        0xffff_ffffu32
                    ],
                );
                e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(other), 1u32]);
            }
        }
    }
    e.call(
        ANIMATION_PLAY_GROUP,
        &args![
            animation,
            u32::from(picked),
            1u32,
            0xffff_ffffu32,
            0xffff_ffffu32
        ],
    );
    if new_action != -1
        && !e
            .call(ANIM_GROUP_TEST_4CC0, &args![u32::from(picked)])
            .bool()
    {
        let slot = group_mode(e, group_type);
        let sequence = e
            .call(ANIMATION_GET_SEQUENCE, &args![animation, slot])
            .ptr::<()>();
        set_anim_action(e, this, new_action as u32, sequence);
    }
    e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(picked), 1u32]);

    if group_type == 0xe3 {
        let mut variant = 0xe4;
        if flags & 0xf != 0 {
            if flags & 1 != 0 {
                variant = 0xec;
            } else if flags & 2 != 0 {
                variant = 0xed;
            } else if flags & 4 != 0 {
                variant = 0xee;
            } else if flags & 8 != 0 {
                variant = 0xef;
            }
        }
        let special = in_special_state(e, this);
        picked = compose_group(e, animation, selector_a, selector_b, variant, special);
        e.call(
            ANIMATION_PLAY_GROUP,
            &args![
                animation,
                u32::from(picked),
                0u32,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
        e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(picked), 0u32]);
    } else if (0xf1..=0xf4).contains(&(group_type as i32)) && flags & 0xf != 0 {
        let mut variant = 0xffu32;
        if flags & 0x200 != 0 {
            if flags & 1 != 0 {
                variant = 7;
            } else if flags & 2 != 0 {
                variant = 8;
            } else if flags & 4 != 0 {
                variant = 9;
            } else if flags & 8 != 0 {
                variant = 0xa;
            }
        } else if flags & 1 != 0 {
            variant = 3;
        } else if flags & 2 != 0 {
            variant = 4;
        } else if flags & 4 != 0 {
            variant = 5;
        } else if flags & 8 != 0 {
            variant = 6;
        }
        if variant != 0xff {
            let special = in_special_state(e, this);
            picked = compose_group(e, animation, selector_a, selector_b, variant, special);
            e.call(
                ANIMATION_PLAY_GROUP,
                &args![
                    animation,
                    u32::from(picked),
                    0u32,
                    0xffff_ffffu32,
                    0xffff_ffffu32
                ],
            );
            e.vcall(this.addr(), ACTOR_SLOT_4B0, &args![u32::from(picked), 0u32]);
        }
    }
}

/// `MobileObject::GetCurrentProcessType` (Xbox PDB): the process level of the
/// object (a signed number; `0` is the high process).
const MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `TaskQueueInterface::QueueActorKill` (Xbox PDB): thiscall on the object
/// `GET_TES` returns, taking the actor, the kill data and a flag.
const QUEUE_ACTOR_KILL: u32 = 0x0087_aff0;
/// `FollowerBarks::TriggerFollowerBark` (Xbox PDB), cdecl: the actor and the
/// bark number.
const TRIGGER_FOLLOWER_BARK: u32 = 0x008d_5cb0;
/// `CombatController::IsActoraCombatTarget` (Xbox PDB): thiscall on the
/// combat controller, taking the actor.
const COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET: u32 = 0x0097_fa10;
/// `Actor::GetFactionFightReaction` (Xbox PDB): the other actor and a pointer
/// to a byte the function writes.
const ACTOR_GET_FACTION_FIGHT_REACTION: u32 = 0x008b_87a0;
/// `Actor::Kill` (Xbox PDB): the attacker and the damage (`float`).
const ACTOR_KILL: u32 = 0x0089_d900;
/// `Actor::GetArmorBeingWorn` (Xbox PDB): thiscall taking a slot number.
const ACTOR_GET_ARMOR_BEING_WORN: u32 = 0x0089_1b90;
/// Test that returns whether the debug messages are enabled (`005b6f70`,
/// takes the actor).
const DEBUG_MESSAGES_ENABLED: u32 = 0x005b_6f70;
/// The cdecl `printf`-style function that prints a debug line (`00703c00`).
const PRINT_DEBUG_LINE: u32 = 0x0070_3c00;
/// `"%.20s blocks %.0f%% of %.20s's blow!"`.
const BLOCKS_BLOW_FORMAT: u32 = 0x0108_4b54;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;
/// `0.25` (`double`).
const QUARTER: u32 = 0x0102_90b0;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `Actor` virtual slot `0x360`: a bool predicate (`0047c850` returns 0 in
/// the base class).
const ACTOR_SLOT_360: u32 = 0x360;
/// `Actor` virtual slot `0x380`: the actor's reach as a `float` (`0088b850`
/// is `CombatFormulas::CalcWeaponReach` of a setting).
const ACTOR_SLOT_REACH: u32 = 0x380;
/// `Actor` virtual slot `0x3ac`: changes an actor value (`00881130`: the
/// value number, the amount, the source actor).
const ACTOR_SLOT_MODIFY_ACTOR_VALUE: u32 = 0x3ac;
/// `Actor` virtual slot `0x3cc` (`008929a0`): drops an item: form, extra
/// data list, count, position, rotation; returns the new reference.
const ACTOR_SLOT_DROP_OBJECT: u32 = 0x3cc;
/// Process virtual slot `0x534`: the item the actor has equipped in the
/// other hand (compared with the weapon entry's form).
const PROCESS_SLOT_534: u32 = 0x534;
/// `CombatFormulas::CalcWeaponReach` (Xbox PDB), cdecl: the weapon's reach
/// as a `float`; the result is in ST0.
const CALC_WEAPON_REACH: u32 = 0x0064_63e0;
/// Reach of a weapon form (`006447f0`, thiscall on the form; ST0).
const WEAPON_REACH: u32 = 0x0064_47f0;
/// `TESObjectREFR::GetScale` (Xbox PDB): ST0.
const REFR_GET_SCALE: u32 = 0x0056_7400;
/// `009a60e0`: cdecl, takes the actor and a `float` distance.
const ACTOR_REACH_WORKER: u32 = 0x009a_60e0;
/// The setting object the reach is multiplied by for a special idle of kind 4
/// (the address `SETTING_GET_VALUE_POINTER` is called on).
const REACH_MULTIPLIER_SETTING: u32 = 0x011c_e194;
/// `ProcessLists::PrintLists` (Xbox PDB, the engine map's name): thiscall on
/// the combat controller of the attacker with the victim and the amount.
const COMBAT_CONTROLLER_NOTIFY_DAMAGE: u32 = 0x008d_0600;
/// `0097f7f0`: thiscall on the combat controller `008a0330` returns, taking
/// the attacker and the amount.
const CONTROLLER_RECORD_DAMAGE: u32 = 0x0097_f7f0;
/// `0097f6d0`: thiscall on the combat controller, taking one flag.
const CONTROLLER_SET_FLAG: u32 = 0x0097_f6d0;
/// `Actor::GetEssential` (Xbox PDB).
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
/// `Actor::DifficultyLevelAdjustHealthModifier` (Xbox PDB): the amount and
/// the source actor; the result is in ST0.
const DIFFICULTY_ADJUST_HEALTH_MODIFIER: u32 = 0x0088_08a0;
/// `00579220`: thiscall on the reference, takes a `float` and a flag.
const REFR_FLOAT_SETTER_579220: u32 = 0x0057_9220;
/// The setting object whose value the fatigue is compared with.
const FATIGUE_SETTING: u32 = 0x011c_e118;
/// Process virtual slot `0x72c`: reacts to damage (attacker, amount).
const PROCESS_SLOT_72C: u32 = 0x72c;
/// Process virtual slot `0x580` (shader effects: 1, 0, 0).
const PROCESS_SLOT_580: u32 = 0x580;
/// `ExtraDataList::GetCanNotWear` (Xbox PDB).
const EXTRA_GET_CAN_NOT_WEAR: u32 = 0x0041_8b10;
/// Whether the form (`this` = form) has bit `0x8` of the byte at `+0x100`
/// (`00891b70`).
const FORM_FLAG_100_BIT_3: u32 = 0x0089_1b70;
/// Whether the form has bit `0x20` of the byte at `+0x100` (`0046e8c0`).
const FORM_FLAG_100_BIT_5: u32 = 0x0046_e8c0;
/// The position of a bone node: the address of its 3 floats (`0045bb80`,
/// `node + 0x8c`).
const NODE_WORLD_TRANSLATE: u32 = 0x0045_bb80;
/// `node + 0x68` (`00461130`): the rotation matrix of a node.
const NODE_ROTATION: u32 = 0x0046_1130;
/// `NiMatrix3::ToEulerAnglesXYZ` (Xbox PDB): thiscall on the matrix with
/// three pointers for the angles.
const NI_MATRIX3_TO_EULER_ANGLES_XYZ: u32 = 0x00a5_92c0;
/// `0056acb0` (cdecl): the dropper actor and the dropped reference.
const ACTOR_DROPPED_OBJECT_WORKER: u32 = 0x0056_acb0;
/// Form flag test (`00461580`): thiscall on the form with a mask; true when
/// `*(this + 4) & mask` is not zero.
const FORM_TEST_FLAGS: u32 = 0x0046_1580;

/// The actor's value owner embedded at `+0xa4` (virtual slot `4`: base value,
/// slot `0xc`: current value, both taking the actor value number).
const ACTOR_VALUE_OWNER_OFFSET: u32 = 0xa4;

/// Reads a `float` constant stored as a `double` at `address`.
fn double_constant(e: &Engine, address: u32) -> f64 {
    e.global::<f64>(address)
}

// Translated from 008985d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends the actor's movement, creates a package of type `0x24` and adds it
/// through the actor's add-package slot (`0x2f4`): with `1, 1` when the
/// actor's process type is at most 1, otherwise after setting the package's
/// second flag, with `0, 0`.
pub fn fn_008985d0(e: &mut Engine, this: Ptr<Actor>) {
    e.call(ACTOR_END_MOVEMENT, &args![this]);
    let package = e.call(PACKAGE_CREATE, &args![0x24u32]).ptr::<()>();
    e.call(PACKAGE_CALCULATE_PROCEDURE_TYPE, &args![package, 0u32]);
    let process_type = e
        .call(MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, &args![this])
        .i32();
    if process_type <= 1 {
        e.vcall(
            this.addr(),
            ACTOR_SLOT_ADD_PACKAGE,
            &args![package, 1u32, 1u32],
        );
    } else {
        e.call(PACKAGE_SET_FLAG_B, &args![package, 1u32]);
        e.vcall(
            this.addr(),
            ACTOR_SLOT_ADD_PACKAGE,
            &args![package, 0u32, 0u32],
        );
    }
}

// Translated from 00898650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a health loss to an actor that is not an essential-style
/// protected one (`this`'s slot `0x22c(0)` false): works out the health
/// ratio (current over base of actor value `0x10`), lets a follower
/// (teammate flag at `+0x18d`) bark when it is under a half (bark 4) or a
/// quarter (bark 5), turns the attacker into nobody when the player attacks
/// a creature that the player's faction reaction does not allow it to fight,
/// and, when the health is below one, kills the actor: directly
/// (`Actor::Kill`) or, when the task queue is used (`008c7aa0`), by queueing
/// the kill with the damage `float` in a 4-byte block and remembering the
/// attacker as the killer (`+0xc0`).
pub fn fn_00898650(e: &mut Engine, this: Ptr<Actor>, attacker: Ptr<Actor>, damage: f32) {
    let mut attacker = attacker;
    if e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return;
    }
    let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
    let current = e.vcall(owner, 0x0c, &args![0x10u32]).f32();
    let base = e.vcall(owner, 0x04, &args![0x10u32]).f32();
    let mut ratio = 1.0f32;
    if base != 0.0 {
        ratio = (current as f64 / base as f64) as f32;
    }
    if e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
        let quarter = double_constant(e, QUARTER);
        let half = double_constant(e, HALF);
        if (ratio as f64) < quarter {
            e.call(TRIGGER_FOLLOWER_BARK, &args![this, 5u32]);
        } else if (ratio as f64) < half {
            e.call(TRIGGER_FOLLOWER_BARK, &args![this, 4u32]);
        }
    }
    let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    let player_address = e.global::<u32>(PLAYER_CHARACTER);
    if attacker.addr() == player_address
        && controller != 0
        && !e
            .call(
                COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET,
                &args![controller, player_address],
            )
            .bool()
    {
        let reaction = e.with_stack(4, |e, out| {
            e.mem.set_u8(out.addr(), 0);
            e.call(
                ACTOR_GET_FACTION_FIGHT_REACTION,
                &args![this, player_address, out],
            )
            .i32()
        });
        if reaction == 2 {
            attacker = Ptr::NULL;
        }
    }
    let one = double_constant(e, ONE_DOUBLE);
    if (current as f64) < one {
        if !e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
            e.call(ACTOR_KILL, &args![this, attacker, damage]);
        } else {
            let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
            e.mem.set_f32(block, damage);
            e.set(this, Actor::pMyKiller, attacker.cast());
            let tes = e.call(GET_TES, &args![]).u32();
            e.call(QUEUE_ACTOR_KILL, &args![tes, this, block, 0u32]);
        }
    }
}

// Translated from 00898fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x10` of the dword at `+0x12c` is set.
pub fn fn_00898fd0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x12c) & 0x10 != 0
}

// Translated from 00898ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual slot `0x18` of the object embedded at `+0x30` of the
/// actor's base form (`004181e0`) and returns its flag.
pub fn fn_00898ff0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
    e.vcall(base + 0x30, 0x18, &args![]).bool()
}

// Translated from 00899030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to virtual slot `0x4f0` of the object
/// `MiddleHighProcess::GetSavedAcquireObject` returns for the actor, and
/// returns its result.
pub fn fn_00899030(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    e.vcall(object, 0x4f0, &args![]).u32()
}

// Translated from 00899060 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the actor has an acquire object (`008d8520`), calls its virtual
/// slot `0x4f4` with `flag`.
pub fn fn_00899060(e: &mut Engine, this: Ptr<Actor>, flag: u8) {
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(object, 0x4f4, &args![flag]);
    }
}

// Translated from 008990a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::HandleBlockedAttack` (Xbox PDB): when debug messages are on,
/// prints `"<other> blocks <percent>% of <this>'s blow!"`; `percent` is
/// `blocked * 100`. Two of the four stack words are never read.
pub fn actor_handle_blocked_attack(
    e: &mut Engine,
    this: Ptr<Actor>,
    _unused_0: u32,
    blocked: f32,
    other: Ptr<Actor>,
    _unused_3: u32,
) {
    if e.call(DEBUG_MESSAGES_ENABLED, &args![]).bool() {
        let this_name = e.call(REFR_GET_NAME, &args![this]).u32();
        let percent = blocked as f64 * double_constant(e, HUNDRED);
        let other_name = e.call(REFR_GET_NAME, &args![other]).u32();
        e.call(
            PRINT_DEBUG_LINE,
            &args![BLOCKS_BLOW_FORMAT, other_name, percent, this_name],
        );
    }
}

// Translated from 008990f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks for the reference the actor can hit: works out the actor's reach (the
/// reach of the weapon it holds, `CalcWeaponReach` of the weapon form's reach,
/// or with no weapon its own, slot `0x380`; times the actor's scale; times
/// the reach setting when slot `0x360` is true and the special idle state
/// object's form is of kind 4) and returns what `009a60e0(actor, reach)`
/// finds.
pub fn fn_008990f0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mut weapon = 0;
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        weapon = e.vcall(object, PROCESS_SLOT_CURRENT_WEAPON, &args![]).u32();
    }
    let mut weapon_form = 0;
    if weapon != 0 {
        weapon_form = e.call(ENTRY_FORM, &args![weapon]).u32();
    }
    let reach = if weapon_form != 0 {
        let raw = e.call(WEAPON_REACH, &args![weapon_form]).f32();
        e.call(CALC_WEAPON_REACH, &args![raw]).f32()
    } else {
        e.vcall(this.addr(), ACTOR_SLOT_REACH, &args![]).f32()
    };
    let scale = e.call(REFR_GET_SCALE, &args![this]).f32();
    let mut result = (scale as f64 * reach as f64) as f32;
    if e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        && e.call(ENTRY_FORM, &args![SPECIAL_IDLE_STATE_OBJECT]).u32() == 4
    {
        let setting = e
            .call(SETTING_GET_VALUE_POINTER, &args![REACH_MULTIPLIER_SETTING])
            .u32();
        let multiplier = e.mem.f32(setting);
        result = (result as f64 * multiplier as f64) as f32;
    }
    e.call(ACTOR_REACH_WORKER, &args![this, result]).u32()
}

// Translated from 0089d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag bit `0x8000000` is set (the flag test `00461580`:
/// `*(this + 4) & mask != 0`).
pub fn fn_0089d5e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_TEST_FLAGS, &args![this, 0x0800_0000u32]).bool()
}

// Translated from 0089d600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the embedded object at `+0x450`.
pub fn fn_0089d600(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x450)
}

// Translated from 0089d620 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `+0x2e4`.
pub fn fn_0089d620(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2e4)
}

// Translated from 0089d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three dwords at `+0xf4`, `+0xf8` and `+0xfc`.
pub fn fn_0089d640(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.mem.set_u32(this.addr() + 0xf4, first);
    e.mem.set_u32(this.addr() + 0xf8, second);
    e.mem.set_u32(this.addr() + 0xfc, third);
}

// Translated from 0089d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x28d`.
pub fn fn_0089d670(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x28d)
}

// Translated from 0089d690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x28e`.
pub fn fn_0089d690(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x28e)
}

// Translated from 0089d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `+0x29c`.
pub fn fn_0089d6b0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x29c)
}

// Translated from 0089d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `+0x298`.
pub fn fn_0089d6d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x298)
}

// Translated from 0089d6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies damage to an actor that is not protected (the object embedded at
/// `+0x94`, slot `0x10`, is false). `health_loss` goes through
/// `DifficultyLevelAdjustHealthModifier`; when the adjusted loss is positive
/// the attacker's combat controller (slot `0x428`) and this actor's process
/// object are told, a non-player actor's acquire object reacts (slot
/// `0x72c`) unless it is essential, actor value `0x10` (health) is lowered
/// by the loss through slot `0x3ac` and `00579220` runs. A positive
/// `fatigue_loss` lowers actor value `0x16` the same way when the fatigue
/// has reached the fatigue setting. Returns slot `0x22c(0)`'s result, or the
/// protected test's result with its low byte cleared.
pub fn fn_0089d6f0(
    e: &mut Engine,
    this: Ptr<Actor>,
    health_loss: f32,
    fatigue_loss: f32,
    attacker: Ptr<Actor>,
) -> u32 {
    let protected = e.vcall(this.addr() + 0x94, 0x10, &args![]).u32();
    if protected as u8 != 0 {
        return protected & 0xffff_ff00;
    }
    let adjusted = e
        .call(
            DIFFICULTY_ADJUST_HEALTH_MODIFIER,
            &args![this, -health_loss, attacker],
        )
        .f32();
    let loss = -adjusted;
    if loss as f64 > double_constant(e, ZERO_DOUBLE) {
        if !attacker.is_null() {
            let controller = e.vcall(attacker.addr(), ACTOR_SLOT_428, &args![]).u32();
            if controller != 0 {
                let controller = e.vcall(attacker.addr(), ACTOR_SLOT_428, &args![]).u32();
                e.call(
                    COMBAT_CONTROLLER_NOTIFY_DAMAGE,
                    &args![controller, this, loss],
                );
            }
        }
        if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
            let controller = fn_008a0330(e, this);
            e.call(CONTROLLER_RECORD_DAMAGE, &args![controller, attacker, loss]);
        }
        if !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
            && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0
            && !e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
        {
            let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            e.vcall(object, PROCESS_SLOT_72C, &args![attacker, loss]);
        }
        e.vcall(
            this.addr(),
            ACTOR_SLOT_MODIFY_ACTOR_VALUE,
            &args![0x10u32, -loss, attacker],
        );
        e.call(REFR_FLOAT_SETTER_579220, &args![this, health_loss, 0u32]);
    }
    if fatigue_loss as f64 > double_constant(e, ZERO_DOUBLE) {
        let fatigue = actor_get_fatigue(e, this) as f64;
        let setting = e
            .call(SETTING_GET_VALUE_POINTER, &args![FATIGUE_SETTING])
            .u32();
        let threshold = e.mem.f32(setting) as f64;
        if threshold <= fatigue {
            e.vcall(
                this.addr(),
                ACTOR_SLOT_MODIFY_ACTOR_VALUE,
                &args![0x16u32, -fatigue_loss, attacker],
            );
        }
    }
    e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).u32()
}

// Translated from 0089d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetRandomWornArmor` (Xbox PDB): with `flag` set, the armor worn in
/// slot 0 or, when there is none, in slot 1; otherwise the armor worn in
/// slot 2.
pub fn actor_get_random_worn_armor(e: &mut Engine, this: Ptr<Actor>, flag: u8) -> u32 {
    if flag == 0 {
        e.call(ACTOR_GET_ARMOR_BEING_WORN, &args![this, 2u32]).u32()
    } else {
        let armor = e.call(ACTOR_GET_ARMOR_BEING_WORN, &args![this, 0u32]).u32();
        if armor == 0 {
            e.call(ACTOR_GET_ARMOR_BEING_WORN, &args![this, 1u32]).u32()
        } else {
            armor
        }
    }
}

// Translated from 0089f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `+0x638`.
pub fn fn_0089f4e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x638)
}

// Translated from 0089f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xe2d`.
pub fn fn_0089f500(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xe2d)
}

// Translated from 0089f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `+0x18a`.
pub fn fn_0089f520(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x18a, value);
}

// Translated from 0089f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x18a`.
pub fn fn_0089f540(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x18a)
}

// Translated from 0089f560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds 1 to the dword at `+0x3c`.
pub fn fn_0089f560(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0x3c);
    e.mem.set_u32(this.addr() + 0x3c, count.wrapping_add(1));
}

// Translated from 0089f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops the weapon the actor has drawn when it is not the item equipped in
/// the other hand (process slot `0x534`), is not flagged (form bytes at
/// `+0x100`: bits `0x8`, `0x20`) and cannot be worn (its extra data list says
/// `GetCanNotWear` false): the drop (`Actor` slot `0x3cc`) happens at the
/// weapon bone's position and angles (process slot `0x190` of the
/// `Actor` slot `0x1e8` result) when the node exists, then the dropped
/// reference is passed to `0056acb0` and, for an enchanted form, process
/// slot `0x580` runs with `1, 0, 0`. Ends by telling the actor's
/// controller of its fight (`0097f6d0(0)`) when it has one (slot `0x428`).
pub fn fn_0089f580(e: &mut Engine, this: Ptr<Actor>) {
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let weapon = e.vcall(object, PROCESS_SLOT_CURRENT_WEAPON, &args![]).u32();
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let other_hand = e.vcall(object, PROCESS_SLOT_534, &args![]).u32();
    if weapon != 0 {
        let form = e.call(ENTRY_FORM, &args![weapon]).u32();
        if e.call(FORM_FLAG_100_BIT_3, &args![form]).bool() {
            return;
        }
        let form = e.call(ENTRY_FORM, &args![weapon]).u32();
        if e.call(FORM_FLAG_100_BIT_5, &args![form]).bool() {
            return;
        }
        if !e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).bool() {
            return;
        }
        let lists = e.call(ENTRY_FIRST_EXTRA_LIST, &args![weapon]).u32();
        let lists = e.call(LIST_ITEM_SLOT, &args![lists]).u32();
        let first = e.mem.u32(lists);
        if e.call(EXTRA_GET_CAN_NOT_WEAR, &args![first]).bool() {
            return;
        }
        if e.call(ENTRY_FORM, &args![weapon]).u32() == other_hand {
            return;
        }
        let mut enchanted = false;
        let form = e.call(ENTRY_FORM, &args![weapon]).u32();
        if e.call(GET_FORM_ENCHANTING, &args![form]).u32() != 0 {
            enchanted = true;
        }
        let bone_slot = e.vcall(this.addr(), ACTOR_SLOT_1E8, &args![]).u32();
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let node = e
            .vcall(process, PROCESS_SLOT_WEAPON_BONE, &args![bone_slot])
            .u32();
        if node != 0 {
            let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
            // The exe's frame (`SUB ESP,0x3c` under `EBP`): the position at +0x1c (`EBP-0x20`), the
            // three angles at +0x10 (`EBP-0x2c` ... `EBP-0x24`).
            e.with_stack(0x3c, |e, frame| {
                let position = frame.addr() + 0x1c;
                for word in 0..3 {
                    let value = e.mem.u32(translate + 4 * word);
                    e.mem.set_u32(position + 4 * word, value);
                }
                let angles = frame.addr() + 0x10;
                let rotation = e.call(NODE_ROTATION, &args![node]).u32();
                e.call(
                    NI_MATRIX3_TO_EULER_ANGLES_XYZ,
                    &args![rotation, angles, angles + 4, angles + 8],
                );
                let lists = e.call(ENTRY_FIRST_EXTRA_LIST, &args![weapon]).u32();
                let first = e.mem.u32(lists);
                let form = e.call(ENTRY_FORM, &args![weapon]).u32();
                let dropped = e
                    .vcall(
                        this.addr(),
                        ACTOR_SLOT_DROP_OBJECT,
                        &args![form, first, 1u32, position, angles],
                    )
                    .u32();
                e.call(ACTOR_DROPPED_OBJECT_WORKER, &args![this, dropped]);
            });
            if enchanted {
                let process = e.get(this, Actor::pCurrentProcess).addr();
                e.vcall(process, PROCESS_SLOT_580, &args![1u32, 0u32, 0u32]);
            }
        }
        if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
            let controller = fn_008a0330(e, this);
            e.call(CONTROLLER_SET_FLAG, &args![controller, 0u32]);
        }
    }
}

// Translated from 008a0250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0xc4` to 1 (`bMurderAlarm` of an `Actor`, Xbox PDB; the
/// combat starter also calls it on a combat controller).
pub fn fn_008a0250(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0xc4, 1);
}

// Translated from 008a0330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual slot `0x22c` of the actor's process (`pCurrentProcess`) and
/// returns its result: the actor's combat controller as the process holds it
/// (`Actor::GetCombatController`, `008a02d0`, checks its type and the combat
/// flag first).
pub fn fn_008a0330(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    e.vcall(process, PROCESS_SLOT_22C, &args![]).u32()
}

/// The `CombatManager` singleton pointer.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// The `TES` singleton pointer (`GET_TES` returns the same object).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The `ModelLoader` singleton pointer.
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The `FaderManager` singleton pointer.
const FADER_MANAGER: u32 = 0x011d_8804;
/// The global object `Actor::Resurrect` locks around its work (`0040fbf0`
/// locks it, `0040fba0` unlocks it).
const RESURRECT_LOCK: u32 = 0x011f_11a0;
/// The global object whose do-nothing method `00483710` the combat start
/// calls around the combat controller's start (`0x11df71c`).
const COMBAT_START_GUARD: u32 = 0x011d_f71c;
/// The pointer read as the `this` of `0047c850` (`TESActorBaseData`'s
/// do-nothing predicate that returns 0).
const BASE_DATA_OBJECT: u32 = 0x011d_e45c;
/// `0047c850`: predicate on `BASE_DATA_OBJECT` (returns 0 in the exe).
const BASE_DATA_PREDICATE: u32 = 0x0047_c850;
/// `PathManager::QInstance` (Xbox PDB): the singleton, no arguments.
const PATH_MANAGER_Q_INSTANCE: u32 = 0x0047_d0b0;
/// `PathManager::ClearInventoryPointersFromCurrentRequests` (Xbox PDB):
/// thiscall on the manager, taking the inventory changes.
const PATH_MANAGER_CLEAR_INVENTORY_POINTERS: u32 = 0x006e_bd10;
/// `00574920`: thiscall on the actor, taking a flag.
const ACTOR_UPDATE_INVENTORY_WORKER: u32 = 0x0057_4920;
/// `TESNPC::InitDefaultWorn`-style worker (`006047c0`, the 5-word form of
/// the creature one): the actor, `1`, a flag, `0`, `1`.
const NPC_INIT_DEFAULT_WORN: u32 = 0x0060_47c0;
/// `TESCreature::InitDefaultWorn` (Xbox PDB): the actor, `1`, a flag, `1`.
const CREATURE_INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// Whether a package (`this`) is one of the kinds `00441b00` accepts.
const PACKAGE_KIND_TEST: u32 = 0x0044_1b00;
/// `Actor` virtual slot `0x1cc`.
const ACTOR_SLOT_1CC: u32 = 0x1cc;
/// Process virtual slot `0x468`.
const PROCESS_SLOT_468: u32 = 0x468;
/// `Actor` virtual slot `0x418`.
const ACTOR_SLOT_418: u32 = 0x418;
/// `Actor` virtual slot `0x434`: `Actor::StopCombat` (Xbox PDB), takes the
/// target.
const ACTOR_SLOT_STOP_COMBAT: u32 = 0x434;
/// Process virtual slot `0x4d4` (a furniture marker entry, whose byte at
/// `+0xe` is compared with `0x14`).
const PROCESS_SLOT_4D4: u32 = 0x4d4;
/// Process virtual slot `0x52c`.
const PROCESS_SLOT_52C: u32 = 0x52c;
/// Process virtual slot `0x208` (takes the target and the reference
/// `0x52c` gave; the result is a `float`).
const PROCESS_SLOT_208: u32 = 0x208;
/// Process virtual slot `0x2c8` (takes a `float`).
const PROCESS_SLOT_2C8: u32 = 0x2c8;
/// `TESObjectREFR::GetDistanceFromReference` (Xbox PDB): the other
/// reference and two flags; the distance is in ST0.
const REFR_GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// `Actor` `0087f9c0` (a `float` read).
const ACTOR_GETTER_87F9C0: u32 = 0x0087_f9c0;
/// `PlayerCharacter::ChangePerceivedActorHostileStatus` (Xbox PDB): thiscall
/// on the player, taking the actor and a flag.
const PLAYER_CHANGE_PERCEIVED_HOSTILE_STATUS: u32 = 0x0096_7220;
/// `00703350`: returns the reference the crosshair targets (compared with
/// the actor).
const CROSSHAIR_TARGET: u32 = 0x0070_3350;
/// `Interface::SetCrosshairTargetType` (Xbox PDB), cdecl.
const SET_CROSSHAIR_TARGET_TYPE: u32 = 0x0070_3860;
/// `Actor::EndInterruptPackage` (Xbox PDB): takes a flag.
const ACTOR_END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
/// `CombatManager::AddGroupMember` (Xbox PDB): thiscall on the manager,
/// taking the actor and the group.
const COMBAT_MANAGER_ADD_GROUP_MEMBER: u32 = 0x0099_22b0;
/// `CombatManager::AddCombatant` (Xbox PDB): the actor, the target, the
/// value and a flag.
const COMBAT_MANAGER_ADD_COMBATANT: u32 = 0x0099_2110;
/// `CombatManager::AddPretendCombatant` (Xbox PDB): the actor and the
/// pretend target.
const COMBAT_MANAGER_ADD_PRETEND_COMBATANT: u32 = 0x0099_23d0;
/// `CombatController::AddTarget` (Xbox PDB): the target, the value, a flag
/// and two `float`s.
const COMBAT_CONTROLLER_ADD_TARGET: u32 = 0x0097_f930;
/// The target of a combat controller: `*(controller + 0xc0)` (`004030b0`).
const COMBAT_CONTROLLER_TARGET: u32 = 0x0040_30b0;
/// The value `Interface` calls `Error`: a do-nothing cdecl function
/// (`0040fbe0`) that takes the actor.
const ERROR_NO_OP: u32 = 0x0040_fbe0;
/// `FaderManager::GetFaderAlpha` (Xbox PDB): thiscall on the manager,
/// taking a fader number; the result is in ST0.
const FADER_MANAGER_GET_FADER_ALPHA: u32 = 0x0070_14e0;
/// Test on an actor (`00576d30`).
const ACTOR_TEST_576D30: u32 = 0x0057_6d30;
/// Test on a combat controller (`00981450`).
const CONTROLLER_TEST_981450: u32 = 0x0098_1450;
/// Test on the player (`005a03f0`, takes a flag).
const PLAYER_TEST_5A03F0: u32 = 0x005a_03f0;
/// Process-type test on the actor (`00437bf0`: process level equal to 5).
const ACTOR_PROCESS_LEVEL_IS_5: u32 = 0x0043_7bf0;
/// Process-type test on the actor (`00437bd0`: process level equal to 3).
const ACTOR_PROCESS_LEVEL_IS_3: u32 = 0x0043_7bd0;
/// Test on an actor (`00440da0`).
const ACTOR_TEST_440DA0: u32 = 0x0044_0da0;
/// Test on an actor (`00440d80`).
const ACTOR_TEST_440D80: u32 = 0x0044_0d80;
/// `Actor` virtual slot `0x2e8`.
const ACTOR_SLOT_2E8: u32 = 0x2e8;
/// How many combat entries the player's combat list holds for the actor
/// (`008a8230`: the player and the actor).
const PLAYER_COMBAT_LIST_COUNT: u32 = 0x008a_8230;
/// `PlayerCharacter::AddActorToPlayerCombatList` (Xbox PDB): thiscall on the
/// player, taking the actor.
const PLAYER_ADD_ACTOR_TO_COMBAT_LIST: u32 = 0x0093_a690;
/// Test on the combat controller (`009818b0`).
const CONTROLLER_TEST_9818B0: u32 = 0x0098_18b0;
/// Process virtual slot `0x64c`.
const PROCESS_SLOT_64C: u32 = 0x64c;
/// Process virtual slot `0x630` (takes the target).
const PROCESS_SLOT_630: u32 = 0x630;
/// Combat controller start for the high process level (`0097da50`).
const CONTROLLER_START_HIGH: u32 = 0x0097_da50;
/// Combat controller start for the other process levels (`0097e4b0`).
const CONTROLLER_START_OTHER: u32 = 0x0097_e4b0;
/// `00972aa0`: thiscall on the process lists, taking the actor.
const PROCESS_LISTS_WORKER_972AA0: u32 = 0x0097_2aa0;
/// `ExtraDataList::RemoveSavedHavokData` (Xbox PDB).
const EXTRA_REMOVE_SAVED_HAVOK_DATA: u32 = 0x0042_2c20;
/// `ExtraDataList::RemoveDismembermentExtra` (Xbox PDB).
const EXTRA_REMOVE_DISMEMBERMENT_EXTRA: u32 = 0x0042_e8e0;
/// `ExtraDataList::RemovePlayerCrimeListExtra` (Xbox PDB): takes the actor.
const EXTRA_REMOVE_PLAYER_CRIME_LIST_EXTRA: u32 = 0x0041_cf30;
/// `MiddleHighProcess::GetLastIdlePlayed` (Xbox PDB).
const PROCESS_GET_LAST_IDLE_PLAYED: u32 = 0x005a_29b0;
/// `Actor::SetLifeState` (Xbox PDB): takes the state number.
const ACTOR_SET_LIFE_STATE: u32 = 0x008a_1800;
/// `00884f80`: thiscall on the actor.
const ACTOR_WORKER_884F80: u32 = 0x0088_4f80;
/// `NiPointer` constructor (`00633c90`): stores the pointer and adds a
/// reference when it is not null.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `bhkWorld::SetMotion` (Xbox PDB), cdecl: the world object (from `Actor`
/// slot `0x1d0`) and four flags.
const BHK_WORLD_SET_MOTION: u32 = 0x00c6_a350;
/// `Actor` virtual slot `0x1d0` (no arguments).
const ACTOR_SLOT_1D0: u32 = 0x1d0;
/// `Actor` virtual slot `0xc4` (`MobileObject::SetDelete`, Xbox PDB).
const ACTOR_SLOT_SET_DELETE: u32 = 0xc4;
/// `Actor` virtual slot `0x4c` (takes a flag mask).
const ACTOR_SLOT_4C: u32 = 0x4c;
/// `Actor` virtual slot `0x1c4` (`MobileObject::InitHavok`, Xbox PDB).
const ACTOR_SLOT_INIT_HAVOK: u32 = 0x1c4;
/// Process virtual slot `0x290`.
const PROCESS_SLOT_290: u32 = 0x290;
/// Process virtual slot `0x410`.
const PROCESS_SLOT_410: u32 = 0x410;
/// `ModifierList::DeleteAllModifiers` (Xbox PDB).
const MODIFIER_LIST_DELETE_ALL: u32 = 0x0093_70b0;
/// `ProcessLists::RemoveReference` (Xbox PDB): thiscall on the process
/// lists, taking the actor and the process level.
const PROCESS_LISTS_REMOVE_REFERENCE: u32 = 0x0096_d470;
/// `ProcessLists::AddReference` (Xbox PDB): the actor, the level and three
/// zeros.
const PROCESS_LISTS_ADD_REFERENCE: u32 = 0x0096_d450;
/// Constructor of the 0xb4-byte process object `Resurrect` installs
/// (`00906dc0`).
const NEW_PROCESS_CONSTRUCT: u32 = 0x0090_6dc0;
/// `Actor` virtual slot `0x208` (`fn_0089fb80`).
const ACTOR_SLOT_208: u32 = 0x208;
/// `Actor` virtual slots `0x240`, `0x248` and `0x24c`: the process-level
/// switches (to the high, middle high and middle low level; the names are
/// those the code's `GetDesiredProcessLevel` switch uses, not confirmed).
const ACTOR_SLOT_240: u32 = 0x240;
const ACTOR_SLOT_248: u32 = 0x248;
const ACTOR_SLOT_24C: u32 = 0x24c;
/// `TES::IsCellLoaded` (Xbox PDB): thiscall on the TES, taking the cell and
/// a flag.
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `TES::GetCellPriority` (Xbox PDB): the cell and a flag (RET 8).
const TES_GET_CELL_PRIORITY: u32 = 0x0045_8be0;
/// `ModelLoader::QueueReference` (Xbox PDB): thiscall on the loader, taking
/// the reference, the priority and a flag (RET 0xc).
const MODEL_LOADER_QUEUE_REFERENCE: u32 = 0x0044_4850;
/// Test on a cell (`00450ff0`).
const CELL_TEST_450FF0: u32 = 0x0045_0ff0;
/// `MobileObject::GetDesiredProcessLevel` (Xbox PDB).
const MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL: u32 = 0x0093_34b0;
/// Lock and unlock of the global at `RESURRECT_LOCK`.
const LOCK_ENTER: u32 = 0x0040_fbf0;
const LOCK_LEAVE: u32 = 0x0040_fba0;
/// `008a1a40`: thiscall on the actor, taking a flag (the function after
/// `Actor::SetLifeState`; its translation is outside this session's list).
const ACTOR_FN_008A1A40: u32 = 0x008a_1a40;
/// Process virtual slot `0x54` (takes the actor and a kind number).
const PROCESS_SLOT_54: u32 = 0x54;
/// `0097efe0`: thiscall on a combat controller, taking two words.
const CONTROLLER_FORWARD_97EFE0: u32 = 0x0097_efe0;
/// `00483710`: a do-nothing method (it only stores `this`) the combat start
/// calls on `COMBAT_START_GUARD` before and after starting the controller.
const COMBAT_START_GUARD_NO_OP: u32 = 0x0048_3710;

// Translated from 0089f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Resurrect` (Xbox PDB), `reset_process`, `queue_load`, `heal`:
/// brings an actor back to life. Clears the delete state (slot `0xc4`),
/// the saved havok and dismemberment extras, the idle played and the
/// player crime extra, removes the actor from the process lists
/// (`00972aa0`), then:
/// - with `heal` set and a collision object (slot `0x1d0`): sets the life
///   state to 0, heals actor value `0x10` to its base through slot `0x3ac`,
///   re-initializes havok and the collision motion and tells the process
///   (slots `0x290`, `0x410`);
/// - otherwise clears the modifier lists, a new `0xb4`-byte process object
///   replaces the actor's one when it is not the player, the life state is
///   set to 0 and the actor is put back in the process level it wants
///   (through the cell load queue when `queue_load` is set).
///
/// The compiler's exception-unwinding frame is not translated.
pub fn actor_resurrect(
    e: &mut Engine,
    this: Ptr<Actor>,
    reset_process: u8,
    queue_load: u8,
    heal: u8,
) {
    e.vcall(this.addr(), ACTOR_SLOT_SET_DELETE, &args![0u32]);
    let extra = extra_data_list_of(e, this.cast());
    e.call(EXTRA_REMOVE_SAVED_HAVOK_DATA, &args![extra]);
    let extra = extra_data_list_of(e, this.cast());
    e.call(EXTRA_REMOVE_DISMEMBERMENT_EXTRA, &args![extra]);
    e.vcall(this.addr(), ACTOR_SLOT_4C, &args![0x0002_0000u32]);
    if e.call(PROCESS_GET_LAST_IDLE_PLAYED, &args![this]).u32() != 0 {
        e.call(ACTOR_FN_008A1A40, &args![this, 0u32]);
    }
    e.mem.set_u8(this.addr() + 0x174, 0);
    e.call(ACTOR_WORKER_884F80, &args![this]);
    e.call(PROCESS_LISTS_WORKER_972AA0, &args![PROCESS_LISTS, this]);
    let extra = extra_data_list_of(e, this.cast());
    e.call(EXTRA_REMOVE_PLAYER_CRIME_LIST_EXTRA, &args![extra, this]);

    if heal != 0 && e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32() != 0 {
        e.call(ACTOR_SET_LIFE_STATE, &args![this, 0u32]);
        let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
        let current = e.vcall(owner, 0x0c, &args![0x10u32]).f32();
        let base = e.vcall(owner, 0x00, &args![0x10u32]).i32();
        let missing = ((base as f32) as f64 - current as f64) as f32;
        e.vcall(
            this.addr(),
            ACTOR_SLOT_MODIFY_ACTOR_VALUE,
            &args![0x10u32, missing, 0u32],
        );
        let process = e.get(this, Actor::pCurrentProcess).addr();
        // A `NiPointer` temporary on the stack, built from null and passed
        // by value.
        e.with_stack(4, |e, pointer| {
            e.call(NI_POINTER_CONSTRUCT, &args![pointer, 0u32]);
            let value = e.mem.u32(pointer.addr());
            e.vcall(process, PROCESS_SLOT_290, &args![value]);
        });
        e.vcall(this.addr(), ACTOR_SLOT_INIT_HAVOK, &args![]);
        let world = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
        e.call(BHK_WORLD_SET_MOTION, &args![world, 1u32, 1u32, 0u32, 1u32]);
        if process != 0 {
            e.vcall(process, PROCESS_SLOT_410, &args![3u32]);
        }
        return;
    }

    e.vcall(this.addr(), ACTOR_SLOT_4C, &args![4u32]);
    e.call(MODIFIER_LIST_DELETE_ALL, &args![this.byte_add(0xd0)]);
    e.call(MODIFIER_LIST_DELETE_ALL, &args![this.byte_add(0xe0)]);
    e.mem.set_u8(this.addr() + 0x118, 0);
    e.call(LOCK_ENTER, &args![RESURRECT_LOCK, 0u32]);
    let player_address = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() != player_address {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        if process != 0 {
            let level = e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32();
            e.call(
                PROCESS_LISTS_REMOVE_REFERENCE,
                &args![PROCESS_LISTS, this, level],
            );
        }
        let old_process = e.get(this, Actor::pCurrentProcess).addr();
        if old_process != 0 {
            e.vcall(old_process, 0, &args![1u32]);
        }
        let block = e.call(OPERATOR_NEW, &args![0xb4u32]).u32();
        let new_process = if block != 0 {
            e.call(NEW_PROCESS_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(this.addr() + 0x68, new_process);
    }
    if reset_process != 0 {
        e.vcall(this.addr(), ACTOR_SLOT_208, &args![0u32]);
    }
    e.call(ACTOR_SET_LIFE_STATE, &args![this, 0u32]);
    if this.addr() != player_address {
        let cell = e.call(REFR_PARENT_CELL, &args![this]).u32();
        if cell != 0 {
            let tes = e.global::<u32>(TES_SINGLETON);
            if e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() && queue_load != 0 {
                // 0089fa9f: `00458be0` takes two words (RET 8); the third pushed 0 is
                // the last word of `00444850` (RET 0xc).
                let priority = e.call(TES_GET_CELL_PRIORITY, &args![tes, cell, 0u32]).u32();
                let loader = e.global::<u32>(MODEL_LOADER);
                e.call(
                    MODEL_LOADER_QUEUE_REFERENCE,
                    &args![loader, this, priority, 0u32],
                );
                if e.call(CELL_TEST_450FF0, &args![cell]).bool() {
                    e.vcall(this.addr(), ACTOR_SLOT_240, &args![]);
                } else {
                    e.vcall(this.addr(), ACTOR_SLOT_24C, &args![]);
                }
                e.call(LOCK_LEAVE, &args![RESURRECT_LOCK]);
                return;
            }
        }
        match e
            .call(MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL, &args![this])
            .u32()
        {
            0 => {
                e.vcall(this.addr(), ACTOR_SLOT_240, &args![]);
            }
            1 => {
                e.vcall(this.addr(), ACTOR_SLOT_24C, &args![]);
            }
            2 => {
                e.vcall(this.addr(), ACTOR_SLOT_248, &args![]);
            }
            3 => {
                e.call(
                    PROCESS_LISTS_ADD_REFERENCE,
                    &args![PROCESS_LISTS, this, 3u32, 0u32, 0u32, 0u32],
                );
            }
            _ => {}
        }
    }
    e.call(LOCK_LEAVE, &args![RESURRECT_LOCK]);
}

// Translated from 0089fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor` virtual slot `0x208`: re-initializes the actor's worn items.
/// Unless the base data predicate (`0047c850`) is set, asks the acquire
/// object (when its field `+0x28` is at most 1; otherwise the actor itself
/// through slot `0x1cc`) to refresh, clears the path manager's pointers into
/// the actor's inventory and runs `00574920`; then, for an actor with a
/// collision object (slot `0x1d0`), runs the default-worn setup of its base
/// form: the NPC one (form type `0x2a`) or the creature one (type `0x2b`),
/// with a flag that is cleared while the current package is of the kind
/// `00441b00` accepts.
pub fn fn_0089fb80(e: &mut Engine, this: Ptr<Actor>, flag: u8) {
    let base_data = e.global::<u32>(BASE_DATA_OBJECT);
    if !e.call(BASE_DATA_PREDICATE, &args![base_data]).bool() {
        let mut object = 0;
        if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            if e.call(ACQUIRE_OBJECT_FIELD_28, &args![acquire]).i32() <= 1 {
                object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            }
        }
        if object != 0 {
            e.vcall(object, PROCESS_SLOT_468, &args![1u32]);
        } else {
            e.vcall(this.addr(), ACTOR_SLOT_1CC, &args![0u32, 1u32]);
        }
    }
    let extra = extra_data_list_of(e, this.cast());
    let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).u32();
    let manager = e.call(PATH_MANAGER_Q_INSTANCE, &args![]).u32();
    e.call(
        PATH_MANAGER_CLEAR_INVENTORY_POINTERS,
        &args![manager, changes],
    );
    e.call(ACTOR_UPDATE_INVENTORY_WORKER, &args![this, flag]);
    if e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32() != 0 {
        let form = base_form(e, this.cast());
        let mut npc_form = Ptr::NULL;
        let mut creature_form = Ptr::NULL;
        match form_type(e, form) {
            0x2a => npc_form = form,
            0x2b => creature_form = form,
            _ => {}
        }
        let mut apply = 1u32;
        let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
        if package != 0 && e.call(PACKAGE_KIND_TEST, &args![package]).bool() {
            apply = 0;
        }
        if !npc_form.is_null() {
            e.call(
                NPC_INIT_DEFAULT_WORN,
                &args![npc_form, this, 1u32, apply, 0u32, 1u32],
            );
        } else if !creature_form.is_null() {
            e.call(
                CREATURE_INIT_DEFAULT_WORN,
                &args![creature_form, this, 1u32, apply, 1u32],
            );
        }
    }
}

// Translated from 0089fcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts combat between this actor and `target` (or a combat group, or a
/// pretend target). Does nothing for actors whose process level is 5 or 3
/// (`00437bf0`, `00437bd0`), for the target being the actor, for protected
/// (slot `0x22c(0)`) or dead actors. Unless `forced`, an actor in an
/// interrupting state (slot `0x214` not 0, 4 or 9) needs a furniture marker
/// entry whose byte `+0xe` is above `0x14` (slot `0x418` runs then); when
/// `0087f9c0` is 0 the target must be within the actor's reach plus a
/// quarter of it. Without a combat controller (slot `0x428`) the actor's
/// process is told (slot `0x2c8(0.0)`), the player's hostile status updated,
/// the crosshair cleared when it was on the actor, the equipped item queued
/// again, the interrupt package ended, and the combatant is added through
/// the `CombatManager` (a group member, a combatant with `value` or a
/// pretend combatant); the new controller then gets the acquire object's
/// slot `0x710` and is added as a package; byte `+0x104` (`bInCombat`)
/// records whether there is one. With a controller, a different target is
/// added to it, otherwise the group member is added. `value` is worked out
/// from the acquire object's slot `0x208` when it is 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_0089fcf0(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: Ptr<Actor>,
    group: u32,
    flag_a: u8,
    _unused_3: u32,
    flag_b: u8,
    value: i32,
    _unused_6: u32,
    pretend: u32,
) {
    if e.call(ACTOR_PROCESS_LEVEL_IS_5, &args![this]).bool()
        || e.call(ACTOR_PROCESS_LEVEL_IS_3, &args![this]).bool()
        || target == this
        || e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
        || e.mem.u8(this.addr() + 0x118) != 0
    {
        return;
    }
    if flag_a == 0 {
        let kind = e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).i32();
        if kind != 0 && kind != 4 && kind != 9 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            if e.vcall(acquire, PROCESS_SLOT_4D4, &args![]).u32() == 0 {
                return;
            }
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let entry = e.vcall(acquire, PROCESS_SLOT_4D4, &args![]).u32();
            if e.mem.u8(entry + 0xe) <= 0x14 {
                return;
            }
            e.vcall(this.addr(), ACTOR_SLOT_418, &args![]);
        }
    }
    if e.call(ACTOR_GETTER_87F9C0, &args![this]).f64() == double_constant(e, ZERO_DOUBLE) {
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![this, target, 0u32, 0u32],
            )
            .f32();
        let quarter_reach = (e.vcall(this.addr(), ACTOR_SLOT_REACH, &args![]).f64()
            * double_constant(e, QUARTER)) as f32;
        let reach = e.vcall(this.addr(), ACTOR_SLOT_REACH, &args![]).f64();
        if distance as f64 > reach + quarter_reach as f64 {
            return;
        }
    }
    let ranked = value > 0;
    let mut combat_value = 0;
    if value == 0 {
        let mut other = this.addr();
        if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            if e.vcall(acquire, PROCESS_SLOT_52C, &args![]).u32() != 0 {
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                other = e.vcall(acquire, PROCESS_SLOT_52C, &args![]).u32();
            }
        }
        if !target.is_null() && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![target]).u32() != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![target]).u32();
            let result = e
                .vcall(acquire, PROCESS_SLOT_208, &args![target, other])
                .f64();
            combat_value = e.call(FTOL, &args![result]).i32();
        }
    } else {
        combat_value = value;
    }
    let manager = e.global::<u32>(COMBAT_MANAGER);
    if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() == 0 {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        e.vcall(process, PROCESS_SLOT_2C8, &args![0.0f32]);
        let player_address = e.global::<u32>(PLAYER_CHARACTER);
        if target.addr() == player_address {
            e.call(
                PLAYER_CHANGE_PERCEIVED_HOSTILE_STATUS,
                &args![player_address, this, 1u32],
            );
        }
        // The base form's type is read but its result is not used.
        let form = base_form(e, this.cast());
        form_type(e, form);
        if e.call(CROSSHAIR_TARGET, &args![]).u32() == this.addr() {
            e.call(SET_CROSSHAIR_TARGET_TYPE, &args![0u32]);
        }
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let held = e.vcall(process, PROCESS_SLOT_22C, &args![]).u32();
        if held != 0 {
            let held = e.vcall(process, PROCESS_SLOT_22C, &args![]).u32();
            if e.call(PACKAGE_KIND_TEST, &args![held]).bool() {
                let item = e.vcall(this.addr(), ACTOR_SLOT_3BC, &args![6u32]).u32();
                if item != 0 {
                    // (the compiler also looked at the entry's form type
                    // here, without using it)
                    if e.call(ENTRY_FORM, &args![item]).u32() != 0 {
                        let form = e.call(ENTRY_FORM, &args![item]).ptr::<()>();
                        if form_type(e, form) == 0x28 {
                            e.call(ENTRY_FORM, &args![item]);
                        }
                    }
                    let form = e.call(ENTRY_FORM, &args![item]).u32();
                    e.call(
                        ACTOR_QUEUE_EQUIP_OBJECT,
                        &args![this, form, 1u32, 0u32, 1u32, 0u32, 1u32],
                    );
                    delete_entry(e, Ptr::new(item));
                }
            }
        }
        e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![this, 0u32]);
        let process = e.get(this, Actor::pCurrentProcess).addr();
        e.vcall(process, PROCESS_SLOT_214, &args![]);
        let kind = e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).i32();
        if kind == 4 || e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).i32() == 9 {
            e.vcall(this.addr(), ACTOR_SLOT_418, &args![]);
        }
        let mut controller = 0;
        if group != 0 {
            controller = e
                .call(
                    COMBAT_MANAGER_ADD_GROUP_MEMBER,
                    &args![manager, this, group],
                )
                .u32();
        } else if !target.is_null() {
            controller = e
                .call(
                    COMBAT_MANAGER_ADD_COMBATANT,
                    &args![manager, this, target, combat_value, u32::from(ranked)],
                )
                .u32();
        } else if pretend != 0 {
            controller = e
                .call(
                    COMBAT_MANAGER_ADD_PRETEND_COMBATANT,
                    &args![manager, this, pretend],
                )
                .u32();
        }
        if flag_b != 0 && controller != 0 {
            e.call(
                COMBAT_CONTROLLER_NOTIFY_DAMAGE,
                &args![controller, target, 0u32],
            );
        }
        if flag_a != 0 && controller != 0 {
            fn_008a0250(e, Ptr::new(controller));
        }
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, PROCESS_SLOT_28, &args![]);
        if controller != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            e.vcall(acquire, PROCESS_SLOT_710, &args![this]);
            e.vcall(
                this.addr(),
                ACTOR_SLOT_ADD_PACKAGE,
                &args![controller, 0u32, 1u32],
            );
            e.mem.set_u8(this.addr() + 0x104, 1);
        } else {
            e.mem.set_u8(this.addr() + 0x104, 0);
        }
    } else if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
        if !target.is_null() {
            let controller = fn_008a0330(e, this);
            if e.call(COMBAT_CONTROLLER_TARGET, &args![controller]).u32() != target.addr() {
                let controller = fn_008a0330(e, this);
                e.call(
                    COMBAT_CONTROLLER_ADD_TARGET,
                    &args![
                        controller,
                        target,
                        combat_value,
                        u32::from(ranked),
                        0.0f32,
                        0.0f32
                    ],
                );
                return;
            }
        }
        if group != 0 {
            e.call(
                COMBAT_MANAGER_ADD_GROUP_MEMBER,
                &args![manager, this, group],
            );
        }
    }
}

// Translated from 008a0270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor is in combat (`bInCombat`, `+0x104`): 0 for the player
/// and for an actor without a process; with `ignore_searching` set, 0 also
/// when it is only searching in combat (`bSearchingInCombat`, `+0x127`).
pub fn fn_008a0270(e: &mut Engine, this: Ptr<Actor>, ignore_searching: u8) -> u8 {
    if this.addr() == e.global::<u32>(PLAYER_CHARACTER) {
        return 0;
    }
    if e.get(this, Actor::pCurrentProcess).is_null() {
        return 0;
    }
    let in_combat = e.mem.u8(this.addr() + 0x104);
    if in_combat != 0 && ignore_searching != 0 && e.mem.u8(this.addr() + 0x127) != 0 {
        return 0;
    }
    in_combat
}

// Translated from 008a02d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCombatController` (the base-class body of `Actor` slot
/// `0x428`): the object the actor's process holds in its slot `0x22c`, when
/// the actor is in combat and that object's type byte (`+0x20`) is `0x12`;
/// otherwise 0.
pub fn fn_008a02d0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process == 0 || !e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
        return 0;
    }
    let held = e.vcall(process, PROCESS_SLOT_22C, &args![]).u32();
    if held != 0 && e.call(PACKAGE_TYPE, &args![held]).i32() == 0x12 {
        held
    } else {
        0
    }
}

// Translated from 008a0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The target of the actor's combat controller (slot `0x428`), or 0.
pub fn fn_008a0360(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    if controller != 0 {
        e.call(COMBAT_CONTROLLER_TARGET, &args![controller]).u32()
    } else {
        0
    }
}

// Translated from 008a03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps the actor's combat going once the screen has faded in: does
/// nothing while the fader (number 1) is visible, for actors failing
/// `00576d30`, actors not in combat or without a combat controller.
/// Stops the combat (slot `0x434`) when `00981450` says so or the
/// controller's target is no longer a combat target or is protected, dead
/// or otherwise unsuitable; otherwise registers the target with the
/// player's combat list, starts the controller (slot `0x64c`, or `0x630`
/// with the target) and runs the high (process level 0) or other combat
/// start of the controller.
pub fn fn_008a03a0(e: &mut Engine, this: Ptr<Actor>) {
    let fader = e.global::<u32>(FADER_MANAGER);
    let alpha = e
        .call(FADER_MANAGER_GET_FADER_ALPHA, &args![fader, 1u32])
        .f32();
    if alpha as f64 > double_constant(e, ZERO_DOUBLE) {
        return;
    }
    if e.call(ACTOR_TEST_576D30, &args![this]).bool() {
        return;
    }
    if !e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
        return;
    }
    let mut controller = fn_008a0330(e, this);
    if controller == 0 {
        return;
    }
    if e.call(CONTROLLER_TEST_981450, &args![controller]).bool() {
        e.vcall(this.addr(), ACTOR_SLOT_STOP_COMBAT, &args![0u32]);
        return;
    }
    let mut target = e.call(COMBAT_CONTROLLER_TARGET, &args![controller]).u32();
    if target != 0
        && !e
            .call(
                COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET,
                &args![controller, target],
            )
            .bool()
    {
        e.vcall(this.addr(), ACTOR_SLOT_STOP_COMBAT, &args![target]);
        controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
        if controller == 0 {
            return;
        }
        target = e.call(COMBAT_CONTROLLER_TARGET, &args![controller]).u32();
    }
    if e.call(ACTOR_PROCESS_LEVEL_IS_5, &args![this]).bool() {
        return;
    }
    let player_address = e.global::<u32>(PLAYER_CHARACTER);
    if target == player_address
        && e.call(PLAYER_TEST_5A03F0, &args![player_address, 1u32])
            .bool()
    {
        return;
    }
    if target != 0 {
        let protected = e.vcall(target, ACTOR_SLOT_22C, &args![0u32]).bool();
        let unsuitable = (protected && !e.vcall(target, ACTOR_SLOT_2E8, &args![]).bool())
            || e.call(ACTOR_TEST_440DA0, &args![target]).bool()
            || e.call(ACTOR_TEST_440D80, &args![target]).bool()
            || e.call(ACTOR_PROCESS_LEVEL_IS_3, &args![target]).bool();
        if unsuitable {
            e.vcall(this.addr(), ACTOR_SLOT_STOP_COMBAT, &args![target]);
            return;
        }
    }
    if target == player_address
        && e.call(PLAYER_COMBAT_LIST_COUNT, &args![player_address, this])
            .i32()
            > 0
    {
        e.call(
            PLAYER_ADD_ACTOR_TO_COMBAT_LIST,
            &args![player_address, this],
        );
    }
    if e.call(CONTROLLER_TEST_9818B0, &args![controller]).bool() {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, PROCESS_SLOT_64C, &args![]);
    } else {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, PROCESS_SLOT_630, &args![target]);
    }
    e.call(COMBAT_START_GUARD_NO_OP, &args![COMBAT_START_GUARD]);
    if e.call(MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, &args![this])
        .i32()
        == 0
    {
        e.call(CONTROLLER_START_HIGH, &args![controller]);
    } else {
        e.call(CONTROLLER_START_OTHER, &args![controller]);
    }
    e.call(COMBAT_START_GUARD_NO_OP, &args![COMBAT_START_GUARD]);
}

// Translated from 008a05f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a combat controller (slot `0x428`) calls the do-nothing `Error`
/// (`0040fbe0`) with the actor; without one sets the actor's process
/// (slot `0x54`) to the kind `1` when the process' field `+0x28` is 1, `2`
/// when it is 2 and `4` otherwise.
pub fn fn_008a05f0(e: &mut Engine, this: Ptr<Actor>) {
    if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
        e.call(ERROR_NO_OP, &args![this]);
        return;
    }
    let process = e.get(this, Actor::pCurrentProcess).addr();
    let kind = match e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() {
        1 => 1u32,
        2 => 2,
        _ => 4,
    };
    e.vcall(process, PROCESS_SLOT_54, &args![this, kind]);
}

// Translated from 008a0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards two words to `0097efe0` on the actor's combat controller (slot
/// `0x428`) when there is one.
pub fn fn_008a0680(e: &mut Engine, this: Ptr<Actor>, first: u32, second: u32) {
    let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    if controller != 0 {
        e.call(CONTROLLER_FORWARD_97EFE0, &args![controller, first, second]);
    }
}

/// `Actor::GetCurrentWeapon` (Xbox PDB).
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `Actor::GetPackageSetAsPcurrent` (Xbox PDB).
const ACTOR_GET_PACKAGE_SET_AS_PCURRENT: u32 = 0x0088_1510;
/// Test on the actor (`008ace90`).
const ACTOR_TEST_8ACE90: u32 = 0x008a_ce90;
/// The process level number of an actor or process (`004f8960`; 4 is
/// compared with).
const PROCESS_LEVEL_NUMBER: u32 = 0x004f_8960;
/// `Actor::GetDetectionLevelAgainstActor` (Xbox PDB), thiscall with the 7
/// words `0, other, &flag, 1, 1, 1, &flag`; returns the detection level.
const ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR: u32 = 0x008a_0d10;
/// Test on a package (`0067a770`).
const PACKAGE_TEST_67A770: u32 = 0x0067_a770;
/// Test on the actor (`005a3790`).
const ACTOR_TEST_5A3790: u32 = 0x005a_3790;
/// Pointer to the integer value of a setting (`0043d4d0`, `this` = the
/// setting).
const SETTING_GET_INT_POINTER: u32 = 0x0043_d4d0;
/// `ExtraDataList` call that runs when the friend-hit limit is at least
/// 1000 (`00422670`).
const EXTRA_RESET_FRIEND_HITS: u32 = 0x0042_2670;
/// `ExtraDataList::AddFriendHit` (Xbox PDB): returns the new count.
const EXTRA_ADD_FRIEND_HIT: u32 = 0x0042_2590;
/// `CombatDialogueManager::StartDialogue_ov2` (Xbox PDB): thiscall on the
/// manager with the speaker, the target and four numbers.
const COMBAT_DIALOGUE_START_DIALOGUE: u32 = 0x0098_39b0;
/// The `CombatDialogueManager` singleton pointer.
const COMBAT_DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// `Actor::IsTalking` (Xbox PDB).
const ACTOR_IS_TALKING: u32 = 0x008a_67f0;
/// `MobileObject::IsinDialogue` (Xbox PDB).
const MOBILE_OBJECT_IS_IN_DIALOGUE: u32 = 0x0093_36c0;
/// `ProcessLists::GetActorRefInHigh` (Xbox PDB): thiscall on the process
/// lists with a form and a flag.
const PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH: u32 = 0x0097_0a20;
/// The three-word variant of that lookup, used for owners of form type 8
/// (`00970b30`: the form, 0, 1).
const PROCESS_LISTS_GET_OWNER_ACTOR: u32 = 0x0097_0b30;
/// `Actor::AttackAlarm` (Xbox PDB): thiscall with the attacker, a flag and
/// `1`.
const ACTOR_ATTACK_ALARM: u32 = 0x008c_0460;
/// Test taking the other actor (`008b0670`).
const ACTOR_TEST_8B0670: u32 = 0x008b_0670;
/// Returns an actor related to the other one (`008b0ba0`).
const ACTOR_RELATED_8B0BA0: u32 = 0x008b_0ba0;
/// `Actor::IsAngryWithPlayer` (Xbox PDB).
const ACTOR_IS_ANGRY_WITH_PLAYER: u32 = 0x008b_ffc0;
/// `Actor::GetShouldAttackActor` (Xbox PDB): the other actor, a flag, a
/// pointer to the reaction and a flag.
const ACTOR_GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// `Actor::IsFleeing` (Xbox PDB): takes a flag.
const ACTOR_IS_FLEEING: u32 = 0x008a_6650;
/// Returns the object at `this + 0x30` of a base form (`005d8a70` is called
/// with it); the result goes to `008256d0`.
const FORM_SUB_OBJECT_ACCESSOR: u32 = 0x005d_8a70;
/// Test on that object (`008256d0`).
const FORM_SUB_OBJECT_TEST: u32 = 0x0082_56d0;
/// `0047ead0`: thiscall on the base form's embedded object (`+0x30`), taking
/// a flag.
const BASE_DATA_SET_FLAG: u32 = 0x0047_ead0;
/// `0097f580`: thiscall on the combat controller with the target and a word.
const CONTROLLER_WORKER_97F580: u32 = 0x0097_f580;
/// Process virtual slot `0x504` (target, flag): returns the detection record
/// of the target, or 0.
const PROCESS_SLOT_504: u32 = 0x504;
/// Process virtual slot `0xf0`: creates a detection record (target, level,
/// flag, detection, 0, flag, 1).
const PROCESS_SLOT_F0: u32 = 0xf0;
/// Process virtual slot `0x33c` (13 words).
const PROCESS_SLOT_33C: u32 = 0x33c;
/// `Actor` virtual slot `0x100` (a bool on the target).
const ACTOR_SLOT_100: u32 = 0x100;
/// `Actor` virtual slot `0x48` (takes a flag mask).
const ACTOR_SLOT_48: u32 = 0x48;
/// `Actor` virtual slot `0x304` (bool).
const ACTOR_SLOT_304: u32 = 0x304;
/// `Actor` virtual slot `0x37c`.
const ACTOR_SLOT_37C: u32 = 0x37c;
/// `Actor` virtual slot `0x42c` (returns an actor: the one this actor is
/// attacking or following).
const ACTOR_SLOT_42C: u32 = 0x42c;
/// `Actor` virtual slot `0x21c` (bool).
const ACTOR_SLOT_21C: u32 = 0x21c;
/// `Actor` virtual slot `0x460` (target, float).
const ACTOR_SLOT_460: u32 = 0x460;
/// `Actor` virtual slot `0x474` (flag).
const ACTOR_SLOT_474: u32 = 0x474;
/// Settings read by `fn_008987f0`: detection threshold (`float`), the
/// `float` that is unused afterwards, the friend-hit limits (ints) and the
/// float passed to slot `0x460`.
const DETECTION_THRESHOLD_SETTING: u32 = 0x011c_df10;
const UNUSED_FLOAT_SETTING: u32 = 0x011c_d45c;
const FRIEND_HIT_LIMIT_SETTINGS: [u32; 4] = [0x011c_d204, 0x011c_d5b0, 0x011c_d4b4, 0x011c_d468];
const SLOT_460_FLOAT_SETTING: u32 = 0x011d_13e0;

// Translated from 008987f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reacts to `target` being noticed (or acting) near this actor. Returns
/// early for protected actors (slot `0x22c(0)`), actors failing `008ace90`,
/// `target == this` and an actor of process level 4 whose slot `0x52c`
/// object is the target. Otherwise, unless this actor's combat controller
/// (slot `0x428`) already has `target` as a combat target:
/// - works out the detection level against the target
///   (`GetDetectionLevelAgainstActor`) and, at the high process level, stores
///   it (with the level 3 when it exceeds the detection setting) in the
///   target's detection record in the acquire object (slot `0x504`, created
///   by slot `0xf0` if needed);
/// - when the target is the player: asks the faction reaction; for reaction
///   2 or 3 counts a friend hit against the setting limit and either starts
///   the dialogue of `CombatDialogueManager` (talking and dialogue states
///   permitting) or goes on to raise the attack alarm through the owner and
///   itself;
/// - when `target` is not the player, this actor passes `0x304`, has no
///   controller and the target answers slots `0x218` (twice) and `0x37c`:
///   raises the attack alarm (unless the player branch did) and returns;
/// - when `008b0670` says so and `target` relates to this actor or the
///   player: works out whether it should attack, steals or alarms, sets the
///   "bark" flag through slot `0x460`, and tells the acquire object (slot
///   `0x33c`, 13 words);
///
/// Then lets the actor's slot `0x474` run, updates the base data (`+0x30`
/// object) and, when nothing alarmed and the flagged alarm is still due,
/// raises it; finally passes `target` and `param` to the combat controller
/// (`0097f580`) when the actor has one.
pub fn fn_008987f0(e: &mut Engine, this: Ptr<Actor>, target: Ptr<Actor>, param: u32) {
    // The exe's frame (`SUB ESP,0x80` under `EBP`): the detection bytes at +0x5b and +0x65
    // (`EBP-0x25`, `EBP-0x1b`), the reaction byte at +0x47 (`EBP-0x39`), the fight reaction
    // at +0x24 (`EBP-0x5c`) and its byte at +0x23 (`EBP-0x5d`).
    e.with_stack(0x80, |e, frame| {
        detect_in_frame(e, this, target, param, frame.addr())
    });
}

/// [`fn_008987f0`] in the exe's frame (`frame` is its lowest address).
fn detect_in_frame(e: &mut Engine, this: Ptr<Actor>, target: Ptr<Actor>, param: u32, frame: u32) {
    if e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return;
    }
    if e.call(ACTOR_TEST_8ACE90, &args![this]).bool() {
        return;
    }
    if target == this {
        return;
    }
    let package = e
        .call(ACTOR_GET_PACKAGE_SET_AS_PCURRENT, &args![this])
        .u32();
    if e.call(PROCESS_LEVEL_NUMBER, &args![this]).i32() == 4 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if e.vcall(acquire, PROCESS_SLOT_52C, &args![]).u32() == target.addr() {
            return;
        }
    }
    let mut weapon_flag = 0u8;
    let mut deferred_alarm = false;
    let mut alarmed = false;
    let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let process_type = e
        .call(MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, &args![this])
        .i32();
    let weapon = if target.is_null() {
        0
    } else {
        e.call(ACTOR_GET_CURRENT_WEAPON, &args![target]).u32()
    };
    if weapon != 0 && fn_00898fd0(e, Ptr::new(weapon)) {
        weapon_flag = 1;
    }
    let player_address = e.global::<u32>(PLAYER_CHARACTER);
    let already_target = controller != 0
        && e.call(
            COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET,
            &args![controller, target],
        )
        .bool();
    if !already_target {
        // Two bytes the detection call writes (the second preset to 1).
        let (flags, flags_second) = (frame + 0x5b, frame + 0x65);
        e.mem.set_u8(flags_second, 1);
        let detection = e
            .call(
                ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR,
                &args![this, 0u32, target, flags, 1u32, 1u32, 1u32, flags_second],
            )
            .i32();
        let mut level = 0u32;
        let setting = e
            .call(
                SETTING_GET_VALUE_POINTER,
                &args![DETECTION_THRESHOLD_SETTING],
            )
            .u32();
        if (e.mem.f32(setting) as f64) < detection as f64 {
            level = 3;
        }
        if process_type == 0 {
            let mut record = e
                .vcall(acquire, PROCESS_SLOT_504, &args![target, 0u32])
                .u32();
            if record == 0 && e.vcall(target.addr(), ACTOR_SLOT_100, &args![]).bool() {
                record = e
                    .vcall(
                        acquire,
                        PROCESS_SLOT_F0,
                        &args![
                            target,
                            level,
                            u32::from(e.mem.u8(flags)),
                            detection,
                            0u32,
                            u32::from(e.mem.u8(flags_second)),
                            1u32
                        ],
                    )
                    .u32();
            }
            e.mem.set_u32(record + 8, detection as u32);
            e.mem.set_u32(record + 4, level);
            let detected_flag = e.mem.u8(flags);
            e.mem.set_u8(record + 0x1e, detected_flag);
            let second_flag = e.mem.u8(flags_second);
            e.mem.set_u8(record + 0x1c, second_flag);
            e.mem.set_u8(record + 0x1d, 1);
        }
        // The value of this setting is read; nothing uses it afterwards.
        e.call(SETTING_GET_VALUE_POINTER, &args![UNUSED_FLOAT_SETTING]);
        if package != 0 {
            e.call(PACKAGE_TEST_67A770, &args![package]);
        }
        if target.addr() == player_address
            && (controller == 0
                || !e
                    .call(
                        COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET,
                        &args![controller, player_address],
                    )
                    .bool())
        {
            let reaction_flag = frame + 0x47;
            e.mem.set_u8(reaction_flag, 0);
            let reaction = e
                .call(
                    ACTOR_GET_FACTION_FIGHT_REACTION,
                    &args![this, player_address, reaction_flag],
                )
                .i32();
            if reaction == 3 || reaction == 2 {
                if e.call(ACTOR_TEST_5A3790, &args![this]).bool() {
                    return;
                }
                let setting_address = match (reaction == 3, controller != 0) {
                    (true, true) => FRIEND_HIT_LIMIT_SETTINGS[0],
                    (true, false) => FRIEND_HIT_LIMIT_SETTINGS[1],
                    (false, true) => FRIEND_HIT_LIMIT_SETTINGS[2],
                    (false, false) => FRIEND_HIT_LIMIT_SETTINGS[3],
                };
                let pointer = e
                    .call(SETTING_GET_INT_POINTER, &args![setting_address])
                    .u32();
                let limit = e.mem.u32(pointer) as i32;
                if limit > 0 {
                    if limit >= 1000 {
                        let extra = extra_data_list_of(e, this.cast());
                        e.call(EXTRA_RESET_FRIEND_HITS, &args![extra]);
                    }
                    let extra = extra_data_list_of(e, this.cast());
                    let hits = e.call(EXTRA_ADD_FRIEND_HIT, &args![extra]).i32();
                    e.vcall(this.addr(), ACTOR_SLOT_48, &args![0x8000_0000u32]);
                    if hits <= limit {
                        if e.call(ACTOR_IS_TALKING, &args![this]).bool() {
                            return;
                        }
                        if e.call(MOBILE_OBJECT_IS_IN_DIALOGUE, &args![this]).bool() {
                            return;
                        }
                        if e.call(ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR, &args![this, 5u32])
                            .bool()
                        {
                            return;
                        }
                        let manager = e.global::<u32>(COMBAT_DIALOGUE_MANAGER);
                        e.call(
                            COMBAT_DIALOGUE_START_DIALOGUE,
                            &args![manager, this, target, 2u32, 2u32, 1u32, 0u32],
                        );
                        return;
                    }
                }
            }
            if e.call(REFR_GET_OWNER, &args![this]).u32() != 0 {
                let owner = e.call(REFR_GET_OWNER, &args![this]).ptr::<()>();
                let owner_actor = if form_type(e, owner) == 8 {
                    e.call(
                        PROCESS_LISTS_GET_OWNER_ACTOR,
                        &args![PROCESS_LISTS, owner, 0u32, 1u32],
                    )
                    .u32()
                } else {
                    e.call(
                        PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH,
                        &args![PROCESS_LISTS, owner, 0u32],
                    )
                    .u32()
                };
                if owner_actor != 0 {
                    e.call(
                        ACTOR_ATTACK_ALARM,
                        &args![owner_actor, target, u32::from(weapon_flag), 1u32],
                    );
                }
            }
            e.call(
                ACTOR_ATTACK_ALARM,
                &args![this, target, u32::from(weapon_flag), 1u32],
            );
            alarmed = true;
        }
        if target.addr() != player_address
            && e.vcall(this.addr(), ACTOR_SLOT_304, &args![]).bool()
            && controller == 0
            && e.vcall(target.addr(), ACTOR_SLOT_218, &args![]).bool()
            && e.vcall(target.addr(), ACTOR_SLOT_218, &args![]).bool()
            && e.vcall(target.addr(), ACTOR_SLOT_37C, &args![]).u32() != 0
        {
            if !alarmed {
                e.call(
                    ACTOR_ATTACK_ALARM,
                    &args![this, target, u32::from(weapon_flag), 1u32],
                );
            }
            return;
        }
        if e.call(ACTOR_TEST_8B0670, &args![this, target]).bool() {
            let related = target.addr() == player_address
                || e.vcall(target.addr(), ACTOR_SLOT_42C, &args![]).u32() == this.addr()
                || (e.call(ACTOR_RELATED_8B0BA0, &args![target]).u32() == player_address
                    && e.vcall(target.addr(), ACTOR_SLOT_42C, &args![]).u32() == player_address);
            if related && this.addr() != player_address {
                let mut should_attack = 0u32;
                let (state, state_byte) = (frame + 0x24, frame + 0x23);
                let reaction = e
                    .call(
                        ACTOR_GET_FACTION_FIGHT_REACTION,
                        &args![this, target, state_byte],
                    )
                    .u32();
                e.mem.set_u32(state, reaction);
                if !e.call(ACTOR_IS_ANGRY_WITH_PLAYER, &args![this]).bool()
                    && e.mem.u8(state_byte) == 0
                    && e.call(
                        ACTOR_GET_SHOULD_ATTACK_ACTOR,
                        &args![this, target, 0u32, state, 0u32],
                    )
                    .bool()
                {
                    should_attack = 1;
                }
                if controller == 0
                    && !e.vcall(target.addr(), ACTOR_SLOT_304, &args![]).bool()
                    && target.addr() == player_address
                {
                    if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() == 0
                        && !e.call(ACTOR_IS_FLEEING, &args![this, 0u32]).bool()
                        && !alarmed
                    {
                        if e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool() {
                            let extra = extra_data_list_of(e, this.cast());
                            if e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32() != 0 {
                                let extra = extra_data_list_of(e, this.cast());
                                let owner = e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32();
                                e.call(
                                    ACTOR_STEAL_ALARM,
                                    &args![player_address, this, 0u32, 1u32, 0u32, owner],
                                );
                            }
                        } else {
                            e.call(
                                ACTOR_ATTACK_ALARM,
                                &args![this, target, u32::from(weapon_flag), 1u32],
                            );
                        }
                    } else {
                        let setting = e
                            .call(SETTING_GET_VALUE_POINTER, &args![SLOT_460_FLOAT_SETTING])
                            .u32();
                        let value = e.mem.f32(setting);
                        e.vcall(this.addr(), ACTOR_SLOT_460, &args![target, value]);
                        deferred_alarm = true;
                    }
                }
                e.vcall(
                    acquire,
                    PROCESS_SLOT_33C,
                    &args![
                        this,
                        target,
                        1u32,
                        1u32,
                        0u32,
                        0u32,
                        0u32,
                        should_attack,
                        0u32,
                        0u32,
                        should_attack,
                        0u32,
                        0u32
                    ],
                );
            }
        }
        if this.addr() != player_address || e.call(ACTOR_IN_COMBAT, &args![target]).bool() {
            e.vcall(this.addr(), ACTOR_SLOT_474, &args![1u32]);
        }
        let mut base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
        if base != 0 {
            let part = e.call(FORM_SUB_OBJECT_ACCESSOR, &args![base + 0x30]).u32();
            if e.call(FORM_SUB_OBJECT_TEST, &args![part]).bool() {
                base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
            }
        }
        if target.addr() == player_address {
            e.call(BASE_DATA_SET_FLAG, &args![base + 0x30, 1u32]);
        }
    }
    if !alarmed && deferred_alarm && !e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
        if e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool() {
            let extra = extra_data_list_of(e, this.cast());
            if e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32() != 0 {
                let extra = extra_data_list_of(e, this.cast());
                let owner = e.call(EXTRA_GET_OWNER_FORM, &args![extra]).u32();
                e.call(
                    ACTOR_STEAL_ALARM,
                    &args![player_address, this, 0u32, 1u32, 0u32, owner],
                );
            }
        } else {
            e.call(
                ACTOR_ATTACK_ALARM,
                &args![this, target, u32::from(weapon_flag), 1u32],
            );
        }
    }
    let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    if controller != 0 {
        e.call(CONTROLLER_WORKER_97F580, &args![controller, target, param]);
    }
}

/// The `VATS` singleton (the address the code passes as `this` of the
/// `VATS` calls and of the accessor `0044ddc0`; the dword at `+8` is 4 while
/// a VATS action plays).
const VATS_OBJECT: u32 = 0x011f_2250;
/// The pointer to the actor VATS is acting for.
const VATS_ACTOR: u32 = 0x011f_21f0;
/// `VATS::GetCurrentAction` (Xbox PDB).
const VATS_GET_CURRENT_ACTION: u32 = 0x009c_71c0;
/// `BGSDefaultObjectManager::GetDefaultObject` (Xbox PDB), cdecl.
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
/// The RTTI type descriptor of `MagicItem` (the class the default object
/// number 5 is cast to).
const RTTI_TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
/// `MiscStatManager::Increment` (Xbox PDB), cdecl.
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// Offset of the magic caster embedded in an `Actor`.
const ACTOR_MAGIC_CASTER_OFFSET: u32 = 0x88;
/// Magic caster virtual slot `0x40`.
const MAGIC_CASTER_SLOT_40: u32 = 0x40;
/// Magic caster virtual slot `0x48`.
const MAGIC_CASTER_SLOT_48: u32 = 0x48;
/// Reads the integer setting at `DEBUG_DRAW_SETTING` (`004503f0`).
const DEBUG_DRAW_LEVEL: u32 = 0x0045_03f0;
const DEBUG_DRAW_SETTING: u32 = 0x011d_f838;
/// Four-`float` constructor (`00414430`, thiscall, 4 floats).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// Rotation matrix times vector (`00524c40`: thiscall on the matrix, taking
/// an output vector and the input vector; returns the output).
const MATRIX_TRANSFORM: u32 = 0x0052_4c40;
/// Vector subtraction (`00439ef0`: thiscall on the minuend, taking an output
/// vector and the subtrahend; returns the output).
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// Vector normalization in place (`004a0c10`).
const VECTOR_NORMALIZE: u32 = 0x004a_0c10;
/// Vector times scalar (`0045bb20`: thiscall on the vector, taking an
/// output vector and the scalar; returns the output).
const VECTOR_SCALE: u32 = 0x0045_bb20;
/// `BSTestObjects::CreateDirArrow` (Xbox PDB), cdecl: vector, colour.
const CREATE_DIR_ARROW: u32 = 0x00c5_9a50;
/// Sets the position of a debug object (`00440460`, takes the position).
const DEBUG_OBJECT_SET_POSITION: u32 = 0x0044_0460;
/// Reads a `float` setting (`00450410`, `this` = `DEBUG_DRAW_DURATION`).
const DEBUG_DRAW_DURATION_GETTER: u32 = 0x0045_0410;
const DEBUG_DRAW_DURATION: u32 = 0x011d_f754;
/// `TES::AddTempDebugObject` (Xbox PDB): the object and the seconds.
const TES_ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// `800.0f` and `0.1f` (the scales applied to the arrow vector).
const ARROW_LENGTH_SCALE: u32 = 0x0102_2454;
const ARROW_WIDTH_SCALE: u32 = 0x0101_e2bc;
/// `0087b040`: thiscall on the object `GET_TES` returns, taking the actor
/// and the two flags.
const TES_QUEUE_ATTACK: u32 = 0x0087_b040;
/// Process virtual slot `0x460`: returns the address of a vector (the weapon
/// bone offset the debug arrow starts from).
const PROCESS_SLOT_460: u32 = 0x460;
/// Process virtual slot `0x448`.
const PROCESS_SLOT_448: u32 = 0x448;
/// `008bc240`: thiscall on the actor, taking one word.
const ACTOR_WORKER_8BC240: u32 = 0x008b_c240;
/// `005fbeb0`: thiscall on the base form, returns a word.
const FORM_GETTER_5FBEB0: u32 = 0x005f_beb0;
/// The integer setting passed to `008bc240` when slot `0x21c` is false.
const WORD_SETTING_3B4: u32 = 0x011c_d3b4;
/// Process virtual slot `0x274`.
const PROCESS_SLOT_274: u32 = 0x274;
/// `*(this + 0x18)` (`009611e0`).
const OBJECT_TYPE_FIELD: u32 = 0x0096_11e0;
/// `*(this + 4)` (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Actor test `008ae660` (takes a flag).
const ACTOR_TEST_8AE660: u32 = 0x008a_e660;
/// `0097f7d0`: thiscall on the combat controller, taking the target.
const CONTROLLER_WORKER_97F7D0: u32 = 0x0097_f7d0;
/// `Actor::CombatHit` (Xbox PDB), `00899cb0`.
const ACTOR_COMBAT_HIT: u32 = 0x0089_9cb0;
/// `BSSoundHandle::SetPosition` (Xbox PDB): the three coordinates.
const SOUND_HANDLE_SET_POSITION_XYZ: u32 = 0x00ad_8b60;
/// `BSSoundHandle::SetObjectToFollow` (Xbox PDB).
const SOUND_HANDLE_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
/// Test on the weapon's actor record (`00406090`).
const RECORD_TEST_406090: u32 = 0x0040_6090;
/// `TESActorBaseData::GetFatigue` (Xbox PDB): a 16-bit value.
const ACTOR_BASE_DATA_GET_FATIGUE: u32 = 0x004a_8ae0;

// Translated from 00899200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Carries out the attack animation's hit on the actor's side.
///
/// With the task queue in use (`008c7aa0`) only forwards `(this, flag_a,
/// flag_b)` to `0087b040`. Otherwise: during a VATS action of this actor
/// uses up a shot of the action or casts the default magic object number 5
/// and counts the stat `0x15`; draws a debug arrow from the weapon bone to
/// the weapon position when the debug setting is above 1; tells the
/// acquire object (slot `0x448`); and runs `008bc240` with the form
/// word (or the setting) for the actor kind. With `flag_b` set it then
/// finds the target to hit (the acquire object's slot `0x274` reference when
/// slot `0x27c`'s object has the type number `0x2c`, otherwise
/// `fn_008990f0`), gives up for an actor whose slot `0x360` says to when
/// `008ae660` agrees, and either runs `Actor::CombatHit` on the target (with
/// the kind 1 for the animation groups `0x38`, `0x64`, `0x65` and 2 for
/// `0x3e`) or, with no target, plays the weapon's sound at the actor and
/// casts the weapon's actor-base spell when the animation matches. Ends by
/// telling the combat controller (slot `0x428`) with `0097f7d0`.
/// The compiler's exception-unwinding frame is not translated; `CombatHit`
/// is called by its address.
pub fn fn_00899200(e: &mut Engine, this: Ptr<Actor>, flag_a: u8, flag_b: u8) {
    if e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
        let tes = e.call(GET_TES, &args![]).u32();
        e.call(TES_QUEUE_ATTACK, &args![tes, this, flag_a, flag_b]);
        return;
    }
    let caster = this.addr() + ACTOR_MAGIC_CASTER_OFFSET;
    if e.call(ENTRY_FORM, &args![VATS_OBJECT]).u32() == 4
        && this.addr() == e.global::<u32>(VATS_ACTOR)
    {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let weapon_entry = e
            .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32();
        let weapon_form = if weapon_entry != 0 {
            let weapon_entry = e
                .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                .u32();
            e.call(ENTRY_FORM, &args![weapon_entry]).u32()
        } else {
            0
        };
        let action = e.call(VATS_GET_CURRENT_ACTION, &args![VATS_OBJECT]).u32();
        if action != 0 {
            if weapon_form != 0 && e.call(WEAPON_BYTE_FLAG, &args![weapon_form]).bool() {
                let shots = e.mem.u8(action + 8);
                e.mem.set_u8(action + 8, shots.wrapping_sub(1));
            } else {
                let reference = e.mem.u32(action + 0xc);
                if e.mem.u8(action + 7) != 0
                    && !e.vcall(reference, 0x234, &args![]).bool()
                    && e.vcall(reference, 0x100, &args![]).bool()
                {
                    let object = e.call(GET_DEFAULT_OBJECT, &args![5u32]).u32();
                    if object != 0 {
                        let item = e
                            .call(
                                RTTI_DYNAMIC_CAST,
                                &args![
                                    object,
                                    0u32,
                                    RTTI_TYPE_TES_FORM,
                                    RTTI_TYPE_MAGIC_ITEM,
                                    0u32
                                ],
                            )
                            .u32();
                        e.vcall(caster, MAGIC_CASTER_SLOT_40, &args![item]);
                        let reference = e.mem.u32(action + 0xc);
                        let source = if reference != 0 { reference + 0x94 } else { 0 };
                        e.vcall(caster, MAGIC_CASTER_SLOT_48, &args![source]);
                        if e.call(MAGIC_CASTER_RELEASE_CAST, &args![caster, 0u32])
                            .bool()
                        {
                            e.call(MISC_STAT_INCREMENT, &args![0x15u32]);
                        }
                    }
                }
            }
        }
    }
    if e.call(DEBUG_DRAW_LEVEL, &args![DEBUG_DRAW_SETTING]).i32() > 1 {
        draw_hit_arrow(e, this);
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    e.vcall(acquire, PROCESS_SLOT_448, &args![1u32]);
    if e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool() {
        let form = base_form(e, this.cast());
        let word = e.call(FORM_GETTER_5FBEB0, &args![form]).u32();
        e.call(ACTOR_WORKER_8BC240, &args![this, word]);
    } else {
        let setting = e
            .call(SETTING_GET_INT_POINTER, &args![WORD_SETTING_3B4])
            .u32();
        let word = e.mem.u32(setting);
        e.call(ACTOR_WORKER_8BC240, &args![this, word]);
    }
    if flag_b == 0 {
        return;
    }

    let mut found = false;
    let mut hit_target = 0u32;
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if acquire != 0 {
        let holder = e.vcall(acquire, PROCESS_SLOT_27C, &args![]).u32();
        if holder != 0 && e.call(OBJECT_TYPE_FIELD, &args![holder]).u32() == 0x2c {
            let entry = e.vcall(acquire, PROCESS_SLOT_274, &args![]).u32();
            if entry != 0 {
                e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]);
                let next = e.call(NODE_NEXT, &args![entry]).u32();
                if next != 0 && e.vcall(next, ACTOR_SLOT_100, &args![]).bool() {
                    hit_target = next;
                    found = true;
                }
            }
        }
    }
    if !found {
        hit_target = fn_008990f0(e, this);
    }
    if hit_target == 0
        && e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        && e.call(ACTOR_TEST_8AE660, &args![this, flag_a]).bool()
    {
        return;
    }
    if hit_target != 0 && e.call(ACTOR_TEST_8ACE90, &args![hit_target]).bool() {
        hit_target = 0;
    }
    let weapon_entry = if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32()
    } else {
        0
    };
    let weapon_form = if weapon_entry != 0 {
        e.call(ENTRY_FORM, &args![weapon_entry]).u32()
    } else {
        0
    };
    if hit_target != 0 {
        let mut kind = 0u32;
        if weapon_form == 0 || e.call(WEAPON_TYPE, &args![weapon_form]).u32() == 0 {
            let process = e.get(this, Actor::pCurrentProcess).addr();
            let sequence = e.vcall(process, PROCESS_SLOT_ANIM_SEQUENCE, &args![]).u32();
            let transform = e
                .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                .u32();
            match e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).u32() {
                0x38 | 0x64 | 0x65 => kind = 1,
                0x3e => kind = 2,
                _ => {}
            }
        }
        e.call(
            ACTOR_COMBAT_HIT,
            &args![this, hit_target, flag_a, 0u32, kind],
        );
    } else {
        let sound_set = if weapon_form != 0 {
            e.call(WEAPON_SOUND_SET, &args![weapon_form]).u32()
        } else {
            0
        };
        if sound_set != 0 {
            play_weapon_sound(e, this, sound_set);
        }
        cast_weapon_spell(e, this, caster);
    }
    let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    if controller != 0 {
        e.call(CONTROLLER_WORKER_97F7D0, &args![controller, hit_target]);
    }
}

/// Copies the three words at `from` to `to`.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The debug drawing of `fn_00899200`: an arrow from the position of the
/// weapon bone towards the weapon position, in blue for an actor whose slot
/// `0x360` is true and red otherwise, shown for a number of seconds.
/// In `00899200`'s frame (`EBP` frame, exception record, `SUB ESP,0x154`, the cookie: 0x164
/// bytes): the colour at +0x108 (`EBP-0x5c`) and its temporary at +0x88 or +0x78
/// (`EBP-0xdc` for the blue one, `EBP-0xec` for the red one); the two points at +0xf8 and
/// +0x118 (`EBP-0x6c`, `EBP-0x4c`) with their temporaries at +0x6c, +0x60 and +0x54; the
/// direction at +0xe8 (`EBP-0x7c`) from the difference at +0x48; the scaled vectors at +0x124
/// and +0x3c (`EBP-0x40`, `EBP-0x128`).
fn draw_hit_arrow(e: &mut Engine, this: Ptr<Actor>) {
    e.with_stack(0x164, |e, frame| {
        draw_hit_arrow_in_frame(e, this, frame.addr())
    });
}

/// [`draw_hit_arrow`] in the exe's frame.
fn draw_hit_arrow_in_frame(e: &mut Engine, this: Ptr<Actor>, frame: u32) {
    let color = frame + 0x108;
    let first = frame + 0xf8;
    let second = frame + 0x118;
    let bone_temp = frame + 0x6c;
    let weapon_temp = frame + 0x60;
    let transformed_temp = frame + 0x54;
    let difference_temp = frame + 0x48;
    let direction = frame + 0xe8;
    let scaled = frame + 0x124;
    let scaled_again = frame + 0x3c;
    let zero = 0.0f32;
    let one = 1.0f32;
    e.call(COLOR_CONSTRUCT, &args![color, zero, zero, zero, zero]);
    let made = if e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool() {
        e.call(COLOR_CONSTRUCT, &args![frame + 0x88, zero, zero, one, one])
    } else {
        e.call(COLOR_CONSTRUCT, &args![frame + 0x78, one, zero, zero, one])
    }
    .u32();
    for word in 0..4 {
        let value = e.mem.u32(made + 4 * word);
        e.mem.set_u32(color + 4 * word, value);
    }
    // `first` and `second` are `NiPoint3` temporaries whose constructor
    // (`006815c0`) does nothing.
    let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
    if node != 0 {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let bone = e.vcall(process, PROCESS_SLOT_460, &args![]).u32();
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        let result = e
            .call(MATRIX_TRANSFORM, &args![rotation, bone_temp, bone])
            .u32();
        copy_vector(e, result, first);
        let position = e
            .call(ACTOR_GET_WEAPON_POSITION, &args![this, weapon_temp, 0u32])
            .u32();
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        let result = e
            .call(
                MATRIX_TRANSFORM,
                &args![rotation, transformed_temp, position],
            )
            .u32();
        copy_vector(e, result, second);
    }
    let result = e
        .call(VECTOR_SUBTRACT, &args![second, difference_temp, first])
        .u32();
    copy_vector(e, result, direction);
    e.call(VECTOR_NORMALIZE, &args![direction]);
    let length_scale = e.global::<f32>(ARROW_LENGTH_SCALE);
    e.call(VECTOR_SCALE, &args![direction, scaled, length_scale]);
    let width_scale = e.global::<f32>(ARROW_WIDTH_SCALE);
    let arrow_vector = e
        .call(VECTOR_SCALE, &args![scaled, scaled_again, width_scale])
        .u32();
    let arrow = e.call(CREATE_DIR_ARROW, &args![arrow_vector, color]).u32();
    e.call(DEBUG_OBJECT_SET_POSITION, &args![arrow, second]);
    let seconds = e
        .call(DEBUG_DRAW_DURATION_GETTER, &args![DEBUG_DRAW_DURATION])
        .f32();
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, arrow, seconds]);
}

/// Plays the sound of the weapon's sound set (`sound_set`) at the actor's
/// position, following its node.
/// The sound handle sits at +0xa0 (`EBP-0xc4`) of `00899200`'s 0x164-byte frame.
fn play_weapon_sound(e: &mut Engine, this: Ptr<Actor>, sound_set: u32) {
    e.with_stack(0x164, |e, frame| {
        let handle = frame.addr() + 0xa0;
        let file_name = e.call(SOUND_SET_FILE_NAME, &args![sound_set]).u32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(
            AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
            &args![audio, handle, file_name, 0x102u32, sound_set],
        );
        let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        e.call(SOUND_HANDLE_SET_POSITION_XYZ, &args![handle, x, y, z]);
        let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
        e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![handle, node]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

/// Casts the spell of the actor base form's record (`base + 0x70`, its first
/// entry plus `0x18`) when its test passes and the animation the actor plays
/// has the group the record names (or the record names `0xff`).
fn cast_weapon_spell(e: &mut Engine, this: Ptr<Actor>, caster: u32) {
    let base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
    let node = e.call(NODE_NEXT, &args![base + 0x70]).u32();
    let record = if node != 0 { node + 0x18 } else { 0 };
    if record == 0 || !e.call(RECORD_TEST_406090, &args![record + 0xc]).bool() {
        return;
    }
    let mut matched = false;
    let base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
    let group = u32::from(
        e.call(ACTOR_BASE_DATA_GET_FATIGUE, &args![base + 0x70])
            .u16(),
    );
    if group == 0xff {
        matched = true;
    } else if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if e.vcall(acquire, PROCESS_SLOT_ANIM_SEQUENCE, &args![]).u32() != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let sequence = e.vcall(acquire, PROCESS_SLOT_ANIM_SEQUENCE, &args![]).u32();
            if e.call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                .u32()
                != 0
            {
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                let sequence = e.vcall(acquire, PROCESS_SLOT_ANIM_SEQUENCE, &args![]).u32();
                let transform = e
                    .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
                    .u32();
                if e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).u32() == group {
                    matched = true;
                }
            }
        }
    }
    if matched {
        e.vcall(caster, MAGIC_CASTER_SLOT_40, &args![record]);
        let source = if this.is_null() {
            0
        } else {
            this.addr() + 0x94
        };
        e.vcall(caster, MAGIC_CASTER_SLOT_48, &args![source]);
        e.call(MAGIC_CASTER_RELEASE_CAST, &args![caster, 0u32]);
    }
}
/// `Actor::HitMe` (Xbox PDB), `0089a760`: thiscall on the target, taking the
/// `HitData` and a kind byte.
const ACTOR_HIT_ME: u32 = 0x0089_a760;
/// Process-level test on an actor (`00437b90`).
const ACTOR_PROCESS_LEVEL_TEST_437B90: u32 = 0x0043_7b90;
/// `HitData::HitData` (Xbox PDB): thiscall on the memory, returns it.
const HIT_DATA_CONSTRUCT: u32 = 0x009b_4d90;
/// `NiPointer<HitData>` construction from a raw pointer (`008c1c30`):
/// thiscall on the 4-byte holder; stores the pointer and takes a reference.
const NI_POINTER_HIT_DATA_CONSTRUCT: u32 = 0x008c_1c30;
/// `NiPointer<HitData>::~NiPointer<HitData>` (Xbox PDB).
const NI_POINTER_HIT_DATA_DESTRUCT: u32 = 0x008c_1c60;
/// `NiPointer::operator->`: returns the pointer the holder (`this`) keeps
/// (`*this`).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `HitData::Copy` (Xbox PDB): thiscall on the destination, taking the
/// source.
const HIT_DATA_COPY: u32 = 0x009b_4ec0;
/// `HitData::InitializeHitData_ov2` (Xbox PDB), cdecl: the hit data, the
/// attacker, the target, the weapon entry, the attack flag byte and the
/// source reference.
const HIT_DATA_INITIALIZE: u32 = 0x009b_5120;
/// Form flag test (`00444d00`): thiscall on the reference, taking a mask.
const REFR_FLAG_TEST: u32 = 0x0044_4d00;
/// `00407e00`: thiscall on the hit data's critical effect holder, taking
/// two words (`0x3fc`, 0).
const HIT_DATA_CLEAR_EFFECT: u32 = 0x0040_7e00;
/// `00476c90`: thiscall on the source reference, taking a flag, 0 and the
/// attacker; returns the weapon object the projectile count is read from.
const SOURCE_WEAPON_LOOKUP: u32 = 0x0047_6c90;
/// `TESObjectWEAP::GetNumProjectiles` (Xbox PDB): thiscall, a byte result.
const WEAPON_GET_NUM_PROJECTILES: u32 = 0x0052_5b20;
/// Test on the base object of the source reference (`004fd3c0`).
const BASE_TEST_4FD3C0: u32 = 0x004f_d3c0;
/// `00885d70`: thiscall on the source reference; the result is in ST0.
const REFR_GETTER_885D70: u32 = 0x0088_5d70;
/// `HitData` flag test (`0058cba0`): thiscall on the hit data, taking a
/// flag number.
const HIT_DATA_FLAG_TEST: u32 = 0x0058_cba0;
/// `ImpactMixer::PlayWeaponBlock` (Xbox PDB), cdecl: the weapon type, the
/// weapon form, two positions of 3 words and a flag.
const IMPACT_MIXER_PLAY_WEAPON_BLOCK: u32 = 0x0083_97f0;
/// The weapon-hit sound call (`00837520`), cdecl, 9 words.
const IMPACT_MIXER_PLAY_WEAPON_HIT: u32 = 0x0083_7520;
/// The pointer read as the second word of the block sound call when the
/// target holds no weapon.
const IMPACT_BLOCK_DEFAULT_SOUND: u32 = 0x011c_a278;
/// `00476c90`'s companion: `ItemChange` number read (`004bd350`, ST0).
const ITEM_CHANGE_GETTER_4BD350: u32 = 0x004b_d350;
/// `Actor` virtual slot `0x3d8` (weapon entry, target, source reference, out
/// byte): returns a byte.
const ACTOR_SLOT_3D8: u32 = 0x3d8;
/// `Actor` virtual slot `0x3c8` (weapon entry, `float`, flag): returns a
/// byte.
const ACTOR_SLOT_3C8: u32 = 0x3c8;
/// `Actor` virtual slots `0x32c` (getter), `0x328` (setter) and `0x330`.
const ACTOR_SLOT_32C: u32 = 0x32c;
const ACTOR_SLOT_328: u32 = 0x328;
const ACTOR_SLOT_330: u32 = 0x330;
/// `Actor` virtual slot `0x4c0` (hit damage, blocked fraction, target,
/// source reference).
const ACTOR_SLOT_4C0: u32 = 0x4c0;
/// `00444d00` mask used for the critical-hit check.
const REFR_FLAG_NO_CRIT: u32 = 0x0002_0000;
/// `TESPackage::GetUseWeaponPackageData` (Xbox PDB).
const PACKAGE_GET_USE_WEAPON_PACKAGE_DATA: u32 = 0x0067_58f0;
/// Test on that data (`00524cd0`).
const USE_WEAPON_DATA_TEST: u32 = 0x0052_4cd0;
/// `00407e00`'s mask word.
const HIT_DATA_EFFECT_MASK: u32 = 0x3fc;
/// Player worker (`009627a0`, thiscall on the player).
const PLAYER_WORKER_9627A0: u32 = 0x0096_27a0;

// `HitData` (Xbox PDB, 0x64 bytes) field offsets used here.
const HIT_SOURCE_REF: u32 = 0x08;
const HIT_HEALTH_DAMAGE: u32 = 0x14;
const HIT_TOTAL_DAMAGE: u32 = 0x18;
const HIT_FATIGUE_DAMAGE: u32 = 0x1c;
const HIT_TARGETED_LIMB_DAMAGE: u32 = 0x20;
const HIT_PERCENT_BLOCKED: u32 = 0x24;
const HIT_ARMOR_DAMAGE: u32 = 0x28;
const HIT_DAMAGE_TO_WEAPON: u32 = 0x2c;
const HIT_LOCATION: u32 = 0x38;
const HIT_DIRECTION: u32 = 0x44;
const HIT_CRITICAL_EFFECT: u32 = 0x50;
const HIT_ATTACK_SKILL: u32 = 0x0c;

/// The `HitData` pointer a holder keeps (through `operator->`, `00559450`).
fn hit_data_of(e: &mut Engine, holder: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![holder]).u32()
}

/// Divides the `float` at `address` by `divisor`.
fn divide_float(e: &mut Engine, address: u32, divisor: u32) {
    let value = e.mem.f32(address) as f64 / divisor as f64;
    e.mem.set_f32(address, value as f32);
}

/// Multiplies the `float` at `address` by `factor`.
fn multiply_float(e: &mut Engine, address: u32, factor: f32) {
    let value = e.mem.f32(address) as f64 * factor as f64;
    e.mem.set_f32(address, value as f32);
}

/// The three words of the position `slot 0x1f4` of `actor` returns.
fn position_words(e: &mut Engine, actor: u32) -> [u32; 3] {
    let position = e.vcall(actor, ACTOR_SLOT_POSITION, &args![]).u32();
    [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ]
}

// Translated from 00899cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CombatHit` (Xbox PDB): the attack `this` makes lands on `target`.
///
/// Gives up for a missing target, `008ace90` and `00437b90`. Looks up the
/// weapon entry/form/type, asks slot `0x3d8` whether the hit connects, builds
/// the `HitData` (a new `0x64`-byte object kept in an `NiPointer`): during a
/// VATS action the action's data is copied in and the freshly initialized
/// one is merged over it, otherwise it is just initialized
/// (`HitData::InitializeHitData_ov2`). Zeroes the damage of an attack whose
/// source reference has the form flag `0x20000`, splits it among the
/// projectiles of the weapon and applies the armor-piercing correction
/// (`004fd3c0`). Updates the actor's hit counter (slots `0x32c`, `0x328`) and
/// slot `0x330`; plays the block or hit sounds (flag 1 of the hit data
/// chooses between them), casts a weapon effect when its threshold is
/// passed, gives up the weapon for a destroyed one (slot `0x3c8`), clears
/// the damage for a use-weapon package and finally lets `target` take the hit
/// (`Actor::HitMe`) unless this actor's combat controller does not have the
/// target as a combat target.
/// The compiler's exception-unwinding frame is not translated; `HitMe` is
/// called by its address.
pub fn actor_combat_hit(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: Ptr<Actor>,
    flag: u8,
    source_ref: Ptr,
    kind: u8,
) {
    if target.is_null()
        || e.call(ACTOR_TEST_8ACE90, &args![target]).bool()
        || e.call(ACTOR_PROCESS_LEVEL_TEST_437B90, &args![target])
            .bool()
    {
        return;
    }
    let player_address = e.global::<u32>(PLAYER_CHARACTER);
    let mut not_a_target = false;
    if this.addr() != player_address && e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
        let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
        if !e
            .call(
                COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET,
                &args![controller, target],
            )
            .bool()
        {
            not_a_target = true;
        }
    }
    let weapon_entry = if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32()
    } else {
        0
    };
    let weapon_form = if weapon_entry != 0 {
        e.call(ENTRY_FORM, &args![weapon_entry]).u32()
    } else {
        0
    };
    let weapon_type = if weapon_form != 0 {
        e.call(WEAPON_TYPE, &args![weapon_form]).u32()
    } else {
        0
    };
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x104`, `ESI`, the cookie): 0x118
    // bytes, the output byte at +0xeb (`EBP-0x2d`), the hit data `NiPointer` at +0xf4
    // (`EBP-0x24`) and the fresh hit data at +0x7c (`EBP-0x9c`).
    e.with_stack(0x118, |e, frame| {
        let frame = frame.addr();
        let out_flag = frame + 0xeb;
        let connects = e
            .vcall(
                this.addr(),
                ACTOR_SLOT_3D8,
                &args![weapon_entry, target, source_ref, out_flag],
            )
            .u8();
        if e.mem.u8(out_flag) != 0 && e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool() {
            e.call(PLAYER_WORKER_9627A0, &args![player_address]);
        }

        let block = e.call(OPERATOR_NEW, &args![0x64u32]).u32();
        let new_hit_data = if block != 0 {
            e.call(HIT_DATA_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let holder = frame + 0xf4;
        e.call(NI_POINTER_HIT_DATA_CONSTRUCT, &args![holder, new_hit_data]);

        let action = if e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
            && e.call(ENTRY_FORM, &args![VATS_OBJECT]).u32() == 4
        {
            e.call(VATS_GET_CURRENT_ACTION, &args![VATS_OBJECT]).u32()
        } else {
            0
        };
        if action != 0 {
            let saved = e.mem.u32(action + 0x14);
            let hit_data = hit_data_of(e, holder);
            e.call(HIT_DATA_COPY, &args![hit_data, saved]);
            let fresh = frame + 0x7c;
            e.call(HIT_DATA_CONSTRUCT, &args![fresh]);
            e.call(
                HIT_DATA_INITIALIZE,
                &args![fresh, this, target, weapon_entry, flag, source_ref],
            );
            let hit_data = hit_data_of(e, holder);
            for word in 0..3 {
                let value = e.mem.u32(fresh + HIT_DIRECTION + 4 * word);
                e.mem.set_u32(hit_data + HIT_DIRECTION + 4 * word, value);
            }
            let hit_data = hit_data_of(e, holder);
            for word in 0..3 {
                let value = e.mem.u32(fresh + HIT_LOCATION + 4 * word);
                e.mem.set_u32(hit_data + HIT_LOCATION + 4 * word, value);
            }
            if e.mem.u8(action + 4) == 0 {
                for offset in [
                    HIT_HEALTH_DAMAGE,
                    HIT_TOTAL_DAMAGE,
                    HIT_FATIGUE_DAMAGE,
                    HIT_ARMOR_DAMAGE,
                    HIT_TARGETED_LIMB_DAMAGE,
                ] {
                    let hit_data = hit_data_of(e, holder);
                    let value = e.mem.f32(fresh + offset);
                    e.mem.set_f32(hit_data + offset, value);
                }
            }
            if !source_ref.is_null()
                && e.call(REFR_FLAG_TEST, &args![source_ref, REFR_FLAG_NO_CRIT])
                    .bool()
            {
                let hit_data = hit_data_of(e, holder);
                e.mem.set_u32(hit_data + HIT_CRITICAL_EFFECT, 0);
                for offset in [
                    HIT_HEALTH_DAMAGE,
                    HIT_TOTAL_DAMAGE,
                    HIT_FATIGUE_DAMAGE,
                    HIT_DAMAGE_TO_WEAPON,
                    HIT_ARMOR_DAMAGE,
                    HIT_TARGETED_LIMB_DAMAGE,
                ] {
                    let hit_data = hit_data_of(e, holder);
                    e.mem.set_f32(hit_data + offset, 0.0);
                }
                let hit_data = hit_data_of(e, holder);
                e.call(
                    HIT_DATA_CLEAR_EFFECT,
                    &args![hit_data, HIT_DATA_EFFECT_MASK, 0u32],
                );
            }
            let hit_data = hit_data_of(e, holder);
            if e.mem.u32(hit_data + HIT_SOURCE_REF) == 0 && !source_ref.is_null() {
                let hit_data = hit_data_of(e, holder);
                e.mem.set_u32(hit_data + HIT_SOURCE_REF, source_ref.addr());
            }
            let hit_data = hit_data_of(e, holder);
            if !source_ref.is_null() && e.mem.f32(hit_data + HIT_HEALTH_DAMAGE) as f64 != 0.0 {
                let mut has_effect = 0u32;
                if weapon_entry != 0
                    && e.call(
                        ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE,
                        &args![weapon_entry, 0xcu32],
                    )
                    .bool()
                {
                    has_effect = 1;
                }
                // 0089a0f6: `00476c90` takes no stack word; the three pushed words
                // belong to `00525b20` (RET 0xc).
                let weapon_object = e.call(SOURCE_WEAPON_LOOKUP, &args![source_ref]).u32();
                let projectiles = e
                    .call(
                        WEAPON_GET_NUM_PROJECTILES,
                        &args![weapon_object, has_effect, 0u32, this],
                    )
                    .u8() as u32;
                if projectiles as i32 > 1 {
                    for offset in [
                        HIT_HEALTH_DAMAGE,
                        HIT_TOTAL_DAMAGE,
                        HIT_TARGETED_LIMB_DAMAGE,
                    ] {
                        let hit_data = hit_data_of(e, holder);
                        divide_float(e, hit_data + offset, projectiles);
                    }
                }
                let base = e.call(ACTOR_BASE_FORM, &args![source_ref]).u32();
                if e.call(BASE_TEST_4FD3C0, &args![base]).u32() != 0 {
                    let hit_data = hit_data_of(e, holder);
                    let reduction = e.call(REFR_GETTER_885D70, &args![source_ref]).f64();
                    let numerator =
                        (e.mem.f32(hit_data + HIT_HEALTH_DAMAGE) as f64 - reduction) as f32;
                    let hit_data = hit_data_of(e, holder);
                    let ratio = numerator as f64 / e.mem.f32(hit_data + HIT_HEALTH_DAMAGE) as f64;
                    let factor = (1.0 - ratio) as f32;
                    for offset in [
                        HIT_HEALTH_DAMAGE,
                        HIT_TOTAL_DAMAGE,
                        HIT_TARGETED_LIMB_DAMAGE,
                    ] {
                        let hit_data = hit_data_of(e, holder);
                        multiply_float(e, hit_data + offset, factor);
                    }
                }
            }
            hit_data_of(e, holder);
        } else {
            let hit_data = hit_data_of(e, holder);
            e.call(
                HIT_DATA_INITIALIZE,
                &args![hit_data, this, target, weapon_entry, flag, source_ref],
            );
        }

        let counter = e.vcall(this.addr(), ACTOR_SLOT_32C, &args![]).u32();
        e.vcall(this.addr(), ACTOR_SLOT_328, &args![counter.wrapping_add(1)]);
        let hit_data = hit_data_of(e, holder);
        let attack_skill = e.mem.u32(hit_data + HIT_ATTACK_SKILL);
        e.vcall(this.addr(), ACTOR_SLOT_330, &args![attack_skill]);

        let hit_data = hit_data_of(e, holder);
        if e.call(HIT_DATA_FLAG_TEST, &args![hit_data, 1u32]).bool() {
            let hit_data = hit_data_of(e, holder);
            let health_damage = e.mem.f32(hit_data + HIT_HEALTH_DAMAGE);
            let hit_data = hit_data_of(e, holder);
            let percent_blocked = e.mem.f32(hit_data + HIT_PERCENT_BLOCKED);
            e.vcall(
                this.addr(),
                ACTOR_SLOT_4C0,
                &args![health_damage, percent_blocked, target, source_ref],
            );
            let hit_data = hit_data_of(e, holder);
            if e.call(HIT_DATA_FLAG_TEST, &args![hit_data, 2u32]).bool() {
                let target_process = e.get(target, Actor::pCurrentProcess).addr();
                let target_weapon = e
                    .vcall(target_process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                    .u32();
                if target_weapon != 0 {
                    let target_position = position_words(e, target.addr());
                    let this_position = position_words(e, this.addr());
                    let target_process = e.get(target, Actor::pCurrentProcess).addr();
                    let target_weapon = e
                        .vcall(target_process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                        .u32();
                    let target_form = e.call(ENTRY_FORM, &args![target_weapon]).u32();
                    e.call(
                        IMPACT_MIXER_PLAY_WEAPON_BLOCK,
                        &args![
                            weapon_type,
                            target_form,
                            this_position[0],
                            this_position[1],
                            this_position[2],
                            target_position[0],
                            target_position[1],
                            target_position[2],
                            u32::from(connects)
                        ],
                    );
                } else {
                    let target_position = position_words(e, target.addr());
                    let this_position = position_words(e, this.addr());
                    let default_sound = e.global::<u32>(IMPACT_BLOCK_DEFAULT_SOUND);
                    e.call(
                        IMPACT_MIXER_PLAY_WEAPON_BLOCK,
                        &args![
                            weapon_type,
                            default_sound,
                            this_position[0],
                            this_position[1],
                            this_position[2],
                            target_position[0],
                            target_position[1],
                            target_position[2],
                            u32::from(connects)
                        ],
                    );
                }
            } else {
                let target_position = position_words(e, target.addr());
                let this_position = position_words(e, this.addr());
                e.call(
                    IMPACT_MIXER_PLAY_WEAPON_BLOCK,
                    &args![
                        weapon_type,
                        0u32,
                        this_position[0],
                        this_position[1],
                        this_position[2],
                        target_position[0],
                        target_position[1],
                        target_position[2],
                        u32::from(connects == 0)
                    ],
                );
            }
        } else {
            let hit_type = if weapon_form != 0 {
                e.call(WEAPON_TYPE, &args![weapon_form]).u32()
            } else {
                0xffff_ffff
            };
            let hit_data = hit_data_of(e, holder);
            let health_damage = e.mem.f32(hit_data + HIT_HEALTH_DAMAGE);
            e.call(
                IMPACT_MIXER_PLAY_WEAPON_HIT,
                &args![
                    this,
                    health_damage,
                    0.0f32,
                    target,
                    hit_type,
                    0xffff_ffffu32,
                    0xffff_ffffu32,
                    0u32,
                    u32::from(connects)
                ],
            );
        }

        if weapon_form != 0 && weapon_entry != 0 {
            let mut spell = 0u32;
            if spell == 0 {
                let node = e.call(NODE_NEXT, &args![weapon_form + 0x74]).u32();
                spell = if node != 0 { node + 0x18 } else { 0 };
            }
            if spell == 0 {
                let poison = e.call(ITEM_CHANGE_GET_POISON, &args![weapon_entry]).u32();
                spell = if poison != 0 { poison + 0x30 } else { 0 };
            }
            if spell != 0 && connects != 0 && source_ref.is_null() {
                let threshold = e
                    .call(ITEM_CHANGE_GETTER_4BD350, &args![weapon_entry])
                    .f64();
                let value = e.vcall(spell + 0xc, 8, &args![this]).f64();
                if value > threshold {
                    let process = e.get(this, Actor::pCurrentProcess).addr();
                    e.vcall(process, PROCESS_SLOT_580, &args![1u32, 0u32, 0u32]);
                }
            }
        }
        if weapon_entry != 0
            && weapon_form != 0
            && source_ref.is_null()
            && e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
            && !e.call(FORM_FLAG_100_BIT_5, &args![weapon_form]).bool()
        {
            let hit_data = hit_data_of(e, holder);
            let damage_to_weapon = e.mem.f32(hit_data + HIT_DAMAGE_TO_WEAPON);
            // The result clears a local that nothing reads afterwards.
            e.vcall(
                this.addr(),
                ACTOR_SLOT_3C8,
                &args![weapon_entry, damage_to_weapon, 0u32],
            );
        }
        let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
        if package != 0 {
            let data = e
                .call(PACKAGE_GET_USE_WEAPON_PACKAGE_DATA, &args![package])
                .u32();
            if data != 0 && e.call(USE_WEAPON_DATA_TEST, &args![data]).bool() {
                let hit_data = hit_data_of(e, holder);
                e.mem.set_f32(hit_data + HIT_HEALTH_DAMAGE, 0.0);
            }
        }
        if !not_a_target {
            let hit_data = hit_data_of(e, holder);
            e.call(ACTOR_HIT_ME, &args![target, hit_data, kind]);
        }
        e.call(NI_POINTER_HIT_DATA_DESTRUCT, &args![holder]);
    });
}
/// `Actor::GetCurrentEditorPackage` (Xbox PDB).
const ACTOR_GET_CURRENT_EDITOR_PACKAGE: u32 = 0x0088_14b0;
/// `ExtraDataList::GetWeaponAttackSound` (Xbox PDB): thiscall on the extra
/// data list, taking the sound handle to fill.
const EXTRA_GET_WEAPON_ATTACK_SOUND: u32 = 0x0041_8a00;
/// `ExtraDataList::SetWeaponAttackSound` (Xbox PDB).
const EXTRA_SET_WEAPON_ATTACK_SOUND: u32 = 0x0041_a540;
/// `ExtraDataList::GetWeaponIdleSound` (Xbox PDB).
const EXTRA_GET_WEAPON_IDLE_SOUND: u32 = 0x0041_89c0;
/// `ExtraDataList::SetWeaponIdleSound` (Xbox PDB).
const EXTRA_SET_WEAPON_IDLE_SOUND: u32 = 0x0041_a3e0;
/// `ExtraDataList::GetCreatureAwakeSound` (Xbox PDB).
const EXTRA_GET_CREATURE_AWAKE_SOUND: u32 = 0x0041_8940;
/// `ExtraDataList::SetCreatureAwakeSound` (Xbox PDB).
const EXTRA_SET_CREATURE_AWAKE_SOUND: u32 = 0x0041_a090;
/// `BSSoundHandle::IsPlaying` (Xbox PDB).
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
/// `BSSoundHandle::FadeOutAndRelease` (Xbox PDB): the fade time in
/// milliseconds.
const SOUND_HANDLE_FADE_OUT_AND_RELEASE: u32 = 0x00ad_8da0;
/// `BSSoundHandle::Release` (Xbox PDB).
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// `BSSoundHandle::Stop` (Xbox PDB).
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// `BSAudio::GetSoundHandleByNumericID` (Xbox PDB): thiscall on the audio
/// object, taking the output handle, the id and the flags.
const AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID: u32 = 0x00ad_73b0;
/// `BSAudio::GetSoundHandleByName` (Xbox PDB): the output handle, the name
/// and the flags; returns the handle.
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// `"MUSDeath"`.
const DEATH_MUSIC_NAME: u32 = 0x0108_4d00;
/// `MobileObject::StopCurrentDialogue` (Xbox PDB).
const MOBILE_OBJECT_STOP_CURRENT_DIALOGUE: u32 = 0x0093_4250;
/// `bhkRigidBody::SetFixed` (Xbox PDB), cdecl: the body and two flags.
const BHK_RIGID_BODY_SET_FIXED: u32 = 0x00c8_f210;
/// `bhkRagdollController::DisableRagdollAnim` (Xbox PDB): thiscall on the
/// controller, taking a flag.
const BHK_RAGDOLL_DISABLE_RAGDOLL_ANIM: u32 = 0x00c7_c150;
/// Pointer to the byte value of a setting (`00408d60`, `this` = the
/// setting).
const SETTING_GET_BOOL_POINTER: u32 = 0x0040_8d60;
/// The `bool` setting that, with `GetEssential`, keeps an actor from dying.
const ESSENTIAL_SETTING: u32 = 0x011e_0888;
/// The `float` setting `Kill` multiplies the base health by for the actors
/// of process level 3 and 5.
const LOW_LEVEL_HEALTH_SETTING: u32 = 0x011d_0144;
/// The `float` setting passed to process slot `0xe8` for an essential
/// actor.
const ESSENTIAL_DOWN_TIMER_SETTING: u32 = 0x011d_0700;
/// `Actor::ClearInCombat` (Xbox PDB): takes a flag.
const ACTOR_CLEAR_IN_COMBAT: u32 = 0x008a_08e0;
/// `Actor::SetHavokWeapon` (Xbox PDB).
const ACTOR_SET_HAVOK_WEAPON: u32 = 0x008a_5eb0;
/// `Actor::RestoreFullHealthAndConditions` (Xbox PDB).
const ACTOR_RESTORE_FULL_HEALTH_AND_CONDITIONS: u32 = 0x008a_0960;
/// `MagicCaster::InterruptCast` (Xbox PDB), `this` = the caster at `+0x88`.
const MAGIC_CASTER_INTERRUPT_CAST: u32 = 0x0081_5b00;
/// `TESObjectREFR::SetMarkerUsed` (Xbox PDB): thiscall on the marker with
/// the index and a flag.
const REFR_SET_MARKER_USED: u32 = 0x0056_8020;
/// The CRT's `sprintf` (`00ec623a`), cdecl.
const SPRINTF: u32 = 0x00ec_623a;
/// `"%s %s"`.
const TWO_STRINGS_FORMAT: u32 = 0x0101_2058;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_surprised.dds"`.
const MESSAGE_ICON_SURPRISED: u32 = 0x0104_9638;
/// The setting object whose string follows the actor's name in the message
/// shown for an essential actor that goes down.
const DOWN_MESSAGE_SETTING: u32 = 0x011d_1e24;
/// String of a setting (`00403df0`, `this` = the setting).
const SETTING_GET_STRING: u32 = 0x0040_3df0;
/// `ProcessLists` worker taking the actor (`0096e2f0`).
const PROCESS_LISTS_WORKER_96E2F0: u32 = 0x0096_e2f0;
/// `TESObjectREFR::IsPartofEvilFaction` (Xbox PDB).
const REFR_IS_PART_OF_EVIL_FACTION: u32 = 0x0056_78a0;
/// `PlayerCharacter::SetIsAMurderer` (Xbox PDB).
const PLAYER_SET_IS_A_MURDERER: u32 = 0x0095_3f80;
/// `TESActorBaseData::GetAlignmentForKarma` (Xbox PDB), cdecl, a `float`.
const GET_ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
/// `Actor::HasFactionThatCaresAboutCrime` (Xbox PDB).
const ACTOR_HAS_FACTION_THAT_CARES_ABOUT_CRIME: u32 = 0x008b_8490;
/// `PlayerCharacter::RewardKarma` (Xbox PDB): thiscall on the player.
const PLAYER_REWARD_KARMA: u32 = 0x0094_fd30;
/// `float` settings of the karma reward: the default, the one for
/// creatures and the ones per alignment (indexed by the alignment number;
/// entry 1 is the default).
const KARMA_REWARD_SETTING: u32 = 0x011c_df6c;
const KARMA_REWARD_CREATURE_SETTING: u32 = 0x011c_d164;
const KARMA_REWARD_ALIGNMENT_SETTINGS: [u32; 5] = [
    0x011c_d7a4,
    KARMA_REWARD_SETTING,
    0x011c_ded4,
    0x011c_dca4,
    0x011c_d508,
];
/// Test on the player (`004997b0`).
const PLAYER_TEST_4997B0: u32 = 0x0049_97b0;
/// `PlayerCharacter::IsPlayerDetectedByNonTeammates` (Xbox PDB): thiscall on
/// the player, taking an actor.
const PLAYER_IS_DETECTED_BY_NON_TEAMMATES: u32 = 0x0096_7160;
/// Test on a weapon form (`006450c0`).
const WEAPON_FORM_TEST_6450C0: u32 = 0x0064_50c0;
/// `00525620`: thiscall on a weapon form, taking two flags.
const WEAPON_FORM_NUMBER: u32 = 0x0052_5620;
/// The integer setting that number is compared with.
const WEAPON_NUMBER_SETTING: u32 = 0x011c_d3b4;
/// Test of the tracked damage against the health (`00670700`, cdecl, two
/// `float`s).
const TRACKED_DAMAGE_TEST: u32 = 0x0067_0700;
/// `0087f9f0`: thiscall on the actor, a 16-bit result.
const ACTOR_GETTER_87F9F0: u32 = 0x0087_f9f0;
/// `ExperiencePoints::GetExperiencePoints` (Xbox PDB), cdecl: a flag and a
/// 16-bit number; returns an integer.
const GET_EXPERIENCE_POINTS: u32 = 0x0067_05b0;
/// `005be4d0`: thiscall on the player, returns an integer.
const PLAYER_INT_5BE4D0: u32 = 0x005b_e4d0;
/// `00648c80`: cdecl, a `float` and an integer; returns a `float`.
const SCALE_EXPERIENCE: u32 = 0x0064_8c80;
/// `00406ce0`: cdecl, a `float`; returns a `float`.
const FLOAT_ROUND: u32 = 0x0040_6ce0;
/// Player virtual slot `0x488` (takes the amount).
const PLAYER_SLOT_488: u32 = 0x488;
/// `Actor` virtual slot `0x1a0` (takes a flag).
const ACTOR_SLOT_1A0: u32 = 0x1a0;
/// `Actor` virtual slot `0x2a0`.
const ACTOR_SLOT_2A0: u32 = 0x2a0;
/// `Actor::GetHeading` (Xbox PDB), `Actor` virtual slot `0x2bc` (takes a
/// flag; the result is a `float`).
const ACTOR_SLOT_GET_HEADING: u32 = 0x2bc;
/// `Actor` virtual slot `0x38c` (a bool).
const ACTOR_SLOT_38C: u32 = 0x38c;
/// `Actor` virtual slot `0x448` (a bool).
const ACTOR_SLOT_448: u32 = 0x448;
/// Process virtual slots used by `Kill`: `GetIsAggressor` (`0x1c4`),
/// `ClearMuzzleFlash` (`0x6c4`), `RemoveAllQueuedItems` (`0x180`),
/// `SetGreetingFlag` (`0x310`), `GetCurrentFurniture` (`0x4c8`),
/// `GetCurrentFurnitureIndex` (`0x4d0`), `SetBreathTimer` (`0x2fc`),
/// `GetTrackedDamage` (`0x730`), `ClearAllHeadTrackTargets` (`0x660`),
/// `SetDeathTime` (`0x6a0`), `SetGreetingTimer` (`0x4b4`) and
/// `GetKnockState` (`0x40c`) (Xbox PDB names of the `HighProcess` vtable).
const PROCESS_SLOT_1C4: u32 = 0x1c4;
const PROCESS_SLOT_6C4: u32 = 0x6c4;
const PROCESS_SLOT_180: u32 = 0x180;
const PROCESS_SLOT_310: u32 = 0x310;
const PROCESS_SLOT_4C8: u32 = 0x4c8;
const PROCESS_SLOT_4D0: u32 = 0x4d0;
const PROCESS_SLOT_2FC: u32 = 0x2fc;
const PROCESS_SLOT_730: u32 = 0x730;
const PROCESS_SLOT_660: u32 = 0x660;
const PROCESS_SLOT_6A0: u32 = 0x6a0;
const PROCESS_SLOT_4B4: u32 = 0x4b4;
const PROCESS_SLOT_40C: u32 = 0x40c;
/// Static byte read (`00525420`: a global flag).
const GLOBAL_FLAG_525420: u32 = 0x0052_5420;
/// `TESCreature::PickCreatureSound` (Xbox PDB): thiscall on the creature,
/// taking the sound type.
const CREATURE_PICK_CREATURE_SOUND: u32 = 0x005f_92d0;
/// The numeric id of a sound form (`0084e3a0`; also used for the ids in the
/// death report).
const FORM_NUMERIC_ID: u32 = 0x0084_e3a0;
/// `FalloutRadio::PipboyRadioEnable` (Xbox PDB), cdecl.
const PIPBOY_RADIO_ENABLE: u32 = 0x0083_24e0;
/// `00830680`: cdecl, `1000, 0`.
const SOUND_FADE_WORKER: u32 = 0x0083_0680;
/// `VATS::QuitVATSPlayback` (Xbox PDB): thiscall on `VATS`, two flags.
const VATS_QUIT_PLAYBACK: u32 = 0x009c_8950;
/// `Interface::IsTopMenuID` (Xbox PDB), cdecl.
const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `SurgeryMenu::Close` (Xbox PDB).
const SURGERY_MENU_CLOSE: u32 = 0x007e_1980;
/// `Actor::GetFaceAnimationData` (Xbox PDB).
const ACTOR_GET_FACE_ANIMATION_DATA: u32 = 0x008a_dcb0;
/// `009ca1f0`: thiscall on `VATS`.
const VATS_WORKER_9CA1F0: u32 = 0x009c_a1f0;
/// `0080ced0`: cdecl, the actor and a number; returns a byte.
const ACTOR_GETTER_80CED0: u32 = 0x0080_ced0;
/// `00867e30`: thiscall on `TIME_STAMP_OBJECT`, an unsigned result.
const TIME_STAMP_867E30: u32 = 0x0086_7e30;
const TIME_STAMP_OBJECT: u32 = 0x011d_e7b8;
/// `00459060`: thiscall on `TES`, taking a form and `1`.
const TES_WORKER_459060: u32 = 0x0045_9060;
/// `ExtraDataList::RemoveTrespassPackage` (Xbox PDB).
const EXTRA_REMOVE_TRESPASS_PACKAGE: u32 = 0x0041_cda0;
/// `0043fcd0`: the collision object of the actor (the base function of
/// `Actor` slot `0x1d0`).
const ACTOR_COLLISION_OBJECT: u32 = 0x0043_fcd0;
/// Four-`float` constructor (`005532a0`).
const VECTOR4_CONSTRUCT: u32 = 0x0055_32a0;
/// `bhkCharacterProxy::GetLinearVelocity` (Xbox PDB): thiscall, takes the
/// output vector.
const BHK_CHARACTER_PROXY_GET_LINEAR_VELOCITY: u32 = 0x0066_ca00;
/// Vector assignment (`004a3e00`, cdecl: destination, source).
const VECTOR_ASSIGN: u32 = 0x004a_3e00;
/// `MobileObject::SetChaseBip` (Xbox PDB): takes a flag.
const MOBILE_OBJECT_SET_CHASE_BIP: u32 = 0x0093_1fb0;
/// The `float` setting passed to process slot `0x4b4`.
const GREETING_TIMER_SETTING: u32 = 0x011c_cf7c;
/// Test on the actor (`0087eeb0`).
const ACTOR_TEST_87EEB0: u32 = 0x0087_eeb0;
/// `Actor` helper (`0087eef0`, thiscall on the actor).
const ACTOR_WORKER_87EEF0: u32 = 0x0087_eef0;
/// The three `float`s at `0x11f426c`: the default impulse direction.
const DEFAULT_DIRECTION: u32 = 0x011f_426c;
/// `00630b40`: thiscall on a vector, taking an output; returns an object.
const VECTOR_TRANSFORM_630B40: u32 = 0x0063_0b40;
/// `004586d0`: thiscall on that object; the result is in ST0.
const VECTOR_LENGTH_4586D0: u32 = 0x0045_86d0;
/// `00458620`: cdecl, output vector and input vector.
const VECTOR_COPY_458620: u32 = 0x0045_8620;
/// `004a0c90`: thiscall on a vector, taking a `float`.
const VECTOR_ROTATE_4A0C90: u32 = 0x004a_0c90;
/// `004b4500`: thiscall on a vector, taking an output and a vector;
/// returns the output.
const VECTOR_MULTIPLY_4B4500: u32 = 0x004b_4500;
/// A constant vector at `0x11a9478`.
const UNIT_VECTOR: u32 = 0x011a_9478;
/// `bhkCharacterController::GetPosition` (Xbox PDB, `00812b00`): thiscall,
/// takes the output vector.
const BHK_CHARACTER_CONTROLLER_GET_POSITION: u32 = 0x0081_2b00;
/// Three-`float` constructors (`0043d410` and `00416870`).
const VECTOR3_CONSTRUCT_43D410: u32 = 0x0043_d410;
const VECTOR3_CONSTRUCT_416870: u32 = 0x0041_6870;
/// `00a59c60`: thiscall on the collision object, taking a vector.
const COLLISION_OBJECT_SET_ANGULAR_VELOCITY: u32 = 0x00a5_9c60;
/// `00439180`: thiscall on a vector, taking a scalar.
const VECTOR_SCALE_IN_PLACE: u32 = 0x0043_9180;
/// The scalar setting multiplying the impulse (`0x11cf724`).
const DEATH_IMPULSE_SETTING: u32 = 0x011c_f724;
/// `TESHavokUtilities::AddVelocity` (Xbox PDB), cdecl: body, vector, flag.
const TES_HAVOK_ADD_VELOCITY: u32 = 0x0062_b8d0;
/// `0062b930`: cdecl, body, vector, flag.
const TES_HAVOK_SET_VELOCITY: u32 = 0x0062_b930;
/// The `float` used as the upward component (`0x101712c`).
const UPWARD_COMPONENT: u32 = 0x0101_712c;
/// `CombatManager::GetCombatantCount` (Xbox PDB): thiscall on the manager,
/// taking an actor and a pointer.
const COMBAT_MANAGER_GET_COMBATANT_COUNT: u32 = 0x0099_31c0;
/// Integer settings: the combatant count limit and the drop chance.
const COMBATANT_COUNT_SETTING: u32 = 0x011d_f780;
const DROP_WEAPON_CHANCE_SETTING: u32 = 0x011d_055c;
/// Random number (`00487f50`).
const RANDOM_NUMBER: u32 = 0x0048_7f50;
/// `Actor::DoDeathStuff` (Xbox PDB).
const ACTOR_DO_DEATH_STUFF: u32 = 0x008b_01c0;
/// `TESObjectREFR::RunScript` (Xbox PDB).
const REFR_RUN_SCRIPT: u32 = 0x0056_5870;
/// `bhkRagdollPenetrationUtil::Clear` (Xbox PDB).
const BHK_RAGDOLL_PENETRATION_UTIL_CLEAR: u32 = 0x00ca_20e0;
/// `00931ed0`: thiscall on the actor, taking an output; returns an object.
const ACTOR_OUTPUT_931ED0: u32 = 0x0093_1ed0;
/// `004a3a20` (thiscall).
const OBJECT_ACCESSOR_4A3A20: u32 = 0x004a_3a20;
/// `00ca2ad0`: thiscall on the penetration utility, taking the collision
/// object and a value.
const PENETRATION_UTIL_WORKER: u32 = 0x00ca_2ad0;
/// The `bool` setting that enables the death report.
const DEATH_REPORT_SETTING: u32 = 0x011d_f690;
/// `"'%s' (%08X) was killed by '%s' (%08X)."`.
const KILLED_BY_FORMAT: u32 = 0x0108_4cc8;
/// `"'%s' (%08X) has died with no attacker."`.
const DIED_WITH_NO_ATTACKER_FORMAT: u32 = 0x0108_4ca0;
/// `"%.20s is dead!"`.
const IS_DEAD_FORMAT: u32 = 0x0108_4cf0;
/// `MapMarkerData::GetLocationName` (Xbox PDB), used on the base form's
/// name member (`+0xd0`).
const LOCATION_NAME: u32 = 0x0040_8da0;
/// `Error` (Xbox PDB), cdecl, does nothing.
const ERROR_REPORT: u32 = 0x0040_fbe0;
/// Test on a setting object (`00454af0`).
const SETTING_TEST_454AF0: u32 = 0x0045_4af0;
const EXPORT_SETTING: u32 = 0x011e_0a30;
/// `PlayerCharacter::ExportProgressData` (Xbox PDB).
const PLAYER_EXPORT_PROGRESS_DATA: u32 = 0x0094_ed40;
/// `Actor::AddFactionMinorCrime` (Xbox PDB).
const ACTOR_ADD_FACTION_MINOR_CRIME: u32 = 0x008b_7c00;
/// `PlayerCharacter::SetSlowMoCamera` (Xbox PDB).
const PLAYER_SET_SLOW_MO_CAMERA: u32 = 0x0093_e530;
const SLOW_MO_SETTING: u32 = 0x011d_00fc;
/// `ProcessLists::FinishMagicShaderHitEffect` (Xbox PDB).
const PROCESS_LISTS_FINISH_MAGIC_SHADER_HIT_EFFECT: u32 = 0x0097_4a50;
/// `ContinuousBeamProjectile::SetKill` (Xbox PDB).
const CONTINUOUS_BEAM_PROJECTILE_SET_KILL: u32 = 0x009a_b9a0;
/// The pointer that is the `this` of `fn_0089f4e0`.
const MAGIC_SHADER_HOLDER: u32 = 0x011c_3f2c;
/// Test on a form (`0056af40`).
const ACTOR_TEST_56AF40: u32 = 0x0056_af40;
/// `ExtraDataList` accessor (`00421720`).
const EXTRA_GET_BASE_FORM_421720: u32 = 0x0042_1720;
/// `TESCreature` accessor (`0059f3a0`): a signed byte.
const CREATURE_GETTER_59F3A0: u32 = 0x0059_f3a0;
/// RTTI type descriptor of `TESActorBase` and of `TESCreature`.
const RTTI_TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
const RTTI_TYPE_TES_CREATURE: u32 = 0x0118_3a00;
/// Statistics worker (`005f5950`, cdecl, 7 words).
const STATISTICS_WORKER_5F5950: u32 = 0x005f_5950;
/// Weapon-form accessor returning a 16-bit value (`008d85e0`).
const WEAPON_GETTER_8D85E0: u32 = 0x008d_85e0;
/// The `float` setting (distance) that decides whether a dying actor speaks.
const DEATH_VOICE_DISTANCE_SETTING: u32 = 0x011d_0f9c;
/// `Actor::GetAnimGroup` (Xbox PDB), called by its address.
const ACTOR_GET_ANIM_GROUP_ADDRESS: u32 = 0x0089_7910;

/// Fetches one of the actor's remembered sounds from its extra data list
/// into `handle`, stops it (fading it out first while it plays, when
/// `fade_out`; otherwise `Stop` then `Release`) and stores the handle back.
fn forget_actor_sound(
    e: &mut Engine,
    this: Ptr<Actor>,
    handle: u32,
    getter: u32,
    setter: u32,
    fade_out: bool,
) {
    e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
    let extra = extra_data_list_of(e, this.cast());
    e.call(getter, &args![extra, handle]);
    if fade_out {
        if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool() {
            e.call(SOUND_HANDLE_FADE_OUT_AND_RELEASE, &args![handle, 500u32]);
        } else {
            e.call(SOUND_HANDLE_RELEASE, &args![handle]);
        }
    } else {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    let extra = extra_data_list_of(e, this.cast());
    e.call(setter, &args![extra, handle]);
}

/// Destroys the three sound handles `actor_kill` keeps (in the order the
/// compiler's cleanup does) and releases their memory.
/// `handles` is the awake handle (`EBP-0x3c` of `0089d900`); the idle and the attack handles
/// follow at +12 and +24 (`EBP-0x30`, `EBP-0x24`).
fn destroy_kill_sound_handles(e: &mut Engine, handles: u32) {
    e.call(SOUND_HANDLE_DESTROY, &args![handles]);
    e.call(SOUND_HANDLE_DESTROY, &args![handles + 12]);
    e.call(SOUND_HANDLE_DESTROY, &args![handles + 24]);
}

/// Calls `TESObjectREFR::SetMarkerUsed` on the furniture the process
/// reports (`GetCurrentFurniture`) with its index, when it reports one.
fn release_furniture_marker(e: &mut Engine, this: Ptr<Actor>) {
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if e.vcall(process, PROCESS_SLOT_4C8, &args![]).u32() != 0 {
            let index_source = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let furniture_source = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let index = e.vcall(index_source, PROCESS_SLOT_4D0, &args![]).u32();
            let furniture = e.vcall(furniture_source, PROCESS_SLOT_4C8, &args![]).u32();
            e.call(REFR_SET_MARKER_USED, &args![furniture, index, 0u32]);
        }
    }
}

/// `*(setting)` as a `float`, for a `float` setting object.
fn float_setting(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_GET_VALUE_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

/// `*(setting)` as an integer, for an integer setting object.
fn int_setting(e: &mut Engine, setting: u32) -> u32 {
    let pointer = e.call(SETTING_GET_INT_POINTER, &args![setting]).u32();
    e.mem.u32(pointer)
}

/// The kill statistics `Actor::Kill` records when the attacker belongs to
/// the player: the entry point `0x15`, and the statistics worker three
/// times with the weapon, the creature type and the weapon's numbers; then
/// the `VATS` counter.
fn record_kill_statistics(e: &mut Engine, this: Ptr<Actor>, attacker: Ptr<Actor>, player: u32) {
    e.call(HANDLE_ENTRY_POINT, &args![0x15u32, player, this, this]);
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
    let weapon_entry = e
        .vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32();
    let weapon_form = if weapon_entry != 0 {
        e.call(ENTRY_FORM, &args![weapon_entry]).u32()
    } else {
        0
    };
    let mut base = base_form(e, this.cast()).addr();
    if e.call(ACTOR_TEST_56AF40, &args![this]).bool() {
        let extra = extra_data_list_of(e, this.cast());
        base = e.call(EXTRA_GET_BASE_FORM_421720, &args![extra]).u32();
    }
    let creature_type = if e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool() {
        0u8
    } else {
        let actor_base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
        let creature = e
            .call(
                RTTI_DYNAMIC_CAST,
                &args![
                    actor_base,
                    0u32,
                    RTTI_TYPE_TES_ACTOR_BASE,
                    RTTI_TYPE_TES_CREATURE,
                    0u32
                ],
            )
            .u32();
        let kind = e.call(CREATURE_GETTER_59F3A0, &args![creature]).u8() as i8;
        (kind as i32 + 1) as u8
    };
    if !e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
        for (first, second) in [(1u32, 1u32), (0, 1)] {
            let number = weapon_number(e, weapon_form);
            let slot = weapon_slot(e, weapon_form);
            e.call(
                STATISTICS_WORKER_5F5950,
                &args![first, second, base, weapon_form, 0u32, slot, number],
            );
        }
        let number = weapon_number(e, weapon_form);
        e.call(
            STATISTICS_WORKER_5F5950,
            &args![
                2u32,
                1u32,
                0u32,
                weapon_form,
                0u32,
                u32::from(creature_type as i8 as i16 as u16),
                number
            ],
        );
    }
    if e.call(ENTRY_FORM, &args![VATS_OBJECT]).u32() == 4 {
        fn_0089f560(e, Ptr::new(VATS_OBJECT));
    }
}

/// `(weapon type + 1) & 0xffff` of a weapon form, 1 when there is none.
fn weapon_number(e: &mut Engine, weapon_form: u32) -> u32 {
    if weapon_form != 0 {
        u32::from((e.call(WEAPON_TYPE, &args![weapon_form]).u16()).wrapping_add(1))
    } else {
        1
    }
}

/// The 16-bit slot number of a weapon form (`008d85e0`), `0x2d` when none.
fn weapon_slot(e: &mut Engine, weapon_form: u32) -> u32 {
    if weapon_form != 0 {
        u32::from(e.call(WEAPON_GETTER_8D85E0, &args![weapon_form]).u16())
    } else {
        0x2d
    }
}

/// The part of `Actor::Kill` for an essential actor (the essential setting
/// is on and `GetEssential` is true): it does not die. At process levels 3
/// and 5 its health is set to the setting's share of the base health;
/// otherwise its crime list, timers and package are reset, it leaves
/// combat, releases its furniture, goes to life state 6 with full health
/// and, at process level 6, shows a message. Returns true when the whole
/// kill ends here (the first case, which also destroys the sound handles).
fn kill_essential_actor(e: &mut Engine, ebp: u32, this: Ptr<Actor>, handles: u32) -> bool {
    if e.call(ACTOR_PROCESS_LEVEL_IS_3, &args![this]).bool()
        || e.call(ACTOR_PROCESS_LEVEL_IS_5, &args![this]).bool()
    {
        let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
        let base_health = e.vcall(owner, 0x00, &args![0x10u32]).i32() as f32;
        let share = float_setting(e, LOW_LEVEL_HEALTH_SETTING);
        let mut amount = (share as f64 * base_health as f64) as f32;
        let current = e.vcall(owner, 0x0c, &args![0x10u32]).f32();
        amount = (amount as f64 - current as f64) as f32;
        e.vcall(
            this.addr(),
            ACTOR_SLOT_MODIFY_ACTOR_VALUE,
            &args![0x10u32, amount, 0u32],
        );
        destroy_kill_sound_handles(e, handles);
        return true;
    }
    let package = e.call(ACTOR_GET_CURRENT_EDITOR_PACKAGE, &args![this]).u32();
    let extra = extra_data_list_of(e, this.cast());
    e.call(EXTRA_REMOVE_PLAYER_CRIME_LIST_EXTRA, &args![extra, this]);
    let timer = float_setting(e, ESSENTIAL_DOWN_TIMER_SETTING);
    let process = e.get(this, Actor::pCurrentProcess).addr();
    e.vcall(process, PROCESS_SLOT_E8, &args![timer]);
    let twenty: f32 = e.global(TWENTY);
    let process = e.get(this, Actor::pCurrentProcess).addr();
    e.vcall(process, PROCESS_SLOT_2FC, &args![twenty]);
    if package != 0 && e.call(PACKAGE_TYPE, &args![package]).i32() != 0xf {
        e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![this, 0u32]);
    }
    e.call(ACTOR_CLEAR_IN_COMBAT, &args![this, 1u32]);
    release_furniture_marker(e, this);
    e.call(MAGIC_CASTER_INTERRUPT_CAST, &args![this.byte_add(0x88)]);
    e.call(ACTOR_SET_LIFE_STATE, &args![this, 6u32]);
    e.call(ACTOR_SET_HAVOK_WEAPON, &args![this]);
    e.call(ACTOR_RESTORE_FULL_HEALTH_AND_CONDITIONS, &args![this]);
    if e.call(PROCESS_LEVEL_NUMBER, &args![this]).i32() == 6 {
        let text = e
            .call(SETTING_GET_STRING, &args![DOWN_MESSAGE_SETTING])
            .u32();
        let name = e.call(REFR_GET_NAME, &args![this]).u32();
        let buffer = ebp - 0x170;
        e.call(SPRINTF, &args![buffer, TWO_STRINGS_FORMAT, name, text]);
        let duration: f32 = e.global(MESSAGE_DURATION);
        e.call(
            SHOW_MESSAGE,
            &args![buffer, 0u32, MESSAGE_ICON_SURPRISED, 0u32, duration, 0u32],
        );
    }
    false
}

/// Reports the death through `Error` (which does nothing) with the two
/// actors' names and ids.
fn report_death(e: &mut Engine, this: Ptr<Actor>, attacker: Ptr<Actor>) {
    if !attacker.is_null() {
        let attacker_id = e.call(FORM_NUMERIC_ID, &args![attacker]).u32();
        let base = e.call(ACTOR_BASE_FORM, &args![attacker]).u32();
        let attacker_name = e.call(LOCATION_NAME, &args![base + 0xd0]).u32();
        let id = e.call(FORM_NUMERIC_ID, &args![this]).u32();
        let base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
        let name = e.call(LOCATION_NAME, &args![base + 0xd0]).u32();
        e.call(
            ERROR_REPORT,
            &args![KILLED_BY_FORMAT, name, id, attacker_name, attacker_id],
        );
    } else {
        let id = e.call(FORM_NUMERIC_ID, &args![this]).u32();
        let base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
        let name = e.call(LOCATION_NAME, &args![base + 0xd0]).u32();
        e.call(ERROR_REPORT, &args![DIED_WITH_NO_ATTACKER_FORMAT, name, id]);
    }
}

/// The part of `Actor::Kill` that updates the combat bookkeeping for the
/// player or a teammate of the player: crime for a combatant that sees the
/// player, and the slow-motion camera when nothing else is fighting.
fn update_kill_counts(
    e: &mut Engine,
    ebp: u32,
    this: Ptr<Actor>,
    attacker: Ptr<Actor>,
    player: u32,
) {
    // 0089f35e: the count word at `EBP-0x34c` (and the result at `EBP-0x350`) start at 0.
    let counter = ebp - 0x34c;
    e.mem.set_u32(counter, 0);
    if attacker.addr() == player
        || (!attacker.is_null()
            && e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![attacker])
                .bool())
    {
        let manager = e.global::<u32>(COMBAT_MANAGER);
        let count = e
            .call(
                COMBAT_MANAGER_GET_COMBATANT_COUNT,
                &args![manager, player, counter],
            )
            .u32();
        if count > 0
            && e.call(PLAYER_IS_DETECTED_BY_NON_TEAMMATES, &args![player, this])
                .bool()
        {
            e.call(ACTOR_ADD_FACTION_MINOR_CRIME, &args![this, 1u32, 1u32]);
        }
        if e.call(ENTRY_FORM, &args![VATS_OBJECT]).u32() == 0 && count == 0 {
            let factor = float_setting(e, SLOW_MO_SETTING);
            e.call(
                PLAYER_SET_SLOW_MO_CAMERA,
                &args![player, this, factor, 1u32, 0xffff_ffffu32],
            );
        }
    }
}

/// The killing part of `Actor::Kill` (the actor is not essential): clears
/// the actor out of the process lists, settles who gets the blame
/// (`attacker` may be replaced by its commanding actor), does the crime,
/// karma and experience bookkeeping for the player, releases furniture,
/// plays the death dialogue or creature sound, sets the life state to 1,
/// handles the player's own death, gives the body a ragdoll impulse and
/// finishes with `Actor::DoDeathStuff` and the debug line.
fn kill_actor(
    e: &mut Engine,
    ebp: u32,
    this: Ptr<Actor>,
    attacker: &mut Ptr<Actor>,
    was_aggressor: bool,
    player: u32,
) {
    e.call(PROCESS_LISTS_WORKER_96E2F0, &args![PROCESS_LISTS, this]);
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(process, PROCESS_SLOT_310, &args![0u32]);
    }
    if !attacker.is_null() && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![*attacker]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![*attacker]).u32();
        if e.vcall(process, PROCESS_SLOT_52C, &args![]).u32() != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![*attacker]).u32();
            *attacker = Ptr::new(e.vcall(process, PROCESS_SLOT_52C, &args![]).u32());
        }
    }
    if !attacker.is_null() && e.vcall(attacker.addr(), ACTOR_SLOT_218, &args![]).bool() {
        let mut blamed = true;
        if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
            let controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
            blamed = e
                .call(
                    COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET,
                    &args![controller, *attacker],
                )
                .bool();
        }
        if blamed {
            e.mem.set_u8(this.addr() + 0xc4, u8::from(!was_aggressor));
            if attacker.addr() == player {
                let flag = ebp - 0x17d;
                e.mem.set_u8(flag, 0);
                let reaction = e
                    .call(ACTOR_GET_FACTION_FIGHT_REACTION, &args![this, player, flag])
                    .i32();
                if (reaction == 3 || reaction == 2)
                    && e.call(ACTOR_TEST_5A3790, &args![this]).bool()
                {
                    e.mem.set_u8(this.addr() + 0xc4, 0);
                }
            }
        }
    }
    if attacker.addr() == player && !e.vcall(this.addr(), ACTOR_SLOT_2E8, &args![]).bool() {
        if e.vcall(this.addr(), ACTOR_SLOT_218, &args![]).bool() {
            e.call(MISC_STAT_INCREMENT, &args![2u32]);
            if !e.call(REFR_IS_PART_OF_EVIL_FACTION, &args![this]).bool()
                && e.mem.u8(this.addr() + 0xc4) != 0
            {
                e.call(PLAYER_SET_IS_A_MURDERER, &args![player]);
            }
        } else {
            e.call(MISC_STAT_INCREMENT, &args![3u32]);
        }
        e.call(MISC_STAT_INCREMENT, &args![0x23u32]);
        e.call(ACTOR_BASE_FORM, &args![this]);
        let base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
        let karma = e.vcall(base + 0x100, 0x0c, &args![0x17u32]).f32();
        let alignment = e.call(GET_ALIGNMENT_FOR_KARMA, &args![karma]).u32();
        if !was_aggressor
            && e.call(ACTOR_HAS_FACTION_THAT_CARES_ABOUT_CRIME, &args![this])
                .bool()
        {
            let mut reward = float_setting(e, KARMA_REWARD_SETTING);
            if e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool() {
                reward = float_setting(e, KARMA_REWARD_CREATURE_SETTING);
            }
            if alignment != 1 && alignment <= 4 {
                reward = float_setting(e, KARMA_REWARD_ALIGNMENT_SETTINGS[alignment as usize]);
            }
            if reward as f64 != 0.0 {
                let amount = e.call(FTOL, &args![reward as f64]).u32();
                e.call(PLAYER_REWARD_KARMA, &args![player, amount]);
            }
        }
    }
    if attacker.addr() == player
        && e.call(PLAYER_TEST_4997B0, &args![player]).bool()
        && !e
            .call(PLAYER_IS_DETECTED_BY_NON_TEAMMATES, &args![player, 0u32])
            .bool()
    {
        let mut weapon_form = 0u32;
        if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32() != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
            if e.vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                .u32()
                != 0
            {
                let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
                let entry = e
                    .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                    .u32();
                weapon_form = e.call(ENTRY_FORM, &args![entry]).u32();
            }
        }
        let mut clear = true;
        if weapon_form != 0 && !e.call(WEAPON_FORM_TEST_6450C0, &args![weapon_form]).bool() {
            let number = e
                .call(WEAPON_FORM_NUMBER, &args![weapon_form, 0u32, 0u32])
                .u32();
            if number != int_setting(e, WEAPON_NUMBER_SETTING) {
                let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
                let entry = e
                    .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                    .u32();
                if !e
                    .call(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, &args![entry, 0xbu32])
                    .bool()
                {
                    clear = false;
                }
            }
        }
        if clear {
            e.mem.set_u8(this.addr() + 0xc4, 0);
        }
    }
    award_experience(e, this, *attacker, player);
    release_furniture_marker(e, this);
    if this.addr() != player {
        e.call(ACTOR_WORKER_87EEF0, &args![this]);
        e.set(this, Actor::pMyKiller, attacker.cast());
    }
    let group = e
        .call(
            ACTOR_GET_ANIM_GROUP_ADDRESS,
            &args![this, 0xe0u32, 0u32, 0u32, 0u32],
        )
        .u16();
    if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).i32() == 0xe0 {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        e.vcall(process, PROCESS_SLOT_614, &args![0x20u32]);
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(process, PROCESS_SLOT_660, &args![]);
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0 {
            play_death_voice(e, ebp, this, *attacker, player);
        }
    }
    e.call(ACTOR_SET_LIFE_STATE, &args![this, 1u32]);
    e.call(ACTOR_SET_HAVOK_WEAPON, &args![this]);
    if this.addr() == player {
        player_dies(e, ebp, this);
    }
    finish_dying(e, ebp, this, *attacker, player);
    if e.call(DEBUG_MESSAGES_ENABLED, &args![]).bool() {
        let name = e.call(REFR_GET_NAME, &args![this]).u32();
        e.call(PRINT_DEBUG_LINE, &args![IS_DEAD_FORMAT, name]);
    }
}

/// The experience the player gets for the kill: only when the actor was
/// not a teammate's kill (or the attacker is a teammate of the player), is
/// neither a protected nor an unfriendly actor, and the damage tracked on
/// it passes the test against its health.
fn award_experience(e: &mut Engine, this: Ptr<Actor>, attacker: Ptr<Actor>, player: u32) {
    let mut by_player_side = true;
    if e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
        by_player_side =
            !attacker.is_null() && e.vcall(attacker.addr(), ACTOR_SLOT_360, &args![]).bool();
    }
    if e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        || e.vcall(this.addr(), ACTOR_SLOT_2E8, &args![]).bool()
        || !by_player_side
    {
        return;
    }
    let tracked = if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(process, PROCESS_SLOT_730, &args![]).f32()
    } else {
        0.0
    };
    if (tracked as f64) <= 0.0 || tracked.is_nan() {
        return;
    }
    let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
    let base_health = e.vcall(owner, 0x00, &args![0x10u32]).i32() as f32;
    if !e
        .call(TRACKED_DAMAGE_TEST, &args![tracked, base_health])
        .bool()
    {
        return;
    }
    let word = e.call(ACTOR_GETTER_87F9F0, &args![this]).u16();
    let flag = u32::from(!e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool());
    let points = e
        .call(GET_EXPERIENCE_POINTS, &args![flag, u32::from(word)])
        .i32() as f32;
    let multiplier = e.call(PLAYER_INT_5BE4D0, &args![player]).u32();
    let scaled = e.call(SCALE_EXPERIENCE, &args![points, multiplier]).f32();
    let rounded = e.call(FLOAT_ROUND, &args![scaled]).f64();
    let amount = e.call(FTOL, &args![rounded]).u32();
    e.vcall(player, PLAYER_SLOT_488, &args![amount]);
}

/// Death dialogue or sound: unless the actor is within the hearing
/// distance of the player (and is not a teammate-killed or protected
/// case), a creature plays a creature sound (`PickCreatureSound(8)`) and
/// anything else starts the combat dialogue 6.
fn play_death_voice(e: &mut Engine, ebp: u32, this: Ptr<Actor>, attacker: Ptr<Actor>, player: u32) {
    let mut near = !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        && e.call(GLOBAL_FLAG_525420, &args![]).bool();
    if near {
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![this, player, 0u32, 0u32],
            )
            .f32();
        let limit = float_setting(e, DEATH_VOICE_DISTANCE_SETTING);
        if limit as f64 >= distance as f64 {
            near = false;
        }
    }
    if near {
        return;
    }
    let manager = e.global::<u32>(COMBAT_DIALOGUE_MANAGER);
    if !e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool() {
        e.call(
            COMBAT_DIALOGUE_START_DIALOGUE,
            &args![manager, this, attacker, 2u32, 6u32, 1u32, 0u32],
        );
        return;
    }
    let mut creature = 0u32;
    if e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool() {
        creature = base_form(e, this.cast()).addr();
    }
    let sound = if creature != 0 {
        e.call(CREATURE_PICK_CREATURE_SOUND, &args![creature, 8u32])
            .u32()
    } else {
        0
    };
    if sound == 0 {
        e.call(
            COMBAT_DIALOGUE_START_DIALOGUE,
            &args![manager, this, attacker, 2u32, 6u32, 1u32, 0u32],
        );
        return;
    }
    let handle = ebp - 0x1f8;
    let id = e.call(FORM_NUMERIC_ID, &args![sound]).u32();
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(
        AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID,
        &args![audio, handle, id, 0x102u32],
    );
    let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
    let x = e.mem.u32(position);
    let y = e.mem.u32(position + 4);
    let z = e.mem.u32(position + 8);
    e.call(SOUND_HANDLE_SET_POSITION_XYZ, &args![handle, x, y, z]);
    let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
    e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![handle, node]);
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
}

/// What happens when the player is the one dying: the Pipboy radio is
/// switched off, the death music plays, a VATS playback ends and an open
/// surgery menu closes.
fn player_dies(e: &mut Engine, ebp: u32, this: Ptr<Actor>) {
    e.call(PIPBOY_RADIO_ENABLE, &args![0u32]);
    e.call(SOUND_FADE_WORKER, &args![0x3e8u32, 0u32]);
    let handle = ebp - 0x210;
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let music = e
        .call(
            AUDIO_GET_SOUND_HANDLE_BY_NAME,
            &args![audio, handle, DEATH_MUSIC_NAME, 0x901u32],
        )
        .u32();
    e.call(SOUND_HANDLE_PLAY, &args![music, 0u32]);
    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    e.call(VATS_QUIT_PLAYBACK, &args![VATS_OBJECT, 0u32, 0u32]);
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(process, PROCESS_SLOT_3F4, &args![0u32]);
    }
    if e.call(INTERFACE_IS_TOP_MENU_ID, &args![0x41eu32]).bool() {
        e.call(SURGERY_MENU_CLOSE, &args![]);
    }
}

/// The last part of the killing: face animation, VATS, process state,
/// `TES` bookkeeping, the ragdoll impulse (with the many vector helpers)
/// and the weapon drop, or `Actor::DoDeathStuff` and the dead flag when
/// there is no process.
fn finish_dying(e: &mut Engine, ebp: u32, this: Ptr<Actor>, attacker: Ptr<Actor>, player: u32) {
    let face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![this]).u32();
    if face != 0 {
        e.vcall(face, 0xd8, &args![1u32, 0u32]);
    }
    if this.addr() == player {
        let vats_acting = e.call(ENTRY_FORM, &args![VATS_OBJECT]).u32() == 4
            && e.call(VATS_GET_CURRENT_ACTION, &args![VATS_OBJECT]).u32() != 0
            && {
                let action = e.call(VATS_GET_CURRENT_ACTION, &args![VATS_OBJECT]).u32();
                e.mem.u32(action) == 0xf
            };
        if !vats_acting {
            e.call(VATS_WORKER_9CA1F0, &args![VATS_OBJECT]);
        }
    }
    let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if process != 0 && e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0 {
        let flag = fn_0089f540(e, Ptr::new(process));
        if flag != 0 {
            let result = e
                .call(ACTOR_GETTER_80CED0, &args![this, u32::from(flag)])
                .u8();
            fn_0089f520(e, Ptr::new(process), result);
        }
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(process, PROCESS_SLOT_28, &args![]);
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let stamp_object = TIME_STAMP_OBJECT;
        let stamp = e.call(TIME_STAMP_867E30, &args![stamp_object]).u32();
        e.vcall(process, PROCESS_SLOT_6A0, &args![stamp as f32]);
    }
    let extra = extra_data_list_of(e, this.cast());
    let mut base = e
        .call(EXTRA_GET_LEV_CREA_ORIGINAL_BASE, &args![extra])
        .u32();
    if base == 0 {
        base = e.call(ACTOR_BASE_FORM, &args![this]).u32();
    }
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(TES_WORKER_459060, &args![tes, base, 1u32]);
    if e.vcall(this.addr(), ACTOR_SLOT_448, &args![]).bool() {
        let extra = extra_data_list_of(e, this.cast());
        e.call(EXTRA_REMOVE_TRESPASS_PACKAGE, &args![extra]);
    }
    let node = e.call(ACTOR_COLLISION_OBJECT, &args![this]).u32();
    let velocity = ebp - 0x250;
    let zero = 0.0f32;
    e.call(VECTOR4_CONSTRUCT, &args![velocity, zero, zero, zero, zero]);
    if e.call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
        .u32()
        != 0
    {
        let linear = ebp - 0x25c;
        let controller = e
            .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
            .u32();
        e.call(
            BHK_CHARACTER_PROXY_GET_LINEAR_VELOCITY,
            &args![controller, linear],
        );
        e.call(VECTOR_ASSIGN, &args![velocity, linear]);
    }
    if this.addr() == player {
        e.call(MOBILE_OBJECT_SET_CHASE_BIP, &args![this, 1u32]);
    } else {
        e.vcall(this.addr(), ACTOR_SLOT_2A0, &args![]);
    }
    let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let alive_body =
        process != 0 && e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0 && node != 0;
    if alive_body {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let timer = float_setting(e, GREETING_TIMER_SETTING);
        e.vcall(process, PROCESS_SLOT_4B4, &args![timer]);
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let mut impulse_branch = e.vcall(process, PROCESS_SLOT_40C, &args![]).u32() == 0;
        if !impulse_branch {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            impulse_branch = e.vcall(process, PROCESS_SLOT_40C, &args![]).u32() == 6;
        }
        if impulse_branch {
            apply_death_impulse(e, ebp, this, attacker, player, node, velocity);
        } else if e.vcall(this.addr(), ACTOR_SLOT_234, &args![]).bool() {
            apply_knocked_impulse(e, ebp, this, node);
        }
        let manager = e.global::<u32>(COMBAT_MANAGER);
        let count = e
            .call(
                COMBAT_MANAGER_GET_COMBATANT_COUNT,
                &args![manager, 0u32, 0u32],
            )
            .u32();
        let too_many = count > int_setting(e, COMBATANT_COUNT_SETTING);
        let skip = too_many && e.vcall(this.addr(), ACTOR_SLOT_38C, &args![]).bool();
        if !skip && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            if e.vcall(process, PROCESS_SLOT_52C, &args![]).u32() == 0 {
                let roll = e.call(RANDOM_NUMBER, &args![]).u32() % 100;
                if roll < int_setting(e, DROP_WEAPON_CHANCE_SETTING)
                    || !e.vcall(this.addr(), ACTOR_SLOT_38C, &args![]).bool()
                {
                    fn_0089f580(e, this);
                }
            }
        }
    } else {
        e.call(ACTOR_DO_DEATH_STUFF, &args![this]);
        e.call(REFR_RUN_SCRIPT, &args![this]);
        e.mem.set_u8(this.addr() + 0x118, 1);
    }
}

/// The impulse a dying actor's body gets when it is not knocked down in a
/// way that already moves it: the direction is the character proxy's
/// velocity direction, the actor's heading when that is zero, or away from
/// the player when the player is the killer.
fn apply_death_impulse(
    e: &mut Engine,
    ebp: u32,
    this: Ptr<Actor>,
    attacker: Ptr<Actor>,
    player: u32,
    node: u32,
    velocity: u32,
) {
    if e.call(ACTOR_TEST_87EEB0, &args![this]).bool() {
        return;
    }
    let direction = ebp - 0x274;
    let transformed = ebp - 0x290;
    let rotation = ebp - 0x2b4;
    let rotated = ebp - 0x2c0;
    let player_position = ebp - 0x2cc;
    let own_position = ebp - 0x2d8;
    let difference = ebp - 0x2e4;
    let angular = ebp - 0x2f0;
    copy_vector(e, DEFAULT_DIRECTION, direction);
    let object = e
        .call(VECTOR_TRANSFORM_630B40, &args![velocity, transformed])
        .u32();
    let length = e.call(VECTOR_LENGTH_4586D0, &args![object]).f64();
    if length == 0.0 {
        let heading = e
            .vcall(this.addr(), ACTOR_SLOT_GET_HEADING, &args![0u32])
            .f32();
        e.call(VECTOR_ROTATE_4A0C90, &args![rotation, heading]);
        let result = e
            .call(
                VECTOR_MULTIPLY_4B4500,
                &args![rotation, rotated, UNIT_VECTOR],
            )
            .u32();
        copy_vector(e, result, direction);
    } else {
        e.call(VECTOR_COPY_458620, &args![direction, velocity]);
        e.call(VECTOR_NORMALIZE, &args![direction]);
    }
    if attacker.addr() == player
        && this.addr() != player
        && e.call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
            .u32()
            != 0
    {
        copy_vector(e, DEFAULT_DIRECTION, player_position);
        copy_vector(e, DEFAULT_DIRECTION, own_position);
        let controller = e
            .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![player])
            .u32();
        e.call(
            BHK_CHARACTER_CONTROLLER_GET_POSITION,
            &args![controller, player_position],
        );
        let controller = e
            .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
            .u32();
        e.call(
            BHK_CHARACTER_CONTROLLER_GET_POSITION,
            &args![controller, own_position],
        );
        let result = e
            .call(
                VECTOR_SUBTRACT,
                &args![own_position, difference, player_position],
            )
            .u32();
        copy_vector(e, result, direction);
        e.call(VECTOR_NORMALIZE, &args![direction]);
    }
    e.call(BHK_WORLD_SET_MOTION, &args![node, 1u32, 1u32, 1u32, 1u32]);
    let zero = 0.0f32;
    e.call(VECTOR3_CONSTRUCT_43D410, &args![angular, zero, 0u32, 0u32]);
    e.call(COLLISION_OBJECT_SET_ANGULAR_VELOCITY, &args![node, angular]);
    let factor = float_setting(e, DEATH_IMPULSE_SETTING);
    e.call(VECTOR_SCALE_IN_PLACE, &args![direction, factor]);
    e.call(TES_HAVOK_ADD_VELOCITY, &args![node, direction, 0u32]);
    e.vcall(this.addr(), ACTOR_SLOT_48, &args![4u32]);
}

/// The push a dying actor gets in the knocked-down state: the body is
/// un-fixed and given an upward velocity along its heading.
fn apply_knocked_impulse(e: &mut Engine, ebp: u32, this: Ptr<Actor>, node: u32) {
    e.call(BHK_RIGID_BODY_SET_FIXED, &args![node, 0u32, 1u32]);
    let rotation = ebp - 0x314;
    let up = ebp - 0x320;
    let rotated = ebp - 0x32c;
    let velocity = ebp - 0x340;
    let heading = e
        .vcall(this.addr(), ACTOR_SLOT_GET_HEADING, &args![0u32])
        .f32();
    e.call(VECTOR_ROTATE_4A0C90, &args![rotation, heading]);
    let upward: f32 = e.global(UPWARD_COMPONENT);
    e.call(VECTOR3_CONSTRUCT_416870, &args![up, 0.0f32, upward, 0.0f32]);
    let result = e
        .call(VECTOR_MULTIPLY_4B4500, &args![rotation, rotated, up])
        .u32();
    copy_vector(e, result, up);
    e.call(VECTOR_ASSIGN, &args![velocity, up]);
    e.call(TES_HAVOK_SET_VELOCITY, &args![node, velocity, 0u32]);
}

// Translated from 0089d900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Kill` (Xbox PDB): the actor `attacker` (may be null) kills this
/// one with `damage`.
///
/// Does nothing for protected actors (slot `0x22c(0)`). Forgets the
/// actor's remembered weapon, idle and awake sounds. With the task queue in
/// use it only queues the kill (`QueueActorKill`, `damage` in a 4-byte
/// block, `attacker` stored at `+0xc0`). Otherwise records the kill
/// statistics for a player-owned attacker, clears `bMurderAlarm`, stops
/// dialogue and combat, tells the process and clears collision and ragdoll
/// state, and then takes one of two branches chosen by the essential
/// setting (`0x11e0888`) and `GetEssential`: [`kill_essential_actor`] keeps
/// the actor alive, [`kill_actor`] kills it. Both end in the common tail:
/// clearing the penetration utility, the death report, exporting progress
/// for the player, the combat bookkeeping, the magic shader effect and the
/// continuous beam.
/// The compiler's exception-unwinding frame is not translated;
/// `Actor::GetAnimGroup` is called by its address.
pub fn actor_kill(e: &mut Engine, this: Ptr<Actor>, attacker: Ptr<Actor>, damage: f32) {
    let mut attacker = attacker;
    if e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return;
    }
    let player_address = e.global::<u32>(PLAYER_CHARACTER);
    if e.vcall(this.addr(), ACTOR_SLOT_1A0, &args![0u32]).bool() && this.addr() != player_address {
        return;
    }
    // The exe's frame aligns `EBP` to 16 bytes, with 0x364 bytes under it (`SUB ESP,0x34c` under
    // the exception record and `EBX`, then `ESI` and the record link). Three sound handles of
    // 12 bytes (`BSSoundHandle`): attack at `EBP-0x24`, idle at `EBP-0x30`, awake at `EBP-0x3c`.
    e.with_stack(0x364 + 0x10, |e, raw| {
        let ebp = (raw.addr() + 0x364 + 0xf) & !0xf;
        let handles = ebp - 0x3c;
        forget_actor_sound(
            e,
            this,
            handles + 24,
            EXTRA_GET_WEAPON_ATTACK_SOUND,
            EXTRA_SET_WEAPON_ATTACK_SOUND,
            true,
        );
        forget_actor_sound(
            e,
            this,
            handles + 12,
            EXTRA_GET_WEAPON_IDLE_SOUND,
            EXTRA_SET_WEAPON_IDLE_SOUND,
            true,
        );
        forget_actor_sound(
            e,
            this,
            handles,
            EXTRA_GET_CREATURE_AWAKE_SOUND,
            EXTRA_SET_CREATURE_AWAKE_SOUND,
            false,
        );
        if e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
            let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
            e.mem.set_f32(block, damage);
            e.set(this, Actor::pMyKiller, attacker.cast());
            let tes = e.call(GET_TES, &args![]).u32();
            e.call(QUEUE_ACTOR_KILL, &args![tes, this, block, 0u32]);
            destroy_kill_sound_handles(e, handles);
            return;
        }

        if !attacker.is_null()
            && e.vcall(attacker.addr(), ACTOR_SLOT_360, &args![]).bool()
            && !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        {
            record_kill_statistics(e, this, attacker, player_address);
        }

        e.mem.set_u8(this.addr() + 0xc4, 0);
        let mut was_aggressor = false;
        if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            if e.vcall(process, PROCESS_SLOT_1C4, &args![]).bool() {
                was_aggressor = true;
            }
        }
        e.call(MOBILE_OBJECT_STOP_CURRENT_DIALOGUE, &args![this]);
        e.vcall(this.addr(), ACTOR_SLOT_STOP_COMBAT, &args![0u32]);
        if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            e.vcall(process, PROCESS_SLOT_6C4, &args![]);
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            e.vcall(process, PROCESS_SLOT_180, &args![]);
        }
        if e.vcall(this.addr(), ACTOR_SLOT_234, &args![]).bool() {
            let collision = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
            e.call(BHK_RIGID_BODY_SET_FIXED, &args![collision, 0u32, 1u32]);
        }
        let ragdoll = e.mem.u32(this.addr() + 0xac);
        if ragdoll != 0 {
            e.call(BHK_RAGDOLL_DISABLE_RAGDOLL_ANIM, &args![ragdoll, 1u32]);
        }

        let setting = e
            .call(SETTING_GET_BOOL_POINTER, &args![ESSENTIAL_SETTING])
            .u32();
        if e.mem.u8(setting) != 0 && e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool() {
            if kill_essential_actor(e, ebp, this, handles) {
                return;
            }
        } else {
            kill_actor(e, ebp, this, &mut attacker, was_aggressor, player_address);
        }

        // The common tail.
        let penetration = e.mem.u32(this.addr() + 0xb0);
        if penetration != 0 {
            e.call(BHK_RAGDOLL_PENETRATION_UTIL_CLEAR, &args![penetration]);
            let output = ebp - 0x348;
            let object = e.call(ACTOR_OUTPUT_931ED0, &args![this, output]).u32();
            let value = e.call(OBJECT_ACCESSOR_4A3A20, &args![object]).u32();
            let collision = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
            e.call(
                PENETRATION_UTIL_WORKER,
                &args![penetration, collision, value],
            );
        }
        let report = e
            .call(SETTING_GET_BOOL_POINTER, &args![DEATH_REPORT_SETTING])
            .u32();
        if e.mem.u8(report) != 0 {
            report_death(e, this, attacker);
        }
        let export = e.call(SETTING_TEST_454AF0, &args![EXPORT_SETTING]).bool();
        if export && this.addr() == player_address {
            e.set(this, Actor::pMyKiller, attacker.cast());
            e.call(PLAYER_EXPORT_PROGRESS_DATA, &args![player_address, 1u32]);
            e.set(this, Actor::pMyKiller, Ptr::NULL);
        }
        update_kill_counts(e, ebp, this, attacker, player_address);
        if fn_0089f500(e, Ptr::new(player_address)) != 0 {
            let holder = e.global::<u32>(MAGIC_SHADER_HOLDER);
            let shader = fn_0089f4e0(e, Ptr::new(holder));
            if shader != 0 {
                e.call(
                    PROCESS_LISTS_FINISH_MAGIC_SHADER_HIT_EFFECT,
                    &args![PROCESS_LISTS, this, shader],
                );
            }
        }
        let beam = e.mem.u32(this.addr() + 0x1a0);
        if beam != 0 {
            e.call(CONTINUOUS_BEAM_PROJECTILE_SET_KILL, &args![beam, this]);
        }
        destroy_kill_sound_handles(e, handles);
    });
}
/// Acquire object slot `0x128`: returns the actor it is acquiring from.
const ACQUIRE_OBJECT_SLOT_128: u32 = 0x128;
/// Acquire object slot `0x5c4` (takes `-1` from `StopCombat`).
const ACQUIRE_OBJECT_SLOT_5C4: u32 = 0x5c4;
/// Acquire object slot `0x1c8` (takes `0` from `StopCombat`).
const ACQUIRE_OBJECT_SLOT_1C8: u32 = 0x1c8;
/// `PlayerCharacter::RemoveActorFromPlayercombatList` (Xbox PDB): thiscall on
/// the player.
const PLAYER_REMOVE_ACTOR_FROM_COMBAT_LIST: u32 = 0x0093_a660;
/// `CombatController::StopCombat` (Xbox PDB): thiscall on the controller,
/// taking the target; returns a byte.
const COMBAT_CONTROLLER_STOP_COMBAT: u32 = 0x0097_f4d0;
/// `Script::SetActionFlag` (Xbox PDB), cdecl: the actor, its extra data list
/// and the flag.
const SCRIPT_SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `"%.20s stops combat."`.
const STOPS_COMBAT_FORMAT: u32 = 0x0108_4d0c;
/// Process slot `0x620` (takes `1` at the end of `StopCombat`).
const PROCESS_SLOT_620: u32 = 0x620;
/// `Actor::RestoreActorValue` (Xbox PDB): the actor value and the amount.
const ACTOR_RESTORE_ACTOR_VALUE: u32 = 0x0088_b740;
/// `1000.0` (`float`), the amount restored to the condition values.
const RESTORE_AMOUNT: u32 = 0x0101_3974;
/// Speed in ST0 (`00885ed0`), used when the animation flags `0x200` and
/// `0x800` are both set.
const ACTOR_SPEED_885ED0: u32 = 0x0088_5ed0;
/// Speed in ST0 (`00885d90`), used with flag `0x800` and not `0x200`.
const ACTOR_SPEED_885D90: u32 = 0x0088_5d90;
/// Speed in ST0 (`00886010`), used with flag `0x2000`.
const ACTOR_SPEED_886010: u32 = 0x0088_6010;
/// `MiddleHighProcess::IsHeavyBodyArmorWorn` (Xbox PDB), called with the
/// actor.
const IS_HEAVY_BODY_ARMOR_WORN: u32 = 0x008d_8270;
/// Actor slot `0x30c` (takes a `float`).
const ACTOR_SLOT_30C: u32 = 0x30c;
/// Actor slot `0x310` (returns a `float`; `-1.0` means not yet computed).
const ACTOR_SLOT_310: u32 = 0x310;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), cdecl.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges` method `004d1180` (thiscall, takes the actor): returns
/// a `float` in ST0.
const CHANGES_VALUE_4D1180: u32 = 0x004d_1180;
/// `004d1360`: byte result on the player (`thePlayer` is `this`).
const PLAYER_FLAG_4D1360: u32 = 0x004d_1360;
/// `00577250`: thiscall on an actor, taking a byte.
const ACTOR_SET_FLAG_577250: u32 = 0x0057_7250;

// Translated from 008a06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::StopCombat` (Xbox PDB): ends the actor's combat against `other`
/// (null or the player stops it altogether). Releases the player's acquire
/// object when the actor is its source, removes the actor from the player's
/// combat list, unblocks, tells the combat controller to stop and, when that
/// ended the combat and the actor is not fleeing, clears the combat state.
pub fn actor_stop_combat(e: &mut Engine, this: Ptr<Actor>, other: Ptr<Actor>) {
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    if e.vcall(this.addr(), ACTOR_SLOT_304, &args![]).bool()
        && fn_008a08c0(e, Ptr::new(player_addr)) != 0
        && e.call(ACTOR_IN_COMBAT, &args![this]).bool()
    {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let source = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_128, &args![]).u32();
        if source == player_addr {
            let player_acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player_addr]).u32();
            e.vcall(
                player_acquire,
                ACQUIRE_OBJECT_SLOT_5C4,
                &args![0xffff_ffffu32],
            );
        }
    }
    if other.is_null() || other.addr() == player_addr {
        e.call(
            PLAYER_REMOVE_ACTOR_FROM_COMBAT_LIST,
            &args![player_addr, this],
        );
    }
    if other.addr() == player_addr {
        e.call(
            PLAYER_CHANGE_PERCEIVED_HOSTILE_STATUS,
            &args![player_addr, this, 0u32],
        );
    }
    actor_set_block(e, this, 0);
    e.vcall(this.addr(), ACTOR_SLOT_474, &args![0u32]);
    if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() == 0 {
        actor_clear_in_combat(e, this, 1);
    } else {
        let controller = fn_008a0330(e, this);
        let stopped = e
            .call(COMBAT_CONTROLLER_STOP_COMBAT, &args![controller, other])
            .u8();
        let fleeing = e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool();
        if !fleeing && stopped != 0 {
            e.call(ACTOR_WORKER_884F80, &args![this]);
            // 008a07f7: `005b6f70` is called with nothing pushed (it only
            // reads the byte at `011f158c` and returns).
            if e.call(DEBUG_MESSAGES_ENABLED, &args![]).bool() {
                let name = e.call(REFR_GET_NAME, &args![this]).u32();
                e.call(PRINT_DEBUG_LINE, &args![STOPS_COMBAT_FORMAT, name]);
            }
            e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![this, 1u32]);
            actor_clear_in_combat(e, this, 1);
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
            e.call(SCRIPT_SET_ACTION_FLAG, &args![this, extra, 0x40u32]);
            e.set(this, Actor::eQueuedattack, 0xff);
            let process = e.get(this, Actor::pCurrentProcess).addr();
            e.vcall(process, PROCESS_SLOT_620, &args![1u32]);
        }
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, ACQUIRE_OBJECT_SLOT_1C8, &args![0u32]);
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, PROCESS_SLOT_64C, &args![]);
    }
}

// Translated from 008a08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x7c4` (`StopCombat` calls it on the player).
pub fn fn_008a08c0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x7c4)
}

// Translated from 008a08e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ClearInCombat` (Xbox PDB): clears `bInCombat`; when `restore` is
/// set, the actor was in combat, the tests of slots `0x360` and `0x21c` are
/// false and the actor is essential, it gets its health and conditions back.
pub fn actor_clear_in_combat(e: &mut Engine, this: Ptr<Actor>, restore: u8) {
    let was_in_combat = e.get(this, Actor::bInCombat);
    e.set(this, Actor::bInCombat, false);
    if !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        && !e.vcall(this.addr(), ACTOR_SLOT_21C, &args![]).bool()
        && restore != 0
        && was_in_combat
        && e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
    {
        actor_restore_full_health_and_conditions(e, this);
    }
}

// Translated from 008a0960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RestoreFullHealthAndConditions` (Xbox PDB): lifts the current
/// values of actor values `0x10` and `0x16` back to 1 when they are below 0,
/// then restores `1000.0` of each of the values `0x1f`, `0x1d`, `0x1e`,
/// `0x1b`, `0x1c`, `0x1a` and `0x19`.
pub fn actor_restore_full_health_and_conditions(e: &mut Engine, this: Ptr<Actor>) {
    for actor_value in [0x10u32, 0x16] {
        let current = e.vcall(this.addr() + 0xa4, 0x14, &args![actor_value]).f32();
        if (current as f64) < e.global::<f64>(ZERO_DOUBLE) {
            let amount = (-(current as f64) + e.global::<f64>(ONE_DOUBLE)) as f32;
            e.call(ACTOR_RESTORE_ACTOR_VALUE, &args![this, actor_value, amount]);
        }
    }
    let amount: f32 = e.global(RESTORE_AMOUNT);
    for actor_value in [0x1fu32, 0x1d, 0x1e, 0x1b, 0x1c, 0x1a, 0x19] {
        e.call(ACTOR_RESTORE_ACTOR_VALUE, &args![this, actor_value, amount]);
    }
}

/// The `Actor` an actor-value owner pointer (`Actor + 0xa4`) belongs to, or 0.
fn actor_of_value_owner(owner: u32) -> u32 {
    if owner == 0 {
        0
    } else {
        owner.wrapping_sub(0xa4)
    }
}

// Translated from 008a0a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl callback of the actor value table: calls virtual slot `0x43c` of
/// the actor the owner pointer (`Actor + 0xa4`) belongs to and answers its
/// `ST0` unchanged (the exe ends on that call). It is the derivation function
/// (`pDerivationFunction`) that `0066efe0` registers for actor value 0x12,
/// and `0066ed60` stores its `ST0` as the derived `float`; the slot is
/// `Actor::GetArmorRating` (`float`; Xbox PDB slot `+0x438`, the `-4` of the
/// bounded `Actor` slots).
pub fn fn_008a0a90(e: &mut Engine, owner: u32) -> f64 {
    let actor = actor_of_value_owner(owner);
    e.vcall(actor, 0x43c, &args![]).f64()
}

// Translated from 008a0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl callback of the actor value table: calls virtual slot `0x440` of
/// the actor the owner pointer (`Actor + 0xa4`) belongs to and answers its
/// `ST0` unchanged (the exe ends on that call). It is the derivation function
/// that `0066efe0` registers for actor value 0x4c (read back by `0066ed60`);
/// the slot is `Actor::GetArmorThreshold` (`float`; Xbox PDB slot `+0x43c`).
pub fn fn_008a0ad0(e: &mut Engine, owner: u32) -> f64 {
    let actor = actor_of_value_owner(owner);
    e.vcall(actor, 0x440, &args![]).f64()
}

// Translated from 008a0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentSpeed` (Xbox PDB): with a process whose `0045cd60` test
/// is 0 the speed follows the animation flags (each test reads them anew:
/// `0x200` running, `0x800` and `0x2000` the other movement kinds); without
/// one the actor runs when in combat and not in heavy body armor, and walks
/// otherwise.
pub fn actor_get_current_speed(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process != 0 && e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0 {
        let flags = |e: &mut Engine, mask: u32| -> bool {
            e.call(ACTOR_ANIM_FLAGS, &args![this]).u32() & mask != 0
        };
        let speed = if !flags(e, 0x200) {
            if !flags(e, 0x800) {
                if !flags(e, 0x2000) {
                    ACTOR_GET_WALK_SPEED
                } else {
                    ACTOR_SPEED_886010
                }
            } else {
                ACTOR_SPEED_885D90
            }
        } else if !flags(e, 0x800) {
            if !flags(e, 0x2000) {
                ACTOR_GET_RUN_SPEED
            } else {
                ACTOR_SPEED_886010
            }
        } else {
            ACTOR_SPEED_885ED0
        };
        e.call(speed, &args![this]).f32()
    } else if e.call(ACTOR_IN_COMBAT, &args![this]).bool()
        && !e.call(IS_HEAVY_BODY_ARMOR_WORN, &args![this]).bool()
    {
        e.call(ACTOR_GET_RUN_SPEED, &args![this]).f32()
    } else {
        e.call(ACTOR_GET_WALK_SPEED, &args![this]).f32()
    }
}

// Translated from 008a0c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The current value of actor value `0xd` (slot `0x0c` of the owner at
/// `this + 0xa4`) passed through entry point `0x16`; returns the result.
pub fn fn_008a0c20(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let value = e.vcall(this.addr() + 0xa4, 0x0c, &args![0xdu32]).f32();
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), value);
        e.call(HANDLE_ENTRY_POINT, &args![0x16u32, this, slot]);
        e.mem.f32(slot.addr())
    })
}

// Translated from 008a0c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the speed of slot `0x310` is `-1.0`, computes it from the actor's
/// inventory changes (`004d1180`) and stores it through slot `0x30c`; returns
/// the speed of slot `0x310`.
pub fn fn_008a0c60(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let speed = e.vcall(this.addr(), ACTOR_SLOT_310, &args![]).f32();
    if speed as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        let value = e.call(CHANGES_VALUE_4D1180, &args![changes, this]).f32();
        e.vcall(this.addr(), ACTOR_SLOT_30C, &args![value]);
    }
    e.vcall(this.addr(), ACTOR_SLOT_310, &args![]).f32()
}

// Translated from 008a0cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl callback of the actor value table: asks the player (`004d1360`)
/// and hands the byte to `00577250` on the actor the owner pointer
/// (`Actor + 0xa4`) belongs to, answering that call's `ST0` unchanged (the exe
/// ends on it; `00577250` answers a `float`). It is a derivation function that
/// `0066efe0` registers, read back as a `float` by `0066ed60`.
pub fn fn_008a0cd0(e: &mut Engine, owner: u32) -> f64 {
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    let flag = e.call(PLAYER_FLAG_4D1360, &args![player_addr]).u8();
    let actor = actor_of_value_owner(owner);
    e.call(ACTOR_SET_FLAG_577250, &args![actor, flag]).f64()
}
/// `8192.0` (`double`), the distance from the player beyond which nothing is
/// detected.
const DETECTION_MAX_DISTANCE: u32 = 0x0108_4d28;
/// Acquire object slot `0x504`: the detection record of an actor (takes the
/// actor and a flag; `+8` level, `+0x1c` and `+0x1e` bytes).
const ACQUIRE_OBJECT_SLOT_504: u32 = 0x504;
/// Acquire object slot `0x1fc` (takes two actors).
const ACQUIRE_OBJECT_SLOT_1FC: u32 = 0x1fc;
/// Acquire object slot `0x734` (returns a `float`).
const ACQUIRE_OBJECT_SLOT_734: u32 = 0x734;
/// Acquire object slot `0xf0` (takes seven words, see
/// [`detection_level_against_actor`]).
const ACQUIRE_OBJECT_SLOT_F0: u32 = 0xf0;
/// Counter incremented by every line of sight detection check
/// (`0x011dea38`).
const DETECTION_CHECK_COUNTER: u32 = 0x011d_ea38;
/// `float` `2.0` the combined check stores when the package test fails.
const DETECTION_FLOAT_TWO: u32 = 0x0101_62c0;
/// Settings object whose float is the detection scale (`0x011d0f90`).
const SETTING_DETECTION_SCALE: u32 = 0x011d_0f90;
/// Settings object whose float is the view cone angle in degrees
/// (`0x011cd668`).
const SETTING_VIEW_CONE_ANGLE: u32 = 0x011c_d668;
/// Settings object whose integer is added for worn armor of the first kind
/// (`0x011d1554`).
const SETTING_ARMOR_BONUS_A: u32 = 0x011d_1554;
/// Settings object whose integer is added for worn armor of the second kind
/// (`0x011d0538`).
const SETTING_ARMOR_BONUS_B: u32 = 0x011d_0538;
/// Settings object whose float scales the distance when the entry point
/// `0x45` result is positive (`0x011d031c`).
const SETTING_DISTANCE_SCALE: u32 = 0x011d_031c;
/// `0.017453292` (`double`), degrees to radians.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `0.875` (`float`) passed to `00885520`.
const FLOAT_0_875: u32 = 0x0108_4d20;
/// `Actor::LineOfSight` (Xbox PDB): thiscall with the flag, the other
/// reference, a flag, a pointer to a number and a flag byte; returns a byte.
const ACTOR_LINE_OF_SIGHT: u32 = 0x0088_b880;
/// `0097fd60`: thiscall on the combat controller, takes the line of sight
/// number.
const COMBAT_CONTROLLER_SET_LINE_OF_SIGHT: u32 = 0x0097_fd60;
/// `00436aa0`: thiscall on an actor, returns the value `IsPointInViewCone`
/// is given (no stack arguments).
const ACTOR_VIEW_CONE_TARGET: u32 = 0x0043_6aa0;
/// `Actor::IsPointInViewCone` (Xbox PDB): thiscall with an angle in radians
/// and the point.
const ACTOR_IS_POINT_IN_VIEW_CONE: u32 = 0x0088_c570;
/// `00891be0`: thiscall on the actor, taking the worn armor entry; returns
/// the value passed to the detection formula.
const ACTOR_ARMOR_VALUE: u32 = 0x0089_1be0;
/// `004c0bd0`: test on the armor form's `+0x70` part (first armor kind).
const ARMOR_KIND_TEST_A: u32 = 0x004c_0bd0;
/// `00514450`: test on the armor form's `+0x70` part (second armor kind).
const ARMOR_KIND_TEST_B: u32 = 0x0051_4450;
/// `00472380`: thiscall on the other actor, returns a word for the formula.
const ACTOR_VALUE_472380: u32 = 0x0047_2380;
/// `004938e0`: thiscall on the other actor, returns a byte for the formula.
const ACTOR_FLAG_4938E0: u32 = 0x0049_38e0;
/// `00885520`: thiscall on the actor with the position, the cell and a
/// `float`; returns a byte that is not used further.
const ACTOR_POSITION_CELL_TEST: u32 = 0x0088_5520;
/// `ActorValueOwner::GetClampedActorFloatValue` (Xbox PDB): thiscall on the
/// owner at `actor + 0xa4`, takes the actor value; returns a `float`.
const GET_CLAMPED_ACTOR_FLOAT_VALUE: u32 = 0x0066_ef50;
/// `Actor::GetHeight` (Xbox PDB): the height in ST0.
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `00642e80`: cdecl, a `float` height to the formula's size class.
const HEIGHT_CLASS: u32 = 0x0064_2e80;
/// `00642ed0`: cdecl, the detection formula (25 words); returns the
/// detection level.
const DETECTION_FORMULA: u32 = 0x0064_2ed0;
/// Actor value number whose clamped value is the detecting actor's
/// perception.
const ACTOR_VALUE_PERCEPTION: u32 = 6;

// Translated from 008a0d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetDetectionLevelAgainstActor` (Xbox PDB): how well `this` detects
/// `other`; returns the level (`-100` when nothing can be detected, `0` when
/// the actor has no acquire object). `*seen` receives whether `other` was
/// seen; `*seen_line` (optional) the line of sight result. With `use_line`
/// set the level is computed by the detection formula (`00642ed0`) from the
/// two actors' perception and stealth values, distance, armor, movement and
/// line of sight, and reported to both acquire objects (slot `0xf0`) when
/// `report_always` is not 0 or `this` is the player; without it the stored
/// detection record is read. `report` is passed on to slot `0xf0`.
#[allow(clippy::too_many_arguments)]
pub fn actor_get_detection_level_against_actor(
    e: &mut Engine,
    this: Ptr<Actor>,
    use_line: u8,
    other: Ptr<Actor>,
    seen: Ptr,
    report: u8,
    report_always: u8,
    _unused_7: u32,
    seen_line: Ptr,
) -> i32 {
    e.with_stack(0x18, |e, scratch| {
        detection_level_against_actor(
            e,
            this,
            use_line,
            other,
            seen,
            report,
            report_always,
            seen_line,
            scratch.addr(),
        )
    })
}

/// The body of [`actor_get_detection_level_against_actor`]; `scratch` is the
/// block of locals the game keeps on its stack (`+0` the entry point `0x45`
/// result, `+4` the line of sight number, `+8` the entry point `0x1f`
/// result, `+0xc`, `+0x10`, `+0x14` the formula's three out `float`s).
#[allow(clippy::too_many_arguments)]
fn detection_level_against_actor(
    e: &mut Engine,
    this: Ptr<Actor>,
    use_line: u8,
    other: Ptr<Actor>,
    seen: Ptr,
    report: u8,
    report_always: u8,
    seen_line: Ptr,
    scratch: u32,
) -> i32 {
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() == 0 {
        return 0;
    }
    let mut result: i32 = 0x7fff_ffff;
    e.mem.set_u8(seen.addr(), 0);
    e.mem.set_f32(scratch, 0.0);
    e.call(HANDLE_ENTRY_POINT, &args![0x45u32, this, scratch]);
    let limit: f64 = e.global(DETECTION_MAX_DISTANCE);
    // x87 `FCOMP` + `TEST AH,1` treats an unordered result as "below".
    let first = e
        .call(
            REFR_GET_DISTANCE_FROM_REFERENCE,
            &args![this, player_addr, 0u32, 0u32],
        )
        .f64();
    if first >= limit {
        return -100;
    }
    let second = e
        .call(
            REFR_GET_DISTANCE_FROM_REFERENCE,
            &args![other, player_addr, 0u32, 0u32],
        )
        .f64();
    if second >= limit {
        return -100;
    }
    if e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![other]).bool()
        && !e.call(ACTOR_IN_COMBAT, &args![other]).bool()
        && e.call(ACTOR_IS_MOVING, &args![player_addr]).bool()
        && !e.call(ACTOR_IN_COMBAT, &args![player_addr]).bool()
    {
        return -100;
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let record = e
        .vcall(acquire, ACQUIRE_OBJECT_SLOT_504, &args![other, 0u32])
        .u32();
    if record != 0 {
        result = e.mem.i32(record + 8);
        let byte = e.mem.u8(record + 0x1e);
        e.mem.set_u8(seen.addr(), byte);
    }
    if e.vcall(other.addr(), ACTOR_SLOT_1D0, &args![]).u32() == 0 {
        return -100;
    }
    if use_line == 0 {
        if result == 0x7fff_ffff {
            result = -100;
        } else if record != 0 && !seen_line.is_null() {
            let byte = e.mem.u8(record + 0x1c);
            e.mem.set_u8(seen_line.addr(), byte);
        }
        return result;
    }

    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let package_target = e.vcall(acquire, PROCESS_SLOT_22C, &args![]).u32();
    let mut package_bonus: f32 = 0.0;
    if package_target != 0 && e.call(PACKAGE_TYPE, &args![package_target]).u32() == 9 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        package_bonus = if e
            .vcall(acquire, ACQUIRE_OBJECT_SLOT_1FC, &args![this, other])
            .bool()
        {
            1.0
        } else {
            e.global(DETECTION_FLOAT_TWO)
        };
    }
    let mut distance = e
        .call(
            REFR_GET_DISTANCE_FROM_REFERENCE,
            &args![this, other, 0u32, 0u32],
        )
        .f32();
    let counter: u32 = e.global(DETECTION_CHECK_COUNTER);
    e.set_global(DETECTION_CHECK_COUNTER, counter.wrapping_add(1));
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![other]).u32();
    let stealth_float = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_734, &args![]).f64();
    let mut sneak_bonus = e.call(FTOL, &args![stealth_float]).i32();
    let value = e.vcall(this.addr() + 0xa4, 0x0c, &args![0x32u32]).f64();
    if value > zero {
        let scale_ptr = e
            .call(SETTING_GET_VALUE_POINTER, &args![SETTING_DETECTION_SCALE])
            .u32();
        let scale = e.mem.f32(scale_ptr) as f64;
        let scaled = e.call(FTOL, &args![scale]).i32();
        sneak_bonus = scaled.wrapping_mul(sneak_bonus);
        if sneak_bonus > 100 {
            sneak_bonus = 100;
        }
    }
    // The line of sight call reports through the number at `scratch + 4`.
    e.mem.set_u32(scratch + 4, 3);
    let line_of_sight_flag = 1u32;
    let line = e
        .call(
            ACTOR_LINE_OF_SIGHT,
            &args![this, 1u32, other, 1u32, scratch + 4, line_of_sight_flag],
        )
        .u8();
    if e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32() != 0 {
        let controller = fn_008a0330(e, this);
        if e.call(COMBAT_CONTROLLER_TARGET, &args![controller]).u32() == other.addr() {
            let line_number = e.mem.u32(scratch + 4);
            let controller = fn_008a0330(e, this);
            e.call(
                COMBAT_CONTROLLER_SET_LINE_OF_SIGHT,
                &args![controller, line_number],
            );
        }
    }
    // The flag passed to the line of sight call is always 1, so the result
    // always goes to the second output (`seen_line`), which the callers of
    // this mode supply.
    e.mem.set_u8(seen_line.addr(), line);
    if line != 0 {
        let angle_ptr = e
            .call(SETTING_GET_VALUE_POINTER, &args![SETTING_VIEW_CONE_ANGLE])
            .u32();
        let radians: f64 = e.global(DEGREES_TO_RADIANS);
        let angle = (e.mem.f32(angle_ptr) as f64 * radians) as f32;
        let point = e.call(ACTOR_VIEW_CONE_TARGET, &args![other]).u32();
        let in_cone = e
            .call(ACTOR_IS_POINT_IN_VIEW_CONE, &args![this, point, angle])
            .bool();
        e.mem.set_u8(seen.addr(), in_cone as u8);
    } else {
        e.mem.set_u8(seen.addr(), 0);
    }

    // Worn armor of the detected actor.
    let armor = e
        .call(ACTOR_GET_ARMOR_BEING_WORN, &args![other, 2u32])
        .u32();
    let armor_value = e.call(ACTOR_ARMOR_VALUE, &args![other, armor]).u32();
    let mut armor_bonus: i32 = 0;
    let armor_form = if armor != 0 {
        e.call(ENTRY_FORM, &args![armor]).u32()
    } else {
        0
    };
    if armor != 0 {
        e.call(ENTRY_DELETE, &args![armor, 1u32]);
    }
    if armor_form != 0 {
        if e.call(ARMOR_KIND_TEST_A, &args![armor_form + 0x70]).bool() {
            let bonus = e
                .call(SETTING_GET_INT_POINTER, &args![SETTING_ARMOR_BONUS_A])
                .u32();
            armor_bonus = armor_bonus.wrapping_add(e.mem.i32(bonus));
        } else if e.call(ARMOR_KIND_TEST_B, &args![armor_form + 0x70]).bool() {
            let bonus = e
                .call(SETTING_GET_INT_POINTER, &args![SETTING_ARMOR_BONUS_B])
                .u32();
            armor_bonus = armor_bonus.wrapping_add(e.mem.i32(bonus));
        }
    }

    let this_in_combat = e.call(ACTOR_IN_COMBAT, &args![this]).u8();
    let word_472380 = e.call(ACTOR_VALUE_472380, &args![other]).u32();
    let seen_now = (e.mem.u8(seen.addr()) != 0) as u32;
    let mut byte_4938e0 = e.call(ACTOR_FLAG_4938E0, &args![other]).u8();
    let mut other_running = 0u8;
    if e.call(ACTOR_ANIM_FLAGS, &args![other]).u32() & 0x200 != 0 {
        other_running = 1;
    }
    e.mem.set_f32(scratch + 8, 0.0);
    e.call(HANDLE_ENTRY_POINT, &args![0x1fu32, other, scratch + 8]);
    if e.mem.f32(scratch + 8) as f64 != zero {
        other_running = 0;
    }
    let mut other_moving = e.call(ACTOR_IS_MOVING, &args![other]).u8();
    let cell = e.call(REFR_PARENT_CELL, &args![this]).u32();
    let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
    let float_0_875: f32 = e.global(FLOAT_0_875);
    e.call(
        ACTOR_POSITION_CELL_TEST,
        &args![this, position, cell, float_0_875],
    );
    let this_flag_1ac = e.call(REFR_FIELD_1AC_IS_9, &args![this]).u8();
    // The code tests the other actor's cell flag (`00425fd0`) and keeps the
    // result in a local it never reads.
    if e.call(REFR_PARENT_CELL, &args![other]).u32() != 0 {
        let cell = e.call(REFR_PARENT_CELL, &args![other]).u32();
        e.call(CELL_FLAG_24_BIT_0, &args![cell]);
    }
    let own_perception = e
        .call(
            GET_CLAMPED_ACTOR_FLOAT_VALUE,
            &args![this.addr() + 0xa4, ACTOR_VALUE_PERCEPTION],
        )
        .f64();
    let own_perception = e.call(FTOL, &args![own_perception]).i32();
    let other_stealth = e
        .call(
            GET_CLAMPED_ACTOR_FLOAT_VALUE,
            &args![other.addr() + 0xa4, 0x2au32],
        )
        .f64();
    let mut other_stealth = e.call(FTOL, &args![other_stealth]).i32();
    if e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![other]).bool() {
        let player_stealth = e
            .call(
                GET_CLAMPED_ACTOR_FLOAT_VALUE,
                &args![player_addr + 0xa4, 0x2au32],
            )
            .f64();
        let player_stealth = e.call(FTOL, &args![player_stealth]).i32();
        if other_stealth < player_stealth {
            other_stealth = player_stealth;
        }
    }
    let mut stealth_flag = 0u8;
    let value = e.vcall(other.addr() + 0xa4, 0x0c, &args![0x30u32]).f64();
    if value > zero || e.vcall(other.addr() + 0xa4, 0x08, &args![0x31u32]).i32() > 0 {
        stealth_flag = 1;
    }
    if e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![other]).bool() {
        let value = e.vcall(player_addr + 0xa4, 0x0c, &args![0x30u32]).f64();
        if value > zero || e.vcall(player_addr + 0xa4, 0x08, &args![0x31u32]).i32() > 0 {
            stealth_flag = 1;
        }
    }
    let height = e.call(ACTOR_GET_HEIGHT, &args![this]).f32();
    let height_class = e.call(HEIGHT_CLASS, &args![height]).u32();
    e.mem.set_f32(scratch + 0x0c, 0.0);
    e.mem.set_f32(scratch + 0x10, 0.0);
    e.mem.set_f32(scratch + 0x14, 0.0);
    let mut cell_flag_clear = 1u8;
    if e.call(REFR_PARENT_CELL, &args![this]).u32() != 0 {
        let cell = e.call(REFR_PARENT_CELL, &args![this]).u32();
        if e.call(CELL_FLAG_24_BIT_0, &args![cell]).bool() {
            cell_flag_clear = 0;
        }
    }
    let mut group_member_other = 0u8;
    if !e.call(IS_HEAVY_BODY_ARMOR_WORN, &args![this]).bool()
        && fn_008a16b0(e, this) != 0
        && other.addr() != fn_008a16b0(e, this)
    {
        group_member_other = 1;
    }
    e.mem.set_f32(scratch + 8, 0.0);
    e.call(HANDLE_ENTRY_POINT, &args![0x1fu32, other, scratch + 8]);
    if e.mem.f32(scratch + 8) as f64 != zero && other_moving != 0 {
        byte_4938e0 = 0;
    }
    let this_word = e.vcall(this.addr() + 0xa4, 0x28, &args![]).u32() & 0xffff;
    let other_word = e.vcall(other.addr() + 0xa4, 0x28, &args![]).u32() & 0xffff;
    let entry_result = e.mem.f32(scratch);
    let mut line_flag = line;
    if (entry_result as f64) > zero {
        line_flag = 1;
        let scale_ptr = e
            .call(SETTING_GET_VALUE_POINTER, &args![SETTING_DISTANCE_SCALE])
            .u32();
        distance = (distance as f64 * e.mem.f32(scale_ptr) as f64) as f32;
        stealth_flag = 0;
        other_moving = 0;
    }
    let package_int = e.call(FTOL, &args![package_bonus as f64]).u32();
    result = e
        .call(
            DETECTION_FORMULA,
            &args![
                own_perception as u32,
                other_stealth as u32,
                line_flag as u32,
                seen_now,
                distance,
                sneak_bonus as u32,
                0u32,
                stealth_flag as u32,
                armor_value,
                byte_4938e0 as u32,
                other_moving as u32,
                other_running as u32,
                this_flag_1ac as u32,
                this_in_combat as u32,
                word_472380,
                height_class,
                cell_flag_clear as u32,
                group_member_other as u32,
                package_int,
                scratch + 0x0c,
                scratch + 0x10,
                scratch + 0x14,
                this_word,
                other_word,
                armor_bonus as u32
            ],
        )
        .i32();

    if use_line == 0 || report_always != 0 || this.addr() == player_addr {
        let mut level = 0u32;
        if result > 0 {
            level = 3;
        }
        if e.vcall(other.addr(), ACTOR_SLOT_100, &args![]).bool() {
            let seen_value = if seen_line.is_null() {
                e.mem.u8(seen.addr())
            } else {
                e.mem.u8(seen_line.addr())
            };
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let seen_byte = e.mem.u8(seen.addr());
            e.vcall(
                acquire,
                ACQUIRE_OBJECT_SLOT_F0,
                &args![
                    other,
                    level,
                    seen_byte as u32,
                    result,
                    0u32,
                    seen_value as u32,
                    report as u32
                ],
            );
            if result > 0 {
                let seen_value = if seen_line.is_null() {
                    e.mem.u8(seen.addr())
                } else {
                    e.mem.u8(seen_line.addr())
                };
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![other]).u32();
                let seen_byte = e.mem.u8(seen.addr());
                e.vcall(
                    acquire,
                    ACQUIRE_OBJECT_SLOT_F0,
                    &args![
                        this,
                        level,
                        seen_byte as u32,
                        result,
                        1u32,
                        seen_value as u32,
                        report as u32
                    ],
                );
            }
        }
    }
    result
}
/// Table of `u32` indexed by the weapon type (`0x0118a838`), read by
/// `fn_008a1760`.
const WEAPON_TYPE_TABLE: u32 = 0x0118_a838;
/// The setting object read through [`SETTING_GET_BOOL_POINTER`].
const SETTING_ESSENTIAL_PROTECTION: u32 = 0x011e_0888;
/// `Actor::CanKnockDown` (Xbox PDB).
const ACTOR_CAN_KNOCK_DOWN: u32 = 0x0088_45a0;
/// `Actor::StopMoving` (Xbox PDB).
const ACTOR_STOP_MOVING: u32 = 0x008b_3ab0;
/// Face animation data slot `0xd8` (takes `1, 1`).
const FACE_ANIMATION_SLOT_D8: u32 = 0xd8;
/// `004534f0`: thiscall on the container changes, takes the actor.
const CHANGES_FN_4534F0: u32 = 0x0045_34f0;
/// `0043fcd0`: thiscall on the actor, returns what `StopMovingSounds`
/// takes first.
const ACTOR_SOUND_SOURCE: u32 = 0x0043_fcd0;
/// `BSAudio::QInstance` (Xbox PDB): the audio singleton (no arguments).
const AUDIO_GET_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::StopMovingSounds` (Xbox PDB): thiscall on the audio object with
/// a word, a `float` and a word.
const AUDIO_STOP_MOVING_SOUNDS: u32 = 0x00ad_8570;
/// `00483710`: thiscall on the sound handle, the handle's destructor.
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// Process slot `0x610` (a state number; 3 and 4 fade the actor in).
const PROCESS_SLOT_610: u32 = 0x610;
/// `HighProcess::FadeIn` (Xbox PDB): thiscall on the process, takes the actor
/// and a flag.
const HIGH_PROCESS_FADE_IN: u32 = 0x008f_e8f0;
/// `Actor::ResetLoadedAnimations` (Xbox PDB).
const ACTOR_RESET_LOADED_ANIMATIONS: u32 = 0x0088_7d00;
/// `TaskQueueInterface` getter (`004537b0`, no arguments).
const TASK_QUEUE_INTERFACE: u32 = 0x0045_37b0;
/// `TaskQueueInterface::QueueActorApplyCriticalStage` (Xbox PDB): thiscall on
/// the interface, takes the actor.
const QUEUE_ACTOR_APPLY_CRITICAL_STAGE: u32 = 0x0087_b810;
/// Actor slot `0x130` (returns the actor's name for debug output).
const ACTOR_SLOT_130: u32 = 0x130;
/// `005b5e40`: cdecl debug print: format, stage, name, form id.
const PRINT_ERROR_LINE: u32 = 0x005b_5e40;
/// `"Applying critical stage %i to actor '%s' (%08X) who is not dead"`.
const CRITICAL_STAGE_NOT_DEAD_FORMAT: u32 = 0x0108_4d30;
/// `bhkUtilFunctions::ReplaceConstraints` (Xbox PDB), cdecl: the body and a
/// constraint kind.
const BHK_REPLACE_CONSTRAINTS: u32 = 0x00c8_0ce0;
/// `bhkUtilFunctions::DampBodiesVelocity` (Xbox PDB), cdecl: the body and a
/// `float` damping.
const BHK_DAMP_BODIES_VELOCITY: u32 = 0x00c8_03a0;
/// `TESObjectREFR::RemoveFromAllWater` (Xbox PDB): takes a flag.
const REFR_REMOVE_FROM_ALL_WATER: u32 = 0x0057_b520;
/// `00450f90`: thiscall on the body, takes `1`.
const BODY_FN_450F90: u32 = 0x0045_0f90;
/// Actor slot `0x1c0`.
const ACTOR_SLOT_1C0: u32 = 0x1c0;

// Translated from 008a16b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x128` (`pCurrentCombatTarget`, Xbox PDB). The map
/// names this body `Actor::IsCombatGroupMember`.
pub fn fn_008a16b0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    e.mem.u32(this.addr() + 0x128)
}

// Translated from 008a16d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsWeaponDrawn` (Xbox PDB): asks the actor's process (slot
/// `0x454`); false without a process.
pub fn actor_is_weapon_drawn(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process == 0 {
        return 0;
    }
    e.vcall(process, PROCESS_SLOT_454, &args![]).u8()
}

// Translated from 008a1710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentWeapon` (Xbox PDB): the form of the weapon entry the
/// process reports (slot `0x148`), or 0.
pub fn actor_get_current_weapon(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process != 0
        && e.vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32()
            != 0
    {
        let entry = e
            .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32();
        return e.call(ENTRY_FORM, &args![entry]).u32();
    }
    0
}

// Translated from 008a1760 (decompiled, FalloutNV.exe 1.4.0.525)
/// A number for the actor's state: 0 when fleeing (slot `0x22c`), `0xc` for
/// the player; for an actor with an acquire object whose level (`0045cd60`)
/// is 0 or 1, `1` without a weapon and otherwise the entry of the table at
/// `0x0118a838` for the weapon's type; 0 in the other cases.
pub fn fn_008a1760(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return 0;
    }
    if this.addr() == e.global::<u32>(PLAYER_CHARACTER) {
        return 0xc;
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let level = e.call(ACQUIRE_OBJECT_FIELD_28, &args![acquire]).i32();
        if (0..=1).contains(&level) {
            let weapon = actor_get_current_weapon(e, this);
            if weapon != 0 {
                let weapon_type = e.call(WEAPON_TYPE, &args![weapon]).u32();
                return e
                    .mem
                    .u32(WEAPON_TYPE_TABLE.wrapping_add(weapon_type.wrapping_mul(4)));
            }
            return 1;
        }
    }
    0
}

// Translated from 008a1800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetLifeState` (Xbox PDB). With the essential protection setting
/// on, an essential actor's state 1 or 2 becomes 6; state 6 becomes 0 when
/// the actor cannot be knocked down. Entering the states 1, 2 or 6 from 0
/// stops the actor moving. State 2 (dead) drops the face animation, tells the
/// container changes, silences the creature awake sound and stops the moving
/// sounds. A non-zero state fades the actor in when its process is in the
/// state 3 or 4 (slot `0x610`). A change of state calls slot `0x48` with
/// `0x400`, and entering or leaving 2 resets the loaded animations.
/// C++ exception unwinding (the sound handle's cleanup) is not translated.
pub fn actor_set_life_state(e: &mut Engine, this: Ptr<Actor>, state: u32) {
    let mut state = state;
    let flag = e
        .call(
            SETTING_GET_BOOL_POINTER,
            &args![SETTING_ESSENTIAL_PROTECTION],
        )
        .u32();
    if e.mem.u8(flag) != 0
        && e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
        && (state == 2 || state == 1)
    {
        state = 6;
    }
    if state == 6 && !e.call(ACTOR_CAN_KNOCK_DOWN, &args![this]).bool() {
        state = 0;
    }
    if e.get(this, Actor::eLifeState) == 0 && (state == 1 || state == 2 || state == 6) {
        e.call(ACTOR_STOP_MOVING, &args![this]);
    }
    if state == 2 {
        let face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![this]).u32();
        if face != 0 {
            e.vcall(face, FACE_ANIMATION_SLOT_D8, &args![1u32, 1u32]);
        }
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
        let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).u32();
        if changes != 0 {
            e.call(CHANGES_FN_4534F0, &args![changes, this]);
        }
        e.with_stack(12, |e, handle| {
            e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
            e.call(EXTRA_GET_CREATURE_AWAKE_SOUND, &args![extra, handle]);
            e.call(SOUND_HANDLE_STOP, &args![handle]);
            e.call(SOUND_HANDLE_RELEASE, &args![handle]);
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
            e.call(EXTRA_SET_CREATURE_AWAKE_SOUND, &args![extra, handle]);
            let source = e.call(ACTOR_SOUND_SOURCE, &args![this]).u32();
            let audio = e.call(AUDIO_GET_INSTANCE, &args![]).u32();
            e.call(
                AUDIO_STOP_MOVING_SOUNDS,
                &args![audio, source, 0.0f32, 0u32],
            );
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
        });
    }
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if state != 0
        && process != 0
        && e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0
        && (e.vcall(process, PROCESS_SLOT_610, &args![]).u32() == 3
            || e.vcall(process, PROCESS_SLOT_610, &args![]).u32() == 4)
    {
        e.call(HIGH_PROCESS_FADE_IN, &args![process, this, 0u32]);
    }
    if e.get(this, Actor::eLifeState) != state {
        e.vcall(this.addr(), ACTOR_SLOT_48, &args![0x400u32]);
    }
    let old = e.get(this, Actor::eLifeState);
    e.set(this, Actor::eLifeState, state);
    let new = e.get(this, Actor::eLifeState);
    if old != new && ((old == 2 && new != 1) || new == 2) {
        e.call(ACTOR_RESET_LOADED_ANIMATIONS, &args![this]);
    }
}

// Translated from 008a1a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the critical stage (`eCriticalStage`, Xbox PDB) and applies it.
pub fn fn_008a1a40(e: &mut Engine, this: Ptr<Actor>, stage: u32) {
    e.set(this, Actor::eCriticalStage, stage);
    actor_apply_critical_stage(e, this);
}

// Translated from 008a1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ApplyCriticalStage` (Xbox PDB): for an actor with a body (slot
/// `0x1d0`). When the task queue is in use (`008c7aa0`) queues the stage
/// instead. Otherwise stage 1 replaces the body's constraints and stops its
/// velocity; stages 2 and 4 take the actor out of the water, run slot
/// `0x1c0` and, when the killer (`pMyKiller`) is the player (slot `0x360`
/// of the killer) and the actor is not, count the statistic `0x1d`. A stage
/// other than 0 on a living actor prints a debug line.
pub fn actor_apply_critical_stage(e: &mut Engine, this: Ptr<Actor>) {
    let body = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
    if body == 0 {
        return;
    }
    if e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
        let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
        e.call(QUEUE_ACTOR_APPLY_CRITICAL_STAGE, &args![queue, this]);
        return;
    }
    let stage = e.get(this, Actor::eCriticalStage);
    if !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() && stage != 0 {
        let form_id = e.call(REFR_FORM_ID, &args![this]).u32();
        let name = e.vcall(this.addr(), ACTOR_SLOT_130, &args![]).u32();
        e.call(
            PRINT_ERROR_LINE,
            &args![CRITICAL_STAGE_NOT_DEAD_FORMAT, stage, name, form_id],
        );
    }
    match e.get(this, Actor::eCriticalStage) {
        1 => {
            e.call(BHK_REPLACE_CONSTRAINTS, &args![body, 8u32]);
            e.call(BHK_DAMP_BODIES_VELOCITY, &args![body, 0.0f32]);
        }
        2 | 4 => {
            e.call(REFR_REMOVE_FROM_ALL_WATER, &args![this, 0u32]);
            e.call(BODY_FN_450F90, &args![body, 1u32]);
            e.vcall(this.addr(), ACTOR_SLOT_1C0, &args![]);
            if !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool() {
                let killer = e.get(this, Actor::pMyKiller).addr();
                if killer != 0 && e.vcall(killer, ACTOR_SLOT_360, &args![]).bool() {
                    e.call(MISC_STAT_INCREMENT, &args![0x1du32]);
                }
            }
        }
        _ => {}
    }
}
/// `TESTopic::CreateDialogueItem` (Xbox PDB): thiscall on the topic, taking
/// the speaker, a word and three zeros; returns the dialogue item.
const TOPIC_CREATE_DIALOGUE_ITEM: u32 = 0x0061_b320;
/// `TESObjectREFR::SetSayToTopic` (Xbox PDB).
const REFR_SET_SAY_TO_TOPIC: u32 = 0x0057_ad20;
/// `TESObjectREFR::SetSayToTopicInfo` (Xbox PDB).
const REFR_SET_SAY_TO_TOPIC_INFO: u32 = 0x0057_ace0;
/// `TESTopicInfo::AddTopicList` (Xbox PDB).
const TOPIC_INFO_ADD_TOPIC_LIST: u32 = 0x0061_f150;
/// `TESTopicInfo::RunResult` (Xbox PDB): thiscall on the info, taking a flag
/// and the speaker.
const TOPIC_INFO_RUN_RESULT: u32 = 0x0061_f170;
/// `DialogueItem::FirstResponse` (Xbox PDB).
const DIALOGUE_ITEM_FIRST_RESPONSE: u32 = 0x0083_c7b0;
/// `DialogueItem::GetCurrentResponse` (Xbox PDB).
const DIALOGUE_ITEM_GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
/// `00933150`: thiscall on the actor, fills a 12-byte handle (first word:
/// what `SetCompletionCallback` is given) from the form id, a zero, a flag
/// word and a flag.
const MOBILE_OBJECT_FILL_SOUND_HANDLE: u32 = 0x0093_3150;
/// `BSAudioManager::QInstance` (Xbox PDB): the audio manager singleton.
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
/// `BSAudioManager::SetCompletionCallback` (Xbox PDB): thiscall on the
/// manager, taking the sound, the callback and a word.
const AUDIO_MANAGER_SET_COMPLETION_CALLBACK: u32 = 0x00ad_bfd0;
/// The completion callback the manager is given (`00935cc0`).
const SAY_TO_COMPLETION_CALLBACK: u32 = 0x0093_5cc0;
/// `00460140`: thiscall on a dialogue response, returns the object whose first
/// word is the sound file name pointer.
const RESPONSE_SOUND_HOLDER: u32 = 0x0046_0140;
/// `00406d30`: cdecl string copy (destination, size, source).
const COPY_STRING_LIMITED: u32 = 0x0040_6d30;
/// `BSSoundHandle::SetStaticAttenuation` (Xbox PDB): takes an integer.
const SOUND_HANDLE_SET_STATIC_ATTENUATION: u32 = 0x00ad_89b0;
/// `BSSoundHandle::SetCompletionCallback` (Xbox PDB): the callback and a
/// word.
const SOUND_HANDLE_SET_COMPLETION_CALLBACK: u32 = 0x00ad_8e60;
/// `MobileObject::SayToCallBack` (Xbox PDB), cdecl: a form id and a flag.
const SAY_TO_CALLBACK: u32 = 0x0093_6a20;
/// `BSSoundHandle::GetDuration` (Xbox PDB).
const SOUND_HANDLE_GET_DURATION: u32 = 0x00ad_8b30;
/// `Actor::AlwaysShowSubtitles` (Xbox PDB).
const ACTOR_ALWAYS_SHOW_SUBTITLES: u32 = 0x008c_1bc0;
/// `Interface::ShowSubtitle` (Xbox PDB), cdecl: the text, the sound handle
/// and the position by value (three words each), a word and a flag.
const INTERFACE_SHOW_SUBTITLE: u32 = 0x0070_5210;
/// `DialogueItem` destructor (`005c90d0`): thiscall, takes the delete flag.
const DIALOGUE_ITEM_DESTROY: u32 = 0x005c_90d0;
/// Setting whose byte forces the subtitles on (`0x011d8928`).
const SETTING_SHOW_SUBTITLES: u32 = 0x011d_8928;
/// Setting whose `float` scales the static attenuation (`0x011e060c`).
const SETTING_STATIC_ATTENUATION: u32 = 0x011e_060c;
/// Process slot `0x490` (takes a flag and a sound handle by value).
const PROCESS_SLOT_490: u32 = 0x490;

/// A copy of the sound handle at `handle` made by the assignment function
/// (`00418900`) into a fresh 12-byte block, as the code builds the by-value
/// argument on its stack; returns the three words.
fn sound_handle_words(e: &mut Engine, handle: u32) -> [u32; 3] {
    e.with_stack(12, |e, copy| {
        e.call(SOUND_HANDLE_ASSIGN, &args![copy, handle]);
        [
            e.mem.u32(copy.addr()),
            e.mem.u32(copy.addr() + 4),
            e.mem.u32(copy.addr() + 8),
        ]
    })
}

// Translated from 008a1bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the actor say `topic` to `speaker_arg`: builds the dialogue item,
/// stops the current dialogue, records the topic and its info on the actor,
/// runs the info's result, and for the item's current response plays its
/// voice file (as a static sound when `static_sound` is set, otherwise at the
/// actor's position following its node) and shows its subtitle (unless
/// `no_subtitle`). The sound handle is returned through `result` (copied
/// into it); the return value is `result`. `+0x7f` and `+0x80` of the actor
/// are bytes the code clears when it starts the line.
/// C++ exception unwinding is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_008a1bd0(
    e: &mut Engine,
    this: Ptr<Actor>,
    result: Ptr,
    topic: Ptr,
    speaker_arg: u32,
    static_sound: u8,
    no_subtitle: u8,
    _unused_6: u32,
    notify_when_done: u8,
) -> u32 {
    e.with_stack(12, |e, sound| {
        let sound = sound.addr();
        e.call(SOUND_HANDLE_CONSTRUCT, &args![sound]);
        let item = e
            .call(
                TOPIC_CREATE_DIALOGUE_ITEM,
                &args![topic, this, speaker_arg, 0u32, 0u32, 0u32],
            )
            .u32();
        e.call(MOBILE_OBJECT_STOP_CURRENT_DIALOGUE, &args![this]);
        e.call(REFR_SET_SAY_TO_TOPIC, &args![this, topic]);
        if item != 0 {
            say_item(
                e,
                this,
                item,
                sound,
                speaker_arg,
                static_sound,
                no_subtitle,
                notify_when_done,
            );
            e.call(DIALOGUE_ITEM_DESTROY, &args![item, 1u32]);
        }
        e.call(SOUND_HANDLE_ASSIGN, &args![result, sound]);
        e.call(SOUND_HANDLE_DESTROY, &args![sound]);
        result.addr()
    })
}

/// The part of [`fn_008a1bd0`] that runs when the dialogue item exists.
#[allow(clippy::too_many_arguments)]
fn say_item(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: u32,
    sound: u32,
    speaker_arg: u32,
    static_sound: u8,
    no_subtitle: u8,
    notify_when_done: u8,
) {
    let info = e.call(REFR_FORM_ID, &args![item]).u32();
    e.call(REFR_SET_SAY_TO_TOPIC_INFO, &args![this, info]);
    if info != 0 {
        e.call(TOPIC_INFO_ADD_TOPIC_LIST, &args![info]);
        e.call(TOPIC_INFO_RUN_RESULT, &args![info, 0u32, this]);
    }
    e.call(DIALOGUE_ITEM_FIRST_RESPONSE, &args![item]);
    let response = e
        .call(DIALOGUE_ITEM_GET_CURRENT_RESPONSE, &args![item])
        .u32();
    if response == 0 {
        return;
    }
    let mut speaking = true;
    let form = e.call(REFR_BASE_FORM, &args![response]).u32();
    if form != 0 && e.mem.u8(this.addr() + 0x7f) != 0 {
        let form = e.call(REFR_BASE_FORM, &args![response]).u32();
        let form_id = e.call(REFR_FORM_ID, &args![form]).u32();
        e.with_stack(12, |e, filled| {
            e.call(
                MOBILE_OBJECT_FILL_SOUND_HANDLE,
                &args![this, filled, form_id, 0u32, 0x4000_0102u32, 1u32],
            );
            let audio = e.call(AUDIO_MANAGER_INSTANCE, &args![]).u32();
            let this_id = e.call(REFR_FORM_ID, &args![this]).u32();
            let first = e.call(ENTRY_FIRST_EXTRA_LIST, &args![filled]).u32();
            e.call(
                AUDIO_MANAGER_SET_COMPLETION_CALLBACK,
                &args![audio, first, SAY_TO_COMPLETION_CALLBACK, this_id],
            );
            e.mem.set_u8(this.addr() + 0x7f, 0);
            speaking = false;
            e.call(SOUND_HANDLE_DESTROY, &args![filled]);
        });
    }
    if speaking && e.mem.u8(this.addr() + 0x7f) != 0 {
        let holder = e.call(RESPONSE_SOUND_HOLDER, &args![response]).u32();
        if e.call(ENTRY_FIRST_EXTRA_LIST, &args![holder]).u32() == 0 {
            if info != 0 && notify_when_done != 0 {
                let this_id = e.call(REFR_FORM_ID, &args![this]).u32();
                e.call(SAY_TO_CALLBACK, &args![this_id, 1u32]);
            }
        } else {
            let holder = e.call(RESPONSE_SOUND_HOLDER, &args![response]).u32();
            let file_name = e.call(ENTRY_FIRST_EXTRA_LIST, &args![holder]).u32();
            e.with_stack(0x200 + 12, |e, block| {
                let buffer = block.addr() + 12;
                let found_handle = block.addr();
                e.call(COPY_STRING_LIMITED, &args![buffer, 0x200u32, file_name]);
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let flags = if static_sound != 0 { 0x105u32 } else { 0x106 };
                let found = e
                    .call(
                        AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
                        &args![audio, found_handle, buffer, flags, 0u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![sound, found]);
                e.call(SOUND_HANDLE_DESTROY, &args![found_handle]);
            });
            if static_sound == 0 {
                let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
                let x = e.mem.u32(position);
                let y = e.mem.u32(position + 4);
                let z = e.mem.u32(position + 8);
                e.call(SOUND_HANDLE_SET_POSITION_XYZ, &args![sound, x, y, z]);
                let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
                e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![sound, node]);
            } else {
                let scale_ptr = e
                    .call(
                        SETTING_GET_VALUE_POINTER,
                        &args![SETTING_STATIC_ATTENUATION],
                    )
                    .u32();
                let hundred: f64 = e.global(HUNDRED);
                let value = e.mem.f32(scale_ptr) as f64 * hundred;
                let attenuation = e.call(FTOL, &args![value]).u32();
                e.call(
                    SOUND_HANDLE_SET_STATIC_ATTENUATION,
                    &args![sound, attenuation],
                );
            }
            e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
            if notify_when_done != 0 {
                let this_id = e.call(REFR_FORM_ID, &args![this]).u32();
                e.call(
                    SOUND_HANDLE_SET_COMPLETION_CALLBACK,
                    &args![sound, SAY_TO_CALLBACK, this_id],
                );
            }
            e.mem.set_u8(this.addr() + 0x80, 0);
            let words = sound_handle_words(e, sound);
            let process = e.get(this, Actor::pCurrentProcess).addr();
            e.vcall(
                process,
                PROCESS_SLOT_490,
                &args![0u32, words[0], words[1], words[2]],
            );
            // The duration is converted to a `float` local the code never
            // reads again.
            e.call(SOUND_HANDLE_GET_DURATION, &args![sound]);
        }
    }
    let always = e
        .call(SETTING_GET_BOOL_POINTER, &args![SETTING_SHOW_SUBTITLES])
        .u32();
    if (e.mem.u8(always) != 0 || e.call(ACTOR_ALWAYS_SHOW_SUBTITLES, &args![this]).bool())
        && no_subtitle == 0
    {
        let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
        let at = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        let words = sound_handle_words(e, sound);
        let text_holder = e.call(LIST_ITEM_SLOT, &args![response]).u32();
        let text = e.call(ENTRY_FIRST_EXTRA_LIST, &args![text_holder]).u32();
        e.call(
            INTERFACE_SHOW_SUBTITLE,
            &args![
                text,
                words[0],
                words[1],
                words[2],
                at[0],
                at[1],
                at[2],
                speaker_arg,
                1u32
            ],
        );
    }
}
/// Actor slot `0x1e8`: returns the node the head node is searched under.
const ACTOR_SLOT_1E8_NODE: u32 = 0x1e8;
/// `1000.0` (`double`).
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
/// `0.5` (`double`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// Settings object whose `float` is subtracted from 1000 in the head track
/// score (`0x011cd114`).
const SETTING_11CD114: u32 = 0x011c_d114;
/// Settings object whose `float` is compared with the distance to the
/// target (`0x011cd750`).
const SETTING_11CD750: u32 = 0x011c_d750;
/// Process slot `0x68c` (an actor the process remembers).
const PROCESS_SLOT_68C: u32 = 0x68c;
/// `0043_0830`: thiscall on an actor, returns a pointer to its rotation
/// (`+8` is the heading).
const ACTOR_ROTATION_POINTER: u32 = 0x0043_0830;
/// `pi` (`double`).
const PI_DOUBLE: u32 = 0x0101_ff40;
/// `2 * pi` (`double`).
const TWO_PI_DOUBLE: u32 = 0x0101_ff48;
/// `-pi` (`double`).
const MINUS_PI_DOUBLE: u32 = 0x0101_ff58;
/// `pi` (`float`), the turn limit towards the actor the process targets.
const PI_FLOAT: u32 = 0x0102_b3c8;
/// `2 * pi / 3` (`float`), the turn limit towards other actors.
const TURN_LIMIT_FLOAT: u32 = 0x0108_4d88;
/// `00560d30`: thiscall on a 4-`float` vector, takes an index; returns a
/// pointer to that component.
const FLOAT_ARRAY_SLOT: u32 = 0x0056_0d30;
/// Settings object whose `float` `fn_008a2d20` returns (`0x011d5788`).
const SETTING_11D5788: u32 = 0x011d_5788;
/// Process slot `0x278`: the entry of the object the process is working on.
const PROCESS_SLOT_278: u32 = 0x278;
/// Package method `00671d10`: the package's target holder.
const PACKAGE_GET_TARGET_OBJECT: u32 = 0x0067_1d10;
/// `PackageTarget::GetTargType` (Xbox PDB).
const PACKAGE_TARGET_GET_TARG_TYPE: u32 = 0x0051_9b00;
/// `PackageTarget::GetTargObject` (Xbox PDB).
const PACKAGE_TARGET_GET_TARG_OBJECT: u32 = 0x0068_0050;
/// `PackageTarget::GetTargObjectType` (Xbox PDB): thiscall on the target,
/// takes a pointer to a word.
const PACKAGE_TARGET_GET_TARG_OBJECT_TYPE: u32 = 0x0068_0080;
/// `InventoryChanges::GetObjectbyPackObjType` (Xbox PDB): thiscall on the
/// changes, takes the object type result and a pointer to a word.
const INVENTORY_CHANGES_GET_OBJECT_BY_PACK_OBJ_TYPE: u32 = 0x004c_6a10;
/// Reference slot `0xe4` (a test on the target object).
const REFERENCE_SLOT_E4: u32 = 0xe4;
/// `TESObjectREFR::GetInventoryItem` (Xbox PDB): thiscall on the reference,
/// takes the base form and a form id (or an object and 0).
const REFR_GET_INVENTORY_ITEM_FN: u32 = 0x0057_6260;
/// `0047c850`: thiscall test on the object the pointer at `0x011de45c`
/// holds.
const SINGLETON_TEST_47C850: u32 = 0x0047_c850;
/// The pointer `fn_008a2ed0` tests (`0x011de45c`).
const SINGLETON_11DE45C: u32 = 0x011d_e45c;
/// Acquire object slot `0x678`: the actor it targets.
const ACQUIRE_OBJECT_SLOT_678: u32 = 0x678;
/// Settings object whose `float` is the head tracking distance
/// (`0x011cd938`).
const SETTING_11CD938: u32 = 0x011c_d938;
/// `004ab230`: thiscall on the node `Actor` slot `0x1e8` returns, takes a
/// word; returns a node.
const NODE_FN_4AB230: u32 = 0x004a_b230;
/// `NiObjectNET::GetController` (Xbox PDB): thiscall on the object, takes a
/// controller type descriptor.
const NI_OBJECT_NET_GET_CONTROLLER: u32 = 0x00a5_c570;
/// The controller type descriptor `fn_008a2fa0` asks for (`0x011f36ac`).
const CONTROLLER_TYPE_11F36AC: u32 = 0x011f_36ac;
/// `00537bd0`: thiscall on a controller, returns the object whose slot
/// `0x8c` finds the head node.
const CONTROLLER_OWNER_537BD0: u32 = 0x0053_7bd0;
/// Slot `0x8c` of that object (takes the value of `fn_008a30f0`).
const CONTROLLER_OWNER_SLOT_8C: u32 = 0x8c;
/// `NiPoint3::operator+=` (`0063c8a0`): thiscall on a 3-`float` vector, adds
/// the vector at the pointer to it.
const VECTOR_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `bhkRagdollController::GetEyeOffset` (Xbox PDB): thiscall on the
/// controller, fills the 12-byte vector at the pointer; returns a byte.
const RAGDOLL_GET_EYE_OFFSET: u32 = 0x00c7_57b0;
/// Global words the eye position starts from (`0x011f426c`, three
/// `float`s).
const EYE_POSITION_DEFAULT: u32 = 0x011f_426c;
/// `0.9` (`double`), the share of the height the eye sits at.
const EYE_HEIGHT_SHARE: u32 = 0x0106_b9e8;
/// Value `fn_008a30f0` returns (`0x011c61ac`).
const GLOBAL_11C61AC: u32 = 0x011c_61ac;
/// Settings object whose `float` is the head tracking range (`0x011cd2f4`).
const SETTING_11CD2F4: u32 = 0x011c_d2f4;
/// `PlayerCharacter::IsPlayerCharacterInCombat` (Xbox PDB): thiscall on the
/// player, takes a pointer.
const PLAYER_IS_IN_COMBAT: u32 = 0x0095_3c50;
/// `0067a690`: test on a package.
const PACKAGE_TEST_67A690: u32 = 0x0067_a690;
/// `00888a50`: test on the ragdoll controller.
const RAGDOLL_TEST_888A50: u32 = 0x0088_8a50;
/// `00c75580`: thiscall on the ragdoll controller, takes `1`.
const RAGDOLL_FN_C75580: u32 = 0x00c7_5580;
/// `Actor::IsVisible` (Xbox PDB): takes a flag number.
const ACTOR_IS_VISIBLE: u32 = 0x008b_0190;
/// Process slot `0x264` (returns a `float`).
const PROCESS_SLOT_264: u32 = 0x264;
/// Process slot `0x268` (takes a `float`).
const PROCESS_SLOT_268: u32 = 0x268;
/// Process slot `0x2d8` (a test).
const PROCESS_SLOT_2D8: u32 = 0x2d8;
/// Process slot `0x30c` (a test).
const PROCESS_SLOT_30C_TEST: u32 = 0x30c;
/// Process slot `0x668` (a test).
const PROCESS_SLOT_668: u32 = 0x668;
/// Process slot `0x624` (takes the new target).
const PROCESS_SLOT_624: u32 = 0x624;
/// Counter of head tracking updates (`0x011df674`).
const HEAD_TRACK_UPDATE_COUNTER: u32 = 0x011d_f674;
/// Settings object whose integer is the updates per frame limit
/// (`0x011df848`).
const SETTING_INT_11DF848: u32 = 0x011d_f848;
/// Settings objects for the random target timer (`0x011df828` upper,
/// `0x011df810` lower).
const SETTING_11DF828: u32 = 0x011d_f828;
/// See [`SETTING_11DF828`].
const SETTING_11DF810: u32 = 0x011d_f810;
/// Settings object whose `float` the process timer is set to after a new
/// target (`0x011cdbb8`).
const SETTING_11CDBB8: u32 = 0x011c_dbb8;
/// Settings objects for the random head track timer (`0x011df78c` upper,
/// `0x011df700` lower).
const SETTING_11DF78C: u32 = 0x011d_f78c;
/// See [`SETTING_11DF78C`].
const SETTING_11DF700: u32 = 0x011d_f700;
/// `0.8` (`double`).
const HEAD_TURN_SHARE: u32 = 0x0102_17d8;
/// Settings object whose integer is the head turn limit in degrees
/// (`0x011cd480`).
const SETTING_INT_11CD480: u32 = 0x011c_d480;
/// Settings object whose integer is the head turn limit in degrees while the
/// animation flags `0x30` are set (`0x011cda1c`).
const SETTING_INT_11CDA1C: u32 = 0x011c_da1c;
/// `GetZAngleFromVector` (Xbox PDB), cdecl: the vector; returns a `float`.
const GET_Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// `004b15e0`, cdecl: the heading, the angle and a pointer to a `float`;
/// returns a `float` angle difference.
const ANGLE_DIFFERENCE: u32 = 0x004b_15e0;
/// `00408840`, cdecl: the absolute value of a `float` (`float` result).
const ABSOLUTE_VALUE: u32 = 0x0040_8840;
/// `00408820`, cdecl: the absolute value of a `float` (`float` result).
const ABSOLUTE_VALUE_ALT: u32 = 0x0040_8820;
/// `008bb520`: thiscall on the actor, takes a position (three words) and a
/// flag: turns the actor towards the point.
const ACTOR_TURN_TOWARDS: u32 = 0x008b_b520;
/// Settings object whose byte forces the head tracking on (`0x011df6d0`).
const SETTING_BOOL_11DF6D0: u32 = 0x011d_f6d0;
/// `004ade00`, cdecl: the actor's 3D node and the value of `fn_008a30f0`;
/// returns a node.
const NODE_FN_4ADE00: u32 = 0x004a_de00;
/// The controller type descriptor the head tracking asks for (`0x011f3e6c`).
const CONTROLLER_TYPE_11F3E6C: u32 = 0x011f_3e6c;
/// Controller slot `0xc4` (takes `0`).
const CONTROLLER_SLOT_C4: u32 = 0xc4;
/// `0045bad0`, cdecl: the descriptor `0x011f3e58` and the value of slot
/// `0xc4`.
const CONTROLLER_MATCH_45BAD0: u32 = 0x0045_bad0;
/// The descriptor passed to [`CONTROLLER_MATCH_45BAD0`] (`0x011f3e58`).
const CONTROLLER_DESCRIPTOR_11F3E58: u32 = 0x011f_3e58;
/// `00508100`: thiscall, returns a `float` (the controller's time).
const CONTROLLER_TIME_508100: u32 = 0x0050_8100;
/// `00457990`: thiscall on a vector, returns its length (`float`).
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// Actor slot `0x194` (the eye position; see [`fn_008a2fa0`]).
const ACTOR_SLOT_194: u32 = 0x194;
/// Actor slot `0xfc` (a test on the tracked actor).
const ACTOR_SLOT_FC: u32 = 0xfc;
/// Form type byte the head tracking leaves alone (`0x1c`; its name is not
/// confirmed).
const FORM_TYPE_NUMBER_1C: u32 = 0x1c;
/// `Actor::IsPathingComplete` (Xbox PDB).
const ACTOR_IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
/// `00717e50`: the process lists' process array getter (no stack
/// arguments).
const PROCESS_LISTS_GET_ARRAY: u32 = 0x0071_7e50;
/// `005be5c0`: thiscall on the array, takes the level; returns the count.
const PROCESS_ARRAY_COUNT: u32 = 0x005b_e5c0;
/// `00968670`: thiscall on the array, takes an index; returns the actor.
const PROCESS_ARRAY_GET_ACTOR: u32 = 0x0096_8670;
/// `005ae3d0`: thiscall on a list, appends the value at the pointer.
const LIST_APPEND: u32 = 0x005a_e3d0;
/// `0096a2d0`: thiscall on a list head, initializes it.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0046ffb0`: thiscall on a list head, the destructor.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `008256d0`: thiscall test on a list node (true when the walk stops).
const LIST_NODE_STOP_TEST: u32 = 0x0082_56d0;
/// `005b7470`: returns the byte at `0x0119b4e0`.
const GLOBAL_FLAG_5B7470: u32 = 0x005b_7470;
/// Face animation data slot `0xb4`.
const FACE_ANIMATION_SLOT_B4: u32 = 0xb4;
/// Face animation data slot `0x114` (takes a number and a `float`).
const FACE_ANIMATION_SLOT_114: u32 = 0x114;
/// Face animation data slot `0x11c` (takes a value).
const FACE_ANIMATION_SLOT_11C: u32 = 0x11c;
/// Process slot `0x48c` (fills a sound handle).
const PROCESS_SLOT_48C: u32 = 0x48c;
/// Process slot `0x2e8` (a test).
const PROCESS_SLOT_2E8: u32 = 0x2e8;
/// `BSSoundHandle::IsValid` (Xbox PDB).
const SOUND_HANDLE_IS_VALID_FN: u32 = 0x00ad_8ce0;
/// `TESAnimGroup::IsAttackAction` (Xbox PDB), cdecl: the group.
const IS_ATTACK_ACTION: u32 = 0x005f_2540;
/// `1.0` as a `float` word.
const FLOAT_ONE_BITS: u32 = 0x3f80_0000;
/// Distance setting for the player's head tracking (`0x01013d84`, `float`).
const PLAYER_TRACK_DISTANCE: u32 = 0x0101_3d84;
/// Distance for other actors' head tracking (`0x01016410`, `float`).
const OTHER_TRACK_DISTANCE: u32 = 0x0101_6410;
/// Process slot `0x674` (takes a number).
const PROCESS_SLOT_674: u32 = 0x674;
/// `PackageLocation` holder getter (`0055b980`).
const PACKAGE_GET_LOCATION_FN: u32 = 0x0055_b980;
/// Second package location getter (`00672dd0`).
const PACKAGE_GET_LOCATION_B: u32 = 0x0067_2dd0;
/// `PackageLocation::GetLocReference` (Xbox PDB).
const PACKAGE_LOCATION_GET_LOC_REFERENCE: u32 = 0x0067_f390;
/// `PackageLocation::GetLocType` (Xbox PDB).
const PACKAGE_LOCATION_GET_LOC_TYPE_FN: u32 = 0x0067_8ca0;
/// `TESObjectREFR::IsFurniture` (Xbox PDB).
const REFR_IS_FURNITURE: u32 = 0x0056_8680;
/// `009dcb30`: thiscall test on the actor mover (`Actor + 0x190`).
const ACTOR_MOVER_TEST_9DCB30: u32 = 0x009d_cb30;
/// A form pointer the packages compare locations with (`0x011ca248`).
const FORM_POINTER_11CA248: u32 = 0x011c_a248;
/// `008d0430`: thiscall on the process, returns an object.
const PROCESS_OBJECT_8D0430: u32 = 0x008d_0430;
/// `009f4300`: thiscall on that object, takes the actor and a flag.
const OBJECT_TEST_9F4300: u32 = 0x009f_4300;
/// `00639b40`: thiscall on that object, returns a reference.
const OBJECT_REFERENCE_639B40: u32 = 0x0063_9b40;
/// `ACTOR_FORM_TEST_FLAGS` (`00461580`): thiscall, takes a flag mask.
const FORM_TEST_FLAGS_FN: u32 = 0x0046_1580;
/// `Actor` base form accessor (`004181e0`).
const ACTOR_BASE_FORM_FN: u32 = 0x0041_81e0;
/// Actor slot `0x21c` (a test).
const ACTOR_SLOT_21C_TEST: u32 = 0x21c;
/// Actor slot `0x1e4` (the animation).
const ACTOR_SLOT_1E4: u32 = 0x1e4;

/// The `float` of the setting object at `setting` (`00403e20` returns a
/// pointer to it).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_GET_VALUE_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

/// Whether the head tracking controller of the actor's 3D object blocks the
/// tracking: the actor's node (slot `0x1e8`, or for slot `0x21c` actors the
/// node found by `004ade00`) has a controller of the type `0x011f3e6c` whose
/// value matches `0x011f3e58` and whose time is at least `90.0`.
fn head_tracking_controller_blocks(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let mut node = 0u32;
    if e.vcall(this.addr(), ACTOR_SLOT_1E8_NODE, &args![]).u32() != 0 {
        let owner = e.vcall(this.addr(), ACTOR_SLOT_1E8_NODE, &args![]).u32();
        node = e.call(NODE_FN_4AB230, &args![owner, 0u32]).u32();
    } else if e.vcall(this.addr(), ACTOR_SLOT_21C_TEST, &args![]).bool() {
        let key = fn_008a30f0(e);
        let body = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
        node = e.call(NODE_FN_4ADE00, &args![body, key]).u32();
    }
    if node == 0 {
        return false;
    }
    let controller = e
        .call(
            NI_OBJECT_NET_GET_CONTROLLER,
            &args![node, CONTROLLER_TYPE_11F3E6C],
        )
        .u32();
    if controller == 0 {
        return false;
    }
    let value = e.vcall(controller, CONTROLLER_SLOT_C4, &args![0u32]).u32();
    if !e
        .call(
            CONTROLLER_MATCH_45BAD0,
            &args![CONTROLLER_DESCRIPTOR_11F3E58, value],
        )
        .bool()
    {
        return false;
    }
    let time = e.call(CONTROLLER_TIME_508100, &args![value]).f64();
    time >= e.global::<f64>(NINETY)
}

// Translated from 008a2d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` of the setting at `0x011d5788`.
pub fn fn_008a2d20(e: &mut Engine) -> f32 {
    setting_float(e, SETTING_11D5788)
}

// Translated from 008a2d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inventory item (`TESObjectREFR::GetInventoryItem`) of the object the
/// actor's current package targets: for target types 0 and 3 the process'
/// object (slot `0x278`), for type 1 the target object when it passes the
/// test of its slot `0xe4`, for type 2 the object the container changes find
/// for the target's object type; 0 without a package.
pub fn fn_008a2d40(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mut item_form = 0u32;
    let mut target_object = 0u32;
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process != 0 && e.vcall(process, PROCESS_SLOT_22C, &args![]).u32() != 0 {
        let package = e.vcall(process, PROCESS_SLOT_22C, &args![]).u32();
        let target = e.call(PACKAGE_GET_TARGET_OBJECT, &args![package]).u32();
        if target != 0 {
            match e.call(PACKAGE_TARGET_GET_TARG_TYPE, &args![target]).u32() {
                0 | 3 => {
                    let entry = e.vcall(process, PROCESS_SLOT_278, &args![]).u32();
                    item_form = e.call(ENTRY_FORM, &args![entry]).u32();
                }
                1 => {
                    if e.call(PACKAGE_TARGET_GET_TARG_OBJECT, &args![target]).u32() != 0 {
                        let object = e.call(PACKAGE_TARGET_GET_TARG_OBJECT, &args![target]).u32();
                        if e.vcall(object, REFERENCE_SLOT_E4, &args![]).bool() {
                            target_object =
                                e.call(PACKAGE_TARGET_GET_TARG_OBJECT, &args![target]).u32();
                        }
                    }
                }
                2 => {
                    let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
                    if e.call(GET_CONTAINER_CHANGES, &args![extra]).u32() != 0 {
                        e.with_stack(4, |e, word| {
                            let object_type = e
                                .call(PACKAGE_TARGET_GET_TARG_OBJECT_TYPE, &args![target, word])
                                .u32();
                            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
                            let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).u32();
                            target_object = e
                                .call(
                                    INVENTORY_CHANGES_GET_OBJECT_BY_PACK_OBJ_TYPE,
                                    &args![changes, object_type, word],
                                )
                                .u32();
                        });
                    }
                }
                _ => {}
            }
        }
    }
    if item_form != 0 {
        let form_id = e.call(REFR_FORM_ID, &args![item_form]).u32();
        let base = e.call(REFR_BASE_FORM, &args![item_form]).u32();
        e.call(REFR_GET_INVENTORY_ITEM_FN, &args![this, base, form_id])
            .u32()
    } else if target_object != 0 {
        e.call(
            REFR_GET_INVENTORY_ITEM_FN,
            &args![this, target_object, 0u32],
        )
        .u32()
    } else {
        0
    }
}

// Translated from 008a2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// What the actor's acquire object reports as the actor it targets (slot
/// `0x678`) when the test `0047c850` on the object at `0x011de45c` is false;
/// the checks of the current package target distance (`0x011cd938`) do not
/// change the result, which is 0 otherwise.
pub fn fn_008a2ed0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let singleton = e.global::<u32>(SINGLETON_11DE45C);
    if e.call(SINGLETON_TEST_47C850, &args![singleton]).bool() {
        return 0;
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let targeted = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
        if targeted != 0 {
            return targeted;
        }
        if e.call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![this])
            .u32()
            != 0
            && this.addr() != e.global::<u32>(PLAYER_CHARACTER)
        {
            let package_target = e.call(ACTOR_GET_CURRENT_PACKAGE_TARGET, &args![this]).u32();
            if package_target != 0 {
                // The distance is compared with the setting, but the
                // result of the comparison is not used.
                e.call(
                    REFR_GET_DISTANCE_FROM_REFERENCE,
                    &args![package_target, this, 0u32, 0u32],
                );
                setting_float(e, SETTING_11CD938);
            }
        }
    }
    0
}

// Translated from 008a2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the position the actor looks from to `out` (three `float`s) and
/// returns it. The position is the default (`0x011f426c`) plus the world
/// translation of the head node (found through slot `0x1e8`'s node or the
/// controller of the actor's 3D object), with the eye offset of the ragdoll
/// controller replacing the height when there is one; without a head node it
/// is the default raised by 0.9 of the actor's height plus the actor's
/// position.
pub fn fn_008a2fa0(e: &mut Engine, this: Ptr<Actor>, out: Ptr) -> u32 {
    e.with_stack(0x20, |e, block| {
        let position = block.addr();
        let eye_offset = block.addr() + 0x10;
        for i in 0..3 {
            let word = e.mem.u32(EYE_POSITION_DEFAULT + 4 * i);
            e.mem.set_u32(position + 4 * i, word);
        }
        let owner = e.vcall(this.addr(), ACTOR_SLOT_1E8_NODE, &args![]).u32();
        let mut head = 0u32;
        if owner != 0 {
            head = e.call(NODE_FN_4AB230, &args![owner, 0u32]).u32();
        }
        if head == 0 {
            let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
            if node != 0 {
                let controller = e
                    .call(
                        NI_OBJECT_NET_GET_CONTROLLER,
                        &args![node, CONTROLLER_TYPE_11F36AC],
                    )
                    .u32();
                if controller != 0 {
                    let finder = e.call(CONTROLLER_OWNER_537BD0, &args![controller]).u32();
                    if finder != 0 {
                        let key = fn_008a30f0(e);
                        head = e.vcall(finder, CONTROLLER_OWNER_SLOT_8C, &args![key]).u32();
                    }
                }
            }
        }
        if head != 0 {
            let translation = e.call(NODE_WORLD_TRANSLATE, &args![head]).u32();
            e.call(VECTOR_ADD_ASSIGN, &args![position, translation]);
            e.call(LIST_ITEM_SLOT, &args![eye_offset]);
            let ragdoll = e.get(this, Actor::pRagdollController).addr();
            if ragdoll != 0
                && e.call(RAGDOLL_GET_EYE_OFFSET, &args![ragdoll, eye_offset])
                    .bool()
            {
                let z = e.mem.u32(eye_offset + 8);
                e.mem.set_u32(position + 8, z);
            }
        } else {
            let height = e.call(ACTOR_GET_HEIGHT, &args![this]).f64();
            let share: f64 = e.global(EYE_HEIGHT_SHARE);
            let z = e.mem.f32(position + 8) as f64;
            e.mem.set_f32(position + 8, (height * share + z) as f32);
            let actor_position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
            e.call(VECTOR_ADD_ASSIGN, &args![position, actor_position]);
        }
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(out.addr() + 4 * i, word);
        }
        out.addr()
    })
}

// Translated from 008a30f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c61ac`.
pub fn fn_008a30f0(e: &mut Engine) -> u32 {
    e.global(GLOBAL_11C61AC)
}

// Translated from 008a3100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of the actor's head tracking (`dt` is the frame time).
/// Does nothing for the player or an actor without a process. With a package
/// the process reports (slot `0x27c`) that passes `0067a690` it only marks
/// the ragdoll controller (byte `+0xb2`). Otherwise it runs the timers,
/// picks a new target (`fn_008a3ed0`) when the update timer runs out,
/// turns the actor towards the target when needed, and either lets the
/// ragdoll controller look at the target (`fn_008a3b70`) or marks it.
pub fn fn_008a3100(e: &mut Engine, this: Ptr<Actor>, dt: f32) {
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if this.addr() == player_addr || process == 0 {
        return;
    }
    let ragdoll = e.get(this, Actor::pRagdollController);
    let package = e.vcall(process, PROCESS_SLOT_27C, &args![]).u32();
    if package != 0 && e.call(PACKAGE_TEST_67A690, &args![package]).bool() {
        if !ragdoll.is_null()
            && e.call(RAGDOLL_TEST_888A50, &args![ragdoll]).bool()
            && fn_008a3bd0(e, ragdoll) == 0
        {
            fn_008a3bf0(e, ragdoll, 1);
        }
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let controller = e
        .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
        .u32();
    if controller != 0 {
        let value = fn_008a3b50(e, Ptr::new(controller)) as f64;
        if value < zero {
            return;
        }
        let value = fn_008a3b50(e, Ptr::new(controller)) as f64;
        if (setting_float(e, SETTING_11CD2F4) as f64) < value {
            return;
        }
    }
    let update_timer = e.mem.f32(this.addr() + 0x74);
    e.mem
        .set_f32(this.addr() + 0x74, (update_timer as f64 - dt as f64) as f32);
    let process_timer = e.vcall(process, PROCESS_SLOT_264, &args![]).f64();
    let reduced = (process_timer - dt as f64) as f32;
    e.vcall(process, PROCESS_SLOT_268, &args![reduced]);
    let process_target = e.vcall(process, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
    let package = e.vcall(process, PROCESS_SLOT_27C, &args![]).u32();
    let package_is_type_2 = package != 0 && {
        let package = e.vcall(process, PROCESS_SLOT_27C, &args![]).u32();
        e.call(PACKAGE_TYPE, &args![package]).u32() == 2
    };
    if process_target == player_addr {
        let record = e
            .vcall(process, PROCESS_SLOT_504, &args![player_addr, 0u32])
            .u32();
        if record != 0
            && e.mem.i32(record + 8) <= 0
            && !e.vcall(process, PROCESS_SLOT_2D8, &args![]).bool()
            && !package_is_type_2
            && !e.vcall(process, PROCESS_SLOT_30C_TEST, &args![]).bool()
        {
            e.vcall(process, PROCESS_SLOT_660, &args![]);
        }
    }
    let allowed = e
        .call(SETTING_GET_INT_POINTER, &args![SETTING_INT_11DF848])
        .u32();
    let allowed = e.mem.i32(allowed);
    if (e.global::<u32>(HEAD_TRACK_UPDATE_COUNTER) as i32) > allowed {
        return;
    }
    let mut acquire = 0u32;
    if e.call(ACQUIRE_OBJECT_FIELD_28, &args![process]).u32() == 0 {
        acquire = e.get(this, Actor::pCurrentProcess).addr();
    }
    if acquire == 0 {
        return;
    }
    if e.call(ACTOR_SOUND_SOURCE, &args![this]).u32() == 0 {
        return;
    }
    if e.mem.u8(this.addr() + 0x15c) == 0 {
        e.mem.set_u8(this.addr() + 0x15c, 1);
    }
    if !e.call(ACTOR_IS_VISIBLE, &args![this, 7u32]).bool() {
        e.mem.set_u8(this.addr() + 0x15c, 0);
    }
    let face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![this]).u32();
    if face != 0 {
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![player_addr, this, 0u32, 0u32],
            )
            .f64();
        if (setting_float(e, SETTING_11CD2F4) as f64) < distance {
            return;
        }
    }
    let mut target = e.vcall(process, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
    if target != 0
        && e.vcall(target, ACTOR_SLOT_FC, &args![]).bool()
        && e.vcall(target, ACTOR_SLOT_1D0, &args![]).u32() == 0
    {
        e.vcall(process, PROCESS_SLOT_660, &args![]);
        target = 0;
    }
    let head_timer = e.mem.f32(this.addr() + 0x158);
    e.mem
        .set_f32(this.addr() + 0x158, (head_timer as f64 - dt as f64) as f32);
    if e.mem.f32(this.addr() + 0x74) as f64 <= zero {
        let counter: u32 = e.global(HEAD_TRACK_UPDATE_COUNTER);
        e.set_global(HEAD_TRACK_UPDATE_COUNTER, counter.wrapping_add(1));
        let upper = setting_float(e, SETTING_11DF828);
        let lower = setting_float(e, SETTING_11DF810);
        let random = e.call(RANDOM_FLOAT, &args![lower, upper]).f32();
        e.mem.set_f32(this.addr() + 0x74, random);
        if e.vcall(process, PROCESS_SLOT_668, &args![]).bool() {
            if (e.mem.f32(this.addr() + 0x158) as f64) < zero {
                e.mem.set_f32(this.addr() + 0x158, 0.0);
            }
            let chosen = fn_008a3ed0(e, this);
            if chosen != target {
                target = chosen;
                e.vcall(process, PROCESS_SLOT_624, &args![chosen]);
                let timer = setting_float(e, SETTING_11CDBB8);
                e.vcall(process, PROCESS_SLOT_268, &args![timer]);
                let upper = setting_float(e, SETTING_11DF78C);
                let lower = setting_float(e, SETTING_11DF700);
                let random = e.call(RANDOM_FLOAT, &args![lower, upper]).f32();
                e.mem.set_f32(this.addr() + 0x158, random);
            }
        }
    }
    e.with_stack(0x50, |e, block| {
        let position = block.addr();
        let difference = block.addr() + 0x10;
        let heading_out = block.addr() + 0x20;
        let offset = block.addr() + 0x30;
        let eye_buffer = block.addr() + 0x40;
        let look_at = if target != 0 {
            e.vcall(target, ACTOR_SLOT_POSITION, &args![]).u32()
        } else {
            EYE_POSITION_DEFAULT
        };
        for i in 0..3 {
            let word = e.mem.u32(look_at + 4 * i);
            e.mem.set_u32(position + 4 * i, word);
        }
        if target != 0 {
            if e.mem.u8(this.addr() + 0x15d) != 0
                && e.call(ACTOR_IS_PATHING_COMPLETE, &args![this]).bool()
            {
                let actor_position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
                e.call(
                    VECTOR_SUBTRACT,
                    &args![position, difference, actor_position],
                );
                let angle = e.call(GET_Z_ANGLE_FROM_VECTOR, &args![difference]).f32();
                e.mem.set_f32(heading_out, 0.0);
                let heading = e
                    .vcall(this.addr(), ACTOR_SLOT_GET_HEADING, &args![0u32])
                    .f32();
                let turned = e
                    .call(ANGLE_DIFFERENCE, &args![heading, angle, heading_out])
                    .f32();
                let degrees = e
                    .call(SETTING_GET_INT_POINTER, &args![SETTING_INT_11CD480])
                    .u32();
                let radians: f64 = e.global(DEGREES_TO_RADIANS);
                let share: f64 = e.global(HEAD_TURN_SHARE);
                let mut limit = (e.mem.i32(degrees) as f64 * radians * share) as f32;
                if fn_008a3b30(e, this) != 0 {
                    let degrees = e
                        .call(SETTING_GET_INT_POINTER, &args![SETTING_INT_11CDA1C])
                        .u32();
                    limit = (e.mem.i32(degrees) as f64 * radians * share) as f32;
                }
                let magnitude = e.call(ABSOLUTE_VALUE, &args![turned]).f64();
                if (limit as f64) < magnitude {
                    let words = [
                        e.mem.u32(position),
                        e.mem.u32(position + 4),
                        e.mem.u32(position + 8),
                    ];
                    e.call(
                        ACTOR_TURN_TOWARDS,
                        &args![this, words[0], words[1], words[2], 0u32],
                    );
                }
            }
            // The distance from the player to the target is compared with a
            // setting; the result is not used afterwards.
            e.call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![player_addr, target, 0u32, 1u32],
            );
            setting_float(e, SETTING_11CD750);
        }
        let mut busy = false;
        if e.call(PROCESS_LEVEL_NUMBER, &args![this]).u32() != 0
            && e.call(PROCESS_LEVEL_NUMBER, &args![this]).u32() != 5
        {
            busy = true;
        }
        let forced = e
            .call(SETTING_GET_BOOL_POINTER, &args![SETTING_BOOL_11DF6D0])
            .u32();
        if e.mem.u8(forced) != 0 {
            busy = true;
        }
        let process_now = e.get(this, Actor::pCurrentProcess).addr();
        if process_now != 0 && !package_is_type_2 {
            let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
            let group = e
                .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
                .u16();
            if e.call(IS_ATTACK_ACTION, &args![group as u32]).bool()
                || head_tracking_controller_blocks(e, this)
            {
                busy = true;
            }
        }
        let form = if target != 0 {
            e.call(REFR_BASE_FORM, &args![target]).u32()
        } else {
            0
        };
        let form_is_skipped =
            form != 0 && e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_NUMBER_1C;
        if !ragdoll.is_null() && !busy && target != 0 && target != this.addr() && !form_is_skipped {
            let actor_position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
            e.call(VECTOR_SUBTRACT, &args![actor_position, offset, position]);
            let length = e.call(VECTOR_LENGTH, &args![offset]).f32() as f64;
            let near = fn_008a3c10(e) as f64;
            if length <= near {
                let far = fn_008a3c30(e) as f64;
                if far < length {
                    fn_008a3bf0(e, ragdoll, 0);
                    e.call(RAGDOLL_FN_C75580, &args![ragdoll, 1u32]);
                    let eye = e.vcall(target, ACTOR_SLOT_194, &args![eye_buffer]).u32();
                    fn_008a3b70(e, ragdoll, eye, 1);
                }
            }
            return;
        }
        if !ragdoll.is_null()
            && e.call(RAGDOLL_TEST_888A50, &args![ragdoll]).bool()
            && fn_008a3bd0(e, ragdoll) == 0
        {
            fn_008a3bf0(e, ragdoll, 1);
        }
    });
}

// Translated from 008a3b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any of the animation flags `0x30` of the actor is set.
pub fn fn_008a3b30(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    (e.call(ACTOR_ANIM_FLAGS, &args![this]).u32() & 0x30 != 0) as u8
}

// Translated from 008a3b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `+0x5fc` (on the character controller).
pub fn fn_008a3b50(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x5fc)
}

// Translated from 008a3b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the ragdoll controller: copies the vector `source` into the vector at
/// `+0xd0` (`004a3e00`), zeroes its fourth component and, when `reset` is
/// set, calls `00c75580(1)` and clears the bytes at `+0xb2` and `+0x43`.
pub fn fn_008a3b70(e: &mut Engine, this: Ptr, source: u32, reset: u8) {
    e.call(VECTOR_ASSIGN, &args![this.addr() + 0xd0, source]);
    let slot = e
        .call(FLOAT_ARRAY_SLOT, &args![this.addr() + 0xd0, 3u32])
        .u32();
    e.mem.set_f32(slot, 0.0);
    if reset != 0 {
        e.call(RAGDOLL_FN_C75580, &args![this, 1u32]);
        e.mem.set_u8(this.addr() + 0xb2, 0);
        e.mem.set_u8(this.addr() + 0x43, 0);
    }
}

// Translated from 008a3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xb2`.
pub fn fn_008a3bd0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xb2)
}

// Translated from 008a3bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0xb2`.
pub fn fn_008a3bf0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xb2, value);
}

// Translated from 008a3c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` of the setting at `0x01267d08`.
pub fn fn_008a3c10(e: &mut Engine) -> f32 {
    setting_float(e, 0x0126_7d08)
}

// Translated from 008a3c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` of the setting at `0x01267d14`.
pub fn fn_008a3c30(e: &mut Engine) -> f32 {
    setting_float(e, 0x0126_7d14)
}

// Translated from 008a3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame face animation update: for an actor in process level 0 whose
/// character controller and distance to the player are within the head
/// tracking range, sets the face animation (slot `0xb4`, `0x114` or `0x11c`
/// of the face animation data) according to the global flag `0119b4e0`,
/// fleeing, combat and the process' sound state.
/// C++ exception unwinding (the sound handle's cleanup) is not translated.
/// The stack word (`RET 4`) is not read.
pub fn fn_008a3c50(e: &mut Engine, this: Ptr<Actor>, _unused_1: u32) {
    if e.call(MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, &args![this])
        .u32()
        != 0
    {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let controller = e
        .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
        .u32();
    if controller != 0 {
        let value = fn_008a3b50(e, Ptr::new(controller)) as f64;
        if value < zero {
            return;
        }
        let value = fn_008a3b50(e, Ptr::new(controller)) as f64;
        if (setting_float(e, SETTING_11CD2F4) as f64) < value {
            return;
        }
    }
    let face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![this]).u32();
    if face != 0 {
        let player_addr = e.global::<u32>(PLAYER_CHARACTER);
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![player_addr, this, 0u32, 0u32],
            )
            .f64();
        if (setting_float(e, SETTING_11CD2F4) as f64) < distance {
            return;
        }
    }
    if face == 0 {
        return;
    }
    if !e.call(GLOBAL_FLAG_5B7470, &args![]).bool() {
        e.vcall(
            face,
            FACE_ANIMATION_SLOT_B4,
            &args![0.0f32, 1u32, 0u32, 0u32, 0u32, 0u32],
        );
        return;
    }
    if e.call(ACTOR_IS_FLEEING, &args![this, 0u32]).bool() {
        e.vcall(face, FACE_ANIMATION_SLOT_114, &args![1u32, FLOAT_ONE_BITS]);
    } else if e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
        e.vcall(
            face,
            FACE_ANIMATION_SLOT_114,
            &args![0xeu32, FLOAT_ONE_BITS],
        );
    } else {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let set_value = if process == 0 {
            true
        } else {
            e.with_stack(12, |e, handle| {
                let found = e
                    .vcall(process, PROCESS_SLOT_48C, &args![handle, 0u32])
                    .u32();
                let valid = e.call(SOUND_HANDLE_IS_VALID_FN, &args![found]).bool();
                let idle = !valid && e.vcall(process, PROCESS_SLOT_2E8, &args![]).u32() == 0;
                e.call(SOUND_HANDLE_DESTROY, &args![handle]);
                idle
            })
        };
        if set_value {
            // 008a3e4f / 008a3e7d: the actor-value owner's +0x8 takes one word (RET 4);
            // the 0 pushed first is the last word of the face data's +0x11c (RET 8).
            let value = e.vcall(this.addr() + 0xa4, 0x08, &args![4u32]).u32();
            e.vcall(face, FACE_ANIMATION_SLOT_11C, &args![value, 0u32]);
        }
    }
}

// Translated from 008a3ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the actor to track with the head: the package's target when the
/// package (type 6 or 9) has one and it is closer than the limit (500 for the
/// player, 100 otherwise); else, of the high process actors and the player
/// that `fn_008a4810` allows, the one with the best `fn_008a46c0` score;
/// 0 when none.
/// C++ exception unwinding (the list's cleanup) is not translated.
pub fn fn_008a3ed0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let package = e
        .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![this])
        .u32();
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    if package != 0
        && (e.call(PACKAGE_TYPE, &args![package]).u32() == 6
            || e.call(PACKAGE_TYPE, &args![package]).u32() == 9)
        && e.call(PACKAGE_GET_TARGET_OBJECT, &args![package]).u32() != 0
    {
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let entry = e.vcall(process, PROCESS_SLOT_278, &args![]).u32();
        let form = e.call(ENTRY_FORM, &args![entry]).u32();
        if form != 0 {
            let limit = if form == player_addr {
                e.global::<f32>(PLAYER_TRACK_DISTANCE)
            } else {
                e.global::<f32>(OTHER_TRACK_DISTANCE)
            };
            let distance = e
                .call(
                    REFR_GET_DISTANCE_FROM_REFERENCE,
                    &args![this, form, 0u32, 1u32],
                )
                .f64();
            // The form is returned when the distance is below the limit.
            if distance < limit as f64 {
                return form;
            }
        }
    }
    e.with_stack(8, |e, list| {
        e.call(LIST_CONSTRUCT, &args![list]);
        let mut index = 0u32;
        loop {
            let array = e.call(PROCESS_LISTS_GET_ARRAY, &args![PROCESS_LISTS]).u32();
            let count = e.call(PROCESS_ARRAY_COUNT, &args![array, 0u32]).u32();
            if index >= count {
                break;
            }
            let array = e.call(PROCESS_LISTS_GET_ARRAY, &args![PROCESS_LISTS]).u32();
            let candidate = e.call(PROCESS_ARRAY_GET_ACTOR, &args![array, index]).u32();
            if actor_can_head_track(e, this, Ptr::new(candidate), 1) != 0 {
                e.with_stack(4, |e, holder| {
                    e.mem.set_u32(holder.addr(), candidate);
                    e.call(LIST_APPEND, &args![list, holder]);
                });
            }
            index = index.wrapping_add(1);
        }
        if actor_can_head_track(e, this, Ptr::new(player_addr), 1) != 0 {
            e.with_stack(4, |e, holder| {
                e.mem.set_u32(holder.addr(), player_addr);
                e.call(LIST_APPEND, &args![list, holder]);
            });
        }
        let mut best_score = 0.0f32;
        let mut best = 0u32;
        let mut node = list.addr();
        while node != 0 && !e.call(LIST_NODE_STOP_TEST, &args![node]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let candidate = e.mem.u32(slot);
            let score = fn_008a46c0(e, this, Ptr::new(candidate));
            if (best_score as f64) < score as f64 {
                best = candidate;
                best_score = score;
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        e.call(LIST_CLEAR, &args![list]);
        e.call(LIST_DESTRUCT, &args![list]);
        best
    })
}

// Translated from 008a40e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor may be told to start an idle or package behaviour that
/// involves `other` (the actor its process targets when 0): false for the
/// player, without a 3D object, when its base form passes `fn_008a46a0`, with
/// attached arrows, without a target or targeting itself, while not at the
/// end of its path, in combat, during an attack animation, while the head
/// tracking controller blocks it, when it cannot be seen by the player, and
/// in several package dependent cases; true otherwise.
pub fn fn_008a40e0(e: &mut Engine, this: Ptr<Actor>, other: u32) -> u8 {
    let mut other = other;
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() == player_addr {
        return 0;
    }
    if e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32() == 0 {
        return 0;
    }
    let base = e.call(ACTOR_BASE_FORM_FN, &args![this]).u32();
    if base != 0 && fn_008a46a0(e, Ptr::new(base + 0x30)) != 0 {
        return 0;
    }
    if other == 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        other = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
    }
    if e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0 {
        return 0;
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if e.vcall(acquire, ACQUIRE_OBJECT_SLOT_678, &args![]).u32() == 0 {
        return 0;
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if e.vcall(acquire, ACQUIRE_OBJECT_SLOT_678, &args![]).u32() == this.addr() {
        return 0;
    }
    if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![this]).bool() {
        return 0;
    }
    if e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
        return 0;
    }
    let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
    if animation != 0 {
        let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
        let group = e
            .call(ANIMATION_GROUP_OF_SLOT, &args![animation, 4u32])
            .u16();
        if e.call(IS_ATTACK_ACTION, &args![group as u32]).bool() {
            return 0;
        }
    }
    if head_tracking_controller_blocks(e, this) {
        return 0;
    }
    if e.call(ACTOR_IS_MOVING, &args![player_addr]).bool() && other == player_addr {
        let mut lvl = 0i32;
        e.with_stack(4, |e, flag| {
            e.mem.set_u8(flag.addr(), 1);
            let in_combat = e.call(PLAYER_IS_IN_COMBAT, &args![player_addr, 0u32]).u8();
            lvl = actor_get_detection_level_against_actor(
                e,
                this,
                0,
                Ptr::new(player_addr),
                flag,
                in_combat,
                0,
                0,
                Ptr::new(0),
            );
        });
        if lvl <= 0 {
            return 0;
        }
    }
    let process = e.get(this, Actor::pCurrentProcess).addr();
    let package = e.vcall(process, PROCESS_SLOT_22C, &args![]).u32();
    if package != 0 {
        let package_type = e.call(PACKAGE_TYPE, &args![package]).u32();
        // The compiler's switch on the package type: the case (handler) of each type; the types
        // past `0x1c` take the last case.
        let handler = match package_type {
            0 | 2 => 0,
            5 => 1,
            6 | 13..=14 => 2,
            8 | 15..=16 | 20 | 28 => 3,
            12 => 4,
            _ => 5,
        };
        match handler {
            0 => {
                if e.get(this, Actor::pCurrentProcess).addr() != 0
                    && e.call(ACTOR_ANIM_FLAGS, &args![this]).u32() & 0xf != 0
                {
                    return 0;
                }
            }
            1 => {
                if e.get(this, Actor::pCurrentProcess).addr() != 0
                    && e.call(ACTOR_ANIM_FLAGS, &args![this]).u32() & 0xf != 0
                {
                    return 0;
                }
                let animation = e.vcall(this.addr(), ACTOR_SLOT_1E4, &args![]).u32();
                if animation != 0
                    && !e
                        .call(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, &args![animation])
                        .bool()
                {
                    return 0;
                }
            }
            2 => {
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                if e.vcall(acquire, PROCESS_SLOT_674, &args![5u32]).u32() != 0 {
                    return 1;
                }
                let mut location_ref = 0u32;
                if e.call(PACKAGE_GET_LOCATION_FN, &args![package]).u32() != 0 {
                    let location = e.call(PACKAGE_GET_LOCATION_FN, &args![package]).u32();
                    location_ref = e
                        .call(PACKAGE_LOCATION_GET_LOC_REFERENCE, &args![location])
                        .u32();
                }
                if location_ref == 0 && e.call(PACKAGE_GET_LOCATION_B, &args![package]).u32() != 0 {
                    let location = e.call(PACKAGE_GET_LOCATION_B, &args![package]).u32();
                    location_ref = e
                        .call(PACKAGE_LOCATION_GET_LOC_REFERENCE, &args![location])
                        .u32();
                }
                if location_ref != 0 {
                    let form = e.call(REFR_BASE_FORM, &args![location_ref]).u32();
                    if form == e.global::<u32>(FORM_POINTER_11CA248) {
                        return 0;
                    }
                    let other_base = e.call(REFR_BASE_FORM, &args![other]).u32();
                    if e.call(FORM_TYPE, &args![other_base]).u32() == 0x30 {
                        let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
                        if animation != 0
                            && !e
                                .call(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, &args![animation])
                                .bool()
                        {
                            return 0;
                        }
                    }
                    if e.call(PACKAGE_GET_LOCATION_FN, &args![package]).u32() != 0 {
                        let location = e.call(PACKAGE_GET_LOCATION_FN, &args![package]).u32();
                        if e.call(PACKAGE_LOCATION_GET_LOC_TYPE_FN, &args![location])
                            .u32()
                            == 3
                        {
                            return 0;
                        }
                    }
                    if e.call(REFR_IS_FURNITURE, &args![other]).bool() {
                        return 0;
                    }
                    let mover = e.mem.u32(this.addr() + 0x190);
                    if e.call(ACTOR_MOVER_TEST_9DCB30, &args![mover]).bool() {
                        return 0;
                    }
                }
            }
            3 => return 0,
            4 => {
                if e.get(this, Actor::pCurrentProcess).addr() != 0
                    && e.call(ACTOR_ANIM_FLAGS, &args![this]).u32() & 0xf != 0
                {
                    return 0;
                }
                let process = e.get(this, Actor::pCurrentProcess).addr();
                let object = e.call(PROCESS_OBJECT_8D0430, &args![process]).u32();
                if object == 0
                    || !e
                        .call(OBJECT_TEST_9F4300, &args![object, this, 1u32])
                        .bool()
                {
                    return 0;
                }
                let reference = e.call(OBJECT_REFERENCE_639B40, &args![object]).u32();
                if reference != 0 {
                    if e.call(REFR_IS_FURNITURE, &args![reference]).bool() {
                        return 0;
                    }
                    let form = e.call(REFR_BASE_FORM, &args![reference]).u32();
                    if form != 0 {
                        if form == e.global::<u32>(FORM_POINTER_11CA248) {
                            return 0;
                        }
                        if e.call(FORM_TYPE, &args![form]).u32() == 0x30 {
                            return 0;
                        }
                    }
                }
                let animation = e.vcall(this.addr(), ACTOR_SLOT_1E4, &args![]).u32();
                if animation != 0
                    && !e
                        .call(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, &args![animation])
                        .bool()
                    && e.call(ENTRY_FORM, &args![object]).u32() == 3
                {
                    return 0;
                }
            }
            _ => {}
        }
    }
    1
}

// Translated from 008a46a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests the flag `0x40000000` of the object (`00461580`).
pub fn fn_008a46a0(e: &mut Engine, this: Ptr) -> u8 {
    e.call(FORM_TEST_FLAGS_FN, &args![this, 0x4000_0000u32])
        .u8()
}

// Translated from 008a46c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// How interesting `other` is to track: 0 without a usable acquire object;
/// otherwise `(1000 - distance) / (1000 - setting)` (`0x011cd114`), halved
/// without a line of sight to a target the slot `0x100` test accepts, times
/// the head track timer (`+0x158`) plus one when `other` is the actor the
/// acquire object targets, halved when `other` is fleeing.
pub fn fn_008a46c0(e: &mut Engine, this: Ptr<Actor>, other: Ptr<Actor>) -> f32 {
    let mut acquire = 0u32;
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let candidate = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if e.call(ACQUIRE_OBJECT_FIELD_28, &args![candidate]).u32() == 0 {
            acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        }
    }
    if acquire == 0 {
        return 0.0;
    }
    let mut tracked = 0u32;
    if e.vcall(other.addr(), ACTOR_SLOT_100, &args![]).bool() {
        tracked = other.addr();
    }
    let distance = e
        .call(
            REFR_GET_DISTANCE_FROM_REFERENCE,
            &args![other, this, 0u32, 0u32],
        )
        .f32();
    let mut line_of_sight = 1u8;
    if tracked != 0 {
        line_of_sight = e
            .call(
                ACTOR_LINE_OF_SIGHT,
                &args![this, 0u32, tracked, 1u32, 0u32, 0u32],
            )
            .u8();
    }
    let thousand: f64 = e.global(THOUSAND_DOUBLE);
    let numerator = thousand - distance as f64;
    let setting = setting_float(e, SETTING_11CD114) as f64;
    let mut score = (numerator / (thousand - setting)) as f32;
    let half: f64 = e.global(HALF_DOUBLE);
    if line_of_sight == 0 {
        score = (score as f64 * half) as f32;
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let targeted = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
    if other.addr() == targeted {
        let head_timer = e.mem.f32(this.addr() + 0x158);
        score = ((head_timer as f64 + 1.0) * score as f64) as f32;
    }
    if e.vcall(other.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        score = (score as f64 * half) as f32;
    }
    score
}

// Translated from 008a4810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CanHeadTrack` (Xbox PDB): whether `this` may turn its head to
/// `other`: not itself, `other` passes slot `0x100` and has a 3D object, is
/// within the head tracking distance (`0x011cd938`), is either under 100
/// units away, the player or the actor `this` targets (slot `0x68c`), is
/// within the turn limit of the heading (`pi` for the actor the process
/// targets, 120 degrees otherwise), and, when `check_detection` is set, can
/// be detected.
pub fn actor_can_head_track(
    e: &mut Engine,
    this: Ptr<Actor>,
    other: Ptr<Actor>,
    check_detection: u8,
) -> u8 {
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() == other.addr()
        || !e.vcall(other.addr(), ACTOR_SLOT_100, &args![]).bool()
        || e.call(FORM_FLAG_800, &args![other]).bool()
        || e.vcall(other.addr(), ACTOR_SLOT_1D0, &args![]).u32() == 0
    {
        return 0;
    }
    e.with_stack(0x10, |e, difference| {
        let this_point = e.call(ACTOR_VIEW_CONE_TARGET, &args![this]).u32();
        let other_point = e.call(ACTOR_VIEW_CONE_TARGET, &args![other]).u32();
        e.call(VECTOR_SUBTRACT, &args![other_point, difference, this_point]);
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![other, this, 0u32, 1u32],
            )
            .f32();
        let process = e.get(this, Actor::pCurrentProcess).addr();
        // Both branches of the code read the same setting.
        let _targeted = e.vcall(process, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
        let limit = setting_float(e, SETTING_11CD938);
        if (limit as f64) < distance as f64 {
            return 0;
        }
        let hundred: f64 = e.global(HUNDRED);
        if !((distance as f64) >= hundred || distance.is_nan())
            && other.addr() != player_addr
            && e.vcall(process, PROCESS_SLOT_68C, &args![]).u32() != other.addr()
        {
            return 0;
        }
        let angle = e.call(GET_Z_ANGLE_FROM_VECTOR, &args![difference]).f32();
        let rotation = e.call(ACTOR_ROTATION_POINTER, &args![this]).u32();
        let facing = e.mem.f32(rotation + 8);
        let mut turn = (angle as f64 - facing as f64) as f32;
        let pi: f64 = e.global(PI_DOUBLE);
        let two_pi: f64 = e.global(TWO_PI_DOUBLE);
        let minus_pi: f64 = e.global(MINUS_PI_DOUBLE);
        if turn as f64 > pi {
            turn = (turn as f64 - two_pi) as f32;
        }
        if (turn as f64) < minus_pi {
            turn = (turn as f64 + two_pi) as f32;
        }
        let targeted = e.vcall(process, ACQUIRE_OBJECT_SLOT_678, &args![]).u32();
        let turn_limit: f32 = if other.addr() == targeted {
            e.global(PI_FLOAT)
        } else {
            e.global(TURN_LIMIT_FLOAT)
        };
        let magnitude = e.call(ABSOLUTE_VALUE_ALT, &args![turn]).f64();
        if (turn_limit as f64) < magnitude {
            return 0;
        }
        if this.addr() != player_addr
            && other.addr() != player_addr
            && e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0
            && e.vcall(other.addr(), ACTOR_SLOT_214, &args![]).u32() != 0
        {
            return 0;
        }
        if check_detection != 0 {
            let level = e.with_stack(4, |e, flags| {
                // `+0` the "seen" byte (starts as 1), `+1` the second output
                // (starts 0), `+2` the player combat output.
                e.mem.set_u8(flags.addr(), 1);
                e.mem.set_u8(flags.addr() + 1, 0);
                e.mem.set_u8(flags.addr() + 2, 0);
                let in_combat = if other.addr() == player_addr {
                    e.call(PLAYER_IS_IN_COMBAT, &args![player_addr, flags.addr() + 2])
                        .u8()
                } else {
                    e.call(ACTOR_IN_COMBAT, &args![other]).u8()
                };
                let level = actor_get_detection_level_against_actor(
                    e,
                    this,
                    0,
                    other,
                    flags,
                    in_combat,
                    0,
                    in_combat as u32,
                    Ptr::new(flags.addr() + 1),
                );
                (level, e.mem.u8(flags.addr()), e.mem.u8(flags.addr() + 1))
            });
            let (level, seen, second) = level;
            if level <= 0 {
                return 0;
            }
            if other.addr() == player_addr
                && e.vcall(process, ACQUIRE_OBJECT_SLOT_678, &args![]).u32() == other.addr()
                && second == 0
            {
                return 0;
            }
            if e.call(ACTOR_FLAG_4938E0, &args![this]).bool() && seen == 0 {
                return 0;
            }
        }
        1
    })
}
/// `TESObjectREFR::GetDismembered` (Xbox PDB): thiscall on the reference,
/// takes a limb number.
const REFR_GET_DISMEMBERED: u32 = 0x0057_3090;
/// `String` constructor from a character pointer (`0040c0e0`): thiscall on a
/// 4-byte string object.
const STRING_CONSTRUCT: u32 = 0x0040_c0e0;
/// The string object's destructor (`004037d0`).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `009336c0`-like `cdecl` callback (`00936ac0`): a form id and a flag; the
/// variant of `MobileObject::SayToCallBack` used while in dialogue.
const SAY_TO_CALLBACK_IN_DIALOGUE: u32 = 0x0093_6ac0;
/// Lock entry function used with [`SAY_LOCK`] (`004538a0`): thiscall on the
/// lock, takes a flag.
const SAY_LOCK_ENTER: u32 = 0x0045_38a0;
/// Lock exit function used with [`SAY_LOCK`] (`004538c0`): thiscall on the
/// lock.
const SAY_LOCK_LEAVE: u32 = 0x0045_38c0;
/// The critical section `fn_008a20d0` holds while it works (`0x011df7e0`).
const SAY_LOCK: u32 = 0x011d_f7e0;
/// Setting object whose integer limits the distance at which the face is
/// animated (`0x011cd378`).
const SETTING_INT_11CD378: u32 = 0x011c_d378;
/// Acquire object slot `0x37c`: the actor's lip sync animation.
const ACQUIRE_OBJECT_SLOT_37C: u32 = 0x37c;
/// Acquire object slot `0x378` (a test).
const ACQUIRE_OBJECT_SLOT_378: u32 = 0x378;
/// Acquire object slot `0x370` (takes a flag).
const ACQUIRE_OBJECT_SLOT_370: u32 = 0x370;
/// Acquire object slot `0x154` (takes a flag).
const ACQUIRE_OBJECT_SLOT_154: u32 = 0x154;
/// Acquire object slot `0x374` (takes a flag).
const ACQUIRE_OBJECT_SLOT_374: u32 = 0x374;
/// Acquire object slot `0x2ec` (takes a word).
const ACQUIRE_OBJECT_SLOT_2EC: u32 = 0x2ec;
/// Acquire object slot `0x74c` (takes the duration in milliseconds).
const ACQUIRE_OBJECT_SLOT_74C: u32 = 0x74c;
/// Acquire object slot `0x744` (takes the lip sync animation).
const ACQUIRE_OBJECT_SLOT_744: u32 = 0x744;
/// Acquire object slot `0x748` (returns the lip sync animation).
const ACQUIRE_OBJECT_SLOT_748: u32 = 0x748;
/// Acquire object slot `0x380` (takes a flag).
const ACQUIRE_OBJECT_SLOT_380: u32 = 0x380;
/// Acquire object slot `0x44` (takes the actor, a word, a number, two
/// flags and a flag).
const ACQUIRE_OBJECT_SLOT_44: u32 = 0x44;
/// Process slot `0x1d4` (returns an object handed to `00600900`).
const PROCESS_SLOT_1D4: u32 = 0x1d4;
/// `TESIdleManager::SetUsedItem` (Xbox PDB), cdecl: the value.
const IDLE_MANAGER_SET_USED_ITEM_FN: u32 = 0x0060_0900;
/// Actor slot `0x2d4` (takes a word).
const ACTOR_SLOT_2D4: u32 = 0x2d4;
/// Actor slot `0x2dc` (takes a word).
const ACTOR_SLOT_2DC: u32 = 0x2dc;
/// `4d5ad0`: cdecl test on the line's string object.
const LINE_STRING_TEST_4D5AD0: u32 = 0x004d_5ad0;
/// `4d52d0`: cdecl, finds the lip sync animation for the line's string
/// object.
const LINE_STRING_FIND_4D52D0: u32 = 0x004d_52d0;
/// Setting object whose byte switches the early return of the line on
/// (`0x011e05ec`).
const SETTING_BOOL_11E05EC: u32 = 0x011e_05ec;
/// Manager singleton the early return reports to (`0x011f11e0`).
const SAY_REPORT_MANAGER: u32 = 0x011f_11e0;
/// `00905f30`: thiscall on the manager, takes the actor and the line text.
const SAY_REPORT: u32 = 0x0090_5f30;
/// `00935f00`: cdecl, a form id and a word: when the reference is an actor
/// (virtual `+0xfc`), `004bc4a0(3)`. It is also the voice's played callback
/// (`008a29ba` pushes it for `00adc050`, message 0x3b), which runs when the
/// sound starts (`00ae9af0` from `00aed080`).
const SAY_CANCEL_FN: u32 = 0x0093_5f00;
/// Global `float` the cancel of a missing lip sync stores
/// (`0x011d951c`).
const SAY_COUNTDOWN: u32 = 0x011d_951c;
/// Settings object whose `float` is stored there (`0x011d32c4`).
const SETTING_11D32C4: u32 = 0x011d_32c4;
/// Face animation data slot `0x124` (takes the text at `0x01084d70`).
const FACE_ANIMATION_SLOT_124: u32 = 0x124;
/// String passed to face animation slot `0x124` (`0x01084d70`).
const FACE_ANIMATION_STRING_1084D70: u32 = 0x0108_4d70;
/// `ActorMover::GetPreferredMoveMode` (map name; the body returns a global
/// pointer).
const PACKAGE_FN_5F36F0: u32 = 0x005f_36f0;
/// `4d5930`: thiscall on the lip sync animation, takes the face animation
/// data and a `float`.
const LIP_SYNC_START: u32 = 0x004d_5930;
/// `0.2` (`float`) passed to [`LIP_SYNC_START`].
const LIP_SYNC_START_FLOAT: u32 = 0x0101_df44;
/// `30.0` (`double`), the lip sync frame rate.
const LIP_SYNC_FRAME_RATE: u32 = 0x0101_db88;
/// `Actor` method `00880910` (a test).
const ACTOR_TEST_880910: u32 = 0x0088_0910;
/// `BSSoundHandle::SetPriority` (Xbox PDB).
const SOUND_HANDLE_SET_PRIORITY: u32 = 0x00ad_9030;
/// `005790d0`: returns a `float` (the sound's maximum distance).
const SOUND_MAX_DISTANCE: u32 = 0x0057_90d0;
/// `005790b0`: returns a `float` (the sound's minimum distance).
const SOUND_MIN_DISTANCE: u32 = 0x0057_90b0;
/// `BSSoundHandle::SetMinMax` (Xbox PDB): the two distances.
const SOUND_HANDLE_SET_MIN_MAX: u32 = 0x00ad_8be0;
/// `200.0` (`double`), added to the height of the sound position.
const SOUND_HEIGHT_OFFSET: u32 = 0x0101_79e0;
/// `BSAudioManager::SetCompletionCallback` variant (`00adc050`): thiscall on
/// the manager, takes the sound, the callback and a word.
const AUDIO_MANAGER_SET_CALLBACK_B: u32 = 0x00ad_c050;
/// `LipSynchAnim` method `004d5a90`: the animation's length (`float`).
const LIP_SYNC_LENGTH: u32 = 0x004d_5a90;
/// `BSSoundHandle::PlayAfter` (Xbox PDB): thiscall on the handle, takes a
/// delay in milliseconds and a flag.
const SOUND_HANDLE_PLAY_AFTER: u32 = 0x00ad_8870;
/// Settings object whose `float` is the seconds per unit of the line
/// (`0x011cd0bc`).
const SETTING_11CD0BC: u32 = 0x011c_d0bc;
/// `LipSynchAnim::~LipSynchAnim` (Xbox PDB): takes the delete flag.
const LIP_SYNC_DESTRUCT: u32 = 0x004d_5850;
/// Bytes of the actor: `+0x7c` and `+0x7d`.
const ACTOR_BYTE_7C: u32 = 0x7c;

// Translated from 008a20d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the actor speak the line with the file name `name`: stops the
/// current dialogue, finds its lip sync animation, starts the sound (a static
/// one or one placed at the actor and following its node), registers the
/// completion callback and tells the acquire object about the sound
/// (slot `0x490`). Returns the length of the line in seconds; `out_sound`
/// receives the sound handle. Returns 0 when the line cannot be spoken (no
/// name, a dismembered head or no 3D object). The code works inside a
/// critical section. The other arguments are not named by the exe: words
/// handed to the actor and the acquire object (`arg_10`, `arg_14`, `arg_1c`,
/// `arg_20`), `arg_18` a multiplier of a setting, `arg_24` another actor, and
/// bytes `flag_28`..`flag_38` that select what is done.
/// C++ exception unwinding (the string object's cleanup) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_008a20d0(
    e: &mut Engine,
    this: Ptr<Actor>,
    name: u32,
    out_sound: Ptr,
    arg_10: u32,
    arg_14: u32,
    arg_18: i32,
    arg_1c: u32,
    arg_20: u32,
    arg_24: Ptr<Actor>,
    flag_28: u8,
    flag_2c: u8,
    flag_30: u8,
    flag_34: u8,
    flag_38: u8,
) -> f32 {
    e.with_stack(8, |e, string_object| {
        e.call(STRING_CONSTRUCT, &args![string_object, name]);
        let result = e.with_stack(0x200, |e, buffer| {
            say_line(
                e,
                this,
                name,
                out_sound,
                [arg_10, arg_14, arg_1c, arg_20],
                arg_18,
                arg_24,
                [flag_28, flag_2c, flag_30, flag_34, flag_38],
                string_object.addr(),
                buffer.addr(),
            )
        });
        e.call(STRING_DESTRUCT, &args![string_object]);
        result
    })
}

/// The body of [`fn_008a20d0`]; `string_object` is the string object built
/// from the name and `buffer` the 512-byte copy of the name.
#[allow(clippy::too_many_arguments)]
fn say_line(
    e: &mut Engine,
    this: Ptr<Actor>,
    name: u32,
    out_sound: Ptr,
    words: [u32; 4],
    arg_18: i32,
    other_actor: Ptr<Actor>,
    flags: [u8; 5],
    string_object: u32,
    buffer: u32,
) -> f32 {
    let [arg_10, arg_14, arg_1c, arg_20] = words;
    let [flag_28, flag_2c, flag_30, flag_34, flag_38] = flags;
    let player_addr = e.global::<u32>(PLAYER_CHARACTER);
    let zero: f64 = e.global(ZERO_DOUBLE);
    e.vcall(this.addr(), ACTOR_SLOT_2D4, &args![arg_10]);
    e.vcall(this.addr(), ACTOR_SLOT_2DC, &args![arg_14]);
    if name == 0
        || e.call(REFR_GET_DISMEMBERED, &args![this, 1u32]).bool()
        || e.call(REFR_GET_DISMEMBERED, &args![this, 2u32]).bool()
    {
        let form_id = e.call(REFR_FORM_ID, &args![this]).u32();
        if e.call(MOBILE_OBJECT_IS_IN_DIALOGUE, &args![this]).bool() {
            e.call(SAY_TO_CALLBACK_IN_DIALOGUE, &args![form_id, 1u32]);
        } else {
            e.call(SAY_TO_CALLBACK, &args![form_id, 1u32]);
        }
        return 0.0;
    }
    e.call(COPY_STRING_LIMITED, &args![buffer, 0x200u32, name]);
    if this.is_null() || e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32() == 0 {
        return 0.0;
    }
    e.call(SAY_LOCK_ENTER, &args![SAY_LOCK, 0u32]);
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let mut lip_sync = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_37C, &args![]).u32();
    e.call(MOBILE_OBJECT_STOP_CURRENT_DIALOGUE, &args![this]);
    let is_flagged = e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).u8();
    let mut lip_length = 0.0f32;
    let static_sound: u8 = if is_flagged != 0 { 1 } else { flag_2c };
    let mut face = 0u32;
    let near_player = if e.call(IS_IN_DIALOGUE_WITH_PLAYER, &args![this]).bool() {
        true
    } else {
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![this, player_addr, 0u32, 0u32],
            )
            .f64();
        let limit = e
            .call(SETTING_GET_INT_POINTER, &args![SETTING_INT_11CD378])
            .u32();
        (e.mem.i32(limit) as f64) > distance
    };
    if near_player
        && (is_flagged == 0
            || e.call(PLAYER_FIRST_PERSON_CHECK, &args![player_addr])
                .bool())
    {
        face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![this]).u32();
    }
    if lip_sync == 0 && this.addr() != player_addr {
        let mut other_branch = flag_34 == 0;
        if !other_branch {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            other_branch = e.vcall(acquire, ACQUIRE_OBJECT_SLOT_378, &args![]).bool();
        }
        if !other_branch {
            if e.call(LINE_STRING_TEST_4D5AD0, &args![string_object])
                .bool()
            {
                let forced = flag_30 != 0 && {
                    let pointer = e
                        .call(SETTING_GET_BOOL_POINTER, &args![SETTING_BOOL_11E05EC])
                        .u32();
                    e.mem.u8(pointer) != 0
                };
                if forced {
                    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                    e.vcall(acquire, ACQUIRE_OBJECT_SLOT_370, &args![1u32]);
                    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                    e.vcall(acquire, ACQUIRE_OBJECT_SLOT_154, &args![1u32]);
                    let text = e.call(ENTRY_FIRST_EXTRA_LIST, &args![string_object]).u32();
                    e.call(SAY_REPORT, &args![SAY_REPORT_MANAGER, this, text]);
                    e.call(SAY_LOCK_LEAVE, &args![SAY_LOCK]);
                    return 0.0;
                }
                lip_sync = e.call(LINE_STRING_FIND_4D52D0, &args![string_object]).u32();
                if lip_sync == 0 {
                    let form_id = e.call(REFR_FORM_ID, &args![this]).u32();
                    e.call(SAY_CANCEL_FN, &args![form_id, 0u32]);
                    let countdown = setting_float(e, SETTING_11D32C4);
                    e.set_global(SAY_COUNTDOWN, countdown);
                }
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                e.vcall(acquire, ACQUIRE_OBJECT_SLOT_370, &args![0u32]);
            }
        } else {
            let own_face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![this]).u32();
            if own_face != 0 {
                e.vcall(
                    own_face,
                    FACE_ANIMATION_SLOT_124,
                    &args![FACE_ANIMATION_STRING_1084D70],
                );
            }
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            if e.vcall(acquire, ACQUIRE_OBJECT_SLOT_378, &args![]).bool() {
                let form_id = e.call(REFR_FORM_ID, &args![this]).u32();
                e.call(SAY_CANCEL_FN, &args![form_id, 0u32]);
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                e.vcall(acquire, ACQUIRE_OBJECT_SLOT_374, &args![0u32]);
            }
        }
    }
    let animation = e.vcall(this.addr(), ACTOR_SLOT_1E4, &args![]).u32();
    if animation != 0 {
        if arg_1c != 0 || flag_38 != 0 {
            let mut flag_b = 1u32;
            if e.call(ACTOR_IN_COMBAT, &args![this]).bool() {
                flag_b = 0;
            }
            let process = e.get(this, Actor::pCurrentProcess).addr();
            let held = e.vcall(process, PROCESS_SLOT_1D4, &args![]).u32();
            e.call(IDLE_MANAGER_SET_USED_ITEM_FN, &args![held]);
            e.vcall(
                process,
                ACQUIRE_OBJECT_SLOT_44,
                &args![this, arg_1c, (arg_1c != 0) as u32 + 2, 0u32, flag_b, 1u32],
            );
            e.call(IDLE_MANAGER_SET_USED_ITEM_FN, &args![0u32]);
        }
        if !other_actor.is_null() && other_actor.addr() != this.addr() {
            let other = other_actor.addr();
            if e.vcall(other, ACTOR_SLOT_FC, &args![]).bool()
                || e.vcall(other, ACTOR_SLOT_100, &args![]).bool()
            {
                let mut flag_a = 1u32;
                if e.vcall(other, ACTOR_SLOT_100, &args![]).bool()
                    && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![other]).u32() != 0
                {
                    let package = e
                        .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![other])
                        .u32();
                    if package != 0 && e.call(PACKAGE_FN_5F36F0, &args![package]).u32() != 0 {
                        flag_a = 0;
                    }
                    if arg_20 != 0 || flag_a != 0 {
                        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![other]).u32();
                        e.vcall(
                            acquire,
                            ACQUIRE_OBJECT_SLOT_44,
                            &args![other, arg_20, (arg_20 != 0) as u32 + 2, 0u32, 1u32, 1u32],
                        );
                    }
                }
            }
        }
    }
    if lip_sync != 0 {
        let first = e.call(ENTRY_FIRST_EXTRA_LIST, &args![lip_sync]).u32();
        lip_length = (first as f64 / e.global::<f64>(LIP_SYNC_FRAME_RATE)) as f32;
        let start: f32 = e.global(LIP_SYNC_START_FLOAT);
        e.call(LIP_SYNC_START, &args![lip_sync, face, start]);
    }
    if flag_28 != 0 && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, ACQUIRE_OBJECT_SLOT_2EC, &args![arg_10]);
    }
    let mut sound_flags = 0x104u32;
    if is_flagged == 0 && e.call(ACTOR_TEST_880910, &args![this]).bool() {
        sound_flags |= 0x8_0000;
    }
    if static_sound == 0 {
        sound_flags |= 2;
    } else {
        sound_flags |= 1;
    }
    e.with_stack(12, |e, found_handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
                &args![audio, found_handle, buffer, sound_flags, 0u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![out_sound, found]);
        e.call(SOUND_HANDLE_DESTROY, &args![found_handle]);
    });
    e.call(SOUND_HANDLE_SET_PRIORITY, &args![out_sound, 0x40u32]);
    if static_sound == 0 {
        let position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.f32(position + 8);
        let maximum = e.call(SOUND_MAX_DISTANCE, &args![]).f32();
        let minimum = e.call(SOUND_MIN_DISTANCE, &args![]).f32();
        e.call(
            SOUND_HANDLE_SET_MIN_MAX,
            &args![out_sound, minimum, maximum],
        );
        let raised = (z as f64 + e.global::<f64>(SOUND_HEIGHT_OFFSET)) as f32;
        e.call(
            SOUND_HANDLE_SET_POSITION_XYZ,
            &args![out_sound, x, y, raised],
        );
        let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
        e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![out_sound, node]);
    } else {
        let scale = setting_float(e, SETTING_STATIC_ATTENUATION) as f64;
        let value = scale * e.global::<f64>(HUNDRED);
        let attenuation = e.call(FTOL, &args![value]).u32();
        e.call(
            SOUND_HANDLE_SET_STATIC_ATTENUATION,
            &args![out_sound, attenuation],
        );
    }
    let manager = e.call(AUDIO_MANAGER_INSTANCE, &args![]).u32();
    let form_id = e.call(REFR_FORM_ID, &args![this]).u32();
    let first = e.call(ENTRY_FIRST_EXTRA_LIST, &args![out_sound]).u32();
    // 008a29ae..008a29cb: the form id, `PUSH 0x935f00`, the sound id. Not
    // `SayToCallBack` (`00936a20`): as the played callback it would end the
    // line the moment its voice starts.
    e.call(
        AUDIO_MANAGER_SET_CALLBACK_B,
        &args![manager, first, SAY_CANCEL_FN, form_id],
    );
    let result: f32;
    if lip_sync != 0 {
        let length = e.call(LIP_SYNC_LENGTH, &args![lip_sync]).f64();
        let base = fn_008a2d20(e) as f64;
        let milliseconds = ((base + length) * e.global::<f64>(THOUSAND_DOUBLE)) as f32;
        let delay = e.call(FTOL, &args![milliseconds as f64]).u32();
        e.call(SOUND_HANDLE_PLAY_AFTER, &args![out_sound, delay, 0u32]);
        let length = e.call(LIP_SYNC_LENGTH, &args![lip_sync]).f64();
        let mut total = (fn_008a2d20(e) as f64 + length) as f32;
        if lip_length as f64 > zero {
            total = (total as f64 + lip_length as f64) as f32;
        } else if e.call(SOUND_HANDLE_GET_DURATION, &args![out_sound]).u32() != 0 {
            let duration = e.call(SOUND_HANDLE_GET_DURATION, &args![out_sound]).u32() as i32;
            total = (total as f64 + duration as f64 / e.global::<f64>(THOUSAND_DOUBLE)) as f32;
        } else {
            let per_unit = setting_float(e, SETTING_11CD0BC) as f64;
            total = (per_unit * arg_18 as f64 + total as f64) as f32;
        }
        result = total;
        if e.vcall(this.addr(), ACTOR_SLOT_21C_TEST, &args![]).bool() {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let animation_time = e.call(ANIMATION_FLOAT_D0, &args![animation]).f64();
            let value = animation_time * e.global::<f64>(THOUSAND_DOUBLE) + milliseconds as f64;
            e.vcall(
                acquire,
                ACQUIRE_OBJECT_SLOT_74C,
                &args![value.trunc() as i64 as u32],
            );
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            e.vcall(acquire, ACQUIRE_OBJECT_SLOT_744, &args![lip_sync]);
        }
    } else {
        e.call(SOUND_HANDLE_PLAY_AFTER, &args![out_sound, 1u32, 0u32]);
        let per_unit = setting_float(e, SETTING_11CD0BC) as f64;
        result = (per_unit * arg_18 as f64) as f32;
    }
    if e.mem.u8(this.addr() + 0x7d) != 0 {
        e.mem.set_u8(this.addr() + ACTOR_BYTE_7C, 0);
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let handle_words = sound_handle_words(e, out_sound.addr());
    e.vcall(
        acquire,
        PROCESS_SLOT_490,
        &args![0u32, handle_words[0], handle_words[1], handle_words[2]],
    );
    if lip_sync != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if lip_sync != e.vcall(acquire, ACQUIRE_OBJECT_SLOT_748, &args![]).u32() {
            e.call(LIP_SYNC_DESTRUCT, &args![lip_sync, 1u32]);
        }
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, ACQUIRE_OBJECT_SLOT_380, &args![0u32]);
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        e.vcall(acquire, ACQUIRE_OBJECT_SLOT_370, &args![0u32]);
    }
    e.call(SAY_LOCK_LEAVE, &args![SAY_LOCK]);
    result
}
layout! {
    /// `HitData` (Xbox PDB), `0x64` bytes in the PDB (the PC allocation size has
    /// not been checked). Only the fields `Actor::HitMe` uses are listed; the
    /// offsets are the PDB's, which the PC code agrees with. `kHitLocation`
    /// (`+0x38`) and `kHitDirection` (`+0x44`) are vectors of three `float`s
    /// read by address.
    pub struct HitData: 0x64 {
        /// `pAggressor` (Xbox PDB): `Actor *`.
        0x00 pAggressor: Ptr,
        /// `pTarget` (Xbox PDB): `Actor *`.
        0x04 pTarget: Ptr,
        /// `pSourceRef` (Xbox PDB): `TESObjectREFR *`.
        0x08 pSourceRef: Ptr,
        /// `eAttackSkill` (Xbox PDB): `ActorValue::Index`.
        0x0C eAttackSkill: u32,
        /// `eDamageLimb` (Xbox PDB): `BGSBodyPart::LIMB_ENUM` (`-1` is none).
        0x10 eDamageLimb: i32,
        /// `fHealthDamage` (Xbox PDB).
        0x14 fHealthDamage: f32,
        /// `fTotalDamage` (Xbox PDB).
        0x18 fTotalDamage: f32,
        /// `fFatigueDamage` (Xbox PDB).
        0x1C fFatigueDamage: f32,
        /// `fTargetedLimbDamage` (Xbox PDB).
        0x20 fTargetedLimbDamage: f32,
        /// `fPercentBlocked` (Xbox PDB).
        0x24 fPercentBlocked: f32,
        /// `fArmorDamage` (Xbox PDB).
        0x28 fArmorDamage: f32,
        /// `pWeapon` (Xbox PDB): `TESObjectWEAP *`.
        0x30 pWeapon: Ptr,
        /// `pCriticalEffect` (Xbox PDB): `SpellItem *`.
        0x50 pCriticalEffect: Ptr,
        /// `pVATSCommand` (Xbox PDB): `VATS_COMMAND *`.
        0x54 pVATSCommand: Ptr,
        /// `uiFlags` (Xbox PDB).
        0x58 uiFlags: u32,
    }
}

/// Size of the stack frame `Actor::HitMe` keeps its structure locals in; the
/// translation lays them out at the offsets the code uses (`HIT_ME_FRAME_EBP`
/// stands for `EBP`).
const HIT_ME_FRAME_SIZE: u32 = 0x700;
/// Offset of `EBP` in the emulated frame of `Actor::HitMe`.
const HIT_ME_FRAME_EBP: u32 = 0x640;

/// The locals of `Actor::HitMe` that live across its sections. The names
/// say what the code does with them; the comments give the offsets below
/// `EBP` the code keeps them at.
struct HitMe {
    this: Ptr<Actor>,
    hit: Ptr<HitData>,
    /// The second argument (a byte; `1` and `2` mean attack kinds).
    kind: i32,
    player: u32,
    /// The emulated `EBP`.
    ebp: u32,
    /// `-0x19`: the actor was moved out of the way (arrows or a knockback).
    moved: bool,
    /// `-0x1a`.
    flag_1a: bool,
    /// `-0x30`: the result of `Actor` slot `0x214` (attached arrows).
    arrows: u32,
    /// `-0x50`: weapon type number plus one.
    weapon_type: u16,
    /// `-0x61`: a limb became crippled by this hit.
    crippled: bool,
    /// `-0x62`: the kind of the hit is 1.
    kind_one: bool,
    /// `-0x63`.
    flag_63: bool,
    /// `-0x64`.
    flag_64: bool,
    /// `-0x68`, `-0x6c`, `-0x70`, `-0x74`: copies of the hit's health, fatigue,
    /// total damage and percent blocked.
    health: f32,
    fatigue: f32,
    total: f32,
    blocked: f32,
    /// `-0x279` of the frame: whether the actor is not fleeing.
    not_fleeing: bool,
    /// `-0x2ad` of the frame: whether `Actor` slot `0x338` reported the
    /// damage as lethal.
    dead: bool,
    /// The limb index the hit damaged (`-1` for none).
    limb: i32,
    /// Whether the limb damage may cripple the limb (`-0x338` of the frame).
    cripple_roll: bool,
    /// Whether the hit targeted the torso (limb 0; `-0x33e` of the frame).
    torso_hit: bool,
}

impl HitMe {
    fn aggressor(&self, e: &Engine) -> u32 {
        e.get(self.hit, HitData::pAggressor).addr()
    }
    fn weapon(&self, e: &Engine) -> u32 {
        e.get(self.hit, HitData::pWeapon).addr()
    }
    fn source(&self, e: &Engine) -> u32 {
        e.get(self.hit, HitData::pSourceRef).addr()
    }
    fn flags(&self, e: &Engine) -> u32 {
        e.get(self.hit, HitData::uiFlags)
    }
    /// The address of the local at `EBP + offset`.
    fn local(&self, offset: i32) -> u32 {
        self.ebp.wrapping_add(offset as u32)
    }
}

/// Process slot `0x4c4` (`MiddleHighProcess::GetFurnitureMarkerID`, Xbox PDB).
const PROCESS_SLOT_4C4: u32 = 0x4c4;
/// Process slot `0x20c`: what the actor is sitting on (a package).
const PROCESS_SLOT_20C: u32 = 0x20c;
/// `MiddleHighProcess::KnockExplosion` (Xbox PDB), process slot `0x418`:
/// the actor, a position (three words) and a speed.
const PROCESS_SLOT_KNOCK_EXPLOSION: u32 = 0x418;
/// `HighProcess::GetWeaponLastPos` (Xbox PDB), process slot `0x460`.
const PROCESS_SLOT_WEAPON_LAST_POS: u32 = 0x460;
/// `MiddleHighProcess::GetSitSleepState` (Xbox PDB), process slot `0x4bc`.
const PROCESS_SLOT_4BC: u32 = 0x4bc;
/// `MiddleHighProcess::ShouldCheckFlare` (Xbox PDB), process slot `0x5c8`:
/// two words.
const PROCESS_SLOT_5C8: u32 = 0x5c8;
/// `HighProcess::SetCurrentProcessIdle` (Xbox PDB), process slot `0x71c`.
const PROCESS_SLOT_SET_IDLE: u32 = 0x71c;
/// `HighProcess::GetAnimActionAnimSeq` (Xbox PDB), process slot `0x3e8`.
const PROCESS_SLOT_3E8: u32 = 0x3e8;
/// `MiddleHighProcess::GetCharController` (Xbox PDB), process slot `0x28c`.
const PROCESS_SLOT_CHAR_CONTROLLER: u32 = 0x28c;
/// `0087ae10`: thiscall on the task queue interface, queues a hit (the actor
/// and the hit data).
const QUEUE_ACTOR_HIT: u32 = 0x0087_ae10;
/// `0087ada0`: thiscall on the task queue interface, queues the hit
/// feedback (the actor and the hit data).
const QUEUE_ACTOR_HIT_FEEDBACK: u32 = 0x0087_ada0;
/// `008a5300`: thiscall on the actor, run at the start and the end of the hit.
const ACTOR_HIT_BRACKET_8A5300: u32 = 0x008a_5300;
/// `Actor::GetOutofFurnitureQuick` (Xbox PDB).
const ACTOR_GET_OUT_OF_FURNITURE_QUICK: u32 = 0x0088_d640;
/// `008a5230`: thiscall on the aggressor.
const AGGRESSOR_FN_8A5230: u32 = 0x008a_5230;
/// `008a51f0`: thiscall on the aggressor, takes the hit data.
const AGGRESSOR_FN_8A51F0: u32 = 0x008a_51f0;
/// `008a52c0`: thiscall on the target, takes the hit data.
const TARGET_FN_8A52C0: u32 = 0x008a_52c0;
/// Table of `u16` per limb number (`0x01196dac`, stride 4).
const LIMB_WORD_TABLE: u32 = 0x0119_6dac;
/// Global pointer to the default weapon form (`0x011ca278`).
const DEFAULT_WEAPON_FORM: u32 = 0x011c_a278;
/// `Script::RunScriptEffectStart` (Xbox PDB): thiscall, takes the target and
/// a flag.
const SCRIPT_RUN_EFFECT_START: u32 = 0x005a_c340;
/// `"%s %s"` (`0x01012058`).
const HIT_MESSAGE_FORMAT: u32 = 0x0101_2058;
/// The string the hit message uses as its icon text (`0x01011584`).
const HIT_MESSAGE_TEXT: u32 = 0x0101_1584;
/// The sound path of the hit message (`0x0103a830`).
const HIT_MESSAGE_SOUND: u32 = 0x0103_a830;
/// Setting holding the first hit message word (`0x011d3240`).
const SETTING_HIT_MESSAGE_A: u32 = 0x011d_3240;
/// Setting holding the second hit message word (`0x011d502c`).
const SETTING_HIT_MESSAGE_B: u32 = 0x011d_502c;
/// `00406d00`: cdecl `snprintf`-style formatter: the buffer, its size, the
/// format and the arguments.
const FORMAT_STRING_406D00: u32 = 0x0040_6d00;
/// `ActorValue::GetActorValueName` (Xbox PDB), cdecl: the actor value number.
const ACTOR_VALUE_GET_NAME: u32 = 0x0066_e950;
/// The debug line format for the health and fatigue damage (`0x01084c68`).
const HIT_DEBUG_FORMAT: u32 = 0x0108_4c68;
/// `Interface::SetEnemyActor` (Xbox PDB), cdecl: the actor.
const INTERFACE_SET_ENEMY_ACTOR: u32 = 0x0070_38b0;
/// `00647b70`: cdecl, a damage integer and a flag; returns the statistics
/// value.
const DAMAGE_STATISTIC_VALUE: u32 = 0x0064_7b70;
/// The error line printed when the player has no animation (`0x01084bb8`).
const NO_ANIMATION_FORMAT: u32 = 0x0108_4bb8;
/// Offset of the magic caster sub-object in an actor.
const MAGIC_CASTER_OFFSET: u32 = 0x88;
/// Magic caster slot `0x40`: sets the spell's magic item.
const MAGIC_CASTER_SLOT_SET_SPELL: u32 = 0x40;
/// `MagicCaster::FindTargets` (Xbox PDB): thiscall with five words.
const MAGIC_CASTER_FIND_TARGETS: u32 = 0x0081_5d00;
/// `00824110`: thiscall on `Actor + 0x94`, takes the spell's magic item.
const MAGIC_TARGET_FN_824110: u32 = 0x0082_4110;
/// Actor slot `0x338`: applies the health and fatigue damage and returns
/// whether it kills (`fn_0089d6f0`).
const ACTOR_SLOT_APPLY_DAMAGE: u32 = 0x338;
/// Actor slot `0x3ac`: damages a limb.
const ACTOR_SLOT_DAMAGE_LIMB: u32 = 0x3ac;
/// `Actor::DamageEquipment` (Xbox PDB), actor slot `0x3c8`.
const ACTOR_SLOT_DAMAGE_EQUIPMENT: u32 = 0x3c8;
/// `Actor::HitMe_ov2` (Xbox PDB), actor slot `0x478`.
const ACTOR_SLOT_HIT_ME_OV2: u32 = 0x478;
/// Actor slot `0x4bc` (`fn_008987f0`): the actor and a number.
const ACTOR_SLOT_HIT_NOTICE: u32 = 0x4bc;
/// Reference slot `0x220`.
const REFERENCE_SLOT_220: u32 = 0x220;
/// Reference slot `0x224`.
const REFERENCE_SLOT_224: u32 = 0x224;
/// Reference slot `0x168`.
const REFERENCE_SLOT_168: u32 = 0x168;
/// `004839c0`: cdecl, finds a form by its id.
const FIND_FORM_BY_ID: u32 = 0x0048_39c0;
/// The id of the form `Actor::HitMe` looks up (`0x1768d7`).
const FORM_ID_1768D7: u32 = 0x0017_68d7;
/// `00500940`: thiscall on a form, takes a pointer to a form pointer; returns
/// a list.
const FORM_LIST_FIRST_500940: u32 = 0x0050_0940;
/// `005f65d0`: thiscall on that list, returns a byte.
const LIST_TEST_5F65D0: u32 = 0x005f_65d0;
/// Setting byte that suppresses the critical effect cast (`0x011df7f8`).
const SETTING_SKIP_CRITICAL_EFFECT: u32 = 0x011d_f7f8;
/// `CombatFormulas::GetKnockbackSpeed` (Xbox PDB), cdecl: the value owner and
/// a number; the speed is in ST0.
const GET_KNOCKBACK_SPEED: u32 = 0x0064_6580;
/// `HighProcess::GetDialogTarget` (Xbox PDB), thiscall (called on a weapon).
const HIGH_PROCESS_GET_DIALOG_TARGET: u32 = 0x0051_f510;
/// `009ac9c0`: cdecl, creates a reference (sixteen words).
const CREATE_REFERENCE_AT: u32 = 0x009a_c9c0;
/// `004a3c90`: thiscall on a structure, takes a pointer.
const OBJECT_FN_4A3C90: u32 = 0x004a_3c90;
/// `00476c70`: thiscall on a reference, takes a form.
const REFERENCE_FN_476C70: u32 = 0x0047_6c70;
/// Setting (`0x011d0334`) multiplying the targeted limb damage when the kind
/// is 2.
const SETTING_KIND_TWO_SCALE: u32 = 0x011d_0334;
/// Setting integer (`0x011cf308`): percent chance of a torso hit crippling.
const SETTING_LIMB_CHANCE_A: u32 = 0x011c_f308;
/// Setting integer (`0x011ce0fc`): percent chance of dropping the weapon.
const SETTING_TORSO_CHANCE: u32 = 0x011c_e0fc;
/// `BGSBodyPartData::GetBodyPart` (Xbox PDB): thiscall on the data, takes a
/// number.
const BODY_PART_DATA_GET_BODY_PART: u32 = 0x005e_50f0;
/// `005e5190`: thiscall on a body part, returns its limb number (a byte).
const BODY_PART_LIMB_NUMBER: u32 = 0x005e_5190;
/// `00647ac0`: thiscall on a body part, returns a percentage (a byte).
const BODY_PART_PERCENT: u32 = 0x0064_7ac0;
/// `0.01` (`double`).
const ONE_HUNDREDTH: u32 = 0x0101_6408;
/// `009b1720`: thiscall on the source reference, takes the actor; returns the
/// first node of its body part list.
const SOURCE_BODY_PART_LIST: u32 = 0x009b_1720;
/// Actor base form slot `0x180`: the body part data.
const BASE_FORM_SLOT_BODY_PART_DATA: u32 = 0x180;
/// `00646cb0`: thiscall test on a form.
const EQUIPMENT_TEST_646CB0: u32 = 0x0064_6cb0;
/// `008b43a0`: thiscall on the actor, takes the hit data, the attacker's base
/// form and a category; the death handler.
const ACTOR_DEATH_HANDLER: u32 = 0x008b_43a0;
/// `008ae000`: thiscall on the aggressor, takes the hit data.
const AGGRESSOR_KILL_NOTICE: u32 = 0x008a_e000;
/// Setting byte (`0x011df760`) that enables the crippled-limb idle.
const SETTING_CRIPPLE_IDLE: u32 = 0x011d_f760;
/// The idle manager pointer (`0x011cb6a0`).
const IDLE_MANAGER: u32 = 0x011c_b6a0;
/// `TESIdleManager::GetIdleToPlay` (Xbox PDB): thiscall on the manager, takes
/// the actor and a reference.
const IDLE_MANAGER_GET_IDLE_TO_PLAY: u32 = 0x0060_0950;
/// `Animation::SpecialIdleWorking` (Xbox PDB): takes the idle.
const ANIMATION_SPECIAL_IDLE_WORKING: u32 = 0x0049_8d30;
/// `TESIdleForm::GetIdleAnimGroupSection` (Xbox PDB).
const IDLE_FORM_GET_ANIM_GROUP_SECTION: u32 = 0x005f_f160;
/// `00497f20`: thiscall on the animation, starts an idle (the idle, the
/// actor, the section and a number).
const ANIMATION_START_IDLE: u32 = 0x0049_7f20;
/// The VATS singleton (`0x011f2250`).
const VATS_SINGLETON: u32 = 0x011f_2250;
/// `008d0300`: thiscall on the aggressor, takes the target and the skill.
const ACTOR_FN_8D0300: u32 = 0x008d_0300;
/// Setting (`0x011cf5a0`): the health ratio above which a hit staggers.
const SETTING_STAGGER_RATIO: u32 = 0x011c_f5a0;
/// Setting (`0x011cee30`): the chance of a stagger.
const SETTING_STAGGER_CHANCE: u32 = 0x011c_ee30;
/// `009a6ae0`: cdecl, takes the actor, the aggressor and a flag.
const ACTOR_STAGGER_TEST_9A6AE0: u32 = 0x009a_6ae0;
/// The debug line for a stunning hit (`0x01084b7c`).
const STUN_DEBUG_FORMAT: u32 = 0x0108_4b7c;
/// The global the rumble object is read from (`0x011dea0c`).
const CONTROLS_HOLDER: u32 = 0x011d_ea0c;
/// `Controls::Rumble` (Xbox PDB): thiscall with seven words.
const CONTROLS_RUMBLE: u32 = 0x00a2_55b0;
/// `00974e90`: thiscall on the process lists, takes the actor and a number.
const PROCESS_LISTS_NOTIFY: u32 = 0x0097_4e90;
/// `0047c850` as a predicate of the actor.
const ACTOR_PREDICATE_47C850: u32 = 0x0047_c850;
/// `008d0370`: thiscall on the actor, takes the aggressor.
const ACTOR_FN_8D0370: u32 = 0x008d_0370;
/// `TESImageSpaceModifier::GetGetHit` (Xbox PDB), no arguments.
const IMAGE_SPACE_GET_HIT: u32 = 0x005d_2860;
/// `ImageSpaceModifierInstanceForm::Trigger` (Xbox PDB), cdecl: the modifier,
/// the strength and a node.
const IMAGE_SPACE_TRIGGER: u32 = 0x0052_99a0;
/// Setting (`0x011cf834`): the strength of the hit screen effect.
const SETTING_BLOCK_SCREEN_EFFECT: u32 = 0x011c_f834;
/// Rumble time setting for a blocked hit on the player (`0x011cfeac`).
const SETTING_BLOCKED_RUMBLE_TIME: u32 = 0x011c_feac;
/// Rumble pulse setting for a blocked hit on the player (`0x011d0078`).
const SETTING_BLOCKED_RUMBLE_PULSE: u32 = 0x011d_0078;
/// Rumble time setting for a blocked hit by the player (`0x011d0988`).
const SETTING_BLOCK_RUMBLE_TIME_OUT: u32 = 0x011d_0988;
/// Rumble pulse setting for a blocked hit by the player (`0x011d1008`).
const SETTING_BLOCK_RUMBLE_PULSE_OUT: u32 = 0x011d_1008;
/// Rumble time setting for a hit on the player (`0x011d0acc`).
const SETTING_HIT_RUMBLE_TIME: u32 = 0x011d_0acc;
/// Rumble pulse setting for a hit on the player (`0x011d1374`).
const SETTING_HIT_RUMBLE_PULSE: u32 = 0x011d_1374;
/// Rumble time setting for a hit by the player (`0x011d067c`).
const SETTING_HIT_OUT_RUMBLE_TIME: u32 = 0x011d_067c;
/// Rumble pulse setting for a hit by the player (`0x011d0b30`).
const SETTING_HIT_OUT_RUMBLE_PULSE: u32 = 0x011d_0b30;
/// `0088e1e0`: thiscall on the hit actor, takes the hit data.
const ACTOR_FN_88E1E0: u32 = 0x0088_e1e0;
/// `0088e8d0`: thiscall on the hit actor, takes the damage and the hit data.
const ACTOR_FN_88E8D0: u32 = 0x0088_e8d0;
/// `0088fb00`: thiscall on the aggressor, takes the damage, the hit data and
/// the hit actor.
const ACTOR_FN_88FB00: u32 = 0x0088_fb00;
/// Setting (`0x011cf5ac`): the largest knockback speed.
const SETTING_KNOCKBACK_CAP: u32 = 0x011c_f5ac;
/// Setting (`0x011cfbd8`): the velocity modifier of the knockback.
const SETTING_VELOCITY_MODIFIER: u32 = 0x011c_fbd8;
/// `bhkCharacterController::SetVelocityModifier` (Xbox PDB): a vector and a
/// `float`.
const CHARACTER_SET_VELOCITY_MODIFIER: u32 = 0x00c6_d4d0;
/// `00552490`: thiscall test on the ragdoll controller.
const RAGDOLL_TEST_552490: u32 = 0x0055_2490;
/// `bhkRagdollController::SetRagdollFeedbackActive` (Xbox PDB).
const RAGDOLL_SET_FEEDBACK_ACTIVE: u32 = 0x00c7_b6a0;
/// `004a3c20`: thiscall, constructs a structure.
const OBJECT_CONSTRUCT_4A3C20: u32 = 0x004a_3c20;
/// `00553fc0`: cdecl, copies a vector (destination, source).
const VECTOR_COPY_553FC0: u32 = 0x0055_3fc0;
/// `00458880`: thiscall on a vector, takes two vectors.
const VECTOR_CROSS_458880: u32 = 0x0045_8880;
/// `00c76320`: thiscall on the ragdoll controller.
const RAGDOLL_PICK_C76320: u32 = 0x00c7_6320;
/// `00c762f0`: thiscall on the ragdoll controller, returns a rigid body.
const RAGDOLL_BODY_C762F0: u32 = 0x00c7_62f0;
/// `008c71b0`: thiscall on a structure, takes a flag.
const OBJECT_FN_8C71B0: u32 = 0x008c_71b0;
/// `0059ce80`: thiscall on a structure, takes a flag.
const OBJECT_FN_59CE80: u32 = 0x0059_ce80;
/// `004a39f0`: thiscall on a structure, takes a number.
const OBJECT_FN_4A39F0: u32 = 0x004a_39f0;
/// `004a3f70`: thiscall on a structure, takes a word.
const OBJECT_FN_4A3F70: u32 = 0x004a_3f70;
/// `00458a10`: thiscall on a vector, takes two vectors.
const VECTOR_FN_458A10: u32 = 0x0045_8a10;
/// `0056d340`: thiscall on a structure, takes a vector.
const OBJECT_FN_56D340: u32 = 0x0056_d340;
/// `004a3f40`: thiscall on a structure, takes a vector.
const OBJECT_FN_4A3F40: u32 = 0x004a_3f40;
/// Slot `0xc8` of the ragdoll controller's pick object.
const PICK_SLOT_C8: u32 = 0xc8;
/// `bhkPickData::GethkRigidBody` (Xbox PDB).
const PICK_DATA_GET_RIGID_BODY: u32 = 0x00c6_6fd0;
/// `004b5a20`: cdecl, the reference of a rigid body.
const RIGID_BODY_REFERENCE_4B5A20: u32 = 0x004b_5a20;
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), cdecl.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `0062aea0`: thiscall, constructs a structure.
const OBJECT_CONSTRUCT_62AEA0: u32 = 0x0062_aea0;
/// `005dbf20`: thiscall on a structure, takes a `float`.
const OBJECT_FN_5DBF20: u32 = 0x005d_bf20;
/// `00627920`: thiscall on a structure, takes a structure.
const OBJECT_FN_627920: u32 = 0x0062_7920;
/// `00541c80`: thiscall, returns a sound handle-like object.
const PICK_HANDLE_541C80: u32 = 0x0054_1c80;
/// `hkpRigidBody::applyLinearImpulse` (Xbox PDB).
const RIGID_BODY_APPLY_LINEAR_IMPULSE: u32 = 0x0081_82d0;
/// `00525430`: thiscall test on the VATS singleton.
const VATS_TEST_525430: u32 = 0x0052_5430;
/// `007a9280`: thiscall test on the VATS singleton.
const VATS_TEST_7A9280: u32 = 0x007a_9280;
/// `00716440`: returns a `float` (no arguments).
const VATS_SCALE_716440: u32 = 0x0071_6440;
/// `004410d0`: thiscall on the VATS singleton, takes a flag.
const VATS_FN_4410D0: u32 = 0x0044_10d0;
/// `005ca4f0`: thiscall test.
const REFERENCE_TEST_5CA4F0: u32 = 0x005c_a4f0;
/// `GetTickCount` (the import slot).
const IMPORT_GET_TICK_COUNT: u32 = 0x00fd_f060;
/// Deadline (a tick count) before which the combat dialogue is not started
/// again (`0x011df684`).
const HIT_BARK_DEADLINE: u32 = 0x011d_f684;
/// Setting (`0x011df70c`): the upper delay in seconds.
const SETTING_BARK_DELAY_UPPER: u32 = 0x011d_f70c;
/// Setting (`0x011df73c`): the lower delay in seconds.
const SETTING_BARK_DELAY_LOWER: u32 = 0x011d_f73c;
/// `00944460`: cdecl, a random number between two bounds.
const RANDOM_RANGE_944460: u32 = 0x0094_4460;

// Translated from 0089a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::HitMe` (Xbox PDB): applies the hit `hit` (a `HitData`) to the actor.
/// `kind` is a byte: `1` and `2` mark special attack kinds (a limb hit that
/// always cripples; a limb damage scaled by a setting).
///
/// Does nothing for an actor without a process or for a missing hit; when the
/// task queue is in use (`008c7aa0`) only queues the hit (`0087ae10`).
/// Otherwise: gets the actor off furniture it sits on when it carries arrows,
/// notifies the aggressor, records the weapon type, reports the attack to the
/// statistics and the script action flags, applies the health and fatigue
/// damage (`Actor` slot `0x338`), the critical effect and the VATS
/// effects, knocks the actor back, damages the limb (`Actor` slot `0x3ac`) and
/// hands a lethal hit on to the death handler (`008b43a0`); then plays the
/// hit reactions (the blocked reaction, the ragdoll impulse, the animation
/// and the controller rumble when the player is involved), damages armor
/// and tells the combat controller and the combat dialogue manager.
/// C++ exception unwinding (the string and structure locals' cleanup) is not
/// translated.
pub fn actor_hit_me(e: &mut Engine, this: Ptr<Actor>, hit: Ptr<HitData>, kind: u8) {
    e.with_stack(HIT_ME_FRAME_SIZE, |e, frame| {
        let mut s = HitMe {
            this,
            hit,
            kind: kind as i8 as i32,
            player: e.global(PLAYER_CHARACTER),
            ebp: frame.addr() + HIT_ME_FRAME_EBP,
            moved: false,
            flag_1a: false,
            arrows: 0,
            weapon_type: 1,
            crippled: false,
            kind_one: false,
            flag_63: false,
            flag_64: false,
            health: 0.0,
            fatigue: 0.0,
            total: 0.0,
            blocked: 0.0,
            not_fleeing: false,
            dead: false,
            limb: -1,
            cripple_roll: false,
            torso_hit: false,
        };
        hit_me_body(e, &mut s);
    });
}

/// The sections of `Actor::HitMe` in order; every `return` of a section that
/// returns `false` is a jump to the end of the function (`0089d4fa`).
fn hit_me_body(e: &mut Engine, s: &mut HitMe) {
    if !hit_me_start(e, s) {
        return;
    }
    hit_me_report_attack(e, s);
    hit_me_damage_values(e, s);
    hit_me_damage(e, s);
    hit_me_reactions(e, s);
    hit_me_finish(e, s);
}

/// `0089a7a1`..`0089aafb`: the guards, the arrows, the weapon type and the
/// first statistics call. Returns `false` when the function ends here.
fn hit_me_start(e: &mut Engine, s: &mut HitMe) -> bool {
    let this = s.this;
    let hit = s.hit;
    if e.get(this, Actor::pCurrentProcess).addr() == 0 {
        return false;
    }
    e.call(ACTOR_HIT_BRACKET_8A5300, &args![this]);
    if hit.is_null() {
        return false;
    }
    if e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
        let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
        e.call(QUEUE_ACTOR_HIT, &args![queue, this, hit]);
        return false;
    }
    if e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        && e.get(hit, HitData::eDamageLimb) == 0xe
    {
        return false;
    }
    if e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool() {
        // The names of the aggressor and the weapon are looked up, but the
        // code never uses them.
        let aggressor = s.aggressor(e);
        if aggressor != 0 {
            e.call(REFR_GET_NAME, &args![aggressor]);
        }
        let weapon = s.weapon(e);
        if weapon != 0 {
            e.call(MAP_MARKER_DATA_GET_LOCATION_NAME, &args![weapon + 0x30]);
        }
    }
    s.arrows = e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32();
    if e.vcall(this.addr(), ACTOR_SLOT_214, &args![]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        if e.vcall(acquire, PROCESS_SLOT_4C4, &args![]).u32() > 0x14 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
            let sitting_on = e.vcall(acquire, PROCESS_SLOT_20C, &args![]).u32();
            if sitting_on == 0 || e.call(PACKAGE_TYPE, &args![sitting_on]).u32() != 0x1a {
                if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
                    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                    if e.vcall(acquire, PROCESS_SLOT_4C8, &args![]).u32() != 0 {
                        let first = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                        let second = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                        let index = e.vcall(first, PROCESS_SLOT_4D0, &args![]).u32();
                        let furniture = e.vcall(second, PROCESS_SLOT_4C8, &args![]).u32();
                        e.call(REFR_SET_MARKER_USED, &args![furniture, index, 0u32]);
                    }
                }
                e.call(MOBILE_OBJECT_SET_CHASE_BIP, &args![this, 0u32]);
                e.call(ACTOR_GET_OUT_OF_FURNITURE_QUICK, &args![this]);
                let mut who = this.addr();
                if s.aggressor(e) != 0 {
                    who = s.aggressor(e);
                }
                let position = e.vcall(who, ACTOR_SLOT_POSITION, &args![]).u32();
                let [x, y, z] = read_words3(e, position);
                let process = e.get(this, Actor::pCurrentProcess).addr();
                e.vcall(
                    process,
                    PROCESS_SLOT_KNOCK_EXPLOSION,
                    &args![this, x, y, z, 1.0f32],
                );
                s.moved = true;
            }
        }
    }
    let aggressor = s.aggressor(e);
    if aggressor != 0 {
        e.call(AGGRESSOR_FN_8A5230, &args![aggressor]);
        e.call(AGGRESSOR_FN_8A51F0, &args![aggressor, hit]);
    }
    e.call(TARGET_FN_8A52C0, &args![this, hit]);
    s.weapon_type = 1;
    let weapon = s.weapon(e);
    if weapon != 0 {
        s.weapon_type = (e.call(WEAPON_TYPE, &args![weapon]).u16()).wrapping_add(1);
        if e.call(WEAPON_GETTER_8D85E0, &args![weapon]).u32() == 0x2d {
            s.flag_1a = true;
        }
    } else if s.flags(e) & 0x2000 == 0 {
        s.flag_1a = true;
    }
    let flagged = s.flags(e) & 4 != 0;
    if s.aggressor(e) == s.player
        && !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
        && flagged
        && !e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool()
    {
        let mut base = e.call(REFR_BASE_FORM, &args![this]).u32();
        if e.call(ACTOR_TEST_56AF40, &args![this]).bool() {
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
            base = e.call(EXTRA_GET_BASE_FORM_421720, &args![extra]).u32();
        }
        let limb_word = limb_word(e, hit);
        e.call(
            STATISTICS_WORKER_5F5950,
            &args![
                3u32,
                1u32,
                base,
                weapon,
                limb_word,
                1u32,
                u32::from(s.weapon_type)
            ],
        );
    }
    true
}

/// Reads three words at `address`.
fn read_words3(e: &Engine, address: u32) -> [u32; 3] {
    [
        e.mem.u32(address),
        e.mem.u32(address + 4),
        e.mem.u32(address + 8),
    ]
}

/// The `u16` of the limb table for the hit's damage limb.
fn limb_word(e: &Engine, hit: Ptr<HitData>) -> u32 {
    let limb = e.get(hit, HitData::eDamageLimb);
    u32::from(
        e.mem
            .u16(LIMB_WORD_TABLE.wrapping_add((limb as u32).wrapping_mul(4))),
    )
}

/// `0089aafe`..`0089ac5b`: the script action flags of the aggressor and the
/// weapon, and the script effect of the ammo.
fn hit_me_report_attack(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    let aggressor = s.aggressor(e);
    let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
    e.call(SCRIPT_SET_ACTION_FLAG, &args![aggressor, extra, 0x80u32]);
    let weapon = s.weapon(e);
    if weapon != 0 {
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
        e.call(SCRIPT_SET_ACTION_FLAG, &args![weapon, extra, 0x100u32]);
        if aggressor != 0
            && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![aggressor]).u32() != 0
            && e.call(LIST_NEXT, &args![weapon + 0x68]).u32() != 0
        {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![aggressor]).u32();
            let entry = e
                .vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                .u32();
            if entry != 0 && e.call(ENTRY_FIRST_EXTRA_LIST, &args![entry]).u32() != 0 {
                let first = e.call(ENTRY_FIRST_EXTRA_LIST, &args![entry]).u32();
                let slot = e.call(LIST_ITEM_SLOT, &args![first]).u32();
                if e.mem.u32(slot) != 0 {
                    let first = e.call(ENTRY_FIRST_EXTRA_LIST, &args![entry]).u32();
                    let slot = e.call(LIST_ITEM_SLOT, &args![first]).u32();
                    let value = e.mem.u32(slot);
                    let target = e.get(hit, HitData::pTarget).addr();
                    e.call(SCRIPT_SET_ACTION_FLAG, &args![target, value, 0x80u32]);
                }
            }
        }
        let ammo = e
            .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, aggressor])
            .u32();
        if ammo != 0 {
            let ammo = e
                .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, aggressor])
                .u32();
            let effect = e.call(LIST_NEXT, &args![ammo + 0x9c]).u32();
            if effect != 0 {
                let ammo = e
                    .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, aggressor])
                    .u32();
                let effect = e.call(LIST_NEXT, &args![ammo + 0x9c]).u32();
                e.call(SCRIPT_RUN_EFFECT_START, &args![effect, this, 0u32]);
            }
        }
    } else {
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
        let default_weapon = e.global::<u32>(DEFAULT_WEAPON_FORM);
        e.call(
            SCRIPT_SET_ACTION_FLAG,
            &args![default_weapon, extra, 0x100u32],
        );
    }
}

/// `0089ac5b`..`0089ae19`: reads the damage values from the hit and prints the
/// player's hit message.
fn hit_me_damage_values(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    s.crippled = false;
    e.mem.set_u8(this.addr() + 0x1b1, 0);
    s.kind_one = false;
    s.flag_63 = false;
    s.flag_64 = false;
    s.health = e.get(hit, HitData::fHealthDamage);
    s.fatigue = e.get(hit, HitData::fFatigueDamage);
    s.total = e.get(hit, HitData::fTotalDamage);
    s.blocked = e.get(hit, HitData::fPercentBlocked);
    let aggressor = s.aggressor(e);
    if aggressor != 0
        && e.vcall(aggressor, ACTOR_SLOT_360, &args![]).bool()
        && e.call(HIT_DATA_FLAG_TEST, &args![hit, 4u32]).bool()
        && !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
        && !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
    {
        let word = if e.call(HIT_DATA_FLAG_TEST, &args![hit, 0x400u32]).bool() {
            e.call(SETTING_GET_STRING, &args![SETTING_HIT_MESSAGE_A])
                .u32()
        } else {
            e.call(SETTING_GET_STRING, &args![SETTING_HIT_MESSAGE_B])
                .u32()
        };
        let name = e.call(REFR_GET_NAME, &args![this]).u32();
        e.with_stack(0x1f4, |e, text| {
            e.call(
                FORMAT_STRING_406D00,
                &args![text, 0x1f4u32, HIT_MESSAGE_FORMAT, word, name],
            );
            // A `BSString` local (8 bytes: the text, its length and capacity, `0040c0e0`).
            e.with_stack(8, |e, message| {
                e.call(STRING_CONSTRUCT, &args![message, HIT_MESSAGE_TEXT]);
                let characters = e.call(ENTRY_FIRST_EXTRA_LIST, &args![message]).u32();
                let two: f32 = e.global(DETECTION_FLOAT_TWO);
                e.call(
                    SHOW_MESSAGE,
                    &args![text, 1u32, HIT_MESSAGE_SOUND, characters, two, 0u32],
                );
                e.call(STRING_DESTRUCT, &args![message]);
            });
        });
    }
    s.not_fleeing = !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool();
}

/// The effect the hit casts on the actor: interrupts the actor's magic
/// caster (`+0x88`), sets the spell (`spell` is read after the interrupt and
/// again after the targets were found, as the code does), finds the targets
/// and tells `Actor + 0x94`.
fn hit_me_cast_effect(e: &mut Engine, this: Ptr<Actor>, mut spell: impl FnMut(&mut Engine) -> u32) {
    let caster = this.addr() + MAGIC_CASTER_OFFSET;
    e.call(MAGIC_CASTER_INTERRUPT_CAST, &args![caster]);
    let first = spell(e);
    let item = if first != 0 { first + 0x18 } else { 0 };
    e.vcall(caster, MAGIC_CASTER_SLOT_SET_SPELL, &args![item]);
    e.call(
        MAGIC_CASTER_FIND_TARGETS,
        &args![caster, 1.0f32, 0u32, 0u32, 1.0f32, 0u32],
    );
    let second = spell(e);
    let item = if second != 0 { second + 0x18 } else { 0 };
    e.call(MAGIC_TARGET_FN_824110, &args![this.addr() + 0x94, item]);
}

/// `0089ae19`..`0089c5be`: the damage section, run when the hit does health,
/// fatigue or limb damage.
fn hit_me_damage(e: &mut Engine, s: &mut HitMe) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let hit = s.hit;
    let limb_damage = e.get(hit, HitData::fTargetedLimbDamage);
    if !((s.health as f64) > zero || (s.fatigue as f64) > zero || limb_damage as f64 != zero) {
        return;
    }
    hit_me_debug_lines(e, s);
    let this = s.this;
    let aggressor = s.aggressor(e);
    if aggressor == s.player {
        e.call(INTERFACE_SET_ENEMY_ACTOR, &args![this]);
    }
    if aggressor == s.player {
        hit_me_player_statistics(e, s);
        if s.flag_1a {
            hit_me_player_spell(e, s);
        }
    }
    s.dead = e
        .vcall(
            this.addr(),
            ACTOR_SLOT_APPLY_DAMAGE,
            &args![s.health, s.fatigue, aggressor],
        )
        .bool();
    if s.not_fleeing {
        hit_me_critical_effect(e, s);
        hit_me_kill_effect(e, s);
    }
    hit_me_vats_effect(e, s);
    hit_me_knockback(e, s);
    hit_me_limb_damage(e, s);
    hit_me_death(e, s);
}

/// `0089ae19`..`0089aefd`: the debug lines about the health and fatigue
/// damage.
fn hit_me_debug_lines(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let aggressor = s.aggressor(e);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !e.call(DEBUG_MESSAGES_ENABLED, &args![]).bool() || aggressor == 0 {
        return;
    }
    for (damage, actor_value) in [(s.health, 0x10u32), (s.fatigue, 0x16)] {
        if (damage as f64) > zero {
            let value_name = e.call(ACTOR_VALUE_GET_NAME, &args![actor_value]).u32();
            let current = e.vcall(this.addr() + 0xa4, 0x0c, &args![actor_value]).f64();
            let this_name = e.call(REFR_GET_NAME, &args![this]).u32();
            let aggressor_name = e.call(REFR_GET_NAME, &args![aggressor]).u32();
            e.call(
                PRINT_DEBUG_LINE,
                &args![
                    HIT_DEBUG_FORMAT,
                    aggressor_name,
                    this_name,
                    damage as f64,
                    current,
                    value_name
                ],
            );
        }
    }
}

/// `0089af16`..`0089b051`: when the player hits a living actor, reports the
/// hit to the statistics.
fn hit_me_player_statistics(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    if !s.not_fleeing || e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
        return;
    }
    let mut default_kind = 0x2du32;
    if s.weapon(e) == 0 && s.flags(e) & 0x2000 != 0 {
        default_kind = 0x23;
    }
    let mut base = e.call(REFR_BASE_FORM, &args![this]).u32();
    if e.call(ACTOR_TEST_56AF40, &args![this]).bool() {
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
        base = e.call(EXTRA_GET_BASE_FORM_421720, &args![extra]).u32();
    }
    if this.addr() != s.player {
        let weapon = s.weapon(e);
        let selected = if weapon != 0 {
            e.call(WEAPON_GETTER_8D85E0, &args![weapon]).u32()
        } else {
            default_kind
        };
        let damage = e.call(FTOL, &args![s.health as f64]).u32();
        let value = e.call(DAMAGE_STATISTIC_VALUE, &args![damage, 1u32]).u32();
        let limb = limb_word(e, hit);
        e.call(
            STATISTICS_WORKER_5F5950,
            &args![
                8u32,
                value,
                base,
                s.weapon(e),
                limb,
                selected & 0xffff,
                u32::from(s.weapon_type)
            ],
        );
    }
    let limb = limb_word(e, hit);
    e.call(
        STATISTICS_WORKER_5F5950,
        &args![
            3u32,
            1u32,
            base,
            s.weapon(e),
            limb,
            0u32,
            u32::from(s.weapon_type)
        ],
    );
}

/// `0089b051`..`0089b26b`: the player's current spell effect is released with
/// the default object chosen by the animation's group number.
fn hit_me_player_spell(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![s.player]).u32();
    let sequence = e.vcall(acquire, PROCESS_SLOT_3E8, &args![]).u32();
    if sequence == 0 {
        e.call(PRINT_ERROR_LINE, &args![NO_ANIMATION_FORMAT]);
        return;
    }
    let transform = e
        .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
        .u32();
    let group = e.call(ANIM_GROUP_GET_NUMBER, &args![transform]).i32();
    if !((0x61..=0x65).contains(&group) || group == 0xa8) {
        return;
    }
    let object_number = match group {
        0x61 => Some(0x19u32),
        0x62 => Some(0x1a),
        0x63 => Some(0x1d),
        0x64 => Some(0x1b),
        0x65 => Some(0x1c),
        0xa8 => Some(0x1e),
        _ => None,
    };
    let mut chosen = 0u32;
    if let Some(number) = object_number {
        chosen = e.call(GET_DEFAULT_OBJECT, &args![number]).u32();
    }
    let spell = chosen;
    if spell == 0 {
        return;
    }
    e.call(MAGIC_CASTER_INTERRUPT_CAST, &args![this.addr() + 0x88]);
    let item = spell + 0x18;
    e.vcall(
        this.addr() + 0x88,
        MAGIC_CASTER_SLOT_SET_SPELL,
        &args![item],
    );
    e.call(
        MAGIC_CASTER_FIND_TARGETS,
        &args![this.addr() + 0x88, 1.0f32, 0u32, 0u32, 1.0f32, 0u32],
    );
    e.call(MAGIC_TARGET_FN_824110, &args![this.addr() + 0x94, item]);
}

/// `0089b295`..`0089b43f`: the critical effect of the hit.
fn hit_me_critical_effect(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    if !e.call(HIT_DATA_FLAG_TEST, &args![hit, 4u32]).bool()
        || e.get(hit, HitData::pCriticalEffect).addr() == 0
    {
        return;
    }
    let essential = e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool();
    if (essential || !s.dead) && e.call(HIT_DATA_FLAG_TEST, &args![hit, 8u32]).bool() {
        return;
    }
    let mut effect_found = false;
    if s.arrows != 0 {
        let form = e.call(FIND_FORM_BY_ID, &args![FORM_ID_1768D7]).u32();
        let critical = e.get(hit, HitData::pCriticalEffect).addr();
        if critical != 0 && form != 0 {
            e.with_stack(4, |e, holder| {
                e.mem.set_u32(holder.addr(), critical);
                // 0089b333: the pushed holder is the word of `005f65d0` (RET 4).
                let list = e.call(FORM_LIST_FIRST_500940, &args![form]).u32();
                if e.call(LIST_TEST_5F65D0, &args![list, holder]).bool() {
                    effect_found = true;
                }
            });
        }
    }
    let skip = e
        .call(
            SETTING_GET_BOOL_POINTER,
            &args![SETTING_SKIP_CRITICAL_EFFECT],
        )
        .u32();
    if e.mem.u8(skip) == 0 && !effect_found {
        hit_me_cast_effect(e, this, |e| e.get(hit, HitData::pCriticalEffect).addr());
    }
}

/// `0089b43f`..`0089b6d0`: a kill of the player's bound weapon 0x22 can
/// fire a projectile-like reference back at the target.
fn hit_me_kill_effect(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let aggressor = s.aggressor(e);
    if aggressor == 0 {
        return;
    }
    let mut entry_form = 0u32;
    if s.player != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![s.player]).u32();
        if acquire != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![s.player]).u32();
            if e.vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                .u32()
                != 0
            {
                let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![s.player]).u32();
                let entry = e
                    .vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
                    .u32();
                entry_form = e.call(ENTRY_FORM, &args![entry]).u32();
            }
        }
    }
    if entry_form == 0 || e.call(WEAPON_GETTER_8D85E0, &args![entry_form]).u32() != 0x22 || !s.dead
    {
        return;
    }
    let chance = e.with_stack(4, |e, out| {
        e.mem.set_f32(out.addr(), 0.0);
        e.call(HANDLE_ENTRY_POINT, &args![0x2eu32, aggressor, out]);
        e.mem.f32(out.addr())
    });
    if (chance as f64) <= e.global::<f64>(ZERO_DOUBLE) {
        return;
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() == 0 {
        return;
    }
    let object = e.call(GET_DEFAULT_OBJECT, &args![0x18u32]).u32();
    if object == 0 {
        return;
    }
    let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
    let rotation = e.call(NODE_ROTATION, &args![node]).u32();
    let rotation_words: Vec<u32> = (0..9).map(|i| e.mem.u32(rotation + 4 * i)).collect();
    let node = e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32();
    let translation = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
    let [tx, ty, tz] = read_words3(e, translation);
    let cell = e.call(REFR_PARENT_CELL, &args![this]).u32();
    let mut words = vec![object, aggressor, 0u32, cell, tx, ty, tz];
    words.extend(rotation_words);
    let reference = e.call(CREATE_REFERENCE_AT, &words).u32();
    if reference == 0 {
        return;
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    let controller = e
        .vcall(acquire, PROCESS_SLOT_CHAR_CONTROLLER, &args![])
        .u32();
    let offset = fn_0089d600(e, Ptr::new(controller)).addr();
    let copy = s.local(-0x300);
    e.call(OBJECT_FN_4A3C90, &args![copy, offset]);
    let vector = s.local(-0x30c);
    let built = e
        .call(
            VECTOR3_CONSTRUCT_416870,
            &args![vector, 0.0f32, 0.0f32, 1.0f32],
        )
        .u32();
    let [x, y, z] = read_words3(e, built);
    fn_0089d640(e, Ptr::new(reference), x, y, z);
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![aggressor]).u32();
    if e.vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32()
        != 0
    {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![aggressor]).u32();
        let entry = e
            .vcall(acquire, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32();
        let weapon = e.call(ENTRY_FORM, &args![entry]).u32();
        e.call(REFERENCE_FN_476C70, &args![reference, weapon]);
    }
}

/// `0089b6d0`..`0089b7e2`: a VATS command of kind `0x10` or `0x11` casts the
/// effect of the weapon's dialog target.
fn hit_me_vats_effect(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    let command = e.get(hit, HitData::pVATSCommand).addr();
    if command == 0 {
        return;
    }
    let kind = e.mem.u32(command);
    let weapon = s.weapon(e);
    if (kind != 0x10 && kind != 0x11)
        || weapon == 0
        || e.call(HIGH_PROCESS_GET_DIALOG_TARGET, &args![weapon]).u32() == 0
    {
        return;
    }
    hit_me_cast_effect(e, this, |e| {
        e.call(HIGH_PROCESS_GET_DIALOG_TARGET, &args![weapon]).u32()
    });
}

/// `0089b7e2`..`0089b970`: a chance, from the aggressor's entry points, of
/// knocking the actor back.
fn hit_me_knockback(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let aggressor = s.aggressor(e);
    if aggressor == 0 {
        return;
    }
    let chance = e.with_stack(4, |e, out| {
        e.mem.set_f32(out.addr(), 0.0);
        let weapon = s.weapon(e);
        e.call(HANDLE_ENTRY_POINT, &args![0x34u32, aggressor, weapon, out]);
        e.mem.f32(out.addr())
    });
    if (chance as f64) <= e.global::<f64>(ZERO_DOUBLE) {
        return;
    }
    let roll = e.call(RANDOM_FLOAT, &args![0.0f32, 1.0f32]).f64();
    if roll > chance as f64 {
        return;
    }
    let owner = this.addr() + 0xa4;
    let speed = e.call(GET_KNOCKBACK_SPEED, &args![owner, 5u32]).f32();
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if acquire == 0 || e.call(ACQUIRE_OBJECT_FIELD_28, &args![acquire]).u32() != 0 {
        return;
    }
    s.moved = true;
    let source = s.source(e);
    let from = if source != 0 { source } else { aggressor };
    let position = e.vcall(from, ACTOR_SLOT_POSITION, &args![]).u32();
    let [x, y, z] = read_words3(e, position);
    e.vcall(
        acquire,
        PROCESS_SLOT_KNOCK_EXPLOSION,
        &args![this, x, y, z, speed],
    );
}

/// `0089b970`..`0089bfaf`: the damage to a limb. With a source reference that
/// passes `0x220`, each body part named in the list of the source damages its
/// limb in proportion to the health damage; otherwise the limb the hit
/// names is damaged by the targeted limb damage.
fn hit_me_limb_damage(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    s.limb = -1;
    s.cripple_roll = false;
    s.torso_hit = false;
    let source = s.source(e);
    if source != 0 && e.vcall(source, REFERENCE_SLOT_220, &args![]).bool() {
        let base = e.call(ACTOR_BASE_FORM_FN, &args![this]).u32();
        let body_parts = e.vcall(base, BASE_FORM_SLOT_BODY_PART_DATA, &args![]).u32();
        if body_parts == 0 {
            return;
        }
        let mut node = e.call(SOURCE_BODY_PART_LIST, &args![source, this]).u32();
        while node != 0 && !e.call(LIST_NODE_STOP_TEST, &args![node]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(slot);
            node = e.call(LIST_NEXT, &args![node]).u32();
            let number = e.mem.u32(item);
            let part = e
                .call(BODY_PART_DATA_GET_BODY_PART, &args![body_parts, number])
                .u32();
            if part != 0 {
                s.limb = e.call(BODY_PART_LIMB_NUMBER, &args![part]).u8() as i8 as i32;
                if s.limb >= 0 && s.limb < 0x4d {
                    let percent = e.call(BODY_PART_PERCENT, &args![part]).u8();
                    let share = f64::from(percent) * e.global::<f64>(ONE_HUNDREDTH);
                    let maximum = e.vcall(this.addr() + 0xa4, 0x00, &args![0x10u32]).i32();
                    let damage = (s.health as f64 / (maximum as f64 * share)
                        * e.global::<f64>(HUNDRED)) as f32;
                    let weapon = s.weapon(e);
                    let selected = if weapon != 0 {
                        weapon
                    } else {
                        e.global::<u32>(DEFAULT_WEAPON_FORM)
                    };
                    let aggressor = s.aggressor(e);
                    e.with_stack(4, |e, out| {
                        e.mem.set_f32(out.addr(), damage);
                        e.call(
                            HANDLE_ENTRY_POINT,
                            &args![6u32, this, aggressor, selected, out],
                        );
                        let scaled = e.mem.f32(out.addr());
                        e.vcall(
                            this.addr(),
                            ACTOR_SLOT_DAMAGE_LIMB,
                            &args![s.limb as u32, -scaled, aggressor],
                        );
                    });
                }
                s.limb = -1;
            }
        }
        return;
    }
    // The limb the hit names.
    e.vcall(this.addr(), ACTOR_SLOT_360, &args![]);
    let named = e.get(hit, HitData::eDamageLimb);
    if named == -1 {
        return;
    }
    if named == 0 {
        s.torso_hit = true;
    } else if named != 0xe {
        // The general limb path below.
    } else {
        // The head-mounted equipment of limb 0xe takes the damage.
        let limb_damage = e.get(hit, HitData::fTargetedLimbDamage);
        let process = e.get(this, Actor::pCurrentProcess).addr();
        let entry = e
            .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32();
        e.vcall(
            this.addr(),
            ACTOR_SLOT_DAMAGE_EQUIPMENT,
            &args![entry, limb_damage, 0u32],
        );
        if !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
            && !e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
            && e.call(HIT_DATA_FLAG_TEST, &args![hit, 4u32]).bool()
        {
            let roll = e.call(RANDOM_NUMBER, &args![]).u32() % 100;
            let chance = e
                .call(SETTING_GET_INT_POINTER, &args![SETTING_TORSO_CHANCE])
                .u32();
            if roll < e.mem.u32(chance) {
                fn_0089f580(e, this);
            }
        }
        return;
    }
    let base = e.call(ACTOR_BASE_FORM_FN, &args![this]).u32();
    let body_parts = e.vcall(base, BASE_FORM_SLOT_BODY_PART_DATA, &args![]).u32();
    if body_parts == 0 {
        return;
    }
    let part = e
        .call(
            BODY_PART_DATA_GET_BODY_PART,
            &args![body_parts, named as u32],
        )
        .u32();
    if part == 0 {
        return;
    }
    s.limb = e.call(BODY_PART_LIMB_NUMBER, &args![part]).u8() as i8 as i32;
    if !(s.limb >= 0 && s.limb < 0x4d) {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let limb_health = e
        .vcall(this.addr() + 0xa4, 0x0c, &args![s.limb as u32])
        .f32();
    let mut damage = e.get(hit, HitData::fTargetedLimbDamage);
    if s.kind == 2 {
        let scale = e
            .call(SETTING_GET_VALUE_POINTER, &args![SETTING_KIND_TWO_SCALE])
            .u32();
        damage = (damage as f64 * e.mem.f32(scale) as f64) as f32;
    }
    let weapon = s.weapon(e);
    let selected = if weapon != 0 {
        weapon
    } else {
        e.global::<u32>(DEFAULT_WEAPON_FORM)
    };
    let aggressor = s.aggressor(e);
    e.with_stack(4, |e, out| {
        e.mem.set_f32(out.addr(), damage);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![6u32, this, aggressor, selected, out],
        );
        damage = e.mem.f32(out.addr());
    });
    e.vcall(
        this.addr(),
        ACTOR_SLOT_DAMAGE_LIMB,
        &args![s.limb as u32, -damage, aggressor],
    );
    s.crippled = limb_health as f64 > zero
        && e.vcall(this.addr() + 0xa4, 0x0c, &args![s.limb as u32])
            .f64()
            <= zero;
    if s.torso_hit && !s.crippled {
        let current = e
            .vcall(this.addr() + 0xa4, 0x0c, &args![s.limb as u32])
            .f64();
        if current <= zero {
            let roll = e.call(RANDOM_NUMBER, &args![]).u32() % 100;
            let chance = e
                .call(SETTING_GET_INT_POINTER, &args![SETTING_LIMB_CHANCE_A])
                .u32();
            if roll <= e.mem.u32(chance) {
                s.cripple_roll = true;
            }
        }
    } else if s.crippled {
        s.cripple_roll = true;
    }
    if e.vcall(this.addr() + 0xa4, 0x08, &args![0x48u32]).u32() != 0 {
        s.cripple_roll = false;
    }
    if s.kind == 1 {
        s.cripple_roll = true;
        e.mem.set_u8(this.addr() + 0x1b1, 1);
    }
    let process = e.get(this, Actor::pCurrentProcess).addr();
    let entry = e
        .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32();
    let worn = if entry != 0 {
        e.call(ENTRY_FORM, &args![entry]).u32()
    } else {
        0
    };
    if s.cripple_roll
        && worn != 0
        && !e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
        && !e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
    {
        let named = e.get(hit, HitData::eDamageLimb);
        let drop_weapon = if named == 5 || named == 6 {
            true
        } else if !e.call(EQUIPMENT_TEST_646CB0, &args![worn]).bool() {
            false
        } else {
            named == 3 || named == 4
        };
        if drop_weapon {
            fn_0089f580(e, this);
        }
    }
}

/// The death-category number for the form type `form_type` of the source
/// reference (`0089c0ba`..`0089c13b`; the table is the one at `0x0089d598`).
fn hit_me_death_category(form_type: u32) -> u32 {
    match form_type {
        0x28 => 2,
        0x2f => 5,
        0x33 | 0x3d | 0x3f | 0x40 | 0x69 => 1,
        0x3e | 0x51 => 0,
        0x52 => 4,
        _ => 3,
    }
}

/// `0089bfaf`..`0089c5be`: for an actor with a 3D object in a loaded cell,
/// the death handling (`008b43a0`), the aggressor's kill notice, the idle that
/// reacts to a crippled limb and the stun.
fn hit_me_death(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    if e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32() == 0
        || e.call(REFR_PARENT_CELL, &args![this]).u32() == 0
    {
        return;
    }
    let cell = e.call(REFR_PARENT_CELL, &args![this]).u32();
    if !e.call(CELL_TEST_450FF0, &args![cell]).bool() {
        return;
    }
    let source = s.source(e);
    let mut attacker_base = 0u32;
    if source != 0 {
        if e.vcall(source, ACTOR_SLOT_100, &args![]).bool() {
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![source]).u32();
            attacker_base = e
                .call(EXTRA_GET_LEV_CREA_ORIGINAL_BASE, &args![extra])
                .u32();
        }
        if attacker_base == 0 {
            attacker_base = e.call(REFR_BASE_FORM, &args![source]).u32();
        }
    }
    let source_flag = source != 0 && e.vcall(source, REFERENCE_SLOT_220, &args![]).bool();
    let aggressor = s.aggressor(e);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if s.dead && e.call(PROCESS_LEVEL_NUMBER, &args![this]).u32() != 6 {
        let mut category = 3u32;
        if source != 0 {
            let form_type = e.call(FORM_TYPE, &args![source]).u32();
            category = hit_me_death_category(form_type);
        }
        if source_flag {
            category = 0;
        }
        let announce = if s.limb == -1 {
            true
        } else {
            e.vcall(this.addr() + 0xa4, 0x0c, &args![s.limb as u32])
                .f64()
                <= zero
        };
        if announce {
            e.call(
                ACTOR_DEATH_HANDLER,
                &args![this, hit, attacker_base, category],
            );
            if aggressor == s.player
                && s.not_fleeing
                && !e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool()
            {
                e.call(MISC_STAT_INCREMENT, &args![0x24u32]);
            }
        }
    }
    if s.not_fleeing && s.dead {
        if e.call(ACTOR_GETTER_87F9C0, &args![this]).f64() > zero && aggressor != 0 {
            e.call(AGGRESSOR_KILL_NOTICE, &args![aggressor, hit]);
        }
    } else if !e.vcall(this.addr(), ACTOR_SLOT_230, &args![]).bool() {
        let enabled = e
            .call(SETTING_GET_BOOL_POINTER, &args![SETTING_CRIPPLE_IDLE])
            .u32();
        if e.mem.u8(enabled) != 0
            && e.get(hit, HitData::eDamageLimb) != -1
            && s.cripple_roll
            && e.vcall(this.addr() + 0xa4, 0x08, &args![0x48u32]).i32() <= 0
        {
            let manager = e.global::<u32>(IDLE_MANAGER);
            let idle = e
                .call(IDLE_MANAGER_GET_IDLE_TO_PLAY, &args![manager, this, source])
                .u32();
            let animation = e.vcall(this.addr(), ACTOR_SLOT_1E4, &args![]).u32();
            if idle != 0
                && animation != 0
                && !e
                    .call(ANIMATION_SPECIAL_IDLE_WORKING, &args![animation, idle])
                    .bool()
            {
                if this.addr() == s.player {
                    let section = e.call(IDLE_FORM_GET_ANIM_GROUP_SECTION, &args![idle]).u32();
                    e.call(
                        ANIMATION_START_IDLE,
                        &args![animation, idle, this, section, 2u32],
                    );
                    e.call(VATS_QUIT_PLAYBACK, &args![VATS_SINGLETON, 0u32, 0u32]);
                } else {
                    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
                    e.vcall(acquire, PROCESS_SLOT_SET_IDLE, &args![idle]);
                    e.vcall(acquire, PROCESS_SLOT_614, &args![0x10u32]);
                }
            }
        }
    }
    e.mem.set_u8(this.addr() + 0x1b1, 0);
    if s.dead {
        return;
    }
    let mut stunned = false;
    if (s.fatigue as f64) > zero {
        let value = e.vcall(this.addr() + 0xa4, 0x08, &args![0x16u32]).i32();
        if (value as f64) <= zero {
            stunned = true;
        }
    }
    if s.kind == 1 {
        s.kind_one = true;
    }
    let maximum = e.vcall(this.addr() + 0xa4, 0x08, &args![0x10u32]).i32();
    let ratio = (s.health as f64 / maximum as f64) as f32;
    if aggressor != 0 {
        let skill = e.get(hit, HitData::eAttackSkill);
        e.call(ACTOR_FN_8D0300, &args![aggressor, this, skill]);
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if !e.call(ACTOR_IS_TALKING, &args![this]).bool()
        && !e.call(MOBILE_OBJECT_IS_IN_DIALOGUE, &args![this]).bool()
        && !e.vcall(acquire, PROCESS_SLOT_30C, &args![]).bool()
    {
        if stunned {
            s.flag_64 = true;
        } else {
            let limit = e
                .call(SETTING_GET_VALUE_POINTER, &args![SETTING_STAGGER_RATIO])
                .u32();
            let limit = e.mem.f32(limit);
            if (limit as f64) < ratio as f64 {
                s.flag_64 = true;
            } else {
                let chance = e
                    .call(SETTING_GET_VALUE_POINTER, &args![SETTING_STAGGER_CHANCE])
                    .u32();
                let chance = e.mem.f32(chance);
                if e.call(CHANCE_ROLL, &args![chance]).bool() {
                    s.flag_64 = true;
                }
            }
        }
    }
    if stunned && !e.vcall(this.addr(), ACTOR_SLOT_230, &args![]).bool() && aggressor != 0 {
        if e.call(ACTOR_STAGGER_TEST_9A6AE0, &args![this, aggressor, 0u32])
            .bool()
        {
            s.flag_63 = true;
        }
        if e.call(DEBUG_MESSAGES_ENABLED, &args![]).bool() {
            let aggressor_name = e.call(REFR_GET_NAME, &args![aggressor]).u32();
            let this_name = e.call(REFR_GET_NAME, &args![this]).u32();
            e.call(
                PRINT_DEBUG_LINE,
                &args![STUN_DEBUG_FORMAT, this_name, aggressor_name, s.total as f64],
            );
        }
    }
}

/// The controller rumble of the hit (`Controls::Rumble`): the time is the
/// setting `time_setting` (times `scale`, or times 1000.0 without one) and
/// both pulse strengths are the setting `pulse_setting`.
fn hit_me_rumble(e: &mut Engine, time_setting: u32, pulse_setting: u32, scale: Option<f64>) {
    let holder = e.global::<u32>(CONTROLS_HOLDER);
    let controls = e.call(OBJECT_011DEA0C_GET, &args![holder]).u32();
    let time = setting_float(e, time_setting) as f64;
    let milliseconds = match scale {
        Some(multiplier) => time * multiplier,
        None => time * e.global::<f64>(THOUSAND_DOUBLE),
    };
    let milliseconds = e.call(FTOL, &args![milliseconds]).u32();
    let first = setting_float(e, pulse_setting);
    let second = setting_float(e, pulse_setting);
    e.call(
        CONTROLS_RUMBLE,
        &args![
            controls,
            second,
            first,
            milliseconds,
            0u32,
            0u32,
            0u32,
            0u32
        ],
    );
}

/// `0089c5be`..`0089d0af`: the reactions of the hit actor that need a 3D
/// object in a loaded cell: the blocked hit, the knockback and ragdoll
/// impulse, the damage hooks and the player's screen and controller feedback.
fn hit_me_reactions(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    if e.vcall(this.addr(), ACTOR_SLOT_1D0, &args![]).u32() == 0
        || e.call(REFR_PARENT_CELL, &args![this]).u32() == 0
    {
        return;
    }
    let cell = e.call(REFR_PARENT_CELL, &args![this]).u32();
    if !e.call(CELL_TEST_450FF0, &args![cell]).bool() {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if (s.blocked as f64) > zero {
        hit_me_blocked_reaction(e, s);
    } else {
        hit_me_unblocked_reaction(e, s);
    }
    if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32() != 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
        let value = e.vcall(this.addr() + 0x94, 0x08, &args![]).u32();
        if e.vcall(acquire, PROCESS_SLOT_5C8, &args![value, 8u32])
            .bool()
        {
            e.call(PROCESS_LISTS_NOTIFY, &args![PROCESS_LISTS, this, 8u32]);
        }
    }
}

/// `0089c614`..`0089c7b7`: the hit was (partly) blocked: the block idle for
/// the actor and the player's feedback.
fn hit_me_blocked_reaction(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let source = s.source(e);
    let aggressor = s.aggressor(e);
    let flag = if source != 0 {
        false
    } else {
        e.call(ACTOR_PREDICATE_47C850, &args![this]).bool()
    };
    fn_00894d90(e, this, u32::from(flag));
    if !flag {
        let result = e.call(ACTOR_FN_8D0370, &args![this, aggressor]).bool();
        if source == 0 || result {
            fn_00894e90(e, Ptr::new(aggressor));
        }
    }
    if this.addr() == s.player {
        if e.call(ENTRY_FORM, &args![VATS_SINGLETON]).u32() == 0 {
            let strength = setting_float(e, SETTING_BLOCK_SCREEN_EFFECT);
            let modifier = e.call(IMAGE_SPACE_GET_HIT, &args![]).u32();
            e.call(IMAGE_SPACE_TRIGGER, &args![modifier, strength, 0u32]);
        }
        if !e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).bool() {
            hit_me_rumble(
                e,
                SETTING_BLOCKED_RUMBLE_TIME,
                SETTING_BLOCKED_RUMBLE_PULSE,
                None,
            );
        }
    } else if aggressor == s.player
        && source == 0
        && !e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).bool()
    {
        hit_me_rumble(
            e,
            SETTING_BLOCK_RUMBLE_TIME_OUT,
            SETTING_BLOCK_RUMBLE_PULSE_OUT,
            None,
        );
    }
}

/// `0089c7b7`..`0089d0af`: the hit was not blocked.
fn hit_me_unblocked_reaction(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    let aggressor = s.aggressor(e);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !s.flag_63 && aggressor != 0 {
        if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![aggressor]).u32() != 0 {
            let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![aggressor]).u32();
            let weapon_buffer = s.local(-0x3fc);
            let position = e
                .call(
                    ACTOR_GET_WEAPON_POSITION,
                    &args![aggressor, weapon_buffer, 0u32],
                )
                .u32();
            let last = e
                .vcall(acquire, PROCESS_SLOT_WEAPON_LAST_POS, &args![])
                .u32();
            e.vcall(
                this.addr(),
                ACTOR_SLOT_HIT_ME_OV2,
                &args![aggressor, s.health, s.blocked, last, position],
            );
        }
        let positive = e.call(ACTOR_GETTER_87F9C0, &args![this]).f64() > zero;
        let base = if positive {
            e.call(ACTOR_BASE_FORM_FN, &args![this]).u32()
        } else {
            0
        };
        if positive
            && !fn_0089d5e0(e, Ptr::new(base + 0x30))
            && !(s.source(e) != 0 && e.vcall(s.source(e), REFERENCE_SLOT_224, &args![]).bool())
        {
            let owner = this.addr() + 0xa4;
            let amount = e.call(FTOL, &args![s.total as f64]).u32();
            let mut speed = e.call(GET_KNOCKBACK_SPEED, &args![owner, amount]).f32();
            let aggressor_position = e.vcall(aggressor, ACTOR_SLOT_POSITION, &args![]).u32();
            let this_position = e.vcall(this.addr(), ACTOR_SLOT_POSITION, &args![]).u32();
            let difference = s.local(-0x410);
            let scaled = s.local(-0x428);
            let result_words = s.local(-0x41c);
            e.call(
                VECTOR_SUBTRACT,
                &args![this_position, difference, aggressor_position],
            );
            e.call(LIST_ITEM_SLOT, &args![result_words]);
            let cap = setting_float(e, SETTING_KNOCKBACK_CAP);
            if (cap as f64) < speed as f64 {
                speed = cap;
            }
            e.call(VECTOR_NORMALIZE, &args![difference]);
            let scaled_vector = e
                .call(VECTOR_SCALE, &args![difference, scaled, speed])
                .u32();
            let words = read_words3(e, scaled_vector);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(result_words + 4 * i as u32, *word);
            }
            let controller = e
                .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
                .u32();
            if controller != 0 {
                let modifier = setting_float(e, SETTING_VELOCITY_MODIFIER);
                e.call(
                    CHARACTER_SET_VELOCITY_MODIFIER,
                    &args![controller, result_words, modifier],
                );
            }
        }
    }
    e.call(ACTOR_FN_88E1E0, &args![this, hit]);
    // Whether the actor's process has a ragdoll body to push.
    let mut ragdoll_ready = false;
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if acquire != 0 && e.call(ACQUIRE_OBJECT_FIELD_28, &args![acquire]).u32() == 0 {
        let kind = fn_0089d620(e, Ptr::new(acquire));
        if kind != 0 && e.call(PACKAGE_FN_5F36F0, &args![kind]).i32() <= 1 {
            ragdoll_ready = true;
        }
    }
    if !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        hit_me_ragdoll_impulse(e, s, ragdoll_ready);
    }
    if !e.call(PICK_UP_GOES_TO_TASK_QUEUE, &args![]).bool() {
        e.call(ACTOR_FN_88E8D0, &args![this, s.health, hit]);
        if aggressor != 0 {
            e.call(ACTOR_FN_88FB00, &args![aggressor, s.health, hit, this]);
        }
    } else {
        let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
        e.call(QUEUE_ACTOR_HIT_FEEDBACK, &args![queue, this, hit]);
    }
    hit_me_player_feedback(e, s);
}

/// `0089ca53`..`0089ce0a`: pushes the ragdoll body the hit location names with
/// an impulse along the hit direction.
fn hit_me_ragdoll_impulse(e: &mut Engine, s: &mut HitMe, ready: bool) {
    let this = s.this;
    let hit = s.hit;
    if !ready || e.call(ACTOR_PROCESS_LEVEL_IS_3, &args![this]).bool() {
        return;
    }
    let ragdoll = e.get(this, Actor::pRagdollController).addr();
    if ragdoll == 0
        || !e.call(RAGDOLL_TEST_552490, &args![ragdoll]).bool()
        || fn_0089d670(e, Ptr::new(ragdoll)) == 0
    {
        return;
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![this]).u32();
    if e.vcall(acquire, PROCESS_SLOT_4BC, &args![]).u32() != 0 {
        return;
    }
    e.call(RAGDOLL_SET_FEEDBACK_ACTIVE, &args![ragdoll, 1u32]);
    if fn_0089d690(e, Ptr::new(ragdoll)) == 0 {
        return;
    }
    let mut body = 0u32;
    let transform = s.local(-0x500);
    let location = s.local(-0x510);
    let direction = s.local(-0x520);
    let cross = s.local(-0x530);
    let flags_holder = s.local(-0x538);
    let strength = s.local(-0x570);
    e.call(OBJECT_CONSTRUCT_4A3C20, &args![transform]);
    e.call(LIST_ITEM_SLOT, &args![location]);
    e.call(VECTOR_ASSIGN, &args![location, hit.addr() + 0x38]);
    e.call(LIST_ITEM_SLOT, &args![direction]);
    e.call(VECTOR_COPY_553FC0, &args![direction, hit.addr() + 0x44]);
    e.call(LIST_ITEM_SLOT, &args![cross]);
    e.call(VECTOR_CROSS_458880, &args![cross, direction, location]);
    let pick = e.call(RAGDOLL_PICK_C76320, &args![ragdoll]).u32();
    if s.kind_one {
        body = e.call(RAGDOLL_BODY_C762F0, &args![ragdoll]).u32();
    } else {
        e.call(OBJECT_FN_8C71B0, &args![flags_holder, 0u32]);
        e.call(OBJECT_FN_59CE80, &args![flags_holder, 0u32]);
        e.call(OBJECT_FN_4A39F0, &args![flags_holder, 8u32]);
        let flags = e.mem.u32(flags_holder);
        e.call(OBJECT_FN_4A3F70, &args![transform, flags]);
        e.call(VECTOR_FN_458A10, &args![location, location, direction]);
        e.call(OBJECT_FN_56D340, &args![transform, location]);
        e.call(OBJECT_FN_4A3F40, &args![transform, cross]);
        if pick != 0 && e.vcall(pick, PICK_SLOT_C8, &args![transform]).bool() {
            body = e.call(PICK_DATA_GET_RIGID_BODY, &args![transform]).u32();
        }
    }
    if body == 0 {
        return;
    }
    let reference = e.call(RIGID_BODY_REFERENCE_4B5A20, &args![body]).u32();
    let form = if reference != 0 {
        e.call(ENTRY_FORM, &args![reference]).u32()
    } else {
        0
    };
    let found = if form != 0 {
        e.call(FIND_REFERENCE_FOR_3D, &args![form]).u32()
    } else {
        0
    };
    if found != 0 && e.vcall(found, REFERENCE_SLOT_224, &args![]).bool() {
        return;
    }
    e.call(OBJECT_CONSTRUCT_62AEA0, &args![direction]);
    let mut impulse = fn_0089d6b0(e, Ptr::new(ragdoll));
    let weapon = s.weapon(e);
    if s.kind_one || (weapon != 0 && e.call(WEAPON_FORM_TEST_6450C0, &args![weapon]).bool()) {
        impulse = fn_0089d6d0(e, Ptr::new(ragdoll));
    }
    e.call(OBJECT_FN_5DBF20, &args![strength, impulse as f32]);
    e.call(OBJECT_FN_627920, &args![direction, strength]);
    let handle = e.call(PICK_HANDLE_541C80, &args![pick]).u32();
    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    e.call(RIGID_BODY_APPLY_LINEAR_IMPULSE, &args![body, direction]);
    let handle = e.call(PICK_HANDLE_541C80, &args![pick]).u32();
    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
}

/// `0089ce60`..`0089d0af`: the player's controller rumble and screen effect
/// for a hit that is not blocked.
fn hit_me_player_feedback(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let aggressor = s.aggressor(e);
    let source = s.source(e);
    if this.addr() == s.player {
        if aggressor != 0 && e.call(ENTRY_FORM, &args![VATS_SINGLETON]).u32() == 0 {
            let node = e.vcall(aggressor, ACTOR_SLOT_1D0, &args![]).u32();
            let strength = setting_float(e, SETTING_BLOCK_SCREEN_EFFECT);
            let modifier = e.call(IMAGE_SPACE_GET_HIT, &args![]).u32();
            e.call(IMAGE_SPACE_TRIGGER, &args![modifier, strength, node]);
        }
        if !e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).bool()
            && !e.call(VATS_TEST_525430, &args![VATS_SINGLETON]).bool()
        {
            hit_me_rumble(e, SETTING_HIT_RUMBLE_TIME, SETTING_HIT_RUMBLE_PULSE, None);
        }
    } else if aggressor == s.player
        && source == 0
        && !e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).bool()
    {
        if !e.call(VATS_TEST_525430, &args![VATS_SINGLETON]).bool() {
            hit_me_rumble(
                e,
                SETTING_HIT_OUT_RUMBLE_TIME,
                SETTING_HIT_OUT_RUMBLE_PULSE,
                None,
            );
        } else if e.call(VATS_TEST_7A9280, &args![VATS_SINGLETON]).bool() {
            let scale =
                e.call(VATS_SCALE_716440, &args![]).f64() * e.global::<f64>(THOUSAND_DOUBLE);
            hit_me_rumble(
                e,
                SETTING_HIT_OUT_RUMBLE_TIME,
                SETTING_HIT_OUT_RUMBLE_PULSE,
                Some(scale),
            );
            e.call(VATS_FN_4410D0, &args![VATS_SINGLETON, 0u32]);
        } else {
            let scale =
                e.call(VATS_SCALE_716440, &args![]).f64() * e.global::<f64>(THOUSAND_DOUBLE);
            hit_me_rumble(
                e,
                SETTING_HIT_OUT_RUMBLE_TIME,
                SETTING_HIT_OUT_RUMBLE_PULSE,
                Some(scale),
            );
        }
    }
}

/// `0089d110`..`0089d4f2`: the follower barks, the statistics for the
/// crippled and moved actor, the armor damage and the notices to the combat
/// controller and the combat dialogue manager.
fn hit_me_finish(e: &mut Engine, s: &mut HitMe) {
    let this = s.this;
    let hit = s.hit;
    let aggressor = s.aggressor(e);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        if e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
            if s.moved {
                e.call(TRIGGER_FOLLOWER_BARK, &args![this, 1u32]);
            } else if s.crippled {
                e.call(TRIGGER_FOLLOWER_BARK, &args![this, 2u32]);
                e.call(TRIGGER_FOLLOWER_BARK, &args![this, 6u32]);
            }
        }
        let mut base = e.call(REFR_BASE_FORM, &args![this]).u32();
        if e.call(ACTOR_TEST_56AF40, &args![this]).bool() {
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![this]).u32();
            base = e.call(EXTRA_GET_BASE_FORM_421720, &args![extra]).u32();
        }
        if aggressor == s.player && !e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
            let weapon = s.weapon(e);
            if s.crippled {
                let limb = limb_word(e, hit);
                e.call(
                    STATISTICS_WORKER_5F5950,
                    &args![
                        3u32,
                        1u32,
                        base,
                        weapon,
                        limb,
                        3u32,
                        u32::from(s.weapon_type)
                    ],
                );
            }
            if s.moved {
                let limb = limb_word(e, hit);
                e.call(
                    STATISTICS_WORKER_5F5950,
                    &args![
                        3u32,
                        1u32,
                        base,
                        weapon,
                        limb,
                        2u32,
                        u32::from(s.weapon_type)
                    ],
                );
            }
        }
    } else if s.not_fleeing
        && e.call(PROCESS_GET_FORCE_NEXT_UPDATE, &args![this]).bool()
        && !e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
    {
        e.call(TRIGGER_FOLLOWER_BARK, &args![this, 3u32]);
    }
    if !e.vcall(this.addr(), ACTOR_SLOT_22C, &args![0u32]).bool()
        && e.get(hit, HitData::fArmorDamage) as f64 > zero
        && e.vcall(this.addr(), ACTOR_SLOT_360, &args![]).bool()
    {
        let limb = e.get(hit, HitData::eDamageLimb);
        let prefer_head = limb == 1 || limb == 2;
        let item = actor_get_random_worn_armor(e, this, u8::from(prefer_head));
        if item != 0 {
            let damage = e.get(hit, HitData::fArmorDamage);
            e.vcall(
                this.addr(),
                ACTOR_SLOT_DAMAGE_EQUIPMENT,
                &args![item, damage, 0u32],
            );
            e.call(ENTRY_DELETE, &args![item, 1u32]);
        }
    }
    let mut target = aggressor;
    let source = s.source(e);
    if target == 0 && source != 0 {
        let owner = e.vcall(source, REFERENCE_SLOT_168, &args![]).u32();
        if owner != 0 && e.call(REFERENCE_TEST_5CA4F0, &args![owner]).bool() {
            target = e.call(ENTRY_FIRST_EXTRA_LIST, &args![owner]).u32();
        }
    }
    let mut controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
    if target != 0 {
        e.vcall(this.addr(), ACTOR_SLOT_HIT_NOTICE, &args![target, 0u32]);
    }
    if e.call(ACTOR_IS_TALKING, &args![this]).bool()
        || e.call(MOBILE_OBJECT_IS_IN_DIALOGUE, &args![this]).bool()
    {
        s.flag_64 = false;
    }
    let now = e.call(IMPORT_GET_TICK_COUNT, &args![]).u32();
    if e.global::<u32>(HIT_BARK_DEADLINE) > now {
        s.flag_64 = false;
    }
    let now = e.call(IMPORT_GET_TICK_COUNT, &args![]).u32();
    if e.global::<u32>(HIT_BARK_DEADLINE) < now {
        let start = e.call(IMPORT_GET_TICK_COUNT, &args![]).u32();
        let upper =
            setting_float(e, SETTING_BARK_DELAY_UPPER) as f64 * e.global::<f64>(THOUSAND_DOUBLE);
        let upper = e.call(FTOL, &args![upper]).u32();
        let lower =
            setting_float(e, SETTING_BARK_DELAY_LOWER) as f64 * e.global::<f64>(THOUSAND_DOUBLE);
        let lower = e.call(FTOL, &args![lower]).u32();
        let delay = e.call(RANDOM_RANGE_944460, &args![lower, upper]).u32();
        e.set_global(HIT_BARK_DEADLINE, start.wrapping_add(delay));
    }
    if controller == 0 {
        controller = e.vcall(this.addr(), ACTOR_SLOT_428, &args![]).u32();
        if controller != 0 {
            e.call(
                CONTROLLER_RECORD_DAMAGE,
                &args![controller, target, s.health],
            );
        }
    }
    if s.flag_64 {
        let manager = e.global::<u32>(COMBAT_DIALOGUE_MANAGER);
        e.call(
            COMBAT_DIALOGUE_START_DIALOGUE,
            &args![manager, this, aggressor, 2u32, 2u32, 1u32, controller],
        );
    }
    e.call(ACTOR_HIT_BRACKET_8A5300, &args![this]);
}
/// `bhkBlendController::DoHit` (Xbox PDB), cdecl: the first value found by
/// `004aae30`, a `float` and a flag.
const BLEND_CONTROLLER_DO_HIT: u32 = 0x00c9_b250;
/// `004aae30`: cdecl, takes what `0043fcd0` returned and the result of
/// `004c69f0`; returns an object or zero.
const FIND_BY_NAME_4AAE30: u32 = 0x004a_ae30;
/// The `this` of [`MESSAGE_TEXT`] for the first lookup of `fn_008a4af0`.
const HIT_NAME_SOURCE_FIRST: u32 = 0x011c_c83c;
/// The `this` of [`MESSAGE_TEXT`] for the fall-back lookup.
const HIT_NAME_SOURCE_FALLBACK: u32 = 0x011c_c8cc;
/// The `float` settings `fn_008a4af0` reads through `00450410` (the `this`
/// of each call): the pair behind the first amount ...
const HIT_SETTING_FIRST_A: u32 = 0x011c_c77c;
const HIT_SETTING_FIRST_B: u32 = 0x011c_c86c;
/// ... and the pair behind the second amount.
const HIT_SETTING_SECOND_A: u32 = 0x011c_c860;
const HIT_SETTING_SECOND_B: u32 = 0x011c_c7dc;
/// `100.0` as a `double`: the limit of the first percentage.
const HIT_LIMIT_DOUBLE: u32 = 0x0101_7a40;
/// `100.0f`: what the first percentage becomes above the limit.
const HIT_LIMIT_FLOAT: u32 = 0x0101_6410;
/// `0.01` as a `double`.
const HIT_PERCENT_SCALE: u32 = 0x0101_6408;
/// `75.0` as a `double`: a second amount above it clears the first.
const HIT_SECOND_CUTOFF: u32 = 0x0107_31a8;
/// `0.0` as a `double`.
const HIT_ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` as a `double`.
const HIT_ONE_DOUBLE: u32 = 0x0101_2070;
/// `30.0f`: handed to `008a50d0`.
const HIT_REACH: u32 = 0x0101_8f5c;
/// `0043b1b0`: thiscall on the result of `00413f40`, returns text.
const HIT_NAME_STRING_43B1B0: u32 = 0x0043_b1b0;
/// `00413f40`: thiscall, returns the object `0043b1b0` reads.
const HIT_TEXT_SOURCE_413F40: u32 = 0x0041_3f40;
/// `00c806b0`: cdecl (the value `0043fcd0` returned, the ray object, zero);
/// returns a result or zero.
const HIT_RAY_C806B0: u32 = 0x00c8_06b0;
/// `bhkCharacterProxy::operatorP` (Xbox PDB), `004ae750`: thiscall on the
/// ray result.
const HIT_RAY_RESULT_4AE750: u32 = 0x004a_e750;
/// `008a50f0`: thiscall on the ray object, takes a vector.
const HIT_RAY_SET_VECTOR_8A50F0: u32 = 0x008a_50f0;
/// `008a50d0`: thiscall on the ray object, takes a `float`.
const HIT_RAY_SET_REACH_8A50D0: u32 = 0x008a_50d0;
/// `008a5170`: thiscall on the member at `+0x410` of the character
/// controller, returns a flag.
const HIT_CONTROLLER_FLAG_8A5170: u32 = 0x008a_5170;
/// `008a5190`: returns the word at `011c6254`.
const HIT_GLOBAL_VALUE_8A5190: u32 = 0x008a_5190;
/// `00571530`: thiscall on the actor, takes what `0043fcd0` returned and a
/// word; returns a word.
const HIT_ACTOR_WORKER_571530: u32 = 0x0057_1530;
/// `00cb9320`: cdecl, takes the first object `004aae30` found, a vector and
/// a word.
const HIT_APPLY_CB9320: u32 = 0x00cb_9320;
/// `"Hit At %s\r\n"`.
const HIT_AT_FORMAT: u32 = 0x0108_4d8c;

/// `0044ddc0`: thiscall on a rigid body's reference, returns the object the
/// hit is applied to.
const HIT_BODY_OBJECT_44DDC0: u32 = 0x0044_ddc0;

// Translated from 008a4af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::HitMe_ov2` (Xbox PDB): thiscall on the actor that is hit,
/// `RET 0x14`. `source` is another actor whose slot `0x1d0` supplies the
/// object whose rotation turns the two vectors `offset_a` and `offset_b`
/// (addresses) into `first_point` and `second_point`; its slot `0x360` picks
/// the colour of the debug arrow.
///
/// Nothing happens when `008ace90` is true, or (after the vectors are
/// worked out) when `004aae30` finds nothing. Two amounts come from the
/// settings: `first = low + (high - low) * percent * 0.01` (`percent`
/// limited to 100) and `second = low + (high - low) * second_amount`; a
/// `second_amount` above 75 clears `first`; both zero ends the function.
/// Otherwise `bhkBlendController::DoHit` runs unless slot `0x230` is true
/// or, for the player, `004eaf60` is false. A non-zero `first` draws the
/// debug arrow (when the debug setting is positive) and, unless that
/// player check failed, calls `00c806b0` with the ray object and applies
/// the outcome with `00cb9320`; a non-zero `second` applies a second
/// outcome (`00571530`) the same way.
///
/// The `NiPoint3` constructor `006815c0` and `006240d0` do nothing and are
/// not called; the vectors stay zero until written (the exe leaves them
/// undefined). The exception frame is not translated.
pub fn actor_hit_me_ov2(
    e: &mut Engine,
    this: Ptr<Actor>,
    source: Ptr<Actor>,
    percent: f32,
    second_amount: f32,
    offset_a: u32,
    offset_b: u32,
) {
    // The exe aligns `EBP` to 16 bytes, with `SUB ESP,0x150` under it.
    e.with_stack(0x150 + 0x10, |e, raw| {
        let ebp = (raw.addr() + 0x150 + 0xf) & !0xf;
        hit_me_ov2_body(
            e,
            this,
            source,
            (percent, second_amount),
            (offset_a, offset_b),
            ebp,
        );
    });
}

/// `fn_008a4af0` proper; `locals` is zeroed memory for the vectors and
/// objects the exe keeps on its stack.
fn hit_me_ov2_body(
    e: &mut Engine,
    this: Ptr<Actor>,
    source: Ptr<Actor>,
    (mut percent, second_amount): (f32, f32),
    (offset_a, offset_b): (u32, u32),
    ebp: u32,
) {
    // The locals at their offsets under the exe's aligned `EBP`.
    let first_point = ebp - 0x1c;
    let second_point = ebp - 0x28;
    let direction = ebp - 0x50;
    let temp_a = ebp - 0x38;
    let temp_b = ebp - 0x44;
    let subtract_temp = ebp - 0x5c;
    let scaled = ebp - 0xa8;
    let color = ebp - 0xb8;
    let small_temp = ebp - 0xe4;
    let copy_a = ebp - 0x100;
    let ray = ebp - 0x110;
    let ray_vector = ebp - 0x120;
    let scaled_second = ebp - 0x138;
    let copy_b = ebp - 0x150;
    let collision = e.call(ACTOR_COLLISION_OBJECT, &args![this]).u32();
    if e.call(ACTOR_TEST_8ACE90, &args![this]).bool() {
        return;
    }
    let first_name = e.call(MESSAGE_TEXT, &args![HIT_NAME_SOURCE_FIRST]).u32();
    let found = e
        .call(FIND_BY_NAME_4AAE30, &args![collision, first_name])
        .u32();
    let node = e.vcall(source.addr(), ACTOR_SLOT_1D0, &args![]).u32();
    if node != 0 {
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        let result = e
            .call(MATRIX_TRANSFORM, &args![rotation, temp_a, offset_a])
            .u32();
        copy_vector(e, result, first_point);
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        let result = e
            .call(MATRIX_TRANSFORM, &args![rotation, temp_b, offset_b])
            .u32();
        copy_vector(e, result, second_point);
    }
    let result = e
        .call(
            VECTOR_SUBTRACT,
            &args![second_point, subtract_temp, first_point],
        )
        .u32();
    copy_vector(e, result, direction);
    e.call(VECTOR_NORMALIZE, &args![direction]);
    if found == 0 {
        return;
    }
    if percent as f64 > e.global::<f64>(HIT_LIMIT_DOUBLE) {
        percent = e.global::<f32>(HIT_LIMIT_FLOAT);
    }
    let high = hit_setting(e, HIT_SETTING_FIRST_A);
    let low = hit_setting(e, HIT_SETTING_FIRST_B);
    let span = (high as f64 - low as f64) as f32;
    let step = span as f64 * percent as f64 * e.global::<f64>(HIT_PERCENT_SCALE);
    let low = hit_setting(e, HIT_SETTING_FIRST_B);
    let mut first = (low as f64 + step) as f32;
    let high = hit_setting(e, HIT_SETTING_SECOND_A);
    let low = hit_setting(e, HIT_SETTING_SECOND_B);
    let span = (high as f64 - low as f64) as f32;
    let step = span as f64 * second_amount as f64;
    let low = hit_setting(e, HIT_SETTING_SECOND_B);
    let second = (low as f64 + step) as f32;
    if second_amount as f64 > e.global::<f64>(HIT_SECOND_CUTOFF) {
        first = 0.0;
    }
    let zero = e.global::<f64>(HIT_ZERO_DOUBLE);
    if first as f64 == zero && second as f64 == zero {
        return;
    }
    let mut full_hit = true;
    if this.addr() == e.global::<u32>(PLAYER_CHARACTER) {
        full_hit = e.call(PLAYER_FIRST_PERSON_CHECK, &args![this]).bool();
    }
    if !e.vcall(this.addr(), ACTOR_SLOT_230, &args![]).bool() && full_hit {
        let mut limb = 0u32;
        let controller = e
            .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![this])
            .u32();
        if controller != 0 {
            limb = e
                .call(HIT_CONTROLLER_FLAG_8A5170, &args![controller + 0x410])
                .u32()
                & 0xff;
        }
        e.call(BLEND_CONTROLLER_DO_HIT, &args![found, 0.0f32, limb]);
    }
    if first as f64 != zero {
        e.call(VECTOR_SCALE, &args![direction, scaled, first]);
        if e.call(DEBUG_DRAW_LEVEL, &args![DEBUG_DRAW_SETTING]).u32() as i32 > 0 {
            let zero_f = 0.0f32;
            let one_f = 1.0f32;
            e.call(
                COLOR_CONSTRUCT,
                &args![color, zero_f, zero_f, zero_f, zero_f],
            );
            let made = if e.vcall(source.addr(), ACTOR_SLOT_360, &args![]).bool() {
                e.call(
                    COLOR_CONSTRUCT,
                    &args![ebp - 0xc8, zero_f, zero_f, one_f, one_f],
                )
            } else {
                e.call(
                    COLOR_CONSTRUCT,
                    &args![ebp - 0xd8, one_f, zero_f, zero_f, one_f],
                )
            }
            .u32();
            for word in 0..4 {
                let value = e.mem.u32(made + 4 * word);
                e.mem.set_u32(color + 4 * word, value);
            }
            let width = e.global::<f32>(ARROW_WIDTH_SCALE);
            let arrow_vector = e
                .call(VECTOR_SCALE, &args![scaled, small_temp, width])
                .u32();
            let arrow = e.call(CREATE_DIR_ARROW, &args![arrow_vector, color]).u32();
            e.call(DEBUG_OBJECT_SET_POSITION, &args![arrow, second_point]);
            let seconds = e
                .call(DEBUG_DRAW_DURATION_GETTER, &args![DEBUG_DRAW_DURATION])
                .f32();
            let tes = e.global::<u32>(TES_SINGLETON);
            e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, arrow, seconds]);
        }
        if full_hit {
            // The exe constructs `copy_a`, `ray` and `ray_vector` with
            // constructors (`006815c0`, `006240d0`) that do nothing.
            e.call(VECTOR_COPY_553FC0, &args![copy_a, scaled]);
            e.call(VECTOR_ASSIGN, &args![ray_vector, second_point]);
            e.call(HIT_RAY_SET_VECTOR_8A50F0, &args![ray, ray_vector]);
            let reach = e.global::<f32>(HIT_REACH);
            e.call(HIT_RAY_SET_REACH_8A50D0, &args![ray, reach]);
            let mut picked = 0u32;
            let result = e.call(HIT_RAY_C806B0, &args![collision, ray, 0u32]).u32();
            if result != 0 {
                let proxy = e.call(HIT_RAY_RESULT_4AE750, &args![result]).u32();
                let body = e.call(RIGID_BODY_REFERENCE_4B5A20, &args![proxy]).u32();
                if body != 0 {
                    picked = e.call(HIT_BODY_OBJECT_44DDC0, &args![body]).u32();
                }
            }
            if picked == 0 {
                let name = e.call(MESSAGE_TEXT, &args![HIT_NAME_SOURCE_FALLBACK]).u32();
                picked = e.call(FIND_BY_NAME_4AAE30, &args![collision, name]).u32();
            }
            if picked != 0 {
                let object = e.call(HIT_TEXT_SOURCE_413F40, &args![picked]).u32();
                let text = e.call(HIT_NAME_STRING_43B1B0, &args![object]).u32();
                e.call(ERROR_REPORT, &args![HIT_AT_FORMAT, text]);
            }
            e.call(HIT_APPLY_CB9320, &args![found, copy_a, picked]);
        }
    }
    if second as f64 != zero && full_hit {
        let one = e.global::<f64>(HIT_ONE_DOUBLE);
        let x = (e.mem.f32(direction) as f64 + one) as f32;
        e.mem.set_f32(direction, x);
        let y = (e.mem.f32(direction + 4) as f64 + one) as f32;
        e.mem.set_f32(direction + 4, y);
        e.call(VECTOR_NORMALIZE, &args![direction]);
        e.call(VECTOR_SCALE, &args![direction, scaled_second, first]);
        e.call(VECTOR_COPY_553FC0, &args![copy_b, scaled_second]);
        let word = e.call(HIT_GLOBAL_VALUE_8A5190, &args![]).u32();
        let picked = e
            .call(HIT_ACTOR_WORKER_571530, &args![this, collision, word])
            .u32();
        e.call(HIT_APPLY_CB9320, &args![found, copy_b, picked]);
    }
}

/// Reads the `float` setting whose object is at `setting` (`00450410`).
fn hit_setting(e: &mut Engine, setting: u32) -> f32 {
    e.call(DEBUG_DRAW_DURATION_GETTER, &args![setting]).f32()
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00891d70, actor_get_gold_amount(Ptr<Actor>) -> u32),
        entry!(
            0x00891db0,
            actor_has_objects(Ptr<Actor>, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00891e00, fn_00891e00(Ptr<Actor>, Ptr, u32, u8)),
        entry!(
            0x00892460,
            actor_remove_picked_up_object_from_world(Ptr, u8)
        ),
        entry!(0x008924e0, actor_pay_gold_to_actor(Ptr<Actor>, u32, u32)),
        entry!(0x00892520, fn_00892520(Ptr<Actor>, Ptr, Ptr, u32)),
        entry!(
            0x008929a0,
            fn_008929a0(Ptr<Actor>, Ptr, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00892e90,
            fn_00892e90(Ptr<Actor>, Ptr, u32, Ptr, u32, u32) -> bool
        ),
        entry!(0x00893190, actor_should_talk_to(Ptr<Actor>, Ptr) -> bool),
        entry!(0x00893340, actor_init_package_locations(Ptr<Actor>, u8)),
        entry!(0x00893500, actor_get_fatigue(Ptr<Actor>) -> f32),
        entry!(0x00893530, actor_get_fatigue_percentage(Ptr<Actor>) -> f32),
        entry!(0x00893590, actor_get_health_percentage(Ptr<Actor>) -> f32),
        entry!(0x008935f0, actor_queue_attack(Ptr<Actor>, u32) -> bool),
        entry!(0x00893a40, actor_start_attack(Ptr<Actor>, u32) -> bool),
        entry!(0x00894900, fn_00894900(Ptr<Actor>) -> bool),
        entry!(0x00894940, fn_00894940(Ptr<Actor>)),
        entry!(0x00894cc0, actor_set_block(Ptr<Actor>, u8) -> bool),
        entry!(0x00894d60, actor_get_blocked(Ptr<Actor>) -> bool),
        entry!(0x00894d90, fn_00894d90(Ptr<Actor>, u32)),
        entry!(0x00894e90, fn_00894e90(Ptr<Actor>)),
        entry!(0x00894f90, fn_00894f90(Ptr<Actor>, u8) -> u32),
        entry!(0x008950f0, fn_008950f0(Ptr, f32)),
        entry!(0x00895110, actor_pick_animations(Ptr<Actor>, f32, f32)),
        entry!(0x00897890, fn_00897890(Ptr) -> bool),
        entry!(0x008978b0, fn_008978b0() -> u32),
        entry!(0x008978c0, fn_008978c0() -> u32),
        entry!(0x008978d0, fn_008978d0(Ptr) -> bool),
        entry!(0x008978f0, fn_008978f0(Ptr, u32, u8)),
        entry!(
            0x00897910,
            actor_get_anim_group(Ptr<Actor>, u32, u32, u8, Ptr) -> u16
        ),
        entry!(0x00897b30, fn_00897b30(Ptr) -> u32),
        entry!(
            0x00897b50,
            actor_get_anim_group_duration(Ptr<Actor>, u32) -> f32
        ),
        entry!(0x00897bd0, fn_00897bd0(Ptr<Actor>, u32)),
        entry!(
            0x00897de0,
            fn_00897de0(Ptr<Actor>, Ptr, u8, u8, u8, Ptr, Ptr, f32, f32)
        ),
        entry!(0x00898200, fn_00898200(Ptr, u8)),
        entry!(0x00898250, fn_00898250(Ptr, f32)),
        entry!(0x00898280, fn_00898280(Ptr, u8)),
        entry!(0x008982a0, fn_008982a0(Ptr, u8)),
        entry!(
            0x008982c0,
            actor_initiate_avoid_package(Ptr<Actor>, Ptr, f32)
        ),
        entry!(0x008984c0, actor_initiate_surface_package(Ptr<Actor>)),
        entry!(0x008985d0, fn_008985d0(Ptr<Actor>)),
        entry!(0x00898650, fn_00898650(Ptr<Actor>, Ptr<Actor>, f32)),
        entry!(0x00898fd0, fn_00898fd0(Ptr) -> bool),
        entry!(0x00898ff0, fn_00898ff0(Ptr<Actor>) -> bool),
        entry!(0x00899030, fn_00899030(Ptr<Actor>) -> u32),
        entry!(0x00899060, fn_00899060(Ptr<Actor>, u8)),
        entry!(
            0x008990a0,
            actor_handle_blocked_attack(Ptr<Actor>, u32, f32, Ptr<Actor>, u32)
        ),
        entry!(0x008990f0, fn_008990f0(Ptr<Actor>) -> u32),
        entry!(0x0089d5e0, fn_0089d5e0(Ptr) -> bool),
        entry!(0x0089d600, fn_0089d600(Ptr) -> Ptr),
        entry!(0x0089d620, fn_0089d620(Ptr) -> u32),
        entry!(0x0089d640, fn_0089d640(Ptr, u32, u32, u32)),
        entry!(0x0089d670, fn_0089d670(Ptr) -> u8),
        entry!(0x0089d690, fn_0089d690(Ptr) -> u8),
        entry!(0x0089d6b0, fn_0089d6b0(Ptr) -> u32),
        entry!(0x0089d6d0, fn_0089d6d0(Ptr) -> u32),
        entry!(
            0x0089d6f0,
            fn_0089d6f0(Ptr<Actor>, f32, f32, Ptr<Actor>) -> u32
        ),
        entry!(
            0x0089d8b0,
            actor_get_random_worn_armor(Ptr<Actor>, u8) -> u32
        ),
        entry!(0x0089f4e0, fn_0089f4e0(Ptr) -> u32),
        entry!(0x0089f500, fn_0089f500(Ptr) -> u8),
        entry!(0x0089f520, fn_0089f520(Ptr, u8)),
        entry!(0x0089f540, fn_0089f540(Ptr) -> u8),
        entry!(0x0089f560, fn_0089f560(Ptr)),
        entry!(0x0089f580, fn_0089f580(Ptr<Actor>)),
        entry!(0x008a0250, fn_008a0250(Ptr)),
        entry!(0x008a0330, fn_008a0330(Ptr<Actor>) -> u32),
        entry!(0x0089f780, actor_resurrect(Ptr<Actor>, u8, u8, u8)),
        entry!(0x0089fb80, fn_0089fb80(Ptr<Actor>, u8)),
        entry!(
            0x0089fcf0,
            fn_0089fcf0(Ptr<Actor>, Ptr<Actor>, u32, u8, u32, u8, i32, u32, u32)
        ),
        entry!(0x008a0270, fn_008a0270(Ptr<Actor>, u8) -> u8),
        entry!(0x008a02d0, fn_008a02d0(Ptr<Actor>) -> u32),
        entry!(0x008a0360, fn_008a0360(Ptr<Actor>) -> u32),
        entry!(0x008a03a0, fn_008a03a0(Ptr<Actor>)),
        entry!(0x008a05f0, fn_008a05f0(Ptr<Actor>)),
        entry!(0x008a0680, fn_008a0680(Ptr<Actor>, u32, u32)),
        entry!(0x008987f0, fn_008987f0(Ptr<Actor>, Ptr<Actor>, u32)),
        entry!(0x00899200, fn_00899200(Ptr<Actor>, u8, u8)),
        entry!(
            0x00899cb0,
            actor_combat_hit(Ptr<Actor>, Ptr<Actor>, u8, Ptr, u8)
        ),
        entry!(0x0089d900, actor_kill(Ptr<Actor>, Ptr<Actor>, f32)),
        entry!(0x008a06c0, actor_stop_combat(Ptr<Actor>, Ptr<Actor>)),
        entry!(0x008a08c0, fn_008a08c0(Ptr) -> u8),
        entry!(0x008a08e0, actor_clear_in_combat(Ptr<Actor>, u8)),
        entry!(
            0x008a0960,
            actor_restore_full_health_and_conditions(Ptr<Actor>)
        ),
        entry!(0x008a0a90, fn_008a0a90(u32) -> f64),
        entry!(0x008a0ad0, fn_008a0ad0(u32) -> f64),
        entry!(0x008a0b10, actor_get_current_speed(Ptr<Actor>) -> f32),
        entry!(0x008a0c20, fn_008a0c20(Ptr<Actor>) -> f32),
        entry!(0x008a0c60, fn_008a0c60(Ptr<Actor>) -> f32),
        entry!(0x008a0cd0, fn_008a0cd0(u32) -> f64),
        entry!(
            0x008a0d10,
            actor_get_detection_level_against_actor(
                Ptr<Actor>,
                u8,
                Ptr<Actor>,
                Ptr,
                u8,
                u8,
                u32,
                Ptr,
            ) -> i32
        ),
        entry!(0x008a16b0, fn_008a16b0(Ptr<Actor>) -> u32),
        entry!(0x008a16d0, actor_is_weapon_drawn(Ptr<Actor>) -> u8),
        entry!(0x008a1710, actor_get_current_weapon(Ptr<Actor>) -> u32),
        entry!(0x008a1760, fn_008a1760(Ptr<Actor>) -> u32),
        entry!(0x008a1800, actor_set_life_state(Ptr<Actor>, u32)),
        entry!(0x008a1a40, fn_008a1a40(Ptr<Actor>, u32)),
        entry!(0x008a1a70, actor_apply_critical_stage(Ptr<Actor>)),
        entry!(
            0x008a1bd0,
            fn_008a1bd0(Ptr<Actor>, Ptr, Ptr, u32, u8, u8, u32, u8) -> u32
        ),
        entry!(0x008a2d20, fn_008a2d20() -> f32),
        entry!(0x008a2d40, fn_008a2d40(Ptr<Actor>) -> u32),
        entry!(0x008a2ed0, fn_008a2ed0(Ptr<Actor>) -> u32),
        entry!(0x008a2fa0, fn_008a2fa0(Ptr<Actor>, Ptr) -> u32),
        entry!(0x008a30f0, fn_008a30f0() -> u32),
        entry!(0x008a3100, fn_008a3100(Ptr<Actor>, f32)),
        entry!(0x008a3b30, fn_008a3b30(Ptr<Actor>) -> u8),
        entry!(0x008a3b50, fn_008a3b50(Ptr) -> f32),
        entry!(0x008a3b70, fn_008a3b70(Ptr, u32, u8)),
        entry!(0x008a3bd0, fn_008a3bd0(Ptr) -> u8),
        entry!(0x008a3bf0, fn_008a3bf0(Ptr, u8)),
        entry!(0x008a3c10, fn_008a3c10() -> f32),
        entry!(0x008a3c30, fn_008a3c30() -> f32),
        entry!(0x008a3c50, fn_008a3c50(Ptr<Actor>, u32)),
        entry!(0x008a3ed0, fn_008a3ed0(Ptr<Actor>) -> u32),
        entry!(0x008a40e0, fn_008a40e0(Ptr<Actor>, u32) -> u8),
        entry!(0x008a46a0, fn_008a46a0(Ptr) -> u8),
        entry!(0x008a46c0, fn_008a46c0(Ptr<Actor>, Ptr<Actor>) -> f32),
        entry!(
            0x008a4810,
            actor_can_head_track(Ptr<Actor>, Ptr<Actor>, u8) -> u8
        ),
        entry!(
            0x008a20d0,
            fn_008a20d0(
                Ptr<Actor>,
                u32,
                Ptr,
                u32,
                u32,
                i32,
                u32,
                u32,
                Ptr<Actor>,
                u8,
                u8,
                u8,
                u8,
                u8,
            ) -> f32
        ),
        entry!(0x0089a760, actor_hit_me(Ptr<Actor>, Ptr<HitData>, u8)),
        entry!(
            0x008a4af0,
            actor_hit_me_ov2(Ptr<Actor>, Ptr<Actor>, f32, f32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every function outside this file that the translations call. The test
    /// engine registers a double returning 0 at each, so that a test names
    /// only the ones whose results matter (and the real translations of other
    /// units never run).
    const EXTERNAL_CALLEES: &[u32] = &[
        0x00401170, 0x00403550, 0x00403e20, 0x00408840, 0x00408d60, 0x00408da0, 0x004181e0,
        0x00418520, 0x00418660, 0x00418770, 0x00418890, 0x00418900, 0x00418ab0, 0x00419700,
        0x0041a250, 0x0041a800, 0x0041aeb0, 0x0041aed0, 0x0041c7f0, 0x0041c8d0, 0x0041ca90,
        0x004216f0, 0x00425fd0, 0x0042cde0, 0x0042eb60, 0x004301b0, 0x00437bd0, 0x00437bf0,
        0x00440da0, 0x004459e0, 0x00446390, 0x00448a60, 0x00448a80, 0x0044ddc0, 0x00453700,
        0x004537b0, 0x00453a70, 0x0045bc00, 0x0045cd60, 0x00460250, 0x004702f0, 0x00470470,
        0x00474a40, 0x00474a80, 0x00474cb0, 0x00476b70, 0x0047d3b0, 0x00483710, 0x00483a00,
        0x00484e60, 0x0048e8a0, 0x0048f7f0, 0x00491040, 0x004937c0, 0x004937e0, 0x00493800,
        0x00493bb0, 0x00494300, 0x00494710, 0x00494740, 0x004954e0, 0x00495740, 0x00496080,
        0x00496500, 0x004974a0, 0x004981f0, 0x00498290, 0x004985b0, 0x004985f0, 0x00498910,
        0x00498cf0, 0x004994f0, 0x004997b0, 0x004bda70, 0x004bdcc0, 0x004bddd0, 0x004be330,
        0x004c0c30, 0x004c0c90, 0x004c69f0, 0x004cb320, 0x004cb4b0, 0x004cffe0, 0x004d0650,
        0x004dff00, 0x004e4620, 0x004eaf60, 0x004eec60, 0x004f1540, 0x004fe160, 0x00500940,
        0x00508100, 0x00511840, 0x0051e2a0, 0x0051f5f0, 0x005224c0, 0x00524b40, 0x00524b60,
        0x00524d10, 0x00525980, 0x00546a40, 0x0054ca90, 0x00551110, 0x00551180, 0x005585e0,
        0x00559450, 0x0055b980, 0x0055d520, 0x00564db0, 0x005653d0, 0x00566950, 0x00567790,
        0x00572230, 0x00572d30, 0x00573800, 0x00574b30, 0x00574fa0, 0x00575770, 0x00576260,
        0x005785e0, 0x00579670, 0x00598040, 0x0059bb30, 0x005b5e40, 0x005c0880, 0x005d0b80,
        0x005d43c0, 0x005e3fc0, 0x005e58f0, 0x005f2370, 0x005f23a0, 0x005f23c0, 0x005f2420,
        0x005f2440, 0x005f2540, 0x005f2670, 0x005f2700, 0x005f2b60, 0x005f3780, 0x005f4cc0,
        0x005f4d60, 0x00600900, 0x00600920, 0x00600940, 0x00639aa0, 0x006450c0, 0x00646020,
        0x006477b0, 0x006477e0, 0x00670b30, 0x00670b90, 0x00670fc0, 0x00671d10, 0x00671d30,
        0x00672fc0, 0x00675a50, 0x00675c20, 0x00675de0, 0x006777b0, 0x00678610, 0x00678ca0,
        0x0067a1b0, 0x0067a720, 0x0067f030, 0x0067f1f0, 0x0067f3c0, 0x0067f410, 0x0067ff70,
        0x006800b0, 0x00680110, 0x006815c0, 0x0068a7d0, 0x006ca4e0, 0x00702360, 0x007043c0,
        0x00704af0, 0x00705020, 0x00705040, 0x007052f0, 0x00709430, 0x00709470, 0x0070f490,
        0x00726070, 0x007af430, 0x007b3fa0, 0x008041a0, 0x00810530, 0x00815870, 0x008216c0,
        0x00825c00, 0x00826b40, 0x00826b90, 0x008391b0, 0x0084e3a0, 0x00867f90, 0x00877720,
        0x0087afa0, 0x0087faa0, 0x00881620, 0x00881650, 0x008843a0, 0x008846e0, 0x00884dc0,
        0x00884eb0, 0x0088b0f0, 0x0088c650, 0x0088c830, 0x00899200, 0x008a0330, 0x008a16d0,
        0x008a5340, 0x008a6840, 0x008a6970, 0x008a6a40, 0x008a73e0, 0x008a7570, 0x008a78f0,
        0x008aded0, 0x008b28c0, 0x008b3ad0, 0x008b70d0, 0x008ba3e0, 0x008ba410, 0x008bb650,
        0x008bc9d0, 0x008be940, 0x008bfa40, 0x008c7aa0, 0x008d6a80, 0x008d6f30, 0x008d8520,
        0x009306d0, 0x00933270, 0x00933840, 0x009344a0, 0x00944460, 0x00950a60, 0x00950b00,
        0x00966a70, 0x00969040, 0x0096f450, 0x00974db0, 0x009818e0, 0x009819d0, 0x00984f60,
        0x00994ef0, 0x009f0e00, 0x009f1140, 0x009f1310, 0x00a24660, 0x00a5bdd0, 0x00ad7480,
        0x00ad8830, 0x00ad8ce0, 0x00c6a270, 0x00c6bd00, 0x00c74890, 0x00ec43fb, 0x00ec62c0,
        0x00931850, 0x0087aff0, 0x008d5cb0, 0x0097fa10, 0x008b87a0, 0x00891b90, 0x005b6f70,
        0x00703c00, 0x006463e0, 0x006447f0, 0x00567400, 0x009a60e0, 0x008d0600, 0x0097f7f0,
        0x0097f6d0, 0x0087f3d0, 0x008808a0, 0x00579220, 0x00418b10, 0x00891b70, 0x0046e8c0,
        0x0045bb80, 0x00461130, 0x00a592c0, 0x0056acb0, 0x00461580, 0x0089d900, 0x00422c20,
        0x0042e8e0, 0x005a29b0, 0x008a1a40, 0x00884f80, 0x00972aa0, 0x0041cf30, 0x00633c90,
        0x00c6a350, 0x009370b0, 0x0096d470, 0x0096d450, 0x00906dc0, 0x004511e0, 0x00458be0,
        0x00444850, 0x00450ff0, 0x009334b0, 0x0040fbf0, 0x0040fba0, 0x0047c850, 0x0047d0b0,
        0x006ebd10, 0x00574920, 0x006047c0, 0x005f9e00, 0x00441b00, 0x005723b0, 0x0087f9c0,
        0x00967220, 0x00703350, 0x00703860, 0x00881680, 0x009922b0, 0x00992110, 0x009923d0,
        0x0097f930, 0x004030b0, 0x0040fbe0, 0x007014e0, 0x00576d30, 0x00981450, 0x005a03f0,
        0x00440d80, 0x008a8230, 0x0093a690, 0x009818b0, 0x0097da50, 0x0097e4b0, 0x0097efe0,
        0x008a1800, 0x00881510, 0x008ace90, 0x004f8960, 0x008a0d10, 0x0067a770, 0x005a3790,
        0x0043d4d0, 0x00422670, 0x00422590, 0x009839b0, 0x008a67f0, 0x009336c0, 0x00970a20,
        0x00970b30, 0x008c0460, 0x008b0670, 0x008b0ba0, 0x008bffc0, 0x008b06d0, 0x008a6650,
        0x005d8a70, 0x008256d0, 0x0047ead0, 0x0097f580, 0x008a1710, 0x009c71c0, 0x0058db10,
        0x004d5c60, 0x004503f0, 0x00414430, 0x00524c40, 0x00439ef0, 0x004a0c10, 0x0045bb20,
        0x00c59a50, 0x00440460, 0x00450410, 0x00458e20, 0x0087b040, 0x008bc240, 0x005fbeb0,
        0x009611e0, 0x008ae660, 0x0097f7d0, 0x00ad8b60, 0x00ad8f20, 0x00406090, 0x004a8ae0,
        0x00899cb0, 0x0089a760, 0x00437b90, 0x009b4d90, 0x008c1c30, 0x008c1c60, 0x009b4ec0,
        0x009b5120, 0x00444d00, 0x00407e00, 0x00476c90, 0x00525b20, 0x004fd3c0, 0x00885d70,
        0x0058cba0, 0x008397f0, 0x00837520, 0x004bd350, 0x009627a0, 0x006758f0, 0x00524cd0,
        0x008814b0, 0x00418a00, 0x0041a540, 0x004189c0, 0x0041a3e0, 0x00418940, 0x0041a090,
        0x00ad8930, 0x00ad8da0, 0x00ad8d10, 0x00ad88f0, 0x00ad73b0, 0x00ad7550, 0x00934250,
        0x00c8f210, 0x00c7c150, 0x008a08e0, 0x008a5eb0, 0x008a0960, 0x00815b00, 0x00568020,
        0x00ec623a, 0x00403df0, 0x0096e2f0, 0x005678a0, 0x00953f80, 0x0047e040, 0x008b8490,
        0x0094fd30, 0x00967160, 0x00525620, 0x00670700, 0x0087f9f0, 0x006705b0, 0x005be4d0,
        0x00648c80, 0x00406ce0, 0x00525420, 0x005f92d0, 0x008324e0, 0x00830680, 0x009c8950,
        0x00702450, 0x007e1980, 0x008adcb0, 0x009ca1f0, 0x0080ced0, 0x00867e30, 0x00459060,
        0x0041cda0, 0x0043fcd0, 0x005532a0, 0x0066ca00, 0x004a3e00, 0x00931fb0, 0x0087eeb0,
        0x0087eef0, 0x00630b40, 0x004586d0, 0x00458620, 0x004a0c90, 0x004b4500, 0x00812b00,
        0x0043d410, 0x00416870, 0x00a59c60, 0x00439180, 0x0062b8d0, 0x0062b930, 0x009931c0,
        0x00487f50, 0x008b01c0, 0x00565870, 0x00ca20e0, 0x00931ed0, 0x004a3a20, 0x00ca2ad0,
        0x00454af0, 0x0094ed40, 0x008b7c00, 0x0093e530, 0x00974a50, 0x009ab9a0, 0x0056af40,
        0x00421720, 0x0059f3a0, 0x005f5950, 0x008d85e0, 0x00881130, 0x004037d0, 0x00406d30,
        0x00408820, 0x0040c0e0, 0x00430830, 0x00436aa0, 0x00450f90, 0x004534f0, 0x004538a0,
        0x004538c0, 0x00457990, 0x0045bad0, 0x00460140, 0x0046ffb0, 0x00472380, 0x004938e0,
        0x004ab230, 0x004ade00, 0x004b13c0, 0x004b15e0, 0x004bf220, 0x004c0bd0, 0x004c6a10,
        0x004d1180, 0x004d1360, 0x004d52d0, 0x004d5850, 0x004d5930, 0x004d5a90, 0x004d5ad0,
        0x00514450, 0x00519b00, 0x00537bd0, 0x00560d30, 0x00568680, 0x00573090, 0x00577250,
        0x005790b0, 0x005790d0, 0x0057ace0, 0x0057ad20, 0x0057b520, 0x005ac750, 0x005ae3d0,
        0x005b7470, 0x005be5c0, 0x005c90d0, 0x005f36f0, 0x0061b320, 0x0061f150, 0x0061f170,
        0x00639b40, 0x0063c8a0, 0x00642e80, 0x00642ed0, 0x0066ef50, 0x00672dd0, 0x0067a690,
        0x0067f390, 0x00680050, 0x00680080, 0x00705210, 0x00717e50, 0x0083c7b0, 0x0083c820,
        0x0087b810, 0x00880910, 0x008845a0, 0x008853a0, 0x00885520, 0x00885d90, 0x00885ed0,
        0x00886010, 0x00887d00, 0x00888a50, 0x0088b740, 0x0088b880, 0x0088c570, 0x00891be0,
        0x008b0190, 0x008b3ab0, 0x008b3bb0, 0x008bb520, 0x008c1bc0, 0x008d0430, 0x008d8270,
        0x008fe8f0, 0x00905f30, 0x00933150, 0x00935cc0, 0x00935f00, 0x00936a20, 0x00936ac0,
        0x0093a660, 0x00953c50, 0x00968670, 0x0096a2d0, 0x0097f4d0, 0x0097fd60, 0x009dcb30,
        0x009f4300, 0x00a5c570, 0x00ad8570, 0x00ad8870, 0x00ad89b0, 0x00ad8b30, 0x00ad8be0,
        0x00ad8e60, 0x00ad9030, 0x00ad9060, 0x00adbfd0, 0x00adc050, 0x00c75580, 0x00c757b0,
        0x00c803a0, 0x00c80ce0, 0x00406d00, 0x004410d0, 0x00458880, 0x00458a10, 0x00476c70,
        0x004839c0, 0x00497f20, 0x00498d30, 0x004a39f0, 0x004a3c20, 0x004a3c90, 0x004a3f40,
        0x004a3f70, 0x004b5a20, 0x0051f510, 0x00525430, 0x005299a0, 0x00541c80, 0x00552490,
        0x00553fc0, 0x0056d340, 0x0056f930, 0x0059ce80, 0x005ac340, 0x005ca4f0, 0x005d2860,
        0x005dbf20, 0x005e50f0, 0x005e5190, 0x005f65d0, 0x005ff160, 0x00600950, 0x00627920,
        0x0062aea0, 0x00646580, 0x00646cb0, 0x00647ac0, 0x00647b70, 0x0066e950, 0x007038b0,
        0x00716440, 0x007a9280, 0x00815d00, 0x008182d0, 0x00824110, 0x0087ada0, 0x0087ae10,
        0x0088d640, 0x0088e1e0, 0x0088e8d0, 0x0088fb00, 0x008a51f0, 0x008a5230, 0x008a52c0,
        0x008a5300, 0x008ae000, 0x008b43a0, 0x008c71b0, 0x008d0300, 0x008d0370, 0x00974e90,
        0x009a6ae0, 0x009ac9c0, 0x009b1720, 0x00a255b0, 0x00c66fd0, 0x00c6d4d0, 0x00c762f0,
        0x00c76320, 0x00c7b6a0, 0x00fdf060,
    ];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// The test engine: doubles returning 0 at every external callee, and the
    /// page holding `thePlayer`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in EXTERNAL_CALLEES {
            e.register_double(*address, |_, _| Ret::default());
        }
        e.map(PLAYER_CHARACTER, 4);
        e.call_log = Some(vec![]);
        e
    }

    /// Makes `address` return `value` in EAX.
    fn stub(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| ret(value));
    }

    /// Makes `address` return a `float` in ST0.
    fn stub_float(e: &mut Engine, address: u32, value: f32) {
        e.register_double(address, move |_, _| float_ret(value));
    }

    /// Makes `address` run `body` (a double that sees its arguments).
    fn stub_with(
        e: &mut Engine,
        address: u32,
        body: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        e.register_double(address, body);
    }

    /// Allocates an object of `size` bytes whose virtual table has one double
    /// per `(slot offset, result)` pair; returns the object.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, Ret)]) -> Ptr {
        let table = e.mem.alloc(0x800);
        for (offset, result) in slots {
            let target = e.mem.alloc(8);
            let result = *result;
            e.register_double(target, move |_, _| result);
            e.mem.set_u32(table + offset, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        Ptr::new(object)
    }

    /// The function a virtual slot of `object` points at.
    fn slot_target(e: &Engine, object: Ptr, offset: u32) -> u32 {
        let table = e.mem.u32(object.addr());
        e.mem.u32(table + offset)
    }

    /// An actor with the given virtual slots, and (if any) a process object
    /// with its own slots.
    fn actor_with(
        e: &mut Engine,
        slots: &[(u32, Ret)],
        process_slots: Option<&[(u32, Ret)]>,
    ) -> Ptr<Actor> {
        let actor = object(e, 0x200, slots).cast::<Actor>();
        if let Some(process_slots) = process_slots {
            let process = object(e, 0x800, process_slots);
            e.set(actor, Actor::pCurrentProcess, process);
        }
        actor
    }

    /// Sets `thePlayer`.
    fn set_player(e: &mut Engine, player: Ptr<Actor>) {
        e.set_global(PLAYER_CHARACTER, player.addr());
    }

    /// The argument lists of every logged call to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn call_count(e: &Engine, address: u32) -> usize {
        calls_to(e, address).len()
    }

    /// Writes the animation group table entry of group `number`: the slot
    /// mode word (`+8`) and the kind word (`+0xc`).
    fn set_group(e: &mut Engine, number: u32, mode: u32, kind: u32) {
        let base = ANIM_GROUP_TABLE + number * 0x24;
        e.map(base, 0x24);
        e.mem.set_u32(base + 8, mode);
        e.mem.set_u32(base + 0xc, kind);
    }

    fn clear_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// Installs a virtual table with the given slots at `address` (the
    /// address of a sub-object's vtable pointer).
    fn vtable_at(e: &mut Engine, address: u32, slots: &[(u32, Ret)]) {
        let table = e.mem.alloc(0x100);
        for (offset, result) in slots {
            let target = e.mem.alloc(8);
            let result = *result;
            e.register_double(target, move |_, _| result);
            e.mem.set_u32(table + offset, target);
        }
        e.mem.set_u32(address, table);
    }

    #[test]
    fn gold_amount_comes_from_the_inventory_changes() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        stub(&mut e, REFR_EXTRA_DATA_LIST, 0x1000);
        assert_eq!(e.call(0x0089_1d70, &args![actor]).u32(), 0);
        stub(&mut e, GET_CONTAINER_CHANGES, 0x2000);
        stub(&mut e, INVENTORY_CHANGES_GET_GOLD_AMOUNT, 150);
        assert_eq!(actor_get_gold_amount(&mut e, actor), 150);
        assert_eq!(calls_to(&e, GET_CONTAINER_CHANGES), vec![vec![0x1000]; 2]);
        assert_eq!(
            calls_to(&e, INVENTORY_CHANGES_GET_GOLD_AMOUNT),
            vec![vec![0x2000]]
        );
    }

    #[test]
    fn has_objects_passes_its_words_on_with_two_swapped() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        stub(&mut e, GET_CONTAINER_CHANGES, 0);
        assert!(!e
            .call(0x0089_1db0, &args![actor, 1u32, 2u32, 3u32, 4u32, 5u32])
            .bool());
        assert_eq!(call_count(&e, CONTAINER_HAS_OBJECTS), 0);

        stub(&mut e, GET_CONTAINER_CHANGES, 0x2000);
        stub(&mut e, CONTAINER_HAS_OBJECTS, 1);
        assert!(actor_has_objects(&mut e, actor, 1, 2, 3, 4, 5));
        assert_eq!(
            calls_to(&e, CONTAINER_HAS_OBJECTS),
            vec![vec![0x2000, 1, 2, 3, 5, 4, actor.addr()]]
        );
    }

    #[test]
    fn picked_up_objects_are_marked_or_removed() {
        let mut e = engine();
        let refr = object(&mut e, 0x100, &[(0x1d0, ret(0x77)), (0x1c0, ret(0))]);
        stub(&mut e, REFR_PARENT_CELL, 0x9000);
        // Not deleted: only marked as picked up.
        actor_remove_picked_up_object_from_world(&mut e, refr, 0);
        assert_eq!(calls_to(&e, BHK_WORLD_ACTIVATE), vec![vec![0x77, 1, 1, 0]]);
        assert_eq!(calls_to(&e, MARK_AS_PICKED_UP), vec![vec![refr.addr()]]);
        assert_eq!(call_count(&e, GARBAGE_COLLECTOR_ADD), 0);
        // Deleted: detached, garbage collected and removed from its cell.
        assert_eq!(e.call(0x0089_2460, &args![refr, 1u32]).u32(), 0);
        assert_eq!(call_count(&e, MARK_AS_PICKED_UP), 1);
        assert_eq!(calls_to(&e, GARBAGE_COLLECTOR_ADD), vec![vec![refr.addr()]]);
        assert_eq!(
            calls_to(&e, CELL_REMOVE_REFERENCE),
            vec![vec![0x9000, refr.addr()]]
        );
        assert_eq!(call_count(&e, slot_target(&e, refr, 0x1c0)), 1);
    }

    #[test]
    fn paying_gold_swaps_the_two_words() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        stub(&mut e, GET_CONTAINER_CHANGES, 0x2000);
        e.call(0x0089_24e0, &args![actor, 7u32, 8u32]);
        assert_eq!(
            calls_to(&e, INVENTORY_CHANGES_REMOVE_GOLD),
            vec![vec![0x2000, actor.addr(), 8, 7]]
        );
    }

    #[test]
    fn fatigue_and_health_read_the_actor_value_owner() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        let owner = actor.addr() + 0xa4;
        vtable_at(&mut e, owner, &[(0x00, ret(100)), (0x0c, float_ret(25.0))]);
        assert_eq!(e.call(0x0089_3500, &args![actor]).f32(), 25.0);
        assert_eq!(e.call(0x0089_3530, &args![actor]).f32(), 0.25);
        assert_eq!(actor_get_health_percentage(&mut e, actor), 0.25);
        let first = calls_to(&e, slot_target(&e, Ptr::new(owner - 0xa4 + 0xa4), 0));
        let _ = first;
        // The actor value numbers are passed in.
        let table = e.mem.u32(owner);
        let base_slot = e.mem.u32(table);
        let current_slot = e.mem.u32(table + 0x0c);
        assert_eq!(calls_to(&e, current_slot)[0], vec![owner, 0x16]);
        assert_eq!(calls_to(&e, base_slot).last().unwrap(), &vec![owner, 0x10]);
        // A base of 0 gives 1.0.
        vtable_at(&mut e, owner, &[(0x00, ret(0)), (0x0c, float_ret(25.0))]);
        assert_eq!(actor_get_fatigue_percentage(&mut e, actor), 1.0);
    }

    const BASE_FORM: u32 = 0x0000_b000;

    /// An actor with a process (no weapon, no ammunition) and a reference of
    /// a plain form type 5, as `fn_00891e00` and `fn_00892520` expect.
    fn pickup_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr) {
        let actor = actor_with(
            e,
            &[
                (0x218, ret(0)),
                (0x22c, ret(0)),
                (0x428, ret(0)),
                (0x1e4, ret(0)),
                (0x3bc, ret(0)),
                (0x17c, ret(0)),
                (0x3c4, ret(0)),
            ],
            Some(&[(0x14c, ret(0)), (0x148, ret(0))]),
        );
        let refr = object(
            e,
            0x100,
            &[
                (0x4c, ret(0)),
                (0x224, ret(0)),
                (0xc4, ret(0)),
                (0x48, ret(0)),
                (0x1d0, ret(0x77)),
                (0x1c0, ret(0)),
            ],
        );
        stub(e, REFR_BASE_FORM, BASE_FORM);
        stub(e, FORM_TYPE, 5);
        stub(e, REFR_PARENT_CELL, 0x9000);
        stub(e, REFR_EXTRA_DATA_LIST, 0x5000);
        (actor, refr)
    }

    #[test]
    fn picking_up_a_plain_object_deletes_it_from_the_world() {
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        e.call(0x0089_1e00, &args![actor, refr, 3u32, 0u32]);
        assert_eq!(
            calls_to(&e, ACTOR_PICK_UP_WORKER),
            vec![vec![actor.addr(), refr.addr(), 3, 0, 0]]
        );
        // The reference is released (virtual slot 0x4c with 0) and removed.
        assert_eq!(
            calls_to(&e, slot_target(&e, refr, 0x4c)),
            vec![vec![refr.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, refr, 0xc4)),
            vec![vec![refr.addr(), 1]]
        );
        assert_eq!(calls_to(&e, GARBAGE_COLLECTOR_ADD), vec![vec![refr.addr()]]);
        assert_eq!(call_count(&e, MARK_AS_PICKED_UP), 0);
        // Actors that had it as a target were looked up by form id.
        assert_eq!(call_count(&e, PROCESS_LISTS_FIND_ACTORS), 1);
    }

    #[test]
    fn a_persistent_reference_is_marked_and_queued() {
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        stub(&mut e, REFR_GET_REF_PERSISTS, 1);
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, GET_TES, 0x4321);
        fn_00891e00(&mut e, actor, refr, 3, 1);
        assert_eq!(
            calls_to(&e, ACTOR_PICK_UP_WORKER),
            vec![vec![actor.addr(), refr.addr(), 3, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, TES_QUEUE_PICK_UP),
            vec![vec![0x4321, refr.addr(), 0]]
        );
        assert_eq!(call_count(&e, GARBAGE_COLLECTOR_ADD), 0);
        assert_eq!(call_count(&e, MARK_AS_PICKED_UP), 0);
    }

    #[test]
    fn picking_up_stolen_goods_raises_an_alarm_and_takes_ownership() {
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        // Slot 0x218 true: the ownership checks run.
        let actor = {
            let table = e.mem.u32(actor.addr());
            let target = e.mem.alloc(8);
            e.register_double(target, |_, _| ret(1));
            e.mem.set_u32(table + 0x218, target);
            actor
        };
        stub(&mut e, REFR_GET_OWNER, 0x7770);
        stub(&mut e, REFR_IS_AN_OWNER, 0);
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 0);
        fn_00891e00(&mut e, actor, refr, 2, 0);
        assert_eq!(
            calls_to(&e, ACTOR_STEAL_ALARM),
            vec![vec![actor.addr(), refr.addr(), BASE_FORM, 2, 0, 0x7770]]
        );
        assert_eq!(
            calls_to(&e, EXTRA_SET_OWNERSHIP),
            vec![vec![0x5000, BASE_FORM]]
        );
        assert_eq!(call_count(&e, EXTRA_REMOVE_OWNERSHIP), 0);

        // An owner of the object just removes the ownership.
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        let table = e.mem.u32(actor.addr());
        let target = e.mem.alloc(8);
        e.register_double(target, |_, _| ret(1));
        e.mem.set_u32(table + 0x218, target);
        stub(&mut e, REFR_GET_OWNER, 0x7770);
        stub(&mut e, REFR_IS_AN_OWNER, 1);
        fn_00891e00(&mut e, actor, refr, 2, 0);
        assert_eq!(call_count(&e, ACTOR_STEAL_ALARM), 0);
        assert_eq!(calls_to(&e, EXTRA_REMOVE_OWNERSHIP), vec![vec![0x5000]]);
    }

    #[test]
    fn picking_up_ammunition_without_a_stack_uses_the_forms_count() {
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        stub_with(&mut e, FORM_TYPE, |_, a| {
            ret(if a[0] == BASE_FORM + 0x80 { 12 } else { 0x29 })
        });
        stub(&mut e, EXTRA_GET_COUNT, 0);
        fn_00891e00(&mut e, actor, refr, 3, 1);
        // The flag is cleared and the count comes from the form.
        assert_eq!(
            calls_to(&e, ACTOR_PICK_UP_WORKER),
            vec![vec![actor.addr(), refr.addr(), 12, 0, 0]]
        );
        // A stack of 5 keeps the caller's count.
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        stub(&mut e, FORM_TYPE, 0x29);
        stub(&mut e, EXTRA_GET_COUNT, 5);
        fn_00891e00(&mut e, actor, refr, 3, 1);
        assert_eq!(
            calls_to(&e, ACTOR_PICK_UP_WORKER),
            vec![vec![actor.addr(), refr.addr(), 3, 0, 0]]
        );
    }

    #[test]
    fn everyone_targeting_the_object_is_retargeted() {
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        // One list node holding another actor that targets the reference.
        let other = 0x7000_0001;
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, other);
        let node = e.mem.alloc(8);
        stub(&mut e, PROCESS_LISTS_FIND_ACTORS, node);
        stub(&mut e, LIST_ITEM_SLOT, slot);
        stub(&mut e, LIST_NEXT, 0);
        stub(&mut e, ACTOR_GET_CURRENT_PACKAGE_TARGET, refr.addr());
        fn_00891e00(&mut e, actor, refr, 1, 0);
        assert_eq!(
            calls_to(&e, ACTOR_SET_CURRENT_TARGET),
            vec![vec![other, actor.addr()]]
        );
        // The temporary list is cleared and deleted.
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![node]]);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![node, 1]]);
    }

    /// Replaces slot `offset` of `object`'s table by a double returning `value`.
    fn set_slot(e: &mut Engine, object: Ptr, offset: u32, value: Ret) {
        let table = e.mem.u32(object.addr());
        let target = e.mem.alloc(8);
        e.register_double(target, move |_, _| value);
        e.mem.set_u32(table + offset, target);
    }

    #[test]
    fn a_note_given_to_the_player_is_added_to_the_pip_boy() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        set_player(&mut e, actor);
        let extra = object(&mut e, 0x40, &[(0, ret(0))]);
        stub(&mut e, FORM_TYPE, 0x31);
        e.call(0x0089_2520, &args![actor, 0x6000u32, extra, 0u32]);
        assert_eq!(
            calls_to(&e, PLAYER_ADD_NOTE),
            vec![vec![actor.addr(), 0x6000, 1]]
        );
        // The extra data list is destroyed through its first virtual slot.
        assert_eq!(call_count(&e, slot_target(&e, extra, 0)), 1);
        assert_eq!(call_count(&e, ACTOR_ADD_ITEM_WORKER), 0);
    }

    #[test]
    fn items_are_added_through_the_worker_and_the_combat_controller_is_told() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        set_slot(&mut e, actor.cast(), 0x428, ret(0x3300));
        fn_00892520(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 7);
        assert_eq!(
            calls_to(&e, ACTOR_ADD_ITEM_WORKER),
            vec![vec![actor.addr(), 0x6000, 0, 7]]
        );
        assert_eq!(calls_to(&e, COMBAT_CONTROLLER_CALL), vec![vec![0x3300, 1]]);
    }

    #[test]
    fn a_weapon_given_to_an_npc_is_equipped() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        let entry = Ptr::<()>::new(0x6100);
        set_slot(&mut e, actor.cast(), 0x218, ret(1));
        set_slot(&mut e, actor.cast(), 0x3bc, ret(entry.addr()));
        stub(&mut e, FORM_TYPE, 0x28);
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        fn_00892520(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 0);
        let equip_args = vec![actor.addr(), 0xf0f0, 1, 0, 1, 0, 1];
        assert_eq!(
            calls_to(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![equip_args.clone()]
        );
        assert_eq!(calls_to(&e, ENTRY_DELETE), vec![vec![entry.addr(), 1]]);
        assert_eq!(call_count(&e, ACTOR_EQUIP_OBJECT), 0);

        // In menu mode with a forced update the equip is immediate.
        stub(&mut e, INTERFACE_IS_IN_MENU_MODE, 1);
        stub(&mut e, PROCESS_GET_FORCE_NEXT_UPDATE, 1);
        fn_00892520(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 0);
        assert_eq!(calls_to(&e, ACTOR_EQUIP_OBJECT), vec![equip_args]);
        assert_eq!(call_count(&e, ACTOR_QUEUE_EQUIP_OBJECT), 1);
    }

    #[test]
    fn a_reference_in_the_extra_list_gets_the_actor_and_its_targeters_move_on() {
        let mut e = engine();
        let (actor, refr) = pickup_setup(&mut e);
        let extra = Ptr::<()>::new(0x6200);
        stub(&mut e, EXTRA_GET_REFERENCE_POINTER, refr.addr());
        stub(&mut e, REFR_FORM_ID, 0x44);
        fn_00892520(&mut e, actor, Ptr::new(0x6000), extra, 0);
        assert_eq!(
            calls_to(&e, EXTRA_SET_ACTOR),
            vec![vec![0x5000, actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, refr, 0x48)),
            vec![vec![refr.addr(), 0x400]]
        );
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_FIND_ACTORS),
            vec![vec![PROCESS_LISTS, 0x44, actor.addr()]]
        );
    }

    #[test]
    fn the_players_ownership_is_removed_when_the_player_picks_up_the_item() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        set_player(&mut e, actor);
        stub(&mut e, EXTRA_GET_OWNER_FORM, BASE_FORM);
        fn_00892520(&mut e, actor, Ptr::new(0x6000), Ptr::new(0x6200), 0);
        assert_eq!(calls_to(&e, EXTRA_REMOVE_OWNERSHIP), vec![vec![0x6200]]);
    }

    #[test]
    fn ammunition_regeneration_requips_a_weapon_without_regeneration() {
        let mut e = engine();
        e.map(ZERO_DOUBLE, 8);
        let weapon_item = Ptr::<()>::new(0x6300);
        let actor = actor_with(
            &mut e,
            &[(0x218, ret(0)), (0x428, ret(0)), (0x3ec, ret(0))],
            Some(&[(0x14c, ret(0)), (0x148, ret(weapon_item.addr()))]),
        );
        set_player(&mut e, actor);
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, FORM_TYPE, 5);
        // The weapon's ammunition is the picked-up item.
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0x6000);
        stub(&mut e, ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, 1);
        stub(&mut e, ACTOR_IS_WEAPON_DRAWN, 1);
        stub_float(&mut e, WEAPON_GET_AMMO_REGEN_RATE, 0.0);
        fn_00892520(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 0);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), 0x3ec)),
            vec![vec![actor.addr(), 0xf0f0, 1, 1, 0]]
        );
        // With regeneration nothing is re-equipped.
        stub_float(&mut e, WEAPON_GET_AMMO_REGEN_RATE, 2.0);
        fn_00892520(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 0);
        assert_eq!(call_count(&e, slot_target(&e, actor.cast(), 0x3ec)), 1);
    }

    #[test]
    fn the_player_cannot_wear_clothes_during_an_animation_action() {
        let mut e = engine();
        e.map(MESSAGE_DURATION, 4);
        e.set_global(MESSAGE_DURATION, 2.0f32);
        let (actor, _) = pickup_setup(&mut e);
        set_player(&mut e, actor);
        stub(&mut e, EXTRA_GET_WORN, 1);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 3);
        stub(&mut e, MESSAGE_TEXT, 0x555);
        let result = e.call(
            0x0089_29a0,
            &args![actor, 0x6000u32, 0x6200u32, 1u32, 0u32, 0u32],
        );
        assert_eq!(result.u32(), 0);
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE),
            vec![vec![0x555, 0, MESSAGE_ICON_PATH, 0, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(
            call_count(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_ITEM)),
            0
        );
    }

    #[test]
    fn items_are_given_through_slot_17c_and_the_reference_is_reset() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        let result = object(&mut e, 0x100, &[(0x1d0, ret(0x77)), (0x48, ret(0))]);
        set_slot(
            &mut e,
            actor.cast(),
            ACTOR_SLOT_ADD_ITEM,
            ret(result.addr()),
        );
        let given = e
            .call(
                0x0089_29a0,
                &args![actor, 0x6000u32, 0u32, 4u32, 8u32, 9u32],
            )
            .ptr::<()>();
        assert_eq!(given, result);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_ITEM)),
            vec![vec![actor.addr(), 0x6000, 0, 4, 0, 1, 0, 8, 9, 1, 0]]
        );
        assert_eq!(calls_to(&e, RESET_SIM), vec![vec![0x77, 1]]);
        // The reference's base form is handed to the actor-form worker.
        assert_eq!(
            calls_to(&e, ACTOR_FORM_WORKER),
            vec![vec![actor.addr(), BASE_FORM, 0, 0]]
        );
    }

    #[test]
    fn items_for_a_cell_are_added_into_it_directly() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        set_slot(&mut e, actor.cast(), ACTOR_SLOT_ADD_ITEM, ret(0x1234));
        stub(&mut e, ADD_INTO_CELL_ENABLED, 1);
        stub(&mut e, CELL_GET_ADD_TARGET, 0);
        stub(&mut e, CELL_FIND_ADD_TARGET, 0x888);
        let given = fn_008929a0(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 4, 8, 9);
        assert_eq!(given, Ptr::NULL);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_ITEM)),
            vec![vec![actor.addr(), 0x6000, 0, 4, 0, 0, 0x888, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, CELL_FIND_ADD_TARGET),
            vec![vec![0x9000, actor.addr()]]
        );
    }

    #[test]
    fn a_weapon_given_to_an_npc_gets_its_ammunition() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        let result = object(&mut e, 0x100, &[(0x1d0, ret(0x77)), (0x48, ret(0))]);
        set_slot(
            &mut e,
            actor.cast(),
            ACTOR_SLOT_ADD_ITEM,
            ret(result.addr()),
        );
        stub(&mut e, FORM_TYPE, 0x28);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        stub(&mut e, WEAPON_GET_FORM_CLIP_ROUNDS, 3);
        stub(&mut e, AMMO_COUNT_FOR_CLIP, 5);
        fn_008929a0(&mut e, actor, Ptr::new(0x6000), Ptr::NULL, 1, 0, 0);
        assert_eq!(calls_to(&e, AMMO_COUNT_FOR_CLIP), vec![vec![0, 3]]);
        assert_eq!(calls_to(&e, EXTRA_SET_AMMO), vec![vec![0x5000, 0xa1, 5]]);
        assert_eq!(
            calls_to(&e, slot_target(&e, result, 0x48)),
            vec![vec![result.addr(), 0x800]]
        );
    }

    #[test]
    fn a_worn_weapons_animation_gets_its_time_range_adjusted() {
        let mut e = engine();
        e.map(FLOAT_MAX, 4);
        e.set_global(FLOAT_MAX, f32::MAX);
        let (actor, _) = pickup_setup(&mut e);
        let animation = Ptr::<()>::new(0x6400);
        set_slot(&mut e, actor.cast(), 0x1e4, ret(animation.addr()));
        set_slot(&mut e, actor.cast(), ACTOR_SLOT_ADD_ITEM, ret(0));
        stub(&mut e, EXTRA_GET_WORN, 1);
        stub(&mut e, FORM_TYPE, 0x28);
        stub(&mut e, ANIMATION_GET_SEQUENCE, 0x6500);
        stub(&mut e, ANIMATION_ZERO_GLOBAL_TRANSFORM, 0x6600);
        stub_float(&mut e, ANIMATION_FLOAT_D0, 2.0);
        stub_float(&mut e, SEQUENCE_GET_SCALED_TIME, 1.5);
        stub_float(&mut e, SEQUENCE_FLOAT_30, 0.25);
        stub_float(&mut e, SEQUENCE_FLOAT_48, 0.125);
        fn_008929a0(&mut e, actor, Ptr::new(0x6000), Ptr::new(0x6200), 1, 0, 0);
        assert_eq!(
            calls_to(&e, ANIMATION_APPLY_TIME_RANGE),
            vec![vec![
                0x6600,
                actor.addr(),
                1.5f32.to_bits(),
                2.375f32.to_bits(),
                0x6500
            ]]
        );
        // The sentinel -FLT_MAX leaves it alone.
        stub_float(&mut e, SEQUENCE_GET_SCALED_TIME, -f32::MAX);
        fn_008929a0(&mut e, actor, Ptr::new(0x6000), Ptr::new(0x6200), 1, 0, 0);
        assert_eq!(call_count(&e, ANIMATION_APPLY_TIME_RANGE), 1);
    }

    #[test]
    fn owned_goods_in_an_owned_cell_get_the_players_ownership_when_cheap_enough() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        set_slot(&mut e, actor.cast(), ACTOR_SLOT_ADD_ITEM, ret(0));
        let extra = Ptr::<()>::new(0x6200);
        // The cell has an owner; the item is worth 10, the threshold is 25.
        stub(&mut e, CELL_GET_OWNER, 0x7777);
        stub(&mut e, VALUE_FORM_GET_FORM_VALUE, 10);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 25.0);
        stub(&mut e, SETTING_GET_VALUE_POINTER, setting);
        fn_008929a0(&mut e, actor, Ptr::new(0x6000), extra, 1, 0, 0);
        assert_eq!(
            calls_to(&e, EXTRA_SET_OWNERSHIP),
            vec![vec![0x6200, BASE_FORM]]
        );
        // Worth more than the threshold: no ownership is set.
        stub(&mut e, VALUE_FORM_GET_FORM_VALUE, 100);
        fn_008929a0(&mut e, actor, Ptr::new(0x6000), extra, 1, 0, 0);
        assert_eq!(call_count(&e, EXTRA_SET_OWNERSHIP), 1);
    }

    #[test]
    fn dropping_items_at_a_target_repeats_for_the_count() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        let target = object(&mut e, 0x100, &[(0x1f4, ret(0)), (0x218, ret(0))]);
        // The actor's and the target's positions (z at +8).
        let actor_position = e.mem.alloc(16);
        e.mem.set_f32(actor_position + 8, 10.0);
        let target_position = e.mem.alloc(16);
        e.mem.set_f32(target_position + 8, 20.0);
        set_slot(&mut e, actor.cast(), 0x1f4, ret(actor_position));
        set_slot(&mut e, target, 0x1f4, ret(target_position));
        set_slot(&mut e, actor.cast(), ACTOR_SLOT_ADD_ITEM, ret(0));
        set_slot(&mut e, actor.cast(), 0x3bc, ret(0));
        let acquire = object(&mut e, 0x40, &[(0x44, ret(0))]);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(&mut e, GET_INVENTORY_ITEM, 0x7100);
        stub(&mut e, CHANGES_IS_EMPTY, 1);
        stub(&mut e, GET_CONTAINER_CHANGES, 0x2000);
        stub_float(&mut e, ACTOR_GET_EYE_LEVEL, 5.0);
        let done = e.call(
            0x0089_2e90,
            &args![actor, 0x6000u32, 0u32, target, 2u32, 77u32],
        );
        assert!(!done.bool());
        let adds = calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_ITEM));
        assert_eq!(adds.len(), 2);
        assert_eq!(
            adds[0],
            vec![actor.addr(), 0x6000, 0, 2, 0, 0, target.addr(), 0, 0, 1, 0]
        );
        assert_eq!(adds[1][3], 1);
        assert_eq!(
            calls_to(&e, GET_INVENTORY_ITEM),
            vec![vec![0x2000, 0x6000, 77]; 2]
        );
        assert_eq!(call_count(&e, EXTRA_REMOVE_CONTAINER_CHANGES), 1);
        // The idle: the item level is 1 because the target is above the eyes.
        assert_eq!(
            calls_to(&e, IDLE_MANAGER_SET_USED_ITEM_LEVEL),
            vec![vec![1], vec![0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, 0x44)),
            vec![vec![acquire.addr(), actor.addr(), 0, 2, 1, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, IDLE_MANAGER_SET_USED_ITEM_ACTIVATE),
            vec![vec![1], vec![0]]
        );
    }

    #[test]
    fn the_player_dropping_items_refreshes_the_inventory_instead() {
        let mut e = engine();
        let (actor, _) = pickup_setup(&mut e);
        set_player(&mut e, actor);
        stub(&mut e, GET_CONTAINER_CHANGES, 0);
        let target = Ptr::<()>::new(0x6700);
        assert!(!fn_00892e90(
            &mut e,
            actor,
            Ptr::new(0x6000),
            0,
            target,
            1,
            0
        ));
        assert_eq!(call_count(&e, INTERFACE_REFRESH_INVENTORY), 1);
        assert_eq!(call_count(&e, IDLE_MANAGER_SET_USED_ITEM), 0);
    }

    /// An actor with a process and the stubs `Actor::StartAttack` and
    /// `Actor::QueueAttack` need to get through a plain attack: the weapon is
    /// drawn, the process can attack, there is an animation with an attack
    /// sequence and every group is an attack group.
    fn attack_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr) {
        let animation = Ptr::<()>::new(e.mem.alloc(0x200));
        let acquire = object(
            e,
            0x40,
            &[
                (0x27c, ret(0)),
                (0x1b0, ret(0)),
                (0x44, ret(0)),
                (0x614, ret(0)),
                (0x22c, ret(0)),
                (0x28, ret(0)),
                (0xe4, float_ret(0.0)),
                (0xe8, ret(0)),
                (0x710, ret(0)),
                (0x30c, ret(0)),
                (0x1e0, float_ret(0.0)),
            ],
        );
        let position = e.mem.alloc(16);
        let actor = actor_with(
            e,
            &[
                (0x218, ret(0)),
                (0x22c, ret(0)),
                (0x1e4, ret(animation.addr())),
                (0x4b0, ret(0)),
                (0x1f4, ret(position)),
                (0x390, ret(0)),
                (0x2f4, ret(0)),
                (0x214, ret(0)),
                (0x234, ret(0)),
                (0x230, ret(0)),
                (0x428, ret(0)),
            ],
            Some(&[
                (0x148, ret(0)),
                (0x14c, ret(0)),
                (0x3f8, ret(1)),
                (0x3f4, ret(0)),
                (0x6dc, ret(0)),
                (0x368, ret(0)),
                (0x614, ret(0)),
                (0x454, ret(0)),
            ]),
        );
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(e, ACTOR_IS_WEAPON_DRAWN, 1);
        stub(e, ANIM_GROUP_IS_ATTACK_ACTION, 1);
        stub(e, ANIM_GROUP_COMPOSE, 0x55);
        stub(e, ANIMATION_PICK_BEST_ANIMATION, 0x55);
        stub(e, ANIM_GROUP_GET_TYPE, 0x55);
        stub(e, ANIMATION_GET_SEQUENCE, 0x6500);
        stub(e, ANIMATION_ZERO_GLOBAL_TRANSFORM, 0x6600);
        stub(e, TRANSFORM_U16, 0x1234);
        e.map(ANIM_GROUP_TABLE, 0x1000);
        e.map(ZERO_DOUBLE, 8);
        (actor, animation, acquire)
    }

    #[test]
    fn non_player_actors_do_not_queue_attacks_while_the_setting_is_set() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting + 4, 0);
        stub(&mut e, SETTING_VALUE_POINTER, setting);
        e.mem.set_u8(setting, 1);
        assert!(!actor_queue_attack(&mut e, actor, 0x30));
        assert_eq!(call_count(&e, ACTOR_IS_WEAPON_DRAWN), 0);
    }

    #[test]
    fn a_queued_attack_group_is_remembered_for_npcs() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        let setting = e.mem.alloc(4);
        stub(&mut e, SETTING_VALUE_POINTER, setting);
        assert!(e.call(0x0089_35f0, &args![actor, 0x30u32]).bool());
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0x30);
    }

    #[test]
    fn a_queued_group_that_is_not_an_attack_is_reported() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        let setting = e.mem.alloc(4);
        stub(&mut e, SETTING_VALUE_POINTER, setting);
        stub(&mut e, ANIM_GROUP_IS_ATTACK_ACTION, 0);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0x31);
        e.mem.set_u32(ANIM_GROUP_TABLE + 0x30 * 0x24, 0x7001);
        e.mem.set_u32(ANIM_GROUP_TABLE + 0x31 * 0x24, 0x7002);
        stub(&mut e, REFR_GET_NAME, 0x7003);
        assert!(!actor_queue_attack(&mut e, actor, 0x30));
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![
                QUEUE_NON_ATTACK_FORMAT,
                0x7003,
                HAND_TO_HAND_NAME,
                0x7001,
                0x7002
            ]]
        );
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0);
    }

    #[test]
    fn the_player_starts_an_attack_directly() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        set_player(&mut e, actor);
        vtable_at(&mut e, actor.addr() + 0xa4, &[(0x08, ret(0))]);
        let setting_value = e.mem.alloc(4);
        stub(&mut e, SETTING_GET_VALUE_POINTER, setting_value);
        assert!(actor_queue_attack(&mut e, actor, 0x30));
        // `Actor::StartAttack` ran: it played the group through 008b28c0.
        assert_eq!(call_count(&e, ACTOR_PLAY_GROUP_WORKER), 1);
    }

    #[test]
    fn a_jamming_weapon_queues_its_jam_attack() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        let weapon_item = 0x6800;
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x148, ret(weapon_item));
        set_slot(&mut e, process, 0x14c, ret(0x6900));
        let setting = e.mem.alloc(4);
        stub(&mut e, SETTING_VALUE_POINTER, setting);
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        stub(&mut e, LIST_NEXT, 9);
        stub_float(&mut e, GET_WEAPON_CONDITION_JAM_MULT, 1.0);
        stub(&mut e, CHANCE_ROLL, 1);
        stub(&mut e, WEAPON_GET_ATTACK_ANIM, 0x10);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x27);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0x27);
        assert!(!actor_queue_attack(&mut e, actor, 0x30));
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0x27);
        assert_eq!(calls_to(&e, CHANCE_ROLL), vec![vec![1.0f32.to_bits()]]);
    }

    #[test]
    fn a_weapon_with_ammunition_but_no_ammunition_item_cannot_be_queued() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x148, ret(0x6800));
        let setting = e.mem.alloc(4);
        stub(&mut e, SETTING_VALUE_POINTER, setting);
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        assert!(!actor_queue_attack(&mut e, actor, 0x30));
        assert_eq!(call_count(&e, ACTOR_IS_WEAPON_DRAWN), 0);
    }

    #[test]
    fn starting_a_plain_attack_plays_the_group_in_slot_4() {
        let mut e = engine();
        let (actor, animation, _) = attack_setup(&mut e);
        assert!(e.call(0x0089_3a40, &args![actor, 0x30u32]).bool());
        assert_eq!(
            calls_to(&e, ANIMATION_SET_SPEED),
            vec![vec![animation.addr(), 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_PLAY_GROUP_WORKER),
            vec![vec![actor.addr(), 0x30, animation.addr()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION),
            vec![vec![actor.addr(), 2, 0x6500]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), 0x4b0)),
            vec![vec![actor.addr(), 0x1234, 1]]
        );
    }

    #[test]
    fn starting_an_attack_with_no_ammunition_fails() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x148, ret(0x6800));
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        assert!(!actor_start_attack(&mut e, actor, 0x30));
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x3f4)),
            vec![vec![process.addr(), 0]]
        );
        assert_eq!(call_count(&e, ANIMATION_SET_SPEED), 0);
    }

    #[test]
    fn the_player_hears_the_dry_fire_sound() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        set_player(&mut e, actor);
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x148, ret(0x6800));
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        stub(&mut e, WEAPON_SOUND_SET, 0x700);
        stub(&mut e, DRY_FIRE_ALLOWED, 1);
        stub(&mut e, AUDIO_INSTANCE, 0x4400);
        stub(&mut e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, 0x4500);
        stub(&mut e, SOUND_SET_FILE_NAME, 0x4600);
        assert!(!actor_start_attack(&mut e, actor, 0x30));
        let lookups = calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(lookups.len(), 1);
        // audio, out handle, file name, flags (0x200101 without the weapon
        // test), sound set
        assert_eq!(lookups[0][0], 0x4400);
        assert_eq!(lookups[0][2..], [0x4600, 0x20_0101, 0x700]);
        assert_eq!(call_count(&e, SOUND_HANDLE_PLAY), 1);
        assert_eq!(call_count(&e, EXTRA_SET_SOUND), 1);
        // Both temporary sound handles are destroyed again.
        assert_eq!(call_count(&e, SOUND_HANDLE_DESTROY), 2);
        // The dry-fire is not allowed: the attack just fails.
        stub(&mut e, DRY_FIRE_ALLOWED, 0);
        assert!(!actor_start_attack(&mut e, actor, 0x30));
        assert_eq!(call_count(&e, SOUND_HANDLE_PLAY), 1);
    }

    #[test]
    fn a_power_attack_group_is_played_through_the_worker() {
        let mut e = engine();
        let (actor, animation, _) = attack_setup(&mut e);
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x148, ret(0x6800));
        set_slot(&mut e, process, 0x14c, ret(0x6900));
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        stub(&mut e, LIST_NEXT, 9);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 210);
        assert!(!actor_start_attack(&mut e, actor, 210));
        assert_eq!(
            calls_to(&e, ACTOR_PLAY_GROUP_WORKER),
            vec![vec![actor.addr(), 210, animation.addr()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION),
            vec![vec![actor.addr(), 9, 0x6500]]
        );
    }

    #[test]
    fn a_group_that_is_not_an_attack_is_only_reported() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        stub(&mut e, ANIM_GROUP_IS_ATTACK_ACTION, 0);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0x31);
        e.mem.set_u32(ANIM_GROUP_TABLE + 0x30 * 0x24, 0x7001);
        e.mem.set_u32(ANIM_GROUP_TABLE + 0x31 * 0x24, 0x7002);
        stub(&mut e, REFR_GET_NAME, 0x7003);
        assert!(!actor_start_attack(&mut e, actor, 0x30));
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![
                START_NON_ATTACK_FORMAT,
                0x7003,
                HAND_TO_HAND_NAME,
                0x7001,
                0x7002
            ]]
        );
    }

    #[test]
    fn a_strong_actor_may_swap_the_attack_for_group_0x38() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        e.map(RANDOM_ROLL_MAX, 4);
        e.set_global(RANDOM_ROLL_MAX, 100.0f32);
        set_slot(&mut e, actor.cast(), 0x218, ret(1));
        vtable_at(&mut e, actor.addr() + 0xa4, &[(0x08, ret(50))]);
        // Settings: threshold A 10 (< 50) with factor 0.5, B never.
        let values = [
            (SETTING_THRESHOLD_A, 10.0f32),
            (SETTING_FACTOR_A, 0.5),
            (SETTING_THRESHOLD_B, 1000.0),
            (SETTING_FACTOR_B, 0.1),
        ];
        stub_with(&mut e, SETTING_GET_VALUE_POINTER, move |e, a| {
            let value = values.iter().find(|(object, _)| *object == a[0]).unwrap().1;
            let slot = e.mem.alloc(4);
            e.mem.set_f32(slot, value);
            ret(slot)
        });
        // The roll (10) is below the first chance (25): group 0x38.
        stub_float(&mut e, RANDOM_FLOAT, 10.0);
        e.call(0x0089_3a40, &args![actor, 0x30u32]);
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE)[0][2], 0x38);
        // A roll above both chances keeps the group.
        let mut e2 = engine();
        let (actor2, _, _) = attack_setup(&mut e2);
        e2.map(RANDOM_ROLL_MAX, 4);
        e2.set_global(RANDOM_ROLL_MAX, 100.0f32);
        set_slot(&mut e2, actor2.cast(), 0x218, ret(1));
        vtable_at(&mut e2, actor2.addr() + 0xa4, &[(0x08, ret(50))]);
        stub_with(&mut e2, SETTING_GET_VALUE_POINTER, move |e, a| {
            let value = values.iter().find(|(object, _)| *object == a[0]).unwrap().1;
            let slot = e.mem.alloc(4);
            e.mem.set_f32(slot, value);
            ret(slot)
        });
        stub_float(&mut e2, RANDOM_FLOAT, 99.0);
        // 25 >= 99 is false and 99 < 25 + 0 is false: unchanged.
        e2.call(0x0089_3a40, &args![actor2, 0x30u32]);
        assert_eq!(calls_to(&e2, ANIM_GROUP_COMPOSE)[0][2], 0x30);
    }

    #[test]
    fn animation_actions_two_to_six_are_the_blocking_range() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        for (action, expected) in [(-1i32, false), (1, false), (2, true), (6, true), (7, false)] {
            stub(&mut e, ACTOR_GET_ANIM_ACTION, action as u32);
            assert_eq!(e.call(0x0089_4900, &args![actor]).bool(), expected);
        }
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 7);
        assert!(e.call(0x0089_4d60, &args![actor]).bool());
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 6);
        assert!(!actor_get_blocked(&mut e, actor));
    }

    /// An attack setup plus the stubs the blocking and idle animation
    /// functions use.
    fn block_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr) {
        let (actor, animation, acquire) = attack_setup(e);
        set_slot(e, actor.cast(), 0x214, ret(0));
        set_slot(e, actor.cast(), 0x428, ret(0x3300));
        stub(e, ACTOR_PROCESS_OBJECT, 0x3400);
        stub(e, ACTOR_GET_ANIMATION, animation.addr());
        stub(e, SEQUENCE_GET_GENERIC_LOCATION, 1);
        (actor, animation, acquire)
    }

    #[test]
    fn a_blocking_actor_starts_the_block_group() {
        let mut e = engine();
        let (actor, animation, _) = block_setup(&mut e);
        e.mem.set_u8(actor.addr() + 0xf0, 1);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 0xffff_ffff);
        stub(&mut e, ANIM_GROUP_GET_NUMBER, 0x10);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x77);
        fn_00894940(&mut e, actor);
        assert_eq!(
            calls_to(&e, ANIMATION_PLAY_GROUP),
            vec![vec![animation.addr(), 0x77, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION),
            vec![vec![actor.addr(), 7, 0x6500]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), 0x4b0)),
            vec![vec![actor.addr(), 0x77, 1]]
        );
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_SET_BLOCKED),
            vec![vec![0x3400, 1]]
        );
    }

    #[test]
    fn a_block_group_that_is_not_generic_only_notifies_the_combat_controller() {
        let mut e = engine();
        let (actor, _, _) = block_setup(&mut e);
        stub(&mut e, SEQUENCE_GET_GENERIC_LOCATION, 0);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 7);
        fn_00894940(&mut e, actor);
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_SET_BLOCKED),
            vec![vec![0x3400, 1]]
        );
        assert_eq!(call_count(&e, ANIMATION_PLAY_GROUP), 0);
    }

    #[test]
    fn a_stopped_block_clears_the_group_with_a_blend_time() {
        let mut e = engine();
        let (actor, animation, _) = block_setup(&mut e);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 0.25);
        e.map(THIRTY, 8);
        e.set_global(THIRTY, 30.0f64);
        stub(&mut e, SETTING_GET_VALUE_POINTER, setting);
        stub(&mut e, ANIM_GROUP_GET_NUMBER, 0xaa);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 7);
        fn_00894940(&mut e, actor);
        assert_eq!(
            calls_to(&e, ANIMATION_CLEAR_GROUP),
            vec![vec![animation.addr(), 2, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION),
            vec![vec![actor.addr(), 0xffff_ffff, 0]]
        );
        // A byte from the transform gives the blend time in 30ths.
        clear_log(&mut e);
        stub(&mut e, TRANSFORM_BYTE, 15);
        fn_00894940(&mut e, actor);
        assert_eq!(
            calls_to(&e, ANIMATION_CLEAR_GROUP),
            vec![vec![animation.addr(), 2, 0.5f32.to_bits()]]
        );
        // Still blocking: nothing is cleared.
        clear_log(&mut e);
        e.mem.set_u8(actor.addr() + 0xf0, 1);
        fn_00894940(&mut e, actor);
        assert_eq!(call_count(&e, ANIMATION_CLEAR_GROUP), 0);
    }

    #[test]
    fn setting_the_block_applies_it() {
        let mut e = engine();
        let (actor, _, acquire) = block_setup(&mut e);
        assert!(e.call(0x0089_4cc0, &args![actor, 1u32]).bool());
        assert_eq!(e.mem.u8(actor.addr() + 0xf0), 1);
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, 0x614)),
            vec![vec![acquire.addr(), 0x8000]]
        );
        // Without an animation the combat controller is told and it fails.
        set_slot(&mut e, actor.cast(), 0x1e4, ret(0));
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 7);
        assert!(!actor_set_block(&mut e, actor, 1));
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_SET_BLOCKED),
            vec![vec![0x3400, 1]]
        );
    }

    #[test]
    fn idle_groups_are_started_by_type() {
        let mut e = engine();
        let (actor, animation, _) = block_setup(&mut e);
        set_group(&mut e, 0xab, 4, 0);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0xab);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x66);
        e.call(0x0089_4d90, &args![actor, 0u32]);
        assert_eq!(
            calls_to(&e, ANIMATION_PLAY_GROUP),
            vec![vec![animation.addr(), 0x66, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        // The table gives the slot (4) whose sequence becomes the action's.
        assert_eq!(
            calls_to(&e, ANIMATION_GET_SEQUENCE),
            vec![vec![animation.addr(), 4]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION),
            vec![vec![actor.addr(), 7, 0x6500]]
        );
        // Busy actors do nothing.
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 10);
        fn_00894d90(&mut e, actor, 0);
        assert_eq!(call_count(&e, ANIMATION_PLAY_GROUP), 1);
    }

    #[test]
    fn the_second_idle_clears_slot_4_unless_the_group_matches() {
        let mut e = engine();
        let (actor, animation, _) = block_setup(&mut e);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0xac);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x66);
        e.call(0x0089_4e90, &args![actor]);
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION),
            vec![vec![actor.addr(), 8, 0x6500]]
        );
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0x10);
        set_player(&mut e, actor);
        stub(&mut e, PLAYER_GET_ANIMATION, 0x6a00);
        fn_00894e90(&mut e, actor);
        assert_eq!(
            calls_to(&e, ANIMATION_CLEAR_GROUP),
            vec![
                vec![animation.addr(), 4, 0.0f32.to_bits()],
                vec![0x6a00, 4, 0.0f32.to_bits()]
            ]
        );
    }

    #[test]
    fn walking_directions_pick_a_movement_group() {
        let mut e = engine();
        let (actor, animation, _) = block_setup(&mut e);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x0b02);
        stub_float(&mut e, ACTOR_GET_WALK_SPEED, 150.0);
        stub(&mut e, ANIMATION_MOVEMENT_SPEED, 100);
        // Direction bit 2 asks for group 0xc, which the actor maps to 0xb02.
        let asked = e.call(0x0089_4f90, &args![actor, 2u32]).u32();
        assert_eq!(asked, 0xff);
        // 0xb02 is outside 0xb..=0xe (it is 0xb02): the group is refused.
        assert_eq!(call_count(&e, ANIMATION_PLAY_GROUP), 0);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x000c);
        stub_with(&mut e, ANIMATION_MOVEMENT_SPEED, |_, a| {
            ret(if a[1] == 3 { 100 } else { 7 })
        });
        assert_eq!(fn_00894f90(&mut e, actor, 2), 0xc);
        assert_eq!(
            calls_to(&e, ANIMATION_PLAY_GROUP),
            vec![vec![animation.addr(), 0xc, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        // The speed multiplier is walk speed over the group's speed.
        assert_eq!(e.mem.f32(animation.addr() + 0x10c), 1.5);
        // No direction bits: asks for 0xff and gets nothing.
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0xff);
        assert_eq!(fn_00894f90(&mut e, actor, 0), 0xff);
    }

    #[test]
    fn small_accessors() {
        let mut e = engine();
        let object = Ptr::<()>::new(e.mem.alloc(0x800));
        fn_008950f0(&mut e, object, 2.5);
        assert_eq!(e.mem.f32(object.addr() + 0x10c), 2.5);
        assert_eq!(e.call(0x0089_50f0, &args![object, 1.0f32]).u32(), 0);

        e.mem.set_u32(object.addr() + 0x12c, 4);
        assert!(e.call(0x0089_7890, &args![object]).bool());
        e.mem.set_u32(object.addr() + 0x12c, 3);
        assert!(!fn_00897890(&mut e, object));

        e.map(0x011c_6000, 0x1000);
        e.set_global(0x011c_6230, 11u32);
        e.set_global(0x011c_6234, 22u32);
        assert_eq!(e.call(0x0089_78b0, &args![]).u32(), 11);
        assert_eq!(e.call(0x0089_78c0, &args![]).u32(), 22);

        stub(&mut e, 0x0044_8a60, 1);
        assert!(e.call(0x0089_78d0, &args![object]).bool());
        assert_eq!(calls_to(&e, 0x0044_8a60), vec![vec![object.addr(), 0x100]]);

        e.call(0x0089_78f0, &args![object, 2u32, 9u32]);
        assert_eq!(e.mem.u8(object.addr() + 2 * 0x30 + 0x1bd), 9);

        e.mem.set_u32(object.addr() + 0x6a0, 0xabcd);
        assert_eq!(e.call(0x0089_7b30, &args![object]).u32(), 0xabcd);
    }

    #[test]
    fn package_setters() {
        let mut e = engine();
        let package = Ptr::<()>::new(e.mem.alloc(0x100));
        fn_00898200(&mut e, package, 1);
        assert_eq!(e.mem.u32(package.addr() + 0x1c), 0x0200_0000);
        e.mem.set_u32(package.addr() + 0x1c, 0xffff_ffff);
        e.call(0x0089_8200, &args![package, 0u32]);
        assert_eq!(e.mem.u32(package.addr() + 0x1c), 0xfdff_ffff);

        e.mem.set_f32(package.addr() + 0x90, 1.5);
        e.call(0x0089_8250, &args![package, 2.25f32]);
        assert_eq!(e.mem.f32(package.addr() + 0x90), 3.75);

        e.call(0x0089_8280, &args![package, 5u32]);
        assert_eq!(e.mem.u8(package.addr() + 0x81), 5);
        e.call(0x0089_82a0, &args![package, 6u32]);
        assert_eq!(e.mem.u8(package.addr() + 0xa8), 6);
    }

    #[test]
    fn the_anim_group_is_composed_from_the_actor_flags_and_the_weapon() {
        let mut e = engine();
        let (actor, animation, _) = attack_setup(&mut e);
        // No animation at all.
        set_slot(&mut e, actor.cast(), 0x1e4, ret(0));
        assert_eq!(
            actor_get_anim_group(&mut e, actor, 5, 0, 0, Ptr::NULL),
            0xff
        );
        set_slot(&mut e, actor.cast(), 0x1e4, ret(animation.addr()));

        // Flags 0x800 give selector 2; in combat the weapon decides the second
        // selector (no weapon: 1).
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x800);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        stub(&mut e, ANIM_GROUP_COMPOSE, 0x1234);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x4321);
        assert_eq!(
            e.call(0x0089_7910, &args![actor, 5u32, 0u32, 0u32, 0u32])
                .u16(),
            0x4321
        );
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE), vec![vec![2, 1, 5, 0]]);
        assert_eq!(
            calls_to(&e, ANIMATION_PICK_BEST_ANIMATION),
            vec![vec![animation.addr(), 0x1234, 0]]
        );

        // A given weapon of type 3 takes its selector from the table; the
        // special state sets the last flag.
        clear_log(&mut e);
        e.map(WEAPON_TYPE_ANIM_TABLE, 0x40);
        e.mem.set_u32(WEAPON_TYPE_ANIM_TABLE + 12, 0x42);
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_TYPE, 3);
        stub(&mut e, ACTOR_TEST_8BA3E0, 1);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x2000);
        actor_get_anim_group(&mut e, actor, 5, 0x6800, 0, Ptr::NULL);
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE), vec![vec![3, 0x42, 5, 1]]);

        // `0xffffffff` means no weapon; flags 0x400 give selector 1.
        clear_log(&mut e);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x400);
        actor_get_anim_group(&mut e, actor, 5, 0xffff_ffff, 0, Ptr::NULL);
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE), vec![vec![1, 1, 5, 1]]);

        // Not in combat and nothing special: the second selector stays 0.
        clear_log(&mut e);
        stub(&mut e, ACTOR_IN_COMBAT, 0);
        actor_get_anim_group(&mut e, actor, 5, 0, 0, Ptr::NULL);
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE), vec![vec![1, 0, 5, 1]]);
    }

    #[test]
    fn the_duration_of_a_group_comes_from_its_animation_group() {
        let mut e = engine();
        let (actor, animation, _) = attack_setup(&mut e);
        stub(&mut e, ANIMATION_GET_TES_ANIM_GROUP, 0x8000);
        stub_float(&mut e, ANIM_GROUP_GET_TIME, 3.5);
        assert_eq!(e.call(0x0089_7b50, &args![actor, 9u32]).f32(), 3.5);
        assert_eq!(
            calls_to(&e, ANIMATION_GET_TES_ANIM_GROUP),
            vec![vec![animation.addr(), 0x55]]
        );
        assert_eq!(calls_to(&e, ANIM_GROUP_GET_TIME), vec![vec![0x8000, 1]]);
        // No animation: 0.0.
        set_slot(&mut e, actor.cast(), 0x1e4, ret(0));
        assert_eq!(actor_get_anim_group_duration(&mut e, actor, 9), 0.0);
    }

    #[test]
    fn a_package_of_type_0xe_targets_the_actor_itself() {
        let mut e = engine();
        let (actor, _, acquire) = attack_setup(&mut e);
        stub(&mut e, PACKAGE_CREATE, 0x7100);
        stub(&mut e, PACKAGE_GET_TARGET, 0x7200);
        stub(&mut e, ACTOR_BASE_FORM, 0xb000);
        stub(&mut e, PACKAGE_LOCATION_CONSTRUCT, 0x7500);
        stub(&mut e, PACKAGE_TARGET_CONSTRUCT, 0x7400);
        stub(&mut e, FORM_FIELD_14, 77);
        set_slot(&mut e, acquire, 0x22c, ret(0x7300));
        e.call(0x0089_7bd0, &args![actor, 0u32]);
        assert_eq!(calls_to(&e, PACKAGE_CREATE), vec![vec![0xe]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_PACK_TYPE), vec![vec![0x7100, 0xe]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_FIELD_18), vec![vec![0x7100, 0x29]]);
        assert_eq!(
            calls_to(&e, PACKAGE_TARGET_SET_REFERENCE),
            vec![vec![0x7200, actor.addr()]]
        );
        assert_eq!(calls_to(&e, PACKAGE_COPY_FROM), vec![vec![0x7100, 0x7300]]);
        assert_eq!(calls_to(&e, FORM_FIELD_14), vec![vec![0xb090]]);
        assert_eq!(calls_to(&e, TARGET_SET_COUNT), vec![vec![0x7200, 77]]);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_PACKAGE)),
            vec![vec![actor.addr(), 0x7100, 1, 1]]
        );
        // The location the package got is released again.
        assert_eq!(call_count(&e, PACKAGE_LOCATION_DESTROY), 1);
        assert_eq!(call_count(&e, PACKAGE_TARGET_DESTROY), 1);
    }

    #[test]
    fn avoiding_and_surfacing_packages_are_put_on_the_actor() {
        let mut e = engine();
        let (actor, _, acquire) = attack_setup(&mut e);
        stub(&mut e, PACKAGE_CREATE, 0x7100);
        stub(&mut e, PACKAGE_GET_TARGET, 0x7200);
        set_slot(&mut e, acquire, 0x22c, ret(0x7300));
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x25c, ret(0));
        let target = Ptr::<()>::new(0x7400);
        e.call(0x0089_82c0, &args![actor, target, 2.5f32]);
        assert_eq!(calls_to(&e, ACTOR_END_MOVEMENT), vec![vec![actor.addr()]]);
        assert_eq!(calls_to(&e, PACKAGE_CREATE), vec![vec![0x1f]]);
        assert_eq!(calls_to(&e, REFR_SET_TARGETED), vec![vec![0x7400, 1]]);
        assert_eq!(
            calls_to(&e, PACKAGE_TARGET_SET_REFERENCE),
            vec![vec![0x7200, 0x7400]]
        );
        assert_eq!(
            calls_to(&e, PACKAGE_CALCULATE_PROCEDURE_TYPE),
            vec![vec![0x7100, 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x25c)),
            vec![vec![process.addr(), 2.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_PACKAGE)),
            vec![vec![actor.addr(), 0x7100, 1, 1]]
        );

        clear_log(&mut e);
        actor_initiate_surface_package(&mut e, actor);
        assert_eq!(
            calls_to(&e, ACTOR_FORCE_STOP_MOVING),
            vec![vec![actor.addr()]]
        );
        assert_eq!(calls_to(&e, PACKAGE_CREATE), vec![vec![0x1d]]);
        assert_eq!(
            calls_to(&e, PACKAGE_SET_PACK_TYPE),
            vec![vec![0x7100, 0x1d]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_PACKAGE)),
            vec![vec![actor.addr(), 0x7100, 1, 1]]
        );
    }

    /// A flee package block big enough for the setters.
    fn flee_package(e: &mut Engine) -> u32 {
        let flee = e.mem.alloc(0x200);
        stub(e, FLEE_PACKAGE_CONSTRUCT, flee);
        flee
    }

    #[test]
    fn fleeing_makes_a_flee_package_with_a_location_reference() {
        let mut e = engine();
        let (actor, _, acquire) = attack_setup(&mut e);
        let flee = flee_package(&mut e);
        stub(&mut e, PACKAGE_TARGET_CONSTRUCT, 0x7400);
        stub(&mut e, PACKAGE_LOCATION_CONSTRUCT, 0x7500);
        stub(&mut e, FTOL, 5);
        stub_float(&mut e, PACKAGE_FLOAT_90, 4.0);
        e.map(ZERO_DOUBLE, 8);
        e.map(MINUS_ONE_DOUBLE, 8);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        let avoided = Ptr::<()>::new(0x7600);
        let location_ref = Ptr::<()>::new(0x7700);
        e.call(
            0x0089_7de0,
            &args![
                actor,
                avoided,
                0u32,
                3u32,
                1u32,
                0u32,
                location_ref,
                7.5f32,
                12.75f32
            ],
        );
        assert_eq!(
            calls_to(&e, FLEE_PACKAGE_CONSTRUCT)[0][1..],
            [avoided.addr(), 0, 0]
        );
        assert_eq!(
            calls_to(&e, FLEE_PACKAGE_ADD_AVOIDED_REF),
            vec![vec![flee, avoided.addr()]]
        );
        assert_eq!(calls_to(&e, PACKAGE_SET_TARGET), vec![vec![flee, 0x7400]]);
        assert_eq!(
            calls_to(&e, PACKAGE_TARGET_SET_REFERENCE),
            vec![vec![0x7400, avoided.addr()]]
        );
        assert_eq!(calls_to(&e, TARGET_SET_COUNT), vec![vec![0x7400, 5]]);
        // The radius is truncated: 12.75 becomes 12.
        assert_eq!(
            calls_to(&e, PACKAGE_LOCATION_SET_RADIUS),
            vec![vec![0x7500, 12]]
        );
        assert_eq!(
            calls_to(&e, PACKAGE_LOCATION_SET_REFERENCE),
            vec![vec![0x7500, location_ref.addr()]]
        );
        assert_eq!(calls_to(&e, PACKAGE_SET_LOCATION), vec![vec![flee, 0x7500]]);
        assert_eq!(call_count(&e, FLEE_PACKAGE_FIND_TELEPORT_DOOR), 0);
        // The package's own setters ran: +0xa8 = arg_3, +0x81 = arg_4, +0x90
        // minus its own value, and bit 0x2000000 cleared.
        assert_eq!(e.mem.u8(flee + 0xa8), 3);
        assert_eq!(e.mem.u8(flee + 0x81), 1);
        assert_eq!(e.mem.f32(flee + 0x90), -4.0);
        assert_eq!(calls_to(&e, PACKAGE_CALL_67A720), vec![vec![flee, 0]]);
        // arg_2 is 0: the process is told, then the package is added.
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, PROCESS_SLOT_710)),
            vec![vec![acquire.addr(), actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_PACKAGE)),
            vec![vec![actor.addr(), flee, 0, 1]]
        );
    }

    #[test]
    fn fleeing_reuses_the_current_flee_package_and_may_look_for_a_teleport_door() {
        let mut e = engine();
        let (actor, _, acquire) = attack_setup(&mut e);
        let flee = e.mem.alloc(0x200);
        stub(&mut e, GET_CURRENT_PACKAGE, flee);
        stub(&mut e, PACKAGE_TYPE, 0x16);
        stub(&mut e, PACKAGE_GET_TARGET, 0x7400);
        stub(&mut e, PACKAGE_LOCATION_CONSTRUCT, 0x7500);
        stub(&mut e, PACKAGE_FIELD_A4, 0);
        stub_float(&mut e, PACKAGE_FLOAT_90, 1.0);
        e.map(ZERO_DOUBLE, 8);
        e.map(MINUS_ONE_DOUBLE, 8);
        e.map(TWENTY, 4);
        e.set_global(TWENTY, 20.0f32);
        set_slot(&mut e, acquire, PROCESS_SLOT_E4, float_ret(-1.0));
        let avoided = Ptr::<()>::new(0x7600);
        // Negative counts and radii are not applied.
        e.call(
            0x0089_7de0,
            &args![actor, avoided, 1u32, 0u32, 0u32, 0u32, 0u32, -1.0f32, -1.0f32],
        );
        assert_eq!(call_count(&e, FLEE_PACKAGE_CONSTRUCT), 0);
        assert_eq!(call_count(&e, TARGET_SET_COUNT), 0);
        assert_eq!(call_count(&e, PACKAGE_LOCATION_SET_RADIUS), 0);
        // No reference and no cell: the teleport door search runs, after the
        // process's essential timer (-1.0 <= 0) is set to 20.
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, PROCESS_SLOT_E8)),
            vec![vec![acquire.addr(), 20.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, FLEE_PACKAGE_FIND_TELEPORT_DOOR),
            vec![vec![flee, actor.addr(), avoided.addr()]]
        );
        // arg_2 is 1: the process is not told.
        assert_eq!(
            call_count(&e, slot_target(&e, acquire, PROCESS_SLOT_710)),
            0
        );

        // A fleeing-excluded actor does nothing.
        clear_log(&mut e);
        stub(&mut e, EXTRA_TYPE_IS_5, 1);
        fn_00897de0(
            &mut e,
            actor,
            avoided,
            1,
            0,
            0,
            Ptr::NULL,
            Ptr::NULL,
            0.0,
            0.0,
        );
        assert_eq!(call_count(&e, FLEE_PACKAGE_ADD_AVOIDED_REF), 0);
    }

    /// An actor and a conversation partner for `Actor::ShouldTalkTo`, with a
    /// process object whose slots give "may talk".
    fn talk_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr) {
        e.map(ZERO_DOUBLE, 8);
        e.map(NINETY, 8);
        e.set_global(NINETY, 90.0f64);
        let process_object = object(
            e,
            0x40,
            &[(0x30c, ret(0)), (0x22c, ret(0)), (0x1e0, float_ret(0.0))],
        );
        stub(e, GET_SAVED_ACQUIRE_OBJECT, process_object.addr());
        let position = e.mem.alloc(16);
        let actor = actor_with(
            e,
            &[(0x234, ret(0)), (0x214, ret(0)), (0x1f4, ret(position))],
            None,
        );
        let other_position = e.mem.alloc(16);
        let other = object(e, 0x40, &[(0x1f4, ret(other_position))]);
        (actor, other, process_object)
    }

    #[test]
    fn talking_is_refused_in_the_cases_the_code_lists() {
        let mut e = engine();
        let (actor, other, process_object) = talk_setup(&mut e);
        assert!(e.call(0x0089_3190, &args![actor, other]).bool());

        // The process's own refusal (slot 0x30c) when nothing else forbids.
        set_slot(&mut e, process_object, 0x30c, ret(1));
        assert!(!actor_should_talk_to(&mut e, actor, other));
        // ...which a forbidden state (here the object test) overrides.
        stub(&mut e, EXTRA_OBJECTS_TEST, 1);
        assert!(actor_should_talk_to(&mut e, actor, other));
        stub(&mut e, EXTRA_OBJECTS_TEST, 0);
        set_slot(&mut e, process_object, 0x30c, ret(0));

        // A positive timer value (slot 0x1e0) refuses.
        set_slot(&mut e, process_object, 0x1e0, float_ret(2.0));
        assert!(!actor_should_talk_to(&mut e, actor, other));
        set_slot(&mut e, process_object, 0x1e0, float_ret(0.0));

        // The actor state must be 0 or 4.
        set_slot(&mut e, actor.cast(), 0x214, ret(3));
        assert!(!actor_should_talk_to(&mut e, actor, other));
        set_slot(&mut e, actor.cast(), 0x214, ret(4));
        assert!(actor_should_talk_to(&mut e, actor, other));
        set_slot(&mut e, actor.cast(), 0x214, ret(0));

        // An interrupt package refuses.
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, 0x8000);
        stub(&mut e, PACKAGE_IS_INTERRUPT_PACKAGE, 1);
        assert!(!actor_should_talk_to(&mut e, actor, other));
        stub(&mut e, PACKAGE_IS_INTERRUPT_PACKAGE, 0);
        assert!(actor_should_talk_to(&mut e, actor, other));

        // The process object's slot 0x22c and skipping the fall-out behaviour.
        set_slot(&mut e, process_object, 0x22c, ret(1));
        stub(&mut e, ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR, 1);
        assert!(!actor_should_talk_to(&mut e, actor, other));

        // No process object at all.
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        assert!(!actor_should_talk_to(&mut e, actor, other));
    }

    #[test]
    fn in_an_interior_a_large_height_difference_refuses() {
        let mut e = engine();
        let (actor, other, _) = talk_setup(&mut e);
        stub(&mut e, REFR_PARENT_CELL, 0x9000);
        stub(&mut e, CELL_FLAG_24_BIT_0, 1);
        stub_float(&mut e, FLOAT_ABS, 40.0);
        assert!(actor_should_talk_to(&mut e, actor, other));
        stub_float(&mut e, FLOAT_ABS, 95.0);
        assert!(!actor_should_talk_to(&mut e, actor, other));
        // Outdoors the height difference does not matter.
        stub(&mut e, CELL_FLAG_24_BIT_0, 0);
        assert!(actor_should_talk_to(&mut e, actor, other));
    }

    /// An actor with a process whose package (slot `0x22c`) is `package`, a
    /// package object with slot `0x13c`, and the globals
    /// `Actor::InitPackageLocations` reads.
    fn package_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr) {
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, -1.0f32);
        e.map(LARGE_FLOAT, 4);
        e.set_global(LARGE_FLOAT, f32::MAX);
        let package = object(e, 0x40, &[(0x13c, ret(0))]);
        let actor = actor_with(
            e,
            &[(0x2a8, ret(0))],
            Some(&[
                (0x28, ret(0)),
                (0x14, ret(0)),
                (0x24, ret(0)),
                (0x22c, ret(package.addr())),
            ]),
        );
        let process = e.get(actor, Actor::pCurrentProcess);
        stub(e, PACKAGE_TYPE, 5);
        stub(e, PACKAGE_GET_LOCATION_CELL, 0xc0de);
        stub(e, PACKAGE_GET_LOCATION_WORLD, 0xdead);
        stub(e, PACKAGE_GET_LOCATION_COORD, 0x7a00);
        (actor, package, process)
    }

    #[test]
    fn package_locations_are_initialized_and_the_actor_moved() {
        let mut e = engine();
        let (actor, package, process) = package_setup(&mut e);
        stub(&mut e, REFR_GET_REF_PERSISTS, 1);
        e.call(0x0089_3340, &args![actor, 0u32]);
        // The process is reset (slot 0x28), and its package slots are called.
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x28)),
            vec![vec![process.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x14)),
            vec![vec![process.addr(), actor.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x24)),
            vec![vec![process.addr(), actor.addr(), 0]]
        );
        // The package is remembered (the global object's flag is 0).
        assert_eq!(e.get(actor, Actor::pInitialPackage), package.addr());
        // The package accepts the actor? No (slot 0x13c false): it moves.
        assert_eq!(
            calls_to(&e, slot_target(&e, package, 0x13c)),
            vec![vec![
                package.addr(),
                actor.addr(),
                0,
                (-1.0f32).to_bits(),
                0
            ]]
        );
        assert_eq!(
            calls_to(&e, MOVE_REF_TO_NEW_SPACE),
            vec![vec![actor.addr(), 0xc0de, 0xdead]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_2A8)),
            vec![vec![actor.addr(), 0x7a00]]
        );
        assert_eq!(
            calls_to(&e, REFR_FLOAT_CALL),
            vec![vec![actor.addr(), f32::MAX.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_UNLOCK_LOCK_DOORS_PROCEDURE),
            vec![vec![actor.addr()]]
        );
    }

    #[test]
    fn package_locations_are_skipped_for_flagged_actors_and_simple_packages() {
        let mut e = engine();
        let (actor, package, _) = package_setup(&mut e);
        stub(&mut e, FORM_FLAG_800, 1);
        actor_init_package_locations(&mut e, actor, 0);
        assert_eq!(call_count(&e, ACTOR_UNLOCK_LOCK_DOORS_PROCEDURE), 0);
        stub(&mut e, FORM_FLAG_800, 0);
        // A package of type 2 is left alone.
        stub(&mut e, PACKAGE_TYPE, 2);
        actor_init_package_locations(&mut e, actor, 0);
        assert_eq!(call_count(&e, MOVE_REF_TO_NEW_SPACE), 0);
        assert_eq!(call_count(&e, ACTOR_UNLOCK_LOCK_DOORS_PROCEDURE), 1);
        // With the "use remembered" flag the remembered package is used and the
        // process is not reset.
        stub(&mut e, PACKAGE_TYPE, 5);
        e.set(actor, Actor::pInitialPackage, package.addr());
        clear_log(&mut e);
        actor_init_package_locations(&mut e, actor, 1);
        assert_eq!(
            call_count(
                &e,
                slot_target(&e, e.get(actor, Actor::pCurrentProcess), 0x28)
            ),
            0
        );
        assert_eq!(call_count(&e, MOVE_REF_TO_NEW_SPACE), 1);
        // A package of type 0xd does not move the actor.
        stub(&mut e, PACKAGE_TYPE, 0xd);
        clear_log(&mut e);
        actor_init_package_locations(&mut e, actor, 1);
        assert_eq!(call_count(&e, MOVE_REF_TO_NEW_SPACE), 0);
    }

    /// An actor, a real animation block, and the process slots
    /// `Actor::PickAnimations` reads (nothing queued, nothing sitting, no
    /// weapon), with the stubs that let it run to the end.
    fn pick_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr) {
        e.map(ANIM_GROUP_TABLE, 0x4000);
        e.map(ZERO_DOUBLE, 8);
        e.map(ONE_DOUBLE, 8);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.map(WEAPON_TYPE_ANIM_TABLE, 0x100);
        let animation = Ptr::<()>::new(e.mem.alloc(0x200));
        let acquire = object(
            e,
            0x40,
            &[(0x458, ret(0)), (0x27c, ret(0)), (0x1b0, ret(0))],
        );
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        let actor = actor_with(
            e,
            &[
                (0x22c, ret(0)),
                (0x230, ret(0)),
                (0x234, ret(0)),
                (0x218, ret(0)),
                (0x390, ret(0)),
                (0x1e4, ret(animation.addr())),
                (0x1e8, ret(0x6e00)),
                (0x4b0, ret(0)),
                (0x3ec, ret(0)),
                (0x428, ret(0)),
            ],
            Some(&[
                (0x148, ret(0)),
                (0x3e4, ret(0xffff_ffff)),
                (0x3e8, ret(0)),
                (0x4bc, ret(0)),
                (0x4d4, ret(0)),
                (0x444, float_ret(1.0)),
                (0x454, ret(0)),
                (0x458, ret(0)),
                (0x580, ret(0)),
                (0x574, ret(0)),
                (0x1cc, ret(0)),
                (0x190, ret(0)),
                (0x45c, ret(0)),
                (0x614, ret(0)),
                (0x368, ret(0)),
            ]),
        );
        let process = e.get(actor, Actor::pCurrentProcess);
        let controller = e.mem.alloc(0x500);
        stub(e, MOBILE_OBJECT_GET_CHAR_CONTROLLER, controller);
        stub(e, ACQUIRE_OBJECT_FIELD_28, 1);
        stub(e, ANIMATION_GET_SEQUENCE, 0x6500);
        stub(e, ANIMATION_ZERO_GLOBAL_TRANSFORM, 0x6600);
        stub(e, ANIMATION_GROUP_LOADED, 1);
        stub(e, ANIM_GROUP_COMPOSE, 0x55);
        stub(e, ANIMATION_PICK_BEST_ANIMATION, 0x55);
        stub(e, ANIM_GROUP_GET_TYPE, 0x55);
        (actor, animation, process)
    }

    #[test]
    fn picking_animations_does_nothing_for_dead_or_frozen_actors() {
        let mut e = engine();
        let (actor, _, _) = pick_setup(&mut e);
        set_slot(&mut e, actor.cast(), 0x22c, ret(1));
        e.call(0x0089_5110, &args![actor, 1.0f32, 1.0f32]);
        assert_eq!(call_count(&e, ANIMATION_GET_SEQUENCE), 0);
        set_slot(&mut e, actor.cast(), 0x22c, ret(0));
        set_slot(&mut e, actor.cast(), 0x230, ret(1));
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(call_count(&e, ANIMATION_GET_SEQUENCE), 0);
        // The first-person player is refused with a message.
        set_slot(&mut e, actor.cast(), 0x230, ret(0));
        set_player(&mut e, actor);
        stub(&mut e, PLAYER_FIRST_PERSON_CHECK, 0);
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(calls_to(&e, DEBUG_PRINT), vec![vec![FIRST_PERSON_MESSAGE]]);
        assert_eq!(call_count(&e, ANIMATION_GET_SEQUENCE), 0);
    }

    #[test]
    fn walking_forward_picks_the_walk_group_and_scales_its_speed() {
        let mut e = engine();
        let (actor, animation, _) = pick_setup(&mut e);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x0101);
        stub_float(&mut e, ACTOR_GET_WALK_SPEED, 150.0);
        stub(&mut e, ANIMATION_MOVEMENT_SPEED, 100);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 3);
        stub(&mut e, ANIMATION_PICK_BEST_ANIMATION, 0x0503);
        set_group(&mut e, 3, 1, 0);
        e.call(0x0089_5110, &args![actor, 2.0f32, 9.0f32]);
        // Flags bit 0 and a direction bit: group 3, selectors 0.
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE)[0], vec![0, 0, 3, 0]);
        // (150 / 100) * 2.0
        assert_eq!(e.mem.f32(animation.addr() + 0x10c), 3.0);
        assert_eq!(
            calls_to(&e, ANIMATION_PLAY_GROUP),
            vec![vec![animation.addr(), 0x0503, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), 0x4b0)),
            vec![vec![actor.addr(), 0x0503, 1]]
        );
    }

    #[test]
    fn turning_in_place_uses_the_second_speed_argument() {
        let mut e = engine();
        let (actor, animation, _) = pick_setup(&mut e);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x0010);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0xf);
        actor_pick_animations(&mut e, actor, 2.0, 9.0);
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE)[0], vec![0, 0, 0xf, 0]);
        assert_eq!(e.mem.f32(animation.addr() + 0x10c), 9.0);
        // Flag 0x20 asks for group 0x10.
        clear_log(&mut e);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x0020);
        actor_pick_animations(&mut e, actor, 2.0, 9.0);
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE)[0], vec![0, 0, 0x10, 0]);
    }

    #[test]
    fn a_queued_draw_tells_the_process_and_forces_the_animation() {
        let mut e = engine();
        let (actor, animation, process) = pick_setup(&mut e);
        set_slot(&mut e, process, 0x3e4, ret(0));
        set_slot(&mut e, process, 0x3e8, ret(0x6500));
        stub(&mut e, SEQUENCE_GET_GENERIC_LOCATION, 1);
        stub(&mut e, ANIMATION_SLOT_STATE, 1);
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x458)),
            vec![vec![process.addr(), actor.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x1cc)),
            vec![vec![
                process.addr(),
                1,
                0x6e00,
                animation.addr(),
                actor.addr()
            ]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_IRON_SIGHTS),
            vec![vec![actor.addr(), 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x580)),
            vec![vec![process.addr(), 1, 0, 0]]
        );
        // Already drawn: a queued draw changes nothing.
        clear_log(&mut e);
        set_slot(&mut e, process, 0x454, ret(1));
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(call_count(&e, slot_target(&e, process, 0x458)), 0);
    }

    #[test]
    fn the_player_draw_also_forces_the_first_person_pair() {
        let mut e = engine();
        let (actor, animation, process) = pick_setup(&mut e);
        set_player(&mut e, actor);
        stub(&mut e, PLAYER_FIRST_PERSON_CHECK, 1);
        stub(&mut e, PLAYER_GET_ANIMATION, 0x6a00);
        stub(&mut e, PLAYER_GET_BIPED, 0x6b00);
        set_slot(&mut e, process, 0x3e4, ret(0));
        set_slot(&mut e, process, 0x3e8, ret(0x6500));
        set_slot(&mut e, process, 0x190, ret(0));
        stub(&mut e, SEQUENCE_GET_GENERIC_LOCATION, 1);
        stub(&mut e, ANIMATION_SLOT_STATE, 1);
        e.map(actor.addr() + 0xac, 4);
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        let forced = calls_to(&e, slot_target(&e, process, 0x1cc));
        assert_eq!(
            forced,
            vec![
                vec![process.addr(), 1, 0x6e00, animation.addr(), actor.addr()],
                vec![process.addr(), 1, 0x6b00, 0x6a00, actor.addr()]
            ]
        );
        // Queued action 0 (draw) for the player sets shader values 0 and 1.
        assert_eq!(call_count(&e, SET_SHADER_VALUE), 2);
    }

    #[test]
    fn a_queued_landing_state_clears_the_fall_action() {
        let mut e = engine();
        let (actor, _, process) = pick_setup(&mut e);
        set_slot(&mut e, process, 0x3e4, ret(0xc));
        set_slot(&mut e, process, 0x3e8, ret(0x6500));
        stub(&mut e, SEQUENCE_GET_GENERIC_LOCATION, 1);
        stub(&mut e, CONTROLLER_STATE, 1);
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION)[0],
            vec![actor.addr(), 0xffff_ffff, 0]
        );
    }

    #[test]
    fn landing_from_a_jump_picks_a_landing_group_and_plays_the_sound() {
        let mut e = engine();
        let (actor, animation, _) = pick_setup(&mut e);
        stub(&mut e, ACTOR_ANIM_FLAGS, 1);
        stub(&mut e, CONTROLLER_STATE, 0);
        stub(&mut e, TRANSFORM_IS_LOOPING, 1);
        stub(&mut e, LANDING_SOUND_LOOKUP, 0x7b00);
        stub(&mut e, ANIM_GROUP_GET_TYPE, 0xf1);
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(
            calls_to(&e, ANIMATION_LAND_RESET),
            vec![vec![animation.addr()]]
        );
        assert_eq!(
            calls_to(&e, IMPACT_MIXER_PLAY_JUMP_LAND),
            vec![vec![actor.addr(), 0x7b00]]
        );
        // The landing group follows the direction flag: 0xf1 for bit 0.
        assert_eq!(calls_to(&e, ANIM_GROUP_COMPOSE)[0][2], 0xf1);
        // The action is set to 0xc for the group's sequence.
        assert_eq!(
            calls_to(&e, ACTOR_SET_ANIM_ACTION)
                .iter()
                .filter(|a| a[1] == 0xc)
                .count(),
            1
        );
    }

    #[test]
    fn the_weapon_position_is_saved_while_an_attack_plays() {
        let mut e = engine();
        let (actor, animation, process) = pick_setup(&mut e);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        stub(&mut e, ACTOR_GET_WEAPON_POSITION, position);
        stub(&mut e, ANIM_GROUP_IS_ATTACK_ACTION, 1);
        actor_pick_animations(&mut e, actor, 1.0, 1.0);
        assert_eq!(calls_to(&e, ACTOR_GET_WEAPON_POSITION)[0][0], actor.addr());
        assert_eq!(
            calls_to(&e, slot_target(&e, process, 0x45c)),
            vec![vec![
                process.addr(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        let _ = animation;
    }

    /// Gives the object whose vtable pointer is at `address` (an embedded
    /// part of a larger object) a table with one double per slot.
    fn embedded_table(e: &mut Engine, address: u32, slots: &[(u32, Ret)]) {
        let table = e.mem.alloc(0x800);
        for (offset, result) in slots {
            let target = e.mem.alloc(8);
            let result = *result;
            e.register_double(target, move |_, _| result);
            e.mem.set_u32(table + offset, target);
        }
        e.mem.set_u32(address, table);
    }

    /// Maps a `double` constant of the exe and sets it.
    fn double_global(e: &mut Engine, address: u32, value: f64) {
        e.map(address, 8);
        e.set_global(address, value);
    }

    /// An actor whose value owner (`+0xa4`) reports `current` for slot `0xc`
    /// and `base` for slot `4`.
    fn actor_with_health(
        e: &mut Engine,
        extra_slots: &[(u32, Ret)],
        current: f32,
        base: f32,
    ) -> Ptr<Actor> {
        let mut slots = extra_slots.to_vec();
        if !slots.iter().any(|(offset, _)| *offset == ACTOR_SLOT_22C) {
            slots.push((ACTOR_SLOT_22C, ret(0)));
        }
        let actor = actor_with(e, &slots, None);
        embedded_table(
            e,
            actor.addr() + 0xa4,
            &[(0x0c, float_ret(current)), (0x04, float_ret(base))],
        );
        actor
    }

    #[test]
    fn fn_008985d0_adds_a_package_by_process_type() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_ADD_PACKAGE, ret(0))], None);
        stub(&mut e, PACKAGE_CREATE, 0x5000);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 1);
        fn_008985d0(&mut e, actor);
        assert_eq!(calls_to(&e, PACKAGE_CREATE), vec![vec![0x24]]);
        assert_eq!(calls_to(&e, ACTOR_END_MOVEMENT), vec![vec![actor.addr()]]);
        assert_eq!(
            calls_to(&e, PACKAGE_CALCULATE_PROCEDURE_TYPE),
            vec![vec![0x5000, 0]]
        );
        let add = slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_PACKAGE);
        assert_eq!(calls_to(&e, add), vec![vec![actor.addr(), 0x5000, 1, 1]]);
        assert_eq!(call_count(&e, PACKAGE_SET_FLAG_B), 0);

        e.call_log = Some(vec![]);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 2);
        fn_008985d0(&mut e, actor);
        assert_eq!(calls_to(&e, PACKAGE_SET_FLAG_B), vec![vec![0x5000, 1]]);
        assert_eq!(calls_to(&e, add), vec![vec![actor.addr(), 0x5000, 0, 0]]);
    }

    /// The constants `fn_00898650` reads.
    fn kill_constants(e: &mut Engine) {
        double_global(e, QUARTER, 0.25);
        double_global(e, HALF, 0.5);
        double_global(e, ONE_DOUBLE, 1.0);
    }

    #[test]
    fn fn_00898650_kills_at_zero_health_and_barks_at_low_health() {
        let mut e = engine();
        kill_constants(&mut e);
        let attacker = actor_with(&mut e, &[], None);
        // Health 0.5 of 10: the actor is killed directly; the follower barks
        // the quarter-health bark.
        let actor = actor_with_health(&mut e, &[(ACTOR_SLOT_428, ret(0))], 0.5, 10.0);
        stub(&mut e, PROCESS_GET_FORCE_NEXT_UPDATE, 1);
        fn_00898650(&mut e, actor, attacker, 3.0);
        assert_eq!(
            calls_to(&e, TRIGGER_FOLLOWER_BARK),
            vec![vec![actor.addr(), 5]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_KILL),
            vec![vec![actor.addr(), attacker.addr(), 3.0f32.to_bits()]]
        );

        // A third of the health gives bark 4 and, being above 1, no kill.
        e.call_log = Some(vec![]);
        let actor = actor_with_health(&mut e, &[(ACTOR_SLOT_428, ret(0))], 3.0, 10.0);
        fn_00898650(&mut e, actor, attacker, 3.0);
        assert_eq!(
            calls_to(&e, TRIGGER_FOLLOWER_BARK),
            vec![vec![actor.addr(), 4]]
        );
        assert_eq!(call_count(&e, ACTOR_KILL), 0);

        // A base of 0 gives the ratio 1.0: no bark.
        e.call_log = Some(vec![]);
        let actor = actor_with_health(&mut e, &[(ACTOR_SLOT_428, ret(0))], 0.0, 0.0);
        fn_00898650(&mut e, actor, attacker, 3.0);
        assert_eq!(call_count(&e, TRIGGER_FOLLOWER_BARK), 0);
        assert_eq!(call_count(&e, ACTOR_KILL), 1);
    }

    #[test]
    fn fn_00898650_queues_the_kill_when_the_task_queue_is_used() {
        let mut e = engine();
        kill_constants(&mut e);
        let attacker = actor_with(&mut e, &[], None);
        let actor = actor_with_health(&mut e, &[(ACTOR_SLOT_428, ret(0))], 0.0, 10.0);
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, GET_TES, 0x7777);
        stub_with(&mut e, OPERATOR_NEW, |e, _| ret(e.mem.alloc(4)));
        fn_00898650(&mut e, actor, attacker, 2.5);
        let queued = calls_to(&e, QUEUE_ACTOR_KILL);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][..2], [0x7777, actor.addr()]);
        assert_eq!(queued[0][3], 0);
        assert_eq!(e.mem.f32(queued[0][2]), 2.5);
        assert_eq!(e.get(actor, Actor::pMyKiller).addr(), attacker.addr());
        assert_eq!(call_count(&e, ACTOR_KILL), 0);
    }

    #[test]
    fn fn_00898650_drops_the_player_as_attacker_when_the_reaction_is_two() {
        let mut e = engine();
        kill_constants(&mut e);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let actor = actor_with_health(&mut e, &[(ACTOR_SLOT_428, ret(0x4400))], 0.0, 10.0);
        stub(&mut e, ACTOR_GET_FACTION_FIGHT_REACTION, 2);
        fn_00898650(&mut e, actor, player, 1.0);
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET),
            vec![vec![0x4400, player.addr()]]
        );
        let reaction = calls_to(&e, ACTOR_GET_FACTION_FIGHT_REACTION);
        assert_eq!(reaction[0][..2], [actor.addr(), player.addr()]);
        assert_eq!(calls_to(&e, ACTOR_KILL)[0][1], 0);

        // Another reaction keeps the attacker.
        e.call_log = Some(vec![]);
        stub(&mut e, ACTOR_GET_FACTION_FIGHT_REACTION, 1);
        fn_00898650(&mut e, actor, player, 1.0);
        assert_eq!(calls_to(&e, ACTOR_KILL)[0][1], player.addr());

        // A combat target is never asked.
        e.call_log = Some(vec![]);
        stub(&mut e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET, 1);
        fn_00898650(&mut e, actor, player, 1.0);
        assert_eq!(call_count(&e, ACTOR_GET_FACTION_FIGHT_REACTION), 0);
    }

    #[test]
    fn fn_00898650_does_nothing_for_a_protected_actor() {
        let mut e = engine();
        kill_constants(&mut e);
        let attacker = actor_with(&mut e, &[], None);
        let actor = actor_with_health(&mut e, &[(ACTOR_SLOT_22C, ret(1))], 0.0, 10.0);
        fn_00898650(&mut e, actor, attacker, 1.0);
        assert_eq!(call_count(&e, ACTOR_KILL), 0);
        assert_eq!(call_count(&e, TRIGGER_FOLLOWER_BARK), 0);
    }

    #[test]
    fn small_flag_and_field_accessors() {
        let mut e = engine();
        let object = Ptr::<()>::new(e.mem.alloc(0x1000));
        e.mem.set_u32(object.addr() + 0x12c, 0x10);
        assert!(fn_00898fd0(&mut e, object));
        e.mem.set_u32(object.addr() + 0x12c, 0xffef);
        assert!(!fn_00898fd0(&mut e, object));
        assert_eq!(fn_0089d600(&mut e, object), object.byte_add(0x450));
        e.mem.set_u32(object.addr() + 0x2e4, 0x2e4);
        assert_eq!(fn_0089d620(&mut e, object), 0x2e4);
        fn_0089d640(&mut e, object, 1, 2, 3);
        assert_eq!(e.mem.u32(object.addr() + 0xf4), 1);
        assert_eq!(e.mem.u32(object.addr() + 0xf8), 2);
        assert_eq!(e.mem.u32(object.addr() + 0xfc), 3);
        e.mem.set_u8(object.addr() + 0x28d, 7);
        e.mem.set_u8(object.addr() + 0x28e, 8);
        e.mem.set_u32(object.addr() + 0x29c, 0x29c);
        e.mem.set_u32(object.addr() + 0x298, 0x298);
        assert_eq!(fn_0089d670(&mut e, object), 7);
        assert_eq!(fn_0089d690(&mut e, object), 8);
        assert_eq!(fn_0089d6b0(&mut e, object), 0x29c);
        assert_eq!(fn_0089d6d0(&mut e, object), 0x298);
        e.mem.set_u32(object.addr() + 0x638, 0x638);
        e.mem.set_u8(object.addr() + 0xe2d, 9);
        assert_eq!(fn_0089f4e0(&mut e, object), 0x638);
        assert_eq!(fn_0089f500(&mut e, object), 9);
        fn_0089f520(&mut e, object, 5);
        assert_eq!(e.mem.u8(object.addr() + 0x18a), 5);
        assert_eq!(fn_0089f540(&mut e, object), 5);
        e.mem.set_u32(object.addr() + 0x3c, 0xffff_ffff);
        fn_0089f560(&mut e, object);
        assert_eq!(e.mem.u32(object.addr() + 0x3c), 0);
        assert!(!fn_0089d5e0(&mut e, object));
        stub(&mut e, FORM_TEST_FLAGS, 1);
        assert!(fn_0089d5e0(&mut e, object));
        assert_eq!(
            calls_to(&e, FORM_TEST_FLAGS),
            vec![
                vec![object.addr(), 0x0800_0000],
                vec![object.addr(), 0x0800_0000]
            ]
        );
        let actor = actor_with(&mut e, &[], None);
        fn_008a0250(&mut e, actor.cast());
        assert_eq!(e.mem.u8(actor.addr() + 0xc4), 1);
    }

    #[test]
    fn fn_00898ff0_asks_the_object_inside_the_base_form() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        let base = e.mem.alloc(0x40);
        embedded_table(&mut e, base + 0x30, &[(0x18, ret(1))]);
        stub(&mut e, ACTOR_BASE_FORM, base);
        assert!(fn_00898ff0(&mut e, actor));
        embedded_table(&mut e, base + 0x30, &[(0x18, ret(0))]);
        assert!(!fn_00898ff0(&mut e, actor));
    }

    #[test]
    fn acquire_object_forwarders() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        let acquire = object(&mut e, 0x10, &[(0x4f0, ret(0x1234)), (0x4f4, ret(0))]);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        assert_eq!(fn_00899030(&mut e, actor), 0x1234);
        fn_00899060(&mut e, actor, 7);
        let slot = slot_target(&e, acquire, 0x4f4);
        assert_eq!(calls_to(&e, slot), vec![vec![acquire.addr(), 7]]);

        e.call_log = Some(vec![]);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        fn_00899060(&mut e, actor, 7);
        assert_eq!(call_count(&e, slot), 0);
    }

    #[test]
    fn blocked_attack_prints_the_message_only_when_debugging() {
        let mut e = engine();
        double_global(&mut e, HUNDRED, 100.0);
        let this = actor_with(&mut e, &[], None);
        let other = actor_with(&mut e, &[], None);
        stub_with(&mut e, REFR_GET_NAME, |_, a| ret(a[0] + 1));
        actor_handle_blocked_attack(&mut e, this, 0, 0.5, other, 0);
        assert_eq!(call_count(&e, PRINT_DEBUG_LINE), 0);

        stub(&mut e, DEBUG_MESSAGES_ENABLED, 1);
        actor_handle_blocked_attack(&mut e, this, 0, 0.5, other, 0);
        let percent = (0.5f32 as f64 * 100.0).to_bits();
        assert_eq!(
            calls_to(&e, PRINT_DEBUG_LINE),
            vec![vec![
                BLOCKS_BLOW_FORMAT,
                other.addr() + 1,
                percent as u32,
                (percent >> 32) as u32,
                this.addr() + 1
            ]]
        );
    }

    #[test]
    fn fn_008990f0_scales_the_weapon_reach() {
        let mut e = engine();
        e.map(SPECIAL_IDLE_STATE_OBJECT, 0x10);
        let actor = actor_with(
            &mut e,
            &[(ACTOR_SLOT_REACH, float_ret(2.0)), (ACTOR_SLOT_360, ret(0))],
            None,
        );
        stub_float(&mut e, REFR_GET_SCALE, 1.5);
        stub(&mut e, ACTOR_REACH_WORKER, 0x99);
        // No weapon: the actor's own reach times the scale.
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        assert_eq!(fn_008990f0(&mut e, actor), 0x99);
        let passed = calls_to(&e, ACTOR_REACH_WORKER);
        assert_eq!(passed, vec![vec![actor.addr(), 3.0f32.to_bits()]]);

        // A weapon: its reach goes through CalcWeaponReach.
        e.call_log = Some(vec![]);
        let acquire = object(&mut e, 0x10, &[(PROCESS_SLOT_CURRENT_WEAPON, ret(0x6000))]);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(&mut e, ENTRY_FORM, 0x6100);
        stub_float(&mut e, WEAPON_REACH, 4.0);
        stub_float(&mut e, CALC_WEAPON_REACH, 8.0);
        fn_008990f0(&mut e, actor);
        assert_eq!(
            calls_to(&e, CALC_WEAPON_REACH),
            vec![vec![4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_REACH_WORKER),
            vec![vec![actor.addr(), 12.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_008990f0_multiplies_by_the_setting_for_special_idle_kind_four() {
        let mut e = engine();
        e.map(SPECIAL_IDLE_STATE_OBJECT, 0x10);
        let actor = actor_with(
            &mut e,
            &[(ACTOR_SLOT_REACH, float_ret(2.0)), (ACTOR_SLOT_360, ret(1))],
            None,
        );
        stub_float(&mut e, REFR_GET_SCALE, 1.0);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 0.5);
        stub(&mut e, SETTING_GET_VALUE_POINTER, setting);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        stub_with(&mut e, ENTRY_FORM, |_, a| {
            ret(if a[0] == SPECIAL_IDLE_STATE_OBJECT {
                4
            } else {
                0
            })
        });
        fn_008990f0(&mut e, actor);
        assert_eq!(
            calls_to(&e, SETTING_GET_VALUE_POINTER),
            vec![vec![REACH_MULTIPLIER_SETTING]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_REACH_WORKER),
            vec![vec![actor.addr(), 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_0089d6f0_applies_health_and_fatigue_loss() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let attacker = actor_with(&mut e, &[(ACTOR_SLOT_428, ret(0x4400))], None);
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_428, ret(0x4500)),
                (ACTOR_SLOT_360, ret(0)),
                (ACTOR_SLOT_MODIFY_ACTOR_VALUE, ret(0)),
                (ACTOR_SLOT_22C, ret(0x42)),
            ],
            None,
        );
        embedded_table(&mut e, actor.addr() + 0x94, &[(0x10, ret(0))]);
        let acquire = object(&mut e, 0x10, &[(PROCESS_SLOT_72C, ret(0))]);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        // The difficulty adjustment returns -4 for an amount of -3.
        stub_float(&mut e, DIFFICULTY_ADJUST_HEALTH_MODIFIER, -4.0);
        // The fatigue is 10 and the setting 5: the fatigue loss applies.
        embedded_table(&mut e, actor.addr() + 0xa4, &[(0x0c, float_ret(10.0))]);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 5.0);
        stub(&mut e, SETTING_GET_VALUE_POINTER, setting);
        stub(&mut e, CONTROLLER_SET_FLAG, 0);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 0);
        stub(&mut e, PROCESS_GET_FORCE_NEXT_UPDATE, 0);
        let process = object(&mut e, 0x10, &[(PROCESS_SLOT_22C, ret(0x4600))]);
        e.set(actor, Actor::pCurrentProcess, process);

        let result = fn_0089d6f0(&mut e, actor, 3.0, 2.0, attacker);
        assert_eq!(result, 0x42);
        assert_eq!(
            calls_to(&e, DIFFICULTY_ADJUST_HEALTH_MODIFIER),
            vec![vec![actor.addr(), (-3.0f32).to_bits(), attacker.addr()]]
        );
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_NOTIFY_DAMAGE),
            vec![vec![0x4400, actor.addr(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, CONTROLLER_RECORD_DAMAGE),
            vec![vec![0x4600, attacker.addr(), 4.0f32.to_bits()]]
        );
        let reaction = slot_target(&e, acquire, PROCESS_SLOT_72C);
        assert_eq!(
            calls_to(&e, reaction),
            vec![vec![acquire.addr(), attacker.addr(), 4.0f32.to_bits()]]
        );
        let modify = slot_target(&e, actor.cast(), ACTOR_SLOT_MODIFY_ACTOR_VALUE);
        assert_eq!(
            calls_to(&e, modify),
            vec![
                vec![actor.addr(), 0x10, (-4.0f32).to_bits(), attacker.addr()],
                vec![actor.addr(), 0x16, (-2.0f32).to_bits(), attacker.addr()],
            ]
        );
        assert_eq!(
            calls_to(&e, REFR_FLOAT_SETTER_579220),
            vec![vec![actor.addr(), 3.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn fn_0089d6f0_stops_for_a_protected_actor_and_skips_small_losses() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_22C, ret(0))], None);
        embedded_table(&mut e, actor.addr() + 0x94, &[(0x10, ret(0x1201))]);
        assert_eq!(fn_0089d6f0(&mut e, actor, 3.0, 2.0, Ptr::NULL), 0x1200);
        assert_eq!(call_count(&e, DIFFICULTY_ADJUST_HEALTH_MODIFIER), 0);

        // A zero adjusted loss and a zero fatigue loss change nothing.
        embedded_table(&mut e, actor.addr() + 0x94, &[(0x10, ret(0))]);
        stub_float(&mut e, DIFFICULTY_ADJUST_HEALTH_MODIFIER, 0.0);
        fn_0089d6f0(&mut e, actor, 3.0, 0.0, Ptr::NULL);
        assert_eq!(call_count(&e, REFR_FLOAT_SETTER_579220), 0);
        assert_eq!(call_count(&e, SETTING_GET_VALUE_POINTER), 0);
    }

    #[test]
    fn worn_armor_falls_back_to_the_second_slot() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        stub_with(&mut e, ACTOR_GET_ARMOR_BEING_WORN, |_, a| {
            ret(if a[1] == 1 { 0xaa } else { 0 })
        });
        assert_eq!(actor_get_random_worn_armor(&mut e, actor, 1), 0xaa);
        assert_eq!(
            calls_to(&e, ACTOR_GET_ARMOR_BEING_WORN),
            vec![vec![actor.addr(), 0], vec![actor.addr(), 1]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(actor_get_random_worn_armor(&mut e, actor, 0), 0);
        assert_eq!(
            calls_to(&e, ACTOR_GET_ARMOR_BEING_WORN),
            vec![vec![actor.addr(), 2]]
        );
    }

    /// An actor with a drawn weapon entry; `fn_0089f580` finds the weapon
    /// bone's node and drops the weapon.
    fn drop_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr) {
        let process = object(
            e,
            0x800,
            &[
                (PROCESS_SLOT_CURRENT_WEAPON, ret(0)),
                (PROCESS_SLOT_534, ret(0x7000)),
                (PROCESS_SLOT_WEAPON_BONE, ret(0)),
                (PROCESS_SLOT_580, ret(0)),
                (PROCESS_SLOT_22C, ret(0x4600)),
            ],
        );
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_1E8, ret(3)),
                (ACTOR_SLOT_DROP_OBJECT, ret(0x8000)),
                (ACTOR_SLOT_428, ret(1)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        let weapon = e.mem.alloc(8);
        let list = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(weapon, list);
        e.mem.set_u32(list, first);
        stub(e, GET_SAVED_ACQUIRE_OBJECT, process.addr());
        stub(e, ACTOR_IS_WEAPON_DRAWN, 1);
        stub(e, ENTRY_FORM, 0x7100);
        stub(e, ENTRY_FIRST_EXTRA_LIST, list);
        stub_with(e, LIST_ITEM_SLOT, |_, a| ret(a[0]));
        // The process reports `weapon` as the current weapon.
        let slot = slot_target(e, process, PROCESS_SLOT_CURRENT_WEAPON);
        e.register_double(slot, move |_, _| ret(weapon));
        (actor, process, Ptr::new(weapon))
    }

    #[test]
    fn fn_0089f580_drops_the_drawn_weapon_at_the_bone() {
        let mut e = engine();
        let (actor, process, weapon) = drop_setup(&mut e);
        let node = e.mem.alloc(0x100);
        let bone = slot_target(&e, process, PROCESS_SLOT_WEAPON_BONE);
        e.register_double(bone, move |_, _| ret(node));
        let translate = e.mem.alloc(12);
        e.mem.set_f32(translate, 1.0);
        e.mem.set_f32(translate + 4, 2.0);
        e.mem.set_f32(translate + 8, 3.0);
        stub(&mut e, NODE_WORLD_TRANSLATE, translate);
        stub(&mut e, NODE_ROTATION, node + 0x68);
        stub(&mut e, GET_FORM_ENCHANTING, 0x9999);
        let seen_z = e.mem.alloc(4);
        let drop_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_DROP_OBJECT);
        e.register_double(drop_slot, move |e, a| {
            let z = e.mem.f32(a[4] + 8);
            e.mem.set_f32(seen_z, z);
            ret(0x8000)
        });
        fn_0089f580(&mut e, actor);

        let slot = slot_target(&e, process, PROCESS_SLOT_WEAPON_BONE);
        assert_eq!(calls_to(&e, slot), vec![vec![process.addr(), 3]]);
        let angles = calls_to(&e, NI_MATRIX3_TO_EULER_ANGLES_XYZ);
        assert_eq!(angles.len(), 1);
        assert_eq!(angles[0][0], node + 0x68);
        assert_eq!(angles[0][2], angles[0][1] + 4);
        let drop = calls_to(&e, drop_slot);
        assert_eq!(drop.len(), 1);
        // this, form, extra data list, count, position, angles
        assert_eq!(
            drop[0][..4],
            [actor.addr(), 0x7100, e.mem.u32(e.mem.u32(weapon.addr())), 1]
        );
        assert_eq!(e.mem.f32(seen_z), 3.0);
        assert_eq!(drop[0][5], angles[0][1]);
        assert_eq!(
            calls_to(&e, ACTOR_DROPPED_OBJECT_WORKER),
            vec![vec![actor.addr(), 0x8000]]
        );
        // The form is enchanted: the shader effects slot runs.
        let effects = slot_target(&e, process, PROCESS_SLOT_580);
        assert_eq!(calls_to(&e, effects), vec![vec![process.addr(), 1, 0, 0]]);
        assert_eq!(calls_to(&e, CONTROLLER_SET_FLAG), vec![vec![0x4600, 0]]);
    }

    #[test]
    fn fn_0089f580_stops_at_each_blocking_condition() {
        // The form is the one equipped in the other hand.
        let mut e = engine();
        let (actor, _, _) = drop_setup(&mut e);
        stub(&mut e, ENTRY_FORM, 0x7000);
        fn_0089f580(&mut e, actor);
        assert_eq!(call_count(&e, GET_FORM_ENCHANTING), 0);
        assert_eq!(call_count(&e, CONTROLLER_SET_FLAG), 0);

        // The weapon cannot be worn.
        let mut e = engine();
        let (actor, _, _) = drop_setup(&mut e);
        stub(&mut e, EXTRA_GET_CAN_NOT_WEAR, 1);
        fn_0089f580(&mut e, actor);
        assert_eq!(call_count(&e, GET_FORM_ENCHANTING), 0);

        // Not drawn.
        let mut e = engine();
        let (actor, _, _) = drop_setup(&mut e);
        stub(&mut e, ACTOR_IS_WEAPON_DRAWN, 0);
        fn_0089f580(&mut e, actor);
        assert_eq!(call_count(&e, EXTRA_GET_CAN_NOT_WEAR), 0);

        // The form flags.
        let mut e = engine();
        let (actor, _, _) = drop_setup(&mut e);
        stub(&mut e, FORM_FLAG_100_BIT_3, 1);
        fn_0089f580(&mut e, actor);
        assert_eq!(call_count(&e, FORM_FLAG_100_BIT_5), 0);
        let mut e = engine();
        let (actor, _, _) = drop_setup(&mut e);
        stub(&mut e, FORM_FLAG_100_BIT_5, 1);
        fn_0089f580(&mut e, actor);
        assert_eq!(call_count(&e, ACTOR_IS_WEAPON_DRAWN), 0);

        // Without a bone node nothing is dropped but the process is told.
        let mut e = engine();
        let (actor, _, _) = drop_setup(&mut e);
        fn_0089f580(&mut e, actor);
        assert_eq!(call_count(&e, ACTOR_DROPPED_OBJECT_WORKER), 0);
        assert_eq!(call_count(&e, CONTROLLER_SET_FLAG), 1);
    }

    #[test]
    fn fn_0089fb80_refreshes_the_worn_items_by_form_type() {
        let mut e = engine();
        e.map(BASE_DATA_OBJECT, 4);
        e.set_global(BASE_DATA_OBJECT, 0x5a5a_0000u32);
        let actor = actor_with(
            &mut e,
            &[(ACTOR_SLOT_1CC, ret(0)), (ACTOR_SLOT_1D0, ret(0x9000))],
            None,
        );
        stub(&mut e, GET_CONTAINER_CHANGES, 0x6100);
        stub(&mut e, PATH_MANAGER_Q_INSTANCE, 0x6200);
        stub(&mut e, REFR_BASE_FORM, 0x6300);
        stub(&mut e, FORM_TYPE, 0x2a);
        fn_0089fb80(&mut e, actor, 7);
        // No acquire object: the actor's own slot 0x1cc is called.
        let refresh = slot_target(&e, actor.cast(), ACTOR_SLOT_1CC);
        assert_eq!(calls_to(&e, refresh), vec![vec![actor.addr(), 0, 1]]);
        assert_eq!(calls_to(&e, BASE_DATA_PREDICATE), vec![vec![0x5a5a_0000]]);
        assert_eq!(
            calls_to(&e, PATH_MANAGER_CLEAR_INVENTORY_POINTERS),
            vec![vec![0x6200, 0x6100]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_UPDATE_INVENTORY_WORKER),
            vec![vec![actor.addr(), 7]]
        );
        assert_eq!(
            calls_to(&e, NPC_INIT_DEFAULT_WORN),
            vec![vec![0x6300, actor.addr(), 1, 1, 0, 1]]
        );
        assert_eq!(call_count(&e, CREATURE_INIT_DEFAULT_WORN), 0);

        // A creature form, and a current package of the accepted kind.
        e.call_log = Some(vec![]);
        stub(&mut e, FORM_TYPE, 0x2b);
        stub(&mut e, GET_CURRENT_PACKAGE, 0x6400);
        stub(&mut e, PACKAGE_KIND_TEST, 1);
        fn_0089fb80(&mut e, actor, 0);
        assert_eq!(
            calls_to(&e, CREATURE_INIT_DEFAULT_WORN),
            vec![vec![0x6300, actor.addr(), 1, 0, 1]]
        );
        assert_eq!(call_count(&e, NPC_INIT_DEFAULT_WORN), 0);
    }

    #[test]
    fn fn_0089fb80_prefers_the_acquire_object_and_honours_the_predicate() {
        let mut e = engine();
        e.map(BASE_DATA_OBJECT, 4);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_1D0, ret(0))], None);
        let acquire = object(&mut e, 0x10, &[(PROCESS_SLOT_468, ret(0))]);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 1);
        fn_0089fb80(&mut e, actor, 0);
        let slot = slot_target(&e, acquire, PROCESS_SLOT_468);
        assert_eq!(calls_to(&e, slot), vec![vec![acquire.addr(), 1]]);

        // A field above 1 uses the actor's slot (not present in this vtable:
        // the predicate is set instead, so nothing is refreshed at all).
        e.call_log = Some(vec![]);
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 2);
        stub(&mut e, BASE_DATA_PREDICATE, 1);
        fn_0089fb80(&mut e, actor, 0);
        assert_eq!(call_count(&e, slot), 0);
        assert_eq!(call_count(&e, GET_SAVED_ACQUIRE_OBJECT), 0);
        assert_eq!(call_count(&e, NPC_INIT_DEFAULT_WORN), 0);
    }

    /// An actor ready for `fn_0089fcf0`: no combat controller yet, a process
    /// object and the constants the function reads.
    fn combat_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, Ptr<Actor>, Ptr) {
        double_global(e, ZERO_DOUBLE, 0.0);
        double_global(e, QUARTER, 0.25);
        e.map(COMBAT_MANAGER, 4);
        e.set_global(COMBAT_MANAGER, 0x7100u32);
        let process = object(
            e,
            0x800,
            &[
                (PROCESS_SLOT_2C8, ret(0)),
                (PROCESS_SLOT_22C, ret(0)),
                (ACTOR_SLOT_214, ret(0)),
            ],
        );
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_22C, ret(0)),
                (ACTOR_SLOT_214, ret(0)),
                (ACTOR_SLOT_REACH, float_ret(10.0)),
                (ACTOR_SLOT_428, ret(0)),
                (ACTOR_SLOT_3BC, ret(0)),
                (ACTOR_SLOT_ADD_PACKAGE, ret(0)),
                (ACTOR_SLOT_418, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        let target = actor_with(e, &[], None);
        let acquire = object(
            e,
            0x10,
            &[
                (PROCESS_SLOT_208, float_ret(55.7)),
                (PROCESS_SLOT_28, ret(0)),
                (PROCESS_SLOT_710, ret(0)),
                (PROCESS_SLOT_52C, ret(0)),
                (PROCESS_SLOT_4D4, ret(0)),
            ],
        );
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        (actor, process, target, acquire)
    }

    #[test]
    fn fn_0089fcf0_adds_a_combatant_and_starts_the_package() {
        let mut e = engine();
        let (actor, process, target, acquire) = combat_setup(&mut e);
        let controller = e.mem.alloc(0x100);
        stub_with(&mut e, FTOL, |_, a| {
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        stub(&mut e, COMBAT_MANAGER_ADD_COMBATANT, controller);
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 1, 0, 0, 0);
        assert_eq!(
            calls_to(&e, COMBAT_MANAGER_ADD_COMBATANT),
            vec![vec![0x7100, actor.addr(), target.addr(), 55, 0]]
        );
        let slot_208 = slot_target(&e, acquire, PROCESS_SLOT_208);
        assert_eq!(
            calls_to(&e, slot_208),
            vec![vec![acquire.addr(), target.addr(), actor.addr()]]
        );
        // flag_b set: the controller is told about the target.
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_NOTIFY_DAMAGE),
            vec![vec![controller, target.addr(), 0]]
        );
        let add_package = slot_target(&e, actor.cast(), ACTOR_SLOT_ADD_PACKAGE);
        assert_eq!(
            calls_to(&e, add_package),
            vec![vec![actor.addr(), controller, 0, 1]]
        );
        let slot_710 = slot_target(&e, acquire, PROCESS_SLOT_710);
        assert_eq!(
            calls_to(&e, slot_710),
            vec![vec![acquire.addr(), actor.addr()]]
        );
        assert_eq!(e.mem.u8(actor.addr() + 0x104), 1);
        let hostile = slot_target(&e, process, PROCESS_SLOT_2C8);
        assert_eq!(calls_to(&e, hostile), vec![vec![process.addr(), 0]]);
        // The controller was not marked (flag_a is 0).
        assert_eq!(e.mem.u8(controller + 0xc4), 0);
    }

    #[test]
    fn fn_0089fcf0_adds_a_group_member_or_a_pretend_combatant() {
        let mut e = engine();
        let (actor, _, target, _) = combat_setup(&mut e);
        stub(&mut e, COMBAT_MANAGER_ADD_GROUP_MEMBER, 0);
        fn_0089fcf0(&mut e, actor, target, 0x55, 0, 0, 0, 3, 0, 0x66);
        assert_eq!(
            calls_to(&e, COMBAT_MANAGER_ADD_GROUP_MEMBER),
            vec![vec![0x7100, actor.addr(), 0x55]]
        );
        assert_eq!(call_count(&e, COMBAT_MANAGER_ADD_COMBATANT), 0);
        // No controller came back: the combat flag is cleared.
        assert_eq!(e.mem.u8(actor.addr() + 0x104), 0);

        e.call_log = Some(vec![]);
        stub(&mut e, COMBAT_MANAGER_ADD_PRETEND_COMBATANT, 0);
        fn_0089fcf0(&mut e, actor, Ptr::NULL, 0, 0, 0, 0, 3, 0, 0x66);
        assert_eq!(
            calls_to(&e, COMBAT_MANAGER_ADD_PRETEND_COMBATANT),
            vec![vec![0x7100, actor.addr(), 0x66]]
        );
    }

    #[test]
    fn fn_0089fcf0_marks_the_controller_and_clears_the_crosshair_for_the_player() {
        let mut e = engine();
        let (actor, _, _, _) = combat_setup(&mut e);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let controller = e.mem.alloc(0x100);
        stub(&mut e, COMBAT_MANAGER_ADD_COMBATANT, controller);
        stub(&mut e, CROSSHAIR_TARGET, actor.addr());
        fn_0089fcf0(&mut e, actor, player, 0, 1, 0, 0, 5, 0, 0);
        assert_eq!(e.mem.u8(controller + 0xc4), 1);
        assert_eq!(
            calls_to(&e, PLAYER_CHANGE_PERCEIVED_HOSTILE_STATUS),
            vec![vec![player.addr(), actor.addr(), 1]]
        );
        assert_eq!(calls_to(&e, SET_CROSSHAIR_TARGET_TYPE), vec![vec![0]]);
        // A positive value is passed on with the "ranked" flag.
        assert_eq!(
            calls_to(&e, COMBAT_MANAGER_ADD_COMBATANT),
            vec![vec![0x7100, actor.addr(), player.addr(), 5, 1]]
        );
    }

    #[test]
    fn fn_0089fcf0_queues_the_held_item_again() {
        let mut e = engine();
        let (actor, process, target, _) = combat_setup(&mut e);
        let held = e.mem.alloc(0x40);
        let process_slot = slot_target(&e, process, PROCESS_SLOT_22C);
        e.register_double(process_slot, move |_, _| ret(held));
        stub(&mut e, PACKAGE_KIND_TEST, 1);
        let item_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_3BC);
        e.register_double(item_slot, |_, _| ret(0x7700));
        stub(&mut e, ENTRY_FORM, 0x7800);
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(calls_to(&e, item_slot), vec![vec![actor.addr(), 6]]);
        assert_eq!(
            calls_to(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![actor.addr(), 0x7800, 1, 0, 1, 0, 1]]
        );
        assert_eq!(calls_to(&e, ENTRY_DELETE), vec![vec![0x7700, 1]]);
        assert_eq!(
            calls_to(&e, ACTOR_END_INTERRUPT_PACKAGE),
            vec![vec![actor.addr(), 0]]
        );
    }

    #[test]
    fn fn_0089fcf0_adds_the_target_to_an_existing_controller() {
        let mut e = engine();
        let (actor, process, target, _) = combat_setup(&mut e);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0x9200));
        let process_slot = slot_target(&e, process, PROCESS_SLOT_22C);
        e.register_double(process_slot, |_, _| ret(0x9300));
        stub(&mut e, COMBAT_CONTROLLER_TARGET, 0x1111);
        fn_0089fcf0(&mut e, actor, target, 0x44, 0, 0, 0, 4, 0, 0);
        let float_zero = 0.0f32.to_bits();
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_ADD_TARGET),
            vec![vec![0x9300, target.addr(), 4, 1, float_zero, float_zero]]
        );
        assert_eq!(call_count(&e, COMBAT_MANAGER_ADD_GROUP_MEMBER), 0);

        // The same target again: only the group member is added.
        e.call_log = Some(vec![]);
        stub(&mut e, COMBAT_CONTROLLER_TARGET, target.addr());
        fn_0089fcf0(&mut e, actor, target, 0x44, 0, 0, 0, 4, 0, 0);
        assert_eq!(call_count(&e, COMBAT_CONTROLLER_ADD_TARGET), 0);
        assert_eq!(
            calls_to(&e, COMBAT_MANAGER_ADD_GROUP_MEMBER),
            vec![vec![0x7100, actor.addr(), 0x44]]
        );
    }

    #[test]
    fn fn_0089fcf0_refuses_in_the_cases_the_code_lists() {
        // Process level 5.
        let mut e = engine();
        let (actor, _, target, _) = combat_setup(&mut e);
        stub(&mut e, ACTOR_PROCESS_LEVEL_IS_5, 1);
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(call_count(&e, COMBAT_MANAGER_ADD_COMBATANT), 0);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 0);

        // The target is the actor itself.
        let mut e = engine();
        let (actor, _, _, _) = combat_setup(&mut e);
        fn_0089fcf0(&mut e, actor, actor, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 0);

        // Dead.
        let mut e = engine();
        let (actor, _, target, _) = combat_setup(&mut e);
        e.mem.set_u8(actor.addr() + 0x118, 1);
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 0);

        // Too far away: distance 20 against reach 10 plus a quarter.
        let mut e = engine();
        let (actor, _, target, _) = combat_setup(&mut e);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 20.0);
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(
            calls_to(&e, REFR_GET_DISTANCE_FROM_REFERENCE),
            vec![vec![actor.addr(), target.addr(), 0, 0]]
        );
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 0);
        // Distance 12.5 is within 10 + 2.5.
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 12.5);
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 1);

        // A busy state (3) without a furniture entry above 0x14 stops it.
        let mut e = engine();
        let (actor, _, target, acquire) = combat_setup(&mut e);
        let kind = slot_target(&e, actor.cast(), ACTOR_SLOT_214);
        e.register_double(kind, |_, _| ret(3));
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 0);
        // A furniture entry whose byte is 0x15 lets it go on, after slot 0x418.
        let entry = e.mem.alloc(0x20);
        e.mem.set_u8(entry + 0xe, 0x15);
        let furniture = slot_target(&e, acquire, PROCESS_SLOT_4D4);
        e.register_double(furniture, move |_, _| ret(entry));
        fn_0089fcf0(&mut e, actor, target, 0, 0, 0, 0, 1, 0, 0);
        let slot_418 = slot_target(&e, actor.cast(), ACTOR_SLOT_418);
        assert_eq!(call_count(&e, slot_418), 1);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 1);
    }

    #[test]
    fn fn_008a0270_reports_the_combat_flag() {
        let mut e = engine();
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let actor = actor_with(&mut e, &[], None);
        // No process: 0.
        e.mem.set_u8(actor.addr() + 0x104, 1);
        assert_eq!(fn_008a0270(&mut e, actor, 0), 0);
        let process = e.mem.alloc(8);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        assert_eq!(fn_008a0270(&mut e, actor, 0), 1);
        // Searching in combat is ignored on request.
        e.mem.set_u8(actor.addr() + 0x127, 1);
        assert_eq!(fn_008a0270(&mut e, actor, 0), 1);
        assert_eq!(fn_008a0270(&mut e, actor, 1), 0);
        // The player is never "in combat" here.
        e.set(player, Actor::pCurrentProcess, Ptr::new(process));
        e.mem.set_u8(player.addr() + 0x104, 1);
        assert_eq!(fn_008a0270(&mut e, player, 0), 0);
    }

    #[test]
    fn fn_008a02d0_returns_the_controller_of_type_0x12() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        assert_eq!(fn_008a02d0(&mut e, actor), 0);
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_22C, ret(0x9300))]);
        e.set(actor, Actor::pCurrentProcess, process);
        // Not in combat.
        assert_eq!(fn_008a02d0(&mut e, actor), 0);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        stub(&mut e, PACKAGE_TYPE, 0x11);
        assert_eq!(fn_008a02d0(&mut e, actor), 0);
        stub(&mut e, PACKAGE_TYPE, 0x12);
        assert_eq!(fn_008a02d0(&mut e, actor), 0x9300);
        let none = slot_target(&e, process, PROCESS_SLOT_22C);
        e.register_double(none, |_, _| ret(0));
        assert_eq!(fn_008a02d0(&mut e, actor), 0);
    }

    #[test]
    fn small_combat_controller_forwarders() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_428, ret(0))], None);
        assert_eq!(fn_008a0360(&mut e, actor), 0);
        fn_008a0680(&mut e, actor, 1, 2);
        assert_eq!(call_count(&e, CONTROLLER_FORWARD_97EFE0), 0);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0x9400));
        stub(&mut e, COMBAT_CONTROLLER_TARGET, 0x1234);
        assert_eq!(fn_008a0360(&mut e, actor), 0x1234);
        fn_008a0680(&mut e, actor, 1, 2);
        assert_eq!(
            calls_to(&e, CONTROLLER_FORWARD_97EFE0),
            vec![vec![0x9400, 1, 2]]
        );
    }

    #[test]
    fn fn_008a05f0_picks_a_process_kind_without_a_controller() {
        let mut e = engine();
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_54, ret(0))]);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_428, ret(0))], None);
        e.set(actor, Actor::pCurrentProcess, process);
        let slot = slot_target(&e, process, PROCESS_SLOT_54);
        for (field, kind) in [(1, 1), (2, 2), (3, 4), (0, 4)] {
            e.call_log = Some(vec![]);
            stub(&mut e, ACQUIRE_OBJECT_FIELD_28, field);
            fn_008a05f0(&mut e, actor);
            assert_eq!(
                calls_to(&e, slot),
                vec![vec![process.addr(), actor.addr(), kind]]
            );
        }
        // With a controller only the do-nothing function is called.
        e.call_log = Some(vec![]);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(1));
        fn_008a05f0(&mut e, actor);
        assert_eq!(call_count(&e, slot), 0);
        assert_eq!(calls_to(&e, ERROR_NO_OP), vec![vec![actor.addr()]]);
    }

    /// An actor with a combat controller for `fn_008a03a0`.
    fn keep_combat_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, u32) {
        double_global(e, ZERO_DOUBLE, 0.0);
        e.map(FADER_MANAGER, 4);
        e.set_global(FADER_MANAGER, 0x7200u32);
        let target = actor_with(
            e,
            &[(ACTOR_SLOT_22C, ret(0)), (ACTOR_SLOT_2E8, ret(0))],
            None,
        );
        let process = object(e, 0x800, &[(PROCESS_SLOT_22C, ret(0x9500))]);
        let acquire = object(
            e,
            0x10,
            &[(PROCESS_SLOT_64C, ret(0)), (PROCESS_SLOT_630, ret(0))],
        );
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_428, ret(0x9500)),
                (ACTOR_SLOT_STOP_COMBAT, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        stub(e, ACTOR_IN_COMBAT, 1);
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(e, COMBAT_CONTROLLER_TARGET, target.addr());
        stub(e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET, 1);
        (actor, acquire, target.addr())
    }

    #[test]
    fn fn_008a03a0_restarts_the_combat_controller() {
        let mut e = engine();
        let (actor, acquire, target) = keep_combat_setup(&mut e);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 0);
        fn_008a03a0(&mut e, actor);
        assert_eq!(
            calls_to(&e, FADER_MANAGER_GET_FADER_ALPHA),
            vec![vec![0x7200, 1]]
        );
        let slot_630 = slot_target(&e, acquire.cast(), PROCESS_SLOT_630);
        assert_eq!(calls_to(&e, slot_630), vec![vec![acquire.addr(), target]]);
        assert_eq!(calls_to(&e, CONTROLLER_START_HIGH), vec![vec![0x9500]]);
        assert_eq!(call_count(&e, CONTROLLER_START_OTHER), 0);
        assert_eq!(call_count(&e, COMBAT_START_GUARD_NO_OP), 2);

        // Another process level, and the controller test 9818b0.
        e.call_log = Some(vec![]);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 2);
        stub(&mut e, CONTROLLER_TEST_9818B0, 1);
        fn_008a03a0(&mut e, actor);
        let slot_64c = slot_target(&e, acquire.cast(), PROCESS_SLOT_64C);
        assert_eq!(calls_to(&e, slot_64c), vec![vec![acquire.addr()]]);
        assert_eq!(calls_to(&e, CONTROLLER_START_OTHER), vec![vec![0x9500]]);
    }

    #[test]
    fn fn_008a03a0_adds_the_actor_to_the_players_combat_list() {
        let mut e = engine();
        let (actor, _, _) = keep_combat_setup(&mut e);
        let player = actor_with(
            &mut e,
            &[(ACTOR_SLOT_22C, ret(0)), (ACTOR_SLOT_2E8, ret(0))],
            None,
        );
        set_player(&mut e, player);
        stub(&mut e, COMBAT_CONTROLLER_TARGET, player.addr());
        stub(&mut e, PLAYER_COMBAT_LIST_COUNT, 2);
        fn_008a03a0(&mut e, actor);
        assert_eq!(
            calls_to(&e, PLAYER_ADD_ACTOR_TO_COMBAT_LIST),
            vec![vec![player.addr(), actor.addr()]]
        );
        // The player's refusal stops everything.
        e.call_log = Some(vec![]);
        stub(&mut e, PLAYER_TEST_5A03F0, 1);
        fn_008a03a0(&mut e, actor);
        assert_eq!(
            call_count(&e, CONTROLLER_START_HIGH) + call_count(&e, CONTROLLER_START_OTHER),
            0
        );
    }

    #[test]
    fn fn_008a03a0_stops_the_combat_in_the_cases_the_code_lists() {
        // Visible fader: nothing.
        let mut e = engine();
        let (actor, _, _) = keep_combat_setup(&mut e);
        stub_float(&mut e, FADER_MANAGER_GET_FADER_ALPHA, 0.5);
        fn_008a03a0(&mut e, actor);
        assert_eq!(call_count(&e, ACTOR_TEST_576D30), 0);

        // The controller says to stop: slot 0x434 with 0.
        let mut e = engine();
        let (actor, _, _) = keep_combat_setup(&mut e);
        stub(&mut e, CONTROLLER_TEST_981450, 1);
        fn_008a03a0(&mut e, actor);
        let stop = slot_target(&e, actor.cast(), ACTOR_SLOT_STOP_COMBAT);
        assert_eq!(calls_to(&e, stop), vec![vec![actor.addr(), 0]]);
        assert_eq!(call_count(&e, CONTROLLER_START_HIGH), 0);

        // The target is no longer a combat target: stop with it, and give up
        // when the actor has no controller afterwards.
        let mut e = engine();
        let (actor, _, target) = keep_combat_setup(&mut e);
        stub(&mut e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET, 0);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0));
        fn_008a03a0(&mut e, actor);
        let stop = slot_target(&e, actor.cast(), ACTOR_SLOT_STOP_COMBAT);
        assert_eq!(calls_to(&e, stop), vec![vec![actor.addr(), target]]);
        assert_eq!(call_count(&e, CONTROLLER_START_HIGH), 0);

        // A protected target that is not "2e8": stop with it.
        let mut e = engine();
        let (actor, _, target) = keep_combat_setup(&mut e);
        let protected = slot_target(&e, Ptr::new(target), ACTOR_SLOT_22C);
        e.register_double(protected, |_, _| ret(1));
        fn_008a03a0(&mut e, actor);
        let stop = slot_target(&e, actor.cast(), ACTOR_SLOT_STOP_COMBAT);
        assert_eq!(calls_to(&e, stop), vec![vec![actor.addr(), target]]);
        // ... but a "2e8" target is not stopped by that alone.
        e.call_log = Some(vec![]);
        let flagged = slot_target(&e, Ptr::new(target), ACTOR_SLOT_2E8);
        e.register_double(flagged, |_, _| ret(1));
        fn_008a03a0(&mut e, actor);
        assert_eq!(call_count(&e, stop), 0);
        // The three other tests each stop it too.
        for test in [
            ACTOR_TEST_440DA0,
            ACTOR_TEST_440D80,
            ACTOR_PROCESS_LEVEL_IS_3,
        ] {
            e.call_log = Some(vec![]);
            stub(&mut e, test, 1);
            fn_008a03a0(&mut e, actor);
            assert_eq!(calls_to(&e, stop), vec![vec![actor.addr(), target]]);
            stub(&mut e, test, 0);
        }
    }

    #[test]
    fn actor_resurrect_brings_the_actor_back_with_new_process_data() {
        let mut e = engine();
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x7300u32);
        e.map(MODEL_LOADER, 4);
        e.set_global(MODEL_LOADER, 0x7400u32);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let old_process = object(&mut e, 0x800, &[(0, ret(0))]);
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_SET_DELETE, ret(0)),
                (ACTOR_SLOT_4C, ret(0)),
                (ACTOR_SLOT_1D0, ret(0)),
                (ACTOR_SLOT_208, ret(0)),
                (ACTOR_SLOT_240, ret(0)),
                (ACTOR_SLOT_248, ret(0)),
                (ACTOR_SLOT_24C, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, old_process);
        e.mem.set_u8(actor.addr() + 0x174, 1);
        e.mem.set_u8(actor.addr() + 0x118, 1);
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 2);
        stub_with(&mut e, OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub_with(&mut e, NEW_PROCESS_CONSTRUCT, |_, a| ret(a[0]));
        stub(&mut e, MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL, 2);
        actor_resurrect(&mut e, actor, 1, 0, 0);
        let slot = |e: &Engine, offset: u32| slot_target(e, actor.cast(), offset);
        assert_eq!(
            calls_to(&e, slot(&e, ACTOR_SLOT_SET_DELETE)),
            vec![vec![actor.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, slot(&e, ACTOR_SLOT_4C)),
            vec![vec![actor.addr(), 0x20000], vec![actor.addr(), 4]]
        );
        assert_eq!(e.mem.u8(actor.addr() + 0x174), 0);
        assert_eq!(e.mem.u8(actor.addr() + 0x118), 0);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, actor.addr(), 2]]
        );
        // The old process object is deleted, a new one built and installed.
        assert_eq!(
            calls_to(&e, slot_target(&e, old_process, 0)),
            vec![vec![old_process.addr(), 1]]
        );
        let new_process = e.get(actor, Actor::pCurrentProcess).addr();
        assert_ne!(new_process, old_process.addr());
        assert_eq!(calls_to(&e, NEW_PROCESS_CONSTRUCT), vec![vec![new_process]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0xb4]]);
        assert_eq!(
            calls_to(&e, MODIFIER_LIST_DELETE_ALL),
            vec![vec![actor.addr() + 0xd0], vec![actor.addr() + 0xe0]]
        );
        assert_eq!(
            calls_to(&e, slot(&e, ACTOR_SLOT_208)),
            vec![vec![actor.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_LIFE_STATE),
            vec![vec![actor.addr(), 0]]
        );
        // Desired level 2 uses slot 0x248; the lock is taken and released.
        assert_eq!(call_count(&e, slot(&e, ACTOR_SLOT_248)), 1);
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![RESURRECT_LOCK, 0]]);
        assert_eq!(calls_to(&e, LOCK_LEAVE), vec![vec![RESURRECT_LOCK]]);
        assert_eq!(
            calls_to(&e, EXTRA_REMOVE_PLAYER_CRIME_LIST_EXTRA),
            vec![vec![0, actor.addr()]]
        );
    }

    #[test]
    fn actor_resurrect_picks_the_process_level_switch() {
        for (level, slot) in [
            (0, ACTOR_SLOT_240),
            (1, ACTOR_SLOT_24C),
            (2, ACTOR_SLOT_248),
        ] {
            let mut e = engine();
            e.map(TES_SINGLETON, 4);
            let player = actor_with(
                &mut e,
                &[(ACTOR_SLOT_SET_DELETE, ret(0)), (ACTOR_SLOT_4C, ret(0))],
                None,
            );
            set_player(&mut e, player);
            let actor = actor_with(
                &mut e,
                &[
                    (ACTOR_SLOT_4C, ret(0)),
                    (ACTOR_SLOT_1D0, ret(0)),
                    (ACTOR_SLOT_240, ret(0)),
                    (ACTOR_SLOT_248, ret(0)),
                    (ACTOR_SLOT_24C, ret(0)),
                    (ACTOR_SLOT_SET_DELETE, ret(0)),
                ],
                None,
            );
            stub_with(&mut e, OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
            stub(&mut e, MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL, level);
            actor_resurrect(&mut e, actor, 0, 0, 0);
            let target = slot_target(&e, actor.cast(), slot);
            assert_eq!(call_count(&e, target), 1, "level {level}");
        }
        // Level 3 goes to the process lists; the player skips the switch.
        let mut e = engine();
        e.map(TES_SINGLETON, 4);
        let player = actor_with(
            &mut e,
            &[(ACTOR_SLOT_SET_DELETE, ret(0)), (ACTOR_SLOT_4C, ret(0))],
            None,
        );
        set_player(&mut e, player);
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_4C, ret(0)),
                (ACTOR_SLOT_1D0, ret(0)),
                (ACTOR_SLOT_SET_DELETE, ret(0)),
            ],
            None,
        );
        stub_with(&mut e, OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub(&mut e, MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL, 3);
        actor_resurrect(&mut e, actor, 0, 0, 0);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, actor.addr(), 3, 0, 0, 0]]
        );
        e.call_log = Some(vec![]);
        actor_resurrect(&mut e, player, 0, 0, 0);
        assert_eq!(call_count(&e, MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL), 0);
        assert_eq!(call_count(&e, PROCESS_LISTS_REMOVE_REFERENCE), 0);
    }

    #[test]
    fn actor_resurrect_queues_the_load_of_a_loaded_cell() {
        let mut e = engine();
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x7300u32);
        e.map(MODEL_LOADER, 4);
        e.set_global(MODEL_LOADER, 0x7400u32);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_4C, ret(0)),
                (ACTOR_SLOT_1D0, ret(0)),
                (ACTOR_SLOT_240, ret(0)),
                (ACTOR_SLOT_24C, ret(0)),
                (ACTOR_SLOT_SET_DELETE, ret(0)),
            ],
            None,
        );
        stub_with(&mut e, OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub(&mut e, REFR_PARENT_CELL, 0x7500);
        stub(&mut e, TES_IS_CELL_LOADED, 1);
        stub(&mut e, TES_GET_CELL_PRIORITY, 0x33);
        actor_resurrect(&mut e, actor, 0, 1, 0);
        assert_eq!(
            calls_to(&e, TES_IS_CELL_LOADED),
            vec![vec![0x7300, 0x7500, 0]]
        );
        assert_eq!(
            calls_to(&e, TES_GET_CELL_PRIORITY),
            vec![vec![0x7300, 0x7500, 0]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_REFERENCE),
            vec![vec![0x7400, actor.addr(), 0x33, 0]]
        );
        // The cell test picks slot 0x24c (false) here.
        let low = slot_target(&e, actor.cast(), ACTOR_SLOT_24C);
        assert_eq!(call_count(&e, low), 1);
        assert_eq!(call_count(&e, MOBILE_OBJECT_GET_DESIRED_PROCESS_LEVEL), 0);
        assert_eq!(call_count(&e, LOCK_LEAVE), 1);
    }

    #[test]
    fn actor_resurrect_with_heal_restores_health_and_havok() {
        let mut e = engine();
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_SET_DELETE, ret(0)),
                (ACTOR_SLOT_4C, ret(0)),
                (ACTOR_SLOT_1D0, ret(0x9900)),
                (ACTOR_SLOT_MODIFY_ACTOR_VALUE, ret(0)),
                (ACTOR_SLOT_INIT_HAVOK, ret(0)),
            ],
            None,
        );
        embedded_table(
            &mut e,
            actor.addr() + 0xa4,
            &[(0x0c, float_ret(30.5)), (0x00, ret(100))],
        );
        let process = object(
            &mut e,
            0x800,
            &[(PROCESS_SLOT_290, ret(0)), (PROCESS_SLOT_410, ret(0))],
        );
        e.set(actor, Actor::pCurrentProcess, process);
        stub_with(&mut e, NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        actor_resurrect(&mut e, actor, 0, 0, 1);
        assert_eq!(
            calls_to(&e, ACTOR_SET_LIFE_STATE),
            vec![vec![actor.addr(), 0]]
        );
        let modify = slot_target(&e, actor.cast(), ACTOR_SLOT_MODIFY_ACTOR_VALUE);
        assert_eq!(
            calls_to(&e, modify),
            vec![vec![actor.addr(), 0x10, 69.5f32.to_bits(), 0]]
        );
        let motion = calls_to(&e, BHK_WORLD_SET_MOTION);
        assert_eq!(motion, vec![vec![0x9900, 1, 1, 0, 1]]);
        let slot_290 = slot_target(&e, process, PROCESS_SLOT_290);
        assert_eq!(calls_to(&e, slot_290), vec![vec![process.addr(), 0]]);
        let slot_410 = slot_target(&e, process, PROCESS_SLOT_410);
        assert_eq!(calls_to(&e, slot_410), vec![vec![process.addr(), 3]]);
        // The branch for the dead: no modifier lists were cleared.
        assert_eq!(call_count(&e, MODIFIER_LIST_DELETE_ALL), 0);
        assert_eq!(call_count(&e, LOCK_ENTER), 0);
    }

    /// Makes `SETTING_GET_VALUE_POINTER` return a pointer to the `float`
    /// given for the setting address it is called on.
    fn float_settings(e: &mut Engine, values: &[(u32, f32)]) {
        let mut table = std::collections::HashMap::new();
        for (address, value) in values {
            let cell = e.mem.alloc(4);
            e.mem.set_f32(cell, *value);
            table.insert(*address, cell);
        }
        e.register_double(SETTING_GET_VALUE_POINTER, move |_, a| ret(table[&a[0]]));
    }

    /// Makes `SETTING_GET_INT_POINTER` return a pointer to the integer given
    /// for the setting address it is called on.
    fn int_settings(e: &mut Engine, values: &[(u32, u32)]) {
        let mut table = std::collections::HashMap::new();
        for (address, value) in values {
            let cell = e.mem.alloc(4);
            e.mem.set_u32(cell, *value);
            table.insert(*address, cell);
        }
        e.register_double(SETTING_GET_INT_POINTER, move |_, a| ret(table[&a[0]]));
    }

    /// An actor, a target and an acquire object for `fn_008987f0`.
    fn reaction_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr<Actor>, Ptr) {
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_22C, ret(0)),
                (ACTOR_SLOT_428, ret(0)),
                (ACTOR_SLOT_304, ret(0)),
                (ACTOR_SLOT_21C, ret(0)),
                (ACTOR_SLOT_460, ret(0)),
                (ACTOR_SLOT_474, ret(0)),
                (ACTOR_SLOT_48, ret(0)),
            ],
            None,
        );
        let target = actor_with(
            e,
            &[
                (ACTOR_SLOT_100, ret(1)),
                (ACTOR_SLOT_218, ret(0)),
                (ACTOR_SLOT_37C, ret(0)),
                (ACTOR_SLOT_42C, ret(0)),
                (ACTOR_SLOT_304, ret(0)),
            ],
            None,
        );
        let acquire = object(
            e,
            0x10,
            &[
                (PROCESS_SLOT_52C, ret(0)),
                (PROCESS_SLOT_504, ret(0)),
                (PROCESS_SLOT_F0, ret(0)),
                (PROCESS_SLOT_33C, ret(0)),
            ],
        );
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 1);
        float_settings(
            e,
            &[
                (DETECTION_THRESHOLD_SETTING, 10.0),
                (UNUSED_FLOAT_SETTING, 1.0),
                (SLOT_460_FLOAT_SETTING, 5.0),
            ],
        );
        e.map(COMBAT_DIALOGUE_MANAGER, 4);
        e.set_global(COMBAT_DIALOGUE_MANAGER, 0x7600u32);
        (actor, target, acquire)
    }

    #[test]
    fn fn_008987f0_returns_early_in_the_cases_the_code_lists() {
        let mut e = engine();
        let (actor, target, _) = reaction_setup(&mut e);
        // Protected.
        let protected = slot_target(&e, actor.cast(), ACTOR_SLOT_22C);
        e.register_double(protected, |_, _| ret(1));
        fn_008987f0(&mut e, actor, target, 0);
        assert_eq!(call_count(&e, ACTOR_TEST_8ACE90), 0);

        // The test 008ace90.
        let mut e = engine();
        let (actor, target, _) = reaction_setup(&mut e);
        stub(&mut e, ACTOR_TEST_8ACE90, 1);
        fn_008987f0(&mut e, actor, target, 0);
        assert_eq!(call_count(&e, ACTOR_GET_PACKAGE_SET_AS_PCURRENT), 0);

        // The target is the actor itself.
        let mut e = engine();
        let (actor, _, _) = reaction_setup(&mut e);
        fn_008987f0(&mut e, actor, actor, 0);
        assert_eq!(call_count(&e, ACTOR_GET_PACKAGE_SET_AS_PCURRENT), 0);

        // Process level 4 and the target is the acquire object's slot 0x52c.
        let mut e = engine();
        let (actor, target, acquire) = reaction_setup(&mut e);
        stub(&mut e, PROCESS_LEVEL_NUMBER, 4);
        let slot = slot_target(&e, acquire, PROCESS_SLOT_52C);
        let target_address = target.addr();
        e.register_double(slot, move |_, _| ret(target_address));
        fn_008987f0(&mut e, actor, target, 0);
        assert_eq!(call_count(&e, ACTOR_GET_CURRENT_WEAPON), 0);
    }

    #[test]
    fn fn_008987f0_records_the_detection_at_the_high_process_level() {
        let mut e = engine();
        let (actor, target, acquire) = reaction_setup(&mut e);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 0);
        // The detection level is 50 (above the setting 10): level 3; the
        // call writes 7 into its first flag byte and leaves the second at 1.
        stub_with(&mut e, ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR, |e, a| {
            e.mem.set_u8(a[3], 7);
            ret(50)
        });
        let record = e.mem.alloc(0x40);
        let creator = slot_target(&e, acquire, PROCESS_SLOT_F0);
        e.register_double(creator, move |_, _| ret(record));
        fn_008987f0(&mut e, actor, target, 0);
        assert_eq!(e.mem.u32(record + 8), 50);
        assert_eq!(e.mem.u32(record + 4), 3);
        assert_eq!(e.mem.u8(record + 0x1e), 7);
        assert_eq!(e.mem.u8(record + 0x1c), 1);
        assert_eq!(e.mem.u8(record + 0x1d), 1);
        let created = calls_to(&e, creator);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1..], [target.addr(), 3, 7, 50, 0, 1, 1]);
        let detection = calls_to(&e, ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR);
        assert_eq!(detection[0][..3], [actor.addr(), 0, target.addr()]);

        // An existing record is reused, with the level 0 for a low detection.
        e.call_log = Some(vec![]);
        stub(&mut e, ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR, 4);
        let finder = slot_target(&e, acquire, PROCESS_SLOT_504);
        let existing = e.mem.alloc(0x40);
        e.register_double(finder, move |_, _| ret(existing));
        fn_008987f0(&mut e, actor, target, 0);
        assert_eq!(call_count(&e, creator), 0);
        assert_eq!(e.mem.u32(existing + 8), 4);
        assert_eq!(e.mem.u32(existing + 4), 0);
    }

    #[test]
    fn fn_008987f0_starts_the_dialogue_for_the_player_on_a_friend_hit() {
        let mut e = engine();
        let (actor, _, _) = reaction_setup(&mut e);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        stub(&mut e, ACTOR_GET_FACTION_FIGHT_REACTION, 2);
        stub(&mut e, EXTRA_ADD_FRIEND_HIT, 1);
        int_settings(&mut e, &[(FRIEND_HIT_LIMIT_SETTINGS[3], 3)]);
        fn_008987f0(&mut e, actor, player, 0);
        assert_eq!(
            calls_to(&e, COMBAT_DIALOGUE_START_DIALOGUE),
            vec![vec![0x7600, actor.addr(), player.addr(), 2, 2, 1, 0]]
        );
        let flags = slot_target(&e, actor.cast(), ACTOR_SLOT_48);
        assert_eq!(calls_to(&e, flags), vec![vec![actor.addr(), 0x8000_0000]]);
        assert_eq!(call_count(&e, ACTOR_ATTACK_ALARM), 0);
        // The limit of 1000 or more resets the hits first.
        e.call_log = Some(vec![]);
        int_settings(&mut e, &[(FRIEND_HIT_LIMIT_SETTINGS[3], 1000)]);
        fn_008987f0(&mut e, actor, player, 0);
        assert_eq!(call_count(&e, EXTRA_RESET_FRIEND_HITS), 1);

        // Talking, dialogue and the skip test each stop the dialogue (and the
        // whole reaction).
        for blocker in [
            ACTOR_IS_TALKING,
            MOBILE_OBJECT_IS_IN_DIALOGUE,
            ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR,
        ] {
            e.call_log = Some(vec![]);
            stub(&mut e, blocker, 1);
            fn_008987f0(&mut e, actor, player, 0);
            assert_eq!(call_count(&e, COMBAT_DIALOGUE_START_DIALOGUE), 0);
            assert_eq!(call_count(&e, ACTOR_ATTACK_ALARM), 0);
            stub(&mut e, blocker, 0);
        }
        // The test 005a3790 ends it before any count.
        e.call_log = Some(vec![]);
        stub(&mut e, ACTOR_TEST_5A3790, 1);
        fn_008987f0(&mut e, actor, player, 0);
        assert_eq!(call_count(&e, EXTRA_ADD_FRIEND_HIT), 0);
    }

    #[test]
    fn fn_008987f0_raises_the_alarm_through_the_owner_for_the_player() {
        let mut e = engine();
        let (actor, _, _) = reaction_setup(&mut e);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        // A reaction other than 2 and 3: no friend-hit counting.
        stub(&mut e, ACTOR_GET_FACTION_FIGHT_REACTION, 0);
        stub(&mut e, REFR_GET_OWNER, 0x7700);
        stub(&mut e, FORM_TYPE, 8);
        stub(&mut e, PROCESS_LISTS_GET_OWNER_ACTOR, 0x7800);
        stub(&mut e, ACTOR_IN_COMBAT, 0);
        fn_008987f0(&mut e, actor, player, 0);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_GET_OWNER_ACTOR),
            vec![vec![PROCESS_LISTS, 0x7700, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_ATTACK_ALARM),
            vec![
                vec![0x7800, player.addr(), 0, 1],
                vec![actor.addr(), player.addr(), 0, 1]
            ]
        );
        assert_eq!(call_count(&e, EXTRA_ADD_FRIEND_HIT), 0);

        // Another owner type uses the other lookup.
        e.call_log = Some(vec![]);
        stub(&mut e, FORM_TYPE, 9);
        stub(&mut e, PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH, 0);
        fn_008987f0(&mut e, actor, player, 0);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH),
            vec![vec![PROCESS_LISTS, 0x7700, 0]]
        );
        assert_eq!(call_count(&e, ACTOR_ATTACK_ALARM), 1);
    }

    #[test]
    fn fn_008987f0_alarms_for_a_target_that_is_not_the_player() {
        let mut e = engine();
        let (actor, target, _) = reaction_setup(&mut e);
        let actor_304 = slot_target(&e, actor.cast(), ACTOR_SLOT_304);
        e.register_double(actor_304, |_, _| ret(1));
        for slot in [ACTOR_SLOT_218, ACTOR_SLOT_37C] {
            let target_slot = slot_target(&e, target.cast(), slot);
            e.register_double(target_slot, |_, _| ret(1));
        }
        fn_008987f0(&mut e, actor, target, 0);
        assert_eq!(
            calls_to(&e, ACTOR_ATTACK_ALARM),
            vec![vec![actor.addr(), target.addr(), 0, 1]]
        );
        // It returns before the controller work.
        assert_eq!(call_count(&e, CONTROLLER_WORKER_97F580), 0);
    }

    #[test]
    fn fn_008987f0_tells_the_acquire_object_when_the_target_relates_to_the_player() {
        let mut e = engine();
        let (actor, _, acquire) = reaction_setup(&mut e);
        let player = actor_with(
            &mut e,
            &[(ACTOR_SLOT_304, ret(0)), (ACTOR_SLOT_42C, ret(0))],
            None,
        );
        set_player(&mut e, player);
        stub(&mut e, ACTOR_TEST_8B0670, 1);
        // The reaction 0 with the byte 0 and "should attack" true.
        stub(&mut e, ACTOR_GET_FACTION_FIGHT_REACTION, 0);
        stub(&mut e, ACTOR_GET_SHOULD_ATTACK_ACTOR, 1);
        stub(&mut e, REFR_GET_OWNER, 0);
        fn_008987f0(&mut e, actor, player, 0x55);
        let slot = slot_target(&e, acquire, PROCESS_SLOT_33C);
        assert_eq!(
            calls_to(&e, slot),
            vec![vec![
                acquire.addr(),
                actor.addr(),
                player.addr(),
                1,
                1,
                0,
                0,
                0,
                1,
                0,
                0,
                1,
                0,
                0
            ]]
        );
        // No combat controller, not fleeing, not alarmed before (the player
        // branch above already alarmed through the first block).
        assert_eq!(call_count(&e, ACTOR_ATTACK_ALARM), 1);
        let reaction_args = calls_to(&e, ACTOR_GET_SHOULD_ATTACK_ACTOR);
        assert_eq!(reaction_args[0][..3], [actor.addr(), player.addr(), 0]);
        // No controller, not fleeing but already alarmed: slot 0x460 gets the
        // target and the setting 5.0.
        let slot_460 = slot_target(&e, actor.cast(), ACTOR_SLOT_460);
        assert_eq!(
            calls_to(&e, slot_460),
            vec![vec![actor.addr(), player.addr(), 5.0f32.to_bits()]]
        );
        // The actor's slot 0x474 runs with 1 (the actor is not the player).
        let slot_474 = slot_target(&e, actor.cast(), ACTOR_SLOT_474);
        assert_eq!(calls_to(&e, slot_474), vec![vec![actor.addr(), 1]]);
    }

    #[test]
    fn fn_008987f0_tells_the_acquire_object_without_alarm_for_other_targets() {
        let mut e = engine();
        let (actor, target, acquire) = reaction_setup(&mut e);
        // A non-player target that relates to the actor through slot 0x42c.
        let target_42c = slot_target(&e, target.cast(), ACTOR_SLOT_42C);
        let actor_address = actor.addr();
        e.register_double(target_42c, move |_, _| ret(actor_address));
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        stub(&mut e, ACTOR_TEST_8B0670, 1);
        stub(&mut e, ACTOR_IS_FLEEING, 0);
        // The target is not the player, so only the plain tell happens.
        fn_008987f0(&mut e, actor, target, 0);
        let slot = slot_target(&e, acquire, PROCESS_SLOT_33C);
        assert_eq!(call_count(&e, slot), 1);
        assert_eq!(call_count(&e, ACTOR_ATTACK_ALARM), 0);
        let slot_460 = slot_target(&e, actor.cast(), ACTOR_SLOT_460);
        assert_eq!(call_count(&e, slot_460), 0);
    }

    #[test]
    fn fn_008987f0_with_a_controller_that_has_the_target_only_passes_it_on() {
        let mut e = engine();
        let (actor, target, _) = reaction_setup(&mut e);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0x9700));
        stub(&mut e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET, 1);
        fn_008987f0(&mut e, actor, target, 0x66);
        assert_eq!(
            calls_to(&e, CONTROLLER_WORKER_97F580),
            vec![vec![0x9700, target.addr(), 0x66]]
        );
        assert_eq!(call_count(&e, ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR), 0);
        assert_eq!(call_count(&e, ACTOR_ATTACK_ALARM), 0);
    }

    /// An actor for `fn_00899200` with the slots the function uses and an
    /// acquire object without a weapon.
    fn hit_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr) {
        let process = object(
            e,
            0x800,
            &[
                (PROCESS_SLOT_CURRENT_WEAPON, ret(0)),
                (PROCESS_SLOT_ANIM_SEQUENCE, ret(0x7a00)),
                (PROCESS_SLOT_460, ret(0x7b00)),
            ],
        );
        let acquire = object(
            e,
            0x10,
            &[
                (PROCESS_SLOT_448, ret(0)),
                (PROCESS_SLOT_27C, ret(0)),
                (PROCESS_SLOT_274, ret(0)),
                (PROCESS_SLOT_CURRENT_WEAPON, ret(0)),
                (PROCESS_SLOT_ANIM_SEQUENCE, ret(0x7a00)),
            ],
        );
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_21C, ret(0)),
                (ACTOR_SLOT_360, ret(0)),
                (ACTOR_SLOT_428, ret(0)),
                (ACTOR_SLOT_1D0, ret(0x7c00)),
                (ACTOR_SLOT_REACH, float_ret(2.0)),
                (ACTOR_SLOT_POSITION, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        embedded_table(
            e,
            actor.addr() + ACTOR_MAGIC_CASTER_OFFSET,
            &[
                (MAGIC_CASTER_SLOT_40, ret(0)),
                (MAGIC_CASTER_SLOT_48, ret(0)),
            ],
        );
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(e, DEBUG_DRAW_LEVEL, 0);
        int_settings(e, &[(WORD_SETTING_3B4, 0x1234)]);
        float_settings(e, &[]);
        stub_float(e, REFR_GET_SCALE, 1.0);
        e.map(SPECIAL_IDLE_STATE_OBJECT, 0x10);
        e.map(VATS_ACTOR, 4);
        (actor, acquire)
    }

    #[test]
    fn fn_00899200_only_queues_when_the_task_queue_is_used() {
        let mut e = engine();
        let (actor, _) = hit_setup(&mut e);
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, GET_TES, 0x7d00);
        fn_00899200(&mut e, actor, 3, 4);
        assert_eq!(
            calls_to(&e, TES_QUEUE_ATTACK),
            vec![vec![0x7d00, actor.addr(), 3, 4]]
        );
        assert_eq!(call_count(&e, GET_SAVED_ACQUIRE_OBJECT), 0);
    }

    #[test]
    fn fn_00899200_updates_the_acquire_object_and_the_word() {
        let mut e = engine();
        let (actor, acquire) = hit_setup(&mut e);
        fn_00899200(&mut e, actor, 1, 0);
        let slot = slot_target(&e, acquire, PROCESS_SLOT_448);
        assert_eq!(calls_to(&e, slot), vec![vec![acquire.addr(), 1]]);
        // Slot 0x21c false: the setting is the word.
        assert_eq!(
            calls_to(&e, ACTOR_WORKER_8BC240),
            vec![vec![actor.addr(), 0x1234]]
        );
        // Slot 0x21c true: the base form's word.
        e.call_log = Some(vec![]);
        let slot_21c = slot_target(&e, actor.cast(), ACTOR_SLOT_21C);
        e.register_double(slot_21c, |_, _| ret(1));
        stub(&mut e, REFR_BASE_FORM, 0x7e00);
        stub(&mut e, FORM_GETTER_5FBEB0, 0x77);
        fn_00899200(&mut e, actor, 1, 0);
        assert_eq!(
            calls_to(&e, ACTOR_WORKER_8BC240),
            vec![vec![actor.addr(), 0x77]]
        );
        // With flag_b clear nothing else happens.
        assert_eq!(call_count(&e, ACTOR_COMBAT_HIT), 0);
        assert_eq!(call_count(&e, CONTROLLER_WORKER_97F7D0), 0);
    }

    #[test]
    fn fn_00899200_vats_uses_up_a_shot_or_casts_the_default_spell() {
        let mut e = engine();
        let (actor, _) = hit_setup(&mut e);
        e.set_global(VATS_ACTOR, actor.addr());
        stub_with(&mut e, ENTRY_FORM, |_, a| {
            ret(if a[0] == VATS_OBJECT { 4 } else { 0x7f00 })
        });
        let weapon_slot = slot_target(
            &e,
            e.get(actor, Actor::pCurrentProcess),
            PROCESS_SLOT_CURRENT_WEAPON,
        );
        e.register_double(weapon_slot, |_, _| ret(0x7f10));
        let action = e.mem.alloc(0x20);
        stub(&mut e, VATS_GET_CURRENT_ACTION, action);
        e.mem.set_u8(action + 8, 5);
        stub(&mut e, WEAPON_BYTE_FLAG, 1);
        fn_00899200(&mut e, actor, 0, 0);
        assert_eq!(e.mem.u8(action + 8), 4);

        // Without the weapon flag: the spell is cast through the caster.
        let reference = object(&mut e, 0x10, &[(0x234, ret(0)), (0x100, ret(1))]);
        e.mem.set_u8(action + 7, 1);
        e.mem.set_u32(action + 0xc, reference.addr());
        stub(&mut e, WEAPON_BYTE_FLAG, 0);
        stub(&mut e, GET_DEFAULT_OBJECT, 0x8100);
        stub(&mut e, RTTI_DYNAMIC_CAST, 0x8200);
        stub(&mut e, MAGIC_CASTER_RELEASE_CAST, 1);
        e.call_log = Some(vec![]);
        fn_00899200(&mut e, actor, 0, 0);
        assert_eq!(calls_to(&e, GET_DEFAULT_OBJECT), vec![vec![5]]);
        assert_eq!(
            calls_to(&e, RTTI_DYNAMIC_CAST),
            vec![vec![0x8100, 0, RTTI_TYPE_TES_FORM, RTTI_TYPE_MAGIC_ITEM, 0]]
        );
        let caster = actor.addr() + ACTOR_MAGIC_CASTER_OFFSET;
        let slot_40 = slot_target(&e, Ptr::new(caster), MAGIC_CASTER_SLOT_40);
        let slot_48 = slot_target(&e, Ptr::new(caster), MAGIC_CASTER_SLOT_48);
        assert_eq!(calls_to(&e, slot_40), vec![vec![caster, 0x8200]]);
        assert_eq!(
            calls_to(&e, slot_48),
            vec![vec![caster, reference.addr() + 0x94]]
        );
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_RELEASE_CAST),
            vec![vec![caster, 0]]
        );
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x15]]);
        // A target that is already blocked (slot 0x234) casts nothing.
        let blocked = slot_target(&e, reference, 0x234);
        e.register_double(blocked, |_, _| ret(1));
        e.call_log = Some(vec![]);
        fn_00899200(&mut e, actor, 0, 0);
        assert_eq!(call_count(&e, GET_DEFAULT_OBJECT), 0);
    }

    #[test]
    fn fn_00899200_draws_the_debug_arrow() {
        let mut e = engine();
        let (actor, _) = hit_setup(&mut e);
        stub(&mut e, DEBUG_DRAW_LEVEL, 2);
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x8300u32);
        e.map(ARROW_LENGTH_SCALE, 4);
        e.set_global(ARROW_LENGTH_SCALE, 800.0f32);
        e.map(ARROW_WIDTH_SCALE, 4);
        e.set_global(ARROW_WIDTH_SCALE, 0.1f32);
        // The colour constructor returns the address of what it built.
        stub_with(&mut e, COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        stub(&mut e, NODE_ROTATION, 0x8400);
        let vector = e.mem.alloc(12);
        e.mem.set_f32(vector, 1.0);
        stub(&mut e, MATRIX_TRANSFORM, vector);
        stub(&mut e, VECTOR_SUBTRACT, vector);
        stub(&mut e, VECTOR_SCALE, vector);
        stub(&mut e, ACTOR_GET_WEAPON_POSITION, vector);
        stub(&mut e, CREATE_DIR_ARROW, 0x8500);
        stub_float(&mut e, DEBUG_DRAW_DURATION_GETTER, 4.5);
        fn_00899200(&mut e, actor, 0, 0);
        // Red: the actor's slot 0x360 is false.
        let arrow = calls_to(&e, CREATE_DIR_ARROW);
        assert_eq!(arrow.len(), 1);
        assert_eq!(arrow[0][0], vector);
        let color = arrow[0][1];
        assert_eq!(
            [
                e.mem.f32(color),
                e.mem.f32(color + 4),
                e.mem.f32(color + 8),
                e.mem.f32(color + 12)
            ],
            [1.0, 0.0, 0.0, 1.0]
        );
        let scales = calls_to(&e, VECTOR_SCALE);
        assert_eq!(scales[0][2], 800.0f32.to_bits());
        assert_eq!(scales[1][2], 0.1f32.to_bits());
        assert_eq!(
            calls_to(&e, TES_ADD_TEMP_DEBUG_OBJECT),
            vec![vec![0x8300, 0x8500, 4.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, MATRIX_TRANSFORM).len(), 2);
        assert_eq!(calls_to(&e, VECTOR_NORMALIZE).len(), 1);
        assert_eq!(calls_to(&e, DEBUG_OBJECT_SET_POSITION)[0][0], 0x8500);

        // Blue when slot 0x360 is true.
        e.call_log = Some(vec![]);
        let slot_360 = slot_target(&e, actor.cast(), ACTOR_SLOT_360);
        e.register_double(slot_360, |_, _| ret(1));
        fn_00899200(&mut e, actor, 0, 0);
        let color = calls_to(&e, CREATE_DIR_ARROW)[0][1];
        assert_eq!([e.mem.f32(color + 4), e.mem.f32(color + 8)], [0.0, 1.0]);
    }

    #[test]
    fn fn_00899200_hits_the_target_the_acquire_object_names() {
        let mut e = engine();
        let (actor, acquire) = hit_setup(&mut e);
        let holder = slot_target(&e, acquire, PROCESS_SLOT_27C);
        e.register_double(holder, |_, _| ret(0x8600));
        let entry = slot_target(&e, acquire, PROCESS_SLOT_274);
        e.register_double(entry, |_, _| ret(0x8700));
        stub(&mut e, OBJECT_TYPE_FIELD, 0x2c);
        let target = actor_with(&mut e, &[(ACTOR_SLOT_100, ret(1))], None);
        let target_address = target.addr();
        stub_with(&mut e, NODE_NEXT, move |_, a| {
            ret(if a[0] == 0x8700 { target_address } else { 0 })
        });
        // The animation group 0x64: hand to hand kind 1.
        stub(&mut e, ANIMATION_ZERO_GLOBAL_TRANSFORM, 0x8800);
        stub(&mut e, ANIM_GROUP_GET_NUMBER, 0x64);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0x8900));
        fn_00899200(&mut e, actor, 9, 1);
        assert_eq!(
            calls_to(&e, ACTOR_COMBAT_HIT),
            vec![vec![actor.addr(), target.addr(), 9, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, CONTROLLER_WORKER_97F7D0),
            vec![vec![0x8900, target.addr()]]
        );

        // The group 0x3e gives kind 2; a weapon of another type gives 0.
        e.call_log = Some(vec![]);
        stub(&mut e, ANIM_GROUP_GET_NUMBER, 0x3e);
        fn_00899200(&mut e, actor, 9, 1);
        assert_eq!(calls_to(&e, ACTOR_COMBAT_HIT)[0][4], 2);
        e.call_log = Some(vec![]);
        let weapon_slot = slot_target(&e, acquire, PROCESS_SLOT_CURRENT_WEAPON);
        e.register_double(weapon_slot, |_, _| ret(0x8a00));
        stub(&mut e, ENTRY_FORM, 0x8b00);
        stub(&mut e, WEAPON_TYPE, 3);
        fn_00899200(&mut e, actor, 9, 1);
        assert_eq!(calls_to(&e, ACTOR_COMBAT_HIT)[0][4], 0);

        // The test 008ace90 on the target drops it (no hit).
        e.call_log = Some(vec![]);
        stub(&mut e, ACTOR_TEST_8ACE90, 1);
        fn_00899200(&mut e, actor, 9, 1);
        assert_eq!(call_count(&e, ACTOR_COMBAT_HIT), 0);
    }

    #[test]
    fn fn_00899200_plays_the_weapon_sound_and_casts_when_nothing_is_hit() {
        let mut e = engine();
        let (actor, acquire) = hit_setup(&mut e);
        let weapon_slot = slot_target(&e, acquire, PROCESS_SLOT_CURRENT_WEAPON);
        e.register_double(weapon_slot, |_, _| ret(0x8a00));
        stub(&mut e, ENTRY_FORM, 0x8b00);
        stub(&mut e, WEAPON_SOUND_SET, 0x8c00);
        stub(&mut e, SOUND_SET_FILE_NAME, 0x8d00);
        stub(&mut e, AUDIO_INSTANCE, 0x8e00);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.5);
        e.mem.set_f32(position + 4, 2.5);
        e.mem.set_f32(position + 8, 3.5);
        let position_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_POSITION);
        e.register_double(position_slot, move |_, _| ret(position));
        // The reach worker finds no target.
        stub(&mut e, ACTOR_REACH_WORKER, 0);
        // Spell part: the base form record names group 0xff.
        stub(&mut e, ACTOR_BASE_FORM, 0x9000);
        stub(&mut e, NODE_NEXT, 0x9100);
        stub(&mut e, RECORD_TEST_406090, 1);
        stub(&mut e, ACTOR_BASE_DATA_GET_FATIGUE, 0xff);
        fn_00899200(&mut e, actor, 0, 1);
        let play = calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(play.len(), 1);
        assert_eq!(play[0][0], 0x8e00);
        assert_eq!(play[0][2..], [0x8d00, 0x102, 0x8c00]);
        let handle = play[0][1];
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_POSITION_XYZ),
            vec![vec![
                handle,
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                3.5f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_OBJECT_TO_FOLLOW),
            vec![vec![handle, 0x7c00]]
        );
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![handle, 0]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTROY), vec![vec![handle]]);
        // The spell is cast: record at node + 0x18.
        let caster = actor.addr() + ACTOR_MAGIC_CASTER_OFFSET;
        let slot_40 = slot_target(&e, Ptr::new(caster), MAGIC_CASTER_SLOT_40);
        let slot_48 = slot_target(&e, Ptr::new(caster), MAGIC_CASTER_SLOT_48);
        assert_eq!(calls_to(&e, slot_40), vec![vec![caster, 0x9118]]);
        assert_eq!(
            calls_to(&e, slot_48),
            vec![vec![caster, actor.addr() + 0x94]]
        );
        assert_eq!(calls_to(&e, RECORD_TEST_406090), vec![vec![0x9118 + 0xc]]);
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_RELEASE_CAST),
            vec![vec![caster, 0]]
        );
        assert_eq!(call_count(&e, ACTOR_COMBAT_HIT), 0);

        // The record names a group the animation does not match: no cast.
        e.call_log = Some(vec![]);
        stub(&mut e, ACTOR_BASE_DATA_GET_FATIGUE, 0x20);
        stub(&mut e, ANIMATION_ZERO_GLOBAL_TRANSFORM, 0x9200);
        stub(&mut e, ANIM_GROUP_GET_NUMBER, 0x21);
        fn_00899200(&mut e, actor, 0, 1);
        assert_eq!(call_count(&e, slot_40), 0);
        // ... and one it does.
        stub(&mut e, ANIM_GROUP_GET_NUMBER, 0x20);
        fn_00899200(&mut e, actor, 0, 1);
        assert_eq!(call_count(&e, slot_40), 1);
    }

    #[test]
    fn fn_00899200_gives_up_for_an_actor_that_the_test_8ae660_stops() {
        let mut e = engine();
        let (actor, _) = hit_setup(&mut e);
        let slot_360 = slot_target(&e, actor.cast(), ACTOR_SLOT_360);
        e.register_double(slot_360, |_, _| ret(1));
        stub(&mut e, ACTOR_REACH_WORKER, 0);
        stub(&mut e, ACTOR_TEST_8AE660, 1);
        let controller_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0x8900));
        fn_00899200(&mut e, actor, 6, 1);
        assert_eq!(calls_to(&e, ACTOR_TEST_8AE660), vec![vec![actor.addr(), 6]]);
        assert_eq!(call_count(&e, CONTROLLER_WORKER_97F7D0), 0);
        assert_eq!(call_count(&e, SOUND_HANDLE_PLAY), 0);
    }

    /// What the `HitData` doubles do: the allocation is a plain block, the
    /// pointer holder stores and returns the pointer, and the initializer
    /// writes 12.5 as the health damage.
    fn hit_data_doubles(e: &mut Engine) {
        stub_with(e, OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub_with(e, HIT_DATA_CONSTRUCT, |_, a| ret(a[0]));
        stub_with(e, NI_POINTER_HIT_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        stub_with(e, NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        stub_with(e, HIT_DATA_INITIALIZE, |e, a| {
            e.mem.set_f32(a[0] + HIT_HEALTH_DAMAGE, 12.5);
            e.mem.set_f32(a[0] + HIT_TOTAL_DAMAGE, 20.0);
            e.mem.set_f32(a[0] + HIT_TARGETED_LIMB_DAMAGE, 8.0);
            e.mem.set_u32(a[0] + HIT_ATTACK_SKILL, 0x1b);
            ret(0)
        });
    }

    /// An attacker with a drawn weapon entry, a target and the `HitData`
    /// doubles. Returns (attacker, target, weapon entry).
    fn combat_hit_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr<Actor>, u32) {
        hit_data_doubles(e);
        let player = actor_with(e, &[], None);
        set_player(e, player);
        let process = object(
            e,
            0x800,
            &[
                (PROCESS_SLOT_CURRENT_WEAPON, ret(0)),
                (PROCESS_SLOT_580, ret(0)),
            ],
        );
        let weapon_entry = 0x8a00u32;
        let acquire = object(e, 0x10, &[(PROCESS_SLOT_CURRENT_WEAPON, ret(weapon_entry))]);
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        stub(e, ENTRY_FORM, 0x8b00);
        stub(e, WEAPON_TYPE, 6);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let slots = [
            (ACTOR_SLOT_428, ret(0)),
            (ACTOR_SLOT_360, ret(0)),
            (ACTOR_SLOT_3D8, ret(1)),
            (ACTOR_SLOT_32C, ret(5)),
            (ACTOR_SLOT_328, ret(0)),
            (ACTOR_SLOT_330, ret(0)),
            (ACTOR_SLOT_4C0, ret(0)),
            (ACTOR_SLOT_3C8, ret(0)),
            (ACTOR_SLOT_POSITION, ret(position)),
        ];
        let attacker = actor_with(e, &slots, None);
        e.set(attacker, Actor::pCurrentProcess, process);
        let target_process = object(e, 0x800, &[(PROCESS_SLOT_CURRENT_WEAPON, ret(0))]);
        let target = actor_with(e, &[(ACTOR_SLOT_POSITION, ret(position))], None);
        e.set(target, Actor::pCurrentProcess, target_process);
        e.map(IMPACT_BLOCK_DEFAULT_SOUND, 4);
        e.set_global(IMPACT_BLOCK_DEFAULT_SOUND, 0x4141u32);
        e.map(VATS_OBJECT, 0x10);
        (attacker, target, weapon_entry)
    }

    #[test]
    fn actor_combat_hit_gives_up_without_a_usable_target() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        actor_combat_hit(&mut e, attacker, Ptr::NULL, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, HIT_DATA_INITIALIZE), 0);
        stub(&mut e, ACTOR_TEST_8ACE90, 1);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, HIT_DATA_INITIALIZE), 0);
        stub(&mut e, ACTOR_TEST_8ACE90, 0);
        stub(&mut e, ACTOR_PROCESS_LEVEL_TEST_437B90, 1);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, HIT_DATA_INITIALIZE), 0);
        assert_eq!(call_count(&e, NI_POINTER_HIT_DATA_DESTRUCT), 0);
    }

    #[test]
    fn actor_combat_hit_plays_the_hit_sound_and_hits_the_target() {
        let mut e = engine();
        let (attacker, target, weapon_entry) = combat_hit_setup(&mut e);
        actor_combat_hit(&mut e, attacker, target, 3, Ptr::NULL, 2);
        let init = calls_to(&e, HIT_DATA_INITIALIZE);
        assert_eq!(init.len(), 1);
        assert_eq!(
            init[0][1..],
            [attacker.addr(), target.addr(), weapon_entry, 3, 0]
        );
        let hit_data = init[0][0];
        // The slot 0x3d8 got the weapon entry, the target, the source and an
        // out byte.
        let connects = calls_to(&e, slot_target(&e, attacker.cast(), ACTOR_SLOT_3D8));
        assert_eq!(
            connects[0][..4],
            [attacker.addr(), weapon_entry, target.addr(), 0]
        );
        // The counter is read, incremented and the attack skill passed on.
        let setter = slot_target(&e, attacker.cast(), ACTOR_SLOT_328);
        assert_eq!(calls_to(&e, setter), vec![vec![attacker.addr(), 6]]);
        let skill = slot_target(&e, attacker.cast(), ACTOR_SLOT_330);
        assert_eq!(calls_to(&e, skill), vec![vec![attacker.addr(), 0x1b]]);
        // Flag 1 of the hit data is not set: the plain hit sound.
        assert_eq!(
            calls_to(&e, IMPACT_MIXER_PLAY_WEAPON_HIT),
            vec![vec![
                attacker.addr(),
                12.5f32.to_bits(),
                0.0f32.to_bits(),
                target.addr(),
                6,
                0xffff_ffff,
                0xffff_ffff,
                0,
                1
            ]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_HIT_ME),
            vec![vec![target.addr(), hit_data, 2]]
        );
        assert_eq!(calls_to(&e, NI_POINTER_HIT_DATA_DESTRUCT).len(), 1);
    }

    #[test]
    fn actor_combat_hit_without_a_weapon_uses_the_unarmed_sound_type() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        let sound = calls_to(&e, IMPACT_MIXER_PLAY_WEAPON_HIT);
        assert_eq!(sound[0][4], 0xffff_ffff);
        assert_eq!(call_count(&e, ACTOR_HIT_ME), 1);
    }

    #[test]
    fn actor_combat_hit_plays_the_block_sounds() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        // Flag 1 set, flag 2 clear: the block sound with the "connects"
        // flag inverted, a zero form and the two positions.
        stub_with(&mut e, HIT_DATA_FLAG_TEST, |_, a| ret(u32::from(a[1] == 1)));
        stub_with(&mut e, HIT_DATA_INITIALIZE, |e, a| {
            e.mem.set_f32(a[0] + HIT_HEALTH_DAMAGE, 12.5);
            e.mem.set_f32(a[0] + HIT_PERCENT_BLOCKED, 0.25);
            ret(0)
        });
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        let one = 1.0f32.to_bits();
        let two = 2.0f32.to_bits();
        let three = 3.0f32.to_bits();
        assert_eq!(
            calls_to(&e, IMPACT_MIXER_PLAY_WEAPON_BLOCK),
            vec![vec![6, 0, one, two, three, one, two, three, 0]]
        );
        // Slot 0x4c0 got the damage, the blocked fraction, target, source.
        let report = slot_target(&e, attacker.cast(), ACTOR_SLOT_4C0);
        assert_eq!(
            calls_to(&e, report),
            vec![vec![
                attacker.addr(),
                12.5f32.to_bits(),
                0.25f32.to_bits(),
                target.addr(),
                0
            ]]
        );
        assert_eq!(call_count(&e, IMPACT_MIXER_PLAY_WEAPON_HIT), 0);

        // Both flags: the target's weapon form is used when it holds one,
        // the default sound word otherwise.
        e.call_log = Some(vec![]);
        stub(&mut e, HIT_DATA_FLAG_TEST, 1);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(
            calls_to(&e, IMPACT_MIXER_PLAY_WEAPON_BLOCK),
            vec![vec![6, 0x4141, one, two, three, one, two, three, 1]]
        );
        e.call_log = Some(vec![]);
        let target_weapon = slot_target(
            &e,
            Ptr::new(e.get(target, Actor::pCurrentProcess).addr()),
            PROCESS_SLOT_CURRENT_WEAPON,
        );
        e.register_double(target_weapon, |_, _| ret(0x8c00));
        stub(&mut e, ENTRY_FORM, 0x8d00);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(
            calls_to(&e, IMPACT_MIXER_PLAY_WEAPON_BLOCK),
            vec![vec![6, 0x8d00, one, two, three, one, two, three, 1]]
        );
    }

    #[test]
    fn actor_combat_hit_does_not_hit_when_the_controller_has_other_targets() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        let controller_slot = slot_target(&e, attacker.cast(), ACTOR_SLOT_428);
        e.register_double(controller_slot, |_, _| ret(0x9800));
        stub(&mut e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET, 0);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET),
            vec![vec![0x9800, target.addr()]]
        );
        assert_eq!(call_count(&e, ACTOR_HIT_ME), 0);
        assert_eq!(call_count(&e, NI_POINTER_HIT_DATA_DESTRUCT), 1);
        // When it does have the target, the hit goes ahead.
        e.call_log = Some(vec![]);
        stub(&mut e, COMBAT_CONTROLLER_IS_ACTOR_A_COMBAT_TARGET, 1);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, ACTOR_HIT_ME), 1);
    }

    #[test]
    fn actor_combat_hit_vats_merges_the_action_hit_data() {
        let mut e = engine();
        let (attacker, target, weapon_entry) = combat_hit_setup(&mut e);
        let slot_360 = slot_target(&e, attacker.cast(), ACTOR_SLOT_360);
        e.register_double(slot_360, |_, _| ret(1));
        stub_with(&mut e, ENTRY_FORM, |_, a| {
            ret(if a[0] == VATS_OBJECT { 4 } else { 0x8b00 })
        });
        let action = e.mem.alloc(0x20);
        let saved = e.mem.alloc(0x64);
        e.mem.set_u32(action + 0x14, saved);
        e.mem.set_u8(action + 4, 0);
        stub(&mut e, VATS_GET_CURRENT_ACTION, action);
        // The freshly initialized data carries a location, a direction and
        // damages, which are merged into the copy.
        stub_with(&mut e, HIT_DATA_INITIALIZE, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[0] + HIT_LOCATION + 4 * i, 1.0 + i as f32);
                e.mem.set_f32(a[0] + HIT_DIRECTION + 4 * i, 4.0 + i as f32);
            }
            e.mem.set_f32(a[0] + HIT_HEALTH_DAMAGE, 10.0);
            e.mem.set_f32(a[0] + HIT_TOTAL_DAMAGE, 11.0);
            e.mem.set_f32(a[0] + HIT_FATIGUE_DAMAGE, 12.0);
            e.mem.set_f32(a[0] + HIT_ARMOR_DAMAGE, 13.0);
            e.mem.set_f32(a[0] + HIT_TARGETED_LIMB_DAMAGE, 14.0);
            ret(0)
        });
        actor_combat_hit(&mut e, attacker, target, 3, Ptr::NULL, 1);
        let copy = calls_to(&e, HIT_DATA_COPY);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][1], saved);
        let hit_data = copy[0][0];
        let init = calls_to(&e, HIT_DATA_INITIALIZE);
        assert_eq!(init.len(), 1);
        assert_ne!(init[0][0], hit_data);
        assert_eq!(
            init[0][1..],
            [attacker.addr(), target.addr(), weapon_entry, 3, 0]
        );
        assert_eq!(e.mem.f32(hit_data + HIT_LOCATION + 8), 3.0);
        assert_eq!(e.mem.f32(hit_data + HIT_DIRECTION + 4), 5.0);
        assert_eq!(e.mem.f32(hit_data + HIT_HEALTH_DAMAGE), 10.0);
        assert_eq!(e.mem.f32(hit_data + HIT_FATIGUE_DAMAGE), 12.0);
        assert_eq!(e.mem.f32(hit_data + HIT_ARMOR_DAMAGE), 13.0);
        assert_eq!(e.mem.f32(hit_data + HIT_TARGETED_LIMB_DAMAGE), 14.0);
        assert_eq!(
            calls_to(&e, ACTOR_HIT_ME)[0][..2],
            [target.addr(), hit_data]
        );

        // With the action's byte at +4 set, the damages are not merged.
        e.call_log = Some(vec![]);
        e.mem.set_u8(action + 4, 1);
        e.mem.set_f32(saved + HIT_HEALTH_DAMAGE, 99.0);
        actor_combat_hit(&mut e, attacker, target, 3, Ptr::NULL, 1);
        assert_eq!(call_count(&e, ACTOR_HIT_ME), 1);
    }

    #[test]
    fn actor_combat_hit_vats_scales_the_damage_of_a_source_reference() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        let slot_360 = slot_target(&e, attacker.cast(), ACTOR_SLOT_360);
        e.register_double(slot_360, |_, _| ret(1));
        stub_with(&mut e, ENTRY_FORM, |_, a| {
            ret(if a[0] == VATS_OBJECT { 4 } else { 0x8b00 })
        });
        let action = e.mem.alloc(0x20);
        let saved = e.mem.alloc(0x64);
        e.mem.set_u32(action + 0x14, saved);
        e.mem.set_u8(action + 4, 1);
        stub(&mut e, VATS_GET_CURRENT_ACTION, action);
        let source = e.mem.alloc(0x10);
        // The source reference is not flagged; the weapon fires 4
        // projectiles; the base object test passes and the reduction is 1.
        stub(&mut e, REFR_FLAG_TEST, 0);
        stub(&mut e, ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, 1);
        stub(&mut e, SOURCE_WEAPON_LOOKUP, 0x9a00);
        stub(&mut e, WEAPON_GET_NUM_PROJECTILES, 4);
        stub(&mut e, ACTOR_BASE_FORM, 0x9b00);
        stub(&mut e, BASE_TEST_4FD3C0, 1);
        stub_float(&mut e, REFR_GETTER_885D70, 1.0);
        // The merge is skipped (action byte 1), so the damages the copy holds
        // are those of the saved data.
        e.mem.set_f32(saved + HIT_HEALTH_DAMAGE, 16.0);
        stub_with(&mut e, HIT_DATA_COPY, |e, a| {
            for offset in [
                HIT_HEALTH_DAMAGE,
                HIT_TOTAL_DAMAGE,
                HIT_TARGETED_LIMB_DAMAGE,
            ] {
                let value = e.mem.f32(a[1] + offset);
                e.mem.set_f32(a[0] + offset, value);
            }
            ret(0)
        });
        e.mem.set_f32(saved + HIT_TOTAL_DAMAGE, 20.0);
        e.mem.set_f32(saved + HIT_TARGETED_LIMB_DAMAGE, 8.0);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::new(source), 0);
        // The source reference is recorded and the count passed on.
        let lookup = calls_to(&e, SOURCE_WEAPON_LOOKUP);
        assert_eq!(lookup, vec![vec![source]]);
        let hit_me = calls_to(&e, ACTOR_HIT_ME);
        let hit_data = hit_me[0][1];
        assert_eq!(e.mem.u32(hit_data + HIT_SOURCE_REF), source);
        // 16 / 4 = 4 health, then times (1 - (4 - 1) / 4) = 0.25 -> 1.
        assert_eq!(e.mem.f32(hit_data + HIT_HEALTH_DAMAGE), 1.0);
        // 20 / 4 = 5 -> 1.25; 8 / 4 = 2 -> 0.5.
        assert_eq!(e.mem.f32(hit_data + HIT_TOTAL_DAMAGE), 1.25);
        assert_eq!(e.mem.f32(hit_data + HIT_TARGETED_LIMB_DAMAGE), 0.5);
    }

    #[test]
    fn actor_combat_hit_vats_clears_a_flagged_source_reference() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        let slot_360 = slot_target(&e, attacker.cast(), ACTOR_SLOT_360);
        e.register_double(slot_360, |_, _| ret(1));
        stub_with(&mut e, ENTRY_FORM, |_, a| {
            ret(if a[0] == VATS_OBJECT { 4 } else { 0x8b00 })
        });
        let action = e.mem.alloc(0x20);
        let saved = e.mem.alloc(0x64);
        e.mem.set_u32(action + 0x14, saved);
        e.mem.set_u8(action + 4, 1);
        stub(&mut e, VATS_GET_CURRENT_ACTION, action);
        let source = e.mem.alloc(0x10);
        stub(&mut e, REFR_FLAG_TEST, 1);
        stub_with(&mut e, HIT_DATA_COPY, |e, a| {
            e.mem.set_f32(a[0] + HIT_HEALTH_DAMAGE, 9.0);
            e.mem.set_u32(a[0] + HIT_CRITICAL_EFFECT, 0x77);
            ret(0)
        });
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::new(source), 0);
        let flag_test = calls_to(&e, REFR_FLAG_TEST);
        assert_eq!(flag_test, vec![vec![source, 0x20000]]);
        let hit_data = calls_to(&e, ACTOR_HIT_ME)[0][1];
        assert_eq!(e.mem.u32(hit_data + HIT_CRITICAL_EFFECT), 0);
        assert_eq!(e.mem.f32(hit_data + HIT_HEALTH_DAMAGE), 0.0);
        assert_eq!(
            calls_to(&e, HIT_DATA_CLEAR_EFFECT),
            vec![vec![hit_data, 0x3fc, 0]]
        );
        // The damage is zero, so the projectile split is skipped.
        assert_eq!(call_count(&e, SOURCE_WEAPON_LOOKUP), 0);
    }

    #[test]
    fn actor_combat_hit_casts_the_weapon_effect_past_its_threshold() {
        let mut e = engine();
        let (attacker, target, weapon_entry) = combat_hit_setup(&mut e);
        // The form's list holds a record at node + 0x18 whose embedded
        // object (record + 0xc) has a slot 8 reporting 9.0 against the
        // entry's threshold of 5.0.
        let node = e.mem.alloc(0x60);
        stub_with(&mut e, NODE_NEXT, move |_, a| {
            ret(if a[0] == 0x8b74 { node } else { 0 })
        });
        embedded_table(&mut e, node + 0x18 + 0xc, &[(8, float_ret(9.0))]);
        stub_float(&mut e, ITEM_CHANGE_GETTER_4BD350, 5.0);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        let process = e.get(attacker, Actor::pCurrentProcess);
        let slot_580 = slot_target(&e, process, PROCESS_SLOT_580);
        assert_eq!(calls_to(&e, slot_580), vec![vec![process.addr(), 1, 0, 0]]);
        let value_slot = e.mem.u32(e.mem.u32(node + 0x18 + 0xc) + 8);
        assert_eq!(
            calls_to(&e, value_slot),
            vec![vec![node + 0x18 + 0xc, attacker.addr()]]
        );
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_GETTER_4BD350),
            vec![vec![weapon_entry]]
        );

        // Below the threshold nothing happens.
        e.call_log = Some(vec![]);
        stub_float(&mut e, ITEM_CHANGE_GETTER_4BD350, 9.0);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, slot_580), 0);

        // Without a form record the entry's poison is used (record + 0x30).
        e.call_log = Some(vec![]);
        stub(&mut e, NODE_NEXT, 0);
        let poison = e.mem.alloc(0x60);
        embedded_table(&mut e, poison + 0x30 + 0xc, &[(8, float_ret(9.0))]);
        stub(&mut e, ITEM_CHANGE_GET_POISON, poison);
        stub_float(&mut e, ITEM_CHANGE_GETTER_4BD350, 5.0);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, slot_580), 1);
        // A hit that does not connect (slot 0x3d8 false) casts nothing.
        e.call_log = Some(vec![]);
        let connects = slot_target(&e, attacker.cast(), ACTOR_SLOT_3D8);
        e.register_double(connects, |_, _| ret(0));
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, slot_580), 0);
    }

    #[test]
    fn actor_combat_hit_wears_the_weapon_and_clears_use_weapon_damage() {
        let mut e = engine();
        let (attacker, target, weapon_entry) = combat_hit_setup(&mut e);
        let slot_360 = slot_target(&e, attacker.cast(), ACTOR_SLOT_360);
        e.register_double(slot_360, |_, _| ret(1));
        stub_with(&mut e, ENTRY_FORM, |_, a| {
            ret(if a[0] == VATS_OBJECT { 0 } else { 0x8b00 })
        });
        stub_with(&mut e, HIT_DATA_INITIALIZE, |e, a| {
            e.mem.set_f32(a[0] + HIT_HEALTH_DAMAGE, 12.5);
            e.mem.set_f32(a[0] + HIT_DAMAGE_TO_WEAPON, 1.5);
            ret(0)
        });
        stub(&mut e, GET_CURRENT_PACKAGE, 0x9c00);
        stub(&mut e, PACKAGE_GET_USE_WEAPON_PACKAGE_DATA, 0x9d00);
        stub(&mut e, USE_WEAPON_DATA_TEST, 1);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        let wear = slot_target(&e, attacker.cast(), ACTOR_SLOT_3C8);
        assert_eq!(
            calls_to(&e, wear),
            vec![vec![attacker.addr(), weapon_entry, 1.5f32.to_bits(), 0]]
        );
        // The use-weapon package data zeroes the damage before the hit.
        let hit_data = calls_to(&e, ACTOR_HIT_ME)[0][1];
        assert_eq!(e.mem.f32(hit_data + HIT_HEALTH_DAMAGE), 0.0);
        assert_eq!(
            calls_to(&e, PACKAGE_GET_USE_WEAPON_PACKAGE_DATA),
            vec![vec![0x9c00]]
        );

        // A weapon form with the flag bit set is not worn.
        e.call_log = Some(vec![]);
        stub(&mut e, FORM_FLAG_100_BIT_5, 1);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        assert_eq!(call_count(&e, wear), 0);
    }

    /// Allocates a zero cell per setting address and makes the three
    /// setting-pointer functions return the cell of the address they are
    /// called on. Returns the address-to-cell table.
    fn setting_cells(e: &mut Engine, addresses: &[u32]) -> std::collections::HashMap<u32, u32> {
        let mut table = std::collections::HashMap::new();
        for address in addresses {
            table.insert(*address, e.mem.alloc(8));
        }
        for function in [
            SETTING_GET_VALUE_POINTER,
            SETTING_GET_INT_POINTER,
            SETTING_GET_BOOL_POINTER,
        ] {
            let copy = table.clone();
            e.register_double(function, move |_, a| ret(copy[&a[0]]));
        }
        table
    }

    /// An actor of `0x1000` bytes with the given virtual slots (the
    /// player's `+0xe2d` byte is read by `Kill`).
    fn big_actor(e: &mut Engine, slots: &[(u32, Ret)]) -> Ptr<Actor> {
        object(e, 0x1000, slots).cast::<Actor>()
    }

    /// The slots a killed NPC answers.
    fn npc_slots() -> Vec<(u32, Ret)> {
        vec![
            (ACTOR_SLOT_22C, ret(0)),
            (ACTOR_SLOT_1A0, ret(0)),
            (ACTOR_SLOT_360, ret(0)),
            (ACTOR_SLOT_2E8, ret(0)),
            (ACTOR_SLOT_218, ret(1)),
            (ACTOR_SLOT_21C, ret(0)),
            (ACTOR_SLOT_STOP_COMBAT, ret(0)),
            (ACTOR_SLOT_234, ret(0)),
            (ACTOR_SLOT_428, ret(0)),
            (ACTOR_SLOT_1D0, ret(0)),
            (ACTOR_SLOT_48, ret(0)),
            (ACTOR_SLOT_2A0, ret(0)),
            (ACTOR_SLOT_448, ret(0)),
            (ACTOR_SLOT_38C, ret(0)),
            (ACTOR_SLOT_MODIFY_ACTOR_VALUE, ret(0)),
            (ACTOR_SLOT_GET_HEADING, float_ret(0.0)),
            (PLAYER_SLOT_488, ret(0)),
        ]
    }

    /// The slots a process object answers during `Kill`.
    fn process_slots() -> Vec<(u32, Ret)> {
        vec![
            (PROCESS_SLOT_1C4, ret(0)),
            (PROCESS_SLOT_6C4, ret(0)),
            (PROCESS_SLOT_180, ret(0)),
            (PROCESS_SLOT_310, ret(0)),
            (PROCESS_SLOT_4C8, ret(0)),
            (PROCESS_SLOT_4D0, ret(0)),
            (PROCESS_SLOT_660, ret(0)),
            (PROCESS_SLOT_28, ret(0)),
            (PROCESS_SLOT_6A0, ret(0)),
            (PROCESS_SLOT_4B4, ret(0)),
            (PROCESS_SLOT_40C, ret(0)),
            (PROCESS_SLOT_CURRENT_WEAPON, ret(0)),
            (PROCESS_SLOT_52C, ret(0)),
            (PROCESS_SLOT_730, float_ret(0.0)),
            (PROCESS_SLOT_3F4, ret(0)),
            (PROCESS_SLOT_614, ret(0)),
            (PROCESS_SLOT_E8, ret(0)),
            (PROCESS_SLOT_2FC, ret(0)),
            (PROCESS_SLOT_534, ret(0)),
        ]
    }

    /// A victim with a process, a player and the settings, ready for
    /// `actor_kill`. Returns (victim, process, player, setting cells).
    fn kill_setup(
        e: &mut Engine,
    ) -> (
        Ptr<Actor>,
        Ptr,
        Ptr<Actor>,
        std::collections::HashMap<u32, u32>,
    ) {
        let cells = setting_cells(
            e,
            &[
                ESSENTIAL_SETTING,
                DEATH_REPORT_SETTING,
                LOW_LEVEL_HEALTH_SETTING,
                ESSENTIAL_DOWN_TIMER_SETTING,
                KARMA_REWARD_SETTING,
                KARMA_REWARD_CREATURE_SETTING,
                KARMA_REWARD_ALIGNMENT_SETTINGS[0],
                KARMA_REWARD_ALIGNMENT_SETTINGS[2],
                KARMA_REWARD_ALIGNMENT_SETTINGS[3],
                KARMA_REWARD_ALIGNMENT_SETTINGS[4],
                WEAPON_NUMBER_SETTING,
                DEATH_VOICE_DISTANCE_SETTING,
                GREETING_TIMER_SETTING,
                DEATH_IMPULSE_SETTING,
                COMBATANT_COUNT_SETTING,
                DROP_WEAPON_CHANCE_SETTING,
                SLOW_MO_SETTING,
            ],
        );
        let process = object(e, 0x800, &process_slots());
        let player = big_actor(e, &npc_slots());
        e.mem.set_u32(player.addr() + 0xa4, 0);
        set_player(e, player);
        let victim = big_actor(e, &npc_slots());
        e.set(victim, Actor::pCurrentProcess, process);
        embedded_table(
            e,
            victim.addr() + ACTOR_VALUE_OWNER_OFFSET,
            &[(0x00, ret(100)), (0x0c, float_ret(30.0))],
        );
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x8800u32);
        e.map(COMBAT_MANAGER, 4);
        e.set_global(COMBAT_MANAGER, 0x8900u32);
        e.map(VATS_OBJECT, 0x10);
        e.map(MAGIC_SHADER_HOLDER, 4);
        let shader_owner = e.mem.alloc(0x700);
        e.set_global(MAGIC_SHADER_HOLDER, shader_owner);
        e.map(COMBAT_DIALOGUE_MANAGER, 4);
        let karma_base = e.mem.alloc(0x200);
        embedded_table(e, karma_base + 0x100, &[(0x0c, float_ret(0.0))]);
        stub(e, ACTOR_BASE_FORM, karma_base);
        e.map(0x011f_158c, 4);
        stub(e, GET_SAVED_ACQUIRE_OBJECT, process.addr());
        stub_with(e, OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub(e, ACTOR_GET_ANIM_GROUP_ADDRESS, 0xff);
        (victim, process, player, cells)
    }

    #[test]
    fn actor_kill_does_nothing_for_protected_actors() {
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        let slot = slot_target(&e, victim.cast(), ACTOR_SLOT_22C);
        e.register_double(slot, |_, _| ret(1));
        actor_kill(&mut e, victim, Ptr::NULL, 3.0);
        assert_eq!(call_count(&e, SOUND_HANDLE_CONSTRUCT), 0);

        // Slot 0x1a0 stops everything but the player.
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        let slot = slot_target(&e, victim.cast(), ACTOR_SLOT_1A0);
        e.register_double(slot, |_, _| ret(1));
        actor_kill(&mut e, victim, Ptr::NULL, 3.0);
        assert_eq!(call_count(&e, SOUND_HANDLE_CONSTRUCT), 0);
    }

    #[test]
    fn actor_kill_forgets_the_sounds_and_queues_the_kill() {
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        let attacker = big_actor(&mut e, &npc_slots());
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, GET_TES, 0x8a00);
        stub(&mut e, SOUND_HANDLE_IS_PLAYING, 1);
        actor_kill(&mut e, victim, attacker, 7.5);
        let constructed = calls_to(&e, SOUND_HANDLE_CONSTRUCT);
        assert_eq!(constructed.len(), 3);
        let handles = [constructed[0][0], constructed[1][0], constructed[2][0]];
        // 0089d900 keeps them at EBP-0x24, EBP-0x30 and EBP-0x3c.
        assert_eq!(handles[1], handles[0] - 12);
        assert_eq!(handles[2], handles[0] - 24);
        // The extra data list is `this + 0x44` (accessor `005d43c0` is a
        // double returning 0 here).
        assert_eq!(
            calls_to(&e, EXTRA_GET_WEAPON_ATTACK_SOUND),
            vec![vec![0, handles[0]]]
        );
        assert_eq!(
            calls_to(&e, EXTRA_SET_WEAPON_IDLE_SOUND),
            vec![vec![0, handles[1]]]
        );
        // The attack and idle sounds fade out while they play; the awake
        // sound is stopped and released.
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_FADE_OUT_AND_RELEASE),
            vec![vec![handles[0], 500], vec![handles[1], 500]]
        );
        assert_eq!(calls_to(&e, SOUND_HANDLE_STOP), vec![vec![handles[2]]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_RELEASE), vec![vec![handles[2]]]);
        // The kill is queued with the damage in a block and the killer
        // remembered.
        let queued = calls_to(&e, QUEUE_ACTOR_KILL);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][..2], [0x8a00, victim.addr()]);
        assert_eq!(queued[0][3], 0);
        assert_eq!(e.mem.f32(queued[0][2]), 7.5);
        assert_eq!(e.get(victim, Actor::pMyKiller).addr(), attacker.addr());
        // The handles are destroyed in reverse order and nothing else runs.
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTROY),
            vec![vec![handles[2]], vec![handles[1]], vec![handles[0]]]
        );
        assert_eq!(call_count(&e, ACTOR_SET_LIFE_STATE), 0);

        // A sound that is not playing is only released.
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, GET_TES, 0x8a00);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        assert_eq!(call_count(&e, SOUND_HANDLE_FADE_OUT_AND_RELEASE), 0);
        assert_eq!(call_count(&e, SOUND_HANDLE_RELEASE), 3);
    }

    #[test]
    fn actor_kill_without_a_process_finishes_with_the_death_stuff() {
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // 0089d900's aligned frame: the awake, idle and attack handles at EBP-0x3c, -0x30, -0x24.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        let handles: Vec<u32> = calls_to(&e, SOUND_HANDLE_DESTROY)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(handles, vec![ebp - 0x3c, ebp - 0x30, ebp - 0x24]);
        assert_eq!(
            calls_to(&e, ACTOR_SET_LIFE_STATE),
            vec![vec![victim.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_HAVOK_WEAPON),
            vec![vec![victim.addr()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_DO_DEATH_STUFF),
            vec![vec![victim.addr()]]
        );
        assert_eq!(calls_to(&e, REFR_RUN_SCRIPT), vec![vec![victim.addr()]]);
        assert_eq!(e.mem.u8(victim.addr() + 0x118), 1);
        assert_eq!(e.mem.u8(victim.addr() + 0xc4), 0);
        assert_eq!(
            calls_to(&e, MOBILE_OBJECT_STOP_CURRENT_DIALOGUE),
            vec![vec![victim.addr()]]
        );
        let stop = slot_target(&e, victim.cast(), ACTOR_SLOT_STOP_COMBAT);
        assert_eq!(calls_to(&e, stop), vec![vec![victim.addr(), 0]]);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_WORKER_96E2F0),
            vec![vec![PROCESS_LISTS, victim.addr()]]
        );
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTROY).len(), 3);
        // TES learns about the original base form.
        assert_eq!(calls_to(&e, TES_WORKER_459060).len(), 1);
    }

    #[test]
    fn actor_kill_clears_collision_and_ragdoll_state() {
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        let fixed = slot_target(&e, victim.cast(), ACTOR_SLOT_234);
        e.register_double(fixed, |_, _| ret(1));
        let collision = slot_target(&e, victim.cast(), ACTOR_SLOT_1D0);
        e.register_double(collision, |_, _| ret(0x9000));
        e.mem.set_u32(victim.addr() + 0xac, 0x9100);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        assert_eq!(
            calls_to(&e, BHK_RIGID_BODY_SET_FIXED),
            vec![vec![0x9000, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, BHK_RAGDOLL_DISABLE_RAGDOLL_ANIM),
            vec![vec![0x9100, 1]]
        );
    }

    #[test]
    fn actor_kill_by_the_player_records_the_statistics_and_karma() {
        let mut e = engine();
        let (victim, process, player, cells) = kill_setup(&mut e);
        // The player's side: its slot 0x360 is true, it can be killed...
        let player_360 = slot_target(&e, player.cast(), ACTOR_SLOT_360);
        e.register_double(player_360, |_, _| ret(1));
        e.mem
            .set_f32(cells[&KARMA_REWARD_ALIGNMENT_SETTINGS[0]], 5.0);
        stub(&mut e, GET_ALIGNMENT_FOR_KARMA, 0);
        stub(&mut e, ACTOR_HAS_FACTION_THAT_CARES_ABOUT_CRIME, 1);
        stub(&mut e, PLAYER_TEST_4997B0, 1);
        stub(&mut e, REFR_BASE_FORM, 0x9300);
        let _ = process;
        let karma_owner = e.mem.alloc(0x200);
        embedded_table(&mut e, karma_owner + 0x100, &[(0x0c, float_ret(10.0))]);
        stub(&mut e, ACTOR_BASE_FORM, karma_owner);
        stub_with(&mut e, FTOL, |_, a| {
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        stub(&mut e, CREATURE_GETTER_59F3A0, 0);
        stub(&mut e, RTTI_DYNAMIC_CAST, 0x9400);
        actor_kill(&mut e, victim, player, 1.0);

        assert_eq!(
            calls_to(&e, HANDLE_ENTRY_POINT),
            vec![vec![0x15, player.addr(), victim.addr(), victim.addr()]]
        );
        // The three statistics calls: weapon type 0 + 1, slot 0x2d without
        // a weapon.
        let stats = calls_to(&e, STATISTICS_WORKER_5F5950);
        assert_eq!(stats.len(), 3);
        assert_eq!(stats[0], vec![1, 1, 0x9300, 0, 0, 0x2d, 1]);
        assert_eq!(stats[1], vec![0, 1, 0x9300, 0, 0, 0x2d, 1]);
        assert_eq!(stats[2], vec![2, 1, 0, 0, 0, 0, 1]);
        // The kill counts (NPC kill, overall) and the murderer mark.
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![2], vec![0x23]]);
        assert_eq!(
            calls_to(&e, PLAYER_SET_IS_A_MURDERER),
            vec![vec![player.addr()]]
        );
        // Karma: alignment 0 uses its own setting; the amount is truncated.
        assert_eq!(
            calls_to(&e, GET_ALIGNMENT_FOR_KARMA),
            vec![vec![10.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_REWARD_KARMA),
            vec![vec![player.addr(), 5]]
        );
        // The player is not detected and holds no weapon: the murder mark
        // is cleared again.
        assert_eq!(e.mem.u8(victim.addr() + 0xc4), 0);
        assert_eq!(e.get(victim, Actor::pMyKiller).addr(), player.addr());
    }

    #[test]
    fn actor_kill_by_the_player_gives_experience() {
        let mut e = engine();
        let (victim, process, player, _) = kill_setup(&mut e);
        // The tracked damage is 12.0 against a base health of 100.
        let tracked = slot_target(&e, process, PROCESS_SLOT_730);
        e.register_double(tracked, |_, _| float_ret(12.0));
        stub(&mut e, TRACKED_DAMAGE_TEST, 1);
        stub(&mut e, ACTOR_GETTER_87F9F0, 0x77);
        stub(&mut e, GET_EXPERIENCE_POINTS, 50);
        stub(&mut e, PLAYER_INT_5BE4D0, 2);
        stub_float(&mut e, SCALE_EXPERIENCE, 100.0);
        stub_float(&mut e, FLOAT_ROUND, 100.0);
        stub_with(&mut e, FTOL, |_, a| {
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        let player_488 = slot_target(&e, player.cast(), PLAYER_SLOT_488);
        e.register_double(player_488, |_, _| ret(0));
        actor_kill(&mut e, victim, player, 1.0);
        assert_eq!(
            calls_to(&e, TRACKED_DAMAGE_TEST),
            vec![vec![12.0f32.to_bits(), 100.0f32.to_bits()]]
        );
        // Experience points for a non-creature (flag 1) and the word.
        assert_eq!(calls_to(&e, GET_EXPERIENCE_POINTS), vec![vec![1, 0x77]]);
        assert_eq!(
            calls_to(&e, SCALE_EXPERIENCE),
            vec![vec![50.0f32.to_bits(), 2]]
        );
        assert_eq!(calls_to(&e, player_488), vec![vec![player.addr(), 100]]);
        // A teammate-owned victim (process level flag) only counts when the
        // attacker's slot 0x360 is true.
        e.call_log = Some(vec![]);
        stub(&mut e, PROCESS_GET_FORCE_NEXT_UPDATE, 1);
        actor_kill(&mut e, victim, player, 1.0);
        assert_eq!(call_count(&e, GET_EXPERIENCE_POINTS), 0);
    }

    #[test]
    fn actor_kill_of_an_essential_actor_at_a_low_process_level() {
        let mut e = engine();
        let (victim, _, _, cells) = kill_setup(&mut e);
        e.mem.set_u8(cells[&ESSENTIAL_SETTING], 1);
        e.mem.set_f32(cells[&LOW_LEVEL_HEALTH_SETTING], 0.5);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 1);
        stub(&mut e, ACTOR_PROCESS_LEVEL_IS_3, 1);
        e.mem.set_u32(victim.addr() + 0xb0, 0x9500);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // 0.5 * 100 - 30 = 20.
        let modify = slot_target(&e, victim.cast(), ACTOR_SLOT_MODIFY_ACTOR_VALUE);
        assert_eq!(
            calls_to(&e, modify),
            vec![vec![victim.addr(), 0x10, 20.0f32.to_bits(), 0]]
        );
        assert_eq!(call_count(&e, SOUND_HANDLE_DESTROY), 3);
        // The common tail does not run.
        assert_eq!(call_count(&e, BHK_RAGDOLL_PENETRATION_UTIL_CLEAR), 0);
        assert_eq!(call_count(&e, ACTOR_SET_LIFE_STATE), 0);
    }

    #[test]
    fn actor_kill_of_an_essential_actor_puts_it_into_life_state_six() {
        let mut e = engine();
        let (victim, process, _, cells) = kill_setup(&mut e);
        e.mem.set_u8(cells[&ESSENTIAL_SETTING], 1);
        e.mem.set_f32(cells[&ESSENTIAL_DOWN_TIMER_SETTING], 3.0);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 1);
        stub(&mut e, ACTOR_GET_CURRENT_EDITOR_PACKAGE, 0x9600);
        stub(&mut e, PACKAGE_TYPE, 5);
        stub(&mut e, PROCESS_LEVEL_NUMBER, 6);
        stub(&mut e, SETTING_GET_STRING, 0x9700);
        stub(&mut e, REFR_GET_NAME, 0x9800);
        e.map(TWENTY, 4);
        e.set_global(TWENTY, 20.0f32);
        e.map(MESSAGE_DURATION, 4);
        e.set_global(MESSAGE_DURATION, 2.0f32);
        e.mem.set_u32(victim.addr() + 0xb0, 0);
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // The message buffer at EBP-0x170.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(calls_to(&e, SHOW_MESSAGE)[0][0], ebp - 0x170);
        let down_timer = slot_target(&e, process, PROCESS_SLOT_E8);
        assert_eq!(
            calls_to(&e, down_timer),
            vec![vec![process.addr(), 3.0f32.to_bits()]]
        );
        let breath = slot_target(&e, process, PROCESS_SLOT_2FC);
        assert_eq!(
            calls_to(&e, breath),
            vec![vec![process.addr(), 20.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_END_INTERRUPT_PACKAGE),
            vec![vec![victim.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_CLEAR_IN_COMBAT),
            vec![vec![victim.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_INTERRUPT_CAST),
            vec![vec![victim.addr() + 0x88]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SET_LIFE_STATE),
            vec![vec![victim.addr(), 6]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_RESTORE_FULL_HEALTH_AND_CONDITIONS),
            vec![vec![victim.addr()]]
        );
        // Level 6: the message "<name> <string>".
        let sprintf = calls_to(&e, SPRINTF);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(sprintf[0][1..], [TWO_STRINGS_FORMAT, 0x9800, 0x9700]);
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE),
            vec![vec![
                sprintf[0][0],
                0,
                MESSAGE_ICON_SURPRISED,
                0,
                2.0f32.to_bits(),
                0
            ]]
        );
        // The package of type 0xf would not be interrupted.
        e.call_log = Some(vec![]);
        stub(&mut e, PACKAGE_TYPE, 0xf);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        assert_eq!(call_count(&e, ACTOR_END_INTERRUPT_PACKAGE), 0);
        // The dead flag is not set for an essential actor.
        assert_eq!(e.mem.u8(victim.addr() + 0x118), 0);
        assert_eq!(call_count(&e, ACTOR_DO_DEATH_STUFF), 0);
    }

    #[test]
    fn actor_kill_gives_the_body_an_impulse() {
        let mut e = engine();
        let (victim, process, _, cells) = kill_setup(&mut e);
        let node = e.mem.alloc(0x40);
        let collision = slot_target(&e, victim.cast(), ACTOR_SLOT_1D0);
        e.register_double(collision, move |_, _| ret(node));
        stub(&mut e, ACTOR_COLLISION_OBJECT, node);
        e.mem.set_f32(cells[&GREETING_TIMER_SETTING], 4.0);
        e.mem.set_f32(cells[&DEATH_IMPULSE_SETTING], 2.0);
        // The proxy velocity has a length: its direction is normalized.
        stub_float(&mut e, VECTOR_LENGTH_4586D0, 1.0);
        e.map(DEFAULT_DIRECTION, 12);
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // The velocity at EBP-0x250, its transform at EBP-0x290, the direction at EBP-0x274.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(
            calls_to(&e, VECTOR_TRANSFORM_630B40)[0],
            vec![ebp - 0x250, ebp - 0x290]
        );
        assert_eq!(calls_to(&e, TES_HAVOK_ADD_VELOCITY)[0][1], ebp - 0x274);
        let greeting = slot_target(&e, process, PROCESS_SLOT_4B4);
        assert_eq!(
            calls_to(&e, greeting),
            vec![vec![process.addr(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, BHK_WORLD_SET_MOTION),
            vec![vec![node, 1, 1, 1, 1]]
        );
        let scale = calls_to(&e, VECTOR_SCALE_IN_PLACE);
        assert_eq!(scale.len(), 1);
        assert_eq!(scale[0][1], 2.0f32.to_bits());
        let velocity = calls_to(&e, TES_HAVOK_ADD_VELOCITY);
        assert_eq!(velocity, vec![vec![node, scale[0][0], 0]]);
        let flags = slot_target(&e, victim.cast(), ACTOR_SLOT_48);
        assert_eq!(calls_to(&e, flags), vec![vec![victim.addr(), 4]]);
        assert_eq!(call_count(&e, VECTOR_COPY_458620), 1);
        // The weapon drop runs (it asks the process for the other-hand
        // item); no combatant limit is exceeded and no commanding actor.
        let other_hand = slot_target(&e, process, PROCESS_SLOT_534);
        assert_eq!(call_count(&e, other_hand), 1);
        assert_eq!(call_count(&e, ACTOR_DO_DEATH_STUFF), 0);
    }

    #[test]
    fn actor_kill_uses_the_heading_when_the_velocity_is_zero() {
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        let node = e.mem.alloc(0x40);
        stub(&mut e, ACTOR_COLLISION_OBJECT, node);
        stub_float(&mut e, VECTOR_LENGTH_4586D0, 0.0);
        let heading = slot_target(&e, victim.cast(), ACTOR_SLOT_GET_HEADING);
        e.register_double(heading, |_, _| float_ret(1.5));
        let result = e.mem.alloc(12);
        e.mem.set_f32(result, 0.5);
        stub(&mut e, VECTOR_MULTIPLY_4B4500, result);
        e.map(DEFAULT_DIRECTION, 12);
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // The rotation at EBP-0x2b4, the rotated vector at EBP-0x2c0.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(calls_to(&e, VECTOR_ROTATE_4A0C90)[0][0], ebp - 0x2b4);
        assert_eq!(
            calls_to(&e, VECTOR_MULTIPLY_4B4500)[0][..2],
            [ebp - 0x2b4, ebp - 0x2c0]
        );
        let rotate = calls_to(&e, VECTOR_ROTATE_4A0C90);
        assert_eq!(rotate.len(), 1);
        assert_eq!(rotate[0][1], 1.5f32.to_bits());
        assert_eq!(call_count(&e, VECTOR_COPY_458620), 0);
        let multiply = calls_to(&e, VECTOR_MULTIPLY_4B4500);
        assert_eq!(multiply[0][2], UNIT_VECTOR);
        // The impulse direction is the result (normalized).
        let impulse = calls_to(&e, TES_HAVOK_ADD_VELOCITY);
        assert_eq!(e.mem.f32(impulse[0][1]), 0.5);
    }

    #[test]
    fn actor_kill_pushes_a_knocked_down_body() {
        let mut e = engine();
        let (victim, process, _, _) = kill_setup(&mut e);
        let node = e.mem.alloc(0x40);
        stub(&mut e, ACTOR_COLLISION_OBJECT, node);
        let knock = slot_target(&e, process, PROCESS_SLOT_40C);
        e.register_double(knock, |_, _| ret(3));
        let fixed = slot_target(&e, victim.cast(), ACTOR_SLOT_234);
        e.register_double(fixed, |_, _| ret(1));
        let collision = slot_target(&e, victim.cast(), ACTOR_SLOT_1D0);
        e.register_double(collision, move |_, _| ret(node));
        e.map(UPWARD_COMPONENT, 4);
        e.set_global(UPWARD_COMPONENT, 5.0f32);
        let result = e.mem.alloc(12);
        stub(&mut e, VECTOR_MULTIPLY_4B4500, result);
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // The rotation at EBP-0x314, the velocity at EBP-0x340.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(calls_to(&e, VECTOR_ROTATE_4A0C90)[0][0], ebp - 0x314);
        assert_eq!(calls_to(&e, TES_HAVOK_SET_VELOCITY)[0][1], ebp - 0x340);
        // Un-fixed twice: at the start (slot 0x234) and for the push.
        assert_eq!(
            calls_to(&e, BHK_RIGID_BODY_SET_FIXED),
            vec![vec![node, 0, 1], vec![node, 0, 1]]
        );
        let up = calls_to(&e, VECTOR3_CONSTRUCT_416870);
        assert_eq!(
            up[0][1..],
            [0.0f32.to_bits(), 5.0f32.to_bits(), 0.0f32.to_bits()]
        );
        let set = calls_to(&e, TES_HAVOK_SET_VELOCITY);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], node);
        assert_eq!(set[0][2], 0);
        assert_eq!(call_count(&e, BHK_WORLD_SET_MOTION), 0);
    }

    #[test]
    fn actor_kill_plays_the_death_dialogue() {
        let mut e = engine();
        let (victim, process, _, cells) = kill_setup(&mut e);
        e.mem.set_f32(cells[&DEATH_VOICE_DISTANCE_SETTING], 50.0);
        e.map(COMBAT_DIALOGUE_MANAGER, 4);
        e.set_global(COMBAT_DIALOGUE_MANAGER, 0x8b00u32);
        let attacker = big_actor(&mut e, &npc_slots());
        let _ = process;
        actor_kill(&mut e, victim, attacker, 1.0);
        assert_eq!(
            calls_to(&e, COMBAT_DIALOGUE_START_DIALOGUE),
            vec![vec![0x8b00, victim.addr(), attacker.addr(), 2, 6, 1, 0]]
        );

        // The global flag with a distance beyond the setting suppresses it.
        let mut e = engine();
        let (victim, _, _, cells) = kill_setup(&mut e);
        e.mem.set_f32(cells[&DEATH_VOICE_DISTANCE_SETTING], 50.0);
        e.map(COMBAT_DIALOGUE_MANAGER, 4);
        stub(&mut e, GLOBAL_FLAG_525420, 1);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 80.0);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        assert_eq!(call_count(&e, COMBAT_DIALOGUE_START_DIALOGUE), 0);
        // ... and a player within the distance lets the dialogue happen.
        e.call_log = Some(vec![]);
        e.set_global(COMBAT_DIALOGUE_MANAGER, 0x8b00u32);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 10.0);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        assert_eq!(call_count(&e, COMBAT_DIALOGUE_START_DIALOGUE), 1);
    }

    #[test]
    fn actor_kill_of_a_creature_plays_its_death_sound() {
        let mut e = engine();
        let (victim, _, _, _) = kill_setup(&mut e);
        // A creature: slot 0x218 false and 0x21c true.
        for (slot, value) in [(ACTOR_SLOT_218, 0), (ACTOR_SLOT_21C, 1)] {
            let target = slot_target(&e, victim.cast(), slot);
            e.register_double(target, move |_, _| ret(value));
        }
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        let position_slot = slot_target(&e, victim.cast(), ACTOR_SLOT_POSITION);
        e.register_double(position_slot, move |_, _| ret(position));
        let node_slot = slot_target(&e, victim.cast(), ACTOR_SLOT_1D0);
        e.register_double(node_slot, |_, _| ret(0x9900));
        stub(&mut e, REFR_BASE_FORM, 0x9a00);
        stub(&mut e, CREATURE_PICK_CREATURE_SOUND, 0x9b00);
        stub(&mut e, FORM_NUMERIC_ID, 0x4242);
        stub(&mut e, AUDIO_INSTANCE, 0x9c00);
        stub(&mut e, ACTOR_BASE_FORM, 0);
        stub(&mut e, RTTI_DYNAMIC_CAST, 0);
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        // The death sound handle at EBP-0x1f8.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID)[0][1],
            ebp - 0x1f8
        );
        assert_eq!(
            calls_to(&e, CREATURE_PICK_CREATURE_SOUND),
            vec![vec![0x9a00, 8]]
        );
        let play = calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID);
        assert_eq!(play.len(), 1);
        assert_eq!(play[0][0], 0x9c00);
        assert_eq!(play[0][2..], [0x4242, 0x102]);
        let handle = play[0][1];
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_POSITION_XYZ)[0][..2],
            [handle, 1.0f32.to_bits()]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_OBJECT_TO_FOLLOW),
            vec![vec![handle, 0x9900]]
        );
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![handle, 0]]);
        assert_eq!(call_count(&e, COMBAT_DIALOGUE_START_DIALOGUE), 0);
    }

    #[test]
    fn actor_kill_of_the_player_ends_the_world_around_it() {
        let mut e = engine();
        let (_, _, player, _) = kill_setup(&mut e);
        stub(&mut e, INTERFACE_IS_TOP_MENU_ID, 1);
        stub(&mut e, AUDIO_INSTANCE, 0x9c00);
        stub(&mut e, AUDIO_GET_SOUND_HANDLE_BY_NAME, 0x9d00);
        stub(&mut e, SETTING_TEST_454AF0, 1);
        let attacker = big_actor(&mut e, &npc_slots());
        e.mem.set_u32(player.addr() + 0x68, 0);
        let process = object(&mut e, 0x800, &process_slots());
        e.set(player, Actor::pCurrentProcess, process);
        embedded_table(
            &mut e,
            player.addr() + ACTOR_VALUE_OWNER_OFFSET,
            &[(0x00, ret(100)), (0x0c, float_ret(30.0))],
        );
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, player, attacker, 1.0);
        // The music handle at EBP-0x210.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][1],
            ebp - 0x210
        );
        assert_eq!(calls_to(&e, PIPBOY_RADIO_ENABLE), vec![vec![0]]);
        assert_eq!(calls_to(&e, SOUND_FADE_WORKER), vec![vec![0x3e8, 0]]);
        let music = calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(music[0][0], 0x9c00);
        assert_eq!(music[0][2..], [DEATH_MUSIC_NAME, 0x901]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![0x9d00, 0]]);
        assert_eq!(
            calls_to(&e, VATS_QUIT_PLAYBACK),
            vec![vec![VATS_OBJECT, 0, 0]]
        );
        assert_eq!(call_count(&e, SURGERY_MENU_CLOSE), 1);
        // The player is not VATS-playing: the VATS worker runs, and the
        // progress export remembers the killer only for the call.
        assert_eq!(call_count(&e, VATS_WORKER_9CA1F0), 1);
        assert_eq!(
            calls_to(&e, PLAYER_EXPORT_PROGRESS_DATA),
            vec![vec![player.addr(), 1]]
        );
        assert_eq!(e.get(player, Actor::pMyKiller).addr(), 0);
        // The chase bip (not slot 0x2a0) is used for the player.
        assert_eq!(
            calls_to(&e, MOBILE_OBJECT_SET_CHASE_BIP),
            vec![vec![player.addr(), 1]]
        );
    }

    #[test]
    fn actor_kill_tail_reports_and_updates_the_combat_counts() {
        let mut e = engine();
        let (victim, _, player, cells) = kill_setup(&mut e);
        e.mem.set_u8(cells[&DEATH_REPORT_SETTING], 1);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        stub(&mut e, FORM_NUMERIC_ID, 0x55);
        stub(&mut e, LOCATION_NAME, 0x9e00);
        stub(&mut e, ACTOR_BASE_FORM, 0x9f00);
        // The shader effect and the beam.
        e.mem.set_u8(player.addr() + 0xe2d, 1);
        let holder = e.global::<u32>(MAGIC_SHADER_HOLDER);
        e.mem.set_u32(holder + 0x638, 0xa100);
        e.mem.set_u32(victim.addr() + 0x1a0, 0xa200);
        e.mem.set_u32(victim.addr() + 0xb0, 0xa300);
        stub(&mut e, ACTOR_OUTPUT_931ED0, 0xa400);
        stub(&mut e, OBJECT_ACCESSOR_4A3A20, 0xa500);
        let collision = slot_target(&e, victim.cast(), ACTOR_SLOT_1D0);
        e.register_double(collision, |_, _| ret(0xa600));
        // The player kills with a teammate-free combat count of zero.
        stub(&mut e, GET_ALIGNMENT_FOR_KARMA, 1);
        e.map(SLOW_MO_SETTING, 4);
        e.mem.set_f32(cells[&SLOW_MO_SETTING], 0.25);
        let attacker = big_actor(&mut e, &npc_slots());
        let attacker_address = attacker.addr();
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, attacker, 1.0);
        // The output word at EBP-0x348.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(calls_to(&e, ACTOR_OUTPUT_931ED0)[0][1], ebp - 0x348);
        assert_eq!(
            calls_to(&e, ERROR_REPORT),
            vec![vec![KILLED_BY_FORMAT, 0x9e00, 0x55, 0x9e00, 0x55]]
        );
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_FINISH_MAGIC_SHADER_HIT_EFFECT),
            vec![vec![PROCESS_LISTS, victim.addr(), 0xa100]]
        );
        assert_eq!(
            calls_to(&e, CONTINUOUS_BEAM_PROJECTILE_SET_KILL),
            vec![vec![0xa200, victim.addr()]]
        );
        assert_eq!(call_count(&e, BHK_RAGDOLL_PENETRATION_UTIL_CLEAR), 1);
        let utility = calls_to(&e, PENETRATION_UTIL_WORKER);
        assert_eq!(utility, vec![vec![0xa300, 0xa600, 0xa500]]);
        let _ = attacker_address;

        // Without an attacker the other message is used.
        e.call_log = Some(vec![]);
        actor_kill(&mut e, victim, Ptr::NULL, 1.0);
        assert_eq!(
            calls_to(&e, ERROR_REPORT),
            vec![vec![DIED_WITH_NO_ATTACKER_FORMAT, 0x9e00, 0x55]]
        );
    }

    #[test]
    fn actor_kill_by_the_player_counts_combatants_and_the_slow_motion_camera() {
        let mut e = engine();
        let (victim, _, player, cells) = kill_setup(&mut e);
        stub(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0);
        e.mem.set_f32(cells[&SLOW_MO_SETTING], 0.25);
        stub(&mut e, PLAYER_INT_5BE4D0, 0);
        stub(&mut e, ACTOR_IS_FLEEING, 0);
        // No combatants: the slow-motion camera is requested.
        let frame = next_frame(&mut e, 0x374);
        actor_kill(&mut e, victim, player, 1.0);
        // The count word at EBP-0x34c.
        let ebp = (frame + 0x364 + 0xf) & !0xf;
        assert_eq!(
            calls_to(&e, COMBAT_MANAGER_GET_COMBATANT_COUNT)[0][2],
            ebp - 0x34c
        );
        let counted = calls_to(&e, COMBAT_MANAGER_GET_COMBATANT_COUNT);
        assert!(counted
            .iter()
            .any(|call| call[..2] == [0x8900, player.addr()]));
        assert_eq!(
            calls_to(&e, PLAYER_SET_SLOW_MO_CAMERA),
            vec![vec![
                player.addr(),
                victim.addr(),
                0.25f32.to_bits(),
                1,
                0xffff_ffff
            ]]
        );
        assert_eq!(call_count(&e, ACTOR_ADD_FACTION_MINOR_CRIME), 0);

        // With combatants that see the player, a minor crime is added and
        // there is no camera.
        e.call_log = Some(vec![]);
        stub(&mut e, COMBAT_MANAGER_GET_COMBATANT_COUNT, 2);
        stub(&mut e, PLAYER_IS_DETECTED_BY_NON_TEAMMATES, 1);
        actor_kill(&mut e, victim, player, 1.0);
        assert_eq!(
            calls_to(&e, ACTOR_ADD_FACTION_MINOR_CRIME),
            vec![vec![victim.addr(), 1, 1]]
        );
        assert_eq!(call_count(&e, PLAYER_SET_SLOW_MO_CAMERA), 0);
    }

    /// Makes the setting getter (`00403e20`) return a pointer to a `float`
    /// for each `(setting, value)` pair (any other setting reads 0.0).
    fn stub_float_settings(e: &mut Engine, settings: &[(u32, f32)]) {
        let zero = e.mem.alloc(8);
        let mut table = vec![];
        for (setting, value) in settings {
            let cell = e.mem.alloc(8);
            e.mem.set_f32(cell, *value);
            table.push((*setting, cell));
        }
        stub_with(e, SETTING_GET_VALUE_POINTER, move |_, a| {
            let cell = table
                .iter()
                .find(|(setting, _)| *setting == a[0])
                .map(|(_, cell)| *cell)
                .unwrap_or(zero);
            ret(cell)
        });
    }

    /// Makes the integer setting getter (`0043d4d0`) return a pointer to an
    /// integer for each `(setting, value)` pair (any other setting reads 0).
    fn stub_int_settings(e: &mut Engine, settings: &[(u32, i32)]) {
        let zero = e.mem.alloc(8);
        let mut table = vec![];
        for (setting, value) in settings {
            let cell = e.mem.alloc(8);
            e.mem.set_u32(cell, *value as u32);
            table.push((*setting, cell));
        }
        stub_with(e, SETTING_GET_INT_POINTER, move |_, a| {
            let cell = table
                .iter()
                .find(|(setting, _)| *setting == a[0])
                .map(|(_, cell)| *cell)
                .unwrap_or(zero);
            ret(cell)
        });
    }

    /// Makes the byte setting getter (`00408d60`) return a pointer to a byte
    /// for each `(setting, value)` pair (any other setting reads 0).
    fn stub_bool_settings(e: &mut Engine, settings: &[(u32, u8)]) {
        let zero = e.mem.alloc(8);
        let mut table = vec![];
        for (setting, value) in settings {
            let cell = e.mem.alloc(8);
            e.mem.set_u8(cell, *value);
            table.push((*setting, cell));
        }
        stub_with(e, SETTING_GET_BOOL_POINTER, move |_, a| {
            let cell = table
                .iter()
                .find(|(setting, _)| *setting == a[0])
                .map(|(_, cell)| *cell)
                .unwrap_or(zero);
            ret(cell)
        });
    }

    /// An acquire object with the given slots that `GetSavedAcquireObject`
    /// returns for every actor.
    fn shared_acquire(e: &mut Engine, slots: &[(u32, Ret)]) -> Ptr {
        let acquire = object(e, 0x800, slots);
        stub(e, GET_SAVED_ACQUIRE_OBJECT, acquire.addr());
        acquire
    }

    #[test]
    fn actor_stop_combat_ends_the_combat_with_a_controller() {
        let mut e = engine();
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        e.mem.set_u8(player.addr() + 0x7c4, 1);
        let acquire = shared_acquire(
            &mut e,
            &[
                (ACQUIRE_OBJECT_SLOT_128, ret(player.addr())),
                (ACQUIRE_OBJECT_SLOT_5C4, ret(0)),
                (ACQUIRE_OBJECT_SLOT_1C8, ret(0)),
                (PROCESS_SLOT_64C, ret(0)),
                (PROCESS_SLOT_614, ret(0)),
            ],
        );
        let process = object(
            &mut e,
            0x800,
            &[(PROCESS_SLOT_22C, ret(0x9300)), (PROCESS_SLOT_620, ret(0))],
        );
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_304, ret(1)),
                (ACTOR_SLOT_GET_ANIMATION, ret(0x77)),
                (ACTOR_SLOT_474, ret(0)),
                (ACTOR_SLOT_428, ret(1)),
                (ACTOR_SLOT_22C, ret(0)),
                (ACTOR_SLOT_360, ret(0)),
                (ACTOR_SLOT_21C, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        e.mem.set_u8(actor.addr() + 0x104, 1);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        stub(&mut e, COMBAT_CONTROLLER_STOP_COMBAT, 1);
        stub(&mut e, DEBUG_MESSAGES_ENABLED, 1);
        stub(&mut e, REFR_GET_NAME, 0x6000);
        stub(&mut e, REFR_EXTRA_DATA_LIST, 0x5000);
        actor_stop_combat(&mut e, actor, player.cast());
        // The player's acquire object is released: the actor is the source.
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_5C4)),
            vec![vec![acquire.addr(), 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_REMOVE_ACTOR_FROM_COMBAT_LIST),
            vec![vec![player.addr(), actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_CHANGE_PERCEIVED_HOSTILE_STATUS),
            vec![vec![player.addr(), actor.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, COMBAT_CONTROLLER_STOP_COMBAT),
            vec![vec![0x9300, player.addr()]]
        );
        // 008a07f7: the debug-messages test takes no argument word.
        assert_eq!(
            calls_to(&e, DEBUG_MESSAGES_ENABLED),
            vec![Vec::<u32>::new()]
        );
        assert_eq!(
            calls_to(&e, PRINT_DEBUG_LINE),
            vec![vec![STOPS_COMBAT_FORMAT, 0x6000]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_END_INTERRUPT_PACKAGE),
            vec![vec![actor.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, SCRIPT_SET_ACTION_FLAG),
            vec![vec![actor.addr(), 0x5000, 0x40]]
        );
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0xff);
        assert!(!e.get(actor, Actor::bInCombat));
        assert_eq!(
            calls_to(&e, slot_target(&e, process, PROCESS_SLOT_620)),
            vec![vec![process.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_1C8)),
            vec![vec![acquire.addr(), 0]]
        );
        assert_eq!(
            call_count(&e, slot_target(&e, acquire, PROCESS_SLOT_64C)),
            1
        );
    }

    #[test]
    fn actor_stop_combat_without_a_controller_clears_the_combat_state() {
        let mut e = engine();
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        shared_acquire(
            &mut e,
            &[
                (PROCESS_SLOT_614, ret(0)),
                (ACQUIRE_OBJECT_SLOT_1C8, ret(0)),
                (PROCESS_SLOT_64C, ret(0)),
            ],
        );
        let other = actor_with(&mut e, &[], None);
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_304, ret(0)),
                (ACTOR_SLOT_GET_ANIMATION, ret(0x77)),
                (ACTOR_SLOT_474, ret(0)),
                (ACTOR_SLOT_428, ret(0)),
                (ACTOR_SLOT_360, ret(0)),
                (ACTOR_SLOT_21C, ret(0)),
            ],
            None,
        );
        e.mem.set_u8(actor.addr() + 0x104, 1);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 0);
        actor_stop_combat(&mut e, actor, other);
        // Another actor than the player or nobody: the combat list and the
        // hostile status are left alone.
        assert_eq!(call_count(&e, PLAYER_REMOVE_ACTOR_FROM_COMBAT_LIST), 0);
        assert_eq!(call_count(&e, PLAYER_CHANGE_PERCEIVED_HOSTILE_STATUS), 0);
        assert_eq!(call_count(&e, COMBAT_CONTROLLER_STOP_COMBAT), 0);
        assert!(!e.get(actor, Actor::bInCombat));
    }

    #[test]
    fn fn_008a08c0_reads_the_byte_at_7c4() {
        let mut e = engine();
        let block = e.mem.alloc(0x800);
        e.mem.set_u8(block + 0x7c4, 9);
        assert_eq!(fn_008a08c0(&mut e, Ptr::new(block)), 9);
    }

    #[test]
    fn actor_clear_in_combat_restores_an_essential_actor() {
        let mut e = engine();
        let actor = actor_with(
            &mut e,
            &[(ACTOR_SLOT_360, ret(0)), (ACTOR_SLOT_21C, ret(0))],
            None,
        );
        embedded_table(&mut e, actor.addr() + 0xa4, &[(0x14, float_ret(5.0))]);
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        double_global(&mut e, ONE_DOUBLE, 1.0);
        e.map(RESTORE_AMOUNT, 4);
        e.set_global(RESTORE_AMOUNT, 1000.0f32);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 1);
        // Not asked to restore: only the flag is cleared.
        e.mem.set_u8(actor.addr() + 0x104, 1);
        actor_clear_in_combat(&mut e, actor, 0);
        assert!(!e.get(actor, Actor::bInCombat));
        assert_eq!(call_count(&e, ACTOR_RESTORE_ACTOR_VALUE), 0);
        // Was in combat, essential, asked to restore: seven values restored
        // (the two health values are positive, so only the 1000.0 ones).
        e.mem.set_u8(actor.addr() + 0x104, 1);
        actor_clear_in_combat(&mut e, actor, 1);
        assert_eq!(call_count(&e, ACTOR_RESTORE_ACTOR_VALUE), 7);
        // Not in combat before: nothing more.
        actor_clear_in_combat(&mut e, actor, 1);
        assert_eq!(call_count(&e, ACTOR_RESTORE_ACTOR_VALUE), 7);
        // A non-essential actor is not restored.
        stub(&mut e, ACTOR_GET_ESSENTIAL, 0);
        e.mem.set_u8(actor.addr() + 0x104, 1);
        actor_clear_in_combat(&mut e, actor, 1);
        assert_eq!(call_count(&e, ACTOR_RESTORE_ACTOR_VALUE), 7);
    }

    #[test]
    fn actor_restore_full_health_lifts_negative_values_to_one() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        embedded_table(&mut e, actor.addr() + 0xa4, &[(0x14, float_ret(-3.0))]);
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        double_global(&mut e, ONE_DOUBLE, 1.0);
        e.map(RESTORE_AMOUNT, 4);
        e.set_global(RESTORE_AMOUNT, 1000.0f32);
        actor_restore_full_health_and_conditions(&mut e, actor);
        let calls = calls_to(&e, ACTOR_RESTORE_ACTOR_VALUE);
        let a = actor.addr();
        let four = 4.0f32.to_bits();
        let thousand = 1000.0f32.to_bits();
        assert_eq!(
            calls,
            vec![
                vec![a, 0x10, four],
                vec![a, 0x16, four],
                vec![a, 0x1f, thousand],
                vec![a, 0x1d, thousand],
                vec![a, 0x1e, thousand],
                vec![a, 0x1b, thousand],
                vec![a, 0x1c, thousand],
                vec![a, 0x1a, thousand],
                vec![a, 0x19, thousand],
            ]
        );
        // Positive values are left alone.
        let other = actor_with(&mut e, &[], None);
        embedded_table(&mut e, other.addr() + 0xa4, &[(0x14, float_ret(2.0))]);
        e.call_log = Some(vec![]);
        actor_restore_full_health_and_conditions(&mut e, other);
        assert_eq!(call_count(&e, ACTOR_RESTORE_ACTOR_VALUE), 7);
    }

    #[test]
    fn value_table_callbacks_call_the_actor_slots() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[(0x43c, ret(0)), (0x440, ret(0))], None);
        let owner = actor.addr() + 0xa4;
        fn_008a0a90(&mut e, owner);
        fn_008a0ad0(&mut e, owner);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), 0x43c)),
            vec![vec![actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), 0x440)),
            vec![vec![actor.addr()]]
        );
    }

    #[test]
    fn value_table_derivations_answer_the_actor_slot_st0() {
        let mut e = engine();
        let actor = actor_with(
            &mut e,
            &[(0x43c, float_ret(12.5)), (0x440, float_ret(3.25))],
            None,
        );
        let owner = actor.addr() + 0xa4;
        assert_eq!(fn_008a0a90(&mut e, owner), 12.5);
        assert_eq!(fn_008a0ad0(&mut e, owner), 3.25);
        stub_with(&mut e, ACTOR_SET_FLAG_577250, |_, _| float_ret(7.75));
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        stub(&mut e, PLAYER_FLAG_4D1360, 1);
        assert_eq!(fn_008a0cd0(&mut e, owner), 7.75);
    }

    #[test]
    fn actor_get_current_speed_follows_the_animation_flags_or_the_combat_state() {
        let mut e = engine();
        stub_float(&mut e, ACTOR_GET_RUN_SPEED, 5.0);
        stub_float(&mut e, ACTOR_GET_WALK_SPEED, 2.0);
        stub_float(&mut e, ACTOR_SPEED_885ED0, 7.0);
        stub_float(&mut e, ACTOR_SPEED_885D90, 8.0);
        stub_float(&mut e, ACTOR_SPEED_886010, 9.0);
        // No process: run in combat unless heavy armor is worn.
        let actor = actor_with(&mut e, &[], None);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        stub(&mut e, IS_HEAVY_BODY_ARMOR_WORN, 0);
        assert_eq!(actor_get_current_speed(&mut e, actor), 5.0);
        stub(&mut e, IS_HEAVY_BODY_ARMOR_WORN, 1);
        assert_eq!(actor_get_current_speed(&mut e, actor), 2.0);
        stub(&mut e, IS_HEAVY_BODY_ARMOR_WORN, 0);
        stub(&mut e, ACTOR_IN_COMBAT, 0);
        assert_eq!(actor_get_current_speed(&mut e, actor), 2.0);
        // With a process whose level test is 0 the animation flags decide.
        let process = e.mem.alloc(0x10);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 0);
        for (flags, speed) in [
            (0x000, 2.0),
            (0x200, 5.0),
            (0x800, 8.0),
            (0xa00, 7.0),
            (0x2000, 9.0),
            (0x2200, 9.0),
        ] {
            stub(&mut e, ACTOR_ANIM_FLAGS, flags);
            assert_eq!(actor_get_current_speed(&mut e, actor), speed, "{flags:#x}");
        }
        // A process whose level test is not 0 falls back to the combat rule.
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 1);
        assert_eq!(actor_get_current_speed(&mut e, actor), 2.0);
    }

    #[test]
    fn fn_008a0c20_passes_the_value_through_entry_point_0x16() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        embedded_table(&mut e, actor.addr() + 0xa4, &[(0x0c, float_ret(7.0))]);
        stub_with(&mut e, HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value * 2.0);
            ret(0)
        });
        assert_eq!(fn_008a0c20(&mut e, actor), 14.0);
        let calls = calls_to(&e, HANDLE_ENTRY_POINT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][..2], [0x16, actor.addr()]);
    }

    #[test]
    fn fn_008a0c60_computes_a_missing_value_once() {
        let mut e = engine();
        double_global(&mut e, MINUS_ONE_DOUBLE, -1.0);
        let actor = actor_with(
            &mut e,
            &[(ACTOR_SLOT_30C, ret(0)), (ACTOR_SLOT_310, ret(0))],
            None,
        );
        let getter = slot_target(&e, actor.cast(), ACTOR_SLOT_310);
        let reads = std::rc::Rc::new(std::cell::Cell::new(0));
        let counter = reads.clone();
        e.register_double(getter, move |_, _| {
            counter.set(counter.get() + 1);
            // The first read is "not computed yet", the later ones are 6.0.
            float_ret(if counter.get() == 1 { -1.0 } else { 6.0 })
        });
        stub(&mut e, GET_INVENTORY_CHANGES, 0x1234);
        stub_float(&mut e, CHANGES_VALUE_4D1180, 3.0);
        assert_eq!(fn_008a0c60(&mut e, actor), 6.0);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_30C)),
            vec![vec![actor.addr(), 3.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, CHANGES_VALUE_4D1180),
            vec![vec![0x1234, actor.addr()]]
        );
        // A value that is already there is returned as it is.
        e.call_log = Some(vec![]);
        assert_eq!(fn_008a0c60(&mut e, actor), 6.0);
        assert_eq!(call_count(&e, CHANGES_VALUE_4D1180), 0);
    }

    #[test]
    fn fn_008a0cd0_hands_the_players_byte_to_the_actor() {
        let mut e = engine();
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        stub(&mut e, PLAYER_FLAG_4D1360, 1);
        fn_008a0cd0(&mut e, 0x5000 + 0xa4);
        assert_eq!(calls_to(&e, PLAYER_FLAG_4D1360), vec![vec![player.addr()]]);
        assert_eq!(calls_to(&e, ACTOR_SET_FLAG_577250), vec![vec![0x5000, 1]]);
    }

    /// Makes `_ftol2` truncate its `double` argument.
    fn stub_ftol(e: &mut Engine) {
        stub_with(e, FTOL, |_, a| {
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
    }

    #[test]
    fn detection_level_is_zero_without_an_acquire_object() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let actor = actor_with(&mut e, &[], None);
        let other = actor_with(&mut e, &[], None);
        let seen = e.mem.alloc(4);
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            0,
            other,
            Ptr::new(seen),
            0,
            0,
            0,
            Ptr::new(0),
        );
        assert_eq!(level, 0);
        // Nothing was asked of the entry points.
        assert_eq!(call_count(&e, HANDLE_ENTRY_POINT), 0);
    }

    #[test]
    fn detection_level_is_minus_100_beyond_the_maximum_distance() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        shared_acquire(&mut e, &[]);
        double_global(&mut e, DETECTION_MAX_DISTANCE, 8192.0);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 9000.0);
        let actor = actor_with(&mut e, &[], None);
        let other = actor_with(&mut e, &[], None);
        let seen = e.mem.alloc(4);
        e.mem.set_u8(seen, 7);
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            1,
            other,
            Ptr::new(seen),
            0,
            0,
            0,
            Ptr::new(0),
        );
        assert_eq!(level, -100);
        // The seen byte was cleared first.
        assert_eq!(e.mem.u8(seen), 0);
        // Within range, a mover that is not in combat while the player is
        // idle (the test of the other actor, the player's movement and
        // combat) also gives -100.
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 100.0);
        stub(&mut e, PROCESS_GET_FORCE_NEXT_UPDATE, 1);
        stub(&mut e, ACTOR_IS_MOVING, 1);
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            1,
            other,
            Ptr::new(seen),
            0,
            0,
            0,
            Ptr::new(0),
        );
        assert_eq!(level, -100);
    }

    #[test]
    fn detection_level_reads_the_stored_record_without_the_line_of_sight() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        double_global(&mut e, DETECTION_MAX_DISTANCE, 8192.0);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 100.0);
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record + 8, 33);
        e.mem.set_u8(record + 0x1c, 3);
        e.mem.set_u8(record + 0x1e, 1);
        let acquire = shared_acquire(&mut e, &[(ACQUIRE_OBJECT_SLOT_504, ret(record))]);
        let actor = actor_with(&mut e, &[], None);
        let other = actor_with(&mut e, &[(ACTOR_SLOT_1D0, ret(1))], None);
        let seen = e.mem.alloc(4);
        let seen_line = e.mem.alloc(4);
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            0,
            other,
            Ptr::new(seen),
            0,
            0,
            0,
            Ptr::new(seen_line),
        );
        assert_eq!(level, 33);
        assert_eq!(e.mem.u8(seen), 1);
        assert_eq!(e.mem.u8(seen_line), 3);
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_504)),
            vec![vec![acquire.addr(), other.addr(), 0]]
        );
        // Without a record the level is "unknown", which reads as -100.
        e.set_global(PLAYER_CHARACTER, player.addr());
        let slot = slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_504);
        e.register_double(slot, |_, _| ret(0));
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            0,
            other,
            Ptr::new(seen),
            0,
            0,
            0,
            Ptr::new(seen_line),
        );
        assert_eq!(level, -100);
        // An actor that cannot be detected (slot 0x1d0 is 0) also gives -100.
        let blind = actor_with(&mut e, &[(ACTOR_SLOT_1D0, ret(0))], None);
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            0,
            blind,
            Ptr::new(seen),
            0,
            0,
            0,
            Ptr::new(0),
        );
        assert_eq!(level, -100);
    }

    #[test]
    fn detection_level_uses_the_formula_and_reports_it() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        // The detecting actor is the player, so the result is reported.
        let actor = actor_with(
            &mut e,
            &[(ACTOR_SLOT_428, ret(0)), (ACTOR_SLOT_POSITION, ret(0))],
            None,
        );
        set_player(&mut e, actor);
        let position = e.mem.alloc(12);
        let slot = slot_target(&e, actor.cast(), ACTOR_SLOT_POSITION);
        e.register_double(slot, move |_, _| ret(position));
        embedded_table(
            &mut e,
            actor.addr() + 0xa4,
            &[
                (0x0c, float_ret(0.0)),
                (0x08, ret(0)),
                (0x28, ret(0x1_2345)),
            ],
        );
        let acquire = shared_acquire(
            &mut e,
            &[
                (ACQUIRE_OBJECT_SLOT_504, ret(0)),
                (PROCESS_SLOT_22C, ret(0)),
                (ACQUIRE_OBJECT_SLOT_734, float_ret(10.0)),
                (ACQUIRE_OBJECT_SLOT_F0, ret(0)),
            ],
        );
        let other = actor_with(
            &mut e,
            &[(ACTOR_SLOT_1D0, ret(1)), (ACTOR_SLOT_100, ret(1))],
            None,
        );
        embedded_table(
            &mut e,
            other.addr() + 0xa4,
            &[(0x0c, float_ret(0.0)), (0x08, ret(0)), (0x28, ret(7))],
        );
        double_global(&mut e, DETECTION_MAX_DISTANCE, 8192.0);
        double_global(&mut e, DEGREES_TO_RADIANS, 0.017453292);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 100.0);
        stub_float_settings(&mut e, &[(SETTING_VIEW_CONE_ANGLE, 90.0)]);
        stub_with(&mut e, GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            float_ret(if a[1] == ACTOR_VALUE_PERCEPTION {
                40.0
            } else {
                25.0
            })
        });
        stub_ftol(&mut e);
        stub(&mut e, ACTOR_LINE_OF_SIGHT, 1);
        stub(&mut e, ACTOR_IS_POINT_IN_VIEW_CONE, 1);
        stub(&mut e, ACTOR_ARMOR_VALUE, 3);
        stub(&mut e, HEIGHT_CLASS, 2);
        stub(&mut e, DETECTION_FORMULA, 55);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        stub(&mut e, ACTOR_FLAG_4938E0, 1);
        let seen = e.mem.alloc(4);
        let seen_line = e.mem.alloc(4);
        let level = actor_get_detection_level_against_actor(
            &mut e,
            actor,
            1,
            other,
            Ptr::new(seen),
            9,
            0,
            0,
            Ptr::new(seen_line),
        );
        assert_eq!(level, 55);
        assert_eq!(e.mem.u8(seen), 1);
        assert_eq!(e.mem.u8(seen_line), 1);
        let formula = calls_to(&e, DETECTION_FORMULA);
        assert_eq!(formula.len(), 1);
        let words = &formula[0];
        assert_eq!(words.len(), 25);
        // Perception, stealth, line of sight, seen, distance, sneak bonus.
        assert_eq!(words[..6], [40, 25, 1, 1, 100.0f32.to_bits(), 10]);
        // Armor value, the movement flag byte, combat byte, word 0x472380.
        assert_eq!(words[8], 3);
        assert_eq!(words[9], 1);
        // The two detectors' words masked to 16 bits.
        assert_eq!(words[22..24], [0x2345, 7]);
        // The sight check was made with flag 1 against the other actor.
        let sight = calls_to(&e, ACTOR_LINE_OF_SIGHT);
        assert_eq!(sight.len(), 1);
        assert_eq!(sight[0][..3], [actor.addr(), 1, other.addr()]);
        // The result is reported to both acquire objects (level 3 as the
        // result is positive).
        let reports = calls_to(&e, slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_F0));
        assert_eq!(
            reports,
            vec![
                vec![acquire.addr(), other.addr(), 3, 1, 55, 0, 1, 9],
                vec![acquire.addr(), actor.addr(), 3, 1, 55, 1, 1, 9],
            ]
        );
    }

    #[test]
    fn small_actor_fields_and_process_tests() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        e.mem.set_u32(actor.addr() + 0x128, 0x1234);
        assert_eq!(fn_008a16b0(&mut e, actor), 0x1234);
        // fn_008a16d0 asks the process, and is 0 without one.
        assert_eq!(actor_is_weapon_drawn(&mut e, actor), 0);
        let process = object(
            &mut e,
            0x800,
            &[
                (PROCESS_SLOT_454, ret(1)),
                (PROCESS_SLOT_CURRENT_WEAPON, ret(0x6000)),
            ],
        );
        e.set(actor, Actor::pCurrentProcess, process);
        assert_eq!(actor_is_weapon_drawn(&mut e, actor), 1);
        // The current weapon is the form of the entry the process reports.
        stub(&mut e, ENTRY_FORM, 0x7000);
        assert_eq!(actor_get_current_weapon(&mut e, actor), 0x7000);
        assert_eq!(calls_to(&e, ENTRY_FORM), vec![vec![0x6000]]);
        let none = slot_target(&e, process, PROCESS_SLOT_CURRENT_WEAPON);
        e.register_double(none, |_, _| ret(0));
        assert_eq!(actor_get_current_weapon(&mut e, actor), 0);
    }

    #[test]
    fn fn_008a1760_numbers_the_actors_state() {
        let mut e = engine();
        let player = actor_with(&mut e, &[(ACTOR_SLOT_22C, ret(0))], None);
        set_player(&mut e, player);
        e.map(WEAPON_TYPE_TABLE, 0x40);
        e.mem.set_u32(WEAPON_TYPE_TABLE + 12, 77);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_22C, ret(0))], None);
        // The player always gives 0xc.
        assert_eq!(fn_008a1760(&mut e, player), 0xc);
        // No acquire object: 0.
        assert_eq!(fn_008a1760(&mut e, actor), 0);
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_CURRENT_WEAPON, ret(0))]);
        e.set(actor, Actor::pCurrentProcess, process);
        shared_acquire(&mut e, &[]);
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 1);
        // Level 1 without a weapon: 1.
        assert_eq!(fn_008a1760(&mut e, actor), 1);
        // With a weapon of type 3 the table entry.
        let weapon_slot = slot_target(&e, process, PROCESS_SLOT_CURRENT_WEAPON);
        e.register_double(weapon_slot, |_, _| ret(0x6000));
        stub(&mut e, ENTRY_FORM, 0x7000);
        stub(&mut e, WEAPON_TYPE, 3);
        assert_eq!(fn_008a1760(&mut e, actor), 77);
        // A higher level gives 0.
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 2);
        assert_eq!(fn_008a1760(&mut e, actor), 0);
        // Fleeing gives 0.
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 0);
        let fleeing = actor_with(&mut e, &[(ACTOR_SLOT_22C, ret(1))], None);
        assert_eq!(fn_008a1760(&mut e, fleeing), 0);
    }

    #[test]
    fn actor_set_life_state_turns_essential_deaths_into_state_6() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_48, ret(0))], None);
        stub_bool_settings(&mut e, &[(SETTING_ESSENTIAL_PROTECTION, 1)]);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 1);
        stub(&mut e, ACTOR_CAN_KNOCK_DOWN, 1);
        actor_set_life_state(&mut e, actor, 2);
        assert_eq!(e.get(actor, Actor::eLifeState), 6);
        // Entering 6 from 0 stops the actor; the slot is told of the change.
        assert_eq!(calls_to(&e, ACTOR_STOP_MOVING), vec![vec![actor.addr()]]);
        assert_eq!(
            calls_to(&e, slot_target(&e, actor.cast(), ACTOR_SLOT_48)),
            vec![vec![actor.addr(), 0x400]]
        );
        // An actor that cannot be knocked down goes to 0 instead.
        stub(&mut e, ACTOR_CAN_KNOCK_DOWN, 0);
        actor_set_life_state(&mut e, actor, 1);
        assert_eq!(e.get(actor, Actor::eLifeState), 0);
    }

    #[test]
    fn actor_set_life_state_death_silences_the_actor() {
        let mut e = engine();
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_610, ret(3))]);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_48, ret(0))], None);
        e.set(actor, Actor::pCurrentProcess, process);
        stub_bool_settings(&mut e, &[]);
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 0);
        stub(&mut e, REFR_EXTRA_DATA_LIST, 0x5000);
        stub(&mut e, GET_CONTAINER_CHANGES, 0x5100);
        stub(&mut e, ACTOR_SOUND_SOURCE, 0x5200);
        stub(&mut e, AUDIO_GET_INSTANCE, 0x6100);
        actor_set_life_state(&mut e, actor, 2);
        assert_eq!(e.get(actor, Actor::eLifeState), 2);
        assert_eq!(
            calls_to(&e, CHANGES_FN_4534F0),
            vec![vec![0x5100, actor.addr()]]
        );
        assert_eq!(call_count(&e, SOUND_HANDLE_STOP), 1);
        assert_eq!(call_count(&e, SOUND_HANDLE_RELEASE), 1);
        assert_eq!(
            calls_to(&e, AUDIO_STOP_MOVING_SOUNDS),
            vec![vec![0x6100, 0x5200, 0, 0]]
        );
        // Dead, with a process in state 3: faded in; the animations reset.
        assert_eq!(
            calls_to(&e, HIGH_PROCESS_FADE_IN),
            vec![vec![process.addr(), actor.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_RESET_LOADED_ANIMATIONS),
            vec![vec![actor.addr()]]
        );
        // The same state again changes nothing.
        e.call_log = Some(vec![]);
        actor_set_life_state(&mut e, actor, 2);
        assert_eq!(call_count(&e, ACTOR_RESET_LOADED_ANIMATIONS), 0);
    }

    #[test]
    fn fn_008a1a40_stores_the_stage_and_applies_it() {
        let mut e = engine();
        // No 3D body: applying does nothing more.
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_1D0, ret(0))], None);
        fn_008a1a40(&mut e, actor, 4);
        assert_eq!(e.get(actor, Actor::eCriticalStage), 4);
        assert_eq!(call_count(&e, BHK_REPLACE_CONSTRAINTS), 0);
    }

    #[test]
    fn actor_apply_critical_stage_acts_on_the_stage() {
        let mut e = engine();
        let player = actor_with(&mut e, &[(ACTOR_SLOT_360, ret(1))], None);
        let killer = player;
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_1D0, ret(0x7000)),
                (ACTOR_SLOT_22C, ret(1)),
                (ACTOR_SLOT_1C0, ret(0)),
                (ACTOR_SLOT_360, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pMyKiller, killer.cast());
        // Stage 1: constraints replaced and velocity damped.
        e.set(actor, Actor::eCriticalStage, 1);
        actor_apply_critical_stage(&mut e, actor);
        assert_eq!(calls_to(&e, BHK_REPLACE_CONSTRAINTS), vec![vec![0x7000, 8]]);
        assert_eq!(
            calls_to(&e, BHK_DAMP_BODIES_VELOCITY),
            vec![vec![0x7000, 0.0f32.to_bits()]]
        );
        // Stage 2: out of the water, and the player's kill is counted.
        e.set(actor, Actor::eCriticalStage, 2);
        actor_apply_critical_stage(&mut e, actor);
        assert_eq!(
            calls_to(&e, REFR_REMOVE_FROM_ALL_WATER),
            vec![vec![actor.addr(), 0]]
        );
        assert_eq!(calls_to(&e, BODY_FN_450F90), vec![vec![0x7000, 1]]);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x1d]]);
        // A stage of a living actor prints a line.
        let living = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_1D0, ret(0x7000)),
                (ACTOR_SLOT_22C, ret(0)),
                (ACTOR_SLOT_130, ret(0x6500)),
            ],
            None,
        );
        e.set(living, Actor::eCriticalStage, 3);
        stub(&mut e, REFR_FORM_ID, 0xabcd);
        actor_apply_critical_stage(&mut e, living);
        assert_eq!(
            calls_to(&e, PRINT_ERROR_LINE),
            vec![vec![CRITICAL_STAGE_NOT_DEAD_FORMAT, 3, 0x6500, 0xabcd]]
        );
        // With the task queue in use the stage is queued instead.
        e.call_log = Some(vec![]);
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, TASK_QUEUE_INTERFACE, 0x7300);
        actor_apply_critical_stage(&mut e, actor);
        assert_eq!(
            calls_to(&e, QUEUE_ACTOR_APPLY_CRITICAL_STAGE),
            vec![vec![0x7300, actor.addr()]]
        );
        assert_eq!(call_count(&e, BHK_REPLACE_CONSTRAINTS), 0);
    }

    #[test]
    fn fn_008a1bd0_without_a_dialogue_item_only_copies_the_handle() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        let result = e.mem.alloc(12);
        let topic = e.mem.alloc(4);
        let returned = fn_008a1bd0(
            &mut e,
            actor,
            Ptr::new(result),
            Ptr::new(topic),
            0x10,
            0,
            0,
            0,
            0,
        );
        assert_eq!(returned, result);
        assert_eq!(
            calls_to(&e, TOPIC_CREATE_DIALOGUE_ITEM),
            vec![vec![topic, actor.addr(), 0x10, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, MOBILE_OBJECT_STOP_CURRENT_DIALOGUE),
            vec![vec![actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, REFR_SET_SAY_TO_TOPIC),
            vec![vec![actor.addr(), topic]]
        );
        // The handle built on the stack is copied into the result and then
        // destroyed.
        let assigns = calls_to(&e, SOUND_HANDLE_ASSIGN);
        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0][0], result);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTROY),
            vec![vec![assigns[0][1]]]
        );
        assert_eq!(call_count(&e, DIALOGUE_ITEM_DESTROY), 0);
    }

    #[test]
    fn fn_008a1bd0_plays_the_response_as_a_static_sound_and_shows_the_subtitle() {
        let mut e = engine();
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_490, ret(0))]);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_POSITION, ret(position))], None);
        e.set(actor, Actor::pCurrentProcess, process);
        e.mem.set_u8(actor.addr() + 0x7f, 1);
        let result = e.mem.alloc(12);
        let topic = e.mem.alloc(4);
        stub(&mut e, TOPIC_CREATE_DIALOGUE_ITEM, 0x5000);
        stub(&mut e, REFR_FORM_ID, 0x4400);
        stub(&mut e, DIALOGUE_ITEM_GET_CURRENT_RESPONSE, 0x5500);
        stub(&mut e, REFR_BASE_FORM, 0);
        stub(&mut e, RESPONSE_SOUND_HOLDER, 0x5600);
        stub(&mut e, ENTRY_FIRST_EXTRA_LIST, 0x5700);
        stub(&mut e, AUDIO_INSTANCE, 0x6100);
        stub(&mut e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, 0x6200);
        stub(&mut e, LIST_ITEM_SLOT, 0x5800);
        stub(&mut e, ACTOR_ALWAYS_SHOW_SUBTITLES, 1);
        stub_float_settings(&mut e, &[(SETTING_STATIC_ATTENUATION, 2.0)]);
        stub_bool_settings(&mut e, &[]);
        stub_ftol(&mut e);
        double_global(&mut e, HUNDRED, 100.0);
        stub_with(&mut e, SOUND_HANDLE_ASSIGN, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, 0x11 * (i + 1));
            }
            ret(0)
        });
        let returned = fn_008a1bd0(
            &mut e,
            actor,
            Ptr::new(result),
            Ptr::new(topic),
            0x10,
            1,
            0,
            0,
            1,
        );
        assert_eq!(returned, result);
        // The info of the item gets the result run and the topic list.
        assert_eq!(calls_to(&e, TOPIC_INFO_ADD_TOPIC_LIST), vec![vec![0x4400]]);
        assert_eq!(
            calls_to(&e, TOPIC_INFO_RUN_RESULT),
            vec![vec![0x4400, 0, actor.addr()]]
        );
        // The file name goes through the 512-byte buffer, the sound is looked
        // up with flags 0x105 (static) and its attenuation is set.
        let lookups = calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0][0], 0x6100);
        assert_eq!(lookups[0][3..], [0x105, 0]);
        let copies = calls_to(&e, COPY_STRING_LIMITED);
        assert_eq!(copies[0][1..], [0x200, 0x5700]);
        let attenuation = calls_to(&e, SOUND_HANDLE_SET_STATIC_ATTENUATION);
        assert_eq!(attenuation.len(), 1);
        assert_eq!(attenuation[0][1], 200);
        assert_eq!(call_count(&e, SOUND_HANDLE_SET_POSITION_XYZ), 0);
        // The sound plays and calls back when done.
        let plays = calls_to(&e, SOUND_HANDLE_PLAY);
        assert_eq!(plays.len(), 1);
        assert_eq!(plays[0][1], 0);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_COMPLETION_CALLBACK),
            vec![vec![plays[0][0], SAY_TO_CALLBACK, 0x4400]]
        );
        assert_eq!(e.mem.u8(actor.addr() + 0x80), 0);
        // The process is given the flag and the handle by value.
        assert_eq!(
            calls_to(&e, slot_target(&e, process, PROCESS_SLOT_490)),
            vec![vec![process.addr(), 0, 0x11, 0x22, 0x33]]
        );
        // The subtitle: the text, the handle, the position, the speaker word
        // and a flag.
        assert_eq!(
            calls_to(&e, INTERFACE_SHOW_SUBTITLE),
            vec![vec![
                0x5700,
                0x11,
                0x22,
                0x33,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0x10,
                1
            ]]
        );
        assert_eq!(calls_to(&e, DIALOGUE_ITEM_DESTROY), vec![vec![0x5000, 1]]);
    }

    /// The objects and settings `fn_008a20d0` needs for a line with a lip
    /// sync animation; returns the actor, the acquire object and the lip
    /// sync animation.
    fn say_line_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr, u32) {
        let player = actor_with(e, &[], None);
        set_player(e, player);
        let lip_sync = 0x8800u32;
        let animation = 0x9900u32;
        let acquire = shared_acquire(
            e,
            &[
                (ACQUIRE_OBJECT_SLOT_37C, ret(lip_sync)),
                (ACQUIRE_OBJECT_SLOT_378, ret(0)),
                (PROCESS_SLOT_490, ret(0)),
                (ACQUIRE_OBJECT_SLOT_748, ret(lip_sync)),
                (ACQUIRE_OBJECT_SLOT_380, ret(0)),
                (ACQUIRE_OBJECT_SLOT_370, ret(0)),
            ],
        );
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_2D4, ret(0)),
                (ACTOR_SLOT_2DC, ret(0)),
                (ACTOR_SLOT_1D0, ret(0x7777)),
                (ACTOR_SLOT_360, ret(0)),
                (ACTOR_SLOT_1E4, ret(animation)),
                (ACTOR_SLOT_POSITION, ret(position)),
                (ACTOR_SLOT_21C_TEST, ret(0)),
            ],
            None,
        );
        let process = object(e, 0x800, &[]);
        e.set(actor, Actor::pCurrentProcess, process);
        double_global(e, ZERO_DOUBLE, 0.0);
        double_global(e, LIP_SYNC_FRAME_RATE, 30.0);
        double_global(e, SOUND_HEIGHT_OFFSET, 200.0);
        double_global(e, THOUSAND_DOUBLE, 1000.0);
        double_global(e, HUNDRED, 100.0);
        e.map(LIP_SYNC_START_FLOAT, 4);
        e.set_global(LIP_SYNC_START_FLOAT, 0.2f32);
        e.map(SAY_COUNTDOWN, 4);
        stub_float_settings(e, &[(SETTING_11D5788, 0.5)]);
        stub_int_settings(e, &[(SETTING_INT_11CD378, 1000)]);
        stub_bool_settings(e, &[]);
        stub_ftol(e);
        stub_float(e, REFR_GET_DISTANCE_FROM_REFERENCE, 50.0);
        stub(e, ENTRY_FIRST_EXTRA_LIST, 3000);
        stub(e, REFR_FORM_ID, 0x4400);
        stub(e, ACTOR_GET_FACE_ANIMATION_DATA, 0x6700);
        stub(e, AUDIO_INSTANCE, 0x6100);
        stub(e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, 0x6200);
        stub(e, AUDIO_MANAGER_INSTANCE, 0x6300);
        stub_float(e, LIP_SYNC_LENGTH, 2.0);
        stub_float(e, SOUND_MAX_DISTANCE, 900.0);
        stub_float(e, SOUND_MIN_DISTANCE, 100.0);
        stub(e, ACTOR_TEST_880910, 1);
        (actor, acquire, lip_sync)
    }

    #[test]
    fn fn_008a20d0_speaks_a_line_with_a_lip_sync_animation() {
        let mut e = engine();
        let (actor, acquire, lip_sync) = say_line_setup(&mut e);
        let out = e.mem.alloc(12);
        let result = fn_008a20d0(
            &mut e,
            actor,
            0x4000,
            Ptr::new(out),
            0x10,
            0x14,
            5,
            0,
            0,
            Ptr::new(0),
            0,
            0,
            0,
            0,
            0,
        );
        // 0.5 + 2.0 seconds of lip sync plus the 3000 / 30 = 100 of the frame
        // count.
        assert_eq!(result, 102.5);
        assert_eq!(
            calls_to(&e, LIP_SYNC_START),
            vec![vec![lip_sync, 0x6700, 0.2f32.to_bits()]]
        );
        // The sound is looked up with the flags of a positioned sound.
        let lookups = calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(lookups[0][0], 0x6100);
        assert_eq!(lookups[0][3..], [0x104 | 0x80000 | 2, 0]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_PRIORITY),
            vec![vec![out, 0x40]]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_MIN_MAX),
            vec![vec![out, 100.0f32.to_bits(), 900.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_POSITION_XYZ),
            vec![vec![
                out,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                203.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_OBJECT_TO_FOLLOW),
            vec![vec![out, 0x7777]]
        );
        assert_eq!(
            calls_to(&e, AUDIO_MANAGER_SET_CALLBACK_B),
            vec![vec![0x6300, 3000, SAY_CANCEL_FN, 0x4400]]
        );
        // The played callback is `00935f00` (008a29ba), never `SayToCallBack`,
        // which would end the line as its voice starts.
        assert_eq!(SAY_CANCEL_FN, 0x0093_5f00);
        assert_eq!(call_count(&e, SAY_TO_CALLBACK), 0);
        // Playback is delayed by (0.5 + 2.0) * 1000 milliseconds.
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_PLAY_AFTER),
            vec![vec![out, 2500, 0]]
        );
        assert_eq!(calls_to(&e, SAY_LOCK_ENTER), vec![vec![SAY_LOCK, 0]]);
        assert_eq!(calls_to(&e, SAY_LOCK_LEAVE), vec![vec![SAY_LOCK]]);
        // The lip sync animation is kept: the acquire object already has it.
        assert_eq!(call_count(&e, LIP_SYNC_DESTRUCT), 0);
        assert_eq!(
            calls_to(&e, slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_380)),
            vec![vec![acquire.addr(), 0]]
        );
    }

    #[test]
    fn fn_008a20d0_refuses_lines_that_cannot_be_spoken() {
        let mut e = engine();
        let (actor, _acquire, _lip_sync) = say_line_setup(&mut e);
        let out = e.mem.alloc(12);
        // No name: the callback is told, nothing is looked up.
        let result = fn_008a20d0(
            &mut e,
            actor,
            0,
            Ptr::new(out),
            0,
            0,
            0,
            0,
            0,
            Ptr::new(0),
            0,
            0,
            0,
            0,
            0,
        );
        assert_eq!(result, 0.0);
        assert_eq!(calls_to(&e, SAY_TO_CALLBACK), vec![vec![0x4400, 1]]);
        assert_eq!(call_count(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME), 0);
        // In dialogue the other callback is used.
        e.call_log = Some(vec![]);
        stub(&mut e, MOBILE_OBJECT_IS_IN_DIALOGUE, 1);
        fn_008a20d0(
            &mut e,
            actor,
            0,
            Ptr::new(out),
            0,
            0,
            0,
            0,
            0,
            Ptr::new(0),
            0,
            0,
            0,
            0,
            0,
        );
        assert_eq!(
            calls_to(&e, SAY_TO_CALLBACK_IN_DIALOGUE),
            vec![vec![0x4400, 1]]
        );
        assert_eq!(call_count(&e, SAY_TO_CALLBACK), 0);
        // A dismembered actor cannot speak either.
        e.call_log = Some(vec![]);
        stub(&mut e, REFR_GET_DISMEMBERED, 1);
        let result = fn_008a20d0(
            &mut e,
            actor,
            0x4000,
            Ptr::new(out),
            0,
            0,
            0,
            0,
            0,
            Ptr::new(0),
            0,
            0,
            0,
            0,
            0,
        );
        assert_eq!(result, 0.0);
        assert_eq!(call_count(&e, SAY_LOCK_ENTER), 0);
    }

    #[test]
    fn fn_008a2d20_reads_its_setting() {
        let mut e = engine();
        stub_float_settings(&mut e, &[(SETTING_11D5788, 1.5)]);
        assert_eq!(fn_008a2d20(&mut e), 1.5);
    }

    #[test]
    fn fn_008a2d40_finds_the_item_the_package_targets() {
        let mut e = engine();
        let process = object(
            &mut e,
            0x800,
            &[
                (PROCESS_SLOT_22C, ret(0x5000)),
                (PROCESS_SLOT_278, ret(0x6000)),
            ],
        );
        let actor = actor_with(&mut e, &[], None);
        // Without a process: 0.
        assert_eq!(fn_008a2d40(&mut e, actor), 0);
        e.set(actor, Actor::pCurrentProcess, process);
        stub(&mut e, PACKAGE_GET_TARGET_OBJECT, 0x5100);
        stub(&mut e, REFR_GET_INVENTORY_ITEM_FN, 0x9999);
        // Target type 0: the process' entry form decides.
        stub(&mut e, PACKAGE_TARGET_GET_TARG_TYPE, 0);
        stub(&mut e, ENTRY_FORM, 0x7100);
        stub(&mut e, REFR_FORM_ID, 0x44);
        stub(&mut e, REFR_BASE_FORM, 0x7200);
        assert_eq!(fn_008a2d40(&mut e, actor), 0x9999);
        assert_eq!(
            calls_to(&e, REFR_GET_INVENTORY_ITEM_FN),
            vec![vec![actor.addr(), 0x7200, 0x44]]
        );
        // Target type 1: the target object when its slot 0xe4 accepts.
        e.call_log = Some(vec![]);
        let object_with_test = object(&mut e, 0x100, &[(REFERENCE_SLOT_E4, ret(1))]);
        stub(&mut e, PACKAGE_TARGET_GET_TARG_TYPE, 1);
        stub(&mut e, ENTRY_FORM, 0);
        stub(
            &mut e,
            PACKAGE_TARGET_GET_TARG_OBJECT,
            object_with_test.addr(),
        );
        assert_eq!(fn_008a2d40(&mut e, actor), 0x9999);
        assert_eq!(
            calls_to(&e, REFR_GET_INVENTORY_ITEM_FN),
            vec![vec![actor.addr(), object_with_test.addr(), 0]]
        );
        // Target type 2: the container changes find the object.
        e.call_log = Some(vec![]);
        stub(&mut e, PACKAGE_TARGET_GET_TARG_TYPE, 2);
        stub(&mut e, REFR_EXTRA_DATA_LIST, 0x5200);
        stub(&mut e, GET_CONTAINER_CHANGES, 0x5300);
        stub(&mut e, PACKAGE_TARGET_GET_TARG_OBJECT_TYPE, 7);
        stub(
            &mut e,
            INVENTORY_CHANGES_GET_OBJECT_BY_PACK_OBJ_TYPE,
            0x7400,
        );
        assert_eq!(fn_008a2d40(&mut e, actor), 0x9999);
        let found = calls_to(&e, INVENTORY_CHANGES_GET_OBJECT_BY_PACK_OBJ_TYPE);
        assert_eq!(found[0][..2], [0x5300, 7]);
        assert_eq!(
            calls_to(&e, REFR_GET_INVENTORY_ITEM_FN),
            vec![vec![actor.addr(), 0x7400, 0]]
        );
        // Nothing found: 0.
        stub(&mut e, INVENTORY_CHANGES_GET_OBJECT_BY_PACK_OBJ_TYPE, 0);
        assert_eq!(fn_008a2d40(&mut e, actor), 0);
    }

    #[test]
    fn fn_008a2ed0_reports_the_actor_the_acquire_object_targets() {
        let mut e = engine();
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        e.map(SINGLETON_11DE45C, 4);
        e.set_global(SINGLETON_11DE45C, 0x5000u32);
        let actor = actor_with(&mut e, &[], None);
        // The singleton test is true: 0.
        stub(&mut e, SINGLETON_TEST_47C850, 1);
        assert_eq!(fn_008a2ed0(&mut e, actor), 0);
        stub(&mut e, SINGLETON_TEST_47C850, 0);
        // No acquire object: 0.
        assert_eq!(fn_008a2ed0(&mut e, actor), 0);
        let acquire = shared_acquire(&mut e, &[(ACQUIRE_OBJECT_SLOT_678, ret(0x5555))]);
        assert_eq!(fn_008a2ed0(&mut e, actor), 0x5555);
        // Without a target the package target distance is looked at, but
        // the result stays 0.
        let slot = slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_678);
        e.register_double(slot, |_, _| ret(0));
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, 0x6000);
        stub(&mut e, ACTOR_GET_CURRENT_PACKAGE_TARGET, 0x6100);
        stub_float_settings(&mut e, &[]);
        assert_eq!(fn_008a2ed0(&mut e, actor), 0);
        assert_eq!(
            calls_to(&e, REFR_GET_DISTANCE_FROM_REFERENCE),
            vec![vec![0x6100, actor.addr(), 0, 0]]
        );
    }

    /// Makes `NiPoint3::operator+=` add the vector at the pointer to the
    /// vector at `this`.
    fn stub_vector_add(e: &mut Engine) {
        stub_with(e, VECTOR_ADD_ASSIGN, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, sum);
            }
            ret(a[0])
        });
    }

    #[test]
    fn fn_008a2fa0_uses_the_head_node_and_the_ragdoll_eye_offset() {
        let mut e = engine();
        e.map(EYE_POSITION_DEFAULT, 12);
        stub_vector_add(&mut e);
        let translation = e.mem.alloc(12);
        e.mem.set_f32(translation, 10.0);
        e.mem.set_f32(translation + 4, 20.0);
        e.mem.set_f32(translation + 8, 30.0);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_1E8_NODE, ret(0x7000))], None);
        e.set(actor, Actor::pRagdollController, Ptr::new(0x7800));
        stub(&mut e, NODE_FN_4AB230, 0x7100);
        stub(&mut e, NODE_WORLD_TRANSLATE, translation);
        stub_with(&mut e, RAGDOLL_GET_EYE_OFFSET, |e, a| {
            e.mem.set_f32(a[1] + 8, 99.0);
            ret(1)
        });
        let out = e.mem.alloc(12);
        assert_eq!(fn_008a2fa0(&mut e, actor, Ptr::new(out)), out);
        assert_eq!(e.mem.f32(out), 10.0);
        assert_eq!(e.mem.f32(out + 4), 20.0);
        // The ragdoll controller's eye offset replaces the height.
        assert_eq!(e.mem.f32(out + 8), 99.0);
    }

    #[test]
    fn fn_008a2fa0_without_a_head_node_raises_the_actors_position() {
        let mut e = engine();
        e.map(EYE_POSITION_DEFAULT, 12);
        stub_vector_add(&mut e);
        double_global(&mut e, EYE_HEIGHT_SHARE, 0.9);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_1E8_NODE, ret(0)),
                (ACTOR_SLOT_1D0, ret(0)),
                (ACTOR_SLOT_POSITION, ret(position)),
            ],
            None,
        );
        stub_float(&mut e, ACTOR_GET_HEIGHT, 2.0);
        let out = e.mem.alloc(12);
        fn_008a2fa0(&mut e, actor, Ptr::new(out));
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), (2.0f64 * 0.9 + 3.0) as f32);
    }

    #[test]
    fn fn_008a30f0_returns_the_global_word() {
        let mut e = engine();
        e.map(GLOBAL_11C61AC, 4);
        e.set_global(GLOBAL_11C61AC, 0x1234u32);
        assert_eq!(fn_008a30f0(&mut e), 0x1234);
    }

    /// A ragdoll controller block: a 0x100-byte zeroed object.
    fn ragdoll_block(e: &mut Engine) -> Ptr {
        Ptr::new(e.mem.alloc(0x100))
    }

    #[test]
    fn fn_008a3100_does_nothing_for_the_player_or_without_a_process() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let process = object(&mut e, 0x800, &[]);
        e.set(player, Actor::pCurrentProcess, process);
        fn_008a3100(&mut e, player, 0.5);
        let actor = actor_with(&mut e, &[], None);
        fn_008a3100(&mut e, actor, 0.5);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn fn_008a3100_marks_the_ragdoll_while_a_package_runs() {
        let mut e = engine();
        double_global(&mut e, ZERO_DOUBLE, 0.0);
        let player = actor_with(&mut e, &[], None);
        set_player(&mut e, player);
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_27C, ret(0x5000))]);
        let actor = actor_with(&mut e, &[], None);
        e.set(actor, Actor::pCurrentProcess, process);
        let ragdoll = ragdoll_block(&mut e);
        e.set(actor, Actor::pRagdollController, ragdoll);
        stub(&mut e, PACKAGE_TEST_67A690, 1);
        stub(&mut e, RAGDOLL_TEST_888A50, 1);
        fn_008a3100(&mut e, actor, 0.5);
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 1);
        assert_eq!(calls_to(&e, PACKAGE_TEST_67A690), vec![vec![0x5000]]);
    }

    /// An actor with a process, a tracked target and a ragdoll controller
    /// for the head tracking update; returns them with the acquire object.
    fn head_tracking_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr<Actor>, Ptr, Ptr) {
        double_global(e, ZERO_DOUBLE, 0.0);
        let player = actor_with(e, &[(ACTOR_SLOT_100, ret(0))], None);
        set_player(e, player);
        let eye = e.mem.alloc(12);
        let position = e.mem.alloc(12);
        let target = actor_with(
            e,
            &[
                (ACTOR_SLOT_FC, ret(0)),
                (ACTOR_SLOT_1D0, ret(1)),
                (ACTOR_SLOT_POSITION, ret(position)),
                (ACTOR_SLOT_194, ret(eye)),
            ],
            None,
        );
        let process = object(
            e,
            0x800,
            &[
                (PROCESS_SLOT_27C, ret(0)),
                (PROCESS_SLOT_264, float_ret(5.0)),
                (PROCESS_SLOT_268, ret(0)),
                (ACQUIRE_OBJECT_SLOT_678, ret(target.addr())),
                (PROCESS_SLOT_504, ret(0)),
                (PROCESS_SLOT_660, ret(0)),
                (PROCESS_SLOT_668, ret(0)),
                (PROCESS_SLOT_624, ret(0)),
            ],
        );
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_POSITION, ret(position)),
                (ACTOR_SLOT_1E8_NODE, ret(0)),
                (ACTOR_SLOT_21C_TEST, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        let ragdoll = ragdoll_block(e);
        e.set(actor, Actor::pRagdollController, ragdoll);
        e.mem.set_f32(actor.addr() + 0x74, 1.0);
        e.map(HEAD_TRACK_UPDATE_COUNTER, 4);
        stub_int_settings(e, &[(SETTING_INT_11DF848, 100)]);
        stub_bool_settings(e, &[]);
        stub_float_settings(
            e,
            &[
                (SETTING_11CD2F4, 1000.0),
                (0x0126_7d08, 100.0),
                (0x0126_7d14, 10.0),
            ],
        );
        stub(e, ACQUIRE_OBJECT_FIELD_28, 0);
        stub(e, ACTOR_SOUND_SOURCE, 1);
        stub(e, ACTOR_IS_VISIBLE, 1);
        stub(e, ANIMATION_GROUP_OF_SLOT, 4);
        stub(e, REFR_BASE_FORM, 0);
        stub_float(e, VECTOR_LENGTH, 50.0);
        stub(e, RAGDOLL_TEST_888A50, 1);
        stub_float(e, REFR_GET_DISTANCE_FROM_REFERENCE, 10.0);
        (actor, target, process, ragdoll)
    }

    #[test]
    fn fn_008a3100_points_the_ragdoll_at_the_target() {
        let mut e = engine();
        let (actor, target, process, ragdoll) = head_tracking_setup(&mut e);
        let cell = e.mem.alloc(4);
        stub(&mut e, FLOAT_ARRAY_SLOT, cell);
        e.mem.set_u8(ragdoll.addr() + 0xb2, 7);
        let eye = e.mem.alloc(12);
        let slot = slot_target(&e, target.cast(), ACTOR_SLOT_194);
        e.register_double(slot, move |_, _| ret(eye));
        fn_008a3100(&mut e, actor, 0.5);
        // The timers ran: the update timer 1.0 - 0.5, the process timer
        // 5.0 - 0.5.
        assert_eq!(e.mem.f32(actor.addr() + 0x74), 0.5);
        assert_eq!(
            calls_to(&e, slot_target(&e, process, PROCESS_SLOT_268)),
            vec![vec![process.addr(), 4.5f32.to_bits()]]
        );
        // The target is within the range (50 is between 10 and 100): the
        // ragdoll controller is reset and told where to look.
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 0);
        assert_eq!(
            calls_to(&e, RAGDOLL_FN_C75580),
            vec![vec![ragdoll.addr(), 1], vec![ragdoll.addr(), 1]]
        );
        let copies = calls_to(&e, VECTOR_ASSIGN);
        assert_eq!(copies, vec![vec![ragdoll.addr() + 0xd0, eye]]);
    }

    #[test]
    fn fn_008a3100_marks_the_ragdoll_when_the_target_is_out_of_range() {
        let mut e = engine();
        let (actor, _target, _process, ragdoll) = head_tracking_setup(&mut e);
        stub_float(&mut e, VECTOR_LENGTH, 500.0);
        fn_008a3100(&mut e, actor, 0.5);
        // Too far: nothing is copied, the ragdoll is marked.
        assert_eq!(call_count(&e, VECTOR_ASSIGN), 0);
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 0);
        // A busy actor (process level 1) marks it instead.
        stub(&mut e, PROCESS_LEVEL_NUMBER, 1);
        fn_008a3100(&mut e, actor, 0.5);
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 1);
    }

    #[test]
    fn fn_008a3100_picks_a_new_target_when_the_timer_runs_out() {
        let mut e = engine();
        let (actor, target, process, _ragdoll) = head_tracking_setup(&mut e);
        e.mem.set_f32(actor.addr() + 0x74, 0.25);
        e.map(EYE_POSITION_DEFAULT, 12);
        stub_list_functions(&mut e);
        // The process accepts a new target (slot 0x668) and the best
        // candidate is another actor.
        let tracking = slot_target(&e, process, PROCESS_SLOT_668);
        e.register_double(tracking, |_, _| ret(1));
        stub_with(&mut e, RANDOM_FLOAT, |_, _| float_ret(3.0));
        stub_float_settings(
            &mut e,
            &[
                (SETTING_11CD2F4, 1000.0),
                (0x0126_7d08, 100.0),
                (0x0126_7d14, 10.0),
                (SETTING_11CDBB8, 7.0),
            ],
        );
        // `fn_008a3ed0` finds nothing: the package test fails and the lists
        // are empty.
        stub(&mut e, PROCESS_ARRAY_COUNT, 0);
        fn_008a3100(&mut e, actor, 0.5);
        // The timer was set again by the random value; the target changed to
        // nobody (0), so the process was told.
        assert_eq!(e.mem.f32(actor.addr() + 0x74), 3.0);
        assert_eq!(e.mem.f32(actor.addr() + 0x158), 3.0);
        assert_eq!(
            calls_to(&e, slot_target(&e, process, PROCESS_SLOT_624)),
            vec![vec![process.addr(), 0]]
        );
        let _ = target;
        assert_eq!(e.global::<u32>(HEAD_TRACK_UPDATE_COUNTER), 1);
    }

    #[test]
    fn fn_008a3b30_tests_the_animation_flags_0x30() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[], None);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x10);
        assert_eq!(fn_008a3b30(&mut e, actor), 1);
        stub(&mut e, ACTOR_ANIM_FLAGS, 0x40);
        assert_eq!(fn_008a3b30(&mut e, actor), 0);
    }

    #[test]
    fn fn_008a3b50_reads_the_float_at_5fc() {
        let mut e = engine();
        let block = e.mem.alloc(0x800);
        e.mem.set_f32(block + 0x5fc, 2.5);
        assert_eq!(fn_008a3b50(&mut e, Ptr::new(block)), 2.5);
    }

    #[test]
    fn fn_008a3b70_copies_the_vector_and_resets_on_request() {
        let mut e = engine();
        let ragdoll = ragdoll_block(&mut e);
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, 9.0);
        stub(&mut e, FLOAT_ARRAY_SLOT, cell);
        e.mem.set_u8(ragdoll.addr() + 0xb2, 7);
        e.mem.set_u8(ragdoll.addr() + 0x43, 7);
        fn_008a3b70(&mut e, ragdoll, 0x6000, 0);
        assert_eq!(
            calls_to(&e, VECTOR_ASSIGN),
            vec![vec![ragdoll.addr() + 0xd0, 0x6000]]
        );
        assert_eq!(
            calls_to(&e, FLOAT_ARRAY_SLOT),
            vec![vec![ragdoll.addr() + 0xd0, 3]]
        );
        assert_eq!(e.mem.f32(cell), 0.0);
        // Without the flag the bytes stay.
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 7);
        fn_008a3b70(&mut e, ragdoll, 0x6000, 1);
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 0);
        assert_eq!(e.mem.u8(ragdoll.addr() + 0x43), 0);
        assert_eq!(
            calls_to(&e, RAGDOLL_FN_C75580),
            vec![vec![ragdoll.addr(), 1]]
        );
    }

    #[test]
    fn fn_008a3bd0_and_fn_008a3bf0_access_the_byte_at_b2() {
        let mut e = engine();
        let ragdoll = ragdoll_block(&mut e);
        assert_eq!(fn_008a3bd0(&mut e, ragdoll), 0);
        fn_008a3bf0(&mut e, ragdoll, 5);
        assert_eq!(fn_008a3bd0(&mut e, ragdoll), 5);
        assert_eq!(e.mem.u8(ragdoll.addr() + 0xb2), 5);
    }

    #[test]
    fn head_tracking_range_settings_are_read() {
        let mut e = engine();
        stub_float_settings(&mut e, &[(0x0126_7d08, 12.5), (0x0126_7d14, 3.5)]);
        assert_eq!(fn_008a3c10(&mut e), 12.5);
        assert_eq!(fn_008a3c30(&mut e), 3.5);
    }

    /// The setup for the face animation update: an actor in process level
    /// 0, a face animation data object and in range.
    fn face_update_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr) {
        double_global(e, ZERO_DOUBLE, 0.0);
        let player = actor_with(e, &[], None);
        set_player(e, player);
        let face = object(
            e,
            0x200,
            &[
                (FACE_ANIMATION_SLOT_B4, ret(0)),
                (FACE_ANIMATION_SLOT_114, ret(0)),
                (FACE_ANIMATION_SLOT_11C, ret(0)),
            ],
        );
        let actor = actor_with(e, &[], None);
        embedded_table(e, actor.addr() + 0xa4, &[(0x08, ret(55))]);
        stub(e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 0);
        stub(e, MOBILE_OBJECT_GET_CHAR_CONTROLLER, 0);
        stub(e, ACTOR_GET_FACE_ANIMATION_DATA, face.addr());
        stub_float_settings(e, &[(SETTING_11CD2F4, 1000.0)]);
        stub_float(e, REFR_GET_DISTANCE_FROM_REFERENCE, 10.0);
        (actor, face)
    }

    #[test]
    fn fn_008a3c50_sets_the_face_animation() {
        let mut e = engine();
        let (actor, face) = face_update_setup(&mut e);
        // The global flag is off: the neutral animation (slot 0xb4).
        stub(&mut e, GLOBAL_FLAG_5B7470, 0);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(
            calls_to(&e, slot_target(&e, face, FACE_ANIMATION_SLOT_B4)),
            vec![vec![face.addr(), 0, 1, 0, 0, 0, 0]]
        );
        // Fleeing: animation 1 at full strength.
        stub(&mut e, GLOBAL_FLAG_5B7470, 1);
        stub(&mut e, ACTOR_IS_FLEEING, 1);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(
            calls_to(&e, slot_target(&e, face, FACE_ANIMATION_SLOT_114)),
            vec![vec![face.addr(), 1, FLOAT_ONE_BITS]]
        );
        // In combat: animation 0xe.
        stub(&mut e, ACTOR_IS_FLEEING, 0);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        e.call_log = Some(vec![]);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(
            calls_to(&e, slot_target(&e, face, FACE_ANIMATION_SLOT_114)),
            vec![vec![face.addr(), 0xe, FLOAT_ONE_BITS]]
        );
        // Otherwise without a process: the value of the owner's slot 8.
        stub(&mut e, ACTOR_IN_COMBAT, 0);
        e.call_log = Some(vec![]);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(
            calls_to(&e, slot_target(&e, face, FACE_ANIMATION_SLOT_11C)),
            vec![vec![face.addr(), 55, 0]]
        );
    }

    #[test]
    fn fn_008a3c50_with_a_process_waits_for_the_sound_to_end() {
        let mut e = engine();
        let (actor, face) = face_update_setup(&mut e);
        stub(&mut e, GLOBAL_FLAG_5B7470, 1);
        let process = object(
            &mut e,
            0x800,
            &[(PROCESS_SLOT_48C, ret(0x6000)), (PROCESS_SLOT_2E8, ret(0))],
        );
        e.set(actor, Actor::pCurrentProcess, process);
        // No valid sound handle and the process slot 0x2e8 is 0: set.
        stub(&mut e, SOUND_HANDLE_IS_VALID_FN, 0);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(
            call_count(&e, slot_target(&e, face, FACE_ANIMATION_SLOT_11C)),
            1
        );
        assert_eq!(call_count(&e, SOUND_HANDLE_DESTROY), 1);
        // A valid handle: left alone.
        e.call_log = Some(vec![]);
        stub(&mut e, SOUND_HANDLE_IS_VALID_FN, 1);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(
            call_count(&e, slot_target(&e, face, FACE_ANIMATION_SLOT_11C)),
            0
        );
        // Not in process level 0: nothing at all.
        e.call_log = Some(vec![]);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 1);
        fn_008a3c50(&mut e, actor, 0);
        assert_eq!(call_count(&e, ACTOR_GET_FACE_ANIMATION_DATA), 0);
    }

    /// Doubles for the `BSSimpleList` functions the head tracking uses: the
    /// constructor clears the head, `append` puts the item in the head and
    /// moves the old item into a new node after it, the node test is true
    /// for an empty head, `next` reads the node's second word.
    fn stub_list_functions(e: &mut Engine) {
        stub_with(e, LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        stub_with(e, LIST_APPEND, |e, a| {
            let head = a[0];
            let item = e.mem.u32(a[1]);
            if e.mem.u32(head) != 0 {
                let node = e.mem.alloc(8);
                let old_item = e.mem.u32(head);
                let old_next = e.mem.u32(head + 4);
                e.mem.set_u32(node, old_item);
                e.mem.set_u32(node + 4, old_next);
                e.mem.set_u32(head + 4, node);
            }
            e.mem.set_u32(head, item);
            ret(0)
        });
        stub_with(e, LIST_NODE_STOP_TEST, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        stub_with(e, LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        stub_with(e, LIST_ITEM_SLOT, |_, a| ret(a[0]));
    }

    /// Maps the constants `actor_can_head_track` reads.
    fn head_track_constants(e: &mut Engine) {
        double_global(e, ZERO_DOUBLE, 0.0);
        double_global(e, HUNDRED, 100.0);
        double_global(e, PI_DOUBLE, std::f64::consts::PI);
        double_global(e, TWO_PI_DOUBLE, 2.0 * std::f64::consts::PI);
        double_global(e, MINUS_PI_DOUBLE, -std::f64::consts::PI);
        double_global(e, DETECTION_MAX_DISTANCE, 8192.0);
        e.map(PI_FLOAT, 4);
        e.set_global(PI_FLOAT, std::f32::consts::PI);
        e.map(TURN_LIMIT_FLOAT, 4);
        e.set_global(TURN_LIMIT_FLOAT, 2.0943952f32);
        stub_float_settings(e, &[(SETTING_11CD938, 1000.0), (SETTING_11CD114, 0.0)]);
        double_global(e, THOUSAND_DOUBLE, 1000.0);
        double_global(e, HALF_DOUBLE, 0.5);
        let rotation = e.mem.alloc(16);
        stub(e, ACTOR_ROTATION_POINTER, rotation);
        stub_float(e, GET_Z_ANGLE_FROM_VECTOR, 0.0);
        stub_float(e, ABSOLUTE_VALUE_ALT, 0.0);
    }

    /// An actor that `actor_can_head_track` accepts when `this` asks about
    /// it from `distance` units away; its record in the shared acquire object
    /// is positive.
    fn head_track_candidate(e: &mut Engine) -> Ptr<Actor> {
        actor_with(
            e,
            &[
                (ACTOR_SLOT_100, ret(1)),
                (ACTOR_SLOT_1D0, ret(1)),
                (ACTOR_SLOT_214, ret(0)),
                (ACTOR_SLOT_22C, ret(0)),
            ],
            None,
        )
    }

    /// The acquire object with a positive detection record.
    fn detecting_acquire(e: &mut Engine) -> Ptr {
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record + 8, 5);
        e.mem.set_u8(record + 0x1c, 1);
        e.mem.set_u8(record + 0x1e, 1);
        shared_acquire(
            e,
            &[
                (ACQUIRE_OBJECT_SLOT_504, ret(record)),
                (ACQUIRE_OBJECT_SLOT_678, ret(1)),
            ],
        )
    }

    #[test]
    fn actor_can_head_track_accepts_a_visible_actor_in_range() {
        let mut e = engine();
        head_track_constants(&mut e);
        let player = actor_with(&mut e, &[(ACTOR_SLOT_100, ret(0))], None);
        set_player(&mut e, player);
        detecting_acquire(&mut e);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_214, ret(0))], None);
        let process = object(&mut e, 0x800, &[(ACQUIRE_OBJECT_SLOT_678, ret(1))]);
        e.set(actor, Actor::pCurrentProcess, process);
        let other = head_track_candidate(&mut e);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 200.0);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 1), 1);
        // The distance is asked from the other actor to this one.
        assert_eq!(
            calls_to(&e, REFR_GET_DISTANCE_FROM_REFERENCE)[0],
            vec![other.addr(), actor.addr(), 0, 1]
        );
        // Without the detection check the detection is not asked.
        e.call_log = Some(vec![]);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 0), 1);
        assert_eq!(call_count(&e, GET_SAVED_ACQUIRE_OBJECT), 0);
    }

    #[test]
    fn actor_can_head_track_refuses_the_cases_the_code_lists() {
        let mut e = engine();
        head_track_constants(&mut e);
        let player = actor_with(&mut e, &[(ACTOR_SLOT_100, ret(0))], None);
        set_player(&mut e, player);
        detecting_acquire(&mut e);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_214, ret(0))], None);
        let process = object(
            &mut e,
            0x800,
            &[
                (ACQUIRE_OBJECT_SLOT_678, ret(1)),
                (PROCESS_SLOT_68C, ret(0)),
            ],
        );
        e.set(actor, Actor::pCurrentProcess, process);
        let other = head_track_candidate(&mut e);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 200.0);
        // Itself.
        assert_eq!(actor_can_head_track(&mut e, actor, actor, 0), 0);
        // An actor that fails the test of slot 0x100.
        assert_eq!(actor_can_head_track(&mut e, actor, player, 0), 0);
        // Without a 3D object.
        let blind = actor_with(
            &mut e,
            &[(ACTOR_SLOT_100, ret(1)), (ACTOR_SLOT_1D0, ret(0))],
            None,
        );
        assert_eq!(actor_can_head_track(&mut e, actor, blind, 0), 0);
        // Too far away (the limit is 1000).
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 1500.0);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 0), 0);
        // Closer than 100 units but neither the player nor remembered.
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 50.0);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 0), 0);
        // Remembered by the process (slot 0x68c): accepted.
        let remembered = slot_target(&e, process, PROCESS_SLOT_68C);
        let other_addr = other.addr();
        e.register_double(remembered, move |_, _| ret(other_addr));
        assert_eq!(actor_can_head_track(&mut e, actor, other, 0), 1);
        // Behind the actor: the angle beyond the turn limit.
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 200.0);
        stub_float(&mut e, ABSOLUTE_VALUE_ALT, 3.0);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 0), 0);
        // Two actors that both have an attached-arrow list (slot 0x214).
        stub_float(&mut e, ABSOLUTE_VALUE_ALT, 0.0);
        let arrows = actor_with(
            &mut e,
            &[
                (ACTOR_SLOT_100, ret(1)),
                (ACTOR_SLOT_1D0, ret(1)),
                (ACTOR_SLOT_214, ret(1)),
            ],
            None,
        );
        let slot = slot_target(&e, actor.cast(), ACTOR_SLOT_214);
        e.register_double(slot, |_, _| ret(1));
        assert_eq!(actor_can_head_track(&mut e, actor, arrows, 0), 0);
    }

    #[test]
    fn actor_can_head_track_needs_a_positive_detection_level() {
        let mut e = engine();
        head_track_constants(&mut e);
        let player = actor_with(&mut e, &[(ACTOR_SLOT_100, ret(0))], None);
        set_player(&mut e, player);
        let acquire = detecting_acquire(&mut e);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_214, ret(0))], None);
        let process = object(&mut e, 0x800, &[(ACQUIRE_OBJECT_SLOT_678, ret(1))]);
        e.set(actor, Actor::pCurrentProcess, process);
        let other = head_track_candidate(&mut e);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 200.0);
        // The record's level is 0: not detected.
        let record = e.mem.alloc(0x40);
        let slot = slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_504);
        e.register_double(slot, move |_, _| ret(record));
        assert_eq!(actor_can_head_track(&mut e, actor, other, 1), 0);
        // Detected, but an actor flagged by `004938e0` that was not seen.
        e.mem.set_u32(record + 8, 5);
        e.mem.set_u8(record + 0x1c, 0);
        e.mem.set_u8(record + 0x1e, 0);
        stub(&mut e, ACTOR_FLAG_4938E0, 1);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 1), 0);
        // Seen: accepted.
        e.mem.set_u8(record + 0x1e, 1);
        assert_eq!(actor_can_head_track(&mut e, actor, other, 1), 1);
    }

    #[test]
    fn fn_008a3ed0_prefers_the_package_target_within_range() {
        let mut e = engine();
        head_track_constants(&mut e);
        stub_list_functions(&mut e);
        e.map(PLAYER_TRACK_DISTANCE, 4);
        e.set_global(PLAYER_TRACK_DISTANCE, 500.0f32);
        e.map(OTHER_TRACK_DISTANCE, 4);
        e.set_global(OTHER_TRACK_DISTANCE, 100.0f32);
        let player = actor_with(&mut e, &[(ACTOR_SLOT_100, ret(0))], None);
        set_player(&mut e, player);
        let process = object(&mut e, 0x800, &[(PROCESS_SLOT_278, ret(0x6000))]);
        let actor = actor_with(&mut e, &[], None);
        e.set(actor, Actor::pCurrentProcess, process);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, 0x5000);
        stub(&mut e, PACKAGE_TYPE, 6);
        stub(&mut e, PACKAGE_GET_TARGET_OBJECT, 1);
        stub(&mut e, ENTRY_FORM, 0x7000);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 50.0);
        assert_eq!(fn_008a3ed0(&mut e, actor), 0x7000);
        assert_eq!(
            calls_to(&e, REFR_GET_DISTANCE_FROM_REFERENCE),
            vec![vec![actor.addr(), 0x7000, 0, 1]]
        );
        // Beyond the limit the lists are searched; nobody is there.
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 150.0);
        stub(&mut e, PROCESS_ARRAY_COUNT, 0);
        assert_eq!(fn_008a3ed0(&mut e, actor), 0);
        // The player's own limit is larger.
        stub(&mut e, ENTRY_FORM, player.addr());
        assert_eq!(fn_008a3ed0(&mut e, actor), player.addr());
    }

    #[test]
    fn fn_008a3ed0_picks_the_best_scoring_candidate() {
        let mut e = engine();
        head_track_constants(&mut e);
        stub_list_functions(&mut e);
        let player = actor_with(&mut e, &[(ACTOR_SLOT_100, ret(0))], None);
        set_player(&mut e, player);
        detecting_acquire(&mut e);
        let process = object(&mut e, 0x800, &[(ACQUIRE_OBJECT_SLOT_678, ret(1))]);
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_214, ret(0))], None);
        e.set(actor, Actor::pCurrentProcess, process);
        let near = head_track_candidate(&mut e);
        let far = head_track_candidate(&mut e);
        let (near_addr, far_addr, actor_addr) = (near.addr(), far.addr(), actor.addr());
        stub_with(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, move |_, a| {
            float_ret(if a[0] == far_addr && a[1] == actor_addr {
                300.0
            } else if a[0] == near_addr && a[1] == actor_addr {
                200.0
            } else {
                100.0
            })
        });
        stub(&mut e, PROCESS_LISTS_GET_ARRAY, 0x8000);
        stub(&mut e, PROCESS_ARRAY_COUNT, 2);
        stub_with(&mut e, PROCESS_ARRAY_GET_ACTOR, move |_, a| {
            ret(if a[1] == 0 { far_addr } else { near_addr })
        });
        stub(&mut e, ACTOR_LINE_OF_SIGHT, 1);
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 0);
        assert_eq!(fn_008a3ed0(&mut e, actor), near_addr);
        // Both were appended and the list was cleaned up.
        assert_eq!(call_count(&e, LIST_APPEND), 2);
        assert_eq!(call_count(&e, LIST_CLEAR), 1);
        assert_eq!(call_count(&e, LIST_DESTRUCT), 1);
    }

    #[test]
    fn fn_008a46a0_tests_the_form_flag() {
        let mut e = engine();
        stub(&mut e, FORM_TEST_FLAGS_FN, 1);
        assert_eq!(fn_008a46a0(&mut e, Ptr::new(0x5030)), 1);
        assert_eq!(
            calls_to(&e, FORM_TEST_FLAGS_FN),
            vec![vec![0x5030, 0x4000_0000]]
        );
    }

    #[test]
    fn fn_008a46c0_scores_a_target_by_distance_and_state() {
        let mut e = engine();
        head_track_constants(&mut e);
        let acquire = shared_acquire(&mut e, &[(ACQUIRE_OBJECT_SLOT_678, ret(0))]);
        let actor = actor_with(&mut e, &[], None);
        let other = actor_with(
            &mut e,
            &[(ACTOR_SLOT_100, ret(0)), (ACTOR_SLOT_22C, ret(0))],
            None,
        );
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 0);
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 200.0);
        e.mem.set_f32(actor.addr() + 0x158, 0.5);
        // (1000 - 200) / (1000 - 0) = 0.8 for a target without a line of
        // sight test.
        assert_eq!(fn_008a46c0(&mut e, actor, other), 0.8f32);
        assert_eq!(
            calls_to(&e, REFR_GET_DISTANCE_FROM_REFERENCE),
            vec![vec![other.addr(), actor.addr(), 0, 0]]
        );
        // The actor the acquire object targets: times (timer + 1).
        let slot = slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_678);
        let other_addr = other.addr();
        e.register_double(slot, move |_, _| ret(other_addr));
        let expected = ((0.5f32 as f64 + 1.0) * 0.8f32 as f64) as f32;
        assert_eq!(fn_008a46c0(&mut e, actor, other), expected);
        // A fleeing target is halved.
        let fleeing = slot_target(&e, other.cast(), ACTOR_SLOT_22C);
        e.register_double(fleeing, |_, _| ret(1));
        assert_eq!(
            fn_008a46c0(&mut e, actor, other),
            (expected as f64 * 0.5) as f32
        );
        // Without a line of sight the score is halved first.
        let other_two = actor_with(
            &mut e,
            &[(ACTOR_SLOT_100, ret(1)), (ACTOR_SLOT_22C, ret(0))],
            None,
        );
        stub(&mut e, ACTOR_LINE_OF_SIGHT, 0);
        assert_eq!(fn_008a46c0(&mut e, actor, other_two), 0.4f32);
        // No usable acquire object: 0.
        stub(&mut e, ACQUIRE_OBJECT_FIELD_28, 1);
        assert_eq!(fn_008a46c0(&mut e, actor, other), 0.0);
    }

    /// An actor that `fn_008a40e0` accepts when no package runs, with the
    /// package of its process given by `package`. Returns the actor, the
    /// actor it targets and the process.
    fn idle_check_setup(e: &mut Engine, package: u32) -> (Ptr<Actor>, u32, Ptr) {
        let player = actor_with(e, &[], None);
        set_player(e, player);
        head_track_constants(e);
        e.map(FORM_POINTER_11CA248, 4);
        e.set_global(FORM_POINTER_11CA248, 0x7777u32);
        shared_acquire(
            e,
            &[
                (ACQUIRE_OBJECT_SLOT_678, ret(0x5555)),
                (ACQUIRE_OBJECT_SLOT_504, ret(0)),
                (PROCESS_SLOT_674, ret(0)),
            ],
        );
        let process = object(e, 0x800, &[(PROCESS_SLOT_22C, ret(package))]);
        let actor = actor_with(
            e,
            &[
                (ACTOR_SLOT_1D0, ret(1)),
                (ACTOR_SLOT_214, ret(0)),
                (ACTOR_SLOT_1E8_NODE, ret(0)),
                (ACTOR_SLOT_21C_TEST, ret(0)),
                (ACTOR_SLOT_1E4, ret(0)),
            ],
            None,
        );
        e.set(actor, Actor::pCurrentProcess, process);
        stub(e, ACTOR_IS_PATHING_COMPLETE, 1);
        (actor, 0x5000, process)
    }

    #[test]
    fn fn_008a40e0_refuses_the_cases_the_code_lists() {
        let mut e = engine();
        let (actor, other, _process) = idle_check_setup(&mut e, 0);
        // The player never.
        let player = Ptr::<Actor>::new(e.global::<u32>(PLAYER_CHARACTER));
        assert_eq!(fn_008a40e0(&mut e, player, other), 0);
        // Accepted without a package.
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
        // A base form with the flag.
        stub(&mut e, ACTOR_BASE_FORM_FN, 0x6000);
        stub(&mut e, FORM_TEST_FLAGS_FN, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        assert_eq!(
            calls_to(&e, FORM_TEST_FLAGS_FN),
            vec![vec![0x6030, 0x4000_0000]]
        );
        stub(&mut e, ACTOR_BASE_FORM_FN, 0);
        // Arrows attached (slot 0x214).
        let arrows = slot_target(&e, actor.cast(), ACTOR_SLOT_214);
        e.register_double(arrows, |_, _| ret(1));
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        e.register_double(arrows, |_, _| ret(0));
        // Not at the end of the path, or in combat.
        stub(&mut e, ACTOR_IS_PATHING_COMPLETE, 0);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, ACTOR_IS_PATHING_COMPLETE, 1);
        stub(&mut e, ACTOR_IN_COMBAT, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, ACTOR_IN_COMBAT, 0);
        // During an attack animation.
        stub(&mut e, ACTOR_GET_ANIMATION, 0x6100);
        stub(&mut e, ANIMATION_GROUP_OF_SLOT, 4);
        stub(&mut e, IS_ATTACK_ACTION, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, IS_ATTACK_ACTION, 0);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
        // The acquire object targets the actor itself: refused.
        let acquire_now = Ptr::new(e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32());
        let targeted = slot_target(&e, acquire_now, ACQUIRE_OBJECT_SLOT_678);
        let actor_addr = actor.addr();
        e.register_double(targeted, move |_, _| ret(actor_addr));
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
    }

    #[test]
    fn fn_008a40e0_asks_for_the_detection_of_the_player() {
        let mut e = engine();
        let (actor, _other, _process) = idle_check_setup(&mut e, 0);
        // The player moves and is the other actor: the detection decides.
        let player = actor_with(&mut e, &[(ACTOR_SLOT_1D0, ret(1))], None);
        set_player(&mut e, player);
        stub(&mut e, ACTOR_IS_MOVING, 1);
        stub(&mut e, PLAYER_IS_IN_COMBAT, 1);
        let record = e.mem.alloc(0x40);
        let acquire = Ptr::new(e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32());
        let slot = slot_target(&e, acquire, ACQUIRE_OBJECT_SLOT_504);
        e.register_double(slot, move |_, _| ret(record));
        stub_float(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, 100.0);
        // A record with level 0: not detected, refused.
        assert_eq!(fn_008a40e0(&mut e, actor, player.addr()), 0);
        assert_eq!(
            calls_to(&e, PLAYER_IS_IN_COMBAT),
            vec![vec![player.addr(), 0]]
        );
        // Detected: accepted.
        e.mem.set_u32(record + 8, 5);
        assert_eq!(fn_008a40e0(&mut e, actor, player.addr()), 1);
    }

    #[test]
    fn fn_008a40e0_handles_the_package_types() {
        let mut e = engine();
        let (actor, other, process) = idle_check_setup(&mut e, 0x5100);
        // Types that always refuse: 8, 0xf, 0x10, 0x14 and 0x1c.
        for package_type in [8, 0xf, 0x10, 0x14, 0x1c] {
            stub(&mut e, PACKAGE_TYPE, package_type);
            assert_eq!(fn_008a40e0(&mut e, actor, other), 0, "{package_type:#x}");
        }
        // Types without an own rule.
        for package_type in [1, 3, 4, 7, 9, 0x1d, 0x30] {
            stub(&mut e, PACKAGE_TYPE, package_type);
            assert_eq!(fn_008a40e0(&mut e, actor, other), 1, "{package_type:#x}");
        }
        // Types 0 and 2 refuse while a movement animation flag is set.
        for package_type in [0, 2] {
            stub(&mut e, PACKAGE_TYPE, package_type);
            stub(&mut e, ACTOR_ANIM_FLAGS, 0);
            assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
            stub(&mut e, ACTOR_ANIM_FLAGS, 4);
            assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        }
        stub(&mut e, ACTOR_ANIM_FLAGS, 0);
        // Type 5 also waits for the special idle to finish.
        stub(&mut e, PACKAGE_TYPE, 5);
        let animation_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_1E4);
        e.register_double(animation_slot, |_, _| ret(0x6200));
        stub(&mut e, ANIMATION_SPECIAL_IDLE_DONE_PLAYING, 0);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, ANIMATION_SPECIAL_IDLE_DONE_PLAYING, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
        e.register_double(animation_slot, |_, _| ret(0));
        // Type 6: an acquire object that accepts at once ends it with 1.
        stub(&mut e, PACKAGE_TYPE, 6);
        stub(&mut e, PACKAGE_GET_LOCATION_FN, 0x5400);
        stub(&mut e, PACKAGE_LOCATION_GET_LOC_REFERENCE, 0x5500);
        stub(&mut e, REFR_BASE_FORM, 0x7777);
        // The location's form is the remembered form: refused.
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, REFR_BASE_FORM, 0x7778);
        stub(&mut e, FORM_TYPE, 0x30);
        // A location of kind 3 refuses.
        stub(&mut e, PACKAGE_LOCATION_GET_LOC_TYPE_FN, 3);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, PACKAGE_LOCATION_GET_LOC_TYPE_FN, 0);
        // A furniture other actor refuses.
        stub(&mut e, REFR_IS_FURNITURE, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, REFR_IS_FURNITURE, 0);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
        let _ = process;
    }

    #[test]
    fn fn_008a40e0_type_0xc_needs_the_process_object_to_accept() {
        let mut e = engine();
        let (actor, other, _process) = idle_check_setup(&mut e, 0x5100);
        stub(&mut e, PACKAGE_TYPE, 0xc);
        // No object: refused.
        stub(&mut e, PROCESS_OBJECT_8D0430, 0);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        // An object that does not accept the actor: refused.
        stub(&mut e, PROCESS_OBJECT_8D0430, 0x6300);
        stub(&mut e, OBJECT_TEST_9F4300, 0);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        // Accepted, with a furniture reference: refused.
        stub(&mut e, OBJECT_TEST_9F4300, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
        assert_eq!(
            calls_to(&e, OBJECT_TEST_9F4300),
            vec![vec![0x6300, actor.addr(), 1]; 2]
        );
        stub(&mut e, OBJECT_REFERENCE_639B40, 0x6400);
        stub(&mut e, REFR_IS_FURNITURE, 1);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, REFR_IS_FURNITURE, 0);
        stub(&mut e, REFR_BASE_FORM, 0x7777);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, REFR_BASE_FORM, 0x7778);
        stub(&mut e, FORM_TYPE, 0x30);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, FORM_TYPE, 0x31);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
        // A running animation of the object kind 3 refuses.
        let animation_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_1E4);
        e.register_double(animation_slot, |_, _| ret(0x6200));
        stub(&mut e, ANIMATION_SPECIAL_IDLE_DONE_PLAYING, 0);
        stub(&mut e, ENTRY_FORM, 3);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 0);
        stub(&mut e, ENTRY_FORM, 4);
        assert_eq!(fn_008a40e0(&mut e, actor, other), 1);
    }

    /// The objects of a hit: the player as the aggressor, a victim with a
    /// process and the hit data. Every global and setting the hit reads is
    /// mapped; the callees are the default doubles.
    struct HitWorld {
        player: Ptr<Actor>,
        victim: Ptr<Actor>,
        process: Ptr,
        acquire: Ptr,
        hit: Ptr<HitData>,
    }

    fn hit_world(e: &mut Engine, kills: bool) -> HitWorld {
        let player_position = e.mem.alloc(12);
        double_global(e, ZERO_DOUBLE, 0.0);
        double_global(e, THOUSAND_DOUBLE, 1000.0);
        double_global(e, HUNDRED, 100.0);
        double_global(e, ONE_HUNDREDTH, 0.01);
        for global in [
            DEFAULT_WEAPON_FORM,
            HIT_BARK_DEADLINE,
            CONTROLS_HOLDER,
            COMBAT_DIALOGUE_MANAGER,
            DETECTION_FLOAT_TWO,
        ] {
            e.map(global, 4);
        }
        e.map(LIMB_WORD_TABLE, 0x200);
        stub_float_settings(e, &[]);
        stub_int_settings(e, &[]);
        stub_bool_settings(e, &[]);
        stub_ftol(e);
        stub(e, IMPORT_GET_TICK_COUNT, 1000);
        stub(e, REFR_PARENT_CELL, 0x9000);
        stub(e, CELL_TEST_450FF0, 1);
        stub(e, OBJECT_011DEA0C_GET, 0x9100);
        let player = actor_with(
            e,
            &[
                (ACTOR_SLOT_360, ret(1)),
                (ACTOR_SLOT_22C, ret(0)),
                (ACTOR_SLOT_1D0, ret(0x7100)),
                (ACTOR_SLOT_POSITION, ret(player_position)),
            ],
            None,
        );
        set_player(e, player);
        let position = e.mem.alloc(12);
        let acquire = shared_acquire(
            e,
            &[
                (PROCESS_SLOT_3E8, ret(0)),
                (PROCESS_SLOT_CURRENT_WEAPON, ret(0)),
                (PROCESS_SLOT_WEAPON_LAST_POS, ret(0)),
                (PROCESS_SLOT_5C8, ret(0)),
                (PROCESS_SLOT_30C, ret(0)),
            ],
        );
        let victim = actor_with(
            e,
            &[
                (ACTOR_SLOT_360, ret(0)),
                (ACTOR_SLOT_214, ret(0)),
                (ACTOR_SLOT_22C, ret(0)),
                (ACTOR_SLOT_APPLY_DAMAGE, ret(kills as u32)),
                (ACTOR_SLOT_1D0, ret(0x7000)),
                (ACTOR_SLOT_428, ret(0)),
                (ACTOR_SLOT_230, ret(0)),
                (ACTOR_SLOT_HIT_NOTICE, ret(0)),
                (ACTOR_SLOT_HIT_ME_OV2, ret(0)),
                (ACTOR_SLOT_DAMAGE_EQUIPMENT, ret(0)),
                (ACTOR_SLOT_POSITION, ret(position)),
                (ACTOR_SLOT_DAMAGE_LIMB, ret(0)),
                (ACTOR_SLOT_1E4, ret(0)),
            ],
            None,
        );
        embedded_table(e, victim.addr() + 0x94, &[(0x08, ret(0))]);
        embedded_table(
            e,
            victim.addr() + 0xa4,
            &[(0x08, ret(0)), (0x0c, float_ret(0.0))],
        );
        let process = object(e, 0x800, &[(PROCESS_SLOT_CURRENT_WEAPON, ret(0))]);
        e.set(victim, Actor::pCurrentProcess, process);
        let hit = e.new_object::<HitData>();
        e.set(hit, HitData::pAggressor, player.cast());
        e.set(hit, HitData::pTarget, victim.cast());
        e.set(hit, HitData::eDamageLimb, -1);
        e.set(hit, HitData::fHealthDamage, 12.0);
        e.set(hit, HitData::fTotalDamage, 15.0);
        HitWorld {
            player,
            victim,
            process,
            acquire,
            hit,
        }
    }

    #[test]
    fn actor_hit_me_ignores_actors_without_a_process_and_missing_hits() {
        let mut e = engine();
        let world = hit_world(&mut e, false);
        // No process: nothing at all.
        let idle = actor_with(&mut e, &[], None);
        actor_hit_me(&mut e, idle, world.hit, 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        // No hit data: only the bracket function runs.
        actor_hit_me(&mut e, world.victim, Ptr::NULL, 0);
        assert_eq!(
            calls_to(&e, ACTOR_HIT_BRACKET_8A5300),
            vec![vec![world.victim.addr()]]
        );
        assert_eq!(call_count(&e, SCRIPT_SET_ACTION_FLAG), 0);
    }

    #[test]
    fn actor_hit_me_only_queues_the_hit_when_the_task_queue_is_used() {
        let mut e = engine();
        let world = hit_world(&mut e, false);
        stub(&mut e, PICK_UP_GOES_TO_TASK_QUEUE, 1);
        stub(&mut e, TASK_QUEUE_INTERFACE, 0x7300);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        assert_eq!(
            calls_to(&e, QUEUE_ACTOR_HIT),
            vec![vec![0x7300, world.victim.addr(), world.hit.addr()]]
        );
        assert_eq!(call_count(&e, SCRIPT_SET_ACTION_FLAG), 0);
        assert_eq!(call_count(&e, ACTOR_HIT_BRACKET_8A5300), 1);
    }

    #[test]
    fn actor_hit_me_applies_a_lethal_hit_of_the_player() {
        let mut e = engine();
        let world = hit_world(&mut e, true);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        // The damage was applied through slot 0x338 with the health,
        // fatigue and the aggressor.
        assert_eq!(
            calls_to(
                &e,
                slot_target(&e, world.victim.cast(), ACTOR_SLOT_APPLY_DAMAGE)
            ),
            vec![vec![
                world.victim.addr(),
                12.0f32.to_bits(),
                0.0f32.to_bits(),
                world.player.addr()
            ]]
        );
        // The script action flags of the aggressor are set.
        let flags = calls_to(&e, SCRIPT_SET_ACTION_FLAG);
        assert_eq!(flags[0][0], world.player.addr());
        assert_eq!(flags[0][2], 0x80);
        assert_eq!(flags[1][2], 0x100);
        // The death handler runs with the category 3 (no source) and the
        // kill is counted for the player.
        assert_eq!(
            calls_to(&e, ACTOR_DEATH_HANDLER),
            vec![vec![world.victim.addr(), world.hit.addr(), 0, 3]]
        );
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x24]]);
        // The player's hit feedback: one rumble.
        assert_eq!(call_count(&e, CONTROLS_RUMBLE), 1);
        // The hit is bracketed by the two 0x008a5300 calls.
        assert_eq!(call_count(&e, ACTOR_HIT_BRACKET_8A5300), 2);
        let _ = (world.process, world.acquire);
    }

    /// Makes `HitData::HasFlag` (`0058cba0`) test the hit's flags.
    fn stub_hit_flag_test(e: &mut Engine) {
        stub_with(e, HIT_DATA_FLAG_TEST, |e, a| {
            ret(((e.mem.u32(a[0] + 0x58) & a[1]) != 0) as u32)
        });
    }

    #[test]
    fn actor_hit_me_damages_the_limb_the_hit_names() {
        let mut e = engine();
        let world = hit_world(&mut e, false);
        let base = object(
            &mut e,
            0x400,
            &[(BASE_FORM_SLOT_BODY_PART_DATA, ret(0xc000))],
        );
        stub(&mut e, ACTOR_BASE_FORM_FN, base.addr());
        stub(&mut e, BODY_PART_DATA_GET_BODY_PART, 0xd000);
        stub(&mut e, BODY_PART_LIMB_NUMBER, 3);
        e.set(world.hit, HitData::eDamageLimb, 3);
        e.set(world.hit, HitData::fTargetedLimbDamage, 5.0);
        // The limb has 10 health before the damage and 0 after.
        let reads = std::rc::Rc::new(std::cell::Cell::new(0));
        let counter = reads.clone();
        let table = e.mem.alloc(0x800);
        let current = e.mem.alloc(8);
        e.register_double(current, move |_, a| {
            if a[1] == 3 {
                counter.set(counter.get() + 1);
                float_ret(if counter.get() == 1 { 10.0 } else { 0.0 })
            } else {
                float_ret(0.0)
            }
        });
        e.mem.set_u32(table + 0x0c, current);
        let integer = e.mem.alloc(8);
        e.register_double(integer, |_, _| ret(0));
        e.mem.set_u32(table + 0x08, integer);
        e.mem.set_u32(world.victim.addr() + 0xa4, table);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        assert_eq!(
            calls_to(
                &e,
                slot_target(&e, world.victim.cast(), ACTOR_SLOT_DAMAGE_LIMB)
            ),
            vec![vec![
                world.victim.addr(),
                3,
                (-5.0f32).to_bits(),
                world.player.addr()
            ]]
        );
        // The entry point 6 was asked to adjust the damage.
        let entry = calls_to(&e, HANDLE_ENTRY_POINT);
        assert!(entry
            .iter()
            .any(|call| call[..3] == [6, world.victim.addr(), world.player.addr()]));
    }

    #[test]
    fn actor_hit_me_casts_the_critical_effect() {
        let mut e = engine();
        let world = hit_world(&mut e, false);
        stub_hit_flag_test(&mut e);
        embedded_table(
            &mut e,
            world.victim.addr() + 0x88,
            &[(MAGIC_CASTER_SLOT_SET_SPELL, ret(0))],
        );
        e.set(world.hit, HitData::uiFlags, 4);
        e.set(world.hit, HitData::pCriticalEffect, Ptr::new(0x6600));
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        let caster = world.victim.addr() + 0x88;
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_INTERRUPT_CAST),
            vec![vec![caster]]
        );
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_FIND_TARGETS),
            vec![vec![caster, 1.0f32.to_bits(), 0, 0, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, MAGIC_TARGET_FN_824110),
            vec![vec![world.victim.addr() + 0x94, 0x6618]]
        );
        // The setting that suppresses the cast turns it off.
        e.call_log = Some(vec![]);
        stub_bool_settings(&mut e, &[(SETTING_SKIP_CRITICAL_EFFECT, 1)]);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        assert_eq!(call_count(&e, MAGIC_CASTER_INTERRUPT_CAST), 0);
    }

    #[test]
    fn actor_hit_me_knocks_back_an_actor_with_arrows() {
        let mut e = engine();
        let world = hit_world(&mut e, false);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let player_position = slot_target(&e, world.player.cast(), ACTOR_SLOT_POSITION);
        e.register_double(player_position, move |_, _| ret(position));
        let arrows = slot_target(&e, world.victim.cast(), ACTOR_SLOT_214);
        e.register_double(arrows, |_, _| ret(1));
        // The acquire object reports a furniture marker number above 0x14 and
        // no furniture.
        let acquire_table = e.mem.u32(world.acquire.addr());
        for (slot, value) in [
            (PROCESS_SLOT_4C4, 0x20u32),
            (PROCESS_SLOT_20C, 0),
            (PROCESS_SLOT_4C8, 0),
        ] {
            let function = e.mem.alloc(8);
            e.register_double(function, move |_, _| ret(value));
            e.mem.set_u32(acquire_table + slot, function);
        }
        let knock = e.mem.alloc(8);
        e.register_double(knock, |_, _| ret(0));
        let process_table = e.mem.alloc(0x800);
        e.mem
            .set_u32(process_table + PROCESS_SLOT_KNOCK_EXPLOSION, knock);
        e.mem.set_u32(world.process.addr(), process_table);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        assert_eq!(
            calls_to(&e, MOBILE_OBJECT_SET_CHASE_BIP),
            vec![vec![world.victim.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_GET_OUT_OF_FURNITURE_QUICK),
            vec![vec![world.victim.addr()]]
        );
        // The victim is knocked away from the aggressor at speed 1.0.
        assert_eq!(
            calls_to(&e, knock)[0],
            vec![
                world.process.addr(),
                world.victim.addr(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1.0f32.to_bits()
            ]
        );
    }

    #[test]
    fn actor_hit_me_uses_the_blocked_hit_paths() {
        let mut e = engine();
        let world = hit_world(&mut e, false);
        e.set(world.hit, HitData::fPercentBlocked, 0.5);
        stub(&mut e, INTERFACE_IS_IN_MENU_MODE, 0);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        // A blocked hit by the player rumbles once and skips the damage hooks.
        assert_eq!(call_count(&e, CONTROLS_RUMBLE), 1);
        assert_eq!(call_count(&e, ACTOR_FN_88E8D0), 0);
        // A hit that is not blocked runs them.
        e.call_log = Some(vec![]);
        e.set(world.hit, HitData::fPercentBlocked, 0.0);
        actor_hit_me(&mut e, world.victim, world.hit, 0);
        assert_eq!(call_count(&e, ACTOR_FN_88E8D0), 1);
    }

    /// Doubles for everything `actor_hit_me_ov2` calls and the exe data it
    /// reads: the settings give `first = 100 * percent * 0.01` and
    /// `second = 2 + 8 * second_amount`. Returns the actor that is hit and
    /// the source actor (its slot `0x360` returns `blue`).
    fn hit_ov2_setup(e: &mut Engine, blue: u32) -> (Ptr<Actor>, Ptr<Actor>) {
        for (address, value) in [
            (HIT_LIMIT_DOUBLE, 100.0f64),
            (HIT_PERCENT_SCALE, 0.01),
            (HIT_SECOND_CUTOFF, 75.0),
            (HIT_ZERO_DOUBLE, 0.0),
            (HIT_ONE_DOUBLE, 1.0),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        for (address, value) in [
            (HIT_LIMIT_FLOAT, 100.0f32),
            (HIT_REACH, 30.0),
            (ARROW_WIDTH_SCALE, 0.1),
        ] {
            e.map(address, 4);
            e.set_global(address, value);
        }
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x8300u32);
        let this = actor_with(e, &[(ACTOR_SLOT_230, ret(0))], None);
        let source = actor_with(
            e,
            &[(ACTOR_SLOT_1D0, ret(0x8100)), (ACTOR_SLOT_360, ret(blue))],
            None,
        );
        let vector = e.mem.alloc(16);
        e.mem.set_f32(vector, 1.0);
        stub(e, ACTOR_COLLISION_OBJECT, 0x7000);
        stub(e, ACTOR_TEST_8ACE90, 0);
        stub(e, MESSAGE_TEXT, 0x7100);
        stub(e, FIND_BY_NAME_4AAE30, 0x7200);
        stub(e, NODE_ROTATION, 0x8200);
        stub(e, MATRIX_TRANSFORM, vector);
        stub(e, VECTOR_SUBTRACT, vector);
        stub(e, VECTOR_SCALE, vector);
        stub(e, DEBUG_DRAW_LEVEL, 0);
        stub_with(e, COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        stub(e, CREATE_DIR_ARROW, 0x8500);
        stub(e, HIT_RAY_C806B0, 0);
        stub(e, HIT_ACTOR_WORKER_571530, 0x7300);
        stub(e, HIT_GLOBAL_VALUE_8A5190, 9);
        stub(e, HIT_CONTROLLER_FLAG_8A5170, 1);
        stub(e, MOBILE_OBJECT_GET_CHAR_CONTROLLER, 0);
        stub(e, PLAYER_FIRST_PERSON_CHECK, 1);
        stub(e, HIT_TEXT_SOURCE_413F40, 0x7400);
        stub(e, HIT_NAME_STRING_43B1B0, 0x7500);
        for address in [
            BLEND_CONTROLLER_DO_HIT,
            HIT_APPLY_CB9320,
            ERROR_REPORT,
            VECTOR_NORMALIZE,
            VECTOR_COPY_553FC0,
            VECTOR_ASSIGN,
            HIT_RAY_SET_VECTOR_8A50F0,
            HIT_RAY_SET_REACH_8A50D0,
            DEBUG_OBJECT_SET_POSITION,
            TES_ADD_TEMP_DEBUG_OBJECT,
            HIT_RAY_RESULT_4AE750,
            RIGID_BODY_REFERENCE_4B5A20,
            HIT_BODY_OBJECT_44DDC0,
        ] {
            stub(e, address, 0);
        }
        stub_with(e, DEBUG_DRAW_DURATION_GETTER, |_, a| {
            float_ret(match a[0] {
                HIT_SETTING_FIRST_A => 100.0,
                HIT_SETTING_SECOND_A => 10.0,
                HIT_SETTING_SECOND_B => 2.0,
                DEBUG_DRAW_DURATION => 4.5,
                _ => 0.0,
            })
        });
        (this, source)
    }

    #[test]
    fn hit_me_ov2_applies_both_amounts() {
        let mut e = engine();
        let (this, source) = hit_ov2_setup(&mut e, 0);
        e.call_log = Some(vec![]);
        actor_hit_me_ov2(&mut e, this, source, 50.0, 0.5, 0x9000, 0x9100);
        // The offsets are turned into points by the node of the source.
        let transforms = calls_to(&e, MATRIX_TRANSFORM);
        assert_eq!(transforms.len(), 2);
        assert_eq!((transforms[0][2], transforms[1][2]), (0x9000, 0x9100));
        // Played unless blocked: no controller, so the flag is zero.
        assert_eq!(
            calls_to(&e, BLEND_CONTROLLER_DO_HIT),
            vec![vec![0x7200, 0, 0]]
        );
        // first = 100 * 50 * 0.01 = 50 scales the direction; the second
        // section scales it by the same amount.
        let scales = calls_to(&e, VECTOR_SCALE);
        assert_eq!(scales.len(), 2);
        assert_eq!(scales[0][2], 50.0f32.to_bits());
        assert_eq!(scales[1][2], 50.0f32.to_bits());
        // No hit from the ray: the fall-back lookup supplies the object.
        assert_eq!(calls_to(&e, FIND_BY_NAME_4AAE30).len(), 2);
        assert_eq!(
            calls_to(&e, ERROR_REPORT),
            vec![vec![HIT_AT_FORMAT, 0x7500]]
        );
        assert_eq!(
            calls_to(&e, HIT_RAY_SET_REACH_8A50D0)[0][1],
            30.0f32.to_bits()
        );
        let applied = calls_to(&e, HIT_APPLY_CB9320);
        assert_eq!(applied.len(), 2);
        assert_eq!((applied[0][0], applied[0][2]), (0x7200, 0x7200));
        assert_eq!((applied[1][0], applied[1][2]), (0x7200, 0x7300));
        assert_eq!(
            calls_to(&e, HIT_ACTOR_WORKER_571530),
            vec![vec![this.addr(), 0x7000, 9]]
        );
        assert_eq!(call_count(&e, DEBUG_DRAW_LEVEL), 1);
        assert_eq!(call_count(&e, CREATE_DIR_ARROW), 0);
    }

    #[test]
    fn hit_me_ov2_ray_hit_and_debug_arrow() {
        let mut e = engine();
        let (this, source) = hit_ov2_setup(&mut e, 1);
        stub(&mut e, DEBUG_DRAW_LEVEL, 1);
        stub(&mut e, HIT_RAY_C806B0, 0x6000);
        stub(&mut e, HIT_RAY_RESULT_4AE750, 0x6100);
        stub(&mut e, RIGID_BODY_REFERENCE_4B5A20, 0x6200);
        stub(&mut e, HIT_BODY_OBJECT_44DDC0, 0x6300);
        // A controller whose member test is true: the limb flag is passed.
        stub(&mut e, MOBILE_OBJECT_GET_CHAR_CONTROLLER, 0x6400);
        e.call_log = Some(vec![]);
        // A second amount above 75 clears the first one: only the second
        // section scales, by the cleared first amount.
        let frame = next_frame(&mut e, 0x160);
        actor_hit_me_ov2(&mut e, this, source, 500.0, 100.0, 0x9000, 0x9100);
        // 008a4af0's aligned frame: the points at EBP-0x1c and -0x28, the difference at EBP-0x5c.
        let ebp = (frame + 0x150 + 0xf) & !0xf;
        assert_eq!(
            calls_to(&e, VECTOR_SUBTRACT)[0],
            vec![ebp - 0x28, ebp - 0x5c, ebp - 0x1c]
        );
        assert_eq!(
            calls_to(&e, BLEND_CONTROLLER_DO_HIT),
            vec![vec![0x7200, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, HIT_CONTROLLER_FLAG_8A5170),
            vec![vec![0x6400 + 0x410]]
        );
        assert_eq!(call_count(&e, CREATE_DIR_ARROW), 0);
        assert_eq!(call_count(&e, HIT_RAY_C806B0), 0);
        let scales = calls_to(&e, VECTOR_SCALE);
        assert_eq!(scales.len(), 1);
        assert_eq!(scales[0][2], 0);

        // With a percentage the ray runs, the arrow is blue and the body
        // found by the ray is the object hit.
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x160);
        actor_hit_me_ov2(&mut e, this, source, 500.0, 0.0, 0x9000, 0x9100);
        // The colour at EBP-0xb8 (blue built at EBP-0xc8), the ray at EBP-0x110 with its vector at
        // EBP-0x120.
        let ebp = (frame + 0x150 + 0xf) & !0xf;
        assert_eq!(calls_to(&e, COLOR_CONSTRUCT)[0][0], ebp - 0xb8);
        assert_eq!(calls_to(&e, COLOR_CONSTRUCT)[1][0], ebp - 0xc8);
        assert_eq!(
            calls_to(&e, HIT_RAY_SET_VECTOR_8A50F0)[0],
            vec![ebp - 0x110, ebp - 0x120]
        );
        // The percentage is limited to 100: first = 100.
        let scales = calls_to(&e, VECTOR_SCALE);
        assert_eq!(scales[0][2], 100.0f32.to_bits());
        let arrow = calls_to(&e, CREATE_DIR_ARROW);
        assert_eq!(arrow.len(), 1);
        let color = arrow[0][1];
        assert_eq!(
            [e.mem.f32(color), e.mem.f32(color + 4), e.mem.f32(color + 8)],
            [0.0, 0.0, 1.0]
        );
        assert_eq!(calls_to(&e, HIT_BODY_OBJECT_44DDC0), vec![vec![0x6200]]);
        assert_eq!(calls_to(&e, HIT_APPLY_CB9320)[0][2], 0x6300);
        assert_eq!(call_count(&e, ERROR_REPORT), 1);
        assert_eq!(
            calls_to(&e, TES_ADD_TEMP_DEBUG_OBJECT),
            vec![vec![0x8300, 0x8500, 4.5f32.to_bits()]]
        );
    }

    #[test]
    fn hit_me_ov2_stops_early() {
        // Blocked by 008ace90: nothing else runs.
        let mut e = engine();
        let (this, source) = hit_ov2_setup(&mut e, 0);
        stub(&mut e, ACTOR_TEST_8ACE90, 1);
        e.call_log = Some(vec![]);
        actor_hit_me_ov2(&mut e, this, source, 50.0, 0.5, 0, 0);
        assert_eq!(call_count(&e, FIND_BY_NAME_4AAE30), 0);
        // Nothing found: the vectors are still worked out, then it ends.
        stub(&mut e, ACTOR_TEST_8ACE90, 0);
        stub(&mut e, FIND_BY_NAME_4AAE30, 0);
        e.call_log = Some(vec![]);
        actor_hit_me_ov2(&mut e, this, source, 50.0, 0.5, 0, 0);
        assert_eq!(call_count(&e, VECTOR_NORMALIZE), 1);
        assert_eq!(call_count(&e, BLEND_CONTROLLER_DO_HIT), 0);
        // A zero first amount still plays the hit for the second one.
        stub(&mut e, FIND_BY_NAME_4AAE30, 0x7200);
        e.call_log = Some(vec![]);
        actor_hit_me_ov2(&mut e, this, source, 0.0, 0.0, 0, 0);
        assert_eq!(call_count(&e, BLEND_CONTROLLER_DO_HIT), 1);
        // Both amounts zero: nothing is applied.
        stub_with(&mut e, DEBUG_DRAW_DURATION_GETTER, |_, _| float_ret(0.0));
        e.call_log = Some(vec![]);
        actor_hit_me_ov2(&mut e, this, source, 50.0, 0.5, 0, 0);
        assert_eq!(call_count(&e, BLEND_CONTROLLER_DO_HIT), 0);
        assert_eq!(call_count(&e, HIT_APPLY_CB9320), 0);
    }

    #[test]
    fn hit_me_ov2_player_failing_the_check() {
        let mut e = engine();
        let (this, source) = hit_ov2_setup(&mut e, 0);
        set_player(&mut e, this);
        stub(&mut e, PLAYER_FIRST_PERSON_CHECK, 0);
        stub(&mut e, DEBUG_DRAW_LEVEL, 1);
        e.call_log = Some(vec![]);
        actor_hit_me_ov2(&mut e, this, source, 50.0, 0.5, 0, 0);
        // No blend hit and no ray; the debug arrow is still drawn.
        assert_eq!(call_count(&e, BLEND_CONTROLLER_DO_HIT), 0);
        assert_eq!(call_count(&e, HIT_RAY_C806B0), 0);
        assert_eq!(call_count(&e, HIT_APPLY_CB9320), 0);
        assert_eq!(call_count(&e, CREATE_DIR_ARROW), 1);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes: the frame of the function the
    /// test calls next, taken outside any other local of it (nothing takes its place again
    /// before the function returns, so its words can still be read after the call).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    #[test]
    fn the_dry_fire_sound_uses_the_exe_frame_offsets() {
        let mut e = engine();
        let (actor, _, _) = attack_setup(&mut e);
        set_player(&mut e, actor);
        let process = e.get(actor, Actor::pCurrentProcess);
        set_slot(&mut e, process, 0x148, ret(0x6800));
        stub(&mut e, ENTRY_FORM, 0xf0f0);
        stub(&mut e, WEAPON_GET_CURRENT_AMMO, 0xa1);
        stub(&mut e, WEAPON_SOUND_SET, 0x700);
        stub(&mut e, DRY_FIRE_ALLOWED, 1);
        stub(&mut e, AUDIO_INSTANCE, 0x4400);
        stub(&mut e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, 0x4500);
        stub(&mut e, SOUND_SET_FILE_NAME, 0x4600);
        let frame = next_frame(&mut e, 0x110);
        actor_start_attack(&mut e, actor, 0x30);
        // The handle at +0xc8; the looked-up one at +0x60 (not the positioned sound).
        assert_eq!(calls_to(&e, SOUND_HANDLE_CONSTRUCT)[0], vec![frame + 0xc8]);
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME)[0][1],
            frame + 0x60
        );
    }

    #[test]
    fn the_dropped_weapon_position_and_angles_use_the_exe_frame_offsets() {
        let mut e = engine();
        let (actor, process, _) = drop_setup(&mut e);
        let node = e.mem.alloc(0x100);
        let bone = slot_target(&e, process, PROCESS_SLOT_WEAPON_BONE);
        e.register_double(bone, move |_, _| ret(node));
        let translate = e.mem.alloc(12);
        stub(&mut e, NODE_WORLD_TRANSLATE, translate);
        stub(&mut e, NODE_ROTATION, node + 0x68);
        let drop_slot = slot_target(&e, actor.cast(), ACTOR_SLOT_DROP_OBJECT);
        e.register_double(drop_slot, |_, _| ret(0x8000));
        let frame = next_frame(&mut e, 0x3c);
        fn_0089f580(&mut e, actor);
        // The angles at +0x10, the position at +0x1c.
        let angles = calls_to(&e, NI_MATRIX3_TO_EULER_ANGLES_XYZ);
        assert_eq!(angles[0][1..], [frame + 0x10, frame + 0x14, frame + 0x18]);
        let drop = calls_to(&e, drop_slot);
        assert_eq!(drop[0][4..], [frame + 0x1c, frame + 0x10]);
    }

    #[test]
    fn the_detection_bytes_use_the_exe_frame_offsets() {
        let mut e = engine();
        let (actor, target, acquire) = reaction_setup(&mut e);
        stub(&mut e, MOBILE_OBJECT_GET_CURRENT_PROCESS_TYPE, 0);
        stub(&mut e, ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR, 50);
        let record = e.mem.alloc(0x40);
        let creator = slot_target(&e, acquire, PROCESS_SLOT_F0);
        e.register_double(creator, move |_, _| ret(record));
        let frame = next_frame(&mut e, 0x80);
        fn_008987f0(&mut e, actor, target, 0);
        // The two bytes at +0x5b and +0x65, the second preset to 1.
        let detection = calls_to(&e, ACTOR_GET_DETECTION_LEVEL_AGAINST_ACTOR);
        assert_eq!(detection[0][3], frame + 0x5b);
        assert_eq!(detection[0][7], frame + 0x65);
        assert_eq!(e.mem.u8(frame + 0x65), 1);
    }

    #[test]
    fn the_debug_arrow_uses_the_exe_frame_offsets() {
        let mut e = engine();
        let (actor, _) = hit_setup(&mut e);
        stub(&mut e, DEBUG_DRAW_LEVEL, 2);
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x8300u32);
        e.map(ARROW_LENGTH_SCALE, 4);
        e.map(ARROW_WIDTH_SCALE, 4);
        stub_with(&mut e, COLOR_CONSTRUCT, |_, a| ret(a[0]));
        stub(&mut e, NODE_ROTATION, 0x8400);
        let vector = e.mem.alloc(12);
        stub(&mut e, MATRIX_TRANSFORM, vector);
        stub(&mut e, VECTOR_SUBTRACT, vector);
        stub(&mut e, VECTOR_SCALE, vector);
        stub(&mut e, ACTOR_GET_WEAPON_POSITION, vector);
        stub(&mut e, CREATE_DIR_ARROW, 0x8500);
        stub_float(&mut e, DEBUG_DRAW_DURATION_GETTER, 4.5);
        let frame = next_frame(&mut e, 0x164);
        fn_00899200(&mut e, actor, 0, 0);
        // The colour at +0x108 and the red temporary at +0x78.
        let colours: Vec<u32> = calls_to(&e, COLOR_CONSTRUCT).iter().map(|c| c[0]).collect();
        assert_eq!(colours, vec![frame + 0x108, frame + 0x78]);
        assert_eq!(calls_to(&e, CREATE_DIR_ARROW)[0][1], frame + 0x108);
        let subtract = &calls_to(&e, VECTOR_SUBTRACT)[0];
        assert_eq!(subtract[..3], [frame + 0x118, frame + 0x48, frame + 0xf8]);
        let scales: Vec<u32> = calls_to(&e, VECTOR_SCALE).iter().map(|c| c[1]).collect();
        assert_eq!(scales, vec![frame + 0x124, frame + 0x3c]);
    }

    #[test]
    fn the_combat_hit_keeps_its_locals_at_the_exe_frame_offsets() {
        let mut e = engine();
        let (attacker, target, _) = combat_hit_setup(&mut e);
        let frame = next_frame(&mut e, 0x118);
        actor_combat_hit(&mut e, attacker, target, 0, Ptr::NULL, 0);
        // The output byte at +0xeb, the hit data holder at +0xf4.
        let connects = slot_target(&e, attacker.cast(), ACTOR_SLOT_3D8);
        assert_eq!(calls_to(&e, connects)[0][4], frame + 0xeb);
        assert_eq!(
            calls_to(&e, NI_POINTER_HIT_DATA_CONSTRUCT)[0][0],
            frame + 0xf4
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_HIT_DATA_DESTRUCT),
            vec![vec![frame + 0xf4]]
        );
    }
}
