//! `NiMain/niavobject.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiAVObject` (0x9C bytes on PC: the scalar deleting destructor frees 0x9C;
//! vtable `0109b00c`) is the `NiObjectNET` with a place in the scene graph.
//! Its PC offsets, checked against the code here:
//!
//! - `m_pkParent` +0x18, `m_spCollisionObject` +0x1C (a counted pointer),
//!   `m_kWorldBound` +0x20 (a block of the memory manager, `BSNiBound *` in
//!   the Xbox PDB), `m_kPropertyList` +0x24 (an `NiTPointerList` of counted
//!   `NiProperty` pointers: head, tail, count at +0x2C), `m_uFlags` +0x30;
//! - `m_kLocal` +0x34 and `m_kWorld` +0x68 (`NiTransform`: rotation 0x24
//!   bytes, translation at +0x24, scale at +0x30; the Xbox build aligns them
//!   to 16 bytes, at +0x40 and +0x80).
//!
//! The flag bits this file names come from the viewer strings
//! `GetViewerStrings` prints next to them: bit 0 `m_bAppCulled`, 1
//! `SelUpdate`, 2 `SelUpdateTransforms`, 3 `SelUpdatePropControllers`, 4
//! `SelUpdateRigid`, 10 `No Decals`, 11 `AlwaysDraw`, 13 `Fixed Bound`.
//!
//! Counted pointers: the reference count is the word at +4,
//! `InterlockedIncrement`/`InterlockedDecrement` change it, and `DeleteThis`
//! (vtable slot 4) runs when it reaches zero.
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiAVObject, NiTArray, NiTPointerList, NiTPointerMap};

/// Import slots of `InterlockedIncrement` / `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager (`011f6238`) and its `Allocate` / `Deallocate`
/// (`__thiscall`, one word each).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `0040fbf0`: acquires a recursive lock (`this` = lock; one word, here 0).
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
/// The lock the property functions take: owner word, then the recursion
/// count at +4, which the code releases inline (the Xbox PDB's
/// `NiAVObject` has a static `BSSpinLock PropertyStateCrit`; that this is
/// it is not confirmed).
const PROPERTY_LOCK: u32 = 0x011f_42a0;
const PROPERTY_LOCK_COUNT: u32 = 0x011f_42a4;
/// The vtable `NiAVObject`'s constructor and destructor store.
const NI_AV_OBJECT_VTABLE: u32 = 0x0109_b00c;
/// The `NiRTTI` record `GetRTTI` answers; its first word is the class name.
const NI_AV_OBJECT_RTTI: u32 = 0x011f_4280;
/// The bound `GetViewerStrings` prints when `m_kWorldBound` is null (the
/// eight bytes after the `NiRTTI` record; the Xbox PDB has a static
/// `NullBoundS`, not confirmed to be this).
const NULL_BOUND: u32 = 0x011f_4288;
/// The word that holds the handle of the empty `NiFixedString`.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

/// `NiObjectNET` functions (`NiMain/niobjectnet.cpp`).
const NI_OBJECT_NET_CONSTRUCT: u32 = 0x00a5_c200;
const NI_OBJECT_NET_DESTRUCT: u32 = 0x00a5_c240;
const NI_OBJECT_NET_LOAD_BINARY: u32 = 0x00a5_c080;
const NI_OBJECT_NET_LINK_OBJECT: u32 = 0x00a5_c0e0;
const NI_OBJECT_NET_REGISTER_STREAMABLES: u32 = 0x00a5_c180;
const NI_OBJECT_NET_SAVE_BINARY: u32 = 0x00a5_c5b0;
const NI_OBJECT_NET_IS_EQUAL: u32 = 0x00a5_c820;
const NI_OBJECT_NET_PROCESS_CLONE: u32 = 0x00a5_b8f0;
const NI_OBJECT_NET_GET_VIEWER_STRINGS: u32 = 0x00a5_ca80;
/// `NiAVObject::UpdateObjectControllers` (engine map; `__thiscall`, the
/// update data and a flag word).
const UPDATE_OBJECT_CONTROLLERS: u32 = 0x004e_ff50;
/// `NiNode::UpdatePropertiesUpward` (engine map): fills a property state
/// pointer at its one argument.
const NODE_UPDATE_PROPERTIES_UPWARD: u32 = 0x00a5_e3a0;
/// `NiPropertyState::NiPropertyState` / `~NiPropertyState` (engine map), and
/// `00a89810`, which builds a property state from another (one word).
const PROPERTY_STATE_CONSTRUCT: u32 = 0x00a8_9ab0;
const PROPERTY_STATE_DESTRUCT: u32 = 0x00a8_9890;
const PROPERTY_STATE_COPY: u32 = 0x00a8_9810;
/// Size of an `NiPropertyState` (Xbox PDB 0x1C; the allocation here).
const PROPERTY_STATE_SIZE: u32 = 0x1c;
/// `0066b0d0`: `NiPointer` assignment (`this` = the pointer; releases the
/// old object, counts the new one).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `006e5cc0`: `NiPointer` assignment from another `NiPointer` (one word,
/// the address of the source).
const POINTER_ASSIGN_FROM: u32 = 0x006e_5cc0;
/// `NiTransform::MakeIdentity` and `NiTransform::operator!=`-style compare
/// (`00a86c30`, engine map `NiTransform::operator_`; nonzero when they
/// differ, as `IsEqual` uses it).
const TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
const TRANSFORM_DIFFERS: u32 = 0x00a8_6c30;
/// `004a4650`: frees every node of an `NiTPointerList` and empties it.
const LIST_REMOVE_ALL: u32 = 0x004a_4650;
/// `0043a010`: takes a node from the node allocator (called with the
/// list's count word as `this`; the engine map's name for the folded body is
/// another `AddHead`).
const LIST_NEW_NODE: u32 = 0x0043_a010;
/// `00439fd0`: `NiTPointerListBase<..., NiPointer<NiProperty>>::AddHead`
/// (engine map), the element passed by address.
const LIST_ADD_HEAD: u32 = 0x0043_9fd0;
/// `0057c5d0`: removes an element (passed by address) from a list and
/// writes the removed `NiPointer` at its first argument.
const LIST_REMOVE: u32 = 0x0057_c5d0;
/// Three-float loader / saver (`00a59be0` / `00a69780`, engine map
/// `NiColor::LoadBinary` / `SaveBinary`, used here for points).
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
/// `NiMatrix3::LoadBinary` / `SaveBinary` (engine map).
const MATRIX_LOAD_BINARY: u32 = 0x00a5_9a30;
const MATRIX_SAVE_BINARY: u32 = 0x00a5_9a90;
/// `NiStream::ReadMultipleLinkIDs` (engine map) and `00a645b0` (same unit),
/// both `__thiscall` on the stream without arguments.
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream::GetNumberOfLinkIDs` / `GetObjectFromLinkID` (engine map).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a634b0` (cdecl): creates an object from a class name.
const CREATE_OBJECT_BY_NAME: u32 = 0x00a6_34b0;
/// `00439090`: compares the point at `this` with the one at its argument
/// (`1` when `004390c0` reports them not equal).
const POINT_NOT_EQUAL: u32 = 0x0043_9090;
/// A point `LoadBinary` compares the read velocity with.
const COMPARED_POINT: u32 = 0x011f_426c;
/// `"NiCollisionSwitch"` and `"NiCollisionData"`.
const COLLISION_SWITCH_NAME: u32 = 0x0109_b0f8;
const COLLISION_DATA_NAME: u32 = 0x0109_b0e8;
/// CRT-style allocation (`00aa1070`) and free (`00aa10f0`), cdecl, and the
/// sized delete (`00aa1460`, cdecl: pointer, size).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiTArray<char *>` resize (`0096ad30`, one word: the new size) and
/// `SetAt` (`0096ae90`: index, address of the value).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// Viewer string builders (`NiMain/niviewerstrings.cpp` and friends).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const POINT_VIEWER_STRING: u32 = 0x00a5_9af0;
const MATRIX_VIEWER_STRING: u32 = 0x00a5_98f0;
const BOUND_VIEWER_STRING: u32 = 0x00a7_fb20;
/// The viewer string labels.
const TEXT_APP_CULLED: u32 = 0x0109_b204;
const TEXT_ALWAYS_DRAW: u32 = 0x0109_b1f8;
const TEXT_LOCAL_TRANSLATE: u32 = 0x0109_b1e4;
const TEXT_LOCAL_ROTATE: u32 = 0x0109_b1d4;
const TEXT_LOCAL_SCALE: u32 = 0x0109_b1c4;
const TEXT_WORLD_TRANSLATE: u32 = 0x0109_b1b0;
const TEXT_WORLD_ROTATE: u32 = 0x0109_b1a0;
const TEXT_WORLD_SCALE: u32 = 0x0109_b190;
const TEXT_WORLD_BOUND: u32 = 0x0109_b184;
const TEXT_SEL_UPDATE: u32 = 0x0109_b178;
const TEXT_SEL_UPDATE_TRANSFORMS: u32 = 0x0109_b164;
const TEXT_SEL_UPDATE_RIGID: u32 = 0x0109_b154;
const TEXT_SEL_UPDATE_PROP_CONTROLLERS: u32 = 0x0109_b138;
const TEXT_COLLISION_OBJECT: u32 = 0x0109_b124;
const TEXT_NO_DECALS: u32 = 0x0109_b118;
const TEXT_FIXED_BOUND: u32 = 0x0109_b10c;

/// Vtable slots (byte offsets; names from the Xbox PDB).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_GET_RTTI: u32 = 0x08;
/// `NiStream::SaveLinkID`.
const SLOT_SAVE_LINK_ID: u32 = 0x30;
/// `NiCullingProcess::Process`.
const SLOT_CULLING_PROCESS: u32 = 0x44;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_PROCESS_CLONE: u32 = 0x68;
/// `NiProperty::Type`.
const SLOT_PROPERTY_TYPE: u32 = 0x8c;
/// `NiCollisionObject::Initialize`.
const SLOT_COLLISION_INITIALIZE: u32 = 0x98;
/// `NiCollisionObject::Convert`.
const SLOT_COLLISION_CONVERT: u32 = 0x9c;
/// `NiTimeController::IsTransformController`.
const SLOT_IS_TRANSFORM_CONTROLLER: u32 = 0x9c;
const SLOT_UPDATE_DOWNWARD_PASS: u32 = 0xa4;
const SLOT_UPDATE_PROPERTIES_DOWNWARD: u32 = 0xb0;
const SLOT_UPDATE_WORLD_DATA: u32 = 0xb8;
const SLOT_UPDATE_WORLD_BOUND: u32 = 0xbc;
/// `NiNode::DetachChild` (the PC call passes one word).
const SLOT_NODE_DETACH_CHILD: u32 = 0xe8;
/// The renderer's slot `PurgeRendererData` calls with the property.
const SLOT_RENDERER_PURGE: u32 = 0xf4;
/// `NiNode::UpdateUpwardPass`.
const SLOT_NODE_UPDATE_UPWARD_PASS: u32 = 0xfc;
/// `NiTMapBase::KeyToHashIndex` / `IsKeysEqual`.
const SLOT_MAP_HASH_INDEX: u32 = 0x04;
const SLOT_MAP_KEYS_EQUAL: u32 = 0x08;

/// Number of property types (`GetProperty` answers null from 7 on).
const PROPERTY_TYPE_COUNT: i32 = 7;

/// Offsets inside `NiAVObject` that `crate::types::NiAVObject` does not
/// declare (PC offsets; Xbox PDB names).
const LOCAL_ROTATE: u32 = 0x34;
const LOCAL_TRANSLATE: u32 = 0x58;
const LOCAL_SCALE: u32 = 0x64;
const WORLD_ROTATE: u32 = 0x68;
const WORLD_TRANSLATE: u32 = 0x8c;
const WORLD_SCALE: u32 = 0x98;
/// `NiTimeController::m_spNext` (Xbox PDB +0x30).
const CONTROLLER_NEXT: u32 = 0x30;
/// `NiObjectNET::m_spControllers` of a property (Xbox PDB +0x0C).
const PROPERTY_CONTROLLERS: u32 = 0x0c;

/// `NiStream` fields (Xbox PDB names and offsets, the same on PC as the
/// code uses them).
/// `m_BSStreamHeader.uiVersion`.
const STREAM_BS_VERSION: u32 = 0x04;
/// `m_uiNifFileVersion`.
const STREAM_NIF_VERSION: u32 = 0xd8;
/// `m_pkIstr` / `m_pkOstr` (`NiBinaryStream *`).
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// `m_usNiAVObjectFlags`.
const STREAM_AV_OBJECT_FLAGS: u32 = 0x284;
/// A character string in the stream compared with `"NiCollisionSwitch"`
/// (the Xbox PDB's `m_acLastLoadedRTTI` is at +0x2B4; this PC offset is not
/// matched to it).
const STREAM_COMPARED_NAME: u32 = 0x3bc;
/// `NiBinaryStream` (Xbox PDB): `m_uiAbsoluteCurrentPos` +4, `m_pfnRead` +8,
/// `m_pfnWrite` +0xC. The read and write functions are cdecl: stream,
/// buffer, byte count, address of the component sizes, component count;
/// they answer the number of bytes moved.
const BINARY_STREAM_POSITION: u32 = 0x04;
const BINARY_STREAM_READ: u32 = 0x08;
const BINARY_STREAM_WRITE: u32 = 0x0c;

/// `InterlockedIncrement` of the reference count at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// `InterlockedDecrement` of the reference count at +4; `DeleteThis` runs
/// when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// Releases the inline lock at `011f42a0`: the count drops, and the owner
/// word is cleared when it reaches zero.
fn property_lock_release(e: &mut Engine) {
    let count = e.global::<u32>(PROPERTY_LOCK_COUNT).wrapping_sub(1);
    e.set_global(PROPERTY_LOCK_COUNT, count);
    if count == 0 {
        e.set_global(PROPERTY_LOCK, 0u32);
    }
}

fn flags(e: &Engine, this: Ptr<NiAVObject>) -> u32 {
    e.get(this, NiAVObject::m_uFlags)
}

fn set_flags(e: &mut Engine, this: Ptr<NiAVObject>, value: u32) {
    e.set(this, NiAVObject::m_uFlags, value);
}

/// The address of the property list.
fn property_list(this: Ptr<NiAVObject>) -> Ptr<NiTPointerList> {
    this.at(NiAVObject::m_kPropertyList)
}

fn property_count(e: &Engine, this: Ptr<NiAVObject>) -> u32 {
    e.get(property_list(this), NiTPointerList::m_uiCount)
}

/// Whether some property of the list has a controller (the loop
/// `HasPropertyController`, `UpdatePropertiesDownward` and
/// `SetSelectiveUpdateFlags` share).
fn any_property_controller(e: &Engine, this: Ptr<NiAVObject>) -> bool {
    let mut node = e.get(property_list(this), NiTPointerList::m_pkHead);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if property != 0 && e.mem.u32(property + PROPERTY_CONTROLLERS) != 0 {
            return true;
        }
    }
    false
}

/// Whether one of the controllers is a transform controller (the loop of
/// `HasTransformController` and `SetSelectiveUpdateFlags`).
fn any_transform_controller(e: &mut Engine, this: Ptr<NiAVObject>) -> bool {
    let mut controller = e.get(this, NiAVObject::m_spControllers);
    while controller != 0 {
        if e.vcall(controller, SLOT_IS_TRANSFORM_CONTROLLER, &args![])
            .bool()
        {
            return true;
        }
        controller = e.mem.u32(controller + CONTROLLER_NEXT);
    }
    false
}

// Translated from 00a59cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::PreAttachUpdate` (Xbox PDB): sets the parent for the
/// duration of an `UpdateDownwardPass(update, 0)` and clears it again.
pub fn ni_av_object_pre_attach_update(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    parent: Ptr,
    update: Ptr,
) {
    e.set(this, NiAVObject::m_pkParent, parent.addr());
    e.vcall(this.addr(), SLOT_UPDATE_DOWNWARD_PASS, &args![update, 0u32]);
    e.set(this, NiAVObject::m_pkParent, 0);
}

// Translated from 00a59cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::PostAttachUpdate` (Xbox PDB): the parent's
/// `UpdateUpwardPass` (a tail call; the parent is not checked for null).
pub fn ni_av_object_post_attach_update(e: &mut Engine, this: Ptr<NiAVObject>) {
    let parent = e.get(this, NiAVObject::m_pkParent);
    e.vcall(parent, SLOT_NODE_UPDATE_UPWARD_PASS, &args![]);
}

// Translated from 00a59d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::AttachParent` (Xbox PDB): detaches the object from its old
/// parent (`DetachChild`, slot 0xE8) and stores the new one.
pub fn ni_av_object_attach_parent(e: &mut Engine, this: Ptr<NiAVObject>, parent: Ptr) {
    let old = e.get(this, NiAVObject::m_pkParent);
    if old != 0 {
        e.vcall(old, SLOT_NODE_DETACH_CHILD, &args![this]);
    }
    e.set(this, NiAVObject::m_pkParent, parent.addr());
}

// Translated from 00a59d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::GetProperty` (Xbox PDB): the first property of the list
/// whose `Type()` is `kind`; null for a kind of 7 or more (signed).
pub fn ni_av_object_get_property(e: &mut Engine, this: Ptr<NiAVObject>, kind: i32) -> Ptr {
    if kind >= PROPERTY_TYPE_COUNT {
        return Ptr::NULL;
    }
    let mut node = e.get(property_list(this), NiTPointerList::m_pkHead);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if property != 0 && e.vcall(property, SLOT_PROPERTY_TYPE, &args![]).i32() == kind {
            return Ptr::new(property);
        }
    }
    Ptr::NULL
}

// Translated from 00a59d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateTransformAndBounds` (Xbox PDB): with flag bit 2
/// (`SelUpdateTransforms`) set, `UpdateWorldData(update)` then
/// `UpdateWorldBound()`.
pub fn ni_av_object_update_transform_and_bounds(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    update: Ptr,
) {
    if (flags(e, this) >> 2) & 1 != 0 {
        e.vcall(this.addr(), SLOT_UPDATE_WORLD_DATA, &args![update]);
        e.vcall(this.addr(), SLOT_UPDATE_WORLD_BOUND, &args![]);
    }
}

// Translated from 00a59db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::PreAttachUpdateEffects` (Xbox PDB): clears the parent; the
/// one argument word is not read.
pub fn ni_av_object_pre_attach_update_effects(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    _unused_1: u32,
) {
    e.set(this, NiAVObject::m_pkParent, 0);
}

// Translated from 00a59dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::SetBound` (Xbox PDB): frees the old world bound block,
/// stores the new one and sets flag bit 19 when it is not null (clears it
/// otherwise).
pub fn ni_av_object_set_bound(e: &mut Engine, this: Ptr<NiAVObject>, bound: Ptr) {
    let old = e.get(this, NiAVObject::m_kWorldBound);
    if old != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, old]);
    }
    e.set(this, NiAVObject::m_kWorldBound, bound.addr());
    let value = flags(e, this);
    if bound.is_null() {
        set_flags(e, this, value & 0xfff7_ffff);
    } else {
        set_flags(e, this, value | 0x0008_0000);
    }
}

// Translated from 00a59e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::Cull` (Xbox PDB): unless the object is app-culled (flag bit
/// 0), hands it to the culling process (`Process`, slot 0x44).
pub fn ni_av_object_cull(e: &mut Engine, this: Ptr<NiAVObject>, culler: Ptr) {
    if flags(e, this) & 1 == 0 {
        e.vcall(culler.addr(), SLOT_CULLING_PROCESS, &args![this]);
    }
}

// Translated from 00a59e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::GetObjectByName` (Xbox PDB): the object itself when its
/// name handle is the one at `name`; null for the empty name.
pub fn ni_av_object_get_object_by_name(e: &mut Engine, this: Ptr<NiAVObject>, name: Ptr) -> Ptr {
    let handle = e.mem.u32(name.addr());
    if handle == e.global::<u32>(EMPTY_FIXED_STRING) {
        return Ptr::NULL;
    }
    if e.get(this, NiAVObject::m_kName) == handle {
        this.cast()
    } else {
        Ptr::NULL
    }
}

// Translated from 00a59e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::PurgeRendererData` (Xbox PDB): passes the property of type
/// 2 to the renderer's slot 0xF4 when the object has one.
pub fn ni_av_object_purge_renderer_data(e: &mut Engine, this: Ptr<NiAVObject>, renderer: Ptr) {
    let property = ni_av_object_get_property(e, this, 2);
    if !property.is_null() {
        e.vcall(renderer.addr(), SLOT_RENDERER_PURGE, &args![property]);
    }
}

// Translated from 00a59e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::ProcessPropertyClones` (Xbox PDB): walks the list from the
/// tail to the head and calls `ProcessClone(cloning)` on every property the
/// cloning process's process map (`+4`) does not hold yet (`GetAt`, whose
/// found value goes to the argument word of the list). `this` is not read,
/// nor are the properties checked for null.
pub fn ni_av_object_process_property_clones(
    e: &mut Engine,
    _this: Ptr<NiAVObject>,
    list: Ptr<NiTPointerList>,
    cloning: Ptr,
) {
    e.with_stack(4, |e, found_value| {
        e.mem.set_u32(found_value.addr(), list.addr());
        let mut node = e.get(list, NiTPointerList::m_pkTail);
        while node != 0 {
            let property = e.mem.u32(node + 8);
            node = e.mem.u32(node + 4);
            let process_map: Ptr<NiTPointerMap> = Ptr::new(e.mem.u32(cloning.addr() + 4));
            if !ni_t_map_base_get_at(e, process_map, property, found_value) {
                e.vcall(property, SLOT_PROCESS_CLONE, &args![cloning]);
            }
        }
    });
}

// Translated from 00a59ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::ProcessClone` (Xbox PDB): `NiObjectNET::ProcessClone`, then
/// the properties (when there are any) and the collision object.
pub fn ni_av_object_process_clone(e: &mut Engine, this: Ptr<NiAVObject>, cloning: Ptr) {
    e.call(NI_OBJECT_NET_PROCESS_CLONE, &args![this, cloning]);
    if property_count(e, this) != 0 {
        ni_av_object_process_property_clones(e, this, property_list(this), cloning);
    }
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        e.vcall(collision, SLOT_PROCESS_CLONE, &args![cloning]);
    }
}

// Translated from 00a59f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::RegisterStreamables` (Xbox PDB): false when
/// `NiObjectNET::RegisterStreamables` is; otherwise registers every
/// property and the collision object and answers true.
pub fn ni_av_object_register_streamables(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    stream: Ptr,
) -> bool {
    if !e
        .call(NI_OBJECT_NET_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let mut node = e.get(property_list(this), NiTPointerList::m_pkHead);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if property != 0 {
            e.vcall(property, SLOT_REGISTER_STREAMABLES, &args![stream]);
        }
    }
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        e.vcall(collision, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a59f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a property in a table of property pointers indexed by type
/// (`this` + 4 * `Type()`, through the `NiPointer` assignment `0066b0d0`),
/// when the property is not null and its type is 7 or less (signed).
pub fn fn_00a59f50(e: &mut Engine, this: Ptr, property: Ptr) {
    if property.is_null() {
        return;
    }
    let kind = e.vcall(property.addr(), SLOT_PROPERTY_TYPE, &args![]).i32();
    if kind > 7 {
        return;
    }
    let kind = e.vcall(property.addr(), SLOT_PROPERTY_TYPE, &args![]).u32();
    let slot = this.addr().wrapping_add(kind.wrapping_mul(4));
    e.call(POINTER_ASSIGN, &args![slot, property]);
}

// Translated from 00a59f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateDownwardPass` (Xbox PDB): the controllers when the
/// update data's byte at +4 is set, then `UpdateWorldData(update)` and
/// `UpdateWorldBound()`. The second argument word is not read.
pub fn ni_av_object_update_downward_pass(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    update: Ptr,
    _unused_2: u32,
) {
    if e.mem.u8(update.addr() + 4) != 0 {
        e.call(UPDATE_OBJECT_CONTROLLERS, &args![this, update, 1u32]);
    }
    e.vcall(this.addr(), SLOT_UPDATE_WORLD_DATA, &args![update]);
    e.vcall(this.addr(), SLOT_UPDATE_WORLD_BOUND, &args![]);
}

// Translated from 00a59fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of `NiAVObject`'s vtable slots 0xA8 and 0xAC (Xbox PDB
/// `UpdateSelectedDownwardPass` / `UpdateRigidDownwardPass`): updates the
/// controllers with the update data's byte at +4 temporarily set to flag
/// bit 3 (`SelUpdatePropControllers`), then, with flag bit 2
/// (`SelUpdateTransforms`) set and the byte at +5 clear,
/// `UpdateWorldData(update)` and `UpdateWorldBound()`. The second argument
/// word is not read.
pub fn fn_00a59fd0(e: &mut Engine, this: Ptr<NiAVObject>, update: Ptr, _unused_2: u32) {
    let saved = e.mem.u8(update.addr() + 4);
    let value = flags(e, this);
    e.mem.set_u8(update.addr() + 4, ((value >> 3) & 1) as u8);
    e.call(UPDATE_OBJECT_CONTROLLERS, &args![this, update, 1u32]);
    e.mem.set_u8(update.addr() + 4, saved);
    if (flags(e, this) >> 2) & 1 != 0 && e.mem.u8(update.addr() + 5) == 0 {
        e.vcall(this.addr(), SLOT_UPDATE_WORLD_DATA, &args![update]);
        e.vcall(this.addr(), SLOT_UPDATE_WORLD_BOUND, &args![]);
    }
}

// Translated from 00a5a030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateControllers` (Xbox PDB):
/// `UpdateObjectControllers(update, true)`.
pub fn ni_av_object_update_controllers(e: &mut Engine, this: Ptr<NiAVObject>, update: Ptr) {
    e.call(UPDATE_OBJECT_CONTROLLERS, &args![this, update, 1u32]);
}

// Translated from 00a5a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateProperties` (Xbox PDB): under the property lock,
/// gets a property state (the parent's `UpdatePropertiesUpward`, or a new
/// `NiPropertyState` without a parent), passes it to
/// `UpdatePropertiesDownward`, then destroys and frees it.
pub fn ni_av_object_update_properties(e: &mut Engine, this: Ptr<NiAVObject>) {
    e.with_stack(4, |e, state_slot| {
        e.call(LOCK_ACQUIRE, &args![PROPERTY_LOCK, 0u32]);
        let parent = e.get(this, NiAVObject::m_pkParent);
        if parent != 0 {
            e.call(NODE_UPDATE_PROPERTIES_UPWARD, &args![parent, state_slot]);
        } else {
            let block = e
                .call(
                    MEMORY_MANAGER_ALLOCATE,
                    &args![MEMORY_MANAGER, PROPERTY_STATE_SIZE],
                )
                .u32();
            let state = if block != 0 {
                e.call(PROPERTY_STATE_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(state_slot.addr(), state);
        }
        let state = e.mem.u32(state_slot.addr());
        e.vcall(this.addr(), SLOT_UPDATE_PROPERTIES_DOWNWARD, &args![state]);
        let state = e.mem.u32(state_slot.addr());
        if state != 0 {
            e.call(PROPERTY_STATE_DESTRUCT, &args![state]);
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, state]);
        }
        property_lock_release(e);
    });
}

// Translated from 00a5a0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::HasTransformController` (Xbox PDB): whether one of the
/// controllers answers `IsTransformController`.
pub fn ni_av_object_has_transform_controller(e: &mut Engine, this: Ptr<NiAVObject>) -> bool {
    any_transform_controller(e, this)
}

// Translated from 00a5a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::HasPropertyController` (Xbox PDB): whether a property of
/// the list has a controller.
pub fn ni_av_object_has_property_controller(e: &mut Engine, this: Ptr<NiAVObject>) -> bool {
    any_property_controller(e, this)
}

// Translated from 00a5a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdatePropertiesDownward` (Xbox PDB): sets flag bit 18 when
/// a property has a controller, clears it otherwise. The property state
/// argument is not read.
pub fn ni_av_object_update_properties_downward(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    _unused_1: u32,
) {
    let value = flags(e, this);
    if any_property_controller(e, this) {
        set_flags(e, this, value | 0x0004_0000);
    } else {
        set_flags(e, this, value & 0xfffb_ffff);
    }
}

// Translated from 00a5a170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::PushLocalProperties` (Xbox PDB): without properties, writes
/// `parent_state` at `result`. Otherwise, under the property lock, takes a
/// copy of the parent state (`00a89810` on a new 0x1C-byte block) when
/// `copy` is set, or the parent state itself, and puts each property of
/// type 7 or less (signed) into its slot (`state + 4 * type`, a counted
/// pointer), then writes the state at `result`. Answers `result`.
pub fn ni_av_object_push_local_properties(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    parent_state: Ptr,
    copy: bool,
    result: Ptr,
) -> Ptr {
    if property_count(e, this) == 0 {
        e.mem.set_u32(result.addr(), parent_state.addr());
        return result;
    }
    e.call(LOCK_ACQUIRE, &args![PROPERTY_LOCK, 0u32]);
    let state = if copy {
        let block = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![MEMORY_MANAGER, PROPERTY_STATE_SIZE],
            )
            .u32();
        if block != 0 {
            e.call(PROPERTY_STATE_COPY, &args![block, parent_state])
                .u32()
        } else {
            0
        }
    } else {
        parent_state.addr()
    };
    let mut node = e.get(property_list(this), NiTPointerList::m_pkHead);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if property == 0 || e.vcall(property, SLOT_PROPERTY_TYPE, &args![]).i32() > 7 {
            continue;
        }
        let kind = e.vcall(property, SLOT_PROPERTY_TYPE, &args![]).u32();
        let slot = state.wrapping_add(kind.wrapping_mul(4));
        let old = e.mem.u32(slot);
        if old == property {
            continue;
        }
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, property);
        add_ref(e, property);
    }
    property_lock_release(e);
    e.mem.set_u32(result.addr(), state);
    result
}

// Translated from 00a5a260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::IsEqual` (Xbox PDB): `NiObjectNET::IsEqual`, the same
/// app-culled bit, the same local transform, the same number of properties,
/// each pair of properties equal (both null or `IsEqual`; the other list's
/// nodes are not checked for null), and the collision objects equal (both
/// present and `IsEqual`, or the same pointer).
pub fn ni_av_object_is_equal(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    other: Ptr<NiAVObject>,
) -> bool {
    if !e.call(NI_OBJECT_NET_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if (e.mem.u8(other.addr() + 0x30) ^ e.mem.u8(this.addr() + 0x30)) & 1 != 0 {
        return false;
    }
    if e.call(
        TRANSFORM_DIFFERS,
        &args![this.addr() + LOCAL_ROTATE, other.addr() + LOCAL_ROTATE],
    )
    .bool()
    {
        return false;
    }
    let count = property_count(e, this);
    if count != property_count(e, other) {
        return false;
    }
    if count != 0 {
        let mut mine = e.get(property_list(this), NiTPointerList::m_pkHead);
        let mut theirs = e.get(property_list(other), NiTPointerList::m_pkHead);
        while mine != 0 {
            let a = e.mem.u32(mine + 8);
            mine = e.mem.u32(mine);
            let b = e.mem.u32(theirs + 8);
            theirs = e.mem.u32(theirs);
            if a == 0 {
                if b != 0 {
                    return false;
                }
            } else if b == 0 || !e.vcall(a, SLOT_IS_EQUAL, &args![b]).bool() {
                return false;
            }
        }
    }
    let a = e.get(this, NiAVObject::m_spCollisionObject);
    let b = e.get(other, NiAVObject::m_spCollisionObject);
    if a != 0 && b != 0 {
        e.vcall(a, SLOT_IS_EQUAL, &args![b]).bool()
    } else {
        a == b
    }
}

// Translated from 00a5a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the counted pointer at `pointer` to the end of an
/// `NiTPointerList` of counted pointers: a node from `0043a010` gets the
/// object (the node's old element released, the new one counted, unless
/// they are the same), and is linked after the tail; the count grows.
pub fn fn_00a5a310(e: &mut Engine, list: Ptr<NiTPointerList>, pointer: Ptr) {
    let count_word = list.addr() + 8;
    let node = e.call(LIST_NEW_NODE, &args![count_word]).u32();
    let old = e.mem.u32(node + 8);
    let new = e.mem.u32(pointer.addr());
    if old != new {
        if old != 0 {
            release(e, old);
        }
        let new = e.mem.u32(pointer.addr());
        e.mem.set_u32(node + 8, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
    e.mem.set_u32(node, 0);
    let tail = e.get(list, NiTPointerList::m_pkTail);
    e.mem.set_u32(node + 4, tail);
    let tail = e.get(list, NiTPointerList::m_pkTail);
    if tail != 0 {
        e.mem.set_u32(tail, node);
    }
    let count = e.mem.u32(count_word).wrapping_add(1);
    e.mem.set_u32(count_word, count);
    if tail == 0 {
        e.set(list, NiTPointerList::m_pkHead, node);
    }
    e.set(list, NiTPointerList::m_pkTail, node);
}

// Translated from 00a5a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiAVObject` destructor body: frees the world bound block, empties
/// the property list and drops the collision object (both done twice, as
/// the code does; the second release finds the pointer already null), then
/// runs the `NiObjectNET` destructor.
pub fn fn_00a5a390(e: &mut Engine, this: Ptr<NiAVObject>) {
    e.mem.set_u32(this.addr(), NI_AV_OBJECT_VTABLE);
    let bound = e.get(this, NiAVObject::m_kWorldBound);
    if bound != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, bound]);
    }
    let value = flags(e, this);
    set_flags(e, this, value & 0xfff7_ffff);
    e.set(this, NiAVObject::m_kWorldBound, 0);
    e.call(LIST_REMOVE_ALL, &args![property_list(this)]);
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        release(e, collision);
        e.set(this, NiAVObject::m_spCollisionObject, 0);
    }
    e.call(LIST_REMOVE_ALL, &args![property_list(this)]);
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        release(e, collision);
    }
    e.call(NI_OBJECT_NET_DESTRUCT, &args![this]);
}

// Translated from 00a5a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::GetRTTI` (Xbox PDB): the class's `NiRTTI` record.
pub fn ni_av_object_get_rtti(_e: &mut Engine, _this: Ptr<NiAVObject>) -> Ptr {
    Ptr::new(NI_AV_OBJECT_RTTI)
}

// Translated from 00a5a430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::PreAttachUpdateProperties` (Xbox PDB): sets the parent for
/// the duration of `UpdateProperties` and clears it again.
pub fn ni_av_object_pre_attach_update_properties(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    parent: Ptr,
) {
    e.set(this, NiAVObject::m_pkParent, parent.addr());
    ni_av_object_update_properties(e, this);
    e.set(this, NiAVObject::m_pkParent, 0);
}

// Translated from 00a5a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::SetSelectiveUpdateFlags` (Xbox PDB). The object is rigid
/// (written at `rigid`) when no controller is a transform controller and
/// flag bit 7 is clear. Transforms need updating when
/// `selective_update_transforms` is set or the object is not rigid; it
/// needs a selective update (written at `selective_update`, flag bit 1)
/// when that holds, a property has a controller, or it has controllers.
/// Flag bit 2 is the transforms answer, bit 3 the property controller
/// answer, bit 4 the rigid answer as the code combines them.
pub fn ni_av_object_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    selective_update: Ptr,
    selective_update_transforms: bool,
    rigid: Ptr,
) {
    let is_rigid = !any_transform_controller(e, this) && (flags(e, this) >> 7) & 1 == 0;
    e.mem.set_u8(rigid.addr(), is_rigid as u8);
    let transforms = selective_update_transforms || !is_rigid;
    let property_controllers = any_property_controller(e, this);
    if property_controllers || transforms || e.get(this, NiAVObject::m_spControllers) != 0 {
        e.mem.set_u8(selective_update.addr(), 1);
        let value = flags(e, this) | 2;
        set_flags(e, this, value);
        if transforms {
            set_flags(e, this, value | 4);
        } else {
            set_flags(e, this, value & !4);
        }
    } else {
        e.mem.set_u8(selective_update.addr(), 0);
        let value = flags(e, this) & !2;
        set_flags(e, this, value);
        set_flags(e, this, value & !4);
    }
    let value = flags(e, this);
    if property_controllers {
        set_flags(e, this, value | 0x18);
    } else {
        set_flags(e, this, value & !8);
        let value = flags(e, this);
        set_flags(e, this, value | 0x10);
    }
}

/// Byte-wise C string compare as the code inlines it: whether the strings
/// at `a` and `b` are equal.
fn strings_equal(e: &Engine, a: u32, b: u32) -> bool {
    let mut i = 0u32;
    loop {
        let x = e.mem.u8(a.wrapping_add(i));
        if x != e.mem.u8(b.wrapping_add(i)) {
            return false;
        }
        if x == 0 {
            return true;
        }
        i = i.wrapping_add(1);
    }
}

/// One read through the stream's `NiBinaryStream` (`m_pkIstr`): `count`
/// bytes into `buffer` with one component of `size` bytes (written into
/// `size_slot` first), and the position advanced by what it answers.
fn stream_read(e: &mut Engine, stream: Ptr, buffer: u32, count: u32, size_slot: u32, size: u32) {
    let reader = e.mem.u32(stream.addr() + STREAM_INPUT);
    let read = e.mem.u32(reader + BINARY_STREAM_READ);
    e.mem.set_u32(size_slot, size);
    let moved = e
        .call(read, &args![reader, buffer, count, size_slot, 1u32])
        .u32();
    let position = e
        .mem
        .u32(reader + BINARY_STREAM_POSITION)
        .wrapping_add(moved);
    e.mem.set_u32(reader + BINARY_STREAM_POSITION, position);
}

/// The write counterpart through `m_pkOstr`.
fn stream_write(e: &mut Engine, stream: Ptr, buffer: u32, count: u32, size_slot: u32, size: u32) {
    let writer = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(writer + BINARY_STREAM_WRITE);
    e.mem.set_u32(size_slot, size);
    let moved = e
        .call(write, &args![writer, buffer, count, size_slot, 1u32])
        .u32();
    let position = e
        .mem
        .u32(writer + BINARY_STREAM_POSITION)
        .wrapping_add(moved);
    e.mem.set_u32(writer + BINARY_STREAM_POSITION, position);
}

fn nif_version(e: &Engine, stream: Ptr) -> u32 {
    e.mem.u32(stream.addr() + STREAM_NIF_VERSION)
}

// Translated from 00a5a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::LoadBinary` (Xbox PDB). After `NiObjectNET::LoadBinary`:
/// the flags (two bytes before Bethesda stream version 0x1C, four after),
/// converted for old NIF versions (before 4.1.0.11, 4.1.0.12 and 5.0.0.1
/// the bits move up); the local translation, rotation and scale. Before
/// NIF 5.0.0.19 the old layout follows: a velocity (three floats), the
/// property link IDs, and a bound flag (a word before 4.1.0.0, a byte
/// after); an `"NiCollisionData"` object is then made and initialized
/// (`Initialize`, slot 0x98, with this object, the stream, the flag and a
/// mode word: the old flags' bits 1..3, or 4 for an `"NiCollisionSwitch"`
/// stream without flag bit 3) unless the flag is clear, the velocity
/// equals the point at `011f426c` and the mode is 2. From 5.0.0.19 on, the
/// property and collision link IDs are read. Before 10.0.1.2 the flags go
/// to the stream's `m_usNiAVObjectFlags` and keep only bits 0..5; before
/// 10.0.1.6 the collision object is converted (`Convert(version, 0)`,
/// slot 0x9C); before 20.0.0.4 bit 6 is cleared. Bit 4 is cleared and bits
/// 1, 2, 3 and 19 set at the end.
pub fn ni_av_object_load_binary(e: &mut Engine, this: Ptr<NiAVObject>, stream: Ptr) {
    // The game's frame: locals at +0x10..+0x34 (here +0x00..+0x24) and the
    // argument word, which the code reuses as a buffer (here +0x24).
    e.with_stack(0x28, |e, frame| {
        let base = frame.addr();
        let mode = base;
        let size_slot = base + 0x04;
        let bound_word = base + 0x08;
        let velocity = base + 0x0c;
        let initialize_data = base + 0x18;
        let argument = base + 0x24;
        e.mem.set_u32(argument, stream.addr());

        e.call(NI_OBJECT_NET_LOAD_BINARY, &args![this, stream]);
        let flags_addr = this.addr() + 0x30;
        if e.mem.u32(stream.addr() + STREAM_BS_VERSION) < 0x1c {
            stream_read(e, stream, argument, 2, mode, 2);
            let value = e.mem.u16(argument) as u32;
            e.mem.set_u32(flags_addr, value);
        } else {
            stream_read(e, stream, flags_addr, 4, argument, 4);
        }
        if nif_version(e, stream) < 0x0401_000b {
            let low = e.mem.u16(flags_addr) as u32;
            let value = (e.mem.u32(flags_addr) & 0xffff_fff8)
                .wrapping_add(e.mem.u32(flags_addr) & 0xffff_fff8);
            e.mem.set_u32(flags_addr, value | (low & 7));
        }
        if nif_version(e, stream) < 0x0401_000c {
            let low = e.mem.u16(flags_addr) as u32;
            let shifted = (e.mem.u32(flags_addr) << 4) & 0xffff_ff00 & 0xffff_ff7f;
            e.mem.set_u32(flags_addr, (low & 0xf) | shifted | 0x70);
        }
        if nif_version(e, stream) < 0x0500_0001 {
            let low = e.mem.u16(flags_addr) as u32 & 0xff;
            let high = e.mem.u32(flags_addr) & 0xffff_ff00;
            e.mem.set_u32(flags_addr, high.wrapping_add(high) | low);
        }
        e.call(
            POINT_LOAD_BINARY,
            &args![this.addr() + LOCAL_TRANSLATE, stream],
        );
        e.call(
            MATRIX_LOAD_BINARY,
            &args![this.addr() + LOCAL_ROTATE, stream],
        );
        stream_read(e, stream, this.addr() + LOCAL_SCALE, 4, argument, 4);

        if nif_version(e, stream) >= 0x0500_0013 {
            e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
            e.call(STREAM_READ_LINK_ID, &args![stream]);
        } else {
            let value = e.mem.u32(flags_addr);
            for i in 0..3 {
                e.mem.set_f32(velocity + 4 * i, 0.0);
            }
            e.mem.set_u32(mode, (value >> 1) & 7);
            e.mem
                .set_u32(flags_addr, ((value >> 3) & 0xfffe) | (value & 1));
            e.call(POINT_LOAD_BINARY, &args![velocity, stream]);
            e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
            if nif_version(e, stream) < 0x0401_0000 {
                stream_read(e, stream, bound_word, 4, size_slot, 4);
                let present = e.mem.u32(bound_word) != 0;
                e.mem.set_u8(argument, present as u8);
            } else {
                stream_read(e, stream, argument, 1, size_slot, 1);
            }
            if strings_equal(
                e,
                stream.addr() + STREAM_COMPARED_NAME,
                COLLISION_SWITCH_NAME,
            ) && (e.mem.u32(flags_addr) >> 3) & 1 == 0
            {
                e.mem.set_u32(mode, 4);
            }
            let create = e.mem.u8(argument) != 0
                || e.call(POINT_NOT_EQUAL, &args![velocity, COMPARED_POINT])
                    .bool()
                || e.mem.u16(mode) != 2;
            if create {
                let object = e
                    .call(CREATE_OBJECT_BY_NAME, &args![COLLISION_DATA_NAME])
                    .u32();
                if object != 0 {
                    let collision_slot = this.addr() + 0x1c;
                    e.call(POINTER_ASSIGN, &args![collision_slot, object]);
                    let present = e.mem.u8(argument) != 0;
                    let mode_value = e.mem.u16(mode);
                    e.mem.set_u16(initialize_data + 0x0a, mode_value);
                    let collision = e.mem.u32(collision_slot);
                    e.mem.set_u8(initialize_data + 0x08, present as u8);
                    e.mem.set_u32(initialize_data, this.addr());
                    e.mem.set_u32(initialize_data + 0x04, stream.addr());
                    e.vcall(
                        collision,
                        SLOT_COLLISION_INITIALIZE,
                        &args![initialize_data],
                    );
                }
            }
        }

        if nif_version(e, stream) < 0x0a00_0102 {
            let value = e.mem.u16(flags_addr);
            e.mem.set_u16(stream.addr() + STREAM_AV_OBJECT_FLAGS, value);
            let value = e.mem.u32(flags_addr) & 0x3f;
            e.mem.set_u32(flags_addr, value);
        }
        let version = nif_version(e, stream);
        if version < 0x0a00_0106 {
            let collision = e.get(this, NiAVObject::m_spCollisionObject);
            if collision != 0 {
                e.vcall(collision, SLOT_COLLISION_CONVERT, &args![version, 0u32]);
            }
        }
        if nif_version(e, stream) < 0x1400_0004 {
            let value = e.mem.u32(flags_addr) & 0xffff_ffbf;
            e.mem.set_u32(flags_addr, value);
        }
        let value = e.mem.u32(flags_addr);
        e.mem
            .set_u32(flags_addr, (value & 0xffff_ffef) | 0x0008_000e);
    });
}

// Translated from 00a5a7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::LinkObject` (Xbox PDB): after `NiObjectNET::LinkObject`,
/// adds one property per link ID to the list (`00a5a310` at the tail before
/// NIF 4.1.0.8, `AddHead` after; each object counted while it is passed in
/// the argument word). Before 4.1.0.8 it stops there; otherwise, from
/// 5.0.0.19 on, the next link ID is the collision object, which is then
/// converted (`Convert(version, 0)`, slot 0x9C) when present.
pub fn ni_av_object_link_object(e: &mut Engine, this: Ptr<NiAVObject>, stream: Ptr) {
    e.call(NI_OBJECT_NET_LINK_OBJECT, &args![this, stream]);
    let mut count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let old_layout = nif_version(e, stream) < 0x0401_0008;
    e.with_stack(4, |e, pointer| {
        e.mem.set_u32(pointer.addr(), stream.addr());
        while count != 0 {
            count -= 1;
            let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(pointer.addr(), object);
            if object != 0 {
                add_ref(e, object);
            }
            if old_layout {
                fn_00a5a310(e, property_list(this), pointer);
            } else {
                e.call(LIST_ADD_HEAD, &args![property_list(this), pointer]);
            }
            if object != 0 {
                release(e, object);
            }
        }
    });
    if old_layout || nif_version(e, stream) < 0x0500_0013 {
        return;
    }
    let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let collision_slot = this.addr() + 0x1c;
    e.call(POINTER_ASSIGN, &args![collision_slot, object]);
    let collision = e.mem.u32(collision_slot);
    if collision != 0 {
        let version = nif_version(e, stream);
        e.vcall(collision, SLOT_COLLISION_CONVERT, &args![version, 0u32]);
    }
}

// Translated from 00a5a900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::SaveBinary` (Xbox PDB): after `NiObjectNET::SaveBinary`,
/// writes the flags, the local translation, rotation and scale and the
/// number of properties, then the properties' link IDs from the last to the
/// first (copied into a temporary array first), then the collision object's.
pub fn ni_av_object_save_binary(e: &mut Engine, this: Ptr<NiAVObject>, stream: Ptr) {
    // The game's local word (+0) and argument word (+4).
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = frame.addr() + 4;
        e.mem.set_u32(argument, stream.addr());
        e.call(NI_OBJECT_NET_SAVE_BINARY, &args![this, stream]);
        stream_write(e, stream, this.addr() + 0x30, 4, local, 4);
        e.call(
            POINT_SAVE_BINARY,
            &args![this.addr() + LOCAL_TRANSLATE, stream],
        );
        e.call(
            MATRIX_SAVE_BINARY,
            &args![this.addr() + LOCAL_ROTATE, stream],
        );
        stream_write(e, stream, this.addr() + LOCAL_SCALE, 4, local, 4);
        let count = property_count(e, this);
        e.mem.set_u32(argument, count);
        stream_write(e, stream, argument, 4, local, 4);
        let count = e.mem.i32(argument);
        if count > 0 {
            let buffer = e
                .call(ALLOCATE, &args![(count as u32).wrapping_mul(4)])
                .u32();
            e.mem.set_u32(local, buffer);
            let mut node = e.get(property_list(this), NiTPointerList::m_pkHead);
            let mut out = buffer;
            while node != 0 {
                let next = e.mem.u32(node);
                let property = e.mem.u32(node + 8);
                e.mem.set_u32(out, property);
                out = out.wrapping_add(4);
                node = next;
            }
            let mut index = e.mem.i32(argument).wrapping_sub(1);
            while index >= 0 {
                let buffer = e.mem.u32(local);
                let object = e
                    .mem
                    .u32(buffer.wrapping_add((index as u32).wrapping_mul(4)));
                e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![object]);
                index -= 1;
            }
            let buffer = e.mem.u32(local);
            e.call(FREE, &args![buffer]);
        }
        let collision = e.get(this, NiAVObject::m_spCollisionObject);
        e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![collision]);
    });
}

// Translated from 00a5aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::NiAVObject` (Xbox PDB): the `NiObjectNET` constructor, the
/// vtable, empty parent, collision object, property list and flags,
/// identity local and world transforms, no world bound, then flags with
/// bit 4 clear and bits 1, 2, 3 and 19 set. Answers `this`.
pub fn ni_av_object_ni_av_object(e: &mut Engine, this: Ptr<NiAVObject>) -> Ptr<NiAVObject> {
    e.call(NI_OBJECT_NET_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_AV_OBJECT_VTABLE);
    e.set(this, NiAVObject::m_spCollisionObject, 0);
    let list = property_list(this);
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    e.set(this, NiAVObject::m_pkParent, 0);
    set_flags(e, this, 0);
    e.call(TRANSFORM_MAKE_IDENTITY, &args![this.addr() + LOCAL_ROTATE]);
    e.call(TRANSFORM_MAKE_IDENTITY, &args![this.addr() + WORLD_ROTATE]);
    let value = (flags(e, this) & 0xffff_ffef) | 0x0008_000e;
    e.set(this, NiAVObject::m_kWorldBound, 0);
    set_flags(e, this, value);
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        release(e, collision);
        e.set(this, NiAVObject::m_spCollisionObject, 0);
    }
    this
}

// Translated from 00a5aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::scalar deleting destructor` (Xbox PDB): the destructor
/// body, then the sized delete of 0x9C bytes when bit 0 of `flags` is set.
/// Answers `this`.
pub fn ni_av_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    flags: u32,
) -> Ptr<NiAVObject> {
    fn_00a5a390(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x9cu32]);
    }
    this
}

// Translated from 00a5aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::RemoveProperty` (Xbox PDB): removes the first property
/// whose `Type()` is `kind` from the list (`0057c5d0`, whose removed
/// pointer is released) and assigns it to the `NiPointer` at `result`
/// (`006e5cc0`). For a kind of 7 or more (signed), or none found, the
/// pointer at `result` is released and cleared. Each property is counted
/// while it is looked at.
pub fn ni_av_object_remove_property(e: &mut Engine, this: Ptr<NiAVObject>, kind: i32, result: Ptr) {
    let clear_result = |e: &mut Engine| {
        let old = e.mem.u32(result.addr());
        if old != 0 {
            release(e, old);
            e.mem.set_u32(result.addr(), 0);
        }
    };
    if kind >= PROPERTY_TYPE_COUNT {
        clear_result(e);
        return;
    }
    // The game's local `NiPointer` (+0) and the argument word holding
    // `kind`, which receives the removed pointer (+4).
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let removed = frame.addr() + 4;
        e.mem.set_i32(removed, kind);
        let list = property_list(this);
        let mut node = e.get(list, NiTPointerList::m_pkHead);
        while node != 0 {
            let property = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            e.mem.set_u32(local, property);
            if property == 0 {
                continue;
            }
            add_ref(e, property);
            if e.vcall(property, SLOT_PROPERTY_TYPE, &args![]).i32() == e.mem.i32(removed) {
                e.call(LIST_REMOVE, &args![list, removed, local]);
                let old = e.mem.u32(removed);
                if old != 0 {
                    release(e, old);
                }
                e.call(POINTER_ASSIGN_FROM, &args![result, local]);
                let found = e.mem.u32(local);
                if found != 0 {
                    release(e, found);
                }
                return;
            }
            release(e, property);
        }
        clear_result(e);
    });
}

/// Adds a string pointer (held in the word at `slot`) at the end of an
/// `NiTArray<char *>` as the code inlines `Add`: grows the array by its
/// grow-by count when it is full, then `SetAt(size, slot)`.
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = e.get(strings, NiTArray::m_usSize) as u32;
    if index >= e.get(strings, NiTArray::m_usMaxSize) as u32 {
        let grow = e.get(strings, NiTArray::m_usGrowBy) as u32;
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, index + grow]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

/// A builder answer stored in the slot word, then added.
fn add_string_result(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32, text: u32) {
    e.mem.set_u32(slot, text);
    add_viewer_string(e, strings, slot);
}

/// A boolean viewer string: the flag goes into the low byte of the slot
/// word and the whole word is passed (its upper bytes are what the slot
/// held before).
fn add_bool_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32, label: u32, value: bool) {
    e.mem.set_u8(slot, value as u8);
    let word = e.mem.u32(slot);
    let text = e.call(VIEWER_STRING_BOOL, &args![label, word]).u32();
    add_string_result(e, strings, slot, text);
}

// Translated from 00a5ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::GetViewerStrings` (Xbox PDB): after
/// `NiObjectNET::GetViewerStrings`, adds the class name, `m_bAppCulled`,
/// `AlwaysDraw` (only when set), the local and world translation, rotation
/// and scale, the world bound (or the null bound), `SelUpdate`,
/// `SelUpdateTransforms`, `SelUpdateRigid`, `SelUpdatePropControllers`, one
/// string per property (its class name and address; `SetAt` inlined), the
/// collision object (when present), `No Decals` and `Fixed Bound` (only
/// when set). The argument word holds each string before it is added.
pub fn ni_av_object_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiAVObject>,
    strings: Ptr<NiTArray>,
) {
    e.with_stack(4, |e, frame| {
        let slot = frame.addr();
        e.mem.set_u32(slot, strings.addr());
        e.call(NI_OBJECT_NET_GET_VIEWER_STRINGS, &args![this, strings]);
        let class_name = e.global::<u32>(NI_AV_OBJECT_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        add_string_result(e, strings, slot, text);
        let value = flags(e, this);
        add_bool_string(e, strings, slot, TEXT_APP_CULLED, value & 1 != 0);
        if (flags(e, this) >> 11) & 1 != 0 {
            add_bool_string(e, strings, slot, TEXT_ALWAYS_DRAW, true);
        }
        let text = e
            .call(
                POINT_VIEWER_STRING,
                &args![this.addr() + LOCAL_TRANSLATE, TEXT_LOCAL_TRANSLATE],
            )
            .u32();
        add_string_result(e, strings, slot, text);
        let text = e
            .call(
                MATRIX_VIEWER_STRING,
                &args![this.addr() + LOCAL_ROTATE, TEXT_LOCAL_ROTATE],
            )
            .u32();
        add_string_result(e, strings, slot, text);
        let scale = x87_quiet_bits(e.mem.u32(this.addr() + LOCAL_SCALE));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![TEXT_LOCAL_SCALE, scale])
            .u32();
        add_string_result(e, strings, slot, text);
        let text = e
            .call(
                POINT_VIEWER_STRING,
                &args![this.addr() + WORLD_TRANSLATE, TEXT_WORLD_TRANSLATE],
            )
            .u32();
        add_string_result(e, strings, slot, text);
        let text = e
            .call(
                MATRIX_VIEWER_STRING,
                &args![this.addr() + WORLD_ROTATE, TEXT_WORLD_ROTATE],
            )
            .u32();
        add_string_result(e, strings, slot, text);
        let scale = x87_quiet_bits(e.mem.u32(this.addr() + WORLD_SCALE));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![TEXT_WORLD_SCALE, scale])
            .u32();
        add_string_result(e, strings, slot, text);
        let mut bound = e.get(this, NiAVObject::m_kWorldBound);
        if bound == 0 {
            bound = NULL_BOUND;
        }
        let text = e
            .call(BOUND_VIEWER_STRING, &args![bound, TEXT_WORLD_BOUND])
            .u32();
        add_string_result(e, strings, slot, text);
        for (bit, label) in [
            (1, TEXT_SEL_UPDATE),
            (2, TEXT_SEL_UPDATE_TRANSFORMS),
            (4, TEXT_SEL_UPDATE_RIGID),
            (3, TEXT_SEL_UPDATE_PROP_CONTROLLERS),
        ] {
            let value = (flags(e, this) >> bit) & 1 != 0;
            add_bool_string(e, strings, slot, label, value);
        }
        if property_count(e, this) != 0 {
            let mut node = e.get(property_list(this), NiTPointerList::m_pkHead);
            while node != 0 {
                let property = e.mem.u32(node + 8);
                node = e.mem.u32(node);
                let rtti = e.vcall(property, SLOT_GET_RTTI, &args![]).u32();
                let name = e.mem.u32(rtti);
                let text = e.call(VIEWER_STRING_POINTER, &args![name, property]).u32();
                let index = e.get(strings, NiTArray::m_usSize);
                if index >= e.get(strings, NiTArray::m_usMaxSize) {
                    let grow = e.get(strings, NiTArray::m_usGrowBy) as u32;
                    e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index as u32]);
                }
                let base = e.get(strings, NiTArray::m_pBase);
                let entry = base.wrapping_add(4 * index as u32);
                if index >= e.get(strings, NiTArray::m_usSize) {
                    e.set(strings, NiTArray::m_usSize, index.wrapping_add(1));
                    if text != 0 {
                        let used = e.get(strings, NiTArray::m_usESize).wrapping_add(1);
                        e.set(strings, NiTArray::m_usESize, used);
                    }
                } else if text != 0 {
                    if e.mem.u32(entry) == 0 {
                        let used = e.get(strings, NiTArray::m_usESize).wrapping_add(1);
                        e.set(strings, NiTArray::m_usESize, used);
                    }
                } else if e.mem.u32(entry) != 0 {
                    let used = e.get(strings, NiTArray::m_usESize).wrapping_add(0xffff);
                    e.set(strings, NiTArray::m_usESize, used);
                }
                let base = e.get(strings, NiTArray::m_pBase);
                e.mem.set_u32(base.wrapping_add(4 * index as u32), text);
            }
        }
        let collision = e.get(this, NiAVObject::m_spCollisionObject);
        if collision != 0 {
            let text = e
                .call(
                    VIEWER_STRING_POINTER,
                    &args![TEXT_COLLISION_OBJECT, collision],
                )
                .u32();
            add_string_result(e, strings, slot, text);
        }
        if (flags(e, this) >> 10) & 1 != 0 {
            add_bool_string(e, strings, slot, TEXT_NO_DECALS, true);
        }
        if (flags(e, this) >> 13) & 1 != 0 {
            add_bool_string(e, strings, slot, TEXT_FIXED_BOUND, true);
        }
    });
}

// Translated from 00a5b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::RemoveProperty_ov2` (Xbox PDB): `RemoveProperty(kind)` into
/// a local `NiPointer`, which is then released.
pub fn ni_av_object_remove_property_ov2(e: &mut Engine, this: Ptr<NiAVObject>, kind: i32) {
    e.with_stack(4, |e, local| {
        ni_av_object_remove_property(e, this, kind, local);
        let removed = e.mem.u32(local.addr());
        if removed != 0 {
            release(e, removed);
        }
    });
}

// Translated from 00b63170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,NiObject *,bool>::GetAt`
/// (Xbox PDB): looks `key` up in its bucket (`KeyToHashIndex`, then
/// `IsKeysEqual` on each item: next +0, key +4, value +8); when found,
/// writes the value byte at `value` and answers true.
pub fn ni_t_map_base_get_at(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    key: u32,
    value: Ptr,
) -> bool {
    let index = e.vcall(this.addr(), SLOT_MAP_HASH_INDEX, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(this.addr(), SLOT_MAP_KEYS_EQUAL, &args![key, item_key])
            .bool()
        {
            let found = e.mem.u8(item + 8);
            e.mem.set_u8(value.addr(), found);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a59cc0,
            ni_av_object_pre_attach_update(Ptr<NiAVObject>, Ptr, Ptr)
        ),
        entry!(0x00a59cf0, ni_av_object_post_attach_update(Ptr<NiAVObject>)),
        entry!(0x00a59d00, ni_av_object_attach_parent(Ptr<NiAVObject>, Ptr)),
        entry!(
            0x00a59d30,
            ni_av_object_get_property(Ptr<NiAVObject>, i32) -> Ptr
        ),
        entry!(
            0x00a59d80,
            ni_av_object_update_transform_and_bounds(Ptr<NiAVObject>, Ptr)
        ),
        entry!(
            0x00a59db0,
            ni_av_object_pre_attach_update_effects(Ptr<NiAVObject>, u32)
        ),
        entry!(0x00a59dc0, ni_av_object_set_bound(Ptr<NiAVObject>, Ptr)),
        entry!(0x00a59e00, ni_av_object_cull(Ptr<NiAVObject>, Ptr)),
        entry!(
            0x00a59e20,
            ni_av_object_get_object_by_name(Ptr<NiAVObject>, Ptr) -> Ptr
        ),
        entry!(
            0x00a59e50,
            ni_av_object_purge_renderer_data(Ptr<NiAVObject>, Ptr)
        ),
        entry!(
            0x00a59e70,
            ni_av_object_process_property_clones(Ptr<NiAVObject>, Ptr<NiTPointerList>, Ptr)
        ),
        entry!(0x00a59ec0, ni_av_object_process_clone(Ptr<NiAVObject>, Ptr)),
        entry!(
            0x00a59f00,
            ni_av_object_register_streamables(Ptr<NiAVObject>, Ptr) -> bool
        ),
        entry!(0x00a59f50, fn_00a59f50(Ptr, Ptr)),
        entry!(
            0x00a59f90,
            ni_av_object_update_downward_pass(Ptr<NiAVObject>, Ptr, u32)
        ),
        entry!(0x00a59fd0, fn_00a59fd0(Ptr<NiAVObject>, Ptr, u32)),
        entry!(
            0x00a5a030,
            ni_av_object_update_controllers(Ptr<NiAVObject>, Ptr)
        ),
        entry!(0x00a5a040, ni_av_object_update_properties(Ptr<NiAVObject>)),
        entry!(
            0x00a5a0e0,
            ni_av_object_has_transform_controller(Ptr<NiAVObject>) -> bool
        ),
        entry!(
            0x00a5a110,
            ni_av_object_has_property_controller(Ptr<NiAVObject>) -> bool
        ),
        entry!(
            0x00a5a140,
            ni_av_object_update_properties_downward(Ptr<NiAVObject>, u32)
        ),
        entry!(
            0x00a5a170,
            ni_av_object_push_local_properties(Ptr<NiAVObject>, Ptr, bool, Ptr) -> Ptr
        ),
        entry!(
            0x00a5a260,
            ni_av_object_is_equal(Ptr<NiAVObject>, Ptr<NiAVObject>) -> bool
        ),
        entry!(0x00a5a310, fn_00a5a310(Ptr<NiTPointerList>, Ptr)),
        entry!(0x00a5a390, fn_00a5a390(Ptr<NiAVObject>)),
        entry!(0x00a5a420, ni_av_object_get_rtti(Ptr<NiAVObject>) -> Ptr),
        entry!(
            0x00a5a430,
            ni_av_object_pre_attach_update_properties(Ptr<NiAVObject>, Ptr)
        ),
        entry!(
            0x00a5a450,
            ni_av_object_set_selective_update_flags(Ptr<NiAVObject>, Ptr, bool, Ptr)
        ),
        entry!(0x00a5a510, ni_av_object_load_binary(Ptr<NiAVObject>, Ptr)),
        entry!(0x00a5a7f0, ni_av_object_link_object(Ptr<NiAVObject>, Ptr)),
        entry!(0x00a5a900, ni_av_object_save_binary(Ptr<NiAVObject>, Ptr)),
        entry!(
            0x00a5aa20,
            ni_av_object_ni_av_object(Ptr<NiAVObject>) -> Ptr<NiAVObject>
        ),
        entry!(
            0x00a5aa90,
            ni_av_object_scalar_deleting_destructor(Ptr<NiAVObject>, u32) -> Ptr<NiAVObject>
        ),
        entry!(
            0x00a5aac0,
            ni_av_object_remove_property(Ptr<NiAVObject>, i32, Ptr)
        ),
        entry!(
            0x00a5ad00,
            ni_av_object_get_viewer_strings(Ptr<NiAVObject>, Ptr<NiTArray>)
        ),
        entry!(
            0x00a5b230,
            ni_av_object_remove_property_ov2(Ptr<NiAVObject>, i32)
        ),
        entry!(
            0x00b63170,
            ni_t_map_base_get_at(Ptr<NiTPointerMap>, u32, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Every vtable slot of a test object calls `SLOT_TARGETS + slot`, a
    /// double that does nothing (so calls are told apart by target and
    /// `this`); tests override the slots they need.
    const SLOT_TARGETS: u32 = 0x7100_0000;
    const TRUE_DOUBLE: u32 = 0x7000_0001;
    const FALSE_DOUBLE: u32 = 0x7000_0002;
    /// Answers the word at +0x10 of `this` (a test property's type).
    const TYPE_DOUBLE: u32 = 0x7000_0003;
    /// Writes the word at `this + 0x18` (the parent) into `SCRATCH`.
    const RECORD_PARENT_DOUBLE: u32 = 0x7000_0004;
    /// Answers whether its two argument words are equal.
    const EQUAL_WORDS_DOUBLE: u32 = 0x7000_0005;
    /// Answers 0 (a hash index).
    const ZERO_DOUBLE: u32 = 0x7000_0006;
    /// The test streams' read and write functions.
    const READ_DOUBLE: u32 = 0x7200_0000;
    const WRITE_DOUBLE: u32 = 0x7200_0001;
    const SCRATCH: u32 = 0x6000_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn slot(offset: u32) -> u32 {
        SLOT_TARGETS + offset
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(SCRATCH, 0x100);
        e.map(0x0109_b000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for offset in (0..0x200).step_by(4) {
            e.register(slot(offset), |_, _| Ret::default());
        }
        e.register(TRUE_DOUBLE, |_, _| ret(1));
        e.register(FALSE_DOUBLE, |_, _| ret(0));
        e.register(TYPE_DOUBLE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(RECORD_PARENT_DOUBLE, |e, a| {
            let parent = e.mem.u32(a[0] + 0x18);
            e.mem.set_u32(SCRATCH, parent);
            Ret::default()
        });
        e.register(EQUAL_WORDS_DOUBLE, |_, a| ret((a[1] == a[2]) as u32));
        e.register(ZERO_DOUBLE, |_, _| ret(0));
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        for addr in [
            MEMORY_MANAGER_DEALLOCATE,
            NI_OBJECT_NET_CONSTRUCT,
            NI_OBJECT_NET_DESTRUCT,
            NI_OBJECT_NET_PROCESS_CLONE,
            NI_OBJECT_NET_LOAD_BINARY,
            NI_OBJECT_NET_LINK_OBJECT,
            NI_OBJECT_NET_SAVE_BINARY,
            NI_OBJECT_NET_GET_VIEWER_STRINGS,
            UPDATE_OBJECT_CONTROLLERS,
            PROPERTY_STATE_DESTRUCT,
            TRANSFORM_MAKE_IDENTITY,
            LIST_REMOVE_ALL,
            LIST_ADD_HEAD,
            POINT_LOAD_BINARY,
            POINT_SAVE_BINARY,
            MATRIX_LOAD_BINARY,
            MATRIX_SAVE_BINARY,
            STREAM_READ_MULTIPLE_LINK_IDS,
            STREAM_READ_LINK_ID,
            FREE,
            SIZED_DELETE,
            STRING_ARRAY_SET_SIZE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| ret(e.mem.alloc(a[1])));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(PROPERTY_STATE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(PROPERTY_STATE_COPY, |_, a| ret(a[0]));
        // The lock: owner word set, recursion count up.
        e.register(LOCK_ACQUIRE, |e, a| {
            e.mem.set_u32(a[0], 1);
            let count = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, count);
            Ret::default()
        });
        // `NiPointer` assignment: a plain store in the tests.
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A zeroed object of `size` bytes with its own vtable (every slot at
    /// its `slot()` double, then `slots`), reference count 1.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for offset in (0..0x200).step_by(4) {
            e.mem.set_u32(vtable + offset, slot(offset));
        }
        for &(offset, target) in slots {
            e.mem.set_u32(vtable + offset, target);
        }
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, vtable);
        e.mem.set_u32(p + 4, 1);
        p
    }

    fn av(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<NiAVObject> {
        Ptr::new(object(e, 0x9c, slots))
    }

    /// A property of type `kind` (answered by slot 0x8C).
    fn property(e: &mut Engine, kind: u32, slots: &[(u32, u32)]) -> u32 {
        let mut all = vec![(SLOT_PROPERTY_TYPE, TYPE_DOUBLE)];
        all.extend_from_slice(slots);
        let p = object(e, 0x20, &all);
        e.mem.set_u32(p + 0x10, kind);
        p
    }

    /// Fills the property list with `elements` (head first).
    fn set_properties(e: &mut Engine, this: Ptr<NiAVObject>, elements: &[u32]) {
        let list = property_list(this);
        e.set(list, NiTPointerList::m_pkHead, 0);
        let mut previous = 0u32;
        for &element in elements {
            let node = e.mem.alloc(0x0c);
            e.mem.set_u32(node + 4, previous);
            e.mem.set_u32(node + 8, element);
            if previous == 0 {
                e.set(list, NiTPointerList::m_pkHead, node);
            } else {
                e.mem.set_u32(previous, node);
            }
            previous = node;
        }
        e.set(list, NiTPointerList::m_pkTail, previous);
        e.set(list, NiTPointerList::m_uiCount, elements.len() as u32);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn clear_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    #[test]
    fn pre_attach_update_sets_the_parent_around_the_downward_pass() {
        let mut e = engine();
        let this = av(&mut e, &[(SLOT_UPDATE_DOWNWARD_PASS, RECORD_PARENT_DOUBLE)]);
        e.call(0x00a5_9cc0, &args![this, 0x1234u32, 0x5678u32]);
        assert_eq!(e.mem.u32(SCRATCH), 0x1234);
        assert_eq!(
            calls(&e, RECORD_PARENT_DOUBLE),
            vec![vec![this.addr(), 0x5678, 0]]
        );
        assert_eq!(e.get(this, NiAVObject::m_pkParent), 0);
    }

    #[test]
    fn post_attach_update_calls_the_parents_upward_pass() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let parent = object(&mut e, 0x40, &[]);
        e.set(this, NiAVObject::m_pkParent, parent);
        e.call(0x00a5_9cf0, &args![this]);
        assert_eq!(
            calls(&e, slot(SLOT_NODE_UPDATE_UPWARD_PASS)),
            vec![vec![parent]]
        );
    }

    #[test]
    fn attach_parent_detaches_from_the_old_parent() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let old = object(&mut e, 0x40, &[]);
        e.set(this, NiAVObject::m_pkParent, old);
        e.call(0x00a5_9d00, &args![this, 0x4444u32]);
        assert_eq!(
            calls(&e, slot(SLOT_NODE_DETACH_CHILD)),
            vec![vec![old, this.addr()]]
        );
        assert_eq!(e.get(this, NiAVObject::m_pkParent), 0x4444);
        clear_log(&mut e);
        e.set(this, NiAVObject::m_pkParent, 0);
        e.call(0x00a5_9d00, &args![this, 0x5555u32]);
        assert!(calls(&e, slot(SLOT_NODE_DETACH_CHILD)).is_empty());
        assert_eq!(e.get(this, NiAVObject::m_pkParent), 0x5555);
    }

    #[test]
    fn get_property_finds_the_first_of_its_type() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let a = property(&mut e, 3, &[]);
        let b = property(&mut e, 2, &[]);
        let c = property(&mut e, 2, &[]);
        set_properties(&mut e, this, &[a, 0, b, c]);
        assert_eq!(e.call(0x00a5_9d30, &args![this, 2i32]).u32(), b);
        assert_eq!(e.call(0x00a5_9d30, &args![this, 5i32]).u32(), 0);
        clear_log(&mut e);
        assert_eq!(e.call(0x00a5_9d30, &args![this, 7i32]).u32(), 0);
        assert!(calls(&e, TYPE_DOUBLE).is_empty());
    }

    #[test]
    fn update_transform_and_bounds_needs_flag_bit_2() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        e.call(0x00a5_9d80, &args![this, 0x10u32]);
        assert!(calls(&e, slot(SLOT_UPDATE_WORLD_DATA)).is_empty());
        set_flags(&mut e, this, 4);
        e.call(0x00a5_9d80, &args![this, 0x10u32]);
        assert_eq!(
            calls(&e, slot(SLOT_UPDATE_WORLD_DATA)),
            vec![vec![this.addr(), 0x10]]
        );
        assert_eq!(
            calls(&e, slot(SLOT_UPDATE_WORLD_BOUND)),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn pre_attach_update_effects_clears_the_parent() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        e.set(this, NiAVObject::m_pkParent, 0x1234);
        e.call(0x00a5_9db0, &args![this, 0x99u32]);
        assert_eq!(e.get(this, NiAVObject::m_pkParent), 0);
    }

    #[test]
    fn set_bound_frees_the_old_block_and_tracks_bit_19() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        e.set(this, NiAVObject::m_kWorldBound, 0x1000);
        e.call(0x00a5_9dc0, &args![this, 0x2000u32]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1000]]
        );
        assert_eq!(e.get(this, NiAVObject::m_kWorldBound), 0x2000);
        assert_eq!(flags(&e, this), 0x0008_0000);
        e.call(0x00a5_9dc0, &args![this, 0u32]);
        assert_eq!(flags(&e, this), 0);
        clear_log(&mut e);
        e.call(0x00a5_9dc0, &args![this, 0u32]);
        assert!(calls(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
    }

    #[test]
    fn cull_skips_app_culled_objects() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let culler = object(&mut e, 0x20, &[]);
        e.call(0x00a5_9e00, &args![this, culler]);
        assert_eq!(
            calls(&e, slot(SLOT_CULLING_PROCESS)),
            vec![vec![culler, this.addr()]]
        );
        clear_log(&mut e);
        set_flags(&mut e, this, 1);
        e.call(0x00a5_9e00, &args![this, culler]);
        assert!(calls(&e, slot(SLOT_CULLING_PROCESS)).is_empty());
    }

    #[test]
    fn get_object_by_name_matches_the_handle() {
        let mut e = engine();
        e.set_global(EMPTY_FIXED_STRING, 0x5000_0000u32);
        let this = av(&mut e, &[]);
        e.set(this, NiAVObject::m_kName, 0x5000_0100);
        e.mem.set_u32(SCRATCH, 0x5000_0100);
        assert_eq!(
            e.call(0x00a5_9e20, &args![this, SCRATCH]).u32(),
            this.addr()
        );
        e.mem.set_u32(SCRATCH, 0x5000_0200);
        assert_eq!(e.call(0x00a5_9e20, &args![this, SCRATCH]).u32(), 0);
        // The empty name finds nothing, even on an unnamed object.
        e.set(this, NiAVObject::m_kName, 0x5000_0000);
        e.mem.set_u32(SCRATCH, 0x5000_0000);
        assert_eq!(e.call(0x00a5_9e20, &args![this, SCRATCH]).u32(), 0);
    }

    #[test]
    fn purge_renderer_data_passes_the_type_2_property() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let renderer = object(&mut e, 0x20, &[]);
        e.call(0x00a5_9e50, &args![this, renderer]);
        assert!(calls(&e, slot(SLOT_RENDERER_PURGE)).is_empty());
        let p = property(&mut e, 2, &[]);
        set_properties(&mut e, this, &[p]);
        e.call(0x00a5_9e50, &args![this, renderer]);
        assert_eq!(
            calls(&e, slot(SLOT_RENDERER_PURGE)),
            vec![vec![renderer, p]]
        );
    }

    /// A process map (`NiTMapBase<NiObject *, bool>`) of one bucket holding
    /// `key` with value `value`.
    fn map_with(e: &mut Engine, key: u32, value: u8) -> Ptr<NiTPointerMap> {
        let map = object(
            e,
            0x10,
            &[
                (SLOT_MAP_HASH_INDEX, ZERO_DOUBLE),
                (SLOT_MAP_KEYS_EQUAL, EQUAL_WORDS_DOUBLE),
            ],
        );
        let table = e.mem.alloc(4);
        let item = e.mem.alloc(0x0c);
        e.mem.set_u32(item + 4, key);
        e.mem.set_u8(item + 8, value);
        e.mem.set_u32(table, item);
        let map: Ptr<NiTPointerMap> = Ptr::new(map);
        e.set(map, NiTPointerMap::m_uiHashSize, 1);
        e.set(map, NiTPointerMap::m_ppkHashTable, table);
        map
    }

    #[test]
    fn process_property_clones_skips_processed_properties() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let a = property(&mut e, 1, &[]);
        let b = property(&mut e, 2, &[]);
        let c = property(&mut e, 3, &[]);
        set_properties(&mut e, this, &[a, b, c]);
        let map = map_with(&mut e, b, 1);
        let cloning = e.mem.alloc(0x1c);
        e.mem.set_u32(cloning + 4, map.addr());
        e.call(0x00a5_9e70, &args![this, property_list(this), cloning]);
        // Tail to head, `b` already processed.
        assert_eq!(
            calls(&e, slot(SLOT_PROCESS_CLONE)),
            vec![vec![c, cloning], vec![a, cloning]]
        );
    }

    #[test]
    fn process_clone_covers_properties_and_collision() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let collision = object(&mut e, 0x0c, &[]);
        e.set(this, NiAVObject::m_spCollisionObject, collision);
        let a = property(&mut e, 1, &[]);
        set_properties(&mut e, this, &[a]);
        let map = map_with(&mut e, 0x9999, 1);
        let cloning = e.mem.alloc(0x1c);
        e.mem.set_u32(cloning + 4, map.addr());
        e.call(0x00a5_9ec0, &args![this, cloning]);
        assert_eq!(
            calls(&e, NI_OBJECT_NET_PROCESS_CLONE),
            vec![vec![this.addr(), cloning]]
        );
        assert_eq!(
            calls(&e, slot(SLOT_PROCESS_CLONE)),
            vec![vec![a, cloning], vec![collision, cloning]]
        );
        // Without properties the list is not walked (the map is not asked).
        clear_log(&mut e);
        set_properties(&mut e, this, &[]);
        e.call(0x00a5_9ec0, &args![this, cloning]);
        assert!(calls(&e, ZERO_DOUBLE).is_empty());
        assert_eq!(calls(&e, slot(SLOT_PROCESS_CLONE)).len(), 1);
    }

    #[test]
    fn register_streamables_stops_when_the_base_does() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let collision = object(&mut e, 0x0c, &[]);
        e.set(this, NiAVObject::m_spCollisionObject, collision);
        let a = property(&mut e, 1, &[]);
        set_properties(&mut e, this, &[0, a]);
        e.register(NI_OBJECT_NET_REGISTER_STREAMABLES, |_, _| ret(0));
        assert!(!e.call(0x00a5_9f00, &args![this, 0x77u32]).bool());
        assert!(calls(&e, slot(SLOT_REGISTER_STREAMABLES)).is_empty());
        e.register(NI_OBJECT_NET_REGISTER_STREAMABLES, |_, _| ret(1));
        assert!(e.call(0x00a5_9f00, &args![this, 0x77u32]).bool());
        assert_eq!(
            calls(&e, slot(SLOT_REGISTER_STREAMABLES)),
            vec![vec![a, 0x77], vec![collision, 0x77]]
        );
    }

    #[test]
    fn property_table_store_by_type() {
        let mut e = engine();
        let table: Ptr = Ptr::new(e.mem.alloc(0x40));
        let p = property(&mut e, 3, &[]);
        e.call(0x00a5_9f50, &args![table, p]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![table.addr() + 12, p]]);
        clear_log(&mut e);
        let too_high = property(&mut e, 8, &[]);
        e.call(0x00a5_9f50, &args![table, too_high]);
        e.call(0x00a5_9f50, &args![table, 0u32]);
        assert!(calls(&e, POINTER_ASSIGN).is_empty());
    }

    #[test]
    fn update_downward_pass_updates_controllers_when_asked() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let update = e.mem.alloc(8);
        e.call(0x00a5_9f90, &args![this, update, 0u32]);
        assert!(calls(&e, UPDATE_OBJECT_CONTROLLERS).is_empty());
        assert_eq!(calls(&e, slot(SLOT_UPDATE_WORLD_DATA)).len(), 1);
        e.mem.set_u8(update + 4, 1);
        e.call(0x00a5_9f90, &args![this, update, 0u32]);
        assert_eq!(
            calls(&e, UPDATE_OBJECT_CONTROLLERS),
            vec![vec![this.addr(), update, 1]]
        );
        assert_eq!(
            calls(&e, slot(SLOT_UPDATE_WORLD_BOUND)),
            vec![vec![this.addr()], vec![this.addr()]]
        );
    }

    #[test]
    fn selected_downward_pass_swaps_the_controller_byte() {
        let mut e = engine();
        e.register(UPDATE_OBJECT_CONTROLLERS, |e, a| {
            let byte = e.mem.u8(a[1] + 4);
            e.mem.set_u8(SCRATCH, byte);
            Ret::default()
        });
        let this = av(&mut e, &[]);
        let update = e.mem.alloc(8);
        e.mem.set_u8(update + 4, 7);
        set_flags(&mut e, this, 8 | 4);
        e.call(0x00a5_9fd0, &args![this, update, 0u32]);
        assert_eq!(e.mem.u8(SCRATCH), 1);
        assert_eq!(e.mem.u8(update + 4), 7);
        assert_eq!(calls(&e, slot(SLOT_UPDATE_WORLD_DATA)).len(), 1);
        // The byte at +5 or a clear bit 2 skip the world update.
        e.mem.set_u8(update + 5, 1);
        e.call(0x00a5_9fd0, &args![this, update, 0u32]);
        e.mem.set_u8(update + 5, 0);
        set_flags(&mut e, this, 0);
        e.call(0x00a5_9fd0, &args![this, update, 0u32]);
        assert_eq!(e.mem.u8(SCRATCH), 0);
        assert_eq!(calls(&e, slot(SLOT_UPDATE_WORLD_DATA)).len(), 1);
    }

    #[test]
    fn update_controllers_passes_true() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        e.call(0x00a5_a030, &args![this, 0x20u32]);
        assert_eq!(
            calls(&e, UPDATE_OBJECT_CONTROLLERS),
            vec![vec![this.addr(), 0x20, 1]]
        );
    }

    #[test]
    fn update_properties_makes_and_frees_a_state() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        e.call(0x00a5_a040, &args![this]);
        let made = calls(&e, PROPERTY_STATE_CONSTRUCT);
        assert_eq!(made.len(), 1);
        let state = made[0][0];
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, PROPERTY_STATE_SIZE]]
        );
        assert_eq!(
            calls(&e, slot(SLOT_UPDATE_PROPERTIES_DOWNWARD)),
            vec![vec![this.addr(), state]]
        );
        assert_eq!(calls(&e, PROPERTY_STATE_DESTRUCT), vec![vec![state]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, state]]
        );
        assert_eq!(calls(&e, LOCK_ACQUIRE), vec![vec![PROPERTY_LOCK, 0]]);
        assert_eq!(e.global::<u32>(PROPERTY_LOCK_COUNT), 0);
        assert_eq!(e.global::<u32>(PROPERTY_LOCK), 0);

        // With a parent, the parent's state is used (and still freed).
        clear_log(&mut e);
        e.register(NODE_UPDATE_PROPERTIES_UPWARD, |e, a| {
            e.mem.set_u32(a[1], 0x0bad_0000);
            Ret::default()
        });
        e.set(this, NiAVObject::m_pkParent, 0x4000);
        e.call(0x00a5_a040, &args![this]);
        assert!(calls(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(
            calls(&e, slot(SLOT_UPDATE_PROPERTIES_DOWNWARD)),
            vec![vec![this.addr(), 0x0bad_0000]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0bad_0000]]
        );
    }

    #[test]
    fn has_transform_controller_walks_the_chain() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let first = object(
            &mut e,
            0x34,
            &[(SLOT_IS_TRANSFORM_CONTROLLER, FALSE_DOUBLE)],
        );
        let second = object(&mut e, 0x34, &[(SLOT_IS_TRANSFORM_CONTROLLER, TRUE_DOUBLE)]);
        e.set(this, NiAVObject::m_spControllers, first);
        assert!(!e.call(0x00a5_a0e0, &args![this]).bool());
        e.mem.set_u32(first + CONTROLLER_NEXT, second);
        assert!(e.call(0x00a5_a0e0, &args![this]).bool());
    }

    #[test]
    fn has_property_controller_checks_each_property() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let a = property(&mut e, 1, &[]);
        set_properties(&mut e, this, &[0, a]);
        assert!(!e.call(0x00a5_a110, &args![this]).bool());
        e.mem.set_u32(a + PROPERTY_CONTROLLERS, 0x1234);
        assert!(e.call(0x00a5_a110, &args![this]).bool());
    }

    #[test]
    fn update_properties_downward_tracks_bit_18() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let a = property(&mut e, 1, &[]);
        set_properties(&mut e, this, &[a]);
        set_flags(&mut e, this, 0x0004_0001);
        e.call(0x00a5_a140, &args![this, 0u32]);
        assert_eq!(flags(&e, this), 1);
        e.mem.set_u32(a + PROPERTY_CONTROLLERS, 0x1234);
        e.call(0x00a5_a140, &args![this, 0u32]);
        assert_eq!(flags(&e, this), 0x0004_0001);
    }

    #[test]
    fn push_local_properties_fills_the_state_slots() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let result = e.mem.alloc(4);
        // No properties: the parent state is the answer.
        let back = e.call(0x00a5_a170, &args![this, 0x3000u32, true, result]);
        assert_eq!(back.u32(), result);
        assert_eq!(e.mem.u32(result), 0x3000);
        assert!(calls(&e, LOCK_ACQUIRE).is_empty());

        let state = e.mem.alloc(0x20);
        let old = property(&mut e, 1, &[]);
        e.mem.set_u32(state + 4, old);
        let new = property(&mut e, 1, &[]);
        let high = property(&mut e, 9, &[]);
        set_properties(&mut e, this, &[new, high]);
        e.call(0x00a5_a170, &args![this, state, false, result]);
        assert_eq!(e.mem.u32(result), state);
        assert_eq!(e.mem.u32(state + 4), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        // The old one was released to zero and deleted.
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(calls(&e, slot(SLOT_DELETE_THIS)), vec![vec![old]]);
        assert_eq!(e.global::<u32>(PROPERTY_LOCK_COUNT), 0);

        // `copy`: the state comes from `00a89810` on a new block.
        clear_log(&mut e);
        e.call(0x00a5_a170, &args![this, state, true, result]);
        let copies = calls(&e, PROPERTY_STATE_COPY);
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][1], state);
        assert_eq!(e.mem.u32(result), copies[0][0]);
        assert_eq!(e.mem.u32(copies[0][0] + 4), new);
    }

    #[test]
    fn is_equal_compares_flags_transform_properties_and_collision() {
        let mut e = engine();
        e.register(NI_OBJECT_NET_IS_EQUAL, |_, _| ret(1));
        e.register(TRANSFORM_DIFFERS, |_, _| ret(0));
        let this = av(&mut e, &[]);
        let other = av(&mut e, &[]);
        let a = property(&mut e, 1, &[(SLOT_IS_EQUAL, TRUE_DOUBLE)]);
        let b = property(&mut e, 1, &[]);
        set_properties(&mut e, this, &[a, 0]);
        set_properties(&mut e, other, &[b, 0]);
        assert!(e.call(0x00a5_a260, &args![this, other]).bool());
        assert_eq!(calls(&e, TRUE_DOUBLE), vec![vec![a, b]]);
        // Only one side with a collision object.
        e.set(this, NiAVObject::m_spCollisionObject, 0x1234);
        assert!(!e.call(0x00a5_a260, &args![this, other]).bool());
        e.set(this, NiAVObject::m_spCollisionObject, 0);
        // Different app-culled bits.
        set_flags(&mut e, this, 1);
        assert!(!e.call(0x00a5_a260, &args![this, other]).bool());
        set_flags(&mut e, this, 0);
        // Null against non-null property.
        set_properties(&mut e, other, &[b, a]);
        assert!(!e.call(0x00a5_a260, &args![this, other]).bool());
        // Different transforms.
        set_properties(&mut e, other, &[b, 0]);
        e.register(TRANSFORM_DIFFERS, |_, _| ret(1));
        assert!(!e.call(0x00a5_a260, &args![this, other]).bool());
    }

    #[test]
    fn list_append_links_and_counts() {
        let mut e = engine();
        e.register(LIST_NEW_NODE, |e, _| {
            let node = e.mem.alloc(0x0c);
            ret(node)
        });
        let list: Ptr<NiTPointerList> = e.new_object();
        let first = object(&mut e, 0x10, &[]);
        let second = object(&mut e, 0x10, &[]);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, first);
        e.call(0x00a5_a310, &args![list, pointer]);
        e.mem.set_u32(pointer, second);
        e.call(0x00a5_a310, &args![list, pointer]);
        let head = e.get(list, NiTPointerList::m_pkHead);
        let tail = e.get(list, NiTPointerList::m_pkTail);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);
        assert_eq!(e.mem.u32(head + 8), first);
        assert_eq!(e.mem.u32(head), tail);
        assert_eq!(e.mem.u32(tail + 4), head);
        assert_eq!(e.mem.u32(tail + 8), second);
        assert_eq!(e.mem.u32(tail), 0);
        assert_eq!(e.mem.u32(first + 4), 2);
        assert_eq!(calls(&e, LIST_NEW_NODE)[0], vec![list.addr() + 8]);
    }

    #[test]
    fn destructor_body_releases_everything() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let collision = object(&mut e, 0x0c, &[]);
        e.set(this, NiAVObject::m_spCollisionObject, collision);
        e.set(this, NiAVObject::m_kWorldBound, 0x2000);
        set_flags(&mut e, this, 0x0008_0001);
        e.call(0x00a5_a390, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_AV_OBJECT_VTABLE);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x2000]]
        );
        assert_eq!(flags(&e, this), 1);
        assert_eq!(e.get(this, NiAVObject::m_kWorldBound), 0);
        assert_eq!(calls(&e, LIST_REMOVE_ALL).len(), 2);
        assert_eq!(calls(&e, slot(SLOT_DELETE_THIS)), vec![vec![collision]]);
        assert_eq!(e.get(this, NiAVObject::m_spCollisionObject), 0);
        assert_eq!(calls(&e, NI_OBJECT_NET_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = engine();
        assert_eq!(e.call(0x00a5_a420, &args![0u32]).u32(), NI_AV_OBJECT_RTTI);
    }

    #[test]
    fn pre_attach_update_properties_sets_the_parent_around_the_update() {
        let mut e = engine();
        e.register(NODE_UPDATE_PROPERTIES_UPWARD, |e, a| {
            e.mem.set_u32(SCRATCH, a[0]);
            Ret::default()
        });
        let this = av(&mut e, &[]);
        e.call(0x00a5_a430, &args![this, 0x4321u32]);
        assert_eq!(e.mem.u32(SCRATCH), 0x4321);
        assert_eq!(e.get(this, NiAVObject::m_pkParent), 0);
    }

    #[test]
    fn selective_update_flags_follow_the_controllers() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let out = e.mem.alloc(2);
        let selective = out;
        let rigid = out + 1;
        // Nothing animated: rigid, no selective update.
        set_flags(&mut e, this, !0x80);
        e.call(0x00a5_a450, &args![this, selective, false, rigid]);
        assert_eq!(e.mem.u8(rigid), 1);
        assert_eq!(e.mem.u8(selective), 0);
        assert_eq!(flags(&e, this), !0x80 & !0x0e);
        // Bit 7 makes it non-rigid: transforms and selective update.
        set_flags(&mut e, this, 0x80);
        e.call(0x00a5_a450, &args![this, selective, false, rigid]);
        assert_eq!(e.mem.u8(rigid), 0);
        assert_eq!(e.mem.u8(selective), 1);
        assert_eq!(flags(&e, this), 0x80 | 0x02 | 0x04 | 0x10);
        // A property controller: bits 3 and 4.
        let a = property(&mut e, 1, &[]);
        e.mem.set_u32(a + PROPERTY_CONTROLLERS, 0x1234);
        set_properties(&mut e, this, &[a]);
        set_flags(&mut e, this, 0x04);
        e.call(0x00a5_a450, &args![this, selective, false, rigid]);
        assert_eq!(e.mem.u8(rigid), 1);
        assert_eq!(e.mem.u8(selective), 1);
        assert_eq!(flags(&e, this), 0x02 | 0x18);
        // A transform controller: not rigid.
        set_properties(&mut e, this, &[]);
        let controller = object(&mut e, 0x34, &[(SLOT_IS_TRANSFORM_CONTROLLER, TRUE_DOUBLE)]);
        e.set(this, NiAVObject::m_spControllers, controller);
        set_flags(&mut e, this, 0);
        e.call(0x00a5_a450, &args![this, selective, true, rigid]);
        assert_eq!(e.mem.u8(rigid), 0);
        assert_eq!(flags(&e, this), 0x02 | 0x04 | 0x10);
    }

    /// A stream whose input and output `NiBinaryStream`s call the read and
    /// write doubles.
    fn stream(e: &mut Engine, bs_version: u32, nif_version: u32) -> Ptr {
        let stream = object(e, 0x400, &[]);
        e.mem.set_u32(stream + STREAM_BS_VERSION, bs_version);
        e.mem.set_u32(stream + STREAM_NIF_VERSION, nif_version);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE_DOUBLE);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        Ptr::new(stream)
    }

    /// Reads take the low bytes of the next queued word.
    fn queue_reads(e: &mut Engine, words: &[u32]) {
        let mut queue: VecDeque<u32> = words.iter().copied().collect();
        e.register_double(READ_DOUBLE, move |e, a| {
            let word = queue.pop_front().expect("read");
            let bytes = word.to_le_bytes();
            e.mem.write(a[1], &bytes[..a[2] as usize]);
            ret(a[2])
        });
    }

    #[test]
    fn load_binary_current_layout() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let s = stream(&mut e, 0x22, 0x1402_0007);
        queue_reads(&mut e, &[0x0000_1234, 2.5f32.to_bits()]);
        e.call(0x00a5_a510, &args![this, s]);
        assert_eq!(flags(&e, this), (0x1234 & !0x10) | 0x0008_000e);
        assert_eq!(e.mem.f32(this.addr() + LOCAL_SCALE), 2.5);
        let input = e.mem.u32(s.addr() + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 8);
        let reads = calls(&e, READ_DOUBLE);
        assert_eq!(reads[0][1], this.addr() + 0x30);
        assert_eq!(reads[0][2], 4);
        assert_eq!(reads[0][4], 1);
        assert_eq!(
            calls(&e, POINT_LOAD_BINARY),
            vec![vec![this.addr() + LOCAL_TRANSLATE, s.addr()]]
        );
        assert_eq!(
            calls(&e, MATRIX_LOAD_BINARY),
            vec![vec![this.addr() + LOCAL_ROTATE, s.addr()]]
        );
        assert_eq!(calls(&e, STREAM_READ_MULTIPLE_LINK_IDS).len(), 1);
        assert_eq!(calls(&e, STREAM_READ_LINK_ID).len(), 1);
    }

    #[test]
    fn load_binary_old_layout_makes_collision_data() {
        let mut e = engine();
        e.mem.set_cstr(COLLISION_SWITCH_NAME, b"NiCollisionSwitch");
        e.register(CREATE_OBJECT_BY_NAME, |e, _| {
            let collision = object(e, 0x10, &[(SLOT_COLLISION_INITIALIZE, 0x7200_0002)]);
            ret(collision)
        });
        // `Initialize` keeps the 12 bytes it gets in `SCRATCH`.
        e.register(0x7200_0002, |e, a| {
            let data = e.mem.bytes(a[1], 12);
            e.mem.write(SCRATCH, &data);
            Ret::default()
        });
        let this = av(&mut e, &[]);
        let s = stream(&mut e, 0x10, 0x0402_0200);
        e.mem.set_cstr(s.addr() + STREAM_COMPARED_NAME, b"Other");
        // Two-byte flags 0x2B, the scale, then the bound byte.
        queue_reads(&mut e, &[0x2b, 1.0f32.to_bits(), 1]);
        e.call(0x00a5_a510, &args![this, s]);
        // 0x2B: mode (bits 1..3) 5, flags ((0x2B >> 3) & ~1) | 1 = 5.
        assert_eq!(e.mem.u32(SCRATCH), this.addr());
        assert_eq!(e.mem.u32(SCRATCH + 4), s.addr());
        assert_eq!(e.mem.u8(SCRATCH + 8), 1);
        assert_eq!(e.mem.u16(SCRATCH + 10), 5);
        assert_eq!(
            calls(&e, CREATE_OBJECT_BY_NAME),
            vec![vec![COLLISION_DATA_NAME]]
        );
        assert_eq!(e.mem.u16(s.addr() + STREAM_AV_OBJECT_FLAGS), 5);
        let collision = e.get(this, NiAVObject::m_spCollisionObject);
        assert_eq!(
            calls(&e, slot(SLOT_COLLISION_CONVERT)),
            vec![vec![collision, 0x0402_0200, 0]]
        );
        assert_eq!(flags(&e, this), 0x0008_000f);
        assert_eq!(calls(&e, POINT_LOAD_BINARY).len(), 2);
        assert!(calls(&e, STREAM_READ_LINK_ID).is_empty());
    }

    #[test]
    fn load_binary_old_layout_collision_switch_mode() {
        // No bound byte, a velocity equal to the compared point: nothing is
        // made at mode 2. A "NiCollisionSwitch" stream turns the mode into 4
        // unless the converted flags have bit 3 (0x45 -> 8: set).
        for (word, made) in [(0x45u32, 0usize), (0x05, 1)] {
            let mut e = engine();
            e.mem.set_cstr(COLLISION_SWITCH_NAME, b"NiCollisionSwitch");
            e.register(POINT_NOT_EQUAL, |_, _| ret(0));
            e.register(CREATE_OBJECT_BY_NAME, |_, _| ret(0));
            let this = av(&mut e, &[]);
            let s = stream(&mut e, 0x10, 0x0402_0200);
            e.mem
                .set_cstr(s.addr() + STREAM_COMPARED_NAME, b"NiCollisionSwitch");
            queue_reads(&mut e, &[word, 1.0f32.to_bits(), 0]);
            e.call(0x00a5_a510, &args![this, s]);
            assert_eq!(calls(&e, POINT_NOT_EQUAL)[0][1], COMPARED_POINT);
            assert_eq!(calls(&e, CREATE_OBJECT_BY_NAME).len(), made, "{word:#x}");
        }
    }

    #[test]
    fn link_object_adds_properties_and_the_collision_object() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        let a = object(&mut e, 0x10, &[]);
        let b = object(&mut e, 0x10, &[]);
        let collision = object(&mut e, 0x10, &[]);
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| ret(2));
        let mut objects: VecDeque<u32> = [a, b, collision].into_iter().collect();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(objects.pop_front().unwrap())
        });
        // `AddHead` sees the pointer and its count.
        e.register(LIST_ADD_HEAD, |e, a| {
            let object = e.mem.u32(a[1]);
            let count = e.mem.u32(object + 4);
            let n = e.mem.u32(SCRATCH);
            e.mem.set_u32(SCRATCH + 4 + 8 * n, object);
            e.mem.set_u32(SCRATCH + 8 + 8 * n, count);
            e.mem.set_u32(SCRATCH, n + 1);
            Ret::default()
        });
        let s = stream(&mut e, 0x22, 0x1402_0007);
        e.call(0x00a5_a7f0, &args![this, s]);
        assert_eq!(e.mem.u32(SCRATCH), 2);
        assert_eq!(e.mem.u32(SCRATCH + 4), a);
        assert_eq!(e.mem.u32(SCRATCH + 8), 2);
        assert_eq!(e.mem.u32(SCRATCH + 12), b);
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(e.get(this, NiAVObject::m_spCollisionObject), collision);
        assert_eq!(
            calls(&e, slot(SLOT_COLLISION_CONVERT)),
            vec![vec![collision, 0x1402_0007, 0]]
        );
    }

    #[test]
    fn link_object_old_layout_appends_and_stops() {
        let mut e = engine();
        e.register(LIST_NEW_NODE, |e, _| {
            let node = e.mem.alloc(0x0c);
            ret(node)
        });
        let this = av(&mut e, &[]);
        let a = object(&mut e, 0x10, &[]);
        let b = object(&mut e, 0x10, &[]);
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| ret(2));
        let mut objects: VecDeque<u32> = [a, b].into_iter().collect();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(objects.pop_front().unwrap())
        });
        let s = stream(&mut e, 0x10, 0x0400_0002);
        e.call(0x00a5_a7f0, &args![this, s]);
        let list = property_list(this);
        let head = e.get(list, NiTPointerList::m_pkHead);
        assert_eq!(e.mem.u32(head + 8), a);
        let tail = e.get(list, NiTPointerList::m_pkTail);
        assert_eq!(e.mem.u32(tail + 8), b);
        assert_eq!(e.mem.u32(a + 4), 2);
        assert_eq!(calls(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 2);
        assert!(calls(&e, POINTER_ASSIGN).is_empty());
    }

    #[test]
    fn save_binary_writes_links_last_to_first() {
        let mut e = engine();
        e.register(WRITE_DOUBLE, |e, a| {
            let word = e.mem.u32(a[1]);
            let n = e.mem.u32(SCRATCH);
            e.mem.set_u32(SCRATCH + 4 + 4 * n, word);
            e.mem.set_u32(SCRATCH, n + 1);
            ret(a[2])
        });
        let this = av(&mut e, &[]);
        let a = property(&mut e, 1, &[]);
        let b = property(&mut e, 2, &[]);
        set_properties(&mut e, this, &[a, b]);
        set_flags(&mut e, this, 0x0008_000e);
        e.mem.set_f32(this.addr() + LOCAL_SCALE, 1.5);
        e.set(this, NiAVObject::m_spCollisionObject, 0x4444);
        let s = stream(&mut e, 0x22, 0x1402_0007);
        e.call(0x00a5_a900, &args![this, s]);
        assert_eq!(e.mem.u32(SCRATCH), 3);
        assert_eq!(e.mem.u32(SCRATCH + 4), 0x0008_000e);
        assert_eq!(e.mem.f32(SCRATCH + 8), 1.5);
        assert_eq!(e.mem.u32(SCRATCH + 12), 2);
        let output = e.mem.u32(s.addr() + STREAM_OUTPUT);
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 12);
        assert_eq!(
            calls(&e, slot(SLOT_SAVE_LINK_ID)),
            vec![vec![s.addr(), b], vec![s.addr(), a], vec![s.addr(), 0x4444]]
        );
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls(&e, FREE).len(), 1);
        // Without properties no array is made.
        clear_log(&mut e);
        set_properties(&mut e, this, &[]);
        e.call(0x00a5_a900, &args![this, s]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(calls(&e, slot(SLOT_SAVE_LINK_ID)).len(), 1);
    }

    #[test]
    fn constructor_sets_up_an_empty_object() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        e.set(this, NiAVObject::m_pkParent, 0x1111);
        e.set(this, NiAVObject::m_kWorldBound, 0x2222);
        set_flags(&mut e, this, 0xffff_ffff);
        assert_eq!(e.call(0x00a5_aa20, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_AV_OBJECT_VTABLE);
        assert_eq!(flags(&e, this), 0x0008_000e);
        assert_eq!(e.get(this, NiAVObject::m_pkParent), 0);
        assert_eq!(e.get(this, NiAVObject::m_kWorldBound), 0);
        assert_eq!(
            calls(&e, TRANSFORM_MAKE_IDENTITY),
            vec![
                vec![this.addr() + LOCAL_ROTATE],
                vec![this.addr() + WORLD_ROTATE]
            ]
        );
        assert_eq!(calls(&e, NI_OBJECT_NET_CONSTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = engine();
        let this = av(&mut e, &[]);
        assert_eq!(e.call(0x00a5_aa90, &args![this, 0u32]).u32(), this.addr());
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a5_aa90, &args![this, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this.addr(), 0x9c]]);
        assert_eq!(calls(&e, NI_OBJECT_NET_DESTRUCT).len(), 2);
    }

    /// `0057c5d0` leaves its out word null; `006e5cc0` counts and stores.
    fn removal_doubles(e: &mut Engine) {
        e.register(LIST_REMOVE, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.register(POINTER_ASSIGN_FROM, |e, a| {
            let object = e.mem.u32(a[1]);
            let count = e.mem.u32(object + 4) + 1;
            e.mem.set_u32(object + 4, count);
            e.mem.set_u32(a[0], object);
            ret(a[0])
        });
    }

    #[test]
    fn remove_property_hands_over_the_match() {
        let mut e = engine();
        removal_doubles(&mut e);
        let this = av(&mut e, &[]);
        let a = property(&mut e, 1, &[]);
        let b = property(&mut e, 2, &[]);
        set_properties(&mut e, this, &[a, b]);
        let result = e.mem.alloc(4);
        e.call(0x00a5_aac0, &args![this, 2i32, result]);
        assert_eq!(e.mem.u32(result), b);
        assert_eq!(e.mem.u32(b + 4), 2);
        assert_eq!(e.mem.u32(a + 4), 1);
        let removal = calls(&e, LIST_REMOVE);
        assert_eq!(removal.len(), 1);
        assert_eq!(removal[0][0], property_list(this).addr());

        // Kind 7 (or no match): the result pointer is released and cleared.
        let held = property(&mut e, 4, &[]);
        e.mem.set_u32(result, held);
        e.call(0x00a5_aac0, &args![this, 7i32, result]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(calls(&e, slot(SLOT_DELETE_THIS)), vec![vec![held]]);
        let held = property(&mut e, 4, &[]);
        e.mem.set_u32(result, held);
        e.call(0x00a5_aac0, &args![this, 5i32, result]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.mem.u32(held + 4), 0);
        assert_eq!(e.mem.u32(a + 4), 1);
    }

    #[test]
    fn remove_property_ov2_releases_the_result() {
        let mut e = engine();
        removal_doubles(&mut e);
        let this = av(&mut e, &[]);
        let a = property(&mut e, 3, &[]);
        set_properties(&mut e, this, &[a]);
        e.call(0x00a5_b230, &args![this, 3i32]);
        assert_eq!(calls(&e, POINTER_ASSIGN_FROM).len(), 1);
        assert_eq!(e.mem.u32(a + 4), 1);
    }

    #[test]
    fn get_viewer_strings_lists_the_fields() {
        let mut e = engine();
        // `SetAt` stores the word at its slot argument.
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0x0a, a[1] as u16 + 1);
            Ret::default()
        });
        e.register(VIEWER_STRING_NAME, |_, _| ret(0x100));
        e.register(VIEWER_STRING_BOOL, |_, a| ret(a[1]));
        e.register(VIEWER_STRING_FLOAT, |_, a| ret(a[1]));
        e.register(VIEWER_STRING_POINTER, |_, a| ret(a[1]));
        e.register(POINT_VIEWER_STRING, |_, _| ret(0x300));
        e.register(MATRIX_VIEWER_STRING, |_, _| ret(0x400));
        e.register(BOUND_VIEWER_STRING, |_, a| ret(a[0]));
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti, 0x5555);
        e.set_global(NI_AV_OBJECT_RTTI, 0x7777u32);
        let rtti_double = 0x7200_0003;
        e.register(rtti_double, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let this = av(&mut e, &[]);
        let p = object(&mut e, 0x20, &[(SLOT_GET_RTTI, rtti_double)]);
        e.mem.set_u32(p + 0x10, rtti);
        set_properties(&mut e, this, &[p]);
        let collision = 0x0001_2340;
        e.set(this, NiAVObject::m_spCollisionObject, collision);
        // App-culled, transforms, always draw, fixed bound.
        set_flags(&mut e, this, 0x2805);
        // A signalling NaN local scale comes out quiet.
        e.mem.set_u32(this.addr() + LOCAL_SCALE, 0x7f80_0001);
        e.mem.set_f32(this.addr() + WORLD_SCALE, 2.0);
        let strings: Ptr<NiTArray> = e.new_object();
        let base = e.mem.alloc(0x100);
        e.set(strings, NiTArray::m_pBase, base);
        e.set(strings, NiTArray::m_usMaxSize, 32);
        e.set(strings, NiTArray::m_usGrowBy, 4);
        e.call(0x00a5_ad00, &args![this, strings]);
        let expected = [
            0x100,
            0x101,
            0x101,
            0x300,
            0x400,
            0x7fc0_0001,
            0x300,
            0x400,
            2.0f32.to_bits(),
            NULL_BOUND,
            NULL_BOUND & !0xff,
            (NULL_BOUND & !0xff) | 1,
            NULL_BOUND & !0xff,
            NULL_BOUND & !0xff,
            p,
            collision,
            (collision & !0xff) | 1,
        ];
        assert_eq!(e.get(strings, NiTArray::m_usSize) as usize, expected.len());
        for (i, want) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(base + 4 * i as u32), *want, "entry {i}");
        }
        // The inlined `SetAt` of the property string counted it.
        assert_eq!(e.get(strings, NiTArray::m_usESize), 1);
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x7777]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_POINTER),
            vec![vec![0x5555, p], vec![TEXT_COLLISION_OBJECT, collision]]
        );
        assert!(calls(&e, STRING_ARRAY_SET_SIZE).is_empty());
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array() {
        let mut e = engine();
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            e.mem.set_u16(a[0] + 0x0a, a[1] as u16 + 1);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 0x08, a[1] as u16);
            Ret::default()
        });
        for addr in [
            VIEWER_STRING_NAME,
            VIEWER_STRING_BOOL,
            VIEWER_STRING_FLOAT,
            VIEWER_STRING_POINTER,
            POINT_VIEWER_STRING,
            MATRIX_VIEWER_STRING,
            BOUND_VIEWER_STRING,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        let this = av(&mut e, &[]);
        let strings: Ptr<NiTArray> = e.new_object();
        e.set(strings, NiTArray::m_usGrowBy, 8);
        e.call(0x00a5_ad00, &args![this, strings]);
        // 13 strings: grown at 0 and at 8.
        assert_eq!(
            calls(&e, STRING_ARRAY_SET_SIZE),
            vec![vec![strings.addr(), 8], vec![strings.addr(), 16]]
        );
        assert_eq!(e.get(strings, NiTArray::m_usSize), 13);
    }

    #[test]
    fn get_at_finds_the_value_byte() {
        let mut e = engine();
        let map = map_with(&mut e, 0x1234, 1);
        let value = e.mem.alloc(4);
        assert!(e.call(0x00b6_3170, &args![map, 0x1234u32, value]).bool());
        assert_eq!(e.mem.u8(value), 1);
        e.mem.set_u8(value, 9);
        assert!(!e.call(0x00b6_3170, &args![map, 0x9999u32, value]).bool());
        assert_eq!(e.mem.u8(value), 9);
    }
}
