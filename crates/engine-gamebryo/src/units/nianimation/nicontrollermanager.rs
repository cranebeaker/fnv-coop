//! `NiAnimation/nicontrollermanager.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTPointerMap};
use crate::units::nianimation::nicontrollersequence::NiControllerSequence;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream, NiTPrimitiveSet};

/// `NiAVObjectPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_AV_OBJECT_PALETTE_RTTI: u32 = 0x011f_5b58;

/// `NiDefaultAVObjectPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_DEFAULT_AV_OBJECT_PALETTE_RTTI: u32 = 0x011f_4a04;

// Translated from 00a2e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObjectPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_av_object_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_AV_OBJECT_PALETTE_RTTI)
}

// Translated from 00a2ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_default_av_object_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DEFAULT_AV_OBJECT_PALETTE_RTTI)
}

// Translated from 00a2ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a2ebb0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x7c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_controller_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a2_ebb0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x7cu32]);
    }
    this
}

layout! {
    /// `NiControllerManager` (Xbox PDB), 0x7c bytes on PC as on the Xbox: an
    /// `NiTimeController` (0x34 bytes), the sequences, the active ones, the
    /// name map, the animation-note listener, the cumulative flag, the temporary
    /// blend sequences and the object palette.
    pub struct NiControllerManager: 0x7c {
        /// `NiTimeController::m_uFlags` (Xbox PDB).
        0x08 m_uFlags: u16,
        /// `NiTimeController::m_pkTarget` (Xbox PDB).
        0x2C m_pkTarget: Ptr,
        /// `m_kSequenceArray` (Xbox PDB): `NiTObjectArray<NiPointer<NiControllerSequence>>`.
        0x34 m_kSequenceArray: Inline<NiTArray>,
        /// `m_kActiveSequences` (Xbox PDB): `NiTPrimitiveSet<NiControllerSequence *>`.
        0x44 m_kActiveSequences: Inline<NiTPrimitiveSet>,
        /// `m_kStringMap` (Xbox PDB): `NiTStringPointerMap<NiControllerSequence *>`.
        0x50 m_kStringMap: Inline<NiTStringPointerMap>,
        /// `pListener` (Xbox PDB): `BSAnimNoteListener *`.
        0x64 pListener: Ptr,
        /// `m_bCumulative` (Xbox PDB).
        0x68 m_bCumulative: u8,
        /// `m_kTempBlendSeqs` (Xbox PDB): `NiTObjectSet<NiPointer<NiControllerSequence>>`.
        0x6C m_kTempBlendSeqs: Inline<NiTPrimitiveSet>,
        /// `m_spObjectPalette` (Xbox PDB): `NiPointer<NiAVObjectPalette>`.
        0x78 m_spObjectPalette: Ptr,
    }

    /// `NiTStringPointerMap<NiControllerSequence *>` (Xbox PDB), 0x14 bytes: an
    /// `NiTMapBase` (vtable, bucket count, bucket array, item count) and the copy flag.
    pub struct NiTStringPointerMap: 0x14 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB).
        0x08 m_ppkHashTable: Ptr,
        /// `m_kAllocator.m_uiCount` (Xbox PDB).
        0x0C m_uiCount: u32,
        /// `m_bCopy` (Xbox PDB): whether the map owns copies of its key strings.
        0x10 m_bCopy: bool,
    }

    /// `NiDefaultAVObjectPalette` (Xbox PDB), 0x1c bytes: an `NiAVObjectPalette` (an
    /// `NiObject`, 8 bytes), an `NiTFixedStringMap<NiAVObject *>` at +8 (vtable, bucket count,
    /// bucket array, item count) and the scene.
    pub struct NiDefaultAVObjectPalette: 0x1C {
        /// `m_kMap` (Xbox PDB): the embedded map.
        0x08 m_kMap: Inline<NiTPointerMap>,
        /// `m_pkScene` (Xbox PDB): `NiAVObject *`.
        0x18 m_pkScene: Ptr,
    }
}

/// `NiAVObjectPalette`'s vtable, `NiDefaultAVObjectPalette`'s vtable and the `NiTFixedStringMap`'s
/// vtables (`NiTFixedStringMapBase` then `NiTFixedStringMap`, the order its destructors store them in).
const NI_AV_OBJECT_PALETTE_VTABLE: u32 = 0x0109_607c;
const NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE: u32 = 0x0109_6284;
const FIXED_STRING_MAP_BASE_VTABLE: u32 = 0x0109_6114;
const FIXED_STRING_MAP_VTABLE: u32 = 0x0109_618c;
/// The vtables of the sequence-name map's classes, outermost last stored first: `NiTMapBase`
/// (`01096124`), `NiTPointerMap` (`01096144`), `NiTStringTemplateMap` (`01096164`); and of the
/// sequence array (`NiTArray`/`NiTObjectArray`, `01096184` and `01096258`) and the string map
/// (`NiTStringPointerMap`, `01096260`).
const MAP_BASE_VTABLE: u32 = 0x0109_6124;
const POINTER_MAP_VTABLE: u32 = 0x0109_6144;
const STRING_TEMPLATE_MAP_VTABLE: u32 = 0x0109_6164;
const ARRAY_VTABLE: u32 = 0x0109_6184;
const OBJECT_ARRAY_VTABLE: u32 = 0x0109_6258;
const STRING_POINTER_MAP_VTABLE: u32 = 0x0109_6260;
/// `NiControllerManager`'s vtable.
const NI_CONTROLLER_MANAGER_VTABLE: u32 = 0x0109_619c;
/// The memory manager singleton and its functions.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `operator new` (`(size)`), the table allocator (`(size)`), the table free (`(block)`), the sized
/// `operator delete` (`(block, size)`), `_memset` (`(destination, value, size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMSET: u32 = 0x00ec_61c0;
/// `atexit` (`(function)`).
const ATEXIT: u32 = 0x00ec_658f;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The word that holds the handle of the empty `NiFixedString`, and the static `NiFixedString`
/// at `011f49e0` that the sequences are named with.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const FIXED_STRING_011F49E0: u32 = 0x011f_49e0;
/// `NiFixedString`'s constructor from characters (`(chars)`, answers the handle).
const FIXED_STRING_FROM_CHARS: u32 = 0x00a5_b690;
/// `"__TempBlendSequence__"`.
const TEXT_TEMP_BLEND_SEQUENCE: u32 = 0x0101_dca8;
/// `NiObject`'s constructor and destructor body.
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiTimeController`: constructor (`00a6d220`), destructor body, `IsEqual` (the body of
/// `NiTimeController::IsEqual`, `00a6cc80`), `ProcessClone`, `RegisterStreamables`, `LinkObject`,
/// `LoadBinary`, `SaveBinary`, `SetTarget` and the member copy (`this` = source, `(destination, process)`).
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_SET_TARGET: u32 = 0x00a6_d450;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// `NiObject`'s post-link function reached through `00a89af0` (one stream word).
const OBJECT_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream`: `ResolveLinkID`, `ReadMultipleLinkIDs`, `GetNumberOfLinkIDs`, `GetObjectFromLinkID`.
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// Cloning-process map lookup (`this` = the map, `(key, &out)`).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `NiControllerSequence` functions (`this` = the sequence): `Update` (`(time, flag)`), the
/// detach function `00a30880`, the one that attaches to a target (`00a32c70`, `(target)`, answers a
/// bool), the constructor (`00a32a10`, `(&name, size, grow by)`, answers the sequence),
/// `00a32fa0` (`this` = the source, `(temporary sequence, size, word, flag)`), `00a34f20`
/// (`(priority, start over, weight, ease-in time, time-sync sequence, start frame)`) and `00a350d0`.
const SEQUENCE_UPDATE: u32 = 0x00a3_4ba0;
const SEQUENCE_DETACH: u32 = 0x00a3_0880;
const SEQUENCE_ATTACH_TO_TARGET: u32 = 0x00a3_2c70;
const SEQUENCE_CONSTRUCT: u32 = 0x00a3_2a10;
const SEQUENCE_COPY_BLEND_POSE: u32 = 0x00a3_2fa0;
const SEQUENCE_ACTIVATE: u32 = 0x00a3_4f20;
const SEQUENCE_START_BLEND: u32 = 0x00a3_50d0;
/// `NiFixedString` assignment of a sequence (`this` = the sequence, `(&name)`): `004eeae0`
/// and `0043b9f0`, both jumps to `0043ba10`.
const SEQUENCE_SET_NAME_FROM_TARGET: u32 = 0x004e_eae0;
const SEQUENCE_SET_NAME: u32 = 0x0043_b9f0;
/// The string map's `SetAt` (`this` = the map, `(name, sequence)`) and `RemoveAt` (`(name)`).
const STRING_MAP_SET_AT: u32 = 0x00a6_42a0;
const STRING_MAP_REMOVE_AT: u32 = 0x00e9_1fd0;
/// The map iteration step (`this` = the map, `(&position, &key, &value)`) and the lookup in
/// the other map (`this` = the map, `(key, &out)`, answers whether found).
const MAP_GET_NEXT: u32 = 0x00b5_7f20;
const MAP_GET_AT: u32 = 0x0085_3130;
/// `RemoveAll` of the map classes the sequence map is built from (`this` = the map).
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// `NiTSet<NiPointer<..>>::Realloc` (`this` = the set, `(new size)`) and `Add` (`(&element)`).
const SET_REALLOC: u32 = 0x00e8_e3a0;
const SET_ADD: u32 = 0x00e8_ea60;
/// The array allocator `NiTNewInterface` uses (`(count)`) and the array free (`(elements)`).
const ARRAY_ALLOCATE: u32 = 0x00c1_d790;
const ARRAY_FREE: u32 = 0x00a4_7600;
/// The array function that takes an element out (`this` = the array, `(&removed, index)`).
const ARRAY_REMOVE_AT: u32 = 0x00a4_7540;
/// The listener's constructor (`this` = the 0x10 bytes) and destructor body.
const LISTENER_CONSTRUCT: u32 = 0x00a4_14f0;
const LISTENER_DESTRUCT: u32 = 0x00a4_1450;
/// The palette's scene walk (`this` = the palette), run by its constructor.
const PALETTE_FILL: u32 = 0x00a6_e960;
/// The palette map's item set function (`this` = the map, `(item, &key, value)`).
const PALETTE_ITEM_SET: u32 = 0x00a6_e550;
/// The palette map's `RemoveAt` (`this` = the map, `(&key)`).
const PALETTE_MAP_REMOVE_AT: u32 = 0x00aa_0250;
/// The function-local static `NiPointer` null that `SetAt` compares with, and its guard word.
const NULL_POINTER_STATIC: u32 = 0x011f_36a4;
const NULL_POINTER_GUARD: u32 = 0x011f_36a8;
/// The destructor `atexit` is given for that static.
const NULL_POINTER_ATEXIT: u32 = 0x00fd_ad30;
/// `NiControllerManager::ms_RTTI`.
const NI_CONTROLLER_MANAGER_RTTI: u32 = 0x011f_36ac;
/// Virtual slots: `NiObject::DeleteThis` (4), `RegisterStreamables` (0x54), `IsEqual` (0x5c),
/// `ProcessClone` (0x68), `CreateClone` (0x48); `NiControllerSequence::Deactivate` (0x8c); the
/// map's `HashFunction` (4) and `IsKeysEqual` (8) and `DeleteItem` (8 of `NiTFixedStringMapBase`);
/// `NiStream::SaveLinkID` (0x30).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_PROCESS_CLONE: u32 = 0x68;
const SLOT_CREATE_CLONE: u32 = 0x48;
const SLOT_DEACTIVATE: u32 = 0x8c;
const SLOT_HASH_FUNCTION: u32 = 0x04;
const SLOT_IS_KEYS_EQUAL: u32 = 0x08;
const SLOT_DELETE_ITEM: u32 = 0x08;
const SLOT_NEW_ITEM: u32 = 0x04;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
/// Offsets inside a `NiControllerSequence`'s interpolator array items (0x10 bytes each).
const INTERP_ITEM_SIZE: u32 = 0x10;

/// The `float` the exe copies with `FLD`/`FSTP` from the word at `source`: its bits, a signalling NaN quieted.
fn quiet_word(e: &Engine, source: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(source))
}

/// `InterlockedIncrement` of an object's reference count at `+4`.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at `+4` and, when it
/// reaches zero, the object's `DeleteThis` (virtual slot 4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer stored at `slot`: when the new pointer
/// differs the old one (if set) is released, the new one is stored and (if set) counted.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

/// [`assign_pointer`] for a new pointer the exe reads again from `source` after the release
/// of the old one (an assignment from a reference).
fn assign_pointer_from(e: &mut Engine, slot: u32, source: u32) {
    let old = e.mem.u32(slot);
    if old != e.mem.u32(source) {
        if old != 0 {
            release(e, old);
        }
        let new = e.mem.u32(source);
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

/// `InterlockedIncrement` of a pooled string's count (`handle - 8`) unless it is the empty string.
fn fixed_string_add_ref(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// `InterlockedDecrement` of a pooled string's count (`handle - 8`) unless it is the empty string.
fn fixed_string_release(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The inline `NiFixedString::operator=` for the handle stored at `slot`: when the new handle
/// differs it is counted first, then the old one is released and the new one stored.
fn assign_fixed_string(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        fixed_string_add_ref(e, new);
        let old = e.mem.u32(slot);
        fixed_string_release(e, old);
        e.mem.set_u32(slot, new);
    }
}

/// Makes the `NiDefaultAVObjectPalette` the way the inline sequence of `00a2efa0`, `00a2f610`
/// does: `operator new(0x1c)` and, when that gives memory, the constructor `00a2ed80`
/// (`(scene, hash size)`). Answers the palette or 0.
fn new_palette(e: &mut Engine, scene: u32, hash_size: u32) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    if object == 0 {
        return 0;
    }
    fn_00a2ed80(e, Ptr::new(object), Ptr::new(scene), hash_size).addr()
}

/// The `NiControllerManager` field initialisation both constructors share (everything up to the
/// palette): the time-controller constructor (`00a6d220`), the vtable, an empty sequence array
/// (growth 10), no active sequences, the sequence-name map with 0x25 buckets, the cumulative flag
/// `cumulative`, no temporary blend sequences and no palette.
fn construct_manager_fields(e: &mut Engine, this: Ptr<NiControllerManager>, cumulative: u8) {
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![this]);
    let this_address = this.addr();
    e.mem.set_u32(this_address, NI_CONTROLLER_MANAGER_VTABLE);
    e.mem.set_u32(this_address + 0x38, 0);
    e.mem.set_u32(this_address + 0x34, OBJECT_ARRAY_VTABLE);
    e.mem.set_u16(this_address + 0x3c, 0);
    e.mem.set_u16(this_address + 0x40, 0);
    e.mem.set_u16(this_address + 0x42, 10);
    e.mem.set_u16(this_address + 0x3e, 0);
    e.mem.set_u32(this_address + 0x44, 0);
    e.mem.set_u32(this_address + 0x48, 0);
    e.mem.set_u32(this_address + 0x4c, 0);
    e.mem.set_u32(this_address + 0x50, MAP_BASE_VTABLE);
    e.mem.set_u32(this_address + 0x54, 0x25);
    e.mem.set_u32(this_address + 0x5c, 0);
    let table = e.call(TABLE_ALLOCATE, &args![0x94u32]).u32();
    e.mem.set_u32(this_address + 0x58, table);
    let buckets = e.mem.u32(this_address + 0x54);
    e.call(MEMSET, &args![table, 0u32, buckets * 4]);
    e.mem.set_u8(this_address + 0x60, 0);
    e.mem
        .set_u32(this_address + 0x50, STRING_POINTER_MAP_VTABLE);
    e.mem.set_u8(this_address + 0x68, cumulative);
    e.mem.set_u32(this_address + 0x6c, 0);
    e.mem.set_u32(this_address + 0x70, 0);
    e.mem.set_u32(this_address + 0x74, 0);
    e.mem.set_u32(this_address + 0x78, 0);
}

/// The end of both constructors: `operator new` through the memory manager for the 0x10-byte
/// animation-note listener (`Allocate(0x10)`), its constructor (`00a414f0`, whose answer is stored
/// at `+0x64`), or 0 stored when there is no memory.
fn construct_listener(e: &mut Engine, this: Ptr<NiControllerManager>) {
    let listener = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    if listener == 0 {
        e.set(this, NiControllerManager::pListener, Ptr::NULL);
        return;
    }
    let constructed = e.call(LISTENER_CONSTRUCT, &args![listener]).u32();
    e.set(this, NiControllerManager::pListener, Ptr::new(constructed));
}

// Translated from 00a2e1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObjectPalette::_scalar_deleting_destructor_` (Xbox PDB): stores the class vtable,
/// runs `NiObject`'s destructor body (`00a5d3d0`) and, when bit 0 of `flags` is set, the sized
/// delete (`00aa1460`, `(this, 8)`). Answers `this`.
pub fn ni_av_object_palette_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_AV_OBJECT_PALETTE_VTABLE);
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 8u32]);
    }
    this
}

// Translated from 00a2e220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::Update` (Xbox PDB): when bit 3 of the flags byte at `+8` is set and
/// there are active sequences, updates each one (`NiControllerSequence::Update`, `00a34ba0`,
/// with the time in the first float of `update_data`, quieted by the x87 copy, and 1) and, when
/// that leaves the sequence inactive (state 0 at `+0x44`), takes it out of the active set by
/// moving the last active sequence into its slot (the moved one is not updated again this call).
pub fn ni_controller_manager_update(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    update_data: Ptr,
) {
    if e.mem.u8(this.addr() + 8) >> 3 & 1 == 0 {
        return;
    }
    let active = this.at(NiControllerManager::m_kActiveSequences);
    if e.get(active, NiTPrimitiveSet::m_uiUsed) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let base = e.get(active, NiTPrimitiveSet::m_pBase);
        let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        let time = quiet_word(e, update_data.addr());
        e.call(SEQUENCE_UPDATE, &args![sequence, time, 1u32]);
        if e.mem.u32(sequence + 0x44) == 0 {
            let used = e.get(active, NiTPrimitiveSet::m_uiUsed).wrapping_sub(1);
            e.set(active, NiTPrimitiveSet::m_uiUsed, used);
            let base = e.get(active, NiTPrimitiveSet::m_pBase);
            let last = e.mem.u32(base.wrapping_add(used.wrapping_mul(4)));
            e.mem
                .set_u32(base.wrapping_add(index.wrapping_mul(4)), last);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
        if index >= e.get(active, NiTPrimitiveSet::m_uiUsed) {
            break;
        }
    }
}

// Translated from 00a2e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` not read, seven stack words; the shape of a
/// cross fade between two sequences, name unconfirmed): answers false unless `source` is not
/// inactive (state at `+0x44`) and `destination` is. Then runs the source's `Deactivate` (virtual
/// slot 0x8c, `(first_float, 0)`) and answers the destination's `00a34f20` (`(first_word,
/// second_word, second_float, first_float, other_sequence, 0)`); the floats are copied through the
/// x87 stack (a signalling NaN goes on quieted).
pub fn fn_00a2e280(
    e: &mut Engine,
    _this: Ptr,
    source: Ptr,
    destination: Ptr,
    first_float: f32,
    first_word: u32,
    second_word: u32,
    second_float: f32,
    other_sequence: Ptr,
) -> bool {
    if e.mem.u32(source.addr() + 0x44) == 0 {
        return false;
    }
    if e.mem.u32(destination.addr() + 0x44) != 0 {
        return false;
    }
    e.vcall(
        source.addr(),
        SLOT_DEACTIVATE,
        &args![x87_quiet(first_float), 0u32],
    );
    e.call(
        SEQUENCE_ACTIVATE,
        &args![
            destination,
            first_word,
            second_word,
            x87_quiet(second_float),
            x87_quiet(first_float),
            other_sequence,
            0u32
        ],
    )
    .bool()
}

// Translated from 00a2e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::PostLinkObject` (Xbox PDB): runs the `NiObject` post-link function
/// (`00a89af0`, `(stream)`), then for each non-null sequence (the 16-bit count at `+0x3e`) runs the
/// sequence's `00a32c70` with the manager's target (`+0x2c`).
pub fn ni_controller_manager_post_link_object(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    stream: Ptr,
) {
    e.call(OBJECT_POST_LINK_OBJECT, &args![this, stream]);
    let target = e.get(this, NiControllerManager::m_pkTarget);
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                e.call(SEQUENCE_ATTACH_TO_TARGET, &args![sequence, target]);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
}

// Translated from 00a2e320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::IsEqual` (Xbox PDB): false unless the time-controller comparison
/// (`00a6cc80`) holds and the sequence counts (`+0x3e`) are equal. Then every entry of this
/// manager's name map (walked bucket by bucket with `00b57f20`) must have an entry of the same name
/// in the other's map (`00853130`) whose sequence the first one's `IsEqual` (virtual slot 0x5c)
/// accepts; finally each pair of sequences at the same index must be both null and the
/// same, or both set and accepted by the first one's `IsEqual`.
pub fn ni_controller_manager_is_equal(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    other: Ptr<NiControllerManager>,
) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let other_sequences = other.at(NiControllerManager::m_kSequenceArray);
    if e.get(sequences, NiTArray::m_usSize) != e.get(other_sequences, NiTArray::m_usSize) {
        return false;
    }
    let map = this.at(NiControllerManager::m_kStringMap);
    let other_map = other.at(NiControllerManager::m_kStringMap);
    let hash_size = e.get(map, NiTStringPointerMap::m_uiHashSize);
    let mut position = 0u32;
    if hash_size != 0 {
        let table = e.get(map, NiTStringPointerMap::m_ppkHashTable).addr();
        let mut bucket = 0u32;
        loop {
            let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
            if head != 0 {
                position = head;
                break;
            }
            bucket += 1;
            if bucket >= hash_size {
                break;
            }
        }
    }
    let equal = e.with_stack(0x10, |e, slots| {
        let position_slot = slots.addr();
        let key_slot = slots.addr() + 4;
        let value_slot = slots.addr() + 8;
        let found_slot = slots.addr() + 12;
        e.mem.set_u32(position_slot, position);
        while e.mem.u32(position_slot) != 0 {
            e.call(
                MAP_GET_NEXT,
                &args![map, position_slot, key_slot, value_slot],
            );
            let key = e.mem.u32(key_slot);
            if !e
                .call(MAP_GET_AT, &args![other_map, key, found_slot])
                .bool()
            {
                return false;
            }
            let sequence = e.mem.u32(value_slot);
            let counterpart = e.mem.u32(found_slot);
            if !e.vcall(sequence, SLOT_IS_EQUAL, &args![counterpart]).bool() {
                return false;
            }
        }
        true
    });
    if !equal {
        return false;
    }
    let count = e.get(sequences, NiTArray::m_usSize);
    if count == 0 {
        return true;
    }
    let mut index = 0u32;
    loop {
        let base = e.get(sequences, NiTArray::m_pBase);
        let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        let other_base = e.get(other_sequences, NiTArray::m_pBase);
        let counterpart = e.mem.u32(other_base.wrapping_add(index.wrapping_mul(4)));
        if sequence == 0 || counterpart == 0 {
            if sequence != counterpart {
                return false;
            }
        } else if !e.vcall(sequence, SLOT_IS_EQUAL, &args![counterpart]).bool() {
            return false;
        }
        index += 1;
        if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
            break;
        }
    }
    true
}

// Translated from 00a2e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float): runs `Deactivate` (virtual slot 0x8c,
/// `(time, 0)`) on every active sequence (the set at `+0x44`, in order) whose state (`+0x44`) is 4.
/// Answers 0 in `AL`.
pub fn fn_00a2e420(e: &mut Engine, this: Ptr<NiControllerManager>, time: f32) -> bool {
    let active = this.at(NiControllerManager::m_kActiveSequences);
    let mut index = 0u32;
    if e.get(active, NiTPrimitiveSet::m_uiUsed) != 0 {
        loop {
            let base = e.get(active, NiTPrimitiveSet::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if e.mem.u32(sequence + 0x44) == 4 {
                e.vcall(sequence, SLOT_DEACTIVATE, &args![x87_quiet(time), 0u32]);
            }
            index = index.wrapping_add(1);
            if index >= e.get(active, NiTPrimitiveSet::m_uiUsed) {
                break;
            }
        }
    }
    false
}

// Translated from 00a2e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): the destructor body of the sequence-name map
/// (`NiTStringPointerMap<NiControllerSequence *>`, the class of vtable `01096260`) that the scalar
/// deleting destructor `00a2e550` runs. Stores the `NiTStringTemplateMap` vtable; when the map owns
/// copies of its keys (`+0x10`), frees every item's key string (`00aa10f0`); then runs
/// `00b601a0` (`RemoveAll`) with the `NiTPointerMap` vtable stored and again with the `NiTMapBase`
/// vtable stored, and frees the bucket table (`+8`).
pub fn fn_00a2e460(e: &mut Engine, this: Ptr<NiTStringPointerMap>) {
    e.mem.set_u32(this.addr(), STRING_TEMPLATE_MAP_VTABLE);
    if e.get(this, NiTStringPointerMap::m_bCopy) {
        let mut bucket = 0u32;
        if e.get(this, NiTStringPointerMap::m_uiHashSize) != 0 {
            loop {
                let table = e.get(this, NiTStringPointerMap::m_ppkHashTable).addr();
                let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
                while item != 0 {
                    let key = e.mem.u32(item + 4);
                    item = e.mem.u32(item);
                    e.call(TABLE_FREE, &args![key]);
                }
                bucket += 1;
                if bucket >= e.get(this, NiTStringPointerMap::m_uiHashSize) {
                    break;
                }
            }
        }
    }
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTStringPointerMap::m_ppkHashTable);
    e.call(TABLE_FREE, &args![table]);
}

// Translated from 00a2e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, char const *, NiControllerSequence *>::
/// _scalar_deleting_destructor_` (a name from the vtable `01096124` this stores, whose slot 0
/// this is): stores the vtable, runs `00b601a0` (`RemoveAll`), frees the bucket table (`+8`)
/// and, when bit 0 of `flags` is set, gives `this` back to the memory manager (`Deallocate`).
/// Answers `this`.
pub fn fn_00a2e4d0(
    e: &mut Engine,
    this: Ptr<NiTStringPointerMap>,
    flags: u32,
) -> Ptr<NiTStringPointerMap> {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTStringPointerMap::m_ppkHashTable);
    e.call(TABLE_FREE, &args![table]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a2e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const *, NiControllerSequence *>::_scalar_deleting_destructor_` (a name
/// from the vtable `01096144` this stores, whose slot 0 this is): stores that vtable and runs
/// `RemoveAll` (`00b601a0`), then the `NiTMapBase` vtable and `RemoveAll` again, frees the
/// bucket table (`+8`) and, when bit 0 of `flags` is set, gives `this` back to the memory
/// manager. Answers `this`.
pub fn fn_00a2e510(
    e: &mut Engine,
    this: Ptr<NiTStringPointerMap>,
    flags: u32,
) -> Ptr<NiTStringPointerMap> {
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTStringPointerMap::m_ppkHashTable);
    e.call(TABLE_FREE, &args![table]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a2e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringPointerMap<NiControllerSequence *>::_scalar_deleting_destructor_` (Xbox PDB name
/// from the engine map): runs the destructor body `00a2e460` and, when bit 0 of `flags` is set,
/// gives `this` back to the memory manager. Answers `this`.
pub fn ni_t_string_pointer_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTStringPointerMap>,
    flags: u32,
) -> Ptr<NiTStringPointerMap> {
    fn_00a2e460(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a2e570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): finds the sequence that uses
/// `interpolator`. Walks the sequences (the 16-bit count at `+0x3e`); each non-null one is counted
/// (`InterlockedIncrement`) and, when `only_active` is false or its state (`+0x44`) is not 0, its
/// interpolator array (`+0x14`, `+0xc` items of 0x10 bytes) is searched for a non-null first word
/// equal to `interpolator`. The sequence is released again (the count dropped, the object
/// deleted at zero) and, for a match, answered; otherwise 0.
pub fn fn_00a2e570(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    interpolator: u32,
    only_active: bool,
) -> Ptr {
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                add_ref(e, sequence);
                if !only_active || e.mem.u32(sequence + 0x44) != 0 {
                    let count = e.mem.u32(sequence + 0xc);
                    let mut item = 0u32;
                    if count != 0 {
                        let mut entry = e.mem.u32(sequence + 0x14);
                        loop {
                            let candidate = e.mem.u32(entry);
                            if candidate != 0 && candidate == interpolator {
                                release(e, sequence);
                                return Ptr::new(sequence);
                            }
                            item += 1;
                            entry = entry.wrapping_add(INTERP_ITEM_SIZE);
                            if item >= count {
                                break;
                            }
                        }
                    }
                }
                release(e, sequence);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    Ptr::NULL
}

// Translated from 00a2e620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`, `this` = an `NiTArray<NiPointer<NiControllerSequence>>`):
/// the array's `RemoveAll`: releases every non-null element (the 16-bit size at `+0xa`),
/// clears its slot, and sets the size (`+0xa`) and the element count (`+0xc`) to 0.
pub fn fn_00a2e620(e: &mut Engine, this: Ptr<NiTArray>) {
    let mut index = 0u16;
    if 0 < e.get(this, NiTArray::m_usSize) {
        loop {
            let base = e.get(this, NiTArray::m_pBase);
            let slot = base.wrapping_add(u32::from(index) * 4);
            let element = e.mem.u32(slot);
            if element != 0 {
                release(e, element);
                e.mem.set_u32(slot, 0);
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiTArray::m_usSize) {
                break;
            }
        }
    }
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
}

// Translated from 00a2e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`, `this` = an
/// `NiTArray<NiPointer<NiControllerSequence>>`): `SetAt(index, &element)`. The first time (the
/// guard word `011f36a8` bit 0 clear) it sets that bit, stores 0 in the static null pointer
/// `011f36a4` and gives `atexit` its destructor. An `index` at or past the size grows the size to
/// `index + 1` and counts the element when it is not the null static; otherwise the element count
/// (`+0xc`) goes down by one when the new element is the null static and the old one is not,
/// and up by one in the opposite case. Then the slot takes the element as a pointer assignment
/// (the old one released, the new one counted; read again from `element` after the release).
pub fn fn_00a2e680(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: Ptr) {
    if e.mem.u8(NULL_POINTER_GUARD) & 1 == 0 {
        let guard = e.mem.u32(NULL_POINTER_GUARD);
        e.mem.set_u32(NULL_POINTER_GUARD, guard | 1);
        e.mem.set_u32(NULL_POINTER_STATIC, 0);
        e.call(ATEXIT, &args![NULL_POINTER_ATEXIT]);
    }
    let size = u32::from(e.get(this, NiTArray::m_usSize));
    if index >= size {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if e.mem.u32(element.addr()) != e.mem.u32(NULL_POINTER_STATIC) {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else {
        let null = e.mem.u32(NULL_POINTER_STATIC);
        let base = e.get(this, NiTArray::m_pBase);
        let old = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u32(element.addr()) == null {
            if old != null {
                let count = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, count.wrapping_add(0xffff));
            }
        } else if old == null {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    assign_pointer_from(e, slot, element.addr());
}

// Translated from 00a2e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, `this` = a `NiFixedString`): the assignment
/// from characters. When the new characters are not the current handle, the new handle
/// (`00a5b690`) is stored and the old one is released unless it is the empty string's.
/// Answers `this`.
pub fn fn_00a2e750(e: &mut Engine, this: Ptr, characters: u32) -> Ptr {
    let old = e.mem.u32(this.addr());
    if old != characters {
        let handle = e.call(FIXED_STRING_FROM_CHARS, &args![characters]).u32();
        e.mem.set_u32(this.addr(), handle);
        if old != e.global::<u32>(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(0xffff_fff8)]);
        }
    }
    this
}

// Translated from 00a2e790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMapBase<DFALL<NiTMapItem<NiFixedString, NiAVObject *>>, NiAVObject *>::
/// _scalar_deleting_destructor_` (Xbox PDB name from the engine map): stores the class vtable,
/// runs `RemoveAll` (`00aa01f0`), frees the bucket table (`+8`) and, when bit 0 of `flags` is
/// set, gives `this` back to the memory manager. Answers `this`.
pub fn ni_t_fixed_string_map_base_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), FIXED_STRING_MAP_BASE_VTABLE);
    ni_t_fixed_string_map_base_remove_all(e, this);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(TABLE_FREE, &args![table]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a2e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; the work of `NiControllerManager::RemoveSequence`
/// `00a2ec50`): looks for `sequence` in the sequence array (counting each entry while it looks).
/// Not found: releases and clears the `NiPointer` at `out`. Found at index `i`: takes it out of the
/// array (`00a47540`, `(&removed, i)`), drops the entry of the name map whose value is the sequence
/// (walking the buckets item by item, `RemoveAt` `00e91fd0` with the key), takes it out of the
/// active set (moving the last one into its slot), runs the sequence's `00a30880`, clears its
/// owner (`+0x40`), stores it in `out` as a pointer assignment and releases the walking count.
pub fn fn_00a2e7d0(e: &mut Engine, this: Ptr<NiControllerManager>, sequence: u32, out: Ptr) {
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    let mut found = false;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let entry = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if entry != 0 {
                add_ref(e, entry);
            }
            if entry == sequence {
                found = true;
                break;
            }
            if entry != 0 {
                release(e, entry);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    if !found {
        let old = e.mem.u32(out.addr());
        if old != 0 {
            release(e, old);
            e.mem.set_u32(out.addr(), 0);
        }
        return;
    }
    e.with_stack(4, |e, removed| {
        e.call(ARRAY_REMOVE_AT, &args![sequences, removed, index]);
        let taken = e.mem.u32(removed.addr());
        if taken != 0 {
            release(e, taken);
        }
    });
    let map = this.at(NiControllerManager::m_kStringMap);
    let hash_size = e.get(map, NiTStringPointerMap::m_uiHashSize);
    let mut item = 0u32;
    let mut bucket = 0u32;
    if hash_size != 0 {
        let table = e.get(map, NiTStringPointerMap::m_ppkHashTable).addr();
        loop {
            let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
            if head != 0 {
                item = head;
                break;
            }
            bucket += 1;
            if bucket >= hash_size {
                break;
            }
        }
    }
    while item != 0 {
        let value = e.mem.u32(item + 8);
        let key = e.mem.u32(item + 4);
        let mut next = e.mem.u32(item);
        if next == 0 {
            let hashed = e.vcall(map.addr(), SLOT_HASH_FUNCTION, &args![key]).u32();
            let hash_size = e.get(map, NiTStringPointerMap::m_uiHashSize);
            let mut following = hashed.wrapping_add(1);
            if following < hash_size {
                let table = e.get(map, NiTStringPointerMap::m_ppkHashTable).addr();
                loop {
                    let head = e.mem.u32(table.wrapping_add(following.wrapping_mul(4)));
                    if head != 0 {
                        next = head;
                        break;
                    }
                    following += 1;
                    if following >= hash_size {
                        break;
                    }
                }
            }
        }
        if value == sequence {
            e.call(STRING_MAP_REMOVE_AT, &args![map, key]);
            break;
        }
        item = next;
    }
    let active = this.at(NiControllerManager::m_kActiveSequences);
    let mut slot = 0u32;
    if e.get(active, NiTPrimitiveSet::m_uiUsed) != 0 {
        loop {
            let base = e.get(active, NiTPrimitiveSet::m_pBase);
            if e.mem.u32(base.wrapping_add(slot.wrapping_mul(4))) == sequence {
                let used = e
                    .get(active, NiTPrimitiveSet::m_uiUsed)
                    .wrapping_add(0xffff_ffff);
                e.set(active, NiTPrimitiveSet::m_uiUsed, used);
                let base = e.get(active, NiTPrimitiveSet::m_pBase);
                let last = e.mem.u32(base.wrapping_add(used.wrapping_mul(4)));
                e.mem.set_u32(base.wrapping_add(slot.wrapping_mul(4)), last);
            }
            slot = slot.wrapping_add(1);
            if slot >= e.get(active, NiTPrimitiveSet::m_uiUsed) {
                break;
            }
        }
    }
    e.call(SEQUENCE_DETACH, &args![sequence]);
    e.mem.set_u32(sequence + 0x40, 0);
    let held = e.mem.u32(out.addr());
    if held != sequence {
        if held != 0 {
            release(e, held);
        }
        e.mem.set_u32(out.addr(), sequence);
        add_ref(e, sequence);
    }
    release(e, sequence);
}

// Translated from 00a2e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`, `this` = the manager): runs the sequence detach
/// function `00a30880` on every non-null sequence and clears its owner (`+0x40`), then empties the
/// sequence array (`00a2e620`) and, as a tail jump, runs `00b601a0` (`RemoveAll`) on the name map
/// (`+0x50`).
pub fn fn_00a2e9a0(e: &mut Engine, this: Ptr<NiControllerManager>) {
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                e.call(SEQUENCE_DETACH, &args![sequence]);
                e.mem.set_u32(sequence + 0x40, 0);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    fn_00a2e620(e, sequences);
    e.call(
        MAP_REMOVE_ALL,
        &args![this.at(NiControllerManager::m_kStringMap)],
    );
}

/// One write through a stream's `NiBinaryStream`: `function(stream, data, size, &sizes, 1)` with
/// the sizes word set to `size`, and the byte count it answers added to the position.
fn write_stream(e: &mut Engine, output: Ptr<NiBinaryStream>, data: u32, size: u32) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        let function = e.get(output, NiBinaryStream::m_pfnWrite);
        let done = e
            .call(function, &args![output, data, size, sizes, 1u32])
            .u32();
        let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            output,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

/// The read counterpart of [`write_stream`] (`m_pfnRead`).
fn read_stream(e: &mut Engine, input: Ptr<NiBinaryStream>, data: u32, size: u32) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, data, size, sizes, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

// Translated from 00a2e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::SaveBinary` (Xbox PDB): runs `NiTimeController::SaveBinary`
/// (`00a6d730`), writes the cumulative byte (`+0x68`, 1 byte) and the 16-bit sequence count
/// (`+0x3e`) as a 4-byte word through the stream's output stream, saves the link id of
/// each non-null sequence with the stream's virtual slot 0x30 (`SaveLinkID`) and then that of the
/// object palette (`+0x78`).
pub fn ni_controller_manager_save_binary(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    stream: Ptr<NiStream>,
) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let cumulative = e.get(this, NiControllerManager::m_bCumulative);
    e.with_stack(8, |e, data| {
        e.mem.set_u8(data.addr(), cumulative);
        write_stream(e, output, data.addr(), 1);
    });
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let count = u32::from(e.get(sequences, NiTArray::m_usSize));
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    e.with_stack(8, |e, data| {
        e.mem.set_u32(data.addr(), count);
        write_stream(e, output, data.addr(), 4);
    });
    let mut index = 0u32;
    if count != 0 {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![sequence]);
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    let palette = e.get(this, NiControllerManager::m_spObjectPalette);
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![palette]);
}

// Translated from 00a2eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::LoadBinary` (Xbox PDB): runs `NiTimeController::LoadBinary`
/// (`00a6d5e0`), reads one byte through the stream's input stream and stores whether it is not
/// 0 as the cumulative flag (`+0x68`), runs `NiStream::ReadMultipleLinkIDs` (`00a64670`) and,
/// for NIF versions from `0x0a010068` on (`+0xd8` of the stream), makes the object palette (`+0x78`)
/// the object `NiStream::ResolveLinkID` (`00a64620`) answers, as a pointer assignment.
pub fn ni_controller_manager_load_binary(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    stream: Ptr<NiStream>,
) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let flag = e.with_stack(8, |e, data| {
        read_stream(e, input, data.addr(), 1);
        e.mem.u8(data.addr())
    });
    e.set(
        this,
        NiControllerManager::m_bCumulative,
        u8::from(flag != 0),
    );
    e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    if e.get(stream, NiStream::m_uiNifFileVersion) >= 0x0a01_0068 {
        let palette = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
        assign_pointer(e, this.addr() + 0x78, palette);
    }
}

// Translated from 00a2eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<NiAVObject *>::_scalar_deleting_destructor_` (Xbox PDB name from the
/// engine map): stores the `NiTFixedStringMap` vtable and runs `RemoveAll` (`00aa01f0`), stores
/// the `NiTFixedStringMapBase` vtable and runs it again, frees the bucket table (`+8`) and, when bit 0
/// of `flags` is set, gives `this` back to the memory manager. Answers `this`.
pub fn ni_t_fixed_string_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), FIXED_STRING_MAP_VTABLE);
    ni_t_fixed_string_map_base_remove_all(e, this);
    e.mem.set_u32(this.addr(), FIXED_STRING_MAP_BASE_VTABLE);
    ni_t_fixed_string_map_base_remove_all(e, this);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(TABLE_FREE, &args![table]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a2eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiControllerSequence>, NiTNewInterface<..>>::_scalar_deleting_destructor_`
/// (a name from the vtable `01096184` this stores, whose slot 0 this is): stores that vtable,
/// frees the element storage (`00a47600`, the first word of the array's pointer at `+4`) and,
/// when bit 0 of `flags` is set, gives `this` back to the memory manager. Answers `this`.
pub fn fn_00a2eb80(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    let elements = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    e.call(ARRAY_FREE, &args![elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a2ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::~NiControllerManager` (Xbox PDB; the engine map has no name here, the
/// scalar deleting destructor `00a2ef80` calls this body): stores the class vtable, empties the
/// sequence array (`00a2e620`), clears the temporary blend set's count (`+0x74`), destroys the
/// listener (`00a41450` then `Deallocate`) when there is one, releases the palette (`+0x78`),
/// frees the temporary blend storage (`00a47600`), runs the name map's destructor body
/// (`00a2e460`), frees the active set's storage (`+0x44`), stores the array vtable at `+0x34` and
/// frees the sequence storage (`00a47600`), then continues in the `NiTimeController` destructor
/// body (`00a6d290`).
pub fn ni_controller_manager_destructor(e: &mut Engine, this: Ptr<NiControllerManager>) {
    e.mem.set_u32(this.addr(), NI_CONTROLLER_MANAGER_VTABLE);
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    fn_00a2e620(e, sequences);
    let listener = e.get(this, NiControllerManager::pListener).addr();
    e.mem.set_u32(this.addr() + 0x74, 0);
    if listener != 0 {
        e.call(LISTENER_DESTRUCT, &args![listener]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, listener]);
    }
    let palette = e.get(this, NiControllerManager::m_spObjectPalette).addr();
    if palette != 0 {
        release(e, palette);
    }
    let blend_storage = e.mem.u32(this.addr() + 0x6c);
    e.call(ARRAY_FREE, &args![blend_storage]);
    fn_00a2e460(e, this.at(NiControllerManager::m_kStringMap));
    let active_storage = e.mem.u32(this.addr() + 0x44);
    e.call(TABLE_FREE, &args![active_storage]);
    let elements = e.get(sequences, NiTArray::m_pBase);
    e.mem.set_u32(sequences.addr(), ARRAY_VTABLE);
    e.call(ARRAY_FREE, &args![elements]);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 00a2ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_controller_manager_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CONTROLLER_MANAGER_RTTI)
}

// Translated from 00a2ec50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::RemoveSequence` (Xbox PDB): runs `00a2e7d0` with `sequence` and a
/// local `NiPointer` (starting null) to receive it, then releases what that holds.
pub fn ni_controller_manager_remove_sequence(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    sequence: Ptr,
) {
    e.with_stack(4, |e, removed| {
        fn_00a2e7d0(e, this, sequence.addr(), removed);
        let held = e.mem.u32(removed.addr());
        if held != 0 {
            release(e, held);
        }
    });
}

// Translated from 00a2ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, `this` = an
/// `NiTArray<NiPointer<NiControllerSequence>>`): `AddFirstEmpty(&element)`. A null element answers
/// `0xffffffff`. When some slot is empty (the element count `+0xc` is below the size `+0xa`) the
/// slots are scanned from the last down (as 16-bit signed indexes) for the lowest slot of the top-most
/// run of empty ones; the element is assigned there (the old one released, the new one counted), the
/// element count goes up and the index is answered. Otherwise the element goes after the last: the
/// array is grown first (`00a47770`, by the step `+0xe` plus the size) when the size has reached the
/// capacity, then `00a2e680` stores it at the old size, which is answered.
pub fn fn_00a2ec90(e: &mut Engine, this: Ptr<NiTArray>, element: Ptr) -> u32 {
    if e.mem.u32(element.addr()) == 0 {
        return 0xffff_ffff;
    }
    let size = e.get(this, NiTArray::m_usSize);
    if e.get(this, NiTArray::m_usESize) < size {
        let mut candidate = u32::from(size);
        let mut index = size.wrapping_sub(1);
        if index as i16 >= 0 {
            let base = e.get(this, NiTArray::m_pBase);
            loop {
                let slot = base.wrapping_add((index as i16 as i32 as u32).wrapping_mul(4));
                if e.mem.u32(slot) == 0 {
                    candidate = u32::from(index);
                } else if i32::from(candidate as u16 as i16) != i32::from(size) {
                    break;
                }
                index = index.wrapping_sub(1);
                if (index as i16) < 0 {
                    break;
                }
            }
        }
        let found = i32::from(candidate as u16 as i16);
        if found != i32::from(size) {
            let base = e.get(this, NiTArray::m_pBase);
            let slot = base.wrapping_add((found as u32).wrapping_mul(4));
            assign_pointer_from(e, slot, element.addr());
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
            return found as u32;
        }
    }
    let size = u32::from(size);
    if size >= u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        let step = u32::from(e.get(this, NiTArray::m_usGrowBy));
        ni_t_array_ni_controller_sequence_set_size(e, this, step + size);
    }
    fn_00a2e680(e, this, size, element);
    size
}

// Translated from 00a2ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `NiDefaultAVObjectPalette`'s constructor
/// (the class whose vtable `01096284` it stores). Runs `NiObject`'s constructor, stores the vtable, the
/// bucket count `hash_size` (`+0xc`), the `NiTFixedStringMapBase` vtable for the embedded map (`+8`)
/// and a zero item count (`+0x14`), makes the zeroed bucket table (`+0x10`, `hash_size * 4`
/// bytes), stores the `NiTFixedStringMap` vtable and the scene (`+0x18`), runs `00a6e960` (the
/// scene walk that fills the palette) and answers `this`.
pub fn fn_00a2ed80(
    e: &mut Engine,
    this: Ptr<NiDefaultAVObjectPalette>,
    scene: Ptr,
    hash_size: u32,
) -> Ptr<NiDefaultAVObjectPalette> {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    let address = this.addr();
    e.mem.set_u32(address, NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE);
    e.mem.set_u32(address + 0xc, hash_size);
    e.mem.set_u32(address + 8, FIXED_STRING_MAP_BASE_VTABLE);
    e.mem.set_u32(address + 0x14, 0);
    let table = e
        .call(TABLE_ALLOCATE, &args![hash_size.wrapping_mul(4)])
        .u32();
    e.mem.set_u32(address + 0x10, table);
    let buckets = e.mem.u32(address + 0xc);
    e.call(MEMSET, &args![table, 0u32, buckets.wrapping_mul(4)]);
    e.mem.set_u32(address + 8, FIXED_STRING_MAP_VTABLE);
    e.set(this, NiDefaultAVObjectPalette::m_pkScene, scene);
    e.call(PALETTE_FILL, &args![this]);
    this
}

// Translated from 00a2edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::SetScene` (Xbox PDB): stores the scene at `+0x18`.
pub fn ni_default_av_object_palette_set_scene(
    e: &mut Engine,
    this: Ptr<NiDefaultAVObjectPalette>,
    scene: Ptr,
) {
    e.set(this, NiDefaultAVObjectPalette::m_pkScene, scene);
}

// Translated from 00a2ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::GetAVObject` (Xbox PDB): looks the name (the handle that `name`
/// points to; 0 answers 0) up in the embedded map's buckets (`handle % bucket count`, the bucket
/// count at `+0xc` as the unsigned divisor: 0 stops the exe with a divide error), walks the chain
/// (next at +0, key at +4, value at +8) and answers the value of the item whose key is the handle,
/// or 0.
pub fn ni_default_av_object_palette_get_av_object(
    e: &mut Engine,
    this: Ptr<NiDefaultAVObjectPalette>,
    name: Ptr,
) -> u32 {
    let handle = e.mem.u32(name.addr());
    if handle == 0 {
        return 0;
    }
    let map = this.at(NiDefaultAVObjectPalette::m_kMap);
    let bucket = handle % e.get(map, NiTPointerMap::m_uiHashSize);
    let table = e.get(map, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        if handle == e.mem.u32(item + 4) {
            return e.mem.u32(item + 8);
        }
        item = e.mem.u32(item);
    }
    0
}

// Translated from 00a2ee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::SetAVObject` (Xbox PDB): with no object, removes the name (the
/// handle that `name` points to) from the embedded map (`00aa0250`, `(&name)`); otherwise
/// `SetAt` (`00a6e980`, `(&name, object)`) on it.
pub fn ni_default_av_object_palette_set_av_object(
    e: &mut Engine,
    this: Ptr<NiDefaultAVObjectPalette>,
    name: Ptr,
    object: Ptr,
) {
    let map = this.at(NiDefaultAVObjectPalette::m_kMap);
    if object.is_null() {
        e.call(PALETTE_MAP_REMOVE_AT, &args![map, name]);
    } else {
        ni_t_fixed_string_map_base_set_at(e, map, name, object);
    }
}

// Translated from 00a2ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::_scalar_deleting_destructor_` (Xbox PDB): stores the class vtable,
/// then for the embedded map (`+8`) the `NiTFixedStringMap` vtable and `RemoveAll` (`00aa01f0`),
/// the `NiTFixedStringMapBase` vtable and `RemoveAll` again, frees its bucket table, stores the
/// `NiAVObjectPalette` vtable, runs `NiObject`'s destructor body (`00a5d3d0`) and, when bit 0 of
/// `flags` is set, the sized delete (`00aa1460`, `(this, 0x1c)`). Answers `this`.
pub fn ni_default_av_object_palette_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiDefaultAVObjectPalette>,
    flags: u32,
) -> Ptr<NiDefaultAVObjectPalette> {
    e.mem
        .set_u32(this.addr(), NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE);
    let map = this.at(NiDefaultAVObjectPalette::m_kMap);
    e.mem.set_u32(map.addr(), FIXED_STRING_MAP_VTABLE);
    ni_t_fixed_string_map_base_remove_all(e, map);
    e.mem.set_u32(map.addr(), FIXED_STRING_MAP_BASE_VTABLE);
    ni_t_fixed_string_map_base_remove_all(e, map);
    let table = e.get(map, NiTPointerMap::m_ppkHashTable);
    e.call(TABLE_FREE, &args![table]);
    e.mem.set_u32(this.addr(), NI_AV_OBJECT_PALETTE_VTABLE);
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00a2eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): `NiControllerManager`'s default constructor (the
/// class whose vtable `0109619c` it stores). Sets the fields up (the time-controller constructor
/// `00a6d220`, an empty sequence array with growth 10, no active sequences, the name map
/// with 0x25 zeroed buckets, cumulative 0, no temporary blend sequences, no palette) and makes
/// the animation-note listener (0x10 bytes from the memory manager, constructed by `00a414f0`;
/// 0 stored when there is no memory). Answers `this`.
pub fn fn_00a2eed0(e: &mut Engine, this: Ptr<NiControllerManager>) -> Ptr<NiControllerManager> {
    construct_manager_fields(e, this, 0);
    construct_listener(e, this);
    this
}

// Translated from 00a2efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `NiControllerManager`'s constructor from a
/// target and the cumulative flag (the byte stored at `+0x68`). Sets the fields up as `00a2eed0`
/// does, sets the target (`00a6d450`), makes the palette (`00aa13e0` for 0x1c bytes and
/// `00a2ed80` with the target and 0x4f buckets) and stores it at `+0x78` as a pointer assignment,
/// then makes the listener. Answers `this`.
pub fn fn_00a2efa0(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    target: Ptr,
    cumulative: u8,
) -> Ptr<NiControllerManager> {
    construct_manager_fields(e, this, cumulative);
    e.call(TIME_CONTROLLER_SET_TARGET, &args![this, target]);
    let palette = new_palette(e, target.addr(), 0x4f);
    assign_pointer(e, this.addr() + 0x78, palette);
    construct_listener(e, this);
    this
}

// Translated from 00a2f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; the shape of an `AddSequence` with a name and
/// a "store targets" flag, name unconfirmed): answers false when `this` is set and the sequence
/// already has an owner (`+0x40`). Otherwise makes this manager its owner; when its accum root
/// name (`+0x5c`) was empty gives it the target's name (`004eeae0`, `(target + 8)`); when
/// `store_targets` is set runs the sequence's `00a32c70` with the target and, if that answers false,
/// clears the owner and answers false. When the name handle `name` points to is not the empty
/// string, gives the sequence that name (`0043b9f0`). Then appends the sequence to the sequence
/// array through a counted local (`00a2ec90`) and enters it in the name map under its own name
/// (`00a642a0`, `(name, sequence)`); answers true.
pub fn fn_00a2f0c0(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    sequence: Ptr<NiControllerSequence>,
    name: Ptr,
    store_targets: bool,
) -> bool {
    if !this.is_null() && !e.get(sequence, NiControllerSequence::m_pkOwner).is_null() {
        return false;
    }
    let accum_root_name = e.get(sequence, NiControllerSequence::m_kAccumRootName);
    e.set(sequence, NiControllerSequence::m_pkOwner, this.cast());
    if accum_root_name == 0 {
        let target = e.get(this, NiControllerManager::m_pkTarget).addr();
        e.call(SEQUENCE_SET_NAME_FROM_TARGET, &args![sequence, target + 8]);
    }
    if store_targets {
        let target = e.get(this, NiControllerManager::m_pkTarget);
        if !e
            .call(SEQUENCE_ATTACH_TO_TARGET, &args![sequence, target])
            .bool()
        {
            e.set(sequence, NiControllerSequence::m_pkOwner, Ptr::NULL);
            return false;
        }
    }
    if e.mem.u32(name.addr()) != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(SEQUENCE_SET_NAME, &args![sequence, name]);
    }
    add_to_sequence_array(e, this, sequence.addr());
    let sequence_name = e.get(sequence, NiControllerSequence::m_kName);
    e.call(
        STRING_MAP_SET_AT,
        &args![
            this.at(NiControllerManager::m_kStringMap),
            sequence_name,
            sequence
        ],
    );
    true
}

/// The inline sequence that appends `sequence` to the manager's sequence array through a
/// counted local `NiPointer`: the count goes up, `00a2ec90` adds the local, the count goes down
/// (the object deleted at zero).
fn add_to_sequence_array(e: &mut Engine, this: Ptr<NiControllerManager>, sequence: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), sequence);
        add_ref(e, sequence);
        fn_00a2ec90(e, this.at(NiControllerManager::m_kSequenceArray), local);
        release(e, sequence);
    });
}

/// The inline sequence of a new manager's sequence bookkeeping: the sequence's owner is
/// made `this` when it has none.
fn adopt_sequence(e: &mut Engine, this: Ptr<NiControllerManager>, sequence: u32) {
    let sequence = Ptr::<NiControllerSequence>::new(sequence);
    if e.get(sequence, NiControllerSequence::m_pkOwner).is_null() {
        e.set(sequence, NiControllerSequence::m_pkOwner, this.cast());
    }
}

// Translated from 00a2f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): answers a temporary blend sequence for
/// `source`. Takes the first sequence of the temporary pool (`+0x6c`, count `+0x74`) whose state
/// (`+0x44`) is 0; when there is none makes a new `"__TempBlendSequence__"` of the source's
/// array size (`00aa13e0` for 0x74 bytes, a fixed string from `00a5b690`, the sequence
/// constructor `00a32a10` with that size and growth 0xc, the temporary name released), adds it to the
/// pool (`00e8ea60`), and, if it has no owner, makes this manager its owner, gives it the target's
/// name as its accum root name when it has none and the static name `011f49e0` as its own name
/// unless that is the empty string, then appends it to the sequence array and the name map
/// (`00a642a0`). A reused one gets its detach function `00a30880`, this manager as owner when it
/// had none and the target's name as accum root name, and the flag 1. At the end runs
/// `00a32fa0` on `source` with the temporary sequence, the source's array size, `extra` and
/// the flag, and answers the temporary sequence.
pub fn fn_00a2f170(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    source: Ptr,
    extra: u32,
) -> Ptr<NiControllerSequence> {
    let size = e.mem.u32(source.addr() + 0xc);
    let pool = this.at(NiControllerManager::m_kTempBlendSeqs);
    let used = e.get(pool, NiTPrimitiveSet::m_uiUsed);
    let mut temp = 0u32;
    if used != 0 {
        let base = e.get(pool, NiTPrimitiveSet::m_pBase);
        let mut index = 0u32;
        loop {
            let candidate = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if e.mem.u32(candidate + 0x44) == 0 {
                temp = candidate;
                break;
            }
            index += 1;
            if index >= used {
                break;
            }
        }
    }
    let mut flag = 0u32;
    if temp == 0 {
        let object = e.call(OPERATOR_NEW, &args![0x74u32]).u32();
        if object != 0 {
            let handle = e
                .call(FIXED_STRING_FROM_CHARS, &args![TEXT_TEMP_BLEND_SEQUENCE])
                .u32();
            let (created, name_handle) = e.with_stack(4, |e, name| {
                e.mem.set_u32(name.addr(), handle);
                let created = e
                    .call(SEQUENCE_CONSTRUCT, &args![object, name, size, 0xcu32])
                    .u32();
                (created, e.mem.u32(name.addr()))
            });
            temp = created;
            fixed_string_release(e, name_handle);
        }
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), temp);
            if temp != 0 {
                add_ref(e, temp);
            }
            e.call(SET_ADD, &args![pool, local]);
            if temp != 0 {
                release(e, temp);
            }
        });
        if e.mem.u32(temp + 0x40) == 0 {
            let accum_root_name = e.mem.u32(temp + 0x5c);
            e.mem.set_u32(temp + 0x40, this.addr());
            if accum_root_name == 0 {
                let target = e.get(this, NiControllerManager::m_pkTarget).addr();
                let target_name = e.mem.u32(target + 8);
                assign_fixed_string(e, temp + 0x5c, target_name);
            }
            let static_name = e.mem.u32(FIXED_STRING_011F49E0);
            if static_name != e.global::<u32>(EMPTY_FIXED_STRING) {
                assign_fixed_string(e, temp + 8, static_name);
            }
            add_to_sequence_array(e, this, temp);
            let sequence_name = e.mem.u32(temp + 8);
            e.call(
                STRING_MAP_SET_AT,
                &args![
                    this.at(NiControllerManager::m_kStringMap),
                    sequence_name,
                    temp
                ],
            );
        }
    } else {
        e.call(SEQUENCE_DETACH, &args![temp]);
        adopt_sequence(e, this, temp);
        let target = e.get(this, NiControllerManager::m_pkTarget).addr();
        let target_name = e.mem.u32(target + 8);
        assign_fixed_string(e, temp + 0x5c, target_name);
        flag = 1;
    }
    e.call(
        SEQUENCE_COPY_BLEND_POSE,
        &args![source, temp, size, extra, flag],
    );
    Ptr::new(temp)
}

/// The inline `NiTSet::Add` slot step of `ProcessClone`: when the used count has reached the
/// capacity, `00e8e3a0` reallocates to 1 or double; then takes the slot at the used count and
/// raises it. Answers the slot's address.
fn next_set_slot(e: &mut Engine, set: Ptr<NiTPrimitiveSet>) -> u32 {
    let alloced = e.get(set, NiTPrimitiveSet::m_uiAlloced);
    if e.get(set, NiTPrimitiveSet::m_uiUsed) == alloced {
        let new_size = if alloced == 0 {
            1
        } else {
            alloced.wrapping_add(alloced)
        };
        e.call(SET_REALLOC, &args![set, new_size]);
    }
    let used = e.get(set, NiTPrimitiveSet::m_uiUsed);
    let base = e.get(set, NiTPrimitiveSet::m_pBase);
    e.set(set, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
    base.wrapping_add(used.wrapping_mul(4))
}

// Translated from 00a2f380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the manager's member copy for a clone,
/// `this` being the source. Runs the `NiTimeController` copy (`00a6cfe0`, `(destination, process)`),
/// copies the cumulative flag, makes a clone of each non-null sequence (virtual slot 0x48,
/// `CreateClone`, with `process`) and adds it to the destination through `00a2f0c0` (name `011f49e0`,
/// no stored targets), and clones the palette (`+0x78`, `CreateClone`) into the destination's as a
/// pointer assignment.
pub fn fn_00a2f380(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    destination: Ptr<NiControllerManager>,
    process: Ptr,
) {
    e.call(
        TIME_CONTROLLER_COPY_MEMBERS,
        &args![this, destination, process],
    );
    let cumulative = e.get(this, NiControllerManager::m_bCumulative);
    e.set(destination, NiControllerManager::m_bCumulative, cumulative);
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                let clone = e.vcall(sequence, SLOT_CREATE_CLONE, &args![process]).u32();
                fn_00a2f0c0(
                    e,
                    destination,
                    Ptr::new(clone),
                    Ptr::new(FIXED_STRING_011F49E0),
                    false,
                );
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    let palette = e.get(this, NiControllerManager::m_spObjectPalette).addr();
    if palette != 0 {
        let clone = e.vcall(palette, SLOT_CREATE_CLONE, &args![process]).u32();
        assign_pointer(e, destination.addr() + 0x78, clone);
    }
}

// Translated from 00a2f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::ProcessClone` (Xbox PDB): runs `NiTimeController::ProcessClone`
/// (`00a6d0c0`); for each non-null sequence runs its `ProcessClone` (virtual slot 0x68) and its
/// `00a32c70` with the target (`+0x2c`); looks this manager's clone up in the cloning process's
/// map (the first word of `process`; `00a53c30`, result word starting 0); then for each
/// sequence of the temporary pool (`+0x6c`, count `+0x74`) looks it up in the same map with the
/// map's own `HashFunction` (slot 4) and `IsKeysEqual` (slot 8) and adds what it finds (its clone, counted
/// while it is added) to the clone's temporary pool (`+0x6c` of the clone, grown first by
/// `00e8e3a0` when full), or, when it has none, the sequence itself; finally runs the palette's
/// `ProcessClone` (virtual slot 0x68).
pub fn ni_controller_manager_process_clone(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    process: Ptr,
) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, process]);
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                e.vcall(sequence, SLOT_PROCESS_CLONE, &args![process]);
                let target = e.get(this, NiControllerManager::m_pkTarget);
                e.call(SEQUENCE_ATTACH_TO_TARGET, &args![sequence, target]);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    let map = e.mem.u32(process.addr());
    let clone = e.with_stack(4, |e, found| {
        e.call(CLONE_MAP_GET_AT, &args![map, this, found]);
        e.mem.u32(found.addr())
    });
    let pool = this.at(NiControllerManager::m_kTempBlendSeqs);
    let mut position = 0u32;
    if e.get(pool, NiTPrimitiveSet::m_uiUsed) != 0 {
        let clone_pool = Ptr::<NiTPrimitiveSet>::new(clone.wrapping_add(0x6c));
        loop {
            let pool_base = e.get(pool, NiTPrimitiveSet::m_pBase);
            let map = e.mem.u32(process.addr());
            let offset = position.wrapping_mul(4);
            let sequence = e.mem.u32(pool_base.wrapping_add(offset));
            let bucket = e.vcall(map, SLOT_HASH_FUNCTION, &args![sequence]).u32();
            let table = e.mem.u32(map + 8);
            let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
            let mut counterpart = None;
            while item != 0 {
                let key = e.mem.u32(item + 4);
                if e.vcall(map, SLOT_IS_KEYS_EQUAL, &args![sequence, key])
                    .bool()
                {
                    counterpart = Some(e.mem.u32(item + 8));
                    break;
                }
                item = e.mem.u32(item);
            }
            match counterpart {
                Some(value) => {
                    if value != 0 {
                        add_ref(e, value);
                    }
                    let slot = next_set_slot(e, clone_pool);
                    assign_pointer(e, slot, value);
                    if value != 0 {
                        release(e, value);
                    }
                }
                None => {
                    let pool_base = e.get(pool, NiTPrimitiveSet::m_pBase);
                    let source = pool_base.wrapping_add(offset);
                    let slot = next_set_slot(e, clone_pool);
                    assign_pointer_from(e, slot, source);
                }
            }
            position = position.wrapping_add(1);
            if position >= e.get(pool, NiTPrimitiveSet::m_uiUsed) {
                break;
            }
        }
    }
    let palette = e.get(this, NiControllerManager::m_spObjectPalette).addr();
    if palette != 0 {
        e.vcall(palette, SLOT_PROCESS_CLONE, &args![process]);
    }
}

// Translated from 00a2f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::RegisterStreamables` (Xbox PDB): false when
/// `NiTimeController::RegisterStreamables` (`00a6d0f0`) answers false. Otherwise runs the
/// `RegisterStreamables` (virtual slot 0x54, with the stream) of each non-null sequence and of the
/// palette when there is one; without a palette makes one (`00aa13e0` for 0x1c bytes, `00a2ed80` with the
/// target and 0x25 buckets) and stores it at `+0x78` as a pointer assignment (not registered).
/// Answers true.
pub fn ni_controller_manager_register_streamables(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    stream: Ptr,
) -> bool {
    if !e
        .call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    let mut index = 0u32;
    if 0 < e.get(sequences, NiTArray::m_usSize) {
        loop {
            let base = e.get(sequences, NiTArray::m_pBase);
            let sequence = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if sequence != 0 {
                e.vcall(sequence, SLOT_REGISTER_STREAMABLES, &args![stream]);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(sequences, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    let palette = e.get(this, NiControllerManager::m_spObjectPalette).addr();
    if palette != 0 {
        e.vcall(palette, SLOT_REGISTER_STREAMABLES, &args![stream]);
        return true;
    }
    let target = e.get(this, NiControllerManager::m_pkTarget).addr();
    let made = new_palette(e, target, 0x25);
    assign_pointer(e, this.addr() + 0x78, made);
    true
}

// Translated from 00a2f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::LinkObject` (Xbox PDB): runs `NiTimeController::LinkObject` (`00a6d500`),
/// sizes the sequence array to the stream's number of link ids (`00a63460`, `00a47770`) and for each
/// of them takes the sequence `NiStream::GetObjectFromLinkID` (`00a63430`) answers. A sequence
/// that already has an owner is left alone when `this` is set. Otherwise this manager becomes its
/// owner, it gets the target's name as accum root name when it has none and the static name `011f49e0`
/// as its own name unless that is the empty string, and it is appended to the sequence array
/// (`00a2ec90`, through a counted local) and entered in the name map (`00a642a0`).
pub fn ni_controller_manager_link_object(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let sequences = this.at(NiControllerManager::m_kSequenceArray);
    ni_t_array_ni_controller_sequence_set_size(e, sequences, count);
    let mut remaining = count;
    if remaining == 0 {
        return;
    }
    loop {
        let sequence = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if this.is_null() || e.mem.u32(sequence + 0x40) == 0 {
            let accum_root_name = e.mem.u32(sequence + 0x5c);
            e.mem.set_u32(sequence + 0x40, this.addr());
            if accum_root_name == 0 {
                let target = e.get(this, NiControllerManager::m_pkTarget).addr();
                let target_name = e.mem.u32(target + 8);
                assign_fixed_string(e, sequence + 0x5c, target_name);
            }
            let static_name = e.mem.u32(FIXED_STRING_011F49E0);
            if static_name != e.global::<u32>(EMPTY_FIXED_STRING) {
                assign_fixed_string(e, sequence + 8, static_name);
            }
            add_to_sequence_array(e, this, sequence);
            let sequence_name = e.mem.u32(sequence + 8);
            e.call(
                STRING_MAP_SET_AT,
                &args![
                    this.at(NiControllerManager::m_kStringMap),
                    sequence_name,
                    sequence
                ],
            );
        }
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
}

// Translated from 00a2f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`): answers false when `sequence` is not
/// inactive (state `+0x44`). Otherwise gets a temporary blend sequence for it (`00a2f170`, with
/// `extra`) and answers that sequence's `00a350d0` (`(sequence, weight, duration, priority, 1.0, 1.0,
/// 0)`), the two floats copied through the x87 stack (a signalling NaN goes on quieted).
pub fn fn_00a2f800(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    sequence: Ptr,
    first_float: f32,
    second_float: f32,
    third_word: u32,
    extra: u32,
) -> bool {
    if e.mem.u32(sequence.addr() + 0x44) != 0 {
        return false;
    }
    let temp = fn_00a2f170(e, this, sequence, extra);
    e.call(
        SEQUENCE_START_BLEND,
        &args![
            temp,
            sequence,
            x87_quiet(second_float),
            x87_quiet(first_float),
            third_word,
            1.0f32,
            1.0f32,
            0u32
        ],
    )
    .bool()
}

// Translated from 00a2f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::CreateClone` (Xbox PDB): allocates 0x7c bytes (`00aa13e0`) and, when
/// that gives memory, constructs a manager in it (`00a2eed0`); runs the member copy `00a2f380`
/// from this manager into it (into address 0 when there was no memory, as the exe does) and answers
/// the clone.
pub fn ni_controller_manager_create_clone(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    process: Ptr,
) -> Ptr<NiControllerManager> {
    let object = e.call(OPERATOR_NEW, &args![0x7cu32]).u32();
    if object == 0 {
        fn_00a2f380(e, this, Ptr::NULL, process);
        return Ptr::NULL;
    }
    let clone = fn_00a2eed0(e, Ptr::new(object));
    fn_00a2f380(e, this, clone, process);
    clone
}

// Translated from 00a47770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiControllerSequence>, NiTNewInterface<..>>::SetSize` (Xbox PDB name from
/// the engine map; `thiscall`, `RET 4`): nothing when `new_size` is the capacity (`+8`). When it is
/// below the size (`+0xa`) the elements from its low 16 bits up are released and cleared (the element
/// count `+0xc` going down for each) and the size becomes its low 16 bits. The capacity becomes
/// its low 16 bits; the storage is a new array (`00c1d790`, the capacity as its count) into which the
/// old elements are copied as pointer assignments (counted), the slots from the size to the capacity are
/// released and cleared, and the old storage is freed (`00a47600`); with a size of 0 the storage
/// pointer is cleared and the old one freed.
pub fn ni_t_array_ni_controller_sequence_set_size(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    new_size: u32,
) {
    if new_size == u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        return;
    }
    let size = u32::from(e.get(this, NiTArray::m_usSize));
    let low = new_size as u16;
    if new_size < size {
        let mut index = low;
        if low < e.get(this, NiTArray::m_usSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let slot = base.wrapping_add(u32::from(index) * 4);
                if e.mem.u32(slot) != 0 {
                    let element = e.mem.u32(slot);
                    if element != 0 {
                        release(e, element);
                        e.mem.set_u32(slot, 0);
                    }
                    let count = e.get(this, NiTArray::m_usESize);
                    e.set(this, NiTArray::m_usESize, count.wrapping_add(0xffff));
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        e.set(this, NiTArray::m_usSize, low);
    }
    let old_storage = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, low);
    if new_size != 0 {
        let storage = e.call(ARRAY_ALLOCATE, &args![u32::from(low)]).u32();
        e.set(this, NiTArray::m_pBase, storage);
        let mut index = 0u16;
        if 0 < e.get(this, NiTArray::m_usSize) {
            loop {
                let offset = u32::from(index) * 4;
                let storage = e.get(this, NiTArray::m_pBase);
                let slot = storage.wrapping_add(offset);
                let current = e.mem.u32(slot);
                if current != e.mem.u32(old_storage.wrapping_add(offset)) {
                    if current != 0 {
                        release(e, current);
                    }
                    let value = e.mem.u32(old_storage.wrapping_add(offset));
                    e.mem.set_u32(slot, value);
                    if value != 0 {
                        add_ref(e, value);
                    }
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        if index < e.get(this, NiTArray::m_usMaxSize) {
            loop {
                let storage = e.get(this, NiTArray::m_pBase);
                let slot = storage.wrapping_add(u32::from(index) * 4);
                let element = e.mem.u32(slot);
                if element != 0 {
                    release(e, element);
                    e.mem.set_u32(slot, 0);
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usMaxSize) {
                    break;
                }
            }
        }
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    e.call(ARRAY_FREE, &args![old_storage]);
}

// Translated from 00a6e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMapBase<DFALL<NiTMapItem<NiFixedString, NiAVObject *>>, NiAVObject *>::SetAt`
/// (Xbox PDB name from the engine map; `thiscall`, `RET 8`): the bucket is the handle that `key`
/// points to modulo the bucket count (`+4`; 0 stops the exe with a divide error). An item of the
/// chain (next at +0, key at +4) with the same handle is set with `00a6e550` (`(item, &key,
/// value)`); otherwise a new item (the map's virtual slot 4) is set the same way, put at the front
/// of the bucket and counted (`+0xc`).
pub fn ni_t_fixed_string_map_base_set_at(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    key: Ptr,
    value: Ptr,
) {
    let handle = e.mem.u32(key.addr());
    let bucket = handle % e.get(this, NiTPointerMap::m_uiHashSize);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        if handle == e.mem.u32(item + 4) {
            e.call(PALETTE_ITEM_SET, &args![this, item, key, value]);
            return;
        }
        item = e.mem.u32(item);
    }
    let created = e.vcall(this.addr(), SLOT_NEW_ITEM, &args![]).u32();
    e.call(PALETTE_ITEM_SET, &args![this, created, key, value]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    e.mem.set_u32(created, head);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), created);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 00aa01f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMapBase<DFALL<NiTMapItem<NiFixedString, NiAVObject *>>, NiAVObject *>::RemoveAll`
/// (Xbox PDB name from the engine map; `fastcall`): for each bucket (`+4` of them) takes the
/// items off the front of the chain one at a time and gives each to the map's virtual slot 8
/// (`DeleteItem`); the item count (`+0xc`) is then 0.
pub fn ni_t_fixed_string_map_base_remove_all(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    let mut bucket = 0u32;
    if e.get(this, NiTPointerMap::m_uiHashSize) != 0 {
        loop {
            let table = e.get(this, NiTPointerMap::m_ppkHashTable);
            if e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4))) != 0 {
                loop {
                    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
                    let slot = table.wrapping_add(bucket.wrapping_mul(4));
                    let item = e.mem.u32(slot);
                    let next = e.mem.u32(item);
                    e.mem.set_u32(slot, next);
                    e.vcall(this.addr(), SLOT_DELETE_ITEM, &args![item]);
                    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
                    if e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4))) == 0 {
                        break;
                    }
                }
            }
            bucket = bucket.wrapping_add(1);
            if bucket >= e.get(this, NiTPointerMap::m_uiHashSize) {
                break;
            }
        }
    }
    e.set(this, NiTPointerMap::m_uiCount, 0);
}

// Translated from 00aa0330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<NiAVObject *>::DeleteItem` (Xbox PDB name from the engine map; `thiscall`,
/// `RET 4`, `this` not read): for an item whose key handle (`+4`) is set, replaces it with the handle of
/// the empty fixed string (`00a5b690` of null) and releases the old one unless it is the empty string's
/// (`InterlockedDecrement` of the count at `handle - 8`); clears the value (`+8`), releases the key
/// handle that is now there the same way and gives the item back to the memory manager.
pub fn ni_t_fixed_string_map_delete_item(e: &mut Engine, _this: Ptr, item: Ptr) {
    let key = e.mem.u32(item.addr() + 4);
    if key != 0 {
        let empty = e.call(FIXED_STRING_FROM_CHARS, &args![0u32]).u32();
        e.mem.set_u32(item.addr() + 4, empty);
        fixed_string_release(e, key);
    }
    e.mem.set_u32(item.addr() + 8, 0);
    let current = e.mem.u32(item.addr() + 4);
    fixed_string_release(e, current);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, item]);
}

// Translated from 00a2f6c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiControllerManager`): allocates 0x7c bytes (`operator new`) and, when that
/// succeeds, runs the default constructor (`00a2eed0`) on them. Answers the object, or null.
pub fn fn_00a2f6c0(e: &mut Engine) -> Ptr<NiControllerManager> {
    let block = e.call(OPERATOR_NEW, &args![0x7cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00a2eed0(e, Ptr::new(block))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_f6c0, fn_00a2f6c0() -> Ptr<NiControllerManager>),
        entry!(0x00a2e1e0, ni_av_object_palette_get_rtti() -> Ptr),
        entry!(0x00a2ede0, ni_default_av_object_palette_get_rtti() -> Ptr),
        entry!(0x00a2ef80, ni_controller_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a2e1f0, ni_av_object_palette_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a2e220,
            ni_controller_manager_update(Ptr<NiControllerManager>, Ptr)
        ),
        entry!(0x00a2e280, fn_00a2e280(Ptr, Ptr, Ptr, f32, u32, u32, f32, Ptr) -> bool),
        entry!(
            0x00a2e2e0,
            ni_controller_manager_post_link_object(Ptr<NiControllerManager>, Ptr)
        ),
        entry!(
            0x00a2e320,
            ni_controller_manager_is_equal(
                Ptr<NiControllerManager>,
                Ptr<NiControllerManager>,
            ) -> bool
        ),
        entry!(
            0x00a2e420,
            fn_00a2e420(Ptr<NiControllerManager>, f32) -> bool
        ),
        entry!(0x00a2e460, fn_00a2e460(Ptr<NiTStringPointerMap>)),
        entry!(
            0x00a2e4d0,
            fn_00a2e4d0(Ptr<NiTStringPointerMap>, u32) -> Ptr<NiTStringPointerMap>
        ),
        entry!(
            0x00a2e510,
            fn_00a2e510(Ptr<NiTStringPointerMap>, u32) -> Ptr<NiTStringPointerMap>
        ),
        entry!(
            0x00a2e550,
            ni_t_string_pointer_map_scalar_deleting_destructor(
                Ptr<NiTStringPointerMap>,
                u32,
            )
                -> Ptr<NiTStringPointerMap>
        ),
        entry!(
            0x00a2e570,
            fn_00a2e570(Ptr<NiControllerManager>, u32, bool) -> Ptr
        ),
        entry!(0x00a2e620, fn_00a2e620(Ptr<NiTArray>)),
        entry!(0x00a2e680, fn_00a2e680(Ptr<NiTArray>, u32, Ptr)),
        entry!(0x00a2e750, fn_00a2e750(Ptr, u32) -> Ptr),
        entry!(
            0x00a2e790,
            ni_t_fixed_string_map_base_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a2e7d0, fn_00a2e7d0(Ptr<NiControllerManager>, u32, Ptr)),
        entry!(0x00a2e9a0, fn_00a2e9a0(Ptr<NiControllerManager>)),
        entry!(
            0x00a2e9f0,
            ni_controller_manager_save_binary(Ptr<NiControllerManager>, Ptr<NiStream>)
        ),
        entry!(
            0x00a2eaa0,
            ni_controller_manager_load_binary(Ptr<NiControllerManager>, Ptr<NiStream>)
        ),
        entry!(
            0x00a2eb40,
            ni_t_fixed_string_map_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a2eb80, fn_00a2eb80(Ptr<NiTArray>, u32) -> Ptr<NiTArray>),
        entry!(
            0x00a2ebb0,
            ni_controller_manager_destructor(Ptr<NiControllerManager>)
        ),
        entry!(0x00a2ec40, ni_controller_manager_get_rtti() -> Ptr),
        entry!(
            0x00a2ec50,
            ni_controller_manager_remove_sequence(Ptr<NiControllerManager>, Ptr)
        ),
        entry!(0x00a2ec90, fn_00a2ec90(Ptr<NiTArray>, Ptr) -> u32),
        entry!(
            0x00a2ed80,
            fn_00a2ed80(Ptr<NiDefaultAVObjectPalette>, Ptr, u32) -> Ptr<NiDefaultAVObjectPalette>
        ),
        entry!(
            0x00a2edf0,
            ni_default_av_object_palette_set_scene(Ptr<NiDefaultAVObjectPalette>, Ptr)
        ),
        entry!(
            0x00a2ee00,
            ni_default_av_object_palette_get_av_object(Ptr<NiDefaultAVObjectPalette>, Ptr) -> u32
        ),
        entry!(
            0x00a2ee40,
            ni_default_av_object_palette_set_av_object(Ptr<NiDefaultAVObjectPalette>, Ptr, Ptr)
        ),
        entry!(
            0x00a2ee70,
            ni_default_av_object_palette_scalar_deleting_destructor(
                Ptr<NiDefaultAVObjectPalette>,
                u32,
            ) -> Ptr<
                NiDefaultAVObjectPalette,
            >
        ),
        entry!(
            0x00a2eed0,
            fn_00a2eed0(Ptr<NiControllerManager>) -> Ptr<NiControllerManager>
        ),
        entry!(
            0x00a2efa0,
            fn_00a2efa0(Ptr<NiControllerManager>, Ptr, u8) -> Ptr<NiControllerManager>
        ),
        entry!(
            0x00a2f0c0,
            fn_00a2f0c0(Ptr<NiControllerManager>, Ptr<NiControllerSequence>, Ptr, bool) -> bool
        ),
        entry!(
            0x00a2f170,
            fn_00a2f170(Ptr<NiControllerManager>, Ptr, u32) -> Ptr<NiControllerSequence>
        ),
        entry!(
            0x00a2f380,
            fn_00a2f380(Ptr<NiControllerManager>, Ptr<NiControllerManager>, Ptr)
        ),
        entry!(
            0x00a2f420,
            ni_controller_manager_process_clone(Ptr<NiControllerManager>, Ptr)
        ),
        entry!(
            0x00a2f610,
            ni_controller_manager_register_streamables(Ptr<NiControllerManager>, Ptr) -> bool
        ),
        entry!(
            0x00a2f6e0,
            ni_controller_manager_link_object(Ptr<NiControllerManager>, Ptr)
        ),
        entry!(
            0x00a2f800,
            fn_00a2f800(Ptr<NiControllerManager>, Ptr, f32, f32, u32, u32) -> bool
        ),
        entry!(
            0x00a2f850,
            ni_controller_manager_create_clone(
                Ptr<NiControllerManager>,
                Ptr,
            ) -> Ptr<NiControllerManager>
        ),
        entry!(
            0x00a47770,
            ni_t_array_ni_controller_sequence_set_size(Ptr<NiTArray>, u32)
        ),
        entry!(
            0x00a6e980,
            ni_t_fixed_string_map_base_set_at(Ptr<NiTPointerMap>, Ptr, Ptr)
        ),
        entry!(
            0x00aa01f0,
            ni_t_fixed_string_map_base_remove_all(Ptr<NiTPointerMap>)
        ),
        entry!(0x00aa0330, ni_t_fixed_string_map_delete_item(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The vtable of the test objects and its targets: `DeleteThis` (4), `CreateClone`
    /// (0x48), `RegisterStreamables` (0x54), `IsEqual` (0x5c), `ProcessClone` (0x68),
    /// `Deactivate` (0x8c).
    const OBJECT_VTABLE: u32 = 0x0300_0000;
    const DELETE_THIS: u32 = 0x0300_1004;
    const CREATE_CLONE: u32 = 0x0300_1048;
    const REGISTER: u32 = 0x0300_1054;
    const IS_EQUAL: u32 = 0x0300_105c;
    const PROCESS_CLONE: u32 = 0x0300_1068;
    const DEACTIVATE: u32 = 0x0300_108c;

    /// An engine with the test vtable, counting `InterlockedIncrement`/`Decrement` doubles and a
    /// call log.
    fn setup() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x8c / 4 + 1];
        slots[1] = DELETE_THIS;
        slots[0x48 / 4] = CREATE_CLONE;
        slots[0x54 / 4] = REGISTER;
        slots[0x5c / 4] = IS_EQUAL;
        slots[0x68 / 4] = PROCESS_CLONE;
        slots[0x8c / 4] = DEACTIVATE;
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A counted test object (vtable [`OBJECT_VTABLE`], count 1) with its first fields set.
    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn count_of(e: &Engine, object: u32) -> u32 {
        e.mem.u32(object + 4)
    }

    fn manager(e: &mut Engine) -> Ptr<NiControllerManager> {
        e.new_object::<NiControllerManager>()
    }

    /// Puts `elements` into the manager's sequence array (storage `elements.len() + spare`
    /// slots, size and capacity as given).
    fn set_sequences(
        e: &mut Engine,
        this: Ptr<NiControllerManager>,
        elements: &[u32],
        spare: u16,
    ) -> u32 {
        let storage = e
            .mem
            .alloc(4 * (elements.len() as u32 + u32::from(spare)) + 4);
        let mut present = 0u16;
        for (index, element) in elements.iter().enumerate() {
            e.mem.set_u32(storage + 4 * index as u32, *element);
            if *element != 0 {
                present += 1;
            }
        }
        let array = this.at(NiControllerManager::m_kSequenceArray);
        e.set(array, NiTArray::m_pBase, storage);
        e.set(array, NiTArray::m_usMaxSize, elements.len() as u16 + spare);
        e.set(array, NiTArray::m_usSize, elements.len() as u16);
        e.set(array, NiTArray::m_usESize, present);
        e.set(array, NiTArray::m_usGrowBy, 10);
        storage
    }

    /// Puts `elements` into the manager's active set.
    fn set_active(e: &mut Engine, this: Ptr<NiControllerManager>, elements: &[u32]) -> u32 {
        let storage = e.mem.alloc(4 * elements.len() as u32 + 4);
        for (index, element) in elements.iter().enumerate() {
            e.mem.set_u32(storage + 4 * index as u32, *element);
        }
        let active = this.at(NiControllerManager::m_kActiveSequences);
        e.set(active, NiTPrimitiveSet::m_pBase, storage);
        e.set(active, NiTPrimitiveSet::m_uiAlloced, elements.len() as u32);
        e.set(active, NiTPrimitiveSet::m_uiUsed, elements.len() as u32);
        storage
    }

    #[test]
    fn palette_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = setup();
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a2e1f0, &args![this, 0xfffe_u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_AV_OBJECT_PALETTE_VTABLE);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00a2e1f0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 8]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this]; 2]);
    }

    #[test]
    fn update_updates_active_sequences_and_drops_the_ones_that_went_inactive() {
        let mut e = setup();
        let this = manager(&mut e);
        e.mem.set_u8(this.addr() + 8, 0x08);
        let first = object(&mut e);
        let second = object(&mut e);
        e.mem.set_u32(first + 0x44, 1);
        e.mem.set_u32(second + 0x44, 1);
        let base = set_active(&mut e, this, &[first, second]);
        e.register_double(SEQUENCE_UPDATE, move |e, a| {
            if a[0] == first {
                e.mem.set_u32(first + 0x44, 0);
            }
            Ret::default()
        });
        let update_data = e.mem.alloc(8);
        e.mem.set_u32(update_data, 0x7f80_0001);
        e.call(0x00a2e220, &args![this, update_data]);
        assert_eq!(
            calls_to(&e, SEQUENCE_UPDATE),
            vec![vec![first, 0x7fc0_0001, 1], vec![second, 0x7fc0_0001, 1]]
        );
        let active = this.at(NiControllerManager::m_kActiveSequences);
        assert_eq!(e.get(active, NiTPrimitiveSet::m_uiUsed), 1);
        assert_eq!(e.mem.u32(base), second);
    }

    #[test]
    fn update_does_nothing_when_inactive_flag_or_empty() {
        let mut e = setup();
        e.register(SEQUENCE_UPDATE, |_, _| Ret::default());
        let this = manager(&mut e);
        let sequence = object(&mut e);
        set_active(&mut e, this, &[sequence]);
        let update_data = e.mem.alloc(8);
        e.call(0x00a2e220, &args![this, update_data]);
        e.mem.set_u8(this.addr() + 8, 0x08);
        set_active(&mut e, this, &[]);
        e.call(0x00a2e220, &args![this, update_data]);
        assert!(calls_to(&e, SEQUENCE_UPDATE).is_empty());
    }

    #[test]
    fn fn_00a2e280_switches_between_an_active_and_an_inactive_sequence() {
        let mut e = setup();
        e.register(DEACTIVATE, |_, _| Ret::default());
        e.register(SEQUENCE_ACTIVATE, |_, _| true.into_ret());
        let source = object(&mut e);
        let destination = object(&mut e);
        let sync = object(&mut e);
        let call = |e: &mut Engine| {
            e.call(
                0x00a2e280,
                &args![
                    0x1234u32,
                    source,
                    destination,
                    f32::from_bits(0x7f80_0001),
                    7u32,
                    1u32,
                    0.5f32,
                    sync
                ],
            )
            .bool()
        };
        assert!(!call(&mut e));
        e.mem.set_u32(source + 0x44, 1);
        e.mem.set_u32(destination + 0x44, 2);
        assert!(!call(&mut e));
        assert!(calls_to(&e, SEQUENCE_ACTIVATE).is_empty());
        e.mem.set_u32(destination + 0x44, 0);
        assert!(call(&mut e));
        assert_eq!(calls_to(&e, DEACTIVATE), vec![vec![source, 0x7fc0_0001, 0]]);
        assert_eq!(
            calls_to(&e, SEQUENCE_ACTIVATE),
            vec![vec![
                destination,
                7,
                1,
                0.5f32.to_bits(),
                0x7fc0_0001,
                sync,
                0
            ]]
        );
    }

    #[test]
    fn post_link_object_attaches_each_sequence_to_the_target() {
        let mut e = setup();
        e.register(OBJECT_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(SEQUENCE_ATTACH_TO_TARGET, |_, _| Ret::default());
        let this = manager(&mut e);
        e.set(this, NiControllerManager::m_pkTarget, Ptr::new(0x7700));
        set_sequences(&mut e, this, &[0x11, 0, 0x33], 0);
        e.call(0x00a2e2e0, &args![this, 0x9900u32]);
        assert_eq!(
            calls_to(&e, OBJECT_POST_LINK_OBJECT),
            vec![vec![this.addr(), 0x9900]]
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_ATTACH_TO_TARGET),
            vec![vec![0x11, 0x7700], vec![0x33, 0x7700]]
        );
    }

    /// One entry in `this`'s name map: a table of one bucket holding an item (key, value).
    fn set_one_map_item(e: &mut Engine, this: Ptr<NiControllerManager>, key: u32, value: u32) {
        let map = this.at(NiControllerManager::m_kStringMap);
        let table = e.mem.alloc(4);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 4, key);
        e.mem.set_u32(item + 8, value);
        e.mem.set_u32(table, item);
        e.set(map, NiTStringPointerMap::m_uiHashSize, 1);
        e.set(map, NiTStringPointerMap::m_ppkHashTable, Ptr::new(table));
    }

    /// Doubles for the map iteration and lookup `IsEqual` uses: the step follows the item's
    /// next pointer; the lookup answers `found` with the value `counterpart`.
    fn map_doubles(e: &mut Engine, found: bool, counterpart: u32) {
        e.register(MAP_GET_NEXT, |e, a| {
            let item = e.mem.u32(a[1]);
            let key = e.mem.u32(item + 4);
            let value = e.mem.u32(item + 8);
            let next = e.mem.u32(item);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], counterpart);
            found.into_ret()
        });
    }

    #[test]
    fn is_equal_needs_the_base_comparison_and_equal_sequence_counts() {
        let mut e = setup();
        e.register_double(TIME_CONTROLLER_IS_EQUAL, |_, _| false.into_ret());
        let this = manager(&mut e);
        let other = manager(&mut e);
        assert!(!e.call(0x00a2e320, &args![this, other]).bool());
        e.register_double(TIME_CONTROLLER_IS_EQUAL, |_, _| true.into_ret());
        set_sequences(&mut e, this, &[0x11], 0);
        assert!(!e.call(0x00a2e320, &args![this, other]).bool());
        let other_sequences = other.at(NiControllerManager::m_kSequenceArray);
        e.set(other_sequences, NiTArray::m_usSize, 1u16);
        e.set(other_sequences, NiTArray::m_pBase, 0u32);
        let sequences = this.at(NiControllerManager::m_kSequenceArray);
        e.set(sequences, NiTArray::m_usSize, 0u16);
        e.set(other_sequences, NiTArray::m_usSize, 0u16);
        assert!(e.call(0x00a2e320, &args![this, other]).bool());
    }

    #[test]
    fn is_equal_compares_each_map_entry_and_each_pair_of_sequences() {
        let mut e = setup();
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| true.into_ret());
        let answers = Rc::new(RefCell::new(vec![true, true]));
        let queue = answers.clone();
        e.register_double(IS_EQUAL, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        let first = object(&mut e);
        let second = object(&mut e);
        let this = manager(&mut e);
        let other = manager(&mut e);
        set_sequences(&mut e, this, &[first, 0], 0);
        set_sequences(&mut e, other, &[second, 0], 0);
        set_one_map_item(&mut e, this, 0x55, first);
        map_doubles(&mut e, true, second);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2e320, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, IS_EQUAL),
            vec![vec![first, second], vec![first, second]]
        );
        assert_eq!(
            calls_to(&e, MAP_GET_AT)[0][..2],
            [other.addr() + 0x50, 0x55]
        );
        assert_eq!(calls_to(&e, MAP_GET_NEXT)[0][0], this.addr() + 0x50);
    }

    #[test]
    fn is_equal_is_false_when_a_name_is_missing_or_a_sequence_differs() {
        let mut e = setup();
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| true.into_ret());
        e.register(IS_EQUAL, |_, _| false.into_ret());
        let first = object(&mut e);
        let second = object(&mut e);
        let this = manager(&mut e);
        let other = manager(&mut e);
        set_sequences(&mut e, this, &[first], 0);
        set_sequences(&mut e, other, &[second], 0);
        set_one_map_item(&mut e, this, 0x55, first);
        map_doubles(&mut e, false, 0);
        assert!(!e.call(0x00a2e320, &args![this, other]).bool());
        map_doubles(&mut e, true, second);
        assert!(!e.call(0x00a2e320, &args![this, other]).bool());
        // No map entries: the sequences at each index are compared directly.
        let plain = manager(&mut e);
        let plain_other = manager(&mut e);
        set_sequences(&mut e, plain, &[first], 0);
        set_sequences(&mut e, plain_other, &[second], 0);
        assert!(!e.call(0x00a2e320, &args![plain, plain_other]).bool());
        set_sequences(&mut e, plain_other, &[0], 0);
        assert!(!e.call(0x00a2e320, &args![plain, plain_other]).bool());
        set_sequences(&mut e, plain, &[0], 0);
        assert!(e.call(0x00a2e320, &args![plain, plain_other]).bool());
    }

    #[test]
    fn fn_00a2e420_deactivates_the_sequences_in_state_4() {
        let mut e = setup();
        e.register(DEACTIVATE, |_, _| Ret::default());
        let this = manager(&mut e);
        let sequences: Vec<u32> = (0..3).map(|_| object(&mut e)).collect();
        for (sequence, state) in sequences.iter().zip([4u32, 1, 4]) {
            e.mem.set_u32(sequence + 0x44, state);
        }
        set_active(&mut e, this, &sequences);
        assert!(!e
            .call(0x00a2e420, &args![this, f32::from_bits(0x7f80_0001)])
            .bool());
        assert_eq!(
            calls_to(&e, DEACTIVATE),
            vec![
                vec![sequences[0], 0x7fc0_0001, 0],
                vec![sequences[2], 0x7fc0_0001, 0]
            ]
        );
    }

    /// A string map object: `buckets` buckets holding the items `(bucket, key)`; the copy
    /// flag as given. Answers the map and the item addresses.
    fn string_map(
        e: &mut Engine,
        buckets: u32,
        items: &[(u32, u32)],
        copy: bool,
    ) -> (u32, Vec<u32>) {
        let map = e.mem.alloc(0x14);
        let table = e.mem.alloc(4 * buckets.max(1));
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u8(map + 0x10, u8::from(copy));
        let mut addresses = Vec::new();
        for (bucket, key) in items {
            let item = e.mem.alloc(12);
            e.mem.set_u32(item, e.mem.u32(table + 4 * bucket));
            e.mem.set_u32(item + 4, *key);
            e.mem.set_u32(table + 4 * bucket, item);
            addresses.push(item);
        }
        (map, addresses)
    }

    #[test]
    fn fn_00a2e460_frees_the_copied_keys_and_runs_remove_all_for_each_vtable() {
        let mut e = setup();
        e.register(TABLE_FREE, |_, _| Ret::default());
        let vtables = Rc::new(RefCell::new(Vec::new()));
        let seen = vtables.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        let (map, _) = string_map(&mut e, 3, &[(0, 0xa0), (0, 0xa1), (2, 0xa2)], true);
        let table = e.mem.u32(map + 8);
        e.call(0x00a2e460, &args![map]);
        let mut freed = calls_to(&e, TABLE_FREE);
        assert_eq!(freed.pop(), Some(vec![table]));
        freed.sort();
        assert_eq!(freed, vec![vec![0xa0], vec![0xa1], vec![0xa2]]);
        assert_eq!(*vtables.borrow(), vec![POINTER_MAP_VTABLE, MAP_BASE_VTABLE]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
    }

    #[test]
    fn fn_00a2e460_leaves_the_keys_alone_without_the_copy_flag_or_buckets() {
        let mut e = setup();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        let (map, _) = string_map(&mut e, 2, &[(1, 0xa0)], false);
        e.call(0x00a2e460, &args![map]);
        let (empty, _) = string_map(&mut e, 0, &[], true);
        e.call(0x00a2e460, &args![empty]);
        assert_eq!(calls_to(&e, TABLE_FREE).len(), 2);
    }

    #[test]
    fn map_scalar_deleting_destructors_run_remove_all_free_the_table_and_delete() {
        let mut e = setup();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (map, _) = string_map(&mut e, 1, &[], false);
        let table = e.mem.u32(map + 8);
        assert_eq!(e.call(0x00a2e4d0, &args![map, 0xfeu32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00a2e4d0, &args![map, 1u32]).u32(), map);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![table]; 2]);
        assert_eq!(e.call(0x00a2e510, &args![map, 1u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 4);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
    }

    #[test]
    fn string_pointer_map_scalar_deleting_destructor_runs_the_body_and_deletes() {
        let mut e = setup();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (map, _) = string_map(&mut e, 1, &[], false);
        assert_eq!(e.call(0x00a2e550, &args![map, 0u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00a2e550, &args![map, 1u32]).u32(), map);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
    }

    /// A sequence with an interpolator array of `interpolators` (first words of 0x10-byte items).
    fn sequence_with_interpolators(e: &mut Engine, state: u32, interpolators: &[u32]) -> u32 {
        let sequence = object(e);
        let array = e.mem.alloc(0x10 * interpolators.len() as u32 + 0x10);
        for (index, interpolator) in interpolators.iter().enumerate() {
            e.mem.set_u32(array + 0x10 * index as u32, *interpolator);
        }
        e.mem.set_u32(sequence + 0x14, array);
        e.mem.set_u32(sequence + 0xc, interpolators.len() as u32);
        e.mem.set_u32(sequence + 0x44, state);
        sequence
    }

    #[test]
    fn fn_00a2e570_finds_the_sequence_that_holds_an_interpolator() {
        let mut e = setup();
        let this = manager(&mut e);
        let idle = sequence_with_interpolators(&mut e, 0, &[0x70, 0x71]);
        let busy = sequence_with_interpolators(&mut e, 1, &[0, 0x80, 0x71]);
        set_sequences(&mut e, this, &[0, idle, busy], 0);
        assert_eq!(e.call(0x00a2e570, &args![this, 0x71u32, false]).u32(), idle);
        assert_eq!(e.call(0x00a2e570, &args![this, 0x71u32, true]).u32(), busy);
        assert_eq!(e.call(0x00a2e570, &args![this, 0x99u32, false]).u32(), 0);
        assert_eq!(e.call(0x00a2e570, &args![this, 0x70u32, true]).u32(), 0);
        assert_eq!(count_of(&e, idle), 1);
        assert_eq!(count_of(&e, busy), 1);
        assert!(calls_to(&e, DELETE_THIS).is_empty());
    }

    #[test]
    fn fn_00a2e570_deletes_a_sequence_whose_last_count_it_drops() {
        let mut e = setup();
        let this = manager(&mut e);
        let sequence = sequence_with_interpolators(&mut e, 1, &[0x70]);
        e.mem.set_u32(sequence + 4, 0);
        set_sequences(&mut e, this, &[sequence], 0);
        assert_eq!(e.call(0x00a2e570, &args![this, 0x99u32, false]).u32(), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![sequence]]);
    }

    #[test]
    fn fn_00a2e620_releases_and_clears_every_element() {
        let mut e = setup();
        let this = manager(&mut e);
        let kept = object(&mut e);
        e.mem.set_u32(kept + 4, 2);
        let dropped = object(&mut e);
        let storage = set_sequences(&mut e, this, &[kept, 0, dropped], 1);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        e.call(0x00a2e620, &args![array]);
        assert_eq!(count_of(&e, kept), 1);
        assert_eq!(count_of(&e, dropped), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![dropped]]);
        assert_eq!(
            [
                e.mem.u32(storage),
                e.mem.u32(storage + 4),
                e.mem.u32(storage + 8)
            ],
            [0, 0, 0]
        );
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        e.call(0x00a2e620, &args![array]);
    }

    #[test]
    fn fn_00a2e680_sets_an_element_and_keeps_the_counts() {
        let mut e = setup();
        e.register(ATEXIT, |_, _| Ret::default());
        e.map(NULL_POINTER_STATIC, 8);
        e.mem.set_u32(NULL_POINTER_STATIC, 0xffff);
        let this = manager(&mut e);
        let old = object(&mut e);
        let new = object(&mut e);
        let storage = set_sequences(&mut e, this, &[old, 0], 2);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, new);
        // Replace the old element by a non-null one at index 0: count unchanged.
        e.call_log = Some(vec![]);
        e.call(0x00a2e680, &args![array, 0u32, holder]);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![NULL_POINTER_ATEXIT]]);
        assert_eq!(e.mem.u32(NULL_POINTER_STATIC), 0);
        assert_eq!(e.mem.u8(NULL_POINTER_GUARD) & 1, 1);
        assert_eq!(e.mem.u32(storage), new);
        assert_eq!(count_of(&e, new), 2);
        assert_eq!(count_of(&e, old), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        // Fill the empty slot 1 with a non-null element: the element count goes up.
        e.call(0x00a2e680, &args![array, 1u32, holder]);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        assert_eq!(calls_to(&e, ATEXIT).len(), 1);
        // Clear slot 0 with a null element: the count goes down.
        let null_holder = e.mem.alloc(4);
        e.call(0x00a2e680, &args![array, 0u32, null_holder]);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.mem.u32(storage), 0);
        // Past the size: the size grows to index + 1 and the element is counted.
        e.call(0x00a2e680, &args![array, 3u32, holder]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 4);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        // A null element past the size is not counted.
        e.call(0x00a2e680, &args![array, 4u32, null_holder]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 5);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
    }

    #[test]
    fn fn_00a2e750_assigns_characters_to_a_fixed_string() {
        let mut e = setup();
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x9000u32);
        e.register(FIXED_STRING_FROM_CHARS, |_, a| (a[0] + 0x1000).into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x2000);
        assert_eq!(e.call(0x00a2e750, &args![slot, 0x3000u32]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x4000);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x2000 - 8]]);
        // The same characters: nothing happens.
        e.mem.set_u32(slot, 0x3000);
        e.call(0x00a2e750, &args![slot, 0x3000u32]);
        assert_eq!(e.mem.u32(slot), 0x3000);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
        // Replacing the empty string releases nothing.
        e.mem.set_u32(slot, 0x9000);
        e.call(0x00a2e750, &args![slot, 0x5000u32]);
        assert_eq!(e.mem.u32(slot), 0x6000);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fixed_string_map_scalar_deleting_destructors_run_remove_all_and_delete() {
        let mut e = setup();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        // RemoveAll is the translation in this file: an empty map with no buckets.
        let (map, _) = string_map(&mut e, 0, &[], false);
        let table = e.mem.u32(map + 8);
        assert_eq!(e.call(0x00a2e790, &args![map, 1u32]).u32(), map);
        assert_eq!(e.mem.u32(map), FIXED_STRING_MAP_BASE_VTABLE);
        assert_eq!(e.call(0x00a2eb40, &args![map, 0u32]).u32(), map);
        assert_eq!(e.mem.u32(map), FIXED_STRING_MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![table]; 2]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
    }

    #[test]
    fn fn_00a2e7d0_takes_a_sequence_out_of_the_manager() {
        let mut e = setup();
        e.register(SEQUENCE_DETACH, |_, _| Ret::default());
        e.register(STRING_MAP_REMOVE_AT, |_, _| Ret::default());
        e.register(ARRAY_REMOVE_AT, |e, a| {
            let storage = e.mem.u32(a[0] + 4);
            let element = e.mem.u32(storage + 4 * a[2]);
            e.mem.set_u32(storage + 4 * a[2], 0);
            e.mem.set_u32(a[1], element);
            e.mem.set_u32(element + 4, e.mem.u32(element + 4) + 1);
            Ret::default()
        });
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        let this = manager(&mut e);
        let first = object(&mut e);
        let second = object(&mut e);
        set_sequences(&mut e, this, &[first, second], 0);
        set_active(&mut e, this, &[first, second]);
        e.put_vtable(0x0300_2000, &[0, 0x0300_2004]);
        e.register(0x0300_2004, |_, _| Ret::default());
        e.mem.set_u32(this.addr() + 0x50, 0x0300_2000);
        set_one_map_item(&mut e, this, 0x55, second);
        e.mem.set_u32(second + 0x40, this.addr());
        let out = e.mem.alloc(4);
        e.call(0x00a2e7d0, &args![this, second, out]);
        assert_eq!(e.mem.u32(out), second);
        assert_eq!(e.mem.u32(second + 0x40), 0);
        assert_eq!(calls_to(&e, SEQUENCE_DETACH), vec![vec![second]]);
        assert_eq!(calls_to(&e, STRING_MAP_REMOVE_AT).len(), 1);
        assert_eq!(
            calls_to(&e, STRING_MAP_REMOVE_AT)[0],
            vec![this.addr() + 0x50, 0x55]
        );
        assert_eq!(
            calls_to(&e, ARRAY_REMOVE_AT)[0][1..],
            [calls_to(&e, ARRAY_REMOVE_AT)[0][1], 1]
        );
        let active = this.at(NiControllerManager::m_kActiveSequences);
        assert_eq!(e.get(active, NiTPrimitiveSet::m_uiUsed), 1);
        // Counts: the array's reference was handed to the temporary and released; `out` holds one.
        assert_eq!(count_of(&e, second), 2);
        assert_eq!(count_of(&e, first), 1);
    }

    #[test]
    fn fn_00a2e7d0_releases_the_out_pointer_when_the_sequence_is_not_there() {
        let mut e = setup();
        let this = manager(&mut e);
        let first = object(&mut e);
        set_sequences(&mut e, this, &[first], 0);
        let held = object(&mut e);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, held);
        e.call(0x00a2e7d0, &args![this, 0x1234u32, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(count_of(&e, held), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![held]]);
        assert_eq!(count_of(&e, first), 1);
        e.call(0x00a2e7d0, &args![this, 0x1234u32, out]);
        assert_eq!(e.mem.u32(out), 0);
    }

    /// Counting doubles that do not touch memory (for tests with fixed-string handles, which
    /// are not addresses in the test heap): the increment answers 2 and the decrement 1, so
    /// nothing is ever deleted.
    fn silent_interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |_, _| 2u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
    }

    /// The globals for fixed strings: the empty string's handle word (0x9000) and the static
    /// `NiFixedString` at `011f49e0` (handle 0x7800).
    fn map_fixed_strings(e: &mut Engine) {
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x9000u32);
        e.map(FIXED_STRING_011F49E0, 4);
        e.set_global(FIXED_STRING_011F49E0, 0x7800u32);
    }

    /// A stream whose input and output streams are 0x10-byte objects with the given read and
    /// write function addresses and version.
    fn stream_with(
        e: &mut Engine,
        read: u32,
        write: u32,
        version: u32,
    ) -> (Ptr<NiStream>, u32, u32) {
        let stream = e.new_object::<NiStream>();
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, read);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, write);
        e.set(stream, NiStream::m_pkIstr, input);
        e.set(stream, NiStream::m_pkOstr, output);
        e.set(stream, NiStream::m_uiNifFileVersion, version);
        (stream, input, output)
    }

    const STREAM_VTABLE: u32 = 0x0300_2000;
    const SAVE_LINK_ID: u32 = 0x0300_2030;

    #[test]
    fn fn_00a2e9a0_detaches_and_clears_every_sequence() {
        let mut e = setup();
        e.register(SEQUENCE_DETACH, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        let this = manager(&mut e);
        let first = object(&mut e);
        let second = object(&mut e);
        e.mem.set_u32(first + 0x40, this.addr());
        e.mem.set_u32(second + 0x40, this.addr());
        let storage = set_sequences(&mut e, this, &[first, 0, second], 0);
        e.call(0x00a2e9a0, &args![this]);
        assert_eq!(
            calls_to(&e, SEQUENCE_DETACH),
            vec![vec![first], vec![second]]
        );
        assert_eq!(e.mem.u32(first + 0x40), 0);
        assert_eq!(e.mem.u32(second + 0x40), 0);
        assert_eq!(e.mem.u32(storage), 0);
        assert_eq!(count_of(&e, first), 0);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![this.addr() + 0x50]]);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
    }

    #[test]
    fn save_binary_writes_the_flag_the_count_and_the_links() {
        const WRITE: u32 = 0x0300_3000;
        let mut e = setup();
        e.put_vtable(STREAM_VTABLE, &[0; 13]);
        e.mem.set_u32(STREAM_VTABLE + 0x30, SAVE_LINK_ID);
        e.register(SAVE_LINK_ID, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        let written = Rc::new(RefCell::new(Vec::new()));
        let seen = written.clone();
        e.register_double(WRITE, move |e, a| {
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            seen.borrow_mut().push((value, a[2], e.mem.u32(a[3]), a[4]));
            a[2].into_ret()
        });
        let (stream, _, output) = stream_with(&mut e, 0, WRITE, 0);
        e.mem.set_u32(stream.addr(), STREAM_VTABLE);
        e.mem.set_u32(output + 4, 100);
        let this = manager(&mut e);
        e.set(this, NiControllerManager::m_bCumulative, 1);
        e.set(
            this,
            NiControllerManager::m_spObjectPalette,
            Ptr::new(0x5151),
        );
        set_sequences(&mut e, this, &[0x11, 0, 0x33], 0);
        e.call(0x00a2e9f0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SAVE_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
        assert_eq!(*written.borrow(), vec![(1, 1, 1, 1), (3, 4, 4, 1)]);
        assert_eq!(e.mem.u32(output + 4), 105);
        assert_eq!(
            calls_to(&e, SAVE_LINK_ID),
            vec![
                vec![stream.addr(), 0x11],
                vec![stream.addr(), 0x33],
                vec![stream.addr(), 0x5151]
            ]
        );
    }

    #[test]
    fn load_binary_reads_the_flag_and_replaces_the_palette_from_new_versions() {
        const READ: u32 = 0x0300_3000;
        let mut e = setup();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        let new_palette = object(&mut e);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| new_palette.into_ret());
        e.register(READ, |e, a| {
            e.mem.set_u8(a[1], 7);
            1u32.into_ret()
        });
        let old_palette = object(&mut e);
        let this = manager(&mut e);
        e.set(
            this,
            NiControllerManager::m_spObjectPalette,
            Ptr::new(old_palette),
        );
        let (stream, input, _) = stream_with(&mut e, READ, 0, 0x0a01_0067);
        e.mem.set_u32(input + 4, 10);
        e.call(0x00a2eaa0, &args![this, stream]);
        assert_eq!(e.get(this, NiControllerManager::m_bCumulative), 1);
        assert_eq!(e.mem.u32(input + 4), 11);
        assert_eq!(
            calls_to(&e, STREAM_READ_MULTIPLE_LINK_IDS),
            vec![vec![stream.addr()]]
        );
        assert!(calls_to(&e, STREAM_RESOLVE_LINK_ID).is_empty());
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_0068);
        e.call(0x00a2eaa0, &args![this, stream]);
        assert_eq!(
            e.get(this, NiControllerManager::m_spObjectPalette).addr(),
            new_palette
        );
        assert_eq!(count_of(&e, new_palette), 2);
        assert_eq!(count_of(&e, old_palette), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old_palette]]);
    }

    #[test]
    fn fn_00a2eb80_stores_the_array_vtable_and_frees_the_elements() {
        let mut e = setup();
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = manager(&mut e);
        let storage = set_sequences(&mut e, this, &[0x11], 0);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.call(0x00a2eb80, &args![array, 1u32]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![storage]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array.addr()]]
        );
        e.call(0x00a2eb80, &args![array, 0u32]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn destructor_takes_the_manager_apart() {
        let mut e = setup();
        e.register(LISTENER_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        let this = manager(&mut e);
        let sequence = object(&mut e);
        let storage = set_sequences(&mut e, this, &[sequence], 0);
        let active = set_active(&mut e, this, &[sequence]);
        let palette = object(&mut e);
        let listener = e.mem.alloc(0x10);
        e.set(
            this,
            NiControllerManager::m_spObjectPalette,
            Ptr::new(palette),
        );
        e.set(this, NiControllerManager::pListener, Ptr::new(listener));
        e.mem.set_u32(this.addr() + 0x6c, 0x6100);
        e.mem.set_u32(this.addr() + 0x74, 5);
        let map_table = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 0x58, map_table);
        e.call_log = Some(vec![]);
        e.call(0x00a2ebb0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_CONTROLLER_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x34), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x74), 0);
        assert_eq!(calls_to(&e, LISTENER_DESTRUCT), vec![vec![listener]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, listener]]
        );
        assert_eq!(count_of(&e, palette), 0);
        assert_eq!(count_of(&e, sequence), 0);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![0x6100], vec![storage]]);
        assert_eq!(
            calls_to(&e, TABLE_FREE),
            vec![vec![map_table], vec![active]]
        );
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_DESTRUCT),
            vec![vec![this.addr()]]
        );
        assert_eq!(e.mem.u32(storage), 0);
    }

    #[test]
    fn destructor_without_listener_or_palette_skips_them() {
        let mut e = setup();
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        let this = manager(&mut e);
        e.mem.set_u32(this.addr() + 0x58, 0);
        e.call(0x00a2ebb0, &args![this]);
        assert_eq!(calls_to(&e, ARRAY_FREE).len(), 2);
    }

    #[test]
    fn get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2ec40, &args![]).u32(),
            NI_CONTROLLER_MANAGER_RTTI
        );
    }

    #[test]
    fn remove_sequence_releases_what_the_removal_hands_back() {
        let mut e = setup();
        e.register(SEQUENCE_DETACH, |_, _| Ret::default());
        e.register(STRING_MAP_REMOVE_AT, |_, _| Ret::default());
        e.register(ARRAY_REMOVE_AT, |e, a| {
            let storage = e.mem.u32(a[0] + 4);
            let element = e.mem.u32(storage + 4 * a[2]);
            e.mem.set_u32(storage + 4 * a[2], 0);
            e.mem.set_u32(a[1], element);
            e.mem.set_u32(element + 4, e.mem.u32(element + 4) + 1);
            Ret::default()
        });
        let this = manager(&mut e);
        let sequence = object(&mut e);
        set_sequences(&mut e, this, &[sequence], 0);
        e.call(0x00a2ec50, &args![this, sequence]);
        assert_eq!(count_of(&e, sequence), 1);
        assert_eq!(calls_to(&e, SEQUENCE_DETACH), vec![vec![sequence]]);
        // Not in the manager: nothing is held and nothing is released.
        e.call(0x00a2ec50, &args![this, 0x1234u32]);
        assert_eq!(count_of(&e, sequence), 1);
    }

    /// Doubles for the array storage `SetSize` makes and frees.
    fn array_storage_doubles(e: &mut Engine) {
        e.register(ARRAY_ALLOCATE, |e, a| {
            (e.mem.alloc(4 * a[0] + 8) + 4).into_ret()
        });
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.map(NULL_POINTER_STATIC, 8);
    }

    #[test]
    fn add_first_empty_refuses_a_null_element() {
        let mut e = setup();
        let this = manager(&mut e);
        set_sequences(&mut e, this, &[0x11], 1);
        let holder = e.mem.alloc(4);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.call(0x00a2ec90, &args![array, holder]).u32(), 0xffff_ffff);
    }

    #[test]
    fn add_first_empty_uses_the_lowest_slot_of_the_top_most_run_of_empty_slots() {
        let mut e = setup();
        let this = manager(&mut e);
        let [a, b, c] = [object(&mut e), object(&mut e), object(&mut e)];
        let storage = set_sequences(&mut e, this, &[a, 0, 0, b, c], 0);
        let element = object(&mut e);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, element);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.call(0x00a2ec90, &args![array, holder]).u32(), 1);
        assert_eq!(e.mem.u32(storage + 4), element);
        assert_eq!(count_of(&e, element), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 4);
        // Another with an empty slot at the top: that one is used.
        let other = manager(&mut e);
        let storage = set_sequences(&mut e, other, &[a, 0, b, 0], 0);
        let array = other.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.call(0x00a2ec90, &args![array, holder]).u32(), 3);
        assert_eq!(e.mem.u32(storage + 12), element);
    }

    #[test]
    fn add_first_empty_appends_when_there_are_no_empty_slots() {
        let mut e = setup();
        array_storage_doubles(&mut e);
        let this = manager(&mut e);
        let a = object(&mut e);
        let old_storage = set_sequences(&mut e, this, &[a], 0);
        let element = object(&mut e);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, element);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.call(0x00a2ec90, &args![array, holder]).u32(), 1);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 11);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        let storage = e.get(array, NiTArray::m_pBase);
        assert_ne!(storage, old_storage);
        assert_eq!(e.mem.u32(storage), a);
        assert_eq!(e.mem.u32(storage + 4), element);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![old_storage]]);
    }

    #[test]
    fn add_first_empty_appends_without_growing_while_there_is_capacity() {
        let mut e = setup();
        array_storage_doubles(&mut e);
        let this = manager(&mut e);
        set_sequences(&mut e, this, &[], 3);
        let element = object(&mut e);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, element);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.call(0x00a2ec90, &args![array, holder]).u32(), 0);
        assert!(calls_to(&e, ARRAY_ALLOCATE).is_empty());
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
    }

    #[test]
    fn set_size_grows_copies_and_frees_the_old_storage() {
        let mut e = setup();
        array_storage_doubles(&mut e);
        let this = manager(&mut e);
        let a = object(&mut e);
        let b = object(&mut e);
        let old = set_sequences(&mut e, this, &[a, 0, b], 0);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        e.call(0x00a47770, &args![array, 6u32]);
        let storage = e.get(array, NiTArray::m_pBase);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![6]]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 6);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
        assert_eq!(
            [
                e.mem.u32(storage),
                e.mem.u32(storage + 4),
                e.mem.u32(storage + 8)
            ],
            [a, 0, b]
        );
        assert_eq!(count_of(&e, a), 2);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![old]]);
        // The same size again: nothing.
        e.call(0x00a47770, &args![array, 6u32]);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE).len(), 1);
    }

    #[test]
    fn set_size_shrinks_releasing_the_elements_that_no_longer_fit() {
        let mut e = setup();
        array_storage_doubles(&mut e);
        let this = manager(&mut e);
        let a = object(&mut e);
        let b = object(&mut e);
        let c = object(&mut e);
        set_sequences(&mut e, this, &[a, b, c], 0);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        e.call(0x00a47770, &args![array, 1u32]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(count_of(&e, b), 0);
        assert_eq!(count_of(&e, c), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![b], vec![c]]);
        let storage = e.get(array, NiTArray::m_pBase);
        assert_eq!(e.mem.u32(storage), a);
        e.call(0x00a47770, &args![array, 0u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(count_of(&e, a), 1);
    }

    #[test]
    fn palette_constructor_builds_the_map_and_fills_it() {
        let mut e = setup();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(PALETTE_FILL, |_, _| Ret::default());
        let this = e.new_object::<NiDefaultAVObjectPalette>();
        e.mem.set_u32(this.addr() + 0x14, 0xdead);
        assert_eq!(
            e.call(0x00a2ed80, &args![this, 0x6600u32, 0x4fu32]).u32(),
            this.addr()
        );
        let table = e.mem.u32(this.addr() + 0x10);
        assert_eq!(e.mem.u32(this.addr()), NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 8), FIXED_STRING_MAP_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0x4f);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x6600);
        assert_eq!(calls_to(&e, TABLE_ALLOCATE), vec![vec![0x4f * 4]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 0x4f * 4]]);
        assert_eq!(calls_to(&e, PALETTE_FILL), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn palette_set_scene_stores_the_scene() {
        let mut e = Engine::new();
        let this = e.new_object::<NiDefaultAVObjectPalette>();
        e.call(0x00a2edf0, &args![this, 0x6600u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x6600);
    }

    /// A palette whose map has `buckets` buckets and the items `(handle, object)`.
    fn palette_with(
        e: &mut Engine,
        buckets: u32,
        items: &[(u32, u32)],
    ) -> Ptr<NiDefaultAVObjectPalette> {
        let this = e.new_object::<NiDefaultAVObjectPalette>();
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(this.addr() + 0xc, buckets);
        e.mem.set_u32(this.addr() + 0x10, table);
        for (handle, object) in items {
            let item = e.mem.alloc(12);
            let bucket_slot = table + 4 * (handle % buckets);
            e.mem.set_u32(item, e.mem.u32(bucket_slot));
            e.mem.set_u32(item + 4, *handle);
            e.mem.set_u32(item + 8, *object);
            e.mem.set_u32(bucket_slot, item);
        }
        this
    }

    #[test]
    fn palette_get_av_object_finds_an_object_by_its_name_handle() {
        let mut e = Engine::new();
        let this = palette_with(&mut e, 4, &[(0x1004, 0xa), (0x1008, 0xb), (0x1005, 0xc)]);
        let name = e.mem.alloc(4);
        let lookup = |e: &mut Engine, handle: u32| {
            e.mem.set_u32(name, handle);
            e.call(0x00a2ee00, &args![this, name]).u32()
        };
        assert_eq!(lookup(&mut e, 0x1004), 0xa);
        assert_eq!(lookup(&mut e, 0x1008), 0xb);
        assert_eq!(lookup(&mut e, 0x1005), 0xc);
        assert_eq!(lookup(&mut e, 0x100c), 0);
        assert_eq!(lookup(&mut e, 0), 0);
        assert_eq!(lookup(&mut e, 0x1006), 0);
    }

    #[test]
    #[should_panic(expected = "divisor of zero")]
    fn palette_get_av_object_divides_by_the_bucket_count_as_the_exe_does() {
        let mut e = Engine::new();
        let this = palette_with(&mut e, 1, &[]);
        e.mem.set_u32(this.addr() + 0xc, 0);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x1000);
        e.call(0x00a2ee00, &args![this, name]);
    }

    #[test]
    fn palette_set_av_object_removes_for_a_null_object_and_sets_otherwise() {
        let mut e = setup();
        e.register(PALETTE_MAP_REMOVE_AT, |_, _| Ret::default());
        e.register(PALETTE_ITEM_SET, |_, _| Ret::default());
        let this = palette_with(&mut e, 4, &[]);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 0x1001);
        e.call(0x00a2ee40, &args![this, key, 0u32]);
        assert_eq!(
            calls_to(&e, PALETTE_MAP_REMOVE_AT),
            vec![vec![this.addr() + 8, key]]
        );
        assert!(calls_to(&e, PALETTE_ITEM_SET).is_empty());
    }

    #[test]
    fn palette_set_av_object_without_removal_sets_the_item() {
        const NEW_ITEM: u32 = 0x0300_4004;
        let mut e = setup();
        e.register(PALETTE_ITEM_SET, |_, _| Ret::default());
        e.register(NEW_ITEM, |e, _| e.mem.alloc(12).into_ret());
        let this = palette_with(&mut e, 4, &[]);
        e.put_vtable(0x0300_4000, &[0, NEW_ITEM]);
        e.mem.set_u32(this.addr() + 8, 0x0300_4000);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 0x1001);
        e.call(0x00a2ee40, &args![this, key, 0x77u32]);
        let sets = calls_to(&e, PALETTE_ITEM_SET);
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0][0], this.addr() + 8);
        assert_eq!(sets[0][2..], [key, 0x77]);
        let map = this.at(NiDefaultAVObjectPalette::m_kMap);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 1);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        assert_eq!(e.mem.u32(table + 4), sets[0][1]);
    }

    #[test]
    fn palette_scalar_deleting_destructor_clears_the_map_and_the_object() {
        let mut e = setup();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = palette_with(&mut e, 0, &[]);
        let table = e.mem.u32(this.addr() + 0x10);
        assert_eq!(e.call(0x00a2ee70, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_AV_OBJECT_PALETTE_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 8), FIXED_STRING_MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![table]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x1c]]);
        e.call(0x00a2ee70, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn fixed_string_map_remove_all_deletes_every_item_of_every_chain() {
        const DELETE_ITEM: u32 = 0x0300_5008;
        let mut e = setup();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(DELETE_ITEM, move |e, a| {
            log.borrow_mut().push((a[1], e.mem.u32(a[1] + 8)));
            Ret::default()
        });
        let this = palette_with(&mut e, 3, &[(0x1000, 0xa), (0x1003, 0xb), (0x1002, 0xc)]);
        e.put_vtable(0x0300_5000, &[0, 0, DELETE_ITEM]);
        let map = this.at(NiDefaultAVObjectPalette::m_kMap);
        e.mem.set_u32(map.addr(), 0x0300_5000);
        e.set(map, NiTPointerMap::m_uiCount, 3);
        e.call(0x00aa01f0, &args![map]);
        let deleted: Vec<u32> = seen.borrow().iter().map(|d| d.1).collect();
        assert_eq!(deleted, vec![0xc, 0xb, 0xa]);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        assert_eq!(
            [e.mem.u32(table), e.mem.u32(table + 4), e.mem.u32(table + 8)],
            [0, 0, 0]
        );
        let empty = palette_with(&mut e, 0, &[]);
        let empty_map = empty.at(NiDefaultAVObjectPalette::m_kMap);
        e.set(empty_map, NiTPointerMap::m_uiCount, 5);
        e.call(0x00aa01f0, &args![empty_map]);
        assert_eq!(e.get(empty_map, NiTPointerMap::m_uiCount), 0);
    }

    #[test]
    fn fixed_string_map_delete_item_replaces_the_key_and_frees_the_item() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        e.register(FIXED_STRING_FROM_CHARS, |_, _| 0x9000u32.into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 4, 0x4000);
        e.mem.set_u32(item + 8, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00aa0330, &args![0u32, item]);
        assert_eq!(e.mem.u32(item + 4), 0x9000);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(calls_to(&e, FIXED_STRING_FROM_CHARS), vec![vec![0]]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x4000 - 8]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, item]]
        );
        // A null key is left alone but the (null) key is still checked against the empty string.
        let bare = e.mem.alloc(12);
        e.call(0x00aa0330, &args![0u32, bare]);
        assert_eq!(calls_to(&e, FIXED_STRING_FROM_CHARS).len(), 1);
    }

    #[test]
    fn fixed_string_map_set_at_updates_or_adds_an_item() {
        const NEW_ITEM: u32 = 0x0300_4004;
        let mut e = setup();
        e.register(PALETTE_ITEM_SET, |_, _| Ret::default());
        e.register(NEW_ITEM, |e, _| e.mem.alloc(12).into_ret());
        let this = palette_with(&mut e, 4, &[(0x1001, 0xa)]);
        e.put_vtable(0x0300_4000, &[0, NEW_ITEM]);
        let map = this.at(NiDefaultAVObjectPalette::m_kMap);
        e.mem.set_u32(map.addr(), 0x0300_4000);
        e.set(map, NiTPointerMap::m_uiCount, 1);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 0x1001);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let existing = e.mem.u32(table + 4);
        e.call(0x00a6e980, &args![map, key, 0x55u32]);
        assert_eq!(
            calls_to(&e, PALETTE_ITEM_SET),
            vec![vec![map.addr(), existing, key, 0x55]]
        );
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 1);
        // A new handle in the same bucket goes to the front of the chain.
        e.mem.set_u32(key, 0x1005);
        e.call(0x00a6e980, &args![map, key, 0x66u32]);
        let created = e.mem.u32(table + 4);
        assert_ne!(created, existing);
        assert_eq!(e.mem.u32(created), existing);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 2);
        assert_eq!(
            calls_to(&e, PALETTE_ITEM_SET)[1],
            vec![map.addr(), created, key, 0x66]
        );
    }

    /// Doubles for the allocations and calls both constructors make.
    fn constructor_doubles(e: &mut Engine) {
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(LISTENER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(PALETTE_FILL, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_SET_TARGET, |_, _| Ret::default());
    }

    /// A manager object filled with junk so that a constructor's stores show.
    fn junk_manager(e: &mut Engine) -> Ptr<NiControllerManager> {
        let this = manager(e);
        for offset in (4..0x7c).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdead_0000 + offset);
        }
        this
    }

    #[test]
    fn default_constructor_sets_up_the_empty_manager_and_its_listener() {
        let mut e = setup();
        constructor_doubles(&mut e);
        let this = junk_manager(&mut e);
        assert_eq!(e.call(0x00a2eed0, &args![this]).u32(), this.addr());
        let a = this.addr();
        assert_eq!(e.mem.u32(a), NI_CONTROLLER_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(a + 0x34), OBJECT_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(a + 0x38), 0);
        assert_eq!(
            [
                e.mem.u16(a + 0x3c),
                e.mem.u16(a + 0x3e),
                e.mem.u16(a + 0x40),
                e.mem.u16(a + 0x42)
            ],
            [0, 0, 0, 10]
        );
        assert_eq!(
            [
                e.mem.u32(a + 0x44),
                e.mem.u32(a + 0x48),
                e.mem.u32(a + 0x4c)
            ],
            [0, 0, 0]
        );
        assert_eq!(e.mem.u32(a + 0x50), STRING_POINTER_MAP_VTABLE);
        assert_eq!(e.mem.u32(a + 0x54), 0x25);
        assert_eq!(e.mem.u32(a + 0x5c), 0);
        assert_eq!(e.mem.u8(a + 0x60), 0);
        assert_eq!(e.mem.u8(a + 0x68), 0);
        assert_eq!(
            [
                e.mem.u32(a + 0x6c),
                e.mem.u32(a + 0x70),
                e.mem.u32(a + 0x74),
                e.mem.u32(a + 0x78)
            ],
            [0; 4]
        );
        let table = e.mem.u32(a + 0x58);
        assert_eq!(calls_to(&e, TABLE_ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 0x94]]);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT), vec![vec![a]]);
        let listener = e.mem.u32(a + 0x64);
        assert_ne!(listener, 0);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x10]]
        );
        assert_eq!(calls_to(&e, LISTENER_CONSTRUCT), vec![vec![listener]]);
    }

    #[test]
    fn default_constructor_stores_null_when_there_is_no_memory_for_the_listener() {
        let mut e = setup();
        constructor_doubles(&mut e);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let this = junk_manager(&mut e);
        e.call(0x00a2eed0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0);
        assert!(calls_to(&e, LISTENER_CONSTRUCT).is_empty());
    }

    #[test]
    fn target_constructor_sets_the_flag_the_target_and_a_palette() {
        let mut e = setup();
        constructor_doubles(&mut e);
        let this = junk_manager(&mut e);
        assert_eq!(
            e.call(0x00a2efa0, &args![this, 0x6600u32, 3u8]).u32(),
            this.addr()
        );
        let a = this.addr();
        assert_eq!(e.mem.u8(a + 0x68), 3);
        assert_eq!(e.mem.u32(a + 0x50), STRING_POINTER_MAP_VTABLE);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SET_TARGET),
            vec![vec![a, 0x6600]]
        );
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x1c]]);
        let palette = e.mem.u32(a + 0x78);
        assert_ne!(palette, 0);
        assert_eq!(e.mem.u32(palette), NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE);
        assert_eq!(e.mem.u32(palette + 0xc), 0x4f);
        assert_eq!(e.mem.u32(palette + 0x18), 0x6600);
        assert_eq!(count_of(&e, palette), 1);
        assert_ne!(e.mem.u32(a + 0x64), 0);
    }

    #[test]
    fn target_constructor_without_memory_for_the_palette_leaves_it_null() {
        let mut e = setup();
        constructor_doubles(&mut e);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let this = junk_manager(&mut e);
        e.call(0x00a2efa0, &args![this, 0x6600u32, 0u8]);
        assert_eq!(e.mem.u32(this.addr() + 0x78), 0);
    }

    /// A manager with a target (name handle 0x7700 at `+8`), an empty sequence array with room
    /// for `spare` more and the doubles `AddSequence` and its callers need.
    fn adding_manager(e: &mut Engine, spare: u16) -> (Ptr<NiControllerManager>, u32) {
        map_fixed_strings(e);
        silent_interlocked(e);
        array_storage_doubles(e);
        e.register(SEQUENCE_SET_NAME_FROM_TARGET, |_, _| Ret::default());
        e.register(SEQUENCE_SET_NAME, |_, _| Ret::default());
        e.register(SEQUENCE_ATTACH_TO_TARGET, |_, _| true.into_ret());
        e.register(STRING_MAP_SET_AT, |_, _| Ret::default());
        let this = manager(e);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(target + 8, 0x7700);
        e.set(this, NiControllerManager::m_pkTarget, Ptr::new(target));
        set_sequences(e, this, &[], spare);
        (this, target)
    }

    #[test]
    fn fn_00a2f0c0_adds_a_new_sequence_under_its_name() {
        let mut e = setup();
        let (this, target) = adding_manager(&mut e, 4);
        let sequence = object(&mut e);
        e.mem.set_u32(sequence + 8, 0x7a00);
        e.mem.set_u32(sequence + 4, 5);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x7b00);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a2f0c0, &args![this, sequence, name, false])
            .bool());
        assert_eq!(e.mem.u32(sequence + 0x40), this.addr());
        assert_eq!(
            calls_to(&e, SEQUENCE_SET_NAME_FROM_TARGET),
            vec![vec![sequence, target + 8]]
        );
        assert!(calls_to(&e, SEQUENCE_ATTACH_TO_TARGET).is_empty());
        assert_eq!(calls_to(&e, SEQUENCE_SET_NAME), vec![vec![sequence, name]]);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        let storage = e.get(array, NiTArray::m_pBase);
        assert_eq!(e.mem.u32(storage), sequence);
        assert_eq!(
            calls_to(&e, STRING_MAP_SET_AT),
            vec![vec![this.addr() + 0x50, 0x7a00, sequence]]
        );
    }

    #[test]
    fn fn_00a2f0c0_refuses_a_sequence_with_an_owner_and_undoes_a_failed_attach() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 4);
        let owned = object(&mut e);
        e.mem.set_u32(owned + 0x40, 0x1234);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x9000);
        assert!(!e.call(0x00a2f0c0, &args![this, owned, name, false]).bool());
        assert_eq!(e.mem.u32(owned + 0x40), 0x1234);
        assert!(calls_to(&e, STRING_MAP_SET_AT).is_empty());
        let fresh = object(&mut e);
        e.mem.set_u32(fresh + 0x5c, 0x7c00);
        e.register(SEQUENCE_ATTACH_TO_TARGET, |_, _| false.into_ret());
        assert!(!e.call(0x00a2f0c0, &args![this, fresh, name, true]).bool());
        assert_eq!(e.mem.u32(fresh + 0x40), 0);
        assert!(calls_to(&e, SEQUENCE_SET_NAME_FROM_TARGET).is_empty());
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
    }

    #[test]
    fn fn_00a2f0c0_skips_the_name_when_it_is_the_empty_string() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 4);
        let sequence = object(&mut e);
        e.mem.set_u32(sequence + 0x5c, 0x7c00);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x9000);
        assert!(e
            .call(0x00a2f0c0, &args![this, sequence, name, true])
            .bool());
        assert!(calls_to(&e, SEQUENCE_SET_NAME).is_empty());
        assert!(calls_to(&e, SEQUENCE_SET_NAME_FROM_TARGET).is_empty());
        assert_eq!(calls_to(&e, SEQUENCE_ATTACH_TO_TARGET).len(), 1);
    }

    #[test]
    fn fn_00a2f170_makes_a_temporary_blend_sequence_when_none_is_free() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 4);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FIXED_STRING_FROM_CHARS, |_, _| 0x8800u32.into_ret());
        let constructed = Rc::new(RefCell::new(Vec::new()));
        let log = constructed.clone();
        e.register_double(SEQUENCE_CONSTRUCT, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1]), a[2], a[3]));
            a[0].into_ret()
        });
        let added = Rc::new(RefCell::new(Vec::new()));
        let log = added.clone();
        e.register_double(SET_ADD, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.register(SEQUENCE_COPY_BLEND_POSE, |_, _| Ret::default());
        let source = object(&mut e);
        e.mem.set_u32(source + 0xc, 7);
        e.call_log = Some(vec![]);
        let temp = e.call(0x00a2f170, &args![this, source, 0x55u32]).u32();
        assert_ne!(temp, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x74]]);
        assert_eq!(
            calls_to(&e, FIXED_STRING_FROM_CHARS),
            vec![vec![TEXT_TEMP_BLEND_SEQUENCE]]
        );
        assert_eq!(*constructed.borrow(), vec![(temp, 0x8800, 7, 0xc)]);
        assert_eq!(*added.borrow(), vec![(this.addr() + 0x6c, temp)]);
        assert_eq!(e.mem.u32(temp + 0x40), this.addr());
        assert_eq!(e.mem.u32(temp + 0x5c), 0x7700);
        assert_eq!(e.mem.u32(temp + 8), 0x7800);
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.mem.u32(e.get(array, NiTArray::m_pBase)), temp);
        assert_eq!(
            calls_to(&e, STRING_MAP_SET_AT),
            vec![vec![this.addr() + 0x50, 0x7800, temp]]
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_COPY_BLEND_POSE),
            vec![vec![source, temp, 7, 0x55, 0]]
        );
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT)[0], vec![0x8800 - 8]);
    }

    #[test]
    fn fn_00a2f170_reuses_an_inactive_temporary_blend_sequence() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 4);
        e.register(SEQUENCE_DETACH, |_, _| Ret::default());
        e.register(SEQUENCE_COPY_BLEND_POSE, |_, _| Ret::default());
        let busy = object(&mut e);
        e.mem.set_u32(busy + 0x44, 1);
        let idle = object(&mut e);
        let pool = e.mem.alloc(8);
        e.mem.set_u32(pool, busy);
        e.mem.set_u32(pool + 4, idle);
        let set = this.at(NiControllerManager::m_kTempBlendSeqs);
        e.set(set, NiTPrimitiveSet::m_pBase, pool);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, 2);
        e.set(set, NiTPrimitiveSet::m_uiUsed, 2);
        let source = object(&mut e);
        e.mem.set_u32(source + 0xc, 3);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a2f170, &args![this, source, 0x66u32]).u32(),
            idle
        );
        assert_eq!(calls_to(&e, SEQUENCE_DETACH), vec![vec![idle]]);
        assert_eq!(e.mem.u32(idle + 0x40), this.addr());
        assert_eq!(e.mem.u32(idle + 0x5c), 0x7700);
        assert_eq!(
            calls_to(&e, SEQUENCE_COPY_BLEND_POSE),
            vec![vec![source, idle, 3, 0x66, 1]]
        );
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn fn_00a2f380_copies_the_flag_the_sequences_and_the_palette() {
        let mut e = setup();
        let (destination, _) = adding_manager(&mut e, 4);
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let clone = object(&mut e);
        e.register_double(CREATE_CLONE, move |_, _| clone.into_ret());
        let this = manager(&mut e);
        e.set(this, NiControllerManager::m_bCumulative, 1);
        let sequence = object(&mut e);
        set_sequences(&mut e, this, &[sequence, 0], 0);
        let palette = object(&mut e);
        e.set(
            this,
            NiControllerManager::m_spObjectPalette,
            Ptr::new(palette),
        );
        e.call_log = Some(vec![]);
        e.call(0x00a2f380, &args![this, destination, 0x7777u32]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), 0x7777]]
        );
        assert_eq!(e.get(destination, NiControllerManager::m_bCumulative), 1);
        assert_eq!(
            calls_to(&e, CREATE_CLONE),
            vec![vec![sequence, 0x7777], vec![palette, 0x7777]]
        );
        assert_eq!(e.mem.u32(clone + 0x40), destination.addr());
        assert_eq!(
            calls_to(&e, SEQUENCE_SET_NAME),
            vec![vec![clone, FIXED_STRING_011F49E0]]
        );
        let array = destination.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.mem.u32(e.get(array, NiTArray::m_pBase)), clone);
        // The palette's clone is the same test object: stored at +0x78.
        assert_eq!(
            e.get(destination, NiControllerManager::m_spObjectPalette)
                .addr(),
            clone
        );
    }

    #[test]
    fn fn_00a2f380_without_sequences_or_palette_copies_only_the_flag() {
        let mut e = setup();
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let this = manager(&mut e);
        let destination = manager(&mut e);
        e.set(this, NiControllerManager::m_bCumulative, 5);
        e.call(0x00a2f380, &args![this, destination, 0x7777u32]);
        assert_eq!(e.get(destination, NiControllerManager::m_bCumulative), 5);
        assert!(calls_to(&e, CREATE_CLONE).is_empty());
    }

    #[test]
    fn process_clone_shares_or_clones_the_temporary_blend_sequences() {
        const MAP_VTABLE: u32 = 0x0300_6000;
        const HASH: u32 = 0x0300_6004;
        const KEYS_EQUAL: u32 = 0x0300_6008;
        let mut e = setup();
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(PROCESS_CLONE, |_, _| Ret::default());
        e.register(SEQUENCE_ATTACH_TO_TARGET, |_, _| Ret::default());
        e.register(HASH, |_, _| 0u32.into_ret());
        e.register(KEYS_EQUAL, |_, a| (a[1] == a[2]).into_ret());
        e.register(SET_REALLOC, |e, a| {
            let old = e.mem.u32(a[0]);
            let used = e.mem.u32(a[0] + 8);
            let storage = e.mem.alloc(4 * a[1]);
            for index in 0..used {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(storage + 4 * index, value);
            }
            e.mem.set_u32(a[0], storage);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.put_vtable(MAP_VTABLE, &[0, HASH, KEYS_EQUAL]);
        let this = manager(&mut e);
        e.set(this, NiControllerManager::m_pkTarget, Ptr::new(0x7700));
        let clone = manager(&mut e);
        let sequence = object(&mut e);
        set_sequences(&mut e, this, &[sequence], 0);
        let shared = object(&mut e);
        let cloned = object(&mut e);
        let counterpart = object(&mut e);
        let pool = e.mem.alloc(8);
        e.mem.set_u32(pool, cloned);
        e.mem.set_u32(pool + 4, shared);
        let set = this.at(NiControllerManager::m_kTempBlendSeqs);
        e.set(set, NiTPrimitiveSet::m_pBase, pool);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, 2);
        e.set(set, NiTPrimitiveSet::m_uiUsed, 2);
        let palette = object(&mut e);
        e.set(
            this,
            NiControllerManager::m_spObjectPalette,
            Ptr::new(palette),
        );
        // The map: one bucket holding the item (cloned -> counterpart); the lookup of the
        // manager itself answers the clone manager.
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, MAP_VTABLE);
        let table = e.mem.alloc(4);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 4, cloned);
        e.mem.set_u32(item + 8, counterpart);
        e.mem.set_u32(table, item);
        e.mem.set_u32(map + 8, table);
        let controller = this.addr();
        let clone_address = clone.addr();
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            assert_eq!(a[1], controller);
            e.mem.set_u32(a[2], clone_address);
            true.into_ret()
        });
        let process = e.mem.alloc(8);
        e.mem.set_u32(process, map);
        e.call_log = Some(vec![]);
        e.call(0x00a2f420, &args![this, process]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_PROCESS_CLONE),
            vec![vec![controller, process]]
        );
        assert_eq!(
            calls_to(&e, PROCESS_CLONE),
            vec![vec![sequence, process], vec![palette, process]]
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_ATTACH_TO_TARGET),
            vec![vec![sequence, 0x7700]]
        );
        let clone_set = clone.at(NiControllerManager::m_kTempBlendSeqs);
        assert_eq!(e.get(clone_set, NiTPrimitiveSet::m_uiUsed), 2);
        assert_eq!(
            calls_to(&e, SET_REALLOC),
            vec![vec![clone_set.addr(), 1], vec![clone_set.addr(), 2]]
        );
        let base = e.get(clone_set, NiTPrimitiveSet::m_pBase);
        assert_eq!(
            [e.mem.u32(base), e.mem.u32(base + 4)],
            [counterpart, shared]
        );
        assert_eq!(count_of(&e, counterpart), 2);
        assert_eq!(count_of(&e, shared), 2);
    }

    #[test]
    fn register_streamables_registers_each_sequence_and_the_palette() {
        let mut e = setup();
        e.register_double(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, _| {
            false.into_ret()
        });
        e.register(REGISTER, |_, _| Ret::default());
        let this = manager(&mut e);
        let sequence = object(&mut e);
        set_sequences(&mut e, this, &[sequence, 0], 0);
        assert!(!e.call(0x00a2f610, &args![this, 0x9900u32]).bool());
        assert!(calls_to(&e, REGISTER).is_empty());
        e.register_double(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, _| true.into_ret());
        let palette = object(&mut e);
        e.set(
            this,
            NiControllerManager::m_spObjectPalette,
            Ptr::new(palette),
        );
        assert!(e.call(0x00a2f610, &args![this, 0x9900u32]).bool());
        assert_eq!(
            calls_to(&e, REGISTER),
            vec![vec![sequence, 0x9900], vec![palette, 0x9900]]
        );
    }

    #[test]
    fn register_streamables_makes_a_palette_when_there_is_none() {
        let mut e = setup();
        constructor_doubles(&mut e);
        e.register(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, _| true.into_ret());
        let this = manager(&mut e);
        e.set(this, NiControllerManager::m_pkTarget, Ptr::new(0x7700));
        assert!(e.call(0x00a2f610, &args![this, 0x9900u32]).bool());
        let palette = e.get(this, NiControllerManager::m_spObjectPalette).addr();
        assert_ne!(palette, 0);
        assert_eq!(e.mem.u32(palette + 0xc), 0x25);
        assert_eq!(e.mem.u32(palette + 0x18), 0x7700);
        assert_eq!(count_of(&e, palette), 1);
        assert!(calls_to(&e, REGISTER).is_empty());
    }

    #[test]
    fn link_object_adopts_each_sequence_that_has_no_owner() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 0);
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| 2u32.into_ret());
        let free = object(&mut e);
        let owned = object(&mut e);
        e.mem.set_u32(owned + 0x40, 0x1234);
        let links = Rc::new(RefCell::new(vec![free, owned]));
        let queue = links.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2f6e0, &args![this, 0x9900u32]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_LINK_OBJECT),
            vec![vec![this.addr(), 0x9900]]
        );
        let array = this.at(NiControllerManager::m_kSequenceArray);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.mem.u32(e.get(array, NiTArray::m_pBase)), free);
        assert_eq!(e.mem.u32(free + 0x40), this.addr());
        assert_eq!(e.mem.u32(free + 0x5c), 0x7700);
        assert_eq!(e.mem.u32(free + 8), 0x7800);
        assert_eq!(
            calls_to(&e, STRING_MAP_SET_AT),
            vec![vec![this.addr() + 0x50, 0x7800, free]]
        );
        assert_eq!(e.mem.u32(owned + 0x40), 0x1234);
    }

    #[test]
    fn link_object_with_no_links_only_sizes_the_array() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 0);
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| Ret::default());
        e.call(0x00a2f6e0, &args![this, 0x9900u32]);
        assert!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).is_empty());
    }

    #[test]
    fn fn_00a2f800_starts_a_blend_from_an_inactive_sequence() {
        let mut e = setup();
        let (this, _) = adding_manager(&mut e, 4);
        e.register(SEQUENCE_DETACH, |_, _| Ret::default());
        e.register(SEQUENCE_COPY_BLEND_POSE, |_, _| Ret::default());
        e.register(SEQUENCE_START_BLEND, |_, _| true.into_ret());
        let idle = object(&mut e);
        let pool = e.mem.alloc(4);
        e.mem.set_u32(pool, idle);
        let set = this.at(NiControllerManager::m_kTempBlendSeqs);
        e.set(set, NiTPrimitiveSet::m_pBase, pool);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, 1);
        e.set(set, NiTPrimitiveSet::m_uiUsed, 1);
        let sequence = object(&mut e);
        e.mem.set_u32(sequence + 0x44, 1);
        let first = f32::from_bits(0x7f80_0001);
        let call = |e: &mut Engine| {
            e.call(
                0x00a2f800,
                &args![this, sequence, first, 0.25f32, 9u32, 0x44u32],
            )
            .bool()
        };
        assert!(!call(&mut e));
        assert!(calls_to(&e, SEQUENCE_START_BLEND).is_empty());
        e.mem.set_u32(sequence + 0x44, 0);
        assert!(call(&mut e));
        assert_eq!(
            calls_to(&e, SEQUENCE_START_BLEND),
            vec![vec![
                idle,
                sequence,
                0.25f32.to_bits(),
                0x7fc0_0001,
                9,
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(calls_to(&e, SEQUENCE_COPY_BLEND_POSE)[0][3], 0x44);
    }

    #[test]
    fn create_clone_makes_a_manager_and_copies_into_it() {
        let mut e = setup();
        constructor_doubles(&mut e);
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let this = manager(&mut e);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a2f850, &args![this, 0x7777u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x7c]]);
        assert_eq!(e.mem.u32(clone), NI_CONTROLLER_MANAGER_VTABLE);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x7777]]
        );
    }

    #[test]
    fn create_clone_copies_into_address_0_when_there_is_no_memory() {
        let mut e = setup();
        e.mem.map(0, 0x1000);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let this = manager(&mut e);
        assert_eq!(e.call(0x00a2f850, &args![this, 0x7777u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), 0, 0x7777]]
        );
    }

    #[test]
    fn ni_av_object_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2e1e0, &args![]).u32(),
            NI_AV_OBJECT_PALETTE_RTTI
        );
    }

    #[test]
    fn ni_default_av_object_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2ede0, &args![]).u32(),
            NI_DEFAULT_AV_OBJECT_PALETTE_RTTI
        );
    }

    #[test]
    fn ni_controller_manager_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a2_ebb0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a2ef80, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a2ef80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x7c]]);
        assert_eq!(calls_to(&e, 0x00a2_ebb0), vec![vec![this]; 2]);
    }

    #[test]
    fn create_object_00a2f6c0_constructs_a_new_manager_or_answers_null() {
        let mut e = setup();
        constructor_doubles(&mut e);
        let object = e.call(0x00a2_f6c0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), NI_CONTROLLER_MANAGER_VTABLE);
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], vec![0x7c]);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT), vec![vec![object]]);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_f6c0, &args![]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT).is_empty());
    }
}
