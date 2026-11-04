//! `fallout shared/extradatalist.cpp` (Xbox PDB source unit), part 3: its functions from `0041db00` up to
//! (not including) `00421400` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradatalist`]; anything public there may be used here.
//!
//! Translated so far, in address order: the first 40 functions of the range,
//! `0041db00` to `0041eb60`: the accessors of the extra data of the enable
//! state parent (type `0x37`) and children (`0x38`), the item dropper
//! (`0x39`), the dropped item list (`0x3A`), the water type (`3`), the
//! type `0x3B` marker, the ash pile reference (`0x89`), the linked
//! reference (`0x51`) and its children (`0x52`), the open/close activate
//! reference (`0x6C`) and the activate reference (`0x53`, up to its flag
//! getter). The second session did the next 40, `0041eba0` to `00420210`:
//! the rest of the activate reference (flag setter, activate text,
//! children `0x54` with their timer), the decal references (`0x57`), the
//! reflected and reflector references (`0x65`, `0x66`), the water light and
//! lit water references (`0x84`, `0x85`), the primitive (`0x6B`), the
//! patrol reference data (`0x6F`), the occlusion plane reference data
//! (`0x76`) and the setter of the portal reference data (`0x77`). It also
//! translated three functions of the range that neither the engine map nor
//! Ghidra's function list know, found by following calls: `0041fcd0`
//! (constructor of the patrol reference data), `0041fdb0` and `00420060`
//! (the destructors of the patrol and the occlusion plane reference data).
//! The third session did the last 40 and the two unlisted destructors found
//! by following calls (`004203b0`, `00420630`), `004202d0` to `004213c0`: the
//! portal reference data (`0x77`, with the portal words `00420a60`,
//! `00420bc0`), the room reference data (`0x7B`, its linked data, the master
//! room search), the portal and the room (`0x78`, `0x79`), the collision data
//! (`0x72`), the actor package data (`0x70`) and the guarded reference data
//! (`0x7C`). The range is complete.
//!
//! The type numbers are `EXTRA_DATA_TYPE` of the Xbox PDB (`0x37`
//! `EXTRA_ENABLESTATEPARENT`, `0x3B` `EXTRA_TELEPORTMARKER`, `0x51`
//! `EXTRA_LINKED_REF`, ...). The functions of the same unit outside this
//! file (`GetExtraData` `00410220`, `AddExtra` `0040ff60`, the two
//! `RemoveExtra` `00410020` and `00410140`, the node constructor `00414010`)
//! are called by address, like every other callee. The compiler's
//! exception-unwinding frames (the `FS:[0]` chains of the functions that
//! allocate) are not translated.

#[allow(unused_imports)]
use super::extradatalist::*;
use super::extradataobjects::{
    ExtraActivateRef, ExtraActivateRefChildren, ExtraAshPileRef, ExtraEnableStateChildren,
    ExtraEnableStateParent, ExtraLinkedRef, ExtraLinkedRefChildren, ExtraRandomTeleportMarker,
    NiPoint3, RefActivateData, RefReflectData,
};
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, BSStringT};

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `ExtraItemDropper` (Xbox PDB), type `0x39`, 0x10 bytes.
    pub struct ExtraItemDropper: 0x10 {
        /// `pDropper` (Xbox PDB): `TESObjectREFR*`.
        0x0C pDropper: Ptr,
    }

    /// `ExtraDroppedItemList` (Xbox PDB), type `0x3A`, 0x14 bytes.
    pub struct ExtraDroppedItemList: 0x14 {
        /// `DroppedItemList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x0C DroppedItemList: Inline<BSSimpleList>,
    }

    /// `ExtraCellWaterType` (Xbox PDB), type 3, 0x10 bytes.
    pub struct ExtraCellWaterType: 0x10 {
        /// `pWater` (Xbox PDB): `TESWaterForm*`.
        0x0C pWater: Ptr,
    }

    /// `ExtraOpenCloseActivateRef` (Xbox PDB), type `0x6C`, 0x10 bytes.
    pub struct ExtraOpenCloseActivateRef: 0x10 {
        /// `pActivateRef` (Xbox PDB): `TESObjectREFR*`.
        0x0C pActivateRef: Ptr,
    }

    /// `ExtraPrimitive` (Xbox PDB), type `0x6B`, 0x10 bytes: owns its
    /// primitive.
    pub struct ExtraPrimitive: 0x10 {
        /// `pPrimitive` (Xbox PDB): `BGSPrimitive*`.
        0x0C pPrimitive: Ptr,
    }

    /// `ExtraPatrolRefData` (Xbox PDB), type `0x6F`, 0x10 bytes: owns its
    /// patrol data.
    pub struct ExtraPatrolRefData: 0x10 {
        /// `pPatrolData` (Xbox PDB): `PatrolRefData*`.
        0x0C pPatrolData: Ptr,
    }

    /// `ExtraOcclusionPlaneRefData` (Xbox PDB), type `0x76`, 0x10 bytes:
    /// owns its data, four words (the references of the plane).
    pub struct ExtraOcclusionPlaneRefData: 0x10 {
        /// `pData` (Xbox PDB): `OcclusionPlaneLinkedRefData*`, 0x10 bytes.
        0x0C pData: Ptr,
    }

    /// `ExtraPortalRefData` (Xbox PDB), type `0x77`, 0x10 bytes: owns its
    /// data, two words (the references of the portal).
    pub struct ExtraPortalRefData: 0x10 {
        /// `pData` (Xbox PDB): `PortalLinkedRefData*`, 8 bytes
        /// (`pLinkedRefs`, two `TESObjectREFR*` on PC).
        0x0C pData: Ptr,
    }

    /// `ExtraRoomRefData` (Xbox PDB), type `0x7B`, 0x10 bytes: owns its data.
    pub struct ExtraRoomRefData: 0x10 {
        /// `pData` (Xbox PDB): `RoomLinkedRefData*`.
        0x0C pData: Ptr,
    }

    /// `RoomLinkedRefData` (Xbox PDB), 0x14 bytes.
    pub struct RoomLinkedRefData: 0x14 {
        /// `PortalList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x00 PortalList: Inline<BSSimpleList>,
        /// `RoomList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x08 RoomList: Inline<BSSimpleList>,
        /// `cMaster` (Xbox PDB): the room is a master room.
        0x10 cMaster: u8,
    }

    /// `ExtraCollisionData` (Xbox PDB), type `0x72`, 0x10 bytes: owns its
    /// collision data (freed with `operator delete`).
    pub struct ExtraCollisionData: 0x10 {
        /// `pCollisionData` (Xbox PDB): `CollisionData*`.
        0x0C pCollisionData: Ptr,
    }

    /// `ExtraPackageData` (Xbox PDB), type `0x70`, 0x10 bytes: the Xbox PDB
    /// names the extra data class of `SetActorPackageData` so.
    pub struct ExtraPackageData: 0x10 {
        /// `pActorPackageData` (Xbox PDB): `ActorPackageData*`.
        0x0C pActorPackageData: Ptr,
    }
}

// ---------------------------------------------------------------------------
// Constants

/// `ExtraEnableStateParent` (`EXTRA_ENABLESTATEPARENT`).
const EXTRA_ENABLE_STATE_PARENT: u8 = 0x37;
/// `ExtraEnableStateChildren` (`EXTRA_ENABLESTATECHILDREN`).
const EXTRA_ENABLE_STATE_CHILDREN: u8 = 0x38;
/// `ExtraItemDropper` (`EXTRA_ITEMDROPPER`).
const EXTRA_ITEM_DROPPER_TYPE: u8 = 0x39;
/// `ExtraDroppedItemList` (`EXTRA_DROPPEDITEMLIST`).
const EXTRA_DROPPED_ITEM_LIST: u8 = 0x3a;
/// `ExtraCellWaterType` (`EXTRA_WATERTYPE`).
const EXTRA_WATER_TYPE: u8 = 0x03;
/// `ExtraRandomTeleportMarker` (`EXTRA_TELEPORTMARKER`).
const EXTRA_TELEPORT_MARKER: u8 = 0x3b;
/// `ExtraAshPileRef` (`EXTRA_ASHPILE_REF`).
const EXTRA_ASH_PILE_REF: u8 = 0x89;
/// `ExtraLinkedRef` (`EXTRA_LINKED_REF`).
const EXTRA_LINKED_REF_TYPE: u8 = 0x51;
/// `ExtraLinkedRefChildren` (`EXTRA_LINKED_REF_CHILDREN`).
const EXTRA_LINKED_REF_CHILDREN_TYPE: u8 = 0x52;
/// `ExtraActivateRef` (`EXTRA_ACTIVATE_REF`).
const EXTRA_ACTIVATE_REF_TYPE: u8 = 0x53;
/// `ExtraOpenCloseActivateRef` (`EXTRA_OPENCLOSEACTIVATE_REF`).
const EXTRA_OPEN_CLOSE_ACTIVATE_REF_TYPE: u8 = 0x6c;

/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BaseExtraList::GetExtraData(type)` (`00410220`).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `BaseExtraList::AddExtra(extra)` (`0040ff60`).
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `BaseExtraList::RemoveExtra(extra, destroy)` (`00410020`).
const REMOVE_EXTRA: u32 = 0x0041_0020;
/// `BaseExtraList::RemoveExtra_ov2(type)` (`00410140`).
const REMOVE_EXTRA_BY_TYPE: u32 = 0x0041_0140;
/// `BSExtraData::BSExtraData(type)` (`0040ec80`): base vtable, type, null next.
const BS_EXTRA_DATA_INIT: u32 = 0x0040_ec80;
/// `00414010`: constructor of a two-word list node (item and next null; the
/// engine map files it as `BSSimpleList<REF_ACTIVATE_DATA_P>::AddHead`).
const LIST_NODE_INIT: u32 = 0x0041_4010;
/// `BSSimpleList::AddHead(&item)` (`005ae3d0`); `this` is the head node.
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
/// Whether the list at `this` holds an item equal to the word at the address
/// given (`005f65d0`).
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// Removes from the list at `this` the first node whose item equals the word
/// at the address given (`00905330`).
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `BSSimpleList::IsEmpty`: no item and no next (`008256d0`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `MOV EAX,ECX`: the address of a list node is the address of its item slot
/// (`006815c0`).
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// `MOV EAX,[ECX+4]`: the next node of a list node (`00726070`).
const LIST_NEXT: u32 = 0x0072_6070;
/// `BGSSaveFormBuffer::GetForm` (`007af430`, `MOV EAX,[ECX+0x20]`): on a
/// reference, its base form.
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// `TESForm::GetFormType` (`00401170`, `MOVZX EAX,[ECX+4]`).
const FORM_TYPE: u32 = 0x0040_1170;
/// Form type the dropped item search looks for.
const FORM_TYPE_DROPPED_ITEM_MATCH: u32 = 0x28;
/// Search in an `ExtraActivateRef` (`00433a00`): the `REF_ACTIVATE_DATA` of
/// its parent list whose reference is the argument, or null.
const ACTIVATE_REF_FIND: u32 = 0x0043_3a00;

/// Constructors of the extra data (`this` = the new block, no other
/// argument), in `extradataobjects.cpp` and `extradatacell.cpp`.
const ENABLE_STATE_CHILDREN_INIT: u32 = 0x0043_33a0;
const ITEM_DROPPER_INIT: u32 = 0x0043_5920;
const DROPPED_ITEM_LIST_INIT: u32 = 0x0043_5950;
const WATER_TYPE_INIT: u32 = 0x0040_f370;
const TELEPORT_MARKER_INIT: u32 = 0x0043_34b0;
const ASH_PILE_REF_INIT: u32 = 0x0043_36c0;
const LINKED_REF_INIT: u32 = 0x0043_3640;
const LINKED_REF_CHILDREN_INIT: u32 = 0x0043_3530;
const ACTIVATE_REF_INIT: u32 = 0x0043_38b0;

/// Vtable set by the constructor `0041e750`.
const VTABLE_EXTRA_OPEN_CLOSE_ACTIVATE_REF: u32 = 0x0101_51a8;

/// `ExtraActivateRefChildren` (`EXTRA_ACTIVATE_REF_CHILDREN`).
const EXTRA_ACTIVATE_REF_CHILDREN_TYPE: u8 = 0x54;
/// `ExtraDecalRefs` (`EXTRA_DECAL_REFS`).
const EXTRA_DECAL_REFS_TYPE: u8 = 0x57;
/// `ExtraReflectedRefs` (`EXTRA_REFLECTED_REFS`).
const EXTRA_REFLECTED_REFS_TYPE: u8 = 0x65;
/// `ExtraReflectorRefs` (`EXTRA_REFLECTOR_REFS`).
const EXTRA_REFLECTOR_REFS_TYPE: u8 = 0x66;
/// `ExtraPrimitive` (`EXTRA_PRIMITIVE`).
const EXTRA_PRIMITIVE_TYPE: u8 = 0x6b;
/// `ExtraPatrolRefData` (`EXTRA_PATROL_REF_DATA`).
const EXTRA_PATROL_REF_DATA_TYPE: u8 = 0x6f;
/// `ExtraOcclusionPlaneRefData` (`EXTRA_OCCLUSION_PLANE_REF_DATA`).
const EXTRA_OCCLUSION_PLANE_REF_DATA_TYPE: u8 = 0x76;
/// `ExtraPortalRefData` (`EXTRA_PORTAL_REF_DATA`).
const EXTRA_PORTAL_REF_DATA_TYPE: u8 = 0x77;
/// `ExtraWaterLightRefs` (`EXTRA_WATER_LIGHT_REFS`).
const EXTRA_WATER_LIGHT_REFS_TYPE: u8 = 0x84;
/// `ExtraLitWaterRefs` (`EXTRA_LIT_WATER_REFS`).
const EXTRA_LIT_WATER_REFS_TYPE: u8 = 0x85;

/// Constructors of the extra data and of the data they own (`this` = the new
/// block): in `extradataobjects.cpp` and `extradatalist.cpp`.
const ACTIVATE_REF_CHILDREN_INIT: u32 = 0x0043_3790;
const DECAL_REFS_INIT: u32 = 0x0043_3ca0;
const REFLECTED_REFS_INIT: u32 = 0x0041_1b40;
const REFLECTOR_REFS_INIT: u32 = 0x0041_1be0;
const WATER_LIGHT_REFS_INIT: u32 = 0x0041_1c80;
const LIT_WATER_REFS_INIT: u32 = 0x0041_1d20;
/// Constructor of the four-word `OcclusionPlaneLinkedRefData` (`00411dc0`).
const OCCLUSION_PLANE_DATA_INIT: u32 = 0x0041_1dc0;

/// `ExtraDecalRefs` method (`00434480`) that adds, or updates, the decal
/// of a reference: `(this, reference, intersect*, normal*)`.
const DECAL_REFS_ADD: u32 = 0x0043_4480;
/// Method of `ExtraReflectedRefs` and `ExtraReflectorRefs` (`00434800`)
/// that removes the entry of a reference: `(this, reference)`.
const REFLECT_REFS_REMOVE: u32 = 0x0043_4800;
/// Method of `ExtraReflectedRefs` and `ExtraReflectorRefs` (`004348a0`)
/// that sets the effect flags of the entry of a reference, adding the entry
/// when there is none: `(this, reference, flags)`.
const REFLECT_REFS_SET_FLAGS: u32 = 0x0043_48a0;
/// Stores its argument at `this + 0x0C` (`0041fd00`, `RET 4`; the engine
/// map files it as `NonActorMagicCaster::SetCurrentSpell`).
const STORE_WORD_AT_0C: u32 = 0x0041_fd00;
/// `PatrolRefData::Compare(other)` (Xbox PDB, `0067c720`): true when the two
/// differ.
const PATROL_DATA_COMPARE: u32 = 0x0067_c720;
/// Destructor body of `PatrolRefData` (`0067c670`).
const PATROL_DATA_DESTROY: u32 = 0x0067_c670;

/// `BSStringT<char>` copy constructor (`004047f0`): `this` = the new string,
/// the argument the string copied.
const STRING_COPY_CONSTRUCT: u32 = 0x0040_47f0;
/// `BSStringT<char>` constructor from characters (`0040c0e0`): `this` = the
/// new string, the argument the address of a C string.
const STRING_CONSTRUCT_FROM_CHARS: u32 = 0x0040_c0e0;
/// `BSStringT<char>` assignment from characters (`004037f0`): `this` = the
/// string, then the address of a C string and a second word (0 here).
const STRING_SET: u32 = 0x0040_37f0;
/// An empty C string in `.rdata`.
const EMPTY_TEXT: u32 = 0x0101_1584;
/// The float (-1.0) `GetActivateChildrenTimer` gives when there is no such
/// extra data.
const NO_TIMER: u32 = 0x0101_2054;

/// Vtables set by the constructors `0041faf0`, `0041fcd0` and `0041ff80`.
const VTABLE_EXTRA_PRIMITIVE: u32 = 0x0101_51b4;
const VTABLE_EXTRA_PATROL_REF_DATA: u32 = 0x0101_51c0;
const VTABLE_EXTRA_OCCLUSION_PLANE_REF_DATA: u32 = 0x0101_51cc;

/// Vtables set by the constructors `004202d0`, `00420500`, `00421090` and
/// `00421280`.
const VTABLE_EXTRA_PORTAL_REF_DATA: u32 = 0x0101_51d8;
const VTABLE_EXTRA_ROOM_REF_DATA: u32 = 0x0101_51e4;
const VTABLE_EXTRA_COLLISION_DATA: u32 = 0x0101_51f0;
const VTABLE_EXTRA_PACKAGE_DATA: u32 = 0x0101_51fc;

/// `ExtraPackageData` (`EXTRA_PACKAGE_DATA`).
const EXTRA_PACKAGE_DATA_TYPE: u8 = 0x70;
/// `ExtraCollisionData` (`EXTRA_COLLISION_DATA`).
const EXTRA_COLLISION_DATA_TYPE: u8 = 0x72;
/// `ExtraPortal` (`EXTRA_PORTAL`): holds a `NiPointer<BSPortal>` at +0x0C.
const EXTRA_PORTAL_TYPE: u8 = 0x78;
/// `ExtraRoom` (`EXTRA_ROOM`): holds a `NiPointer<BSMultiBoundRoom>` at
/// +0x0C.
const EXTRA_ROOM_TYPE: u8 = 0x79;
/// `ExtraRoomRefData` (`EXTRA_ROOM_REF_DATA`).
const EXTRA_ROOM_REF_DATA_TYPE: u8 = 0x7b;
/// `ExtraGuardedRefData` (`EXTRA_GUARDED_REF_DATA`).
const EXTRA_GUARDED_REF_DATA_TYPE: u8 = 0x7c;

/// Constructors of this session (`this` = the new block): the portal
/// linked data (8 bytes, `004143c0`), the room linked data (0x14 bytes,
/// `00416b80`), `ExtraPortal` (`00435820`), `ExtraRoom` (`004358a0`) and
/// `ExtraGuardedRefData` (0x1c bytes, `00430f30`).
const PORTAL_LINKED_DATA_INIT: u32 = 0x0041_43c0;
const ROOM_LINKED_DATA_INIT: u32 = 0x0041_6b80;
const EXTRA_PORTAL_INIT: u32 = 0x0043_5820;
const EXTRA_ROOM_INIT: u32 = 0x0043_58a0;
const EXTRA_GUARDED_REF_DATA_INIT: u32 = 0x0043_0f30;
/// `ExtraGuardedRefData::AddGuard(guard)` (Xbox PDB, `004310d0`).
const GUARDED_REF_DATA_ADD_GUARD: u32 = 0x0043_10d0;
/// Method of `ExtraGuardedRefData` (`00431120`) taking two words, called by
/// `004213c0`; the engine map gives it no name.
const GUARDED_REF_DATA_TWO_WORDS: u32 = 0x0043_1120;
/// Destructor body of the two `BSSimpleList`s of a `RoomLinkedRefData`
/// (`0046ffb0`, `this` = the list).
const LIST_DESTROY: u32 = 0x0046_ffb0;
/// `NiPointer<T>::operator=(T*)` (`0066b0d0`): `this` is the address of the
/// pointer; references the new object and releases the old one.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `TESObjectREFR` to its `ExtraDataList` (`005d43c0`, `this + 0x44`).
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// Constructor of the `NiTMap<TESObjectREFR *, bool>` visited set of
/// `GetMasterRoom` (`0042f4e0(this, hash size)`) and its destructor body
/// (`0042fe80(this)`); the set is 0x10 bytes.
const VISITED_SET_INIT: u32 = 0x0042_f4e0;
const VISITED_SET_DESTROY: u32 = 0x0042_fe80;
/// `NiTMap::GetAt(key, &value)` (`0057c850`): true and `value` set when
/// the key is there.
const VISITED_SET_FIND: u32 = 0x0057_c850;
/// `NiTMap::SetAt(key, value)` (`0084d310`).
const VISITED_SET_INSERT: u32 = 0x0084_d310;
/// `009707f0(this, room)`: stores `room` at `this + 0xFC` of a portal.
const PORTAL_SET_FIRST_ROOM: u32 = 0x0097_07f0;

/// Offsets of the two lists of a `RoomLinkedRefData` (`PortalList`,
/// `RoomList`).
const ROOM_DATA_PORTAL_LIST: u32 = 0x00;
const ROOM_DATA_ROOM_LIST: u32 = 0x08;

// ---------------------------------------------------------------------------
// Helpers

/// `list->GetExtraData(extra_type)`.
fn find_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> Ptr<BSExtraData> {
    e.call(GET_EXTRA_DATA, &args![list, extra_type as u32])
        .ptr()
}

/// `list->AddExtra(extra)`.
fn add_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    e.call(ADD_EXTRA, &args![list, extra]);
}

/// `list->RemoveExtra(extra, true)`: unlinks and deletes the extra data.
fn remove_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    e.call(REMOVE_EXTRA, &args![list, extra, 1u32]);
}

/// `list->RemoveExtra(extra_type)`.
fn remove_extra_by_type(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) {
    e.call(REMOVE_EXTRA_BY_TYPE, &args![list, extra_type as u32]);
}

/// `new T`: a block of `size` bytes built by the constructor at `construct`
/// (`this` = the block). As in the code, a failed allocation gives null and
/// the constructor is not run.
fn new_object(e: &mut Engine, size: u32, construct: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        0
    } else {
        e.call(construct, &args![block]).u32()
    }
}

/// The word at +0x0C of the first extra data of `extra_type`, or 0.
fn extra_word_or_zero(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    }
}

/// The address of the list at +0x0C of the first extra data of `extra_type`,
/// or null.
fn extra_list_or_null(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
) -> Ptr<BSSimpleList> {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        Ptr::NULL
    } else {
        extra.byte_add(0x0c).cast()
    }
}

/// The shape of the setters of a reference (water, teleport marker, linked
/// reference): a null `value` deletes the extra data of `extra_type` (by
/// type); otherwise the word at +0x0C of the existing one is overwritten, or
/// a new `0x10`-byte one is built by `construct`, given the value, and
/// added.
fn set_word_or_remove_by_type(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    value: u32,
) {
    if value == 0 {
        remove_extra_by_type(e, list, extra_type);
        return;
    }
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        let block = new_object(e, 0x10, construct);
        e.mem.set_u32(block + 0x0c, value);
        add_extra(e, list, Ptr::new(block));
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, value);
    }
}

/// The shape of the adders to a reference list: when `item` is not null,
/// finds the extra data of `extra_type` or builds a `0x14`-byte one with
/// `construct` and adds it, then puts `item` at the head of the list at
/// +0x0C unless the list already holds it.
fn add_to_reference_list(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    item: u32,
) {
    if item == 0 {
        return;
    }
    let mut extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x14, construct));
        add_extra(e, list, extra);
    }
    let head = extra.addr() + 0x0c;
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        if !e.call(LIST_CONTAINS, &args![head, slot]).bool() {
            e.call(LIST_ADD_HEAD, &args![head, slot]);
        }
    });
}

/// The shape of the removers from a reference list: when `item` is not
/// null and the list has an extra data of `extra_type`, removes `item` from
/// the list at +0x0C, and deletes the extra data if that left the list empty.
fn remove_from_reference_list(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8, item: u32) {
    if item == 0 {
        return;
    }
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        return;
    }
    let head = extra.addr() + 0x0c;
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_REMOVE_ITEM, &args![head, slot]);
    });
    if e.call(LIST_IS_EMPTY, &args![head]).bool() {
        remove_extra(e, list, extra);
    }
}

/// A new `REF_ACTIVATE_DATA` (8 bytes, node constructor `00414010`): its
/// address, or 0 when the allocation fails (then the code stores through the
/// null pointer all the same; the model has no failing allocation).
fn new_activate_data(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(LIST_NODE_INIT, &args![block]).u32()
    }
}

/// Stores `reference` in `entry` (`pActivateRef`), then puts the entry at the
/// head of the `ParentList` of `extra` (`AddHead` is given the address of a
/// local holding the entry).
fn push_activate_parent(e: &mut Engine, extra: Ptr<BSExtraData>, entry: u32, reference: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.mem.set_u32(entry, reference);
        e.call(LIST_ADD_HEAD, &args![extra.addr() + 0x0c, slot]);
    });
}

// Translated from 0041db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag bit 0 of `cFlags` of the type `0x37` extra data
/// (`ExtraEnableStateParent`, `0041db30`), false when the list has none. No
/// Xbox PDB name.
pub fn fn_0041db00(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    let extra = find_extra(e, this, EXTRA_ENABLE_STATE_PARENT);
    if extra.is_null() {
        false
    } else {
        fn_0041db30(e, extra.cast())
    }
}

// Translated from 0041db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of `cFlags` (+0x10) of an `ExtraEnableStateParent`.
pub fn fn_0041db30(e: &mut Engine, this: Ptr<ExtraEnableStateParent>) -> bool {
    e.get(this, ExtraEnableStateParent::cFlags) & 1 != 0
}

// Translated from 0041db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears (`value`) flag bit 0 of the type `0x37` extra data
/// (`0041db80`); does nothing when the list has none.
pub fn fn_0041db50(e: &mut Engine, this: Ptr<ExtraDataList>, value: bool) {
    let extra = find_extra(e, this, EXTRA_ENABLE_STATE_PARENT);
    if !extra.is_null() {
        fn_0041db80(e, extra.cast(), value);
    }
}

// Translated from 0041db80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` true) or clears bit 0 of `cFlags` (+0x10) of an
/// `ExtraEnableStateParent`.
pub fn fn_0041db80(e: &mut Engine, this: Ptr<ExtraEnableStateParent>, value: bool) {
    let flags = e.get(this, ExtraEnableStateParent::cFlags);
    let flags = if value { flags | 1 } else { flags & 0xfe };
    e.set(this, ExtraEnableStateParent::cFlags, flags);
}

// Translated from 0041dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::ShouldPopInWhenEnabledByParent` (Xbox PDB): flag bit 1 of
/// `cFlags` of the type `0x37` extra data (`0041dc00`), false when the list
/// has none.
pub fn extra_data_list_should_pop_in_when_enabled_by_parent(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> bool {
    let extra = find_extra(e, this, EXTRA_ENABLE_STATE_PARENT);
    if extra.is_null() {
        false
    } else {
        fn_0041dc00(e, extra.cast())
    }
}

// Translated from 0041dc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 of `cFlags` (+0x10) of an `ExtraEnableStateParent`.
pub fn fn_0041dc00(e: &mut Engine, this: Ptr<ExtraEnableStateParent>) -> bool {
    e.get(this, ExtraEnableStateParent::cFlags) & 2 != 0
}

// Translated from 0041dc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag bit 2 of `cFlags` of the type `0x37` extra data (`0041dc50`), false
/// when the list has none. No Xbox PDB name.
pub fn fn_0041dc20(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    let extra = find_extra(e, this, EXTRA_ENABLE_STATE_PARENT);
    if extra.is_null() {
        false
    } else {
        fn_0041dc50(e, extra.cast())
    }
}

// Translated from 0041dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 2 of `cFlags` (+0x10) of an `ExtraEnableStateParent`.
pub fn fn_0041dc50(e: &mut Engine, this: Ptr<ExtraEnableStateParent>) -> bool {
    e.get(this, ExtraEnableStateParent::cFlags) & 4 != 0
}

// Translated from 0041dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `flags` in the whole `cFlags` byte of the type `0x37` extra data;
/// does nothing when the list has none. No Xbox PDB name.
pub fn fn_0041dc70(e: &mut Engine, this: Ptr<ExtraDataList>, flags: u8) {
    let extra: Ptr<ExtraEnableStateParent> = find_extra(e, this, EXTRA_ENABLE_STATE_PARENT).cast();
    if !extra.is_null() {
        e.set(extra, ExtraEnableStateParent::cFlags, flags);
    }
}

// Translated from 0041dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ChildList` (+0x0C, the inline head node) of the type `0x38` extra
/// data (`ExtraEnableStateChildren`), or null when the list has none.
pub fn fn_0041dca0(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSSimpleList> {
    let extra: Ptr<ExtraEnableStateChildren> =
        find_extra(e, this, EXTRA_ENABLE_STATE_CHILDREN).cast();
    if extra.is_null() {
        Ptr::NULL
    } else {
        extra.at(ExtraEnableStateChildren::ChildList)
    }
}

// Translated from 0041dcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `child` to the `ChildList` of the type `0x38` extra data
/// (`ExtraEnableStateChildren`, `0x14` bytes, `004333a0`), which it builds
/// and adds to the list when there is none, unless the child is already in
/// the list. A null `child` does nothing. No Xbox PDB name.
pub fn fn_0041dcd0(e: &mut Engine, this: Ptr<ExtraDataList>, child: u32) {
    add_to_reference_list(
        e,
        this,
        EXTRA_ENABLE_STATE_CHILDREN,
        ENABLE_STATE_CHILDREN_INIT,
        child,
    );
}

// Translated from 0041dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `child` from the `ChildList` of the type `0x38` extra data and
/// deletes the extra data when the list is then empty. A null `child`, or no
/// such extra data, does nothing. No Xbox PDB name.
pub fn fn_0041dda0(e: &mut Engine, this: Ptr<ExtraDataList>, child: u32) {
    remove_from_reference_list(e, this, EXTRA_ENABLE_STATE_CHILDREN, child);
}

// Translated from 0041de00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetItemDropper` (Xbox PDB): `pDropper` of the type `0x39`
/// extra data (`ExtraItemDropper`), or 0.
pub fn extra_data_list_get_item_dropper(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_ITEM_DROPPER_TYPE)
}

// Translated from 0041de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddDroppedItem` (Xbox PDB; the body sets `pDropper`): a
/// null `dropper` deletes the type `0x39` extra data (by type); otherwise the
/// extra data is found or built (`0x10` bytes, `00435920`) and added, and
/// `dropper` is stored in it (after the add, for a new one).
pub fn extra_data_list_add_dropped_item(e: &mut Engine, this: Ptr<ExtraDataList>, dropper: u32) {
    if dropper == 0 {
        remove_extra_by_type(e, this, EXTRA_ITEM_DROPPER_TYPE);
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_ITEM_DROPPER_TYPE);
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x10, ITEM_DROPPER_INIT));
        add_extra(e, this, extra);
    }
    e.set(extra.cast(), ExtraItemDropper::pDropper, Ptr::new(dropper));
}

// Translated from 0041df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the dropped item list (`DroppedItemList` of the type `0x3A` extra
/// data) and returns the first reference whose base form (`007af430`) has
/// form type `0x28` (`00401170`). The walk stops with 0 at the first node
/// whose item is null, and ends with 0 when the list has none or no match.
/// No Xbox PDB name.
pub fn fn_0041df00(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let mut found = 0;
    let extra = find_extra(e, this, EXTRA_DROPPED_ITEM_LIST);
    if extra.is_null() {
        return 0;
    }
    let mut node = extra.addr() + 0x0c;
    while found == 0 && node != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item == 0 {
            return 0;
        }
        let form = e.call(REFERENCE_BASE_FORM, &args![item]).u32();
        let form_type = e.call(FORM_TYPE, &args![form]).u32();
        if form_type == FORM_TYPE_DROPPED_ITEM_MATCH {
            found = item;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    found
}

// Translated from 0041df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetDroppedItemList` (Xbox PDB): the `DroppedItemList`
/// (+0x0C, the inline head node) of the type `0x3A` extra data, or null.
pub fn extra_data_list_get_dropped_item_list(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    extra_list_or_null(e, this, EXTRA_DROPPED_ITEM_LIST)
}

// Translated from 0041dfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveDroppedItemList` (Xbox PDB): deletes the type `0x3A`
/// extra data (by type) when the list has one.
pub fn extra_data_list_remove_dropped_item_list(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_DROPPED_ITEM_LIST);
    if !extra.is_null() {
        remove_extra_by_type(e, this, EXTRA_DROPPED_ITEM_LIST);
    }
}

// Translated from 0041e000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `item` to the `DroppedItemList` of the type `0x3A` extra data
/// (`0x14` bytes, `00435950`, built and added when there is none), unless the
/// list already holds it. A null `item` does nothing. No Xbox PDB name.
pub fn fn_0041e000(e: &mut Engine, this: Ptr<ExtraDataList>, item: u32) {
    add_to_reference_list(
        e,
        this,
        EXTRA_DROPPED_ITEM_LIST,
        DROPPED_ITEM_LIST_INIT,
        item,
    );
}

// Translated from 0041e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveDroppedItem` (Xbox PDB): removes `item` from the
/// `DroppedItemList` of the type `0x3A` extra data and deletes the extra data
/// when the list is then empty.
pub fn extra_data_list_remove_dropped_item(e: &mut Engine, this: Ptr<ExtraDataList>, item: u32) {
    remove_from_reference_list(e, this, EXTRA_DROPPED_ITEM_LIST, item);
}

// Translated from 0041e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWaterType` (Xbox PDB): `pWater` of the type 3 extra data
/// (`ExtraCellWaterType`), or 0.
pub fn extra_data_list_get_water_type(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_WATER_TYPE)
}

// Translated from 0041e160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pWater` of the type 3 extra data: a null `water` deletes it (by
/// type); otherwise the existing one is overwritten, or a `0x10`-byte one
/// (`0040f370`) is built, given the water and added. No Xbox PDB name.
pub fn fn_0041e160(e: &mut Engine, this: Ptr<ExtraDataList>, water: u32) {
    set_word_or_remove_by_type(e, this, EXTRA_WATER_TYPE, WATER_TYPE_INIT, water);
}

// Translated from 0041e220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pMarker` of the type `0x3B` extra data (`ExtraRandomTeleportMarker`), or
/// 0. No Xbox PDB name.
pub fn fn_0041e220(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra: Ptr<ExtraRandomTeleportMarker> = find_extra(e, this, EXTRA_TELEPORT_MARKER).cast();
    if extra.is_null() {
        0
    } else {
        e.get(extra, ExtraRandomTeleportMarker::pMarker).addr()
    }
}

// Translated from 0041e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pMarker` of the type `0x3B` extra data: a null `marker` deletes it
/// (by type); otherwise the existing one is overwritten, or a `0x10`-byte one
/// (`004334b0`) is built, given the marker and added. No Xbox PDB name.
pub fn fn_0041e250(e: &mut Engine, this: Ptr<ExtraDataList>, marker: u32) {
    set_word_or_remove_by_type(e, this, EXTRA_TELEPORT_MARKER, TELEPORT_MARKER_INIT, marker);
}

// Translated from 0041e310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetAshPileRef` (Xbox PDB): `pAshPileRef` of the type
/// `0x89` extra data (`ExtraAshPileRef`), or 0.
pub fn extra_data_list_get_ash_pile_ref(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra: Ptr<ExtraAshPileRef> = find_extra(e, this, EXTRA_ASH_PILE_REF).cast();
    if extra.is_null() {
        0
    } else {
        e.get(extra, ExtraAshPileRef::pAshPileRef).addr()
    }
}

// Translated from 0041e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pAshPileRef` of the type `0x89` extra data. The extra data is looked
/// up first: a null `reference` deletes it (the object itself, if any); a
/// non-null one is stored in the existing extra data, or in a `0x10`-byte one
/// (`004336c0`) built and added first. No Xbox PDB name.
pub fn fn_0041e340(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    let mut extra = find_extra(e, this, EXTRA_ASH_PILE_REF);
    if reference == 0 {
        if !extra.is_null() {
            remove_extra(e, this, extra);
        }
        return;
    }
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x10, ASH_PILE_REF_INIT));
        add_extra(e, this, extra);
    }
    e.set(
        extra.cast(),
        ExtraAshPileRef::pAshPileRef,
        Ptr::new(reference),
    );
}

// Translated from 0041e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pLinkedRef` of the type `0x51` extra data (`ExtraLinkedRef`), or 0. No
/// Xbox PDB name.
pub fn fn_0041e410(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra: Ptr<ExtraLinkedRef> = find_extra(e, this, EXTRA_LINKED_REF_TYPE).cast();
    if extra.is_null() {
        0
    } else {
        e.get(extra, ExtraLinkedRef::pLinkedRef).addr()
    }
}

// Translated from 0041e440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pLinkedRef` of the type `0x51` extra data: a null `reference`
/// deletes it (by type); otherwise the existing one is overwritten, or a
/// `0x10`-byte one (`00433640`) is built, given the reference and added. No
/// Xbox PDB name.
pub fn fn_0041e440(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    set_word_or_remove_by_type(e, this, EXTRA_LINKED_REF_TYPE, LINKED_REF_INIT, reference);
}

// Translated from 0041e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetLinkedRefChildren` (Xbox PDB): the `ChildList` (+0x0C,
/// the inline head node) of the type `0x52` extra data
/// (`ExtraLinkedRefChildren`), or null.
pub fn extra_data_list_get_linked_ref_children(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    let extra: Ptr<ExtraLinkedRefChildren> =
        find_extra(e, this, EXTRA_LINKED_REF_CHILDREN_TYPE).cast();
    if extra.is_null() {
        Ptr::NULL
    } else {
        extra.at(ExtraLinkedRefChildren::ChildList)
    }
}

// Translated from 0041e530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `child` to the `ChildList` of the type `0x52` extra data
/// (`0x14` bytes, `00433530`, built and added when there is none), unless the
/// list already holds it. A null `child` does nothing. No Xbox PDB name.
pub fn fn_0041e530(e: &mut Engine, this: Ptr<ExtraDataList>, child: u32) {
    add_to_reference_list(
        e,
        this,
        EXTRA_LINKED_REF_CHILDREN_TYPE,
        LINKED_REF_CHILDREN_INIT,
        child,
    );
}

// Translated from 0041e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `child` from the `ChildList` of the type `0x52` extra data and
/// deletes the extra data when the list is then empty. No Xbox PDB name.
pub fn fn_0041e600(e: &mut Engine, this: Ptr<ExtraDataList>, child: u32) {
    remove_from_reference_list(e, this, EXTRA_LINKED_REF_CHILDREN_TYPE, child);
}

// Translated from 0041e660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pActivateRef` of the type `0x6C` extra data (`ExtraOpenCloseActivateRef`),
/// or 0. No Xbox PDB name.
pub fn fn_0041e660(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra: Ptr<ExtraOpenCloseActivateRef> =
        find_extra(e, this, EXTRA_OPEN_CLOSE_ACTIVATE_REF_TYPE).cast();
    if extra.is_null() {
        0
    } else {
        e.get(extra, ExtraOpenCloseActivateRef::pActivateRef).addr()
    }
}

// Translated from 0041e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetOpenCloseActivateRef` (Xbox PDB): a null `reference`
/// deletes the type `0x6C` extra data (by type); otherwise the existing one
/// is overwritten, or a `0x10`-byte one is built with the constructor of this
/// file (`0041e750`), given the reference and added.
pub fn extra_data_list_set_open_close_activate_ref(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: u32,
) {
    if reference == 0 {
        remove_extra_by_type(e, this, EXTRA_OPEN_CLOSE_ACTIVATE_REF_TYPE);
        return;
    }
    let extra = find_extra(e, this, EXTRA_OPEN_CLOSE_ACTIVATE_REF_TYPE);
    if extra.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let built = if block == 0 {
            0
        } else {
            fn_0041e750(e, Ptr::new(block)).addr()
        };
        e.mem.set_u32(built + 0x0c, reference);
        add_extra(e, this, Ptr::new(built));
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, reference);
    }
}

// Translated from 0041e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraOpenCloseActivateRef` (type `0x6C`): the base
/// constructor (`0040ec80`) with the type, the vtable `010151a8`, and a null
/// `pActivateRef`. Returns `this`.
pub fn fn_0041e750(
    e: &mut Engine,
    this: Ptr<ExtraOpenCloseActivateRef>,
) -> Ptr<ExtraOpenCloseActivateRef> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_OPEN_CLOSE_ACTIVATE_REF_TYPE as u32],
    );
    e.mem
        .set_u32(this.addr(), VTABLE_EXTRA_OPEN_CLOSE_ACTIVATE_REF);
    e.set(this, ExtraOpenCloseActivateRef::pActivateRef, Ptr::NULL);
    this
}

// Translated from 0041e780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ParentList` (+0x0C, the inline head node) of the type `0x53` extra
/// data (`ExtraActivateRef`), or null. No Xbox PDB name.
pub fn fn_0041e780(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSSimpleList> {
    let extra: Ptr<ExtraActivateRef> = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE).cast();
    if extra.is_null() {
        Ptr::NULL
    } else {
        extra.at(ExtraActivateRef::ParentList)
    }
}

// Translated from 0041e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `REF_ACTIVATE_DATA` of the type `0x53` extra data's parent list whose
/// reference is `reference` (`00433a00`), or 0 when the list has no such
/// extra data or the parent is not in it. No Xbox PDB name.
pub fn fn_0041e7b0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) -> u32 {
    let extra = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE);
    if extra.is_null() {
        0
    } else {
        e.call(ACTIVATE_REF_FIND, &args![extra, reference]).u32()
    }
}

// Translated from 0041e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `reference` as an activate parent: a new `REF_ACTIVATE_DATA` (8
/// bytes, node constructor `00414010`, `pActivateRef` = `reference`) goes to
/// the head of the `ParentList` of the type `0x53` extra data (`0x20` bytes,
/// `004338b0`, built and added when there is none), unless the existing extra
/// data already has an entry for `reference` (`00433a00`). A null `reference`
/// does nothing. No Xbox PDB name.
pub fn fn_0041e7f0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    if reference == 0 {
        return;
    }
    let extra = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE);
    let extra = if extra.is_null() {
        let built = Ptr::new(new_object(e, 0x20, ACTIVATE_REF_INIT));
        add_extra(e, this, built);
        built
    } else {
        if e.call(ACTIVATE_REF_FIND, &args![extra, reference]).u32() != 0 {
            return;
        }
        extra
    };
    let entry = new_activate_data(e);
    push_activate_parent(e, extra, entry, reference);
}

// Translated from 0041e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveActivateParent` (Xbox PDB): finds the entry for
/// `reference` in the `ParentList` of the type `0x53` extra data (`00433a00`),
/// takes it out of the list (`00905330`), deletes it, and deletes the extra
/// data (by type) when the list is then empty. A null `reference`, no such
/// extra data or no entry does nothing.
pub fn extra_data_list_remove_activate_parent(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: u32,
) {
    if reference == 0 {
        return;
    }
    let extra = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE);
    if extra.is_null() {
        return;
    }
    let entry = e.call(ACTIVATE_REF_FIND, &args![extra, reference]).u32();
    if entry == 0 {
        return;
    }
    let head = extra.addr() + 0x0c;
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_REMOVE_ITEM, &args![head, slot]);
    });
    e.call(OPERATOR_DELETE, &args![entry]);
    if e.call(LIST_IS_EMPTY, &args![head]).bool() {
        remove_extra_by_type(e, this, EXTRA_ACTIVATE_REF_TYPE);
    }
}

// Translated from 0041e9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fActivateDelay` (+4) of the `REF_ACTIVATE_DATA` for `reference` in the
/// type `0x53` extra data (`00433a00`), or 0.0 when there is none. Returned
/// in `ST0` (`FLD`, so a signalling NaN comes out quiet). No Xbox PDB name.
pub fn fn_0041e9e0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) -> f32 {
    let extra = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE);
    if !extra.is_null() {
        let entry: Ptr<RefActivateData> = e.call(ACTIVATE_REF_FIND, &args![extra, reference]).ptr();
        if !entry.is_null() {
            return x87_quiet(e.get(entry, RefActivateData::fActivateDelay));
        }
    }
    0.0
}

// Translated from 0041ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the activate delay of the entry for `reference` in the type `0x53`
/// extra data: the extra data is built (`0x20` bytes, `004338b0`) and added
/// when there is none; when it has no entry for `reference` (`00433a00`), a
/// new `REF_ACTIVATE_DATA` (8 bytes, `00414010`) goes to the head of the
/// `ParentList` and gets `pActivateRef` = `reference`; then `delay` is
/// stored in the entry (`FLD`/`FSTP`, so a signalling NaN is stored quiet).
/// A null `reference` does nothing. No Xbox PDB name.
pub fn fn_0041ea30(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, delay: f32) {
    if reference == 0 {
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE);
    let mut entry = 0;
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x20, ACTIVATE_REF_INIT));
        add_extra(e, this, extra);
    } else {
        entry = e.call(ACTIVATE_REF_FIND, &args![extra, reference]).u32();
    }
    if entry == 0 {
        entry = new_activate_data(e);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), entry);
            e.call(LIST_ADD_HEAD, &args![extra.addr() + 0x0c, slot]);
        });
        e.mem.set_u32(entry, reference);
    }
    e.set(
        Ptr::<RefActivateData>::new(entry),
        RefActivateData::fActivateDelay,
        x87_quiet(delay),
    );
}

// Translated from 0041eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of `cActivateFlags` (+0x14) of the type `0x53` extra data, false
/// when the list has none. No Xbox PDB name.
pub fn fn_0041eb60(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    let extra: Ptr<ExtraActivateRef> = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE).cast();
    !extra.is_null() && e.get(extra, ExtraActivateRef::cActivateFlags) & 1 != 0
}

/// `new T` whose constructor is a Rust function of this file: a block of
/// `size` bytes built by `build` (given the block). As in the code, a failed
/// allocation gives null and the constructor is not run.
fn new_built(e: &mut Engine, size: u32, build: impl FnOnce(&mut Engine, u32) -> u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        0
    } else {
        build(e, block)
    }
}

/// The shape of the setters that replace an extra data holding one owned
/// word (patrol reference data, occlusion plane reference data, portal
/// reference data): the extra data of `extra_type` is deleted when there is
/// one; then, when `value` is not null, a `0x10`-byte one is built by
/// `build`, given the value (`0041fd00`) and added.
fn replace_extra_holding_word(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    value: u32,
    build: impl FnOnce(&mut Engine, u32) -> u32,
) {
    let old = find_extra(e, list, extra_type);
    if !old.is_null() {
        remove_extra(e, list, old);
    }
    if value != 0 {
        let built = new_built(e, 0x10, build);
        e.call(STORE_WORD_AT_0C, &args![built, value]);
        add_extra(e, list, Ptr::new(built));
    }
}

/// The shape of the setters of one reflected or reflector reference
/// (`0041f1a0`, `0041f330`, `0041f4c0`, `0041f650`): walks the entries
/// (`REF_REFLECTED_DATA` / `REF_REFLECTOR_DATA`, the list at +0x0C of the
/// extra data of `extra_type`) up to the first one whose reference is
/// `reference`, stopping at an entry that is null. Found: with `set`, `bit`
/// is set in its `iEffectFlags`; otherwise `bit` is cleared, and if no flag
/// is left the entry is removed (`00434800`) and the extra data deleted when
/// its list is then empty. Not found, with `set`: the extra data is built
/// (`0x14` bytes, `construct`) and added if there is none, and the entry is
/// added with `bit` as flags (`004348a0`). Not found, without `set`: nothing.
fn set_reflect_ref_flag(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    reference: u32,
    set: bool,
    bit: u32,
) {
    let mut extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        let mut node = extra.addr() + 0x0c;
        while node != 0 {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let entry: Ptr<RefReflectData> = Ptr::new(e.mem.u32(slot));
            if entry.is_null() {
                break;
            }
            if e.get(entry, RefReflectData::pRef).addr() == reference {
                let flags = e.get(entry, RefReflectData::iEffectFlags);
                if set {
                    e.set(entry, RefReflectData::iEffectFlags, flags | bit);
                } else {
                    let flags = flags & !bit;
                    e.set(entry, RefReflectData::iEffectFlags, flags);
                    if flags == 0 {
                        e.call(REFLECT_REFS_REMOVE, &args![extra, reference]);
                        if e.call(LIST_IS_EMPTY, &args![extra.addr() + 0x0c]).bool() {
                            remove_extra(e, list, extra);
                        }
                    }
                }
                return;
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
    if set {
        if extra.is_null() {
            extra = Ptr::new(new_object(e, 0x14, construct));
            add_extra(e, list, extra);
        }
        e.call(REFLECT_REFS_SET_FLAGS, &args![extra, reference, bit]);
    }
}

/// The shape of the setters of the water light and lit water references:
/// `reference` is the only item of a one-word local whose address is given
/// to the list calls. With the extra data of `extra_type` present, `add`
/// puts the reference at the head of its list unless the list holds it, and
/// otherwise the reference is removed from the list (`00905330`), then the
/// extra data deleted if `delete_when_empty` and the list is empty. Without
/// it, `add` builds one (`0x14` bytes, `construct`), adds it and puts the
/// reference in its list; otherwise nothing happens.
fn change_reference_in_list(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    reference: u32,
    add: bool,
    delete_when_empty: bool,
) {
    let mut extra = find_extra(e, list, extra_type);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), reference);
        if !extra.is_null() {
            let head = extra.addr() + 0x0c;
            if add {
                if !e.call(LIST_CONTAINS, &args![head, slot]).bool() {
                    e.call(LIST_ADD_HEAD, &args![head, slot]);
                }
            } else {
                e.call(LIST_REMOVE_ITEM, &args![head, slot]);
                if delete_when_empty && e.call(LIST_IS_EMPTY, &args![head]).bool() {
                    remove_extra(e, list, extra);
                }
            }
        } else if add {
            extra = Ptr::new(new_object(e, 0x14, construct));
            add_extra(e, list, extra);
            e.call(LIST_ADD_HEAD, &args![extra.addr() + 0x0c, slot]);
        }
    });
}

// Translated from 0041eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` true) or clears bit 0 of `cActivateFlags` (+0x14) of the
/// type `0x53` extra data (`ExtraActivateRef`). With `flag` true and no such
/// extra data, one is built (`0x20` bytes, `004338b0`) and added first; with
/// `flag` false and none, nothing happens. No Xbox PDB name. The
/// exception-unwinding frame is not translated.
pub fn fn_0041eba0(e: &mut Engine, this: Ptr<ExtraDataList>, flag: bool) {
    let mut extra: Ptr<ExtraActivateRef> = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE).cast();
    if extra.is_null() && flag {
        extra = Ptr::new(new_object(e, 0x20, ACTIVATE_REF_INIT));
        add_extra(e, this, extra.cast());
    }
    if !extra.is_null() {
        let flags = e.get(extra, ExtraActivateRef::cActivateFlags);
        let flags = if flag { flags | 1 } else { flags & 0xfe };
        e.set(extra, ExtraActivateRef::cActivateFlags, flags);
    }
}

// Translated from 0041ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds in `out` (the caller's `BSStringT<char>`, an out parameter) a copy
/// of `ActivateTextOverride` (+0x18) of the type `0x53` extra data
/// (`004047f0`), or an empty string (`0040c0e0` on an empty C string) when
/// the list has none. Returns `out`. No Xbox PDB name.
pub fn fn_0041ec80(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr<BSStringT>,
) -> Ptr<BSStringT> {
    let extra: Ptr<ExtraActivateRef> = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE).cast();
    if extra.is_null() {
        e.call(STRING_CONSTRUCT_FROM_CHARS, &args![out, EMPTY_TEXT]);
    } else {
        let text = extra.at(ExtraActivateRef::ActivateTextOverride);
        e.call(STRING_COPY_CONSTRUCT, &args![out, text]);
    }
    out
}

// Translated from 0041ece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `ActivateTextOverride` of the type `0x53` extra data from the C
/// string at `text` (`004037f0`, second argument 0). With no such extra data
/// and a non-null `text`, one is built (`0x20` bytes, `004338b0`) and added
/// first; with no extra data and a null `text`, nothing happens. A null
/// `text` on an existing extra data is passed on all the same. No Xbox PDB
/// name. The exception-unwinding frame is not translated.
pub fn fn_0041ece0(e: &mut Engine, this: Ptr<ExtraDataList>, text: u32) {
    let mut extra: Ptr<ExtraActivateRef> = find_extra(e, this, EXTRA_ACTIVATE_REF_TYPE).cast();
    if extra.is_null() && text != 0 {
        extra = Ptr::new(new_object(e, 0x20, ACTIVATE_REF_INIT));
        add_extra(e, this, extra.cast());
    }
    if !extra.is_null() {
        let string = extra.at(ExtraActivateRef::ActivateTextOverride);
        e.call(STRING_SET, &args![string, text, 0u32]);
    }
}

// Translated from 0041eda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetActivateRefChildren` (Xbox PDB): the `ChildList`
/// (+0x0C, the inline head node) of the type `0x54` extra data
/// (`ExtraActivateRefChildren`), or null.
pub fn extra_data_list_get_activate_ref_children(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    let extra: Ptr<ExtraActivateRefChildren> =
        find_extra(e, this, EXTRA_ACTIVATE_REF_CHILDREN_TYPE).cast();
    if extra.is_null() {
        Ptr::NULL
    } else {
        extra.at(ExtraActivateRefChildren::ChildList)
    }
}

// Translated from 0041edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddActivateRefChild` (Xbox PDB): adds `child` to the
/// `ChildList` of the type `0x54` extra data (`0x18` bytes, `00433790`,
/// built and added when there is none) as a new `REF_ACTIVATE_DATA` (8
/// bytes, node constructor `00414010`, `pActivateRef` = `child`) at the head,
/// unless an entry of the list already names `child`. The walk stops at the
/// empty node. A null `child` does nothing. The exception-unwinding frame is
/// not translated.
pub fn extra_data_list_add_activate_ref_child(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    child: u32,
) {
    if child == 0 {
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_ACTIVATE_REF_CHILDREN_TYPE);
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x18, ACTIVATE_REF_CHILDREN_INIT));
        add_extra(e, this, extra);
    }
    let mut found = false;
    let mut node = extra.addr() + 0x0c;
    while node != 0 {
        if e.call(LIST_IS_EMPTY, &args![node]).bool() || found {
            break;
        }
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let entry: Ptr<RefActivateData> = Ptr::new(e.mem.u32(slot));
        if e.get(entry, RefActivateData::pActivateRef).addr() == child {
            found = true;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if !found {
        let entry = new_activate_data(e);
        push_activate_parent(e, extra, entry, child);
    }
}

// Translated from 0041ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the entry for `child` from the `ChildList` of the type `0x54`
/// extra data and deletes it, then deletes the extra data (the object, with
/// `RemoveExtra(extra, true)`) when the list is empty, whether or not an
/// entry was removed. The walk stops at the empty node. As in the exe, the
/// "previous node" the code keeps for the unlink is never set, so the entry
/// is always taken out with the remove-head call (`0063f7b0`) on the node it
/// was found at. A null `child` or no such extra data does nothing. No Xbox
/// PDB name.
pub fn fn_0041ef20(e: &mut Engine, this: Ptr<ExtraDataList>, child: u32) {
    if child == 0 {
        return;
    }
    let extra = find_extra(e, this, EXTRA_ACTIVATE_REF_CHILDREN_TYPE);
    if extra.is_null() {
        return;
    }
    let head = extra.addr() + 0x0c;
    let mut node = head;
    while node != 0 {
        if e.call(LIST_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let entry: Ptr<RefActivateData> = Ptr::new(e.mem.u32(slot));
        if e.get(entry, RefActivateData::pActivateRef).addr() == child {
            e.call(LIST_REMOVE_HEAD, &args![node]);
            e.call(OPERATOR_DELETE, &args![entry]);
            break;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if e.call(LIST_IS_EMPTY, &args![head]).bool() {
        remove_extra(e, this, extra);
    }
}

// Translated from 0041eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetActivateChildrenTimer` (Xbox PDB):
/// `fActivateChildrenTimer` (+0x14) of the type `0x54` extra data; does
/// nothing when the list has none. Stored through the x87 stack (`FLD`/`FSTP`,
/// so a signalling NaN is stored quiet).
pub fn extra_data_list_set_activate_children_timer(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    timer: f32,
) {
    let extra: Ptr<ExtraActivateRefChildren> =
        find_extra(e, this, EXTRA_ACTIVATE_REF_CHILDREN_TYPE).cast();
    if !extra.is_null() {
        e.set(
            extra,
            ExtraActivateRefChildren::fActivateChildrenTimer,
            x87_quiet(timer),
        );
    }
}

// Translated from 0041f020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetActivateChildrenTimer` (Xbox PDB):
/// `fActivateChildrenTimer` of the type `0x54` extra data, or -1.0 (the
/// float at `01012054`) when the list has none. Returned in `ST0` (`FLD`, so
/// a signalling NaN comes out quiet).
pub fn extra_data_list_get_activate_children_timer(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> f32 {
    let extra: Ptr<ExtraActivateRefChildren> =
        find_extra(e, this, EXTRA_ACTIVATE_REF_CHILDREN_TYPE).cast();
    let timer = if extra.is_null() {
        e.global::<f32>(NO_TIMER)
    } else {
        e.get(extra, ExtraActivateRefChildren::fActivateChildrenTimer)
    };
    x87_quiet(timer)
}

// Translated from 0041f050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetDecalRefs` (Xbox PDB): the `DecalRefList` (+0x0C, the
/// inline head node) of the type `0x57` extra data (`ExtraDecalRefs`), or
/// null.
pub fn extra_data_list_get_decal_refs(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    extra_list_or_null(e, this, EXTRA_DECAL_REFS_TYPE)
}

// Translated from 0041f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a decal for `reference` to the type `0x57` extra data
/// (`ExtraDecalRefs`, `0x14` bytes, `00433ca0`, built and added when there
/// is none): its method `00434480` is given the reference and the addresses
/// of the intersection point and the normal (`NiPoint3`), which it copies
/// into the entry. A null `reference` does nothing. No Xbox PDB name. The
/// exception-unwinding frame is not translated.
pub fn fn_0041f080(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: u32,
    intersect: Ptr<NiPoint3>,
    normal: Ptr<NiPoint3>,
) {
    if reference == 0 {
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_DECAL_REFS_TYPE);
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x14, DECAL_REFS_INIT));
        add_extra(e, this, extra);
    }
    e.call(DECAL_REFS_ADD, &args![extra, reference, intersect, normal]);
}

// Translated from 0041f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetReflectedRefs` (Xbox PDB): the `RefList` (+0x0C, the
/// inline head node) of the type `0x65` extra data (`ExtraReflectedRefs`),
/// or null.
pub fn extra_data_list_get_reflected_refs(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    extra_list_or_null(e, this, EXTRA_REFLECTED_REFS_TYPE)
}

// Translated from 0041f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `RefList` (+0x0C, the inline head node) of the type `0x66` extra data
/// (`ExtraReflectorRefs`), or null. No Xbox PDB name.
pub fn fn_0041f170(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSSimpleList> {
    extra_list_or_null(e, this, EXTRA_REFLECTOR_REFS_TYPE)
}

// Translated from 0041f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears flag bit 0 of the entry for `reference` in the
/// type `0x66` extra data (`ExtraReflectorRefs`, constructor `00411be0`);
/// see [`set_reflect_ref_flag`]. No Xbox PDB name. The exception-unwinding
/// frame is not translated.
pub fn fn_0041f1a0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, set: bool) {
    set_reflect_ref_flag(
        e,
        this,
        EXTRA_REFLECTOR_REFS_TYPE,
        REFLECTOR_REFS_INIT,
        reference,
        set,
        1,
    );
}

// Translated from 0041f330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears flag bit 1 (value 2) of the entry for
/// `reference` in the type `0x66` extra data (`ExtraReflectorRefs`,
/// constructor `00411be0`); see [`set_reflect_ref_flag`]. No Xbox PDB name.
/// The exception-unwinding frame is not translated.
pub fn fn_0041f330(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, set: bool) {
    set_reflect_ref_flag(
        e,
        this,
        EXTRA_REFLECTOR_REFS_TYPE,
        REFLECTOR_REFS_INIT,
        reference,
        set,
        2,
    );
}

// Translated from 0041f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears flag bit 0 of the entry for `reference` in the
/// type `0x65` extra data (`ExtraReflectedRefs`, constructor `00411b40`);
/// see [`set_reflect_ref_flag`]. No Xbox PDB name. The exception-unwinding
/// frame is not translated.
pub fn fn_0041f4c0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, set: bool) {
    set_reflect_ref_flag(
        e,
        this,
        EXTRA_REFLECTED_REFS_TYPE,
        REFLECTED_REFS_INIT,
        reference,
        set,
        1,
    );
}

// Translated from 0041f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears flag bit 1 (value 2) of the entry for
/// `reference` in the type `0x65` extra data (`ExtraReflectedRefs`,
/// constructor `00411b40`); see [`set_reflect_ref_flag`]. No Xbox PDB name.
/// The exception-unwinding frame is not translated.
pub fn fn_0041f650(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, set: bool) {
    set_reflect_ref_flag(
        e,
        this,
        EXTRA_REFLECTED_REFS_TYPE,
        REFLECTED_REFS_INIT,
        reference,
        set,
        2,
    );
}

// Translated from 0041f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWaterLightRefs` (Xbox PDB): the `RefList` (+0x0C, the
/// inline head node) of the type `0x84` extra data (`ExtraWaterLightRefs`),
/// or null.
pub fn extra_data_list_get_water_light_refs(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    extra_list_or_null(e, this, EXTRA_WATER_LIGHT_REFS_TYPE)
}

// Translated from 0041f810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetLitWaterRefs` (Xbox PDB): the `RefList` (+0x0C, the
/// inline head node) of the type `0x85` extra data (`ExtraLitWaterRefs`), or
/// null.
pub fn extra_data_list_get_lit_water_refs(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSSimpleList> {
    extra_list_or_null(e, this, EXTRA_LIT_WATER_REFS_TYPE)
}

// Translated from 0041f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetWaterLightRef` (Xbox PDB): adds (`add` true) or removes
/// `reference` in the `RefList` of the type `0x84` extra data
/// (`ExtraWaterLightRefs`, `0x14` bytes, `00411c80`, built when an add finds
/// none); see [`change_reference_in_list`]. A removal never deletes the
/// extra data. The exception-unwinding frame is not translated.
pub fn extra_data_list_set_water_light_ref(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: u32,
    add: bool,
) {
    change_reference_in_list(
        e,
        this,
        EXTRA_WATER_LIGHT_REFS_TYPE,
        WATER_LIGHT_REFS_INIT,
        reference,
        add,
        false,
    );
}

// Translated from 0041f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds (`add` true) or removes `reference` in the `RefList` of the type
/// `0x85` extra data (`ExtraLitWaterRefs`, `0x14` bytes, `00411d20`, built
/// when an add finds none); see [`change_reference_in_list`]. A removal
/// deletes the extra data when its list is then empty. No Xbox PDB name. The
/// exception-unwinding frame is not translated.
pub fn fn_0041f940(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, add: bool) {
    change_reference_in_list(
        e,
        this,
        EXTRA_LIT_WATER_REFS_TYPE,
        LIT_WATER_REFS_INIT,
        reference,
        add,
        true,
    );
}

// Translated from 0041fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddPrimitive` (Xbox PDB): adds a new `ExtraPrimitive`
/// (`0x10` bytes, constructor `0041faf0`) owning `primitive`; a null
/// `primitive` does nothing. The list is not checked for an existing one.
/// The exception-unwinding frame is not translated.
pub fn extra_data_list_add_primitive(e: &mut Engine, this: Ptr<ExtraDataList>, primitive: u32) {
    if primitive == 0 {
        return;
    }
    let built = new_built(e, 0x10, |e, block| {
        fn_0041faf0(e, Ptr::new(block), primitive).addr()
    });
    add_extra(e, this, Ptr::new(built));
}

// Translated from 0041faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraPrimitive` (type `0x6B`): the base constructor
/// (`0040ec80`) with the type, the vtable `010151b4`, and `pPrimitive` =
/// `primitive`. Returns `this`.
pub fn fn_0041faf0(
    e: &mut Engine,
    this: Ptr<ExtraPrimitive>,
    primitive: u32,
) -> Ptr<ExtraPrimitive> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_PRIMITIVE_TYPE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PRIMITIVE);
    e.set(this, ExtraPrimitive::pPrimitive, Ptr::new(primitive));
    this
}

// Translated from 0041fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPrimitive::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`0041fb50`) and frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_primitive_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraPrimitive>,
    flags: u32,
) -> Ptr<ExtraPrimitive> {
    fn_0041fb50(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 0041fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraPrimitive`: sets its vtable, deletes the primitive it
/// owns through its own scalar deleting destructor (slot 0, argument 1) when
/// there is one, then runs the `BSExtraData` destructor (`0040ecb0`). The
/// exception-unwinding frame is not translated.
pub fn fn_0041fb50(e: &mut Engine, this: Ptr<ExtraPrimitive>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PRIMITIVE);
    let primitive = e.get(this, ExtraPrimitive::pPrimitive);
    if !primitive.is_null() {
        e.vcall(primitive.addr(), 0, &args![1u32]);
    }
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0041fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPrimitive` (Xbox PDB): `pPrimitive` of the type `0x6B`
/// extra data (`ExtraPrimitive`), or 0.
pub fn extra_data_list_get_primitive(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_PRIMITIVE_TYPE)
}

// Translated from 0041fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the patrol reference data: the type `0x6F` extra data
/// (`ExtraPatrolRefData`) is deleted when there is one, then, for a non-null
/// `patrol_data`, a new one (`0x10` bytes, constructor `0041fcd0`) is given
/// it (`0041fd00`) and added. No Xbox PDB name. The exception-unwinding frame
/// is not translated.
pub fn fn_0041fc10(e: &mut Engine, this: Ptr<ExtraDataList>, patrol_data: u32) {
    replace_extra_holding_word(
        e,
        this,
        EXTRA_PATROL_REF_DATA_TYPE,
        patrol_data,
        |e, block| fn_0041fcd0(e, Ptr::new(block)).addr(),
    );
}

// Translated from 0041fcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraPatrolRefData` (type `0x6F`): the base constructor
/// (`0040ec80`) with the type and the vtable `010151c0`; `pPatrolData` is
/// left as it is (the setter `0041fd00` fills it). Neither the engine map nor
/// Ghidra's function list has this function (the map files it under no unit);
/// it is the callee of `0041fc10`. Returns `this`.
pub fn fn_0041fcd0(e: &mut Engine, this: Ptr<ExtraPatrolRefData>) -> Ptr<ExtraPatrolRefData> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_PATROL_REF_DATA_TYPE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PATROL_REF_DATA);
    this
}

// Translated from 0041fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefData::Compare` (Xbox PDB): true when the two differ. Two
/// extra data without patrol data are equal; one without differs; otherwise
/// `PatrolRefData::Compare` (`0067c720`) decides.
pub fn extra_patrol_ref_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraPatrolRefData>,
    other: Ptr<ExtraPatrolRefData>,
) -> bool {
    let mine = e.get(this, ExtraPatrolRefData::pPatrolData);
    let theirs = e.get(other, ExtraPatrolRefData::pPatrolData);
    if mine.is_null() && theirs.is_null() {
        return false;
    }
    if mine.is_null() || theirs.is_null() {
        return true;
    }
    e.call(PATROL_DATA_COMPARE, &args![mine, theirs]).bool()
}

// Translated from 0041fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPatrolRefData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`0041fdb0`) and frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_patrol_ref_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraPatrolRefData>,
    flags: u32,
) -> Ptr<ExtraPatrolRefData> {
    fn_0041fdb0(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 0041fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraPatrolRefData` (called by `0041fd80`; not in the
/// engine map nor in Ghidra's function list): sets its vtable, deletes the
/// patrol data (`0041fe10`), then runs the `BSExtraData` destructor
/// (`0040ecb0`). The exception-unwinding frame is not translated.
pub fn fn_0041fdb0(e: &mut Engine, this: Ptr<ExtraPatrolRefData>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PATROL_REF_DATA);
    fn_0041fe10(e, this);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0041fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the patrol data of an `ExtraPatrolRefData` through its scalar
/// deleting destructor (`0041fe60`, argument 1) when there is one, and sets
/// `pPatrolData` to null. No Xbox PDB name.
pub fn fn_0041fe10(e: &mut Engine, this: Ptr<ExtraPatrolRefData>) {
    let patrol_data = e.get(this, ExtraPatrolRefData::pPatrolData);
    if !patrol_data.is_null() {
        fn_0041fe60(e, patrol_data, 1);
    }
    e.set(this, ExtraPatrolRefData::pPatrolData, Ptr::NULL);
}

// Translated from 0041fe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `PatrolRefData` (the compiler emitted a
/// copy in this unit): runs the destructor body (`0067c670`) and frees the
/// object when bit 0 of `flags` is set. Returns `this`. No Xbox PDB name.
pub fn fn_0041fe60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(PATROL_DATA_DESTROY, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// Translated from 0041fe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pPatrolData` of the type `0x6F` extra data (`ExtraPatrolRefData`), or 0.
/// No Xbox PDB name.
pub fn fn_0041fe90(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_PATROL_REF_DATA_TYPE)
}

// Translated from 0041fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the occlusion plane reference data: the type `0x76` extra data
/// (`ExtraOcclusionPlaneRefData`) is deleted when there is one, then, for a
/// non-null `data`, a new one (`0x10` bytes, constructor `0041ff80`) is given
/// it (`0041fd00`) and added. No Xbox PDB name. The exception-unwinding frame
/// is not translated.
pub fn fn_0041fec0(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) {
    replace_extra_holding_word(
        e,
        this,
        EXTRA_OCCLUSION_PLANE_REF_DATA_TYPE,
        data,
        |e, block| fn_0041ff80(e, Ptr::new(block)).addr(),
    );
}

// Translated from 0041ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraOcclusionPlaneRefData` (type `0x76`): the base
/// constructor (`0040ec80`) with the type and the vtable `010151cc`; `pData`
/// is left as it is (the setter `0041fd00` fills it). Returns `this`.
pub fn fn_0041ff80(
    e: &mut Engine,
    this: Ptr<ExtraOcclusionPlaneRefData>,
) -> Ptr<ExtraOcclusionPlaneRefData> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_OCCLUSION_PLANE_REF_DATA_TYPE as u32],
    );
    e.mem
        .set_u32(this.addr(), VTABLE_EXTRA_OCCLUSION_PLANE_REF_DATA);
    this
}

// Translated from 0041ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOcclusionPlaneRefData::Compare` (Xbox PDB): true when the two
/// differ. Two extra data without data are equal; one without differs;
/// otherwise they differ when any of the four words of the data differs.
pub fn extra_occlusion_plane_ref_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraOcclusionPlaneRefData>,
    other: Ptr<ExtraOcclusionPlaneRefData>,
) -> bool {
    let mine = e.get(this, ExtraOcclusionPlaneRefData::pData);
    let theirs = e.get(other, ExtraOcclusionPlaneRefData::pData);
    if mine.is_null() && theirs.is_null() {
        return false;
    }
    if mine.is_null() || theirs.is_null() {
        return true;
    }
    (0..4).any(|index| e.mem.u32(mine.addr() + 4 * index) != e.mem.u32(theirs.addr() + 4 * index))
}

// Translated from 00420030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraOcclusionPlaneRefData::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor (`00420060`) and frees the object when bit 0 of
/// `flags` is set. Returns `this`.
pub fn extra_occlusion_plane_ref_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraOcclusionPlaneRefData>,
    flags: u32,
) -> Ptr<ExtraOcclusionPlaneRefData> {
    fn_00420060(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00420060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraOcclusionPlaneRefData` (called by `00420030`; not in
/// the engine map nor in Ghidra's function list): sets its vtable, frees the
/// data (`004200c0`), then runs the `BSExtraData` destructor (`0040ecb0`).
/// The exception-unwinding frame is not translated.
pub fn fn_00420060(e: &mut Engine, this: Ptr<ExtraOcclusionPlaneRefData>) {
    e.mem
        .set_u32(this.addr(), VTABLE_EXTRA_OCCLUSION_PLANE_REF_DATA);
    fn_004200c0(e, this);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 004200c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the data of an `ExtraOcclusionPlaneRefData` (plain `operator
/// delete`) when there is one, and sets `pData` to null. No Xbox PDB name.
pub fn fn_004200c0(e: &mut Engine, this: Ptr<ExtraOcclusionPlaneRefData>) {
    let data = e.get(this, ExtraOcclusionPlaneRefData::pData);
    if !data.is_null() {
        e.call(OPERATOR_DELETE, &args![data]);
    }
    e.set(this, ExtraOcclusionPlaneRefData::pData, Ptr::NULL);
}

// Translated from 00420100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetOcclusionPlaneLinkedRefData` (Xbox PDB): `pData` of the
/// type `0x76` extra data (`ExtraOcclusionPlaneRefData`), or 0.
pub fn extra_data_list_get_occlusion_plane_linked_ref_data(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> u32 {
    extra_word_or_zero(e, this, EXTRA_OCCLUSION_PLANE_REF_DATA_TYPE)
}

// Translated from 00420130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Word `index` of the occlusion plane reference data of the list
/// (`00420100`), or 0 when the list has none. The index is not checked
/// against the four words. No Xbox PDB name.
pub fn fn_00420130(e: &mut Engine, this: Ptr<ExtraDataList>, index: i32) -> u32 {
    let data = extra_data_list_get_occlusion_plane_linked_ref_data(e, this);
    if data == 0 {
        0
    } else {
        e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
    }
}

// Translated from 00420160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `reference` as word `index` of the occlusion plane reference data.
/// When the list has none and `reference` is not null, the data is built
/// first (`0x10` bytes, constructor `00411dc0`) and set (`0041fec0`); the
/// word is then stored if the list has data, whatever the value (a null
/// `reference` clears the word). The index is not checked against the four
/// words. No Xbox PDB name. The exception-unwinding frame is not translated.
pub fn fn_00420160(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32, index: i32) {
    if extra_data_list_get_occlusion_plane_linked_ref_data(e, this) == 0 && reference != 0 {
        let data = new_object(e, 0x10, OCCLUSION_PLANE_DATA_INIT);
        fn_0041fec0(e, this, data);
    }
    let data = extra_data_list_get_occlusion_plane_linked_ref_data(e, this);
    if data != 0 {
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), reference);
    }
}

// Translated from 00420210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the portal reference data: the type `0x77` extra data
/// (`ExtraPortalRefData`) is deleted when there is one, then, for a non-null
/// `data`, a new one (`0x10` bytes, constructor `004202d0`) is given it
/// (`0041fd00`) and added. No Xbox PDB name. The exception-unwinding frame is
/// not translated.
pub fn fn_00420210(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) {
    replace_extra_holding_word(e, this, EXTRA_PORTAL_REF_DATA_TYPE, data, |e, block| {
        fn_004202d0(e, Ptr::new(block)).addr()
    });
}

/// The value (+0x0C) of the `NiPointer` of the extra data of `extra_type`
/// (`ExtraPortal`, `ExtraRoom`), read through `00559450`, or 0 when the list
/// has none.
fn ni_pointer_extra_value(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        0
    } else {
        e.call(READ_WORD, &args![extra.addr() + 0x0c]).u32()
    }
}

/// The shape of `SetPortal` and `SetRoom`: a null `value` deletes the extra
/// data of `extra_type` (by type); otherwise the `NiPointer` at +0x0C of the
/// existing one is assigned (`0066b0d0`), or a new `0x10`-byte one is built
/// by `construct`, assigned and added. As in the code, a failed allocation
/// still assigns at `0 + 0x0C` and adds a null extra data.
fn set_ni_pointer_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    value: u32,
) {
    if value == 0 {
        remove_extra_by_type(e, list, extra_type);
        return;
    }
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        e.call(NI_POINTER_ASSIGN, &args![extra.addr() + 0x0c, value]);
    } else {
        let built = new_object(e, 0x10, construct);
        e.call(NI_POINTER_ASSIGN, &args![built.wrapping_add(0x0c), value]);
        add_extra(e, list, Ptr::new(built));
    }
}

/// The shape of the adders to a list of the room reference data
/// (`00420c10`, `00420ce0`): when the list has no room reference data and
/// `reference` is not null, a `0x14`-byte `RoomLinkedRefData` is built
/// (`00416b80`) and set (`00420440`); then `reference` (the list functions
/// take the address of the argument) is put at the head of the list at
/// `list_offset` of the data unless the list already holds it. With no data
/// (a null `reference` and none before) the code goes on with a null data
/// pointer; the model panics at the first read where the game would crash.
fn add_to_room_data_list(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    list_offset: u32,
    reference: u32,
) {
    if extra_data_list_get_room_linked_ref_data(e, list) == 0 && reference != 0 {
        let data = new_object(e, 0x14, ROOM_LINKED_DATA_INIT);
        fn_00420440(e, list, data);
    }
    let head = extra_data_list_get_room_linked_ref_data(e, list).wrapping_add(list_offset);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), reference);
        if !e.call(LIST_CONTAINS, &args![head, slot]).bool() {
            e.call(LIST_ADD_HEAD, &args![head, slot]);
        }
    });
}

// Translated from 004202d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraPortalRefData` (type `0x77`): the base constructor
/// (`0040ec80`) with the type and the vtable `010151d8`; `pData` is left as
/// it is (the setter `0041fd00` fills it). No Xbox PDB name. Returns `this`.
pub fn fn_004202d0(e: &mut Engine, this: Ptr<ExtraPortalRefData>) -> Ptr<ExtraPortalRefData> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_PORTAL_REF_DATA_TYPE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PORTAL_REF_DATA);
    this
}

// Translated from 00420300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPortalRefData::Compare` (Xbox PDB): true when the two differ. Two
/// extra data without data are equal; one without differs; otherwise they
/// differ when either of the two words of the data differs.
pub fn extra_portal_ref_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraPortalRefData>,
    other: Ptr<ExtraPortalRefData>,
) -> bool {
    let mine = e.get(this, ExtraPortalRefData::pData);
    let theirs = e.get(other, ExtraPortalRefData::pData);
    if mine.is_null() && theirs.is_null() {
        return false;
    }
    if mine.is_null() || theirs.is_null() {
        return true;
    }
    (0..2).any(|index| e.mem.u32(mine.addr() + 4 * index) != e.mem.u32(theirs.addr() + 4 * index))
}

// Translated from 00420380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraPortalRefData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`004203b0`) and frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_portal_ref_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraPortalRefData>,
    flags: u32,
) -> Ptr<ExtraPortalRefData> {
    fn_004203b0(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 004203b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraPortalRefData` (called by `00420380`; not in the
/// engine map nor in Ghidra's function list): sets its vtable, frees the
/// data with `004200c0` (the same body as for the occlusion plane data:
/// plain `operator delete`, then `pData` = null), then runs the
/// `BSExtraData` destructor (`0040ecb0`). The exception-unwinding frame is
/// not translated.
pub fn fn_004203b0(e: &mut Engine, this: Ptr<ExtraPortalRefData>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PORTAL_REF_DATA);
    fn_004200c0(e, this.cast());
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 00420410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pData` of the type `0x77` extra data (`ExtraPortalRefData`), or 0. No
/// Xbox PDB name.
pub fn fn_00420410(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_PORTAL_REF_DATA_TYPE)
}

// Translated from 00420440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the room reference data: the type `0x7B` extra data
/// (`ExtraRoomRefData`) is deleted when there is one, then, for a non-null
/// `room_data`, a new one (`0x10` bytes, constructor `00420500`) is given it
/// (`0041fd00`) and added. No Xbox PDB name. The exception-unwinding frame is
/// not translated.
pub fn fn_00420440(e: &mut Engine, this: Ptr<ExtraDataList>, room_data: u32) {
    replace_extra_holding_word(e, this, EXTRA_ROOM_REF_DATA_TYPE, room_data, |e, block| {
        fn_00420500(e, Ptr::new(block)).addr()
    });
}

// Translated from 00420500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraRoomRefData` (type `0x7B`): the base constructor
/// (`0040ec80`) with the type and the vtable `010151e4`; `pData` is left as
/// it is (the setter `0041fd00` fills it). No Xbox PDB name. Returns `this`.
pub fn fn_00420500(e: &mut Engine, this: Ptr<ExtraRoomRefData>) -> Ptr<ExtraRoomRefData> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_ROOM_REF_DATA_TYPE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_ROOM_REF_DATA);
    this
}

// Translated from 00420530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRoomRefData::Compare` (Xbox PDB): true when the two differ. Two
/// extra data without data are equal; one without differs; otherwise the
/// master flags (`cMaster`) must be equal and the two `RoomList`s must hold
/// the same items in the same order.
pub fn extra_room_ref_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraRoomRefData>,
    other: Ptr<ExtraRoomRefData>,
) -> bool {
    let mine = e.get(this, ExtraRoomRefData::pData);
    let theirs = e.get(other, ExtraRoomRefData::pData);
    if mine.is_null() && theirs.is_null() {
        return false;
    }
    if mine.is_null() || theirs.is_null() {
        return true;
    }
    let mine: Ptr<RoomLinkedRefData> = mine.cast();
    let theirs: Ptr<RoomLinkedRefData> = theirs.cast();
    if e.get(mine, RoomLinkedRefData::cMaster) != e.get(theirs, RoomLinkedRefData::cMaster) {
        return true;
    }
    let mut mine_node = mine.addr() + ROOM_DATA_ROOM_LIST;
    let mut theirs_node = theirs.addr() + ROOM_DATA_ROOM_LIST;
    while mine_node != 0 && theirs_node != 0 {
        let mine_slot = e.call(LIST_ITEM_SLOT, &args![mine_node]).u32();
        let theirs_slot = e.call(LIST_ITEM_SLOT, &args![theirs_node]).u32();
        if e.mem.u32(mine_slot) != e.mem.u32(theirs_slot) {
            return true;
        }
        mine_node = e.call(LIST_NEXT, &args![mine_node]).u32();
        theirs_node = e.call(LIST_NEXT, &args![theirs_node]).u32();
    }
    mine_node != 0 || theirs_node != 0
}

// Translated from 00420600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraRoomRefData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00420630`) and frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_room_ref_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRoomRefData>,
    flags: u32,
) -> Ptr<ExtraRoomRefData> {
    fn_00420630(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00420630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraRoomRefData` (called by `00420600`; not in the engine
/// map nor in Ghidra's function list): sets its vtable, deletes the data
/// (`00420690`), then runs the `BSExtraData` destructor (`0040ecb0`). The
/// exception-unwinding frame is not translated.
pub fn fn_00420630(e: &mut Engine, this: Ptr<ExtraRoomRefData>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_ROOM_REF_DATA);
    fn_00420690(e, this);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 00420690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the `RoomLinkedRefData` of an `ExtraRoomRefData` through its
/// scalar deleting destructor (`004206e0`, argument 1) when there is one,
/// and sets `pData` to null. No Xbox PDB name.
pub fn fn_00420690(e: &mut Engine, this: Ptr<ExtraRoomRefData>) {
    let data = e.get(this, ExtraRoomRefData::pData);
    if !data.is_null() {
        fn_004206e0(e, data.cast(), 1);
    }
    e.set(this, ExtraRoomRefData::pData, Ptr::NULL);
}

// Translated from 004206e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `RoomLinkedRefData` (the compiler emitted a
/// copy in this unit; Ghidra's library matcher names it
/// `_AsyncTaskCollection`, wrongly): runs the destructor (`00420710`) and
/// frees the object when bit 0 of `flags` is set. Returns `this`. No Xbox
/// PDB name.
pub fn fn_004206e0(e: &mut Engine, this: Ptr<RoomLinkedRefData>, flags: u32) -> Ptr {
    fn_00420710(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags)
}

// Translated from 00420710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `RoomLinkedRefData` (Ghidra's library matcher names it
/// `_AsyncTaskCollection::~_AsyncTaskCollection`, wrongly): destroys the
/// `RoomList` (+0x08), then the `PortalList` (+0x00), both with `0046ffb0`.
/// The exception-unwinding frame is not translated. No Xbox PDB name.
pub fn fn_00420710(e: &mut Engine, this: Ptr<RoomLinkedRefData>) {
    e.call(LIST_DESTROY, &args![this.addr() + ROOM_DATA_ROOM_LIST]);
    e.call(LIST_DESTROY, &args![this.addr() + ROOM_DATA_PORTAL_LIST]);
}

// Translated from 00420770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the `RoomList` (+0x08) of the room reference data of the
/// list (`004207e0`), or 0 when the list has none. No Xbox PDB name (Ghidra's
/// library matcher names it `UMSSchedulerProxy::GetUnblockNotifications`,
/// wrongly).
pub fn fn_00420770(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let data = extra_data_list_get_room_linked_ref_data(e, this);
    if data == 0 {
        0
    } else {
        data.wrapping_add(ROOM_DATA_ROOM_LIST)
    }
}

// Translated from 004207b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetRoomLinkedRefData` (`004207e0`) with the result masked by itself
/// (`-(x != 0) & x`, which is `x`). No Xbox PDB name.
pub fn fn_004207b0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_data_list_get_room_linked_ref_data(e, this)
}

// Translated from 004207e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRoomLinkedRefData` (Xbox PDB): `pData` of the type
/// `0x7B` extra data (`ExtraRoomRefData`), or 0.
pub fn extra_data_list_get_room_linked_ref_data(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_ROOM_REF_DATA_TYPE)
}

// Translated from 00420810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRoomIsMaster` (Xbox PDB): true when the list has no
/// room reference data, or its `cMaster` flag is set, or its `RoomList` is
/// empty (`008256d0`).
pub fn extra_data_list_get_room_is_master(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    let extra = find_extra(e, this, EXTRA_ROOM_REF_DATA_TYPE);
    if extra.is_null() {
        return true;
    }
    let data: Ptr<RoomLinkedRefData> = Ptr::new(e.mem.u32(extra.addr() + 0x0c));
    if e.get(data, RoomLinkedRefData::cMaster) != 0 {
        return true;
    }
    e.call(LIST_IS_EMPTY, &args![data.addr() + ROOM_DATA_ROOM_LIST])
        .bool()
}

// Translated from 00420870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetRoomIsMaster` (Xbox PDB): stores `master` (as 0 or 1)
/// in `cMaster` of the room reference data; nothing when the list has no
/// room reference data. The data itself is not checked.
pub fn extra_data_list_set_room_is_master(e: &mut Engine, this: Ptr<ExtraDataList>, master: bool) {
    let extra = find_extra(e, this, EXTRA_ROOM_REF_DATA_TYPE);
    if !extra.is_null() {
        let data: Ptr<RoomLinkedRefData> = Ptr::new(e.mem.u32(extra.addr() + 0x0c));
        e.set(data, RoomLinkedRefData::cMaster, master as u8);
    }
}

// Translated from 004208b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetMasterRoom` (Xbox PDB): the reference of the master
/// room this room leads to, or 0. 0 when the list has no room reference
/// data or is itself a master room (`GetRoomIsMaster`); otherwise the search
/// `00420950` runs with a fresh visited set (a 0x10-byte
/// `NiTMap<TESObjectREFR *, bool>` local, hash size `0x25`: constructor
/// `0042f4e0`, destructor `0042fe80`). The exception-unwinding frame is not
/// translated.
pub fn extra_data_list_get_master_room(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_ROOM_REF_DATA_TYPE);
    if extra.is_null() {
        return 0;
    }
    if extra_data_list_get_room_is_master(e, this) {
        return 0;
    }
    e.with_stack(0x10, |e, visited| {
        e.call(VISITED_SET_INIT, &args![visited, 0x25u32]);
        let master = fn_00420950(e, this, visited);
        e.call(VISITED_SET_DESTROY, &args![visited]);
        master
    })
}

// Translated from 00420950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The search of `GetMasterRoom` (no Xbox PDB name; the visited set is the
/// stack argument): 0 when the list has no room reference data. First the
/// first reference of the `RoomList` (stopping at a null one) whose own
/// extra data list answers `GetRoomIsMaster` is the result. Then each
/// reference of the `RoomList` (same stop) not yet in the visited set
/// (`0057c850`) is recorded (`0084d310`) and searched in its own extra data
/// list (`005d43c0`), recursively; the first non-null answer is the result.
/// The flag the lookup writes only when it finds the key is a local the code
/// never initialises; the model gives 0.
pub fn fn_00420950(e: &mut Engine, this: Ptr<ExtraDataList>, visited: Ptr) -> u32 {
    let extra = find_extra(e, this, EXTRA_ROOM_REF_DATA_TYPE);
    if extra.is_null() {
        return 0;
    }
    let mut node = e.mem.u32(extra.addr() + 0x0c) + ROOM_DATA_ROOM_LIST;
    while node != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let room = e.mem.u32(slot);
        let room_list = e.call(REFERENCE_EXTRA_LIST, &args![room]).ptr();
        if extra_data_list_get_room_is_master(e, room_list) {
            return room;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let mut node = e.mem.u32(extra.addr() + 0x0c) + ROOM_DATA_ROOM_LIST;
    e.with_stack(4, |e, seen| {
        while node != 0 {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let room = e.mem.u32(slot);
            if !e.call(VISITED_SET_FIND, &args![visited, room, seen]).bool() {
                let seen_value = e.mem.u8(seen.addr()) as u32;
                e.call(VISITED_SET_INSERT, &args![visited, room, seen_value]);
                let room_list = e.call(REFERENCE_EXTRA_LIST, &args![room]).ptr();
                let found = fn_00420950(e, room_list, visited);
                if found != 0 {
                    return found;
                }
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        0
    })
}

// Translated from 00420a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `reference` as word `index` of the portal reference data
/// (`ExtraPortalRefData`, built first when the list has none and
/// `reference` is not null: 8 bytes, constructor `004143c0`, set with
/// `00420210`; the word is stored whatever the value, a null one clears it,
/// when the list has data) and passes the room of the reference to the
/// portal (`ExtraPortal`, `00420dd0`): the room of the master room of
/// `reference` (`GetMasterRoom` of its extra data list, `005d43c0`), or of
/// `reference` itself when it has none, or 0. Index 0 goes to the portal's
/// first room (`009707f0`), index 1 to its second (`00420ba0`); another
/// index only stores the word. No Xbox PDB name. The exception-unwinding
/// frame is not translated.
pub fn fn_00420a60(e: &mut Engine, this: Ptr<ExtraDataList>, index: i32, reference: u32) {
    if fn_00420410(e, this) == 0 && reference != 0 {
        let data = new_object(e, 8, PORTAL_LINKED_DATA_INIT);
        fn_00420210(e, this, data);
    }
    let data = fn_00420410(e, this);
    if data != 0 {
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), reference);
    }
    let portal = fn_00420dd0(e, this);
    let master = if reference == 0 {
        0
    } else {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).ptr();
        extra_data_list_get_master_room(e, list)
    };
    let room_reference = if master == 0 { reference } else { master };
    let room = if room_reference == 0 {
        0
    } else {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![room_reference]).ptr();
        extra_data_list_get_room(e, list)
    };
    if portal != 0 {
        if index == 0 {
            e.call(PORTAL_SET_FIRST_ROOM, &args![portal, room]);
        }
        if index == 1 {
            fn_00420ba0(e, Ptr::new(portal), room);
        }
    }
}

// Translated from 00420ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `room` at `this + 0x100` of a portal (the second room; the first
/// is at `+0xFC`, `009707f0`; the Xbox PDB's `BSPortal::pMultiBoundRoom`
/// array sits elsewhere in that build). No Xbox PDB name.
pub fn fn_00420ba0(e: &mut Engine, this: Ptr, room: u32) {
    e.mem.set_u32(this.addr() + 0x100, room);
}

// Translated from 00420bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Word `index` of the portal reference data of the list, or 0 when the list
/// has no type `0x77` extra data or it has no data. The index is not checked
/// against the two words. No Xbox PDB name.
pub fn fn_00420bc0(e: &mut Engine, this: Ptr<ExtraDataList>, index: i32) -> u32 {
    let data = fn_00420410(e, this);
    if data == 0 {
        0
    } else {
        e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
    }
}

// Translated from 00420c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `reference` to the `RoomList` of the room reference data (+0x08
/// of the data), building the data first when the list has none. No Xbox
/// PDB name. The exception-unwinding frame is not translated.
pub fn fn_00420c10(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    add_to_room_data_list(e, this, ROOM_DATA_ROOM_LIST, reference);
}

// Translated from 00420ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `reference` to the `PortalList` of the room reference data (+0x00 of
/// the data), building the data first when the list has none. No Xbox PDB
/// name. The exception-unwinding frame is not translated.
pub fn fn_00420ce0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    add_to_room_data_list(e, this, ROOM_DATA_PORTAL_LIST, reference);
}

// Translated from 00420da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `reference` from the `PortalList` (+0x00 of the data) of the room
/// reference data (`00905330`, given the address of the argument); nothing
/// when the list has none. No Xbox PDB name.
pub fn fn_00420da0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    let data = extra_data_list_get_room_linked_ref_data(e, this);
    if data != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), reference);
            e.call(LIST_REMOVE_ITEM, &args![data, slot]);
        });
    }
}

// Translated from 00420dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The portal (`NiPointer<BSPortal>` at +0x0C, read through `00559450`) of
/// the type `0x78` extra data (`ExtraPortal`), or 0; the getter that goes
/// with `SetPortal`. No Xbox PDB name.
pub fn fn_00420dd0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    ni_pointer_extra_value(e, this, EXTRA_PORTAL_TYPE)
}

// Translated from 00420e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPortal` (Xbox PDB): a null `portal` deletes the type
/// `0x78` extra data (`ExtraPortal`) by type; otherwise the `NiPointer` of the
/// existing one is assigned (`0066b0d0`), or a new `0x10`-byte one
/// (constructor `00435820`) is assigned and added. The exception-unwinding
/// frame is not translated.
pub fn extra_data_list_set_portal(e: &mut Engine, this: Ptr<ExtraDataList>, portal: u32) {
    set_ni_pointer_extra(e, this, EXTRA_PORTAL_TYPE, EXTRA_PORTAL_INIT, portal);
}

// Translated from 00420ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRoom` (Xbox PDB): the room (`NiPointer<BSMultiBoundRoom>`
/// at +0x0C, read through `00559450`) of the type `0x79` extra data
/// (`ExtraRoom`), or 0.
pub fn extra_data_list_get_room(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    ni_pointer_extra_value(e, this, EXTRA_ROOM_TYPE)
}

// Translated from 00420f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetRoom` (Xbox PDB): like `SetPortal` for the type `0x79`
/// extra data (`ExtraRoom`, constructor `004358a0`). The exception-unwinding
/// frame is not translated.
pub fn extra_data_list_set_room(e: &mut Engine, this: Ptr<ExtraDataList>, room: u32) {
    set_ni_pointer_extra(e, this, EXTRA_ROOM_TYPE, EXTRA_ROOM_INIT, room);
}

// Translated from 00420fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the collision data: the type `0x72` extra data
/// (`ExtraCollisionData`) is deleted when there is one, then, for a non-null
/// `collision_data`, a new one (`0x10` bytes, constructor `00421090`) is
/// given it (`0041fd00`) and added. No Xbox PDB name. The
/// exception-unwinding frame is not translated.
pub fn fn_00420fd0(e: &mut Engine, this: Ptr<ExtraDataList>, collision_data: u32) {
    replace_extra_holding_word(
        e,
        this,
        EXTRA_COLLISION_DATA_TYPE,
        collision_data,
        |e, block| fn_00421090(e, Ptr::new(block)).addr(),
    );
}

// Translated from 00421090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraCollisionData` (type `0x72`): the base constructor
/// (`0040ec80`) with the type and the vtable `010151f0`; `pCollisionData` is
/// left as it is (the setter `0041fd00` fills it). No Xbox PDB name. Returns
/// `this`.
pub fn fn_00421090(e: &mut Engine, this: Ptr<ExtraCollisionData>) -> Ptr<ExtraCollisionData> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_COLLISION_DATA_TYPE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_COLLISION_DATA);
    this
}

// Translated from 004210c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCollisionData::Compare` (Xbox PDB): true when the two differ. Two
/// extra data without collision data are equal; one without differs;
/// otherwise the word each collision data starts with (read through
/// `00559450`, `this` first) is compared.
pub fn extra_collision_data_compare(
    e: &mut Engine,
    this: Ptr<ExtraCollisionData>,
    other: Ptr<ExtraCollisionData>,
) -> bool {
    let mine = e.get(this, ExtraCollisionData::pCollisionData);
    let theirs = e.get(other, ExtraCollisionData::pCollisionData);
    if mine.is_null() && theirs.is_null() {
        return false;
    }
    if mine.is_null() || theirs.is_null() {
        return true;
    }
    let mine_word = e.call(READ_WORD, &args![mine]).u32();
    let theirs_word = e.call(READ_WORD, &args![theirs]).u32();
    mine_word != theirs_word
}

// Translated from 00421130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCollisionData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00421160`) and frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn extra_collision_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraCollisionData>,
    flags: u32,
) -> Ptr<ExtraCollisionData> {
    fn_00421160(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00421160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraCollisionData`: sets its vtable, frees the collision
/// data with `operator delete` (null or not), then runs the `BSExtraData`
/// destructor (`0040ecb0`). No Xbox PDB name.
pub fn fn_00421160(e: &mut Engine, this: Ptr<ExtraCollisionData>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_COLLISION_DATA);
    let collision_data = e.get(this, ExtraCollisionData::pCollisionData);
    e.call(OPERATOR_DELETE, &args![collision_data]);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 004211a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetCollisionData` (Xbox PDB): `pCollisionData` of the type
/// `0x72` extra data (`ExtraCollisionData`), or 0.
pub fn extra_data_list_get_collision_data(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_COLLISION_DATA_TYPE)
}

// Translated from 004211d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetActorPackageData` (Xbox PDB): stores `data` in
/// `pActorPackageData` of the type `0x70` extra data (`ExtraPackageData`), or
/// adds a new one (`0x10` bytes, constructor `00421280`) holding it. A null
/// `data` is stored like any other. The exception-unwinding frame is not
/// translated.
pub fn extra_data_list_set_actor_package_data(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) {
    let extra = find_extra(e, this, EXTRA_PACKAGE_DATA_TYPE);
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x0c, data);
    } else {
        let built = new_built(e, 0x10, |e, block| {
            fn_00421280(e, Ptr::new(block), data).addr()
        });
        add_extra(e, this, Ptr::new(built));
    }
}

// Translated from 00421280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraPackageData` (type `0x70`): the base constructor
/// (`0040ec80`) with the type, the vtable `010151fc`, and `pActorPackageData`
/// = `data`. No Xbox PDB name. Returns `this`.
pub fn fn_00421280(
    e: &mut Engine,
    this: Ptr<ExtraPackageData>,
    data: u32,
) -> Ptr<ExtraPackageData> {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_PACKAGE_DATA_TYPE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_PACKAGE_DATA);
    e.set(this, ExtraPackageData::pActorPackageData, Ptr::new(data));
    this
}

// Translated from 004212b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetActorPackageData` (Xbox PDB): `pActorPackageData` of
/// the type `0x70` extra data (`ExtraPackageData`), or 0.
pub fn extra_data_list_get_actor_package_data(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or_zero(e, this, EXTRA_PACKAGE_DATA_TYPE)
}

// Translated from 004212e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveActorPackageData` (Xbox PDB): deletes the type
/// `0x70` extra data (`ExtraPackageData`) when the list has one.
pub fn extra_data_list_remove_actor_package_data(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_PACKAGE_DATA_TYPE);
    if !extra.is_null() {
        remove_extra(e, this, extra);
    }
}

// Translated from 00421310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddGuard` (Xbox PDB): finds the type `0x7C` extra data
/// (`ExtraGuardedRefData`) or builds one (`0x1c` bytes, constructor
/// `00430f30`) and adds it, then gives it `guard`
/// (`ExtraGuardedRefData::AddGuard`, `004310d0`). The exception-unwinding
/// frame is not translated.
pub fn extra_data_list_add_guard(e: &mut Engine, this: Ptr<ExtraDataList>, guard: u32) {
    let mut extra = find_extra(e, this, EXTRA_GUARDED_REF_DATA_TYPE);
    if extra.is_null() {
        extra = Ptr::new(new_object(e, 0x1c, EXTRA_GUARDED_REF_DATA_INIT));
        add_extra(e, this, extra);
    }
    e.call(GUARDED_REF_DATA_ADD_GUARD, &args![extra, guard]);
}

// Translated from 004213c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives its two words to the method `00431120` of the type `0x7C` extra data
/// (`ExtraGuardedRefData`) when the list has one. No Xbox PDB name.
pub fn fn_004213c0(e: &mut Engine, this: Ptr<ExtraDataList>, first: u32, second: u32) {
    let extra = find_extra(e, this, EXTRA_GUARDED_REF_DATA_TYPE);
    if !extra.is_null() {
        e.call(GUARDED_REF_DATA_TWO_WORDS, &args![extra, first, second]);
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0041db00, fn_0041db00(Ptr<ExtraDataList>) -> bool),
        entry!(0x0041db30, fn_0041db30(Ptr<ExtraEnableStateParent>) -> bool),
        entry!(0x0041db50, fn_0041db50(Ptr<ExtraDataList>, bool)),
        entry!(0x0041db80, fn_0041db80(Ptr<ExtraEnableStateParent>, bool)),
        entry!(
            0x0041dbd0,
            extra_data_list_should_pop_in_when_enabled_by_parent(Ptr<ExtraDataList>) -> bool
        ),
        entry!(0x0041dc00, fn_0041dc00(Ptr<ExtraEnableStateParent>) -> bool),
        entry!(0x0041dc20, fn_0041dc20(Ptr<ExtraDataList>) -> bool),
        entry!(0x0041dc50, fn_0041dc50(Ptr<ExtraEnableStateParent>) -> bool),
        entry!(0x0041dc70, fn_0041dc70(Ptr<ExtraDataList>, u8)),
        entry!(
            0x0041dca0,
            fn_0041dca0(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(0x0041dcd0, fn_0041dcd0(Ptr<ExtraDataList>, u32)),
        entry!(0x0041dda0, fn_0041dda0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041de00,
            extra_data_list_get_item_dropper(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x0041de40,
            extra_data_list_add_dropped_item(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0041df00, fn_0041df00(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041df90,
            extra_data_list_get_dropped_item_list(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(
            0x0041dfd0,
            extra_data_list_remove_dropped_item_list(Ptr<ExtraDataList>)
        ),
        entry!(0x0041e000, fn_0041e000(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041e0d0,
            extra_data_list_remove_dropped_item(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041e130,
            extra_data_list_get_water_type(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041e160, fn_0041e160(Ptr<ExtraDataList>, u32)),
        entry!(0x0041e220, fn_0041e220(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041e250, fn_0041e250(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041e310,
            extra_data_list_get_ash_pile_ref(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041e340, fn_0041e340(Ptr<ExtraDataList>, u32)),
        entry!(0x0041e410, fn_0041e410(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041e440, fn_0041e440(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041e500,
            extra_data_list_get_linked_ref_children(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(0x0041e530, fn_0041e530(Ptr<ExtraDataList>, u32)),
        entry!(0x0041e600, fn_0041e600(Ptr<ExtraDataList>, u32)),
        entry!(0x0041e660, fn_0041e660(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041e690,
            extra_data_list_set_open_close_activate_ref(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041e750,
            fn_0041e750(Ptr<ExtraOpenCloseActivateRef>) -> Ptr<ExtraOpenCloseActivateRef>
        ),
        entry!(
            0x0041e780,
            fn_0041e780(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(0x0041e7b0, fn_0041e7b0(Ptr<ExtraDataList>, u32) -> u32),
        entry!(0x0041e7f0, fn_0041e7f0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041e960,
            extra_data_list_remove_activate_parent(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0041e9e0, fn_0041e9e0(Ptr<ExtraDataList>, u32) -> f32),
        entry!(0x0041ea30, fn_0041ea30(Ptr<ExtraDataList>, u32, f32)),
        entry!(0x0041eb60, fn_0041eb60(Ptr<ExtraDataList>) -> bool),
        entry!(0x0041eba0, fn_0041eba0(Ptr<ExtraDataList>, bool)),
        entry!(
            0x0041ec80,
            fn_0041ec80(Ptr<ExtraDataList>, Ptr<BSStringT>) -> Ptr<BSStringT>
        ),
        entry!(0x0041ece0, fn_0041ece0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041eda0,
            extra_data_list_get_activate_ref_children(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(
            0x0041edd0,
            extra_data_list_add_activate_ref_child(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0041ef20, fn_0041ef20(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041eff0,
            extra_data_list_set_activate_children_timer(Ptr<ExtraDataList>, f32)
        ),
        entry!(
            0x0041f020,
            extra_data_list_get_activate_children_timer(Ptr<ExtraDataList>) -> f32
        ),
        entry!(
            0x0041f050,
            extra_data_list_get_decal_refs(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(
            0x0041f080,
            fn_0041f080(Ptr<ExtraDataList>, u32, Ptr<NiPoint3>, Ptr<NiPoint3>)
        ),
        entry!(
            0x0041f140,
            extra_data_list_get_reflected_refs(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(
            0x0041f170,
            fn_0041f170(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(0x0041f1a0, fn_0041f1a0(Ptr<ExtraDataList>, u32, bool)),
        entry!(0x0041f330, fn_0041f330(Ptr<ExtraDataList>, u32, bool)),
        entry!(0x0041f4c0, fn_0041f4c0(Ptr<ExtraDataList>, u32, bool)),
        entry!(0x0041f650, fn_0041f650(Ptr<ExtraDataList>, u32, bool)),
        entry!(
            0x0041f7e0,
            extra_data_list_get_water_light_refs(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(
            0x0041f810,
            extra_data_list_get_lit_water_refs(Ptr<ExtraDataList>) -> Ptr<BSSimpleList>
        ),
        entry!(
            0x0041f840,
            extra_data_list_set_water_light_ref(Ptr<ExtraDataList>, u32, bool)
        ),
        entry!(0x0041f940, fn_0041f940(Ptr<ExtraDataList>, u32, bool)),
        entry!(
            0x0041fa60,
            extra_data_list_add_primitive(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041faf0,
            fn_0041faf0(Ptr<ExtraPrimitive>, u32) -> Ptr<ExtraPrimitive>
        ),
        entry!(
            0x0041fb20,
            extra_primitive_scalar_deleting_destructor(
                Ptr<ExtraPrimitive>,
                u32,
            ) -> Ptr<ExtraPrimitive>
        ),
        entry!(0x0041fb50, fn_0041fb50(Ptr<ExtraPrimitive>)),
        entry!(
            0x0041fbe0,
            extra_data_list_get_primitive(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041fc10, fn_0041fc10(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041fcd0,
            fn_0041fcd0(Ptr<ExtraPatrolRefData>) -> Ptr<ExtraPatrolRefData>
        ),
        entry!(
            0x0041fd20,
            extra_patrol_ref_data_compare(Ptr<ExtraPatrolRefData>, Ptr<ExtraPatrolRefData>) -> bool
        ),
        entry!(
            0x0041fd80,
            extra_patrol_ref_data_scalar_deleting_destructor(
                Ptr<ExtraPatrolRefData>,
                u32,
            )
                -> Ptr<ExtraPatrolRefData>
        ),
        entry!(0x0041fdb0, fn_0041fdb0(Ptr<ExtraPatrolRefData>)),
        entry!(0x0041fe10, fn_0041fe10(Ptr<ExtraPatrolRefData>)),
        entry!(0x0041fe60, fn_0041fe60(Ptr, u32) -> Ptr),
        entry!(0x0041fe90, fn_0041fe90(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041fec0, fn_0041fec0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041ff80,
            fn_0041ff80(Ptr<ExtraOcclusionPlaneRefData>) -> Ptr<ExtraOcclusionPlaneRefData>
        ),
        entry!(
            0x0041ffb0,
            extra_occlusion_plane_ref_data_compare(
                Ptr<ExtraOcclusionPlaneRefData>,
                Ptr<ExtraOcclusionPlaneRefData>,
            ) -> bool
        ),
        entry!(
            0x00420030,
            extra_occlusion_plane_ref_data_scalar_deleting_destructor(
                Ptr<ExtraOcclusionPlaneRefData>,
                u32,
            ) -> Ptr<
                ExtraOcclusionPlaneRefData,
            >
        ),
        entry!(0x00420060, fn_00420060(Ptr<ExtraOcclusionPlaneRefData>)),
        entry!(0x004200c0, fn_004200c0(Ptr<ExtraOcclusionPlaneRefData>)),
        entry!(
            0x00420100,
            extra_data_list_get_occlusion_plane_linked_ref_data(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00420130, fn_00420130(Ptr<ExtraDataList>, i32) -> u32),
        entry!(0x00420160, fn_00420160(Ptr<ExtraDataList>, u32, i32)),
        entry!(0x00420210, fn_00420210(Ptr<ExtraDataList>, u32)),
        entry!(
            0x004202d0,
            fn_004202d0(Ptr<ExtraPortalRefData>) -> Ptr<ExtraPortalRefData>
        ),
        entry!(
            0x00420300,
            extra_portal_ref_data_compare(Ptr<ExtraPortalRefData>, Ptr<ExtraPortalRefData>) -> bool
        ),
        entry!(
            0x00420380,
            extra_portal_ref_data_scalar_deleting_destructor(
                Ptr<ExtraPortalRefData>,
                u32,
            )
                -> Ptr<ExtraPortalRefData>
        ),
        entry!(0x004203b0, fn_004203b0(Ptr<ExtraPortalRefData>)),
        entry!(0x00420410, fn_00420410(Ptr<ExtraDataList>) -> u32),
        entry!(0x00420440, fn_00420440(Ptr<ExtraDataList>, u32)),
        entry!(
            0x00420500,
            fn_00420500(Ptr<ExtraRoomRefData>) -> Ptr<ExtraRoomRefData>
        ),
        entry!(
            0x00420530,
            extra_room_ref_data_compare(Ptr<ExtraRoomRefData>, Ptr<ExtraRoomRefData>) -> bool
        ),
        entry!(
            0x00420600,
            extra_room_ref_data_scalar_deleting_destructor(
                Ptr<ExtraRoomRefData>,
                u32,
            ) -> Ptr<ExtraRoomRefData>
        ),
        entry!(0x00420630, fn_00420630(Ptr<ExtraRoomRefData>)),
        entry!(0x00420690, fn_00420690(Ptr<ExtraRoomRefData>)),
        entry!(0x004206e0, fn_004206e0(Ptr<RoomLinkedRefData>, u32) -> Ptr),
        entry!(0x00420710, fn_00420710(Ptr<RoomLinkedRefData>)),
        entry!(0x00420770, fn_00420770(Ptr<ExtraDataList>) -> u32),
        entry!(0x004207b0, fn_004207b0(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x004207e0,
            extra_data_list_get_room_linked_ref_data(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00420810,
            extra_data_list_get_room_is_master(Ptr<ExtraDataList>) -> bool
        ),
        entry!(
            0x00420870,
            extra_data_list_set_room_is_master(Ptr<ExtraDataList>, bool)
        ),
        entry!(
            0x004208b0,
            extra_data_list_get_master_room(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00420950, fn_00420950(Ptr<ExtraDataList>, Ptr) -> u32),
        entry!(0x00420a60, fn_00420a60(Ptr<ExtraDataList>, i32, u32)),
        entry!(0x00420ba0, fn_00420ba0(Ptr, u32)),
        entry!(0x00420bc0, fn_00420bc0(Ptr<ExtraDataList>, i32) -> u32),
        entry!(0x00420c10, fn_00420c10(Ptr<ExtraDataList>, u32)),
        entry!(0x00420ce0, fn_00420ce0(Ptr<ExtraDataList>, u32)),
        entry!(0x00420da0, fn_00420da0(Ptr<ExtraDataList>, u32)),
        entry!(0x00420dd0, fn_00420dd0(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00420e00,
            extra_data_list_set_portal(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x00420ed0,
            extra_data_list_get_room(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00420f00,
            extra_data_list_set_room(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00420fd0, fn_00420fd0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x00421090,
            fn_00421090(Ptr<ExtraCollisionData>) -> Ptr<ExtraCollisionData>
        ),
        entry!(
            0x004210c0,
            extra_collision_data_compare(Ptr<ExtraCollisionData>, Ptr<ExtraCollisionData>) -> bool
        ),
        entry!(
            0x00421130,
            extra_collision_data_scalar_deleting_destructor(
                Ptr<ExtraCollisionData>,
                u32,
            ) -> Ptr<ExtraCollisionData>
        ),
        entry!(0x00421160, fn_00421160(Ptr<ExtraCollisionData>)),
        entry!(
            0x004211a0,
            extra_data_list_get_collision_data(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x004211d0,
            extra_data_list_set_actor_package_data(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x00421280,
            fn_00421280(Ptr<ExtraPackageData>, u32) -> Ptr<ExtraPackageData>
        ),
        entry!(
            0x004212b0,
            extra_data_list_get_actor_package_data(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x004212e0,
            extra_data_list_remove_actor_package_data(Ptr<ExtraDataList>)
        ),
        entry!(
            0x00421310,
            extra_data_list_add_guard(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x004213c0, fn_004213c0(Ptr<ExtraDataList>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// A fake list: the extra data of type `t` sits in the word at
    /// `list + 0x100 + 4 * t` (the doubles of `GetExtraData`, `AddExtra` and
    /// the two `RemoveExtra` read and write it).
    fn table_slot(list: u32, extra_type: u32) -> u32 {
        list + 0x100 + 4 * extra_type
    }

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Makes the constructor at `address` a double that sets the type byte
    /// of the new block and returns it.
    fn constructor_double(e: &mut Engine, address: u32, extra_type: u8) {
        e.register_double(address, move |e, a| {
            e.mem.set_u8(a[0] + 4, extra_type);
            returns(a[0])
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_EXTRA_DATA, |e, a| {
            returns(e.mem.u32(table_slot(a[0], a[1])))
        });
        e.register(ADD_EXTRA, |e, a| {
            let extra_type = e.mem.u8(a[1] + 4) as u32;
            e.mem.set_u32(table_slot(a[0], extra_type), a[1]);
            returns(a[1])
        });
        e.register(REMOVE_EXTRA, |e, a| {
            let extra_type = e.mem.u8(a[1] + 4) as u32;
            e.mem.set_u32(table_slot(a[0], extra_type), 0);
            Ret::default()
        });
        e.register(REMOVE_EXTRA_BY_TYPE, |e, a| {
            e.mem.set_u32(table_slot(a[0], a[1]), 0);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(BS_EXTRA_DATA_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        // The list doubles work on the inline head node of an extra data;
        // an add puts the item in the head node, which is all the tests need.
        e.register(LIST_CONTAINS, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut node = a[0];
            let mut found = false;
            while node != 0 {
                found |= e.mem.u32(node) == wanted;
                node = e.mem.u32(node + 4);
            }
            returns(found as u32)
        });
        e.register(LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(LIST_REMOVE_ITEM, |e, a| {
            let wanted = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == wanted {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_ITEM_SLOT, |_, a| returns(a[0]));
        e.register(LIST_NEXT, |e, a| returns(e.mem.u32(a[0] + 4)));
        constructor_double(&mut e, ENABLE_STATE_CHILDREN_INIT, 0x38);
        constructor_double(&mut e, ITEM_DROPPER_INIT, 0x39);
        constructor_double(&mut e, DROPPED_ITEM_LIST_INIT, 0x3a);
        constructor_double(&mut e, WATER_TYPE_INIT, 0x03);
        constructor_double(&mut e, TELEPORT_MARKER_INIT, 0x3b);
        constructor_double(&mut e, ASH_PILE_REF_INIT, 0x89);
        constructor_double(&mut e, LINKED_REF_INIT, 0x51);
        constructor_double(&mut e, LINKED_REF_CHILDREN_INIT, 0x52);
        constructor_double(&mut e, ACTIVATE_REF_INIT, 0x53);
        e
    }

    fn new_list(e: &mut Engine) -> Ptr<ExtraDataList> {
        Ptr::new(e.mem.alloc(0x400))
    }

    /// An extra data of the given type, put in the list's table.
    fn put_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
        let extra = e.mem.alloc(0x40);
        e.mem.set_u8(extra + 4, extra_type);
        e.mem
            .set_u32(table_slot(list.addr(), extra_type as u32), extra);
        extra
    }

    fn extra_of(e: &Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
        e.mem.u32(table_slot(list.addr(), extra_type as u32))
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    // 0041db00, 0041db30, 0041db50, 0041db80, 0041dbd0, 0041dc00, 0041dc20,
    // 0041dc50, 0041dc70: the flags of the enable state parent.

    #[test]
    fn flag_bit_zero_getter_reads_the_flags_or_gives_false() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(!e.call(0x0041db00, &args![list]).bool());
        let extra = put_extra(&mut e, list, 0x37);
        assert!(!e.call(0x0041db00, &args![list]).bool());
        e.mem.set_u8(extra + 0x10, 0b110);
        assert!(!e.call(0x0041db00, &args![list]).bool());
        e.mem.set_u8(extra + 0x10, 0b001);
        assert!(e.call(0x0041db00, &args![list]).bool());
    }

    #[test]
    fn flag_bit_zero_of_the_extra_data_is_its_low_bit() {
        let mut e = engine();
        let extra: Ptr<ExtraEnableStateParent> = e.new_object();
        e.set(extra, ExtraEnableStateParent::cFlags, 0xfe);
        assert!(!e.call(0x0041db30, &args![extra]).bool());
        e.set(extra, ExtraEnableStateParent::cFlags, 0x01);
        assert!(e.call(0x0041db30, &args![extra]).bool());
    }

    #[test]
    fn flag_bit_zero_setter_goes_through_the_extra_data_setter() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call(0x0041db50, &args![list, true]);
        let extra = put_extra(&mut e, list, 0x37);
        e.mem.set_u8(extra + 0x10, 0b1000_0110);
        e.call_log = Some(vec![]);
        e.call(0x0041db50, &args![list, true]);
        assert_eq!(e.mem.u8(extra + 0x10), 0b1000_0111);
        e.call(0x0041db50, &args![list, false]);
        assert_eq!(e.mem.u8(extra + 0x10), 0b1000_0110);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, GET_EXTRA_DATA).len(), 2);
    }

    #[test]
    fn flag_bit_zero_setter_on_the_extra_data_keeps_the_other_bits() {
        let mut e = engine();
        let extra: Ptr<ExtraEnableStateParent> = e.new_object();
        e.set(extra, ExtraEnableStateParent::cFlags, 0b0101_0100);
        e.call(0x0041db80, &args![extra, true]);
        assert_eq!(e.get(extra, ExtraEnableStateParent::cFlags), 0b0101_0101);
        e.call(0x0041db80, &args![extra, false]);
        assert_eq!(e.get(extra, ExtraEnableStateParent::cFlags), 0b0101_0100);
        e.call(0x0041db80, &args![extra, false]);
        assert_eq!(e.get(extra, ExtraEnableStateParent::cFlags), 0b0101_0100);
    }

    #[test]
    fn pop_in_when_enabled_by_parent_is_bit_one() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(!e.call(0x0041dbd0, &args![list]).bool());
        let extra = put_extra(&mut e, list, 0x37);
        e.mem.set_u8(extra + 0x10, 0b101);
        assert!(!e.call(0x0041dbd0, &args![list]).bool());
        e.mem.set_u8(extra + 0x10, 0b010);
        assert!(e.call(0x0041dbd0, &args![list]).bool());
    }

    #[test]
    fn flag_bit_one_of_the_extra_data() {
        let mut e = engine();
        let extra: Ptr<ExtraEnableStateParent> = e.new_object();
        e.set(extra, ExtraEnableStateParent::cFlags, 0b101);
        assert!(!e.call(0x0041dc00, &args![extra]).bool());
        e.set(extra, ExtraEnableStateParent::cFlags, 0b010);
        assert!(e.call(0x0041dc00, &args![extra]).bool());
    }

    #[test]
    fn flag_bit_two_getter_reads_the_flags_or_gives_false() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(!e.call(0x0041dc20, &args![list]).bool());
        let extra = put_extra(&mut e, list, 0x37);
        e.mem.set_u8(extra + 0x10, 0b011);
        assert!(!e.call(0x0041dc20, &args![list]).bool());
        e.mem.set_u8(extra + 0x10, 0b100);
        assert!(e.call(0x0041dc20, &args![list]).bool());
    }

    #[test]
    fn flag_bit_two_of_the_extra_data() {
        let mut e = engine();
        let extra: Ptr<ExtraEnableStateParent> = e.new_object();
        e.set(extra, ExtraEnableStateParent::cFlags, 0b011);
        assert!(!e.call(0x0041dc50, &args![extra]).bool());
        e.set(extra, ExtraEnableStateParent::cFlags, 0b100);
        assert!(e.call(0x0041dc50, &args![extra]).bool());
    }

    #[test]
    fn flags_setter_replaces_the_whole_byte_or_does_nothing() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call(0x0041dc70, &args![list, 0x55u32]);
        let extra = put_extra(&mut e, list, 0x37);
        e.mem.set_u8(extra + 0x10, 0xff);
        e.call(0x0041dc70, &args![list, 0x12u32]);
        assert_eq!(e.mem.u8(extra + 0x10), 0x12);
        // Only the byte is stored.
        e.call(0x0041dc70, &args![list, 0x1234u32]);
        assert_eq!(e.mem.u8(extra + 0x10), 0x34);
    }

    // 0041dca0, 0041dcd0, 0041dda0: the children of the enable state parent.

    #[test]
    fn enable_state_children_getter_gives_the_inline_list_or_null() {
        let mut e = engine();
        let list = new_list(&mut e);
        let none = e.call(0x0041dca0, &args![list]).ptr::<BSSimpleList>();
        assert!(none.is_null());
        let extra = put_extra(&mut e, list, 0x38);
        let head = e.call(0x0041dca0, &args![list]).ptr::<BSSimpleList>();
        assert_eq!(head.addr(), extra + 0x0c);
    }

    #[test]
    fn add_enable_state_child_builds_the_extra_data_and_adds_the_child() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041dcd0, &args![list, 0xaaa0u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x38);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, ENABLE_STATE_CHILDREN_INIT).len(), 1);
        assert_eq!(calls_to(&log, ADD_EXTRA), vec![vec![list.addr(), extra]]);
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
        assert_eq!(calls_to(&log, LIST_ADD_HEAD)[0][0], extra + 0x0c);
        assert_eq!(e.mem.u32(extra + 0x0c), 0xaaa0);
    }

    #[test]
    fn add_enable_state_child_skips_a_child_already_in_the_list() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x38);
        e.mem.set_u32(extra + 0x0c, 0xaaa0);
        e.call_log = Some(vec![]);
        e.call(0x0041dcd0, &args![list, 0xaaa0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, LIST_CONTAINS).len(), 1);
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
    }

    #[test]
    fn add_enable_state_child_ignores_a_null_child() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041dcd0, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(extra_of(&e, list, 0x38), 0);
    }

    #[test]
    fn remove_enable_state_child_deletes_the_extra_data_when_empty() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x38);
        e.mem.set_u32(extra + 0x0c, 0xaaa0);
        e.call_log = Some(vec![]);
        e.call(0x0041dda0, &args![list, 0xaaa0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_REMOVE_ITEM)[0][0], extra + 0x0c);
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
        assert_eq!(extra_of(&e, list, 0x38), 0);
    }

    #[test]
    fn remove_enable_state_child_keeps_the_extra_data_when_not_empty() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x38);
        e.mem.set_u32(extra + 0x0c, 0xaaa0);
        e.mem.set_u32(extra + 0x10, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x0041dda0, &args![list, 0xbbb0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        assert_eq!(extra_of(&e, list, 0x38), extra);
    }

    #[test]
    fn remove_enable_state_child_needs_a_child_and_an_extra_data() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041dda0, &args![list, 0xaaa0u32]);
        e.call(0x0041dda0, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_REMOVE_ITEM).is_empty());
        assert!(calls_to(&log, LIST_IS_EMPTY).is_empty());
    }

    // 0041de00, 0041de40: the item dropper.

    #[test]
    fn item_dropper_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041de00, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x39);
        e.mem.set_u32(extra + 0x0c, 0x7777);
        assert_eq!(e.call(0x0041de00, &args![list]).u32(), 0x7777);
    }

    #[test]
    fn add_dropped_item_with_null_removes_by_type() {
        let mut e = engine();
        let list = new_list(&mut e);
        put_extra(&mut e, list, 0x39);
        e.call_log = Some(vec![]);
        e.call(0x0041de40, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x39]]
        );
        assert_eq!(extra_of(&e, list, 0x39), 0);
    }

    #[test]
    fn add_dropped_item_builds_the_extra_data_then_stores_the_dropper() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041de40, &args![list, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x39);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, ITEM_DROPPER_INIT), vec![vec![extra]]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x7777);
    }

    #[test]
    fn add_dropped_item_overwrites_an_existing_dropper() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x39);
        e.mem.set_u32(extra + 0x0c, 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x0041de40, &args![list, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra + 0x0c), 0x7777);
    }

    // 0041df00: the search of the dropped item list.

    /// A reference whose base form has the given form type.
    fn reference_with_form_type(e: &mut Engine, form_type: u8) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, form_type);
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(reference + 0x20, form);
        reference
    }

    fn engine_with_form_doubles() -> Engine {
        let mut e = engine();
        e.register(REFERENCE_BASE_FORM, |e, a| returns(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e
    }

    #[test]
    fn dropped_item_search_without_the_extra_data_gives_zero() {
        let mut e = engine_with_form_doubles();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041df00, &args![list]).u32(), 0);
    }

    #[test]
    fn dropped_item_search_returns_the_first_item_of_form_type_0x28() {
        let mut e = engine_with_form_doubles();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x3a);
        let other = reference_with_form_type(&mut e, 0x10);
        let wanted = reference_with_form_type(&mut e, 0x28);
        let later = reference_with_form_type(&mut e, 0x28);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(extra + 0x0c, other);
        e.mem.set_u32(extra + 0x10, second);
        e.mem.set_u32(second, wanted);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, later);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0041df00, &args![list]).u32(), wanted);
        let log = e.call_log.take().unwrap();
        // The walk stops at the first match: the third node is not read.
        assert_eq!(calls_to(&log, REFERENCE_BASE_FORM).len(), 2);
    }

    #[test]
    fn dropped_item_search_without_a_match_gives_zero() {
        let mut e = engine_with_form_doubles();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x3a);
        let only = reference_with_form_type(&mut e, 0x11);
        e.mem.set_u32(extra + 0x0c, only);
        assert_eq!(e.call(0x0041df00, &args![list]).u32(), 0);
    }

    #[test]
    fn dropped_item_search_stops_at_a_null_item() {
        let mut e = engine_with_form_doubles();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x3a);
        let wanted = reference_with_form_type(&mut e, 0x28);
        let second = e.mem.alloc(8);
        e.mem.set_u32(extra + 0x10, second);
        e.mem.set_u32(second, wanted);
        // The head item is null: the later node is never reached.
        assert_eq!(e.call(0x0041df00, &args![list]).u32(), 0);
    }

    // 0041df90, 0041dfd0, 0041e000, 0041e0d0: the dropped item list.

    #[test]
    fn dropped_item_list_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(e
            .call(0x0041df90, &args![list])
            .ptr::<BSSimpleList>()
            .is_null());
        let extra = put_extra(&mut e, list, 0x3a);
        assert_eq!(e.call(0x0041df90, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn remove_dropped_item_list_only_when_there_is_one() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041dfd0, &args![list]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
        put_extra(&mut e, list, 0x3a);
        e.call_log = Some(vec![]);
        e.call(0x0041dfd0, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x3a]]
        );
        assert_eq!(extra_of(&e, list, 0x3a), 0);
    }

    #[test]
    fn add_to_dropped_item_list_builds_the_extra_data_and_adds_the_item() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041e000, &args![list, 0x4242u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x3a);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, DROPPED_ITEM_LIST_INIT), vec![vec![extra]]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x4242);
    }

    #[test]
    fn add_to_dropped_item_list_skips_an_item_already_there() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x3a);
        e.mem.set_u32(extra + 0x0c, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x0041e000, &args![list, 0x4242u32]);
        e.call(0x0041e000, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn remove_dropped_item_deletes_the_extra_data_when_empty() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x3a);
        e.mem.set_u32(extra + 0x0c, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x0041e0d0, &args![list, 0x4242u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
    }

    #[test]
    fn remove_dropped_item_keeps_a_non_empty_list() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x3a);
        e.mem.set_u32(extra + 0x0c, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x0041e0d0, &args![list, 0x9999u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        assert_eq!(extra_of(&e, list, 0x3a), extra);
    }

    // 0041e130, 0041e160: the water type.

    #[test]
    fn water_type_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e130, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x03);
        e.mem.set_u32(extra + 0x0c, 0x6060);
        assert_eq!(e.call(0x0041e130, &args![list]).u32(), 0x6060);
    }

    #[test]
    fn water_type_setter_stores_the_value_before_adding_a_new_extra_data() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.register(ADD_EXTRA, |e, a| {
            // The value is already in the block when it is added.
            assert_eq!(e.mem.u32(a[1] + 0x0c), 0x6060);
            e.mem.set_u32(table_slot(a[0], 3), a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0041e160, &args![list, 0x6060u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, WATER_TYPE_INIT).len(), 1);
        assert_eq!(e.mem.u32(extra_of(&e, list, 3) + 0x0c), 0x6060);
    }

    #[test]
    fn water_type_setter_overwrites_or_removes() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x03);
        e.call_log = Some(vec![]);
        e.call(0x0041e160, &args![list, 0x7070u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra + 0x0c), 0x7070);
        e.call_log = Some(vec![]);
        e.call(0x0041e160, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 3]]
        );
        assert_eq!(extra_of(&e, list, 3), 0);
    }

    // 0041e220, 0041e250: the type 0x3B marker.

    #[test]
    fn marker_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e220, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x3b);
        e.mem.set_u32(extra + 0x0c, 0x3131);
        assert_eq!(e.call(0x0041e220, &args![list]).u32(), 0x3131);
    }

    #[test]
    fn marker_setter_builds_overwrites_and_removes() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041e250, &args![list, 0x3131u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x3b);
        assert_eq!(calls_to(&log, TELEPORT_MARKER_INIT), vec![vec![extra]]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x3131);
        e.call(0x0041e250, &args![list, 0x3232u32]);
        assert_eq!(extra_of(&e, list, 0x3b), extra);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x3232);
        e.call_log = Some(vec![]);
        e.call(0x0041e250, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x3b]]
        );
    }

    // 0041e310, 0041e340: the ash pile reference.

    #[test]
    fn ash_pile_ref_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e310, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x89);
        e.mem.set_u32(extra + 0x0c, 0x8989);
        assert_eq!(e.call(0x0041e310, &args![list]).u32(), 0x8989);
    }

    #[test]
    fn ash_pile_ref_setter_adds_before_it_stores() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.register(ADD_EXTRA, |e, a| {
            // The value is stored after the add in this function.
            assert_eq!(e.mem.u32(a[1] + 0x0c), 0);
            e.mem.set_u32(table_slot(a[0], 0x89), a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0041e340, &args![list, 0x8989u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x89);
        assert_eq!(calls_to(&log, ASH_PILE_REF_INIT), vec![vec![extra]]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x8989);
    }

    #[test]
    fn ash_pile_ref_setter_overwrites_or_deletes_the_object() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x89);
        e.call(0x0041e340, &args![list, 0x9090u32]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x9090);
        e.call_log = Some(vec![]);
        e.call(0x0041e340, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
        // With no extra data a null reference does nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x0041e340, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
    }

    // 0041e410, 0041e440: the linked reference.

    #[test]
    fn linked_ref_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e410, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x51);
        e.mem.set_u32(extra + 0x0c, 0x5151);
        assert_eq!(e.call(0x0041e410, &args![list]).u32(), 0x5151);
    }

    #[test]
    fn linked_ref_setter_builds_overwrites_and_removes() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041e440, &args![list, 0x5151u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x51);
        assert_eq!(calls_to(&log, LINKED_REF_INIT), vec![vec![extra]]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x5151);
        e.call(0x0041e440, &args![list, 0x5252u32]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x5252);
        e.call(0x0041e440, &args![list, 0u32]);
        assert_eq!(extra_of(&e, list, 0x51), 0);
    }

    // 0041e500, 0041e530, 0041e600: the linked reference children.

    #[test]
    fn linked_ref_children_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(e
            .call(0x0041e500, &args![list])
            .ptr::<BSSimpleList>()
            .is_null());
        let extra = put_extra(&mut e, list, 0x52);
        assert_eq!(e.call(0x0041e500, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn add_linked_ref_child_builds_the_extra_data_and_adds_the_child() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041e530, &args![list, 0x5353u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x52);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, LINKED_REF_CHILDREN_INIT), vec![vec![extra]]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x5353);
        // The same child again, and a null one, change nothing.
        e.call_log = Some(vec![]);
        e.call(0x0041e530, &args![list, 0x5353u32]);
        e.call(0x0041e530, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
    }

    #[test]
    fn remove_linked_ref_child_deletes_the_extra_data_when_empty() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x52);
        e.mem.set_u32(extra + 0x0c, 0x5353);
        e.call_log = Some(vec![]);
        e.call(0x0041e600, &args![list, 0x5353u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
        assert_eq!(extra_of(&e, list, 0x52), 0);
    }

    // 0041e660, 0041e690, 0041e750: the open/close activate reference.

    #[test]
    fn open_close_activate_ref_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e660, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x6c);
        e.mem.set_u32(extra + 0x0c, 0x6c6c);
        assert_eq!(e.call(0x0041e660, &args![list]).u32(), 0x6c6c);
    }

    #[test]
    fn open_close_activate_ref_setter_builds_with_the_constructor_of_this_file() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.register(ADD_EXTRA, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 0x0c), 0x6c6c);
            e.mem.set_u32(table_slot(a[0], 0x6c), a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0041e690, &args![list, 0x6c6cu32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x6c);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_INIT), vec![vec![extra, 0x6c]]);
        assert_eq!(e.mem.u32(extra), 0x0101_51a8);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x6c6c);
    }

    #[test]
    fn open_close_activate_ref_setter_overwrites_or_removes() {
        let mut e = engine();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x6c);
        e.call(0x0041e690, &args![list, 0x6d6du32]);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x6d6d);
        e.call_log = Some(vec![]);
        e.call(0x0041e690, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x6c]]
        );
        assert_eq!(extra_of(&e, list, 0x6c), 0);
    }

    #[test]
    fn open_close_activate_ref_constructor() {
        let mut e = engine();
        let extra: Ptr<ExtraOpenCloseActivateRef> = e.new_object();
        e.mem.set_u32(extra.addr() + 0x0c, 0xdead);
        let result = e
            .call(0x0041e750, &args![extra])
            .ptr::<ExtraOpenCloseActivateRef>();
        assert_eq!(result, extra);
        assert_eq!(e.mem.u8(extra.addr() + 4), 0x6c);
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51a8);
        assert_eq!(
            e.get(extra, ExtraOpenCloseActivateRef::pActivateRef),
            Ptr::NULL
        );
    }

    // 0041e780, 0041e7b0, 0041e7f0, 0041e960, 0041e9e0, 0041ea30, 0041eb60:
    // the activate reference.

    /// Doubles `00433a00` as a search of the first entry of the extra data's
    /// parent list (an entry is `[reference, delay]`, found when its first
    /// word is the reference; the test lists are one entry long).
    fn activate_find_double(e: &mut Engine) {
        e.register(ACTIVATE_REF_FIND, |e, a| {
            let entry = e.mem.u32(a[0] + 0x0c);
            let found = entry != 0 && e.mem.u32(entry) == a[1];
            returns(if found { entry } else { 0 })
        });
    }

    #[test]
    fn parent_list_getter() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(e
            .call(0x0041e780, &args![list])
            .ptr::<BSSimpleList>()
            .is_null());
        let extra = put_extra(&mut e, list, 0x53);
        assert_eq!(e.call(0x0041e780, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn parent_entry_finder_asks_the_extra_data() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e7b0, &args![list, 0x5000u32]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x53);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(extra + 0x0c, entry);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0041e7b0, &args![list, 0x5000u32]).u32(), entry);
        assert_eq!(e.call(0x0041e7b0, &args![list, 0x5001u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ACTIVATE_REF_FIND).len(), 2);
        assert_eq!(calls_to(&log, ACTIVATE_REF_FIND)[0], vec![extra, 0x5000]);
    }

    #[test]
    fn add_activate_parent_builds_the_extra_data_and_the_entry() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041e7f0, &args![list, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x53);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20], vec![0x08]]);
        assert_eq!(calls_to(&log, ACTIVATE_REF_INIT), vec![vec![extra]]);
        // A new extra data is not searched.
        assert!(calls_to(&log, ACTIVATE_REF_FIND).is_empty());
        let entry = e.mem.u32(extra + 0x0c);
        assert_ne!(entry, 0);
        assert_eq!(e.mem.u32(entry), 0x5000);
        assert_eq!(e.mem.u32(entry + 4), 0);
        assert_eq!(calls_to(&log, LIST_NODE_INIT).len(), 1);
    }

    #[test]
    fn add_activate_parent_adds_an_entry_only_for_a_new_parent() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x53);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(extra + 0x0c, entry);
        e.call_log = Some(vec![]);
        e.call(0x0041e7f0, &args![list, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0041e7f0, &args![list, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x08]]);
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
        assert!(calls_to(&log, ADD_EXTRA).is_empty());
        let added = e.mem.u32(extra + 0x0c);
        assert_ne!(added, entry);
        assert_eq!(e.mem.u32(added), 0x6000);
    }

    #[test]
    fn add_activate_parent_ignores_a_null_reference() {
        let mut e = engine();
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041e7f0, &args![list, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn remove_activate_parent_deletes_the_entry_and_the_empty_extra_data() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x53);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(extra + 0x0c, entry);
        e.call_log = Some(vec![]);
        e.call(0x0041e960, &args![list, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_REMOVE_ITEM)[0][0], extra + 0x0c);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![entry]]);
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x53]]
        );
        assert_eq!(extra_of(&e, list, 0x53), 0);
    }

    #[test]
    fn remove_activate_parent_keeps_a_non_empty_list_and_ignores_unknown_parents() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x53);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(extra + 0x0c, entry);
        e.mem.set_u32(extra + 0x10, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0041e960, &args![list, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![entry]]);
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0041e960, &args![list, 0x7000u32]);
        e.call(0x0041e960, &args![list, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert!(calls_to(&log, LIST_REMOVE_ITEM).is_empty());
    }

    #[test]
    fn activate_delay_getter() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041e9e0, &args![list, 0x5000u32]).f32(), 0.0);
        let extra = put_extra(&mut e, list, 0x53);
        assert_eq!(e.call(0x0041e9e0, &args![list, 0x5000u32]).f32(), 0.0);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(entry + 4, 2.5f32.to_bits());
        e.mem.set_u32(extra + 0x0c, entry);
        assert_eq!(e.call(0x0041e9e0, &args![list, 0x5000u32]).f32(), 2.5);
        assert_eq!(e.call(0x0041e9e0, &args![list, 0x5001u32]).f32(), 0.0);
        // A signalling NaN comes out of the x87 load quiet.
        e.mem.set_u32(entry + 4, 0x7f80_0001);
        let result = e.call(0x0041e9e0, &args![list, 0x5000u32]).f32();
        assert_eq!(result.to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn set_activate_delay_builds_everything_for_a_new_parent() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041ea30, &args![list, 0x5000u32, 1.5f32]);
        let log = e.call_log.take().unwrap();
        let extra = extra_of(&e, list, 0x53);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20], vec![0x08]]);
        assert_eq!(calls_to(&log, ACTIVATE_REF_INIT), vec![vec![extra]]);
        let entry = e.mem.u32(extra + 0x0c);
        assert_eq!(e.mem.u32(entry), 0x5000);
        assert_eq!(f32::from_bits(e.mem.u32(entry + 4)), 1.5);
    }

    #[test]
    fn set_activate_delay_updates_an_existing_entry() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x53);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(extra + 0x0c, entry);
        e.call_log = Some(vec![]);
        e.call(0x0041ea30, &args![list, 0x5000u32, 4.0f32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        assert_eq!(f32::from_bits(e.mem.u32(entry + 4)), 4.0);
    }

    #[test]
    fn set_activate_delay_adds_an_entry_for_a_new_parent_of_an_existing_extra_data() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x53);
        e.call_log = Some(vec![]);
        e.call(0x0041ea30, &args![list, 0x6000u32, 0.25f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x08]]);
        assert!(calls_to(&log, ADD_EXTRA).is_empty());
        let entry = e.mem.u32(extra + 0x0c);
        assert_eq!(e.mem.u32(entry), 0x6000);
        assert_eq!(f32::from_bits(e.mem.u32(entry + 4)), 0.25);
    }

    #[test]
    fn set_activate_delay_ignores_a_null_reference_and_quiets_a_nan() {
        let mut e = engine();
        activate_find_double(&mut e);
        let list = new_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0041ea30, &args![list, 0u32, 1.0f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let extra = put_extra(&mut e, list, 0x53);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5000);
        e.mem.set_u32(extra + 0x0c, entry);
        e.call(
            0x0041ea30,
            &args![list, 0x5000u32, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(e.mem.u32(entry + 4), 0x7fc0_0001);
    }

    #[test]
    fn activate_flag_getter_is_bit_zero_of_the_activate_flags() {
        let mut e = engine();
        let list = new_list(&mut e);
        assert!(!e.call(0x0041eb60, &args![list]).bool());
        let extra = put_extra(&mut e, list, 0x53);
        e.mem.set_u8(extra + 0x14, 0b10);
        assert!(!e.call(0x0041eb60, &args![list]).bool());
        e.mem.set_u8(extra + 0x14, 0b11);
        assert!(e.call(0x0041eb60, &args![list]).bool());
    }

    // ------------------------------------------------------------------
    // Second session: 0041eba0 to 00420210.

    /// The engine of the first session, with the constructors and helpers
    /// the second session's functions call.
    fn engine_b() -> Engine {
        let mut e = engine();
        constructor_double(&mut e, ACTIVATE_REF_CHILDREN_INIT, 0x54);
        constructor_double(&mut e, DECAL_REFS_INIT, 0x57);
        constructor_double(&mut e, REFLECTED_REFS_INIT, 0x65);
        constructor_double(&mut e, REFLECTOR_REFS_INIT, 0x66);
        constructor_double(&mut e, WATER_LIGHT_REFS_INIT, 0x84);
        constructor_double(&mut e, LIT_WATER_REFS_INIT, 0x85);
        e.register(LIST_NODE_INIT, |_, a| returns(a[0]));
        e.register(OCCLUSION_PLANE_DATA_INIT, |_, a| returns(a[0]));
        e.register(STORE_WORD_AT_0C, |e, a| {
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            Ret::default()
        });
        e.register(BS_EXTRA_DATA_DESTROY, |_, _| Ret::default());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e
    }

    /// An 8-byte entry (first word `reference`) made the head item of the
    /// list at +0x0C of `extra`.
    fn put_head_entry(e: &mut Engine, extra: u32, reference: u32) -> u32 {
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, reference);
        e.mem.set_u32(extra + 0x0c, entry);
        entry
    }

    /// Runs `f` and gives the calls it made.
    fn logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    // 0041eba0: the activate flag setter.

    #[test]
    fn activate_flag_setter_builds_sets_and_clears_bit_zero() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        // Clearing with no extra data builds nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041eba0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(extra_of(&e, list, 0x53), 0);
        // Setting builds a 0x20-byte one, adds it and sets bit 0 (only the
        // low byte of the argument counts).
        let log = logged(&mut e, |e| {
            e.call(0x0041eba0, &args![list, 0x0100_0001u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        let extra = extra_of(&e, list, 0x53);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, ACTIVATE_REF_INIT).len(), 1);
        assert_eq!(e.mem.u8(extra + 0x14), 0b1);
        // The other bits stay, and the extra data is not built again.
        e.mem.set_u8(extra + 0x14, 0b1010_0100);
        let log = logged(&mut e, |e| {
            e.call(0x0041eba0, &args![list, 1u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u8(extra + 0x14), 0b1010_0101);
        e.call(0x0041eba0, &args![list, 0u32]);
        assert_eq!(e.mem.u8(extra + 0x14), 0b1010_0100);
    }

    // 0041ec80: the activate text getter.

    #[test]
    fn activate_text_getter_copies_the_override_or_makes_an_empty_string() {
        let mut e = engine_b();
        e.register(STRING_COPY_CONSTRUCT, |_, a| returns(a[0]));
        e.register(STRING_CONSTRUCT_FROM_CHARS, |_, a| returns(a[0]));
        let list = new_list(&mut e);
        let out = e.mem.alloc(8);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041ec80, &args![list, out]).u32(), out);
        });
        assert_eq!(
            calls_to(&log, STRING_CONSTRUCT_FROM_CHARS),
            vec![vec![out, EMPTY_TEXT]]
        );
        assert!(calls_to(&log, STRING_COPY_CONSTRUCT).is_empty());
        let extra = put_extra(&mut e, list, 0x53);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041ec80, &args![list, out]).u32(), out);
        });
        assert_eq!(
            calls_to(&log, STRING_COPY_CONSTRUCT),
            vec![vec![out, extra + 0x18]]
        );
        assert!(calls_to(&log, STRING_CONSTRUCT_FROM_CHARS).is_empty());
    }

    // 0041ece0: the activate text setter.

    #[test]
    fn activate_text_setter_builds_the_extra_data_only_for_a_text() {
        let mut e = engine_b();
        e.register(STRING_SET, |_, _| Ret::default());
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0041ece0, &args![list, 0u32]);
        });
        assert_eq!(log.len(), 1 + 1, "the call itself and the lookup");
        assert_eq!(extra_of(&e, list, 0x53), 0);
        let log = logged(&mut e, |e| {
            e.call(0x0041ece0, &args![list, 0x7000u32]);
        });
        let extra = extra_of(&e, list, 0x53);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        assert_eq!(
            calls_to(&log, STRING_SET),
            vec![vec![extra + 0x18, 0x7000, 0]]
        );
        // An existing one is given the text, also a null one.
        let log = logged(&mut e, |e| {
            e.call(0x0041ece0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, STRING_SET), vec![vec![extra + 0x18, 0, 0]]);
    }

    // 0041eda0, 0041edd0, 0041ef20, 0041eff0, 0041f020: the activate
    // reference children.

    #[test]
    fn activate_children_getter_gives_the_inline_list_or_null() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert!(e
            .call(0x0041eda0, &args![list])
            .ptr::<BSSimpleList>()
            .is_null());
        let extra = put_extra(&mut e, list, 0x54);
        assert_eq!(e.call(0x0041eda0, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn add_activate_child_builds_the_extra_data_and_an_entry_once() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        // A null child does nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041edd0, &args![list, 0u32]);
        });
        assert_eq!(log.len(), 1);
        // The first child builds the 0x18-byte extra data and an 8-byte
        // entry, which holds the child and goes to the head of the list.
        let log = logged(&mut e, |e| {
            e.call(0x0041edd0, &args![list, 0x5001u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x18], vec![8]]);
        let extra = extra_of(&e, list, 0x54);
        assert_eq!(calls_to(&log, ACTIVATE_REF_CHILDREN_INIT).len(), 1);
        let entry = e.mem.u32(extra + 0x0c);
        assert_eq!(e.mem.u32(entry), 0x5001);
        assert_eq!(calls_to(&log, LIST_NODE_INIT), vec![vec![entry]]);
        // The same child again: found, nothing is built.
        let log = logged(&mut e, |e| {
            e.call(0x0041edd0, &args![list, 0x5001u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        // Another child: a new entry (the head-node double keeps one item).
        let log = logged(&mut e, |e| {
            e.call(0x0041edd0, &args![list, 0x5002u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![8]]);
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
        let entry = e.mem.u32(extra + 0x0c);
        assert_eq!(e.mem.u32(entry), 0x5002);
    }

    #[test]
    fn add_activate_child_finds_the_child_further_down_the_list() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x54);
        put_head_entry(&mut e, extra, 0x5001);
        let second = e.mem.alloc(8);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5002);
        e.mem.set_u32(second, entry);
        e.mem.set_u32(extra + 0x0c + 4, second);
        let log = logged(&mut e, |e| {
            e.call(0x0041edd0, &args![list, 0x5002u32]);
        });
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0041edd0, &args![list, 0x5003u32]);
        });
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
    }

    #[test]
    fn remove_activate_child_deletes_the_entry_and_the_empty_extra_data() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        // No extra data, or a null child: only the lookup.
        let log = logged(&mut e, |e| {
            e.call(0x0041ef20, &args![list, 0x5001u32]);
            e.call(0x0041ef20, &args![list, 0u32]);
        });
        assert_eq!(log.len(), 3);
        let extra = put_extra(&mut e, list, 0x54);
        let entry = put_head_entry(&mut e, extra, 0x5001);
        // A child that is not in the list: nothing removed, the extra data
        // stays (the list is not empty).
        let log = logged(&mut e, |e| {
            e.call(0x0041ef20, &args![list, 0x5009u32]);
        });
        assert!(calls_to(&log, LIST_REMOVE_HEAD).is_empty());
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        // The child: taken out with the remove-head call on its node,
        // deleted, and the empty list takes the extra data with it.
        let log = logged(&mut e, |e| {
            e.call(0x0041ef20, &args![list, 0x5001u32]);
        });
        assert_eq!(calls_to(&log, LIST_REMOVE_HEAD), vec![vec![extra + 0x0c]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![entry]]);
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
        assert_eq!(extra_of(&e, list, 0x54), 0);
    }

    #[test]
    fn remove_activate_child_uses_remove_head_even_for_a_later_node() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x54);
        let first = put_head_entry(&mut e, extra, 0x5001);
        let second = e.mem.alloc(8);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x5002);
        e.mem.set_u32(second, entry);
        e.mem.set_u32(extra + 0x0c + 4, second);
        let log = logged(&mut e, |e| {
            e.call(0x0041ef20, &args![list, 0x5002u32]);
        });
        // The node the child was found at, not its predecessor.
        assert_eq!(calls_to(&log, LIST_REMOVE_HEAD), vec![vec![second]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![entry]]);
        assert!(calls_to(&log, LIST_REMOVE_AFTER).is_empty());
        // The list still holds the first entry: the extra data stays.
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        assert_eq!(e.mem.u32(extra + 0x0c), first);
    }

    #[test]
    fn activate_children_timer_setter_and_getter() {
        let mut e = engine_b();
        e.map(NO_TIMER, 4);
        e.set_global(NO_TIMER, -1.0f32);
        let list = new_list(&mut e);
        // No extra data: the setter does nothing, the getter gives -1.0.
        e.call(0x0041eff0, &args![list, 2.5f32]);
        assert_eq!(extra_of(&e, list, 0x54), 0);
        assert_eq!(e.call(0x0041f020, &args![list]).f32(), -1.0);
        let extra = put_extra(&mut e, list, 0x54);
        e.call(0x0041eff0, &args![list, 2.5f32]);
        assert_eq!(f32::from_bits(e.mem.u32(extra + 0x14)), 2.5);
        assert_eq!(e.call(0x0041f020, &args![list]).f32(), 2.5);
        // The x87 load and store quiet a signalling NaN.
        e.call(0x0041eff0, &args![list, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(extra + 0x14), 0x7fc0_0001);
        e.mem.set_u32(extra + 0x14, 0x7f80_0002);
        assert_eq!(
            e.call(0x0041f020, &args![list]).f32().to_bits(),
            0x7fc0_0002
        );
    }

    // 0041f050, 0041f080: the decal references.

    #[test]
    fn decal_refs_getter_gives_the_inline_list_or_null() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041f050, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x57);
        assert_eq!(e.call(0x0041f050, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn add_decal_ref_builds_the_extra_data_and_passes_the_points_on() {
        let mut e = engine_b();
        e.register(DECAL_REFS_ADD, |_, _| Ret::default());
        let list = new_list(&mut e);
        let intersect = e.mem.alloc(12);
        let normal = e.mem.alloc(12);
        // A null reference does nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041f080, &args![list, 0u32, intersect, normal]);
        });
        assert_eq!(log.len(), 1);
        let log = logged(&mut e, |e| {
            e.call(0x0041f080, &args![list, 0x6001u32, intersect, normal]);
        });
        let extra = extra_of(&e, list, 0x57);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(
            calls_to(&log, DECAL_REFS_ADD),
            vec![vec![extra, 0x6001, intersect, normal]]
        );
        // With the extra data there, it is not built again.
        let log = logged(&mut e, |e| {
            e.call(0x0041f080, &args![list, 0x6002u32, intersect, normal]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, DECAL_REFS_ADD),
            vec![vec![extra, 0x6002, intersect, normal]]
        );
    }

    // 0041f140, 0041f170, 0041f7e0, 0041f810: the reference list getters.

    #[test]
    fn reflected_refs_getter_gives_the_inline_list_or_null() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041f140, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x65);
        assert_eq!(e.call(0x0041f140, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn reflector_refs_getter_gives_the_inline_list_or_null() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041f170, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x66);
        assert_eq!(e.call(0x0041f170, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn water_light_refs_getter_gives_the_inline_list_or_null() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041f7e0, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x84);
        assert_eq!(e.call(0x0041f7e0, &args![list]).u32(), extra + 0x0c);
    }

    #[test]
    fn lit_water_refs_getter_gives_the_inline_list_or_null() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041f810, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x85);
        assert_eq!(e.call(0x0041f810, &args![list]).u32(), extra + 0x0c);
    }

    // 0041f1a0, 0041f330, 0041f4c0, 0041f650: the reflector and reflected
    // flag setters, one body for the four.

    /// The branches of the shared body for the function at `address`, which
    /// works on extra data `extra_type` and the flag `bit`.
    fn check_reflect_flag_setter(address: u32, extra_type: u8, bit: u32) {
        let mut e = engine_b();
        e.register(REFLECT_REFS_REMOVE, |e, a| {
            // Takes the only entry out of the list.
            e.mem.set_u32(a[0] + 0x0c, 0);
            Ret::default()
        });
        e.register(REFLECT_REFS_SET_FLAGS, |_, _| Ret::default());
        let list = new_list(&mut e);
        let reference = 0x7001u32;
        let set = |e: &mut Engine, value: u32| {
            logged(e, |e| {
                e.call(address, &args![list, reference, value]);
            })
        };
        // Clearing with no extra data: nothing.
        let log = set(&mut e, 0);
        assert_eq!(log.len(), 2);
        // Setting with no extra data: built (0x14 bytes), added, and the
        // entry added with the bit as flags.
        let log = set(&mut e, 1);
        let extra = extra_of(&e, list, extra_type);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(
            calls_to(&log, REFLECT_REFS_SET_FLAGS),
            vec![vec![extra, reference, bit]]
        );
        // Setting with an extra data that has no entry for the reference:
        // not built again, the entry added.
        put_head_entry(&mut e, extra, 0x7777);
        let log = set(&mut e, 1);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, REFLECT_REFS_SET_FLAGS).len(), 1);
        // Clearing with no entry for the reference: nothing.
        let log = set(&mut e, 0);
        assert!(calls_to(&log, REFLECT_REFS_REMOVE).is_empty());
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        // A null first entry ends the walk even if more follow.
        let behind = e.mem.alloc(8);
        let wanted = e.mem.alloc(8);
        e.mem.set_u32(wanted, reference);
        e.mem.set_u32(behind, wanted);
        e.mem.set_u32(extra + 0x0c, 0);
        e.mem.set_u32(extra + 0x0c + 4, behind);
        let log = set(&mut e, 1);
        assert_eq!(calls_to(&log, REFLECT_REFS_SET_FLAGS).len(), 1);
        assert_eq!(e.mem.u32(wanted + 4), 0);
        // The entry is there: setting ors the bit in, no entry is added.
        let entry = put_head_entry(&mut e, extra, reference);
        e.mem.set_u32(extra + 0x0c + 4, 0);
        e.mem.set_u32(entry + 4, 0x10);
        let log = set(&mut e, 1);
        assert!(calls_to(&log, REFLECT_REFS_SET_FLAGS).is_empty());
        assert_eq!(e.mem.u32(entry + 4), 0x10 | bit);
        // Clearing removes the bit and keeps the entry while other flags
        // are left.
        let log = set(&mut e, 0);
        assert!(calls_to(&log, REFLECT_REFS_REMOVE).is_empty());
        assert_eq!(e.mem.u32(entry + 4), 0x10);
        // The last flag gone: the entry removed, then the empty extra data.
        e.mem.set_u32(entry + 4, bit);
        let log = set(&mut e, 0);
        assert_eq!(
            calls_to(&log, REFLECT_REFS_REMOVE),
            vec![vec![extra, reference]]
        );
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
        assert_eq!(extra_of(&e, list, extra_type), 0);
    }

    #[test]
    fn reflector_flag_zero_setter() {
        check_reflect_flag_setter(0x0041f1a0, 0x66, 1);
    }

    #[test]
    fn reflector_flag_one_setter() {
        check_reflect_flag_setter(0x0041f330, 0x66, 2);
    }

    #[test]
    fn reflected_flag_zero_setter() {
        check_reflect_flag_setter(0x0041f4c0, 0x65, 1);
    }

    #[test]
    fn reflected_flag_one_setter() {
        check_reflect_flag_setter(0x0041f650, 0x65, 2);
    }

    #[test]
    fn reflect_setters_keep_the_other_bit_of_a_shared_entry() {
        // Both flag setters of a class work on the same entry.
        let mut e = engine_b();
        e.register(REFLECT_REFS_REMOVE, |_, _| Ret::default());
        let list = new_list(&mut e);
        let extra = put_extra(&mut e, list, 0x66);
        let entry = put_head_entry(&mut e, extra, 0x7001);
        e.call(0x0041f1a0, &args![list, 0x7001u32, 1u32]);
        e.call(0x0041f330, &args![list, 0x7001u32, 1u32]);
        assert_eq!(e.mem.u32(entry + 4), 3);
        e.call(0x0041f1a0, &args![list, 0x7001u32, 0u32]);
        assert_eq!(e.mem.u32(entry + 4), 2);
    }

    // 0041f840, 0041f940: the water light and lit water setters.

    /// The branches of the shared body for the function at `address`, which
    /// works on extra data `extra_type` (constructor double of that type in
    /// `engine_b`), deleting the empty extra data after a removal when
    /// `deletes_when_empty`.
    fn check_reference_list_setter(address: u32, extra_type: u8, deletes_when_empty: bool) {
        let mut e = engine_b();
        let list = new_list(&mut e);
        let reference = 0x8001u32;
        let call = |e: &mut Engine, add: u32| {
            logged(e, |e| {
                e.call(address, &args![list, reference, add]);
            })
        };
        // Removing with no extra data: nothing.
        let log = call(&mut e, 0);
        assert_eq!(log.len(), 2);
        // Adding with none: built (0x14 bytes), added, the reference put in
        // its list through a local (the list calls get its address).
        let log = call(&mut e, 1);
        let extra = extra_of(&e, list, extra_type);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let added = calls_to(&log, LIST_ADD_HEAD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], extra + 0x0c);
        assert_eq!(e.mem.u32(extra + 0x0c), reference);
        // Adding what the list holds: only the lookup.
        let log = call(&mut e, 1);
        assert_eq!(calls_to(&log, LIST_CONTAINS).len(), 1);
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Removing it: the list call gets the head and the address of a
        // local that holds the reference.
        let log = call(&mut e, 0);
        let removed = calls_to(&log, LIST_REMOVE_ITEM);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], extra + 0x0c);
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA).len(),
            deletes_when_empty as usize
        );
        assert_eq!(extra_of(&e, list, extra_type) == 0, deletes_when_empty);
        // Adding to an extra data that is there, not holding it.
        let extra = put_extra(&mut e, list, extra_type);
        e.mem.set_u32(extra + 0x0c, 0x8002);
        let log = call(&mut e, 1);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
        // Removing from a list that is not left empty keeps the extra data.
        e.mem.set_u32(extra + 0x0c, 0x8002);
        e.mem.set_u32(extra + 0x0c + 4, 0x1234);
        let log = call(&mut e, 0);
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
    }

    #[test]
    fn water_light_ref_setter_never_deletes_the_extra_data() {
        check_reference_list_setter(0x0041f840, 0x84, false);
    }

    #[test]
    fn lit_water_ref_setter_deletes_the_empty_extra_data() {
        check_reference_list_setter(0x0041f940, 0x85, true);
    }

    // 0041fa60, 0041faf0, 0041fb20, 0041fb50, 0041fbe0: the primitive.

    const PRIMITIVE_DESTRUCTOR: u32 = 0x00a0_0100;
    const PRIMITIVE_VTABLE: u32 = 0x00a0_0000;

    /// A fake primitive whose vtable has a scalar deleting destructor
    /// double at slot 0.
    fn fake_primitive(e: &mut Engine) -> u32 {
        e.put_vtable(PRIMITIVE_VTABLE, &[PRIMITIVE_DESTRUCTOR]);
        e.register(PRIMITIVE_DESTRUCTOR, |_, a| returns(a[0]));
        let primitive = e.mem.alloc(0x20);
        e.mem.set_u32(primitive, PRIMITIVE_VTABLE);
        primitive
    }

    #[test]
    fn add_primitive_builds_an_extra_primitive_owning_it() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0041fa60, &args![list, 0u32]);
        });
        assert_eq!(log.len(), 1);
        let log = logged(&mut e, |e| {
            e.call(0x0041fa60, &args![list, 0x9001u32]);
        });
        let extra = extra_of(&e, list, 0x6b);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(extra), 0x0101_51b4);
        assert_eq!(e.mem.u32(extra + 0x0c), 0x9001);
        assert_eq!(calls_to(&log, ADD_EXTRA), vec![vec![list.addr(), extra]]);
        // An existing one is not looked for: another is built.
        e.call(0x0041fa60, &args![list, 0x9002u32]);
        assert_eq!(e.mem.u32(extra_of(&e, list, 0x6b) + 0x0c), 0x9002);
    }

    #[test]
    fn extra_primitive_constructor() {
        let mut e = engine_b();
        let extra: Ptr<ExtraPrimitive> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0041faf0, &args![extra, 0x9001u32]).u32(),
                extra.addr()
            );
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x6b]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51b4);
        assert_eq!(e.get(extra, ExtraPrimitive::pPrimitive).addr(), 0x9001);
    }

    #[test]
    fn extra_primitive_destructor_deletes_the_primitive_it_owns() {
        let mut e = engine_b();
        let primitive = fake_primitive(&mut e);
        let extra: Ptr<ExtraPrimitive> = e.new_object();
        e.set(extra, ExtraPrimitive::pPrimitive, Ptr::new(primitive));
        let log = logged(&mut e, |e| {
            e.call(0x0041fb50, &args![extra]);
        });
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51b4);
        assert_eq!(
            calls_to(&log, PRIMITIVE_DESTRUCTOR),
            vec![vec![primitive, 1]]
        );
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_DESTROY),
            vec![vec![extra.addr()]]
        );
        // Without a primitive only the base destructor runs.
        e.set(extra, ExtraPrimitive::pPrimitive, Ptr::NULL);
        let log = logged(&mut e, |e| {
            e.call(0x0041fb50, &args![extra]);
        });
        assert!(calls_to(&log, PRIMITIVE_DESTRUCTOR).is_empty());
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
    }

    #[test]
    fn extra_primitive_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_b();
        let extra: Ptr<ExtraPrimitive> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fb20, &args![extra, 0u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fb20, &args![extra, 3u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
    }

    #[test]
    fn primitive_getter() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041fbe0, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x6b);
        e.mem.set_u32(extra + 0x0c, 0x9001);
        assert_eq!(e.call(0x0041fbe0, &args![list]).u32(), 0x9001);
    }

    // 0041fc10, 0041fcd0, 0041fd20, 0041fd80, 0041fdb0, 0041fe10, 0041fe60,
    // 0041fe90: the patrol reference data.

    #[test]
    fn patrol_ref_data_setter_replaces_the_extra_data() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        // Null with none: nothing built.
        let log = logged(&mut e, |e| {
            e.call(0x0041fc10, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Data: built (0x10 bytes), given the data, added.
        let log = logged(&mut e, |e| {
            e.call(0x0041fc10, &args![list, 0xa001u32]);
        });
        let first = extra_of(&e, list, 0x6f);
        assert_ne!(first, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(first), 0x0101_51c0);
        assert_eq!(e.mem.u32(first + 0x0c), 0xa001);
        assert_eq!(calls_to(&log, STORE_WORD_AT_0C), vec![vec![first, 0xa001]]);
        // New data: the old extra data is deleted (the object), a new one
        // built.
        let log = logged(&mut e, |e| {
            e.call(0x0041fc10, &args![list, 0xa002u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), first, 1]]
        );
        let second = extra_of(&e, list, 0x6f);
        assert_eq!(e.mem.u32(second + 0x0c), 0xa002);
        // Null: only the deletion.
        let log = logged(&mut e, |e| {
            e.call(0x0041fc10, &args![list, 0u32]);
        });
        assert_eq!(calls_to(&log, REMOVE_EXTRA).len(), 1);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(extra_of(&e, list, 0x6f), 0);
    }

    #[test]
    fn patrol_ref_data_constructor_leaves_the_data_word() {
        let mut e = engine_b();
        let extra: Ptr<ExtraPatrolRefData> = e.new_object();
        e.set(extra, ExtraPatrolRefData::pPatrolData, Ptr::new(0xa5a5));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fcd0, &args![extra]).u32(), extra.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x6f]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51c0);
        assert_eq!(e.get(extra, ExtraPatrolRefData::pPatrolData).addr(), 0xa5a5);
    }

    #[test]
    fn patrol_ref_data_compare_decides_by_presence_then_by_the_data() {
        let mut e = engine_b();
        e.register(PATROL_DATA_COMPARE, |e, a| {
            returns((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        let this: Ptr<ExtraPatrolRefData> = e.new_object();
        let other: Ptr<ExtraPatrolRefData> = e.new_object();
        // Neither has data: equal.
        assert!(!e.call(0x0041fd20, &args![this, other]).bool());
        // Only one has: different.
        let data = e.mem.alloc(4);
        e.set(this, ExtraPatrolRefData::pPatrolData, Ptr::new(data));
        assert!(e.call(0x0041fd20, &args![this, other]).bool());
        e.set(this, ExtraPatrolRefData::pPatrolData, Ptr::NULL);
        e.set(other, ExtraPatrolRefData::pPatrolData, Ptr::new(data));
        assert!(e.call(0x0041fd20, &args![this, other]).bool());
        // Both: the patrol data compare gets this's data as `this` and
        // other's as its argument, and decides.
        let same = e.mem.alloc(4);
        e.set(this, ExtraPatrolRefData::pPatrolData, Ptr::new(same));
        e.mem.set_u32(same, 7);
        e.mem.set_u32(data, 7);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0041fd20, &args![this, other]).bool());
        });
        assert_eq!(calls_to(&log, PATROL_DATA_COMPARE), vec![vec![same, data]]);
        e.mem.set_u32(data, 8);
        assert!(e.call(0x0041fd20, &args![this, other]).bool());
    }

    #[test]
    fn patrol_ref_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_b();
        e.register(PATROL_DATA_DESTROY, |_, _| Ret::default());
        let extra: Ptr<ExtraPatrolRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fd80, &args![extra, 0u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fd80, &args![extra, 1u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
    }

    #[test]
    fn patrol_ref_data_destructor_deletes_the_data_then_the_base() {
        let mut e = engine_b();
        e.register(PATROL_DATA_DESTROY, |_, _| Ret::default());
        let extra: Ptr<ExtraPatrolRefData> = e.new_object();
        let data = e.mem.alloc(0x20);
        e.set(extra, ExtraPatrolRefData::pPatrolData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x0041fdb0, &args![extra]);
        });
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51c0);
        assert_eq!(calls_to(&log, PATROL_DATA_DESTROY), vec![vec![data]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(e.get(extra, ExtraPatrolRefData::pPatrolData).addr(), 0);
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            order,
            vec![
                0x0041fdb0,
                PATROL_DATA_DESTROY,
                OPERATOR_DELETE,
                BS_EXTRA_DATA_DESTROY
            ]
        );
    }

    #[test]
    fn patrol_data_cleanup_does_nothing_without_data() {
        let mut e = engine_b();
        e.register(PATROL_DATA_DESTROY, |_, _| Ret::default());
        let extra: Ptr<ExtraPatrolRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            e.call(0x0041fe10, &args![extra]);
        });
        assert_eq!(log.len(), 1);
        let data = e.mem.alloc(0x20);
        e.set(extra, ExtraPatrolRefData::pPatrolData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x0041fe10, &args![extra]);
        });
        assert_eq!(calls_to(&log, PATROL_DATA_DESTROY).len(), 1);
        assert_eq!(e.get(extra, ExtraPatrolRefData::pPatrolData).addr(), 0);
    }

    #[test]
    fn patrol_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_b();
        e.register(PATROL_DATA_DESTROY, |_, _| Ret::default());
        let data = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fe60, &args![data, 0u32]).u32(), data);
        });
        assert_eq!(calls_to(&log, PATROL_DATA_DESTROY), vec![vec![data]]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041fe60, &args![data, 1u32]).u32(), data);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
    }

    #[test]
    fn patrol_ref_data_getter() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x0041fe90, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x6f);
        e.mem.set_u32(extra + 0x0c, 0xa001);
        assert_eq!(e.call(0x0041fe90, &args![list]).u32(), 0xa001);
    }

    // 0041fec0, 0041ff80, 0041ffb0, 00420030, 00420060, 004200c0, 00420100,
    // 00420130, 00420160: the occlusion plane reference data.

    #[test]
    fn occlusion_plane_ref_data_setter_replaces_the_extra_data() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0041fec0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0041fec0, &args![list, 0xb001u32]);
        });
        let first = extra_of(&e, list, 0x76);
        assert_ne!(first, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(first), 0x0101_51cc);
        assert_eq!(e.mem.u32(first + 0x0c), 0xb001);
        let log = logged(&mut e, |e| {
            e.call(0x0041fec0, &args![list, 0xb002u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), first, 1]]
        );
        let second = extra_of(&e, list, 0x76);
        assert_eq!(e.mem.u32(second + 0x0c), 0xb002);
        e.call(0x0041fec0, &args![list, 0u32]);
        assert_eq!(extra_of(&e, list, 0x76), 0);
    }

    #[test]
    fn occlusion_plane_ref_data_constructor() {
        let mut e = engine_b();
        let extra: Ptr<ExtraOcclusionPlaneRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041ff80, &args![extra]).u32(), extra.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x76]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51cc);
    }

    #[test]
    fn occlusion_plane_ref_data_compare_looks_at_four_words() {
        let mut e = engine_b();
        let this: Ptr<ExtraOcclusionPlaneRefData> = e.new_object();
        let other: Ptr<ExtraOcclusionPlaneRefData> = e.new_object();
        assert!(!e.call(0x0041ffb0, &args![this, other]).bool());
        let mine = e.mem.alloc(0x14);
        let theirs = e.mem.alloc(0x14);
        e.set(this, ExtraOcclusionPlaneRefData::pData, Ptr::new(mine));
        assert!(e.call(0x0041ffb0, &args![this, other]).bool());
        e.set(this, ExtraOcclusionPlaneRefData::pData, Ptr::NULL);
        e.set(other, ExtraOcclusionPlaneRefData::pData, Ptr::new(theirs));
        assert!(e.call(0x0041ffb0, &args![this, other]).bool());
        e.set(this, ExtraOcclusionPlaneRefData::pData, Ptr::new(mine));
        assert!(!e.call(0x0041ffb0, &args![this, other]).bool());
        // Each of the four words counts, the fifth does not.
        for word in 0..4 {
            e.mem.set_u32(theirs + 4 * word, 1);
            assert!(e.call(0x0041ffb0, &args![this, other]).bool());
            e.mem.set_u32(theirs + 4 * word, 0);
        }
        e.mem.set_u32(theirs + 0x10, 1);
        assert!(!e.call(0x0041ffb0, &args![this, other]).bool());
    }

    #[test]
    fn occlusion_plane_ref_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_b();
        let extra: Ptr<ExtraOcclusionPlaneRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420030, &args![extra, 0u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420030, &args![extra, 1u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
    }

    #[test]
    fn occlusion_plane_ref_data_destructor_frees_the_data_then_the_base() {
        let mut e = engine_b();
        let extra: Ptr<ExtraOcclusionPlaneRefData> = e.new_object();
        let data = e.mem.alloc(0x10);
        e.set(extra, ExtraOcclusionPlaneRefData::pData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x00420060, &args![extra]);
        });
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51cc);
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            order,
            vec![0x00420060, OPERATOR_DELETE, BS_EXTRA_DATA_DESTROY]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(e.get(extra, ExtraOcclusionPlaneRefData::pData).addr(), 0);
    }

    #[test]
    fn occlusion_plane_data_cleanup_frees_only_what_is_there() {
        let mut e = engine_b();
        let extra: Ptr<ExtraOcclusionPlaneRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            e.call(0x004200c0, &args![extra]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let data = e.mem.alloc(0x10);
        e.set(extra, ExtraOcclusionPlaneRefData::pData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x004200c0, &args![extra]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(e.get(extra, ExtraOcclusionPlaneRefData::pData).addr(), 0);
    }

    #[test]
    fn occlusion_plane_data_getters() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x00420100, &args![list]).u32(), 0);
        assert_eq!(e.call(0x00420130, &args![list, 2u32]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x76);
        // An extra data without data gives 0 too.
        assert_eq!(e.call(0x00420130, &args![list, 2u32]).u32(), 0);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(extra + 0x0c, data);
        e.mem.set_u32(data + 8, 0xc002);
        assert_eq!(e.call(0x00420100, &args![list]).u32(), data);
        assert_eq!(e.call(0x00420130, &args![list, 2u32]).u32(), 0xc002);
        assert_eq!(e.call(0x00420130, &args![list, 0u32]).u32(), 0);
    }

    #[test]
    fn occlusion_plane_word_setter_builds_the_data_for_a_reference() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        // A null reference with no data: nothing built, nothing stored.
        let log = logged(&mut e, |e| {
            e.call(0x00420160, &args![list, 0u32, 1u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(extra_of(&e, list, 0x76), 0);
        // A reference: the four-word data is built (0x10 bytes,
        // constructor 00411dc0), set through the setter of this file, and
        // the word stored.
        let log = logged(&mut e, |e| {
            e.call(0x00420160, &args![list, 0xc001u32, 3u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10], vec![0x10]]);
        assert_eq!(calls_to(&log, OCCLUSION_PLANE_DATA_INIT).len(), 1);
        let extra = extra_of(&e, list, 0x76);
        let data = e.mem.u32(extra + 0x0c);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data + 0x0c), 0xc001);
        // With data there: only the word is stored, a null reference
        // clears it.
        let log = logged(&mut e, |e| {
            e.call(0x00420160, &args![list, 0xc002u32, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(data), 0xc002);
        assert_eq!(e.mem.u32(data + 0x0c), 0xc001);
        e.call(0x00420160, &args![list, 0u32, 3u32]);
        assert_eq!(e.mem.u32(data + 0x0c), 0);
    }

    // 00420210: the portal reference data setter.

    #[test]
    fn portal_ref_data_setter_replaces_the_extra_data() {
        let mut e = engine_b();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x00420210, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x00420210, &args![list, 0xd001u32]);
        });
        let first = extra_of(&e, list, 0x77);
        assert_ne!(first, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(first), 0x0101_51d8);
        assert_eq!(e.mem.u32(first + 0x0c), 0xd001);
        let log = logged(&mut e, |e| {
            e.call(0x00420210, &args![list, 0xd002u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), first, 1]]
        );
        assert_eq!(e.mem.u32(extra_of(&e, list, 0x77) + 0x0c), 0xd002);
        e.call(0x00420210, &args![list, 0u32]);
        assert_eq!(extra_of(&e, list, 0x77), 0);
    }

    // Third session: 004202d0 to 004213c0.

    /// The engine of the second session with the callees of the third.
    fn engine_c() -> Engine {
        let mut e = engine_b();
        constructor_double(&mut e, EXTRA_PORTAL_INIT, 0x78);
        constructor_double(&mut e, EXTRA_ROOM_INIT, 0x79);
        constructor_double(&mut e, EXTRA_GUARDED_REF_DATA_INIT, 0x7c);
        e.register(PORTAL_LINKED_DATA_INIT, |_, a| returns(a[0]));
        e.register(ROOM_LINKED_DATA_INIT, |_, a| returns(a[0]));
        e.register(LIST_DESTROY, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        e.register(REFERENCE_EXTRA_LIST, |_, a| returns(a[0] + 0x44));
        e.register(PORTAL_SET_FIRST_ROOM, |e, a| {
            e.mem.set_u32(a[0] + 0xfc, a[1]);
            Ret::default()
        });
        e.register(GUARDED_REF_DATA_ADD_GUARD, |_, _| Ret::default());
        e.register(GUARDED_REF_DATA_TWO_WORDS, |_, _| Ret::default());
        e.register(VISITED_SET_INIT, |_, a| returns(a[0]));
        e.register(VISITED_SET_DESTROY, |_, _| Ret::default());
        // The visited set: the keys recorded, shared by the two doubles.
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(u32, u32)>::new()));
        let finder = seen.clone();
        e.register_double(VISITED_SET_FIND, move |e, a| {
            let found = finder
                .borrow()
                .iter()
                .find(|(key, _)| *key == a[1])
                .copied();
            if let Some((_, value)) = found {
                e.mem.set_u8(a[2], value as u8);
            }
            returns(found.is_some() as u32)
        });
        e.register_double(VISITED_SET_INSERT, move |_, a| {
            seen.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        e
    }

    /// A room linked data (0x14 bytes) with the given master flag whose
    /// `RoomList` holds `rooms` (the first in the head node, the others in
    /// nodes of their own).
    fn new_room_data(e: &mut Engine, master: u8, rooms: &[u32]) -> u32 {
        let data = e.mem.alloc(0x14);
        e.mem.set_u8(data + 0x10, master);
        let mut node = data + 8;
        for (position, room) in rooms.iter().enumerate() {
            e.mem.set_u32(node, *room);
            if position + 1 < rooms.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        data
    }

    /// A reference whose extra data list (at +0x44, as `005d43c0` gives it)
    /// has a room reference data holding `data`; the list's address.
    fn new_reference_with_room_data(e: &mut Engine, data: u32) -> (u32, Ptr<ExtraDataList>) {
        let reference = e.mem.alloc(0x500);
        let list = Ptr::new(reference + 0x44);
        let extra = put_extra(e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        (reference, list)
    }

    // 004202d0, 00420300, 00420380, 004203b0, 00420410: the portal
    // reference data.

    #[test]
    fn portal_ref_data_constructor() {
        let mut e = engine_c();
        let extra: Ptr<ExtraPortalRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x004202d0, &args![extra]).u32(), extra.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x77]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51d8);
    }

    #[test]
    fn portal_ref_data_compare_looks_at_two_words() {
        let mut e = engine_c();
        let this: Ptr<ExtraPortalRefData> = e.new_object();
        let other: Ptr<ExtraPortalRefData> = e.new_object();
        assert!(!e.call(0x00420300, &args![this, other]).bool());
        let mine = e.mem.alloc(0x10);
        let theirs = e.mem.alloc(0x10);
        e.set(this, ExtraPortalRefData::pData, Ptr::new(mine));
        assert!(e.call(0x00420300, &args![this, other]).bool());
        e.set(this, ExtraPortalRefData::pData, Ptr::NULL);
        e.set(other, ExtraPortalRefData::pData, Ptr::new(theirs));
        assert!(e.call(0x00420300, &args![this, other]).bool());
        e.set(this, ExtraPortalRefData::pData, Ptr::new(mine));
        assert!(!e.call(0x00420300, &args![this, other]).bool());
        for word in 0..2 {
            e.mem.set_u32(theirs + 4 * word, 1);
            assert!(e.call(0x00420300, &args![this, other]).bool());
            e.mem.set_u32(theirs + 4 * word, 0);
        }
        // The third word is not part of the comparison.
        e.mem.set_u32(theirs + 8, 1);
        assert!(!e.call(0x00420300, &args![this, other]).bool());
    }

    #[test]
    fn portal_ref_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_c();
        let extra: Ptr<ExtraPortalRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420380, &args![extra, 0u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420380, &args![extra, 1u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
    }

    #[test]
    fn portal_ref_data_destructor_frees_the_data_then_the_base() {
        let mut e = engine_c();
        let extra: Ptr<ExtraPortalRefData> = e.new_object();
        let data = e.mem.alloc(8);
        e.set(extra, ExtraPortalRefData::pData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x004203b0, &args![extra]);
        });
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51d8);
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            order,
            vec![0x004203b0, OPERATOR_DELETE, BS_EXTRA_DATA_DESTROY]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(e.get(extra, ExtraPortalRefData::pData).addr(), 0);
    }

    #[test]
    fn portal_ref_data_getter() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x00420410, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x77);
        e.mem.set_u32(extra + 0x0c, 0xd101);
        assert_eq!(e.call(0x00420410, &args![list]).u32(), 0xd101);
    }

    // 00420440, 00420500, 00420530, 00420600, 00420630, 00420690, 004206e0,
    // 00420710: the room reference data.

    #[test]
    fn room_ref_data_setter_replaces_the_extra_data() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x00420440, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x00420440, &args![list, 0xe001u32]);
        });
        let first = extra_of(&e, list, 0x7b);
        assert_ne!(first, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(first), 0x0101_51e4);
        assert_eq!(e.mem.u32(first + 0x0c), 0xe001);
        let log = logged(&mut e, |e| {
            e.call(0x00420440, &args![list, 0xe002u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), first, 1]]
        );
        assert_eq!(e.mem.u32(extra_of(&e, list, 0x7b) + 0x0c), 0xe002);
        e.call(0x00420440, &args![list, 0u32]);
        assert_eq!(extra_of(&e, list, 0x7b), 0);
    }

    #[test]
    fn room_ref_data_constructor() {
        let mut e = engine_c();
        let extra: Ptr<ExtraRoomRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420500, &args![extra]).u32(), extra.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x7b]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51e4);
    }

    #[test]
    fn room_ref_data_compare_looks_at_the_flag_and_the_room_list() {
        let mut e = engine_c();
        let this: Ptr<ExtraRoomRefData> = e.new_object();
        let other: Ptr<ExtraRoomRefData> = e.new_object();
        // Both without data: equal. One without: different.
        assert!(!e.call(0x00420530, &args![this, other]).bool());
        let mine = new_room_data(&mut e, 0, &[0x11, 0x12]);
        e.set(this, ExtraRoomRefData::pData, Ptr::new(mine));
        assert!(e.call(0x00420530, &args![this, other]).bool());
        e.set(this, ExtraRoomRefData::pData, Ptr::NULL);
        e.set(other, ExtraRoomRefData::pData, Ptr::new(mine));
        assert!(e.call(0x00420530, &args![this, other]).bool());
        // The same items in the same order: equal.
        let theirs = new_room_data(&mut e, 0, &[0x11, 0x12]);
        e.set(this, ExtraRoomRefData::pData, Ptr::new(mine));
        e.set(other, ExtraRoomRefData::pData, Ptr::new(theirs));
        assert!(!e.call(0x00420530, &args![this, other]).bool());
        // Another master flag.
        e.mem.set_u8(theirs + 0x10, 1);
        assert!(e.call(0x00420530, &args![this, other]).bool());
        // Another item, a shorter list, a longer list.
        let other_item = new_room_data(&mut e, 0, &[0x11, 0x13]);
        e.set(other, ExtraRoomRefData::pData, Ptr::new(other_item));
        assert!(e.call(0x00420530, &args![this, other]).bool());
        let shorter = new_room_data(&mut e, 0, &[0x11]);
        e.set(other, ExtraRoomRefData::pData, Ptr::new(shorter));
        assert!(e.call(0x00420530, &args![this, other]).bool());
        let longer = new_room_data(&mut e, 0, &[0x11, 0x12, 0x14]);
        e.set(other, ExtraRoomRefData::pData, Ptr::new(longer));
        assert!(e.call(0x00420530, &args![this, other]).bool());
    }

    #[test]
    fn room_ref_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_c();
        let extra: Ptr<ExtraRoomRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420600, &args![extra, 0u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420600, &args![extra, 1u32]).u32(), extra.addr());
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
    }

    #[test]
    fn room_ref_data_destructor_deletes_the_data_then_the_base() {
        let mut e = engine_c();
        let extra: Ptr<ExtraRoomRefData> = e.new_object();
        let data = e.mem.alloc(0x14);
        e.set(extra, ExtraRoomRefData::pData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x00420630, &args![extra]);
        });
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51e4);
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            order,
            vec![
                0x00420630,
                LIST_DESTROY,
                LIST_DESTROY,
                OPERATOR_DELETE,
                BS_EXTRA_DATA_DESTROY
            ]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(e.get(extra, ExtraRoomRefData::pData).addr(), 0);
    }

    #[test]
    fn room_ref_data_cleanup_deletes_only_what_is_there() {
        let mut e = engine_c();
        let extra: Ptr<ExtraRoomRefData> = e.new_object();
        let log = logged(&mut e, |e| {
            e.call(0x00420690, &args![extra]);
        });
        assert!(log.iter().all(|(callee, _)| *callee == 0x00420690));
        let data = e.mem.alloc(0x14);
        e.set(extra, ExtraRoomRefData::pData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x00420690, &args![extra]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(e.get(extra, ExtraRoomRefData::pData).addr(), 0);
    }

    #[test]
    fn room_linked_data_scalar_deleting_destructor_destroys_the_lists() {
        let mut e = engine_c();
        let data = e.mem.alloc(0x14);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x004206e0, &args![data, 0u32]).u32(), data);
        });
        // The room list first, then the portal list.
        assert_eq!(
            calls_to(&log, LIST_DESTROY),
            vec![vec![data + 8], vec![data]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x004206e0, &args![data, 3u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
    }

    #[test]
    fn room_linked_data_destructor_destroys_room_list_then_portal_list() {
        let mut e = engine_c();
        let data = e.mem.alloc(0x14);
        let log = logged(&mut e, |e| {
            e.call(0x00420710, &args![data]);
        });
        assert_eq!(
            calls_to(&log, LIST_DESTROY),
            vec![vec![data + 8], vec![data]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    // 00420770, 004207b0, 004207e0, 00420810, 00420870: the accessors of the
    // room reference data.

    #[test]
    fn room_linked_data_getters() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        for address in [0x00420770u32, 0x004207b0, 0x004207e0] {
            assert_eq!(e.call(address, &args![list]).u32(), 0);
        }
        let extra = put_extra(&mut e, list, 0x7b);
        let data = e.mem.alloc(0x14);
        e.mem.set_u32(extra + 0x0c, data);
        assert_eq!(e.call(0x004207e0, &args![list]).u32(), data);
        assert_eq!(e.call(0x004207b0, &args![list]).u32(), data);
        // The room list is the second list of the data.
        assert_eq!(e.call(0x00420770, &args![list]).u32(), data + 8);
        // An extra data without data gives 0 for both.
        e.mem.set_u32(extra + 0x0c, 0);
        assert_eq!(e.call(0x00420770, &args![list]).u32(), 0);
        assert_eq!(e.call(0x004207e0, &args![list]).u32(), 0);
    }

    #[test]
    fn room_is_master_getter() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // No room reference data: master.
        assert!(e.call(0x00420810, &args![list]).bool());
        let data = new_room_data(&mut e, 0, &[0x21]);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        // Flag clear and a non-empty room list: not master.
        assert!(!e.call(0x00420810, &args![list]).bool());
        // Flag set: master.
        e.mem.set_u8(data + 0x10, 1);
        assert!(e.call(0x00420810, &args![list]).bool());
        // Flag clear and an empty room list: master.
        let empty = new_room_data(&mut e, 0, &[]);
        e.mem.set_u32(extra + 0x0c, empty);
        assert!(e.call(0x00420810, &args![list]).bool());
    }

    #[test]
    fn room_is_master_setter() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // No room reference data: nothing happens.
        e.call(0x00420870, &args![list, true]);
        let data = e.mem.alloc(0x14);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        e.call(0x00420870, &args![list, true]);
        assert_eq!(e.mem.u8(data + 0x10), 1);
        // Only the low byte of the argument counts, and the flag is 0 or 1.
        e.call(0x00420870, &args![list, 0x0100_0000u32]);
        assert_eq!(e.mem.u8(data + 0x10), 0);
        e.call(0x00420870, &args![list, 0x80u32]);
        assert_eq!(e.mem.u8(data + 0x10), 1);
    }

    // 004208b0, 00420950: the master room search.

    #[test]
    fn master_room_is_zero_without_data_or_for_a_master() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x004208b0, &args![list]).u32(), 0);
        });
        assert!(calls_to(&log, VISITED_SET_INIT).is_empty());
        let data = new_room_data(&mut e, 1, &[0x31]);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x004208b0, &args![list]).u32(), 0);
        });
        assert!(calls_to(&log, VISITED_SET_INIT).is_empty());
    }

    #[test]
    fn master_room_searches_with_a_fresh_visited_set() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // A master room, and a room that is not (its room list is not empty).
        let master_data = new_room_data(&mut e, 1, &[]);
        let (master, _) = new_reference_with_room_data(&mut e, master_data);
        let data = new_room_data(&mut e, 0, &[master]);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x004208b0, &args![list]).u32(), master);
        });
        let init = calls_to(&log, VISITED_SET_INIT);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1], 0x25);
        assert_eq!(calls_to(&log, VISITED_SET_DESTROY), vec![vec![init[0][0]]]);
    }

    #[test]
    fn master_room_search_takes_the_first_master_of_the_room_list() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let plain_data = new_room_data(&mut e, 0, &[0x41]);
        let (plain, _) = new_reference_with_room_data(&mut e, plain_data);
        let master_data = new_room_data(&mut e, 1, &[]);
        let (master, _) = new_reference_with_room_data(&mut e, master_data);
        let data = new_room_data(&mut e, 0, &[plain, master]);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        let visited = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420950, &args![list, visited]).u32(), master);
        });
        // Found by the first pass: the visited set is not used.
        assert!(calls_to(&log, VISITED_SET_FIND).is_empty());
    }

    #[test]
    fn master_room_search_follows_unvisited_rooms_once() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // `via` is not a master; its room list leads to a master.
        let master_data = new_room_data(&mut e, 1, &[]);
        let (master, _) = new_reference_with_room_data(&mut e, master_data);
        let via_data = new_room_data(&mut e, 0, &[master]);
        let (via, _) = new_reference_with_room_data(&mut e, via_data);
        let data = new_room_data(&mut e, 0, &[via]);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        let visited = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420950, &args![list, visited]).u32(), master);
        });
        assert_eq!(
            calls_to(&log, VISITED_SET_INSERT),
            vec![vec![visited, via, 0]]
        );
        // A second search finds `via` in the set and does not follow it.
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00420950, &args![list, visited]).u32(), 0);
        });
        assert!(calls_to(&log, VISITED_SET_INSERT).is_empty());
        assert_eq!(calls_to(&log, VISITED_SET_FIND).len(), 1);
    }

    #[test]
    fn master_room_search_without_data_is_zero() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let visited = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00420950, &args![list, visited]).u32(), 0);
    }

    // 00420a60, 00420ba0, 00420bc0: the portal words.

    #[test]
    fn portal_word_setter_stores_the_word_and_the_room() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let portal = e.mem.alloc(0x200);
        let portal_extra = put_extra(&mut e, list, 0x78);
        e.mem.set_u32(portal_extra + 0x0c, portal);
        // The reference has a room of its own and no room reference data.
        let reference = e.mem.alloc(0x500);
        let room_extra = put_extra(&mut e, Ptr::new(reference + 0x44), 0x79);
        e.mem.set_u32(room_extra + 0x0c, 0x5001);
        // Index 0: the portal data is built (8 bytes), the first room set.
        let log = logged(&mut e, |e| {
            e.call(0x00420a60, &args![list, 0u32, reference]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![8], vec![0x10]]);
        assert_eq!(calls_to(&log, PORTAL_LINKED_DATA_INIT).len(), 1);
        let data = e.mem.u32(extra_of(&e, list, 0x77) + 0x0c);
        assert_eq!(e.mem.u32(data), reference);
        assert_eq!(
            calls_to(&log, PORTAL_SET_FIRST_ROOM),
            vec![vec![portal, 0x5001]]
        );
        assert_eq!(e.mem.u32(portal + 0x100), 0);
        // Index 1: the second room, at +0x100 of the portal.
        e.call(0x00420a60, &args![list, 1u32, reference]);
        assert_eq!(e.mem.u32(data + 4), reference);
        assert_eq!(e.mem.u32(portal + 0x100), 0x5001);
        // A null reference clears the word and the first room.
        let log = logged(&mut e, |e| {
            e.call(0x00420a60, &args![list, 0u32, 0u32]);
        });
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(calls_to(&log, PORTAL_SET_FIRST_ROOM), vec![vec![portal, 0]]);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn portal_word_setter_without_portal_or_data() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // A null reference with no data: nothing built, nothing stored.
        let log = logged(&mut e, |e| {
            e.call(0x00420a60, &args![list, 1u32, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(extra_of(&e, list, 0x77), 0);
        // No portal: the data is built and stored, no room handed on.
        let reference = e.mem.alloc(0x500);
        let log = logged(&mut e, |e| {
            e.call(0x00420a60, &args![list, 1u32, reference]);
        });
        assert!(calls_to(&log, PORTAL_SET_FIRST_ROOM).is_empty());
        let data = e.mem.u32(extra_of(&e, list, 0x77) + 0x0c);
        assert_eq!(e.mem.u32(data + 4), reference);
    }

    #[test]
    fn portal_word_setter_hands_on_the_room_of_the_master_room() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let portal = e.mem.alloc(0x200);
        let portal_extra = put_extra(&mut e, list, 0x78);
        e.mem.set_u32(portal_extra + 0x0c, portal);
        // The reference is not a master; its room list leads to a master
        // room, whose room is the one given to the portal.
        let master_data = new_room_data(&mut e, 1, &[]);
        let (master, master_list) = new_reference_with_room_data(&mut e, master_data);
        let master_room = put_extra(&mut e, master_list, 0x79);
        e.mem.set_u32(master_room + 0x0c, 0x5002);
        let data = new_room_data(&mut e, 0, &[master]);
        let (reference, _) = new_reference_with_room_data(&mut e, data);
        let room_extra = put_extra(&mut e, Ptr::new(reference + 0x44), 0x79);
        e.mem.set_u32(room_extra + 0x0c, 0x5001);
        let log = logged(&mut e, |e| {
            e.call(0x00420a60, &args![list, 0u32, reference]);
        });
        assert_eq!(
            calls_to(&log, PORTAL_SET_FIRST_ROOM),
            vec![vec![portal, 0x5002]]
        );
    }

    #[test]
    fn portal_second_room_setter() {
        let mut e = engine_c();
        let portal = e.mem.alloc(0x200);
        e.call(0x00420ba0, &args![portal, 0x6001u32]);
        assert_eq!(e.mem.u32(portal + 0x100), 0x6001);
        assert_eq!(e.mem.u32(portal + 0xfc), 0);
    }

    #[test]
    fn portal_word_getter() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x00420bc0, &args![list, 1u32]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x77);
        // An extra data without data gives 0 too.
        assert_eq!(e.call(0x00420bc0, &args![list, 1u32]).u32(), 0);
        let data = e.mem.alloc(8);
        e.mem.set_u32(extra + 0x0c, data);
        e.mem.set_u32(data, 0x7001);
        e.mem.set_u32(data + 4, 0x7002);
        assert_eq!(e.call(0x00420bc0, &args![list, 0u32]).u32(), 0x7001);
        assert_eq!(e.call(0x00420bc0, &args![list, 1u32]).u32(), 0x7002);
    }

    // 00420c10, 00420ce0, 00420da0: the lists of the room reference data.

    #[test]
    fn room_list_adder_builds_the_data_and_adds_once() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // A null reference with no data would run on with a null data
        // pointer (the game crashes there): not tried. A reference builds
        // the 0x14-byte data and puts the reference at the head of the room
        // list.
        let log = logged(&mut e, |e| {
            e.call(0x00420c10, &args![list, 0x8001u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14], vec![0x10]]);
        assert_eq!(calls_to(&log, ROOM_LINKED_DATA_INIT).len(), 1);
        let data = e.mem.u32(extra_of(&e, list, 0x7b) + 0x0c);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data + 8), 0x8001);
        assert_eq!(e.mem.u32(data), 0);
        // The data is there: no new one; a reference already in the list
        // is not added again.
        let log = logged(&mut e, |e| {
            e.call(0x00420c10, &args![list, 0x8001u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x00420c10, &args![list, 0x8002u32]);
        });
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
        assert_eq!(e.mem.u32(data + 8), 0x8002);
        // A null reference with data is added like any other.
        e.call(0x00420c10, &args![list, 0u32]);
        assert_eq!(e.mem.u32(data + 8), 0);
    }

    #[test]
    fn portal_list_adder_works_on_the_first_list() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x00420ce0, &args![list, 0x8101u32]);
        });
        assert_eq!(calls_to(&log, ROOM_LINKED_DATA_INIT).len(), 1);
        let data = e.mem.u32(extra_of(&e, list, 0x7b) + 0x0c);
        assert_eq!(e.mem.u32(data), 0x8101);
        assert_eq!(e.mem.u32(data + 8), 0);
        let log = logged(&mut e, |e| {
            e.call(0x00420ce0, &args![list, 0x8101u32]);
        });
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        e.call(0x00420ce0, &args![list, 0x8102u32]);
        assert_eq!(e.mem.u32(data), 0x8102);
    }

    #[test]
    fn portal_list_remover_works_on_the_first_list() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // No data: nothing is called.
        let log = logged(&mut e, |e| {
            e.call(0x00420da0, &args![list, 0x8201u32]);
        });
        assert!(calls_to(&log, LIST_REMOVE_ITEM).is_empty());
        let data = new_room_data(&mut e, 0, &[0x8301]);
        e.mem.set_u32(data, 0x8201);
        let extra = put_extra(&mut e, list, 0x7b);
        e.mem.set_u32(extra + 0x0c, data);
        let log = logged(&mut e, |e| {
            e.call(0x00420da0, &args![list, 0x8201u32]);
        });
        let calls = calls_to(&log, LIST_REMOVE_ITEM);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], data);
        assert_eq!(e.mem.u32(data), 0);
        // The room list is not touched.
        assert_eq!(e.mem.u32(data + 8), 0x8301);
    }

    // 00420dd0, 00420e00, 00420ed0, 00420f00: the portal and the room.

    #[test]
    fn portal_and_room_getters_read_the_pointer() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x00420dd0, &args![list]).u32(), 0);
        assert_eq!(e.call(0x00420ed0, &args![list]).u32(), 0);
        let portal = put_extra(&mut e, list, 0x78);
        let room = put_extra(&mut e, list, 0x79);
        e.mem.set_u32(portal + 0x0c, 0x9001);
        e.mem.set_u32(room + 0x0c, 0x9002);
        assert_eq!(e.call(0x00420dd0, &args![list]).u32(), 0x9001);
        assert_eq!(e.call(0x00420ed0, &args![list]).u32(), 0x9002);
    }

    /// The shape of `SetPortal` and `SetRoom`: `setter` for the extra data of
    /// `extra_type`.
    fn check_pointer_setter(setter: u32, extra_type: u8) {
        let mut e = engine_c();
        let list = new_list(&mut e);
        // A null value with nothing there removes by type.
        let log = logged(&mut e, |e| {
            e.call(setter, &args![list, 0u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), extra_type as u32]]
        );
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // A value builds a 0x10-byte extra data, assigns and adds it.
        let log = logged(&mut e, |e| {
            e.call(setter, &args![list, 0xa001u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = extra_of(&e, list, extra_type);
        assert_ne!(extra, 0);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![extra + 0x0c, 0xa001]]
        );
        assert_eq!(e.mem.u32(extra + 0x0c), 0xa001);
        // With one there, only the pointer is assigned.
        let log = logged(&mut e, |e| {
            e.call(setter, &args![list, 0xa002u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, ADD_EXTRA).is_empty());
        assert_eq!(extra_of(&e, list, extra_type), extra);
        assert_eq!(e.mem.u32(extra + 0x0c), 0xa002);
        // A null value removes it (by type, not through its address).
        let log = logged(&mut e, |e| {
            e.call(setter, &args![list, 0u32]);
        });
        assert_eq!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).len(), 1);
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        assert_eq!(extra_of(&e, list, extra_type), 0);
    }

    #[test]
    fn portal_setter() {
        check_pointer_setter(0x00420e00, 0x78);
    }

    #[test]
    fn room_setter() {
        check_pointer_setter(0x00420f00, 0x79);
    }

    // 00420fd0, 00421090, 004210c0, 00421130, 00421160, 004211a0: the
    // collision data.

    #[test]
    fn collision_data_setter_replaces_the_extra_data() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x00420fd0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x00420fd0, &args![list, 0xb001u32]);
        });
        let first = extra_of(&e, list, 0x72);
        assert_ne!(first, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(first), 0x0101_51f0);
        assert_eq!(e.mem.u32(first + 0x0c), 0xb001);
        let log = logged(&mut e, |e| {
            e.call(0x00420fd0, &args![list, 0xb002u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), first, 1]]
        );
        assert_eq!(e.mem.u32(extra_of(&e, list, 0x72) + 0x0c), 0xb002);
        e.call(0x00420fd0, &args![list, 0u32]);
        assert_eq!(extra_of(&e, list, 0x72), 0);
    }

    #[test]
    fn collision_data_constructor() {
        let mut e = engine_c();
        let extra: Ptr<ExtraCollisionData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00421090, &args![extra]).u32(), extra.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x72]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51f0);
    }

    #[test]
    fn collision_data_compare_looks_at_the_first_word() {
        let mut e = engine_c();
        let this: Ptr<ExtraCollisionData> = e.new_object();
        let other: Ptr<ExtraCollisionData> = e.new_object();
        assert!(!e.call(0x004210c0, &args![this, other]).bool());
        let mine = e.mem.alloc(0x10);
        let theirs = e.mem.alloc(0x10);
        e.set(this, ExtraCollisionData::pCollisionData, Ptr::new(mine));
        assert!(e.call(0x004210c0, &args![this, other]).bool());
        e.set(this, ExtraCollisionData::pCollisionData, Ptr::NULL);
        e.set(other, ExtraCollisionData::pCollisionData, Ptr::new(theirs));
        assert!(e.call(0x004210c0, &args![this, other]).bool());
        e.set(this, ExtraCollisionData::pCollisionData, Ptr::new(mine));
        // Different blocks starting with the same word are equal.
        assert!(!e.call(0x004210c0, &args![this, other]).bool());
        e.mem.set_u32(theirs, 5);
        assert!(e.call(0x004210c0, &args![this, other]).bool());
        // Only the first word counts.
        e.mem.set_u32(theirs, 0);
        e.mem.set_u32(theirs + 4, 5);
        assert!(!e.call(0x004210c0, &args![this, other]).bool());
    }

    #[test]
    fn collision_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine_c();
        let extra: Ptr<ExtraCollisionData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00421130, &args![extra, 0u32]).u32(), extra.addr());
        });
        // The destructor deletes the collision data (here null).
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0]]);
        assert_eq!(calls_to(&log, BS_EXTRA_DATA_DESTROY).len(), 1);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x00421130, &args![extra, 1u32]).u32(), extra.addr());
        });
        assert_eq!(
            calls_to(&log, OPERATOR_DELETE),
            vec![vec![0], vec![extra.addr()]]
        );
    }

    #[test]
    fn collision_data_destructor_deletes_the_data_then_the_base() {
        let mut e = engine_c();
        let extra: Ptr<ExtraCollisionData> = e.new_object();
        let data = e.mem.alloc(0x10);
        e.set(extra, ExtraCollisionData::pCollisionData, Ptr::new(data));
        let log = logged(&mut e, |e| {
            e.call(0x00421160, &args![extra]);
        });
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51f0);
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            order,
            vec![0x00421160, OPERATOR_DELETE, BS_EXTRA_DATA_DESTROY]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
    }

    #[test]
    fn collision_data_getter() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x004211a0, &args![list]).u32(), 0);
        let extra = put_extra(&mut e, list, 0x72);
        e.mem.set_u32(extra + 0x0c, 0xb101);
        assert_eq!(e.call(0x004211a0, &args![list]).u32(), 0xb101);
    }

    // 004211d0, 00421280, 004212b0, 004212e0: the actor package data.

    #[test]
    fn actor_package_data_setter_updates_or_adds() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x004211d0, &args![list, 0xc001u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = extra_of(&e, list, 0x70);
        assert_ne!(extra, 0);
        assert_eq!(e.mem.u32(extra), 0x0101_51fc);
        assert_eq!(e.mem.u32(extra + 0x0c), 0xc001);
        // With one there, the word is overwritten, a null one included.
        let log = logged(&mut e, |e| {
            e.call(0x004211d0, &args![list, 0xc002u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, ADD_EXTRA).is_empty());
        assert_eq!(e.mem.u32(extra + 0x0c), 0xc002);
        e.call(0x004211d0, &args![list, 0u32]);
        assert_eq!(extra_of(&e, list, 0x70), extra);
        assert_eq!(e.mem.u32(extra + 0x0c), 0);
        // With none, a null value still builds one.
        let other = new_list(&mut e);
        e.call(0x004211d0, &args![other, 0u32]);
        assert_ne!(extra_of(&e, other, 0x70), 0);
    }

    #[test]
    fn actor_package_data_constructor() {
        let mut e = engine_c();
        let extra: Ptr<ExtraPackageData> = e.new_object();
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x00421280, &args![extra, 0xc101u32]).u32(),
                extra.addr()
            );
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![extra.addr(), 0x70]]
        );
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_51fc);
        assert_eq!(
            e.get(extra, ExtraPackageData::pActorPackageData).addr(),
            0xc101
        );
    }

    #[test]
    fn actor_package_data_getter_and_remover() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        assert_eq!(e.call(0x004212b0, &args![list]).u32(), 0);
        // Removing with none does nothing.
        let log = logged(&mut e, |e| {
            e.call(0x004212e0, &args![list]);
        });
        assert!(calls_to(&log, REMOVE_EXTRA).is_empty());
        let extra = put_extra(&mut e, list, 0x70);
        e.mem.set_u32(extra + 0x0c, 0xc201);
        assert_eq!(e.call(0x004212b0, &args![list]).u32(), 0xc201);
        let log = logged(&mut e, |e| {
            e.call(0x004212e0, &args![list]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA),
            vec![vec![list.addr(), extra, 1]]
        );
        assert_eq!(extra_of(&e, list, 0x70), 0);
    }

    // 00421310, 004213c0: the guarded reference data.

    #[test]
    fn add_guard_builds_the_extra_data_once() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x00421310, &args![list, 0xd001u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1c]]);
        let extra = extra_of(&e, list, 0x7c);
        assert_ne!(extra, 0);
        assert_eq!(
            calls_to(&log, GUARDED_REF_DATA_ADD_GUARD),
            vec![vec![extra, 0xd001]]
        );
        let log = logged(&mut e, |e| {
            e.call(0x00421310, &args![list, 0xd002u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, ADD_EXTRA).is_empty());
        assert_eq!(
            calls_to(&log, GUARDED_REF_DATA_ADD_GUARD),
            vec![vec![extra, 0xd002]]
        );
    }

    #[test]
    fn guarded_ref_data_forwarder_needs_the_extra_data() {
        let mut e = engine_c();
        let list = new_list(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x004213c0, &args![list, 1u32, 2u32]);
        });
        assert!(calls_to(&log, GUARDED_REF_DATA_TWO_WORDS).is_empty());
        let extra = put_extra(&mut e, list, 0x7c);
        let log = logged(&mut e, |e| {
            e.call(0x004213c0, &args![list, 1u32, 2u32]);
        });
        assert_eq!(
            calls_to(&log, GUARDED_REF_DATA_TWO_WORDS),
            vec![vec![extra, 1, 2]]
        );
    }
}
