//! `fallout shared/extradataobjects.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the constructors, destructors and `Compare` methods of the
//! `BSExtraData` subclasses that hang on a reference's `ExtraDataList`
//! (`ExtraAnim`, `ExtraDismemberedLimbs`, `ExtraStartingPosition`,
//! `ExtraLight`, `ExtraLock`, `ExtraFollower`, `ExtraGuardedRefData`,
//! `ExtraTeleport`, ...), in address order, together with `REFR_LOCK`.
//!
//! Translated so far: the first 200 functions of the queue (`004300f0` to
//! `004353d0`), which is the whole address range `00000000` to `004353f0` of
//! this file; the part files and the main file continue from `004353f0`.
//!
//! Notes for the next session:
//! - The layouts of the simple one-field classes (`ExtraRank`, `ExtraCount`,
//!   `ExtraUses`, `ExtraHealth`, ...) are declared below with the Xbox PDB
//!   names; `compare_prologue` is the cast-and-base-compare start every
//!   `Compare` shares and `destroy_owner` the destructor body of the classes
//!   that own one object through the pointer at +0xc.
//! - Every destructor and constructor here is compiled without inlining: the
//!   base `BSExtraData` constructor (`0040ec80`, takes the extra-data type
//!   byte) and destructor (`0040ecb0`) are separate calls, the vtable pointer
//!   is stored in between, and every `_scalar_deleting_destructor_` is the
//!   destructor followed by `operator delete` (`00401030`) when bit 0 of its
//!   flags is set.
//! - The compiler's exception-unwinding frames (`FS:[0]` chains, the
//!   `__CxxFrameHandler` state words) are not translated.
//! - Functions of this unit outside this file's address range that the ones
//!   below call by address: `00438570`, `004385a0` (the `DismemberedLimbs`
//!   array constructor/destructor), `00438660`, `00438690` (the `Guards`
//!   array constructor/destructor) and `00438470` (`BSStringT<char>::Set(other)`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList, BSStringT};

/// `operator new(size)` (cdecl, one stack argument).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)` (cdecl, one stack argument).
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSExtraData::BSExtraData(type)` (the base constructor; `this`, then the
/// extra-data type byte).
pub(crate) const BS_EXTRA_DATA_CONSTRUCT: u32 = 0x0040_ec80;
/// `BSExtraData::~BSExtraData` (the base destructor body).
pub(crate) const BS_EXTRA_DATA_DESTRUCT: u32 = 0x0040_ecb0;
/// `BSExtraData::Compare` (Xbox PDB; `this`, other): true when `other` is
/// null or `004f1540` gives a different answer for the two.
pub(crate) const BS_EXTRA_DATA_COMPARE: u32 = 0x0040_f700;
/// `BSSimpleArray<T,1024>::size` (the folded accessor: `this` is the array,
/// the count is returned in EAX).
pub(crate) const SIMPLE_ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `BSSimpleArray<T,1024>::operator[]` (`this` is the array, the index is
/// the argument): the address of the element slot.
pub(crate) const SIMPLE_ARRAY_AT: u32 = 0x006a_7ad0;
/// `BSSimpleArray<BGSBodyPart_P_1024>::AddUninitialized` (Xbox PDB name of
/// the folded body): appends the pointer stored at the address given and
/// returns the index of the new slot.
pub(crate) const SIMPLE_ARRAY_ADD: u32 = 0x007c_b2e0;
/// `BSSimpleArray<TESBoundObject_P_1024>::CompareBuffer<1024>` (Xbox PDB):
/// whether the `this` array holds the same elements as the one given.
pub(crate) const SIMPLE_ARRAY_COMPARE_BUFFER: u32 = 0x0043_87b0;
/// `BSSimpleArray<T,1024>::Find` of the folded body (`this` is the array;
/// the arguments are the address of the value, the first index to look at
/// and a comparison function; the index found, or -1).
pub(crate) const SIMPLE_ARRAY_FIND: u32 = 0x0071_9b20;
/// The comparison function `00719b20` is given by `ExtraGuardedRefData`.
pub(crate) const GUARD_COMPARE: u32 = 0x009a_3830;
/// The array's clear (`this` is the array, the argument says whether to free
/// the buffer); `CompareBuffer`'s caller uses it to drop an array that
/// duplicates an earlier one.
pub(crate) const SIMPLE_ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`.
pub(crate) const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `memcmp(a, b, size)`.
pub(crate) const MEMCMP: u32 = 0x00ec_4835;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The uniform form has no ST0 argument, so the value is passed as an
/// `f64` argument (two words, exact for every `float`).
pub(crate) const FTOL: u32 = 0x00ec_62c0;

/// `RTTI Type Descriptor` of `BSExtraData`, the source type of the
/// `dynamic_cast`s in the `Compare` methods.
pub(crate) const BS_EXTRA_DATA_TYPE: u32 = 0x0118_3b2c;
/// `RTTI Type Descriptor` of `ExtraStartingPosition`.
pub(crate) const EXTRA_STARTING_POSITION_TYPE: u32 = 0x0118_4b10;
/// `RTTI Type Descriptor` of `ExtraLock`.
pub(crate) const EXTRA_LOCK_TYPE: u32 = 0x0118_4754;

/// `0.0` (`double`), what `REFR_LOCK::IsBroken` compares the entry point's
/// result with.
pub(crate) const ZERO: u32 = 0x0101_2060;
/// The global holding the `PlayerCharacter` pointer (`PlayerCharacter::pSingleton`).
pub(crate) const PLAYER: u32 = 0x011d_ea3c;

/// `Actor::GetLevel` (returns the level in AX; `this` is the actor).
pub(crate) const ACTOR_GET_LEVEL: u32 = 0x0087_f9f0;
/// `TESObjectREFR::GetCalcLevel(bool)` (Xbox PDB name).
pub(crate) const REFR_GET_CALC_LEVEL: u32 = 0x0056_7e10;
/// A float game-setting accessor: `this` is the setting (an exe global), the
/// result is the address of its value.
pub(crate) const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// An integer game-setting accessor: `this` is the setting (an exe global),
/// the result is the address of its value.
pub(crate) const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// `BGSEntryPoint::HandleEntryPoint(entryPoint, actor, result, ...)` (Xbox
/// PDB name, cdecl).
pub(crate) const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The entry point `REFR_LOCK::IsBroken` asks (its result tells whether the
/// player's perks make a failed attempt count less).
pub(crate) const ENTRY_POINT_LOCK_BROKEN: u32 = 0x20;

/// The float setting `REFR_LOCK::GetLevel` scales a leveled lock's level
/// bonus by.
pub(crate) const LEVELED_LOCK_SETTING: u32 = 0x011c_39b4;
/// The integer settings that hold the top of each lock difficulty bracket
/// (very easy, easy, average, hard, very hard), and the one `00430bc0` also
/// knows for the enumeration value 5 (key only). See
/// `crates/world/src/locks.rs`.
pub(crate) const LOCK_LEVEL_SETTINGS: [u32; 6] = [
    0x011c_3a4c,
    0x011c_3a30,
    0x011c_3a00,
    0x011c_3ab0,
    0x011c_399c,
    0x011c_3a24,
];

/// `ExtraAnim`'s vtable.
pub(crate) const EXTRA_ANIM_VTABLE: u32 = 0x0101_5b28;
/// `ExtraDismemberedLimbs`'s vtable.
pub(crate) const EXTRA_DISMEMBERED_LIMBS_VTABLE: u32 = 0x0101_5b34;
/// `ExtraStartingPosition`'s vtable.
pub(crate) const EXTRA_STARTING_POSITION_VTABLE: u32 = 0x0101_5b40;
/// The vtable of the `BSExtraData` subclass of type 0x49 built by `004308c0`.
pub(crate) const EXTRA_TYPE_49_VTABLE: u32 = 0x0101_5b4c;
/// `ExtraLight`'s vtable.
pub(crate) const EXTRA_LIGHT_VTABLE: u32 = 0x0101_5b58;
/// `ExtraLock`'s vtable.
pub(crate) const EXTRA_LOCK_VTABLE: u32 = 0x0101_589c;
/// `ExtraFollower`'s vtable.
pub(crate) const EXTRA_FOLLOWER_VTABLE: u32 = 0x0101_5b64;
/// `ExtraGuardedRefData`'s vtable.
pub(crate) const EXTRA_GUARDED_REF_DATA_VTABLE: u32 = 0x0101_5b70;
/// The vtable of the `BSExtraData` subclass of type 0x2e built by `004311f0`.
pub(crate) const EXTRA_TYPE_2E_VTABLE: u32 = 0x0101_5b7c;
/// `ExtraTeleport`'s vtable.
pub(crate) const EXTRA_TELEPORT_VTABLE: u32 = 0x0101_58a8;

/// Extra-data type bytes, as passed to the base constructor.
pub(crate) const TYPE_DISMEMBERED_LIMBS: u32 = 0x5f;
pub(crate) const TYPE_STARTING_POSITION: u32 = 0x0f;
pub(crate) const TYPE_49: u32 = 0x49;
pub(crate) const TYPE_LIGHT: u32 = 0x29;
pub(crate) const TYPE_LOCK: u32 = 0x2a;
pub(crate) const TYPE_FOLLOWER: u32 = 0x1d;
pub(crate) const TYPE_GUARDED_REF_DATA: u32 = 0x7c;
pub(crate) const TYPE_2E: u32 = 0x2e;
pub(crate) const TYPE_TELEPORT: u32 = 0x2b;
pub(crate) const TYPE_MAP_MARKER: u32 = 0x2c;
pub(crate) const TYPE_AUDIO_MARKER: u32 = 0x90;
pub(crate) const TYPE_AUDIO_BUOY_MARKER: u32 = 0x91;
pub(crate) const TYPE_ACTION: u32 = 0x0e;
pub(crate) const TYPE_CONTAINER_CHANGES: u32 = 0x15;
pub(crate) const TYPE_ORIGINAL_REFERENCE: u32 = 0x20;
pub(crate) const TYPE_OWNERSHIP: u32 = 0x21;
pub(crate) const TYPE_GLOBAL: u32 = 0x22;
pub(crate) const TYPE_RANK: u32 = 0x23;
pub(crate) const TYPE_COUNT: u32 = 0x24;
pub(crate) const TYPE_HEALTH: u32 = 0x25;
pub(crate) const TYPE_USES: u32 = 0x26;
pub(crate) const TYPE_TIME_LEFT: u32 = 0x27;
pub(crate) const TYPE_CHARGE: u32 = 0x28;

/// `RTTI Type Descriptor`s of the classes whose `Compare` casts `other`.
pub(crate) const EXTRA_TELEPORT_TYPE: u32 = 0x0118_4430;
pub(crate) const EXTRA_MAP_MARKER_TYPE: u32 = 0x0118_45fc;
pub(crate) const EXTRA_AUDIO_MARKER_TYPE: u32 = 0x0118_45dc;
pub(crate) const EXTRA_AUDIO_BUOY_MARKER_TYPE: u32 = 0x0118_45b8;
pub(crate) const EXTRA_ACTION_TYPE: u32 = 0x0118_4bc0;
pub(crate) const EXTRA_ORIGINAL_REFERENCE_TYPE: u32 = 0x0118_4c00;
pub(crate) const EXTRA_OWNERSHIP_TYPE: u32 = 0x0118_476c;
pub(crate) const EXTRA_GLOBAL_TYPE: u32 = 0x0118_478c;
pub(crate) const EXTRA_RANK_TYPE: u32 = 0x0118_47a8;
pub(crate) const EXTRA_COUNT_TYPE: u32 = 0x0118_47c0;
pub(crate) const EXTRA_LEVELED_ITEM_TYPE: u32 = 0x0118_4680;
pub(crate) const EXTRA_HEALTH_TYPE: u32 = 0x0118_47dc;
pub(crate) const EXTRA_HEALTH_PERC_TYPE: u32 = 0x0118_4208;
pub(crate) const EXTRA_USES_TYPE: u32 = 0x0118_47f8;
pub(crate) const EXTRA_TIME_LEFT_TYPE: u32 = 0x0118_4810;

/// The vtables of the classes built by `00431360` to `00431f60`.
pub(crate) const EXTRA_MAP_MARKER_VTABLE: u32 = 0x0101_5b88;
pub(crate) const EXTRA_AUDIO_MARKER_VTABLE: u32 = 0x0101_5b94;
pub(crate) const EXTRA_AUDIO_BUOY_MARKER_VTABLE: u32 = 0x0101_5ba0;
pub(crate) const EXTRA_ACTION_VTABLE: u32 = 0x0101_5bac;
pub(crate) const EXTRA_CONTAINER_CHANGES_VTABLE: u32 = 0x0101_5bb8;
pub(crate) const EXTRA_ORIGINAL_REFERENCE_VTABLE: u32 = 0x0101_5bc4;
pub(crate) const EXTRA_OWNERSHIP_VTABLE: u32 = 0x0101_58b4;
pub(crate) const EXTRA_GLOBAL_VTABLE: u32 = 0x0101_58c0;
pub(crate) const EXTRA_RANK_VTABLE: u32 = 0x0101_58cc;
pub(crate) const EXTRA_COUNT_VTABLE: u32 = 0x0101_58d8;
pub(crate) const EXTRA_HEALTH_VTABLE: u32 = 0x0101_58e4;
pub(crate) const EXTRA_USES_VTABLE: u32 = 0x0101_58f0;
pub(crate) const EXTRA_TIME_LEFT_VTABLE: u32 = 0x0101_58fc;
pub(crate) const EXTRA_CHARGE_VTABLE: u32 = 0x0101_5908;

/// `DoorTeleportData::Compare` (Xbox PDB; `this` is the data, the argument is
/// the other data): true when the two differ.
pub(crate) const DOOR_TELEPORT_DATA_COMPARE: u32 = 0x0043_a860;
/// `MapMarkerData::Compare` (Xbox PDB; `this` is the data, the argument is
/// the other data): true when the two differ.
pub(crate) const MAP_MARKER_DATA_COMPARE: u32 = 0x0043_8e40;
/// `AudioMarkerData::Compare` (Xbox PDB; `this` is the data, the argument is
/// the other data): true when the two differ.
pub(crate) const AUDIO_MARKER_DATA_COMPARE: u32 = 0x0058_97c0;
/// A 15-byte folded body that ignores its argument and returns true (the
/// engine map names it `DetailedActorPathHandler::IsDetailedPathHandler`);
/// `ExtraAudioBuoyMarker::Compare` calls it with the two `AudioBuoyMarkerData`
/// pointers (`this` is the first, the argument the second).
pub(crate) const AUDIO_BUOY_DATA_COMPARE: u32 = 0x0040_1290;
/// Scalar deleting destructor of the `MapMarkerData` an `ExtraMapMarker` owns
/// (`this` is the data, the argument says whether to free it).
pub(crate) const MAP_MARKER_DATA_DELETE: u32 = 0x0041_9350;
/// Scalar deleting destructor of the `AudioMarkerData` an `ExtraAudioMarker`
/// owns (`this` is the data, the argument says whether to free it).
pub(crate) const AUDIO_MARKER_DATA_DELETE: u32 = 0x0041_9480;
/// Scalar deleting destructor of the `AudioBuoyMarkerData` an
/// `ExtraAudioBuoyMarker` owns (in `racesexmenu.cpp` by address range; `this`
/// is the data, the argument says whether to free it).
pub(crate) const AUDIO_BUOY_DATA_DELETE: u32 = 0x007b_3fa0;
/// The destructor body of the `InventoryChanges` an `ExtraContainerChanges`
/// owns (in `inventorychanges.cpp`; `this` is the object).
pub(crate) const INVENTORY_CHANGES_DESTRUCT: u32 = 0x004b_f150;

/// `RTTI Type Descriptor`s of the classes whose `Compare` casts `other`
/// (second batch).
pub(crate) const EXTRA_CHARGE_TYPE: u32 = 0x0118_482c;
pub(crate) const EXTRA_SCRIPT_TYPE: u32 = 0x0118_4514;
pub(crate) const EXTRA_WEAPON_MOD_FLAGS_TYPE: u32 = 0x0118_48d8;
pub(crate) const EXTRA_MODDING_ITEM_TYPE: u32 = 0x0118_493c;
pub(crate) const EXTRA_SCALE_TYPE: u32 = 0x0118_4848;
pub(crate) const EXTRA_HOT_KEY_TYPE: u32 = 0x0118_4864;
pub(crate) const EXTRA_SEED_TYPE: u32 = 0x0118_4c98;
pub(crate) const EXTRA_PACKAGE_START_LOCATION_TYPE: u32 = 0x0118_448c;

/// The vtables of the classes built by `00432000` to `00432cb0`.
pub(crate) const EXTRA_SCRIPT_VTABLE: u32 = 0x0101_5914;
pub(crate) const EXTRA_SCALE_VTABLE: u32 = 0x0101_5920;
pub(crate) const EXTRA_HOT_KEY_VTABLE: u32 = 0x0101_592c;
pub(crate) const EXTRA_REFERENCE_POINTER_VTABLE: u32 = 0x0101_5938;
pub(crate) const EXTRA_TRES_PASS_PACKAGE_VTABLE: u32 = 0x0101_5944;
pub(crate) const EXTRA_LEVELED_ITEM_VTABLE: u32 = 0x0101_5950;
pub(crate) const EXTRA_GHOST_VTABLE: u32 = 0x0101_5bd0;
pub(crate) const EXTRA_WORN_VTABLE: u32 = 0x0101_5bdc;
pub(crate) const EXTRA_WORN_LEFT_VTABLE: u32 = 0x0101_5be8;
pub(crate) const EXTRA_CANNOT_WEAR_VTABLE: u32 = 0x0101_5bf4;
pub(crate) const EXTRA_INFO_GENERAL_TOPIC_VTABLE: u32 = 0x0101_5c00;
pub(crate) const EXTRA_SEED_VTABLE: u32 = 0x0101_5c0c;
pub(crate) const EXTRA_PACKAGE_START_LOCATION_VTABLE: u32 = 0x0101_5c18;
pub(crate) const EXTRA_PACKAGE_VTABLE: u32 = 0x0101_5c24;
pub(crate) const EXTRA_PLAYER_CRIME_LIST_VTABLE: u32 = 0x0101_5c30;
pub(crate) const EXTRA_PERSISTENT_CELL_VTABLE: u32 = 0x0101_5c3c;
pub(crate) const EXTRA_RAG_DOLL_DATA_VTABLE: u32 = 0x0101_5c48;

/// Extra-data type bytes of the second batch.
pub(crate) const TYPE_SCRIPT: u32 = 0x0d;
pub(crate) const TYPE_SCALE: u32 = 0x30;
pub(crate) const TYPE_GHOST: u32 = 0x1f;
pub(crate) const TYPE_WORN: u32 = 0x16;
pub(crate) const TYPE_WORN_LEFT: u32 = 0x17;
pub(crate) const TYPE_CANNOT_WEAR: u32 = 0x3e;
pub(crate) const TYPE_HOT_KEY: u32 = 0x4a;
pub(crate) const TYPE_INFO_GENERAL_TOPIC: u32 = 0x4d;
pub(crate) const TYPE_SEED: u32 = 0x31;
pub(crate) const TYPE_PACKAGE_START_LOCATION: u32 = 0x18;
pub(crate) const TYPE_REFERENCE_POINTER: u32 = 0x1c;
pub(crate) const TYPE_PACKAGE: u32 = 0x19;
pub(crate) const TYPE_TRES_PASS_PACKAGE: u32 = 0x1a;
pub(crate) const TYPE_PLAYER_CRIME_LIST: u32 = 0x35;
pub(crate) const TYPE_LEVELED_ITEM: u32 = 0x2f;
pub(crate) const TYPE_PERSISTENT_CELL: u32 = 0x0c;
pub(crate) const TYPE_RAG_DOLL_DATA: u32 = 0x14;

/// Scalar deleting destructor of the `ScriptLocals` an `ExtraScript` owns
/// (in `extradatalist.cpp` by address range; `this` is the object, the
/// argument says whether to free it).
pub(crate) const SCRIPT_LOCALS_DELETE: u32 = 0x0041_af70;
/// `MenuTopic::~MenuTopic` (Xbox PDB name; it is the scalar deleting
/// destructor: `this` is the topic, the argument says whether to free it).
pub(crate) const MENU_TOPIC_DELETE: u32 = 0x0042_5ff0;
/// The constructor of the 0x2c-byte `MenuTopic` an `ExtraInfoGeneralTopic`
/// creates (`this` is the memory, the result is the object).
pub(crate) const MENU_TOPIC_CONSTRUCT: u32 = 0x0083_da50;
/// The constructor of the `WORLD_LOCATION` member of
/// `ExtraPackageStartLocation` (`this` is the member).
pub(crate) const WORLD_LOCATION_CONSTRUCT: u32 = 0x006d_5320;
/// `TESPackage::SetIsCreated(created)` (Xbox PDB name; `this` is the
/// package).
pub(crate) const PACKAGE_SET_IS_CREATED: u32 = 0x0067_4d70;
/// The global holding the `TESSaveLoadGame` object pointer (the singleton
/// whose `DeleteForm` `ExtraTresPassPackage`'s destructor calls).
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// A 13-byte stub (`xor al, al`) the destructor of `ExtraTresPassPackage`
/// calls on the save/load object: it always answers false.
pub(crate) const SAVE_LOAD_GAME_ALWAYS_FALSE: u32 = 0x0047_c850;
/// `TESSaveLoadGame::DeleteForm(form)` (Xbox PDB name; `this` is the
/// save/load object).
pub(crate) const SAVE_LOAD_GAME_DELETE_FORM: u32 = 0x0085_a2e0;
/// Adds the item held at the address given to the `BSSimpleList` (`this` is
/// the list; an unplaced body, `005ae3d0`).
pub(crate) const CRIME_LIST_ADD: u32 = 0x005a_e3d0;
/// The destructor body of the `RagDollData` an `ExtraRagDollData` owns (in
/// `ragdolldata.cpp`; `this` is the object).
pub(crate) const RAG_DOLL_DATA_DESTRUCT: u32 = 0x004d_9380;

/// `RagDollData::Compare` (Xbox PDB; `this` is the data, the argument the
/// other data): true when the two differ.
pub(crate) const RAG_DOLL_DATA_COMPARE: u32 = 0x004d_9780;

/// `RTTI Type Descriptor`s of the classes whose `Compare` or `Copy` casts
/// `other` (third batch).
pub(crate) const EXTRA_RAG_DOLL_DATA_TYPE: u32 = 0x0118_4cd4;
pub(crate) const EXTRA_ENCOUNTER_ZONE_TYPE: u32 = 0x0118_4cf4;
pub(crate) const EXTRA_ENABLE_STATE_PARENT_TYPE: u32 = 0x0118_4d7c;
pub(crate) const EXTRA_RANDOM_TELEPORT_MARKER_TYPE: u32 = 0x0118_4dcc;
pub(crate) const EXTRA_LINKED_REF_TYPE: u32 = 0x0118_4e1c;
pub(crate) const EXTRA_ACTIVATE_REF_TYPE: u32 = 0x0118_4e84;

/// The vtables of the classes built by `00432e60` to `00433ca0` (names from
/// the RTTI type descriptors).
pub(crate) const EXTRA_ENCOUNTER_ZONE_VTABLE: u32 = 0x0101_5c54;
pub(crate) const EXTRA_USED_MARKERS_VTABLE: u32 = 0x0101_5c60;
pub(crate) const EXTRA_RESERVED_MARKERS_VTABLE: u32 = 0x0101_5c6c;
pub(crate) const EXTRA_RUN_ONCE_PACKS_VTABLE: u32 = 0x0101_5c78;
pub(crate) const EXTRA_DISTANT_DATA_VTABLE: u32 = 0x0101_5c84;
pub(crate) const EXTRA_ENABLE_STATE_PARENT_VTABLE: u32 = 0x0101_5c90;
pub(crate) const EXTRA_ENABLE_STATE_CHILDREN_VTABLE: u32 = 0x0101_5c9c;
pub(crate) const EXTRA_RANDOM_TELEPORT_MARKER_VTABLE: u32 = 0x0101_5ca8;
pub(crate) const EXTRA_LINKED_REF_CHILDREN_VTABLE: u32 = 0x0101_5cb4;
pub(crate) const EXTRA_LINKED_REF_VTABLE: u32 = 0x0101_5cc0;
pub(crate) const EXTRA_ASH_PILE_REF_VTABLE: u32 = 0x0101_5ccc;
pub(crate) const EXTRA_ACTIVATE_REF_CHILDREN_VTABLE: u32 = 0x0101_5cd8;
pub(crate) const EXTRA_ACTIVATE_REF_VTABLE: u32 = 0x0101_5ce4;
pub(crate) const EXTRA_DECAL_REFS_VTABLE: u32 = 0x0101_5cf0;

/// Extra-data type bytes of the third batch.
pub(crate) const TYPE_ENCOUNTER_ZONE: u32 = 0x74;
pub(crate) const TYPE_USED_MARKERS: u32 = 0x12;
pub(crate) const TYPE_RESERVED_MARKERS: u32 = 0x82;
pub(crate) const TYPE_RUN_ONCE_PACKS: u32 = 0x1b;
pub(crate) const TYPE_DISTANT_DATA: u32 = 0x13;
pub(crate) const TYPE_ENABLE_STATE_PARENT: u32 = 0x37;
pub(crate) const TYPE_ENABLE_STATE_CHILDREN: u32 = 0x38;
pub(crate) const TYPE_RANDOM_TELEPORT_MARKER: u32 = 0x3b;
pub(crate) const TYPE_LINKED_REF_CHILDREN: u32 = 0x52;
pub(crate) const TYPE_LINKED_REF: u32 = 0x51;
pub(crate) const TYPE_ASH_PILE_REF: u32 = 0x89;
pub(crate) const TYPE_ACTIVATE_REF_CHILDREN: u32 = 0x54;
pub(crate) const TYPE_ACTIVATE_REF: u32 = 0x53;
pub(crate) const TYPE_DECAL_REFS: u32 = 0x57;

/// `BSSimpleList<T>`'s constructor (`this` is the head node: item and next
/// are cleared).
pub(crate) const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList<T>::Clear` (the engine map has no name; `this` is the head
/// node: frees every node after the head and clears its item).
pub(crate) const SIMPLE_LIST_CLEAR: u32 = 0x0047_0470;
/// The destructor body of a `BSSimpleList<T>` (`this` is the head node): it
/// clears the list.
pub(crate) const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `BSSimpleList<T>`'s scalar deleting destructor (`this` is the list, the
/// argument says whether to free it).
pub(crate) const SIMPLE_LIST_DELETE: u32 = 0x0047_02f0;
/// The address of the item slot of a list node (`this` is the node; returns
/// `this`, the item is the first word).
pub(crate) const SIMPLE_LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// The next node of a list node (`this` is the node; null at the end).
pub(crate) const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
/// Whether a list node holds no item and has no successor (`this` is the
/// node).
pub(crate) const SIMPLE_LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Removes the head of a list (`this` is the head node): the next node's
/// item and successor move into the head and the next node is deleted; the
/// head's item is cleared when there is no next node.
pub(crate) const SIMPLE_LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// Adds the item held at the address given as the new head of the list (`this`
/// is the list; the same body as `CRIME_LIST_ADD`, which names it by its use
/// in `ExtraPlayerCrimeList`).
pub(crate) const SIMPLE_LIST_ADD_HEAD: u32 = CRIME_LIST_ADD;
/// The number of non-empty nodes of a list (`this` is the head node;
/// the engine map calls it `VATS::GetCount` by mistake).
pub(crate) const SIMPLE_LIST_COUNT: u32 = 0x005a_e380;

/// `NiPoint3::NiPoint3(x, y, z)` (Xbox PDB name of the folded body; `this`
/// is the point, then three floats on the stack).
pub(crate) const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// The `float` constant `ExtraDistantData`'s constructor uses for the
/// z component of its land normal.
pub(crate) const DISTANT_DATA_NORMAL_Z: u32 = 0x0101_45a8;
/// `BSStringT<char>::BSStringT` (`this` is the string: empties it).
pub(crate) const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT<char>::Set(source, length)` (`this` is the string; the source
/// is a character pointer and the length 0 means all of it).
pub(crate) const BS_STRING_SET: u32 = 0x0040_37f0;
/// `BSStringT<char>::Clear` (`this` is the string; `Set(0, 0)`).
pub(crate) const BS_STRING_CLEAR: u32 = 0x0040_37d0;
/// `BSStringT<char>::Set(other)` (Xbox PDB; `this` is the string, the
/// argument the string to copy; in this unit, not yet translated).
pub(crate) const BS_STRING_COPY: u32 = 0x0043_8470;
/// `BSStringT<char>::GetLength` (the folded body: the stored length, or the
/// C string's length when that is 0xffff; `this` is the string).
pub(crate) const BS_STRING_LENGTH: u32 = 0x0040_48e0;
/// `BSStringT<char>::c_str` (`this` is the string: its character pointer).
pub(crate) const BS_STRING_DATA: u32 = 0x0055_9450;
/// `strcmp(a, b)` (the wrapper; `00ec6da0` is the CRT function).
pub(crate) const STRCMP: u32 = 0x0040_8b20;
/// `memcpy(destination, source, size)` (the wrapper around `00ec44d0`).
pub(crate) const MEMCPY: u32 = 0x0040_1460;
/// The constructor of a `REF_ACTIVATE_DATA` (`this` is the 8-byte data:
/// clears the reference and the delay).
pub(crate) const REF_ACTIVATE_DATA_CONSTRUCT: u32 = 0x0041_4010;
/// An empty C string in the exe's data (what `ExtraActivateRef`'s constructor sets
/// its text to).
pub(crate) const EMPTY_STRING: u32 = 0x0101_1584;

/// Constructor of the `DismemberedLimbs` array member of
/// `ExtraDismemberedLimbs` (`this` is the array). In this unit; not yet
/// translated.
pub(crate) const DISMEMBERED_LIMBS_ARRAY_CONSTRUCT: u32 = 0x0043_8570;
/// Destructor of the same array. In this unit; not yet translated.
pub(crate) const DISMEMBERED_LIMBS_ARRAY_DESTRUCT: u32 = 0x0043_85a0;
/// Constructor of the `Guards` array member of `ExtraGuardedRefData`. In
/// this unit; not yet translated.
pub(crate) const GUARDS_ARRAY_CONSTRUCT: u32 = 0x0043_8660;
/// Destructor of the same array. In this unit; not yet translated.
pub(crate) const GUARDS_ARRAY_DESTRUCT: u32 = 0x0043_8690;
/// Constructor of a `DismemberedLimb` entry (`this` is the 0x14-byte block).
pub(crate) const DISMEMBERED_LIMB_CONSTRUCT: u32 = 0x0042_c470;
/// Destructor body of the `ObjectArray` member of a `DismemberedLimb` (`this`
/// is the array).
pub(crate) const DISMEMBERED_LIMB_ARRAY_DESTRUCT: u32 = 0x0042_ff80;
/// `TESNPC::BuildObjectArray(reference, base, array)` (Xbox PDB name); `this`
/// is what `004181e0` returns for the reference.
pub(crate) const TESNPC_BUILD_OBJECT_ARRAY: u32 = 0x0060_5fc0;
/// Takes a reference and returns the object `TESNPC::BuildObjectArray` is
/// called on (a 19-byte accessor in `extradatalist.cpp`).
pub(crate) const REFR_NPC_ACCESSOR: u32 = 0x0041_81e0;
/// The constructor of the `FILE_POS_ROT` member of `ExtraStartingPosition`
/// (`this` is the member).
pub(crate) const FILE_POS_ROT_CONSTRUCT: u32 = 0x0069_2710;
/// Deletes the animation an `ExtraAnim` owns (`this` is the animation, the
/// argument says whether to free it).
pub(crate) const ANIMATION_DELETE: u32 = 0x0041_8d20;
/// Deletes the light an `ExtraLight` owns (`this` is the light, the argument
/// says whether to free it).
pub(crate) const LIGHT_DELETE: u32 = 0x0041_8f10;
/// Deletes the teleport data an `ExtraTeleport` owns (`this` is the data,
/// the argument says whether to free it).
pub(crate) const TELEPORT_DATA_DELETE: u32 = 0x0041_9220;
/// Constructor of the 8-byte actor list an `ExtraFollower` owns.
pub(crate) const ACTOR_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Clears the actor list (`this` is the list).
pub(crate) const ACTOR_LIST_CLEAR: u32 = 0x0047_0470;
/// Scalar deleting destructor of the actor list (`this` is the list, the
/// argument says whether to free it).
pub(crate) const ACTOR_LIST_DELETE: u32 = 0x0047_02f0;
/// Returns the value `ExtraGuardedRefData::AddGuard` stores for a reference
/// (`this` is the reference).
pub(crate) const REFR_GUARD_VALUE: u32 = 0x0084_e3a0;
/// `LookupFormByID(id)` (cdecl, one stack argument).
pub(crate) const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// Returns the process object a form's guard notification goes to (the
/// engine map's `MiddleHighProcess::GetSavedAcquireObject`; `this` is the
/// form).
pub(crate) const FORM_PROCESS: u32 = 0x008d_8520;

/// `RTTI Type Descriptor`s used by the fourth batch: the source and target of
/// the `InitItem`s' cast of a looked-up form (`TESForm`, `TESObjectREFR`), and
/// the classes whose `Compare` or `Copy` casts `other`.
pub(crate) const TES_FORM_TYPE: u32 = 0x0118_3028;
pub(crate) const TES_OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
pub(crate) const EXTRA_DECAL_REFS_TYPE: u32 = 0x0118_4ea4;
pub(crate) const EXTRA_REFLECTED_REFS_TYPE: u32 = 0x0118_3fec;
pub(crate) const EXTRA_REFLECTOR_REFS_TYPE: u32 = 0x0118_4010;
pub(crate) const EXTRA_WATER_LIGHT_REFS_TYPE: u32 = 0x0118_4034;
pub(crate) const EXTRA_LIT_WATER_REFS_TYPE: u32 = 0x0118_4058;
pub(crate) const EXTRA_MERCHANT_CONTAINER_TYPE: u32 = 0x0118_4ec4;
pub(crate) const EXTRA_LEV_CREA_MODIFIER_TYPE: u32 = 0x0118_4eec;
pub(crate) const EXTRA_POISON_TYPE: u32 = 0x0118_461c;
pub(crate) const EXTRA_LAST_FINISHED_SEQUENCE_TYPE: u32 = 0x0118_4408;

/// The vtables of the classes of the fourth batch (names from the RTTI
/// type descriptors).
pub(crate) const EXTRA_REFLECTED_REFS_VTABLE: u32 = 0x0101_4428;
pub(crate) const EXTRA_REFLECTOR_REFS_VTABLE: u32 = 0x0101_4434;
pub(crate) const EXTRA_WATER_LIGHT_REFS_VTABLE: u32 = 0x0101_4440;
pub(crate) const EXTRA_LIT_WATER_REFS_VTABLE: u32 = 0x0101_444c;
pub(crate) const EXTRA_MERCHANT_CONTAINER_VTABLE: u32 = 0x0101_5e04;
pub(crate) const EXTRA_LEV_CREA_MODIFIER_VTABLE: u32 = 0x0101_5e10;
pub(crate) const EXTRA_POISON_VTABLE: u32 = 0x0101_595c;
pub(crate) const EXTRA_LAST_FINISHED_SEQUENCE_VTABLE: u32 = 0x0101_5e1c;
pub(crate) const EXTRA_X_TARGET_VTABLE: u32 = 0x0101_5e28;

/// Extra-data type bytes of the fourth batch.
pub(crate) const TYPE_MERCHANT_CONTAINER: u32 = 0x3c;
pub(crate) const TYPE_LEV_CREA_MODIFIER: u32 = 0x1e;
pub(crate) const TYPE_POISON: u32 = 0x3f;
pub(crate) const TYPE_LAST_FINISHED_SEQUENCE: u32 = 0x41;
pub(crate) const TYPE_X_TARGET: u32 = 0x44;

/// `LCM_NONE` (Xbox PDB `LEV_CREA_MODIFIER`; the others are `LCM_EASY` 0,
/// `LCM_MEDIUM` 1, `LCM_HARD` 2, `LCM_BOSS` 3).
pub(crate) const LEV_CREA_MODIFIER_NONE: u32 = 4;
/// The table of float-setting pointers, one per `LEV_CREA_MODIFIER` value
/// below `LCM_NONE`, that `fn_004350c0` indexes.
pub(crate) const LEV_CREA_SETTING_TABLE: u32 = 0x0118_4ab0;

/// `TESForm::GetFile(index)` (Xbox PDB name; `this` is the form, the argument
/// -1 asks for the last file that changed it).
pub(crate) const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex(formIdAddress, file)` (Xbox PDB name, cdecl, two
/// stack arguments): adds the file's compile index to the form id held at the
/// address given.
pub(crate) const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB name; `this` is the reference).
pub(crate) const REFR_GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// The accessor of a reference's extra-data list (`this + 0x44`, in
/// `tesscriptfunctions.cpp` by address range; `this` is the reference, the
/// result the address of the embedded list).
pub(crate) const REFR_EXTRA_LIST: u32 = 0x005d_43c0;
/// The log function (cdecl, `format, ...`) the `InitItem`s report with.
pub(crate) const MASTERFILE_LOG: u32 = 0x005b_5e40;
/// Two methods of `ExtraDataList` (in `extradatalist.cpp` by address range;
/// `this` is the list, the arguments are a reference and a flag, 1 to add)
/// that `ExtraReflectorRefs::InitItem` calls for effect flag bits 0 and 1.
pub(crate) const REFLECTOR_ADD_FIRST: u32 = 0x0041_f4c0;
pub(crate) const REFLECTOR_ADD_SECOND: u32 = 0x0041_f650;
/// `ExtraDataList::SetWaterLightRef(reference, add)` (engine map name; `this`
/// is the extra-data list).
pub(crate) const WATER_LIGHT_REF_SET: u32 = 0x0041_f840;
/// Removes from the list starting at `this` the first node whose item equals
/// the word at the address given (in `highprocess.cpp` by address range).
pub(crate) const SIMPLE_LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// Whether the list at `this` holds an item equal to the word at the address
/// given (the engine map has no name for it).
pub(crate) const SIMPLE_LIST_CONTAINS: u32 = 0x005f_65d0;
/// The constructor of a `REF_DECAL_DATA` (`this` is the 0x1c-byte block; it
/// runs two folded `NiPoint3` constructors and returns `this`).
pub(crate) const REF_DECAL_DATA_CONSTRUCT: u32 = 0x0055_a400;
/// `strlen(string)` (the wrapper around the CRT function `00ec6130`, cdecl).
pub(crate) const STRLEN: u32 = 0x0044_a670;
/// `strcpy_s(destination, size, source)` (the wrapper around `00ec65a6`,
/// cdecl).
pub(crate) const STRING_COPY_CHECKED: u32 = 0x0040_6d30;

/// The messages the `InitItem`s log (the format strings of `MASTERFILE_LOG`;
/// each takes one form id).
pub(crate) const MESSAGE_DECAL_NOT_FOUND: u32 = 0x0101_5cf8;
pub(crate) const MESSAGE_REFLECTOR_NOT_FOUND: u32 = 0x0101_5d8c;
pub(crate) const MESSAGE_NOT_PERSISTENT: u32 = 0x0101_5d30;
pub(crate) const MESSAGE_LIT_WATER_NOT_FOUND: u32 = 0x0101_5dc8;

layout! {
    /// `NiPoint3` (Xbox PDB): three floats.
    pub struct NiPoint3: 0x0C {
        /// `x` (Xbox PDB).
        0x00 x: f32,
        /// `y` (Xbox PDB).
        0x04 y: f32,
        /// `z` (Xbox PDB).
        0x08 z: f32,
    }

    /// `FILE_POS_ROT` (Xbox PDB): a position and a rotation.
    pub struct FilePosRot: 0x18 {
        /// `pos` (Xbox PDB).
        0x00 pos: Inline<NiPoint3>,
        /// `rot` (Xbox PDB).
        0x0C rot: Inline<NiPoint3>,
    }

    /// `ExtraAnim` (Xbox PDB), 0x10 bytes. The `BSExtraData` base holds the
    /// vtable at +0, `cEtype` at +4 and `pNext` at +8.
    pub struct ExtraAnim: 0x10 {
        /// `pAnimation` (Xbox PDB): `Animation*`, owned.
        0x0C pAnimation: Ptr,
    }

    /// `ExtraDismemberedLimbs` (Xbox PDB), 0x30 bytes.
    pub struct ExtraDismemberedLimbs: 0x30 {
        /// `sLimbs` (Xbox PDB): bit `n` is set when limb `n` is dismembered.
        0x0C sLimbs: u16,
        /// `eCauseofDeath` (Xbox PDB).
        0x10 eCauseofDeath: u32,
        /// `pDeathObject` (Xbox PDB): `TESForm*`.
        0x14 pDeathObject: Ptr,
        /// `eLastHitLimb` (Xbox PDB).
        0x18 eLastHitLimb: u32,
        /// `bEaten` (Xbox PDB).
        0x1C bEaten: bool,
        /// `DismemberedLimbs` (Xbox PDB): `BSSimpleArray<DismemberedLimb *,1024>`.
        0x20 DismemberedLimbs: Inline<BSSimpleArray>,
    }

    /// `DismemberedLimb` (Xbox PDB), 0x14 bytes.
    pub struct DismemberedLimb: 0x14 {
        /// `cLimb` (Xbox PDB).
        0x00 cLimb: u8,
        /// `bLimbExploded` (Xbox PDB; a `bool`, kept as the byte the caller
        /// passed).
        0x01 bLimbExploded: u8,
        /// `bObjectArrayIdentical` (Xbox PDB; a `bool`, kept as a byte).
        0x02 bObjectArrayIdentical: u8,
        /// `bLimbRemoved` (Xbox PDB; a `bool`, kept as the byte the caller
        /// passed).
        0x03 bLimbRemoved: u8,
        /// `ObjectArray` (Xbox PDB): `BSSimpleArray<TESBoundObject *,1024>`.
        0x04 ObjectArray: Inline<BSSimpleArray>,
    }

    /// `ExtraStartingPosition` (Xbox PDB), 0x24 bytes.
    pub struct ExtraStartingPosition: 0x24 {
        /// `startPosition` (Xbox PDB).
        0x0C startPosition: Inline<FilePosRot>,
    }

    /// The `BSExtraData` subclass of type 0x49 (vtable `01015b4c`), 0x10
    /// bytes; the Xbox PDB name is not known.
    pub struct ExtraType49: 0x10 {
        /// The word `004308c0` clears.
        0x0C unnamed0C: u32,
    }

    /// `ExtraLight` (Xbox PDB), 0x10 bytes.
    pub struct ExtraLight: 0x10 {
        /// `pLight` (Xbox PDB): `REFR_LIGHT*`, owned.
        0x0C pLight: Ptr,
    }

    /// `REFR_LOCK` (Xbox PDB), 0x14 bytes.
    pub struct RefrLock: 0x14 {
        /// `cBaseLevel` (Xbox PDB).
        0x00 cBaseLevel: u8,
        /// `pKey` (Xbox PDB): `TESKey*`.
        0x04 pKey: Ptr,
        /// `cFlags` (Xbox PDB): bit 0 locked, bit 2 leveled.
        0x08 cFlags: i8,
        /// `uiNumTries` (Xbox PDB).
        0x0C uiNumTries: u32,
        /// `uiTimesUnlocked` (Xbox PDB).
        0x10 uiTimesUnlocked: u32,
    }

    /// `ExtraLock` (Xbox PDB), 0x10 bytes.
    pub struct ExtraLock: 0x10 {
        /// `pLock` (Xbox PDB): `REFR_LOCK*`, owned.
        0x0C pLock: Ptr<RefrLock>,
    }

    /// `ExtraFollower` (Xbox PDB), 0x10 bytes.
    pub struct ExtraFollower: 0x10 {
        /// `pActorlist` (Xbox PDB): `BSSimpleList<Actor *>*`, owned.
        0x0C pActorlist: Ptr,
    }

    /// `ExtraGuardedRefData` (Xbox PDB), 0x1C bytes.
    pub struct ExtraGuardedRefData: 0x1C {
        /// `Guards` (Xbox PDB): `BSSimpleArray<unsigned int,1024>`.
        0x0C Guards: Inline<BSSimpleArray>,
    }

    /// The `BSExtraData` subclass of type 0x2e (vtable `01015b7c`), 0x14
    /// bytes; the Xbox PDB name is not known.
    pub struct ExtraType2e: 0x14 {
        /// The first word `004311f0` clears.
        0x0C unnamed0C: u32,
        /// The second word `004311f0` clears.
        0x10 unnamed10: u32,
    }

    /// `ExtraTeleport` (Xbox PDB), 0x10 bytes.
    pub struct ExtraTeleport: 0x10 {
        /// `pData` (Xbox PDB): the teleport data, owned.
        0x0C pData: Ptr,
    }

    /// `ExtraMapMarker` (Xbox PDB), 0x10 bytes.
    pub struct ExtraMapMarker: 0x10 {
        /// `pMapData` (Xbox PDB): `MapMarkerData*`, owned.
        0x0C pMapData: Ptr,
    }

    /// `ExtraAudioMarker` (Xbox PDB), 0x10 bytes.
    pub struct ExtraAudioMarker: 0x10 {
        /// `pAudioData` (Xbox PDB): `AudioMarkerData*`, owned.
        0x0C pAudioData: Ptr,
    }

    /// `ExtraAudioBuoyMarker` (Xbox PDB), 0x10 bytes.
    pub struct ExtraAudioBuoyMarker: 0x10 {
        /// `pAudioBuoyData` (Xbox PDB): `AudioBuoyMarkerData*`, owned.
        0x0C pAudioBuoyData: Ptr,
    }

    /// `ExtraAction` (Xbox PDB), 0x14 bytes.
    pub struct ExtraAction: 0x14 {
        /// `eAction` (Xbox PDB).
        0x0C eAction: u8,
        /// `pActionRef` (Xbox PDB): `TESObjectREFR*`.
        0x10 pActionRef: Ptr,
    }

    /// `ExtraContainerChanges` (Xbox PDB), 0x10 bytes.
    pub struct ExtraContainerChanges: 0x10 {
        /// `pChanges` (Xbox PDB): `InventoryChanges*`, owned.
        0x0C pChanges: Ptr,
    }

    /// `ExtraOriginalReference` (Xbox PDB), 0x10 bytes.
    pub struct ExtraOriginalReference: 0x10 {
        /// `pReference` (Xbox PDB): `TESObjectREFR*`.
        0x0C pReference: Ptr,
    }

    /// `ExtraOwnership` (Xbox PDB), 0x10 bytes.
    pub struct ExtraOwnership: 0x10 {
        /// `pOwner` (Xbox PDB): `TESForm*`.
        0x0C pOwner: Ptr,
    }

    /// `ExtraGlobal` (Xbox PDB), 0x10 bytes.
    pub struct ExtraGlobal: 0x10 {
        /// `pGlobal` (Xbox PDB): `TESGlobal*`.
        0x0C pGlobal: Ptr,
    }

    /// `ExtraRank` (Xbox PDB), 0x10 bytes.
    pub struct ExtraRank: 0x10 {
        /// `iRank` (Xbox PDB).
        0x0C iRank: i32,
    }

    /// `ExtraCount` (Xbox PDB), 0x10 bytes.
    pub struct ExtraCount: 0x10 {
        /// `iCount` (Xbox PDB): a signed 16-bit count.
        0x0C iCount: i16,
    }

    /// `ExtraLeveledItem` (Xbox PDB), 0x14 bytes.
    pub struct ExtraLeveledItem: 0x14 {
        /// `iIndex` (Xbox PDB).
        0x0C iIndex: i32,
        /// `bdefault` (Xbox PDB).
        0x10 bdefault: bool,
    }

    /// `ExtraHealth` (Xbox PDB), 0x10 bytes.
    pub struct ExtraHealth: 0x10 {
        /// `fHealth` (Xbox PDB).
        0x0C fHealth: f32,
    }

    /// `ExtraHealthPerc` (Xbox PDB), 0x10 bytes.
    pub struct ExtraHealthPerc: 0x10 {
        /// `fHealthPerc` (Xbox PDB).
        0x0C fHealthPerc: f32,
    }

    /// `ExtraUses` (Xbox PDB), 0x10 bytes.
    pub struct ExtraUses: 0x10 {
        /// `cUses` (Xbox PDB).
        0x0C cUses: u8,
    }

    /// `ExtraTimeLeft` (Xbox PDB), 0x10 bytes.
    pub struct ExtraTimeLeft: 0x10 {
        /// `fTime` (Xbox PDB).
        0x0C fTime: f32,
    }

    /// `ExtraCharge` (Xbox PDB), 0x10 bytes.
    pub struct ExtraCharge: 0x10 {
        /// `fCharge` (Xbox PDB).
        0x0C fCharge: f32,
    }

    /// `ExtraScript` (Xbox PDB), 0x14 bytes.
    pub struct ExtraScript: 0x14 {
        /// `pScript` (Xbox PDB): `Script*`, not owned.
        0x0C pScript: Ptr,
        /// `pScriptVars` (Xbox PDB): `ScriptLocals*`, owned.
        0x10 pScriptVars: Ptr,
    }

    /// `ExtraWeaponModFlags` (Xbox PDB), 0x10 bytes.
    pub struct ExtraWeaponModFlags: 0x10 {
        /// `cWeaponModsActive` (Xbox PDB).
        0x0C cWeaponModsActive: u8,
    }

    /// `ExtraModdingItem` (Xbox PDB), 0x10 bytes.
    pub struct ExtraModdingItem: 0x10 {
        /// `bIsModding` (Xbox PDB; a `bool`, compared as a byte).
        0x0C bIsModding: u8,
    }

    /// `ExtraScale` (Xbox PDB), 0x10 bytes.
    pub struct ExtraScale: 0x10 {
        /// `fScale` (Xbox PDB).
        0x0C fScale: f32,
    }

    /// `ExtraGhost` (Xbox PDB), 0x0C bytes: the base only.
    pub struct ExtraGhost: 0x0C {}

    /// `ExtraWorn` (Xbox PDB), 0x0C bytes: the base only.
    pub struct ExtraWorn: 0x0C {}

    /// `ExtraWornLeft` (Xbox PDB), 0x0C bytes: the base only.
    pub struct ExtraWornLeft: 0x0C {}

    /// `ExtraCannotWear` (Xbox PDB), 0x0C bytes: the base only.
    pub struct ExtraCannotWear: 0x0C {}

    /// `ExtraHotKey` (Xbox PDB), 0x10 bytes.
    pub struct ExtraHotKey: 0x10 {
        /// `chotkey` (Xbox PDB): a `char`.
        0x0C chotkey: i8,
    }

    /// `ExtraInfoGeneralTopic` (Xbox PDB), 0x10 bytes.
    pub struct ExtraInfoGeneralTopic: 0x10 {
        /// `pInfoGen` (Xbox PDB): `MenuTopic*`, owned.
        0x0C pInfoGen: Ptr,
    }

    /// `ExtraSeed` (Xbox PDB), 0x10 bytes.
    pub struct ExtraSeed: 0x10 {
        /// `iSeed` (Xbox PDB): a byte.
        0x0C iSeed: u8,
    }

    /// `WORLD_LOCATION` (Xbox PDB), 0x14 bytes.
    pub struct WorldLocation: 0x14 {
        /// `pLocationForm` (Xbox PDB): `TESForm*`.
        0x00 pLocationForm: Ptr,
        /// `locPt` (Xbox PDB).
        0x04 locPt: Inline<NiPoint3>,
        /// `fZRot` (Xbox PDB).
        0x10 fZRot: f32,
    }

    /// `ExtraPackageStartLocation` (Xbox PDB), 0x20 bytes.
    pub struct ExtraPackageStartLocation: 0x20 {
        /// `worldLoc` (Xbox PDB).
        0x0C worldLoc: Inline<WorldLocation>,
    }

    /// `ExtraReferencePointer` (Xbox PDB), 0x10 bytes.
    pub struct ExtraReferencePointer: 0x10 {
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x0C pRef: Ptr,
    }

    /// `ExtraPackage` (Xbox PDB), 0x1C bytes.
    pub struct ExtraPackage: 0x1C {
        /// `pPack` (Xbox PDB): `TESPackage*`.
        0x0C pPack: Ptr,
        /// `iindex` (Xbox PDB).
        0x10 iindex: i32,
        /// `pTarg` (Xbox PDB): `TESObjectREFR*`.
        0x14 pTarg: Ptr,
        /// `bActionComplete` (Xbox PDB; a `bool`, kept as the byte passed).
        0x18 bActionComplete: u8,
        /// `bActivated` (Xbox PDB; a `bool`, kept as the byte passed).
        0x19 bActivated: u8,
        /// `bDoneOnce` (Xbox PDB; a `bool`, kept as the byte passed).
        0x1A bDoneOnce: u8,
    }

    /// `ExtraTresPassPackage` (Xbox PDB), 0x10 bytes.
    pub struct ExtraTresPassPackage: 0x10 {
        /// `pPack` (Xbox PDB): `TrespassPackage*`, owned.
        0x0C pPack: Ptr,
    }

    /// `ExtraPlayerCrimeList` (Xbox PDB), 0x10 bytes.
    pub struct ExtraPlayerCrimeList: 0x10 {
        /// `pCrime` (Xbox PDB): `BSSimpleList<Crime *>*`, owned.
        0x0C pCrime: Ptr,
    }

    /// `ExtraPersistentCell` (Xbox PDB), 0x10 bytes.
    pub struct ExtraPersistentCell: 0x10 {
        /// `pPersistentCell` (Xbox PDB): `TESObjectCELL*`.
        0x0C pPersistentCell: Ptr,
    }

    /// `ExtraRagDollData` (Xbox PDB), 0x10 bytes.
    pub struct ExtraRagDollData: 0x10 {
        /// `pRagDollData` (Xbox PDB): `RagDollData*`, owned.
        0x0C pRagDollData: Ptr,
    }

    /// `ExtraEncounterZone` (Xbox PDB), 0x10 bytes.
    pub struct ExtraEncounterZone: 0x10 {
        /// `pZone` (Xbox PDB): `BGSEncounterZone*`.
        0x0C pZone: Ptr,
    }

    /// `ExtraUsedMarkers` (Xbox PDB), 0x10 bytes.
    pub struct ExtraUsedMarkers: 0x10 {
        /// `iUsedMarkers` (Xbox PDB): bit `n` is set when marker `n` is used.
        0x0C iUsedMarkers: u32,
    }

    /// `ExtraReservedMarkers` (Xbox PDB), 0x10 bytes.
    pub struct ExtraReservedMarkers: 0x10 {
        /// `iReservedMarkers` (Xbox PDB).
        0x0C iReservedMarkers: u32,
    }

    /// `ExtraRunOncePacks` (Xbox PDB), 0x10 bytes.
    pub struct ExtraRunOncePacks: 0x10 {
        /// `pPackageList` (Xbox PDB): `BSSimpleList<RunOncePackage *>*`,
        /// owned (the PC build allocates it).
        0x0C pPackageList: Ptr,
    }

    /// `ExtraDistantData` (Xbox PDB), 0x18 bytes.
    pub struct ExtraDistantData: 0x18 {
        /// `LandNormal` (Xbox PDB).
        0x0C LandNormal: Inline<NiPoint3>,
    }

    /// `ExtraEnableStateParent` (Xbox PDB), 0x14 bytes.
    pub struct ExtraEnableStateParent: 0x14 {
        /// `pParent` (Xbox PDB): `TESObjectREFR*`.
        0x0C pParent: Ptr,
        /// `cFlags` (Xbox PDB).
        0x10 cFlags: u8,
    }

    /// `ExtraEnableStateChildren` (Xbox PDB), 0x14 bytes.
    pub struct ExtraEnableStateChildren: 0x14 {
        /// `ChildList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x0C ChildList: Inline<BSSimpleList>,
    }

    /// `ExtraRandomTeleportMarker` (Xbox PDB), 0x10 bytes.
    pub struct ExtraRandomTeleportMarker: 0x10 {
        /// `pMarker` (Xbox PDB): `TESObjectREFR*`.
        0x0C pMarker: Ptr,
    }

    /// `ExtraLinkedRefChildren` (Xbox PDB), 0x14 bytes.
    pub struct ExtraLinkedRefChildren: 0x14 {
        /// `ChildList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x0C ChildList: Inline<BSSimpleList>,
    }

    /// `ExtraLinkedRef` (Xbox PDB), 0x10 bytes.
    pub struct ExtraLinkedRef: 0x10 {
        /// `pLinkedRef` (Xbox PDB): `TESObjectREFR*`.
        0x0C pLinkedRef: Ptr,
    }

    /// `ExtraAshPileRef` (Xbox PDB), 0x10 bytes.
    pub struct ExtraAshPileRef: 0x10 {
        /// `pAshPileRef` (Xbox PDB): `TESObjectREFR*`.
        0x0C pAshPileRef: Ptr,
    }

    /// `REF_ACTIVATE_DATA` (Xbox PDB), 8 bytes.
    pub struct RefActivateData: 0x08 {
        /// `pActivateRef` (Xbox PDB): `TESObjectREFR*`.
        0x00 pActivateRef: Ptr,
        /// `fActivateDelay` (Xbox PDB).
        0x04 fActivateDelay: f32,
    }

    /// `ExtraActivateRefChildren` (Xbox PDB), 0x18 bytes.
    pub struct ExtraActivateRefChildren: 0x18 {
        /// `ChildList` (Xbox PDB): `BSSimpleList<REF_ACTIVATE_DATA *>`.
        0x0C ChildList: Inline<BSSimpleList>,
        /// `fActivateChildrenTimer` (Xbox PDB).
        0x14 fActivateChildrenTimer: f32,
    }

    /// `ExtraActivateRef` (Xbox PDB), 0x20 bytes.
    pub struct ExtraActivateRef: 0x20 {
        /// `ParentList` (Xbox PDB): `BSSimpleList<REF_ACTIVATE_DATA *>`.
        0x0C ParentList: Inline<BSSimpleList>,
        /// `cActivateFlags` (Xbox PDB).
        0x14 cActivateFlags: u8,
        /// `ActivateTextOverride` (Xbox PDB): `BSStringT<char>`.
        0x18 ActivateTextOverride: Inline<BSStringT>,
    }

    /// `ExtraDecalRefs` (Xbox PDB), 0x14 bytes.
    pub struct ExtraDecalRefs: 0x14 {
        /// `DecalRefList` (Xbox PDB): `BSSimpleList<REF_DECAL_DATA *>`.
        0x0C DecalRefList: Inline<BSSimpleList>,
    }
}

layout! {
    /// `REF_DECAL_DATA` (Xbox PDB), 0x1c bytes.
    pub struct RefDecalData: 0x1C {
        /// `pDecalRef` (Xbox PDB): `TESObjectREFR*`; holds the form id until
        /// `ExtraDecalRefs::InitItem` resolves it.
        0x00 pDecalRef: Ptr,
        /// `Intersect` (Xbox PDB).
        0x04 Intersect: Inline<NiPoint3>,
        /// `Normal` (Xbox PDB).
        0x10 Normal: Inline<NiPoint3>,
    }

    /// `REF_REFLECTED_DATA` and `REF_REFLECTOR_DATA` (Xbox PDB, same layout),
    /// 8 bytes.
    pub struct RefReflectData: 0x08 {
        /// `pRef` (Xbox PDB): `TESObjectREFR*`; holds the form id until
        /// `InitItem` resolves it.
        0x00 pRef: Ptr,
        /// `iEffectFlags` (Xbox PDB).
        0x04 iEffectFlags: u32,
    }

    /// `ExtraReflectedRefs` (Xbox PDB), 0x14 bytes.
    pub struct ExtraReflectedRefs: 0x14 {
        /// `RefList` (Xbox PDB): `BSSimpleList<REF_REFLECTED_DATA *>`, the
        /// entries owned.
        0x0C RefList: Inline<BSSimpleList>,
    }

    /// `ExtraReflectorRefs` (Xbox PDB), 0x14 bytes.
    pub struct ExtraReflectorRefs: 0x14 {
        /// `RefList` (Xbox PDB): `BSSimpleList<REF_REFLECTOR_DATA *>`, the
        /// entries owned.
        0x0C RefList: Inline<BSSimpleList>,
    }

    /// `ExtraWaterLightRefs` (Xbox PDB), 0x14 bytes.
    pub struct ExtraWaterLightRefs: 0x14 {
        /// `RefList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x0C RefList: Inline<BSSimpleList>,
    }

    /// `ExtraLitWaterRefs` (Xbox PDB), 0x14 bytes.
    pub struct ExtraLitWaterRefs: 0x14 {
        /// `RefList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x0C RefList: Inline<BSSimpleList>,
    }

    /// `ExtraMerchantContainer` (Xbox PDB), 0x10 bytes.
    pub struct ExtraMerchantContainer: 0x10 {
        /// `pContainer` (Xbox PDB): `TESObjectREFR*`.
        0x0C pContainer: Ptr,
    }

    /// `ExtraLevCreaModifier` (Xbox PDB), 0x10 bytes.
    pub struct ExtraLevCreaModifier: 0x10 {
        /// `eModifier` (Xbox PDB): a `LEV_CREA_MODIFIER`, read as a 32-bit word.
        0x0C eModifier: u32,
    }

    /// `ExtraPoison` (Xbox PDB), 0x10 bytes.
    pub struct ExtraPoison: 0x10 {
        /// `pPoison` (Xbox PDB): `AlchemyItem*`.
        0x0C pPoison: Ptr,
    }

    /// `ExtraLastFinishedSequence` (Xbox PDB), 0x10 bytes.
    pub struct ExtraLastFinishedSequence: 0x10 {
        /// `pLastSequenceName` (Xbox PDB): `char*`, owned (allocated).
        0x0C pLastSequenceName: Ptr,
    }

    /// `ExtraXTarget` (Xbox PDB), 0x10 bytes.
    pub struct ExtraXTarget: 0x10 {
        /// `pTarget` (Xbox PDB): `TESObjectREFR*`.
        0x0C pTarget: Ptr,
    }
}

/// The count of a `BSSimpleArray`.
pub(crate) fn array_size(e: &mut Engine, array: Ptr<BSSimpleArray>) -> u32 {
    e.call(SIMPLE_ARRAY_SIZE, &args![array]).u32()
}

/// The address of slot `index` of a `BSSimpleArray`.
pub(crate) fn array_slot(e: &mut Engine, array: Ptr<BSSimpleArray>, index: u32) -> Ptr {
    e.call(SIMPLE_ARRAY_AT, &args![array, index]).ptr()
}

/// The pointer stored in slot `index` of a `BSSimpleArray` of pointers.
pub(crate) fn array_pointer_at<T>(e: &mut Engine, array: Ptr<BSSimpleArray>, index: u32) -> Ptr<T> {
    let slot = array_slot(e, array, index);
    Ptr::new(e.mem.u32(slot.addr()))
}

/// `_ftol2_sse` on a `float`.
pub(crate) fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

/// The base constructor, the vtable store that follows it.
pub(crate) fn construct_base(e: &mut Engine, this: Ptr, extra_type: u32, vtable: u32) {
    e.call(BS_EXTRA_DATA_CONSTRUCT, &args![this, extra_type]);
    e.mem.set_u32(this.addr(), vtable);
}

/// `operator delete(this)` when bit 0 of `flags` is set (the tail of every
/// scalar deleting destructor).
pub(crate) fn delete_when_asked(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
}

/// The start every `Compare` below shares: casts `other` to the class with
/// the RTTI descriptor `target_type` and asks `BSExtraData::Compare`. `None`
/// means the answer is already true (`other` is not of that class, or the base
/// says they differ); `Some(cast)` is `other` as the class, whose own fields
/// remain to be compared.
pub(crate) fn compare_prologue<T, U>(
    e: &mut Engine,
    this: Ptr<T>,
    other: Ptr,
    target_type: u32,
) -> Option<Ptr<U>> {
    let cast: Ptr<U> = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, BS_EXTRA_DATA_TYPE, target_type, 0u32],
        )
        .ptr();
    if cast.is_null() {
        return None;
    }
    if e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return None;
    }
    Some(cast)
}

/// The destructor body of the classes that own one object through the pointer
/// at +0xc: resets the vtable, runs the owned object's scalar deleting
/// destructor (`deleter`, with its flag set to 1) when there is one, then the
/// base destructor. The exception-unwinding frame is not translated.
pub(crate) fn destroy_owner(e: &mut Engine, this: Ptr, vtable: u32, deleter: u32) {
    e.mem.set_u32(this.addr(), vtable);
    let owned = Ptr::<()>::new(e.mem.u32(this.addr() + 0xc));
    if !owned.is_null() {
        e.call(deleter, &args![owned, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004300f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAnim::_scalar_deleting_destructor_` (Xbox PDB): the destructor, then
/// `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_anim_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraAnim>,
    flags: u32,
) -> Ptr<ExtraAnim> {
    fn_00430120(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00430120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAnim`'s destructor body (the engine map has no name for it): resets
/// the vtable, deletes the owned animation if there is one, then runs the
/// base destructor. The exception-unwinding frame is not translated.
pub fn fn_00430120(e: &mut Engine, this: Ptr<ExtraAnim>) {
    e.mem.set_u32(this.addr(), EXTRA_ANIM_VTABLE);
    let animation = e.get(this, ExtraAnim::pAnimation);
    if !animation.is_null() {
        e.call(ANIMATION_DELETE, &args![animation, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004301b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `u16` at `this + 0x4c + 2 * slot` of an animation-sequence object, where
/// `slot` is the section number except that section 0x14 (the whole body) uses
/// slot 1 and section 0x15 (the upper body) uses slot 4. The class is not named
/// by the Xbox PDB or the engine map.
pub fn fn_004301b0(e: &mut Engine, this: Ptr, section: i32) -> u16 {
    let slot = match section {
        0x14 => 1,
        0x15 => 4,
        other => other,
    };
    e.mem.u16(
        this.addr()
            .wrapping_add(0x4c)
            .wrapping_add((slot as u32).wrapping_mul(2)),
    )
}

// Translated from 00430200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDismemberedLimbs::ExtraDismemberedLimbs` (Xbox PDB): extra-data type
/// 0x5f, an empty limb array, no limbs dismembered, no cause of death
/// (`eCauseofDeath` and `eLastHitLimb` are -1), not eaten. Returns `this`.
pub fn extra_dismembered_limbs_extra_dismembered_limbs(
    e: &mut Engine,
    this: Ptr<ExtraDismemberedLimbs>,
) -> Ptr<ExtraDismemberedLimbs> {
    construct_base(
        e,
        this.cast(),
        TYPE_DISMEMBERED_LIMBS,
        EXTRA_DISMEMBERED_LIMBS_VTABLE,
    );
    let array = this.at(ExtraDismemberedLimbs::DismemberedLimbs);
    e.call(DISMEMBERED_LIMBS_ARRAY_CONSTRUCT, &args![array]);
    e.set(this, ExtraDismemberedLimbs::sLimbs, 0);
    e.set(this, ExtraDismemberedLimbs::eCauseofDeath, 0xffff_ffff);
    e.set(this, ExtraDismemberedLimbs::pDeathObject, Ptr::NULL);
    e.set(this, ExtraDismemberedLimbs::eLastHitLimb, 0xffff_ffff);
    e.set(this, ExtraDismemberedLimbs::bEaten, false);
    this
}

// Translated from 004302a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDismemberedLimbs::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_dismembered_limbs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraDismemberedLimbs>,
    flags: u32,
) -> Ptr<ExtraDismemberedLimbs> {
    fn_004302d0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004302d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDismemberedLimbs`' destructor body: deletes every non-null
/// `DismemberedLimb` in the array (the count is read again on every pass, as
/// the loop condition does), destroys the array, then runs the base
/// destructor. The exception-unwinding frame is not translated.
pub fn fn_004302d0(e: &mut Engine, this: Ptr<ExtraDismemberedLimbs>) {
    e.mem.set_u32(this.addr(), EXTRA_DISMEMBERED_LIMBS_VTABLE);
    let array = this.at(ExtraDismemberedLimbs::DismemberedLimbs);
    let mut index = 0;
    while index < array_size(e, array) {
        let limb: Ptr<DismemberedLimb> = array_pointer_at(e, array, index);
        if !limb.is_null() {
            fn_00430390(e, limb, 1);
        }
        index += 1;
    }
    e.call(DISMEMBERED_LIMBS_ARRAY_DESTRUCT, &args![array]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00430390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DismemberedLimb`'s scalar deleting destructor (the engine map has no
/// name): the destructor, then `operator delete` when `flags & 1`. Returns
/// `this`.
pub fn fn_00430390(e: &mut Engine, this: Ptr<DismemberedLimb>, flags: u32) -> Ptr<DismemberedLimb> {
    fn_004303c0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004303c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DismemberedLimb`'s destructor: destroys the `ObjectArray` member.
pub fn fn_004303c0(e: &mut Engine, this: Ptr<DismemberedLimb>) {
    let array = this.at(DismemberedLimb::ObjectArray);
    e.call(DISMEMBERED_LIMB_ARRAY_DESTRUCT, &args![array]);
}

// Translated from 004303e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDismemberedLimbs::Dismembered` (Xbox PDB): whether bit `limb` of
/// `sLimbs` is set. The shift count is taken modulo 32, as `SHL` does.
pub fn extra_dismembered_limbs_dismembered(
    e: &mut Engine,
    this: Ptr<ExtraDismemberedLimbs>,
    limb: u8,
) -> bool {
    let limbs = e.get(this, ExtraDismemberedLimbs::sLimbs) as u32;
    limbs & (1u32 << (limb & 31)) != 0
}

// Translated from 00430410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDismemberedLimbs::Dismember` (Xbox PDB): records that `limb` of
/// `actor` (a `TESObjectREFR*`) is dismembered, unless it already is.
///
/// A new `DismemberedLimb` (limb, `exploded`, not identical) is appended to
/// the array. An exploded limb is simply marked identical. Otherwise, for a
/// non-player actor whose vtable `+0x100` and `+0x218` functions are both
/// true, the actor's object array is built into the new entry, and if the
/// most recent earlier entry that is not itself marked identical has the same
/// object array, the new entry is marked identical and its array cleared
/// (`008454f0(array, 1)`).
///
/// The exception-unwinding frame is not translated.
pub fn extra_dismembered_limbs_dismember(
    e: &mut Engine,
    this: Ptr<ExtraDismemberedLimbs>,
    actor: Ptr,
    limb: u8,
    exploded: u8,
) {
    if extra_dismembered_limbs_dismembered(e, this, limb) {
        return;
    }
    let limbs = e.get(this, ExtraDismemberedLimbs::sLimbs) as u32;
    e.set(
        this,
        ExtraDismemberedLimbs::sLimbs,
        (limbs | (1u32 << (limb & 31))) as u16,
    );

    let memory = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let entry: Ptr<DismemberedLimb> = if memory != 0 {
        e.call(DISMEMBERED_LIMB_CONSTRUCT, &args![memory]).ptr()
    } else {
        Ptr::NULL
    };
    let array = this.at(ExtraDismemberedLimbs::DismemberedLimbs);
    // The game appends the pointer held in a stack local and then reads the
    // local again to fill the entry in.
    let (entry, previous_count) = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), entry.addr());
        let count = e.call(SIMPLE_ARRAY_ADD, &args![array, local]).u32();
        (Ptr::<DismemberedLimb>::new(e.mem.u32(local.addr())), count)
    });
    e.set(entry, DismemberedLimb::cLimb, limb);
    e.set(entry, DismemberedLimb::bLimbExploded, exploded);
    e.set(entry, DismemberedLimb::bObjectArrayIdentical, 0);

    if exploded != 0 {
        e.set(entry, DismemberedLimb::bObjectArrayIdentical, 1);
        return;
    }
    if !e.vcall(actor.addr(), 0x100, &[]).bool() {
        return;
    }
    if actor.addr() == e.global::<u32>(PLAYER) {
        return;
    }
    if !e.vcall(actor.addr(), 0x218, &[]).bool() {
        return;
    }
    let npc = e.call(REFR_NPC_ACCESSOR, &args![actor]).u32();
    let object_array = entry.at(DismemberedLimb::ObjectArray);
    let base = e.vcall(actor.addr(), 0x1e8, &[]).u32();
    e.call(
        TESNPC_BUILD_OBJECT_ARRAY,
        &args![npc, actor, base, object_array],
    );
    if previous_count == 0 {
        return;
    }
    // The newest earlier entry that is not itself marked identical.
    let mut index = previous_count as i32 - 1;
    let earlier: Ptr<DismemberedLimb> = loop {
        if index < 0 {
            return;
        }
        let candidate: Ptr<DismemberedLimb> = array_pointer_at(e, array, index as u32);
        if e.get(candidate, DismemberedLimb::bObjectArrayIdentical) == 0 {
            break candidate;
        }
        index -= 1;
    };
    let earlier_array = earlier.at(DismemberedLimb::ObjectArray);
    if e.call(
        SIMPLE_ARRAY_COMPARE_BUFFER,
        &args![earlier_array, object_array],
    )
    .bool()
    {
        e.set(entry, DismemberedLimb::bObjectArrayIdentical, 1);
        e.call(SIMPLE_ARRAY_CLEAR, &args![object_array, 1u32]);
    }
}

// Translated from 004305f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bLimbRemoved` to `removed` on every entry whose `cLimb` is `limb`
/// (the Xbox PDB has no name for this method of `ExtraDismemberedLimbs`).
pub fn fn_004305f0(e: &mut Engine, this: Ptr<ExtraDismemberedLimbs>, limb: u32, removed: u8) {
    let array = this.at(ExtraDismemberedLimbs::DismemberedLimbs);
    let mut index = 0;
    while index < array_size(e, array) {
        let entry: Ptr<DismemberedLimb> = array_pointer_at(e, array, index);
        if !entry.is_null() && e.get(entry, DismemberedLimb::cLimb) as u32 == limb {
            e.set(entry, DismemberedLimb::bLimbRemoved, removed);
        }
        index += 1;
    }
}

// Translated from 00430660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDismemberedLimbs::CheckDismemberedLimbsIdentical` (Xbox PDB):
/// whether `other` has the same dismembered limbs, the same number of
/// entries, and entry by entry the same limb, exploded, identical and removed
/// flags and the same object array.
pub fn extra_dismembered_limbs_check_dismembered_limbs_identical(
    e: &mut Engine,
    this: Ptr<ExtraDismemberedLimbs>,
    other: Ptr<ExtraDismemberedLimbs>,
) -> bool {
    if e.get(other, ExtraDismemberedLimbs::sLimbs) != e.get(this, ExtraDismemberedLimbs::sLimbs) {
        return false;
    }
    let other_array = other.at(ExtraDismemberedLimbs::DismemberedLimbs);
    let this_array = this.at(ExtraDismemberedLimbs::DismemberedLimbs);
    if array_size(e, other_array) != array_size(e, this_array) {
        return false;
    }
    let mut index = 0;
    while index < array_size(e, this_array) {
        let mine: Ptr<DismemberedLimb> = array_pointer_at(e, this_array, index);
        let theirs: Ptr<DismemberedLimb> = array_pointer_at(e, other_array, index);
        if e.get(mine, DismemberedLimb::cLimb) != e.get(theirs, DismemberedLimb::cLimb)
            || e.get(mine, DismemberedLimb::bLimbExploded)
                != e.get(theirs, DismemberedLimb::bLimbExploded)
            || e.get(mine, DismemberedLimb::bObjectArrayIdentical)
                != e.get(theirs, DismemberedLimb::bObjectArrayIdentical)
            || e.get(mine, DismemberedLimb::bLimbRemoved)
                != e.get(theirs, DismemberedLimb::bLimbRemoved)
        {
            return false;
        }
        let mine_objects = mine.at(DismemberedLimb::ObjectArray);
        let their_objects = theirs.at(DismemberedLimb::ObjectArray);
        if !e
            .call(
                SIMPLE_ARRAY_COMPARE_BUFFER,
                &args![mine_objects, their_objects],
            )
            .bool()
        {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00430780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraStartingPosition`'s constructor from a reference (the engine map has
/// no name for it): extra-data type 0x0f; the position (the three floats the
/// reference's vtable `+0x1f4` function returns the address of) and the
/// rotation (`00430830` of the reference) are copied into `startPosition`.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_00430780(
    e: &mut Engine,
    this: Ptr<ExtraStartingPosition>,
    reference: Ptr,
) -> Ptr<ExtraStartingPosition> {
    construct_base(
        e,
        this.cast(),
        TYPE_STARTING_POSITION,
        EXTRA_STARTING_POSITION_VTABLE,
    );
    let start = this.at(ExtraStartingPosition::startPosition);
    e.call(FILE_POS_ROT_CONSTRUCT, &args![start]);
    let position = e.vcall(reference.addr(), 0x1f4, &[]).u32();
    let target = start.at(FilePosRot::pos).addr();
    copy_words(e, position, target);
    let rotation = fn_00430830(e, reference);
    let target = start.at(FilePosRot::rot).addr();
    copy_words(e, rotation.addr(), target);
    this
}

/// Copies three 32-bit words (a `NiPoint3`) bit for bit.
pub(crate) fn copy_words(e: &mut Engine, from: u32, to: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00430830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the 12-byte vector at `this + 0x24` (an accessor of the
/// reference passed to `00430780`; the engine map attributes it to this unit
/// by address range only).
pub fn fn_00430830(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x24)
}

// Translated from 00430850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraStartingPosition::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraStartingPosition`, when `BSExtraData::Compare` returns true, or when
/// the 0x18 bytes of `startPosition` differ.
pub fn extra_starting_position_compare(
    e: &mut Engine,
    this: Ptr<ExtraStartingPosition>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraStartingPosition> = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                BS_EXTRA_DATA_TYPE,
                EXTRA_STARTING_POSITION_TYPE,
                0u32
            ],
        )
        .ptr();
    if cast.is_null() {
        return true;
    }
    if e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return true;
    }
    let mine = this.at(ExtraStartingPosition::startPosition);
    let theirs = cast.at(ExtraStartingPosition::startPosition);
    e.call(MEMCMP, &args![mine, theirs, 0x18u32]).u32() != 0
}

// Translated from 004308c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSExtraData` subclass of type 0x49 (vtable
/// `01015b4c`; no Xbox PDB name is known): the word at +0xc is cleared.
/// Returns `this`.
pub fn fn_004308c0(e: &mut Engine, this: Ptr<ExtraType49>) -> Ptr<ExtraType49> {
    construct_base(e, this.cast(), TYPE_49, EXTRA_TYPE_49_VTABLE);
    e.set(this, ExtraType49::unnamed0C, 0);
    this
}

// Translated from 004308f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLight`'s constructor (the engine map has no name for it): extra-data
/// type 0x29, taking ownership of `light`. Returns `this`.
pub fn fn_004308f0(e: &mut Engine, this: Ptr<ExtraLight>, light: Ptr) -> Ptr<ExtraLight> {
    construct_base(e, this.cast(), TYPE_LIGHT, EXTRA_LIGHT_VTABLE);
    e.set(this, ExtraLight::pLight, light);
    this
}

// Translated from 00430920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLight::_scalar_deleting_destructor_` (Xbox PDB): the destructor,
/// then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_light_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraLight>,
    flags: u32,
) -> Ptr<ExtraLight> {
    fn_00430950(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00430950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLight`'s destructor body (the engine map has no name for it): resets
/// the vtable, deletes the owned light if there is one, then runs the base
/// destructor. The exception-unwinding frame is not translated.
pub fn fn_00430950(e: &mut Engine, this: Ptr<ExtraLight>) {
    e.mem.set_u32(this.addr(), EXTRA_LIGHT_VTABLE);
    let light = e.get(this, ExtraLight::pLight);
    if !light.is_null() {
        e.call(LIGHT_DELETE, &args![light, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004309e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `REFR_LOCK::GetLockLevel` (Xbox PDB): the difficulty bracket (0 very easy
/// to 4 very hard, 5 key only) of the lock's level for `reference`.
pub fn refr_lock_get_lock_level(e: &mut Engine, this: Ptr<RefrLock>, reference: Ptr) -> i32 {
    let level = refr_lock_get_level(e, this, reference);
    refr_lock_numeric_value_to_enum(e, level)
}

// Translated from 00430a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `REFR_LOCK::GetLevel` (Xbox PDB): the lock's level. A leveled lock
/// (`cFlags` bit 2) adds the level of `reference` (the player when it is
/// null) times the float setting at `011c39b4` truncated to an integer, and
/// is capped at 99.
pub fn refr_lock_get_level(e: &mut Engine, this: Ptr<RefrLock>, reference: Ptr) -> i32 {
    let mut level = e.get(this, RefrLock::cBaseLevel) as i32;
    if e.get(this, RefrLock::cFlags) as i32 & 4 != 0 {
        let reference_level = if reference.is_null() {
            let player = e.global::<u32>(PLAYER);
            e.call(ACTOR_GET_LEVEL, &args![player]).u16() as i32
        } else {
            e.call(REFR_GET_CALC_LEVEL, &args![reference, 0u32]).i32()
        };
        let setting = e
            .call(SETTING_FLOAT_VALUE, &args![LEVELED_LOCK_SETTING])
            .u32();
        let scale = e.mem.f32(setting);
        let scale = float_to_int(e, scale);
        level = scale.wrapping_mul(reference_level).wrapping_add(level);
        if level > 99 {
            level = 99;
        }
    }
    level
}

// Translated from 00430a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `REFR_LOCK::SetLocked` (Xbox PDB): sets or clears bit 0 of `cFlags`;
/// unlocking also resets `uiNumTries`.
pub fn refr_lock_set_locked(e: &mut Engine, this: Ptr<RefrLock>, locked: bool) {
    let flags = e.get(this, RefrLock::cFlags);
    if locked {
        e.set(this, RefrLock::cFlags, flags | 1);
    } else {
        e.set(this, RefrLock::cFlags, flags & !1);
        e.set(this, RefrLock::uiNumTries, 0);
    }
}

// Translated from 00430ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `REFR_LOCK::IsBroken` (Xbox PDB): whether `uiNumTries` has reached the
/// number of tries the lock takes: 1 when the entry point 0x20 (asked of the
/// player) leaves its value at 0.0, otherwise 2.
pub fn refr_lock_is_broken(e: &mut Engine, this: Ptr<RefrLock>) -> bool {
    let player = e.global::<u32>(PLAYER);
    let value = e.with_stack(4, |e, result| {
        e.mem.set_f32(result.addr(), 0.0);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![ENTRY_POINT_LOCK_BROKEN, player, result],
        );
        e.mem.f32(result.addr())
    });
    let zero: f64 = e.global(ZERO);
    let tries_allowed: u32 = if value as f64 == zero { 1 } else { 2 };
    e.get(this, RefrLock::uiNumTries) >= tries_allowed
}

// Translated from 00430b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `REFR_LOCK::NumericValueToEnum` (Xbox PDB): the first difficulty bracket
/// whose top setting (`LOCK_LEVEL_SETTINGS`) is at least `value`; 5 when
/// `value` passes every one.
pub fn refr_lock_numeric_value_to_enum(e: &mut Engine, value: i32) -> i32 {
    for (bracket, setting) in LOCK_LEVEL_SETTINGS[..5].iter().enumerate() {
        let top = e.call(SETTING_INT_VALUE, &args![*setting]).u32();
        if value <= e.mem.i32(top) {
            return bracket as i32;
        }
    }
    5
}

// Translated from 00430bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `REFR_LOCK::NumericValueToEnum` (no Xbox PDB name): the
/// value of the setting for difficulty `bracket` (0 to 5), 0 for anything
/// else. The switch is the jump table at `00430c50`.
pub fn fn_00430bc0(e: &mut Engine, bracket: u32) -> i32 {
    match LOCK_LEVEL_SETTINGS.get(bracket as usize) {
        Some(setting) => {
            let value = e.call(SETTING_INT_VALUE, &args![*setting]).u32();
            e.mem.i32(value)
        }
        None => 0,
    }
}

// Translated from 00430c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLock::ExtraLock` (Xbox PDB): extra-data type 0x2a, taking ownership
/// of `lock`. Returns `this`.
pub fn extra_lock_extra_lock(
    e: &mut Engine,
    this: Ptr<ExtraLock>,
    lock: Ptr<RefrLock>,
) -> Ptr<ExtraLock> {
    construct_base(e, this.cast(), TYPE_LOCK, EXTRA_LOCK_VTABLE);
    e.set(this, ExtraLock::pLock, lock);
    this
}

// Translated from 00430ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLock`'s destructor body (the engine map has no name for it; the
/// scalar deleting destructor is `0042c550`, in `extradatalist.cpp`): resets
/// the vtable, frees the owned lock (without a null check), then runs the base
/// destructor.
pub fn fn_00430ca0(e: &mut Engine, this: Ptr<ExtraLock>) {
    e.mem.set_u32(this.addr(), EXTRA_LOCK_VTABLE);
    let lock = e.get(this, ExtraLock::pLock);
    e.call(OPERATOR_DELETE, &args![lock]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00430ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLock::Compare` (Xbox PDB): true when `other` is not an `ExtraLock`,
/// when `BSExtraData::Compare` returns true, or when the two locks differ
/// in `cFlags`, `pKey`, difficulty bracket of `cBaseLevel`, `uiNumTries` or
/// `uiTimesUnlocked`.
pub fn extra_lock_compare(e: &mut Engine, this: Ptr<ExtraLock>, other: Ptr) -> bool {
    let cast: Ptr<ExtraLock> = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, BS_EXTRA_DATA_TYPE, EXTRA_LOCK_TYPE, 0u32],
        )
        .ptr();
    if cast.is_null() {
        return true;
    }
    if e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return true;
    }
    let mine = e.get(this, ExtraLock::pLock);
    let theirs = e.get(cast, ExtraLock::pLock);
    if e.get(mine, RefrLock::cFlags) != e.get(theirs, RefrLock::cFlags) {
        return true;
    }
    if e.get(mine, RefrLock::pKey) != e.get(theirs, RefrLock::pKey) {
        return true;
    }
    let my_bracket = refr_lock_numeric_value_to_enum(e, e.get(mine, RefrLock::cBaseLevel) as i32);
    let their_bracket =
        refr_lock_numeric_value_to_enum(e, e.get(theirs, RefrLock::cBaseLevel) as i32);
    if my_bracket != their_bracket {
        return true;
    }
    if e.get(mine, RefrLock::uiNumTries) != e.get(theirs, RefrLock::uiNumTries) {
        return true;
    }
    e.get(mine, RefrLock::uiTimesUnlocked) != e.get(theirs, RefrLock::uiTimesUnlocked)
}

// Translated from 00430dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFollower::ExtraFollower` (Xbox PDB): extra-data type 0x1d and a
/// newly allocated, constructed 8-byte actor list (null when the allocation
/// fails). Returns `this`. The exception-unwinding frame is not translated.
pub fn extra_follower_extra_follower(
    e: &mut Engine,
    this: Ptr<ExtraFollower>,
) -> Ptr<ExtraFollower> {
    construct_base(e, this.cast(), TYPE_FOLLOWER, EXTRA_FOLLOWER_VTABLE);
    let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list: Ptr = if memory != 0 {
        e.call(ACTOR_LIST_CONSTRUCT, &args![memory]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, ExtraFollower::pActorlist, list);
    this
}

// Translated from 00430e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFollower::_scalar_deleting_destructor_` (Xbox PDB): the destructor,
/// then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_follower_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraFollower>,
    flags: u32,
) -> Ptr<ExtraFollower> {
    fn_00430ea0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00430ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFollower`'s destructor body (the engine map has no name for it):
/// resets the vtable, clears the actor list (called even when the pointer is
/// null), deletes it if there is one, then runs the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_00430ea0(e: &mut Engine, this: Ptr<ExtraFollower>) {
    e.mem.set_u32(this.addr(), EXTRA_FOLLOWER_VTABLE);
    let list = e.get(this, ExtraFollower::pActorlist);
    e.call(ACTOR_LIST_CLEAR, &args![list]);
    let list = e.get(this, ExtraFollower::pActorlist);
    if !list.is_null() {
        e.call(ACTOR_LIST_DELETE, &args![list, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00430f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGuardedRefData::ExtraGuardedRefData` (Xbox PDB): extra-data type
/// 0x7c and an empty `Guards` array. Returns `this`. The exception-unwinding
/// frame is not translated.
pub fn extra_guarded_ref_data_extra_guarded_ref_data(
    e: &mut Engine,
    this: Ptr<ExtraGuardedRefData>,
) -> Ptr<ExtraGuardedRefData> {
    construct_base(
        e,
        this.cast(),
        TYPE_GUARDED_REF_DATA,
        EXTRA_GUARDED_REF_DATA_VTABLE,
    );
    let guards = this.at(ExtraGuardedRefData::Guards);
    e.call(GUARDS_ARRAY_CONSTRUCT, &args![guards]);
    this
}

// Translated from 00430fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGuardedRefData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_guarded_ref_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraGuardedRefData>,
    flags: u32,
) -> Ptr<ExtraGuardedRefData> {
    fn_00430fd0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00430fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGuardedRefData`'s destructor body (the engine map has no name for
/// it): resets the vtable, destroys the `Guards` array, then runs the base
/// destructor. The exception-unwinding frame is not translated.
pub fn fn_00430fd0(e: &mut Engine, this: Ptr<ExtraGuardedRefData>) {
    e.mem.set_u32(this.addr(), EXTRA_GUARDED_REF_DATA_VTABLE);
    let guards = this.at(ExtraGuardedRefData::Guards);
    e.call(GUARDS_ARRAY_DESTRUCT, &args![guards]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00431030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGuardedRefData::Compare` (Xbox PDB): true when
/// `BSExtraData::Compare` returns false (the opposite test to the other
/// `Compare` methods of this unit, as the code has it), when the guard counts
/// differ, or when some guard of `this` is not in `other`'s array. `other` is
/// used as an `ExtraGuardedRefData` without a type check.
pub fn extra_guarded_ref_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraGuardedRefData>,
    other: Ptr<ExtraGuardedRefData>,
) -> bool {
    if !e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return true;
    }
    let their_guards = other.at(ExtraGuardedRefData::Guards);
    let my_guards = this.at(ExtraGuardedRefData::Guards);
    if array_size(e, their_guards) != array_size(e, my_guards) {
        return true;
    }
    let mut index = 0;
    while index < array_size(e, my_guards) {
        let guard = array_slot(e, my_guards, index);
        let found = e
            .call(
                SIMPLE_ARRAY_FIND,
                &args![their_guards, guard, 0u32, GUARD_COMPARE],
            )
            .u32();
        if found == 0xffff_ffff {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 004310d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGuardedRefData::AddGuard` (Xbox PDB): appends the value
/// `0084e3a0` gives for `reference` to `Guards` unless it is already there.
pub fn extra_guarded_ref_data_add_guard(
    e: &mut Engine,
    this: Ptr<ExtraGuardedRefData>,
    reference: Ptr,
) {
    let guard = e.call(REFR_GUARD_VALUE, &args![reference]).u32();
    let guards = this.at(ExtraGuardedRefData::Guards);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), guard);
        let found = e
            .call(
                SIMPLE_ARRAY_FIND,
                &args![guards, local, 0u32, GUARD_COMPARE],
            )
            .u32();
        if found == 0xffff_ffff {
            e.call(SIMPLE_ARRAY_ADD, &args![guards, local]);
        }
    });
}

// Translated from 00431120 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every guard in `Guards` (the engine map has no name for this method of
/// `ExtraGuardedRefData`): looks the form up, and when it is a form whose
/// vtable `+0xf0` and `+0x100` functions are both true and `008d8520` gives it
/// a process, calls that process's vtable `+0x60c` function with the form,
/// `second` and `first`.
pub fn fn_00431120(e: &mut Engine, this: Ptr<ExtraGuardedRefData>, first: u32, second: u32) {
    let guards = this.at(ExtraGuardedRefData::Guards);
    let mut index = 0;
    while index < array_size(e, guards) {
        let id = array_pointer_at::<()>(e, guards, index).addr();
        let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
        if form != 0 && e.vcall(form, 0xf0, &[]).bool() && e.vcall(form, 0x100, &[]).bool() {
            let process = e.call(FORM_PROCESS, &args![form]).u32();
            if process != 0 {
                e.vcall(process, 0x60c, &args![form, second, first]);
            }
        }
        index += 1;
    }
}

// Translated from 004311f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSExtraData` subclass of type 0x2e (vtable
/// `01015b7c`; no Xbox PDB name is known): the two words at +0xc and +0x10
/// are cleared. Returns `this`.
pub fn fn_004311f0(e: &mut Engine, this: Ptr<ExtraType2e>) -> Ptr<ExtraType2e> {
    construct_base(e, this.cast(), TYPE_2E, EXTRA_TYPE_2E_VTABLE);
    e.set(this, ExtraType2e::unnamed0C, 0);
    e.set(this, ExtraType2e::unnamed10, 0);
    this
}

// Translated from 00431230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTeleport`'s constructor (the engine map has no name for it):
/// extra-data type 0x2b, taking ownership of `data`. Returns `this`.
pub fn fn_00431230(e: &mut Engine, this: Ptr<ExtraTeleport>, data: Ptr) -> Ptr<ExtraTeleport> {
    construct_base(e, this.cast(), TYPE_TELEPORT, EXTRA_TELEPORT_VTABLE);
    e.set(this, ExtraTeleport::pData, data);
    this
}

// Translated from 00431260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTeleport`'s destructor body (the engine map has no name for it; the
/// scalar deleting destructor is `0042c5b0`, in `extradatalist.cpp`): resets
/// the vtable, deletes the owned teleport data if there is any, then runs the
/// base destructor. The exception-unwinding frame is not translated.
pub fn fn_00431260(e: &mut Engine, this: Ptr<ExtraTeleport>) {
    e.mem.set_u32(this.addr(), EXTRA_TELEPORT_VTABLE);
    let data = e.get(this, ExtraTeleport::pData);
    if !data.is_null() {
        e.call(TELEPORT_DATA_DELETE, &args![data, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004312f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTeleport::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraTeleport`, when `BSExtraData::Compare` returns true, or when
/// `DoorTeleportData::Compare` (Xbox PDB) says the two teleport data differ.
pub fn extra_teleport_compare(e: &mut Engine, this: Ptr<ExtraTeleport>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraTeleport>(e, this, other, EXTRA_TELEPORT_TYPE)
    else {
        return true;
    };
    let mine = e.get(this, ExtraTeleport::pData);
    let theirs = e.get(cast, ExtraTeleport::pData);
    e.call(DOOR_TELEPORT_DATA_COMPARE, &args![mine, theirs])
        .bool()
}

// Translated from 00431360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMapMarker`'s constructor (the engine map has no name for it; the class
/// is the one whose vtable `01015b88` it stores): extra-data type 0x2c, taking
/// ownership of `data`. Returns `this`.
pub fn fn_00431360(e: &mut Engine, this: Ptr<ExtraMapMarker>, data: Ptr) -> Ptr<ExtraMapMarker> {
    construct_base(e, this.cast(), TYPE_MAP_MARKER, EXTRA_MAP_MARKER_VTABLE);
    e.set(this, ExtraMapMarker::pMapData, data);
    this
}

// Translated from 00431390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMapMarker::_scalar_deleting_destructor_` (Xbox PDB): the destructor,
/// then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_map_marker_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraMapMarker>,
    flags: u32,
) -> Ptr<ExtraMapMarker> {
    fn_004313c0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004313c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMapMarker`'s destructor body (the engine map has no name for it):
/// resets the vtable, runs the destructor of the owned `MapMarkerData`
/// (`00419350`) if there is one, then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_004313c0(e: &mut Engine, this: Ptr<ExtraMapMarker>) {
    destroy_owner(
        e,
        this.cast(),
        EXTRA_MAP_MARKER_VTABLE,
        MAP_MARKER_DATA_DELETE,
    );
}

// Translated from 00431450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMapMarker::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraMapMarker`, when `BSExtraData::Compare` returns true, or when
/// `MapMarkerData::Compare` (Xbox PDB) says the two map data differ.
pub fn extra_map_marker_compare(e: &mut Engine, this: Ptr<ExtraMapMarker>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraMapMarker>(e, this, other, EXTRA_MAP_MARKER_TYPE)
    else {
        return true;
    };
    let mine = e.get(this, ExtraMapMarker::pMapData);
    let theirs = e.get(cast, ExtraMapMarker::pMapData);
    e.call(MAP_MARKER_DATA_COMPARE, &args![mine, theirs]).bool()
}

// Translated from 004314c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioMarker`'s constructor (the engine map has no name for it; the
/// class is the one whose vtable `01015b94` it stores): extra-data type 0x90,
/// taking ownership of `data`. Returns `this`.
pub fn fn_004314c0(
    e: &mut Engine,
    this: Ptr<ExtraAudioMarker>,
    data: Ptr,
) -> Ptr<ExtraAudioMarker> {
    construct_base(e, this.cast(), TYPE_AUDIO_MARKER, EXTRA_AUDIO_MARKER_VTABLE);
    e.set(this, ExtraAudioMarker::pAudioData, data);
    this
}

// Translated from 004314f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioMarker::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_audio_marker_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraAudioMarker>,
    flags: u32,
) -> Ptr<ExtraAudioMarker> {
    fn_00431520(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00431520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioMarker`'s destructor body (the engine map has no name for it):
/// resets the vtable, runs the destructor of the owned `AudioMarkerData`
/// (`00419480`) if there is one, then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_00431520(e: &mut Engine, this: Ptr<ExtraAudioMarker>) {
    destroy_owner(
        e,
        this.cast(),
        EXTRA_AUDIO_MARKER_VTABLE,
        AUDIO_MARKER_DATA_DELETE,
    );
}

// Translated from 004315b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioMarker::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraAudioMarker`, when `BSExtraData::Compare` returns true, or when
/// `AudioMarkerData::Compare` (Xbox PDB) says the two audio data differ.
pub fn extra_audio_marker_compare(e: &mut Engine, this: Ptr<ExtraAudioMarker>, other: Ptr) -> bool {
    let Some(cast) =
        compare_prologue::<_, ExtraAudioMarker>(e, this, other, EXTRA_AUDIO_MARKER_TYPE)
    else {
        return true;
    };
    let mine = e.get(this, ExtraAudioMarker::pAudioData);
    let theirs = e.get(cast, ExtraAudioMarker::pAudioData);
    e.call(AUDIO_MARKER_DATA_COMPARE, &args![mine, theirs])
        .bool()
}

// Translated from 00431620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioBuoyMarker`'s constructor (the engine map has no name for it;
/// the class is the one whose vtable `01015ba0` it stores): extra-data type
/// 0x91, taking ownership of `data`. Returns `this`.
pub fn fn_00431620(
    e: &mut Engine,
    this: Ptr<ExtraAudioBuoyMarker>,
    data: Ptr,
) -> Ptr<ExtraAudioBuoyMarker> {
    construct_base(
        e,
        this.cast(),
        TYPE_AUDIO_BUOY_MARKER,
        EXTRA_AUDIO_BUOY_MARKER_VTABLE,
    );
    e.set(this, ExtraAudioBuoyMarker::pAudioBuoyData, data);
    this
}

// Translated from 00431650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioBuoyMarker::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_audio_buoy_marker_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraAudioBuoyMarker>,
    flags: u32,
) -> Ptr<ExtraAudioBuoyMarker> {
    fn_00431680(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00431680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioBuoyMarker`'s destructor body (the engine map has no name for
/// it): resets the vtable, runs the destructor of the owned
/// `AudioBuoyMarkerData` (`007b3fa0`) if there is one, then the base
/// destructor. The exception-unwinding frame is not translated.
pub fn fn_00431680(e: &mut Engine, this: Ptr<ExtraAudioBuoyMarker>) {
    destroy_owner(
        e,
        this.cast(),
        EXTRA_AUDIO_BUOY_MARKER_VTABLE,
        AUDIO_BUOY_DATA_DELETE,
    );
}

// Translated from 00431710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAudioBuoyMarker::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraAudioBuoyMarker`, when `BSExtraData::Compare` returns true, or when
/// the folded body `00401290` (which returns true whatever it is given) says
/// the two buoy data differ; so, past the first two tests, it is always true.
pub fn extra_audio_buoy_marker_compare(
    e: &mut Engine,
    this: Ptr<ExtraAudioBuoyMarker>,
    other: Ptr,
) -> bool {
    let Some(cast) =
        compare_prologue::<_, ExtraAudioBuoyMarker>(e, this, other, EXTRA_AUDIO_BUOY_MARKER_TYPE)
    else {
        return true;
    };
    let mine = e.get(this, ExtraAudioBuoyMarker::pAudioBuoyData);
    let theirs = e.get(cast, ExtraAudioBuoyMarker::pAudioBuoyData);
    e.call(AUDIO_BUOY_DATA_COMPARE, &args![mine, theirs]).bool()
}

// Translated from 00431780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAction`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `01015bac` it stores): extra-data type 0x0e, `eAction`
/// set to 1 (only that byte) and `pActionRef` cleared. Returns `this`.
pub fn fn_00431780(e: &mut Engine, this: Ptr<ExtraAction>) -> Ptr<ExtraAction> {
    construct_base(e, this.cast(), TYPE_ACTION, EXTRA_ACTION_VTABLE);
    e.set(this, ExtraAction::eAction, 1);
    e.set(this, ExtraAction::pActionRef, Ptr::NULL);
    this
}

// Translated from 004317c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAction::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraAction`, when `BSExtraData::Compare` returns true, or when
/// `eAction` differs.
pub fn extra_action_compare(e: &mut Engine, this: Ptr<ExtraAction>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraAction>(e, this, other, EXTRA_ACTION_TYPE) else {
        return true;
    };
    e.get(this, ExtraAction::eAction) != e.get(cast, ExtraAction::eAction)
}

// Translated from 00431830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraContainerChanges`'s constructor (the engine map has no name for it;
/// the class is the one whose vtable `01015bb8` it stores): extra-data type
/// 0x15, taking ownership of `changes`. Returns `this`.
pub fn fn_00431830(
    e: &mut Engine,
    this: Ptr<ExtraContainerChanges>,
    changes: Ptr,
) -> Ptr<ExtraContainerChanges> {
    construct_base(
        e,
        this.cast(),
        TYPE_CONTAINER_CHANGES,
        EXTRA_CONTAINER_CHANGES_VTABLE,
    );
    e.set(this, ExtraContainerChanges::pChanges, changes);
    this
}

// Translated from 00431860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraContainerChanges::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_container_changes_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraContainerChanges>,
    flags: u32,
) -> Ptr<ExtraContainerChanges> {
    fn_00431890(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00431890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraContainerChanges`'s destructor body (the engine map has no name for
/// it): resets the vtable, runs the scalar deleting destructor of the owned
/// `InventoryChanges` (`00431920`, with its flag set to 1) if there is one,
/// then the base destructor. The exception-unwinding frame is not translated.
pub fn fn_00431890(e: &mut Engine, this: Ptr<ExtraContainerChanges>) {
    e.mem.set_u32(this.addr(), EXTRA_CONTAINER_CHANGES_VTABLE);
    let changes = e.get(this, ExtraContainerChanges::pChanges);
    if !changes.is_null() {
        fn_00431920(e, changes, 1);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00431920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the object `ExtraContainerChanges::pChanges`
/// points at (an `InventoryChanges` by the Xbox PDB's field type; the engine
/// map has no name for it): its destructor body (`004bf150`), then
/// `operator delete` when `flags & 1`. Returns `this`.
pub fn fn_00431920(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(INVENTORY_CHANGES_DESTRUCT, &args![this]);
    delete_when_asked(e, this, flags);
    this
}

// Translated from 00431950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOriginalReference`'s constructor (the engine map has no name for it;
/// the class is the one whose vtable `01015bc4` it stores): extra-data type
/// 0x20, storing `reference`. Returns `this`.
pub fn fn_00431950(
    e: &mut Engine,
    this: Ptr<ExtraOriginalReference>,
    reference: Ptr,
) -> Ptr<ExtraOriginalReference> {
    construct_base(
        e,
        this.cast(),
        TYPE_ORIGINAL_REFERENCE,
        EXTRA_ORIGINAL_REFERENCE_VTABLE,
    );
    e.set(this, ExtraOriginalReference::pReference, reference);
    this
}

// Translated from 00431980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOriginalReference`'s scalar deleting destructor (the engine map has no
/// name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_00431980(
    e: &mut Engine,
    this: Ptr<ExtraOriginalReference>,
    flags: u32,
) -> Ptr<ExtraOriginalReference> {
    fn_004319b0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004319b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOriginalReference`'s destructor body (the engine map has no name for
/// it): resets the vtable, then runs the base destructor.
pub fn fn_004319b0(e: &mut Engine, this: Ptr<ExtraOriginalReference>) {
    e.mem.set_u32(this.addr(), EXTRA_ORIGINAL_REFERENCE_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004319d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOriginalReference::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraOriginalReference`, when `BSExtraData::Compare` returns true, or when
/// `pReference` differs.
pub fn extra_original_reference_compare(
    e: &mut Engine,
    this: Ptr<ExtraOriginalReference>,
    other: Ptr,
) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraOriginalReference>(
        e,
        this,
        other,
        EXTRA_ORIGINAL_REFERENCE_TYPE,
    ) else {
        return true;
    };
    e.get(this, ExtraOriginalReference::pReference)
        != e.get(cast, ExtraOriginalReference::pReference)
}

// Translated from 00431a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOwnership`'s constructor (the engine map has no name for it; the class
/// is the one whose vtable `010158b4` it stores): extra-data type 0x21,
/// storing `owner`. Returns `this`.
pub fn fn_00431a40(e: &mut Engine, this: Ptr<ExtraOwnership>, owner: Ptr) -> Ptr<ExtraOwnership> {
    construct_base(e, this.cast(), TYPE_OWNERSHIP, EXTRA_OWNERSHIP_VTABLE);
    e.set(this, ExtraOwnership::pOwner, owner);
    this
}

// Translated from 00431a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOwnership::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraOwnership`, when `BSExtraData::Compare` returns true, or when
/// `pOwner` differs.
pub fn extra_ownership_compare(e: &mut Engine, this: Ptr<ExtraOwnership>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraOwnership>(e, this, other, EXTRA_OWNERSHIP_TYPE)
    else {
        return true;
    };
    e.get(this, ExtraOwnership::pOwner) != e.get(cast, ExtraOwnership::pOwner)
}

// Translated from 00431ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGlobal`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `010158c0` it stores): extra-data type 0x22, storing
/// `global`. Returns `this`.
pub fn fn_00431ae0(e: &mut Engine, this: Ptr<ExtraGlobal>, global: Ptr) -> Ptr<ExtraGlobal> {
    construct_base(e, this.cast(), TYPE_GLOBAL, EXTRA_GLOBAL_VTABLE);
    e.set(this, ExtraGlobal::pGlobal, global);
    this
}

// Translated from 00431b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGlobal::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraGlobal`, when `BSExtraData::Compare` returns true, or when `pGlobal`
/// differs.
pub fn extra_global_compare(e: &mut Engine, this: Ptr<ExtraGlobal>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraGlobal>(e, this, other, EXTRA_GLOBAL_TYPE) else {
        return true;
    };
    e.get(this, ExtraGlobal::pGlobal) != e.get(cast, ExtraGlobal::pGlobal)
}

// Translated from 00431b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRank`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `010158cc` it stores): extra-data type 0x23, storing
/// `rank`. Returns `this`.
pub fn fn_00431b80(e: &mut Engine, this: Ptr<ExtraRank>, rank: i32) -> Ptr<ExtraRank> {
    construct_base(e, this.cast(), TYPE_RANK, EXTRA_RANK_VTABLE);
    e.set(this, ExtraRank::iRank, rank);
    this
}

// Translated from 00431bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRank::Compare` (Xbox PDB): true when `other` is not an `ExtraRank`,
/// when `BSExtraData::Compare` returns true, or when `iRank` differs.
pub fn extra_rank_compare(e: &mut Engine, this: Ptr<ExtraRank>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraRank>(e, this, other, EXTRA_RANK_TYPE) else {
        return true;
    };
    e.get(this, ExtraRank::iRank) != e.get(cast, ExtraRank::iRank)
}

// Translated from 00431c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCount`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `010158d8` it stores): extra-data type 0x24, storing
/// the 16-bit `count` (the two bytes above it are left alone). Returns `this`.
pub fn fn_00431c20(e: &mut Engine, this: Ptr<ExtraCount>, count: i16) -> Ptr<ExtraCount> {
    construct_base(e, this.cast(), TYPE_COUNT, EXTRA_COUNT_VTABLE);
    e.set(this, ExtraCount::iCount, count);
    this
}

// Translated from 00431c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCount::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraCount`, when `BSExtraData::Compare` returns true, or when `iCount`
/// differs.
pub fn extra_count_compare(e: &mut Engine, this: Ptr<ExtraCount>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraCount>(e, this, other, EXTRA_COUNT_TYPE) else {
        return true;
    };
    e.get(this, ExtraCount::iCount) != e.get(cast, ExtraCount::iCount)
}

// Translated from 00431cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLeveledItem::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraLeveledItem` or when `BSExtraData::Compare` returns true; the
/// object's own fields (`iIndex`, `bdefault`) are not compared.
pub fn extra_leveled_item_compare(e: &mut Engine, this: Ptr<ExtraLeveledItem>, other: Ptr) -> bool {
    compare_prologue::<_, ExtraLeveledItem>(e, this, other, EXTRA_LEVELED_ITEM_TYPE).is_none()
}

// Translated from 00431d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHealth`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `010158e4` it stores): extra-data type 0x25, storing
/// `health` through x87 FLD/FSTP (signaling NaNs become quiet). Returns `this`.
pub fn fn_00431d10(e: &mut Engine, this: Ptr<ExtraHealth>, health: f32) -> Ptr<ExtraHealth> {
    construct_base(e, this.cast(), TYPE_HEALTH, EXTRA_HEALTH_VTABLE);
    let bits = x87_quiet_bits(health.to_bits());
    e.set(this, ExtraHealth::fHealth, f32::from_bits(bits));
    this
}

// Translated from 00431d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHealth::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraHealth`, when `BSExtraData::Compare` returns true, or when `fHealth`
/// differs (as `float`s: a NaN differs from everything, `0.0` equals `-0.0`).
pub fn extra_health_compare(e: &mut Engine, this: Ptr<ExtraHealth>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraHealth>(e, this, other, EXTRA_HEALTH_TYPE) else {
        return true;
    };
    e.get(this, ExtraHealth::fHealth) != e.get(cast, ExtraHealth::fHealth)
}

// Translated from 00431db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHealthPerc::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraHealthPerc`, when `BSExtraData::Compare` returns true, or when
/// `fHealthPerc` differs (as `float`s: a NaN differs from everything, `0.0`
/// equals `-0.0`).
pub fn extra_health_perc_compare(e: &mut Engine, this: Ptr<ExtraHealthPerc>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraHealthPerc>(e, this, other, EXTRA_HEALTH_PERC_TYPE)
    else {
        return true;
    };
    e.get(this, ExtraHealthPerc::fHealthPerc) != e.get(cast, ExtraHealthPerc::fHealthPerc)
}

// Translated from 00431e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraUses`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `010158f0` it stores): extra-data type 0x26, storing
/// the byte `uses` (the three bytes above it are left alone). Returns `this`.
pub fn fn_00431e20(e: &mut Engine, this: Ptr<ExtraUses>, uses: u8) -> Ptr<ExtraUses> {
    construct_base(e, this.cast(), TYPE_USES, EXTRA_USES_VTABLE);
    e.set(this, ExtraUses::cUses, uses);
    this
}

// Translated from 00431e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraUses::Compare` (Xbox PDB): true when `other` is not an `ExtraUses`,
/// when `BSExtraData::Compare` returns true, or when `cUses` differs.
pub fn extra_uses_compare(e: &mut Engine, this: Ptr<ExtraUses>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraUses>(e, this, other, EXTRA_USES_TYPE) else {
        return true;
    };
    e.get(this, ExtraUses::cUses) != e.get(cast, ExtraUses::cUses)
}

// Translated from 00431ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTimeLeft`'s constructor (the engine map has no name for it; the class
/// is the one whose vtable `010158fc` it stores): extra-data type 0x27,
/// storing `time`. Returns `this`.
pub fn fn_00431ec0(e: &mut Engine, this: Ptr<ExtraTimeLeft>, time: f32) -> Ptr<ExtraTimeLeft> {
    construct_base(e, this.cast(), TYPE_TIME_LEFT, EXTRA_TIME_LEFT_VTABLE);
    e.set(this, ExtraTimeLeft::fTime, time);
    this
}

// Translated from 00431ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTimeLeft::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraTimeLeft`, when `BSExtraData::Compare` returns true, or when `fTime`
/// differs (as `float`s: a NaN differs from everything, `0.0` equals `-0.0`).
pub fn extra_time_left_compare(e: &mut Engine, this: Ptr<ExtraTimeLeft>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraTimeLeft>(e, this, other, EXTRA_TIME_LEFT_TYPE)
    else {
        return true;
    };
    e.get(this, ExtraTimeLeft::fTime) != e.get(cast, ExtraTimeLeft::fTime)
}

// Translated from 00431f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCharge`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `01015908` it stores): extra-data type 0x28, storing
/// `charge`. Returns `this`.
pub fn fn_00431f60(e: &mut Engine, this: Ptr<ExtraCharge>, charge: f32) -> Ptr<ExtraCharge> {
    construct_base(e, this.cast(), TYPE_CHARGE, EXTRA_CHARGE_VTABLE);
    e.set(this, ExtraCharge::fCharge, charge);
    this
}

// Translated from 00431f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCharge::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraCharge`, when `BSExtraData::Compare` returns true, or when `fCharge`
/// differs (as `float`s: a NaN differs from everything, `0.0` equals `-0.0`).
pub fn extra_charge_compare(e: &mut Engine, this: Ptr<ExtraCharge>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraCharge>(e, this, other, EXTRA_CHARGE_TYPE) else {
        return true;
    };
    e.get(this, ExtraCharge::fCharge) != e.get(cast, ExtraCharge::fCharge)
}

// Translated from 00432000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraScript::ExtraScript` (Xbox PDB): extra-data type 0x0d, storing
/// `script` and no script variables. Returns `this`.
pub fn extra_script_extra_script(
    e: &mut Engine,
    this: Ptr<ExtraScript>,
    script: Ptr,
) -> Ptr<ExtraScript> {
    construct_base(e, this.cast(), TYPE_SCRIPT, EXTRA_SCRIPT_VTABLE);
    e.set(this, ExtraScript::pScript, script);
    e.set(this, ExtraScript::pScriptVars, Ptr::NULL);
    this
}

// Translated from 00432040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraScript`'s destructor body (the engine map has no name for it): resets
/// the vtable, deletes the owned script variables if there are any and clears
/// the pointer, then runs the base destructor. The exception-unwinding frame
/// is not translated.
pub fn fn_00432040(e: &mut Engine, this: Ptr<ExtraScript>) {
    e.mem.set_u32(this.addr(), EXTRA_SCRIPT_VTABLE);
    let variables = e.get(this, ExtraScript::pScriptVars);
    if !variables.is_null() {
        e.call(SCRIPT_LOCALS_DELETE, &args![variables, 1u32]);
    }
    e.set(this, ExtraScript::pScriptVars, Ptr::NULL);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004320d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraScript::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraScript`, when `BSExtraData::Compare` returns true, or when `pScript`
/// differs.
pub fn extra_script_compare(e: &mut Engine, this: Ptr<ExtraScript>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraScript>(e, this, other, EXTRA_SCRIPT_TYPE) else {
        return true;
    };
    e.get(this, ExtraScript::pScript) != e.get(cast, ExtraScript::pScript)
}

// Translated from 00432140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponModFlags::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraWeaponModFlags`, when `BSExtraData::Compare` returns true, or when
/// `cWeaponModsActive` differs.
pub fn extra_weapon_mod_flags_compare(
    e: &mut Engine,
    this: Ptr<ExtraWeaponModFlags>,
    other: Ptr,
) -> bool {
    let Some(cast) =
        compare_prologue::<_, ExtraWeaponModFlags>(e, this, other, EXTRA_WEAPON_MOD_FLAGS_TYPE)
    else {
        return true;
    };
    e.get(this, ExtraWeaponModFlags::cWeaponModsActive)
        != e.get(cast, ExtraWeaponModFlags::cWeaponModsActive)
}

// Translated from 004321b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraModdingItem::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraModdingItem`, when `BSExtraData::Compare` returns true, or when
/// `bIsModding` differs (compared as a byte).
pub fn extra_modding_item_compare(e: &mut Engine, this: Ptr<ExtraModdingItem>, other: Ptr) -> bool {
    let Some(cast) =
        compare_prologue::<_, ExtraModdingItem>(e, this, other, EXTRA_MODDING_ITEM_TYPE)
    else {
        return true;
    };
    e.get(this, ExtraModdingItem::bIsModding) != e.get(cast, ExtraModdingItem::bIsModding)
}

// Translated from 00432220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraScale`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `01015920` it stores): extra-data type 0x30, storing
/// `scale`. Returns `this`.
pub fn fn_00432220(e: &mut Engine, this: Ptr<ExtraScale>, scale: f32) -> Ptr<ExtraScale> {
    construct_base(e, this.cast(), TYPE_SCALE, EXTRA_SCALE_VTABLE);
    e.set(this, ExtraScale::fScale, scale);
    this
}

// Translated from 00432250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraScale::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraScale`, when `BSExtraData::Compare` returns true, or when `fScale`
/// differs (as `float`s: a NaN differs from everything, `0.0` equals `-0.0`).
pub fn extra_scale_compare(e: &mut Engine, this: Ptr<ExtraScale>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraScale>(e, this, other, EXTRA_SCALE_TYPE) else {
        return true;
    };
    e.get(this, ExtraScale::fScale) != e.get(cast, ExtraScale::fScale)
}

// Translated from 004322c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraGhost`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `01015bd0` it stores): extra-data type 0x1f, no data
/// of its own. Returns `this`.
pub fn fn_004322c0(e: &mut Engine, this: Ptr<ExtraGhost>) -> Ptr<ExtraGhost> {
    construct_base(e, this.cast(), TYPE_GHOST, EXTRA_GHOST_VTABLE);
    this
}

// Translated from 004322f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWorn`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `01015bdc` it stores): extra-data type 0x16, no data
/// of its own. Returns `this`.
pub fn fn_004322f0(e: &mut Engine, this: Ptr<ExtraWorn>) -> Ptr<ExtraWorn> {
    construct_base(e, this.cast(), TYPE_WORN, EXTRA_WORN_VTABLE);
    this
}

// Translated from 00432320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWornLeft`'s constructor (the engine map has no name for it; the class
/// is the one whose vtable `01015be8` it stores): extra-data type 0x17, no
/// data of its own. Returns `this`.
pub fn fn_00432320(e: &mut Engine, this: Ptr<ExtraWornLeft>) -> Ptr<ExtraWornLeft> {
    construct_base(e, this.cast(), TYPE_WORN_LEFT, EXTRA_WORN_LEFT_VTABLE);
    this
}

// Translated from 00432350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCannotWear`'s constructor (the engine map has no name for it; the
/// class is the one whose vtable `01015bf4` it stores): extra-data type 0x3e,
/// no data of its own. Returns `this`.
pub fn fn_00432350(e: &mut Engine, this: Ptr<ExtraCannotWear>) -> Ptr<ExtraCannotWear> {
    construct_base(e, this.cast(), TYPE_CANNOT_WEAR, EXTRA_CANNOT_WEAR_VTABLE);
    this
}

// Translated from 00432380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHotKey`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `0101592c` it stores): extra-data type 0x4a, storing
/// the byte `hotkey`. Returns `this`.
pub fn fn_00432380(e: &mut Engine, this: Ptr<ExtraHotKey>, hotkey: i8) -> Ptr<ExtraHotKey> {
    construct_base(e, this.cast(), TYPE_HOT_KEY, EXTRA_HOT_KEY_VTABLE);
    e.set(this, ExtraHotKey::chotkey, hotkey);
    this
}

// Translated from 004323b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraInfoGeneralTopic::~ExtraInfoGeneralTopic` (Xbox PDB): resets the
/// vtable, clears `MenuTopic::bGeneralTopic` (+0x24) of the owned topic, then
/// runs the topic's scalar deleting destructor (with its flag set to 1) and
/// the base destructor. The byte store comes before the null test, as
/// compiled: with no topic the game would fault, and the memory model
/// panics. The exception-unwinding frame is not translated.
pub fn extra_info_general_topic_destructor(e: &mut Engine, this: Ptr<ExtraInfoGeneralTopic>) {
    e.mem.set_u32(this.addr(), EXTRA_INFO_GENERAL_TOPIC_VTABLE);
    let topic = e.get(this, ExtraInfoGeneralTopic::pInfoGen);
    // MenuTopic::bGeneralTopic (Xbox PDB) +0x24
    e.mem.set_u8(topic.addr().wrapping_add(0x24), 0);
    if !topic.is_null() {
        e.call(MENU_TOPIC_DELETE, &args![topic, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00432440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraInfoGeneralTopic::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_info_general_topic_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraInfoGeneralTopic>,
    flags: u32,
) -> Ptr<ExtraInfoGeneralTopic> {
    extra_info_general_topic_destructor(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00432470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraInfoGeneralTopic`'s constructor taking the topic (the engine map has
/// no name for it; Ghidra's library match calls it `CPrintDialog`): extra-data
/// type 0x4d, storing `topic`. Returns `this`.
pub fn fn_00432470(
    e: &mut Engine,
    this: Ptr<ExtraInfoGeneralTopic>,
    topic: Ptr,
) -> Ptr<ExtraInfoGeneralTopic> {
    construct_base(
        e,
        this.cast(),
        TYPE_INFO_GENERAL_TOPIC,
        EXTRA_INFO_GENERAL_TOPIC_VTABLE,
    );
    e.set(this, ExtraInfoGeneralTopic::pInfoGen, topic);
    this
}

// Translated from 004324a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraInfoGeneralTopic`'s default constructor (the engine map has no name
/// for it): extra-data type 0x4d and a newly allocated, constructed 0x2c-byte
/// `MenuTopic` (null when the allocation fails). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_004324a0(e: &mut Engine, this: Ptr<ExtraInfoGeneralTopic>) -> Ptr<ExtraInfoGeneralTopic> {
    construct_base(
        e,
        this.cast(),
        TYPE_INFO_GENERAL_TOPIC,
        EXTRA_INFO_GENERAL_TOPIC_VTABLE,
    );
    let memory = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
    let topic: Ptr = if memory != 0 {
        e.call(MENU_TOPIC_CONSTRUCT, &args![memory]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, ExtraInfoGeneralTopic::pInfoGen, topic);
    this
}

// Translated from 00432540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHotKey::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraHotKey`, when `BSExtraData::Compare` returns true, or when
/// `chotkey` differs.
pub fn extra_hot_key_compare(e: &mut Engine, this: Ptr<ExtraHotKey>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraHotKey>(e, this, other, EXTRA_HOT_KEY_TYPE) else {
        return true;
    };
    e.get(this, ExtraHotKey::chotkey) != e.get(cast, ExtraHotKey::chotkey)
}

// Translated from 004325b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSeed`'s constructor (the engine map has no name for it; the class is
/// the one whose vtable `01015c0c` it stores): extra-data type 0x31, storing
/// the byte `seed`. Returns `this`.
pub fn fn_004325b0(e: &mut Engine, this: Ptr<ExtraSeed>, seed: u8) -> Ptr<ExtraSeed> {
    construct_base(e, this.cast(), TYPE_SEED, EXTRA_SEED_VTABLE);
    e.set(this, ExtraSeed::iSeed, seed);
    this
}

// Translated from 004325e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSeed::Compare` (Xbox PDB): true when `other` is not an `ExtraSeed`,
/// when `BSExtraData::Compare` returns true, or when `iSeed` differs.
pub fn extra_seed_compare(e: &mut Engine, this: Ptr<ExtraSeed>, other: Ptr) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraSeed>(e, this, other, EXTRA_SEED_TYPE) else {
        return true;
    };
    e.get(this, ExtraSeed::iSeed) != e.get(cast, ExtraSeed::iSeed)
}

// Translated from 00432650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPackageStartLocation`'s default constructor (the engine map has no
/// name for it): extra-data type 0x18 and a default-constructed
/// `WORLD_LOCATION`. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_00432650(
    e: &mut Engine,
    this: Ptr<ExtraPackageStartLocation>,
) -> Ptr<ExtraPackageStartLocation> {
    construct_base(
        e,
        this.cast(),
        TYPE_PACKAGE_START_LOCATION,
        EXTRA_PACKAGE_START_LOCATION_VTABLE,
    );
    let location = this.at(ExtraPackageStartLocation::worldLoc);
    e.call(WORLD_LOCATION_CONSTRUCT, &args![location]);
    this
}

// Translated from 004326c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPackageStartLocation`'s constructor from a place (the engine map has
/// no name for it): the default construction, then `pLocationForm` is `form`
/// (or `fallback_form` when `form` is null), `locPt` is a copy of the
/// `NiPoint3` at `position` and `fZRot` is `z_rot`. Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_004326c0(
    e: &mut Engine,
    this: Ptr<ExtraPackageStartLocation>,
    form: Ptr,
    fallback_form: Ptr,
    position: Ptr<NiPoint3>,
    z_rot: f32,
) -> Ptr<ExtraPackageStartLocation> {
    fn_00432650(e, this);
    let location = this.at(ExtraPackageStartLocation::worldLoc);
    let chosen = if form.is_null() { fallback_form } else { form };
    e.set(location, WorldLocation::pLocationForm, chosen);
    let point = location.at(WorldLocation::locPt);
    let x = e.get(position, NiPoint3::x);
    let y = e.get(position, NiPoint3::y);
    let z = e.get(position, NiPoint3::z);
    e.set(point, NiPoint3::x, x);
    e.set(point, NiPoint3::y, y);
    e.set(point, NiPoint3::z, z);
    e.set(location, WorldLocation::fZRot, z_rot);
    this
}

// Translated from 00432770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPackageStartLocation::Compare` (Xbox PDB): true when `other` is not
/// an `ExtraPackageStartLocation`, when `BSExtraData::Compare` returns true,
/// or when the 0x14 bytes of `worldLoc` differ (`memcmp`).
pub fn extra_package_start_location_compare(
    e: &mut Engine,
    this: Ptr<ExtraPackageStartLocation>,
    other: Ptr,
) -> bool {
    let Some(cast) = compare_prologue::<_, ExtraPackageStartLocation>(
        e,
        this,
        other,
        EXTRA_PACKAGE_START_LOCATION_TYPE,
    ) else {
        return true;
    };
    let mine = this.at(ExtraPackageStartLocation::worldLoc);
    let theirs = cast.at(ExtraPackageStartLocation::worldLoc);
    e.call(MEMCMP, &args![mine, theirs, 0x14u32]).u32() != 0
}

// Translated from 004327e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReferencePointer`'s constructor (the engine map has no name for it;
/// Ghidra's library match calls it `CPrintDialog`): extra-data type 0x1c,
/// storing `reference`. Returns `this`.
pub fn fn_004327e0(
    e: &mut Engine,
    this: Ptr<ExtraReferencePointer>,
    reference: Ptr,
) -> Ptr<ExtraReferencePointer> {
    construct_base(
        e,
        this.cast(),
        TYPE_REFERENCE_POINTER,
        EXTRA_REFERENCE_POINTER_VTABLE,
    );
    e.set(this, ExtraReferencePointer::pRef, reference);
    this
}

// Translated from 00432810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPackage`'s default constructor (the engine map has no name for it):
/// extra-data type 0x19, no package (`pPack` null), `iindex` -1, no target
/// and the three flag bytes clear. Returns `this`.
pub fn fn_00432810(e: &mut Engine, this: Ptr<ExtraPackage>) -> Ptr<ExtraPackage> {
    construct_base(e, this.cast(), TYPE_PACKAGE, EXTRA_PACKAGE_VTABLE);
    e.set(this, ExtraPackage::pPack, Ptr::NULL);
    e.set(this, ExtraPackage::iindex, -1);
    e.set(this, ExtraPackage::pTarg, Ptr::NULL);
    e.set(this, ExtraPackage::bActionComplete, 0);
    e.set(this, ExtraPackage::bActivated, 0);
    e.set(this, ExtraPackage::bDoneOnce, 0);
    this
}

// Translated from 00432870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPackage`'s constructor from its six fields (the engine map has no
/// name for it): extra-data type 0x19, storing the package, index, target and
/// the low bytes of the three flag arguments. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00432870(
    e: &mut Engine,
    this: Ptr<ExtraPackage>,
    package: Ptr,
    index: i32,
    target: Ptr,
    action_complete: u8,
    activated: u8,
    done_once: u8,
) -> Ptr<ExtraPackage> {
    construct_base(e, this.cast(), TYPE_PACKAGE, EXTRA_PACKAGE_VTABLE);
    e.set(this, ExtraPackage::pPack, package);
    e.set(this, ExtraPackage::iindex, index);
    e.set(this, ExtraPackage::pTarg, target);
    e.set(this, ExtraPackage::bActionComplete, action_complete);
    e.set(this, ExtraPackage::bActivated, activated);
    e.set(this, ExtraPackage::bDoneOnce, done_once);
    this
}

// Translated from 004328d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTresPassPackage`'s constructor (the engine map has no name for it;
/// Ghidra's library match calls it `CPrintDialog`): extra-data type 0x1a,
/// storing `package`. Returns `this`.
pub fn fn_004328d0(
    e: &mut Engine,
    this: Ptr<ExtraTresPassPackage>,
    package: Ptr,
) -> Ptr<ExtraTresPassPackage> {
    construct_base(
        e,
        this.cast(),
        TYPE_TRES_PASS_PACKAGE,
        EXTRA_TRES_PASS_PACKAGE_VTABLE,
    );
    e.set(this, ExtraTresPassPackage::pPack, package);
    this
}

// Translated from 00432900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTresPassPackage::~ExtraTresPassPackage` (Xbox PDB): resets the
/// vtable; when there is a package, marks it created
/// (`TESPackage::SetIsCreated(true)`), then deletes it: through
/// `TESSaveLoadGame::DeleteForm` when the save/load object's stub
/// (`0047c850`) answers true, else through the form's virtual scalar deleting
/// destructor (slot +0x10, flag 1). The stub always answers false, so the
/// virtual destructor is what runs in the game. Then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn extra_tres_pass_package_destructor(e: &mut Engine, this: Ptr<ExtraTresPassPackage>) {
    e.mem.set_u32(this.addr(), EXTRA_TRES_PASS_PACKAGE_VTABLE);
    let package = e.get(this, ExtraTresPassPackage::pPack);
    if !package.is_null() {
        e.call(PACKAGE_SET_IS_CREATED, &args![package, 1u32]);
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        if e.call(SAVE_LOAD_GAME_ALWAYS_FALSE, &args![save_load])
            .bool()
        {
            let package = e.get(this, ExtraTresPassPackage::pPack);
            e.call(SAVE_LOAD_GAME_DELETE_FORM, &args![save_load, package]);
        } else {
            let package = e.get(this, ExtraTresPassPackage::pPack);
            if !package.is_null() {
                e.vcall(package.addr(), 0x10, &args![1u32]);
            }
        }
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// The start the two `ExtraPlayerCrimeList` constructors share: the base
/// constructor, then a newly allocated, constructed 8-byte list (null when the
/// allocation fails) stored in `pCrime`. `ACTOR_LIST_CONSTRUCT` is the
/// constructor of an empty `BSSimpleList` of any element type.
pub(crate) fn construct_player_crime_list(e: &mut Engine, this: Ptr<ExtraPlayerCrimeList>) {
    construct_base(
        e,
        this.cast(),
        TYPE_PLAYER_CRIME_LIST,
        EXTRA_PLAYER_CRIME_LIST_VTABLE,
    );
    let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list: Ptr = if memory != 0 {
        e.call(ACTOR_LIST_CONSTRUCT, &args![memory]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, ExtraPlayerCrimeList::pCrime, list);
}

// Translated from 004329d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPlayerCrimeList`'s default constructor (the engine map has no name
/// for it): extra-data type 0x35 and a newly allocated, constructed 8-byte
/// crime list (null when the allocation fails). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_004329d0(e: &mut Engine, this: Ptr<ExtraPlayerCrimeList>) -> Ptr<ExtraPlayerCrimeList> {
    construct_player_crime_list(e, this);
    this
}

// Translated from 00432a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPlayerCrimeList::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_player_crime_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraPlayerCrimeList>,
    flags: u32,
) -> Ptr<ExtraPlayerCrimeList> {
    fn_00432b50(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00432aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPlayerCrimeList`'s constructor taking a crime (the engine map has no
/// name for it): the default construction, then the crime is added to the new
/// list (`005ae3d0` takes the address of a variable holding it, and does
/// nothing for a null crime). Returns `this`. The exception-unwinding frame is
/// not translated.
pub fn fn_00432aa0(
    e: &mut Engine,
    this: Ptr<ExtraPlayerCrimeList>,
    crime: Ptr,
) -> Ptr<ExtraPlayerCrimeList> {
    construct_player_crime_list(e, this);
    let list = e.get(this, ExtraPlayerCrimeList::pCrime);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), crime.addr());
        e.call(CRIME_LIST_ADD, &args![list, argument]);
    });
    this
}

// Translated from 00432b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPlayerCrimeList`'s destructor body (the engine map has no name for
/// it): resets the vtable, runs the owned list's scalar deleting destructor
/// (flag 1) when there is one, then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_00432b50(e: &mut Engine, this: Ptr<ExtraPlayerCrimeList>) {
    destroy_owner(
        e,
        this.cast(),
        EXTRA_PLAYER_CRIME_LIST_VTABLE,
        ACTOR_LIST_DELETE,
    );
}

// Translated from 00432be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLeveledItem`'s constructor (the engine map has no name for it; Ghidra
/// does not name it): extra-data type 0x2f, storing `index` and setting
/// `bdefault`. Returns `this`.
pub fn fn_00432be0(
    e: &mut Engine,
    this: Ptr<ExtraLeveledItem>,
    index: i32,
) -> Ptr<ExtraLeveledItem> {
    construct_base(e, this.cast(), TYPE_LEVELED_ITEM, EXTRA_LEVELED_ITEM_VTABLE);
    e.set(this, ExtraLeveledItem::iIndex, index);
    e.set(this, ExtraLeveledItem::bdefault, true);
    this
}

// Translated from 00432c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPersistentCell`'s constructor (the engine map has no name for it):
/// extra-data type 0x0c, storing `cell` (null stays null). Returns `this`.
pub fn fn_00432c20(
    e: &mut Engine,
    this: Ptr<ExtraPersistentCell>,
    cell: Ptr,
) -> Ptr<ExtraPersistentCell> {
    construct_base(
        e,
        this.cast(),
        TYPE_PERSISTENT_CELL,
        EXTRA_PERSISTENT_CELL_VTABLE,
    );
    e.set(this, ExtraPersistentCell::pPersistentCell, cell);
    this
}

// Translated from 00432c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPersistentCell`'s scalar deleting destructor (the engine map has no
/// name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_00432c60(
    e: &mut Engine,
    this: Ptr<ExtraPersistentCell>,
    flags: u32,
) -> Ptr<ExtraPersistentCell> {
    fn_00432c90(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00432c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPersistentCell`'s destructor body (the engine map has no name for
/// it): resets the vtable, then runs the base destructor; the cell is not
/// owned.
pub fn fn_00432c90(e: &mut Engine, this: Ptr<ExtraPersistentCell>) {
    e.mem.set_u32(this.addr(), EXTRA_PERSISTENT_CELL_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00432cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRagDollData::ExtraRagDollData` (Xbox PDB): extra-data type 0x14 and
/// no ragdoll data. Returns `this`.
pub fn extra_rag_doll_data_extra_rag_doll_data(
    e: &mut Engine,
    this: Ptr<ExtraRagDollData>,
) -> Ptr<ExtraRagDollData> {
    construct_base(
        e,
        this.cast(),
        TYPE_RAG_DOLL_DATA,
        EXTRA_RAG_DOLL_DATA_VTABLE,
    );
    e.set(this, ExtraRagDollData::pRagDollData, Ptr::NULL);
    this
}

// Translated from 00432ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRagDollData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_rag_doll_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRagDollData>,
    flags: u32,
) -> Ptr<ExtraRagDollData> {
    fn_00432d10(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00432d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRagDollData`'s destructor body (the engine map has no name for it):
/// resets the vtable, runs the owned `RagDollData`'s scalar deleting
/// destructor (flag 1) when there is one, then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_00432d10(e: &mut Engine, this: Ptr<ExtraRagDollData>) {
    e.mem.set_u32(this.addr(), EXTRA_RAG_DOLL_DATA_VTABLE);
    let data = e.get(this, ExtraRagDollData::pRagDollData);
    if !data.is_null() {
        fn_00432da0(e, data, 1);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00432da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RagDollData`'s scalar deleting destructor (the engine map has no name for
/// it): the destructor body (`004d9380`, in `ragdolldata.cpp`), then
/// `operator delete` when `flags & 1`. Returns `this`.
pub fn fn_00432da0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(RAG_DOLL_DATA_DESTRUCT, &args![this]);
    delete_when_asked(e, this, flags);
    this
}

/// The `__RTDynamicCast` the `Compare` and `Copy` methods start with: `other`
/// as the class with the RTTI descriptor `target_type`, null when it is not
/// one.
pub(crate) fn dynamic_cast_extra<U>(e: &mut Engine, other: Ptr, target_type: u32) -> Ptr<U> {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, BS_EXTRA_DATA_TYPE, target_type, 0u32],
    )
    .ptr()
}

/// Whether the list node holds no item and has no successor.
pub(crate) fn list_node_is_empty(e: &mut Engine, node: Ptr<BSSimpleList>) -> bool {
    e.call(SIMPLE_LIST_IS_EMPTY, &args![node]).bool()
}

/// The item of a list node (the word its item slot holds).
pub(crate) fn list_node_item(e: &mut Engine, node: Ptr<BSSimpleList>) -> u32 {
    let slot = e.call(SIMPLE_LIST_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// The node after `node` (null at the end).
pub(crate) fn list_node_next(e: &mut Engine, node: Ptr<BSSimpleList>) -> Ptr<BSSimpleList> {
    e.call(SIMPLE_LIST_NEXT, &args![node]).ptr()
}

/// The constructor of a class that holds a list of its own at +0xc and
/// nothing else: the base constructor, the vtable, then the list's
/// constructor.
pub(crate) fn construct_with_list(e: &mut Engine, this: Ptr, extra_type: u32, vtable: u32) {
    construct_base(e, this, extra_type, vtable);
    e.call(
        SIMPLE_LIST_CONSTRUCT,
        &args![Ptr::<BSSimpleList>::new(this.addr() + 0xc)],
    );
}

/// The destructor body of the classes that hold a list at +0xc and free its
/// nodes themselves: resets the vtable, runs the list's clear and its
/// destructor body, then the base destructor.
pub(crate) fn destroy_with_list(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    let list = Ptr::<BSSimpleList>::new(this.addr() + 0xc);
    e.call(SIMPLE_LIST_CLEAR, &args![list]);
    e.call(SIMPLE_LIST_DESTRUCT, &args![list]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00432dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRagDollData::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraRagDollData`, when `BSExtraData::Compare` returns true, when exactly
/// one of the two has ragdoll data, or when `RagDollData::Compare` says the
/// data differ.
pub fn extra_rag_doll_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraRagDollData>,
    other: Ptr,
) -> bool {
    let Some(cast) =
        compare_prologue::<_, ExtraRagDollData>(e, this, other, EXTRA_RAG_DOLL_DATA_TYPE)
    else {
        return true;
    };
    let mine = e.get(this, ExtraRagDollData::pRagDollData);
    let theirs = e.get(cast, ExtraRagDollData::pRagDollData);
    if mine.is_null() {
        return !theirs.is_null();
    }
    e.call(RAG_DOLL_DATA_COMPARE, &args![mine, theirs]).bool()
}

// Translated from 00432e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEncounterZone::ExtraEncounterZone` (Xbox PDB): extra-data type 0x74
/// and no zone. Returns `this`.
pub fn extra_encounter_zone_extra_encounter_zone(
    e: &mut Engine,
    this: Ptr<ExtraEncounterZone>,
) -> Ptr<ExtraEncounterZone> {
    construct_base(
        e,
        this.cast(),
        TYPE_ENCOUNTER_ZONE,
        EXTRA_ENCOUNTER_ZONE_VTABLE,
    );
    e.set(this, ExtraEncounterZone::pZone, Ptr::NULL);
    this
}

// Translated from 00432e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEncounterZone::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraEncounterZone`, when `BSExtraData::Compare` returns true, or when
/// the zones differ.
pub fn extra_encounter_zone_compare(
    e: &mut Engine,
    this: Ptr<ExtraEncounterZone>,
    other: Ptr,
) -> bool {
    let Some(cast) =
        compare_prologue::<_, ExtraEncounterZone>(e, this, other, EXTRA_ENCOUNTER_ZONE_TYPE)
    else {
        return true;
    };
    e.get(this, ExtraEncounterZone::pZone) != e.get(cast, ExtraEncounterZone::pZone)
}

// Translated from 00432f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraUsedMarkers`' constructor (the engine map has no name for it; the
/// class is the one whose vtable `01015c60` it stores): extra-data type 0x12
/// and no marker used. Returns `this`.
pub fn fn_00432f00(e: &mut Engine, this: Ptr<ExtraUsedMarkers>) -> Ptr<ExtraUsedMarkers> {
    construct_base(e, this.cast(), TYPE_USED_MARKERS, EXTRA_USED_MARKERS_VTABLE);
    e.set(this, ExtraUsedMarkers::iUsedMarkers, 0);
    this
}

// Translated from 00432f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor body (the engine map has no name for it) that only runs the
/// base destructor: it does not reset the vtable.
pub fn fn_00432f30(e: &mut Engine, this: Ptr) {
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00432f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReservedMarkers`' constructor (the engine map has no name for it;
/// the class is the one whose vtable `01015c6c` it stores): extra-data type
/// 0x82 and no marker reserved. Returns `this`.
pub fn fn_00432f50(e: &mut Engine, this: Ptr<ExtraReservedMarkers>) -> Ptr<ExtraReservedMarkers> {
    construct_base(
        e,
        this.cast(),
        TYPE_RESERVED_MARKERS,
        EXTRA_RESERVED_MARKERS_VTABLE,
    );
    e.set(this, ExtraReservedMarkers::iReservedMarkers, 0);
    this
}

// Translated from 00432f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraUsedMarkers::GetMarkerUsed` (Xbox PDB): whether bit `index` of the
/// used markers is set; false for an index of 30 or more.
pub fn extra_used_markers_get_marker_used(
    e: &mut Engine,
    this: Ptr<ExtraUsedMarkers>,
    index: u32,
) -> bool {
    index < 0x1e && e.get(this, ExtraUsedMarkers::iUsedMarkers) & (1 << (index & 0x1f)) != 0
}

// Translated from 00432fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraUsedMarkers::SetMarkerUsed` (Xbox PDB): sets bit `index` of the used
/// markers when `used` is not 0, clears it otherwise; nothing happens for an
/// index of 30 or more.
pub fn extra_used_markers_set_marker_used(
    e: &mut Engine,
    this: Ptr<ExtraUsedMarkers>,
    index: u32,
    used: u8,
) {
    if index >= 0x1e {
        return;
    }
    let bit = 1u32 << (index & 0x1f);
    let markers = e.get(this, ExtraUsedMarkers::iUsedMarkers);
    let markers = if used != 0 {
        markers | bit
    } else {
        markers & !bit
    };
    e.set(this, ExtraUsedMarkers::iUsedMarkers, markers);
}

// Translated from 00433010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRunOncePacks`' constructor (the engine map has no name for it):
/// extra-data type 0x1b and a newly allocated, constructed 8-byte list (null
/// when the allocation fails). Returns `this`. The exception-unwinding frame
/// is not translated.
pub fn fn_00433010(e: &mut Engine, this: Ptr<ExtraRunOncePacks>) -> Ptr<ExtraRunOncePacks> {
    construct_base(
        e,
        this.cast(),
        TYPE_RUN_ONCE_PACKS,
        EXTRA_RUN_ONCE_PACKS_VTABLE,
    );
    let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list: Ptr = if memory != 0 {
        e.call(SIMPLE_LIST_CONSTRUCT, &args![memory]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, ExtraRunOncePacks::pPackageList, list);
    this
}

// Translated from 004330b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRunOncePacks::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_run_once_packs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRunOncePacks>,
    flags: u32,
) -> Ptr<ExtraRunOncePacks> {
    fn_004330e0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004330e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRunOncePacks`' destructor body (the engine map has no name for it):
/// resets the vtable, deletes the item of every node up to the first node
/// without an item, clears the list (called even when the pointer is null),
/// deletes it if there is one, then runs the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_004330e0(e: &mut Engine, this: Ptr<ExtraRunOncePacks>) {
    e.mem.set_u32(this.addr(), EXTRA_RUN_ONCE_PACKS_VTABLE);
    let mut node = e
        .get(this, ExtraRunOncePacks::pPackageList)
        .cast::<BSSimpleList>();
    while !node.is_null() {
        if list_node_item(e, node) == 0 {
            break;
        }
        let item = list_node_item(e, node);
        e.call(OPERATOR_DELETE, &args![item]);
        node = list_node_next(e, node);
    }
    let list = e.get(this, ExtraRunOncePacks::pPackageList);
    e.call(SIMPLE_LIST_CLEAR, &args![list]);
    let list = e.get(this, ExtraRunOncePacks::pPackageList);
    if !list.is_null() {
        e.call(SIMPLE_LIST_DELETE, &args![list, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004331c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraRunOncePacks` (the engine map has no name for it): sets
/// the flag byte of the 8-byte entry whose first word is `key`. The entry is
/// searched in the package list (the search ends at the first node without
/// an item); a missing one is allocated and added to the list first. The
/// entry's first word is then `key` and its byte at +4 `flag`.
pub fn fn_004331c0(e: &mut Engine, this: Ptr<ExtraRunOncePacks>, key: u32, flag: u8) {
    let mut found = 0u32;
    let mut node = e
        .get(this, ExtraRunOncePacks::pPackageList)
        .cast::<BSSimpleList>();
    while !node.is_null() {
        let item = list_node_item(e, node);
        if item == 0 {
            node = Ptr::NULL;
        } else if e.mem.u32(item) == key {
            found = item;
            break;
        } else {
            node = list_node_next(e, node);
        }
    }
    if found == 0 {
        let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = e.get(this, ExtraRunOncePacks::pPackageList);
        // The list is given the address of the variable that holds the new
        // entry; the entry is read back from it afterwards.
        found = e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), entry);
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
            e.mem.u32(local.addr())
        });
    }
    e.mem.set_u32(found, key);
    e.mem.set_u8(found + 4, flag);
}

// Translated from 00433260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDistantData`'s constructor (the engine map has no name for it; the
/// class is the one whose vtable `01015c84` it stores): extra-data type 0x13
/// and the land normal `(0, 0, z)` with `z` the float constant at `010145a8`
/// (built by `NiPoint3`'s constructor in a temporary, then copied). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_00433260(e: &mut Engine, this: Ptr<ExtraDistantData>) -> Ptr<ExtraDistantData> {
    construct_base(e, this.cast(), TYPE_DISTANT_DATA, EXTRA_DISTANT_DATA_VTABLE);
    let normal = this.at(ExtraDistantData::LandNormal);
    // The call on the member's address does nothing (it returns its argument).
    e.call(SIMPLE_LIST_ITEM_SLOT, &args![normal]);
    let z: f32 = e.global(DISTANT_DATA_NORMAL_Z);
    let temporary = e.with_stack(0xc, |e, point| {
        let made = e
            .call(NI_POINT3_CONSTRUCT, &args![point, 0.0f32, 0.0f32, z])
            .u32();
        [e.mem.u32(made), e.mem.u32(made + 4), e.mem.u32(made + 8)]
    });
    for (i, word) in temporary.iter().enumerate() {
        e.mem.set_u32(normal.addr() + 4 * i as u32, *word);
    }
    this
}

// Translated from 00433300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEnableStateParent`'s constructor (the engine map has no name for
/// it): extra-data type 0x37, no parent and no flags. Returns `this`.
pub fn fn_00433300(
    e: &mut Engine,
    this: Ptr<ExtraEnableStateParent>,
) -> Ptr<ExtraEnableStateParent> {
    construct_base(
        e,
        this.cast(),
        TYPE_ENABLE_STATE_PARENT,
        EXTRA_ENABLE_STATE_PARENT_VTABLE,
    );
    e.set(this, ExtraEnableStateParent::pParent, Ptr::NULL);
    e.set(this, ExtraEnableStateParent::cFlags, 0);
    this
}

// Translated from 00433340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEnableStateParent::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraEnableStateParent`, or when the parent or the flags differ.
/// (`BSExtraData::Compare` is not asked.)
pub fn extra_enable_state_parent_compare(
    e: &mut Engine,
    this: Ptr<ExtraEnableStateParent>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraEnableStateParent> =
        dynamic_cast_extra(e, other, EXTRA_ENABLE_STATE_PARENT_TYPE);
    if cast.is_null() {
        return true;
    }
    e.get(cast, ExtraEnableStateParent::pParent) != e.get(this, ExtraEnableStateParent::pParent)
        || e.get(cast, ExtraEnableStateParent::cFlags)
            != e.get(this, ExtraEnableStateParent::cFlags)
}

// Translated from 004333a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEnableStateChildren`'s constructor (the engine map has no name for
/// it): extra-data type 0x38 and an empty child list. Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_004333a0(
    e: &mut Engine,
    this: Ptr<ExtraEnableStateChildren>,
) -> Ptr<ExtraEnableStateChildren> {
    construct_with_list(
        e,
        this.cast(),
        TYPE_ENABLE_STATE_CHILDREN,
        EXTRA_ENABLE_STATE_CHILDREN_VTABLE,
    );
    this
}

// Translated from 00433410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEnableStateChildren::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_enable_state_children_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraEnableStateChildren>,
    flags: u32,
) -> Ptr<ExtraEnableStateChildren> {
    fn_00433440(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00433440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEnableStateChildren`'s destructor body (the decompiler names it
/// `CMFCRibbonInfo::XQAT::~XQAT` by mistake; the body is this class's):
/// resets the vtable, clears the child list, runs the list's destructor body,
/// then the base destructor. The exception-unwinding frame is not translated.
pub fn fn_00433440(e: &mut Engine, this: Ptr<ExtraEnableStateChildren>) {
    destroy_with_list(e, this.cast(), EXTRA_ENABLE_STATE_CHILDREN_VTABLE);
}

// Translated from 004334b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRandomTeleportMarker`'s constructor (the engine map has no name for
/// it): extra-data type 0x3b and no marker. Returns `this`.
pub fn fn_004334b0(
    e: &mut Engine,
    this: Ptr<ExtraRandomTeleportMarker>,
) -> Ptr<ExtraRandomTeleportMarker> {
    construct_base(
        e,
        this.cast(),
        TYPE_RANDOM_TELEPORT_MARKER,
        EXTRA_RANDOM_TELEPORT_MARKER_VTABLE,
    );
    e.set(this, ExtraRandomTeleportMarker::pMarker, Ptr::NULL);
    this
}

// Translated from 004334e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRandomTeleportMarker::Compare` (Xbox PDB): true when `other` is not
/// an `ExtraRandomTeleportMarker` or when the markers differ.
/// (`BSExtraData::Compare` is not asked.)
pub fn extra_random_teleport_marker_compare(
    e: &mut Engine,
    this: Ptr<ExtraRandomTeleportMarker>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraRandomTeleportMarker> =
        dynamic_cast_extra(e, other, EXTRA_RANDOM_TELEPORT_MARKER_TYPE);
    if cast.is_null() {
        return true;
    }
    e.get(cast, ExtraRandomTeleportMarker::pMarker)
        != e.get(this, ExtraRandomTeleportMarker::pMarker)
}

// Translated from 00433530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLinkedRefChildren`'s constructor (the engine map has no name for
/// it): extra-data type 0x52 and an empty child list. Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_00433530(
    e: &mut Engine,
    this: Ptr<ExtraLinkedRefChildren>,
) -> Ptr<ExtraLinkedRefChildren> {
    construct_with_list(
        e,
        this.cast(),
        TYPE_LINKED_REF_CHILDREN,
        EXTRA_LINKED_REF_CHILDREN_VTABLE,
    );
    this
}

// Translated from 004335a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLinkedRefChildren::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_linked_ref_children_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraLinkedRefChildren>,
    flags: u32,
) -> Ptr<ExtraLinkedRefChildren> {
    fn_004335d0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004335d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLinkedRefChildren`'s destructor body (the decompiler names it
/// `CMFCRibbonInfo::XQAT::~XQAT` by mistake; the body is this class's):
/// resets the vtable, clears the child list, runs the list's destructor body,
/// then the base destructor. The exception-unwinding frame is not translated.
pub fn fn_004335d0(e: &mut Engine, this: Ptr<ExtraLinkedRefChildren>) {
    destroy_with_list(e, this.cast(), EXTRA_LINKED_REF_CHILDREN_VTABLE);
}

// Translated from 00433640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLinkedRef`'s constructor (the engine map has no name for it):
/// extra-data type 0x51 and no linked reference. Returns `this`.
pub fn fn_00433640(e: &mut Engine, this: Ptr<ExtraLinkedRef>) -> Ptr<ExtraLinkedRef> {
    construct_base(e, this.cast(), TYPE_LINKED_REF, EXTRA_LINKED_REF_VTABLE);
    e.set(this, ExtraLinkedRef::pLinkedRef, Ptr::NULL);
    this
}

// Translated from 00433670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLinkedRef::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraLinkedRef` or when the linked references differ.
/// (`BSExtraData::Compare` is not asked.)
pub fn extra_linked_ref_compare(e: &mut Engine, this: Ptr<ExtraLinkedRef>, other: Ptr) -> bool {
    let cast: Ptr<ExtraLinkedRef> = dynamic_cast_extra(e, other, EXTRA_LINKED_REF_TYPE);
    if cast.is_null() {
        return true;
    }
    e.get(cast, ExtraLinkedRef::pLinkedRef) != e.get(this, ExtraLinkedRef::pLinkedRef)
}

// Translated from 004336c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAshPileRef`'s constructor (the engine map has no name for it):
/// extra-data type 0x89 and no reference. Returns `this`.
pub fn fn_004336c0(e: &mut Engine, this: Ptr<ExtraAshPileRef>) -> Ptr<ExtraAshPileRef> {
    construct_base(e, this.cast(), TYPE_ASH_PILE_REF, EXTRA_ASH_PILE_REF_VTABLE);
    e.set(this, ExtraAshPileRef::pAshPileRef, Ptr::NULL);
    this
}

// Translated from 004336f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cdecl search in a list of `REF_ACTIVATE_DATA` pointers (the engine map
/// has no name for it): the first item whose `pActivateRef` is `reference`,
/// null if there is none. Stops at the first empty node.
pub fn fn_004336f0(
    e: &mut Engine,
    reference: u32,
    list: Ptr<BSSimpleList>,
) -> Ptr<RefActivateData> {
    let mut node = list;
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        if e.mem.u32(item) == reference {
            return Ptr::new(item);
        }
        node = list_node_next(e, node);
    }
    Ptr::NULL
}

// Translated from 00433750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cdecl function (the engine map has no name for it): deletes the item of
/// the list's head and removes the head, until the list is empty.
pub fn fn_00433750(e: &mut Engine, list: Ptr<BSSimpleList>) {
    while !list.is_null() && !list_node_is_empty(e, list) {
        let item = list_node_item(e, list);
        e.call(OPERATOR_DELETE, &args![item]);
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 00433790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRefChildren`'s constructor (the engine map has no name for
/// it): extra-data type 0x54, an empty child list and a timer of 0. Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_00433790(
    e: &mut Engine,
    this: Ptr<ExtraActivateRefChildren>,
) -> Ptr<ExtraActivateRefChildren> {
    construct_with_list(
        e,
        this.cast(),
        TYPE_ACTIVATE_REF_CHILDREN,
        EXTRA_ACTIVATE_REF_CHILDREN_VTABLE,
    );
    e.set(this, ExtraActivateRefChildren::fActivateChildrenTimer, 0.0);
    this
}

// Translated from 00433800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRefChildren::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_activate_ref_children_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraActivateRefChildren>,
    flags: u32,
) -> Ptr<ExtraActivateRefChildren> {
    fn_00433830(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00433830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRefChildren`'s destructor body (the engine map has no name
/// for it): resets the vtable, deletes the child list's items and nodes
/// (`00433750`), runs the list's destructor body, then the base destructor.
/// The exception-unwinding frame is not translated.
pub fn fn_00433830(e: &mut Engine, this: Ptr<ExtraActivateRefChildren>) {
    e.mem
        .set_u32(this.addr(), EXTRA_ACTIVATE_REF_CHILDREN_VTABLE);
    let list = this.at(ExtraActivateRefChildren::ChildList);
    fn_00433750(e, list);
    e.call(SIMPLE_LIST_DESTRUCT, &args![list]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004338b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRef`'s constructor (the engine map has no name for it):
/// extra-data type 0x53, an empty parent list, no flags, and an
/// `ActivateTextOverride` string that is built and then `Set` to the empty string
/// at `01011584` (an empty C string; length 0 means all of it). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_004338b0(e: &mut Engine, this: Ptr<ExtraActivateRef>) -> Ptr<ExtraActivateRef> {
    construct_with_list(e, this.cast(), TYPE_ACTIVATE_REF, EXTRA_ACTIVATE_REF_VTABLE);
    let text = this.at(ExtraActivateRef::ActivateTextOverride);
    e.call(BS_STRING_CONSTRUCT, &args![text]);
    e.set(this, ExtraActivateRef::cActivateFlags, 0);
    e.call(BS_STRING_SET, &args![text, EMPTY_STRING, 0u32]);
    this
}

// Translated from 00433940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRef::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_activate_ref_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraActivateRef>,
    flags: u32,
) -> Ptr<ExtraActivateRef> {
    fn_00433970(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00433970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRef`'s destructor body (the engine map has no name for it):
/// resets the vtable, deletes the parent list's items and nodes (`00433750`),
/// clears the `ActivateTextOverride` string, runs the list's destructor body,
/// then the base destructor. The exception-unwinding frame is not translated.
pub fn fn_00433970(e: &mut Engine, this: Ptr<ExtraActivateRef>) {
    e.mem.set_u32(this.addr(), EXTRA_ACTIVATE_REF_VTABLE);
    let list = this.at(ExtraActivateRef::ParentList);
    fn_00433750(e, list);
    e.call(
        BS_STRING_CLEAR,
        &args![this.at(ExtraActivateRef::ActivateTextOverride)],
    );
    e.call(SIMPLE_LIST_DESTRUCT, &args![list]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00433a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraActivateRef` (the engine map has no name for it): the
/// entry of the parent list for `reference` (`004336f0`), null if there is
/// none.
pub fn fn_00433a00(
    e: &mut Engine,
    this: Ptr<ExtraActivateRef>,
    reference: u32,
) -> Ptr<RefActivateData> {
    fn_004336f0(e, reference, this.at(ExtraActivateRef::ParentList))
}

// Translated from 00433a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRef::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraActivateRef`, when the parent lists have a different number of
/// entries, when an entry of this list has no entry for its reference in the
/// other list or differs from it in its 8 bytes, when the flags differ, or
/// when the activate texts differ (an empty text equals only an empty text,
/// otherwise the strings are compared with `strcmp`). `BSExtraData::Compare`
/// is not asked.
pub fn extra_activate_ref_compare(e: &mut Engine, this: Ptr<ExtraActivateRef>, other: Ptr) -> bool {
    let cast: Ptr<ExtraActivateRef> = dynamic_cast_extra(e, other, EXTRA_ACTIVATE_REF_TYPE);
    if cast.is_null() {
        return true;
    }
    let mine = this.at(ExtraActivateRef::ParentList);
    let theirs = cast.at(ExtraActivateRef::ParentList);
    let my_count = e.call(SIMPLE_LIST_COUNT, &args![mine]).u32();
    let their_count = e.call(SIMPLE_LIST_COUNT, &args![theirs]).u32();
    if my_count != their_count {
        return true;
    }
    let mut node = mine;
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        let reference = e.mem.u32(item);
        let counterpart = fn_00433a00(e, cast, reference);
        if counterpart.is_null() || e.call(MEMCMP, &args![counterpart, item, 8u32]).u32() != 0 {
            return true;
        }
        node = list_node_next(e, node);
    }
    if e.get(this, ExtraActivateRef::cActivateFlags)
        != e.get(cast, ExtraActivateRef::cActivateFlags)
    {
        return true;
    }
    let my_text = this.at(ExtraActivateRef::ActivateTextOverride);
    let their_text = cast.at(ExtraActivateRef::ActivateTextOverride);
    if e.call(BS_STRING_LENGTH, &args![my_text]).u32() == 0
        && e.call(BS_STRING_LENGTH, &args![their_text]).u32() == 0
    {
        return false;
    }
    if e.call(BS_STRING_LENGTH, &args![my_text]).u32() == 0
        || e.call(BS_STRING_LENGTH, &args![their_text]).u32() == 0
    {
        return true;
    }
    let their_chars = e.call(BS_STRING_DATA, &args![their_text]).u32();
    let my_chars = e.call(BS_STRING_DATA, &args![my_text]).u32();
    e.call(STRCMP, &args![my_chars, their_chars]).u32() != 0
}

// Translated from 00433b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateRef::Copy` (Xbox PDB): when `other` is an `ExtraActivateRef`,
/// deletes this parent list's entries, copies each of the other list's
/// 8-byte entries into a new one added to the list (at the head, so the order
/// is reversed), and copies the flags and the activate text (`BSStringT::Set`).
/// Nothing happens when `other` is another class. The exception-unwinding
/// frame is not translated.
pub fn extra_activate_ref_copy(e: &mut Engine, this: Ptr<ExtraActivateRef>, other: Ptr) {
    let cast: Ptr<ExtraActivateRef> = dynamic_cast_extra(e, other, EXTRA_ACTIVATE_REF_TYPE);
    if cast.is_null() {
        return;
    }
    let list = this.at(ExtraActivateRef::ParentList);
    fn_00433750(e, list);
    let mut node = cast.at(ExtraActivateRef::ParentList);
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        node = list_node_next(e, node);
        let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let copy = if memory != 0 {
            e.call(REF_ACTIVATE_DATA_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(MEMCPY, &args![copy, item, 8u32]);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), copy);
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
        });
    }
    let flags = e.get(cast, ExtraActivateRef::cActivateFlags);
    e.set(this, ExtraActivateRef::cActivateFlags, flags);
    e.call(
        BS_STRING_COPY,
        &args![
            this.at(ExtraActivateRef::ActivateTextOverride),
            cast.at(ExtraActivateRef::ActivateTextOverride)
        ],
    );
}

// Translated from 00433ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs`' constructor (the engine map has no name for it):
/// extra-data type 0x57 and an empty decal list. Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_00433ca0(e: &mut Engine, this: Ptr<ExtraDecalRefs>) -> Ptr<ExtraDecalRefs> {
    construct_with_list(e, this.cast(), TYPE_DECAL_REFS, EXTRA_DECAL_REFS_VTABLE);
    this
}

// Translated from 00433d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_decal_refs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraDecalRefs>,
    flags: u32,
) -> Ptr<ExtraDecalRefs> {
    fn_00433d40(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00433d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs`' destructor body (the decompiler names it
/// `CMFCFilterChunkValueImpl::~CMFCFilterChunkValueImpl` by mistake; the body
/// is this class's): resets the vtable, frees the decal list's entries
/// (`004343c0`), runs the list's destructor body, then the base destructor.
/// The exception-unwinding frame is not translated.
pub fn fn_00433d40(e: &mut Engine, this: Ptr<ExtraDecalRefs>) {
    e.mem.set_u32(this.addr(), EXTRA_DECAL_REFS_VTABLE);
    fn_004343c0(e, this);
    e.call(
        SIMPLE_LIST_DESTRUCT,
        &args![this.at(ExtraDecalRefs::DecalRefList)],
    );
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// The frame helper of the three `InitItem`s: asks the owner's file for its
/// compile index, adds it to the form id held at `id_slot` (a word on the
/// game's stack, modified in place), looks the form up and casts it to a
/// `TESObjectREFR`. Returns the reference, or null when there is none.
pub(crate) fn resolve_reference(e: &mut Engine, owner: Ptr, id_slot: Ptr) -> Ptr {
    let file = e.call(FORM_GET_FILE, &args![owner, 0xffff_ffffu32]).u32();
    e.call(FORM_ADD_COMPILE_INDEX, &args![id_slot, file]);
    let id = e.mem.u32(id_slot.addr());
    let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TES_FORM_TYPE, TES_OBJECT_REFR_TYPE, 0u32],
    )
    .ptr()
}

/// Drops the entry of a list that an `InitItem` could not resolve: the head
/// is removed (remove-head, `0063f7b0`) when there is no previous node, otherwise the
/// entry (the pointer held at `item_slot`) is removed after `previous`, and
/// the walk continues at the node after `previous`. The entry is then
/// deleted. Returns the node the walk goes on with.
pub(crate) fn drop_unresolved_entry(
    e: &mut Engine,
    node: Ptr<BSSimpleList>,
    previous: Ptr<BSSimpleList>,
    item_slot: Ptr,
) -> Ptr<BSSimpleList> {
    let next = if previous.is_null() {
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![node]);
        node
    } else {
        e.call(SIMPLE_LIST_REMOVE_ITEM, &args![previous, item_slot]);
        list_node_next(e, previous)
    };
    let item = e.mem.u32(item_slot.addr());
    e.call(OPERATOR_DELETE, &args![item]);
    e.mem.set_u32(item_slot.addr(), 0);
    next
}

// Translated from 00433db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs::InitItem` (engine map name, not in the Xbox PDB; `this`,
/// then the reference the extra data belongs to, whose file is asked for the
/// compile index): resolves the form id of every decal entry to the
/// reference. An entry whose form cannot be found (not a `TESObjectREFR`) is
/// logged ("MASTERFILE: Unable to find decal reference %08X."), removed from
/// the list and deleted; the others have their first word replaced by the
/// reference pointer. The exception-unwinding frame is not translated.
pub fn extra_decal_refs_init_item(e: &mut Engine, this: Ptr<ExtraDecalRefs>, owner: Ptr) {
    e.with_stack(8, |e, frame| {
        // `frame` holds the entry pointer (its address goes to `00905330`),
        // `frame + 4` the form id (its address goes to `AddCompileIndex`).
        let item_slot = frame;
        let id_slot = frame.byte_add(4);
        let mut node = this.at(ExtraDecalRefs::DecalRefList);
        let mut previous: Ptr<BSSimpleList> = Ptr::NULL;
        while !node.is_null() && !list_node_is_empty(e, node) {
            let mut removed = false;
            let item = list_node_item(e, node);
            e.mem.set_u32(item_slot.addr(), item);
            let id = e.mem.u32(item);
            e.mem.set_u32(id_slot.addr(), id);
            let reference = resolve_reference(e, owner, id_slot);
            if reference.is_null() {
                let id = e.mem.u32(id_slot.addr());
                e.call(MASTERFILE_LOG, &args![MESSAGE_DECAL_NOT_FOUND, id]);
                node = drop_unresolved_entry(e, node, previous, item_slot);
                removed = true;
            } else {
                e.mem.set_u32(item, reference.addr());
            }
            if !removed {
                previous = node;
                node = list_node_next(e, node);
            }
        }
    });
}

// Translated from 00433ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectorRefs::InitItem` (engine map name, not in the Xbox PDB;
/// `this`, then the reference the extra data belongs to): resolves the form
/// id of every entry to a persistent reference. An entry whose form is not a
/// reference, or whose reference is not persistent, is logged ("Unable to find
/// water reflector reference" / "is not persistent so cannot reflect or
/// refract other references"), removed and deleted. For the others, the
/// reference's extra-data list is told about the owner once for each of the
/// entry's effect flags (bit 0: `0041f4c0`, bit 1: `0041f650`), and the first
/// word of the entry becomes the reference pointer. The exception-unwinding
/// frame is not translated.
pub fn extra_reflector_refs_init_item(e: &mut Engine, this: Ptr<ExtraReflectorRefs>, owner: Ptr) {
    e.with_stack(8, |e, frame| {
        let item_slot = frame;
        let id_slot = frame.byte_add(4);
        let mut node = this.at(ExtraReflectorRefs::RefList);
        let mut previous: Ptr<BSSimpleList> = Ptr::NULL;
        while !node.is_null() && !list_node_is_empty(e, node) {
            let mut removed = false;
            let item = list_node_item(e, node);
            e.mem.set_u32(item_slot.addr(), item);
            let id = e.mem.u32(item);
            e.mem.set_u32(id_slot.addr(), id);
            let reference = resolve_reference(e, owner, id_slot);
            if reference.is_null() || !e.call(REFR_GET_REF_PERSISTS, &args![reference]).bool() {
                let id = e.mem.u32(id_slot.addr());
                if reference.is_null() {
                    e.call(MASTERFILE_LOG, &args![MESSAGE_REFLECTOR_NOT_FOUND, id]);
                } else {
                    e.call(MASTERFILE_LOG, &args![MESSAGE_NOT_PERSISTENT, id]);
                }
                node = drop_unresolved_entry(e, node, previous, item_slot);
                removed = true;
            } else {
                // REF_REFLECTOR_DATA::iEffectFlags (Xbox PDB) +0x04
                if e.mem.u32(item + 4) & 1 != 0 {
                    let list = e.call(REFR_EXTRA_LIST, &args![reference]).u32();
                    e.call(REFLECTOR_ADD_FIRST, &args![list, owner, 1u32]);
                }
                if e.mem.u32(item + 4) & 2 != 0 {
                    let list = e.call(REFR_EXTRA_LIST, &args![reference]).u32();
                    e.call(REFLECTOR_ADD_SECOND, &args![list, owner, 1u32]);
                }
                e.mem.set_u32(item, reference.addr());
            }
            if !removed {
                previous = node;
                node = list_node_next(e, node);
            }
        }
    });
}

// Translated from 00434050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLitWaterRefs::InitItem` (engine map name, not in the Xbox PDB;
/// `this`, then the reference the extra data belongs to): resolves the form
/// id of every entry to a persistent reference and rebuilds the list from
/// the ones that resolve (the temporary list reverses their order, the
/// refill reverses it again, so the order is kept). An entry whose form is not a
/// reference, or whose reference is not persistent, is logged ("Unable to find
/// lit water reference" / "is not persistent so cannot reflect or refract
/// other references") and dropped. For each good reference, its extra-data
/// list is told about the owner (`ExtraDataList::SetWaterLightRef`, adding)
/// and the reference is added at the head of a temporary list; the list is
/// then cleared and refilled from the temporary one, which is cleared and
/// destroyed. The exception-unwinding frame is not translated.
pub fn extra_lit_water_refs_init_item(e: &mut Engine, this: Ptr<ExtraLitWaterRefs>, owner: Ptr) {
    // `frame`: the temporary list (8 bytes), then the form id and the
    // reference, whose addresses are passed on.
    e.with_stack(0x10, |e, frame| {
        let temporary: Ptr<BSSimpleList> = frame.cast();
        let id_slot = frame.byte_add(8);
        let reference_slot = frame.byte_add(0xc);
        e.call(SIMPLE_LIST_CONSTRUCT, &args![temporary]);
        let list = this.at(ExtraLitWaterRefs::RefList);
        let mut node = list;
        while !node.is_null() && !list_node_is_empty(e, node) {
            let id = list_node_item(e, node);
            e.mem.set_u32(id_slot.addr(), id);
            let reference = resolve_reference(e, owner, id_slot);
            e.mem.set_u32(reference_slot.addr(), reference.addr());
            if reference.is_null() || !e.call(REFR_GET_REF_PERSISTS, &args![reference]).bool() {
                let id = e.mem.u32(id_slot.addr());
                if reference.is_null() {
                    e.call(MASTERFILE_LOG, &args![MESSAGE_LIT_WATER_NOT_FOUND, id]);
                } else {
                    e.call(MASTERFILE_LOG, &args![MESSAGE_NOT_PERSISTENT, id]);
                }
            } else {
                let extra_list = e.call(REFR_EXTRA_LIST, &args![reference]).u32();
                e.call(WATER_LIGHT_REF_SET, &args![extra_list, owner, 1u32]);
                e.call(SIMPLE_LIST_ADD_HEAD, &args![temporary, reference_slot]);
            }
            node = list_node_next(e, node);
        }
        e.call(SIMPLE_LIST_CLEAR, &args![list]);
        let mut walk = temporary;
        while !walk.is_null() {
            let slot = e.call(SIMPLE_LIST_ITEM_SLOT, &args![walk]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(SIMPLE_LIST_ITEM_SLOT, &args![walk]).u32();
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, slot]);
            walk = list_node_next(e, walk);
        }
        e.call(SIMPLE_LIST_CLEAR, &args![temporary]);
        e.call(SIMPLE_LIST_DESTRUCT, &args![temporary]);
    });
}

/// The entries of two decal, reflected or reflector lists, compared the way
/// the three `Compare` methods do after their cast: the lists must have the
/// same number of entries, and every entry of `other_list` must have an entry
/// of the same first word in `this` (`GetRefDecalData`) whose first
/// `entry_size` bytes are equal. Returns true when they differ.
pub(crate) fn entry_lists_differ(
    e: &mut Engine,
    this: Ptr<ExtraDecalRefs>,
    other_list: Ptr<BSSimpleList>,
    entry_size: u32,
) -> bool {
    let my_count = e
        .call(
            SIMPLE_LIST_COUNT,
            &args![this.at(ExtraDecalRefs::DecalRefList)],
        )
        .u32();
    let their_count = e.call(SIMPLE_LIST_COUNT, &args![other_list]).u32();
    if my_count != their_count {
        return true;
    }
    let mut node = other_list;
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        let reference = e.mem.u32(item);
        let counterpart = extra_decal_refs_get_ref_decal_data(e, this, reference);
        if counterpart.is_null() {
            return true;
        }
        if e.call(MEMCMP, &args![counterpart, item, entry_size]).u32() != 0 {
            return true;
        }
        node = list_node_next(e, node);
    }
    false
}

// Translated from 004341e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs::Compare` (engine map name, `vt` slot): true when `other`
/// is not an `ExtraDecalRefs`, when the lists have a different number of
/// entries, or when an entry of the other list has no entry with the same
/// first word here or differs from it in its 0x1c bytes. `BSExtraData::Compare`
/// is not asked.
pub fn extra_decal_refs_compare(e: &mut Engine, this: Ptr<ExtraDecalRefs>, other: Ptr) -> bool {
    let cast: Ptr<ExtraDecalRefs> = dynamic_cast_extra(e, other, EXTRA_DECAL_REFS_TYPE);
    if cast.is_null() {
        return true;
    }
    entry_lists_differ(e, this, cast.at(ExtraDecalRefs::DecalRefList), 0x1c)
}

// Translated from 004342b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs::Copy` (engine map name, `vt` slot): when `other` is an
/// `ExtraDecalRefs`, deletes this list's entries (`004343c0`) and adds a copy
/// (0x1c bytes, built with the `REF_DECAL_DATA` constructor) of each of the
/// other list's entries at the head, so the order is reversed. Nothing
/// happens when `other` is another class. The exception-unwinding frame is not
/// translated.
pub fn extra_decal_refs_copy(e: &mut Engine, this: Ptr<ExtraDecalRefs>, other: Ptr) {
    let cast: Ptr<ExtraDecalRefs> = dynamic_cast_extra(e, other, EXTRA_DECAL_REFS_TYPE);
    if cast.is_null() {
        return;
    }
    fn_004343c0(e, this);
    let mut node = cast.at(ExtraDecalRefs::DecalRefList);
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        let memory = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        let copy = if memory != 0 {
            e.call(REF_DECAL_DATA_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(MEMCPY, &args![copy, item, 0x1cu32]);
        let list = this.at(ExtraDecalRefs::DecalRefList);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), copy);
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
        });
        node = list_node_next(e, node);
    }
}

// Translated from 004343c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraDecalRefs` (the engine map has no name for it): deletes
/// every decal entry and empties the list (removes the head, then deletes
/// the entry it held, until the list is empty).
pub fn fn_004343c0(e: &mut Engine, this: Ptr<ExtraDecalRefs>) {
    let list = this.at(ExtraDecalRefs::DecalRefList);
    while !list_node_is_empty(e, list) {
        let item = list_node_item(e, list);
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![list]);
        e.call(OPERATOR_DELETE, &args![item]);
    }
}

// Translated from 00434410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDecalRefs::GetRefDecalData` (engine map name; the body is shared,
/// by identical-code folding, with the reflected and reflector classes, whose
/// entries also start with the reference): the first entry of the list whose
/// first word is `reference`, null if there is none.
pub fn extra_decal_refs_get_ref_decal_data(
    e: &mut Engine,
    this: Ptr<ExtraDecalRefs>,
    reference: u32,
) -> Ptr<RefDecalData> {
    let mut node = this.at(ExtraDecalRefs::DecalRefList);
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        node = list_node_next(e, node);
        if e.mem.u32(item) == reference {
            return Ptr::new(item);
        }
    }
    Ptr::NULL
}

// Translated from 00434480 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraDecalRefs` (the engine map has no name for it): sets the
/// decal data of `reference`. When the list has no entry whose first word is
/// `reference`, a 0x1c-byte entry is built (`REF_DECAL_DATA` constructor),
/// given `reference` as its first word and added at the head of the list. The
/// entry's `Intersect` is then set from the vector at `intersect` and its
/// `Normal` from the vector at `normal`. The exception-unwinding frame is not
/// translated.
pub fn fn_00434480(
    e: &mut Engine,
    this: Ptr<ExtraDecalRefs>,
    reference: u32,
    intersect: Ptr,
    normal: Ptr,
) {
    let mut entry = extra_decal_refs_get_ref_decal_data(e, this, reference).addr();
    if entry == 0 {
        let memory = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        entry = if memory != 0 {
            e.call(REF_DECAL_DATA_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.mem.set_u32(entry, reference);
        let list = this.at(ExtraDecalRefs::DecalRefList);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), entry);
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
        });
    }
    // REF_DECAL_DATA::Intersect (Xbox PDB) +0x04, ::Normal +0x10
    copy_words(e, intersect.addr(), entry + 4);
    copy_words(e, normal.addr(), entry + 0x10);
}

/// The destructor body of the classes that own the entries of the list at +0xc
/// (`ExtraReflectedRefs`, `ExtraReflectorRefs`): resets the vtable, deletes
/// every entry, clears the list, runs the list's destructor body, then the
/// base destructor. The exception-unwinding frame is not translated.
pub(crate) fn destroy_with_owned_entries(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    let list = Ptr::<BSSimpleList>::new(this.addr() + 0xc);
    let mut node = list;
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        e.call(OPERATOR_DELETE, &args![item]);
        node = list_node_next(e, node);
    }
    e.call(SIMPLE_LIST_CLEAR, &args![list]);
    e.call(SIMPLE_LIST_DESTRUCT, &args![list]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00434560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectedRefs`' destructor body (the decompiler names it
/// `CMFCRibbonInfo::XQAT::~XQAT` by mistake; the vtable `01014428` is this
/// class's): deletes every entry, clears the list, then the base destructor.
/// The exception-unwinding frame is not translated.
pub fn fn_00434560(e: &mut Engine, this: Ptr<ExtraReflectedRefs>) {
    destroy_with_owned_entries(e, this.cast(), EXTRA_REFLECTED_REFS_VTABLE);
}

// Translated from 00434620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectedRefs::Compare` (engine map name, `vt` slot): true when `other`
/// is not an `ExtraReflectedRefs`, when the lists have a different number of
/// entries, or when an entry of the other list has no entry with the same
/// first word here (`ExtraDecalRefs::GetRefDecalData`, whose body is shared)
/// or differs from it in its 8 bytes. `BSExtraData::Compare` is not asked.
pub fn extra_reflected_refs_compare(
    e: &mut Engine,
    this: Ptr<ExtraReflectedRefs>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraReflectedRefs> = dynamic_cast_extra(e, other, EXTRA_REFLECTED_REFS_TYPE);
    if cast.is_null() {
        return true;
    }
    entry_lists_differ(e, this.cast(), cast.at(ExtraReflectedRefs::RefList), 8)
}

// Translated from 004346f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectedRefs::Copy` (engine map name, `vt` slot): when `other` is an
/// `ExtraReflectedRefs`, removes every entry from this list without deleting
/// them (`ClearReferenceData`) and adds a copy (8 bytes) of each of the other
/// list's entries at the head, so the order is reversed. Nothing happens when
/// `other` is another class. The exception-unwinding frame is not translated.
pub fn extra_reflected_refs_copy(e: &mut Engine, this: Ptr<ExtraReflectedRefs>, other: Ptr) {
    let cast: Ptr<ExtraReflectedRefs> = dynamic_cast_extra(e, other, EXTRA_REFLECTED_REFS_TYPE);
    if cast.is_null() {
        return;
    }
    extra_reflected_refs_clear_reference_data(e, this);
    copy_reference_entries(
        e,
        this.at(ExtraReflectedRefs::RefList),
        cast.at(ExtraReflectedRefs::RefList),
    );
}

/// The loop of the reflected and reflector `Copy` methods: adds at the head of
/// `list` a copy (8 bytes, built like `REF_REFLECTED_DATA`: the list-node
/// constructor `0096a2d0` clears it) of each entry of `source`.
pub(crate) fn copy_reference_entries(
    e: &mut Engine,
    list: Ptr<BSSimpleList>,
    source: Ptr<BSSimpleList>,
) {
    let mut node = source;
    while !node.is_null() && !list_node_is_empty(e, node) {
        let item = list_node_item(e, node);
        let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let copy = if memory != 0 {
            e.call(SIMPLE_LIST_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(MEMCPY, &args![copy, item, 8u32]);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), copy);
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
        });
        node = list_node_next(e, node);
    }
}

// Translated from 00434800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraReflectedRefs` (the engine map has no name for it; its
/// callers in `extradatalist.cpp` use it for the reflected and the reflector
/// classes alike, so the body is shared): removes the first entry whose
/// first word is `reference` from the list (the head with remove-head `0063f7b0`, any
/// other node after its predecessor) and deletes the entry. Nothing happens
/// when there is none.
pub fn fn_00434800(e: &mut Engine, this: Ptr<ExtraReflectedRefs>, reference: u32) {
    e.with_stack(4, |e, item_slot| {
        let mut node = this.at(ExtraReflectedRefs::RefList);
        let mut previous: Ptr<BSSimpleList> = Ptr::NULL;
        while !node.is_null() && !list_node_is_empty(e, node) {
            let item = list_node_item(e, node);
            e.mem.set_u32(item_slot.addr(), item);
            if e.mem.u32(item) == reference {
                if previous.is_null() {
                    e.call(SIMPLE_LIST_REMOVE_HEAD, &args![node]);
                } else {
                    e.call(SIMPLE_LIST_REMOVE_ITEM, &args![previous, item_slot]);
                }
                let removed = e.mem.u32(item_slot.addr());
                e.call(OPERATOR_DELETE, &args![removed]);
                e.mem.set_u32(item_slot.addr(), 0);
                return;
            }
            previous = node;
            node = list_node_next(e, node);
        }
    });
}

// Translated from 004348a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraReflectedRefs` (the engine map has no name for it; the
/// body is shared with the reflector class): sets the effect flags of the
/// entry for `reference`. When the list has no entry whose first word is
/// `reference` (`ExtraDecalRefs::GetRefDecalData`), an 8-byte entry is built
/// (`0096a2d0` clears it), given `reference` as its first word and added at
/// the head of the list. The entry's `iEffectFlags` is then set to `flags`.
/// The exception-unwinding frame is not translated.
pub fn fn_004348a0(e: &mut Engine, this: Ptr<ExtraReflectedRefs>, reference: u32, flags: u32) {
    let mut entry = extra_decal_refs_get_ref_decal_data(e, this.cast(), reference).addr();
    if entry == 0 {
        let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
        entry = if memory != 0 {
            e.call(SIMPLE_LIST_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.mem.set_u32(entry, reference);
        let list = this.at(ExtraReflectedRefs::RefList);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), entry);
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
        });
    }
    // REF_REFLECTED_DATA::iEffectFlags (Xbox PDB) +0x04
    e.mem.set_u32(entry + 4, flags);
}

// Translated from 00434950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterLightRefs`' destructor body (the decompiler names it
/// `CMFCRibbonInfo::XQAT::~XQAT` by mistake; the vtable `01014440` is this
/// class's): resets the vtable, clears the list (the references are not
/// owned), runs the list's destructor body, then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_00434950(e: &mut Engine, this: Ptr<ExtraWaterLightRefs>) {
    destroy_with_list(e, this.cast(), EXTRA_WATER_LIGHT_REFS_VTABLE);
}

/// The `Compare` of the two reference-list classes `ExtraWaterLightRefs` and
/// `ExtraLitWaterRefs` after their cast: true when the lists have a different
/// number of entries or when an entry of the other list is not in this list.
pub(crate) fn reference_lists_differ(
    e: &mut Engine,
    this_list: Ptr<BSSimpleList>,
    other_list: Ptr<BSSimpleList>,
) -> bool {
    let my_count = e.call(SIMPLE_LIST_COUNT, &args![this_list]).u32();
    let their_count = e.call(SIMPLE_LIST_COUNT, &args![other_list]).u32();
    if my_count != their_count {
        return true;
    }
    let mut node = other_list;
    while !node.is_null() && !list_node_is_empty(e, node) {
        let slot = e.call(SIMPLE_LIST_ITEM_SLOT, &args![node]).u32();
        if !e.call(SIMPLE_LIST_CONTAINS, &args![this_list, slot]).bool() {
            return true;
        }
        node = list_node_next(e, node);
    }
    false
}

// Translated from 004349c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterLightRefs::Compare` (engine map name, `vt` slot): true when
/// `other` is not an `ExtraWaterLightRefs`, when the lists have a different
/// number of entries, or when an entry of the other list is not in this list.
/// `BSExtraData::Compare` is not asked.
pub fn extra_water_light_refs_compare(
    e: &mut Engine,
    this: Ptr<ExtraWaterLightRefs>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraWaterLightRefs> = dynamic_cast_extra(e, other, EXTRA_WATER_LIGHT_REFS_TYPE);
    if cast.is_null() {
        return true;
    }
    reference_lists_differ(
        e,
        this.at(ExtraWaterLightRefs::RefList),
        cast.at(ExtraWaterLightRefs::RefList),
    )
}

// Translated from 00434a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterLightRefs::Copy` (engine map name, `vt` slot): when `other` is an
/// `ExtraWaterLightRefs`, clears this list (`00434fa0`) and adds each reference
/// of the other list that is not already there (`00434f60`). Nothing happens
/// when `other` is another class.
pub fn extra_water_light_refs_copy(e: &mut Engine, this: Ptr<ExtraWaterLightRefs>, other: Ptr) {
    let cast: Ptr<ExtraWaterLightRefs> = dynamic_cast_extra(e, other, EXTRA_WATER_LIGHT_REFS_TYPE);
    if cast.is_null() {
        return;
    }
    fn_00434fa0(e, this.cast());
    let mut node = cast.at(ExtraWaterLightRefs::RefList);
    while !node.is_null() && !list_node_is_empty(e, node) {
        let reference = list_node_item(e, node);
        fn_00434f60(e, this.cast(), reference);
        node = list_node_next(e, node);
    }
}

// Translated from 00434af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectorRefs`' destructor body (the decompiler names it
/// `CMFCRibbonInfo::XQAT::~XQAT` by mistake; the vtable `01014434` is this
/// class's): deletes every entry, clears the list, then the base destructor.
/// The exception-unwinding frame is not translated.
pub fn fn_00434af0(e: &mut Engine, this: Ptr<ExtraReflectorRefs>) {
    destroy_with_owned_entries(e, this.cast(), EXTRA_REFLECTOR_REFS_VTABLE);
}

// Translated from 00434bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectorRefs::Compare` (engine map name, `vt` slot): true when `other`
/// is not an `ExtraReflectorRefs`, when the lists have a different number of
/// entries, or when an entry of the other list has no entry with the same
/// first word here (`ExtraDecalRefs::GetRefDecalData`, whose body is shared)
/// or differs from it in its 8 bytes. `BSExtraData::Compare` is not asked.
pub fn extra_reflector_refs_compare(
    e: &mut Engine,
    this: Ptr<ExtraReflectorRefs>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraReflectorRefs> = dynamic_cast_extra(e, other, EXTRA_REFLECTOR_REFS_TYPE);
    if cast.is_null() {
        return true;
    }
    entry_lists_differ(e, this.cast(), cast.at(ExtraReflectorRefs::RefList), 8)
}

// Translated from 00434c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectorRefs::Copy` (engine map name, `vt` slot): when `other` is an
/// `ExtraReflectorRefs`, removes every entry from this list without deleting
/// them (`ExtraReflectedRefs::ClearReferenceData`, whose body is shared) and
/// adds a copy (8 bytes) of each of the other list's entries at the head, so
/// the order is reversed. Nothing happens when `other` is another class. The
/// exception-unwinding frame is not translated.
pub fn extra_reflector_refs_copy(e: &mut Engine, this: Ptr<ExtraReflectorRefs>, other: Ptr) {
    let cast: Ptr<ExtraReflectorRefs> = dynamic_cast_extra(e, other, EXTRA_REFLECTOR_REFS_TYPE);
    if cast.is_null() {
        return;
    }
    extra_reflected_refs_clear_reference_data(e, this.cast());
    copy_reference_entries(
        e,
        this.at(ExtraReflectorRefs::RefList),
        cast.at(ExtraReflectorRefs::RefList),
    );
}

// Translated from 00434d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectedRefs::ClearReferenceData` (engine map name; the body is
/// shared with `ExtraReflectorRefs`): removes the head of the list until it is
/// empty. The entries are not deleted.
pub fn extra_reflected_refs_clear_reference_data(e: &mut Engine, this: Ptr<ExtraReflectedRefs>) {
    let list = this.at(ExtraReflectedRefs::RefList);
    while !list_node_is_empty(e, list) {
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 00434dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLitWaterRefs`' destructor body (the decompiler names it
/// `CMFCRibbonInfo::XQAT::~XQAT` by mistake; the vtable `0101444c` is this
/// class's): resets the vtable, clears the list (the references are not
/// owned), runs the list's destructor body, then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_00434dc0(e: &mut Engine, this: Ptr<ExtraLitWaterRefs>) {
    destroy_with_list(e, this.cast(), EXTRA_LIT_WATER_REFS_VTABLE);
}

// Translated from 00434e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLitWaterRefs::Compare` (engine map name, `vt` slot): true when `other`
/// is not an `ExtraLitWaterRefs`, when the lists have a different number of
/// entries, or when an entry of the other list is not in this list.
/// `BSExtraData::Compare` is not asked.
pub fn extra_lit_water_refs_compare(
    e: &mut Engine,
    this: Ptr<ExtraLitWaterRefs>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraLitWaterRefs> = dynamic_cast_extra(e, other, EXTRA_LIT_WATER_REFS_TYPE);
    if cast.is_null() {
        return true;
    }
    reference_lists_differ(
        e,
        this.at(ExtraLitWaterRefs::RefList),
        cast.at(ExtraLitWaterRefs::RefList),
    )
}

// Translated from 00434ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLitWaterRefs::Copy` (engine map name, `vt` slot): when `other` is an
/// `ExtraLitWaterRefs`, clears this list (`00434fa0`) and adds each reference
/// of the other list that is not already there (`00434f60`). Nothing happens
/// when `other` is another class.
pub fn extra_lit_water_refs_copy(e: &mut Engine, this: Ptr<ExtraLitWaterRefs>, other: Ptr) {
    let cast: Ptr<ExtraLitWaterRefs> = dynamic_cast_extra(e, other, EXTRA_LIT_WATER_REFS_TYPE);
    if cast.is_null() {
        return;
    }
    fn_00434fa0(e, this.cast());
    let mut node = cast.at(ExtraLitWaterRefs::RefList);
    while !node.is_null() && !list_node_is_empty(e, node) {
        let reference = list_node_item(e, node);
        fn_00434f60(e, this.cast(), reference);
        node = list_node_next(e, node);
    }
}

// Translated from 00434f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraWaterLightRefs` (the engine map has no name for it; the
/// body is shared with `ExtraLitWaterRefs`): adds `reference` at the head of
/// the list unless the list already holds it.
pub fn fn_00434f60(e: &mut Engine, this: Ptr<ExtraWaterLightRefs>, reference: u32) {
    let list = this.at(ExtraWaterLightRefs::RefList);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), reference);
        if !e.call(SIMPLE_LIST_CONTAINS, &args![list, local]).bool() {
            e.call(SIMPLE_LIST_ADD_HEAD, &args![list, local]);
        }
    });
}

// Translated from 00434fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraWaterLightRefs` (the engine map has no name for it; the
/// body is shared with `ExtraLitWaterRefs`): clears the list (`BSSimpleList`
/// clear: frees the nodes after the head and empties the head).
pub fn fn_00434fa0(e: &mut Engine, this: Ptr<ExtraWaterLightRefs>) {
    e.call(
        SIMPLE_LIST_CLEAR,
        &args![this.at(ExtraWaterLightRefs::RefList)],
    );
}

// Translated from 00434fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMerchantContainer`'s constructor (the engine map has no name for it):
/// extra-data type 0x3c and no container. Returns `this`.
pub fn fn_00434fc0(
    e: &mut Engine,
    this: Ptr<ExtraMerchantContainer>,
) -> Ptr<ExtraMerchantContainer> {
    construct_base(
        e,
        this.cast(),
        TYPE_MERCHANT_CONTAINER,
        EXTRA_MERCHANT_CONTAINER_VTABLE,
    );
    e.set(this, ExtraMerchantContainer::pContainer, Ptr::NULL);
    this
}

// Translated from 00434ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMerchantContainer::Compare` (engine map name, `vt` slot): true when
/// `other` is not an `ExtraMerchantContainer` or when the containers differ.
/// (`BSExtraData::Compare` is not asked.)
pub fn extra_merchant_container_compare(
    e: &mut Engine,
    this: Ptr<ExtraMerchantContainer>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraMerchantContainer> =
        dynamic_cast_extra(e, other, EXTRA_MERCHANT_CONTAINER_TYPE);
    if cast.is_null() {
        return true;
    }
    e.get(cast, ExtraMerchantContainer::pContainer)
        != e.get(this, ExtraMerchantContainer::pContainer)
}

// Translated from 00435040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLevCreaModifier`'s constructor (the engine map has no name for it):
/// extra-data type 0x1e and the modifier 4 (`LCM_NONE`, Xbox PDB). Returns
/// `this`.
pub fn fn_00435040(e: &mut Engine, this: Ptr<ExtraLevCreaModifier>) -> Ptr<ExtraLevCreaModifier> {
    construct_base(
        e,
        this.cast(),
        TYPE_LEV_CREA_MODIFIER,
        EXTRA_LEV_CREA_MODIFIER_VTABLE,
    );
    e.set(
        this,
        ExtraLevCreaModifier::eModifier,
        LEV_CREA_MODIFIER_NONE,
    );
    this
}

// Translated from 00435070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLevCreaModifier::Compare` (engine map name, `vt` slot): true when
/// `other` is not an `ExtraLevCreaModifier` or when the modifiers differ.
/// (`BSExtraData::Compare` is not asked.)
pub fn extra_lev_crea_modifier_compare(
    e: &mut Engine,
    this: Ptr<ExtraLevCreaModifier>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraLevCreaModifier> =
        dynamic_cast_extra(e, other, EXTRA_LEV_CREA_MODIFIER_TYPE);
    if cast.is_null() {
        return true;
    }
    e.get(cast, ExtraLevCreaModifier::eModifier) != e.get(this, ExtraLevCreaModifier::eModifier)
}

// Translated from 004350c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraLevCreaModifier` (the engine map has no name for it): the
/// modifier's multiplier. 1.0 for `LCM_NONE` (4); otherwise the float game
/// setting the modifier selects in the table at `01184ab0` (one setting
/// pointer per modifier), read through the setting accessor. Returned in ST0.
pub fn fn_004350c0(e: &mut Engine, this: Ptr<ExtraLevCreaModifier>) -> f32 {
    let modifier = e.get(this, ExtraLevCreaModifier::eModifier);
    if modifier == LEV_CREA_MODIFIER_NONE {
        return 1.0;
    }
    let setting = e.mem.u32(LEV_CREA_SETTING_TABLE + modifier.wrapping_mul(4));
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

// Translated from 00435100 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraLevCreaModifier` (the engine map has no name for it): 1
/// for the modifier 0 (`LCM_EASY`), 0 for 4 (`LCM_NONE`) and -1 for any other.
pub fn fn_00435100(e: &mut Engine, this: Ptr<ExtraLevCreaModifier>) -> i32 {
    match e.get(this, ExtraLevCreaModifier::eModifier) {
        0 => 1,
        LEV_CREA_MODIFIER_NONE => 0,
        _ => -1,
    }
}

// Translated from 00435150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPoison`'s constructor (the decompiler names it
/// `CPrintDialog::CPrintDialog` by mistake; the engine map has no name for it):
/// extra-data type 0x3f and the poison given. Returns `this`.
pub fn fn_00435150(e: &mut Engine, this: Ptr<ExtraPoison>, poison: Ptr) -> Ptr<ExtraPoison> {
    construct_base(e, this.cast(), TYPE_POISON, EXTRA_POISON_VTABLE);
    e.set(this, ExtraPoison::pPoison, poison);
    this
}

// Translated from 00435180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPoison::Compare` (engine map name, `vt` slot): true when `other` is
/// not an `ExtraPoison` or when the poisons differ. (`BSExtraData::Compare` is
/// not asked.)
pub fn extra_poison_compare(e: &mut Engine, this: Ptr<ExtraPoison>, other: Ptr) -> bool {
    let cast: Ptr<ExtraPoison> = dynamic_cast_extra(e, other, EXTRA_POISON_TYPE);
    if cast.is_null() {
        return true;
    }
    e.get(cast, ExtraPoison::pPoison) != e.get(this, ExtraPoison::pPoison)
}

// Translated from 004351d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLastFinishedSequence`'s constructor (the engine map has no name for
/// it): extra-data type 0x41 and a copy of the C string given, in a new
/// buffer of `strlen + 1` bytes (`operator new`, then the checked string copy
/// `00406d30(destination, size, source)`). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_004351d0(
    e: &mut Engine,
    this: Ptr<ExtraLastFinishedSequence>,
    name: Ptr,
) -> Ptr<ExtraLastFinishedSequence> {
    construct_base(
        e,
        this.cast(),
        TYPE_LAST_FINISHED_SEQUENCE,
        EXTRA_LAST_FINISHED_SEQUENCE_VTABLE,
    );
    let size = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
    let buffer = e.call(OPERATOR_NEW, &args![size]).ptr();
    e.set(this, ExtraLastFinishedSequence::pLastSequenceName, buffer);
    e.call(STRING_COPY_CHECKED, &args![buffer, size, name]);
    this
}

// Translated from 00435270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLastFinishedSequence::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_last_finished_sequence_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraLastFinishedSequence>,
    flags: u32,
) -> Ptr<ExtraLastFinishedSequence> {
    fn_004352a0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004352a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLastFinishedSequence`'s destructor body (the engine map has no name
/// for it): resets the vtable, deletes the name buffer (without testing it for
/// null), then runs the base destructor. The exception-unwinding frame is not
/// translated.
pub fn fn_004352a0(e: &mut Engine, this: Ptr<ExtraLastFinishedSequence>) {
    e.mem
        .set_u32(this.addr(), EXTRA_LAST_FINISHED_SEQUENCE_VTABLE);
    let name = e.get(this, ExtraLastFinishedSequence::pLastSequenceName);
    e.call(OPERATOR_DELETE, &args![name]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00435310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLastFinishedSequence::Compare` (Xbox PDB): true when `other` is not
/// an `ExtraLastFinishedSequence` or when `strcmp` of the two names is not 0.
/// (`BSExtraData::Compare` is not asked.)
pub fn extra_last_finished_sequence_compare(
    e: &mut Engine,
    this: Ptr<ExtraLastFinishedSequence>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraLastFinishedSequence> =
        dynamic_cast_extra(e, other, EXTRA_LAST_FINISHED_SEQUENCE_TYPE);
    if cast.is_null() {
        return true;
    }
    let mine = e.get(this, ExtraLastFinishedSequence::pLastSequenceName);
    let theirs = e.get(cast, ExtraLastFinishedSequence::pLastSequenceName);
    e.call(STRCMP, &args![mine, theirs]).u32() != 0
}

// Translated from 00435370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraXTarget`'s constructor (the engine map has no name for it):
/// extra-data type 0x44 and no target. Returns `this`.
pub fn fn_00435370(e: &mut Engine, this: Ptr<ExtraXTarget>) -> Ptr<ExtraXTarget> {
    construct_base(e, this.cast(), TYPE_X_TARGET, EXTRA_X_TARGET_VTABLE);
    e.set(this, ExtraXTarget::pTarget, Ptr::NULL);
    this
}

// Translated from 004353a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraXTarget::_scalar_deleting_destructor_` (the engine map has no name
/// for it): the destructor, then `operator delete` when `flags & 1`. Returns
/// `this`.
pub fn fn_004353a0(e: &mut Engine, this: Ptr<ExtraXTarget>, flags: u32) -> Ptr<ExtraXTarget> {
    fn_004353d0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004353d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraXTarget`'s destructor body (the engine map has no name for it):
/// resets the vtable, then runs the base destructor. The target is not owned.
pub fn fn_004353d0(e: &mut Engine, this: Ptr<ExtraXTarget>) {
    e.mem.set_u32(this.addr(), EXTRA_X_TARGET_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// `00435c10` (`thiscall`, no stack word; engine map `ExtraSavedAnimation::DeleteBuffer`):
/// releases the buffer of the extra data.
const SAVED_DATA_DELETE_BUFFER: u32 = 0x0043_5c10;

// Translated from 00435ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00435ac0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeb968, state variable) is not translated
/// The `ExtraSavedAnimation` destructor (no Ghidra function; `thiscall`, no stack word; called
/// by its scalar deleting destructor `00435a90`): sets the vtable `01015ea0`, releases the
/// buffer (`00435c10`) and ends with the `BSExtraData` destructor (`0040ecb0`).
pub fn fn_00435ac0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_5ea0);
    e.call(SAVED_DATA_DELETE_BUFFER, &args![this]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00435bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00435bb0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeb968, state variable) is not translated
/// The `ExtraSavedHavokData` destructor (no Ghidra function; `thiscall`, no stack word;
/// called by its scalar deleting destructor `00435b80`): sets the vtable `01015eac`, releases
/// the buffer (`00435c10`) and ends with the `BSExtraData` destructor (`0040ecb0`).
pub fn fn_00435bb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_5eac);
    e.call(SAVED_DATA_DELETE_BUFFER, &args![this]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004300f0,
            extra_anim_scalar_deleting_destructor(Ptr<ExtraAnim>, u32) -> Ptr<ExtraAnim>
        ),
        entry!(0x00430120, fn_00430120(Ptr<ExtraAnim>)),
        entry!(0x004301b0, fn_004301b0(Ptr, i32) -> u16),
        entry!(
            0x00430200,
            extra_dismembered_limbs_extra_dismembered_limbs(
                Ptr<ExtraDismemberedLimbs>,
            )
                -> Ptr<ExtraDismemberedLimbs>
        ),
        entry!(
            0x004302a0,
            extra_dismembered_limbs_scalar_deleting_destructor(
                Ptr<ExtraDismemberedLimbs>,
                u32,
            )
                -> Ptr<ExtraDismemberedLimbs>
        ),
        entry!(0x004302d0, fn_004302d0(Ptr<ExtraDismemberedLimbs>)),
        entry!(
            0x00430390,
            fn_00430390(Ptr<DismemberedLimb>, u32) -> Ptr<DismemberedLimb>
        ),
        entry!(0x004303c0, fn_004303c0(Ptr<DismemberedLimb>)),
        entry!(
            0x004303e0,
            extra_dismembered_limbs_dismembered(Ptr<ExtraDismemberedLimbs>, u8) -> bool
        ),
        entry!(
            0x00430410,
            extra_dismembered_limbs_dismember(Ptr<ExtraDismemberedLimbs>, Ptr, u8, u8)
        ),
        entry!(0x004305f0, fn_004305f0(Ptr<ExtraDismemberedLimbs>, u32, u8)),
        entry!(
            0x00430660,
            extra_dismembered_limbs_check_dismembered_limbs_identical(
                Ptr<ExtraDismemberedLimbs>,
                Ptr<ExtraDismemberedLimbs>,
            ) -> bool
        ),
        entry!(
            0x00430780,
            fn_00430780(Ptr<ExtraStartingPosition>, Ptr) -> Ptr<ExtraStartingPosition>
        ),
        entry!(0x00430830, fn_00430830(Ptr) -> Ptr),
        entry!(
            0x00430850,
            extra_starting_position_compare(Ptr<ExtraStartingPosition>, Ptr) -> bool
        ),
        entry!(
            0x004308c0,
            fn_004308c0(Ptr<ExtraType49>) -> Ptr<ExtraType49>
        ),
        entry!(
            0x004308f0,
            fn_004308f0(Ptr<ExtraLight>, Ptr) -> Ptr<ExtraLight>
        ),
        entry!(
            0x00430920,
            extra_light_scalar_deleting_destructor(Ptr<ExtraLight>, u32) -> Ptr<ExtraLight>
        ),
        entry!(0x00430950, fn_00430950(Ptr<ExtraLight>)),
        entry!(
            0x004309e0,
            refr_lock_get_lock_level(Ptr<RefrLock>, Ptr) -> i32
        ),
        entry!(0x00430a10, refr_lock_get_level(Ptr<RefrLock>, Ptr) -> i32),
        entry!(0x00430a90, refr_lock_set_locked(Ptr<RefrLock>, bool)),
        entry!(0x00430ae0, refr_lock_is_broken(Ptr<RefrLock>) -> bool),
        entry!(0x00430b40, refr_lock_numeric_value_to_enum(i32) -> i32),
        entry!(0x00430bc0, fn_00430bc0(u32) -> i32),
        entry!(
            0x00430c70,
            extra_lock_extra_lock(Ptr<ExtraLock>, Ptr<RefrLock>) -> Ptr<ExtraLock>
        ),
        entry!(0x00430ca0, fn_00430ca0(Ptr<ExtraLock>)),
        entry!(0x00430ce0, extra_lock_compare(Ptr<ExtraLock>, Ptr) -> bool),
        entry!(
            0x00430dd0,
            extra_follower_extra_follower(Ptr<ExtraFollower>) -> Ptr<ExtraFollower>
        ),
        entry!(
            0x00430e70,
            extra_follower_scalar_deleting_destructor(
                Ptr<ExtraFollower>,
                u32,
            ) -> Ptr<ExtraFollower>
        ),
        entry!(0x00430ea0, fn_00430ea0(Ptr<ExtraFollower>)),
        entry!(
            0x00430f30,
            extra_guarded_ref_data_extra_guarded_ref_data(
                Ptr<ExtraGuardedRefData>,
            ) -> Ptr<ExtraGuardedRefData>
        ),
        entry!(
            0x00430fa0,
            extra_guarded_ref_data_scalar_deleting_destructor(
                Ptr<ExtraGuardedRefData>,
                u32,
            )
                -> Ptr<ExtraGuardedRefData>
        ),
        entry!(0x00430fd0, fn_00430fd0(Ptr<ExtraGuardedRefData>)),
        entry!(
            0x00431030,
            extra_guarded_ref_data_compare(
                Ptr<ExtraGuardedRefData>,
                Ptr<ExtraGuardedRefData>,
            ) -> bool
        ),
        entry!(
            0x004310d0,
            extra_guarded_ref_data_add_guard(Ptr<ExtraGuardedRefData>, Ptr)
        ),
        entry!(0x00431120, fn_00431120(Ptr<ExtraGuardedRefData>, u32, u32)),
        entry!(
            0x004311f0,
            fn_004311f0(Ptr<ExtraType2e>) -> Ptr<ExtraType2e>
        ),
        entry!(
            0x00431230,
            fn_00431230(Ptr<ExtraTeleport>, Ptr) -> Ptr<ExtraTeleport>
        ),
        entry!(0x00431260, fn_00431260(Ptr<ExtraTeleport>)),
        entry!(
            0x004312f0,
            extra_teleport_compare(Ptr<ExtraTeleport>, Ptr) -> bool
        ),
        entry!(
            0x00431360,
            fn_00431360(Ptr<ExtraMapMarker>, Ptr) -> Ptr<ExtraMapMarker>
        ),
        entry!(
            0x00431390,
            extra_map_marker_scalar_deleting_destructor(
                Ptr<ExtraMapMarker>,
                u32,
            ) -> Ptr<ExtraMapMarker>
        ),
        entry!(0x004313c0, fn_004313c0(Ptr<ExtraMapMarker>)),
        entry!(
            0x00431450,
            extra_map_marker_compare(Ptr<ExtraMapMarker>, Ptr) -> bool
        ),
        entry!(
            0x004314c0,
            fn_004314c0(Ptr<ExtraAudioMarker>, Ptr) -> Ptr<ExtraAudioMarker>
        ),
        entry!(
            0x004314f0,
            extra_audio_marker_scalar_deleting_destructor(
                Ptr<ExtraAudioMarker>,
                u32,
            ) -> Ptr<ExtraAudioMarker>
        ),
        entry!(0x00431520, fn_00431520(Ptr<ExtraAudioMarker>)),
        entry!(
            0x004315b0,
            extra_audio_marker_compare(Ptr<ExtraAudioMarker>, Ptr) -> bool
        ),
        entry!(
            0x00431620,
            fn_00431620(Ptr<ExtraAudioBuoyMarker>, Ptr) -> Ptr<ExtraAudioBuoyMarker>
        ),
        entry!(
            0x00431650,
            extra_audio_buoy_marker_scalar_deleting_destructor(
                Ptr<ExtraAudioBuoyMarker>,
                u32,
            )
                -> Ptr<ExtraAudioBuoyMarker>
        ),
        entry!(0x00431680, fn_00431680(Ptr<ExtraAudioBuoyMarker>)),
        entry!(
            0x00431710,
            extra_audio_buoy_marker_compare(Ptr<ExtraAudioBuoyMarker>, Ptr) -> bool
        ),
        entry!(
            0x00431780,
            fn_00431780(Ptr<ExtraAction>) -> Ptr<ExtraAction>
        ),
        entry!(
            0x004317c0,
            extra_action_compare(Ptr<ExtraAction>, Ptr) -> bool
        ),
        entry!(
            0x00431830,
            fn_00431830(Ptr<ExtraContainerChanges>, Ptr) -> Ptr<ExtraContainerChanges>
        ),
        entry!(
            0x00431860,
            extra_container_changes_scalar_deleting_destructor(
                Ptr<ExtraContainerChanges>,
                u32,
            )
                -> Ptr<ExtraContainerChanges>
        ),
        entry!(0x00431890, fn_00431890(Ptr<ExtraContainerChanges>)),
        entry!(0x00431920, fn_00431920(Ptr, u32) -> Ptr),
        entry!(
            0x00431950,
            fn_00431950(Ptr<ExtraOriginalReference>, Ptr) -> Ptr<ExtraOriginalReference>
        ),
        entry!(
            0x00431980,
            fn_00431980(Ptr<ExtraOriginalReference>, u32) -> Ptr<ExtraOriginalReference>
        ),
        entry!(0x004319b0, fn_004319b0(Ptr<ExtraOriginalReference>)),
        entry!(
            0x004319d0,
            extra_original_reference_compare(Ptr<ExtraOriginalReference>, Ptr) -> bool
        ),
        entry!(
            0x00431a40,
            fn_00431a40(Ptr<ExtraOwnership>, Ptr) -> Ptr<ExtraOwnership>
        ),
        entry!(
            0x00431a70,
            extra_ownership_compare(Ptr<ExtraOwnership>, Ptr) -> bool
        ),
        entry!(
            0x00431ae0,
            fn_00431ae0(Ptr<ExtraGlobal>, Ptr) -> Ptr<ExtraGlobal>
        ),
        entry!(
            0x00431b10,
            extra_global_compare(Ptr<ExtraGlobal>, Ptr) -> bool
        ),
        entry!(
            0x00431b80,
            fn_00431b80(Ptr<ExtraRank>, i32) -> Ptr<ExtraRank>
        ),
        entry!(0x00431bb0, extra_rank_compare(Ptr<ExtraRank>, Ptr) -> bool),
        entry!(
            0x00431c20,
            fn_00431c20(Ptr<ExtraCount>, i16) -> Ptr<ExtraCount>
        ),
        entry!(
            0x00431c50,
            extra_count_compare(Ptr<ExtraCount>, Ptr) -> bool
        ),
        entry!(
            0x00431cc0,
            extra_leveled_item_compare(Ptr<ExtraLeveledItem>, Ptr) -> bool
        ),
        entry!(
            0x00431d10,
            fn_00431d10(Ptr<ExtraHealth>, f32) -> Ptr<ExtraHealth>
        ),
        entry!(
            0x00431d40,
            extra_health_compare(Ptr<ExtraHealth>, Ptr) -> bool
        ),
        entry!(
            0x00431db0,
            extra_health_perc_compare(Ptr<ExtraHealthPerc>, Ptr) -> bool
        ),
        entry!(
            0x00431e20,
            fn_00431e20(Ptr<ExtraUses>, u8) -> Ptr<ExtraUses>
        ),
        entry!(0x00431e50, extra_uses_compare(Ptr<ExtraUses>, Ptr) -> bool),
        entry!(
            0x00431ec0,
            fn_00431ec0(Ptr<ExtraTimeLeft>, f32) -> Ptr<ExtraTimeLeft>
        ),
        entry!(
            0x00431ef0,
            extra_time_left_compare(Ptr<ExtraTimeLeft>, Ptr) -> bool
        ),
        entry!(
            0x00431f60,
            fn_00431f60(Ptr<ExtraCharge>, f32) -> Ptr<ExtraCharge>
        ),
        entry!(
            0x00431f90,
            extra_charge_compare(Ptr<ExtraCharge>, Ptr) -> bool
        ),
        entry!(
            0x00432000,
            extra_script_extra_script(Ptr<ExtraScript>, Ptr) -> Ptr<ExtraScript>
        ),
        entry!(0x00432040, fn_00432040(Ptr<ExtraScript>)),
        entry!(
            0x004320d0,
            extra_script_compare(Ptr<ExtraScript>, Ptr) -> bool
        ),
        entry!(
            0x00432140,
            extra_weapon_mod_flags_compare(Ptr<ExtraWeaponModFlags>, Ptr) -> bool
        ),
        entry!(
            0x004321b0,
            extra_modding_item_compare(Ptr<ExtraModdingItem>, Ptr) -> bool
        ),
        entry!(
            0x00432220,
            fn_00432220(Ptr<ExtraScale>, f32) -> Ptr<ExtraScale>
        ),
        entry!(
            0x00432250,
            extra_scale_compare(Ptr<ExtraScale>, Ptr) -> bool
        ),
        entry!(0x004322c0, fn_004322c0(Ptr<ExtraGhost>) -> Ptr<ExtraGhost>),
        entry!(0x004322f0, fn_004322f0(Ptr<ExtraWorn>) -> Ptr<ExtraWorn>),
        entry!(
            0x00432320,
            fn_00432320(Ptr<ExtraWornLeft>) -> Ptr<ExtraWornLeft>
        ),
        entry!(
            0x00432350,
            fn_00432350(Ptr<ExtraCannotWear>) -> Ptr<ExtraCannotWear>
        ),
        entry!(
            0x00432380,
            fn_00432380(Ptr<ExtraHotKey>, i8) -> Ptr<ExtraHotKey>
        ),
        entry!(
            0x004323b0,
            extra_info_general_topic_destructor(Ptr<ExtraInfoGeneralTopic>)
        ),
        entry!(
            0x00432440,
            extra_info_general_topic_scalar_deleting_destructor(
                Ptr<ExtraInfoGeneralTopic>,
                u32,
            )
                -> Ptr<ExtraInfoGeneralTopic>
        ),
        entry!(
            0x00432470,
            fn_00432470(Ptr<ExtraInfoGeneralTopic>, Ptr) -> Ptr<ExtraInfoGeneralTopic>
        ),
        entry!(
            0x004324a0,
            fn_004324a0(Ptr<ExtraInfoGeneralTopic>) -> Ptr<ExtraInfoGeneralTopic>
        ),
        entry!(
            0x00432540,
            extra_hot_key_compare(Ptr<ExtraHotKey>, Ptr) -> bool
        ),
        entry!(
            0x004325b0,
            fn_004325b0(Ptr<ExtraSeed>, u8) -> Ptr<ExtraSeed>
        ),
        entry!(0x004325e0, extra_seed_compare(Ptr<ExtraSeed>, Ptr) -> bool),
        entry!(
            0x00432650,
            fn_00432650(Ptr<ExtraPackageStartLocation>) -> Ptr<ExtraPackageStartLocation>
        ),
        entry!(
            0x004326c0,
            fn_004326c0(
                Ptr<ExtraPackageStartLocation>,
                Ptr,
                Ptr,
                Ptr<NiPoint3>,
                f32,
            ) -> Ptr<ExtraPackageStartLocation>
        ),
        entry!(
            0x00432770,
            extra_package_start_location_compare(Ptr<ExtraPackageStartLocation>, Ptr) -> bool
        ),
        entry!(
            0x004327e0,
            fn_004327e0(Ptr<ExtraReferencePointer>, Ptr) -> Ptr<ExtraReferencePointer>
        ),
        entry!(
            0x00432810,
            fn_00432810(Ptr<ExtraPackage>) -> Ptr<ExtraPackage>
        ),
        entry!(
            0x00432870,
            fn_00432870(Ptr<ExtraPackage>, Ptr, i32, Ptr, u8, u8, u8) -> Ptr<ExtraPackage>
        ),
        entry!(
            0x004328d0,
            fn_004328d0(Ptr<ExtraTresPassPackage>, Ptr) -> Ptr<ExtraTresPassPackage>
        ),
        entry!(
            0x00432900,
            extra_tres_pass_package_destructor(Ptr<ExtraTresPassPackage>)
        ),
        entry!(
            0x004329d0,
            fn_004329d0(Ptr<ExtraPlayerCrimeList>) -> Ptr<ExtraPlayerCrimeList>
        ),
        entry!(
            0x00432a70,
            extra_player_crime_list_scalar_deleting_destructor(
                Ptr<ExtraPlayerCrimeList>,
                u32,
            )
                -> Ptr<ExtraPlayerCrimeList>
        ),
        entry!(
            0x00432aa0,
            fn_00432aa0(Ptr<ExtraPlayerCrimeList>, Ptr) -> Ptr<ExtraPlayerCrimeList>
        ),
        entry!(0x00432b50, fn_00432b50(Ptr<ExtraPlayerCrimeList>)),
        entry!(
            0x00432be0,
            fn_00432be0(Ptr<ExtraLeveledItem>, i32) -> Ptr<ExtraLeveledItem>
        ),
        entry!(
            0x00432c20,
            fn_00432c20(Ptr<ExtraPersistentCell>, Ptr) -> Ptr<ExtraPersistentCell>
        ),
        entry!(
            0x00432c60,
            fn_00432c60(Ptr<ExtraPersistentCell>, u32) -> Ptr<ExtraPersistentCell>
        ),
        entry!(0x00432c90, fn_00432c90(Ptr<ExtraPersistentCell>)),
        entry!(
            0x00432cb0,
            extra_rag_doll_data_extra_rag_doll_data(Ptr<ExtraRagDollData>) -> Ptr<ExtraRagDollData>
        ),
        entry!(
            0x00432ce0,
            extra_rag_doll_data_scalar_deleting_destructor(
                Ptr<ExtraRagDollData>,
                u32,
            ) -> Ptr<ExtraRagDollData>
        ),
        entry!(0x00432d10, fn_00432d10(Ptr<ExtraRagDollData>)),
        entry!(0x00432da0, fn_00432da0(Ptr, u32) -> Ptr),
        entry!(
            0x00432dd0,
            extra_rag_doll_data_compare(Ptr<ExtraRagDollData>, Ptr) -> bool
        ),
        entry!(
            0x00432e60,
            extra_encounter_zone_extra_encounter_zone(
                Ptr<ExtraEncounterZone>,
            ) -> Ptr<ExtraEncounterZone>
        ),
        entry!(
            0x00432e90,
            extra_encounter_zone_compare(Ptr<ExtraEncounterZone>, Ptr) -> bool
        ),
        entry!(
            0x00432f00,
            fn_00432f00(Ptr<ExtraUsedMarkers>) -> Ptr<ExtraUsedMarkers>
        ),
        entry!(0x00432f30, fn_00432f30(Ptr)),
        entry!(
            0x00432f50,
            fn_00432f50(Ptr<ExtraReservedMarkers>) -> Ptr<ExtraReservedMarkers>
        ),
        entry!(
            0x00432f80,
            extra_used_markers_get_marker_used(Ptr<ExtraUsedMarkers>, u32) -> bool
        ),
        entry!(
            0x00432fc0,
            extra_used_markers_set_marker_used(Ptr<ExtraUsedMarkers>, u32, u8)
        ),
        entry!(
            0x00433010,
            fn_00433010(Ptr<ExtraRunOncePacks>) -> Ptr<ExtraRunOncePacks>
        ),
        entry!(
            0x004330b0,
            extra_run_once_packs_scalar_deleting_destructor(
                Ptr<ExtraRunOncePacks>,
                u32,
            ) -> Ptr<ExtraRunOncePacks>
        ),
        entry!(0x004330e0, fn_004330e0(Ptr<ExtraRunOncePacks>)),
        entry!(0x004331c0, fn_004331c0(Ptr<ExtraRunOncePacks>, u32, u8)),
        entry!(
            0x00433260,
            fn_00433260(Ptr<ExtraDistantData>) -> Ptr<ExtraDistantData>
        ),
        entry!(
            0x00433300,
            fn_00433300(Ptr<ExtraEnableStateParent>) -> Ptr<ExtraEnableStateParent>
        ),
        entry!(
            0x00433340,
            extra_enable_state_parent_compare(Ptr<ExtraEnableStateParent>, Ptr) -> bool
        ),
        entry!(
            0x004333a0,
            fn_004333a0(Ptr<ExtraEnableStateChildren>) -> Ptr<ExtraEnableStateChildren>
        ),
        entry!(
            0x00433410,
            extra_enable_state_children_scalar_deleting_destructor(
                Ptr<ExtraEnableStateChildren>,
                u32,
            )
                -> Ptr<ExtraEnableStateChildren>
        ),
        entry!(0x00433440, fn_00433440(Ptr<ExtraEnableStateChildren>)),
        entry!(
            0x004334b0,
            fn_004334b0(Ptr<ExtraRandomTeleportMarker>) -> Ptr<ExtraRandomTeleportMarker>
        ),
        entry!(
            0x004334e0,
            extra_random_teleport_marker_compare(Ptr<ExtraRandomTeleportMarker>, Ptr) -> bool
        ),
        entry!(
            0x00433530,
            fn_00433530(Ptr<ExtraLinkedRefChildren>) -> Ptr<ExtraLinkedRefChildren>
        ),
        entry!(
            0x004335a0,
            extra_linked_ref_children_scalar_deleting_destructor(
                Ptr<ExtraLinkedRefChildren>,
                u32,
            )
                -> Ptr<ExtraLinkedRefChildren>
        ),
        entry!(0x004335d0, fn_004335d0(Ptr<ExtraLinkedRefChildren>)),
        entry!(
            0x00433640,
            fn_00433640(Ptr<ExtraLinkedRef>) -> Ptr<ExtraLinkedRef>
        ),
        entry!(
            0x00433670,
            extra_linked_ref_compare(Ptr<ExtraLinkedRef>, Ptr) -> bool
        ),
        entry!(
            0x004336c0,
            fn_004336c0(Ptr<ExtraAshPileRef>) -> Ptr<ExtraAshPileRef>
        ),
        entry!(
            0x004336f0,
            fn_004336f0(u32, Ptr<BSSimpleList>) -> Ptr<RefActivateData>
        ),
        entry!(0x00433750, fn_00433750(Ptr<BSSimpleList>)),
        entry!(
            0x00433790,
            fn_00433790(Ptr<ExtraActivateRefChildren>) -> Ptr<ExtraActivateRefChildren>
        ),
        entry!(
            0x00433800,
            extra_activate_ref_children_scalar_deleting_destructor(
                Ptr<ExtraActivateRefChildren>,
                u32,
            )
                -> Ptr<ExtraActivateRefChildren>
        ),
        entry!(0x00433830, fn_00433830(Ptr<ExtraActivateRefChildren>)),
        entry!(
            0x004338b0,
            fn_004338b0(Ptr<ExtraActivateRef>) -> Ptr<ExtraActivateRef>
        ),
        entry!(
            0x00433940,
            extra_activate_ref_scalar_deleting_destructor(
                Ptr<ExtraActivateRef>,
                u32,
            ) -> Ptr<ExtraActivateRef>
        ),
        entry!(0x00433970, fn_00433970(Ptr<ExtraActivateRef>)),
        entry!(
            0x00433a00,
            fn_00433a00(Ptr<ExtraActivateRef>, u32) -> Ptr<RefActivateData>
        ),
        entry!(
            0x00433a20,
            extra_activate_ref_compare(Ptr<ExtraActivateRef>, Ptr) -> bool
        ),
        entry!(
            0x00433b70,
            extra_activate_ref_copy(Ptr<ExtraActivateRef>, Ptr)
        ),
        entry!(
            0x00433ca0,
            fn_00433ca0(Ptr<ExtraDecalRefs>) -> Ptr<ExtraDecalRefs>
        ),
        entry!(
            0x00433d10,
            extra_decal_refs_scalar_deleting_destructor(
                Ptr<ExtraDecalRefs>,
                u32,
            ) -> Ptr<ExtraDecalRefs>
        ),
        entry!(0x00433d40, fn_00433d40(Ptr<ExtraDecalRefs>)),
        entry!(
            0x00433db0,
            extra_decal_refs_init_item(Ptr<ExtraDecalRefs>, Ptr)
        ),
        entry!(
            0x00433ed0,
            extra_reflector_refs_init_item(Ptr<ExtraReflectorRefs>, Ptr)
        ),
        entry!(
            0x00434050,
            extra_lit_water_refs_init_item(Ptr<ExtraLitWaterRefs>, Ptr)
        ),
        entry!(
            0x004341e0,
            extra_decal_refs_compare(Ptr<ExtraDecalRefs>, Ptr) -> bool
        ),
        entry!(0x004342b0, extra_decal_refs_copy(Ptr<ExtraDecalRefs>, Ptr)),
        entry!(0x004343c0, fn_004343c0(Ptr<ExtraDecalRefs>)),
        entry!(
            0x00434410,
            extra_decal_refs_get_ref_decal_data(Ptr<ExtraDecalRefs>, u32) -> Ptr<RefDecalData>
        ),
        entry!(0x00434480, fn_00434480(Ptr<ExtraDecalRefs>, u32, Ptr, Ptr)),
        entry!(0x00434560, fn_00434560(Ptr<ExtraReflectedRefs>)),
        entry!(
            0x00434620,
            extra_reflected_refs_compare(Ptr<ExtraReflectedRefs>, Ptr) -> bool
        ),
        entry!(
            0x004346f0,
            extra_reflected_refs_copy(Ptr<ExtraReflectedRefs>, Ptr)
        ),
        entry!(0x00434800, fn_00434800(Ptr<ExtraReflectedRefs>, u32)),
        entry!(0x004348a0, fn_004348a0(Ptr<ExtraReflectedRefs>, u32, u32)),
        entry!(0x00434950, fn_00434950(Ptr<ExtraWaterLightRefs>)),
        entry!(
            0x004349c0,
            extra_water_light_refs_compare(Ptr<ExtraWaterLightRefs>, Ptr) -> bool
        ),
        entry!(
            0x00434a70,
            extra_water_light_refs_copy(Ptr<ExtraWaterLightRefs>, Ptr)
        ),
        entry!(0x00434af0, fn_00434af0(Ptr<ExtraReflectorRefs>)),
        entry!(
            0x00434bb0,
            extra_reflector_refs_compare(Ptr<ExtraReflectorRefs>, Ptr) -> bool
        ),
        entry!(
            0x00434c80,
            extra_reflector_refs_copy(Ptr<ExtraReflectorRefs>, Ptr)
        ),
        entry!(
            0x00434d90,
            extra_reflected_refs_clear_reference_data(Ptr<ExtraReflectedRefs>)
        ),
        entry!(0x00434dc0, fn_00434dc0(Ptr<ExtraLitWaterRefs>)),
        entry!(
            0x00434e30,
            extra_lit_water_refs_compare(Ptr<ExtraLitWaterRefs>, Ptr) -> bool
        ),
        entry!(
            0x00434ee0,
            extra_lit_water_refs_copy(Ptr<ExtraLitWaterRefs>, Ptr)
        ),
        entry!(0x00434f60, fn_00434f60(Ptr<ExtraWaterLightRefs>, u32)),
        entry!(0x00434fa0, fn_00434fa0(Ptr<ExtraWaterLightRefs>)),
        entry!(
            0x00434fc0,
            fn_00434fc0(Ptr<ExtraMerchantContainer>) -> Ptr<ExtraMerchantContainer>
        ),
        entry!(
            0x00434ff0,
            extra_merchant_container_compare(Ptr<ExtraMerchantContainer>, Ptr) -> bool
        ),
        entry!(
            0x00435040,
            fn_00435040(Ptr<ExtraLevCreaModifier>) -> Ptr<ExtraLevCreaModifier>
        ),
        entry!(
            0x00435070,
            extra_lev_crea_modifier_compare(Ptr<ExtraLevCreaModifier>, Ptr) -> bool
        ),
        entry!(0x004350c0, fn_004350c0(Ptr<ExtraLevCreaModifier>) -> f32),
        entry!(0x00435100, fn_00435100(Ptr<ExtraLevCreaModifier>) -> i32),
        entry!(
            0x00435150,
            fn_00435150(Ptr<ExtraPoison>, Ptr) -> Ptr<ExtraPoison>
        ),
        entry!(
            0x00435180,
            extra_poison_compare(Ptr<ExtraPoison>, Ptr) -> bool
        ),
        entry!(
            0x004351d0,
            fn_004351d0(Ptr<ExtraLastFinishedSequence>, Ptr) -> Ptr<ExtraLastFinishedSequence>
        ),
        entry!(
            0x00435270,
            extra_last_finished_sequence_scalar_deleting_destructor(
                Ptr<ExtraLastFinishedSequence>,
                u32,
            ) -> Ptr<
                ExtraLastFinishedSequence,
            >
        ),
        entry!(0x004352a0, fn_004352a0(Ptr<ExtraLastFinishedSequence>)),
        entry!(
            0x00435310,
            extra_last_finished_sequence_compare(Ptr<ExtraLastFinishedSequence>, Ptr) -> bool
        ),
        entry!(
            0x00435370,
            fn_00435370(Ptr<ExtraXTarget>) -> Ptr<ExtraXTarget>
        ),
        entry!(
            0x004353a0,
            fn_004353a0(Ptr<ExtraXTarget>, u32) -> Ptr<ExtraXTarget>
        ),
        entry!(0x004353d0, fn_004353d0(Ptr<ExtraXTarget>)),
        entry!(0x0043_5ac0, fn_00435ac0(Ptr)),
        entry!(0x0043_5bb0, fn_00435bb0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_constructor_matches_native_header_and_x87_store_vectors() {
        // Generated cases checked on FalloutNV.exe 1.4.0.525, SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        // Ryzen AI 9 HX 370, FCW 027F / MXCSR 1F80; no game-function doubles.
        let mut e = Engine::new();
        let object = e.mem.alloc(20);
        for (input, output) in [
            (0x0000_0000u32, 0x0000_0000u32),
            (0x8000_0000, 0x8000_0000),
            (0x42c8_8000, 0x42c8_8000),
            (0xbf80_0000, 0xbf80_0000),
            (0x7f80_0001, 0x7fc0_0001),
            (0xff80_0001, 0xffc0_0001),
        ] {
            e.mem.write(object, &[0xaa; 20]);
            assert_eq!(e.call(0x00431d10, &[object, input]).u32(), object);
            let mut expected = [0xaa; 20];
            expected[..4].copy_from_slice(&0x0101_58e4u32.to_le_bytes());
            expected[4] = 0x25;
            expected[8..12].fill(0);
            expected[12..16].copy_from_slice(&output.to_le_bytes());
            assert_eq!(e.mem.bytes(object, 20), expected);
        }
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Test doubles for the callees every function here shares (allocator,
    /// the base `BSExtraData` constructor and destructor, `BSSimpleArray`'s
    /// accessors and append, `_ftol2_sse`) and the pages holding the exe
    /// globals the code reads.
    fn extra_engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        // The base constructor stores the type byte at +4 (`cEtype`).
        e.register(BS_EXTRA_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            ret(a[0])
        });
        e.register(BS_EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        // `BSSimpleArray`: buffer at +4, count at +8.
        e.register(SIMPLE_ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SIMPLE_ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(SIMPLE_ARRAY_ADD, |e, a| {
            let old_count = e.mem.u32(a[0] + 8);
            let old_buffer = e.mem.u32(a[0] + 4);
            let buffer = e.mem.alloc(4 * (old_count + 1));
            for i in 0..old_count {
                let item = e.mem.u32(old_buffer + 4 * i);
                e.mem.set_u32(buffer + 4 * i, item);
            }
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * old_count, item);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, old_count + 1);
            ret(old_count)
        });
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        for page in [0x0101_2000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Fills the `BSSimpleArray` at `array` with `items`.
    fn make_array(e: &mut Engine, array: u32, items: &[u32]) {
        let buffer = e.mem.alloc(4 * items.len().max(1) as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *item);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, items.len() as u32);
        e.mem.set_u32(array + 0xc, items.len().max(1) as u32);
    }

    fn array_items(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    /// An object whose vtable (on the heap) has the given `(byte offset,
    /// function)` slots; the functions are registered by the test.
    fn object_with_vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x700);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    fn extra_type(e: &Engine, extra: u32) -> u8 {
        e.mem.u8(extra + 4)
    }

    fn vtable_of(e: &Engine, extra: u32) -> u32 {
        e.mem.u32(extra)
    }

    #[test]
    fn anim_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(ANIMATION_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraAnim> = e.new_object();
        assert_eq!(
            e.call(0x0043_00f0, &args![this, 0u32]).ptr::<ExtraAnim>(),
            this
        );
        assert!(e.mem.block_size(this.addr()).is_some());
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_ANIM_VTABLE);
        assert_eq!(
            e.call(0x0043_00f0, &args![this, 3u32]).ptr::<ExtraAnim>(),
            this
        );
        assert_eq!(e.mem.block_size(this.addr()), None);
    }

    #[test]
    fn anim_destructor_deletes_the_animation_only_when_there_is_one() {
        let mut e = extra_engine();
        e.register(ANIMATION_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraAnim> = e.new_object();
        let animation = e.mem.alloc(8);
        e.set(this, ExtraAnim::pAnimation, Ptr::new(animation));
        start_log(&mut e);
        e.call(0x0043_0120, &args![this]);
        assert_eq!(calls(&e, ANIMATION_DELETE), vec![vec![animation, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_ANIM_VTABLE);

        let bare: Ptr<ExtraAnim> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_0120, &args![bare]);
        assert!(calls(&e, ANIMATION_DELETE).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT).len(), 1);
    }

    #[test]
    fn slot_table_entry_maps_the_whole_and_upper_body_sections() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x80);
        for slot in 0..10u32 {
            e.mem.set_u16(this + 0x4c + 2 * slot, 100 + slot as u16);
        }
        let read = |e: &mut Engine, section: i32| e.call(0x0043_01b0, &args![this, section]).u16();
        assert_eq!(read(&mut e, 0x14), 101);
        assert_eq!(read(&mut e, 0x15), 104);
        assert_eq!(read(&mut e, 0), 100);
        assert_eq!(read(&mut e, 7), 107);
    }

    #[test]
    fn dismembered_limbs_constructor_sets_the_empty_state() {
        let mut e = extra_engine();
        e.register(DISMEMBERED_LIMBS_ARRAY_CONSTRUCT, |_, _| Ret::default());
        let this: Ptr<ExtraDismemberedLimbs> = e.new_object();
        for offset in (0x0c..0x20).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        let back = e
            .call(0x0043_0200, &args![this])
            .ptr::<ExtraDismemberedLimbs>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x5f);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_DISMEMBERED_LIMBS_VTABLE);
        assert_eq!(
            calls(&e, DISMEMBERED_LIMBS_ARRAY_CONSTRUCT),
            vec![vec![this.addr() + 0x20]]
        );
        assert_eq!(e.get(this, ExtraDismemberedLimbs::sLimbs), 0);
        assert_eq!(
            e.get(this, ExtraDismemberedLimbs::eCauseofDeath),
            0xffff_ffff
        );
        assert!(e.get(this, ExtraDismemberedLimbs::pDeathObject).is_null());
        assert_eq!(
            e.get(this, ExtraDismemberedLimbs::eLastHitLimb),
            0xffff_ffff
        );
        assert!(!e.get(this, ExtraDismemberedLimbs::bEaten));
    }

    #[test]
    fn dismembered_limbs_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(DISMEMBERED_LIMBS_ARRAY_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<ExtraDismemberedLimbs> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_02a0, &args![this, 0u32]);
        assert!(e.mem.block_size(this.addr()).is_some());
        let back = e
            .call(0x0043_02a0, &args![this, 1u32])
            .ptr::<ExtraDismemberedLimbs>();
        assert_eq!(back, this);
        assert_eq!(e.mem.block_size(this.addr()), None);
        assert_eq!(calls(&e, DISMEMBERED_LIMBS_ARRAY_DESTRUCT).len(), 2);
    }

    #[test]
    fn dismembered_limbs_destructor_deletes_each_entry_and_the_array() {
        let mut e = extra_engine();
        e.register(DISMEMBERED_LIMBS_ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(DISMEMBERED_LIMB_ARRAY_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<ExtraDismemberedLimbs> = e.new_object();
        let first = e.mem.alloc(0x14);
        let last = e.mem.alloc(0x14);
        make_array(&mut e, this.addr() + 0x20, &[first, 0, last]);
        start_log(&mut e);
        e.call(0x0043_02d0, &args![this]);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.mem.block_size(last), None);
        assert_eq!(
            calls(&e, DISMEMBERED_LIMB_ARRAY_DESTRUCT),
            vec![vec![first + 4], vec![last + 4]]
        );
        assert_eq!(
            calls(&e, DISMEMBERED_LIMBS_ARRAY_DESTRUCT),
            vec![vec![this.addr() + 0x20]]
        );
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_DISMEMBERED_LIMBS_VTABLE);
    }

    #[test]
    fn dismembered_limb_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(DISMEMBERED_LIMB_ARRAY_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<DismemberedLimb> = e.new_object();
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_0390, &args![this, 0u32])
                .ptr::<DismemberedLimb>(),
            this
        );
        assert!(e.mem.block_size(this.addr()).is_some());
        e.call(0x0043_0390, &args![this, 1u32]);
        assert_eq!(e.mem.block_size(this.addr()), None);
        assert_eq!(calls(&e, DISMEMBERED_LIMB_ARRAY_DESTRUCT).len(), 2);
    }

    #[test]
    fn dismembered_limb_destructor_destroys_the_object_array() {
        let mut e = extra_engine();
        e.register(DISMEMBERED_LIMB_ARRAY_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<DismemberedLimb> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_03c0, &args![this]);
        assert_eq!(
            calls(&e, DISMEMBERED_LIMB_ARRAY_DESTRUCT),
            vec![vec![this.addr() + 4]]
        );
    }

    #[test]
    fn dismembered_tests_the_limb_bit() {
        let mut e = extra_engine();
        let this: Ptr<ExtraDismemberedLimbs> = e.new_object();
        e.set(this, ExtraDismemberedLimbs::sLimbs, 0b101);
        let test = |e: &mut Engine, limb: u8| e.call(0x0043_03e0, &args![this, limb]).bool();
        assert!(test(&mut e, 0));
        assert!(!test(&mut e, 1));
        assert!(test(&mut e, 2));
        // The shift count wraps at 32, as `SHL` does.
        assert!(test(&mut e, 32));
        assert!(!test(&mut e, 33));
    }

    /// Everything `Dismember` calls, with the actor's vtable answering
    /// `+0x100` with `is_actor` and `+0x218` with `flag`; `+0x1e8` gives
    /// `0x7000_0001`. `identical` is what the array comparison answers.
    fn dismember_setup(is_actor: bool, flag: bool, identical: bool) -> (Engine, u32, u32) {
        let mut e = extra_engine();
        e.register(DISMEMBERED_LIMB_CONSTRUCT, |_, a| ret(a[0]));
        e.register(REFR_NPC_ACCESSOR, |_, _| ret(0x4e50));
        e.register(TESNPC_BUILD_OBJECT_ARRAY, |_, _| Ret::default());
        e.register(SIMPLE_ARRAY_CLEAR, |_, _| Ret::default());
        e.register_double(SIMPLE_ARRAY_COMPARE_BUFFER, move |_, _| {
            ret(identical as u32)
        });
        e.register_double(0x00a0_0100, move |_, _| ret(is_actor as u32));
        e.register_double(0x00a0_0218, move |_, _| ret(flag as u32));
        e.register(0x00a0_01e8, |_, _| ret(0x7000_0001));
        let actor = object_with_vtable(
            &mut e,
            &[
                (0x100, 0x00a0_0100),
                (0x218, 0x00a0_0218),
                (0x1e8, 0x00a0_01e8),
            ],
        );
        let this = e.mem.alloc(0x30);
        (e, this, actor)
    }

    /// A `DismemberedLimb` entry with the given limb and identical flag.
    fn limb_entry(e: &mut Engine, limb: u8, identical: u8) -> u32 {
        let entry = e.mem.alloc(0x14);
        e.mem.set_u8(entry, limb);
        e.mem.set_u8(entry + 2, identical);
        entry
    }

    fn dismember(e: &mut Engine, this: u32, actor: u32, limb: u8, exploded: u8) {
        e.call(
            0x0043_0410,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(actor), limb, exploded],
        );
    }

    fn entry_flags(e: &Engine, entry: u32) -> [u8; 4] {
        [
            e.mem.u8(entry),
            e.mem.u8(entry + 1),
            e.mem.u8(entry + 2),
            e.mem.u8(entry + 3),
        ]
    }

    #[test]
    fn dismember_ignores_a_limb_that_is_already_dismembered() {
        let (mut e, this, actor) = dismember_setup(true, true, false);
        e.mem.set_u16(this + 0xc, 0b100);
        start_log(&mut e);
        dismember(&mut e, this, actor, 2, 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0b100);
    }

    #[test]
    fn dismember_an_exploded_limb_is_marked_identical_at_once() {
        let (mut e, this, actor) = dismember_setup(true, true, false);
        start_log(&mut e);
        dismember(&mut e, this, actor, 5, 1);
        assert_eq!(e.mem.u16(this + 0xc), 1 << 5);
        let entries = array_items(&e, this + 0x20);
        assert_eq!(entries.len(), 1);
        let entry = entries[0];
        assert_eq!(entry_flags(&e, entry), [5, 1, 1, 0]);
        assert!(calls(&e, TESNPC_BUILD_OBJECT_ARRAY).is_empty());
        assert_eq!(calls(&e, DISMEMBERED_LIMB_CONSTRUCT), vec![vec![entry]]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x14]]);
    }

    #[test]
    fn dismember_builds_nothing_for_a_non_actor_the_player_or_a_flagged_actor() {
        // Vtable +0x100 false.
        let (mut e, this, actor) = dismember_setup(false, true, true);
        start_log(&mut e);
        dismember(&mut e, this, actor, 1, 0);
        let entry = array_items(&e, this + 0x20)[0];
        assert_eq!(e.mem.u8(entry + 2), 0);
        assert!(calls(&e, TESNPC_BUILD_OBJECT_ARRAY).is_empty());

        // The player.
        let (mut e, this, actor) = dismember_setup(true, true, true);
        e.set_global(PLAYER, actor);
        start_log(&mut e);
        dismember(&mut e, this, actor, 1, 0);
        assert!(calls(&e, TESNPC_BUILD_OBJECT_ARRAY).is_empty());

        // Vtable +0x218 false.
        let (mut e, this, actor) = dismember_setup(true, false, true);
        start_log(&mut e);
        dismember(&mut e, this, actor, 1, 0);
        assert!(calls(&e, TESNPC_BUILD_OBJECT_ARRAY).is_empty());
        assert_eq!(array_items(&e, this + 0x20).len(), 1);
    }

    #[test]
    fn dismember_marks_a_duplicate_object_array_identical_and_clears_it() {
        let (mut e, this, actor) = dismember_setup(true, true, true);
        let older = limb_entry(&mut e, 1, 0);
        let newest_identical = limb_entry(&mut e, 2, 1);
        make_array(&mut e, this + 0x20, &[older, newest_identical]);
        start_log(&mut e);
        dismember(&mut e, this, actor, 5, 0);
        let entries = array_items(&e, this + 0x20);
        assert_eq!(entries.len(), 3);
        let entry = entries[2];
        assert_eq!(entry_flags(&e, entry), [5, 0, 1, 0]);
        assert_eq!(e.mem.u16(this + 0xc), 1 << 5);
        assert_eq!(
            calls(&e, TESNPC_BUILD_OBJECT_ARRAY),
            vec![vec![0x4e50, actor, 0x7000_0001, entry + 4]]
        );
        // The entry marked identical is skipped; the older one is compared.
        assert_eq!(
            calls(&e, SIMPLE_ARRAY_COMPARE_BUFFER),
            vec![vec![older + 4, entry + 4]]
        );
        assert_eq!(calls(&e, SIMPLE_ARRAY_CLEAR), vec![vec![entry + 4, 1]]);
    }

    #[test]
    fn dismember_keeps_a_different_object_array_and_stops_without_earlier_entries() {
        let (mut e, this, actor) = dismember_setup(true, true, false);
        let older = limb_entry(&mut e, 1, 0);
        make_array(&mut e, this + 0x20, &[older]);
        start_log(&mut e);
        dismember(&mut e, this, actor, 5, 0);
        let entry = array_items(&e, this + 0x20)[1];
        assert_eq!(e.mem.u8(entry + 2), 0);
        assert_eq!(calls(&e, SIMPLE_ARRAY_COMPARE_BUFFER).len(), 1);
        assert!(calls(&e, SIMPLE_ARRAY_CLEAR).is_empty());

        // Only an earlier entry that is itself identical: nothing to compare.
        let (mut e, this, actor) = dismember_setup(true, true, true);
        let skipped = limb_entry(&mut e, 1, 1);
        make_array(&mut e, this + 0x20, &[skipped]);
        start_log(&mut e);
        dismember(&mut e, this, actor, 5, 0);
        assert!(calls(&e, SIMPLE_ARRAY_COMPARE_BUFFER).is_empty());

        // No earlier entry at all.
        let (mut e, this, actor) = dismember_setup(true, true, true);
        start_log(&mut e);
        dismember(&mut e, this, actor, 5, 0);
        assert!(calls(&e, SIMPLE_ARRAY_COMPARE_BUFFER).is_empty());
        assert_eq!(calls(&e, TESNPC_BUILD_OBJECT_ARRAY).len(), 1);
    }

    #[test]
    fn set_removed_updates_every_entry_of_the_limb() {
        let mut e = extra_engine();
        let this: Ptr<ExtraDismemberedLimbs> = e.new_object();
        let first = limb_entry(&mut e, 3, 0);
        let other = limb_entry(&mut e, 4, 0);
        let last = limb_entry(&mut e, 3, 0);
        make_array(&mut e, this.addr() + 0x20, &[first, 0, other, last]);
        e.call(0x0043_05f0, &args![this, 3u32, 1u8]);
        assert_eq!(e.mem.u8(first + 3), 1);
        assert_eq!(e.mem.u8(other + 3), 0);
        assert_eq!(e.mem.u8(last + 3), 1);
        e.call(0x0043_05f0, &args![this, 3u32, 0u8]);
        assert_eq!(e.mem.u8(first + 3), 0);
        assert_eq!(e.mem.u8(last + 3), 0);
    }

    /// An `ExtraDismemberedLimbs` with the given limb mask and entries
    /// `(limb, exploded, identical, removed)`.
    fn limbs_object(e: &mut Engine, mask: u16, entries: &[(u8, u8, u8, u8)]) -> u32 {
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 0xc, mask);
        let mut pointers = vec![];
        for (limb, exploded, identical, removed) in entries {
            let entry = limb_entry(e, *limb, *identical);
            e.mem.set_u8(entry + 1, *exploded);
            e.mem.set_u8(entry + 3, *removed);
            pointers.push(entry);
        }
        make_array(e, this + 0x20, &pointers);
        this
    }

    fn limbs_identical(e: &mut Engine, this: u32, other: u32) -> bool {
        e.call(
            0x0043_0660,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(other)],
        )
        .bool()
    }

    #[test]
    fn check_dismembered_limbs_identical_compares_every_field() {
        let mut e = extra_engine();
        e.register(SIMPLE_ARRAY_COMPARE_BUFFER, |_, _| ret(1));
        let same = [(1, 0, 0, 0), (2, 1, 1, 0)];
        let this = limbs_object(&mut e, 0b110, &same);
        let twin = limbs_object(&mut e, 0b110, &same);
        start_log(&mut e);
        assert!(limbs_identical(&mut e, this, twin));
        let this_first = e.mem.u32(e.mem.u32(this + 0x24));
        let twin_first = e.mem.u32(e.mem.u32(twin + 0x24));
        assert_eq!(
            calls(&e, SIMPLE_ARRAY_COMPARE_BUFFER)[0],
            vec![this_first + 4, twin_first + 4]
        );

        let other_mask = limbs_object(&mut e, 0b111, &same);
        assert!(!limbs_identical(&mut e, this, other_mask));
        let shorter = limbs_object(&mut e, 0b110, &same[..1]);
        assert!(!limbs_identical(&mut e, this, shorter));
        for field in 0..4 {
            let mut changed = same;
            match field {
                0 => changed[1].0 = 9,
                1 => changed[1].1 = 0,
                2 => changed[1].2 = 0,
                _ => changed[1].3 = 1,
            }
            let other = limbs_object(&mut e, 0b110, &changed);
            assert!(!limbs_identical(&mut e, this, other), "field {field}");
        }
    }

    #[test]
    fn check_dismembered_limbs_identical_fails_on_different_object_arrays() {
        let mut e = extra_engine();
        e.register(SIMPLE_ARRAY_COMPARE_BUFFER, |_, _| ret(0));
        let this = limbs_object(&mut e, 1, &[(0, 0, 0, 0)]);
        let other = limbs_object(&mut e, 1, &[(0, 0, 0, 0)]);
        assert!(!limbs_identical(&mut e, this, other));
        // Two empty lists are identical without comparing anything.
        let empty = limbs_object(&mut e, 0, &[]);
        let also_empty = limbs_object(&mut e, 0, &[]);
        assert!(limbs_identical(&mut e, empty, also_empty));
    }

    #[test]
    fn starting_position_constructor_copies_position_and_rotation() {
        let mut e = extra_engine();
        e.register(FILE_POS_ROT_CONSTRUCT, |_, _| Ret::default());
        e.register(0x00a0_01f4, |_, a| ret(a[0] + 0x30));
        let reference = object_with_vtable(&mut e, &[(0x1f4, 0x00a0_01f4)]);
        for (i, value) in [1.5f32, -2.0, 3.25].iter().enumerate() {
            e.mem.set_f32(reference + 0x30 + 4 * i as u32, *value);
        }
        for (i, value) in [0.5f32, 0.25, 8.0].iter().enumerate() {
            e.mem.set_f32(reference + 0x24 + 4 * i as u32, *value);
        }
        let this: Ptr<ExtraStartingPosition> = e.new_object();
        start_log(&mut e);
        let back = e
            .call(0x0043_0780, &args![this, Ptr::<()>::new(reference)])
            .ptr::<ExtraStartingPosition>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x0f);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_STARTING_POSITION_VTABLE);
        assert_eq!(
            calls(&e, FILE_POS_ROT_CONSTRUCT),
            vec![vec![this.addr() + 0xc]]
        );
        let start = this.at(ExtraStartingPosition::startPosition);
        let position = start.at(FilePosRot::pos);
        let rotation = start.at(FilePosRot::rot);
        assert_eq!(e.get(position, NiPoint3::x), 1.5);
        assert_eq!(e.get(position, NiPoint3::y), -2.0);
        assert_eq!(e.get(position, NiPoint3::z), 3.25);
        assert_eq!(e.get(rotation, NiPoint3::x), 0.5);
        assert_eq!(e.get(rotation, NiPoint3::y), 0.25);
        assert_eq!(e.get(rotation, NiPoint3::z), 8.0);
        assert_eq!(rotation.addr(), this.addr() + 0x18);
    }

    #[test]
    fn vector_accessor_is_at_offset_0x24() {
        let mut e = extra_engine();
        assert_eq!(
            e.call(0x0043_0830, &args![Ptr::<()>::new(0x1000)]).u32(),
            0x1024
        );
    }

    /// The doubles `Compare` methods use: a cast that gives back its
    /// argument (or null), the base `Compare`, and a real `memcmp`.
    fn compare_engine(cast_ok: bool, base_result: bool) -> Engine {
        let mut e = extra_engine();
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(if cast_ok { a[0] } else { 0 })
        });
        e.register_double(BS_EXTRA_DATA_COMPARE, move |_, _| ret(base_result as u32));
        e.register(MEMCMP, |e, a| {
            let first = e.mem.bytes(a[0], a[2]);
            let second = e.mem.bytes(a[1], a[2]);
            ret(match first.cmp(&second) {
                std::cmp::Ordering::Less => u32::MAX,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })
        });
        e
    }

    fn starting_position_compare(
        e: &mut Engine,
        this: Ptr<ExtraStartingPosition>,
        other: Ptr<ExtraStartingPosition>,
    ) -> bool {
        e.call(0x0043_0850, &args![this, other]).bool()
    }

    #[test]
    fn starting_position_compare_checks_type_base_and_the_position_bytes() {
        // Not an ExtraStartingPosition.
        let mut e = compare_engine(false, false);
        let this: Ptr<ExtraStartingPosition> = e.new_object();
        let other: Ptr<ExtraStartingPosition> = e.new_object();
        start_log(&mut e);
        assert!(starting_position_compare(&mut e, this, other));
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other.addr(),
                0,
                BS_EXTRA_DATA_TYPE,
                EXTRA_STARTING_POSITION_TYPE,
                0
            ]]
        );
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());

        // The base says they differ.
        let mut e = compare_engine(true, true);
        let this: Ptr<ExtraStartingPosition> = e.new_object();
        let other: Ptr<ExtraStartingPosition> = e.new_object();
        assert!(starting_position_compare(&mut e, this, other));

        // Same bytes: equal. One byte of the rotation differs: not.
        let mut e = compare_engine(true, false);
        let this: Ptr<ExtraStartingPosition> = e.new_object();
        let other: Ptr<ExtraStartingPosition> = e.new_object();
        start_log(&mut e);
        assert!(!starting_position_compare(&mut e, this, other));
        assert_eq!(
            calls(&e, MEMCMP),
            vec![vec![this.addr() + 0xc, other.addr() + 0xc, 0x18]]
        );
        e.mem.set_u8(other.addr() + 0x23, 1);
        assert!(starting_position_compare(&mut e, this, other));
    }

    #[test]
    fn type_49_constructor_clears_its_word() {
        let mut e = extra_engine();
        let this: Ptr<ExtraType49> = e.new_object();
        e.set(this, ExtraType49::unnamed0C, 0xdead);
        let back = e.call(0x0043_08c0, &args![this]).ptr::<ExtraType49>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x49);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_TYPE_49_VTABLE);
        assert_eq!(e.get(this, ExtraType49::unnamed0C), 0);
    }

    #[test]
    fn light_constructor_takes_the_light() {
        let mut e = extra_engine();
        let this: Ptr<ExtraLight> = e.new_object();
        let back = e
            .call(0x0043_08f0, &args![this, Ptr::<()>::new(0x4444)])
            .ptr::<ExtraLight>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x29);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_LIGHT_VTABLE);
        assert_eq!(e.get(this, ExtraLight::pLight).addr(), 0x4444);
    }

    #[test]
    fn light_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(LIGHT_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraLight> = e.new_object();
        e.call(0x0043_0920, &args![this, 0u32]);
        assert!(e.mem.block_size(this.addr()).is_some());
        e.call(0x0043_0920, &args![this, 1u32]);
        assert_eq!(e.mem.block_size(this.addr()), None);
    }

    #[test]
    fn light_destructor_deletes_the_light_only_when_there_is_one() {
        let mut e = extra_engine();
        e.register(LIGHT_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraLight> = e.new_object();
        e.set(this, ExtraLight::pLight, Ptr::new(0x4444));
        start_log(&mut e);
        e.call(0x0043_0950, &args![this]);
        assert_eq!(calls(&e, LIGHT_DELETE), vec![vec![0x4444, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        let bare: Ptr<ExtraLight> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_0950, &args![bare]);
        assert!(calls(&e, LIGHT_DELETE).is_empty());
    }

    /// The setting accessor: the five bracket settings hold 0, 25, 50, 75,
    /// 100 and the sixth 7.
    fn register_lock_settings(e: &mut Engine) {
        e.register(SETTING_INT_VALUE, |e, a| {
            let values = [0, 25, 50, 75, 100, 7];
            let index = LOCK_LEVEL_SETTINGS
                .iter()
                .position(|setting| *setting == a[0])
                .unwrap();
            let cell = e.mem.alloc(4);
            e.mem.set_i32(cell, values[index]);
            ret(cell)
        });
    }

    fn lock_engine() -> Engine {
        let mut e = extra_engine();
        register_lock_settings(&mut e);
        e
    }

    fn lock(e: &mut Engine, level: u8, flags: i8) -> Ptr<RefrLock> {
        let lock: Ptr<RefrLock> = e.new_object();
        e.set(lock, RefrLock::cBaseLevel, level);
        e.set(lock, RefrLock::cFlags, flags);
        lock
    }

    #[test]
    fn lock_level_is_the_bracket_of_the_level() {
        let mut e = lock_engine();
        let plain = lock(&mut e, 30, 0);
        assert_eq!(
            e.call(0x0043_09e0, &args![plain, Ptr::<()>::new(0)]).i32(),
            2
        );
        let key_only = lock(&mut e, 120, 0);
        assert_eq!(
            e.call(0x0043_09e0, &args![key_only, Ptr::<()>::new(0)])
                .i32(),
            5
        );
    }

    #[test]
    fn lock_level_adds_the_reference_level_for_a_leveled_lock() {
        let mut e = lock_engine();
        e.register(SETTING_FLOAT_VALUE, |e, _| {
            let cell = e.mem.alloc(4);
            e.mem.set_f32(cell, 2.5);
            ret(cell)
        });
        e.register(REFR_GET_CALC_LEVEL, |_, a| {
            assert_eq!(a[1], 0);
            ret(10)
        });
        e.register(ACTOR_GET_LEVEL, |_, _| ret(0xabcd_0005));
        e.set_global(PLAYER, 0x5000);
        let reference = Ptr::<()>::new(0x6000);

        // Unleveled: the base level, no lookups.
        let plain = lock(&mut e, 30, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_0a10, &args![plain, reference]).i32(), 30);
        assert!(calls(&e, REFR_GET_CALC_LEVEL).is_empty());

        // Leveled with a reference: 30 + trunc(2.5) * 10.
        let leveled = lock(&mut e, 30, 4);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_0a10, &args![leveled, reference]).i32(), 50);
        assert_eq!(calls(&e, REFR_GET_CALC_LEVEL), vec![vec![0x6000, 0]]);
        assert_eq!(
            calls(&e, SETTING_FLOAT_VALUE),
            vec![vec![LEVELED_LOCK_SETTING]]
        );

        // Leveled without one: the player's level, 16 bits of it.
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_0a10, &args![leveled, Ptr::<()>::new(0)])
                .i32(),
            30 + 2 * 5
        );
        assert_eq!(calls(&e, ACTOR_GET_LEVEL), vec![vec![0x5000]]);

        // Capped at 99.
        let high = lock(&mut e, 90, 5);
        assert_eq!(e.call(0x0043_0a10, &args![high, reference]).i32(), 99);
        // The flag byte is signed: a set top bit does not matter.
        let negative = lock(&mut e, 10, -128);
        assert_eq!(e.call(0x0043_0a10, &args![negative, reference]).i32(), 10);
    }

    #[test]
    fn set_locked_sets_or_clears_bit_0_and_resets_tries() {
        let mut e = lock_engine();
        let this = lock(&mut e, 0, 4);
        e.set(this, RefrLock::uiNumTries, 3);
        e.call(0x0043_0a90, &args![this, true]);
        assert_eq!(e.get(this, RefrLock::cFlags), 5);
        assert_eq!(e.get(this, RefrLock::uiNumTries), 3);
        e.call(0x0043_0a90, &args![this, false]);
        assert_eq!(e.get(this, RefrLock::cFlags), 4);
        assert_eq!(e.get(this, RefrLock::uiNumTries), 0);
    }

    #[test]
    fn is_broken_needs_one_try_or_two_by_the_entry_point() {
        for (entry_point_value, tries, broken) in [
            (0.0f32, 0, false),
            (0.0, 1, true),
            (1.0, 1, false),
            (1.0, 2, true),
            (-0.5, 5, true),
        ] {
            let mut e = lock_engine();
            e.set_global(PLAYER, 0x5000);
            e.register_double(HANDLE_ENTRY_POINT, move |e, a| {
                e.mem.set_f32(a[2], entry_point_value);
                Ret::default()
            });
            let this = lock(&mut e, 0, 1);
            e.set(this, RefrLock::uiNumTries, tries);
            start_log(&mut e);
            assert_eq!(
                e.call(0x0043_0ae0, &args![this]).bool(),
                broken,
                "{entry_point_value} {tries}"
            );
            let call = &calls(&e, HANDLE_ENTRY_POINT)[0];
            assert_eq!(&call[..2], &[0x20, 0x5000]);
        }
    }

    #[test]
    fn numeric_value_to_enum_finds_the_first_bracket() {
        let mut e = lock_engine();
        for (value, expected) in [
            (-5, 0),
            (0, 0),
            (1, 1),
            (25, 1),
            (26, 2),
            (75, 3),
            (100, 4),
            (101, 5),
        ] {
            assert_eq!(
                e.call(0x0043_0b40, &args![value]).i32(),
                expected,
                "value {value}"
            );
        }
        // Stops at the first bracket that fits.
        start_log(&mut e);
        e.call(0x0043_0b40, &args![30i32]);
        assert_eq!(calls(&e, SETTING_INT_VALUE).len(), 3);
    }

    #[test]
    fn enum_to_numeric_value_reads_the_bracket_setting() {
        let mut e = lock_engine();
        for (bracket, expected) in [
            (0u32, 0),
            (1, 25),
            (2, 50),
            (3, 75),
            (4, 100),
            (5, 7),
            (6, 0),
            (99, 0),
        ] {
            assert_eq!(
                e.call(0x0043_0bc0, &args![bracket]).i32(),
                expected,
                "bracket {bracket}"
            );
        }
        start_log(&mut e);
        e.call(0x0043_0bc0, &args![6u32]);
        assert!(calls(&e, SETTING_INT_VALUE).is_empty());
    }

    #[test]
    fn lock_extra_constructor_takes_the_lock() {
        let mut e = extra_engine();
        let this: Ptr<ExtraLock> = e.new_object();
        let held: Ptr<RefrLock> = e.new_object();
        let back = e.call(0x0043_0c70, &args![this, held]).ptr::<ExtraLock>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x2a);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_LOCK_VTABLE);
        assert_eq!(e.get(this, ExtraLock::pLock), held);
    }

    #[test]
    fn lock_extra_destructor_frees_the_lock() {
        let mut e = extra_engine();
        let this: Ptr<ExtraLock> = e.new_object();
        let held: Ptr<RefrLock> = e.new_object();
        e.set(this, ExtraLock::pLock, held);
        start_log(&mut e);
        e.call(0x0043_0ca0, &args![this]);
        assert_eq!(e.mem.block_size(held.addr()), None);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![held.addr()]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_LOCK_VTABLE);
    }

    /// An `ExtraLock` holding a lock with the given level, key, flags, tries
    /// and times unlocked.
    fn extra_lock(
        e: &mut Engine,
        (level, key, flags, tries, unlocked): (u8, u32, i8, u32, u32),
    ) -> Ptr<ExtraLock> {
        let held = lock(e, level, flags);
        e.set(held, RefrLock::pKey, Ptr::new(key));
        e.set(held, RefrLock::uiNumTries, tries);
        e.set(held, RefrLock::uiTimesUnlocked, unlocked);
        let this: Ptr<ExtraLock> = e.new_object();
        e.set(this, ExtraLock::pLock, held);
        this
    }

    fn lock_compare_engine(cast_ok: bool, base_result: bool) -> Engine {
        let mut e = compare_engine(cast_ok, base_result);
        register_lock_settings(&mut e);
        e
    }

    #[test]
    fn lock_compare_checks_type_base_and_every_lock_field() {
        let base = (30, 7, 1, 2, 3);
        let mut e = lock_compare_engine(false, false);
        let this = extra_lock(&mut e, base);
        let other = extra_lock(&mut e, base);
        start_log(&mut e);
        assert!(e.call(0x0043_0ce0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other.addr(),
                0,
                BS_EXTRA_DATA_TYPE,
                EXTRA_LOCK_TYPE,
                0
            ]]
        );

        let mut e = lock_compare_engine(true, true);
        let this = extra_lock(&mut e, base);
        let other = extra_lock(&mut e, base);
        assert!(e.call(0x0043_0ce0, &args![this, other]).bool());

        let mut e = lock_compare_engine(true, false);
        let this = extra_lock(&mut e, base);
        // Equal, including a base level that differs inside the same bracket.
        for same in [base, (40, 7, 1, 2, 3)] {
            let other = extra_lock(&mut e, same);
            assert!(!e.call(0x0043_0ce0, &args![this, other]).bool());
        }
        // One field differs each time.
        for (i, different) in [
            (30, 7, 5, 2, 3),
            (30, 8, 1, 2, 3),
            (60, 7, 1, 2, 3),
            (30, 7, 1, 9, 3),
            (30, 7, 1, 2, 9),
        ]
        .into_iter()
        .enumerate()
        {
            let other = extra_lock(&mut e, different);
            assert!(e.call(0x0043_0ce0, &args![this, other]).bool(), "field {i}");
        }
    }

    #[test]
    fn follower_constructor_allocates_the_actor_list() {
        let mut e = extra_engine();
        e.register(ACTOR_LIST_CONSTRUCT, |_, a| ret(a[0]));
        let this: Ptr<ExtraFollower> = e.new_object();
        start_log(&mut e);
        let back = e.call(0x0043_0dd0, &args![this]).ptr::<ExtraFollower>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x1d);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_FOLLOWER_VTABLE);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        let list = e.get(this, ExtraFollower::pActorlist);
        assert_eq!(e.mem.block_size(list.addr()), Some(8));
        assert_eq!(calls(&e, ACTOR_LIST_CONSTRUCT), vec![vec![list.addr()]]);

        // A failed allocation leaves the list null and constructs nothing.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let failed: Ptr<ExtraFollower> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_0dd0, &args![failed]);
        assert!(e.get(failed, ExtraFollower::pActorlist).is_null());
        assert!(calls(&e, ACTOR_LIST_CONSTRUCT).is_empty());
    }

    #[test]
    fn follower_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(ACTOR_LIST_CLEAR, |_, _| Ret::default());
        let this: Ptr<ExtraFollower> = e.new_object();
        e.call(0x0043_0e70, &args![this, 0u32]);
        assert!(e.mem.block_size(this.addr()).is_some());
        let back = e
            .call(0x0043_0e70, &args![this, 1u32])
            .ptr::<ExtraFollower>();
        assert_eq!(back, this);
        assert_eq!(e.mem.block_size(this.addr()), None);
    }

    #[test]
    fn follower_destructor_clears_then_deletes_the_list() {
        let mut e = extra_engine();
        e.register(ACTOR_LIST_CLEAR, |_, _| Ret::default());
        e.register(ACTOR_LIST_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraFollower> = e.new_object();
        e.set(this, ExtraFollower::pActorlist, Ptr::new(0x4000));
        start_log(&mut e);
        e.call(0x0043_0ea0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|(addr, _)| *addr)
            .collect();
        assert_eq!(
            order,
            vec![ACTOR_LIST_CLEAR, ACTOR_LIST_DELETE, BS_EXTRA_DATA_DESTRUCT]
        );
        assert_eq!(calls(&e, ACTOR_LIST_DELETE), vec![vec![0x4000, 1]]);

        // Without a list the clear is still called (on null) and nothing is
        // deleted.
        let bare: Ptr<ExtraFollower> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_0ea0, &args![bare]);
        assert_eq!(calls(&e, ACTOR_LIST_CLEAR), vec![vec![0]]);
        assert!(calls(&e, ACTOR_LIST_DELETE).is_empty());
        assert_eq!(vtable_of(&e, bare.addr()), EXTRA_FOLLOWER_VTABLE);
    }

    #[test]
    fn guarded_ref_data_constructor_builds_the_guard_array() {
        let mut e = extra_engine();
        e.register(GUARDS_ARRAY_CONSTRUCT, |_, _| Ret::default());
        let this: Ptr<ExtraGuardedRefData> = e.new_object();
        start_log(&mut e);
        let back = e
            .call(0x0043_0f30, &args![this])
            .ptr::<ExtraGuardedRefData>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x7c);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_GUARDED_REF_DATA_VTABLE);
        assert_eq!(
            calls(&e, GUARDS_ARRAY_CONSTRUCT),
            vec![vec![this.addr() + 0xc]]
        );
    }

    #[test]
    fn guarded_ref_data_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(GUARDS_ARRAY_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<ExtraGuardedRefData> = e.new_object();
        e.call(0x0043_0fa0, &args![this, 0u32]);
        assert!(e.mem.block_size(this.addr()).is_some());
        let back = e
            .call(0x0043_0fa0, &args![this, 1u32])
            .ptr::<ExtraGuardedRefData>();
        assert_eq!(back, this);
        assert_eq!(e.mem.block_size(this.addr()), None);
    }

    #[test]
    fn guarded_ref_data_destructor_destroys_the_guard_array() {
        let mut e = extra_engine();
        e.register(GUARDS_ARRAY_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<ExtraGuardedRefData> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_0fd0, &args![this]);
        assert_eq!(
            calls(&e, GUARDS_ARRAY_DESTRUCT),
            vec![vec![this.addr() + 0xc]]
        );
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_GUARDED_REF_DATA_VTABLE);
    }

    /// `BSSimpleArray::Find` of the guards: the index of the word at `a[1]`
    /// in the array `a[0]`, or -1.
    fn register_find(e: &mut Engine) {
        e.register(SIMPLE_ARRAY_FIND, |e, a| {
            assert_eq!(&a[2..], &[0, GUARD_COMPARE]);
            let wanted = e.mem.u32(a[1]);
            let found = array_items(e, a[0]).iter().position(|item| *item == wanted);
            ret(found.map_or(u32::MAX, |i| i as u32))
        });
    }

    fn guards_object(e: &mut Engine, guards: &[u32]) -> Ptr<ExtraGuardedRefData> {
        let this: Ptr<ExtraGuardedRefData> = e.new_object();
        make_array(e, this.addr() + 0xc, guards);
        this
    }

    #[test]
    fn guarded_ref_data_compare_needs_the_same_guards() {
        // The base test is the other way round from the other `Compare`
        // methods: a base result of false means "differs".
        let mut e = compare_engine(true, true);
        register_find(&mut e);
        let this = guards_object(&mut e, &[1, 2, 3]);
        let compare = |e: &mut Engine, other: Ptr<ExtraGuardedRefData>| {
            e.call(0x0043_1030, &args![this, other]).bool()
        };
        let reordered = guards_object(&mut e, &[3, 1, 2]);
        assert!(!compare(&mut e, reordered));
        let shorter = guards_object(&mut e, &[1, 2]);
        assert!(compare(&mut e, shorter));
        let different = guards_object(&mut e, &[1, 2, 4]);
        assert!(compare(&mut e, different));
        let empty = guards_object(&mut e, &[]);
        let also_empty = guards_object(&mut e, &[]);
        assert!(!e.call(0x0043_1030, &args![empty, also_empty]).bool());

        let mut e = compare_engine(true, false);
        register_find(&mut e);
        let this = guards_object(&mut e, &[1]);
        let other = guards_object(&mut e, &[1]);
        start_log(&mut e);
        assert!(e.call(0x0043_1030, &args![this, other]).bool());
        assert_eq!(
            calls(&e, BS_EXTRA_DATA_COMPARE),
            vec![vec![this.addr(), other.addr()]]
        );
        assert!(calls(&e, SIMPLE_ARRAY_SIZE).is_empty());
    }

    #[test]
    fn add_guard_appends_a_new_value_once() {
        let mut e = extra_engine();
        register_find(&mut e);
        e.register(REFR_GUARD_VALUE, |_, a| ret(a[0] + 0x100));
        let this = guards_object(&mut e, &[0x1111]);
        let reference = Ptr::<()>::new(0x2000);
        start_log(&mut e);
        e.call(0x0043_10d0, &args![this, reference]);
        assert_eq!(array_items(&e, this.addr() + 0xc), vec![0x1111, 0x2100]);
        assert_eq!(calls(&e, REFR_GUARD_VALUE), vec![vec![0x2000]]);
        assert_eq!(calls(&e, SIMPLE_ARRAY_ADD).len(), 1);
        // Already present: found, nothing appended.
        start_log(&mut e);
        e.call(0x0043_10d0, &args![this, reference]);
        assert_eq!(array_items(&e, this.addr() + 0xc), vec![0x1111, 0x2100]);
        assert!(calls(&e, SIMPLE_ARRAY_ADD).is_empty());
    }

    #[test]
    fn guard_notification_reaches_the_process_of_each_acting_form() {
        let mut e = extra_engine();
        // Ids 10, 30, 40 and 50 name forms (kept in a table the double
        // reads); 20 names none.
        e.map(0x011d_e100, 8);
        e.register(LOOKUP_FORM_BY_ID, |e, a| {
            let forms = e.mem.u32(0x011d_e100);
            let slot = match a[0] {
                10 => 0,
                30 => 2,
                40 => 3,
                50 => 4,
                _ => return ret(0),
            };
            ret(e.mem.u32(forms + 4 * slot))
        });
        // The two vtable tests read a byte each from the form; the process
        // lookup reads a pointer.
        e.register(0x00a0_00f0, |e, a| ret(e.mem.u8(a[0] + 0x30) as u32));
        e.register(0x00a0_0100, |e, a| ret(e.mem.u8(a[0] + 0x31) as u32));
        e.register(FORM_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        e.register(0x00a0_060c, |_, _| Ret::default());
        let process = object_with_vtable(&mut e, &[(0x60c, 0x00a0_060c)]);
        let mut forms = vec![];
        for (first, second, has_process) in [
            (1u8, 1u8, true),
            (0, 0, false),
            (0, 1, true),
            (1, 0, true),
            (1, 1, false),
        ] {
            let form = object_with_vtable(&mut e, &[(0xf0, 0x00a0_00f0), (0x100, 0x00a0_0100)]);
            e.mem.set_u8(form + 0x30, first);
            e.mem.set_u8(form + 0x31, second);
            if has_process {
                e.mem.set_u32(form + 0x34, process);
            }
            forms.push(form);
        }
        let table = e.mem.alloc(0x20);
        for (i, form) in forms.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *form);
        }
        e.mem.set_u32(0x011d_e100, table);
        let this = guards_object(&mut e, &[10, 20, 30, 40, 50]);
        start_log(&mut e);
        e.call(0x0043_1120, &args![this, 0xaau32, 0xbbu32]);
        // Only form 10 passes everything; the arguments are the form, then
        // the second and the first word.
        assert_eq!(
            calls(&e, 0x00a0_060c),
            vec![vec![process, forms[0], 0xbb, 0xaa]]
        );
        assert_eq!(calls(&e, LOOKUP_FORM_BY_ID).len(), 5);
    }

    #[test]
    fn type_2e_constructor_clears_its_words() {
        let mut e = extra_engine();
        let this: Ptr<ExtraType2e> = e.new_object();
        e.set(this, ExtraType2e::unnamed0C, 5);
        e.set(this, ExtraType2e::unnamed10, 6);
        let back = e.call(0x0043_11f0, &args![this]).ptr::<ExtraType2e>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x2e);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_TYPE_2E_VTABLE);
        assert_eq!(e.get(this, ExtraType2e::unnamed0C), 0);
        assert_eq!(e.get(this, ExtraType2e::unnamed10), 0);
    }

    #[test]
    fn teleport_constructor_takes_the_data() {
        let mut e = extra_engine();
        let this: Ptr<ExtraTeleport> = e.new_object();
        let back = e
            .call(0x0043_1230, &args![this, Ptr::<()>::new(0x7777)])
            .ptr::<ExtraTeleport>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x2b);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_TELEPORT_VTABLE);
        assert_eq!(e.get(this, ExtraTeleport::pData).addr(), 0x7777);
    }

    #[test]
    fn teleport_destructor_deletes_the_data_only_when_there_is_some() {
        let mut e = extra_engine();
        e.register(TELEPORT_DATA_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraTeleport> = e.new_object();
        e.set(this, ExtraTeleport::pData, Ptr::new(0x7777));
        start_log(&mut e);
        e.call(0x0043_1260, &args![this]);
        assert_eq!(calls(&e, TELEPORT_DATA_DELETE), vec![vec![0x7777, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_TELEPORT_VTABLE);
        let bare: Ptr<ExtraTeleport> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_1260, &args![bare]);
        assert!(calls(&e, TELEPORT_DATA_DELETE).is_empty());
    }

    // ---- the simple extra-data classes (`004312f0` to `00431f60`) ----

    /// A constructor taking one word that is stored at +0xc: the object is
    /// returned, has the type byte and vtable, and holds the word; the other
    /// words of the object (here 0xaa-filled) are left alone.
    fn check_word_constructor(addr: u32, extra_type: u8, vtable: u32, size: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(size);
        for offset in (0x0c..size).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        assert_eq!(
            e.call(addr, &args![Ptr::<()>::new(this), 0x1234_5678u32])
                .u32(),
            this
        );
        assert_eq!(calls(&e, BS_EXTRA_DATA_CONSTRUCT).len(), 1);
        assert_eq!(extra_type_of(&e, this), extra_type);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234_5678);
        for offset in (0x10..size).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0xaaaa_aaaa);
        }
    }

    fn extra_type_of(e: &Engine, extra: u32) -> u8 {
        extra_type(e, extra)
    }

    /// A scalar deleting destructor: runs the destructor body (the base
    /// destructor shows it), frees only when bit 0 of the flags is set.
    fn check_scalar_deleting_destructor(addr: u32, vtable: u32, size: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(size);
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![Ptr::<()>::new(this), 2u32]).u32(), this);
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.call(addr, &args![Ptr::<()>::new(this), 3u32]).u32(), this);
        assert_eq!(e.mem.block_size(this), None);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT).len(), 2);
    }

    /// The destructor body of a class owning one object at +0xc: the owned
    /// object's deleter runs with flag 1 only when the pointer is set.
    fn check_owner_destructor(addr: u32, vtable: u32, deleter: u32) {
        let mut e = extra_engine();
        e.register(deleter, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x7777);
        start_log(&mut e);
        e.call(addr, &args![Ptr::<()>::new(this)]);
        assert_eq!(calls(&e, deleter), vec![vec![0x7777, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(vtable_of(&e, this), vtable);
        let bare = e.mem.alloc(0x10);
        start_log(&mut e);
        e.call(addr, &args![Ptr::<()>::new(bare)]);
        assert!(calls(&e, deleter).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![bare]]);
        assert_eq!(vtable_of(&e, bare), vtable);
    }

    /// The two exits every `Compare` starts with: `other` is not of the class
    /// (true, and nothing else is called), and the base says they differ
    /// (true, after the base was asked about `this` and `other`).
    fn check_compare_start(addr: u32, target_type: u32, size: u32) {
        let mut e = compare_engine(false, false);
        let this = e.mem.alloc(size);
        let other = e.mem.alloc(size);
        start_log(&mut e);
        assert!(e
            .call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
            .bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
        );
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());

        let mut e = compare_engine(true, true);
        let this = e.mem.alloc(size);
        let other = e.mem.alloc(size);
        start_log(&mut e);
        assert!(e
            .call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
            .bool());
        assert_eq!(calls(&e, BS_EXTRA_DATA_COMPARE), vec![vec![this, other]]);
    }

    /// A `Compare` that ends with the value at +0xc of `width` bytes (1, 2 or
    /// 4) and nothing else: equal values give false, different ones true.
    /// Bytes above the value are filled differently in the two objects and
    /// must not matter.
    fn check_value_compare(addr: u32, target_type: u32, width: u32, equal: u32, different: u32) {
        check_compare_start(addr, target_type, 0x10);
        let mut e = compare_engine(true, false);
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let set = |e: &mut Engine, object: u32, value: u32, filler: u8| {
            for offset in 0..4 {
                e.mem.set_u8(object + 0xc + offset, filler);
            }
            match width {
                1 => e.mem.set_u8(object + 0xc, value as u8),
                2 => e.mem.set_u16(object + 0xc, value as u16),
                _ => e.mem.set_u32(object + 0xc, value),
            }
        };
        set(&mut e, this, equal, 0x11);
        set(&mut e, other, equal, 0x22);
        let compare = |e: &mut Engine| {
            e.call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
                .bool()
        };
        assert!(!compare(&mut e));
        set(&mut e, other, different, 0x22);
        assert!(compare(&mut e));
    }

    #[test]
    fn teleport_compare_asks_the_teleport_data() {
        check_compare_start(0x0043_12f0, EXTRA_TELEPORT_TYPE, 0x10);
        for answer in [false, true] {
            let mut e = compare_engine(true, false);
            e.register_double(DOOR_TELEPORT_DATA_COMPARE, move |_, _| ret(answer as u32));
            let this: Ptr<ExtraTeleport> = e.new_object();
            let other: Ptr<ExtraTeleport> = e.new_object();
            e.set(this, ExtraTeleport::pData, Ptr::new(0x1111));
            e.set(other, ExtraTeleport::pData, Ptr::new(0x2222));
            start_log(&mut e);
            assert_eq!(e.call(0x0043_12f0, &args![this, other]).bool(), answer);
            assert_eq!(
                calls(&e, DOOR_TELEPORT_DATA_COMPARE),
                vec![vec![0x1111, 0x2222]]
            );
        }
    }

    /// The `Compare` of a class whose own data are compared by a callee given
    /// the two data pointers at +0xc.
    fn check_data_compare(addr: u32, target_type: u32, callee: u32) {
        check_compare_start(addr, target_type, 0x10);
        for answer in [false, true] {
            let mut e = compare_engine(true, false);
            e.register_double(callee, move |_, _| ret(answer as u32));
            let this = e.mem.alloc(0x10);
            let other = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 0x1111);
            e.mem.set_u32(other + 0xc, 0x2222);
            start_log(&mut e);
            let result = e
                .call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
                .bool();
            assert_eq!(result, answer);
            assert_eq!(calls(&e, callee), vec![vec![0x1111, 0x2222]]);
        }
    }

    #[test]
    fn map_marker_constructor_takes_the_data() {
        check_word_constructor(0x0043_1360, 0x2c, EXTRA_MAP_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn map_marker_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_1390, EXTRA_MAP_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn map_marker_destructor_deletes_the_map_data_when_there_is_some() {
        check_owner_destructor(0x0043_13c0, EXTRA_MAP_MARKER_VTABLE, MAP_MARKER_DATA_DELETE);
    }

    #[test]
    fn map_marker_compare_asks_the_map_data() {
        check_data_compare(0x0043_1450, EXTRA_MAP_MARKER_TYPE, MAP_MARKER_DATA_COMPARE);
    }

    #[test]
    fn audio_marker_constructor_takes_the_data() {
        check_word_constructor(0x0043_14c0, 0x90, EXTRA_AUDIO_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn audio_marker_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_14f0, EXTRA_AUDIO_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn audio_marker_destructor_deletes_the_audio_data_when_there_is_some() {
        check_owner_destructor(
            0x0043_1520,
            EXTRA_AUDIO_MARKER_VTABLE,
            AUDIO_MARKER_DATA_DELETE,
        );
    }

    #[test]
    fn audio_marker_compare_asks_the_audio_data() {
        check_data_compare(
            0x0043_15b0,
            EXTRA_AUDIO_MARKER_TYPE,
            AUDIO_MARKER_DATA_COMPARE,
        );
    }

    #[test]
    fn audio_buoy_marker_constructor_takes_the_data() {
        check_word_constructor(0x0043_1620, 0x91, EXTRA_AUDIO_BUOY_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn audio_buoy_marker_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_1650, EXTRA_AUDIO_BUOY_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn audio_buoy_marker_destructor_deletes_the_buoy_data_when_there_is_some() {
        check_owner_destructor(
            0x0043_1680,
            EXTRA_AUDIO_BUOY_MARKER_VTABLE,
            AUDIO_BUOY_DATA_DELETE,
        );
    }

    #[test]
    fn audio_buoy_marker_compare_asks_the_folded_body() {
        check_data_compare(
            0x0043_1710,
            EXTRA_AUDIO_BUOY_MARKER_TYPE,
            AUDIO_BUOY_DATA_COMPARE,
        );
    }

    #[test]
    fn action_constructor_sets_the_default_action() {
        let mut e = extra_engine();
        let this: Ptr<ExtraAction> = e.new_object();
        e.mem.set_u32(this.addr() + 0xc, 0xaaaa_aaaa);
        e.mem.set_u32(this.addr() + 0x10, 0xbbbb_bbbb);
        let back = e.call(0x0043_1780, &args![this]).ptr::<ExtraAction>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x0e);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_ACTION_VTABLE);
        // Only the byte is written.
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0xaaaa_aa01);
        assert_eq!(e.get(this, ExtraAction::eAction), 1);
        assert!(e.get(this, ExtraAction::pActionRef).is_null());
    }

    #[test]
    fn action_compare_checks_the_action_byte_only() {
        check_value_compare(0x0043_17c0, EXTRA_ACTION_TYPE, 1, 5, 6);
        // The reference at +0x10 is not compared.
        let mut e = compare_engine(true, false);
        let this: Ptr<ExtraAction> = e.new_object();
        let other: Ptr<ExtraAction> = e.new_object();
        e.set(this, ExtraAction::pActionRef, Ptr::new(0x10));
        e.set(other, ExtraAction::pActionRef, Ptr::new(0x20));
        assert!(!e.call(0x0043_17c0, &args![this, other]).bool());
    }

    #[test]
    fn container_changes_constructor_takes_the_changes() {
        check_word_constructor(0x0043_1830, 0x15, EXTRA_CONTAINER_CHANGES_VTABLE, 0x10);
    }

    #[test]
    fn container_changes_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_1860, EXTRA_CONTAINER_CHANGES_VTABLE, 0x10);
    }

    #[test]
    fn container_changes_destructor_destroys_the_changes_when_there_are_some() {
        let mut e = extra_engine();
        e.register(INVENTORY_CHANGES_DESTRUCT, |_, _| Ret::default());
        let changes = e.mem.alloc(0x20);
        let this: Ptr<ExtraContainerChanges> = e.new_object();
        e.set(this, ExtraContainerChanges::pChanges, Ptr::new(changes));
        start_log(&mut e);
        e.call(0x0043_1890, &args![this]);
        // The owned object's scalar deleting destructor: its body, then the
        // free.
        assert_eq!(calls(&e, INVENTORY_CHANGES_DESTRUCT), vec![vec![changes]]);
        assert_eq!(e.mem.block_size(changes), None);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_CONTAINER_CHANGES_VTABLE);

        let bare: Ptr<ExtraContainerChanges> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_1890, &args![bare]);
        assert!(calls(&e, INVENTORY_CHANGES_DESTRUCT).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT).len(), 1);
    }

    #[test]
    fn inventory_changes_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(INVENTORY_CHANGES_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_1920, &args![Ptr::<()>::new(this), 0u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(
            e.call(0x0043_1920, &args![Ptr::<()>::new(this), 1u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.block_size(this), None);
        assert_eq!(
            calls(&e, INVENTORY_CHANGES_DESTRUCT),
            vec![vec![this], vec![this]]
        );
    }

    #[test]
    fn original_reference_constructor_takes_the_reference() {
        check_word_constructor(0x0043_1950, 0x20, EXTRA_ORIGINAL_REFERENCE_VTABLE, 0x10);
    }

    #[test]
    fn original_reference_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_1980, EXTRA_ORIGINAL_REFERENCE_VTABLE, 0x10);
    }

    #[test]
    fn original_reference_destructor_resets_the_vtable() {
        let mut e = extra_engine();
        let this: Ptr<ExtraOriginalReference> = e.new_object();
        e.mem.set_u32(this.addr(), 0x1234);
        start_log(&mut e);
        e.call(0x0043_19b0, &args![this]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_ORIGINAL_REFERENCE_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn original_reference_compare_checks_the_reference() {
        check_value_compare(
            0x0043_19d0,
            EXTRA_ORIGINAL_REFERENCE_TYPE,
            4,
            0x4000,
            0x4004,
        );
    }

    #[test]
    fn ownership_constructor_takes_the_owner() {
        check_word_constructor(0x0043_1a40, 0x21, EXTRA_OWNERSHIP_VTABLE, 0x10);
    }

    #[test]
    fn ownership_compare_checks_the_owner() {
        check_value_compare(0x0043_1a70, EXTRA_OWNERSHIP_TYPE, 4, 0x4000, 0x4004);
    }

    #[test]
    fn global_constructor_takes_the_global() {
        check_word_constructor(0x0043_1ae0, 0x22, EXTRA_GLOBAL_VTABLE, 0x10);
    }

    #[test]
    fn global_compare_checks_the_global() {
        check_value_compare(0x0043_1b10, EXTRA_GLOBAL_TYPE, 4, 0x4000, 0x4004);
    }

    #[test]
    fn rank_constructor_takes_the_rank() {
        let mut e = extra_engine();
        let this: Ptr<ExtraRank> = e.new_object();
        let back = e.call(0x0043_1b80, &args![this, -3i32]).ptr::<ExtraRank>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x23);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_RANK_VTABLE);
        assert_eq!(e.get(this, ExtraRank::iRank), -3);
    }

    #[test]
    fn rank_compare_checks_the_rank() {
        check_value_compare(0x0043_1bb0, EXTRA_RANK_TYPE, 4, 2, 0xffff_ffff);
    }

    #[test]
    fn count_constructor_stores_only_the_16_bit_count() {
        let mut e = extra_engine();
        let this: Ptr<ExtraCount> = e.new_object();
        e.mem.set_u32(this.addr() + 0xc, 0xaaaa_aaaa);
        let back = e.call(0x0043_1c20, &args![this, -2i16]).ptr::<ExtraCount>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x24);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_COUNT_VTABLE);
        assert_eq!(e.get(this, ExtraCount::iCount), -2);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0xaaaa_fffe);
    }

    #[test]
    fn count_compare_checks_the_16_bit_count() {
        check_value_compare(0x0043_1c50, EXTRA_COUNT_TYPE, 2, 7, 0x8007);
    }

    #[test]
    fn leveled_item_compare_ignores_the_fields() {
        check_compare_start(0x0043_1cc0, EXTRA_LEVELED_ITEM_TYPE, 0x14);
        let mut e = compare_engine(true, false);
        let this: Ptr<ExtraLeveledItem> = e.new_object();
        let other: Ptr<ExtraLeveledItem> = e.new_object();
        e.set(this, ExtraLeveledItem::iIndex, 1);
        e.set(other, ExtraLeveledItem::iIndex, 2);
        e.set(this, ExtraLeveledItem::bdefault, true);
        assert!(!e.call(0x0043_1cc0, &args![this, other]).bool());
    }

    /// A `float` constructor.
    fn check_float_constructor(addr: u32, extra_type: u8, vtable: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        let back = e.call(addr, &args![Ptr::<()>::new(this), 12.5f32]).u32();
        assert_eq!(back, this);
        assert_eq!(extra_type_of(&e, this), extra_type);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.f32(this + 0xc), 12.5);
    }

    /// A `Compare` of one `float` at +0xc, with `==` semantics (NaN differs
    /// from itself, zero equals negative zero).
    fn check_float_compare(addr: u32, target_type: u32) {
        check_compare_start(addr, target_type, 0x10);
        let mut e = compare_engine(true, false);
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let compare = |e: &mut Engine, mine: f32, theirs: f32| {
            e.mem.set_f32(this + 0xc, mine);
            e.mem.set_f32(other + 0xc, theirs);
            e.call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
                .bool()
        };
        assert!(!compare(&mut e, 1.5, 1.5));
        assert!(compare(&mut e, 1.5, 2.5));
        assert!(!compare(&mut e, 0.0, -0.0));
        assert!(compare(&mut e, f32::NAN, f32::NAN));
        assert!(compare(&mut e, f32::NAN, 1.0));
    }

    #[test]
    fn health_constructor_takes_the_health() {
        check_float_constructor(0x0043_1d10, 0x25, EXTRA_HEALTH_VTABLE);
    }

    #[test]
    fn health_compare_checks_the_health_as_a_float() {
        check_float_compare(0x0043_1d40, EXTRA_HEALTH_TYPE);
    }

    #[test]
    fn health_perc_compare_checks_the_percentage_as_a_float() {
        check_float_compare(0x0043_1db0, EXTRA_HEALTH_PERC_TYPE);
    }

    #[test]
    fn uses_constructor_stores_only_the_byte() {
        let mut e = extra_engine();
        let this: Ptr<ExtraUses> = e.new_object();
        e.mem.set_u32(this.addr() + 0xc, 0xaaaa_aaaa);
        let back = e.call(0x0043_1e20, &args![this, 9u8]).ptr::<ExtraUses>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x26);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_USES_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0xaaaa_aa09);
    }

    #[test]
    fn uses_compare_checks_the_use_count_byte() {
        check_value_compare(0x0043_1e50, EXTRA_USES_TYPE, 1, 4, 5);
    }

    #[test]
    fn time_left_constructor_takes_the_time() {
        check_float_constructor(0x0043_1ec0, 0x27, EXTRA_TIME_LEFT_VTABLE);
    }

    #[test]
    fn time_left_compare_checks_the_time_as_a_float() {
        check_float_compare(0x0043_1ef0, EXTRA_TIME_LEFT_TYPE);
    }

    #[test]
    fn charge_constructor_takes_the_charge() {
        check_float_constructor(0x0043_1f60, 0x28, EXTRA_CHARGE_VTABLE);
    }

    #[test]
    fn charge_compare_checks_the_charge_as_a_float() {
        check_float_compare(0x0043_1f90, EXTRA_CHARGE_TYPE);
    }

    /// A constructor with one byte argument: only the low byte of the word
    /// is stored, the three bytes above it are left alone.
    fn check_byte_constructor(addr: u32, extra_type: u8, vtable: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xaaaa_aaaa);
        let back = e
            .call(addr, &args![Ptr::<()>::new(this), 0x1234_567bu32])
            .u32();
        assert_eq!(back, this);
        assert_eq!(extra_type_of(&e, this), extra_type);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.u32(this + 0xc), 0xaaaa_aa7b);
    }

    /// A constructor with no data of its own: the type, the vtable, and
    /// nothing above +0xc.
    fn check_base_only_constructor(addr: u32, extra_type: u8, vtable: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xaaaa_aaaa);
        start_log(&mut e);
        let back = e.call(addr, &args![Ptr::<()>::new(this)]).u32();
        assert_eq!(back, this);
        assert_eq!(calls(&e, BS_EXTRA_DATA_CONSTRUCT).len(), 1);
        assert_eq!(extra_type_of(&e, this), extra_type);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.u32(this + 0xc), 0xaaaa_aaaa);
    }

    #[test]
    fn script_constructor_stores_the_script_and_clears_the_variables() {
        let mut e = extra_engine();
        let this: Ptr<ExtraScript> = e.new_object();
        e.set(this, ExtraScript::pScriptVars, Ptr::new(0xdead));
        let back = e
            .call(0x0043_2000, &args![this, 0x6000u32])
            .ptr::<ExtraScript>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x0d);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_SCRIPT_VTABLE);
        assert_eq!(e.get(this, ExtraScript::pScript).addr(), 0x6000);
        assert!(e.get(this, ExtraScript::pScriptVars).is_null());
    }

    #[test]
    fn script_destructor_deletes_the_variables_only_when_there_are_some() {
        let mut e = extra_engine();
        e.register(SCRIPT_LOCALS_DELETE, |_, _| Ret::default());
        let this: Ptr<ExtraScript> = e.new_object();
        e.set(this, ExtraScript::pScript, Ptr::new(0x6000));
        e.set(this, ExtraScript::pScriptVars, Ptr::new(0x7000));
        start_log(&mut e);
        e.call(0x0043_2040, &args![this]);
        assert_eq!(calls(&e, SCRIPT_LOCALS_DELETE), vec![vec![0x7000, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_SCRIPT_VTABLE);
        assert!(e.get(this, ExtraScript::pScriptVars).is_null());
        // The script is not owned.
        assert_eq!(e.get(this, ExtraScript::pScript).addr(), 0x6000);

        let bare: Ptr<ExtraScript> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_2040, &args![bare]);
        assert!(calls(&e, SCRIPT_LOCALS_DELETE).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![bare.addr()]]);
    }

    #[test]
    fn script_compare_checks_the_script_pointer() {
        check_value_compare(0x0043_20d0, EXTRA_SCRIPT_TYPE, 4, 0x6000, 0x6004);
    }

    #[test]
    fn weapon_mod_flags_compare_checks_the_flag_byte() {
        check_value_compare(0x0043_2140, EXTRA_WEAPON_MOD_FLAGS_TYPE, 1, 3, 7);
    }

    #[test]
    fn modding_item_compare_checks_the_byte() {
        check_value_compare(0x0043_21b0, EXTRA_MODDING_ITEM_TYPE, 1, 1, 2);
    }

    #[test]
    fn scale_constructor_takes_the_scale() {
        check_float_constructor(0x0043_2220, 0x30, EXTRA_SCALE_VTABLE);
    }

    #[test]
    fn scale_compare_checks_the_scale_as_a_float() {
        check_float_compare(0x0043_2250, EXTRA_SCALE_TYPE);
    }

    #[test]
    fn ghost_constructor_builds_the_bare_base() {
        check_base_only_constructor(0x0043_22c0, 0x1f, EXTRA_GHOST_VTABLE);
    }

    #[test]
    fn worn_constructor_builds_the_bare_base() {
        check_base_only_constructor(0x0043_22f0, 0x16, EXTRA_WORN_VTABLE);
    }

    #[test]
    fn worn_left_constructor_builds_the_bare_base() {
        check_base_only_constructor(0x0043_2320, 0x17, EXTRA_WORN_LEFT_VTABLE);
    }

    #[test]
    fn cannot_wear_constructor_builds_the_bare_base() {
        check_base_only_constructor(0x0043_2350, 0x3e, EXTRA_CANNOT_WEAR_VTABLE);
    }

    #[test]
    fn hot_key_constructor_stores_only_the_byte() {
        check_byte_constructor(0x0043_2380, 0x4a, EXTRA_HOT_KEY_VTABLE);
    }

    /// An `ExtraInfoGeneralTopic` owning a topic whose `bGeneralTopic` flag
    /// (+0x24) is set.
    fn info_general_topic(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let topic = e.mem.alloc(0x2c);
        e.mem.set_u8(topic + 0x24, 1);
        e.mem.set_u32(this + 0xc, topic);
        (this, topic)
    }

    #[test]
    fn info_general_topic_destructor_clears_the_flag_then_deletes_the_topic() {
        let mut e = extra_engine();
        // The flag is already clear when the topic's destructor runs.
        e.register(MENU_TOPIC_DELETE, |e, a| {
            assert_eq!(e.mem.u8(a[0] + 0x24), 0);
            Ret::default()
        });
        let (this, topic) = info_general_topic(&mut e);
        start_log(&mut e);
        e.call(0x0043_23b0, &args![Ptr::<()>::new(this)]);
        assert_eq!(calls(&e, MENU_TOPIC_DELETE), vec![vec![topic, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(vtable_of(&e, this), EXTRA_INFO_GENERAL_TOPIC_VTABLE);
        assert_eq!(e.mem.u8(topic + 0x24), 0);
    }

    #[test]
    fn info_general_topic_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(MENU_TOPIC_DELETE, |_, _| Ret::default());
        let (this, _) = info_general_topic(&mut e);
        let back = e
            .call(0x0043_2440, &args![Ptr::<()>::new(this), 2u32])
            .u32();
        assert_eq!(back, this);
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_INFO_GENERAL_TOPIC_VTABLE);
        let back = e
            .call(0x0043_2440, &args![Ptr::<()>::new(this), 3u32])
            .u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn info_general_topic_constructor_takes_the_topic() {
        check_word_constructor(0x0043_2470, 0x4d, EXTRA_INFO_GENERAL_TOPIC_VTABLE, 0x10);
    }

    #[test]
    fn info_general_topic_default_constructor_builds_a_topic() {
        let mut e = extra_engine();
        e.register(MENU_TOPIC_CONSTRUCT, |_, a| ret(a[0]));
        let this: Ptr<ExtraInfoGeneralTopic> = e.new_object();
        start_log(&mut e);
        let back = e
            .call(0x0043_24a0, &args![this])
            .ptr::<ExtraInfoGeneralTopic>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x4d);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_INFO_GENERAL_TOPIC_VTABLE);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x2c]]);
        let topic = e.get(this, ExtraInfoGeneralTopic::pInfoGen);
        assert!(e.mem.block_size(topic.addr()).unwrap() >= 0x2c);
        assert_eq!(calls(&e, MENU_TOPIC_CONSTRUCT), vec![vec![topic.addr()]]);

        // A failed allocation leaves the topic null and constructs nothing.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let failed: Ptr<ExtraInfoGeneralTopic> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_24a0, &args![failed]);
        assert!(e.get(failed, ExtraInfoGeneralTopic::pInfoGen).is_null());
        assert!(calls(&e, MENU_TOPIC_CONSTRUCT).is_empty());
    }

    #[test]
    fn hot_key_compare_checks_the_key_byte() {
        check_value_compare(0x0043_2540, EXTRA_HOT_KEY_TYPE, 1, 0x31, 0xb1);
    }

    #[test]
    fn seed_constructor_stores_only_the_byte() {
        check_byte_constructor(0x0043_25b0, 0x31, EXTRA_SEED_VTABLE);
    }

    #[test]
    fn seed_compare_checks_the_seed_byte() {
        check_value_compare(0x0043_25e0, EXTRA_SEED_TYPE, 1, 9, 10);
    }

    #[test]
    fn package_start_location_default_constructor_constructs_the_location() {
        let mut e = extra_engine();
        e.register(WORLD_LOCATION_CONSTRUCT, |_, a| ret(a[0]));
        let this: Ptr<ExtraPackageStartLocation> = e.new_object();
        start_log(&mut e);
        let back = e
            .call(0x0043_2650, &args![this])
            .ptr::<ExtraPackageStartLocation>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x18);
        assert_eq!(
            vtable_of(&e, this.addr()),
            EXTRA_PACKAGE_START_LOCATION_VTABLE
        );
        assert_eq!(
            calls(&e, WORLD_LOCATION_CONSTRUCT),
            vec![vec![this.addr() + 0xc]]
        );
    }

    #[test]
    fn package_start_location_constructor_copies_the_place() {
        let mut e = extra_engine();
        e.register(WORLD_LOCATION_CONSTRUCT, |_, a| ret(a[0]));
        let position: Ptr<NiPoint3> = e.new_object();
        e.set(position, NiPoint3::x, 1.5);
        e.set(position, NiPoint3::y, -2.0);
        e.set(position, NiPoint3::z, 3.25);
        for (form, fallback, expected) in [(0x4000u32, 0x5000u32, 0x4000u32), (0, 0x5000, 0x5000)] {
            let this: Ptr<ExtraPackageStartLocation> = e.new_object();
            start_log(&mut e);
            let back = e
                .call(0x0043_26c0, &args![this, form, fallback, position, 0.75f32])
                .ptr::<ExtraPackageStartLocation>();
            assert_eq!(back, this);
            assert_eq!(extra_type(&e, this.addr()), 0x18);
            assert_eq!(
                calls(&e, WORLD_LOCATION_CONSTRUCT),
                vec![vec![this.addr() + 0xc]]
            );
            let location = this.at(ExtraPackageStartLocation::worldLoc);
            assert_eq!(
                e.get(location, WorldLocation::pLocationForm).addr(),
                expected
            );
            let point = location.at(WorldLocation::locPt);
            assert_eq!(e.get(point, NiPoint3::x), 1.5);
            assert_eq!(e.get(point, NiPoint3::y), -2.0);
            assert_eq!(e.get(point, NiPoint3::z), 3.25);
            assert_eq!(e.get(location, WorldLocation::fZRot), 0.75);
        }
    }

    #[test]
    fn package_start_location_compare_checks_type_base_and_the_location_bytes() {
        check_compare_start(0x0043_2770, EXTRA_PACKAGE_START_LOCATION_TYPE, 0x20);
        let mut e = compare_engine(true, false);
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        start_log(&mut e);
        let compare = |e: &mut Engine| {
            e.call(
                0x0043_2770,
                &args![Ptr::<()>::new(this), Ptr::<()>::new(other)],
            )
            .bool()
        };
        assert!(!compare(&mut e));
        assert_eq!(calls(&e, MEMCMP), vec![vec![this + 0xc, other + 0xc, 0x14]]);
        // The last byte of the location differs; the byte before the location
        // is not part of it.
        e.mem.set_u8(other + 0x1f, 1);
        assert!(compare(&mut e));
        e.mem.set_u8(other + 0x1f, 0);
        e.mem.set_u8(this + 0x0b, 1);
        assert!(!compare(&mut e));
    }

    #[test]
    fn reference_pointer_constructor_takes_the_reference() {
        check_word_constructor(0x0043_27e0, 0x1c, EXTRA_REFERENCE_POINTER_VTABLE, 0x10);
    }

    #[test]
    fn package_default_constructor_sets_the_defaults() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x20);
        for offset in (0x0c..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        let back = e.call(0x0043_2810, &args![Ptr::<()>::new(this)]).u32();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this), 0x19);
        assert_eq!(vtable_of(&e, this), EXTRA_PACKAGE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x0c), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        // The three flag bytes are cleared, the byte after them is not.
        assert_eq!(e.mem.u32(this + 0x18), 0xaa00_0000);
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_aaaa);
    }

    #[test]
    fn package_constructor_takes_the_six_fields() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x20);
        for offset in (0x0c..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        let back = e
            .call(
                0x0043_2870,
                &args![
                    Ptr::<()>::new(this),
                    0x4000u32,
                    7i32,
                    0x5000u32,
                    0x0101u32,
                    0x0200u32,
                    0x03u32
                ],
            )
            .u32();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this), 0x19);
        assert_eq!(vtable_of(&e, this), EXTRA_PACKAGE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x0c), 0x4000);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        assert_eq!(e.mem.u32(this + 0x14), 0x5000);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u8(this + 0x19), 0);
        assert_eq!(e.mem.u8(this + 0x1a), 3);
        assert_eq!(e.mem.u8(this + 0x1b), 0xaa);
    }

    #[test]
    fn tres_pass_package_constructor_takes_the_package() {
        check_word_constructor(0x0043_28d0, 0x1a, EXTRA_TRES_PASS_PACKAGE_VTABLE, 0x10);
    }

    /// An `ExtraTresPassPackage` owning a package whose virtual slot 0x10
    /// (the scalar deleting destructor) is a registered double.
    fn tres_pass_package(e: &mut Engine) -> (u32, u32) {
        e.register(PACKAGE_SET_IS_CREATED, |_, _| Ret::default());
        e.register(SAVE_LOAD_GAME_DELETE_FORM, |_, _| Ret::default());
        e.register(0x00a0_0010, |_, a| ret(a[0]));
        e.set_global(SAVE_LOAD_GAME, 0x5000u32);
        let package = object_with_vtable(e, &[(0x10, 0x00a0_0010)]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, package);
        (this, package)
    }

    #[test]
    fn tres_pass_package_destructor_destroys_the_package_through_its_vtable() {
        let mut e = extra_engine();
        let (this, package) = tres_pass_package(&mut e);
        e.register(SAVE_LOAD_GAME_ALWAYS_FALSE, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0043_2900, &args![Ptr::<()>::new(this)]);
        assert_eq!(calls(&e, PACKAGE_SET_IS_CREATED), vec![vec![package, 1]]);
        assert_eq!(calls(&e, SAVE_LOAD_GAME_ALWAYS_FALSE), vec![vec![0x5000]]);
        assert_eq!(calls(&e, 0x00a0_0010), vec![vec![package, 1]]);
        assert!(calls(&e, SAVE_LOAD_GAME_DELETE_FORM).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(vtable_of(&e, this), EXTRA_TRES_PASS_PACKAGE_VTABLE);
    }

    #[test]
    fn tres_pass_package_destructor_uses_delete_form_when_the_stub_says_so() {
        let mut e = extra_engine();
        let (this, package) = tres_pass_package(&mut e);
        e.register(SAVE_LOAD_GAME_ALWAYS_FALSE, |_, _| ret(1));
        start_log(&mut e);
        e.call(0x0043_2900, &args![Ptr::<()>::new(this)]);
        assert_eq!(
            calls(&e, SAVE_LOAD_GAME_DELETE_FORM),
            vec![vec![0x5000, package]]
        );
        assert!(calls(&e, 0x00a0_0010).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn tres_pass_package_destructor_does_nothing_more_without_a_package() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        start_log(&mut e);
        e.call(0x0043_2900, &args![Ptr::<()>::new(this)]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // The call itself and the base destructor, nothing else.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(vtable_of(&e, this), EXTRA_TRES_PASS_PACKAGE_VTABLE);
    }

    #[test]
    fn player_crime_list_default_constructor_allocates_the_list() {
        let mut e = extra_engine();
        e.register(ACTOR_LIST_CONSTRUCT, |_, a| ret(a[0]));
        let this: Ptr<ExtraPlayerCrimeList> = e.new_object();
        start_log(&mut e);
        let back = e
            .call(0x0043_29d0, &args![this])
            .ptr::<ExtraPlayerCrimeList>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x35);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_PLAYER_CRIME_LIST_VTABLE);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        let list = e.get(this, ExtraPlayerCrimeList::pCrime);
        assert_eq!(e.mem.block_size(list.addr()), Some(8));
        assert_eq!(calls(&e, ACTOR_LIST_CONSTRUCT), vec![vec![list.addr()]]);

        // A failed allocation leaves the list null and constructs nothing.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let failed: Ptr<ExtraPlayerCrimeList> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_29d0, &args![failed]);
        assert!(e.get(failed, ExtraPlayerCrimeList::pCrime).is_null());
        assert!(calls(&e, ACTOR_LIST_CONSTRUCT).is_empty());
    }

    #[test]
    fn player_crime_list_constructor_adds_the_crime_by_address() {
        let mut e = extra_engine();
        e.register(ACTOR_LIST_CONSTRUCT, |_, a| ret(a[0]));
        // The list gets the address of a variable holding the crime.
        e.register(CRIME_LIST_ADD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7777);
            Ret::default()
        });
        let this: Ptr<ExtraPlayerCrimeList> = e.new_object();
        start_log(&mut e);
        let back = e
            .call(0x0043_2aa0, &args![this, 0x7777u32])
            .ptr::<ExtraPlayerCrimeList>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x35);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_PLAYER_CRIME_LIST_VTABLE);
        let list = e.get(this, ExtraPlayerCrimeList::pCrime);
        let added = calls(&e, CRIME_LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], list.addr());
    }

    #[test]
    fn player_crime_list_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_2a70, EXTRA_PLAYER_CRIME_LIST_VTABLE, 0x10);
    }

    #[test]
    fn player_crime_list_destructor_deletes_the_list() {
        check_owner_destructor(
            0x0043_2b50,
            EXTRA_PLAYER_CRIME_LIST_VTABLE,
            ACTOR_LIST_DELETE,
        );
    }

    #[test]
    fn leveled_item_constructor_takes_the_index_and_sets_the_default_flag() {
        let mut e = extra_engine();
        let this: Ptr<ExtraLeveledItem> = e.new_object();
        let back = e
            .call(0x0043_2be0, &args![this, 5i32])
            .ptr::<ExtraLeveledItem>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x2f);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_LEVELED_ITEM_VTABLE);
        assert_eq!(e.get(this, ExtraLeveledItem::iIndex), 5);
        assert_eq!(e.mem.u8(this.addr() + 0x10), 1);
    }

    #[test]
    fn persistent_cell_constructor_takes_the_cell() {
        check_word_constructor(0x0043_2c20, 0x0c, EXTRA_PERSISTENT_CELL_VTABLE, 0x10);
        // A null cell stays null.
        let mut e = extra_engine();
        let this: Ptr<ExtraPersistentCell> = e.new_object();
        e.mem.set_u32(this.addr() + 0xc, 0xaaaa_aaaa);
        e.call(0x0043_2c20, &args![this, 0u32]);
        assert!(e.get(this, ExtraPersistentCell::pPersistentCell).is_null());
    }

    #[test]
    fn persistent_cell_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_2c60, EXTRA_PERSISTENT_CELL_VTABLE, 0x10);
    }

    #[test]
    fn persistent_cell_destructor_resets_the_vtable_and_owns_nothing() {
        let mut e = extra_engine();
        let this: Ptr<ExtraPersistentCell> = e.new_object();
        e.set(this, ExtraPersistentCell::pPersistentCell, Ptr::new(0x6000));
        e.mem.set_u32(this.addr(), 0x1234);
        start_log(&mut e);
        e.call(0x0043_2c90, &args![this]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_PERSISTENT_CELL_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        // The call itself and the base destructor, nothing else.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn rag_doll_data_constructor_clears_the_data() {
        let mut e = extra_engine();
        let this: Ptr<ExtraRagDollData> = e.new_object();
        e.set(this, ExtraRagDollData::pRagDollData, Ptr::new(0xdead));
        let back = e.call(0x0043_2cb0, &args![this]).ptr::<ExtraRagDollData>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x14);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_RAG_DOLL_DATA_VTABLE);
        assert!(e.get(this, ExtraRagDollData::pRagDollData).is_null());
    }

    #[test]
    fn rag_doll_data_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting_destructor(0x0043_2ce0, EXTRA_RAG_DOLL_DATA_VTABLE, 0x10);
    }

    #[test]
    fn rag_doll_data_destructor_deletes_the_owned_data_only_when_there_is_some() {
        let mut e = extra_engine();
        e.register(RAG_DOLL_DATA_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, data);
        start_log(&mut e);
        e.call(0x0043_2d10, &args![Ptr::<()>::new(this)]);
        // The data's scalar deleting destructor ran with flag 1: destructor
        // body, then the memory is freed.
        assert_eq!(calls(&e, RAG_DOLL_DATA_DESTRUCT), vec![vec![data]]);
        assert_eq!(e.mem.block_size(data), None);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(vtable_of(&e, this), EXTRA_RAG_DOLL_DATA_VTABLE);

        let bare = e.mem.alloc(0x10);
        start_log(&mut e);
        e.call(0x0043_2d10, &args![Ptr::<()>::new(bare)]);
        assert!(calls(&e, RAG_DOLL_DATA_DESTRUCT).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![bare]]);
    }

    #[test]
    fn rag_doll_data_object_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        e.register(RAG_DOLL_DATA_DESTRUCT, |_, _| Ret::default());
        let data = e.mem.alloc(0x40);
        start_log(&mut e);
        let back = e
            .call(0x0043_2da0, &args![Ptr::<()>::new(data), 0u32])
            .u32();
        assert_eq!(back, data);
        assert!(e.mem.block_size(data).is_some());
        let back = e
            .call(0x0043_2da0, &args![Ptr::<()>::new(data), 1u32])
            .u32();
        assert_eq!(back, data);
        assert_eq!(e.mem.block_size(data), None);
        assert_eq!(calls(&e, RAG_DOLL_DATA_DESTRUCT).len(), 2);
    }

    // ---- the third batch (`00432dd0` to `00433d40`) ----

    /// A constructor with no argument that stores the type, the vtable and
    /// zero at +0xc (the words above are left alone).
    fn check_cleared_constructor(addr: u32, extra_type: u8, vtable: u32, size: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(size);
        for offset in (0x0c..size).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![Ptr::<()>::new(this)]).u32(), this);
        assert_eq!(calls(&e, BS_EXTRA_DATA_CONSTRUCT).len(), 1);
        assert_eq!(extra_type_of(&e, this), extra_type);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        for offset in (0x10..size).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0xaaaa_aaaa);
        }
    }

    /// A `Compare` that only casts `other` (it does not ask the base): a
    /// failed cast gives true, then the word at +0xc decides.
    fn check_cast_only_compare(addr: u32, target_type: u32) {
        let mut e = compare_engine(false, false);
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        start_log(&mut e);
        assert!(e
            .call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
            .bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
        );

        // The base would say "differ", but it is not asked.
        let mut e = compare_engine(true, true);
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0x1111);
        e.mem.set_u32(other + 0xc, 0x1111);
        start_log(&mut e);
        assert!(!e
            .call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
            .bool());
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
        e.mem.set_u32(other + 0xc, 0x2222);
        assert!(e
            .call(addr, &args![Ptr::<()>::new(this), Ptr::<()>::new(other)])
            .bool());
    }

    /// Doubles for the `BSSimpleList` functions the classes of this batch
    /// call, on top of the `Compare` doubles. A list is a head node (item,
    /// next); an empty list has item 0 and no next node.
    fn list_engine() -> Engine {
        let mut e = compare_engine(true, false);
        e.register(SIMPLE_LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        e.register(SIMPLE_LIST_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(SIMPLE_LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SIMPLE_LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(SIMPLE_LIST_CLEAR, |e, a| {
            if a[0] != 0 {
                clear_list(e, a[0]);
            }
            Ret::default()
        });
        e.register(SIMPLE_LIST_DESTRUCT, |e, a| {
            clear_list(e, a[0]);
            Ret::default()
        });
        e.register(SIMPLE_LIST_DELETE, |e, a| {
            clear_list(e, a[0]);
            if a[1] & 1 != 0 {
                e.mem.free(a[0]);
            }
            ret(a[0])
        });
        e.register(SIMPLE_LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
                e.mem.free(next);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        // `BSSimpleList::AddHead` of the item held at the address given: the
        // old head item moves into a new second node.
        e.register(SIMPLE_LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            if item != 0 {
                if e.mem.u32(a[0]) == 0 {
                    e.mem.set_u32(a[0], item);
                } else {
                    let node = e.mem.alloc(8);
                    let old = e.mem.u32(a[0]);
                    let next = e.mem.u32(a[0] + 4);
                    e.mem.set_u32(node, old);
                    e.mem.set_u32(node + 4, next);
                    e.mem.set_u32(a[0] + 4, node);
                    e.mem.set_u32(a[0], item);
                }
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            for (i, byte) in bytes.iter().enumerate() {
                e.mem.set_u8(a[0] + i as u32, *byte);
            }
            ret(a[0])
        });
        e
    }

    /// Frees every node after the head and clears the head.
    fn clear_list(e: &mut Engine, head: u32) {
        let mut node = e.mem.u32(head + 4);
        while node != 0 {
            let next = e.mem.u32(node + 4);
            e.mem.free(node);
            node = next;
        }
        e.mem.set_u32(head, 0);
        e.mem.set_u32(head + 4, 0);
    }

    /// Builds a list in place at `head` (an 8-byte block) from `items`; the
    /// later nodes are allocated.
    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        e.mem.set_u32(head, items.first().copied().unwrap_or(0));
        e.mem.set_u32(head + 4, 0);
        let mut last = head;
        for item in items.iter().skip(1) {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(last + 4, node);
            last = node;
        }
    }

    /// The items of the list at `head`, node by node.
    fn list_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = vec![];
        let mut node = head;
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn rag_doll_data_compare_asks_the_data_only_when_both_have_some() {
        check_compare_start(0x0043_2dd0, EXTRA_RAG_DOLL_DATA_TYPE, 0x10);
        for answer in [false, true] {
            let mut e = compare_engine(true, false);
            e.register_double(RAG_DOLL_DATA_COMPARE, move |_, _| ret(answer as u32));
            let this: Ptr<ExtraRagDollData> = e.new_object();
            let other: Ptr<ExtraRagDollData> = e.new_object();
            e.set(this, ExtraRagDollData::pRagDollData, Ptr::new(0x1111));
            e.set(other, ExtraRagDollData::pRagDollData, Ptr::new(0x2222));
            start_log(&mut e);
            assert_eq!(e.call(0x0043_2dd0, &args![this, other]).bool(), answer);
            // `this` is the first data, the other's data the argument.
            assert_eq!(calls(&e, RAG_DOLL_DATA_COMPARE), vec![vec![0x1111, 0x2222]]);
        }
        // Without data of its own: equal only to another without data.
        let mut e = compare_engine(true, false);
        e.register(RAG_DOLL_DATA_COMPARE, |_, _| panic!("no data to compare"));
        let this: Ptr<ExtraRagDollData> = e.new_object();
        let other: Ptr<ExtraRagDollData> = e.new_object();
        assert!(!e.call(0x0043_2dd0, &args![this, other]).bool());
        e.set(other, ExtraRagDollData::pRagDollData, Ptr::new(0x2222));
        assert!(e.call(0x0043_2dd0, &args![this, other]).bool());
        // Data on `this` but none on the other: the data's compare is asked
        // with a null argument.
        let mut e = compare_engine(true, false);
        e.register_double(RAG_DOLL_DATA_COMPARE, |_, a| ret((a[1] == 0) as u32));
        let this: Ptr<ExtraRagDollData> = e.new_object();
        let other: Ptr<ExtraRagDollData> = e.new_object();
        e.set(this, ExtraRagDollData::pRagDollData, Ptr::new(0x1111));
        assert!(e.call(0x0043_2dd0, &args![this, other]).bool());
    }

    #[test]
    fn encounter_zone_constructor_clears_the_zone() {
        check_cleared_constructor(0x0043_2e60, 0x74, EXTRA_ENCOUNTER_ZONE_VTABLE, 0x10);
    }

    #[test]
    fn encounter_zone_compare_checks_type_base_and_zone() {
        check_value_compare(0x0043_2e90, EXTRA_ENCOUNTER_ZONE_TYPE, 4, 0x6000, 0x7000);
    }

    #[test]
    fn used_markers_constructor_clears_the_markers() {
        check_cleared_constructor(0x0043_2f00, 0x12, EXTRA_USED_MARKERS_VTABLE, 0x10);
    }

    #[test]
    fn fn_00432f30_runs_only_the_base_destructor() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x1234);
        start_log(&mut e);
        e.call(0x0043_2f30, &args![Ptr::<()>::new(this)]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // Only the call itself and the base destructor; the vtable stays.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(this), 0x1234);
    }

    #[test]
    fn reserved_markers_constructor_clears_the_markers() {
        check_cleared_constructor(0x0043_2f50, 0x82, EXTRA_RESERVED_MARKERS_VTABLE, 0x10);
    }

    #[test]
    fn used_markers_get_marker_used_tests_a_bit_below_30() {
        let mut e = extra_engine();
        let this: Ptr<ExtraUsedMarkers> = e.new_object();
        e.set(this, ExtraUsedMarkers::iUsedMarkers, 0xc000_0005);
        let get = |e: &mut Engine, index: u32| e.call(0x0043_2f80, &args![this, index]).bool();
        assert!(get(&mut e, 0));
        assert!(!get(&mut e, 1));
        assert!(get(&mut e, 2));
        assert!(!get(&mut e, 29));
        // Bits 30 and 31 are set but never reported.
        assert!(!get(&mut e, 30));
        assert!(!get(&mut e, 31));
        assert!(!get(&mut e, 0x20));
        e.set(this, ExtraUsedMarkers::iUsedMarkers, 0x2000_0000);
        assert!(get(&mut e, 29));
    }

    #[test]
    fn used_markers_set_marker_used_sets_or_clears_a_bit_below_30() {
        let mut e = extra_engine();
        let this: Ptr<ExtraUsedMarkers> = e.new_object();
        e.call(0x0043_2fc0, &args![this, 3u32, 1u32]);
        assert_eq!(e.get(this, ExtraUsedMarkers::iUsedMarkers), 8);
        e.call(0x0043_2fc0, &args![this, 29u32, 0x100u32 | 5]);
        assert_eq!(e.get(this, ExtraUsedMarkers::iUsedMarkers), 0x2000_0008);
        // Only the low byte of the flag counts: 0x100 clears, 0x1ff sets.
        e.call(0x0043_2fc0, &args![this, 3u32, 0x100u32]);
        assert_eq!(e.get(this, ExtraUsedMarkers::iUsedMarkers), 0x2000_0000);
        e.call(0x0043_2fc0, &args![this, 3u32, 0x1ffu32]);
        assert_eq!(e.get(this, ExtraUsedMarkers::iUsedMarkers), 0x2000_0008);
        e.call(0x0043_2fc0, &args![this, 3u32, 0u32]);
        assert_eq!(e.get(this, ExtraUsedMarkers::iUsedMarkers), 0x2000_0000);
        // Out of range: nothing changes.
        e.call(0x0043_2fc0, &args![this, 30u32, 1u32]);
        e.call(0x0043_2fc0, &args![this, 31u32, 1u32]);
        e.call(0x0043_2fc0, &args![this, 0xffu32, 0u32]);
        assert_eq!(e.get(this, ExtraUsedMarkers::iUsedMarkers), 0x2000_0000);
    }

    #[test]
    fn run_once_packs_constructor_builds_the_list() {
        let mut e = list_engine();
        let this: Ptr<ExtraRunOncePacks> = e.new_object();
        start_log(&mut e);
        let back = e.call(0x0043_3010, &args![this]).ptr::<ExtraRunOncePacks>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x1b);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_RUN_ONCE_PACKS_VTABLE);
        let list = e.get(this, ExtraRunOncePacks::pPackageList);
        assert!(!list.is_null());
        assert_eq!(e.mem.block_size(list.addr()), Some(8));
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![list.addr()]]);

        // A failed allocation leaves no list and constructs nothing.
        let mut e = list_engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let failed: Ptr<ExtraRunOncePacks> = e.new_object();
        e.set(failed, ExtraRunOncePacks::pPackageList, Ptr::new(0xdead));
        start_log(&mut e);
        e.call(0x0043_3010, &args![failed]);
        assert!(e.get(failed, ExtraRunOncePacks::pPackageList).is_null());
        assert!(calls(&e, SIMPLE_LIST_CONSTRUCT).is_empty());
    }

    #[test]
    fn run_once_packs_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x0043_30b0, &args![Ptr::<()>::new(this), 0u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_RUN_ONCE_PACKS_VTABLE);
        e.call(0x0043_30b0, &args![Ptr::<()>::new(this), 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn run_once_packs_destructor_deletes_the_items_up_to_the_first_empty_one() {
        let mut e = list_engine();
        let this: Ptr<ExtraRunOncePacks> = e.new_object();
        let list = e.mem.alloc(8);
        let (first, second, after_gap) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8));
        fill_list(&mut e, list, &[first, second, 0, after_gap]);
        e.set(this, ExtraRunOncePacks::pPackageList, Ptr::new(list));
        start_log(&mut e);
        e.call(0x0043_30e0, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![first], vec![second]]);
        assert!(e.mem.block_size(after_gap).is_some());
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![list]]);
        assert_eq!(calls(&e, SIMPLE_LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(e.mem.block_size(list), None);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_RUN_ONCE_PACKS_VTABLE);
    }

    #[test]
    fn run_once_packs_destructor_without_a_list_still_clears_the_null_list() {
        let mut e = list_engine();
        let this: Ptr<ExtraRunOncePacks> = e.new_object();
        start_log(&mut e);
        e.call(0x0043_30e0, &args![this]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![0]]);
        assert!(calls(&e, SIMPLE_LIST_DELETE).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn run_once_packs_entry_is_updated_in_place_or_added() {
        let mut e = list_engine();
        let this: Ptr<ExtraRunOncePacks> = e.new_object();
        let list = e.mem.alloc(8);
        let (first, second) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, 0x10);
        e.mem.set_u32(second, 0x20);
        fill_list(&mut e, list, &[first, second]);
        e.set(this, ExtraRunOncePacks::pPackageList, Ptr::new(list));
        start_log(&mut e);
        // An existing entry gets its flag and nothing is added.
        e.call(0x0043_31c0, &args![this, 0x20u32, 0x101u32]);
        assert_eq!(e.mem.u32(second), 0x20);
        assert_eq!(e.mem.u8(second + 4), 1);
        assert_eq!(e.mem.u8(first + 4), 0);
        assert!(calls(&e, SIMPLE_LIST_ADD_HEAD).is_empty());
        e.call(0x0043_31c0, &args![this, 0x20u32, 0u32]);
        assert_eq!(e.mem.u8(second + 4), 0);

        // A missing one is allocated (8 bytes), added with the address of a
        // variable holding it, and filled in.
        e.call(0x0043_31c0, &args![this, 0x30u32, 7u32]);
        let added = calls(&e, SIMPLE_LIST_ADD_HEAD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], list);
        let items = list_items(&e, list);
        assert_eq!(items.len(), 3);
        let entry = items[0];
        assert_eq!(e.mem.block_size(entry), Some(8));
        assert_eq!(e.mem.u32(entry), 0x30);
        assert_eq!(e.mem.u8(entry + 4), 7);
        assert_eq!(items[1..], [first, second]);
    }

    #[test]
    fn run_once_packs_entry_search_ends_at_a_node_without_an_item() {
        let mut e = list_engine();
        let this: Ptr<ExtraRunOncePacks> = e.new_object();
        let list = e.mem.alloc(8);
        let (first, hidden) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, 0x10);
        e.mem.set_u32(hidden, 0x30);
        fill_list(&mut e, list, &[first, 0, hidden]);
        e.set(this, ExtraRunOncePacks::pPackageList, Ptr::new(list));
        start_log(&mut e);
        // The entry behind the gap is not seen; a new one is added.
        e.call(0x0043_31c0, &args![this, 0x30u32, 1u32]);
        assert_eq!(calls(&e, SIMPLE_LIST_ADD_HEAD).len(), 1);
        assert_eq!(e.mem.u8(hidden + 4), 0);
    }

    #[test]
    fn distant_data_constructor_builds_the_land_normal() {
        let mut e = list_engine();
        e.map(0x0101_4000, 0x1000);
        e.set_global(DISTANT_DATA_NORMAL_Z, 0.9706f32);
        e.register(NI_POINT3_CONSTRUCT, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            ret(a[0])
        });
        let this = e.mem.alloc(0x18);
        for offset in (0x0c..0x18).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        let back = e.call(0x0043_3260, &args![Ptr::<()>::new(this)]).u32();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this), 0x13);
        assert_eq!(vtable_of(&e, this), EXTRA_DISTANT_DATA_VTABLE);
        assert_eq!(e.mem.f32(this + 0xc), 0.0);
        assert_eq!(e.mem.f32(this + 0x10), 0.0);
        assert_eq!(e.mem.f32(this + 0x14), 0.9706f32);
        // The point is built in a temporary with (0, 0, z).
        let made = calls(&e, NI_POINT3_CONSTRUCT);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1..], [0, 0, 0.9706f32.to_bits()]);
        assert_ne!(made[0][0], this + 0xc);
    }

    #[test]
    fn enable_state_parent_constructor_clears_the_parent_and_flags() {
        let mut e = extra_engine();
        let this: Ptr<ExtraEnableStateParent> = e.new_object();
        e.mem.set_u32(this.addr() + 0xc, 0xaaaa_aaaa);
        e.mem.set_u32(this.addr() + 0x10, 0xaaaa_aaaa);
        let back = e
            .call(0x0043_3300, &args![this])
            .ptr::<ExtraEnableStateParent>();
        assert_eq!(back, this);
        assert_eq!(extra_type(&e, this.addr()), 0x37);
        assert_eq!(vtable_of(&e, this.addr()), EXTRA_ENABLE_STATE_PARENT_VTABLE);
        assert!(e.get(this, ExtraEnableStateParent::pParent).is_null());
        // Only the flag byte is written.
        assert_eq!(e.mem.u32(this.addr() + 0x10), 0xaaaa_aa00);
    }

    #[test]
    fn enable_state_parent_compare_checks_type_parent_and_flags() {
        let mut e = compare_engine(false, false);
        let this: Ptr<ExtraEnableStateParent> = e.new_object();
        let other: Ptr<ExtraEnableStateParent> = e.new_object();
        start_log(&mut e);
        assert!(e.call(0x0043_3340, &args![this, other]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other.addr(),
                0,
                BS_EXTRA_DATA_TYPE,
                EXTRA_ENABLE_STATE_PARENT_TYPE,
                0
            ]]
        );

        // The base would say "differ", but it is not asked.
        let mut e = compare_engine(true, true);
        let this: Ptr<ExtraEnableStateParent> = e.new_object();
        let other: Ptr<ExtraEnableStateParent> = e.new_object();
        e.set(this, ExtraEnableStateParent::pParent, Ptr::new(0x6000));
        e.set(other, ExtraEnableStateParent::pParent, Ptr::new(0x6000));
        e.set(this, ExtraEnableStateParent::cFlags, 3);
        e.set(other, ExtraEnableStateParent::cFlags, 3);
        start_log(&mut e);
        assert!(!e.call(0x0043_3340, &args![this, other]).bool());
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
        e.set(other, ExtraEnableStateParent::cFlags, 1);
        assert!(e.call(0x0043_3340, &args![this, other]).bool());
        e.set(other, ExtraEnableStateParent::cFlags, 3);
        e.set(other, ExtraEnableStateParent::pParent, Ptr::new(0x7000));
        assert!(e.call(0x0043_3340, &args![this, other]).bool());
    }

    /// A constructor of a class with an inline list at +0xc: the base
    /// constructor, the vtable, and the list constructed in place.
    fn check_list_constructor(addr: u32, extra_type: u8, vtable: u32, size: u32) {
        let mut e = list_engine();
        let this = e.mem.alloc(size);
        for offset in (0x0c..size).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![Ptr::<()>::new(this)]).u32(), this);
        assert_eq!(extra_type_of(&e, this), extra_type);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        for offset in (0x14..size).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0xaaaa_aaaa);
        }
    }

    /// The destructor body of a class that clears its inline list: the list
    /// is cleared, then its destructor body runs, then the base destructor.
    fn check_list_destructor(addr: u32, vtable: u32) {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        let nodes = (e.mem.alloc(8), e.mem.alloc(8));
        fill_list(&mut e, this + 0xc, &[nodes.0, nodes.1]);
        start_log(&mut e);
        e.call(addr, &args![Ptr::<()>::new(this)]);
        assert_eq!(vtable_of(&e, this), vtable);
        let log = e.call_log.clone().unwrap();
        let order: Vec<u32> = log.iter().map(|(called, _)| *called).collect();
        assert_eq!(
            order,
            vec![
                addr,
                SIMPLE_LIST_CLEAR,
                SIMPLE_LIST_DESTRUCT,
                BS_EXTRA_DATA_DESTRUCT
            ]
        );
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // The nodes after the head are gone; the items are not the list's.
        assert_eq!(list_items(&e, this + 0xc), vec![0]);
        assert!(e.mem.block_size(nodes.1).is_some());
    }

    #[test]
    fn enable_state_children_constructor_builds_an_empty_list() {
        check_list_constructor(0x0043_33a0, 0x38, EXTRA_ENABLE_STATE_CHILDREN_VTABLE, 0x14);
    }

    #[test]
    fn enable_state_children_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(
            e.call(0x0043_3410, &args![Ptr::<()>::new(this), 0u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_ENABLE_STATE_CHILDREN_VTABLE);
        e.call(0x0043_3410, &args![Ptr::<()>::new(this), 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn enable_state_children_destructor_clears_the_list() {
        check_list_destructor(0x0043_3440, EXTRA_ENABLE_STATE_CHILDREN_VTABLE);
    }

    #[test]
    fn random_teleport_marker_constructor_clears_the_marker() {
        check_cleared_constructor(0x0043_34b0, 0x3b, EXTRA_RANDOM_TELEPORT_MARKER_VTABLE, 0x10);
    }

    #[test]
    fn random_teleport_marker_compare_checks_type_and_marker() {
        check_cast_only_compare(0x0043_34e0, EXTRA_RANDOM_TELEPORT_MARKER_TYPE);
    }

    #[test]
    fn linked_ref_children_constructor_builds_an_empty_list() {
        check_list_constructor(0x0043_3530, 0x52, EXTRA_LINKED_REF_CHILDREN_VTABLE, 0x14);
    }

    #[test]
    fn linked_ref_children_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        assert_eq!(
            e.call(0x0043_35a0, &args![Ptr::<()>::new(this), 2u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_LINKED_REF_CHILDREN_VTABLE);
        e.call(0x0043_35a0, &args![Ptr::<()>::new(this), 3u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn linked_ref_children_destructor_clears_the_list() {
        check_list_destructor(0x0043_35d0, EXTRA_LINKED_REF_CHILDREN_VTABLE);
    }

    #[test]
    fn linked_ref_constructor_clears_the_reference() {
        check_cleared_constructor(0x0043_3640, 0x51, EXTRA_LINKED_REF_VTABLE, 0x10);
    }

    #[test]
    fn linked_ref_compare_checks_type_and_reference() {
        check_cast_only_compare(0x0043_3670, EXTRA_LINKED_REF_TYPE);
    }

    #[test]
    fn ash_pile_ref_constructor_clears_the_reference() {
        check_cleared_constructor(0x0043_36c0, 0x89, EXTRA_ASH_PILE_REF_VTABLE, 0x10);
    }

    #[test]
    fn activate_data_search_finds_the_entry_for_a_reference() {
        let mut e = list_engine();
        let list = e.mem.alloc(8);
        let (first, second) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, 0x1000);
        e.mem.set_u32(second, 0x2000);
        fill_list(&mut e, list, &[first, second]);
        let find = |e: &mut Engine, reference: u32| {
            e.call(0x0043_36f0, &args![reference, Ptr::<()>::new(list)])
                .u32()
        };
        assert_eq!(find(&mut e, 0x1000), first);
        assert_eq!(find(&mut e, 0x2000), second);
        assert_eq!(find(&mut e, 0x3000), 0);
        // An empty list and a null list have nothing.
        let empty = e.mem.alloc(8);
        fill_list(&mut e, empty, &[]);
        assert_eq!(
            e.call(0x0043_36f0, &args![0x1000u32, Ptr::<()>::new(empty)])
                .u32(),
            0
        );
        assert_eq!(
            e.call(0x0043_36f0, &args![0x1000u32, Ptr::<()>::NULL])
                .u32(),
            0
        );
    }

    #[test]
    fn activate_data_free_deletes_the_items_and_empties_the_list() {
        let mut e = list_engine();
        let list = e.mem.alloc(8);
        let items = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        fill_list(&mut e, list, &items);
        start_log(&mut e);
        e.call(0x0043_3750, &args![Ptr::<()>::new(list)]);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            items.iter().map(|item| vec![*item]).collect::<Vec<_>>()
        );
        assert_eq!(list_items(&e, list), vec![0]);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).len(), 3);

        // Already empty: nothing is deleted.
        start_log(&mut e);
        e.call(0x0043_3750, &args![Ptr::<()>::new(list)]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0043_3750, &args![Ptr::<()>::NULL]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn activate_ref_children_constructor_builds_a_list_and_clears_the_timer() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x14, 0xaaaa_aaaa);
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_3790, &args![Ptr::<()>::new(this)]).u32(),
            this
        );
        assert_eq!(extra_type_of(&e, this), 0x54);
        assert_eq!(vtable_of(&e, this), EXTRA_ACTIVATE_REF_CHILDREN_VTABLE);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(e.mem.f32(this + 0x14), 0.0);
    }

    #[test]
    fn activate_ref_children_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x0043_3800, &args![Ptr::<()>::new(this), 0u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_ACTIVATE_REF_CHILDREN_VTABLE);
        e.call(0x0043_3800, &args![Ptr::<()>::new(this), 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn activate_ref_children_destructor_deletes_the_items_then_the_list() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x18);
        let items = [e.mem.alloc(8), e.mem.alloc(8)];
        fill_list(&mut e, this + 0xc, &items);
        start_log(&mut e);
        e.call(0x0043_3830, &args![Ptr::<()>::new(this)]);
        assert_eq!(vtable_of(&e, this), EXTRA_ACTIVATE_REF_CHILDREN_VTABLE);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            items.iter().map(|item| vec![*item]).collect::<Vec<_>>()
        );
        assert_eq!(list_items(&e, this + 0xc), vec![0]);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // The destructor body of the list comes after the entries are freed.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(called, _)| *called)
            .filter(|called| [OPERATOR_DELETE, SIMPLE_LIST_DESTRUCT].contains(called))
            .collect();
        assert_eq!(
            order,
            vec![OPERATOR_DELETE, OPERATOR_DELETE, SIMPLE_LIST_DESTRUCT]
        );
    }

    #[test]
    fn activate_ref_constructor_builds_the_list_and_the_text() {
        let mut e = list_engine();
        e.register(BS_STRING_CONSTRUCT, |_, a| ret(a[0]));
        e.register(BS_STRING_SET, |_, _| ret(1));
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0xaaaa_aaaa);
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_38b0, &args![Ptr::<()>::new(this)]).u32(),
            this
        );
        assert_eq!(extra_type_of(&e, this), 0x53);
        assert_eq!(vtable_of(&e, this), EXTRA_ACTIVATE_REF_VTABLE);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![this + 0xc]]);
        // Only the flag byte is cleared.
        assert_eq!(e.mem.u32(this + 0x14), 0xaaaa_aa00);
        assert_eq!(calls(&e, BS_STRING_CONSTRUCT), vec![vec![this + 0x18]]);
        assert_eq!(
            calls(&e, BS_STRING_SET),
            vec![vec![this + 0x18, EMPTY_STRING, 0]]
        );
        // List, then string constructor, then the string is set.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(called, _)| *called)
            .filter(|called| {
                [SIMPLE_LIST_CONSTRUCT, BS_STRING_CONSTRUCT, BS_STRING_SET].contains(called)
            })
            .collect();
        assert_eq!(
            order,
            vec![SIMPLE_LIST_CONSTRUCT, BS_STRING_CONSTRUCT, BS_STRING_SET]
        );
    }

    #[test]
    fn activate_ref_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = list_engine();
        e.register(BS_STRING_CLEAR, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x0043_3940, &args![Ptr::<()>::new(this), 0u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_ACTIVATE_REF_VTABLE);
        e.call(0x0043_3940, &args![Ptr::<()>::new(this), 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn activate_ref_destructor_frees_the_entries_clears_the_text_and_the_list() {
        let mut e = list_engine();
        e.register(BS_STRING_CLEAR, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let items = [e.mem.alloc(8), e.mem.alloc(8)];
        fill_list(&mut e, this + 0xc, &items);
        start_log(&mut e);
        e.call(0x0043_3970, &args![Ptr::<()>::new(this)]);
        assert_eq!(vtable_of(&e, this), EXTRA_ACTIVATE_REF_VTABLE);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            items.iter().map(|item| vec![*item]).collect::<Vec<_>>()
        );
        assert_eq!(calls(&e, BS_STRING_CLEAR), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(called, _)| *called)
            .filter(|called| {
                [
                    OPERATOR_DELETE,
                    BS_STRING_CLEAR,
                    SIMPLE_LIST_DESTRUCT,
                    BS_EXTRA_DATA_DESTRUCT,
                ]
                .contains(called)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                OPERATOR_DELETE,
                OPERATOR_DELETE,
                BS_STRING_CLEAR,
                SIMPLE_LIST_DESTRUCT,
                BS_EXTRA_DATA_DESTRUCT
            ]
        );
    }

    /// Doubles for the string and list-count functions `Compare` uses.
    fn activate_ref_engine(cast_ok: bool) -> Engine {
        let mut e = list_engine();
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(if cast_ok { a[0] } else { 0 })
        });
        e.register(SIMPLE_LIST_COUNT, |e, a| {
            let mut count = 0;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            ret(count)
        });
        // The length is the stored one (the 0xffff case is not needed).
        e.register(BS_STRING_LENGTH, |e, a| ret(e.mem.u16(a[0] + 4) as u32));
        e.register(BS_STRING_DATA, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRCMP, |e, a| {
            let read = |e: &Engine, mut at: u32| {
                let mut text = vec![];
                while e.mem.u8(at) != 0 {
                    text.push(e.mem.u8(at));
                    at += 1;
                }
                text
            };
            let (first, second) = (read(e, a[0]), read(e, a[1]));
            ret(match first.cmp(&second) {
                std::cmp::Ordering::Less => u32::MAX,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })
        });
        e
    }

    /// An `ExtraActivateRef` holding `entries` (reference, delay), the flag
    /// byte and the text.
    fn make_activate_ref(e: &mut Engine, entries: &[(u32, f32)], flags: u8, text: &str) -> u32 {
        let this = e.mem.alloc(0x20);
        let items: Vec<u32> = entries
            .iter()
            .map(|(reference, delay)| {
                let item = e.mem.alloc(8);
                e.mem.set_u32(item, *reference);
                e.mem.set_f32(item + 4, *delay);
                item
            })
            .collect();
        fill_list(e, this + 0xc, &items);
        e.mem.set_u8(this + 0x14, flags);
        if !text.is_empty() {
            let chars = e.mem.alloc(text.len() as u32 + 1);
            for (i, byte) in text.bytes().enumerate() {
                e.mem.set_u8(chars + i as u32, byte);
            }
            e.mem.set_u32(this + 0x18, chars);
            e.mem.set_u16(this + 0x1c, text.len() as u16);
        }
        this
    }

    fn activate_ref_compare(e: &mut Engine, this: u32, other: u32) -> bool {
        e.call(
            0x0043_3a20,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(other)],
        )
        .bool()
    }

    #[test]
    fn activate_ref_entry_lookup_searches_the_parent_list() {
        let mut e = activate_ref_engine(true);
        let this = make_activate_ref(&mut e, &[(0x1000, 1.0), (0x2000, 2.0)], 0, "");
        let items = list_items(&e, this + 0xc);
        let find = |e: &mut Engine, reference: u32| {
            e.call(0x0043_3a00, &args![Ptr::<()>::new(this), reference])
                .u32()
        };
        assert_eq!(find(&mut e, 0x1000), items[0]);
        assert_eq!(find(&mut e, 0x2000), items[1]);
        assert_eq!(find(&mut e, 0x3000), 0);
    }

    #[test]
    fn activate_ref_compare_checks_the_class_and_does_not_ask_the_base() {
        let mut e = activate_ref_engine(false);
        let this = make_activate_ref(&mut e, &[], 0, "");
        let other = make_activate_ref(&mut e, &[], 0, "");
        start_log(&mut e);
        assert!(activate_ref_compare(&mut e, this, other));
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                BS_EXTRA_DATA_TYPE,
                EXTRA_ACTIVATE_REF_TYPE,
                0
            ]]
        );
        assert!(calls(&e, SIMPLE_LIST_COUNT).is_empty());

        let mut e = activate_ref_engine(true);
        e.register_double(BS_EXTRA_DATA_COMPARE, |_, _| ret(1));
        let this = make_activate_ref(&mut e, &[], 0, "");
        let other = make_activate_ref(&mut e, &[], 0, "");
        start_log(&mut e);
        assert!(!activate_ref_compare(&mut e, this, other));
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
    }

    #[test]
    fn activate_ref_compare_checks_the_entries_flags_and_texts() {
        let mut e = activate_ref_engine(true);
        let entries = [(0x1000, 1.0), (0x2000, 2.0)];
        let this = make_activate_ref(&mut e, &entries, 3, "hello");
        // The same entries in the other order, the same flags and text.
        let same = make_activate_ref(&mut e, &[entries[1], entries[0]], 3, "hello");
        assert!(!activate_ref_compare(&mut e, this, same));

        // A different number of entries.
        let fewer = make_activate_ref(&mut e, &entries[..1], 3, "hello");
        assert!(activate_ref_compare(&mut e, this, fewer));
        // The same number, but one reference is missing in the other.
        let other_ref = make_activate_ref(&mut e, &[entries[0], (0x3000, 2.0)], 3, "hello");
        assert!(activate_ref_compare(&mut e, this, other_ref));
        // The same references, but a delay differs (the 8 bytes are compared).
        let other_delay = make_activate_ref(&mut e, &[entries[0], (0x2000, 5.0)], 3, "hello");
        assert!(activate_ref_compare(&mut e, this, other_delay));
        // Different flags.
        let other_flags = make_activate_ref(&mut e, &entries, 2, "hello");
        assert!(activate_ref_compare(&mut e, this, other_flags));
        // Different text, same length.
        let other_text = make_activate_ref(&mut e, &entries, 3, "hellp");
        assert!(activate_ref_compare(&mut e, this, other_text));
        // Text against no text, in both directions.
        let no_text = make_activate_ref(&mut e, &entries, 3, "");
        assert!(activate_ref_compare(&mut e, this, no_text));
        assert!(activate_ref_compare(&mut e, no_text, this));
        // No text on both sides.
        let also_no_text = make_activate_ref(&mut e, &entries, 3, "");
        assert!(!activate_ref_compare(&mut e, no_text, also_no_text));
    }

    #[test]
    fn activate_ref_compare_orders_its_string_calls_like_the_game() {
        let mut e = activate_ref_engine(true);
        let this = make_activate_ref(&mut e, &[], 0, "abc");
        let other = make_activate_ref(&mut e, &[], 0, "abd");
        start_log(&mut e);
        assert!(activate_ref_compare(&mut e, this, other));
        // `strcmp(thisText, otherText)`, the characters read from the other
        // string first.
        let chars_this = e.mem.u32(this + 0x18);
        let chars_other = e.mem.u32(other + 0x18);
        assert_eq!(calls(&e, STRCMP), vec![vec![chars_this, chars_other]]);
        assert_eq!(
            calls(&e, BS_STRING_DATA),
            vec![vec![other + 0x18], vec![this + 0x18]]
        );
    }

    #[test]
    fn activate_ref_copy_replaces_the_entries_flags_and_text() {
        let mut e = activate_ref_engine(true);
        e.register(REF_ACTIVATE_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        e.register(BS_STRING_COPY, |_, _| Ret::default());
        let this = make_activate_ref(&mut e, &[(0x9000, 9.0), (0x9100, 9.5)], 1, "");
        let old_items = list_items(&e, this + 0xc);
        let other = make_activate_ref(&mut e, &[(0x1000, 1.0), (0x2000, 2.0)], 5, "text");
        let source_items = list_items(&e, other + 0xc);
        start_log(&mut e);
        e.call(
            0x0043_3b70,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(other)],
        );
        // The old entries were deleted.
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            old_items.iter().map(|item| vec![*item]).collect::<Vec<_>>()
        );
        // Each source entry was copied (8 bytes) into a new one, each added
        // at the head: the order is reversed.
        let copies = list_items(&e, this + 0xc);
        assert_eq!(copies.len(), 2);
        assert!(source_items.iter().all(|item| !copies.contains(item)));
        assert_eq!(
            (e.mem.u32(copies[0]), e.mem.f32(copies[0] + 4)),
            (0x2000, 2.0)
        );
        assert_eq!(
            (e.mem.u32(copies[1]), e.mem.f32(copies[1] + 4)),
            (0x1000, 1.0)
        );
        assert_eq!(calls(&e, MEMCPY).len(), 2);
        assert_eq!(calls(&e, MEMCPY)[0][1..], [source_items[0], 8]);
        // The flags and the text were copied.
        assert_eq!(e.mem.u8(this + 0x14), 5);
        assert_eq!(
            calls(&e, BS_STRING_COPY),
            vec![vec![this + 0x18, other + 0x18]]
        );
        // The source is untouched.
        assert_eq!(list_items(&e, other + 0xc), source_items);
    }

    #[test]
    fn activate_ref_copy_of_another_class_does_nothing() {
        let mut e = activate_ref_engine(false);
        e.register(BS_STRING_COPY, |_, _| panic!("no text copy"));
        let this = make_activate_ref(&mut e, &[(0x9000, 9.0)], 1, "");
        let other = make_activate_ref(&mut e, &[(0x1000, 1.0)], 5, "text");
        let before = list_items(&e, this + 0xc);
        start_log(&mut e);
        e.call(
            0x0043_3b70,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(other)],
        );
        assert_eq!(list_items(&e, this + 0xc), before);
        assert_eq!(e.mem.u8(this + 0x14), 1);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn decal_refs_constructor_builds_an_empty_list() {
        check_list_constructor(0x0043_3ca0, 0x57, EXTRA_DECAL_REFS_VTABLE, 0x14);
    }

    #[test]
    fn decal_refs_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        assert_eq!(
            e.call(0x0043_3d10, &args![Ptr::<()>::new(this), 0u32])
                .u32(),
            this
        );
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(vtable_of(&e, this), EXTRA_DECAL_REFS_VTABLE);
        e.call(0x0043_3d10, &args![Ptr::<()>::new(this), 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn decal_refs_destructor_frees_the_entries_then_destroys_the_list() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        let entries = [e.mem.alloc(0x1c), e.mem.alloc(0x1c)];
        fill_list(&mut e, this + 0xc, &entries);
        start_log(&mut e);
        e.call(0x0043_3d40, &args![Ptr::<()>::new(this)]);
        assert_eq!(vtable_of(&e, this), EXTRA_DECAL_REFS_VTABLE);
        // Each entry is removed from the head and deleted, then the list's
        // destructor body and the base destructor run.
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![entries[0]], vec![entries[1]]]
        );
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).len(), 2);
        assert!(calls(&e, SIMPLE_LIST_REMOVE_HEAD)
            .iter()
            .all(|call| *call == vec![this + 0xc]));
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(list_items(&e, this + 0xc), vec![0]);
    }

    // ---- fourth batch: 00433db0 to 004353d0 ----

    fn raw(addr: u32) -> Ptr<()> {
        Ptr::new(addr)
    }

    /// An entry block holding `words`.
    fn make_entry(e: &mut Engine, size: u32, words: &[u32]) -> u32 {
        let entry = e.mem.alloc(size);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(entry + 4 * i as u32, *word);
        }
        entry
    }

    /// The doubles for the list functions the fourth batch calls on top of
    /// `list_engine`: the count, the membership test, the removal of an item
    /// (same algorithm as `00905330`: the search starts at the node given) and
    /// the `REF_DECAL_DATA` constructor.
    fn refs_engine(cast_ok: bool) -> Engine {
        let mut e = list_engine();
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(if cast_ok { a[0] } else { 0 })
        });
        e.register(SIMPLE_LIST_COUNT, |e, a| {
            let mut count = 0;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            ret(count)
        });
        e.register(SIMPLE_LIST_CONTAINS, |e, a| {
            let target = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == target {
                    return ret(1);
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(SIMPLE_LIST_REMOVE_ITEM, |e, a| {
            let start = a[0];
            let target = e.mem.u32(a[1]);
            let (mut previous, mut current) = (start, start);
            while current != 0 && e.mem.u32(current) != target {
                previous = current;
                current = e.mem.u32(current + 4);
            }
            if current == 0 {
                return Ret::default();
            }
            if current == start {
                let next = e.mem.u32(start + 4);
                if next != 0 {
                    let item = e.mem.u32(next);
                    let after = e.mem.u32(next + 4);
                    e.mem.set_u32(start, item);
                    e.mem.set_u32(start + 4, after);
                    e.mem.free(next);
                } else {
                    e.mem.set_u32(start, 0);
                }
            } else {
                let after = e.mem.u32(current + 4);
                e.mem.set_u32(previous + 4, after);
                e.mem.free(current);
            }
            Ret::default()
        });
        e.register(REF_DECAL_DATA_CONSTRUCT, |e, a| {
            for word in 0..7 {
                e.mem.set_u32(a[0] + 4 * word, 0);
            }
            ret(a[0])
        });
        e
    }

    /// The doubles for what the `InitItem`s call besides the list functions.
    /// `references` maps a resolved form id (the file index 1 is added to the
    /// top byte by the `AddCompileIndex` double) to a reference; `persistent`
    /// lists the references that are persistent.
    fn init_engine(references: Vec<(u32, u32)>, persistent: Vec<u32>) -> Engine {
        let mut e = refs_engine(true);
        e.register(FORM_GET_FILE, |_, _| ret(1));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | (a[1] << 24));
            Ret::default()
        });
        // The form looked up is the id itself; the cast finds the reference.
        e.register(LOOKUP_FORM_BY_ID, |_, a| ret(a[0]));
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(references
                .iter()
                .find(|(id, _)| *id == a[0])
                .map_or(0, |(_, reference)| *reference))
        });
        e.register_double(REFR_GET_REF_PERSISTS, move |_, a| {
            ret(persistent.contains(&a[0]) as u32)
        });
        e.register(MASTERFILE_LOG, |_, _| Ret::default());
        e.register(REFR_EXTRA_LIST, |_, a| ret(a[0] + 0x44));
        for addr in [
            REFLECTOR_ADD_FIRST,
            REFLECTOR_ADD_SECOND,
            WATER_LIGHT_REF_SET,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn decal_init_item_resolves_ids_and_drops_unresolved_entries() {
        let mut e = init_engine(vec![(0x0100_0010, 0x5010), (0x0100_0030, 0x5030)], vec![]);
        let this = e.mem.alloc(0x14);
        let entries: Vec<u32> = [0x10u32, 0x20, 0x30]
            .iter()
            .map(|id| make_entry(&mut e, 0x1c, &[*id, 0xdead, 0xbeef]))
            .collect();
        fill_list(&mut e, this + 0xc, &entries);
        start_log(&mut e);
        e.call(0x0043_3db0, &args![raw(this), raw(0x9000)]);
        // The middle entry is gone (removed after its predecessor, deleted
        // and logged with the id after the compile index); the others hold
        // the reference in their first word.
        assert_eq!(list_items(&e, this + 0xc), vec![entries[0], entries[2]]);
        assert_eq!(e.mem.u32(entries[0]), 0x5010);
        assert_eq!(e.mem.u32(entries[2]), 0x5030);
        assert_eq!(e.mem.u32(entries[0] + 4), 0xdead);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![entries[1]]]);
        assert_eq!(e.mem.block_size(entries[1]), None);
        assert_eq!(
            calls(&e, MASTERFILE_LOG),
            vec![vec![MESSAGE_DECAL_NOT_FOUND, 0x0100_0020]]
        );
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_ITEM).len(), 1);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_ITEM)[0][0], this + 0xc);
        assert!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).is_empty());
        // The owner's file is asked for (-1) once per entry, and the id
        // handed to the lookup carries the file's compile index.
        assert_eq!(calls(&e, FORM_GET_FILE), vec![vec![0x9000, 0xffff_ffff]; 3]);
        assert_eq!(
            calls(&e, LOOKUP_FORM_BY_ID),
            vec![vec![0x0100_0010], vec![0x0100_0020], vec![0x0100_0030]]
        );
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![0x0100_0010, 0, TES_FORM_TYPE, TES_OBJECT_REFR_TYPE, 0]
        );
    }

    #[test]
    fn decal_init_item_removes_unresolved_heads_with_remove_head() {
        let mut e = init_engine(vec![(0x0100_0010, 0x5010)], vec![]);
        let this = e.mem.alloc(0x14);
        let entries: Vec<u32> = [0x20u32, 0x40, 0x10]
            .iter()
            .map(|id| make_entry(&mut e, 0x1c, &[*id]))
            .collect();
        fill_list(&mut e, this + 0xc, &entries);
        start_log(&mut e);
        e.call(0x0043_3db0, &args![raw(this), raw(0x9000)]);
        // Both unresolved entries were at the head when they were dropped.
        assert_eq!(list_items(&e, this + 0xc), vec![entries[2]]);
        assert_eq!(e.mem.u32(entries[2]), 0x5010);
        assert_eq!(
            calls(&e, SIMPLE_LIST_REMOVE_HEAD),
            vec![vec![this + 0xc]; 2]
        );
        assert!(calls(&e, SIMPLE_LIST_REMOVE_ITEM).is_empty());
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![entries[0]], vec![entries[1]]]
        );
        // An empty list does nothing at all.
        let empty = e.mem.alloc(0x14);
        start_log(&mut e);
        e.call(0x0043_3db0, &args![raw(empty), raw(0x9000)]);
        assert!(calls(&e, FORM_GET_FILE).is_empty());
    }

    #[test]
    fn reflector_init_item_checks_persistence_and_notifies_the_reference() {
        let mut e = init_engine(
            vec![
                (0x0100_0010, 0x5010),
                (0x0100_0020, 0x5020),
                (0x0100_0030, 0x5030),
                (0x0100_0050, 0x5050),
            ],
            vec![0x5010, 0x5030, 0x5050],
        );
        let this = e.mem.alloc(0x14);
        // (id, effect flags): 0x10 flags 1, 0x20 not persistent, 0x30 flags 3,
        // 0x40 unknown, 0x50 no flags.
        let specs = [(0x10u32, 1u32), (0x20, 3), (0x30, 3), (0x40, 1), (0x50, 0)];
        let entries: Vec<u32> = specs
            .iter()
            .map(|(id, flags)| make_entry(&mut e, 8, &[*id, *flags]))
            .collect();
        fill_list(&mut e, this + 0xc, &entries);
        start_log(&mut e);
        e.call(0x0043_3ed0, &args![raw(this), raw(0x9000)]);
        assert_eq!(
            list_items(&e, this + 0xc),
            vec![entries[0], entries[2], entries[4]]
        );
        assert_eq!(e.mem.u32(entries[0]), 0x5010);
        assert_eq!(e.mem.u32(entries[2]), 0x5030);
        assert_eq!(e.mem.u32(entries[4]), 0x5050);
        assert_eq!(
            calls(&e, MASTERFILE_LOG),
            vec![
                vec![MESSAGE_NOT_PERSISTENT, 0x0100_0020],
                vec![MESSAGE_REFLECTOR_NOT_FOUND, 0x0100_0040]
            ]
        );
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![entries[1]], vec![entries[3]]]
        );
        // Bit 0 calls the first method of the reference's extra-data list
        // (the reference + 0x44) with the owner and 1, bit 1 the second.
        assert_eq!(
            calls(&e, REFLECTOR_ADD_FIRST),
            vec![
                vec![0x5010 + 0x44, 0x9000, 1],
                vec![0x5030 + 0x44, 0x9000, 1]
            ]
        );
        assert_eq!(
            calls(&e, REFLECTOR_ADD_SECOND),
            vec![vec![0x5030 + 0x44, 0x9000, 1]]
        );
        // Only the persistent references were asked about (not the unknown).
        assert_eq!(
            calls(&e, REFR_GET_REF_PERSISTS),
            vec![vec![0x5010], vec![0x5020], vec![0x5030], vec![0x5050]]
        );
    }

    #[test]
    fn lit_water_init_item_keeps_the_good_references_in_order() {
        let mut e = init_engine(
            vec![
                (0x0100_0010, 0x5010),
                (0x0100_0020, 0x5020),
                (0x0100_0030, 0x5030),
            ],
            vec![0x5010, 0x5030],
        );
        let this = e.mem.alloc(0x14);
        // 0x10 and 0x30 are good, 0x20 is not persistent, 0x40 is unknown.
        fill_list(&mut e, this + 0xc, &[0x10, 0x20, 0x30, 0x40]);
        start_log(&mut e);
        e.call(0x0043_4050, &args![raw(this), raw(0x9000)]);
        // The temporary list reverses the good references, the refill
        // reverses them again.
        assert_eq!(list_items(&e, this + 0xc), vec![0x5010, 0x5030]);
        assert_eq!(
            calls(&e, MASTERFILE_LOG),
            vec![
                vec![MESSAGE_NOT_PERSISTENT, 0x0100_0020],
                vec![MESSAGE_LIT_WATER_NOT_FOUND, 0x0100_0040]
            ]
        );
        assert_eq!(
            calls(&e, WATER_LIGHT_REF_SET),
            vec![
                vec![0x5010 + 0x44, 0x9000, 1],
                vec![0x5030 + 0x44, 0x9000, 1]
            ]
        );
        // The temporary list (a stack local) is built, cleared and destroyed;
        // the own list is cleared once.
        let constructed = calls(&e, SIMPLE_LIST_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        let temporary = constructed[0][0];
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![temporary]]);
        assert_eq!(
            calls(&e, SIMPLE_LIST_CLEAR),
            vec![vec![this + 0xc], vec![temporary]]
        );
        // Nothing is deleted: the references are not owned.
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    /// An extra data of `class_size` bytes whose list at +0xc holds entries of
    /// `entry_size` bytes with the first words `keys`; the other words of an
    /// entry are `fill`.
    fn make_entry_extra(
        e: &mut Engine,
        class_size: u32,
        entry_size: u32,
        keys: &[u32],
        fill: u32,
    ) -> u32 {
        let this = e.mem.alloc(class_size);
        let entries: Vec<u32> = keys
            .iter()
            .map(|key| {
                let entry = e.mem.alloc(entry_size);
                for word in 1..entry_size / 4 {
                    e.mem.set_u32(entry + 4 * word, fill);
                }
                e.mem.set_u32(entry, *key);
                entry
            })
            .collect();
        fill_list(e, this + 0xc, &entries);
        this
    }

    /// The `Compare` of the decal, reflected and reflector classes (the entry
    /// lists are compared by key then by bytes).
    fn check_entry_list_compare(addr: u32, entry_size: u32) {
        let compare = |e: &mut Engine, this: u32, other: u32| {
            e.call(addr, &args![raw(this), raw(other)]).bool()
        };
        // Not of the class: true, and no list is looked at.
        let mut e = refs_engine(false);
        let this = make_entry_extra(&mut e, 0x14, entry_size, &[1], 0);
        let other = make_entry_extra(&mut e, 0x14, entry_size, &[1], 0);
        start_log(&mut e);
        assert!(compare(&mut e, this, other));
        assert!(calls(&e, SIMPLE_LIST_COUNT).is_empty());
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());

        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, entry_size, &[1, 2], 7);
        // The same keys in another order and the same bytes: equal. The base
        // is not asked.
        let same = make_entry_extra(&mut e, 0x14, entry_size, &[2, 1], 7);
        start_log(&mut e);
        assert!(!compare(&mut e, this, same));
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
        // A different number of entries (either way).
        let fewer = make_entry_extra(&mut e, 0x14, entry_size, &[1], 7);
        assert!(compare(&mut e, this, fewer));
        assert!(compare(&mut e, fewer, this));
        // The same number, but a key is missing here.
        let other_key = make_entry_extra(&mut e, 0x14, entry_size, &[1, 3], 7);
        assert!(compare(&mut e, this, other_key));
        // The same keys, but the last compared word differs.
        let other_bytes = make_entry_extra(&mut e, 0x14, entry_size, &[1, 2], 8);
        assert!(compare(&mut e, this, other_bytes));
        // Two empty lists are equal.
        let none = make_entry_extra(&mut e, 0x14, entry_size, &[], 0);
        let also_none = make_entry_extra(&mut e, 0x14, entry_size, &[], 0);
        assert!(!compare(&mut e, none, also_none));
        // Only `entry_size` bytes are compared.
        if entry_size == 8 {
            let beyond = make_entry_extra(&mut e, 0x14, 0x1c, &[1, 2], 7);
            let beyond_other = make_entry_extra(&mut e, 0x14, 0x1c, &[1, 2], 7);
            let entry = e.mem.u32(beyond_other + 0xc);
            e.mem.set_u32(entry + 8, 0x1234);
            assert!(!compare(&mut e, beyond, beyond_other));
        }
    }

    #[test]
    fn decal_refs_compare_checks_class_count_keys_and_bytes() {
        check_entry_list_compare(0x0043_41e0, 0x1c);
        // The lookup is asked on `this` for the other list's keys.
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 0x1c, &[5], 1);
        let other = make_entry_extra(&mut e, 0x14, 0x1c, &[5], 1);
        let this_entry = e.mem.u32(this + 0xc);
        let other_entry = e.mem.u32(other + 0xc);
        start_log(&mut e);
        assert!(!e.call(0x0043_41e0, &args![raw(this), raw(other)]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, BS_EXTRA_DATA_TYPE, EXTRA_DECAL_REFS_TYPE, 0]]
        );
        assert_eq!(
            calls(&e, SIMPLE_LIST_COUNT),
            vec![vec![this + 0xc], vec![other + 0xc]]
        );
        assert_eq!(calls(&e, MEMCMP), vec![vec![this_entry, other_entry, 0x1c]]);
    }

    #[test]
    fn reflected_and_reflector_refs_compare_check_class_count_keys_and_bytes() {
        check_entry_list_compare(0x0043_4620, 8);
        check_entry_list_compare(0x0043_4bb0, 8);
        for (addr, target_type) in [
            (0x0043_4620, EXTRA_REFLECTED_REFS_TYPE),
            (0x0043_4bb0, EXTRA_REFLECTOR_REFS_TYPE),
        ] {
            let mut e = refs_engine(false);
            let this = make_entry_extra(&mut e, 0x14, 8, &[1], 0);
            let other = make_entry_extra(&mut e, 0x14, 8, &[1], 0);
            start_log(&mut e);
            assert!(e.call(addr, &args![raw(this), raw(other)]).bool());
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
            );
        }
    }

    /// The list of the entries (each as its words) at +0xc of `this`.
    fn entry_words(e: &Engine, this: u32, words: u32) -> Vec<Vec<u32>> {
        list_items(e, this + 0xc)
            .into_iter()
            .filter(|entry| *entry != 0)
            .map(|entry| (0..words).map(|word| e.mem.u32(entry + 4 * word)).collect())
            .collect()
    }

    #[test]
    fn decal_refs_copy_replaces_the_entries_with_reversed_copies() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 0x1c, &[0x90, 0x91], 9);
        let old_entries = list_items(&e, this + 0xc);
        let other = make_entry_extra(&mut e, 0x14, 0x1c, &[1, 2], 7);
        let source_entries = list_items(&e, other + 0xc);
        start_log(&mut e);
        e.call(0x0043_42b0, &args![raw(this), raw(other)]);
        // The old entries were deleted, the source entries copied (0x1c
        // bytes each) and added at the head: the order is reversed.
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            old_entries
                .iter()
                .map(|entry| vec![*entry])
                .collect::<Vec<_>>()
        );
        assert_eq!(entry_words(&e, this, 3), vec![vec![2, 7, 7], vec![1, 7, 7]]);
        let copies = list_items(&e, this + 0xc);
        assert!(source_entries.iter().all(|entry| !copies.contains(entry)));
        assert_eq!(calls(&e, REF_DECAL_DATA_CONSTRUCT).len(), 2);
        assert_eq!(calls(&e, MEMCPY)[0][1..], [source_entries[0], 0x1c]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x1c], vec![0x1c]]);
        // The source is untouched.
        assert_eq!(list_items(&e, other + 0xc), source_entries);
    }

    #[test]
    fn decal_refs_copy_of_another_class_does_nothing() {
        let mut e = refs_engine(false);
        let this = make_entry_extra(&mut e, 0x14, 0x1c, &[0x90], 9);
        let before = list_items(&e, this + 0xc);
        let other = make_entry_extra(&mut e, 0x14, 0x1c, &[1], 7);
        start_log(&mut e);
        e.call(0x0043_42b0, &args![raw(this), raw(other)]);
        assert_eq!(list_items(&e, this + 0xc), before);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn reflected_and_reflector_refs_copy_clear_without_deleting_then_copy() {
        for (addr, target_type) in [
            (0x0043_46f0, EXTRA_REFLECTED_REFS_TYPE),
            (0x0043_4c80, EXTRA_REFLECTOR_REFS_TYPE),
        ] {
            let mut e = refs_engine(true);
            let this = make_entry_extra(&mut e, 0x14, 8, &[0x90, 0x91], 9);
            let other = make_entry_extra(&mut e, 0x14, 8, &[1, 2], 7);
            let source_entries = list_items(&e, other + 0xc);
            start_log(&mut e);
            e.call(addr, &args![raw(this), raw(other)]);
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
            );
            // The old entries are only removed from the list (not deleted):
            // the copies are new 8-byte blocks, added at the head.
            assert!(calls(&e, OPERATOR_DELETE).is_empty());
            assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).len(), 2);
            assert_eq!(entry_words(&e, this, 2), vec![vec![2, 7], vec![1, 7]]);
            assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8], vec![8]]);
            assert_eq!(calls(&e, MEMCPY)[0][1..], [source_entries[0], 8]);
            assert_eq!(list_items(&e, other + 0xc), source_entries);

            // Another class: nothing happens.
            let mut e = refs_engine(false);
            let this = make_entry_extra(&mut e, 0x14, 8, &[0x90], 9);
            let other = make_entry_extra(&mut e, 0x14, 8, &[1], 7);
            start_log(&mut e);
            e.call(addr, &args![raw(this), raw(other)]);
            assert_eq!(entry_words(&e, this, 2), vec![vec![0x90, 9]]);
            assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        }
    }

    #[test]
    fn decal_refs_free_list_deletes_every_entry() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 0x1c, &[1, 2, 3], 0);
        let entries = list_items(&e, this + 0xc);
        start_log(&mut e);
        e.call(0x0043_43c0, &args![raw(this)]);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            entries.iter().map(|entry| vec![*entry]).collect::<Vec<_>>()
        );
        assert_eq!(list_items(&e, this + 0xc), vec![0]);
        // An empty list: nothing is called but the emptiness test.
        start_log(&mut e);
        e.call(0x0043_43c0, &args![raw(this)]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).is_empty());
    }

    #[test]
    fn get_ref_decal_data_finds_the_entry_by_its_first_word() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 0x1c, &[5, 6, 7], 0);
        let entries = list_items(&e, this + 0xc);
        let find = |e: &mut Engine, key: u32| e.call(0x0043_4410, &args![raw(this), key]).u32();
        assert_eq!(find(&mut e, 5), entries[0]);
        assert_eq!(find(&mut e, 7), entries[2]);
        assert_eq!(find(&mut e, 8), 0);
        let empty = make_entry_extra(&mut e, 0x14, 0x1c, &[], 0);
        assert_eq!(e.call(0x0043_4410, &args![raw(empty), 5u32]).u32(), 0);
    }

    #[test]
    fn set_decal_data_creates_or_updates_the_entry() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 0x1c, &[5], 0);
        let existing = e.mem.u32(this + 0xc);
        let intersect = e.mem.alloc(12);
        let normal = e.mem.alloc(12);
        for word in 0..3 {
            e.mem.set_f32(intersect + 4 * word, 1.0 + word as f32);
            e.mem.set_f32(normal + 4 * word, 10.0 + word as f32);
        }
        start_log(&mut e);
        // An existing key: the entry is updated in place.
        e.call(
            0x0043_4480,
            &args![raw(this), 5u32, raw(intersect), raw(normal)],
        );
        assert_eq!(list_items(&e, this + 0xc), vec![existing]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.f32(existing + 4), 1.0);
        assert_eq!(e.mem.f32(existing + 0xc), 3.0);
        assert_eq!(e.mem.f32(existing + 0x10), 10.0);
        assert_eq!(e.mem.f32(existing + 0x18), 12.0);
        // A new key: a 0x1c-byte entry is built and added at the head.
        e.call(
            0x0043_4480,
            &args![raw(this), 6u32, raw(normal), raw(intersect)],
        );
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x1c]]);
        let items = list_items(&e, this + 0xc);
        assert_eq!(items.len(), 2);
        assert_eq!(items[1], existing);
        let created = items[0];
        assert_eq!(calls(&e, REF_DECAL_DATA_CONSTRUCT), vec![vec![created]]);
        assert_eq!(e.mem.u32(created), 6);
        assert_eq!(e.mem.f32(created + 4), 10.0);
        assert_eq!(e.mem.f32(created + 0x10), 1.0);
        assert_eq!(e.mem.f32(created + 0x18), 3.0);
    }

    #[test]
    fn reflected_refs_destructors_delete_the_entries_and_the_others_do_not() {
        for (addr, vtable, owned) in [
            (0x0043_4560, EXTRA_REFLECTED_REFS_VTABLE, true),
            (0x0043_4af0, EXTRA_REFLECTOR_REFS_VTABLE, true),
            (0x0043_4950, EXTRA_WATER_LIGHT_REFS_VTABLE, false),
            (0x0043_4dc0, EXTRA_LIT_WATER_REFS_VTABLE, false),
        ] {
            let mut e = refs_engine(true);
            let this = make_entry_extra(&mut e, 0x14, 8, &[1, 2], 0);
            let entries = list_items(&e, this + 0xc);
            start_log(&mut e);
            e.call(addr, &args![raw(this)]);
            assert_eq!(vtable_of(&e, this), vtable);
            let expected: Vec<Vec<u32>> = if owned {
                entries.iter().map(|entry| vec![*entry]).collect()
            } else {
                vec![]
            };
            assert_eq!(calls(&e, OPERATOR_DELETE), expected);
            let order: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(called, _)| *called)
                .filter(|called| {
                    [
                        SIMPLE_LIST_CLEAR,
                        SIMPLE_LIST_DESTRUCT,
                        BS_EXTRA_DATA_DESTRUCT,
                    ]
                    .contains(called)
                })
                .collect();
            assert_eq!(
                order,
                vec![
                    SIMPLE_LIST_CLEAR,
                    SIMPLE_LIST_DESTRUCT,
                    BS_EXTRA_DATA_DESTRUCT
                ]
            );
            assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![this + 0xc]]);
            assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0xc]]);
            assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
            assert_eq!(list_items(&e, this + 0xc), vec![0]);
        }
    }

    #[test]
    fn remove_reference_data_removes_head_or_inner_entry_and_deletes_it() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 8, &[1, 2, 3], 0);
        let entries = list_items(&e, this + 0xc);
        start_log(&mut e);
        // An inner entry is removed after its predecessor.
        e.call(0x0043_4800, &args![raw(this), 2u32]);
        assert_eq!(list_items(&e, this + 0xc), vec![entries[0], entries[2]]);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_ITEM).len(), 1);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_ITEM)[0][0], this + 0xc);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![entries[1]]]);
        // The head entry is removed with remove-head.
        e.call(0x0043_4800, &args![raw(this), 1u32]);
        assert_eq!(list_items(&e, this + 0xc), vec![entries[2]]);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
        // A key that is not there: nothing happens.
        start_log(&mut e);
        e.call(0x0043_4800, &args![raw(this), 9u32]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(list_items(&e, this + 0xc), vec![entries[2]]);
    }

    #[test]
    fn set_effect_flags_creates_or_updates_the_entry() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 8, &[5], 0);
        let existing = e.mem.u32(this + 0xc);
        start_log(&mut e);
        e.call(0x0043_48a0, &args![raw(this), 5u32, 3u32]);
        assert_eq!(list_items(&e, this + 0xc), vec![existing]);
        assert_eq!(e.mem.u32(existing + 4), 3);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        // A new key: an 8-byte entry (cleared by the list-node constructor)
        // is added at the head.
        e.call(0x0043_48a0, &args![raw(this), 6u32, 2u32]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        let items = list_items(&e, this + 0xc);
        assert_eq!(items.len(), 2);
        assert_eq!(items[1], existing);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![items[0]]]);
        assert_eq!((e.mem.u32(items[0]), e.mem.u32(items[0] + 4)), (6, 2));
    }

    #[test]
    fn reference_list_compare_checks_class_count_and_membership() {
        for (addr, target_type) in [
            (0x0043_49c0, EXTRA_WATER_LIGHT_REFS_TYPE),
            (0x0043_4e30, EXTRA_LIT_WATER_REFS_TYPE),
        ] {
            let compare = |e: &mut Engine, this: u32, other: u32| {
                e.call(addr, &args![raw(this), raw(other)]).bool()
            };
            let mut e = refs_engine(false);
            let this = make_entry_extra(&mut e, 0x14, 4, &[], 0);
            let other = make_entry_extra(&mut e, 0x14, 4, &[], 0);
            start_log(&mut e);
            assert!(compare(&mut e, this, other));
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
            );
            assert!(calls(&e, SIMPLE_LIST_COUNT).is_empty());

            let mut e = refs_engine(true);
            let build = |e: &mut Engine, items: &[u32]| {
                let this = e.mem.alloc(0x14);
                fill_list(e, this + 0xc, items);
                this
            };
            let this = build(&mut e, &[0x51, 0x52]);
            // The same references in another order: equal.
            let same = build(&mut e, &[0x52, 0x51]);
            start_log(&mut e);
            assert!(!compare(&mut e, this, same));
            assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
            // The test is asked on this list with the other list's item slot.
            assert_eq!(
                calls(&e, SIMPLE_LIST_CONTAINS),
                vec![vec![this + 0xc, same + 0xc], {
                    let second = e.mem.u32(same + 0xc + 4);
                    vec![this + 0xc, second]
                }]
            );
            // A different count, and a reference that is not in this list.
            let fewer = build(&mut e, &[0x51]);
            assert!(compare(&mut e, this, fewer));
            let other_item = build(&mut e, &[0x51, 0x53]);
            assert!(compare(&mut e, this, other_item));
            let none = build(&mut e, &[]);
            let also_none = build(&mut e, &[]);
            assert!(!compare(&mut e, none, also_none));
        }
    }

    #[test]
    fn reference_list_copy_clears_then_adds_each_missing_reference() {
        for (addr, target_type) in [
            (0x0043_4a70, EXTRA_WATER_LIGHT_REFS_TYPE),
            (0x0043_4ee0, EXTRA_LIT_WATER_REFS_TYPE),
        ] {
            let mut e = refs_engine(true);
            let build = |e: &mut Engine, items: &[u32]| {
                let this = e.mem.alloc(0x14);
                fill_list(e, this + 0xc, items);
                this
            };
            let this = build(&mut e, &[0x99]);
            // The other list holds a duplicate: it is added once.
            let other = build(&mut e, &[0x51, 0x52, 0x51]);
            start_log(&mut e);
            e.call(addr, &args![raw(this), raw(other)]);
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
            );
            assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![this + 0xc]]);
            // Added at the head: the later reference ends up first.
            assert_eq!(list_items(&e, this + 0xc), vec![0x52, 0x51]);
            assert_eq!(calls(&e, SIMPLE_LIST_ADD_HEAD).len(), 2);
            assert_eq!(list_items(&e, other + 0xc), vec![0x51, 0x52, 0x51]);

            // Another class: nothing happens.
            let mut e = refs_engine(false);
            let this = build(&mut e, &[0x99]);
            let other = build(&mut e, &[0x51]);
            start_log(&mut e);
            e.call(addr, &args![raw(this), raw(other)]);
            assert_eq!(list_items(&e, this + 0xc), vec![0x99]);
            assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        }
    }

    #[test]
    fn add_reference_if_absent_and_clear_list() {
        let mut e = refs_engine(true);
        let this = e.mem.alloc(0x14);
        fill_list(&mut e, this + 0xc, &[0x51]);
        start_log(&mut e);
        e.call(0x0043_4f60, &args![raw(this), 0x51u32]);
        assert_eq!(list_items(&e, this + 0xc), vec![0x51]);
        assert!(calls(&e, SIMPLE_LIST_ADD_HEAD).is_empty());
        e.call(0x0043_4f60, &args![raw(this), 0x52u32]);
        assert_eq!(list_items(&e, this + 0xc), vec![0x52, 0x51]);
        // Both calls ask about the address of a stack word holding the item.
        let asked = calls(&e, SIMPLE_LIST_CONTAINS);
        assert_eq!(asked.len(), 2);
        assert_eq!(asked[0][0], this + 0xc);
        assert_eq!(calls(&e, SIMPLE_LIST_ADD_HEAD)[0][0], this + 0xc);
        e.call(0x0043_4fa0, &args![raw(this)]);
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![this + 0xc]]);
        assert_eq!(list_items(&e, this + 0xc), vec![0]);
    }

    #[test]
    fn clear_reference_data_removes_the_heads_without_deleting() {
        let mut e = refs_engine(true);
        let this = make_entry_extra(&mut e, 0x14, 8, &[1, 2, 3], 0);
        start_log(&mut e);
        e.call(0x0043_4d90, &args![raw(this)]);
        assert_eq!(list_items(&e, this + 0xc), vec![0]);
        assert_eq!(
            calls(&e, SIMPLE_LIST_REMOVE_HEAD),
            vec![vec![this + 0xc]; 3]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn merchant_container_constructor_and_compare() {
        let mut e = compare_engine(true, false);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xaaaa_aaaa);
        assert_eq!(e.call(0x0043_4fc0, &args![raw(this)]).u32(), this);
        assert_eq!(extra_type_of(&e, this), 0x3c);
        assert_eq!(vtable_of(&e, this), EXTRA_MERCHANT_CONTAINER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        check_value_compare_without_base(0x0043_4ff0, EXTRA_MERCHANT_CONTAINER_TYPE);
    }

    /// A `Compare` that casts `other`, then compares the word at +0xc and does
    /// not ask the base.
    fn check_value_compare_without_base(addr: u32, target_type: u32) {
        let mut e = compare_engine(false, false);
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        start_log(&mut e);
        assert!(e.call(addr, &args![raw(this), raw(other)]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, BS_EXTRA_DATA_TYPE, target_type, 0]]
        );
        let mut e = compare_engine(true, true);
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x1111);
        e.mem.set_u32(other + 0xc, 0x1111);
        start_log(&mut e);
        // The base answer (true) is never asked.
        assert!(!e.call(addr, &args![raw(this), raw(other)]).bool());
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
        e.mem.set_u32(other + 0xc, 0x2222);
        assert!(e.call(addr, &args![raw(this), raw(other)]).bool());
    }

    #[test]
    fn lev_crea_modifier_constructor_compare_and_accessors() {
        let mut e = compare_engine(true, false);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0043_5040, &args![raw(this)]).u32(), this);
        assert_eq!(extra_type_of(&e, this), 0x1e);
        assert_eq!(vtable_of(&e, this), EXTRA_LEV_CREA_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        check_value_compare_without_base(0x0043_5070, EXTRA_LEV_CREA_MODIFIER_TYPE);

        // 00435100: 1 for 0, 0 for 4, -1 otherwise.
        for (modifier, expected) in [(0u32, 1i32), (4, 0), (1, -1), (2, -1), (3, -1), (7, -1)] {
            e.mem.set_u32(this + 0xc, modifier);
            assert_eq!(e.call(0x0043_5100, &args![raw(this)]).i32(), expected);
        }
    }

    #[test]
    fn lev_crea_modifier_multiplier_reads_the_setting_for_the_modifier() {
        let mut e = compare_engine(true, false);
        e.map(0x0118_4000, 0x1000);
        // Four settings; the accessor answers the address of the value, which
        // sits at +8 of the setting.
        let settings: Vec<u32> = [0.5f32, 1.5, 2.5, 4.0]
            .iter()
            .map(|value| {
                let setting = e.mem.alloc(0x10);
                e.mem.set_f32(setting + 8, *value);
                setting
            })
            .collect();
        for (i, setting) in settings.iter().enumerate() {
            e.mem
                .set_u32(LEV_CREA_SETTING_TABLE + 4 * i as u32, *setting);
        }
        e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0] + 8));
        let this = e.mem.alloc(0x10);
        for (modifier, expected) in [(0u32, 0.5f32), (1, 1.5), (2, 2.5), (3, 4.0), (4, 1.0)] {
            e.mem.set_u32(this + 0xc, modifier);
            start_log(&mut e);
            assert_eq!(e.call(0x0043_50c0, &args![raw(this)]).f32(), expected);
            // The setting is only read for the modifiers below 4.
            assert_eq!(
                calls(&e, SETTING_FLOAT_VALUE).len(),
                (modifier < 4) as usize
            );
        }
    }

    #[test]
    fn poison_constructor_and_compare() {
        let mut e = compare_engine(true, false);
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x0043_5150, &args![raw(this), raw(0x7777)]).u32(),
            this
        );
        assert_eq!(extra_type_of(&e, this), 0x3f);
        assert_eq!(vtable_of(&e, this), EXTRA_POISON_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x7777);
        check_value_compare_without_base(0x0043_5180, EXTRA_POISON_TYPE);
    }

    #[test]
    fn last_finished_sequence_constructor_copies_the_name() {
        let mut e = extra_engine();
        e.register(STRLEN, |e, a| {
            let mut length = 0;
            while e.mem.u8(a[0] + length) != 0 {
                length += 1;
            }
            ret(length)
        });
        e.register(STRING_COPY_CHECKED, |e, a| {
            for i in 0..a[1] {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        let name = e.mem.alloc(8);
        for (i, byte) in b"idle\0".iter().enumerate() {
            e.mem.set_u8(name + i as u32, *byte);
        }
        let this = e.mem.alloc(0x10);
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_51d0, &args![raw(this), raw(name)]).u32(),
            this
        );
        assert_eq!(extra_type_of(&e, this), 0x41);
        assert_eq!(vtable_of(&e, this), EXTRA_LAST_FINISHED_SEQUENCE_VTABLE);
        let copy = e.mem.u32(this + 0xc);
        assert_ne!(copy, name);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![5]]);
        assert_eq!(calls(&e, STRING_COPY_CHECKED), vec![vec![copy, 5, name]]);
        assert_eq!(e.mem.bytes(copy, 5), b"idle\0".to_vec());
    }

    #[test]
    fn last_finished_sequence_destructors_free_the_name() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        let name = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, name);
        start_log(&mut e);
        e.call(0x0043_52a0, &args![raw(this)]);
        assert_eq!(vtable_of(&e, this), EXTRA_LAST_FINISHED_SEQUENCE_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![name]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.block_size(name), None);

        // The scalar deleting destructor: the body, then the block when asked.
        let this = e.mem.alloc(0x10);
        let name = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, name);
        assert_eq!(e.call(0x0043_5270, &args![raw(this), 0u32]).u32(), this);
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(e.mem.block_size(name), None);
        // (The name pointer still points to the freed block: null it so the
        // second destruction deletes nothing.)
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x0043_5270, &args![raw(this), 1u32]).u32(), this);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn last_finished_sequence_compare_uses_strcmp_of_the_names() {
        let mut e = compare_engine(false, false);
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        start_log(&mut e);
        assert!(e.call(0x0043_5310, &args![raw(this), raw(other)]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                BS_EXTRA_DATA_TYPE,
                EXTRA_LAST_FINISHED_SEQUENCE_TYPE,
                0
            ]]
        );
        for answer in [0u32, 1, u32::MAX] {
            let mut e = compare_engine(true, true);
            e.register_double(STRCMP, move |_, _| ret(answer));
            let this = e.mem.alloc(0x10);
            let other = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 0x1111);
            e.mem.set_u32(other + 0xc, 0x2222);
            start_log(&mut e);
            // The base answer (true) is not asked; the names are given as
            // (this name, other name).
            assert_eq!(
                e.call(0x0043_5310, &args![raw(this), raw(other)]).bool(),
                answer != 0
            );
            assert_eq!(calls(&e, STRCMP), vec![vec![0x1111, 0x2222]]);
            assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
        }
    }

    #[test]
    fn x_target_constructor_and_destructors() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xaaaa_aaaa);
        assert_eq!(e.call(0x0043_5370, &args![raw(this)]).u32(), this);
        assert_eq!(extra_type_of(&e, this), 0x44);
        assert_eq!(vtable_of(&e, this), EXTRA_X_TARGET_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        check_scalar_deleting_destructor(0x0043_53a0, EXTRA_X_TARGET_VTABLE, 0x10);
        // The destructor body resets the vtable and runs the base.
        let other = e.mem.alloc(0x10);
        start_log(&mut e);
        e.call(0x0043_53d0, &args![raw(other)]);
        assert_eq!(vtable_of(&e, other), EXTRA_X_TARGET_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![other]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn saved_data_destructors_release_the_buffer_then_the_base() {
        let mut e = Engine::new();
        e.register(SAVED_DATA_DELETE_BUFFER, |_, _| Ret::default());
        e.register(BS_EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        for (address, vtable) in [(0x0043_5ac0u32, 0x0101_5ea0u32), (0x0043_5bb0, 0x0101_5eac)] {
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [
                    (SAVED_DATA_DELETE_BUFFER, vec![this]),
                    (BS_EXTRA_DATA_DESTRUCT, vec![this]),
                ]
            );
            assert_eq!(e.mem.u32(this), vtable);
        }
    }
}
