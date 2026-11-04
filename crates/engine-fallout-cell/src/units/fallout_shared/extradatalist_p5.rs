//! `fallout shared/extradatalist.cpp` (Xbox PDB source unit), part 5: its functions from `0042dd90` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradatalist`]; anything public there may be used here.
//!
//! First session (40 functions, `0042dd90` to `0042ea50`): the hot key,
//! info general topic, talking actor, model swap, navmesh portal, weapon mod
//! slot / is-modding, faction changes, dismembered limbs, actor cause and
//! combat style extra data (getters, setters, removers, and the small
//! constructors and destructors the compiler placed here), with the
//! `ExtraDataList` function at `0042dd90` that drops the saved animation,
//! havok data and last finished sequence.
//!
//! Second session (40 functions, `0042eb10` to `0042f8a0`): the combat style
//! constructor, the ammo, say-to-topic-info, say-topic-info-once-a-day,
//! water zone map, ignored-by-sandbox, patrol-ref-in-use and follower swim
//! breadcrumbs extra data (getters, setters, removers and constructors), the
//! `NavMeshPtr` and `TESBoundObject *` `BSSimpleArray` constructors and
//! destructors, the `NiTMap<TESObjectREFR *, bool>` constructor and deleting
//! destructor, the `NiPointer<ActorCause>` functions, and the array
//! functions `0042f5f0` (set size), `0042f850` (add) and `0042f8a0` (remove
//! at an index). The next session continues at `0042f9c0`.
//!
//! Third session (25 functions, `0042f9c0` to `00569140`, the end of the
//! range): the functions of the one-word smart pointer of the
//! `BSSimpleArray<NavMeshPtr,1024>` (copy, null and object constructors,
//! destructor, assignment, comparison, deleting destructor), that array's
//! clear, move-buffer, constructor-body and element functions, the
//! `NiTMapBase<..., TESObjectREFR *, bool>` constructor and destructor
//! bodies, the `BSSimpleArray<TESBoundObject *,1024>` destructors, the
//! type `0x10` extra data constructor, and the two getters at `00555bc0` and
//! `00569140`.
//!
//! The extra data type numbers are `EXTRA_DATA_TYPE` of the Xbox PDB
//! (`0x4A` `EXTRA_HOT_KEY`, `0x4D` `EXTRA_INFO_GENERAL_TOPIC`, ...). The
//! compiler's exception-unwinding frames (the `FS:[0]` chains of the setters
//! that call `new`) are not translated. Every setter allocates with
//! `operator new` and, as the code does, carries a failed allocation (a null
//! block) on to `AddExtra`.

#[allow(unused_imports)]
use super::extradatalist::*;
#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use crate::types::BSSimpleArray;

// ---------------------------------------------------------------------------
// Constants

/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (`platform`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSExtraData::BSExtraData(type)`: sets the base vtable, the type, and a
/// null next.
const BS_EXTRA_DATA_INIT: u32 = 0x0040_ec80;
/// `BSExtraData::~BSExtraData` (`0040ecb0`).
const BS_EXTRA_DATA_DESTROY: u32 = 0x0040_ecb0;
/// Count of the non-null items of a `BSSimpleList` (`005ae380`).
const LIST_COUNT: u32 = 0x005a_e380;

/// `ExtraDataList::RemoveSavedAnimation` (`00422aa0`).
const REMOVE_SAVED_ANIMATION: u32 = 0x0042_2aa0;
/// `ExtraDataList::RemoveSavedHavokData` (`00422c20`).
const REMOVE_SAVED_HAVOK_DATA: u32 = 0x0042_2c20;
/// `ExtraDataList::RemoveLastFinishedSequence` (`00422920`).
const REMOVE_LAST_FINISHED_SEQUENCE: u32 = 0x0042_2920;
/// `MOV EAX,[ECX+0x2C]; MOV EDX,[ESP+4]; MOV [EDX],EAX`: copies the word at
/// +0x2C of `this` (a form's flags word) into the slot it is given and
/// returns that slot (`0042ce30`).
const COPY_FLAGS_WORD: u32 = 0x0042_ce30;
/// Whether `[this] & mask` is nonzero (`004280f0`, `this` a pointer to a
/// flags word).
const FLAGS_TEST: u32 = 0x0042_80f0;
/// The routine `fn_0042e010` forwards to, with the words it reads from the
/// `MenuTopic` (`0083df80`, `this` = the `MenuTopic`, four words).
const MENU_TOPIC_FORWARD: u32 = 0x0083_df80;

/// Constructor of the type `0x4A` extra data (`ExtraHotKey`, `00432380`,
/// `this` = the new block, then the hot key byte zero-extended).
const EXTRA_HOT_KEY_INIT: u32 = 0x0043_2380;
/// Constructor of the type `0x4D` extra data (`ExtraInfoGeneralTopic`,
/// `00432470`).
const EXTRA_INFO_GENERAL_TOPIC_INIT: u32 = 0x0043_2470;
/// `ExtraTalkingActor::ExtraTalkingActor` (Xbox PDB, `00436b20`; two words).
const EXTRA_TALKING_ACTOR_INIT: u32 = 0x0043_6b20;
/// Copy of the 8-byte value at +0x0C of the navmesh portal record
/// (`0069a690`, `this` = the copy).
const NAVMESH_PORTAL_COPY: u32 = 0x0069_a690;
/// Constructor of the type `0x8D` extra data without argument (`0042cc40`).
const EXTRA_WEAPON_MOD_SLOTS_INIT: u32 = 0x0042_cc40;
/// Reads the byte at +0x0C of the type `0x8D` extra data (`00424940`).
const WEAPON_MOD_FLAGS_READ: u32 = 0x0042_4940;
/// Constructor of the type `0x5E` extra data (`00436cb0`).
const EXTRA_FACTION_CHANGES_INIT: u32 = 0x0043_6cb0;
/// `ExtraDismemberedLimbs::ExtraDismemberedLimbs` (Xbox PDB, `00430200`).
const EXTRA_DISMEMBERED_LIMBS_INIT: u32 = 0x0043_0200;
/// Constructor of the type `0x60` extra data (`ExtraActorCause`, `0042ca90`).
const EXTRA_ACTOR_CAUSE_INIT: u32 = 0x0042_ca90;
/// `NiPointer<ActorCause>::operator=` (Xbox PDB, `0042f780`; `this` = the
/// smart pointer, then the new pointer).
const ACTOR_CAUSE_POINTER_ASSIGN: u32 = 0x0042_f780;
/// Constructor of the type `0x69` extra data (`ExtraCombatStyle`, `0042eb10`).
const EXTRA_COMBAT_STYLE_INIT: u32 = 0x0042_eb10;

/// Extra data types (`EXTRA_DATA_TYPE` of the Xbox PDB) of this part.
const EXTRA_USED_MARKERS: u8 = 0x12;
const EXTRA_HOT_KEY: u8 = 0x4a;
const EXTRA_INFO_GENERAL_TOPIC: u8 = 0x4d;
const EXTRA_TALKING_ACTOR: u8 = 0x55;
const EXTRA_NAVMESH_PORTAL: u8 = 0x5a;
const EXTRA_MODEL_SWAP: u8 = 0x5b;
const EXTRA_FACTION_CHANGES: u8 = 0x5e;
const EXTRA_DISMEMBERED_LIMBS: u8 = 0x5f;
const EXTRA_ACTOR_CAUSE: u8 = 0x60;
const EXTRA_COMBAT_STYLE: u8 = 0x69;
const EXTRA_WEAPON_MOD_SLOTS: u8 = 0x8d;
const EXTRA_WEAPON_IS_MODDING: u8 = 0x8e;

/// Vtables set by the constructors of this part.
const VTABLE_EXTRA_MODEL_SWAP: u32 = 0x0101_5980;
const VTABLE_EXTRA_WEAPON_MOD_SLOTS: u32 = 0x0101_59a4;
const VTABLE_EXTRA_WEAPON_IS_MODDING: u32 = 0x0101_59bc;

// Second session (`0042eb10` to `0042f8a0`).

/// Constructor of the type `0x75` extra data (`ExtraSayToTopicInfo`) with no
/// argument (`00437690`, `this` = the new block).
const EXTRA_SAY_TO_TOPIC_INFO_INIT_EMPTY: u32 = 0x0043_7690;
/// Constructor of `ExtraSayToTopicInfo` that stores `pInfo` (`004376e0`,
/// `this` = the new block, then the topic info).
const EXTRA_SAY_TO_TOPIC_INFO_INIT: u32 = 0x0043_76e0;
/// Stores `pTopic` (the word at +0x10) of an `ExtraSayToTopicInfo`
/// (`00437730`, `this` = the extra data, then the topic).
const EXTRA_SAY_TO_TOPIC_INFO_SET_TOPIC: u32 = 0x0043_7730;
/// Constructor of the type `0x73` extra data (`ExtraSayTopicInfoOnceADay`,
/// `00437510`, `this` = the new block, then one word).
const EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_INIT: u32 = 0x0043_7510;
/// Called by `fn_0042ef20` with `this` = the word at +0x0C of an existing
/// `ExtraSayTopicInfoOnceADay` (`pListofSaidOnceTopicInfos`) and the address
/// of a stack word that holds its argument (`005ae3d0`).
const SAID_ONCE_LIST_ADD: u32 = 0x005a_e3d0;
/// Constructor of the type `0x7e` extra data (`ExtraWaterZoneMap`, `00437750`).
const EXTRA_WATER_ZONE_MAP_INIT: u32 = 0x0043_7750;
/// Called by `fn_0042f030` on an `ExtraWaterZoneMap` (`00437850`, `this` =
/// the extra data, then a word and a byte).
const EXTRA_WATER_ZONE_MAP_UPDATE: u32 = 0x0043_7850;
/// `0084e3a0`, `this` = the address of a map member: called with the
/// `WaterZoneMap` at +0x0C of an `ExtraWaterZoneMap` (a zero result means it
/// is empty) and with the address +4 of the object at +0x1C of it
/// (`pHighestWaterZone`).
const MAP_COUNT: u32 = 0x0084_e3a0;
/// Called by `fn_0042f130` with `this` = the `WaterZoneMap` at +0x0C, a word
/// and the address of a count that starts at 0 (`00853130`).
const WATER_ZONE_MAP_COUNT_OF: u32 = 0x0085_3130;
/// Called by `fn_0042f180` with `this` = the `WaterZoneMap` (`004b9ba0`); its
/// result is the key handed to `006b7f20`.
const WATER_ZONE_MAP_KEY: u32 = 0x004b_9ba0;
/// Called by `fn_0042f180` with `this` = the `WaterZoneMap`, then the
/// addresses of three stack words: the key (read), an output word and a third
/// word (`006b7f20`).
const WATER_ZONE_MAP_LOOKUP: u32 = 0x006b_7f20;
/// Constructor of the type `0x80` extra data (`ExtraIgnoredBySandbox`,
/// `00437970`).
const EXTRA_IGNORED_BY_SANDBOX_INIT: u32 = 0x0043_7970;
/// Constructor of the type `0x88` extra data (`ExtraPatrolRefInUseData`,
/// `004379a0`, `this` = the new block, then the `User` word).
const EXTRA_PATROL_REF_IN_USE_DATA_INIT: u32 = 0x0043_79a0;
/// `ExtraPatrolRefInUseData::GetCanBeUsedBy` (Xbox PDB, `00437ab0`; `this` =
/// the extra data, then one word; returns a byte).
const PATROL_REF_GET_CAN_BE_USED_BY: u32 = 0x0043_7ab0;
/// `ExtraFollowerSwimBreadcrumbs::ExtraFollowerSwimBreadcrumbs` (Xbox PDB,
/// `00437c40`).
const EXTRA_FOLLOWER_SWIM_BREADCRUMBS_INIT: u32 = 0x0043_7c40;

/// The memory manager object (`00401020`, a function that returns a
/// constant; `platform`).
const GET_MEMORY_MANAGER: u32 = 0x0040_1020;
/// The memory manager routine `fn_0042f5d0` forwards to (`00aa4150`, `this` =
/// the manager, then the two words `fn_0042f5d0` was given; its result is
/// returned).
const MEMORY_MANAGER_ROUTINE: u32 = 0x00aa_4150;

/// Takes a reference on the object at `this` (`0044bbc0`; the engine map's
/// name for it, a `LockFreeMap` method, is an identical-code fold).
const OBJECT_ADD_REFERENCE: u32 = 0x0044_bbc0;
/// Releases a reference of the `ActorCause` at `this` (`0066e2b0`, in
/// `actorcause.cpp`).
const ACTOR_CAUSE_RELEASE: u32 = 0x0066_e2b0;

/// The callees of the array functions of this session. The array is the
/// shared `BSSimpleArray` (vtable, `pBuffer`, `iSize`, `iReservedSize`) with
/// pointer-sized elements; `this` is the array unless said otherwise.
/// `008454f0(this, byte)`, run by `fn_0042f5f0` for a new size of 0.
const ARRAY_SET_SIZE_ZERO: u32 = 0x0084_54f0;
/// `0042fc00(this, new capacity, count)`: moves the buffer to a new capacity.
const ARRAY_REALLOCATE: u32 = 0x0042_fc00;
/// `00430000(this, address, count)`.
const ARRAY_RANGE_CONSTRUCT: u32 = 0x0043_0000;
/// `0072ba80(this, address, count)`.
const ARRAY_RANGE_DESTROY: u32 = 0x0072_ba80;
/// `0042fb20(this, address, count)`.
const ARRAY_ELEMENTS_RELEASE: u32 = 0x0042_fb20;
/// `0042fb60(this, destination, source, count)`.
const ARRAY_ELEMENTS_MOVE: u32 = 0x0042_fb60;
/// `00761540(this)`: makes room for one element and returns its index.
const ARRAY_ADD_SLOT: u32 = 0x0076_1540;
/// `0042fa60(this, address, count)`.
const ARRAY_ELEMENTS_PREPARE: u32 = 0x0042_fa60;
/// `0042f9c0(this, flag)`.
const ARRAY_FREE: u32 = 0x0042_f9c0;
/// `0042fcb0(this, capacity, size)`.
const ARRAY_INIT: u32 = 0x0042_fcb0;
/// `0042fdc0(this, word)`: builds the element at `this`.
const ARRAY_ELEMENT_INIT: u32 = 0x0042_fdc0;
/// `0042fe10(this, hash size)`: constructor body of the
/// `NiTMap<TESObjectREFR *, bool>`.
const NI_T_MAP_REFR_BOOL_INIT: u32 = 0x0042_fe10;
/// `0042fe80(this)`: destructor body of the `NiTMap<TESObjectREFR *, bool>`.
const NI_T_MAP_REFR_BOOL_DESTROY: u32 = 0x0042_fe80;
/// `006b3eb0(this, capacity, size)`: constructor body of the
/// `BSSimpleArray<TESBoundObject *, 1024>`.
const BOUND_OBJECT_ARRAY_INIT: u32 = 0x006b_3eb0;
/// `006f3170(this)`: whether the array is sparse enough to shrink (a byte).
const ARRAY_CAN_SHRINK: u32 = 0x006f_3170;
/// `00869600(this)`: the capacity the array shrinks to.
const ARRAY_SHRUNK_CAPACITY: u32 = 0x0086_9600;
/// `006a8500(this)`: frees the buffer through the array's vtable.
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;

/// Extra data types of this session.
const EXTRA_AMMO: u8 = 0x6e;
const EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY: u8 = 0x73;
const EXTRA_SAY_TO_TOPIC_INFO: u8 = 0x75;
const EXTRA_WATER_ZONE_MAP: u8 = 0x7e;
const EXTRA_IGNORED_BY_SANDBOX: u8 = 0x80;
const EXTRA_PATROL_REF_IN_USE_DATA: u8 = 0x88;
const EXTRA_FOLLOWER_SWIM_BREADCRUMBS: u8 = 0x8b;

/// Vtables set by the constructors of this session (the class of each is the
/// RTTI name of the vtable).
const VTABLE_EXTRA_AMMO: u32 = 0x0101_5998;
const VTABLE_EXTRA_COMBAT_STYLE: u32 = 0x0101_59c8;
const VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR: u32 = 0x0101_59d4;
const VTABLE_NI_T_MAP_REFR_BOOL: u32 = 0x0101_59e8;
const VTABLE_BS_SIMPLE_ARRAY_BOUND_OBJECT: u32 = 0x0101_5a08;

// Third session (`0042f9c0` to `00569140`).

/// `006e6da0(size, address)`: placement `new`, which gives `address` back.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `memmove(destination, source, size)` (`00ec7230`, cdecl).
const MEMORY_MOVE: u32 = 0x00ec_7230;
/// `memset(block, value, size)` (`00403d30`, cdecl).
const MEMORY_SET: u32 = 0x0040_3d30;
/// Takes a reference: `this` = the address of the reference count, +0x1C of
/// the object the one-word smart pointer holds (`0040f6e0`).
const OBJECT_REFERENCE_ADD: u32 = 0x0040_f6e0;
/// Releases a reference: `this` = the same address (`00401970`).
const OBJECT_REFERENCE_RELEASE: u32 = 0x0040_1970;
/// Offset of the reference count in the object the smart pointer holds.
const REFERENCE_COUNT_OFFSET: u32 = 0x1c;
/// `00aa1070(bytes)`: allocates the hash table of the
/// `NiTMapBase<..., TESObjectREFR *, bool>` (cdecl).
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
/// `00aa10f0(block)`: frees that table (cdecl; a null block is ignored).
const TABLE_FREE: u32 = 0x00aa_10f0;
/// `00438af0(this)`: empties the `NiTMapBase<..., TESObjectREFR *, bool>`.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `004b0460(this, other)`: whether the word at `this` equals the word at
/// `other` (a byte in AL, the rest of EAX zero).
const POINTER_EQUAL: u32 = 0x004b_0460;
/// `this + 0x28` (`004610d0`), the list of the object `fn_00555bc0` works on.
const LIST_AT_OFFSET_0X28: u32 = 0x0046_10d0;
/// `this + 0x44` (`005d43c0`), the list of the object `fn_00569140` works on.
const LIST_AT_OFFSET_0X44: u32 = 0x005d_43c0;
/// The word at +0x0C of the list's type 5 extra data, or 0 (`00421910`).
const EXTRA_WORD_SEEN_DATA: u32 = 0x0042_1910;
/// The word at +0x0C of the list's type `0x2A` extra data, or 0 (`004182b0`).
const EXTRA_WORD_LOCK: u32 = 0x0041_82b0;

const VTABLE_NI_T_MAP_BASE_REFR_BOOL: u32 = 0x0101_5a1c;
const VTABLE_EXTRA_ANIM: u32 = 0x0101_5b28;

// ---------------------------------------------------------------------------
// Helpers

/// The first extra data of `extra_type` in the list (`GetExtraData`).
fn find_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> Ptr<BSExtraData> {
    base_extra_list_get_extra_data(e, list.cast(), extra_type)
}

/// `new` and construct: allocates `size` bytes and runs `construct` on the
/// block, or gives null when the allocation failed.
fn new_object(e: &mut Engine, size: u32, construct: impl FnOnce(&mut Engine, u32) -> u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        0
    } else {
        construct(e, block)
    }
}

/// A new extra data of `size` bytes, built by the constructor at `construct`
/// (`this` = the block, then `construct_args`). Returns the extra data
/// (null when the allocation failed).
fn build_extra(
    e: &mut Engine,
    size: u32,
    construct: u32,
    construct_args: &[u32],
) -> Ptr<BSExtraData> {
    Ptr::new(new_object(e, size, |e, block| {
        let mut words = vec![block];
        words.extend_from_slice(construct_args);
        e.call(construct, &words).u32()
    }))
}

/// `AddExtra` of the list.
fn add_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    base_extra_list_add_extra(e, list.cast(), extra);
}

/// The shape of the getters of a pointer or word: the word at +0x0C of the
/// first extra data of `extra_type`, or 0 when the list has none.
fn extra_word(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    }
}

/// `if (extra = GetExtraData(type)) RemoveExtra(extra, true)`.
fn remove_if_present(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) {
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        base_extra_list_remove_extra(e, list.cast(), extra, true);
    }
}

// Translated from 0042dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function of `ExtraDataList` (`this`) called with a form: runs
/// `RemoveSavedAnimation` (`00422aa0`), `RemoveSavedHavokData` (`00422c20`)
/// and `RemoveLastFinishedSequence` (`00422920`), then, when bit 31 of the
/// form's flags word (the word at +0x2C, copied through `0042ce30` and tested
/// by `004280f0`) is set, removes the type `0x12` extra data
/// (`EXTRA_USEDMARKERS` in the Xbox enum). Its only caller is `005629a0`.
pub fn fn_0042dd90(e: &mut Engine, this: Ptr<ExtraDataList>, form: u32) {
    e.call(REMOVE_SAVED_ANIMATION, &args![this]);
    e.call(REMOVE_SAVED_HAVOK_DATA, &args![this]);
    e.call(REMOVE_LAST_FINISHED_SEQUENCE, &args![this]);
    let flagged = e.with_stack(4, |e, copy| {
        let copied = e.call(COPY_FLAGS_WORD, &args![form, copy]).u32();
        e.call(FLAGS_TEST, &args![copied, 0x8000_0000u32]).bool()
    });
    if flagged {
        base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_USED_MARKERS);
    }
}

// Translated from 0042dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetHotKey` (Xbox PDB): stores `hot_key` (a byte at +0x0C)
/// in the type `0x4A` extra data (`ExtraHotKey`), or builds one (`0x10`
/// bytes, `00432380`, given the byte zero-extended) and adds it.
pub fn extra_data_list_set_hot_key(e: &mut Engine, this: Ptr<ExtraDataList>, hot_key: u8) {
    let extra = find_extra(e, this, EXTRA_HOT_KEY);
    if extra.is_null() {
        let extra = build_extra(e, 0x10, EXTRA_HOT_KEY_INIT, &[hot_key as u32]);
        add_extra(e, this, extra);
    } else {
        e.mem.set_u8(extra.addr() + 0x0c, hot_key);
    }
}

// Translated from 0042de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetHotKey` (Xbox PDB): the byte at +0x0C of the type
/// `0x4A` extra data, or `0xFF` when the list has none.
pub fn extra_data_list_get_hot_key(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_HOT_KEY);
    if extra.is_null() {
        0xff
    } else {
        e.mem.u8(extra.addr() + 0x0c)
    }
}

// Translated from 0042dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveHotKey` (Xbox PDB): `RemoveExtra` by type `0x4A`.
pub fn extra_data_list_remove_hot_key(e: &mut Engine, this: Ptr<ExtraDataList>) {
    base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_HOT_KEY);
}

// Translated from 0042dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetInfoGeneralTopic` (Xbox PDB): stores `topic`
/// (`pInfoGen`, a `MenuTopic*` at +0x0C) in the type `0x4D` extra data
/// (`ExtraInfoGeneralTopic`), or builds one (`0x10` bytes, `00432470`) and
/// adds it.
pub fn extra_data_list_set_info_general_topic(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    topic: u32,
) {
    let extra = find_extra(e, this, EXTRA_INFO_GENERAL_TOPIC);
    if extra.is_null() {
        let extra = build_extra(e, 0x10, EXTRA_INFO_GENERAL_TOPIC_INIT, &[topic]);
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, topic);
    }
}

// Translated from 0042df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic of the type `0x4D` extra data (the word at +0x0C, a
/// `MenuTopic*`), or 0 when there is none. A non-null topic that
/// `fn_0042dff0` finds empty is also passed to `fn_0042e010` with `argument`.
pub fn fn_0042df90(e: &mut Engine, this: Ptr<ExtraDataList>, argument: u32) -> u32 {
    let extra = find_extra(e, this, EXTRA_INFO_GENERAL_TOPIC);
    if extra.is_null() {
        return 0;
    }
    let topic = e.mem.u32(extra.addr() + 0x0c);
    if topic != 0 && fn_0042dff0(e, Ptr::new(topic)) {
        fn_0042e010(e, Ptr::new(topic), argument);
    }
    topic
}

// Translated from 0042dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `BSSimpleList` at +0x0C of the `MenuTopic` has no item
/// (its count, `005ae380`, is 0).
pub fn fn_0042dff0(e: &mut Engine, this: Ptr) -> bool {
    e.call(LIST_COUNT, &args![this.addr() + 0x0c]).u32() == 0
}

// Translated from 0042e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0083df80` on the `MenuTopic` with the words at +0x14, +0x28 and
/// +0x18 of it, then `argument`.
pub fn fn_0042e010(e: &mut Engine, this: Ptr, argument: u32) {
    let first = e.mem.u32(this.addr() + 0x14);
    let second = e.mem.u32(this.addr() + 0x28);
    let third = e.mem.u32(this.addr() + 0x18);
    e.call(
        MENU_TOPIC_FORWARD,
        &args![this, first, second, third, argument],
    );
}

// Translated from 0042e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remover of the type `0x4D` extra data: `RemoveExtra` by type.
pub fn fn_0042e040(e: &mut Engine, this: Ptr<ExtraDataList>) {
    base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_INFO_GENERAL_TOPIC);
}

// Translated from 0042e060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetTalkingActorExtra` (Xbox PDB): deletes an existing type
/// `0x55` extra data (`ExtraTalkingActor`), then always builds a new one
/// (`0x10` bytes, `00436b20`, given `first` and `second`) and adds it.
pub fn extra_data_list_set_talking_actor_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    first: u32,
    second: u32,
) {
    remove_if_present(e, this, EXTRA_TALKING_ACTOR);
    let extra = build_extra(e, 0x10, EXTRA_TALKING_ACTOR_INIT, &[first, second]);
    add_extra(e, this, extra);
}

// Translated from 0042e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetTalkingActorExtra` (Xbox PDB): the type `0x55` extra
/// data, or null.
pub fn extra_data_list_get_talking_actor_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_TALKING_ACTOR)
}

// Translated from 0042e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remover of the type `0x55` extra data: `RemoveExtra` by type.
pub fn fn_0042e130(e: &mut Engine, this: Ptr<ExtraDataList>) {
    base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_TALKING_ACTOR);
}

// Translated from 0042e150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the type `0x5B` extra data (`ExtraModelSwap`: `pModelSwap` at
/// +0x0C and `pModelSwapForm` at +0x10, Xbox PDB): stores both words in an
/// existing one, or builds one (`0x14` bytes, `fn_0042e210`) and adds it.
pub fn fn_0042e150(e: &mut Engine, this: Ptr<ExtraDataList>, model_swap: u32, swap_form: u32) {
    let extra = find_extra(e, this, EXTRA_MODEL_SWAP);
    if extra.is_null() {
        let extra = Ptr::new(new_object(e, 0x14, |e, block| {
            fn_0042e210(e, Ptr::new(block), model_swap, swap_form).addr()
        }));
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, model_swap);
        e.mem.set_u32(extra.addr() + 0x10, swap_form);
    }
}

// Translated from 0042e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraModelSwap` (Xbox PDB): a `BSExtraData` of type `0x5B`
/// with `pModelSwap` and `pModelSwapForm` set. Returns `this`.
pub fn fn_0042e210(e: &mut Engine, this: Ptr, model_swap: u32, swap_form: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_MODEL_SWAP as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_MODEL_SWAP);
    e.mem.set_u32(this.addr() + 0x0c, model_swap);
    e.mem.set_u32(this.addr() + 0x10, swap_form);
    this
}

// Translated from 0042e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetModelSwap` (Xbox PDB): `pModelSwap` (the word at +0x0C)
/// of the type `0x5B` extra data, or 0.
pub fn extra_data_list_get_model_swap(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word(e, this, EXTRA_MODEL_SWAP)
}

// Translated from 0042e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remover of the type `0x5B` extra data: `RemoveExtra` by type.
pub fn fn_0042e280(e: &mut Engine, this: Ptr<ExtraDataList>) {
    base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_MODEL_SWAP);
}

// Translated from 0042e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetNavMeshPortal` (Xbox PDB): the type `0x5A` extra data
/// (`EXTRA_NAVMESH_PORTAL`), or null.
pub fn extra_data_list_get_nav_mesh_portal(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_NAVMESH_PORTAL)
}

// Translated from 0042e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the type `0x5A` extra data (the engine map leaves it unnamed;
/// `ExtraDataList::GetNavMeshPortal` and `RemoveNavMeshPortal` are its
/// neighbours): when the list has none, builds one (`0x14` bytes) whose
/// 8-byte member is a copy (`0069a690`) of the 8 bytes at +0x0C of `portal`,
/// handed by value to its constructor (`fn_00416ad0`), and adds it. An
/// existing extra data is left as it is.
pub fn fn_0042e2c0(e: &mut Engine, this: Ptr<ExtraDataList>, portal: u32) {
    let extra = find_extra(e, this, EXTRA_NAVMESH_PORTAL);
    if !extra.is_null() {
        return;
    }
    let extra = Ptr::new(new_object(e, 0x14, |e, block| {
        e.with_stack(8, |e, copy| {
            e.call(NAVMESH_PORTAL_COPY, &args![copy, portal + 0x0c]);
            let low = e.mem.u32(copy.addr());
            let high = e.mem.u32(copy.addr() + 4);
            fn_00416ad0(e, Ptr::new(block), low, high).addr()
        })
    }));
    add_extra(e, this, extra);
}

// Translated from 0042e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetWeaponModSlot` (Xbox PDB): ORs the bits `slot` into the
/// byte at +0x0C of the type `0x8D` extra data (`ExtraWeaponModFlags`,
/// `cWeaponModsActive`). With none in the list and a nonzero `slot`, builds
/// one (`0x10` bytes, `fn_0042e450`), ORs the bits in, and adds it. With none
/// and `slot` 0 the code still calls `fn_0042e480` on the null extra data,
/// which reads memory at +0x0C of address 0 (the game would fault).
pub fn extra_data_list_set_weapon_mod_slot(e: &mut Engine, this: Ptr<ExtraDataList>, slot: u8) {
    let extra = find_extra(e, this, EXTRA_WEAPON_MOD_SLOTS);
    if extra.is_null() && slot != 0 {
        let extra: Ptr = Ptr::new(new_object(e, 0x10, |e, block| {
            fn_0042e450(e, Ptr::new(block), slot).addr()
        }));
        fn_0042e480(e, extra, slot);
        add_extra(e, this, extra.cast());
    } else {
        fn_0042e480(e, extra.cast(), slot);
    }
}

// Translated from 0042e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraWeaponModFlags` (Xbox PDB), type `0x8D`: sets
/// `cWeaponModsActive` (byte at +0x0C) to `slot`. Returns `this`.
pub fn fn_0042e450(e: &mut Engine, this: Ptr, slot: u8) -> Ptr {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_WEAPON_MOD_SLOTS as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_WEAPON_MOD_SLOTS);
    e.mem.set_u8(this.addr() + 0x0c, slot);
    this
}

// Translated from 0042e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// ORs `bits` into the byte at +0x0C of the extra data.
pub fn fn_0042e480(e: &mut Engine, this: Ptr, bits: u8) {
    let flags = e.mem.u8(this.addr() + 0x0c);
    e.mem.set_u8(this.addr() + 0x0c, flags | bits);
}

// Translated from 0042e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0x0C of the type `0x8D` extra data to `value`
/// (`fn_0042e680`); with none in the list, builds one (`0x10` bytes,
/// `0042cc40`), sets the byte, and adds it.
pub fn fn_0042e4a0(e: &mut Engine, this: Ptr<ExtraDataList>, value: u8) {
    let extra = find_extra(e, this, EXTRA_WEAPON_MOD_SLOTS);
    if extra.is_null() {
        let extra: Ptr = build_extra(e, 0x10, EXTRA_WEAPON_MOD_SLOTS_INIT, &[]).cast();
        fn_0042e680(e, extra, value);
        add_extra(e, this, extra.cast());
    } else {
        fn_0042e680(e, extra.cast(), value);
    }
}

// Translated from 0042e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWeaponModFlags` (Xbox PDB): the byte of the type `0x8D`
/// extra data (read by `00424940`), or 0 when the list has none.
pub fn extra_data_list_get_weapon_mod_flags(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_WEAPON_MOD_SLOTS);
    if extra.is_null() {
        0
    } else {
        e.call(WEAPON_MOD_FLAGS_READ, &args![extra]).u8()
    }
}

// Translated from 0042e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetIsModding` (Xbox PDB): stores `modding` (a byte at
/// +0x0C, `fn_0042e680`) in the type `0x8E` extra data
/// (`EXTRA_WEAPON_IS_MODDING`), or builds one (`0x10` bytes, `fn_0042e650`)
/// and adds it.
pub fn extra_data_list_set_is_modding(e: &mut Engine, this: Ptr<ExtraDataList>, modding: u8) {
    let extra = find_extra(e, this, EXTRA_WEAPON_IS_MODDING);
    if extra.is_null() {
        let extra = Ptr::new(new_object(e, 0x10, |e, block| {
            fn_0042e650(e, Ptr::new(block), modding).addr()
        }));
        add_extra(e, this, extra);
    } else {
        fn_0042e680(e, extra.cast(), modding);
    }
}

// Translated from 0042e650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x8E` extra data: sets the byte at +0x0C to
/// `modding`. Returns `this`.
pub fn fn_0042e650(e: &mut Engine, this: Ptr, modding: u8) -> Ptr {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_WEAPON_IS_MODDING as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_WEAPON_IS_MODDING);
    e.mem.set_u8(this.addr() + 0x0c, modding);
    this
}

// Translated from 0042e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x0C of the extra data.
pub fn fn_0042e680(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x0c, value);
}

// Translated from 0042e6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the type `0x8E` extra data: the destructor
/// (`fn_0042e6d0`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0042e6a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0042e6d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0042e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the type `0x8E` extra data: sets its vtable, then runs
/// `BSExtraData::~BSExtraData` (`0040ecb0`).
pub fn fn_0042e6d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_WEAPON_IS_MODDING);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0042e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveIsModding` (Xbox PDB): deletes the type `0x8E`
/// extra data if the list has one.
pub fn extra_data_list_remove_is_modding(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_if_present(e, this, EXTRA_WEAPON_IS_MODDING);
}

// Translated from 0042e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveNavMeshPortal` (Xbox PDB): deletes the type `0x5A`
/// extra data if the list has one.
pub fn extra_data_list_remove_nav_mesh_portal(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_if_present(e, this, EXTRA_NAVMESH_PORTAL);
}

// Translated from 0042e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a type `0x5E` extra data (`EXTRA_FACTION_CHANGES` in the Xbox enum;
/// `0x10` bytes, constructor `00436cb0`) when the list has none.
pub fn fn_0042e760(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_FACTION_CHANGES);
    if extra.is_null() {
        let extra = build_extra(e, 0x10, EXTRA_FACTION_CHANGES_INIT, &[]);
        add_extra(e, this, extra);
    }
}

// Translated from 0042e800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type `0x5E` extra data, or null.
pub fn fn_0042e800(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_FACTION_CHANGES)
}

// Translated from 0042e820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddDismembermentExtra` (Xbox PDB): the type `0x5F` extra
/// data (`ExtraDismemberedLimbs`), built (`0x30` bytes, `00430200`) and added
/// first when the list has none. Returns it.
pub fn extra_data_list_add_dismemberment_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    let mut extra = find_extra(e, this, EXTRA_DISMEMBERED_LIMBS);
    if extra.is_null() {
        extra = build_extra(e, 0x30, EXTRA_DISMEMBERED_LIMBS_INIT, &[]);
        add_extra(e, this, extra);
    }
    extra
}

// Translated from 0042e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetDismembermentExtra` (Xbox PDB): the type `0x5F` extra
/// data, or null.
pub fn extra_data_list_get_dismemberment_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_DISMEMBERED_LIMBS)
}

// Translated from 0042e8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveDismembermentExtra` (Xbox PDB): deletes the type
/// `0x5F` extra data if the list has one.
pub fn extra_data_list_remove_dismemberment_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_if_present(e, this, EXTRA_DISMEMBERED_LIMBS);
}

// Translated from 0042e910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type `0x60` extra data (`EXTRA_ACTOR_CAUSE` in the Xbox enum), or
/// null.
pub fn fn_0042e910(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_ACTOR_CAUSE)
}

// Translated from 0042e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the actor cause of the type `0x60` extra data
/// (`ExtraActorCause`): a null `cause` removes the extra data by type;
/// otherwise the cause is assigned (`fn_0042ea00`) to an existing one, or to
/// a new one (`0x10` bytes, `0042ca90`) that is then added.
pub fn fn_0042e930(e: &mut Engine, this: Ptr<ExtraDataList>, cause: u32) {
    if cause == 0 {
        base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_ACTOR_CAUSE);
        return;
    }
    let extra = find_extra(e, this, EXTRA_ACTOR_CAUSE);
    if extra.is_null() {
        let extra: Ptr = build_extra(e, 0x10, EXTRA_ACTOR_CAUSE_INIT, &[]).cast();
        fn_0042ea00(e, extra, cause);
        add_extra(e, this, extra.cast());
    } else {
        fn_0042ea00(e, extra.cast(), cause);
    }
}

// Translated from 0042ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `cause` to the `NiPointer<ActorCause>` (`spActorCause`) at +0x0C
/// of the extra data (`NiPointer<ActorCause>::operator=`, `0042f780`).
pub fn fn_0042ea00(e: &mut Engine, this: Ptr, cause: u32) {
    e.call(
        ACTOR_CAUSE_POINTER_ASSIGN,
        &args![this.addr() + 0x0c, cause],
    );
}

// Translated from 0042ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pCombatStyle` (the word at +0x0C) of the type `0x69` extra data
/// (`ExtraCombatStyle`), or 0.
pub fn fn_0042ea20(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word(e, this, EXTRA_COMBAT_STYLE)
}

// Translated from 0042ea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the combat style of the type `0x69` extra data: a null
/// `combat_style` removes the extra data by type; otherwise it is stored in
/// an existing one, or a new one (`0x10` bytes, `0042eb10`) is built and
/// added.
pub fn fn_0042ea50(e: &mut Engine, this: Ptr<ExtraDataList>, combat_style: u32) {
    if combat_style == 0 {
        base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_COMBAT_STYLE);
        return;
    }
    let extra = find_extra(e, this, EXTRA_COMBAT_STYLE);
    if extra.is_null() {
        let extra = build_extra(e, 0x10, EXTRA_COMBAT_STYLE_INIT, &[combat_style]);
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, combat_style);
    }
}

// Translated from 0042eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraCombatStyle` (RTTI of vtable `010159c8`), type
/// `0x69`: sets `pCombatStyle` (the word at +0x0C). Returns `this`.
pub fn fn_0042eb10(e: &mut Engine, this: Ptr, combat_style: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_COMBAT_STYLE as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_COMBAT_STYLE);
    e.mem.set_u32(this.addr() + 0x0c, combat_style);
    this
}

// Translated from 0042eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetAmmo` (named so in the unit's main file): the type
/// `0x6E` extra data (`ExtraAmmo`), or null.
pub fn extra_data_list_get_ammo(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_AMMO)
}

// Translated from 0042eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetAmmo(ammo, count)` (named so in the unit's main file):
/// with a null `ammo` or a zero `count`, removes the type `0x6E` extra data
/// (`ExtraAmmo`) by type; otherwise stores both in an existing one
/// (`pAmmo` at +0x0C, `iCount` at +0x10), or builds one (`0x14` bytes,
/// `fn_0042ec30`) and adds it.
pub fn extra_data_list_set_ammo(e: &mut Engine, this: Ptr<ExtraDataList>, ammo: u32, count: u32) {
    if ammo == 0 || count == 0 {
        base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_AMMO);
        return;
    }
    let extra = find_extra(e, this, EXTRA_AMMO);
    if extra.is_null() {
        let extra = Ptr::new(new_object(e, 0x14, |e, block| {
            fn_0042ec30(e, Ptr::new(block), ammo, count).addr()
        }));
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, ammo);
        e.mem.set_u32(extra.addr() + 0x10, count);
    }
}

// Translated from 0042ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraAmmo` (RTTI of vtable `01015998`), type `0x6E`: sets
/// `pAmmo` (+0x0C) and `iCount` (+0x10). Returns `this`.
pub fn fn_0042ec30(e: &mut Engine, this: Ptr, ammo: u32, count: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_AMMO as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_AMMO);
    e.mem.set_u32(this.addr() + 0x0c, ammo);
    e.mem.set_u32(this.addr() + 0x10, count);
    this
}

// Translated from 0042ec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of `pInfo` (the word at +0x0C) of the type `0x75` extra data
/// (`ExtraSayToTopicInfo`): a null `info` removes the extra data
/// (`ExtraDataList::RemoveSayToInfoExtra`); otherwise the word is stored in
/// an existing one, or a new one (`0x1C` bytes, `004376e0`) is built and
/// added.
pub fn fn_0042ec70(e: &mut Engine, this: Ptr<ExtraDataList>, info: u32) {
    if info == 0 {
        extra_data_list_remove_say_to_info_extra(e, this);
        return;
    }
    let extra = find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO);
    if extra.is_null() {
        let extra = build_extra(e, 0x1c, EXTRA_SAY_TO_TOPIC_INFO_INIT, &[info]);
        add_extra(e, this, extra);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, info);
    }
}

// Translated from 0042ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pTopic` (the word at +0x10) of the type `0x75` extra data
/// (`ExtraSayToTopicInfo`), or 0.
pub fn fn_0042ed30(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x10)
    }
}

// Translated from 0042ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pInfo` (the word at +0x0C) of the type `0x75` extra data
/// (`ExtraSayToTopicInfo`), or 0.
pub fn fn_0042ed70(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word(e, this, EXTRA_SAY_TO_TOPIC_INFO)
}

// Translated from 0042edb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveSayToInfoExtra` (Xbox PDB): deletes the type `0x75`
/// extra data (`ExtraSayToTopicInfo`) if the list has one.
pub fn extra_data_list_remove_say_to_info_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_if_present(e, this, EXTRA_SAY_TO_TOPIC_INFO);
}

// Translated from 0042ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetSayToExtra` (Xbox PDB): the type `0x75` extra data, or
/// null.
pub fn extra_data_list_get_say_to_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO)
}

// Translated from 0042ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of `pTopic` (the word at +0x10) of the type `0x75` extra data
/// (`ExtraSayToTopicInfo`): a null `topic` removes the extra data. Otherwise,
/// with none in the list, builds one (`0x1C` bytes, constructor `00437690`),
/// zeroes `pInfo` and adds it; then stores the topic in the extra data
/// (`00437730`). A failed allocation would make the code write at address
/// 0x0C.
pub fn fn_0042ee00(e: &mut Engine, this: Ptr<ExtraDataList>, topic: u32) {
    if topic == 0 {
        extra_data_list_remove_say_to_info_extra(e, this);
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO);
    if extra.is_null() {
        extra = build_extra(e, 0x1c, EXTRA_SAY_TO_TOPIC_INFO_INIT_EMPTY, &[]);
        e.mem.set_u32(extra.addr() + 0x0c, 0);
        add_extra(e, this, extra);
    }
    e.call(EXTRA_SAY_TO_TOPIC_INFO_SET_TOPIC, &args![extra, topic]);
}

// Translated from 0042eec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `response_id` (`nResponseID`, the word at +0x14) in the type
/// `0x75` extra data (`ExtraSayToTopicInfo`) when the list has one.
pub fn fn_0042eec0(e: &mut Engine, this: Ptr<ExtraDataList>, response_id: u32) {
    let extra = find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO);
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x14, response_id);
    }
}

// Translated from 0042eef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `nResponseID` (the word at +0x14) of the type `0x75` extra data
/// (`ExtraSayToTopicInfo`), or 0.
pub fn fn_0042eef0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_SAY_TO_TOPIC_INFO);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x14)
    }
}

// Translated from 0042ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of the type `0x73` extra data (`ExtraSayTopicInfoOnceADay`): a null
/// `value` removes the extra data (`fn_0042f000`). Otherwise, with one in the
/// list, `005ae3d0` is called on the word at its +0x0C with the address of a
/// stack word holding `value`; with none, one is built (`0x10` bytes,
/// `00437510` given `value`) and added.
pub fn fn_0042ef20(e: &mut Engine, this: Ptr<ExtraDataList>, value: u32) {
    if value == 0 {
        fn_0042f000(e, this);
        return;
    }
    let extra = find_extra(e, this, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY);
    if extra.is_null() {
        let extra = build_extra(e, 0x10, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_INIT, &[value]);
        add_extra(e, this, extra);
    } else {
        let list = e.mem.u32(extra.addr() + 0x0c);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), value);
            e.call(SAID_ONCE_LIST_ADD, &args![list, slot]);
        });
    }
}

// Translated from 0042efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type `0x73` extra data (`ExtraSayTopicInfoOnceADay`), or null.
pub fn fn_0042efe0(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY)
}

// Translated from 0042f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x73` extra data (`ExtraSayTopicInfoOnceADay`) if the
/// list has one.
pub fn fn_0042f000(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_if_present(e, this, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY);
}

// Translated from 0042f030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the type `0x7e` extra data (`ExtraWaterZoneMap`) with `zone` and
/// `flag`: builds one (`0x20` bytes, `00437750`) and adds it when the list
/// has none, then runs `00437850` on it. When the `WaterZoneMap` at +0x0C is
/// then empty (`0084e3a0` gives 0) the extra data is deleted and the result
/// is 0; otherwise the result is `0084e3a0` of the address +4 of the object
/// at +0x1C (`pHighestWaterZone`), or 0 when that is null.
pub fn fn_0042f030(e: &mut Engine, this: Ptr<ExtraDataList>, zone: u32, flag: u8) -> u32 {
    let mut extra = find_extra(e, this, EXTRA_WATER_ZONE_MAP);
    if extra.is_null() {
        extra = build_extra(e, 0x20, EXTRA_WATER_ZONE_MAP_INIT, &[]);
        add_extra(e, this, extra);
    }
    e.call(
        EXTRA_WATER_ZONE_MAP_UPDATE,
        &args![extra, zone, flag as u32],
    );
    if e.call(MAP_COUNT, &args![extra.addr() + 0x0c]).u32() == 0 {
        base_extra_list_remove_extra(e, this.cast(), extra, true);
        return 0;
    }
    let highest = e.mem.u32(extra.addr() + 0x1c);
    if highest == 0 {
        0
    } else {
        e.call(MAP_COUNT, &args![highest + 4]).u32()
    }
}

// Translated from 0042f130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `WaterZoneMap` of the type `0x7e` extra data
/// (`ExtraWaterZoneMap`) gives a positive count (`00853130`, with `value`
/// and the address of a count that starts at 0); false when the list has
/// none.
pub fn fn_0042f130(e: &mut Engine, this: Ptr<ExtraDataList>, value: u32) -> bool {
    let extra = find_extra(e, this, EXTRA_WATER_ZONE_MAP);
    if extra.is_null() {
        return false;
    }
    let count = e.with_stack(4, |e, slot| {
        e.call(
            WATER_ZONE_MAP_COUNT_OF,
            &args![extra.addr() + 0x0c, value, slot],
        );
        e.mem.u32(slot.addr()) as i32
    });
    count > 0
}

// Translated from 0042f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up in the `WaterZoneMap` of the list (`fn_0042f1d0`): with a map
/// whose key (`004b9ba0`) is nonzero, runs the lookup `006b7f20` with the
/// address of the key and of two output words, and returns the first output
/// word; 0 otherwise.
pub fn fn_0042f180(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let map = extra_data_list_q_water_zone_map(e, this);
    if map.is_null() {
        return 0;
    }
    let key = e.call(WATER_ZONE_MAP_KEY, &args![map]).u32();
    if key == 0 {
        return 0;
    }
    e.with_stack(12, |e, slots| {
        let key_slot = slots.addr();
        let output = slots.addr() + 4;
        let third = slots.addr() + 8;
        e.mem.set_u32(key_slot, key);
        e.call(WATER_ZONE_MAP_LOOKUP, &args![map, key_slot, output, third]);
        e.mem.u32(output)
    })
}

// Translated from 0042f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::QWaterZoneMap` (Xbox PDB): the address of the
/// `WaterZoneMap` (+0x0C) in the type `0x7e` extra data (`ExtraWaterZoneMap`),
/// or null.
pub fn extra_data_list_q_water_zone_map(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr {
    let extra = find_extra(e, this, EXTRA_WATER_ZONE_MAP);
    if extra.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(extra.addr() + 0x0c)
    }
}

// Translated from 0042f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetIgnoredBySandbox` (Xbox PDB): with a nonzero `ignored`
/// and no type `0x80` extra data (`ExtraIgnoredBySandbox`), builds one
/// (`0x0C` bytes, `00437970`) and adds it; with a zero `ignored` and one in
/// the list, deletes it.
pub fn extra_data_list_set_ignored_by_sandbox(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    ignored: u8,
) {
    let extra = find_extra(e, this, EXTRA_IGNORED_BY_SANDBOX);
    if extra.is_null() && ignored != 0 {
        let extra = build_extra(e, 0x0c, EXTRA_IGNORED_BY_SANDBOX_INIT, &[]);
        add_extra(e, this, extra);
    } else if !extra.is_null() && ignored == 0 {
        base_extra_list_remove_extra(e, this.cast(), extra, true);
    }
}

// Translated from 0042f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the list has a type `0x80` extra data (`ExtraIgnoredBySandbox`).
pub fn fn_0042f2d0(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    !find_extra(e, this, EXTRA_IGNORED_BY_SANDBOX).is_null()
}

// Translated from 0042f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a nonzero `user` and no type `0x88` extra data
/// (`ExtraPatrolRefInUseData`), builds one (`0x10` bytes, `004379a0` given
/// `user`) and adds it; with a zero `user` and one in the list, deletes it.
pub fn fn_0042f300(e: &mut Engine, this: Ptr<ExtraDataList>, user: u32) {
    let extra = find_extra(e, this, EXTRA_PATROL_REF_IN_USE_DATA);
    if extra.is_null() && user != 0 {
        let extra = build_extra(e, 0x10, EXTRA_PATROL_REF_IN_USE_DATA_INIT, &[user]);
        add_extra(e, this, extra);
    } else if !extra.is_null() && user == 0 {
        base_extra_list_remove_extra(e, this.cast(), extra, true);
    }
}

// Translated from 0042f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPatrolRefCanBeUsedBy` (Xbox PDB): true when the list
/// has no type `0x88` extra data (`ExtraPatrolRefInUseData`), otherwise
/// whether `ExtraPatrolRefInUseData::GetCanBeUsedBy` (`00437ab0`) gives a
/// nonzero byte for `user`.
pub fn extra_data_list_get_patrol_ref_can_be_used_by(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    user: u32,
) -> bool {
    let extra = find_extra(e, this, EXTRA_PATROL_REF_IN_USE_DATA);
    if extra.is_null() {
        return true;
    }
    e.call(PATROL_REF_GET_CAN_BE_USED_BY, &args![extra, user])
        .u8()
        != 0
}

// Translated from 0042f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetOrCreateExtraFollowerSwimBreadcrumbs` (Xbox PDB): the
/// type `0x8B` extra data (`ExtraFollowerSwimBreadcrumbs`); with none in the
/// list, builds one (`0x28` bytes, `00437c40`), adds it and returns it.
pub fn extra_data_list_get_or_create_extra_follower_swim_breadcrumbs(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    let mut extra = find_extra(e, this, EXTRA_FOLLOWER_SWIM_BREADCRUMBS);
    if extra.is_null() {
        extra = build_extra(e, 0x28, EXTRA_FOLLOWER_SWIM_BREADCRUMBS_INIT, &[]);
        add_extra(e, this, extra);
    }
    extra
}

// Translated from 0042f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `word` to `0042fdc0` on `this` (it builds the array element at
/// `this`).
pub fn fn_0042f4c0(e: &mut Engine, this: Ptr, word: u32) {
    e.call(ARRAY_ELEMENT_INIT, &args![this, word]);
}

// Translated from 0042f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap<TESObjectREFR *, bool>` (RTTI of vtable
/// `010159e8`): the constructor body `0042fe10` with `hash_size`, then the
/// vtable. Returns `this`.
pub fn fn_0042f4e0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.call(NI_T_MAP_REFR_BOOL_INIT, &args![this, hash_size]);
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP_REFR_BOOL);
    this
}

// Translated from 0042f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshPtr,1024>::scalar deleting destructor` (Xbox PDB):
/// the destructor `fn_0042f830`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_simple_array_nav_mesh_ptr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0042f830(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0042f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, bool>::scalar deleting destructor` (Xbox PDB):
/// the destructor body `0042fe80`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_t_map_tes_object_refr_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(NI_T_MAP_REFR_BOOL_DESTROY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0042f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of the `BSSimpleArray<TESBoundObject *, 1024>` (RTTI
/// of vtable `01015a08`): sets the vtable, then runs the constructor body
/// `006b3eb0` with capacity 0 and size 0. Returns `this`.
pub fn fn_0042f570(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem
        .set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY_BOUND_OBJECT);
    e.call(BOUND_OBJECT_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0042f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smaller of two unsigned words (cdecl `min`).
pub fn fn_0042f5a0(_e: &mut Engine, first: u32, second: u32) -> u32 {
    if first < second {
        first
    } else {
        second
    }
}

// Translated from 0042f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards both words to the memory manager routine `00aa4150` (on the
/// object `00401020` gives) and returns its result (cdecl; its caller
/// `005f2820` stores the result in a pointer field, so it is a block address:
/// the first word is the block, the second the size).
pub fn fn_0042f5d0(e: &mut Engine, block: u32, size: u32) -> u32 {
    let manager = e.call(GET_MEMORY_MANAGER, &args![]).u32();
    e.call(MEMORY_MANAGER_ROUTINE, &args![manager, block, size])
        .u32()
}

// Translated from 0042f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the size of a `BSSimpleArray` (`iSize` at +8, `iReservedSize` at
/// +0x0C, `pBuffer` at +4; the vtable's slot at +4 allocates a buffer of the
/// given capacity). A new size of 0 runs `008454f0` with `shrink`. A size
/// above the capacity allocates (through the vtable) when the capacity is 0,
/// or moves the buffer (`0042fc00`), then constructs the new elements
/// (`00430000`). A smaller size destroys the elements above it (`0072ba80`)
/// and, with `shrink` set and the size no more than a quarter of the
/// capacity, moves the buffer to the new size. The same or a larger size
/// within the capacity only constructs the new elements.
pub fn fn_0042f5f0(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32, shrink: u8) {
    if new_size == 0 {
        e.call(ARRAY_SET_SIZE_ZERO, &args![this, shrink as u32]);
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    if new_size > reserved {
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![new_size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        } else {
            e.call(ARRAY_REALLOCATE, &args![this, new_size, size]);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        }
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let size = e.get(this, BSSimpleArray::iSize);
        e.call(
            ARRAY_RANGE_CONSTRUCT,
            &args![this, buffer + size * 4, new_size - size],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
    } else if new_size < size {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_RANGE_DESTROY,
            &args![this, buffer + new_size * 4, size - new_size],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
        if shrink != 0 {
            let reserved = e.get(this, BSSimpleArray::iReservedSize);
            if new_size <= reserved >> 2 {
                e.call(ARRAY_REALLOCATE, &args![this, new_size, new_size]);
                e.set(this, BSSimpleArray::iReservedSize, new_size);
            }
        }
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_RANGE_CONSTRUCT,
            &args![this, buffer + size * 4, new_size - size],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
    }
}

// Translated from 0042f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a smart pointer (one word, `NiPointer`-like): stores
/// `object` and, when it is not null, takes a reference on it (`0044bbc0`).
/// Returns `this`.
pub fn fn_0042f730(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(this.addr(), object);
    if object != 0 {
        e.call(OBJECT_ADD_REFERENCE, &args![object]);
    }
    this
}

// Translated from 0042f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `NiPointer<ActorCause>`: releases the pointed-to
/// `ActorCause` (`0066e2b0`) when the pointer is not null.
pub fn fn_0042f760(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(ACTOR_CAUSE_RELEASE, &args![object]);
    }
}

// Translated from 0042f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<ActorCause>::operator=` (Xbox PDB): when `object` differs from
/// the held pointer, releases the old one (`0066e2b0`), stores `object` and
/// takes a reference on it (`0044bbc0`). Returns `this`.
pub fn ni_pointer_actor_cause_assign(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    let held = e.mem.u32(this.addr());
    if held != object {
        if held != 0 {
            e.call(ACTOR_CAUSE_RELEASE, &args![held]);
        }
        e.mem.set_u32(this.addr(), object);
        if object != 0 {
            e.call(OBJECT_ADD_REFERENCE, &args![object]);
        }
    }
    this
}

// Translated from 0042f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<NavMeshPtr,1024>` (RTTI of vtable
/// `010159d4`) with a capacity: sets the vtable, then runs `0042fcb0` with
/// `capacity` twice. Returns `this`.
pub fn fn_0042f7d0(e: &mut Engine, this: Ptr, capacity: u32) -> Ptr {
    e.mem
        .set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
    e.call(ARRAY_INIT, &args![this, capacity, capacity]);
    this
}

// Translated from 0042f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of the `BSSimpleArray<NavMeshPtr,1024>`: sets the
/// vtable, then runs `0042fcb0` with 0 and 0. Returns `this`.
pub fn fn_0042f800(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem
        .set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0042f830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray<NavMeshPtr,1024>`: sets its vtable, then
/// runs `0042f9c0` with 1.
pub fn fn_0042f830(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
    e.call(ARRAY_FREE, &args![this, 1u32]);
}

// Translated from 0042f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends an element to a `BSSimpleArray`: makes room (`00761540`, gives the
/// index), runs `0042fa60` on the new slot with 1, builds the element there
/// from `word` (`fn_0042f4c0`) and returns the index.
pub fn fn_0042f850(e: &mut Engine, this: Ptr<BSSimpleArray>, word: u32) -> u32 {
    let index = e.call(ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(
        ARRAY_ELEMENTS_PREPARE,
        &args![this, buffer + index * 4, 1u32],
    );
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    fn_0042f4c0(e, Ptr::new(buffer + index * 4), word);
    index
}

// Translated from 0042f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the element at `index` of a `BSSimpleArray` and decrements its
/// size. With `shrink` set and the array sparse enough (`006f3170`), the
/// buffer is rebuilt at the smaller capacity (`00869600`, allocated through
/// the vtable slot at +4): the elements before `index` are moved to it
/// (`0042fb60`), the element is released (`0042fb20`), the rest are moved
/// after `index` (`0042fb60`, with `iSize - 1` as the count, as the code
/// does), the old buffer is freed (`006a8500`) and the new one is installed.
/// Otherwise the element is released in place and the rest shifted down one
/// slot.
pub fn fn_0042f8a0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, shrink: u8) {
    let shrunk = shrink != 0 && e.call(ARRAY_CAN_SHRINK, &args![this]).u8() != 0;
    if shrunk {
        let capacity = e.call(ARRAY_SHRUNK_CAPACITY, &args![this]).u32();
        let new_buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(ARRAY_ELEMENTS_MOVE, &args![this, new_buffer, buffer, index]);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_ELEMENTS_RELEASE,
            &args![this, buffer + index * 4, 1u32],
        );
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let size = e.get(this, BSSimpleArray::iSize);
        e.call(
            ARRAY_ELEMENTS_MOVE,
            &args![
                this,
                new_buffer + index * 4,
                buffer + index * 4 + 4,
                size.wrapping_sub(1)
            ],
        );
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_ELEMENTS_RELEASE,
            &args![this, buffer + index * 4, 1u32],
        );
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let size = e.get(this, BSSimpleArray::iSize);
        e.call(
            ARRAY_ELEMENTS_MOVE,
            &args![
                this,
                buffer + index * 4,
                buffer + index * 4 + 4,
                size.wrapping_sub(index).wrapping_sub(1)
            ],
        );
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(1));
}

// Translated from 0042f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a `BSSimpleArray` of one-word smart pointers (the
/// `BSSimpleArray<NavMeshPtr,1024>`): with a buffer, destroys its `iSize`
/// elements (`fn_0042fb20`); with `free_buffer` set, frees the buffer
/// (`006a8500`) and zeroes `pBuffer` and `iReservedSize`; then `iSize` is 0.
/// An array without a buffer is left as it is.
pub fn fn_0042f9c0(e: &mut Engine, this: Ptr<BSSimpleArray>, free_buffer: u8) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    fn_0042fb20(e, this.cast(), buffer, size);
    if free_buffer != 0 {
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 0042fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of the one-word smart pointer: runs `fn_0042fd40` with
/// `source` (the address of the pointer to copy). Returns `this`.
pub fn fn_0042fa20(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    fn_0042fd40(e, this, source);
    this
}

// Translated from 0042fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the one-word smart pointer: runs `fn_0042fda0`.
pub fn fn_0042fa40(e: &mut Engine, this: Ptr) {
    fn_0042fda0(e, this);
}

// Translated from 0042fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds `count` one-word smart pointers in place from `address` on: for
/// each slot, `006e6da0` (placement `new`, which gives the slot back) and,
/// when its result is not null, the null constructor `fn_0042fb00`. The
/// array `this` is not used. The compiler's exception frame is not
/// translated.
pub fn fn_0042fa60(e: &mut Engine, _this: Ptr, address: u32, count: u32) {
    for index in 0..count {
        let slot = address.wrapping_add(index.wrapping_mul(4));
        let block = e.call(PLACEMENT_NEW, &args![4u32, slot]).u32();
        if block != 0 {
            fn_0042fb00(e, Ptr::new(block));
        }
    }
}

// Translated from 0042fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of the one-word smart pointer: `fn_0042fd70` with a
/// null object. Returns `this`.
pub fn fn_0042fb00(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0042fd70(e, this, 0);
    this
}

// Translated from 0042fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` one-word smart pointers from `address` on: for each,
/// `fn_0042ffa0` with flags 0 (the destructor without `operator delete`).
/// The array `this` is not used.
pub fn fn_0042fb20(e: &mut Engine, _this: Ptr, address: u32, count: u32) {
    for index in 0..count {
        fn_0042ffa0(e, Ptr::new(address.wrapping_add(index.wrapping_mul(4))), 0);
    }
}

// Translated from 0042fb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` one-word elements from `source` to `destination` (the
/// ranges may overlap): forwards, a word at a time with `memmove`
/// (`00ec7230`), when the destination is below the source; backwards when it
/// is above (the index counted down as a signed word, as the code does);
/// nothing when they are the same or `count` is 0. The array `this` is not
/// used.
pub fn fn_0042fb60(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        for index in 0..count {
            let offset = index.wrapping_mul(4);
            e.call(
                MEMORY_MOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    4u32
                ],
            );
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(4);
            e.call(
                MEMORY_MOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    4u32
                ],
            );
            index -= 1;
        }
    }
}

// Translated from 0042fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the buffer of a `BSSimpleArray` to `new_capacity` words (the
/// caller stores `iReservedSize`, except for a first allocation). Without a
/// buffer: allocates one through the vtable slot at +4 and records the
/// capacity. When `count` is the current capacity: reallocates through the
/// vtable slot at +0x0C (old buffer, new capacity). Otherwise allocates a new
/// buffer (slot +4), moves `count` words to it (`fn_0042fb60`), frees the old
/// buffer (`006a8500`) and installs the new one.
pub fn fn_0042fc00(e: &mut Engine, this: Ptr<BSSimpleArray>, new_capacity: u32, count: u32) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let new_buffer = e.vcall(this.addr(), 4, &args![new_capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, new_capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let new_buffer = e
            .vcall(this.addr(), 0x0c, &args![buffer, new_capacity])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
    } else {
        let new_buffer = e.vcall(this.addr(), 4, &args![new_capacity]).u32();
        let old_buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_0042fb60(e, this.cast(), new_buffer, old_buffer, count);
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
    }
}

// Translated from 0042fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor body of the `BSSimpleArray<NavMeshPtr,1024>`: zeroes
/// `pBuffer`, `iSize` and `iReservedSize`; the capacity is the larger of
/// `capacity` and `size`. A nonzero capacity allocates the buffer through the
/// vtable slot at +4; a nonzero `size` builds that many null elements
/// (`fn_0042fa60`) and sets `iSize`.
pub fn fn_0042fcb0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(size);
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_0042fa60(e, this.cast(), buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0042fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of the one-word smart pointer (the `NavMeshPtr`):
/// stores the pointer held at `source` and, when it is not null, takes a
/// reference on the object (`0040f6e0` on the object's address + 0x1C).
/// Returns `this`.
pub fn fn_0042fd40(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let object = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), object);
    if object != 0 {
        e.call(
            OBJECT_REFERENCE_ADD,
            &args![object.wrapping_add(REFERENCE_COUNT_OFFSET)],
        );
    }
    this
}

// Translated from 0042fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the one-word smart pointer from an object: stores
/// `object` and, when it is not null, takes a reference on it (`0040f6e0`
/// on the object's address + 0x1C). Returns `this`.
pub fn fn_0042fd70(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(this.addr(), object);
    if object != 0 {
        e.call(
            OBJECT_REFERENCE_ADD,
            &args![object.wrapping_add(REFERENCE_COUNT_OFFSET)],
        );
    }
    this
}

// Translated from 0042fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the one-word smart pointer: when the object is not
/// null, releases a reference (`00401970` on the object's address + 0x1C).
pub fn fn_0042fda0(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(
            OBJECT_REFERENCE_RELEASE,
            &args![object.wrapping_add(REFERENCE_COUNT_OFFSET)],
        );
    }
}

// Translated from 0042fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of the one-word smart pointer from the pointer held at
/// `source` (the address of another smart pointer): when the two differ,
/// releases the old object (`00401970`), stores the new one and takes a
/// reference on it (`0040f6e0`, both on the object's address + 0x1C, when not
/// null). Returns `this`.
pub fn fn_0042fdc0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let held = e.mem.u32(this.addr());
    let object = e.mem.u32(source.addr());
    if held != object {
        if held != 0 {
            e.call(
                OBJECT_REFERENCE_RELEASE,
                &args![held.wrapping_add(REFERENCE_COUNT_OFFSET)],
            );
        }
        e.mem.set_u32(this.addr(), object);
        if object != 0 {
            e.call(
                OBJECT_REFERENCE_ADD,
                &args![object.wrapping_add(REFERENCE_COUNT_OFFSET)],
            );
        }
    }
    this
}

// Translated from 0042fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor body of the `NiTMapBase<..., TESObjectREFR *, bool>`: sets
/// its vtable (`01015a1c`), the hash size (+4) and a zero count (+0x0C),
/// allocates the table of `hash_size` words (`00aa1070`, stored at +8) and
/// zeroes it (`memset`, `00403d30`). Returns `this`.
pub fn fn_0042fe10(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP_BASE_REFR_BOOL);
    e.mem.set_u32(this.addr() + 4, hash_size);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    let bytes = e.mem.u32(this.addr() + 4).wrapping_shl(2);
    let table = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, table);
    let table = e.mem.u32(this.addr() + 8);
    let bytes = e.mem.u32(this.addr() + 4).wrapping_shl(2);
    e.call(MEMORY_SET, &args![table, 0u32, bytes]);
    this
}

// Translated from 0042fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTMap<TESObjectREFR *, bool>` (the engine map
/// names it after a folded `ctype<char>` destructor): sets its vtable
/// (`010159e8`), runs `00438af0` (it empties the map), then the base
/// destructor `fn_0042fee0`. The compiler's exception frame is not
/// translated.
pub fn fn_0042fe80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP_REFR_BOOL);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_0042fee0(e, this);
}

// Translated from 0042fee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTMapBase<..., TESObjectREFR *, bool>`: sets
/// its vtable (`01015a1c`), runs `00438af0` (it empties the map) and frees
/// the table at +8 (`00aa10f0`).
pub fn fn_0042fee0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP_BASE_REFR_BOOL);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(TABLE_FREE, &args![table]);
}

// Translated from 0042ff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl wrapper of `fn_0042ff30`: `first` is `this`, `second` the other
/// pointer. Returns its result.
pub fn fn_0042ff10(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    fn_0042ff30(e, first, second)
}

// Translated from 0042ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares two one-word smart pointers through `004b0460` (`this` = the
/// first, then the address of the second; it gives whether the words they
/// hold are equal). Returns its result.
pub fn fn_0042ff30(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.call(POINTER_EQUAL, &args![this, other]).u32()
}

// Translated from 0042ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESBoundObject *,1024>::scalar deleting destructor` (Xbox
/// PDB): the destructor `fn_0042ff80`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_simple_array_tes_bound_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0042ff80(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0042ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `BSSimpleArray<TESBoundObject *,1024>`: sets its
/// vtable (`01015a08`), then runs `008454f0` with `shrink` 1 (it sets the
/// size to 0 and frees the buffer).
pub fn fn_0042ff80(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY_BOUND_OBJECT);
    e.call(ARRAY_SET_SIZE_ZERO, &args![this, 1u32]);
}

// Translated from 0042ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the one-word smart pointer: the destructor
/// `fn_0042fa40`, then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0042ffa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0042fa40(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0042ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<TESObjectREFR *,bool>>,TESObjectREFR *,bool>::
/// scalar deleting destructor` (Xbox PDB): the destructor `fn_0042fee0`, then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_tes_object_refr_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0042fee0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004300c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x10` extra data (`ExtraAnim`, RTTI of vtable
/// `01015b28`): the `BSExtraData` constructor (`0040ec80`) with the type, the
/// vtable, and `animation` stored at +0x0C. Returns `this`.
pub fn fn_004300c0(e: &mut Engine, this: Ptr, animation: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_ANIM as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_ANIM);
    e.mem.set_u32(this.addr() + 0x0c, animation);
    this
}

// Translated from 00555bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `ExtraDataList::GetSeenData` (Xbox PDB name; the body
/// works on an object that holds the list at +0x28): takes the list from
/// `004610d0` (`this` + 0x28) and gives the word at +0x0C of its type 5 extra
/// data, or 0 (`00421910`).
pub fn fn_00555bc0(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.call(LIST_AT_OFFSET_0X28, &args![this]).u32();
    e.call(EXTRA_WORD_SEEN_DATA, &args![list]).u32()
}

// Translated from 00569140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `ExtraDataList::GetLock` (Xbox PDB name; the body works
/// on an object that holds the list at +0x44): takes the list from `005d43c0`
/// (`this` + 0x44) and gives the word at +0x0C of its type `0x2A` extra data,
/// or 0 (`004182b0`).
pub fn fn_00569140(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.call(LIST_AT_OFFSET_0X44, &args![this]).u32();
    e.call(EXTRA_WORD_LOCK, &args![list]).u32()
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0042dd90, fn_0042dd90(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0042dde0,
            extra_data_list_set_hot_key(Ptr<ExtraDataList>, u8)
        ),
        entry!(
            0x0042de90,
            extra_data_list_get_hot_key(Ptr<ExtraDataList>) -> u8
        ),
        entry!(
            0x0042dec0,
            extra_data_list_remove_hot_key(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0042dee0,
            extra_data_list_set_info_general_topic(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0042df90, fn_0042df90(Ptr<ExtraDataList>, u32) -> u32),
        entry!(0x0042dff0, fn_0042dff0(Ptr) -> bool),
        entry!(0x0042e010, fn_0042e010(Ptr, u32)),
        entry!(0x0042e040, fn_0042e040(Ptr<ExtraDataList>)),
        entry!(
            0x0042e060,
            extra_data_list_set_talking_actor_extra(Ptr<ExtraDataList>, u32, u32)
        ),
        entry!(
            0x0042e110,
            extra_data_list_get_talking_actor_extra(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(0x0042e130, fn_0042e130(Ptr<ExtraDataList>)),
        entry!(0x0042e150, fn_0042e150(Ptr<ExtraDataList>, u32, u32)),
        entry!(0x0042e210, fn_0042e210(Ptr, u32, u32) -> Ptr),
        entry!(
            0x0042e250,
            extra_data_list_get_model_swap(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0042e280, fn_0042e280(Ptr<ExtraDataList>)),
        entry!(
            0x0042e2a0,
            extra_data_list_get_nav_mesh_portal(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(0x0042e2c0, fn_0042e2c0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0042e380,
            extra_data_list_set_weapon_mod_slot(Ptr<ExtraDataList>, u8)
        ),
        entry!(0x0042e450, fn_0042e450(Ptr, u8) -> Ptr),
        entry!(0x0042e480, fn_0042e480(Ptr, u8)),
        entry!(0x0042e4a0, fn_0042e4a0(Ptr<ExtraDataList>, u8)),
        entry!(
            0x0042e560,
            extra_data_list_get_weapon_mod_flags(Ptr<ExtraDataList>) -> u8
        ),
        entry!(
            0x0042e5a0,
            extra_data_list_set_is_modding(Ptr<ExtraDataList>, u8)
        ),
        entry!(0x0042e650, fn_0042e650(Ptr, u8) -> Ptr),
        entry!(0x0042e680, fn_0042e680(Ptr, u8)),
        entry!(0x0042e6a0, fn_0042e6a0(Ptr, u32) -> Ptr),
        entry!(0x0042e6d0, fn_0042e6d0(Ptr)),
        entry!(
            0x0042e6f0,
            extra_data_list_remove_is_modding(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0042e730,
            extra_data_list_remove_nav_mesh_portal(Ptr<ExtraDataList>)
        ),
        entry!(0x0042e760, fn_0042e760(Ptr<ExtraDataList>)),
        entry!(
            0x0042e800,
            fn_0042e800(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x0042e820,
            extra_data_list_add_dismemberment_extra(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x0042e8c0,
            extra_data_list_get_dismemberment_extra(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x0042e8e0,
            extra_data_list_remove_dismemberment_extra(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0042e910,
            fn_0042e910(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(0x0042e930, fn_0042e930(Ptr<ExtraDataList>, u32)),
        entry!(0x0042ea00, fn_0042ea00(Ptr, u32)),
        entry!(0x0042ea20, fn_0042ea20(Ptr<ExtraDataList>) -> u32),
        entry!(0x0042ea50, fn_0042ea50(Ptr<ExtraDataList>, u32)),
        entry!(0x0042eb10, fn_0042eb10(Ptr, u32) -> Ptr),
        entry!(
            0x0042eb40,
            extra_data_list_get_ammo(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x0042eb60,
            extra_data_list_set_ammo(Ptr<ExtraDataList>, u32, u32)
        ),
        entry!(0x0042ec30, fn_0042ec30(Ptr, u32, u32) -> Ptr),
        entry!(0x0042ec70, fn_0042ec70(Ptr<ExtraDataList>, u32)),
        entry!(0x0042ed30, fn_0042ed30(Ptr<ExtraDataList>) -> u32),
        entry!(0x0042ed70, fn_0042ed70(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0042edb0,
            extra_data_list_remove_say_to_info_extra(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0042ede0,
            extra_data_list_get_say_to_extra(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(0x0042ee00, fn_0042ee00(Ptr<ExtraDataList>, u32)),
        entry!(0x0042eec0, fn_0042eec0(Ptr<ExtraDataList>, u32)),
        entry!(0x0042eef0, fn_0042eef0(Ptr<ExtraDataList>) -> u32),
        entry!(0x0042ef20, fn_0042ef20(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0042efe0,
            fn_0042efe0(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(0x0042f000, fn_0042f000(Ptr<ExtraDataList>)),
        entry!(0x0042f030, fn_0042f030(Ptr<ExtraDataList>, u32, u8) -> u32),
        entry!(0x0042f130, fn_0042f130(Ptr<ExtraDataList>, u32) -> bool),
        entry!(0x0042f180, fn_0042f180(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0042f1d0,
            extra_data_list_q_water_zone_map(Ptr<ExtraDataList>) -> Ptr
        ),
        entry!(
            0x0042f200,
            extra_data_list_set_ignored_by_sandbox(Ptr<ExtraDataList>, u8)
        ),
        entry!(0x0042f2d0, fn_0042f2d0(Ptr<ExtraDataList>) -> bool),
        entry!(0x0042f300, fn_0042f300(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0042f3d0,
            extra_data_list_get_patrol_ref_can_be_used_by(Ptr<ExtraDataList>, u32) -> bool
        ),
        entry!(
            0x0042f420,
            extra_data_list_get_or_create_extra_follower_swim_breadcrumbs(
                Ptr<ExtraDataList>,
            )
                -> Ptr<BSExtraData>
        ),
        entry!(0x0042f4c0, fn_0042f4c0(Ptr, u32)),
        entry!(0x0042f4e0, fn_0042f4e0(Ptr, u32) -> Ptr),
        entry!(
            0x0042f510,
            bs_simple_array_nav_mesh_ptr_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0042f540,
            ni_t_map_tes_object_refr_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042f570, fn_0042f570(Ptr) -> Ptr),
        entry!(0x0042f5a0, fn_0042f5a0(u32, u32) -> u32),
        entry!(0x0042f5d0, fn_0042f5d0(u32, u32) -> u32),
        entry!(0x0042f5f0, fn_0042f5f0(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x0042f730, fn_0042f730(Ptr, u32) -> Ptr),
        entry!(0x0042f760, fn_0042f760(Ptr)),
        entry!(
            0x0042f780,
            ni_pointer_actor_cause_assign(Ptr, u32) -> Ptr
        ),
        entry!(0x0042f7d0, fn_0042f7d0(Ptr, u32) -> Ptr),
        entry!(0x0042f800, fn_0042f800(Ptr) -> Ptr),
        entry!(0x0042f830, fn_0042f830(Ptr)),
        entry!(0x0042f850, fn_0042f850(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x0042f8a0, fn_0042f8a0(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x0042f9c0, fn_0042f9c0(Ptr<BSSimpleArray>, u8)),
        entry!(0x0042fa20, fn_0042fa20(Ptr, Ptr) -> Ptr),
        entry!(0x0042fa40, fn_0042fa40(Ptr)),
        entry!(0x0042fa60, fn_0042fa60(Ptr, u32, u32)),
        entry!(0x0042fb00, fn_0042fb00(Ptr) -> Ptr),
        entry!(0x0042fb20, fn_0042fb20(Ptr, u32, u32)),
        entry!(0x0042fb60, fn_0042fb60(Ptr, u32, u32, u32)),
        entry!(0x0042fc00, fn_0042fc00(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0042fcb0, fn_0042fcb0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0042fd40, fn_0042fd40(Ptr, Ptr) -> Ptr),
        entry!(0x0042fd70, fn_0042fd70(Ptr, u32) -> Ptr),
        entry!(0x0042fda0, fn_0042fda0(Ptr)),
        entry!(0x0042fdc0, fn_0042fdc0(Ptr, Ptr) -> Ptr),
        entry!(0x0042fe10, fn_0042fe10(Ptr, u32) -> Ptr),
        entry!(0x0042fe80, fn_0042fe80(Ptr)),
        entry!(0x0042fee0, fn_0042fee0(Ptr)),
        entry!(0x0042ff10, fn_0042ff10(Ptr, Ptr) -> u32),
        entry!(0x0042ff30, fn_0042ff30(Ptr, Ptr) -> u32),
        entry!(
            0x0042ff50,
            bs_simple_array_tes_bound_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0042ff80, fn_0042ff80(Ptr)),
        entry!(0x0042ffa0, fn_0042ffa0(Ptr, u32) -> Ptr),
        entry!(
            0x0042ffd0,
            ni_t_map_base_tes_object_refr_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004300c0, fn_004300c0(Ptr, u32) -> Ptr),
        entry!(0x00555bc0, fn_00555bc0(Ptr) -> u32),
        entry!(0x00569140, fn_00569140(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Test vtable of the extra data (slot 0 the scalar deleting destructor).
    const VTABLE: u32 = 0x0200_0000;
    const DESTRUCTOR: u32 = 0x0200_1000;
    // The list machinery the unit's main file calls (private there).
    const LOCK: u32 = 0x0040_fbf0;
    const UNLOCK: u32 = 0x0040_fba0;
    const GET_TYPE: u32 = 0x004f_1540;
    const GET_NEXT: u32 = 0x0044_ddc0;
    const SET_NEXT: u32 = 0x0040_3550;
    const MEMSET: u32 = 0x0040_3d30;
    /// `MOV EAX,[ECX+4]`: the head of the chain, where the by-type remover
    /// starts its walk.
    const LIST_HEAD: u32 = 0x0072_6070;

    /// The constructors of the extra data these setters build: address and
    /// type. Each double sets the test vtable, the type, a null next and the
    /// words it is given at +0x0C.
    const CONSTRUCTORS: [(u32, u8); 15] = [
        (EXTRA_HOT_KEY_INIT, EXTRA_HOT_KEY),
        (EXTRA_INFO_GENERAL_TOPIC_INIT, EXTRA_INFO_GENERAL_TOPIC),
        (EXTRA_TALKING_ACTOR_INIT, EXTRA_TALKING_ACTOR),
        (EXTRA_WEAPON_MOD_SLOTS_INIT, EXTRA_WEAPON_MOD_SLOTS),
        (EXTRA_FACTION_CHANGES_INIT, EXTRA_FACTION_CHANGES),
        (EXTRA_DISMEMBERED_LIMBS_INIT, EXTRA_DISMEMBERED_LIMBS),
        (EXTRA_ACTOR_CAUSE_INIT, EXTRA_ACTOR_CAUSE),
        (EXTRA_COMBAT_STYLE_INIT, EXTRA_COMBAT_STYLE),
        (EXTRA_SAY_TO_TOPIC_INFO_INIT, EXTRA_SAY_TO_TOPIC_INFO),
        (EXTRA_SAY_TO_TOPIC_INFO_INIT_EMPTY, EXTRA_SAY_TO_TOPIC_INFO),
        (
            EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_INIT,
            EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY,
        ),
        (EXTRA_WATER_ZONE_MAP_INIT, EXTRA_WATER_ZONE_MAP),
        (EXTRA_IGNORED_BY_SANDBOX_INIT, EXTRA_IGNORED_BY_SANDBOX),
        (
            EXTRA_PATROL_REF_IN_USE_DATA_INIT,
            EXTRA_PATROL_REF_IN_USE_DATA,
        ),
        (
            EXTRA_FOLLOWER_SWIM_BREADCRUMBS_INIT,
            EXTRA_FOLLOWER_SWIM_BREADCRUMBS,
        ),
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

    /// An engine with working doubles for the callees of the list
    /// operations (type and next accessors, lock, `memset`), `operator new`,
    /// the `BSExtraData` constructor, and the constructors of the extra data
    /// these functions build.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.put_vtable(VTABLE, &[DESTRUCTOR]);
        stub(&mut e, DESTRUCTOR);
        e.register(GET_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_NEXT, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(LIST_HEAD, |e, a| returns(e.mem.u32(a[0] + 4)));
        stub(&mut e, LOCK);
        stub(&mut e, UNLOCK);
        stub(&mut e, OPERATOR_DELETE);
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        e.register(BS_EXTRA_DATA_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        for (address, extra_type) in CONSTRUCTORS {
            e.register_double(address, move |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                e.mem.set_u8(a[0] + 4, extra_type);
                e.mem.set_u32(a[0] + 8, 0);
                for (index, word) in a[1..].iter().enumerate() {
                    e.mem.set_u32(a[0] + 0x0c + 4 * index as u32, *word);
                }
                returns(a[0])
            });
        }
        e
    }

    /// A list holding extra data of the given types with the given word at
    /// +0x0C, added through `AddExtra`.
    fn list_with(e: &mut Engine, items: &[(u8, u32)]) -> Ptr<ExtraDataList> {
        let list: Ptr<ExtraDataList> = e.new_object();
        for &(extra_type, word) in items {
            let extra: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x40));
            e.mem.set_u32(extra.addr(), VTABLE);
            e.set(extra, BSExtraData::cEtype, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, word);
            base_extra_list_add_extra(e, list.cast(), extra);
        }
        list
    }

    /// The types of the chain, in order.
    fn chain_types(e: &Engine, list: Ptr<ExtraDataList>) -> Vec<u8> {
        let mut types = vec![];
        let mut current: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
        while !current.is_null() {
            types.push(e.get(current, BSExtraData::cEtype));
            current = e.get(current, BSExtraData::pNext).cast();
        }
        types
    }

    fn payload_of(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
        let extra = find_extra(e, list, extra_type);
        assert!(!extra.is_null());
        e.mem.u32(extra.addr() + 0x0c)
    }

    /// Runs `call` with the call log on and returns the log.
    fn logged(e: &mut Engine, call: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        call(e);
        e.call_log.take().unwrap()
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The extra data the log shows deleted (the destructor called).
    fn deleted(log: &Log) -> Vec<u32> {
        calls_to(log, DESTRUCTOR).iter().map(|a| a[0]).collect()
    }

    /// The sizes passed to `operator new`.
    fn allocated_sizes(log: &Log) -> Vec<u32> {
        calls_to(log, OPERATOR_NEW).iter().map(|a| a[0]).collect()
    }

    /// Doubles for the callees of `fn_0042dd90`: three removers, the flags
    /// word copy and the bit test.
    fn flag_engine() -> Engine {
        let mut e = engine();
        stub(&mut e, REMOVE_SAVED_ANIMATION);
        stub(&mut e, REMOVE_SAVED_HAVOK_DATA);
        stub(&mut e, REMOVE_LAST_FINISHED_SEQUENCE);
        e.register(COPY_FLAGS_WORD, |e, a| {
            let word = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[1], word);
            returns(a[1])
        });
        e.register(FLAGS_TEST, |e, a| {
            returns((e.mem.u32(a[0]) & a[1] != 0) as u32)
        });
        e
    }

    // ---- 0042dd90

    #[test]
    fn fn_0042dd90_removes_used_markers_when_the_form_flag_is_set() {
        let mut e = flag_engine();
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0x2c, 0x8000_0001);
        let list = list_with(&mut e, &[(EXTRA_USED_MARKERS, 0), (0x30, 0)]);
        let extra = find_extra(&mut e, list, EXTRA_USED_MARKERS);
        let log = logged(&mut e, |e| {
            e.call(0x0042dd90, &args![list, form]);
        });
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            &order[..6],
            &[
                0x0042dd90,
                REMOVE_SAVED_ANIMATION,
                REMOVE_SAVED_HAVOK_DATA,
                REMOVE_LAST_FINISHED_SEQUENCE,
                COPY_FLAGS_WORD,
                FLAGS_TEST
            ]
        );
        assert_eq!(calls_to(&log, FLAGS_TEST)[0][1], 0x8000_0000);
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert_eq!(chain_types(&e, list), vec![0x30]);
    }

    #[test]
    fn fn_0042dd90_keeps_used_markers_when_the_form_flag_is_clear() {
        let mut e = flag_engine();
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0x2c, 0x7fff_ffff);
        let list = list_with(&mut e, &[(EXTRA_USED_MARKERS, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0042dd90, &args![list, form]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![EXTRA_USED_MARKERS]);
    }

    // ---- hot key

    #[test]
    fn set_hot_key_builds_the_extra_data_or_stores_the_byte() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042dde0, &args![list, 0x7fu32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(calls_to(&log, EXTRA_HOT_KEY_INIT)[0][1], 0x7f);
        assert_eq!(chain_types(&e, list), vec![EXTRA_HOT_KEY]);
        assert_eq!(payload_of(&mut e, list, EXTRA_HOT_KEY) & 0xff, 0x7f);
        let log = logged(&mut e, |e| {
            e.call(0x0042dde0, &args![list, 3u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(payload_of(&mut e, list, EXTRA_HOT_KEY) & 0xff, 3);
        assert_eq!(chain_types(&e, list), vec![EXTRA_HOT_KEY]);
    }

    #[test]
    fn get_hot_key_gives_the_byte_or_ff() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_HOT_KEY, 0x1234_5605)]);
        assert_eq!(e.call(0x0042de90, &args![list]).u8(), 0x05);
        let empty = list_with(&mut e, &[(0x30, 1)]);
        assert_eq!(e.call(0x0042de90, &args![empty]).u8(), 0xff);
    }

    #[test]
    fn remove_hot_key_deletes_it_by_type() {
        let mut e = engine();
        let list = list_with(&mut e, &[(0x30, 0), (EXTRA_HOT_KEY, 5)]);
        let extra = find_extra(&mut e, list, EXTRA_HOT_KEY);
        let log = logged(&mut e, |e| {
            e.call(0x0042dec0, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert_eq!(chain_types(&e, list), vec![0x30]);
        let log = logged(&mut e, |e| {
            e.call(0x0042dec0, &args![list]);
        });
        assert!(deleted(&log).is_empty());
    }

    // ---- info general topic

    #[test]
    fn set_info_general_topic_builds_or_stores() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042dee0, &args![list, 0x1111u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(calls_to(&log, EXTRA_INFO_GENERAL_TOPIC_INIT)[0][1], 0x1111);
        assert_eq!(payload_of(&mut e, list, EXTRA_INFO_GENERAL_TOPIC), 0x1111);
        e.call(0x0042dee0, &args![list, 0x2222u32]);
        assert_eq!(payload_of(&mut e, list, EXTRA_INFO_GENERAL_TOPIC), 0x2222);
        assert_eq!(chain_types(&e, list), vec![EXTRA_INFO_GENERAL_TOPIC]);
    }

    #[test]
    fn fn_0042df90_forwards_only_an_empty_topic() {
        let mut e = engine();
        e.register(LIST_COUNT, |e, a| returns(e.mem.u32(a[0])));
        stub(&mut e, MENU_TOPIC_FORWARD);
        // No extra data.
        let none = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0042df90, &args![none, 9u32]).u32(), 0);
        // A null topic.
        let null_topic = list_with(&mut e, &[(EXTRA_INFO_GENERAL_TOPIC, 0)]);
        assert_eq!(e.call(0x0042df90, &args![null_topic, 9u32]).u32(), 0);
        // A topic with items: returned, not forwarded.
        let topic = e.mem.alloc(0x40);
        e.mem.set_u32(topic + 0x0c, 2);
        let busy = list_with(&mut e, &[(EXTRA_INFO_GENERAL_TOPIC, topic)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042df90, &args![busy, 9u32]).u32(), topic);
        });
        assert!(calls_to(&log, MENU_TOPIC_FORWARD).is_empty());
        // An empty topic: forwarded with the argument.
        let empty_topic = e.mem.alloc(0x40);
        e.mem.set_u32(empty_topic + 0x14, 0xa1);
        e.mem.set_u32(empty_topic + 0x28, 0xa2);
        e.mem.set_u32(empty_topic + 0x18, 0xa3);
        let ready = list_with(&mut e, &[(EXTRA_INFO_GENERAL_TOPIC, empty_topic)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042df90, &args![ready, 9u32]).u32(), empty_topic);
        });
        assert_eq!(
            calls_to(&log, MENU_TOPIC_FORWARD),
            vec![vec![empty_topic, 0xa1, 0xa2, 0xa3, 9]]
        );
    }

    #[test]
    fn fn_0042dff0_tests_the_list_count() {
        let mut e = engine();
        e.register(LIST_COUNT, |e, a| returns(e.mem.u32(a[0])));
        let topic = e.mem.alloc(0x40);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0042dff0, &args![Ptr::<()>::new(topic)]).bool());
        });
        assert_eq!(calls_to(&log, LIST_COUNT), vec![vec![topic + 0x0c]]);
        e.mem.set_u32(topic + 0x0c, 3);
        assert!(!e.call(0x0042dff0, &args![Ptr::<()>::new(topic)]).bool());
    }

    #[test]
    fn fn_0042e010_passes_three_fields_and_the_argument() {
        let mut e = engine();
        stub(&mut e, MENU_TOPIC_FORWARD);
        let topic = e.mem.alloc(0x40);
        e.mem.set_u32(topic + 0x14, 1);
        e.mem.set_u32(topic + 0x18, 3);
        e.mem.set_u32(topic + 0x28, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0042e010, &args![Ptr::<()>::new(topic), 4u32]);
        });
        assert_eq!(
            calls_to(&log, MENU_TOPIC_FORWARD),
            vec![vec![topic, 1, 2, 3, 4]]
        );
    }

    #[test]
    fn fn_0042e040_removes_the_info_general_topic() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_INFO_GENERAL_TOPIC, 1), (0x30, 0)]);
        e.call(0x0042e040, &args![list]);
        assert_eq!(chain_types(&e, list), vec![0x30]);
    }

    // ---- talking actor

    #[test]
    fn set_talking_actor_extra_replaces_an_existing_one() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e060, &args![list, 0x10u32, 0x20u32]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(
            calls_to(&log, EXTRA_TALKING_ACTOR_INIT)[0][1..],
            [0x10, 0x20]
        );
        let old = find_extra(&mut e, list, EXTRA_TALKING_ACTOR);
        let log = logged(&mut e, |e| {
            e.call(0x0042e060, &args![list, 0x30u32, 0x40u32]);
        });
        assert_eq!(deleted(&log), vec![old.addr()]);
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(chain_types(&e, list), vec![EXTRA_TALKING_ACTOR]);
        assert_eq!(payload_of(&mut e, list, EXTRA_TALKING_ACTOR), 0x30);
    }

    #[test]
    fn get_talking_actor_extra_and_its_remover() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_TALKING_ACTOR, 5)]);
        let extra = e.call(0x0042e110, &args![list]).ptr::<BSExtraData>();
        assert!(!extra.is_null());
        e.call(0x0042e130, &args![list]);
        assert!(e
            .call(0x0042e110, &args![list])
            .ptr::<BSExtraData>()
            .is_null());
    }

    // ---- model swap

    #[test]
    fn model_swap_setter_getter_and_remover() {
        let mut e = engine();
        // The vtable the constructor installs, for the remover's delete.
        e.put_vtable(VTABLE_EXTRA_MODEL_SWAP, &[DESTRUCTOR]);
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e150, &args![list, 0xa0u32, 0xb0u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x14]);
        let extra = find_extra(&mut e, list, EXTRA_MODEL_SWAP);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_MODEL_SWAP);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0xa0);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0xb0);
        assert_eq!(e.call(0x0042e250, &args![list]).u32(), 0xa0);
        let log = logged(&mut e, |e| {
            e.call(0x0042e150, &args![list, 0xc0u32, 0xd0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0xc0);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0xd0);
        e.call(0x0042e280, &args![list]);
        assert_eq!(e.call(0x0042e250, &args![list]).u32(), 0);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn fn_0042e210_builds_the_model_swap_extra_data() {
        let mut e = engine();
        let block = e.mem.alloc(0x14);
        let result = e
            .call(0x0042e210, &args![Ptr::<()>::new(block), 7u32, 8u32])
            .u32();
        assert_eq!(result, block);
        assert_eq!(e.mem.u8(block + 4), EXTRA_MODEL_SWAP);
        assert_eq!(e.mem.u32(block), 0x0101_5980);
        assert_eq!(e.mem.u32(block + 0x0c), 7);
        assert_eq!(e.mem.u32(block + 0x10), 8);
    }

    #[test]
    fn model_swap_getter_gives_zero_without_the_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[(0x30, 1)]);
        assert_eq!(e.call(0x0042e250, &args![list]).u32(), 0);
    }

    // ---- navmesh portal

    #[test]
    fn fn_0042e2c0_copies_the_portal_record_into_a_new_extra_data() {
        let mut e = engine();
        e.register(NAVMESH_PORTAL_COPY, |e, a| {
            let (low, high) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], low);
            e.mem.set_u32(a[0] + 4, high);
            Ret::default()
        });
        let portal = e.mem.alloc(0x40);
        e.mem.set_u32(portal + 0x0c, 0x1234_5678);
        e.mem.set_u32(portal + 0x10, 0x0000_9abc);
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e2c0, &args![list, portal]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x14]);
        let extra = find_extra(&mut e, list, EXTRA_NAVMESH_PORTAL);
        assert!(!extra.is_null());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x1234_5678);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x0000_9abc);
        // An existing one is left alone.
        e.mem.set_u32(portal + 0x0c, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042e2c0, &args![list, portal]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x1234_5678);
    }

    #[test]
    fn nav_mesh_portal_getter_and_remover() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_NAVMESH_PORTAL, 1), (0x30, 0)]);
        let extra = find_extra(&mut e, list, EXTRA_NAVMESH_PORTAL);
        assert_eq!(e.call(0x0042e2a0, &args![list]).ptr::<BSExtraData>(), extra);
        let log = logged(&mut e, |e| {
            e.call(0x0042e730, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(e
            .call(0x0042e2a0, &args![list])
            .ptr::<BSExtraData>()
            .is_null());
        let log = logged(&mut e, |e| {
            e.call(0x0042e730, &args![list]);
        });
        assert!(deleted(&log).is_empty());
    }

    // ---- weapon mod slots

    #[test]
    fn set_weapon_mod_slot_builds_with_a_nonzero_slot() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e380, &args![list, 0x04u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        let extra = find_extra(&mut e, list, EXTRA_WEAPON_MOD_SLOTS);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_WEAPON_MOD_SLOTS);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 0x04);
    }

    #[test]
    fn set_weapon_mod_slot_ors_into_an_existing_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_WEAPON_MOD_SLOTS, 0x01)]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e380, &args![list, 0x04u32]);
            e.call(0x0042e380, &args![list, 0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(
            payload_of(&mut e, list, EXTRA_WEAPON_MOD_SLOTS) & 0xff,
            0x05
        );
    }

    #[test]
    #[should_panic]
    fn set_weapon_mod_slot_with_slot_zero_and_no_extra_data_reads_null() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        e.call(0x0042e380, &args![list, 0u32]);
    }

    #[test]
    fn fn_0042e450_builds_the_weapon_mod_flags_extra_data() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        let result = e
            .call(0x0042e450, &args![Ptr::<()>::new(block), 0x82u32])
            .u32();
        assert_eq!(result, block);
        assert_eq!(e.mem.u8(block + 4), EXTRA_WEAPON_MOD_SLOTS);
        assert_eq!(e.mem.u32(block), 0x0101_59a4);
        assert_eq!(e.mem.u8(block + 0x0c), 0x82);
    }

    #[test]
    fn fn_0042e480_ors_the_bits() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        e.mem.set_u8(block + 0x0c, 0x11);
        e.call(0x0042e480, &args![Ptr::<()>::new(block), 0x30u32]);
        assert_eq!(e.mem.u8(block + 0x0c), 0x31);
    }

    #[test]
    fn fn_0042e4a0_sets_the_byte_or_builds_the_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e4a0, &args![list, 0x21u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(calls_to(&log, EXTRA_WEAPON_MOD_SLOTS_INIT).len(), 1);
        assert_eq!(
            payload_of(&mut e, list, EXTRA_WEAPON_MOD_SLOTS) & 0xff,
            0x21
        );
        // An existing byte is replaced, not ORed.
        e.call(0x0042e4a0, &args![list, 0x02u32]);
        assert_eq!(
            payload_of(&mut e, list, EXTRA_WEAPON_MOD_SLOTS) & 0xff,
            0x02
        );
        assert_eq!(chain_types(&e, list), vec![EXTRA_WEAPON_MOD_SLOTS]);
    }

    #[test]
    fn get_weapon_mod_flags_reads_through_the_helper() {
        let mut e = engine();
        e.register(WEAPON_MOD_FLAGS_READ, |e, a| {
            returns(e.mem.u8(a[0] + 0x0c) as u32)
        });
        let list = list_with(&mut e, &[(EXTRA_WEAPON_MOD_SLOTS, 0x0a)]);
        assert_eq!(e.call(0x0042e560, &args![list]).u8(), 0x0a);
        let empty = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042e560, &args![empty]).u8(), 0);
        });
        assert!(calls_to(&log, WEAPON_MOD_FLAGS_READ).is_empty());
    }

    // ---- is modding

    #[test]
    fn set_is_modding_builds_or_stores_the_byte() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e5a0, &args![list, 1u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        let extra = find_extra(&mut e, list, EXTRA_WEAPON_IS_MODDING);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_WEAPON_IS_MODDING);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 1);
        let log = logged(&mut e, |e| {
            e.call(0x0042e5a0, &args![list, 0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 0);
    }

    #[test]
    fn fn_0042e650_builds_the_is_modding_extra_data() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        let result = e
            .call(0x0042e650, &args![Ptr::<()>::new(block), 1u32])
            .u32();
        assert_eq!(result, block);
        assert_eq!(e.mem.u8(block + 4), EXTRA_WEAPON_IS_MODDING);
        assert_eq!(e.mem.u32(block), 0x0101_59bc);
        assert_eq!(e.mem.u8(block + 0x0c), 1);
    }

    #[test]
    fn fn_0042e680_stores_the_byte() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 0x0c, 0xffff_ffff);
        e.call(0x0042e680, &args![Ptr::<()>::new(block), 0x12u32]);
        assert_eq!(e.mem.u32(block + 0x0c), 0xffff_ff12);
    }

    #[test]
    fn is_modding_scalar_deleting_destructor_deletes_only_on_bit_zero() {
        let mut e = engine();
        stub(&mut e, BS_EXTRA_DATA_DESTROY);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042e6a0, &args![Ptr::<()>::new(block), 0u32]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY), vec![vec![block]]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(e.mem.u32(block), VTABLE_EXTRA_WEAPON_IS_MODDING);
        let log = logged(&mut e, |e| {
            e.call(0x0042e6a0, &args![Ptr::<()>::new(block), 3u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block]]);
    }

    #[test]
    fn fn_0042e6d0_sets_the_vtable_and_runs_the_base_destructor() {
        let mut e = engine();
        stub(&mut e, BS_EXTRA_DATA_DESTROY);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0042e6d0, &args![Ptr::<()>::new(block)]);
        });
        assert_eq!(e.mem.u32(block), 0x0101_59bc);
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY), vec![vec![block]]);
    }

    #[test]
    fn remove_is_modding_deletes_if_present() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_WEAPON_IS_MODDING, 1)]);
        let extra = find_extra(&mut e, list, EXTRA_WEAPON_IS_MODDING);
        let log = logged(&mut e, |e| {
            e.call(0x0042e6f0, &args![list]);
            e.call(0x0042e6f0, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    // ---- faction changes (0x5E)

    #[test]
    fn fn_0042e760_adds_the_type_5e_extra_data_once() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042e760, &args![list]);
            e.call(0x0042e760, &args![list]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(chain_types(&e, list), vec![EXTRA_FACTION_CHANGES]);
        let found = e.call(0x0042e800, &args![list]).ptr::<BSExtraData>();
        assert_eq!(found, find_extra(&mut e, list, EXTRA_FACTION_CHANGES));
        let empty = list_with(&mut e, &[]);
        assert!(e
            .call(0x0042e800, &args![empty])
            .ptr::<BSExtraData>()
            .is_null());
    }

    // ---- dismembered limbs

    #[test]
    fn dismemberment_extra_is_added_returned_and_removed() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        assert!(e
            .call(0x0042e8c0, &args![list])
            .ptr::<BSExtraData>()
            .is_null());
        let log = logged(&mut e, |e| {
            e.call(0x0042e820, &args![list]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x30]);
        let extra = find_extra(&mut e, list, EXTRA_DISMEMBERED_LIMBS);
        assert!(!extra.is_null());
        let log = logged(&mut e, |e| {
            let again = e.call(0x0042e820, &args![list]).ptr::<BSExtraData>();
            assert_eq!(again, extra);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(e.call(0x0042e8c0, &args![list]).ptr::<BSExtraData>(), extra);
        let log = logged(&mut e, |e| {
            e.call(0x0042e8e0, &args![list]);
            e.call(0x0042e8e0, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn add_dismemberment_extra_returns_the_new_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let returned = e.call(0x0042e820, &args![list]).ptr::<BSExtraData>();
        assert_eq!(returned, find_extra(&mut e, list, EXTRA_DISMEMBERED_LIMBS));
    }

    // ---- actor cause

    #[test]
    fn fn_0042e910_gets_the_actor_cause_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_ACTOR_CAUSE, 1)]);
        assert_eq!(
            e.call(0x0042e910, &args![list]).ptr::<BSExtraData>(),
            find_extra(&mut e, list, EXTRA_ACTOR_CAUSE)
        );
        let empty = list_with(&mut e, &[]);
        assert!(e
            .call(0x0042e910, &args![empty])
            .ptr::<BSExtraData>()
            .is_null());
    }

    #[test]
    fn fn_0042e930_assigns_builds_or_removes() {
        let mut e = engine();
        stub(&mut e, ACTOR_CAUSE_POINTER_ASSIGN);
        let list = list_with(&mut e, &[]);
        // Null with no extra data: nothing happens.
        let log = logged(&mut e, |e| {
            e.call(0x0042e930, &args![list, 0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        // Non-null with none: built (0x10 bytes), assigned, added.
        let log = logged(&mut e, |e| {
            e.call(0x0042e930, &args![list, 0xcau32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        let extra = find_extra(&mut e, list, EXTRA_ACTOR_CAUSE);
        assert_eq!(
            calls_to(&log, ACTOR_CAUSE_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x0c, 0xca]]
        );
        // Non-null with one: assigned only.
        let log = logged(&mut e, |e| {
            e.call(0x0042e930, &args![list, 0xcbu32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(
            calls_to(&log, ACTOR_CAUSE_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x0c, 0xcb]]
        );
        // Null with one: removed.
        let log = logged(&mut e, |e| {
            e.call(0x0042e930, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn fn_0042ea00_assigns_the_smart_pointer_at_0c() {
        let mut e = engine();
        stub(&mut e, ACTOR_CAUSE_POINTER_ASSIGN);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0042ea00, &args![Ptr::<()>::new(block), 0x77u32]);
        });
        assert_eq!(
            calls_to(&log, ACTOR_CAUSE_POINTER_ASSIGN),
            vec![vec![block + 0x0c, 0x77]]
        );
    }

    // ---- combat style

    #[test]
    fn fn_0042ea20_gets_the_combat_style() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_COMBAT_STYLE, 0xabcd)]);
        assert_eq!(e.call(0x0042ea20, &args![list]).u32(), 0xabcd);
        let empty = list_with(&mut e, &[(0x30, 5)]);
        assert_eq!(e.call(0x0042ea20, &args![empty]).u32(), 0);
    }

    #[test]
    fn fn_0042ea50_stores_builds_or_removes() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        // Null with none: nothing.
        e.call(0x0042ea50, &args![list, 0u32]);
        assert!(chain_types(&e, list).is_empty());
        // Non-null with none: built with the value.
        let log = logged(&mut e, |e| {
            e.call(0x0042ea50, &args![list, 0x51u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(calls_to(&log, EXTRA_COMBAT_STYLE_INIT)[0][1], 0x51);
        // Non-null with one: stored.
        e.call(0x0042ea50, &args![list, 0x52u32]);
        assert_eq!(payload_of(&mut e, list, EXTRA_COMBAT_STYLE), 0x52);
        assert_eq!(chain_types(&e, list), vec![EXTRA_COMBAT_STYLE]);
        // Null with one: removed.
        let extra = find_extra(&mut e, list, EXTRA_COMBAT_STYLE);
        let log = logged(&mut e, |e| {
            e.call(0x0042ea50, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    // ---- second session: doubles and fixtures

    /// Test vtable of the arrays (slot 0 the destructor, slot 1 `+4` the
    /// buffer allocator).
    const ARRAY_VTABLE: u32 = 0x0200_2000;
    const ARRAY_ALLOCATOR: u32 = 0x0200_2100;

    /// An engine for the array functions: the vtable's allocator hands out a
    /// zeroed block of `capacity * 4` bytes.
    fn array_engine() -> Engine {
        let mut e = engine();
        e.put_vtable(ARRAY_VTABLE, &[DESTRUCTOR, ARRAY_ALLOCATOR]);
        e.register(ARRAY_ALLOCATOR, |e, a| returns(e.mem.alloc(a[1] * 4)));
        e
    }

    /// An array object (`BSSimpleArray`) with the given buffer, size and
    /// reserved size.
    fn array_with(e: &mut Engine, buffer: u32, size: u32, reserved: u32) -> Ptr<BSSimpleArray> {
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), ARRAY_VTABLE);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        e.set(array, BSSimpleArray::iReservedSize, reserved);
        array
    }

    fn array_fields(e: &Engine, array: Ptr<BSSimpleArray>) -> (u32, u32, u32) {
        (
            e.get(array, BSSimpleArray::pBuffer),
            e.get(array, BSSimpleArray::iSize),
            e.get(array, BSSimpleArray::iReservedSize),
        )
    }

    // ---- 0042eb10

    #[test]
    fn fn_0042eb10_builds_the_combat_style_extra_data() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let result = fn_0042eb10(&mut e, Ptr::new(block), 0x51);
        let log = e.call_log.take().unwrap();
        assert_eq!(result.addr(), block);
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_INIT), vec![vec![block, 0x69]]);
        assert_eq!(e.mem.u32(block), VTABLE_EXTRA_COMBAT_STYLE);
        assert_eq!(e.mem.u32(block + 0x0c), 0x51);
        assert_eq!(e.mem.u8(block + 4), 0x69);
    }

    // ---- ammo

    #[test]
    fn get_ammo_gives_the_type_6e_extra_data_or_null() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_AMMO, 1)]);
        assert_eq!(
            e.call(0x0042eb40, &args![list]).ptr::<BSExtraData>(),
            find_extra(&mut e, list, EXTRA_AMMO)
        );
        let empty = list_with(&mut e, &[(0x30, 1)]);
        assert!(e
            .call(0x0042eb40, &args![empty])
            .ptr::<BSExtraData>()
            .is_null());
    }

    #[test]
    fn set_ammo_builds_stores_or_removes() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        // Nothing to remove, nothing built.
        let log = logged(&mut e, |e| {
            e.call(0x0042eb60, &args![list, 0u32, 3u32]);
            e.call(0x0042eb60, &args![list, 0x70u32, 0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        // Both given, none in the list: built (0x14 bytes) and added.
        let log = logged(&mut e, |e| {
            e.call(0x0042eb60, &args![list, 0x70u32, 3u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x14]);
        assert_eq!(chain_types(&e, list), vec![EXTRA_AMMO]);
        let extra = find_extra(&mut e, list, EXTRA_AMMO);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x70);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 3);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_AMMO);
        // One in the list: both words stored, nothing built.
        let log = logged(&mut e, |e| {
            e.call(0x0042eb60, &args![list, 0x71u32, 9u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x71);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 9);
    }

    #[test]
    fn set_ammo_removes_the_extra_data_for_a_null_ammo_or_zero_count() {
        let mut e = engine();
        for (ammo, count) in [(0u32, 5u32), (0x70, 0)] {
            let list = list_with(&mut e, &[(0x30, 0), (EXTRA_AMMO, 0x70)]);
            let extra = find_extra(&mut e, list, EXTRA_AMMO);
            let log = logged(&mut e, |e| {
                e.call(0x0042eb60, &args![list, ammo, count]);
            });
            assert_eq!(deleted(&log), vec![extra.addr()]);
            assert_eq!(chain_types(&e, list), vec![0x30]);
        }
    }

    #[test]
    fn fn_0042ec30_builds_the_ammo_extra_data() {
        let mut e = engine();
        let block = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        let result = e.call(0x0042ec30, &args![Ptr::<()>::new(block), 0x70u32, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(result.u32(), block);
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_INIT), vec![vec![block, 0x6e]]);
        assert_eq!(e.mem.u32(block), VTABLE_EXTRA_AMMO);
        assert_eq!(e.mem.u32(block + 0x0c), 0x70);
        assert_eq!(e.mem.u32(block + 0x10), 4);
    }

    // ---- say to topic info (0x75)

    #[test]
    fn fn_0042ec70_stores_builds_or_removes_the_info() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        // Null with none: nothing happens.
        e.call(0x0042ec70, &args![list, 0u32]);
        assert!(chain_types(&e, list).is_empty());
        // Non-null with none: built (0x1C bytes) with the info.
        let log = logged(&mut e, |e| {
            e.call(0x0042ec70, &args![list, 0x91u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x1c]);
        assert_eq!(calls_to(&log, EXTRA_SAY_TO_TOPIC_INFO_INIT)[0][1], 0x91);
        assert_eq!(payload_of(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO), 0x91);
        // Non-null with one: stored.
        let log = logged(&mut e, |e| {
            e.call(0x0042ec70, &args![list, 0x92u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(payload_of(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO), 0x92);
        // Null with one: removed.
        let extra = find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO);
        let log = logged(&mut e, |e| {
            e.call(0x0042ec70, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn fn_0042ed30_gives_the_topic_at_10() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TO_TOPIC_INFO, 1)]);
        let extra = find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO);
        e.mem.set_u32(extra.addr() + 0x10, 0x4242);
        assert_eq!(e.call(0x0042ed30, &args![list]).u32(), 0x4242);
        let empty = list_with(&mut e, &[(0x30, 5)]);
        assert_eq!(e.call(0x0042ed30, &args![empty]).u32(), 0);
    }

    #[test]
    fn fn_0042ed70_gives_the_info_at_0c() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TO_TOPIC_INFO, 0x77)]);
        assert_eq!(e.call(0x0042ed70, &args![list]).u32(), 0x77);
        let empty = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0042ed70, &args![empty]).u32(), 0);
    }

    #[test]
    fn remove_say_to_info_extra_deletes_if_present() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TO_TOPIC_INFO, 1)]);
        let extra = find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO);
        let log = logged(&mut e, |e| {
            e.call(0x0042edb0, &args![list]);
            e.call(0x0042edb0, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn get_say_to_extra_gives_the_extra_data_or_null() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TO_TOPIC_INFO, 1)]);
        assert_eq!(
            e.call(0x0042ede0, &args![list]).ptr::<BSExtraData>(),
            find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO)
        );
        let empty = list_with(&mut e, &[]);
        assert!(e
            .call(0x0042ede0, &args![empty])
            .ptr::<BSExtraData>()
            .is_null());
    }

    #[test]
    fn fn_0042ee00_sets_the_topic_on_a_new_or_existing_extra_data() {
        let mut e = engine();
        e.register(EXTRA_SAY_TO_TOPIC_INFO_SET_TOPIC, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        let list = list_with(&mut e, &[]);
        // Null with none: nothing.
        e.call(0x0042ee00, &args![list, 0u32]);
        assert!(chain_types(&e, list).is_empty());
        // Non-null with none: built with the empty constructor, pInfo zeroed,
        // added, topic stored.
        let log = logged(&mut e, |e| {
            e.call(0x0042ee00, &args![list, 0x5000u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x1c]);
        assert_eq!(calls_to(&log, EXTRA_SAY_TO_TOPIC_INFO_INIT_EMPTY).len(), 1);
        assert!(calls_to(&log, EXTRA_SAY_TO_TOPIC_INFO_INIT).is_empty());
        let extra = find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO);
        assert_eq!(
            calls_to(&log, EXTRA_SAY_TO_TOPIC_INFO_SET_TOPIC),
            vec![vec![extra.addr(), 0x5000]]
        );
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x5000);
        // Non-null with one: only the topic is stored.
        let log = logged(&mut e, |e| {
            e.call(0x0042ee00, &args![list, 0x5001u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x5001);
        // Null with one: removed.
        let log = logged(&mut e, |e| {
            e.call(0x0042ee00, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
    }

    #[test]
    fn fn_0042eec0_stores_the_response_id_when_present() {
        let mut e = engine();
        let empty = list_with(&mut e, &[]);
        e.call(0x0042eec0, &args![empty, 5u32]);
        assert!(chain_types(&e, empty).is_empty());
        let list = list_with(&mut e, &[(EXTRA_SAY_TO_TOPIC_INFO, 1)]);
        e.call(0x0042eec0, &args![list, 5u32]);
        let extra = find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO);
        assert_eq!(e.mem.u32(extra.addr() + 0x14), 5);
    }

    #[test]
    fn fn_0042eef0_gives_the_response_id() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TO_TOPIC_INFO, 1)]);
        let extra = find_extra(&mut e, list, EXTRA_SAY_TO_TOPIC_INFO);
        e.mem.set_u32(extra.addr() + 0x14, 0x33);
        assert_eq!(e.call(0x0042eef0, &args![list]).u32(), 0x33);
        let empty = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0042eef0, &args![empty]).u32(), 0);
    }

    // ---- say topic info once a day (0x73)

    #[test]
    fn fn_0042ef20_adds_builds_or_removes() {
        let mut e = engine();
        // The list routine stores the word it is handed the address of at
        // +0 of the list block.
        e.register(SAID_ONCE_LIST_ADD, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let list = list_with(&mut e, &[]);
        e.call(0x0042ef20, &args![list, 0u32]);
        assert!(chain_types(&e, list).is_empty());
        // None: built (0x10 bytes) with the value.
        let log = logged(&mut e, |e| {
            e.call(0x0042ef20, &args![list, 0x61u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(
            calls_to(&log, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_INIT)[0][1],
            0x61
        );
        assert_eq!(chain_types(&e, list), vec![EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY]);
        // One in the list: the list routine gets the word at +0x0C and the
        // address of a stack word holding the value.
        let block = e.mem.alloc(0x10);
        let extra = find_extra(&mut e, list, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY);
        e.mem.set_u32(extra.addr() + 0x0c, block);
        let log = logged(&mut e, |e| {
            e.call(0x0042ef20, &args![list, 0x62u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        let calls = calls_to(&log, SAID_ONCE_LIST_ADD);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], block);
        assert_eq!(e.mem.u32(block), 0x62);
        // Null with one: removed.
        let log = logged(&mut e, |e| {
            e.call(0x0042ef20, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
    }

    #[test]
    fn fn_0042efe0_gets_the_type_73_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY, 1)]);
        assert_eq!(
            e.call(0x0042efe0, &args![list]).ptr::<BSExtraData>(),
            find_extra(&mut e, list, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY)
        );
        let empty = list_with(&mut e, &[]);
        assert!(e
            .call(0x0042efe0, &args![empty])
            .ptr::<BSExtraData>()
            .is_null());
    }

    #[test]
    fn fn_0042f000_removes_the_type_73_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY, 1)]);
        let extra = find_extra(&mut e, list, EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY);
        let log = logged(&mut e, |e| {
            e.call(0x0042f000, &args![list]);
            e.call(0x0042f000, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    // ---- water zone map (0x7e)

    /// Doubles for `fn_0042f030`: the update stores the zone word in the
    /// map's first word, the map count reads the word at its address.
    fn water_engine() -> Engine {
        let mut e = engine();
        e.register(EXTRA_WATER_ZONE_MAP_UPDATE, |e, a| {
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            Ret::default()
        });
        e.register(MAP_COUNT, |e, a| returns(e.mem.u32(a[0])));
        e
    }

    #[test]
    fn fn_0042f030_builds_updates_and_reports_the_count() {
        let mut e = water_engine();
        let list = list_with(&mut e, &[]);
        // None: built (0x20 bytes) and added; a zero count deletes it again.
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f030, &args![list, 0u32, 1u32]).u32();
            assert_eq!(result, 0);
        });
        assert_eq!(allocated_sizes(&log), vec![0x20]);
        assert_eq!(calls_to(&log, EXTRA_WATER_ZONE_MAP_INIT).len(), 1);
        assert_eq!(deleted(&log).len(), 1);
        assert!(chain_types(&e, list).is_empty());
        // A nonzero count with no highest zone: result 0, kept.
        let result = e.call(0x0042f030, &args![list, 3u32, 1u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(chain_types(&e, list), vec![EXTRA_WATER_ZONE_MAP]);
        // With a highest zone: the count of its address +4.
        let extra = find_extra(&mut e, list, EXTRA_WATER_ZONE_MAP);
        let zone = e.mem.alloc(0x10);
        e.mem.set_u32(zone + 4, 0x1234);
        e.mem.set_u32(extra.addr() + 0x1c, zone);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f030, &args![list, 4u32, 0u32]).u32();
            assert_eq!(result, 0x1234);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(
            calls_to(&log, EXTRA_WATER_ZONE_MAP_UPDATE),
            vec![vec![extra.addr(), 4, 0]]
        );
        // An empty map deletes the extra data and gives 0.
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f030, &args![list, 0u32, 0u32]).u32();
            assert_eq!(result, 0);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn fn_0042f130_is_true_for_a_positive_count() {
        let mut e = engine();
        // The double stores `a[1]` as the count.
        e.register(WATER_ZONE_MAP_COUNT_OF, |e, a| {
            e.mem.set_u32(a[2], a[1]);
            Ret::default()
        });
        let list = list_with(&mut e, &[(EXTRA_WATER_ZONE_MAP, 0)]);
        let extra = find_extra(&mut e, list, EXTRA_WATER_ZONE_MAP);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0042f130, &args![list, 2u32]).bool());
        });
        let call = &calls_to(&log, WATER_ZONE_MAP_COUNT_OF)[0];
        assert_eq!((call[0], call[1]), (extra.addr() + 0x0c, 2));
        assert!(!e.call(0x0042f130, &args![list, 0u32]).bool());
        assert!(!e.call(0x0042f130, &args![list, 0xffff_ffffu32]).bool());
        let empty = list_with(&mut e, &[]);
        assert!(!e.call(0x0042f130, &args![empty, 2u32]).bool());
    }

    #[test]
    fn fn_0042f180_looks_up_with_the_key() {
        let mut e = engine();
        e.register(WATER_ZONE_MAP_KEY, |e, a| returns(e.mem.u32(a[0])));
        // The lookup stores key + 1 in the output word.
        e.register(WATER_ZONE_MAP_LOOKUP, |e, a| {
            let key = e.mem.u32(a[1]);
            e.mem.set_u32(a[2], key + 1);
            Ret::default()
        });
        let empty = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0042f180, &args![empty]).u32(), 0);
        // A zero key: no lookup.
        let list = list_with(&mut e, &[(EXTRA_WATER_ZONE_MAP, 0)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042f180, &args![list]).u32(), 0);
        });
        assert!(calls_to(&log, WATER_ZONE_MAP_LOOKUP).is_empty());
        // A nonzero key: the output word comes back.
        let list = list_with(&mut e, &[(EXTRA_WATER_ZONE_MAP, 0x40)]);
        let extra = find_extra(&mut e, list, EXTRA_WATER_ZONE_MAP);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042f180, &args![list]).u32(), 0x41);
        });
        let call = &calls_to(&log, WATER_ZONE_MAP_LOOKUP)[0];
        assert_eq!(call[0], extra.addr() + 0x0c);
        assert_eq!(call.len(), 4);
    }

    #[test]
    fn q_water_zone_map_gives_the_address_of_the_map() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_WATER_ZONE_MAP, 0)]);
        let extra = find_extra(&mut e, list, EXTRA_WATER_ZONE_MAP);
        assert_eq!(e.call(0x0042f1d0, &args![list]).u32(), extra.addr() + 0x0c);
        let empty = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0042f1d0, &args![empty]).u32(), 0);
    }

    // ---- ignored by sandbox (0x80)

    #[test]
    fn set_ignored_by_sandbox_builds_and_removes() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        // Clear with none: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0042f200, &args![list, 0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        // Set with none: built (0x0C bytes) and added.
        let log = logged(&mut e, |e| {
            e.call(0x0042f200, &args![list, 1u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x0c]);
        assert_eq!(chain_types(&e, list), vec![EXTRA_IGNORED_BY_SANDBOX]);
        // Set with one: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0042f200, &args![list, 1u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![EXTRA_IGNORED_BY_SANDBOX]);
        // Clear with one: deleted.
        let extra = find_extra(&mut e, list, EXTRA_IGNORED_BY_SANDBOX);
        let log = logged(&mut e, |e| {
            e.call(0x0042f200, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn fn_0042f2d0_tests_for_the_type_80_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[(EXTRA_IGNORED_BY_SANDBOX, 0)]);
        assert!(e.call(0x0042f2d0, &args![list]).bool());
        let empty = list_with(&mut e, &[(0x30, 0)]);
        assert!(!e.call(0x0042f2d0, &args![empty]).bool());
    }

    // ---- patrol ref in use data (0x88)

    #[test]
    fn fn_0042f300_builds_and_removes() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042f300, &args![list, 0u32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0042f300, &args![list, 0x99u32]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x10]);
        assert_eq!(
            calls_to(&log, EXTRA_PATROL_REF_IN_USE_DATA_INIT)[0][1],
            0x99
        );
        assert_eq!(payload_of(&mut e, list, EXTRA_PATROL_REF_IN_USE_DATA), 0x99);
        // A nonzero user with one in the list leaves it as it is.
        let log = logged(&mut e, |e| {
            e.call(0x0042f300, &args![list, 0x9au32]);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(payload_of(&mut e, list, EXTRA_PATROL_REF_IN_USE_DATA), 0x99);
        let extra = find_extra(&mut e, list, EXTRA_PATROL_REF_IN_USE_DATA);
        let log = logged(&mut e, |e| {
            e.call(0x0042f300, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list).is_empty());
    }

    #[test]
    fn get_patrol_ref_can_be_used_by_asks_the_extra_data() {
        let mut e = engine();
        // The byte answer is the low bit of the user word.
        e.register(PATROL_REF_GET_CAN_BE_USED_BY, |_, a| returns(a[1] & 1));
        let none = list_with(&mut e, &[]);
        assert!(e.call(0x0042f3d0, &args![none, 2u32]).bool());
        let list = list_with(&mut e, &[(EXTRA_PATROL_REF_IN_USE_DATA, 0)]);
        let extra = find_extra(&mut e, list, EXTRA_PATROL_REF_IN_USE_DATA);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0042f3d0, &args![list, 3u32]).bool());
            assert!(!e.call(0x0042f3d0, &args![list, 2u32]).bool());
        });
        assert_eq!(
            calls_to(&log, PATROL_REF_GET_CAN_BE_USED_BY),
            vec![vec![extra.addr(), 3], vec![extra.addr(), 2]]
        );
    }

    // ---- follower swim breadcrumbs (0x8b)

    #[test]
    fn get_or_create_follower_swim_breadcrumbs() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0042f420, &args![list]);
        });
        assert_eq!(allocated_sizes(&log), vec![0x28]);
        let extra = find_extra(&mut e, list, EXTRA_FOLLOWER_SWIM_BREADCRUMBS);
        assert!(!extra.is_null());
        let log = logged(&mut e, |e| {
            let again = e.call(0x0042f420, &args![list]).ptr::<BSExtraData>();
            assert_eq!(again, extra);
        });
        assert!(allocated_sizes(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![EXTRA_FOLLOWER_SWIM_BREADCRUMBS]);
    }

    #[test]
    fn get_or_create_returns_the_new_extra_data() {
        let mut e = engine();
        let list = list_with(&mut e, &[]);
        let made = e.call(0x0042f420, &args![list]).ptr::<BSExtraData>();
        assert_eq!(
            made,
            find_extra(&mut e, list, EXTRA_FOLLOWER_SWIM_BREADCRUMBS)
        );
    }

    // ---- arrays and small constructors

    #[test]
    fn fn_0042f4c0_forwards_to_the_element_builder() {
        let mut e = engine();
        stub(&mut e, ARRAY_ELEMENT_INIT);
        let log = logged(&mut e, |e| {
            e.call(0x0042f4c0, &args![Ptr::<()>::new(0x5000), 0x77u32]);
        });
        assert_eq!(calls_to(&log, ARRAY_ELEMENT_INIT), vec![vec![0x5000, 0x77]]);
    }

    #[test]
    fn fn_0042f4e0_runs_the_body_then_sets_the_vtable() {
        let mut e = engine();
        e.register(NI_T_MAP_REFR_BOOL_INIT, |e, a| {
            // The body runs before the vtable is set.
            assert_eq!(e.mem.u32(a[0]), 0);
            Ret::default()
        });
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f4e0, &args![Ptr::<()>::new(block), 37u32]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(
            calls_to(&log, NI_T_MAP_REFR_BOOL_INIT),
            vec![vec![block, 37]]
        );
        assert_eq!(e.mem.u32(block), VTABLE_NI_T_MAP_REFR_BOOL);
    }

    #[test]
    fn nav_mesh_ptr_array_scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = engine();
        stub(&mut e, ARRAY_FREE);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f510, &args![Ptr::<()>::new(block), 3u32]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(calls_to(&log, ARRAY_FREE), vec![vec![block, 1]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
        let log = logged(&mut e, |e| {
            e.call(0x0042f510, &args![Ptr::<()>::new(block), 2u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&log, ARRAY_FREE).len(), 1);
    }

    #[test]
    fn ref_bool_map_scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = engine();
        stub(&mut e, NI_T_MAP_REFR_BOOL_DESTROY);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f540, &args![Ptr::<()>::new(block), 1u32]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(
            calls_to(&log, NI_T_MAP_REFR_BOOL_DESTROY),
            vec![vec![block]]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block]]);
        let log = logged(&mut e, |e| {
            e.call(0x0042f540, &args![Ptr::<()>::new(block), 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_0042f570_sets_the_vtable_and_runs_the_body() {
        let mut e = engine();
        stub(&mut e, BOUND_OBJECT_ARRAY_INIT);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f570, &args![Ptr::<()>::new(block)]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(
            calls_to(&log, BOUND_OBJECT_ARRAY_INIT),
            vec![vec![block, 0, 0]]
        );
        assert_eq!(e.mem.u32(block), VTABLE_BS_SIMPLE_ARRAY_BOUND_OBJECT);
    }

    #[test]
    fn fn_0042f5a0_is_the_unsigned_minimum() {
        let mut e = engine();
        assert_eq!(e.call(0x0042f5a0, &args![3u32, 9u32]).u32(), 3);
        assert_eq!(e.call(0x0042f5a0, &args![9u32, 3u32]).u32(), 3);
        assert_eq!(e.call(0x0042f5a0, &args![5u32, 5u32]).u32(), 5);
        // Unsigned: 0xFFFFFFFF is the largest.
        assert_eq!(e.call(0x0042f5a0, &args![0xffff_ffffu32, 1u32]).u32(), 1);
    }

    #[test]
    fn fn_0042f5d0_forwards_both_words_to_the_memory_manager() {
        let mut e = engine();
        e.register(GET_MEMORY_MANAGER, |_, _| returns(0x1234));
        e.register(MEMORY_MANAGER_ROUTINE, |_, a| returns(a[1] + a[2]));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042f5d0, &args![0x100u32, 0x20u32]).u32(), 0x120);
        });
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_ROUTINE),
            vec![vec![0x1234, 0x100, 0x20]]
        );
    }

    #[test]
    fn fn_0042f5f0_sets_the_size_of_the_array() {
        let mut e = array_engine();
        for address in [
            ARRAY_SET_SIZE_ZERO,
            ARRAY_REALLOCATE,
            ARRAY_RANGE_CONSTRUCT,
            ARRAY_RANGE_DESTROY,
        ] {
            stub(&mut e, address);
        }
        // Size 0 goes to the clearing routine with the shrink byte.
        let array = array_with(&mut e, 0x4000, 2, 4);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 0u32, 1u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_SET_SIZE_ZERO),
            vec![vec![array.addr(), 1]]
        );
        assert_eq!(array_fields(&e, array), (0x4000, 2, 4));
        // Growing an array with no capacity allocates through the vtable.
        let array = array_with(&mut e, 0, 0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 3u32, 0u32]);
        });
        let (buffer, size, reserved) = array_fields(&e, array);
        assert_ne!(buffer, 0);
        assert_eq!((size, reserved), (3, 3));
        assert_eq!(calls_to(&log, ARRAY_ALLOCATOR), vec![vec![array.addr(), 3]]);
        assert_eq!(
            calls_to(&log, ARRAY_RANGE_CONSTRUCT),
            vec![vec![array.addr(), buffer, 3]]
        );
        // Growing past the capacity moves the buffer, then constructs the
        // new elements.
        let array = array_with(&mut e, 0x4000, 2, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 5u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_REALLOCATE),
            vec![vec![array.addr(), 5, 2]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_RANGE_CONSTRUCT),
            vec![vec![array.addr(), 0x4008, 3]]
        );
        assert_eq!(array_fields(&e, array), (0x4000, 5, 5));
        // Growing within the capacity only constructs.
        let array = array_with(&mut e, 0x4000, 2, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 4u32, 0u32]);
        });
        assert!(calls_to(&log, ARRAY_REALLOCATE).is_empty());
        assert_eq!(
            calls_to(&log, ARRAY_RANGE_CONSTRUCT),
            vec![vec![array.addr(), 0x4008, 2]]
        );
        assert_eq!(array_fields(&e, array), (0x4000, 4, 8));
        // The same size: a zero-length construct.
        let array = array_with(&mut e, 0x4000, 4, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 4u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_RANGE_CONSTRUCT),
            vec![vec![array.addr(), 0x4010, 0]]
        );
        // Shrinking destroys the elements above the new size.
        let array = array_with(&mut e, 0x4000, 6, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 4u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_RANGE_DESTROY),
            vec![vec![array.addr(), 0x4010, 2]]
        );
        assert!(calls_to(&log, ARRAY_REALLOCATE).is_empty());
        assert_eq!(array_fields(&e, array), (0x4000, 4, 8));
    }

    #[test]
    fn fn_0042f5f0_shrinks_the_buffer_only_when_asked_and_sparse() {
        let mut e = array_engine();
        stub(&mut e, ARRAY_REALLOCATE);
        stub(&mut e, ARRAY_RANGE_DESTROY);
        // 2 <= 16 / 4: with the shrink byte the buffer is moved.
        let array = array_with(&mut e, 0x4000, 6, 16);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 2u32, 1u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_REALLOCATE),
            vec![vec![array.addr(), 2, 2]]
        );
        assert_eq!(array_fields(&e, array), (0x4000, 2, 2));
        // 5 > 16 / 4: not moved.
        let array = array_with(&mut e, 0x4000, 6, 16);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 5u32, 1u32]);
        });
        assert!(calls_to(&log, ARRAY_REALLOCATE).is_empty());
        assert_eq!(array_fields(&e, array), (0x4000, 5, 16));
        // Sparse but the shrink byte is clear: not moved.
        let array = array_with(&mut e, 0x4000, 6, 16);
        let log = logged(&mut e, |e| {
            e.call(0x0042f5f0, &args![array, 2u32, 0u32]);
        });
        assert!(calls_to(&log, ARRAY_REALLOCATE).is_empty());
        assert_eq!(array_fields(&e, array), (0x4000, 2, 16));
    }

    #[test]
    fn fn_0042f730_holds_the_object_and_takes_a_reference() {
        let mut e = engine();
        stub(&mut e, OBJECT_ADD_REFERENCE);
        let slot = e.mem.alloc(4);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f730, &args![Ptr::<()>::new(slot), 0x6000u32]);
            assert_eq!(result.u32(), slot);
        });
        assert_eq!(e.mem.u32(slot), 0x6000);
        assert_eq!(calls_to(&log, OBJECT_ADD_REFERENCE), vec![vec![0x6000]]);
        let log = logged(&mut e, |e| {
            e.call(0x0042f730, &args![Ptr::<()>::new(slot), 0u32]);
        });
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls_to(&log, OBJECT_ADD_REFERENCE).is_empty());
    }

    #[test]
    fn fn_0042f760_releases_the_held_actor_cause() {
        let mut e = engine();
        stub(&mut e, ACTOR_CAUSE_RELEASE);
        let slot = e.mem.alloc(4);
        let log = logged(&mut e, |e| {
            e.call(0x0042f760, &args![Ptr::<()>::new(slot)]);
        });
        assert!(calls_to(&log, ACTOR_CAUSE_RELEASE).is_empty());
        e.mem.set_u32(slot, 0x7000);
        let log = logged(&mut e, |e| {
            e.call(0x0042f760, &args![Ptr::<()>::new(slot)]);
        });
        assert_eq!(calls_to(&log, ACTOR_CAUSE_RELEASE), vec![vec![0x7000]]);
    }

    #[test]
    fn actor_cause_pointer_assign_swaps_the_reference() {
        let mut e = engine();
        stub(&mut e, ACTOR_CAUSE_RELEASE);
        stub(&mut e, OBJECT_ADD_REFERENCE);
        let slot = e.mem.alloc(4);
        // Empty to an object: only the new reference is taken.
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f780, &args![Ptr::<()>::new(slot), 0x7100u32]);
            assert_eq!(result.u32(), slot);
        });
        assert!(calls_to(&log, ACTOR_CAUSE_RELEASE).is_empty());
        assert_eq!(calls_to(&log, OBJECT_ADD_REFERENCE), vec![vec![0x7100]]);
        assert_eq!(e.mem.u32(slot), 0x7100);
        // The same object: nothing happens.
        let log = logged(&mut e, |e| {
            e.call(0x0042f780, &args![Ptr::<()>::new(slot), 0x7100u32]);
        });
        assert!(calls_to(&log, ACTOR_CAUSE_RELEASE).is_empty());
        assert!(calls_to(&log, OBJECT_ADD_REFERENCE).is_empty());
        // Another object: the old one is released first.
        let log = logged(&mut e, |e| {
            e.call(0x0042f780, &args![Ptr::<()>::new(slot), 0x7200u32]);
        });
        assert_eq!(calls_to(&log, ACTOR_CAUSE_RELEASE), vec![vec![0x7100]]);
        assert_eq!(calls_to(&log, OBJECT_ADD_REFERENCE), vec![vec![0x7200]]);
        // Null: the old one is released, no reference taken.
        let log = logged(&mut e, |e| {
            e.call(0x0042f780, &args![Ptr::<()>::new(slot), 0u32]);
        });
        assert_eq!(calls_to(&log, ACTOR_CAUSE_RELEASE), vec![vec![0x7200]]);
        assert!(calls_to(&log, OBJECT_ADD_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn nav_mesh_ptr_array_constructors_set_the_vtable_and_the_capacity() {
        let mut e = engine();
        stub(&mut e, ARRAY_INIT);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f7d0, &args![Ptr::<()>::new(block), 12u32]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(calls_to(&log, ARRAY_INIT), vec![vec![block, 12, 12]]);
        assert_eq!(e.mem.u32(block), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
        let other = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042f800, &args![Ptr::<()>::new(other)]);
            assert_eq!(result.u32(), other);
        });
        assert_eq!(calls_to(&log, ARRAY_INIT), vec![vec![other, 0, 0]]);
        assert_eq!(e.mem.u32(other), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
    }

    #[test]
    fn fn_0042f830_sets_the_vtable_and_frees_the_buffer() {
        let mut e = engine();
        stub(&mut e, ARRAY_FREE);
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0042f830, &args![Ptr::<()>::new(block)]);
        });
        assert_eq!(calls_to(&log, ARRAY_FREE), vec![vec![block, 1]]);
        assert_eq!(e.mem.u32(block), VTABLE_BS_SIMPLE_ARRAY_NAV_MESH_PTR);
    }

    #[test]
    fn fn_0042f850_adds_an_element_and_gives_its_index() {
        let mut e = array_engine();
        stub(&mut e, ARRAY_ELEMENTS_PREPARE);
        stub(&mut e, ARRAY_ELEMENT_INIT);
        e.register(ARRAY_ADD_SLOT, |_, _| returns(2));
        let array = array_with(&mut e, 0x4000, 2, 4);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042f850, &args![array, 0x88u32]).u32(), 2);
        });
        assert_eq!(
            calls_to(&log, ARRAY_ELEMENTS_PREPARE),
            vec![vec![array.addr(), 0x4008, 1]]
        );
        assert_eq!(calls_to(&log, ARRAY_ELEMENT_INIT), vec![vec![0x4008, 0x88]]);
    }

    #[test]
    fn fn_0042f8a0_removes_in_place_or_into_a_smaller_buffer() {
        let mut e = array_engine();
        for address in [
            ARRAY_ELEMENTS_RELEASE,
            ARRAY_ELEMENTS_MOVE,
            ARRAY_FREE_BUFFER,
        ] {
            stub(&mut e, address);
        }
        e.register(ARRAY_CAN_SHRINK, |e, a| {
            // Sparse when the array is empty enough: size <= 1.
            returns((e.mem.u32(a[0] + 8) <= 1) as u32)
        });
        e.register(ARRAY_SHRUNK_CAPACITY, |_, _| returns(3));
        // In place (no shrink byte): release, shift the rest down.
        let array = array_with(&mut e, 0x4000, 5, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f8a0, &args![array, 1u32, 0u32]);
        });
        assert!(calls_to(&log, ARRAY_CAN_SHRINK).is_empty());
        assert_eq!(
            calls_to(&log, ARRAY_ELEMENTS_RELEASE),
            vec![vec![array.addr(), 0x4004, 1]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_ELEMENTS_MOVE),
            vec![vec![array.addr(), 0x4004, 0x4008, 3]]
        );
        assert_eq!(array_fields(&e, array), (0x4000, 4, 8));
        // The shrink byte but a dense array: in place as well.
        let array = array_with(&mut e, 0x4000, 5, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f8a0, &args![array, 4u32, 1u32]);
        });
        assert_eq!(calls_to(&log, ARRAY_CAN_SHRINK).len(), 1);
        assert!(calls_to(&log, ARRAY_FREE_BUFFER).is_empty());
        assert_eq!(
            calls_to(&log, ARRAY_ELEMENTS_MOVE),
            vec![vec![array.addr(), 0x4010, 0x4014, 0]]
        );
        assert_eq!(array_fields(&e, array), (0x4000, 4, 8));
        // The shrink byte and a sparse array: a new buffer of the shrunk
        // capacity, then the old one is freed.
        let array = array_with(&mut e, 0x4000, 1, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f8a0, &args![array, 0u32, 1u32]);
        });
        let (buffer, size, reserved) = array_fields(&e, array);
        assert_ne!(buffer, 0x4000);
        assert_eq!((size, reserved), (0, 3));
        assert_eq!(calls_to(&log, ARRAY_ALLOCATOR), vec![vec![array.addr(), 3]]);
        // The elements before the index, the released element, then the
        // rest (counted `iSize - 1`, as the game does).
        assert_eq!(
            calls_to(&log, ARRAY_ELEMENTS_MOVE),
            vec![
                vec![array.addr(), buffer, 0x4000, 0],
                vec![array.addr(), buffer, 0x4004, 0]
            ]
        );
        assert_eq!(
            calls_to(&log, ARRAY_ELEMENTS_RELEASE),
            vec![vec![array.addr(), 0x4000, 1]]
        );
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER), vec![vec![array.addr()]]);
    }

    // ---- third session: the one-word smart pointer and the array machinery

    /// Test vtable with the slots the array move code uses: +4 allocates,
    /// +0x0C reallocates.
    const MOVE_VTABLE: u32 = 0x0200_2200;
    const MOVE_ALLOCATOR: u32 = 0x0200_2300;
    const MOVE_REALLOCATOR: u32 = 0x0200_2400;

    /// An engine with the reference count calls, `operator delete`, the
    /// placement `new`, `memmove` and the array vtable `MOVE_VTABLE` stood in
    /// for.
    fn pointer_engine() -> Engine {
        let mut e = engine();
        e.put_vtable(
            MOVE_VTABLE,
            &[DESTRUCTOR, MOVE_ALLOCATOR, 0, MOVE_REALLOCATOR],
        );
        e.register(MOVE_ALLOCATOR, |e, a| returns(e.mem.alloc(a[1] * 4)));
        e.register(MOVE_REALLOCATOR, |e, a| returns(e.mem.alloc(a[2] * 4)));
        stub(&mut e, OBJECT_REFERENCE_ADD);
        stub(&mut e, OBJECT_REFERENCE_RELEASE);
        e.register(PLACEMENT_NEW, |_, a| returns(a[1]));
        e.register(MEMORY_MOVE, |e, a| {
            let bytes: Vec<u8> = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            for (i, byte) in bytes.into_iter().enumerate() {
                e.mem.set_u8(a[0] + i as u32, byte);
            }
            Ret::default()
        });
        e
    }

    /// A slot holding `object`.
    fn slot_with(e: &mut Engine, object: u32) -> Ptr {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, object);
        Ptr::new(slot)
    }

    #[test]
    fn fn_0042fd70_stores_the_object_and_takes_a_reference() {
        let mut e = pointer_engine();
        let slot = slot_with(&mut e, 0xdead);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042fd70, &args![slot, 0x6000u32]);
            assert_eq!(result.u32(), slot.addr());
        });
        assert_eq!(e.mem.u32(slot.addr()), 0x6000);
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_ADD), vec![vec![0x601c]]);
        // A null object is stored without a reference.
        let log = logged(&mut e, |e| {
            e.call(0x0042fd70, &args![slot, 0u32]);
        });
        assert_eq!(e.mem.u32(slot.addr()), 0);
        assert!(calls_to(&log, OBJECT_REFERENCE_ADD).is_empty());
    }

    #[test]
    fn fn_0042fd40_copies_the_held_pointer() {
        let mut e = pointer_engine();
        let source = slot_with(&mut e, 0x6000);
        let target = slot_with(&mut e, 0);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042fd40, &args![target, source]);
            assert_eq!(result.u32(), target.addr());
        });
        assert_eq!(e.mem.u32(target.addr()), 0x6000);
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_ADD), vec![vec![0x601c]]);
        // Copying a null pointer takes no reference.
        let empty = slot_with(&mut e, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042fd40, &args![target, empty]);
        });
        assert_eq!(e.mem.u32(target.addr()), 0);
        assert!(calls_to(&log, OBJECT_REFERENCE_ADD).is_empty());
    }

    #[test]
    fn fn_0042fda0_releases_a_held_object_only() {
        let mut e = pointer_engine();
        let held = slot_with(&mut e, 0x6000);
        let log = logged(&mut e, |e| {
            e.call(0x0042fda0, &args![held]);
        });
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_RELEASE), vec![vec![0x601c]]);
        let empty = slot_with(&mut e, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042fda0, &args![empty]);
        });
        assert!(calls_to(&log, OBJECT_REFERENCE_RELEASE).is_empty());
    }

    #[test]
    fn fn_0042fdc0_assigns_with_release_then_add() {
        let mut e = pointer_engine();
        // Different objects: the old one is released, the new one referenced.
        let target = slot_with(&mut e, 0x6000);
        let source = slot_with(&mut e, 0x7000);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042fdc0, &args![target, source]);
            assert_eq!(result.u32(), target.addr());
        });
        assert_eq!(e.mem.u32(target.addr()), 0x7000);
        assert_eq!(
            log.iter()
                .map(|(callee, a)| (*callee, a.clone()))
                .filter(|(callee, _)| {
                    *callee == OBJECT_REFERENCE_ADD || *callee == OBJECT_REFERENCE_RELEASE
                })
                .collect::<Vec<_>>(),
            vec![
                (OBJECT_REFERENCE_RELEASE, vec![0x601c]),
                (OBJECT_REFERENCE_ADD, vec![0x701c])
            ]
        );
        // The same object: nothing happens.
        let same = slot_with(&mut e, 0x7000);
        let log = logged(&mut e, |e| {
            e.call(0x0042fdc0, &args![target, same]);
        });
        assert_eq!(log.len(), 1);
        // Assigning to an empty pointer only takes a reference.
        let empty = slot_with(&mut e, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042fdc0, &args![empty, source]);
        });
        assert!(calls_to(&log, OBJECT_REFERENCE_RELEASE).is_empty());
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_ADD), vec![vec![0x701c]]);
        // Assigning a null pointer releases the old object only.
        let null = slot_with(&mut e, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0042fdc0, &args![target, null]);
        });
        assert_eq!(e.mem.u32(target.addr()), 0);
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_RELEASE), vec![vec![0x701c]]);
        assert!(calls_to(&log, OBJECT_REFERENCE_ADD).is_empty());
    }

    #[test]
    fn fn_0042fa20_fa40_fb00_wrap_the_pointer_functions() {
        let mut e = pointer_engine();
        // Copy constructor.
        let source = slot_with(&mut e, 0x6000);
        let target = slot_with(&mut e, 0);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042fa20, &args![target, source]);
            assert_eq!(result.u32(), target.addr());
        });
        assert_eq!(e.mem.u32(target.addr()), 0x6000);
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_ADD), vec![vec![0x601c]]);
        // Destructor.
        let log = logged(&mut e, |e| {
            e.call(0x0042fa40, &args![target]);
        });
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_RELEASE), vec![vec![0x601c]]);
        // Null constructor: stores null, takes no reference, returns `this`.
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042fb00, &args![target]);
            assert_eq!(result.u32(), target.addr());
        });
        assert_eq!(e.mem.u32(target.addr()), 0);
        assert!(calls_to(&log, OBJECT_REFERENCE_ADD).is_empty());
    }

    #[test]
    fn fn_0042ffa0_deletes_on_bit_0_only() {
        let mut e = pointer_engine();
        let held = slot_with(&mut e, 0x6000);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042ffa0, &args![held, 0u32]);
            assert_eq!(result.u32(), held.addr());
        });
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_RELEASE), vec![vec![0x601c]]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0042ffa0, &args![held, 3u32]);
        });
        assert_eq!(calls_to(&log, OBJECT_REFERENCE_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![held.addr()]]);
    }

    #[test]
    fn fn_0042ff30_and_ff10_pass_the_comparison_result_on() {
        let mut e = pointer_engine();
        e.register(POINTER_EQUAL, |e, a| {
            returns((e.mem.u32(a[0]) == e.mem.u32(a[1])) as u32)
        });
        let first = slot_with(&mut e, 0x6000);
        let same = slot_with(&mut e, 0x6000);
        let other = slot_with(&mut e, 0x7000);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0042ff30, &args![first, same]).u32(), 1);
            assert_eq!(e.call(0x0042ff30, &args![first, other]).u32(), 0);
            assert_eq!(e.call(0x0042ff10, &args![first, same]).u32(), 1);
            assert_eq!(e.call(0x0042ff10, &args![first, other]).u32(), 0);
        });
        assert_eq!(
            calls_to(&log, POINTER_EQUAL),
            vec![
                vec![first.addr(), same.addr()],
                vec![first.addr(), other.addr()],
                vec![first.addr(), same.addr()],
                vec![first.addr(), other.addr()]
            ]
        );
    }

    #[test]
    fn fn_0042fa60_builds_null_pointers_in_each_slot() {
        let mut e = pointer_engine();
        let block = e.mem.alloc(12);
        for index in 0..3 {
            e.mem.set_u32(block + 4 * index, 0xaaaa);
        }
        let log = logged(&mut e, |e| {
            e.call(0x0042fa60, &args![Ptr::<()>::new(0x5000), block, 3u32]);
        });
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW),
            vec![vec![4, block], vec![4, block + 4], vec![4, block + 8]]
        );
        for index in 0..3 {
            assert_eq!(e.mem.u32(block + 4 * index), 0);
        }
        assert!(calls_to(&log, OBJECT_REFERENCE_ADD).is_empty());
        // A count of 0 does nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0042fa60, &args![Ptr::<()>::new(0x5000), block, 0u32]);
        });
        assert!(calls_to(&log, PLACEMENT_NEW).is_empty());
        // A placement `new` that gives null leaves the slot as it is.
        e.register(PLACEMENT_NEW, |_, _| Ret::default());
        e.mem.set_u32(block, 0xaaaa);
        e.call(0x0042fa60, &args![Ptr::<()>::new(0x5000), block, 1u32]);
        assert_eq!(e.mem.u32(block), 0xaaaa);
    }

    #[test]
    fn fn_0042fb20_destroys_each_element_without_delete() {
        let mut e = pointer_engine();
        let block = e.mem.alloc(12);
        e.mem.set_u32(block, 0x6000);
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0x7000);
        let log = logged(&mut e, |e| {
            e.call(0x0042fb20, &args![Ptr::<()>::new(0x5000), block, 3u32]);
        });
        assert_eq!(
            calls_to(&log, OBJECT_REFERENCE_RELEASE),
            vec![vec![0x601c], vec![0x701c]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_0042fb60_moves_forwards_backwards_or_not_at_all() {
        let mut e = pointer_engine();
        let block = e.mem.alloc(32);
        let fill = |e: &mut Engine| {
            for index in 0..8 {
                e.mem.set_u32(block + 4 * index, 10 + index);
            }
        };
        let words = |e: &Engine| -> Vec<u32> { (0..8).map(|i| e.mem.u32(block + 4 * i)).collect() };
        // Destination below the source: forwards, index 0 first.
        fill(&mut e);
        let log = logged(&mut e, |e| {
            e.call(
                0x0042fb60,
                &args![Ptr::<()>::new(0x5000), block, block + 4, 3u32],
            );
        });
        assert_eq!(
            calls_to(&log, MEMORY_MOVE),
            vec![
                vec![block, block + 4, 4],
                vec![block + 4, block + 8, 4],
                vec![block + 8, block + 12, 4]
            ]
        );
        assert_eq!(words(&e), vec![11, 12, 13, 13, 14, 15, 16, 17]);
        // Destination above the source: backwards, the last word first.
        fill(&mut e);
        let log = logged(&mut e, |e| {
            e.call(
                0x0042fb60,
                &args![Ptr::<()>::new(0x5000), block + 4, block, 3u32],
            );
        });
        assert_eq!(
            calls_to(&log, MEMORY_MOVE),
            vec![
                vec![block + 12, block + 8, 4],
                vec![block + 8, block + 4, 4],
                vec![block + 4, block, 4]
            ]
        );
        assert_eq!(words(&e), vec![10, 10, 11, 12, 14, 15, 16, 17]);
        // The same address, or no words: nothing.
        let log = logged(&mut e, |e| {
            e.call(
                0x0042fb60,
                &args![Ptr::<()>::new(0x5000), block, block, 3u32],
            );
            e.call(
                0x0042fb60,
                &args![Ptr::<()>::new(0x5000), block, block + 4, 0u32],
            );
        });
        assert!(calls_to(&log, MEMORY_MOVE).is_empty());
    }

    #[test]
    fn fn_0042fc00_allocates_reallocates_or_moves() {
        let mut e = pointer_engine();
        // No buffer yet: allocates and records the capacity.
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), MOVE_VTABLE);
        let log = logged(&mut e, |e| {
            e.call(0x0042fc00, &args![array, 6u32, 0u32]);
        });
        let (buffer, size, reserved) = array_fields(&e, array);
        assert_ne!(buffer, 0);
        assert_eq!((size, reserved), (0, 6));
        assert_eq!(calls_to(&log, MOVE_ALLOCATOR), vec![vec![array.addr(), 6]]);
        // `count` equals the capacity: the vtable reallocates (old buffer,
        // new capacity); the capacity field is left to the caller.
        let array = array_with(&mut e, 0x4000, 4, 4);
        e.mem.set_u32(array.addr(), MOVE_VTABLE);
        let log = logged(&mut e, |e| {
            e.call(0x0042fc00, &args![array, 9u32, 4u32]);
        });
        let (buffer, size, reserved) = array_fields(&e, array);
        assert_ne!(buffer, 0x4000);
        assert_eq!((size, reserved), (4, 4));
        assert_eq!(
            calls_to(&log, MOVE_REALLOCATOR),
            vec![vec![array.addr(), 0x4000, 9]]
        );
        assert!(calls_to(&log, MOVE_ALLOCATOR).is_empty());
        // Another count: a new buffer, the words moved, the old one freed.
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x11);
        e.mem.set_u32(source + 4, 0x22);
        let array = array_with(&mut e, source, 2, 8);
        e.mem.set_u32(array.addr(), MOVE_VTABLE);
        stub(&mut e, ARRAY_FREE_BUFFER);
        let log = logged(&mut e, |e| {
            e.call(0x0042fc00, &args![array, 3u32, 2u32]);
        });
        let (buffer, _, reserved) = array_fields(&e, array);
        assert_ne!(buffer, source);
        assert_eq!(reserved, 8);
        assert_eq!(e.mem.u32(buffer), 0x11);
        assert_eq!(e.mem.u32(buffer + 4), 0x22);
        assert_eq!(calls_to(&log, MOVE_ALLOCATOR), vec![vec![array.addr(), 3]]);
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER), vec![vec![array.addr()]]);
    }

    #[test]
    fn fn_0042fcb0_allocates_the_larger_of_capacity_and_size() {
        let mut e = pointer_engine();
        // Nothing requested: all zero, no allocation.
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), MOVE_VTABLE);
        e.set(array, BSSimpleArray::pBuffer, 0x1234);
        e.set(array, BSSimpleArray::iSize, 5);
        e.set(array, BSSimpleArray::iReservedSize, 7);
        let log = logged(&mut e, |e| {
            e.call(0x0042fcb0, &args![array, 0u32, 0u32]);
        });
        assert_eq!(array_fields(&e, array), (0, 0, 0));
        assert!(calls_to(&log, MOVE_ALLOCATOR).is_empty());
        // Capacity above the size: that capacity, the size's elements built.
        let log = logged(&mut e, |e| {
            e.call(0x0042fcb0, &args![array, 4u32, 2u32]);
        });
        let (buffer, size, reserved) = array_fields(&e, array);
        assert_ne!(buffer, 0);
        assert_eq!((size, reserved), (2, 4));
        assert_eq!(calls_to(&log, MOVE_ALLOCATOR), vec![vec![array.addr(), 4]]);
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW),
            vec![vec![4, buffer], vec![4, buffer + 4]]
        );
        // Size above the capacity: the size is the capacity.
        let log = logged(&mut e, |e| {
            e.call(0x0042fcb0, &args![array, 1u32, 3u32]);
        });
        let (_, size, reserved) = array_fields(&e, array);
        assert_eq!((size, reserved), (3, 3));
        assert_eq!(calls_to(&log, MOVE_ALLOCATOR), vec![vec![array.addr(), 3]]);
        // A capacity and no elements: the buffer only.
        let log = logged(&mut e, |e| {
            e.call(0x0042fcb0, &args![array, 5u32, 0u32]);
        });
        let (buffer, size, reserved) = array_fields(&e, array);
        assert_ne!(buffer, 0);
        assert_eq!((size, reserved), (0, 5));
        assert!(calls_to(&log, PLACEMENT_NEW).is_empty());
    }

    #[test]
    fn fn_0042f9c0_clears_and_optionally_frees_the_buffer() {
        let mut e = pointer_engine();
        stub(&mut e, ARRAY_FREE_BUFFER);
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer, 0x6000);
        e.mem.set_u32(buffer + 4, 0x7000);
        // Without the flag: the elements are destroyed, the buffer is kept.
        let array = array_with(&mut e, buffer, 2, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f9c0, &args![array, 0u32]);
        });
        assert_eq!(
            calls_to(&log, OBJECT_REFERENCE_RELEASE),
            vec![vec![0x601c], vec![0x701c]]
        );
        assert!(calls_to(&log, ARRAY_FREE_BUFFER).is_empty());
        assert_eq!(array_fields(&e, array), (buffer, 0, 8));
        // With the flag: the buffer is freed and the fields cleared.
        let array = array_with(&mut e, buffer, 1, 8);
        let log = logged(&mut e, |e| {
            e.call(0x0042f9c0, &args![array, 1u32]);
        });
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER), vec![vec![array.addr()]]);
        assert_eq!(array_fields(&e, array), (0, 0, 0));
        // No buffer: nothing happens, whatever the other fields say.
        let array = array_with(&mut e, 0, 3, 5);
        let log = logged(&mut e, |e| {
            e.call(0x0042f9c0, &args![array, 1u32]);
        });
        assert_eq!(log.len(), 1);
        assert_eq!(array_fields(&e, array), (0, 3, 5));
    }

    #[test]
    fn fn_0042fe10_builds_a_zeroed_table() {
        let mut e = pointer_engine();
        e.register(TABLE_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in 0..a[0] {
                e.mem.set_u8(block + offset, 0xcc);
            }
            returns(block)
        });
        e.register(MEMORY_SET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        let map = e.mem.alloc(0x14);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042fe10, &args![Ptr::<()>::new(map), 5u32]);
            assert_eq!(result.u32(), map);
        });
        assert_eq!(e.mem.u32(map), VTABLE_NI_T_MAP_BASE_REFR_BOOL);
        assert_eq!(e.mem.u32(map + 4), 5);
        assert_eq!(e.mem.u32(map + 0x0c), 0);
        let table = e.mem.u32(map + 8);
        assert_eq!(calls_to(&log, TABLE_ALLOCATE), vec![vec![20]]);
        assert_eq!(calls_to(&log, MEMORY_SET), vec![vec![table, 0, 20]]);
        for index in 0..5 {
            assert_eq!(e.mem.u32(table + 4 * index), 0);
        }
    }

    #[test]
    fn map_destructors_empty_the_map_then_free_the_table() {
        let mut e = pointer_engine();
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, TABLE_FREE);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x9000);
        // The base destructor body.
        let log = logged(&mut e, |e| {
            e.call(0x0042fee0, &args![Ptr::<()>::new(map)]);
        });
        assert_eq!(e.mem.u32(map), VTABLE_NI_T_MAP_BASE_REFR_BOOL);
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls_to(&log, TABLE_FREE), vec![vec![0x9000]]);
        // The `NiTMap` body sets its own vtable and then runs the base one.
        e.register(MAP_REMOVE_ALL, |e, a| {
            // The vtable is set before the map is emptied.
            assert_ne!(e.mem.u32(a[0]), 0);
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0042fe80, &args![Ptr::<()>::new(map)]);
        });
        assert_eq!(e.mem.u32(map), VTABLE_NI_T_MAP_BASE_REFR_BOOL);
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(calls_to(&log, TABLE_FREE), vec![vec![0x9000]]);
        // The deleting destructor of the base deletes on bit 0 only.
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042ffd0, &args![Ptr::<()>::new(map), 0u32]);
            assert_eq!(result.u32(), map);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&log, TABLE_FREE).len(), 1);
        let log = logged(&mut e, |e| {
            e.call(0x0042ffd0, &args![Ptr::<()>::new(map), 1u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![map]]);
        assert_eq!(calls_to(&log, TABLE_FREE).len(), 1);
    }

    #[test]
    fn bound_object_array_destructors() {
        let mut e = pointer_engine();
        stub(&mut e, ARRAY_SET_SIZE_ZERO);
        let array = e.mem.alloc(0x10);
        // The body: vtable, then the size reset with shrink 1.
        let log = logged(&mut e, |e| {
            e.call(0x0042ff80, &args![Ptr::<()>::new(array)]);
        });
        assert_eq!(e.mem.u32(array), VTABLE_BS_SIMPLE_ARRAY_BOUND_OBJECT);
        assert_eq!(calls_to(&log, ARRAY_SET_SIZE_ZERO), vec![vec![array, 1]]);
        // The deleting destructor deletes on bit 0 only.
        let log = logged(&mut e, |e| {
            let result = e.call(0x0042ff50, &args![Ptr::<()>::new(array), 0u32]);
            assert_eq!(result.u32(), array);
        });
        assert_eq!(calls_to(&log, ARRAY_SET_SIZE_ZERO).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0042ff50, &args![Ptr::<()>::new(array), 1u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![array]]);
    }

    #[test]
    fn fn_004300c0_builds_the_type_0x10_extra_data() {
        let mut e = pointer_engine();
        let block = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            let result = e.call(0x004300c0, &args![Ptr::<()>::new(block), 0x6000u32]);
            assert_eq!(result.u32(), block);
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_INIT), vec![vec![block, 0x10]]);
        assert_eq!(e.mem.u32(block), VTABLE_EXTRA_ANIM);
        assert_eq!(e.mem.u8(block + 4), 0x10);
        assert_eq!(e.mem.u32(block + 0x0c), 0x6000);
    }

    #[test]
    fn list_word_getters_chain_the_list_and_the_reader() {
        let mut e = pointer_engine();
        e.register(LIST_AT_OFFSET_0X28, |_, a| returns(a[0] + 0x28));
        e.register(LIST_AT_OFFSET_0X44, |_, a| returns(a[0] + 0x44));
        e.register(EXTRA_WORD_SEEN_DATA, |_, a| returns(a[0] ^ 0x1111));
        e.register(EXTRA_WORD_LOCK, |_, a| returns(a[0] ^ 0x2222));
        let log = logged(&mut e, |e| {
            let seen = e.call(0x00555bc0, &args![Ptr::<()>::new(0x8000)]);
            assert_eq!(seen.u32(), 0x8028 ^ 0x1111);
            let lock = e.call(0x00569140, &args![Ptr::<()>::new(0x9000)]);
            assert_eq!(lock.u32(), 0x9044 ^ 0x2222);
        });
        assert_eq!(calls_to(&log, EXTRA_WORD_SEEN_DATA), vec![vec![0x8028]]);
        assert_eq!(calls_to(&log, EXTRA_WORD_LOCK), vec![vec![0x9044]]);
    }

    #[test]
    fn funcs_registers_one_hundred_five_distinct_addresses_in_range() {
        let entries = funcs();
        assert_eq!(entries.len(), 105);
        let addresses: Vec<u32> = entries.iter().map(|(address, _)| *address).collect();
        let mut sorted = addresses.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(addresses, sorted);
        assert_eq!(addresses[0], 0x0042_dd90);
        assert_eq!(addresses[39], 0x0042_ea50);
        assert_eq!(addresses[40], 0x0042_eb10);
        assert_eq!(addresses[79], 0x0042_f8a0);
        assert_eq!(addresses[80], 0x0042_f9c0);
        assert_eq!(addresses[103], 0x0055_5bc0);
        assert_eq!(addresses[104], 0x0056_9140);
    }
}
