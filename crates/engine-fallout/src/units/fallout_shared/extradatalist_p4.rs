//! `fallout shared/extradatalist.cpp` (Xbox PDB source unit), part 4: its functions from `00421400` up to
//! (not including) `0042dd90` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradatalist`]; anything public there may be used here.
//!
//! Translated so far, in address order: the 120 functions from `00421400` to
//! `0042dd70`. The first 40 (to `00422640`) are the getters and setters of
//! single extra data (the merchant container, the leveled creature modifier
//! and original base, the cell detach time, the seen data, the north
//! rotation, the X target, the encounter zone, the emittance source, the
//! multibound reference, data and volume, the occlusion plane, the radius and
//! radiation floats, the follower list and the friend hits). The next 40 are
//! the follower removal, the refraction property, the last finished
//! sequence, the saved animation and havok data, the save size
//! (`00422c40`), the save writer and loader of the old format
//! (`SaveGame`, `LoadGame`), the save writer and loader of the buffered
//! format (`SaveGame_ov2`, `LoadGame_ov2`), the helpers of the save buffer
//! (`00428070` to `00428130`) and the constructors and destructors of the
//! lock, teleport, ownership, global, rank, count, health and uses extra data
//! (`0042c470` to `0042c6d0`). The last 40 are the constructors and destructors of
//! the extra data of the types `0x27` to `0x8F` (`0042c700` to `0042cd70`), the
//! small buffer accessors (`0042cde0` to `0042ce90`), the fix-up after the
//! buffered load (`0042ceb0`), `FinishLoadGame` (`0042d9e0`), the clean-up
//! (`0042dae0`) and `0042dd70`. The range is complete.
//!
//! The save functions test the change flags with masks that are the constant
//! 0 in this build (`AND reg,0`); the Xbox arms behind them never run and are
//! not translated (the doc of each says so).
//!
//! Every setter follows the compiler's one pattern: a null (or neutral)
//! value removes the extra data of its type; otherwise the existing extra
//! data is updated, or a new one is built (`operator new`, the subclass
//! constructor, the value stored at +0x0C) and added. The compiler's
//! exception-unwinding frames around the constructors are not translated.
//! The `float` setters and getters go through the x87 stack (`FLD`/`FSTP`),
//! so a signalling NaN comes out quiet.

#[allow(unused_imports)]
use super::extradatalist::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees outside this file, by address

/// `BaseExtraList::GetExtraData(type)` (Xbox PDB).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `BaseExtraList::AddExtra(extra)` (Xbox PDB).
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `BaseExtraList::RemoveExtra(type)` (Xbox PDB, `_ov2`).
const REMOVE_EXTRA: u32 = 0x0041_0140;
/// `BaseExtraList::HasExtra(type)` (Xbox PDB).
const HAS_EXTRA: u32 = 0x0040_fe80;
/// `BSExtraData::BSExtraData(type)`: sets the base vtable, the type and a
/// null next.
const BS_EXTRA_DATA_INIT: u32 = 0x0040_ec80;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `MOV EAX,[ECX]`: reads the word a handle points at.
const READ_WORD: u32 = 0x0055_9450;
/// Assigns the handle at `this` (the word at +0x0C of the extra data) the
/// value on the stack: when it differs, one reference is released and one
/// taken (`00401970`, `0040f6e0`).
const ASSIGN_HANDLE: u32 = 0x0066_b0d0;

/// Constructors of the extra data (`this` in ECX, the new block; they return
/// it). Each is in a unit of its own and builds a 0x10-byte object unless
/// the size is given with the call.
const MERCHANT_CONTAINER_INIT: u32 = 0x0043_4fc0;
const LEV_CREA_MOD_INIT: u32 = 0x0043_5040;
/// Takes the byte to store (a stack word).
const NO_RUMORS_INIT: u32 = 0x0043_6090;
/// 0x14 bytes.
const LEVELED_CREATURE_INIT: u32 = 0x0043_11f0;
const SEEN_DATA_INIT: u32 = 0x0040_f590;
const NORTH_ROTATION_INIT: u32 = 0x0040_f680;
const DETACH_TIME_INIT: u32 = 0x0040_f6b0;
const X_TARGET_INIT: u32 = 0x0043_5370;
const ENCOUNTER_ZONE_INIT: u32 = 0x0043_2e60;
const EMITTANCE_SOURCE_INIT: u32 = 0x0043_5440;
const MULTIBOUND_REF_INIT: u32 = 0x0043_5510;
const MULTIBOUND_DATA_INIT: u32 = 0x0043_5590;
const MULTIBOUND_INIT: u32 = 0x0043_5690;
const OCCLUSION_PLANE_INIT: u32 = 0x0043_57a0;
const FOLLOWER_INIT: u32 = 0x0043_0dd0;
/// 0x1C bytes.
const FRIEND_HITS_INIT: u32 = 0x0043_5c40;

/// `float` of the type `0x1E` extra data, computed by the extra data itself
/// (ST0), and the word it also gives (EAX); `this` = the extra data.
const LEV_CREA_MOD_GET_FLOAT: u32 = 0x0043_50c0;
const LEV_CREA_MOD_GET_WORD: u32 = 0x0043_5100;
/// `ExtraFriendHits::AddHit` and `ExtraFriendHits::GetHitCount` (Xbox PDB).
const FRIEND_HITS_ADD_HIT: u32 = 0x0043_5d40;
const FRIEND_HITS_GET_HIT_COUNT: u32 = 0x0043_5e20;
/// A linked-list membership test: `this` is a list node, the stack word the
/// address of an item; true when a node's item (the first word) equals the
/// word there.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// Adds the item at the address on the stack at the head of the list at
/// `this` (`BSSimpleList::AddHead`).
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;

// ---------------------------------------------------------------------------
// Data

/// The `double` `0.0` the float setters compare with.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The player character singleton pointer (`011dea3c`).
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;

/// Vtables set by the constructors of this file.
const VTABLE_EXTRA_RADIUS: u32 = 0x0101_5208;
const VTABLE_EXTRA_RADIATION: u32 = 0x0101_5214;

/// Extra data types, `EXTRA_DATA_TYPE` of the Xbox PDB.
const EXTRA_SEEN_DATA: u8 = 0x05;
const EXTRA_CELL_DETACH_TIME: u8 = 0x0b;
const EXTRA_FOLLOWER: u8 = 0x1d;
const EXTRA_LEV_CREA_MOD: u8 = 0x1e;
const EXTRA_LEVELED_CREATURE: u8 = 0x2e;
const EXTRA_MERCHANT_CONTAINER: u8 = 0x3c;
const EXTRA_NORTH_ROTATION: u8 = 0x43;
const EXTRA_X_TARGET: u8 = 0x44;
const EXTRA_FRIEND_HITS: u8 = 0x45;
const EXTRA_NO_RUMORS: u8 = 0x4e;
const EXTRA_RADIUS: u8 = 0x5c;
const EXTRA_RADIATION: u8 = 0x5d;
const EXTRA_MULTIBOUND: u8 = 0x61;
const EXTRA_MULTIBOUND_DATA: u8 = 0x62;
const EXTRA_MULTIBOUND_REF: u8 = 0x63;
const EXTRA_EMITTANCE_SOURCE: u8 = 0x67;
const EXTRA_OCCLUSION_PLANE: u8 = 0x71;
const EXTRA_ENCOUNTER_ZONE: u8 = 0x74;
const EXTRA_SAVED_HAVOK_DATA: u8 = 0x3d;
const EXTRA_LASTFINISHEDSEQUENCE: u8 = 0x41;
const EXTRA_SAVED_ANIMATION: u8 = 0x42;
const EXTRA_REFRACTION_PROPERTY: u8 = 0x48;

// ---------------------------------------------------------------------------
// Second block: callees and data

/// `BaseExtraList::RemoveExtra(extra, destroy)` (Xbox PDB): removes the given
/// extra data from the list, deleting it when `destroy` is set.
const REMOVE_EXTRA_OBJECT: u32 = 0x0041_0020;
/// Removes from the list at `this` the first node whose item equals the word
/// at the address given (`BSSimpleList::remove`).
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// The pointer to the save-load object (`TESSaveLoadGame`, `011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `MOV AL,0` on the save-load object (false in the PC build); the code asks
/// it before the parts it keeps only for the Xbox.
const SAVE_LOAD_UNAVAILABLE: u32 = 0x0047_c850;
/// Constructors of extra data (the block in ECX, the argument on the stack).
/// `ExtraRefractionProperty` takes a `float`.
const REFRACTION_PROPERTY_INIT: u32 = 0x0043_5f20;
/// Takes a string, which it copies into a block of its own.
const LAST_FINISHED_SEQUENCE_INIT: u32 = 0x0043_51d0;
const SAVED_ANIMATION_INIT: u32 = 0x0043_5a60;
const SAVED_HAVOK_DATA_INIT: u32 = 0x0043_5b50;
/// `SaveGameWarning(format, ...)` (Xbox PDB, cdecl).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `ExtraSavedAnimation::DeleteBuffer` (Xbox PDB; the body also serves the
/// saved havok data): frees the buffer the extra data holds.
const DELETE_SAVED_BUFFER: u32 = 0x0043_5c10;
/// `MOV [ECX],value`, returning ECX: the compiler's `std::locale::id::id`
/// name on a one-word holder.
const STORE_VALUE: u32 = 0x008c_71b0;
/// `BGSLoadGameSubBuffer::LoadAnimation` and `LoadHavokData` (Xbox PDB).
const LOAD_ANIMATION: u32 = 0x0086_4b00;
const LOAD_HAVOK_DATA: u32 = 0x0086_4d80;
const MESSAGE_SAVED_ANIMATION_EXISTS: u32 = 0x0101_5220;
const MESSAGE_SAVED_HAVOK_DATA_EXISTS: u32 = 0x0101_5278;

// ---------------------------------------------------------------------------
// Eighth block: the loader of the buffered save format

/// `BGSLoadGameBuffer` (Xbox PDB) methods, `this` the buffer: load `size`
/// bytes to an address (`00864980(address, size)`), a form id (`LoadFormID`,
/// `008648a0`, gives the id), a form id stored where the address points
/// (`LoadFormID_ov2`, `008648e0(address)`), a string to a buffer
/// (`008649a0(characters)`) and a counted value's count
/// (`LoadVariableSizedValue`, `00864a60`).
const LOAD_BYTES: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48a0;
const LOAD_FORM_ID_OV2: u32 = 0x0086_48e0;
const LOAD_STRING: u32 = 0x0086_49a0;
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `BaseExtraList::RemoveExtra(extra, destroy)`'s sibling by type is
/// `REMOVE_EXTRA`; `ExtraDataList::SetWorn` and `SetCanNotWear` (Xbox PDB).
const SET_WORN: u32 = 0x0041_aa20;
const SET_CAN_NOT_WEAR: u32 = 0x0041_ab70;
/// The destructible object form functions the type `0x56` extra data calls:
/// `00477bc0(reference, float)` finds what to apply and
/// `BGSDestructibleObjectForm::SetSelfDamage` (Xbox PDB) applies it.
const DESTRUCTIBLE_LOOKUP_00477BC0: u32 = 0x0047_7bc0;
const SET_SELF_DAMAGE: u32 = 0x0047_7ce0;
/// `ScriptLocals` (0x14 bytes): the constructor (`005a8b80`), `LoadGame`
/// (Xbox PDB) and the destructor (`ScriptLocals::~ScriptLocals`), and the
/// locals a script builds (`005abf60`).
const SCRIPT_LOCALS_INIT: u32 = 0x005a_8b80;
const SCRIPT_LOCALS_LOAD_GAME: u32 = 0x005a_9f20;
const SCRIPT_LOCALS_DESTROY: u32 = 0x005a_8bc0;
const SCRIPT_CREATE_LOCALS: u32 = 0x005a_bf60;
/// `0042ce00()` gives a word that `BGSMenuPacker::RecomputePacking`
/// (Xbox PDB, `007037c0`, `this` the buffer) takes before and after the
/// script locals load.
const MENU_PACKING_VALUE: u32 = 0x0042_ce00;
const MENU_PACKER_RECOMPUTE: u32 = 0x0070_37c0;
/// `0x5B` model swap: `004759a0(form, index)` gives the swapped form.
const MODEL_SWAP_FROM_INDEX_004759A0: u32 = 0x0047_59a0;
/// `0x73`/`0x75`/`0x5E` ...: the classes the loaders cast the forms to.
const RTTI_TES_GLOBAL: u32 = 0x0118_3ad4;
const RTTI_TES_SCRIPT: u32 = 0x0118_46bc;
const RTTI_TYPE_73_FORM: u32 = 0x0118_4738;
const RTTI_TYPE_75_FORM: u32 = 0x0118_4720;
const RTTI_TYPE_5E_FORM: u32 = 0x0118_4704;
const RTTI_TES_ENCOUNTER_ZONE: u32 = 0x0118_40dc;
const RTTI_TYPE_6E_FORM: u32 = 0x0118_40c4;
const RTTI_TYPE_3F_SOURCE: u32 = 0x0118_3140;
const RTTI_TYPE_3F_TARGET: u32 = 0x0118_30cc;
const RTTI_TYPE_2E_FORM: u32 = 0x0118_46e8;
const RTTI_TYPE_DISMEMBERED_FORM: u32 = 0x0118_3108;
const RTTI_TYPE_92_EXTRA: u32 = 0x0118_4078;
/// Constructor table of the extra data the loader builds when the list has
/// none: `(type, size, constructor)`. The constructors at `0042c4b0` to
/// `0042c6d0` are in this file (`construct_extra_at`).
const OV2_CONSTRUCTIONS: &[(u8, u32, u32)] = &[
    (0x0d, 0x14, 0x0042_c760),
    (0x18, 0x20, 0x0043_2650),
    (0x19, 0x1c, 0x0043_2810),
    (0x1a, 0x10, 0x0042_c860),
    (0x1b, 0x10, 0x0043_3010),
    (0x1c, 0x10, 0x0042_c830),
    (0x1d, 0x10, 0x0043_0dd0),
    (0x1e, 0x10, 0x0043_5040),
    (0x21, 0x10, 0x0042_c5e0),
    (0x22, 0x10, 0x0042_c610),
    (0x23, 0x10, 0x0042_c640),
    (0x24, 0x10, 0x0042_c670),
    (0x25, 0x10, 0x0042_c6a0),
    (0x26, 0x10, 0x0042_c6d0),
    (0x27, 0x10, 0x0042_c700),
    (0x28, 0x10, 0x0042_c730),
    (0x2a, 0x10, 0x0042_c4b0),
    (0x2b, 0x10, 0x0042_c580),
    (0x2e, 0x14, 0x0043_11f0),
    (0x2f, 0x14, 0x0042_c8c0),
    (0x30, 0x10, 0x0042_c7d0),
    (0x32, 0x24, 0x0082_5a10),
    (0x33, 0x28, 0x0082_5ea0),
    (0x35, 0x10, 0x0043_29d0),
    (0x39, 0x10, 0x0043_5920),
    (0x3c, 0x10, 0x0043_4fc0),
    (0x3f, 0x10, 0x0042_c900),
    (0x45, 0x1c, 0x0043_5c40),
    (0x46, 0x10, 0x0042_c930),
    (0x49, 0x10, 0x0043_08c0),
    (0x4a, 0x10, 0x0042_c800),
    (0x4d, 0x10, 0x0043_24a0),
    (0x4e, 0x10, 0x0042_c990),
    (0x50, 0x10, 0x0041_bef0),
    (0x55, 0x10, 0x0043_6ac0),
    (0x56, 0x10, 0x0042_c9c0),
    (0x5b, 0x14, 0x0042_c9f0),
    (0x5c, 0x10, 0x0042_ca30),
    (0x5d, 0x10, 0x0042_ca60),
    (0x5e, 0x10, 0x0043_6cb0),
    (0x60, 0x10, 0x0042_ca90),
    (0x6c, 0x10, 0x0041_e750),
    (0x6e, 0x14, 0x0042_cba0),
    (0x70, 0x10, 0x0042_cbe0),
    (0x73, 0x10, 0x0043_7440),
    (0x74, 0x10, 0x0043_2e60),
    (0x75, 0x1c, 0x0043_7690),
    (0x7c, 0x1c, 0x0043_0f30),
    (0x89, 0x10, 0x0043_36c0),
    (0x8b, 0x28, 0x0043_7c40),
    (0x8d, 0x10, 0x0042_cc40),
    (0x8f, 0x1c, 0x0042_ccc0),
    (0x92, 0x14, 0x0041_1e40),
];
/// Constructors of the items some arms build: `0042cc10` (12 bytes, the
/// type `0x73` records), `00437c10` (0x24 bytes, the type `0x8B` records),
/// `00430200` (0x30 bytes, `EXTRA_DISMEMBERED_LIMBS`).
const ITEM_73_INIT: u32 = 0x0042_cc10;
const ITEM_8B_INIT: u32 = 0x0043_7c10;
const DISMEMBERED_LIMBS_INIT: u32 = 0x0043_0200;
/// The type `0x35` arm: `009724e0(second, first)` on the object at
/// `011e0e80` gives the item a pair of words names; when there is none a
/// warning is logged with the reference's form id and name (or the default
/// text at `01015890`).
const PAIR_LOOKUP_009724E0: u32 = 0x0097_24e0;
const PAIR_LOOKUP_OBJECT: u32 = 0x011e_0e80;
const NAME_UNKNOWN_REFERENCE: u32 = 0x0101_5890;
const MESSAGE_PAIR_NOT_FOUND: u32 = 0x0101_5838;
/// Arrays of the extra data: set the size of the array at `this` (`0042f5f0`
/// and `006f2290`, `(count, 1)`) and remove an item (`009a4320`, `(index,
/// 1)`).
const ARRAY_RESIZE: u32 = 0x0042_f5f0;
const ARRAY_RESIZE_OTHER: u32 = 0x006f_2290;
const ARRAY_REMOVE_AT: u32 = 0x009a_4320;
/// `CombatTimeStamp::LoadGame` (`009a5ff0`, `this` the hit, the buffer given).
const COMBAT_TIME_STAMP_LOAD_GAME: u32 = 0x009a_5ff0;
/// `008c6ca0(kind)`: builds the object the type `0x70` extra data holds.
const BUILD_OBJECT_OF_KIND: u32 = 0x008c_6ca0;
/// `0042e680(marker data, flags)`: sets the flags byte of the map marker data.
const MAP_MARKER_SET_FLAGS: u32 = 0x0042_e680;
/// The object at `011ddf38` and its method `0084aa30(form, id)` the type
/// `0x1A` arm calls; the package factory `00670b90(type)`.
const HANDLER_OBJECT: u32 = 0x011d_df38;
const OBJECT_MANAGER_REGISTER: u32 = 0x0084_aa30;
const PACKAGE_FACTORY: u32 = 0x0067_0b90;
/// `ActiveEffect` list loader (`00806a90(buffer, list)`).
const ACTIVE_EFFECT_LOAD_LIST: u32 = 0x0080_6a90;
/// `0040a250(form)`: the magic item of a form.
const MAGIC_ITEM_FROM_FORM: u32 = 0x0040_a250;
/// `MenuTopic::LoadGame_ov2`-style loader (`0083fbe0`, `this` the topic) and
/// `DoorTeleportData::LoadGame_ov2` (`0043aaa0`, `this` the data).
const MENU_TOPIC_LOAD_GAME_OV2: u32 = 0x0083_fbe0;
const DOOR_TELEPORT_DATA_LOAD_GAME_OV2: u32 = 0x0043_aaa0;
/// `0066e330(id)`: the handle of an id; `0042f780(this, handle)` assigns the
/// handle at `this`.
const HANDLE_OF_ID: u32 = 0x0066_e330;
const ASSIGN_HANDLE_AT: u32 = 0x0042_f780;
/// The strings of the type `0x8F` extra data (`00438260`, `00438280`).
const EXTRA_SET_FIRST_STRING: u32 = 0x0043_8260;
const EXTRA_SET_SECOND_STRING: u32 = 0x0043_8280;
const LOAD_GAME_OV2_UNKNOWN_TYPE_FORMAT: u32 = 0x0101_57f8;
/// Fields of the load buffer the arms for the types `0x5F` and `0x2E`
/// read and replace: `0042ce30(buffer, out)` copies one to `out`,
/// `0042ce90(buffer)` gives a flag byte, `0086cf00(buffer, value)` and
/// `0042ce50(buffer, flag)` set them.
const BUFFER_FIELD_COPY_42CE30: u32 = 0x0042_ce30;
const BUFFER_FLAG_42CE90: u32 = 0x0042_ce90;
const BUFFER_SET_FIELD_86CF00: u32 = 0x0086_cf00;
const BUFFER_SET_FLAG_42CE50: u32 = 0x0042_ce50;
/// The owner of a loaded extra data (the buffer's virtual `+0x0C`): the
/// notifications `008b6820` and `008b6ae0`, `0042ce10(handler object)`, the
/// refresh `008adc50`, and setting its base `00575690(owner, creature)`.
const OWNER_NOTIFY_BEGIN: u32 = 0x008b_6820;
const OWNER_NOTIFY_END: u32 = 0x008b_6ae0;
const HANDLER_OBJECT_QUERY: u32 = 0x0042_ce10;
const OWNER_REFRESH: u32 = 0x008a_dc50;
const OWNER_SET_BASE: u32 = 0x0057_5690;
/// `00430660(old, new)`: whether two dismembered limbs extra data match.
const DISMEMBERED_LIMBS_COMPARE: u32 = 0x0043_0660;
/// The type `0x2E` arm: whether a form can be leveled (`0047cdb0`), the
/// creature made for a base and a template (`0047d130`), the two creature
/// constructors (`00601170` of 0x20C bytes, `005f7230` of 0x160) and the
/// marker setter (`0047e340`).
const FORM_CAN_BE_LEVELED: u32 = 0x0047_cdb0;
const MAKE_LEVELED_CREATURE: u32 = 0x0047_d130;
const CREATURE_ACTOR_INIT: u32 = 0x0060_1170;
const CREATURE_INIT: u32 = 0x005f_7230;
const CREATURE_SET_MARKER: u32 = 0x0047_e340;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB, cdecl: the reference).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;

/// Seventh block: `0042f570` initializes the member at +4 of the structure
/// `0042c470` builds; `00430ca0` and `00431260` are the destructors of
/// `ExtraLock` and `ExtraTeleport`.
const FLAGS_MEMBER_INIT: u32 = 0x0042_f570;
const EXTRA_LOCK_DESTROY: u32 = 0x0043_0ca0;
const EXTRA_TELEPORT_DESTROY: u32 = 0x0043_1260;
const EXTRA_LOCK: u8 = 0x2a;
const EXTRA_TELEPORT: u8 = 0x2b;
const EXTRA_OWNERSHIP: u8 = 0x21;
const EXTRA_GLOBAL: u8 = 0x22;
const EXTRA_RANK: u8 = 0x23;
const EXTRA_COUNT: u8 = 0x24;
const EXTRA_HEALTH: u8 = 0x25;
const EXTRA_USES: u8 = 0x26;
const VTABLE_EXTRA_LOCK: u32 = 0x0101_589c;
const VTABLE_EXTRA_TELEPORT: u32 = 0x0101_58a8;
const VTABLE_EXTRA_OWNERSHIP: u32 = 0x0101_58b4;
const VTABLE_EXTRA_GLOBAL: u32 = 0x0101_58c0;
const VTABLE_EXTRA_RANK: u32 = 0x0101_58cc;
const VTABLE_EXTRA_COUNT: u32 = 0x0101_58d8;
const VTABLE_EXTRA_HEALTH: u32 = 0x0101_58e4;
const VTABLE_EXTRA_USES: u32 = 0x0101_58f0;

// ---------------------------------------------------------------------------
// Sixth block: the writer of the buffered save format

/// `BGSSaveGameBuffer` (Xbox PDB) methods, `this` the buffer: save `size`
/// bytes at an address (`00865e50(address, size, 0)`), a form id
/// (`SaveFormID_ov2`, `00865df0(form, 0)`; `SaveFormID`, `00865db0(form,
/// 0)`), a string (`SaveString`, `00865e70(string, 0)`), start a value whose
/// size is known at the end (`StartVariableSizedValue`, `00865f20`, gives
/// the position), save that value's size (`SaveVariableSizedValue_ov2`,
/// `00865ff0(count, start)`) and write a count (`SaveVariableSizedValue`,
/// `00865f60(count)`).
const BUFFER_SAVE_BYTES: u32 = 0x0086_5e50;
const BUFFER_SAVE_FORM_ID_OV2: u32 = 0x0086_5df0;
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5db0;
const BUFFER_SAVE_STRING: u32 = 0x0086_5e70;
const BUFFER_START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2: u32 = 0x0086_5ff0;
const BUFFER_SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// The table, indexed by extra data type, of the masks of save kinds that
/// save the type (0 for a type that is not saved); bit `0x40000000` marks a
/// type that is saved only when the buffer's reference is a file form.
const SAVE_KIND_TABLE: u32 = 0x0118_3d30;
/// Type `0x89`, saved only when the buffer's second virtual says so.
const EXTRA_TYPE_CONDITIONAL: u8 = 0x89;
/// `ScriptLocals::SaveGame` (Xbox PDB), `this` the locals, the buffer given.
const SCRIPT_LOCALS_SAVE_GAME: u32 = 0x005a_9db0;
/// `BGSDestructibleObjectForm::GetModelSwapIndex` (Xbox PDB; cdecl, two
/// words), used by the type `0x5B` extra data.
const GET_MODEL_SWAP_INDEX: u32 = 0x0047_5920;
/// `MOV EAX,[ECX+0x28]`: the word at +0x28 of an object, used by the
/// buffered save.
const GET_WORD_28: u32 = 0x0045_cd60;
/// `MagicItem::GetMagicItemFormID` and `MagicTarget::GetMagicTargetFormID`
/// (Xbox PDB).
const GET_MAGIC_ITEM_FORM_ID: u32 = 0x0040_a1e0;
const GET_MAGIC_TARGET_FORM_ID: u32 = 0x0082_54c0;
/// `ExtraFriendHits::RemoveOldHits` (Xbox PDB), run before the hits are
/// saved.
const FRIEND_HITS_REMOVE_OLD_HITS: u32 = 0x0043_5e40;
/// `0044ddc0(this)` on the member at +0x0C/+0x20 of an extra data: the
/// number of items of the array it is (the same body as `GET_NEXT` for a
/// `BSExtraData`), and `006a7ad0(this, index)`: the item at an index.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_ITEM: u32 = 0x006a_7ad0;
/// `CombatTimeStamp::SaveGame` (Xbox PDB) and
/// `ActiveEffect::SaveActiveEffectList` (Xbox PDB, cdecl: the buffer and the
/// list).
const COMBAT_TIME_STAMP_SAVE_GAME: u32 = 0x009a_5f90;
const ACTIVE_EFFECT_SAVE_LIST: u32 = 0x0080_6a10;
/// `MenuTopic::SaveGame` and `DoorTeleportData::SaveGame_ov2` (Xbox PDB).
const MENU_TOPIC_SAVE_GAME_OV2: u32 = 0x0083_fb10;
const DOOR_TELEPORT_DATA_SAVE_GAME_OV2: u32 = 0x0043_aa40;
/// `00428070` and `004280b0` copy the string at +0x0C and +0x14 into a
/// string `this` points at, through `004047f0(this, source)`; `004037d0` is
/// the string destructor.
const STRING_ASSIGN: u32 = 0x0040_47f0;
/// Message of the unknown type.
const SAVE_GAME_OV2_UNKNOWN_TYPE_FORMAT: u32 = 0x0101_57b0;
/// `00825c00(this)` on the object a handle holds (a 17-byte getter, the
/// same body as the stream's position getter).
const HANDLE_TARGET_WORD: u32 = 0x0082_5c00;

// ---------------------------------------------------------------------------
// Fifth block: the loader of the old save format

/// `TESSaveLoadGame::LoadNumericID(buffer, size)` (Xbox PDB): reads a form
/// id of the save (and converts it to the id of the load order).
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// The form header of the form being loaded (`004fd3c0`, on the save-load
/// object): form id at +0, flags at +5, version byte at +9.
const LOADING_FORM_HEADER: u32 = 0x004f_d3c0;
/// The package of the given form id and type byte
/// (`0085a240(save_load, id, type)`: a package of that type is built,
/// `00670b90`, and loaded from the id, virtual `+0x128`).
const LOAD_PACKAGE_BY_ID: u32 = 0x0085_a240;
/// `ExtraDataList::SetPackageExtra` (Xbox PDB).
const SET_PACKAGE_EXTRA: u32 = 0x0041_c930;
/// Adds a package to the run once packages (`0041d700`).
const ADD_RUN_ONCE_PACKAGE: u32 = 0x0041_d700;
/// `ExtraDataList::SetLockPtr` (Xbox PDB).
const SET_LOCK_PTR: u32 = 0x0041_9050;
/// `ExtraDataList::SetScale` (Xbox PDB), and the same on a reference
/// (`00567490`).
const SET_SCALE: u32 = 0x0041_9fb0;
const REFERENCE_SET_SCALE: u32 = 0x0056_7490;
/// `ExtraDataList::SetGhost` (Xbox PDB).
const SET_GHOST: u32 = 0x0041_a960;
/// `DoorTeleportData::LoadGame` (Xbox PDB).
const DOOR_TELEPORT_DATA_LOAD_GAME: u32 = 0x0043_a9a0;
/// `ExtraDataList::GetPersistentCell` and `SetPersistentCell` (Xbox PDB; the
/// getter is unnamed in the map).
const GET_PERSISTENT_CELL: u32 = 0x0041_d460;
const SET_PERSISTENT_CELL: u32 = 0x0041_d390;
/// `TESObjectCELL::RemoveReference` and `AddReference` (Xbox PDB).
const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
const CELL_ADD_REFERENCE: u32 = 0x0054_8230;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB).
const REFERENCE_GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// `MOV EAX,[ECX+0x34]`: the persistent cell of a world space (the map
/// names it `ActorMover::GetPreferredMoveMode`, an identical-code fold). It
/// takes no stack argument, so the words pushed before a call stay for the
/// next call.
const WORLD_SPACE_GET_WORD_34: u32 = 0x005f_36f0;
/// `MOV EAX,[ECX+0x40]`: the parent cell of a reference.
const REFERENCE_GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TEST byte [ECX+0x24],1`: whether a cell is an interior cell.
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// The info general topic (`MenuTopic`, 0x2C bytes): built by `0083da50`,
/// loaded by `MenuTopic::LoadGame` (`0083f680`, given the reference), checked
/// by `009611e0`, deleted by the scalar deleting destructor `00425ff0`.
const MENU_TOPIC_INIT: u32 = 0x0083_da50;
const MENU_TOPIC_LOAD_GAME: u32 = 0x0083_f680;
const MENU_TOPIC_CHECK: u32 = 0x0096_11e0;
const MENU_TOPIC_DELETE: u32 = 0x0042_5ff0;
/// `ExtraDataList::SetInfoGeneralTopic` (Xbox PDB).
const SET_INFO_GENERAL_TOPIC: u32 = 0x0042_dee0;
/// Setters the loader calls for the single ids: the item dropper
/// (`00419dc0`), the entry of the dropped item list (`0041de40`) and the
/// whole list's item (`0041e000`).
const SET_ITEM_DROPPER: u32 = 0x0041_9dc0;
const ADD_DROPPED_ITEM: u32 = 0x0041_de40;
const ADD_DROPPED_ITEM_LIST_ENTRY: u32 = 0x0041_e000;
/// The table of 0xF5 entries of 0x24 bytes at `011977d8` whose first word is
/// the name of a sequence (the old saves store its index).
const SEQUENCE_NAME_TABLE: u32 = 0x0119_77d8;
/// Size of the buffer the old saves read a sequence name into.
const TEXT_SEQUENCE_BUFFER_SIZE: u32 = 0x104;
/// `RTTI` of the classes the loader casts to.
const RTTI_TES_WORLD_SPACE: u32 = 0x0118_3fd0;
const RTTI_TES_OBJECT_CELL: u32 = 0x0118_3fb4;
const RTTI_TES_PACKAGE: u32 = 0x0118_46a0;
const RTTI_ACTOR: u32 = 0x0118_46d4;
/// Messages of `LoadGame`: the block tag missing (with the form header and
/// without), an unknown extra data type, and the block size mismatches (with
/// the form header, for too much and too little read, and without).
const LOAD_GAME_BAD_TAG_FORMAT: u32 = 0x0101_5718;
const LOAD_GAME_BAD_TAG_SHORT_FORMAT: u32 = 0x0101_56a8;
const LOAD_GAME_UNKNOWN_TYPE_FORMAT: u32 = 0x0101_5610;
const LOAD_GAME_OVERREAD_FORMAT: u32 = 0x0101_5588;
const LOAD_GAME_UNDERREAD_FORMAT: u32 = 0x0101_5500;
const LOAD_GAME_OVERREAD_SHORT_FORMAT: u32 = 0x0101_54a0;
const LOAD_GAME_UNDERREAD_SHORT_FORMAT: u32 = 0x0101_5440;
/// Initializes the 0x14-byte lock record (`00411b00`, the constructor of the
/// structure behind `ExtraLock`).
const LOCK_RECORD_INIT: u32 = 0x0041_1b00;

/// `TESSaveLoadGame::SaveNumericID(buffer, size)` (Xbox PDB): writes a
/// form id.
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `DoorTeleportData::SaveGame` (Xbox PDB), `this` the teleport data.
const DOOR_TELEPORT_DATA_SAVE_GAME: u32 = 0x0043_a930;
/// Writes the info general topic data (`0083f3e0`, `this` the data).
const INFO_GENERAL_TOPIC_SAVE: u32 = 0x0083_f3e0;
/// The type byte of a package (`0041ca90`, the signed byte at +0x20).
const PACKAGE_GET_TYPE: u32 = 0x0041_ca90;
/// The first word of the save block: the four bytes `KOLB`.
const SAVE_BLOCK_TAG: u32 = 0x424c_4f4b;
/// Messages of `SaveGame`: the logged size with a world space and without,
/// the block that does not fit 16 bits, and the package start location
/// without a location (form id and name).
const SAVE_GAME_SIZE_FORMAT: u32 = 0x0101_53a0;
const SAVE_GAME_SIZE_SHORT_FORMAT: u32 = 0x0101_536c;
const SAVE_BLOCK_TOO_LARGE_MESSAGE: u32 = 0x0101_5318;
const PACKAGE_START_LOCATION_NULL_MESSAGE: u32 = 0x0101_53f0;
/// `RTTI` of the extra data `SaveGame` also casts to.
const RTTI_EXTRA_PERSISTENT_CELL: u32 = 0x0118_4594;
const RTTI_EXTRA_ITEM_DROPPER: u32 = 0x0118_4574;
const RTTI_EXTRA_HEAD_TRACK_TARGET: u32 = 0x0118_4550;
const RTTI_EXTRA_NO_RUMORS: u32 = 0x0118_4530;

// ---------------------------------------------------------------------------
// Third block: the save size, the save writer and the loader

/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB), on the save-load object.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// The save version byte of the save-load object (`008df040`).
const SAVE_VERSION: u32 = 0x008d_f040;
/// The save-load stream: write `size` bytes from a buffer (`008579b0`), read
/// `size` bytes into one (`008579e0`), the current position (`00825c00`).
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
const SAVE_POSITION: u32 = 0x0082_5c00;
/// The object whose byte `00408d60` finds: the debug switch that makes the
/// save functions log what they wrote.
const DEBUG_SWITCHES_OBJECT: u32 = 0x011d_e4e8;
/// `00408d60(object)`: the address of the object's byte value.
const SETTING_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// The form header of the form being saved (`004fd3e0`, on the save-load
/// object): form id at +0, flags at +5.
const SAVING_FORM_HEADER: u32 = 0x004f_d3e0;
/// `Error(format, ...)` (Xbox PDB, cdecl).
const ERROR_LOG: u32 = 0x0040_fbe0;
/// `00469860(id)` on the data handler: true when the form id is below
/// `0xFF000000` (a form of a data file, not one created in play).
const FORM_ID_IS_FILE_FORM: u32 = 0x0046_9860;
/// The name of the source file the messages quote.
const SOURCE_FILE_NAME: u32 = 0x0101_52d0;
/// `TESForm::GetEditorID`-like name slot of a form (`+0x130`).
const FORM_NAME_SLOT: u32 = 0x130;
/// The type descriptors: `BSExtraData`, the source of every cast below, and
/// the extra data classes they are cast to.
const RTTI_BS_EXTRA_DATA: u32 = 0x0118_3b2c;
const RTTI_EXTRA_PACKAGE_START_LOCATION: u32 = 0x0118_448c;
const RTTI_EXTRA_PACKAGE: u32 = 0x0118_44f8;
const RTTI_EXTRA_RUN_ONCE_PACKS: u32 = 0x0118_44b4;
const RTTI_EXTRA_FOLLOWER: u32 = 0x0118_4470;
const RTTI_EXTRA_TELEPORT: u32 = 0x0118_4430;
const RTTI_EXTRA_LAST_FINISHED_SEQUENCE: u32 = 0x0118_4408;
const RTTI_EXTRA_INFO_GENERAL_TOPIC: u32 = 0x0118_43e4;
/// Messages of `GetSaveSize`: with a world space (the form id, its name and
/// its flags) and without.
const GET_SAVE_SIZE_FORMAT: u32 = 0x0101_2cb0;
const GET_SAVE_SIZE_SHORT_FORMAT: u32 = 0x0101_2c78;
/// Size functions of the data some extra data carry.
const TELEPORT_SAVE_SIZE: u32 = 0x0043_a8f0;
const STRING_LENGTH_OF: u32 = 0x0044_a670;
const INFO_GENERAL_TOPIC_SAVE_SIZE: u32 = 0x0083_f280;

// ---------------------------------------------------------------------------
// Helpers

/// `GetExtraData`: the first extra data of `extra_type`, or null.
fn find_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> Ptr<BSExtraData> {
    e.call(GET_EXTRA_DATA, &args![list, extra_type as u32])
        .ptr()
}

/// `AddExtra`.
fn add_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    e.call(ADD_EXTRA, &args![list, extra]);
}

/// `RemoveExtra(type)`.
fn remove_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) {
    e.call(REMOVE_EXTRA, &args![list, extra_type as u32]);
}

/// The word at +0x0C of the first extra data of `extra_type`, or `default`
/// when the list has none.
fn extra_word_or(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8, default: u32) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        default
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    }
}

/// A `float` as the x87 stack stores it.
fn x87_float(value: f32) -> u32 {
    x87_quiet_bits(value.to_bits())
}

/// The `float` at +0x0C of the first extra data of `extra_type`, or 0.0
/// (`FLDZ`); returned in ST0.
fn extra_float_or_zero(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> f32 {
    let bits = extra_word_or(e, list, extra_type, 0);
    f32::from_bits(x87_quiet_bits(bits))
}

/// `new` and construct: allocates `size` bytes and runs the constructor at
/// `construct` on the block (it returns the object), or gives null when the
/// allocation failed.
fn new_extra(e: &mut Engine, size: u32, construct: u32) -> Ptr<BSExtraData> {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        e.call(construct, &args![block]).ptr()
    }
}

/// [`new_extra`] for a constructor that takes one stack word, `argument`.
fn new_extra_with(e: &mut Engine, size: u32, construct: u32, argument: u32) -> Ptr<BSExtraData> {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        e.call(construct, &args![block, argument]).ptr()
    }
}

/// The body of the word setters: `value == remove_value` removes the extra
/// data of `extra_type`; otherwise `release_old` runs on the word the
/// existing extra data holds before the new value replaces it, or a new
/// 0x10-byte extra data is built with `construct`, given the value and added.
fn set_word_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    value: u32,
    remove_value: u32,
    construct: u32,
    release_old: impl FnOnce(&mut Engine, u32),
) {
    if value == remove_value {
        remove_extra(e, list, extra_type);
        return;
    }
    let existing = find_extra(e, list, extra_type);
    if existing.is_null() {
        let extra = new_extra(e, 0x10, construct);
        e.mem.set_u32(extra.addr().wrapping_add(0x0c), value);
        add_extra(e, list, extra);
    } else {
        let old = e.mem.u32(existing.addr() + 0x0c);
        release_old(e, old);
        e.mem.set_u32(existing.addr() + 0x0c, value);
    }
}

/// The body of the handle setters: a zero `value` removes the extra data of
/// `extra_type`; otherwise the handle at +0x0C of the existing extra data, or
/// of a new 0x10-byte one built with `construct` (added afterwards), is
/// assigned `value` (`0066b0d0`).
fn set_handle_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    value: u32,
    construct: u32,
) {
    if value == 0 {
        remove_extra(e, list, extra_type);
        return;
    }
    let existing = find_extra(e, list, extra_type);
    if existing.is_null() {
        let extra = new_extra(e, 0x10, construct);
        e.call(
            ASSIGN_HANDLE,
            &args![extra.addr().wrapping_add(0x0c), value],
        );
        add_extra(e, list, extra);
    } else {
        e.call(ASSIGN_HANDLE, &args![existing.addr() + 0x0c, value]);
    }
}

/// Whether the `float` setters take `value` as the neutral value: `FCOMP`
/// against the `double` at [`ZERO_DOUBLE`] (a NaN compares unordered, so it
/// is stored).
fn float_is_zero(e: &Engine, value: f32) -> bool {
    value as f64 == e.global::<f64>(ZERO_DOUBLE)
}

// ---------------------------------------------------------------------------
// Translations

// Translated from 00421400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetMerchantContainer` (Xbox PDB): the word at +0x0C of the
/// type `0x3C` extra data (`EXTRA_MERCHANTCONTAINER`), or 0.
pub fn extra_data_list_get_merchant_container(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_MERCHANT_CONTAINER, 0)
}

// Translated from 00421430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Merchant container setter (type `0x3C`, `EXTRA_MERCHANTCONTAINER`): a null
/// `value` removes the extra data; otherwise it is stored in the existing
/// one or in a new 0x10-byte one.
pub fn fn_00421430(e: &mut Engine, this: Ptr<ExtraDataList>, value: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_MERCHANT_CONTAINER,
        value,
        0,
        MERCHANT_CONTAINER_INIT,
        |_, _| {},
    );
}

// Translated from 004214f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the leveled creature modifier (type `0x1E`, `EXTRA_LEVCREA_MOD`):
/// stores `1.0` in `out_float` and 0 in `out_word`, then, when the list has
/// the extra data, replaces them with its float (`004350c0`, ST0) and its
/// word (`00435100`, EAX).
pub fn fn_004214f0(e: &mut Engine, this: Ptr<ExtraDataList>, out_float: Ptr, out_word: Ptr) {
    e.mem.set_f32(out_float.addr(), 1.0);
    e.mem.set_u32(out_word.addr(), 0);
    let extra = find_extra(e, this, EXTRA_LEV_CREA_MOD);
    if !extra.is_null() {
        let scale = e.call(LEV_CREA_MOD_GET_FLOAT, &args![extra]).f32();
        e.mem.set_f32(out_float.addr(), scale);
        let word = e.call(LEV_CREA_MOD_GET_WORD, &args![extra]).u32();
        e.mem.set_u32(out_word.addr(), word);
    }
}

// Translated from 00421540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Leveled creature modifier setter (type `0x1E`, `EXTRA_LEVCREA_MOD`): the
/// value 4 removes the extra data; any other value is stored in the existing
/// one or in a new 0x10-byte one.
pub fn fn_00421540(e: &mut Engine, this: Ptr<ExtraDataList>, value: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_LEV_CREA_MOD,
        value,
        4,
        LEV_CREA_MOD_INIT,
        |_, _| {},
    );
}

// Translated from 00421600 (decompiled, FalloutNV.exe 1.4.0.525)
/// "No rumors" flag setter (type `0x4E`, `EXTRA_NO_RUMORS`): stores the byte
/// at +0x0C of the existing extra data, or builds a new 0x10-byte one (its
/// constructor, `00436090`, takes the byte) and adds it. The flag never
/// removes the extra data.
pub fn fn_00421600(e: &mut Engine, this: Ptr<ExtraDataList>, flag: u8) {
    let existing = find_extra(e, this, EXTRA_NO_RUMORS);
    if existing.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let extra: Ptr<BSExtraData> = if block == 0 {
            Ptr::NULL
        } else {
            e.call(NO_RUMORS_INIT, &args![block, flag as u32]).ptr()
        };
        add_extra(e, this, extra);
    } else {
        e.mem.set_u8(existing.addr() + 0x0c, flag);
    }
}

// Translated from 004216b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the type `0x4E` extra data (`EXTRA_NO_RUMORS`).
pub fn fn_004216b0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra(e, this, EXTRA_NO_RUMORS);
}

// Translated from 004216d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the list has a type `0x2E` extra data (`EXTRA_LEVELEDCREATURE`):
/// `BaseExtraList::HasExtra`.
pub fn fn_004216d0(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    e.call(HAS_EXTRA, &args![this, EXTRA_LEVELED_CREATURE as u32])
        .bool()
}

// Translated from 004216f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetLevCreaOriginalBase` (Xbox PDB): the word at +0x0C of
/// the type `0x2E` extra data (`EXTRA_LEVELEDCREATURE`), or 0.
pub fn extra_data_list_get_lev_crea_original_base(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_LEVELED_CREATURE, 0)
}

// Translated from 00421720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x10 of the type `0x2E` extra data (`EXTRA_LEVELEDCREATURE`),
/// or 0.
pub fn fn_00421720(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_LEVELED_CREATURE);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x10)
    }
}

// Translated from 00421750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Leveled creature setter (type `0x2E`, `EXTRA_LEVELEDCREATURE`): when
/// either word is 0 the extra data is removed; otherwise the words go to +0x0C
/// and +0x10 of the existing extra data, or of a new 0x14-byte one
/// (constructor `004311f0`) that is added first.
pub fn fn_00421750(e: &mut Engine, this: Ptr<ExtraDataList>, first: u32, second: u32) {
    if first == 0 || second == 0 {
        remove_extra(e, this, EXTRA_LEVELED_CREATURE);
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_LEVELED_CREATURE);
    if extra.is_null() {
        extra = new_extra(e, 0x14, LEVELED_CREATURE_INIT);
        add_extra(e, this, extra);
    }
    e.mem.set_u32(extra.addr().wrapping_add(0x0c), first);
    e.mem.set_u32(extra.addr().wrapping_add(0x10), second);
}

// Translated from 00421820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetDetachTime` (Xbox PDB): the word at +0x0C of the type
/// `0x0B` extra data (`EXTRA_CELLDETACHTIME`), or 0.
pub fn extra_data_list_get_detach_time(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_CELL_DETACH_TIME, 0)
}

// Translated from 00421850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cell detach time setter (type `0x0B`, `EXTRA_CELLDETACHTIME`): a zero
/// `time` removes the extra data; otherwise it is stored in the existing one
/// or in a new 0x10-byte one (constructor `0040f6b0`).
pub fn fn_00421850(e: &mut Engine, this: Ptr<ExtraDataList>, time: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_CELL_DETACH_TIME,
        time,
        0,
        DETACH_TIME_INIT,
        |_, _| {},
    );
}

// Translated from 00421910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `5` extra data (`EXTRA_SEENDATA`), or 0.
pub fn fn_00421910(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_SEEN_DATA, 0)
}

// Translated from 00421940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetSeenData` (Xbox PDB), type `5` (`EXTRA_SEENDATA`): a
/// null `data` removes the extra data; otherwise it is stored in the
/// existing extra data, after the object the old pointer holds is deleted
/// (virtual slot 0 with 1), or in a new 0x10-byte one (constructor
/// `0040f590`). The compiler's exception-unwinding frame is not translated.
pub fn extra_data_list_set_seen_data(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_SEEN_DATA,
        data,
        0,
        SEEN_DATA_INIT,
        |e, old| {
            if old != 0 {
                e.vcall(old, 0, &args![1u32]);
            }
        },
    );
}

// Translated from 00421a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x0C of the type `0x43` extra data
/// (`EXTRA_NORTHROTATION`), or 0.0. Returned in ST0.
pub fn fn_00421a40(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    extra_float_or_zero(e, this, EXTRA_NORTH_ROTATION)
}

// Translated from 00421a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetNorthRotation` (Xbox PDB), type `0x43`
/// (`EXTRA_NORTHROTATION`): 0.0 removes the extra data; otherwise the angle
/// is stored in the existing one or in a new 0x10-byte one (constructor
/// `0040f680`).
pub fn extra_data_list_set_north_rotation(e: &mut Engine, this: Ptr<ExtraDataList>, angle: f32) {
    if float_is_zero(e, angle) {
        remove_extra(e, this, EXTRA_NORTH_ROTATION);
        return;
    }
    let existing = find_extra(e, this, EXTRA_NORTH_ROTATION);
    if existing.is_null() {
        let extra = new_extra(e, 0x10, NORTH_ROTATION_INIT);
        e.mem
            .set_u32(extra.addr().wrapping_add(0x0c), x87_float(angle));
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(existing.addr() + 0x0c, x87_float(angle));
    }
}

// Translated from 00421b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetXTarget` (Xbox PDB): the word at +0x0C of the type
/// `0x44` extra data (`EXTRA_XTARGET`), or 0.
pub fn extra_data_list_get_x_target(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_X_TARGET, 0)
}

// Translated from 00421b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// X target setter (type `0x44`, `EXTRA_XTARGET`): a zero `target` removes
/// the extra data; otherwise it is stored in the existing one or in a new
/// 0x10-byte one (constructor `00435370`).
pub fn fn_00421b70(e: &mut Engine, this: Ptr<ExtraDataList>, target: u32) {
    set_word_extra(e, this, EXTRA_X_TARGET, target, 0, X_TARGET_INIT, |_, _| {});
}

// Translated from 00421c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x74` extra data (`EXTRA_ENCOUNTERZONE`),
/// or 0.
pub fn fn_00421c30(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_ENCOUNTER_ZONE, 0)
}

// Translated from 00421c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetEncounterZone` (Xbox PDB), type `0x74`
/// (`EXTRA_ENCOUNTERZONE`): a zero `zone` removes the extra data; otherwise
/// it is stored in the existing one or in a new 0x10-byte one
/// (`ExtraEncounterZone::ExtraEncounterZone`, `00432e60`).
pub fn extra_data_list_set_encounter_zone(e: &mut Engine, this: Ptr<ExtraDataList>, zone: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_ENCOUNTER_ZONE,
        zone,
        0,
        ENCOUNTER_ZONE_INIT,
        |_, _| {},
    );
}

// Translated from 00421d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x67` extra data (`EXTRA_EMITTANCE_SOURCE`),
/// or 0.
pub fn fn_00421d20(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_EMITTANCE_SOURCE, 0)
}

// Translated from 00421d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Emittance source setter (type `0x67`, `EXTRA_EMITTANCE_SOURCE`): a zero
/// `source` removes the extra data; otherwise it is stored in the existing one
/// or in a new 0x10-byte one (constructor `00435440`).
pub fn fn_00421d50(e: &mut Engine, this: Ptr<ExtraDataList>, source: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_EMITTANCE_SOURCE,
        source,
        0,
        EMITTANCE_SOURCE_INIT,
        |_, _| {},
    );
}

// Translated from 00421e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x63` extra data (`EXTRA_MULTIBOUND_REF`),
/// or 0.
pub fn fn_00421e10(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_MULTIBOUND_REF, 0)
}

// Translated from 00421e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multibound reference setter (type `0x63`, `EXTRA_MULTIBOUND_REF`): a zero
/// `reference` removes the extra data; otherwise it is stored in the existing
/// one or in a new 0x10-byte one (constructor `00435510`).
pub fn fn_00421e40(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_MULTIBOUND_REF,
        reference,
        0,
        MULTIBOUND_REF_INIT,
        |_, _| {},
    );
}

// Translated from 00421f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x62` extra data (`EXTRA_MULTIBOUND_DATA`),
/// or 0.
pub fn fn_00421f00(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_MULTIBOUND_DATA, 0)
}

// Translated from 00421f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multibound data setter (type `0x62`, `EXTRA_MULTIBOUND_DATA`): a zero
/// `data` removes the extra data; otherwise it is stored in the existing one,
/// after the block its old pointer holds is freed (`operator delete`, when
/// not null), or in a new 0x10-byte one (constructor `00435590`).
pub fn fn_00421f30(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_MULTIBOUND_DATA,
        data,
        0,
        MULTIBOUND_DATA_INIT,
        |e, old| {
            if old != 0 {
                e.call(OPERATOR_DELETE, &args![old]);
            }
        },
    );
}

// Translated from 00422020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetMultiBound` (Xbox PDB): the word the handle at +0x0C of
/// the type `0x61` extra data (`EXTRA_MULTIBOUND`) points at (`00559450`), or 0.
pub fn extra_data_list_get_multi_bound(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_MULTIBOUND);
    if extra.is_null() {
        0
    } else {
        e.call(READ_WORD, &args![extra.addr() + 0x0c]).u32()
    }
}

// Translated from 00422050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetMultiBound` (Xbox PDB), type `0x61`
/// (`EXTRA_MULTIBOUND`): a zero `value` removes the extra data; otherwise the
/// handle at its +0x0C is assigned (`0066b0d0`); a new extra data
/// (constructor `00435690`) gets the handle assigned before it is added.
pub fn extra_data_list_set_multi_bound(e: &mut Engine, this: Ptr<ExtraDataList>, value: u32) {
    set_handle_extra(e, this, EXTRA_MULTIBOUND, value, MULTIBOUND_INIT);
}

// Translated from 00422120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word the handle at +0x0C of the type `0x71` extra data
/// (`EXTRA_OCCLUSION_PLANE`) points at (`00559450`), or 0.
pub fn fn_00422120(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_OCCLUSION_PLANE);
    if extra.is_null() {
        0
    } else {
        e.call(READ_WORD, &args![extra.addr() + 0x0c]).u32()
    }
}

// Translated from 00422150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetOcclusionPlane` (Xbox PDB), type `0x71`
/// (`EXTRA_OCCLUSION_PLANE`): as `SetMultiBound`, with the constructor
/// `004357a0`.
pub fn extra_data_list_set_occlusion_plane(e: &mut Engine, this: Ptr<ExtraDataList>, value: u32) {
    set_handle_extra(e, this, EXTRA_OCCLUSION_PLANE, value, OCCLUSION_PLANE_INIT);
}

// Translated from 00422220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetRadius` (Xbox PDB), type `0x5C` (`EXTRA_RADIUS`): 0.0
/// removes the extra data; otherwise the radius is stored in the existing one
/// or a new 0x10-byte one (built with the radius, `004222f0`) is added.
pub fn extra_data_list_set_radius(e: &mut Engine, this: Ptr<ExtraDataList>, radius: f32) {
    if float_is_zero(e, radius) {
        remove_extra(e, this, EXTRA_RADIUS);
        return;
    }
    let existing = find_extra(e, this, EXTRA_RADIUS);
    if existing.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let extra: Ptr<BSExtraData> = if block == 0 {
            Ptr::NULL
        } else {
            fn_004222f0(e, Ptr::new(block), radius).cast()
        };
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(existing.addr() + 0x0c, x87_float(radius));
    }
}

// Translated from 004222f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraRadius` (RTTI name; type `0x5C`): the `BSExtraData`
/// base with the type, the vtable at `01015208` and the radius at +0x0C.
/// Returns `this`.
pub fn fn_004222f0(e: &mut Engine, this: Ptr, radius: f32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_RADIUS as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_RADIUS);
    e.mem.set_u32(this.addr() + 0x0c, x87_float(radius));
    this
}

// Translated from 00422320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRadius` (Xbox PDB): the `float` at +0x0C of the type
/// `0x5C` extra data (`EXTRA_RADIUS`), or 0.0. Returned in ST0.
pub fn extra_data_list_get_radius(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    extra_float_or_zero(e, this, EXTRA_RADIUS)
}

// Translated from 00422350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Radiation setter (type `0x5D`, `EXTRA_RADIATION`): 0.0 removes the extra
/// data; otherwise the value is stored in the existing one or a new 0x10-byte
/// one (built with the value, `00422420`) is added.
pub fn fn_00422350(e: &mut Engine, this: Ptr<ExtraDataList>, radiation: f32) {
    if float_is_zero(e, radiation) {
        remove_extra(e, this, EXTRA_RADIATION);
        return;
    }
    let existing = find_extra(e, this, EXTRA_RADIATION);
    if existing.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let extra: Ptr<BSExtraData> = if block == 0 {
            Ptr::NULL
        } else {
            fn_00422420(e, Ptr::new(block), radiation).cast()
        };
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(existing.addr() + 0x0c, x87_float(radiation));
    }
}

// Translated from 00422420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraRadiation` (RTTI name; type `0x5D`): the
/// `BSExtraData` base with the type, the vtable at `01015214` and the value at
/// +0x0C. Returns `this`.
pub fn fn_00422420(e: &mut Engine, this: Ptr, radiation: f32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_RADIATION as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_RADIATION);
    e.mem.set_u32(this.addr() + 0x0c, x87_float(radiation));
    this
}

// Translated from 00422450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x0C of the type `0x5D` extra data (`EXTRA_RADIATION`), or
/// 0.0. Returned in ST0.
pub fn fn_00422450(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    extra_float_or_zero(e, this, EXTRA_RADIATION)
}

// Translated from 00422480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddFollower` (Xbox PDB): unless `follower` is the player
/// character (the pointer at `011dea3c`), makes sure the list has a type
/// `0x1D` extra data (`EXTRA_FOLLOWER`; a new one is built by
/// `ExtraFollower::ExtraFollower`, `00430dd0`, 0x10 bytes) and adds
/// `follower` at the head of the list its +0x0C points at, unless that list
/// already holds it (`005f65d0`, `005ae3d0`, both given the address of a
/// stack slot holding `follower`).
pub fn extra_data_list_add_follower(e: &mut Engine, this: Ptr<ExtraDataList>, follower: Ptr) {
    if follower.addr() == e.global::<u32>(PLAYER_SINGLETON) {
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_FOLLOWER);
    if extra.is_null() {
        extra = new_extra(e, 0x10, FOLLOWER_INIT);
        add_extra(e, this, extra);
    }
    let followers = e.mem.u32(extra.addr() + 0x0c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), follower.addr());
        let known = e.call(LIST_CONTAINS, &args![followers, slot]).bool();
        if !known {
            e.call(LIST_ADD_HEAD, &args![followers, slot]);
        }
    });
}

// Translated from 00422550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the follower list of the type `0x1D` extra data
/// (`EXTRA_FOLLOWER`) holds `follower` (`005f65d0`, given the address of a
/// stack slot holding it); false when the list has no such extra data.
pub fn fn_00422550(e: &mut Engine, this: Ptr<ExtraDataList>, follower: Ptr) -> bool {
    let extra = find_extra(e, this, EXTRA_FOLLOWER);
    if extra.is_null() {
        return false;
    }
    let followers = e.mem.u32(extra.addr() + 0x0c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), follower.addr());
        e.call(LIST_CONTAINS, &args![followers, slot]).bool()
    })
}

// Translated from 00422590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddFriendHit` (Xbox PDB): makes sure the list has a type
/// `0x45` extra data (`EXTRA_FRIEND_HITS`; a new 0x1C-byte one is built by
/// `ExtraFriendHits::ExtraFriendHits`, `00435c40`), then calls
/// `ExtraFriendHits::AddHit` and `ExtraFriendHits::GetHitCount` on it, whose
/// count is the answer: the exe ends on that call and keeps its `EAX`
/// (`Actor` `008987f0` stores it).
pub fn extra_data_list_add_friend_hit(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let mut extra = find_extra(e, this, EXTRA_FRIEND_HITS);
    if extra.is_null() {
        extra = new_extra(e, 0x1c, FRIEND_HITS_INIT);
        add_extra(e, this, extra);
    }
    e.call(FRIEND_HITS_ADD_HIT, &args![extra]);
    e.call(FRIEND_HITS_GET_HIT_COUNT, &args![extra]).u32()
}

// Translated from 00422640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetFriendHitCount` (Xbox PDB): `ExtraFriendHits::GetHitCount`
/// of the type `0x45` extra data (`EXTRA_FRIEND_HITS`), or 0.
pub fn extra_data_list_get_friend_hit_count(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_FRIEND_HITS);
    if extra.is_null() {
        0
    } else {
        e.call(FRIEND_HITS_GET_HIT_COUNT, &args![extra]).u32()
    }
}

// ---------------------------------------------------------------------------
// Second block: the follower and refraction extra data, the last finished
// sequence, the saved animation and havok data

// Translated from 00422670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the type `0x45` extra data (`EXTRA_FRIEND_HITS`) from the list
/// (`BaseExtraList::RemoveExtra` by type).
pub fn fn_00422670(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra(e, this, EXTRA_FRIEND_HITS);
}

// Translated from 00422690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveFollower` (Xbox PDB): when the list has a type
/// `0x1D` extra data (`EXTRA_FOLLOWER`), removes `follower` from the list its
/// +0x0C points at (`00905330`, given the address of a stack slot holding
/// it); then asks the save-load object at `011de45c` (`0047c850`, false in
/// this build) and, when it says no, reads `bClearingData` of the data
/// handler ([`fn_004226e0`]); neither answer is used.
pub fn extra_data_list_remove_follower(e: &mut Engine, this: Ptr<ExtraDataList>, follower: u32) {
    let extra = find_extra(e, this, EXTRA_FOLLOWER);
    if extra.is_null() {
        return;
    }
    let followers = e.mem.u32(extra.addr() + 0x0c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), follower);
        e.call(LIST_REMOVE_ITEM, &args![followers, slot]);
    });
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let unavailable = e.call(SAVE_LOAD_UNAVAILABLE, &args![save_load]).bool();
    if !unavailable {
        let handler = e.global::<u32>(DATA_HANDLER);
        fn_004226e0(e, Ptr::new(handler));
    }
}

// Translated from 004226e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDataHandler::bClearingData` (+0x61D, Xbox PDB) of the handler it is
/// called on.
pub fn fn_004226e0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x61d)
}

// Translated from 00422700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type `0x1D` extra data (`EXTRA_FOLLOWER`) of the list, or null.
pub fn fn_00422700(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_FOLLOWER)
}

// Translated from 00422720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveFollowerExtra` (Xbox PDB): removes the type `0x1D`
/// extra data (`EXTRA_FOLLOWER`) when the list has one.
pub fn extra_data_list_remove_follower_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    if !find_extra(e, this, EXTRA_FOLLOWER).is_null() {
        remove_extra(e, this, EXTRA_FOLLOWER);
    }
}

// Translated from 00422750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the type `0x48` extra data (`EXTRA_REFRACTION_PROPERTY`, from
/// the name of its getter): a zero `enable` removes the extra data (and
/// deletes it, `RemoveExtra(extra, true)`) when there is one; otherwise the
/// `float` `value` is stored at +0x0C of the existing extra data, or of a new
/// 0x10-byte one (built with the value by `00435f20`, added first).
pub fn fn_00422750(e: &mut Engine, this: Ptr<ExtraDataList>, enable: u8, value: f32) {
    let mut extra = find_extra(e, this, EXTRA_REFRACTION_PROPERTY);
    if enable == 0 {
        if !extra.is_null() {
            e.call(REMOVE_EXTRA_OBJECT, &args![this, extra, 1u32]);
        }
        return;
    }
    if extra.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        extra = if block == 0 {
            Ptr::NULL
        } else {
            e.call(REFRACTION_PROPERTY_INIT, &args![block, value]).ptr()
        };
        add_extra(e, this, extra);
    }
    e.mem.set_u32(extra.addr() + 0x0c, x87_float(value));
}

// Translated from 00422820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRefractionPropertyExtra` (Xbox PDB): the type `0x48`
/// extra data, or null.
pub fn extra_data_list_get_refraction_property_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_REFRACTION_PROPERTY)
}

// Translated from 00422850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the last finished sequence (type `0x41`,
/// `EXTRA_LASTFINISHEDSEQUENCE`): removes the old one
/// ([`extra_data_list_remove_last_finished_sequence`]), then, for a non-null
/// `name`, builds a new 0x10-byte extra data with it (`004351d0`, which copies
/// the string) and adds it.
pub fn fn_00422850(e: &mut Engine, this: Ptr<ExtraDataList>, name: u32) {
    extra_data_list_remove_last_finished_sequence(e, this);
    if name != 0 {
        let extra = new_extra_with(e, 0x10, LAST_FINISHED_SEQUENCE_INIT, name);
        add_extra(e, this, extra);
    }
}

// Translated from 004228f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetLastFinishedSequence` (Xbox PDB): the word at +0x0C of
/// the type `0x41` extra data (`EXTRA_LASTFINISHEDSEQUENCE`), or 0.
pub fn extra_data_list_get_last_finished_sequence(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_LASTFINISHEDSEQUENCE, 0)
}

// Translated from 00422920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveLastFinishedSequence` (Xbox PDB): removes the type
/// `0x41` extra data.
pub fn extra_data_list_remove_last_finished_sequence(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra(e, this, EXTRA_LASTFINISHEDSEQUENCE);
}

/// The setters of the saved animation and the saved havok data (types `0x42`
/// and `0x3D`): a zero `buffer` removes the extra data; otherwise, when the
/// list already has one, `warning` is logged (`SaveGameWarning`, `00845cc0`)
/// and the old buffer is deleted (`ExtraSavedAnimation::DeleteBuffer`,
/// `00435c10`); else a new 0x10-byte extra data is built with `construct` and
/// added. Then `buffer` is stored at +0x0C.
fn set_saved_buffer(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    buffer: u32,
    warning: u32,
    construct: u32,
) {
    if buffer == 0 {
        remove_extra(e, list, extra_type);
        return;
    }
    let mut extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        extra = new_extra(e, 0x10, construct);
        add_extra(e, list, extra);
    } else {
        e.call(SAVE_GAME_WARNING, &args![warning]);
        e.call(DELETE_SAVED_BUFFER, &args![extra]);
    }
    e.mem.set_u32(extra.addr() + 0x0c, buffer);
}

/// The restorers: when the list has an extra data of `extra_type`, loads its
/// buffer through `load` (`this` = a stack slot holding the buffer address,
/// stored with `008c71b0`; the argument `destination`), clears the buffer word
/// and removes the extra data. True when there was one.
fn restore_saved_buffer(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    destination: u32,
    load: u32,
) -> bool {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        return false;
    }
    let buffer = e.mem.u32(extra.addr() + 0x0c);
    e.with_stack(4, |e, slot| {
        let slot_address = e.call(STORE_VALUE, &args![slot, buffer]).u32();
        e.call(load, &args![slot_address, destination]);
    });
    e.mem.set_u32(extra.addr() + 0x0c, 0);
    remove_extra(e, list, extra_type);
    true
}

// Translated from 00422940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetSavedAnimation` (Xbox PDB), type `0x42`
/// (`EXTRA_SAVED_ANIMATION`): see `set_saved_buffer`.
pub fn extra_data_list_set_saved_animation(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32) {
    set_saved_buffer(
        e,
        this,
        EXTRA_SAVED_ANIMATION,
        buffer,
        MESSAGE_SAVED_ANIMATION_EXISTS,
        SAVED_ANIMATION_INIT,
    );
}

// Translated from 00422a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetSavedAnimation` (Xbox PDB): the word at +0x0C of the
/// type `0x42` extra data, or 0.
pub fn extra_data_list_get_saved_animation(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_SAVED_ANIMATION, 0)
}

// Translated from 00422a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RestoreSavedAnimation` (Xbox PDB): loads the saved
/// animation into `destination` (`BGSLoadGameSubBuffer::LoadAnimation`,
/// `00864b00`, on the buffer), then forgets it and removes the extra data;
/// false when the list has none.
pub fn extra_data_list_restore_saved_animation(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    destination: u32,
) -> bool {
    restore_saved_buffer(e, this, EXTRA_SAVED_ANIMATION, destination, LOAD_ANIMATION)
}

// Translated from 00422aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveSavedAnimation` (Xbox PDB): removes the type `0x42`
/// extra data.
pub fn extra_data_list_remove_saved_animation(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra(e, this, EXTRA_SAVED_ANIMATION);
}

// Translated from 00422ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetSavedHavokData` (Xbox PDB), type `0x3D`
/// (`EXTRA_SAVED_HAVOK_DATA`): see `set_saved_buffer`.
pub fn extra_data_list_set_saved_havok_data(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32) {
    set_saved_buffer(
        e,
        this,
        EXTRA_SAVED_HAVOK_DATA,
        buffer,
        MESSAGE_SAVED_HAVOK_DATA_EXISTS,
        SAVED_HAVOK_DATA_INIT,
    );
}

// Translated from 00422b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetSavedHavokData` (Xbox PDB): the word at +0x0C of the
/// type `0x3D` extra data, or 0.
pub fn extra_data_list_get_saved_havok_data(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_SAVED_HAVOK_DATA, 0)
}

// Translated from 00422bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RestoreSavedHavokData` (Xbox PDB): as
/// [`extra_data_list_restore_saved_animation`] with
/// `BGSLoadGameSubBuffer::LoadHavokData` (`00864d80`) and the type `0x3D`.
pub fn extra_data_list_restore_saved_havok_data(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    destination: u32,
) -> bool {
    restore_saved_buffer(
        e,
        this,
        EXTRA_SAVED_HAVOK_DATA,
        destination,
        LOAD_HAVOK_DATA,
    )
}

// Translated from 00422c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveSavedHavokData` (Xbox PDB): removes the type `0x3D`
/// extra data.
pub fn extra_data_list_remove_saved_havok_data(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra(e, this, EXTRA_SAVED_HAVOK_DATA);
}

// ---------------------------------------------------------------------------
// Third block: the save size

/// `__RTDynamicCast(extra, 0, BSExtraData, target, 0)`.
fn extra_cast(e: &mut Engine, extra: Ptr<BSExtraData>, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![extra, 0u32, RTTI_BS_EXTRA_DATA, target, 0u32],
    )
    .u32()
}

// Translated from 00422c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size in bytes `ExtraDataList::SaveGame` (`004235e0`) writes for the
/// change flags `flags` and the reference `reference` (the Xbox name is not
/// in the map; the exe's message calls it `GetSaveSize`): under the list lock,
/// 2 for the count of extra data, 6 more when the game uses save-game blocks
/// (`TESSaveLoadGame::UseSaveGameBlocks`), then for each extra data of a type
/// that is saved the id byte (1) and its payload, as a 16-bit wrapping sum.
/// The arms (jump table at `004234c8`, indexed by `00423550`): type `0x0C`
/// (persistent cell) 4 when `reference` is an actor (virtual `+0x100`);
/// `0x18` (package start location) `4 + 12 + 4`; `0x19` (package) 14, and from
/// save version `0x40` on, for a form id below `0xFF000000`, 1 plus the
/// package's own size (virtual `+0x14C` of the package at +0x0C); `0x1B` (run
/// once packages) 2 + 5 per item; `0x1D` (follower) 2 + 4 per item; `0x2A`
/// (lock) 6 with change flag `0x1000`; `0x2B` (teleport) the teleport data's
/// size with flag `0x20000`; `0x39` and `0x46` 4; `0x41` (last finished
/// sequence) 1 + the string length with flag `0x10000000`; `0x4D` (info
/// general topic) its size when it holds one; `0x4E` 1. A type `0x1F` (ghost)
/// adds its id byte only. In the other builds the compiler left more arms
/// guarded by change flags; here those masks are the constant 0 (`AND reg,0`)
/// and the arms never run. With the debug switch set it logs the size
/// (`GetSaveSize(): ...`, line 0x29B5 of the source).
pub fn fn_00422c40(e: &mut Engine, this: Ptr<ExtraDataList>, flags: u32, reference: Ptr) -> u16 {
    lock(e, 0);
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let mut size: u16 = 0;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(2);
    let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let before = size;
        match get_type(e, current) {
            // EXTRA_PERSISTENT_CELL
            0x0c => {
                if !reference.is_null() && e.vcall(reference.addr(), 0x100, &args![]).bool() {
                    size = size.wrapping_add(4);
                }
            }
            // EXTRA_PACKAGESTARTLOC
            0x18 => {
                extra_cast(e, current, RTTI_EXTRA_PACKAGE_START_LOCATION);
                size = size.wrapping_add(4 + 0x0c + 4);
            }
            // EXTRA_PACKAGE
            0x19 => {
                let package = extra_cast(e, current, RTTI_EXTRA_PACKAGE);
                size = size.wrapping_add(0x0e);
                let version = e.call(SAVE_VERSION, &args![save_load]).u8();
                if version >= 0x40 {
                    let form = e.mem.u32(package + 0x0c);
                    let id = form_id(e, form);
                    let handler = e.global::<u32>(DATA_HANDLER);
                    if e.call(FORM_ID_IS_FILE_FORM, &args![handler, id]).bool() {
                        size = size.wrapping_add(1);
                        let own = e.vcall(form, 0x14c, &args![]).u16();
                        size = size.wrapping_add(own);
                    }
                }
            }
            // EXTRA_RUN_ONCE_PACKAGES
            0x1b => {
                let packages = extra_cast(e, current, RTTI_EXTRA_RUN_ONCE_PACKS);
                size = size.wrapping_add(2);
                let list = e.mem.u32(packages + 0x0c);
                let count = e.call(LIST_COUNT, &args![list]).u32();
                size = size.wrapping_add(count.wrapping_mul(5) as u16);
            }
            // EXTRA_FOLLOWER
            0x1d => {
                let followers = extra_cast(e, current, RTTI_EXTRA_FOLLOWER);
                let list = e.mem.u32(followers + 0x0c);
                let count = e.call(LIST_COUNT, &args![list]).u32();
                size = size
                    .wrapping_add(count.wrapping_mul(4) as u16)
                    .wrapping_add(2);
            }
            // EXTRA_LOCK
            0x2a => {
                if flags & 0x1000 != 0 {
                    size = size.wrapping_add(1 + 4 + 1);
                }
            }
            // EXTRA_TELEPORT
            0x2b => {
                if flags & 0x2_0000 != 0 {
                    let teleport = extra_cast(e, current, RTTI_EXTRA_TELEPORT);
                    let data = e.mem.u32(teleport + 0x0c);
                    let own = e.call(TELEPORT_SAVE_SIZE, &args![data]).u16();
                    size = size.wrapping_add(own);
                }
            }
            // EXTRA_ITEMDROPPER and EXTRA_HEAD_TRACK_TARGET: one id each.
            0x39 | 0x46 => size = size.wrapping_add(4),
            // EXTRA_LASTFINISHEDSEQUENCE
            0x41 => {
                if flags & 0x1000_0000 != 0 {
                    let sequence = extra_cast(e, current, RTTI_EXTRA_LAST_FINISHED_SEQUENCE);
                    size = size.wrapping_add(1);
                    let name = e.mem.u32(sequence + 0x0c);
                    let length = e.call(STRING_LENGTH_OF, &args![name]).u32();
                    size = size.wrapping_add(length as u16);
                }
            }
            // EXTRA_INFO_GENERAL_TOPIC
            0x4d => {
                let topic = extra_cast(e, current, RTTI_EXTRA_INFO_GENERAL_TOPIC);
                let data = e.mem.u32(topic + 0x0c);
                if data != 0 {
                    let own = e.call(INFO_GENERAL_TOPIC_SAVE_SIZE, &args![data]).u16();
                    size = size.wrapping_add(own);
                }
            }
            // EXTRA_NO_RUMORS
            0x4e => size = size.wrapping_add(1),
            _ => {}
        }
        if before != size {
            size = size.wrapping_add(1);
        } else if get_type(e, current) == 0x1f {
            // EXTRA_GHOST: the id byte only.
            size = size.wrapping_add(1);
        }
        current = get_next(e, current);
    }
    let switch = e
        .call(SETTING_BYTE_ADDRESS, &args![DEBUG_SWITCHES_OBJECT])
        .u32();
    if e.mem.u8(switch) != 0 {
        let written = size as u32;
        let world = e.call(SAVING_FORM_HEADER, &args![save_load]).u32();
        if world != 0 {
            let form_id = e.mem.u32(world);
            let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
            let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
            let header_flags = e.mem.u32(world + 5);
            e.call(
                ERROR_LOG,
                &args![
                    GET_SAVE_SIZE_FORMAT,
                    written,
                    form_id,
                    name,
                    header_flags,
                    0x29b5u32,
                    SOURCE_FILE_NAME
                ],
            );
        } else {
            e.call(
                ERROR_LOG,
                &args![
                    GET_SAVE_SIZE_SHORT_FORMAT,
                    written,
                    0x29b5u32,
                    SOURCE_FILE_NAME
                ],
            );
        }
    }
    unlock(e);
    size
}

// ---------------------------------------------------------------------------
// Fourth block: the save writer

/// The current position of the save-load stream (`00825c00`): a pointer into
/// the buffer being written.
fn save_position(e: &mut Engine) -> u32 {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_POSITION, &args![save_load]).u32()
}

/// `TESSaveLoadGame::Save(buffer, size)`: writes `size` bytes at `buffer`.
fn save_bytes(e: &mut Engine, buffer: u32, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_WRITE, &args![save_load, buffer, size]);
}

/// Writes the low `size` bytes of `value`, kept in a local the stream reads.
fn save_local(e: &mut Engine, value: u32, size: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        save_bytes(e, slot.addr(), size);
    });
}

/// `SaveNumericID` of the form id `id`, kept in a local the stream reads.
fn save_numeric_id(e: &mut Engine, id: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![save_load, slot, 4u32]);
    });
}

/// The id of `form` for the save, 0 for a null form.
fn saved_form_id(e: &mut Engine, form: u32) -> u32 {
    if form == 0 {
        0
    } else {
        form_id(e, form)
    }
}

/// Whether the debug switch (the byte `00408d60` finds for `011de4e8`) is set.
fn debug_switch_set(e: &mut Engine) -> bool {
    let switch = e
        .call(SETTING_BYTE_ADDRESS, &args![DEBUG_SWITCHES_OBJECT])
        .u32();
    e.mem.u8(switch) != 0
}

/// The debug message of the save functions: `format` with the number of
/// bytes `written`, and for the world space of the save-load object (the form
/// header `004fd3e0` gives, when there is one) its id, name and flags; the
/// line and file of the source follow.
fn log_save_size(e: &mut Engine, written: u32, format: u32, short_format: u32, line: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let world = e.call(SAVING_FORM_HEADER, &args![save_load]).u32();
    if world != 0 {
        let form_id = e.mem.u32(world);
        let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let header_flags = e.mem.u32(world + 5);
        e.call(
            ERROR_LOG,
            &args![
                format,
                written,
                form_id,
                name,
                header_flags,
                line,
                SOURCE_FILE_NAME
            ],
        );
    } else {
        e.call(
            ERROR_LOG,
            &args![short_format, written, line, SOURCE_FILE_NAME],
        );
    }
}

// Translated from 004235e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SaveGame` (Xbox PDB): writes the list to the save-load
/// stream for the change flags `flags` and the reference `reference`. With
/// save-game blocks the stream gets the tag `KOLB` and a 16-bit block size
/// (filled in at the end; a warning when it does not fit). Then, under the
/// list lock, a 16-bit count of the extra data written (filled in at the
/// end), and for each extra data of a type that is saved its type byte and
/// payload; the count goes up when the stream position moved. The size
/// that [`fn_00422c40`] computes is what this writes. The arms (jump table at
/// `00424818`, indexed by `004248b0`) are the ones of [`fn_00422c40`]:
/// `0x0C` (persistent cell, for an actor reference) the id of the cell's
/// world space; `0x18` (package start location) the location as an id (a
/// message when there is none, using the reference's name), 12 bytes at
/// +0x10 and 4 at +0x1C; `0x19` (package) the package id, 4 bytes at +0x10,
/// the id of the word at +0x14, bytes +0x18 and +0x19 and, from save version
/// `0x40` for a form id below `0xFF000000`, the package's type byte and its
/// own data (virtual `+0x150`); `0x1B` (run once packages) the count and a
/// pair id and byte per item; `0x1D` (follower) the count and an id per
/// item; `0x1F` the type byte; `0x2A` (lock, change flag `0x1000`) the lock
/// level, key id and flags byte; `0x2B` (teleport, flag `0x20000`)
/// `DoorTeleportData::SaveGame`; `0x39` and `0x46` an id; `0x41` (flag
/// `0x10000000`) a length byte and the string; `0x4D` the topic data; `0x4E`
/// the byte at +0x0C. The other masks of the Xbox build are the constant 0
/// here, so the arms of those types never write. With the debug switch set it
/// logs the number of bytes written (`SaveGame(): ...`, line `0x2C1F`).
pub fn extra_data_list_save_game(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    flags: u32,
    reference: Ptr,
) {
    let mut block_size_at = 0u32;
    let mut start = save_position(e);
    if debug_switch_set(e) {
        start = save_position(e);
    }
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        save_local(e, SAVE_BLOCK_TAG, 4);
        block_size_at = save_position(e);
        save_local(e, 0, 2);
    }
    lock(e, 0);
    let mut count: u16 = 0;
    let count_at = save_position(e);
    save_local(e, 0, 2);
    let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let type_byte = get_type(e, current) as u32;
        let before = save_position(e);
        match get_type(e, current) {
            // EXTRA_PERSISTENT_CELL
            0x0c => {
                if !reference.is_null() && e.vcall(reference.addr(), 0x100, &args![]).bool() {
                    save_local(e, type_byte, 1);
                    let cell = extra_cast(e, current, RTTI_EXTRA_PERSISTENT_CELL);
                    let mut id = 0;
                    let data = e.mem.u32(cell + 0x0c);
                    if data != 0 {
                        let world = e.call(CELL_GET_WORLD_SPACE, &args![data]).u32();
                        id = form_id(e, world);
                    }
                    save_numeric_id(e, id);
                }
            }
            // EXTRA_PACKAGESTARTLOC
            0x18 => {
                save_local(e, type_byte, 1);
                let location = extra_cast(e, current, RTTI_EXTRA_PACKAGE_START_LOCATION);
                let mut id = 0;
                let form = e.mem.u32(location + 0x0c);
                if form == 0 {
                    let name = e.vcall(reference.addr(), 0x130, &args![]).u32();
                    let reference_id = form_id(e, reference.addr());
                    e.call(
                        LOG_MESSAGE,
                        &args![PACKAGE_START_LOCATION_NULL_MESSAGE, reference_id, name],
                    );
                } else {
                    id = form_id(e, form);
                }
                save_numeric_id(e, id);
                save_bytes(e, location + 0x10, 0x0c);
                save_bytes(e, location + 0x1c, 4);
            }
            // EXTRA_PACKAGE
            0x19 => {
                save_local(e, type_byte, 1);
                let package = extra_cast(e, current, RTTI_EXTRA_PACKAGE);
                let form = e.mem.u32(package + 0x0c);
                let id = form_id(e, form);
                let other = e.mem.u32(package + 0x14);
                let other_id = if other != 0 { form_id(e, other) } else { 0 };
                save_numeric_id(e, id);
                save_bytes(e, package + 0x10, 4);
                save_numeric_id(e, other_id);
                save_bytes(e, package + 0x18, 1);
                save_bytes(e, package + 0x19, 1);
                let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                let version = e.call(SAVE_VERSION, &args![save_load]).u8();
                if version >= 0x40 {
                    let handler = e.global::<u32>(DATA_HANDLER);
                    if e.call(FORM_ID_IS_FILE_FORM, &args![handler, id]).bool() {
                        let package_type = e.call(PACKAGE_GET_TYPE, &args![form]).u8();
                        save_local(e, package_type as u32, 1);
                        e.vcall(form, 0x150, &args![]);
                    }
                }
            }
            // EXTRA_RUN_ONCE_PACKAGES: the count is filled in afterwards.
            0x1b => {
                save_local(e, type_byte, 1);
                let mut items: u16 = 0;
                let items_at = save_position(e);
                save_local(e, 0, 2);
                let packages = extra_cast(e, current, RTTI_EXTRA_RUN_ONCE_PACKS);
                let mut node = e.mem.u32(packages + 0x0c);
                while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                    let item = list_item(e, node);
                    if item != 0 {
                        let form = e.mem.u32(item);
                        let id = form_id(e, form);
                        save_numeric_id(e, id);
                        save_bytes(e, item + 4, 1);
                        items = items.wrapping_add(1);
                    }
                    node = list_next(e, node);
                }
                e.mem.set_u16(items_at, items);
            }
            // EXTRA_FOLLOWER: the count is filled in afterwards.
            0x1d => {
                save_local(e, type_byte, 1);
                let followers = extra_cast(e, current, RTTI_EXTRA_FOLLOWER);
                let mut items: u16 = 0;
                let items_at = save_position(e);
                save_local(e, 0, 2);
                let mut node = e.mem.u32(followers + 0x0c);
                while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                    let item = list_item(e, node);
                    if item != 0 {
                        let id = form_id(e, item);
                        save_numeric_id(e, id);
                        items = items.wrapping_add(1);
                    }
                    node = list_next(e, node);
                }
                e.mem.set_u16(items_at, items);
            }
            // EXTRA_GHOST
            0x1f => save_local(e, type_byte, 1),
            // EXTRA_LOCK
            0x2a => {
                if flags & 0x1000 != 0 {
                    save_local(e, type_byte, 1);
                    let lock_data = e.mem.u32(current.addr() + 0x0c);
                    let key = e.mem.u32(lock_data + 4);
                    let key_id = if key != 0 { form_id(e, key) } else { 0 };
                    save_bytes(e, lock_data, 1);
                    save_numeric_id(e, key_id);
                    save_bytes(e, lock_data + 8, 1);
                }
            }
            // EXTRA_TELEPORT
            0x2b => {
                if flags & 0x2_0000 != 0 {
                    save_local(e, type_byte, 1);
                    let teleport = extra_cast(e, current, RTTI_EXTRA_TELEPORT);
                    let data = e.mem.u32(teleport + 0x0c);
                    e.call(DOOR_TELEPORT_DATA_SAVE_GAME, &args![data]);
                }
            }
            // EXTRA_ITEMDROPPER
            0x39 => {
                save_local(e, type_byte, 1);
                let dropper = extra_cast(e, current, RTTI_EXTRA_ITEM_DROPPER);
                let form = e.mem.u32(dropper + 0x0c);
                let id = saved_form_id(e, form);
                save_numeric_id(e, id);
            }
            // EXTRA_LASTFINISHEDSEQUENCE
            0x41 => {
                if flags & 0x1000_0000 != 0 {
                    save_local(e, type_byte, 1);
                    let sequence = extra_cast(e, current, RTTI_EXTRA_LAST_FINISHED_SEQUENCE);
                    let name = e.mem.u32(sequence + 0x0c);
                    let length = e.call(STRING_LENGTH_OF, &args![name]).u8();
                    save_local(e, length as u32, 1);
                    save_bytes(e, name, length as u32);
                }
            }
            // EXTRA_HEAD_TRACK_TARGET
            0x46 => {
                save_local(e, type_byte, 1);
                let target = extra_cast(e, current, RTTI_EXTRA_HEAD_TRACK_TARGET);
                let form = e.mem.u32(target + 0x0c);
                let id = saved_form_id(e, form);
                save_numeric_id(e, id);
            }
            // EXTRA_INFO_GENERAL_TOPIC
            0x4d => {
                let topic = extra_cast(e, current, RTTI_EXTRA_INFO_GENERAL_TOPIC);
                let data = e.mem.u32(topic + 0x0c);
                if data != 0 {
                    save_local(e, type_byte, 1);
                    e.call(INFO_GENERAL_TOPIC_SAVE, &args![data]);
                }
            }
            // EXTRA_NO_RUMORS
            0x4e => {
                save_local(e, type_byte, 1);
                let rumors = extra_cast(e, current, RTTI_EXTRA_NO_RUMORS);
                save_bytes(e, rumors + 0x0c, 1);
            }
            _ => {}
        }
        if before != save_position(e) {
            count = count.wrapping_add(1);
        }
        current = get_next(e, current);
    }
    e.mem.set_u16(count_at, count);
    if debug_switch_set(e) {
        let written = save_position(e).wrapping_sub(start);
        log_save_size(
            e,
            written,
            SAVE_GAME_SIZE_FORMAT,
            SAVE_GAME_SIZE_SHORT_FORMAT,
            0x2c1f,
        );
    }
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        let end = save_position(e);
        if end > block_size_at.wrapping_add(0xffff) {
            e.call(
                LOG_MESSAGE,
                &args![SAVE_BLOCK_TOO_LARGE_MESSAGE, SOURCE_FILE_NAME, 0x2c1fu32],
            );
        }
        e.mem
            .set_u16(block_size_at, end.wrapping_sub(block_size_at) as u16);
    }
    unlock(e);
}

// ---------------------------------------------------------------------------
// Fifth block: the loader of the old save format

/// Reads `size` bytes of the save-load stream into `buffer` (`008579e0`).
fn load_bytes(e: &mut Engine, buffer: u32, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_READ, &args![save_load, buffer, size]);
}

/// Reads `size` (1, 2 or 4) bytes of the stream into a local and gives them.
fn load_local(e: &mut Engine, size: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        load_bytes(e, slot.addr(), size);
        e.mem.u32(slot.addr())
    })
}

/// `LoadNumericID` of one form id.
fn load_id(e: &mut Engine) -> u32 {
    e.with_stack(4, |e, slot| {
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![save_load, slot, 4u32]);
        e.mem.u32(slot.addr())
    })
}

/// The save version byte (`008df040`).
fn save_version(e: &mut Engine) -> u8 {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_VERSION, &args![save_load]).u8()
}

/// The form with the id `id` (`004839c0`) cast to `target` (the cast of a
/// null form is null).
fn form_as(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    dynamic_cast(e, form, target)
}

/// The messages of the loaders about the form being loaded: with the form
/// header of the save-load object (`004fd3c0`) the format gets the file, the
/// line, the form id, its name, the header's version byte and flags; without,
/// the short format gets the file, the line and the save version.
fn log_load_problem(e: &mut Engine, format: u32, short_format: u32, leading: &[u32], line: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let header = e.call(LOADING_FORM_HEADER, &args![save_load]).u32();
    let mut words = vec![];
    if header != 0 {
        let id = e.mem.u32(header);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let header_flags = e.mem.u32(header + 5);
        let header_version = e.mem.u8(header + 9) as u32;
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        words.push(format);
        words.extend_from_slice(leading);
        words.extend([
            SOURCE_FILE_NAME,
            line,
            id,
            name,
            header_version,
            header_flags,
        ]);
    } else {
        let version = save_version(e) as u32;
        words.push(short_format);
        words.extend_from_slice(leading);
        words.extend([SOURCE_FILE_NAME, line, version]);
    }
    e.call(LOG_MESSAGE, &words);
}

// Translated from 00424940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x0C of the object it is called on (an extra data).
pub fn fn_00424940(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x0c)
}

// Translated from 00424960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::LoadGame` (Xbox PDB): reads the list written by
/// [`extra_data_list_save_game`] (the old save format) from the save-load
/// stream into the list, for the change flags `flags` and the reference
/// `reference` (the second word is never read). With save-game blocks the
/// stream starts with the tag `KOLB` (a message when it is missing, with the
/// form being loaded when the save-load object knows one) and a 16-bit block
/// size; then comes the count (16 bits) of the extra data and for each its
/// type byte and payload, set on the list with the setters of the list. The
/// types (jump table at `00425e94`, indexed by `00425f40`): `0x0C` the
/// persistent cell of an actor reference (the old cell is removed from, the
/// world space's persistent cell set and the reference added to it, and the
/// reference removed from the interior cell it is in), `0x18` the package
/// start location, `0x19` a package, `0x1B` the run once packages, `0x1D` the
/// followers (as the ids the stream holds), `0x1F` the ghost, `0x2A` the lock
/// (change flag `0x1000`), `0x2B` the teleport (flag `0x20000`), `0x30` the
/// scale (before save version `0x43`, flag `0x10`), `0x39`, `0x3A` and `0x46`
/// ids for the item dropper and the dropped items, `0x41` the last finished
/// sequence (flag `0x10000000`; a table index in versions `0x15` and `0x16`,
/// a length-prefixed name otherwise), `0x4D` the info general topic and
/// `0x4E` no rumors. The types `0x2E`, `0x45`, `0x90` and `0x91` read nothing;
/// the others the Xbox build saved with change flags whose masks are 0 here
/// and are skipped too; a type outside the table gets a message. At the end
/// the stream position is compared with the block size and any difference is
/// reported.
pub fn extra_data_list_load_game(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    flags: u32,
    _unused_2: u32,
    reference: Ptr,
) {
    let mut block_size: u32 = 0;
    let mut block_start: u32 = 0;
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let blocks = e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool();
    if blocks {
        let tag = load_local(e, 4);
        if tag != SAVE_BLOCK_TAG {
            log_load_problem(
                e,
                LOAD_GAME_BAD_TAG_FORMAT,
                LOAD_GAME_BAD_TAG_SHORT_FORMAT,
                &[],
                0x2c2c,
            );
        }
        block_start = save_position(e);
        block_size = load_local(e, 2) & 0xffff;
    }
    // The reference as an actor; the result is not used.
    e.call(
        RT_DYNAMIC_CAST,
        &args![reference, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
    );
    let count = load_local(e, 2) & 0xffff;
    for _ in 0..count {
        let extra_type = load_local(e, 1) as u8;
        match extra_type {
            // EXTRA_PERSISTENT_CELL
            0x0c => {
                if !reference.is_null() && e.vcall(reference.addr(), 0x100, &args![]).bool() {
                    let cell = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
                    if cell != 0 {
                        e.call(SET_PERSISTENT_CELL, &args![this, 0u32]);
                        e.call(CELL_REMOVE_REFERENCE, &args![cell, reference]);
                    }
                    let id = load_id(e);
                    if e.call(REFERENCE_GET_REF_PERSISTS, &args![reference]).bool() {
                        let world = form_as(e, id, RTTI_TES_WORLD_SPACE);
                        if world != 0 {
                            let cell = e.call(WORLD_SPACE_GET_WORD_34, &args![world]).u32();
                            e.call(SET_PERSISTENT_CELL, &args![this, cell]);
                            // (the code tests the reference again here; it
                            // cannot be null in this branch)
                            let cell = e.call(WORLD_SPACE_GET_WORD_34, &args![world]).u32();
                            e.call(CELL_ADD_REFERENCE, &args![cell, reference, 0u32]);
                            if e.call(REFERENCE_GET_PARENT_CELL, &args![reference]).u32() != 0 {
                                let parent =
                                    e.call(REFERENCE_GET_PARENT_CELL, &args![reference]).u32();
                                if e.call(CELL_IS_INTERIOR, &args![parent]).bool() {
                                    let parent =
                                        e.call(REFERENCE_GET_PARENT_CELL, &args![reference]).u32();
                                    e.call(CELL_REMOVE_REFERENCE, &args![parent, reference]);
                                }
                            }
                        }
                    }
                }
            }
            // EXTRA_PACKAGESTARTLOC: a WORLD_LOCATION record (the location as
            // a form, 12 bytes of position, the rotation).
            0x18 => e.with_stack(0x14, |e, record| {
                e.call(PATH_LOCATION_INIT, &args![record]);
                let id = load_id(e);
                load_bytes(e, record.addr() + 4, 0x0c);
                load_bytes(e, record.addr() + 0x10, 4);
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                e.mem.set_u32(record.addr(), form);
                let world = dynamic_cast(e, form, RTTI_TES_WORLD_SPACE);
                let cell = dynamic_cast(e, form, RTTI_TES_OBJECT_CELL);
                if world != 0 || cell != 0 {
                    let rotation = f32::from_bits(x87_quiet_bits(e.mem.u32(record.addr() + 0x10)));
                    e.call(
                        SET_PACKAGE_START_LOCATION,
                        &args![this, world, cell, record.addr() + 4, rotation],
                    );
                }
            }),
            // EXTRA_PACKAGE
            0x19 => {
                let id = load_id(e);
                let word = load_local(e, 4);
                let other_id = load_id(e);
                let first_byte = load_local(e, 1);
                let second_byte = load_local(e, 1);
                let mut package = 0;
                if save_version(e) >= 0x40 {
                    let handler = e.global::<u32>(DATA_HANDLER);
                    if e.call(FORM_ID_IS_FILE_FORM, &args![handler, id]).bool() {
                        let package_type = load_local(e, 1);
                        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                        package = e
                            .call(LOAD_PACKAGE_BY_ID, &args![save_load, id, package_type])
                            .u32();
                        e.vcall(package, 0x154, &args![]);
                    }
                }
                if package == 0 {
                    package = form_as(e, id, RTTI_TES_PACKAGE);
                }
                if package != 0 {
                    e.call(
                        SET_PACKAGE_EXTRA,
                        &args![this, package, word, other_id, first_byte, second_byte, 0u32],
                    );
                }
            }
            // EXTRA_RUN_ONCE_PACKAGES
            0x1b => {
                let packages = load_local(e, 2) & 0xffff;
                for _ in 0..packages {
                    let id = load_id(e);
                    let package_byte = load_local(e, 1);
                    let package = form_as(e, id, RTTI_TES_PACKAGE);
                    if package != 0 {
                        e.call(ADD_RUN_ONCE_PACKAGE, &args![this, package, package_byte]);
                    }
                }
            }
            // EXTRA_FOLLOWER: the ids go in as they are.
            0x1d => {
                let followers = load_local(e, 2) & 0xffff;
                for _ in 0..followers {
                    let id = load_id(e);
                    extra_data_list_add_follower(e, this, Ptr::new(id));
                }
            }
            // EXTRA_GHOST
            0x1f => {
                e.call(SET_GHOST, &args![this, 1u32]);
            }
            // EXTRA_LOCK
            0x2a => {
                if flags & 0x1000 != 0 {
                    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
                    let lock_data = if block == 0 {
                        0
                    } else {
                        e.call(LOCK_RECORD_INIT, &args![block]).u32()
                    };
                    if save_version(e) >= 0x15 {
                        load_bytes(e, lock_data, 1);
                        let key_id = load_id(e);
                        if save_version(e) < 0x1a {
                            load_id(e);
                        }
                        load_bytes(e, lock_data + 8, 1);
                        if key_id != 0 {
                            let key = form_as(e, key_id, RTTI_TES_KEY);
                            e.mem.set_u32(lock_data + 4, key);
                        }
                    }
                    e.call(SET_LOCK_PTR, &args![this, lock_data]);
                }
            }
            // EXTRA_TELEPORT
            0x2b => {
                if flags & 0x2_0000 != 0 {
                    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                    let teleport = if block == 0 {
                        0
                    } else {
                        e.call(DOOR_TELEPORT_DATA_INIT, &args![block]).u32()
                    };
                    e.call(DOOR_TELEPORT_DATA_LOAD_GAME, &args![teleport]);
                    e.call(SET_TELEPORT, &args![this, teleport]);
                }
            }
            // EXTRA_SCALE: a float, saved only before version 0x43.
            0x30 => {
                if save_version(e) < 0x43 && flags & 0x10 != 0 {
                    let word = load_local(e, 4);
                    let scale = f32::from_bits(x87_quiet_bits(word));
                    if reference.is_null() {
                        e.call(SET_SCALE, &args![this, scale]);
                    } else {
                        e.call(REFERENCE_SET_SCALE, &args![reference, scale]);
                    }
                }
            }
            // EXTRA_ITEMDROPPER
            0x39 => {
                let id = load_id(e);
                if id != 0 {
                    e.call(ADD_DROPPED_ITEM, &args![this, id]);
                }
            }
            // EXTRA_DROPPEDITEMLIST
            0x3a => {
                let items = load_local(e, 1);
                for _ in 0..items {
                    let id = load_id(e);
                    if id != 0 {
                        e.call(ADD_DROPPED_ITEM_LIST_ENTRY, &args![this, id]);
                    }
                }
            }
            // EXTRA_LASTFINISHEDSEQUENCE
            0x41 => {
                if flags & 0x1000_0000 != 0 {
                    if save_version(e) >= 0x15 && save_version(e) < 0x17 {
                        let index = load_local(e, 4);
                        if (index as i32) < 0xf5 {
                            let entry = SEQUENCE_NAME_TABLE.wrapping_add(index.wrapping_mul(0x24));
                            let name = e.mem.u32(entry);
                            fn_00422850(e, this, name);
                        }
                    }
                    if save_version(e) < 0x15 || save_version(e) >= 0x17 {
                        let length = load_local(e, 1);
                        e.with_stack(TEXT_SEQUENCE_BUFFER_SIZE, |e, buffer| {
                            e.call(MEMSET, &args![buffer, 0u32, TEXT_SEQUENCE_BUFFER_SIZE]);
                            load_bytes(e, buffer.addr(), length);
                            fn_00422850(e, this, buffer.addr());
                        });
                    }
                }
            }
            // EXTRA_HEAD_TRACK_TARGET
            0x46 => {
                let id = load_id(e);
                if id != 0 {
                    e.call(SET_ITEM_DROPPER, &args![this, id]);
                }
            }
            // EXTRA_INFO_GENERAL_TOPIC
            0x4d => {
                let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
                let topic = if block == 0 {
                    0
                } else {
                    e.call(MENU_TOPIC_INIT, &args![block]).u32()
                };
                e.call(MENU_TOPIC_LOAD_GAME, &args![topic, reference]);
                if e.call(MENU_TOPIC_CHECK, &args![topic]).u32() != 0 {
                    e.call(SET_INFO_GENERAL_TOPIC, &args![this, topic]);
                } else if topic != 0 {
                    e.call(MENU_TOPIC_DELETE, &args![topic, 1u32]);
                }
            }
            // EXTRA_NO_RUMORS
            0x4e => {
                let flag = load_local(e, 1) as u8;
                fn_00421600(e, this, flag);
            }
            // No payload.
            0x2e | 0x45 | 0x90 | 0x91 => {}
            // The types whose masks are 0 in this build: the Xbox arms are
            // not here.
            0x0d
            | 0x16
            | 0x17
            | 0x1a
            | 0x1c
            | 0x21..=0x28
            | 0x2c
            | 0x2f
            | 0x32
            | 0x33
            | 0x3f
            | 0x49
            | 0x4a
            | 0x8d
            | 0x3e
            | 0x12 => {}
            _ => {
                e.call(
                    LOG_MESSAGE,
                    &args![LOAD_GAME_UNKNOWN_TYPE_FORMAT, extra_type as u32],
                );
            }
        }
    }
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        let position = save_position(e);
        let expected = block_size.wrapping_add(block_start);
        if position > expected {
            log_load_problem(
                e,
                LOAD_GAME_OVERREAD_FORMAT,
                LOAD_GAME_OVERREAD_SHORT_FORMAT,
                &[position.wrapping_sub(expected)],
                0x2ee9,
            );
        } else if position < expected {
            log_load_problem(
                e,
                LOAD_GAME_UNDERREAD_FORMAT,
                LOAD_GAME_UNDERREAD_SHORT_FORMAT,
                &[expected.wrapping_sub(position)],
                0x2ee9,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Sixth block: the writer of the buffered save format

/// `BGSSaveGameBuffer::` save of `size` bytes at `address`.
fn buffer_save(e: &mut Engine, buffer: u32, address: u32, size: u32) {
    e.call(BUFFER_SAVE_BYTES, &args![buffer, address, size, 0u32]);
}

/// Saves the low `size` bytes of `value`, kept in a local.
fn buffer_save_local(e: &mut Engine, buffer: u32, value: u32, size: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        buffer_save(e, buffer, slot.addr(), size);
    });
}

/// `SaveFormID_ov2` of the form (0 for none).
fn buffer_save_form_id_ov2(e: &mut Engine, buffer: u32, form: u32) {
    e.call(BUFFER_SAVE_FORM_ID_OV2, &args![buffer, form, 0u32]);
}

/// `SaveFormID` of the form.
fn buffer_save_form_id(e: &mut Engine, buffer: u32, form: u32) {
    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
}

/// The item of a list node and its next node, as the saves walk the lists of
/// extra data (`006815c0`, `00726070`).
fn walk_list(e: &mut Engine, first: u32, mut each: impl FnMut(&mut Engine, u32)) {
    let mut node = first;
    while node != 0 {
        let item = list_item(e, node);
        each(e, item);
        node = list_next(e, node);
    }
}

/// The saves of a list whose items are `(form, byte)` records: counted, with
/// the size saved at the end (`StartVariableSizedValue` and
/// `SaveVariableSizedValue_ov2`).
fn save_form_byte_list(e: &mut Engine, buffer: u32, first: u32) {
    let mut count = 0u32;
    let start = e
        .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
        .u32();
    walk_list(e, first, |e, item| {
        if item != 0 {
            let form = e.mem.u32(item);
            buffer_save_form_id_ov2(e, buffer, form);
            buffer_save(e, buffer, item + 4, 1);
            count += 1;
        }
    });
    e.call(
        BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2,
        &args![buffer, count, start],
    );
}

/// The count of the array at `array` (0 when `array` is null).
fn array_count(e: &mut Engine, array: u32) -> u32 {
    if array == 0 {
        0
    } else {
        e.call(ARRAY_COUNT, &args![array]).u32()
    }
}

// Translated from 00428070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the string at +0x0C of the object it is called on into the string
/// `out` (`004047f0(out, this + 0x0C)`). Returns `out`.
pub fn fn_00428070(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    e.call(STRING_ASSIGN, &args![out, this.addr() + 0x0c]);
    out
}

// Translated from 004280b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As [`fn_00428070`] for the string at +0x14.
pub fn fn_004280b0(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    e.call(STRING_ASSIGN, &args![out, this.addr() + 0x14]);
    out
}

// Translated from 004280f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `this` has any of the bits of `mask`.
pub fn fn_004280f0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr()) & mask != 0
}

// Translated from 00428110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at +0x17 of the object it is called on (a save buffer's
/// current save kind, unaligned) in `out`. Returns `out`.
pub fn fn_00428110(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let word = e.mem.u32(this.addr() + 0x17);
    e.mem.set_u32(out.addr(), word);
    out
}

// Translated from 00428130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the word at +0x17 of the object it is called on.
pub fn fn_00428130(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x17, value);
}

// Translated from 00426a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SaveGame_ov2` (Xbox PDB): saves the list into the
/// `BGSSaveGameBuffer` `buffer`: a variable-sized value holding, for each
/// extra data that the buffer's save kind lets through, its type byte and
/// payload. The buffer is asked (virtual `+4`) for its reference and (virtual
/// `+8`) for a flag; the table at `01183d30` gives for each type the mask of
/// save kinds that save it (read through `00428110`/`004280f0` from the
/// buffer's word at +0x17), a type with bit `0x40000000` is saved only when
/// the reference is a file form (`00469860` of its id), and type `0x89` only
/// when the flag is set. Each arm saves the fields of its type with the
/// buffer's methods (see the constants), a type outside the table gets a
/// warning. The count of the extra data saved is the size of the value
/// (`SaveVariableSizedValue_ov2`).
pub fn extra_data_list_save_game_ov2(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32) {
    let mut count = 0u32;
    let start = e
        .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
        .u32();
    let reference = e.vcall(buffer, 4, &args![]).u32();
    let flag = e.vcall(buffer, 8, &args![]).u32();
    let file_form = if reference == 0 {
        false
    } else {
        let id = form_id(e, reference);
        let handler = e.global::<u32>(DATA_HANDLER);
        e.call(FORM_ID_IS_FILE_FORM, &args![handler, id]).bool()
    };
    let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let extra_type = get_type(e, current);
        let kinds = e.mem.u32(SAVE_KIND_TABLE + extra_type as u32 * 4);
        let mut saved = false;
        if kinds != 0 {
            e.with_stack(4, |e, out| {
                let out_value = fn_00428110(e, Ptr::new(buffer), out);
                saved = fn_004280f0(e, out_value, kinds);
            });
        }
        if !file_form && saved && kinds & 0x4000_0000 != 0 {
            saved = false;
        }
        if flag == 0 && extra_type == EXTRA_TYPE_CONDITIONAL {
            saved = false;
        }
        if saved {
            buffer_save_local(e, buffer, extra_type as u32, 1);
            save_extra_ov2(e, buffer, current.addr(), extra_type, flag);
            count += 1;
        }
        current = get_next(e, current);
    }
    e.call(
        BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2,
        &args![buffer, count, start],
    );
}

/// One arm of the buffered save writer.
fn save_extra_ov2(e: &mut Engine, buffer: u32, extra: u32, extra_type: u8, flag: u32) {
    let at = |offset: u32| extra + offset;
    let word = |e: &Engine, offset: u32| e.mem.u32(extra + offset);
    match extra_type {
        // No payload.
        0x16 | 0x3e | 0x90 | 0x91 | 0x1f => {}
        // EXTRA_COUNT: 2 bytes at +0x0C.
        0x24 => buffer_save(e, buffer, at(0x0c), 2),
        // EXTRA_FRIEND_HITS: the old hits go first, then the array of hits.
        0x45 => {
            e.call(FRIEND_HITS_REMOVE_OLD_HITS, &args![extra]);
            let array = at(0x0c);
            let count = array_count(e, array);
            e.call(BUFFER_SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
            for index in 0..count {
                let hit = e.call(ARRAY_ITEM, &args![array, index]).u32();
                e.call(COMBAT_TIME_STAMP_SAVE_GAME, &args![hit, buffer]);
            }
        }
        // EXTRA_DISMEMBERED_LIMBS: fields, a form id and an array of records, each
        // with an array of form ids.
        0x5f => {
            buffer_save(e, buffer, at(0x0c), 2);
            buffer_save(e, buffer, at(0x10), 4);
            buffer_save(e, buffer, at(0x18), 4);
            buffer_save(e, buffer, at(0x1c), 1);
            let form = word(e, 0x14);
            buffer_save_form_id_ov2(e, buffer, form);
            let array = at(0x20);
            let count = array_count(e, array);
            e.call(BUFFER_SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
            for index in 0..count {
                let slot = e.call(ARRAY_ITEM, &args![array, index]).u32();
                let entry = e.mem.u32(slot);
                for offset in 0..4 {
                    buffer_save(e, buffer, entry + offset, 1);
                }
                let inner = entry + 4;
                let inner_count = array_count(e, inner);
                e.call(
                    BUFFER_SAVE_VARIABLE_SIZED_VALUE,
                    &args![buffer, inner_count],
                );
                for inner_index in 0..inner_count {
                    let inner_slot = e.call(ARRAY_ITEM, &args![inner, inner_index]).u32();
                    let form = e.mem.u32(inner_slot);
                    buffer_save_form_id_ov2(e, buffer, form);
                }
            }
        }
        // 4 bytes at +0x0C.
        0x25 | 0x30 | 0x23 | 0x56 | 0x5c | 0x5d | 0x1e | 0x27 | 0x28 => {
            buffer_save(e, buffer, at(0x0c), 4);
        }
        // 1 byte at +0x0C.
        0x4a | 0x26 | 0x8d | 0x4e => buffer_save(e, buffer, at(0x0c), 1),
        // The form id at +0x0C.
        0x21 | 0x22 | 0x1c | 0x49 | 0x6c | 0x74 | 0x55 | 0x3c | 0x39 | 0x46 | 0x89 => {
            let form = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, form);
        }
        // EXTRA_SCRIPT: the form id, then the script locals.
        0x0d => {
            let form = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, form);
            let locals = word(e, 0x10);
            e.call(SCRIPT_LOCALS_SAVE_GAME, &args![locals, buffer]);
        }
        // EXTRA_MODEL_SWAP.
        0x5b => {
            let form = word(e, 0x10);
            buffer_save_form_id_ov2(e, buffer, form);
            let first = word(e, 0x0c);
            let index = e.call(GET_MODEL_SWAP_INDEX, &args![form, first]).u32();
            buffer_save_local(e, buffer, index, 4);
        }
        // EXTRA_PACKAGE.
        0x19 => {
            let form = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, form);
            let other = word(e, 0x14);
            buffer_save_form_id_ov2(e, buffer, other);
            buffer_save(e, buffer, at(0x10), 4);
            buffer_save(e, buffer, at(0x18), 1);
            buffer_save(e, buffer, at(0x19), 1);
            buffer_save(e, buffer, at(0x1a), 1);
        }
        // EXTRA_PACKAGESTARTLOC.
        0x18 => {
            let form = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, form);
            buffer_save(e, buffer, at(0x10), 0x0c);
            buffer_save(e, buffer, at(0x1c), 4);
        }
        // EXTRA_RUN_ONCE_PACKAGES and the type 0x5E list of the same shape.
        0x1b | 0x5e => {
            let first = word(e, 0x0c);
            save_form_byte_list(e, buffer, first);
        }
        // EXTRA_FOLLOWER: a counted list of form ids.
        0x1d => {
            let mut count = 0u32;
            let start = e
                .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
                .u32();
            let first = word(e, 0x0c);
            walk_list(e, first, |e, item| {
                if item != 0 {
                    buffer_save_form_id_ov2(e, buffer, item);
                    count += 1;
                }
            });
            e.call(
                BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2,
                &args![buffer, count, start],
            );
        }
        // A list of `(form, word, word)` records.
        0x73 => {
            let mut count = 0u32;
            let start = e
                .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
                .u32();
            let first = word(e, 0x0c);
            walk_list(e, first, |e, item| {
                if item != 0 {
                    let form = e.mem.u32(item);
                    buffer_save_form_id_ov2(e, buffer, form);
                    buffer_save(e, buffer, item + 4, 4);
                    buffer_save(e, buffer, item + 8, 4);
                    count += 1;
                }
            });
            e.call(
                BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2,
                &args![buffer, count, start],
            );
        }
        // Two form ids and a byte.
        0x75 => {
            let first = word(e, 0x10);
            buffer_save_form_id_ov2(e, buffer, first);
            let second = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, second);
            buffer_save(e, buffer, at(0x18), 1);
        }
        // A list whose items give two words by `0045cd60` and `00726070`.
        0x35 => {
            let mut count = 0u32;
            let start = e
                .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
                .u32();
            let first = word(e, 0x0c);
            walk_list(e, first, |e, item| {
                if item != 0 {
                    let a = e.call(GET_WORD_28, &args![item]).u32();
                    let b = e.call(SIMPLE_LIST_NEXT, &args![item]).u32();
                    buffer_save_local(e, buffer, a, 4);
                    buffer_save_local(e, buffer, b, 4);
                    count += 1;
                }
            });
            e.call(
                BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2,
                &args![buffer, count, start],
            );
        }
        // EXTRA_ACTION-like: a virtual answer of the object at +0x0C.
        0x70 => {
            let object = word(e, 0x0c);
            let value = if object != 0 {
                e.vcall(object, 8, &args![]).u32()
            } else {
                0xffff_ffff
            };
            buffer_save_local(e, buffer, value, 1);
            if object != 0 {
                e.vcall(object, 0x0c, &args![buffer]);
            }
        }
        // EXTRA_MAPMARKER: the byte `00424940` gives.
        0x2c => {
            let data = word(e, 0x0c);
            let flags = if data != 0 {
                fn_00424940(e, Ptr::new(data))
            } else {
                0
            };
            buffer_save_local(e, buffer, flags as u32, 1);
        }
        // A counted array of form ids.
        0x7c => {
            let array = at(0x0c);
            let count = array_count(e, array);
            e.call(BUFFER_SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
            for index in 0..count {
                let item = e.call(ARRAY_ITEM, &args![array, index]).u32();
                let form = e.mem.u32(item);
                buffer_save_form_id(e, buffer, form);
            }
        }
        // A form id and 4 bytes at +0x10.
        0x6e => {
            let form = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, form);
            buffer_save(e, buffer, at(0x10), 4);
        }
        // A form id and the saved state of the object at +0x0C (virtual +0x54).
        0x1a => {
            let object = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, object);
            if object != 0 {
                e.vcall(object, 0x54, &args![buffer]);
            }
        }
        // EXTRA_LOCK.
        0x2a => {
            let lock_data = word(e, 0x0c);
            buffer_save(e, buffer, lock_data, 1);
            buffer_save(e, buffer, lock_data + 8, 1);
            let key = e.mem.u32(lock_data + 4);
            buffer_save_form_id_ov2(e, buffer, key);
            buffer_save(e, buffer, lock_data + 0x0c, 4);
            buffer_save(e, buffer, lock_data + 0x10, 4);
        }
        // 4 bytes and a byte.
        0x2f => {
            buffer_save(e, buffer, at(0x0c), 4);
            buffer_save(e, buffer, at(0x10), 1);
        }
        // Magic caster/target forms and the form at +0x20.
        0x32 => {
            let caster = word(e, 0x18);
            let caster_id = if caster != 0 {
                e.call(GET_MAGIC_ITEM_FORM_ID, &args![caster]).u32()
            } else {
                0
            };
            buffer_save_form_id(e, buffer, caster_id);
            let target = word(e, 0x1c);
            let target_id = if target != 0 {
                e.call(GET_MAGIC_TARGET_FORM_ID, &args![target]).u32()
            } else {
                0
            };
            buffer_save_form_id(e, buffer, target_id);
            let form = word(e, 0x20);
            buffer_save_form_id_ov2(e, buffer, form);
        }
        // The form at +0x1C and the active effects at +0x20.
        0x33 => {
            let form = word(e, 0x1c);
            buffer_save_form_id_ov2(e, buffer, form);
            e.call(ACTIVE_EFFECT_SAVE_LIST, &args![buffer, at(0x20)]);
        }
        // EXTRA_POISON: the form id of the magic item at +0x30 of the form.
        0x3f => {
            let form = word(e, 0x0c);
            let id = if form != 0 {
                e.call(GET_MAGIC_ITEM_FORM_ID, &args![form + 0x30]).u32()
            } else {
                0
            };
            buffer_save_form_id(e, buffer, id);
        }
        // EXTRA_INFO_GENERAL_TOPIC.
        0x4d => {
            let topic = word(e, 0x0c);
            e.call(MENU_TOPIC_SAVE_GAME_OV2, &args![topic, buffer]);
        }
        // EXTRA_TELEPORT.
        0x2b => {
            let data = word(e, 0x0c);
            e.call(DOOR_TELEPORT_DATA_SAVE_GAME_OV2, &args![data, buffer]);
        }
        // A handle: the word `00825c00` gives for its object, or -1.
        0x60 => {
            let handle = at(0x0c);
            let object = e.call(READ_WORD, &args![handle]).u32();
            let value = if object != 0 {
                let again = e.call(READ_WORD, &args![handle]).u32();
                e.call(HANDLE_TARGET_WORD, &args![again]).u32()
            } else {
                0xffff_ffff
            };
            buffer_save_local(e, buffer, value, 4);
        }
        // EXTRA_ASHPILE-like: the form ids and the state of the object at
        // +0x10 saved with the buffer's word at +0x17 swapped.
        0x2e => {
            let first = word(e, 0x0c);
            buffer_save_form_id_ov2(e, buffer, first);
            let second = word(e, 0x10);
            buffer_save_form_id_ov2(e, buffer, second);
            let object = e.call(0x0041_81e0, &args![flag]).u32();
            let marker = e.call(GET_WORD_28, &args![object + 0x30]).u32();
            buffer_save_local(e, buffer, marker, 4);
            let saved = e.with_stack(4, |e, out| {
                fn_00428110(e, Ptr::new(buffer), out);
                e.mem.u32(out.addr())
            });
            fn_00428130(e, Ptr::new(buffer), marker);
            e.vcall(object, 0x54, &args![buffer]);
            fn_00428130(e, Ptr::new(buffer), saved);
        }
        // 4 bytes at +0x14.
        0x54 => buffer_save(e, buffer, at(0x14), 4),
        // Two bytes.
        0x50 => {
            buffer_save(e, buffer, at(0x0c), 1);
            buffer_save(e, buffer, at(0x0d), 1);
        }
        // 12 bytes, a value, 4 bytes and a list of records.
        0x8b => {
            buffer_save(e, buffer, at(0x10), 0x0c);
            let value = word(e, 0x1c);
            buffer_save_form_id(e, buffer, value);
            buffer_save(e, buffer, at(0x0c), 4);
            let mut count = 0u32;
            let start = e
                .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
                .u32();
            walk_list(e, at(0x20), |e, item| {
                if item != 0 {
                    buffer_save(e, buffer, item, 0x0c);
                    let first = e.mem.u32(item + 0x0c);
                    buffer_save_form_id(e, buffer, first);
                    buffer_save(e, buffer, item + 0x10, 0x0c);
                    let second = e.mem.u32(item + 0x1c);
                    buffer_save_form_id(e, buffer, second);
                    buffer_save(e, buffer, item + 0x20, 1);
                    count += 1;
                }
            });
            e.call(
                BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2,
                &args![buffer, count, start],
            );
        }
        // Two strings.
        0x8f => {
            for second in [false, true] {
                e.with_stack(8, |e, text| {
                    let copy = if second {
                        fn_004280b0(e, Ptr::new(extra), text.addr())
                    } else {
                        fn_00428070(e, Ptr::new(extra), text.addr())
                    };
                    let characters = e.call(READ_WORD, &args![copy]).u32();
                    e.call(BUFFER_SAVE_STRING, &args![buffer, characters, 0u32]);
                    e.call(STRING_DESTROY, &args![text]);
                });
            }
        }
        // A form id from the getter at +0x0C and 4 bytes at +0x10.
        0x92 => {
            let id = form_id(e, extra);
            buffer_save_local(e, buffer, id, 4);
            buffer_save(e, buffer, at(0x10), 4);
        }
        _ => {
            e.call(
                SAVE_GAME_WARNING,
                &args![SAVE_GAME_OV2_UNKNOWN_TYPE_FORMAT, extra_type as u32],
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Seventh block: constructors and destructors of the extra data that follow

/// The part every constructor here shares: the `BSExtraData` base with the
/// type (`0040ec80`), then the vtable of the subclass. Returns `this`.
fn construct_extra(e: &mut Engine, this: Ptr, extra_type: u8, vtable: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, extra_type as u32]);
    e.mem.set_u32(this.addr(), vtable);
    this
}

// Translated from 0042c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a structure of four zeroed flag bytes followed by a member
/// that `0042f570` initializes (at +4). Returns `this`.
pub fn fn_0042c470(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in 0..4 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.call(FLAGS_MEMBER_INIT, &args![this.addr() + 4]);
    this
}

// Translated from 0042c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraLock` (type `0x2A`, vtable `0101589c`): builds the
/// 0x14-byte lock record (`00411b00`) that the extra data holds at +0x0C.
/// The compiler's exception-unwinding frame is not translated. Returns `this`.
pub fn fn_0042c4b0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_LOCK, VTABLE_EXTRA_LOCK);
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let record = if block == 0 {
        0
    } else {
        e.call(LOCK_RECORD_INIT, &args![block]).u32()
    };
    e.mem.set_u32(this.addr() + 0x0c, record);
    this
}

// Translated from 0042c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLock::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// (`00430ca0`), then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn extra_lock_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_LOCK_DESTROY, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042c580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraTeleport` (type `0x2B`, vtable `010158a8`): the
/// teleport data at +0x0C is null. Returns `this`.
pub fn fn_0042c580(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_TELEPORT, VTABLE_EXTRA_TELEPORT);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTeleport::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// (`00431260`), then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn extra_teleport_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_TELEPORT_DESTROY, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x21` extra data (`EXTRA_OWNERSHIP`, vtable
/// `010158b4`): the owner at +0x0C is null. Returns `this`.
pub fn fn_0042c5e0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_OWNERSHIP, VTABLE_EXTRA_OWNERSHIP);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x22` extra data (`EXTRA_GLOBAL`, vtable
/// `010158c0`): the word at +0x0C is 0. Returns `this`.
pub fn fn_0042c610(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_GLOBAL, VTABLE_EXTRA_GLOBAL);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x23` extra data (`EXTRA_RANK`, vtable
/// `010158cc`): the word at +0x0C is 0. Returns `this`.
pub fn fn_0042c640(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_RANK, VTABLE_EXTRA_RANK);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x24` extra data (`EXTRA_COUNT`, vtable
/// `010158d8`): the 16-bit count at +0x0C is 0. Returns `this`.
pub fn fn_0042c670(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_COUNT, VTABLE_EXTRA_COUNT);
    e.mem.set_u16(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x25` extra data (`EXTRA_HEALTH`, vtable
/// `010158e4`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042c6a0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_HEALTH, VTABLE_EXTRA_HEALTH);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x26` extra data (`EXTRA_USES`, vtable
/// `010158f0`): the byte at +0x0C is 0. Returns `this`.
pub fn fn_0042c6d0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_USES, VTABLE_EXTRA_USES);
    e.mem.set_u8(this.addr() + 0x0c, 0);
    this
}

// ---------------------------------------------------------------------------
// Eighth block: the loader of the buffered save format

/// Loads `size` bytes of the buffer to `address`.
fn load_bytes_ov2(e: &mut Engine, buffer: u32, address: u32, size: u32) {
    e.call(LOAD_BYTES, &args![buffer, address, size]);
}

/// `LoadFormID` of the buffer: the id of the next form.
fn load_form_id(e: &mut Engine, buffer: u32) -> u32 {
    e.call(LOAD_FORM_ID, &args![buffer]).u32()
}

/// The version byte of the buffer (virtual `+0`).
fn buffer_version(e: &mut Engine, buffer: u32) -> u8 {
    e.vcall(buffer, 0, &args![]).u8()
}

/// Runs the constructor at `address` on the block. The constructors of this
/// file are called directly.
fn construct_extra_at(e: &mut Engine, address: u32, block: u32) -> u32 {
    let this = Ptr::new(block);
    match address {
        0x0042_c470 => fn_0042c470(e, this).addr(),
        0x0042_c4b0 => fn_0042c4b0(e, this).addr(),
        0x0042_c580 => fn_0042c580(e, this).addr(),
        0x0042_c5e0 => fn_0042c5e0(e, this).addr(),
        0x0042_c610 => fn_0042c610(e, this).addr(),
        0x0042_c640 => fn_0042c640(e, this).addr(),
        0x0042_c670 => fn_0042c670(e, this).addr(),
        0x0042_c6a0 => fn_0042c6a0(e, this).addr(),
        0x0042_c6d0 => fn_0042c6d0(e, this).addr(),
        _ => e.call(address, &args![block]).u32(),
    }
}

/// The extra data of `extra_type` for the loader: the one the list has when
/// `present` says so, else a new one (`operator new` of the size and the
/// constructor of the table [`OV2_CONSTRUCTIONS`]) added to the list
/// (`AddExtra` gives the extra data back).
fn ov2_get_or_create(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    present: &[u8],
    extra_type: u8,
) -> u32 {
    if present[extra_type as usize] != 0 {
        return find_extra(e, list, extra_type).addr();
    }
    let &(_, size, construct) = OV2_CONSTRUCTIONS
        .iter()
        .find(|entry| entry.0 == extra_type)
        .unwrap_or_else(|| panic!("no constructor for the extra data type {extra_type:#x}"));
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let extra = if block == 0 {
        0
    } else {
        construct_extra_at(e, construct, block)
    };
    e.call(ADD_EXTRA, &args![list, extra]).u32()
}

/// Pushes the item at the address `item` holds onto the head of the list at
/// `list` (`BSSimpleList::AddHead` is given the address of a stack slot).
fn list_add_head_of(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD_HEAD, &args![list, slot]);
    });
}

// Translated from 00428150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::LoadGame_ov2` (Xbox PDB): loads the list from the
/// `BGSLoadGameBuffer` `buffer`. The buffer is asked for its reference
/// (virtual `+8`) and its owner (virtual `+0x0C`). Before reading, the types
/// the list already has that the buffer's save kind lets through (the table
/// at `01183d30`, `00428110`, `004280f0`) are marked; then a counted value
/// holds, for each extra data, its type byte and payload, which the arm of its
/// type reads into the extra data of the list (the marked one, or a new one
/// built and added); the mark of the type is cleared after each. At the end
/// the types still marked, which the buffer did not load, are removed from the
/// list, except type `0x2E` (kept) and type `0x92` (its special render flags
/// are set from the form id of the extra data, then it is kept too). A type
/// outside the table gets a warning.
pub fn extra_data_list_load_game_ov2(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32) {
    let reference = e.vcall(buffer, 8, &args![]).u32();
    let owner = e.vcall(buffer, 0x0c, &args![]).u32();
    let mut present = [0u8; 0x93];
    let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let extra_type = get_type(e, current);
        let kinds = e
            .mem
            .u32(SAVE_KIND_TABLE.wrapping_add(extra_type as u32 * 4));
        if kinds != 0 {
            let allowed = e.with_stack(4, |e, out| {
                let out_value = fn_00428110(e, Ptr::new(buffer), out);
                fn_004280f0(e, out_value, kinds)
            });
            if allowed {
                present[extra_type as usize] = 1;
            }
        }
        current = get_next(e, current);
    }
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        let extra_type = e.with_stack(4, |e, slot| {
            load_bytes_ov2(e, buffer, slot.addr(), 1);
            e.mem.u8(slot.addr())
        });
        load_extra_ov2(e, this, buffer, reference, owner, &present, extra_type);
        if let Some(mark) = present.get_mut(extra_type as usize) {
            *mark = 0;
        }
    }
    for extra_type in 0..0x93u32 {
        let mut remove = present[extra_type as usize] != 0;
        if remove {
            if extra_type == 0x2e {
                remove = false;
            } else if extra_type == 0x92 {
                let extra = find_extra(e, this, 0x92);
                let special = extra_cast(e, extra, RTTI_TYPE_92_EXTRA);
                if special != 0 {
                    let id = form_id(e, special);
                    e.call(SET_SPECIAL_RENDER_WORD, &args![special, id & 7]);
                }
                remove = false;
            }
        }
        if remove {
            remove_extra(e, this, extra_type as u8);
        }
    }
}

/// One arm of the buffered loader.
fn load_extra_ov2(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    buffer: u32,
    reference: u32,
    owner: u32,
    present: &[u8; 0x93],
    extra_type: u8,
) {
    match extra_type {
        // EXTRA_WORN
        0x16 => {
            e.call(SET_WORN, &args![this, 1u32, 0u32]);
        }
        // EXTRA_CANNOTWEAR
        0x3e => {
            e.call(SET_CAN_NOT_WEAR, &args![this, 1u32]);
        }
        // A word, a half word or a byte at +0x0C.
        0x24 | 0x25 | 0x30 | 0x23 | 0x8d | 0x4a | 0x5c | 0x5d | 0x1e | 0x26 | 0x27 | 0x28
        | 0x4e => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let size = match extra_type {
                0x24 => 2,
                0x8d | 0x4a | 0x26 | 0x4e => 1,
                _ => 4,
            };
            load_bytes_ov2(e, buffer, extra + 0x0c, size);
        }
        // The form the id names, as it is.
        0x21 | 0x49 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.mem.set_u32(extra + 0x0c, form);
        }
        // The form the id names, cast to the class of the type.
        0x22 | 0x74 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let target = if extra_type == 0x22 {
                RTTI_TES_GLOBAL
            } else {
                RTTI_TES_ENCOUNTER_ZONE
            };
            let form = form_as(e, id, target);
            e.mem.set_u32(extra + 0x0c, form);
        }
        // The form id stored at +0x0C by `LoadFormID_ov2`.
        0x1c | 0x6c | 0x39 | 0x3c | 0x46 | 0x55 | 0x89 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x0c]);
        }
        // 4 bytes at +0x0C, then the destructible object's self damage.
        0x56 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            load_bytes_ov2(e, buffer, extra + 0x0c, 4);
            let damage = f32::from_bits(x87_quiet_bits(e.mem.u32(extra + 0x0c)));
            let target = e
                .call(DESTRUCTIBLE_LOOKUP_00477BC0, &args![reference, damage])
                .u32();
            if target != 0 {
                e.call(SET_SELF_DAMAGE, &args![reference, target]);
            }
        }
        // EXTRA_SCRIPT: the script the id names, and its locals.
        0x0d => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let script = form_as(e, id, RTTI_TES_SCRIPT);
            e.mem.set_u32(extra + 0x0c, script);
            if e.mem.u32(extra + 0x10) == 0 {
                let locals = if script != 0 {
                    e.call(SCRIPT_CREATE_LOCALS, &args![script]).u32()
                } else {
                    0
                };
                e.mem.set_u32(extra + 0x10, locals);
            }
            if reference == 0 {
                let packing = e.call(MENU_PACKING_VALUE, &args![]).u32();
                e.call(MENU_PACKER_RECOMPUTE, &args![buffer, packing]);
            }
            let locals = e.mem.u32(extra + 0x10);
            if locals == 0 {
                e.with_stack(0x14, |e, temporary| {
                    e.call(SCRIPT_LOCALS_INIT, &args![temporary]);
                    e.call(SCRIPT_LOCALS_LOAD_GAME, &args![temporary, buffer]);
                    e.call(SCRIPT_LOCALS_DESTROY, &args![temporary]);
                });
            } else {
                e.call(SCRIPT_LOCALS_LOAD_GAME, &args![locals, buffer]);
            }
            if reference == 0 {
                e.call(MENU_PACKER_RECOMPUTE, &args![buffer, 0u32]);
            }
        }
        // EXTRA_MODEL_SWAP: the form, then the index in it.
        0x5b => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.mem.set_u32(extra + 0x10, form);
            let index = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 0xffff_ffff);
                load_bytes_ov2(e, buffer, slot.addr(), 4);
                e.mem.u32(slot.addr())
            });
            if form != 0 {
                let swap = e
                    .call(MODEL_SWAP_FROM_INDEX_004759A0, &args![form, index])
                    .u32();
                e.mem.set_u32(extra + 0x0c, swap);
            }
            if e.mem.u32(extra + 0x10) != 0 && e.mem.u32(extra + 0x0c) != 0 {
                if reference != 0 {
                    e.vcall(reference, 0x1cc, &args![0u32, 1u32]);
                }
            } else {
                remove_extra(e, this, 0x5b);
            }
        }
        // EXTRA_PACKAGE
        0x19 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x0c]);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x14]);
            load_bytes_ov2(e, buffer, extra + 0x10, 4);
            load_bytes_ov2(e, buffer, extra + 0x18, 1);
            load_bytes_ov2(e, buffer, extra + 0x19, 1);
            load_bytes_ov2(e, buffer, extra + 0x1a, 1);
        }
        // EXTRA_PACKAGESTARTLOC
        0x18 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.mem.set_u32(extra + 0x0c, form);
            load_bytes_ov2(e, buffer, extra + 0x10, 0x0c);
            load_bytes_ov2(e, buffer, extra + 0x1c, 4);
        }
        // EXTRA_RUN_ONCE_PACKAGES: the packages are added to the list.
        0x1b => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let item = e.call(OPERATOR_NEW, &args![8u32]).u32();
                let id = load_form_id(e, buffer);
                let package = form_as(e, id, RTTI_TES_PACKAGE);
                e.mem.set_u32(item, package);
                load_bytes_ov2(e, buffer, item + 4, 1);
                let list = e.mem.u32(extra + 0x0c);
                list_add_head_of(e, list, item);
            }
        }
        // EXTRA_FOLLOWER: the forms go in as `LoadFormID_ov2` gives them.
        0x1d => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let form = e.with_stack(4, |e, slot| {
                    e.call(LOAD_FORM_ID_OV2, &args![buffer, slot]);
                    e.mem.u32(slot.addr())
                });
                let list = e.mem.u32(extra + 0x0c);
                list_add_head_of(e, list, form);
            }
        }
        // A list of `(form, word, word)` records.
        0x73 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let block = e.call(OPERATOR_NEW, &args![0x0cu32]).u32();
                let item = if block == 0 {
                    0
                } else {
                    e.call(ITEM_73_INIT, &args![block]).u32()
                };
                let id = load_form_id(e, buffer);
                let form = form_as(e, id, RTTI_TYPE_73_FORM);
                e.mem.set_u32(item, form);
                load_bytes_ov2(e, buffer, item + 4, 4);
                load_bytes_ov2(e, buffer, item + 8, 4);
                let list = e.mem.u32(extra + 0x0c);
                list_add_head_of(e, list, item);
            }
        }
        // Two forms and a byte.
        0x75 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let first = form_as(e, id, RTTI_TYPE_75_FORM);
            e.mem.set_u32(extra + 0x10, first);
            let id = load_form_id(e, buffer);
            let second = form_as(e, id, RTTI_TYPE_73_FORM);
            e.mem.set_u32(extra + 0x0c, second);
            load_bytes_ov2(e, buffer, extra + 0x18, 1);
        }
        // A list of pairs of words that `009724e0` turns into items.
        0x35 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let (first, second) = e.with_stack(8, |e, pair| {
                    e.mem.set_u32(pair.addr(), 0);
                    e.mem.set_u32(pair.addr() + 4, 0xffff_ffff);
                    load_bytes_ov2(e, buffer, pair.addr(), 4);
                    load_bytes_ov2(e, buffer, pair.addr() + 4, 4);
                    (e.mem.u32(pair.addr()), e.mem.u32(pair.addr() + 4))
                });
                let item = e
                    .call(
                        PAIR_LOOKUP_009724E0,
                        &args![PAIR_LOOKUP_OBJECT, second, first],
                    )
                    .u32();
                if item != 0 {
                    let list = e.mem.u32(extra + 0x0c);
                    list_add_head_of(e, list, item);
                } else {
                    let form_id_of_reference = if reference != 0 {
                        form_id(e, reference)
                    } else {
                        0
                    };
                    let name = if reference != 0 {
                        e.vcall(reference, FORM_NAME_SLOT, &args![]).u32()
                    } else {
                        NAME_UNKNOWN_REFERENCE
                    };
                    e.call(
                        SAVE_GAME_WARNING,
                        &args![
                            MESSAGE_PAIR_NOT_FOUND,
                            first,
                            second,
                            name,
                            form_id_of_reference
                        ],
                    );
                }
            }
            let list = e.mem.u32(extra + 0x0c);
            let slot = e.call(SIMPLE_LIST_ITEM, &args![list]).u32();
            if e.mem.u32(slot) == 0 {
                remove_extra(e, this, 0x35);
            }
        }
        // EXTRA_FRIEND_HITS
        0x45 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            e.call(ARRAY_RESIZE, &args![extra + 0x0c, count, 1u32]);
            for index in 0..count {
                let hit = e.call(ARRAY_ITEM, &args![extra + 0x0c, index]).u32();
                e.call(COMBAT_TIME_STAMP_LOAD_GAME, &args![hit, buffer]);
            }
            if e.call(ARRAY_COUNT, &args![extra + 0x0c]).u32() == 0 {
                fn_00422670(e, this);
            }
        }
        // A list of `(form, byte)` records, the ones without a form dropped.
        0x5e => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let item = e.call(OPERATOR_NEW, &args![8u32]).u32();
                let id = load_form_id(e, buffer);
                let form = form_as(e, id, RTTI_TYPE_5E_FORM);
                e.mem.set_u32(item, form);
                load_bytes_ov2(e, buffer, item + 4, 1);
                let list = e.mem.u32(extra + 0x0c);
                list_add_head_of(e, list, item);
            }
            let mut node = e.mem.u32(extra + 0x0c);
            let mut previous = 0u32;
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let item = list_item(e, node);
                let mut remove = false;
                if item != 0 {
                    if e.mem.u32(item) == 0 {
                        e.call(OPERATOR_DELETE, &args![item]);
                        remove = true;
                    }
                } else {
                    remove = true;
                }
                if remove {
                    if previous != 0 {
                        let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                        e.call(LIST_REMOVE_AFTER, &args![previous, slot]);
                        node = list_next(e, previous);
                    } else {
                        e.call(LIST_REMOVE_HEAD, &args![node]);
                    }
                } else {
                    previous = node;
                    node = list_next(e, node);
                }
            }
        }
        // A signed byte, -1 for none, that names an object the extra data
        // builds (`008c6ca0`) and loads (virtual +0x10).
        0x70 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let kind = e.with_stack(4, |e, slot| {
                e.mem.set_u8(slot.addr(), 0xff);
                load_bytes_ov2(e, buffer, slot.addr(), 1);
                e.mem.u8(slot.addr())
            });
            if kind as i8 != -1 {
                let object = e
                    .call(BUILD_OBJECT_OF_KIND, &args![kind as i8 as i32 as u32])
                    .u32();
                e.mem.set_u32(extra + 0x0c, object);
                e.vcall(object, 0x10, &args![buffer]);
            }
        }
        // EXTRA_MAPMARKER: a byte for the marker data the list already holds.
        0x2c => {
            let flags = e.with_stack(4, |e, slot| {
                load_bytes_ov2(e, buffer, slot.addr(), 1);
                e.mem.u8(slot.addr())
            });
            let extra = if present[0x2c] != 0 {
                find_extra(e, this, 0x2c).addr()
            } else {
                0
            };
            if extra != 0 {
                let data = e.mem.u32(extra + 0x0c);
                if data != 0 {
                    e.call(MAP_MARKER_SET_FLAGS, &args![data, flags as u32]);
                }
            }
        }
        // No payload.
        0x90 | 0x91 => {}
        // A counted array of form ids.
        0x7c => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            e.call(ARRAY_RESIZE_OTHER, &args![extra + 0x0c, count, 1u32]);
            for index in 0..count {
                let slot = e.call(ARRAY_ITEM, &args![extra + 0x0c, index]).u32();
                e.call(LOAD_FORM_ID_OV2, &args![buffer, slot]);
            }
        }
        // A form and 4 bytes at +0x10.
        0x6e => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = load_form_id(e, buffer);
            let form = form_as(e, id, RTTI_TYPE_6E_FORM);
            e.mem.set_u32(extra + 0x0c, form);
            load_bytes_ov2(e, buffer, extra + 0x10, 4);
        }
        // A form id, and the object the extra data builds for it.
        0x1a => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let form = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 0);
                e.call(LOAD_FORM_ID_OV2, &args![buffer, slot]);
                e.mem.u32(slot.addr())
            });
            if form != 0 {
                let object = e.call(PACKAGE_FACTORY, &args![0x17u32]).u32();
                e.mem.set_u32(extra + 0x0c, object);
                let id = form_id(e, object);
                let manager = e.global::<u32>(HANDLER_OBJECT);
                e.call(OBJECT_MANAGER_REGISTER, &args![manager, form, id]);
                e.vcall(object, 0x5c, &args![buffer]);
            }
        }
        // EXTRA_GHOST
        0x1f => {
            e.call(SET_GHOST, &args![this, 1u32]);
        }
        // EXTRA_LOCK
        0x2a => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let lock_data = e.mem.u32(extra + 0x0c);
            load_bytes_ov2(e, buffer, lock_data, 1);
            load_bytes_ov2(e, buffer, lock_data + 8, 1);
            let id = load_form_id(e, buffer);
            let key = form_as(e, id, RTTI_TES_KEY);
            e.mem.set_u32(lock_data + 4, key);
            if buffer_version(e, buffer) >= 0x10 {
                load_bytes_ov2(e, buffer, lock_data + 0x0c, 4);
                load_bytes_ov2(e, buffer, lock_data + 0x10, 4);
            }
        }
        // 4 bytes and a byte.
        0x2f => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            load_bytes_ov2(e, buffer, extra + 0x0c, 4);
            load_bytes_ov2(e, buffer, extra + 0x10, 1);
        }
        // Three forms.
        0x32 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x18]);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x1c]);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x20]);
        }
        // A form and the active effects at +0x20.
        0x33 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x1c]);
            e.call(ACTIVE_EFFECT_LOAD_LIST, &args![buffer, extra + 0x20]);
        }
        // EXTRA_POISON: the form, then the magic item it is cast to.
        0x3f => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x0c]);
            let form = e.mem.u32(extra + 0x0c);
            let source = e.call(MAGIC_ITEM_FROM_FORM, &args![form]).u32();
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![source, 0u32, RTTI_TYPE_3F_SOURCE, RTTI_TYPE_3F_TARGET, 0u32],
                )
                .u32();
            e.mem.set_u32(extra + 0x0c, cast);
        }
        // EXTRA_INFO_GENERAL_TOPIC
        0x4d => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let topic = e.mem.u32(extra + 0x0c);
            e.call(MENU_TOPIC_LOAD_GAME_OV2, &args![topic, buffer]);
        }
        // EXTRA_TELEPORT
        0x2b => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            if e.mem.u32(extra + 0x0c) == 0 {
                let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let data = if block == 0 {
                    0
                } else {
                    e.call(DOOR_TELEPORT_DATA_INIT, &args![block]).u32()
                };
                e.mem.set_u32(extra + 0x0c, data);
            }
            let data = e.mem.u32(extra + 0x0c);
            e.call(DOOR_TELEPORT_DATA_LOAD_GAME_OV2, &args![data, buffer]);
        }
        // EXTRA_HANDLE-like: a 4-byte id turned into a handle (`0066e330`).
        0x60 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let id = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 0xffff_ffff);
                load_bytes_ov2(e, buffer, slot.addr(), 4);
                e.mem.u32(slot.addr())
            });
            let handle = e.call(HANDLE_OF_ID, &args![id]).u32();
            e.call(ASSIGN_HANDLE_AT, &args![extra + 0x0c, handle]);
            if e.call(READ_WORD, &args![extra + 0x0c]).u32() == 0 {
                remove_extra(e, this, 0x60);
            }
        }
        // 4 bytes at +0x14 (or dropped when the list has no such extra data).
        0x54 => {
            let extra = if present[0x54] != 0 {
                find_extra(e, this, 0x54).addr()
            } else {
                0
            };
            if extra != 0 {
                load_bytes_ov2(e, buffer, extra + 0x14, 4);
            } else {
                e.with_stack(4, |e, slot| load_bytes_ov2(e, buffer, slot.addr(), 4));
            }
        }
        // Two bytes.
        0x50 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            load_bytes_ov2(e, buffer, extra + 0x0c, 1);
            load_bytes_ov2(e, buffer, extra + 0x0d, 1);
        }
        // 12 bytes, a form, 4 bytes, and a list of records.
        0x8b => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            load_bytes_ov2(e, buffer, extra + 0x10, 0x0c);
            if buffer_version(e, buffer) >= 0x12 {
                e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x1c]);
            } else {
                load_bytes_ov2(e, buffer, extra + 0x1c, 4);
            }
            load_bytes_ov2(e, buffer, extra + 0x0c, 4);
            let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
                let item = if block == 0 {
                    0
                } else {
                    e.call(ITEM_8B_INIT, &args![block]).u32()
                };
                load_bytes_ov2(e, buffer, item, 0x0c);
                if buffer_version(e, buffer) >= 0x12 {
                    e.call(LOAD_FORM_ID_OV2, &args![buffer, item + 0x0c]);
                } else {
                    load_bytes_ov2(e, buffer, item + 0x0c, 4);
                }
                load_bytes_ov2(e, buffer, item + 0x10, 0x0c);
                if buffer_version(e, buffer) >= 0x12 {
                    e.call(LOAD_FORM_ID_OV2, &args![buffer, item + 0x1c]);
                } else {
                    load_bytes_ov2(e, buffer, item + 0x1c, 4);
                }
                load_bytes_ov2(e, buffer, item + 0x20, 1);
                let list = extra + 0x20;
                list_add_head_of(e, list, item);
            }
        }
        // Two strings, loaded into a local buffer and given to the extra data.
        0x8f => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            e.with_stack(0x100, |e, text| {
                e.call(LOAD_STRING, &args![buffer, text]);
                e.call(EXTRA_SET_FIRST_STRING, &args![extra, text]);
                e.call(LOAD_STRING, &args![buffer, text]);
                e.call(EXTRA_SET_SECOND_STRING, &args![extra, text]);
            });
        }
        // EXTRA_SPECIAL_RENDER_FLAGS: the word is set through `0041fd00`.
        0x92 => {
            let extra = ov2_get_or_create(e, this, present, extra_type);
            let flags = e.with_stack(4, |e, slot| {
                load_bytes_ov2(e, buffer, slot.addr(), 4);
                e.mem.u32(slot.addr())
            });
            load_bytes_ov2(e, buffer, extra + 0x10, 4);
            e.call(SET_SPECIAL_RENDER_WORD, &args![extra, flags]);
        }
        // EXTRA_DISMEMBERED_LIMBS: the one the list has is detached and
        // replaced by the loaded one.
        0x5f => load_dismembered_limbs(e, this, buffer, owner, present),
        // EXTRA_LEVELEDCREATURE
        0x2e => load_leveled_creature(e, this, buffer, owner, present),
        _ => {
            e.call(
                SAVE_GAME_WARNING,
                &args![LOAD_GAME_OV2_UNKNOWN_TYPE_FORMAT, extra_type as u32],
            );
        }
    }
}

/// The type `0x5F` arm (`EXTRA_DISMEMBERED_LIMBS`): the extra data the list
/// has is detached (`RemoveExtra(extra, false)`); when the list had none, the
/// buffer's own question (`0042ce30`, `004280f0` with `0x20000`) decides
/// whether the owner is told (`virtual +0x1d0`, then `008b6820`). A new
/// 0x30-byte extra data is built and loaded: 2, 4, 4 and 1 bytes, a form id,
/// and an array of records of 4 flag bytes (the last one only from buffer
/// version `0x10`) and an array of form ids, the ids turned into forms
/// (and the entries without a form dropped). The detached one is compared
/// with the new one (`00430660`); then it is deleted, the new one added and
/// the owner told (`008b6ae0`) when they differ.
fn load_dismembered_limbs(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    buffer: u32,
    owner: u32,
    present: &[u8],
) {
    let existing = if present[0x5f] != 0 {
        find_extra(e, this, 0x5f).addr()
    } else {
        0
    };
    if existing != 0 {
        e.call(REMOVE_EXTRA_OBJECT, &args![this, existing, 0u32]);
    } else {
        let asks = e.with_stack(4, |e, out| {
            let out_value = e.call(BUFFER_FIELD_COPY_42CE30, &args![buffer, out]).u32();
            fn_004280f0(e, Ptr::new(out_value), 0x2_0000)
        });
        if asks && e.vcall(owner, 0x1d0, &args![]).u32() != 0 {
            e.call(OWNER_NOTIFY_BEGIN, &args![owner]);
        }
    }
    let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    let extra = if block == 0 {
        0
    } else {
        e.call(DISMEMBERED_LIMBS_INIT, &args![block]).u32()
    };
    load_bytes_ov2(e, buffer, extra + 0x0c, 2);
    load_bytes_ov2(e, buffer, extra + 0x10, 4);
    load_bytes_ov2(e, buffer, extra + 0x18, 4);
    load_bytes_ov2(e, buffer, extra + 0x1c, 1);
    e.call(LOAD_FORM_ID_OV2, &args![buffer, extra + 0x14]);
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let records = extra + 0x20;
    e.call(ARRAY_RESIZE_OTHER, &args![records, count, 1u32]);
    for index in 0..count {
        let slot = e.call(ARRAY_ITEM, &args![records, index]).u32();
        let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
        let entry = if block == 0 {
            0
        } else {
            fn_0042c470(e, Ptr::new(block)).addr()
        };
        e.mem.set_u32(slot, entry);
        load_bytes_ov2(e, buffer, entry, 1);
        load_bytes_ov2(e, buffer, entry + 1, 1);
        load_bytes_ov2(e, buffer, entry + 2, 1);
        if buffer_version(e, buffer) >= 0x10 {
            load_bytes_ov2(e, buffer, entry + 3, 1);
        }
        let inner = entry + 4;
        let inner_count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        e.call(ARRAY_RESIZE_OTHER, &args![inner, inner_count, 1u32]);
        for inner_index in 0..inner_count {
            let inner_slot = e.call(ARRAY_ITEM, &args![inner, inner_index]).u32();
            e.call(LOAD_FORM_ID_OV2, &args![buffer, inner_slot]);
        }
        let mut remaining = e.call(ARRAY_COUNT, &args![inner]).u32();
        let mut inner_index = 0u32;
        while inner_index < remaining {
            let inner_slot = e.call(ARRAY_ITEM, &args![inner, inner_index]).u32();
            let id = e.mem.u32(inner_slot);
            let form = if id != 0 {
                form_as(e, id, RTTI_TYPE_DISMEMBERED_FORM)
            } else {
                0
            };
            e.mem.set_u32(inner_slot, form);
            if form == 0 {
                e.call(ARRAY_REMOVE_AT, &args![inner, inner_index, 1u32]);
                inner_index = inner_index.wrapping_sub(1);
                remaining -= 1;
            }
            inner_index = inner_index.wrapping_add(1);
        }
    }
    let mut changed = false;
    if existing != 0 {
        if e.call(DISMEMBERED_LIMBS_COMPARE, &args![existing, extra])
            .bool()
        {
            changed = true;
        } else {
            e.call(OWNER_NOTIFY_BEGIN, &args![owner]);
        }
        e.vcall(existing, 0, &args![1u32]);
    }
    e.call(ADD_EXTRA, &args![this, extra]);
    if !changed {
        e.call(OWNER_NOTIFY_END, &args![owner]);
    }
}

/// The type `0x2E` arm (`EXTRA_LEVELEDCREATURE`, 0x14 bytes: the base form at
/// +0x0C and the template at +0x10): both forms are read as ids and cast, and
/// 4 bytes follow. The owner is told when the template changes
/// (virtual `+0x1cc`), then, when the owner, both forms and the form's flag
/// (`0047cdb0` of the base form's member at +0x30) say so, a creature is made
/// for the pair (`0047d130`); otherwise the old forms are put back, or the
/// extra data is removed. A creature that does not exist yet is built (a
/// 0x20C-byte object for an actor of the kind virtual `+0x218` says, else a
/// 0x160-byte one) and loaded with the buffer's fields swapped for the
/// duration (virtual `+0x5c`, `00428130`, `0086cf00`, `0042ce50`).
fn load_leveled_creature(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    buffer: u32,
    owner: u32,
    present: &[u8],
) {
    let extra = ov2_get_or_create(e, this, present, 0x2e);
    let old_base = e.mem.u32(extra + 0x0c);
    let old_template = e.mem.u32(extra + 0x10);
    let id = load_form_id(e, buffer);
    let base = form_as(e, id, RTTI_TYPE_2E_FORM);
    e.mem.set_u32(extra + 0x0c, base);
    let id = load_form_id(e, buffer);
    let template = form_as(e, id, RTTI_TYPE_2E_FORM);
    e.mem.set_u32(extra + 0x10, template);
    let marker = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        load_bytes_ov2(e, buffer, slot.addr(), 4);
        e.mem.u32(slot.addr())
    });
    let template_changed = owner != 0
        && e.vcall(owner, 0x1d0, &args![]).u32() != 0
        && old_template != 0
        && template != old_template;
    if template_changed {
        e.vcall(owner, 0x1cc, &args![0u32, 0u32]);
    } else {
        let handler = e.global::<u32>(HANDLER_OBJECT);
        if !e.call(HANDLER_OBJECT_QUERY, &args![handler]).bool() {
            e.vcall(owner, 0x1cc, &args![0u32, 0u32]);
        }
    }
    let mut creature = 0u32;
    let mut built = false;
    let mut kind_not_allowed = false;
    if old_template != e.mem.u32(extra + 0x10) {
        let allowed = e.with_stack(4, |e, out| {
            let out_value = fn_00428110(e, Ptr::new(buffer), out);
            fn_004280f0(e, out_value, 0x0800_0020)
        });
        if !allowed {
            kind_not_allowed = true;
        }
    }
    if kind_not_allowed {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![owner]).u32();
        e.call(0x0041_aeb0, &args![list]);
        e.call(OWNER_REFRESH, &args![owner]);
    }
    let base = e.mem.u32(extra + 0x0c);
    let template = e.mem.u32(extra + 0x10);
    if owner != 0
        && base != 0
        && template != 0
        && e.call(FORM_CAN_BE_LEVELED, &args![base + 0x30]).bool()
    {
        creature = e.call(MAKE_LEVELED_CREATURE, &args![base, template]).u32();
        let actor_base = e.call(0x0041_81e0, &args![owner]).u32();
        let id = form_id(e, actor_base);
        let handler = e.global::<u32>(DATA_HANDLER);
        if e.call(FORM_ID_IS_FILE_FORM, &args![handler, id]).bool() && actor_base != 0 {
            e.vcall(actor_base, 0x10, &args![1u32]);
        }
        e.call(OWNER_SET_BASE, &args![owner, creature]);
    } else if old_base != 0 && old_template != 0 {
        e.mem.set_u32(extra + 0x0c, old_base);
        e.mem.set_u32(extra + 0x10, old_template);
    } else {
        remove_extra(e, this, 0x2e);
    }
    if kind_not_allowed {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![owner]).u32();
        e.call(0x004d_1440, &args![changes]);
        e.call(0x004d_1610, &args![changes]);
        e.call(0x004d_1960, &args![changes]);
        if e.call(0x0042_cde0, &args![changes]).bool() {
            let list = e.call(REFERENCE_EXTRA_LIST, &args![owner]).u32();
            e.call(0x0041_aeb0, &args![list]);
        }
    }
    if creature == 0 {
        let is_actor_kind = e.vcall(owner, 0x218, &args![]).bool();
        let (size, construct) = if is_actor_kind {
            (0x20c, CREATURE_ACTOR_INIT)
        } else {
            (0x160, CREATURE_INIT)
        };
        let block = e.call(OPERATOR_NEW, &args![size]).u32();
        creature = if block == 0 {
            0
        } else {
            e.call(construct, &args![block]).u32()
        };
        built = true;
    }
    // The buffer's three fields are replaced while the creature loads.
    let saved_kind = e.with_stack(4, |e, out| {
        fn_00428110(e, Ptr::new(buffer), out);
        e.mem.u32(out.addr())
    });
    let saved_second = e.with_stack(4, |e, out| {
        e.call(BUFFER_FIELD_COPY_42CE30, &args![buffer, out]);
        e.mem.u32(out.addr())
    });
    let saved_flag = e.call(BUFFER_FLAG_42CE90, &args![buffer]).u8();
    fn_00428130(e, Ptr::new(buffer), marker);
    e.call(BUFFER_SET_FIELD_86CF00, &args![buffer, 0u32]);
    e.call(BUFFER_SET_FLAG_42CE50, &args![buffer, 0u32]);
    e.vcall(creature, 0x5c, &args![buffer]);
    e.call(CREATURE_SET_MARKER, &args![creature + 0x30, marker]);
    fn_00428130(e, Ptr::new(buffer), saved_kind);
    e.call(BUFFER_SET_FIELD_86CF00, &args![buffer, saved_second]);
    e.call(BUFFER_SET_FLAG_42CE50, &args![buffer, saved_flag as u32]);
    if built && creature != 0 {
        e.vcall(creature, 0x10, &args![1u32]);
    }
}
// ---------------------------------------------------------------------------
// Ninth block: constructors and destructors of the extra data of the types
// 0x0D to 0x8F, the helpers of the save buffer, and the fix-ups that follow a
// load

/// Vtables the constructors of this block set.
const VTABLE_EXTRA_TIME_LEFT: u32 = 0x0101_58fc;
const VTABLE_EXTRA_CHARGE: u32 = 0x0101_5908;
const VTABLE_EXTRA_SCRIPT: u32 = 0x0101_5914;
const VTABLE_EXTRA_SCALE: u32 = 0x0101_5920;
const VTABLE_EXTRA_HOT_KEY: u32 = 0x0101_592c;
const VTABLE_EXTRA_REFERENCE_POINTER: u32 = 0x0101_5938;
const VTABLE_EXTRA_TRESPASS_PACKAGE: u32 = 0x0101_5944;
const VTABLE_EXTRA_LEVELED_ITEM: u32 = 0x0101_5950;
const VTABLE_EXTRA_POISON: u32 = 0x0101_595c;
const VTABLE_EXTRA_HEAD_TRACK_TARGET: u32 = 0x0101_5968;
const VTABLE_EXTRA_NO_RUMORS: u32 = 0x0101_5974;
const VTABLE_EXTRA_OBJECT_HEALTH: u32 = 0x0101_5184;
const VTABLE_EXTRA_MODEL_SWAP: u32 = 0x0101_5980;
const VTABLE_EXTRA_ACTOR_CAUSE: u32 = 0x0101_598c;
const VTABLE_EXTRA_AMMO: u32 = 0x0101_5998;
const VTABLE_EXTRA_PACKAGE_DATA: u32 = 0x0101_51fc;
const VTABLE_EXTRA_WEAPON_MOD_SLOTS: u32 = 0x0101_59a4;
const VTABLE_EXTRA_SECURITRON_FACE: u32 = 0x0101_59b0;

/// Extra data types constructed here (`EXTRA_DATA_TYPE`, Xbox PDB).
const EXTRA_TIMELEFT: u8 = 0x27;
const EXTRA_SCRIPT: u8 = 0x0d;
const EXTRA_HOT_KEY: u8 = 0x4a;
const EXTRA_REFERENCE_POINTER: u8 = 0x1c;
const EXTRA_TRESPASS_PACKAGE: u8 = 0x1a;
const EXTRA_LEVELITEM: u8 = 0x2f;
const EXTRA_HEAD_TRACK_TARGET: u8 = 0x46;
const EXTRA_OBJECT_HEALTH: u8 = 0x56;
const EXTRA_MODEL_SWAP: u8 = 0x5b;
const EXTRA_ACTOR_CAUSE: u8 = 0x60;
const EXTRA_AMMO: u8 = 0x6e;
const EXTRA_PACKAGE_DATA: u8 = 0x70;
const EXTRA_WEAPON_MOD_SLOTS: u8 = 0x8d;
const EXTRA_SECURITRON_FACE: u8 = 0x8f;
/// The other types the fix-ups read.
const EXTRA_MAGICCASTER: u8 = 0x32;
const EXTRA_MAGICTARGET: u8 = 0x33;
const EXTRA_ITEMDROPPER: u8 = 0x39;
const EXTRA_MERCHANTCONTAINER: u8 = 0x3c;
const EXTRA_PACKAGE: u8 = 0x19;
const EXTRA_TALKING_ACTOR: u8 = 0x55;
const EXTRA_DISMEMBERED_LIMBS: u8 = 0x5f;
const EXTRA_OPENCLOSEACTIVATE_REF: u8 = 0x6c;
const EXTRA_SAY_TO_TOPIC_INFO: u8 = 0x75;
const EXTRA_ASHPILE_REF: u8 = 0x89;
const EXTRA_PLAYERCRIMELIST: u8 = 0x35;

/// Destructors called by the scalar deleting destructors of this block
/// (`this` the extra data): `ExtraScript` (`00432040`),
/// `ExtraTresPassPackage` (`00432900`), `ExtraHeadingTarget` (`00435ef0`).
const EXTRA_SCRIPT_DESTROY: u32 = 0x0043_2040;
const EXTRA_TRESPASS_PACKAGE_DESTROY: u32 = 0x0043_2900;
const EXTRA_HEADING_TARGET_DESTROY: u32 = 0x0043_5ef0;
/// Member of the actor cause extra data at +0x0C: `0042f730(this, 0)` builds
/// it and `0042f760(this)` lets go of it (the destructor body); the value is
/// assigned with `0042f780` ([`ASSIGN_HANDLE_AT`]).
const ACTOR_CAUSE_MEMBER_INIT: u32 = 0x0042_f730;
const ACTOR_CAUSE_MEMBER_RELEASE: u32 = 0x0042_f760;
/// String members at +0x0C and +0x14 of the securitron face extra data:
/// the constructor (`004037b0`) and the destructor (`004037d0`).
const STRING_MEMBER_INIT: u32 = 0x0040_37b0;
const STRING_MEMBER_DESTROY: u32 = 0x0040_37d0;
/// The `float` that `0042cc10` copies (`01012054`).
const FLOAT_COPIED_BY_RECORD_INIT: u32 = 0x0101_2054;
/// The word `0042ce00` gives (`011c6444`).
const WORD_GIVEN_BY_GETTER: u32 = 0x011c_6444;

/// `__RTDynamicCast` targets used by the fix-up: the type descriptors of
/// `TESScriptableForm` and `MobileObject`.
const RTTI_TES_SCRIPTABLE_FORM: u32 = 0x0118_3254;
const RTTI_MOBILE_OBJECT: u32 = 0x0118_4920;

/// `ExtraDataList::GetModelSwap` (`0042e250`), `GetItemDropper`
/// (`0041de00`) and `RemoveDroppedItem` (`0041e0d0`, this the dropper's
/// list, the reference given) (Xbox PDB).
const GET_MODEL_SWAP: u32 = 0x0042_e250;
const GET_ITEM_DROPPER: u32 = 0x0041_de00;
const REMOVE_DROPPED_ITEM: u32 = 0x0041_e0d0;
/// `ExtraDataList::SetActivateChildrenTimer(float)` (Xbox PDB).
const SET_ACTIVATE_CHILDREN_TIMER: u32 = 0x0041_eff0;
/// `ExtraDataList::GetScriptLocals` (Xbox PDB) and the locals' fix-up
/// (`005aa090(locals, buffer)`).
const GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
const SCRIPT_LOCALS_FINISH_LOAD: u32 = 0x005a_a090;
/// `0084e3a0(form)`: the word at +0x0C of a form (read by the code as a
/// handle). It is declared with no parameter; the callers push one word.
const FORM_WORD_0C: u32 = 0x0084_e3a0;
/// The object at `011c3f2c` (read as a word) whose `00469860(value)`
/// ([`FORM_ID_IS_FILE_FORM`]) is true when the value is below `0xFF000000`.
const FILE_FORM_TESTER: u32 = 0x011c_3f2c;
/// `00936a20(handle, flag)` (cdecl): the Xbox PDB names it
/// `MobileObject::SayToCallBack`.
const MOBILE_OBJECT_SAY_TO_CALL_BACK: u32 = 0x0093_6a20;
/// `ActiveEffect::FinishLoadActiveEffectList(buffer, list)` (Xbox PDB,
/// cdecl).
const FINISH_LOAD_ACTIVE_EFFECT_LIST: u32 = 0x0080_6b50;
/// `00806b00(buffer, list)` (cdecl): the fix-up of the list of the type `0x33`
/// extra data after its form ids were resolved.
const RESOLVE_ACTIVE_EFFECT_LIST: u32 = 0x0080_6b00;
/// `MagicTarget::GetMagicTargetByNumericID` (Xbox PDB, cdecl).
const MAGIC_TARGET_FROM_ID: u32 = 0x0082_5550;
/// `TESPackage::CalculateProcedureType` (Xbox PDB, `this` the package, the
/// extra data's +0x14 given) and the test `009611e0` before it.
const PACKAGE_CALCULATE_PROCEDURE_TYPE: u32 = 0x0067_77b0;
const PACKAGE_PROCEDURE_TYPE: u32 = 0x0096_11e0;
/// `ExtraDataList::GetPackageExtra` (Xbox PDB).
const GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;
/// `0041e340(list, owner)`: the fix-up of the list a type `0x89` extra data
/// points at.
const ASHPILE_LIST_FIX: u32 = 0x0041_e340;
/// `009eb9c0(item, owner)`, run for each item of the type `0x35` list.
const PLAYER_CRIME_ITEM_FIX: u32 = 0x009e_b9c0;
/// `00835fe0(teleport data, buffer)`.
const TELEPORT_DATA_FINISH_LOAD: u32 = 0x0083_5fe0;

// Translated from 0042c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x27` extra data (`EXTRA_TIMELEFT`, vtable
/// `010158fc`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042c700(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_TIMELEFT, VTABLE_EXTRA_TIME_LEFT);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x28` extra data (`EXTRA_CHARGE`, vtable
/// `01015908`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042c730(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_CHARGE, VTABLE_EXTRA_CHARGE);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x0D` extra data (`EXTRA_SCRIPT`, vtable
/// `01015914`): the words at +0x0C and +0x10 are 0. Returns `this`.
pub fn fn_0042c760(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_SCRIPT, VTABLE_EXTRA_SCRIPT);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

// Translated from 0042c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraScript::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// (`00432040`), then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn extra_script_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_SCRIPT_DESTROY, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x30` extra data (`EXTRA_SCALE`, vtable
/// `01015920`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042c7d0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_SCALE, VTABLE_EXTRA_SCALE);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x4A` extra data (`EXTRA_HOT_KEY`, vtable
/// `0101592c`): the byte at +0x0C is 0. Returns `this`.
pub fn fn_0042c800(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_HOT_KEY, VTABLE_EXTRA_HOT_KEY);
    e.mem.set_u8(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x1C` extra data (`EXTRA_REFERENCE_POINTER`,
/// vtable `01015938`): the reference at +0x0C is null. Returns `this`.
pub fn fn_0042c830(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(
        e,
        this,
        EXTRA_REFERENCE_POINTER,
        VTABLE_EXTRA_REFERENCE_POINTER,
    );
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x1A` extra data (`EXTRA_TRESPASS_PACKAGE`,
/// vtable `01015944`): the package at +0x0C is null. Returns `this`.
pub fn fn_0042c860(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(
        e,
        this,
        EXTRA_TRESPASS_PACKAGE,
        VTABLE_EXTRA_TRESPASS_PACKAGE,
    );
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTresPassPackage::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`00432900`), then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_tres_pass_package_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(EXTRA_TRESPASS_PACKAGE_DESTROY, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042c8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x2F` extra data (`EXTRA_LEVELITEM`, vtable
/// `01015950`): the word at +0x0C is 0 and the byte at +0x10 is 0. Returns
/// `this`.
pub fn fn_0042c8c0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_LEVELITEM, VTABLE_EXTRA_LEVELED_ITEM);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    e.mem.set_u8(this.addr() + 0x10, 0);
    this
}

// Translated from 0042c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x3F` extra data (`EXTRA_POISON`, vtable
/// `0101595c`): the word at +0x0C is 0. Returns `this`.
pub fn fn_0042c900(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_POISON, VTABLE_EXTRA_POISON);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x46` extra data (`EXTRA_HEAD_TRACK_TARGET`,
/// vtable `01015968`): the word at +0x0C is 0. Returns `this`.
pub fn fn_0042c930(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(
        e,
        this,
        EXTRA_HEAD_TRACK_TARGET,
        VTABLE_EXTRA_HEAD_TRACK_TARGET,
    );
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHeadingTarget::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`00435ef0`), then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_heading_target_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(EXTRA_HEADING_TARGET_DESTROY, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042c990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x4E` extra data (`EXTRA_NO_RUMORS`, vtable
/// `01015974`): the byte at +0x0C is 0. Returns `this`.
pub fn fn_0042c990(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_NO_RUMORS, VTABLE_EXTRA_NO_RUMORS);
    e.mem.set_u8(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x56` extra data (`EXTRA_OBJECT_HEALTH`, vtable
/// `01015184`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042c9c0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_OBJECT_HEALTH, VTABLE_EXTRA_OBJECT_HEALTH);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042c9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x5B` extra data (`EXTRA_MODEL_SWAP`, vtable
/// `01015980`): the words at +0x0C and +0x10 are 0. Returns `this`.
pub fn fn_0042c9f0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_MODEL_SWAP, VTABLE_EXTRA_MODEL_SWAP);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

// Translated from 0042ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraRadius`, the type `0x5C` extra data (vtable
/// `01015208`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042ca30(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_RADIUS, VTABLE_EXTRA_RADIUS);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042ca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraRadiation`, the type `0x5D` extra data (vtable
/// `01015214`): the `float` at +0x0C is 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0042ca60(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_RADIATION, VTABLE_EXTRA_RADIATION);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraActorCause`, the type `0x60` extra data (vtable
/// `0101598c`): builds the member at +0x0C (`0042f730`, given 0). The
/// compiler's exception-unwinding frame is not translated. Returns `this`.
pub fn fn_0042ca90(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_ACTOR_CAUSE, VTABLE_EXTRA_ACTOR_CAUSE);
    e.call(ACTOR_CAUSE_MEMBER_INIT, &args![this.addr() + 0x0c, 0u32]);
    this
}

// Translated from 0042cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActorCause::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor ([`fn_0042cb30`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn extra_actor_cause_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0042cb30(e, this);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraActorCause`: sets its vtable (`0101598c`), assigns 0
/// to the member at +0x0C (`0042f780`), releases it (`0042f760`) and runs the
/// `BSExtraData` destructor (`0040ecb0`). The compiler's exception-unwinding
/// frame is not translated.
pub fn fn_0042cb30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_ACTOR_CAUSE);
    e.call(ASSIGN_HANDLE_AT, &args![this.addr() + 0x0c, 0u32]);
    e.call(ACTOR_CAUSE_MEMBER_RELEASE, &args![this.addr() + 0x0c]);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0042cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x6E` extra data (`EXTRA_AMMO`, vtable
/// `01015998`): the words at +0x0C and +0x10 are 0. Returns `this`.
pub fn fn_0042cba0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_AMMO, VTABLE_EXTRA_AMMO);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

// Translated from 0042cbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x70` extra data (`EXTRA_PACKAGE_DATA`, vtable
/// `010151fc`): the object at +0x0C is null. Returns `this`.
pub fn fn_0042cbe0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(e, this, EXTRA_PACKAGE_DATA, VTABLE_EXTRA_PACKAGE_DATA);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a three-word record: the word at +0 is 0, the word at +4 is
/// -1 and the `float` at +8 is the one at `01012054` (loaded and stored
/// through the x87 stack, so a signalling NaN would come out quiet). Returns
/// `this`.
pub fn fn_0042cc10(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0xffff_ffff);
    let value = e.mem.u32(FLOAT_COPIED_BY_RECORD_INIT);
    e.mem.set_u32(this.addr() + 8, x87_quiet_bits(value));
    this
}

// Translated from 0042cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x8D` extra data (`EXTRA_WEAPON_MOD_SLOTS`,
/// vtable `010159a4`): the byte at +0x0C is 0. Returns `this`.
pub fn fn_0042cc40(e: &mut Engine, this: Ptr) -> Ptr {
    construct_extra(
        e,
        this,
        EXTRA_WEAPON_MOD_SLOTS,
        VTABLE_EXTRA_WEAPON_MOD_SLOTS,
    );
    e.mem.set_u8(this.addr() + 0x0c, 0);
    this
}

// Translated from 0042cc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the type `0x8D` extra data: the destructor
/// ([`fn_0042cca0`]), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0042cc70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0042cca0(e, this);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the type `0x8D` extra data: sets its vtable (`010159a4`)
/// and runs the `BSExtraData` destructor (`0040ecb0`).
pub fn fn_0042cca0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_WEAPON_MOD_SLOTS);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0042ccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraSecuritronFace`, the type `0x8F` extra data (vtable
/// `010159b0`): builds the string members at +0x0C and +0x14 (`004037b0`).
/// The compiler's exception-unwinding frame is not translated. Returns `this`.
pub fn fn_0042ccc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_SECURITRON_FACE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_SECURITRON_FACE);
    e.call(STRING_MEMBER_INIT, &args![this.addr() + 0x0c]);
    e.call(STRING_MEMBER_INIT, &args![this.addr() + 0x14]);
    this
}

// Translated from 0042cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSecuritronFace::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor ([`fn_0042cd70`]), then `operator delete` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn extra_securitron_face_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0042cd70(e, this);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0042cd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraSecuritronFace`: sets its vtable (`010159b0`),
/// destroys the string members at +0x14 and +0x0C (`004037d0`) and runs the
/// `BSExtraData` destructor (`0040ecb0`). The compiler's exception-unwinding
/// frame is not translated.
pub fn fn_0042cd70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_SECURITRON_FACE);
    e.call(STRING_MEMBER_DESTROY, &args![this.addr() + 0x14]);
    e.call(STRING_MEMBER_DESTROY, &args![this.addr() + 0x0c]);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0042cde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList::IsEmpty` (`008256d0`) of the node the word at `this` points
/// at; the answer is the low byte of the result.
pub fn fn_0042cde0(e: &mut Engine, this: Ptr) -> bool {
    let node = e.mem.u32(this.addr());
    e.call(LIST_IS_EMPTY, &args![node]).bool()
}

// Translated from 0042ce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c6444`.
pub fn fn_0042ce00(e: &mut Engine) -> u32 {
    e.mem.u32(WORD_GIVEN_BY_GETTER)
}

// Translated from 0042ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the word at +0x244 of the object it is called on is set.
pub fn fn_0042ce10(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x244) & 2 != 0
}

// Translated from 0042ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at +0x2C of the object it is called on (a save buffer's
/// word of flags) in `out`. Returns `out`.
pub fn fn_0042ce30(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let word = e.mem.u32(this.addr() + 0x2c);
    e.mem.set_u32(out.addr(), word);
    out
}

// Translated from 0042ce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 3 (`0x8`) of the word at +0x28 of the
/// object it is called on.
pub fn fn_0042ce50(e: &mut Engine, this: Ptr, flag: u8) {
    let word = e.mem.u32(this.addr() + 0x28);
    let word = if flag != 0 { word | 8 } else { word & !8 };
    e.mem.set_u32(this.addr() + 0x28, word);
}

// Translated from 0042ce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 3 (`0x8`) of the word at +0x28 of the object it is called on
/// is set.
pub fn fn_0042ce90(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x28) & 8 != 0
}

/// Whether the save kind of the buffer (`00428110`) is in the mask the table
/// at `01183d30` has for the extra data type (the exe reads the entry by its
/// address); `004280f0` tests the mask.
fn buffer_allows(e: &mut Engine, buffer: u32, extra_type: u8) -> bool {
    let mask = e.mem.u32(SAVE_KIND_TABLE + extra_type as u32 * 4);
    e.with_stack(4, |e, out| {
        let kind = fn_00428110(e, Ptr::new(buffer), out);
        fn_004280f0(e, kind, mask)
    })
}

/// The form with the id `id` cast to `target`; null for an id of 0 (the exe
/// tests the id before looking it up).
fn form_or_null(e: &mut Engine, id: u32, target: u32) -> u32 {
    if id == 0 {
        0
    } else {
        form_as(e, id, target)
    }
}

/// Replaces the form id at `offset` of the extra data by the form it names
/// cast to `target` (null when there is none). Returns the form.
fn resolve_word(e: &mut Engine, extra: Ptr<BSExtraData>, offset: u32, target: u32) -> u32 {
    let id = e.mem.u32(extra.addr() + offset);
    let form = form_or_null(e, id, target);
    e.mem.set_u32(extra.addr() + offset, form);
    form
}

// Translated from 0042ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fix-up of a list after the buffered loader read it: `buffer` is the
/// `BGSLoadGameBuffer`, `form` the form the list belongs to (called by
/// `004bef60` and `00562660`). The buffer is asked for its reference
/// (virtual `+8`) and its owner (virtual `+0x0C`). For each type below whose
/// mask in the table at `01183d30` has the buffer's save kind, the form ids
/// the extra data holds become the forms they name (`004839c0` and a dynamic
/// cast, null when there is none): the type `0x1C` reference; the type
/// `0x0D` (script) is removed unless the list it holds is the owner's base
/// form's list at +0xF4 (or, without an owner, `form` cast to
/// `TESScriptableForm`); then, with a reference: the types `0x6C` (removed
/// when its reference is gone), `0x32` (magic item, magic target by id,
/// reference), `0x33` (reference, then the active effect list), `0x39`
/// (removed when the reference is gone, else the list of that reference is
/// fixed), `0x55`; with an owner: the types `0x19` (package and reference;
/// removed with no package, else the procedure type is computed when the
/// package has none), `0x1D` (followers: those that are no actor are removed
/// from the list; when the buffer's version byte is below `0x0E` the player is
/// removed), `0x70`, `0x3C`, `0x1A`, `0x46`, `0x5F` (the form, no cast),
/// `0x89` and `0x35`; with a reference but no owner: the type `0x2B` extra
/// data's teleport data is finished (`00835fe0`).
pub fn fn_0042ceb0(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32, form: u32) {
    let reference = e.vcall(buffer, 8, &args![]).u32();
    let owner = e.vcall(buffer, 0x0c, &args![]).u32();
    if buffer_allows(e, buffer, EXTRA_REFERENCE_POINTER) {
        let extra = find_extra(e, this, EXTRA_REFERENCE_POINTER);
        if !extra.is_null() {
            resolve_word(e, extra, 0x0c, RTTI_TES_OBJECT_REFR);
        }
    }
    if buffer_allows(e, buffer, EXTRA_SCRIPT) {
        let extra = find_extra(e, this, EXTRA_SCRIPT);
        if !extra.is_null() && form != 0 {
            let mut list = 0;
            if owner == 0 {
                list = dynamic_cast(e, form, RTTI_TES_SCRIPTABLE_FORM);
            } else {
                let base = e.call(REFERENCE_BASE_FORM, &args![owner]).u32();
                if base != 0 {
                    list = base + 0xf4;
                }
            }
            let held = e.mem.u32(extra.addr() + 0x0c);
            if list == 0 || held == 0 || list_next(e, list) != held {
                remove_extra(e, this, EXTRA_SCRIPT);
            }
        }
    }
    if reference == 0 {
        return;
    }
    if buffer_allows(e, buffer, EXTRA_OPENCLOSEACTIVATE_REF) {
        let extra = find_extra(e, this, EXTRA_OPENCLOSEACTIVATE_REF);
        if !extra.is_null() && resolve_word(e, extra, 0x0c, RTTI_TES_OBJECT_REFR) == 0 {
            remove_extra(e, this, EXTRA_OPENCLOSEACTIVATE_REF);
        }
    }
    if buffer_allows(e, buffer, EXTRA_MAGICCASTER) {
        let extra = find_extra(e, this, EXTRA_MAGICCASTER);
        if !extra.is_null() {
            let id = e.mem.u32(extra.addr() + 0x18);
            let item = if id == 0 {
                0
            } else {
                e.call(MAGIC_ITEM_FROM_FORM, &args![id]).u32()
            };
            e.mem.set_u32(extra.addr() + 0x18, item);
            let id = e.mem.u32(extra.addr() + 0x1c);
            let target = if id == 0 {
                0
            } else {
                e.call(MAGIC_TARGET_FROM_ID, &args![id]).u32()
            };
            e.mem.set_u32(extra.addr() + 0x1c, target);
            resolve_word(e, extra, 0x20, RTTI_TES_OBJECT_REFR);
        }
    }
    if buffer_allows(e, buffer, EXTRA_MAGICTARGET) {
        let extra = find_extra(e, this, EXTRA_MAGICTARGET);
        if !extra.is_null() {
            resolve_word(e, extra, 0x1c, RTTI_TES_OBJECT_REFR);
            e.call(
                RESOLVE_ACTIVE_EFFECT_LIST,
                &args![buffer, extra.addr() + 0x20],
            );
        }
    }
    if buffer_allows(e, buffer, EXTRA_ITEMDROPPER) {
        let extra = find_extra(e, this, EXTRA_ITEMDROPPER);
        if !extra.is_null() {
            let dropper = resolve_word(e, extra, 0x0c, RTTI_TES_OBJECT_REFR);
            if dropper == 0 {
                remove_extra(e, this, EXTRA_ITEMDROPPER);
            } else {
                // The `PUSH reference` before `005d43c0` belongs to the
                // second call.
                let list = reference_extra_list(e, dropper);
                e.call(ADD_DROPPED_ITEM_LIST_ENTRY, &args![list, reference]);
            }
        }
    }
    if buffer_allows(e, buffer, EXTRA_TALKING_ACTOR) {
        let extra = find_extra(e, this, EXTRA_TALKING_ACTOR);
        if !extra.is_null() {
            resolve_word(e, extra, 0x0c, RTTI_MOBILE_OBJECT);
        }
    }
    if owner == 0 {
        if buffer_allows(e, buffer, EXTRA_TELEPORT) {
            let extra = find_extra(e, this, EXTRA_TELEPORT);
            if !extra.is_null() {
                let data = e.mem.u32(extra.addr() + 0x0c);
                e.call(TELEPORT_DATA_FINISH_LOAD, &args![data, buffer]);
            }
        }
        return;
    }
    if buffer_allows(e, buffer, EXTRA_PACKAGE) {
        let extra = find_extra(e, this, EXTRA_PACKAGE);
        if !extra.is_null() {
            let package = resolve_word(e, extra, 0x0c, RTTI_TES_PACKAGE);
            let target = resolve_word(e, extra, 0x14, RTTI_TES_OBJECT_REFR);
            if package == 0 {
                remove_extra(e, this, EXTRA_PACKAGE);
            } else if e.call(PACKAGE_PROCEDURE_TYPE, &args![package]).u32() == 0xffff_ffff {
                e.call(PACKAGE_CALCULATE_PROCEDURE_TYPE, &args![package, target]);
            }
        }
    }
    if buffer_allows(e, buffer, EXTRA_FOLLOWER) {
        let extra = find_extra(e, this, EXTRA_FOLLOWER);
        if !extra.is_null() {
            fix_followers(e, extra.addr());
            if buffer_version(e, buffer) < 0x0e {
                let player = e.global::<u32>(PLAYER_SINGLETON);
                extra_data_list_remove_follower(e, this, player);
            }
        }
    }
    if buffer_allows(e, buffer, EXTRA_PACKAGE_DATA) {
        let extra = find_extra(e, this, EXTRA_PACKAGE_DATA);
        if !extra.is_null() {
            let object = e.mem.u32(extra.addr() + 0x0c);
            if object != 0 {
                // The result is not used.
                e.call(GET_PACKAGE_EXTRA, &args![this]);
                e.vcall(object, 0x14, &args![buffer]);
            }
        }
    }
    if buffer_allows(e, buffer, EXTRA_MERCHANTCONTAINER) {
        let extra = find_extra(e, this, EXTRA_MERCHANTCONTAINER);
        if !extra.is_null() {
            resolve_word(e, extra, 0x0c, RTTI_TES_OBJECT_REFR);
        }
    }
    if buffer_allows(e, buffer, EXTRA_TRESPASS_PACKAGE) {
        let extra = find_extra(e, this, EXTRA_TRESPASS_PACKAGE);
        if !extra.is_null() {
            let object = e.mem.u32(extra.addr() + 0x0c);
            if object != 0 {
                e.vcall(object, 0x64, &args![buffer]);
            }
        }
    }
    if buffer_allows(e, buffer, EXTRA_HEAD_TRACK_TARGET) {
        let extra = find_extra(e, this, EXTRA_HEAD_TRACK_TARGET);
        if !extra.is_null() {
            resolve_word(e, extra, 0x0c, RTTI_TES_OBJECT_REFR);
        }
    }
    if buffer_allows(e, buffer, EXTRA_DISMEMBERED_LIMBS) {
        let extra = find_extra(e, this, EXTRA_DISMEMBERED_LIMBS);
        if !extra.is_null() {
            let id = e.mem.u32(extra.addr() + 0x14);
            let found = if id == 0 {
                0
            } else {
                e.call(LOOKUP_FORM, &args![id]).u32()
            };
            e.mem.set_u32(extra.addr() + 0x14, found);
        }
    }
    if buffer_allows(e, buffer, EXTRA_ASHPILE_REF) {
        let extra = find_extra(e, this, EXTRA_ASHPILE_REF);
        if !extra.is_null() {
            let target = resolve_word(e, extra, 0x0c, RTTI_TES_OBJECT_REFR);
            if target != 0 {
                let list = reference_extra_list(e, target);
                e.call(ASHPILE_LIST_FIX, &args![list, owner]);
            }
        }
    }
    if buffer_allows(e, buffer, EXTRA_PLAYERCRIMELIST) {
        let extra = find_extra(e, this, EXTRA_PLAYERCRIMELIST);
        if !extra.is_null() {
            let mut node = e.mem.u32(extra.addr() + 0x0c);
            while node != 0 {
                let item = list_item(e, node);
                if item != 0 {
                    e.call(PLAYER_CRIME_ITEM_FIX, &args![item, owner]);
                }
                node = list_next(e, node);
            }
        }
    }
}

/// The followers of the type `0x1D` extra data (the list its +0x0C points at)
/// become the actors they name: each item is looked up and cast to `Actor`
/// and written back to the node (`00726c60`); a node whose item is null or no
/// actor is removed (after the previous node, `00905330`, or, at the head,
/// `0063f7b0`).
fn fix_followers(e: &mut Engine, extra: u32) {
    let mut node = e.mem.u32(extra + 0x0c);
    let mut previous = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let id = list_item(e, node);
        let actor = form_or_null(e, id, RTTI_ACTOR);
        if actor != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), actor);
                e.call(LIST_SET_ITEM, &args![node, slot]);
            });
            previous = node;
            node = list_next(e, node);
        } else if previous == 0 {
            e.call(LIST_REMOVE_HEAD, &args![node]);
        } else {
            let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
            e.call(LIST_REMOVE_ITEM, &args![previous, slot]);
            node = list_next(e, previous);
        }
    }
}

// Translated from 0042d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::FinishLoadGame` (Xbox PDB), `buffer` the load buffer: the
/// buffer is asked for its reference (virtual `+8`). When the buffer's save
/// kind is in the mask of the type `0x75` extra data, the list has it and
/// there is a reference: if the reference's virtual `+0xFC` says no, the byte
/// at +0x18 of the extra data is set to 1; otherwise `00936a20` is called with
/// the reference's word at +0x0C (`0084e3a0`) and 1 when the extra data's
/// words at +0x10 and +0x0C are both non-zero, else 0. Then, with a reference,
/// when the buffer's kind is in the mask of the type `0x33` extra data and the
/// list has it, its active effect list at +0x20 is finished
/// (`ActiveEffect::FinishLoadActiveEffectList`, `00806b50`).
pub fn extra_data_list_finish_load_game(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32) {
    let reference = e.vcall(buffer, 8, &args![]).u32();
    if buffer_allows(e, buffer, EXTRA_SAY_TO_TOPIC_INFO) {
        let extra = find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO);
        if !extra.is_null() && reference != 0 {
            if !e.vcall(reference, 0xfc, &args![]).bool() {
                e.mem.set_u8(extra.addr() + 0x18, 1);
            } else {
                // The word pushed before `0084e3a0` is not read by it.
                let handle = e.call(FORM_WORD_0C, &args![reference]).u32();
                let both =
                    e.mem.u32(extra.addr() + 0x10) != 0 && e.mem.u32(extra.addr() + 0x0c) != 0;
                e.call(MOBILE_OBJECT_SAY_TO_CALL_BACK, &args![handle, both as u32]);
            }
        }
    }
    if reference != 0 && buffer_allows(e, buffer, EXTRA_MAGICTARGET) {
        let extra = find_extra(e, this, EXTRA_MAGICTARGET);
        if !extra.is_null() {
            e.call(
                FINISH_LOAD_ACTIVE_EFFECT_LIST,
                &args![buffer, extra.addr() + 0x20],
            );
        }
    }
}

// Translated from 0042dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The clean-up of a list when the buffer `buffer` has been loaded (called by
/// `005629a0`). The buffer is asked for its reference (virtual `+8`). With a
/// reference: the buffer's owner is asked (virtual `+0x0C`, not used); a model
/// swap on the list (`0042e250`) is undone by the reference's virtual
/// `+0x1CC` (0, 1); the type `0x56` extra data sets the self damage of the
/// reference to 0 (`00477ce0`); the dropper of an item dropper (`0041de00`)
/// has this reference removed from its dropped item list (`0041e0d0`); the
/// map marker (`00418490`) is passed to `0042dd70`; when the buffer's word at
/// +0x2C has bit `0x4000000` the activate children timer is reset
/// (`0041eff0`, 0.0); the script locals (`00418830`) are finished
/// (`005aa090`). Then every extra data type of the table at `01183d30` whose
/// mask has a bit of the buffer's word at +0x2C is removed from the list,
/// except the types `0x0D`, `0x2C`, `0x2E`, `0x49`, `0x54`, `0x5F`, `0x90` and
/// `0x91` (the arms of the compiler's switch) and those whose mask has bit
/// `0x40000000` when the reference is no file form (`00469860` on its word at
/// +0x0C; false without a reference).
pub fn fn_0042dae0(e: &mut Engine, this: Ptr<ExtraDataList>, buffer: u32) {
    let reference = e.vcall(buffer, 8, &args![]).u32();
    if reference != 0 {
        e.vcall(buffer, 0x0c, &args![]);
        if e.call(GET_MODEL_SWAP, &args![this]).u32() != 0 {
            e.vcall(reference, 0x1cc, &args![0u32, 1u32]);
        }
        if !find_extra(e, this, EXTRA_OBJECT_HEALTH).is_null() {
            e.call(SET_SELF_DAMAGE, &args![reference, 0u32]);
        }
        let dropper = e.call(GET_ITEM_DROPPER, &args![this]).u32();
        if dropper != 0 {
            // The `PUSH reference` before `005d43c0` belongs to
            // `RemoveDroppedItem`.
            let list = reference_extra_list(e, dropper);
            e.call(REMOVE_DROPPED_ITEM, &args![list, reference]);
        }
        let marker = e.call(GET_MAP_MARKER, &args![this]).u32();
        if marker != 0 {
            fn_0042dd70(e, Ptr::new(marker));
        }
        let timer_flag = e.with_stack(4, |e, out| {
            let word = fn_0042ce30(e, Ptr::new(buffer), out);
            fn_004280f0(e, word, 0x0400_0000)
        });
        if timer_flag {
            e.call(SET_ACTIVATE_CHILDREN_TIMER, &args![this, 0u32]);
        }
        let locals = e.call(GET_SCRIPT_LOCALS, &args![this]).u32();
        if locals != 0 {
            e.call(SCRIPT_LOCALS_FINISH_LOAD, &args![locals, buffer]);
        }
    }
    let is_file_form = if reference == 0 {
        false
    } else {
        let handle = e.call(FORM_WORD_0C, &args![reference]).u32();
        let tester = e.global::<u32>(FILE_FORM_TESTER);
        e.call(FORM_ID_IS_FILE_FORM, &args![tester, handle]).u8() != 0
    };
    for extra_type in 0..0x93u32 {
        let mask = e.mem.u32(SAVE_KIND_TABLE + extra_type * 4);
        let mut remove = false;
        if mask != 0 {
            remove = e.with_stack(4, |e, out| {
                let word = fn_0042ce30(e, Ptr::new(buffer), out);
                fn_004280f0(e, word, mask)
            });
        }
        if !is_file_form && remove && mask & 0x4000_0000 != 0 {
            remove = false;
        }
        if remove
            && matches!(
                extra_type,
                0x0d | 0x2c | 0x2e | 0x49 | 0x54 | 0x5f | 0x90 | 0x91
            )
        {
            remove = false;
        }
        if remove {
            remove_extra(e, this, extra_type as u8);
        }
    }
}

// Translated from 0042dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the byte at +0x0D of the object it is called on to +0x0C.
pub fn fn_0042dd70(e: &mut Engine, this: Ptr) {
    let value = e.mem.u8(this.addr() + 0x0d);
    e.mem.set_u8(this.addr() + 0x0c, value);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00421400,
            extra_data_list_get_merchant_container(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00421430, fn_00421430(Ptr<ExtraDataList>, u32)),
        entry!(0x004214f0, fn_004214f0(Ptr<ExtraDataList>, Ptr, Ptr)),
        entry!(0x00421540, fn_00421540(Ptr<ExtraDataList>, u32)),
        entry!(0x00421600, fn_00421600(Ptr<ExtraDataList>, u8)),
        entry!(0x004216b0, fn_004216b0(Ptr<ExtraDataList>)),
        entry!(0x004216d0, fn_004216d0(Ptr<ExtraDataList>) -> bool),
        entry!(
            0x004216f0,
            extra_data_list_get_lev_crea_original_base(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00421720, fn_00421720(Ptr<ExtraDataList>) -> u32),
        entry!(0x00421750, fn_00421750(Ptr<ExtraDataList>, u32, u32)),
        entry!(
            0x00421820,
            extra_data_list_get_detach_time(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00421850, fn_00421850(Ptr<ExtraDataList>, u32)),
        entry!(0x00421910, fn_00421910(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00421940,
            extra_data_list_set_seen_data(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00421a40, fn_00421a40(Ptr<ExtraDataList>) -> f32),
        entry!(
            0x00421a70,
            extra_data_list_set_north_rotation(Ptr<ExtraDataList>, f32)
        ),
        entry!(
            0x00421b40,
            extra_data_list_get_x_target(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00421b70, fn_00421b70(Ptr<ExtraDataList>, u32)),
        entry!(0x00421c30, fn_00421c30(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00421c60,
            extra_data_list_set_encounter_zone(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00421d20, fn_00421d20(Ptr<ExtraDataList>) -> u32),
        entry!(0x00421d50, fn_00421d50(Ptr<ExtraDataList>, u32)),
        entry!(0x00421e10, fn_00421e10(Ptr<ExtraDataList>) -> u32),
        entry!(0x00421e40, fn_00421e40(Ptr<ExtraDataList>, u32)),
        entry!(0x00421f00, fn_00421f00(Ptr<ExtraDataList>) -> u32),
        entry!(0x00421f30, fn_00421f30(Ptr<ExtraDataList>, u32)),
        entry!(
            0x00422020,
            extra_data_list_get_multi_bound(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00422050,
            extra_data_list_set_multi_bound(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00422120, fn_00422120(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00422150,
            extra_data_list_set_occlusion_plane(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x00422220,
            extra_data_list_set_radius(Ptr<ExtraDataList>, f32)
        ),
        entry!(0x004222f0, fn_004222f0(Ptr, f32) -> Ptr),
        entry!(
            0x00422320,
            extra_data_list_get_radius(Ptr<ExtraDataList>) -> f32
        ),
        entry!(0x00422350, fn_00422350(Ptr<ExtraDataList>, f32)),
        entry!(0x00422420, fn_00422420(Ptr, f32) -> Ptr),
        entry!(0x00422450, fn_00422450(Ptr<ExtraDataList>) -> f32),
        entry!(
            0x00422480,
            extra_data_list_add_follower(Ptr<ExtraDataList>, Ptr)
        ),
        entry!(0x00422550, fn_00422550(Ptr<ExtraDataList>, Ptr) -> bool),
        entry!(
            0x00422590,
            extra_data_list_add_friend_hit(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00422640,
            extra_data_list_get_friend_hit_count(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00422670, fn_00422670(Ptr<ExtraDataList>)),
        entry!(
            0x00422690,
            extra_data_list_remove_follower(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x004226e0, fn_004226e0(Ptr) -> u8),
        entry!(
            0x00422700,
            fn_00422700(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00422720,
            extra_data_list_remove_follower_extra(Ptr<ExtraDataList>)
        ),
        entry!(0x00422750, fn_00422750(Ptr<ExtraDataList>, u8, f32)),
        entry!(
            0x00422820,
            extra_data_list_get_refraction_property_extra(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(0x00422850, fn_00422850(Ptr<ExtraDataList>, u32)),
        entry!(
            0x004228f0,
            extra_data_list_get_last_finished_sequence(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00422920,
            extra_data_list_remove_last_finished_sequence(Ptr<ExtraDataList>)
        ),
        entry!(
            0x00422940,
            extra_data_list_set_saved_animation(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x00422a10,
            extra_data_list_get_saved_animation(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00422a40,
            extra_data_list_restore_saved_animation(Ptr<ExtraDataList>, u32) -> bool
        ),
        entry!(
            0x00422aa0,
            extra_data_list_remove_saved_animation(Ptr<ExtraDataList>)
        ),
        entry!(
            0x00422ac0,
            extra_data_list_set_saved_havok_data(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x00422b90,
            extra_data_list_get_saved_havok_data(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00422bc0,
            extra_data_list_restore_saved_havok_data(Ptr<ExtraDataList>, u32) -> bool
        ),
        entry!(
            0x00422c20,
            extra_data_list_remove_saved_havok_data(Ptr<ExtraDataList>)
        ),
        entry!(0x00422c40, fn_00422c40(Ptr<ExtraDataList>, u32, Ptr) -> u16),
        entry!(
            0x004235e0,
            extra_data_list_save_game(Ptr<ExtraDataList>, u32, Ptr)
        ),
        entry!(0x00424940, fn_00424940(Ptr) -> u8),
        entry!(
            0x00424960,
            extra_data_list_load_game(Ptr<ExtraDataList>, u32, u32, Ptr)
        ),
        entry!(
            0x00426a30,
            extra_data_list_save_game_ov2(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00428070, fn_00428070(Ptr, u32) -> u32),
        entry!(0x004280b0, fn_004280b0(Ptr, u32) -> u32),
        entry!(0x004280f0, fn_004280f0(Ptr, u32) -> bool),
        entry!(0x00428110, fn_00428110(Ptr, Ptr) -> Ptr),
        entry!(0x00428130, fn_00428130(Ptr, u32)),
        entry!(0x0042c470, fn_0042c470(Ptr) -> Ptr),
        entry!(0x0042c4b0, fn_0042c4b0(Ptr) -> Ptr),
        entry!(
            0x0042c550,
            extra_lock_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042c580, fn_0042c580(Ptr) -> Ptr),
        entry!(
            0x0042c5b0,
            extra_teleport_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042c5e0, fn_0042c5e0(Ptr) -> Ptr),
        entry!(0x0042c610, fn_0042c610(Ptr) -> Ptr),
        entry!(0x0042c640, fn_0042c640(Ptr) -> Ptr),
        entry!(0x0042c670, fn_0042c670(Ptr) -> Ptr),
        entry!(0x0042c6a0, fn_0042c6a0(Ptr) -> Ptr),
        entry!(0x0042c6d0, fn_0042c6d0(Ptr) -> Ptr),
        entry!(
            0x00428150,
            extra_data_list_load_game_ov2(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0042c700, fn_0042c700(Ptr) -> Ptr),
        entry!(0x0042c730, fn_0042c730(Ptr) -> Ptr),
        entry!(0x0042c760, fn_0042c760(Ptr) -> Ptr),
        entry!(
            0x0042c7a0,
            extra_script_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042c7d0, fn_0042c7d0(Ptr) -> Ptr),
        entry!(0x0042c800, fn_0042c800(Ptr) -> Ptr),
        entry!(0x0042c830, fn_0042c830(Ptr) -> Ptr),
        entry!(0x0042c860, fn_0042c860(Ptr) -> Ptr),
        entry!(
            0x0042c890,
            extra_tres_pass_package_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042c8c0, fn_0042c8c0(Ptr) -> Ptr),
        entry!(0x0042c900, fn_0042c900(Ptr) -> Ptr),
        entry!(0x0042c930, fn_0042c930(Ptr) -> Ptr),
        entry!(
            0x0042c960,
            extra_heading_target_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042c990, fn_0042c990(Ptr) -> Ptr),
        entry!(0x0042c9c0, fn_0042c9c0(Ptr) -> Ptr),
        entry!(0x0042c9f0, fn_0042c9f0(Ptr) -> Ptr),
        entry!(0x0042ca30, fn_0042ca30(Ptr) -> Ptr),
        entry!(0x0042ca60, fn_0042ca60(Ptr) -> Ptr),
        entry!(0x0042ca90, fn_0042ca90(Ptr) -> Ptr),
        entry!(
            0x0042cb00,
            extra_actor_cause_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042cb30, fn_0042cb30(Ptr)),
        entry!(0x0042cba0, fn_0042cba0(Ptr) -> Ptr),
        entry!(0x0042cbe0, fn_0042cbe0(Ptr) -> Ptr),
        entry!(0x0042cc10, fn_0042cc10(Ptr) -> Ptr),
        entry!(0x0042cc40, fn_0042cc40(Ptr) -> Ptr),
        entry!(0x0042cc70, fn_0042cc70(Ptr, u32) -> Ptr),
        entry!(0x0042cca0, fn_0042cca0(Ptr)),
        entry!(0x0042ccc0, fn_0042ccc0(Ptr) -> Ptr),
        entry!(
            0x0042cd40,
            extra_securitron_face_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042cd70, fn_0042cd70(Ptr)),
        entry!(0x0042cde0, fn_0042cde0(Ptr) -> bool),
        entry!(0x0042ce00, fn_0042ce00() -> u32),
        entry!(0x0042ce10, fn_0042ce10(Ptr) -> bool),
        entry!(0x0042ce30, fn_0042ce30(Ptr, Ptr) -> Ptr),
        entry!(0x0042ce50, fn_0042ce50(Ptr, u8)),
        entry!(0x0042ce90, fn_0042ce90(Ptr) -> bool),
        entry!(0x0042ceb0, fn_0042ceb0(Ptr<ExtraDataList>, u32, u32)),
        entry!(
            0x0042d9e0,
            extra_data_list_finish_load_game(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0042dae0, fn_0042dae0(Ptr<ExtraDataList>, u32)),
        entry!(0x0042dd70, fn_0042dd70(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    /// The `(list, item)` pairs `LIST_ADD_HEAD` was given.
    type Added = Rc<RefCell<Vec<(u32, u32)>>>;

    /// Test vtable of the extra data (slot 0 the scalar deleting destructor).
    const VTABLE: u32 = 0x0200_0000;
    const DESTRUCTOR: u32 = 0x0200_1000;

    // Callees of the list operations in `extradatalist.rs`, which run for
    // real here on top of these doubles.
    const GET_TYPE: u32 = 0x004f_1540;
    const GET_NEXT: u32 = 0x0044_ddc0;
    const SET_NEXT: u32 = 0x0040_3550;
    const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
    const SIMPLE_LIST_ITEM: u32 = 0x0068_15c0;
    const MEMSET: u32 = 0x0040_3d30;
    const LOCK: u32 = 0x0040_fbf0;
    const UNLOCK: u32 = 0x0040_fba0;

    /// (constructor, extra data type): each double sets the vtable, the type
    /// and a null next, and stores the argument word, if any, at +0x0C.
    const CONSTRUCTORS: &[(u32, u8)] = &[
        (MERCHANT_CONTAINER_INIT, EXTRA_MERCHANT_CONTAINER),
        (LEV_CREA_MOD_INIT, EXTRA_LEV_CREA_MOD),
        (NO_RUMORS_INIT, EXTRA_NO_RUMORS),
        (LEVELED_CREATURE_INIT, EXTRA_LEVELED_CREATURE),
        (SEEN_DATA_INIT, EXTRA_SEEN_DATA),
        (X_TARGET_INIT, EXTRA_X_TARGET),
        (ENCOUNTER_ZONE_INIT, EXTRA_ENCOUNTER_ZONE),
        (EMITTANCE_SOURCE_INIT, EXTRA_EMITTANCE_SOURCE),
        (MULTIBOUND_REF_INIT, EXTRA_MULTIBOUND_REF),
        (MULTIBOUND_DATA_INIT, EXTRA_MULTIBOUND_DATA),
        (MULTIBOUND_INIT, EXTRA_MULTIBOUND),
        (OCCLUSION_PLANE_INIT, EXTRA_OCCLUSION_PLANE),
        (FOLLOWER_INIT, EXTRA_FOLLOWER),
        (FRIEND_HITS_INIT, EXTRA_FRIEND_HITS),
        (REFRACTION_PROPERTY_INIT, EXTRA_REFRACTION_PROPERTY),
        (LAST_FINISHED_SEQUENCE_INIT, EXTRA_LASTFINISHEDSEQUENCE),
        (SAVED_ANIMATION_INIT, EXTRA_SAVED_ANIMATION),
        (SAVED_HAVOK_DATA_INIT, EXTRA_SAVED_HAVOK_DATA),
    ];

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    /// An engine with doubles for the small callees of the list operations
    /// (type and next accessors, the lock, `operator new`/`delete`, the
    /// `BSExtraData` constructor, the handle accessors) and for the
    /// constructors of the extra data other units own.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.put_vtable(VTABLE, &[DESTRUCTOR]);
        stub(&mut e, DESTRUCTOR);
        // The vtables the real constructors of this unit set.
        for vtable in [
            0x0101_42a0,
            0x0101_42ac,
            VTABLE_EXTRA_RADIUS,
            VTABLE_EXTRA_RADIATION,
        ] {
            e.put_vtable(vtable, &[DESTRUCTOR]);
        }
        e.register(GET_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_NEXT, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(SIMPLE_LIST_NEXT, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(SIMPLE_LIST_ITEM, |_, a| returns(a[0]));
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(BS_EXTRA_DATA_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        stub(&mut e, OPERATOR_DELETE);
        stub(&mut e, LOCK);
        stub(&mut e, UNLOCK);
        e.register(ASSIGN_HANDLE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        for &(address, extra_type) in CONSTRUCTORS {
            e.register_double(address, move |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                e.mem.set_u8(a[0] + 4, extra_type);
                e.mem.set_u32(a[0] + 8, 0);
                if let Some(word) = a.get(1) {
                    e.mem.set_u32(a[0] + 0x0c, *word);
                }
                returns(a[0])
            });
        }
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<f64>(ZERO_DOUBLE, 0.0);
        e.set_global::<u32>(PLAYER_SINGLETON, 0x7777);
        e
    }

    fn new_list(e: &mut Engine) -> Ptr<ExtraDataList> {
        e.new_object()
    }

    /// Adds an extra data of `extra_type` holding `word` at +0x0C to the list
    /// through `AddExtra`.
    fn add(
        e: &mut Engine,
        list: Ptr<ExtraDataList>,
        extra_type: u8,
        word: u32,
    ) -> Ptr<BSExtraData> {
        let extra: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(extra.addr(), VTABLE);
        e.set(extra, BSExtraData::cEtype, extra_type);
        e.mem.set_u32(extra.addr() + 0x0c, word);
        add_extra(e, list, extra);
        extra
    }

    fn chain_types(e: &Engine, list: Ptr<ExtraDataList>) -> Vec<u8> {
        let mut types = vec![];
        let mut current: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
        while !current.is_null() {
            types.push(e.get(current, BSExtraData::cEtype));
            current = e.get(current, BSExtraData::pNext).cast();
        }
        types
    }

    fn word_of(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
        let extra = find_extra(e, list, extra_type);
        assert!(!extra.is_null());
        e.mem.u32(extra.addr() + 0x0c)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Runs `call` with the call log on and returns the log.
    fn logged(e: &mut Engine, call: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        call(e);
        e.call_log.take().unwrap()
    }

    /// A word getter: 0 without the extra data, the word at +0x0C with it.
    fn check_word_getter(address: u32, extra_type: u8) {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(address, &args![list]).u32(), 0);
        add(&mut e, list, 0x01, 0x1111);
        assert_eq!(e.call(address, &args![list]).u32(), 0);
        add(&mut e, list, extra_type, 0xabcd_0001);
        assert_eq!(e.call(address, &args![list]).u32(), 0xabcd_0001);
    }

    /// A word setter: builds the extra data on the first call (one 0x10-byte
    /// allocation, its constructor), updates it on the second, removes (and
    /// deletes) it on `remove_value`.
    fn check_word_setter(address: u32, extra_type: u8, construct: u32, remove_value: u32) {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x4000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, construct).len(), 1);
        assert_eq!(chain_types(&e, list), vec![extra_type]);
        assert_eq!(word_of(&mut e, list, extra_type), 0x4000);

        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x5000u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, construct).is_empty());
        assert_eq!(chain_types(&e, list), vec![extra_type]);
        assert_eq!(word_of(&mut e, list, extra_type), 0x5000);

        let extra = find_extra(&mut e, list, extra_type);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, remove_value]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn merchant_container_getter() {
        check_word_getter(0x0042_1400, EXTRA_MERCHANT_CONTAINER);
    }

    #[test]
    fn merchant_container_setter() {
        check_word_setter(
            0x0042_1430,
            EXTRA_MERCHANT_CONTAINER,
            MERCHANT_CONTAINER_INIT,
            0,
        );
    }

    #[test]
    fn lev_crea_mod_getter_defaults_to_one_and_zero() {
        let mut e = engine();
        e.register(LEV_CREA_MOD_GET_FLOAT, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register(LEV_CREA_MOD_GET_WORD, |_, _| returns(7));
        let list = new_list(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(out.addr(), 0xdead);
        e.mem.set_u32(out.addr() + 4, 0xbeef);
        let log = logged(&mut e, |e| {
            e.call(0x0042_14f0, &args![list, out, out.addr() + 4]);
        });
        assert_eq!(e.mem.f32(out.addr()), 1.0);
        assert_eq!(e.mem.u32(out.addr() + 4), 0);
        assert!(calls_to(&log, LEV_CREA_MOD_GET_FLOAT).is_empty());

        let extra = add(&mut e, list, EXTRA_LEV_CREA_MOD, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_14f0, &args![list, out, out.addr() + 4]);
        });
        assert_eq!(e.mem.f32(out.addr()), 2.5);
        assert_eq!(e.mem.u32(out.addr() + 4), 7);
        assert_eq!(
            calls_to(&log, LEV_CREA_MOD_GET_FLOAT),
            vec![vec![extra.addr()]]
        );
        assert_eq!(
            calls_to(&log, LEV_CREA_MOD_GET_WORD),
            vec![vec![extra.addr()]]
        );
    }

    #[test]
    fn lev_crea_mod_setter_removes_on_four() {
        check_word_setter(0x0042_1540, EXTRA_LEV_CREA_MOD, LEV_CREA_MOD_INIT, 4);
    }

    #[test]
    fn no_rumors_setter_stores_a_byte_and_never_removes() {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1600, &args![list, 1u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        // The constructor receives the byte.
        let constructed = calls_to(&log, NO_RUMORS_INIT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][1], 1);
        assert_eq!(chain_types(&e, list), vec![EXTRA_NO_RUMORS]);
        assert_eq!(word_of(&mut e, list, EXTRA_NO_RUMORS), 1);

        // An existing extra data gets only the byte at +0x0C.
        let extra = find_extra(&mut e, list, EXTRA_NO_RUMORS);
        e.mem.set_u32(extra.addr() + 0x0c, 0x1122_3344);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1600, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_of(&mut e, list, EXTRA_NO_RUMORS), 0x1122_3300);
        assert_eq!(chain_types(&e, list), vec![EXTRA_NO_RUMORS]);
    }

    #[test]
    fn no_rumors_remover_deletes_the_extra_data() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = add(&mut e, list, EXTRA_NO_RUMORS, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0042_16b0, &args![list]);
        });
        assert_eq!(calls_to(&log, REMOVE_EXTRA), vec![vec![list.addr(), 0x4e]]);
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn leveled_creature_presence_is_has_extra() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(!e.call(0x0042_16d0, &args![list]).bool());
        add(&mut e, list, 0x01, 0);
        assert!(!e.call(0x0042_16d0, &args![list]).bool());
        add(&mut e, list, EXTRA_LEVELED_CREATURE, 0);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0042_16d0, &args![list]).bool());
        });
        assert_eq!(calls_to(&log, HAS_EXTRA), vec![vec![list.addr(), 0x2e]]);
    }

    #[test]
    fn lev_crea_original_base_getter() {
        check_word_getter(0x0042_16f0, EXTRA_LEVELED_CREATURE);
    }

    #[test]
    fn leveled_creature_second_word_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0042_1720, &args![list]).u32(), 0);
        let extra = add(&mut e, list, EXTRA_LEVELED_CREATURE, 0x11);
        e.mem.set_u32(extra.addr() + 0x10, 0x22);
        assert_eq!(e.call(0x0042_1720, &args![list]).u32(), 0x22);
    }

    #[test]
    fn leveled_creature_setter() {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1750, &args![list, 0x11u32, 0x22u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, LEVELED_CREATURE_INIT).len(), 1);
        assert_eq!(word_of(&mut e, list, EXTRA_LEVELED_CREATURE), 0x11);
        let extra = find_extra(&mut e, list, EXTRA_LEVELED_CREATURE);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x22);

        // An existing extra data is updated in place.
        let log = logged(&mut e, |e| {
            e.call(0x0042_1750, &args![list, 0x33u32, 0x44u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x33);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x44);

        // A zero in either word removes it.
        e.call(0x0042_1750, &args![list, 0x33u32, 0u32]);
        assert!(chain_types(&e, list).is_empty());
        e.call(0x0042_1750, &args![list, 0x33u32, 0x44u32]);
        assert_eq!(chain_types(&e, list), vec![EXTRA_LEVELED_CREATURE]);
        e.call(0x0042_1750, &args![list, 0u32, 0x44u32]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn detach_time_getter() {
        check_word_getter(0x0042_1820, EXTRA_CELL_DETACH_TIME);
    }

    #[test]
    fn detach_time_setter() {
        check_word_setter(0x0042_1850, EXTRA_CELL_DETACH_TIME, DETACH_TIME_INIT, 0);
    }

    #[test]
    fn seen_data_getter() {
        check_word_getter(0x0042_1910, EXTRA_SEEN_DATA);
    }

    #[test]
    fn seen_data_setter_deletes_the_old_object() {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1940, &args![list, 0x4000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, SEEN_DATA_INIT).len(), 1);
        assert_eq!(word_of(&mut e, list, EXTRA_SEEN_DATA), 0x4000);
        assert!(calls_to(&log, DESTRUCTOR).is_empty());

        // The old word is an object: it is deleted before the replacement.
        let old: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(old.addr(), VTABLE);
        let extra = find_extra(&mut e, list, EXTRA_SEEN_DATA);
        e.mem.set_u32(extra.addr() + 0x0c, old.addr());
        let log = logged(&mut e, |e| {
            e.call(0x0042_1940, &args![list, 0x5000u32]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![old.addr(), 1]]);
        assert_eq!(word_of(&mut e, list, EXTRA_SEEN_DATA), 0x5000);

        // A null old word is not deleted; a null value removes.
        e.mem.set_u32(extra.addr() + 0x0c, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1940, &args![list, 0x6000u32]);
        });
        assert!(calls_to(&log, DESTRUCTOR).is_empty());
        assert_eq!(word_of(&mut e, list, EXTRA_SEEN_DATA), 0x6000);
        e.call(0x0042_1940, &args![list, 0u32]);
        assert!(chain_types(&e, list).is_empty());
    }

    /// Float getter: 0.0 without the extra data, the float with it; a
    /// signalling NaN comes out quiet (x87 load).
    fn check_float_getter(address: u32, extra_type: u8) {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(address, &args![list]).f32(), 0.0);
        let extra = add(&mut e, list, extra_type, 2.5f32.to_bits());
        assert_eq!(e.call(address, &args![list]).f32(), 2.5);
        e.mem.set_u32(extra.addr() + 0x0c, 0x7f80_0001);
        let quiet = e.call(address, &args![list]).f32();
        assert_eq!(quiet.to_bits() & 0x7fc0_0000, 0x7fc0_0000);
    }

    /// Float setter: builds, updates, stores a NaN, and removes on 0.0 and
    /// -0.0.
    fn check_float_setter(address: u32, extra_type: u8) {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 1.5f32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(chain_types(&e, list), vec![extra_type]);
        assert_eq!(word_of(&mut e, list, extra_type), 1.5f32.to_bits());

        let log = logged(&mut e, |e| {
            e.call(address, &args![list, -4.25f32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_of(&mut e, list, extra_type), (-4.25f32).to_bits());

        // A signalling NaN is not zero, and is stored quiet.
        e.call(address, &args![list, f32::from_bits(0x7f80_0001)]);
        assert_eq!(chain_types(&e, list), vec![extra_type]);
        assert_eq!(word_of(&mut e, list, extra_type), 0x7fc0_0001);

        let extra = find_extra(&mut e, list, extra_type);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0.0f32]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
        add(&mut e, list, extra_type, 1.0f32.to_bits());
        e.call(address, &args![list, -0.0f32]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn north_rotation_getter() {
        check_float_getter(0x0042_1a40, EXTRA_NORTH_ROTATION);
    }

    #[test]
    fn north_rotation_setter() {
        check_float_setter(0x0042_1a70, EXTRA_NORTH_ROTATION);
    }

    #[test]
    fn x_target_getter() {
        check_word_getter(0x0042_1b40, EXTRA_X_TARGET);
    }

    #[test]
    fn x_target_setter() {
        check_word_setter(0x0042_1b70, EXTRA_X_TARGET, X_TARGET_INIT, 0);
    }

    #[test]
    fn encounter_zone_getter() {
        check_word_getter(0x0042_1c30, EXTRA_ENCOUNTER_ZONE);
    }

    #[test]
    fn encounter_zone_setter() {
        check_word_setter(0x0042_1c60, EXTRA_ENCOUNTER_ZONE, ENCOUNTER_ZONE_INIT, 0);
    }

    #[test]
    fn emittance_source_getter() {
        check_word_getter(0x0042_1d20, EXTRA_EMITTANCE_SOURCE);
    }

    #[test]
    fn emittance_source_setter() {
        check_word_setter(
            0x0042_1d50,
            EXTRA_EMITTANCE_SOURCE,
            EMITTANCE_SOURCE_INIT,
            0,
        );
    }

    #[test]
    fn multibound_reference_getter() {
        check_word_getter(0x0042_1e10, EXTRA_MULTIBOUND_REF);
    }

    #[test]
    fn multibound_reference_setter() {
        check_word_setter(0x0042_1e40, EXTRA_MULTIBOUND_REF, MULTIBOUND_REF_INIT, 0);
    }

    #[test]
    fn multibound_data_getter() {
        check_word_getter(0x0042_1f00, EXTRA_MULTIBOUND_DATA);
    }

    #[test]
    fn multibound_data_setter_frees_the_old_block() {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1f30, &args![list, 0x4000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, MULTIBOUND_DATA_INIT).len(), 1);
        assert_eq!(word_of(&mut e, list, EXTRA_MULTIBOUND_DATA), 0x4000);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());

        // The old block is freed before it is replaced.
        let log = logged(&mut e, |e| {
            e.call(0x0042_1f30, &args![list, 0x5000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x4000]]);
        assert_eq!(word_of(&mut e, list, EXTRA_MULTIBOUND_DATA), 0x5000);

        // A null old word is not freed.
        let extra = find_extra(&mut e, list, EXTRA_MULTIBOUND_DATA);
        e.mem.set_u32(extra.addr() + 0x0c, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_1f30, &args![list, 0x6000u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());

        // A null value removes the extra data (without freeing the block).
        let log = logged(&mut e, |e| {
            e.call(0x0042_1f30, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert!(chain_types(&e, list).is_empty());
    }

    /// A handle getter reads the word the handle at +0x0C points at.
    fn check_handle_getter(address: u32, extra_type: u8) {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(address, &args![list]).u32(), 0);
        let extra = add(&mut e, list, extra_type, 0x6001);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(address, &args![list]).u32(), 0x6001);
        });
        assert_eq!(calls_to(&log, READ_WORD), vec![vec![extra.addr() + 0x0c]]);
    }

    /// A handle setter: the new extra data is constructed, assigned the
    /// handle, then added; an existing one is only assigned; zero removes.
    fn check_handle_setter(address: u32, extra_type: u8, construct: u32) {
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x4000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, construct).len(), 1);
        let extra = find_extra(&mut e, list, extra_type);
        assert_eq!(
            calls_to(&log, ASSIGN_HANDLE),
            vec![vec![extra.addr() + 0x0c, 0x4000]]
        );
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        let assigned = order.iter().position(|a| *a == ASSIGN_HANDLE).unwrap();
        let added = order.iter().position(|a| *a == ADD_EXTRA).unwrap();
        assert!(assigned < added);
        assert_eq!(word_of(&mut e, list, extra_type), 0x4000);

        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x5000u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, ASSIGN_HANDLE),
            vec![vec![extra.addr() + 0x0c, 0x5000]]
        );

        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0u32]);
        });
        assert!(calls_to(&log, ASSIGN_HANDLE).is_empty());
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn multibound_getter_reads_the_handle() {
        check_handle_getter(0x0042_2020, EXTRA_MULTIBOUND);
    }

    #[test]
    fn multibound_setter_assigns_the_handle() {
        check_handle_setter(0x0042_2050, EXTRA_MULTIBOUND, MULTIBOUND_INIT);
    }

    #[test]
    fn occlusion_plane_getter_reads_the_handle() {
        check_handle_getter(0x0042_2120, EXTRA_OCCLUSION_PLANE);
    }

    #[test]
    fn occlusion_plane_setter_assigns_the_handle() {
        check_handle_setter(0x0042_2150, EXTRA_OCCLUSION_PLANE, OCCLUSION_PLANE_INIT);
    }

    #[test]
    fn radius_setter() {
        check_float_setter(0x0042_2220, EXTRA_RADIUS);
        // The new extra data is built with the radius.
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2220, &args![list, 3.0f32]);
        });
        let extra = find_extra(&mut e, list, EXTRA_RADIUS);
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x5c]]
        );
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_RADIUS);
    }

    #[test]
    fn radius_constructor() {
        let mut e = engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x10));
        let result = e.call(0x0042_22f0, &args![block, 12.5f32]).ptr::<()>();
        assert_eq!(result, block);
        assert_eq!(e.mem.u32(block.addr()), 0x0101_5208);
        assert_eq!(e.mem.u8(block.addr() + 4), 0x5c);
        assert_eq!(e.mem.f32(block.addr() + 0x0c), 12.5);
    }

    #[test]
    fn radius_getter() {
        check_float_getter(0x0042_2320, EXTRA_RADIUS);
    }

    #[test]
    fn radiation_setter() {
        check_float_setter(0x0042_2350, EXTRA_RADIATION);
        let mut e = engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2350, &args![list, 3.0f32]);
        });
        let extra = find_extra(&mut e, list, EXTRA_RADIATION);
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x5d]]
        );
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_RADIATION);
    }

    #[test]
    fn radiation_constructor() {
        let mut e = engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x10));
        let result = e.call(0x0042_2420, &args![block, -0.5f32]).ptr::<()>();
        assert_eq!(result, block);
        assert_eq!(e.mem.u32(block.addr()), 0x0101_5214);
        assert_eq!(e.mem.u8(block.addr() + 4), 0x5d);
        assert_eq!(e.mem.f32(block.addr() + 0x0c), -0.5);
    }

    #[test]
    fn radiation_getter() {
        check_float_getter(0x0042_2450, EXTRA_RADIATION);
    }

    /// Doubles for the follower list: `LIST_CONTAINS` is true for the item
    /// 0x5555, `LIST_ADD_HEAD` records the items added.
    fn follower_engine() -> (Engine, Added) {
        let mut e = engine();
        e.register(FOLLOWER_INIT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, EXTRA_FOLLOWER);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 0x0c, 0xaaaa);
            returns(a[0])
        });
        e.register(LIST_CONTAINS, |e, a| {
            returns((e.mem.u32(a[1]) == 0x5555) as u32)
        });
        let added = Rc::new(RefCell::new(vec![]));
        let record = added.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        (e, added)
    }

    #[test]
    fn add_follower_builds_the_extra_data_and_adds_new_followers() {
        let (mut e, added) = follower_engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2480, &args![list, 0x1234u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, FOLLOWER_INIT).len(), 1);
        assert_eq!(chain_types(&e, list), vec![EXTRA_FOLLOWER]);
        assert_eq!(*added.borrow(), vec![(0xaaaa, 0x1234)]);
        // The test is made on the same list and a stack slot with the item.
        assert_eq!(calls_to(&log, LIST_CONTAINS)[0][0], 0xaaaa);

        // The extra data exists now: nothing is built, a second follower is
        // added, a follower already in the list is not.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2480, &args![list, 0x2345u32]);
            e.call(0x0042_2480, &args![list, 0x5555u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(*added.borrow(), vec![(0xaaaa, 0x1234), (0xaaaa, 0x2345)]);
    }

    #[test]
    fn add_follower_ignores_the_player() {
        let (mut e, added) = follower_engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2480, &args![list, 0x7777u32]);
        });
        // Only the top-level call: nothing else happens.
        assert_eq!(log.len(), 1);
        assert!(chain_types(&e, list).is_empty());
        assert!(added.borrow().is_empty());
    }

    #[test]
    fn follower_membership_test() {
        let (mut e, _) = follower_engine();
        let list = new_list(&mut e);
        // No extra data: false, without testing a list.
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0042_2550, &args![list, 0x5555u32]).bool());
        });
        assert!(calls_to(&log, LIST_CONTAINS).is_empty());
        add(&mut e, list, EXTRA_FOLLOWER, 0xaaaa);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0042_2550, &args![list, 0x5555u32]).bool());
            assert!(!e.call(0x0042_2550, &args![list, 0x1111u32]).bool());
        });
        let tests = calls_to(&log, LIST_CONTAINS);
        assert_eq!(tests.len(), 2);
        assert!(tests.iter().all(|a| a[0] == 0xaaaa));
    }

    #[test]
    fn friend_hit_builds_the_extra_data_then_adds_a_hit() {
        let mut e = engine();
        stub(&mut e, FRIEND_HITS_ADD_HIT);
        e.register(FRIEND_HITS_GET_HIT_COUNT, |_, _| returns(3));
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2590, &args![list]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1c]]);
        assert_eq!(calls_to(&log, FRIEND_HITS_INIT).len(), 1);
        let extra = find_extra(&mut e, list, EXTRA_FRIEND_HITS);
        assert_eq!(
            calls_to(&log, FRIEND_HITS_ADD_HIT),
            vec![vec![extra.addr()]]
        );
        assert_eq!(
            calls_to(&log, FRIEND_HITS_GET_HIT_COUNT),
            vec![vec![extra.addr()]]
        );

        // Existing extra data: nothing built.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2590, &args![list]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, FRIEND_HITS_ADD_HIT).len(), 1);
    }

    #[test]
    fn friend_hit_answers_the_hit_count() {
        let mut e = engine();
        stub(&mut e, FRIEND_HITS_ADD_HIT);
        e.register(FRIEND_HITS_GET_HIT_COUNT, |_, _| returns(3));
        let list = new_list(&mut e);
        // The exe ends on GetHitCount and keeps its EAX.
        assert_eq!(e.call(0x0042_2590, &args![list]).u32(), 3);
    }

    #[test]
    fn friend_hit_count_getter() {
        let mut e = engine();
        e.register(FRIEND_HITS_GET_HIT_COUNT, |_, _| returns(3));
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0042_2640, &args![list]).u32(), 0);
        let extra = add(&mut e, list, EXTRA_FRIEND_HITS, 0);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_2640, &args![list]).u32(), 3);
        });
        assert_eq!(
            calls_to(&log, FRIEND_HITS_GET_HIT_COUNT),
            vec![vec![extra.addr()]]
        );
    }

    #[test]
    fn friend_hits_remover_removes_the_extra_data() {
        let mut e = engine();
        let list = new_list(&mut e);
        add(&mut e, list, EXTRA_FRIEND_HITS, 0);
        add(&mut e, list, 0x01, 0x1111);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2670, &args![list]);
        });
        assert_eq!(chain_types(&e, list), vec![0x01]);
        assert_eq!(calls_to(&log, DESTRUCTOR).len(), 1);
    }

    #[test]
    fn remove_follower_takes_the_item_out_of_the_follower_list() {
        let mut e = engine();
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(0));
        let removed: Added = Rc::new(RefCell::new(vec![]));
        let seen = removed.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            let item = e.mem.u32(a[1]);
            seen.borrow_mut().push((a[0], item));
            returns(1)
        });
        let list = new_list(&mut e);
        // Without the extra data nothing happens.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2690, &args![list, 0x5555u32]);
        });
        assert!(removed.borrow().is_empty());
        assert!(calls_to(&log, SAVE_LOAD_UNAVAILABLE).is_empty());
        add(&mut e, list, EXTRA_FOLLOWER, 0xaaaa);
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x1234);
        let handler = e.mem.alloc(0x700);
        e.set_global::<u32>(DATA_HANDLER, handler);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2690, &args![list, 0x5555u32]);
        });
        assert_eq!(*removed.borrow(), vec![(0xaaaa, 0x5555)]);
        assert_eq!(calls_to(&log, SAVE_LOAD_UNAVAILABLE), vec![vec![0x1234]]);
    }

    #[test]
    fn data_handler_clearing_flag_getter() {
        let mut e = engine();
        let handler: Ptr = Ptr::new(e.mem.alloc(0x700));
        assert_eq!(e.call(0x0042_26e0, &args![handler]).u8(), 0);
        e.mem.set_u8(handler.addr() + 0x61d, 1);
        assert_eq!(e.call(0x0042_26e0, &args![handler]).u8(), 1);
    }

    #[test]
    fn follower_extra_getter_and_remover() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0042_2700, &args![list]).u32(), 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2720, &args![list]);
        });
        assert!(calls_to(&log, DESTRUCTOR).is_empty());
        let extra = add(&mut e, list, EXTRA_FOLLOWER, 0xaaaa);
        assert_eq!(e.call(0x0042_2700, &args![list]).u32(), extra.addr());
        let log = logged(&mut e, |e| {
            e.call(0x0042_2720, &args![list]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn refraction_property_setter_builds_updates_and_removes() {
        let mut e = engine();
        let list = new_list(&mut e);
        // Disabling with nothing there does nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2750, &args![list, 0u8, 1.5f32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(chain_types(&e, list).is_empty());
        // Enabling builds the extra data (constructor given the float).
        let log = logged(&mut e, |e| {
            e.call(0x0042_2750, &args![list, 1u8, 1.5f32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let built: Vec<u32> = calls_to(&log, REFRACTION_PROPERTY_INIT)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(built, vec![1.5f32.to_bits()]);
        assert_eq!(chain_types(&e, list), vec![EXTRA_REFRACTION_PROPERTY]);
        assert_eq!(
            word_of(&mut e, list, EXTRA_REFRACTION_PROPERTY),
            1.5f32.to_bits()
        );
        // Enabling again only stores the value.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2750, &args![list, 7u8, 2.5f32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            word_of(&mut e, list, EXTRA_REFRACTION_PROPERTY),
            2.5f32.to_bits()
        );
        // The getter returns the extra data.
        let extra = find_extra(&mut e, list, EXTRA_REFRACTION_PROPERTY);
        assert_eq!(e.call(0x0042_2820, &args![list]).u32(), extra.addr());
        // Disabling removes and deletes it.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2750, &args![list, 0u8, 0.0f32]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
        assert_eq!(e.call(0x0042_2820, &args![list]).u32(), 0);
    }

    #[test]
    fn last_finished_sequence_replaces_the_old_name() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0042_28f0, &args![list]).u32(), 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2850, &args![list, 0x7000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let built: Vec<u32> = calls_to(&log, LAST_FINISHED_SEQUENCE_INIT)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(built, vec![0x7000]);
        assert_eq!(e.call(0x0042_28f0, &args![list]).u32(), 0x7000);
        // A second set removes the first extra data and builds another.
        let first = find_extra(&mut e, list, EXTRA_LASTFINISHEDSEQUENCE);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2850, &args![list, 0x7100u32]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![first.addr(), 1]]);
        assert_eq!(e.call(0x0042_28f0, &args![list]).u32(), 0x7100);
        // A null name only removes it.
        let log = logged(&mut e, |e| {
            e.call(0x0042_2850, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(chain_types(&e, list).is_empty());
        // The plain remover.
        e.call(0x0042_2850, &args![list, 0x7200u32]);
        e.call(0x0042_2920, &args![list]);
        assert!(chain_types(&e, list).is_empty());
    }

    /// Setter of a saved buffer (types `0x42`, `0x3D`): builds the extra data,
    /// warns and deletes the old buffer when it exists, removes on 0.
    fn check_saved_buffer_setter(address: u32, extra_type: u8, construct: u32, message: u32) {
        let mut e = engine();
        stub(&mut e, SAVE_GAME_WARNING);
        stub(&mut e, DELETE_SAVED_BUFFER);
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x9000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, construct).len(), 1);
        assert!(calls_to(&log, SAVE_GAME_WARNING).is_empty());
        assert_eq!(word_of(&mut e, list, extra_type), 0x9000);
        let extra = find_extra(&mut e, list, extra_type);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x9100u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, SAVE_GAME_WARNING), vec![vec![message]]);
        assert_eq!(
            calls_to(&log, DELETE_SAVED_BUFFER),
            vec![vec![extra.addr()]]
        );
        assert_eq!(word_of(&mut e, list, extra_type), 0x9100);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0u32]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn saved_animation_setter() {
        check_saved_buffer_setter(
            0x0042_2940,
            EXTRA_SAVED_ANIMATION,
            SAVED_ANIMATION_INIT,
            MESSAGE_SAVED_ANIMATION_EXISTS,
        );
    }

    #[test]
    fn saved_havok_data_setter() {
        check_saved_buffer_setter(
            0x0042_2ac0,
            EXTRA_SAVED_HAVOK_DATA,
            SAVED_HAVOK_DATA_INIT,
            MESSAGE_SAVED_HAVOK_DATA_EXISTS,
        );
    }

    #[test]
    fn saved_buffer_getters() {
        check_word_getter(0x0042_2a10, EXTRA_SAVED_ANIMATION);
        check_word_getter(0x0042_2b90, EXTRA_SAVED_HAVOK_DATA);
    }

    /// Restorer of a saved buffer: loads it into the destination (through a
    /// holder of the buffer address), clears the word and removes the extra
    /// data; false when there is none.
    fn check_saved_buffer_restorer(address: u32, extra_type: u8, load: u32) {
        let mut e = engine();
        e.register(STORE_VALUE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        let loaded: Added = Rc::new(RefCell::new(vec![]));
        let seen = loaded.clone();
        e.register_double(load, move |e, a| {
            let buffer = e.mem.u32(a[0]);
            seen.borrow_mut().push((buffer, a[1]));
            Ret::default()
        });
        let list = new_list(&mut e);
        assert!(!e.call(address, &args![list, 0x3000u32]).bool());
        assert!(loaded.borrow().is_empty());
        let extra = add(&mut e, list, extra_type, 0x9000);
        let log = logged(&mut e, |e| {
            assert!(e.call(address, &args![list, 0x3000u32]).bool());
        });
        assert_eq!(*loaded.borrow(), vec![(0x9000, 0x3000)]);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0);
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![extra.addr(), 1]]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn saved_animation_restorer() {
        check_saved_buffer_restorer(0x0042_2a40, EXTRA_SAVED_ANIMATION, LOAD_ANIMATION);
    }

    #[test]
    fn saved_havok_data_restorer() {
        check_saved_buffer_restorer(0x0042_2bc0, EXTRA_SAVED_HAVOK_DATA, LOAD_HAVOK_DATA);
    }

    #[test]
    fn saved_buffer_removers() {
        for (address, extra_type) in [
            (0x0042_2aa0, EXTRA_SAVED_ANIMATION),
            (0x0042_2c20, EXTRA_SAVED_HAVOK_DATA),
        ] {
            let mut e = engine();
            let list = new_list(&mut e);
            add(&mut e, list, extra_type, 0x9000);
            e.call(address, &args![list]);
            assert!(chain_types(&e, list).is_empty());
        }
    }

    // ---- the save size (00422c40) ----

    /// An object whose vtable has the double `target` in the slot at byte
    /// offset `slot`.
    fn object_with_slot(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let mut slots = vec![0u32; (slot / 4 + 1) as usize];
        slots[(slot / 4) as usize] = target;
        let vtable = e.mem.alloc(4 * slots.len() as u32);
        e.put_vtable(vtable, &slots);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    /// An object (0x40 bytes) whose vtable has the given `(byte offset, target)` slots.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|s| s.0 / 4 + 1).max().unwrap_or(1);
        let mut table = vec![0u32; size as usize];
        for &(slot, target) in slots {
            table[(slot / 4) as usize] = target;
        }
        let vtable = e.mem.alloc(4 * size);
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The byte `00408d60` finds for the debug switch.
    const DEBUG_SWITCH_BYTE: u32 = 0x011d_e600;

    /// An engine for the save size and the writer: doubles for the stream,
    /// the casts (they give the object back), the form id, the list count (the
    /// "list" is the count itself) and the debug switch.
    fn size_engine() -> Engine {
        let mut e = engine();
        e.register(USE_SAVE_GAME_BLOCKS, |_, _| returns(0));
        e.register(SAVE_VERSION, |_, _| returns(0x42));
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        e.register(FORM_ID, |_, a| returns(a[0]));
        e.register(LIST_COUNT, |_, a| returns(a[0]));
        e.register(SETTING_BYTE_ADDRESS, |_, _| returns(DEBUG_SWITCH_BYTE));
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x0100_0000);
        e.mem.set_u8(DEBUG_SWITCH_BYTE, 0);
        e
    }

    fn save_size(e: &mut Engine, extra_type: u8, word: u32, flags: u32, reference: u32) -> u16 {
        let list = new_list(e);
        add(e, list, extra_type, word);
        e.call(0x0042_2c40, &args![list, flags, reference]).u16()
    }

    #[test]
    fn save_size_counts_the_header_and_the_blocks() {
        let mut e = size_engine();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_2c40, &args![list, 0u32, 0u32]).u16(), 2);
        });
        assert_eq!(calls_to(&log, LOCK).len(), 1);
        assert_eq!(calls_to(&log, UNLOCK).len(), 1);
        e.register(USE_SAVE_GAME_BLOCKS, |_, _| returns(1));
        assert_eq!(e.call(0x0042_2c40, &args![list, 0u32, 0u32]).u16(), 8);
    }

    #[test]
    fn save_size_of_each_extra_data_type() {
        let mut e = size_engine();
        e.register(TELEPORT_SAVE_SIZE, |_, _| returns(10));
        e.register(STRING_LENGTH_OF, |_, _| returns(7));
        e.register(INFO_GENERAL_TOPIC_SAVE_SIZE, |_, _| returns(9));
        let actor = object_with_slot(&mut e, 0x100, 0x0200_2000);
        e.register(0x0200_2000, |_, _| returns(1));
        // (type, word at +0x0C, change flags, expected size beyond the 2)
        let cases: [(u8, u32, u32, u16); 22] = [
            (0x03, 0, 0, 0),
            (0x0c, 0, 0, 5),
            (0x18, 0, 0, 0x14 + 1),
            (0x1b, 3, 0, 2 + 15 + 1),
            (0x1d, 2, 0, 2 + 8 + 1),
            (0x1f, 0, 0, 1),
            (0x2a, 0, 0, 0),
            (0x2a, 0, 0x1000, 7),
            (0x2b, 0x77, 0, 0),
            (0x2b, 0x77, 0x2_0000, 11),
            (0x39, 0, 0, 5),
            (0x41, 0x55, 0, 0),
            (0x41, 0x55, 0x1000_0000, 1 + 7 + 1),
            (0x46, 0, 0, 5),
            (0x4d, 0, 0, 0),
            (0x4d, 0x66, 0, 10),
            (0x4e, 0, 0, 2),
            // Types whose masks are 0 in this build, and unknown ones.
            (0x21, 0x1234, 0xffff_ffff, 0),
            (0x2c, 0, 0xffff_ffff, 0),
            (0x8d, 0, 0xffff_ffff, 0),
            (0x90, 0, 0xffff_ffff, 0),
            (0x91, 0, 0xffff_ffff, 0),
        ];
        for (extra_type, word, flags, expected) in cases {
            let reference = if extra_type == 0x0c { actor } else { 0 };
            assert_eq!(
                save_size(&mut e, extra_type, word, flags, reference),
                2 + expected,
                "type {extra_type:#x} flags {flags:#x}"
            );
        }
        // The persistent cell counts only for an actor.
        assert_eq!(save_size(&mut e, 0x0c, 0, 0, 0), 2);
        let not_actor = object_with_slot(&mut e, 0x100, 0x0200_2100);
        e.register(0x0200_2100, |_, _| returns(0));
        assert_eq!(save_size(&mut e, 0x0c, 0, 0, not_actor), 2);
    }

    #[test]
    fn save_size_of_a_package() {
        let mut e = size_engine();
        e.register(SAVE_VERSION, |_, _| returns(0x3f));
        let package = object_with_slot(&mut e, 0x14c, 0x0200_2200);
        e.register(0x0200_2200, |_, _| returns(5));
        // Older save versions: 14 and the id byte.
        assert_eq!(save_size(&mut e, 0x19, package, 0, 0), 2 + 15);
        e.register(SAVE_VERSION, |_, _| returns(0x40));
        e.set_global::<u32>(DATA_HANDLER, 0x0300_0000);
        e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(0));
        assert_eq!(save_size(&mut e, 0x19, package, 0, 0), 2 + 15);
        let log = logged(&mut e, |e| {
            e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(1));
            // 14, 1, the package's 5, and the id byte.
            assert_eq!(save_size(e, 0x19, package, 0, 0), 2 + 14 + 1 + 5 + 1);
        });
        assert_eq!(
            calls_to(&log, FORM_ID_IS_FILE_FORM),
            vec![vec![0x0300_0000, package]]
        );
    }

    #[test]
    fn save_size_logs_what_it_computed_when_the_switch_is_set() {
        let mut e = size_engine();
        e.register(ERROR_LOG, |_, _| Ret::default());
        e.mem.set_u8(DEBUG_SWITCH_BYTE, 1);
        e.register(SAVING_FORM_HEADER, |_, _| returns(0));
        let list = new_list(&mut e);
        add(&mut e, list, 0x4e, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_2c40, &args![list, 0u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                GET_SAVE_SIZE_SHORT_FORMAT,
                4,
                0x29b5,
                SOURCE_FILE_NAME
            ]]
        );
        // With a world space: its id, name and flags.
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, 0x3c);
        e.mem.set_u32(header + 5, 0x0a0b_0c0d);
        e.register_double(SAVING_FORM_HEADER, move |_, _| returns(header));
        let form = object_with_slot(&mut e, FORM_NAME_SLOT, 0x0200_2310);
        e.register_double(LOOKUP_FORM, move |_, a| {
            assert_eq!(a[0], 0x3c);
            returns(form)
        });
        e.register(0x0200_2310, |_, _| returns(0x5000));
        let log = logged(&mut e, |e| {
            e.call(0x0042_2c40, &args![list, 0u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                GET_SAVE_SIZE_FORMAT,
                4,
                0x3c,
                0x5000,
                0x0a0b_0c0d,
                0x29b5,
                SOURCE_FILE_NAME
            ]]
        );
    }

    // ---- the save writer (004235e0) ----

    /// Where the stream double keeps its position, and the buffer it writes.
    const STREAM_POSITION: u32 = 0x011d_e700;
    const STREAM_BUFFER: u32 = 0x0300_0000;

    /// `Save(buffer, size)` and `SaveNumericID(buffer, size)` of the stream
    /// double: copy the bytes to the buffer and move the position.
    fn stream_write(e: &mut Engine, a: &[u32]) -> Ret {
        let position = e.mem.u32(STREAM_POSITION);
        for offset in 0..a[2] {
            let byte = e.mem.u8(a[1] + offset);
            e.mem.set_u8(position + offset, byte);
        }
        e.mem.set_u32(STREAM_POSITION, position + a[2]);
        Ret::default()
    }

    /// An engine with the stream double (a 128 KiB buffer) and the doubles of
    /// the save size.
    fn stream_engine() -> Engine {
        let mut e = size_engine();
        e.map(STREAM_BUFFER, 0x2_0000);
        e.mem.set_u32(STREAM_POSITION, STREAM_BUFFER);
        e.register(SAVE_POSITION, |e, _| returns(e.mem.u32(STREAM_POSITION)));
        e.register(SAVE_WRITE, stream_write);
        e.register(SAVE_NUMERIC_ID, stream_write);
        // `BSSimpleList::IsEmpty`: no item and no next.
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e
    }

    /// The bytes written so far.
    fn stream_bytes(e: &Engine) -> Vec<u8> {
        let end = e.mem.u32(STREAM_POSITION);
        (STREAM_BUFFER..end).map(|a| e.mem.u8(a)).collect()
    }

    /// A list with the extra data `(type, word at +0x0C)`, saved with the
    /// given change flags and reference; returns the bytes.
    fn save_bytes_of(e: &mut Engine, extras: &[(u8, u32)], flags: u32, reference: u32) -> Vec<u8> {
        let list = new_list(e);
        for &(extra_type, word) in extras {
            add(e, list, extra_type, word);
        }
        save_list(e, list, flags, reference)
    }

    /// Saves the list from the start of the stream buffer; returns the bytes.
    fn save_list(e: &mut Engine, list: Ptr<ExtraDataList>, flags: u32, reference: u32) -> Vec<u8> {
        e.mem.set_u32(STREAM_POSITION, STREAM_BUFFER);
        e.call(0x0042_35e0, &args![list, flags, reference]);
        stream_bytes(e)
    }

    #[test]
    fn save_game_of_an_empty_list_and_with_blocks() {
        let mut e = stream_engine();
        let log = logged(&mut e, |e| {
            let bytes = save_bytes_of(e, &[], 0, 0);
            assert_eq!(bytes, vec![0, 0]);
        });
        assert_eq!(calls_to(&log, LOCK).len(), 1);
        assert_eq!(calls_to(&log, UNLOCK).len(), 1);
        e.register(USE_SAVE_GAME_BLOCKS, |_, _| returns(1));
        // The tag, the block size (from its own position to the end), the count.
        let bytes = save_bytes_of(&mut e, &[], 0, 0);
        assert_eq!(bytes, vec![0x4b, 0x4f, 0x4c, 0x42, 4, 0, 0, 0]);
    }

    #[test]
    fn save_game_block_larger_than_sixteen_bits_is_reported() {
        let mut e = stream_engine();
        e.register(USE_SAVE_GAME_BLOCKS, |_, _| returns(1));
        e.register(DOOR_TELEPORT_DATA_SAVE_GAME, |e, _| {
            let position = e.mem.u32(STREAM_POSITION);
            e.mem.set_u32(STREAM_POSITION, position + 0x1_0000);
            Ret::default()
        });
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            let bytes = save_bytes_of(e, &[(0x2b, 0x55)], 0x2_0000, 0);
            assert_eq!(bytes.len(), 8 + 1 + 0x1_0000);
            // The block size is stored as 16 bits.
            let size = bytes.len() as u32 - 4;
            assert_eq!(bytes[4..6], [size as u8, (size >> 8) as u8]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![SAVE_BLOCK_TOO_LARGE_MESSAGE, SOURCE_FILE_NAME, 0x2c1f]]
        );
    }

    #[test]
    fn save_game_simple_types() {
        let mut e = stream_engine();
        // The ghost: only the type byte.
        assert_eq!(save_bytes_of(&mut e, &[(0x1f, 0)], 0, 0), vec![1, 0, 0x1f]);
        // No rumors: the byte at +0x0C.
        assert_eq!(
            save_bytes_of(&mut e, &[(0x4e, 0x01)], 0, 0),
            vec![1, 0, 0x4e, 1]
        );
        // The item dropper and head track target: an id, 0 for none.
        assert_eq!(
            save_bytes_of(&mut e, &[(0x39, 0x1234)], 0, 0),
            vec![1, 0, 0x39, 0x34, 0x12, 0, 0]
        );
        assert_eq!(
            save_bytes_of(&mut e, &[(0x46, 0)], 0, 0),
            vec![1, 0, 0x46, 0, 0, 0, 0]
        );
        // Types whose masks are 0 write nothing and are not counted.
        assert_eq!(
            save_bytes_of(
                &mut e,
                &[(0x24, 5), (0x21, 0x1234), (0x03, 0)],
                0xffff_ffff,
                0
            ),
            vec![0, 0]
        );
        // The count follows the extra data written: two of three.
        assert_eq!(
            save_bytes_of(&mut e, &[(0x1f, 0), (0x24, 0), (0x4e, 0)], 0, 0),
            vec![2, 0, 0x1f, 0x4e, 0]
        );
    }

    #[test]
    fn save_game_lock_teleport_sequence_and_topic_depend_on_the_flags() {
        let mut e = stream_engine();
        // A lock record: level, key, flags byte.
        let lock_data = e.mem.alloc(0x10);
        e.mem.set_u8(lock_data, 3);
        e.mem.set_u32(lock_data + 4, 0x77);
        e.mem.set_u8(lock_data + 8, 1);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x2a, lock_data)], 0, 0),
            vec![0, 0]
        );
        assert_eq!(
            save_bytes_of(&mut e, &[(0x2a, lock_data)], 0x1000, 0),
            vec![1, 0, 0x2a, 3, 0x77, 0, 0, 0, 1]
        );
        // Without a key the id is 0.
        e.mem.set_u32(lock_data + 4, 0);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x2a, lock_data)], 0x1000, 0),
            vec![1, 0, 0x2a, 3, 0, 0, 0, 0, 1]
        );
        // The teleport data saves itself after the type byte.
        e.register_double(DOOR_TELEPORT_DATA_SAVE_GAME, |e, a| {
            let position = e.mem.u32(STREAM_POSITION);
            e.mem.set_u8(position, a[0] as u8);
            e.mem.set_u32(STREAM_POSITION, position + 1);
            Ret::default()
        });
        assert_eq!(save_bytes_of(&mut e, &[(0x2b, 0x55)], 0, 0), vec![0, 0]);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x2b, 0x55)], 0x2_0000, 0),
            vec![1, 0, 0x2b, 0x55]
        );
        // The last finished sequence: a length byte and the characters.
        let name = e.mem.alloc(8);
        for (i, b) in b"abc\0".iter().enumerate() {
            e.mem.set_u8(name + i as u32, *b);
        }
        e.register(STRING_LENGTH_OF, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            returns(n)
        });
        assert_eq!(save_bytes_of(&mut e, &[(0x41, name)], 0, 0), vec![0, 0]);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x41, name)], 0x1000_0000, 0),
            vec![1, 0, 0x41, 3, b'a', b'b', b'c']
        );
        // The info general topic saves itself when it holds data.
        e.register_double(INFO_GENERAL_TOPIC_SAVE, |e, a| {
            let position = e.mem.u32(STREAM_POSITION);
            e.mem.set_u8(position, a[0] as u8);
            e.mem.set_u32(STREAM_POSITION, position + 1);
            Ret::default()
        });
        assert_eq!(save_bytes_of(&mut e, &[(0x4d, 0)], 0, 0), vec![0, 0]);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x4d, 0x66)], 0, 0),
            vec![1, 0, 0x4d, 0x66]
        );
    }

    #[test]
    fn save_game_persistent_cell_is_saved_for_actors_as_the_world_space_id() {
        let mut e = stream_engine();
        e.register(CELL_GET_WORLD_SPACE, |_, a| returns(a[0] + 1));
        let actor = object_with_slot(&mut e, 0x100, 0x0200_2000);
        e.register(0x0200_2000, |_, _| returns(1));
        let other = object_with_slot(&mut e, 0x100, 0x0200_2100);
        e.register(0x0200_2100, |_, _| returns(0));
        assert_eq!(
            save_bytes_of(&mut e, &[(0x0c, 0x65)], 0, actor),
            vec![1, 0, 0x0c, 0x66, 0, 0, 0]
        );
        assert_eq!(
            save_bytes_of(&mut e, &[(0x0c, 0)], 0, actor),
            vec![1, 0, 0x0c, 0, 0, 0, 0]
        );
        assert_eq!(save_bytes_of(&mut e, &[(0x0c, 0x65)], 0, other), vec![0, 0]);
        assert_eq!(save_bytes_of(&mut e, &[(0x0c, 0x65)], 0, 0), vec![0, 0]);
    }

    #[test]
    fn save_game_package_start_location() {
        let mut e = stream_engine();
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        let reference = object_with_slot(&mut e, 0x130, 0x0200_2300);
        e.register(0x0200_2300, |_, _| returns(0x5000));
        let list = new_list(&mut e);
        let location = add(&mut e, list, 0x18, 0x21).addr();
        for i in 0..12 {
            e.mem.set_u8(location + 0x10 + i, 0x40 + i as u8);
        }
        for i in 0..4 {
            e.mem.set_u8(location + 0x1c + i, 0x60 + i as u8);
        }
        let log = logged(&mut e, |e| {
            let bytes = save_list(e, list, 0, reference);
            let mut expected = vec![1, 0, 0x18, 0x21, 0, 0, 0];
            expected.extend((0..12).map(|i| 0x40 + i as u8));
            expected.extend((0..4).map(|i| 0x60 + i as u8));
            assert_eq!(bytes, expected);
        });
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        // Without a location a message names the reference, and the id is 0.
        e.mem.set_u32(location + 0x0c, 0);
        let log = logged(&mut e, |e| {
            let bytes = save_list(e, list, 0, reference);
            assert_eq!(bytes[2..7], [0x18, 0, 0, 0, 0]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![PACKAGE_START_LOCATION_NULL_MESSAGE, reference, 0x5000]]
        );
    }

    #[test]
    fn save_game_package_writes_the_type_byte_and_data_for_file_forms() {
        let mut e = stream_engine();
        e.register(PACKAGE_GET_TYPE, |_, _| returns(0x0a));
        e.register_double(0x0200_2400, |e, a| {
            let position = e.mem.u32(STREAM_POSITION);
            e.mem.set_u8(position, 0xee);
            e.mem.set_u32(STREAM_POSITION, position + 1);
            assert_ne!(a[0], 0);
            Ret::default()
        });
        let form = object_with_slot(&mut e, 0x150, 0x0200_2400);
        let list = new_list(&mut e);
        let package = add(&mut e, list, 0x19, form).addr();
        e.mem.set_u32(package + 0x10, 0x0d0c_0b0a);
        e.mem.set_u32(package + 0x14, 0x31);
        e.mem.set_u8(package + 0x18, 7);
        e.mem.set_u8(package + 0x19, 8);
        e.set_global::<u32>(DATA_HANDLER, 0x0300_0000 - 8);
        e.register(SAVE_VERSION, |_, _| returns(0x3f));
        let mut expected = vec![1, 0, 0x19];
        expected.extend(form.to_le_bytes());
        expected.extend([0x0a, 0x0b, 0x0c, 0x0d]);
        expected.extend([0x31, 0, 0, 0, 7, 8]);
        // Before save version 0x40: nothing more.
        assert_eq!(save_list(&mut e, list, 0, 0), expected);
        e.register(SAVE_VERSION, |_, _| returns(0x40));
        e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(0));
        assert_eq!(save_list(&mut e, list, 0, 0), expected);
        // For a file form: the package's type byte and its own data.
        e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(1));
        expected.extend([0x0a, 0xee]);
        assert_eq!(save_list(&mut e, list, 0, 0), expected);
        // No second form: its id is 0.
        e.mem.set_u32(package + 0x14, 0);
        assert_eq!(save_list(&mut e, list, 0, 0)[11..15], [0, 0, 0, 0]);
    }

    /// A list node `{item, next}`.
    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    #[test]
    fn save_game_run_once_packages_and_followers_write_their_items() {
        let mut e = stream_engine();
        // Run once packages: items are {form, byte}; a null item is skipped.
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x11);
        e.mem.set_u8(first + 4, 0xa1);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x22);
        e.mem.set_u8(second + 4, 0xa2);
        let tail = list_node(&mut e, second, 0);
        let skipped = list_node(&mut e, 0, tail);
        let head = list_node(&mut e, first, skipped);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x1b, head)], 0, 0),
            vec![1, 0, 0x1b, 2, 0, 0x11, 0, 0, 0, 0xa1, 0x22, 0, 0, 0, 0xa2]
        );
        assert_eq!(
            save_bytes_of(&mut e, &[(0x1b, 0)], 0, 0),
            vec![1, 0, 0x1b, 0, 0]
        );
        // Followers: the items are the forms themselves.
        let tail = list_node(&mut e, 0x32, 0);
        let head = list_node(&mut e, 0x31, tail);
        assert_eq!(
            save_bytes_of(&mut e, &[(0x1d, head)], 0, 0),
            vec![1, 0, 0x1d, 2, 0, 0x31, 0, 0, 0, 0x32, 0, 0, 0]
        );
    }

    #[test]
    fn save_game_logs_what_it_wrote_when_the_switch_is_set() {
        let mut e = stream_engine();
        e.register(ERROR_LOG, |_, _| Ret::default());
        e.register(SAVING_FORM_HEADER, |_, _| returns(0));
        e.mem.set_u8(DEBUG_SWITCH_BYTE, 1);
        let log = logged(&mut e, |e| {
            save_bytes_of(e, &[(0x1f, 0)], 0, 0);
        });
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                SAVE_GAME_SIZE_SHORT_FORMAT,
                3,
                0x2c1f,
                SOURCE_FILE_NAME
            ]]
        );
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, 0x3c);
        e.mem.set_u32(header + 5, 0x0a0b_0c0d);
        e.register_double(SAVING_FORM_HEADER, move |_, _| returns(header));
        let form = object_with_slot(&mut e, FORM_NAME_SLOT, 0x0200_2310);
        e.register_double(LOOKUP_FORM, move |_, _| returns(form));
        e.register(0x0200_2310, |_, _| returns(0x5000));
        let log = logged(&mut e, |e| {
            save_bytes_of(e, &[(0x1f, 0)], 0, 0);
        });
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                SAVE_GAME_SIZE_FORMAT,
                3,
                0x3c,
                0x5000,
                0x0a0b_0c0d,
                0x2c1f,
                SOURCE_FILE_NAME
            ]]
        );
    }

    // ---- the loader of the old save format (00424960) ----

    /// Where the read stream double keeps its cursor, and its input.
    const READ_CURSOR: u32 = 0x011d_e710;
    const READ_INPUT: u32 = 0x0301_0000;

    /// `Read(buffer, size)` and `LoadNumericID(buffer, size)` of the stream
    /// double: copy the next bytes of the input.
    fn stream_read(e: &mut Engine, a: &[u32]) -> Ret {
        let cursor = e.mem.u32(READ_CURSOR);
        for offset in 0..a[2] {
            let byte = e.mem.u8(cursor + offset);
            e.mem.set_u8(a[1] + offset, byte);
        }
        e.mem.set_u32(READ_CURSOR, cursor + a[2]);
        Ret::default()
    }

    /// An engine with the read stream double and the doubles of the save
    /// size; the form of an id is the id itself and a cast gives it back.
    fn read_engine() -> Engine {
        let mut e = size_engine();
        e.map(READ_INPUT, 0x1000);
        e.mem.set_u32(READ_CURSOR, READ_INPUT);
        e.register(SAVE_READ, stream_read);
        e.register(LOAD_NUMERIC_ID, stream_read);
        e.register(SAVE_POSITION, |e, _| returns(e.mem.u32(READ_CURSOR)));
        e.register(LOOKUP_FORM, |_, a| returns(a[0]));
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        e
    }

    /// The stream of `items` extra data: a 16-bit count and their bytes.
    fn extra_stream(items: &[&[u8]]) -> Vec<u8> {
        let mut bytes = (items.len() as u16).to_le_bytes().to_vec();
        for item in items {
            bytes.extend_from_slice(item);
        }
        bytes
    }

    /// Feeds `bytes` to the stream and runs `LoadGame`; every byte must be
    /// read.
    fn load_game(
        e: &mut Engine,
        list: Ptr<ExtraDataList>,
        flags: u32,
        reference: u32,
        bytes: &[u8],
    ) {
        for (i, byte) in bytes.iter().enumerate() {
            e.mem.set_u8(READ_INPUT + i as u32, *byte);
        }
        e.mem.set_u32(READ_CURSOR, READ_INPUT);
        e.call(0x0042_4960, &args![list, flags, 0u32, reference]);
        assert_eq!(e.mem.u32(READ_CURSOR), READ_INPUT + bytes.len() as u32);
    }

    /// The same, for a list of its own; returns the list.
    fn load_into_new_list(
        e: &mut Engine,
        flags: u32,
        reference: u32,
        bytes: &[u8],
    ) -> Ptr<ExtraDataList> {
        let list = new_list(e);
        load_game(e, list, flags, reference, bytes);
        list
    }

    #[test]
    fn extra_byte_getter() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u8(object.addr() + 0x0c, 0x5a);
        assert_eq!(e.call(0x0042_4940, &args![object]).u8(), 0x5a);
    }

    #[test]
    fn load_game_simple_types_and_the_types_without_payload() {
        let mut e = read_engine();
        stub(&mut e, SET_GHOST);
        let log = logged(&mut e, |e| {
            let list = load_into_new_list(
                e,
                0,
                0,
                &extra_stream(&[
                    &[0x1f],
                    &[0x4e, 1],
                    &[0x2e],
                    &[0x90],
                    &[0x91],
                    &[0x45],
                    &[0x24],
                ]),
            );
            assert_eq!(chain_types(e, list), vec![0x4e]);
        });
        assert_eq!(calls_to(&log, SET_GHOST).len(), 1);
        assert_eq!(calls_to(&log, SET_GHOST)[0][1], 1);
        // Nothing was reported.
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
    }

    #[test]
    fn load_game_reports_a_type_it_does_not_know() {
        let mut e = read_engine();
        let log = logged(&mut e, |e| {
            load_into_new_list(e, 0, 0, &extra_stream(&[&[0x40], &[0x47], &[0x03]]));
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![
                vec![LOAD_GAME_UNKNOWN_TYPE_FORMAT, 0x40],
                vec![LOAD_GAME_UNKNOWN_TYPE_FORMAT, 0x47],
                vec![LOAD_GAME_UNKNOWN_TYPE_FORMAT, 0x03],
            ]
        );
    }

    #[test]
    fn load_game_lock_reads_the_record_by_save_version() {
        let mut e = read_engine();
        e.register(LOCK_RECORD_INIT, |_, a| returns(a[0]));
        let locks: Added = Rc::new(RefCell::new(vec![]));
        let seen = locks.clone();
        e.register_double(SET_LOCK_PTR, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        // Without the change flag nothing is read.
        let list = load_into_new_list(&mut e, 0, 0, &extra_stream(&[&[0x2a]]));
        assert!(locks.borrow().is_empty());
        // Version 0x42: level, key id, flags; the key is the cast form.
        let mut record = vec![0x2a, 3];
        record.extend(0x77u32.to_le_bytes());
        record.push(1);
        let list = {
            let log = logged(&mut e, |e| {
                load_game(e, list, 0x1000, 0, &extra_stream(&[&record]));
            });
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
            assert_eq!(
                calls_to(&log, RT_DYNAMIC_CAST),
                vec![
                    vec![0, 0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0],
                    vec![0x77, 0, 0x0118_3028, 0x0118_41b4, 0]
                ]
            );
            list
        };
        let (target, lock_data) = locks.borrow()[0];
        assert_eq!(target, list.addr());
        assert_eq!(e.mem.u8(lock_data), 3);
        assert_eq!(e.mem.u32(lock_data + 4), 0x77);
        assert_eq!(e.mem.u8(lock_data + 8), 1);
        // Version 0x19: a second id is read and dropped.
        e.register(SAVE_VERSION, |_, _| returns(0x19));
        let mut record = vec![0x2a, 4];
        record.extend(0u32.to_le_bytes());
        record.extend(0x99u32.to_le_bytes());
        record.push(2);
        load_game(&mut e, list, 0x1000, 0, &extra_stream(&[&record]));
        let (_, lock_data) = locks.borrow()[1];
        assert_eq!(e.mem.u8(lock_data), 4);
        assert_eq!(e.mem.u32(lock_data + 4), 0);
        assert_eq!(e.mem.u8(lock_data + 8), 2);
        // Before version 0x15 the record stays as the constructor left it.
        e.register(SAVE_VERSION, |_, _| returns(0x14));
        load_game(&mut e, list, 0x1000, 0, &extra_stream(&[&[0x2a]]));
        let (_, lock_data) = locks.borrow()[2];
        assert_eq!(e.mem.u8(lock_data), 0);
    }

    #[test]
    fn load_game_scale_is_read_before_version_0x43() {
        let mut e = read_engine();
        let scales: Added = Rc::new(RefCell::new(vec![]));
        let seen = scales.clone();
        e.register_double(SET_SCALE, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let seen = scales.clone();
        e.register_double(REFERENCE_SET_SCALE, move |_, a| {
            seen.borrow_mut().push((a[0] | 0x8000_0000, a[1]));
            Ret::default()
        });
        let mut item = vec![0x30];
        item.extend(1.5f32.to_bits().to_le_bytes());
        // The list's setter without a reference, the reference's with one.
        let list = load_into_new_list(&mut e, 0x10, 0, &extra_stream(&[&item]));
        load_game(&mut e, list, 0x10, 0x4444, &extra_stream(&[&item]));
        assert_eq!(
            *scales.borrow(),
            vec![
                (list.addr(), 1.5f32.to_bits()),
                (0x4444 | 0x8000_0000, 1.5f32.to_bits())
            ]
        );
        // Without the change flag, and from version 0x43 on, nothing is read.
        load_game(&mut e, list, 0, 0, &extra_stream(&[&[0x30]]));
        e.register(SAVE_VERSION, |_, _| returns(0x43));
        load_game(&mut e, list, 0x10, 0, &extra_stream(&[&[0x30]]));
        assert_eq!(scales.borrow().len(), 2);
    }

    #[test]
    fn load_game_single_ids() {
        let mut e = read_engine();
        for address in [
            ADD_DROPPED_ITEM,
            ADD_DROPPED_ITEM_LIST_ENTRY,
            SET_ITEM_DROPPER,
        ] {
            stub(&mut e, address);
        }
        let id = |n: u32| n.to_le_bytes();
        let mut dropper = vec![0x39];
        dropper.extend(id(0x21));
        let mut dropper_none = vec![0x39];
        dropper_none.extend(id(0));
        let mut target = vec![0x46];
        target.extend(id(0x22));
        let mut target_none = vec![0x46];
        target_none.extend(id(0));
        let mut dropped = vec![0x3a, 3];
        dropped.extend(id(0x23));
        dropped.extend(id(0));
        dropped.extend(id(0x24));
        let log = logged(&mut e, |e| {
            load_into_new_list(
                e,
                0,
                0,
                &extra_stream(&[&dropper, &dropper_none, &target, &target_none, &dropped]),
            );
        });
        assert_eq!(calls_to(&log, ADD_DROPPED_ITEM).len(), 1);
        assert_eq!(calls_to(&log, ADD_DROPPED_ITEM)[0][1], 0x21);
        assert_eq!(calls_to(&log, SET_ITEM_DROPPER).len(), 1);
        assert_eq!(calls_to(&log, SET_ITEM_DROPPER)[0][1], 0x22);
        let entries: Vec<u32> = calls_to(&log, ADD_DROPPED_ITEM_LIST_ENTRY)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(entries, vec![0x23, 0x24]);
    }

    #[test]
    fn load_game_package_start_location() {
        let mut e = read_engine();
        stub(&mut e, PATH_LOCATION_INIT);
        let seen: Records = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(SET_PACKAGE_START_LOCATION, move |e, a| {
            let position = (0..12).map(|i| e.mem.u8(a[3] + i)).collect();
            record.borrow_mut().push((a.to_vec(), position));
            Ret::default()
        });
        let mut item = vec![0x18];
        item.extend(0x31u32.to_le_bytes());
        item.extend((0..12).map(|i| 0x10 + i as u8));
        item.extend(2.5f32.to_bits().to_le_bytes());
        // A form that is neither a world space nor a cell (the casts give it
        // back here, so it is both): the setter gets the position and the
        // rotation.
        let list = load_into_new_list(&mut e, 0, 0, &extra_stream(&[&item]));
        let (args, position) = seen.borrow()[0].clone();
        assert_eq!(args[0], list.addr());
        assert_eq!(args[1..3], [0x31, 0x31]);
        assert_eq!(args[4], 2.5f32.to_bits());
        assert_eq!(
            position,
            (0..12).map(|i| 0x10 + i as u8).collect::<Vec<_>>()
        );
        // Casts that fail: the setter is not called.
        e.register(RT_DYNAMIC_CAST, |_, _| returns(0));
        load_game(&mut e, list, 0, 0, &extra_stream(&[&item]));
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn load_game_package() {
        let mut e = read_engine();
        let set: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(vec![]));
        let record = set.clone();
        e.register_double(SET_PACKAGE_EXTRA, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let mut item = vec![0x19];
        item.extend(0x41u32.to_le_bytes());
        item.extend(0x0d0c_0b0au32.to_le_bytes());
        item.extend(0x42u32.to_le_bytes());
        item.extend([7, 8]);
        // Before version 0x40: the package is the form the id names.
        e.register(SAVE_VERSION, |_, _| returns(0x3f));
        let list = load_into_new_list(&mut e, 0, 0, &extra_stream(&[&item]));
        assert_eq!(
            *set.borrow(),
            vec![vec![list.addr(), 0x41, 0x0d0c_0b0a, 0x42, 7, 8, 0]]
        );
        // From version 0x40, for a file form: a type byte follows, the package
        // is built by the save-load object and loaded (virtual +0x154).
        e.register(SAVE_VERSION, |_, _| returns(0x40));
        e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(1));
        e.set_global::<u32>(DATA_HANDLER, 0x0300_0000);
        e.register_double(0x0200_3000, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 0xbeef);
            Ret::default()
        });
        let package = object_with_slot(&mut e, 0x154, 0x0200_3000);
        e.register_double(LOAD_PACKAGE_BY_ID, move |_, a| {
            assert_eq!(a[1..], [0x41, 0x0c]);
            returns(package)
        });
        let mut with_type = item.clone();
        with_type.push(0x0c);
        load_game(&mut e, list, 0, 0, &extra_stream(&[&with_type]));
        assert_eq!(set.borrow()[1][1], package);
        assert_eq!(e.mem.u32(package + 0x10), 0xbeef);
        // The cast failing leaves no package to set.
        e.register(SAVE_VERSION, |_, _| returns(0x3f));
        e.register(RT_DYNAMIC_CAST, |_, _| returns(0));
        load_game(&mut e, list, 0, 0, &extra_stream(&[&item]));
        assert_eq!(set.borrow().len(), 2);
    }

    #[test]
    fn load_game_run_once_packages() {
        let mut e = read_engine();
        let added: Added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(ADD_RUN_ONCE_PACKAGE, move |_, a| {
            seen.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let mut item = vec![0x1b, 2, 0];
        item.extend(0x51u32.to_le_bytes());
        item.push(0xa1);
        item.extend(0x52u32.to_le_bytes());
        item.push(0xa2);
        load_into_new_list(&mut e, 0, 0, &extra_stream(&[&item]));
        assert_eq!(*added.borrow(), vec![(0x51, 0xa1), (0x52, 0xa2)]);
    }

    #[test]
    fn load_game_followers_go_in_as_ids() {
        let (mut e, added) = follower_engine();
        // Stream doubles on top of the follower engine.
        e.map(READ_INPUT, 0x1000);
        e.mem.set_u32(READ_CURSOR, READ_INPUT);
        e.register(SAVE_READ, stream_read);
        e.register(LOAD_NUMERIC_ID, stream_read);
        e.register(USE_SAVE_GAME_BLOCKS, |_, _| returns(0));
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x0100_0000);
        let mut item = vec![0x1d, 2, 0];
        item.extend(0x61u32.to_le_bytes());
        item.extend(0x62u32.to_le_bytes());
        let list = load_into_new_list(&mut e, 0, 0, &extra_stream(&[&item]));
        assert_eq!(chain_types(&e, list), vec![EXTRA_FOLLOWER]);
        assert_eq!(*added.borrow(), vec![(0xaaaa, 0x61), (0xaaaa, 0x62)]);
    }

    #[test]
    fn load_game_teleport_is_built_loaded_and_set() {
        let mut e = read_engine();
        e.register(DOOR_TELEPORT_DATA_INIT, |_, a| returns(a[0]));
        stub(&mut e, DOOR_TELEPORT_DATA_LOAD_GAME);
        stub(&mut e, SET_TELEPORT);
        let list = load_into_new_list(&mut e, 0, 0, &extra_stream(&[&[0x2b]]));
        let log = logged(&mut e, |e| {
            load_game(e, list, 0x2_0000, 0, &extra_stream(&[&[0x2b]]));
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        let data = calls_to(&log, DOOR_TELEPORT_DATA_INIT)[0][0];
        assert_eq!(
            calls_to(&log, DOOR_TELEPORT_DATA_LOAD_GAME),
            vec![vec![data]]
        );
        assert_eq!(calls_to(&log, SET_TELEPORT), vec![vec![list.addr(), data]]);
        // Without the flag nothing is built.
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, 0, &extra_stream(&[&[0x2b]]));
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn load_game_last_finished_sequence() {
        let mut e = read_engine();
        // The table entry of index 1: its first word is the name.
        e.map(SEQUENCE_NAME_TABLE, 0x100);
        e.mem.set_u32(SEQUENCE_NAME_TABLE + 0x24, 0x7000);
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        let item_with_index = {
            let mut item = vec![0x41];
            item.extend(1u32.to_le_bytes());
            item
        };
        // Version 0x16: an index into the table.
        e.register(SAVE_VERSION, |_, _| returns(0x16));
        let list = load_into_new_list(&mut e, 0x1000_0000, 0, &extra_stream(&[&item_with_index]));
        assert_eq!(word_of(&mut e, list, EXTRA_LASTFINISHEDSEQUENCE), 0x7000);
        // Without the flag nothing is read.
        load_game(&mut e, list, 0, 0, &extra_stream(&[&[0x41]]));
        // Version 0x42: a length and the name, copied to a zeroed buffer.
        e.register(SAVE_VERSION, |_, _| returns(0x42));
        let names: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(vec![]));
        let seen = names.clone();
        e.register_double(LAST_FINISHED_SEQUENCE_INIT, move |e, a| {
            seen.borrow_mut()
                .push((0..5).map(|i| e.mem.u8(a[1] + i)).collect());
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, EXTRA_LASTFINISHEDSEQUENCE);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            returns(a[0])
        });
        load_game(
            &mut e,
            list,
            0x1000_0000,
            0,
            &extra_stream(&[&[0x41, 3, b'a', b'b', b'c']]),
        );
        assert_eq!(*names.borrow(), vec![vec![b'a', b'b', b'c', 0, 0]]);
    }

    #[test]
    fn load_game_info_general_topic() {
        let mut e = read_engine();
        e.register(MENU_TOPIC_INIT, |_, a| returns(a[0]));
        stub(&mut e, MENU_TOPIC_LOAD_GAME);
        stub(&mut e, SET_INFO_GENERAL_TOPIC);
        stub(&mut e, MENU_TOPIC_DELETE);
        e.register(MENU_TOPIC_CHECK, |_, _| returns(1));
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, 0x4444, &extra_stream(&[&[0x4d]]));
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x2c]]);
        let topic = calls_to(&log, MENU_TOPIC_INIT)[0][0];
        assert_eq!(
            calls_to(&log, MENU_TOPIC_LOAD_GAME),
            vec![vec![topic, 0x4444]]
        );
        assert_eq!(
            calls_to(&log, SET_INFO_GENERAL_TOPIC),
            vec![vec![list.addr(), topic]]
        );
        assert!(calls_to(&log, MENU_TOPIC_DELETE).is_empty());
        // A topic that does not pass the check is deleted.
        e.register(MENU_TOPIC_CHECK, |_, _| returns(0));
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, 0x4444, &extra_stream(&[&[0x4d]]));
        });
        let topic = calls_to(&log, MENU_TOPIC_INIT)[0][0];
        assert!(calls_to(&log, SET_INFO_GENERAL_TOPIC).is_empty());
        assert_eq!(calls_to(&log, MENU_TOPIC_DELETE), vec![vec![topic, 1]]);
    }

    #[test]
    fn load_game_persistent_cell_of_an_actor() {
        let mut e = read_engine();
        let actor = object_with_slot(&mut e, 0x100, 0x0200_2000);
        e.register(0x0200_2000, |_, _| returns(1));
        e.register(GET_PERSISTENT_CELL, |_, _| returns(0x1111));
        for address in [
            SET_PERSISTENT_CELL,
            CELL_REMOVE_REFERENCE,
            CELL_ADD_REFERENCE,
        ] {
            stub(&mut e, address);
        }
        e.register(REFERENCE_GET_REF_PERSISTS, |_, _| returns(1));
        e.register(WORLD_SPACE_GET_WORD_34, |_, _| returns(0x2222));
        e.register(REFERENCE_GET_PARENT_CELL, |_, _| returns(0x3333));
        e.register(CELL_IS_INTERIOR, |_, _| returns(1));
        let list = new_list(&mut e);
        let mut item = vec![0x0c];
        item.extend(0x55u32.to_le_bytes());
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, actor, &extra_stream(&[&item]));
        });
        assert_eq!(
            calls_to(&log, SET_PERSISTENT_CELL),
            vec![vec![list.addr(), 0], vec![list.addr(), 0x2222]]
        );
        assert_eq!(
            calls_to(&log, CELL_REMOVE_REFERENCE),
            vec![vec![0x1111, actor], vec![0x3333, actor]]
        );
        assert_eq!(
            calls_to(&log, CELL_ADD_REFERENCE),
            vec![vec![0x2222, actor, 0]]
        );
        // An exterior cell: the reference is not taken out of it.
        e.register(CELL_IS_INTERIOR, |_, _| returns(0));
        e.register(GET_PERSISTENT_CELL, |_, _| returns(0));
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, actor, &extra_stream(&[&item]));
        });
        assert!(calls_to(&log, CELL_REMOVE_REFERENCE).is_empty());
        // Not persistent: the id is read and nothing is set.
        e.register(REFERENCE_GET_REF_PERSISTS, |_, _| returns(0));
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, actor, &extra_stream(&[&item]));
        });
        assert!(calls_to(&log, SET_PERSISTENT_CELL).is_empty());
        // Not an actor (or no reference): the id is not even read.
        let log = logged(&mut e, |e| {
            load_game(e, list, 0, 0, &extra_stream(&[&[0x0c]]));
        });
        assert!(calls_to(&log, GET_PERSISTENT_CELL).is_empty());
    }

    #[test]
    fn load_game_block_tag_and_size_checks() {
        let mut e = read_engine();
        e.register(USE_SAVE_GAME_BLOCKS, |_, _| returns(1));
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, 0x3c);
        e.mem.set_u32(header + 5, 0x0a0b_0c0d);
        e.mem.set_u8(header + 9, 7);
        e.register(LOADING_FORM_HEADER, |_, _| returns(0));
        // A well-formed block: the tag, its size (2 + 2 + 1 bytes after the
        // size field's position) and one ghost.
        let mut block = SAVE_BLOCK_TAG.to_le_bytes().to_vec();
        block.extend(5u16.to_le_bytes());
        block.extend(extra_stream(&[&[0x1f]]));
        stub(&mut e, SET_GHOST);
        let log = logged(&mut e, |e| {
            load_into_new_list(e, 0, 0, &block);
        });
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        // A wrong tag is reported (here without a form being loaded).
        let mut bad = block.clone();
        bad[0] = 0;
        let log = logged(&mut e, |e| {
            load_into_new_list(e, 0, 0, &bad);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![
                LOAD_GAME_BAD_TAG_SHORT_FORMAT,
                SOURCE_FILE_NAME,
                0x2c2c,
                0x42
            ]]
        );
        // The block says more than was read, and less.
        let mut longer = block.clone();
        longer[4] = 9;
        let log = logged(&mut e, |e| {
            load_into_new_list(e, 0, 0, &longer);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![
                LOAD_GAME_UNDERREAD_SHORT_FORMAT,
                4,
                SOURCE_FILE_NAME,
                0x2ee9,
                0x42
            ]]
        );
        let mut shorter = block.clone();
        shorter[4] = 3;
        let log = logged(&mut e, |e| {
            load_into_new_list(e, 0, 0, &shorter);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![
                LOAD_GAME_OVERREAD_SHORT_FORMAT,
                2,
                SOURCE_FILE_NAME,
                0x2ee9,
                0x42
            ]]
        );
        // With a form being loaded the messages name it.
        e.register_double(LOADING_FORM_HEADER, move |_, _| returns(header));
        let form = object_with_slot(&mut e, FORM_NAME_SLOT, 0x0200_2310);
        e.register_double(LOOKUP_FORM, move |_, _| returns(form));
        e.register(0x0200_2310, |_, _| returns(0x5000));
        let log = logged(&mut e, |e| {
            load_into_new_list(e, 0, 0, &shorter);
            load_into_new_list(e, 0, 0, &longer);
            load_into_new_list(e, 0, 0, &bad);
        });
        let messages = calls_to(&log, LOG_MESSAGE);
        let named = |format: u32, difference: Option<u32>, line: u32| {
            let mut words = vec![format];
            words.extend(difference);
            words.extend([SOURCE_FILE_NAME, line, 0x3c, 0x5000, 7, 0x0a0b_0c0d]);
            words
        };
        assert_eq!(
            messages,
            vec![
                named(LOAD_GAME_OVERREAD_FORMAT, Some(2), 0x2ee9),
                named(LOAD_GAME_UNDERREAD_FORMAT, Some(4), 0x2ee9),
                named(LOAD_GAME_BAD_TAG_FORMAT, None, 0x2c2c),
            ]
        );
    }

    // ---- the buffered save writer (00426a30) ----

    type Event = (&'static str, Vec<u32>);
    type Events = Rc<RefCell<Vec<Event>>>;
    type Records = Rc<RefCell<Vec<(Vec<u32>, Vec<u8>)>>>;

    /// An engine with a `BGSSaveGameBuffer` double that records what is
    /// saved: `("bytes", [size, first word])`, `("form2", [form])`,
    /// `("form", [form])`, `("string", [pointer])`, `("count", [n])`,
    /// `("end", [count, start])`. The buffer's save kind is the word at
    /// +0x17, its reference (virtual +4) the word at +0x30 and its flag
    /// (virtual +8) the word at +0x34. Returns the buffer too.
    fn buffer_engine() -> (Engine, Events, u32) {
        let mut e = size_engine();
        let events: Events = Rc::new(RefCell::new(vec![]));
        e.map(SAVE_KIND_TABLE, 0x300);
        let record = events.clone();
        e.register_double(BUFFER_SAVE_BYTES, move |e, a| {
            let size = a[2];
            let first =
                (0..size.min(4)).fold(0u32, |w, i| w | (e.mem.u8(a[1] + i) as u32) << (8 * i));
            record.borrow_mut().push(("bytes", vec![size, first]));
            Ret::default()
        });
        for (address, name) in [
            (BUFFER_SAVE_FORM_ID_OV2, "form2"),
            (BUFFER_SAVE_FORM_ID, "form"),
            (BUFFER_SAVE_STRING, "string"),
            (BUFFER_SAVE_VARIABLE_SIZED_VALUE, "count"),
        ] {
            let record = events.clone();
            e.register_double(address, move |_, a| {
                record.borrow_mut().push((name, vec![a[1]]));
                Ret::default()
            });
        }
        e.register(BUFFER_START_VARIABLE_SIZED_VALUE, |_, _| returns(0x5000));
        let record = events.clone();
        e.register_double(BUFFER_SAVE_VARIABLE_SIZED_VALUE_OV2, move |_, a| {
            record.borrow_mut().push(("end", vec![a[1], a[2]]));
            Ret::default()
        });
        e.register(0x0200_4000, |e, a| returns(e.mem.u32(a[0] + 0x30)));
        e.register(0x0200_4004, |e, a| returns(e.mem.u32(a[0] + 0x34)));
        let buffer = object_with_slots(&mut e, &[(4, 0x0200_4000), (8, 0x0200_4004)]);
        e.mem.set_u32(buffer + 0x17, 2);
        (e, events, buffer)
    }

    /// Saves a list of `(type, word at +0x0C)` through `SaveGame_ov2` with the
    /// types' masks all 2 and returns the extra data and the events.
    fn save_ov2(
        e: &mut Engine,
        events: &Events,
        buffer: u32,
        extras: &[(u8, u32)],
    ) -> Vec<Ptr<BSExtraData>> {
        let list = new_list(e);
        let mut made = vec![];
        for &(extra_type, word) in extras {
            e.mem.set_u32(SAVE_KIND_TABLE + extra_type as u32 * 4, 2);
            made.push(add(e, list, extra_type, word));
        }
        events.borrow_mut().clear();
        e.call(0x0042_6a30, &args![list, buffer]);
        made
    }

    /// The events with the frame (the start and the end) removed.
    fn inner_events(events: &Events) -> Vec<(&'static str, Vec<u32>)> {
        let all = events.borrow().clone();
        assert_eq!(all.last().unwrap().0, "end");
        all[..all.len() - 1].to_vec()
    }

    #[test]
    fn save_game_ov2_saves_the_types_the_buffer_kind_allows() {
        let (mut e, events, buffer) = buffer_engine();
        // 0x4e saved; 0x4a has no mask; 0x26 needs a file form; 0x89 needs
        // the buffer's flag.
        e.mem.set_u32(SAVE_KIND_TABLE + 0x26 * 4, 0x4000_0002);
        e.mem.set_u32(SAVE_KIND_TABLE + 0x4a * 4, 0);
        let list = new_list(&mut e);
        let mut made = vec![];
        for (extra_type, word) in [(0x4e, 7), (0x4a, 8), (0x26, 9), (0x89, 0x1234)] {
            made.push(add(&mut e, list, extra_type, word));
        }
        e.mem.set_u32(SAVE_KIND_TABLE + 0x4e * 4, 2);
        e.mem.set_u32(SAVE_KIND_TABLE + 0x89 * 4, 2);
        events.borrow_mut().clear();
        e.call(0x0042_6a30, &args![list, buffer]);
        assert_eq!(
            *events.borrow(),
            vec![
                ("bytes", vec![1, 0x4e]),
                ("bytes", vec![1, 7]),
                ("end", vec![1, 0x5000])
            ]
        );
        // With the flag, the type 0x89 is saved too; with a file form
        // reference, the type 0x26 as well.
        e.mem.set_u32(buffer + 0x34, 1);
        e.mem.set_u32(buffer + 0x30, 0x77);
        e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(1));
        events.borrow_mut().clear();
        e.call(0x0042_6a30, &args![list, buffer]);
        assert_eq!(
            *events.borrow(),
            vec![
                ("bytes", vec![1, 0x4e]),
                ("bytes", vec![1, 7]),
                ("bytes", vec![1, 0x26]),
                ("bytes", vec![1, 9]),
                ("bytes", vec![1, 0x89]),
                ("form2", vec![0x1234]),
                ("end", vec![3, 0x5000])
            ]
        );
        // A kind that does not match the mask: nothing.
        e.mem.set_u32(buffer + 0x17, 4);
        events.borrow_mut().clear();
        e.call(0x0042_6a30, &args![list, buffer]);
        assert_eq!(*events.borrow(), vec![("end", vec![0, 0x5000])]);
    }

    #[test]
    fn save_game_ov2_fixed_size_types() {
        let (mut e, events, buffer) = buffer_engine();
        // (type, word at +0x0C, expected events)
        let cases: Vec<(u8, u32, Vec<Event>)> = vec![
            (0x16, 5, vec![]),
            (0x3e, 5, vec![]),
            (0x1f, 5, vec![]),
            (0x90, 5, vec![]),
            (0x24, 0x1234, vec![("bytes", vec![2, 0x1234])]),
            (0x25, 0x12345678, vec![("bytes", vec![4, 0x12345678])]),
            (0x4e, 0x66, vec![("bytes", vec![1, 0x66])]),
            (0x21, 0x31, vec![("form2", vec![0x31])]),
            (0x39, 0x32, vec![("form2", vec![0x32])]),
            (0x89, 0x33, vec![("form2", vec![0x33])]),
        ];
        for (extra_type, word, expected) in cases {
            e.mem.set_u32(buffer + 0x34, 1);
            save_ov2(&mut e, &events, buffer, &[(extra_type, word)]);
            let mut full = vec![("bytes", vec![1, extra_type as u32])];
            full.extend(expected);
            assert_eq!(inner_events(&events), full, "type {extra_type:#x}");
        }
    }

    #[test]
    fn save_game_ov2_unknown_type_is_reported() {
        let (mut e, events, buffer) = buffer_engine();
        e.register(SAVE_GAME_WARNING, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            save_ov2(e, &events, buffer, &[(0x40, 0)]);
        });
        assert_eq!(
            calls_to(&log, SAVE_GAME_WARNING),
            vec![vec![SAVE_GAME_OV2_UNKNOWN_TYPE_FORMAT, 0x40]]
        );
    }

    /// Saves one extra data of `extra_type` (all masks 2) after `fill` has
    /// set its fields; returns the events after its type byte, without the
    /// frame's end.
    fn save_one(
        e: &mut Engine,
        events: &Events,
        buffer: u32,
        extra_type: u8,
        fill: impl FnOnce(&mut Engine, u32),
    ) -> Vec<(&'static str, Vec<u32>)> {
        let list = new_list(e);
        e.mem.set_u32(SAVE_KIND_TABLE + extra_type as u32 * 4, 2);
        let extra = add(e, list, extra_type, 0);
        fill(e, extra.addr());
        events.borrow_mut().clear();
        e.call(0x0042_6a30, &args![list, buffer]);
        let inner = inner_events(events);
        assert_eq!(inner[0], ("bytes", vec![1, extra_type as u32]));
        inner[1..].to_vec()
    }

    #[test]
    fn save_game_ov2_forms_and_fields() {
        let (mut e, events, buffer) = buffer_engine();
        e.mem.set_u32(buffer + 0x34, 1);
        let b = |size: u32, word: u32| ("bytes", vec![size, word]);
        let f2 = |form: u32| ("form2", vec![form]);
        // EXTRA_PACKAGE
        let got = save_one(&mut e, &events, buffer, 0x19, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x0d0c_0b0a);
            e.mem.set_u32(x + 0x14, 0x42);
            e.mem.set_u8(x + 0x18, 1);
            e.mem.set_u8(x + 0x19, 2);
            e.mem.set_u8(x + 0x1a, 3);
        });
        assert_eq!(
            got,
            vec![
                f2(0x41),
                f2(0x42),
                b(4, 0x0d0c_0b0a),
                b(1, 1),
                b(1, 2),
                b(1, 3)
            ]
        );
        // EXTRA_PACKAGESTARTLOC
        let got = save_one(&mut e, &events, buffer, 0x18, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x1111);
            e.mem.set_u32(x + 0x1c, 0x2222);
        });
        assert_eq!(got, vec![f2(0x41), b(12, 0x1111), b(4, 0x2222)]);
        // The 0x6E pair, the lock, the 4+1 byte type and the two bytes.
        let got = save_one(&mut e, &events, buffer, 0x6e, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x3333);
        });
        assert_eq!(got, vec![f2(0x41), b(4, 0x3333)]);
        let got = save_one(&mut e, &events, buffer, 0x2a, |e, x| {
            let lock_data = e.mem.alloc(0x20);
            e.mem.set_u8(lock_data, 3);
            e.mem.set_u32(lock_data + 4, 0x77);
            e.mem.set_u8(lock_data + 8, 1);
            e.mem.set_u32(lock_data + 0x0c, 0x44);
            e.mem.set_u32(lock_data + 0x10, 0x55);
            e.mem.set_u32(x + 0x0c, lock_data);
        });
        assert_eq!(
            got,
            vec![b(1, 3), b(1, 1), f2(0x77), b(4, 0x44), b(4, 0x55)]
        );
        let got = save_one(&mut e, &events, buffer, 0x2f, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x66);
            e.mem.set_u8(x + 0x10, 9);
        });
        assert_eq!(got, vec![b(4, 0x66), b(1, 9)]);
        let got = save_one(&mut e, &events, buffer, 0x50, |e, x| {
            e.mem.set_u8(x + 0x0c, 5);
            e.mem.set_u8(x + 0x0d, 6);
        });
        assert_eq!(got, vec![b(1, 5), b(1, 6)]);
        let got = save_one(&mut e, &events, buffer, 0x54, |e, x| {
            e.mem.set_u32(x + 0x14, 0x88)
        });
        assert_eq!(got, vec![b(4, 0x88)]);
        let got = save_one(&mut e, &events, buffer, 0x75, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x42);
            e.mem.set_u8(x + 0x18, 4);
        });
        assert_eq!(got, vec![f2(0x42), f2(0x41), b(1, 4)]);
        // EXTRA_SCRIPT: the form id, then the locals save themselves.
        e.register(SCRIPT_LOCALS_SAVE_GAME, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            let got = save_one(e, &events, buffer, 0x0d, |e, x| {
                e.mem.set_u32(x + 0x0c, 0x41);
                e.mem.set_u32(x + 0x10, 0x9000);
            });
            assert_eq!(got, vec![f2(0x41)]);
        });
        assert_eq!(
            calls_to(&log, SCRIPT_LOCALS_SAVE_GAME),
            vec![vec![0x9000, buffer]]
        );
        // EXTRA_MODEL_SWAP
        e.register(GET_MODEL_SWAP_INDEX, |_, a| returns(a[0] * 0x100 + a[1]));
        let got = save_one(&mut e, &events, buffer, 0x5b, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x42);
        });
        assert_eq!(got, vec![f2(0x42), b(4, 0x4241)]);
    }

    #[test]
    fn save_game_ov2_lists() {
        let (mut e, events, buffer) = buffer_engine();
        e.mem.set_u32(buffer + 0x34, 1);
        let b = |size: u32, word: u32| ("bytes", vec![size, word]);
        let f2 = |form: u32| ("form2", vec![form]);
        // Run once packages (0x1B) and the list of the same shape (0x5E).
        for extra_type in [0x1b, 0x5e] {
            let first = e.mem.alloc(8);
            e.mem.set_u32(first, 0x11);
            e.mem.set_u8(first + 4, 0xa1);
            let tail = list_node(&mut e, 0, 0);
            let skipped = list_node(&mut e, 0, tail);
            let head = list_node(&mut e, first, skipped);
            let got = save_one(&mut e, &events, buffer, extra_type, |e, x| {
                e.mem.set_u32(x + 0x0c, head);
            });
            assert_eq!(got, vec![f2(0x11), b(1, 0xa1), ("end", vec![1, 0x5000])]);
        }
        // Followers: the items are forms.
        let tail = list_node(&mut e, 0x32, 0);
        let head = list_node(&mut e, 0x31, tail);
        let got = save_one(&mut e, &events, buffer, 0x1d, |e, x| {
            e.mem.set_u32(x + 0x0c, head)
        });
        assert_eq!(got, vec![f2(0x31), f2(0x32), ("end", vec![2, 0x5000])]);
        // The records of three words.
        let record = e.mem.alloc(16);
        e.mem.set_u32(record, 0x41);
        e.mem.set_u32(record + 4, 0x1111);
        e.mem.set_u32(record + 8, 0x2222);
        let head = list_node(&mut e, record, 0);
        let got = save_one(&mut e, &events, buffer, 0x73, |e, x| {
            e.mem.set_u32(x + 0x0c, head)
        });
        assert_eq!(
            got,
            vec![
                f2(0x41),
                b(4, 0x1111),
                b(4, 0x2222),
                ("end", vec![1, 0x5000])
            ]
        );
        // The list whose items give two words.
        e.register(GET_WORD_28, |_, a| returns(a[0] + 1));
        let item = e.mem.alloc(8);
        let head = list_node(&mut e, item, 0);
        let got = save_one(&mut e, &events, buffer, 0x35, |e, x| {
            e.mem.set_u32(x + 0x0c, head)
        });
        assert_eq!(got, vec![b(4, item + 1), b(4, 0), ("end", vec![1, 0x5000])]);
        // 0x8B: 12 bytes, a value, 4 bytes, then the records of the list at +0x20.
        let record = e.mem.alloc(0x30);
        e.mem.set_u32(record + 0x0c, 0x51);
        e.mem.set_u32(record + 0x1c, 0x52);
        e.mem.set_u8(record + 0x20, 7);
        let got = save_one(&mut e, &events, buffer, 0x8b, |e, x| {
            e.mem.set_u32(x + 0x10, 0x10);
            e.mem.set_u32(x + 0x1c, 0x53);
            e.mem.set_u32(x + 0x0c, 0x54);
            e.mem.set_u32(x + 0x20, record);
            e.mem.set_u32(x + 0x24, 0);
        });
        assert_eq!(
            got,
            vec![
                b(12, 0x10),
                ("form", vec![0x53]),
                b(4, 0x54),
                b(12, 0),
                ("form", vec![0x51]),
                b(12, 0),
                ("form", vec![0x52]),
                b(1, 7),
                ("end", vec![1, 0x5000]),
            ]
        );
    }

    #[test]
    fn save_game_ov2_arrays_and_objects() {
        let (mut e, events, buffer) = buffer_engine();
        e.mem.set_u32(buffer + 0x34, 1);
        let b = |size: u32, word: u32| ("bytes", vec![size, word]);
        // The count of an array is the word at +8 of it (the `GET_NEXT` double of
        // the engine reads it); an item is the address of a cell holding the
        // index.
        e.register(ARRAY_ITEM, |e, a| {
            let cell = e.mem.alloc(8);
            e.mem.set_u32(cell, a[1] + 0x100);
            returns(cell)
        });
        // The array at +0x0C (0x7C): the count and the ids of the items.
        let got = save_one(&mut e, &events, buffer, 0x7c, |e, x| {
            e.mem.set_u32(x + 0x14, 2)
        });
        assert_eq!(
            got,
            vec![
                ("count", vec![2]),
                ("form", vec![0x100]),
                ("form", vec![0x101])
            ]
        );
        // The friend hits (0x45): the old hits go first, then each hit saves
        // itself.
        stub(&mut e, FRIEND_HITS_REMOVE_OLD_HITS);
        e.register(COMBAT_TIME_STAMP_SAVE_GAME, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            let got = save_one(e, &events, buffer, 0x45, |e, x| e.mem.set_u32(x + 0x14, 1));
            assert_eq!(got, vec![("count", vec![1])]);
        });
        let hits = calls_to(&log, COMBAT_TIME_STAMP_SAVE_GAME);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0][1], buffer);
        assert_eq!(calls_to(&log, FRIEND_HITS_REMOVE_OLD_HITS).len(), 1);
        // 0x5F: fields, a form id, and an array of records of 4 bytes and an
        // inner array of ids.
        let entry = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u8(entry + i, 0x10 + i as u8);
        }
        e.mem.set_u32(entry + 0x0c, 1);
        e.register_double(ARRAY_ITEM, move |e, a| {
            let cell = e.mem.alloc(8);
            let value = if a[0] == entry + 4 {
                0x300 + a[1]
            } else {
                entry
            };
            e.mem.set_u32(cell, value);
            returns(cell)
        });
        let got = save_one(&mut e, &events, buffer, 0x5f, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x1234);
            e.mem.set_u32(x + 0x10, 0x10);
            e.mem.set_u32(x + 0x14, 0x61);
            e.mem.set_u32(x + 0x18, 0x18);
            e.mem.set_u8(x + 0x1c, 1);
            e.mem.set_u32(x + 0x28, 1);
        });
        assert_eq!(
            got,
            vec![
                b(2, 0x1234),
                b(4, 0x10),
                b(4, 0x18),
                b(1, 1),
                ("form2", vec![0x61]),
                ("count", vec![1]),
                b(1, 0x10),
                b(1, 0x11),
                b(1, 0x12),
                b(1, 0x13),
                ("count", vec![1]),
                ("form2", vec![0x300]),
            ]
        );
        // Objects with virtuals: 0x70 (answer, then save), 0x1a.
        let object = object_with_slots(
            &mut e,
            &[(8, 0x0200_5000), (0x0c, 0x0200_5004), (0x54, 0x0200_5008)],
        );
        e.register(0x0200_5000, |_, _| returns(0x1234_5602));
        e.register_double(0x0200_5004, |_, a| {
            assert_ne!(a[1], 0);
            Ret::default()
        });
        e.register_double(0x0200_5008, |_, a| {
            assert_ne!(a[1], 0);
            Ret::default()
        });
        let got = save_one(&mut e, &events, buffer, 0x70, |e, x| {
            e.mem.set_u32(x + 0x0c, object)
        });
        assert_eq!(got, vec![b(1, 2)]);
        let got = save_one(&mut e, &events, buffer, 0x70, |_, _| {});
        assert_eq!(got, vec![b(1, 0xff)]);
        let got = save_one(&mut e, &events, buffer, 0x1a, |e, x| {
            e.mem.set_u32(x + 0x0c, object)
        });
        assert_eq!(got, vec![("form2", vec![object])]);
        // 0x2C: the byte of the map marker data.
        let marker = e.mem.alloc(0x10);
        e.mem.set_u8(marker + 0x0c, 3);
        let got = save_one(&mut e, &events, buffer, 0x2c, |e, x| {
            e.mem.set_u32(x + 0x0c, marker)
        });
        assert_eq!(got, vec![b(1, 3)]);
        let got = save_one(&mut e, &events, buffer, 0x2c, |_, _| {});
        assert_eq!(got, vec![b(1, 0)]);
        // The handle (0x60): the word of its object, or -1.
        e.register(HANDLE_TARGET_WORD, |_, a| returns(a[0] + 5));
        let got = save_one(&mut e, &events, buffer, 0x60, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x9000)
        });
        assert_eq!(got, vec![b(4, 0x9005)]);
        let got = save_one(&mut e, &events, buffer, 0x60, |_, _| {});
        assert_eq!(got, vec![b(4, 0xffff_ffff)]);
        // The form id getter type (0x92): the getter reads +0x0C.
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        let got = save_one(&mut e, &events, buffer, 0x92, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x42);
        });
        assert_eq!(got, vec![b(4, 0x41), b(4, 0x42)]);
    }

    #[test]
    fn save_game_ov2_magic_topic_teleport_and_strings() {
        let (mut e, events, buffer) = buffer_engine();
        e.mem.set_u32(buffer + 0x34, 1);
        e.register(GET_MAGIC_ITEM_FORM_ID, |_, a| returns(a[0] + 1));
        e.register(GET_MAGIC_TARGET_FORM_ID, |_, a| returns(a[0] + 2));
        // 0x32: the caster's and target's ids (0 for none) and a form id.
        let got = save_one(&mut e, &events, buffer, 0x32, |e, x| {
            e.mem.set_u32(x + 0x18, 0x100);
            e.mem.set_u32(x + 0x1c, 0x200);
            e.mem.set_u32(x + 0x20, 0x41);
        });
        assert_eq!(
            got,
            vec![
                ("form", vec![0x101]),
                ("form", vec![0x202]),
                ("form2", vec![0x41])
            ]
        );
        let got = save_one(&mut e, &events, buffer, 0x32, |_, _| {});
        assert_eq!(
            got,
            vec![("form", vec![0]), ("form", vec![0]), ("form2", vec![0])]
        );
        // 0x3F: the id of the magic item inside the form.
        let got = save_one(&mut e, &events, buffer, 0x3f, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x300)
        });
        assert_eq!(got, vec![("form", vec![0x331])]);
        // 0x33: a form id, then the active effects.
        e.register(ACTIVE_EFFECT_SAVE_LIST, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            let got = save_one(e, &events, buffer, 0x33, |e, x| {
                e.mem.set_u32(x + 0x1c, 0x41)
            });
            assert_eq!(got, vec![("form2", vec![0x41])]);
        });
        assert_eq!(calls_to(&log, ACTIVE_EFFECT_SAVE_LIST).len(), 1);
        assert_eq!(calls_to(&log, ACTIVE_EFFECT_SAVE_LIST)[0][0], buffer);
        // 0x4D and 0x2B hand the buffer to the data they hold.
        stub(&mut e, MENU_TOPIC_SAVE_GAME_OV2);
        stub(&mut e, DOOR_TELEPORT_DATA_SAVE_GAME_OV2);
        let log = logged(&mut e, |e| {
            save_one(e, &events, buffer, 0x4d, |e, x| {
                e.mem.set_u32(x + 0x0c, 0x6000)
            });
            save_one(e, &events, buffer, 0x2b, |e, x| {
                e.mem.set_u32(x + 0x0c, 0x7000)
            });
        });
        assert_eq!(
            calls_to(&log, MENU_TOPIC_SAVE_GAME_OV2),
            vec![vec![0x6000, buffer]]
        );
        assert_eq!(
            calls_to(&log, DOOR_TELEPORT_DATA_SAVE_GAME_OV2),
            vec![vec![0x7000, buffer]]
        );
        // 0x2E: two form ids, then the object `004181e0` gives is saved with
        // the buffer's kind word replaced.
        e.register(0x0041_81e0, |e, _| returns(e.mem.u32(0x011d_e720)));
        let object = object_with_slot(&mut e, 0x54, 0x0200_5100);
        e.mem.set_u32(0x011d_e720, object);
        e.register(GET_WORD_28, |_, _| returns(0x9999));
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(0x0200_5100, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[1] + 0x17));
            Ret::default()
        });
        let got = save_one(&mut e, &events, buffer, 0x2e, |e, x| {
            e.mem.set_u32(x + 0x0c, 0x41);
            e.mem.set_u32(x + 0x10, 0x42);
        });
        assert_eq!(
            got,
            vec![
                ("form2", vec![0x41]),
                ("form2", vec![0x42]),
                ("bytes", vec![4, 0x9999])
            ]
        );
        assert_eq!(*seen.borrow(), vec![0x9999]);
        // ... and put back afterwards.
        assert_eq!(e.mem.u32(buffer + 0x17), 2);
        // 0x8F: two strings, each copied to a local, saved and destroyed.
        e.register(STRING_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], 0xc000 + (a[1] & 0xff));
            Ret::default()
        });
        stub(&mut e, STRING_DESTROY);
        let log = logged(&mut e, |e| {
            let got = save_one(e, &events, buffer, 0x8f, |_, _| {});
            assert_eq!(got.len(), 2);
            assert_eq!(got[0].0, "string");
            assert_eq!(got[1].0, "string");
            assert_eq!(got[1].1[0] - got[0].1[0], 8);
        });
        assert_eq!(calls_to(&log, STRING_DESTROY).len(), 2);
        // The string sources are at +0x0C and +0x14.
        let sources: Vec<u32> = calls_to(&log, STRING_ASSIGN)
            .iter()
            .map(|a| a[1] & 0xff)
            .collect();
        assert_eq!(sources[1] - sources[0], 8);
    }

    #[test]
    fn buffer_helpers_of_the_save_buffer_state() {
        let mut e = engine();
        let buffer: Ptr = Ptr::new(e.mem.alloc(0x40));
        let out: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(buffer.addr() + 0x17, 0x1234_5678);
        assert_eq!(e.call(0x0042_8110, &args![buffer, out]).u32(), out.addr());
        assert_eq!(e.mem.u32(out.addr()), 0x1234_5678);
        e.call(0x0042_8130, &args![buffer, 0x77u32]);
        assert_eq!(e.mem.u32(buffer.addr() + 0x17), 0x77);
        e.mem.set_u32(out.addr(), 0b1010);
        assert!(e.call(0x0042_80f0, &args![out, 0b0010u32]).bool());
        assert!(!e.call(0x0042_80f0, &args![out, 0b0101u32]).bool());
    }

    #[test]
    fn string_copy_helpers_assign_the_members() {
        let mut e = engine();
        e.register(STRING_ASSIGN, |_, a| returns(a[1]));
        let log = logged(&mut e, |e| {
            let object: Ptr = Ptr::new(e.mem.alloc(0x40));
            let out = 0x5000u32;
            assert_eq!(e.call(0x0042_8070, &args![object, out]).u32(), out);
            assert_eq!(e.call(0x0042_80b0, &args![object, out]).u32(), out);
            let base = object.addr();
            let calls = e.call_log.clone().unwrap();
            let assigns = calls_to(&calls, STRING_ASSIGN);
            assert_eq!(
                assigns,
                vec![vec![out, base + 0x0c], vec![out, base + 0x14]]
            );
        });
        let _ = log;
    }

    // ---- the constructors and destructors at the end of the range ----

    #[test]
    fn simple_extra_data_constructors() {
        let mut e = engine();
        for vtable in [
            VTABLE_EXTRA_LOCK,
            VTABLE_EXTRA_TELEPORT,
            VTABLE_EXTRA_OWNERSHIP,
            VTABLE_EXTRA_GLOBAL,
            VTABLE_EXTRA_RANK,
            VTABLE_EXTRA_COUNT,
            VTABLE_EXTRA_HEALTH,
            VTABLE_EXTRA_USES,
        ] {
            e.map(vtable, 16);
        }
        // (constructor, type, vtable, size of the word at +0x0C)
        let cases = [
            (0x0042_c580u32, EXTRA_TELEPORT, VTABLE_EXTRA_TELEPORT, 4),
            (0x0042_c5e0, EXTRA_OWNERSHIP, VTABLE_EXTRA_OWNERSHIP, 4),
            (0x0042_c610, EXTRA_GLOBAL, VTABLE_EXTRA_GLOBAL, 4),
            (0x0042_c640, EXTRA_RANK, VTABLE_EXTRA_RANK, 4),
            (0x0042_c670, EXTRA_COUNT, VTABLE_EXTRA_COUNT, 2),
            (0x0042_c6a0, EXTRA_HEALTH, VTABLE_EXTRA_HEALTH, 4),
            (0x0042_c6d0, EXTRA_USES, VTABLE_EXTRA_USES, 1),
        ];
        for (address, extra_type, vtable, size) in cases {
            let object = e.mem.alloc(0x10);
            e.mem.set_u32(object + 0x0c, 0xffff_ffff);
            assert_eq!(e.call(address, &args![object]).u32(), object);
            assert_eq!(e.mem.u32(object), vtable);
            assert_eq!(e.mem.u8(object + 4), extra_type);
            assert_eq!(e.mem.u32(object + 8), 0);
            let word = e.mem.u32(object + 0x0c);
            let mask = if size == 4 {
                u32::MAX
            } else {
                (1u32 << (8 * size)) - 1
            };
            assert_eq!(word & mask, 0, "{address:#x}");
            if size < 4 {
                // The bytes above the member are left alone.
                assert_eq!(word >> (8 * size), u32::MAX >> (8 * size));
            }
        }
    }

    #[test]
    fn extra_lock_constructor_builds_the_lock_record() {
        let mut e = engine();
        e.map(VTABLE_EXTRA_LOCK, 16);
        e.register(LOCK_RECORD_INIT, |_, a| returns(a[0]));
        let object = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0042_c4b0, &args![object]);
        });
        assert_eq!(e.mem.u32(object), VTABLE_EXTRA_LOCK);
        assert_eq!(e.mem.u8(object + 4), EXTRA_LOCK);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let record = e.mem.u32(object + 0x0c);
        assert_ne!(record, 0);
        assert_eq!(calls_to(&log, LOCK_RECORD_INIT), vec![vec![record]]);
    }

    #[test]
    fn flag_structure_constructor() {
        let mut e = engine();
        stub(&mut e, FLAGS_MEMBER_INIT);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, 0x0101_0101);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_c470, &args![object]).u32(), object);
        });
        assert_eq!(e.mem.u32(object), 0);
        assert_eq!(calls_to(&log, FLAGS_MEMBER_INIT), vec![vec![object + 4]]);
    }

    #[test]
    fn lock_and_teleport_destructors_free_on_request() {
        for (address, destroy) in [
            (0x0042_c550u32, EXTRA_LOCK_DESTROY),
            (0x0042_c5b0, EXTRA_TELEPORT_DESTROY),
        ] {
            let mut e = engine();
            stub(&mut e, destroy);
            let object = e.mem.alloc(0x10);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(address, &args![object, 0u32]).u32(), object);
            });
            assert_eq!(calls_to(&log, destroy), vec![vec![object]]);
            assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
            let log = logged(&mut e, |e| {
                e.call(address, &args![object, 1u32]);
            });
            assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![object]]);
        }
    }

    // ---- the buffered loader (00428150) ----

    /// The next `size` bytes of the read stream, as the loader's buffer
    /// double gives them.
    fn stream_pop(e: &mut Engine, size: u32) -> u32 {
        let cursor = e.mem.u32(READ_CURSOR);
        let mut word = 0u32;
        for offset in 0..size.min(4) {
            word |= (e.mem.u8(cursor + offset) as u32) << (8 * offset);
        }
        e.mem.set_u32(READ_CURSOR, cursor + size);
        word
    }

    /// An engine for the buffered loader: the read stream double (bytes,
    /// form ids, counts all come from the same queue), a buffer whose version
    /// (virtual 0) is 0x20, reference (virtual 8) and owner (virtual 0xC) are
    /// the words at +0x30 and +0x34; the save kind of the buffer (+0x17) and
    /// of every type in the table is 2. The constructors of other units are
    /// doubles that set the type and a null next.
    fn load_ov2_engine() -> (Engine, u32) {
        let mut e = read_engine();
        e.map(0x011d_d000, 0x1000);
        e.map(SAVE_KIND_TABLE, 0x300);
        e.register(LOAD_BYTES, |e, a| {
            for offset in 0..a[2] {
                let byte = stream_pop(e, 1) as u8;
                e.mem.set_u8(a[1] + offset, byte);
            }
            Ret::default()
        });
        e.register(LOAD_FORM_ID, |e, _| returns(stream_pop(e, 4)));
        e.register(LOAD_FORM_ID_OV2, |e, a| {
            let id = stream_pop(e, 4);
            e.mem.set_u32(a[1], id);
            Ret::default()
        });
        e.register(LOAD_VARIABLE_SIZED_VALUE, |e, _| returns(stream_pop(e, 4)));
        e.register(0x0200_6000, |_, _| returns(0x20));
        e.register(0x0200_6008, |e, a| returns(e.mem.u32(a[0] + 0x30)));
        e.register(0x0200_600c, |e, a| returns(e.mem.u32(a[0] + 0x34)));
        let buffer = object_with_slots(
            &mut e,
            &[(0, 0x0200_6000), (8, 0x0200_6008), (0x0c, 0x0200_600c)],
        );
        // The vtables this file's constructors set: slot 0 the destructor.
        for vtable in [
            VTABLE_EXTRA_LOCK,
            VTABLE_EXTRA_TELEPORT,
            VTABLE_EXTRA_OWNERSHIP,
            VTABLE_EXTRA_GLOBAL,
            VTABLE_EXTRA_RANK,
            VTABLE_EXTRA_COUNT,
            VTABLE_EXTRA_HEALTH,
            VTABLE_EXTRA_USES,
        ] {
            e.put_vtable(vtable, &[DESTRUCTOR]);
        }
        e.mem.set_u32(buffer + 0x17, 2);
        for &(extra_type, _, construct) in OV2_CONSTRUCTIONS {
            e.mem.set_u32(SAVE_KIND_TABLE + extra_type as u32 * 4, 2);
            if !(0x0042_c470..=0x0042_c6d0).contains(&construct) {
                e.register_double(construct, move |e, a| {
                    e.mem.set_u32(a[0], VTABLE);
                    e.mem.set_u8(a[0] + 4, extra_type);
                    e.mem.set_u32(a[0] + 8, 0);
                    returns(a[0])
                });
            }
        }
        for extra_type in [0x16u32, 0x3e, 0x1f, 0x90, 0x91, 0x2c, 0x54, 0x5f] {
            e.mem.set_u32(SAVE_KIND_TABLE + extra_type * 4, 2);
        }
        (e, buffer)
    }

    /// The bytes of a stream: little-endian pieces.
    struct Bytes(Vec<u8>);

    impl Bytes {
        fn new() -> Bytes {
            Bytes(vec![])
        }
        fn b(mut self, value: u8) -> Bytes {
            self.0.push(value);
            self
        }
        fn w(mut self, value: u32) -> Bytes {
            self.0.extend(value.to_le_bytes());
            self
        }
        fn h(mut self, value: u16) -> Bytes {
            self.0.extend(value.to_le_bytes());
            self
        }
    }

    /// Loads `stream` (the count of extra data first, as a word) into `list`.
    fn load_ov2(e: &mut Engine, buffer: u32, list: Ptr<ExtraDataList>, stream: Bytes) {
        for (i, byte) in stream.0.iter().enumerate() {
            e.mem.set_u8(READ_INPUT + i as u32, *byte);
        }
        e.mem.set_u32(READ_CURSOR, READ_INPUT);
        e.call(0x0042_8150, &args![list, buffer]);
        assert_eq!(
            e.mem.u32(READ_CURSOR),
            READ_INPUT + stream.0.len() as u32,
            "the whole stream is read"
        );
    }

    #[test]
    fn load_game_ov2_word_arms_build_the_extra_data_with_their_constructors() {
        let (mut e, buffer) = load_ov2_engine();
        let list = new_list(&mut e);
        // (type, size, payload bytes)
        let cases: &[(u8, u32)] = &[
            (0x24, 2),
            (0x25, 4),
            (0x30, 4),
            (0x23, 4),
            (0x8d, 1),
            (0x4a, 1),
        ];
        for &(extra_type, size) in cases {
            let payload = Bytes::new().w(1).b(extra_type);
            let payload = (0..size).fold(payload, |p, i| p.b(0xa0 + i as u8));
            let log = logged(&mut e, |e| {
                load_ov2(e, buffer, list, payload);
            });
            assert_eq!(
                calls_to(&log, OPERATOR_NEW),
                vec![vec![0x10]],
                "{extra_type:#x}"
            );
            let extra = find_extra(&mut e, list, extra_type);
            let word = e.mem.u32(extra.addr() + 0x0c);
            let mask = if size == 4 {
                u32::MAX
            } else {
                (1u32 << (8 * size)) - 1
            };
            let expected = (0..size).fold(0u32, |w, i| w | (0xa0 + i) << (8 * i));
            assert_eq!(word & mask, expected, "{extra_type:#x}");
        }
        // Each load removes the marked types it did not load, so only the last
        // type (0x4A) is left; loading it again updates it (it is marked) and
        // builds nothing.
        assert_eq!(chain_types(&e, list), vec![0x4a]);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x4a).b(0x5c));
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_of(&mut e, list, 0x4a) & 0xff, 0x5c);
    }

    #[test]
    fn load_game_ov2_removes_the_marked_types_the_buffer_did_not_load() {
        let (mut e, buffer) = load_ov2_engine();
        let list = new_list(&mut e);
        add(&mut e, list, 0x24, 5);
        add(&mut e, list, 0x25, 6);
        // The type 0x2E is marked and kept; type 0x03 has no mask, it stays.
        add(&mut e, list, 0x2e, 7);
        add(&mut e, list, 0x03, 8);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x24).h(9));
        });
        assert_eq!(word_of(&mut e, list, 0x24) & 0xffff, 9);
        let mut types = chain_types(&e, list);
        types.sort();
        assert_eq!(types, vec![0x03, 0x24, 0x2e]);
        assert_eq!(calls_to(&log, DESTRUCTOR).len(), 1);
        // The special render flags of type 0x92 are set from its form id and
        // the extra data stays.
        e.mem.set_u32(SAVE_KIND_TABLE + 0x92 * 4, 2);
        let special = add(&mut e, list, 0x92, 0x0f);
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        stub(&mut e, SET_SPECIAL_RENDER_WORD);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(0));
        });
        assert!(chain_types(&e, list).contains(&0x92));
        assert_eq!(
            calls_to(&log, SET_SPECIAL_RENDER_WORD),
            vec![vec![special.addr(), 0x0f & 7]]
        );
    }

    #[test]
    fn load_game_ov2_forms_lists_and_objects() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        let list = new_list(&mut e);
        // 0x21: the form as it is; 0x22: cast; 0x1c: LoadFormID_ov2 at +0x0C.
        load_ov2(
            &mut e,
            buffer,
            list,
            Bytes::new()
                .w(3)
                .b(0x21)
                .w(0x51)
                .b(0x22)
                .w(0x52)
                .b(0x1c)
                .w(0x53),
        );
        assert_eq!(word_of(&mut e, list, 0x21), 0x51);
        assert_eq!(word_of(&mut e, list, 0x22), 0x52);
        assert_eq!(word_of(&mut e, list, 0x1c), 0x53);
        // 0x19 (package) and 0x18 (package start location).
        let stream = Bytes::new()
            .w(2)
            .b(0x19)
            .w(0x61)
            .w(0x62)
            .w(0x0d0c_0b0a)
            .b(1)
            .b(2)
            .b(3)
            .b(0x18)
            .w(0x63)
            .w(1)
            .w(2)
            .w(3)
            .w(4);
        load_ov2(&mut e, buffer, list, stream);
        let package = find_extra(&mut e, list, 0x19).addr();
        assert_eq!(e.mem.u32(package + 0x0c), 0x61);
        assert_eq!(e.mem.u32(package + 0x14), 0x62);
        assert_eq!(e.mem.u32(package + 0x10), 0x0d0c_0b0a);
        assert_eq!(
            [
                e.mem.u8(package + 0x18),
                e.mem.u8(package + 0x19),
                e.mem.u8(package + 0x1a)
            ],
            [1, 2, 3]
        );
        let location = find_extra(&mut e, list, 0x18).addr();
        assert_eq!(e.mem.u32(location + 0x0c), 0x63);
        assert_eq!(e.mem.u32(location + 0x10), 1);
        assert_eq!(e.mem.u32(location + 0x1c), 4);
        // Lists: run once packages and followers go to the head of their list.
        let added: Added = Rc::new(RefCell::new(vec![]));
        let record = added.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let stream = Bytes::new()
            .w(2)
            .b(0x1d)
            .w(2)
            .w(0x71)
            .w(0x72)
            .b(0x1b)
            .w(1)
            .w(0x73)
            .b(0xb7);
        load_ov2(&mut e, buffer, list, stream);
        let followers = word_of(&mut e, list, 0x1d);
        let run_once = word_of(&mut e, list, 0x1b);
        let added = added.borrow().clone();
        assert_eq!(added[0], (followers, 0x71));
        assert_eq!(added[1], (followers, 0x72));
        assert_eq!(added[2].0, run_once);
        let item = added[2].1;
        assert_eq!(e.mem.u32(item), 0x73);
        assert_eq!(e.mem.u8(item + 4), 0xb7);
    }

    #[test]
    fn load_game_ov2_worn_ghost_and_unknown_types() {
        let (mut e, buffer) = load_ov2_engine();
        stub(&mut e, SET_WORN);
        stub(&mut e, SET_CAN_NOT_WEAR);
        stub(&mut e, SET_GHOST);
        e.register(SAVE_GAME_WARNING, |_, _| Ret::default());
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            load_ov2(
                e,
                buffer,
                list,
                Bytes::new()
                    .w(6)
                    .b(0x16)
                    .b(0x3e)
                    .b(0x1f)
                    .b(0x90)
                    .b(0x91)
                    .b(0x40),
            );
        });
        assert_eq!(calls_to(&log, SET_WORN)[0][1..], [1, 0]);
        assert_eq!(calls_to(&log, SET_CAN_NOT_WEAR)[0][1], 1);
        assert_eq!(calls_to(&log, SET_GHOST)[0][1], 1);
        assert_eq!(
            calls_to(&log, SAVE_GAME_WARNING),
            vec![vec![LOAD_GAME_OV2_UNKNOWN_TYPE_FORMAT, 0x40]]
        );
    }

    #[test]
    fn load_game_ov2_lock_teleport_and_markers() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        let list = new_list(&mut e);
        // EXTRA_LOCK (the constructor of this file builds the record).
        e.register(LOCK_RECORD_INIT, |_, a| returns(a[0]));
        let stream = Bytes::new().w(1).b(0x2a).b(3).b(1).w(0x77).w(0x44).w(0x55);
        load_ov2(&mut e, buffer, list, stream);
        let lock_data = word_of(&mut e, list, 0x2a);
        assert_eq!(e.mem.u8(lock_data), 3);
        assert_eq!(e.mem.u8(lock_data + 8), 1);
        assert_eq!(e.mem.u32(lock_data + 4), 0x77);
        assert_eq!(e.mem.u32(lock_data + 0x0c), 0x44);
        assert_eq!(e.mem.u32(lock_data + 0x10), 0x55);
        // EXTRA_TELEPORT: the data is built when missing, then loaded.
        e.register(DOOR_TELEPORT_DATA_INIT, |_, a| returns(a[0]));
        stub(&mut e, DOOR_TELEPORT_DATA_LOAD_GAME_OV2);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x2b));
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW).last().unwrap(), &vec![0x20]);
        let data = word_of(&mut e, list, 0x2b);
        assert_eq!(
            calls_to(&log, DOOR_TELEPORT_DATA_LOAD_GAME_OV2),
            vec![vec![data, buffer]]
        );
        // EXTRA_MAPMARKER: the byte goes to the marker data the list holds.
        e.mem.set_u32(SAVE_KIND_TABLE + 0x2c * 4, 2);
        stub(&mut e, MAP_MARKER_SET_FLAGS);
        let marker = add(&mut e, list, 0x2c, 0x8000);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x2c).b(5));
        });
        assert_eq!(calls_to(&log, MAP_MARKER_SET_FLAGS), vec![vec![0x8000, 5]]);
        let _ = marker;
        // Without the extra data the byte is read and dropped.
        let other = new_list(&mut e);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, other, Bytes::new().w(1).b(0x2c).b(5));
        });
        assert!(calls_to(&log, MAP_MARKER_SET_FLAGS).is_empty());
        // 0x54 reads into the existing extra data at +0x14 or drops the word.
        let holder = add(&mut e, list, 0x54, 0);
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x54).w(0x1234));
        assert_eq!(e.mem.u32(holder.addr() + 0x14), 0x1234);
        load_ov2(&mut e, buffer, other, Bytes::new().w(1).b(0x54).w(0x1234));
    }

    #[test]
    fn load_game_ov2_friend_hits_and_the_pruned_list() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        let list = new_list(&mut e);
        // 0x45: the array is sized, each hit loads itself; an empty array
        // removes the extra data (`fn_00422670`).
        e.register_double(ARRAY_RESIZE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ARRAY_ITEM, |_, a| returns(0x7000 + a[1]));
        let hits: Added = Rc::new(RefCell::new(vec![]));
        let record = hits.clone();
        e.register_double(COMBAT_TIME_STAMP_LOAD_GAME, move |_, a| {
            record.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x45).w(2));
        });
        assert_eq!(*hits.borrow(), vec![(0x7000, buffer), (0x7001, buffer)]);
        assert_eq!(calls_to(&log, ARRAY_RESIZE)[0][1..], [2, 1]);
        // The count of the array is the word at +8; 0 removes the extra data.
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x45).w(0));
        assert!(!chain_types(&e, list).contains(&0x45));
        // 0x5e: the items without a form are deleted and dropped from the list
        // (the head is removed by copying the next node over it).
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let item = e.mem.u32(next);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        stub(&mut e, LIST_REMOVE_AFTER);
        let list = new_list(&mut e);
        let bad = e.mem.alloc(8);
        let good = e.mem.alloc(8);
        e.mem.set_u32(good, 5);
        let tail = list_node(&mut e, good, 0);
        let head = list_node(&mut e, bad, tail);
        e.mem.set_u32(SAVE_KIND_TABLE + 0x5e * 4, 2);
        add(&mut e, list, 0x5e, head);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x5e).w(0));
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![bad]]);
        assert_eq!(e.mem.u32(head), good);
        assert_eq!(e.mem.u32(head + 4), 0);
    }

    #[test]
    fn load_game_ov2_script_locals() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        e.register(SCRIPT_CREATE_LOCALS, |_, _| returns(0x9000));
        e.register(MENU_PACKING_VALUE, |_, _| returns(5));
        stub(&mut e, MENU_PACKER_RECOMPUTE);
        stub(&mut e, SCRIPT_LOCALS_LOAD_GAME);
        stub(&mut e, SCRIPT_LOCALS_INIT);
        stub(&mut e, SCRIPT_LOCALS_DESTROY);
        let list = new_list(&mut e);
        // Without a reference the packer is told around the load; the locals
        // the script builds are used.
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x0d).w(0x41));
        });
        assert_eq!(word_of(&mut e, list, 0x0d), 0x41);
        assert_eq!(
            calls_to(&log, MENU_PACKER_RECOMPUTE),
            vec![vec![buffer, 5], vec![buffer, 0]]
        );
        assert_eq!(
            calls_to(&log, SCRIPT_LOCALS_LOAD_GAME),
            vec![vec![0x9000, buffer]]
        );
        // With a reference, and a script that gives no locals: a temporary.
        e.mem.set_u32(buffer + 0x30, 0x7777);
        e.register(SCRIPT_CREATE_LOCALS, |_, _| returns(0));
        let other = new_list(&mut e);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, other, Bytes::new().w(1).b(0x0d).w(0x41));
        });
        assert!(calls_to(&log, MENU_PACKER_RECOMPUTE).is_empty());
        let temporary = calls_to(&log, SCRIPT_LOCALS_INIT)[0][0];
        assert_eq!(
            calls_to(&log, SCRIPT_LOCALS_LOAD_GAME),
            vec![vec![temporary, buffer]]
        );
        assert_eq!(calls_to(&log, SCRIPT_LOCALS_DESTROY), vec![vec![temporary]]);
    }

    #[test]
    fn load_game_ov2_model_swap_and_self_damage() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(MODEL_SWAP_FROM_INDEX_004759A0, |_, a| returns(a[1] + 1));
        e.register_double(0x0200_6100, |_, a| {
            assert_eq!(a[1..], [0, 1]);
            Ret::default()
        });
        let reference = object_with_slots(&mut e, &[(0x1cc, 0x0200_6100)]);
        e.mem.set_u32(buffer + 0x30, reference);
        let list = new_list(&mut e);
        // A form and an index: the swap is kept and the reference is told.
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x5b).w(0x41).w(9));
        });
        assert_eq!(word_of(&mut e, list, 0x5b), 10);
        assert!(!calls_to(&log, MODEL_SWAP_FROM_INDEX_004759A0).is_empty());
        // No form: the extra data is removed.
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x5b).w(0).w(9));
        assert!(!chain_types(&e, list).contains(&0x5b));
        // The type 0x56 float goes to the destructible lookup.
        e.register(DESTRUCTIBLE_LOOKUP_00477BC0, |_, a| returns(a[1] & 0xff));
        stub(&mut e, SET_SELF_DAMAGE);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x56).w(0x4000_0001));
        });
        assert_eq!(
            calls_to(&log, DESTRUCTIBLE_LOOKUP_00477BC0),
            vec![vec![reference, 0x4000_0001]]
        );
        assert_eq!(calls_to(&log, SET_SELF_DAMAGE), vec![vec![reference, 1]]);
    }

    #[test]
    fn load_game_ov2_pairs_and_objects() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        // 0x35: the constructor gives the extra data an empty list; a pair
        // that names nothing is reported, and the empty list removes it.
        let head = e.mem.alloc(8);
        e.register_double(0x0043_29d0, move |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, 0x35);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 0x0c, head);
            returns(a[0])
        });
        e.register(PAIR_LOOKUP_009724E0, |_, _| returns(0));
        e.register(SAVE_GAME_WARNING, |_, _| Ret::default());
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            load_ov2(
                e,
                buffer,
                list,
                Bytes::new().w(1).b(0x35).w(1).w(0x11).w(0x22),
            );
        });
        assert_eq!(
            calls_to(&log, SAVE_GAME_WARNING),
            vec![vec![
                MESSAGE_PAIR_NOT_FOUND,
                0x11,
                0x22,
                NAME_UNKNOWN_REFERENCE,
                0
            ]]
        );
        assert_eq!(
            calls_to(&log, PAIR_LOOKUP_009724E0),
            vec![vec![PAIR_LOOKUP_OBJECT, 0x22, 0x11]]
        );
        assert!(!chain_types(&e, list).contains(&0x35));
        // 0x70: a signed kind; -1 builds nothing.
        let loaded: Added = Rc::new(RefCell::new(vec![]));
        let seen = loaded.clone();
        e.register_double(0x0200_6200, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let object = object_with_slots(&mut e, &[(0x10, 0x0200_6200)]);
        e.register_double(BUILD_OBJECT_OF_KIND, move |_, a| {
            assert_eq!(a[0], 3);
            returns(object)
        });
        let list = new_list(&mut e);
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x70).b(3));
        assert_eq!(word_of(&mut e, list, 0x70), object);
        assert_eq!(*loaded.borrow(), vec![(object, buffer)]);
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x70).b(0xff));
        // 0x1A: a form, a package built by the factory and registered.
        let package = object_with_slots(&mut e, &[(0x5c, 0x0200_6200)]);
        e.register_double(PACKAGE_FACTORY, move |_, a| {
            assert_eq!(a[0], 0x17);
            returns(package)
        });
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        e.mem.set_u32(package + 0x0c, 0x77);
        e.set_global::<u32>(HANDLER_OBJECT, 0x4242);
        stub(&mut e, OBJECT_MANAGER_REGISTER);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x1a).w(0x55));
        });
        assert_eq!(
            calls_to(&log, OBJECT_MANAGER_REGISTER),
            vec![vec![0x4242, 0x55, 0x77]]
        );
        assert_eq!(word_of(&mut e, list, 0x1a), package);
        // 0x60: the id turned into a handle; an empty handle removes it.
        e.register(HANDLE_OF_ID, |_, a| returns(a[0]));
        e.register_double(ASSIGN_HANDLE_AT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x60).w(0x33));
        assert_eq!(word_of(&mut e, list, 0x60), 0x33);
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x60).w(0));
        assert!(!chain_types(&e, list).contains(&0x60));
    }

    #[test]
    fn load_game_ov2_strings_render_flags_and_records() {
        let (mut e, buffer) = load_ov2_engine();
        // 0x8F: two strings loaded into the same local and given in turn.
        stub(&mut e, LOAD_STRING);
        let given: Added = Rc::new(RefCell::new(vec![]));
        let first = given.clone();
        e.register_double(EXTRA_SET_FIRST_STRING, move |_, a| {
            first.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let second = given.clone();
        e.register_double(EXTRA_SET_SECOND_STRING, move |_, a| {
            second.borrow_mut().push((a[0] | 1, a[1]));
            Ret::default()
        });
        let list = new_list(&mut e);
        load_ov2(&mut e, buffer, list, Bytes::new().w(1).b(0x8f));
        let extra = find_extra(&mut e, list, 0x8f).addr();
        let seen = given.borrow().clone();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0, extra);
        assert_eq!(seen[1].0, extra | 1);
        assert_eq!(seen[0].1, seen[1].1);
        // 0x92: the flags and a word; the flags go through the setter.
        stub(&mut e, SET_SPECIAL_RENDER_WORD);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, Bytes::new().w(1).b(0x92).w(5).w(0x66));
        });
        let extra = find_extra(&mut e, list, 0x92).addr();
        assert_eq!(e.mem.u32(extra + 0x10), 0x66);
        assert!(calls_to(&log, SET_SPECIAL_RENDER_WORD).contains(&vec![extra, 5]));
        // 0x8B: records with forms (version 0x20 loads them as form ids).
        let head = e.mem.alloc(8);
        e.register(0x0043_7c40, move |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, 0x8b);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        e.register(ITEM_8B_INIT, |_, a| returns(a[0]));
        let added: Added = Rc::new(RefCell::new(vec![]));
        let record = added.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let _ = head;
        let mut stream = Bytes::new().w(1).b(0x8b);
        for i in 0..12 {
            stream = stream.b(0x10 + i);
        }
        stream = stream.w(0x31).w(0x32).w(1);
        for i in 0..12 {
            stream = stream.b(0x40 + i);
        }
        stream = stream.w(0x33);
        for i in 0..12 {
            stream = stream.b(0x50 + i);
        }
        stream = stream.w(0x34).b(7);
        let other = new_list(&mut e);
        load_ov2(&mut e, buffer, other, stream);
        let extra = find_extra(&mut e, other, 0x8b).addr();
        assert_eq!(e.mem.u32(extra + 0x1c), 0x31);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x32);
        let (target, item) = added.borrow()[0];
        assert_eq!(target, extra + 0x20);
        assert_eq!(e.mem.u32(item + 0x0c), 0x33);
        assert_eq!(e.mem.u32(item + 0x1c), 0x34);
        assert_eq!(e.mem.u8(item + 0x20), 7);
    }

    #[test]
    fn load_game_ov2_dismembered_limbs_replace_the_old_ones() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| {
            returns(if a[0] == 0x99 { 0 } else { a[0] })
        });
        e.register_double(0x0043_0200, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, 0x5f);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        // Arrays: the count is the word at +8, an item is a stable cell.
        let cells: Rc<RefCell<std::collections::HashMap<(u32, u32), u32>>> =
            Rc::new(RefCell::new(Default::default()));
        let store = cells.clone();
        e.register_double(ARRAY_ITEM, move |e, a| {
            let mut cells = store.borrow_mut();
            let cell = *cells.entry((a[0], a[1])).or_insert_with(|| e.mem.alloc(8));
            returns(cell)
        });
        e.register_double(ARRAY_RESIZE_OTHER, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register_double(ARRAY_REMOVE_AT, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            Ret::default()
        });
        e.register(BUFFER_FIELD_COPY_42CE30, |_, a| returns(a[1]));
        stub(&mut e, FLAGS_MEMBER_INIT);
        stub(&mut e, OWNER_NOTIFY_END);
        stub(&mut e, OWNER_NOTIFY_BEGIN);
        e.register(DISMEMBERED_LIMBS_COMPARE, |_, _| returns(0));
        // Fields, a form, one record: 4 flag bytes (the buffer is new
        // enough), two ids of which the second names nothing.
        let mut stream = Bytes::new()
            .w(1)
            .b(0x5f)
            .h(0x1234)
            .w(0x10)
            .w(0x18)
            .b(1)
            .w(0x41)
            .w(1);
        stream = stream.b(1).b(2).b(3).b(4).w(2).w(0x51).w(0x99);
        let list = new_list(&mut e);
        e.mem.set_u32(buffer + 0x34, 0x6600);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, stream);
        });
        let extra = find_extra(&mut e, list, 0x5f).addr();
        assert_eq!(e.mem.u16(extra + 0x0c), 0x1234);
        assert_eq!(e.mem.u32(extra + 0x14), 0x41);
        let slot = cells.borrow()[&(extra + 0x20, 0)];
        let entry = e.mem.u32(slot);
        assert_eq!(
            [
                e.mem.u8(entry),
                e.mem.u8(entry + 1),
                e.mem.u8(entry + 2),
                e.mem.u8(entry + 3)
            ],
            [1, 2, 3, 4]
        );
        // The id that named nothing was dropped: one id is left.
        assert_eq!(e.mem.u32(entry + 4 + 8), 1);
        assert_eq!(calls_to(&log, ARRAY_REMOVE_AT).len(), 1);
        // Nothing to compare with: the owner is told at the end.
        assert_eq!(calls_to(&log, OWNER_NOTIFY_END), vec![vec![0x6600]]);
        // A second load detaches the old one (it is marked), compares and
        // deletes it.
        e.mem.set_u32(SAVE_KIND_TABLE + 0x5f * 4, 2);
        let stream = Bytes::new().w(1).b(0x5f).h(1).w(0).w(0).b(0).w(0).w(0);
        let log = logged(&mut e, |e| {
            load_ov2(e, buffer, list, stream);
        });
        assert_eq!(calls_to(&log, DISMEMBERED_LIMBS_COMPARE).len(), 1);
        assert_eq!(calls_to(&log, DESTRUCTOR).len(), 1);
        assert_eq!(calls_to(&log, OWNER_NOTIFY_BEGIN), vec![vec![0x6600]]);
    }

    #[test]
    fn load_game_ov2_leveled_creature_makes_the_creature_and_loads_it() {
        let (mut e, buffer) = load_ov2_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[0]));
        e.register_double(0x0200_6300, |_, _| Ret::default());
        e.register_double(0x0200_6304, |_, a| {
            assert_ne!(a[1], 0);
            Ret::default()
        });
        let owner = object_with_slots(
            &mut e,
            &[
                (0x1cc, 0x0200_6300),
                (0x1d0, 0x0200_6308),
                (0x218, 0x0200_630c),
            ],
        );
        e.register(0x0200_6308, |_, _| returns(0));
        e.register(0x0200_630c, |_, _| returns(0));
        e.mem.set_u32(buffer + 0x34, owner);
        let creature = object_with_slots(&mut e, &[(0x5c, 0x0200_6304)]);
        e.mem.set_u32(buffer + 0x17, 0x0800_0022);
        e.register(HANDLER_OBJECT_QUERY, |_, _| returns(1));
        e.register(FORM_CAN_BE_LEVELED, |_, _| returns(1));
        e.register_double(MAKE_LEVELED_CREATURE, move |_, a| {
            assert_eq!(a, [0x41, 0x42]);
            returns(creature)
        });
        e.register(0x0041_81e0, |_, _| returns(0));
        e.register(FORM_ID, |e, a| {
            returns(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 0x0c) })
        });
        e.register(FORM_ID_IS_FILE_FORM, |_, _| returns(0));
        stub(&mut e, OWNER_SET_BASE);
        e.register(BUFFER_FIELD_COPY_42CE30, |e, a| {
            e.mem.set_u32(a[1], 0xc0de);
            returns(a[1])
        });
        e.register(BUFFER_FLAG_42CE90, |_, _| returns(3));
        stub(&mut e, BUFFER_SET_FIELD_86CF00);
        stub(&mut e, BUFFER_SET_FLAG_42CE50);
        stub(&mut e, CREATURE_SET_MARKER);
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            load_ov2(
                e,
                buffer,
                list,
                Bytes::new().w(1).b(0x2e).w(0x41).w(0x42).w(0xaa),
            );
        });
        let extra = find_extra(&mut e, list, 0x2e).addr();
        assert_eq!(e.mem.u32(extra + 0x0c), 0x41);
        assert_eq!(e.mem.u32(extra + 0x10), 0x42);
        assert_eq!(calls_to(&log, OWNER_SET_BASE), vec![vec![owner, creature]]);
        assert_eq!(
            calls_to(&log, CREATURE_SET_MARKER),
            vec![vec![creature + 0x30, 0xaa]]
        );
        // The buffer's fields were replaced and put back.
        assert_eq!(
            calls_to(&log, BUFFER_SET_FIELD_86CF00),
            vec![vec![buffer, 0], vec![buffer, 0xc0de]]
        );
        assert_eq!(
            calls_to(&log, BUFFER_SET_FLAG_42CE50),
            vec![vec![buffer, 0], vec![buffer, 3]]
        );
        assert_eq!(e.mem.u32(buffer + 0x17), 0x0800_0022);
    }

    // ---- the constructors, destructors and fix-ups from 0042c700 ----

    fn sorted_types(e: &Engine, list: Ptr<ExtraDataList>) -> Vec<u8> {
        let mut types = chain_types(e, list);
        types.sort();
        types
    }

    #[test]
    fn extra_data_constructors_of_the_ninth_block() {
        // (constructor, type, vtable, the members it zeroes: offset, size)
        type Case = (u32, u8, u32, &'static [(u32, u32)]);
        let cases: &[Case] = &[
            (0x0042_c700, 0x27, 0x0101_58fc, &[(0x0c, 4)]),
            (0x0042_c730, 0x28, 0x0101_5908, &[(0x0c, 4)]),
            (0x0042_c760, 0x0d, 0x0101_5914, &[(0x0c, 4), (0x10, 4)]),
            (0x0042_c7d0, 0x30, 0x0101_5920, &[(0x0c, 4)]),
            (0x0042_c800, 0x4a, 0x0101_592c, &[(0x0c, 1)]),
            (0x0042_c830, 0x1c, 0x0101_5938, &[(0x0c, 4)]),
            (0x0042_c860, 0x1a, 0x0101_5944, &[(0x0c, 4)]),
            (0x0042_c8c0, 0x2f, 0x0101_5950, &[(0x0c, 4), (0x10, 1)]),
            (0x0042_c900, 0x3f, 0x0101_595c, &[(0x0c, 4)]),
            (0x0042_c930, 0x46, 0x0101_5968, &[(0x0c, 4)]),
            (0x0042_c990, 0x4e, 0x0101_5974, &[(0x0c, 1)]),
            (0x0042_c9c0, 0x56, 0x0101_5184, &[(0x0c, 4)]),
            (0x0042_c9f0, 0x5b, 0x0101_5980, &[(0x0c, 4), (0x10, 4)]),
            (0x0042_ca30, 0x5c, 0x0101_5208, &[(0x0c, 4)]),
            (0x0042_ca60, 0x5d, 0x0101_5214, &[(0x0c, 4)]),
            (0x0042_cba0, 0x6e, 0x0101_5998, &[(0x0c, 4), (0x10, 4)]),
            (0x0042_cbe0, 0x70, 0x0101_51fc, &[(0x0c, 4)]),
            (0x0042_cc40, 0x8d, 0x0101_59a4, &[(0x0c, 1)]),
        ];
        for &(address, extra_type, vtable, members) in cases {
            let mut e = engine();
            let object = e.mem.alloc(0x20);
            for offset in 0..0x20 {
                e.mem.set_u8(object + offset, 0xff);
            }
            assert_eq!(e.call(address, &args![object]).u32(), object);
            assert_eq!(e.mem.u32(object), vtable, "{address:#x}");
            assert_eq!(e.mem.u8(object + 4), extra_type, "{address:#x}");
            assert_eq!(e.mem.u32(object + 8), 0, "{address:#x}");
            for offset in 0x0c..0x20 {
                let inside = members
                    .iter()
                    .any(|&(start, size)| offset >= start && offset < start + size);
                let expected = if inside { 0 } else { 0xff };
                assert_eq!(
                    e.mem.u8(object + offset),
                    expected,
                    "{address:#x} +{offset:#x}"
                );
            }
        }
    }

    #[test]
    fn scalar_deleting_destructors_call_theirs_and_free_on_request() {
        for (address, destroy) in [
            (0x0042_c7a0u32, EXTRA_SCRIPT_DESTROY),
            (0x0042_c890, EXTRA_TRESPASS_PACKAGE_DESTROY),
            (0x0042_c960, EXTRA_HEADING_TARGET_DESTROY),
        ] {
            let mut e = engine();
            stub(&mut e, destroy);
            let object = e.mem.alloc(0x10);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(address, &args![object, 0u32]).u32(), object);
            });
            assert_eq!(calls_to(&log, destroy), vec![vec![object]]);
            assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
            let log = logged(&mut e, |e| {
                e.call(address, &args![object, 1u32]);
            });
            assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![object]]);
        }
    }

    #[test]
    fn actor_cause_constructor_builds_its_member() {
        let mut e = engine();
        stub(&mut e, ACTOR_CAUSE_MEMBER_INIT);
        let object = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_ca90, &args![object]).u32(), object);
        });
        assert_eq!(e.mem.u32(object), VTABLE_EXTRA_ACTOR_CAUSE);
        assert_eq!(e.mem.u8(object + 4), EXTRA_ACTOR_CAUSE);
        assert_eq!(
            calls_to(&log, ACTOR_CAUSE_MEMBER_INIT),
            vec![vec![object + 0x0c, 0]]
        );
    }

    #[test]
    fn actor_cause_destructor_assigns_releases_and_runs_the_base() {
        let mut e = engine();
        stub(&mut e, ASSIGN_HANDLE_AT);
        stub(&mut e, ACTOR_CAUSE_MEMBER_RELEASE);
        stub(&mut e, BS_EXTRA_DATA_DESTROY);
        let object = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_cb00, &args![object, 1u32]).u32(), object);
        });
        assert_eq!(e.mem.u32(object), VTABLE_EXTRA_ACTOR_CAUSE);
        let order: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![
                0x0042_cb00,
                ASSIGN_HANDLE_AT,
                ACTOR_CAUSE_MEMBER_RELEASE,
                BS_EXTRA_DATA_DESTROY,
                OPERATOR_DELETE
            ]
        );
        assert_eq!(
            calls_to(&log, ASSIGN_HANDLE_AT),
            vec![vec![object + 0x0c, 0]]
        );
        assert_eq!(
            calls_to(&log, ACTOR_CAUSE_MEMBER_RELEASE),
            vec![vec![object + 0x0c]]
        );
        // Called on its own, the destructor frees nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0042_cb30, &args![object]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn weapon_mod_slots_destructors() {
        let mut e = engine();
        stub(&mut e, BS_EXTRA_DATA_DESTROY);
        let object = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            e.call(0x0042_cca0, &args![object]);
        });
        assert_eq!(e.mem.u32(object), VTABLE_EXTRA_WEAPON_MOD_SLOTS);
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY), vec![vec![object]]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_cc70, &args![object, 1u32]).u32(), object);
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY), vec![vec![object]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![object]]);
    }

    #[test]
    fn securitron_face_constructor_and_destructors() {
        let mut e = engine();
        stub(&mut e, STRING_MEMBER_INIT);
        stub(&mut e, STRING_MEMBER_DESTROY);
        stub(&mut e, BS_EXTRA_DATA_DESTROY);
        let object = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_ccc0, &args![object]).u32(), object);
        });
        assert_eq!(e.mem.u32(object), VTABLE_EXTRA_SECURITRON_FACE);
        assert_eq!(e.mem.u8(object + 4), EXTRA_SECURITRON_FACE);
        assert_eq!(
            calls_to(&log, STRING_MEMBER_INIT),
            vec![vec![object + 0x0c], vec![object + 0x14]]
        );
        e.mem.set_u32(object, 0);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042_cd40, &args![object, 1u32]).u32(), object);
        });
        assert_eq!(e.mem.u32(object), VTABLE_EXTRA_SECURITRON_FACE);
        // The second string goes first, then the base, then the memory.
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (STRING_MEMBER_DESTROY, vec![object + 0x14]),
                (STRING_MEMBER_DESTROY, vec![object + 0x0c]),
                (BS_EXTRA_DATA_DESTROY, vec![object]),
                (OPERATOR_DELETE, vec![object]),
            ]
        );
    }

    #[test]
    fn record_constructor_copies_the_float_through_the_x87_stack() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(FLOAT_COPIED_BY_RECORD_INIT, 0xbf80_0000);
        assert_eq!(e.call(0x0042_cc10, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0);
        assert_eq!(e.mem.u32(object + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(object + 8), 0xbf80_0000);
        // A signalling NaN comes out quiet.
        e.mem.set_u32(FLOAT_COPIED_BY_RECORD_INIT, 0x7fa0_0001);
        e.call(0x0042_cc10, &args![object]);
        assert_eq!(e.mem.u32(object + 8), 0x7fe0_0001);
    }

    #[test]
    fn small_accessors_of_the_buffer_and_the_list_node() {
        let mut e = engine();
        // 0042cde0: IsEmpty of the node the word points at.
        e.register(LIST_IS_EMPTY, |_, a| returns((a[0] == 0x1111) as u32));
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, 0x1111);
        assert!(e.call(0x0042_cde0, &args![holder]).bool());
        e.mem.set_u32(holder, 0x2222);
        assert!(!e.call(0x0042_cde0, &args![holder]).bool());
        // 0042ce00
        e.map(0x011c_6000, 0x1000);
        e.mem.set_u32(WORD_GIVEN_BY_GETTER, 0xcafe);
        assert_eq!(e.call(0x0042_ce00, &args![]).u32(), 0xcafe);
        // 0042ce10, 0042ce30, 0042ce50, 0042ce90, 0042dd70
        let object = e.mem.alloc(0x300);
        assert!(!e.call(0x0042_ce10, &args![object]).bool());
        e.mem.set_u32(object + 0x244, 0b101);
        assert!(!e.call(0x0042_ce10, &args![object]).bool());
        e.mem.set_u32(object + 0x244, 0b110);
        assert!(e.call(0x0042_ce10, &args![object]).bool());
        let out = e.mem.alloc(8);
        e.mem.set_u32(object + 0x2c, 0x1234_5678);
        assert_eq!(e.call(0x0042_ce30, &args![object, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x1234_5678);
        e.mem.set_u32(object + 0x28, 0xf0);
        assert!(!e.call(0x0042_ce90, &args![object]).bool());
        e.call(0x0042_ce50, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x28), 0xf8);
        assert!(e.call(0x0042_ce90, &args![object]).bool());
        e.call(0x0042_ce50, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x28), 0xf0);
        e.mem.set_u8(object + 0x0c, 1);
        e.mem.set_u8(object + 0x0d, 9);
        e.call(0x0042_dd70, &args![object]);
        assert_eq!(e.mem.u8(object + 0x0c), 9);
    }

    // The buffer double of the fix-ups: virtual 0 gives the version byte
    // (the word at +0x38), virtual 8 the reference (+0x30), virtual 0xC the
    // owner (+0x34); the save kind (+0x17) is 2 and the buffer's flags word
    // (+0x2C) is 4.
    const BUFFER_VERSION: u32 = 0x0200_6000;
    const BUFFER_REFERENCE: u32 = 0x0200_6008;
    const BUFFER_OWNER: u32 = 0x0200_600c;

    fn fixup_engine(reference: u32, owner: u32, version: u32) -> (Engine, u32, Ptr<ExtraDataList>) {
        let mut e = engine();
        e.map(SAVE_KIND_TABLE, 0x300);
        e.register(BUFFER_VERSION, |e, a| returns(e.mem.u32(a[0] + 0x38)));
        e.register(BUFFER_REFERENCE, |e, a| returns(e.mem.u32(a[0] + 0x30)));
        e.register(BUFFER_OWNER, |e, a| returns(e.mem.u32(a[0] + 0x34)));
        let buffer = object_with_slots(
            &mut e,
            &[
                (0, BUFFER_VERSION),
                (8, BUFFER_REFERENCE),
                (0x0c, BUFFER_OWNER),
            ],
        );
        e.mem.set_u32(buffer + 0x17, 2);
        e.mem.set_u32(buffer + 0x2c, 4);
        e.mem.set_u32(buffer + 0x30, reference);
        e.mem.set_u32(buffer + 0x34, owner);
        e.mem.set_u32(buffer + 0x38, version);
        // Ids below 0x8000 name forms; the form of an id is the id itself and
        // the casts give it back, except that an `Actor` cast fails from
        // 0x4000.
        e.register(LOOKUP_FORM, |_, a| {
            returns(if a[0] < 0x8000 { a[0] } else { 0 })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            returns(if a[3] == RTTI_ACTOR && a[0] >= 0x4000 {
                0
            } else {
                a[0]
            })
        });
        e.register(REFERENCE_EXTRA_LIST, |_, a| returns(a[0] + 0x44));
        let list = new_list(&mut e);
        (e, buffer, list)
    }

    fn allow(e: &mut Engine, extra_type: u8, mask: u32) {
        e.mem.set_u32(SAVE_KIND_TABLE + extra_type as u32 * 4, mask);
    }

    /// The extra data of `extra_type` of the list, with the word at +0x0C.
    fn extra_with_words(
        e: &mut Engine,
        list: Ptr<ExtraDataList>,
        extra_type: u8,
        words: &[(u32, u32)],
    ) -> Ptr<BSExtraData> {
        let extra = add(e, list, extra_type, 0);
        for &(offset, word) in words {
            e.mem.set_u32(extra.addr() + offset, word);
        }
        extra
    }

    /// A chain of list nodes `(item, next)` holding `items`; gives the nodes.
    fn node_chain(e: &mut Engine, items: &[u32]) -> Vec<u32> {
        let nodes: Vec<u32> = items.iter().map(|_| e.mem.alloc(8)).collect();
        for (index, &item) in items.iter().enumerate() {
            e.mem.set_u32(nodes[index], item);
            let next = nodes.get(index + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[index] + 4, next);
        }
        nodes
    }

    #[test]
    fn finish_load_game_marks_or_calls_back_the_say_to_topic_extra_data() {
        for (answer, words, expected_call) in [
            (1u32, (0x10u32, 0x20u32), Some(1u32)),
            (1, (0x10, 0), Some(0)),
            (1, (0, 0x20), Some(0)),
            (0, (0x10, 0x20), None),
        ] {
            let reference = 0x5000;
            let (mut e, buffer, list) = fixup_engine(reference, 0, 0);
            allow(&mut e, 0x75, 2);
            let object = object_with_slots(&mut e, &[(0xfc, 0x0200_7000)]);
            e.mem.set_u32(buffer + 0x30, object);
            e.register_double(0x0200_7000, move |_, _| returns(answer));
            e.register(FORM_WORD_0C, |_, _| returns(0xabc));
            stub(&mut e, MOBILE_OBJECT_SAY_TO_CALL_BACK);
            let extra = extra_with_words(&mut e, list, 0x75, &[(0x0c, words.1), (0x10, words.0)]);
            let log = logged(&mut e, |e| {
                e.call(0x0042_d9e0, &args![list, buffer]);
            });
            match expected_call {
                Some(flag) => {
                    assert_eq!(
                        calls_to(&log, MOBILE_OBJECT_SAY_TO_CALL_BACK),
                        vec![vec![0xabc, flag]]
                    );
                    assert_eq!(e.mem.u8(extra.addr() + 0x18), 0);
                }
                None => {
                    assert!(calls_to(&log, MOBILE_OBJECT_SAY_TO_CALL_BACK).is_empty());
                    assert_eq!(e.mem.u8(extra.addr() + 0x18), 1);
                }
            }
        }
    }

    #[test]
    fn finish_load_game_needs_a_reference_and_a_matching_save_kind() {
        // Without a reference nothing happens, even with the extra data.
        let (mut e, buffer, list) = fixup_engine(0, 0, 0);
        allow(&mut e, 0x75, 2);
        allow(&mut e, 0x33, 2);
        stub(&mut e, MOBILE_OBJECT_SAY_TO_CALL_BACK);
        stub(&mut e, FINISH_LOAD_ACTIVE_EFFECT_LIST);
        extra_with_words(&mut e, list, 0x75, &[(0x0c, 1), (0x10, 1)]);
        extra_with_words(&mut e, list, 0x33, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_d9e0, &args![list, buffer]);
        });
        assert!(calls_to(&log, MOBILE_OBJECT_SAY_TO_CALL_BACK).is_empty());
        assert!(calls_to(&log, FINISH_LOAD_ACTIVE_EFFECT_LIST).is_empty());
        // With a reference, the active effect list of the type 0x33 extra
        // data is finished; a save kind outside its mask stops it.
        let object = object_with_slots(&mut e, &[(0xfc, 0x0200_7000)]);
        e.register(0x0200_7000, |_, _| returns(1));
        e.register(FORM_WORD_0C, |_, _| returns(0xabc));
        e.mem.set_u32(buffer + 0x30, object);
        let extra = find_extra(&mut e, list, 0x33);
        let log = logged(&mut e, |e| {
            e.call(0x0042_d9e0, &args![list, buffer]);
        });
        assert_eq!(
            calls_to(&log, FINISH_LOAD_ACTIVE_EFFECT_LIST),
            vec![vec![buffer, extra.addr() + 0x20]]
        );
        allow(&mut e, 0x33, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0042_d9e0, &args![list, buffer]);
        });
        assert!(calls_to(&log, FINISH_LOAD_ACTIVE_EFFECT_LIST).is_empty());
    }

    #[test]
    fn clean_up_runs_the_reference_steps_and_removes_the_masked_types() {
        let reference_slots = [(0xfc, 0x0200_7000), (0x1cc, 0x0200_7004)];
        let (mut e, buffer, list) = fixup_engine(0, 0, 0);
        let reference = object_with_slots(&mut e, &reference_slots);
        e.mem.set_u32(buffer + 0x30, reference);
        e.mem.set_u32(buffer + 0x2c, 0x0400_0004);
        stub(&mut e, 0x0200_7004);
        e.register(GET_MODEL_SWAP, |_, _| returns(1));
        stub(&mut e, SET_SELF_DAMAGE);
        e.register(GET_ITEM_DROPPER, |_, _| returns(0x6000));
        stub(&mut e, REMOVE_DROPPED_ITEM);
        let marker = e.mem.alloc(0x10);
        e.mem.set_u8(marker + 0x0d, 5);
        e.register_double(GET_MAP_MARKER, move |_, _| returns(marker));
        stub(&mut e, SET_ACTIVATE_CHILDREN_TIMER);
        e.register(GET_SCRIPT_LOCALS, |_, _| returns(0x7000));
        stub(&mut e, SCRIPT_LOCALS_FINISH_LOAD);
        e.register(FORM_WORD_0C, |_, _| returns(0x0abc));
        e.register(FORM_ID_IS_FILE_FORM, |_, a| {
            returns((a[1] < 0xff00_0000) as u32)
        });
        e.mem.set_u32(buffer + 0x38, 0);
        // The 0x56 extra data is there, so the self damage is reset; the
        // types: 0x05 (buffer kind in its mask), 0x0D (kept by the switch),
        // 0x10 (file forms only), 0x20 (mask outside the buffer's flags).
        extra_with_words(&mut e, list, 0x56, &[]);
        for (extra_type, mask) in [
            (0x05u8, 0x0400_0000u32),
            (0x0d, 0x0400_0000),
            (0x10, 0x4400_0000),
            (0x20, 0x0000_0001),
        ] {
            allow(&mut e, extra_type, mask);
            extra_with_words(&mut e, list, extra_type, &[]);
        }
        let log = logged(&mut e, |e| {
            e.call(0x0042_dae0, &args![list, buffer]);
        });
        assert_eq!(calls_to(&log, 0x0200_7004), vec![vec![reference, 0, 1]]);
        assert_eq!(calls_to(&log, SET_SELF_DAMAGE), vec![vec![reference, 0]]);
        assert_eq!(
            calls_to(&log, REMOVE_DROPPED_ITEM),
            vec![vec![0x6044, reference]]
        );
        assert_eq!(e.mem.u8(marker + 0x0c), 5);
        assert_eq!(
            calls_to(&log, SET_ACTIVATE_CHILDREN_TIMER),
            vec![vec![list.addr(), 0]]
        );
        assert_eq!(
            calls_to(&log, SCRIPT_LOCALS_FINISH_LOAD),
            vec![vec![0x7000, buffer]]
        );
        // A file form: the type 0x10 extra data goes too; the 0x0D one stays.
        let mut types = sorted_types(&e, list);
        types.sort();
        assert_eq!(types, vec![0x0d, 0x20, 0x56]);
    }

    #[test]
    fn clean_up_keeps_the_file_form_only_types_of_a_runtime_reference() {
        let (mut e, buffer, list) = fixup_engine(0x5000, 0, 0);
        let reference = object_with_slots(&mut e, &[(0xfc, 0x0200_7000)]);
        e.mem.set_u32(buffer + 0x30, reference);
        e.mem.set_u32(buffer + 0x2c, 0x0400_0004);
        e.register(GET_MODEL_SWAP, |_, _| returns(0));
        e.register(GET_ITEM_DROPPER, |_, _| returns(0));
        e.register(GET_MAP_MARKER, |_, _| returns(0));
        e.register(GET_SCRIPT_LOCALS, |_, _| returns(0));
        stub(&mut e, SET_ACTIVATE_CHILDREN_TIMER);
        // The word of a runtime reference is 0xFF000010: not a file form.
        e.register(FORM_WORD_0C, |_, _| returns(0xff00_0010));
        e.register(FORM_ID_IS_FILE_FORM, |_, a| {
            returns((a[1] < 0xff00_0000) as u32)
        });
        allow(&mut e, 0x10, 0x4400_0000);
        allow(&mut e, 0x11, 0x0000_0004);
        extra_with_words(&mut e, list, 0x10, &[]);
        extra_with_words(&mut e, list, 0x11, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_dae0, &args![list, buffer]);
        });
        assert_eq!(sorted_types(&e, list), vec![0x10]);
        assert_eq!(
            calls_to(&log, SET_ACTIVATE_CHILDREN_TIMER),
            vec![vec![list.addr(), 0]]
        );
        // Without a reference only the removal loop runs.
        let (mut e, buffer, list) = fixup_engine(0, 0, 0);
        e.mem.set_u32(buffer + 0x2c, 4);
        allow(&mut e, 0x10, 0x4000_0004);
        allow(&mut e, 0x11, 0x0000_0004);
        extra_with_words(&mut e, list, 0x10, &[]);
        extra_with_words(&mut e, list, 0x11, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_dae0, &args![list, buffer]);
        });
        assert_eq!(sorted_types(&e, list), vec![0x10]);
        assert!(calls_to(&log, GET_MODEL_SWAP).is_empty());
    }

    #[test]
    fn fix_up_resolves_the_reference_pointer_and_checks_the_script() {
        let (mut e, buffer, list) = fixup_engine(0, 0, 0);
        allow(&mut e, 0x1c, 2);
        allow(&mut e, 0x0d, 2);
        let pointer = extra_with_words(&mut e, list, 0x1c, &[(0x0c, 0x1234)]);
        // The script's list is the next node of the scriptable form's.
        let scriptable = e.mem.alloc(0x10);
        e.mem.set_u32(scriptable + 4, 0x4444);
        let script = extra_with_words(&mut e, list, 0x0d, &[(0x0c, 0x4444)]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, scriptable]);
        });
        assert_eq!(e.mem.u32(pointer.addr() + 0x0c), 0x1234);
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![
                vec![0x1234, 0, RTTI_TES_FORM, RTTI_TES_OBJECT_REFR, 0],
                vec![scriptable, 0, RTTI_TES_FORM, RTTI_TES_SCRIPTABLE_FORM, 0]
            ]
        );
        assert_eq!(sorted_types(&e, list), vec![0x0d, 0x1c]);
        // Another list in the script extra data removes it.
        e.mem.set_u32(script.addr() + 0x0c, 0x5555);
        e.call(0x0042_ceb0, &args![list, buffer, scriptable]);
        assert_eq!(sorted_types(&e, list), vec![0x1c]);
        // An id that names no form leaves null; no id looks nothing up.
        e.mem.set_u32(pointer.addr() + 0x0c, 0x9000);
        e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        assert_eq!(e.mem.u32(pointer.addr() + 0x0c), 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        assert!(calls_to(&log, LOOKUP_FORM).is_empty());
    }

    #[test]
    fn fix_up_takes_the_script_list_of_the_owners_base_form() {
        let owner = 0x6000;
        let (mut e, buffer, list) = fixup_engine(0, owner, 0);
        allow(&mut e, 0x0d, 2);
        let base = e.mem.alloc(0x200);
        e.mem.set_u32(base + 0xf4 + 4, 0x4444);
        e.register_double(REFERENCE_BASE_FORM, move |_, _| returns(base));
        extra_with_words(&mut e, list, 0x0d, &[(0x0c, 0x4444)]);
        e.call(0x0042_ceb0, &args![list, buffer, 0x7000u32]);
        assert_eq!(sorted_types(&e, list), vec![0x0d]);
        e.mem.set_u32(base + 0xf4 + 4, 0x4445);
        e.call(0x0042_ceb0, &args![list, buffer, 0x7000u32]);
        assert!(sorted_types(&e, list).is_empty());
        // No base form: removed too.
        let (mut e, buffer, list) = fixup_engine(0, owner, 0);
        allow(&mut e, 0x0d, 2);
        e.register(REFERENCE_BASE_FORM, |_, _| returns(0));
        extra_with_words(&mut e, list, 0x0d, &[(0x0c, 0x4444)]);
        e.call(0x0042_ceb0, &args![list, buffer, 0x7000u32]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn fix_up_with_a_reference_and_no_owner() {
        let (mut e, buffer, list) = fixup_engine(0x5000, 0, 0);
        for extra_type in [0x6c, 0x32, 0x33, 0x39, 0x55, 0x2b] {
            allow(&mut e, extra_type, 2);
        }
        e.register(MAGIC_ITEM_FROM_FORM, |_, a| returns(a[0] + 1));
        e.register(MAGIC_TARGET_FROM_ID, |_, a| returns(a[0] + 2));
        stub(&mut e, RESOLVE_ACTIVE_EFFECT_LIST);
        stub(&mut e, ADD_DROPPED_ITEM_LIST_ENTRY);
        stub(&mut e, TELEPORT_DATA_FINISH_LOAD);
        extra_with_words(&mut e, list, 0x6c, &[(0x0c, 0x9000)]);
        let caster = extra_with_words(
            &mut e,
            list,
            0x32,
            &[(0x18, 0x100), (0x1c, 0x200), (0x20, 0x300)],
        );
        let target = extra_with_words(&mut e, list, 0x33, &[(0x1c, 0x400)]);
        let dropper = extra_with_words(&mut e, list, 0x39, &[(0x0c, 0x500)]);
        let talking = extra_with_words(&mut e, list, 0x55, &[(0x0c, 0x600)]);
        extra_with_words(&mut e, list, 0x2b, &[(0x0c, 0x7700)]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        assert!(!sorted_types(&e, list).contains(&0x6c));
        assert_eq!(e.mem.u32(caster.addr() + 0x18), 0x101);
        assert_eq!(e.mem.u32(caster.addr() + 0x1c), 0x202);
        assert_eq!(e.mem.u32(caster.addr() + 0x20), 0x300);
        assert_eq!(e.mem.u32(target.addr() + 0x1c), 0x400);
        assert_eq!(
            calls_to(&log, RESOLVE_ACTIVE_EFFECT_LIST),
            vec![vec![buffer, target.addr() + 0x20]]
        );
        assert_eq!(e.mem.u32(dropper.addr() + 0x0c), 0x500);
        assert_eq!(
            calls_to(&log, ADD_DROPPED_ITEM_LIST_ENTRY),
            vec![vec![0x544, 0x5000]]
        );
        assert_eq!(e.mem.u32(talking.addr() + 0x0c), 0x600);
        assert!(calls_to(&log, RT_DYNAMIC_CAST)
            .iter()
            .any(|c| c[3] == RTTI_MOBILE_OBJECT));
        assert_eq!(
            calls_to(&log, TELEPORT_DATA_FINISH_LOAD),
            vec![vec![0x7700, buffer]]
        );
        // A dropper that is gone removes its extra data; zero ids stay zero.
        e.mem.set_u32(dropper.addr() + 0x0c, 0x9001);
        e.mem.set_u32(caster.addr() + 0x18, 0);
        e.mem.set_u32(caster.addr() + 0x1c, 0);
        e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        assert!(!sorted_types(&e, list).contains(&0x39));
        assert_eq!(e.mem.u32(caster.addr() + 0x18), 0);
        assert_eq!(e.mem.u32(caster.addr() + 0x1c), 0);
    }

    #[test]
    fn fix_up_without_a_reference_stops_after_the_script() {
        let (mut e, buffer, list) = fixup_engine(0, 0x6000, 0);
        allow(&mut e, 0x6c, 2);
        allow(&mut e, 0x3c, 2);
        extra_with_words(&mut e, list, 0x6c, &[(0x0c, 0x9000)]);
        extra_with_words(&mut e, list, 0x3c, &[(0x0c, 0x9000)]);
        e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        // Neither is touched.
        assert_eq!(sorted_types(&e, list), vec![0x3c, 0x6c]);
        assert_eq!(word_of(&mut e, list, 0x6c), 0x9000);
    }

    #[test]
    fn fix_up_package_and_owner_arms() {
        let (mut e, buffer, list) = fixup_engine(0x5000, 0x6000, 0x20);
        for extra_type in [0x19, 0x70, 0x3c, 0x1a, 0x46, 0x5f, 0x89, 0x35] {
            allow(&mut e, extra_type, 2);
        }
        stub(&mut e, PACKAGE_CALCULATE_PROCEDURE_TYPE);
        e.register(PACKAGE_PROCEDURE_TYPE, |_, _| returns(0xffff_ffff));
        stub(&mut e, GET_PACKAGE_EXTRA);
        stub(&mut e, ASHPILE_LIST_FIX);
        stub(&mut e, PLAYER_CRIME_ITEM_FIX);
        stub(&mut e, 0x0200_7010);
        stub(&mut e, 0x0200_7014);
        let package_object = object_with_slots(&mut e, &[(0x14, 0x0200_7010)]);
        let trespass_object = object_with_slots(&mut e, &[(0x64, 0x0200_7014)]);
        let package = extra_with_words(&mut e, list, 0x19, &[(0x0c, 0x100), (0x14, 0x200)]);
        let data = extra_with_words(&mut e, list, 0x70, &[(0x0c, package_object)]);
        let merchant = extra_with_words(&mut e, list, 0x3c, &[(0x0c, 0x300)]);
        extra_with_words(&mut e, list, 0x1a, &[(0x0c, trespass_object)]);
        let head = extra_with_words(&mut e, list, 0x46, &[(0x0c, 0x400)]);
        let limbs = extra_with_words(&mut e, list, 0x5f, &[(0x14, 0x500)]);
        let ashpile = extra_with_words(&mut e, list, 0x89, &[(0x0c, 0x600)]);
        let crimes = extra_with_words(&mut e, list, 0x35, &[]);
        let nodes = node_chain(&mut e, &[0x11, 0, 0x12]);
        e.mem.set_u32(crimes.addr() + 0x0c, nodes[0]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        let _ = (data, merchant, head);
        // The procedure type is computed with the package and the reference.
        assert_eq!(e.mem.u32(package.addr() + 0x0c), 0x100);
        assert_eq!(
            calls_to(&log, PACKAGE_CALCULATE_PROCEDURE_TYPE),
            vec![vec![0x100, 0x200]]
        );
        assert_eq!(calls_to(&log, GET_PACKAGE_EXTRA), vec![vec![list.addr()]]);
        assert_eq!(
            calls_to(&log, 0x0200_7010),
            vec![vec![package_object, buffer]]
        );
        assert_eq!(
            calls_to(&log, 0x0200_7014),
            vec![vec![trespass_object, buffer]]
        );
        // The dismembered limbs' form is looked up, without a cast.
        assert_eq!(e.mem.u32(limbs.addr() + 0x14), 0x500);
        assert!(calls_to(&log, RT_DYNAMIC_CAST)
            .iter()
            .all(|cast| cast[0] != 0x500));
        assert_eq!(calls_to(&log, ASHPILE_LIST_FIX), vec![vec![0x644, 0x6000]]);
        assert_eq!(e.mem.u32(ashpile.addr() + 0x0c), 0x600);
        assert_eq!(
            calls_to(&log, PLAYER_CRIME_ITEM_FIX),
            vec![vec![0x11, 0x6000], vec![0x12, 0x6000]]
        );
    }

    #[test]
    fn fix_up_package_arm_removes_a_package_that_is_gone_or_known() {
        let (mut e, buffer, list) = fixup_engine(0x5000, 0x6000, 0x20);
        allow(&mut e, 0x19, 2);
        stub(&mut e, PACKAGE_CALCULATE_PROCEDURE_TYPE);
        e.register(PACKAGE_PROCEDURE_TYPE, |_, _| returns(5));
        extra_with_words(&mut e, list, 0x19, &[(0x0c, 0x100), (0x14, 0x200)]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        // A known procedure type is not computed again.
        assert!(calls_to(&log, PACKAGE_CALCULATE_PROCEDURE_TYPE).is_empty());
        assert_eq!(sorted_types(&e, list), vec![0x19]);
        let package_extra = find_extra(&mut e, list, 0x19);
        e.mem.set_u32(package_extra.addr() + 0x0c, 0x9000);
        e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn fix_up_followers_keeps_actors_and_removes_the_others() {
        // Followers 0x10, 0x4001 (no actor), 0x20, 0x4002 (no actor): version
        // 0x20 does not remove the player.
        let (mut e, buffer, list) = fixup_engine(0x5000, 0x6000, 0x20);
        allow(&mut e, EXTRA_FOLLOWER, 2);
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(1));
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_SET_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            if item != 0 {
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE_ITEM, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let head = a[0];
            let (mut node, mut previous) = (head, head);
            while node != 0 && e.mem.u32(node) != wanted {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            if node == head {
                let next = e.mem.u32(head + 4);
                if next == 0 {
                    e.mem.set_u32(head, 0);
                } else {
                    let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                    e.mem.set_u32(head, item);
                    e.mem.set_u32(head + 4, after);
                }
            } else if node != 0 {
                let after = e.mem.u32(node + 4);
                e.mem.set_u32(previous + 4, after);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(SIMPLE_LIST_ITEM, |_, a| returns(a[0]));
        let nodes = node_chain(&mut e, &[0x10, 0x4001, 0x20, 0x4002]);
        extra_with_words(&mut e, list, EXTRA_FOLLOWER, &[(0x0c, nodes[0])]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        // The remaining chain is 0x10 -> 0x20.
        let first = word_of(&mut e, list, EXTRA_FOLLOWER);
        assert_eq!(first, nodes[0]);
        assert_eq!(e.mem.u32(nodes[0]), 0x10);
        let second = e.mem.u32(nodes[0] + 4);
        assert_eq!(e.mem.u32(second), 0x20);
        assert_eq!(e.mem.u32(second + 4), 0);
        // The player (0x7777) is only removed below version 0x0E.
        let removals = calls_to(&log, LIST_REMOVE_ITEM);
        assert_eq!(removals.len(), 2);
        // An older save removes the player too.
        e.mem.set_u32(buffer + 0x38, 0x0d);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        let removals = calls_to(&log, LIST_REMOVE_ITEM);
        assert_eq!(removals.len(), 1);
        assert_eq!(removals[0][0], nodes[0]);
    }

    #[test]
    fn fix_up_followers_removes_a_head_that_is_no_actor() {
        let (mut e, buffer, list) = fixup_engine(0x5000, 0x6000, 0x20);
        allow(&mut e, EXTRA_FOLLOWER, 2);
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(1));
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_SET_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        let nodes = node_chain(&mut e, &[0x4001, 0x30]);
        extra_with_words(&mut e, list, EXTRA_FOLLOWER, &[(0x0c, nodes[0])]);
        let log = logged(&mut e, |e| {
            e.call(0x0042_ceb0, &args![list, buffer, 0u32]);
        });
        // The head took the next node's item and link.
        assert_eq!(e.mem.u32(nodes[0]), 0x30);
        assert_eq!(e.mem.u32(nodes[0] + 4), 0);
        assert_eq!(calls_to(&log, LIST_REMOVE_HEAD), vec![vec![nodes[0]]]);
    }
}
