//! `fallout shared/inventorychanges.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Session notes (read these first when you continue the unit).
//!
//! * The unit is large (148 functions). Translated so far: `004bc550` to
//!   `004d0490` (the `ItemChange` methods, the weapon mod helpers, the
//!   save/load code of `ItemChange`, the `InventoryChanges` constructor and
//!   its hot key, equip and lookup methods; in the third session the item
//!   removal `004c0cf0`, the reference and object adders, the big removal
//!   `004c37d0`, the drop into the world, the best weapon/armor/ammunition
//!   choices, the stack counting and the fast iterator, gold and food, the
//!   load fix `004cbbc0`, the duplicate `004cd9c0`, the whole transfer
//!   `004ce380` and the container queries up to `004d0490`). The next
//!   session (the fourth) translated the rest: `004d0650` to `004d4f40` and
//!   `0076b630` (the weight and value totals, the leveled item and script
//!   steps, `RunScripts`, the copy into another changes `004d26d0`,
//!   `ClearAllChangeItems`, the save, load and buffer code, the item
//!   walks and groups and the `NiTMap<TESObject *, bool>` members); the unit
//!   is complete. The `InventoryChanges` layout is
//!   declared below with the constants. The third session's helpers
//!   (`entry_number`, `set_entry_number`, `announce_taken_item`,
//!   `removal_owner`, the list pop and scripted-form clone helpers) sit
//!   next to the functions that use them.
//! * Shared tiny accessors of other units are called by address and are
//!   named for what they do here: `00726070` reads the word at +4 of an
//!   object (the next node of a list, the number of an `ItemChange`),
//!   `0044ddc0` the word at +8 (the form of an `ItemChange`), `00559450`
//!   the word at +0, `00500940` gives the address of the form list inside
//!   a `BGSListForm`.
//! * An `ItemChange` is a `BSSimpleList<ExtraDataList *>` of the extra data
//!   lists of one stack of an item, the number of items, and the item's form
//!   (see [`ItemChange`]). The list is walked the way the game does it: the
//!   head node `[this]` is nonnull and its item is the first extra list; an
//!   empty list is a head node whose item is null. The accessors
//!   `006815c0` (address of the item word of a node) and `00726070` (next
//!   node) are the shared ones in `extradatalist.rs` (`list_item`,
//!   `list_next`).
//! * Most methods act on the first extra list only (`first_item`); the ones
//!   that loop restart from the head after they delete a list from the
//!   `BSSimpleList`, as the game does.
//! * Form pointers are read at their PC offsets with the Xbox PDB names in
//!   comments (the classes belong to other units).
//! * x87 note: values the game keeps as `float` are rounded to `f32` where
//!   the code stores a `float`; a `float` returned in `ST0` is an `f32`
//!   result of the translation.
//!
//! The translation keeps the structure of the game's code, so clippy's
//! `if_same_then_else` (branches of the original that do the same),
//! `too_many_arguments` (the original's calls) and
//! `neg_cmp_op_on_partial_ord` (`!(a < b)` is not `a >= b` for NaN) are
//! allowed for the unit.

#![allow(
    clippy::if_same_then_else,
    clippy::too_many_arguments,
    clippy::neg_cmp_op_on_partial_ord
)]

#[allow(unused_imports)]
use crate::prelude::*;

use super::extradatalist::{list_item, list_next};
use crate::types::NiTMap;

/// `operator new(size)` (`platform`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The value is passed as an `f64` argument (two words).
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `__RTDynamicCast(object, vfDelta, srcType, targetType, isReference)`.
pub(crate) const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The word at +4 of an object (`00726070`; the next node of a list node, or
/// the value of a `TESHealthForm`).
pub(crate) const WORD_AT_4: u32 = 0x0072_6070;
/// The word at +8 of an object (`0044ddc0`; for an `ItemChange` its form).
pub(crate) const WORD_AT_8: u32 = 0x0044_ddc0;
/// The word at +0 of an object (`00559450`; for an `ItemChange` or an
/// inventory entry its list).
pub(crate) const WORD_AT_0: u32 = 0x0055_9450;
/// The byte at +4 of a form (`00401170`): its form type.
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;
/// `BSSimpleList<T>::BSSimpleList()` (`0096a2d0`, named
/// `Concurrency::details::QuickBitSet::QuickBitSet` by the linker's folding):
/// zeroes the item and next words. The map has no name for it.
pub(crate) const LIST_NODE_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList<T>::Remove(&item)` (`00905330`): unlinks the node holding
/// `*item`, starting from the node it is called on.
pub(crate) const LIST_REMOVE: u32 = 0x0090_5330;
/// `BSSimpleList<T>::AddHead/Append(&item)` (`005ae3d0`): stores `*item` in
/// the head node when it is empty, else in a new node after the head.
pub(crate) const LIST_ADD: u32 = 0x005a_e3d0;
/// `BSSimpleList<T>::AddTail(&item)` (`00905820`): stores `*item` in the
/// last node when it is empty, else in a new node after it.
pub(crate) const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// `BSSimpleList` clear (`00470470`) and the deleting destructor
/// (`004702f0(list, flags)`).
pub(crate) const LIST_CLEAR: u32 = 0x0047_0470;
pub(crate) const LIST_DESTROY: u32 = 0x0047_02f0;
/// `ExtraDataList::ExtraDataList` (Xbox PDB).
pub(crate) const EXTRA_DATA_LIST_CONSTRUCT: u32 = 0x0041_0360;
/// `ExtraDataList::DuplicateExtraListForContainer` (Xbox PDB).
pub(crate) const EXTRA_DATA_LIST_DUPLICATE: u32 = 0x0041_2380;
/// `BaseExtraList::RemoveAll(bool)` (Xbox PDB).
pub(crate) const EXTRA_REMOVE_ALL: u32 = 0x0040_fae0;
/// `BaseExtraList::ItemsInList` (Xbox PDB).
pub(crate) const EXTRA_ITEMS_IN_LIST: u32 = 0x0040_fe20;
/// `BaseExtraList::RemoveExtra(extra, bool)` (Xbox PDB).
pub(crate) const EXTRA_REMOVE_EXTRA: u32 = 0x0041_0020;
/// `ExtraDataList` accessors (Xbox PDB names where the map has one).
pub(crate) const EXTRA_IS_DEFAULT_FOR_CONTAINER: u32 = 0x0041_d120;
pub(crate) const EXTRA_GET_WORN: u32 = 0x0041_8ab0;
pub(crate) const EXTRA_GET_COUNT: u32 = 0x0041_8770;
pub(crate) const EXTRA_REMOVE_COUNT: u32 = 0x0041_af10;
pub(crate) const EXTRA_HAS_LEVELED_ITEM: u32 = 0x0041_8750;
pub(crate) const EXTRA_GET_LEVELED_ITEM: u32 = 0x0041_8720;
/// `ExtraDataList::GetOriginalReference` / `RemoveOriginalReferenceExtra`.
pub(crate) const EXTRA_GET_ORIGINAL_REFERENCE: u32 = 0x0041_8630;
pub(crate) const EXTRA_REMOVE_ORIGINAL_REFERENCE: u32 = 0x0041_8600;
/// The unnamed `ExtraDataList` test used by `004bccb0`.
pub(crate) const EXTRA_TEST_D030: u32 = 0x0041_d030;
/// `ExtraDataList::GetHealth` / `SetHealth` / `RemoveHealth` (Xbox PDB), and
/// the setter `00419c60` that `SetItemHealth` uses on a new list.
pub(crate) const EXTRA_GET_HEALTH: u32 = 0x0041_86f0;
pub(crate) const EXTRA_SET_HEALTH: u32 = 0x0041_9970;
pub(crate) const EXTRA_SET_HEALTH_NEW: u32 = 0x0041_9c60;
pub(crate) const EXTRA_REMOVE_HEALTH: u32 = 0x0041_aef0;
/// `00418660` (the "ownership" extra), `004187a0` (a float extra, `-1.0`
/// when absent), `004187d0` (`ExtraDataList::GetPoison`), `0041af30` (removes
/// it), `00419d10` (`ExtraDataList::SetRank`), `00419700` (copies another
/// list's extras into this one).
pub(crate) const EXTRA_GET_OWNERSHIP: u32 = 0x0041_8660;
pub(crate) const EXTRA_GET_FLOAT_EXTRA: u32 = 0x0041_87a0;
pub(crate) const EXTRA_GET_POISON: u32 = 0x0041_87d0;
pub(crate) const EXTRA_REMOVE_POISON: u32 = 0x0041_af30;
pub(crate) const EXTRA_SET_RANK: u32 = 0x0041_9d10;
pub(crate) const EXTRA_COPY_FROM: u32 = 0x0041_9700;
/// `ExtraDataList::GetHotKey`, `GetScript`, `GetScriptLocals`.
pub(crate) const EXTRA_GET_HOT_KEY: u32 = 0x0042_de90;
pub(crate) const EXTRA_GET_SCRIPT: u32 = 0x0041_8800;
pub(crate) const EXTRA_GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
/// `ExtraDataList::HasWeaponMods`, `GetWeaponModFlags`,
/// `GetWeaponModSlotActive(slot)`, `SetIsModding(bool)`, `RemoveIsModding`.
pub(crate) const EXTRA_HAS_WEAPON_MODS: u32 = 0x0041_8ba0;
pub(crate) const EXTRA_GET_WEAPON_MOD_FLAGS: u32 = 0x0042_e560;
pub(crate) const EXTRA_GET_WEAPON_MOD_SLOT_ACTIVE: u32 = 0x0041_8c00;
pub(crate) const EXTRA_SET_IS_MODDING: u32 = 0x0042_e5a0;
pub(crate) const EXTRA_REMOVE_IS_MODDING: u32 = 0x0042_e6f0;
/// `TESHealthForm::GetFormAsHealthForm(form)` (`__cdecl`) and
/// `TESHealthForm::GetFormHealth(form)` (`__cdecl`), `TESValueForm::GetFormValue`.
pub(crate) const GET_FORM_AS_HEALTH_FORM: u32 = 0x0048_72e0;
pub(crate) const GET_FORM_HEALTH: u32 = 0x0048_73d0;
pub(crate) const GET_FORM_VALUE: u32 = 0x0048_e8a0;
/// `min(a, b)` on floats (`__cdecl`, result in ST0).
pub(crate) const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `InventoryChanges::GetObjectInList(form, 1, 0)` (Xbox PDB).
pub(crate) const INVENTORY_CHANGES_GET_OBJECT_IN_LIST: u32 = 0x004b_fba0;
/// `CombatFormulas::CalcWeaponDamage` and `CalcArmorRating` (Xbox PDB,
/// `__cdecl`), and the price formula `00647c00(value, health)`.
pub(crate) const CALC_WEAPON_DAMAGE: u32 = 0x0064_4ce0;
pub(crate) const CALC_ARMOR_RATING: u32 = 0x0064_6360;
pub(crate) const CALC_ITEM_PRICE: u32 = 0x0064_7c00;
/// `004be060(armorForm)`: the armor's rating as a float (it calls `004be080`).
pub(crate) const ARMOR_RATING_FLOAT: u32 = 0x004b_e060;
/// `00476b20(float)` (`__cdecl`): the rounding applied to the armor result.
pub(crate) const ROUND_ARMOR_RESULT: u32 = 0x0047_6b20;
/// `TESActorBaseData::GetFatigue` (the map's name, source `vt`): a 16-bit
/// value read from the enchantable form.
pub(crate) const ENCHANTABLE_FORM_VALUE: u32 = 0x004a_8ae0;

/// RTTI type descriptors the casts use (names from the `.?AV...@@` strings).
pub(crate) const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
pub(crate) const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
pub(crate) const TYPE_TES_ENCHANTABLE_FORM: u32 = 0x0118_39b4;
pub(crate) const TYPE_TES_HEALTH_FORM: u32 = 0x0118_6c3c;

/// `-1.0f` (the "no value" float).
pub(crate) const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `-1.0` as a double (the comparison constant).
pub(crate) const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `100.0f`.
pub(crate) const HUNDRED_FLOAT: u32 = 0x0101_6410;
/// `100.0` as a double.
pub(crate) const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// `0.5` as a double (the rounding threshold of `004bd510`).
pub(crate) const HALF_DOUBLE: u32 = 0x0101_1588;
/// The float multiple `GetItemValue` rounds the price to (`0.1f`).
pub(crate) const PRICE_STEP: u32 = 0x0101_e2bc;

/// `TESObjectWEAP` fields (PC offsets; the Xbox PDB offsets are 0x10 higher
/// up to here): `TESHealthForm` sub-object at +0x94 (its `iHealth` is the
/// word at +4 of it), `OBJ_WEAP::eModActionOne..Three` at +0x180, +0x184,
/// +0x188, `fModActionOneValue..` at +0x18c, +0x190, +0x194 and
/// `fModActionOneValueTwo..` at +0x1ac, +0x1b0, +0x1b4,
/// `pModObjectOne..Three` at +0x350, +0x354, +0x358.
pub(crate) const WEAPON_HEALTH_FORM: u32 = 0x94;
pub(crate) const WEAPON_MOD_ACTION: u32 = 0x180;
pub(crate) const WEAPON_MOD_ACTION_VALUE: u32 = 0x18c;
pub(crate) const WEAPON_MOD_ACTION_VALUE_TWO: u32 = 0x1ac;
pub(crate) const WEAPON_MOD_OBJECT: u32 = 0x350;
/// `TESObjectIMOD`: the value word read at +0x7c + 4 of a mod object.
pub(crate) const MOD_OBJECT_VALUE: u32 = 0x7c;
/// The weapon mod effect type `10` that `GetItemHealth` asks about.
pub(crate) const MOD_EFFECT_HEALTH: u32 = 10;

layout! {
    /// `ItemChange` (Xbox PDB), 0xC bytes: one stack of an item in an
    /// inventory.
    pub struct ItemChange: 0x0C {
        /// `pExtraObjectList` (Xbox PDB): `BSSimpleList<ExtraDataList *>*`.
        0x00 pExtraObjectList: Ptr,
        /// `iNumber` (Xbox PDB).
        0x04 iNumber: i32,
        /// `pContainerObj` (Xbox PDB): `TESBoundObject*`.
        0x08 pContainerObj: Ptr,
    }
}

// Constants of the second session (functions 004be060 to 004c0c60).

/// RTTI type descriptors the casts of this part use (names from the
/// `.?AV...@@` strings): `TESForm`, `TESFullName`, `TESBipedModelForm`,
/// `TESNPC`.
pub(crate) const TYPE_TES_FORM: u32 = 0x0118_3028;
pub(crate) const TYPE_TES_FULL_NAME: u32 = 0x0118_3158;
pub(crate) const TYPE_TES_BIPED_MODEL_FORM: u32 = 0x0118_3978;
pub(crate) const TYPE_TES_NPC: u32 = 0x0118_3a1c;

/// The source path string the game's logging passes
/// (`D:\_Fallout3\Platforms\Common\Code\Fallout Shared\InventoryChanges.cpp`).
pub(crate) const SOURCE_FILE_NAME: u32 = 0x0102_0778;
/// The player character pointer: the global the game compares actors with
/// and whose `+0xe3c` holds the hot key table.
pub(crate) const PLAYER_GLOBAL: u32 = 0x011d_ea3c;
/// The save/load game object (`TESSaveLoadGame`, by the names of the methods
/// called on it): the `this` of every save size/save/load call of the unit.
pub(crate) const SAVE_LOAD_GAME_GLOBAL: u32 = 0x011d_e45c;
/// The object `00408d60` turns into the "log the save sizes" flag byte.
pub(crate) const SAVE_SIZE_LOG_OBJECT: u32 = 0x011d_e4e8;
/// `InventoryChanges::pTempRef` (Xbox PDB): the scratch `TESObjectREFR`
/// the first `InventoryChanges` constructor creates.
pub(crate) const TEMP_REF_GLOBAL: u32 = 0x011c_6444;
/// The two string objects `00403df0` turns into a `char *` when no icon or
/// no name was found.
pub(crate) const NO_ICON_STRING: u32 = 0x011d_33f0;
pub(crate) const NO_NAME_STRING: u32 = 0x011d_35b8;
/// `0.0` as a double (the comparison constant of `fn_004be0b0`).
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `00404eb0(guard, 0x36, 1, file, line)` and `00404ee0(guard)`: the scope
/// guard (4 bytes on the stack) the methods of the unit open and close.
pub(crate) const SCOPE_GUARD_OPEN: u32 = 0x0040_4eb0;
pub(crate) const SCOPE_GUARD_CLOSE: u32 = 0x0040_4ee0;
/// `operator delete(ptr)` and `memset(ptr, value, size)` (`__cdecl`).
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
pub(crate) const MEMSET: u32 = 0x0040_3d30;
/// `Error(format, ...)` (Xbox PDB, `__cdecl`, varargs) and the save/load
/// log `005b5e40(format, ...)` (`__cdecl`, varargs).
pub(crate) const ERROR_LOG: u32 = 0x0040_fbe0;
pub(crate) const SAVE_LOAD_LOG: u32 = 0x005b_5e40;
/// `delete item` of an `ItemChange` (`004459e0(item, flags)`, a scalar
/// deleting destructor).
pub(crate) const ITEM_CHANGE_DELETE: u32 = 0x0044_59e0;
/// `005f65d0(list, &item)` (`__thiscall`): true when the list holds the item.
pub(crate) const LIST_CONTAINS: u32 = 0x005f_65d0;
/// The word at +0xC of a form (`0084e3a0`): its form id.
pub(crate) const FORM_ID: u32 = 0x0084_e3a0;
/// `LookupFormByID(id)` (`004839c0`, `__cdecl`).
pub(crate) const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `InventoryChanges::StartOEIFastInventoryIteration()` and
/// `GetNextOEIFastInventoryItem(iterator)` (Xbox PDB), and the iterator's
/// destructor body (`004ca1a0`).
pub(crate) const START_FAST_ITERATION: u32 = 0x004c_a200;
pub(crate) const GET_NEXT_FAST_ITEM: u32 = 0x004c_a330;
pub(crate) const FAST_ITERATOR_DESTRUCT: u32 = 0x004c_a1a0;
/// `004c0cf0` (2030 bytes, of this unit, translated by a later session): the
/// routine `RemoveAllObjectsWorn` calls to take an item out.
pub(crate) const INVENTORY_REMOVE_ITEM: u32 = 0x004c_0cf0;
/// `ExtraDataList` methods used by this part (Xbox PDB names).
pub(crate) const EXTRA_COMPARE_LIST: u32 = 0x0041_27e0;
pub(crate) const EXTRA_COPY_LIST: u32 = 0x0041_1ec0;
pub(crate) const EXTRA_SET_COUNT: u32 = 0x0041_9ad0;
pub(crate) const EXTRA_SET_WORN: u32 = 0x0041_aa20;
pub(crate) const EXTRA_SET_CAN_NOT_WEAR: u32 = 0x0041_ab70;
pub(crate) const EXTRA_GET_CAN_NOT_WEAR: u32 = 0x0041_8b10;
pub(crate) const EXTRA_REMOVE_HOT_KEY: u32 = 0x0042_dec0;
pub(crate) const EXTRA_SET_HOT_KEY: u32 = 0x0042_dde0;
pub(crate) const EXTRA_GET_REFERENCE_POINTER: u32 = 0x0041_c8d0;
pub(crate) const EXTRA_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
pub(crate) const EXTRA_SET_CONTAINER_CHANGES: u32 = 0x0041_9650;
/// Unnamed `ExtraDataList` methods: the save size (`00422c40(0, 0)`), the
/// `SaveGame(0, 0)` and `LoadGame(0, 0, 0)` of the file format, the
/// `00426020(0, 0, 0, form)` that runs after loading and `0042ceb0(arg,
/// form)` (the other, buffer based, finishing step).
pub(crate) const EXTRA_GET_SAVE_SIZE: u32 = 0x0042_2c40;
pub(crate) const EXTRA_SAVE_GAME: u32 = 0x0042_35e0;
pub(crate) const EXTRA_LOAD_GAME: u32 = 0x0042_4960;
pub(crate) const EXTRA_AFTER_LOAD_GAME: u32 = 0x0042_6020;
pub(crate) const EXTRA_AFTER_LOAD_GAME_BUFFER: u32 = 0x0042_ceb0;
/// `ExtraDataList::SaveGame_ov2(buffer)` and `LoadGame_ov2(buffer)`
/// (Xbox PDB).
pub(crate) const EXTRA_SAVE_GAME_BUFFER: u32 = 0x0042_6a30;
pub(crate) const EXTRA_LOAD_GAME_BUFFER: u32 = 0x0042_8150;
/// `BGSSaveGameBuffer::SaveFormID_ov2(form, 0)`, the raw data save
/// (`00865e50(ptr, size, 0)`), `StartVariableSizedValue()` and
/// `SaveVariableSizedValue_ov2(count, start)` (Xbox PDB).
pub(crate) const SAVE_BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
pub(crate) const SAVE_BUFFER_SAVE_DATA: u32 = 0x0086_5e50;
pub(crate) const SAVE_BUFFER_START_SIZED: u32 = 0x0086_5f20;
pub(crate) const SAVE_BUFFER_END_SIZED: u32 = 0x0086_5ff0;
/// `BGSLoadGameBuffer::LoadFormID()`, the raw data load (`00864980(ptr,
/// size)`) and `LoadVariableSizedValue()` (Xbox PDB).
pub(crate) const LOAD_BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
pub(crate) const LOAD_BUFFER_LOAD_DATA: u32 = 0x0086_4980;
pub(crate) const LOAD_BUFFER_LOAD_SIZED: u32 = 0x0086_4a60;
/// `TESSaveLoadGame` methods: `UseSaveGameBlocks` (Xbox PDB), the stream
/// position (`00825c00`, the word at +0x14), the raw save (`008579b0(ptr,
/// size)`), `SaveNumericID(ptr, size)` (Xbox PDB), the raw load
/// (`008579e0(ptr, size)`), `LoadNumericID(ptr, size)` (Xbox PDB), the two
/// current-form words (`004fd3c0` at +0x84 for loading, `004fd3e0` at +0x88
/// for saving, the map names the latter `TES::GetWorldSpace` for a folded
/// body) and the version byte (`008df040`).
pub(crate) const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
pub(crate) const SAVE_POSITION: u32 = 0x0082_5c00;
pub(crate) const SAVE_BYTES: u32 = 0x0085_79b0;
pub(crate) const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
pub(crate) const LOAD_BYTES: u32 = 0x0085_79e0;
pub(crate) const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
pub(crate) const CURRENT_LOAD_FORM: u32 = 0x004f_d3c0;
pub(crate) const CURRENT_SAVE_FORM: u32 = 0x004f_d3e0;
pub(crate) const LOAD_VERSION: u32 = 0x008d_f040;
/// `00408d60(object)`: the address of the flag byte at +4 of `object` (of a
/// global zero byte for null).
pub(crate) const FLAG_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// `008256d0(node)`: true for the head node of an empty `BSSimpleList`
/// (no item, no next node).
pub(crate) const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Virtual slot 0x130 of a form (called without arguments by the log
/// messages, its result is the `%s` of the message).
pub(crate) const FORM_NAME_SLOT: u32 = 0x130;
/// Callees of `004bffe0` and the other equip/hot key functions.
pub(crate) const GET_FORM_AS_BIPED_MODEL: u32 = 0x0048_0db0;
pub(crate) const BIPED_IS_POWER_ARMOR: u32 = 0x0048_0d10;
pub(crate) const BIPED_FLAG_FOUR: u32 = 0x0048_0d30;
pub(crate) const CONTAINER_COUNT: u32 = 0x0048_2a90;
pub(crate) const REFR_HAS_CONTAINER: u32 = 0x0055_d310;
pub(crate) const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
pub(crate) const REFR_CONSTRUCT: u32 = 0x0055_a2f0;
pub(crate) const REFR_EXTRA_LIST: u32 = 0x005d_43c0;
/// The map names the body of `008d8520` `MiddleHighProcess::GetSavedAcquireObject`;
/// it returns the word at +0x68 of an `Actor` (`pCurrentProcess`).
pub(crate) const ACTOR_PROCESS: u32 = 0x008d_8520;
pub(crate) const ACTOR_RELOAD_TARGETS: u32 = 0x008b_0b00;
pub(crate) const ACTOR_INIT_LIGHTING: u32 = 0x008b_0bd0;
pub(crate) const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
pub(crate) const ACTOR_LIGHTING_ARGUMENT: u32 = 0x0043_fcd0;
pub(crate) const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
pub(crate) const FORM_WEIGHT_FIELD: u32 = 0x004e_4620;
pub(crate) const ANIMATION_STORE_FLOAT: u32 = 0x004c_0c90;
/// `008248e0(form, 1)` on the sub-object at +0x94 of a reference (the actor):
/// the call `RemoveAllObjectsWorn` makes before removing a worn item.
pub(crate) const REFERENCE_FORM_NOTIFY: u32 = 0x0082_48e0;
pub(crate) const PLAYER_STORE_FORM: u32 = 0x004c_0cd0;
pub(crate) const REFR_UPDATE_WEAPON: u32 = 0x0057_1760;
pub(crate) const PREDICATE_008C7AA0: u32 = 0x008c_7aa0;
pub(crate) const THREAD_VALUE_0047B200: u32 = 0x0047_b200;
pub(crate) const TASK_QUEUE_GETTER: u32 = 0x0045_37b0;
pub(crate) const TASK_QUEUE_ATTACH_WEAPON: u32 = 0x0087_b360;
pub(crate) const STORE_NOTIFY_004534F0: u32 = 0x0045_34f0;
/// `006b9130(weightForm)`: the weight of a `TESWeightForm`; for a weapon it
/// sits at +0x8c.
pub(crate) const GET_WEIGHT: u32 = 0x006b_9130;
pub(crate) const WEAPON_WEIGHT_FORM: u32 = 0x8c;
/// `TESActorBase::GetSex` (Xbox PDB), the inventory icon of a biped form for
/// a sex (`00481230`), the generic icon of a form (`0048e730`), the string
/// object to `char *` conversion (`00403df0`), and the two calls of the full
/// name lookup (`0048cee0` test, `MapMarkerData::GetLocationName` `00408da0`).
pub(crate) const ACTOR_BASE_GET_SEX: u32 = 0x005f_0cc0;
pub(crate) const BIPED_ICON_FOR_SEX: u32 = 0x0048_1230;
pub(crate) const FORM_ICON: u32 = 0x0048_e730;
pub(crate) const STRING_OBJECT_TO_CHARS: u32 = 0x0040_3df0;
pub(crate) const FULL_NAME_TEST: u32 = 0x0048_cee0;
pub(crate) const FULL_NAME_GET: u32 = 0x0040_8da0;
/// The player's hot key table (a map at `+0xe3c`): `006a7ad0(index)` returns
/// the address of the entry, `0047a110(index, &value)` sets it.
pub(crate) const HOT_KEY_TABLE: u32 = 0xe3c;
pub(crate) const HOT_KEY_TABLE_ENTRY: u32 = 0x006a_7ad0;
pub(crate) const HOT_KEY_TABLE_SET: u32 = 0x0047_a110;
/// `TESObjectARMO` fields (PC offsets, the Xbox PDB ones are 0x10 higher):
/// the `TESBipedModelForm` sub-object at +0x70, `OBJ_ARMO::sRating` at +0x178
/// (u16), `fDamageThreshold` at +0x17c and `cFlags` at +0x180.
pub(crate) const ARMOR_BIPED_MODEL: u32 = 0x70;
pub(crate) const ARMOR_RATING: u32 = 0x178;
pub(crate) const ARMOR_DAMAGE_THRESHOLD: u32 = 0x17c;
pub(crate) const ARMOR_FLAGS: u32 = 0x180;
/// `TESObjectWEAP::OBJ_WEAP::eType` (a signed byte) at +0xf4.
pub(crate) const WEAPON_TYPE: u32 = 0xf4;
/// `Actor::pCurrentProcess` (Xbox PDB `+0x78`) on PC.
pub(crate) const ACTOR_CURRENT_PROCESS: u32 = 0x68;

layout! {
    /// `InventoryChanges` (Xbox PDB), 0x14 bytes: the changes of one
    /// container's inventory.
    pub struct InventoryChanges: 0x14 {
        /// `pListofChanges` (Xbox PDB): `BSSimpleList<ItemChange *>*`.
        0x00 pListofChanges: Ptr,
        /// `pRef` (Xbox PDB): the owning `TESObjectREFR*`.
        0x04 pRef: Ptr,
        /// `fcontainerweight` (Xbox PDB).
        0x08 fcontainerweight: f32,
        /// `fpreviousContainerWeight` (Xbox PDB).
        0x0C fpreviousContainerWeight: f32,
        /// `bcountdirty` (Xbox PDB).
        0x10 bcountdirty: bool,
    }
}

// Constants of the third session (functions 004c0c90 to 004d0490).

/// `-3.4028235e38f` (`-FLT_MAX`): the "nothing yet" rating of the best
/// ammunition search, and the double `0.1f` widened that `fn_004c0c90`
/// compares with.
pub(crate) const NEGATIVE_FLOAT_MAX: u32 = 0x0101_5f5c;
pub(crate) const ANIMATION_MINIMUM_DOUBLE: u32 = 0x0101_ffa0;
/// `0.99` (double): the factor of the "full health" weapon and armor health
/// computation of `fn_004c1c90`.
pub(crate) const NEAR_FULL_HEALTH: u32 = 0x0102_0858;
/// `fn_004c1c90` log formats ("Adding full health weapon ref '%s' (%08X) to
/// ref '%s' (%08X).", the throwing weapon one and the armor one).
pub(crate) const ADDING_FULL_HEALTH_WEAPON: u32 = 0x0102_0860;
pub(crate) const ADDING_DAMAGED_THROWING_WEAPON: u32 = 0x0102_0810;
pub(crate) const ADDING_FULL_HEALTH_ARMOR: u32 = 0x0102_07d0;
/// The global the `InventoryChanges` merge code keeps the extra data list it
/// merged into in (`fn_004c0cf0`), 0 at the start of a removal.
pub(crate) const LAST_MERGED_EXTRA_LIST: u32 = 0x011c_6448;
/// `ExtraDataList::GetDismembermentExtra` (Xbox PDB), the element getter
/// `00441420(array holder, index)` and the `fn_004c0cf0` helpers.
pub(crate) const GET_DISMEMBERMENT_EXTRA: u32 = 0x0042_e8c0;
pub(crate) const ARRAY_ELEMENT: u32 = 0x0044_1420;
/// `TESObjectREFR::RemoveWeapon` (Xbox PDB).
pub(crate) const REFR_REMOVE_WEAPON: u32 = 0x0057_1b50;
/// An empty function called with an actor (`00483710`, 11 bytes).
pub(crate) const NO_OPERATION_00483710: u32 = 0x0048_3710;
/// The task queue's weapon detach (`0087b3b0(queue, actor, weapon)`), the
/// counterpart of `TASK_QUEUE_ATTACH_WEAPON`.
pub(crate) const TASK_QUEUE_DETACH_WEAPON: u32 = 0x0087_b3b0;
/// `ExtraDataList::CompareListForContainer(this, other, 0, flag)` (Xbox PDB).
pub(crate) const COMPARE_LIST_FOR_CONTAINER: u32 = 0x0041_26c0;
/// `ExtraDataList::CopyListForContainer(this, source, 0)` (Xbox PDB), the
/// scale setter `ExtraDataList::SetScale` and the ownership remover.
pub(crate) const EXTRA_COPY_LIST_FOR_CONTAINER: u32 = 0x0041_21e0;
pub(crate) const EXTRA_SET_SCALE: u32 = 0x0041_9fb0;
pub(crate) const EXTRA_REMOVE_OWNERSHIP: u32 = 0x0041_aed0;
/// `007af430`: the word at +0x20 of an object (a reference's base form; the
/// map names it `BGSSaveFormBuffer::GetForm`), and the extra data list of a
/// reference (`005d43c0`).
pub(crate) const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB) and the scale of a reference
/// (`00598040`, the float at +0x3c), the scale `TESObjectREFR` setter
/// (`00567490(reference, float)`) and the health setter (`00568bd0(reference,
/// float)`).
pub(crate) const REFERENCE_PERSISTS: u32 = 0x0056_53d0;
pub(crate) const REFERENCE_SCALE: u32 = 0x0059_8040;
pub(crate) const REFERENCE_SET_SCALE: u32 = 0x0056_7490;
pub(crate) const REFR_SET_HEALTH: u32 = 0x0056_8bd0;
/// `ExtraDataList::SetReference(reference)` (`0041c7f0`, the list's own
/// reference), the extra list ammo getter and setter of type 0x6e
/// (`0042eb40`, `0042eb60(list, 0, 0)`) and the `unsigned min` `0042f5a0`.
pub(crate) const EXTRA_SET_REFERENCE: u32 = 0x0041_c7f0;
pub(crate) const EXTRA_GET_AMMO: u32 = 0x0042_eb40;
pub(crate) const EXTRA_SET_AMMO: u32 = 0x0042_eb60;
pub(crate) const UNSIGNED_MINIMUM: u32 = 0x0042_f5a0;
/// The weapon type getter (`00446390`, the signed byte at +0xf4) and
/// `TESObjectWEAP::GetCurrentAmmo(this, actor)` (Xbox PDB).
pub(crate) const WEAPON_TYPE_GETTER: u32 = 0x0044_6390;
pub(crate) const WEAPON_GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// `InventoryChanges` item-number setter (`006ecd40`, stores the argument at
/// +4), the number of non-null items of a `BSSimpleList` (`005ae380`, the
/// map names it `VATS::GetCount`) and two key state tests (`00705020` and
/// `00705000`, each asks `00a09030` about one key).
pub(crate) const ITEM_SET_NUMBER: u32 = 0x006e_cd40;
pub(crate) const LIST_COUNT_NONNULL: u32 = 0x005a_e380;
pub(crate) const KEY_STATE_TEST_041D: u32 = 0x0070_5020;
pub(crate) const KEY_STATE_TEST_0435: u32 = 0x0070_5000;
/// `ExtraDataList` methods `fn_004c37d0` uses: `GetExtraData(type)`
/// (`BaseExtraList`, Xbox PDB), the owner copier `00419700(list, owner)`,
/// the copy for a reference (`CopyListForReference(this, source, flag)`,
/// Xbox PDB) and the unnamed ones `0041b010(list, 0)`, `0041d000`,
/// `0041d280(list, value)`, `0041afd0`, `0041c900`,
/// `GetStartingWorldOrCell` and `SetStartingWorldOrCellForRef` (Xbox PDB), and
/// the scale float getter `00418860`.
pub(crate) const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
pub(crate) const EXTRA_SET_OWNER: u32 = 0x0041_9700;
pub(crate) const EXTRA_COPY_FOR_REFERENCE: u32 = 0x0041_2490;
pub(crate) const EXTRA_FN_0041B010: u32 = 0x0041_b010;
pub(crate) const EXTRA_FN_0041D000: u32 = 0x0041_d000;
pub(crate) const EXTRA_FN_0041D280: u32 = 0x0041_d280;
pub(crate) const EXTRA_FINISH_COPY: u32 = 0x0041_afd0;
pub(crate) const EXTRA_FN_0041C900: u32 = 0x0041_c900;
pub(crate) const EXTRA_GET_STARTING_WORLD_OR_CELL: u32 = 0x0041_b320;
pub(crate) const EXTRA_SET_STARTING_WORLD_OR_CELL: u32 = 0x0041_b2a0;
pub(crate) const EXTRA_GET_SCALE_FLOAT: u32 = 0x0041_8860;
/// `ExtraDataList::GetLevCreaOriginalBase` (Xbox PDB), `TESObjectREFR::GetOwner`
/// (Xbox PDB), `TESContainer::IsGold` (Xbox PDB, `__cdecl`).
pub(crate) const EXTRA_GET_ORIGINAL_BASE: u32 = 0x0042_16f0;
pub(crate) const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
pub(crate) const IS_GOLD: u32 = 0x0048_1f10;
/// `ItemChange::ItemChange_ov3` (Xbox PDB): the constructor that zeroes the
/// list, number and form.
pub(crate) const ITEM_CHANGE_CONSTRUCT_EMPTY: u32 = 0x0076_b630;
/// The checks that run after an item was dropped: the actor's parent cell
/// (`008d6f30`), a test of the dropped reference (`00567770`),
/// `TESObjectCELL::GetOwner` (Xbox PDB), the cell/actor test `00546ca0(cell,
/// actor)`, `TESBipedModelForm::GetWorldTESModel_ov2(flag)` (Xbox PDB) and
/// the hand over `00567ad0(reference, form)`.
pub(crate) const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
pub(crate) const DROPPED_REFERENCE_TEST: u32 = 0x0056_7770;
pub(crate) const CELL_GET_OWNER: u32 = 0x0054_6a40;
pub(crate) const CELL_TEST_ACTOR: u32 = 0x0054_6ca0;
pub(crate) const BIPED_WORLD_MODEL: u32 = 0x0048_1110;
pub(crate) const REFERENCE_HAND_OVER: u32 = 0x0056_7ad0;
/// `Script::SetActionFlag(actor, extra, 4)` (Xbox PDB, `__cdecl`).
pub(crate) const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `Actor::UnEquipObject(form, count, extra, 0, 0, 1)` and
/// `Actor::EquipObject(form, count, extra, 1, 0, 0)` (Xbox PDB), and the
/// actor test `00440da0`.
pub(crate) const ACTOR_UNEQUIP_OBJECT: u32 = 0x0088_d7d0;
pub(crate) const ACTOR_EQUIP_OBJECT: u32 = 0x0088_c830;
pub(crate) const ACTOR_TEST_00440DA0: u32 = 0x0044_0da0;
/// `Interface::IsMenuIDVisible(id, 0)` (Xbox PDB, `__cdecl`).
pub(crate) const IS_MENU_VISIBLE: u32 = 0x0070_2680;
/// The "can not unequip" message of `fn_004c37d0`: the text object
/// `0x011d31f8` (turned into a `char *` by `fn_004c69f0`), the icon path
/// string `0x010208a0`, the display time float `0x010162c0` and the
/// function `007052f0(text, 0, icon, 0, time, 0)` (`__cdecl`).
pub(crate) const NO_UNEQUIP_MESSAGE_OBJECT: u32 = 0x011d_31f8;
pub(crate) const MESSAGE_ICON: u32 = 0x0102_08a0;
pub(crate) const MESSAGE_DURATION: u32 = 0x0101_62c0;
pub(crate) const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// Quest note handling of `fn_004c37d0`: `TESActorBaseData::GetAlignmentForKarma`
/// (Xbox PDB, `__cdecl`), the actor test `0047d7c0`, `00403e20` on the
/// object `0x011cde20` (a pointer to the karma reward float), `PlayerCharacter::
/// RewardKarma(int)` and `PlayerCharacter::AddNote(form, 1)` (Xbox PDB).
pub(crate) const ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
pub(crate) const ACTOR_TEST_0047D7C0: u32 = 0x0047_d7c0;
pub(crate) const KARMA_REWARD_OBJECT: u32 = 0x0040_3e20;
pub(crate) const KARMA_REWARD_OWNER: u32 = 0x011c_de20;
pub(crate) const PLAYER_REWARD_KARMA: u32 = 0x0094_fd30;
pub(crate) const PLAYER_ADD_NOTE: u32 = 0x0096_6a70;
/// `Actor::GetCurrentWeapon` (Xbox PDB).
pub(crate) const ACTOR_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `DropItemIntoWorld` callees: the node item address (`006815c0`, returns
/// its `this`), `NiMatrix3::FromEulerAnglesXYZ` (Xbox PDB), the `NiPoint3`
/// constructor `00416870(this, x, y, z)`, matrix times vector
/// `004b4500(this, out, vector)`, `NiPoint3` add `0063c8a0(this, other)`, the
/// actor rotation `00430830`, `TESObjectREFR::GetWorldSpace` (Xbox PDB), the
/// game data manager global and its object placement
/// `004698a0(form, &position, &rotation, cell, world space)`.
pub(crate) const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
pub(crate) const MATRIX_FROM_EULER_ANGLES: u32 = 0x00a5_9540;
pub(crate) const POINT3_CONSTRUCT: u32 = 0x0041_6870;
pub(crate) const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
pub(crate) const POINT3_ADD: u32 = 0x0063_c8a0;
pub(crate) const ACTOR_ROTATION: u32 = 0x0043_0830;
pub(crate) const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
pub(crate) const GAME_DATA_MANAGER_GLOBAL: u32 = 0x011c_3f2c;
pub(crate) const PLACE_OBJECT: u32 = 0x0046_98a0;
/// The offsets (floats `50.0` and `30.0`) of the drop vector and the default
/// rotation (`0x011f426c`).
pub(crate) const DROP_OFFSET_Y: u32 = 0x0101_b268;
pub(crate) const DROP_OFFSET_Z: u32 = 0x0101_8f5c;
pub(crate) const DEFAULT_ROTATION: u32 = 0x011f_426c;
/// The container's own object list (`00717e50`, the address +4 of the
/// container), `TESContainer` form test (`00481eb0`, true when the container
/// lists the form), and `TESPackage::GetObjectTypeFromForm` (Xbox PDB,
/// `__cdecl`).
pub(crate) const CONTAINER_OBJECT_LIST: u32 = 0x0071_7e50;
pub(crate) const CONTAINER_HAS_FORM: u32 = 0x0048_1eb0;
pub(crate) const PACKAGE_OBJECT_TYPE_FROM_FORM: u32 = 0x0067_9ba0;
/// `TESAmmo` type descriptor and `TESActorBase::GetDesirability(this, form)`
/// (Xbox PDB).
pub(crate) const TYPE_TES_AMMO: u32 = 0x0118_40c4;
pub(crate) const GET_DESIRABILITY: u32 = 0x005f_0d60;
/// `fn_004c7300` helpers: the ammunition list of the slot object (`00474a40`,
/// form at +4 when its type is 0x55), the single ammunition (`00474a00`, type
/// 0x29) and the list head of a form list (`00500940`, the address +0x18).
pub(crate) const AMMO_LIST_OF_SLOT: u32 = 0x0047_4a40;
pub(crate) const SINGLE_AMMO_OF_SLOT: u32 = 0x0047_4a00;
pub(crate) const FORM_LIST_HEAD: u32 = 0x0050_0940;
/// `fn_004c7400` callees: `TESObjectWEAP::GetCombatWeaponType` (Xbox PDB),
/// the usability test `008bc9d0(actor, weapon, 1)`, the weapon rating
/// `00646060`, the score rounding `00644540(float, 0, 0)`,
/// `ActorValueOwner::GetClampedActorFloatValue` and
/// `GetClampedActorValue` (Xbox PDB), `Actor::GetFatiguePercentage` (Xbox
/// PDB) and `MiddleHighProcess::GetLastBoundWeapon` (the map's name for
/// `008d85e0`, which returns the weapon's value id).
pub(crate) const COMBAT_WEAPON_TYPE: u32 = 0x0052_2c80;
pub(crate) const ACTOR_CAN_USE_WEAPON: u32 = 0x008b_c9d0;
pub(crate) const RATE_WEAPON: u32 = 0x0064_6060;
pub(crate) const ROUND_SCORE: u32 = 0x0064_4540;
pub(crate) const ACTOR_VALUE_FLOAT: u32 = 0x0066_ef50;
pub(crate) const ACTOR_VALUE_INT: u32 = 0x0066_ef20;
pub(crate) const ACTOR_FATIGUE_PERCENTAGE: u32 = 0x0089_3530;
pub(crate) const WEAPON_VALUE_ID: u32 = 0x008d_85e0;
/// `TESBipedModelForm::GetFormAsBipedModelList`/`FillsBipedSlot` callees of
/// `GetWornItem`: the biped model list getter (`00475020`) and
/// `FillsBipedSlot(this, slot, 0, list)` (`00480af0`).
pub(crate) const GET_FORM_AS_BIPED_MODEL_LIST: u32 = 0x0047_5020;
pub(crate) const BIPED_FILLS_SLOT: u32 = 0x0048_0af0;

/// An entry's number (`ItemChange::iNumber`).
fn entry_number(e: &mut Engine, entry: u32) -> i32 {
    e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber)
}

/// Sets an entry's number (`ItemChange::iNumber`).
fn set_entry_number(e: &mut Engine, entry: u32, value: i32) {
    e.set(Ptr::<ItemChange>::new(entry), ItemChange::iNumber, value);
}

/// More type descriptors and getters used by the third session:
/// `TESObjectARMO`, `TESLevItem`, the weapon form test `0047bcf0` (flag bit
/// 7 at +0x100 clear) and `TESBipedModelForm::GetPlayable` (Xbox PDB).
pub(crate) const TYPE_TES_OBJECT_ARMO: u32 = 0x0118_3a34;
pub(crate) const TYPE_TES_LEV_ITEM: u32 = 0x0118_6f7c;
pub(crate) const WEAPON_PLAYABLE: u32 = 0x0047_bcf0;
pub(crate) const BIPED_PLAYABLE: u32 = 0x0048_0d90;

/// Constants of the food, stolen item and message code (third session):
/// `IngredientItem` and `AlchemyItem` type descriptors, `Actor::IsInFaction`
/// (Xbox PDB), the leveled item position getter `0041d360`, the string
/// object constructor/destructor (`004037b0`, `004037d0`), the formatting
/// functions `00406f60(object, format, ...)` and `00406d00(buffer, size,
/// format, ...)`, `TESFullName::GetFullName(form)` (Xbox PDB, `__cdecl`), the
/// form icon name `0048e730(form, player)`, `Actor::GetPickUpSoundName`
/// (Xbox PDB), the message words `0x011d3db4` and `0x011d3114`, the format
/// strings and the icon directory.
pub(crate) const TYPE_INGREDIENT_ITEM: u32 = 0x0118_39dc;
pub(crate) const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;
pub(crate) const ACTOR_IS_IN_FACTION: u32 = 0x008b_8e90;
pub(crate) const EXTRA_LEVELED_POSITION: u32 = 0x0041_d360;
pub(crate) const STRING_CONSTRUCT: u32 = 0x0040_37b0;
pub(crate) const STRING_DESTRUCT: u32 = 0x0040_37d0;
pub(crate) const FORMAT_STRING: u32 = 0x0040_6f60;
pub(crate) const FORMAT_BUFFER: u32 = 0x0040_6d00;
pub(crate) const FULL_NAME_OF_FORM: u32 = 0x0048_2720;
pub(crate) const FORM_ICON_FOR: u32 = 0x0048_e730;
pub(crate) const PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
pub(crate) const MESSAGE_WORD_ONE: u32 = 0x011d_3db4;
pub(crate) const MESSAGE_WORD_TWO: u32 = 0x011d_3114;
pub(crate) const FORMAT_COUNT_NAME: u32 = 0x0101_c178;
pub(crate) const FORMAT_NAME: u32 = 0x0101_2058;
pub(crate) const FORMAT_PATH: u32 = 0x0101_dcc4;
pub(crate) const ICON_DIRECTORY: u32 = 0x0102_07c0;
pub(crate) const STOLEN_ITEM_ICON: u32 = 0x0102_08e0;
/// `InventoryChanges` item form setter (`00403550(item, form)`, stores the
/// word at +8) and the copy of a container's object list (`00482b00`).
pub(crate) const ITEM_SET_FORM: u32 = 0x0040_3550;
pub(crate) const CONTAINER_OBJECT_COPY: u32 = 0x0048_2b00;

/// Script and list helpers of `fn_004cbbc0`: `BSSimpleList` pop head
/// (`0063f7b0`), the script event list maker (`005abf60`, called on the
/// script), `ExtraDataList::SetScript(list, script)` (Xbox PDB) and the
/// event list setter (`00419f80(list, events)`), `Script::Run(script,
/// reference, locals, 0, 0)` (Xbox PDB), `TESScriptableForm::GetFormScript`
/// (Xbox PDB, `__cdecl`), the unnamed `InventoryChanges` routine
/// `004d1960`, the type byte of a `BSExtraData` (`004f1540`) and
/// `BaseExtraList::AddExtra` (Xbox PDB).
pub(crate) const LIST_POP_HEAD: u32 = 0x0063_f7b0;
pub(crate) const SCRIPT_MAKE_EVENT_LIST: u32 = 0x005a_bf60;
pub(crate) const EXTRA_SET_SCRIPT: u32 = 0x0041_9ed0;
pub(crate) const EXTRA_SET_SCRIPT_EVENTS: u32 = 0x0041_9f80;
pub(crate) const SCRIPT_RUN: u32 = 0x005a_c1e0;
pub(crate) const FORM_SCRIPT_OF: u32 = 0x0048_26d0;
pub(crate) const INVENTORY_FN_004D1960: u32 = 0x004d_1960;
pub(crate) const EXTRA_DATA_TYPE: u32 = 0x004f_1540;
pub(crate) const EXTRA_ADD_EXTRA: u32 = 0x0040_ff60;
pub(crate) const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
pub(crate) const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;
pub(crate) const ADD_CREATED_BASE_OBJECT: u32 = 0x0086_1780;
pub(crate) const SEEN_FORMS_LOOKUP: u32 = 0x0057_c850;
pub(crate) const SEEN_FORMS_INSERT: u32 = 0x0084_d310;
pub(crate) const SEEN_FORMS_CONSTRUCT: u32 = 0x004d_4de0;
pub(crate) const SEEN_FORMS_DESTRUCT: u32 = 0x004d_4eb0;
pub(crate) const PACKAGE_FORM_MATCHES: u32 = 0x0067_9e00;
pub(crate) const TYPE_TES_SCRIPTABLE_FORM: u32 = 0x0118_3254;
pub(crate) const MESSAGE_WORD_THREE: u32 = 0x011d_4e7c;
pub(crate) const AFTER_TRANSFER_REFRESH: u32 = 0x0070_4af0;

/// Runs `body` between the scope guard `00404eb0(guard, 0x36, 1, file,
/// line)` and `00404ee0(guard)` (the guard is a 4-byte local of the game).
fn with_scope_guard<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_OPEN,
            &args![guard, 0x36u32, 1u32, SOURCE_FILE_NAME, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_CLOSE, &args![guard]);
        result
    })
}

/// The form type byte of `form` (`00401170`).
fn form_type_of(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_TYPE, &args![form]).u32()
}

/// The save/load game object the save size/save/load methods run on.
fn save_load_game(e: &mut Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME_GLOBAL)
}

/// Whether the "log the save sizes" flag byte is set (`00408d60` on the
/// flag object returns the byte's address).
fn save_size_logging(e: &mut Engine) -> bool {
    let flag = e
        .call(FLAG_BYTE_ADDRESS, &args![SAVE_SIZE_LOG_OBJECT])
        .u32();
    e.mem.u8(flag) != 0
}

/// `new ItemChange(form, number)`: 0xC bytes, constructed unless the
/// allocation failed.
fn new_item_change(e: &mut Engine, form: u32, number: u32) -> u32 {
    let item = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if item == 0 {
        return 0;
    }
    item_change_item_change(e, Ptr::new(item), Ptr::new(form), number as i32).addr()
}

/// `delete item` of an `ItemChange` through `004459e0(item, 1)`, nothing for
/// null.
fn delete_item_change(e: &mut Engine, item: u32) {
    if item != 0 {
        e.call(ITEM_CHANGE_DELETE, &args![item, 1u32]);
    }
}

/// Makes sure `*(item + 0)` (the `pExtraObjectList` of an `ItemChange`, the
/// list of an entry) is a list: creates the empty head node when it is null.
fn ensure_extra_list(e: &mut Engine, item: u32) {
    if e.mem.u32(item) == 0 {
        let node = new_list_node(e);
        e.mem.set_u32(item, node);
    }
}

/// The first part of the error messages of the save size and save code: the
/// log line for the form being saved (`world`, the word the save/load object
/// keeps) or without one.
fn log_save_size(e: &mut Engine, saved: u32, current: u32, line: u32, formats: (u32, u32)) {
    if current != 0 {
        let form_id = e.mem.u32(current);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        // The 0x130 virtual returns the `%s` of the message; the two words
        // pushed before it stay on the stack as the message's own arguments.
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let flags = e.mem.u32(current + 5);
        e.call(
            ERROR_LOG,
            &args![
                formats.0,
                saved,
                form_id,
                name,
                flags,
                line,
                SOURCE_FILE_NAME
            ],
        );
    } else {
        e.call(ERROR_LOG, &args![formats.1, saved, line, SOURCE_FILE_NAME]);
    }
}

/// The head node of the extra list list of `this`.
fn list_head(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    e.get(this, ItemChange::pExtraObjectList).addr()
}

/// The item's form (`pContainerObj`).
fn container_object(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    e.get(this, ItemChange::pContainerObj).addr()
}

/// The first extra data list of `this`, or 0: the head node is nonnull and its
/// item word is set (the check every accessor of the unit starts with).
fn first_item(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let node = list_head(e, this);
    if node == 0 {
        return 0;
    }
    list_item(e, node)
}

/// `delete object` through its scalar deleting destructor (slot 0), nothing
/// for null.
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

/// Calls a list method that takes the address of an item word
/// (`list->Method(&item)`).
fn list_call_with_item(e: &mut Engine, method: u32, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(method, &args![list, slot]);
    });
}

/// `new ExtraDataList`: 0x20 bytes, constructed unless the allocation failed.
fn new_extra_list(e: &mut Engine) -> u32 {
    let extra = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    if extra == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_LIST_CONSTRUCT, &args![extra]).u32()
}

/// `new BSSimpleList` node: 8 bytes, constructed unless the allocation
/// failed.
fn new_list_node(e: &mut Engine) -> u32 {
    let node = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if node == 0 {
        return 0;
    }
    e.call(LIST_NODE_CONSTRUCT, &args![node]).u32()
}

/// `_ftol2_sse` on a float held in ST0.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// `FISTP` with the truncating rounding mode set: out of range (and NaN)
/// values give the "integer indefinite" `0x80000000`.
fn truncate_to_i32(value: f64) -> i32 {
    let t = value.trunc();
    if t.is_nan() || !(-2147483648.0..2147483648.0).contains(&t) {
        i32::MIN
    } else {
        t as i32
    }
}

// Translated from 004bc550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::ItemChange` (Xbox PDB): allocates an empty list node for the
/// extra lists and stores the item's form and the count.
pub fn item_change_item_change(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    container_obj: Ptr,
    number: i32,
) -> Ptr<ItemChange> {
    e.set(this, ItemChange::pContainerObj, container_obj);
    let node = new_list_node(e);
    e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
    e.set(this, ItemChange::iNumber, number);
    this
}

// Translated from 004bc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the list of extra data lists: clears it (`00470470`), deletes
/// the list (`004702f0(list, 1)`) and nulls the field. The extra lists
/// themselves are not deleted.
pub fn fn_004bc5f0(e: &mut Engine, this: Ptr<ItemChange>) {
    let list = list_head(e, this);
    if list != 0 {
        e.call(LIST_CLEAR, &args![list]);
        let list = list_head(e, this);
        if list != 0 {
            e.call(LIST_DESTROY, &args![list, 1u32]);
        }
    }
    e.set(this, ItemChange::pExtraObjectList, Ptr::NULL);
}

// Translated from 004bc650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::ItemChange_ov2` (Xbox PDB), the copy constructor: copies the
/// form, makes an empty list and appends a duplicate
/// (`DuplicateExtraListForContainer`) of every extra list of `other`, then
/// copies the count.
pub fn item_change_item_change_ov2(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    other: Ptr<ItemChange>,
) -> Ptr<ItemChange> {
    let form = container_object(e, other);
    e.set(this, ItemChange::pContainerObj, Ptr::new(form));
    let node = new_list_node(e);
    e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
    let mut cursor = list_head(e, other);
    while cursor != 0 {
        let source = list_item(e, cursor);
        if source == 0 {
            break;
        }
        let copy = new_extra_list(e);
        e.call(EXTRA_DATA_LIST_DUPLICATE, &args![copy, source]);
        let list = list_head(e, this);
        list_call_with_item(e, LIST_ADD_TAIL, list, copy);
        cursor = list_next(e, cursor);
    }
    let number = e.get(other, ItemChange::iNumber);
    e.set(this, ItemChange::iNumber, number);
    this
}

// Translated from 004bc780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::DeleteAllExtra` (Xbox PDB): for every extra list, removes all
/// its extras (`RemoveAll(true)`), takes it out of the list (the removal is
/// started from the current node, as the game does) and deletes it.
pub fn item_change_delete_all_extra(e: &mut Engine, this: Ptr<ItemChange>) {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        e.call(EXTRA_REMOVE_ALL, &args![extra, 1u32]);
        list_call_with_item(e, LIST_REMOVE, cursor, extra);
        delete_object(e, extra);
        cursor = list_next(e, cursor);
    }
}

// Translated from 004bc810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shallow copy constructor from another `ItemChange`: when `other` is not
/// null, copies its form, makes an empty list and appends the same extra
/// list pointers (no duplicates) of `other`'s list.
pub fn fn_004bc810(e: &mut Engine, this: Ptr<ItemChange>, other: Ptr<ItemChange>) {
    if other.is_null() {
        return;
    }
    let form = container_object(e, other);
    e.set(this, ItemChange::pContainerObj, Ptr::new(form));
    let node = new_list_node(e);
    e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
    let mut cursor = e.call(WORD_AT_0, &args![other]).u32();
    while cursor != 0 {
        let item = list_item(e, cursor);
        if item == 0 {
            break;
        }
        let list = e.call(WORD_AT_0, &args![this]).u32();
        list_call_with_item(e, LIST_ADD, list, item);
        cursor = list_next(e, cursor);
    }
}

// Translated from 004bc8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetExtraTotalCount` (Xbox PDB): the sum of the counts of the
/// extra lists that are not the container's default one and, unless
/// `include_worn`, are not worn.
pub fn item_change_get_extra_total_count(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    include_worn: bool,
) -> i32 {
    let mut cursor = list_head(e, this);
    let mut total = 0i32;
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        let is_default = e
            .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![extra, 0u32])
            .bool();
        if !is_default && (include_worn || !e.call(EXTRA_GET_WORN, &args![extra, 0u32]).bool()) {
            let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
            total += count as i32;
        }
        cursor = list_next(e, cursor);
    }
    total
}

// Translated from 004bc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetExtraTotalDefaultCount` (Xbox PDB): the sum of the counts
/// of the container's default extra lists. A list left without any extras
/// (`fn_004bca60`) is removed and deleted, and the walk starts over with a
/// total of zero.
pub fn item_change_get_extra_total_default_count(e: &mut Engine, this: Ptr<ItemChange>) -> i32 {
    let mut cursor = list_head(e, this);
    let mut total = 0i32;
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![extra, 0u32])
            .bool()
        {
            let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
            total += count as i32;
        }
        cursor = list_next(e, cursor);
        if fn_004bca60(e, Ptr::new(extra)) {
            let list = list_head(e, this);
            list_call_with_item(e, LIST_REMOVE, list, extra);
            delete_object(e, extra);
            total = 0;
            cursor = list_head(e, this);
        }
    }
    total
}

// Translated from 004bca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an extra data list has no extras (its first extra pointer, +4, is
/// null).
pub fn fn_004bca60(e: &mut Engine, extra_list: Ptr) -> bool {
    e.mem.u32(extra_list.addr() + 4) == 0 // BaseExtraList::pHead
}

// Translated from 004bca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetAmountNonDefaultExtra` (Xbox PDB): the number of extra
/// lists that are not the container's default and have a positive count. A
/// list that holds one item type (`ItemsInList == 1`) with a count above 1
/// has its count removed, is taken out of the list and deleted, and the walk
/// starts over with a total of zero.
pub fn item_change_get_amount_non_default_extra(e: &mut Engine, this: Ptr<ItemChange>) -> i32 {
    let mut cursor = list_head(e, this);
    let mut total = 0i32;
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
        let is_default = e
            .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![extra, 0u32])
            .bool();
        if !is_default && count > 0 {
            total += 1;
        }
        cursor = list_next(e, cursor);
        if e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).i32() == 1 && count > 1 {
            e.call(EXTRA_REMOVE_COUNT, &args![extra]);
            let list = list_head(e, this);
            list_call_with_item(e, LIST_REMOVE, list, extra);
            delete_object(e, extra);
            total = 0;
            cursor = list_head(e, this);
        }
    }
    total
}

// Translated from 004bcb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any extra list has a leveled item extra (`HasLeveledItem`).
pub fn fn_004bcb70(e: &mut Engine, this: Ptr<ItemChange>) -> bool {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_HAS_LEVELED_ITEM, &args![extra]).bool() {
            return true;
        }
        cursor = list_next(e, cursor);
    }
    false
}

// Translated from 004bcbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::RemoveOriginalChanges` (Xbox PDB): removes the original
/// reference extra from every extra list that has one.
pub fn item_change_remove_original_changes(e: &mut Engine, this: Ptr<ItemChange>) {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_ORIGINAL_REFERENCE, &args![extra]).u32() != 0 {
            e.call(EXTRA_REMOVE_ORIGINAL_REFERENCE, &args![extra]);
        }
        cursor = list_next(e, cursor);
    }
}

// Translated from 004bcc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::NumberLeveledExtra` (Xbox PDB): the sum of the counts of the
/// extra lists that have a leveled item extra.
pub fn item_change_number_leveled_extra(e: &mut Engine, this: Ptr<ItemChange>) -> i32 {
    let mut cursor = list_head(e, this);
    let mut total = 0i32;
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_HAS_LEVELED_ITEM, &args![extra]).bool() {
            let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
            total += count as i32;
        }
        cursor = list_next(e, cursor);
    }
    total
}

// Translated from 004bccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any extra list passes the test `0041d030` (an `ExtraDataList`
/// predicate the map does not name).
pub fn fn_004bccb0(e: &mut Engine, this: Ptr<ItemChange>) -> bool {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_TEST_D030, &args![extra]).bool() {
            return true;
        }
        cursor = list_next(e, cursor);
    }
    false
}

// Translated from 004bcd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::RemoveLeveledItemExtra` (Xbox PDB): removes the leveled item
/// extra from every extra list that has one; a list left without extras is
/// taken out of the list (not deleted) and the walk resumes after the head.
/// Returns the last leveled item extra it removed.
pub fn item_change_remove_leveled_item_extra(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let mut cursor = list_head(e, this);
    let mut removed = 0u32;
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_LEVELED_ITEM, &args![extra]).u32() != 0 {
            removed = e.call(EXTRA_GET_LEVELED_ITEM, &args![extra]).u32();
            e.call(EXTRA_REMOVE_EXTRA, &args![extra, removed, 0u32]);
            if fn_004bca60(e, Ptr::new(extra)) {
                let list = list_head(e, this);
                list_call_with_item(e, LIST_REMOVE, list, extra);
            }
            cursor = list_head(e, this);
        }
        cursor = list_next(e, cursor);
    }
    removed
}

// Translated from 004bcdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetItemHealth` (Xbox PDB): the health of the item. The
/// base health is the form's (`GetFormHealth` through the health form's
/// virtual slot 0x10), or for a weapon `TESObjectWEAP::GetFormHealth` with
/// the mod effect flag. With `as_percent` false the first extra list's health
/// if it has one, else the base health; with it true the percentage of the
/// base health (the ratio limited to 100, times 100) or 100 without a
/// health extra.
/// `-1.0` for a form without health.
pub fn item_change_get_item_health(e: &mut Engine, this: Ptr<ItemChange>, as_percent: bool) -> f32 {
    let form = container_object(e, this);
    let health_form = e.call(GET_FORM_AS_HEALTH_FORM, &args![form]).u32();
    if health_form == 0 {
        return e.global::<f32>(MINUS_ONE_FLOAT);
    }
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_OBJECT_WEAP,
                0u32
            ],
        )
        .u32();
    let mut base_health = e.vcall(health_form, 0x10, &args![]).i32();
    if weapon != 0 {
        let mod_active = item_change_has_mod_effect_active_ov2(e, this, MOD_EFFECT_HEALTH as u8);
        base_health = tes_object_weap_get_form_health(e, Ptr::new(weapon), mod_active as u8);
    }
    let extra = first_item(e, this);
    if extra != 0 {
        let health = e.call(EXTRA_GET_HEALTH, &args![extra]).f32();
        let none: f64 = e.global(MINUS_ONE_DOUBLE);
        if health as f64 != none {
            if !as_percent {
                return health;
            }
            let ratio = (health as f64 / base_health as f64) as f32;
            let cap: f32 = e.global(HUNDRED_FLOAT);
            let limited = e.call(FLOAT_MIN, &args![cap, ratio]).f32();
            let hundred: f64 = e.global(HUNDRED_DOUBLE);
            return (limited as f64 * hundred) as f32;
        }
    }
    if as_percent {
        e.global::<f32>(HUNDRED_FLOAT)
    } else {
        base_health as f32
    }
}

// Translated from 004bcf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetFormHealth` (Xbox PDB): the weapon's health, with
/// `with_mods` plus the health mod effect value (type 10), truncated.
pub fn tes_object_weap_get_form_health(e: &mut Engine, this: Ptr, with_mods: u8) -> i32 {
    // TESHealthForm sub-object at +0x94, TESHealthForm::iHealth (Xbox PDB).
    let health_form = this.addr() + WEAPON_HEALTH_FORM;
    if with_mods != 0 {
        let bonus = tes_object_weap_get_mod_effect_value(e, this, MOD_EFFECT_HEALTH, 0);
        let health = e.call(WORD_AT_4, &args![health_form]).u32();
        let total = health as f64 + bonus as f64;
        float_to_int(e, total)
    } else {
        e.call(WORD_AT_4, &args![health_form]).i32()
    }
}

// Translated from 004bcf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetModEffectValue` (Xbox PDB): the value of the weapon's
/// mod action of type `effect`: the first (`variant` 0) or second (1) value
/// of the action slot that has that type, 0.0 for another variant or when no
/// slot has the type.
pub fn tes_object_weap_get_mod_effect_value(
    e: &mut Engine,
    this: Ptr,
    effect: u32,
    variant: u8,
) -> f32 {
    let base = this.addr();
    for slot in 0..3u32 {
        // OBJ_WEAP::eModActionOne..Three (Xbox PDB).
        if e.mem.u32(base + WEAPON_MOD_ACTION + 4 * slot) == effect {
            return match variant {
                // fModActionOneValue.. / fModActionOneValueTwo.. (Xbox PDB)
                0 => e.mem.f32(base + WEAPON_MOD_ACTION_VALUE + 4 * slot),
                1 => e.mem.f32(base + WEAPON_MOD_ACTION_VALUE_TWO + 4 * slot),
                _ => 0.0,
            };
        }
    }
    0.0
}

// Translated from 004bd030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::SetItemHealth` (Xbox PDB): sets the health of the extra list
/// `extra` (of this item) to `health`. `inventory_changes` is the owning
/// `InventoryChanges`; when its owner (+4, `pRef`) is set, that object gets
/// its virtual slot 0x48 called with 0x20. For a form with health: if the
/// new health is below the form's health or the current health is above it,
/// the health is stored in `extra` (found in the list, or in a new extra
/// list added to it; a new list node is made when there is none); otherwise
/// the health extra is removed from `extra`, and when that leaves the list
/// empty and `remove_empty` is set the list is taken out of the entry
/// `GetObjectInList` finds and deleted.
pub fn item_change_set_item_health(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    health: f32,
    inventory_changes: Ptr,
    extra: Ptr,
    remove_empty: bool,
) {
    let form = container_object(e, this);
    let health_form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_HEALTH_FORM,
                0u32
            ],
        )
        .u32();
    let form_health = e.call(GET_FORM_HEALTH, &args![form]).i32();
    let current = item_change_get_item_health(e, this, false);
    let current = float_to_int(e, current as f64);
    // InventoryChanges::pRef (Xbox PDB) +4.
    let owner = e.call(WORD_AT_4, &args![inventory_changes]).u32();
    if owner != 0 {
        e.vcall(owner, 0x48, &args![0x20u32]);
    }
    if health_form == 0 {
        return;
    }
    if (health as f64) < form_health as f64 || current > form_health {
        let list = list_head(e, this);
        if list == 0 {
            let new_extra = new_extra_list(e);
            let node = new_list_node(e);
            e.call(EXTRA_SET_HEALTH_NEW, &args![new_extra, health]);
            list_call_with_item(e, LIST_ADD, node, new_extra);
            e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
            return;
        }
        let mut cursor = list;
        while cursor != 0 {
            let item = list_item(e, cursor);
            if item == 0 {
                break;
            }
            if item == extra.addr() {
                e.call(EXTRA_SET_HEALTH, &args![item, health]);
                return;
            }
            cursor = list_next(e, cursor);
        }
        let new_extra = new_extra_list(e);
        e.call(EXTRA_SET_HEALTH, &args![new_extra, health]);
        let list = list_head(e, this);
        list_call_with_item(e, LIST_ADD, list, new_extra);
    } else {
        let mut cursor = list_head(e, this);
        while cursor != 0 {
            let item = list_item(e, cursor);
            if item == 0 {
                break;
            }
            if item == extra.addr() {
                e.call(EXTRA_REMOVE_HEALTH, &args![item]);
                if fn_004bca60(e, Ptr::new(item)) && remove_empty {
                    let form = e.call(WORD_AT_8, &args![this]).u32();
                    let entry = e
                        .call(
                            INVENTORY_CHANGES_GET_OBJECT_IN_LIST,
                            &args![inventory_changes, form, 1u32, 0u32],
                        )
                        .u32();
                    let entry_list = e.call(WORD_AT_0, &args![entry]).u32();
                    list_call_with_item(e, LIST_REMOVE, entry_list, item);
                    delete_object(e, item);
                }
                return;
            }
            cursor = list_next(e, cursor);
        }
    }
}

// Translated from 004bd350 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float for the item: when its form is an enchantable form (the cast to
/// `TESEnchantableForm`) the first extra list's float extra (`004187a0`) if
/// it has one (not `-1.0`), else the form's 16-bit value (`004a8ae0`); `-1.0`
/// for a form that is not enchantable.
pub fn fn_004bd350(e: &mut Engine, this: Ptr<ItemChange>) -> f32 {
    let form = container_object(e, this);
    let enchantable = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_ENCHANTABLE_FORM,
                0u32
            ],
        )
        .u32();
    if enchantable == 0 {
        return e.global::<f32>(MINUS_ONE_FLOAT);
    }
    let extra = first_item(e, this);
    if extra != 0 {
        let value = e.call(EXTRA_GET_FLOAT_EXTRA, &args![extra]).f32();
        let none: f64 = e.global(MINUS_ONE_DOUBLE);
        if value as f64 != none {
            return e.call(EXTRA_GET_FLOAT_EXTRA, &args![extra]).f32();
        }
    }
    e.call(ENCHANTABLE_FORM_VALUE, &args![enchantable]).u16() as f32
}

// Translated from 004bd400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetItemValue` (Xbox PDB): the item's price: the form's value
/// (`GetFormValue`) scaled by the item's health percentage
/// (`00647c00(value, health)`), plus for a weapon the value word of each
/// mod object whose slot bit (1, 2, 4) is set in `GetModSlots`, rounded
/// with `fn_004bd510(price, 0.1)`.
pub fn item_change_get_item_value(e: &mut Engine, this: Ptr<ItemChange>) -> f32 {
    let health = item_change_get_item_health(e, this, true);
    let form = container_object(e, this);
    let value = e.call(GET_FORM_VALUE, &args![form]).i32();
    let mut price = e.call(CALC_ITEM_PRICE, &args![value as f32, health]).f32();
    let form = e.call(WORD_AT_8, &args![this]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_OBJECT_WEAP,
                0u32
            ],
        )
        .u32();
    if weapon != 0 {
        let slots = item_change_get_mod_slots(e, this) as u32;
        if slots != 0 {
            for bit in [1u32, 2, 4] {
                if slots & bit != 0 {
                    let mod_object = fn_004bd570(e, Ptr::new(weapon), bit);
                    // TESObjectIMOD value word.
                    let word = e
                        .call(WORD_AT_4, &args![mod_object + MOD_OBJECT_VALUE])
                        .i32();
                    price = (word as f64 + price as f64) as f32;
                }
            }
        }
    }
    let step: f32 = e.global(PRICE_STEP);
    fn_004bd510(e, price, step)
}

// Translated from 004bd510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rounds `value` to the nearest multiple of `step`: `value / step` is
/// truncated and one is added when the fraction is at least 0.5.
pub fn fn_004bd510(e: &mut Engine, value: f32, step: f32) -> f32 {
    let quotient = (value as f64 / step as f64) as f32;
    let whole = float_to_int(e, quotient as f64);
    let fraction = quotient as f64 - whole as f64;
    let half: f64 = e.global(HALF_DOUBLE);
    // `fcomp`; the carry is only left out when the fraction is below 0.5.
    let carry = if fraction < half { 0 } else { 1 };
    let rounded = float_to_int(e, quotient as f64) + carry;
    (rounded as f64 * step as f64) as f32
}

// Translated from 004bd570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's mod object for slot bit 1, 2 or 4 (`pModObjectOne..Three`,
/// Xbox PDB), else 0.
pub fn fn_004bd570(e: &mut Engine, this: Ptr, slot_bit: u32) -> u32 {
    let index = match slot_bit {
        1 => 0,
        2 => 1,
        4 => 2,
        _ => return 0,
    };
    e.mem.u32(this.addr() + WEAPON_MOD_OBJECT + 4 * index)
}

// Translated from 004bd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a source extra list `source`: copies it (`00419700`) into the first
/// extra list, or into a new extra list that is added to the list (made
/// first when there is none). Without one: removes the first extra list
/// from the list and deletes it.
pub fn fn_004bd5c0(e: &mut Engine, this: Ptr<ItemChange>, source: Ptr) {
    if !source.is_null() {
        let list = list_head(e, this);
        if list != 0 {
            let first = list_item(e, list);
            if first != 0 {
                e.call(EXTRA_COPY_FROM, &args![first, source]);
                return;
            }
        }
        let new_extra = new_extra_list(e);
        e.call(EXTRA_COPY_FROM, &args![new_extra, source]);
        if list_head(e, this) == 0 {
            let node = new_list_node(e);
            e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
        }
        let list = list_head(e, this);
        list_call_with_item(e, LIST_ADD, list, new_extra);
    } else {
        let list = list_head(e, this);
        if list != 0 {
            let first = list_item(e, list);
            if first != 0 {
                list_call_with_item(e, LIST_REMOVE, list, first);
                delete_object(e, first);
            }
        }
    }
}

// Translated from 004bd740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetItemOwnership` (Xbox PDB): the first extra list's
/// `00418660` value, 0 when there is none.
pub fn item_change_get_item_ownership(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let extra = first_item(e, this);
    if extra != 0 && e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32() != 0 {
        return e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32();
    }
    0
}

// Translated from 004bd7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetHotKey` (Xbox PDB): the hot key (a signed byte) of the
/// first extra list that has one (not negative), else -1.
pub fn item_change_get_hot_key(e: &mut Engine, this: Ptr<ItemChange>) -> i32 {
    let mut hot_key = -1i32;
    let mut cursor = e.call(WORD_AT_0, &args![this]).u32();
    while hot_key < 0 && cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_HOT_KEY, &args![extra]).u8() as i8 >= 0 {
            hot_key = e.call(EXTRA_GET_HOT_KEY, &args![extra]).u8() as i8 as i32;
        }
        cursor = list_next(e, cursor);
    }
    hot_key
}

// Translated from 004bd820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetModSlots` (Xbox PDB): the weapon mod flags of the first
/// extra list when it has weapon mods, else 0.
pub fn item_change_get_mod_slots(e: &mut Engine, this: Ptr<ItemChange>) -> u8 {
    let extra = first_item(e, this);
    if extra != 0 && e.call(EXTRA_HAS_WEAPON_MODS, &args![extra]).bool() {
        return e.call(EXTRA_GET_WEAPON_MOD_FLAGS, &args![extra]).u8();
    }
    0
}

// Translated from 004bd880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's mod action type (`eModActionOne..Three`, Xbox PDB) of slot
/// bit 1, 2 or 4, else 0.
pub fn fn_004bd880(e: &mut Engine, this: Ptr, slot_bit: u32) -> u32 {
    let index = match slot_bit {
        1 => 0,
        2 => 1,
        4 => 2,
        _ => return 0,
    };
    e.mem.u32(this.addr() + WEAPON_MOD_ACTION + 4 * index)
}

// Translated from 004bd8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::HasModEffectActive` (Xbox PDB): whether the first extra
/// list has an active weapon mod slot (1, 2, then 4) whose mod action type
/// is `effect`; the value of that action (`fn_004bd9d0`, first value) is
/// stored through `value_out`.
pub fn item_change_has_mod_effect_active(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    effect: u8,
    value_out: Ptr,
) -> bool {
    let extra = first_item(e, this);
    if extra == 0 {
        return false;
    }
    let weapon = container_object(e, this);
    for slot_bit in [1u32, 2, 4] {
        if e.call(EXTRA_GET_WEAPON_MOD_SLOT_ACTIVE, &args![extra, slot_bit])
            .bool()
            && fn_004bd880(e, Ptr::new(weapon), slot_bit) == effect as u32
        {
            let value = fn_004bd9d0(e, Ptr::new(weapon), slot_bit, 0);
            e.mem.set_f32(value_out.addr(), value);
            return true;
        }
    }
    false
}

// Translated from 004bd9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's mod action value (`fModActionOneValue..` for `variant` 0,
/// `fModActionOneValueTwo..` for 1; Xbox PDB) of slot bit 1, 2 or 4, else 0.0.
pub fn fn_004bd9d0(e: &mut Engine, this: Ptr, slot_bit: u32, variant: u8) -> f32 {
    let index = match slot_bit {
        1 => 0,
        2 => 1,
        4 => 2,
        _ => return 0.0,
    };
    match variant {
        0 => e.mem.f32(this.addr() + WEAPON_MOD_ACTION_VALUE + 4 * index),
        1 => e
            .mem
            .f32(this.addr() + WEAPON_MOD_ACTION_VALUE_TWO + 4 * index),
        _ => 0.0,
    }
}

// Translated from 004bda70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB): as `HasModEffectActive`
/// without the value.
pub fn item_change_has_mod_effect_active_ov2(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    effect: u8,
) -> bool {
    let extra = first_item(e, this);
    if extra == 0 {
        return false;
    }
    let weapon = container_object(e, this);
    for slot_bit in [1u32, 2, 4] {
        if e.call(EXTRA_GET_WEAPON_MOD_SLOT_ACTIVE, &args![extra, slot_bit])
            .bool()
            && fn_004bd880(e, Ptr::new(weapon), slot_bit) == effect as u32
        {
            return true;
        }
    }
    false
}

// Translated from 004bdb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::SetIsModding` (Xbox PDB): sets the modding flag on the first
/// extra list; when there is none, makes the list node (if the item has no
/// list) and a new extra list (added to it) first.
pub fn item_change_set_is_modding(e: &mut Engine, this: Ptr<ItemChange>) {
    let mut list = list_head(e, this);
    if list == 0 {
        list = new_list_node(e);
        e.set(this, ItemChange::pExtraObjectList, Ptr::new(list));
    }
    let first = list_item(e, list);
    if first == 0 {
        let extra = new_extra_list(e);
        e.call(EXTRA_SET_IS_MODDING, &args![extra, 1u32]);
        let list = list_head(e, this);
        list_call_with_item(e, LIST_ADD, list, extra);
    } else {
        e.call(EXTRA_SET_IS_MODDING, &args![first, 1u32]);
    }
}

// Translated from 004bdc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::RemoveIsModding` (Xbox PDB): removes the modding flag from
/// the first extra list.
pub fn item_change_remove_is_modding(e: &mut Engine, this: Ptr<ItemChange>) {
    let extra = first_item(e, this);
    if extra != 0 {
        e.call(EXTRA_REMOVE_IS_MODDING, &args![extra]);
    }
}

// Translated from 004bdcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetPoison` (Xbox PDB): the poison of the first extra list,
/// 0 when there is none.
pub fn item_change_get_poison(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let extra = first_item(e, this);
    if extra != 0 && e.call(EXTRA_GET_POISON, &args![extra]).u32() != 0 {
        return e.call(EXTRA_GET_POISON, &args![extra]).u32();
    }
    0
}

// Translated from 004bdd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the first extra list has no poison, calls `00419d10` (the map's
/// `ExtraDataList::SetRank`) on it with `value`. The game's function returns
/// whatever is left in `EAX`.
pub fn fn_004bdd20(e: &mut Engine, this: Ptr<ItemChange>, value: u32) {
    let extra = first_item(e, this);
    if extra != 0 && e.call(EXTRA_GET_POISON, &args![extra]).u32() == 0 {
        e.call(EXTRA_SET_RANK, &args![extra, value]);
    }
}

// Translated from 004bdd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the first extra list has a poison, removes it (`0041af30`).
pub fn fn_004bdd80(e: &mut Engine, this: Ptr<ItemChange>) {
    let extra = first_item(e, this);
    if extra != 0 && e.call(EXTRA_GET_POISON, &args![extra]).u32() != 0 {
        e.call(EXTRA_REMOVE_POISON, &args![extra]);
    }
}

// Translated from 004bddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetWorn` (Xbox PDB): whether any extra list is worn
/// (`ExtraDataList::GetWorn(flag)`).
pub fn item_change_get_worn(e: &mut Engine, this: Ptr<ItemChange>, flag: u8) -> bool {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_WORN, &args![extra, flag as u32]).bool() {
            return true;
        }
        cursor = list_next(e, cursor);
    }
    false
}

// Translated from 004bde40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetScript` (Xbox PDB): the first script found on an extra
/// list, 0 when none.
pub fn item_change_get_script(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_SCRIPT, &args![extra]).u32() != 0 {
            return e.call(EXTRA_GET_SCRIPT, &args![extra]).u32();
        }
        cursor = list_next(e, cursor);
    }
    0
}

// Translated from 004bdea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetScriptLocals` (Xbox PDB): the first script locals found
/// on an extra list, 0 when none.
pub fn item_change_get_script_locals(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_SCRIPT_LOCALS, &args![extra]).u32() != 0 {
            return e.call(EXTRA_GET_SCRIPT_LOCALS, &args![extra]).u32();
        }
        cursor = list_next(e, cursor);
    }
    0
}

// Translated from 004bdf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetModifiedAttackDamage` (Xbox PDB): for a form of type
/// 0x28, `CalcWeaponDamage(attacker, form, health ratio, scale, 0, flag,
/// this, 1)` where the ratio is the item's health over the form's health;
/// 0.0 for any other form.
pub fn item_change_get_modified_attack_damage(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    attacker: Ptr,
    scale: f32,
    flag: u8,
) -> f32 {
    let form = container_object(e, this);
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    if form_type == 0x28 {
        let health = item_change_get_item_health(e, this, false);
        let base = e.call(GET_FORM_HEALTH, &args![form]).u32();
        let ratio = (health as f64 / base as f64) as f32;
        e.call(
            CALC_WEAPON_DAMAGE,
            &args![attacker, form, ratio, scale, 0u32, flag as u32, this, 1u32],
        )
        .f32()
    } else {
        e.call(FORM_TYPE, &args![form]);
        0.0
    }
}

// Translated from 004bdf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The armor rating of the item: for a form of type 0x18, the form's rating
/// (`004be060`) truncated, with the item's health ratio, through
/// `CalcArmorRating(rating, ratio)`; the result (or `-1.0` for another form)
/// goes through `00476b20`. The parameter is not read. (The game also
/// pushes the ratio for `004be060`, which does not take it; that word is the
/// second argument of `CalcArmorRating`.)
pub fn fn_004bdf90(e: &mut Engine, this: Ptr<ItemChange>, _unused_1: u32) -> f32 {
    let mut result = e.global::<f32>(MINUS_ONE_FLOAT);
    let form = container_object(e, this);
    if e.call(FORM_TYPE, &args![form]).u32() == 0x18 {
        let form_health = e.call(GET_FORM_HEALTH, &args![form]).u32() as f32;
        let ratio = if form_health == 0.0 {
            0.0f32
        } else {
            let health = item_change_get_item_health(e, this, false);
            (health as f64 / form_health as f64) as f32
        };
        let rating = e.call(ARMOR_RATING_FLOAT, &args![form]).f32();
        let whole = truncate_to_i32(rating as f64);
        result = e
            .call(CALC_ARMOR_RATING, &args![whole as u16 as u32, ratio])
            .f32();
    }
    e.call(ROUND_ARMOR_RESULT, &args![result]).f32()
}

// Translated from 004be060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The armor's rating as a float: forwards to `004be080` (the game's
/// wrapper).
pub fn fn_004be060(e: &mut Engine, this: Ptr) -> f32 {
    fn_004be080(e, this)
}

// Translated from 004be080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The armor's rating as a float: `OBJ_ARMO::sRating` (a 16-bit value at
/// +0x178 of a `TESObjectARMO`) divided by 100.
pub fn fn_004be080(e: &mut Engine, this: Ptr) -> f32 {
    let rating = e.mem.u16(this.addr() + ARMOR_RATING);
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    (rating as f64 / hundred) as f32
}

// Translated from 004be0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The damage threshold of the item as the game shows it: for a form of type
/// 0x18 the form's `fDamageThreshold` (`004be180`) truncated to 16 bits goes
/// through `CalcArmorRating(threshold, ratio)` with the item's health ratio
/// (the item health over the form's health, zero without form health);
/// another form gives `-1.0`. The result goes through `00476b20`. The stack
/// word is not read.
pub fn fn_004be0b0(e: &mut Engine, this: Ptr<ItemChange>, _unused_1: u32) -> f32 {
    let mut result = e.global::<f32>(MINUS_ONE_FLOAT);
    let form = container_object(e, this);
    if form_type_of(e, form) == 0x18 {
        let form_health = e.call(GET_FORM_HEALTH, &args![form]).u32() as f32;
        let zero: f64 = e.global(ZERO_DOUBLE);
        let ratio = if form_health as f64 == zero {
            0.0f32
        } else {
            let health = item_change_get_item_health(e, this, false);
            (health as f64 / form_health as f64) as f32
        };
        let threshold = fn_004be180(e, Ptr::new(form));
        let whole = truncate_to_i32(threshold as f64);
        result = e
            .call(CALC_ARMOR_RATING, &args![whole as u16 as u32, ratio])
            .f32();
    }
    e.call(ROUND_ARMOR_RESULT, &args![result]).f32()
}

// Translated from 004be180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OBJ_ARMO::fDamageThreshold` (Xbox PDB) of a `TESObjectARMO`, at +0x17c.
pub fn fn_004be180(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + ARMOR_DAMAGE_THRESHOLD)
}

// Translated from 004be1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A small class number of the item's form: 4 for form types 0x18 and 0x1a,
/// 3 for 0x28, 5 for every other form. (The meaning of the numbers is not
/// confirmed by the exe.)
pub fn fn_004be1a0(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let form = container_object(e, this);
    match form_type_of(e, form) {
        0x18 | 0x1a => 4,
        0x28 => 3,
        _ => 5,
    }
}

// Translated from 004be200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetIconOne` (Xbox PDB): the inventory icon path of the item.
/// For a biped form (`TESBipedModelForm`) whose item is owned by a `TESNPC`
/// the icon for the owner's sex (`00481230`); otherwise, or when that is
/// empty, the form's own icon for `variant` (`0048e730`); when that is empty
/// too the string object `011d33f0` as a `char *`.
pub fn item_change_get_icon_one(e: &mut Engine, this: Ptr<ItemChange>, variant: u32) -> u32 {
    let mut icon = 0u32;
    let owner = item_change_get_item_ownership(e, this);
    let form = container_object(e, this);
    let biped = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_BIPED_MODEL_FORM,
                0u32
            ],
        )
        .u32();
    if owner != 0 && biped != 0 {
        let npc = e
            .call(
                RT_DYNAMIC_CAST,
                &args![owner, 0u32, TYPE_TES_FORM, TYPE_TES_NPC, 0u32],
            )
            .u32();
        if npc != 0 {
            let sex = e.call(ACTOR_BASE_GET_SEX, &args![npc]).u32();
            icon = e.call(BIPED_ICON_FOR_SEX, &args![biped, sex]).u32();
        }
    }
    if icon == 0 || e.mem.u8(icon) == 0 {
        let form = container_object(e, this);
        icon = e.call(FORM_ICON, &args![form, variant]).u32();
    }
    if icon == 0 || e.mem.u8(icon) == 0 {
        return e.call(STRING_OBJECT_TO_CHARS, &args![NO_ICON_STRING]).u32();
    }
    icon
}

// Translated from 004be2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::GetFullName` (Xbox PDB): the item's name. The form cast to
/// `TESFullName`; when it is one and `0048cee0` accepts it, its name
/// (`00408da0`, the map's `MapMarkerData::GetLocationName`), otherwise the
/// string object `011d35b8` as a `char *`.
pub fn item_change_get_full_name(e: &mut Engine, this: Ptr<ItemChange>) -> u32 {
    let form = container_object(e, this);
    let full_name = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_FULL_NAME, 0u32],
        )
        .u32();
    if full_name != 0 && e.call(FULL_NAME_TEST, &args![full_name]).u32() != 0 {
        return e.call(FULL_NAME_GET, &args![full_name]).u32();
    }
    e.call(STRING_OBJECT_TO_CHARS, &args![NO_NAME_STRING]).u32()
}

// Translated from 004be330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::GetFormEnchanting` (Xbox PDB), a `__cdecl` function of
/// a form: the form cast to `TESEnchantableForm` and its enchantment (the word
/// at +4, `00726070`), 0 when the form is not enchantable.
pub fn tes_enchantable_form_get_form_enchanting(e: &mut Engine, form: u32) -> u32 {
    let enchantable = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_ENCHANTABLE_FORM, 0u32],
        )
        .u32();
    if enchantable == 0 {
        return 0;
    }
    e.call(WORD_AT_4, &args![enchantable]).u32()
}

// Translated from 004be380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetFormWeight` (Xbox PDB): the weapon's weight (the
/// `TESWeightForm` sub-object at +0x8c, `006b9130`); with `with_mods` the
/// value of the weight mod effect (type 4) is subtracted.
pub fn tes_object_weap_get_form_weight(e: &mut Engine, this: Ptr, with_mods: u8) -> f32 {
    let weight = e
        .call(GET_WEIGHT, &args![this.addr() + WEAPON_WEIGHT_FORM])
        .f32();
    if with_mods == 0 {
        return weight;
    }
    let bonus = tes_object_weap_get_mod_effect_value(e, this, 4, 0);
    (weight as f64 - bonus as f64) as f32
}

// Translated from 004be3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::SetWorn` (Xbox PDB): with `wear` a new extra data list
/// marked worn (`SetWorn(1, flag)`) is appended to the item's lists (their
/// list node is created when there is none). Without it the first extra list
/// that is worn (`GetWorn(flag)`) is taken out of the list, and deleted when
/// `delete_list` is set. The C++ exception frame is not translated.
pub fn item_change_set_worn(
    e: &mut Engine,
    this: Ptr<ItemChange>,
    wear: u8,
    flag: u8,
    delete_list: u8,
) {
    if wear != 0 {
        let extra = new_extra_list(e);
        e.call(EXTRA_SET_WORN, &args![extra, 1u32, flag as u32]);
        if list_head(e, this) == 0 {
            let node = new_list_node(e);
            e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
        }
        let list = list_head(e, this);
        list_call_with_item(e, LIST_ADD, list, extra);
        return;
    }
    if !item_change_get_worn(e, this, flag) {
        return;
    }
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            return;
        }
        if e.call(EXTRA_GET_WORN, &args![extra, flag as u32]).bool() {
            list_call_with_item(e, LIST_REMOVE, cursor, extra);
            if delete_list != 0 {
                delete_object(e, extra);
            }
            return;
        }
        cursor = list_next(e, cursor);
    }
}

// Translated from 004be580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x240 of the object (a getter, called by `FilterFunc`).
pub fn fn_004be580(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x240)
}

// Translated from 004be5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange` save size (the 16-bit size of `SaveGame`): 6 more bytes when
/// save game blocks are used, 12 for the form id, the count and the list
/// count, and the save size (`00422c40`) of every extra list. With the size
/// logging flag set the size is reported through `Error` (the message names
/// the form being saved when there is one).
pub fn fn_004be5a0(e: &mut Engine, this: Ptr<ItemChange>) -> u16 {
    let save = save_load_game(e);
    let mut size: u16 = 0;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(8);
    size = size.wrapping_add(4);
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        if e.call(LIST_IS_EMPTY, &args![cursor]).bool() {
            break;
        }
        let extra = list_item(e, cursor);
        let extra_size = e.call(EXTRA_GET_SAVE_SIZE, &args![extra, 0u32, 0u32]).u16();
        size = size.wrapping_add(extra_size);
        cursor = list_next(e, cursor);
    }
    if save_size_logging(e) {
        let current = e.call(CURRENT_SAVE_FORM, &args![save]).u32();
        // "GetSaveSize(): %-5i for form %08X %s with flags %08X ending at
        // line %i in file %s", or without the form part.
        log_save_size(e, size as u32, current, 0x622, (0x0101_2cb0, 0x0101_2c78));
    }
    size
}

// Translated from 004be6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange` save (to the save/load game object): with save game blocks
/// the block tag `BLOK` and a 16-bit length (patched at the end), then the
/// item form's numeric id, the count, the number of extra lists (patched
/// after the lists) and the lists (`ExtraDataList::SaveGame(0, 0)`). With the
/// logging flag set the bytes written are reported through `Error`; a block
/// over 0xFFFF bytes is reported through `005b5e40`.
pub fn fn_004be6f0(e: &mut Engine, this: Ptr<ItemChange>) {
    let save = save_load_game(e);
    let start = e.call(SAVE_POSITION, &args![save]).u32();
    let start = if save_size_logging(e) {
        e.call(SAVE_POSITION, &args![save]).u32()
    } else {
        start
    };
    // Locals of the game's frame passed by address: the tag, the form id,
    // the block length and the list count.
    e.with_stack(0x10, |e, locals| {
        let (tag, form_id_slot, length_slot, count_slot) = (
            locals.addr(),
            locals.addr() + 4,
            locals.addr() + 8,
            locals.addr() + 12,
        );
        let mut block = 0u32;
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            e.mem.set_u32(tag, 0x424c_4f4b);
            e.call(SAVE_BYTES, &args![save, tag, 4u32]);
            block = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(SAVE_BYTES, &args![save, length_slot, 2u32]);
        }
        let form = container_object(e, this);
        let form_id = e.call(FORM_ID, &args![form]).u32();
        e.mem.set_u32(form_id_slot, form_id);
        e.call(SAVE_NUMERIC_ID, &args![save, form_id_slot, 4u32]);
        let number = this.addr() + 4;
        e.call(SAVE_BYTES, &args![save, number, 4u32]);
        e.mem.set_u32(count_slot, 0);
        let count_position = e.call(SAVE_POSITION, &args![save]).u32();
        e.call(SAVE_BYTES, &args![save, count_slot, 4u32]);
        let mut cursor = list_head(e, this);
        while cursor != 0 {
            if e.call(LIST_IS_EMPTY, &args![cursor]).bool() {
                break;
            }
            let extra = list_item(e, cursor);
            e.call(EXTRA_SAVE_GAME, &args![extra, 0u32, 0u32]);
            let count = e.mem.u32(count_slot);
            e.mem.set_u32(count_slot, count.wrapping_add(1));
            cursor = list_next(e, cursor);
        }
        let count = e.mem.u32(count_slot);
        e.mem.set_u32(count_position, count);
        if save_size_logging(e) {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            let current = e.call(CURRENT_SAVE_FORM, &args![save]).u32();
            // "SaveGame(): %-5i for form %08X %s with flags %08X ending at
            // line %i in file %s", or without the form part.
            log_save_size(
                e,
                end.wrapping_sub(start),
                current,
                0x641,
                (0x0101_53a0, 0x0101_536c),
            );
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            if end > block.wrapping_add(0xffff) {
                e.call(
                    SAVE_LOAD_LOG,
                    &args![0x0101_5318u32, SOURCE_FILE_NAME, 0x641u32],
                );
            }
            e.mem.set_u16(block, end.wrapping_sub(block) as u16);
        }
    });
}

// Translated from 004be930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::LoadGame` (Xbox PDB): reads what `fn_004be6f0` wrote. With
/// save game blocks the `BLOK` tag is checked (a wrong tag is logged through
/// `005b5e40`) and the 16-bit length read; then the numeric id (looked up
/// and cast from `TESForm` to `TESBoundObject` into `pContainerObj`), the
/// count, and the extra lists, each in a new `ExtraDataList` loaded with
/// `LoadGame(0, 0, 0)` and appended. The block length is checked against the
/// bytes read (overrun and underrun are logged). The C++ exception frame is
/// not translated.
pub fn item_change_load_game(e: &mut Engine, this: Ptr<ItemChange>) {
    let save = save_load_game(e);
    let mut length = 0u16;
    let mut block = 0u32;
    // 0x10-byte frame of the locals passed by address: tag, form id,
    // length, number of lists.
    e.with_stack(0x10, |e, locals| {
        let (tag, form_id_slot, length_slot, count_slot) = (
            locals.addr(),
            locals.addr() + 4,
            locals.addr() + 8,
            locals.addr() + 12,
        );
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            e.call(LOAD_BYTES, &args![save, tag, 4u32]);
            if e.mem.u32(tag) != 0x424c_4f4b {
                let current = e.call(CURRENT_LOAD_FORM, &args![save]).u32();
                if current != 0 {
                    let form_id = e.mem.u32(current);
                    let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
                    let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
                    let version = e.mem.u8(current + 9) as u32;
                    let flags = e.mem.u32(current + 5);
                    e.call(
                        SAVE_LOAD_LOG,
                        &args![
                            0x0101_5718u32,
                            SOURCE_FILE_NAME,
                            0x647u32,
                            form_id,
                            name,
                            version,
                            flags
                        ],
                    );
                } else {
                    let version = e.call(LOAD_VERSION, &args![save]).u8() as u32;
                    e.call(
                        SAVE_LOAD_LOG,
                        &args![0x0101_56a8u32, SOURCE_FILE_NAME, 0x647u32, version],
                    );
                }
            }
            block = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(LOAD_BYTES, &args![save, length_slot, 2u32]);
            length = e.mem.u16(length_slot);
        }
        e.call(LOAD_NUMERIC_ID, &args![save, form_id_slot, 4u32]);
        let number = this.addr() + 4;
        e.call(LOAD_BYTES, &args![save, number, 4u32]);
        let form_id = e.mem.u32(form_id_slot);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0u32],
            )
            .u32();
        e.set(this, ItemChange::pContainerObj, Ptr::new(cast));
        e.set(this, ItemChange::pExtraObjectList, Ptr::NULL);
        e.call(LOAD_BYTES, &args![save, count_slot, 4u32]);
        let lists = e.mem.i32(count_slot);
        if lists != 0 {
            let node = new_list_node(e);
            e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
            let mut index = 0;
            while index < lists {
                let extra = new_extra_list(e);
                e.call(EXTRA_LOAD_GAME, &args![extra, 0u32, 0u32, 0u32]);
                let list = list_head(e, this);
                list_call_with_item(e, LIST_ADD_TAIL, list, extra);
                index += 1;
            }
        }
    });
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
        let position = e.call(SAVE_POSITION, &args![save]).u32();
        let current = e.call(CURRENT_LOAD_FORM, &args![save]).u32();
        let expected = (length as u32).wrapping_add(block);
        // The messages: "SAVELOAD: LoadGame Buffer overrun/underrun of %i
        // bytes in file %s on line %i.  Current version is %i", or with the
        // form being loaded.
        let (overrun, underrun) = if current != 0 {
            (0x0101_5588u32, 0x0101_5500u32)
        } else {
            (0x0101_54a0u32, 0x0101_5440u32)
        };
        let difference = if position > expected {
            Some((overrun, position - expected))
        } else if position < expected {
            Some((underrun, expected - position))
        } else {
            None
        };
        if current != 0 {
            let form_id = e.mem.u32(current);
            let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
            if let Some((format, bytes)) = difference {
                let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
                let version = e.mem.u8(current + 9) as u32;
                let flags = e.mem.u32(current + 5);
                e.call(
                    SAVE_LOAD_LOG,
                    &args![
                        format,
                        bytes,
                        SOURCE_FILE_NAME,
                        0x66bu32,
                        form_id,
                        name,
                        version,
                        flags
                    ],
                );
            }
        } else if let Some((format, bytes)) = difference {
            let version = e.call(LOAD_VERSION, &args![save]).u8() as u32;
            e.call(
                SAVE_LOAD_LOG,
                &args![format, bytes, SOURCE_FILE_NAME, 0x66bu32, version],
            );
        }
    }
}

// Translated from 004bed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00426020(0, 0, 0, form)` on every extra data list of the item
/// (after loading): the walk stops at an empty head node.
pub fn fn_004bed00(e: &mut Engine, this: Ptr<ItemChange>) {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        if e.call(LIST_IS_EMPTY, &args![cursor]).bool() {
            break;
        }
        let extra = list_item(e, cursor);
        if extra != 0 {
            let form = container_object(e, this);
            e.call(EXTRA_AFTER_LOAD_GAME, &args![extra, 0u32, 0u32, 0u32, form]);
        }
        cursor = list_next(e, cursor);
    }
}

// Translated from 004bed60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::SaveGame` (Xbox PDB), the buffer version: the form id, the
/// count (4 bytes), then a variable sized value holding the number of extra
/// lists written (`ExtraDataList::SaveGame_ov2(buffer)` for each non-null
/// one).
pub fn item_change_save_game(e: &mut Engine, this: Ptr<ItemChange>, buffer: u32) {
    let form = container_object(e, this);
    e.call(SAVE_BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
    let number = this.addr() + 4;
    e.call(SAVE_BUFFER_SAVE_DATA, &args![buffer, number, 4u32, 0u32]);
    let mut written = 0u32;
    let start = e.call(SAVE_BUFFER_START_SIZED, &args![buffer]).u32();
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra != 0 {
            e.call(EXTRA_SAVE_GAME_BUFFER, &args![extra, buffer]);
            written += 1;
        }
        cursor = list_next(e, cursor);
    }
    e.call(SAVE_BUFFER_END_SIZED, &args![buffer, written, start]);
}

// Translated from 004bee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::LoadGame_ov2` (Xbox PDB), the buffer version: the form
/// (loaded id, looked up, cast from `TESForm` to `TESBoundObject`), the count
/// (4 bytes), the old extra lists deleted (`DeleteAllExtra`), the number of
/// lists as a variable sized value and each list loaded in a new
/// `ExtraDataList` (`LoadGame_ov2(buffer)`) and appended. The C++ exception
/// frame is not translated.
pub fn item_change_load_game_ov2(e: &mut Engine, this: Ptr<ItemChange>, buffer: u32) {
    let form_id = e.call(LOAD_BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0u32],
        )
        .u32();
    e.set(this, ItemChange::pContainerObj, Ptr::new(cast));
    let number = this.addr() + 4;
    e.call(LOAD_BUFFER_LOAD_DATA, &args![buffer, number, 4u32]);
    item_change_delete_all_extra(e, this);
    let lists = e.call(LOAD_BUFFER_LOAD_SIZED, &args![buffer]).u32();
    if lists != 0 {
        if list_head(e, this) == 0 {
            let node = new_list_node(e);
            e.set(this, ItemChange::pExtraObjectList, Ptr::new(node));
        }
        let mut index = 0u32;
        while index < lists {
            let extra = new_extra_list(e);
            e.call(EXTRA_LOAD_GAME_BUFFER, &args![extra, buffer]);
            let list = list_head(e, this);
            list_call_with_item(e, LIST_ADD_TAIL, list, extra);
            index += 1;
        }
    }
}

// Translated from 004bef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0042ceb0(argument, form)` on every non-null extra data list of the
/// item (the finishing step after loading with the buffer).
pub fn fn_004bef60(e: &mut Engine, this: Ptr<ItemChange>, argument: u32) {
    let mut cursor = list_head(e, this);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra != 0 {
            let form = container_object(e, this);
            e.call(EXTRA_AFTER_LOAD_GAME_BUFFER, &args![extra, argument, form]);
        }
        cursor = list_next(e, cursor);
    }
}

// Translated from 004befb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::InventoryChanges` (Xbox PDB): stores the owner
/// reference, makes an empty list of changes, creates the shared scratch
/// `TESObjectREFR` (`pTempRef`, Xbox PDB; made temporary with
/// `TESForm::SetTemporary`) the first time, resets the weights to `-1.0`
/// (`fn_004bf0e0`) and clears the count dirty flag. The C++ exception frame
/// is not translated.
pub fn inventory_changes_inventory_changes(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    owner: Ptr,
) -> Ptr<InventoryChanges> {
    with_scope_guard(e, 0x6de, |e| {
        e.set(this, InventoryChanges::pRef, owner);
        let node = new_list_node(e);
        e.set(this, InventoryChanges::pListofChanges, Ptr::new(node));
        if e.global::<u32>(TEMP_REF_GLOBAL) == 0 {
            let memory = e.call(OPERATOR_NEW, &args![0x68u32]).u32();
            let temp_ref = if memory == 0 {
                0
            } else {
                e.call(REFR_CONSTRUCT, &args![memory]).u32()
            };
            e.set_global(TEMP_REF_GLOBAL, temp_ref);
            let temp_ref = e.global::<u32>(TEMP_REF_GLOBAL);
            e.call(FORM_SET_TEMPORARY, &args![temp_ref]);
        }
        let none = e.global::<f32>(MINUS_ONE_FLOAT);
        e.set(this, InventoryChanges::fcontainerweight, none);
        let weight = e.get(this, InventoryChanges::fcontainerweight);
        e.set(this, InventoryChanges::fpreviousContainerWeight, weight);
        fn_004bf0e0(e, this);
        e.set(this, InventoryChanges::bcountdirty, false);
    });
    this
}

// Translated from 004bf0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets the cached container weight: the previous weight becomes the
/// current one, the current one `-1.0`, and when the owner reference's
/// virtual 0x100 says so the owner is told (virtual 0x30c with `-1.0`).
pub fn fn_004bf0e0(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let weight = e.get(this, InventoryChanges::fcontainerweight);
    e.set(this, InventoryChanges::fpreviousContainerWeight, weight);
    let none = e.global::<f32>(MINUS_ONE_FLOAT);
    e.set(this, InventoryChanges::fcontainerweight, none);
    let owner = e.get(this, InventoryChanges::pRef).addr();
    if owner != 0 && e.vcall(owner, 0x100, &args![]).bool() {
        let owner = e.get(this, InventoryChanges::pRef).addr();
        let none = e.global::<f32>(MINUS_ONE_FLOAT);
        e.vcall(owner, 0x30c, &args![none]);
    }
}

// Translated from 004bf150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of the `InventoryChanges` destructor: every `ItemChange` of the
/// list gets `DeleteAllExtra` and is deleted (the next node is taken first),
/// the list is cleared, the owner is told (virtual 0x4c with `0x8000020`) and
/// the list deleted.
pub fn fn_004bf150(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let mut cursor = e.get(this, InventoryChanges::pListofChanges).addr();
    while cursor != 0 {
        let item = list_item(e, cursor);
        if item == 0 {
            break;
        }
        cursor = list_next(e, cursor);
        item_change_delete_all_extra(e, Ptr::new(item));
        delete_item_change(e, item);
    }
    let list = e.get(this, InventoryChanges::pListofChanges).addr();
    e.call(LIST_CLEAR, &args![list]);
    let owner = e.get(this, InventoryChanges::pRef).addr();
    if owner != 0 {
        e.vcall(owner, 0x4c, &args![0x0800_0020u32]);
    }
    let list = e.get(this, InventoryChanges::pListofChanges).addr();
    if list != 0 {
        e.call(LIST_DESTROY, &args![list, 1u32]);
    }
}

// Translated from 004bf220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), a `__cdecl` function of
/// a reference: the `InventoryChanges` kept in the reference's extra data
/// (`GetContainerChanges` on the list `005d43c0` returns), made with the
/// constructor and stored (`SetContainerChanges`) when there is none. Nothing
/// is made (0 returned) when virtual 0xfc is true and virtual 0x100 false.
/// The C++ exception frame is not translated.
pub fn inventory_changes_get_inventory_changes(e: &mut Engine, reference: u32) -> u32 {
    if e.vcall(reference, 0xfc, &args![]).bool() && !e.vcall(reference, 0x100, &args![]).bool() {
        return 0;
    }
    let extra_list = e.call(REFR_EXTRA_LIST, &args![reference]).u32();
    let mut changes = e
        .call(EXTRA_GET_CONTAINER_CHANGES, &args![extra_list])
        .u32();
    if changes == 0 {
        changes = with_scope_guard(e, 0x71d, |e| {
            let memory = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
            let created = if memory == 0 {
                0
            } else {
                inventory_changes_inventory_changes(e, Ptr::new(memory), Ptr::new(reference)).addr()
            };
            let extra_list = e.call(REFR_EXTRA_LIST, &args![reference]).u32();
            e.call(EXTRA_SET_CONTAINER_CHANGES, &args![extra_list, created]);
            created
        });
    }
    changes
}

// Translated from 004bf330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::RemoveHotKeyItem` (Xbox PDB): takes the hot key off the
/// extra lists of the entry for the item's form (`GetObjectInList`): all
/// hot keys with `hot_key == -1`, else the lists with that key. A list left
/// without extras (`fn_004bca60`), or one of a count above 1 that holds one
/// kind of extra, is removed from the entry (the walk then restarts at the
/// head). An entry left without lists is emptied, and when the item's count
/// is zero removed from the changes and deleted.
pub fn inventory_changes_remove_hot_key_item(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    item: Ptr<ItemChange>,
    hot_key: i32,
) {
    let form = container_object(e, item);
    let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
    if entry == 0 {
        return;
    }
    let mut cursor = e.mem.u32(entry);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra != 0 {
            let mut matches = false;
            if hot_key != -1 {
                let key = e.call(EXTRA_GET_HOT_KEY, &args![extra]).u8() as i8;
                if key as i32 == hot_key {
                    matches = true;
                }
            }
            if !matches && hot_key == -1 {
                let key = e.call(EXTRA_GET_HOT_KEY, &args![extra]).u8() as i8;
                if key >= 0 {
                    matches = true;
                }
            }
            if matches {
                e.call(EXTRA_REMOVE_HOT_KEY, &args![extra]);
                let emptied = fn_004bca60(e, Ptr::new(extra));
                let single_kind = !emptied && {
                    let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
                    count > 1 && e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).i32() == 1
                };
                if emptied || single_kind {
                    let list = e.mem.u32(entry);
                    list_call_with_item(e, LIST_REMOVE, list, extra);
                    cursor = e.mem.u32(entry);
                } else {
                    cursor = list_next(e, cursor);
                }
            } else {
                cursor = list_next(e, cursor);
            }
        } else {
            cursor = list_next(e, cursor);
        }
    }
    let list = e.mem.u32(entry);
    if list != 0 && e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let list = e.mem.u32(entry);
        if list != 0 {
            e.call(LIST_DESTROY, &args![list, 1u32]);
        }
        e.mem.set_u32(entry, 0);
        if e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber) == 0 {
            let changes = e.get(this, InventoryChanges::pListofChanges).addr();
            list_call_with_item(e, LIST_REMOVE, changes, entry);
            delete_item_change(e, entry);
        }
    }
}

// Translated from 004bf4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ItemChange` (a new copy made by the fast inventory iteration) that
/// has an extra list with the hot key `hot_key`, 0 when there is none. The
/// iteration (`StartOEIFastInventoryIteration`/`GetNextOEIFastInventoryItem`)
/// hands out new items: the ones without a match are deleted, the iterator
/// too. The C++ exception frame is not translated.
pub fn fn_004bf4b0(e: &mut Engine, this: Ptr<InventoryChanges>, hot_key: i32) -> u32 {
    with_scope_guard(e, 0x76a, |e| {
        let iterator = e.call(START_FAST_ITERATION, &args![this]).u32();
        loop {
            let item = e.call(GET_NEXT_FAST_ITEM, &args![this, iterator]).u32();
            if item != 0 {
                let mut cursor = e.call(WORD_AT_0, &args![item]).u32();
                while cursor != 0 {
                    let extra = list_item(e, cursor);
                    if extra == 0 {
                        break;
                    }
                    let key = e.call(EXTRA_GET_HOT_KEY, &args![extra]).u8() as i8;
                    if key as i32 == hot_key {
                        if iterator != 0 {
                            oei_fast_inventory_iterator_scalar_deleting_destructor(e, iterator, 1);
                        }
                        return item;
                    }
                    cursor = list_next(e, cursor);
                }
                delete_item_change(e, item);
            }
            if item == 0 {
                break;
            }
        }
        if iterator != 0 {
            oei_fast_inventory_iterator_scalar_deleting_destructor(e, iterator, 1);
        }
        0
    })
}

// Translated from 004bf630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OEI_Fast_InventoryIterator::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body (`004ca1a0`) and frees the iterator when bit 0 of
/// `flags` is set; returns `this`.
pub fn oei_fast_inventory_iterator_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(FAST_ITERATOR_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004bf660 (decompiled, FalloutNV.exe 1.4.0.525)
/// A new array of `high - low + 1` `ItemChange` pointers (zeroed) indexed by
/// hot key minus `low`: for every item of the fast inventory iteration the
/// extra lists with a hot key in `low..=high` put the item in their slot
/// (deleting an item already there). Items that were not used are deleted,
/// the iterator too. 0 when `low > high`.
pub fn fn_004bf660(e: &mut Engine, this: Ptr<InventoryChanges>, low: i32, high: i32) -> u32 {
    if low > high {
        return 0;
    }
    let count = (high.wrapping_sub(low).wrapping_add(1)) as u32;
    // `count * 4`, the allocation size saturating on overflow.
    let size = count.saturating_mul(4);
    let array = e.call(OPERATOR_NEW, &args![size]).u32();
    e.call(MEMSET, &args![array, 0u32, count << 2]);
    let iterator = e.call(START_FAST_ITERATION, &args![this]).u32();
    loop {
        let item = e.call(GET_NEXT_FAST_ITEM, &args![this, iterator]).u32();
        if item != 0 {
            let mut used = false;
            let mut cursor = e.call(WORD_AT_0, &args![item]).u32();
            while cursor != 0 {
                let extra = list_item(e, cursor);
                if extra == 0 {
                    break;
                }
                let key = e.call(EXTRA_GET_HOT_KEY, &args![extra]).u8() as i8 as i32;
                if key >= low && key <= high {
                    let slot = array.wrapping_add(((key - low) as u32).wrapping_mul(4));
                    let previous = e.mem.u32(slot);
                    delete_item_change(e, previous);
                    e.mem.set_u32(slot, item);
                    used = true;
                }
                cursor = list_next(e, cursor);
            }
            if !used {
                delete_item_change(e, item);
            }
        }
        if item == 0 {
            break;
        }
    }
    if iterator != 0 {
        oei_fast_inventory_iterator_scalar_deleting_destructor(e, iterator, 1);
    }
    array
}

// Translated from 004bf800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::SetHotKeyItem` (Xbox PDB): gives the hot key
/// `hot_key` (0 to 7) to `extra` of `item`. Without such arguments nothing is
/// done. The player's hot key slot (`004bfb30`) is cleared (`004bfb70(key,
/// 0)`) when set, the item's other hot keys removed
/// (`RemoveHotKeyItem(item, -1)`). When the changes have an entry for the
/// item's form the key goes to `extra` if the entry has it, else to the
/// default list of the entry, else to a new list holding the item count not
/// yet in lists (`iNumber - extra count - 1 if worn`); with no entry a new
/// `ItemChange` with a new list holding the key is added. The C++ exception
/// frame is not translated.
pub fn inventory_changes_set_hot_key_item(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    item: Ptr<ItemChange>,
    extra: u32,
    hot_key: i32,
) {
    with_scope_guard(e, 0x7b6, |e| {
        if item.is_null() || !(0..=7).contains(&hot_key) {
            return;
        }
        let player = e.global::<u32>(PLAYER_GLOBAL);
        if fn_004bfb30(e, player, hot_key) != 0 {
            fn_004bfb70(e, player, hot_key, 0);
        }
        inventory_changes_remove_hot_key_item(e, this, item, -1);
        let form = container_object(e, item);
        let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        if entry != 0 {
            let mut chosen = 0u32;
            ensure_extra_list(e, entry);
            let mut cursor = e.mem.u32(entry);
            while cursor != 0 {
                let candidate = list_item(e, cursor);
                if candidate == 0 {
                    break;
                }
                if chosen == 0 {
                    let candidate = list_item(e, cursor);
                    if e.call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![candidate, 0u32])
                        .bool()
                    {
                        chosen = list_item(e, cursor);
                    }
                }
                if list_item(e, cursor) == extra {
                    chosen = extra;
                }
                cursor = list_next(e, cursor);
            }
            if chosen == 0 {
                let new_list = new_extra_list(e);
                e.call(EXTRA_SET_HOT_KEY, &args![new_list, hot_key as u8 as u32]);
                let number = e.get(item, ItemChange::iNumber);
                let counted = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                let worn = item_change_get_worn(e, Ptr::new(entry), 0);
                let remaining = number.wrapping_sub(counted).wrapping_sub(worn as i32);
                e.call(EXTRA_SET_COUNT, &args![new_list, remaining]);
                let list = e.mem.u32(entry);
                list_call_with_item(e, LIST_ADD, list, new_list);
            } else {
                e.call(EXTRA_SET_HOT_KEY, &args![chosen, hot_key as u8 as u32]);
            }
        } else {
            let number = e.get(item, ItemChange::iNumber);
            let change = new_item_change(e, form, number as u32);
            let new_list = new_extra_list(e);
            // The constructor made a list node; the game replaces it.
            let node = new_list_node(e);
            e.mem.set_u32(change, node);
            e.call(EXTRA_SET_HOT_KEY, &args![new_list, hot_key as u8 as u32]);
            let change_list = e.mem.u32(change);
            list_call_with_item(e, LIST_ADD, change_list, new_list);
            let changes = e.get(this, InventoryChanges::pListofChanges).addr();
            list_call_with_item(e, LIST_ADD, changes, change);
        }
    });
}

// Translated from 004bfb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's hot key table entry for `index` (0 to 8; 0 outside): reads
/// the entry `006a7ad0` finds in the map at +0xe3c.
pub fn fn_004bfb30(e: &mut Engine, this: u32, index: i32) -> u32 {
    if !(0..=8).contains(&index) {
        return 0;
    }
    let entry = e
        .call(HOT_KEY_TABLE_ENTRY, &args![this + HOT_KEY_TABLE, index])
        .u32();
    e.mem.u32(entry)
}

// Translated from 004bfb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the player's hot key table entry for `index` to `value`
/// (`0047a110(index, &value)` on the map at +0xe3c).
pub fn fn_004bfb70(e: &mut Engine, this: u32, index: i32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(HOT_KEY_TABLE_SET, &args![this + HOT_KEY_TABLE, index, slot]);
    });
}

// Translated from 004bfba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetObjectInList` (Xbox PDB): the `ItemChange` of the
/// list of changes whose form is `form`. With a non-zero `reference_id` only
/// one that has an extra list whose reference pointer
/// (`GetReferencePointer`) has that form id; 0 when there is none. The
/// second stack word is not read.
pub fn inventory_changes_get_object_in_list(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    _unused_1: u32,
    reference_id: u32,
) -> u32 {
    let mut cursor = e.get(this, InventoryChanges::pListofChanges).addr();
    let mut searching = true;
    while cursor != 0 && searching {
        let item = list_item(e, cursor);
        if item != 0
            && e.get(Ptr::<ItemChange>::new(item), ItemChange::pContainerObj)
                .addr()
                == form
        {
            searching = false;
        } else {
            cursor = list_next(e, cursor);
        }
    }
    if cursor == 0 {
        return 0;
    }
    let item = list_item(e, cursor);
    if item == 0 || reference_id == 0 {
        return item;
    }
    let mut inner = e.mem.u32(item);
    while inner != 0 {
        let extra = list_item(e, inner);
        if extra != 0 && e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32() != 0 {
            let reference = e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32();
            if e.call(FORM_ID, &args![reference]).u32() == reference_id {
                return item;
            }
        }
        inner = list_next(e, inner);
    }
    0
}

// Translated from 004bfc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an armor form (type 0x18) of the changes has a worn extra list
/// (`GetWorn(0)`) while its `TESObjectARMO` flag bit 0 (`fn_004bfd80`) is
/// set. The walk of the changes stops at the first one that is null.
pub fn fn_004bfc80(e: &mut Engine, this: Ptr<InventoryChanges>) -> bool {
    let mut found = false;
    let mut cursor = e.get(this, InventoryChanges::pListofChanges).addr();
    while cursor != 0 {
        let entry = list_item(e, cursor);
        if entry == 0 {
            break;
        }
        let form = container_object(e, Ptr::new(entry));
        if form_type_of(e, form) == 0x18 && !found {
            let armor = container_object(e, Ptr::new(entry));
            let mut inner = e.mem.u32(entry);
            while inner != 0 {
                let extra = list_item(e, inner);
                if extra == 0 || found {
                    break;
                }
                if e.call(EXTRA_GET_WORN, &args![extra, 0u32]).bool()
                    && fn_004bfd80(e, Ptr::new(armor))
                {
                    found = true;
                }
                inner = list_next(e, inner);
            }
        }
        cursor = list_next(e, cursor);
    }
    found
}

// Translated from 004bfd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OBJ_ARMO::cFlags` (Xbox PDB) bit 0 of a `TESObjectARMO`, at +0x180.
pub fn fn_004bfd80(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + ARMOR_FLAGS) & 1 != 0
}

// Translated from 004bfda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::WearingObject` (Xbox PDB): the worn extra list of the
/// entry for `form` (`GetWorn(0)`); with `wear_unwearable` also one that is
/// marked "can not wear", which then gets `SetWorn(1, 0)`. 0 when there is
/// none.
pub fn inventory_changes_wearing_object(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    wear_unwearable: u8,
) -> u32 {
    let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
    if entry == 0 {
        return 0;
    }
    let mut cursor = e.mem.u32(entry);
    while cursor != 0 {
        let extra = list_item(e, cursor);
        if extra == 0 {
            break;
        }
        if e.call(EXTRA_GET_WORN, &args![extra, 0u32]).bool() {
            return extra;
        }
        if e.call(EXTRA_GET_CAN_NOT_WEAR, &args![extra]).bool() && wear_unwearable != 0 {
            e.call(EXTRA_SET_WORN, &args![extra, 1u32, 0u32]);
            return extra;
        }
        cursor = list_next(e, cursor);
    }
    0
}

// Translated from 004bfe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::RemoveAllObjectsWorn` (Xbox PDB): for every change that
/// is worn (`ItemChange::GetWorn(0)`) takes the item out with `004c0cf0`,
/// passing the count and extra list of its first worn extra list (1 and 0
/// without one); with a `reference` (the actor) the form is first passed to
/// `008248e0` on its sub-object at +0x94. When `004c0cf0` reports a removal
/// the walk restarts at the head. The first stack word is not read.
pub fn inventory_changes_remove_all_objects_worn(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    _unused_1: u32,
    reference: u32,
) {
    e.with_stack(4, |e, removed| {
        let mut cursor = e.get(this, InventoryChanges::pListofChanges).addr();
        while cursor != 0 {
            let entry = list_item(e, cursor);
            if entry == 0 {
                cursor = list_next(e, cursor);
                continue;
            }
            e.mem.set_u8(removed.addr(), 0);
            if !item_change_get_worn(e, Ptr::new(entry), 0) {
                cursor = list_next(e, cursor);
                continue;
            }
            let mut count = 1i32;
            let mut worn_extra = 0u32;
            let mut inner = e.mem.u32(entry);
            while inner != 0 {
                let extra = list_item(e, inner);
                if extra == 0 {
                    break;
                }
                if e.call(EXTRA_GET_WORN, &args![extra, 0u32]).bool() {
                    count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                    worn_extra = list_item(e, inner);
                    break;
                }
                inner = list_next(e, inner);
            }
            let form = container_object(e, Ptr::new(entry));
            if reference != 0 {
                e.call(REFERENCE_FORM_NOTIFY, &args![reference + 0x94, form, 1u32]);
            }
            e.call(
                INVENTORY_REMOVE_ITEM,
                &args![this, removed, form, count, reference, worn_extra, 0u32, 1u32, entry, 0u32],
            );
            if e.mem.u8(removed.addr()) != 0 {
                cursor = e.get(this, InventoryChanges::pListofChanges).addr();
            } else {
                cursor = list_next(e, cursor);
            }
        }
    });
}

// Translated from 004bffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container of the owner reference (`TESObjectREFR::HasContainer`, the
/// map's name for `0055d310`, which returns it), 0 without an owner.
pub fn fn_004bffb0(e: &mut Engine, this: Ptr<InventoryChanges>) -> u32 {
    let owner = e.get(this, InventoryChanges::pRef).addr();
    if owner == 0 {
        return 0;
    }
    e.call(REFR_HAS_CONTAINER, &args![owner]).u32()
}

/// Splits one unit off the extra list `source` of an equipped stack:
/// `new ExtraDataList` copied from `source` (`CopyList`) with count 1, the
/// count of `source` reduced by one and its hot key removed. Returns the new
/// list.
fn split_extra_list(e: &mut Engine, source: u32) -> u32 {
    let created = new_extra_list(e);
    e.call(EXTRA_COPY_LIST, &args![created, source]);
    e.call(EXTRA_SET_COUNT, &args![created, 1u32]);
    let count = e.call(EXTRA_GET_COUNT, &args![source]).u16() as i16 as i32;
    e.call(EXTRA_SET_COUNT, &args![source, count - 1]);
    let key = e.call(EXTRA_GET_HOT_KEY, &args![source]).u8() as i8;
    if key >= 0 {
        e.call(EXTRA_REMOVE_HOT_KEY, &args![source]);
    }
    created
}

/// Whether `form` is a weapon (type 0x28) of the kinds `fn_004c0bf0` accepts.
fn is_special_weapon(e: &mut Engine, form: u32) -> bool {
    form_type_of(e, form) == 0x28 && fn_004c0bf0(e, Ptr::new(form))
}

// Translated from 004bffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts `count` of `form` on `actor` as worn (the equip step of the
/// inventory): `extra` is the extra list to wear (0 for any), `worn_flag` is
/// the `SetWorn` argument and `cannot_wear` the `SetCanNotWear` one.
///
/// The cache of the weight is reset, the owner told (virtual 0x48 with 0x20)
/// and the entry of the changes for `form` found (or made, with an empty
/// list, when there is none or the item is not in the changes yet). The
/// number of items not in extra lists is `count in the container + entry
/// number - the extra lists' total`. A list of the entry that is the given
/// one, or the first with a free count when none was given, is the one worn:
/// when its count is above 1 and the form is neither type 0x29 nor a special
/// weapon one unit is split off (`CopyList`) and worn, otherwise the whole
/// list. A list that can not be worn is not used; a new list is made when no
/// list could be used. With an entry already worn nothing is done.
///
/// For an `actor` with a current process the equip is announced to the
/// actor by form type: 0x18 (armor) tells the process (virtual 0x6e4 of its
/// acquire object) for power armor kinds and sets the biped slot
/// (virtual 0x468); 0x1a only the latter; 0x28 (weapon) and 0x29 (ammo)
/// keep an `ItemChange` in the process (virtual 0x148/0x14c, filled with the
/// worn list, or a new one accepted through virtual 0x160/0x168), the weapon
/// also running the entry point 0x2b and updating the weapon model. At the end
/// the actor's lighting is refreshed. The C++ exception frame is not
/// translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_004bffe0(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    count: i32,
    actor: u32,
    mut extra: u32,
    worn_flag: u8,
    cannot_wear: u8,
) {
    with_scope_guard(e, 0x960, |e| {
        fn_004bf0e0(e, this);
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 {
            e.vcall(owner, 0x48, &args![0x20u32]);
            let kind = form_type_of(e, form);
            if kind == 0x1a || kind == 0x18 {
                e.call(GET_FORM_AS_BIPED_MODEL, &args![form]);
            }
        }
        let mut entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        let container = fn_004bffb0(e, this);
        let in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
        let mut total = in_container;
        if entry != 0 {
            total += e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
        }
        let mut in_lists = 0i32;
        if entry != 0 && e.mem.u32(entry) != 0 {
            let counted = item_change_get_extra_total_count(e, Ptr::new(entry), false);
            in_lists = counted + item_change_get_extra_total_default_count(e, Ptr::new(entry));
        }
        let mut free = total - in_lists;
        if entry != 0 && e.mem.u32(entry) != 0 {
            let head = e.mem.u32(entry);
            if list_item(e, head) != 0 {
                let first = list_item(e, head);
                if e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool() {
                    free = in_container - in_lists;
                }
            }
            let mut matched = false;
            if extra != 0 {
                let head = e.mem.u32(entry);
                let held = with_item_slot(e, extra, |e, slot| {
                    e.call(LIST_CONTAINS, &args![head, slot]).bool()
                });
                if !held {
                    let mut cursor = e.mem.u32(entry);
                    while cursor != 0 {
                        let candidate = list_item(e, cursor);
                        if candidate == 0 {
                            break;
                        }
                        if !e.call(EXTRA_COMPARE_LIST, &args![candidate, extra]).bool() {
                            extra = candidate;
                            matched = true;
                            break;
                        }
                        cursor = list_next(e, cursor);
                    }
                    if !matched {
                        extra = 0;
                    }
                }
            }
        }
        let mut needs_new = true;
        let mut used = 0u32;
        if (extra == 0 && free > 0) || entry == 0 {
            if entry == 0 {
                entry = new_item_change(e, form, 0);
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_ADD_TAIL, list, entry);
                extra = 0;
            }
        } else {
            if item_change_get_worn(e, Ptr::new(entry), 0) {
                return;
            }
            let mut cursor = e.mem.u32(entry);
            while cursor != 0 && needs_new {
                used = list_item(e, cursor);
                if extra != 0 && extra == used {
                    let special = is_special_weapon(e, form);
                    let count_here = e.call(EXTRA_GET_COUNT, &args![used]).u16() as i16;
                    if count_here > 1 && form_type_of(e, form) != 0x29 && !special {
                        extra = split_extra_list(e, used);
                    } else {
                        e.call(EXTRA_SET_WORN, &args![used, 1u32, worn_flag as u32]);
                        let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
                        e.call(EXTRA_SET_COUNT, &args![used, number]);
                    }
                    needs_new = false;
                } else if used != 0 && extra == 0 {
                    if e.call(EXTRA_GET_COUNT, &args![used]).u16() as i16 == 1 {
                        extra = used;
                    } else {
                        let special = is_special_weapon(e, form);
                        let count_here = e.call(EXTRA_GET_COUNT, &args![used]).u16() as i16;
                        if count_here > 1 && form_type_of(e, form) != 0x29 && !special {
                            extra = split_extra_list(e, used);
                        } else {
                            extra = used;
                        }
                    }
                    needs_new = false;
                } else {
                    cursor = list_next(e, cursor);
                }
            }
        }
        if extra != 0 && !e.call(EXTRA_GET_CAN_NOT_WEAR, &args![extra]).bool() {
            e.call(EXTRA_SET_WORN, &args![extra, 1u32, worn_flag as u32]);
            e.call(EXTRA_SET_COUNT, &args![extra, count as u16 as u32]);
            needs_new = false;
            ensure_extra_list(e, entry);
            let head = e.mem.u32(entry);
            let held = with_item_slot(e, extra, |e, slot| {
                e.call(LIST_CONTAINS, &args![head, slot]).bool()
            });
            if !held {
                list_call_with_item(e, LIST_ADD, head, extra);
            }
            used = extra;
        }
        if needs_new {
            used = new_extra_list(e);
            e.call(EXTRA_SET_WORN, &args![used, 1u32, worn_flag as u32]);
            if count > 1 {
                e.call(EXTRA_SET_COUNT, &args![used, count as u16 as u32]);
            }
            ensure_extra_list(e, entry);
            let head = e.mem.u32(entry);
            let held = with_item_slot(e, used, |e, slot| {
                e.call(LIST_CONTAINS, &args![head, slot]).bool()
            });
            if !held {
                list_call_with_item(e, LIST_ADD, head, used);
            }
        }
        if used != 0 {
            e.call(EXTRA_SET_CAN_NOT_WEAR, &args![used, cannot_wear as u32]);
        }
        if e.mem.u32(actor + ACTOR_CURRENT_PROCESS) != 0 {
            announce_equip(e, form, count, actor, used);
        }
    });
}

/// Calls `body` with the address of a 4-byte slot holding `item` (the
/// `&item` the list methods take).
fn with_item_slot<R>(e: &mut Engine, item: u32, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        body(e, slot.addr())
    })
}

/// The last part of `004bffe0`: tells the actor's process about the worn
/// extra list `worn_list` of `form`, by form type.
fn announce_equip(e: &mut Engine, form: u32, count: i32, actor: u32, worn_list: u32) {
    e.call(GET_FORM_AS_BIPED_MODEL, &args![form]);
    let kind = form_type_of(e, form);
    match kind {
        0x18 => {
            let biped = form + ARMOR_BIPED_MODEL;
            let power_armor = e.call(BIPED_IS_POWER_ARMOR, &args![biped]).bool()
                || fn_004c0bd0(e, Ptr::new(biped))
                || e.call(BIPED_FLAG_FOUR, &args![biped]).bool();
            if power_armor && e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
                let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                e.vcall(acquire, 0x6e4, &args![actor]);
            }
            let process = e.mem.u32(actor + ACTOR_CURRENT_PROCESS);
            e.vcall(process, 0x468, &args![1u32]);
        }
        0x1a => {
            let process = e.mem.u32(actor + ACTOR_CURRENT_PROCESS);
            e.vcall(process, 0x468, &args![1u32]);
        }
        0x28 => {
            let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let mut kept = e.vcall(acquire, 0x148, &args![]).u32();
            if kept != 0 {
                let list = e.mem.u32(kept);
                e.call(LIST_CLEAR, &args![list]);
                let list = e.mem.u32(kept);
                list_call_with_item(e, LIST_ADD, list, worn_list);
                e.mem.set_u32(kept + 8, form);
                e.call(ACTOR_RELOAD_TARGETS, &args![actor, 0u32]);
            } else {
                kept = new_item_change(e, form, count as u32);
                let list = e.mem.u32(kept);
                list_call_with_item(e, LIST_ADD, list, worn_list);
                let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                // 004c082e: slot 0x1d0 takes no stack word; the pushed 0 is the
                // third word of slot 0x160.
                let hand = e.vcall(actor, 0x1d0, &args![]).u32();
                if !e.vcall(acquire, 0x160, &args![kept, hand, 0u32]).bool() {
                    delete_item_change(e, kept);
                    kept = 0;
                } else {
                    let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
                    let item_form = e.mem.u32(kept + 8);
                    let value = e.call(FORM_WEIGHT_FIELD, &args![item_form]).f32();
                    e.with_stack(4, |e, slot| {
                        e.mem.set_f32(slot.addr(), value);
                        let item_form = e.mem.u32(kept + 8);
                        e.call(HANDLE_ENTRY_POINT, &args![0x2bu32, actor, item_form, slot]);
                        let value = e.mem.f32(slot.addr());
                        if animation != 0 {
                            e.call(ANIMATION_STORE_FLOAT, &args![animation, value]);
                        }
                    });
                }
            }
            let player = e.global::<u32>(PLAYER_GLOBAL);
            let process = e.mem.u32(actor + ACTOR_CURRENT_PROCESS);
            let announced = actor != player && e.vcall(process, 0x474, &args![1u32]).bool();
            if !announced && kept != 0 {
                let item_form = e.mem.u32(kept + 8);
                if actor == player {
                    e.call(PLAYER_STORE_FORM, &args![player, item_form]);
                } else if e.call(PREDICATE_008C7AA0, &args![]).bool()
                    && e.call(THREAD_VALUE_0047B200, &args![]).u32() != actor
                {
                    let queue = e.call(TASK_QUEUE_GETTER, &args![]).u32();
                    e.call(TASK_QUEUE_ATTACH_WEAPON, &args![queue, actor, item_form]);
                } else {
                    e.call(REFR_UPDATE_WEAPON, &args![actor, item_form]);
                }
            }
        }
        0x29 => {
            let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let mut kept = e.vcall(acquire, 0x14c, &args![]).u32();
            if kept != 0 {
                let list = e.mem.u32(kept);
                e.call(LIST_CLEAR, &args![list]);
                let list = e.mem.u32(kept);
                list_call_with_item(e, LIST_ADD, list, worn_list);
                e.mem.set_u32(kept + 8, form);
            } else {
                kept = new_item_change(e, form, 0);
                let list = e.mem.u32(kept);
                list_call_with_item(e, LIST_ADD, list, worn_list);
                let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                if !e.vcall(acquire, 0x168, &args![kept]).bool() {
                    delete_item_change(e, kept);
                    kept = 0;
                }
            }
            let process = e.mem.u32(actor + ACTOR_CURRENT_PROCESS);
            if !e.vcall(process, 0x474, &args![1u32]).bool() && kept != 0 {
                let item_form = e.mem.u32(kept + 8);
                e.call(STORE_NOTIFY_004534F0, &args![actor, item_form]);
            }
        }
        _ => {}
    }
    let lighting = e.call(ACTOR_LIGHTING_ARGUMENT, &args![actor]).u32();
    e.call(ACTOR_INIT_LIGHTING, &args![actor, lighting]);
}

// Translated from 004c0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 7 of the flag byte (`BIPED_MODEL::sFlags`, +8 of a
/// `TESBipedModelForm`).
pub fn fn_004c0bd0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 0x80 != 0
}

// Translated from 004c0bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A weapon whose type is in the `004c0c30` range but not in the `004c0c60`
/// one.
pub fn fn_004c0bf0(e: &mut Engine, this: Ptr) -> bool {
    fn_004c0c30(e, this) && !fn_004c0c60(e, this)
}

// Translated from 004c0c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon type (`OBJ_WEAP::eType`, a signed byte at +0xf4 of a
/// `TESObjectWEAP`) is 3 to 13.
pub fn fn_004c0c30(e: &mut Engine, this: Ptr) -> bool {
    (3..=13).contains(&e.mem.i8(this.addr() + WEAPON_TYPE))
}

// Translated from 004c0c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon type (`OBJ_WEAP::eType`, a signed byte at +0xf4 of a
/// `TESObjectWEAP`) is 3 to 9.
pub fn fn_004c0c60(e: &mut Engine, this: Ptr) -> bool {
    (3..=9).contains(&e.mem.i8(this.addr() + WEAPON_TYPE))
}

// Translated from 004c0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at +0x110 of the animation object `this` (the one
/// `PickAnimations` and `StartAttack` pass): `value`, or `0.1f` when `value`
/// is below the double `0.1f` (an unordered `value` is stored as it is).
pub fn fn_004c0c90(e: &mut Engine, this: Ptr, value: f32) {
    let minimum = e.global::<f64>(ANIMATION_MINIMUM_DOUBLE);
    let stored = if (value as f64) < minimum {
        e.global::<f32>(PRICE_STEP)
    } else {
        x87_quiet(value)
    };
    e.mem.set_f32(this.addr() + 0x110, stored);
}

// Translated from 004c0cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0x1f0 of `this` (the player character, when
/// `fn_004c0cf0` and the equip code reset or set it).
pub fn fn_004c0cd0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x1f0, value);
}

/// The check the unequip code runs on the dismemberment data of a reference
/// (`ExtraDataList::GetDismembermentExtra` of its extra data list): true
/// unless one of its entries has a first byte 0 and a second byte nonzero.
fn dismemberment_allows_slot_update(e: &mut Engine, actor: u32) -> bool {
    let mut allowed = true;
    let extra_list = e.call(REFR_EXTRA_LIST, &args![actor]).u32();
    let data = e.call(GET_DISMEMBERMENT_EXTRA, &args![extra_list]).u32();
    if data != 0
        && e.call(WORD_AT_8, &args![data + 0x20]).u32() != 0
        && e.call(ARRAY_ELEMENT, &args![data, 0u32]).u32() != 0
    {
        let mut index = 0u32;
        while index < e.call(WORD_AT_8, &args![data + 0x20]).u32() {
            let entry = e.call(ARRAY_ELEMENT, &args![data, index]).u32();
            if e.mem.u8(entry) == 0 && e.mem.u8(entry + 1) != 0 {
                allowed = false;
            }
            index += 1;
        }
    }
    allowed
}

// Translated from 004c0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The counterpart of `fn_004bffe0` that takes `count` of `form` off
/// `actor` (called by `RemoveAllObjectsWorn`): finds the entry of the
/// changes for `form` (`entry`, or `GetObjectInList(form, 1, 0)` when it is
/// 0; without an entry nothing is done and the result is true) and
/// processes the first of its extra lists that is worn (`GetWorn(worn_flag)`)
/// and, when `extra` is given, is `extra`. `*removed` is set to 1 first and
/// reset to 0 when the item can not be taken off (`GetCanNotWear` and the
/// actor's virtual 0x22c refuses).
///
/// For an actor with a current process the form type decides how the actor
/// is told: 0x18 (armor) sets the power armor flag (the biped model's
/// `IsPowerArmor` or `fn_004c0bd0`) and, when the dismemberment data allows
/// it, calls virtual 0x468 of the process with 1; 0x1a only the latter;
/// 0x28 (weapon) acts when the weapon kept in the process (virtual 0x148)
/// is this extra list or no list was given: resets the player's word at
/// +0x1f0 and either has the task queue detach the weapon (`0087b3b0`) or
/// removes it (`RemoveWeapon`, virtual 0x160 and 0x168); 0x29 (ammunition)
/// does the same through virtual 0x14c and 0x168.
///
/// Then the list is not worn any more (`SetWorn(0, worn_flag)`) and its
/// count is set (0 with a hot key, else the entry's number minus the lists'
/// total). A list that `fn_004bca60` says is a plain one is unlinked and
/// deleted (and `*matched` set when it was `extra`); the others are merged
/// into the first list of the entry they are equal to
/// (`CompareListForContainer` either way): the count is added, the hot key
/// moved, the list unlinked and deleted, and the merged-into list remembered
/// in the global `LAST_MERGED_EXTRA_LIST`.
///
/// The tail tells a power armor actor's process (virtual 0x6e4), notifies the
/// owner (virtual 0x48 with 0x20) and deletes the entry when it is empty
/// and the container holds as many as its number; else clears the list of an
/// entry with a negative number. The actor's lighting is refreshed. Returns
/// false when the entry was deleted or a list was merged, true otherwise.
#[allow(clippy::too_many_arguments)]
pub fn fn_004c0cf0(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    removed: u32,
    form: u32,
    count: i32,
    actor: u32,
    extra: u32,
    worn_flag: u8,
    _unused_1: u32,
    entry: u32,
    matched: u32,
) -> bool {
    e.mem.set_u32(LAST_MERGED_EXTRA_LIST, 0);
    fn_004bf0e0(e, this);
    let mut entry = entry;
    if entry == 0 {
        entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
    }
    if entry == 0 {
        return true;
    }
    e.mem.set_u8(removed, 1);
    let mut result = true;
    let mut power_armor = false;
    let mut special_weapon = false;
    let mut node = e.mem.u32(entry);
    while node != 0 {
        let worn_extra = list_item(e, node);
        if worn_extra == 0 {
            break;
        }
        if !e
            .call(EXTRA_GET_WORN, &args![worn_extra, worn_flag as u32])
            .bool()
            || (extra != 0 && extra != worn_extra)
        {
            node = list_next(e, node);
            continue;
        }
        e.call(GET_FORM_AS_BIPED_MODEL, &args![form]);
        if e.call(EXTRA_GET_CAN_NOT_WEAR, &args![worn_extra]).bool()
            && !e.vcall(actor, 0x22c, &args![0u32]).bool()
        {
            e.mem.set_u8(removed, 0);
            return result;
        }
        if actor != 0 && e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
            match form_type_of(e, form) {
                0x18 => {
                    let biped = form + ARMOR_BIPED_MODEL;
                    if e.call(BIPED_IS_POWER_ARMOR, &args![biped]).bool()
                        || fn_004c0bd0(e, Ptr::new(biped))
                    {
                        power_armor = true;
                    }
                    if dismemberment_allows_slot_update(e, actor) {
                        let process = e.mem.u32(actor + ACTOR_CURRENT_PROCESS);
                        e.vcall(process, 0x468, &args![1u32]);
                    }
                }
                0x1a => {
                    if dismemberment_allows_slot_update(e, actor) {
                        let process = e.mem.u32(actor + ACTOR_CURRENT_PROCESS);
                        e.vcall(process, 0x468, &args![1u32]);
                    }
                }
                0x28 => {
                    let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                    let kept = e.vcall(acquire, 0x148, &args![]).u32();
                    if kept != 0 {
                        let head = e.mem.u32(kept);
                        if list_item(e, head) == worn_extra || extra == 0 {
                            if actor == e.global::<u32>(PLAYER_GLOBAL) {
                                fn_004c0cd0(e, Ptr::new(actor), 0);
                            }
                            if fn_004c0bf0(e, Ptr::new(form)) {
                                special_weapon = true;
                            }
                            if e.call(PREDICATE_008C7AA0, &args![]).bool() {
                                let kept_form = e.call(WORD_AT_8, &args![kept]).u32();
                                let queue = e.call(TASK_QUEUE_GETTER, &args![]).u32();
                                e.call(TASK_QUEUE_DETACH_WEAPON, &args![queue, actor, kept_form]);
                            } else {
                                e.call(REFR_REMOVE_WEAPON, &args![actor]);
                                let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                                e.vcall(acquire, 0x160, &args![0u32, 0u32, 0u32]);
                                let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                                e.vcall(acquire, 0x168, &args![0u32]);
                            }
                        }
                    }
                }
                0x29 => {
                    let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                    let kept = e.vcall(acquire, 0x14c, &args![]).u32();
                    if kept != 0 {
                        let head = e.mem.u32(kept);
                        if list_item(e, head) == worn_extra {
                            e.call(NO_OPERATION_00483710, &args![actor]);
                            let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
                            e.vcall(acquire, 0x168, &args![0u32]);
                        }
                    }
                }
                _ => {}
            }
        }
        e.call(EXTRA_SET_WORN, &args![worn_extra, 0u32, worn_flag as u32]);
        if e.call(EXTRA_ITEMS_IN_LIST, &args![worn_extra]).u32() < 2 {
            if (e.call(EXTRA_GET_HOT_KEY, &args![worn_extra]).u8() as i8) < 0 {
                e.call(EXTRA_SET_COUNT, &args![worn_extra, 0u32]);
            } else {
                let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
                let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                e.call(EXTRA_SET_COUNT, &args![worn_extra, number - listed]);
            }
        }
        if fn_004bca60(e, Ptr::new(worn_extra)) {
            let list = e.mem.u32(entry);
            list_call_with_item(e, LIST_REMOVE, list, worn_extra);
            delete_object(e, worn_extra);
            if worn_extra == extra && matched != 0 {
                e.mem.set_u8(matched, 1);
            }
            result = false;
            break;
        }
        let mut other_node = e.mem.u32(entry);
        while other_node != 0 && list_item(e, other_node) != 0 && result {
            let other = list_item(e, other_node);
            let flag = special_weapon as u32;
            let equal = e
                .call(
                    COMPARE_LIST_FOR_CONTAINER,
                    &args![worn_extra, other, 0u32, flag],
                )
                .bool()
                || e.call(
                    COMPARE_LIST_FOR_CONTAINER,
                    &args![other, worn_extra, 0u32, flag],
                )
                .bool();
            if equal || worn_extra == other {
                other_node = list_next(e, other_node);
                continue;
            }
            let held = e.call(EXTRA_GET_COUNT, &args![other]).u16() as i16 as i32;
            e.call(EXTRA_SET_COUNT, &args![other, held + count]);
            e.mem.set_u32(LAST_MERGED_EXTRA_LIST, other);
            if (e.call(EXTRA_GET_HOT_KEY, &args![worn_extra]).u8() as i8) >= 0 {
                let key = e.call(EXTRA_GET_HOT_KEY, &args![worn_extra]).u8() as u32;
                e.call(EXTRA_SET_HOT_KEY, &args![other, key]);
            }
            let list = e.mem.u32(entry);
            if fn_004bca60(e, Ptr::new(other)) {
                list_call_with_item(e, LIST_REMOVE, list, other);
                delete_object(e, other);
            } else {
                list_call_with_item(e, LIST_REMOVE, list, worn_extra);
            }
            delete_object(e, worn_extra);
            result = false;
        }
        break;
    }
    if power_armor && actor != 0 && e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
        let acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(acquire, 0x6e4, &args![actor]);
    }
    let owner = e.get(this, InventoryChanges::pRef).addr();
    if owner != 0 {
        e.vcall(owner, 0x48, &args![0x20u32]);
    }
    let container = fn_004bffb0(e, this);
    let in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
    let head = e.mem.u32(entry);
    let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
    if head != 0
        && e.call(LIST_IS_EMPTY, &args![head]).bool()
        && (number == 0 || in_container == number)
    {
        let list = e.get(this, InventoryChanges::pListofChanges).addr();
        list_call_with_item(e, LIST_REMOVE, list, entry);
        delete_item_change(e, entry);
        return false;
    }
    if number < 0 && head != 0 {
        e.call(LIST_CLEAR, &args![head]);
    }
    if actor != 0 {
        let lighting = e.call(ACTOR_LIGHTING_ARGUMENT, &args![actor]).u32();
        e.call(ACTOR_INIT_LIGHTING, &args![actor, lighting]);
    }
    result
}

// Translated from 004c1c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the scratch `TESObjectREFR` (`InventoryChanges::pTempRef`, Xbox
/// PDB, a global) through its scalar deleting destructor and clears the
/// global.
pub fn fn_004c1c40(e: &mut Engine) {
    let temp_ref = e.global::<u32>(TEMP_REF_GLOBAL);
    delete_object(e, temp_ref);
    e.mem.set_u32(TEMP_REF_GLOBAL, 0);
}

// Translated from 004c1520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts `count` of `form` into the changes as an extra data list of its own
/// (the counterpart of `fn_004bffe0` that does not wear anything): the
/// entry of `form` is found (or made, appended to the changes) like in
/// `fn_004bffe0`, `extra` (0 for any) is matched against the lists of the
/// entry (`CompareList`), and an entry that is already worn does nothing.
///
/// The list used is `extra`, else the first list of the entry whose count is
/// 1 (or that must not be split: count below 2, ammunition, a special
/// weapon), else a unit split off the first list (`CopyList` with count 1,
/// the original reduced by one, its hot key removed). A given `extra` that
/// is in the entry is given the entry's number as its count when it can not
/// be split. The chosen list gets the count `count` (16 bits) and is added
/// to the entry's lists and to those of `extra_out` (an `ItemChange`, or 0)
/// when they do not hold it yet. When no list was found a new one is made,
/// given the count when it is above 1, and added to both. The C++ exception
/// frame is not translated.
pub fn fn_004c1520(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    count: i32,
    _unused_1: u32,
    extra: u32,
    extra_out: u32,
) {
    let mut extra = extra;
    with_scope_guard(e, 0xb83, |e| {
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 {
            e.vcall(owner, 0x48, &args![0x20u32]);
        }
        let mut entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        let container = fn_004bffb0(e, this);
        let in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
        let mut total = in_container;
        if entry != 0 {
            total += e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
        }
        let mut in_lists = 0i32;
        if entry != 0 && e.mem.u32(entry) != 0 {
            let counted = item_change_get_extra_total_count(e, Ptr::new(entry), false);
            in_lists = counted + item_change_get_extra_total_default_count(e, Ptr::new(entry));
        }
        let mut free = total - in_lists;
        if entry != 0 && e.mem.u32(entry) != 0 {
            let head = e.mem.u32(entry);
            if list_item(e, head) != 0 {
                let first = list_item(e, head);
                if e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool() {
                    free = in_container - in_lists;
                }
            }
            let mut matched = false;
            if extra != 0 {
                let head = e.mem.u32(entry);
                let held = with_item_slot(e, extra, |e, slot| {
                    e.call(LIST_CONTAINS, &args![head, slot]).bool()
                });
                if !held {
                    let mut cursor = e.mem.u32(entry);
                    while cursor != 0 && list_item(e, cursor) != 0 {
                        let candidate = list_item(e, cursor);
                        if candidate != 0
                            && !e.call(EXTRA_COMPARE_LIST, &args![candidate, extra]).bool()
                        {
                            extra = candidate;
                            matched = true;
                            break;
                        }
                        cursor = list_next(e, cursor);
                    }
                    if !matched {
                        extra = 0;
                    }
                }
            }
        }
        let mut needs_new = true;
        if (extra == 0 && free > 0) || entry == 0 {
            if entry == 0 {
                entry = new_item_change(e, form, 0);
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_ADD_TAIL, list, entry);
                extra = 0;
            }
        } else {
            if item_change_get_worn(e, Ptr::new(entry), 0) {
                return;
            }
            let mut cursor = e.mem.u32(entry);
            while cursor != 0 && needs_new {
                let used = list_item(e, cursor);
                if extra != 0 && extra == used {
                    let special = is_special_weapon(e, form);
                    let count_here = e.call(EXTRA_GET_COUNT, &args![used]).u16() as i16;
                    if count_here < 2 || form_type_of(e, form) == 0x29 || special {
                        let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
                        e.call(EXTRA_SET_COUNT, &args![used, number]);
                    } else {
                        extra = split_extra_list(e, used);
                    }
                    needs_new = false;
                } else if used != 0 && extra == 0 {
                    if e.call(EXTRA_GET_COUNT, &args![used]).u16() as i16 == 1 {
                        extra = used;
                    } else {
                        let special = is_special_weapon(e, form);
                        let count_here = e.call(EXTRA_GET_COUNT, &args![used]).u16() as i16;
                        if count_here < 2 || form_type_of(e, form) == 0x29 || special {
                            extra = used;
                        } else {
                            extra = split_extra_list(e, used);
                        }
                    }
                    needs_new = false;
                } else {
                    cursor = list_next(e, cursor);
                }
            }
        }
        if extra != 0 {
            e.call(EXTRA_SET_COUNT, &args![extra, count as u16 as u32]);
            needs_new = false;
            ensure_extra_list(e, entry);
            let head = e.mem.u32(entry);
            let held = with_item_slot(e, extra, |e, slot| {
                e.call(LIST_CONTAINS, &args![head, slot]).bool()
            });
            if !held {
                list_call_with_item(e, LIST_ADD, head, extra);
            }
            if extra_out != 0 {
                let out_head = e.mem.u32(extra_out);
                let held = with_item_slot(e, extra, |e, slot| {
                    e.call(LIST_CONTAINS, &args![out_head, slot]).bool()
                });
                if !held {
                    list_call_with_item(e, LIST_ADD, out_head, extra);
                }
            }
        }
        if needs_new {
            let created = new_extra_list(e);
            if count > 1 {
                e.call(EXTRA_SET_COUNT, &args![created, count as u16 as u32]);
            }
            ensure_extra_list(e, entry);
            let head = e.mem.u32(entry);
            let held = with_item_slot(e, created, |e, slot| {
                e.call(LIST_CONTAINS, &args![head, slot]).bool()
            });
            if !held {
                list_call_with_item(e, LIST_ADD, head, created);
            }
            if extra_out != 0 {
                ensure_extra_list(e, extra_out);
                let out_head = e.mem.u32(extra_out);
                let held = with_item_slot(e, created, |e, slot| {
                    e.call(LIST_CONTAINS, &args![out_head, slot]).bool()
                });
                if !held {
                    list_call_with_item(e, LIST_ADD, out_head, created);
                }
            }
        }
    });
}

/// The low word of the 64-bit integer the game's `FISTP` (truncating mode)
/// stores for `value`.
fn truncate_to_low_word(value: f64) -> u32 {
    (value.trunc() as i64) as u32
}

/// `005b5e40(format, name, id, owner name, owner id)`: the log line of the
/// "Adding ... ref '%s' (%08X) to ref '%s' (%08X)." messages of `fn_004c1c90`
/// (the owner's id and name are evaluated first, as the game pushes them).
fn log_adding_reference(e: &mut Engine, this: Ptr<InventoryChanges>, format: u32, reference: u32) {
    let owner = e.get(this, InventoryChanges::pRef).addr();
    let owner_id = e.call(FORM_ID, &args![owner]).u32();
    let owner_name = e.vcall(owner, FORM_NAME_SLOT, &args![]).u32();
    let reference_id = e.call(FORM_ID, &args![reference]).u32();
    let reference_name = e.vcall(reference, FORM_NAME_SLOT, &args![]).u32();
    e.call(
        SAVE_LOAD_LOG,
        &args![format, reference_name, reference_id, owner_name, owner_id],
    );
}

/// The base object of a reference (`007af430`; the map names it
/// `BGSSaveFormBuffer::GetForm` because of folding).
fn reference_base_form(e: &mut Engine, reference: u32) -> u32 {
    e.call(REFERENCE_BASE_FORM, &args![reference]).u32()
}

/// The extra data list of a reference (`005d43c0`).
fn reference_extra_list(e: &mut Engine, reference: u32) -> u32 {
    e.call(REFR_EXTRA_LIST, &args![reference]).u32()
}

/// The `TESHealthForm` of a base object, by a dynamic cast from
/// `TESBoundObject`.
fn base_health_form(e: &mut Engine, base: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            base,
            0u32,
            TYPE_TES_BOUND_OBJECT,
            TYPE_TES_HEALTH_FORM,
            0u32
        ],
    )
    .u32()
}

/// Whether the health extra of `reference` is the "unset" `-1.0`.
fn reference_health_is_unset(e: &mut Engine, reference: u32) -> bool {
    let list = reference_extra_list(e, reference);
    let health = e.call(EXTRA_GET_HEALTH, &args![list]).f32();
    health as f64 == e.global::<f64>(MINUS_ONE_DOUBLE)
}

/// `TESObjectREFR` health setter `00568bd0(reference, health)` on a health
/// value taken as an unsigned integer.
fn set_reference_health(e: &mut Engine, reference: u32, health: u32) {
    e.call(
        REFR_SET_HEALTH,
        &args![reference, (health as u64 as f64) as f32],
    );
}

// Translated from 004c1c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the reference `reference` (an item lying in the world, `count` of
/// it) to the changes: the cached container weight is forgotten, health is
/// given to items that have none yet, the reference's own extra data list
/// becomes (or is merged into) an extra data list of the entry for its base
/// object, and the weight of the ammunition it holds is added.
///
/// * A weapon that has health and no health extra (`-1.0`), with a weapon
///   type below 10, gets a health of 99% to the full health (the unsigned
///   minimum of `health * 0.99` and `health - 1`, the weapon's mod effect
///   10 added when it has mods), logged as "Adding full health weapon ref".
///   A throwing weapon (`fn_004c0bf0`) whose health is set is given its
///   base health back, logged as "Adding damaged throwing weapon ref".
/// * Armor (0x18) and 0x1a forms with health and no health extra get the
///   unsigned minimum of `health / 0.99` and `health - 1`, logged as
///   "Adding full health armor ref".
/// * The ammo extra (type 0x6e) of the reference is removed and the owner
///   told (virtual 0x48 with 0x800); its value times `count` is added to
///   the owner's ammunition at the end (`fn_004c29a0`).
/// * A new extra list is copied from the reference's, scaled (not for
///   ammunition or special weapons) and stripped of the player's ownership
///   when the player adds it to the player; a worn one is marked (`worn`).
///   With an entry for the base object its number grows by `count`; a
///   persistent reference (`GetRefPersists`) keeps the new list as its own
///   with the reference set, else it is merged into the first list of the
///   entry it is equal to (the count added) or added; a plain list is
///   dropped; an entry left empty with number 0 is removed and deleted.
///   Without an entry a new `ItemChange` is appended to the changes.
///
/// The C++ exception frame is not translated.
pub fn fn_004c1c90(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    reference: u32,
    count: i32,
    _unused_1: u32,
    worn: u8,
) {
    with_scope_guard(e, 0xc3b, |e| {
        e.set(this, InventoryChanges::bcountdirty, true);
        let previous = e.get(this, InventoryChanges::fcontainerweight);
        e.set(this, InventoryChanges::fpreviousContainerWeight, previous);
        let none = e.global::<f32>(MINUS_ONE_FLOAT);
        e.set(this, InventoryChanges::fcontainerweight, none);
        let base = reference_base_form(e, reference);
        if form_type_of(e, base) == 0x28 {
            let base = reference_base_form(e, reference);
            let has_health = e.call(GET_FORM_HEALTH, &args![base]).u32() != 0;
            if has_health && reference_health_is_unset(e, reference) {
                let base = reference_base_form(e, reference);
                if (e.call(WEAPON_TYPE_GETTER, &args![base]).u32() as i32) < 10 {
                    log_adding_reference(e, this, ADDING_FULL_HEALTH_WEAPON, reference);
                    let base = reference_base_form(e, reference);
                    let health_form = base_health_form(e, base);
                    let mut health = e.vcall(health_form, 0x10, &args![]).u32();
                    let list = reference_extra_list(e, reference);
                    if e.call(EXTRA_HAS_WEAPON_MODS, &args![list]).bool() {
                        let base = reference_base_form(e, reference);
                        let bonus = tes_object_weap_get_mod_effect_value(
                            e,
                            Ptr::new(base),
                            MOD_EFFECT_HEALTH,
                            0,
                        );
                        health = truncate_to_low_word(health as f64 + bonus as f64);
                    }
                    let scaled =
                        truncate_to_low_word(health as f64 * e.global::<f64>(NEAR_FULL_HEALTH));
                    health = e
                        .call(UNSIGNED_MINIMUM, &args![scaled, health.wrapping_sub(1)])
                        .u32();
                    set_reference_health(e, reference, health);
                }
            } else {
                let base = reference_base_form(e, reference);
                if fn_004c0bf0(e, Ptr::new(base)) && !reference_health_is_unset(e, reference) {
                    log_adding_reference(e, this, ADDING_DAMAGED_THROWING_WEAPON, reference);
                    let base = reference_base_form(e, reference);
                    let health_form = base_health_form(e, base);
                    let health = e.vcall(health_form, 0x10, &args![]).u32();
                    set_reference_health(e, reference, health);
                }
            }
        }
        let base = reference_base_form(e, reference);
        let mut armor = form_type_of(e, base) == 0x18;
        if !armor {
            let base = reference_base_form(e, reference);
            armor = form_type_of(e, base) == 0x1a;
        }
        if armor {
            let base = reference_base_form(e, reference);
            let has_health = e.call(GET_FORM_HEALTH, &args![base]).u32() != 0;
            if has_health && reference_health_is_unset(e, reference) {
                log_adding_reference(e, this, ADDING_FULL_HEALTH_ARMOR, reference);
                let base = reference_base_form(e, reference);
                let health_form = base_health_form(e, base);
                let health = e.vcall(health_form, 0x10, &args![]).u32();
                let scaled =
                    truncate_to_low_word(health as f64 / e.global::<f64>(NEAR_FULL_HEALTH));
                let health = e
                    .call(UNSIGNED_MINIMUM, &args![scaled, health.wrapping_sub(1)])
                    .u32();
                set_reference_health(e, reference, health);
            }
        }
        let mut ammunition = 0i32;
        let list = reference_extra_list(e, reference);
        let ammo_extra = e.call(EXTRA_GET_AMMO, &args![list]).u32();
        if ammo_extra != 0 {
            ammunition = e.mem.u32(ammo_extra + 0x10) as i32;
            let list = reference_extra_list(e, reference);
            e.call(EXTRA_SET_AMMO, &args![list, 0u32, 0u32]);
            e.vcall(reference, 0x48, &args![0x800u32]);
        }
        ammunition = ammunition.wrapping_mul(count);
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 {
            e.vcall(owner, 0x48, &args![0x20u32]);
        }
        let base = reference_base_form(e, reference);
        let entry = inventory_changes_get_object_in_list(e, this, base, 1, 0);
        let list = reference_extra_list(e, reference);
        e.call(EXTRA_REMOVE_COUNT, &args![list]);
        let mut created = new_extra_list(e);
        let mut special_weapon = false;
        let list = reference_extra_list(e, reference);
        if !fn_004bca60(e, Ptr::new(list)) {
            let mut weapon = 0u32;
            let base = reference_base_form(e, reference);
            if form_type_of(e, base) == 0x28 {
                weapon = reference_base_form(e, reference);
                special_weapon = fn_004c0bf0(e, Ptr::new(weapon));
            }
            let list = reference_extra_list(e, reference);
            e.call(EXTRA_COPY_LIST_FOR_CONTAINER, &args![created, list, 0u32]);
            let base = reference_base_form(e, reference);
            if form_type_of(e, base) != 0x29 && (weapon == 0 || !fn_004c0bf0(e, Ptr::new(weapon))) {
                let scale = e.call(REFERENCE_SCALE, &args![reference]).f32();
                e.call(EXTRA_SET_SCALE, &args![created, scale]);
            }
            let player = e.global::<u32>(PLAYER_GLOBAL);
            if e.call(EXTRA_GET_OWNERSHIP, &args![created]).u32() == player
                && e.get(this, InventoryChanges::pRef).addr() == player
            {
                e.call(EXTRA_REMOVE_OWNERSHIP, &args![created]);
            }
        }
        if worn != 0 {
            e.call(EXTRA_SET_WORN, &args![created, 1u32, 0u32]);
        }
        if entry != 0 {
            let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
            e.set(
                Ptr::<ItemChange>::new(entry),
                ItemChange::iNumber,
                number + count,
            );
            if e.call(REFERENCE_PERSISTS, &args![reference]).bool() {
                e.call(EXTRA_SET_REFERENCE, &args![created, reference]);
                if count > 1 {
                    e.call(EXTRA_SET_COUNT, &args![created, count as u16 as u32]);
                }
                ensure_extra_list(e, entry);
                let head = e.mem.u32(entry);
                list_call_with_item(e, LIST_ADD, head, created);
            } else {
                let mut cursor = e.mem.u32(entry);
                let mut searching = true;
                while cursor != 0 && list_item(e, cursor) != 0 && searching {
                    let other = list_item(e, cursor);
                    let flag = special_weapon as u32;
                    let mut equal = true;
                    if created != 0 {
                        equal = e
                            .call(
                                COMPARE_LIST_FOR_CONTAINER,
                                &args![created, other, 0u32, flag],
                            )
                            .bool()
                            || e.call(
                                COMPARE_LIST_FOR_CONTAINER,
                                &args![other, created, 0u32, flag],
                            )
                            .bool();
                    }
                    if equal {
                        cursor = list_next(e, cursor);
                        continue;
                    }
                    let held = e.call(EXTRA_GET_COUNT, &args![other]).u16() as i16 as i32;
                    e.call(EXTRA_SET_COUNT, &args![other, held + count]);
                    if fn_004bca60(e, Ptr::new(other)) {
                        let head = e.mem.u32(entry);
                        list_call_with_item(e, LIST_REMOVE, head, other);
                        delete_object(e, other);
                    }
                    searching = false;
                }
                if !searching {
                    delete_object(e, created);
                    created = 0;
                } else if created != 0 && fn_004bca60(e, Ptr::new(created)) {
                    delete_object(e, created);
                    created = 0;
                } else {
                    ensure_extra_list(e, entry);
                    let head = e.mem.u32(entry);
                    list_call_with_item(e, LIST_ADD, head, created);
                }
            }
            let head = e.mem.u32(entry);
            if head != 0
                && e.call(LIST_IS_EMPTY, &args![head]).bool()
                && e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber) == 0
            {
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_REMOVE, list, entry);
                delete_object(e, created);
                created = 0;
                delete_item_change(e, entry);
            }
        } else {
            let base = reference_base_form(e, reference);
            let added = new_item_change(e, base, count as u32);
            if count > 1 {
                e.call(EXTRA_SET_COUNT, &args![created, count as u16 as u32]);
            }
            if e.call(REFERENCE_PERSISTS, &args![reference]).bool() {
                e.call(EXTRA_SET_REFERENCE, &args![created, reference]);
                ensure_extra_list(e, added);
                let head = e.mem.u32(added);
                list_call_with_item(e, LIST_ADD, head, created);
            } else if created != 0 {
                if fn_004bca60(e, Ptr::new(created)) {
                    delete_object(e, created);
                    created = 0;
                } else {
                    ensure_extra_list(e, added);
                    let head = e.mem.u32(added);
                    list_call_with_item(e, LIST_ADD, head, created);
                }
            }
            let list = e.get(this, InventoryChanges::pListofChanges).addr();
            list_call_with_item(e, LIST_ADD_TAIL, list, added);
        }
        if created != 0 && fn_004bca60(e, Ptr::new(created)) {
            delete_object(e, created);
        }
        if ammunition > 0 {
            let base = reference_base_form(e, reference);
            let owner = e.get(this, InventoryChanges::pRef).addr();
            // The owner is asked whether it is an actor (virtual 0x100); the
            // answer is not used.
            e.vcall(owner, 0x100, &args![]);
            let ammo = e.call(WEAPON_GET_CURRENT_AMMO, &args![base, 0u32]).u32();
            if ammo != 0 {
                fn_004c29a0(e, this, ammo, 0, ammunition);
            }
        }
    });
}

// Translated from 004c29a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `count` of `form` to the changes, with `extra` as its extra data
/// list (0 for none; an empty one is made first). The cached container
/// weight is forgotten; the value of the ammo extra (type 0x6e) of `extra`
/// times `count` (a count of 0 is taken as 1 afterwards) is added to the
/// owner's ammunition at the end (the weapon's current ammo, `fn_004c29a0`
/// again).
///
/// The player's ownership is removed from `extra` when the player adds it to
/// the player. For a weapon (0x28) added to an actor with a current process
/// whose process keeps this weapon as its ammunition-less item (virtual
/// 0x14c, a special weapon), the numbers of the kept items (virtual 0x148
/// and 0x14c) are raised by `count` and `extra` is marked worn (`SetWorn(1,
/// 0)`) unless the entry already is. A reference kept in `extra`
/// (`GetReferencePointer`) gets its own extra list set to the owner.
///
/// Without an entry for `form` a new `ItemChange` is made with the list
/// (a plain `extra` is dropped first) and handed to `fn_004c3380`. With one:
/// a leveled item list with no number and no ownership or script is
/// added to by its first list; else `extra` is merged into the list of the
/// entry it is equal to (the count added, a plain list unlinked and deleted;
/// `extra` is deleted unless one of two key state tests, `00705020` and
/// `00705000`, is true) or added to the entry. The entry's number is raised
/// by `count` (or set to it when it was negative and the container holds
/// none) and the entry removed when it is empty with number 0.
///
/// The C++ exception frame is not translated.
pub fn fn_004c29a0(e: &mut Engine, this: Ptr<InventoryChanges>, form: u32, extra: u32, count: i32) {
    let mut extra = extra;
    let mut count = count;
    with_scope_guard(e, 0xd28, |e| {
        e.set(this, InventoryChanges::bcountdirty, true);
        let previous = e.get(this, InventoryChanges::fcontainerweight);
        e.set(this, InventoryChanges::fpreviousContainerWeight, previous);
        let none = e.global::<f32>(MINUS_ONE_FLOAT);
        e.set(this, InventoryChanges::fcontainerweight, none);
        let mut ammunition = 0i32;
        if extra == 0 {
            extra = new_extra_list(e);
        }
        if extra != 0 {
            let ammo_extra = e.call(EXTRA_GET_AMMO, &args![extra]).u32();
            if ammo_extra != 0 {
                ammunition = e.mem.u32(ammo_extra + 0x10) as i32;
            }
        }
        ammunition = ammunition.wrapping_mul(count);
        if count == 0 {
            count = 1;
        }
        let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        let mut special_weapon = false;
        let player = e.global::<u32>(PLAYER_GLOBAL);
        if extra != 0
            && e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32() == player
            && e.get(this, InventoryChanges::pRef).addr() == player
        {
            e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
        }
        let mut actor = 0u32;
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 && e.vcall(owner, 0x100, &args![]).bool() {
            actor = e.get(this, InventoryChanges::pRef).addr();
        }
        if form_type_of(e, form) == 0x28
            && actor != 0
            && e.call(ACTOR_PROCESS, &args![actor]).u32() != 0
        {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            if fn_004c0bf0(e, Ptr::new(form)) && e.vcall(process, 0x14c, &args![]).u32() != 0 {
                let kept = e.vcall(process, 0x14c, &args![]).u32();
                if form == e.call(WORD_AT_8, &args![kept]).u32() {
                    special_weapon = true;
                    if e.vcall(process, 0x148, &args![]).u32() != 0 {
                        let kept = e.vcall(process, 0x148, &args![]).u32();
                        let number =
                            e.get(Ptr::<ItemChange>::new(kept), ItemChange::iNumber) + count;
                        let kept = e.vcall(process, 0x148, &args![]).u32();
                        e.call(ITEM_SET_NUMBER, &args![kept, number]);
                    }
                    if e.vcall(process, 0x14c, &args![]).u32() != 0 {
                        let kept = e.vcall(process, 0x14c, &args![]).u32();
                        let number =
                            e.get(Ptr::<ItemChange>::new(kept), ItemChange::iNumber) + count;
                        let kept = e.vcall(process, 0x14c, &args![]).u32();
                        e.call(ITEM_SET_NUMBER, &args![kept, number]);
                    }
                    if entry == 0 || !item_change_get_worn(e, Ptr::new(entry), 0) {
                        if extra == 0 {
                            extra = new_extra_list(e);
                        }
                        e.call(EXTRA_SET_WORN, &args![extra, 1u32, 0u32]);
                    }
                }
            }
        }
        if extra != 0 && e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32() != 0 {
            let held = e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32();
            if held != 0 {
                let owner = e.get(this, InventoryChanges::pRef).addr();
                let held_list = reference_extra_list(e, held);
                e.call(EXTRA_SET_REFERENCE, &args![held_list, owner]);
            }
        }
        if entry == 0 {
            if extra != 0 && fn_004bca60(e, Ptr::new(extra)) {
                delete_object(e, extra);
                extra = 0;
            }
            let added = new_item_change(e, form, count as u32);
            ensure_extra_list(e, added);
            let head = e.mem.u32(added);
            list_call_with_item(e, LIST_ADD, head, extra);
            fn_004c3380(e, this, added, 1);
        } else {
            let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
            if number <= 0
                && extra != 0
                && e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32() == 0
                && e.call(EXTRA_GET_SCRIPT, &args![extra]).u32() == 0
                && e.mem.u32(entry) != 0
            {
                let head = e.mem.u32(entry);
                if e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0 {
                    let first = list_item(e, head);
                    if e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool() {
                        let first = list_item(e, head);
                        let held = e.call(EXTRA_GET_COUNT, &args![first]).u16() as i16 as i32;
                        let first = list_item(e, head);
                        e.call(EXTRA_SET_COUNT, &args![first, held + count]);
                        delete_object(e, extra);
                        extra = 0;
                    }
                }
            }
            if extra != 0 && e.mem.u32(entry) != 0 {
                let mut cursor = e.mem.u32(entry);
                let mut searching = true;
                while cursor != 0 && list_item(e, cursor) != 0 && searching {
                    let other = list_item(e, cursor);
                    let flag = special_weapon as u32;
                    let equal = e
                        .call(COMPARE_LIST_FOR_CONTAINER, &args![extra, other, 0u32, flag])
                        .bool()
                        || e.call(COMPARE_LIST_FOR_CONTAINER, &args![other, extra, 0u32, flag])
                            .bool();
                    if equal {
                        cursor = list_next(e, cursor);
                        continue;
                    }
                    let held = e.call(EXTRA_GET_COUNT, &args![other]).u16() as i16 as i32;
                    e.call(EXTRA_SET_COUNT, &args![other, held + count]);
                    if fn_004bca60(e, Ptr::new(other)) {
                        let head = e.mem.u32(entry);
                        list_call_with_item(e, LIST_REMOVE, head, other);
                        delete_object(e, other);
                    }
                    searching = false;
                }
                if !searching {
                    if !e.call(KEY_STATE_TEST_041D, &args![]).bool()
                        && !e.call(KEY_STATE_TEST_0435, &args![]).bool()
                    {
                        delete_object(e, extra);
                        extra = 0;
                    }
                } else if extra != 0 && fn_004bca60(e, Ptr::new(extra)) {
                    delete_object(e, extra);
                    extra = 0;
                } else {
                    ensure_extra_list(e, entry);
                    let head = e.mem.u32(entry);
                    list_call_with_item(e, LIST_ADD, head, extra);
                }
            } else if extra != 0 && !fn_004bca60(e, Ptr::new(extra)) {
                ensure_extra_list(e, entry);
                let head = e.mem.u32(entry);
                list_call_with_item(e, LIST_ADD, head, extra);
            }
            let container = fn_004bffb0(e, this);
            let in_container = if container != 0 {
                e.call(CONTAINER_COUNT, &args![container, form]).i32()
            } else {
                0
            };
            let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
            if number < 0 && in_container <= 0 {
                e.call(ITEM_SET_NUMBER, &args![entry, count]);
            } else {
                let number = e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber);
                e.call(ITEM_SET_NUMBER, &args![entry, number + count]);
            }
            let head = e.mem.u32(entry);
            if head != 0
                && e.call(LIST_IS_EMPTY, &args![head]).bool()
                && e.get(Ptr::<ItemChange>::new(entry), ItemChange::iNumber) == 0
            {
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_REMOVE, list, entry);
                delete_item_change(e, entry);
            }
        }
        if ammunition > 0 {
            let owner = e.get(this, InventoryChanges::pRef).addr();
            let actor = if e.vcall(owner, 0x100, &args![]).bool() {
                e.get(this, InventoryChanges::pRef).addr()
            } else {
                0
            };
            let ammo = e.call(WEAPON_GET_CURRENT_AMMO, &args![form, actor]).u32();
            if ammo != 0 {
                fn_004c29a0(e, this, ammo, 0, ammunition);
            }
        }
    });
}

// Translated from 004c3380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Merges the `ItemChange` `source` (a new entry that is not in the changes)
/// into the changes: the cached container weight is forgotten and the owner
/// told (virtual 0x48 with 0x20). The destination is the entry for the same
/// form (`GetObjectInList`); without one `source` is appended to the
/// changes. Else the destination's number grows by the source's, and each
/// extra list of the source (with a reference pointer, its reference set to
/// the owner) is added to the destination's lists, or when the destination
/// holds an equal one (`CompareListForContainer` either way, with the
/// special weapon flag for a weapon form) only its count is added to that
/// one and the source's list unlinked and deleted. With `delete_source` the
/// source is deleted afterwards. A destination that ends empty with number
/// 0 is removed from the changes and deleted (and the source with it). A
/// source whose list is an empty head node gets that list deleted first.
/// The C++ exception frame is not translated.
pub fn fn_004c3380(e: &mut Engine, this: Ptr<InventoryChanges>, source: u32, delete_source: u8) {
    let mut source = source;
    with_scope_guard(e, 0xe03, |e| {
        let previous = e.get(this, InventoryChanges::fcontainerweight);
        e.set(this, InventoryChanges::fpreviousContainerWeight, previous);
        let none = e.global::<f32>(MINUS_ONE_FLOAT);
        e.set(this, InventoryChanges::fcontainerweight, none);
        if source == 0 {
            return;
        }
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 {
            e.vcall(owner, 0x48, &args![0x20u32]);
        }
        let form = e.call(WORD_AT_8, &args![source]).u32();
        let mut destination = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        let form = e.call(WORD_AT_8, &args![source]).u32();
        let mut special_weapon = false;
        if form_type_of(e, form) == 0x28 {
            special_weapon = fn_004c0bf0(e, Ptr::new(form));
        }
        let source_list = e.mem.u32(source);
        if source_list != 0 && e.call(LIST_IS_EMPTY, &args![source_list]).bool() {
            if source_list != 0 {
                e.call(LIST_DESTROY, &args![source_list, 1u32]);
            }
            e.mem.set_u32(source, 0);
        }
        if destination == 0 {
            let list = e.get(this, InventoryChanges::pListofChanges).addr();
            list_call_with_item(e, LIST_ADD_TAIL, list, source);
            return;
        }
        let total = e.get(Ptr::<ItemChange>::new(destination), ItemChange::iNumber)
            + e.get(Ptr::<ItemChange>::new(source), ItemChange::iNumber);
        e.call(ITEM_SET_NUMBER, &args![destination, total]);
        let source_list = e.mem.u32(source);
        if source_list == 0 || !e.call(LIST_IS_EMPTY, &args![source_list]).bool() {
            let mut destination_node = e.mem.u32(destination);
            let mut source_node = e.mem.u32(source);
            while source_node != 0 && list_item(e, source_node) != 0 {
                let moved = list_item(e, source_node);
                let mut is_new = true;
                if e.call(EXTRA_GET_REFERENCE_POINTER, &args![moved]).u32() != 0 {
                    let owner = e.get(this, InventoryChanges::pRef).addr();
                    e.call(EXTRA_SET_REFERENCE, &args![moved, owner]);
                }
                while destination_node != 0 && list_item(e, destination_node) != 0 && is_new {
                    let held = list_item(e, destination_node);
                    let flag = special_weapon as u32;
                    let mut equal = true;
                    if moved != 0 {
                        equal = e
                            .call(COMPARE_LIST_FOR_CONTAINER, &args![moved, held, 0u32, flag])
                            .bool()
                            || e.call(COMPARE_LIST_FOR_CONTAINER, &args![held, moved, 0u32, flag])
                                .bool();
                    }
                    if equal {
                        destination_node = list_next(e, destination_node);
                        continue;
                    }
                    let count = e.call(EXTRA_GET_COUNT, &args![held]).u16() as i16 as i32
                        + e.call(EXTRA_GET_COUNT, &args![moved]).u16() as i16 as i32;
                    e.call(EXTRA_SET_COUNT, &args![held, count]);
                    if moved != 0 {
                        let list = e.mem.u32(source);
                        list_call_with_item(e, LIST_REMOVE, list, moved);
                        delete_object(e, moved);
                    }
                    is_new = false;
                }
                if is_new {
                    ensure_extra_list(e, destination);
                    let head = e.mem.u32(destination);
                    list_call_with_item(e, LIST_ADD, head, moved);
                }
                source_node = list_next(e, source_node);
            }
            if delete_source != 0 {
                delete_item_change(e, source);
                source = 0;
            }
        }
        destination = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        if destination != 0 {
            let head = e.mem.u32(destination);
            if (head == 0 || e.call(LIST_IS_EMPTY, &args![head]).bool())
                && e.get(Ptr::<ItemChange>::new(destination), ItemChange::iNumber) == 0
            {
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_REMOVE, list, destination);
                delete_item_change(e, destination);
                delete_item_change(e, source);
            }
        }
    });
}

// Translated from 004c69f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands its object to `00403df0` (the string object to `char *`
/// conversion, `STRING_OBJECT_TO_CHARS`) and returns that function's result.
pub fn fn_004c69f0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(STRING_OBJECT_TO_CHARS, &args![this]).u32()
}

/// The search both `GetObjectbyPackObjType` and `fn_004c6ba0` run: `matches`
/// tells whether a form is of the wanted kind.
///
/// First the container's own objects (the list `00717e50` gives for the
/// container of the owner, items of two words: count, form) are searched
/// for a matching form; for the first one the count is stored in `*count`
/// (the entry's number added when the changes have the form; nothing
/// stored when that total is 0). Then, when none was found, the changes
/// themselves are searched (the entry's form, at +8): a matching form that
/// is not in the container's objects (`00481eb0`) gives its number as
/// `*count` when above 0. Returns the form or 0.
fn find_object_by_kind(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    count: u32,
    matches: &mut dyn FnMut(&mut Engine, u32) -> bool,
) -> u32 {
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    let mut found = 0u32;
    while node != 0 && found == 0 && list_item(e, node) != 0 {
        let object = list_item(e, node);
        found = e.mem.u32(object + 4);
        if found != 0 && matches(e, found) {
            let entry = inventory_changes_get_object_in_list(e, this, found, 1, 0);
            let object_count = {
                let object = list_item(e, node);
                e.mem.u32(object)
            };
            if entry == 0 {
                e.mem.set_u32(count, object_count);
            } else {
                let total = entry_number(e, entry).wrapping_add(object_count as i32);
                if total != 0 {
                    e.mem.set_u32(count, total as u32);
                }
            }
        } else {
            found = 0;
        }
        node = list_next(e, node);
    }
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    while node != 0 && found == 0 && list_item(e, node) != 0 {
        let item = list_item(e, node);
        found = e.mem.u32(item + 8);
        if found != 0 && matches(e, found) {
            let container = fn_004bffb0(e, this);
            if !e.call(CONTAINER_HAS_FORM, &args![container, found]).bool() {
                let item = list_item(e, node);
                let number = entry_number(e, item);
                if number > 0 {
                    e.mem.set_u32(count, number as u32);
                }
            }
        } else {
            found = 0;
        }
        node = list_next(e, node);
    }
    found
}

// Translated from 004c6a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetObjectbyPackObjType` (Xbox PDB): the first form of
/// the container or the changes whose package object type
/// (`TESPackage::GetObjectTypeFromForm`) is `object_type`; its count is
/// stored in `*count` (see `find_object_by_kind`). 0 when there is none.
pub fn inventory_changes_get_objectby_pack_obj_type(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    object_type: u32,
    count: u32,
) -> u32 {
    find_object_by_kind(e, this, count, &mut |e, form| {
        e.call(PACKAGE_OBJECT_TYPE_FROM_FORM, &args![form]).u32() == object_type
    })
}

// Translated from 004c6ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same search as `GetObjectbyPackObjType`, for the first form whose
/// form type (`00401170`) is `form_type`.
pub fn fn_004c6ba0(e: &mut Engine, this: Ptr<InventoryChanges>, form_type: u32, count: u32) -> u32 {
    find_object_by_kind(e, this, count, &mut |e, form| {
        form_type_of(e, form) == form_type
    })
}

// Translated from 004c6d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `count` items of the kind `form_type` out of the changes: the
/// cached container weight is forgotten (the previous weight kept) and, while
/// something is left to take, the form `fn_004c6ba0` finds (its count
/// limited to what is left) is removed with `fn_004c37d0` (no owner, drop,
/// target or extra list, `delete_extras` set). The game calls it with a
/// null owner, which `fn_004c37d0` calls virtual 0x100 on.
pub fn fn_004c6d40(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form_type: u32,
    owner_flag: u8,
    count: i32,
) {
    let previous = e.get(this, InventoryChanges::fcontainerweight);
    e.set(this, InventoryChanges::fpreviousContainerWeight, previous);
    let none = e.global::<f32>(MINUS_ONE_FLOAT);
    e.set(this, InventoryChanges::fcontainerweight, none);
    let mut count = count;
    e.with_stack(8, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.mem.set_u32(slot.addr() + 4, 0);
        while count > 0 {
            let form = fn_004c6ba0(e, this, form_type, slot.addr() + 4);
            let mut taken = e.mem.u32(slot.addr() + 4) as i32;
            if taken > count {
                taken = count;
            }
            e.mem.set_u32(slot.addr() + 4, taken as u32);
            fn_004c37d0(e, this, 0, form, owner_flag, taken, 0, 0, 0, 0, 0, 1, 0, 0);
            count -= taken;
        }
    });
}

/// A new empty `ItemChange` (`0076b630`, the third constructor: list, number
/// and form all 0), constructed unless the allocation failed.
fn new_default_item_change(e: &mut Engine) -> u32 {
    let item = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if item == 0 {
        return 0;
    }
    e.call(ITEM_CHANGE_CONSTRUCT_EMPTY, &args![item]).u32()
}

/// The check the item-dropping code of `fn_004c37d0` runs after it has
/// placed `dropped` in the world for `actor`: when the actor's parent cell is
/// owned by someone else (`546a40`) and does not allow the actor
/// (`546ca0`) and `567770` of the dropped reference is 0, or when the actor's
/// base form (type 0x2a) is of sex 1 and the dropped object is a biped
/// model form whose world models for 1 and 0 differ, the dropped reference
/// is handed the actor's base form (`00567ad0`).
fn after_drop_checks(e: &mut Engine, actor: u32, dropped: u32) {
    let mut handed_over = false;
    let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
    if e.call(DROPPED_REFERENCE_TEST, &args![dropped]).u32() == 0
        && cell != 0
        && e.call(CELL_GET_OWNER, &args![cell]).u32() != 0
        && !e.call(CELL_TEST_ACTOR, &args![cell, actor]).bool()
    {
        handed_over = true;
    }
    if !handed_over {
        let base = reference_base_form(e, actor);
        if form_type_of(e, base) == 0x2a {
            let base = reference_base_form(e, actor);
            if e.call(ACTOR_BASE_GET_SEX, &args![base]).u32() == 1 {
                let dropped_base = reference_base_form(e, dropped);
                let biped = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![
                            dropped_base,
                            0u32,
                            TYPE_TES_BOUND_OBJECT,
                            TYPE_TES_BIPED_MODEL_FORM,
                            0u32
                        ],
                    )
                    .u32();
                if biped != 0 {
                    let first = e.call(BIPED_WORLD_MODEL, &args![biped, 1u32]).u32();
                    let second = e.call(BIPED_WORLD_MODEL, &args![biped, 0u32]).u32();
                    if first != second {
                        handed_over = true;
                    }
                }
            }
        }
    }
    if handed_over {
        let base = reference_base_form(e, actor);
        e.call(REFERENCE_HAND_OVER, &args![dropped, base]);
    }
}

/// The owner a removed item is given to the receiving container with: for
/// an actor (virtual 0x100) the original base of its leveled creature
/// extra (`GetLevCreaOriginalBase` of its extra data list) or else its base
/// form; for anything else `TESObjectREFR::GetOwner`.
fn removal_owner(e: &mut Engine, actor: u32) -> u32 {
    if e.vcall(actor, 0x100, &args![]).bool() {
        let list = reference_extra_list(e, actor);
        let mut owner = e.call(EXTRA_GET_ORIGINAL_BASE, &args![list]).u32();
        if owner == 0 {
            owner = reference_base_form(e, actor);
        }
        owner
    } else {
        e.call(REFERENCE_GET_OWNER, &args![actor]).u32()
    }
}

/// Gives the dropped reference `dropped` the extra data of `split`: its
/// scale (`00418860` of the list) is set on the reference (`00567490`), its
/// extras copied into the reference's list (`CopyListForReference` with
/// `1`) and the list's `0041afd0` run on that list.
fn copy_extra_onto_dropped(e: &mut Engine, dropped: u32, split: u32) {
    let scale = e.call(EXTRA_GET_SCALE_FLOAT, &args![split]).f32();
    e.call(REFERENCE_SET_SCALE, &args![dropped, scale]);
    let list = reference_extra_list(e, dropped);
    e.call(EXTRA_COPY_FOR_REFERENCE, &args![list, split, 1u32]);
    let list = reference_extra_list(e, dropped);
    e.call(EXTRA_FINISH_COPY, &args![list]);
}

/// Drops `count` of the entry's form for `actor` when `split` (or, if there
/// is none, `other`) is an extra data list that carries a reference
/// (`GetReferencePointer`): that reference is the one dropped (`*result`
/// keeps its old value when neither list has one, and nothing happens when
/// it is 0). The reference gets virtual 0xc4 with 0, the list's
/// `0041c900`, the scale and extras of `split` (flag: `other` is null),
/// and the starting world or cell of its list is set to itself when it has
/// none (`GetStartingWorldOrCell`, `SetStartingWorldOrCellForRef`, then
/// virtual 0x48 with 0x400); then `DropItemIntoWorld` places it. The temporary
/// `ItemChange` and `split` are deleted and the follow-up checks run.
#[allow(clippy::too_many_arguments)]
fn drop_referenced_item(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    entry: u32,
    count: i32,
    split: &mut u32,
    other: u32,
    position: u32,
    rotation: u32,
    temp: &mut u32,
    result: &mut u32,
) {
    if *split != 0 {
        *result = e.call(EXTRA_GET_REFERENCE_POINTER, &args![*split]).u32();
    } else if other != 0 {
        *result = e.call(EXTRA_GET_REFERENCE_POINTER, &args![other]).u32();
    }
    if *result == 0 {
        return;
    }
    e.vcall(*result, 0xc4, &args![0u32]);
    e.call(EXTRA_FN_0041C900, &args![*split]);
    let other_is_null = other == 0;
    let scale = e.call(EXTRA_GET_SCALE_FLOAT, &args![*split]).f32();
    e.call(REFERENCE_SET_SCALE, &args![*result, scale]);
    let list = reference_extra_list(e, *result);
    e.call(
        EXTRA_COPY_FOR_REFERENCE,
        &args![list, *split, other_is_null as u32],
    );
    let list = reference_extra_list(e, *result);
    e.call(EXTRA_FINISH_COPY, &args![list]);
    let list = reference_extra_list(e, *result);
    e.call(EXTRA_FN_0041C900, &args![list]);
    let list = reference_extra_list(e, *result);
    if e.call(EXTRA_GET_STARTING_WORLD_OR_CELL, &args![list]).u32() == 0 {
        let list = reference_extra_list(e, *result);
        e.call(EXTRA_SET_STARTING_WORLD_OR_CELL, &args![list, *result]);
        e.vcall(*result, 0x48, &args![0x400u32]);
    }
    let entry_form = e.mem.u32(entry + 8);
    inventory_changes_drop_item_into_world(
        e, this, actor, entry_form, count, *result, position, rotation,
    );
    delete_item_change(e, *temp);
    *temp = 0;
    delete_object(e, *split);
    *split = 0;
    after_drop_checks(e, actor, *result);
}

// Translated from 004c6dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::DropItemIntoWorld` (Xbox PDB): places `count` of
/// `form` in the world next to `actor` and returns the new reference.
///
/// The position is the one `position` points to (three floats), else the
/// actor's (virtual 0x1f4) moved by a vector built from the actor's
/// rotation (`00430830`, a `NiMatrix3` from Euler angles) applied to
/// `(0.0, 0x0101b268, 0x01018f5c)` (`NiPoint3` constructor `00416870`,
/// matrix times vector `004b4500`, vector add `0063c8a0`). The rotation is
/// the one `rotation` points to, else the actor's (`00430830`), else the
/// default at `0x011f426c`. The object is made by the game data manager
/// (the global at `0x011c3f2c`, `004698a0(form, &position, &rotation,
/// parent cell, 575d70(actor), reference, 0, 0)`); a count above 1 is stored in its
/// extra data (16 bits) and the reference is told (virtual 0x48 with
/// 0x400).
pub fn inventory_changes_drop_item_into_world(
    e: &mut Engine,
    _this: Ptr<InventoryChanges>,
    actor: u32,
    form: u32,
    count: i32,
    reference: u32,
    position: u32,
    rotation: u32,
) -> u32 {
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr();
        // Locals of the game's frame: the position, the actor's rotation, a
        // matrix, a vector, the matrix product and the rotation copy.
        let pos = base;
        let actor_rotation = base + 0x10;
        let matrix = base + 0x20;
        let vector = base + 0x50;
        let product = base + 0x60;
        let rotation_copy = base + 0x70;
        e.call(NODE_ITEM_ADDRESS, &args![pos]);
        if position != 0 {
            for i in 0..3 {
                let word = e.mem.u32(position + 4 * i);
                e.mem.set_u32(pos + 4 * i, word);
            }
        } else {
            let actor_pos = e.vcall(actor, 0x1f4, &args![]).u32();
            for i in 0..3 {
                let word = e.mem.u32(actor_pos + 4 * i);
                e.mem.set_u32(pos + 4 * i, word);
            }
            let actor_rot = e.call(ACTOR_ROTATION, &args![actor]).u32();
            for i in 0..3 {
                let word = e.mem.u32(actor_rot + 4 * i);
                e.mem.set_u32(actor_rotation + 4 * i, word);
            }
            e.call(NODE_ITEM_ADDRESS, &args![matrix]);
            let x = e.mem.f32(actor_rotation);
            let y = e.mem.f32(actor_rotation + 4);
            let z = e.mem.f32(actor_rotation + 8);
            e.call(MATRIX_FROM_EULER_ANGLES, &args![matrix, x, y, z]);
            let offset_z = e.global::<f32>(DROP_OFFSET_Z);
            let offset_y = e.global::<f32>(DROP_OFFSET_Y);
            e.call(POINT3_CONSTRUCT, &args![vector, 0.0f32, offset_y, offset_z]);
            let moved = e
                .call(MATRIX_TIMES_VECTOR, &args![matrix, product, vector])
                .u32();
            e.call(POINT3_ADD, &args![pos, moved]);
        }
        let rotation_source = if rotation != 0 {
            rotation
        } else if actor != 0 {
            e.call(ACTOR_ROTATION, &args![actor]).u32()
        } else {
            DEFAULT_ROTATION
        };
        for i in 0..3 {
            let word = e.mem.u32(rotation_source + 4 * i);
            e.mem.set_u32(rotation_copy + 4 * i, word);
        }
        // 004c6ef9: `00575d70` takes no stack word (plain RET); the pushed
        // reference, 0, 0 are the last three words of `004698a0` (RET 0x20).
        let placed_in = e.call(REFERENCE_WORLD_SPACE, &args![actor]).u32();
        let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
        let manager = e.global::<u32>(GAME_DATA_MANAGER_GLOBAL);
        let placed = e
            .call(
                PLACE_OBJECT,
                &args![
                    manager,
                    form,
                    pos,
                    rotation_copy,
                    cell,
                    placed_in,
                    reference,
                    0u32,
                    0u32
                ],
            )
            .u32();
        if count > 1 {
            let list = reference_extra_list(e, placed);
            e.call(EXTRA_SET_COUNT, &args![list, count as u16 as u32]);
        }
        e.vcall(placed, 0x48, &args![0x400u32]);
        placed
    })
}

// Translated from 004c37d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `count` of `form` out of the changes of `this` (the removal that
/// drops, transfers or just discards items); returns the reference of the
/// last item dropped into the world (or the reference of the last extra
/// list that carried one), else 0.
///
/// * `actor` is the owner reference (virtual 0x100 says whether it is an
///   actor), `set_owner` (when `form` is not gold) gives a transferred item
///   the owner it comes from, `extra` is the extra data list to take (0 for
///   any), `drop_flag` drops the items into the world next to `actor`
///   (`DropItemIntoWorld`, with `position` and `rotation`), else `target`
///   (0 for none) receives them through its virtual 0x190(form, extra,
///   count). `delete_extras` and `repeat` are flags (`repeat` runs the whole
///   removal again with the rest when something is left, and matches any
///   extra list with an owner); `entry_in` is the entry of the changes for
///   `form` when the caller has it.
/// * The cached container weight is forgotten and `actor`'s owner and
///   `target` told (virtual 0x48 with 0x20). The item count of the
///   container (`TESContainer::GetObjectCount`) adds to the entry's number;
///   with a script on a stack the container's count is not used.
/// * With extra lists to take (path A: `extra` has items or `repeat`), the
///   first list that is `extra` (or, with no `extra` and `repeat`, owned) is
///   used: a worn one that is fully taken is first unequipped (the actor's
///   `UnEquipObject`, or virtual 0x188) and the removal repeated on the
///   rest, then a re-equip of what is left; a list holding more than asked
///   (or a leveled item) is split (`CopyListForReference`) and the rest
///   kept; else the whole list is unlinked (a hot key moved with
///   `SetHotKeyItem`). The taken lists go to a temporary `ItemChange` that
///   is dropped (`drop_referenced_item` for lists with a reference),
///   handed to `target` or just deleted (`DeleteAllExtra` unless the menus
///   0x40b/0x422 are visible or two key state tests are true).
/// * Without extra lists (path B) the count not in extra lists is taken
///   from the entry (the worn count and listed lists excepted), the extra
///   lists with a count above 1 reduced, and the same drop/transfer done;
///   what is still to take then comes from the extra lists one by one.
/// * At the end the part of the removal that is only in the container
///   (counts not in the changes) is dropped or transferred (a transferred
///   quest note 0x31 to the player is added as a note, with karma for an
///   actor of the wrong alignment), the entry's number lowered or an
///   entry with a negative number made; an entry left empty is removed
///   and deleted; the player's kept weapon is told (virtual 0x3ec) when it
///   was the removed form.
///
/// The C++ exception frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_004c37d0(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    form: u32,
    set_owner: u8,
    count: i32,
    extra: u32,
    drop_flag: u8,
    target: u32,
    position: u32,
    rotation: u32,
    delete_extras: u8,
    repeat: u8,
    entry_in: u32,
) -> u32 {
    with_scope_guard(e, 0xe74, |e| {
        let mut count = count;
        let mut extra = extra;
        e.set(this, InventoryChanges::bcountdirty, true);
        let previous = e.get(this, InventoryChanges::fcontainerweight);
        e.set(this, InventoryChanges::fpreviousContainerWeight, previous);
        let none = e.global::<f32>(MINUS_ONE_FLOAT);
        e.set(this, InventoryChanges::fcontainerweight, none);
        let mut result = 0u32;
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 {
            e.vcall(owner, 0x48, &args![0x20u32]);
        }
        if target != 0 {
            e.vcall(target, 0x48, &args![0x20u32]);
        }
        let container = fn_004bffb0(e, this);
        let mut entry = if entry_in == 0 {
            inventory_changes_get_object_in_list(e, this, form, 1, 0)
        } else {
            entry_in
        };
        let mut temp = 0u32;
        let mut kept_weapon = false;
        let mut player_actor = 0u32;
        let player = e.global::<u32>(PLAYER_GLOBAL);
        if e.vcall(actor, 0x100, &args![]).bool() && actor == player {
            player_actor = actor;
        }
        if player_actor != 0 && (drop_flag != 0 || target != 0) {
            let process = e.call(ACTOR_PROCESS, &args![player_actor]).u32();
            if e.vcall(process, 0x14c, &args![]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS, &args![player_actor]).u32();
                let kept = e.vcall(process, 0x14c, &args![]).u32();
                if e.call(WORD_AT_8, &args![kept]).u32() == form {
                    kept_weapon = true;
                }
            }
        }
        if (drop_flag != 0 || target != 0)
            && extra != 0
            && e.call(EXTRA_GET_BY_TYPE, &args![extra, 0x4au32]).u32() != 0
            && e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32 == count
        {
            e.call(EXTRA_REMOVE_HOT_KEY, &args![extra]);
        }
        if extra != 0 && e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).u32() == 0 {
            if entry != 0 && e.mem.u32(entry) != 0 {
                let list = e.mem.u32(entry);
                list_call_with_item(e, LIST_REMOVE, list, extra);
            }
            delete_object(e, extra);
            extra = 0;
        }
        let mut leveled = 0u32;
        let mut in_container = 0i32;
        if actor != 0 {
            if container != 0 {
                in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
            }
            if entry != 0 && !item_change_get_worn(e, Ptr::new(entry), 0) {
                let total = in_container.wrapping_add(entry_number(e, entry));
                let defaults = item_change_get_extra_total_default_count(e, Ptr::new(entry));
                let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                if total > defaults + listed && item_change_get_script(e, Ptr::new(entry)) != 0 {
                    in_container = 0;
                }
            }
        }
        let mut negative_container = 0i32;
        if in_container < 0 {
            negative_container = in_container;
            in_container = in_container.wrapping_mul(-1);
        }
        if in_container < 0 && target != player {
            return 0;
        }
        if (entry != 0
            && entry_number(e, entry) != 0
            && entry_number(e, entry).wrapping_add(in_container) == 0)
            || (entry == 0 && in_container == 0)
        {
            return result;
        }
        let mut kept_going = false;
        while entry != 0 && count > 0 && entry_number(e, entry).wrapping_add(in_container) > 0 {
            delete_item_change(e, temp);
            temp = new_default_item_change(e);
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            e.mem.set_u32(temp + 8, entry_form);
            let mut node = e.mem.u32(entry);
            let mut searching = true;
            let wants_extras = (extra != 0
                && e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).u32() != 0)
                || repeat != 0;
            if wants_extras && node != 0 && list_item(e, node) != 0 {
                // Path A: take the extra lists one by one.
                loop {
                    if node == 0 || list_item(e, node) == 0 || !searching {
                        break;
                    }
                    let mut extra_item = list_item(e, node);
                    let mut split: u32;
                    let matched = extra == extra_item
                        || (extra == 0
                            && repeat != 0
                            && e.call(EXTRA_GET_OWNERSHIP, &args![extra_item]).u32() != 0);
                    if !matched {
                        node = list_next(e, node);
                    } else {
                        extra = 0;
                        searching = false;
                        let mut item_count =
                            e.call(EXTRA_GET_COUNT, &args![extra_item]).u16() as i16 as i32;
                        if e.call(EXTRA_GET_WORN, &args![extra_item, 0u32]).bool()
                            && e.call(EXTRA_GET_COUNT, &args![extra_item]).u16() as i16 as i32
                                <= count
                        {
                            // A worn list that is fully taken: unequip first.
                            let mut merge_flag = false;
                            let items = e.call(EXTRA_ITEMS_IN_LIST, &args![extra_item]).u32();
                            if items <= 1 {
                                merge_flag = true;
                            } else if e.call(EXTRA_ITEMS_IN_LIST, &args![extra_item]).u32() == 2
                                && e.call(EXTRA_GET_COUNT, &args![extra_item]).u16() as i16 as i32
                                    > 1
                            {
                                merge_flag = true;
                            } else if e.call(EXTRA_HAS_LEVELED_ITEM, &args![extra_item]).bool()
                                && e.call(EXTRA_ITEMS_IN_LIST, &args![extra_item]).u32() == 2
                            {
                                merge_flag = true;
                            }
                            let mut unequipped = false;
                            if e.vcall(actor, 0x100, &args![]).bool()
                                && !e.call(ACTOR_TEST_00440DA0, &args![actor]).bool()
                            {
                                if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
                                    let held = e.call(EXTRA_GET_COUNT, &args![extra_item]).u16()
                                        as i16
                                        as i32;
                                    let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                                    unequipped = e
                                        .call(
                                            ACTOR_UNEQUIP_OBJECT,
                                            &args![
                                                actor, entry_form, held, extra_item, 0u32, 0u32,
                                                1u32
                                            ],
                                        )
                                        .bool();
                                }
                            } else {
                                let held =
                                    e.call(EXTRA_GET_COUNT, &args![extra_item]).u16() as i16 as i32;
                                let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                                unequipped = e
                                    .vcall(actor, 0x188, &args![entry_form, held, extra_item])
                                    .bool();
                            }
                            if !unequipped || merge_flag {
                                extra_item = 0;
                            }
                            entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                            if entry == 0 || !item_change_get_worn(e, Ptr::new(entry), 0) {
                                result = fn_004c37d0(
                                    e, this, actor, form, set_owner, count, extra_item, drop_flag,
                                    target, position, rotation, 1, 0, 0,
                                );
                            }
                            if entry != 0 && item_count > 1 && item_count - count > 0 && {
                                let owner = e.get(this, InventoryChanges::pRef).addr();
                                e.vcall(owner, 0x100, &args![]).bool()
                            } {
                                let owner = e.get(this, InventoryChanges::pRef).addr();
                                let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                                e.call(
                                    ACTOR_EQUIP_OBJECT,
                                    &args![
                                        owner,
                                        entry_form,
                                        item_count - count,
                                        extra_item,
                                        1u32,
                                        0u32,
                                        0u32
                                    ],
                                );
                            }
                            delete_item_change(e, temp);
                            return result;
                        }
                        let leveled_now = e.call(EXTRA_GET_LEVELED_ITEM, &args![extra_item]).u32();
                        if item_count > count || leveled_now != 0 {
                            leveled = e.call(EXTRA_GET_LEVELED_ITEM, &args![extra_item]).u32();
                            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                            let kind = form_type_of(e, entry_form);
                            if e.call(EXTRA_GET_LEVELED_ITEM, &args![extra_item]).u32() == 0
                                || e.call(EXTRA_GET_SCRIPT, &args![extra_item]).u32() == 0
                                || kind == 0x18
                                || kind == 0x1a
                            {
                                if e.call(EXTRA_HAS_LEVELED_ITEM, &args![extra_item]).bool() {
                                    if count == item_count {
                                        let number = entry_number(e, entry) - item_count;
                                        set_entry_number(e, entry, number);
                                        e.call(
                                            EXTRA_SET_COUNT,
                                            &args![extra_item, item_count.wrapping_mul(-1)],
                                        );
                                    } else {
                                        let number = entry_number(e, entry) - count;
                                        set_entry_number(e, entry, number);
                                        e.call(
                                            EXTRA_SET_COUNT,
                                            &args![extra_item, item_count - count],
                                        );
                                    }
                                } else {
                                    let number = entry_number(e, entry) - count;
                                    set_entry_number(e, entry, number);
                                    e.call(EXTRA_SET_COUNT, &args![extra_item, item_count - count]);
                                }
                                split = new_extra_list(e);
                                e.call(EXTRA_COPY_FOR_REFERENCE, &args![split, extra_item, 0u32]);
                                e.call(EXTRA_FN_0041D000, &args![split]);
                                e.call(EXTRA_SET_COUNT, &args![split, count as u16 as u32]);
                                let items = e.call(EXTRA_ITEMS_IN_LIST, &args![split]).u32();
                                let drop_split = if items == 1
                                    && e.call(EXTRA_GET_COUNT, &args![split]).u16() as i16 as i32
                                        > 1
                                {
                                    true
                                } else {
                                    e.call(EXTRA_ITEMS_IN_LIST, &args![split]).u32() == 0
                                };
                                if drop_split {
                                    delete_object(e, split);
                                    split = 0;
                                }
                            } else {
                                if kind == 0x73 || kind == 0x18 || kind == 0x1a {
                                    e.call(EXTRA_SET_COUNT, &args![extra_item, 1u32]);
                                }
                                split = extra_item;
                                let list = e.mem.u32(entry);
                                list_call_with_item(e, LIST_REMOVE, list, extra_item);
                                let replacement = new_extra_list(e);
                                let leveled_base =
                                    e.call(EXTRA_GET_LEVELED_ITEM, &args![extra_item]).u32();
                                let leveled_value = e.mem.u32(leveled_base + 0xc);
                                e.call(EXTRA_FN_0041D280, &args![replacement, leveled_value]);
                                let list = e.mem.u32(entry);
                                list_call_with_item(e, LIST_ADD, list, replacement);
                                e.call(EXTRA_FN_0041D000, &args![split]);
                                let held =
                                    e.call(EXTRA_GET_COUNT, &args![extra_item]).u16() as i16 as i32;
                                set_entry_number(e, entry, held.wrapping_mul(-1));
                            }
                            e.call(SET_ACTION_FLAG, &args![actor, split, 4u32]);
                            item_count = count;
                            count -= count;
                        } else {
                            let number = entry_number(e, entry) - item_count;
                            set_entry_number(e, entry, number);
                            let temp_number =
                                e.get(Ptr::<ItemChange>::new(temp), ItemChange::iNumber);
                            e.call(ITEM_SET_NUMBER, &args![temp, temp_number + item_count]);
                            split = extra_item;
                            count -= item_count;
                            let list = e.mem.u32(entry);
                            list_call_with_item(e, LIST_REMOVE, list, extra_item);
                            if extra_item != 0
                                && (e.call(EXTRA_GET_HOT_KEY, &args![extra_item]).u8() as i8) >= 0
                                && entry_number(e, entry) > 0
                            {
                                let head_item = if e.mem.u32(entry) != 0 {
                                    let head = e.mem.u32(entry);
                                    list_item(e, head)
                                } else {
                                    0
                                };
                                let key =
                                    e.call(EXTRA_GET_HOT_KEY, &args![extra_item]).u8() as i8 as i32;
                                inventory_changes_set_hot_key_item(
                                    e,
                                    this,
                                    Ptr::new(entry),
                                    head_item,
                                    key,
                                );
                            }
                            extra_item = 0;
                            e.call(SET_ACTION_FLAG, &args![actor, split, 4u32]);
                            if fn_004bca60(e, Ptr::new(split)) {
                                delete_object(e, split);
                                split = 0;
                            }
                        }
                        if split != 0 && e.call(EXTRA_GET_WORN, &args![split, 0u32]).bool() {
                            e.call(EXTRA_FN_0041B010, &args![split, 0u32]);
                            if fn_004bca60(e, Ptr::new(split)) {
                                delete_object(e, split);
                                split = 0;
                            }
                        }
                        if split != 0 && e.call(EXTRA_ITEMS_IN_LIST, &args![split]).u32() != 0 {
                            ensure_extra_list(e, temp);
                            let head = e.mem.u32(temp);
                            list_call_with_item(e, LIST_ADD, head, split);
                        }
                        if drop_flag != 0 {
                            if split == 0
                                || e.call(EXTRA_GET_REFERENCE_POINTER, &args![split]).u32() == 0
                            {
                                let entry_form = e.mem.u32(entry + 8);
                                result = inventory_changes_drop_item_into_world(
                                    e, this, actor, entry_form, item_count, 0, position, rotation,
                                );
                                e.vcall(result, 0x48, &args![0x400u32]);
                                if split != 0
                                    && e.call(EXTRA_ITEMS_IN_LIST, &args![split]).u32() != 0
                                {
                                    copy_extra_onto_dropped(e, result, split);
                                }
                                after_drop_checks(e, actor, result);
                                delete_item_change(e, temp);
                                temp = 0;
                                delete_object(e, split);
                            } else {
                                drop_referenced_item(
                                    e,
                                    this,
                                    actor,
                                    entry,
                                    item_count,
                                    &mut split,
                                    extra_item,
                                    position,
                                    rotation,
                                    &mut temp,
                                    &mut result,
                                );
                            }
                        } else if target != 0 {
                            let temp_head = e.mem.u32(temp);
                            if temp_head != 0 {
                                let mut node_t = temp_head;
                                while node_t != 0 && list_item(e, node_t) != 0 {
                                    let given = list_item(e, node_t);
                                    if e.call(EXTRA_GET_OWNERSHIP, &args![given]).u32() == 0 {
                                        let item_owner = removal_owner(e, actor);
                                        if set_owner != 0 {
                                            let temp_form = e.call(WORD_AT_8, &args![temp]).u32();
                                            if !e.call(IS_GOLD, &args![temp_form]).bool() {
                                                e.call(EXTRA_SET_OWNER, &args![given, item_owner]);
                                            }
                                        }
                                    }
                                    let temp_form = e.call(WORD_AT_8, &args![temp]).u32();
                                    e.vcall(target, 0x190, &args![temp_form, given, item_count]);
                                    node_t = list_next(e, node_t);
                                }
                                delete_item_change(e, temp);
                                temp = 0;
                            } else {
                                let mut given = 0u32;
                                let item_owner = removal_owner(e, actor);
                                if set_owner != 0 {
                                    let temp_form = e.call(WORD_AT_8, &args![temp]).u32();
                                    if !e.call(IS_GOLD, &args![temp_form]).bool() {
                                        given = new_extra_list(e);
                                        e.call(EXTRA_SET_OWNER, &args![given, item_owner]);
                                        e.call(EXTRA_SET_COUNT, &args![given, count as u16 as u32]);
                                    }
                                }
                                let temp_form = e.call(WORD_AT_8, &args![temp]).u32();
                                e.vcall(target, 0x190, &args![temp_form, given, item_count]);
                                delete_item_change(e, temp);
                                temp = 0;
                            }
                            searching = false;
                        } else {
                            if !e.call(KEY_STATE_TEST_041D, &args![]).bool()
                                && !e.call(KEY_STATE_TEST_0435, &args![]).bool()
                                && !e.call(IS_MENU_VISIBLE, &args![0x40bu32, 0u32]).bool()
                                && !e.call(IS_MENU_VISIBLE, &args![0x422u32, 0u32]).bool()
                            {
                                item_change_delete_all_extra(e, Ptr::new(temp));
                            }
                            delete_item_change(e, temp);
                            temp = 0;
                        }
                    }
                    if count > 0 {
                        if repeat != 0 {
                            return fn_004c37d0(
                                e, this, actor, form, set_owner, count, 0, drop_flag, target,
                                position, rotation, 1, 0, 0,
                            );
                        }
                        if result != 0 {
                            return result;
                        }
                        if node == 0 {
                            extra = 0;
                            break;
                        }
                    }
                }
            } else {
                // Path B: take the count that is not in extra lists, then
                // the extra lists one by one.
                let available = if in_container < 0 {
                    in_container
                        .wrapping_mul(-1)
                        .wrapping_add(entry_number(e, entry))
                } else {
                    in_container.wrapping_add(entry_number(e, entry))
                };
                if available < 0 && in_container >= 0 {
                    return result;
                }
                let mut in_lists = 0i32;
                if e.mem.u32(entry) != 0 {
                    let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                    let worn = item_change_get_worn(e, Ptr::new(entry), 0);
                    in_lists = listed + worn as i32;
                }
                let mut free = available - in_lists;
                if free < 0 {
                    free = 0;
                }
                let mut taken_extra = 0u32;
                let mut dropped_extra = 0u32;
                let mut cursor = e.mem.u32(entry);
                let mut taken = 0i32;
                if free > 0 {
                    if item_change_get_worn(e, Ptr::new(entry), 0) {
                        let mut walk = e.mem.u32(entry);
                        while walk != 0 && list_item(e, walk) != 0 {
                            let list = list_item(e, walk);
                            let held = e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                            if e.call(EXTRA_GET_WORN, &args![list, 0u32]).bool() && held > 1 {
                                e.call(EXTRA_SET_COUNT, &args![list, held - count]);
                            }
                            walk = list_next(e, walk);
                        }
                    } else if fn_004bcb70(e, Ptr::new(entry)) {
                        let mut walk = e.mem.u32(entry);
                        while walk != 0 && list_item(e, walk) != 0 {
                            let list = list_item(e, walk);
                            let held = e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                            if e.call(EXTRA_HAS_LEVELED_ITEM, &args![list]).bool() && held > 1 {
                                e.call(EXTRA_SET_COUNT, &args![list, held - count]);
                            }
                            walk = list_next(e, walk);
                        }
                    }
                    if free >= count {
                        if negative_container == 0 {
                            let number = entry_number(e, entry) - count;
                            set_entry_number(e, entry, number);
                        }
                        taken = count;
                        count = 0;
                    } else {
                        if container != 0
                            && e.call(CONTAINER_COUNT, &args![container, form]).i32() > 0
                        {
                            let number = entry_number(e, entry) - free;
                            set_entry_number(e, entry, number);
                        }
                        taken = free;
                        count -= free;
                        if entry_number(e, entry).wrapping_add(in_container) < 0 {
                            if in_container != 0 {
                                set_entry_number(e, entry, in_container.wrapping_mul(-1));
                            } else {
                                set_entry_number(e, entry, 0);
                            }
                        } else {
                            let number = entry_number(e, entry) - taken;
                            set_entry_number(e, entry, number);
                        }
                    }
                    if drop_flag != 0 {
                        result = inventory_changes_drop_item_into_world(
                            e, this, actor, form, taken, 0, position, rotation,
                        );
                        after_drop_checks(e, actor, result);
                        delete_item_change(e, temp);
                        temp = 0;
                        if dropped_extra != 0 {
                            delete_object(e, dropped_extra);
                        }
                        dropped_extra = 0;
                    } else if target != 0 {
                        let item_owner = removal_owner(e, actor);
                        if set_owner != 0
                            && !e.call(IS_GOLD, &args![form]).bool()
                            && (form_type_of(e, form) != 0x29 || target != player)
                        {
                            if extra == 0 {
                                extra = new_extra_list(e);
                            }
                            e.call(EXTRA_SET_OWNER, &args![extra, item_owner]);
                            e.call(EXTRA_SET_COUNT, &args![extra, taken as u16 as u32]);
                        }
                        if extra != 0 && target == player && form_type_of(e, form) == 0x29 {
                            e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
                        }
                        e.vcall(target, 0x190, &args![form, extra, taken]);
                        delete_item_change(e, temp);
                        temp = 0;
                    }
                    delete_item_change(e, temp);
                    temp = 0;
                }
                // The count still to take comes from the extra lists.
                let listed_head = e.mem.u32(entry);
                if count > 0
                    && listed_head != 0
                    && !e.call(LIST_IS_EMPTY, &args![listed_head]).bool()
                {
                    cursor = e.mem.u32(entry);
                    while cursor != 0 && list_item(e, cursor) != 0 && count > 0 {
                        let mut usable = true;
                        taken_extra = list_item(e, cursor);
                        delete_item_change(e, temp);
                        temp = new_default_item_change(e);
                        if entry != 0 {
                            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                            e.mem.set_u32(temp + 8, entry_form);
                        }
                        if (e.call(EXTRA_GET_COUNT, &args![taken_extra]).u16() as i16) < 0 {
                            usable = false;
                            cursor = list_next(e, cursor);
                        } else {
                            if e.call(EXTRA_GET_WORN, &args![taken_extra, 0u32]).bool()
                                && e.call(EXTRA_GET_COUNT, &args![taken_extra]).u16() as i16 as i32
                                    <= 1
                            {
                                let mut clear_extra = false;
                                let items = e.call(EXTRA_ITEMS_IN_LIST, &args![taken_extra]).u32();
                                if items <= 1 {
                                    clear_extra = true;
                                } else if e.call(EXTRA_ITEMS_IN_LIST, &args![taken_extra]).u32()
                                    == 2
                                    && e.call(EXTRA_GET_COUNT, &args![taken_extra]).u16() as i16
                                        as i32
                                        > 1
                                {
                                    clear_extra = true;
                                }
                                if clear_extra {
                                    extra = 0;
                                }
                                let held = e.call(EXTRA_GET_COUNT, &args![taken_extra]).u16() as i16
                                    as i32;
                                let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                                let unequipped = e
                                    .vcall(actor, 0x188, &args![entry_form, held, taken_extra])
                                    .bool();
                                if unequipped {
                                    result = fn_004c37d0(
                                        e, this, actor, form, set_owner, count, extra, drop_flag,
                                        target, position, rotation, 1, 0, 0,
                                    );
                                } else {
                                    let text = fn_004c69f0(e, Ptr::new(NO_UNEQUIP_MESSAGE_OBJECT));
                                    let duration = e.global::<f32>(MESSAGE_DURATION);
                                    e.call(
                                        SHOW_MESSAGE,
                                        &args![text, 0u32, MESSAGE_ICON, 0u32, duration, 0u32],
                                    );
                                }
                                delete_item_change(e, temp);
                                return result;
                            }
                            let held =
                                e.call(EXTRA_GET_COUNT, &args![taken_extra]).u16() as i16 as i32;
                            taken = held;
                            if taken > count
                                || e.call(EXTRA_GET_LEVELED_ITEM, &args![taken_extra]).u32() != 0
                            {
                                leveled = e.call(EXTRA_GET_LEVELED_ITEM, &args![taken_extra]).u32();
                                if leveled != 0 {
                                    if count >= taken {
                                        let number =
                                            entry_number(e, entry) + taken.wrapping_mul(-1);
                                        set_entry_number(e, entry, number);
                                        e.call(
                                            EXTRA_SET_COUNT,
                                            &args![taken_extra, taken.wrapping_mul(-1)],
                                        );
                                    } else {
                                        let number =
                                            entry_number(e, entry) + count.wrapping_mul(-1);
                                        set_entry_number(e, entry, number);
                                        e.call(EXTRA_SET_COUNT, &args![taken_extra, taken - count]);
                                    }
                                } else {
                                    let number = entry_number(e, entry) - count;
                                    set_entry_number(e, entry, number);
                                    e.call(EXTRA_SET_COUNT, &args![taken_extra, taken - count]);
                                }
                                dropped_extra = new_extra_list(e);
                                e.call(
                                    EXTRA_COPY_FOR_REFERENCE,
                                    &args![dropped_extra, taken_extra, 0u32],
                                );
                                e.call(EXTRA_FN_0041D000, &args![dropped_extra]);
                                if taken < count {
                                    e.call(
                                        EXTRA_SET_COUNT,
                                        &args![dropped_extra, taken as u16 as u32],
                                    );
                                } else {
                                    e.call(
                                        EXTRA_SET_COUNT,
                                        &args![dropped_extra, count as u16 as u32],
                                    );
                                }
                                if count < taken {
                                    taken = count;
                                }
                                count -= taken;
                                if fn_004bca60(e, Ptr::new(dropped_extra)) {
                                    delete_object(e, dropped_extra);
                                    dropped_extra = 0;
                                } else {
                                    ensure_extra_list(e, temp);
                                    let head = e.mem.u32(temp);
                                    list_call_with_item(e, LIST_ADD, head, dropped_extra);
                                }
                            } else {
                                let number = entry_number(e, entry) - taken;
                                set_entry_number(e, entry, number);
                                let temp_number =
                                    e.get(Ptr::<ItemChange>::new(temp), ItemChange::iNumber);
                                e.call(ITEM_SET_NUMBER, &args![temp, temp_number + taken]);
                                dropped_extra = taken_extra;
                                count -= taken;
                                let list = e.mem.u32(entry);
                                list_call_with_item(e, LIST_REMOVE, list, dropped_extra);
                                taken_extra = 0;
                                if fn_004bca60(e, Ptr::new(dropped_extra)) {
                                    delete_object(e, dropped_extra);
                                    dropped_extra = 0;
                                } else {
                                    ensure_extra_list(e, temp);
                                    let head = e.mem.u32(temp);
                                    list_call_with_item(e, LIST_ADD, head, dropped_extra);
                                }
                            }
                        }
                        if entry != 0 && usable {
                            if drop_flag != 0 {
                                if dropped_extra == 0
                                    || e.call(EXTRA_GET_REFERENCE_POINTER, &args![dropped_extra])
                                        .u32()
                                        == 0
                                {
                                    let entry_form = e.mem.u32(entry + 8);
                                    result = inventory_changes_drop_item_into_world(
                                        e, this, actor, entry_form, taken, 0, position, rotation,
                                    );
                                    after_drop_checks(e, actor, result);
                                    if dropped_extra != 0
                                        && e.call(EXTRA_ITEMS_IN_LIST, &args![dropped_extra]).u32()
                                            != 0
                                    {
                                        copy_extra_onto_dropped(e, result, dropped_extra);
                                    }
                                    delete_item_change(e, temp);
                                    temp = 0;
                                    delete_object(e, dropped_extra);
                                    dropped_extra = 0;
                                } else {
                                    drop_referenced_item(
                                        e,
                                        this,
                                        actor,
                                        entry,
                                        taken,
                                        &mut dropped_extra,
                                        taken_extra,
                                        position,
                                        rotation,
                                        &mut temp,
                                        &mut result,
                                    );
                                }
                                delete_item_change(e, temp);
                                temp = 0;
                            } else if target != 0 {
                                let given;
                                let temp_head = e.mem.u32(temp);
                                if temp_head != 0 {
                                    given = list_item(e, temp_head);
                                } else {
                                    given = new_extra_list(e);
                                    let node_slot = new_list_node(e);
                                    e.mem.set_u32(temp, node_slot);
                                    list_call_with_item(e, LIST_ADD, node_slot, given);
                                }
                                let item_owner = removal_owner(e, actor);
                                let temp_form = e.call(WORD_AT_8, &args![temp]).u32();
                                if set_owner != 0 && !e.call(IS_GOLD, &args![temp_form]).bool() {
                                    e.call(EXTRA_SET_OWNER, &args![given, item_owner]);
                                    e.call(EXTRA_SET_COUNT, &args![given, taken as u16 as u32]);
                                } else {
                                    e.call(EXTRA_REMOVE_OWNERSHIP, &args![given]);
                                }
                                let temp_form = e.call(WORD_AT_8, &args![temp]).u32();
                                e.vcall(target, 0x190, &args![temp_form, given, taken]);
                                delete_item_change(e, temp);
                                temp = 0;
                            } else {
                                if delete_extras != 0 {
                                    item_change_delete_all_extra(e, Ptr::new(temp));
                                }
                                delete_item_change(e, temp);
                                temp = 0;
                            }
                            if taken_extra != 0 {
                                cursor = list_next(e, cursor);
                            }
                            if count != 0 {
                                kept_going = true;
                            }
                        } else {
                            delete_item_change(e, temp);
                            temp = 0;
                            delete_object(e, dropped_extra);
                            dropped_extra = 0;
                        }
                    }
                }
                let _ = (taken_extra, dropped_extra, cursor);
            }
            entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        }
        if repeat != 0 && count > 0 {
            return fn_004c37d0(
                e, this, actor, form, set_owner, count, extra, drop_flag, target, position,
                rotation, 1, 0, 0,
            );
        }
        if (entry == 0 || kept_going) && in_container != 0 {
            if entry != 0 {
                in_container = in_container.wrapping_add(entry_number(e, entry));
            }
            if in_container == 0 {
                let list = e.mem.u32(entry);
                e.call(LIST_CLEAR, &args![list]);
                return result;
            }
            if drop_flag != 0 {
                result = inventory_changes_drop_item_into_world(
                    e, this, actor, form, count, 0, position, rotation,
                );
                after_drop_checks(e, actor, result);
            } else if target != 0 {
                let item_owner = removal_owner(e, actor);
                if set_owner != 0 && !e.call(IS_GOLD, &args![form]).bool() {
                    if extra == 0 {
                        extra = new_extra_list(e);
                    }
                    e.call(EXTRA_SET_OWNER, &args![extra, item_owner]);
                    e.call(EXTRA_SET_COUNT, &args![extra, count as u16 as u32]);
                } else if extra != 0 {
                    e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
                }
                if count > 0 {
                    if form_type_of(e, form) == 0x31 && target == player {
                        let mut bad_karma = false;
                        if set_owner != 0 && item_owner != 0 {
                            if form_type_of(e, item_owner) == 8 {
                                bad_karma = !e.call(ACTOR_TEST_0047D7C0, &args![item_owner]).bool();
                            } else {
                                let karma = e.vcall(item_owner + 0x100, 0xc, &args![0x17u32]).f32();
                                let alignment = e.call(ALIGNMENT_FOR_KARMA, &args![karma]).u32();
                                if alignment != 2 && alignment != 4 {
                                    bad_karma = true;
                                }
                            }
                        }
                        if bad_karma {
                            let reward = e
                                .call(KARMA_REWARD_OBJECT, &args![KARMA_REWARD_OWNER])
                                .u32();
                            let value = e.mem.f32(reward);
                            let amount = float_to_int(e, value as f64);
                            e.call(PLAYER_REWARD_KARMA, &args![player, amount]);
                        }
                        e.call(PLAYER_ADD_NOTE, &args![player, form, 1u32]);
                    } else {
                        e.vcall(target, 0x190, &args![form, extra, count]);
                    }
                }
            }
            delete_item_change(e, temp);
            if entry == 0 {
                entry = new_item_change(e, form, count.wrapping_mul(-1) as u32);
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_ADD_TAIL, list, entry);
            } else {
                let number = entry_number(e, entry) - count;
                set_entry_number(e, entry, number);
            }
        }
        entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        if entry != 0 {
            let mut remove_entry = false;
            if leveled == 0 || e.vcall(actor, 0x100, &args![]).bool() {
                let number = entry_number(e, entry);
                remove_entry = if number != 0 || in_container != 0 {
                    let head = e.mem.u32(entry);
                    head != 0 && e.call(LIST_IS_EMPTY, &args![head]).bool() && number == 0
                } else {
                    true
                };
            }
            if remove_entry {
                if !e.call(KEY_STATE_TEST_041D, &args![]).bool()
                    && !e.call(KEY_STATE_TEST_0435, &args![]).bool()
                    && !e.call(IS_MENU_VISIBLE, &args![0x40bu32, 0u32]).bool()
                    && !e.call(IS_MENU_VISIBLE, &args![0x40bu32, 0u32]).bool()
                {
                    item_change_delete_all_extra(e, Ptr::new(entry));
                }
                let list = e.get(this, InventoryChanges::pListofChanges).addr();
                list_call_with_item(e, LIST_REMOVE, list, entry);
                delete_item_change(e, entry);
            } else if entry_number(e, entry).wrapping_add(in_container) < 0
                && !item_change_get_worn(e, Ptr::new(entry), 0)
                && e.mem.u32(entry) != 0
                && leveled == 0
            {
                let list = e.mem.u32(entry);
                e.call(LIST_CLEAR, &args![list]);
            }
        }
        if kept_weapon && e.call(ACTOR_CURRENT_WEAPON, &args![player_actor]).u32() != 0 {
            let weapon = e.call(ACTOR_CURRENT_WEAPON, &args![player_actor]).u32();
            e.vcall(player_actor, 0x3ec, &args![weapon, 0u32, 0u32, 0u32]);
        }
        result
    })
}

// Translated from 004c6f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ammunition with the best desirability (`TESActorBase::GetDesirability`
/// of `reference`'s object, the highest strictly above `-FLT_MAX`) among the
/// container's objects and the changes, as a new `ItemChange`; 0 when there
/// is none. An ammunition (a `TESAmmo` by dynamic cast) of the container
/// counts when the changes do not have it, hold more than they remove, or
/// remove (`count < 0`); one of the changes counts when its number is not 0
/// (and not negative on a leveled item) and the container does not hold it.
/// When both passes found one, the changes' one wins if it is strictly more
/// desirable. The result takes the changes' entry list and number when the
/// changes have an entry, else the container's count (made positive). The C++
/// exception frame is not translated.
pub fn fn_004c6f60(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    reference: u32,
    _unused_1: u32,
) -> u32 {
    with_scope_guard(e, 0x13b5, |e| {
        let mut best = e.global::<f32>(NEGATIVE_FLOAT_MAX);
        let mut best_container_form = 0u32;
        let mut best_changes_form = 0u32;
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        while node != 0 {
            let object = list_item(e, node);
            let mut ammo = 0u32;
            if object != 0 {
                let form = e.mem.u32(object + 4);
                ammo = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_AMMO, 0u32],
                    )
                    .u32();
            }
            if ammo != 0 {
                let entry = inventory_changes_get_object_in_list(e, this, ammo, 1, 0);
                let object = list_item(e, node);
                let count = e.mem.u32(object) as i32;
                let counts = entry == 0 || entry_number(e, entry).wrapping_add(count) > 0 || {
                    let object = list_item(e, node);
                    (e.mem.u32(object) as i32) < 0
                };
                if counts {
                    let desirability = e.call(GET_DESIRABILITY, &args![reference, ammo]).f32();
                    if best < desirability {
                        best = desirability;
                        best_container_form = ammo;
                    }
                }
            }
            node = list_next(e, node);
        }
        let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
        while node != 0 {
            let item = list_item(e, node);
            let mut ammo = 0u32;
            if item != 0 {
                let form = e.mem.u32(item + 8);
                ammo = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_AMMO, 0u32],
                    )
                    .u32();
            }
            if ammo != 0
                && (!fn_004bcb70(e, Ptr::new(item)) || entry_number(e, item) >= 0)
                && entry_number(e, item) != 0
            {
                let container = fn_004bffb0(e, this);
                if !e.call(CONTAINER_HAS_FORM, &args![container, ammo]).bool() {
                    let desirability = e.call(GET_DESIRABILITY, &args![reference, ammo]).f32();
                    if best < desirability {
                        best = desirability;
                        best_changes_form = ammo;
                    }
                }
            }
            node = list_next(e, node);
        }
        if best_changes_form != 0 && best_changes_form != best_container_form {
            let changes_value = e
                .call(GET_DESIRABILITY, &args![reference, best_changes_form])
                .f32();
            let container_value = e
                .call(GET_DESIRABILITY, &args![reference, best_container_form])
                .f32();
            if container_value < changes_value {
                best_container_form = best_changes_form;
            }
        }
        let entry = inventory_changes_get_object_in_list(e, this, best_container_form, 1, 0);
        let mut result = 0u32;
        if best_container_form != 0 {
            result = new_default_item_change(e);
        }
        if entry != 0 {
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            e.mem.set_u32(result + 8, entry_form);
            let entry_list = e.mem.u32(entry);
            if entry_list != 0 && e.mem.u32(entry_list) != 0 {
                let node = new_list_node(e);
                e.mem.set_u32(result, node);
                e.call(LIST_ADD, &args![node, entry_list]);
                let number = entry_number(e, entry);
                e.call(ITEM_SET_NUMBER, &args![result, number]);
            }
        } else if best_container_form != 0 {
            e.mem.set_u32(result + 8, best_container_form);
            let container = fn_004bffb0(e, this);
            let mut count = e
                .call(CONTAINER_COUNT, &args![container, best_container_form])
                .i32();
            if count < 0 {
                count = count.wrapping_mul(-1);
            }
            e.call(ITEM_SET_NUMBER, &args![result, count]);
        }
        result
    })
}

// Translated from 004c7300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ammunition `weapon`'s owner `actor` has loaded for it: `*found` is
/// set to 0, then to 1 once an ammunition list (the form at +4 of the
/// object at `actor + 0xa4` when it is of type 0x55; its list at +0x18) or
/// a single ammunition (type 0x29, from `00474a00`) is looked at. In a list
/// the first ammunition with a positive count in the changes
/// (`fn_004c8f30`) is taken from index `list[+0x20]` on; an earlier one is
/// kept as a fallback. 0 for a null `actor`.
pub fn fn_004c7300(e: &mut Engine, this: Ptr<InventoryChanges>, actor: u32, found: u32) -> u32 {
    e.mem.set_u8(found, 0);
    if actor == 0 {
        return 0;
    }
    let ammo_list = e.call(AMMO_LIST_OF_SLOT, &args![actor + 0xa4]).u32();
    let mut fallback = 0u32;
    if ammo_list != 0 {
        let mut index = 0u32;
        let first_added = e.call(REFERENCE_BASE_FORM, &args![ammo_list]).u32();
        let mut node = e.call(FORM_LIST_HEAD, &args![ammo_list]).u32();
        while node != 0 && list_item(e, node) != 0 {
            let ammo = list_item(e, node);
            e.mem.set_u8(found, 1);
            if fn_004c8f30(e, this, ammo) > 0 {
                if index < first_added {
                    fallback = ammo;
                } else {
                    return ammo;
                }
            }
            node = list_next(e, node);
            index += 1;
        }
    }
    if fallback != 0 {
        return fallback;
    }
    let single = e.call(SINGLE_AMMO_OF_SLOT, &args![actor + 0xa4]).u32();
    if single != 0 {
        e.mem.set_u8(found, 1);
        if fn_004c8f30(e, this, single) > 0 {
            return single;
        }
    }
    0
}

/// Replaces the best weapon found so far by a new `ItemChange` for `weapon`
/// (the old one deleted), with `extra` as its extra data list when given.
fn replace_best_weapon(e: &mut Engine, best: &mut u32, weapon: u32, extra: Option<u32>) {
    delete_item_change(e, *best);
    *best = new_default_item_change(e);
    e.mem.set_u32(*best + 8, weapon);
    if let Some(extra) = extra {
        let node = new_list_node(e);
        e.mem.set_u32(*best, node);
        list_call_with_item(e, LIST_ADD, node, extra);
    }
}

/// The health of an extra data list holding a weapon: the health extra (type
/// 0x25) when it has one, else the weapon form's own health as a float.
fn weapon_list_health(e: &mut Engine, weapon: u32, extra: u32) -> f32 {
    if e.call(EXTRA_GET_BY_TYPE, &args![extra, 0x25u32]).u32() != 0 {
        e.call(EXTRA_GET_HEALTH, &args![extra]).f32()
    } else {
        (e.call(GET_FORM_HEALTH, &args![weapon]).u32() as u64 as f64) as f32
    }
}

/// The rating `00646060(actor, weapon, health fraction, 0, entry,
/// ammunition)` gives a candidate weapon (a `__cdecl` function returning a
/// float).
fn rate_weapon(
    e: &mut Engine,
    actor: u32,
    weapon: u32,
    fraction: f32,
    entry: u32,
    ammunition: u32,
) -> f32 {
    e.call(
        RATE_WEAPON,
        &args![actor, weapon, fraction, 0u32, entry, ammunition],
    )
    .f32()
}

/// Asks the actor-value owner of `actor` (the sub-object at `actor + 0x100`,
/// virtual 0xc) for the value `008d85e0(weapon)` names; the game keeps the
/// result in a local it does not read again.
fn ask_actor_value(e: &mut Engine, actor: u32, weapon: u32) {
    let value_id = e.call(WEAPON_VALUE_ID, &args![weapon]).u32();
    e.vcall(actor + 0x100, 0xc, &args![value_id]);
}

// Translated from 004c7400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the best weapon of the kind `weapon_type` (6 for any) for `actor`
/// among the container's weapons and the changes', as a new `ItemChange`
/// (with the extra list of the best condition when it has some), and stores
/// its score (`00644540` of the best rating) in the float `*out`.
///
/// Each candidate weapon form (0x28) whose `GetCombatWeaponType` is
/// `weapon_type` is rated by `00646060(owner, weapon, health fraction, 0,
/// entry, ammunition)`, with the health fraction of each extra list (its
/// health, or the form's, over the form's health) or the last fraction
/// (1.0 to begin with), and kept when strictly better than the best so far
/// (the earlier result is deleted). The container pass counts a weapon
/// that the changes do not have, hold more than they remove or remove; the
/// pass over the changes counts an entry with a non-zero number (not
/// negative on a leveled entry) that the container does not hold. A weapon
/// whose ammunition is missing (`fn_004c7300`) or that the owner (an actor,
/// from the changes' owner reference) can not use (`008bc9d0`) is skipped.
/// An owner that has a process whose virtual 0x388 is true gets no result;
/// with `use_worn` a worn weapon (slot 5) that can not be taken off is
/// returned as it is. A result with a weapon type of 10, 11 or 13 gets the
/// count of the form (`fn_004c8f30`). The C++ exception frame is not
/// translated.
pub fn fn_004c7400(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    out: u32,
    weapon_type: i32,
    use_worn: u8,
) -> u32 {
    with_scope_guard(e, 0x1444, |e| {
        let mut best = 0.0f32;
        let mut result = 0u32;
        let mut rating_actor = actor;
        let mut owner_actor = 0u32;
        let owner = e.get(this, InventoryChanges::pRef).addr();
        if owner != 0 && e.vcall(owner, 0x100, &args![]).bool() {
            owner_actor = owner;
            rating_actor = owner;
            if e.call(ACTOR_PROCESS, &args![owner_actor]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS, &args![owner_actor]).u32();
                if e.vcall(process, 0x388, &args![]).bool() {
                    return result;
                }
            }
        }
        if use_worn != 0 {
            let worn = fn_004c8c10(e, this, 5, 0);
            if worn != 0 {
                let list = e.mem.u32(worn);
                let first = e.mem.u32(list);
                if e.call(EXTRA_GET_CAN_NOT_WEAR, &args![first]).bool() {
                    return worn;
                }
            }
            delete_item_change(e, worn);
        }
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        e.call(ACTOR_VALUE_FLOAT, &args![actor + 0x100, 0xbu32]);
        e.call(ACTOR_VALUE_INT, &args![actor + 0x100, 5u32]);
        if owner_actor != 0 {
            e.call(ACTOR_FATIGUE_PERCENTAGE, &args![owner_actor]);
        }
        let mut fraction = 1.0f32;
        // The container's weapons.
        while node != 0 {
            let object = list_item(e, node);
            let mut weapon = 0u32;
            if object != 0 {
                let form = e.mem.u32(object + 4);
                if form_type_of(e, form) == 0x28 {
                    weapon = e.mem.u32(object + 4);
                }
            }
            if weapon != 0 {
                let entry = inventory_changes_get_object_in_list(e, this, weapon, 1, 0);
                let counts = entry == 0
                    || {
                        let object = list_item(e, node);
                        entry_number(e, entry).wrapping_add(e.mem.u32(object) as i32) > 0
                    }
                    || {
                        let object = list_item(e, node);
                        (e.mem.u32(object) as i32) < 0
                    };
                if counts
                    && (weapon_type == 6
                        || weapon_type == e.call(COMBAT_WEAPON_TYPE, &args![weapon]).i32())
                {
                    let mut usable = true;
                    let owner = e.get(this, InventoryChanges::pRef).addr();
                    e.vcall(owner, 0x100, &args![]);
                    let (ammo, ammo_asked) = e.with_stack(4, |e, flag| {
                        e.mem.set_u8(flag.addr(), 0);
                        let ammo = fn_004c7300(e, this, weapon, flag.addr());
                        (ammo, e.mem.u8(flag.addr()))
                    });
                    if ammo_asked != 0 && ammo == 0 {
                        usable = false;
                    }
                    if owner_actor == 0
                        || (e
                            .call(ACTOR_CAN_USE_WEAPON, &args![owner_actor, weapon, 1u32])
                            .bool()
                            && usable)
                    {
                        e.vcall(weapon + 0x9c, 0x10, &args![]);
                        // The object `00726070` answers for `weapon + 0x74` (kept at
                        // EBP-0x70, 004c77d3..004c77f0) is the one whose slot 8 at +0x24 gives
                        // the limit below; 0 when there is none.
                        let ammo_slot = weapon + 0x74;
                        let mut limit_slot = if ammo_slot != 0 {
                            e.call(WORD_AT_4, &args![ammo_slot]).u32()
                        } else {
                            0
                        };
                        ask_actor_value(e, actor, weapon);
                        let has_extras = entry != 0 && e.mem.u32(entry) != 0 && {
                            let head = e.mem.u32(entry);
                            list_item(e, head) != 0
                        };
                        if has_extras {
                            if limit_slot != 0 {
                                let health = fn_004bd350(e, Ptr::new(entry));
                                let limit = e.vcall(limit_slot + 0x24, 0x8, &args![0u32]).f32();
                                if health < limit {
                                    limit_slot = 0;
                                }
                            }
                            let _ = limit_slot;
                            let mut cursor = e.mem.u32(entry);
                            while cursor != 0 {
                                let list = list_item(e, cursor);
                                if list != 0 {
                                    let health = weapon_list_health(e, weapon, list);
                                    if health > 0.0 {
                                        let form_health =
                                            e.call(GET_FORM_HEALTH, &args![weapon]).u32();
                                        fraction =
                                            (health as f64 / (form_health as u64 as f64)) as f32;
                                        let rating = rate_weapon(
                                            e,
                                            rating_actor,
                                            weapon,
                                            fraction,
                                            entry,
                                            ammo,
                                        );
                                        if best < rating {
                                            best = rating;
                                            replace_best_weapon(e, &mut result, weapon, Some(list));
                                        }
                                    }
                                }
                                cursor = list_next(e, cursor);
                            }
                        } else {
                            let rating =
                                rate_weapon(e, rating_actor, weapon, fraction, entry, ammo);
                            if best < rating {
                                best = rating;
                                replace_best_weapon(e, &mut result, weapon, None);
                            }
                        }
                    }
                }
            }
            node = list_next(e, node);
        }
        // The weapons of the changes.
        let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item = list_item(e, node);
            let mut weapon = 0u32;
            let item_form = e.mem.u32(item + 8);
            if form_type_of(e, item_form) == 0x28 {
                weapon = e.mem.u32(item + 8);
            }
            if weapon != 0 {
                let mut usable = true;
                let owner = e.get(this, InventoryChanges::pRef).addr();
                e.vcall(owner, 0x100, &args![]);
                let (ammo, ammo_asked) = e.with_stack(4, |e, flag| {
                    e.mem.set_u8(flag.addr(), 0);
                    let ammo = fn_004c7300(e, this, weapon, flag.addr());
                    (ammo, e.mem.u8(flag.addr()))
                });
                if ammo_asked != 0 && ammo == 0 {
                    usable = false;
                }
                if (weapon_type == 6
                    || weapon_type == e.call(COMBAT_WEAPON_TYPE, &args![weapon]).i32())
                    && entry_number(e, item) != 0
                {
                    let container = fn_004bffb0(e, this);
                    if !e.call(CONTAINER_HAS_FORM, &args![container, weapon]).bool()
                        && e.call(ACTOR_CAN_USE_WEAPON, &args![owner_actor, weapon, 1u32])
                            .bool()
                        && (!fn_004bcb70(e, Ptr::new(item)) || entry_number(e, item) >= 0)
                        && usable
                        && !((e.global::<f32>(MINUS_ONE_FLOAT) as f64)
                            < e.global::<f64>(MINUS_ONE_DOUBLE))
                    {
                        ask_actor_value(e, actor, weapon);
                        e.vcall(weapon + 0x9c, 0x10, &args![]);
                        let has_extras = item != 0 && e.mem.u32(item) != 0 && {
                            let head = e.mem.u32(item);
                            list_item(e, head) != 0
                        };
                        if has_extras {
                            let mut cursor = e.mem.u32(item);
                            while cursor != 0 && list_item(e, cursor) != 0 {
                                let list = list_item(e, cursor);
                                let health = weapon_list_health(e, weapon, list);
                                if health > 0.0 {
                                    let ammo_slot = weapon + 0x74;
                                    e.call(WORD_AT_4, &args![ammo_slot]);
                                    let form_health = e.call(GET_FORM_HEALTH, &args![weapon]).u32();
                                    fraction = (health as f64 / (form_health as u64 as f64)) as f32;
                                    let rating =
                                        rate_weapon(e, rating_actor, weapon, fraction, item, ammo);
                                    if best < rating {
                                        best = rating;
                                        replace_best_weapon(e, &mut result, weapon, Some(list));
                                    }
                                }
                                cursor = list_next(e, cursor);
                            }
                        } else {
                            let rating = rate_weapon(e, rating_actor, weapon, fraction, item, ammo);
                            if best < rating {
                                best = rating;
                                replace_best_weapon(e, &mut result, weapon, None);
                            }
                        }
                    }
                }
            }
            node = list_next(e, node);
        }
        let score = e.call(ROUND_SCORE, &args![best, 0u32, 0u32]).f32();
        e.mem.set_f32(out, score);
        if result != 0 {
            let form = e.call(WORD_AT_8, &args![result]).u32();
            if e.call(WEAPON_TYPE_GETTER, &args![form]).u32() as i32 == 10
                || e.call(WEAPON_TYPE_GETTER, &args![form]).u32() as i32 == 0xb
                || e.call(WEAPON_TYPE_GETTER, &args![form]).u32() as i32 == 0xd
            {
                let form = e.call(WORD_AT_8, &args![result]).u32();
                let count = fn_004c8f30(e, this, form);
                e.call(ITEM_SET_NUMBER, &args![result, count]);
            }
        }
        result
    })
}

// Translated from 004c8c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetWornItem` (Xbox PDB): a new `ItemChange` holding the
/// first worn extra list found in the changes for the slot `slot` (5: a
/// weapon form (0x28); else the biped model form must fill the slot, with
/// the biped model list of the form for slots 0, 1 and 10), with that list's
/// count as its number. With `armor_only` an item that is not armor (0x18) is
/// not returned (0 is). An entry with a negative number that is leveled gets
/// its worn list removed first (`SetWorn(0, 0, 1)`). The C++ exception frame
/// is not translated.
pub fn fn_004c8c10(e: &mut Engine, this: Ptr<InventoryChanges>, slot: i32, armor_only: u8) -> u32 {
    with_scope_guard(e, 0x175a, |e| {
        let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
        let result = 0u32;
        while node != 0 && list_item(e, node) != 0 {
            let item = list_item(e, node);
            if item != 0 {
                let mut extra_node = e.mem.u32(item);
                let mut worn_extra = 0u32;
                let mut found = false;
                if fn_004bcb70(e, Ptr::new(item)) && entry_number(e, item) < 0 {
                    item_change_set_worn(e, Ptr::new(item), 0, 0, 1);
                }
                while extra_node != 0 && list_item(e, extra_node) != 0 && !found {
                    worn_extra = list_item(e, extra_node);
                    if e.call(EXTRA_GET_WORN, &args![worn_extra, 0u32]).bool() {
                        if slot == 5 {
                            let form = e.call(WORD_AT_8, &args![item]).u32();
                            if form_type_of(e, form) == 0x28 {
                                found = true;
                            }
                        } else {
                            let form = e.call(WORD_AT_8, &args![item]).u32();
                            let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
                            if slot < 0 || (slot > 1 && slot != 10) {
                                if biped != 0
                                    && e.call(BIPED_FILLS_SLOT, &args![biped, slot, 0u32, 0u32])
                                        .bool()
                                {
                                    found = true;
                                }
                            } else {
                                let form = e.call(WORD_AT_8, &args![item]).u32();
                                let list = e.call(GET_FORM_AS_BIPED_MODEL_LIST, &args![form]).u32();
                                if biped != 0
                                    && e.call(BIPED_FILLS_SLOT, &args![biped, slot, 0u32, list])
                                        .bool()
                                {
                                    found = true;
                                }
                            }
                        }
                    }
                    extra_node = list_next(e, extra_node);
                }
                if found {
                    if armor_only != 0 {
                        let form = e.call(WORD_AT_8, &args![item]).u32();
                        if form_type_of(e, form) != 0x18 {
                            return result;
                        }
                    }
                    let worn_item = new_default_item_change(e);
                    let form = e.call(WORD_AT_8, &args![item]).u32();
                    e.mem.set_u32(worn_item + 8, form);
                    ensure_extra_list(e, worn_item);
                    let head = e.mem.u32(worn_item);
                    list_call_with_item(e, LIST_ADD, head, worn_extra);
                    let count = e.call(EXTRA_GET_COUNT, &args![worn_extra]).u16() as i16 as i32;
                    e.call(ITEM_SET_NUMBER, &args![worn_item, count]);
                    return worn_item;
                }
            }
            node = list_next(e, node);
        }
        result
    })
}

// Translated from 004c8f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetObjectCount` (Xbox PDB): how many of `form` the
/// owner has: the container's count (made positive; 0 without a container)
/// plus the changes' number for it; 1 when both are 0 and the changes have an
/// entry for it.
pub fn fn_004c8f30(e: &mut Engine, this: Ptr<InventoryChanges>, form: u32) -> i32 {
    let container = fn_004bffb0(e, this);
    let mut count = if container != 0 {
        e.call(CONTAINER_COUNT, &args![container, form]).i32()
    } else {
        0
    };
    if count < 0 {
        count = count.wrapping_mul(-1);
    }
    let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
    if entry != 0 {
        if count == 0 && entry_number(e, entry) == 0 {
            count = 1;
        } else {
            count = entry_number(e, entry).wrapping_add(count);
        }
    }
    count
}

/// The score of an armor with the health `health`: the rating
/// (`fn_004be060`, truncated to 16 bits) goes through
/// `CombatFormulas::CalcArmorRating(rating, health)` (the health is the word
/// the game pushed before calling `004be060`, which does not take it) and the
/// armor's damage threshold (`fn_004be180`) is added.
fn rate_armor(e: &mut Engine, armor: u32, health: f32) -> f32 {
    let rating = fn_004be060(e, Ptr::new(armor));
    let whole = truncate_to_i32(rating as f64) as u16;
    let calculated = e
        .call(CALC_ARMOR_RATING, &args![whole as u32, health])
        .f32();
    let threshold = fn_004be180(e, Ptr::new(armor));
    (threshold as f64 + calculated as f64) as f32
}

/// The health of an extra data list holding an armor: the health extra
/// (type 0x25) when it has one, else the form's own health.
fn armor_list_health(e: &mut Engine, armor: u32, extra: u32) -> f32 {
    weapon_list_health(e, armor, extra)
}

// Translated from 004c8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the best armor for the biped slot `slot` among the container's
/// objects and the changes', as a new `ItemChange` (with the extra list of
/// the best condition when it has some); 0 when there is none. An armor
/// (`TESObjectARMO` by dynamic cast) counts when its biped model fills the
/// slot (`FillsBipedSlot`, never for slot -1), it is not removed by the
/// changes (container pass) or is held by the changes and not by the container
/// (changes pass), and it scores strictly above `-FLT_MAX` and the best so
/// far (`rate_armor`, with the health of each extra list). With `use_worn`
/// the worn item of the slot that can not be taken off is returned as it is.
/// The first stack word is not read. The C++ exception frame is not
/// translated.
pub fn fn_004c8220(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    _unused_1: u32,
    slot: i32,
    use_worn: u8,
) -> u32 {
    with_scope_guard(e, 0x163e, |e| {
        let mut best = e.global::<f32>(NEGATIVE_FLOAT_MAX);
        let mut result = 0u32;
        if use_worn != 0 {
            let worn = fn_004c8c10(e, this, slot, 0);
            if worn != 0 {
                let list = e.mem.u32(worn);
                let first = e.mem.u32(list);
                if e.call(EXTRA_GET_CAN_NOT_WEAR, &args![first]).bool() {
                    return worn;
                }
            }
            delete_item_change(e, worn);
        }
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        // The container's armors.
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let object = list_item(e, node);
            let form = e.mem.u32(object + 4);
            let armor = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_TES_BOUND_OBJECT,
                        TYPE_TES_OBJECT_ARMO,
                        0u32
                    ],
                )
                .u32();
            let entry = inventory_changes_get_object_in_list(e, this, armor, 1, 0);
            let counts = entry == 0
                || {
                    let object = list_item(e, node);
                    entry_number(e, entry).wrapping_add(e.mem.u32(object) as i32) > 0
                }
                || {
                    let object = list_item(e, node);
                    (e.mem.u32(object) as i32) < 0
                };
            if counts
                && armor != 0
                && slot != -1
                && e.call(
                    BIPED_FILLS_SLOT,
                    &args![armor + ARMOR_BIPED_MODEL, slot, 0u32, 0u32],
                )
                .bool()
            {
                let has_extras = entry != 0 && e.mem.u32(entry) != 0 && {
                    let head = e.mem.u32(entry);
                    list_item(e, head) != 0
                };
                if !has_extras {
                    let health =
                        (e.call(GET_FORM_HEALTH, &args![armor]).u32() as u64 as f64) as f32;
                    let total = rate_armor(e, armor, health);
                    if best < total {
                        best = total;
                        replace_best_weapon(e, &mut result, armor, None);
                    }
                } else {
                    let mut cursor = e.mem.u32(entry);
                    while cursor != 0 && list_item(e, cursor) != 0 {
                        let list = list_item(e, cursor);
                        let health = armor_list_health(e, armor, list);
                        if health > 0.0 {
                            let total = rate_armor(e, armor, health);
                            if best < total {
                                best = total;
                                replace_best_weapon(e, &mut result, armor, Some(list));
                            }
                        }
                        cursor = list_next(e, cursor);
                    }
                }
            }
            node = list_next(e, node);
        }
        // The changes' armors.
        let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item_slot = list_item(e, node);
            let form = e.mem.u32(item_slot + 8);
            let armor = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_TES_BOUND_OBJECT,
                        TYPE_TES_OBJECT_ARMO,
                        0u32
                    ],
                )
                .u32();
            let item = list_item(e, node);
            if armor != 0 && entry_number(e, item) != 0 {
                let container = fn_004bffb0(e, this);
                if !e.call(CONTAINER_HAS_FORM, &args![container, armor]).bool()
                    && slot != -1
                    && e.call(
                        BIPED_FILLS_SLOT,
                        &args![armor + ARMOR_BIPED_MODEL, slot, 0u32, 0u32],
                    )
                    .bool()
                    && entry_number(e, item) >= 0
                {
                    let has_extras = item != 0 && e.mem.u32(item) != 0 && {
                        let head = e.mem.u32(item);
                        list_item(e, head) != 0
                    };
                    if !has_extras {
                        let health =
                            (e.call(GET_FORM_HEALTH, &args![armor]).u32() as u64 as f64) as f32;
                        let total = rate_armor(e, armor, health);
                        if best < total {
                            best = total;
                            replace_best_weapon(e, &mut result, armor, None);
                        }
                    } else {
                        let mut cursor = e.mem.u32(item);
                        while cursor != 0 && list_item(e, cursor) != 0 {
                            let list = list_item(e, cursor);
                            let health = armor_list_health(e, armor, list);
                            if health > 0.0 {
                                let total = rate_armor(e, armor, health);
                                if best < total {
                                    best = total;
                                    replace_best_weapon(e, &mut result, armor, Some(list));
                                }
                            }
                            cursor = list_next(e, cursor);
                        }
                    }
                }
            }
            node = list_next(e, node);
        }
        result
    })
}

// Translated from 004c8fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of different stacks the owner's inventory shows, plus one:
/// every object of the container that is not a leveled item (`TESLevItem` by
/// dynamic cast) and, unless `include_all` is set, is playable (a weapon
/// whose flag bit 7 at +0x100 is clear, ammunition `fn_004c94d0` accepts, a
/// biped model form whose `GetPlayable` is true) counts for the number of its
/// extra lists that differ (`GetAmountNonDefaultExtra`, one more for
/// `fn_004bccb0`, one more when the entry has more items than its lists
/// hold) or 1 when the changes do not hold it (or remove all of it); then
/// the entries of the changes that the container does not list (or that
/// hold a leveled list) count the same way. An entry with a negative number
/// (leveled lists excepted) is deleted from the changes and the second pass
/// restarts with the first pass' count. Returns the count plus one.
pub fn fn_004c8fd0(e: &mut Engine, this: Ptr<InventoryChanges>, include_all: u8) -> i32 {
    let mut count = 0i32;
    let container = fn_004bffb0(e, this);
    if container != 0 {
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        while node != 0 && list_item(e, node) != 0 {
            let mut object = list_item(e, node);
            let form = e.mem.u32(object + 4);
            let leveled_item = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_LEV_ITEM, 0u32],
                )
                .u32();
            let container = fn_004bffb0(e, this);
            let in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
            if include_all == 0 {
                let kind = form_type_of(e, form);
                if kind == 0x28 {
                    if !e.call(WEAPON_PLAYABLE, &args![form]).bool() {
                        object = 0;
                    }
                } else if kind == 0x29 {
                    if !fn_004c94d0(e, Ptr::new(form)) {
                        object = 0;
                    }
                } else {
                    let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
                    if biped != 0 && !e.call(BIPED_PLAYABLE, &args![biped]).bool() {
                        object = 0;
                    }
                }
            }
            if object != 0 && leveled_item == 0 {
                let form = e.mem.u32(object + 4);
                let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                let magnitude = in_container.wrapping_abs();
                let kept = entry != 0
                    && entry_number(e, entry).wrapping_add(magnitude) > 0
                    && !(in_container < 0 && entry_number(e, entry) <= in_container);
                if !kept {
                    if entry == 0 {
                        count += 1;
                    }
                } else if e.mem.u32(entry) == 0 || {
                    let head = e.mem.u32(entry);
                    list_item(e, head) == 0
                } {
                    count += 1;
                } else {
                    count += item_change_get_amount_non_default_extra(e, Ptr::new(entry));
                    if fn_004bccb0(e, Ptr::new(entry)) {
                        count += 1;
                    }
                    let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                    if listed < in_container.wrapping_add(entry_number(e, entry))
                        && {
                            let head = e.mem.u32(entry);
                            list_item(e, head) != 0
                        }
                        && !item_change_get_worn(e, Ptr::new(entry), 0)
                    {
                        let head = e.mem.u32(entry);
                        let first = list_item(e, head);
                        if !e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool() {
                            count += 1;
                        }
                    }
                }
            }
            node = list_next(e, node);
        }
    }
    let first_pass_count = count;
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    loop {
        if node == 0 || list_item(e, node) == 0 {
            return count + 1;
        }
        let entry = list_item(e, node);
        let mut advance = true;
        let mut listed_here = true;
        if entry != 0 {
            let list = e.mem.u32(entry);
            let leveled_head = list != 0 && {
                let first = list_item(e, list);
                first != 0 && e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool()
            };
            if leveled_head {
                listed_here = true;
            } else {
                let container = fn_004bffb0(e, this);
                if container != 0 {
                    let form = e.mem.u32(entry + 8);
                    let container = fn_004bffb0(e, this);
                    if e.call(CONTAINER_HAS_FORM, &args![container, form]).bool() {
                        listed_here = false;
                    }
                }
            }
            if include_all == 0 {
                let form = e.call(WORD_AT_8, &args![entry]).u32();
                let kind = form_type_of(e, form);
                if kind == 0x28 {
                    let form = e.call(WORD_AT_8, &args![entry]).u32();
                    if !e.call(WEAPON_PLAYABLE, &args![form]).bool() {
                        listed_here = false;
                    }
                } else if kind == 0x29 {
                    let form = e.call(WORD_AT_8, &args![entry]).u32();
                    if !fn_004c94d0(e, Ptr::new(form)) {
                        listed_here = false;
                    }
                } else {
                    let form = e.call(WORD_AT_8, &args![entry]).u32();
                    let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
                    if biped != 0 {
                        listed_here = e.call(BIPED_PLAYABLE, &args![biped]).bool();
                    }
                }
            }
        }
        if entry != 0 && entry_number(e, entry) > 0 && listed_here {
            count += item_change_get_amount_non_default_extra(e, Ptr::new(entry));
            if fn_004bccb0(e, Ptr::new(entry)) {
                count += 1;
            }
            if item_change_get_extra_total_count(e, Ptr::new(entry), false) < entry_number(e, entry)
            {
                count += 1;
            }
            if entry_number(e, entry) < 0 {
                let list = e.mem.u32(entry);
                let leveled = list != 0 && {
                    let first = list_item(e, list);
                    first != 0 && e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool()
                };
                if !leveled {
                    let changes = e.get(this, InventoryChanges::pListofChanges).addr();
                    list_call_with_item(e, LIST_REMOVE, changes, entry);
                    item_change_delete_all_extra(e, Ptr::new(entry));
                    delete_item_change(e, entry);
                    count = first_pass_count;
                    node = e.get(this, InventoryChanges::pListofChanges).addr();
                    advance = false;
                }
            }
        }
        if advance {
            node = list_next(e, node);
        }
    }
}

// Translated from 004c94d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 of the flag byte at +0xac of an ammunition form is clear (the
/// ammunition is playable).
pub fn fn_004c94d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0xac) & 2 == 0
}

// Translated from 004c94f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `index`-th item stack the owner's inventory shows, as a new
/// `ItemChange` (the stacks are numbered like `fn_004c8fd0` counts them: one
/// per extra list that is not the default one, plus one for the rest);
/// 0 when `index` is past the end. The container's objects are walked first
/// (the list `00482b00` makes of them; a quest note form 0x34 is skipped, and
/// so is an object the changes remove, or that has a leveled list), then the
/// entries of the changes that hold more than the container (their negative
/// container count made positive on the worn list). The C++ exception frame
/// is not translated.
pub fn fn_004c94f0(e: &mut Engine, this: Ptr<InventoryChanges>, index: i32) -> u32 {
    with_scope_guard(e, 0x187f, |e| {
        let mut running = 0i32;
        let mut result = 0u32;
        let mut object_list = 0u32;
        if fn_004bffb0(e, this) != 0 {
            let container = fn_004bffb0(e, this);
            object_list = e.call(CONTAINER_OBJECT_COPY, &args![container]).u32();
        }
        let container = fn_004bffb0(e, this);
        let mut node = object_list;
        while node != 0 && list_item(e, node) != 0 && result == 0 {
            let object = list_item(e, node);
            let mut note = 0u32;
            let form = e.mem.u32(object + 4);
            if form_type_of(e, form) == 0x34 {
                note = e.mem.u32(object + 4);
            }
            if object != 0 && note == 0 {
                let form = e.mem.u32(object + 4);
                let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                let count = e.call(CONTAINER_COUNT, &args![container, form]).i32();
                if object != 0 && entry != 0 && count <= 0 {
                    result = 0;
                } else if (count < 0 && entry != 0)
                    || (entry != 0 && entry_number(e, entry).wrapping_add(count) <= 0)
                {
                    result = 0;
                } else if entry != 0
                    && e.mem.u32(entry) != 0
                    && {
                        let head = e.mem.u32(entry);
                        list_item(e, head) != 0
                    }
                    && {
                        let head = e.mem.u32(entry);
                        let first = list_item(e, head);
                        e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool()
                    }
                {
                    result = 0;
                } else if object != 0
                    && running == index
                    && (entry == 0
                        || item_change_get_amount_non_default_extra(e, Ptr::new(entry)) == 0)
                {
                    result = new_default_item_change(e);
                    if entry != 0 {
                        if count < 0 {
                            let number = count.wrapping_add(entry_number(e, entry));
                            set_entry_number(e, result, number);
                        } else {
                            let listed =
                                item_change_get_extra_total_count(e, Ptr::new(entry), false);
                            let number = count.wrapping_add(entry_number(e, entry)) - listed;
                            set_entry_number(e, result, number);
                        }
                    } else if count < 0 {
                        set_entry_number(e, result, count.wrapping_mul(-1));
                    } else {
                        set_entry_number(e, result, count);
                    }
                    let form = e.mem.u32(object + 4);
                    e.call(ITEM_SET_FORM, &args![result, form]);
                    if entry != 0 {
                        let copy_lists = !(item_change_get_script(e, Ptr::new(entry)) == 0
                            && item_change_get_hot_key(e, Ptr::new(entry)) < 0
                            && item_change_get_item_ownership(e, Ptr::new(entry)) as i32
                                == entry_number(e, result));
                        if copy_lists && e.mem.u32(entry) != 0 && {
                            let head = e.mem.u32(entry);
                            e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0
                        } {
                            if e.mem.u32(result) == 0 {
                                let node = new_list_node(e);
                                e.mem.set_u32(result, node);
                            }
                            let mut copy = e.mem.u32(entry);
                            while copy != 0 && list_item(e, copy) != 0 {
                                let head = e.mem.u32(result);
                                e.call(LIST_ADD, &args![head, copy]);
                                copy = list_next(e, copy);
                            }
                        }
                    }
                } else if entry == 0 {
                    running += 1;
                } else {
                    let mut non_default = 0i32;
                    if e.mem.u32(entry) != 0 {
                        non_default = item_change_get_amount_non_default_extra(e, Ptr::new(entry));
                    }
                    if index < non_default + running {
                        let mut cursor = e.mem.u32(entry);
                        while cursor != 0 && list_item(e, cursor) != 0 && result == 0 {
                            let list = list_item(e, cursor);
                            if !e
                                .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                .bool()
                            {
                                if running == index {
                                    let mut held =
                                        e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                                    let remaining = entry_number(e, entry).wrapping_add(count);
                                    if held > remaining {
                                        held = remaining;
                                    }
                                    let entry_form = e.mem.u32(entry + 8);
                                    result = new_item_change(e, entry_form, held as u32);
                                    let head = e.mem.u32(result);
                                    list_call_with_item(e, LIST_ADD, head, list);
                                }
                                running += 1;
                            }
                            cursor = list_next(e, cursor);
                        }
                    } else {
                        running += item_change_get_amount_non_default_extra(e, Ptr::new(entry));
                        let counts_as_one = entry_counts_as_one(e, entry);
                        if !counts_as_one
                            && item_change_get_extra_total_count(e, Ptr::new(entry), true)
                                < count.wrapping_add(entry_number(e, entry))
                            && (!fn_004bcb70(e, Ptr::new(entry)) || entry_number(e, entry) <= count)
                        {
                            if running == index {
                                result = new_default_item_change(e);
                                let listed =
                                    item_change_get_extra_total_count(e, Ptr::new(entry), false);
                                let number = count.wrapping_add(entry_number(e, entry)) - listed;
                                set_entry_number(e, result, number);
                                let form = e.mem.u32(object + 4);
                                e.call(ITEM_SET_FORM, &args![result, form]);
                                if item_change_get_script(e, Ptr::new(entry)) != 0
                                    && e.mem.u32(entry) != 0
                                    && {
                                        let head = e.mem.u32(entry);
                                        e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0
                                    }
                                {
                                    if e.mem.u32(result) == 0 {
                                        let node = new_list_node(e);
                                        e.mem.set_u32(result, node);
                                    }
                                    let mut copy = e.mem.u32(entry);
                                    while copy != 0 && list_item(e, copy) != 0 {
                                        let head = e.mem.u32(result);
                                        e.call(LIST_ADD, &args![head, copy]);
                                        copy = list_next(e, copy);
                                    }
                                }
                            } else {
                                running += 1;
                            }
                        }
                    }
                }
            }
            node = list_next(e, node);
        }
        if object_list != 0 {
            e.call(LIST_CLEAR, &args![object_list]);
            e.call(LIST_DESTROY, &args![object_list, 1u32]);
        }
        if result == 0 {
            let mut node = e.mem.u32(this.addr());
            while node != 0 && list_item(e, node) != 0 && result == 0 {
                let item = list_item(e, node);
                let mut in_container = 0i32;
                if fn_004bffb0(e, this) != 0 && item != 0 {
                    let container = fn_004bffb0(e, this);
                    let form = e.mem.u32(item + 8);
                    in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
                }
                if in_container < 0 && item_change_get_worn(e, Ptr::new(item), 0) {
                    let head = e.mem.u32(item);
                    let list = list_item(e, head);
                    e.call(EXTRA_SET_COUNT, &args![list, in_container.wrapping_mul(-1)]);
                }
                if item != 0 && (entry_number(e, item) > 0 || in_container < 0) {
                    let head = e.mem.u32(item);
                    let leveled = head != 0 && list_item(e, head) != 0 && {
                        let first = list_item(e, head);
                        e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool()
                    };
                    let held_by_container = !leveled && in_container >= 0 && {
                        let container = fn_004bffb0(e, this);
                        container != 0 && {
                            let form = e.mem.u32(item + 8);
                            let container = fn_004bffb0(e, this);
                            e.call(CONTAINER_HAS_FORM, &args![container, form]).bool()
                        }
                    };
                    if !held_by_container {
                        let mut in_lists = 0i32;
                        let mut cursor = e.mem.u32(item);
                        while cursor != 0 && list_item(e, cursor) != 0 {
                            let list = list_item(e, cursor);
                            if !e
                                .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                .bool()
                            {
                                let held =
                                    e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                                if held > 0 {
                                    in_lists += held;
                                }
                            }
                            cursor = list_next(e, cursor);
                        }
                        let head = e.mem.u32(item);
                        if head != 0
                            && list_item(e, head) != 0
                            && item_change_get_amount_non_default_extra(e, Ptr::new(item)) + running
                                > index
                        {
                            let mut cursor = e.mem.u32(item);
                            while cursor != 0 && list_item(e, cursor) != 0 && result == 0 {
                                let list = list_item(e, cursor);
                                if running == index {
                                    if !e
                                        .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                        .bool()
                                    {
                                        let held = e.call(EXTRA_GET_COUNT, &args![list]).u16()
                                            as i16
                                            as i32;
                                        if held > 0 {
                                            let item_form = e.mem.u32(item + 8);
                                            result = new_item_change(e, item_form, held as u32);
                                            let head = e.mem.u32(result);
                                            list_call_with_item(e, LIST_ADD, head, list);
                                        }
                                    }
                                } else if !e
                                    .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                    .bool()
                                    && e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 > 0
                                {
                                    running += 1;
                                }
                                cursor = list_next(e, cursor);
                            }
                        } else if entry_number(e, item) == in_lists {
                            running += item_change_get_amount_non_default_extra(e, Ptr::new(item));
                        } else {
                            if entry_number(e, item) > in_lists {
                                let non_default =
                                    item_change_get_amount_non_default_extra(e, Ptr::new(item));
                                if non_default > 0 {
                                    running += non_default;
                                }
                            }
                            if running == index
                                && result == 0
                                && entry_number(e, item) - in_lists > 0
                            {
                                let item_form = e.mem.u32(item + 8);
                                result = new_item_change(e, item_form, 0);
                                let number = entry_number(e, item) - in_lists;
                                set_entry_number(e, result, number);
                                if e.mem.u32(item) != 0 {
                                    let mut cursor = e.mem.u32(item);
                                    while cursor != 0 && list_item(e, cursor) != 0 {
                                        let list = list_item(e, cursor);
                                        if !e.call(EXTRA_GET_WORN, &args![list, 0u32]).bool()
                                            && e.call(
                                                EXTRA_IS_DEFAULT_FOR_CONTAINER,
                                                &args![list, 1u32],
                                            )
                                            .bool()
                                        {
                                            let head = e.mem.u32(result);
                                            list_call_with_item(e, LIST_ADD_TAIL, head, list);
                                        }
                                        cursor = list_next(e, cursor);
                                    }
                                }
                            } else {
                                running += 1;
                            }
                        }
                    }
                }
                node = list_next(e, node);
            }
        }
        result
    })
}

/// The flag the stack counting code computes for a worn entry without extra
/// lists: true when it counts as one stack of its own (its extra lists hold
/// nothing, it is worn, and its number is not above 0, or its form is
/// ammunition (0x29) or a weapon of type 10, 11 or 13).
fn entry_counts_as_one(e: &mut Engine, entry: u32) -> bool {
    let mut counts_as_one = false;
    let mut weapon = 0u32;
    let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
    if form_type_of(e, entry_form) == 0x28 {
        weapon = e.call(WORD_AT_8, &args![entry]).u32();
    }
    if item_change_get_extra_total_count(e, Ptr::new(entry), false) == 0
        && item_change_get_worn(e, Ptr::new(entry), 0)
    {
        counts_as_one = true;
        if entry_number(e, entry) > 0 {
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            let kind = form_type_of(e, entry_form);
            if kind != 0x29
                && (weapon == 0
                    || (e.call(WEAPON_TYPE_GETTER, &args![weapon]).u32() as i32 != 10
                        && e.call(WEAPON_TYPE_GETTER, &args![weapon]).u32() as i32 != 0xb
                        && e.call(WEAPON_TYPE_GETTER, &args![weapon]).u32() as i32 != 0xd))
            {
                counts_as_one = false;
            }
        }
    }
    counts_as_one
}

/// Fills `result` (a new `ItemChange` for the container object `object`)
/// from the changes' `entry` for it: the number (`count` plus the entry's,
/// less the lists' when positive; `-count` or `count` without an entry),
/// the form and, when the entry has a script, a hot key or another owner,
/// copies of its extra lists.
fn fill_item_from_entry(e: &mut Engine, result: u32, entry: u32, count: i32, object: u32) {
    if entry != 0 {
        if count < 0 {
            let number = count.wrapping_add(entry_number(e, entry));
            set_entry_number(e, result, number);
        } else {
            let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
            let number = count.wrapping_add(entry_number(e, entry)) - listed;
            set_entry_number(e, result, number);
        }
    } else if count < 0 {
        set_entry_number(e, result, count.wrapping_mul(-1));
    } else {
        set_entry_number(e, result, count);
    }
    let form = e.mem.u32(object + 4);
    e.call(ITEM_SET_FORM, &args![result, form]);
    if entry != 0 {
        let copy_lists = !(item_change_get_script(e, Ptr::new(entry)) == 0
            && item_change_get_hot_key(e, Ptr::new(entry)) < 0
            && item_change_get_item_ownership(e, Ptr::new(entry)) as i32
                == entry_number(e, result));
        if copy_lists && e.mem.u32(entry) != 0 && {
            let head = e.mem.u32(entry);
            e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0
        } {
            if e.mem.u32(result) == 0 {
                let node = new_list_node(e);
                e.mem.set_u32(result, node);
            }
            let mut copy = e.mem.u32(entry);
            while copy != 0 && list_item(e, copy) != 0 {
                let head = e.mem.u32(result);
                e.call(LIST_ADD, &args![head, copy]);
                copy = list_next(e, copy);
            }
        }
    }
}

// Translated from 004ca1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the fast inventory iterator
/// (`OEI_Fast_InventoryIterator`): its list of container objects (the word
/// at +0) is cleared, deleted and the word set to 0.
pub fn fn_004ca1a0(e: &mut Engine, this: u32) {
    let list = e.mem.u32(this);
    if list != 0 {
        e.call(LIST_CLEAR, &args![list]);
        let list = e.mem.u32(this);
        if list != 0 {
            e.call(LIST_DESTROY, &args![list, 1u32]);
        }
        e.mem.set_u32(this, 0);
    }
}

// Translated from 004ca200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::StartOEIFastInventoryIteration` (Xbox PDB): a new
/// iterator (0x28 bytes) for `fn_004ca330`. Its words: +0 the copy of the
/// container's object list (`00482b00`; 0 without a container) and +4 the
/// node the container pass is at (the same list); +8 0; +0xc the item
/// being returned; +0x10 the container; +0x14 the node of the changes' list
/// the second pass is at; +0x18 the pass (0 or 1); +0x1c, +0x20 and +0x24
/// the extra list nodes the passes are in the middle of. The C++ exception
/// frame is not translated.
pub fn inventory_changes_start_oei_fast_inventory_iteration(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
) -> u32 {
    with_scope_guard(e, 0x19b2, |e| {
        let memory = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
        let iterator = if memory == 0 {
            0
        } else {
            e.call(NODE_ITEM_ADDRESS, &args![memory]).u32()
        };
        e.mem.set_u32(iterator, 0);
        e.mem.set_u32(iterator + 4, 0);
        if fn_004bffb0(e, this) != 0 {
            let container = fn_004bffb0(e, this);
            let copy = e.call(CONTAINER_OBJECT_COPY, &args![container]).u32();
            e.mem.set_u32(iterator, copy);
        }
        e.mem.set_u32(iterator + 8, 0);
        e.mem.set_u32(iterator + 0xc, 0);
        let container = fn_004bffb0(e, this);
        e.mem.set_u32(iterator + 0x10, container);
        let first = e.mem.u32(iterator);
        e.mem.set_u32(iterator + 4, first);
        e.mem.set_u32(iterator + 0x18, 0);
        e.mem.set_u32(iterator + 0x1c, 0);
        e.mem.set_u32(iterator + 0x24, 0);
        e.mem.set_u32(iterator + 0x20, 0);
        iterator
    })
}

// Translated from 004ca330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetNextOEIFastInventoryItem` (Xbox PDB): the next item
/// stack of the owner's inventory for the iterator `iterator`, as a new
/// `ItemChange` (0 when the iteration is over). It is `fn_004c94f0` made
/// resumable: pass 0 walks the container's objects (iterator +4, the extra
/// list node in +0x1c), pass 1 (iterator +0x18) the entries of the changes
/// (+0x14, extra list nodes in +0x20 and +0x24); the result is stored in +0xc.
/// A stack is the extra lists that are not the default one, one each, then
/// the rest of the number as one stack. The C++ exception frame is not
/// translated.
pub fn inventory_changes_get_next_oei_fast_inventory_item(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    iterator: u32,
) -> u32 {
    with_scope_guard(e, 0x19c9, |e| {
        e.mem.set_u32(iterator + 0xc, 0);
        if e.mem.u32(iterator + 0x18) == 0 {
            loop {
                let node = e.mem.u32(iterator + 4);
                if node == 0 || list_item(e, node) == 0 || e.mem.u32(iterator + 0xc) != 0 {
                    break;
                }
                let object = list_item(e, node);
                let mut advance = true;
                let mut note = 0u32;
                let form = e.mem.u32(object + 4);
                if form_type_of(e, form) == 0x34 {
                    note = e.mem.u32(object + 4);
                }
                if object != 0 && note == 0 {
                    let form = e.mem.u32(object + 4);
                    let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                    let container = e.mem.u32(iterator + 0x10);
                    let count = e.call(CONTAINER_COUNT, &args![container, form]).i32();
                    if object != 0 && entry != 0 && count <= 0 {
                        e.mem.set_u32(iterator + 0xc, 0);
                    } else if (count < 0 && entry != 0)
                        || (entry != 0 && entry_number(e, entry).wrapping_add(count) <= 0)
                    {
                        e.mem.set_u32(iterator + 0xc, 0);
                    } else if entry != 0
                        && e.mem.u32(entry) != 0
                        && {
                            let head = e.mem.u32(entry);
                            list_item(e, head) != 0
                        }
                        && {
                            let head = e.mem.u32(entry);
                            let first = list_item(e, head);
                            e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool()
                        }
                    {
                        e.mem.set_u32(iterator + 0xc, 0);
                    } else if object != 0
                        && (entry == 0
                            || item_change_get_amount_non_default_extra(e, Ptr::new(entry)) == 0)
                    {
                        let created = new_default_item_change(e);
                        e.mem.set_u32(iterator + 0xc, created);
                        fill_item_from_entry(e, created, entry, count, object);
                    } else if entry != 0 {
                        if e.mem.u32(entry) != 0 {
                            item_change_get_amount_non_default_extra(e, Ptr::new(entry));
                        }
                        if e.mem.u32(iterator + 0x1c) == 0 {
                            let head = e.mem.u32(entry);
                            e.mem.set_u32(iterator + 0x1c, head);
                        }
                        loop {
                            let cursor = e.mem.u32(iterator + 0x1c);
                            if cursor == 0
                                || list_item(e, cursor) == 0
                                || e.mem.u32(iterator + 0xc) != 0
                            {
                                break;
                            }
                            let list = list_item(e, cursor);
                            if !e
                                .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                .bool()
                            {
                                let mut held =
                                    e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                                let remaining = entry_number(e, entry).wrapping_add(count);
                                if remaining < held {
                                    held = remaining;
                                }
                                let entry_form = e.mem.u32(entry + 8);
                                let created = new_item_change(e, entry_form, held as u32);
                                e.mem.set_u32(iterator + 0xc, created);
                                let head = e.mem.u32(created);
                                list_call_with_item(e, LIST_ADD, head, list);
                            }
                            let next = list_next(e, cursor);
                            e.mem.set_u32(iterator + 0x1c, next);
                        }
                        let cursor = e.mem.u32(iterator + 0x1c);
                        advance = cursor == 0 || list_item(e, cursor) == 0;
                    } else {
                        // Not reachable: `entry` is nonzero here; the game's
                        // code still has this stack counting branch.
                        item_change_get_amount_non_default_extra(e, Ptr::new(entry));
                        let counts_as_one = entry_counts_as_one(e, entry);
                        if !counts_as_one
                            && item_change_get_extra_total_count(e, Ptr::new(entry), true)
                                < count.wrapping_add(entry_number(e, entry))
                            && (!fn_004bcb70(e, Ptr::new(entry)) || entry_number(e, entry) <= count)
                        {
                            let created = new_default_item_change(e);
                            e.mem.set_u32(iterator + 0xc, created);
                            let listed =
                                item_change_get_extra_total_count(e, Ptr::new(entry), false);
                            let number = count.wrapping_add(entry_number(e, entry)) - listed;
                            set_entry_number(e, created, number);
                            let form = e.mem.u32(object + 4);
                            e.call(ITEM_SET_FORM, &args![created, form]);
                            if item_change_get_script(e, Ptr::new(entry)) != 0
                                && e.mem.u32(entry) != 0
                                && {
                                    let head = e.mem.u32(entry);
                                    e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0
                                }
                            {
                                if e.mem.u32(created) == 0 {
                                    let node = new_list_node(e);
                                    e.mem.set_u32(created, node);
                                }
                                let mut copy = e.mem.u32(entry);
                                while copy != 0 && list_item(e, copy) != 0 {
                                    let head = e.mem.u32(created);
                                    e.call(LIST_ADD, &args![head, copy]);
                                    copy = list_next(e, copy);
                                }
                            }
                        }
                    }
                }
                if advance {
                    let node = e.mem.u32(iterator + 4);
                    let next = list_next(e, node);
                    e.mem.set_u32(iterator + 4, next);
                }
            }
            if e.mem.u32(iterator + 0xc) == 0 {
                e.mem.set_u32(iterator + 0x18, 1);
                let changes = e.get(this, InventoryChanges::pListofChanges).addr();
                e.mem.set_u32(iterator + 0x14, changes);
            }
        }
        if e.mem.u32(iterator + 0x18) == 1 && e.mem.u32(iterator + 0xc) == 0 {
            loop {
                let node = e.mem.u32(iterator + 0x14);
                if node == 0 || list_item(e, node) == 0 || e.mem.u32(iterator + 0xc) != 0 {
                    break;
                }
                let mut advance = true;
                let item = list_item(e, node);
                let mut in_container = 0i32;
                if fn_004bffb0(e, this) != 0 && item != 0 {
                    let container = fn_004bffb0(e, this);
                    let form = e.mem.u32(item + 8);
                    in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
                }
                if in_container < 0 && item_change_get_worn(e, Ptr::new(item), 0) {
                    let head = e.mem.u32(item);
                    let list = list_item(e, head);
                    e.call(EXTRA_SET_COUNT, &args![list, in_container.wrapping_mul(-1)]);
                }
                if item != 0 && (entry_number(e, item) > 0 || in_container < 0) {
                    let head = e.mem.u32(item);
                    let leveled = head != 0 && list_item(e, head) != 0 && {
                        let first = list_item(e, head);
                        e.call(EXTRA_HAS_LEVELED_ITEM, &args![first]).bool()
                    };
                    let held_by_container = !leveled && in_container >= 0 && {
                        let container = fn_004bffb0(e, this);
                        container != 0 && {
                            let form = e.mem.u32(item + 8);
                            let container = fn_004bffb0(e, this);
                            e.call(CONTAINER_HAS_FORM, &args![container, form]).bool()
                        }
                    };
                    if !held_by_container {
                        let mut in_lists = 0i32;
                        if e.mem.u32(iterator + 0x20) == 0 {
                            let head = e.mem.u32(item);
                            e.mem.set_u32(iterator + 0x20, head);
                        }
                        loop {
                            let cursor = e.mem.u32(iterator + 0x20);
                            if cursor == 0 || list_item(e, cursor) == 0 {
                                break;
                            }
                            let list = list_item(e, cursor);
                            if !e
                                .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                .bool()
                            {
                                let held =
                                    e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                                if held > 0 {
                                    in_lists += held;
                                }
                            }
                            let next = list_next(e, cursor);
                            e.mem.set_u32(iterator + 0x20, next);
                        }
                        e.mem.set_u32(iterator + 0x20, 0);
                        let head = e.mem.u32(item);
                        if head != 0 && list_item(e, head) != 0 {
                            item_change_get_amount_non_default_extra(e, Ptr::new(item));
                            if e.mem.u32(iterator + 0x24) == 0 {
                                let head = e.mem.u32(item);
                                e.mem.set_u32(iterator + 0x24, head);
                            }
                            loop {
                                let cursor = e.mem.u32(iterator + 0x24);
                                if cursor == 0
                                    || list_item(e, cursor) == 0
                                    || e.mem.u32(iterator + 0xc) != 0
                                {
                                    break;
                                }
                                let list = list_item(e, cursor);
                                if !e
                                    .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![list, 0u32])
                                    .bool()
                                {
                                    let held =
                                        e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                                    if held > 0 {
                                        let item_form = e.mem.u32(item + 8);
                                        let created = new_item_change(e, item_form, held as u32);
                                        e.mem.set_u32(iterator + 0xc, created);
                                        let head = e.mem.u32(created);
                                        list_call_with_item(e, LIST_ADD, head, list);
                                    }
                                }
                                let next = list_next(e, cursor);
                                e.mem.set_u32(iterator + 0x24, next);
                            }
                            let cursor = e.mem.u32(iterator + 0x24);
                            advance = cursor == 0 || list_item(e, cursor) == 0;
                        } else if entry_number(e, item) == in_lists {
                            item_change_get_amount_non_default_extra(e, Ptr::new(item));
                        }
                        if e.mem.u32(iterator + 0xc) == 0 {
                            if entry_number(e, item) > in_lists {
                                item_change_get_amount_non_default_extra(e, Ptr::new(item));
                            }
                            if e.mem.u32(iterator + 0xc) == 0
                                && entry_number(e, item) - in_lists > 0
                            {
                                let item_form = e.mem.u32(item + 8);
                                let created = new_item_change(e, item_form, 0);
                                e.mem.set_u32(iterator + 0xc, created);
                                let number = entry_number(e, item) - in_lists;
                                set_entry_number(e, created, number);
                                if e.mem.u32(item) != 0 {
                                    let mut cursor = e.mem.u32(item);
                                    while cursor != 0 && list_item(e, cursor) != 0 {
                                        let list = list_item(e, cursor);
                                        if !e.call(EXTRA_GET_WORN, &args![list, 0u32]).bool()
                                            && e.call(
                                                EXTRA_IS_DEFAULT_FOR_CONTAINER,
                                                &args![list, 1u32],
                                            )
                                            .bool()
                                        {
                                            let head = e.mem.u32(created);
                                            list_call_with_item(e, LIST_ADD_TAIL, head, list);
                                        }
                                        cursor = list_next(e, cursor);
                                    }
                                }
                            }
                        }
                    }
                }
                if advance {
                    let node = e.mem.u32(iterator + 0x14);
                    let next = list_next(e, node);
                    e.mem.set_u32(iterator + 0x14, next);
                }
            }
        }
        e.mem.u32(iterator + 0xc)
    })
}

/// Whether `form` is a food: an `IngredientItem` or `AlchemyItem` (dynamic
/// casts from `TESBoundObject`) whose sub-object at +0x3c answers true to
/// virtual 0x4. Returns the cast result that answered, else 0. `alchemy_first`
/// is the order the two casts are asked in (the container pass asks the
/// `AlchemyItem` first, the changes pass the `IngredientItem`).
fn food_form(e: &mut Engine, form: u32, alchemy_first: bool) -> u32 {
    let ingredient = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_INGREDIENT_ITEM,
                0u32
            ],
        )
        .u32();
    let mut alchemy = 0u32;
    if ingredient == 0 {
        alchemy = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_ALCHEMY_ITEM, 0u32],
            )
            .u32();
    }
    let selected = if ingredient != 0 { ingredient } else { alchemy };
    let (first, second) = if alchemy_first {
        (alchemy, ingredient)
    } else {
        (ingredient, alchemy)
    };
    if first != 0 && e.vcall(first + 0x3c, 4, &args![]).bool() {
        return selected;
    }
    if second != 0 && e.vcall(second + 0x3c, 4, &args![]).bool() {
        return selected;
    }
    0
}

// Translated from 004cafe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetBestFood` (Xbox PDB): the first food of the
/// container's objects (an ingredient or alchemy item whose +0x3c
/// sub-object's virtual 4 says it is food) that the changes do not remove
/// completely: the entry of the changes for it, or a new `ItemChange` for it
/// (number 0) without one. Failing that the first entry of the changes that
/// is a food, has a positive number and is not listed by the container.
/// 0 when there is none. The C++ exception frame is not translated.
pub fn inventory_changes_get_best_food(e: &mut Engine, this: Ptr<InventoryChanges>) -> u32 {
    with_scope_guard(e, 0x1afa, |e| {
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let object = list_item(e, node);
            let form = e.mem.u32(object + 4);
            // The container pass asks the alchemy item first.
            let selected = food_form(e, form, true);
            if selected != 0 {
                let entry = inventory_changes_get_object_in_list(e, this, selected, 1, 0);
                let counted = entry == 0 || {
                    let object = list_item(e, node);
                    entry_number(e, entry).wrapping_add(e.mem.u32(object) as i32) != 0
                };
                if counted {
                    if entry == 0 {
                        return new_item_change(e, selected, 0);
                    }
                    return entry;
                }
            }
            node = list_next(e, node);
        }
        let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item_slot = list_item(e, node);
            let form = e.mem.u32(item_slot + 8);
            let selected = food_form(e, form, false);
            if selected != 0 {
                let container = fn_004bffb0(e, this);
                if !e
                    .call(CONTAINER_HAS_FORM, &args![container, selected])
                    .bool()
                {
                    let item = list_item(e, node);
                    if entry_number(e, item) > 0 {
                        return item;
                    }
                }
            }
            node = list_next(e, node);
        }
        0
    })
}

// Translated from 004cb320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetGoldAmount` (Xbox PDB): the gold the owner has:
/// every gold object (`TESContainer::IsGold`) of the container adds its
/// count and the changes' number for it (or, when the changes do not have
/// it, only the count, the total made positive when it went negative); then
/// the entries of the changes that are gold and not listed by the container
/// (all of them without a container) with a positive number add that
/// number.
pub fn inventory_changes_get_gold_amount(e: &mut Engine, this: Ptr<InventoryChanges>) -> i32 {
    let mut gold = 0i32;
    let container = fn_004bffb0(e, this);
    let mut node = if container != 0 {
        e.call(CONTAINER_OBJECT_LIST, &args![container]).u32()
    } else {
        0
    };
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        if form != 0 && e.call(IS_GOLD, &args![form]).bool() {
            let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            let object = list_item(e, node);
            let count = e.mem.u32(object) as i32;
            if entry != 0 {
                gold = entry_number(e, entry)
                    .wrapping_add(count)
                    .wrapping_add(gold);
            } else {
                gold = gold.wrapping_add(count);
                if gold < 0 {
                    gold = gold.wrapping_mul(-1);
                }
            }
        }
        node = list_next(e, node);
    }
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item_slot = list_item(e, node);
        let form = e.mem.u32(item_slot + 8);
        if form != 0 && e.call(IS_GOLD, &args![form]).bool() {
            let item = list_item(e, node);
            let counts = container == 0 || {
                !e.call(CONTAINER_HAS_FORM, &args![container, form]).bool()
                    && entry_number(e, item) > 0
            };
            if counts {
                gold = entry_number(e, item).wrapping_add(gold);
            }
        }
        node = list_next(e, node);
    }
    gold
}

// Translated from 004cb4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::RemoveGold` (Xbox PDB): removes `count` gold from the
/// owner (the first gold object of the container, else of the changes) with
/// `fn_004c37d0(actor, gold, no owner, count, no extra list, no drop,
/// target, ..., delete extras)`.
pub fn inventory_changes_remove_gold(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    count: i32,
    target: u32,
) {
    let container = fn_004bffb0(e, this);
    let mut node = if container != 0 {
        e.call(CONTAINER_OBJECT_LIST, &args![container]).u32()
    } else {
        0
    };
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        if form != 0 && e.call(IS_GOLD, &args![form]).bool() {
            fn_004c37d0(e, this, actor, form, 0, count, 0, 0, target, 0, 0, 1, 0, 0);
            return;
        }
        node = list_next(e, node);
    }
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item_slot = list_item(e, node);
        let form = e.mem.u32(item_slot + 8);
        if form != 0 && e.call(IS_GOLD, &args![form]).bool() {
            fn_004c37d0(e, this, actor, form, 0, count, 0, 0, target, 0, 0, 1, 0, 0);
            return;
        }
        node = list_next(e, node);
    }
}

/// The node of the changes' list the stolen items scan continues at: the
/// node saved before the current one was handled when it still follows it,
/// else the head of the list (the current one may have been removed).
fn reseat_node(e: &mut Engine, this: Ptr<InventoryChanges>, node: u32, saved_next: u32) -> u32 {
    let next = list_next(e, node);
    if saved_next == next {
        saved_next
    } else {
        e.get(this, InventoryChanges::pListofChanges).addr()
    }
}

// Translated from 004cb5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::RemoveStolenItems` (Xbox PDB): takes the items of the
/// changes whose extra lists carry an owner other than `actor` (the owner
/// must be `owner_filter` when that is not 0; with a null filter and a
/// `target` that is an actor, the target's base form becomes the filter) from
/// the owner and hands them to `target`, one extra list at a time. An
/// owner that is a faction or form of type 8 (an actor base) must be the
/// target's own base form, or the one the target is a member of
/// (`Actor::IsInFaction`), when the target is an actor. With a non-zero
/// filter a message is shown for each ("N name(s) removed", icon, pick up
/// sound, 2.0 seconds). Entries with a form whose virtual 0x94 is true,
/// non-positive numbers and empty lists are skipped. The C++ exception frame
/// is not translated.
pub fn inventory_changes_remove_stolen_items(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    target: u32,
    owner_filter: u32,
) {
    let mut owner_filter = owner_filter;
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    loop {
        if node == 0 || e.call(LIST_IS_EMPTY, &args![node]).bool() {
            return;
        }
        let saved_next = list_next(e, node);
        let item = list_item(e, node);
        let mut done = false;
        let skip = item == 0 || entry_number(e, item) <= 0 || {
            let form = e.call(WORD_AT_8, &args![item]).u32();
            e.vcall(form, 0x94, &args![]).bool()
        };
        if skip {
            node = reseat_node(e, this, node, saved_next);
            continue;
        }
        let mut extra_list = e.mem.u32(item);
        let taken_form = e.call(WORD_AT_8, &args![item]).u32();
        let mut victim = 0u32;
        if e.vcall(target, 0x100, &args![]).bool() {
            victim = target;
        }
        if extra_list == 0 || list_item(e, extra_list) == 0 {
            node = reseat_node(e, this, node, saved_next);
            continue;
        }
        while extra_list != 0 && list_item(e, extra_list) != 0 && !done {
            let following = list_next(e, extra_list);
            let extra = list_item(e, extra_list);
            let mut merge_all = false;
            let owner = e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32();
            let mut faction = 0u32;
            let mut actor_base = 0u32;
            if owner != 0 {
                if form_type_of(e, owner) == 8 {
                    actor_base = owner;
                } else {
                    faction = owner;
                }
            }
            if owner_filter == 0 || owner_filter == e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32()
            {
                if victim != 0 && owner_filter == 0 {
                    owner_filter = e.call(REFERENCE_BASE_FORM, &args![victim]).u32();
                }
                let owner = e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32();
                if owner != 0
                    && owner != actor
                    && (victim == 0
                        || e.call(REFERENCE_BASE_FORM, &args![victim]).u32() == faction
                        || e.call(ACTOR_IS_IN_FACTION, &args![victim, actor_base])
                            .bool())
                {
                    let list = e.mem.u32(item);
                    if e.call(LIST_COUNT_NONNULL, &args![list]).u32() <= 1 {
                        done = true;
                    } else {
                        merge_all = true;
                    }
                    if owner_filter != 0 {
                        announce_removed_item(e, taken_form, extra);
                    }
                    let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                    fn_004c37d0(
                        e, this, actor, taken_form, 0, held, extra, 0, target, 0, 0, 1, 0, 0,
                    );
                    node = e.get(this, InventoryChanges::pListofChanges).addr();
                }
            }
            if done {
                node = e.get(this, InventoryChanges::pListofChanges).addr();
                extra_list = 0;
            } else if merge_all {
                extra_list = e.mem.u32(item);
            } else {
                let next = list_next(e, extra_list);
                extra_list = if following == next {
                    following
                } else {
                    e.mem.u32(item)
                };
                if extra_list == 0 {
                    node = reseat_node(e, this, node, saved_next);
                }
            }
        }
    }
}

/// The message `fn_004cb5f0` shows for a removed item: "N name(s)" or
/// "name" in the text of a string object, shown with the item's icon path
/// (`Interface\Icons\<icon>`, formatted into a local buffer the game does
/// not use again) and the pick up sound of the form.
fn announce_removed_item(e: &mut Engine, form: u32, extra: u32) {
    e.with_stack(0x120, |e, frame| {
        let text_object = frame.addr();
        let buffer = frame.addr() + 0x10;
        e.call(STRING_CONSTRUCT, &args![text_object]);
        let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
        if count > 1 {
            let plural = e
                .call(STRING_OBJECT_TO_CHARS, &args![MESSAGE_WORD_ONE])
                .u32();
            let suffix = e
                .call(STRING_OBJECT_TO_CHARS, &args![MESSAGE_WORD_TWO])
                .u32();
            let name = e.call(FULL_NAME_OF_FORM, &args![form]).u32();
            e.call(
                FORMAT_STRING,
                &args![text_object, FORMAT_COUNT_NAME, count, name, suffix, plural],
            );
        } else {
            let plural = e
                .call(STRING_OBJECT_TO_CHARS, &args![MESSAGE_WORD_ONE])
                .u32();
            let name = e.call(FULL_NAME_OF_FORM, &args![form]).u32();
            e.call(
                FORMAT_STRING,
                &args![text_object, FORMAT_NAME, name, plural],
            );
        }
        let player = e.global::<u32>(PLAYER_GLOBAL);
        let icon = e.call(FORM_ICON_FOR, &args![form, player]).u32();
        e.call(
            FORMAT_BUFFER,
            &args![buffer, 0x104u32, FORMAT_PATH, ICON_DIRECTORY, icon],
        );
        let duration = e.global::<f32>(MESSAGE_DURATION);
        let sound = e
            .call(PICK_UP_SOUND_NAME, &args![player, form, 0u32, 0u32])
            .u32();
        let text = e.mem.u32(text_object);
        e.call(
            SHOW_MESSAGE,
            &args![text, 0u32, STOLEN_ITEM_ICON, sound, duration, 0u32],
        );
        e.call(STRING_DESTRUCT, &args![text_object]);
    });
}

// Translated from 004cba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form the changes record for the leveled item list `leveled`: the
/// position of `leveled` among the container's leveled item objects
/// (`TESLevItem` casts) is looked up and the entry whose extra list answers
/// that position (`0041d360`) gives its form. 0 for a null `leveled`.
pub fn fn_004cba70(e: &mut Engine, this: Ptr<InventoryChanges>, leveled: u32) -> u32 {
    let mut result = 0u32;
    if leveled == 0 {
        return result;
    }
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    let mut position = 0i32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        let lev_item = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_LEV_ITEM, 0u32],
            )
            .u32();
        if lev_item != 0 && lev_item == leveled {
            break;
        }
        if lev_item != 0 {
            position += 1;
        }
        node = list_next(e, node);
    }
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && list_item(e, node) != 0 {
        let item = list_item(e, node);
        if item != 0 {
            let mut cursor = e.mem.u32(item);
            while cursor != 0 && list_item(e, cursor) != 0 {
                let extra = list_item(e, cursor);
                if extra != 0 && e.call(EXTRA_LEVELED_POSITION, &args![extra]).i32() == position {
                    result = e.call(WORD_AT_8, &args![item]).u32();
                    break;
                }
                cursor = list_next(e, cursor);
            }
        }
        node = list_next(e, node);
    }
    result
}

/// A new temporary `TESObjectREFR` (0x68 bytes, constructed unless the
/// allocation failed, then `TESForm::SetTemporary`) the script runs of the
/// changes use.
fn new_temporary_reference(e: &mut Engine) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x68u32]).u32();
    let reference = if memory == 0 {
        0
    } else {
        e.call(REFR_CONSTRUCT, &args![memory]).u32()
    };
    e.call(FORM_SET_TEMPORARY, &args![reference]);
    reference
}

/// Gives the extra list `extra` the script variables of its script: the
/// script (`GetScript`) makes its event list (`005abf60`) and the list
/// stores it (`00419f80`).
fn attach_script_events(e: &mut Engine, extra: u32) {
    let script = e.call(EXTRA_GET_SCRIPT, &args![extra]).u32();
    let events = e.call(SCRIPT_MAKE_EVENT_LIST, &args![script]).u32();
    e.call(EXTRA_SET_SCRIPT_EVENTS, &args![extra, events]);
}

/// Pops the first item of the list `list` (`0063f7b0`: the next node's item
/// moves into the head node and the next node is freed; an empty head
/// stays).
fn list_pop_head(e: &mut Engine, list: u32) {
    e.call(LIST_POP_HEAD, &args![list]);
}

/// The health fix step of `fn_004cbbc0` for an entry: when the entry's form
/// has health (a `TESHealthForm` by dynamic cast), each extra list whose
/// health is above `-1.0` and equals the form's base health (virtual 0x10
/// of the health form) loses its health extra and count extra; a list that
/// is then plain (`fn_004bca60`) is unlinked and deleted, else its count is
/// put back.
fn fix_default_health(e: &mut Engine, entry: u32) {
    let form = e.call(WORD_AT_8, &args![entry]).u32();
    let health_form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_HEALTH_FORM,
                0u32
            ],
        )
        .u32();
    if health_form == 0 || e.mem.u32(entry) == 0 {
        return;
    }
    let mut node = e.mem.u32(entry);
    loop {
        if node == 0 || list_item(e, node) == 0 {
            break;
        }
        let list = list_item(e, node);
        let health = e.call(EXTRA_GET_HEALTH, &args![list]).f32() as f64;
        if health > e.global::<f64>(MINUS_ONE_DOUBLE) {
            let health = e.call(EXTRA_GET_HEALTH, &args![list]).f32() as f64;
            let base_health = e.vcall(health_form, 0x10, &args![]).u32();
            if health == base_health as f64 {
                e.call(EXTRA_REMOVE_HEALTH, &args![list]);
                let count = e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                e.call(EXTRA_REMOVE_COUNT, &args![list]);
                if fn_004bca60(e, Ptr::new(list)) {
                    let head = e.mem.u32(entry);
                    list_call_with_item(e, LIST_REMOVE, head, list);
                    delete_object(e, list);
                    node = e.mem.u32(entry);
                } else {
                    e.call(EXTRA_SET_COUNT, &args![list, count as u16 as u32]);
                    node = list_next(e, node);
                }
                continue;
            }
        }
        node = list_next(e, node);
    }
}

/// Removes the player ownership from every extra list of an entry of the
/// changes (the ammunition steps of `fn_004cbbc0` do it when the owner is
/// the player).
fn remove_ownership_from_lists(e: &mut Engine, entry: u32) {
    let mut node = e.mem.u32(entry);
    while node != 0 && list_item(e, node) != 0 {
        let list = list_item(e, node);
        node = list_next(e, node);
        e.call(EXTRA_REMOVE_OWNERSHIP, &args![list]);
    }
}

/// Deletes every extra list of an entry of the changes, one at a time from
/// the head (the gold step of `fn_004cbbc0` for the player).
fn delete_all_lists_of(e: &mut Engine, entry: u32) {
    let head = e.mem.u32(entry);
    while head != 0 && list_item(e, head) != 0 {
        let list = list_item(e, head);
        list_pop_head(e, head);
        delete_object(e, list);
    }
}

/// The step of `fn_004cbbc0` for an entry with a script on some list: each
/// extra list with a script that appears again later in the entry's lists
/// has the later node popped, a new extra list with the same script made, its
/// script run on a temporary reference and the new list appended.
fn rerun_duplicate_scripts(e: &mut Engine, entry: u32) {
    let mut outer = e.mem.u32(entry);
    while outer != 0 && list_item(e, outer) != 0 {
        let list = list_item(e, outer);
        if list != 0 && e.call(EXTRA_GET_SCRIPT, &args![list]).u32() != 0 {
            let mut inner = list_next(e, outer);
            while inner != 0 && list_item(e, inner) != 0 {
                let other = list_item(e, inner);
                if list == other {
                    list_pop_head(e, inner);
                    let created = new_extra_list(e);
                    let script = e.call(EXTRA_GET_SCRIPT, &args![list]).u32();
                    e.call(EXTRA_SET_SCRIPT, &args![created, script]);
                    attach_script_events(e, created);
                    let reference = new_temporary_reference(e);
                    let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![created]).u32();
                    let script = e.call(EXTRA_GET_SCRIPT, &args![list]).u32();
                    e.call(SCRIPT_RUN, &args![script, reference, locals, 0u32, 0u32]);
                    list_call_with_item(e, LIST_ADD_TAIL, inner, created);
                    inner = list_next(e, outer);
                } else {
                    inner = list_next(e, inner);
                }
            }
        }
        outer = list_next(e, outer);
    }
}

/// The step of `fn_004cbbc0` that gives an entry its form's script: with no
/// extra lists (or an empty list) `fn_004d1960` is told; else each list
/// without a script gets the form's script, its variables and the entry's
/// script run on a temporary reference (deleted afterwards when
/// `delete_reference`), then as many new lists as the entry's number
/// exceeds its lists get the entry's script and the form's script run on
/// them (those references are deleted) and are appended.
fn run_form_scripts(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    entry: u32,
    form_script: u32,
    delete_reference: bool,
) {
    let head = e.mem.u32(entry);
    if head == 0 || e.call(LIST_IS_EMPTY, &args![head]).bool() {
        e.call(INVENTORY_FN_004D1960, &args![this]);
        return;
    }
    let listed = e.call(LIST_COUNT_NONNULL, &args![head]).u32() as i32;
    let missing = entry_number(e, entry) - listed;
    let mut cursor = head;
    while cursor != 0 && list_item(e, cursor) != 0 {
        let list = list_item(e, cursor);
        if list != 0 && e.call(EXTRA_GET_SCRIPT, &args![list]).u32() == 0 {
            e.call(EXTRA_SET_SCRIPT, &args![list, form_script]);
            attach_script_events(e, list);
            let reference = new_temporary_reference(e);
            let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![list]).u32();
            let entry_script = item_change_get_script(e, Ptr::new(entry));
            e.call(
                SCRIPT_RUN,
                &args![entry_script, reference, locals, 0u32, 0u32],
            );
            if delete_reference && reference != 0 {
                e.vcall(reference, 0x10, &args![1u32]);
            }
        }
        cursor = list_next(e, cursor);
    }
    let list_head = e.mem.u32(entry);
    for _ in 0..missing.max(0) {
        let created = new_extra_list(e);
        let entry_script = item_change_get_script(e, Ptr::new(entry));
        e.call(EXTRA_SET_SCRIPT, &args![created, entry_script]);
        attach_script_events(e, created);
        let reference = new_temporary_reference(e);
        let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![created]).u32();
        e.call(
            SCRIPT_RUN,
            &args![form_script, reference, locals, 0u32, 0u32],
        );
        if reference != 0 {
            e.vcall(reference, 0x10, &args![1u32]);
        }
        list_call_with_item(e, LIST_ADD, list_head, created);
    }
}

/// The step of `fn_004cbbc0` that trims an entry's extra lists to `total`
/// (unsigned) of them: while there are more lists, the first one is popped
/// and its extras that the new first list does not have are moved to it
/// (`GetExtraData`, `RemoveExtra`, `AddExtra`) before it is deleted; then
/// an empty list is dropped from the entry, and lists without extras are
/// popped (the entry's list being cleared and deleted when that empties it).
fn trim_extra_lists(e: &mut Engine, entry: u32, total: u32) {
    let mut list = e.mem.u32(entry);
    if list == 0 || list_item(e, list) == 0 {
        return;
    }
    while total < e.call(LIST_COUNT_NONNULL, &args![list]).u32() {
        let popped = list_item(e, list);
        list_pop_head(e, list);
        if popped != 0 {
            let destination = list_item(e, list);
            if destination != 0 {
                let mut data = e.call(WORD_AT_4, &args![popped]).u32();
                while data != 0 {
                    let kind = e.call(EXTRA_DATA_TYPE, &args![data]).u8() as u32;
                    if e.call(EXTRA_GET_BY_TYPE, &args![destination, kind]).u32() != 0 {
                        e.call(EXTRA_REMOVE_EXTRA, &args![popped, data, 1u32]);
                    } else {
                        e.call(EXTRA_REMOVE_EXTRA, &args![popped, data, 0u32]);
                        e.call(EXTRA_ADD_EXTRA, &args![destination, data]);
                    }
                    data = e.call(WORD_AT_4, &args![popped]).u32();
                }
            }
            delete_object(e, popped);
        }
    }
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let head = e.mem.u32(entry);
        if head != 0 {
            e.call(LIST_DESTROY, &args![head, 1u32]);
        }
        e.mem.set_u32(entry, 0);
    } else {
        list = e.mem.u32(entry);
    }
    while list != 0 && list_item(e, list) != 0 {
        let item = list_item(e, list);
        if e.call(EXTRA_ITEMS_IN_LIST, &args![item]).u32() < 1 {
            list_pop_head(e, list);
            let head = e.mem.u32(entry);
            if e.call(LIST_IS_EMPTY, &args![head]).bool() {
                e.call(LIST_CLEAR, &args![list]);
                e.call(LIST_DESTROY, &args![list, 1u32]);
                list = 0;
                e.mem.set_u32(entry, 0);
            } else {
                list = e.mem.u32(entry);
            }
        } else {
            list = list_next(e, list);
        }
    }
}

// Translated from 004cbbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reconciles the changes with the container after a load: first for every
/// object of the container (not a leveled item, and playable when it is a
/// biped model form) that has an entry in the changes, then for every entry
/// of the changes. Each entry is brought in line with the container's
/// count: lists whose health still is the form's base health lose their
/// health and count extras (`fix_default_health`); the player's gold lists
/// are deleted (his gold is the container's count); lists with a script that
/// appear twice are rebuilt (`rerun_duplicate_scripts`); entries of a
/// scripted form get the form's script on every list (`run_form_scripts`, also
/// `fn_004d1960` when an entry is missing or empty); ammunition entries
/// have the player's ownership removed and their worn lists and numbers
/// tidied (the worn list of a leveled entry is merged, the number brought to
/// the worn lists' total or the other way round); and the extra lists are
/// trimmed to the entry's number plus the container's count
/// (`trim_extra_lists`). The C++ exception frame is not translated.
pub fn fn_004cbbc0(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let player = e.global::<u32>(PLAYER_GLOBAL);
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    // The container's objects.
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        let object = list_item(e, node);
        let object_form = e.mem.u32(object + 4);
        let leveled_item = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    object_form,
                    0u32,
                    TYPE_TES_BOUND_OBJECT,
                    TYPE_TES_LEV_ITEM,
                    0u32
                ],
            )
            .u32();
        let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
        if form != 0
            && leveled_item == 0
            && (biped == 0 || e.call(BIPED_PLAYABLE, &args![biped]).bool())
        {
            let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            if entry != 0 {
                fix_default_health(e, entry);
            }
            if entry != 0
                && e.call(IS_GOLD, &args![form]).bool()
                && e.get(this, InventoryChanges::pRef).addr() == player
                && e.mem.u32(entry) != 0
            {
                delete_all_lists_of(e, entry);
            }
            if entry != 0 && item_change_get_script(e, Ptr::new(entry)) != 0 {
                rerun_duplicate_scripts(e, entry);
            }
            let form_script = e.call(FORM_SCRIPT_OF, &args![form]).u32();
            if entry == 0 && form_script != 0 {
                e.call(INVENTORY_FN_004D1960, &args![this]);
            } else if entry != 0 && entry_number(e, entry) >= 0 && form_script != 0 {
                run_form_scripts(e, this, entry, form_script, true);
            }
            if entry != 0 && form_type_of(e, form) == 0x29 {
                let mut worn_list = 0u32;
                if e.get(this, InventoryChanges::pRef).addr() == player {
                    let mut cursor = e.mem.u32(entry);
                    while cursor != 0 && list_item(e, cursor) != 0 {
                        worn_list = list_item(e, cursor);
                        cursor = list_next(e, cursor);
                        e.call(EXTRA_REMOVE_OWNERSHIP, &args![worn_list]);
                    }
                }
                let worn = item_change_get_worn(e, Ptr::new(entry), 0);
                let leveled = worn && fn_004bcb70(e, Ptr::new(entry));
                if worn && !leveled {
                    // A worn stack that is wholly the container's: its number is dropped.
                    if entry_number(e, entry) > 0 {
                        let object = list_item(e, node);
                        let count = e.mem.u32(object) as i32;
                        if entry_number(e, entry) >= count {
                            set_entry_number(e, entry, 0);
                        }
                    }
                } else if worn && leveled {
                    if entry_number(e, entry) < 0 {
                        item_change_set_worn(e, Ptr::new(entry), 0, 0, 1);
                    }
                    let other_lists = item_change_get_amount_non_default_extra(e, Ptr::new(entry));
                    if other_lists > 1 {
                        let mut cursor = e.mem.u32(entry);
                        while cursor != 0 && list_item(e, cursor) != 0 {
                            let list = list_item(e, cursor);
                            cursor = list_next(e, cursor);
                            if list != 0
                                && e.call(EXTRA_GET_WORN, &args![list, 0u32]).bool()
                                && worn_list == 0
                            {
                                worn_list = list;
                            } else if worn_list != 0 {
                                e.call(EXTRA_REMOVE_ALL, &args![worn_list, 1u32]);
                                e.call(EXTRA_DATA_LIST_DUPLICATE, &args![worn_list, list]);
                                e.call(EXTRA_SET_WORN, &args![worn_list, 1u32, 0u32]);
                                let head = e.mem.u32(entry);
                                e.call(LIST_CLEAR, &args![head]);
                                let head = e.mem.u32(entry);
                                list_call_with_item(e, LIST_ADD, head, worn_list);
                            }
                        }
                    }
                    let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                    if entry_number(e, entry) < listed {
                        set_entry_number(e, entry, listed);
                    }
                }
            }
            let object = list_item(e, node);
            let mut count = e.mem.u32(object) as i32;
            if count < 0 {
                count = count.wrapping_mul(-1);
            }
            let total = if entry == 0 {
                count
            } else {
                entry_number(e, entry).wrapping_add(count)
            };
            if total > 0 && entry != 0 && e.mem.u32(entry) != 0 && {
                let head = e.mem.u32(entry);
                e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0
            } {
                trim_extra_lists(e, entry, total as u32);
            }
        }
        node = list_next(e, node);
    }
    // The changes' entries.
    let mut node = e.get(this, InventoryChanges::pListofChanges).addr();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let entry = list_item(e, node);
        let mut in_container = 0i32;
        if fn_004bffb0(e, this) != 0 && entry != 0 {
            let form = e.mem.u32(entry + 8);
            let container = fn_004bffb0(e, this);
            in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
        }
        let total = entry_number(e, entry).wrapping_add(in_container);
        if entry != 0 {
            fix_default_health(e, entry);
        }
        let listed = item_change_get_extra_total_count(e, Ptr::new(entry), false);
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        if entry != 0 && item_change_get_script(e, Ptr::new(entry)) != 0 {
            rerun_duplicate_scripts(e, entry);
        }
        if entry != 0 && entry_number(e, entry) >= 0 {
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            let form_script = e.call(FORM_SCRIPT_OF, &args![entry_form]).u32();
            if form_script != 0 {
                run_form_scripts(e, this, entry, form_script, false);
            }
        }
        if entry != 0 && form_type_of(e, form) == 0x29 {
            let mut kept_container_count = 0i32;
            if fn_004bffb0(e, this) != 0 && entry != 0 {
                let entry_form = e.mem.u32(entry + 8);
                let container = fn_004bffb0(e, this);
                kept_container_count = e.call(CONTAINER_COUNT, &args![container, entry_form]).i32();
            }
            if e.get(this, InventoryChanges::pRef).addr() == player {
                remove_ownership_from_lists(e, entry);
            }
            if item_change_get_worn(e, Ptr::new(entry), 0) {
                let mut in_lists = 0i32;
                let mut cursor = e.mem.u32(entry);
                while cursor != 0 && list_item(e, cursor) != 0 {
                    let list = list_item(e, cursor);
                    in_lists += e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                    cursor = list_next(e, cursor);
                }
                let number = entry_number(e, entry).wrapping_add(kept_container_count);
                if number < in_lists {
                    set_entry_number(e, entry, in_lists - number);
                } else if number > in_lists {
                    let mut cursor = e.mem.u32(entry);
                    while cursor != 0 && list_item(e, cursor) != 0 {
                        let list = list_item(e, cursor);
                        cursor = list_next(e, cursor);
                        if list != 0 && e.call(EXTRA_GET_WORN, &args![list, 0u32]).bool() {
                            let number = entry_number(e, entry);
                            e.call(EXTRA_SET_COUNT, &args![list, number as u16 as u32]);
                        }
                    }
                }
            }
        }
        if entry != 0 && e.mem.u32(entry) != 0 {
            let mut cursor = e.mem.u32(entry);
            while cursor != 0 && list_item(e, cursor) != 0 {
                let list = list_item(e, cursor);
                if list != 0 && e.call(EXTRA_GET_SCRIPT, &args![list]).u32() != 0 {
                    e.call(EXTRA_REMOVE_COUNT, &args![list]);
                }
                cursor = list_next(e, cursor);
            }
        }
        if entry != 0
            && e.call(IS_GOLD, &args![form]).bool()
            && e.get(this, InventoryChanges::pRef).addr() == player
            && e.mem.u32(entry) != 0
        {
            delete_all_lists_of(e, entry);
        }
        if entry != 0 && total < listed && e.mem.u32(entry) != 0 {
            let mut in_lists = 0i32;
            let mut cursor = e.mem.u32(entry);
            while cursor != 0 && list_item(e, cursor) != 0 {
                let list = list_item(e, cursor);
                in_lists += e.call(EXTRA_GET_COUNT, &args![list]).u16() as i16 as i32;
                cursor = list_next(e, cursor);
            }
            set_entry_number(e, entry, in_lists);
        }
        if entry != 0 && total > 0 {
            let head = e.mem.u32(entry);
            let list = head;
            if list != 0 && list_item(e, list) != 0 {
                trim_extra_lists(e, entry, total as u32);
            }
        }
        node = list_next(e, node);
    }
}

/// Copies the extra lists of the changes' `entry` (the item it holds is at
/// most the container's object count and the entry's number) onto the new
/// `ItemChange` `new_item`, one copy each (`CopyListForContainer`, flag 1)
/// appended to the new item's list. With `keep_script`, the script extra
/// (type 0xd) of each source list is taken out before the copy and put back
/// after it, so the copy has none.
fn copy_lists_for_duplicate(e: &mut Engine, entry: u32, new_item: u32, keep_script: bool) {
    if entry == 0 || e.mem.u32(entry) == 0 {
        return;
    }
    let head = e.mem.u32(entry);
    if e.call(LIST_COUNT_NONNULL, &args![head]).u32() == 0 {
        return;
    }
    let mut node = e.mem.u32(entry);
    while node != 0 && list_item(e, node) != 0 {
        let copy = new_extra_list(e);
        let source = list_item(e, node);
        let mut script_data = 0u32;
        if keep_script {
            script_data = e.call(EXTRA_GET_BY_TYPE, &args![source, 0xdu32]).u32();
            if script_data != 0 {
                e.call(EXTRA_REMOVE_EXTRA, &args![source, script_data, 0u32]);
            }
        }
        let source = list_item(e, node);
        e.call(EXTRA_COPY_LIST_FOR_CONTAINER, &args![copy, source, 1u32]);
        let target = e.mem.u32(new_item);
        list_call_with_item(e, LIST_ADD_TAIL, target, copy);
        if keep_script && script_data != 0 {
            let source = list_item(e, node);
            e.call(EXTRA_ADD_EXTRA, &args![source, script_data]);
        }
        node = list_next(e, node);
    }
}

/// A copy of the base form `form` without its script (for the duplicated
/// items of a scripted form): a new form of the same type
/// (`TESDataHandler::CreateFormOfType`), made a copy of `form` through its
/// virtual 0x108, its script cleared (`fn_004ce300`) and registered with the
/// data handler and the save game as a created base object.
fn clone_form_without_script(e: &mut Engine, form: u32) -> u32 {
    let kind = form_type_of(e, form);
    let cloned = e.call(CREATE_FORM_OF_TYPE, &args![kind]).u32();
    e.vcall(cloned, 0x108, &args![form]);
    tes_scriptable_form_set_form_script(e, cloned, 0);
    let manager = e.global::<u32>(GAME_DATA_MANAGER_GLOBAL);
    e.call(ADD_FORM_TO_DATA_HANDLER, &args![manager, cloned]);
    let save_game = e.global::<u32>(SAVE_LOAD_GAME_GLOBAL);
    e.call(ADD_CREATED_BASE_OBJECT, &args![save_game, cloned]);
    cloned
}

// Translated from 004cd9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::DuplicateAllItems` (Xbox PDB): gives the
/// `InventoryChanges` of the reference `reference` (made if needed,
/// `GetInventoryChanges`) a copy of every item of this inventory: for each
/// container object (not a leveled item) with a positive count (its count
/// made positive plus the changes' number), and for each entry of the changes
/// with a positive number, a new `ItemChange` with a copy of each extra
/// list (`CopyListForContainer`) is merged into the other changes
/// (`fn_004c3380`, deleting the new one). An item whose form has a script
/// gets a clone of the form without the script (`clone_form_without_script`)
/// and its lists lose their script extras in the copy. Entries first have their
/// leveled item extras removed. Does nothing without `reference` or a
/// non-null `_flag`'s first word being 0. The C++ exception frame is not
/// translated.
pub fn inventory_changes_duplicate_all_items(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    flag: u32,
    reference: u32,
) {
    if reference == 0 || flag == 0 {
        return;
    }
    let destination = inventory_changes_get_inventory_changes(e, reference);
    if destination == 0 {
        return;
    }
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        let object = list_item(e, node);
        let object_form = e.mem.u32(object + 4);
        e.call(
            RT_DYNAMIC_CAST,
            &args![
                object_form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_LEV_ITEM,
                0u32
            ],
        );
        if form != 0 {
            let container = fn_004bffb0(e, this);
            let in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
            let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            let magnitude = in_container.wrapping_abs();
            let total = if entry == 0 {
                magnitude
            } else {
                entry_number(e, entry).wrapping_add(magnitude)
            };
            if total > 0 {
                let new_item;
                if e.call(FORM_SCRIPT_OF, &args![form]).u32() == 0 {
                    new_item = new_item_change(e, form, total as u32);
                    copy_lists_for_duplicate(e, entry, new_item, false);
                } else {
                    let cloned = clone_form_without_script(e, form);
                    let mut count = in_container;
                    if entry != 0 {
                        count = entry_number(e, entry).wrapping_add(count);
                    }
                    new_item = new_item_change(e, cloned, count as u32);
                    copy_lists_for_duplicate(e, entry, new_item, true);
                }
                fn_004c3380(e, Ptr::new(destination), new_item, 1);
            }
        }
        node = list_next(e, node);
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let entry = list_item(e, node);
        if entry != 0 && entry_number(e, entry) > 0 {
            let form = e.call(WORD_AT_8, &args![entry]).u32();
            item_change_remove_leveled_item_extra(e, Ptr::new(entry));
            let new_item;
            if item_change_get_script(e, Ptr::new(entry)) == 0 {
                let number = entry_number(e, entry);
                new_item = new_item_change(e, form, number as u32);
                copy_lists_for_duplicate(e, entry, new_item, false);
            } else {
                let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                let cloned = clone_form_without_script(e, entry_form);
                let number = entry_number(e, entry);
                new_item = new_item_change(e, cloned, number as u32);
                copy_lists_for_duplicate(e, entry, new_item, true);
            }
            fn_004c3380(e, Ptr::new(destination), new_item, 1);
        }
        node = list_next(e, node);
    }
}

// Translated from 004ce300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESScriptableForm::SetFormScript` (Xbox PDB), a `__cdecl` function of a
/// form: when `form` is a `TESScriptableForm` (dynamic cast from `TESForm`)
/// its script word (+4) is set to `script` (`006ecd40`).
pub fn tes_scriptable_form_set_form_script(e: &mut Engine, form: u32, script: u32) {
    let scriptable = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0u32],
        )
        .u32();
    if scriptable != 0 {
        e.call(ITEM_SET_NUMBER, &args![scriptable, script]);
    }
}

// Translated from 004cfe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the owner has something with the form id `form_id`: an object of
/// the container with that form id when its count is negative (removed), or
/// the changes' number plus its count (just the count when that adds up to
/// 0), summed over the container's objects, is positive; else an extra data
/// list of the changes whose reference has that form id.
pub fn fn_004cfe20(e: &mut Engine, this: Ptr<InventoryChanges>, form_id: u32) -> bool {
    let mut total = 0i32;
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        if form != 0 && e.call(FORM_ID, &args![form]).u32() == form_id {
            let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            let object = list_item(e, node);
            let count = e.mem.u32(object) as i32;
            if count < 0 {
                return true;
            }
            if entry != 0 && entry_number(e, entry).wrapping_add(count) != 0 {
                total = entry_number(e, entry)
                    .wrapping_add(count)
                    .wrapping_add(total);
            } else {
                total = total.wrapping_add(count);
            }
            if total > 0 {
                return true;
            }
        }
        node = list_next(e, node);
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = list_item(e, node);
        let mut cursor = e.mem.u32(item);
        while cursor != 0 && list_item(e, cursor) != 0 {
            let extra = list_item(e, cursor);
            if extra != 0 && e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32() != 0 {
                let reference = e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32();
                if e.call(FORM_ID, &args![reference]).u32() == form_id {
                    return true;
                }
            }
            cursor = list_next(e, cursor);
        }
        node = list_next(e, node);
    }
    false
}

// Translated from 004cffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::ContainerHasObjects` (Xbox PDB): whether the owner has
/// at least `count` of `form`, or (without `form`) of a form matching the
/// package object type `object_type` (`00679e00`). With a `form_id` only
/// `fn_004cfe20` is asked. On success `*out` is set to the object type
/// (`TESPackage::GetObjectTypeFromForm`) of the form found, and when
/// `actor` is an actor with a process, that process is told (virtual
/// 0x1d0) the form. The container's objects are searched first (a negative
/// count counts as found at once, an object the changes do not hold also),
/// then the totals, then the changes' entries.
pub fn inventory_changes_container_has_objects(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    object_type: u32,
    count: i32,
    out: u32,
    form_id: u32,
    actor: u32,
) -> bool {
    let mut total = 0i32;
    if form_id != 0 {
        return fn_004cfe20(e, this, form_id);
    }
    let mut actor_obj = 0u32;
    if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
        actor_obj = actor;
    }
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let found = e.mem.u32(object + 4);
        if found != 0 {
            let matches = if form != 0 {
                found == form
            } else {
                object_type != 0
                    && e.call(PACKAGE_FORM_MATCHES, &args![found, object_type])
                        .bool()
            };
            if matches {
                let entry = inventory_changes_get_object_in_list(e, this, found, 1, 0);
                if entry != 0 {
                    let object = list_item(e, node);
                    let held = e.mem.u32(object) as i32;
                    if held < 0 {
                        return true;
                    }
                    total = entry_number(e, entry)
                        .wrapping_add(held)
                        .wrapping_add(total);
                    if actor_obj != 0 && e.call(ACTOR_PROCESS, &args![actor_obj]).u32() != 0 {
                        let process = e.call(ACTOR_PROCESS, &args![actor_obj]).u32();
                        let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                        e.vcall(process, 0x1d0, &args![entry_form]);
                    }
                    let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                    let kind = e
                        .call(PACKAGE_OBJECT_TYPE_FROM_FORM, &args![entry_form])
                        .u32();
                    e.mem.set_u32(out, kind);
                } else {
                    if actor_obj != 0 && e.call(ACTOR_PROCESS, &args![actor_obj]).u32() != 0 {
                        let process = e.call(ACTOR_PROCESS, &args![actor_obj]).u32();
                        e.vcall(process, 0x1d0, &args![found]);
                    }
                    let kind = e.call(PACKAGE_OBJECT_TYPE_FROM_FORM, &args![found]).u32();
                    e.mem.set_u32(out, kind);
                    return true;
                }
            }
        }
        node = list_next(e, node);
    }
    if total >= count {
        return true;
    }
    if form != 0 {
        let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        if entry != 0 && entry_number(e, entry) >= count {
            if actor_obj != 0 && e.call(ACTOR_PROCESS, &args![actor_obj]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS, &args![actor_obj]).u32();
                let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                e.vcall(process, 0x1d0, &args![entry_form]);
            }
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            let kind = e
                .call(PACKAGE_OBJECT_TYPE_FROM_FORM, &args![entry_form])
                .u32();
            e.mem.set_u32(out, kind);
            return true;
        }
        return false;
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && list_item(e, node) != 0 {
        let item = list_item(e, node);
        let found = e.mem.u32(item + 8);
        if found != 0
            && object_type != 0
            && e.call(PACKAGE_FORM_MATCHES, &args![found, object_type])
                .bool()
        {
            let item = list_item(e, node);
            if entry_number(e, item) >= count {
                if actor_obj != 0 && e.call(ACTOR_PROCESS, &args![actor_obj]).u32() != 0 {
                    let process = e.call(ACTOR_PROCESS, &args![actor_obj]).u32();
                    let item = list_item(e, node);
                    let item_form = e.call(WORD_AT_8, &args![item]).u32();
                    e.vcall(process, 0x1d0, &args![item_form]);
                }
                let item = list_item(e, node);
                let item_form = e.call(WORD_AT_8, &args![item]).u32();
                let kind = e
                    .call(PACKAGE_OBJECT_TYPE_FROM_FORM, &args![item_form])
                    .u32();
                e.mem.set_u32(out, kind);
                return true;
            }
        }
        node = list_next(e, node);
    }
    false
}

// Translated from 004d0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the owner has an item whose form answers true to its virtual
/// 0x94: an object of the container that the changes do not remove
/// completely, else an entry of the changes with a positive number.
pub fn fn_004d0360(e: &mut Engine, this: Ptr<InventoryChanges>) -> bool {
    let mut found = false;
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && list_item(e, node) != 0 && !found {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        if e.vcall(form, 0x94, &args![]).bool() {
            found = true;
            let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            if entry != 0 && entry_number(e, entry).wrapping_add(e.mem.u32(object) as i32) == 0 {
                found = false;
            }
        }
        node = list_next(e, node);
    }
    if !found {
        let mut node = e.mem.u32(this.addr());
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && !found {
            let item = list_item(e, node);
            if item != 0 {
                let form = e.mem.u32(item + 8);
                if form != 0 && e.vcall(form, 0x94, &args![]).bool() && entry_number(e, item) > 0 {
                    found = true;
                }
            }
            node = list_next(e, node);
        }
    }
    found
}

// Translated from 004d0490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the owner has an item whose form has a script
/// (`TESScriptableForm::GetFormScript`): an object of the container that the
/// changes do not remove completely, else an entry of the changes with a
/// non-zero number. Every form is looked at once: a `NiTMap` of forms seen
/// (`004d4de0` with 0x25 buckets, filled by `0084d310`, asked by
/// `0057c850`) keeps the container's forms from being counted again by the
/// changes.
pub fn fn_004d0490(e: &mut Engine, this: Ptr<InventoryChanges>) -> bool {
    e.with_stack(0x1c, |e, map| {
        e.call(SEEN_FORMS_CONSTRUCT, &args![map, 0x25u32]);
        let mut found = false;
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && !found {
            let object = list_item(e, node);
            let form = e.mem.u32(object + 4);
            let seen = e.with_stack(4, |e, flag| {
                e.mem.set_u8(flag.addr(), 0);
                e.call(SEEN_FORMS_LOOKUP, &args![map, form, flag]).bool()
            });
            if !seen {
                e.call(SEEN_FORMS_INSERT, &args![map, form, 1u32]);
                if e.call(FORM_SCRIPT_OF, &args![form]).u32() != 0 {
                    found = true;
                    let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                    if entry != 0
                        && entry_number(e, entry).wrapping_add(e.mem.u32(object) as i32) == 0
                    {
                        found = false;
                    }
                }
            }
            node = list_next(e, node);
        }
        if !found {
            let mut node = e.mem.u32(this.addr());
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && !found {
                let item = list_item(e, node);
                if entry_number(e, item) != 0 {
                    let form = e.mem.u32(item + 8);
                    let seen = e.with_stack(4, |e, flag| {
                        e.mem.set_u8(flag.addr(), 0);
                        e.call(SEEN_FORMS_LOOKUP, &args![map, form, flag]).bool()
                    });
                    if !seen && e.call(FORM_SCRIPT_OF, &args![form]).u32() != 0 {
                        found = true;
                    }
                }
                node = list_next(e, node);
            }
        }
        e.call(SEEN_FORMS_DESTRUCT, &args![map]);
        found
    })
}

// Translated from 004ce340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper of the inventory transfer routine `fn_004ce380`: hands its
/// own `this` and all eight stack arguments on unchanged (the four flags as
/// bytes) and leaves the returned total value as it is.
#[allow(clippy::too_many_arguments)]
pub fn fn_004ce340(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    target: u32,
    set_owner: u8,
    flag_b: u8,
    strip_owner: u8,
    quiet: u8,
    filter: i32,
    mode: i32,
) -> f32 {
    fn_004ce380(
        e,
        this,
        actor,
        target,
        set_owner,
        flag_b,
        strip_owner,
        quiet,
        filter,
        mode as u32,
    )
}

/// Whether the form list `list` (its embedded list at +0x18,
/// `00500940`) holds `form` (`005f65d0` is given the address of a slot
/// with the form in it).
fn form_in_skip_list(e: &mut Engine, list: u32, form: u32) -> bool {
    with_item_slot(e, form, |e, slot| {
        let head = e.call(FORM_LIST_HEAD, &args![list]).u32();
        e.call(LIST_CONTAINS, &args![head, slot]).bool()
    })
}

/// Whether the form type `kind` is one whose taking is announced.
fn is_announced_type(kind: u32) -> bool {
    matches!(
        kind,
        0x18 | 0x19 | 0x1e | 0x1f | 0x28 | 0x29 | 0x2e | 0x2f | 0x32 | 0x67 | 0x6c | 0x73 | 0x74
    )
}

/// The message `fn_004ce380` shows when the player is given `form`: the
/// text "name" or "N name(s)" in a string object (plural when `plural_count`
/// is above 1, the number shown is `shown_count`, or the count of the extra
/// list `shown_extra` when that is not 0), and, for the form types of
/// `is_announced_type`, the message itself with the vault boy icon, the
/// pick up sound of the player for the form and the message duration.
fn announce_taken_item(
    e: &mut Engine,
    form: u32,
    plural_count: i32,
    shown_count: i32,
    shown_extra: u32,
) {
    e.with_stack(0x10, |e, text_object| {
        let text_object = text_object.addr();
        e.call(STRING_CONSTRUCT, &args![text_object]);
        if plural_count > 1 {
            let word = e
                .call(STRING_OBJECT_TO_CHARS, &args![MESSAGE_WORD_THREE])
                .u32();
            let suffix = e
                .call(STRING_OBJECT_TO_CHARS, &args![MESSAGE_WORD_TWO])
                .u32();
            let name = e.call(FULL_NAME_OF_FORM, &args![form]).u32();
            let shown = if shown_extra != 0 {
                e.call(EXTRA_GET_COUNT, &args![shown_extra]).u16() as i16 as i32
            } else {
                shown_count
            };
            e.call(
                FORMAT_STRING,
                &args![text_object, FORMAT_COUNT_NAME, shown, name, suffix, word],
            );
        } else {
            let word = e
                .call(STRING_OBJECT_TO_CHARS, &args![MESSAGE_WORD_THREE])
                .u32();
            let name = e.call(FULL_NAME_OF_FORM, &args![form]).u32();
            e.call(FORMAT_STRING, &args![text_object, FORMAT_NAME, name, word]);
        }
        let kind = form_type_of(e, form);
        if is_announced_type(kind) {
            let player = e.global::<u32>(PLAYER_GLOBAL);
            let sound = e
                .call(PICK_UP_SOUND_NAME, &args![player, form, 1u32, 0u32])
                .u32();
            let duration = e.global::<f32>(MESSAGE_DURATION);
            let text = e.mem.u32(text_object);
            e.call(
                SHOW_MESSAGE,
                &args![text, 0u32, STOLEN_ITEM_ICON, sound, duration, 0u32],
            );
        }
        e.call(STRING_DESTRUCT, &args![text_object]);
    });
}

/// Announces `form` when the player is the one given the items and the
/// message is not turned off (`quiet`), then runs `00704af0` (called either
/// way once the target is the player).
fn announce_if_player(
    e: &mut Engine,
    target: u32,
    form: u32,
    quiet: u8,
    plural_count: i32,
    shown_count: i32,
    shown_extra: u32,
) {
    let player = e.global::<u32>(PLAYER_GLOBAL);
    if target == player {
        if form != 0 && quiet == 0 {
            announce_taken_item(e, form, plural_count, shown_count, shown_extra);
        }
        e.call(AFTER_TRANSFER_REFRESH, &args![]);
    }
}

/// What the transfer does with the ownership of the extra list `extra`
/// taken for `form` (owner `owner`): with the flags `set_owner` or
/// `flag_b` and a form that is not gold, a list without an owner gets
/// `owner`; otherwise, with `strip_owner`, the owner is removed, and the
/// list is dropped (returned as 0) when it carried one and the number of
/// items in it was too low to keep the count (`items`, `GetCount`).
fn prepare_taken_extra(
    e: &mut Engine,
    form: u32,
    extra: u32,
    owner: u32,
    set_owner: u8,
    flag_b: u8,
    strip_owner: u8,
) -> u32 {
    if (set_owner != 0 || flag_b != 0) && !e.call(IS_GOLD, &args![form]).bool() {
        if e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32() == 0 {
            e.call(EXTRA_SET_OWNER, &args![extra, owner]);
        }
        return extra;
    }
    if strip_owner == 0 {
        return extra;
    }
    let items = e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).i32();
    if e.call(EXTRA_GET_OWNERSHIP, &args![extra]).u32() == 0 {
        e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
        return extra;
    }
    let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
    let clear = if count > 1 { items <= 2 } else { items <= 1 };
    e.call(EXTRA_REMOVE_OWNERSHIP, &args![extra]);
    if clear {
        0
    } else {
        extra
    }
}

/// Takes the head extra list off the entry's list when it has no items
/// (`0063f7b0` pops the head); a list that is then empty is cleared and
/// freed and the entry's list word becomes 0. Returns the node the scan
/// goes on at: 0 for an emptied list, else the entry's list word.
fn drop_empty_extra(e: &mut Engine, entry: u32, node: u32) -> u32 {
    list_pop_head(e, node);
    if e.call(LIST_IS_EMPTY, &args![node]).bool() {
        e.call(LIST_CLEAR, &args![node]);
        e.call(LIST_DESTROY, &args![node, 1u32]);
        e.mem.set_u32(entry, 0);
        0
    } else {
        e.mem.u32(entry)
    }
}

/// Gives `target` the extra list `extra` of the entry (`fn_004c37d0` with
/// the entry's form, `set_owner`, the list's count, `entry_in` as the
/// entry) once the entry's worn state is dealt with: a worn entry first
/// goes through `fn_004c0cf0` (the actor only when it is an actor), and
/// when that does not take it the list is given anyway.
#[allow(clippy::too_many_arguments)]
fn give_extra_of_entry(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    target: u32,
    form: u32,
    set_owner: u8,
    entry: u32,
    extra: u32,
) {
    let worn = item_change_get_worn(e, Ptr::new(entry), 0);
    if worn {
        let given = e.with_stack(2, |e, flags| {
            let removed = flags.addr();
            let matched = flags.addr() + 1;
            e.mem.set_u8(removed, 0);
            e.mem.set_u8(matched, 0);
            let is_actor = e.vcall(actor, 0x100, &args![]).bool();
            let number = entry_number(e, entry);
            fn_004c0cf0(
                e,
                this,
                removed,
                form,
                number,
                if is_actor { actor } else { 0 },
                extra,
                0,
                0,
                entry,
                matched,
            );
            e.mem.u8(matched) != 0
        });
        if given {
            return;
        }
    }
    let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
    fn_004c37d0(
        e, this, actor, form, set_owner, held, extra, 0, target, 0, 0, 1, 0, entry,
    );
}

// Translated from 004ce380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the owner's items to `target` and returns the sum of their
/// values as a float. It first walks the container's objects, then the
/// changes' entries; leveled items, items of an unplayable biped form,
/// items in the form list `skip_list` (when not null), items whose type is
/// not `type_filter` (when that is not negative) and entries the changes
/// hold for the container's forms are left out. The player's equipped
/// worn items stay with the player (through `fn_004c0cf0`). Each extra list
/// of an item is given on its own (`fn_004c37d0`), with `set_owner` or
/// `flag_b` the owner of the owner's base form is set on lists without one,
/// with `strip_owner` the owner is removed, and a rest of the plain count
/// is given with the entry. When the target is the player a message
/// "N name(s)" is shown for each (`quiet` turns it off, `announce_taken_item`).
/// The value of an item is its form value times the number; the sum is a
/// 32-bit float. The C++ exception frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_004ce380(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    actor: u32,
    target: u32,
    set_owner: u8,
    flag_b: u8,
    strip_owner: u8,
    quiet: u8,
    type_filter: i32,
    skip_list: u32,
) -> f32 {
    let mut total = 0.0f32;
    let has_skip = skip_list != 0;
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        let leveled = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_LEV_ITEM, 0u32],
            )
            .u32();
        let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
        let skipped = (has_skip && form_in_skip_list(e, skip_list, form))
            || (type_filter > -1 && form != 0 && form_type_of(e, form) as i32 != type_filter);
        if skipped {
            node = list_next(e, node);
            continue;
        }
        let player = e.global::<u32>(PLAYER_GLOBAL);
        if e.vcall(form, 0x94, &args![]).bool() && actor == player {
            let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            if entry != 0 && item_change_get_worn(e, Ptr::new(entry), 0) {
                let object = list_item(e, node);
                let object_form = e.mem.u32(object + 4);
                let container = fn_004bffb0(e, this);
                let held = e
                    .call(CONTAINER_COUNT, &args![container, object_form])
                    .i32();
                let number = entry_number(e, entry).wrapping_add(held);
                e.with_stack(4, |e, removed| {
                    e.mem.set_u8(removed.addr(), 0);
                    fn_004c0cf0(e, this, removed.addr(), form, number, actor, 0, 0, 0, 0, 0);
                });
            }
            node = list_next(e, node);
            continue;
        }
        let takes = form != 0
            && leveled == 0
            && (biped == 0 || e.call(BIPED_PLAYABLE, &args![biped]).bool());
        if !takes {
            node = list_next(e, node);
            continue;
        }
        let mut entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        if entry != 0 && fn_004bcb70(e, Ptr::new(entry)) {
            node = list_next(e, node);
            continue;
        }
        let mut remaining = 0i32;
        let object = list_item(e, node);
        let object_form = e.mem.u32(object + 4);
        let container = fn_004bffb0(e, this);
        let mut in_container = e
            .call(CONTAINER_COUNT, &args![container, object_form])
            .i32();
        if in_container < 0 {
            in_container = in_container.wrapping_mul(-1);
        }
        if entry != 0 {
            remaining = entry_number(e, entry)
                .wrapping_add(in_container)
                .wrapping_add(remaining);
        } else {
            remaining = remaining.wrapping_add(in_container);
        }
        if remaining <= 0 {
            node = list_next(e, node);
            continue;
        }
        let value = e.call(GET_FORM_VALUE, &args![form]).i32();
        total = ((value.wrapping_mul(remaining) as f64) + (total as f64)) as f32;
        if entry != 0 && e.mem.u32(entry) != 0 && {
            let head = e.mem.u32(entry);
            e.call(LIST_COUNT_NONNULL, &args![head]).u32() != 0
        } {
            let mut list_node = e.mem.u32(entry);
            if list_node != 0 && list_item(e, list_node) != 0 {
                let mut single = false;
                while list_node != 0 && list_item(e, list_node) != 0 && !single {
                    let mut extra = list_item(e, list_node);
                    if e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).u32() < 1 {
                        list_node = drop_empty_extra(e, entry, list_node);
                        continue;
                    }
                    let mut multi = false;
                    let head = e.mem.u32(entry);
                    if e.call(LIST_COUNT_NONNULL, &args![head]).u32() > 1 {
                        multi = true;
                    } else {
                        single = true;
                    }
                    let owner = removal_owner(e, actor);
                    extra =
                        prepare_taken_extra(e, form, extra, owner, set_owner, flag_b, strip_owner);
                    let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                    remaining = remaining.wrapping_sub(held);
                    give_extra_of_entry(e, this, actor, target, form, set_owner, entry, extra);
                    if single {
                        entry = 0;
                        list_node = 0;
                    } else if multi {
                        list_node = e.mem.u32(entry);
                    } else {
                        list_node = list_next(e, list_node);
                    }
                }
            }
        }
        if remaining > 0 {
            if set_owner == 0 && flag_b == 0 {
                fn_004c37d0(
                    e, this, actor, form, set_owner, remaining, 0, 0, target, 0, 0, 1, 0, 0,
                );
            } else {
                let created = new_item_change(e, form, remaining as u32);
                let extra_list = reference_extra_list(e, actor);
                let mut owner = e.call(EXTRA_GET_ORIGINAL_BASE, &args![extra_list]).u32();
                if owner == 0 {
                    owner = reference_base_form(e, actor);
                }
                fn_004bd5c0(e, Ptr::new(created), Ptr::new(owner));
                let head = e.mem.u32(created);
                let extra = list_item(e, head);
                e.call(EXTRA_SET_COUNT, &args![extra, remaining as u32 & 0xffff]);
                let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                fn_004c37d0(
                    e, this, actor, form, set_owner, held, extra, 0, target, 0, 0, 1, 0, 0,
                );
            }
            announce_if_player(e, target, form, quiet, remaining, remaining, 0);
        }
        node = list_next(e, node);
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let mut taken = false;
        let mut entry = list_item(e, node);
        if has_skip {
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            if form_in_skip_list(e, skip_list, entry_form) {
                node = list_next(e, node);
                continue;
            }
        }
        if type_filter > -1 && entry != 0 {
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            if form_type_of(e, entry_form) as i32 != type_filter {
                node = list_next(e, node);
                continue;
            }
        }
        if entry == 0 {
            node = list_next(e, node);
            continue;
        }
        let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
        let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![entry_form]).u32();
        if biped != 0 && !e.call(BIPED_PLAYABLE, &args![biped]).bool() {
            node = list_next(e, node);
            continue;
        }
        if !fn_004bcb70(e, Ptr::new(entry)) && fn_004bffb0(e, this) != 0 {
            let entry_form = e.mem.u32(entry + 8);
            let container = fn_004bffb0(e, this);
            if e.call(CONTAINER_HAS_FORM, &args![container, entry_form])
                .bool()
            {
                node = list_next(e, node);
                continue;
            }
        }
        let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
        let player = e.global::<u32>(PLAYER_GLOBAL);
        if e.vcall(entry_form, 0x94, &args![]).bool() && actor == player {
            if entry != 0 && item_change_get_worn(e, Ptr::new(entry), 0) {
                let number = entry_number(e, entry);
                let worn_flag = item_change_get_worn(e, Ptr::new(entry), 1) as u8;
                let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                e.with_stack(4, |e, removed| {
                    e.mem.set_u8(removed.addr(), 0);
                    fn_004c0cf0(
                        e,
                        this,
                        removed.addr(),
                        entry_form,
                        number,
                        actor,
                        0,
                        worn_flag,
                        1,
                        0,
                        0,
                    );
                });
            }
            node = list_next(e, node);
            continue;
        }
        if entry == 0 || entry_number(e, entry) <= 0 {
            node = list_next(e, node);
            continue;
        }
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        let value = e.call(GET_FORM_VALUE, &args![form]).i32();
        total = ((value.wrapping_mul(entry_number(e, entry)) as f64) + (total as f64)) as f32;
        let mut in_container = 0i32;
        let container = fn_004bffb0(e, this);
        if container != 0 && entry != 0 {
            let entry_form = e.mem.u32(entry + 8);
            let container = fn_004bffb0(e, this);
            in_container = e.call(CONTAINER_COUNT, &args![container, entry_form]).i32();
        }
        let mut remaining = in_container.wrapping_add(entry_number(e, entry));
        let mut list_node = e.mem.u32(entry);
        if list_node == 0 || list_item(e, list_node) == 0 {
            let owner = removal_owner(e, actor);
            let mut extra = 0u32;
            if (set_owner != 0 || flag_b != 0) && !e.call(IS_GOLD, &args![form]).bool() {
                extra = new_extra_list(e);
                e.call(EXTRA_SET_OWNER, &args![extra, owner]);
            }
            let number = entry_number(e, entry);
            fn_004c37d0(
                e,
                this,
                actor,
                form,
                set_owner,
                number,
                extra,
                0,
                target,
                0,
                0,
                1,
                0,
                if taken { 0 } else { entry },
            );
            announce_if_player(e, target, form, quiet, number, number, 0);
            node = e.mem.u32(this.addr());
            continue;
        }
        let mut single = false;
        let mut run_final = true;
        'extras: loop {
            loop {
                if list_node == 0 || list_item(e, list_node) == 0 || single {
                    run_final = false;
                    break 'extras;
                }
                let mut extra = list_item(e, list_node);
                let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                if count < 1 {
                    break;
                }
                if e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).u32() < 1 {
                    list_node = drop_empty_extra(e, entry, list_node);
                    continue;
                }
                if e.call(LIST_COUNT_NONNULL, &args![list_node]).u32() <= 1 {
                    single = true;
                }
                let owner = removal_owner(e, actor);
                extra = prepare_taken_extra(e, form, extra, owner, set_owner, flag_b, strip_owner);
                if extra != 0 {
                    let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                    remaining = remaining.wrapping_sub(held);
                    let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                    fn_004c37d0(
                        e, this, actor, form, set_owner, held, extra, 0, target, 0, 0, 1, 0, entry,
                    );
                    taken = true;
                    announce_if_player(e, target, form, quiet, remaining, 0, extra);
                }
                entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                if remaining <= 0 && entry == 0 {
                    node = e.mem.u32(this.addr());
                } else if remaining <= 0 && entry != 0 {
                    node = list_next(e, node);
                }
                if entry == 0 || remaining <= 0 {
                    list_node = 0;
                } else {
                    list_node = e.mem.u32(entry);
                }
            }
            list_node = list_next(e, list_node);
            if list_node == 0 {
                break;
            }
        }
        if run_final {
            let number = e.mem.u32(entry + 4) as i32;
            fn_004c37d0(
                e, this, actor, form, set_owner, number, 0, 0, target, 0, 0, 1, 0, entry,
            );
            taken = true;
            announce_if_player(e, target, form, quiet, remaining, number, 0);
            remaining = 0;
            node = e.mem.u32(this.addr());
        }
        if remaining > 0 {
            fn_004c37d0(
                e,
                this,
                actor,
                form,
                set_owner,
                remaining,
                0,
                0,
                target,
                0,
                0,
                1,
                0,
                if taken { 0 } else { entry },
            );
            node = e.mem.u32(this.addr());
            announce_if_player(e, target, form, quiet, remaining, remaining, 0);
        }
    }
    total
}

// Constants of the fourth session (functions 004d0650 to 0076b630).

/// `00403e20` (`__thiscall`) on a game setting object: the address of the
/// setting's float value (the same accessor `KARMA_REWARD_OBJECT` names).
pub(crate) const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// The setting objects `fn_004d0900` reads: the multiplier applied to light
/// items and the weight limit under which it applies.
pub(crate) const LIGHT_WEIGHT_FACTOR_SETTING: u32 = 0x011c_64a8;
pub(crate) const LIGHT_WEIGHT_LIMIT_SETTING: u32 = 0x011c_6478;
/// `10.0` as a double: the weapon weight from which the entry point 0x49
/// (weapon weight modifier) applies.
pub(crate) const HEAVY_WEAPON_WEIGHT: u32 = 0x0102_0758;
/// `TESWeightForm::GetFormWeight(form, withMods)` (Xbox PDB, `__cdecl`).
pub(crate) const WEIGHT_FORM_GET_WEIGHT: u32 = 0x0048_ebc0;
/// Numbers of `HandleEntryPoint` the weight code uses (not named here):
/// 0x37 switches the light item factor on, 0x49 modifies a weapon's weight.
pub(crate) const ENTRY_POINT_LIGHT_ITEMS: u32 = 0x37;
pub(crate) const ENTRY_POINT_WEAPON_WEIGHT: u32 = 0x49;
/// `008ba2f0(object, 0x2e, previous, current, 0)` (`__cdecl`, floats passed
/// as one word each): called on the sub-object at +0xa4 of the owner actor
/// with the old weight and the change.
pub(crate) const ACTOR_WEIGHT_CHANGED: u32 = 0x008b_a2f0;
/// Virtual slot 0x94 of a form (a bool the weight code tests to skip it).
pub(crate) const FORM_EXCLUDED_SLOT: u32 = 0x94;
/// Form type bytes the weight code compares: weapon (0x28) and 0x18.
pub(crate) const FORM_TYPE_WEAPON: u32 = 0x28;
pub(crate) const FORM_TYPE_ARMOR: u32 = 0x18;

/// `HandleEntryPoint(kind, actor, &value)` with `value` starting at `initial`;
/// returns the float the entry point left in it.
fn entry_point_float(e: &mut Engine, kind: u32, actor: u32, initial: f32) -> f32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), initial);
        e.call(HANDLE_ENTRY_POINT, &args![kind, actor, slot]);
        e.mem.f32(slot.addr())
    })
}

// Translated from 004d0650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetInventoryItem` (Xbox PDB): a new `ItemChange` for
/// `form` made from the changes' entry (the number is the entry's plus the
/// owner container's count; the extra lists are copied: all of them when the
/// first one is the container's default one, else the first only) or, when
/// the changes have no entry, from the owner container's own object for the
/// form; 0 when there is neither. The second stack word goes to
/// `GetObjectInList`. When the entry holds no lists, the new item's list is
/// deleted. The C++ exception frame is not translated.
pub fn inventory_changes_get_inventory_item(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    reference_id: u32,
) -> u32 {
    let entry = inventory_changes_get_object_in_list(e, this, form, 1, reference_id);
    if entry == 0 {
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let object = list_item(e, node);
            if e.mem.u32(object + 4) == form {
                let object = list_item(e, node);
                let count = e.mem.u32(object);
                return new_item_change(e, form, count);
            }
            node = list_next(e, node);
        }
        return 0;
    }
    let container = fn_004bffb0(e, this);
    let in_container = e.call(CONTAINER_COUNT, &args![container, form]).i32();
    let number = e
        .call(WORD_AT_4, &args![entry])
        .i32()
        .wrapping_add(in_container);
    let result = new_item_change(e, form, number as u32);
    let entry_lists = e.call(WORD_AT_0, &args![entry]).u32();
    if entry_lists != 0 {
        let list = e.call(WORD_AT_0, &args![entry]).u32();
        if e.call(LIST_COUNT_NONNULL, &args![list]).u32() != 0 {
            if e.call(WORD_AT_0, &args![result]).u32() == 0 {
                let node = new_list_node(e);
                e.mem.set_u32(result, node);
            }
            let source = e.call(WORD_AT_0, &args![entry]).u32();
            let first = list_item(e, source);
            if e.call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![first, 0u32])
                .bool()
            {
                let mut cursor = e.call(WORD_AT_0, &args![entry]).u32();
                while cursor != 0 && list_item(e, cursor) != 0 {
                    let item = list_item(e, cursor);
                    let target = e.mem.u32(result);
                    list_call_with_item(e, LIST_ADD_TAIL, target, item);
                    cursor = list_next(e, cursor);
                }
            } else {
                let target = e.mem.u32(result);
                list_call_with_item(e, LIST_ADD, target, first);
            }
            return result;
        }
    }
    if e.call(WORD_AT_0, &args![result]).u32() != 0 {
        let list = e.mem.u32(result);
        if list != 0 {
            e.call(LIST_DESTROY, &args![list, 1u32]);
        }
        e.mem.set_u32(result, 0);
    }
    result
}

/// A weapon weight with the weapon weight modifier (entry point 0x49 of the
/// player) applied from `HEAVY_WEAPON_WEIGHT` up.
fn heavy_weapon_adjusted(e: &mut Engine, weight: f32) -> f32 {
    let limit = e.global::<f64>(HEAVY_WEAPON_WEIGHT);
    if (weight as f64) >= limit {
        let player = e.global::<u32>(PLAYER_GLOBAL);
        let factor = entry_point_float(e, ENTRY_POINT_WEAPON_WEIGHT, player, 1.0);
        return (weight as f64 * factor as f64) as f32;
    }
    weight
}

// Translated from 004d0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The total weight the owner carries (the map has no name), cached in
/// `fcontainerweight` and recomputed only when that is `-1.0`. Walks the owner
/// container's objects (an object with a changes entry counts the entry's
/// number plus its own; weapons use the weight with mods, scaled by entry
/// point 0x49 from 10.0 up), then the changes' entries the container does not
/// list (weapons with an active mod of action 4 are weighed per stack with the
/// modded weight). With the entry point 0x37 above zero, weights at or under
/// the limit setting are multiplied by the factor setting. A worn armor piece
/// of an actor owner counts once at full weight. When the weight was
/// recomputed for an actor, its sub-object at +0xa4 is told the old weight and
/// the change (`008ba2f0`). Returns the cached weight (an `f32` in ST0).
pub fn fn_004d0900(e: &mut Engine, this: Ptr<InventoryChanges>, with_mods: u8) -> f32 {
    let mut actor = 0u32;
    let owner = e.get(this, InventoryChanges::pRef).addr();
    if e.vcall(owner, 0x100, &args![]).bool() {
        actor = e.get(this, InventoryChanges::pRef).addr();
    }
    let light_bonus = entry_point_float(e, ENTRY_POINT_LIGHT_ITEMS, actor, 0.0);
    let factor_address = e
        .call(SETTING_FLOAT_ADDRESS, &args![LIGHT_WEIGHT_FACTOR_SETTING])
        .u32();
    let factor = e.mem.f32(factor_address);
    let limit_address = e
        .call(SETTING_FLOAT_ADDRESS, &args![LIGHT_WEIGHT_LIMIT_SETTING])
        .u32();
    let limit = e.mem.f32(limit_address);
    let light_items = 0.0 < light_bonus;
    if e.get(this, InventoryChanges::fcontainerweight) as f64 != e.global::<f64>(MINUS_ONE_DOUBLE) {
        return e.get(this, InventoryChanges::fcontainerweight);
    }
    let mut total = 0.0f32;
    let mut mod_weight = 0.0f32;
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
        if !e.vcall(form, FORM_EXCLUDED_SLOT, &args![]).bool() {
            let mut weight;
            if form_type_of(e, form) == FORM_TYPE_WEAPON && entry != 0 {
                let with_mod = item_change_has_mod_effect_active_ov2(e, Ptr::new(entry), 4);
                weight = tes_object_weap_get_form_weight(e, Ptr::new(form), with_mod as u8);
                weight = heavy_weapon_adjusted(e, weight);
            } else {
                weight = e
                    .call(WEIGHT_FORM_GET_WEIGHT, &args![form, with_mods as u32])
                    .f32();
            }
            if weight as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
                weight = 0.0;
            }
            if light_items && weight <= limit {
                weight = (weight as f64 * factor as f64) as f32;
            }
            let object = list_item(e, node);
            let count = e.mem.u32(object) as i32;
            if entry == 0 {
                total = (count as f64 * weight as f64 + total as f64) as f32;
            } else {
                let number = e.call(WORD_AT_4, &args![entry]).i32().wrapping_add(count);
                if number != 0 {
                    total = (number as f64 * weight as f64 + total as f64) as f32;
                }
            }
        }
        node = list_next(e, node);
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && list_item(e, node) != 0 {
        let mut is_weapon = false;
        let entry = list_item(e, node);
        let form = e.mem.u32(entry + 8);
        if form != 0 {
            let mut weight;
            let container = fn_004bffb0(e, this);
            if container != 0 && e.call(CONTAINER_HAS_FORM, &args![container, form]).bool() {
                node = list_next(e, node);
                continue;
            }
            if e.vcall(form, FORM_EXCLUDED_SLOT, &args![]).bool() {
                node = list_next(e, node);
                continue;
            }
            if form_type_of(e, form) == FORM_TYPE_WEAPON {
                is_weapon = true;
                let with_mod = item_change_has_mod_effect_active_ov2(e, Ptr::new(entry), 4);
                mod_weight = tes_object_weap_get_form_weight(e, Ptr::new(form), with_mod as u8);
                weight = e
                    .call(WEIGHT_FORM_GET_WEIGHT, &args![form, with_mods as u32])
                    .f32();
                weight = heavy_weapon_adjusted(e, weight);
                mod_weight = heavy_weapon_adjusted(e, mod_weight);
            } else {
                weight = e
                    .call(WEIGHT_FORM_GET_WEIGHT, &args![form, with_mods as u32])
                    .f32();
            }
            if weight as f64 > e.global::<f64>(ZERO_DOUBLE) {
                if light_items && weight <= limit {
                    weight = (weight as f64 * factor as f64) as f32;
                }
                if light_items && mod_weight <= limit {
                    mod_weight = (mod_weight as f64 * factor as f64) as f32;
                }
                let armor = if form_type_of(e, form) == FORM_TYPE_ARMOR {
                    form
                } else {
                    0
                };
                let mut count = e.call(WORD_AT_4, &args![entry]).i32();
                let lists = e.call(WORD_AT_0, &args![entry]).u32();
                if lists != 0
                    && is_weapon
                    && item_change_get_extra_total_count(e, Ptr::new(entry), true) >= 1
                {
                    let weapon = e.call(WORD_AT_8, &args![entry]).u32();
                    let mut modded = 0i32;
                    let mut cursor = e.call(WORD_AT_0, &args![entry]).u32();
                    while cursor != 0 {
                        let extra = list_item(e, cursor);
                        if extra != 0 && e.call(EXTRA_HAS_WEAPON_MODS, &args![extra]).bool() {
                            for slot_bit in [1u32, 2, 4] {
                                if e.call(EXTRA_GET_WEAPON_MOD_SLOT_ACTIVE, &args![extra, slot_bit])
                                    .bool()
                                    && fn_004bd880(e, Ptr::new(weapon), slot_bit) == 4
                                {
                                    let held =
                                        e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                                    modded = modded.wrapping_add(held);
                                }
                            }
                        }
                        cursor = list_next(e, cursor);
                    }
                    weight = (count.wrapping_sub(modded) as f64 * weight as f64
                        + modded as f64 * mod_weight as f64) as f32;
                } else if is_weapon {
                    weight = (count as f64 * weight as f64) as f32;
                }
                if actor != 0
                    && armor != 0
                    && item_change_get_worn(e, Ptr::new(entry), 0)
                    && count > 0
                {
                    total = (total as f64 + weight as f64) as f32;
                    count -= 1;
                }
                if count > 0 && !is_weapon {
                    total = (count as f64 * weight as f64 + total as f64) as f32;
                } else if count != 0 {
                    total = (total as f64 + weight as f64) as f32;
                }
            }
        }
        node = list_next(e, node);
    }
    e.set(this, InventoryChanges::fcontainerweight, total);
    if actor != 0 {
        let sub_object = actor + 0xa4;
        let previous = e.get(this, InventoryChanges::fpreviousContainerWeight);
        let current = e.get(this, InventoryChanges::fcontainerweight);
        let difference = (current as f64 - previous as f64) as f32;
        e.call(
            ACTOR_WEIGHT_CHANGED,
            &args![
                sub_object,
                0x2eu32,
                previous.to_bits(),
                difference.to_bits(),
                0u32
            ],
        );
    }
    e.get(this, InventoryChanges::fcontainerweight)
}

/// `TESObject` type descriptor (`.?AVTESObject@@`) and the word at +0x7bc of
/// the player character `fn_004d1360` tests.
pub(crate) const TYPE_TES_OBJECT: u32 = 0x0118_3128;
pub(crate) const PLAYER_WEIGHT_MODE: u32 = 0x7bc;

// Translated from 004d0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The total value of everything the owner carries (the map has no name):
/// the owner container's objects first (value times the number, the entry's
/// number added when the changes hold one), then the changes' entries the
/// container does not list. A form is left out when its virtual 0x94 says so
/// (unless `include_excluded`) and the form with the id 0xf is left out
/// (unless `include_caps`); a value of `-1` counts as 0.
pub fn fn_004d0f40(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    include_excluded: u8,
    include_caps: u8,
) -> i32 {
    let mut total = 0i32;
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        if (include_excluded != 0 || !e.vcall(form, FORM_EXCLUDED_SLOT, &args![]).bool())
            && (include_caps != 0 || e.call(FORM_ID, &args![form]).u32() != 0xf)
        {
            let mut value = e.call(GET_FORM_VALUE, &args![form]).i32();
            if value == -1 {
                value = 0;
            }
            if value != 0 {
                let entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
                let object = list_item(e, node);
                let count = e.mem.u32(object) as i32;
                if entry == 0 {
                    total = value.wrapping_mul(count).wrapping_add(total);
                } else {
                    let number = e.call(WORD_AT_4, &args![entry]).i32().wrapping_add(count);
                    if number != 0 {
                        total = number.wrapping_mul(value).wrapping_add(total);
                    }
                }
            }
        }
        node = list_next(e, node);
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && list_item(e, node) != 0 {
        let entry = list_item(e, node);
        let form = e.mem.u32(entry + 8);
        if form != 0 {
            let container = fn_004bffb0(e, this);
            let listed =
                container != 0 && e.call(CONTAINER_HAS_FORM, &args![container, form]).bool();
            if !listed
                && (include_excluded != 0 || !e.vcall(form, FORM_EXCLUDED_SLOT, &args![]).bool())
                && (include_caps != 0 || e.call(FORM_ID, &args![form]).u32() != 0xf)
            {
                let mut value = e.call(GET_FORM_VALUE, &args![form]).i32();
                if value == -1 {
                    value = 0;
                }
                if value != 0 && e.call(WORD_AT_4, &args![entry]).u32() != 0 {
                    let number = e.call(WORD_AT_4, &args![entry]).i32();
                    total = number.wrapping_mul(value).wrapping_add(total);
                }
            }
        }
        node = list_next(e, node);
    }
    total
}

// Translated from 004d1360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at +0x7bc of the player character is 1 (the value that
/// selects the weight code path the weight functions pass on as their flag).
pub fn fn_004d1360(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + PLAYER_WEIGHT_MODE) == 1
}

// Translated from 004d1180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weight of the worn items of the changes (the map has no name): the
/// entries that have a worn extra list (`GetWorn(0)`) and are not of form type
/// 0x29, each at its weight (weapons with the weapon weight modifier) times
/// the number the container holds (`fn_004c8f30`), but a weapon the actor's
/// process says is the same form for both its virtual 0x148 and 0x14c
/// results counts once. Returns the `f32` total (ST0). `actor` is the stack
/// word.
pub fn fn_004d1180(e: &mut Engine, this: Ptr<InventoryChanges>, actor: u32) -> f32 {
    let mut total = 0.0f32;
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && list_item(e, node) != 0 {
        let entry = list_item(e, node);
        let form = e.mem.u32(entry + 8);
        if item_change_get_worn(e, Ptr::new(entry), 0) && form_type_of(e, form) != 0x29 {
            let mut weight;
            if form_type_of(e, form) == FORM_TYPE_WEAPON {
                let with_mod = item_change_has_mod_effect_active_ov2(e, Ptr::new(entry), 4);
                weight = tes_object_weap_get_form_weight(e, Ptr::new(form), with_mod as u8);
                weight = heavy_weapon_adjusted(e, weight);
            } else {
                let player = e.global::<u32>(PLAYER_GLOBAL);
                let mode = fn_004d1360(e, Ptr::new(player));
                weight = e
                    .call(WEIGHT_FORM_GET_WEIGHT, &args![form, mode as u32])
                    .f32();
            }
            let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
            let mut count = fn_004c8f30(e, this, entry_form);
            if form_type_of(e, form) == FORM_TYPE_ARMOR {
                // The result of this cast is not used.
                e.call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_OBJECT, TYPE_TES_OBJECT_ARMO, 0u32],
                );
            } else if form_type_of(e, form) == FORM_TYPE_WEAPON {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                if process != 0 {
                    let first = e.vcall(process, 0x148, &args![]).u32();
                    let second = e.vcall(process, 0x14c, &args![]).u32();
                    if first != 0
                        && second != 0
                        && e.call(WORD_AT_8, &args![first]).u32()
                            == e.call(WORD_AT_8, &args![second]).u32()
                    {
                        count = 1;
                    }
                }
            }
            total = (count as f64 * weight as f64 + total as f64) as f32;
        }
        node = list_next(e, node);
    }
    total
}

// Translated from 004d1380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any extra data list of any entry has the leveled item position
/// (`0041d360`) `position`.
pub fn fn_004d1380(e: &mut Engine, this: Ptr<InventoryChanges>, position: i32) -> bool {
    let mut found = false;
    let mut node = e.mem.u32(this.addr());
    while node != 0
        && !e.call(LIST_IS_EMPTY, &args![node]).bool()
        && list_item(e, node) != 0
        && !found
    {
        let entry = list_item(e, node);
        if entry != 0 {
            let mut cursor = e.mem.u32(entry);
            while cursor != 0 && list_item(e, cursor) != 0 {
                let extra = list_item(e, cursor);
                if extra != 0 && e.call(EXTRA_LEVELED_POSITION, &args![extra]).i32() == position {
                    found = true;
                }
                cursor = list_next(e, cursor);
            }
        }
        node = list_next(e, node);
    }
    found
}

/// `TESContainer` temporary (12 bytes on the stack): constructor `00481610`
/// (`TESContainer::TESContainer`, Xbox PDB), destructor `00481680`, and the
/// methods `fn_004d1440` calls on it: `00487f70(leveled item + 0x30, level,
/// |count|, container, 0)` fills it from a leveled item, `00482090(container,
/// float)` scales its counts, `00482770(container, index, changes)` adds its
/// map's name, a getter of the float at +8 of the object it is called on)
/// and the double `1.0` at `0x01012070`.
/// map's name, a getter of the float at +8 of the object it is called on)
pub(crate) const CONTAINER_TEMP_CONSTRUCT: u32 = 0x0048_1610;
pub(crate) const CONTAINER_TEMP_DESTRUCT: u32 = 0x0048_1680;
pub(crate) const LEVELED_ITEM_FILL_CONTAINER: u32 = 0x0048_7f70;
pub(crate) const CONTAINER_SCALE_COUNTS: u32 = 0x0048_2090;
pub(crate) const CONTAINER_ADD_TO_CHANGES: u32 = 0x0048_2770;
pub(crate) const LEVELED_ITEM_CHANCE: u32 = 0x0048_8d50;
pub(crate) const ONE_DOUBLE: u32 = 0x0101_2070;
/// `abs(int)` (`__cdecl`, `00ec7d40`), the actor's level (`0087f9f0`, a
/// 16-bit value, on the owner of the changes), `TESObjectREFR::GetCalcLevel`
/// (Xbox PDB, `this`, one flag word) and the call `fn_004d1610` makes on the
/// third word of a container object: `0040ea20(word, extra, healthForm)`.
pub(crate) const ABS_INT: u32 = 0x00ec_7d40;
pub(crate) const ACTOR_LEVEL: u32 = 0x0087_f9f0;
pub(crate) const REFERENCE_GET_CALC_LEVEL: u32 = 0x0056_7e10;
pub(crate) const CONTAINER_OBJECT_ATTACH_EXTRA: u32 = 0x0040_ea20;
/// The form type byte of a leveled item form in a container (0x34).
pub(crate) const FORM_TYPE_LEVELED_ITEM: u32 = 0x34;

// Translated from 004d1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Expands the leveled items of the owner container into the changes (the
/// map has no name): for each object of the owner's container whose form is of
/// type 0x34 (a leveled item), when no extra list of the changes already has
/// that object's index as its leveled item position (`fn_004d1380`), a
/// temporary `TESContainer` is filled from the leveled item for the owner's
/// level (an actor's `0087f9f0` level, else `GetCalcLevel(1)`; 16 bits), its
/// counts are scaled when the item's third word gives a chance other than
/// `1.0`, and it is added to the changes under the index. The index counts
/// the leveled objects. The C++ exception frame is not translated.
pub fn fn_004d1440(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    let mut index = 0i32;
    let owner = e.call(WORD_AT_4, &args![this]).u32();
    let level = if e.vcall(owner, 0x100, &args![]).bool() {
        let owner = e.call(WORD_AT_4, &args![this]).u32();
        e.call(ACTOR_LEVEL, &args![owner]).u32() & 0xffff
    } else {
        let owner = e.call(WORD_AT_4, &args![this]).u32();
        e.call(REFERENCE_GET_CALC_LEVEL, &args![owner, 1u32]).u32()
    };
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let mut leveled = 0u32;
        let form = e.mem.u32(object + 4);
        if form != 0 && form_type_of(e, form) == FORM_TYPE_LEVELED_ITEM {
            leveled = e.mem.u32(object + 4);
        }
        if leveled != 0 {
            if !fn_004d1380(e, this, index) {
                e.with_stack(0xc, |e, temporary| {
                    e.call(CONTAINER_TEMP_CONSTRUCT, &args![temporary]);
                    let count = e.mem.u32(object);
                    let magnitude = e.call(ABS_INT, &args![count]).u32();
                    e.call(
                        LEVELED_ITEM_FILL_CONTAINER,
                        &args![leveled + 0x30, level & 0xffff, magnitude, temporary, 0u32],
                    );
                    let third = e.mem.u32(object + 8);
                    if object != 0 && third != 0 {
                        let chance = e.call(LEVELED_ITEM_CHANCE, &args![third]).f32();
                        if chance as f64 != e.global::<f64>(ONE_DOUBLE) {
                            let chance = e.call(LEVELED_ITEM_CHANCE, &args![third]).f32();
                            e.call(CONTAINER_SCALE_COUNTS, &args![temporary, chance.to_bits()]);
                        }
                    }
                    e.call(CONTAINER_ADD_TO_CHANGES, &args![temporary, index, this]);
                    e.call(CONTAINER_TEMP_DESTRUCT, &args![temporary]);
                });
            }
            index += 1;
        }
        node = list_next(e, node);
    }
}

// Translated from 004d1610 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each object of the owner container whose form is not a leveled item
/// (type 0x34) and that has a third word: a new extra data list with the
/// object's count (`SetCount`), attached to the third word together with the
/// form's health form (`0040ea20`), added to the lists of a new `ItemChange`
/// for the form (number 0), which is merged into the changes
/// (`fn_004c3380`, deleting the source). The C++ exception frame is not
/// translated.
pub fn fn_004d1610(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        node = list_next(e, node);
        let form = e.mem.u32(object + 4);
        if form != 0
            && form_type_of(e, form) != FORM_TYPE_LEVELED_ITEM
            && e.mem.u32(object + 8) != 0
        {
            let extra = new_extra_list(e);
            let count = e.mem.u16(object) as u32;
            e.call(EXTRA_SET_COUNT, &args![extra, count]);
            let health_form = e.call(GET_FORM_AS_HEALTH_FORM, &args![form]).u32();
            let third = e.mem.u32(object + 8);
            e.call(
                CONTAINER_OBJECT_ATTACH_EXTRA,
                &args![third, extra, health_form],
            );
            let item = new_item_change(e, form, 0);
            let list = e.mem.u32(item);
            list_call_with_item(e, LIST_ADD, list, extra);
            fn_004c3380(e, this, item, 1);
        }
    }
}

// Translated from 004d17a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the leveled item extra lists out of the changes (the map has no
/// name): the owner is told (virtual 0x48 with `0x8000000`), then in every
/// entry each extra list that has a leveled item (`GetLeveledItem`) is
/// cleaned (`0041d000`), the entry's number reduced by the list's count, the
/// list unlinked and deleted, and the walk restarted at the entry's first
/// list. An entry left with an empty list head and number 0 is unlinked from
/// the changes and deleted, and the walk restarts.
pub fn fn_004d17a0(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let owner = e.get(this, InventoryChanges::pRef).addr();
    e.vcall(owner, 0x48, &args![0x0800_0000u32]);
    let mut node = e.mem.u32(this.addr());
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && list_item(e, node) != 0 {
        let entry = list_item(e, node);
        let mut inner = e.mem.u32(entry);
        while inner != 0 && list_item(e, inner) != 0 {
            let extra = list_item(e, inner);
            if extra == 0 {
                inner = list_next(e, inner);
            } else if e.call(EXTRA_GET_LEVELED_ITEM, &args![extra]).u32() != 0 {
                e.call(EXTRA_FN_0041D000, &args![extra]);
                let number = e.call(WORD_AT_4, &args![entry]).i32();
                let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                e.call(ITEM_SET_NUMBER, &args![entry, number.wrapping_sub(held)]);
                list_call_with_item(e, LIST_REMOVE, inner, extra);
                delete_object(e, extra);
                inner = e.mem.u32(entry);
            } else {
                inner = list_next(e, inner);
            }
        }
        let lists = e.mem.u32(entry);
        if entry != 0
            && lists != 0
            && e.call(LIST_IS_EMPTY, &args![lists]).bool()
            && e.call(WORD_AT_4, &args![entry]).u32() == 0
        {
            let head = e.mem.u32(this.addr());
            list_call_with_item(e, LIST_REMOVE, head, entry);
            delete_item_change(e, entry);
            node = e.mem.u32(this.addr());
        } else {
            node = list_next(e, node);
        }
    }
}

/// Runs `script` on a new temporary reference with the script variables of
/// the extra list `extra`, then deletes the reference (virtual 0x10).
fn run_script_on_temporary_reference(e: &mut Engine, script: u32, extra: u32) {
    let reference = new_temporary_reference(e);
    let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![extra]).u32();
    e.call(SCRIPT_RUN, &args![script, reference, locals, 0u32, 0u32]);
    if reference != 0 {
        e.vcall(reference, 0x10, &args![1u32]);
    }
}

/// Gives the extra list `extra` the script `script` and its event list, then
/// runs the script on it (`run_script_on_temporary_reference`).
fn give_script_to_list(e: &mut Engine, extra: u32, script: u32) {
    e.call(EXTRA_SET_SCRIPT, &args![extra, script]);
    attach_script_events(e, extra);
    run_script_on_temporary_reference(e, script, extra);
}

/// Appends `count` new extra lists (count 1 each) to the lists of `entry`;
/// each one that has no script yet gets `script`.
fn append_scripted_lists(e: &mut Engine, entry: u32, count: i32, script: u32) {
    for _ in 0..count {
        let created = new_extra_list(e);
        let head = e.mem.u32(entry);
        list_call_with_item(e, LIST_ADD, head, created);
        e.call(EXTRA_SET_COUNT, &args![created, 1u32]);
        if created != 0 && e.call(EXTRA_GET_SCRIPT, &args![created]).u32() == 0 {
            give_script_to_list(e, created, script);
        }
    }
}

/// Gives every extra list of `entry` without a script the script `script`
/// (running it); returns how many it did. An entry without a list gets an
/// empty list head instead.
fn script_lists_of_entry(e: &mut Engine, entry: u32, script: u32) -> i32 {
    let mut given = 0;
    if e.mem.u32(entry) == 0 {
        let node = new_list_node(e);
        e.mem.set_u32(entry, node);
    } else {
        let mut cursor = e.mem.u32(entry);
        while cursor != 0 && list_item(e, cursor) != 0 {
            let extra = list_item(e, cursor);
            if extra != 0 && e.call(EXTRA_GET_SCRIPT, &args![extra]).u32() == 0 {
                give_script_to_list(e, extra, script);
                given += 1;
            }
            cursor = list_next(e, cursor);
        }
    }
    given
}

/// Runs `script` once for each extra list of `entry`; returns how many.
fn run_script_for_lists_of_entry(e: &mut Engine, entry: u32, script: u32) -> i32 {
    let mut ran = 0;
    let mut cursor = e.mem.u32(entry);
    while cursor != 0 && list_item(e, cursor) != 0 {
        let extra = list_item(e, cursor);
        if extra != 0 {
            run_script_on_temporary_reference(e, script, extra);
            ran += 1;
        }
        cursor = list_next(e, cursor);
    }
    ran
}

// Translated from 004d1960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the stacks of items that have a script their scripts (the map has
/// no name). For each object of the owner container whose form has a script
/// (`GetFormScript`) and a count above 0 (`abs`): the changes' entry for the
/// form, made when missing, gets the script on each extra list without one
/// (each run on a temporary reference, one less to make for each) and then
/// as many new extra lists (count 1) as remain; a made entry is merged into
/// the changes (`fn_004c3380`). An entry that already has a script
/// (`ItemChange::GetScript`) only has the script run once for each of its
/// lists. Then the same is done for every entry of the changes (a null entry
/// counts as having a script, so nothing is done for it). The C++ exception
/// frame is not translated.
pub fn fn_004d1960(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let object = list_item(e, node);
        let form = e.mem.u32(object + 4);
        let script = e.call(FORM_SCRIPT_OF, &args![form]).u32();
        let raw_count = e.mem.u32(object);
        let mut count = e.call(ABS_INT, &args![raw_count]).i32();
        if script != 0 && count > 0 {
            let mut entry = inventory_changes_get_object_in_list(e, this, form, 1, 0);
            if entry == 0 || item_change_get_script(e, Ptr::new(entry)) == 0 {
                let mut created = false;
                if entry == 0 {
                    created = true;
                    entry = new_item_change(e, form, 0);
                }
                count -= script_lists_of_entry(e, entry, script);
                if count > 0 {
                    append_scripted_lists(e, entry, count, script);
                }
                if created {
                    fn_004c3380(e, this, entry, 1);
                }
            } else if entry != 0 {
                run_script_for_lists_of_entry(e, entry, script);
            }
        }
        node = list_next(e, node);
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 && list_item(e, node) != 0 {
        let entry = list_item(e, node);
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        let script = e.call(FORM_SCRIPT_OF, &args![form]).u32();
        let number = e.call(WORD_AT_4, &args![entry]).u32();
        let count = e.call(ABS_INT, &args![number]).i32();
        if script != 0 && count > 0 {
            if entry == 0 || item_change_get_script(e, Ptr::new(entry)) != 0 {
                if entry != 0 {
                    run_script_for_lists_of_entry(e, entry, script);
                }
            } else {
                let remaining = count - script_lists_of_entry(e, entry, script);
                append_scripted_lists(e, entry, remaining, script);
            }
        }
        node = list_next(e, node);
    }
}

/// The critical section `fn_004d2480` takes (`0040fbf0` with the name 0,
/// `0040fba0` to leave) and the temporary reference methods it uses on the
/// scratch reference `TEMP_REF_GLOBAL`: `00568bb0(reference, extra)` is
/// `TESObjectREFR::SetExtra` (Xbox PDB), `00575690(reference, form)` is
/// `TESObjectREFR::SetObjectReference` (Xbox PDB), `0087ce80(reference, cell)`
/// sets its parent cell, `00436aa0(owner)` gives the word `0049eea0(reference,
/// word)` stores, `RemoveAllCopyableExtra(list, flag)` is `00411fd0` (Xbox
/// PDB) and `004013e0(script)` tests a script object.
pub(crate) const RUN_SCRIPTS_LOCK_OBJECT: u32 = 0x011c_64e0;
pub(crate) const SECTION_LOCK: u32 = 0x0040_fbf0;
pub(crate) const SECTION_UNLOCK: u32 = 0x0040_fba0;
pub(crate) const REFERENCE_SET_EXTRA: u32 = 0x0056_8bb0;
pub(crate) const REFERENCE_SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
pub(crate) const REFERENCE_SET_PARENT_CELL: u32 = 0x0087_ce80;
pub(crate) const OWNER_STORED_WORD: u32 = 0x0043_6aa0;
pub(crate) const REFERENCE_STORE_OWNER_WORD: u32 = 0x0049_eea0;
pub(crate) const EXTRA_REMOVE_ALL_COPYABLE: u32 = 0x0041_1fd0;
pub(crate) const SCRIPT_OBJECT_TEST: u32 = 0x0040_13e0;
/// The form type byte of a script object (0x11).
pub(crate) const FORM_TYPE_SCRIPT: u32 = 0x11;

// Translated from 004d2480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::RunScripts` (Xbox PDB): runs the script of every extra
/// data list of the changes on the scratch reference (`TEMP_REF_GLOBAL`), set
/// up with the list (`SetExtra`), the entry's form (`SetObjectReference`) and
/// the cell of `owner`; returns whether any run returned true. `bcountdirty`
/// is cleared first and, when a script sets it again, the walk of that entry's
/// lists stops and the walk of the entries restarts after the first node (the
/// game's behaviour). Nothing is done without an `owner` or a cell. The C++
/// exception frame is not translated.
pub fn inventory_changes_run_scripts(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    owner: u32,
) -> bool {
    e.set(this, InventoryChanges::bcountdirty, false);
    if owner == 0 {
        return false;
    }
    let cell = e.call(REFERENCE_PARENT_CELL, &args![owner]).u32();
    if cell == 0 {
        return false;
    }
    e.call(SECTION_LOCK, &args![RUN_SCRIPTS_LOCK_OBJECT, 0u32]);
    let mut node = e.mem.u32(this.addr());
    let mut ran = false;
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_OPEN,
            &args![guard, 0x15u32, 1u32, SOURCE_FILE_NAME, 0x264bu32],
        );
        let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
        e.call(REFERENCE_SET_PARENT_CELL, &args![temporary, cell]);
        let stored = e.call(OWNER_STORED_WORD, &args![owner]).u32();
        let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
        e.call(REFERENCE_STORE_OWNER_WORD, &args![temporary, stored]);
        while node != 0 && list_item(e, node) != 0 {
            let entry = list_item(e, node);
            let mut inner = e.mem.u32(entry);
            while inner != 0 && list_item(e, inner) != 0 {
                let extra = list_item(e, inner);
                if extra != 0 {
                    let script = e.call(EXTRA_GET_SCRIPT, &args![extra]).u32();
                    if script != 0 && e.call(SCRIPT_OBJECT_TEST, &args![script]).u8() != 0 {
                        if form_type_of(e, script) == FORM_TYPE_SCRIPT {
                            let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
                            e.call(REFERENCE_SET_EXTRA, &args![temporary, extra]);
                            let form = e.call(WORD_AT_8, &args![entry]).u32();
                            let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
                            e.call(REFERENCE_SET_OBJECT_REFERENCE, &args![temporary, form]);
                            let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![extra]).u32();
                            let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
                            let result = e
                                .call(SCRIPT_RUN, &args![script, temporary, locals, owner, 0u32])
                                .u8();
                            if result != 0 {
                                ran = true;
                            }
                            let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
                            let list = e.call(REFR_EXTRA_LIST, &args![temporary]).u32();
                            e.call(EXTRA_SET_SCRIPT_EVENTS, &args![list, 0u32]);
                            let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
                            let list = e.call(REFR_EXTRA_LIST, &args![temporary]).u32();
                            e.call(EXTRA_REMOVE_ALL_COPYABLE, &args![list, 1u32]);
                        }
                        if e.get(this, InventoryChanges::bcountdirty) {
                            break;
                        }
                    }
                }
                inner = list_next(e, inner);
            }
            if e.get(this, InventoryChanges::bcountdirty) {
                node = e.mem.u32(this.addr());
                e.set(this, InventoryChanges::bcountdirty, false);
            }
            node = list_next(e, node);
        }
        let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
        e.call(REFERENCE_SET_PARENT_CELL, &args![temporary, 0u32]);
        let temporary = e.global::<u32>(TEMP_REF_GLOBAL);
        e.call(REFERENCE_SET_OBJECT_REFERENCE, &args![temporary, 0u32]);
        e.call(SCOPE_GUARD_CLOSE, &args![guard]);
    });
    e.call(SECTION_UNLOCK, &args![RUN_SCRIPTS_LOCK_OBJECT]);
    ran
}

/// `00418550(extra list, owner)` (`__thiscall`): sets the owner (the extra
/// of type 0x20, made when the list has none) of an extra data list; the
/// map has no name for it.
pub(crate) const EXTRA_SET_OWNER_EXTRA: u32 = 0x0041_8550;
/// The extra data type `fn_004d26d0` looks for in the lists (0x2f).
pub(crate) const EXTRA_TYPE_0X2F: u32 = 0x2f;

/// `new ItemChange()` through `ItemChange_ov3`: 0xC bytes, constructed unless
/// the allocation failed.
fn new_empty_item_change(e: &mut Engine) -> u32 {
    let item = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if item == 0 {
        return 0;
    }
    item_change_item_change_ov3(e, Ptr::new(item)).addr()
}

/// A new extra data list owned by `owner` (`00418550`) holding `count` items
/// (`SetCount`), added to the lists of `item` (`fn_004d26d0`).
fn add_counted_list(e: &mut Engine, item: u32, owner: u32, count: u32) {
    let extra = new_extra_list(e);
    e.call(EXTRA_SET_OWNER_EXTRA, &args![extra, owner]);
    e.call(EXTRA_SET_COUNT, &args![extra, count]);
    let head = e.mem.u32(item);
    list_call_with_item(e, LIST_ADD, head, extra);
}

/// The end of the worn case of `fn_004d26d0`: the original reference extra of
/// the first extra list of `item` is removed and the item deleted.
fn drop_worn_item(e: &mut Engine, item: u32) {
    let head = e.mem.u32(item);
    let first = list_item(e, head);
    if first != 0 {
        e.call(EXTRA_REMOVE_ORIGINAL_REFERENCE, &args![first]);
    }
    delete_item_change(e, item);
}

// Translated from 0076b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemChange::ItemChange_ov3` (Xbox PDB): the constructor that zeroes the
/// extra list pointer, the number and the form.
pub fn item_change_item_change_ov3(e: &mut Engine, this: Ptr<ItemChange>) -> Ptr<ItemChange> {
    e.set(this, ItemChange::pExtraObjectList, Ptr::NULL);
    e.set(this, ItemChange::iNumber, 0);
    e.set(this, ItemChange::pContainerObj, Ptr::NULL);
    this
}

// Translated from 004d26d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the contents of this inventory into the changes `target` (the map
/// has no name; `owner` is the owner extra given to the new extra lists and
/// `skip_excluded` skips forms whose virtual 0x94 is true). First the owner
/// container's objects that are not leveled items (`TESLevItem` by dynamic
/// cast): with the changes' entry for the form (its original changes removed
/// first) an entry that is a plain container item (number plus count 0, a
/// worn or skipped one) is dropped; with no extra lists in the entry a new item
/// of `|count|` (less what the entry's lists hold) is made, getting the
/// entry's lists when the entry has a script, else one counted list; with
/// extra lists each is given the owner and moved over one by one, then a list
/// of the remainder. Every made item is merged into `target`
/// (`fn_004c3380`, deleting the source); one that is worn is deleted. Then the
/// same for every entry of the changes the owner container does not list,
/// whose number is above 0 (moving its lists, or one counted list, keeping the
/// extra of type 0x2f and the class-4 forms apart as the code shows). The scope
/// guard of line 0x26b7 is kept around it all. The C++ exception frame is not
/// translated.
pub fn fn_004d26d0(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    target: Ptr<InventoryChanges>,
    owner: u32,
    skip_excluded: u8,
) {
    with_scope_guard(e, 0x26b7, |e| {
        let container = fn_004bffb0(e, this);
        let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        while node != 0 && list_item(e, node) != 0 {
            let object = list_item(e, node);
            let object_form = e.mem.u32(object + 4);
            let leveled = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        object_form,
                        0u32,
                        TYPE_TES_BOUND_OBJECT,
                        TYPE_TES_LEV_ITEM,
                        0u32
                    ],
                )
                .u32();
            let item = new_empty_item_change(e);
            let container = fn_004bffb0(e, this);
            let count = e
                .call(CONTAINER_COUNT, &args![container, object_form])
                .i32();
            ensure_extra_list(e, item);
            if object == 0 || leveled != 0 {
                delete_item_change(e, item);
            } else {
                let entry = inventory_changes_get_object_in_list(e, this, object_form, 1, 0);
                if entry != 0 {
                    item_change_remove_original_changes(e, Ptr::new(entry));
                }
                let mut proceed = true;
                if entry != 0 {
                    let keep = (fn_004bcb70(e, Ptr::new(entry))
                        && item_change_number_leveled_extra(e, Ptr::new(entry)) >= 1)
                        || entry_number(e, entry).wrapping_add(count) != 0;
                    if keep {
                        let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
                        let excluded = e.vcall(entry_form, FORM_EXCLUDED_SLOT, &args![]).bool();
                        if (excluded && skip_excluded != 0)
                            || item_change_get_worn(e, Ptr::new(entry), 0)
                        {
                            proceed = false;
                        }
                    } else {
                        proceed = false;
                    }
                    if !proceed {
                        delete_item_change(e, item);
                    }
                }
                if proceed {
                    let has_extras = entry != 0
                        && item_change_get_extra_total_count(e, Ptr::new(entry), false) != 0;
                    if !has_extras {
                        // The entry holds no extra lists of its own.
                        let magnitude = count.wrapping_abs();
                        if entry != 0 {
                            let held = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                            let number = magnitude
                                .wrapping_add(entry_number(e, entry))
                                .wrapping_sub(held);
                            set_entry_number(e, item, number);
                        } else {
                            set_entry_number(e, item, magnitude);
                        }
                        e.call(ITEM_SET_FORM, &args![item, object_form]);
                        if entry != 0 && item_change_get_script(e, Ptr::new(entry)) != 0 {
                            let mut cursor = e.call(WORD_AT_0, &args![entry]).u32();
                            while cursor != 0 && list_item(e, cursor) != 0 {
                                let extra = list_item(e, cursor);
                                e.call(EXTRA_SET_OWNER_EXTRA, &args![extra, owner]);
                                let head = e.mem.u32(item);
                                list_call_with_item(e, LIST_ADD_TAIL, head, extra);
                                cursor = list_next(e, cursor);
                            }
                            fn_004c3380(e, target, item, 1);
                        } else if !item_change_get_worn(e, Ptr::new(item), 0) {
                            let number = e.call(WORD_AT_4, &args![item]).u32();
                            add_counted_list(e, item, owner, number);
                            fn_004c3380(e, target, item, 1);
                        } else {
                            delete_item_change(e, item);
                        }
                    } else {
                        // The entry has extra lists of its own.
                        if e.mem.u32(entry) != 0 {
                            item_change_get_extra_total_count(e, Ptr::new(entry), false);
                        }
                        let mut cursor = e.mem.u32(entry);
                        // The new item is never null here, so this loop does
                        // not run in the game.
                        while cursor != 0 && list_item(e, cursor) != 0 && item == 0 {
                            let extra = list_item(e, cursor);
                            let entry_form = e.mem.u32(entry + 8);
                            e.call(ITEM_SET_FORM, &args![item, entry_form]);
                            let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                            e.call(ITEM_SET_NUMBER, &args![item, held]);
                            ensure_extra_list(e, item);
                            e.call(EXTRA_SET_OWNER_EXTRA, &args![extra, owner]);
                            let head = e.mem.u32(item);
                            list_call_with_item(e, LIST_ADD, head, extra);
                            if !item_change_get_worn(e, Ptr::new(item), 0) {
                                fn_004c3380(e, target, item, 1);
                            }
                            cursor = list_next(e, cursor);
                        }
                        let held = item_change_get_extra_total_count(e, Ptr::new(entry), false);
                        let wanted = count.wrapping_add(entry_number(e, entry));
                        if held < wanted {
                            ensure_extra_list(e, item);
                            if entry != 0 {
                                let held =
                                    item_change_get_extra_total_count(e, Ptr::new(entry), false);
                                let number = count
                                    .wrapping_add(entry_number(e, entry))
                                    .wrapping_sub(held);
                                set_entry_number(e, item, number);
                            } else {
                                set_entry_number(e, item, count);
                            }
                            e.call(ITEM_SET_FORM, &args![item, object_form]);
                            if !item_change_get_worn(e, Ptr::new(item), 0) {
                                let number = e.call(WORD_AT_4, &args![item]).u32();
                                add_counted_list(e, item, owner, number);
                                fn_004c3380(e, target, item, 1);
                            } else {
                                delete_item_change(e, item);
                            }
                        }
                    }
                }
            }
            node = list_next(e, node);
        }
        let mut node = e.mem.u32(this.addr());
        while node != 0 && list_item(e, node) != 0 {
            let item = new_empty_item_change(e);
            ensure_extra_list(e, item);
            let entry = list_item(e, node);
            if entry == 0 || e.call(WORD_AT_4, &args![entry]).i32() < 1 {
                delete_item_change(e, item);
            } else {
                let container = fn_004bffb0(e, this);
                let entry_form = e.mem.u32(entry + 8);
                let container_again = fn_004bffb0(e, this);
                if container != 0
                    && e.call(CONTAINER_HAS_FORM, &args![container_again, entry_form])
                        .bool()
                {
                    delete_item_change(e, item);
                } else if item_change_get_extra_total_count(e, Ptr::new(entry), false) == 0
                    && item_change_get_script(e, Ptr::new(entry)) == 0
                {
                    // No extra lists and no script on the entry.
                    let mut found = 0u32;
                    if e.mem.u32(entry) != 0 {
                        let mut cursor = e.mem.u32(entry);
                        let mut sum = 0i32;
                        while cursor != 0 && list_item(e, cursor) != 0 && found == 0 {
                            found = list_item(e, cursor);
                            sum = sum.wrapping_add(
                                e.call(EXTRA_GET_COUNT, &args![found]).u16() as i16 as i32
                            );
                            if e.call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![found, 0u32])
                                .bool()
                            {
                                found = 0;
                            }
                            cursor = list_next(e, cursor);
                        }
                        if found != 0 {
                            ensure_extra_list(e, item);
                            e.call(EXTRA_SET_OWNER_EXTRA, &args![found, owner]);
                            let head = e.mem.u32(item);
                            list_call_with_item(e, LIST_ADD, head, found);
                            let number = e.call(WORD_AT_4, &args![entry]).i32();
                            if !item_change_get_worn(e, Ptr::new(item), 0) && sum < number {
                                add_counted_list(e, item, owner, number.wrapping_sub(sum) as u32);
                            }
                        } else {
                            let number = e.call(WORD_AT_4, &args![entry]).u32();
                            add_counted_list(e, item, owner, number);
                        }
                        e.call(ITEM_SET_FORM, &args![item, entry_form]);
                        let number = e.call(WORD_AT_4, &args![entry]).u32();
                        e.call(ITEM_SET_NUMBER, &args![item, number]);
                        if !item_change_get_worn(e, Ptr::new(item), 0) {
                            let number = e.call(WORD_AT_4, &args![entry]).u32();
                            e.call(ITEM_SET_NUMBER, &args![item, number]);
                            fn_004c3380(e, target, item, 1);
                        } else {
                            drop_worn_item(e, item);
                        }
                    } else {
                        let form = e.call(WORD_AT_8, &args![entry]).u32();
                        e.call(ITEM_SET_FORM, &args![item, form]);
                        let number = e.call(WORD_AT_4, &args![entry]).u32();
                        e.call(ITEM_SET_NUMBER, &args![item, number]);
                        let number = e.call(WORD_AT_4, &args![item]).u32();
                        add_counted_list(e, item, owner, number);
                        fn_004c3380(e, target, item, 1);
                    }
                } else {
                    // The entry has extra lists or a script.
                    let mut moved = 0i32;
                    let mut has_type_0x2f = false;
                    let mut cursor = e.mem.u32(entry);
                    while cursor != 0 && list_item(e, cursor) != 0 {
                        let extra = list_item(e, cursor);
                        if e.call(EXTRA_GET_BY_TYPE, &args![extra, EXTRA_TYPE_0X2F])
                            .u32()
                            != 0
                        {
                            has_type_0x2f = true;
                        }
                        cursor = list_next(e, cursor);
                    }
                    cursor = e.mem.u32(entry);
                    if !(has_type_0x2f && fn_004be1a0(e, Ptr::new(entry)) == 4) {
                        while cursor != 0 && list_item(e, cursor) != 0 {
                            let extra = list_item(e, cursor);
                            e.call(EXTRA_SET_OWNER_EXTRA, &args![extra, owner]);
                            let head = e.mem.u32(item);
                            list_call_with_item(e, LIST_ADD, head, extra);
                            cursor = list_next(e, cursor);
                            moved =
                                moved.wrapping_add(e.call(EXTRA_GET_COUNT, &args![extra]).u16()
                                    as i16
                                    as i32);
                        }
                    } else {
                        if fn_004be1a0(e, Ptr::new(entry)) == 4
                            && item_change_get_script(e, Ptr::new(entry)) != 0
                        {
                            e.call(ITEM_SET_NUMBER, &args![entry, 1u32]);
                        }
                        while cursor != 0 && list_item(e, cursor) != 0 {
                            let extra = list_item(e, cursor);
                            e.call(EXTRA_REMOVE_COUNT, &args![extra]);
                            e.call(EXTRA_SET_OWNER_EXTRA, &args![extra, owner]);
                            let head = e.mem.u32(item);
                            list_call_with_item(e, LIST_ADD, head, extra);
                            cursor = list_next(e, cursor);
                            moved = 1;
                        }
                    }
                    let entry_form = e.mem.u32(entry + 8);
                    e.call(ITEM_SET_FORM, &args![item, entry_form]);
                    let number = e.call(WORD_AT_4, &args![entry]).u32();
                    e.call(ITEM_SET_NUMBER, &args![item, number]);
                    let number = e.call(WORD_AT_4, &args![entry]).i32();
                    if !item_change_get_worn(e, Ptr::new(item), 0) && moved < number {
                        add_counted_list(e, item, owner, number.wrapping_sub(moved) as u32);
                    }
                    if !item_change_get_worn(e, Ptr::new(item), 0) {
                        fn_004c3380(e, target, item, 1);
                    } else {
                        drop_worn_item(e, item);
                    }
                }
            }
            node = list_next(e, node);
        }
    });
}

// Translated from 004d3660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::ClearAllChangeItems` (Xbox PDB): empties the changes.
/// Each entry's extra lists are popped one by one from the head; a list that
/// has an original reference (`GetOriginalReference`) loses that extra; when
/// the reference has no container the list is deleted unless it has a script;
/// when it has one the list also loses its count when it holds nothing else,
/// is unlinked again, and is deleted unless the reference's own changes entry
/// for the form, or the player's, still lists it. Lists without an original
/// reference are left alone (not deleted). The entry is then unlinked from
/// the changes (from the node `[this]` the walk started at) and deleted, and
/// the walk restarts at that node.
pub fn inventory_changes_clear_all_change_items(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let mut node = e.mem.u32(this.addr());
    let first_node = e.mem.u32(this.addr());
    while node != 0 && list_item(e, node) != 0 {
        let entry = list_item(e, node);
        loop {
            let lists = e.mem.u32(entry);
            if lists == 0 || list_item(e, lists) == 0 {
                break;
            }
            let extra = list_item(e, lists);
            list_pop_head(e, lists);
            let reference = if extra != 0 {
                e.call(EXTRA_GET_ORIGINAL_REFERENCE, &args![extra]).u32()
            } else {
                0
            };
            if reference == 0 {
                continue;
            }
            if e.call(REFR_HAS_CONTAINER, &args![reference]).u32() == 0 {
                e.call(EXTRA_REMOVE_ORIGINAL_REFERENCE, &args![extra]);
                if e.call(EXTRA_GET_SCRIPT, &args![extra]).u32() == 0 {
                    delete_object(e, extra);
                }
                continue;
            }
            e.call(EXTRA_REMOVE_ORIGINAL_REFERENCE, &args![extra]);
            if e.call(EXTRA_ITEMS_IN_LIST, &args![extra]).u32() < 2
                && e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 != 0
            {
                e.call(EXTRA_REMOVE_COUNT, &args![extra]);
            }
            let lists = e.call(WORD_AT_0, &args![entry]).u32();
            list_call_with_item(e, LIST_REMOVE, lists, extra);
            let mut unlisted = true;
            let reference_extras = e.call(REFR_EXTRA_LIST, &args![reference]).u32();
            let changes = e
                .call(EXTRA_GET_CONTAINER_CHANGES, &args![reference_extras])
                .u32();
            unlisted = lists_hold_no(e, changes, entry, extra, unlisted);
            let player = e.global::<u32>(PLAYER_GLOBAL);
            let player_extras = e.call(REFR_EXTRA_LIST, &args![player]).u32();
            let changes = e
                .call(EXTRA_GET_CONTAINER_CHANGES, &args![player_extras])
                .u32();
            unlisted = lists_hold_no(e, changes, entry, extra, unlisted);
            if unlisted {
                delete_object(e, extra);
            }
        }
        list_call_with_item(e, LIST_REMOVE, first_node, entry);
        delete_item_change(e, entry);
        node = first_node;
    }
}

/// The check `ClearAllChangeItems` makes in a container's changes: `unlisted`
/// stays true unless the changes' entry for the form of `entry` has `extra`
/// among its lists.
fn lists_hold_no(e: &mut Engine, changes: u32, entry: u32, extra: u32, unlisted: bool) -> bool {
    let mut unlisted = unlisted;
    if changes != 0 {
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        let other = inventory_changes_get_object_in_list(e, Ptr::new(changes), form, 1, 0);
        if other != 0 && e.call(WORD_AT_0, &args![other]).u32() != 0 {
            let mut cursor = e.call(WORD_AT_0, &args![other]).u32();
            while cursor != 0 && list_item(e, cursor) != 0 && unlisted {
                if list_item(e, cursor) == extra {
                    unlisted = false;
                } else {
                    cursor = list_next(e, cursor);
                }
            }
        }
    }
    unlisted
}

// Translated from 004d3960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges` save size (the 16-bit size of `SaveGame`): 6 more bytes
/// when save game blocks are used, 2 for the number of entries and the save
/// size (`fn_004be5a0`) of every entry. With the size logging flag set the
/// size is reported through `Error` (the message names the form being saved
/// when there is one).
pub fn fn_004d3960(e: &mut Engine, this: Ptr<InventoryChanges>) -> u16 {
    let save = save_load_game(e);
    let mut size: u16 = 0;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(2);
    let mut cursor = e.mem.u32(this.addr());
    while cursor != 0 {
        if e.call(LIST_IS_EMPTY, &args![cursor]).bool() {
            break;
        }
        let entry = list_item(e, cursor);
        if entry != 0 {
            size = size.wrapping_add(fn_004be5a0(e, Ptr::new(entry)));
        }
        cursor = list_next(e, cursor);
    }
    if save_size_logging(e) {
        let current = e.call(CURRENT_SAVE_FORM, &args![save]).u32();
        log_save_size(e, size as u32, current, 0x284e, (0x0101_2cb0, 0x0101_2c78));
    }
    size
}

// Translated from 004d3ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges` save (to the save/load game object): with save game
/// blocks the block tag `BLOK` and a 16-bit length (patched at the end), then
/// a 16-bit number of entries (patched after them) and every non-null entry
/// saved with `fn_004be6f0`. With the logging flag set the bytes written are
/// reported through `Error`; a block over 0xFFFF bytes is reported through
/// `005b5e40`.
pub fn fn_004d3ab0(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let save = save_load_game(e);
    let mut start = e.call(SAVE_POSITION, &args![save]).u32();
    if save_size_logging(e) {
        start = e.call(SAVE_POSITION, &args![save]).u32();
    }
    // Locals of the game's frame passed by address: the tag, the block length
    // and the number of entries.
    e.with_stack(0xc, |e, locals| {
        let (tag, length_slot, count_slot) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        e.mem.set_u16(length_slot, 0);
        let mut block = 0u32;
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            e.mem.set_u32(tag, 0x424c_4f4b);
            e.call(SAVE_BYTES, &args![save, tag, 4u32]);
            block = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(SAVE_BYTES, &args![save, length_slot, 2u32]);
        }
        e.mem.set_u16(count_slot, 0);
        let count_position = e.call(SAVE_POSITION, &args![save]).u32();
        e.call(SAVE_BYTES, &args![save, count_slot, 2u32]);
        let mut cursor = e.mem.u32(this.addr());
        while cursor != 0 {
            if e.call(LIST_IS_EMPTY, &args![cursor]).bool() {
                break;
            }
            let entry = list_item(e, cursor);
            if entry != 0 {
                fn_004be6f0(e, Ptr::new(entry));
                let count = e.mem.u16(count_slot);
                e.mem.set_u16(count_slot, count.wrapping_add(1));
            }
            cursor = list_next(e, cursor);
        }
        let count = e.mem.u16(count_slot);
        e.mem.set_u16(count_position, count);
        if save_size_logging(e) {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            let current = e.call(CURRENT_SAVE_FORM, &args![save]).u32();
            log_save_size(
                e,
                end.wrapping_sub(start),
                current,
                0x286a,
                (0x0101_53a0, 0x0101_536c),
            );
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            if end > block.wrapping_add(0xffff) {
                e.call(
                    SAVE_LOAD_LOG,
                    &args![0x0101_5318u32, SOURCE_FILE_NAME, 0x286au32],
                );
            }
            e.mem.set_u16(block, end.wrapping_sub(block) as u16);
        }
    });
}

// Translated from 004d3cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::LoadGame` (Xbox PDB): reads what `fn_004d3ab0` wrote.
/// With save game blocks the `BLOK` tag is checked (a wrong tag is logged
/// through `005b5e40`) and the 16-bit length read; then the 16-bit number of
/// entries, each loaded into a new `ItemChange` (`ItemChange::LoadGame`) and
/// added to the changes, or, when its form could not be found, its extra
/// lists deleted and the entry deleted. Then `fn_004cbbc0` fixes the loaded
/// changes up and the block length is checked against the bytes read
/// (overrun and underrun are logged). The C++ exception frame is not
/// translated.
pub fn inventory_changes_load_game(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let save = save_load_game(e);
    let mut length = 0u16;
    let mut block = 0u32;
    let mut entries = 0u16;
    e.with_stack(0xc, |e, locals| {
        let (tag, length_slot, count_slot) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            e.call(LOAD_BYTES, &args![save, tag, 4u32]);
            if e.mem.u32(tag) != 0x424c_4f4b {
                let current = e.call(CURRENT_LOAD_FORM, &args![save]).u32();
                if current != 0 {
                    let form_id = e.mem.u32(current);
                    let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
                    let version = e.mem.u8(current + 9) as u32;
                    let flags = e.mem.u32(current + 5);
                    let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
                    e.call(
                        SAVE_LOAD_LOG,
                        &args![
                            0x0101_5718u32,
                            SOURCE_FILE_NAME,
                            0x2871u32,
                            form_id,
                            name,
                            version,
                            flags
                        ],
                    );
                } else {
                    let version = e.call(LOAD_VERSION, &args![save]).u8() as u32;
                    e.call(
                        SAVE_LOAD_LOG,
                        &args![0x0101_56a8u32, SOURCE_FILE_NAME, 0x2871u32, version],
                    );
                }
            }
            block = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(LOAD_BYTES, &args![save, length_slot, 2u32]);
            length = e.mem.u16(length_slot);
        }
        e.call(LOAD_BYTES, &args![save, count_slot, 2u32]);
        entries = e.mem.u16(count_slot);
    });
    for _ in 0..entries {
        let item = new_empty_item_change(e);
        item_change_load_game(e, Ptr::new(item));
        if e.call(WORD_AT_8, &args![item]).u32() != 0 {
            let list = e.mem.u32(this.addr());
            list_call_with_item(e, LIST_ADD, list, item);
        } else {
            item_change_delete_all_extra(e, Ptr::new(item));
            delete_item_change(e, item);
        }
    }
    fn_004cbbc0(e, this);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
        let position = e.call(SAVE_POSITION, &args![save]).u32();
        let current = e.call(CURRENT_LOAD_FORM, &args![save]).u32();
        let expected = (length as u32).wrapping_add(block);
        // The messages: "SAVELOAD: LoadGame Buffer overrun/underrun of %i
        // bytes in file %s on line %i.  Current version is %i", or with the
        // form being loaded.
        let (overrun, underrun) = if current != 0 {
            (0x0101_5588u32, 0x0101_5500u32)
        } else {
            (0x0101_54a0u32, 0x0101_5440u32)
        };
        let difference = if position > expected {
            Some((overrun, position - expected))
        } else if position < expected {
            Some((underrun, expected - position))
        } else {
            None
        };
        if current != 0 {
            let form_id = e.mem.u32(current);
            let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
            if let Some((format, bytes)) = difference {
                let version = e.mem.u8(current + 9) as u32;
                let flags = e.mem.u32(current + 5);
                let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
                e.call(
                    SAVE_LOAD_LOG,
                    &args![
                        format,
                        bytes,
                        SOURCE_FILE_NAME,
                        0x2884u32,
                        form_id,
                        name,
                        version,
                        flags
                    ],
                );
            }
        } else if let Some((format, bytes)) = difference {
            let version = e.call(LOAD_VERSION, &args![save]).u8() as u32;
            e.call(
                SAVE_LOAD_LOG,
                &args![format, bytes, SOURCE_FILE_NAME, 0x2884u32, version],
            );
        }
    }
}

/// `0046e8c0` (`__thiscall` on a weapon form): a test the inventory item
/// filters use on weapons; the map has no name for it.
pub(crate) const WEAPON_FILTER_TEST: u32 = 0x0046_e8c0;
/// The save/load buffer (`BGSSaveGameBuffer` / `BGSLoadGameBuffer`) setters
/// and getters `fn_004d4090`, `fn_004d4160` and `fn_004d42f0` use to switch a
/// buffer's mode words while they work: the word at +0x17 (get `00428110(buffer,
/// &out)`, set `00428130(buffer, value)`), +0x20 (`0050f9c0`), +0x24
/// (`007037c0`, the map's `BGSMenuPacker::RecomputePacking` for a folded
/// body) and +0x2c (get `0042ce30(buffer, &out)`, set `0086cf00`). Virtual
/// slot 0 (save buffers) or 4 (load buffers) returns the mode they restore.
pub(crate) const BUFFER_WORD_17_GET: u32 = 0x0042_8110;
pub(crate) const BUFFER_WORD_17_SET: u32 = 0x0042_8130;
pub(crate) const BUFFER_WORD_20_SET: u32 = 0x0050_f9c0;
pub(crate) const BUFFER_WORD_24_SET: u32 = 0x0070_37c0;
pub(crate) const BUFFER_WORD_2C_GET: u32 = 0x0042_ce30;
pub(crate) const BUFFER_WORD_2C_SET: u32 = 0x0086_cf00;
/// `NiMalloc(size)` and `NiFree(ptr)` (`__cdecl`) used by the `NiTMap`
/// constructor and destructor, the map's clear (`00438af0`, `__thiscall`) and
/// the two vtables of `NiTMapBase<TESObject *, bool>` (`01020944`) and its
/// derived `NiTMap<TESObject *, bool>` (`01020924`).
pub(crate) const NI_ALLOCATE: u32 = 0x00aa_1070;
pub(crate) const NI_FREE: u32 = 0x00aa_10f0;
pub(crate) const NI_MAP_CLEAR: u32 = 0x0043_8af0;
pub(crate) const NI_MAP_BASE_VTABLE: u32 = 0x0102_0944;
pub(crate) const NI_MAP_VTABLE: u32 = 0x0102_0924;
/// `0047bb50(itemForm, form)` (`__cdecl`): the test `GetRepairItemGroup`
/// asks about each item's form.
pub(crate) const REPAIR_ITEM_TEST: u32 = 0x0047_bb50;
/// The form type byte of a mod object (0x67).
pub(crate) const FORM_TYPE_WEAPON_MOD: u32 = 0x67;

/// The filter the item iterations of `fn_004d4530` and `fn_004d4830` apply
/// when bit 0 of their flags is set: a form is skipped when its virtual 0x94
/// says so, when it is a biped model that is not playable, or when it is a
/// weapon that `0046e8c0` rejects.
fn skipped_by_item_filter(e: &mut Engine, form: u32) -> bool {
    if e.vcall(form, FORM_EXCLUDED_SLOT, &args![]).bool() {
        return true;
    }
    let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
    if biped != 0 && !e.call(BIPED_PLAYABLE, &args![biped]).bool() {
        return true;
    }
    form_type_of(e, form) == FORM_TYPE_WEAPON && e.call(WEAPON_FILTER_TEST, &args![form]).u8() != 0
}

// Translated from 004d4030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `fn_004bed00` (the step after loading) on every non-null entry of
/// the changes; the walk stops at an empty head node.
pub fn fn_004d4030(e: &mut Engine, this: Ptr<InventoryChanges>) {
    let mut cursor = e.mem.u32(this.addr());
    while cursor != 0 {
        if e.call(LIST_IS_EMPTY, &args![cursor]).bool() {
            break;
        }
        let entry = list_item(e, cursor);
        if entry != 0 {
            fn_004bed00(e, Ptr::new(entry));
        }
        cursor = list_next(e, cursor);
    }
}

/// Runs `body` with the buffer modes `fn_004d4090`, `fn_004d4160` and
/// `fn_004d42f0` set (and restores them afterwards): saves the mode the
/// buffer's virtual `mode_slot` reports and the words +0x17 (and +0x2c for
/// the load buffers), sets the mode word (+0x20 or +0x24) to 0, +0x17 to
/// 0x400 (and +0x2c to 0).
fn with_buffer_modes<R>(
    e: &mut Engine,
    buffer: u32,
    loading: bool,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    let mode_setter = if loading {
        BUFFER_WORD_24_SET
    } else {
        BUFFER_WORD_20_SET
    };
    let mode_slot = if loading { 4 } else { 0 };
    let mode = e.vcall(buffer, mode_slot, &args![]).u32();
    let (kind, word_2c) = e.with_stack(8, |e, out| {
        e.call(BUFFER_WORD_17_GET, &args![buffer, out]);
        let kind = e.mem.u32(out.addr());
        let mut word_2c = 0;
        if loading {
            let second = out.addr() + 4;
            e.call(BUFFER_WORD_2C_GET, &args![buffer, second]);
            word_2c = e.mem.u32(second);
        }
        (kind, word_2c)
    });
    e.call(mode_setter, &args![buffer, 0u32]);
    e.call(BUFFER_WORD_17_SET, &args![buffer, 0x400u32]);
    if loading {
        e.call(BUFFER_WORD_2C_SET, &args![buffer, 0u32]);
    }
    let result = body(e);
    e.call(mode_setter, &args![buffer, mode]);
    e.call(BUFFER_WORD_17_SET, &args![buffer, kind]);
    if loading {
        e.call(BUFFER_WORD_2C_SET, &args![buffer, word_2c]);
    }
    result
}

// Translated from 004d4090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges` save to a buffer (the map has no name): with the
/// buffer's mode words switched (see `with_buffer_modes`), a variable sized
/// value holding the number of entries written
/// (`ItemChange::SaveGame(buffer)` for each non-null entry).
pub fn fn_004d4090(e: &mut Engine, this: Ptr<InventoryChanges>, buffer: u32) {
    with_buffer_modes(e, buffer, false, |e| {
        let mut written = 0u32;
        let start = e.call(SAVE_BUFFER_START_SIZED, &args![buffer]).u32();
        let mut cursor = e.mem.u32(this.addr());
        while cursor != 0 {
            let entry = list_item(e, cursor);
            if entry != 0 {
                item_change_save_game(e, Ptr::new(entry), buffer);
                written += 1;
            }
            cursor = list_next(e, cursor);
        }
        e.call(SAVE_BUFFER_END_SIZED, &args![buffer, written, start]);
    });
}

// Translated from 004d4160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges` load from a buffer (the map has no name): with the
/// buffer's mode words switched, the number of entries (a variable sized
/// value), each loaded into a new `ItemChange` (`LoadGame_ov2`); when its
/// form could not be found its extra lists are deleted and the item
/// deleted, and the (then null) item is added to the changes in any case
/// (the game's behaviour). The C++ exception frame is not translated.
pub fn fn_004d4160(e: &mut Engine, this: Ptr<InventoryChanges>, buffer: u32) {
    with_buffer_modes(e, buffer, true, |e| {
        let entries = e.call(LOAD_BUFFER_LOAD_SIZED, &args![buffer]).u32();
        for _ in 0..entries {
            let mut item = new_empty_item_change(e);
            item_change_load_game_ov2(e, Ptr::new(item), buffer);
            if e.call(WORD_AT_8, &args![item]).u32() == 0 {
                item_change_delete_all_extra(e, Ptr::new(item));
                delete_item_change(e, item);
                item = 0;
            }
            let list = e.mem.u32(this.addr());
            list_call_with_item(e, LIST_ADD, list, item);
        }
    });
}

// Translated from 004d42f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `fn_004bef60(entry, buffer)` (the step after loading from a buffer)
/// on every non-null entry of the changes, with the buffer's mode words
/// switched (see `with_buffer_modes`).
pub fn fn_004d42f0(e: &mut Engine, this: Ptr<InventoryChanges>, buffer: u32) {
    with_buffer_modes(e, buffer, true, |e| {
        let mut cursor = e.mem.u32(this.addr());
        while cursor != 0 {
            let entry = list_item(e, cursor);
            if entry != 0 {
                fn_004bef60(e, Ptr::new(entry), buffer);
            }
            cursor = list_next(e, cursor);
        }
    });
}

// Translated from 004d43c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The position of the extra data list `extra` among the extra lists of the
/// changes' entry for `form`; -1 when there is no such entry (or no `form`
/// or `extra`) or the entry does not hold the list.
pub fn fn_004d43c0(e: &mut Engine, this: Ptr<InventoryChanges>, form: u32, extra: u32) -> i32 {
    if form == 0 || extra == 0 {
        return -1;
    }
    let mut cursor = e.mem.u32(this.addr());
    while cursor != 0 {
        let entry = list_item(e, cursor);
        cursor = list_next(e, cursor);
        if entry != 0 && e.call(WORD_AT_8, &args![entry]).u32() == form {
            let mut index = 0i32;
            let mut lists = e.call(WORD_AT_0, &args![entry]).u32();
            while lists != 0 {
                let item = list_item(e, lists);
                if item != 0 && item == extra {
                    return index;
                }
                index += 1;
                lists = list_next(e, lists);
            }
            return -1;
        }
    }
    -1
}

// Translated from 004d4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The extra data list at position `index` among the extra lists of the
/// changes' entry for `form`; 0 when there is none (or no `form`, or `index`
/// is -1).
pub fn fn_004d4480(e: &mut Engine, this: Ptr<InventoryChanges>, form: u32, index: i32) -> u32 {
    if form == 0 || index == -1 {
        return 0;
    }
    let mut cursor = e.mem.u32(this.addr());
    while cursor != 0 {
        let entry = list_item(e, cursor);
        cursor = list_next(e, cursor);
        if entry != 0 && e.call(WORD_AT_8, &args![entry]).u32() == form {
            let mut position = 0i32;
            let mut lists = e.call(WORD_AT_0, &args![entry]).u32();
            while lists != 0 {
                let item = list_item(e, lists);
                if position == index {
                    return item;
                }
                position += 1;
                lists = list_next(e, lists);
            }
            return 0;
        }
    }
    0
}

// Translated from 004d4530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `callback(&item, user_data)` (`__cdecl`) for each item stack of the
/// owner's inventory until it returns true; returns how many were offered.
/// The container's objects the changes have no entry for come first (as an
/// `ItemChange` on the stack with the object's count and form), then the
/// entries of the changes (number plus what the container holds, sharing the
/// entry's extra lists; entries with that total 0 are left out). With bit 0
/// of `flags` the filter `skipped_by_item_filter` applies. The C++ exception
/// frame is not translated.
pub fn fn_004d4530(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    callback: u32,
    user_data: u32,
    flags: u32,
) -> i32 {
    let mut offered = 0i32;
    let container = fn_004bffb0(e, this);
    let mut node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
    while node != 0 {
        let object = list_item(e, node);
        node = list_next(e, node);
        if object == 0 {
            continue;
        }
        let object_form = e.mem.u32(object + 4);
        if inventory_changes_get_object_in_list(e, this, object_form, 1, 0) != 0 {
            continue;
        }
        if flags & 1 != 0 && skipped_by_item_filter(e, object_form) {
            continue;
        }
        let stop = e.with_stack(0xc, |e, item| {
            item_change_item_change_ov3(e, Ptr::new(item.addr()));
            let count = e.mem.u32(object);
            e.call(ITEM_SET_NUMBER, &args![item, count]);
            e.call(ITEM_SET_FORM, &args![item, object_form]);
            offered += 1;
            let stop = e.call(callback, &args![item, user_data]).u8() != 0;
            fn_004bc5f0(e, Ptr::new(item.addr()));
            stop
        });
        if stop {
            return offered;
        }
    }
    let mut node = e.mem.u32(this.addr());
    while node != 0 {
        let entry = list_item(e, node);
        node = list_next(e, node);
        if entry == 0 {
            continue;
        }
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        if flags & 1 != 0 && skipped_by_item_filter(e, form) {
            continue;
        }
        let container = fn_004bffb0(e, this);
        let held = e.call(CONTAINER_COUNT, &args![container, form]).i32();
        let number = e.call(WORD_AT_4, &args![entry]).i32().wrapping_add(held);
        if number == 0 {
            continue;
        }
        let stop = e.with_stack(0xc, |e, item| {
            item_change_item_change_ov3(e, Ptr::new(item.addr()));
            let lists = e.call(WORD_AT_0, &args![entry]).u32();
            e.mem.set_u32(item.addr(), lists);
            e.call(ITEM_SET_NUMBER, &args![item, number]);
            e.call(ITEM_SET_FORM, &args![item, form]);
            offered += 1;
            let stop = e.call(callback, &args![item, user_data]).u8() != 0;
            // The item shares the entry's lists: it must not free them.
            e.mem.set_u32(item.addr(), 0);
            fn_004bc5f0(e, Ptr::new(item.addr()));
            stop
        });
        if stop {
            return offered;
        }
    }
    offered
}

// Translated from 004d4830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The next item stack of the owner's inventory for the resumable iteration
/// state `iterator` (an `ItemChange` followed by the container node at +0xc,
/// the changes node at +0x10 and the finished byte at +0x14), written into
/// the iterator itself and stored in `*out`; false when the iteration is
/// over. Like `fn_004d4530` it first gives the container objects without a
/// changes entry (the iterator's lists cleared, the object's count and form),
/// then the entries of the changes (sharing the entry's lists, with the
/// number plus what the container holds when that is above 0). Bit 0 of
/// `flags` applies `skipped_by_item_filter`.
pub fn fn_004d4830(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    iterator: u32,
    out: u32,
    flags: u32,
) -> bool {
    if e.mem.u8(iterator + 0x14) != 0 {
        return false;
    }
    if e.mem.u32(iterator + 0xc) == 0 && e.mem.u32(iterator + 0x10) == 0 {
        let container = fn_004bffb0(e, this);
        let node = e.call(CONTAINER_OBJECT_LIST, &args![container]).u32();
        e.mem.set_u32(iterator + 0xc, node);
    }
    if e.mem.u32(iterator + 0xc) != 0 {
        loop {
            let node = e.mem.u32(iterator + 0xc);
            if node == 0 {
                break;
            }
            let object = list_item(e, node);
            let next = list_next(e, node);
            e.mem.set_u32(iterator + 0xc, next);
            if next == 0 {
                let changes = e.mem.u32(this.addr());
                if changes != 0 {
                    e.mem.set_u32(iterator + 0x10, changes);
                } else {
                    e.mem.set_u8(iterator + 0x14, 1);
                }
            }
            if object == 0 {
                continue;
            }
            let object_form = e.mem.u32(object + 4);
            if inventory_changes_get_object_in_list(e, this, object_form, 1, 0) != 0 {
                continue;
            }
            if flags & 1 != 0 && skipped_by_item_filter(e, object_form) {
                continue;
            }
            e.mem.set_u32(iterator, 0);
            let count = e.mem.u32(object);
            e.call(ITEM_SET_NUMBER, &args![iterator, count]);
            e.call(ITEM_SET_FORM, &args![iterator, object_form]);
            e.mem.set_u32(out, iterator);
            return true;
        }
    }
    if e.mem.u32(iterator + 0x10) == 0 {
        return false;
    }
    loop {
        let node = e.mem.u32(iterator + 0x10);
        if node == 0 {
            return false;
        }
        let entry = list_item(e, node);
        let next = list_next(e, node);
        e.mem.set_u32(iterator + 0x10, next);
        if next == 0 {
            e.mem.set_u8(iterator + 0x14, 1);
        }
        if entry == 0 {
            continue;
        }
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        if flags & 1 != 0 && skipped_by_item_filter(e, form) {
            continue;
        }
        let held_by = e.call(WORD_AT_4, &args![entry]).i32();
        let container = fn_004bffb0(e, this);
        let held = e.call(CONTAINER_COUNT, &args![container, form]).i32();
        let number = held_by.wrapping_add(held);
        if number <= 0 {
            continue;
        }
        e.mem.set_u32(out, entry);
        let lists = e.call(WORD_AT_0, &args![entry]).u32();
        e.mem.set_u32(iterator, lists);
        e.call(ITEM_SET_NUMBER, &args![iterator, number]);
        e.call(ITEM_SET_FORM, &args![iterator, form]);
        e.mem.set_u32(out, iterator);
        return true;
    }
}

/// Runs the fast iteration of the owner's inventory: `visit(item)` is called
/// for each item stack the iteration gives (a new `ItemChange` the visitor
/// owns), then the iterator is deleted (virtual-free through its scalar
/// deleting destructor `004bf630`).
fn for_each_fast_item(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    mut visit: impl FnMut(&mut Engine, u32) -> bool,
) {
    let iterator = inventory_changes_start_oei_fast_inventory_iteration(e, this);
    loop {
        let item = inventory_changes_get_next_oei_fast_inventory_item(e, this, iterator);
        if item != 0 && !visit(e, item) {
            break;
        }
        if item == 0 {
            break;
        }
    }
    if iterator != 0 {
        oei_fast_inventory_iterator_scalar_deleting_destructor(e, iterator, 1);
    }
}

// Translated from 004d4b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetItemGroup` (Xbox PDB): adds to `list` (a
/// `BSSimpleList<ItemChange *>`) the item stacks of the fast iteration whose
/// form is `form`; the other stacks are deleted, and once a stack has been
/// found the first other one ends the walk. Nothing is done without a form or
/// a list.
pub fn inventory_changes_get_item_group(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    list: u32,
) {
    if form == 0 || list == 0 {
        return;
    }
    let mut found = false;
    for_each_fast_item(e, this, |e, item| {
        if e.call(WORD_AT_8, &args![item]).u32() == form {
            found = true;
            list_call_with_item(e, LIST_ADD, list, item);
            true
        } else {
            delete_item_change(e, item);
            !found
        }
    });
}

// Translated from 004d4bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetRepairItemGroup` (Xbox PDB): adds to `list` the item
/// stacks of the fast iteration whose form `0047bb50(itemForm, form)` accepts;
/// the other stacks are deleted. Nothing is done without a form or a list.
pub fn inventory_changes_get_repair_item_group(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    form: u32,
    list: u32,
) {
    if form == 0 || list == 0 {
        return;
    }
    for_each_fast_item(e, this, |e, item| {
        let item_form = e.call(WORD_AT_8, &args![item]).u32();
        if e.call(REPAIR_ITEM_TEST, &args![item_form, form]).u8() != 0 {
            list_call_with_item(e, LIST_ADD, list, item);
        } else {
            delete_item_change(e, item);
        }
        true
    });
}

// Translated from 004d4ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryChanges::GetItemModGroup` (Xbox PDB): for the weapon `weapon`
/// (a form of type 0x28), adds to `list` the item stacks of the fast
/// iteration whose form is one of the weapon's three mod objects; stacks that
/// are mod objects (form type 0x67) but none of its three are neither added
/// nor deleted, other stacks are deleted. Nothing is done without a weapon or
/// a list.
pub fn inventory_changes_get_item_mod_group(
    e: &mut Engine,
    this: Ptr<InventoryChanges>,
    weapon: u32,
    list: u32,
) {
    if weapon == 0 || list == 0 || form_type_of(e, weapon) != FORM_TYPE_WEAPON {
        return;
    }
    for_each_fast_item(e, this, |e, item| {
        let item_form = e.call(WORD_AT_8, &args![item]).u32();
        if form_type_of(e, item_form) == FORM_TYPE_WEAPON_MOD {
            let mod_form = e.call(WORD_AT_8, &args![item]).u32();
            for slot_bit in [1u32, 2, 4] {
                if mod_form == fn_004bd570(e, Ptr::new(weapon), slot_bit) {
                    list_call_with_item(e, LIST_ADD, list, item);
                    break;
                }
            }
        } else {
            delete_item_change(e, item);
        }
        true
    });
}

// Translated from 004d4e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor body of `NiTMapBase<TESObject *, bool>` (the map's
/// `NiTMapBase` layout, [`NiTMap`]): vtable, `size` buckets (a zeroed bucket
/// array of `size` words from `NiMalloc`) and no items.
pub fn fn_004d4e40(e: &mut Engine, this: Ptr<NiTMap>, size: u32) -> Ptr<NiTMap> {
    e.mem.set_u32(this.addr(), NI_MAP_BASE_VTABLE);
    e.set(this, NiTMap::m_uiHashSize, size);
    e.set(this, NiTMap::m_uiCount, 0);
    let bytes = size.wrapping_shl(2);
    let table = e.call(NI_ALLOCATE, &args![bytes]).u32();
    e.set(this, NiTMap::m_ppkHashTable, table);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    let bytes = e.get(this, NiTMap::m_uiHashSize).wrapping_shl(2);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 004d4de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObject *, bool>::NiTMap` (the map has no name): the base
/// constructor, then the derived vtable.
pub fn fn_004d4de0(e: &mut Engine, this: Ptr<NiTMap>, size: u32) -> Ptr<NiTMap> {
    fn_004d4e40(e, this, size);
    e.mem.set_u32(this.addr(), NI_MAP_VTABLE);
    this
}

// Translated from 004d4f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `NiTMapBase<TESObject *, bool>`: the base vtable,
/// the map cleared (`00438af0`) and its bucket array freed (`NiFree`).
pub fn fn_004d4f10(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), NI_MAP_BASE_VTABLE);
    e.call(NI_MAP_CLEAR, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(NI_FREE, &args![table]);
}

// Translated from 004d4eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTMap<TESObject *, bool>` (the map names the body of
/// a std `ctype` destructor through folding): the derived vtable, the map
/// cleared (`00438af0`) and then the base destructor body. The C++ exception
/// frame is not translated.
pub fn fn_004d4eb0(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), NI_MAP_VTABLE);
    e.call(NI_MAP_CLEAR, &args![this]);
    fn_004d4f10(e, this);
}

// Translated from 004d4e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObject *, bool>::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, and the memory freed when bit 0 of `flags` is set; returns
/// `this`.
pub fn ni_tmap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004d4eb0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004d4f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<TESObject *, bool>>, TESObject *,
/// bool>::_scalar_deleting_destructor_` (Xbox PDB): the base destructor body,
/// and the memory freed when bit 0 of `flags` is set; returns `this`.
pub fn ni_tmap_base_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004d4f10(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004bc550,
            item_change_item_change(Ptr<ItemChange>, Ptr, i32) -> Ptr<ItemChange>
        ),
        entry!(0x004bc5f0, fn_004bc5f0(Ptr<ItemChange>)),
        entry!(
            0x004bc650,
            item_change_item_change_ov2(Ptr<ItemChange>, Ptr<ItemChange>) -> Ptr<ItemChange>
        ),
        entry!(0x004bc780, item_change_delete_all_extra(Ptr<ItemChange>)),
        entry!(0x004bc810, fn_004bc810(Ptr<ItemChange>, Ptr<ItemChange>)),
        entry!(
            0x004bc8f0,
            item_change_get_extra_total_count(Ptr<ItemChange>, bool) -> i32
        ),
        entry!(
            0x004bc980,
            item_change_get_extra_total_default_count(Ptr<ItemChange>) -> i32
        ),
        entry!(0x004bca60, fn_004bca60(Ptr) -> bool),
        entry!(
            0x004bca80,
            item_change_get_amount_non_default_extra(Ptr<ItemChange>) -> i32
        ),
        entry!(0x004bcb70, fn_004bcb70(Ptr<ItemChange>) -> bool),
        entry!(
            0x004bcbd0,
            item_change_remove_original_changes(Ptr<ItemChange>)
        ),
        entry!(
            0x004bcc30,
            item_change_number_leveled_extra(Ptr<ItemChange>) -> i32
        ),
        entry!(0x004bccb0, fn_004bccb0(Ptr<ItemChange>) -> bool),
        entry!(
            0x004bcd10,
            item_change_remove_leveled_item_extra(Ptr<ItemChange>) -> u32
        ),
        entry!(
            0x004bcdb0,
            item_change_get_item_health(Ptr<ItemChange>, bool) -> f32
        ),
        entry!(0x004bcf00, tes_object_weap_get_form_health(Ptr, u8) -> i32),
        entry!(0x004bcf60, tes_object_weap_get_mod_effect_value(Ptr, u32, u8) -> f32),
        entry!(
            0x004bd030,
            item_change_set_item_health(Ptr<ItemChange>, f32, Ptr, Ptr, bool)
        ),
        entry!(0x004bd350, fn_004bd350(Ptr<ItemChange>) -> f32),
        entry!(
            0x004bd400,
            item_change_get_item_value(Ptr<ItemChange>) -> f32
        ),
        entry!(0x004bd510, fn_004bd510(f32, f32) -> f32),
        entry!(0x004bd570, fn_004bd570(Ptr, u32) -> u32),
        entry!(0x004bd5c0, fn_004bd5c0(Ptr<ItemChange>, Ptr)),
        entry!(
            0x004bd740,
            item_change_get_item_ownership(Ptr<ItemChange>) -> u32
        ),
        entry!(0x004bd7a0, item_change_get_hot_key(Ptr<ItemChange>) -> i32),
        entry!(0x004bd820, item_change_get_mod_slots(Ptr<ItemChange>) -> u8),
        entry!(0x004bd880, fn_004bd880(Ptr, u32) -> u32),
        entry!(
            0x004bd8d0,
            item_change_has_mod_effect_active(Ptr<ItemChange>, u8, Ptr) -> bool
        ),
        entry!(0x004bd9d0, fn_004bd9d0(Ptr, u32, u8) -> f32),
        entry!(
            0x004bda70,
            item_change_has_mod_effect_active_ov2(Ptr<ItemChange>, u8) -> bool
        ),
        entry!(0x004bdb40, item_change_set_is_modding(Ptr<ItemChange>)),
        entry!(0x004bdc70, item_change_remove_is_modding(Ptr<ItemChange>)),
        entry!(0x004bdcc0, item_change_get_poison(Ptr<ItemChange>) -> u32),
        entry!(0x004bdd20, fn_004bdd20(Ptr<ItemChange>, u32)),
        entry!(0x004bdd80, fn_004bdd80(Ptr<ItemChange>)),
        entry!(
            0x004bddd0,
            item_change_get_worn(Ptr<ItemChange>, u8) -> bool
        ),
        entry!(0x004bde40, item_change_get_script(Ptr<ItemChange>) -> u32),
        entry!(
            0x004bdea0,
            item_change_get_script_locals(Ptr<ItemChange>) -> u32
        ),
        entry!(
            0x004bdf00,
            item_change_get_modified_attack_damage(Ptr<ItemChange>, Ptr, f32, u8) -> f32
        ),
        entry!(0x004bdf90, fn_004bdf90(Ptr<ItemChange>, u32) -> f32),
        entry!(0x004be060, fn_004be060(Ptr) -> f32),
        entry!(0x004be080, fn_004be080(Ptr) -> f32),
        entry!(0x004be0b0, fn_004be0b0(Ptr<ItemChange>, u32) -> f32),
        entry!(0x004be180, fn_004be180(Ptr) -> f32),
        entry!(0x004be1a0, fn_004be1a0(Ptr<ItemChange>) -> u32),
        entry!(
            0x004be200,
            item_change_get_icon_one(Ptr<ItemChange>, u32) -> u32
        ),
        entry!(
            0x004be2d0,
            item_change_get_full_name(Ptr<ItemChange>) -> u32
        ),
        entry!(
            0x004be330,
            tes_enchantable_form_get_form_enchanting(u32) -> u32
        ),
        entry!(0x004be380, tes_object_weap_get_form_weight(Ptr, u8) -> f32),
        entry!(
            0x004be3d0,
            item_change_set_worn(Ptr<ItemChange>, u8, u8, u8)
        ),
        entry!(0x004be580, fn_004be580(Ptr) -> u8),
        entry!(0x004be5a0, fn_004be5a0(Ptr<ItemChange>) -> u16),
        entry!(0x004be6f0, fn_004be6f0(Ptr<ItemChange>)),
        entry!(0x004be930, item_change_load_game(Ptr<ItemChange>)),
        entry!(0x004bed00, fn_004bed00(Ptr<ItemChange>)),
        entry!(0x004bed60, item_change_save_game(Ptr<ItemChange>, u32)),
        entry!(0x004bee00, item_change_load_game_ov2(Ptr<ItemChange>, u32)),
        entry!(0x004bef60, fn_004bef60(Ptr<ItemChange>, u32)),
        entry!(
            0x004befb0,
            inventory_changes_inventory_changes(
                Ptr<InventoryChanges>,
                Ptr,
            ) -> Ptr<InventoryChanges>
        ),
        entry!(0x004bf0e0, fn_004bf0e0(Ptr<InventoryChanges>)),
        entry!(0x004bf150, fn_004bf150(Ptr<InventoryChanges>)),
        entry!(
            0x004bf220,
            inventory_changes_get_inventory_changes(u32) -> u32
        ),
        entry!(
            0x004bf330,
            inventory_changes_remove_hot_key_item(Ptr<InventoryChanges>, Ptr<ItemChange>, i32)
        ),
        entry!(0x004bf4b0, fn_004bf4b0(Ptr<InventoryChanges>, i32) -> u32),
        entry!(
            0x004bf630,
            oei_fast_inventory_iterator_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x004bf660,
            fn_004bf660(Ptr<InventoryChanges>, i32, i32) -> u32
        ),
        entry!(
            0x004bf800,
            inventory_changes_set_hot_key_item(Ptr<InventoryChanges>, Ptr<ItemChange>, u32, i32)
        ),
        entry!(0x004bfb30, fn_004bfb30(u32, i32) -> u32),
        entry!(0x004bfb70, fn_004bfb70(u32, i32, u32)),
        entry!(
            0x004bfba0,
            inventory_changes_get_object_in_list(Ptr<InventoryChanges>, u32, u32, u32) -> u32
        ),
        entry!(0x004bfc80, fn_004bfc80(Ptr<InventoryChanges>) -> bool),
        entry!(0x004bfd80, fn_004bfd80(Ptr) -> bool),
        entry!(
            0x004bfda0,
            inventory_changes_wearing_object(Ptr<InventoryChanges>, u32, u8) -> u32
        ),
        entry!(
            0x004bfe50,
            inventory_changes_remove_all_objects_worn(Ptr<InventoryChanges>, u32, u32)
        ),
        entry!(0x004bffb0, fn_004bffb0(Ptr<InventoryChanges>) -> u32),
        entry!(
            0x004bffe0,
            fn_004bffe0(Ptr<InventoryChanges>, u32, i32, u32, u32, u8, u8)
        ),
        entry!(0x004c0bd0, fn_004c0bd0(Ptr) -> bool),
        entry!(0x004c0bf0, fn_004c0bf0(Ptr) -> bool),
        entry!(0x004c0c30, fn_004c0c30(Ptr) -> bool),
        entry!(0x004c0c60, fn_004c0c60(Ptr) -> bool),
        entry!(0x004c0c90, fn_004c0c90(Ptr, f32)),
        entry!(0x004c0cd0, fn_004c0cd0(Ptr, u32)),
        entry!(
            0x004c0cf0,
            fn_004c0cf0(Ptr<InventoryChanges>, u32, u32, i32, u32, u32, u8, u32, u32, u32) -> bool
        ),
        entry!(
            0x004c1520,
            fn_004c1520(Ptr<InventoryChanges>, u32, i32, u32, u32, u32)
        ),
        entry!(0x004c1c40, fn_004c1c40()),
        entry!(
            0x004c1c90,
            fn_004c1c90(Ptr<InventoryChanges>, u32, i32, u32, u8)
        ),
        entry!(
            0x004c29a0,
            fn_004c29a0(Ptr<InventoryChanges>, u32, u32, i32)
        ),
        entry!(0x004c3380, fn_004c3380(Ptr<InventoryChanges>, u32, u8)),
        entry!(
            0x004c37d0,
            fn_004c37d0(
                Ptr<InventoryChanges>,
                u32,
                u32,
                u8,
                i32,
                u32,
                u8,
                u32,
                u32,
                u32,
                u8,
                u8,
                u32,
            ) -> u32
        ),
        entry!(0x004c69f0, fn_004c69f0(Ptr) -> u32),
        entry!(
            0x004c6a10,
            inventory_changes_get_objectby_pack_obj_type(Ptr<InventoryChanges>, u32, u32) -> u32
        ),
        entry!(
            0x004c6ba0,
            fn_004c6ba0(Ptr<InventoryChanges>, u32, u32) -> u32
        ),
        entry!(0x004c6d40, fn_004c6d40(Ptr<InventoryChanges>, u32, u8, i32)),
        entry!(
            0x004c6dd0,
            inventory_changes_drop_item_into_world(
                Ptr<InventoryChanges>,
                u32,
                u32,
                i32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x004c6f60,
            fn_004c6f60(Ptr<InventoryChanges>, u32, u32) -> u32
        ),
        entry!(
            0x004c7300,
            fn_004c7300(Ptr<InventoryChanges>, u32, u32) -> u32
        ),
        entry!(
            0x004c7400,
            fn_004c7400(Ptr<InventoryChanges>, u32, u32, i32, u8) -> u32
        ),
        entry!(
            0x004c8c10,
            fn_004c8c10(Ptr<InventoryChanges>, i32, u8) -> u32
        ),
        entry!(0x004c8f30, fn_004c8f30(Ptr<InventoryChanges>, u32) -> i32),
        entry!(
            0x004c8220,
            fn_004c8220(Ptr<InventoryChanges>, u32, i32, u8) -> u32
        ),
        entry!(0x004c8fd0, fn_004c8fd0(Ptr<InventoryChanges>, u8) -> i32),
        entry!(0x004c94d0, fn_004c94d0(Ptr) -> bool),
        entry!(0x004c94f0, fn_004c94f0(Ptr<InventoryChanges>, i32) -> u32),
        entry!(0x004ca1a0, fn_004ca1a0(u32)),
        entry!(
            0x004ca200,
            inventory_changes_start_oei_fast_inventory_iteration(Ptr<InventoryChanges>) -> u32
        ),
        entry!(
            0x004ca330,
            inventory_changes_get_next_oei_fast_inventory_item(Ptr<InventoryChanges>, u32) -> u32
        ),
        entry!(
            0x004cafe0,
            inventory_changes_get_best_food(Ptr<InventoryChanges>) -> u32
        ),
        entry!(
            0x004cb320,
            inventory_changes_get_gold_amount(Ptr<InventoryChanges>) -> i32
        ),
        entry!(
            0x004cb4b0,
            inventory_changes_remove_gold(Ptr<InventoryChanges>, u32, i32, u32)
        ),
        entry!(
            0x004cb5f0,
            inventory_changes_remove_stolen_items(Ptr<InventoryChanges>, u32, u32, u32)
        ),
        entry!(0x004cba70, fn_004cba70(Ptr<InventoryChanges>, u32) -> u32),
        entry!(0x004cbbc0, fn_004cbbc0(Ptr<InventoryChanges>)),
        entry!(
            0x004cd9c0,
            inventory_changes_duplicate_all_items(Ptr<InventoryChanges>, u32, u32)
        ),
        entry!(0x004ce300, tes_scriptable_form_set_form_script(u32, u32)),
        entry!(0x004cfe20, fn_004cfe20(Ptr<InventoryChanges>, u32) -> bool),
        entry!(
            0x004cffe0,
            inventory_changes_container_has_objects(
                Ptr<InventoryChanges>,
                u32,
                u32,
                i32,
                u32,
                u32,
                u32,
            ) -> bool
        ),
        entry!(0x004d0360, fn_004d0360(Ptr<InventoryChanges>) -> bool),
        entry!(0x004d0490, fn_004d0490(Ptr<InventoryChanges>) -> bool),
        entry!(
            0x004ce340,
            fn_004ce340(Ptr<InventoryChanges>, u32, u32, u8, u8, u8, u8, i32, i32) -> f32
        ),
        entry!(
            0x004ce380,
            fn_004ce380(Ptr<InventoryChanges>, u32, u32, u8, u8, u8, u8, i32, u32) -> f32
        ),
        entry!(
            0x004d0650,
            inventory_changes_get_inventory_item(Ptr<InventoryChanges>, u32, u32) -> u32
        ),
        entry!(0x004d0900, fn_004d0900(Ptr<InventoryChanges>, u8) -> f32),
        entry!(
            0x004d0f40,
            fn_004d0f40(Ptr<InventoryChanges>, u8, u8) -> i32
        ),
        entry!(0x004d1180, fn_004d1180(Ptr<InventoryChanges>, u32) -> f32),
        entry!(0x004d1360, fn_004d1360(Ptr) -> bool),
        entry!(0x004d1380, fn_004d1380(Ptr<InventoryChanges>, i32) -> bool),
        entry!(0x004d1440, fn_004d1440(Ptr<InventoryChanges>)),
        entry!(0x004d1610, fn_004d1610(Ptr<InventoryChanges>)),
        entry!(0x004d17a0, fn_004d17a0(Ptr<InventoryChanges>)),
        entry!(0x004d1960, fn_004d1960(Ptr<InventoryChanges>)),
        entry!(
            0x004d2480,
            inventory_changes_run_scripts(Ptr<InventoryChanges>, u32) -> bool
        ),
        entry!(
            0x004d26d0,
            fn_004d26d0(Ptr<InventoryChanges>, Ptr<InventoryChanges>, u32, u8)
        ),
        entry!(
            0x004d3660,
            inventory_changes_clear_all_change_items(Ptr<InventoryChanges>)
        ),
        entry!(0x004d3960, fn_004d3960(Ptr<InventoryChanges>) -> u16),
        entry!(0x004d3ab0, fn_004d3ab0(Ptr<InventoryChanges>)),
        entry!(
            0x004d3cc0,
            inventory_changes_load_game(Ptr<InventoryChanges>)
        ),
        entry!(0x004d4030, fn_004d4030(Ptr<InventoryChanges>)),
        entry!(0x004d4090, fn_004d4090(Ptr<InventoryChanges>, u32)),
        entry!(0x004d4160, fn_004d4160(Ptr<InventoryChanges>, u32)),
        entry!(0x004d42f0, fn_004d42f0(Ptr<InventoryChanges>, u32)),
        entry!(
            0x004d43c0,
            fn_004d43c0(Ptr<InventoryChanges>, u32, u32) -> i32
        ),
        entry!(
            0x004d4480,
            fn_004d4480(Ptr<InventoryChanges>, u32, i32) -> u32
        ),
        entry!(
            0x004d4530,
            fn_004d4530(Ptr<InventoryChanges>, u32, u32, u32) -> i32
        ),
        entry!(
            0x004d4830,
            fn_004d4830(Ptr<InventoryChanges>, u32, u32, u32) -> bool
        ),
        entry!(
            0x004d4b00,
            inventory_changes_get_item_group(Ptr<InventoryChanges>, u32, u32)
        ),
        entry!(
            0x004d4bd0,
            inventory_changes_get_repair_item_group(Ptr<InventoryChanges>, u32, u32)
        ),
        entry!(
            0x004d4ca0,
            inventory_changes_get_item_mod_group(Ptr<InventoryChanges>, u32, u32)
        ),
        entry!(0x004d4de0, fn_004d4de0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004d4e10,
            ni_tmap_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x004d4e40, fn_004d4e40(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004d4eb0, fn_004d4eb0(Ptr<NiTMap>)),
        entry!(0x004d4f10, fn_004d4f10(Ptr<NiTMap>)),
        entry!(
            0x004d4f40,
            ni_tmap_base_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(
            0x0076b630,
            item_change_item_change_ov3(Ptr<ItemChange>) -> Ptr<ItemChange>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vtable of every fake object: slot 0 the scalar deleting destructor,
    /// slot 4 (+0x10) the health form's base health, slot 18 (+0x48) the
    /// owner's change notification.
    const VTABLE: u32 = 0x0200_0000;
    const DESTRUCTOR: u32 = 0x0200_1000;
    const BASE_HEALTH: u32 = 0x0200_1010;
    const OWNER_NOTIFY: u32 = 0x0200_1048;
    /// Cast results: weapon, enchantable form, health form (words).
    const CAST_TABLE: u32 = 0x0200_3000;

    // Fake extra data list (0x80 bytes): the doubles read these words.
    const X_ITEMS_IN_LIST: u32 = 0x20;
    const X_COUNT: u32 = 0x24;
    const X_DEFAULT: u32 = 0x28;
    const X_WORN: u32 = 0x2c;
    const X_HAS_LEVELED: u32 = 0x30;
    const X_LEVELED_ITEM: u32 = 0x34;
    const X_HOT_KEY: u32 = 0x38;
    const X_ORIGINAL: u32 = 0x3c;
    const X_HEALTH: u32 = 0x40;
    const X_POISON: u32 = 0x44;
    const X_SCRIPT: u32 = 0x48;
    const X_LOCALS: u32 = 0x4c;
    const X_HAS_MODS: u32 = 0x50;
    const X_MOD_FLAGS: u32 = 0x54;
    const X_SLOT_ACTIVE: u32 = 0x58;
    const X_OWNERSHIP: u32 = 0x5c;
    const X_FLOAT: u32 = 0x60;
    const X_D030: u32 = 0x64;

    // Fake form (0x400 bytes, also the weapon).
    const F_TYPE: u32 = 0x04;
    const F_VALUE: u32 = 0x10;
    const F_HEALTH_FORM: u32 = 0x14;
    const F_HEALTH: u32 = 0x18;
    const F_ENCHANT_VALUE: u32 = 0x1c;
    const F_RATING: u32 = 0x20;

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn returns_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    fn word_arg(a: &[u32], i: usize) -> f64 {
        f64::from_bits(a[i] as u64 | (a[i + 1] as u64) << 32)
    }

    macro_rules! read_word {
        ($e:expr, $addr:expr, $off:expr) => {
            $e.register($addr, |e, a| returns(e.mem.u32(a[0] + $off)))
        };
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10000);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(HUNDRED_FLOAT, 100.0f32);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(PRICE_STEP, 0.1f32);
        e.map(0x0200_0000, 0x4000);
        let mut slots = vec![0u32; 20];
        slots[0] = DESTRUCTOR;
        slots[4] = BASE_HEALTH;
        slots[18] = OWNER_NOTIFY;
        e.put_vtable(VTABLE, &slots);
        stub(&mut e, DESTRUCTOR);
        stub(&mut e, OWNER_NOTIFY);
        e.register(BASE_HEALTH, |e, a| returns(e.mem.u32(a[0] + F_HEALTH)));
        // List nodes and small accessors.
        e.register(0x0068_15c0, |_, a| returns(a[0]));
        e.register(WORD_AT_4, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_8, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_0, |e, a| returns(e.mem.u32(a[0])));
        e.register(FORM_TYPE, |e, a| returns(e.mem.u8(a[0] + F_TYPE) as u32));
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        e.register(LIST_NODE_CONSTRUCT, |_, a| returns(a[0]));
        e.register(EXTRA_DATA_LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            returns(a[0])
        });
        e.register(EXTRA_DATA_LIST_DUPLICATE, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u32(a[0] + 0x14, a[1]);
            Ret::default()
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                let next = e.mem.u32(a[0] + 4);
                e.mem.set_u32(node + 4, next);
                e.mem.set_u32(a[0] + 4, node);
            }
            Ret::default()
        });
        e.register(LIST_ADD_TAIL, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut last = a[0];
            while e.mem.u32(last + 4) != 0 {
                last = e.mem.u32(last + 4);
            }
            if e.mem.u32(last) == 0 {
                e.mem.set_u32(last, item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(last + 4, node);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut previous = a[0];
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != item {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            if node == a[0] {
                let next = e.mem.u32(node + 4);
                if next != 0 {
                    let next_item = e.mem.u32(next);
                    let next_next = e.mem.u32(next + 4);
                    e.mem.set_u32(node, next_item);
                    e.mem.set_u32(node + 4, next_next);
                } else {
                    e.mem.set_u32(node, 0);
                }
            } else if node != 0 {
                let next = e.mem.u32(node + 4);
                e.mem.set_u32(previous + 4, next);
            }
            Ret::default()
        });
        e.register(FTOL, |_, a| Ret {
            eax: (word_arg(a, 0).trunc() as i32) as u32,
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |e, a| {
            returns(match a[3] {
                TYPE_TES_OBJECT_WEAP => e.mem.u32(CAST_TABLE),
                TYPE_TES_ENCHANTABLE_FORM => e.mem.u32(CAST_TABLE + 4),
                TYPE_TES_HEALTH_FORM => e.mem.u32(CAST_TABLE + 8),
                other => panic!("unexpected cast target {other:08x}"),
            })
        });
        // Extra data list accessors.
        read_word!(e, EXTRA_IS_DEFAULT_FOR_CONTAINER, X_DEFAULT);
        read_word!(e, EXTRA_GET_WORN, X_WORN);
        read_word!(e, EXTRA_GET_COUNT, X_COUNT);
        read_word!(e, EXTRA_HAS_LEVELED_ITEM, X_HAS_LEVELED);
        read_word!(e, EXTRA_GET_LEVELED_ITEM, X_LEVELED_ITEM);
        read_word!(e, EXTRA_GET_ORIGINAL_REFERENCE, X_ORIGINAL);
        read_word!(e, EXTRA_ITEMS_IN_LIST, X_ITEMS_IN_LIST);
        read_word!(e, EXTRA_GET_HOT_KEY, X_HOT_KEY);
        read_word!(e, EXTRA_GET_POISON, X_POISON);
        read_word!(e, EXTRA_GET_SCRIPT, X_SCRIPT);
        read_word!(e, EXTRA_GET_SCRIPT_LOCALS, X_LOCALS);
        read_word!(e, EXTRA_HAS_WEAPON_MODS, X_HAS_MODS);
        read_word!(e, EXTRA_GET_WEAPON_MOD_FLAGS, X_MOD_FLAGS);
        read_word!(e, EXTRA_GET_OWNERSHIP, X_OWNERSHIP);
        read_word!(e, EXTRA_TEST_D030, X_D030);
        e.register(EXTRA_GET_WEAPON_MOD_SLOT_ACTIVE, |e, a| {
            returns((e.mem.u32(a[0] + X_SLOT_ACTIVE) & a[1] != 0) as u32)
        });
        e.register(EXTRA_GET_HEALTH, |e, a| {
            returns_float(e.mem.f32(a[0] + X_HEALTH))
        });
        e.register(EXTRA_GET_FLOAT_EXTRA, |e, a| {
            returns_float(e.mem.f32(a[0] + X_FLOAT))
        });
        e.register(EXTRA_SET_HEALTH, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        e.register(EXTRA_SET_HEALTH_NEW, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        e.register(EXTRA_SET_IS_MODDING, |e, a| {
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            Ret::default()
        });
        for address in [
            EXTRA_REMOVE_ALL,
            EXTRA_REMOVE_COUNT,
            EXTRA_REMOVE_ORIGINAL_REFERENCE,
            EXTRA_REMOVE_EXTRA,
            EXTRA_REMOVE_HEALTH,
            EXTRA_REMOVE_IS_MODDING,
            EXTRA_REMOVE_POISON,
            EXTRA_SET_RANK,
            EXTRA_COPY_FROM,
            LIST_CLEAR,
            LIST_DESTROY,
        ] {
            stub(&mut e, address);
        }
        // Forms and formulas.
        read_word!(e, GET_FORM_AS_HEALTH_FORM, F_HEALTH_FORM);
        read_word!(e, GET_FORM_HEALTH, F_HEALTH);
        read_word!(e, GET_FORM_VALUE, F_VALUE);
        e.register(ENCHANTABLE_FORM_VALUE, |e, a| {
            returns(e.mem.u16(a[0] + F_ENCHANT_VALUE) as u32)
        });
        e.register(FLOAT_MIN, |_, a| {
            returns_float(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        e.register(CALC_ITEM_PRICE, |_, a| {
            returns_float(f32::from_bits(a[0]) * f32::from_bits(a[1]) / 100.0)
        });
        e.register(CALC_WEAPON_DAMAGE, |_, a| {
            returns_float(f32::from_bits(a[2]) * f32::from_bits(a[3]))
        });
        e.register(CALC_ARMOR_RATING, |_, a| {
            returns_float(a[0] as f32 * f32::from_bits(a[1]))
        });
        e.register(ARMOR_RATING_FLOAT, |e, a| {
            returns_float(e.mem.f32(a[0] + F_RATING))
        });
        e.register(ROUND_ARMOR_RESULT, |_, a| {
            returns_float(f32::from_bits(a[0]) * 2.0)
        });
        e
    }

    /// An engine that logs every call.
    fn logged() -> Engine {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn extra(e: &mut Engine) -> u32 {
        let x = e.mem.alloc(0x80);
        e.mem.set_u32(x, VTABLE);
        e.mem.set_f32(x + X_HEALTH, -1.0);
        e.mem.set_f32(x + X_FLOAT, -1.0);
        x
    }

    fn set(e: &mut Engine, base: u32, offset: u32, value: u32) {
        e.mem.set_u32(base + offset, value);
    }

    /// A `BSSimpleList` of `items`; an empty one is a head node with a null
    /// item.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    fn items(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    fn form(e: &mut Engine, form_type: u8, value: u32, health: u32) -> u32 {
        let f = e.mem.alloc(0x400);
        e.mem.set_u32(f, VTABLE);
        e.mem.set_u8(f + F_TYPE, form_type);
        set(e, f, F_VALUE, value);
        set(e, f, F_HEALTH_FORM, f);
        set(e, f, F_HEALTH, health);
        f
    }

    fn change(e: &mut Engine, list_head: u32, form: u32) -> Ptr<ItemChange> {
        let c = e.new_object::<ItemChange>();
        e.set(c, ItemChange::pExtraObjectList, Ptr::new(list_head));
        e.set(c, ItemChange::pContainerObj, Ptr::new(form));
        e.set(c, ItemChange::iNumber, 1);
        c
    }

    fn casts(e: &mut Engine, weapon: u32, enchantable: u32, health_form: u32) {
        set(e, CAST_TABLE, 0, weapon);
        set(e, CAST_TABLE, 4, enchantable);
        set(e, CAST_TABLE, 8, health_form);
    }

    #[test]
    fn constructor_stores_form_number_and_an_empty_list() {
        let mut e = logged();
        let this = e.new_object::<ItemChange>();
        let r = e.call(0x004bc550, &args![this, 0x1234u32, 7i32]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.get(this, ItemChange::pContainerObj), Ptr::new(0x1234));
        assert_eq!(e.get(this, ItemChange::iNumber), 7);
        let node = e.get(this, ItemChange::pExtraObjectList).addr();
        assert_ne!(node, 0);
        assert_eq!((e.mem.u32(node), e.mem.u32(node + 4)), (0, 0));
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
    }

    #[test]
    fn release_list_clears_and_deletes_the_list() {
        let mut e = logged();
        let head = list(&mut e, &[]);
        let this = change(&mut e, head, 0);
        e.call(0x004bc5f0, &args![this]);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![head]]);
        assert_eq!(calls(&e, LIST_DESTROY), vec![vec![head, 1]]);
        assert_eq!(e.get(this, ItemChange::pExtraObjectList), Ptr::NULL);
        // Without a list nothing is called.
        let none = change(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x004bc5f0, &args![none]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn copy_constructor_duplicates_every_extra_list() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let source_list = list(&mut e, &[a, b]);
        let other = change(&mut e, source_list, 0x4321);
        e.set(other, ItemChange::iNumber, 9);
        let this = e.new_object::<ItemChange>();
        let r = e.call(0x004bc650, &args![this, other]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.get(this, ItemChange::pContainerObj), Ptr::new(0x4321));
        assert_eq!(e.get(this, ItemChange::iNumber), 9);
        let copies = items(&e, e.get(this, ItemChange::pExtraObjectList).addr());
        assert_eq!(copies.len(), 2);
        assert_eq!(e.mem.u32(copies[0] + 0x14), a);
        assert_eq!(e.mem.u32(copies[1] + 0x14), b);
        assert_eq!(calls(&e, EXTRA_DATA_LIST_DUPLICATE).len(), 2);
    }

    #[test]
    fn delete_all_extra_removes_and_deletes_starting_from_the_node() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        e.call(0x004bc780, &args![this]);
        // The removal copies the next node over the current one, so the walk
        // ends after the first list (the game's behaviour).
        assert_eq!(calls(&e, EXTRA_REMOVE_ALL), vec![vec![a, 1]]);
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![a, 1]]);
        assert_eq!(items(&e, head), vec![b]);
    }

    #[test]
    fn shallow_copy_shares_the_extra_lists() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let source_list = list(&mut e, &[a, b]);
        let other = change(&mut e, source_list, 0x4321);
        let this = e.new_object::<ItemChange>();
        e.call(0x004bc810, &args![this, other]);
        assert_eq!(e.get(this, ItemChange::pContainerObj), Ptr::new(0x4321));
        let copied = items(&e, e.get(this, ItemChange::pExtraObjectList).addr());
        assert_eq!(copied, vec![a, b]);
        // A null source does nothing.
        let untouched = e.new_object::<ItemChange>();
        e.call(0x004bc810, &args![untouched, Ptr::<ItemChange>::NULL]);
        assert_eq!(e.get(untouched, ItemChange::pExtraObjectList), Ptr::NULL);
    }

    #[test]
    fn extra_total_count_skips_default_and_optionally_worn() {
        let mut e = logged();
        let (default, worn, plain) = (extra(&mut e), extra(&mut e), extra(&mut e));
        set(&mut e, default, X_DEFAULT, 1);
        set(&mut e, default, X_COUNT, 100);
        set(&mut e, worn, X_WORN, 1);
        set(&mut e, worn, X_COUNT, 3);
        set(&mut e, plain, X_COUNT, 5);
        let head = list(&mut e, &[default, worn, plain]);
        let this = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bc8f0, &args![this, false]).i32(), 5);
        assert_eq!(e.call(0x004bc8f0, &args![this, true]).i32(), 8);
    }

    #[test]
    fn extra_total_default_count_restarts_after_deleting_an_empty_list() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        for x in [a, b] {
            set(&mut e, x, X_DEFAULT, 1);
        }
        set(&mut e, a, X_COUNT, 4);
        set(&mut e, a, 4, 0x77); // has extras
        set(&mut e, b, X_COUNT, 1); // no extras: pHead is 0
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bc980, &args![this]).i32(), 4);
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![b, 1]]);
        assert_eq!(items(&e, head), vec![a]);
    }

    #[test]
    fn has_no_extras_tests_the_first_extra_pointer() {
        let mut e = engine();
        let x = extra(&mut e);
        assert!(e.call(0x004bca60, &args![x]).bool());
        set(&mut e, x, 4, 1);
        assert!(!e.call(0x004bca60, &args![x]).bool());
    }

    #[test]
    fn amount_non_default_extra_counts_and_collapses_stacks() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        set(&mut e, a, X_COUNT, 2);
        set(&mut e, a, X_ITEMS_IN_LIST, 2);
        set(&mut e, b, X_COUNT, 1);
        set(&mut e, b, X_ITEMS_IN_LIST, 2);
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bca80, &args![this]).i32(), 2);
        // A single item type with a count above one: count removed, list
        // deleted, total restarts at zero.
        let c = extra(&mut e);
        set(&mut e, c, X_COUNT, 3);
        set(&mut e, c, X_ITEMS_IN_LIST, 1);
        let head = list(&mut e, &[c]);
        let other = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bca80, &args![other]).i32(), 0);
        assert_eq!(calls(&e, EXTRA_REMOVE_COUNT), vec![vec![c]]);
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![c, 1]]);
        assert!(items(&e, head).is_empty());
    }

    #[test]
    fn leveled_item_predicates_and_counts() {
        let mut e = engine();
        let (a, b) = (extra(&mut e), extra(&mut e));
        set(&mut e, b, X_HAS_LEVELED, 1);
        set(&mut e, b, X_COUNT, 6);
        set(&mut e, a, X_COUNT, 2);
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        assert!(e.call(0x004bcb70, &args![this]).bool());
        assert_eq!(e.call(0x004bcc30, &args![this]).i32(), 6);
        let empty = list(&mut e, &[]);
        let none = change(&mut e, empty, 0);
        assert!(!e.call(0x004bcb70, &args![none]).bool());
        assert_eq!(e.call(0x004bcc30, &args![none]).i32(), 0);
    }

    #[test]
    fn remove_original_changes_only_touches_lists_with_an_original() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        set(&mut e, b, X_ORIGINAL, 0x55);
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        e.call(0x004bcbd0, &args![this]);
        assert_eq!(calls(&e, EXTRA_REMOVE_ORIGINAL_REFERENCE), vec![vec![b]]);
    }

    #[test]
    fn any_extra_list_test_d030() {
        let mut e = engine();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        assert!(!e.call(0x004bccb0, &args![this]).bool());
        set(&mut e, b, X_D030, 1);
        assert!(e.call(0x004bccb0, &args![this]).bool());
    }

    #[test]
    fn remove_leveled_item_extra_removes_and_unlinks_empty_lists() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        set(&mut e, a, X_LEVELED_ITEM, 0x9000);
        set(&mut e, b, 4, 1);
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bcd10, &args![this]).u32(), 0x9000);
        assert_eq!(calls(&e, EXTRA_REMOVE_EXTRA), vec![vec![a, 0x9000, 0]]);
        // `a` had no extras left, so it is out of the list but not deleted.
        assert_eq!(items(&e, head), vec![b]);
        assert!(calls(&e, DESTRUCTOR).is_empty());
    }

    #[test]
    fn item_health_variants() {
        let mut e = engine();
        // A form without a health form gives -1.0.
        let plain = form(&mut e, 0x28, 0, 100);
        set(&mut e, plain, F_HEALTH_FORM, 0);
        casts(&mut e, 0, 0, 0);
        let empty = list(&mut e, &[]);
        let this = change(&mut e, empty, plain);
        assert_eq!(e.call(0x004bcdb0, &args![this, false]).f32(), -1.0);
        // Base health 100, no health extra.
        let armor = form(&mut e, 0x18, 0, 100);
        let this = change(&mut e, empty, armor);
        assert_eq!(e.call(0x004bcdb0, &args![this, false]).f32(), 100.0);
        assert_eq!(e.call(0x004bcdb0, &args![this, true]).f32(), 100.0);
        // A health extra of 50.
        let x = extra(&mut e);
        e.mem.set_f32(x + X_HEALTH, 50.0);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        assert_eq!(e.call(0x004bcdb0, &args![this, false]).f32(), 50.0);
        assert_eq!(e.call(0x004bcdb0, &args![this, true]).f32(), 50.0);
        // The ratio is capped at 100, not at 1: 250 / 100 stays 2.5.
        e.mem.set_f32(x + X_HEALTH, 250.0);
        assert_eq!(e.call(0x004bcdb0, &args![this, true]).f32(), 250.0);
        // A weapon takes its base health (plus the active health mod) from
        // the weapon, and an extra list without health (-1.0) uses it.
        let weapon = form(&mut e, 0x28, 0, 100);
        set(&mut e, weapon, 0x98, 80);
        set(&mut e, weapon, WEAPON_MOD_ACTION, 10);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE, 15.5);
        casts(&mut e, weapon, 0, 0);
        let y = extra(&mut e);
        set(&mut e, y, X_SLOT_ACTIVE, 1);
        let head = list(&mut e, &[y]);
        let this = change(&mut e, head, weapon);
        assert_eq!(e.call(0x004bcdb0, &args![this, false]).f32(), 95.0);
        set(&mut e, y, X_SLOT_ACTIVE, 0);
        assert_eq!(e.call(0x004bcdb0, &args![this, false]).f32(), 80.0);
    }

    #[test]
    fn weapon_health_adds_the_mod_effect_value() {
        let mut e = engine();
        let weapon = form(&mut e, 0x28, 0, 0);
        set(&mut e, weapon, 0x98, 80);
        set(&mut e, weapon, WEAPON_MOD_ACTION + 4, 10);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE + 4, 15.75);
        assert_eq!(e.call(0x004bcf00, &args![weapon, 0u32]).i32(), 80);
        assert_eq!(e.call(0x004bcf00, &args![weapon, 1u32]).i32(), 95);
        // Without a mod of that type the bonus is zero.
        set(&mut e, weapon, WEAPON_MOD_ACTION + 4, 3);
        assert_eq!(e.call(0x004bcf00, &args![weapon, 1u32]).i32(), 80);
    }

    #[test]
    fn mod_effect_value_picks_the_slot_and_variant() {
        let mut e = engine();
        let weapon = form(&mut e, 0x28, 0, 0);
        set(&mut e, weapon, WEAPON_MOD_ACTION + 8, 6);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE + 8, 2.5);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE_TWO + 8, 4.5);
        assert_eq!(e.call(0x004bcf60, &args![weapon, 6u32, 0u32]).f32(), 2.5);
        assert_eq!(e.call(0x004bcf60, &args![weapon, 6u32, 1u32]).f32(), 4.5);
        assert_eq!(e.call(0x004bcf60, &args![weapon, 6u32, 2u32]).f32(), 0.0);
        assert_eq!(e.call(0x004bcf60, &args![weapon, 7u32, 0u32]).f32(), 0.0);
    }

    #[test]
    fn set_item_health_stores_the_health_extra() {
        let mut e = logged();
        let armor = form(&mut e, 0x18, 0, 100);
        casts(&mut e, 0, 0, armor);
        let owner = e.mem.alloc(0x10);
        set(&mut e, owner, 0, VTABLE);
        let inventory = e.mem.alloc(0x10);
        set(&mut e, inventory, 4, owner);
        let x = extra(&mut e);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        // Below the form's health: stored in the matching extra list, and the
        // owner is notified.
        let health = 50.0f32;
        e.call(0x004bd030, &args![this, health, inventory, x, true]);
        assert_eq!(calls(&e, OWNER_NOTIFY), vec![vec![owner, 0x20]]);
        assert_eq!(calls(&e, EXTRA_SET_HEALTH), vec![vec![x, health.to_bits()]]);
        // Another extra list that is not in the list: a new one is added.
        let other = extra(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004bd030, &args![this, health, inventory, other, true]);
        let stored = items(&e, head);
        assert_eq!(stored.len(), 2);
        assert_eq!(calls(&e, EXTRA_SET_HEALTH).len(), 1);
        assert_eq!(e.mem.u32(stored[1] + 0x10), health.to_bits());
        // No list at all: list node and extra list are made.
        let bare = change(&mut e, 0, armor);
        e.call(0x004bd030, &args![bare, health, inventory, x, true]);
        let node = e.get(bare, ItemChange::pExtraObjectList).addr();
        let made = items(&e, node);
        assert_eq!(made.len(), 1);
        assert_eq!(e.mem.u32(made[0] + 0x10), health.to_bits());
        // No health form: nothing but the owner notification.
        casts(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x004bd030, &args![this, health, inventory, x, true]);
        assert_eq!(calls(&e, EXTRA_SET_HEALTH).len(), 0);
        assert_eq!(calls(&e, OWNER_NOTIFY).len(), 1);
    }

    #[test]
    fn set_item_health_at_or_above_the_base_removes_the_extra() {
        let mut e = logged();
        let armor = form(&mut e, 0x18, 0, 100);
        casts(&mut e, 0, 0, armor);
        let inventory = e.mem.alloc(0x10);
        let x = extra(&mut e);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        // The extra still has other extras: only RemoveHealth.
        set(&mut e, x, 4, 1);
        e.call(0x004bd030, &args![this, 100.0f32, inventory, x, true]);
        assert_eq!(calls(&e, EXTRA_REMOVE_HEALTH), vec![vec![x]]);
        assert!(calls(&e, DESTRUCTOR).is_empty());
        // Now it is empty and `remove_empty` is set: the entry that holds the
        // item type is found and the list is removed from it and deleted.
        set(&mut e, x, 4, 0);
        let entry_list = list(&mut e, &[x]);
        let entry = e.mem.alloc(8);
        set(&mut e, entry, 0, entry_list);
        e.register(INVENTORY_CHANGES_GET_OBJECT_IN_LIST, |e, _| {
            returns(e.mem.u32(CAST_TABLE + 0x10))
        });
        set(&mut e, CAST_TABLE, 0x10, entry);
        e.call_log = Some(vec![]);
        e.call(0x004bd030, &args![this, 150.0f32, inventory, x, true]);
        assert_eq!(
            calls(&e, INVENTORY_CHANGES_GET_OBJECT_IN_LIST),
            vec![vec![inventory, armor, 1, 0]]
        );
        assert!(items(&e, entry_list).is_empty());
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![x, 1]]);
    }

    #[test]
    fn fn_004bd350_prefers_the_float_extra() {
        let mut e = engine();
        let ench = form(&mut e, 0x2a, 0, 0);
        e.mem.set_u16(ench + F_ENCHANT_VALUE, 40);
        casts(&mut e, 0, ench, 0);
        let x = extra(&mut e);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, ench);
        // No float extra (-1.0): the form's 16-bit value.
        assert_eq!(e.call(0x004bd350, &args![this]).f32(), 40.0);
        e.mem.set_f32(x + X_FLOAT, 7.5);
        assert_eq!(e.call(0x004bd350, &args![this]).f32(), 7.5);
        // No extra list at all.
        let empty = list(&mut e, &[]);
        let bare = change(&mut e, empty, ench);
        assert_eq!(e.call(0x004bd350, &args![bare]).f32(), 40.0);
        // Not an enchantable form.
        casts(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x004bd350, &args![this]).f32(), -1.0);
    }

    #[test]
    fn item_value_scales_by_health_and_adds_weapon_mods() {
        let mut e = engine();
        let armor = form(&mut e, 0x18, 200, 100);
        casts(&mut e, 0, 0, 0);
        let x = extra(&mut e);
        e.mem.set_f32(x + X_HEALTH, 50.0);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        // 200 at 50 percent health, rounded to 0.1.
        assert_eq!(e.call(0x004bd400, &args![this]).f32(), 100.0);
        // A weapon with mods in slots 1 and 4 adds each mod object's value.
        let weapon = form(&mut e, 0x28, 200, 100);
        set(&mut e, weapon, 0x98, 100);
        casts(&mut e, weapon, 0, 0);
        let mods = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        set(&mut e, mods[0], MOD_OBJECT_VALUE + 4, 10);
        set(&mut e, mods[1], MOD_OBJECT_VALUE + 4, 20);
        set(&mut e, weapon, WEAPON_MOD_OBJECT, mods[0]);
        set(&mut e, weapon, WEAPON_MOD_OBJECT + 8, mods[1]);
        set(&mut e, x, X_HAS_MODS, 1);
        set(&mut e, x, X_MOD_FLAGS, 5);
        let this = change(&mut e, head, weapon);
        assert_eq!(e.call(0x004bd400, &args![this]).f32(), 130.0);
    }

    #[test]
    fn rounding_to_a_multiple() {
        let mut e = engine();
        assert_eq!(e.call(0x004bd510, &args![1.04f32, 0.1f32]).f32(), 1.0);
        let up = e.call(0x004bd510, &args![1.06f32, 0.1f32]).f32();
        assert!((up - 1.1).abs() < 1e-6);
        assert_eq!(e.call(0x004bd510, &args![0.0f32, 0.1f32]).f32(), 0.0);
    }

    #[test]
    fn mod_object_and_effect_selectors() {
        let mut e = engine();
        let weapon = form(&mut e, 0x28, 0, 0);
        for (i, bit) in [1u32, 2, 4].into_iter().enumerate() {
            let i = i as u32;
            set(&mut e, weapon, WEAPON_MOD_OBJECT + 4 * i, 0x100 + bit);
            set(&mut e, weapon, WEAPON_MOD_ACTION + 4 * i, 0x200 + bit);
            e.mem
                .set_f32(weapon + WEAPON_MOD_ACTION_VALUE + 4 * i, bit as f32);
            e.mem.set_f32(
                weapon + WEAPON_MOD_ACTION_VALUE_TWO + 4 * i,
                10.0 * bit as f32,
            );
            assert_eq!(e.call(0x004bd570, &args![weapon, bit]).u32(), 0x100 + bit);
            assert_eq!(e.call(0x004bd880, &args![weapon, bit]).u32(), 0x200 + bit);
            assert_eq!(
                e.call(0x004bd9d0, &args![weapon, bit, 0u32]).f32(),
                bit as f32
            );
            assert_eq!(
                e.call(0x004bd9d0, &args![weapon, bit, 1u32]).f32(),
                10.0 * bit as f32
            );
            assert_eq!(e.call(0x004bd9d0, &args![weapon, bit, 2u32]).f32(), 0.0);
        }
        assert_eq!(e.call(0x004bd570, &args![weapon, 3u32]).u32(), 0);
        assert_eq!(e.call(0x004bd880, &args![weapon, 8u32]).u32(), 0);
        assert_eq!(e.call(0x004bd9d0, &args![weapon, 0u32, 0u32]).f32(), 0.0);
    }

    #[test]
    fn set_or_clear_the_first_extra_list() {
        let mut e = logged();
        let source = extra(&mut e);
        let x = extra(&mut e);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, 0);
        // Copy into the first list.
        e.call(0x004bd5c0, &args![this, source]);
        assert_eq!(calls(&e, EXTRA_COPY_FROM), vec![vec![x, source]]);
        // No list at all: node and extra list are made.
        let bare = change(&mut e, 0, 0);
        e.call(0x004bd5c0, &args![bare, source]);
        let node = e.get(bare, ItemChange::pExtraObjectList).addr();
        let made = items(&e, node);
        assert_eq!(made.len(), 1);
        assert_eq!(calls(&e, EXTRA_COPY_FROM)[1], vec![made[0], source]);
        // A null source removes and deletes the first list.
        e.call(0x004bd5c0, &args![this, Ptr::<()>::NULL]);
        assert!(items(&e, head).is_empty());
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![x, 1]]);
    }

    #[test]
    fn first_extra_list_accessors() {
        let mut e = engine();
        let x = extra(&mut e);
        set(&mut e, x, X_OWNERSHIP, 0x66);
        set(&mut e, x, X_POISON, 0x77);
        set(&mut e, x, X_HAS_MODS, 1);
        set(&mut e, x, X_MOD_FLAGS, 6);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bd740, &args![this]).u32(), 0x66);
        assert_eq!(e.call(0x004bdcc0, &args![this]).u32(), 0x77);
        assert_eq!(e.call(0x004bd820, &args![this]).u8(), 6);
        set(&mut e, x, X_HAS_MODS, 0);
        assert_eq!(e.call(0x004bd820, &args![this]).u8(), 0);
        let empty = list(&mut e, &[]);
        let none = change(&mut e, empty, 0);
        assert_eq!(e.call(0x004bd740, &args![none]).u32(), 0);
        assert_eq!(e.call(0x004bdcc0, &args![none]).u32(), 0);
        assert_eq!(e.call(0x004bd820, &args![none]).u8(), 0);
    }

    #[test]
    fn hot_key_is_the_first_one_set() {
        let mut e = engine();
        let (a, b, c) = (extra(&mut e), extra(&mut e), extra(&mut e));
        set(&mut e, a, X_HOT_KEY, 0xff); // -1
        set(&mut e, b, X_HOT_KEY, 3);
        set(&mut e, c, X_HOT_KEY, 5);
        let head = list(&mut e, &[a, b, c]);
        let this = change(&mut e, head, 0);
        assert_eq!(e.call(0x004bd7a0, &args![this]).i32(), 3);
        let only = list(&mut e, &[a]);
        let none = change(&mut e, only, 0);
        assert_eq!(e.call(0x004bd7a0, &args![none]).i32(), -1);
    }

    #[test]
    fn mod_effect_active_checks_the_slots_in_order() {
        let mut e = engine();
        let weapon = form(&mut e, 0x28, 0, 0);
        set(&mut e, weapon, WEAPON_MOD_ACTION, 3);
        set(&mut e, weapon, WEAPON_MOD_ACTION + 4, 9);
        set(&mut e, weapon, WEAPON_MOD_ACTION + 8, 9);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE + 4, 1.5);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE + 8, 2.5);
        let x = extra(&mut e);
        set(&mut e, x, X_SLOT_ACTIVE, 6);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, weapon);
        let out = e.mem.alloc(4);
        assert!(e.call(0x004bd8d0, &args![this, 9u32, out]).bool());
        assert_eq!(e.mem.f32(out), 1.5);
        assert!(e.call(0x004bda70, &args![this, 9u32]).bool());
        // Slot 1 has the type but is not active; type 3 is nowhere active.
        assert!(!e.call(0x004bd8d0, &args![this, 3u32, out]).bool());
        assert!(!e.call(0x004bda70, &args![this, 3u32]).bool());
        // No extra list.
        let empty = list(&mut e, &[]);
        let none = change(&mut e, empty, weapon);
        assert!(!e.call(0x004bda70, &args![none, 9u32]).bool());
        assert!(!e.call(0x004bd8d0, &args![none, 9u32, out]).bool());
    }

    #[test]
    fn modding_flag_is_set_on_the_first_list_or_a_new_one() {
        let mut e = logged();
        let x = extra(&mut e);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, 0);
        e.call(0x004bdb40, &args![this]);
        assert_eq!(e.mem.u32(x + 0x0c), 1);
        e.call(0x004bdc70, &args![this]);
        assert_eq!(calls(&e, EXTRA_REMOVE_IS_MODDING), vec![vec![x]]);
        // An empty list node gets a new extra list.
        let empty = list(&mut e, &[]);
        let bare = change(&mut e, empty, 0);
        e.call(0x004bdb40, &args![bare]);
        let made = items(&e, empty);
        assert_eq!(made.len(), 1);
        assert_eq!(e.mem.u32(made[0] + 0x0c), 1);
        // No list at all: the node is made too.
        let nothing = change(&mut e, 0, 0);
        e.call(0x004bdb40, &args![nothing]);
        let node = e.get(nothing, ItemChange::pExtraObjectList).addr();
        assert_eq!(items(&e, node).len(), 1);
    }

    #[test]
    fn poison_helpers() {
        let mut e = logged();
        let x = extra(&mut e);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, 0);
        // No poison: the rank setter runs, the remover does not.
        e.call(0x004bdd20, &args![this, 4u32]);
        e.call(0x004bdd80, &args![this]);
        assert_eq!(calls(&e, EXTRA_SET_RANK), vec![vec![x, 4]]);
        assert!(calls(&e, EXTRA_REMOVE_POISON).is_empty());
        set(&mut e, x, X_POISON, 0x99);
        e.call(0x004bdd20, &args![this, 5u32]);
        e.call(0x004bdd80, &args![this]);
        assert_eq!(calls(&e, EXTRA_SET_RANK).len(), 1);
        assert_eq!(calls(&e, EXTRA_REMOVE_POISON), vec![vec![x]]);
    }

    #[test]
    fn worn_script_and_locals_search_all_lists() {
        let mut e = logged();
        let (a, b) = (extra(&mut e), extra(&mut e));
        set(&mut e, b, X_WORN, 1);
        set(&mut e, b, X_SCRIPT, 0x31);
        set(&mut e, b, X_LOCALS, 0x32);
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        assert!(e.call(0x004bddd0, &args![this, 1u32]).bool());
        assert_eq!(calls(&e, EXTRA_GET_WORN), vec![vec![a, 1], vec![b, 1]]);
        assert_eq!(e.call(0x004bde40, &args![this]).u32(), 0x31);
        assert_eq!(e.call(0x004bdea0, &args![this]).u32(), 0x32);
        let only = list(&mut e, &[a]);
        let plain = change(&mut e, only, 0);
        assert!(!e.call(0x004bddd0, &args![plain, 0u32]).bool());
        assert_eq!(e.call(0x004bde40, &args![plain]).u32(), 0);
        assert_eq!(e.call(0x004bdea0, &args![plain]).u32(), 0);
    }

    #[test]
    fn attack_damage_is_for_weapon_forms_only() {
        let mut e = logged();
        let weapon = form(&mut e, 0x28, 0, 200);
        casts(&mut e, 0, 0, 0);
        let x = extra(&mut e);
        e.mem.set_f32(x + X_HEALTH, 100.0);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, weapon);
        let attacker = 0x0123_4560u32;
        // Health ratio 100 / 200 = 0.5; the double returns ratio * scale.
        let r = e
            .call(0x004bdf00, &args![this, attacker, 4.0f32, 1u32])
            .f32();
        assert_eq!(r, 2.0);
        assert_eq!(
            calls(&e, CALC_WEAPON_DAMAGE),
            vec![vec![
                attacker,
                weapon,
                0.5f32.to_bits(),
                4.0f32.to_bits(),
                0,
                1,
                this.addr(),
                1
            ]]
        );
        let other = form(&mut e, 0x18, 0, 200);
        let this = change(&mut e, head, other);
        assert_eq!(
            e.call(0x004bdf00, &args![this, attacker, 4.0f32, 1u32])
                .f32(),
            0.0
        );
    }

    #[test]
    fn armor_rating_uses_the_rating_and_health_ratio() {
        let mut e = logged();
        let armor = form(&mut e, 0x18, 0, 200);
        e.mem.set_f32(armor + F_RATING, 12.75);
        let x = extra(&mut e);
        e.mem.set_f32(x + X_HEALTH, 100.0);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        // rating 12 (truncated) * ratio 0.5 = 6, doubled by the rounding stub.
        assert_eq!(e.call(0x004bdf90, &args![this, 0u32]).f32(), 12.0);
        assert_eq!(
            calls(&e, CALC_ARMOR_RATING),
            vec![vec![12, 0.5f32.to_bits()]]
        );
        // Zero form health: the ratio is zero and the item health is not read.
        set(&mut e, armor, F_HEALTH, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004bdf90, &args![this, 0u32]).f32(), 0.0);
        assert!(calls(&e, EXTRA_GET_HEALTH).is_empty());
        // Another form type: -1.0 goes through the rounding.
        let weapon = form(&mut e, 0x28, 0, 200);
        let this = change(&mut e, head, weapon);
        assert_eq!(e.call(0x004bdf90, &args![this, 0u32]).f32(), -2.0);
    }

    #[test]
    fn truncation_matches_fistp() {
        assert_eq!(truncate_to_i32(12.75), 12);
        assert_eq!(truncate_to_i32(-12.75), -12);
        assert_eq!(truncate_to_i32(3.0e10), i32::MIN);
        assert_eq!(truncate_to_i32(f64::NAN), i32::MIN);
    }

    // ---- Tests of the second session (004be060 to 004c0c60) ----

    // More words of the fake extra data list (the base fake has 0x80 bytes).
    const X_CAN_NOT_WEAR: u32 = 0x68;
    const X_REFERENCE: u32 = 0x6c;
    const X_SAVE_SIZE: u32 = 0x70;
    const X_WORN_FLAG: u32 = 0x74;

    /// Virtual table whose slot at byte offset `s` points at `VCALL_BASE + s`
    /// (so `e.vcall(object, s, ..)` calls the double registered there).
    const BIG_VTABLE: u32 = 0x0210_0000;
    const VCALL_BASE: u32 = 0x0210_1000;
    /// The fake save/load game object (+0x14 position, +0x20 "use blocks",
    /// +0x30 a count, +0x80 version, +0x84 form being loaded, +0x88 form being
    /// saved), the stream buffer, the log flag byte, the form lookup cell and
    /// the player object.
    const SAVE_OBJECT: u32 = 0x0210_2000;
    const STREAM: u32 = 0x0211_0000;
    const LOG_FLAG: u32 = 0x0210_2100;
    const LOOKUP_CELL: u32 = 0x0210_2200;
    const PLAYER: u32 = 0x0210_8000;
    const STRINGS: u32 = 0x0210_3000;
    /// Cast results by target: biped form, NPC, full name, bound object.
    const CAST_BIPED: u32 = CAST_TABLE + 0x20;
    const CAST_NPC: u32 = CAST_TABLE + 0x24;
    const CAST_FULL_NAME: u32 = CAST_TABLE + 0x28;
    const CAST_BOUND: u32 = CAST_TABLE + 0x2c;

    fn vcall_address(slot: u32) -> u32 {
        VCALL_BASE + slot
    }

    /// A fake object whose vtable is `BIG_VTABLE` (0x400 bytes).
    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0x400);
        e.mem.set_u32(o, BIG_VTABLE);
        o
    }

    fn put_str(e: &mut Engine, address: u32, text: &[u8]) {
        e.mem.set_cstr(address, text);
    }

    /// The engine with the doubles of the second session.
    fn more() -> Engine {
        let mut e = logged();
        e.map(0x011c_0000, 0x2_0000);
        e.map(0x0210_0000, 0x4_0000);
        let slots: Vec<u32> = (0..0x200).map(|i| VCALL_BASE + 4 * i).collect();
        e.put_vtable(BIG_VTABLE, &slots);
        e.set_global(SAVE_LOAD_GAME_GLOBAL, SAVE_OBJECT);
        e.set_global(PLAYER_GLOBAL, PLAYER);
        for address in [
            SCOPE_GUARD_OPEN,
            SCOPE_GUARD_CLOSE,
            ERROR_LOG,
            SAVE_LOAD_LOG,
            ITEM_CHANGE_DELETE,
            OPERATOR_DELETE,
            FAST_ITERATOR_DESTRUCT,
            EXTRA_AFTER_LOAD_GAME,
            EXTRA_AFTER_LOAD_GAME_BUFFER,
            EXTRA_SAVE_GAME_BUFFER,
            EXTRA_LOAD_GAME_BUFFER,
            SAVE_BUFFER_SAVE_FORM_ID,
            SAVE_BUFFER_SAVE_DATA,
            SAVE_BUFFER_END_SIZED,
            FORM_SET_TEMPORARY,
            REFERENCE_FORM_NOTIFY,
            ACTOR_RELOAD_TARGETS,
            ACTOR_INIT_LIGHTING,
            ANIMATION_STORE_FLOAT,
            PLAYER_STORE_FORM,
            REFR_UPDATE_WEAPON,
            TASK_QUEUE_ATTACH_WEAPON,
            STORE_NOTIFY_004534F0,
            HANDLE_ENTRY_POINT,
        ] {
            stub(&mut e, address);
        }
        // The fake extra data lists are 0x80 bytes: allocate at least that.
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0].max(0x80))));
        e.register(LIST_CONTAINS, |e, a| {
            let mut node = a[0];
            let wanted = e.mem.u32(a[1]);
            while node != 0 && e.mem.u32(node) != wanted {
                node = e.mem.u32(node + 4);
            }
            returns((node != 0) as u32)
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(EXTRA_COMPARE_LIST, |_, a| returns((a[0] != a[1]) as u32));
        e.register(EXTRA_COPY_LIST, |e, a| {
            e.mem.set_u32(a[0] + 0x14, a[1]);
            let count = e.mem.u32(a[1] + X_COUNT);
            e.mem.set_u32(a[0] + X_COUNT, count);
            Ret::default()
        });
        e.register(EXTRA_SET_COUNT, |e, a| {
            e.mem.set_u32(a[0] + X_COUNT, a[1] & 0xffff);
            Ret::default()
        });
        e.register(EXTRA_SET_WORN, |e, a| {
            e.mem.set_u32(a[0] + X_WORN, a[1]);
            e.mem.set_u32(a[0] + X_WORN_FLAG, a[2]);
            Ret::default()
        });
        e.register(EXTRA_SET_CAN_NOT_WEAR, |e, a| {
            e.mem.set_u32(a[0] + X_CAN_NOT_WEAR, a[1]);
            Ret::default()
        });
        read_word!(e, EXTRA_GET_CAN_NOT_WEAR, X_CAN_NOT_WEAR);
        e.register(EXTRA_REMOVE_HOT_KEY, |e, a| {
            e.mem.set_u32(a[0] + X_HOT_KEY, 0xff);
            Ret::default()
        });
        e.register(EXTRA_SET_HOT_KEY, |e, a| {
            e.mem.set_u32(a[0] + X_HOT_KEY, a[1]);
            Ret::default()
        });
        read_word!(e, EXTRA_GET_REFERENCE_POINTER, X_REFERENCE);
        read_word!(e, EXTRA_GET_SAVE_SIZE, X_SAVE_SIZE);
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0xc)));
        e.register(LOOKUP_FORM_BY_ID, |e, _| returns(e.mem.u32(LOOKUP_CELL)));
        e.register(REFR_EXTRA_LIST, |_, a| returns(a[0] + 0x40));
        e.register(EXTRA_GET_CONTAINER_CHANGES, |e, a| {
            returns(e.mem.u32(a[0] + 0x1c))
        });
        e.register(EXTRA_SET_CONTAINER_CHANGES, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, a[1]);
            Ret::default()
        });
        // The save/load game object.
        e.register(USE_SAVE_GAME_BLOCKS, |e, a| returns(e.mem.u32(a[0] + 0x20)));
        e.register(SAVE_POSITION, |e, a| returns(e.mem.u32(a[0] + 0x14)));
        e.register(CURRENT_SAVE_FORM, |e, a| returns(e.mem.u32(a[0] + 0x88)));
        e.register(CURRENT_LOAD_FORM, |e, a| returns(e.mem.u32(a[0] + 0x84)));
        e.register(LOAD_VERSION, |e, a| returns(e.mem.u8(a[0] + 0x80) as u32));
        e.register(FLAG_BYTE_ADDRESS, |_, _| returns(LOG_FLAG));
        for address in [SAVE_BYTES, SAVE_NUMERIC_ID] {
            e.register(address, |e, a| {
                let position = e.mem.u32(a[0] + 0x14);
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(position, &bytes);
                e.mem.set_u32(a[0] + 0x14, position + a[2]);
                Ret::default()
            });
        }
        for address in [LOAD_BYTES, LOAD_NUMERIC_ID] {
            e.register(address, |e, a| {
                let position = e.mem.u32(a[0] + 0x14);
                let bytes = e.mem.bytes(position, a[2]);
                e.mem.write(a[1], &bytes);
                e.mem.set_u32(a[0] + 0x14, position + a[2]);
                Ret::default()
            });
        }
        e.register(EXTRA_SAVE_GAME, |e, a| {
            let position = e.mem.u32(SAVE_OBJECT + 0x14);
            e.mem.write(position, &[0xee, 0xee, 0xee]);
            e.mem.set_u32(SAVE_OBJECT + 0x14, position + 3);
            assert_eq!((a[1], a[2]), (0, 0));
            Ret::default()
        });
        e.register(EXTRA_LOAD_GAME, |e, a| {
            let position = e.mem.u32(SAVE_OBJECT + 0x14);
            e.mem.set_u32(a[0] + 0x14, position);
            e.mem.set_u32(SAVE_OBJECT + 0x14, position + 3);
            Ret::default()
        });
        // Buffers.
        e.register(SAVE_BUFFER_START_SIZED, |_, _| returns(0x77));
        e.register(LOAD_BUFFER_LOAD_FORM_ID, |_, _| returns(0x1234));
        e.register(LOAD_BUFFER_LOAD_SIZED, |e, _| {
            returns(e.mem.u32(SAVE_OBJECT + 0x30))
        });
        e.register(LOAD_BUFFER_LOAD_DATA, |e, a| {
            e.mem.set_u32(a[1], 7);
            Ret::default()
        });
        // Hot key table of the player.
        e.register(HOT_KEY_TABLE_ENTRY, |_, a| returns(a[0] + 0x100 + 4 * a[1]));
        e.register(HOT_KEY_TABLE_SET, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], value);
            Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        // Casts by (source, target) type.
        e.register(RT_DYNAMIC_CAST, |e, a| {
            returns(match (a[2], a[3]) {
                (_, TYPE_TES_OBJECT_WEAP) => e.mem.u32(CAST_TABLE),
                (_, TYPE_TES_ENCHANTABLE_FORM) => e.mem.u32(CAST_TABLE + 4),
                (_, TYPE_TES_HEALTH_FORM) => e.mem.u32(CAST_TABLE + 8),
                (_, TYPE_TES_BIPED_MODEL_FORM) => e.mem.u32(CAST_BIPED),
                (_, TYPE_TES_NPC) => e.mem.u32(CAST_NPC),
                (_, TYPE_TES_FULL_NAME) => e.mem.u32(CAST_FULL_NAME),
                (TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT) => e.mem.u32(CAST_BOUND),
                other => panic!("unexpected cast {other:08x?}"),
            })
        });
        e
    }

    #[test]
    fn armor_rating_float_is_the_rating_over_one_hundred() {
        let mut e = more();
        // The base fake engine stands in for `004be060`: use the translation.
        let (_, real) = funcs().into_iter().find(|(a, _)| *a == 0x004be060).unwrap();
        e.register(0x004be060, real);
        let armor = form(&mut e, 0x18, 0, 0);
        e.mem.set_u16(armor + ARMOR_RATING, 250);
        assert_eq!(e.call(0x004be080, &args![armor]).f32(), 2.5);
        // 004be060 forwards to it.
        assert_eq!(e.call(0x004be060, &args![armor]).f32(), 2.5);
    }

    #[test]
    fn damage_threshold_goes_through_the_armor_rating_formula() {
        let mut e = more();
        let armor = form(&mut e, 0x18, 0, 200);
        e.mem.set_f32(armor + ARMOR_DAMAGE_THRESHOLD, 12.75);
        let x = extra(&mut e);
        e.mem.set_f32(x + X_HEALTH, 100.0);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        // threshold 12 (truncated) * ratio 0.5 = 6, doubled by the rounding stub.
        assert_eq!(e.call(0x004be0b0, &args![this, 0u32]).f32(), 12.0);
        assert_eq!(
            calls(&e, CALC_ARMOR_RATING),
            vec![vec![12, 0.5f32.to_bits()]]
        );
        // No form health: the ratio is zero and the item health not read.
        set(&mut e, armor, F_HEALTH, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004be0b0, &args![this, 0u32]).f32(), 0.0);
        assert!(calls(&e, EXTRA_GET_HEALTH).is_empty());
        // Another form type: -1.0 through the rounding.
        let weapon = form(&mut e, 0x28, 0, 200);
        let other = change(&mut e, head, weapon);
        assert_eq!(e.call(0x004be0b0, &args![other, 0u32]).f32(), -2.0);
    }

    #[test]
    fn damage_threshold_reads_the_float_at_17c() {
        let mut e = more();
        let armor = form(&mut e, 0x18, 0, 0);
        e.mem.set_f32(armor + 0x17c, 3.25);
        assert_eq!(e.call(0x004be180, &args![armor]).f32(), 3.25);
    }

    #[test]
    fn item_class_number_depends_on_the_form_type() {
        let mut e = more();
        let head = list(&mut e, &[]);
        for (form_type, expected) in [(0x18, 4), (0x1a, 4), (0x28, 3), (0x29, 5), (0x20, 5)] {
            let f = form(&mut e, form_type, 0, 0);
            let this = change(&mut e, head, f);
            assert_eq!(e.call(0x004be1a0, &args![this]).u32(), expected);
        }
    }

    #[test]
    fn icon_one_prefers_the_owner_sex_then_the_form_then_the_default() {
        let mut e = more();
        const BIPED_ICON: u32 = STRINGS + 0x100;
        const FORM_ICON_STRING: u32 = STRINGS + 0x200;
        const DEFAULT_ICON: u32 = STRINGS + 0x300;
        put_str(&mut e, BIPED_ICON, b"male.dds");
        put_str(&mut e, FORM_ICON_STRING, b"form.dds");
        put_str(&mut e, DEFAULT_ICON, b"none.dds");
        e.register(ACTOR_BASE_GET_SEX, |e, a| returns(e.mem.u32(a[0] + 0x40)));
        e.register(BIPED_ICON_FOR_SEX, |_, a| {
            returns(STRINGS + 0x100 + a[1] * 0x10)
        });
        e.register(FORM_ICON, |_, _| returns(FORM_ICON_STRING));
        e.register(STRING_OBJECT_TO_CHARS, |_, a| {
            assert_eq!(a[0], NO_ICON_STRING);
            returns(DEFAULT_ICON)
        });
        let armor = form(&mut e, 0x18, 0, 0);
        let npc = e.mem.alloc(0x80);
        set(&mut e, CAST_TABLE, 0x20, armor);
        set(&mut e, CAST_TABLE, 0x24, npc);
        let x = extra(&mut e);
        set(&mut e, x, X_OWNERSHIP, npc);
        let head = list(&mut e, &[x]);
        let this = change(&mut e, head, armor);
        assert_eq!(e.call(0x004be200, &args![this, 3u32]).u32(), BIPED_ICON);
        // The sex picks the string.
        put_str(&mut e, STRINGS + 0x110, b"female.dds");
        set(&mut e, npc, 0x40, 1);
        assert_eq!(
            e.call(0x004be200, &args![this, 3u32]).u32(),
            STRINGS + 0x110
        );
        // An empty string for that sex falls back to the form's icon with the
        // variant.
        put_str(&mut e, STRINGS + 0x110, b"");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004be200, &args![this, 3u32]).u32(),
            FORM_ICON_STRING
        );
        assert_eq!(calls(&e, FORM_ICON), vec![vec![armor, 3]]);
        // No owner: straight to the form's icon.
        set(&mut e, x, X_OWNERSHIP, 0);
        assert_eq!(
            e.call(0x004be200, &args![this, 4u32]).u32(),
            FORM_ICON_STRING
        );
        // An empty form icon gives the default string object.
        put_str(&mut e, FORM_ICON_STRING, b"");
        assert_eq!(e.call(0x004be200, &args![this, 4u32]).u32(), DEFAULT_ICON);
    }

    #[test]
    fn full_name_needs_a_full_name_form_the_lookup_accepts() {
        let mut e = more();
        const NAME: u32 = STRINGS + 0x400;
        const NO_NAME: u32 = STRINGS + 0x500;
        e.register(FULL_NAME_TEST, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(FULL_NAME_GET, |_, _| returns(NAME));
        e.register(STRING_OBJECT_TO_CHARS, |_, a| {
            assert_eq!(a[0], NO_NAME_STRING);
            returns(NO_NAME)
        });
        let f = form(&mut e, 0x18, 0, 0);
        let head = list(&mut e, &[]);
        let this = change(&mut e, head, f);
        // Not a TESFullName.
        assert_eq!(e.call(0x004be2d0, &args![this]).u32(), NO_NAME);
        let named = e.mem.alloc(0x20);
        set(&mut e, CAST_TABLE, 0x28, named);
        // The lookup rejects it.
        assert_eq!(e.call(0x004be2d0, &args![this]).u32(), NO_NAME);
        set(&mut e, named, 4, 1);
        assert_eq!(e.call(0x004be2d0, &args![this]).u32(), NAME);
    }

    #[test]
    fn form_enchanting_reads_the_word_of_the_enchantable_form() {
        let mut e = more();
        let f = form(&mut e, 0x18, 0, 0);
        assert_eq!(e.call(0x004be330, &args![f]).u32(), 0);
        let enchantable = e.mem.alloc(0x20);
        set(&mut e, enchantable, 4, 0x1234_5678);
        set(&mut e, CAST_TABLE, 4, enchantable);
        assert_eq!(e.call(0x004be330, &args![f]).u32(), 0x1234_5678);
    }

    #[test]
    fn form_weight_subtracts_the_weight_mod_effect() {
        let mut e = more();
        e.register(GET_WEIGHT, |e, a| returns_float(e.mem.f32(a[0])));
        let weapon = form(&mut e, 0x28, 0, 0);
        e.mem.set_f32(weapon + WEAPON_WEIGHT_FORM, 3.0);
        set(&mut e, weapon, WEAPON_MOD_ACTION, 4);
        e.mem.set_f32(weapon + WEAPON_MOD_ACTION_VALUE, 0.5);
        assert_eq!(e.call(0x004be380, &args![weapon, 0u32]).f32(), 3.0);
        assert_eq!(e.call(0x004be380, &args![weapon, 1u32]).f32(), 2.5);
        assert_eq!(calls(&e, GET_WEIGHT)[0], vec![weapon + 0x8c]);
    }

    #[test]
    fn set_worn_adds_a_worn_list_or_removes_and_deletes_the_worn_one() {
        let mut e = more();
        // Wear: a new list marked worn is appended; a missing node is made.
        let this = change(&mut e, 0, 0);
        e.call(0x004be3d0, &args![this, 1u32, 5u32, 0u32]);
        let head = e.get(this, ItemChange::pExtraObjectList).addr();
        let added = items(&e, head);
        assert_eq!(added.len(), 1);
        assert_eq!(e.mem.u32(added[0] + X_WORN), 1);
        assert_eq!(e.mem.u32(added[0] + X_WORN_FLAG), 5);
        // Unwear: the worn list is taken out of the list; deleted on request.
        let (plain, worn) = (extra(&mut e), extra(&mut e));
        set(&mut e, worn, X_WORN, 1);
        let head = list(&mut e, &[plain, worn]);
        let this = change(&mut e, head, 0);
        e.call_log = Some(vec![]);
        e.call(0x004be3d0, &args![this, 0u32, 0u32, 0u32]);
        assert_eq!(items(&e, head), vec![plain]);
        assert!(calls(&e, DESTRUCTOR).is_empty());
        let head = list(&mut e, &[plain, worn]);
        let this = change(&mut e, head, 0);
        e.call(0x004be3d0, &args![this, 0u32, 0u32, 1u32]);
        assert_eq!(items(&e, head), vec![plain]);
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![worn, 1]]);
        // Nothing worn: nothing removed.
        let head = list(&mut e, &[plain]);
        let this = change(&mut e, head, 0);
        e.call(0x004be3d0, &args![this, 0u32, 0u32, 1u32]);
        assert_eq!(items(&e, head), vec![plain]);
    }

    #[test]
    fn byte_at_240() {
        let mut e = more();
        let o = object(&mut e);
        e.mem.set_u8(o + 0x240, 0x5a);
        assert_eq!(e.call(0x004be580, &args![o]).u8(), 0x5a);
    }

    const NAME_STRING: u32 = STRINGS + 0x600;

    /// Makes the virtual 0x130 (the name of a form) of the fake objects return
    /// `NAME_STRING`.
    fn name_slot(e: &mut Engine) {
        e.register(vcall_address(0x130), |_, _| returns(NAME_STRING));
    }

    /// A record of "the form being saved/loaded": form id at +0, flags (a
    /// word at the unaligned +5) and the version byte at +9.
    fn current_record(e: &mut Engine, at: u32, form_id: u32, flags: u32, version: u8) {
        e.mem.set_u32(at, form_id);
        e.mem.set_u32(at + 5, flags);
        e.mem.set_u8(at + 9, version);
    }

    /// The stream the save/load doubles read and write.
    fn start_stream(e: &mut Engine) {
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
    }

    fn stream_position(e: &Engine) -> u32 {
        e.mem.u32(SAVE_OBJECT + 0x14)
    }

    #[test]
    fn save_size_adds_the_header_and_every_list() {
        let mut e = more();
        let (a, b) = (extra(&mut e), extra(&mut e));
        set(&mut e, a, X_SAVE_SIZE, 10);
        set(&mut e, b, X_SAVE_SIZE, 20);
        let head = list(&mut e, &[a, b]);
        let this = change(&mut e, head, 0);
        // Save game blocks add the 4 byte tag and the 2 byte length.
        set(&mut e, SAVE_OBJECT, 0x20, 1);
        assert_eq!(e.call(0x004be5a0, &args![this]).u16(), 6 + 12 + 30);
        set(&mut e, SAVE_OBJECT, 0x20, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004be5a0, &args![this]).u16(), 12 + 30);
        assert!(calls(&e, ERROR_LOG).is_empty());
        assert_eq!(
            calls(&e, EXTRA_GET_SAVE_SIZE),
            vec![vec![a, 0, 0], vec![b, 0, 0]]
        );
        // An empty list adds nothing.
        let empty = list(&mut e, &[]);
        let none = change(&mut e, empty, 0);
        assert_eq!(e.call(0x004be5a0, &args![none]).u16(), 12);
    }

    #[test]
    fn save_size_logging_reports_the_size_with_or_without_the_form() {
        let mut e = more();
        name_slot(&mut e);
        let head = list(&mut e, &[]);
        let this = change(&mut e, head, 0);
        e.mem.set_u8(LOG_FLAG, 1);
        e.call(0x004be5a0, &args![this]);
        assert_eq!(
            calls(&e, ERROR_LOG),
            vec![vec![0x0101_2c78, 12, 0x622, SOURCE_FILE_NAME]]
        );
        // With the form being saved: its id, name and flags.
        let saved_form = object(&mut e);
        e.mem.set_u32(LOOKUP_CELL, saved_form);
        current_record(&mut e, 0x0210_2300, 0x14, 0x20, 3);
        set(&mut e, SAVE_OBJECT, 0x88, 0x0210_2300);
        e.call_log = Some(vec![]);
        e.call(0x004be5a0, &args![this]);
        assert_eq!(
            calls(&e, ERROR_LOG),
            vec![vec![
                0x0101_2cb0,
                12,
                0x14,
                NAME_STRING,
                0x20,
                0x622,
                SOURCE_FILE_NAME
            ]]
        );
        assert_eq!(calls(&e, LOOKUP_FORM_BY_ID), vec![vec![0x14]]);
    }

    #[test]
    fn save_writes_the_block_the_ids_and_patches_the_counts() {
        let mut e = more();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let head = list(&mut e, &[a, b]);
        let item_form = form(&mut e, 0x28, 0, 0);
        set(&mut e, item_form, 0xc, 0xabc);
        let this = change(&mut e, head, item_form);
        e.set(this, ItemChange::iNumber, 3);
        start_stream(&mut e);
        set(&mut e, SAVE_OBJECT, 0x20, 1);
        e.call(0x004be6f0, &args![this]);
        // tag, 16 bit length, form id, number, number of lists, the lists.
        assert_eq!(e.mem.u32(STREAM), 0x424c_4f4b);
        assert_eq!(stream_position(&e), STREAM + 24);
        assert_eq!(e.mem.u16(STREAM + 4), 20);
        assert_eq!(e.mem.u32(STREAM + 6), 0xabc);
        assert_eq!(e.mem.u32(STREAM + 10), 3);
        assert_eq!(e.mem.u32(STREAM + 14), 2);
        assert_eq!(e.mem.bytes(STREAM + 18, 6), vec![0xee; 6]);
        assert!(calls(&e, SAVE_LOAD_LOG).is_empty());
        // Without blocks there is no tag and no length.
        set(&mut e, SAVE_OBJECT, 0x20, 0);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM + 0x100);
        e.call(0x004be6f0, &args![this]);
        assert_eq!(e.mem.u32(STREAM + 0x100), 0xabc);
        assert_eq!(e.mem.u32(STREAM + 0x108), 2);
        assert_eq!(stream_position(&e), STREAM + 0x100 + 18);
    }

    #[test]
    fn save_logging_reports_the_bytes_written() {
        let mut e = more();
        name_slot(&mut e);
        let head = list(&mut e, &[]);
        let item_form = form(&mut e, 0x28, 0, 0);
        let this = change(&mut e, head, item_form);
        start_stream(&mut e);
        e.mem.set_u8(LOG_FLAG, 1);
        e.call(0x004be6f0, &args![this]);
        assert_eq!(
            calls(&e, ERROR_LOG),
            vec![vec![0x0101_536c, 12, 0x641, SOURCE_FILE_NAME]]
        );
        let saved_form = object(&mut e);
        e.mem.set_u32(LOOKUP_CELL, saved_form);
        current_record(&mut e, 0x0210_2300, 0x14, 0x20, 3);
        set(&mut e, SAVE_OBJECT, 0x88, 0x0210_2300);
        e.call_log = Some(vec![]);
        e.call(0x004be6f0, &args![this]);
        assert_eq!(
            calls(&e, ERROR_LOG),
            vec![vec![
                0x0101_53a0,
                12,
                0x14,
                NAME_STRING,
                0x20,
                0x641,
                SOURCE_FILE_NAME
            ]]
        );
    }

    /// The stream of a saved item: tag, length, form id, number, number of
    /// lists, then the 3 byte lists.
    fn put_item_stream(e: &mut Engine, tag: u32, length: u16, lists: u32) {
        e.mem.set_u32(STREAM, tag);
        e.mem.set_u16(STREAM + 4, length);
        e.mem.set_u32(STREAM + 6, 0xabc);
        e.mem.set_u32(STREAM + 10, 9);
        e.mem.set_u32(STREAM + 14, lists);
        start_stream(e);
    }

    #[test]
    fn load_reads_the_form_the_number_and_the_lists() {
        let mut e = more();
        let loaded = form(&mut e, 0x28, 0, 0);
        e.mem.set_u32(LOOKUP_CELL, loaded);
        set(&mut e, CAST_TABLE, 0x2c, 0x5555);
        set(&mut e, SAVE_OBJECT, 0x20, 1);
        put_item_stream(&mut e, 0x424c_4f4b, 20, 2);
        let this = e.new_object::<ItemChange>();
        e.call(0x004be930, &args![this]);
        assert_eq!(e.get(this, ItemChange::pContainerObj), Ptr::new(0x5555));
        assert_eq!(e.get(this, ItemChange::iNumber), 9);
        let lists = items(&e, e.get(this, ItemChange::pExtraObjectList).addr());
        assert_eq!(lists.len(), 2);
        // Each list was loaded from its own 3 bytes of the stream.
        assert_eq!(e.mem.u32(lists[0] + 0x14), STREAM + 18);
        assert_eq!(e.mem.u32(lists[1] + 0x14), STREAM + 21);
        assert_eq!(calls(&e, EXTRA_LOAD_GAME).len(), 2);
        assert_eq!(calls(&e, LOOKUP_FORM_BY_ID), vec![vec![0xabc]]);
        // The length matches the bytes read: nothing is logged.
        assert!(calls(&e, SAVE_LOAD_LOG).is_empty());
        // No lists, no blocks: the list pointer stays null.
        set(&mut e, SAVE_OBJECT, 0x20, 0);
        e.mem.set_u32(STREAM, 0xabc);
        e.mem.set_u32(STREAM + 4, 9);
        e.mem.set_u32(STREAM + 8, 0);
        start_stream(&mut e);
        let empty = e.new_object::<ItemChange>();
        e.set(empty, ItemChange::pExtraObjectList, Ptr::new(0x1234));
        e.call(0x004be930, &args![empty]);
        assert_eq!(e.get(empty, ItemChange::pExtraObjectList), Ptr::NULL);
        assert_eq!(e.get(empty, ItemChange::iNumber), 9);
    }

    #[test]
    fn load_logs_a_wrong_tag_and_a_wrong_length() {
        let mut e = more();
        name_slot(&mut e);
        let loaded = object(&mut e);
        e.mem.set_u32(LOOKUP_CELL, loaded);
        set(&mut e, SAVE_OBJECT, 0x20, 1);
        e.mem.set_u8(SAVE_OBJECT + 0x80, 0x2a);
        // Wrong tag, no form being loaded: the version is logged.
        put_item_stream(&mut e, 0x1234_5678, 20, 2);
        let this = e.new_object::<ItemChange>();
        e.call(0x004be930, &args![this]);
        assert_eq!(
            calls(&e, SAVE_LOAD_LOG),
            vec![vec![0x0101_56a8, SOURCE_FILE_NAME, 0x647, 0x2a]]
        );
        // Length too short: overrun of 2 bytes; too long: underrun.
        e.call_log = Some(vec![]);
        put_item_stream(&mut e, 0x424c_4f4b, 18, 2);
        e.call(0x004be930, &args![this]);
        put_item_stream(&mut e, 0x424c_4f4b, 22, 2);
        e.call(0x004be930, &args![this]);
        assert_eq!(
            calls(&e, SAVE_LOAD_LOG),
            vec![
                vec![0x0101_54a0, 2, SOURCE_FILE_NAME, 0x66b, 0x2a],
                vec![0x0101_5440, 2, SOURCE_FILE_NAME, 0x66b, 0x2a],
            ]
        );
        // With a form being loaded the messages carry its id, name, version
        // and flags.
        current_record(&mut e, 0x0210_2300, 0x14, 0x20, 7);
        set(&mut e, SAVE_OBJECT, 0x84, 0x0210_2300);
        e.call_log = Some(vec![]);
        put_item_stream(&mut e, 0x1234_5678, 18, 2);
        e.call(0x004be930, &args![this]);
        let logged_calls = calls(&e, SAVE_LOAD_LOG);
        assert_eq!(
            logged_calls,
            vec![
                vec![
                    0x0101_5718,
                    SOURCE_FILE_NAME,
                    0x647,
                    0x14,
                    NAME_STRING,
                    7,
                    0x20
                ],
                vec![
                    0x0101_5588,
                    2,
                    SOURCE_FILE_NAME,
                    0x66b,
                    0x14,
                    NAME_STRING,
                    7,
                    0x20
                ],
            ]
        );
        e.call_log = Some(vec![]);
        put_item_stream(&mut e, 0x424c_4f4b, 22, 2);
        e.call(0x004be930, &args![this]);
        assert_eq!(
            calls(&e, SAVE_LOAD_LOG),
            vec![vec![
                0x0101_5500,
                2,
                SOURCE_FILE_NAME,
                0x66b,
                0x14,
                NAME_STRING,
                7,
                0x20
            ]]
        );
    }

    #[test]
    fn after_load_runs_on_every_list_until_an_empty_head() {
        let mut e = more();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let head = list(&mut e, &[a, 0, b]);
        let this = change(&mut e, head, 0x4242);
        e.call(0x004bed00, &args![this]);
        assert_eq!(
            calls(&e, EXTRA_AFTER_LOAD_GAME),
            vec![vec![a, 0, 0, 0, 0x4242], vec![b, 0, 0, 0, 0x4242]]
        );
        // An empty head node stops the walk at once.
        let empty = list(&mut e, &[]);
        let none = change(&mut e, empty, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x004bed00, &args![none]);
        assert!(calls(&e, EXTRA_AFTER_LOAD_GAME).is_empty());
    }

    #[test]
    fn save_game_buffer_writes_the_form_the_number_and_the_lists() {
        let mut e = more();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let head = list(&mut e, &[a, 0, b]);
        let this = change(&mut e, head, 0x4242);
        e.call(0x004bed60, &args![this, 0xb0ffu32]);
        assert_eq!(
            calls(&e, SAVE_BUFFER_SAVE_FORM_ID),
            vec![vec![0xb0ff, 0x4242, 0]]
        );
        assert_eq!(
            calls(&e, SAVE_BUFFER_SAVE_DATA),
            vec![vec![0xb0ff, this.addr() + 4, 4, 0]]
        );
        assert_eq!(
            calls(&e, EXTRA_SAVE_GAME_BUFFER),
            vec![vec![a, 0xb0ff], vec![b, 0xb0ff]]
        );
        // Two lists were written; the start value comes from the buffer.
        assert_eq!(
            calls(&e, SAVE_BUFFER_END_SIZED),
            vec![vec![0xb0ff, 2, 0x77]]
        );
    }

    #[test]
    fn load_game_buffer_replaces_the_lists() {
        let mut e = more();
        let old = extra(&mut e);
        let head = list(&mut e, &[old]);
        let this = change(&mut e, head, 0);
        set(&mut e, CAST_TABLE, 0x2c, 0x5555);
        set(&mut e, SAVE_OBJECT, 0x30, 2);
        e.call(0x004bee00, &args![this, 0xb0ffu32]);
        assert_eq!(e.get(this, ItemChange::pContainerObj), Ptr::new(0x5555));
        assert_eq!(e.get(this, ItemChange::iNumber), 7);
        // The old list was deleted, two new ones loaded.
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![old, 1]]);
        let loaded = items(&e, head);
        assert_eq!(loaded.len(), 2);
        assert_eq!(
            calls(&e, EXTRA_LOAD_GAME_BUFFER),
            vec![vec![loaded[0], 0xb0ff], vec![loaded[1], 0xb0ff]]
        );
        // No lists: no node is made.
        set(&mut e, SAVE_OBJECT, 0x30, 0);
        let bare = change(&mut e, 0, 0);
        e.call(0x004bee00, &args![bare, 0xb0ffu32]);
        assert_eq!(e.get(bare, ItemChange::pExtraObjectList), Ptr::NULL);
        // A missing node is made when there are lists.
        set(&mut e, SAVE_OBJECT, 0x30, 1);
        e.call(0x004bee00, &args![bare, 0xb0ffu32]);
        let node = e.get(bare, ItemChange::pExtraObjectList).addr();
        assert_eq!(items(&e, node).len(), 1);
    }

    #[test]
    fn finish_load_runs_on_every_non_null_list() {
        let mut e = more();
        let (a, b) = (extra(&mut e), extra(&mut e));
        let head = list(&mut e, &[a, 0, b]);
        let this = change(&mut e, head, 0x4242);
        e.call(0x004bef60, &args![this, 0x99u32]);
        assert_eq!(
            calls(&e, EXTRA_AFTER_LOAD_GAME_BUFFER),
            vec![vec![a, 0x99, 0x4242], vec![b, 0x99, 0x4242]]
        );
    }

    /// An `InventoryChanges` whose list holds `changes` (`ItemChange`
    /// addresses) and whose owner is `owner`.
    fn inventory(e: &mut Engine, changes: &[u32], owner: u32) -> Ptr<InventoryChanges> {
        let this = e.new_object::<InventoryChanges>();
        let head = list(e, changes);
        e.set(this, InventoryChanges::pListofChanges, Ptr::new(head));
        e.set(this, InventoryChanges::pRef, Ptr::new(owner));
        this
    }

    fn inventory_list(e: &Engine, this: Ptr<InventoryChanges>) -> Vec<u32> {
        items(e, e.get(this, InventoryChanges::pListofChanges).addr())
    }

    /// An `ItemChange` for `form` holding `extras`, with `number` items.
    fn entry(e: &mut Engine, form: u32, extras: &[u32], number: i32) -> u32 {
        let head = list(e, extras);
        let c = change(e, head, form);
        e.set(c, ItemChange::iNumber, number);
        c.addr()
    }

    /// An extra data list with hot key `key` (-1 for none).
    fn hot_extra(e: &mut Engine, key: i8) -> u32 {
        let x = extra(e);
        set(e, x, X_HOT_KEY, key as u8 as u32);
        // A non-empty extra list (+4 is the first extra), count 1.
        set(e, x, 4, 1);
        set(e, x, X_COUNT, 1);
        x
    }

    #[test]
    fn constructor_makes_the_list_and_the_scratch_reference_once() {
        let mut e = more();
        e.register(REFR_CONSTRUCT, |_, a| returns(a[0]));
        e.register(vcall_address(0x100), |_, _| returns(0));
        let owner = object(&mut e);
        let this = e.new_object::<InventoryChanges>();
        let r = e.call(0x004befb0, &args![this, owner]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.get(this, InventoryChanges::pRef), Ptr::new(owner));
        let node = e.get(this, InventoryChanges::pListofChanges).addr();
        assert_eq!((e.mem.u32(node), e.mem.u32(node + 4)), (0, 0));
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
        assert_eq!(
            e.get(this, InventoryChanges::fpreviousContainerWeight),
            -1.0
        );
        assert!(!e.get(this, InventoryChanges::bcountdirty));
        // The scratch reference: 0x68 bytes, made temporary, kept in the global.
        let temp = e.global::<u32>(TEMP_REF_GLOBAL);
        assert_ne!(temp, 0);
        assert_eq!(calls(&e, FORM_SET_TEMPORARY), vec![vec![temp]]);
        assert!(calls(&e, OPERATOR_NEW).contains(&vec![0x68]));
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x6de]);
        // The second one reuses it.
        e.call_log = Some(vec![]);
        let other = e.new_object::<InventoryChanges>();
        e.call(0x004befb0, &args![other, Ptr::<()>::NULL]);
        assert!(calls(&e, FORM_SET_TEMPORARY).is_empty());
        assert_eq!(e.global::<u32>(TEMP_REF_GLOBAL), temp);
    }

    #[test]
    fn forgetting_the_weight_tells_the_owner_when_it_asks_for_it() {
        let mut e = more();
        // Virtual 0x100 answers with the byte after the log flag.
        e.register(vcall_address(0x100), |e, _| {
            returns(e.mem.u8(LOG_FLAG + 1) as u32)
        });
        e.register(vcall_address(0x30c), |_, _| Ret::default());
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.set(this, InventoryChanges::fcontainerweight, 5.0);
        e.call(0x004bf0e0, &args![this]);
        assert_eq!(e.get(this, InventoryChanges::fpreviousContainerWeight), 5.0);
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
        assert!(calls(&e, vcall_address(0x30c)).is_empty());
        e.mem.set_u8(LOG_FLAG + 1, 1);
        e.call(0x004bf0e0, &args![this]);
        assert_eq!(
            calls(&e, vcall_address(0x30c)),
            vec![vec![owner, (-1.0f32).to_bits()]]
        );
        // Without an owner only the weights change.
        let alone = inventory(&mut e, &[], 0);
        e.call_log = Some(vec![]);
        e.call(0x004bf0e0, &args![alone]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn destructor_body_deletes_every_change_and_the_list() {
        let mut e = more();
        e.register(vcall_address(0x4c), |_, _| Ret::default());
        let owner = object(&mut e);
        let x = extra(&mut e);
        let (c1, c2) = (entry(&mut e, 0x10, &[x], 1), entry(&mut e, 0x20, &[], 1));
        let this = inventory(&mut e, &[c1, c2], owner);
        let head = e.get(this, InventoryChanges::pListofChanges).addr();
        e.call(0x004bf150, &args![this]);
        // Each change got DeleteAllExtra (the first one's list is removed
        // and deleted) and was deleted.
        assert_eq!(calls(&e, EXTRA_REMOVE_ALL), vec![vec![x, 1]]);
        assert_eq!(
            calls(&e, ITEM_CHANGE_DELETE),
            vec![vec![c1, 1], vec![c2, 1]]
        );
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![head]]);
        assert_eq!(
            calls(&e, vcall_address(0x4c)),
            vec![vec![owner, 0x0800_0020]]
        );
        assert_eq!(calls(&e, LIST_DESTROY), vec![vec![head, 1]]);
    }

    #[test]
    fn get_inventory_changes_makes_and_stores_the_changes_once() {
        let mut e = more();
        e.register(REFR_CONSTRUCT, |_, a| returns(a[0]));
        // Virtual 0xfc and 0x100 answer with the two bytes after the log flag.
        e.register(vcall_address(0xfc), |e, _| {
            returns(e.mem.u8(LOG_FLAG + 2) as u32)
        });
        e.register(vcall_address(0x100), |e, _| {
            returns(e.mem.u8(LOG_FLAG + 3) as u32)
        });
        let reference = object(&mut e);
        // 0xfc true and 0x100 false: nothing is made.
        e.mem.set_u8(LOG_FLAG + 2, 1);
        assert_eq!(e.call(0x004bf220, &args![reference]).u32(), 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        // 0xfc false: made once, then found again.
        e.mem.set_u8(LOG_FLAG + 2, 0);
        let made = e.call(0x004bf220, &args![reference]).u32();
        assert_ne!(made, 0);
        let made_ref = e.get(Ptr::<InventoryChanges>::new(made), InventoryChanges::pRef);
        assert_eq!(made_ref, Ptr::new(reference));
        assert_eq!(
            calls(&e, EXTRA_SET_CONTAINER_CHANGES),
            vec![vec![reference + 0x40, made]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004bf220, &args![reference]).u32(), made);
        assert!(calls(&e, EXTRA_SET_CONTAINER_CHANGES).is_empty());
        // 0xfc true and 0x100 true also works.
        e.mem.set_u8(LOG_FLAG + 2, 1);
        e.mem.set_u8(LOG_FLAG + 3, 1);
        assert_eq!(e.call(0x004bf220, &args![reference]).u32(), made);
    }

    #[test]
    fn remove_hot_key_item_unhooks_the_keys_and_drops_emptied_lists() {
        let mut e = more();
        let form_a = form(&mut e, 0x28, 0, 0);
        // Key 3 only: its list has no extras left, so it leaves the entry.
        let (x1, x2) = (hot_extra(&mut e, 3), hot_extra(&mut e, 5));
        set(&mut e, x1, 4, 0);
        let entry_a = entry(&mut e, form_a, &[x1, x2], 2);
        let this = inventory(&mut e, &[entry_a], 0);
        let item = change(&mut e, 0, form_a);
        e.call(0x004bf330, &args![this, item, 3i32]);
        assert_eq!(calls(&e, EXTRA_REMOVE_HOT_KEY), vec![vec![x1]]);
        assert_eq!(items(&e, e.mem.u32(entry_a)), vec![x2]);
        // -1 removes every key; lists with extras stay.
        let (y1, y2, y3) = (
            hot_extra(&mut e, 3),
            hot_extra(&mut e, 5),
            hot_extra(&mut e, -1),
        );
        let entry_b = entry(&mut e, form_a, &[y1, y2, y3], 2);
        let this = inventory(&mut e, &[entry_b], 0);
        e.call_log = Some(vec![]);
        e.call(0x004bf330, &args![this, item, -1i32]);
        assert_eq!(calls(&e, EXTRA_REMOVE_HOT_KEY), vec![vec![y1], vec![y2]]);
        assert_eq!(items(&e, e.mem.u32(entry_b)), vec![y1, y2, y3]);
        // A list of a count above one holding one kind of extra leaves too.
        let z = hot_extra(&mut e, 2);
        set(&mut e, z, X_COUNT, 3);
        set(&mut e, z, X_ITEMS_IN_LIST, 1);
        let entry_c = entry(&mut e, form_a, &[z], 2);
        let this = inventory(&mut e, &[entry_c], 0);
        e.call(0x004bf330, &args![this, item, 2i32]);
        // The entry has no list left but still counts items: it stays.
        assert_eq!(e.mem.u32(entry_c), 0);
        assert_eq!(inventory_list(&e, this), vec![entry_c]);
    }

    #[test]
    fn remove_hot_key_item_deletes_an_entry_left_empty_with_no_items() {
        let mut e = more();
        let form_a = form(&mut e, 0x28, 0, 0);
        let x1 = hot_extra(&mut e, 3);
        set(&mut e, x1, 4, 0);
        let entry_a = entry(&mut e, form_a, &[x1], 0);
        let other = entry(&mut e, 0x4444, &[], 1);
        let this = inventory(&mut e, &[other, entry_a], 0);
        let item = change(&mut e, 0, form_a);
        let list_head = e.mem.u32(entry_a);
        e.call(0x004bf330, &args![this, item, 3i32]);
        assert_eq!(calls(&e, LIST_DESTROY), vec![vec![list_head, 1]]);
        assert_eq!(inventory_list(&e, this), vec![other]);
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![entry_a, 1]]);
        // No entry for the form: nothing happens.
        e.call_log = Some(vec![]);
        let stranger = form(&mut e, 0x28, 0, 0);
        let none = change(&mut e, 0, stranger);
        e.call(0x004bf330, &args![this, none, -1i32]);
        assert!(calls(&e, EXTRA_REMOVE_HOT_KEY).is_empty());
        assert!(calls(&e, ITEM_CHANGE_DELETE).is_empty());
    }

    /// Doubles for the fast inventory iteration: it hands out `items` and
    /// then 0; the iterator is 0x1111.
    fn fast_iteration(e: &mut Engine, items: Vec<u32>) {
        e.register(START_FAST_ITERATION, |_, _| returns(0x1111));
        let mut remaining = items.into_iter();
        e.register_double(GET_NEXT_FAST_ITEM, move |_, a| {
            assert_eq!(a[1], 0x1111);
            returns(remaining.next().unwrap_or(0))
        });
    }

    #[test]
    fn hot_key_lookup_returns_the_first_item_with_the_key() {
        let mut e = more();
        let (xa, xb) = (hot_extra(&mut e, 1), hot_extra(&mut e, 2));
        let (a, b) = (entry(&mut e, 0x10, &[xa], 1), entry(&mut e, 0x20, &[xb], 1));
        fast_iteration(&mut e, vec![a, b]);
        let this = inventory(&mut e, &[], 0);
        assert_eq!(e.call(0x004bf4b0, &args![this, 2i32]).u32(), b);
        // The item without the key and the iterator are freed.
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![a, 1]]);
        assert_eq!(calls(&e, FAST_ITERATOR_DESTRUCT), vec![vec![0x1111]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1111]]);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x76a]);
        assert_eq!(calls(&e, SCOPE_GUARD_CLOSE).len(), 1);
        // No item has key 7: everything is freed and 0 returned.
        fast_iteration(&mut e, vec![a, b]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004bf4b0, &args![this, 7i32]).u32(), 0);
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![a, 1], vec![b, 1]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1111]]);
    }

    #[test]
    fn iterator_destructor_frees_only_when_asked() {
        let mut e = more();
        assert_eq!(e.call(0x004bf630, &args![0x1111u32, 0u32]).u32(), 0x1111);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls(&e, FAST_ITERATOR_DESTRUCT), vec![vec![0x1111]]);
        assert_eq!(e.call(0x004bf630, &args![0x1111u32, 1u32]).u32(), 0x1111);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1111]]);
    }

    #[test]
    fn hot_key_array_holds_the_last_item_of_each_key() {
        let mut e = more();
        let (xa, xb, xc, xd) = (
            hot_extra(&mut e, 1),
            hot_extra(&mut e, 3),
            hot_extra(&mut e, 1),
            hot_extra(&mut e, 7),
        );
        let items_made: Vec<u32> = [xa, xb, xc, xd]
            .iter()
            .map(|x| entry(&mut e, 0x10, &[*x], 1))
            .collect();
        let (a, b, c, d) = (items_made[0], items_made[1], items_made[2], items_made[3]);
        fast_iteration(&mut e, items_made);
        let this = inventory(&mut e, &[], 0);
        let array = e.call(0x004bf660, &args![this, 1i32, 3i32]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![12]]);
        assert_eq!(
            (e.mem.u32(array), e.mem.u32(array + 4), e.mem.u32(array + 8)),
            (c, 0, b)
        );
        // `a` was replaced by `c`, `d` has no slot: both freed.
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![a, 1], vec![d, 1]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1111]]);
        // An empty range gives no array.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004bf660, &args![this, 3i32, 1i32]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn set_hot_key_item_ignores_bad_arguments() {
        let mut e = more();
        let this = inventory(&mut e, &[], 0);
        let item = change(&mut e, 0, 0x10);
        e.call(0x004bf800, &args![this, item, 0u32, 8i32]);
        e.call(0x004bf800, &args![this, item, 0u32, -1i32]);
        e.call(
            0x004bf800,
            &args![this, Ptr::<ItemChange>::NULL, 0u32, 3i32],
        );
        // Only the scope guard calls were made.
        for (address, _) in e.call_log.as_ref().unwrap() {
            assert!(
                [0x004bf800, SCOPE_GUARD_OPEN, SCOPE_GUARD_CLOSE].contains(address),
                "{address:08x}"
            );
        }
        assert_eq!(calls(&e, SCOPE_GUARD_OPEN).len(), 3);
    }

    #[test]
    fn set_hot_key_item_uses_the_default_list_or_the_given_one() {
        let mut e = more();
        e.mem.set_u32(PLAYER, BIG_VTABLE);
        let form_a = form(&mut e, 0x28, 0, 0);
        let default = hot_extra(&mut e, -1);
        set(&mut e, default, X_DEFAULT, 1);
        let given = hot_extra(&mut e, -1);
        let entry_a = entry(&mut e, form_a, &[default, given], 2);
        let this = inventory(&mut e, &[entry_a], 0);
        let item = change(&mut e, 0, form_a);
        // The player had a hot key table entry for key 4: it is cleared.
        let table_entry = PLAYER + HOT_KEY_TABLE + 0x100 + 16;
        e.mem.set_u32(table_entry, 9);
        e.call(0x004bf800, &args![this, item, 0u32, 4i32]);
        assert_eq!(e.mem.u32(table_entry), 0);
        assert_eq!(calls(&e, EXTRA_SET_HOT_KEY), vec![vec![default, 4]]);
        // The given list wins when it is in the entry, even after the default.
        e.call_log = Some(vec![]);
        e.call(0x004bf800, &args![this, item, given, 5i32]);
        assert_eq!(calls(&e, EXTRA_SET_HOT_KEY), vec![vec![given, 5]]);
    }

    #[test]
    fn set_hot_key_item_makes_a_list_for_the_items_outside_lists() {
        let mut e = more();
        e.mem.set_u32(PLAYER, BIG_VTABLE);
        let form_a = form(&mut e, 0x28, 0, 0);
        // One list of two (not default, not worn) and 5 items: 3 are left.
        let counted = hot_extra(&mut e, -1);
        set(&mut e, counted, X_COUNT, 2);
        let entry_a = entry(&mut e, form_a, &[counted], 5);
        let this = inventory(&mut e, &[entry_a], 0);
        let item = change(&mut e, 0, form_a);
        e.set(item, ItemChange::iNumber, 5);
        e.call(0x004bf800, &args![this, item, 0u32, 6i32]);
        let lists = items(&e, e.mem.u32(entry_a));
        assert_eq!(lists.len(), 2);
        assert_eq!(e.mem.u32(lists[1] + X_HOT_KEY), 6);
        assert_eq!(e.mem.u32(lists[1] + X_COUNT), 3);
        // A worn one among the lists takes one more.
        let worn = hot_extra(&mut e, -1);
        set(&mut e, worn, X_WORN, 1);
        set(&mut e, worn, X_COUNT, 1);
        let entry_b = entry(&mut e, form_a, &[worn], 5);
        let this = inventory(&mut e, &[entry_b], 0);
        e.call(0x004bf800, &args![this, item, 0u32, 6i32]);
        let lists = items(&e, e.mem.u32(entry_b));
        assert_eq!(e.mem.u32(lists[1] + X_COUNT), 4);
    }

    #[test]
    fn set_hot_key_item_adds_a_change_when_the_form_has_none() {
        let mut e = more();
        e.mem.set_u32(PLAYER, BIG_VTABLE);
        let form_a = form(&mut e, 0x28, 0, 0);
        let this = inventory(&mut e, &[], 0);
        let item = change(&mut e, 0, form_a);
        e.set(item, ItemChange::iNumber, 5);
        e.call(0x004bf800, &args![this, item, 0u32, 2i32]);
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(form_a));
        assert_eq!(e.get(made, ItemChange::iNumber), 5);
        let lists = items(&e, e.get(made, ItemChange::pExtraObjectList).addr());
        assert_eq!(lists.len(), 1);
        assert_eq!(e.mem.u32(lists[0] + X_HOT_KEY), 2);
    }

    #[test]
    fn hot_key_table_of_the_player() {
        let mut e = more();
        let table_entry = PLAYER + HOT_KEY_TABLE + 0x100 + 4 * 8;
        e.mem.set_u32(table_entry, 0x77);
        assert_eq!(e.call(0x004bfb30, &args![PLAYER, 8i32]).u32(), 0x77);
        assert_eq!(e.call(0x004bfb30, &args![PLAYER, 9i32]).u32(), 0);
        assert_eq!(e.call(0x004bfb30, &args![PLAYER, -1i32]).u32(), 0);
        e.call(0x004bfb70, &args![PLAYER, 8i32, 0x99u32]);
        assert_eq!(e.mem.u32(table_entry), 0x99);
        assert_eq!(
            calls(&e, HOT_KEY_TABLE_ENTRY),
            vec![vec![PLAYER + HOT_KEY_TABLE, 8]]
        );
    }

    #[test]
    fn object_in_list_matches_the_form_and_optionally_a_reference() {
        let mut e = more();
        let reference = e.mem.alloc(0x20);
        set(&mut e, reference, 0xc, 0xfeed);
        let with_ref = extra(&mut e);
        set(&mut e, with_ref, X_REFERENCE, reference);
        let (a, b) = (
            entry(&mut e, 0x10, &[], 1),
            entry(&mut e, 0x20, &[with_ref], 1),
        );
        let this = inventory(&mut e, &[a, b], 0);
        assert_eq!(
            e.call(0x004bfba0, &args![this, 0x20u32, 1u32, 0u32]).u32(),
            b
        );
        assert_eq!(
            e.call(0x004bfba0, &args![this, 0x10u32, 1u32, 0u32]).u32(),
            a
        );
        assert_eq!(
            e.call(0x004bfba0, &args![this, 0x30u32, 1u32, 0u32]).u32(),
            0
        );
        // With a reference id: the entry needs a list with that reference.
        assert_eq!(
            e.call(0x004bfba0, &args![this, 0x20u32, 1u32, 0xfeedu32])
                .u32(),
            b
        );
        assert_eq!(
            e.call(0x004bfba0, &args![this, 0x20u32, 1u32, 0xbeefu32])
                .u32(),
            0
        );
        assert_eq!(
            e.call(0x004bfba0, &args![this, 0x10u32, 1u32, 0xfeedu32])
                .u32(),
            0
        );
    }

    #[test]
    fn worn_armor_with_the_flag_is_found() {
        let mut e = more();
        let armor = form(&mut e, 0x18, 0, 0);
        let weapon = form(&mut e, 0x28, 0, 0);
        let worn = extra(&mut e);
        set(&mut e, worn, X_WORN, 1);
        let (armor_entry, weapon_entry) = (
            entry(&mut e, armor, &[worn], 1),
            entry(&mut e, weapon, &[worn], 1),
        );
        // The flag byte is cleared: not found.
        let this = inventory(&mut e, &[armor_entry], 0);
        assert!(!e.call(0x004bfc80, &args![this]).bool());
        e.mem.set_u8(armor + ARMOR_FLAGS, 1);
        assert!(e.call(0x004bfc80, &args![this]).bool());
        // A weapon is not looked at; the worn armor after it is.
        let both = inventory(&mut e, &[weapon_entry, armor_entry], 0);
        assert!(e.call(0x004bfc80, &args![both]).bool());
        let only_weapon = inventory(&mut e, &[weapon_entry], 0);
        assert!(!e.call(0x004bfc80, &args![only_weapon]).bool());
        // An armor with a list that is not worn does not count.
        let plain = extra(&mut e);
        let unworn = entry(&mut e, armor, &[plain], 1);
        let third = inventory(&mut e, &[unworn], 0);
        assert!(!e.call(0x004bfc80, &args![third]).bool());
        // The flag helper.
        assert!(e.call(0x004bfd80, &args![armor]).bool());
        e.mem.set_u8(armor + ARMOR_FLAGS, 0xfe);
        assert!(!e.call(0x004bfd80, &args![armor]).bool());
    }

    #[test]
    fn wearing_object_finds_the_worn_list_or_wears_an_unwearable_one() {
        let mut e = more();
        let form_a = form(&mut e, 0x18, 0, 0);
        let (plain, worn, cannot) = (extra(&mut e), extra(&mut e), extra(&mut e));
        set(&mut e, worn, X_WORN, 1);
        set(&mut e, cannot, X_CAN_NOT_WEAR, 1);
        let with_worn = entry(&mut e, form_a, &[plain, worn], 2);
        let this = inventory(&mut e, &[with_worn], 0);
        assert_eq!(e.call(0x004bfda0, &args![this, form_a, 0u32]).u32(), worn);
        // A can-not-wear list is only taken with the flag, and then worn.
        let with_cannot = entry(&mut e, form_a, &[plain, cannot], 2);
        let other = inventory(&mut e, &[with_cannot], 0);
        assert_eq!(e.call(0x004bfda0, &args![other, form_a, 0u32]).u32(), 0);
        assert!(calls(&e, EXTRA_SET_WORN).is_empty());
        assert_eq!(
            e.call(0x004bfda0, &args![other, form_a, 1u32]).u32(),
            cannot
        );
        assert_eq!(calls(&e, EXTRA_SET_WORN), vec![vec![cannot, 1, 0]]);
        // No entry for the form.
        assert_eq!(e.call(0x004bfda0, &args![other, 0x99u32, 1u32]).u32(), 0);
    }

    #[test]
    fn remove_all_objects_worn_removes_each_worn_change_until_it_stays() {
        let mut e = more();
        let form_a = form(&mut e, 0x18, 0, 0);
        let (plain, worn) = (extra(&mut e), extra(&mut e));
        set(&mut e, worn, X_WORN, 1);
        set(&mut e, worn, X_COUNT, 2);
        let (unworn_entry, worn_entry) = (
            entry(&mut e, 0x4444, &[plain], 1),
            entry(&mut e, form_a, &[plain, worn], 3),
        );
        let this = inventory(&mut e, &[unworn_entry, worn_entry], 0);
        let actor = object(&mut e);
        // The removal reports a removal the first time only.
        let mut removals = 0;
        e.register_double(INVENTORY_REMOVE_ITEM, move |e, a| {
            removals += 1;
            e.mem.set_u8(a[1], (removals == 1) as u8);
            Ret::default()
        });
        e.call(0x004bfe50, &args![this, 0u32, actor]);
        let removes = calls(&e, INVENTORY_REMOVE_ITEM);
        assert_eq!(removes.len(), 2);
        let args_of = |call: &Vec<u32>| {
            let mut words = call.clone();
            words[1] = 0;
            words
        };
        assert_eq!(
            args_of(&removes[0]),
            vec![this.addr(), 0, form_a, 2, actor, worn, 0, 1, worn_entry, 0]
        );
        assert_eq!(args_of(&removes[1]), args_of(&removes[0]));
        assert_eq!(
            calls(&e, REFERENCE_FORM_NOTIFY)[0],
            vec![actor + 0x94, form_a, 1]
        );
        // Nothing worn and no reference: nothing is removed or passed on.
        e.call_log = Some(vec![]);
        let plain_inventory = inventory(&mut e, &[unworn_entry], 0);
        e.call(0x004bfe50, &args![plain_inventory, 0u32, 0u32]);
        assert!(calls(&e, INVENTORY_REMOVE_ITEM).is_empty());
        assert!(calls(&e, REFERENCE_FORM_NOTIFY).is_empty());
    }

    #[test]
    fn container_of_the_owner() {
        let mut e = more();
        e.register(REFR_HAS_CONTAINER, |_, a| returns(a[0] + 0x10));
        let none = inventory(&mut e, &[], 0);
        assert_eq!(e.call(0x004bffb0, &args![none]).u32(), 0);
        let owned = inventory(&mut e, &[], 0x5000);
        assert_eq!(e.call(0x004bffb0, &args![owned]).u32(), 0x5010);
    }

    #[test]
    fn small_form_predicates() {
        let mut e = more();
        let o = object(&mut e);
        e.mem.set_u8(o + 8, 0x80);
        assert!(e.call(0x004c0bd0, &args![o]).bool());
        e.mem.set_u8(o + 8, 0x7f);
        assert!(!e.call(0x004c0bd0, &args![o]).bool());
        // Weapon types: 3..=13 and 3..=9, and their difference.
        for (kind, wide, narrow, difference) in [
            (2i8, false, false, false),
            (3, true, true, false),
            (9, true, true, false),
            (10, true, false, true),
            (13, true, false, true),
            (14, false, false, false),
            (-3, false, false, false),
        ] {
            e.mem.set_i8(o + WEAPON_TYPE, kind);
            assert_eq!(e.call(0x004c0c30, &args![o]).bool(), wide, "{kind}");
            assert_eq!(e.call(0x004c0c60, &args![o]).bool(), narrow, "{kind}");
            assert_eq!(e.call(0x004c0bf0, &args![o]).bool(), difference, "{kind}");
        }
    }

    /// An engine with the doubles `004bffe0` needs on top of `more()`.
    fn equip_engine() -> Engine {
        let mut e = more();
        e.register(vcall_address(0x100), |_, _| returns(0));
        e.register(vcall_address(0x468), |_, _| Ret::default());
        e.register(vcall_address(0x6e4), |_, _| Ret::default());
        e.register(vcall_address(0x48), |_, _| Ret::default());
        e.register(REFR_HAS_CONTAINER, |_, a| returns(a[0] + 0x10));
        e.register(CONTAINER_COUNT, |e, _| returns(e.mem.u32(LOOKUP_CELL + 8)));
        e.register(REFR_CONSTRUCT, |_, a| returns(a[0]));
        e.register(GET_FORM_AS_BIPED_MODEL, |_, _| returns(0));
        e.register(BIPED_IS_POWER_ARMOR, |e, a| {
            returns((e.mem.u8(a[0] + 8) & 0x20 != 0) as u32)
        });
        e.register(BIPED_FLAG_FOUR, |e, a| {
            returns((e.mem.u8(a[0] + 8) & 4 != 0) as u32)
        });
        e.register(ACTOR_PROCESS, |e, a| returns(e.mem.u32(a[0] + 0x68)));
        e.register(ACTOR_GET_ANIMATION, |e, a| returns(e.mem.u32(a[0] + 0x6c)));
        e.register(ACTOR_LIGHTING_ARGUMENT, |_, _| returns(0x9999));
        e.register(FORM_WEIGHT_FIELD, |_, _| returns_float(2.5));
        e.register(PREDICATE_008C7AA0, |e, _| {
            returns(e.mem.u8(LOG_FLAG + 4) as u32)
        });
        e.register(THREAD_VALUE_0047B200, |e, _| {
            returns(e.mem.u32(LOG_FLAG + 8))
        });
        e.register(TASK_QUEUE_GETTER, |_, _| returns(0x7777));
        e.register(vcall_address(0x1d0), |_, _| returns(0x55));
        e.mem.set_u32(PLAYER, BIG_VTABLE);
        e
    }

    /// An inventory (no owner), an armor of type `form_type`, an actor with a
    /// process (both big-vtable objects) and an animation object.
    fn equip_setup(e: &mut Engine, form_type: u8) -> (Ptr<InventoryChanges>, u32, u32, u32) {
        let this = inventory(e, &[], 0);
        let item_form = form(e, form_type, 0, 0);
        let actor = object(e);
        let process = object(e);
        let animation = e.mem.alloc(0x200);
        e.mem.set_u32(actor + 0x68, process);
        e.mem.set_u32(actor + 0x6c, animation);
        (this, item_form, actor, process)
    }

    #[test]
    fn equip_makes_an_entry_and_a_worn_list_for_a_new_form() {
        let mut e = equip_engine();
        let (this, armor, actor, _) = equip_setup(&mut e, 0x18);
        e.call(
            0x004bffe0,
            &args![this, armor, 3i32, actor, 0u32, 1u32, 0u32],
        );
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(armor));
        assert_eq!(e.get(made, ItemChange::iNumber), 0);
        let lists = items(&e, e.get(made, ItemChange::pExtraObjectList).addr());
        assert_eq!(lists.len(), 1);
        // Worn with the given flag, the count 3, the can-not-wear flag given.
        assert_eq!(e.mem.u32(lists[0] + X_WORN), 1);
        assert_eq!(e.mem.u32(lists[0] + X_WORN_FLAG), 1);
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 3);
        assert_eq!(calls(&e, EXTRA_SET_CAN_NOT_WEAR), vec![vec![lists[0], 0]]);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x960]);
        assert_eq!(calls(&e, SCOPE_GUARD_CLOSE).len(), 1);
        // A count of 1 leaves the count of the new list alone.
        let (other, armor_two, actor_two, _) = equip_setup(&mut e, 0x18);
        e.call(
            0x004bffe0,
            &args![other, armor_two, 1i32, actor_two, 0u32, 0u32, 1u32],
        );
        let made = Ptr::<ItemChange>::new(inventory_list(&e, other)[0]);
        let lists = items(&e, e.get(made, ItemChange::pExtraObjectList).addr());
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 0);
        assert_eq!(e.mem.u32(lists[0] + X_CAN_NOT_WEAR), 1);
    }

    #[test]
    fn equip_splits_one_item_off_a_stack() {
        let mut e = equip_engine();
        let (this, armor, actor, _) = equip_setup(&mut e, 0x18);
        // A default list of two items with a hot key, in an entry of 2.
        let stack = hot_extra(&mut e, 3);
        set(&mut e, stack, X_DEFAULT, 1);
        set(&mut e, stack, X_COUNT, 2);
        let entry_a = entry(&mut e, armor, &[stack], 2);
        let head = e.get(this, InventoryChanges::pListofChanges).addr();
        list_call_with_item(&mut e, LIST_ADD, head, entry_a);
        e.call(
            0x004bffe0,
            &args![this, armor, 1i32, actor, 0u32, 1u32, 0u32],
        );
        let lists = items(&e, e.mem.u32(entry_a));
        assert_eq!(lists.len(), 2);
        let (rest, worn) = (lists[0], lists[1]);
        assert_eq!(rest, stack);
        // One left in the stack, without its hot key; the copy is worn.
        assert_eq!(e.mem.u32(rest + X_COUNT), 1);
        assert_eq!(e.mem.u32(rest + X_HOT_KEY), 0xff);
        assert_eq!(e.mem.u32(worn + 0x14), stack);
        assert_eq!(e.mem.u32(worn + X_WORN), 1);
        assert_eq!(e.mem.u32(worn + X_COUNT), 1);
        // Ammunition is not split.
        e.register(vcall_address(0x14c), |_, _| returns(0));
        e.register(vcall_address(0x168), |_, _| returns(1));
        e.register(vcall_address(0x474), |_, _| returns(0));
        let (this, ammo, actor, _) = equip_setup(&mut e, 0x29);
        let stack = hot_extra(&mut e, -1);
        set(&mut e, stack, X_COUNT, 5);
        set(&mut e, stack, X_DEFAULT, 1);
        let entry_b = entry(&mut e, ammo, &[stack], 5);
        let head = e.get(this, InventoryChanges::pListofChanges).addr();
        list_call_with_item(&mut e, LIST_ADD, head, entry_b);
        e.call(
            0x004bffe0,
            &args![this, ammo, 5i32, actor, 0u32, 1u32, 0u32],
        );
        assert_eq!(items(&e, e.mem.u32(entry_b)), vec![stack]);
        assert_eq!(e.mem.u32(stack + X_WORN), 1);
        assert_eq!(e.mem.u32(stack + X_COUNT), 5);
    }

    #[test]
    fn equip_wears_the_given_list_and_does_nothing_for_a_worn_entry() {
        let mut e = equip_engine();
        let (this, armor, actor, _) = equip_setup(&mut e, 0x18);
        let given = hot_extra(&mut e, -1);
        let entry_a = entry(&mut e, armor, &[given], 1);
        let head = e.get(this, InventoryChanges::pListofChanges).addr();
        list_call_with_item(&mut e, LIST_ADD, head, entry_a);
        e.call(
            0x004bffe0,
            &args![this, armor, 1i32, actor, given, 7u32, 0u32],
        );
        // Worn twice over (the stack is whole, then the requested count).
        assert_eq!(e.mem.u32(given + X_WORN), 1);
        assert_eq!(e.mem.u32(given + X_WORN_FLAG), 7);
        assert_eq!(items(&e, e.mem.u32(entry_a)), vec![given]);
        assert_eq!(calls(&e, EXTRA_SET_WORN).len(), 2);
        // With a worn list in the entry nothing more is done.
        e.call_log = Some(vec![]);
        e.call(
            0x004bffe0,
            &args![this, armor, 1i32, actor, given, 7u32, 0u32],
        );
        assert!(calls(&e, EXTRA_SET_WORN).is_empty());
        assert!(calls(&e, ACTOR_INIT_LIGHTING).is_empty());
        assert_eq!(calls(&e, SCOPE_GUARD_CLOSE).len(), 1);
        // The walk wears the list before the can-not-wear check: nothing is
        // added and the flag is set on that list.
        let (this, armor, actor, _) = equip_setup(&mut e, 0x18);
        let fixed = hot_extra(&mut e, -1);
        set(&mut e, fixed, X_CAN_NOT_WEAR, 1);
        let entry_b = entry(&mut e, armor, &[fixed], 1);
        let head = e.get(this, InventoryChanges::pListofChanges).addr();
        list_call_with_item(&mut e, LIST_ADD, head, entry_b);
        e.call(
            0x004bffe0,
            &args![this, armor, 1i32, actor, fixed, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(fixed + X_WORN), 1);
        assert_eq!(items(&e, e.mem.u32(entry_b)), vec![fixed]);
        assert_eq!(calls(&e, EXTRA_SET_CAN_NOT_WEAR), vec![vec![fixed, 0]]);
    }

    #[test]
    fn equip_tells_the_owner_and_counts_the_container() {
        let mut e = equip_engine();
        let (_, armor, actor, _) = equip_setup(&mut e, 0x18);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.mem.set_u32(LOOKUP_CELL + 8, 4);
        e.call(
            0x004bffe0,
            &args![this, armor, 1i32, actor, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, vcall_address(0x48)), vec![vec![owner, 0x20]]);
        assert_eq!(calls(&e, CONTAINER_COUNT), vec![vec![owner + 0x10, armor]]);
        assert_eq!(calls(&e, GET_FORM_AS_BIPED_MODEL)[0], vec![armor]);
        // The weights were forgotten first.
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
    }

    #[test]
    fn equip_announces_armor_to_the_process() {
        let mut e = equip_engine();
        e.register(vcall_address(0x6e4), |_, _| Ret::default());
        e.register(vcall_address(0x468), |_, _| Ret::default());
        let (this, armor, actor, process) = equip_setup(&mut e, 0x18);
        // Power armor: the acquire object is told, then the biped slot set.
        e.mem.set_u8(armor + ARMOR_BIPED_MODEL + 8, 0x20);
        e.call(
            0x004bffe0,
            &args![this, armor, 1i32, actor, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, vcall_address(0x6e4)), vec![vec![process, actor]]);
        assert_eq!(calls(&e, vcall_address(0x468)), vec![vec![process, 1]]);
        assert_eq!(calls(&e, ACTOR_INIT_LIGHTING), vec![vec![actor, 0x9999]]);
        // Plain armor: only the biped slot.
        e.mem.set_u8(armor + ARMOR_BIPED_MODEL + 8, 0);
        e.call_log = Some(vec![]);
        let (other, armor_two, actor_two, process_two) = equip_setup(&mut e, 0x18);
        e.call(
            0x004bffe0,
            &args![other, armor_two, 1i32, actor_two, 0u32, 0u32, 0u32],
        );
        assert!(calls(&e, vcall_address(0x6e4)).is_empty());
        assert_eq!(calls(&e, vcall_address(0x468)), vec![vec![process_two, 1]]);
        // The flag bits 0x80 and 4 also count as power armor kinds.
        for bits in [0x80u8, 0x04] {
            e.call_log = Some(vec![]);
            let (third, armor_three, actor_three, _) = equip_setup(&mut e, 0x18);
            e.mem.set_u8(armor_three + ARMOR_BIPED_MODEL + 8, bits);
            e.call(
                0x004bffe0,
                &args![third, armor_three, 1i32, actor_three, 0u32, 0u32, 0u32],
            );
            assert_eq!(calls(&e, vcall_address(0x6e4)).len(), 1, "{bits:02x}");
        }
        // Type 0x1a: only the biped slot, no acquire object.
        e.call_log = Some(vec![]);
        let (fourth, helmet, actor_four, process_four) = equip_setup(&mut e, 0x1a);
        e.call(
            0x004bffe0,
            &args![fourth, helmet, 1i32, actor_four, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, vcall_address(0x468)), vec![vec![process_four, 1]]);
        // An actor without a process is left alone.
        e.call_log = Some(vec![]);
        let (fifth, armor_five, actor_five, _) = equip_setup(&mut e, 0x18);
        e.mem.set_u32(actor_five + 0x68, 0);
        e.call(
            0x004bffe0,
            &args![fifth, armor_five, 1i32, actor_five, 0u32, 0u32, 0u32],
        );
        assert!(calls(&e, ACTOR_INIT_LIGHTING).is_empty());
    }

    #[test]
    fn equip_a_weapon_keeps_a_new_item_in_the_process() {
        let mut e = equip_engine();
        e.register(vcall_address(0x148), |_, _| returns(0));
        e.register(vcall_address(0x160), |_, _| returns(1));
        e.register(vcall_address(0x474), |_, _| returns(0));
        let (this, weapon, actor, process) = equip_setup(&mut e, 0x28);
        e.call(
            0x004bffe0,
            &args![this, weapon, 2i32, actor, 0u32, 1u32, 0u32],
        );
        let accepted = calls(&e, vcall_address(0x160));
        assert_eq!(accepted.len(), 1);
        let kept = Ptr::<ItemChange>::new(accepted[0][1]);
        assert_eq!(&accepted[0][..1], &[process]);
        assert_eq!(accepted[0][2], 0x55);
        assert_eq!(e.get(kept, ItemChange::pContainerObj), Ptr::new(weapon));
        assert_eq!(e.get(kept, ItemChange::iNumber), 2);
        let worn_lists = items(&e, e.get(kept, ItemChange::pExtraObjectList).addr());
        assert_eq!(worn_lists.len(), 1);
        assert_eq!(e.mem.u32(worn_lists[0] + X_WORN), 1);
        // The entry point 0x2b is run with the weight and the animation told.
        let entry_point = &calls(&e, HANDLE_ENTRY_POINT)[0];
        assert_eq!(&entry_point[..4], &[0x2b, actor, weapon, entry_point[3]]);
        assert_eq!(
            calls(&e, ANIMATION_STORE_FLOAT),
            vec![vec![e.mem.u32(actor + 0x6c), 2.5f32.to_bits()]]
        );
        // The model is updated (not the player, not the task queue).
        assert_eq!(calls(&e, REFR_UPDATE_WEAPON), vec![vec![actor, weapon]]);
        assert!(calls(&e, TASK_QUEUE_ATTACH_WEAPON).is_empty());
        // The special thread asks for the task queue unless it is the actor.
        e.mem.set_u8(LOG_FLAG + 4, 1);
        e.mem.set_u32(LOG_FLAG + 8, 0x1);
        let (other, weapon_two, actor_two, _) = equip_setup(&mut e, 0x28);
        e.call_log = Some(vec![]);
        e.call(
            0x004bffe0,
            &args![other, weapon_two, 1i32, actor_two, 0u32, 1u32, 0u32],
        );
        assert_eq!(
            calls(&e, TASK_QUEUE_ATTACH_WEAPON),
            vec![vec![0x7777, actor_two, weapon_two]]
        );
        assert!(calls(&e, REFR_UPDATE_WEAPON).is_empty());
        e.mem.set_u32(LOG_FLAG + 8, actor_two);
        e.call_log = Some(vec![]);
        let (third, weapon_three, _, _) = equip_setup(&mut e, 0x28);
        e.call(
            0x004bffe0,
            &args![third, weapon_three, 1i32, actor_two, 0u32, 1u32, 0u32],
        );
        assert_eq!(
            calls(&e, REFR_UPDATE_WEAPON),
            vec![vec![actor_two, weapon_three]]
        );
    }

    #[test]
    fn equip_a_weapon_reuses_the_kept_item_or_drops_a_refused_one() {
        let mut e = equip_engine();
        e.register(vcall_address(0x474), |_, _| returns(0));
        // The process already keeps an item: its lists are replaced.
        let kept = e.mem.alloc(0x20);
        let kept_list = list(&mut e, &[0x1234]);
        e.mem.set_u32(kept, kept_list);
        e.mem.set_u32(kept + 8, 0x4321);
        e.mem.set_u32(LOOKUP_CELL + 0x20, kept);
        e.register(vcall_address(0x148), |e, _| {
            returns(e.mem.u32(LOOKUP_CELL + 0x20))
        });
        let (this, weapon, actor, _) = equip_setup(&mut e, 0x28);
        e.call(
            0x004bffe0,
            &args![this, weapon, 1i32, actor, 0u32, 1u32, 0u32],
        );
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![kept_list]]);
        assert_eq!(e.mem.u32(kept + 8), weapon);
        assert_eq!(calls(&e, ACTOR_RELOAD_TARGETS), vec![vec![actor, 0]]);
        assert_eq!(calls(&e, REFR_UPDATE_WEAPON), vec![vec![actor, weapon]]);
        assert!(calls(&e, HANDLE_ENTRY_POINT).is_empty());
        // A refused new item is deleted and nothing is attached.
        e.register(vcall_address(0x148), |_, _| returns(0));
        e.register(vcall_address(0x160), |_, _| returns(0));
        let (other, weapon_two, actor_two, _) = equip_setup(&mut e, 0x28);
        e.call_log = Some(vec![]);
        e.call(
            0x004bffe0,
            &args![other, weapon_two, 1i32, actor_two, 0u32, 1u32, 0u32],
        );
        let refused = calls(&e, vcall_address(0x160))[0][1];
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![refused, 1]]);
        assert!(calls(&e, REFR_UPDATE_WEAPON).is_empty());
        assert!(calls(&e, HANDLE_ENTRY_POINT).is_empty());
    }

    #[test]
    fn equip_a_weapon_on_the_player_or_an_actor_that_asks_not_to_attach() {
        let mut e = equip_engine();
        e.register(vcall_address(0x148), |_, _| returns(0));
        e.register(vcall_address(0x160), |_, _| returns(1));
        // The player: its weapon form is stored, the virtual 0x474 not asked.
        let process = object(&mut e);
        e.mem.set_u32(PLAYER + 0x68, process);
        let animation = e.mem.alloc(0x200);
        e.mem.set_u32(PLAYER + 0x6c, animation);
        let this = inventory(&mut e, &[], 0);
        let weapon = form(&mut e, 0x28, 0, 0);
        e.call(
            0x004bffe0,
            &args![this, weapon, 1i32, PLAYER, 0u32, 1u32, 0u32],
        );
        assert_eq!(calls(&e, PLAYER_STORE_FORM), vec![vec![PLAYER, weapon]]);
        assert!(calls(&e, REFR_UPDATE_WEAPON).is_empty());
        // Another actor whose virtual 0x474 says yes gets nothing.
        e.register(vcall_address(0x474), |_, _| returns(1));
        let (other, weapon_two, actor_two, _) = equip_setup(&mut e, 0x28);
        e.call_log = Some(vec![]);
        e.call(
            0x004bffe0,
            &args![other, weapon_two, 1i32, actor_two, 0u32, 1u32, 0u32],
        );
        assert!(calls(&e, REFR_UPDATE_WEAPON).is_empty());
        assert!(calls(&e, TASK_QUEUE_ATTACH_WEAPON).is_empty());
        assert!(calls(&e, PLAYER_STORE_FORM).is_empty());
        assert_eq!(calls(&e, ACTOR_INIT_LIGHTING).len(), 1);
    }

    #[test]
    fn equip_ammunition_keeps_an_item_in_the_process() {
        let mut e = equip_engine();
        e.register(vcall_address(0x14c), |_, _| returns(0));
        e.register(vcall_address(0x168), |_, _| returns(1));
        e.register(vcall_address(0x474), |_, _| returns(0));
        let (this, ammo, actor, process) = equip_setup(&mut e, 0x29);
        e.call(
            0x004bffe0,
            &args![this, ammo, 9i32, actor, 0u32, 1u32, 0u32],
        );
        let accepted = calls(&e, vcall_address(0x168));
        assert_eq!(accepted.len(), 1);
        let kept = Ptr::<ItemChange>::new(accepted[0][1]);
        assert_eq!(accepted[0][0], process);
        // The count is 0: ammunition items are not counted here.
        assert_eq!(e.get(kept, ItemChange::iNumber), 0);
        assert_eq!(e.get(kept, ItemChange::pContainerObj), Ptr::new(ammo));
        assert_eq!(calls(&e, STORE_NOTIFY_004534F0), vec![vec![actor, ammo]]);
        // Refused: deleted, no notification.
        e.register(vcall_address(0x168), |_, _| returns(0));
        let (other, ammo_two, actor_two, _) = equip_setup(&mut e, 0x29);
        e.call_log = Some(vec![]);
        e.call(
            0x004bffe0,
            &args![other, ammo_two, 1i32, actor_two, 0u32, 1u32, 0u32],
        );
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE).len(), 1);
        assert!(calls(&e, STORE_NOTIFY_004534F0).is_empty());
        // The process already keeps one: its lists are replaced.
        let kept = e.mem.alloc(0x20);
        let kept_list = list(&mut e, &[0x1234]);
        e.mem.set_u32(kept, kept_list);
        e.mem.set_u32(LOOKUP_CELL + 0x20, kept);
        e.register(vcall_address(0x14c), |e, _| {
            returns(e.mem.u32(LOOKUP_CELL + 0x20))
        });
        let (third, ammo_three, actor_three, _) = equip_setup(&mut e, 0x29);
        e.call_log = Some(vec![]);
        e.call(
            0x004bffe0,
            &args![third, ammo_three, 1i32, actor_three, 0u32, 1u32, 0u32],
        );
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![kept_list]]);
        assert_eq!(e.mem.u32(kept + 8), ammo_three);
        assert_eq!(
            calls(&e, STORE_NOTIFY_004534F0),
            vec![vec![actor_three, ammo_three]]
        );
    }

    // ---- Tests of the third session (004c0c90 to 004d0490) ----

    /// Appends the change `entry` to the changes of `this`.
    fn put_entry(e: &mut Engine, this: Ptr<InventoryChanges>, entry: u32) {
        let head = e.get(this, InventoryChanges::pListofChanges).addr();
        list_call_with_item(e, LIST_ADD_TAIL, head, entry);
    }

    /// A worn plain extra list (`+4` null), no hot key.
    fn worn_plain_extra(e: &mut Engine) -> u32 {
        let x = extra(e);
        set(e, x, X_WORN, 1);
        set(e, x, X_ITEMS_IN_LIST, 1);
        set(e, x, X_HOT_KEY, 0xff);
        x
    }

    #[test]
    fn animation_float_is_raised_to_the_minimum() {
        let mut e = engine();
        e.map(ANIMATION_MINIMUM_DOUBLE & !0xfff, 0x2000);
        e.set_global(ANIMATION_MINIMUM_DOUBLE, 0.1f64);
        let animation = e.mem.alloc(0x200);
        e.call(0x004c0c90, &args![animation, 0.5f32]);
        assert_eq!(e.mem.f32(animation + 0x110), 0.5);
        // Below the minimum the stored value is 0.1f.
        e.call(0x004c0c90, &args![animation, 0.01f32]);
        assert_eq!(e.mem.f32(animation + 0x110), 0.1f32);
        // An unordered value is not below it.
        e.call(0x004c0c90, &args![animation, f32::NAN]);
        assert!(e.mem.f32(animation + 0x110).is_nan());
    }

    #[test]
    fn player_word_is_stored_at_0x1f0() {
        let mut e = engine();
        let player = e.mem.alloc(0x400);
        e.call(0x004c0cd0, &args![player, 7u32]);
        assert_eq!(e.mem.u32(player + 0x1f0), 7);
        e.call(0x004c0cd0, &args![player, 0u32]);
        assert_eq!(e.mem.u32(player + 0x1f0), 0);
    }

    #[test]
    fn taking_off_a_worn_armor_deletes_the_emptied_entry() {
        let mut e = equip_engine();
        e.register(GET_DISMEMBERMENT_EXTRA, |_, _| returns(0));
        e.register(COMPARE_LIST_FOR_CONTAINER, |_, _| returns(0));
        let (this, armor, actor, process) = equip_setup(&mut e, 0x18);
        let worn = worn_plain_extra(&mut e);
        let change = entry(&mut e, armor, &[worn], 1);
        put_entry(&mut e, this, change);
        e.mem.set_u32(LOOKUP_CELL + 8, 1);
        e.mem.set_u32(LAST_MERGED_EXTRA_LIST, 0x1234);
        let removed = e.mem.alloc(4);
        let done = e
            .call(
                0x004c0cf0,
                &args![this, removed, armor, 1i32, actor, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool();
        // The armor list was dropped and the empty entry deleted.
        assert!(!done);
        assert_eq!(e.mem.u8(removed), 1);
        assert_eq!(e.mem.u32(LAST_MERGED_EXTRA_LIST), 0);
        assert!(inventory_list(&e, this).is_empty());
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![change, 1]]);
        // The process was told, no power armor.
        assert_eq!(calls(&e, vcall_address(0x468)), vec![vec![process, 1]]);
        assert!(calls(&e, vcall_address(0x6e4)).is_empty());
        // The lighting of the actor is not refreshed once the entry is gone.
        assert!(calls(&e, ACTOR_INIT_LIGHTING).is_empty());
    }

    #[test]
    fn taking_off_power_armor_tells_the_process_at_the_end() {
        let mut e = equip_engine();
        e.register(GET_DISMEMBERMENT_EXTRA, |_, _| returns(0));
        let (this, armor, actor, process) = equip_setup(&mut e, 0x18);
        e.mem.set_u8(armor + ARMOR_BIPED_MODEL + 8, 0x20);
        // A second, unworn list keeps the entry alive.
        let worn = worn_plain_extra(&mut e);
        let kept = extra(&mut e);
        set(&mut e, kept, 4, 1);
        let change = entry(&mut e, armor, &[worn, kept], 2);
        put_entry(&mut e, this, change);
        let removed = e.mem.alloc(4);
        e.register(COMPARE_LIST_FOR_CONTAINER, |_, _| returns(1));
        let done = e
            .call(
                0x004c0cf0,
                &args![this, removed, armor, 1i32, actor, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool();
        assert!(!done);
        assert_eq!(calls(&e, vcall_address(0x6e4)), vec![vec![process, actor]]);
        assert_eq!(inventory_list(&e, this), vec![change]);
        assert_eq!(items(&e, e.mem.u32(change)), vec![kept]);
        // The actor's lighting was refreshed at the end.
        assert_eq!(calls(&e, ACTOR_INIT_LIGHTING), vec![vec![actor, 0x9999]]);
    }

    #[test]
    fn taking_off_without_an_entry_or_a_worn_list_changes_nothing() {
        let mut e = equip_engine();
        let (this, armor, actor, _) = equip_setup(&mut e, 0x18);
        let removed = e.mem.alloc(4);
        e.mem.set_u8(removed, 9);
        // No entry for the form: true, the flag untouched.
        let done = e
            .call(
                0x004c0cf0,
                &args![this, removed, armor, 1i32, actor, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool();
        assert!(done);
        assert_eq!(e.mem.u8(removed), 9);
        // An entry whose list is not worn: the flag is set, nothing else.
        let plain = extra(&mut e);
        set(&mut e, plain, 4, 1);
        let change = entry(&mut e, armor, &[plain], 1);
        put_entry(&mut e, this, change);
        let done = e
            .call(
                0x004c0cf0,
                &args![this, removed, armor, 1i32, actor, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool();
        assert!(done);
        assert_eq!(e.mem.u8(removed), 1);
        assert_eq!(inventory_list(&e, this), vec![change]);
        assert_eq!(e.mem.u32(plain + X_WORN), 0);
    }

    #[test]
    fn scratch_reference_is_deleted_and_cleared() {
        let mut e = logged();
        e.map(TEMP_REF_GLOBAL & !0xfff, 0x2000);
        let temp = e.mem.alloc(0x40);
        e.mem.set_u32(temp, VTABLE);
        e.mem.set_u32(TEMP_REF_GLOBAL, temp);
        e.call(0x004c1c40, &args![]);
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![temp, 1]]);
        assert_eq!(e.mem.u32(TEMP_REF_GLOBAL), 0);
        // Nothing to delete the second time.
        e.call(0x004c1c40, &args![]);
        assert_eq!(calls(&e, DESTRUCTOR).len(), 1);
    }

    #[test]
    fn add_object_makes_an_entry_with_a_counted_list() {
        let mut e = equip_engine();
        let (this, item_form, _, _) = equip_setup(&mut e, 0x28);
        e.register(vcall_address(0x100), |_, _| returns(0));
        let out = entry(&mut e, item_form, &[], 0);
        e.call(0x004c1520, &args![this, item_form, 3i32, 0u32, 0u32, out]);
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(item_form));
        assert_eq!(e.get(made, ItemChange::iNumber), 0);
        // The new list has the count and is on both the entry and `out`.
        let lists = items(&e, e.mem.u32(changes[0]));
        assert_eq!(lists.len(), 1);
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 3);
        assert_eq!(items(&e, e.mem.u32(out)), lists);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0xb83]);
    }

    #[test]
    fn add_object_reuses_a_single_item_list() {
        let mut e = equip_engine();
        let (this, item_form, _, _) = equip_setup(&mut e, 0x28);
        let single = extra(&mut e);
        set(&mut e, single, X_COUNT, 1);
        set(&mut e, single, 4, 1);
        let change = entry(&mut e, item_form, &[single], 1);
        put_entry(&mut e, this, change);
        let out = entry(&mut e, item_form, &[], 0);
        e.call(0x004c1520, &args![this, item_form, 5i32, 0u32, 0u32, out]);
        // The list of the entry got the new count and nothing was added.
        assert_eq!(items(&e, e.mem.u32(change)), vec![single]);
        assert_eq!(e.mem.u32(single + X_COUNT), 5);
        assert_eq!(items(&e, e.mem.u32(out)), vec![single]);
    }

    /// The engine of the tests of the reference and transfer code: the
    /// equip doubles plus the small accessors the third session reads.
    fn tx() -> Engine {
        let mut e = equip_engine();
        e.map(0x0102_0000, 0x2000);
        e.set_global(NEAR_FULL_HEALTH, 0.99f64);
        e.register(ITEM_SET_NUMBER, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(REFERENCE_BASE_FORM, |e, a| returns(e.mem.u32(a[0] + 0x20)));
        e.register(UNSIGNED_MINIMUM, |_, a| returns(a[0].min(a[1])));
        e.register(REFERENCE_PERSISTS, |e, a| returns(e.mem.u32(a[0] + 0x24)));
        e.register(REFERENCE_SCALE, |_, _| returns_float(1.5));
        e.register(EXTRA_GET_AMMO, |e, a| returns(e.mem.u32(a[0] + 0x78)));
        e.register(LIST_COUNT_NONNULL, |e, a| {
            let mut node = a[0];
            let mut count = 0;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            returns(count)
        });
        for address in [
            EXTRA_SET_SCALE,
            EXTRA_SET_REFERENCE,
            EXTRA_SET_AMMO,
            REFR_SET_HEALTH,
            EXTRA_COPY_LIST_FOR_CONTAINER,
            EXTRA_REMOVE_OWNERSHIP,
        ] {
            stub(&mut e, address);
        }
        e.register(KEY_STATE_TEST_041D, |_, _| returns(0));
        e.register(KEY_STATE_TEST_0435, |_, _| returns(0));
        e.register(vcall_address(FORM_NAME_SLOT), |_, a| returns(a[0] + 1));
        e
    }

    /// A reference to `base`: a big-vtable object with its base form at
    /// +0x20, its persistence word at +0x24 and its extra list at +0x40.
    fn world_reference(e: &mut Engine, base: u32, id: u32) -> u32 {
        let reference = object(e);
        set(e, reference, 0x20, base);
        set(e, reference, 0xc, id);
        e.mem.set_f32(reference + 0x40 + X_HEALTH, -1.0);
        e.mem.set_f32(reference + 0x40 + X_FLOAT, -1.0);
        reference
    }

    #[test]
    fn adding_a_reference_gives_armor_its_full_health_and_a_new_entry() {
        let mut e = tx();
        let owner = object(&mut e);
        set(&mut e, owner, 0xc, 0x14);
        let this = inventory(&mut e, &[], owner);
        let base = form(&mut e, 0x18, 0, 100);
        casts(&mut e, 0, 0, base);
        let reference = world_reference(&mut e, base, 0x15);
        e.call(0x004c1c90, &args![this, reference, 2i32, 0u32, 0u32]);
        // 100 / 0.99 truncates to 101 and the minimum with 99 is 99.
        assert_eq!(
            calls(&e, REFR_SET_HEALTH),
            vec![vec![reference, 99.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, SAVE_LOAD_LOG),
            vec![vec![
                ADDING_FULL_HEALTH_ARMOR,
                reference + 1,
                0x15,
                owner + 1,
                0x14
            ]]
        );
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(base));
        assert_eq!(e.get(made, ItemChange::iNumber), 2);
        assert!(e.get(this, InventoryChanges::bcountdirty));
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
        // The plain list made for the reference was thrown away.
        assert_eq!(calls(&e, DESTRUCTOR).len(), 1);
        assert_eq!(calls(&e, vcall_address(0x48)), vec![vec![owner, 0x20]]);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0xc3b]);
    }

    #[test]
    fn adding_a_persistent_reference_keeps_its_list_in_the_entry() {
        let mut e = tx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let base = form(&mut e, 0x18, 0, 100);
        let change = entry(&mut e, base, &[], 1);
        put_entry(&mut e, this, change);
        let reference = world_reference(&mut e, base, 0x15);
        // Health set already, a list that is not plain, persistent.
        e.mem.set_f32(reference + 0x40 + X_HEALTH, 50.0);
        set(&mut e, reference, 0x44, 1);
        set(&mut e, reference, 0x24, 1);
        e.call(0x004c1c90, &args![this, reference, 2i32, 0u32, 1u32]);
        assert!(calls(&e, REFR_SET_HEALTH).is_empty());
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            3
        );
        let lists = items(&e, e.mem.u32(change));
        assert_eq!(lists.len(), 1);
        // The copy is scaled, worn, counted and tied to the reference.
        assert_eq!(
            calls(&e, EXTRA_SET_SCALE),
            vec![vec![lists[0], 1.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, EXTRA_SET_REFERENCE),
            vec![vec![lists[0], reference]]
        );
        assert_eq!(e.mem.u32(lists[0] + X_WORN), 1);
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 2);
        assert_eq!(
            calls(&e, EXTRA_COPY_LIST_FOR_CONTAINER),
            vec![vec![lists[0], reference + 0x40, 0]]
        );
    }

    #[test]
    fn adding_an_object_without_an_entry_makes_one_and_drops_a_plain_list() {
        let mut e = tx();
        let this = inventory(&mut e, &[], 0);
        let item_form = form(&mut e, 0x1f, 0, 0);
        e.call(0x004c29a0, &args![this, item_form, 0u32, 4i32]);
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(item_form));
        assert_eq!(e.get(made, ItemChange::iNumber), 4);
        // The list made for the missing extra was plain and deleted.
        assert_eq!(calls(&e, DESTRUCTOR).len(), 1);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0xd28]);
    }

    #[test]
    fn adding_an_object_merges_an_equal_list_into_the_entry() {
        let mut e = tx();
        let this = inventory(&mut e, &[], 0);
        let item_form = form(&mut e, 0x1f, 0, 0);
        let held = extra(&mut e);
        set(&mut e, held, 4, 1);
        set(&mut e, held, X_COUNT, 2);
        let change = entry(&mut e, item_form, &[held], 2);
        put_entry(&mut e, this, change);
        let added = extra(&mut e);
        set(&mut e, added, 4, 1);
        set(&mut e, added, X_COUNT, 1);
        // CompareListForContainer says "same" (false) for the lists.
        e.register(COMPARE_LIST_FOR_CONTAINER, |_, _| returns(0));
        e.call(0x004c29a0, &args![this, item_form, added, 3i32]);
        assert_eq!(e.mem.u32(held + X_COUNT), 5);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            5
        );
        assert_eq!(items(&e, e.mem.u32(change)), vec![held]);
        assert_eq!(inventory_list(&e, this), vec![change]);
        // No key was held, so the merged list was deleted.
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![added, 1]]);
    }

    #[test]
    fn merging_changes_adds_numbers_and_counts_of_equal_lists() {
        let mut e = tx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item_form = form(&mut e, 0x1f, 0, 0);
        // No entry yet: the source is appended and the owner told.
        let first = entry(&mut e, item_form, &[], 2);
        e.call(0x004c3380, &args![this, first, 0u32]);
        assert_eq!(inventory_list(&e, this), vec![first]);
        assert_eq!(calls(&e, vcall_address(0x48)), vec![vec![owner, 0x20]]);
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
        // An equal list in the source only adds its count to the held one.
        e.register(COMPARE_LIST_FOR_CONTAINER, |_, _| returns(0));
        let held = extra(&mut e);
        set(&mut e, held, 4, 1);
        set(&mut e, held, X_COUNT, 3);
        let held_list = list(&mut e, &[held]);
        e.mem.set_u32(first, held_list);
        let incoming = extra(&mut e);
        set(&mut e, incoming, 4, 1);
        set(&mut e, incoming, X_COUNT, 2);
        let second = entry(&mut e, item_form, &[incoming], 2);
        e.call(0x004c3380, &args![this, second, 1u32]);
        assert_eq!(e.get(Ptr::<ItemChange>::new(first), ItemChange::iNumber), 4);
        assert_eq!(e.mem.u32(held + X_COUNT), 5);
        assert_eq!(items(&e, e.mem.u32(first)), vec![held]);
        // The merged list and, since asked, the source were deleted.
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![incoming, 1]]);
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![second, 1]]);
        // A source that cancels the number of an entry with no lists removes
        // the entry.
        let other_form = form(&mut e, 0x1f, 0, 0);
        let lone = entry(&mut e, other_form, &[], 2);
        put_entry(&mut e, this, lone);
        let cancel = entry(&mut e, other_form, &[], -2);
        e.call(0x004c3380, &args![this, cancel, 0u32]);
        assert_eq!(inventory_list(&e, this), vec![first, lone][..1].to_vec());
        assert!(calls(&e, ITEM_CHANGE_DELETE).contains(&vec![lone, 1]));
    }

    /// `tx()` plus a fake container model: the container of an owner is
    /// `owner + 0x10` and its list of objects (records of count, form) hangs
    /// at `container + 8`.
    fn tx2() -> Engine {
        let mut e = tx();
        e.register(CONTAINER_OBJECT_LIST, |e, a| {
            returns(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 8) })
        });
        e.register(CONTAINER_COUNT, |e, a| {
            let mut sum = 0u32;
            let mut node = if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 8) };
            while node != 0 && e.mem.u32(node) != 0 {
                let record = e.mem.u32(node);
                if e.mem.u32(record + 4) == a[1] {
                    sum = sum.wrapping_add(e.mem.u32(record));
                }
                node = e.mem.u32(node + 4);
            }
            returns(sum)
        });
        e.register(CONTAINER_HAS_FORM, |e, a| {
            let mut node = if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 8) };
            while node != 0 && e.mem.u32(node) != 0 {
                let record = e.mem.u32(node);
                if e.mem.u32(record + 4) == a[1] {
                    return returns(1);
                }
                node = e.mem.u32(node + 4);
            }
            returns(0)
        });
        e.register(IS_GOLD, |e, a| {
            returns((e.mem.u8(a[0] + F_TYPE) == 0x1e) as u32)
        });
        e.register(ITEM_CHANGE_CONSTRUCT_EMPTY, |_, a| returns(a[0]));
        e.register(IS_MENU_VISIBLE, |_, _| returns(0));
        e.register(EXTRA_SET_OWNER, |e, a| {
            e.mem.set_u32(a[0] + X_OWNERSHIP, a[1]);
            Ret::default()
        });
        e.register(EXTRA_GET_BY_TYPE, |_, _| returns(0));
        e.register(REFERENCE_GET_OWNER, |_, _| returns(0x4242));
        e.register(vcall_address(0x190), |_, _| Ret::default());
        e.register(EXTRA_COPY_FOR_REFERENCE, |e, a| {
            for offset in [4, X_COUNT, X_ITEMS_IN_LIST, X_HOT_KEY] {
                let word = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, word);
            }
            Ret::default()
        });
        for address in [EXTRA_FN_0041D000, EXTRA_FN_0041B010, SET_ACTION_FLAG] {
            stub(&mut e, address);
        }
        e
    }

    /// Gives `owner` a container with the objects `(count, form)`.
    fn give_container(e: &mut Engine, owner: u32, objects: &[(i32, u32)]) {
        let records: Vec<u32> = objects
            .iter()
            .map(|(count, form)| {
                let record = e.mem.alloc(8);
                e.mem.set_u32(record, *count as u32);
                e.mem.set_u32(record + 4, *form);
                record
            })
            .collect();
        let head = list(e, &records);
        e.mem.set_u32(owner + 0x10 + 8, head);
    }

    #[test]
    fn string_object_text_comes_from_the_string_conversion() {
        let mut e = logged();
        e.register(STRING_OBJECT_TO_CHARS, |_, a| returns(a[0] + 0x10));
        assert_eq!(e.call(0x004c69f0, &args![0x1000u32]).u32(), 0x1010);
        assert_eq!(calls(&e, STRING_OBJECT_TO_CHARS), vec![vec![0x1000]]);
    }

    #[test]
    fn pack_object_type_search_finds_container_objects_and_changes() {
        let mut e = tx2();
        e.register(PACKAGE_OBJECT_TYPE_FROM_FORM, |e, a| {
            returns(e.mem.u8(a[0] + F_TYPE) as u32 + 100)
        });
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let cloth = form(&mut e, 0x19, 0, 0);
        let armor = form(&mut e, 0x18, 0, 0);
        give_container(&mut e, owner, &[(3, cloth), (5, armor)]);
        let count = e.mem.alloc(4);
        // The container's armor wins; the entry's number is added.
        let change = entry(&mut e, armor, &[], 2);
        put_entry(&mut e, this, change);
        let found = e.call(0x004c6a10, &args![this, 124u32, count]).u32();
        assert_eq!(found, armor);
        assert_eq!(e.mem.u32(count), 7);
        // Nothing of that type: 0 and the count is left alone.
        e.mem.set_u32(count, 99);
        assert_eq!(e.call(0x004c6a10, &args![this, 555u32, count]).u32(), 0);
        assert_eq!(e.mem.u32(count), 99);
        // A form only the changes hold: its number is the count.
        let ammo = form(&mut e, 0x29, 0, 0);
        let loose = entry(&mut e, ammo, &[], 9);
        put_entry(&mut e, this, loose);
        let found = e.call(0x004c6a10, &args![this, 141u32, count]).u32();
        assert_eq!(found, ammo);
        assert_eq!(e.mem.u32(count), 9);
    }

    #[test]
    fn form_type_search_matches_the_form_type_byte() {
        let mut e = tx2();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let cloth = form(&mut e, 0x19, 0, 0);
        let armor = form(&mut e, 0x18, 0, 0);
        give_container(&mut e, owner, &[(3, cloth), (5, armor)]);
        let count = e.mem.alloc(4);
        assert_eq!(
            e.call(0x004c6ba0, &args![this, 0x19u32, count]).u32(),
            cloth
        );
        assert_eq!(e.mem.u32(count), 3);
        assert_eq!(
            e.call(0x004c6ba0, &args![this, 0x18u32, count]).u32(),
            armor
        );
        assert_eq!(e.mem.u32(count), 5);
        // An entry that cancels the container's number stores nothing.
        let change = entry(&mut e, cloth, &[], -3);
        put_entry(&mut e, this, change);
        e.mem.set_u32(count, 77);
        assert_eq!(
            e.call(0x004c6ba0, &args![this, 0x19u32, count]).u32(),
            cloth
        );
        assert_eq!(e.mem.u32(count), 77);
        // A form in neither place.
        assert_eq!(e.call(0x004c6ba0, &args![this, 0x40u32, count]).u32(), 0);
    }

    #[test]
    fn removing_a_form_type_takes_it_from_the_entry() {
        let mut e = tx2();
        // The game passes a null owner; fn_004c37d0 asks it for virtual 0x100.
        e.map(0, 0x1000);
        e.mem.set_u32(0, BIG_VTABLE);
        let this = inventory(&mut e, &[], 0);
        let armor = form(&mut e, 0x18, 0, 0);
        let change = entry(&mut e, armor, &[], 5);
        put_entry(&mut e, this, change);
        // A count of 0 only forgets the weight.
        e.set(this, InventoryChanges::fcontainerweight, 12.0);
        e.call(0x004c6d40, &args![this, 0x18u32, 0u32, 0i32]);
        assert_eq!(
            e.get(this, InventoryChanges::fpreviousContainerWeight),
            12.0
        );
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            5
        );
        e.call(0x004c6d40, &args![this, 0x18u32, 0u32, 3i32]);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            2
        );
        // Taking the rest removes the entry.
        e.call(0x004c6d40, &args![this, 0x18u32, 0u32, 2i32]);
        assert!(inventory_list(&e, this).is_empty());
        assert!(calls(&e, ITEM_CHANGE_DELETE).contains(&vec![change, 1]));
    }

    #[test]
    fn dropping_into_the_world_builds_the_position_and_places_the_object() {
        let mut e = tx2();
        e.map(0x011f_0000, 0x10000);
        e.map(0x0300_0000, 0x1000);
        let scratch = 0x0300_0000u32;
        e.set_global(GAME_DATA_MANAGER_GLOBAL, 0x7000u32);
        e.set_global(DROP_OFFSET_Y, 2.0f32);
        e.set_global(DROP_OFFSET_Z, 3.0f32);
        let placed = object(&mut e);
        set(&mut e, placed, 0, BIG_VTABLE);
        e.register(REFERENCE_WORLD_SPACE, |_, _| returns(0x333));
        e.register(REFERENCE_PARENT_CELL, |_, _| returns(0x444));
        e.register(PLACE_OBJECT, |e, a| {
            // Remember the position and rotation words and the arguments.
            for i in 0..3 {
                let position = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(0x0300_0000 + 4 * i, position);
                let rotation = e.mem.u32(a[3] + 4 * i);
                e.mem.set_u32(0x0300_0010 + 4 * i, rotation);
            }
            for (i, word) in a.iter().enumerate() {
                e.mem.set_u32(0x0300_0100 + 4 * i as u32, *word);
            }
            returns(e.mem.u32(0x0300_0200))
        });
        e.mem.set_u32(scratch + 0x200, placed);
        let item_form = form(&mut e, 0x1f, 0, 0);
        let actor = object(&mut e);
        // A given position and rotation are used as they are.
        let position = e.mem.alloc(12);
        let rotation = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
            e.mem.set_f32(rotation + 4 * i as u32, *v / 10.0);
        }
        let this = inventory(&mut e, &[], 0);
        let result = e
            .call(
                0x004c6dd0,
                &args![this, actor, item_form, 2i32, 0x55u32, position, rotation],
            )
            .u32();
        assert_eq!(result, placed);
        assert_eq!(e.mem.f32(scratch), 1.0);
        assert_eq!(e.mem.f32(scratch + 8), 3.0);
        assert_eq!(e.mem.f32(scratch + 0x14), 0.2);
        let seen: Vec<u32> = (0..9).map(|i| e.mem.u32(scratch + 0x100 + 4 * i)).collect();
        assert_eq!(seen[0], 0x7000);
        assert_eq!(seen[1], item_form);
        assert_eq!(seen[4], 0x444);
        assert_eq!(seen[5], 0x333);
        assert_eq!(seen[6..], [0x55, 0, 0]);
        // The count 2 is stored on the reference's own list and it is told.
        assert_eq!(e.mem.u32(placed + 0x40 + X_COUNT), 2);
        assert_eq!(calls(&e, vcall_address(0x48)), vec![vec![placed, 0x400]]);
        // Without a position the actor's is used, moved by the rotated vector.
        let actor_position = actor + 0x300;
        let actor_rotation = actor + 0x320;
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(actor_position + 4 * i as u32, *v);
            e.mem.set_f32(actor_rotation + 4 * i as u32, 0.5);
        }
        e.register(vcall_address(0x1f4), move |_, a| returns(a[0] + 0x300));
        e.register(ACTOR_ROTATION, |_, a| returns(a[0] + 0x320));
        e.register(MATRIX_FROM_EULER_ANGLES, |_, _| Ret::default());
        e.register(POINT3_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            Ret::default()
        });
        e.register(MATRIX_TIMES_VECTOR, |_, a| returns(a[2]));
        e.register(POINT3_ADD, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, sum);
            }
            Ret::default()
        });
        e.call(
            0x004c6dd0,
            &args![this, actor, item_form, 1i32, 0u32, 0u32, 0u32],
        );
        // 10 + 0, 20 + 2, 30 + 3; the rotation is the actor's.
        assert_eq!(e.mem.f32(scratch), 10.0);
        assert_eq!(e.mem.f32(scratch + 4), 22.0);
        assert_eq!(e.mem.f32(scratch + 8), 33.0);
        assert_eq!(e.mem.f32(scratch + 0x10), 0.5);
    }

    /// Everything `fn_004c37d0(this, actor, form, set_owner, count, extra,
    /// drop, target, ..)` takes with the usual `delete_extras` of 1.
    fn remove_items(
        e: &mut Engine,
        this: Ptr<InventoryChanges>,
        actor: u32,
        form: u32,
        set_owner: u32,
        count: i32,
        extra: u32,
        drop: u32,
        target: u32,
    ) -> u32 {
        e.call(
            0x004c37d0,
            &args![
                this, actor, form, set_owner, count, extra, drop, target, 0u32, 0u32, 1u32, 0u32,
                0u32
            ],
        )
        .u32()
    }

    #[test]
    fn removal_hands_the_counted_part_of_an_entry_to_the_target() {
        let mut e = tx2();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = form(&mut e, 0x1f, 5, 0);
        let change = entry(&mut e, item, &[], 5);
        put_entry(&mut e, this, change);
        let result = remove_items(&mut e, this, owner, item, 0, 3, 0, 0, target);
        assert_eq!(result, 0);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            2
        );
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, item, 0, 3]]
        );
        assert_eq!(
            calls(&e, vcall_address(0x48)),
            vec![vec![owner, 0x20], vec![target, 0x20]]
        );
        assert!(e.get(this, InventoryChanges::bcountdirty));
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), -1.0);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0xe74]);
        assert_eq!(calls(&e, SCOPE_GUARD_CLOSE).len(), 1);
    }

    #[test]
    fn removal_of_everything_gives_the_owner_and_deletes_the_entry() {
        let mut e = tx2();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = form(&mut e, 0x1f, 5, 0);
        let change = entry(&mut e, item, &[], 5);
        put_entry(&mut e, this, change);
        // set_owner: the target is given a list with the owner and the count.
        remove_items(&mut e, this, owner, item, 1, 5, 0, 0, target);
        let given = calls(&e, vcall_address(0x190));
        assert_eq!(given.len(), 1);
        assert_eq!(given[0][..2], [target, item]);
        assert_eq!(given[0][3], 5);
        let list = given[0][2];
        assert_eq!(e.mem.u32(list + X_OWNERSHIP), 0x4242);
        assert_eq!(e.mem.u32(list + X_COUNT), 5);
        // The emptied entry is gone.
        assert!(inventory_list(&e, this).is_empty());
        assert!(calls(&e, ITEM_CHANGE_DELETE).contains(&vec![change, 1]));
    }

    #[test]
    fn removal_of_part_of_a_stack_lowers_its_extra_list() {
        let mut e = tx2();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = form(&mut e, 0x1f, 5, 0);
        let stack = extra(&mut e);
        set(&mut e, stack, 4, 1);
        set(&mut e, stack, X_COUNT, 3);
        set(&mut e, stack, X_ITEMS_IN_LIST, 1);
        let change = entry(&mut e, item, &[stack], 3);
        put_entry(&mut e, this, change);
        remove_items(&mut e, this, owner, item, 0, 2, stack, 0, target);
        // The stack and the entry keep one; the target is told about two.
        assert_eq!(e.mem.u32(stack + X_COUNT), 1);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            1
        );
        assert_eq!(items(&e, e.mem.u32(change)), vec![stack]);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, item, 0, 2]]
        );
        assert_eq!(calls(&e, EXTRA_COPY_FOR_REFERENCE).len(), 1);
    }

    #[test]
    fn removal_with_the_drop_flag_places_the_items_in_the_world() {
        let mut e = tx2();
        e.set_global(GAME_DATA_MANAGER_GLOBAL, 0x7000u32);
        let placed = object(&mut e);
        e.mem.set_u32(0x0210_2300, placed);
        e.register(REFERENCE_WORLD_SPACE, |_, _| returns(0x333));
        e.register(REFERENCE_PARENT_CELL, |_, _| returns(0));
        e.register(DROPPED_REFERENCE_TEST, |_, _| returns(0));
        e.register(PLACE_OBJECT, |e, _| returns(e.mem.u32(0x0210_2300)));
        let owner = object(&mut e);
        let base = form(&mut e, 0x1f, 0, 0);
        set(&mut e, owner, 0x20, base);
        let this = inventory(&mut e, &[], owner);
        let item = form(&mut e, 0x1f, 5, 0);
        let change = entry(&mut e, item, &[], 5);
        put_entry(&mut e, this, change);
        let position = e.mem.alloc(12);
        let rotation = e.mem.alloc(12);
        let result = e
            .call(
                0x004c37d0,
                &args![
                    this, owner, item, 0u32, 3i32, 0u32, 1u32, 0u32, position, rotation, 1u32,
                    0u32, 0u32
                ],
            )
            .u32();
        assert_eq!(result, placed);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            2
        );
        // The placed reference holds the count and was told.
        assert_eq!(e.mem.u32(placed + 0x40 + X_COUNT), 3);
        assert!(calls(&e, vcall_address(0x48)).contains(&vec![placed, 0x400]));
        assert!(calls(&e, vcall_address(0x190)).is_empty());
    }

    #[test]
    fn removal_of_items_only_the_container_holds_records_a_negative_entry() {
        let mut e = tx2();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = form(&mut e, 0x1f, 5, 0);
        give_container(&mut e, owner, &[(4, item)]);
        remove_items(&mut e, this, owner, item, 0, 3, 0, 0, target);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, item, 0, 3]]
        );
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(item));
        assert_eq!(e.get(made, ItemChange::iNumber), -3);
    }

    /// `tx2()` plus the doubles for ammunition and weapon choice: casts to
    /// `TESAmmo` answer the form when its type byte is 0x29, the
    /// desirability and the weapon rating are floats at +0x30 of the form,
    /// the combat weapon type is the word at +0x34.
    fn tx3() -> Engine {
        let mut e = tx2();
        e.set_global(NEGATIVE_FLOAT_MAX, -3.4e38f32);
        e.register(RT_DYNAMIC_CAST, |e, a| {
            returns(
                if a[3] == TYPE_TES_AMMO && e.mem.u8(a[0] + F_TYPE) == 0x29 {
                    a[0]
                } else {
                    0
                },
            )
        });
        e.register(GET_DESIRABILITY, |e, a| {
            returns_float(e.mem.f32(a[1] + 0x30))
        });
        e.register(AMMO_LIST_OF_SLOT, |e, a| returns(e.mem.u32(a[0])));
        e.register(SINGLE_AMMO_OF_SLOT, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(FORM_LIST_HEAD, |_, a| returns(a[0] + 0x18));
        e.register(COMBAT_WEAPON_TYPE, |e, a| returns(e.mem.u32(a[0] + 0x34)));
        e.register(ACTOR_CAN_USE_WEAPON, |_, _| returns(1));
        e.register(RATE_WEAPON, |e, a| returns_float(e.mem.f32(a[1] + 0x30)));
        e.register(ROUND_SCORE, |_, a| {
            returns_float(f32::from_bits(a[0]) * 10.0)
        });
        e.register(WEAPON_VALUE_ID, |_, _| returns(0));
        e.register(WEAPON_TYPE_GETTER, |e, a| returns(e.mem.u32(a[0] + 0x34)));
        e.register(GET_FORM_AS_BIPED_MODEL, |e, a| {
            returns(e.mem.u32(a[0] + 0x38))
        });
        e.register(BIPED_FILLS_SLOT, |e, a| {
            returns((e.mem.u32(a[0]) == a[1]) as u32)
        });
        for address in [
            ACTOR_VALUE_FLOAT,
            ACTOR_VALUE_INT,
            ACTOR_FATIGUE_PERCENTAGE,
            vcall_address(0x10),
            vcall_address(0xc),
        ] {
            stub(&mut e, address);
        }
        e
    }

    /// A form list: the embedded list of forms at +0x18, the number of the
    /// first form that counts as added at +0x20.
    fn form_list(e: &mut Engine, forms: &[u32], first_added: u32) -> u32 {
        let list_form = e.mem.alloc(0x40);
        let mut node = list_form + 0x18;
        for (i, form) in forms.iter().enumerate() {
            e.mem.set_u32(node, *form);
            if i + 1 < forms.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.mem.set_u32(list_form + 0x20, first_added);
        list_form
    }

    /// A weapon form: type 0x28, rating `rating`, combat type `kind`, the
    /// sub-objects at +0x98, +0x9c and an actor value owner vtable.
    fn weapon_form(e: &mut Engine, rating: f32, kind: u32) -> u32 {
        let weapon = form(e, 0x28, 0, 100);
        e.mem.set_f32(weapon + 0x30, rating);
        set(e, weapon, 0x34, kind);
        set(e, weapon, 0x98, BIG_VTABLE);
        set(e, weapon, 0x9c, BIG_VTABLE);
        weapon
    }

    #[test]
    fn best_ammunition_is_the_most_desirable_of_container_and_changes() {
        let mut e = tx3();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let reference = object(&mut e);
        // Nothing at all.
        assert_eq!(e.call(0x004c6f60, &args![this, reference, 0u32]).u32(), 0);
        let low = form(&mut e, 0x29, 0, 0);
        let high = form(&mut e, 0x29, 0, 0);
        e.mem.set_f32(low + 0x30, 1.0);
        e.mem.set_f32(high + 0x30, 2.0);
        give_container(&mut e, owner, &[(10, low), (5, high)]);
        // From the container: the better one with its count.
        let best = e.call(0x004c6f60, &args![this, reference, 0u32]).u32();
        assert_ne!(best, 0);
        assert_eq!(e.mem.u32(best + 8), high);
        assert_eq!(entry_number(&mut e, best), 5);
        // An ammunition only the changes hold and like more takes over, with
        // the entry's list and number.
        let kept = form(&mut e, 0x29, 0, 0);
        e.mem.set_f32(kept + 0x30, 3.0);
        let list_extra = extra(&mut e);
        let change = entry(&mut e, kept, &[list_extra], 4);
        put_entry(&mut e, this, change);
        let best = e.call(0x004c6f60, &args![this, reference, 0u32]).u32();
        assert_eq!(e.mem.u32(best + 8), kept);
        assert_eq!(entry_number(&mut e, best), 4);
        assert_eq!(items(&e, e.mem.u32(best)), vec![list_extra]);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x13b5]);
    }

    #[test]
    fn loaded_ammunition_comes_from_the_actors_ammo_list_or_single_ammo() {
        let mut e = tx3();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let found = e.mem.alloc(4);
        let weapon_actor = e.mem.alloc(0x400);
        // No actor: 0 and the flag cleared.
        e.mem.set_u8(found, 7);
        assert_eq!(e.call(0x004c7300, &args![this, 0u32, found]).u32(), 0);
        assert_eq!(e.mem.u8(found), 0);
        // Two ammunitions in a list; only the second counts as "added".
        let first = form(&mut e, 0x29, 0, 0);
        let second = form(&mut e, 0x29, 0, 0);
        give_container(&mut e, owner, &[(3, first), (5, second)]);
        let ammo_list = form_list(&mut e, &[first, second], 1);
        e.mem.set_u32(weapon_actor + 0xa4, ammo_list);
        assert_eq!(
            e.call(0x004c7300, &args![this, weapon_actor, found]).u32(),
            second
        );
        assert_eq!(e.mem.u8(found), 1);
        // The second one is gone: the first (below the added index) is the
        // fallback.
        let third = form(&mut e, 0x29, 0, 0);
        give_container(&mut e, owner, &[(3, first), (0, second)]);
        let _ = third;
        assert_eq!(
            e.call(0x004c7300, &args![this, weapon_actor, found]).u32(),
            first
        );
        // Without a list the single ammunition of the slot is used.
        e.mem.set_u32(weapon_actor + 0xa4, 0);
        e.mem.set_u32(weapon_actor + 0xa8, second);
        give_container(&mut e, owner, &[(2, second)]);
        e.mem.set_u8(found, 0);
        assert_eq!(
            e.call(0x004c7300, &args![this, weapon_actor, found]).u32(),
            second
        );
        assert_eq!(e.mem.u8(found), 1);
        // A single ammunition with nothing left: flag set, 0 returned.
        give_container(&mut e, owner, &[]);
        e.mem.set_u8(found, 0);
        assert_eq!(
            e.call(0x004c7300, &args![this, weapon_actor, found]).u32(),
            0
        );
        assert_eq!(e.mem.u8(found), 1);
    }

    #[test]
    fn best_weapon_is_the_one_with_the_highest_rating() {
        let mut e = tx3();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let actor = object(&mut e);
        set(&mut e, actor, 0x100, BIG_VTABLE);
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(0x004c7400, &args![this, actor, out, 6i32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.f32(out), 0.0);
        let weak = weapon_form(&mut e, 5.0, 3);
        let strong = weapon_form(&mut e, 7.0, 3);
        let other_kind = weapon_form(&mut e, 99.0, 4);
        give_container(&mut e, owner, &[(1, weak), (1, strong), (1, other_kind)]);
        // Any kind (6): the 99 wins; a given kind leaves the others out.
        let best = e
            .call(0x004c7400, &args![this, actor, out, 3i32, 0u32])
            .u32();
        assert_eq!(e.mem.u32(best + 8), strong);
        assert_eq!(e.mem.f32(out), 70.0);
        let best = e
            .call(0x004c7400, &args![this, actor, out, 6i32, 0u32])
            .u32();
        assert_eq!(e.mem.u32(best + 8), other_kind);
        assert_eq!(e.mem.f32(out), 990.0);
        // A weapon only the changes have, with a list of its own.
        let carried = weapon_form(&mut e, 200.0, 3);
        let condition = extra(&mut e);
        e.mem.set_f32(condition + X_HEALTH, 50.0);
        e.register(EXTRA_GET_BY_TYPE, |_, _| returns(1));
        let change = entry(&mut e, carried, &[condition], 1);
        put_entry(&mut e, this, change);
        let best = e
            .call(0x004c7400, &args![this, actor, out, 3i32, 0u32])
            .u32();
        assert_eq!(e.mem.u32(best + 8), carried);
        assert_eq!(items(&e, e.mem.u32(best)), vec![condition]);
        assert_eq!(e.mem.f32(out), 2000.0);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x1444]);
    }

    #[test]
    fn worn_item_of_a_slot_is_a_new_change_with_the_worn_list() {
        let mut e = tx3();
        let this = inventory(&mut e, &[], 0);
        // A worn weapon and a worn armor filling slot 3 (biped model at +0x38
        // whose first word is the slot it fills).
        let weapon = weapon_form(&mut e, 1.0, 3);
        let weapon_list = worn_plain_extra(&mut e);
        set(&mut e, weapon_list, X_COUNT, 1);
        let armor = form(&mut e, 0x18, 0, 0);
        let biped = e.mem.alloc(8);
        e.mem.set_u32(biped, 3);
        set(&mut e, armor, 0x38, biped);
        let armor_list = worn_plain_extra(&mut e);
        set(&mut e, armor_list, X_COUNT, 2);
        let armor_change = entry(&mut e, armor, &[armor_list], 2);
        let weapon_change = entry(&mut e, weapon, &[weapon_list], 1);
        put_entry(&mut e, this, armor_change);
        put_entry(&mut e, this, weapon_change);
        let worn = e.call(0x004c8c10, &args![this, 3i32, 0u32]).u32();
        assert_eq!(e.mem.u32(worn + 8), armor);
        assert_eq!(entry_number(&mut e, worn), 2);
        assert_eq!(items(&e, e.mem.u32(worn)), vec![armor_list]);
        // Slot 5 is the worn weapon; with armor_only it is refused.
        let worn = e.call(0x004c8c10, &args![this, 5i32, 0u32]).u32();
        assert_eq!(e.mem.u32(worn + 8), weapon);
        assert_eq!(entry_number(&mut e, worn), 1);
        assert_eq!(e.call(0x004c8c10, &args![this, 5i32, 1u32]).u32(), 0);
        // A slot nothing fills.
        assert_eq!(e.call(0x004c8c10, &args![this, 7i32, 0u32]).u32(), 0);
    }

    #[test]
    fn object_count_adds_the_container_count_and_the_entry_number() {
        let mut e = tx3();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = form(&mut e, 0x1f, 0, 0);
        assert_eq!(e.call(0x004c8f30, &args![this, item]).i32(), 0);
        give_container(&mut e, owner, &[(4, item)]);
        assert_eq!(e.call(0x004c8f30, &args![this, item]).i32(), 4);
        // A negative container count is made positive.
        give_container(&mut e, owner, &[(-3, item)]);
        assert_eq!(e.call(0x004c8f30, &args![this, item]).i32(), 3);
        give_container(&mut e, owner, &[(4, item)]);
        let change = entry(&mut e, item, &[], 2);
        put_entry(&mut e, this, change);
        assert_eq!(e.call(0x004c8f30, &args![this, item]).i32(), 6);
        // An entry with nothing counted is one.
        give_container(&mut e, owner, &[]);
        e.set(Ptr::<ItemChange>::new(change), ItemChange::iNumber, 0);
        assert_eq!(e.call(0x004c8f30, &args![this, item]).i32(), 1);
    }

    /// `tx3()` plus the doubles the stack counting and armor code read:
    /// casts to armor (form type 0x18), ammunition (0x29) and leveled item
    /// (0x7e), the object copy, form setter and playable checks.
    fn tx4() -> Engine {
        let mut e = tx3();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            let kind = e.mem.u8(a[0] + F_TYPE);
            let wanted = match a[3] {
                TYPE_TES_AMMO => 0x29,
                TYPE_TES_OBJECT_ARMO => 0x18,
                TYPE_TES_LEV_ITEM => 0x7e,
                other => panic!("unexpected cast target {other:08x}"),
            };
            returns(if a[0] != 0 && kind == wanted { a[0] } else { 0 })
        });
        e.register(ITEM_SET_FORM, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(CONTAINER_OBJECT_COPY, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(WEAPON_PLAYABLE, |e, a| {
            returns((e.mem.u8(a[0] + 0x100) & 0x80 == 0) as u32)
        });
        e.register(BIPED_PLAYABLE, |e, a| returns(e.mem.u32(a[0] + 4)));
        e
    }

    /// An armor with a rating (hundredths), a damage threshold, health 100
    /// and a biped model (at +0x38) filling `slot`.
    fn armor_form(e: &mut Engine, rating: u16, slot: u32) -> u32 {
        let armor = form(e, 0x18, 0, 100);
        e.mem.set_u16(armor + ARMOR_RATING, rating);
        e.mem.set_f32(armor + ARMOR_DAMAGE_THRESHOLD, 0.0);
        set(e, armor, ARMOR_BIPED_MODEL, slot);
        let biped = e.mem.alloc(8);
        e.mem.set_u32(biped, slot);
        e.mem.set_u32(biped + 4, 1);
        set(e, armor, 0x38, biped);
        armor
    }

    #[test]
    fn best_armor_of_a_slot_scores_the_container_and_the_changes() {
        let mut e = tx4();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        // 2.0 and 3.0 times the health 100.
        let low = armor_form(&mut e, 200, 3);
        let other_slot = armor_form(&mut e, 300, 4);
        give_container(&mut e, owner, &[(1, low), (1, other_slot)]);
        let best = e.call(0x004c8220, &args![this, 0u32, 3i32, 0u32]).u32();
        assert_eq!(e.mem.u32(best + 8), low);
        let best = e.call(0x004c8220, &args![this, 0u32, 4i32, 0u32]).u32();
        assert_eq!(e.mem.u32(best + 8), other_slot);
        // Slot -1 and slots nothing fills give nothing.
        assert_eq!(e.call(0x004c8220, &args![this, 0u32, -1i32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x004c8220, &args![this, 0u32, 9i32, 0u32]).u32(), 0);
        // A better armor the changes hold; its list's health is used.
        let carried = armor_form(&mut e, 400, 3);
        let condition = extra(&mut e);
        e.mem.set_f32(condition + X_HEALTH, 60.0);
        e.register(EXTRA_GET_BY_TYPE, |_, _| returns(1));
        let change = entry(&mut e, carried, &[condition], 1);
        put_entry(&mut e, this, change);
        let best = e.call(0x004c8220, &args![this, 0u32, 3i32, 0u32]).u32();
        assert_eq!(e.mem.u32(best + 8), carried);
        assert_eq!(items(&e, e.mem.u32(best)), vec![condition]);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x163e]);
    }

    #[test]
    fn best_armor_returns_a_worn_one_that_can_not_be_taken_off() {
        let mut e = tx4();
        let this = inventory(&mut e, &[], 0);
        let worn_armor = armor_form(&mut e, 100, 3);
        let list = worn_plain_extra(&mut e);
        set(&mut e, list, X_COUNT, 1);
        set(&mut e, list, X_CAN_NOT_WEAR, 1);
        let change = entry(&mut e, worn_armor, &[list], 1);
        put_entry(&mut e, this, change);
        let result = e.call(0x004c8220, &args![this, 0u32, 3i32, 1u32]).u32();
        assert_eq!(e.mem.u32(result + 8), worn_armor);
        assert_eq!(items(&e, e.mem.u32(result)), vec![list]);
    }

    #[test]
    fn stack_count_adds_container_objects_and_changes_that_show() {
        let mut e = tx4();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        // Nothing: the count plus one.
        assert_eq!(e.call(0x004c8fd0, &args![this, 0u32]).i32(), 1);
        let cloth = form(&mut e, 0x1f, 0, 0);
        let drink = form(&mut e, 0x20, 0, 0);
        let gun = form(&mut e, 0x28, 0, 0);
        e.mem.set_u8(gun + 0x100, 0x80);
        let levelled = form(&mut e, 0x7e, 0, 0);
        give_container(
            &mut e,
            owner,
            &[(2, cloth), (1, drink), (1, gun), (1, levelled)],
        );
        // The unplayable weapon and the leveled item are left out.
        assert_eq!(e.call(0x004c8fd0, &args![this, 0u32]).i32(), 3);
        // include_all counts the weapon too.
        assert_eq!(e.call(0x004c8fd0, &args![this, 1u32]).i32(), 4);
        // An entry the container does not list adds one; the container's own
        // ammunition that is not playable is skipped.
        let ammo = form(&mut e, 0x29, 0, 0);
        e.mem.set_u8(ammo + 0xac, 2);
        let loose = form(&mut e, 0x1f, 0, 0);
        let change = entry(&mut e, loose, &[], 3);
        put_entry(&mut e, this, change);
        let hidden = entry(&mut e, ammo, &[], 3);
        put_entry(&mut e, this, hidden);
        assert_eq!(e.call(0x004c8fd0, &args![this, 0u32]).i32(), 4);
    }

    #[test]
    fn ammunition_is_playable_when_bit_one_is_clear() {
        let mut e = logged();
        let ammo = e.mem.alloc(0x100);
        assert!(e.call(0x004c94d0, &args![ammo]).bool());
        e.mem.set_u8(ammo + 0xac, 1);
        assert!(e.call(0x004c94d0, &args![ammo]).bool());
        e.mem.set_u8(ammo + 0xac, 2);
        assert!(!e.call(0x004c94d0, &args![ammo]).bool());
        e.mem.set_u8(ammo + 0xac, 0xfd);
        assert!(e.call(0x004c94d0, &args![ammo]).bool());
    }

    /// An inventory with a container holding 2 of one form and an entry of
    /// the changes for another form (3 of it).
    fn stack_setup(e: &mut Engine) -> (Ptr<InventoryChanges>, u32, u32) {
        let owner = object(e);
        let this = inventory(e, &[], owner);
        let held = form(e, 0x1f, 0, 0);
        let loose = form(e, 0x20, 0, 0);
        give_container(e, owner, &[(2, held)]);
        let change = entry(e, loose, &[], 3);
        put_entry(e, this, change);
        (this, held, loose)
    }

    #[test]
    fn stack_by_index_walks_the_container_then_the_changes() {
        let mut e = tx4();
        let (this, held, loose) = stack_setup(&mut e);
        let first = e.call(0x004c94f0, &args![this, 0i32]).u32();
        assert_eq!(e.mem.u32(first + 8), held);
        assert_eq!(entry_number(&mut e, first), 2);
        let second = e.call(0x004c94f0, &args![this, 1i32]).u32();
        assert_eq!(e.mem.u32(second + 8), loose);
        assert_eq!(entry_number(&mut e, second), 3);
        assert_eq!(e.call(0x004c94f0, &args![this, 2i32]).u32(), 0);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x187f]);
    }

    #[test]
    fn fast_iteration_returns_the_same_stacks_one_call_at_a_time() {
        let mut e = tx4();
        let (this, held, loose) = stack_setup(&mut e);
        let iterator = e.call(0x004ca200, &args![this]).u32();
        let container = this_container(&mut e, this);
        assert_eq!(e.mem.u32(iterator + 0x10), container);
        assert_eq!(e.mem.u32(iterator), e.mem.u32(container + 8));
        assert_eq!(e.mem.u32(iterator + 4), e.mem.u32(container + 8));
        assert_eq!(e.mem.u32(iterator + 0x18), 0);
        let first = e.call(0x004ca330, &args![this, iterator]).u32();
        assert_eq!(e.mem.u32(first + 8), held);
        assert_eq!(entry_number(&mut e, first), 2);
        assert_eq!(e.mem.u32(iterator + 0xc), first);
        let second = e.call(0x004ca330, &args![this, iterator]).u32();
        assert_eq!(e.mem.u32(second + 8), loose);
        assert_eq!(entry_number(&mut e, second), 3);
        assert_eq!(e.mem.u32(iterator + 0x18), 1);
        assert_eq!(e.call(0x004ca330, &args![this, iterator]).u32(), 0);
        let guard = &calls(&e, SCOPE_GUARD_OPEN);
        assert_eq!(&guard[0][1..], &[0x36, 1, SOURCE_FILE_NAME, 0x19b2]);
        assert_eq!(&guard[1][1..], &[0x36, 1, SOURCE_FILE_NAME, 0x19c9]);
    }

    /// The container of the owner of `this`.
    fn this_container(e: &mut Engine, this: Ptr<InventoryChanges>) -> u32 {
        fn_004bffb0(e, this)
    }

    #[test]
    fn iterator_body_clears_and_frees_its_list() {
        let mut e = logged();
        let iterator = e.mem.alloc(0x28);
        let list_head = list(&mut e, &[1]);
        e.mem.set_u32(iterator, list_head);
        e.call(0x004ca1a0, &args![iterator]);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![list_head]]);
        assert_eq!(calls(&e, LIST_DESTROY), vec![vec![list_head, 1]]);
        assert_eq!(e.mem.u32(iterator), 0);
        // Without a list nothing is called.
        e.call(0x004ca1a0, &args![iterator]);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 1);
    }

    /// A form with the big vtable (virtuals can be doubled) and the fields
    /// the fake accessors read.
    fn big_form(e: &mut Engine, form_type: u8, value: u32) -> u32 {
        let f = object(e);
        e.mem.set_u8(f + F_TYPE, form_type);
        set(e, f, F_VALUE, value);
        f
    }

    /// `tx4()` plus the food, gold and ownership doubles: ingredients are
    /// form type 0x2f, alchemy items 0x30, food is the word at +0x40 of the
    /// form, gold is form type 0x1e.
    fn tx5() -> Engine {
        let mut e = tx4();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            let kind = e.mem.u8(a[0] + F_TYPE);
            let wanted = match a[3] {
                TYPE_TES_AMMO => 0x29,
                TYPE_TES_OBJECT_ARMO => 0x18,
                TYPE_TES_LEV_ITEM => 0x7e,
                TYPE_INGREDIENT_ITEM => 0x2f,
                TYPE_ALCHEMY_ITEM => 0x30,
                TYPE_TES_HEALTH_FORM => 0x18,
                other => panic!("unexpected cast target {other:08x}"),
            };
            returns(if a[0] != 0 && kind == wanted { a[0] } else { 0 })
        });
        e.register(vcall_address(4), |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(vcall_address(0x94), |_, _| returns(0));
        e.register(EXTRA_LEVELED_POSITION, |e, a| {
            returns(e.mem.u32(a[0] + 0x7c))
        });
        e
    }

    /// A food of form type `kind`: the sub-object at +0x3c is big-vtable,
    /// its word at +4 (the form's +0x40) says it is food.
    fn food(e: &mut Engine, kind: u8, is_food: bool) -> u32 {
        let f = big_form(e, kind, 0);
        set(e, f, 0x3c, BIG_VTABLE);
        set(e, f, 0x40, is_food as u32);
        f
    }

    #[test]
    fn best_food_is_the_first_food_of_the_container_then_of_the_changes() {
        let mut e = tx5();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        // Nothing to eat.
        assert_eq!(e.call(0x004cafe0, &args![this]).u32(), 0);
        let rock = food(&mut e, 0x30, false);
        let stew = food(&mut e, 0x30, true);
        let apple = food(&mut e, 0x2f, true);
        give_container(&mut e, owner, &[(1, rock), (2, stew)]);
        // The changes know nothing about the stew: a new change, number 0.
        let found = e.call(0x004cafe0, &args![this]).u32();
        assert_eq!(e.mem.u32(found + 8), stew);
        assert_eq!(entry_number(&mut e, found), 0);
        // The changes' entry is returned when it exists and does not cancel.
        let change = entry(&mut e, stew, &[], 1);
        put_entry(&mut e, this, change);
        assert_eq!(e.call(0x004cafe0, &args![this]).u32(), change);
        // Cancelled by the changes (number -2 against 2): the changes' own
        // foods are looked at.
        e.set(Ptr::<ItemChange>::new(change), ItemChange::iNumber, -2);
        assert_eq!(e.call(0x004cafe0, &args![this]).u32(), 0);
        let fruit = entry(&mut e, apple, &[], 3);
        put_entry(&mut e, this, fruit);
        assert_eq!(e.call(0x004cafe0, &args![this]).u32(), fruit);
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x1afa]);
    }

    #[test]
    fn gold_amount_adds_the_container_and_the_changes() {
        let mut e = tx5();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        assert_eq!(e.call(0x004cb320, &args![this]).i32(), 0);
        let caps = big_form(&mut e, 0x1e, 0);
        let loose = big_form(&mut e, 0x1e, 0);
        let sword = big_form(&mut e, 0x28, 0);
        give_container(&mut e, owner, &[(100, caps), (7, sword)]);
        assert_eq!(e.call(0x004cb320, &args![this]).i32(), 100);
        // The changes' number for the same gold adds to the container's.
        let held = entry(&mut e, caps, &[], -30);
        put_entry(&mut e, this, held);
        assert_eq!(e.call(0x004cb320, &args![this]).i32(), 70);
        // Gold only the changes hold adds its number; an empty one does not.
        let extra_gold = entry(&mut e, loose, &[], 5);
        put_entry(&mut e, this, extra_gold);
        assert_eq!(e.call(0x004cb320, &args![this]).i32(), 75);
        e.set(Ptr::<ItemChange>::new(extra_gold), ItemChange::iNumber, 0);
        assert_eq!(e.call(0x004cb320, &args![this]).i32(), 70);
    }

    #[test]
    fn removing_gold_hands_the_first_gold_to_the_target() {
        let mut e = tx5();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let sword = big_form(&mut e, 0x28, 0);
        let caps = big_form(&mut e, 0x1e, 0);
        give_container(&mut e, owner, &[(7, sword), (100, caps)]);
        e.call(0x004cb4b0, &args![this, owner, 30i32, target]);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, caps, 0, 30]]
        );
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        let made = Ptr::<ItemChange>::new(changes[0]);
        assert_eq!(e.get(made, ItemChange::pContainerObj), Ptr::new(caps));
        assert_eq!(e.get(made, ItemChange::iNumber), -30);
        // Gold only the changes hold is found in the second pass.
        let mut e = tx5();
        let this = inventory(&mut e, &[], 0);
        let actor = object(&mut e);
        let target = object(&mut e);
        let pile = big_form(&mut e, 0x1e, 0);
        let change = entry(&mut e, pile, &[], 9);
        put_entry(&mut e, this, change);
        e.call(0x004cb4b0, &args![this, actor, 4i32, target]);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            5
        );
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, pile, 0, 4]]
        );
    }

    #[test]
    fn stolen_items_are_taken_back_with_the_list_that_has_another_owner() {
        let mut e = tx5();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = big_form(&mut e, 0x1f, 0);
        let thief = big_form(&mut e, 0x2b, 0);
        let stolen = extra(&mut e);
        set(&mut e, stolen, 4, 1);
        set(&mut e, stolen, X_COUNT, 2);
        set(&mut e, stolen, X_ITEMS_IN_LIST, 1);
        set(&mut e, stolen, X_OWNERSHIP, thief);
        let change = entry(&mut e, item, &[stolen], 2);
        put_entry(&mut e, this, change);
        e.call(0x004cb5f0, &args![this, owner, target, 0u32]);
        // The target received the stolen list and the entry is gone.
        let given = calls(&e, vcall_address(0x190));
        assert_eq!(given.len(), 1);
        assert_eq!(given[0][..2], [target, item]);
        assert!(inventory_list(&e, this).is_empty());
    }

    #[test]
    fn leveled_item_form_is_found_by_its_position_among_the_leveled_items() {
        let mut e = tx5();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let plain = big_form(&mut e, 0x1f, 0);
        let first = big_form(&mut e, 0x7e, 0);
        let second = big_form(&mut e, 0x7e, 0);
        let result_form = big_form(&mut e, 0x1f, 0);
        give_container(&mut e, owner, &[(1, first), (1, plain), (1, second)]);
        let marked = extra(&mut e);
        set(&mut e, marked, 4, 1);
        set(&mut e, marked, 0x7c, 1);
        let change = entry(&mut e, result_form, &[marked], 1);
        put_entry(&mut e, this, change);
        // The second leveled item is at position 1.
        assert_eq!(e.call(0x004cba70, &args![this, second]).u32(), result_form);
        // The first is at position 0 and no list is marked so.
        assert_eq!(e.call(0x004cba70, &args![this, first]).u32(), 0);
        assert_eq!(e.call(0x004cba70, &args![this, 0u32]).u32(), 0);
    }

    /// `tx5()` plus the script and list pop doubles `fn_004cbbc0` needs:
    /// the script of a form is the word at +0x44, popping the head of a
    /// list moves the next item into the head node.
    fn tx6() -> Engine {
        let mut e = tx5();
        e.register(FORM_SCRIPT_OF, |e, a| returns(e.mem.u32(a[0] + 0x44)));
        e.register(LIST_POP_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn load_fix_trims_the_extra_lists_to_the_number_in_the_container() {
        let mut e = tx6();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x1f, 0);
        give_container(&mut e, owner, &[(1, item)]);
        let mut lists = vec![];
        for _ in 0..3 {
            let x = extra(&mut e);
            set(&mut e, x, X_COUNT, 1);
            set(&mut e, x, X_ITEMS_IN_LIST, 1);
            lists.push(x);
        }
        let change = entry(&mut e, item, &lists, 1);
        put_entry(&mut e, this, change);
        e.call(0x004cbbc0, &args![this]);
        // The entry (1) and the container (1) allow two lists: the first
        // one was popped and deleted.
        assert_eq!(items(&e, e.mem.u32(change)), vec![lists[1], lists[2]]);
        assert_eq!(calls(&e, DESTRUCTOR), vec![vec![lists[0], 1]]);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(change), ItemChange::iNumber),
            1
        );
    }

    #[test]
    fn load_fix_drops_the_health_extra_that_equals_the_base_health() {
        let mut e = tx6();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            let kind = e.mem.u8(a[0] + F_TYPE);
            returns(match a[3] {
                TYPE_TES_HEALTH_FORM if a[0] != 0 && kind == 0x18 => a[0],
                _ => 0,
            })
        });
        e.register(vcall_address(0x10), |e, a| {
            returns(e.mem.u32(a[0] + F_HEALTH))
        });
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let armor = big_form(&mut e, 0x18, 0);
        set(&mut e, armor, F_HEALTH, 100);
        give_container(&mut e, owner, &[(1, armor)]);
        // One list at the base health (plain), one damaged (kept).
        let pristine = extra(&mut e);
        e.mem.set_f32(pristine + X_HEALTH, 100.0);
        set(&mut e, pristine, X_COUNT, 1);
        set(&mut e, pristine, X_ITEMS_IN_LIST, 1);
        let damaged = extra(&mut e);
        e.mem.set_f32(damaged + X_HEALTH, 40.0);
        set(&mut e, damaged, 4, 1);
        set(&mut e, damaged, X_COUNT, 1);
        set(&mut e, damaged, X_ITEMS_IN_LIST, 1);
        let change = entry(&mut e, armor, &[pristine, damaged], 1);
        put_entry(&mut e, this, change);
        e.call(0x004cbbc0, &args![this]);
        assert_eq!(items(&e, e.mem.u32(change)), vec![damaged]);
        assert!(calls(&e, DESTRUCTOR).contains(&vec![pristine, 1]));
        // The health and count extras of the pristine list were removed first.
        assert!(calls(&e, EXTRA_REMOVE_HEALTH).contains(&vec![pristine]));
        assert!(calls(&e, EXTRA_REMOVE_COUNT).contains(&vec![pristine]));
        assert!(!calls(&e, EXTRA_REMOVE_HEALTH).contains(&vec![damaged]));
    }

    /// `tx6()` plus the form creation, scripting and package doubles of the
    /// duplicate, query and transfer code: the scriptable cast answers the
    /// word at +0x50 of the form, a created form is a fresh big-vtable
    /// object of the asked type, the package type of a form is its form
    /// type plus 100 and a form matches a package type when the two agree.
    fn tx7() -> Engine {
        let mut e = tx6();
        e.map(0x0310_0000, 0x1000);
        e.set_global(GAME_DATA_MANAGER_GLOBAL, 0x7000u32);
        e.register(RT_DYNAMIC_CAST, |e, a| {
            let kind = e.mem.u8(a[0] + F_TYPE);
            let wanted = match a[3] {
                TYPE_TES_AMMO => 0x29,
                TYPE_TES_OBJECT_ARMO => 0x18,
                TYPE_TES_LEV_ITEM => 0x7e,
                TYPE_INGREDIENT_ITEM => 0x2f,
                TYPE_ALCHEMY_ITEM => 0x30,
                TYPE_TES_HEALTH_FORM => 0x18,
                TYPE_TES_SCRIPTABLE_FORM => return returns(e.mem.u32(a[0] + 0x50)),
                other => panic!("unexpected cast target {other:08x}"),
            };
            returns(if a[0] != 0 && kind == wanted { a[0] } else { 0 })
        });
        e.register(CREATE_FORM_OF_TYPE, |e, a| {
            let created = e.mem.alloc(0x400);
            e.mem.set_u32(created, BIG_VTABLE);
            e.mem.set_u8(created + F_TYPE, a[0] as u8);
            e.mem.set_u32(created + 0x50, created + 0x3f0);
            returns(created)
        });
        e.register(PACKAGE_FORM_MATCHES, |e, a| {
            returns((e.mem.u8(a[0] + F_TYPE) as u32 + 100 == a[1]) as u32)
        });
        e.register(PACKAGE_OBJECT_TYPE_FROM_FORM, |e, a| {
            returns(e.mem.u8(a[0] + F_TYPE) as u32 + 100)
        });
        e.register(vcall_address(0xfc), |_, _| returns(0));
        e.register(vcall_address(0x108), |_, _| Ret::default());
        e.register(vcall_address(0x1d0), |_, _| Ret::default());
        e.register(vcall_address(0x100), |_, _| returns(0));
        for address in [
            ADD_FORM_TO_DATA_HANDLER,
            ADD_CREATED_BASE_OBJECT,
            SEEN_FORMS_CONSTRUCT,
            SEEN_FORMS_DESTRUCT,
        ] {
            stub(&mut e, address);
        }
        e.register(SEEN_FORMS_CONSTRUCT, |e, _| {
            e.mem.set_u32(0x0310_0000, 0);
            Ret::default()
        });
        e.register(SEEN_FORMS_INSERT, |e, a| {
            let count = e.mem.u32(0x0310_0000);
            e.mem.set_u32(0x0310_0004 + 4 * count, a[1]);
            e.mem.set_u32(0x0310_0000, count + 1);
            Ret::default()
        });
        e.register(SEEN_FORMS_LOOKUP, |e, a| {
            let count = e.mem.u32(0x0310_0000);
            let seen = (0..count).any(|i| e.mem.u32(0x0310_0004 + 4 * i) == a[1]);
            returns(seen as u32)
        });
        e
    }

    /// A reference whose extra data list (at +0x40) already holds the
    /// `InventoryChanges` `changes` (its word at +0x1c).
    fn reference_with_changes(e: &mut Engine) -> (u32, Ptr<InventoryChanges>) {
        let reference = object(e);
        let changes = inventory(e, &[], reference);
        e.mem.set_u32(reference + 0x40 + 0x1c, changes.addr());
        (reference, changes)
    }

    #[test]
    fn duplicating_all_items_gives_the_other_inventory_copies() {
        let mut e = tx7();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (reference, destination) = reference_with_changes(&mut e);
        let item = big_form(&mut e, 0x1f, 0);
        let loose = big_form(&mut e, 0x20, 0);
        give_container(&mut e, owner, &[(2, item)]);
        let change = entry(&mut e, loose, &[], 3);
        put_entry(&mut e, this, change);
        e.call(0x004cd9c0, &args![this, 1u32, reference]);
        let copies = inventory_list(&e, destination);
        assert_eq!(copies.len(), 2);
        let first = Ptr::<ItemChange>::new(copies[0]);
        let second = Ptr::<ItemChange>::new(copies[1]);
        assert_eq!(e.get(first, ItemChange::pContainerObj), Ptr::new(item));
        assert_eq!(e.get(first, ItemChange::iNumber), 2);
        assert_eq!(e.get(second, ItemChange::pContainerObj), Ptr::new(loose));
        assert_eq!(e.get(second, ItemChange::iNumber), 3);
        // The source is left as it was.
        assert_eq!(inventory_list(&e, this), vec![change]);
        // Nothing happens without a reference or the first argument.
        e.call(0x004cd9c0, &args![this, 1u32, 0u32]);
        e.call(0x004cd9c0, &args![this, 0u32, reference]);
        assert_eq!(inventory_list(&e, destination), copies);
    }

    #[test]
    fn duplicating_a_scripted_item_clones_its_form_without_the_script() {
        let mut e = tx7();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (reference, destination) = reference_with_changes(&mut e);
        let item = big_form(&mut e, 0x1f, 0);
        set(&mut e, item, 0x44, 0x1234);
        give_container(&mut e, owner, &[(2, item)]);
        e.call(0x004cd9c0, &args![this, 1u32, reference]);
        let copies = inventory_list(&e, destination);
        assert_eq!(copies.len(), 1);
        let made = Ptr::<ItemChange>::new(copies[0]);
        let cloned = e.get(made, ItemChange::pContainerObj).addr();
        assert_ne!(cloned, item);
        assert_eq!(e.mem.u8(cloned + F_TYPE), 0x1f);
        assert_eq!(e.get(made, ItemChange::iNumber), 2);
        // The clone was copied from the form, had its script cleared (the
        // scriptable word at +4) and was registered twice.
        assert_eq!(calls(&e, vcall_address(0x108)), vec![vec![cloned, item]]);
        assert_eq!(e.mem.u32(cloned + 0x50), cloned + 0x3f0);
        assert_eq!(e.mem.u32(cloned + 0x3f0 + 4), 0);
        assert_eq!(
            calls(&e, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0x7000, cloned]]
        );
        assert_eq!(calls(&e, ADD_CREATED_BASE_OBJECT).len(), 1);
    }

    #[test]
    fn setting_a_form_script_writes_the_scriptable_form_only() {
        let mut e = logged();
        e.register(RT_DYNAMIC_CAST, |e, a| returns(e.mem.u32(a[0] + 0x50)));
        e.register(ITEM_SET_NUMBER, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let scriptable = e.mem.alloc(0x10);
        let form_a = e.mem.alloc(0x100);
        e.mem.set_u32(form_a + 0x50, scriptable);
        let form_b = e.mem.alloc(0x100);
        e.call(0x004ce300, &args![form_a, 0x77u32]);
        assert_eq!(e.mem.u32(scriptable + 4), 0x77);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![form_a, 0, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0]]
        );
        // A form that is not scriptable is left alone.
        e.call(0x004ce300, &args![form_b, 0x88u32]);
        assert_eq!(calls(&e, ITEM_SET_NUMBER).len(), 1);
    }

    #[test]
    fn form_id_query_looks_at_the_container_and_the_references_of_the_lists() {
        let mut e = tx7();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x1f, 0);
        set(&mut e, item, 0xc, 0x77);
        assert!(!e.call(0x004cfe20, &args![this, 0x77u32]).bool());
        give_container(&mut e, owner, &[(2, item)]);
        assert!(e.call(0x004cfe20, &args![this, 0x77u32]).bool());
        assert!(!e.call(0x004cfe20, &args![this, 0x78u32]).bool());
        // A negative count answers at once.
        give_container(&mut e, owner, &[(-1, item)]);
        assert!(e.call(0x004cfe20, &args![this, 0x77u32]).bool());
        // Cancelled by the changes (the sum is 0): just the count is used.
        give_container(&mut e, owner, &[(2, item)]);
        let cancelling = entry(&mut e, item, &[], -2);
        put_entry(&mut e, this, cancelling);
        assert!(e.call(0x004cfe20, &args![this, 0x77u32]).bool());
        // A reference held in an extra list of the changes.
        let mut e = tx7();
        let this = inventory(&mut e, &[], 0);
        let item = big_form(&mut e, 0x1f, 0);
        let referenced = big_form(&mut e, 0x1f, 0);
        set(&mut e, referenced, 0xc, 0x99);
        let list_with_reference = extra(&mut e);
        set(&mut e, list_with_reference, X_REFERENCE, referenced);
        let change = entry(&mut e, item, &[list_with_reference], 1);
        put_entry(&mut e, this, change);
        assert!(e.call(0x004cfe20, &args![this, 0x99u32]).bool());
        assert!(!e.call(0x004cfe20, &args![this, 0x9au32]).bool());
    }

    #[test]
    fn container_has_objects_checks_the_count_and_reports_the_object_type() {
        let mut e = tx7();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x1f, 0);
        let other = big_form(&mut e, 0x19, 0);
        set(&mut e, item, 0xc, 0x77);
        give_container(&mut e, owner, &[(3, other), (3, item)]);
        let out = e.mem.alloc(4);
        let has = |e: &mut Engine, form: u32, kind: u32, count: i32, form_id: u32, actor: u32| {
            e.call(
                0x004cffe0,
                &args![this, form, kind, count, out, form_id, actor],
            )
            .bool()
        };
        // The container holds the form and the changes know nothing of it.
        assert!(has(&mut e, item, 0, 99, 0, 0));
        assert_eq!(e.mem.u32(out), 0x1f + 100);
        // By package object type instead of by form (the first match).
        e.mem.set_u32(out, 0);
        assert!(has(&mut e, 0, 0x19 + 100, 1, 0, 0));
        assert_eq!(e.mem.u32(out), 0x19 + 100);
        // Nothing matches the type.
        assert!(!has(&mut e, 0, 0x55, 1, 0, 0));
        // With an entry the totals are added and compared with the count.
        let change = entry(&mut e, item, &[], 2);
        put_entry(&mut e, this, change);
        assert!(has(&mut e, item, 0, 5, 0, 0));
        assert!(!has(&mut e, 0xdead, 0, 5, 0, 0));
        // A form id is passed on to the id query.
        assert!(has(&mut e, 0, 0, 1, 0x77, 0));
        assert!(!has(&mut e, 0, 0, 1, 0x78, 0));
        // An actor with a process is told the form found.
        let actor = object(&mut e);
        set(&mut e, actor, ACTOR_CURRENT_PROCESS, 0);
        let process = object(&mut e);
        set(&mut e, actor, 0x68, process);
        e.register(vcall_address(0x100), |_, _| returns(1));
        assert!(has(&mut e, item, 0, 5, 0, actor));
        assert_eq!(calls(&e, vcall_address(0x1d0)), vec![vec![process, item]]);
    }

    #[test]
    fn script_query_finds_a_form_whose_virtual_says_so() {
        let mut e = tx7();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.register(vcall_address(0x94), |e, a| {
            returns(e.mem.u8(a[0] + 0x50) as u32)
        });
        let plain = big_form(&mut e, 0x1f, 0);
        let special = big_form(&mut e, 0x1f, 0);
        e.mem.set_u8(special + 0x50, 1);
        assert!(!e.call(0x004d0360, &args![this]).bool());
        give_container(&mut e, owner, &[(1, plain)]);
        assert!(!e.call(0x004d0360, &args![this]).bool());
        give_container(&mut e, owner, &[(1, plain), (1, special)]);
        assert!(e.call(0x004d0360, &args![this]).bool());
        // The changes cancel it out...
        let cancelling = entry(&mut e, special, &[], -1);
        put_entry(&mut e, this, cancelling);
        assert!(!e.call(0x004d0360, &args![this]).bool());
        // An entry of the changes alone holds some of it.
        let mut e2 = tx7();
        e2.register(vcall_address(0x94), |e, a| {
            returns(e.mem.u8(a[0] + 0x50) as u32)
        });
        let this = inventory(&mut e2, &[], 0);
        let special = big_form(&mut e2, 0x1f, 0);
        e2.mem.set_u8(special + 0x50, 1);
        let held = entry(&mut e2, special, &[], 2);
        put_entry(&mut e2, this, held);
        assert!(e2.call(0x004d0360, &args![this]).bool());
        e2.set(Ptr::<ItemChange>::new(held), ItemChange::iNumber, 0);
        assert!(!e2.call(0x004d0360, &args![this]).bool());
    }

    #[test]
    fn scripted_item_query_looks_at_each_form_once() {
        let mut e = tx7();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let plain = big_form(&mut e, 0x1f, 0);
        let scripted = big_form(&mut e, 0x1f, 0);
        set(&mut e, scripted, 0x44, 0x1234);
        assert!(!e.call(0x004d0490, &args![this]).bool());
        give_container(&mut e, owner, &[(1, plain)]);
        assert!(!e.call(0x004d0490, &args![this]).bool());
        give_container(&mut e, owner, &[(1, plain), (1, scripted)]);
        assert!(e.call(0x004d0490, &args![this]).bool());
        assert_eq!(calls(&e, SEEN_FORMS_CONSTRUCT).len(), 3);
        assert_eq!(calls(&e, SEEN_FORMS_CONSTRUCT)[0][1], 0x25);
        assert_eq!(calls(&e, SEEN_FORMS_DESTRUCT).len(), 3);
        // The changes cancel the container's scripted form; an entry for the
        // same form is not counted again since the form was seen.
        let cancelling = entry(&mut e, scripted, &[], -1);
        put_entry(&mut e, this, cancelling);
        assert!(!e.call(0x004d0490, &args![this]).bool());
        e.set(Ptr::<ItemChange>::new(cancelling), ItemChange::iNumber, 3);
        // With the entry holding some, the container's scripted form counts.
        assert!(e.call(0x004d0490, &args![this]).bool());
        // A scripted form only the changes hold counts through its entry.
        let mut e2 = tx7();
        let this = inventory(&mut e2, &[], 0);
        let scripted = big_form(&mut e2, 0x1f, 0);
        set(&mut e2, scripted, 0x44, 0x1234);
        let held = entry(&mut e2, scripted, &[], 2);
        put_entry(&mut e2, this, held);
        assert!(e2.call(0x004d0490, &args![this]).bool());
    }

    /// `tx7()` plus the message doubles of the transfer code: the text of a
    /// string constant is its address plus 0x10, a form's name is the form
    /// plus 1 and the pick up sound is 0x5050.
    fn tx8() -> Engine {
        let mut e = tx7();
        e.register(STRING_OBJECT_TO_CHARS, |_, a| returns(a[0] + 0x10));
        e.register(FULL_NAME_OF_FORM, |_, a| returns(a[0] + 1));
        e.register(PICK_UP_SOUND_NAME, |_, _| returns(0x5050));
        for address in [
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            FORMAT_STRING,
            SHOW_MESSAGE,
            AFTER_TRANSFER_REFRESH,
        ] {
            stub(&mut e, address);
        }
        e
    }

    /// Calls `fn_004ce380` through the engine; returns the total value.
    #[allow(clippy::too_many_arguments)]
    fn transfer_all(
        e: &mut Engine,
        address: u32,
        this: Ptr<InventoryChanges>,
        actor: u32,
        target: u32,
        flags: [u32; 4],
        type_filter: i32,
        skip_list: u32,
    ) -> f32 {
        e.call(
            address,
            &args![
                this,
                actor,
                target,
                flags[0],
                flags[1],
                flags[2],
                flags[3],
                type_filter,
                skip_list
            ],
        )
        .f32()
    }

    #[test]
    fn transfer_of_the_containers_items_sums_their_values() {
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = big_form(&mut e, 0x1f, 10);
        give_container(&mut e, owner, &[(2, item)]);
        let total = transfer_all(&mut e, 0x004ce380, this, owner, target, [0; 4], -1, 0);
        assert_eq!(total, 20.0);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, item, 0, 2]]
        );
        // The owner has no entry for it any more than before: a record of -2.
        let changes = inventory_list(&e, this);
        assert_eq!(changes.len(), 1);
        assert_eq!(
            e.get(Ptr::<ItemChange>::new(changes[0]), ItemChange::iNumber),
            -2
        );
        // No message for a target that is not the player.
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        assert!(calls(&e, AFTER_TRANSFER_REFRESH).is_empty());
    }

    #[test]
    fn transfer_to_the_player_shows_the_message_unless_quiet() {
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x19, 4);
        let single = big_form(&mut e, 0x19, 4);
        give_container(&mut e, owner, &[(3, item), (1, single)]);
        let total = transfer_all(&mut e, 0x004ce380, this, owner, PLAYER, [0; 4], -1, 0);
        assert_eq!(total, 16.0);
        let formats = calls(&e, FORMAT_STRING);
        assert_eq!(formats.len(), 2);
        // Plural: the count, the name, the two words; single: the name.
        assert_eq!(
            formats[0][1..],
            [
                FORMAT_COUNT_NAME,
                3,
                item + 1,
                MESSAGE_WORD_TWO + 0x10,
                MESSAGE_WORD_THREE + 0x10
            ]
        );
        assert_eq!(
            formats[1][1..],
            [FORMAT_NAME, single + 1, MESSAGE_WORD_THREE + 0x10]
        );
        let shown = calls(&e, SHOW_MESSAGE);
        assert_eq!(shown.len(), 2);
        assert_eq!(shown[0][1..], [0, STOLEN_ITEM_ICON, 0x5050, 0, 0]);
        assert_eq!(calls(&e, PICK_UP_SOUND_NAME)[0], vec![PLAYER, item, 1, 0]);
        assert_eq!(calls(&e, AFTER_TRANSFER_REFRESH).len(), 2);
        // Quiet: the refresh still runs, the message does not.
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x19, 4);
        give_container(&mut e, owner, &[(3, item)]);
        transfer_all(&mut e, 0x004ce380, this, owner, PLAYER, [0, 0, 0, 1], -1, 0);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        assert!(calls(&e, FORMAT_STRING).is_empty());
        assert_eq!(calls(&e, AFTER_TRANSFER_REFRESH).len(), 1);
        // A form type that is not announced makes the text but not the message.
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x20, 4);
        give_container(&mut e, owner, &[(3, item)]);
        transfer_all(&mut e, 0x004ce380, this, owner, PLAYER, [0; 4], -1, 0);
        assert_eq!(calls(&e, FORMAT_STRING).len(), 1);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn transfer_of_an_entry_without_lists_takes_the_whole_number() {
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let loose = big_form(&mut e, 0x20, 7);
        let other = big_form(&mut e, 0x1f, 100);
        let change = entry(&mut e, loose, &[], 3);
        let kept = entry(&mut e, other, &[], 5);
        put_entry(&mut e, this, change);
        put_entry(&mut e, this, kept);
        // Only the type 0x20 is moved.
        let total = transfer_all(&mut e, 0x004ce380, this, owner, target, [0; 4], 0x20, 0);
        assert_eq!(total, 21.0);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, loose, 0, 3]]
        );
        assert_eq!(inventory_list(&e, this), vec![kept]);
    }

    #[test]
    fn transfer_leaves_out_the_forms_of_the_skip_list() {
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = big_form(&mut e, 0x1f, 10);
        let wanted = big_form(&mut e, 0x1f, 1);
        give_container(&mut e, owner, &[(2, item), (1, wanted)]);
        let skip = form_list(&mut e, &[item], 0);
        let total = transfer_all(&mut e, 0x004ce380, this, owner, target, [0; 4], -1, skip);
        assert_eq!(total, 1.0);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, wanted, 0, 1]]
        );
    }

    #[test]
    fn transfer_of_an_entry_with_lists_gives_them_one_by_one() {
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let loose = big_form(&mut e, 0x20, 7);
        let first = extra(&mut e);
        let second = extra(&mut e);
        for (list, count) in [(first, 1), (second, 2)] {
            set(&mut e, list, 4, 1);
            set(&mut e, list, X_COUNT, count);
            set(&mut e, list, X_ITEMS_IN_LIST, 1);
            set(&mut e, list, X_HOT_KEY, 0xff);
        }
        let change = entry(&mut e, loose, &[first, second], 3);
        put_entry(&mut e, this, change);
        let total = transfer_all(&mut e, 0x004ce380, this, owner, target, [0; 4], -1, 0);
        assert_eq!(total, 21.0);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![
                vec![target, loose, first, 1],
                vec![target, loose, second, 2]
            ]
        );
        assert!(inventory_list(&e, this).is_empty());
    }

    #[test]
    fn transfer_wrapper_passes_its_arguments_on() {
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target = object(&mut e);
        let item = big_form(&mut e, 0x1f, 10);
        give_container(&mut e, owner, &[(2, item)]);
        let total = transfer_all(&mut e, 0x004ce340, this, owner, target, [0; 4], -1, 0);
        assert_eq!(total, 20.0);
        assert_eq!(
            calls(&e, vcall_address(0x190)),
            vec![vec![target, item, 0, 2]]
        );
        // Quiet and the player as the target through the wrapper too.
        let mut e = tx8();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let item = big_form(&mut e, 0x19, 10);
        give_container(&mut e, owner, &[(2, item)]);
        transfer_all(&mut e, 0x004ce340, this, owner, PLAYER, [0, 0, 0, 1], -1, 0);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        assert_eq!(calls(&e, AFTER_TRANSFER_REFRESH).len(), 1);
    }

    // ---- Tests of the fourth session (004d0650 to 0076b630) ----

    /// The float weight of a fake form, the "excluded" word its virtual 0x94
    /// returns and the "is an actor" word its virtual 0x100 returns.
    const F_WEIGHT: u32 = 0x60;
    const O_EXCLUDED: u32 = 0x54;
    const O_IS_ACTOR: u32 = 0x58;
    /// The float each entry point (by number) leaves in its output.
    const ENTRY_POINT_OUTPUT: u32 = 0x011c_7000;
    /// The word `00418550(extra, owner)` stores in a fake extra list.
    const X_OWNER_SET: u32 = 0x78;
    /// The leveled item position of a fake extra list.
    const X_LEVELED_POSITION: u32 = 0x7c;

    /// `tx8()` plus the doubles of the fourth session.
    fn fx() -> Engine {
        let mut e = tx8();
        e.map(0x0102_0000, 0x2000);
        e.map(0x011c_0000, 0x4_0000);
        e.set_global(HEAVY_WEAPON_WEIGHT, 10.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register(ITEM_SET_NUMBER, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(ITEM_SET_FORM, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(SETTING_FLOAT_ADDRESS, |_, a| returns(a[0] + 0x10));
        e.register(WEIGHT_FORM_GET_WEIGHT, |e, a| {
            returns_float(e.mem.f32(a[0] + F_WEIGHT))
        });
        e.register(GET_WEIGHT, |e, a| returns_float(e.mem.f32(a[0])));
        e.register(EXTRA_SET_OWNER_EXTRA, |e, a| {
            e.mem.set_u32(a[0] + X_OWNER_SET, a[1]);
            Ret::default()
        });
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(ENTRY_POINT_OUTPUT + 4 * a[0]);
            e.mem.set_f32(a[2], value);
            Ret::default()
        });
        stub(&mut e, ACTOR_WEIGHT_CHANGED);
        e.register(ABS_INT, |_, a| returns((a[0] as i32).unsigned_abs()));
        e.register(EXTRA_LEVELED_POSITION, |e, a| {
            returns(e.mem.u32(a[0] + X_LEVELED_POSITION))
        });
        e.register(vcall_address(0), |_, _| Ret::default());
        e.register(vcall_address(0x10), |_, _| Ret::default());
        e.register(vcall_address(0x48), |_, _| Ret::default());
        e.register(vcall_address(0x94), |e, a| {
            returns(e.mem.u32(a[0] + O_EXCLUDED))
        });
        e.register(vcall_address(0x100), |e, a| {
            returns(e.mem.u32(a[0] + O_IS_ACTOR))
        });
        e
    }

    fn set_float(e: &mut Engine, base: u32, offset: u32, value: f32) {
        e.mem.set_f32(base + offset, value);
    }

    /// A form of type `form_type` with the weight `weight`.
    fn weighted_form(e: &mut Engine, form_type: u8, weight: f32) -> u32 {
        let f = big_form(e, form_type, 0);
        set_float(e, f, F_WEIGHT, weight);
        f
    }

    /// An extra list with a leveled item and `count` items.
    fn leveled_extra(e: &mut Engine, count: u32, position: u32) -> u32 {
        let x = extra(e);
        set(e, x, X_LEVELED_ITEM, 1);
        set(e, x, X_COUNT, count);
        set(e, x, X_LEVELED_POSITION, position);
        x
    }

    #[test]
    fn inventory_item_without_an_entry_comes_from_the_container() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let wanted = big_form(&mut e, 0x20, 1);
        let other = big_form(&mut e, 0x21, 1);
        give_container(&mut e, owner, &[(2, other), (3, wanted)]);
        let item = e.call(0x004d0650, &args![this, wanted, 0u32]).u32();
        assert_ne!(item, 0);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (3, wanted));
        let missing = big_form(&mut e, 0x22, 1);
        assert_eq!(e.call(0x004d0650, &args![this, missing, 0u32]).u32(), 0);
    }

    #[test]
    fn inventory_item_with_an_entry_adds_the_container_count_and_copies_lists() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        give_container(&mut e, owner, &[(2, form_a)]);
        // A plain first list: only it is copied.
        let (plain, other) = (extra(&mut e), extra(&mut e));
        let change = entry(&mut e, form_a, &[plain, other], 5);
        put_entry(&mut e, this, change);
        let item = e.call(0x004d0650, &args![this, form_a, 0u32]).u32();
        assert_eq!(e.mem.u32(item + 4), 7);
        assert_eq!(items(&e, e.mem.u32(item)), vec![plain]);
        // A default first list: all lists are copied.
        let (default, second) = (extra(&mut e), extra(&mut e));
        set(&mut e, default, X_DEFAULT, 1);
        let form_b = big_form(&mut e, 0x20, 1);
        let change = entry(&mut e, form_b, &[default, second], 1);
        put_entry(&mut e, this, change);
        let item = e.call(0x004d0650, &args![this, form_b, 0u32]).u32();
        assert_eq!(items(&e, e.mem.u32(item)), vec![default, second]);
        // An entry without lists gives an item without a list.
        let form_c = big_form(&mut e, 0x20, 1);
        let change = entry(&mut e, form_c, &[], 4);
        put_entry(&mut e, this, change);
        let item = e.call(0x004d0650, &args![this, form_c, 0u32]).u32();
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(e.mem.u32(item + 4), 4);
        assert_eq!(calls(&e, LIST_DESTROY).len(), 1);
    }

    /// Inventory weight fixture: a container with a light object (weight 2,
    /// three of them) and an excluded one, and an armor entry (weight 5, two).
    fn weight_setup(e: &mut Engine, actor: u32) -> (Ptr<InventoryChanges>, u32, u32) {
        let owner = object(e);
        e.mem.set_u32(owner + O_IS_ACTOR, actor);
        let this = inventory(e, &[], owner);
        e.set(this, InventoryChanges::fcontainerweight, -1.0f32);
        let light = weighted_form(e, 0x20, 2.0);
        let heavy = weighted_form(e, 0x20, 100.0);
        e.mem.set_u32(heavy + O_EXCLUDED, 1);
        let armor = weighted_form(e, 0x18, 5.0);
        give_container(e, owner, &[(3, light), (1, heavy)]);
        let change = entry(e, armor, &[], 2);
        put_entry(e, this, change);
        (this, light, owner)
    }

    #[test]
    fn inventory_weight_counts_container_and_changes_and_is_cached() {
        let mut e = fx();
        let (this, light, _) = weight_setup(&mut e, 0);
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 16.0);
        assert_eq!(e.get(this, InventoryChanges::fcontainerweight), 16.0);
        // The result is cached while it is not -1.0.
        set_float(&mut e, light, F_WEIGHT, 50.0);
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 16.0);
        assert!(calls(&e, ACTOR_WEIGHT_CHANGED).is_empty());
    }

    #[test]
    fn inventory_weight_applies_the_light_item_factor() {
        let mut e = fx();
        set_float(&mut e, ENTRY_POINT_OUTPUT, ENTRY_POINT_LIGHT_ITEMS * 4, 1.0);
        set_float(&mut e, LIGHT_WEIGHT_FACTOR_SETTING, 0x10, 0.5);
        set_float(&mut e, LIGHT_WEIGHT_LIMIT_SETTING, 0x10, 3.0);
        let (this, _, _) = weight_setup(&mut e, 0);
        // 3 * (2 * 0.5) + 2 * 5: the armor is over the limit.
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 13.0);
    }

    #[test]
    fn inventory_weight_tells_an_actor_owner_the_change() {
        let mut e = fx();
        let (this, _, owner) = weight_setup(&mut e, 1);
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 16.0);
        assert_eq!(
            calls(&e, ACTOR_WEIGHT_CHANGED),
            vec![vec![owner + 0xa4, 0x2e, 0, 16.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls(&e, HANDLE_ENTRY_POINT)[0][..2],
            [ENTRY_POINT_LIGHT_ITEMS, owner]
        );
    }

    #[test]
    fn inventory_weight_of_a_worn_armor_counts_once_at_full_weight() {
        let mut e = fx();
        let owner = object(&mut e);
        e.mem.set_u32(owner + O_IS_ACTOR, 1);
        let this = inventory(&mut e, &[], owner);
        e.set(this, InventoryChanges::fcontainerweight, -1.0f32);
        give_container(&mut e, owner, &[]);
        let armor = weighted_form(&mut e, 0x18, 5.0);
        let worn = worn_plain_extra(&mut e);
        let change = entry(&mut e, armor, &[worn], 2);
        put_entry(&mut e, this, change);
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 10.0);
    }

    #[test]
    fn inventory_weight_of_weapons_uses_the_modded_weight_per_stack() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.set(this, InventoryChanges::fcontainerweight, -1.0f32);
        give_container(&mut e, owner, &[]);
        let weapon = weighted_form(&mut e, 0x28, 4.0);
        set_float(&mut e, weapon, WEAPON_WEIGHT_FORM, 3.0);
        let plain = entry(&mut e, weapon, &[], 2);
        put_entry(&mut e, this, plain);
        // Without lists: the count times the plain weight.
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 8.0);
        // One of two weapons carries a weight mod (mod action 4, value 1).
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.set(this, InventoryChanges::fcontainerweight, -1.0f32);
        give_container(&mut e, owner, &[]);
        let weapon = weighted_form(&mut e, 0x28, 4.0);
        set_float(&mut e, weapon, WEAPON_WEIGHT_FORM, 3.0);
        set(&mut e, weapon, WEAPON_MOD_ACTION, 4);
        set_float(&mut e, weapon, WEAPON_MOD_ACTION_VALUE, 1.0);
        let modded = extra(&mut e);
        set(&mut e, modded, X_COUNT, 1);
        set(&mut e, modded, X_HAS_MODS, 1);
        set(&mut e, modded, X_SLOT_ACTIVE, 1);
        let stack = entry(&mut e, weapon, &[modded], 2);
        put_entry(&mut e, this, stack);
        // (2 - 1) * 4 + 1 * (3 - 1)
        assert_eq!(e.call(0x004d0900, &args![this, 0u32]).f32(), 6.0);
    }

    #[test]
    fn inventory_value_sums_container_and_changes_with_the_filters() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let plain = big_form(&mut e, 0x20, 10);
        let caps = big_form(&mut e, 0x20, 1);
        e.mem.set_u32(caps + 0xc, 0xf);
        let excluded = big_form(&mut e, 0x20, 50);
        e.mem.set_u32(excluded + O_EXCLUDED, 1);
        let unknown = big_form(&mut e, 0x20, 0xffff_ffff);
        let loose = big_form(&mut e, 0x20, 7);
        give_container(
            &mut e,
            owner,
            &[(2, plain), (100, caps), (1, excluded), (4, unknown)],
        );
        let change = entry(&mut e, loose, &[], 3);
        put_entry(&mut e, this, change);
        assert_eq!(e.call(0x004d0f40, &args![this, 0u32, 0u32]).i32(), 41);
        assert_eq!(e.call(0x004d0f40, &args![this, 0u32, 1u32]).i32(), 141);
        assert_eq!(e.call(0x004d0f40, &args![this, 1u32, 0u32]).i32(), 91);
        // An entry for a container object adds its number to the count and is
        // not counted again.
        let listed = entry(&mut e, plain, &[], 4);
        put_entry(&mut e, this, listed);
        assert_eq!(e.call(0x004d0f40, &args![this, 0u32, 0u32]).i32(), 81);
    }

    #[test]
    fn player_weight_mode_is_one_at_0x7bc() {
        let mut e = fx();
        let player = e.mem.alloc(0x800);
        assert!(!e.call(0x004d1360, &args![player]).bool());
        e.mem.set_u32(player + 0x7bc, 1);
        assert!(e.call(0x004d1360, &args![player]).bool());
    }

    #[test]
    fn worn_weight_counts_worn_items_and_a_shared_weapon_once() {
        let mut e = fx();
        e.register(ACTOR_PROCESS, |e, a| returns(e.mem.u32(a[0] + 0x68)));
        e.register(vcall_address(0x148), |e, a| returns(e.mem.u32(a[0] + 0x60)));
        e.register(vcall_address(0x14c), |e, a| returns(e.mem.u32(a[0] + 0x64)));
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        give_container(&mut e, owner, &[]);
        let armor = weighted_form(&mut e, 0x18, 5.0);
        let ammo = weighted_form(&mut e, 0x29, 1.0);
        let weapon = weighted_form(&mut e, 0x28, 4.0);
        set_float(&mut e, weapon, WEAPON_WEIGHT_FORM, 3.0);
        let spare = weighted_form(&mut e, 0x20, 9.0);
        for (form, number) in [(armor, 2), (ammo, 7), (weapon, 4), (spare, 3)] {
            let worn = worn_plain_extra(&mut e);
            // The spare form is not worn.
            if form == spare {
                set(&mut e, worn, X_WORN, 0);
            }
            let change = entry(&mut e, form, &[worn], number);
            put_entry(&mut e, this, change);
        }
        let actor = object(&mut e);
        let process = object(&mut e);
        e.mem.set_u32(actor + 0x68, process);
        // Without a shared weapon item: 2 * 5 + 4 * 3.
        assert_eq!(e.call(0x004d1180, &args![this, actor]).f32(), 22.0);
        // The same item for both virtuals: the weapon counts once.
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 8, weapon);
        e.mem.set_u32(process + 0x60, item);
        e.mem.set_u32(process + 0x64, item);
        assert_eq!(e.call(0x004d1180, &args![this, actor]).f32(), 13.0);
    }

    #[test]
    fn leveled_position_search_looks_at_every_extra_list() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        let (first, second) = (leveled_extra(&mut e, 1, 3), leveled_extra(&mut e, 1, 8));
        let change = entry(&mut e, form_a, &[first, second], 2);
        put_entry(&mut e, this, change);
        assert!(e.call(0x004d1380, &args![this, 8i32]).bool());
        assert!(e.call(0x004d1380, &args![this, 3i32]).bool());
        assert!(!e.call(0x004d1380, &args![this, 4i32]).bool());
    }

    #[test]
    fn leveled_items_of_the_container_are_expanded_once() {
        let mut e = fx();
        e.register(REFERENCE_GET_CALC_LEVEL, |_, _| returns(7));
        e.register(LEVELED_ITEM_CHANCE, |e, a| {
            returns_float(e.mem.f32(a[0] + 8))
        });
        for address in [
            CONTAINER_TEMP_CONSTRUCT,
            CONTAINER_TEMP_DESTRUCT,
            CONTAINER_SCALE_COUNTS,
            CONTAINER_ADD_TO_CHANGES,
        ] {
            stub(&mut e, address);
        }
        e.register(LEVELED_ITEM_FILL_CONTAINER, |_, a| returns(a[2]));
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (first, second, plain) = (
            big_form(&mut e, 0x34, 0),
            big_form(&mut e, 0x34, 0),
            big_form(&mut e, 0x20, 0),
        );
        // The chance object is read at +8: 0.5 for the first, 1.0 for the second.
        let chance_a = e.mem.alloc(0x10);
        set_float(&mut e, chance_a, 8, 0.5);
        let chance_b = e.mem.alloc(0x10);
        set_float(&mut e, chance_b, 8, 1.0);
        let records: Vec<u32> = [
            (-3i32, first, chance_a),
            (1, plain, 0),
            (2, second, chance_b),
        ]
        .iter()
        .map(|(count, form, chance)| {
            let record = e.mem.alloc(0x10);
            e.mem.set_u32(record, *count as u32);
            e.mem.set_u32(record + 4, *form);
            e.mem.set_u32(record + 8, *chance);
            record
        })
        .collect();
        let head = list(&mut e, &records);
        e.mem.set_u32(owner + 0x18, head);
        // The leveled object number 1 is already in the changes.
        let form_a = big_form(&mut e, 0x20, 1);
        let seen = leveled_extra(&mut e, 1, 1);
        let change = entry(&mut e, form_a, &[seen], 1);
        put_entry(&mut e, this, change);
        e.call_log = Some(vec![]);
        e.call(0x004d1440, &args![this]);
        // |count| 3 and the owner's level 7 fill the temporary container.
        let fill = calls(&e, LEVELED_ITEM_FILL_CONTAINER);
        assert_eq!(fill.len(), 1);
        assert_eq!((fill[0][0], fill[0][1], fill[0][2]), (first + 0x30, 7, 3));
        assert_eq!(fill[0][4], 0);
        // The chance 0.5 scales it; index 0 is added to the changes.
        let scale = calls(&e, CONTAINER_SCALE_COUNTS);
        assert_eq!(scale, vec![vec![fill[0][3], 0.5f32.to_bits()]]);
        assert_eq!(
            calls(&e, CONTAINER_ADD_TO_CHANGES),
            vec![vec![fill[0][3], 0, this.addr()]]
        );
        assert_eq!(calls(&e, CONTAINER_TEMP_DESTRUCT).len(), 1);
    }

    #[test]
    fn container_objects_become_entries_with_their_own_extra_list() {
        let mut e = fx();
        e.register(GET_FORM_AS_HEALTH_FORM, |_, a| returns(a[0] + 0x100));
        e.register(CONTAINER_OBJECT_ATTACH_EXTRA, |_, _| Ret::default());
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (plain, leveled, bare) = (
            big_form(&mut e, 0x20, 0),
            big_form(&mut e, 0x34, 0),
            big_form(&mut e, 0x21, 0),
        );
        let records: Vec<u32> = [(2u32, plain, 0x99u32), (5, leveled, 0x98), (4, bare, 0)]
            .iter()
            .map(|(count, form, third)| {
                let record = e.mem.alloc(0x10);
                e.mem.set_u32(record, *count);
                e.mem.set_u32(record + 4, *form);
                e.mem.set_u32(record + 8, *third);
                record
            })
            .collect();
        let head = list(&mut e, &records);
        e.mem.set_u32(owner + 0x18, head);
        e.call(0x004d1610, &args![this]);
        let attach = calls(&e, CONTAINER_OBJECT_ATTACH_EXTRA);
        assert_eq!(attach.len(), 1);
        assert_eq!((attach[0][0], attach[0][2]), (0x99, plain + 0x100));
        let created = inventory_list(&e, this);
        assert_eq!(created.len(), 1);
        let entry_form = e.mem.u32(created[0] + 8);
        assert_eq!(entry_form, plain);
        let lists = items(&e, e.mem.u32(created[0]));
        assert_eq!(lists, vec![attach[0][1]]);
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 2);
    }

    #[test]
    fn leveled_extra_lists_are_taken_out_of_the_changes() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        let form_b = big_form(&mut e, 0x20, 1);
        let (leveled_a, kept) = (leveled_extra(&mut e, 3, 0), extra(&mut e));
        let first = entry(&mut e, form_a, &[leveled_a, kept], 4);
        let leveled_b = leveled_extra(&mut e, 2, 1);
        let second = entry(&mut e, form_b, &[leveled_b], 2);
        put_entry(&mut e, this, first);
        put_entry(&mut e, this, second);
        e.call(0x004d17a0, &args![this]);
        assert_eq!(
            calls(&e, vcall_address(0x48)),
            vec![vec![owner, 0x0800_0000]]
        );
        // The first entry keeps its plain list and 4 - 3 items.
        assert_eq!(items(&e, e.mem.u32(first)), vec![kept]);
        assert_eq!(e.mem.u32(first + 4), 1);
        assert!(calls(&e, DESTRUCTOR).contains(&vec![leveled_a, 1]));
        // The second is left empty with number 0: it goes away.
        assert_eq!(inventory_list(&e, this), vec![first]);
        assert!(calls(&e, ITEM_CHANGE_DELETE).contains(&vec![second, 1]));
        assert!(calls(&e, DESTRUCTOR).contains(&vec![leveled_b, 1]));
    }

    /// Doubles for the scratch reference `fn_004d2480` works on.
    fn run_scripts_setup(e: &mut Engine) -> u32 {
        let temporary = e.mem.alloc(0x100);
        e.set_global(TEMP_REF_GLOBAL, temporary);
        for address in [
            SECTION_LOCK,
            SECTION_UNLOCK,
            REFERENCE_SET_PARENT_CELL,
            REFERENCE_STORE_OWNER_WORD,
            REFERENCE_SET_EXTRA,
            REFERENCE_SET_OBJECT_REFERENCE,
            EXTRA_REMOVE_ALL_COPYABLE,
            EXTRA_SET_SCRIPT_EVENTS,
        ] {
            stub(e, address);
        }
        e.register(REFERENCE_PARENT_CELL, |_, a| returns(a[0] + 0x30));
        e.register(OWNER_STORED_WORD, |_, a| returns(a[0] + 1));
        e.register(SCRIPT_OBJECT_TEST, |_, _| returns(1));
        e.register(SCRIPT_RUN, |_, _| returns(1));
        temporary
    }

    #[test]
    fn run_scripts_runs_each_list_script_on_the_scratch_reference() {
        let mut e = fx();
        let temporary = run_scripts_setup(&mut e);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (script_a, script_b) = (big_form(&mut e, 0x11, 0), big_form(&mut e, 0x11, 0));
        let not_script = big_form(&mut e, 0x20, 0);
        let (x, y, z) = (extra(&mut e), extra(&mut e), extra(&mut e));
        set(&mut e, x, X_SCRIPT, script_a);
        set(&mut e, y, X_SCRIPT, script_b);
        set(&mut e, z, X_SCRIPT, not_script);
        let form_a = big_form(&mut e, 0x20, 0);
        let change = entry(&mut e, form_a, &[x, y, z], 1);
        put_entry(&mut e, this, change);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004d2480, &args![this, owner]).bool());
        // A script that is not of form type 0x11 is not run.
        assert_eq!(
            calls(&e, SCRIPT_RUN),
            vec![
                vec![script_a, temporary, 0, owner, 0],
                vec![script_b, temporary, 0, owner, 0]
            ]
        );
        assert_eq!(
            calls(&e, REFERENCE_SET_EXTRA),
            vec![vec![temporary, x], vec![temporary, y]]
        );
        assert_eq!(
            calls(&e, REFERENCE_SET_OBJECT_REFERENCE),
            vec![
                vec![temporary, form_a],
                vec![temporary, form_a],
                vec![temporary, 0]
            ]
        );
        assert_eq!(
            calls(&e, REFERENCE_SET_PARENT_CELL),
            vec![vec![temporary, owner + 0x30], vec![temporary, 0]]
        );
        assert_eq!(
            calls(&e, SECTION_LOCK),
            vec![vec![RUN_SCRIPTS_LOCK_OBJECT, 0]]
        );
        assert_eq!(
            calls(&e, SECTION_UNLOCK),
            vec![vec![RUN_SCRIPTS_LOCK_OBJECT]]
        );
        // Nothing is done without an owner.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004d2480, &args![this, 0u32]).bool());
        assert!(calls(&e, SECTION_LOCK).is_empty());
    }

    #[test]
    fn run_scripts_stops_the_lists_of_an_entry_when_a_script_marks_the_count_dirty() {
        let mut e = fx();
        run_scripts_setup(&mut e);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let dirty = this.addr() + 0x10;
        e.register_double(SCRIPT_RUN, move |e, _| {
            e.mem.set_u8(dirty, 1);
            returns(0)
        });
        let (script_a, script_b) = (big_form(&mut e, 0x11, 0), big_form(&mut e, 0x11, 0));
        let (x, y) = (extra(&mut e), extra(&mut e));
        set(&mut e, x, X_SCRIPT, script_a);
        set(&mut e, y, X_SCRIPT, script_b);
        let form_a = big_form(&mut e, 0x20, 0);
        let change = entry(&mut e, form_a, &[x, y], 1);
        put_entry(&mut e, this, change);
        e.call_log = Some(vec![]);
        // The second script is not run, the result is false (the run said
        // false) and the flag is cleared again.
        assert!(!e.call(0x004d2480, &args![this, owner]).bool());
        assert_eq!(calls(&e, SCRIPT_RUN).len(), 1);
        assert!(!e.get(this, InventoryChanges::bcountdirty));
    }

    #[test]
    fn container_objects_are_copied_into_the_target_changes() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target_owner = object(&mut e);
        let target = inventory(&mut e, &[], target_owner);
        let (form_a, leveled) = (big_form(&mut e, 0x20, 1), big_form(&mut e, 0x7e, 1));
        give_container(&mut e, owner, &[(3, form_a), (2, leveled)]);
        e.call_log = Some(vec![]);
        e.call(0x004d26d0, &args![this, target, 0xaa00u32, 0u32]);
        let copied = inventory_list(&e, target);
        assert_eq!(copied.len(), 1);
        assert_eq!(
            (e.mem.u32(copied[0] + 8), e.mem.u32(copied[0] + 4)),
            (form_a, 3)
        );
        let lists = items(&e, e.mem.u32(copied[0]));
        assert_eq!(lists.len(), 1);
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 3);
        assert_eq!(e.mem.u32(lists[0] + X_OWNER_SET), 0xaa00);
        // The leveled item's new item was only deleted.
        assert!(!calls(&e, ITEM_CHANGE_DELETE).is_empty());
        let guard = &calls(&e, SCOPE_GUARD_OPEN)[0];
        assert_eq!(&guard[1..], &[0x36, 1, SOURCE_FILE_NAME, 0x26b7]);
    }

    #[test]
    fn changes_entries_are_copied_into_the_target_changes() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target_owner = object(&mut e);
        let target = inventory(&mut e, &[], target_owner);
        give_container(&mut e, owner, &[]);
        // An entry without lists becomes one counted list.
        let form_d = big_form(&mut e, 0x20, 1);
        let bare = entry(&mut e, form_d, &[], 2);
        put_entry(&mut e, this, bare);
        // An entry with a list keeps it, owned by the new owner.
        let form_e = big_form(&mut e, 0x21, 1);
        let held = extra(&mut e);
        set(&mut e, held, X_COUNT, 4);
        let with_list = entry(&mut e, form_e, &[held], 4);
        put_entry(&mut e, this, with_list);
        // An entry with a number of 0 or less is left out.
        let form_f = big_form(&mut e, 0x22, 1);
        let negative = entry(&mut e, form_f, &[], -1);
        put_entry(&mut e, this, negative);
        e.call(0x004d26d0, &args![this, target, 0xbb00u32, 0u32]);
        let copied = inventory_list(&e, target);
        assert_eq!(copied.len(), 2);
        assert_eq!(
            (e.mem.u32(copied[0] + 8), e.mem.u32(copied[0] + 4)),
            (form_d, 2)
        );
        let lists = items(&e, e.mem.u32(copied[0]));
        assert_eq!(
            (
                e.mem.u32(lists[0] + X_COUNT),
                e.mem.u32(lists[0] + X_OWNER_SET)
            ),
            (2, 0xbb00)
        );
        assert_eq!(
            (e.mem.u32(copied[1] + 8), e.mem.u32(copied[1] + 4)),
            (form_e, 4)
        );
        assert_eq!(items(&e, e.mem.u32(copied[1])), vec![held]);
        assert_eq!(e.mem.u32(held + X_OWNER_SET), 0xbb00);
    }

    #[test]
    fn clearing_the_changes_deletes_lists_without_a_container_and_unlisted_ones() {
        let mut e = fx();
        e.register(REFR_HAS_CONTAINER, |e, a| returns(e.mem.u32(a[0] + 0x30)));
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (no_container, with_container) = (e.mem.alloc(0x100), e.mem.alloc(0x100));
        e.mem.set_u32(with_container + 0x30, 1);
        let (x1, x2, y) = (extra(&mut e), extra(&mut e), extra(&mut e));
        set(&mut e, x1, X_ORIGINAL, no_container);
        set(&mut e, y, X_ORIGINAL, with_container);
        set(&mut e, y, X_ITEMS_IN_LIST, 1);
        set(&mut e, y, X_COUNT, 1);
        let (form_a, form_b) = (big_form(&mut e, 0x20, 1), big_form(&mut e, 0x21, 1));
        let first = entry(&mut e, form_a, &[x1, x2], 2);
        let second = entry(&mut e, form_b, &[y], 1);
        put_entry(&mut e, this, first);
        put_entry(&mut e, this, second);
        e.call_log = Some(vec![]);
        e.call(0x004d3660, &args![this]);
        // Both entries are gone; the lists with an original reference are
        // deleted (the one without is not).
        assert_eq!(inventory_list(&e, this), Vec::<u32>::new());
        let deleted = calls(&e, DESTRUCTOR);
        assert!(deleted.contains(&vec![x1, 1]));
        assert!(deleted.contains(&vec![y, 1]));
        assert!(!deleted.contains(&vec![x2, 1]));
        assert_eq!(
            calls(&e, ITEM_CHANGE_DELETE),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(calls(&e, EXTRA_REMOVE_COUNT), vec![vec![y]]);
    }

    #[test]
    fn clearing_keeps_a_list_the_references_own_changes_still_hold() {
        let mut e = fx();
        e.register(REFR_HAS_CONTAINER, |e, a| returns(e.mem.u32(a[0] + 0x30)));
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let reference = e.mem.alloc(0x100);
        e.mem.set_u32(reference + 0x30, 1);
        let y = extra(&mut e);
        set(&mut e, y, X_ORIGINAL, reference);
        let form_a = big_form(&mut e, 0x20, 1);
        let change = entry(&mut e, form_a, &[y], 1);
        put_entry(&mut e, this, change);
        // The reference's changes (kept at +0x1c of its extra list, +0x40)
        // hold an entry for the form with the list y.
        let other_owner = object(&mut e);
        let other = inventory(&mut e, &[], other_owner);
        let kept = entry(&mut e, form_a, &[y], 1);
        put_entry(&mut e, other, kept);
        e.mem.set_u32(reference + 0x40 + 0x1c, other.addr());
        e.call_log = Some(vec![]);
        e.call(0x004d3660, &args![this]);
        assert!(!calls(&e, DESTRUCTOR).contains(&vec![y, 1]));
        assert_eq!(inventory_list(&e, this), Vec::<u32>::new());
    }

    #[test]
    fn save_size_adds_the_entries_and_logs_it_when_asked() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        let listed = extra(&mut e);
        set(&mut e, listed, X_SAVE_SIZE, 30);
        let change = entry(&mut e, form_a, &[listed], 2);
        put_entry(&mut e, this, change);
        assert_eq!(e.call(0x004d3960, &args![this]).u16(), 2 + 12 + 30);
        e.mem.set_u32(SAVE_OBJECT + 0x20, 1);
        assert_eq!(e.call(0x004d3960, &args![this]).u16(), 6 + 2 + 6 + 12 + 30);
        assert!(calls(&e, ERROR_LOG).is_empty());
        e.mem.set_u8(LOG_FLAG, 1);
        e.call(0x004d3960, &args![this]);
        assert_eq!(
            calls(&e, ERROR_LOG).last().unwrap(),
            &vec![0x0101_2c78, 56, 0x284e, SOURCE_FILE_NAME]
        );
    }

    #[test]
    fn save_writes_the_entries_in_a_length_prefixed_block_and_load_reads_them_back() {
        let mut e = fx();
        e.set_global(PLAYER_GLOBAL, PLAYER);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        e.mem.set_u32(SAVE_OBJECT + 0x20, 1);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        e.mem.set_u32(form_a + 0xc, 0x1234);
        let change = entry(&mut e, form_a, &[], 2);
        put_entry(&mut e, this, change);
        e.call(0x004d3ab0, &args![this]);
        assert_eq!(e.mem.u32(STREAM), 0x424c_4f4b);
        // The block length (the bytes after the tag) and the entry count.
        assert_eq!(e.mem.u16(STREAM + 4), 22);
        assert_eq!(e.mem.u16(STREAM + 6), 1);
        // The entry: its own block, form id, number and list count.
        assert_eq!(e.mem.u32(STREAM + 8), 0x424c_4f4b);
        assert_eq!(e.mem.u16(STREAM + 12), 14);
        assert_eq!(e.mem.u32(STREAM + 14), 0x1234);
        assert_eq!(e.mem.u32(STREAM + 18), 2);
        assert_eq!(e.mem.u32(STREAM + 22), 0);
        assert_eq!(e.mem.u32(SAVE_OBJECT + 0x14), STREAM + 26);
        // Reading it back into another inventory.
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        e.mem.set_u32(LOOKUP_CELL, form_a);
        e.register(RT_DYNAMIC_CAST, |_, a| {
            returns(if a[3] == TYPE_TES_BOUND_OBJECT {
                a[0]
            } else {
                0
            })
        });
        let other_owner = object(&mut e);
        let loaded = inventory(&mut e, &[], other_owner);
        e.call(0x004d3cc0, &args![loaded]);
        let entries = inventory_list(&e, loaded);
        assert_eq!(entries.len(), 1);
        assert_eq!(
            (e.mem.u32(entries[0] + 8), e.mem.u32(entries[0] + 4)),
            (form_a, 2)
        );
        assert_eq!(e.mem.u32(SAVE_OBJECT + 0x14), STREAM + 26);
        assert!(calls(&e, SAVE_LOAD_LOG).is_empty());
    }

    #[test]
    fn save_logs_the_bytes_and_a_wrong_load_tag_and_length() {
        let mut e = fx();
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        e.mem.set_u8(LOG_FLAG, 1);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.call(0x004d3ab0, &args![this]);
        // Without save game blocks only the 2 byte entry count was written.
        assert_eq!(
            calls(&e, ERROR_LOG).last().unwrap(),
            &vec![0x0101_536c, 2, 0x286a, SOURCE_FILE_NAME]
        );
        // Loading with blocks: a wrong tag is logged and a length that is too
        // long is reported as an underrun.
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        e.mem.set_u32(SAVE_OBJECT + 0x20, 1);
        e.mem.set_u8(SAVE_OBJECT + 0x80, 9);
        e.mem.set_u32(STREAM, 0x1111_1111);
        e.mem.set_u16(STREAM + 4, 100);
        e.mem.set_u16(STREAM + 6, 0);
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        let loaded = inventory(&mut e, &[], owner);
        e.call_log = Some(vec![]);
        e.call(0x004d3cc0, &args![loaded]);
        let logged = calls(&e, SAVE_LOAD_LOG);
        assert_eq!(logged[0], vec![0x0101_56a8, SOURCE_FILE_NAME, 0x2871, 9]);
        // Read 8 bytes of a block of 100 + 4.
        assert_eq!(
            logged[1],
            vec![
                0x0101_5440,
                100 + 4 + STREAM - (STREAM + 8),
                SOURCE_FILE_NAME,
                0x2884,
                9
            ]
        );
    }

    #[test]
    fn load_deletes_entries_whose_form_is_missing() {
        let mut e = fx();
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        // No blocks: an entry count of 1, then the item: form id, number,
        // list count 0.
        e.mem.set_u16(STREAM, 1);
        e.mem.set_u32(STREAM + 2, 0x77);
        e.mem.set_u32(STREAM + 6, 5);
        e.mem.set_u32(STREAM + 10, 0);
        e.mem.set_u32(LOOKUP_CELL, 0);
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        e.call_log = Some(vec![]);
        e.call(0x004d3cc0, &args![this]);
        assert_eq!(inventory_list(&e, this), Vec::<u32>::new());
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE).len(), 1);
    }

    #[test]
    fn after_load_step_runs_on_every_entry() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (form_a, form_b) = (big_form(&mut e, 0x20, 1), big_form(&mut e, 0x21, 1));
        let (x, y) = (extra(&mut e), extra(&mut e));
        let (first, second) = (
            entry(&mut e, form_a, &[x], 1),
            entry(&mut e, form_b, &[y], 1),
        );
        put_entry(&mut e, this, first);
        put_entry(&mut e, this, second);
        e.call(0x004d4030, &args![this]);
        assert_eq!(
            calls(&e, EXTRA_AFTER_LOAD_GAME),
            vec![vec![x, 0, 0, 0, form_a], vec![y, 0, 0, 0, form_b]]
        );
    }

    /// Doubles for the save/load buffer words: the fields at +0x17, +0x20,
    /// +0x24 and +0x2c of the buffer object.
    fn buffer_setup(e: &mut Engine) -> u32 {
        let buffer = object(e);
        e.mem.set_u32(buffer + 0x17, 0x33);
        e.mem.set_u32(buffer + 0x2c, 0x44);
        e.register(BUFFER_WORD_17_GET, |e, a| {
            let value = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], value);
            returns(a[1])
        });
        e.register(BUFFER_WORD_2C_GET, |e, a| {
            let value = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[1], value);
            returns(a[1])
        });
        for (address, offset) in [
            (BUFFER_WORD_17_SET, 0x17),
            (BUFFER_WORD_20_SET, 0x20),
            (BUFFER_WORD_24_SET, 0x24),
            (BUFFER_WORD_2C_SET, 0x2c),
        ] {
            e.register_double(address, move |e, a| {
                e.mem.set_u32(a[0] + offset, a[1]);
                Ret::default()
            });
        }
        e.register(vcall_address(0), |_, _| returns(0x11));
        e.register(vcall_address(4), |_, _| returns(0x22));
        e.register(SAVE_BUFFER_START_SIZED, |_, _| returns(0x55));
        e.register(LOAD_BUFFER_LOAD_DATA, |_, _| Ret::default());
        buffer
    }

    #[test]
    fn buffer_save_writes_a_counted_list_with_the_mode_words_switched() {
        let mut e = fx();
        let buffer = buffer_setup(&mut e);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (form_a, form_b) = (big_form(&mut e, 0x20, 1), big_form(&mut e, 0x21, 1));
        let (x, y) = (extra(&mut e), extra(&mut e));
        let (first, second) = (
            entry(&mut e, form_a, &[x], 1),
            entry(&mut e, form_b, &[y], 3),
        );
        put_entry(&mut e, this, first);
        put_entry(&mut e, this, second);
        e.call_log = Some(vec![]);
        e.call(0x004d4090, &args![this, buffer]);
        assert_eq!(
            calls(&e, SAVE_BUFFER_SAVE_FORM_ID),
            vec![vec![buffer, form_a, 0], vec![buffer, form_b, 0]]
        );
        assert_eq!(
            calls(&e, EXTRA_SAVE_GAME_BUFFER),
            vec![vec![x, buffer], vec![y, buffer]]
        );
        assert_eq!(
            calls(&e, SAVE_BUFFER_END_SIZED).last().unwrap(),
            &vec![buffer, 2, 0x55]
        );
        // The mode word is set to 0 and the kind word to 0x400, then both
        // are put back.
        assert_eq!(
            calls(&e, BUFFER_WORD_20_SET),
            vec![vec![buffer, 0], vec![buffer, 0x11]]
        );
        assert_eq!(
            calls(&e, BUFFER_WORD_17_SET),
            vec![vec![buffer, 0x400], vec![buffer, 0x33]]
        );
        assert_eq!(e.mem.u32(buffer + 0x17), 0x33);
    }

    #[test]
    fn buffer_load_reads_the_entries_and_adds_even_a_failed_one() {
        let mut e = fx();
        let buffer = buffer_setup(&mut e);
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        e.register(LOAD_BUFFER_LOAD_FORM_ID, |_, _| returns(1));
        // The first value is the entry count, the next ones the list counts.
        let first = std::cell::Cell::new(true);
        e.register_double(LOAD_BUFFER_LOAD_SIZED, move |_, _| {
            returns(if first.replace(false) { 2 } else { 0 })
        });
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        e.mem.set_u32(LOOKUP_CELL, form_a);
        e.call_log = Some(vec![]);
        e.call(0x004d4160, &args![this, buffer]);
        let loaded = inventory_list(&e, this);
        assert_eq!(loaded.len(), 2);
        assert_eq!(e.mem.u32(loaded[0] + 8), form_a);
        assert_eq!(
            calls(&e, BUFFER_WORD_24_SET),
            vec![vec![buffer, 0], vec![buffer, 0x22]]
        );
        assert_eq!(
            calls(&e, BUFFER_WORD_2C_SET),
            vec![vec![buffer, 0], vec![buffer, 0x44]]
        );
        // A form that cannot be found: the item is deleted and a null item
        // is added all the same.
        let other_owner = object(&mut e);
        let again = inventory(&mut e, &[], other_owner);
        let once = std::cell::Cell::new(true);
        e.register_double(LOAD_BUFFER_LOAD_SIZED, move |_, _| {
            returns(if once.replace(false) { 1 } else { 0 })
        });
        e.mem.set_u32(LOOKUP_CELL, 0);
        e.call_log = Some(vec![]);
        e.call(0x004d4160, &args![again, buffer]);
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE).len(), 1);
        assert_eq!(inventory_list(&e, again), Vec::<u32>::new());
    }

    #[test]
    fn buffer_after_load_step_runs_on_every_list() {
        let mut e = fx();
        let buffer = buffer_setup(&mut e);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let form_a = big_form(&mut e, 0x20, 1);
        let x = extra(&mut e);
        let first = entry(&mut e, form_a, &[x], 1);
        put_entry(&mut e, this, first);
        e.call_log = Some(vec![]);
        e.call(0x004d42f0, &args![this, buffer]);
        assert_eq!(
            calls(&e, EXTRA_AFTER_LOAD_GAME_BUFFER),
            vec![vec![x, buffer, form_a]]
        );
        assert_eq!(calls(&e, BUFFER_WORD_24_SET).len(), 2);
        assert_eq!(e.mem.u32(buffer + 0x24), 0x22);
    }

    #[test]
    fn extra_list_position_is_found_and_read_back() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (form_a, form_b) = (big_form(&mut e, 0x20, 1), big_form(&mut e, 0x21, 1));
        let (x, y, z) = (extra(&mut e), extra(&mut e), extra(&mut e));
        let (first, second) = (
            entry(&mut e, form_a, &[x, y], 1),
            entry(&mut e, form_b, &[z], 1),
        );
        put_entry(&mut e, this, first);
        put_entry(&mut e, this, second);
        assert_eq!(e.call(0x004d43c0, &args![this, form_a, y]).i32(), 1);
        assert_eq!(e.call(0x004d43c0, &args![this, form_b, z]).i32(), 0);
        // A list of another entry, an unknown form or null arguments: -1.
        assert_eq!(e.call(0x004d43c0, &args![this, form_a, z]).i32(), -1);
        assert_eq!(e.call(0x004d43c0, &args![this, 0u32, z]).i32(), -1);
        assert_eq!(e.call(0x004d43c0, &args![this, form_b, 0u32]).i32(), -1);
        assert_eq!(e.call(0x004d4480, &args![this, form_a, 1i32]).u32(), y);
        assert_eq!(e.call(0x004d4480, &args![this, form_a, 0i32]).u32(), x);
        assert_eq!(e.call(0x004d4480, &args![this, form_a, 2i32]).u32(), 0);
        assert_eq!(e.call(0x004d4480, &args![this, form_a, -1i32]).u32(), 0);
        assert_eq!(e.call(0x004d4480, &args![this, 0u32, 0i32]).u32(), 0);
    }

    const CALLBACK: u32 = 0x0300_0000;

    /// What the callback of the item walks saw: `(lists, number, form, user)`.
    type Offered = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, u32, u32)>>>;

    /// Registers a callback that records the offered stacks and stops after
    /// `stop_after` calls.
    fn recording_callback(e: &mut Engine, stop_after: usize) -> Offered {
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(CALLBACK, move |e, a| {
            log.borrow_mut().push((
                e.mem.u32(a[0]),
                e.mem.u32(a[0] + 4),
                e.mem.u32(a[0] + 8),
                a[1],
            ));
            let stop = log.borrow().len() >= stop_after;
            returns(stop as u32)
        });
        seen
    }

    #[test]
    fn each_item_stack_is_offered_to_the_callback() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (held, excluded, loose) = (
            big_form(&mut e, 0x20, 1),
            big_form(&mut e, 0x21, 1),
            big_form(&mut e, 0x22, 1),
        );
        e.mem.set_u32(excluded + O_EXCLUDED, 1);
        give_container(&mut e, owner, &[(2, held), (6, excluded)]);
        let x = extra(&mut e);
        let change = entry(&mut e, loose, &[x], 3);
        put_entry(&mut e, this, change);
        let lists = e.mem.u32(change);
        let seen = recording_callback(&mut e, 99);
        assert_eq!(
            e.call(0x004d4530, &args![this, CALLBACK, 0x77u32, 0u32])
                .i32(),
            3
        );
        assert_eq!(
            *seen.borrow(),
            vec![
                (0, 2, held, 0x77),
                (0, 6, excluded, 0x77),
                (lists, 3, loose, 0x77)
            ]
        );
        // With the filter flag the excluded form is skipped.
        let seen = recording_callback(&mut e, 99);
        assert_eq!(
            e.call(0x004d4530, &args![this, CALLBACK, 0x78u32, 1u32])
                .i32(),
            2
        );
        assert_eq!(seen.borrow().len(), 2);
        // The callback stops the walk by returning true.
        let seen = recording_callback(&mut e, 1);
        assert_eq!(
            e.call(0x004d4530, &args![this, CALLBACK, 0x79u32, 0u32])
                .i32(),
            1
        );
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn the_resumable_iteration_gives_one_stack_per_call_and_then_stops() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (held, excluded, loose) = (
            big_form(&mut e, 0x20, 1),
            big_form(&mut e, 0x21, 1),
            big_form(&mut e, 0x22, 1),
        );
        e.mem.set_u32(excluded + O_EXCLUDED, 1);
        give_container(&mut e, owner, &[(2, held), (6, excluded)]);
        let x = extra(&mut e);
        let change = entry(&mut e, loose, &[x], 3);
        put_entry(&mut e, this, change);
        let lists = e.mem.u32(change);
        let iterator = e.mem.alloc(0x20);
        let out = e.mem.alloc(8);
        let next = |e: &mut Engine, flags: u32| {
            e.call(0x004d4830, &args![this, iterator, out, flags])
                .bool()
        };
        assert!(next(&mut e, 1));
        assert_eq!(e.mem.u32(out), iterator);
        assert_eq!(
            (
                e.mem.u32(iterator),
                e.mem.u32(iterator + 4),
                e.mem.u32(iterator + 8)
            ),
            (0, 2, held)
        );
        // The excluded form is skipped with the filter flag; the changes
        // entry follows with its lists shared.
        assert!(next(&mut e, 1));
        assert_eq!(
            (
                e.mem.u32(iterator),
                e.mem.u32(iterator + 4),
                e.mem.u32(iterator + 8)
            ),
            (lists, 3, loose)
        );
        assert_eq!(e.mem.u8(iterator + 0x14), 1);
        assert!(!next(&mut e, 1));
    }

    #[test]
    fn item_groups_collect_the_matching_stacks_and_delete_the_others() {
        let mut e = fx();
        let (this, held, loose) = stack_setup(&mut e);
        let group = list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x004d4b00, &args![this, loose, group]);
        let collected = items(&e, group);
        assert_eq!(collected.len(), 1);
        assert_eq!(
            (e.mem.u32(collected[0] + 8), e.mem.u32(collected[0] + 4)),
            (loose, 3)
        );
        let deleted = calls(&e, ITEM_CHANGE_DELETE);
        assert_eq!(deleted.len(), 1);
        assert_ne!(deleted[0][0], collected[0]);
        assert_eq!(calls(&e, FAST_ITERATOR_DESTRUCT).len(), 1);
        // Nothing is done without a form or a list.
        e.call_log = Some(vec![]);
        e.call(0x004d4b00, &args![this, 0u32, group]);
        e.call(0x004d4b00, &args![this, held, 0u32]);
        assert!(e.call_log.as_ref().unwrap().len() == 2);
    }

    #[test]
    fn repair_item_group_keeps_the_stacks_the_test_accepts() {
        let mut e = fx();
        e.register(REPAIR_ITEM_TEST, |e, a| {
            returns((e.mem.u8(a[0] + F_TYPE) == 0x20 && a[1] == 0x99) as u32)
        });
        let (this, _, loose) = stack_setup(&mut e);
        let group = list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x004d4bd0, &args![this, 0x99u32, group]);
        let collected = items(&e, group);
        assert_eq!(collected.len(), 1);
        assert_eq!(e.mem.u32(collected[0] + 8), loose);
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE).len(), 1);
    }

    #[test]
    fn mod_group_collects_the_mod_objects_of_the_weapon() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let weapon = big_form(&mut e, 0x28, 1);
        let (mod_a, mod_b, other_mod, plain) = (
            big_form(&mut e, 0x67, 1),
            big_form(&mut e, 0x67, 1),
            big_form(&mut e, 0x67, 1),
            big_form(&mut e, 0x20, 1),
        );
        set(&mut e, weapon, WEAPON_MOD_OBJECT, mod_a);
        set(&mut e, weapon, WEAPON_MOD_OBJECT + 8, mod_b);
        give_container(
            &mut e,
            owner,
            &[(1, mod_a), (1, plain), (1, other_mod), (1, mod_b)],
        );
        let group = list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x004d4ca0, &args![this, weapon, group]);
        let forms: Vec<u32> = items(&e, group)
            .iter()
            .map(|item| e.mem.u32(item + 8))
            .collect();
        assert_eq!(forms, vec![mod_a, mod_b]);
        // Only the plain form's stack was deleted.
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE).len(), 1);
        // A weapon that is no weapon, or no list: nothing.
        e.call_log = Some(vec![]);
        e.call(0x004d4ca0, &args![this, plain, group]);
        e.call(0x004d4ca0, &args![this, weapon, 0u32]);
        assert_eq!(items(&e, group).len(), 2);
        assert!(calls(&e, ITEM_CHANGE_DELETE).is_empty());
    }

    #[test]
    fn the_form_map_is_built_with_zeroed_buckets_and_torn_down() {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x1_0000);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| returns(e.mem.alloc(a[0])));
        for address in [NI_MAP_CLEAR, NI_FREE] {
            stub(&mut e, address);
        }
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d4de0, &args![map, 0x25u32]).u32(), map);
        assert_eq!(e.mem.u32(map), NI_MAP_VTABLE);
        assert_eq!((e.mem.u32(map + 4), e.mem.u32(map + 0xc)), (0x25, 0));
        let table = e.mem.u32(map + 8);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![table, 0, 0x94]]);
        // The base constructor leaves the base vtable.
        let base = e.mem.alloc(0x10);
        e.call(0x004d4e40, &args![base, 8u32]);
        assert_eq!(e.mem.u32(base), NI_MAP_BASE_VTABLE);
        // The destructors clear the map; the deleting ones free the map.
        e.call_log = Some(vec![]);
        e.call(0x004d4eb0, &args![map]);
        assert_eq!(calls(&e, NI_MAP_CLEAR), vec![vec![map], vec![map]]);
        assert_eq!(calls(&e, NI_FREE), vec![vec![table]]);
        assert_eq!(e.mem.u32(map), NI_MAP_BASE_VTABLE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d4e10, &args![map, 0u32]).u32(), map);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x004d4e10, &args![map, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![map]]);
        e.call_log = Some(vec![]);
        e.call(0x004d4f10, &args![base]);
        assert_eq!(calls(&e, NI_MAP_CLEAR), vec![vec![base]]);
        e.call(0x004d4f40, &args![base, 1u32]);
        assert_eq!(calls(&e, NI_MAP_CLEAR).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![base]]);
    }

    #[test]
    fn empty_item_constructor_zeroes_the_three_words() {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x1000);
        let item = e.mem.alloc(0xc);
        for offset in [0, 4, 8] {
            e.mem.set_u32(item + offset, 0xdead);
        }
        assert_eq!(e.call(0x0076b630, &args![item]).u32(), item);
        assert_eq!(
            (e.mem.u32(item), e.mem.u32(item + 4), e.mem.u32(item + 8)),
            (0, 0, 0)
        );
    }

    #[test]
    fn stacks_of_script_forms_get_their_scripts_run() {
        let mut e = fx();
        e.register(REFR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], BIG_VTABLE);
            returns(a[0])
        });
        e.register(EXTRA_SET_SCRIPT, |e, a| {
            e.mem.set_u32(a[0] + X_SCRIPT, a[1]);
            Ret::default()
        });
        e.register(SCRIPT_MAKE_EVENT_LIST, |_, _| returns(0));
        stub(&mut e, EXTRA_SET_SCRIPT_EVENTS);
        stub(&mut e, SCRIPT_RUN);
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let (scripted, plain) = (big_form(&mut e, 0x20, 1), big_form(&mut e, 0x21, 1));
        e.mem.set_u32(scripted + 0x44, 0x777);
        give_container(&mut e, owner, &[(2, scripted), (3, plain)]);
        e.call_log = Some(vec![]);
        e.call(0x004d1960, &args![this]);
        // Two new lists (count 1) with the script, each run once.
        let runs = calls(&e, SCRIPT_RUN);
        assert_eq!(runs.len(), 2);
        assert!(runs
            .iter()
            .all(|run| run[0] == 0x777 && run[2..] == [0, 0, 0]));
        let created = inventory_list(&e, this);
        assert_eq!(created.len(), 1);
        assert_eq!(e.mem.u32(created[0] + 8), scripted);
        let lists = items(&e, e.mem.u32(created[0]));
        assert_eq!(lists.len(), 2);
        assert!(lists.iter().all(|list| e.mem.u32(list + X_SCRIPT) == 0x777));
        assert!(lists.iter().all(|list| e.mem.u32(list + X_COUNT) == 1));
        // Now the entry has a script: it is only run again on each list.
        e.call_log = Some(vec![]);
        e.call(0x004d1960, &args![this]);
        assert_eq!(calls(&e, SCRIPT_RUN).len(), 2);
        // An entry with a number but a list without a script: the list gets
        // it and the missing lists are made.
        let other = big_form(&mut e, 0x22, 1);
        e.mem.set_u32(other + 0x44, 0x888);
        let bare = extra(&mut e);
        let change = entry(&mut e, other, &[bare], 3);
        put_entry(&mut e, this, change);
        e.call_log = Some(vec![]);
        e.call(0x004d1960, &args![this]);
        assert_eq!(e.mem.u32(bare + X_SCRIPT), 0x888);
        assert_eq!(items(&e, e.mem.u32(change)).len(), 3);
        let runs = calls(&e, SCRIPT_RUN);
        assert_eq!(runs.iter().filter(|run| run[0] == 0x888).count(), 3);
    }

    #[test]
    fn copying_a_container_object_with_an_entry_adds_the_numbers_and_honours_the_filters() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target_owner = object(&mut e);
        let target = inventory(&mut e, &[], target_owner);
        let (form_a, form_b, form_c) = (
            big_form(&mut e, 0x20, 1),
            big_form(&mut e, 0x21, 1),
            big_form(&mut e, 0x22, 1),
        );
        // form_c is excluded (virtual 0x94).
        e.mem.set_u32(form_c + O_EXCLUDED, 1);
        give_container(&mut e, owner, &[(2, form_a), (2, form_b), (1, form_c)]);
        // 2 + 3: five of form_a; 2 - 2: nothing of form_b; form_c has an entry.
        let (entry_a, entry_b, entry_c) = (
            entry(&mut e, form_a, &[], 3),
            entry(&mut e, form_b, &[], -2),
            entry(&mut e, form_c, &[], 1),
        );
        for change in [entry_a, entry_b, entry_c] {
            put_entry(&mut e, this, change);
        }
        // Excluded forms are only left out when asked to.
        e.call(0x004d26d0, &args![this, target, 0xaa00u32, 1u32]);
        let copied = inventory_list(&e, target);
        assert_eq!(copied.len(), 1);
        assert_eq!(
            (e.mem.u32(copied[0] + 8), e.mem.u32(copied[0] + 4)),
            (form_a, 5)
        );
        let lists = items(&e, e.mem.u32(copied[0]));
        assert_eq!(e.mem.u32(lists[0] + X_COUNT), 5);
        let other_owner = object(&mut e);
        let again = inventory(&mut e, &[], other_owner);
        e.call(0x004d26d0, &args![this, again, 0xaa00u32, 0u32]);
        let forms: Vec<u32> = inventory_list(&e, again)
            .iter()
            .map(|change| e.mem.u32(change + 8))
            .collect();
        assert_eq!(forms, vec![form_a, form_c]);
    }

    #[test]
    fn copying_an_entry_with_a_script_keeps_its_lists() {
        let mut e = fx();
        let owner = object(&mut e);
        let this = inventory(&mut e, &[], owner);
        let target_owner = object(&mut e);
        let target = inventory(&mut e, &[], target_owner);
        let form_a = big_form(&mut e, 0x20, 1);
        give_container(&mut e, owner, &[(2, form_a)]);
        let scripted = extra(&mut e);
        set(&mut e, scripted, X_SCRIPT, 0x777);
        let change = entry(&mut e, form_a, &[scripted], 0);
        put_entry(&mut e, this, change);
        e.call(0x004d26d0, &args![this, target, 0xcc00u32, 0u32]);
        let copied = inventory_list(&e, target);
        assert_eq!(copied.len(), 1);
        assert_eq!(e.mem.u32(copied[0] + 4), 2);
        assert_eq!(items(&e, e.mem.u32(copied[0])), vec![scripted]);
        assert_eq!(e.mem.u32(scripted + X_OWNER_SET), 0xcc00);
    }
}
