//! `NiAnimation/nibonelodcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoneLODController::ms_RTTI` (the class's `NiRTTI` record).
const NI_BONE_LOD_CONTROLLER_RTTI: u32 = 0x011f_405c;

// Translated from 00a55120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bone_lod_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BONE_LOD_CONTROLLER_RTTI)
}

use crate::types::{NiAVObject, NiTArray};
use crate::units::nimain::nistream::NiTPrimitiveSet;
use crate::units::nimain::nitimecontroller::NiTimeController;

layout! {
    /// `NiBoneLODController` (Xbox PDB, 0x68 bytes; the same offsets on PC):
    /// an `NiTimeController`, the LOD fields, two `NiTPrimitiveArray`s of
    /// `NiTPrimitiveSet` pointers (bones per LOD at +0x3C, skin infos per LOD
    /// at +0x4C, each with the [`NiTArray`] layout) and the
    /// `NiTPrimitiveSet<NiTriBasedGeom*>` of skinned geometry at +0x5C.
    pub struct NiBoneLODController: 0x68 {
        /// `m_iLOD` (Xbox PDB).
        0x34 m_iLOD: u32,
        /// `m_uiNumLODs` (Xbox PDB).
        0x38 m_uiNumLODs: u32,
    }
}

/// The two arrays and the set inside the controller (see the layout).
const BONE_ARRAY: u32 = 0x3c;
const SKIN_ARRAY: u32 = 0x4c;
const SKIN_SET: u32 = 0x5c;
/// `NiBoneLODController::SkinInfo` (Xbox PDB, 8 bytes): `m_pkSkinGeom` +0
/// and `m_spSkinInst` (`NiPointer<NiSkinInstance>`) +4.
const SKIN_INFO_GEOMETRY: u32 = 0x0;
const SKIN_INFO_INSTANCE: u32 = 0x4;
/// The geometry's skin-instance `NiPointer` the LOD switch assigns (+0xBC
/// on PC, by its use here).
const GEOMETRY_SKIN_INSTANCE: u32 = 0xbc;
/// The vtables the constructor and destructor store: the class's own and
/// those of its two arrays (RTTI `NiTPrimitiveArray<NiTPrimitiveSet<NiNode*>*>`
/// and `NiTPrimitiveArray<NiTPrimitiveSet<SkinInfo*>*>`; the destructor
/// lowers them to the `NiTArray` vtables before freeing the elements).
const VTABLE: u32 = 0x0109_a2d4;
const BONE_ARRAY_VTABLE: u32 = 0x0109_a2c0;
const SKIN_ARRAY_VTABLE: u32 = 0x0109_a2c8;
const BONE_NI_T_ARRAY_VTABLE: u32 = 0x0109_a2a4;
const SKIN_NI_T_ARRAY_VTABLE: u32 = 0x0109_a2ac;
const OBJECT_SIZE: u32 = 0x68;
/// `NiNode::ms_RTTI` (`011f4428`).
const NI_NODE_RTTI: u32 = 0x011f_4428;
/// The LOD every bone LOD controller is asked to show (`011a90c4`, -1 for
/// none).
const GLOBAL_LOD: u32 = 0x011a_90c4;
/// `NiTimeController` functions (engine map names): `RegisterStreamables`
/// (`00a6d0f0`), `LinkObject` (`00a6d500`), `IsEqual` (`00a6cc80`),
/// `ProcessClone` (`00a6d0c0`), `SaveBinary` (`00a6d730`), `LoadBinary`
/// (`00a6d5e0`); the constructor body (`00a6d220`, `thiscall`, no words),
/// the destructor body (`00a6d290`) and the member copy (`00a6cfe0`,
/// `thiscall (this; clone, cloning)`), unnamed in the engine map.
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// The base `PostLinkObject` (`00a89af0`, a 3-byte body with no engine map
/// name; `thiscall`, one word).
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, engine map; `thiscall`, no
/// words: the next linked object) and the link-id read (`00a645b0`,
/// `thiscall` on the stream, no words; no engine map name).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// The stream's vtable slot the save passes each linked object to.
const STREAM_SAVE_LINK_SLOT: u32 = 0x30;
/// `NiTPointerMap::GetAt` (`00a53c30`, by its body: `thiscall (map; key,
/// &value)`, answers whether the key was found).
const MAP_GET_AT: u32 = 0x00a5_3c30;
/// The pointer array's `SetSize` (`00a90180`, by its use: `thiscall (array;
/// size)`) and `SetAt` (`00ba3aa0`, by its body: `thiscall (array; index,
/// &value)`); no engine map names.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `NiTSet<NiNode*>::Realloc` (`00a311e0`, engine map; `thiscall (set;
/// size)`).
const SET_REALLOC: u32 = 0x00a3_11e0;
/// The `cdecl` allocate (`00aa1070`, size) and free (`00aa10f0`, block),
/// `operator new` (`00aa13e0`) and the sized delete (`00aa1460`, block,
/// size).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The allocator object at `011f6238` (`BSCore/memorymanager.cpp`) whose
/// `00aa3e40` (`thiscall`, one word: the size) hands out the sets and skin
/// infos and whose `00aa4060` (`thiscall`, one word: the block) takes them
/// back, by their use here.
const SMALL_ALLOCATOR: u32 = 0x011f_6238;
const SMALL_ALLOCATE: u32 = 0x00aa_3e40;
const SMALL_FREE: u32 = 0x00aa_4060;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`), its signed
/// (`00a7d4d0`) and unsigned (`00a7d510`) overloads (`cdecl (name,
/// value)`); the viewer-string array's grow (`0096ad30`) and store
/// (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_SIGNED: u32 = 0x00a7_d4d0;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_iLOD"` and `"m_uiNumLODs"`.
const NAME_LOD: u32 = 0x0109_a394;
const NAME_NUM_LODS: u32 = 0x0109_a388;

/// Releases a reference-counted object (`InterlockedDecrement` of +4, its
/// vtable +4 deleting it at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `*slot = new` as `NiPointer` assignment does: when it changes, the old
/// object is released and the new one gains a reference.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
    }
}

fn set_ptr(at: u32) -> Ptr<NiTPrimitiveSet> {
    Ptr::new(at)
}

fn array_ptr(at: u32) -> Ptr<NiTArray> {
    Ptr::new(at)
}

/// The element `index` of a pointer array.
fn array_item(e: &Engine, array: u32, index: u32) -> u32 {
    let base = e.get(array_ptr(array), NiTArray::m_pBase);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// The element `index` of a primitive set.
fn set_item(e: &Engine, set: u32, index: u32) -> u32 {
    let base = e.get(set_ptr(set), NiTPrimitiveSet::m_pBase);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// A zeroed `NiTPrimitiveSet` from the small allocator, null when the
/// allocation fails.
fn new_set(e: &mut Engine) -> u32 {
    let set = e
        .call(SMALL_ALLOCATE, &args![SMALL_ALLOCATOR, 0xcu32])
        .u32();
    if set != 0 {
        e.mem.set_u32(set, 0);
        e.mem.set_u32(set + 4, 0);
        e.mem.set_u32(set + 8, 0);
    }
    set
}

/// A `SkinInfo` from the small allocator with its instance pointer cleared
/// (the geometry word is left as it is), null when the allocation fails.
fn new_skin_info(e: &mut Engine) -> u32 {
    let info = e.call(SMALL_ALLOCATE, &args![SMALL_ALLOCATOR, 8u32]).u32();
    if info != 0 {
        e.mem.set_u32(info + SKIN_INFO_INSTANCE, 0);
    }
    info
}

/// The inlined `NiTPrimitiveArray::SetAt(index, value)`: the array grows
/// (`SetSize` to the grow-by plus the index) when the index reaches the
/// capacity; the size becomes `index + 1` past the end, and the count of
/// non-null elements follows the change; then the element is stored.
fn array_set_at_inline(e: &mut Engine, array: u32, index: u32, value: u32) {
    let a = array_ptr(array);
    if index >= u32::from(e.get(a, NiTArray::m_usMaxSize)) {
        let grow_by = u32::from(e.get(a, NiTArray::m_usGrowBy));
        e.call(ARRAY_SET_SIZE, &args![array, grow_by.wrapping_add(index)]);
    }
    if index >= u32::from(e.get(a, NiTArray::m_usSize)) {
        e.set(a, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if value != 0 {
            let count = e.get(a, NiTArray::m_usESize);
            e.set(a, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else if value != 0 {
        if array_item(e, array, index) == 0 {
            let count = e.get(a, NiTArray::m_usESize);
            e.set(a, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else if array_item(e, array, index) != 0 {
        let count = e.get(a, NiTArray::m_usESize);
        e.set(a, NiTArray::m_usESize, count.wrapping_add(0xffff));
    }
    let base = e.get(a, NiTArray::m_pBase);
    e.mem
        .set_u32(base.wrapping_add(index.wrapping_mul(4)), value);
}

/// The inlined `NiTPrimitiveSet::Add(value)`: a full set grows to twice its
/// capacity (1 when empty) into a new block (`00aa1070`; none for a size of
/// 0) holding the used elements, the old block freed (`00aa10f0`); then the
/// value is appended.
fn set_add_inline(e: &mut Engine, set: u32, value: u32) {
    let s = set_ptr(set);
    let alloced = e.get(s, NiTPrimitiveSet::m_uiAlloced);
    if e.get(s, NiTPrimitiveSet::m_uiUsed) == alloced {
        let size = if alloced > 0 {
            alloced.wrapping_add(alloced)
        } else {
            1
        };
        if size != alloced {
            let block = if size != 0 {
                let block = e.call(ALLOCATE, &args![size.wrapping_mul(4)]).u32();
                let mut index = 0u32;
                while index < e.get(s, NiTPrimitiveSet::m_uiUsed) {
                    let item = set_item(e, set, index);
                    e.mem
                        .set_u32(block.wrapping_add(index.wrapping_mul(4)), item);
                    index += 1;
                }
                block
            } else {
                0
            };
            let old = e.get(s, NiTPrimitiveSet::m_pBase);
            e.call(FREE, &args![old]);
            e.set(s, NiTPrimitiveSet::m_pBase, block);
            e.set(s, NiTPrimitiveSet::m_uiAlloced, size);
        }
    }
    let used = e.get(s, NiTPrimitiveSet::m_uiUsed);
    let base = e.get(s, NiTPrimitiveSet::m_pBase);
    e.mem
        .set_u32(base.wrapping_add(used.wrapping_mul(4)), value);
    e.set(s, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
}

/// The inlined clone-map lookup (`NiTPointerMap::GetAt` on the cloning
/// process's map, its first word): the map's hash (vtable +4) picks the
/// bucket (+8), whose items (next +0, key +4, value +8) are compared by the
/// map's key compare (vtable +8). The value of the first match, if any.
fn clone_lookup(e: &mut Engine, cloning: Ptr, key: u32) -> Option<u32> {
    let map = e.mem.u32(cloning.addr());
    let bucket = e.vcall(map, 4, &args![key]).u32();
    let buckets = e.mem.u32(map + 8);
    let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(map, 8, &args![key, item_key]).bool() {
            return Some(e.mem.u32(item + 8));
        }
        item = e.mem.u32(item);
    }
    None
}

/// One 4-byte write through the stream's output (+0x250, its function at
/// +0xC, `cdecl (output, source, 4, component slot, 1)`), the answer added
/// to the output's +4. The caller fills the slots first.
fn write_word(e: &mut Engine, stream: Ptr, source: u32, component: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(output + 0xc);
    let answer = e
        .call(write, &args![output, source, 4u32, component, 1u32])
        .u32();
    let total = e.mem.u32(output + 4).wrapping_add(answer);
    e.mem.set_u32(output + 4, total);
}

/// One 4-byte read through the stream's input (+0x24C, its function at +8,
/// `cdecl (input, destination, 4, component slot, 1)` with the component
/// slot set to 4), the answer added to the input's +4.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32, component: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    e.mem.set_u32(component, 4);
    let answer = e
        .call(read, &args![input, destination, 4u32, component, 1u32])
        .u32();
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
}

/// Appends `text` to a viewer-string array as the exe's inlined `Add`
/// does: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then the string is stored
/// at the old size (through a stack slot holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a53d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::RegisterStreamables` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): false when
/// `NiTimeController::RegisterStreamables` answers false; otherwise every
/// skin info's skin instance (when set) registers itself (vtable +0x54)
/// and the answer is true.
pub fn ni_bone_lod_controller_register_streamables(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    stream: Ptr,
) -> bool {
    if !e
        .call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let skins = this.addr() + SKIN_ARRAY;
    let mut index = 0u32;
    while index < u32::from(e.get(array_ptr(skins), NiTArray::m_usSize)) {
        let set = array_item(e, skins, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let info = set_item(e, set, item);
                let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                if instance != 0 {
                    e.vcall(instance, 0x54, &args![stream]);
                }
                item += 1;
            }
        }
        index += 1;
    }
    true
}

// Translated from 00a53d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::LinkObject` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): `NiTimeController::LinkObject`, then
/// the linked objects in order (`NiStream::GetObjectFromLinkID`): each
/// bone of every LOD's set (stored while the index is below the set's
/// count, as the exe checks), each skin info's geometry and skin instance
/// (a `NiPointer` assignment), and each skinned geometry.
pub fn ni_bone_lod_controller_link_object(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let object = this.addr();
    let bones = object + BONE_ARRAY;
    let mut index = 0u32;
    while index < u32::from(e.get(array_ptr(bones), NiTArray::m_usSize)) {
        let set = array_item(e, bones, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                if item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                    let base = e.get(set_ptr(set), NiTPrimitiveSet::m_pBase);
                    e.mem.set_u32(base.wrapping_add(item * 4), linked);
                }
                item += 1;
            }
        }
        index += 1;
    }
    let skins = object + SKIN_ARRAY;
    let mut index = 0u32;
    while index < u32::from(e.get(array_ptr(skins), NiTArray::m_usSize)) {
        let set = array_item(e, skins, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let info = set_item(e, set, item);
                let geometry = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                e.mem.set_u32(info + SKIN_INFO_GEOMETRY, geometry);
                let instance = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                assign_pointer(e, info + SKIN_INFO_INSTANCE, instance);
                item += 1;
            }
        }
        index += 1;
    }
    let geometries = set_ptr(object + SKIN_SET);
    let mut item = 0u32;
    while item < e.get(geometries, NiTPrimitiveSet::m_uiUsed) {
        let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if item < e.get(geometries, NiTPrimitiveSet::m_uiUsed) {
            let base = e.get(geometries, NiTPrimitiveSet::m_pBase);
            e.mem.set_u32(base.wrapping_add(item * 4), linked);
        }
        item += 1;
    }
}

// Translated from 00a53ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::IsEqual` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the other controller): `NiTimeController::IsEqual`,
/// the same LOD, LOD count, array sizes (16-bit) and skinned-geometry
/// count; then for each LOD both bone sets null or both set with the same
/// count and every bone equal (its `IsEqual`, vtable +0x5C), the same for
/// the skin sets (each info's geometry, then its skin instance), and every
/// skinned geometry equal.
pub fn ni_bone_lod_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    other: Ptr<NiBoneLODController>,
) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let ours = this.addr();
    let theirs = other.addr();
    if e.get(this, NiBoneLODController::m_iLOD) != e.get(other, NiBoneLODController::m_iLOD)
        || e.get(this, NiBoneLODController::m_uiNumLODs)
            != e.get(other, NiBoneLODController::m_uiNumLODs)
    {
        return false;
    }
    let bone_size = e.get(array_ptr(ours + BONE_ARRAY), NiTArray::m_usSize);
    if bone_size != e.get(array_ptr(theirs + BONE_ARRAY), NiTArray::m_usSize) {
        return false;
    }
    if e.get(array_ptr(ours + SKIN_ARRAY), NiTArray::m_usSize)
        != e.get(array_ptr(theirs + SKIN_ARRAY), NiTArray::m_usSize)
    {
        return false;
    }
    if e.get(set_ptr(ours + SKIN_SET), NiTPrimitiveSet::m_uiUsed)
        != e.get(set_ptr(theirs + SKIN_SET), NiTPrimitiveSet::m_uiUsed)
    {
        return false;
    }
    let mut index = 0u32;
    while index < u32::from(e.get(array_ptr(ours + BONE_ARRAY), NiTArray::m_usSize)) {
        let set = array_item(e, ours + BONE_ARRAY, index);
        let other_set = array_item(e, theirs + BONE_ARRAY, index);
        if set == 0 {
            if other_set != 0 {
                return false;
            }
        } else {
            if other_set == 0 {
                return false;
            }
            let count = e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed);
            if count != e.get(set_ptr(other_set), NiTPrimitiveSet::m_uiUsed) {
                return false;
            }
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let bone = set_item(e, set, item);
                let other_bone = set_item(e, other_set, item);
                if !e.vcall(bone, 0x5c, &args![other_bone]).bool() {
                    return false;
                }
                item += 1;
            }
        }
        index += 1;
    }
    let mut index = 0u32;
    while index < u32::from(e.get(array_ptr(ours + SKIN_ARRAY), NiTArray::m_usSize)) {
        let set = array_item(e, ours + SKIN_ARRAY, index);
        let other_set = array_item(e, theirs + SKIN_ARRAY, index);
        if set == 0 {
            if other_set != 0 {
                return false;
            }
        } else {
            if other_set == 0 {
                return false;
            }
            let count = e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed);
            if count != e.get(set_ptr(other_set), NiTPrimitiveSet::m_uiUsed) {
                return false;
            }
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let info = set_item(e, set, item);
                let other_info = set_item(e, other_set, item);
                let geometry = e.mem.u32(info + SKIN_INFO_GEOMETRY);
                let other_geometry = e.mem.u32(other_info + SKIN_INFO_GEOMETRY);
                if !e.vcall(geometry, 0x5c, &args![other_geometry]).bool() {
                    return false;
                }
                let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                let other_instance = e.mem.u32(other_info + SKIN_INFO_INSTANCE);
                if !e.vcall(instance, 0x5c, &args![other_instance]).bool() {
                    return false;
                }
                item += 1;
            }
        }
        index += 1;
    }
    let mut item = 0u32;
    while item < e.get(set_ptr(ours + SKIN_SET), NiTPrimitiveSet::m_uiUsed) {
        let geometry = set_item(e, ours + SKIN_SET, item);
        let other_geometry = set_item(e, theirs + SKIN_SET, item);
        if !e.vcall(geometry, 0x5c, &args![other_geometry]).bool() {
            return false;
        }
        item += 1;
    }
    true
}

// Translated from 00a541f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::SetBoneLOD` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the LOD). False without LODs, for the current
/// LOD, or for one that is neither -1 nor in `0..m_uiNumLODs`. Otherwise
/// every bone of the LOD's set and of all later sets gets flag 2 in its
/// `NiAVObject` flags and the bones of earlier sets lose it; every skinned
/// geometry gets flag 1 and loses flag 2; for a LOD other than -1 each of
/// its skin infos' geometry loses flag 1, gets flag 2 and takes the info's
/// skin instance (a `NiPointer` assignment at +0xBC). The LOD is stored and
/// the answer is true.
pub fn ni_bone_lod_controller_set_bone_lod(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    lod: i32,
) -> bool {
    let count = e.get(this, NiBoneLODController::m_uiNumLODs);
    if count == 0 || e.get(this, NiBoneLODController::m_iLOD) == lod as u32 {
        return false;
    }
    if lod != -1 && (lod <= -1 || lod as u32 >= count) {
        return false;
    }
    let object = this.addr();
    let bones = object + BONE_ARRAY;
    let mut flagged = false;
    let mut index = 0u32;
    while index < u32::from(e.get(array_ptr(bones), NiTArray::m_usSize)) {
        let set = array_item(e, bones, index);
        if set != 0 {
            if index == lod as u32 {
                flagged = true;
            }
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let node = Ptr::<NiAVObject>::new(set_item(e, set, item));
                let flags = e.get(node, NiAVObject::m_uFlags);
                let flags = if flagged { flags | 2 } else { flags & !2 };
                e.set(node, NiAVObject::m_uFlags, flags);
                item += 1;
            }
        }
        index += 1;
    }
    let geometries = object + SKIN_SET;
    let mut item = 0u32;
    while item < e.get(set_ptr(geometries), NiTPrimitiveSet::m_uiUsed) {
        let geometry = Ptr::<NiAVObject>::new(set_item(e, geometries, item));
        item += 1;
        let flags = e.get(geometry, NiAVObject::m_uFlags);
        e.set(geometry, NiAVObject::m_uFlags, (flags & !2) | 1);
    }
    if lod != -1 {
        let set = array_item(e, object + SKIN_ARRAY, lod as u32);
        if set != 0 {
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let info = set_item(e, set, item);
                let geometry = e.mem.u32(info + SKIN_INFO_GEOMETRY);
                let node = Ptr::<NiAVObject>::new(geometry);
                let flags = e.get(node, NiAVObject::m_uFlags);
                e.set(node, NiAVObject::m_uFlags, (flags & !1) | 2);
                let info = set_item(e, set, item);
                let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                assign_pointer(e, geometry + GEOMETRY_SKIN_INSTANCE, instance);
                item += 1;
            }
        }
    }
    e.set(this, NiBoneLODController::m_iLOD, lod as u32);
    true
}

// Translated from 00a54340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::PostLinkObject` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the stream): the base `PostLinkObject`
/// (`00a89af0`), then the stored LOD applied again
/// ([`ni_bone_lod_controller_set_bone_lod`] after setting the LOD to -1).
pub fn ni_bone_lod_controller_post_link_object(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    let lod = e.get(this, NiBoneLODController::m_iLOD);
    e.set(this, NiBoneLODController::m_iLOD, u32::MAX);
    ni_bone_lod_controller_set_bone_lod(e, this, lod as i32);
}

// Translated from 00a54440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::ProcessClone` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the cloning process):
/// `NiTimeController::ProcessClone`, then this controller's clone from the
/// cloning process's map (`00a53c30` into a stack word the exe does not
/// set first) is filled. For each LOD a new bone set (null when this one
/// has none) goes into the clone's bone array (grown when needed, then
/// `SetAt`, `00ba3aa0`), holding each bone's clone from the map (the bone
/// itself when it has none; full sets grow by `NiTSet::Realloc`); a new
/// skin set (stored the inlined way) holds new skin infos with each
/// geometry's clone (or the geometry) and a clone of each skin instance
/// (vtable +0x48, a `NiPointer` assignment), after which the original
/// instance's `ProcessClone` (vtable +0x68) runs; every skinned geometry's
/// clone (or the geometry) joins the clone's set. Finally the clone's LOD
/// is applied again ([`ni_bone_lod_controller_set_bone_lod`] after
/// setting it to -1).
pub fn ni_bone_lod_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    cloning: Ptr,
) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    let object = this.addr();
    // The frame: the clone +0, the new set handed to `SetAt` +4.
    e.with_stack(8, |e, frame| {
        let clone_slot = frame.addr();
        let set_slot = clone_slot + 4;
        let map = e.mem.u32(cloning.addr());
        e.call(MAP_GET_AT, &args![map, this, clone_slot]);
        let clone = e.mem.u32(clone_slot);
        let clone_bones = clone.wrapping_add(BONE_ARRAY);
        let mut index = 0u32;
        while index < u32::from(e.get(array_ptr(object + BONE_ARRAY), NiTArray::m_usSize)) {
            let old = array_item(e, object + BONE_ARRAY, index);
            let new = if old != 0 { new_set(e) } else { 0 };
            e.mem.set_u32(set_slot, new);
            let a = array_ptr(clone_bones);
            if index >= u32::from(e.get(a, NiTArray::m_usMaxSize)) {
                let grow_by = u32::from(e.get(a, NiTArray::m_usGrowBy));
                e.call(
                    ARRAY_SET_SIZE,
                    &args![clone_bones, grow_by.wrapping_add(index)],
                );
            }
            e.call(ARRAY_SET_AT, &args![clone_bones, index, set_slot]);
            if old != 0 {
                let mut item = 0u32;
                while item < e.get(set_ptr(old), NiTPrimitiveSet::m_uiUsed) {
                    let bone = set_item(e, old, item);
                    let value = clone_lookup(e, cloning, bone).unwrap_or(bone);
                    let s = set_ptr(new);
                    let alloced = e.get(s, NiTPrimitiveSet::m_uiAlloced);
                    if e.get(s, NiTPrimitiveSet::m_uiUsed) == alloced {
                        let size = if alloced > 0 {
                            alloced.wrapping_add(alloced)
                        } else {
                            1
                        };
                        e.call(SET_REALLOC, &args![new, size]);
                    }
                    let used = e.get(s, NiTPrimitiveSet::m_uiUsed);
                    let base = e.get(s, NiTPrimitiveSet::m_pBase);
                    e.mem
                        .set_u32(base.wrapping_add(used.wrapping_mul(4)), value);
                    e.set(s, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
                    item += 1;
                }
            }
            index += 1;
        }
        let mut index = 0u32;
        while index < u32::from(e.get(array_ptr(object + SKIN_ARRAY), NiTArray::m_usSize)) {
            let old = array_item(e, object + SKIN_ARRAY, index);
            if old != 0 {
                let new = new_set(e);
                array_set_at_inline(e, clone.wrapping_add(SKIN_ARRAY), index, new);
                let mut item = 0u32;
                while item < e.get(set_ptr(old), NiTPrimitiveSet::m_uiUsed) {
                    let info = set_item(e, old, item);
                    let new_info = new_skin_info(e);
                    set_add_inline(e, new, new_info);
                    let geometry = e.mem.u32(info + SKIN_INFO_GEOMETRY);
                    let value = match clone_lookup(e, cloning, geometry) {
                        Some(found) => found,
                        None => e.mem.u32(info + SKIN_INFO_GEOMETRY),
                    };
                    e.mem.set_u32(new_info + SKIN_INFO_GEOMETRY, value);
                    let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                    let instance_clone = e.vcall(instance, 0x48, &args![cloning]).u32();
                    assign_pointer(e, new_info + SKIN_INFO_INSTANCE, instance_clone);
                    let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                    e.vcall(instance, 0x68, &args![cloning]);
                    item += 1;
                }
            } else {
                array_set_at_inline(e, clone.wrapping_add(SKIN_ARRAY), index, 0);
            }
            index += 1;
        }
        let mut item = 0u32;
        while item < e.get(set_ptr(object + SKIN_SET), NiTPrimitiveSet::m_uiUsed) {
            let geometry = set_item(e, object + SKIN_SET, item);
            let value = clone_lookup(e, cloning, geometry).unwrap_or(geometry);
            set_add_inline(e, clone.wrapping_add(SKIN_SET), value);
            item += 1;
        }
        let clone = Ptr::<NiBoneLODController>::new(clone);
        let lod = e.get(clone, NiBoneLODController::m_iLOD);
        e.set(clone, NiBoneLODController::m_iLOD, u32::MAX);
        ni_bone_lod_controller_set_bone_lod(e, clone, lod as i32);
    });
}

// Translated from 00a549d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::SaveBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): `NiTimeController::SaveBinary`, then
/// 4-byte writes of the LOD, the LOD count and the bone array size; per
/// LOD the bone set's count (0 for none) and each bone's link (the
/// stream's vtable +0x30); the skin array size, per LOD the skin set's
/// count (0 for none) and each info's geometry and skin instance links;
/// the skinned-geometry count and their links. The counts are written from
/// two stack words and the stream argument's own word, as the exe swaps
/// them between value and component size.
pub fn ni_bone_lod_controller_save_binary(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    let object = this.addr();
    let link = |e: &mut Engine, linked: u32| {
        e.vcall(stream.addr(), STREAM_SAVE_LINK_SLOT, &args![linked]);
    };
    // The frame: two locals +0 and +4, then the stream argument's word
    // (+0xC, past the return address).
    e.with_stack(0x10, |e, frame| {
        let low = frame.addr();
        let high = low + 4;
        let argument = low + 0xc;
        e.mem.set_u32(argument, stream.addr());
        e.mem.set_u32(argument, 4);
        write_word(e, stream, object + 0x34, argument);
        e.mem.set_u32(argument, 4);
        write_word(e, stream, object + 0x38, argument);
        let bones = object + BONE_ARRAY;
        let size = u32::from(e.get(array_ptr(bones), NiTArray::m_usSize));
        e.mem.set_u32(low, size);
        e.mem.set_u32(argument, 4);
        write_word(e, stream, low, argument);
        e.mem.set_u32(argument, 0);
        let mut index = 0u32;
        while index < u32::from(e.get(array_ptr(bones), NiTArray::m_usSize)) {
            let set = array_item(e, bones, index);
            if set != 0 {
                let used = e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed);
                e.mem.set_u32(high, used);
                e.mem.set_u32(low, 4);
                write_word(e, stream, high, low);
                let mut item = 0u32;
                while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                    let bone = set_item(e, set, item);
                    link(e, bone);
                    item += 1;
                }
            } else {
                e.mem.set_u32(low, 0);
                e.mem.set_u32(high, 4);
                write_word(e, stream, low, high);
            }
            index += 1;
            e.mem.set_u32(argument, index);
        }
        let skins = object + SKIN_ARRAY;
        let size = u32::from(e.get(array_ptr(skins), NiTArray::m_usSize));
        e.mem.set_u32(high, size);
        e.mem.set_u32(argument, 4);
        write_word(e, stream, high, argument);
        e.mem.set_u32(argument, 0);
        let mut index = 0u32;
        while index < u32::from(e.get(array_ptr(skins), NiTArray::m_usSize)) {
            let set = array_item(e, skins, index);
            e.mem.set_u32(high, 4);
            if set != 0 {
                let used = e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed);
                e.mem.set_u32(low, used);
                write_word(e, stream, low, high);
                let mut item = 0u32;
                while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                    let info = set_item(e, set, item);
                    e.mem.set_u32(high, info);
                    let geometry = e.mem.u32(info + SKIN_INFO_GEOMETRY);
                    link(e, geometry);
                    let info = e.mem.u32(high);
                    let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                    link(e, instance);
                    item += 1;
                }
            } else {
                e.mem.set_u32(low, 0);
                write_word(e, stream, low, high);
            }
            index += 1;
            e.mem.set_u32(argument, index);
        }
        let geometries = set_ptr(object + SKIN_SET);
        let count = e.get(geometries, NiTPrimitiveSet::m_uiUsed);
        e.mem.set_u32(high, count);
        e.mem.set_u32(argument, 4);
        write_word(e, stream, high, argument);
        let mut item = 0u32;
        while item < e.get(geometries, NiTPrimitiveSet::m_uiUsed) {
            let geometry = set_item(e, object + SKIN_SET, item);
            link(e, geometry);
            item += 1;
        }
    });
}

// Translated from 00a54c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::LoadBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): `NiTimeController::LoadBinary`, then
/// 4-byte reads of the LOD, the LOD count and the bone set count; per bone
/// set a new set (null when the allocation fails) stored the inlined
/// `SetAt` way, its bone count read and, per bone, `this` added as a
/// placeholder and a link id read (`00a645b0`); the skin set count, per
/// skin set a new set, its info count and per info a new `SkinInfo` added
/// and two link ids read; the skinned-geometry count and per geometry
/// `this` added to the controller's own set and a link id read.
pub fn ni_bone_lod_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    // The frame: the inner count +0, the bone set count +4, the skin set
    // count +8, the geometry count +0xC, the component-size word +0x10.
    e.with_stack(0x14, |e, frame| {
        let inner = frame.addr();
        let bone_count = inner + 4;
        let skin_count = inner + 8;
        let geometry_count = inner + 0xc;
        let component = inner + 0x10;
        read_word(e, stream, object + 0x34, component);
        read_word(e, stream, object + 0x38, component);
        e.mem.set_u32(bone_count, 0);
        read_word(e, stream, bone_count, component);
        let mut index = 0u32;
        while index < e.mem.u32(bone_count) {
            let set = new_set(e);
            array_set_at_inline(e, object + BONE_ARRAY, index, set);
            e.mem.set_u32(inner, 0);
            read_word(e, stream, inner, component);
            let mut item = 0u32;
            while item < e.mem.u32(inner) {
                set_add_inline(e, set, object);
                e.call(STREAM_READ_LINK_ID, &args![stream]);
                item += 1;
            }
            index += 1;
        }
        e.mem.set_u32(skin_count, 0);
        read_word(e, stream, skin_count, component);
        let mut index = 0u32;
        while index < e.mem.u32(skin_count) {
            let set = new_set(e);
            array_set_at_inline(e, object + SKIN_ARRAY, index, set);
            e.mem.set_u32(inner, 0);
            read_word(e, stream, inner, component);
            let mut item = 0u32;
            while item < e.mem.u32(inner) {
                let info = new_skin_info(e);
                e.mem.set_u32(component, info);
                set_add_inline(e, set, info);
                e.call(STREAM_READ_LINK_ID, &args![stream]);
                e.call(STREAM_READ_LINK_ID, &args![stream]);
                item += 1;
            }
            index += 1;
        }
        e.mem.set_u32(geometry_count, 0);
        read_word(e, stream, geometry_count, component);
        let mut item = 0u32;
        while item < e.mem.u32(geometry_count) {
            set_add_inline(e, object + SKIN_SET, object);
            e.call(STREAM_READ_LINK_ID, &args![stream]);
            item += 1;
        }
    });
}

// Translated from 00a550b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoneLODController` constructor (`thiscall`, no words; the engine
/// map gives it no name): the `NiTimeController` constructor body
/// (`00a6d220`), the vtable, LOD -1, no LODs, both arrays empty with their
/// vtables and a grow-by of 1, and an empty skinned-geometry set. Returns
/// `this`.
pub fn fn_00a550b0(e: &mut Engine, this: Ptr<NiBoneLODController>) -> Ptr<NiBoneLODController> {
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![this]);
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    e.set(this, NiBoneLODController::m_iLOD, u32::MAX);
    e.set(this, NiBoneLODController::m_uiNumLODs, 0);
    let bones = array_ptr(object + BONE_ARRAY);
    e.set(bones, NiTArray::m_pBase, 0);
    e.mem.set_u32(object + BONE_ARRAY, BONE_ARRAY_VTABLE);
    e.set(bones, NiTArray::m_usGrowBy, 1);
    e.set(bones, NiTArray::m_usESize, 0);
    e.set(bones, NiTArray::m_usMaxSize, 0);
    e.set(bones, NiTArray::m_usSize, 0);
    let skins = array_ptr(object + SKIN_ARRAY);
    e.set(skins, NiTArray::m_usGrowBy, 1);
    e.set(skins, NiTArray::m_pBase, 0);
    e.set(skins, NiTArray::m_usMaxSize, 0);
    e.set(skins, NiTArray::m_usSize, 0);
    e.mem.set_u32(object + SKIN_ARRAY, SKIN_ARRAY_VTABLE);
    e.set(skins, NiTArray::m_usESize, 0);
    let geometries = set_ptr(object + SKIN_SET);
    e.set(geometries, NiTPrimitiveSet::m_pBase, 0);
    e.set(geometries, NiTPrimitiveSet::m_uiAlloced, 0);
    e.set(geometries, NiTPrimitiveSet::m_uiUsed, 0);
    this
}

// Translated from 00a55130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::TargetIsRequiredType` (Xbox PDB, matched by the
/// engine map; `thiscall`, no words): true when the target's `NiRTTI`
/// (vtable +8) or one of its bases (+4) is `NiNode`'s; false without a
/// target.
pub fn ni_bone_lod_controller_target_is_required_type(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
) -> bool {
    let target = e.get(
        Ptr::<NiTimeController>::new(this.addr()),
        NiTimeController::m_pkTarget,
    );
    if target == 0 {
        return false;
    }
    let mut rtti = e.vcall(target, 8, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_NODE_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00a55160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::Update` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the time, not read): the LOD at `011a90c4`, unless
/// it is -1 or the current one, limited to `m_uiNumLODs - 1` when above
/// the count (signed compare), is applied
/// ([`ni_bone_lod_controller_set_bone_lod`], a tail jump that reuses the
/// time's stack word) when it differs from the current LOD.
pub fn ni_bone_lod_controller_update(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    _unused_time: f32,
) {
    let mut lod: i32 = e.global(GLOBAL_LOD);
    if lod == -1 {
        return;
    }
    let current = e.get(this, NiBoneLODController::m_iLOD) as i32;
    if lod == current {
        return;
    }
    let count = e.get(this, NiBoneLODController::m_uiNumLODs) as i32;
    if lod > count {
        lod = count.wrapping_sub(1);
    }
    if lod == current {
        return;
    }
    ni_bone_lod_controller_set_bone_lod(e, this, lod);
}

// Translated from 00a55190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the controller's sets (`thiscall`, no words; the engine map gives
/// it no name; the destructor's first step): each bone set's block
/// (`00aa10f0`) and the set itself (`00aa4060` on `011f6238`), the bone
/// array cleared (elements nulled, size and count 0); each skin set's infos
/// (their skin instances released, the infos freed), its block and the set,
/// the skin array cleared the same way; the skinned-geometry count 0.
pub fn fn_00a55190(e: &mut Engine, this: Ptr<NiBoneLODController>) {
    let object = this.addr();
    let bones = object + BONE_ARRAY;
    let b = array_ptr(bones);
    let mut index = 0u32;
    while index < u32::from(e.get(b, NiTArray::m_usSize)) {
        let set = array_item(e, bones, index);
        if set != 0 {
            let block = e.get(set_ptr(set), NiTPrimitiveSet::m_pBase);
            e.call(FREE, &args![block]);
            e.call(SMALL_FREE, &args![SMALL_ALLOCATOR, set]);
        }
        index += 1;
    }
    let mut index = 0u16;
    while index < e.get(b, NiTArray::m_usSize) {
        let base = e.get(b, NiTArray::m_pBase);
        e.mem.set_u32(base + 4 * u32::from(index), 0);
        index = index.wrapping_add(1);
    }
    e.set(b, NiTArray::m_usSize, 0);
    e.set(b, NiTArray::m_usESize, 0);
    let skins = object + SKIN_ARRAY;
    let s = array_ptr(skins);
    let mut index = 0u32;
    while index < u32::from(e.get(s, NiTArray::m_usSize)) {
        let set = array_item(e, skins, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.get(set_ptr(set), NiTPrimitiveSet::m_uiUsed) {
                let info = set_item(e, set, item);
                if info != 0 {
                    let instance = e.mem.u32(info + SKIN_INFO_INSTANCE);
                    if instance != 0 {
                        release(e, instance);
                    }
                    e.call(SMALL_FREE, &args![SMALL_ALLOCATOR, info]);
                }
                item += 1;
            }
            let block = e.get(set_ptr(set), NiTPrimitiveSet::m_pBase);
            e.call(FREE, &args![block]);
            e.call(SMALL_FREE, &args![SMALL_ALLOCATOR, set]);
        }
        index += 1;
    }
    let mut index = 0u16;
    while index < e.get(s, NiTArray::m_usSize) {
        let base = e.get(s, NiTArray::m_pBase);
        e.mem.set_u32(base + 4 * u32::from(index), 0);
        index = index.wrapping_add(1);
    }
    e.set(s, NiTArray::m_usSize, 0);
    e.set(s, NiTArray::m_usESize, 0);
    e.set(set_ptr(object + SKIN_SET), NiTPrimitiveSet::m_uiUsed, 0);
}

// Translated from 00a552c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::CreateClone` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the cloning process): a 0x68-byte block
/// constructed by [`fn_00a550b0`] (null when the allocation fails), the
/// `NiTimeController` member copy (`00a6cfe0`) into it, then the LOD and
/// the LOD count (into address 0 when the allocation failed, as the exe
/// does). Answers the clone.
pub fn ni_bone_lod_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    cloning: u32,
) -> Ptr<NiBoneLODController> {
    let block = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    let clone = if block != 0 {
        fn_00a550b0(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    };
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, clone, cloning]);
    let lod = e.get(this, NiBoneLODController::m_iLOD);
    e.set(clone, NiBoneLODController::m_iLOD, lod);
    let count = e.get(this, NiBoneLODController::m_uiNumLODs);
    e.set(clone, NiBoneLODController::m_uiNumLODs, count);
    clone
}

// Translated from 00a55300 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; registered for
/// `"NiBoneLODController"` by `00a2d8a0`; Ghidra has no function here and
/// the engine map no name): a 0x68-byte block constructed by
/// [`fn_00a550b0`] (a tail jump), null when the allocation fails.
pub fn fn_00a55300(e: &mut Engine) -> Ptr<NiBoneLODController> {
    let block = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00a550b0(e, Ptr::new(block))
}

// Translated from 00a55320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::GetViewerStrings` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the strings array): the class name (the
/// first word of the `NiRTTI` record), `m_iLOD` (signed) and `m_uiNumLODs`,
/// each appended (no base class strings).
pub fn ni_bone_lod_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    strings: Ptr,
) {
    let class_name: u32 = e.global(NI_BONE_LOD_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let lod = e.get(this, NiBoneLODController::m_iLOD);
    let text = e
        .call(NI_GET_VIEWER_STRING_SIGNED, &args![NAME_LOD, lod])
        .u32();
    push_viewer_string(e, strings, text);
    let count = e.get(this, NiBoneLODController::m_uiNumLODs);
    let text = e
        .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![NAME_NUM_LODS, count])
        .u32();
    push_viewer_string(e, strings, text);
}

// Translated from 00a553f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoneLODController::_scalar_deleting_destructor_` (Xbox PDB, matched
/// by the engine map; `thiscall`, one word: flags): the vtable, the sets
/// freed ([`fn_00a55190`]), the skinned-geometry block, then each array's
/// `NiTArray` vtable and block (`00aa10f0`), the `NiTimeController`
/// destructor body (`00a6d290`) and with flag bit 0 the sized delete of
/// 0x68 bytes. Returns `this`.
pub fn ni_bone_lod_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBoneLODController>,
    flags: u32,
) -> Ptr<NiBoneLODController> {
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    fn_00a55190(e, this);
    let geometries = e.get(set_ptr(object + SKIN_SET), NiTPrimitiveSet::m_pBase);
    e.call(FREE, &args![geometries]);
    let skins = e.get(array_ptr(object + SKIN_ARRAY), NiTArray::m_pBase);
    e.mem.set_u32(object + SKIN_ARRAY, SKIN_NI_T_ARRAY_VTABLE);
    e.call(FREE, &args![skins]);
    let bones = e.get(array_ptr(object + BONE_ARRAY), NiTArray::m_pBase);
    e.mem.set_u32(object + BONE_ARRAY, BONE_NI_T_ARRAY_VTABLE);
    e.call(FREE, &args![bones]);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, OBJECT_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type C = NiBoneLODController;
    vec![
        entry!(0x00a55120, ni_bone_lod_controller_get_rtti() -> Ptr),
        entry!(
            0x00a5_3d00,
            ni_bone_lod_controller_register_streamables(Ptr<C>, Ptr) -> bool
        ),
        entry!(0x00a5_3d70, ni_bone_lod_controller_link_object(Ptr<C>, Ptr)),
        entry!(
            0x00a5_3ea0,
            ni_bone_lod_controller_is_equal(Ptr<C>, Ptr<C>) -> bool
        ),
        entry!(
            0x00a5_41f0,
            ni_bone_lod_controller_set_bone_lod(Ptr<C>, i32) -> bool
        ),
        entry!(
            0x00a5_4340,
            ni_bone_lod_controller_post_link_object(Ptr<C>, Ptr)
        ),
        entry!(
            0x00a5_4440,
            ni_bone_lod_controller_process_clone(Ptr<C>, Ptr)
        ),
        entry!(0x00a5_49d0, ni_bone_lod_controller_save_binary(Ptr<C>, Ptr)),
        entry!(0x00a5_4c40, ni_bone_lod_controller_load_binary(Ptr<C>, Ptr)),
        entry!(0x00a5_50b0, fn_00a550b0(Ptr<C>) -> Ptr<C>),
        entry!(
            0x00a5_5130,
            ni_bone_lod_controller_target_is_required_type(Ptr<C>) -> bool
        ),
        entry!(0x00a5_5160, ni_bone_lod_controller_update(Ptr<C>, f32)),
        entry!(0x00a5_5190, fn_00a55190(Ptr<C>)),
        entry!(
            0x00a5_52c0,
            ni_bone_lod_controller_create_clone(Ptr<C>, u32) -> Ptr<C>
        ),
        entry!(0x00a5_5300, fn_00a55300() -> Ptr<C>),
        entry!(
            0x00a5_5320,
            ni_bone_lod_controller_get_viewer_strings(Ptr<C>, Ptr)
        ),
        entry!(
            0x00a5_53f0,
            ni_bone_lod_controller_scalar_deleting_destructor(Ptr<C>, u32) -> Ptr<C>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bone_lod_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a55120, &args![]).u32(),
            NI_BONE_LOD_CONTROLLER_RTTI
        );
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    type C = NiBoneLODController;

    const OBJECT_VTABLE: u32 = 0x0002_0000;
    const DELETE_SLOT: u32 = 0x0001_0500;
    const CHILD: u32 = 0x0001_0510;
    const CLONE_SLOT: u32 = 0x0001_0520;
    const PROCESS_SLOT: u32 = 0x0001_0530;
    const RTTI_SLOT: u32 = 0x0001_0540;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles for the allocators, reference counts and array helpers, and
    /// one object vtable: +4 deletes, +8 answers the object's word at +0x10
    /// (its RTTI), +0x48 answers the word at +0x14 (its clone), +0x54 /
    /// +0x5C answer whether the argument is the object's word at +0xC,
    /// +0x68 does nothing.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SMALL_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            e.mem.write(block, &vec![0xcd; a[1] as usize]);
            word(block)
        });
        e.register(SMALL_FREE, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| word(e.mem.alloc(a[0].max(4))));
        e.register(FREE, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            let array = a[0];
            let block = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(array + 4);
            for i in 0..u32::from(e.mem.u16(array + 8)) {
                let item = e.mem.u32(old + 4 * i);
                e.mem.set_u32(block + 4 * i, item);
            }
            e.mem.set_u32(array + 4, block);
            e.mem.set_u16(array + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            if a[1] >= u32::from(e.mem.u16(a[0] + 0xa)) {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        e.register(SET_REALLOC, |e, a| {
            let block = e.mem.alloc(4 * a[1]);
            for i in 0..e.mem.u32(a[0] + 8) {
                let item = e.mem.u32(e.mem.u32(a[0]) + 4 * i);
                e.mem.set_u32(block + 4 * i, item);
            }
            e.mem.set_u32(a[0], block);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(DELETE_SLOT, |_, _| Ret::default());
        e.register(CHILD, |e, a| word(u32::from(e.mem.u32(a[0] + 0xc) == a[1])));
        e.register(CLONE_SLOT, |e, a| word(e.mem.u32(a[0] + 0x14)));
        e.register(PROCESS_SLOT, |_, _| Ret::default());
        e.register(RTTI_SLOT, |e, a| word(e.mem.u32(a[0] + 0x10)));
        let mut slots = vec![0u32; 0x1b];
        slots[1] = DELETE_SLOT;
        slots[2] = RTTI_SLOT;
        slots[0x48 / 4] = CLONE_SLOT;
        slots[0x54 / 4] = CHILD;
        slots[0x5c / 4] = CHILD;
        slots[0x68 / 4] = PROCESS_SLOT;
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    /// A counted object (vtable above, one reference) as large as a
    /// geometry, so its flags (+0x30) and skin instance (+0xBC) exist.
    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0xc0);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn set(e: &mut Engine, items: &[u32]) -> u32 {
        let set = e.mem.alloc(0xc);
        let block = e.mem.alloc(4 * items.len() as u32 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *item);
        }
        e.mem.set_u32(set, block);
        e.mem.set_u32(set + 4, items.len() as u32);
        e.mem.set_u32(set + 8, items.len() as u32);
        set
    }

    fn fill_array(e: &mut Engine, array: u32, items: &[u32]) {
        let block = e.mem.alloc(4 * items.len() as u32 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *item);
        }
        e.mem.set_u32(array + 4, block);
        e.mem.set_u16(array + 8, items.len() as u16);
        e.mem.set_u16(array + 0xa, items.len() as u16);
        e.mem.set_u16(array + 0xe, 1);
    }

    fn skin_info(e: &mut Engine, geometry: u32, instance: u32) -> u32 {
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, geometry);
        e.mem.set_u32(info + 4, instance);
        info
    }

    /// A controller with bone sets `bones`, skin sets `skins` and skinned
    /// geometry `geometries`, `lods` LODs at LOD `lod`.
    fn controller(
        e: &mut Engine,
        lods: u32,
        lod: u32,
        bones: &[u32],
        skins: &[u32],
        geometries: &[u32],
    ) -> Ptr<C> {
        let this = e.new_object::<C>();
        e.mem.set_u32(this.addr(), OBJECT_VTABLE);
        e.set(this, C::m_uiNumLODs, lods);
        e.set(this, C::m_iLOD, lod);
        fill_array(e, this.addr() + BONE_ARRAY, bones);
        fill_array(e, this.addr() + SKIN_ARRAY, skins);
        let geometry_set = set(e, geometries);
        for offset in [0u32, 4, 8] {
            let value = e.mem.u32(geometry_set + offset);
            e.mem.set_u32(this.addr() + SKIN_SET + offset, value);
        }
        this
    }

    #[test]
    fn register_streamables_registers_the_skin_instances() {
        let mut e = engine();
        let instance = object(&mut e);
        let info = skin_info(&mut e, 0x6000, instance);
        let empty = skin_info(&mut e, 0x6000, 0);
        let skins = set(&mut e, &[info, empty]);
        let this = controller(&mut e, 1, 0, &[], &[skins, 0], &[]);
        e.register(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, _| word(0));
        assert!(!e.call(0x00a5_3d00, &args![this, 0x700u32]).bool());
        assert!(calls(&e, CHILD).is_empty());
        e.register(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, _| word(1));
        assert!(e.call(0x00a5_3d00, &args![this, 0x700u32]).bool());
        assert_eq!(calls(&e, CHILD), vec![vec![instance, 0x700]]);
    }

    #[test]
    fn link_object_resolves_bones_skins_and_geometry_in_order() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        let next = std::rc::Rc::new(std::cell::Cell::new(0x0900_0000u32));
        let counter = next.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            counter.set(counter.get() + 0x10);
            word(counter.get())
        });
        let old_instance = object(&mut e);
        let bones = set(&mut e, &[1, 2]);
        let info = skin_info(&mut e, 0, old_instance);
        let skins = set(&mut e, &[info]);
        let this = controller(&mut e, 1, 0, &[bones, 0], &[skins], &[3]);
        // The instance the link resolves to is a counted object.
        let instance = 0x0900_0040u32;
        e.map(0x0900_0000, 0x100);
        e.mem.set_u32(instance, OBJECT_VTABLE);
        e.mem.set_u32(instance + 4, 1);
        e.call(0x00a5_3d70, &args![this, 0x700u32]);
        assert_eq!(set_item(&e, bones, 0), 0x0900_0010);
        assert_eq!(set_item(&e, bones, 1), 0x0900_0020);
        assert_eq!(e.mem.u32(info), 0x0900_0030);
        assert_eq!(e.mem.u32(info + 4), 0x0900_0040);
        assert_eq!(set_item(&e, this.addr() + SKIN_SET, 0), 0x0900_0050);
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![old_instance]]);
        assert_eq!(calls(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 5);
    }

    #[test]
    fn is_equal_walks_every_set() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| word(1));
        let bone = object(&mut e);
        let other_bone = object(&mut e);
        e.mem.set_u32(bone + 0xc, other_bone);
        let geometry = object(&mut e);
        let instance = object(&mut e);
        let other_geometry = object(&mut e);
        let other_instance = object(&mut e);
        e.mem.set_u32(geometry + 0xc, other_geometry);
        e.mem.set_u32(instance + 0xc, other_instance);
        let ours_bones = set(&mut e, &[bone]);
        let theirs_bones = set(&mut e, &[other_bone]);
        let ours_info = skin_info(&mut e, geometry, instance);
        let theirs_info = skin_info(&mut e, other_geometry, other_instance);
        let ours_skins = set(&mut e, &[ours_info]);
        let theirs_skins = set(&mut e, &[theirs_info]);
        let this = controller(&mut e, 2, 0, &[ours_bones, 0], &[ours_skins], &[geometry]);
        let other = controller(
            &mut e,
            2,
            0,
            &[theirs_bones, 0],
            &[theirs_skins],
            &[other_geometry],
        );
        assert!(e.call(0x00a5_3ea0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, CHILD),
            vec![
                vec![bone, other_bone],
                vec![geometry, other_geometry],
                vec![instance, other_instance],
                vec![geometry, other_geometry]
            ]
        );
        // An unequal skin instance.
        e.mem.set_u32(instance + 0xc, 0);
        assert!(!e.call(0x00a5_3ea0, &args![this, other]).bool());
        e.mem.set_u32(instance + 0xc, other_instance);
        // A missing bone set on one side.
        let theirs_base = e.get(array_ptr(other.addr() + BONE_ARRAY), NiTArray::m_pBase);
        e.mem.set_u32(theirs_base + 4, ours_bones);
        assert!(!e.call(0x00a5_3ea0, &args![this, other]).bool());
        e.mem.set_u32(theirs_base + 4, 0);
        // Header differences.
        e.set(other, C::m_iLOD, 1);
        assert!(!e.call(0x00a5_3ea0, &args![this, other]).bool());
        e.set(other, C::m_iLOD, 0);
        e.mem.set_u32(other.addr() + SKIN_SET + 8, 0);
        assert!(!e.call(0x00a5_3ea0, &args![this, other]).bool());
        e.mem.set_u32(other.addr() + SKIN_SET + 8, 1);
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_3ea0, &args![this, other]).bool());
    }

    #[test]
    fn set_bone_lod_flags_bones_and_swaps_skins() {
        let mut e = engine();
        let bones: Vec<u32> = (0..3).map(|_| object(&mut e)).collect();
        let sets: Vec<u32> = bones.iter().map(|b| set(&mut e, &[*b])).collect();
        let skinned = object(&mut e);
        let instance = object(&mut e);
        let info = skin_info(&mut e, skinned, instance);
        let skins = set(&mut e, &[info]);
        let this = controller(&mut e, 3, 0, &sets, &[0, skins, 0], &[skinned]);
        for bone in &bones {
            e.mem.set_u32(bone + 0x30, 0x12);
        }
        e.mem.set_u32(skinned + 0x30, 0x10);
        // Rejected: the current LOD, out of range, below -1.
        for lod in [0i32, 3, -2] {
            assert!(!e.call(0x00a5_41f0, &args![this, lod]).bool());
        }
        assert!(e.call(0x00a5_41f0, &args![this, 1i32]).bool());
        assert_eq!(e.get(this, C::m_iLOD), 1);
        assert_eq!(e.mem.u32(bones[0] + 0x30), 0x10);
        assert_eq!(e.mem.u32(bones[1] + 0x30), 0x12);
        assert_eq!(e.mem.u32(bones[2] + 0x30), 0x12);
        // The geometry: flag 1 from the set pass, then the skin swap.
        assert_eq!(e.mem.u32(skinned + 0x30), 0x12);
        assert_eq!(e.mem.u32(skinned + GEOMETRY_SKIN_INSTANCE), instance);
        assert_eq!(e.mem.u32(instance + 4), 2);
        // LOD -1: every bone loses flag 2, the geometry keeps flag 1.
        assert!(e.call(0x00a5_41f0, &args![this, -1i32]).bool());
        for bone in &bones {
            assert_eq!(e.mem.u32(bone + 0x30), 0x10);
        }
        assert_eq!(e.mem.u32(skinned + 0x30), 0x11);
        // No LODs at all.
        e.set(this, C::m_uiNumLODs, 0);
        assert!(!e.call(0x00a5_41f0, &args![this, 0i32]).bool());
    }

    #[test]
    fn post_link_object_applies_the_lod_again() {
        let mut e = engine();
        e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
        let bone = object(&mut e);
        let bones = set(&mut e, &[bone]);
        let this = controller(&mut e, 2, 1, &[0, bones], &[], &[]);
        e.call(0x00a5_4340, &args![this, 0x700u32]);
        assert_eq!(
            calls(&e, BASE_POST_LINK_OBJECT),
            vec![vec![this.addr(), 0x700]]
        );
        assert_eq!(e.get(this, C::m_iLOD), 1);
        assert_eq!(e.mem.u32(bone + 0x30) & 2, 2);
    }

    #[test]
    fn process_clone_fills_the_clone_from_the_map() {
        const MAP_HASH: u32 = 0x0001_0550;
        const MAP_EQUAL: u32 = 0x0001_0560;
        let mut e = engine();
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(MAP_HASH, |_, _| word(0));
        e.register(MAP_EQUAL, |_, a| word(u32::from(a[1] == a[2])));
        let bone = object(&mut e);
        let bone_clone = object(&mut e);
        let unmapped_bone = object(&mut e);
        let geometry = object(&mut e);
        let geometry_clone = object(&mut e);
        let instance = object(&mut e);
        let instance_clone = object(&mut e);
        e.mem.set_u32(instance + 0x14, instance_clone);
        let bones = set(&mut e, &[bone, unmapped_bone]);
        let info = skin_info(&mut e, geometry, instance);
        let skins = set(&mut e, &[info]);
        let this = controller(&mut e, 2, 1, &[bones, 0], &[0, skins], &[geometry]);
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        let block = e.mem.alloc(OBJECT_SIZE);
        let clone = e.call(0x00a5_50b0, &args![block]).u32();
        e.set(Ptr::<C>::new(clone), C::m_iLOD, 1);
        e.set(Ptr::<C>::new(clone), C::m_uiNumLODs, 2);
        // The map: one bucket holding this -> clone, bone -> bone_clone,
        // geometry -> geometry_clone.
        let map = e.mem.alloc(0x10);
        e.put_vtable(0x0002_1000, &[0, MAP_HASH, MAP_EQUAL]);
        e.mem.set_u32(map, 0x0002_1000);
        let mut head = 0u32;
        for (key, value) in [
            (this.addr(), clone),
            (bone, bone_clone),
            (geometry, geometry_clone),
        ] {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, head);
            e.mem.set_u32(item + 4, key);
            e.mem.set_u32(item + 8, value);
            head = item;
        }
        let buckets = e.mem.alloc(4);
        e.mem.set_u32(buckets, head);
        e.mem.set_u32(map + 8, buckets);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, map);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], clone);
            word(1)
        });
        e.call(0x00a5_4440, &args![this, cloning]);
        let clone_bones = clone + BONE_ARRAY;
        assert_eq!(e.mem.u16(clone_bones + 0xa), 2);
        let new_bones = array_item(&e, clone_bones, 0);
        assert_eq!(array_item(&e, clone_bones, 1), 0);
        assert_eq!(e.mem.u32(new_bones + 8), 2);
        assert_eq!(set_item(&e, new_bones, 0), bone_clone);
        assert_eq!(set_item(&e, new_bones, 1), unmapped_bone);
        let clone_skins = clone + SKIN_ARRAY;
        assert_eq!(e.mem.u16(clone_skins + 0xa), 2);
        assert_eq!(e.mem.u16(clone_skins + 0xc), 1);
        assert_eq!(array_item(&e, clone_skins, 0), 0);
        let new_skins = array_item(&e, clone_skins, 1);
        let new_info = set_item(&e, new_skins, 0);
        assert_eq!(e.mem.u32(new_info), geometry_clone);
        assert_eq!(e.mem.u32(new_info + 4), instance_clone);
        assert_eq!(e.mem.u32(instance_clone + 4), 3);
        assert_eq!(calls(&e, PROCESS_SLOT), vec![vec![instance, cloning]]);
        assert_eq!(set_item(&e, clone + SKIN_SET, 0), geometry_clone);
        // The clone's LOD 1 applied again: its skin geometry swapped.
        assert_eq!(e.get(Ptr::<C>::new(clone), C::m_iLOD), 1);
        assert_eq!(
            e.mem.u32(geometry_clone + GEOMETRY_SKIN_INSTANCE),
            instance_clone
        );
    }

    #[test]
    fn save_binary_writes_counts_and_links() {
        const WRITE: u32 = 0x0001_0570;
        const LINK: u32 = 0x0001_0580;
        let mut e = engine();
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = written.clone();
        e.register_double(WRITE, move |e, a| {
            log.borrow_mut().push((e.mem.u32(a[1]), e.mem.u32(a[3])));
            word(4)
        });
        let links = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let link_log = links.clone();
        e.register_double(LINK, move |_, a| {
            link_log.borrow_mut().push(a[1]);
            Ret::default()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, output);
        let mut slots = vec![0u32; 0xd];
        slots[0x30 / 4] = LINK;
        e.put_vtable(0x0002_2000, &slots);
        e.mem.set_u32(stream, 0x0002_2000);
        let bones = set(&mut e, &[0xb1, 0xb2]);
        let info = skin_info(&mut e, 0x61, 0x62);
        let skins = set(&mut e, &[info]);
        let this = controller(&mut e, 3, 1, &[bones, 0], &[0, skins], &[0x71]);
        e.call(0x00a5_49d0, &args![this, stream]);
        assert_eq!(
            *written.borrow(),
            vec![
                (1, 4),
                (3, 4),
                (2, 4),
                (2, 4),
                (0, 4),
                (2, 4),
                (0, 4),
                (1, 4),
                (1, 4)
            ]
        );
        assert_eq!(*links.borrow(), vec![0xb1, 0xb2, 0x61, 0x62, 0x71]);
        assert_eq!(e.mem.u32(output + 4), 36);
    }

    #[test]
    fn load_binary_builds_placeholder_sets() {
        const READ: u32 = 0x0001_0590;
        let mut e = engine();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        // LOD, LOD count, 2 bone sets (3 bones, 0 bones), 1 skin set (2
        // infos), 1 geometry.
        let mut words = vec![1u32, 2, 2, 3, 0, 1, 2, 1].into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], words.next().unwrap());
            word(4)
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x24c, input);
        let block = e.mem.alloc(OBJECT_SIZE);
        let this = e.call(0x00a5_50b0, &args![block]).ptr::<C>();
        e.call(0x00a5_4c40, &args![this, stream]);
        assert_eq!(e.get(this, C::m_iLOD), 1);
        assert_eq!(e.get(this, C::m_uiNumLODs), 2);
        let bones = this.addr() + BONE_ARRAY;
        assert_eq!(e.mem.u16(bones + 0xa), 2);
        assert_eq!(e.mem.u16(bones + 0xc), 2);
        let first = array_item(&e, bones, 0);
        assert_eq!(e.mem.u32(first + 8), 3);
        assert_eq!(e.mem.u32(first + 4), 4);
        for i in 0..3 {
            assert_eq!(set_item(&e, first, i), this.addr());
        }
        assert_eq!(e.mem.u32(array_item(&e, bones, 1) + 8), 0);
        let skins = this.addr() + SKIN_ARRAY;
        let skin_set = array_item(&e, skins, 0);
        assert_eq!(e.mem.u32(skin_set + 8), 2);
        assert_eq!(e.mem.u32(set_item(&e, skin_set, 1) + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + SKIN_SET + 8), 1);
        assert_eq!(set_item(&e, this.addr() + SKIN_SET, 0), this.addr());
        // 3 bones + 2 * 2 skin links + 1 geometry.
        assert_eq!(calls(&e, STREAM_READ_LINK_ID).len(), 8);
        assert_eq!(e.mem.u32(input + 4), 32);
    }

    #[test]
    fn constructor_factory_and_clone_set_up_the_controller() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &vec![0xcd; a[0] as usize]);
            word(block)
        });
        let this = e.call(0x00a5_5300, &args![]).ptr::<C>();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x68]]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, C::m_iLOD), u32::MAX);
        assert_eq!(e.get(this, C::m_uiNumLODs), 0);
        assert_eq!(e.mem.u32(this.addr() + BONE_ARRAY), BONE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + SKIN_ARRAY), SKIN_ARRAY_VTABLE);
        for array in [BONE_ARRAY, SKIN_ARRAY] {
            let a = array_ptr(this.addr() + array);
            assert_eq!(e.get(a, NiTArray::m_pBase), 0);
            assert_eq!(e.get(a, NiTArray::m_usMaxSize), 0);
            assert_eq!(e.get(a, NiTArray::m_usSize), 0);
            assert_eq!(e.get(a, NiTArray::m_usESize), 0);
            assert_eq!(e.get(a, NiTArray::m_usGrowBy), 1);
        }
        for offset in [0u32, 4, 8] {
            assert_eq!(e.mem.u32(this.addr() + SKIN_SET + offset), 0);
        }
        e.set(this, C::m_iLOD, 2);
        e.set(this, C::m_uiNumLODs, 4);
        let clone = e.call(0x00a5_52c0, &args![this, 0x900u32]).ptr::<C>();
        assert_eq!(
            calls(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x900]]
        );
        assert_eq!(e.get(clone, C::m_iLOD), 2);
        assert_eq!(e.get(clone, C::m_uiNumLODs), 4);
        e.register(OPERATOR_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_5300, &args![]).u32(), 0);
        e.map(0, 0x40);
        assert_eq!(e.call(0x00a5_52c0, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(e.mem.u32(0x38), 4);
    }

    #[test]
    fn target_must_be_a_node() {
        let mut e = engine();
        let this = controller(&mut e, 1, 0, &[], &[], &[]);
        assert!(!e.call(0x00a5_5130, &args![this]).bool());
        let target = object(&mut e);
        e.mem.set_u32(this.addr() + 0x2c, target);
        let base = e.mem.alloc(8);
        e.mem.set_u32(base + 4, NI_NODE_RTTI);
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, base);
        e.mem.set_u32(target + 0x10, derived);
        e.map(NI_NODE_RTTI, 8);
        e.mem.set_u32(NI_NODE_RTTI + 4, 0);
        assert!(e.call(0x00a5_5130, &args![this]).bool());
        e.mem.set_u32(base + 4, 0);
        assert!(!e.call(0x00a5_5130, &args![this]).bool());
        e.mem.set_u32(target + 0x10, 0);
        assert!(!e.call(0x00a5_5130, &args![this]).bool());
    }

    #[test]
    fn update_follows_the_global_lod() {
        let mut e = engine();
        e.map(GLOBAL_LOD, 4);
        let bone = object(&mut e);
        let bones = set(&mut e, &[bone]);
        let this = controller(&mut e, 2, 0, &[bones, 0], &[], &[]);
        e.set_global(GLOBAL_LOD, -1i32);
        e.call(0x00a5_5160, &args![this, 1.0f32]);
        assert_eq!(e.get(this, C::m_iLOD), 0);
        e.set_global(GLOBAL_LOD, 1i32);
        e.call(0x00a5_5160, &args![this, 1.0f32]);
        assert_eq!(e.get(this, C::m_iLOD), 1);
        // Above the count: the last LOD.
        e.set_global(GLOBAL_LOD, 5i32);
        e.set(this, C::m_iLOD, 0);
        e.call(0x00a5_5160, &args![this, 1.0f32]);
        assert_eq!(e.get(this, C::m_iLOD), 1);
        // The count itself is not above it: rejected by SetBoneLOD.
        e.set_global(GLOBAL_LOD, 2i32);
        e.call(0x00a5_5160, &args![this, 1.0f32]);
        assert_eq!(e.get(this, C::m_iLOD), 1);
    }

    #[test]
    fn destructor_frees_the_sets_and_arrays() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let instance = object(&mut e);
        let bones = set(&mut e, &[1]);
        let info = skin_info(&mut e, 2, instance);
        let skins = set(&mut e, &[info, 0]);
        let this = controller(&mut e, 1, 0, &[bones, 0], &[skins], &[3]);
        let bone_block = e.mem.u32(bones);
        let skin_block = e.mem.u32(skins);
        let geometry_block = e.mem.u32(this.addr() + SKIN_SET);
        let bone_base = e.get(array_ptr(this.addr() + BONE_ARRAY), NiTArray::m_pBase);
        let skin_base = e.get(array_ptr(this.addr() + SKIN_ARRAY), NiTArray::m_pBase);
        assert_eq!(e.call(0x00a5_53f0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(
            calls(&e, SMALL_FREE),
            vec![
                vec![SMALL_ALLOCATOR, bones],
                vec![SMALL_ALLOCATOR, info],
                vec![SMALL_ALLOCATOR, skins]
            ]
        );
        assert_eq!(
            calls(&e, FREE),
            vec![
                vec![bone_block],
                vec![skin_block],
                vec![geometry_block],
                vec![skin_base],
                vec![bone_base]
            ]
        );
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![instance]]);
        assert_eq!(e.mem.u32(bone_base), 0);
        assert_eq!(e.mem.u16(this.addr() + BONE_ARRAY + 0xa), 0);
        assert_eq!(e.mem.u16(this.addr() + SKIN_ARRAY + 0xa), 0);
        assert_eq!(e.mem.u32(this.addr() + SKIN_SET + 8), 0);
        assert_eq!(e.mem.u32(this.addr() + BONE_ARRAY), BONE_NI_T_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + SKIN_ARRAY), SKIN_NI_T_ARRAY_VTABLE);
        assert_eq!(calls(&e, TIME_CONTROLLER_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this.addr(), 0x68]]);
        e.call(0x00a5_53f0, &args![this, 0u32]);
        assert_eq!(calls(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn get_viewer_strings_appends_name_and_lods() {
        let mut e = engine();
        e.map(NI_BONE_LOD_CONTROLLER_RTTI, 4);
        e.set_global(NI_BONE_LOD_CONTROLLER_RTTI, 0x0109_f0f0u32);
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x91));
        e.register(NI_GET_VIEWER_STRING_SIGNED, |_, _| word(0x92));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, _| word(0x93));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = controller(&mut e, 3, u32::MAX, &[], &[], &[]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 1);
        e.call(0x00a5_5320, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_f0f0]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_SIGNED),
            vec![vec![NAME_LOD, u32::MAX]]
        );
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![vec![NAME_NUM_LODS, 3]]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
    }
}
