//! `fallout shared/extradataobjects.cpp` (Xbox PDB source unit), part 2: its functions from `004353f0` up to
//! (not including) `004382a0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradataobjects`]; anything public there may be used here.
//!
//! Translated: all 120 functions of the range, `004353f0` to `00438280`
//! (`ExtraXTarget::Compare` to `ExtraSecuritronFace::SetMood`). The range is
//! complete.
//!
//! Notes for the next session:
//! - Every class here is a `BSExtraData` subclass whose payload starts at
//!   +0xc. The constructors call the base constructor (`0040ec80`, type byte),
//!   store the vtable, then initialise the payload; the destructors store the
//!   vtable again (not all of them) and end with the base destructor
//!   (`0040ecb0`). The exception-unwinding frames are not translated.
//! - `00435ac0` and `00435bb0` (the destructors of `ExtraSavedAnimation` and
//!   `ExtraSavedHavokData`, called by their scalar deleting destructors) are
//!   not in Ghidra's function list, so the ledger does not queue them; they
//!   are called by address.

use super::extradatalist::BSSoundHandle;
use super::extradatalist_p3::{ExtraDroppedItemList, ExtraItemDropper};
#[allow(unused_imports)]
use super::extradataobjects::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};

/// `operator delete(pointer)` (cdecl, one stack argument).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSExtraData::BSExtraData(type)` (the base constructor; `this`, then the
/// extra-data type byte).
const BS_EXTRA_DATA_CONSTRUCT: u32 = 0x0040_ec80;
/// `BSExtraData::~BSExtraData` (the base destructor body).
const BS_EXTRA_DATA_DESTRUCT: u32 = 0x0040_ecb0;
/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `RTTI Type Descriptor` of `BSExtraData`, the source type of the
/// `dynamic_cast`s in the `Compare` methods.
const BS_EXTRA_DATA_TYPE: u32 = 0x0118_3b2c;
/// `BSSimpleArray<T,1024>::size` of the folded body (`this` is the array,
/// the count is returned in EAX).
const SIMPLE_ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `BSSimpleArray<T,1024>::operator[]` of the folded body: the address of
/// the element slot.
const SIMPLE_ARRAY_AT: u32 = 0x006a_7ad0;
/// The array element removal `RemoveOldHits` uses (`this` is the array; the
/// arguments are the index and the flag 1). The engine map names the folded
/// body `BSSimpleArray<BGSBodyPart_P_1024>::_MoveItems`.
const SIMPLE_ARRAY_REMOVE_AT: u32 = 0x009a_4320;
/// The array append `AddHit` uses (`this` is the array; the argument is the
/// address of the element to copy in; returns the new index).
const SIMPLE_ARRAY_APPEND: u32 = 0x0043_8610;
/// The `ExtraFriendHits::Hits` array's constructor (`this` is the array).
const HITS_ARRAY_CONSTRUCT: u32 = 0x0043_85c0;
/// The `ExtraFriendHits::Hits` array's destructor (`this` is the array).
const HITS_ARRAY_DESTRUCT: u32 = 0x0043_85f0;
/// `MultiBoundMarkerData::Compare` (Xbox PDB; `this`, other): true when
/// `other` is null or the two differ.
const MULTI_BOUND_MARKER_DATA_COMPARE: u32 = 0x0043_9060;
/// `NiPointer<T>::NiPointer(pointer)` (`this` is the smart pointer): stores
/// the pointer and takes a reference when it is not null.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer<T>::operator=(pointer)` (`this` is the smart pointer).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::~NiPointer` (`this` is the smart pointer): releases the
/// reference when it holds one.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The constructor of the `BSSimpleList<TESObjectREFR *>` inside
/// `ExtraDroppedItemList` (`this` is the list; empties it).
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The list's node release (`this` is the list; walks the nodes and frees
/// them).
const SIMPLE_LIST_CLEAR: u32 = 0x0047_0470;
/// The list's destructor body (`this` is the list; it runs the node release
/// again).
const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The word store `ExtraSavedAnimation::DeleteBuffer` builds its holder with
/// (`this` is the holder, the argument the buffer pointer; returns `this`).
/// The engine map names the folded body `BGSChangeFlags::IsForcedChange`.
const HOLDER_CONSTRUCT: u32 = 0x008c_71b0;
/// The destructor of `ExtraSavedAnimation` (not in Ghidra's function list).
const EXTRA_SAVED_ANIMATION_DESTRUCT: u32 = 0x0043_5ac0;
/// The destructor of `ExtraSavedHavokData` (not in Ghidra's function list).
const EXTRA_SAVED_HAVOK_DATA_DESTRUCT: u32 = 0x0043_5bb0;
/// `ExtraRefractionProperty`'s destructor body (reached by
/// address from its scalar deleting destructor).
const EXTRA_REFRACTION_PROPERTY_DESTRUCT: u32 = 0x0043_5f80;
/// A float game-setting accessor: `this` is the setting (an exe global), the
/// result is the address of its value.
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The float setting `AddHit` compares the age of the newest hit with.
const HIT_SETTING_ADD: u32 = 0x011c_d78c;
/// The float setting `RemoveOldHits` compares the age of each hit with.
const HIT_SETTING_REMOVE: u32 = 0x011c_d580;
/// The global float `00435dd0` returns (the time the hit ages are measured
/// against).
const HIT_CLOCK: u32 = 0x011f_1bf0;

/// Extra-data type bytes, as passed to the base constructor.
const TYPE_EMITTANCE_SOURCE: u32 = 0x67;
const TYPE_MULTI_BOUND_REF: u32 = 0x63;
const TYPE_MULTI_BOUND_DATA: u32 = 0x62;
const TYPE_MULTI_BOUND: u32 = 0x61;
const TYPE_OCCLUSION_PLANE: u32 = 0x71;
const TYPE_PORTAL: u32 = 0x78;
const TYPE_ROOM: u32 = 0x79;
const TYPE_ITEM_DROPPER: u32 = 0x39;
const TYPE_DROPPED_ITEM_LIST: u32 = 0x3a;
const TYPE_SAVED_ANIMATION: u32 = 0x42;
const TYPE_SAVED_HAVOK_DATA: u32 = 0x3d;
const TYPE_FRIEND_HITS: u32 = 0x45;
const TYPE_HEADING_TARGET: u32 = 0x46;
const TYPE_REFRACTION_PROPERTY: u32 = 0x48;

/// The vtables.
const EXTRA_EMITTANCE_SOURCE_VTABLE: u32 = 0x0101_5e34;
const EXTRA_MULTI_BOUND_REF_VTABLE: u32 = 0x0101_5e40;
const EXTRA_MULTI_BOUND_DATA_VTABLE: u32 = 0x0101_5e4c;
const EXTRA_MULTI_BOUND_VTABLE: u32 = 0x0101_5e58;
const EXTRA_OCCLUSION_PLANE_VTABLE: u32 = 0x0101_5e64;
const EXTRA_PORTAL_VTABLE: u32 = 0x0101_5e70;
const EXTRA_ROOM_VTABLE: u32 = 0x0101_5e7c;
const EXTRA_ITEM_DROPPER_VTABLE: u32 = 0x0101_5e88;
const EXTRA_DROPPED_ITEM_LIST_VTABLE: u32 = 0x0101_5e94;
const EXTRA_SAVED_ANIMATION_VTABLE: u32 = 0x0101_5ea0;
const EXTRA_SAVED_HAVOK_DATA_VTABLE: u32 = 0x0101_5eac;
const EXTRA_FRIEND_HITS_VTABLE: u32 = 0x0101_5eb8;
const EXTRA_HEADING_TARGET_VTABLE: u32 = 0x0101_5968;
const EXTRA_REFRACTION_PROPERTY_VTABLE: u32 = 0x0101_5ec4;

/// `RTTI Type Descriptor`s of the classes whose `Compare` casts `other`.
const EXTRA_X_TARGET_TYPE: u32 = 0x0118_4f10;
const EXTRA_EMITTANCE_SOURCE_TYPE: u32 = 0x0118_4f2c;
const EXTRA_MULTI_BOUND_REF_TYPE: u32 = 0x0118_4f50;
const EXTRA_MULTI_BOUND_DATA_TYPE: u32 = 0x0118_4f74;

/// `operator new(size)` (cdecl, one stack argument).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `BSExtraData::Compare` (Xbox PDB; `this`, other).
const BS_EXTRA_DATA_COMPARE: u32 = 0x0040_f700;
/// `memcmp(a, b, size)`.
const MEMCMP: u32 = 0x00ec_4835;
/// The folded `NiPoint3` constructor: `this` is returned (nothing is
/// written); `ExtraEditorRefMoveData` runs it on its three vectors.
const NI_POINT3_CONSTRUCT: u32 = 0x0068_15c0;
/// The same body as an accessor of a `BSSimpleList` node: the address of the
/// node's item word (the node itself).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// A `BSSimpleList` node's next pointer (`this` is the node).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// The list's insertion of the item whose pointer is at the address given
/// (`this` is the list).
const SIMPLE_LIST_ADD_HEAD: u32 = 0x005a_e3d0;
/// The list's scalar deleting destructor (`this` is the list, the argument
/// is the flags; with bit 0 it frees the list).
const SIMPLE_LIST_DELETE: u32 = 0x0047_02f0;
/// `BSSoundHandle`'s default constructor (the id -1, the byte 0, the state
/// 0; `this` is the handle).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// `BSSoundHandle`'s copy (`this` is the destination, the argument the
/// source handle).
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSoundHandle`'s destructor (it does nothing; `this` is the handle).
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `MobileObject::MobileObject` (Xbox PDB; `this` is the 0x88-byte block).
const MOBILE_OBJECT_CONSTRUCT: u32 = 0x0092_eb50;
/// `TESObjectREFR::SetObjectReference` (Xbox PDB; `this`, base object).
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB; the flag is returned in AL).
const GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// `TESObjectREFR::SetRefPersists` (Xbox PDB; `this`, the flag).
const SET_REF_PERSISTS: u32 = 0x0056_5480;
/// `TESObjectREFR::SetLocationOnReference` (Xbox PDB; `this`, the address of
/// a 12-byte vector).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// A `TESObjectREFR` method (`this`, three float words), the rotation.
const SET_ROTATION: u32 = 0x0057_5700;
/// A method of the new object that stores its argument in the word at +0x6c.
const SET_SOURCE_REFERENCE: u32 = 0x004f_bf00;
/// A method that stores its byte argument at +0x81.
const SET_FLAG_BYTE_81: u32 = 0x0089_8280;
/// A `MobileObject` initialisation method (`this` only).
const OBJECT_INIT: u32 = 0x0093_5ae0;
/// An accessor: the word at +0x40 (`TESObjectREFR`'s parent cell).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// A predicate on a cell: bit 0 of the byte at +0x24.
const CELL_CHECK: u32 = 0x0042_5fd0;
/// An accessor: the address `this + 0x44` (the embedded extra data list).
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::SetPersistentCell` (Xbox PDB; `this`, the cell).
const SET_PERSISTENT_CELL: u32 = 0x0041_d390;
/// An accessor: the word at +0x68 (`MiddleHighProcess::GetSavedAcquireObject`
/// in the engine map; on the player it is the process).
const GET_PROCESS: u32 = 0x008d_8520;
/// An accessor: the word at +0x28 of a process (its kind, 0 to 3).
const GET_PROCESS_KIND: u32 = 0x0045_cd60;
/// The method that stores its argument in the word at +0x68
/// (`MiddleHighProcess::SetSavedAcquireObject` in the engine map).
const SET_PROCESS: u32 = 0x0040_7800;
/// `HighProcess::HighProcess` (Xbox PDB), process kind 0, 0x46c bytes.
const HIGH_PROCESS_CONSTRUCT: u32 = 0x008d_7510;
/// The constructor of the 0x25c-byte process, kind 1.
const PROCESS_CONSTRUCT_25C: u32 = 0x0091_3fe0;
/// The constructor of the 200-byte process, kind 2.
const PROCESS_CONSTRUCT_0C8: u32 = 0x0092_c950;
/// The constructor of the 0xb4-byte process, kind 3.
const PROCESS_CONSTRUCT_0B4: u32 = 0x0090_6dc0;
/// `ProcessLists::AddReference` (Xbox PDB; `this` is [`PROCESS_LISTS`],
/// then the object, the process kind and three zero words).
const PROCESS_LISTS_ADD_REFERENCE: u32 = 0x0096_d450;
/// The `ProcessLists` singleton object.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The global holding the `PlayerCharacter` pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// The constructor of the faction map `ExtraFactionChanges` builds on its
/// stack (`this` is the 0x10-byte map, the argument the bucket count).
const FACTION_MAP_CONSTRUCT: u32 = 0x0043_83e0;
/// The faction map's destructor (`this` is the map).
const FACTION_MAP_DESTRUCT: u32 = 0x0043_88a0;
/// The faction map's insertion or update (`this`, key, value byte).
const FACTION_MAP_SET_AT: u32 = 0x0055_94e0;
/// The faction map's lookup (`this`, key, address of the value byte;
/// returns whether the key was found).
const FACTION_MAP_LOOKUP: u32 = 0x0057_c850;
/// An accessor: the address `this + 0x2c` (the faction list embedded in a
/// base actor record).
const FACTION_LIST_OF: u32 = 0x005d_8a70;

/// Extra-data type bytes of this session's classes.
const TYPE_EDITOR_REF_MOVE_DATA: u32 = 0x4c;
const TYPE_HAS_NO_RUMORS: u32 = 0x4e;
const TYPE_SOUND: u32 = 0x4f;
const TYPE_CREATURE_AWAKE_SOUND: u32 = 0x7d;
const TYPE_CREATURE_MOVEMENT_SOUND: u32 = 0x8a;
const TYPE_WEAPON_IDLE_SOUND: u32 = 0x83;
const TYPE_WEAPON_ATTACK_SOUND: u32 = 0x86;
const TYPE_ACTIVATE_LOOP_SOUND: u32 = 0x87;
const TYPE_TALKING_ACTOR: u32 = 0x55;
const TYPE_FACTION_CHANGES: u32 = 0x5e;

/// The vtables of this session's classes.
const EXTRA_EDITOR_REF_MOVE_DATA_VTABLE: u32 = 0x0101_5ed0;
const EXTRA_HAS_NO_RUMORS_VTABLE: u32 = 0x0101_5974;
const EXTRA_SOUND_VTABLE: u32 = 0x0101_5edc;
const EXTRA_CREATURE_AWAKE_SOUND_VTABLE: u32 = 0x0101_5ee8;
const EXTRA_CREATURE_MOVEMENT_SOUND_VTABLE: u32 = 0x0101_5ef4;
const EXTRA_WEAPON_IDLE_SOUND_VTABLE: u32 = 0x0101_5f00;
const EXTRA_WEAPON_ATTACK_SOUND_VTABLE: u32 = 0x0101_5f0c;
const EXTRA_ACTIVATE_LOOP_SOUND_VTABLE: u32 = 0x0101_5f18;
const EXTRA_TALKING_ACTOR_VTABLE: u32 = 0x0101_5f24;
const EXTRA_FACTION_CHANGES_VTABLE: u32 = 0x0101_5f30;

/// `RTTI Type Descriptor`s of the classes whose `Compare` casts `other`.
const EXTRA_RADIUS_TYPE: u32 = 0x0118_43a8;
const EXTRA_RADIATION_TYPE: u32 = 0x0118_43c4;
const EXTRA_RADIO_DATA_TYPE: u32 = 0x0118_41e8;

layout! {
    /// `ExtraEmittanceSource` (Xbox PDB), type `0x67`, 0x10 bytes.
    pub struct ExtraEmittanceSource: 0x10 {
        /// `pSource` (Xbox PDB): `TESForm*`.
        0x0C pSource: Ptr,
    }

    /// `ExtraMultiBoundRef` (Xbox PDB), type `0x63`, 0x10 bytes.
    pub struct ExtraMultiBoundRef: 0x10 {
        /// `pBoundRef` (Xbox PDB): `TESObjectREFR*`.
        0x0C pBoundRef: Ptr,
    }

    /// `ExtraMultiBoundData` (Xbox PDB), type `0x62`, 0x10 bytes.
    pub struct ExtraMultiBoundData: 0x10 {
        /// `pBound` (Xbox PDB): `MultiBoundMarkerData*`, owned.
        0x0C pBound: Ptr,
    }

    /// `ExtraMultiBound` (Xbox PDB), type `0x61`, 0x10 bytes.
    pub struct ExtraMultiBound: 0x10 {
        /// `spBound` (Xbox PDB): `NiPointer<BSMultiBound>`.
        0x0C spBound: Ptr,
    }

    /// `ExtraOcclusionPlane` (Xbox PDB), type `0x71`, 0x10 bytes.
    pub struct ExtraOcclusionPlane: 0x10 {
        /// `spPlane` (Xbox PDB): `NiPointer<BSOcclusionPlane>`.
        0x0C spPlane: Ptr,
    }

    /// `ExtraPortal` (Xbox PDB), type `0x78`, 0x10 bytes.
    pub struct ExtraPortal: 0x10 {
        /// `spPortal` (Xbox PDB): `NiPointer<BSPortal>`.
        0x0C spPortal: Ptr,
    }

    /// `ExtraRoom` (Xbox PDB), type `0x79`, 0x10 bytes.
    pub struct ExtraRoom: 0x10 {
        /// `spRoom` (Xbox PDB): `NiPointer<BSMultiBoundRoom>`.
        0x0C spRoom: Ptr,
    }

    /// `ExtraSavedAnimation` (Xbox PDB), type `0x42`, 0x10 bytes.
    pub struct ExtraSavedAnimation: 0x10 {
        /// `pAnimationBuffer` (Xbox PDB): `char*`.
        0x0C pAnimationBuffer: Ptr,
    }

    /// `ExtraSavedHavokData` (Xbox PDB), type `0x3d`, 0x10 bytes.
    pub struct ExtraSavedHavokData: 0x10 {
        /// `pHavokBuffer` (Xbox PDB): `char*`.
        0x0C pHavokBuffer: Ptr,
    }

    /// `ExtraFriendHits` (Xbox PDB), type `0x45`, 0x1C bytes.
    pub struct ExtraFriendHits: 0x1C {
        /// `Hits` (Xbox PDB): `BSSimpleArray<CombatTimeStamp,1024>`; a
        /// `CombatTimeStamp` is one `float` (the time of the hit).
        0x0C Hits: Inline<BSSimpleArray>,
    }

    /// `ExtraHeadingTarget` (Xbox PDB), type `0x46`, 0x10 bytes.
    pub struct ExtraHeadingTarget: 0x10 {
        /// `pheadingtarget` (Xbox PDB): `TESObjectREFR*`.
        0x0C pheadingtarget: Ptr,
    }

    /// `ExtraRefractionProperty` (Xbox PDB), type `0x48`, 0x10 bytes.
    pub struct ExtraRefractionProperty: 0x10 {
        /// `fRefractionPower` (Xbox PDB).
        0x0C fRefractionPower: f32,
    }
}

/// `operator delete(this)` when bit 0 of `flags` is set (the tail of every
/// scalar deleting destructor).
fn delete_when_asked(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
}

/// The base constructor, then the vtable store that follows it.
fn construct_base(e: &mut Engine, this: Ptr, extra_type: u32, vtable: u32) {
    e.call(BS_EXTRA_DATA_CONSTRUCT, &args![this, extra_type]);
    e.mem.set_u32(this.addr(), vtable);
}

/// The address of the payload word at +0xc.
fn payload<T>(this: Ptr<T>) -> Ptr {
    Ptr::new(this.addr() + 0xc)
}

/// A constructor that stores a null pointer in the payload word.
fn construct_null_payload(e: &mut Engine, this: Ptr, extra_type: u32, vtable: u32) {
    construct_base(e, this, extra_type, vtable);
    e.mem.set_u32(this.addr() + 0xc, 0);
}

/// A constructor of the classes that hold a `NiPointer` at +0xc: the smart
/// pointer is constructed from null, then assigned null. The
/// exception-unwinding frame is not translated.
fn construct_ni_pointer_holder(e: &mut Engine, this: Ptr, extra_type: u32, vtable: u32) {
    construct_base(e, this, extra_type, vtable);
    e.call(NI_POINTER_CONSTRUCT, &args![payload(this), 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![payload(this), 0u32]);
}

/// The compare of the classes whose only payload is one word at +0xc: casts
/// `other` to the class with the RTTI descriptor `target_type` (true when it
/// is not one), then reports whether the words differ.
fn compare_payload_word<T>(e: &mut Engine, this: Ptr<T>, other: Ptr, target_type: u32) -> bool {
    let cast: Ptr = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, BS_EXTRA_DATA_TYPE, target_type, 0u32],
        )
        .ptr();
    if cast.is_null() {
        return true;
    }
    e.mem.u32(this.addr() + 0xc) != e.mem.u32(cast.addr() + 0xc)
}

// Translated from 004353f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraXTarget::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraXTarget` or when `pTarget` differs (the base compare is not asked).
pub fn extra_x_target_compare(e: &mut Engine, this: Ptr<ExtraXTarget>, other: Ptr) -> bool {
    compare_payload_word(e, this, other, EXTRA_X_TARGET_TYPE)
}

// Translated from 00435440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEmittanceSource`'s constructor (the engine map has no name for it):
/// extra-data type 0x67, no source. Returns `this`.
pub fn fn_00435440(e: &mut Engine, this: Ptr<ExtraEmittanceSource>) -> Ptr<ExtraEmittanceSource> {
    construct_null_payload(
        e,
        this.cast(),
        TYPE_EMITTANCE_SOURCE,
        EXTRA_EMITTANCE_SOURCE_VTABLE,
    );
    this
}

// Translated from 00435470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEmittanceSource`'s scalar deleting destructor (the engine map has no
/// name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_00435470(
    e: &mut Engine,
    this: Ptr<ExtraEmittanceSource>,
    flags: u32,
) -> Ptr<ExtraEmittanceSource> {
    fn_004354a0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004354a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEmittanceSource`'s destructor body (the engine map has no name for
/// it): resets the vtable, then runs the base destructor. The source is not
/// owned.
pub fn fn_004354a0(e: &mut Engine, this: Ptr<ExtraEmittanceSource>) {
    e.mem.set_u32(this.addr(), EXTRA_EMITTANCE_SOURCE_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004354c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEmittanceSource::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraEmittanceSource` or when `pSource` differs.
pub fn extra_emittance_source_compare(
    e: &mut Engine,
    this: Ptr<ExtraEmittanceSource>,
    other: Ptr,
) -> bool {
    compare_payload_word(e, this, other, EXTRA_EMITTANCE_SOURCE_TYPE)
}

// Translated from 00435510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBoundRef`'s constructor (the engine map has no name for it):
/// extra-data type 0x63, no reference. Returns `this`.
pub fn fn_00435510(e: &mut Engine, this: Ptr<ExtraMultiBoundRef>) -> Ptr<ExtraMultiBoundRef> {
    construct_null_payload(
        e,
        this.cast(),
        TYPE_MULTI_BOUND_REF,
        EXTRA_MULTI_BOUND_REF_VTABLE,
    );
    this
}

// Translated from 00435540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBoundRef::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraMultiBoundRef` or when `pBoundRef` differs.
pub fn extra_multi_bound_ref_compare(
    e: &mut Engine,
    this: Ptr<ExtraMultiBoundRef>,
    other: Ptr,
) -> bool {
    compare_payload_word(e, this, other, EXTRA_MULTI_BOUND_REF_TYPE)
}

// Translated from 00435590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBoundData`'s constructor (the engine map has no name for it):
/// extra-data type 0x62, no bound data. Returns `this`.
pub fn fn_00435590(e: &mut Engine, this: Ptr<ExtraMultiBoundData>) -> Ptr<ExtraMultiBoundData> {
    construct_null_payload(
        e,
        this.cast(),
        TYPE_MULTI_BOUND_DATA,
        EXTRA_MULTI_BOUND_DATA_VTABLE,
    );
    this
}

// Translated from 004355c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBoundData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_multi_bound_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraMultiBoundData>,
    flags: u32,
) -> Ptr<ExtraMultiBoundData> {
    fn_004355f0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004355f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBoundData`'s destructor body (the engine map has no name for
/// it): resets the vtable, `operator delete`s the owned bound data (without a
/// null test), then runs the base destructor.
pub fn fn_004355f0(e: &mut Engine, this: Ptr<ExtraMultiBoundData>) {
    e.mem.set_u32(this.addr(), EXTRA_MULTI_BOUND_DATA_VTABLE);
    let bound = e.get(this, ExtraMultiBoundData::pBound);
    e.call(OPERATOR_DELETE, &args![bound]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00435630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBoundData::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraMultiBoundData`, or when `MultiBoundMarkerData::Compare` (called on
/// this one's `pBound` with the other's) says they differ.
pub fn extra_multi_bound_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraMultiBoundData>,
    other: Ptr,
) -> bool {
    let cast: Ptr<ExtraMultiBoundData> = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                BS_EXTRA_DATA_TYPE,
                EXTRA_MULTI_BOUND_DATA_TYPE,
                0u32
            ],
        )
        .ptr();
    if cast.is_null() {
        return true;
    }
    let other_bound = e.get(cast, ExtraMultiBoundData::pBound);
    let bound = e.get(this, ExtraMultiBoundData::pBound);
    e.call(MULTI_BOUND_MARKER_DATA_COMPARE, &args![bound, other_bound])
        .bool()
}

// Translated from 00435690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBound`'s constructor (the engine map has no name for it):
/// extra-data type 0x61, an empty `NiPointer` (constructed from null, then
/// assigned null). Returns `this`.
pub fn fn_00435690(e: &mut Engine, this: Ptr<ExtraMultiBound>) -> Ptr<ExtraMultiBound> {
    construct_ni_pointer_holder(e, this.cast(), TYPE_MULTI_BOUND, EXTRA_MULTI_BOUND_VTABLE);
    this
}

// Translated from 00435710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBound::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_multi_bound_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraMultiBound>,
    flags: u32,
) -> Ptr<ExtraMultiBound> {
    fn_00435740(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00435740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraMultiBound`'s destructor body (the engine map has no name for it):
/// destroys the `NiPointer` at +0xc, then runs the base destructor (the
/// vtable is not stored again). The exception-unwinding frame is not
/// translated.
pub fn fn_00435740(e: &mut Engine, this: Ptr<ExtraMultiBound>) {
    e.call(NI_POINTER_DESTRUCT, &args![payload(this)]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004357a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOcclusionPlane`'s constructor (the engine map has no name for it):
/// extra-data type 0x71, an empty `NiPointer`. Returns `this`.
pub fn fn_004357a0(e: &mut Engine, this: Ptr<ExtraOcclusionPlane>) -> Ptr<ExtraOcclusionPlane> {
    construct_ni_pointer_holder(
        e,
        this.cast(),
        TYPE_OCCLUSION_PLANE,
        EXTRA_OCCLUSION_PLANE_VTABLE,
    );
    this
}

// Translated from 00435820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPortal`'s constructor (the engine map has no name for it):
/// extra-data type 0x78, an empty `NiPointer`. Returns `this`.
pub fn fn_00435820(e: &mut Engine, this: Ptr<ExtraPortal>) -> Ptr<ExtraPortal> {
    construct_ni_pointer_holder(e, this.cast(), TYPE_PORTAL, EXTRA_PORTAL_VTABLE);
    this
}

// Translated from 004358a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRoom`'s constructor (the engine map has no name for it):
/// extra-data type 0x79, an empty `NiPointer`. Returns `this`.
pub fn fn_004358a0(e: &mut Engine, this: Ptr<ExtraRoom>) -> Ptr<ExtraRoom> {
    construct_ni_pointer_holder(e, this.cast(), TYPE_ROOM, EXTRA_ROOM_VTABLE);
    this
}

// Translated from 00435920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraItemDropper`'s constructor (the engine map has no name for it):
/// extra-data type 0x39, no dropper. Returns `this`.
pub fn fn_00435920(e: &mut Engine, this: Ptr<ExtraItemDropper>) -> Ptr<ExtraItemDropper> {
    construct_null_payload(e, this.cast(), TYPE_ITEM_DROPPER, EXTRA_ITEM_DROPPER_VTABLE);
    this
}

// Translated from 00435950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDroppedItemList`'s constructor (the engine map has no name for it):
/// extra-data type 0x3a, then the constructor of the embedded list at +0xc.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_00435950(e: &mut Engine, this: Ptr<ExtraDroppedItemList>) -> Ptr<ExtraDroppedItemList> {
    construct_base(
        e,
        this.cast(),
        TYPE_DROPPED_ITEM_LIST,
        EXTRA_DROPPED_ITEM_LIST_VTABLE,
    );
    e.call(SIMPLE_LIST_CONSTRUCT, &args![payload(this)]);
    this
}

// Translated from 004359c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDroppedItemList::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_dropped_item_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraDroppedItemList>,
    flags: u32,
) -> Ptr<ExtraDroppedItemList> {
    fn_004359f0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004359f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDroppedItemList`'s destructor body (the engine map has no name for
/// it): resets the vtable, releases the embedded list's nodes, runs the
/// list's destructor body, then the base destructor. The exception-unwinding
/// frame is not translated.
pub fn fn_004359f0(e: &mut Engine, this: Ptr<ExtraDroppedItemList>) {
    e.mem.set_u32(this.addr(), EXTRA_DROPPED_ITEM_LIST_VTABLE);
    e.call(SIMPLE_LIST_CLEAR, &args![payload(this)]);
    e.call(SIMPLE_LIST_DESTRUCT, &args![payload(this)]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00435a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSavedAnimation`'s constructor (the engine map has no name for it):
/// extra-data type 0x42, no buffer. Returns `this`.
pub fn fn_00435a60(e: &mut Engine, this: Ptr<ExtraSavedAnimation>) -> Ptr<ExtraSavedAnimation> {
    construct_null_payload(
        e,
        this.cast(),
        TYPE_SAVED_ANIMATION,
        EXTRA_SAVED_ANIMATION_VTABLE,
    );
    this
}

// Translated from 00435a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSavedAnimation::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`00435ac0`, which Ghidra has no function for, called by
/// address), then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_saved_animation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraSavedAnimation>,
    flags: u32,
) -> Ptr<ExtraSavedAnimation> {
    e.call(EXTRA_SAVED_ANIMATION_DESTRUCT, &args![this]);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00435b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the buffer a pointer holder owns (the engine map has no name for
/// it): `this` is the address of the holder's pointer; the pointed-to block
/// is `operator delete`d (without a null test) and the pointer set to null.
pub fn fn_00435b20(e: &mut Engine, this: Ptr) {
    let buffer = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![buffer]);
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00435b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSavedHavokData`'s constructor (the engine map has no name for it):
/// extra-data type 0x3d, no buffer. Returns `this`.
pub fn fn_00435b50(e: &mut Engine, this: Ptr<ExtraSavedHavokData>) -> Ptr<ExtraSavedHavokData> {
    construct_null_payload(
        e,
        this.cast(),
        TYPE_SAVED_HAVOK_DATA,
        EXTRA_SAVED_HAVOK_DATA_VTABLE,
    );
    this
}

// Translated from 00435b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSavedHavokData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`00435bb0`, which Ghidra has no function for, called by
/// address), then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_saved_havok_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraSavedHavokData>,
    flags: u32,
) -> Ptr<ExtraSavedHavokData> {
    e.call(EXTRA_SAVED_HAVOK_DATA_DESTRUCT, &args![this]);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00435c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSavedAnimation::DeleteBuffer` (Xbox PDB): builds a one-word holder
/// on the stack around `pAnimationBuffer` and releases it (`operator delete`
/// of the buffer, holder cleared). `pAnimationBuffer` itself keeps its value.
pub fn extra_saved_animation_delete_buffer(e: &mut Engine, this: Ptr<ExtraSavedAnimation>) {
    let buffer = e.get(this, ExtraSavedAnimation::pAnimationBuffer);
    e.with_stack(4, |e, holder| {
        let built = e.call(HOLDER_CONSTRUCT, &args![holder, buffer]).ptr();
        fn_00435b20(e, built);
    });
}

// Translated from 00435c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFriendHits::ExtraFriendHits` (Xbox PDB): extra-data type 0x45, then
/// the constructor of the `Hits` array. Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn extra_friend_hits_extra_friend_hits(
    e: &mut Engine,
    this: Ptr<ExtraFriendHits>,
) -> Ptr<ExtraFriendHits> {
    construct_base(e, this.cast(), TYPE_FRIEND_HITS, EXTRA_FRIEND_HITS_VTABLE);
    e.call(HITS_ARRAY_CONSTRUCT, &args![payload(this)]);
    this
}

// Translated from 00435cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFriendHits::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_friend_hits_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraFriendHits>,
    flags: u32,
) -> Ptr<ExtraFriendHits> {
    fn_00435ce0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00435ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFriendHits`' destructor body (the engine map has no name for it):
/// destroys the `Hits` array, then runs the base destructor (the vtable is
/// not stored again). The exception-unwinding frame is not translated.
pub fn fn_00435ce0(e: &mut Engine, this: Ptr<ExtraFriendHits>) {
    e.call(HITS_ARRAY_DESTRUCT, &args![payload(this)]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// The count of a `BSSimpleArray`.
fn array_size(e: &mut Engine, array: Ptr<BSSimpleArray>) -> u32 {
    e.call(SIMPLE_ARRAY_SIZE, &args![array]).u32()
}

/// The address of slot `index` of a `BSSimpleArray`.
fn array_slot(e: &mut Engine, array: Ptr<BSSimpleArray>, index: u32) -> Ptr {
    e.call(SIMPLE_ARRAY_AT, &args![array, index]).ptr()
}

/// The age (`00435e00`) of the time stamp stored in the array slot at `slot`,
/// which the game first copies into a local.
fn stamp_age(e: &mut Engine, slot: Ptr) -> f32 {
    let stamp = e.mem.u32(slot.addr());
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), stamp);
        fn_00435e00(e, holder)
    })
}

/// The float value of a game setting, read through its accessor.
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let slot: Ptr = e.call(SETTING_FLOAT_VALUE, &args![setting]).ptr();
    f32::from_bits(e.mem.u32(slot.addr()))
}

/// The tail of `AddHit`: a new time stamp (`00435de0` stores the clock
/// `00435dd0` returns) appended to `hits`.
fn record_hit(e: &mut Engine, hits: Ptr<BSSimpleArray>) {
    let now = fn_00435dd0(e);
    e.with_stack(4, |e, stamp| {
        fn_00435de0(e, stamp, now);
        e.call(SIMPLE_ARRAY_APPEND, &args![hits, stamp]);
    });
}

// Translated from 00435d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFriendHits::AddHit` (Xbox PDB): drops the hits that are too old
/// (`RemoveOldHits`), then records a hit at the current time (`00435dd0`)
/// unless the age of the newest remaining hit is below the setting at
/// `011cd78c` (a hit is also recorded when there is none, or when the
/// comparison is unordered).
pub fn extra_friend_hits_add_hit(e: &mut Engine, this: Ptr<ExtraFriendHits>) {
    extra_friend_hits_remove_old_hits(e, this);
    let hits = this.at(ExtraFriendHits::Hits);
    let count = array_size(e, hits) as i32;
    if count > 0 {
        let slot = array_slot(e, hits, (count - 1) as u32);
        let age = stamp_age(e, slot);
        let setting = setting_value(e, HIT_SETTING_ADD);
        // FCOMP then `TEST AH,0x41`: setting <= age, or unordered.
        if setting as f64 > age as f64 {
            return;
        }
    }
    record_hit(e, hits);
}

// Translated from 00435dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The current time the hits are aged against (the engine map has no name for
/// it): returns the `float` at `011f1bf0` in ST0.
pub fn fn_00435dd0(e: &mut Engine) -> f32 {
    e.global::<f32>(HIT_CLOCK)
}

// Translated from 00435de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTimeStamp`'s constructor (the engine map has no name for it):
/// stores `value` in the one-float time stamp at `this`. Returns `this`.
pub fn fn_00435de0(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    e.mem.set_u32(this.addr(), x87_quiet(value).to_bits());
    this
}

// Translated from 00435e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The age of the time stamp at `this` (the engine map has no name for it):
/// the current time (`00435dd0`) minus the stamp, rounded to `float`, in ST0.
pub fn fn_00435e00(e: &mut Engine, this: Ptr) -> f32 {
    let now = fn_00435dd0(e);
    let stamp = f32::from_bits(e.mem.u32(this.addr()));
    (now as f64 - stamp as f64) as f32
}

// Translated from 00435e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFriendHits::GetHitCount` (Xbox PDB): drops the old hits, then
/// returns how many remain.
pub fn extra_friend_hits_get_hit_count(e: &mut Engine, this: Ptr<ExtraFriendHits>) -> u32 {
    extra_friend_hits_remove_old_hits(e, this);
    let hits = this.at(ExtraFriendHits::Hits);
    array_size(e, hits)
}

// Translated from 00435e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFriendHits::RemoveOldHits` (Xbox PDB): removes every hit whose age
/// (`00435e00`) is greater than the setting at `011cd580` (the comparison is
/// ordered: an unordered result keeps the hit). The count is read again on
/// every pass; after a removal the same index is looked at again.
pub fn extra_friend_hits_remove_old_hits(e: &mut Engine, this: Ptr<ExtraFriendHits>) {
    let hits = this.at(ExtraFriendHits::Hits);
    let mut index: u32 = 0;
    while index < array_size(e, hits) {
        let slot = array_slot(e, hits, index);
        let age = stamp_age(e, slot);
        let setting = setting_value(e, HIT_SETTING_REMOVE);
        if (setting as f64) < (age as f64) {
            e.call(SIMPLE_ARRAY_REMOVE_AT, &args![hits, index, 1u32]);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00435ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHeadingTarget`'s constructor (the engine map has no name for it):
/// extra-data type 0x46, storing `target`. Returns `this`.
pub fn fn_00435ec0(
    e: &mut Engine,
    this: Ptr<ExtraHeadingTarget>,
    target: Ptr,
) -> Ptr<ExtraHeadingTarget> {
    construct_base(
        e,
        this.cast(),
        TYPE_HEADING_TARGET,
        EXTRA_HEADING_TARGET_VTABLE,
    );
    e.set(this, ExtraHeadingTarget::pheadingtarget, target);
    this
}

// Translated from 00435ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHeadingTarget`'s destructor body (the engine map has no name for
/// it): resets the vtable, clears `pheadingtarget`, then runs the base
/// destructor.
pub fn fn_00435ef0(e: &mut Engine, this: Ptr<ExtraHeadingTarget>) {
    e.mem.set_u32(this.addr(), EXTRA_HEADING_TARGET_VTABLE);
    e.set(this, ExtraHeadingTarget::pheadingtarget, Ptr::NULL);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00435f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRefractionProperty`'s constructor (the engine map has no name for
/// it): extra-data type 0x48, storing `power`. Returns `this`.
pub fn fn_00435f20(
    e: &mut Engine,
    this: Ptr<ExtraRefractionProperty>,
    power: f32,
) -> Ptr<ExtraRefractionProperty> {
    construct_base(
        e,
        this.cast(),
        TYPE_REFRACTION_PROPERTY,
        EXTRA_REFRACTION_PROPERTY_VTABLE,
    );
    e.set(this, ExtraRefractionProperty::fRefractionPower, power);
    this
}

// Translated from 00435f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRefractionProperty`'s scalar deleting destructor (the engine map has
/// no name for it): the destructor (`00435f80`,
/// called by address), then `operator delete` when `flags & 1`. Returns
/// `this`.
pub fn fn_00435f50(
    e: &mut Engine,
    this: Ptr<ExtraRefractionProperty>,
    flags: u32,
) -> Ptr<ExtraRefractionProperty> {
    e.call(EXTRA_REFRACTION_PROPERTY_DESTRUCT, &args![this]);
    delete_when_asked(e, this.cast(), flags);
    this
}

layout! {
    /// `ExtraEditorRefMoveData` (Xbox PDB), type `0x4c`, 0x30 bytes.
    pub struct ExtraEditorRefMoveData: 0x30 {
        /// `realAngle` (Xbox PDB).
        0x0C realAngle: Inline<NiPoint3>,
        /// `realLocation` (Xbox PDB).
        0x18 realLocation: Inline<NiPoint3>,
        /// `oldLocation` (Xbox PDB).
        0x24 oldLocation: Inline<NiPoint3>,
    }

    /// `ExtraHasNoRumors` (Xbox PDB), type `0x4e`, 0x10 bytes.
    pub struct ExtraHasNoRumors: 0x10 {
        /// `bNoRumors` (Xbox PDB).
        0x0C bNoRumors: u8,
    }

    /// `ExtraSound` (Xbox PDB), type `0x4f`, 0x18 bytes.
    pub struct ExtraSound: 0x18 {
        /// `pHandle` (Xbox PDB): the `BSSoundHandle` held by value.
        0x0C pHandle: Inline<BSSoundHandle>,
    }

    /// `ExtraCreatureAwakeSound` (Xbox PDB), type `0x7d`, 0x18 bytes.
    pub struct ExtraCreatureAwakeSound: 0x18 {
        /// `pHandle` (Xbox PDB): the `BSSoundHandle` held by value.
        0x0C pHandle: Inline<BSSoundHandle>,
    }

    /// `ExtraCreatureMovementSound` (Xbox PDB), type `0x8a`, 0x18 bytes.
    pub struct ExtraCreatureMovementSound: 0x18 {
        /// `pHandle` (Xbox PDB): the `BSSoundHandle` held by value.
        0x0C pHandle: Inline<BSSoundHandle>,
    }

    /// `ExtraWeaponIdleSound` (Xbox PDB), type `0x83`, 0x18 bytes.
    pub struct ExtraWeaponIdleSound: 0x18 {
        /// `pHandle` (Xbox PDB): the `BSSoundHandle` held by value.
        0x0C pHandle: Inline<BSSoundHandle>,
    }

    /// `ExtraWeaponAttackSound` (Xbox PDB), type `0x86`, 0x18 bytes.
    pub struct ExtraWeaponAttackSound: 0x18 {
        /// `pHandle` (Xbox PDB): the `BSSoundHandle` held by value.
        0x0C pHandle: Inline<BSSoundHandle>,
    }

    /// `ExtraActivateLoopSound` (Xbox PDB), type `0x87`, 0x18 bytes.
    pub struct ExtraActivateLoopSound: 0x18 {
        /// `pHandle` (Xbox PDB): the `BSSoundHandle` held by value.
        0x0C pHandle: Inline<BSSoundHandle>,
    }

    /// `ExtraTalkingActor` (Xbox PDB), type `0x55`, 0x10 bytes.
    pub struct ExtraTalkingActor: 0x10 {
        /// `pTalkObject` (Xbox PDB): `MobileObject*` (the PC constructor
        /// stores the object `00436780` builds).
        0x0C pTalkObject: Ptr,
    }

    /// `ExtraRadius` (Xbox PDB), 0x10 bytes.
    pub struct ExtraRadius: 0x10 {
        /// `fRadius` (Xbox PDB).
        0x0C fRadius: f32,
    }

    /// `ExtraRadiation` (Xbox PDB), 0x10 bytes.
    pub struct ExtraRadiation: 0x10 {
        /// `fRadiation` (Xbox PDB).
        0x0C fRadiation: f32,
    }

    /// `FACTION_RANK` (Xbox PDB), 0x8 bytes.
    #[allow(non_camel_case_types)]
    pub struct FACTION_RANK: 0x08 {
        /// `pFaction` (Xbox PDB): `TESFaction*`.
        0x00 pFaction: Ptr,
        /// `cRank` (Xbox PDB); `0xff` is rank -1.
        0x04 cRank: u8,
    }

    /// `ExtraFactionChanges` (Xbox PDB), type `0x5e`, 0x10 bytes.
    pub struct ExtraFactionChanges: 0x10 {
        /// `pFactionChanges` (Xbox PDB): `BSSimpleList<FACTION_RANK *>*`,
        /// owned (a list node is a pointer to the item, then the next node).
        0x0C pFactionChanges: Ptr,
    }

    /// `RADIO_DATA` (Xbox PDB), 0x10 bytes.
    #[allow(non_camel_case_types)]
    pub struct RADIO_DATA: 0x10 {
        /// `fRadius` (Xbox PDB).
        0x00 fRadius: f32,
        /// `eRangeType` (Xbox PDB): `RADIO_RANGE_TYPE`.
        0x04 eRangeType: u32,
        /// `fStaticPct` (Xbox PDB).
        0x08 fStaticPct: f32,
        /// `pPositionRef` (Xbox PDB): `TESObjectREFR*`.
        0x0C pPositionRef: Ptr,
    }

    /// `ExtraRadioData` (Xbox PDB), 0x1c bytes.
    pub struct ExtraRadioData: 0x1C {
        /// `Data` (Xbox PDB).
        0x0C Data: Inline<RADIO_DATA>,
    }
}

// Translated from 00435f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRefractionProperty`'s destructor body (the engine map has no name
/// for it): resets the vtable, then runs the base destructor.
pub fn fn_00435f80(e: &mut Engine, this: Ptr<ExtraRefractionProperty>) {
    e.mem.set_u32(this.addr(), EXTRA_REFRACTION_PROPERTY_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// Copies the three words of a `NiPoint3` (the game does it with word moves).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00435fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraEditorRefMoveData`'s constructor (the engine map has no name for
/// it): extra-data type 0x4c, the three vectors constructed (`006815c0`),
/// then, when `reference` is not null, `realAngle` takes the 12 bytes
/// `00430830` gives for the reference, `realLocation` the vector its virtual
/// method at +0x1f4 returns, and `oldLocation` a copy of `realLocation`.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_00435fa0(
    e: &mut Engine,
    this: Ptr<ExtraEditorRefMoveData>,
    reference: Ptr,
) -> Ptr<ExtraEditorRefMoveData> {
    construct_base(
        e,
        this.cast(),
        TYPE_EDITOR_REF_MOVE_DATA,
        EXTRA_EDITOR_REF_MOVE_DATA_VTABLE,
    );
    for offset in [0x0c, 0x18, 0x24] {
        e.call(NI_POINT3_CONSTRUCT, &args![this.addr() + offset]);
    }
    if !reference.is_null() {
        let angle = fn_00430830(e, reference);
        copy_point(e, angle.addr(), this.addr() + 0x0c);
        // The reference's virtual method at +0x1f4 returns the address of a
        // 12-byte vector (`ExtraTalkingActor`'s constructor passes the same
        // result to `TESObjectREFR::SetLocationOnReference`).
        let location = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
        copy_point(e, location, this.addr() + 0x18);
        copy_point(e, this.addr() + 0x18, this.addr() + 0x24);
    }
    this
}

// Translated from 00436090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHasNoRumors`' constructor (the engine map has no name for it):
/// extra-data type 0x4e, storing the flag byte. Returns `this`.
pub fn fn_00436090(
    e: &mut Engine,
    this: Ptr<ExtraHasNoRumors>,
    no_rumors: u8,
) -> Ptr<ExtraHasNoRumors> {
    construct_base(
        e,
        this.cast(),
        TYPE_HAS_NO_RUMORS,
        EXTRA_HAS_NO_RUMORS_VTABLE,
    );
    e.set(this, ExtraHasNoRumors::bNoRumors, no_rumors);
    this
}

/// The constructor of the classes that hold a `BSSoundHandle` at +0xc: base
/// constructor, the handle's default constructor (`0041a250`), then the
/// handle passed by value (three words) is copied in (`00418900`) and its
/// destructor (`00483710`) runs on the argument copy. The exception-unwinding
/// frame is not translated.
fn construct_sound_holder(
    e: &mut Engine,
    this: Ptr,
    extra_type: u32,
    vtable: u32,
    handle: [u32; 3],
) {
    construct_base(e, this, extra_type, vtable);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![payload(this)]);
    e.with_stack(12, |e, copy| {
        for (index, word) in handle.iter().enumerate() {
            e.mem.set_u32(copy.addr() + 4 * index as u32, *word);
        }
        e.call(SOUND_HANDLE_ASSIGN, &args![payload(this), copy]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![copy]);
    });
}

/// The destructor body of those classes: the vtable is stored again when
/// `vtable` is given, the handle's destructor (`00483710`) runs, then the base
/// destructor.
fn destroy_sound_holder(e: &mut Engine, this: Ptr, vtable: Option<u32>) {
    if let Some(vtable) = vtable {
        e.mem.set_u32(this.addr(), vtable);
    }
    e.call(SOUND_HANDLE_DESTRUCT, &args![payload(this)]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004360c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSound`'s constructor (the engine map has no name for it): extra-data
/// type 0x4f holding a copy of the `BSSoundHandle` passed by value (its three
/// words: sound id, assume-success byte, state). Returns `this`.
pub fn fn_004360c0(
    e: &mut Engine,
    this: Ptr<ExtraSound>,
    sound_id: u32,
    assume_success: u32,
    state: u32,
) -> Ptr<ExtraSound> {
    construct_sound_holder(
        e,
        this.cast(),
        TYPE_SOUND,
        EXTRA_SOUND_VTABLE,
        [sound_id, assume_success, state],
    );
    this
}

// Translated from 00436150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSound`'s scalar deleting destructor (the engine map has no name for
/// it): the destructor, then `operator delete` when `flags & 1`. Returns
/// `this`.
pub fn fn_00436150(e: &mut Engine, this: Ptr<ExtraSound>, flags: u32) -> Ptr<ExtraSound> {
    fn_00436180(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00436180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSound`'s destructor body (the engine map has no name for it): resets
/// the vtable, destroys the handle, runs the base destructor.
pub fn fn_00436180(e: &mut Engine, this: Ptr<ExtraSound>) {
    destroy_sound_holder(e, this.cast(), Some(EXTRA_SOUND_VTABLE));
}

// Translated from 004361e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCreatureAwakeSound`'s constructor (the engine map has no name for
/// it): extra-data type 0x7d holding a copy of the `BSSoundHandle` passed by
/// value. Returns `this`.
pub fn fn_004361e0(
    e: &mut Engine,
    this: Ptr<ExtraCreatureAwakeSound>,
    sound_id: u32,
    assume_success: u32,
    state: u32,
) -> Ptr<ExtraCreatureAwakeSound> {
    construct_sound_holder(
        e,
        this.cast(),
        TYPE_CREATURE_AWAKE_SOUND,
        EXTRA_CREATURE_AWAKE_SOUND_VTABLE,
        [sound_id, assume_success, state],
    );
    this
}

// Translated from 00436270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCreatureAwakeSound`'s scalar deleting destructor (the engine map has
/// no name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_00436270(
    e: &mut Engine,
    this: Ptr<ExtraCreatureAwakeSound>,
    flags: u32,
) -> Ptr<ExtraCreatureAwakeSound> {
    fn_004362a0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004362a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCreatureAwakeSound`'s destructor body (the engine map has no name
/// for it): resets the vtable, destroys the handle, runs the base destructor.
pub fn fn_004362a0(e: &mut Engine, this: Ptr<ExtraCreatureAwakeSound>) {
    destroy_sound_holder(e, this.cast(), Some(EXTRA_CREATURE_AWAKE_SOUND_VTABLE));
}

// Translated from 00436300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCreatureMovementSound`'s constructor (the engine map has no name for
/// it): extra-data type 0x8a holding a copy of the `BSSoundHandle` passed by
/// value. Returns `this`.
pub fn fn_00436300(
    e: &mut Engine,
    this: Ptr<ExtraCreatureMovementSound>,
    sound_id: u32,
    assume_success: u32,
    state: u32,
) -> Ptr<ExtraCreatureMovementSound> {
    construct_sound_holder(
        e,
        this.cast(),
        TYPE_CREATURE_MOVEMENT_SOUND,
        EXTRA_CREATURE_MOVEMENT_SOUND_VTABLE,
        [sound_id, assume_success, state],
    );
    this
}

// Translated from 00436390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCreatureMovementSound`'s scalar deleting destructor (the engine map
/// has no name for it): the destructor, then `operator delete` when
/// `flags & 1`. Returns `this`.
pub fn fn_00436390(
    e: &mut Engine,
    this: Ptr<ExtraCreatureMovementSound>,
    flags: u32,
) -> Ptr<ExtraCreatureMovementSound> {
    fn_004363c0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004363c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCreatureMovementSound`'s destructor body (the engine map has no name
/// for it): destroys the handle, runs the base destructor. Unlike its
/// siblings it does not store the vtable again.
pub fn fn_004363c0(e: &mut Engine, this: Ptr<ExtraCreatureMovementSound>) {
    destroy_sound_holder(e, this.cast(), None);
}

// Translated from 00436420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponIdleSound`'s constructor (the engine map has no name for it):
/// extra-data type 0x83 holding a copy of the `BSSoundHandle` passed by
/// value. Returns `this`.
pub fn fn_00436420(
    e: &mut Engine,
    this: Ptr<ExtraWeaponIdleSound>,
    sound_id: u32,
    assume_success: u32,
    state: u32,
) -> Ptr<ExtraWeaponIdleSound> {
    construct_sound_holder(
        e,
        this.cast(),
        TYPE_WEAPON_IDLE_SOUND,
        EXTRA_WEAPON_IDLE_SOUND_VTABLE,
        [sound_id, assume_success, state],
    );
    this
}

// Translated from 004364b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponIdleSound`'s scalar deleting destructor (the engine map has no
/// name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_004364b0(
    e: &mut Engine,
    this: Ptr<ExtraWeaponIdleSound>,
    flags: u32,
) -> Ptr<ExtraWeaponIdleSound> {
    fn_004364e0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004364e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponIdleSound`'s destructor body (the engine map has no name for
/// it): resets the vtable, destroys the handle, runs the base destructor.
pub fn fn_004364e0(e: &mut Engine, this: Ptr<ExtraWeaponIdleSound>) {
    destroy_sound_holder(e, this.cast(), Some(EXTRA_WEAPON_IDLE_SOUND_VTABLE));
}

// Translated from 00436540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponAttackSound`'s constructor (the engine map has no name for
/// it): extra-data type 0x86 holding a copy of the `BSSoundHandle` passed by
/// value. Returns `this`.
pub fn fn_00436540(
    e: &mut Engine,
    this: Ptr<ExtraWeaponAttackSound>,
    sound_id: u32,
    assume_success: u32,
    state: u32,
) -> Ptr<ExtraWeaponAttackSound> {
    construct_sound_holder(
        e,
        this.cast(),
        TYPE_WEAPON_ATTACK_SOUND,
        EXTRA_WEAPON_ATTACK_SOUND_VTABLE,
        [sound_id, assume_success, state],
    );
    this
}

// Translated from 004365d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponAttackSound`'s scalar deleting destructor (the engine map has
/// no name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_004365d0(
    e: &mut Engine,
    this: Ptr<ExtraWeaponAttackSound>,
    flags: u32,
) -> Ptr<ExtraWeaponAttackSound> {
    fn_00436600(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00436600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWeaponAttackSound`'s destructor body (the engine map has no name for
/// it): resets the vtable, destroys the handle, runs the base destructor.
pub fn fn_00436600(e: &mut Engine, this: Ptr<ExtraWeaponAttackSound>) {
    destroy_sound_holder(e, this.cast(), Some(EXTRA_WEAPON_ATTACK_SOUND_VTABLE));
}

// Translated from 00436660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateLoopSound`'s constructor (the engine map has no name for
/// it): extra-data type 0x87 holding a copy of the `BSSoundHandle` passed by
/// value. Returns `this`.
pub fn fn_00436660(
    e: &mut Engine,
    this: Ptr<ExtraActivateLoopSound>,
    sound_id: u32,
    assume_success: u32,
    state: u32,
) -> Ptr<ExtraActivateLoopSound> {
    construct_sound_holder(
        e,
        this.cast(),
        TYPE_ACTIVATE_LOOP_SOUND,
        EXTRA_ACTIVATE_LOOP_SOUND_VTABLE,
        [sound_id, assume_success, state],
    );
    this
}

// Translated from 004366f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateLoopSound`'s scalar deleting destructor (the engine map has
/// no name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_004366f0(
    e: &mut Engine,
    this: Ptr<ExtraActivateLoopSound>,
    flags: u32,
) -> Ptr<ExtraActivateLoopSound> {
    fn_00436720(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00436720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraActivateLoopSound`'s destructor body (the engine map has no name for
/// it): resets the vtable, destroys the handle, runs the base destructor.
pub fn fn_00436720(e: &mut Engine, this: Ptr<ExtraActivateLoopSound>) {
    destroy_sound_holder(e, this.cast(), Some(EXTRA_ACTIVATE_LOOP_SOUND_VTABLE));
}

/// Allocates `size` bytes (`operator new`) and runs `constructor` on them when
/// the allocation succeeded; null otherwise.
fn new_object(e: &mut Engine, size: u32, constructor: u32) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![size]).u32();
    if memory == 0 {
        0
    } else {
        e.call(constructor, &args![memory]).u32()
    }
}

// Translated from 00436780 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function of this unit that the engine map has no name for; it ignores
/// its `this` (ECX) and `ExtraTalkingActor`'s constructor calls it. Builds a
/// `MobileObject` (0x88 bytes) for `source` and returns it (null when
/// `source` is null). The object is given `base_object` (`SetObjectReference`),
/// the persistence flag, the position (`source + 0x30`) and the rotation
/// (`source + 0x24`) of `source`, then `source` itself (the word at +0x6c),
/// and the byte at +0x81 is set to 1. Its cell is taken from the word at
/// `source + 0x40` when that is not null and `00425fd0` approves it (virtual
/// slot 0x228 of the new object); otherwise from the virtual method at slot
/// 0 of the word at `source + 0x18`, put in the new object's extra data
/// (`ExtraDataList::SetPersistentCell`), falling back to `source + 0x40` and
/// the virtual slot 0x228 again when that gives nothing. Finally, when the
/// player has a process (the word at +0x68), a process of the same kind (the
/// word at +0x28; 0 `HighProcess`, 1 to 3 other classes of 0x25c, 200 and
/// 0xb4 bytes, nothing otherwise) is built, stored in the new object, and
/// the object is added to the process lists. The exception-unwinding frame
/// is not translated.
pub fn fn_00436780(e: &mut Engine, _unused_this: Ptr, base_object: Ptr, source: Ptr) -> Ptr {
    if source.is_null() {
        return Ptr::NULL;
    }
    let object = new_object(e, 0x88, MOBILE_OBJECT_CONSTRUCT);
    e.call(SET_OBJECT_REFERENCE, &args![object, base_object]);
    let persists = e.call(GET_REF_PERSISTS, &args![source]).u32() & 0xff;
    e.call(SET_REF_PERSISTS, &args![object, persists]);
    let position = fn_00436aa0(e, source);
    e.call(SET_LOCATION_ON_REFERENCE, &args![object, position]);
    let rotation = fn_00430830(e, source).addr();
    let words = [
        e.mem.u32(rotation),
        e.mem.u32(rotation + 4),
        e.mem.u32(rotation + 8),
    ];
    e.call(SET_ROTATION, &args![object, words[0], words[1], words[2]]);
    e.call(SET_SOURCE_REFERENCE, &args![object, source]);
    e.call(SET_FLAG_BYTE_81, &args![object, 1u32]);
    e.call(OBJECT_INIT, &args![object]);

    let mut cell_set = false;
    if e.call(GET_PARENT_CELL, &args![source]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![source]).u32();
        if e.call(CELL_CHECK, &args![cell]).bool() {
            let cell = e.call(GET_PARENT_CELL, &args![source]).u32();
            e.vcall(object, 0x228, &args![cell]);
            cell_set = true;
        }
    }
    if !cell_set {
        // `source + 0x18` is the `TESChildCell` base; its virtual slot 0
        // answers the persistent cell.
        let persistent = e.vcall(source.addr() + 0x18, 0, &args![]).u32();
        if persistent != 0 {
            let extra_list = e.call(GET_EXTRA_LIST, &args![object]).u32();
            e.call(SET_PERSISTENT_CELL, &args![extra_list, persistent]);
        } else {
            let cell = e.call(GET_PARENT_CELL, &args![source]).u32();
            e.vcall(object, 0x228, &args![cell]);
        }
    }

    let player = e.mem.u32(PLAYER);
    let player_process = e.call(GET_PROCESS, &args![player]).u32();
    if player_process != 0 {
        let kind = e.call(GET_PROCESS_KIND, &args![player_process]).u32();
        let process = match kind {
            0 => new_object(e, 0x46c, HIGH_PROCESS_CONSTRUCT),
            1 => new_object(e, 0x25c, PROCESS_CONSTRUCT_25C),
            2 => new_object(e, 200, PROCESS_CONSTRUCT_0C8),
            3 => new_object(e, 0xb4, PROCESS_CONSTRUCT_0B4),
            _ => 0,
        };
        e.call(SET_PROCESS, &args![object, process]);
        let kind = e.call(GET_PROCESS_KIND, &args![process]).u32();
        e.call(
            PROCESS_LISTS_ADD_REFERENCE,
            &args![PROCESS_LISTS, object, kind, 0u32, 0u32, 0u32],
        );
    }
    Ptr::new(object)
}

// Translated from 00436aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function of this unit that the engine map has no name for: the address
/// of the position vector at `this + 0x30` (`TESObjectREFR`'s `data.pos`).
pub fn fn_00436aa0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x30)
}

// Translated from 00436ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTalkingActor`'s default constructor (the engine map has no name for
/// it): extra-data type 0x55, no talking object. Returns `this`.
pub fn fn_00436ac0(e: &mut Engine, this: Ptr<ExtraTalkingActor>) -> Ptr<ExtraTalkingActor> {
    construct_null_payload(
        e,
        this.cast(),
        TYPE_TALKING_ACTOR,
        EXTRA_TALKING_ACTOR_VTABLE,
    );
    this
}

// Translated from 00436af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTalkingActor`'s scalar deleting destructor (the engine map has no
/// name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_00436af0(
    e: &mut Engine,
    this: Ptr<ExtraTalkingActor>,
    flags: u32,
) -> Ptr<ExtraTalkingActor> {
    fn_00436bb0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00436b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTalkingActor::ExtraTalkingActor` (Xbox PDB): extra-data type 0x55,
/// then `00436780` builds the talking object from `base_object` and `source`
/// (stored in `pTalkObject`), and the object is moved to the vector that
/// `source`'s virtual method at +0x1f4 returns
/// (`TESObjectREFR::SetLocationOnReference`). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn extra_talking_actor_extra_talking_actor(
    e: &mut Engine,
    this: Ptr<ExtraTalkingActor>,
    base_object: Ptr,
    source: Ptr,
) -> Ptr<ExtraTalkingActor> {
    construct_base(
        e,
        this.cast(),
        TYPE_TALKING_ACTOR,
        EXTRA_TALKING_ACTOR_VTABLE,
    );
    let object = fn_00436780(e, this.cast(), base_object, source);
    e.set(this, ExtraTalkingActor::pTalkObject, object);
    let location = e.vcall(source.addr(), 0x1f4, &args![]).u32();
    e.call(SET_LOCATION_ON_REFERENCE, &args![object, location]);
    this
}

// Translated from 00436bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraTalkingActor`'s destructor body (the engine map has no name for it):
/// resets the vtable, then runs the base destructor. The talking object is
/// not released.
pub fn fn_00436bb0(e: &mut Engine, this: Ptr<ExtraTalkingActor>) {
    e.mem.set_u32(this.addr(), EXTRA_TALKING_ACTOR_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// The start of the `Compare`s below: casts `other` to the class with the RTTI
/// descriptor `target_type`, then asks `BSExtraData::Compare`. `None` means the
/// answer is already true.
fn compare_start(e: &mut Engine, this: Ptr, other: Ptr, target_type: u32) -> Option<Ptr> {
    let cast: Ptr = e
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

/// `Compare` of the classes whose payload is one `float` at +0xc.
fn compare_float_payload(e: &mut Engine, this: Ptr, other: Ptr, target_type: u32) -> bool {
    let Some(cast) = compare_start(e, this, other, target_type) else {
        return true;
    };
    // `FUCOMPP`: differing or unordered (NaN) floats count as different.
    f32::from_bits(e.mem.u32(this.addr() + 0xc)) != f32::from_bits(e.mem.u32(cast.addr() + 0xc))
}

// Translated from 00436bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRadius::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraRadius`, when `BSExtraData::Compare` says so, or when `fRadius`
/// differs.
pub fn extra_radius_compare(e: &mut Engine, this: Ptr<ExtraRadius>, other: Ptr) -> bool {
    compare_float_payload(e, this.cast(), other, EXTRA_RADIUS_TYPE)
}

// Translated from 00436c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRadiation::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraRadiation`, when `BSExtraData::Compare` says so, or when
/// `fRadiation` differs.
pub fn extra_radiation_compare(e: &mut Engine, this: Ptr<ExtraRadiation>, other: Ptr) -> bool {
    compare_float_payload(e, this.cast(), other, EXTRA_RADIATION_TYPE)
}

// Translated from 00436cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFactionChanges`' constructor (the engine map has no name for it):
/// extra-data type 0x5e, then a new 8-byte list (`0096a2d0`) stored in
/// `pFactionChanges`. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_00436cb0(e: &mut Engine, this: Ptr<ExtraFactionChanges>) -> Ptr<ExtraFactionChanges> {
    construct_base(
        e,
        this.cast(),
        TYPE_FACTION_CHANGES,
        EXTRA_FACTION_CHANGES_VTABLE,
    );
    let list = new_object(e, 8, SIMPLE_LIST_CONSTRUCT);
    e.set(this, ExtraFactionChanges::pFactionChanges, Ptr::new(list));
    this
}

// Translated from 00436d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFactionChanges::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_faction_changes_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraFactionChanges>,
    flags: u32,
) -> Ptr<ExtraFactionChanges> {
    fn_00436d80(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00436d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFactionChanges`' destructor body (the engine map has no name for
/// it): resets the vtable, runs the list's node release (`00470470`, also
/// when the list pointer is null) and, when the list exists, its scalar
/// deleting destructor with the flag 1; then the base destructor. The
/// entries themselves are not freed. The exception-unwinding frame is not
/// translated.
pub fn fn_00436d80(e: &mut Engine, this: Ptr<ExtraFactionChanges>) {
    e.mem.set_u32(this.addr(), EXTRA_FACTION_CHANGES_VTABLE);
    let list = e.get(this, ExtraFactionChanges::pFactionChanges);
    e.call(SIMPLE_LIST_CLEAR, &args![list]);
    if !list.is_null() {
        e.call(SIMPLE_LIST_DELETE, &args![list, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

/// The item of a list node (`006815c0` gives the address of the node's item
/// word).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node of a list node (`00726070`).
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NODE_NEXT, &args![node]).u32()
}

/// The entry for `faction` in the list that begins at `node`, walking while
/// the nodes exist and hold an item.
fn find_faction_entry(e: &mut Engine, mut node: u32, faction: Ptr) -> Option<Ptr<FACTION_RANK>> {
    while node != 0 && node_item(e, node) != 0 {
        let entry = Ptr::<FACTION_RANK>::new(node_item(e, node));
        if e.get(entry, FACTION_RANK::pFaction) == faction {
            return Some(entry);
        }
        node = node_next(e, node);
    }
    None
}

/// A new 8-byte entry `{faction, rank}`, added to the list at `list` through
/// `005ae3d0` (which takes the address of a word holding the entry pointer).
fn add_faction_entry(e: &mut Engine, list: Ptr, faction: Ptr, rank: u8) {
    let entry = Ptr::<FACTION_RANK>::new(e.call(OPERATOR_NEW, &args![8u32]).u32());
    e.set(entry, FACTION_RANK::cRank, rank);
    e.set(entry, FACTION_RANK::pFaction, faction);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry.addr());
        e.call(SIMPLE_LIST_ADD_HEAD, &args![list, slot]);
    });
}

// Translated from 00436e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraFactionChanges` that the engine map has no name for:
/// sets the rank of `faction` to `0xff` (-1) when the list has an entry for
/// it; otherwise creates the list if there is none and adds a new entry
/// `{faction, 0xff}`. The exception-unwinding frame is not translated.
pub fn fn_00436e10(e: &mut Engine, this: Ptr<ExtraFactionChanges>, faction: Ptr) {
    let head = e.get(this, ExtraFactionChanges::pFactionChanges);
    if let Some(entry) = find_faction_entry(e, head.addr(), faction) {
        e.set(entry, FACTION_RANK::cRank, 0xff);
        return;
    }
    if e.get(this, ExtraFactionChanges::pFactionChanges).is_null() {
        let list = new_object(e, 8, SIMPLE_LIST_CONSTRUCT);
        e.set(this, ExtraFactionChanges::pFactionChanges, Ptr::new(list));
    }
    let list = e.get(this, ExtraFactionChanges::pFactionChanges);
    add_faction_entry(e, list, faction, 0xff);
}

// Translated from 00436f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraFactionChanges` that the engine map has no name for:
/// sets the rank of `faction` when the list has an entry for it; nothing
/// otherwise.
pub fn fn_00436f20(e: &mut Engine, this: Ptr<ExtraFactionChanges>, faction: Ptr, rank: u8) {
    let head = e.get(this, ExtraFactionChanges::pFactionChanges);
    if let Some(entry) = find_faction_entry(e, head.addr(), faction) {
        e.set(entry, FACTION_RANK::cRank, rank);
    }
}

// Translated from 00436f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraFactionChanges` that the engine map has no name for:
/// sets the rank of `faction` when the list has an entry for it, otherwise
/// adds a new entry `{faction, rank}` to the list (which must exist).
pub fn fn_00436f80(e: &mut Engine, this: Ptr<ExtraFactionChanges>, faction: Ptr, rank: u8) {
    let head = e.get(this, ExtraFactionChanges::pFactionChanges);
    if let Some(entry) = find_faction_entry(e, head.addr(), faction) {
        e.set(entry, FACTION_RANK::cRank, rank);
        return;
    }
    let list = e.get(this, ExtraFactionChanges::pFactionChanges);
    add_faction_entry(e, list, faction, rank);
}

// Translated from 00437010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFactionChanges::GetIsInFaction` (Xbox PDB): true when some entry is
/// for `faction` and has a rank above -1. An entry for it with rank -1 does
/// not stop the walk.
pub fn extra_faction_changes_get_is_in_faction(
    e: &mut Engine,
    this: Ptr<ExtraFactionChanges>,
    faction: Ptr,
) -> bool {
    let mut node = e.get(this, ExtraFactionChanges::pFactionChanges).addr();
    while node != 0 && node_item(e, node) != 0 {
        let entry = Ptr::<FACTION_RANK>::new(node_item(e, node));
        if e.get(entry, FACTION_RANK::pFaction) == faction
            && (e.get(entry, FACTION_RANK::cRank) as i8) > -1
        {
            return true;
        }
        node = node_next(e, node);
    }
    false
}

// Translated from 00437080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFactionChanges::GetIsExpelled` (Xbox PDB): true when some entry is
/// for `faction` and has rank -1.
pub fn extra_faction_changes_get_is_expelled(
    e: &mut Engine,
    this: Ptr<ExtraFactionChanges>,
    faction: Ptr,
) -> bool {
    let mut node = e.get(this, ExtraFactionChanges::pFactionChanges).addr();
    while node != 0 && node_item(e, node) != 0 {
        let entry = Ptr::<FACTION_RANK>::new(node_item(e, node));
        if e.get(entry, FACTION_RANK::pFaction) == faction
            && (e.get(entry, FACTION_RANK::cRank) as i8) == -1
        {
            return true;
        }
        node = node_next(e, node);
    }
    false
}

// Translated from 004370f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraFactionChanges` that the engine map has no name for.
/// Builds a map of 37 buckets on the stack (`004383e0`, destroyed at the end
/// by `004388a0`); every entry of the list gets rank `0xff` and its faction
/// is recorded in the map (`005594e0`, value 1). Then, when `other` is not
/// null, every entry of the list `005d8a70` gives for `other + 0x30` whose
/// faction is not in the map, or is there with a zero value (`0057c850`), is
/// added to the list as `{faction, 0xff}`. The exception-unwinding frame is
/// not translated.
pub fn fn_004370f0(e: &mut Engine, this: Ptr<ExtraFactionChanges>, other: Ptr) {
    e.with_stack(0x10, |e, map| {
        e.call(FACTION_MAP_CONSTRUCT, &args![map, 0x25u32]);
        let mut node = e.get(this, ExtraFactionChanges::pFactionChanges).addr();
        while node != 0 && node_item(e, node) != 0 {
            let entry = Ptr::<FACTION_RANK>::new(node_item(e, node));
            node = node_next(e, node);
            e.set(entry, FACTION_RANK::cRank, 0xff);
            let faction = e.get(entry, FACTION_RANK::pFaction);
            e.call(FACTION_MAP_SET_AT, &args![map, faction, 1u32]);
        }
        if !other.is_null() {
            let mut node = e.call(FACTION_LIST_OF, &args![other.addr() + 0x30]).u32();
            while node != 0 && node_item(e, node) != 0 {
                let entry = Ptr::<FACTION_RANK>::new(node_item(e, node));
                node = node_next(e, node);
                let faction = e.get(entry, FACTION_RANK::pFaction);
                let recorded = e.with_stack(4, |e, value| {
                    let found = e
                        .call(FACTION_MAP_LOOKUP, &args![map, faction, value])
                        .bool();
                    found && e.mem.u8(value.addr()) != 0
                });
                if !recorded {
                    let list = e.get(this, ExtraFactionChanges::pFactionChanges);
                    add_faction_entry(e, list, faction, 0xff);
                }
            }
        }
        e.call(FACTION_MAP_DESTRUCT, &args![map]);
    });
}

// Translated from 00437240 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function of this unit that the engine map has no name for: copies a
/// `RADIO_DATA` (0x10 bytes: radius, range type, static percentage, position
/// reference) from `source` into `this`, nothing when `source` is null. The
/// two floats pass through x87 registers, which quiets a signalling NaN; the
/// two integers are copied as they are.
pub fn fn_00437240(e: &mut Engine, this: Ptr<RADIO_DATA>, source: Ptr<RADIO_DATA>) {
    if source.is_null() {
        return;
    }
    // 00437253: +0 radius, FLD float / FSTP float (quiets a signalling NaN).
    let radius = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), x87_quiet_bits(radius));
    // 0043725d: +4 range type, MOV.
    let range = e.mem.u32(source.addr() + 4);
    e.mem.set_u32(this.addr() + 4, range);
    // 00437269: +8 static percentage, FLD float / FSTP float.
    let percentage = e.mem.u32(source.addr() + 8);
    e.mem.set_u32(this.addr() + 8, x87_quiet_bits(percentage));
    // 00437275: +0xc position reference, MOV.
    let reference = e.mem.u32(source.addr() + 0xc);
    e.mem.set_u32(this.addr() + 0xc, reference);
}

// Translated from 00437290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRadioData::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraRadioData`, when `BSExtraData::Compare` says so, or when the 0x10
/// bytes of `Data` differ.
pub fn extra_radio_data_compare(e: &mut Engine, this: Ptr<ExtraRadioData>, other: Ptr) -> bool {
    let Some(cast) = compare_start(e, this.cast(), other, EXTRA_RADIO_DATA_TYPE) else {
        return true;
    };
    e.call(MEMCMP, &args![payload(this), payload(cast), 0x10u32])
        .u32()
        != 0
}

/// Extra-data type bytes of the classes of the last part of this range.
const TYPE_SAY_TOPIC_INFO_ONCE_A_DAY: u32 = 0x73;
const TYPE_SAY_TO_TOPIC_INFO: u32 = 0x75;
const TYPE_WATER_ZONE_MAP: u32 = 0x7e;
const TYPE_IGNORED_BY_SANDBOX: u32 = 0x80;
const TYPE_PATROL_REF_IN_USE_DATA: u32 = 0x88;
const TYPE_FOLLOWER_SWIM_BREADCRUMBS: u32 = 0x8b;

/// Their vtables.
const EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_VTABLE: u32 = 0x0101_5f3c;
const EXTRA_SAY_TO_TOPIC_INFO_VTABLE: u32 = 0x0101_5f48;
const EXTRA_WATER_ZONE_MAP_VTABLE: u32 = 0x0101_5f54;
const EXTRA_IGNORED_BY_SANDBOX_VTABLE: u32 = 0x0101_5f64;
const EXTRA_PATROL_REF_IN_USE_DATA_VTABLE: u32 = 0x0101_5f70;
const EXTRA_FOLLOWER_SWIM_BREADCRUMBS_VTABLE: u32 = 0x0101_5f7c;

/// `RTTI Type Descriptor`s of the classes whose `Compare` casts `other`.
const EXTRA_COMBAT_STYLE_TYPE: u32 = 0x0118_495c;
const EXTRA_PRIMITIVE_TYPE: u32 = 0x0118_42b0;
const EXTRA_AMMO_TYPE: u32 = 0x0118_48c0;

/// The list's head removal (`this` is the list; the first item's node is
/// taken out and the next node moves into the head).
const SIMPLE_LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list's replacement of the head item (`this` is the list; the
/// argument is the address of a word holding the new item).
const SIMPLE_LIST_SET_HEAD: u32 = 0x0072_6c60;
/// The insertion `ExtraFollowerSwimBreadcrumbs` uses for its crumb list
/// (`this` is the list; the argument is the address of the word holding the
/// crumb pointer).
const CRUMB_LIST_ADD: u32 = 0x0090_5820;
/// `ExtraFollowerSwimBreadcrumb`'s scalar deleting destructor (`this`, the
/// flags).
const CRUMB_DELETE: u32 = 0x007b_3fa0;
/// The constructor of the map `ExtraWaterZoneMap` embeds at +0xc (`this` is
/// the map, the argument the bucket count).
const WATER_ZONE_MAP_CONSTRUCT: u32 = 0x0043_83b0;
/// The destructor of that map (`this` is the map).
const WATER_ZONE_MAP_DESTRUCT: u32 = 0x0043_8720;
/// The map's lookup (`this` is the map, the arguments the key and the
/// address of the word that receives the count).
const ZONE_MAP_LOOKUP: u32 = 0x0085_3130;
/// The map's `SetAt` (`this` is the map, the arguments the key and the value).
const ZONE_MAP_SET_AT: u32 = 0x0084_4700;
/// The map's `RemoveAt` (`this` is the map, the argument the key).
const ZONE_MAP_REMOVE_AT: u32 = 0x0040_5430;
/// The map's first position (`this` is the map; 0 when it is empty).
const ZONE_MAP_FIRST: u32 = 0x004b_9ba0;
/// The map's iteration step (`this` is the map; the arguments are the
/// addresses of the position, of the key and of the value, which receive
/// the entry; the position advances).
const ZONE_MAP_NEXT: u32 = 0x006b_7f20;
/// The float `fn_00437850` starts its search for the highest zone from (the
/// lowest `f32`, held in the exe's data).
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
/// The handle of a reference (`this` is the reference).
const REFERENCE_HANDLE: u32 = 0x0084_e3a0;
/// The reference for a handle (cdecl, one stack argument; null when the
/// handle has none).
const REFERENCE_FOR_HANDLE: u32 = 0x0048_39c0;
/// A method of the reference that returns a small integer (the code
/// compared with 3 and with 5 below; `this` only).
const REFERENCE_KIND: u32 = 0x004f_8960;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// The global that holds a fixed string's empty value.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// Creates the fixed string for a C string (cdecl, one stack argument).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// Releases a fixed string's text (cdecl, one stack argument: the address
/// of the text's header, 8 bytes before the text).
const FIXED_STRING_RELEASE: u32 = 0x0040_19a0;
/// The string assignment `ExtraSecuritronFace` uses (`this` is the string,
/// the argument a C string).
const STRING_ASSIGN: u32 = 0x0043_8390;
/// The two methods of `ExtraSecuritronFace` that copy a string out (`this`
/// is the extra data, the argument the address of the destination string).
const GET_FACE_STRING_FIRST: u32 = 0x0042_8070;
const GET_FACE_STRING_SECOND: u32 = 0x0042_80b0;
/// A string's length (`this` is the string).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// A string's destructor (`this` is the string).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// An accessor that returns the word `this` points at (the text of a string,
/// the object of a smart pointer).
const POINTER_VALUE: u32 = 0x0055_9450;
/// `sprintf`-style formatting (cdecl: buffer, size, format, then the values).
const FORMAT_STRING: u32 = 0x0040_6d00;
/// `SwapPlatformLanguageTexturePath(source, destination, size)` (Xbox PDB;
/// cdecl).
const SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH: u32 = 0x004b_7240;
/// The global holding the `TES` pointer.
const TES: u32 = 0x011d_ea10;
/// `TES::CreateTextureImage` (Xbox PDB; `this` is the TES object, then the
/// path, the address of the smart pointer that receives the image and two
/// zero words).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// `NiAVObject::GetProperty` (Xbox PDB; `this`, the property type).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// The format of the texture path `ApplyFace` builds.
const SECURITRON_TEXTURE_FORMAT: u32 = 0x0101_5f84;
/// The name of the object `ApplyFace` looks for.
const SECURITRON_SCREEN_NAME: u32 = 0x0101_5fac;
/// The three-word exe global `ExtraFollowerSwimBreadcrumbs`' constructor
/// copies into `LeaderLocation`.
const DEFAULT_LEADER_LOCATION: u32 = 0x011f_426c;

layout! {
    /// `ExtraCombatStyle` (Xbox PDB), 0x10 bytes.
    pub struct ExtraCombatStyle: 0x10 {
        /// `pCombatStyle` (Xbox PDB): `TESCombatStyle*`.
        0x0C pCombatStyle: Ptr,
    }

    /// `ExtraPrimitive` (Xbox PDB), 0x10 bytes.
    pub struct ExtraPrimitive: 0x10 {
        /// `pPrimitive` (Xbox PDB): `BGSPrimitive*`.
        0x0C pPrimitive: Ptr,
    }

    /// `ExtraAmmo` (Xbox PDB), 0x14 bytes.
    pub struct ExtraAmmo: 0x14 {
        /// `pAmmo` (Xbox PDB): `TESAmmo*`.
        0x0C pAmmo: Ptr,
        /// `iCount` (Xbox PDB).
        0x10 iCount: i32,
    }

    /// `ExtraSayTopicInfoOnceADay` (Xbox PDB), type `0x73`, 0x10 bytes.
    pub struct ExtraSayTopicInfoOnceADay: 0x10 {
        /// `pListofSaidOnceTopicInfos` (Xbox PDB):
        /// `BSSimpleList<SayOnceTopicInfos *>*`, owned.
        0x0C pListofSaidOnceTopicInfos: Ptr,
    }

    /// `ExtraSayToTopicInfo` (Xbox PDB), type `0x75`, 0x1C bytes.
    pub struct ExtraSayToTopicInfo: 0x1C {
        /// `pInfo` (Xbox PDB): `TESTopicInfo*`.
        0x0C pInfo: Ptr,
        /// `pTopic` (Xbox PDB): `TESTopic*`.
        0x10 pTopic: Ptr,
        /// `nResponseID` (Xbox PDB).
        0x14 nResponseID: u32,
        /// `bVoiceDone` (Xbox PDB).
        0x18 bVoiceDone: u8,
    }

    /// `ExtraWaterZoneMap` (Xbox PDB), type `0x7e`, 0x20 bytes. The
    /// `NiTMap<WaterZone *,int>` `WaterZoneMap` sits at +0xc (0x10 bytes).
    pub struct ExtraWaterZoneMap: 0x20 {
        /// `pHighestWaterZone` (Xbox PDB): `WaterZone*`.
        0x1C pHighestWaterZone: Ptr,
    }

    /// `ExtraPatrolRefInUseData` (Xbox PDB), type `0x88`, 0x10 bytes.
    pub struct ExtraPatrolRefInUseData: 0x10 {
        /// `User` (Xbox PDB): the handle of the reference using the patrol
        /// reference.
        0x0C User: u32,
    }

    /// `ExtraFollowerSwimBreadcrumbs` (Xbox PDB), type `0x8b`, 0x28 bytes.
    /// `LeaderLocation` (a `NiPoint3`, +0x10) and `CrumbList` (a
    /// `BSSimpleList`, +0x20) are used by address.
    pub struct ExtraFollowerSwimBreadcrumbs: 0x28 {
        /// `eLeaderState` (Xbox PDB).
        0x0C eLeaderState: u32,
        /// `LeaderNavMeshID` (Xbox PDB).
        0x1C LeaderNavMeshID: u32,
    }

    /// `ExtraSecuritronFace` (Xbox PDB), 0x1C bytes. `msPersonality` (+0xc)
    /// and `msMood` (+0x14) are `BSStringT<char>`s, used by address.
    pub struct ExtraSecuritronFace: 0x1C {
    }
}

// Translated from 00437300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCombatStyle::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraCombatStyle`, when `BSExtraData::Compare` says so, or when
/// `pCombatStyle` differs.
pub fn extra_combat_style_compare(e: &mut Engine, this: Ptr<ExtraCombatStyle>, other: Ptr) -> bool {
    let Some(cast) = compare_start(e, this.cast(), other, EXTRA_COMBAT_STYLE_TYPE) else {
        return true;
    };
    e.mem.u32(this.addr() + 0xc) != e.mem.u32(cast.addr() + 0xc)
}

// Translated from 00437370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPrimitive::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraPrimitive` or when `pPrimitive` differs (the base compare is not
/// asked).
pub fn extra_primitive_compare(e: &mut Engine, this: Ptr<ExtraPrimitive>, other: Ptr) -> bool {
    compare_payload_word(e, this, other, EXTRA_PRIMITIVE_TYPE)
}

// Translated from 004373c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAmmo::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraAmmo`, when `BSExtraData::Compare` says so, or when `iCount` (+0x10)
/// or `pAmmo` (+0xc) differs.
pub fn extra_ammo_compare(e: &mut Engine, this: Ptr<ExtraAmmo>, other: Ptr) -> bool {
    let Some(cast) = compare_start(e, this.cast(), other, EXTRA_AMMO_TYPE) else {
        return true;
    };
    if e.mem.u32(this.addr() + 0x10) != e.mem.u32(cast.addr() + 0x10) {
        return true;
    }
    e.mem.u32(this.addr() + 0xc) != e.mem.u32(cast.addr() + 0xc)
}

// Translated from 00437440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSayTopicInfoOnceADay`'s constructor (the engine map has no name for
/// it): extra-data type 0x73, then a new 8-byte list (`0096a2d0`) stored in
/// `pListofSaidOnceTopicInfos`. Returns `this`. The exception-unwinding frame
/// is not translated.
pub fn fn_00437440(
    e: &mut Engine,
    this: Ptr<ExtraSayTopicInfoOnceADay>,
) -> Ptr<ExtraSayTopicInfoOnceADay> {
    construct_base(
        e,
        this.cast(),
        TYPE_SAY_TOPIC_INFO_ONCE_A_DAY,
        EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_VTABLE,
    );
    let list = new_object(e, 8, SIMPLE_LIST_CONSTRUCT);
    e.set(
        this,
        ExtraSayTopicInfoOnceADay::pListofSaidOnceTopicInfos,
        Ptr::new(list),
    );
    this
}

// Translated from 004374e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSayTopicInfoOnceADay::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_say_topic_info_once_a_day_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraSayTopicInfoOnceADay>,
    flags: u32,
) -> Ptr<ExtraSayTopicInfoOnceADay> {
    fn_004375c0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00437510 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor of `ExtraSayTopicInfoOnceADay` that the engine map has no
/// name for: as `00437440`, then it adds `item` (the one stack word, passed
/// by address) to the new list (`005ae3d0`, with no null check). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_00437510(
    e: &mut Engine,
    this: Ptr<ExtraSayTopicInfoOnceADay>,
    item: u32,
) -> Ptr<ExtraSayTopicInfoOnceADay> {
    fn_00437440(e, this);
    let list = e.get(this, ExtraSayTopicInfoOnceADay::pListofSaidOnceTopicInfos);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(SIMPLE_LIST_ADD_HEAD, &args![list, slot]);
    });
    this
}

// Translated from 004375c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSayTopicInfoOnceADay`'s destructor body (the engine map has no name
/// for it): resets the vtable, then, while the list has a first item, takes
/// the item, removes the head node (`0063f7b0`) and frees the item
/// (`operator delete`); then runs the list's scalar deleting destructor with
/// the flag 1 when the list exists; then the base destructor. The
/// exception-unwinding frame is not translated.
pub fn fn_004375c0(e: &mut Engine, this: Ptr<ExtraSayTopicInfoOnceADay>) {
    e.mem
        .set_u32(this.addr(), EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_VTABLE);
    let list = e
        .get(this, ExtraSayTopicInfoOnceADay::pListofSaidOnceTopicInfos)
        .addr();
    while list != 0 && node_item(e, list) != 0 {
        let item = node_item(e, list);
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![list]);
        e.call(OPERATOR_DELETE, &args![item]);
    }
    let list = e.get(this, ExtraSayTopicInfoOnceADay::pListofSaidOnceTopicInfos);
    if !list.is_null() {
        e.call(SIMPLE_LIST_DELETE, &args![list, 1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00437690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSayToTopicInfo`'s default constructor (the engine map has no name
/// for it): extra-data type 0x75, `pInfo`, `pTopic` and `nResponseID` zero,
/// `bVoiceDone` false. Returns `this`.
pub fn fn_00437690(e: &mut Engine, this: Ptr<ExtraSayToTopicInfo>) -> Ptr<ExtraSayToTopicInfo> {
    construct_base(
        e,
        this.cast(),
        TYPE_SAY_TO_TOPIC_INFO,
        EXTRA_SAY_TO_TOPIC_INFO_VTABLE,
    );
    e.set(this, ExtraSayToTopicInfo::pInfo, Ptr::new(0));
    e.set(this, ExtraSayToTopicInfo::pTopic, Ptr::new(0));
    e.set(this, ExtraSayToTopicInfo::nResponseID, 0);
    e.set(this, ExtraSayToTopicInfo::bVoiceDone, 0);
    this
}

// Translated from 004376e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSayToTopicInfo`'s constructor with the info (the engine map has no
/// name for it): extra-data type 0x75, `pInfo` = `info`, the other fields
/// zero (`nResponseID` is not written by this one). Returns `this`.
pub fn fn_004376e0(
    e: &mut Engine,
    this: Ptr<ExtraSayToTopicInfo>,
    info: u32,
) -> Ptr<ExtraSayToTopicInfo> {
    construct_base(
        e,
        this.cast(),
        TYPE_SAY_TO_TOPIC_INFO,
        EXTRA_SAY_TO_TOPIC_INFO_VTABLE,
    );
    e.set(this, ExtraSayToTopicInfo::pInfo, Ptr::new(info));
    e.set(this, ExtraSayToTopicInfo::pTopic, Ptr::new(0));
    e.set(this, ExtraSayToTopicInfo::nResponseID, 0);
    e.set(this, ExtraSayToTopicInfo::bVoiceDone, 0);
    this
}

// Translated from 00437730 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setter of `ExtraSayToTopicInfo` that the engine map has no name for:
/// stores `topic` in `pTopic` (+0x10).
pub fn fn_00437730(e: &mut Engine, this: Ptr<ExtraSayToTopicInfo>, topic: u32) {
    e.set(this, ExtraSayToTopicInfo::pTopic, Ptr::new(topic));
}

// Translated from 00437750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterZoneMap`'s constructor (the engine map has no name for it):
/// extra-data type 0x7e, the embedded map built with 0x25 buckets (`004383b0`)
/// and `pHighestWaterZone` null. Returns `this`. The exception-unwinding
/// frame is not translated.
pub fn fn_00437750(e: &mut Engine, this: Ptr<ExtraWaterZoneMap>) -> Ptr<ExtraWaterZoneMap> {
    construct_base(
        e,
        this.cast(),
        TYPE_WATER_ZONE_MAP,
        EXTRA_WATER_ZONE_MAP_VTABLE,
    );
    e.call(WATER_ZONE_MAP_CONSTRUCT, &args![this.addr() + 0xc, 0x25u32]);
    e.set(this, ExtraWaterZoneMap::pHighestWaterZone, Ptr::new(0));
    this
}

// Translated from 004377c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterZoneMap::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_water_zone_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraWaterZoneMap>,
    flags: u32,
) -> Ptr<ExtraWaterZoneMap> {
    fn_004377f0(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 004377f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterZoneMap`'s destructor body (the engine map names this body
/// after a folded library class): destroys the embedded map (`00438720`),
/// then runs the base destructor. The vtable is not stored again. The
/// exception-unwinding frame is not translated.
pub fn fn_004377f0(e: &mut Engine, this: Ptr<ExtraWaterZoneMap>) {
    e.call(WATER_ZONE_MAP_DESTRUCT, &args![this.addr() + 0xc]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00437850 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraWaterZoneMap` that the engine map has no name for; it
/// keeps the count of the zone in the map (`WaterZoneMap`, at +0xc) and
/// `pHighestWaterZone`, the zone with the greatest float at +0x18.
///
/// With `add` set: looks the zone up (the result is not used), sets its count
/// to 1 (`SetAt`) and, when its float is above the lowest float, makes it the
/// highest. With `add` clear: looks the count up; at 1 or less the zone is
/// removed (`RemoveAt`) and the highest zone is searched again over the
/// remaining entries (an entry replaces the best one when its float is
/// strictly above it, so the earlier entry wins a tie; `pHighestWaterZone` is
/// left alone when no entry is above the lowest float); above 1 the count is
/// set to the count minus 1.
pub fn fn_00437850(e: &mut Engine, this: Ptr<ExtraWaterZoneMap>, zone: u32, add: bool) {
    let map = this.addr() + 0xc;
    let lowest = e.global::<f32>(LOWEST_FLOAT);
    if add {
        e.with_stack(4, |e, count| {
            e.mem.set_u32(count.addr(), 0);
            e.call(ZONE_MAP_LOOKUP, &args![map, zone, count]);
        });
        e.call(ZONE_MAP_SET_AT, &args![map, zone, 1u32]);
        if lowest < f32::from_bits(e.mem.u32(zone + 0x18)) {
            e.set(this, ExtraWaterZoneMap::pHighestWaterZone, Ptr::new(zone));
        }
        return;
    }
    let count = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(ZONE_MAP_LOOKUP, &args![map, zone, slot]);
        e.mem.u32(slot.addr()) as i32
    });
    if count > 1 {
        e.call(ZONE_MAP_SET_AT, &args![map, zone, (count - 1) as u32]);
        return;
    }
    e.call(ZONE_MAP_REMOVE_AT, &args![map, zone]);
    let first = e.call(ZONE_MAP_FIRST, &args![map]).u32();
    e.with_stack(12, |e, slots| {
        let position = slots.addr();
        let key = slots.addr() + 4;
        let value = slots.addr() + 8;
        e.mem.set_u32(position, first);
        let mut best = lowest;
        while e.mem.u32(position) != 0 {
            e.call(ZONE_MAP_NEXT, &args![map, position, key, value]);
            let candidate = e.mem.u32(key);
            let height = f32::from_bits(e.mem.u32(candidate + 0x18));
            if best < height {
                best = height;
                e.set(
                    this,
                    ExtraWaterZoneMap::pHighestWaterZone,
                    Ptr::new(candidate),
                );
            }
        }
    });
}

// Translated from 00437970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraIgnoredBySandbox`'s constructor (the engine map has no name for it):
/// extra-data type 0x80, no payload. Returns `this`.
pub fn fn_00437970(e: &mut Engine, this: Ptr) -> Ptr {
    construct_base(
        e,
        this,
        TYPE_IGNORED_BY_SANDBOX,
        EXTRA_IGNORED_BY_SANDBOX_VTABLE,
    );
    this
}

// Translated from 004379a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefInUseData`'s constructor (the engine map has no name for
/// it): extra-data type 0x88; `User` is the handle of `user` (`0084e3a0`), or
/// 0 for a null `user`. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_004379a0(
    e: &mut Engine,
    this: Ptr<ExtraPatrolRefInUseData>,
    user: Ptr,
) -> Ptr<ExtraPatrolRefInUseData> {
    construct_base(
        e,
        this.cast(),
        TYPE_PATROL_REF_IN_USE_DATA,
        EXTRA_PATROL_REF_IN_USE_DATA_VTABLE,
    );
    let handle = if user.is_null() {
        0
    } else {
        e.call(REFERENCE_HANDLE, &args![user]).u32()
    };
    e.set(this, ExtraPatrolRefInUseData::User, handle);
    this
}

// Translated from 00437a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefInUseData::Compare` (Xbox PDB): true when `User` differs.
/// It neither casts `other` nor asks the base compare.
pub fn extra_patrol_ref_in_use_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraPatrolRefInUseData>,
    other: Ptr<ExtraPatrolRefInUseData>,
) -> bool {
    e.get(this, ExtraPatrolRefInUseData::User) != e.get(other, ExtraPatrolRefInUseData::User)
}

// Translated from 00437a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefInUseData`'s scalar deleting destructor (the engine map has
/// no name for it): the destructor, then `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_00437a60(
    e: &mut Engine,
    this: Ptr<ExtraPatrolRefInUseData>,
    flags: u32,
) -> Ptr<ExtraPatrolRefInUseData> {
    fn_00437a90(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00437a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefInUseData`'s destructor body (the engine map has no name
/// for it): resets the vtable, then runs the base destructor.
pub fn fn_00437a90(e: &mut Engine, this: Ptr<ExtraPatrolRefInUseData>) {
    e.mem
        .set_u32(this.addr(), EXTRA_PATROL_REF_IN_USE_DATA_VTABLE);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00437ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefInUseData::GetCanBeUsedBy` (Xbox PDB): true when `user` is
/// the reference that already holds the patrol reference (its handle,
/// `0084e3a0`, equals `User`). Otherwise looks up the reference for `User`
/// (`004839c0`): when it is gone, or virtual slot 0x22c (called with 0) is
/// true, or its flags word has `0x820` (`00437b90`), the hold is released
/// (`User` = 0) and the answer is true. A reference for which virtual slot
/// 0x100 is false answers false. Otherwise the hold is released (answer true)
/// when its kind code (`004f8960`) is 5 or 3, or when it has a current process
/// type (`00931850`, non-zero) and the byte at +0x87 is clear; else false.
pub fn extra_patrol_ref_in_use_data_get_can_be_used_by(
    e: &mut Engine,
    this: Ptr<ExtraPatrolRefInUseData>,
    user: Ptr,
) -> bool {
    if !user.is_null() {
        let handle = e.call(REFERENCE_HANDLE, &args![user]).u32();
        if handle == e.get(this, ExtraPatrolRefInUseData::User) {
            return true;
        }
    }
    let handle = e.get(this, ExtraPatrolRefInUseData::User);
    let reference = e.call(REFERENCE_FOR_HANDLE, &args![handle]).ptr();
    if reference.is_null() {
        e.set(this, ExtraPatrolRefInUseData::User, 0);
        return true;
    }
    if e.vcall(reference.addr(), 0x22c, &args![0u32]).bool() || fn_00437b90(e, reference) {
        e.set(this, ExtraPatrolRefInUseData::User, 0);
        return true;
    }
    if !e.vcall(reference.addr(), 0x100, &args![]).bool() {
        return false;
    }
    let releases = fn_00437bf0(e, reference)
        || fn_00437bd0(e, reference)
        || (e.call(GET_CURRENT_PROCESS_TYPE, &args![reference]).u32() != 0
            && !fn_00437bb0(e, reference));
    if releases {
        e.set(this, ExtraPatrolRefInUseData::User, 0);
        return true;
    }
    false
}

// Translated from 00437b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A predicate on a form that the engine map has no name for: true when the
/// flags word at +0x8 has bit `0x20` or `0x800` set.
pub fn fn_00437b90(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & 0x820 != 0
}

// Translated from 00437bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An accessor that the engine map has no name for: the byte at +0x87 of a
/// reference.
pub fn fn_00437bb0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x87) != 0
}

// Translated from 00437bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A predicate on a reference that the engine map has no name for: true when
/// its kind code (`004f8960`) is 3.
pub fn fn_00437bd0(e: &mut Engine, this: Ptr) -> bool {
    e.call(REFERENCE_KIND, &args![this]).u32() == 3
}

// Translated from 00437bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A predicate on a reference that the engine map has no name for: true when
/// its kind code (`004f8960`) is 5.
pub fn fn_00437bf0(e: &mut Engine, this: Ptr) -> bool {
    e.call(REFERENCE_KIND, &args![this]).u32() == 5
}

// Translated from 00437c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of an `ExtraFollowerSwimBreadcrumb` (0x24 bytes) that the
/// engine map has no name for: runs the folded `NiPoint3` constructor on the
/// two vectors at +0 and +0x10 (it writes nothing). Returns `this`.
pub fn fn_00437c10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINT3_CONSTRUCT, &args![this]);
    e.call(NI_POINT3_CONSTRUCT, &args![this.addr() + 0x10]);
    this
}

// Translated from 00437c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFollowerSwimBreadcrumbs::ExtraFollowerSwimBreadcrumbs` (Xbox PDB):
/// extra-data type 0x8b, `eLeaderState` 0, `LeaderLocation` copied from the
/// three-word exe global at `011f426c`, `LeaderNavMeshID` 0 and the crumb
/// list at +0x20 built in place (`0096a2d0`). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn extra_follower_swim_breadcrumbs_extra_follower_swim_breadcrumbs(
    e: &mut Engine,
    this: Ptr<ExtraFollowerSwimBreadcrumbs>,
) -> Ptr<ExtraFollowerSwimBreadcrumbs> {
    construct_base(
        e,
        this.cast(),
        TYPE_FOLLOWER_SWIM_BREADCRUMBS,
        EXTRA_FOLLOWER_SWIM_BREADCRUMBS_VTABLE,
    );
    e.set(this, ExtraFollowerSwimBreadcrumbs::eLeaderState, 0);
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(DEFAULT_LEADER_LOCATION + offset);
        e.mem.set_u32(this.addr() + 0x10 + offset, word);
    }
    e.set(this, ExtraFollowerSwimBreadcrumbs::LeaderNavMeshID, 0);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![this.addr() + 0x20]);
    this
}

// Translated from 00437ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFollowerSwimBreadcrumbs::_scalar_deleting_destructor_` (Xbox PDB):
/// the destructor, then `operator delete` when `flags & 1`. Returns `this`.
pub fn extra_follower_swim_breadcrumbs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraFollowerSwimBreadcrumbs>,
    flags: u32,
) -> Ptr<ExtraFollowerSwimBreadcrumbs> {
    fn_00437d10(e, this);
    delete_when_asked(e, this.cast(), flags);
    this
}

// Translated from 00437d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraFollowerSwimBreadcrumbs`' destructor body (the engine map has no
/// name for it): resets the vtable; walks every node of the crumb list at
/// +0x20, running the scalar deleting destructor (`007b3fa0`, flag 1) of
/// each non-null crumb and clearing the node's item; then the list's
/// destructor (`0046ffb0`) and the base destructor. The exception-unwinding
/// frame is not translated.
pub fn fn_00437d10(e: &mut Engine, this: Ptr<ExtraFollowerSwimBreadcrumbs>) {
    e.mem
        .set_u32(this.addr(), EXTRA_FOLLOWER_SWIM_BREADCRUMBS_VTABLE);
    let list = this.addr() + 0x20;
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let crumb = e.mem.u32(slot);
        if crumb != 0 {
            e.call(CRUMB_DELETE, &args![crumb, 1u32]);
        }
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        e.mem.set_u32(slot, 0);
        node = node_next(e, node);
    }
    e.call(SIMPLE_LIST_DESTRUCT, &args![list]);
    e.call(BS_EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00437dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An accessor of `ExtraFollowerSwimBreadcrumbs` that the engine map has no
/// name for: the first crumb of the list at +0x20.
pub fn fn_00437dd0(e: &mut Engine, this: Ptr<ExtraFollowerSwimBreadcrumbs>) -> u32 {
    node_item(e, this.addr() + 0x20)
}

// Translated from 00437df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraFollowerSwimBreadcrumbs` that the engine map has no
/// name for: destroys the first crumb (`007b3fa0`, flag 1, when not null),
/// sets the head item of the list at +0x20 to null (`00726c60`) and removes
/// the head node (`0063f7b0`).
pub fn fn_00437df0(e: &mut Engine, this: Ptr<ExtraFollowerSwimBreadcrumbs>) {
    let list = this.addr() + 0x20;
    let crumb = node_item(e, list);
    if crumb != 0 {
        e.call(CRUMB_DELETE, &args![crumb, 1u32]);
    }
    e.with_stack(4, |e, zero| {
        e.mem.set_u32(zero.addr(), 0);
        e.call(SIMPLE_LIST_SET_HEAD, &args![list, zero]);
    });
    e.call(SIMPLE_LIST_REMOVE_HEAD, &args![list]);
}

// Translated from 00437e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of `ExtraFollowerSwimBreadcrumbs` that the engine map has no
/// name for (it records the leader's move): `state` is the new
/// `eLeaderState`, and `x`, `y`, `z` and `nav_mesh` the new leader location
/// and navmesh. Does nothing and returns false when `state` is 0. When
/// `state` differs from the current one and the current one is not 0, a new
/// crumb (0x24 bytes, `00437c10`) is appended to the list (`00905820`) first:
/// it holds the old location (+0), the old navmesh (+0xc), the new location
/// (+0x10), the new navmesh (+0x1c) and whether `state` is 2 (+0x20). In
/// every non-zero case the location, navmesh and state are then stored.
/// Returns whether a crumb was appended. The exception-unwinding frame is not
/// translated.
pub fn fn_00437e50(
    e: &mut Engine,
    this: Ptr<ExtraFollowerSwimBreadcrumbs>,
    state: u32,
    x: u32,
    y: u32,
    z: u32,
    nav_mesh: u32,
) -> bool {
    if state == 0 {
        return false;
    }
    let mut appended = false;
    let current = e.get(this, ExtraFollowerSwimBreadcrumbs::eLeaderState);
    if state != current && current != 0 {
        let memory = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
        let crumb = if memory == 0 {
            0
        } else {
            fn_00437c10(e, Ptr::new(memory)).addr()
        };
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(this.addr() + 0x10 + offset);
            e.mem.set_u32(crumb + offset, word);
        }
        let old_nav_mesh = e.get(this, ExtraFollowerSwimBreadcrumbs::LeaderNavMeshID);
        e.mem.set_u32(crumb + 0xc, old_nav_mesh);
        e.mem.set_u32(crumb + 0x10, x);
        e.mem.set_u32(crumb + 0x14, y);
        e.mem.set_u32(crumb + 0x18, z);
        e.mem.set_u32(crumb + 0x1c, nav_mesh);
        e.mem.set_u8(crumb + 0x20, (state == 2) as u8);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), crumb);
            e.call(CRUMB_LIST_ADD, &args![this.addr() + 0x20, slot]);
        });
        appended = true;
    }
    e.mem.set_u32(this.addr() + 0x10, x);
    e.mem.set_u32(this.addr() + 0x14, y);
    e.mem.set_u32(this.addr() + 0x18, z);
    e.set(
        this,
        ExtraFollowerSwimBreadcrumbs::LeaderNavMeshID,
        nav_mesh,
    );
    e.set(this, ExtraFollowerSwimBreadcrumbs::eLeaderState, state);
    appended
}

// Translated from 00437f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSecuritronFace::ApplyFace` (Xbox PDB): `this` holds two strings
/// (personality at +0xc, mood at +0x14) and `root` is the Securitron's scene
/// object. Nothing happens for a null `root`. Otherwise the object named
/// "Screen01:0" is asked of `root` (virtual slot 0x9c, called with a fixed
/// string); when there is none, nothing more happens. When both strings
/// (`00428070`, `004280b0`) are non-empty, the texture path
/// `Textures\creatures\Securitron\%s_%s.dds` is formatted from them, passed
/// through `SwapPlatformLanguageTexturePath` and loaded
/// (`TES::CreateTextureImage`) into a smart pointer on the stack. Then the
/// screen's property of the type `00438220` gives (3) is asked
/// (`NiAVObject::GetProperty`) and, when it exists, receives the texture
/// (`00438230`), null when none was loaded. The exception-unwinding frame and
/// the stack-cookie check are not translated.
pub fn extra_securitron_face_apply_face(e: &mut Engine, this: Ptr<ExtraSecuritronFace>, root: Ptr) {
    if root.is_null() {
        return;
    }
    e.with_stack(0x228, |e, frame| {
        let base = frame.addr();
        let name = Ptr::new(base);
        let first = base + 4;
        let second = base + 0xc;
        let path = base + 0x14;
        let texture = base + 0x120;
        let localized = base + 0x124;
        let name_text = fixed_string_construct(e, name, SECURITRON_SCREEN_NAME);
        let screen: Ptr = e.vcall(root.addr(), 0x9c, &args![name_text]).ptr();
        fixed_string_destruct(e, name);
        if screen.is_null() {
            return;
        }
        e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
        e.call(GET_FACE_STRING_FIRST, &args![this, first]);
        e.call(GET_FACE_STRING_SECOND, &args![this, second]);
        if e.call(STRING_LENGTH, &args![first]).u32() != 0
            && e.call(STRING_LENGTH, &args![second]).u32() != 0
        {
            let second_text = e.call(POINTER_VALUE, &args![second]).u32();
            let first_text = e.call(POINTER_VALUE, &args![first]).u32();
            e.call(
                FORMAT_STRING,
                &args![
                    path,
                    0x104u32,
                    SECURITRON_TEXTURE_FORMAT,
                    first_text,
                    second_text
                ],
            );
            e.call(
                SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH,
                &args![path, localized, 0x104u32],
            );
            let tes = e.global::<u32>(TES);
            e.call(
                CREATE_TEXTURE_IMAGE,
                &args![tes, localized, texture, 0u32, 0u32],
            );
        }
        let property_type = fn_00438220(e);
        let property: Ptr = e.call(GET_PROPERTY, &args![screen, property_type]).ptr();
        if !property.is_null() {
            let image = e.call(POINTER_VALUE, &args![texture]).u32();
            fn_00438230(e, property, image);
        }
        e.call(STRING_DESTRUCT, &args![second]);
        e.call(STRING_DESTRUCT, &args![first]);
        e.call(NI_POINTER_DESTRUCT, &args![texture]);
    });
}

// Translated from 00438170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a fixed string (`NiFixedString`) that the engine map
/// has no name for: stores in `this` the empty value (the word at `0109b220`)
/// for a null `text`, otherwise the fixed string made for `text`
/// (`00a5b690`). Returns `this`.
pub fn fixed_string_construct(e: &mut Engine, this: Ptr, text: u32) -> Ptr {
    let value = if text == 0 {
        e.mem.u32(EMPTY_FIXED_STRING)
    } else {
        e.call(FIXED_STRING_CREATE, &args![text]).u32()
    };
    e.mem.set_u32(this.addr(), value);
    this
}

// Translated from 004381b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of a fixed string that the engine map has no name for:
/// releases the text (`004381d0`).
pub fn fixed_string_destruct(e: &mut Engine, this: Ptr) {
    fixed_string_release(e, this);
}

// Translated from 004381d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the text of the fixed string at `slot` (cdecl, one stack word;
/// the engine map has no name for it): nothing for the empty value, otherwise
/// `004019a0` with the header address of the text (`00438210`).
pub fn fixed_string_release(e: &mut Engine, slot: Ptr) {
    if e.mem.u32(slot.addr()) == e.mem.u32(EMPTY_FIXED_STRING) {
        return;
    }
    let header = fn_00438210(e, slot);
    e.call(FIXED_STRING_RELEASE, &args![header]);
}

// Translated from 00438210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the header of a fixed string's text that the engine map
/// has no name for (cdecl, one stack word): the text pointer stored at `slot`
/// minus 8.
pub fn fn_00438210(e: &mut Engine, slot: Ptr) -> u32 {
    e.mem.u32(slot.addr()).wrapping_sub(8)
}

// Translated from 00438220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function with no arguments that the engine map has no name for: returns
/// 3 (the property type `ApplyFace` asks for).
pub fn fn_00438220(_e: &mut Engine) -> u32 {
    3
}

// Translated from 00438230 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of the property `ApplyFace` finds that the engine map has no
/// name for: assigns `texture` to the smart pointer at +0x60
/// (`NiPointer::operator=`, `0066b0d0`) and clears the word at +0x38.
pub fn fn_00438230(e: &mut Engine, this: Ptr, texture: u32) {
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0x60, texture]);
    e.mem.set_u32(this.addr() + 0x38, 0);
}

// Translated from 00438260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSecuritronFace::SetPersonality` (Xbox PDB): assigns `text` to
/// `msPersonality` (+0xc, `00438390`).
pub fn extra_securitron_face_set_personality(
    e: &mut Engine,
    this: Ptr<ExtraSecuritronFace>,
    text: u32,
) {
    e.call(STRING_ASSIGN, &args![this.addr() + 0xc, text]);
}

// Translated from 00438280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSecuritronFace::SetMood` (Xbox PDB): assigns `text` to `msMood`
/// (+0x14, `00438390`).
pub fn extra_securitron_face_set_mood(e: &mut Engine, this: Ptr<ExtraSecuritronFace>, text: u32) {
    e.call(STRING_ASSIGN, &args![this.addr() + 0x14, text]);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004353f0,
            extra_x_target_compare(Ptr<ExtraXTarget>, Ptr) -> bool
        ),
        entry!(
            0x00435440,
            fn_00435440(Ptr<ExtraEmittanceSource>) -> Ptr<ExtraEmittanceSource>
        ),
        entry!(
            0x00435470,
            fn_00435470(Ptr<ExtraEmittanceSource>, u32) -> Ptr<ExtraEmittanceSource>
        ),
        entry!(0x004354a0, fn_004354a0(Ptr<ExtraEmittanceSource>)),
        entry!(
            0x004354c0,
            extra_emittance_source_compare(Ptr<ExtraEmittanceSource>, Ptr) -> bool
        ),
        entry!(
            0x00435510,
            fn_00435510(Ptr<ExtraMultiBoundRef>) -> Ptr<ExtraMultiBoundRef>
        ),
        entry!(
            0x00435540,
            extra_multi_bound_ref_compare(Ptr<ExtraMultiBoundRef>, Ptr) -> bool
        ),
        entry!(
            0x00435590,
            fn_00435590(Ptr<ExtraMultiBoundData>) -> Ptr<ExtraMultiBoundData>
        ),
        entry!(
            0x004355c0,
            extra_multi_bound_data_scalar_deleting_destructor(
                Ptr<ExtraMultiBoundData>,
                u32,
            )
                -> Ptr<ExtraMultiBoundData>
        ),
        entry!(0x004355f0, fn_004355f0(Ptr<ExtraMultiBoundData>)),
        entry!(
            0x00435630,
            extra_multi_bound_data_compare(Ptr<ExtraMultiBoundData>, Ptr) -> bool
        ),
        entry!(
            0x00435690,
            fn_00435690(Ptr<ExtraMultiBound>) -> Ptr<ExtraMultiBound>
        ),
        entry!(
            0x00435710,
            extra_multi_bound_scalar_deleting_destructor(
                Ptr<ExtraMultiBound>,
                u32,
            ) -> Ptr<ExtraMultiBound>
        ),
        entry!(0x00435740, fn_00435740(Ptr<ExtraMultiBound>)),
        entry!(
            0x004357a0,
            fn_004357a0(Ptr<ExtraOcclusionPlane>) -> Ptr<ExtraOcclusionPlane>
        ),
        entry!(
            0x00435820,
            fn_00435820(Ptr<ExtraPortal>) -> Ptr<ExtraPortal>
        ),
        entry!(0x004358a0, fn_004358a0(Ptr<ExtraRoom>) -> Ptr<ExtraRoom>),
        entry!(
            0x00435920,
            fn_00435920(Ptr<ExtraItemDropper>) -> Ptr<ExtraItemDropper>
        ),
        entry!(
            0x00435950,
            fn_00435950(Ptr<ExtraDroppedItemList>) -> Ptr<ExtraDroppedItemList>
        ),
        entry!(
            0x004359c0,
            extra_dropped_item_list_scalar_deleting_destructor(
                Ptr<ExtraDroppedItemList>,
                u32,
            )
                -> Ptr<ExtraDroppedItemList>
        ),
        entry!(0x004359f0, fn_004359f0(Ptr<ExtraDroppedItemList>)),
        entry!(
            0x00435a60,
            fn_00435a60(Ptr<ExtraSavedAnimation>) -> Ptr<ExtraSavedAnimation>
        ),
        entry!(
            0x00435a90,
            extra_saved_animation_scalar_deleting_destructor(
                Ptr<ExtraSavedAnimation>,
                u32,
            )
                -> Ptr<ExtraSavedAnimation>
        ),
        entry!(0x00435b20, fn_00435b20(Ptr)),
        entry!(
            0x00435b50,
            fn_00435b50(Ptr<ExtraSavedHavokData>) -> Ptr<ExtraSavedHavokData>
        ),
        entry!(
            0x00435b80,
            extra_saved_havok_data_scalar_deleting_destructor(
                Ptr<ExtraSavedHavokData>,
                u32,
            )
                -> Ptr<ExtraSavedHavokData>
        ),
        entry!(
            0x00435c10,
            extra_saved_animation_delete_buffer(Ptr<ExtraSavedAnimation>)
        ),
        entry!(
            0x00435c40,
            extra_friend_hits_extra_friend_hits(Ptr<ExtraFriendHits>) -> Ptr<ExtraFriendHits>
        ),
        entry!(
            0x00435cb0,
            extra_friend_hits_scalar_deleting_destructor(
                Ptr<ExtraFriendHits>,
                u32,
            ) -> Ptr<ExtraFriendHits>
        ),
        entry!(0x00435ce0, fn_00435ce0(Ptr<ExtraFriendHits>)),
        entry!(0x00435d40, extra_friend_hits_add_hit(Ptr<ExtraFriendHits>)),
        entry!(0x00435dd0, fn_00435dd0() -> f32),
        entry!(0x00435de0, fn_00435de0(Ptr, f32) -> Ptr),
        entry!(0x00435e00, fn_00435e00(Ptr) -> f32),
        entry!(
            0x00435e20,
            extra_friend_hits_get_hit_count(Ptr<ExtraFriendHits>) -> u32
        ),
        entry!(
            0x00435e40,
            extra_friend_hits_remove_old_hits(Ptr<ExtraFriendHits>)
        ),
        entry!(
            0x00435ec0,
            fn_00435ec0(Ptr<ExtraHeadingTarget>, Ptr) -> Ptr<ExtraHeadingTarget>
        ),
        entry!(0x00435ef0, fn_00435ef0(Ptr<ExtraHeadingTarget>)),
        entry!(
            0x00435f20,
            fn_00435f20(Ptr<ExtraRefractionProperty>, f32) -> Ptr<ExtraRefractionProperty>
        ),
        entry!(
            0x00435f50,
            fn_00435f50(Ptr<ExtraRefractionProperty>, u32) -> Ptr<ExtraRefractionProperty>
        ),
        entry!(0x00435f80, fn_00435f80(Ptr<ExtraRefractionProperty>)),
        entry!(
            0x00435fa0,
            fn_00435fa0(Ptr<ExtraEditorRefMoveData>, Ptr) -> Ptr<ExtraEditorRefMoveData>
        ),
        entry!(
            0x00436090,
            fn_00436090(Ptr<ExtraHasNoRumors>, u8) -> Ptr<ExtraHasNoRumors>
        ),
        entry!(
            0x004360c0,
            fn_004360c0(Ptr<ExtraSound>, u32, u32, u32) -> Ptr<ExtraSound>
        ),
        entry!(
            0x00436150,
            fn_00436150(Ptr<ExtraSound>, u32) -> Ptr<ExtraSound>
        ),
        entry!(0x00436180, fn_00436180(Ptr<ExtraSound>)),
        entry!(
            0x004361e0,
            fn_004361e0(
                Ptr<ExtraCreatureAwakeSound>,
                u32,
                u32,
                u32,
            ) -> Ptr<ExtraCreatureAwakeSound>
        ),
        entry!(
            0x00436270,
            fn_00436270(Ptr<ExtraCreatureAwakeSound>, u32) -> Ptr<ExtraCreatureAwakeSound>
        ),
        entry!(0x004362a0, fn_004362a0(Ptr<ExtraCreatureAwakeSound>)),
        entry!(
            0x00436300,
            fn_00436300(
                Ptr<ExtraCreatureMovementSound>,
                u32,
                u32,
                u32,
            ) -> Ptr<ExtraCreatureMovementSound>
        ),
        entry!(
            0x00436390,
            fn_00436390(Ptr<ExtraCreatureMovementSound>, u32) -> Ptr<ExtraCreatureMovementSound>
        ),
        entry!(0x004363c0, fn_004363c0(Ptr<ExtraCreatureMovementSound>)),
        entry!(
            0x00436420,
            fn_00436420(Ptr<ExtraWeaponIdleSound>, u32, u32, u32) -> Ptr<ExtraWeaponIdleSound>
        ),
        entry!(
            0x004364b0,
            fn_004364b0(Ptr<ExtraWeaponIdleSound>, u32) -> Ptr<ExtraWeaponIdleSound>
        ),
        entry!(0x004364e0, fn_004364e0(Ptr<ExtraWeaponIdleSound>)),
        entry!(
            0x00436540,
            fn_00436540(Ptr<ExtraWeaponAttackSound>, u32, u32, u32) -> Ptr<ExtraWeaponAttackSound>
        ),
        entry!(
            0x004365d0,
            fn_004365d0(Ptr<ExtraWeaponAttackSound>, u32) -> Ptr<ExtraWeaponAttackSound>
        ),
        entry!(0x00436600, fn_00436600(Ptr<ExtraWeaponAttackSound>)),
        entry!(
            0x00436660,
            fn_00436660(Ptr<ExtraActivateLoopSound>, u32, u32, u32) -> Ptr<ExtraActivateLoopSound>
        ),
        entry!(
            0x004366f0,
            fn_004366f0(Ptr<ExtraActivateLoopSound>, u32) -> Ptr<ExtraActivateLoopSound>
        ),
        entry!(0x00436720, fn_00436720(Ptr<ExtraActivateLoopSound>)),
        entry!(0x00436780, fn_00436780(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00436aa0, fn_00436aa0(Ptr) -> Ptr),
        entry!(
            0x00436ac0,
            fn_00436ac0(Ptr<ExtraTalkingActor>) -> Ptr<ExtraTalkingActor>
        ),
        entry!(
            0x00436af0,
            fn_00436af0(Ptr<ExtraTalkingActor>, u32) -> Ptr<ExtraTalkingActor>
        ),
        entry!(
            0x00436b20,
            extra_talking_actor_extra_talking_actor(
                Ptr<ExtraTalkingActor>,
                Ptr,
                Ptr,
            ) -> Ptr<ExtraTalkingActor>
        ),
        entry!(0x00436bb0, fn_00436bb0(Ptr<ExtraTalkingActor>)),
        entry!(
            0x00436bd0,
            extra_radius_compare(Ptr<ExtraRadius>, Ptr) -> bool
        ),
        entry!(
            0x00436c40,
            extra_radiation_compare(Ptr<ExtraRadiation>, Ptr) -> bool
        ),
        entry!(
            0x00436cb0,
            fn_00436cb0(Ptr<ExtraFactionChanges>) -> Ptr<ExtraFactionChanges>
        ),
        entry!(
            0x00436d50,
            extra_faction_changes_scalar_deleting_destructor(
                Ptr<ExtraFactionChanges>,
                u32,
            )
                -> Ptr<ExtraFactionChanges>
        ),
        entry!(0x00436d80, fn_00436d80(Ptr<ExtraFactionChanges>)),
        entry!(0x00436e10, fn_00436e10(Ptr<ExtraFactionChanges>, Ptr)),
        entry!(0x00436f20, fn_00436f20(Ptr<ExtraFactionChanges>, Ptr, u8)),
        entry!(0x00436f80, fn_00436f80(Ptr<ExtraFactionChanges>, Ptr, u8)),
        entry!(
            0x00437010,
            extra_faction_changes_get_is_in_faction(Ptr<ExtraFactionChanges>, Ptr) -> bool
        ),
        entry!(
            0x00437080,
            extra_faction_changes_get_is_expelled(Ptr<ExtraFactionChanges>, Ptr) -> bool
        ),
        entry!(0x004370f0, fn_004370f0(Ptr<ExtraFactionChanges>, Ptr)),
        entry!(0x00437240, fn_00437240(Ptr<RADIO_DATA>, Ptr<RADIO_DATA>)),
        entry!(
            0x00437290,
            extra_radio_data_compare(Ptr<ExtraRadioData>, Ptr) -> bool
        ),
        entry!(
            0x00437300,
            extra_combat_style_compare(Ptr<ExtraCombatStyle>, Ptr) -> bool
        ),
        entry!(
            0x00437370,
            extra_primitive_compare(Ptr<ExtraPrimitive>, Ptr) -> bool
        ),
        entry!(0x004373c0, extra_ammo_compare(Ptr<ExtraAmmo>, Ptr) -> bool),
        entry!(
            0x00437440,
            fn_00437440(Ptr<ExtraSayTopicInfoOnceADay>) -> Ptr<ExtraSayTopicInfoOnceADay>
        ),
        entry!(
            0x004374e0,
            extra_say_topic_info_once_a_day_scalar_deleting_destructor(
                Ptr<ExtraSayTopicInfoOnceADay>,
                u32,
            ) -> Ptr<
                ExtraSayTopicInfoOnceADay,
            >
        ),
        entry!(
            0x00437510,
            fn_00437510(Ptr<ExtraSayTopicInfoOnceADay>, u32) -> Ptr<ExtraSayTopicInfoOnceADay>
        ),
        entry!(0x004375c0, fn_004375c0(Ptr<ExtraSayTopicInfoOnceADay>)),
        entry!(
            0x00437690,
            fn_00437690(Ptr<ExtraSayToTopicInfo>) -> Ptr<ExtraSayToTopicInfo>
        ),
        entry!(
            0x004376e0,
            fn_004376e0(Ptr<ExtraSayToTopicInfo>, u32) -> Ptr<ExtraSayToTopicInfo>
        ),
        entry!(0x00437730, fn_00437730(Ptr<ExtraSayToTopicInfo>, u32)),
        entry!(
            0x00437750,
            fn_00437750(Ptr<ExtraWaterZoneMap>) -> Ptr<ExtraWaterZoneMap>
        ),
        entry!(
            0x004377c0,
            extra_water_zone_map_scalar_deleting_destructor(
                Ptr<ExtraWaterZoneMap>,
                u32,
            ) -> Ptr<ExtraWaterZoneMap>
        ),
        entry!(0x004377f0, fn_004377f0(Ptr<ExtraWaterZoneMap>)),
        entry!(0x00437850, fn_00437850(Ptr<ExtraWaterZoneMap>, u32, bool)),
        entry!(0x00437970, fn_00437970(Ptr) -> Ptr),
        entry!(
            0x004379a0,
            fn_004379a0(Ptr<ExtraPatrolRefInUseData>, Ptr) -> Ptr<ExtraPatrolRefInUseData>
        ),
        entry!(
            0x00437a30,
            extra_patrol_ref_in_use_data_compare(
                Ptr<ExtraPatrolRefInUseData>,
                Ptr<ExtraPatrolRefInUseData>,
            ) -> bool
        ),
        entry!(
            0x00437a60,
            fn_00437a60(Ptr<ExtraPatrolRefInUseData>, u32) -> Ptr<ExtraPatrolRefInUseData>
        ),
        entry!(0x00437a90, fn_00437a90(Ptr<ExtraPatrolRefInUseData>)),
        entry!(
            0x00437ab0,
            extra_patrol_ref_in_use_data_get_can_be_used_by(
                Ptr<ExtraPatrolRefInUseData>,
                Ptr,
            ) -> bool
        ),
        entry!(0x00437b90, fn_00437b90(Ptr) -> bool),
        entry!(0x00437bb0, fn_00437bb0(Ptr) -> bool),
        entry!(0x00437bd0, fn_00437bd0(Ptr) -> bool),
        entry!(0x00437bf0, fn_00437bf0(Ptr) -> bool),
        entry!(0x00437c10, fn_00437c10(Ptr) -> Ptr),
        entry!(
            0x00437c40,
            extra_follower_swim_breadcrumbs_extra_follower_swim_breadcrumbs(
                Ptr<ExtraFollowerSwimBreadcrumbs>,
            ) -> Ptr<
                ExtraFollowerSwimBreadcrumbs,
            >
        ),
        entry!(
            0x00437ce0,
            extra_follower_swim_breadcrumbs_scalar_deleting_destructor(
                Ptr<ExtraFollowerSwimBreadcrumbs>,
                u32,
            ) -> Ptr<
                ExtraFollowerSwimBreadcrumbs,
            >
        ),
        entry!(0x00437d10, fn_00437d10(Ptr<ExtraFollowerSwimBreadcrumbs>)),
        entry!(
            0x00437dd0,
            fn_00437dd0(Ptr<ExtraFollowerSwimBreadcrumbs>) -> u32
        ),
        entry!(0x00437df0, fn_00437df0(Ptr<ExtraFollowerSwimBreadcrumbs>)),
        entry!(
            0x00437e50,
            fn_00437e50(Ptr<ExtraFollowerSwimBreadcrumbs>, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00437f90,
            extra_securitron_face_apply_face(Ptr<ExtraSecuritronFace>, Ptr)
        ),
        entry!(0x00438170, fixed_string_construct(Ptr, u32) -> Ptr),
        entry!(0x004381b0, fixed_string_destruct(Ptr)),
        entry!(0x004381d0, fixed_string_release(Ptr)),
        entry!(0x00438210, fn_00438210(Ptr) -> u32),
        entry!(0x00438220, fn_00438220() -> u32),
        entry!(0x00438230, fn_00438230(Ptr, u32)),
        entry!(
            0x00438260,
            extra_securitron_face_set_personality(Ptr<ExtraSecuritronFace>, u32)
        ),
        entry!(
            0x00438280,
            extra_securitron_face_set_mood(Ptr<ExtraSecuritronFace>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Test doubles for the callees every function here shares: the
    /// allocator's `operator delete`, the base constructor (stores the type
    /// byte at +4) and destructor, the dynamic cast (the object itself, so a
    /// null `other` stays null), and no-op stand-ins for the member
    /// constructors and destructors; the pages holding the exe globals the
    /// hit functions read are mapped.
    fn extra_engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(BS_EXTRA_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            ret(a[0])
        });
        e.register(BS_EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        for noop in [
            NI_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            NI_POINTER_DESTRUCT,
            SIMPLE_LIST_CONSTRUCT,
            SIMPLE_LIST_CLEAR,
            SIMPLE_LIST_DESTRUCT,
            HITS_ARRAY_CONSTRUCT,
            HITS_ARRAY_DESTRUCT,
            EXTRA_SAVED_ANIMATION_DESTRUCT,
            EXTRA_SAVED_HAVOK_DATA_DESTRUCT,
            EXTRA_REFRACTION_PROPERTY_DESTRUCT,
            SOUND_HANDLE_DESTRUCT,
        ] {
            e.register(noop, |_, _| Ret::default());
        }
        for page in [0x011c_d000, 0x011f_1000] {
            e.map(page, 0x1000);
        }
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

    /// A block of `size` bytes whose vtable word and payload word hold
    /// garbage, so a constructor's stores show.
    fn dirty_object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, 0xdead_0001);
        e.mem.set_u32(object + 0xc, 0xdead_0002);
        object
    }

    fn vtable_of(e: &Engine, object: u32) -> u32 {
        e.mem.u32(object)
    }

    fn extra_type(e: &Engine, object: u32) -> u8 {
        e.mem.u8(object + 4)
    }

    /// A constructor that builds an empty payload (null word at +0xc).
    fn check_null_payload_constructor(addr: u32, extra: u8, vtable: u32) {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, BS_EXTRA_DATA_CONSTRUCT),
            vec![vec![this, extra as u32]]
        );
        assert_eq!(extra_type(&e, this), extra);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    /// A constructor that builds an empty `NiPointer` at +0xc.
    fn check_ni_pointer_constructor(addr: u32, extra: u8, vtable: u32) {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), extra);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(calls(&e, NI_POINTER_CONSTRUCT), vec![vec![this + 0xc, 0]]);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![this + 0xc, 0]]);
    }

    /// A scalar deleting destructor: runs the destructor (the log shows the
    /// call at `inner_call`) and frees the object only when bit 0 of the
    /// flags is set.
    fn check_scalar_deleting(addr: u32, inner_call: u32) {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, inner_call), vec![vec![this]]);
        assert!(e.mem.block_size(this).is_some());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(addr, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.block_size(this), None);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    /// A `Compare` of a one-word class: the answers for a null other, an
    /// equal payload word and a different one.
    fn check_word_compare(addr: u32, target_type: u32) {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        let same = dirty_object(&mut e, 0x10);
        let other = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_u32(same + 0xc, 0x1234);
        e.mem.set_u32(other + 0xc, 0x5678);
        start_log(&mut e);
        assert!(e.call(addr, &args![this, 0u32]).bool());
        assert!(!e.call(addr, &args![this, same]).bool());
        assert!(e.call(addr, &args![this, other]).bool());
        let casts = calls(&e, DYNAMIC_CAST);
        assert_eq!(casts.len(), 3);
        assert_eq!(casts[1], vec![same, 0, BS_EXTRA_DATA_TYPE, target_type, 0]);
    }

    #[test]
    fn x_target_compare_asks_only_the_target() {
        check_word_compare(0x0043_53f0, EXTRA_X_TARGET_TYPE);
    }

    #[test]
    fn emittance_source_constructor_has_no_source() {
        check_null_payload_constructor(0x0043_5440, 0x67, EXTRA_EMITTANCE_SOURCE_VTABLE);
    }

    #[test]
    fn emittance_source_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_5470, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn emittance_source_destructor_resets_the_vtable() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        e.call(0x0043_54a0, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_EMITTANCE_SOURCE_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // The source is not owned.
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_0002);
    }

    #[test]
    fn emittance_source_compare_asks_only_the_source() {
        check_word_compare(0x0043_54c0, EXTRA_EMITTANCE_SOURCE_TYPE);
    }

    #[test]
    fn multi_bound_ref_constructor_has_no_reference() {
        check_null_payload_constructor(0x0043_5510, 0x63, EXTRA_MULTI_BOUND_REF_VTABLE);
    }

    #[test]
    fn multi_bound_ref_compare_asks_only_the_reference() {
        check_word_compare(0x0043_5540, EXTRA_MULTI_BOUND_REF_TYPE);
    }

    #[test]
    fn multi_bound_data_constructor_has_no_data() {
        check_null_payload_constructor(0x0043_5590, 0x62, EXTRA_MULTI_BOUND_DATA_VTABLE);
    }

    #[test]
    fn multi_bound_data_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        let bound = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, bound);
        assert_eq!(e.call(0x0043_55c0, &args![this, 0u32]).u32(), this);
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(e.mem.block_size(bound), None);
        let again = e.mem.alloc(0x10);
        let other_bound = e.mem.alloc(8);
        e.mem.set_u32(again + 0xc, other_bound);
        assert_eq!(e.call(0x0043_55c0, &args![again, 1u32]).u32(), again);
        assert_eq!(e.mem.block_size(again), None);
        assert_eq!(e.mem.block_size(other_bound), None);
    }

    #[test]
    fn multi_bound_data_destructor_deletes_the_bound_data() {
        let mut e = extra_engine();
        let this = e.mem.alloc(0x10);
        let bound = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, bound);
        start_log(&mut e);
        e.call(0x0043_55f0, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_MULTI_BOUND_DATA_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![bound]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn multi_bound_data_compare_defers_to_the_marker_data() {
        let mut e = extra_engine();
        // The marker data compare double answers "differ" when the words do.
        e.register(MULTI_BOUND_MARKER_DATA_COMPARE, |_, a| {
            ret((a[0] != a[1]) as u32)
        });
        let this = dirty_object(&mut e, 0x10);
        let other = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 0x100);
        e.mem.set_u32(other + 0xc, 0x100);
        start_log(&mut e);
        assert!(e.call(0x0043_5630, &args![this, 0u32]).bool());
        assert!(calls(&e, MULTI_BOUND_MARKER_DATA_COMPARE).is_empty());
        assert!(!e.call(0x0043_5630, &args![this, other]).bool());
        e.mem.set_u32(other + 0xc, 0x200);
        assert!(e.call(0x0043_5630, &args![this, other]).bool());
        assert_eq!(
            calls(&e, MULTI_BOUND_MARKER_DATA_COMPARE),
            vec![vec![0x100, 0x100], vec![0x100, 0x200]]
        );
        let casts = calls(&e, DYNAMIC_CAST);
        assert_eq!(
            casts[1],
            vec![other, 0, BS_EXTRA_DATA_TYPE, EXTRA_MULTI_BOUND_DATA_TYPE, 0]
        );
    }

    #[test]
    fn multi_bound_constructor_builds_an_empty_ni_pointer() {
        check_ni_pointer_constructor(0x0043_5690, 0x61, EXTRA_MULTI_BOUND_VTABLE);
    }

    #[test]
    fn multi_bound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_5710, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn multi_bound_destructor_destroys_the_ni_pointer_and_keeps_the_vtable() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        e.call(0x0043_5740, &args![this]);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(vtable_of(&e, this), 0xdead_0001);
    }

    #[test]
    fn occlusion_plane_constructor_builds_an_empty_ni_pointer() {
        check_ni_pointer_constructor(0x0043_57a0, 0x71, EXTRA_OCCLUSION_PLANE_VTABLE);
    }

    #[test]
    fn portal_constructor_builds_an_empty_ni_pointer() {
        check_ni_pointer_constructor(0x0043_5820, 0x78, EXTRA_PORTAL_VTABLE);
    }

    #[test]
    fn room_constructor_builds_an_empty_ni_pointer() {
        check_ni_pointer_constructor(0x0043_58a0, 0x79, EXTRA_ROOM_VTABLE);
    }

    #[test]
    fn item_dropper_constructor_has_no_dropper() {
        check_null_payload_constructor(0x0043_5920, 0x39, EXTRA_ITEM_DROPPER_VTABLE);
    }

    #[test]
    fn dropped_item_list_constructor_builds_the_list() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x14);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_5950, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x3a);
        assert_eq!(vtable_of(&e, this), EXTRA_DROPPED_ITEM_LIST_VTABLE);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![this + 0xc]]);
    }

    #[test]
    fn dropped_item_list_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_59c0, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn dropped_item_list_destructor_clears_then_destroys_the_list() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x14);
        start_log(&mut e);
        e.call(0x0043_59f0, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_DROPPED_ITEM_LIST_VTABLE);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0043_59f0,
                SIMPLE_LIST_CLEAR,
                SIMPLE_LIST_DESTRUCT,
                BS_EXTRA_DATA_DESTRUCT
            ]
        );
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0xc]]);
    }

    #[test]
    fn saved_animation_constructor_has_no_buffer() {
        check_null_payload_constructor(0x0043_5a60, 0x42, EXTRA_SAVED_ANIMATION_VTABLE);
    }

    #[test]
    fn saved_animation_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_5a90, EXTRA_SAVED_ANIMATION_DESTRUCT);
    }

    #[test]
    fn pointer_holder_release_deletes_the_buffer_and_clears_the_holder() {
        let mut e = extra_engine();
        let holder = e.mem.alloc(4);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(holder, buffer);
        start_log(&mut e);
        e.call(0x0043_5b20, &args![holder]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![buffer]]);
        assert_eq!(e.mem.block_size(buffer), None);
        assert_eq!(e.mem.u32(holder), 0);
    }

    #[test]
    fn saved_havok_data_constructor_has_no_buffer() {
        check_null_payload_constructor(0x0043_5b50, 0x3d, EXTRA_SAVED_HAVOK_DATA_VTABLE);
    }

    #[test]
    fn saved_havok_data_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_5b80, EXTRA_SAVED_HAVOK_DATA_DESTRUCT);
    }

    #[test]
    fn saved_animation_delete_buffer_frees_the_buffer_but_keeps_the_pointer() {
        let mut e = extra_engine();
        e.register(HOLDER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let this = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(this + 0xc, buffer);
        start_log(&mut e);
        e.call(0x0043_5c10, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![buffer]]);
        assert_eq!(e.mem.block_size(buffer), None);
        assert_eq!(e.mem.u32(this + 0xc), buffer);
    }

    #[test]
    fn friend_hits_constructor_builds_the_array() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x1c);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_5c40, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x45);
        assert_eq!(vtable_of(&e, this), EXTRA_FRIEND_HITS_VTABLE);
        assert_eq!(calls(&e, HITS_ARRAY_CONSTRUCT), vec![vec![this + 0xc]]);
    }

    #[test]
    fn friend_hits_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_5cb0, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn friend_hits_destructor_destroys_the_array_and_keeps_the_vtable() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x1c);
        start_log(&mut e);
        e.call(0x0043_5ce0, &args![this]);
        assert_eq!(calls(&e, HITS_ARRAY_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(vtable_of(&e, this), 0xdead_0001);
    }

    const ADD_SETTING: u32 = HIT_SETTING_ADD;
    const REMOVE_SETTING: u32 = HIT_SETTING_REMOVE;

    /// The engine for the hit functions: `BSSimpleArray` accessors on the
    /// layout (buffer at +4, count at +8), an element removal that shifts the
    /// tail down, an append, and a float setting accessor that returns the
    /// setting address itself (the test stores the value there).
    fn hits_engine(clock: f32, add_setting: f32, remove_setting: f32) -> Engine {
        let mut e = extra_engine();
        e.register(SIMPLE_ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SIMPLE_ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(SIMPLE_ARRAY_REMOVE_AT, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            let count = e.mem.u32(a[0] + 8);
            for i in a[1]..count - 1 {
                let next = e.mem.u32(buffer + 4 * (i + 1));
                e.mem.set_u32(buffer + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 8, count - 1);
            Ret::default()
        });
        e.register(SIMPLE_ARRAY_APPEND, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            let count = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * count, value);
            e.mem.set_u32(a[0] + 8, count + 1);
            ret(count)
        });
        e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0]));
        e.set_global(HIT_CLOCK, clock);
        e.set_global(ADD_SETTING, add_setting);
        e.set_global(REMOVE_SETTING, remove_setting);
        e
    }

    /// An `ExtraFriendHits` whose array holds `stamps` (room for 8).
    fn friend_hits_with(e: &mut Engine, stamps: &[f32]) -> u32 {
        let this = e.mem.alloc(0x1c);
        let buffer = e.mem.alloc(32);
        for (i, stamp) in stamps.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, stamp.to_bits());
        }
        e.mem.set_u32(this + 0x10, buffer);
        e.mem.set_u32(this + 0x14, stamps.len() as u32);
        this
    }

    fn stamps_of(e: &Engine, this: u32) -> Vec<f32> {
        let buffer = e.mem.u32(this + 0x10);
        (0..e.mem.u32(this + 0x14))
            .map(|i| f32::from_bits(e.mem.u32(buffer + 4 * i)))
            .collect()
    }

    #[test]
    fn friend_hits_add_hit_records_the_first_hit() {
        let mut e = hits_engine(100.0, 5.0, 1000.0);
        let this = friend_hits_with(&mut e, &[]);
        e.call(0x0043_5d40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![100.0]);
    }

    #[test]
    fn friend_hits_add_hit_ignores_a_hit_right_after_the_last() {
        let mut e = hits_engine(100.0, 5.0, 1000.0);
        let this = friend_hits_with(&mut e, &[98.0]);
        e.call(0x0043_5d40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![98.0]);
    }

    #[test]
    fn friend_hits_add_hit_records_when_the_last_is_old_enough() {
        let mut e = hits_engine(100.0, 5.0, 1000.0);
        let this = friend_hits_with(&mut e, &[90.0, 95.0]);
        // Age exactly equal to the setting counts as old enough.
        e.call(0x0043_5d40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![90.0, 95.0, 100.0]);
        let again = friend_hits_with(&mut e, &[90.0, 80.0]);
        e.call(0x0043_5d40, &args![again]);
        assert_eq!(stamps_of(&e, again), vec![90.0, 80.0, 100.0]);
    }

    #[test]
    fn friend_hits_add_hit_drops_old_hits_first() {
        let mut e = hits_engine(100.0, 5.0, 20.0);
        let this = friend_hits_with(&mut e, &[10.0, 99.0]);
        e.call(0x0043_5d40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![99.0]);
        let only_old = friend_hits_with(&mut e, &[10.0]);
        e.call(0x0043_5d40, &args![only_old]);
        assert_eq!(stamps_of(&e, only_old), vec![100.0]);
    }

    #[test]
    fn friend_hits_add_hit_records_when_the_comparison_is_unordered() {
        let mut e = hits_engine(100.0, f32::NAN, 1000.0);
        let this = friend_hits_with(&mut e, &[99.0]);
        e.call(0x0043_5d40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![99.0, 100.0]);
    }

    #[test]
    fn hit_clock_is_the_global_float() {
        let mut e = hits_engine(12.5, 0.0, 0.0);
        assert_eq!(e.call(0x0043_5dd0, &args![]).f32(), 12.5);
    }

    #[test]
    fn time_stamp_constructor_stores_the_float() {
        let mut e = hits_engine(0.0, 0.0, 0.0);
        let stamp = e.mem.alloc(4);
        assert_eq!(e.call(0x0043_5de0, &args![stamp, 3.25f32]).u32(), stamp);
        assert_eq!(f32::from_bits(e.mem.u32(stamp)), 3.25);
    }

    #[test]
    fn time_stamp_age_is_the_clock_minus_the_stamp_in_float() {
        let mut e = hits_engine(100.0, 0.0, 0.0);
        let stamp = e.mem.alloc(4);
        e.mem.set_u32(stamp, 98.5f32.to_bits());
        assert_eq!(e.call(0x0043_5e00, &args![stamp]).f32(), 1.5);
        // The difference is rounded to float: 2^24 + 1 is not representable.
        e.set_global(HIT_CLOCK, 16_777_216.0f32);
        e.mem.set_u32(stamp, (-1.0f32).to_bits());
        assert_eq!(e.call(0x0043_5e00, &args![stamp]).f32(), 16_777_216.0);
    }

    #[test]
    fn friend_hits_get_hit_count_counts_after_dropping_old_hits() {
        let mut e = hits_engine(100.0, 5.0, 20.0);
        let this = friend_hits_with(&mut e, &[10.0, 85.0, 99.0]);
        assert_eq!(e.call(0x0043_5e20, &args![this]).u32(), 2);
        assert_eq!(stamps_of(&e, this), vec![85.0, 99.0]);
        let empty = friend_hits_with(&mut e, &[]);
        assert_eq!(e.call(0x0043_5e20, &args![empty]).u32(), 0);
    }

    #[test]
    fn friend_hits_remove_old_hits_removes_only_the_older_than_the_setting() {
        let mut e = hits_engine(100.0, 5.0, 20.0);
        // Ages 90, 5, 20, 1, 60: the first, the last and none in between
        // (age 20 equals the setting and stays); consecutive old hits are
        // all removed because the index is looked at again after a removal.
        let this = friend_hits_with(&mut e, &[10.0, 95.0, 80.0, 99.0, 40.0]);
        e.call(0x0043_5e40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![95.0, 80.0, 99.0]);
        let run = friend_hits_with(&mut e, &[1.0, 2.0, 3.0, 99.0]);
        e.call(0x0043_5e40, &args![run]);
        assert_eq!(stamps_of(&e, run), vec![99.0]);
    }

    #[test]
    fn friend_hits_remove_old_hits_keeps_everything_when_unordered() {
        let mut e = hits_engine(100.0, 5.0, f32::NAN);
        let this = friend_hits_with(&mut e, &[1.0, 2.0]);
        e.call(0x0043_5e40, &args![this]);
        assert_eq!(stamps_of(&e, this), vec![1.0, 2.0]);
    }

    #[test]
    fn heading_target_constructor_stores_the_target() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_5ec0, &args![this, 0x4321u32]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x46);
        assert_eq!(vtable_of(&e, this), EXTRA_HEADING_TARGET_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x4321);
    }

    #[test]
    fn heading_target_destructor_clears_the_target() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        e.call(0x0043_5ef0, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_HEADING_TARGET_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn refraction_property_constructor_stores_the_power() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_5f20, &args![this, 0.75f32]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x48);
        assert_eq!(vtable_of(&e, this), EXTRA_REFRACTION_PROPERTY_VTABLE);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0xc)), 0.75);
    }

    #[test]
    fn refraction_property_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_5f50, EXTRA_REFRACTION_PROPERTY_DESTRUCT);
    }

    // ---- the session of 00435f80 to 00437290 ----

    /// A fake `BSSoundHandle` constructor/copy/destructor, the identity
    /// `NiPoint3` constructor and list node accessor, the node-next accessor,
    /// a list that appends, and the constructors that return their block.
    fn session_engine() -> Engine {
        let mut e = extra_engine();
        e.register(SOUND_HANDLE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xffff_ffff);
            e.mem.set_u8(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0);
            ret(a[0])
        });
        e.register(SOUND_HANDLE_ASSIGN, |e, a| {
            let id = e.mem.u32(a[1]);
            let assume = e.mem.u8(a[1] + 4);
            let state = e.mem.u32(a[1] + 8);
            e.mem.set_u32(a[0], id);
            e.mem.set_u8(a[0] + 4, assume);
            e.mem.set_u32(a[0] + 8, state);
            ret(a[0])
        });
        e.register(SOUND_HANDLE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_POINT3_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SIMPLE_LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(MOBILE_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SIMPLE_LIST_ADD_HEAD, |e, a| {
            // Appends: the head node takes the first item, later items get
            // new nodes at the tail.
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
                return Ret::default();
            }
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, item);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        e
    }

    fn word_slots(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x300);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    #[test]
    fn refraction_property_destructor_resets_the_vtable() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        fn_00435f80(&mut e, Ptr::new(this));
        assert_eq!(vtable_of(&e, this), EXTRA_REFRACTION_PROPERTY_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn editor_ref_move_data_without_a_reference_only_constructs_the_vectors() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x30);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_5fa0, &args![this, 0u32]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x4c);
        assert_eq!(vtable_of(&e, this), EXTRA_EDITOR_REF_MOVE_DATA_VTABLE);
        assert_eq!(
            calls(&e, NI_POINT3_CONSTRUCT),
            vec![vec![this + 0xc], vec![this + 0x18], vec![this + 0x24]]
        );
        // Nothing else was stored: the payload keeps what it held.
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_0002);
    }

    #[test]
    fn editor_ref_move_data_copies_the_reference_vectors() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x30);
        let reference = e.mem.alloc(0x40);
        for (i, word) in [1u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(reference + 0x24 + 4 * i as u32, *word);
        }
        let location = e.mem.alloc(12);
        for (i, word) in [7u32, 8, 9].iter().enumerate() {
            e.mem.set_u32(location + 4 * i as u32, *word);
        }
        e.register_double(0x00f0_0001, move |_, _| ret(location));
        word_slots(&mut e, reference, &[(0x1f4, 0x00f0_0001)]);
        e.call(0x0043_5fa0, &args![this, reference]);
        let words = |e: &Engine, at: u32| (0..3).map(|i| e.mem.u32(at + 4 * i)).collect::<Vec<_>>();
        assert_eq!(words(&e, this + 0xc), vec![1, 2, 3]);
        assert_eq!(words(&e, this + 0x18), vec![7, 8, 9]);
        assert_eq!(words(&e, this + 0x24), vec![7, 8, 9]);
    }

    #[test]
    fn has_no_rumors_constructor_stores_the_byte() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        assert_eq!(
            e.call(0x0043_6090, &args![this, 0xffff_ff01u32]).u32(),
            this
        );
        assert_eq!(extra_type(&e, this), 0x4e);
        assert_eq!(vtable_of(&e, this), EXTRA_HAS_NO_RUMORS_VTABLE);
        // Only the byte at +0xc is written.
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_0001);
    }

    /// A sound-holder constructor: base constructor, default handle, copy of
    /// the argument handle, destructor of the copy.
    fn check_sound_constructor(addr: u32, extra: u8, vtable: u32) {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x18);
        start_log(&mut e);
        assert_eq!(e.call(addr, &args![this, 77u32, 0x1u32, 3u32]).u32(), this);
        assert_eq!(extra_type(&e, this), extra);
        assert_eq!(vtable_of(&e, this), vtable);
        assert_eq!(e.mem.u32(this + 0xc), 77);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCT), vec![vec![this + 0xc]]);
        let assigns = calls(&e, SOUND_HANDLE_ASSIGN);
        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0][0], this + 0xc);
        assert_eq!(calls(&e, SOUND_HANDLE_DESTRUCT), vec![vec![assigns[0][1]]]);
    }

    /// A sound-holder destructor body: the vtable (when stored), the handle's
    /// destructor, then the base destructor.
    fn check_sound_destructor(addr: u32, vtable: Option<u32>) {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x18);
        start_log(&mut e);
        e.call(addr, &args![this]);
        assert_eq!(
            vtable_of(&e, this),
            vtable.unwrap_or(0xdead_0001),
            "vtable word"
        );
        assert_eq!(calls(&e, SOUND_HANDLE_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn sound_constructor_copies_the_handle() {
        check_sound_constructor(0x0043_60c0, 0x4f, EXTRA_SOUND_VTABLE);
    }

    #[test]
    fn sound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_6150, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn sound_destructor_resets_the_vtable() {
        check_sound_destructor(0x0043_6180, Some(EXTRA_SOUND_VTABLE));
    }

    #[test]
    fn creature_awake_sound_constructor_copies_the_handle() {
        check_sound_constructor(0x0043_61e0, 0x7d, EXTRA_CREATURE_AWAKE_SOUND_VTABLE);
    }

    #[test]
    fn creature_awake_sound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_6270, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn creature_awake_sound_destructor_resets_the_vtable() {
        check_sound_destructor(0x0043_62a0, Some(EXTRA_CREATURE_AWAKE_SOUND_VTABLE));
    }

    #[test]
    fn creature_movement_sound_constructor_copies_the_handle() {
        check_sound_constructor(0x0043_6300, 0x8a, EXTRA_CREATURE_MOVEMENT_SOUND_VTABLE);
    }

    #[test]
    fn creature_movement_sound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_6390, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn creature_movement_sound_destructor_keeps_the_vtable() {
        check_sound_destructor(0x0043_63c0, None);
    }

    #[test]
    fn weapon_idle_sound_constructor_copies_the_handle() {
        check_sound_constructor(0x0043_6420, 0x83, EXTRA_WEAPON_IDLE_SOUND_VTABLE);
    }

    #[test]
    fn weapon_idle_sound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_64b0, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn weapon_idle_sound_destructor_resets_the_vtable() {
        check_sound_destructor(0x0043_64e0, Some(EXTRA_WEAPON_IDLE_SOUND_VTABLE));
    }

    #[test]
    fn weapon_attack_sound_constructor_copies_the_handle() {
        check_sound_constructor(0x0043_6540, 0x86, EXTRA_WEAPON_ATTACK_SOUND_VTABLE);
    }

    #[test]
    fn weapon_attack_sound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_65d0, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn weapon_attack_sound_destructor_resets_the_vtable() {
        check_sound_destructor(0x0043_6600, Some(EXTRA_WEAPON_ATTACK_SOUND_VTABLE));
    }

    #[test]
    fn activate_loop_sound_constructor_copies_the_handle() {
        check_sound_constructor(0x0043_6660, 0x87, EXTRA_ACTIVATE_LOOP_SOUND_VTABLE);
    }

    #[test]
    fn activate_loop_sound_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_66f0, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn activate_loop_sound_destructor_resets_the_vtable() {
        check_sound_destructor(0x0043_6720, Some(EXTRA_ACTIVATE_LOOP_SOUND_VTABLE));
    }

    /// The doubles `00436780` needs: the accessors read the fields the real
    /// ones read, the mutators record their arguments in the call log.
    fn talking_engine() -> Engine {
        let mut e = session_engine();
        e.map(PLAYER & !0xfff, 0x1000);
        e.register(GET_PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(CELL_CHECK, |e, a| ret(e.mem.u8(a[0] + 0x24) as u32 & 1));
        e.register(GET_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        e.register(GET_PROCESS_KIND, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(GET_EXTRA_LIST, |_, a| ret(a[0] + 0x44));
        e.register(GET_REF_PERSISTS, |e, a| ret(e.mem.u8(a[0] + 0x7f) as u32));
        // Every new object gets a vtable whose slot 0x228 (the cell setter)
        // is the recording stand-in 0x00f00002.
        e.register(MOBILE_OBJECT_CONSTRUCT, |e, a| {
            let vtable = e.mem.alloc(0x300);
            e.mem.set_u32(vtable + 0x228, 0x00f0_0002);
            e.mem.set_u32(a[0], vtable);
            ret(a[0])
        });
        for process in [
            HIGH_PROCESS_CONSTRUCT,
            PROCESS_CONSTRUCT_25C,
            PROCESS_CONSTRUCT_0C8,
            PROCESS_CONSTRUCT_0B4,
        ] {
            e.register_double(process, move |e, a| {
                // The kind word at +0x28 of the new process is its address
                // plus nothing: tests tell the processes apart by the log.
                e.mem.set_u32(a[0] + 0x28, process);
                ret(a[0])
            });
        }
        for noop in [
            SET_OBJECT_REFERENCE,
            SET_REF_PERSISTS,
            SET_LOCATION_ON_REFERENCE,
            SET_ROTATION,
            SET_SOURCE_REFERENCE,
            SET_FLAG_BYTE_81,
            OBJECT_INIT,
            SET_PERSISTENT_CELL,
            PROCESS_LISTS_ADD_REFERENCE,
            0x00f0_0002,
        ] {
            e.register(noop, |_, _| Ret::default());
        }
        e.register(SET_PROCESS, |e, a| {
            e.mem.set_u32(a[0] + 0x68, a[1]);
            Ret::default()
        });
        e
    }

    /// A source reference: rotation at +0x24, parent cell at +0x40, the
    /// child-cell base at +0x18 whose slot 0 gives `persistent`, slot 0x1f4
    /// of the object gives `location`.
    fn source_reference(e: &mut Engine, cell: u32, persistent: u32, location: u32) -> u32 {
        let source = e.mem.alloc(0x80);
        e.mem.set_u32(source + 0x24, 0x1111);
        e.mem.set_u32(source + 0x28, 0x2222);
        e.mem.set_u32(source + 0x2c, 0x3333);
        e.mem.set_u32(source + 0x40, cell);
        e.mem.set_u8(source + 0x7f, 1);
        e.register_double(0x00f0_0010, move |_, _| ret(persistent));
        e.register_double(0x00f0_0011, move |_, _| ret(location));
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(vtable + 0x1f4, 0x00f0_0011);
        e.mem.set_u32(source, vtable);
        let child_vtable = e.mem.alloc(8);
        e.mem.set_u32(child_vtable, 0x00f0_0010);
        e.mem.set_u32(source + 0x18, child_vtable);
        source
    }

    /// The player with a process of `kind` (or none).
    fn set_player_process(e: &mut Engine, kind: Option<u32>) {
        let player = e.mem.alloc(0x80);
        e.mem.set_u32(PLAYER, player);
        if let Some(kind) = kind {
            let process = e.mem.alloc(0x40);
            e.mem.set_u32(process + 0x28, kind);
            e.mem.set_u32(player + 0x68, process);
        }
    }

    /// The object `00436780` made: the block `operator new(0x88)` gave.
    fn made_object(e: &Engine) -> u32 {
        let (_, words) = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(called, _)| *called == MOBILE_OBJECT_CONSTRUCT)
            .expect("the object is constructed");
        words[0]
    }

    #[test]
    fn talking_object_of_a_null_source_is_null() {
        let mut e = talking_engine();
        start_log(&mut e);
        assert_eq!(e.call(0x0043_6780, &args![0u32, 0x55u32, 0u32]).u32(), 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn talking_object_copies_the_source_and_uses_its_cell() {
        let mut e = talking_engine();
        set_player_process(&mut e, None);
        let source = source_reference(&mut e, 0x5000, 0, 0);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u8(cell + 0x24, 1);
        e.mem.set_u32(source + 0x40, cell);
        start_log(&mut e);
        let object = e.call(0x0043_6780, &args![0u32, 0x55u32, source]).u32();
        assert_eq!(object, made_object(&e));
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x88]]);
        assert_eq!(calls(&e, SET_OBJECT_REFERENCE), vec![vec![object, 0x55]]);
        assert_eq!(calls(&e, SET_REF_PERSISTS), vec![vec![object, 1]]);
        assert_eq!(
            calls(&e, SET_LOCATION_ON_REFERENCE),
            vec![vec![object, source + 0x30]]
        );
        assert_eq!(
            calls(&e, SET_ROTATION),
            vec![vec![object, 0x1111, 0x2222, 0x3333]]
        );
        assert_eq!(calls(&e, SET_SOURCE_REFERENCE), vec![vec![object, source]]);
        assert_eq!(calls(&e, SET_FLAG_BYTE_81), vec![vec![object, 1]]);
        assert_eq!(calls(&e, OBJECT_INIT), vec![vec![object]]);
        // The cell is asked three times, approved, and given to slot 0x228.
        assert_eq!(calls(&e, GET_PARENT_CELL).len(), 3);
        assert_eq!(calls(&e, CELL_CHECK), vec![vec![cell]]);
        assert!(calls(&e, SET_PERSISTENT_CELL).is_empty());
    }

    #[test]
    fn talking_object_takes_the_persistent_cell_when_the_cell_is_refused() {
        let mut e = talking_engine();
        set_player_process(&mut e, None);
        let source = source_reference(&mut e, 0, 0x7777, 0);
        // The new object's virtual table must exist for the cell store; here
        // the persistent cell is used instead.
        start_log(&mut e);
        let object = e.call(0x0043_6780, &args![0u32, 0x55u32, source]).u32();
        assert_eq!(calls(&e, GET_PARENT_CELL).len(), 1);
        assert!(calls(&e, CELL_CHECK).is_empty());
        assert_eq!(calls(&e, GET_EXTRA_LIST), vec![vec![object]]);
        assert_eq!(
            calls(&e, SET_PERSISTENT_CELL),
            vec![vec![object + 0x44, 0x7777]]
        );
    }

    #[test]
    fn talking_object_falls_back_to_the_cell_when_nothing_is_persistent() {
        let mut e = talking_engine();
        set_player_process(&mut e, None);
        let source = source_reference(&mut e, 0, 0, 0);
        start_log(&mut e);
        let object = e.call(0x0043_6780, &args![0u32, 0x55u32, source]).u32();
        assert!(object != 0);
        assert!(calls(&e, SET_PERSISTENT_CELL).is_empty());
        assert_eq!(calls(&e, GET_PARENT_CELL).len(), 2);
        let slot_calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == 0x00f0_0002)
            .collect();
        assert_eq!(slot_calls.len(), 1);
        assert_eq!(slot_calls[0].1, vec![object, 0]);
    }

    #[test]
    fn talking_object_builds_a_process_of_the_players_kind() {
        let sizes = [
            (0u32, 0x46cu32, HIGH_PROCESS_CONSTRUCT),
            (1, 0x25c, PROCESS_CONSTRUCT_25C),
            (2, 200, PROCESS_CONSTRUCT_0C8),
            (3, 0xb4, PROCESS_CONSTRUCT_0B4),
        ];
        for (kind, size, constructor) in sizes {
            let mut e = talking_engine();
            set_player_process(&mut e, Some(kind));
            let source = source_reference(&mut e, 0, 0x7777, 0);
            start_log(&mut e);
            let object = e.call(0x0043_6780, &args![0u32, 0x55u32, source]).u32();
            assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x88], vec![size]]);
            let built = calls(&e, constructor);
            assert_eq!(built.len(), 1);
            let process = built[0][0];
            assert_eq!(calls(&e, SET_PROCESS), vec![vec![object, process]]);
            assert_eq!(
                calls(&e, PROCESS_LISTS_ADD_REFERENCE),
                vec![vec![PROCESS_LISTS, object, constructor, 0, 0, 0]],
                "kind {kind}"
            );
        }
    }

    #[test]
    fn talking_object_with_an_unknown_process_kind_gets_no_process() {
        let mut e = talking_engine();
        set_player_process(&mut e, Some(9));
        let source = source_reference(&mut e, 0, 0x7777, 0);
        e.register(GET_PROCESS_KIND, |e, a| {
            if a[0] == 0 {
                ret(0)
            } else {
                ret(e.mem.u32(a[0] + 0x28))
            }
        });
        start_log(&mut e);
        let object = e.call(0x0043_6780, &args![0u32, 0x55u32, source]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x88]]);
        assert_eq!(calls(&e, SET_PROCESS), vec![vec![object, 0]]);
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, object, 0, 0, 0, 0]]
        );
    }

    #[test]
    fn position_accessor_adds_0x30() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0043_6aa0, &args![0x1000u32]).u32(), 0x1030);
    }

    #[test]
    fn talking_actor_default_constructor_has_no_object() {
        check_null_payload_constructor(0x0043_6ac0, 0x55, EXTRA_TALKING_ACTOR_VTABLE);
    }

    #[test]
    fn talking_actor_scalar_deleting_destructor_frees_only_when_asked() {
        check_scalar_deleting(0x0043_6af0, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn talking_actor_destructor_resets_the_vtable() {
        let mut e = extra_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        e.call(0x0043_6bb0, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_TALKING_ACTOR_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // The talking object is not released.
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_0002);
    }

    #[test]
    fn talking_actor_constructor_stores_the_object_and_places_it() {
        let mut e = talking_engine();
        set_player_process(&mut e, None);
        let location = e.mem.alloc(12);
        let source = source_reference(&mut e, 0, 0x7777, location);
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(
            e.call(0x0043_6b20, &args![this, 0x55u32, source]).u32(),
            this
        );
        assert_eq!(extra_type(&e, this), 0x55);
        assert_eq!(vtable_of(&e, this), EXTRA_TALKING_ACTOR_VTABLE);
        let object = made_object(&e);
        assert_eq!(e.mem.u32(this + 0xc), object);
        let placed = calls(&e, SET_LOCATION_ON_REFERENCE);
        assert_eq!(
            placed,
            vec![vec![object, source + 0x30], vec![object, location]]
        );
    }

    /// A `Compare` of a one-float class.
    fn check_float_compare(addr: u32, target_type: u32) {
        let mut e = session_engine();
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| Ret::default());
        let this = dirty_object(&mut e, 0x10);
        let same = dirty_object(&mut e, 0x10);
        let other = dirty_object(&mut e, 0x10);
        let nan = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 2.5f32.to_bits());
        e.mem.set_u32(same + 0xc, 2.5f32.to_bits());
        e.mem.set_u32(other + 0xc, 3.5f32.to_bits());
        e.mem.set_u32(nan + 0xc, f32::NAN.to_bits());
        start_log(&mut e);
        assert!(e.call(addr, &args![this, 0u32]).bool());
        assert!(!e.call(addr, &args![this, same]).bool());
        assert!(e.call(addr, &args![this, other]).bool());
        assert!(e.call(addr, &args![this, nan]).bool());
        let casts = calls(&e, DYNAMIC_CAST);
        assert_eq!(casts[1], vec![same, 0, BS_EXTRA_DATA_TYPE, target_type, 0]);
        // A null cast never reaches the base compare.
        assert_eq!(calls(&e, BS_EXTRA_DATA_COMPARE).len(), 3);
        // The base compare's answer true settles it.
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| ret(1));
        assert!(e.call(addr, &args![this, same]).bool());
    }

    #[test]
    fn radius_compare_asks_the_float() {
        check_float_compare(0x0043_6bd0, EXTRA_RADIUS_TYPE);
    }

    #[test]
    fn radiation_compare_asks_the_float() {
        check_float_compare(0x0043_6c40, EXTRA_RADIATION_TYPE);
    }

    #[test]
    fn faction_changes_constructor_makes_an_empty_list() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_6cb0, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x5e);
        assert_eq!(vtable_of(&e, this), EXTRA_FACTION_CHANGES_VTABLE);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        let list = e.mem.u32(this + 0xc);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![list]]);
        assert_eq!(e.mem.block_size(list), Some(8));
    }

    #[test]
    fn faction_changes_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = session_engine();
        let this = e.mem.alloc(0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_6d50, &args![this, 0u32]).u32(), this);
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(e.call(0x0043_6d50, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn faction_changes_destructor_releases_the_list() {
        let mut e = session_engine();
        e.register(SIMPLE_LIST_DELETE, |_, _| Ret::default());
        let this = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 0x4400);
        start_log(&mut e);
        e.call(0x0043_6d80, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_FACTION_CHANGES_VTABLE);
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![0x4400]]);
        assert_eq!(calls(&e, SIMPLE_LIST_DELETE), vec![vec![0x4400, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // With no list the node release still runs, the deleting one not.
        let bare = dirty_object(&mut e, 0x10);
        e.mem.set_u32(bare + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_6d80, &args![bare]);
        assert_eq!(calls(&e, SIMPLE_LIST_CLEAR), vec![vec![0]]);
        assert!(calls(&e, SIMPLE_LIST_DELETE).is_empty());
    }

    /// A list of `(faction, rank)` entries: nodes `{item, next}` in a chain;
    /// returns the head node (a node with no item when `entries` is empty).
    fn faction_list(e: &mut Engine, entries: &[(u32, u8)]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (index, (faction, rank)) in entries.iter().enumerate() {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, *faction);
            e.mem.set_u8(entry + 4, *rank);
            e.mem.set_u32(node, entry);
            if index + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    /// The `(faction, rank)` entries of the list at `head`.
    fn entries_of(e: &Engine, head: u32) -> Vec<(u32, u8)> {
        let mut found = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            let entry = e.mem.u32(node);
            found.push((e.mem.u32(entry), e.mem.u8(entry + 4)));
            node = e.mem.u32(node + 4);
        }
        found
    }

    fn faction_changes_with(e: &mut Engine, list: u32) -> u32 {
        let this = dirty_object(e, 0x10);
        e.mem.set_u32(this + 0xc, list);
        this
    }

    #[test]
    fn faction_expel_marks_an_existing_entry() {
        let mut e = session_engine();
        let list = faction_list(&mut e, &[(0xa1, 2), (0xa2, 5)]);
        let this = faction_changes_with(&mut e, list);
        e.call(0x0043_6e10, &args![this, 0xa2u32]);
        assert_eq!(entries_of(&e, list), vec![(0xa1, 2), (0xa2, 0xff)]);
    }

    #[test]
    fn faction_expel_adds_a_missing_entry() {
        let mut e = session_engine();
        let list = faction_list(&mut e, &[(0xa1, 2)]);
        let this = faction_changes_with(&mut e, list);
        e.call(0x0043_6e10, &args![this, 0xa3u32]);
        assert_eq!(entries_of(&e, list), vec![(0xa1, 2), (0xa3, 0xff)]);
    }

    #[test]
    fn faction_expel_creates_the_list_when_there_is_none() {
        let mut e = session_engine();
        let this = faction_changes_with(&mut e, 0);
        start_log(&mut e);
        e.call(0x0043_6e10, &args![this, 0xa3u32]);
        let list = e.mem.u32(this + 0xc);
        assert!(list != 0);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![list]]);
        assert_eq!(entries_of(&e, list), vec![(0xa3, 0xff)]);
    }

    #[test]
    fn faction_set_rank_changes_only_existing_entries() {
        let mut e = session_engine();
        let list = faction_list(&mut e, &[(0xa1, 2), (0xa2, 5)]);
        let this = faction_changes_with(&mut e, list);
        e.call(0x0043_6f20, &args![this, 0xa1u32, 9u32]);
        e.call(0x0043_6f20, &args![this, 0xa9u32, 9u32]);
        assert_eq!(entries_of(&e, list), vec![(0xa1, 9), (0xa2, 5)]);
        let bare = faction_changes_with(&mut e, 0);
        e.call(0x0043_6f20, &args![bare, 0xa1u32, 9u32]);
        assert_eq!(e.mem.u32(bare + 0xc), 0);
    }

    #[test]
    fn faction_set_or_add_rank_adds_a_missing_entry() {
        let mut e = session_engine();
        let list = faction_list(&mut e, &[(0xa1, 2)]);
        let this = faction_changes_with(&mut e, list);
        e.call(0x0043_6f80, &args![this, 0xa1u32, 7u32]);
        assert_eq!(entries_of(&e, list), vec![(0xa1, 7)]);
        e.call(0x0043_6f80, &args![this, 0xa5u32, 4u32]);
        assert_eq!(entries_of(&e, list), vec![(0xa1, 7), (0xa5, 4)]);
    }

    #[test]
    fn faction_membership_needs_a_rank_above_minus_one() {
        let mut e = session_engine();
        let list = faction_list(
            &mut e,
            &[(0xa1, 0xff), (0xa1, 0), (0xa2, 0xff), (0xa3, 0x7f)],
        );
        let this = faction_changes_with(&mut e, list);
        // The expelled entry for 0xa1 does not stop the walk.
        assert!(e.call(0x0043_7010, &args![this, 0xa1u32]).bool());
        assert!(!e.call(0x0043_7010, &args![this, 0xa2u32]).bool());
        assert!(e.call(0x0043_7010, &args![this, 0xa3u32]).bool());
        assert!(!e.call(0x0043_7010, &args![this, 0xa4u32]).bool());
        let empty = faction_changes_with(&mut e, 0);
        assert!(!e.call(0x0043_7010, &args![empty, 0xa1u32]).bool());
    }

    #[test]
    fn faction_expelled_needs_rank_minus_one() {
        let mut e = session_engine();
        let list = faction_list(&mut e, &[(0xa1, 3), (0xa1, 0xff), (0xa2, 0), (0xa3, 0xfe)]);
        let this = faction_changes_with(&mut e, list);
        assert!(e.call(0x0043_7080, &args![this, 0xa1u32]).bool());
        assert!(!e.call(0x0043_7080, &args![this, 0xa2u32]).bool());
        assert!(!e.call(0x0043_7080, &args![this, 0xa3u32]).bool());
        let empty = faction_changes_with(&mut e, 0);
        assert!(!e.call(0x0043_7080, &args![empty, 0xa1u32]).bool());
    }

    #[test]
    fn faction_merge_expels_own_entries_and_adds_missing_ones() {
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;
        let mut e = session_engine();
        let recorded: Rc<RefCell<HashMap<u32, u8>>> = Rc::default();
        e.register(FACTION_MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FACTION_MAP_DESTRUCT, |_, _| Ret::default());
        let map = recorded.clone();
        e.register_double(FACTION_MAP_SET_AT, move |_, a| {
            map.borrow_mut().insert(a[1], a[2] as u8);
            Ret::default()
        });
        let map = recorded.clone();
        e.register_double(FACTION_MAP_LOOKUP, move |e, a| {
            match map.borrow().get(&a[1]) {
                Some(value) => {
                    e.mem.set_u8(a[2], *value);
                    ret(1)
                }
                None => ret(0),
            }
        });
        e.register(FACTION_LIST_OF, |_, a| ret(a[0] + 0x2c));
        let list = faction_list(&mut e, &[(0xa1, 2), (0xa2, 5)]);
        let this = faction_changes_with(&mut e, list);
        // The other record's list is embedded at +0x5c: head node there.
        let other = e.mem.alloc(0x80);
        let entry_a = e.mem.alloc(8);
        e.mem.set_u32(entry_a, 0xa2);
        let entry_b = e.mem.alloc(8);
        e.mem.set_u32(entry_b, 0xb7);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, entry_b);
        e.mem.set_u32(other + 0x5c, entry_a);
        e.mem.set_u32(other + 0x60, second);
        start_log(&mut e);
        e.call(0x0043_70f0, &args![this, other]);
        assert_eq!(calls(&e, FACTION_MAP_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, FACTION_MAP_CONSTRUCT)[0][1], 0x25);
        assert_eq!(calls(&e, FACTION_MAP_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, FACTION_LIST_OF), vec![vec![other + 0x30]]);
        // 0xa2 was recorded by the first pass; only 0xb7 is added.
        assert_eq!(
            entries_of(&e, list),
            vec![(0xa1, 0xff), (0xa2, 0xff), (0xb7, 0xff)]
        );
        assert_eq!(recorded.borrow().get(&0xa1), Some(&1));
        // Without another record only the first pass runs.
        let list = faction_list(&mut e, &[(0xc1, 4)]);
        let alone = faction_changes_with(&mut e, list);
        e.call(0x0043_70f0, &args![alone, 0u32]);
        assert_eq!(entries_of(&e, list), vec![(0xc1, 0xff)]);
    }

    #[test]
    fn faction_merge_adds_entries_the_map_has_with_a_zero_value() {
        let mut e = session_engine();
        e.register(FACTION_MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FACTION_MAP_DESTRUCT, |_, _| Ret::default());
        e.register(FACTION_MAP_SET_AT, |_, _| Ret::default());
        e.register(FACTION_MAP_LOOKUP, |e, a| {
            e.mem.set_u8(a[2], 0);
            ret(1)
        });
        e.register(FACTION_LIST_OF, |_, a| ret(a[0] + 0x2c));
        let list = faction_list(&mut e, &[]);
        let this = faction_changes_with(&mut e, list);
        let other = e.mem.alloc(0x80);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0xd4);
        e.mem.set_u32(other + 0x5c, entry);
        e.call(0x0043_70f0, &args![this, other]);
        assert_eq!(entries_of(&e, list), vec![(0xd4, 0xff)]);
    }

    #[test]
    fn radio_data_copy_copies_four_words() {
        let mut e = Engine::new();
        let from = e.mem.alloc(0x10);
        let to = e.mem.alloc(0x10);
        for (i, word) in [10.5f32.to_bits(), 2, 0.25f32.to_bits(), 0x9000]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(from + 4 * i as u32, *word);
            e.mem.set_u32(to + 4 * i as u32, 0xdead_beef);
        }
        e.call(0x0043_7240, &args![to, from]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(to + 4 * i), e.mem.u32(from + 4 * i));
        }
        // A null source leaves the destination alone.
        e.mem.set_u32(to, 1);
        e.call(0x0043_7240, &args![to, 0u32]);
        assert_eq!(e.mem.u32(to), 1);
    }

    #[test]
    fn radio_data_copy_quiets_signalling_nans_only_in_the_floats() {
        // 00437253/00437269: the radius and static percentage go through
        // FLD/FSTP float, which sets the quiet bit of a signalling NaN; the
        // range type and position reference (0043725d/00437275) are MOVs.
        let mut e = Engine::new();
        let from = e.mem.alloc(0x10);
        let to = e.mem.alloc(0x10);
        let words = [0x7f80_0001u32, 0x7f80_0001, 0xff80_0002, 0xff80_0002];
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(from + 4 * i as u32, *word);
        }
        e.call(0x0043_7240, &args![to, from]);
        assert_eq!(e.mem.u32(to), 0x7fc0_0001);
        assert_eq!(e.mem.u32(to + 4), 0x7f80_0001);
        assert_eq!(e.mem.u32(to + 8), 0xffc0_0002);
        assert_eq!(e.mem.u32(to + 0xc), 0xff80_0002);
    }

    #[test]
    fn radio_data_compare_compares_the_sixteen_bytes() {
        let mut e = session_engine();
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| Ret::default());
        e.register(MEMCMP, |e, a| {
            let differs = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(differs as u32)
        });
        let this = dirty_object(&mut e, 0x1c);
        let same = dirty_object(&mut e, 0x1c);
        let other = dirty_object(&mut e, 0x1c);
        e.mem.set_u8(other + 0xc + 15, 1);
        start_log(&mut e);
        assert!(e.call(0x0043_7290, &args![this, 0u32]).bool());
        assert!(!e.call(0x0043_7290, &args![this, same]).bool());
        assert!(e.call(0x0043_7290, &args![this, other]).bool());
        let casts = calls(&e, DYNAMIC_CAST);
        assert_eq!(
            casts[1],
            vec![same, 0, BS_EXTRA_DATA_TYPE, EXTRA_RADIO_DATA_TYPE, 0]
        );
        assert_eq!(calls(&e, MEMCMP)[0], vec![this + 0xc, same + 0xc, 0x10]);
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| ret(1));
        assert!(e.call(0x0043_7290, &args![this, same]).bool());
    }

    // ---- The last 40 functions of the range (00437300 to 00438280). ----

    use std::cell::RefCell;
    use std::rc::Rc;

    /// A list of `items`: nodes of 8 bytes (item, next); the head node comes
    /// first. An empty list is one node with a null item.
    fn item_list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    /// The doubles of the list accessors the node walkers use: the item slot
    /// of a node is the node itself, the next node is the word at +4, and the
    /// head removal moves the next node's item and link into the head.
    fn list_engine() -> Engine {
        let mut e = session_engine();
        e.register(SIMPLE_LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(SIMPLE_LIST_DELETE, |_, a| ret(a[0]));
        e
    }

    fn items_of(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = vec![];
        let mut node = head;
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn combat_style_compare_asks_the_base_then_the_style() {
        let mut e = session_engine();
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| Ret::default());
        let this = dirty_object(&mut e, 0x10);
        let same = dirty_object(&mut e, 0x10);
        let other = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_u32(same + 0xc, 0x1234);
        e.mem.set_u32(other + 0xc, 0x5678);
        start_log(&mut e);
        assert!(e.call(0x0043_7300, &args![this, 0u32]).bool());
        assert!(!e.call(0x0043_7300, &args![this, same]).bool());
        assert!(e.call(0x0043_7300, &args![this, other]).bool());
        let casts = calls(&e, DYNAMIC_CAST);
        assert_eq!(
            casts[1],
            vec![same, 0, BS_EXTRA_DATA_TYPE, EXTRA_COMBAT_STYLE_TYPE, 0]
        );
        // A base compare that reports a difference decides alone.
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| ret(1));
        assert!(e.call(0x0043_7300, &args![this, same]).bool());
    }

    #[test]
    fn primitive_compare_asks_only_the_primitive() {
        check_word_compare(0x0043_7370, EXTRA_PRIMITIVE_TYPE);
    }

    #[test]
    fn ammo_compare_checks_the_count_and_the_ammo() {
        let mut e = session_engine();
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| Ret::default());
        let make = |e: &mut Engine, ammo: u32, count: u32| {
            let object = dirty_object(e, 0x14);
            e.mem.set_u32(object + 0xc, ammo);
            e.mem.set_u32(object + 0x10, count);
            object
        };
        let this = make(&mut e, 0x10, 5);
        let same = make(&mut e, 0x10, 5);
        let other_ammo = make(&mut e, 0x11, 5);
        let other_count = make(&mut e, 0x10, 6);
        assert!(e.call(0x0043_73c0, &args![this, 0u32]).bool());
        assert!(!e.call(0x0043_73c0, &args![this, same]).bool());
        assert!(e.call(0x0043_73c0, &args![this, other_ammo]).bool());
        assert!(e.call(0x0043_73c0, &args![this, other_count]).bool());
        e.register(BS_EXTRA_DATA_COMPARE, |_, _| ret(1));
        assert!(e.call(0x0043_73c0, &args![this, same]).bool());
    }

    #[test]
    fn say_topic_info_once_a_day_constructor_builds_a_list() {
        let mut e = session_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7440, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x73);
        assert_eq!(vtable_of(&e, this), EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_VTABLE);
        let list = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.block_size(list), Some(8));
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![list]]);
    }

    #[test]
    fn say_topic_info_once_a_day_scalar_deleting_destructor_frees_on_flag() {
        let mut e = session_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_74e0, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert!(e.mem.block_size(this).is_some());
        e.call(0x0043_74e0, &args![this, 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn say_topic_info_once_a_day_constructor_with_an_item_adds_it() {
        let mut e = session_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7510, &args![this, 0xabcdu32]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x73);
        let list = e.mem.u32(this + 0xc);
        assert_eq!(items_of(&e, list), vec![0xabcd]);
        assert_eq!(calls(&e, SIMPLE_LIST_ADD_HEAD).len(), 1);
        assert_eq!(calls(&e, SIMPLE_LIST_ADD_HEAD)[0][0], list);
    }

    #[test]
    fn say_topic_info_once_a_day_destructor_frees_every_item() {
        let mut e = list_engine();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let list = item_list(&mut e, &[first, second]);
        let this = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, list);
        start_log(&mut e);
        e.call(0x0043_75c0, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_SAY_TOPIC_INFO_ONCE_A_DAY_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![first], vec![second]]);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).len(), 2);
        assert_eq!(calls(&e, SIMPLE_LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        // Without a list only the base destructor runs.
        let bare = dirty_object(&mut e, 0x10);
        e.mem.set_u32(bare + 0xc, 0);
        start_log(&mut e);
        e.call(0x0043_75c0, &args![bare]);
        assert!(calls(&e, SIMPLE_LIST_DELETE).is_empty());
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![bare]]);
    }

    #[test]
    fn say_to_topic_info_default_constructor_clears_the_fields() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x1c);
        for offset in [0x10, 0x14, 0x18] {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x0043_7690, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x75);
        assert_eq!(vtable_of(&e, this), EXTRA_SAY_TO_TOPIC_INFO_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x18), 0);
    }

    #[test]
    fn say_to_topic_info_constructor_stores_the_info() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x1c);
        for offset in [0x10, 0x14, 0x18] {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x0043_76e0, &args![this, 0x4242u32]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x75);
        assert_eq!(e.mem.u32(this + 0xc), 0x4242);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x18), 0);
    }

    #[test]
    fn say_to_topic_info_set_topic_stores_the_topic() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x1c);
        e.call(0x0043_7730, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x77);
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_0002);
    }

    #[test]
    fn water_zone_map_constructor_builds_the_map() {
        let mut e = session_engine();
        e.register(WATER_ZONE_MAP_CONSTRUCT, |_, a| ret(a[0]));
        let this = dirty_object(&mut e, 0x20);
        e.mem.set_u32(this + 0x1c, 0xdead_beef);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7750, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x7e);
        assert_eq!(vtable_of(&e, this), EXTRA_WATER_ZONE_MAP_VTABLE);
        assert_eq!(
            calls(&e, WATER_ZONE_MAP_CONSTRUCT),
            vec![vec![this + 0xc, 0x25]]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn water_zone_map_scalar_deleting_destructor_frees_on_flag() {
        let mut e = session_engine();
        e.register(WATER_ZONE_MAP_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_77c0, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, WATER_ZONE_MAP_DESTRUCT), vec![vec![this + 0xc]]);
        assert!(e.mem.block_size(this).is_some());
        e.call(0x0043_77c0, &args![this, 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn water_zone_map_destructor_keeps_the_vtable() {
        let mut e = session_engine();
        e.register(WATER_ZONE_MAP_DESTRUCT, |_, _| Ret::default());
        let this = dirty_object(&mut e, 0x20);
        start_log(&mut e);
        e.call(0x0043_77f0, &args![this]);
        assert_eq!(vtable_of(&e, this), 0xdead_0001);
        assert_eq!(calls(&e, WATER_ZONE_MAP_DESTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    /// A map of zone pointers to counts, in insertion order, behind the
    /// doubles of the map accessors.
    fn zone_map(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let entries: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        e.map(0x0101_5000, 0x1000);
        e.set_global(LOWEST_FLOAT, 0xff7f_ffffu32);
        let map = entries.clone();
        e.register_double(ZONE_MAP_LOOKUP, move |e, a| {
            if let Some((_, count)) = map.borrow().iter().find(|(key, _)| *key == a[1]) {
                e.mem.set_u32(a[2], *count);
            }
            Ret::default()
        });
        let map = entries.clone();
        e.register_double(ZONE_MAP_SET_AT, move |_, a| {
            let mut map = map.borrow_mut();
            match map.iter_mut().find(|(key, _)| *key == a[1]) {
                Some(entry) => entry.1 = a[2],
                None => map.push((a[1], a[2])),
            }
            Ret::default()
        });
        let map = entries.clone();
        e.register_double(ZONE_MAP_REMOVE_AT, move |_, a| {
            map.borrow_mut().retain(|(key, _)| *key != a[1]);
            Ret::default()
        });
        let map = entries.clone();
        e.register_double(ZONE_MAP_FIRST, move |_, _| {
            ret(!map.borrow().is_empty() as u32)
        });
        let map = entries.clone();
        e.register_double(ZONE_MAP_NEXT, move |e, a| {
            let map = map.borrow();
            let index = e.mem.u32(a[1]) as usize - 1;
            e.mem.set_u32(a[2], map[index].0);
            e.mem.set_u32(a[3], map[index].1);
            let next = if index + 1 < map.len() { index + 2 } else { 0 };
            e.mem.set_u32(a[1], next as u32);
            Ret::default()
        });
        entries
    }

    fn zone_with_height(e: &mut Engine, height: f32) -> u32 {
        let zone = e.mem.alloc(0x20);
        e.mem.set_u32(zone + 0x18, height.to_bits());
        zone
    }

    #[test]
    fn water_zone_map_add_and_remove_keep_the_highest_zone() {
        let mut e = session_engine();
        let entries = zone_map(&mut e);
        let this = dirty_object(&mut e, 0x20);
        e.mem.set_u32(this + 0x1c, 0);
        let low = zone_with_height(&mut e, 2.0);
        let high_a = zone_with_height(&mut e, 7.0);
        let high_b = zone_with_height(&mut e, 7.0);
        start_log(&mut e);
        // Adding sets the count to 1 and makes the zone the highest.
        e.call(0x0043_7850, &args![this, low, 1u32]);
        assert_eq!(*entries.borrow(), vec![(low, 1)]);
        assert_eq!(e.mem.u32(this + 0x1c), low);
        assert_eq!(calls(&e, ZONE_MAP_SET_AT)[0], vec![this + 0xc, low, 1]);
        e.call(0x0043_7850, &args![this, high_a, 1u32]);
        e.call(0x0043_7850, &args![this, high_b, 1u32]);
        assert_eq!(e.mem.u32(this + 0x1c), high_b);
        // A zone as low as the lowest float does not qualify.
        let floor = zone_with_height(&mut e, f32::from_bits(0xff7f_ffff));
        e.call(0x0043_7850, &args![this, floor, 1u32]);
        assert_eq!(e.mem.u32(this + 0x1c), high_b);
        // Removing a zone with a count above 1 lowers the count.
        entries.borrow_mut()[0].1 = 3;
        e.call(0x0043_7850, &args![this, low, 0u32]);
        assert_eq!(entries.borrow()[0], (low, 2));
        assert_eq!(calls(&e, ZONE_MAP_REMOVE_AT).len(), 0);
        // At count 1 the zone goes and the highest is searched again: the
        // earlier of two equal heights wins.
        e.call(0x0043_7850, &args![this, high_b, 0u32]);
        assert!(entries.borrow().iter().all(|(key, _)| *key != high_b));
        assert_eq!(
            calls(&e, ZONE_MAP_REMOVE_AT),
            vec![vec![this + 0xc, high_b]]
        );
        assert_eq!(e.mem.u32(this + 0x1c), high_a);
    }

    #[test]
    fn water_zone_map_remove_of_the_last_zone_leaves_the_highest_alone() {
        let mut e = session_engine();
        let entries = zone_map(&mut e);
        let this = dirty_object(&mut e, 0x20);
        let only = zone_with_height(&mut e, 4.0);
        e.call(0x0043_7850, &args![this, only, 1u32]);
        e.call(0x0043_7850, &args![this, only, 0u32]);
        assert!(entries.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x1c), only);
    }

    #[test]
    fn ignored_by_sandbox_constructor_has_no_payload() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0xc);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7970, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x80);
        assert_eq!(vtable_of(&e, this), EXTRA_IGNORED_BY_SANDBOX_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_CONSTRUCT), vec![vec![this, 0x80]]);
    }

    #[test]
    fn patrol_ref_in_use_data_constructor_stores_the_handle() {
        let mut e = session_engine();
        e.register(REFERENCE_HANDLE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let user = e.mem.alloc(0x20);
        e.mem.set_u32(user + 0x10, 0x5150);
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_79a0, &args![this, user]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x88);
        assert_eq!(vtable_of(&e, this), EXTRA_PATROL_REF_IN_USE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x5150);
        assert_eq!(calls(&e, REFERENCE_HANDLE), vec![vec![user]]);
        // Without a user the handle is 0 and nothing is asked.
        let alone = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        e.call(0x0043_79a0, &args![alone, 0u32]);
        assert_eq!(e.mem.u32(alone + 0xc), 0);
        assert!(calls(&e, REFERENCE_HANDLE).is_empty());
    }

    #[test]
    fn patrol_ref_in_use_data_compare_looks_only_at_the_user() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x10);
        let same = dirty_object(&mut e, 0x10);
        let other = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 9);
        e.mem.set_u32(same + 0xc, 9);
        e.mem.set_u32(other + 0xc, 10);
        start_log(&mut e);
        assert!(!e.call(0x0043_7a30, &args![this, same]).bool());
        assert!(e.call(0x0043_7a30, &args![this, other]).bool());
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        assert!(calls(&e, BS_EXTRA_DATA_COMPARE).is_empty());
    }

    #[test]
    fn patrol_ref_in_use_data_scalar_deleting_destructor_frees_on_flag() {
        check_scalar_deleting(0x0043_7a60, BS_EXTRA_DATA_DESTRUCT);
    }

    #[test]
    fn patrol_ref_in_use_data_destructor_resets_the_vtable() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        e.call(0x0043_7a90, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_PATROL_REF_IN_USE_DATA_VTABLE);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    /// The pieces of a reference for `GetCanBeUsedBy`: virtual slots 0x22c
    /// (index 0x8b) and 0x100 (index 0x40), the flags word at +8, the kind
    /// code and the process type kept in words of the object.
    struct PatrolReference {
        object: u32,
        disabled: bool,
        in_use: bool,
        flags: u32,
        kind: u32,
        process_type: u32,
        byte_87: u8,
    }

    fn patrol_engine(reference: PatrolReference) -> (Engine, u32) {
        let mut e = session_engine();
        e.register(REFERENCE_HANDLE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let table = 0x7100_0000;
        let mut slots = vec![0u32; 0x8c];
        slots[0x8b] = 0x7000_0001;
        slots[0x40] = 0x7000_0002;
        e.put_vtable(table, &slots);
        let disabled = reference.disabled as u32;
        let in_use = reference.in_use as u32;
        e.register_double(0x7000_0001, move |_, _| ret(disabled));
        e.register_double(0x7000_0002, move |_, _| ret(in_use));
        let object = reference.object;
        e.map(object, 0x1000);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 8, reference.flags);
        e.mem.set_u8(object + 0x87, reference.byte_87);
        e.register_double(REFERENCE_FOR_HANDLE, move |_, a| {
            ret(if a[0] == 0x99 { object } else { 0 })
        });
        let kind = reference.kind;
        e.register_double(REFERENCE_KIND, move |_, _| ret(kind));
        let process_type = reference.process_type;
        e.register_double(GET_CURRENT_PROCESS_TYPE, move |_, _| ret(process_type));
        let this = dirty_object(&mut e, 0x10);
        e.mem.set_u32(this + 0xc, 0x99);
        (e, this)
    }

    fn live_reference(e_object: u32) -> PatrolReference {
        PatrolReference {
            object: e_object,
            disabled: false,
            in_use: true,
            flags: 0,
            kind: 1,
            process_type: 0,
            byte_87: 0,
        }
    }

    /// Runs `GetCanBeUsedBy` with no user for a reference described by
    /// `describe`; returns the answer and the `User` word afterwards.
    fn patrol_answer(describe: impl FnOnce(&mut PatrolReference)) -> (bool, u32) {
        let mut reference = live_reference(0x7200_0000);
        describe(&mut reference);
        let (mut e, this) = patrol_engine(reference);
        let answer = e.call(0x0043_7ab0, &args![this, 0u32]).bool();
        (answer, e.mem.u32(this + 0xc))
    }

    #[test]
    fn patrol_ref_in_use_data_is_usable_by_its_user() {
        let mut reference = live_reference(0x7200_0000);
        reference.in_use = false;
        let (mut e, this) = patrol_engine(reference);
        let user = e.mem.alloc(0x20);
        e.mem.set_u32(user + 0x10, 0x99);
        start_log(&mut e);
        assert!(e.call(0x0043_7ab0, &args![this, user]).bool());
        assert_eq!(e.mem.u32(this + 0xc), 0x99);
        assert!(calls(&e, REFERENCE_FOR_HANDLE).is_empty());
    }

    #[test]
    fn patrol_ref_in_use_data_releases_a_vanished_reference() {
        let (mut e, this) = patrol_engine(live_reference(0x7200_0000));
        e.mem.set_u32(this + 0xc, 0x55);
        assert!(e.call(0x0043_7ab0, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn patrol_ref_in_use_data_releases_a_disabled_reference() {
        let (answer, user) = patrol_answer(|r| r.disabled = true);
        assert!(answer);
        assert_eq!(user, 0);
    }

    #[test]
    fn patrol_ref_in_use_data_releases_a_flagged_reference() {
        let (answer, user) = patrol_answer(|r| r.flags = 0x800);
        assert!(answer);
        assert_eq!(user, 0);
        let (answer, user) = patrol_answer(|r| r.flags = 0x1);
        assert!(!answer);
        assert_eq!(user, 0x99);
    }

    #[test]
    fn patrol_ref_in_use_data_keeps_a_reference_that_is_not_in_use() {
        let (answer, user) = patrol_answer(|r| {
            r.in_use = false;
            r.kind = 5;
        });
        assert!(!answer);
        assert_eq!(user, 0x99);
    }

    #[test]
    fn patrol_ref_in_use_data_releases_by_kind_or_process() {
        for kind in [3, 5] {
            let (answer, user) = patrol_answer(|r| r.kind = kind);
            assert!(answer);
            assert_eq!(user, 0);
        }
        // A process type with the byte at +0x87 clear releases it.
        let (answer, user) = patrol_answer(|r| r.process_type = 2);
        assert!(answer);
        assert_eq!(user, 0);
        // With the byte set, or without a process type, it is kept.
        let (answer, user) = patrol_answer(|r| {
            r.process_type = 2;
            r.byte_87 = 1;
        });
        assert!(!answer);
        assert_eq!(user, 0x99);
        let (answer, user) = patrol_answer(|_| {});
        assert!(!answer);
        assert_eq!(user, 0x99);
    }

    #[test]
    fn form_flags_predicate_tests_two_bits() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x10);
        for (flags, expected) in [(0u32, false), (0x20, true), (0x800, true), (0x7df, false)] {
            e.mem.set_u32(form + 8, flags);
            assert_eq!(e.call(0x0043_7b90, &args![form]).bool(), expected);
        }
    }

    #[test]
    fn reference_byte_87_accessor_reads_the_byte() {
        let mut e = Engine::new();
        let reference = e.mem.alloc(0x90);
        assert!(!e.call(0x0043_7bb0, &args![reference]).bool());
        e.mem.set_u8(reference + 0x87, 1);
        assert!(e.call(0x0043_7bb0, &args![reference]).bool());
    }

    #[test]
    fn reference_kind_predicates_compare_the_code() {
        let mut e = Engine::new();
        let kind = Rc::new(RefCell::new(0u32));
        let shared = kind.clone();
        e.register_double(REFERENCE_KIND, move |_, _| ret(*shared.borrow()));
        for (code, is_three, is_five) in [(3, true, false), (5, false, true), (4, false, false)] {
            *kind.borrow_mut() = code;
            assert_eq!(e.call(0x0043_7bd0, &args![0x1000u32]).bool(), is_three);
            assert_eq!(e.call(0x0043_7bf0, &args![0x1000u32]).bool(), is_five);
        }
    }

    #[test]
    fn breadcrumb_constructor_runs_the_vector_constructors() {
        let mut e = session_engine();
        let crumb = e.mem.alloc(0x24);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7c10, &args![crumb]).u32(), crumb);
        assert_eq!(
            calls(&e, NI_POINT3_CONSTRUCT),
            vec![vec![crumb], vec![crumb + 0x10]]
        );
    }

    #[test]
    fn follower_swim_breadcrumbs_constructor_copies_the_default_location() {
        let mut e = session_engine();
        e.map(0x011f_4000, 0x1000);
        for (i, word) in [1.5f32, -2.0, 3.25].iter().enumerate() {
            e.mem
                .set_u32(DEFAULT_LEADER_LOCATION + 4 * i as u32, word.to_bits());
        }
        let this = dirty_object(&mut e, 0x28);
        for offset in [0x10, 0x14, 0x18, 0x1c] {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7c40, &args![this]).u32(), this);
        assert_eq!(extra_type(&e, this), 0x8b);
        assert_eq!(vtable_of(&e, this), EXTRA_FOLLOWER_SWIM_BREADCRUMBS_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x14), (-2.0f32).to_bits());
        assert_eq!(e.mem.u32(this + 0x18), 3.25f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![this + 0x20]]);
    }

    #[test]
    fn follower_swim_breadcrumbs_scalar_deleting_destructor_frees_on_flag() {
        let mut e = session_engine();
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x20, 0);
        e.mem.set_u32(this + 0x24, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_7ce0, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert!(e.mem.block_size(this).is_some());
        e.call(0x0043_7ce0, &args![this, 1u32]);
        assert_eq!(e.mem.block_size(this), None);
    }

    #[test]
    fn follower_swim_breadcrumbs_destructor_destroys_every_crumb() {
        let mut e = session_engine();
        e.register(CRUMB_DELETE, |_, _| Ret::default());
        let this = dirty_object(&mut e, 0x28);
        let crumb = e.mem.alloc(0x24);
        let second = e.mem.alloc(8);
        // The list is embedded at +0x20: the head node holds a crumb, the
        // next node none.
        e.mem.set_u32(this + 0x20, crumb);
        e.mem.set_u32(this + 0x24, second);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, 0);
        start_log(&mut e);
        e.call(0x0043_7d10, &args![this]);
        assert_eq!(vtable_of(&e, this), EXTRA_FOLLOWER_SWIM_BREADCRUMBS_VTABLE);
        assert_eq!(calls(&e, CRUMB_DELETE), vec![vec![crumb, 1]]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTRUCT), vec![vec![this + 0x20]]);
        assert_eq!(calls(&e, BS_EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn follower_swim_breadcrumbs_first_crumb_is_the_head_item() {
        let mut e = session_engine();
        let this = dirty_object(&mut e, 0x28);
        e.mem.set_u32(this + 0x20, 0x1234);
        assert_eq!(e.call(0x0043_7dd0, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn follower_swim_breadcrumbs_drop_first_destroys_the_crumb() {
        let mut e = list_engine();
        e.register(CRUMB_DELETE, |_, _| Ret::default());
        e.register(SIMPLE_LIST_SET_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        let this = dirty_object(&mut e, 0x28);
        let crumb = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, crumb);
        e.mem.set_u32(this + 0x24, 0);
        start_log(&mut e);
        e.call(0x0043_7df0, &args![this]);
        assert_eq!(calls(&e, CRUMB_DELETE), vec![vec![crumb, 1]]);
        assert_eq!(calls(&e, SIMPLE_LIST_SET_HEAD).len(), 1);
        assert_eq!(calls(&e, SIMPLE_LIST_SET_HEAD)[0][0], this + 0x20);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD), vec![vec![this + 0x20]]);
        // With no crumb nothing is destroyed.
        e.mem.set_u32(this + 0x20, 0);
        start_log(&mut e);
        e.call(0x0043_7df0, &args![this]);
        assert!(calls(&e, CRUMB_DELETE).is_empty());
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE_HEAD).len(), 1);
    }

    #[test]
    fn follower_swim_breadcrumbs_record_appends_a_crumb_on_a_state_change() {
        let mut e = session_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let appended: Rc<RefCell<Vec<u32>>> = Rc::default();
        let log = appended.clone();
        e.register_double(CRUMB_LIST_ADD, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let this = dirty_object(&mut e, 0x28);
        let set = |e: &mut Engine, state: u32, nav_mesh: u32| {
            e.mem.set_u32(this + 0xc, state);
            e.mem.set_u32(this + 0x10, 1.0f32.to_bits());
            e.mem.set_u32(this + 0x14, 2.0f32.to_bits());
            e.mem.set_u32(this + 0x18, 3.0f32.to_bits());
            e.mem.set_u32(this + 0x1c, nav_mesh);
        };
        let (x, y, z) = (4.0f32.to_bits(), 5.0f32.to_bits(), 6.0f32.to_bits());
        // A state of 0 does nothing.
        set(&mut e, 1, 0x70);
        assert!(!e
            .call(0x0043_7e50, &args![this, 0u32, x, y, z, 0x71u32])
            .bool());
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x10), 1.0f32.to_bits());
        // From state 0 the location is stored with no crumb.
        set(&mut e, 0, 0x70);
        assert!(!e
            .call(0x0043_7e50, &args![this, 1u32, x, y, z, 0x71u32])
            .bool());
        assert!(appended.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x10), x);
        assert_eq!(e.mem.u32(this + 0x1c), 0x71);
        // The same state again stores without a crumb.
        assert!(!e
            .call(0x0043_7e50, &args![this, 1u32, x, y, z, 0x72u32])
            .bool());
        assert!(appended.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x1c), 0x72);
        // A change from state 1 to 2 appends the old and new positions.
        set(&mut e, 1, 0x70);
        assert!(e
            .call(0x0043_7e50, &args![this, 2u32, x, y, z, 0x71u32])
            .bool());
        let crumb = appended.borrow()[0];
        assert_eq!(e.mem.block_size(crumb), Some(0x28));
        assert_eq!(e.mem.u32(crumb), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(crumb + 4), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(crumb + 8), 3.0f32.to_bits());
        assert_eq!(e.mem.u32(crumb + 0xc), 0x70);
        assert_eq!(e.mem.u32(crumb + 0x10), x);
        assert_eq!(e.mem.u32(crumb + 0x14), y);
        assert_eq!(e.mem.u32(crumb + 0x18), z);
        assert_eq!(e.mem.u32(crumb + 0x1c), 0x71);
        assert_eq!(e.mem.u8(crumb + 0x20), 1);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0x18), z);
        // A change to a state other than 2 clears the flag.
        set(&mut e, 2, 0x70);
        assert!(e
            .call(0x0043_7e50, &args![this, 3u32, x, y, z, 0x71u32])
            .bool());
        assert_eq!(e.mem.u8(appended.borrow()[1] + 0x20), 0);
    }

    #[test]
    fn fixed_string_constructor_uses_the_empty_value_for_null() {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5555);
        e.register(FIXED_STRING_CREATE, |_, a| ret(a[0] + 1));
        let slot = e.mem.alloc(4);
        start_log(&mut e);
        assert_eq!(e.call(0x0043_8170, &args![slot, 0u32]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x5555);
        assert!(calls(&e, FIXED_STRING_CREATE).is_empty());
        e.call(0x0043_8170, &args![slot, 0x9000u32]);
        assert_eq!(e.mem.u32(slot), 0x9001);
        assert_eq!(calls(&e, FIXED_STRING_CREATE), vec![vec![0x9000]]);
    }

    #[test]
    fn fixed_string_release_skips_the_empty_value() {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5555);
        e.register(FIXED_STRING_RELEASE, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x5555);
        start_log(&mut e);
        e.call(0x0043_81d0, &args![slot]);
        assert!(calls(&e, FIXED_STRING_RELEASE).is_empty());
        e.mem.set_u32(slot, 0x6000);
        e.call(0x0043_81d0, &args![slot]);
        assert_eq!(calls(&e, FIXED_STRING_RELEASE), vec![vec![0x5ff8]]);
    }

    #[test]
    fn fixed_string_destructor_releases_the_text() {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5555);
        e.register(FIXED_STRING_RELEASE, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x7000);
        start_log(&mut e);
        e.call(0x0043_81b0, &args![slot]);
        assert_eq!(calls(&e, FIXED_STRING_RELEASE), vec![vec![0x6ff8]]);
    }

    #[test]
    fn fixed_string_header_is_eight_bytes_before_the_text() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x1234);
        assert_eq!(e.call(0x0043_8210, &args![slot]).u32(), 0x122c);
    }

    #[test]
    fn securitron_property_type_is_three() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0043_8220, &args![]).u32(), 3);
    }

    #[test]
    fn securitron_property_takes_the_texture() {
        let mut e = Engine::new();
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let property = e.mem.alloc(0x70);
        e.mem.set_u32(property + 0x38, 0xdead_beef);
        start_log(&mut e);
        e.call(0x0043_8230, &args![property, 0x4321u32]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![property + 0x60, 0x4321]]
        );
        assert_eq!(e.mem.u32(property + 0x38), 0);
    }

    #[test]
    fn securitron_face_setters_assign_their_strings() {
        let mut e = Engine::new();
        e.register(STRING_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        start_log(&mut e);
        e.call(0x0043_8260, &args![this, 0xa0u32]);
        e.call(0x0043_8280, &args![this, 0xb0u32]);
        assert_eq!(
            calls(&e, STRING_ASSIGN),
            vec![vec![this + 0xc, 0xa0], vec![this + 0x14, 0xb0]]
        );
    }

    /// `ApplyFace` with doubles: the two strings and their text addresses,
    /// the root's slot 0x9c (returns `screen`), the property the screen has,
    /// and the texture the loader puts in the smart pointer.
    struct FaceScene {
        e: Engine,
        this: u32,
        root: u32,
        screen: u32,
        property: u32,
        assigned: Rc<RefCell<Vec<u32>>>,
    }

    fn face_scene(first: &'static str, second: &'static str, has_screen: bool) -> FaceScene {
        let mut e = session_engine();
        e.map(0x0109_b000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5555);
        e.mem.set_u32(TES, 0x6000_0000);
        e.register(FIXED_STRING_CREATE, |_, a| ret(a[0] + 8));
        e.register(FIXED_STRING_RELEASE, |_, _| Ret::default());
        // A string is a text address and a length; the first getter fills
        // the first string, the second getter the second.
        let texts = [first, second];
        for (index, getter) in [GET_FACE_STRING_FIRST, GET_FACE_STRING_SECOND]
            .into_iter()
            .enumerate()
        {
            let text = texts[index];
            e.register_double(getter, move |e, a| {
                let buffer = e.mem.alloc(text.len() as u32 + 1);
                for (i, byte) in text.bytes().enumerate() {
                    e.mem.set_u8(buffer + i as u32, byte);
                }
                e.mem.set_u32(a[1], buffer);
                e.mem.set_u32(a[1] + 4, text.len() as u32);
                Ret::default()
            });
        }
        e.register(STRING_LENGTH, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(POINTER_VALUE, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(FORMAT_STRING, |e, a| {
            // Builds "<first>_<second>" so the arguments can be checked.
            let read = |e: &Engine, mut at: u32| {
                let mut text = String::new();
                while e.mem.u8(at) != 0 {
                    text.push(e.mem.u8(at) as char);
                    at += 1;
                }
                text
            };
            let text = format!("{}_{}", read(e, a[3]), read(e, a[4]));
            for (i, byte) in text.bytes().enumerate() {
                e.mem.set_u8(a[0] + i as u32, byte);
            }
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            Ret::default()
        });
        e.register(SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH, |e, a| {
            let mut at = 0;
            loop {
                let byte = e.mem.u8(a[0] + at);
                e.mem.set_u8(a[1] + at, byte);
                if byte == 0 {
                    break;
                }
                at += 1;
            }
            Ret::default()
        });
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(CREATE_TEXTURE_IMAGE, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            Ret::default()
        });
        let root = e.mem.alloc(0x10);
        let screen = if has_screen { e.mem.alloc(0x10) } else { 0 };
        let table = 0x7300_0000;
        let mut slots = vec![0u32; 0x28];
        slots[0x27] = 0x7000_0010;
        e.put_vtable(table, &slots);
        e.register_double(0x7000_0010, move |_, _| ret(screen));
        e.mem.set_u32(root, table);
        let property = e.mem.alloc(0x70);
        e.register_double(GET_PROPERTY, move |_, a| {
            ret(if a[1] == 3 { property } else { 0 })
        });
        let assigned: Rc<RefCell<Vec<u32>>> = Rc::default();
        let log = assigned.clone();
        e.register_double(NI_POINTER_ASSIGN, move |_, a| {
            log.borrow_mut().push(a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x1c);
        FaceScene {
            e,
            this,
            root,
            screen,
            property,
            assigned,
        }
    }

    #[test]
    fn apply_face_builds_the_texture_path_and_assigns_the_texture() {
        let mut scene = face_scene("calm", "happy", true);
        start_log(&mut scene.e);
        scene.e.call(0x0043_7f90, &args![scene.this, scene.root]);
        let e = &scene.e;
        // The name is made from the literal and released afterwards.
        assert_eq!(
            calls(e, FIXED_STRING_CREATE),
            vec![vec![SECURITRON_SCREEN_NAME]]
        );
        assert_eq!(calls(e, FIXED_STRING_RELEASE).len(), 1);
        let format = calls(e, FORMAT_STRING);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1], 0x104);
        assert_eq!(format[0][2], SECURITRON_TEXTURE_FORMAT);
        let localized = calls(e, SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH)[0][1];
        let mut path = String::new();
        let mut at = localized;
        while e.mem.u8(at) != 0 {
            path.push(e.mem.u8(at) as char);
            at += 1;
        }
        assert_eq!(path, "calm_happy");
        let create = &calls(e, CREATE_TEXTURE_IMAGE)[0];
        assert_eq!(create[0], 0x6000_0000);
        assert_eq!(create[1], localized);
        assert_eq!(&create[3..], &[0, 0]);
        assert_eq!(calls(e, GET_PROPERTY), vec![vec![scene.screen, 3]]);
        assert_eq!(*scene.assigned.borrow(), vec![0x7777]);
        assert_eq!(calls(e, STRING_DESTRUCT).len(), 2);
        assert_eq!(calls(e, NI_POINTER_DESTRUCT).len(), 1);
        let _ = scene.property;
    }

    #[test]
    fn apply_face_with_an_empty_string_still_gives_the_property_no_texture() {
        let mut scene = face_scene("calm", "", true);
        start_log(&mut scene.e);
        scene.e.call(0x0043_7f90, &args![scene.this, scene.root]);
        let e = &scene.e;
        assert!(calls(e, FORMAT_STRING).is_empty());
        assert!(calls(e, CREATE_TEXTURE_IMAGE).is_empty());
        assert_eq!(*scene.assigned.borrow(), vec![0]);
        assert_eq!(calls(e, STRING_DESTRUCT).len(), 2);
    }

    #[test]
    fn apply_face_stops_without_a_screen_or_a_root() {
        let mut scene = face_scene("calm", "happy", false);
        start_log(&mut scene.e);
        scene.e.call(0x0043_7f90, &args![scene.this, scene.root]);
        let e = &scene.e;
        assert_eq!(calls(e, FIXED_STRING_RELEASE).len(), 1);
        assert!(calls(e, GET_FACE_STRING_FIRST).is_empty());
        assert!(calls(e, NI_POINTER_CONSTRUCT).is_empty());
        assert!(scene.assigned.borrow().is_empty());
        // A null root does nothing at all.
        start_log(&mut scene.e);
        scene.e.call(0x0043_7f90, &args![scene.this, 0u32]);
        assert!(calls(&scene.e, FIXED_STRING_CREATE).is_empty());
    }
}
