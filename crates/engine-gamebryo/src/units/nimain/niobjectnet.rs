//! `NiMain/niobjectnet.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiObjectNET` (0x18 bytes, vtable `0109b244`) is the `NiObject` that has a
//! name, a chain of time controllers and a list of extra data:
//!
//! - `m_kName` (+8) is a `NiFixedString` handle: a pointer to the pooled
//!   characters, with the string's reference count in the word 8 bytes in
//!   front of them. The handle of the empty string is the word at `0109b220`
//!   (set at start-up).
//! - `m_spControllers` (+0xC) is the first `NiTimeController`; each controller
//!   holds the next one at +0x30. Every pointer to a controller is a counted
//!   reference (count at +4; `DeleteThis`, vtable slot 4, runs at zero).
//! - `m_ppkExtra` (+0x10), `m_usExtraDataSize` (+0x14) and `m_usMaxSize`
//!   (+0x16) are a growable array of counted `NiExtraData` pointers, kept
//!   sorted by the name handle of each entry (compared as signed 32-bit
//!   differences) with no two entries of the same name. Every access to the
//!   array runs inside the critical section at `011f4380`.
//!
//! The name of an extra data object is read through `00a62660`, which the
//! engine map names `NiBSplineBasisData::GetDegree3Basis` because the linker
//! folded the identical code: its body is `lea eax, [ecx + 8]`, the address of
//! the word at +8 of the object.
//!
//! Only `AddExtraData_ov2` (`00a5bca0`) is partial: it carries the `/GS`
//! security-cookie check. Its callers here (`LinkObject`, `CopyMembers`,
//! `PostLinkObject`) are complete themselves.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The critical section every extra data array access takes, and the import
/// slots of `EnterCriticalSection` / `LeaveCriticalSection`.
const CRITICAL_SECTION: u32 = 0x011f_4380;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// Import slots of `InterlockedIncrement` / `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The word that holds the handle of the empty `NiFixedString`.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The vtable `NiObjectNET`'s constructor stores.
pub(crate) const NI_OBJECT_NET_VTABLE: u32 = 0x0109_b244;
/// Vtable of `NiTArray<unsigned short, NiTMallocInterface<unsigned short>>`
/// (RTTI name read from the exe; the deleting destructor `00a5c780` stores it).
const USHORT_ARRAY_VTABLE: u32 = 0x0109_b23c;
/// Vtable of the `NiTPrimitiveArray<unsigned short>` locals `IsEqual` builds
/// (RTTI name read from the exe).
const USHORT_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0109_b2d4;
/// The `NiRTTI` record `GetRTTI` answers; its first word is the class name
/// (`"NiObjectNET"`, filled in at start-up).
const NI_OBJECT_NET_RTTI: u32 = 0x011f_4304;
/// The `NiRTTI` of `NiVertWeightsExtraData` (what `00a89b20`, the engine
/// map's `NiVertWeightsExtraData::GetRTTI`, answers).
const VERT_WEIGHTS_EXTRA_DATA_RTTI: u32 = 0x011f_5b78;
/// `"ExtraData"`, the format `"ED%03d"` and `"m_kName"`.
const EXTRA_DATA_TEXT: u32 = 0x0109_b224;
const EXTRA_DATA_NAME_FORMAT: u32 = 0x0109_b230;
const NAME_FIELD_TEXT: u32 = 0x0109_b2d8;
/// The `MemoryManager` singleton the deleting destructor of the array passes
/// to `Deallocate`.
const MEMORY_MANAGER: u32 = 0x011f_6238;

/// Allocation helpers (cdecl): `alloc(bytes)`, `free(block)`,
/// `operator delete(block, size)` and `memcpy(dst, src, bytes)`.
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiSprintf(buffer, size, format, ...)`, `strstr`, the `_strncpy_s`
/// wrapper `(dst, size, src, count)` (it calls `00ec8d5f`, the CRT's
/// `_strncpy_s`), `_strcat_s`, `_strcpy_s` (all cdecl).
const NI_SPRINTF: u32 = 0x00a3_05b0;
const STRSTR: u32 = 0x00ec_7750;
const STRNCPY_S: u32 = 0x00a5_74b0;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRCPY_S: u32 = 0x00ec_65a6;
/// `NiFixedString::NiFixedString(char *)` stand-in used by `AddExtraData`
/// (`thiscall` on the slot that receives the handle, then the characters)
/// and the plain intern function (`cdecl`, characters, answers the handle).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The address of the name word of an extra data object (`this + 8`).
const NAME_ADDRESS: u32 = 0x00a6_2660;

/// `NiObject` pieces the class builds on: constructor, destructor body,
/// `ProcessClone`, `CopyMembers(this, clone, process)`, `LoadBinary`,
/// `RegisterStreamables`, `IsEqual`, `GetViewerStrings`.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// The empty stub (`RET 4`) that `LinkObject`, `SaveBinary` and
/// `PostLinkObject` call first (`this`, stream).
const LINK_STUB: u32 = 0x00a8_9af0;
/// `NiStream` methods (`this` = the stream): `LoadFixedString(&name)`,
/// `LoadCStringAsFixedString(&name)`, `ReadMultipleLinkIDs()`, the one-word
/// method the engine map has no name for that `LoadBinary` calls after them,
/// `GetObjectFromLinkID()`, `GetNumberOfLinkIDs()` and
/// `SaveFixedString(&name)`.
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB, same offset on PC) and
/// `m_pkOstr`, the output stream (`NiBinaryStream`: counter at +4, write
/// function pointer at +0xC).
const STREAM_FILE_VERSION: u32 = 0xd8;
const STREAM_OUTPUT: u32 = 0x250;
/// Stream versions the load code compares with (`0x14010001`, `0x0500000B`).
const VERSION_FIXED_STRINGS: u32 = 0x1401_0001;
const VERSION_EXTRA_DATA_ARRAY: u32 = 0x0500_000b;
/// `NiTimeController` helpers: the length of a controller chain and the
/// smart-pointer assignment `(holder, new)` used on the head of the chain,
/// the same assignment on a controller's next pointer `(controller, new)` and
/// the destructor of the temporary `NiPointer`.
const CONTROLLER_CHAIN_LENGTH: u32 = 0x00a6_cd00;
const CONTROLLER_HEAD_ASSIGN: u32 = 0x00a5_f860;
const CONTROLLER_NEXT_ASSIGN: u32 = 0x00a6_7260;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Clears the word at +8 of the object (the engine map's
/// `NiAccumulator::FinishAccumulating`, folded identical code).
const CLEAR_WORD_AT_8: u32 = 0x00aa_0fe0;
/// `NiTPrimitiveArray<unsigned short>` helper `(array, newSize)` that grows
/// the element storage, and the same two helpers for the string array
/// `GetViewerStrings` fills.
const USHORT_ARRAY_GROW: u32 = 0x00a5_b790;
/// `NiGetViewerString(char *)` and `NiGetViewerString_ov11(char *, char *)`
/// (cdecl) and the string array helpers `(array, newSize)` / `(array, index,
/// &value)`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_PAIR: u32 = 0x00a7_d5e0;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// Vtable slots (byte offsets) the code indexes. `SLOT_DELETE_THIS` is
/// `NiRefObject::DeleteThis`; the next four are `NiObject` slots named in the
/// Xbox PDB; the three after them are slots of the extra data / controller /
/// stream classes whose names this unit does not confirm.
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_CREATE_CLONE: u32 = 0x48;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_PROCESS_CLONE: u32 = 0x68;
/// Extra data slot that answers whether the entry is streamed (`SaveBinary`,
/// `RegisterStreamables` and `IsEqual` ask it) and the one asked before
/// cloning an entry. Neither is named in the Xbox PDB dump for these offsets.
const SLOT_EXTRA_STREAMABLE: u32 = 0x8c;
const SLOT_EXTRA_CLONEABLE: u32 = 0x90;
/// Controller slot `SaveBinary` asks while it looks for the first controller
/// to save.
const SLOT_CONTROLLER_SAVEABLE: u32 = 0xac;
/// Stream slot `0x20` (`RegisterFixedString` in the Xbox PDB) and `0x30`
/// (`SaveLinkID`).
const SLOT_STREAM_REGISTER_FIXED_STRING: u32 = 0x20;
const SLOT_STREAM_SAVE_LINK_ID: u32 = 0x30;

/// Cloning process (`NiCloningProcess`, Xbox PDB): `m_eCopyType` (+8) and
/// `m_cAppendChar` (+0xC).
const PROCESS_COPY_TYPE: u32 = 0x08;
const PROCESS_APPEND_CHAR: u32 = 0x0c;
const COPY_EXACT: u32 = 1;
const COPY_UNIQUE: u32 = 2;

/// Offset of the next controller in a controller.
const CONTROLLER_NEXT: u32 = 0x30;

layout! {
    /// `NiObjectNET` (Xbox PDB), 0x18 bytes on PC and Xbox alike.
    pub struct NiObjectNET: 0x18 {
        /// `m_kName` (Xbox PDB): a `NiFixedString` handle.
        0x08 m_kName: u32,
        /// `m_spControllers` (Xbox PDB): `NiPointer<NiTimeController>`.
        0x0C m_spControllers: Ptr,
        /// `m_ppkExtra` (Xbox PDB): the array of `NiExtraData *`.
        0x10 m_ppkExtra: Ptr,
        /// `m_usExtraDataSize` (Xbox PDB): number of entries.
        0x14 m_usExtraDataSize: u16,
        /// `m_usMaxSize` (Xbox PDB): capacity of the array.
        0x16 m_usMaxSize: u16,
    }
}

// ---------------------------------------------------------------------------
// Helpers shared by the functions below

fn lock(e: &mut Engine) {
    e.call(ENTER_CRITICAL_SECTION, &args![CRITICAL_SECTION]);
}

fn unlock(e: &mut Engine) {
    e.call(LEAVE_CRITICAL_SECTION, &args![CRITICAL_SECTION]);
}

fn allocate(e: &mut Engine, bytes: u32) -> u32 {
    e.call(ALLOCATE, &args![bytes]).u32()
}

fn free(e: &mut Engine, block: u32) {
    e.call(FREE, &args![block]);
}

/// `InterlockedIncrement` of the reference count at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// `InterlockedDecrement` of the reference count at +4; `DeleteThis` (vtable
/// slot 4) runs when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The handle of the empty `NiFixedString`.
fn empty_name(e: &Engine) -> u32 {
    e.global::<u32>(EMPTY_FIXED_STRING)
}

/// Counts one more reference to a pooled string (the count is the word 8
/// bytes in front of the characters); the empty string is not counted.
fn name_add_ref(e: &mut Engine, handle: u32) {
    if handle != empty_name(e) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Drops one reference to a pooled string; the empty string is not counted.
fn name_release(e: &mut Engine, handle: u32) {
    if handle != empty_name(e) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The name handle of an extra data object: the word at the address
/// `00a62660` answers.
fn extra_name(e: &mut Engine, extra: u32) -> u32 {
    let address = e.call(NAME_ADDRESS, &args![extra]).u32();
    e.mem.u32(address)
}

/// Points the counted reference stored at `slot` to `new`: nothing happens if
/// it already does; otherwise the old target is released, `new` is stored and
/// counted.
fn assign_reference(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        add_ref(e, new);
    }
}

fn extra_count(e: &Engine, this: Ptr<NiObjectNET>) -> u16 {
    e.get(this, NiObjectNET::m_usExtraDataSize)
}

/// Entry `index` of the extra data array (the array pointer is read afresh,
/// as the game does after every call that could move it).
fn extra_at(e: &Engine, this: Ptr<NiObjectNET>, index: u32) -> u32 {
    let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00a5b870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<unsigned short>` element store (the engine map has no name):
/// `(this, index, &value)` writes the 16-bit value at `index`, extends
/// `m_usSize` to `index + 1` when the index is past it and keeps
/// `m_usESize`, the number of non-zero entries, up to date. The storage is
/// not grown here (callers grow it first).
pub fn fn_00a5b870(e: &mut Engine, this: Ptr<NiTArray>, index: u32, value: Ptr) {
    let new_value = e.mem.u16(value.addr());
    let size = e.get(this, NiTArray::m_usSize) as u32;
    let entries = e.get(this, NiTArray::m_usESize);
    if index >= size {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if new_value != 0 {
            e.set(this, NiTArray::m_usESize, entries.wrapping_add(1));
        }
    } else {
        let base = e.get(this, NiTArray::m_pBase);
        let old = e.mem.u16(base.wrapping_add(index.wrapping_mul(2)));
        if new_value == 0 {
            if old != 0 {
                e.set(this, NiTArray::m_usESize, entries.wrapping_sub(1));
            }
        } else if old == 0 {
            e.set(this, NiTArray::m_usESize, entries.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTArray::m_pBase);
    e.mem
        .set_u16(base.wrapping_add(index.wrapping_mul(2)), new_value);
}

// Translated from 00a5b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::ProcessClone` (Xbox PDB): the `NiObject` part, then
/// `ProcessClone` (vtable slot 0x68) on every extra data entry that is not
/// null and on the first controller.
pub fn ni_object_net_process_clone(e: &mut Engine, this: Ptr<NiObjectNET>, process: Ptr) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, process]);
    if extra_count(e, this) != 0 {
        let mut index: u16 = 0;
        loop {
            let extra = extra_at(e, this, index as u32);
            if extra != 0 {
                e.vcall(extra, SLOT_PROCESS_CLONE, &args![process]);
            }
            index = index.wrapping_add(1);
            if index >= extra_count(e, this) {
                break;
            }
        }
    }
    let controller = e.get(this, NiObjectNET::m_spControllers);
    if !controller.is_null() {
        e.vcall(controller.addr(), SLOT_PROCESS_CLONE, &args![process]);
    }
}

// Translated from 00a5b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a name (the engine map has no name; `AddExtraData` also calls it on
/// an extra data object, which keeps its name at the same offset): `(this,
/// &handle)` counts the new pooled string, releases the old one and stores
/// the handle, unless it is the one already stored. The empty string is never
/// counted; a null old name (only possible for the objects the old-format
/// `PostLinkObject` chain produces) is not the empty string, so the game then
/// decrements the word at `0xfffffff8`, and the translation does the same.
pub fn fn_00a5b950(e: &mut Engine, this: Ptr<NiObjectNET>, handle: Ptr) {
    let new = e.mem.u32(handle.addr());
    if e.get(this, NiObjectNET::m_kName) == new {
        return;
    }
    name_add_ref(e, new);
    let old = e.get(this, NiObjectNET::m_kName);
    name_release(e, old);
    e.set(this, NiObjectNET::m_kName, new);
}

// Translated from 00a5b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::RemoveExtraDataAt` (Xbox PDB): inside the critical section,
/// when `index` is below the entry count, releases that entry (its
/// `DeleteThis` runs when the count reaches zero), moves the later entries
/// down one place, shrinks the count and clears the vacated slot.
pub fn ni_object_net_remove_extra_data_at(e: &mut Engine, this: Ptr<NiObjectNET>, index: u16) {
    lock(e);
    if index < extra_count(e, this) {
        let extra = extra_at(e, this, index as u32);
        if extra != 0 {
            release(e, extra);
        }
        let last = extra_count(e, this) as i32 - 1;
        if (index as i32) < last {
            let mut at = index as u32;
            loop {
                let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
                let next = e
                    .mem
                    .u32(base.wrapping_add(at.wrapping_mul(4)).wrapping_add(4));
                e.mem.set_u32(base.wrapping_add(at.wrapping_mul(4)), next);
                at = (at.wrapping_add(1)) as u16 as u32;
                if (at as i32) >= extra_count(e, this) as i32 - 1 {
                    break;
                }
            }
        }
        let count = extra_count(e, this).wrapping_sub(1);
        e.set(this, NiObjectNET::m_usExtraDataSize, count);
        let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
        e.mem.set_u32(base.wrapping_add(count as u32 * 4), 0);
    }
    unlock(e);
}

// Translated from 00a5ba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an extra data object to the sorted array (the engine map has no name;
/// `AddExtraData` and its overload end here): null answers false. Otherwise
/// the object is counted, the array is created (one slot) or doubled plus one
/// when full, the object is put at the end, the unused slots are cleared and
/// the new entry is moved down while the entry before it has a larger name
/// handle (signed difference). If an entry with the same name is met the new
/// object is removed again and the answer is false.
pub fn fn_00a5ba40(e: &mut Engine, this: Ptr<NiObjectNET>, extra: Ptr) -> bool {
    if extra.is_null() {
        return false;
    }
    add_ref(e, extra.addr());
    lock(e);
    let capacity = e.get(this, NiObjectNET::m_usMaxSize);
    if capacity < 1 {
        e.set(this, NiObjectNET::m_usMaxSize, 1);
        e.set(this, NiObjectNET::m_usExtraDataSize, 1);
        let block = allocate(e, 4);
        e.set(this, NiObjectNET::m_ppkExtra, Ptr::new(block));
        e.mem.set_u32(block, extra.addr());
        unlock(e);
        return true;
    }
    if extra_count(e, this) == capacity {
        let new_capacity = (capacity as u32 * 2 + 1) as u16;
        e.set(this, NiObjectNET::m_usMaxSize, new_capacity);
        let block = allocate(e, new_capacity as u32 * 4);
        let bytes = extra_count(e, this) as u32 * 4;
        let old = e.get(this, NiObjectNET::m_ppkExtra).addr();
        e.call(MEMCPY, &args![block, old, bytes]);
        free(e, old);
        e.set(this, NiObjectNET::m_ppkExtra, Ptr::new(block));
    }
    let count = extra_count(e, this);
    let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
    e.mem
        .set_u32(base.wrapping_add(count as u32 * 4), extra.addr());
    e.set(this, NiObjectNET::m_usExtraDataSize, count.wrapping_add(1));
    let mut clear = extra_count(e, this);
    if clear < e.get(this, NiObjectNET::m_usMaxSize) {
        loop {
            let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
            e.mem.set_u32(base.wrapping_add(clear as u32 * 4), 0);
            clear = clear.wrapping_add(1);
            if clear >= e.get(this, NiObjectNET::m_usMaxSize) {
                break;
            }
        }
    }
    let mut at: u16 = extra_count(e, this).wrapping_sub(1);
    while at != 0 {
        let offset = at as u32 * 4;
        let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
        let before = e.mem.u32(base.wrapping_add(offset).wrapping_sub(4));
        let before_name = extra_name(e, before);
        let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
        let current = e.mem.u32(base.wrapping_add(offset));
        let current_name = extra_name(e, current);
        let difference = before_name.wrapping_sub(current_name);
        if difference == 0 {
            ni_object_net_remove_extra_data_at(e, this, at);
            unlock(e);
            return false;
        }
        if (difference as i32) < 0 {
            break;
        }
        let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
        let upper = e.mem.u32(base.wrapping_add(offset));
        let lower = e.mem.u32(base.wrapping_add(offset).wrapping_sub(4));
        e.mem
            .set_u32(base.wrapping_add(offset).wrapping_sub(4), upper);
        let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
        e.mem.set_u32(base.wrapping_add(offset), lower);
        at = at.wrapping_sub(1);
    }
    unlock(e);
    true
}

// Translated from 00a5bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::SetExtraDataSize` (Xbox PDB): size 0 answers false.
/// Otherwise, inside the critical section, the capacity becomes `size`; a
/// first array is allocated with an empty count, an existing one is copied
/// (`count * 4` bytes, as the game does even if that exceeds the new size)
/// into a new block and the old block is freed. Answers true.
pub fn ni_object_net_set_extra_data_size(
    e: &mut Engine,
    this: Ptr<NiObjectNET>,
    size: u16,
) -> bool {
    if size == 0 {
        return false;
    }
    lock(e);
    let old_capacity = e.get(this, NiObjectNET::m_usMaxSize);
    e.set(this, NiObjectNET::m_usMaxSize, size);
    if old_capacity == 0 {
        let block = allocate(e, size as u32 * 4);
        e.set(this, NiObjectNET::m_ppkExtra, Ptr::new(block));
        e.set(this, NiObjectNET::m_usExtraDataSize, 0);
        unlock(e);
        return true;
    }
    let block = allocate(e, size as u32 * 4);
    let old = e.get(this, NiObjectNET::m_ppkExtra).addr();
    let bytes = extra_count(e, this) as u32 * 4;
    e.call(MEMCPY, &args![block, old, bytes]);
    free(e, old);
    e.set(this, NiObjectNET::m_ppkExtra, Ptr::new(block));
    unlock(e);
    true
}

// Translated from 00a5bc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::AddExtraData` (Xbox PDB), the overload taking `(&name,
/// extraData)`: false if the name is the empty string or the object is null.
/// An object whose own name is the empty string gets `name` and is added; one
/// that already has a name is added only if it is the same handle.
pub fn ni_object_net_add_extra_data(
    e: &mut Engine,
    this: Ptr<NiObjectNET>,
    name: Ptr,
    extra: Ptr,
) -> bool {
    if e.mem.u32(name.addr()) == empty_name(e) {
        return false;
    }
    if extra.is_null() {
        return false;
    }
    if extra_name(e, extra.addr()) == empty_name(e) {
        fn_00a5b950(e, extra.cast(), name);
        return fn_00a5ba40(e, this, extra);
    }
    let own = extra_name(e, extra.addr());
    if e.mem.u32(name.addr()) == own {
        return fn_00a5ba40(e, this, extra);
    }
    false
}

// Translated from 00a5bca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a5bca0 - the /GS security-cookie check (00ec408c) on the way out is not translated.
/// `NiObjectNET::AddExtraData` (Xbox PDB `AddExtraData_ov2`), the overload
/// taking only the object: null answers false. If the object's name word is
/// null and its class name (`GetRTTI`, vtable slot 8, first word of the
/// `NiRTTI`) is not empty, it is named: `"ED%03d"` of the current entry count
/// is formatted into an 8-byte buffer, the part of the class name before
/// `"ExtraData"` (or nothing if it starts there or does not contain it) is
/// copied and the formatted text appended, and that string becomes the
/// object's name. Then the object is added with `00a5ba40`, whose answer is
/// returned.
pub fn ni_object_net_add_extra_data_ov2(
    e: &mut Engine,
    this: Ptr<NiObjectNET>,
    extra: Ptr,
) -> bool {
    if extra.is_null() {
        return false;
    }
    if extra_name(e, extra.addr()) == 0 {
        let rtti = e.vcall(extra.addr(), SLOT_GET_RTTI, &args![]).u32();
        let class_name = e.mem.u32(rtti);
        if class_name != 0 && !e.mem.cstr(class_name).is_empty() {
            let index = extra_count(e, this) as u32;
            // Locals: the 8-byte text buffer and the temporary handle.
            e.with_stack(12, |e, locals| {
                let buffer = locals.addr();
                let handle_slot = locals.addr() + 8;
                e.call(
                    NI_SPRINTF,
                    &args![buffer, 6u32, EXTRA_DATA_NAME_FORMAT, index],
                );
                let found = e.call(STRSTR, &args![class_name, EXTRA_DATA_TEXT]).u32();
                let prefix = found.saturating_sub(class_name);
                let size = e.mem.cstr(buffer).len() as u32 + prefix + 1;
                let text = allocate(e, size);
                e.call(STRNCPY_S, &args![text, size, class_name, prefix]);
                e.call(STRCAT_S, &args![text, size, buffer]);
                e.call(FIXED_STRING_CONSTRUCT, &args![handle_slot, text]);
                fn_00a5b950(e, extra.cast(), Ptr::new(handle_slot));
                let handle = e.mem.u32(handle_slot);
                name_release(e, handle);
                free(e, text);
            });
        }
    }
    fn_00a5ba40(e, this, extra)
}

/// The binary search `GetExtraData` and `RemoveExtraData` share: the index
/// (16-bit, as the game keeps it) of the entry whose name handle equals the
/// handle stored at `key_slot`. Bounds are signed 16-bit values and the
/// order is the signed 32-bit difference of the handles, as in
/// `00a5ba40`. The caller holds the critical section.
fn search_extra_data(e: &mut Engine, this: Ptr<NiObjectNET>, key_slot: u32) -> Option<u16> {
    let mut high: u16 = extra_count(e, this).wrapping_sub(1);
    let mut low: u16 = 0;
    if (high as i16) < 0 {
        return None;
    }
    loop {
        let key = e.mem.u32(key_slot);
        let mid = ((high as i16 as i32 + low as i16 as i32) >> 1) as u16;
        let extra = extra_at(e, this, mid as i16 as i32 as u32);
        let name = extra_name(e, extra);
        let difference = key.wrapping_sub(name) as i32;
        if difference == 0 {
            return Some(mid);
        }
        if difference > 0 {
            low = mid.wrapping_add(1);
        } else {
            high = mid.wrapping_sub(1);
        }
        if (low as i16) > (high as i16) {
            return None;
        }
    }
}

// Translated from 00a5bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::GetExtraData` (Xbox PDB): `(this, &name)` answers the extra
/// data object whose name handle is the one stored at `name` (binary search
/// inside the critical section), or null. The empty string answers null
/// without taking the lock.
pub fn ni_object_net_get_extra_data(e: &mut Engine, this: Ptr<NiObjectNET>, name: Ptr) -> Ptr {
    if e.mem.u32(name.addr()) == empty_name(e) {
        return Ptr::NULL;
    }
    lock(e);
    let found = search_extra_data(e, this, name.addr());
    unlock(e);
    match found {
        Some(index) => Ptr::new(extra_at(e, this, index as i16 as i32 as u32)),
        None => Ptr::NULL,
    }
}

// Translated from 00a5be90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::RemoveExtraData` (Xbox PDB): `(this, &name)` removes the
/// entry with that name handle and answers true; false if there are no
/// entries (no lock taken), the name is the empty string or nothing matches.
pub fn ni_object_net_remove_extra_data(e: &mut Engine, this: Ptr<NiObjectNET>, name: Ptr) -> bool {
    if extra_count(e, this) == 0 {
        return false;
    }
    lock(e);
    if e.mem.u32(name.addr()) == empty_name(e) {
        unlock(e);
        return false;
    }
    let found = search_extra_data(e, this, name.addr());
    let removed = match found {
        Some(index) => {
            ni_object_net_remove_extra_data_at(e, this, index);
            true
        }
        None => false,
    };
    unlock(e);
    removed
}

// Translated from 00a5bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked removal by index (the engine map has no name): inside the
/// critical section, `RemoveExtraDataAt` when `index` is below the count
/// (true), else false.
pub fn fn_00a5bf50(e: &mut Engine, this: Ptr<NiObjectNET>, index: u16) -> bool {
    lock(e);
    if index < extra_count(e, this) {
        ni_object_net_remove_extra_data_at(e, this, index);
        unlock(e);
        return true;
    }
    unlock(e);
    false
}

// Translated from 00a5bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes every extra data entry (the engine map has no name; `fastcall`):
/// inside the critical section, `RemoveExtraDataAt` from the last entry down
/// to the first, then frees the array and clears the pointer, count and
/// capacity.
pub fn fn_00a5bfa0(e: &mut Engine, this: Ptr<NiObjectNET>) {
    lock(e);
    let mut index: u16 = extra_count(e, this).wrapping_sub(1);
    while (index as i16) >= 0 {
        ni_object_net_remove_extra_data_at(e, this, index);
        index = index.wrapping_sub(1);
    }
    let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
    free(e, base);
    e.set(this, NiObjectNET::m_ppkExtra, Ptr::NULL);
    e.set(this, NiObjectNET::m_usMaxSize, 0);
    e.set(this, NiObjectNET::m_usExtraDataSize, 0);
    unlock(e);
}

// Translated from 00a5c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the first controller (the engine map has no name): the counted
/// reference at +0xC is pointed to `controller` (old one released, new one
/// counted) unless it already is.
pub fn fn_00a5c000(e: &mut Engine, this: Ptr<NiObjectNET>, controller: Ptr) {
    assign_reference(e, this.addr() + 0x0c, controller.addr());
}

// Translated from 00a5c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::RemoveAllControllers` (Xbox PDB; `fastcall`): releases the
/// first controller, if any, and clears the pointer.
pub fn ni_object_net_remove_all_controllers(e: &mut Engine, this: Ptr<NiObjectNET>) {
    let controller = e.get(this, NiObjectNET::m_spControllers);
    if !controller.is_null() {
        release(e, controller.addr());
        e.set(this, NiObjectNET::m_spControllers, Ptr::NULL);
    }
}

// Translated from 00a5c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::LoadBinary` (Xbox PDB): the `NiObject` part, then the name
/// (`LoadFixedString` from stream version `0x14010001`, else
/// `LoadCStringAsFixedString`), then below version `0x0500000B` two
/// `00a645b0` reads (the single extra data link and the controller link),
/// from there on `ReadMultipleLinkIDs` and one `00a645b0` read.
pub fn ni_object_net_load_binary(e: &mut Engine, this: Ptr<NiObjectNET>, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let name_slot = this.addr() + 8;
    if e.mem.u32(stream.addr() + STREAM_FILE_VERSION) < VERSION_FIXED_STRINGS {
        e.call(
            STREAM_LOAD_C_STRING_AS_FIXED_STRING,
            &args![stream, name_slot],
        );
    } else {
        e.call(STREAM_LOAD_FIXED_STRING, &args![stream, name_slot]);
    }
    if e.mem.u32(stream.addr() + STREAM_FILE_VERSION) < VERSION_EXTRA_DATA_ARRAY {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    } else {
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    }
}

// Translated from 00a5c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::LinkObject` (Xbox PDB): below stream version `0x0500000B`
/// the word at +0x10 is set to the object the next link id names (the old
/// single extra data chain, finished by `PostLinkObject`); from there on the
/// array is sized for the link count and each linked object is added with
/// `AddExtraData_ov2`. Then the first controller is set to the next linked
/// object.
pub fn ni_object_net_link_object(e: &mut Engine, this: Ptr<NiObjectNET>, stream: Ptr) {
    e.call(LINK_STUB, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_FILE_VERSION) < VERSION_EXTRA_DATA_ARRAY {
        let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        e.set(this, NiObjectNET::m_ppkExtra, Ptr::new(linked));
    } else {
        let mut remaining = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
        ni_object_net_set_extra_data_size(e, this, remaining as u16);
        while remaining != 0 {
            let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            ni_object_net_add_extra_data_ov2(e, this, Ptr::new(linked));
            remaining -= 1;
        }
    }
    let controller = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_reference(e, this.addr() + 0x0c, controller);
}

// Translated from 00a5c180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::RegisterStreamables` (Xbox PDB): false if the `NiObject`
/// part or the stream's slot 0x20 (registering the name) answers false.
/// Otherwise every non-null extra data entry that answers true to slot 0x8c
/// gets `RegisterStreamables` (slot 0x54) with the stream, the first
/// controller is asked the same, and the answer is true.
pub fn ni_object_net_register_streamables(
    e: &mut Engine,
    this: Ptr<NiObjectNET>,
    stream: Ptr,
) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let name_slot = this.addr() + 8;
    if !e
        .vcall(
            stream.addr(),
            SLOT_STREAM_REGISTER_FIXED_STRING,
            &args![name_slot],
        )
        .bool()
    {
        return false;
    }
    if extra_count(e, this) != 0 {
        let mut index: u16 = 0;
        loop {
            let extra = extra_at(e, this, index as u32);
            if extra != 0 && e.vcall(extra, SLOT_EXTRA_STREAMABLE, &args![]).bool() {
                e.vcall(extra, SLOT_REGISTER_STREAMABLES, &args![stream]);
            }
            index = index.wrapping_add(1);
            if index >= extra_count(e, this) {
                break;
            }
        }
    }
    let controller = e.get(this, NiObjectNET::m_spControllers);
    if !controller.is_null() {
        e.vcall(controller.addr(), SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a5c200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiObjectNET` constructor (the engine map has no name; `fastcall`):
/// builds the `NiObject` base, stores the vtable `0109b244`, the empty name,
/// no controllers and an empty extra data array. Answers `this`.
pub fn fn_00a5c200(e: &mut Engine, this: Ptr<NiObjectNET>) -> Ptr<NiObjectNET> {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_OBJECT_NET_VTABLE);
    let empty = empty_name(e);
    e.set(this, NiObjectNET::m_kName, empty);
    e.set(this, NiObjectNET::m_spControllers, Ptr::NULL);
    e.set(this, NiObjectNET::m_ppkExtra, Ptr::NULL);
    e.set(this, NiObjectNET::m_usExtraDataSize, 0);
    e.set(this, NiObjectNET::m_usMaxSize, 0);
    this
}

// Translated from 00a5c230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`.
pub fn ni_object_net_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_OBJECT_NET_RTTI)
}

// Translated from 00a5c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiObjectNET` destructor body (the engine map has no name;
/// `fastcall`): stores the vtable, removes every extra data entry, releases
/// the first controller, releases the name's pooled string and runs the
/// `NiObject` destructor body.
pub fn fn_00a5c240(e: &mut Engine, this: Ptr<NiObjectNET>) {
    e.mem.set_u32(this.addr(), NI_OBJECT_NET_VTABLE);
    fn_00a5bfa0(e, this);
    let controller = e.get(this, NiObjectNET::m_spControllers);
    if !controller.is_null() {
        release(e, controller.addr());
    }
    let name = e.get(this, NiObjectNET::m_kName);
    name_release(e, name);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a5c290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET`'s `CopyMembers` (the engine map has no name): `(this, clone,
/// process)`. The `NiObject` part runs first. With the process's copy type
/// (+8) 1 the clone shares this object's name; with 2 the clone gets the
/// name with the process's append character (+0xC) added at the end (a
/// string built in a fresh block and interned, the temporary handle
/// released). Then every non-null extra data entry that answers true to slot
/// 0x90 is cloned (slot 0x48 with the process) and added to the clone with
/// `AddExtraData_ov2`, and the clone's first controller becomes a clone of
/// this one's.
pub fn fn_00a5c290(e: &mut Engine, this: Ptr<NiObjectNET>, clone: Ptr<NiObjectNET>, process: Ptr) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, clone, process]);
    let copy_type = e.mem.u32(process.addr() + PROCESS_COPY_TYPE);
    if copy_type == COPY_EXACT {
        let name = e.get(this, NiObjectNET::m_kName);
        if e.get(clone, NiObjectNET::m_kName) != name {
            name_add_ref(e, name);
            let old = e.get(clone, NiObjectNET::m_kName);
            name_release(e, old);
            e.set(clone, NiObjectNET::m_kName, name);
        }
    } else if copy_type == COPY_UNIQUE {
        let name = e.get(this, NiObjectNET::m_kName);
        if name != 0 {
            let size = e.mem.cstr(name).len() as u32 + 2;
            let text = allocate(e, size);
            let source = e.get(this, NiObjectNET::m_kName);
            e.call(STRCPY_S, &args![text, size, source]);
            let append = e.mem.u8(process.addr() + PROCESS_APPEND_CHAR);
            e.mem.set_u8(text + size - 2, append);
            e.mem.set_u8(text + size - 1, 0);
            let handle = if text == 0 {
                empty_name(e)
            } else {
                e.call(FIXED_STRING_CREATE, &args![text]).u32()
            };
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), handle);
                fn_00a5b950(e, clone, slot);
            });
            name_release(e, handle);
            free(e, text);
        }
    }
    let mut index: u16 = 0;
    if extra_count(e, this) != 0 {
        loop {
            let extra = extra_at(e, this, index as u32);
            if extra != 0 && e.vcall(extra, SLOT_EXTRA_CLONEABLE, &args![]).bool() {
                let copy = e.vcall(extra, SLOT_CREATE_CLONE, &args![process]).u32();
                ni_object_net_add_extra_data_ov2(e, clone, Ptr::new(copy));
            }
            index = index.wrapping_add(1);
            if index >= extra_count(e, this) {
                break;
            }
        }
    }
    let controller = e.get(this, NiObjectNET::m_spControllers);
    if !controller.is_null() {
        let copy = e
            .vcall(controller.addr(), SLOT_CREATE_CLONE, &args![process])
            .u32();
        assign_reference(e, clone.addr() + 0x0c, copy);
    }
}

// Translated from 00a5c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts a controller at the head of the chain (the engine map has no name):
/// null does nothing. Otherwise the controller's next pointer (+0x30) is
/// pointed to the current head and then the head (+0xC) to the controller,
/// each as a counted reference.
pub fn fn_00a5c400(e: &mut Engine, this: Ptr<NiObjectNET>, controller: Ptr) {
    if controller.is_null() {
        return;
    }
    let head = e.get(this, NiObjectNET::m_spControllers);
    assign_reference(e, controller.addr() + CONTROLLER_NEXT, head.addr());
    assign_reference(e, this.addr() + 0x0c, controller.addr());
}

// Translated from 00a5c480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::RemoveController` (Xbox PDB): unlinks `controller` from the
/// chain. Null, or an object without controllers, does nothing. If it is the
/// head, the head becomes its next controller (`00a5f860`) and its own next
/// pointer is released and cleared; the controller is held for the duration
/// (extra count), and deleted if that was the last reference. If it is
/// further down the chain, the controller before it takes over its next
/// pointer (`00a67260`), its own next pointer is released and cleared, and
/// the temporary `NiPointer` that held it is destroyed (`0045cec0`). A
/// controller not in the chain does nothing.
pub fn ni_object_net_remove_controller(e: &mut Engine, this: Ptr<NiObjectNET>, controller: Ptr) {
    if controller.is_null() {
        return;
    }
    let head = e.get(this, NiObjectNET::m_spControllers).addr();
    if head == 0 {
        return;
    }
    let next_slot = controller.addr() + CONTROLLER_NEXT;
    if head == controller.addr() {
        add_ref(e, controller.addr());
        let next = e.mem.u32(next_slot);
        e.call(CONTROLLER_HEAD_ASSIGN, &args![this.addr() + 0x0c, next]);
        let next = e.mem.u32(next_slot);
        if next != 0 {
            release(e, next);
            e.mem.set_u32(next_slot, 0);
        }
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![controller.addr() + 4])
            .u32();
        if count == 0 {
            e.vcall(controller.addr(), SLOT_DELETE_THIS, &args![]);
        }
        return;
    }
    let mut before = head;
    let mut current = e.mem.u32(head + CONTROLLER_NEXT);
    if current == 0 {
        return;
    }
    while current != controller.addr() {
        before = current;
        current = e.mem.u32(current + CONTROLLER_NEXT);
        if current == 0 {
            return;
        }
    }
    add_ref(e, controller.addr());
    let next = e.mem.u32(next_slot);
    e.call(CONTROLLER_NEXT_ASSIGN, &args![before, next]);
    let next = e.mem.u32(next_slot);
    if next != 0 {
        release(e, next);
        e.mem.set_u32(next_slot, 0);
    }
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), controller.addr());
        e.call(NI_POINTER_DESTRUCT, &args![holder]);
    });
}

// Translated from 00a5c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::GetController` (Xbox PDB): the first controller in the
/// chain whose run-time type (`GetRTTI`, slot 8) is `rtti` or derives from it
/// (the walk follows the base-class pointer at +4 of each `NiRTTI`), or null.
pub fn ni_object_net_get_controller(e: &mut Engine, this: Ptr<NiObjectNET>, rtti: Ptr) -> Ptr {
    let mut controller = e.get(this, NiObjectNET::m_spControllers).addr();
    while controller != 0 {
        let mut kind = e.vcall(controller, SLOT_GET_RTTI, &args![]).u32();
        while kind != 0 {
            if kind == rtti.addr() {
                return Ptr::new(controller);
            }
            kind = e.mem.u32(kind + 4);
        }
        controller = e.mem.u32(controller + CONTROLLER_NEXT);
    }
    Ptr::NULL
}

// Translated from 00a5c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::SaveBinary` (Xbox PDB): the stub, the name
/// (`SaveFixedString`), then the number of extra data entries that answer
/// true to slot 0x8c (no null check, as in the game) written as 4 bytes
/// through the output stream's write function pointer (+0xC; its answer is
/// added to the stream's position at +4), then `SaveLinkID` (stream slot
/// 0x30) for each such entry, and finally for the first controller whose slot
/// 0xAC answers true (or null when none does).
pub fn ni_object_net_save_binary(e: &mut Engine, this: Ptr<NiObjectNET>, stream: Ptr) {
    e.call(LINK_STUB, &args![this, stream]);
    e.call(STREAM_SAVE_FIXED_STRING, &args![stream, this.addr() + 8]);
    let mut streamed: u32 = 0;
    let mut index: u16 = 0;
    if extra_count(e, this) != 0 {
        loop {
            let extra = extra_at(e, this, index as u32);
            if e.vcall(extra, SLOT_EXTRA_STREAMABLE, &args![]).bool() {
                streamed += 1;
            }
            index = index.wrapping_add(1);
            if index >= extra_count(e, this) {
                break;
            }
        }
    }
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0x0c);
    // Locals: the count and the size word `4` the write function receives.
    e.with_stack(8, |e, locals| {
        let count_slot = locals.addr();
        let size_slot = locals.addr() + 4;
        e.mem.set_u32(count_slot, streamed);
        e.mem.set_u32(size_slot, 4);
        let written = e
            .call(write, &args![output, count_slot, 4u32, size_slot, 1u32])
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(written));
    });
    let mut index: u16 = 0;
    if extra_count(e, this) != 0 {
        loop {
            let extra = extra_at(e, this, index as u32);
            if e.vcall(extra, SLOT_EXTRA_STREAMABLE, &args![]).bool() {
                e.vcall(stream.addr(), SLOT_STREAM_SAVE_LINK_ID, &args![extra]);
            }
            index = index.wrapping_add(1);
            if index >= extra_count(e, this) {
                break;
            }
        }
    }
    let mut controller = e.get(this, NiObjectNET::m_spControllers).addr();
    while controller != 0
        && !e
            .vcall(controller, SLOT_CONTROLLER_SAVEABLE, &args![])
            .bool()
    {
        controller = e.mem.u32(controller + CONTROLLER_NEXT);
    }
    e.vcall(stream.addr(), SLOT_STREAM_SAVE_LINK_ID, &args![controller]);
}

// Translated from 00a5c6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::PostLinkObject` (Xbox PDB): the stub; below stream version
/// `0x0500000B`, if the word at +0x10 holds a chain (left by `LinkObject`),
/// it is cleared and each object of the chain (the next link is the word at
/// +8, which `00aa0fe0` then clears) is added with `AddExtraData_ov2`. Then,
/// inside the critical section, every extra data entry whose type is
/// `NiVertWeightsExtraData` (`011f5b78`) is removed, restarting from the
/// first entry after each removal.
pub fn ni_object_net_post_link_object(e: &mut Engine, this: Ptr<NiObjectNET>, stream: Ptr) {
    e.call(LINK_STUB, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_FILE_VERSION) < VERSION_EXTRA_DATA_ARRAY {
        let mut chain = e.get(this, NiObjectNET::m_ppkExtra).addr();
        if chain != 0 {
            e.set(this, NiObjectNET::m_ppkExtra, Ptr::NULL);
            loop {
                let next = extra_name(e, chain);
                e.call(CLEAR_WORD_AT_8, &args![chain]);
                ni_object_net_add_extra_data_ov2(e, this, Ptr::new(chain));
                chain = next;
                if chain == 0 {
                    break;
                }
            }
        }
    }
    let mut index: u32 = 0;
    if extra_count(e, this) != 0 {
        loop {
            let extra = extra_at(e, this, index as u16 as u32);
            if extra != 0 {
                let kind = e.vcall(extra, SLOT_GET_RTTI, &args![]).u32();
                if kind == VERT_WEIGHTS_EXTRA_DATA_RTTI {
                    lock(e);
                    if (index as u16) < extra_count(e, this) {
                        ni_object_net_remove_extra_data_at(e, this, index as u16);
                    }
                    unlock(e);
                    index = u32::MAX;
                }
            }
            index = index.wrapping_add(1);
            if index >= extra_count(e, this) as u32 {
                break;
            }
        }
    }
}

// Translated from 00a5c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the `NiTArray<unsigned short,
/// NiTMallocInterface<unsigned short>>` (vtable `0109b23c`) that `IsEqual`'s
/// type uses (the engine map has no name): stores the vtable, frees the
/// element block at +4 and, if bit 0 of `flags` is set, gives the object
/// itself back to the `MemoryManager`. Answers `this`.
pub fn fn_00a5c780(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), USHORT_ARRAY_VTABLE);
    let block = e.mem.u32(this.addr() + 4);
    free(e, block);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a5c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET`'s scalar deleting destructor (the engine map has no name):
/// the same steps as the destructor body `00a5c240`, then, if bit 0 of
/// `flags` is set, `operator delete(this, 0x18)`. Answers `this`.
pub fn fn_00a5c7b0(e: &mut Engine, this: Ptr<NiObjectNET>, flags: u8) -> Ptr<NiObjectNET> {
    fn_00a5c240(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, 0x18u32]);
    }
    this
}

/// Builds an empty `NiTPrimitiveArray<unsigned short>` local as `IsEqual`
/// does: its vtable, no storage, grow-by 1.
fn init_index_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, USHORT_PRIMITIVE_ARRAY_VTABLE);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u16(array + 8, 0);
    e.mem.set_u16(array + 0x0a, 0);
    e.mem.set_u16(array + 0x0c, 0);
    e.mem.set_u16(array + 0x0e, 1);
}

/// Appends to `array` the index of every extra data entry of `object` that
/// answers true to slot 0x8c (growing the storage by `grow-by` when the array
/// is full); `index_slot` is the stack word the game keeps the running index
/// in and passes by address to the element store.
fn collect_streamed_indices(e: &mut Engine, object: Ptr<NiObjectNET>, array: u32, index_slot: u32) {
    let mut index: u32 = 0;
    e.mem.set_u32(index_slot, 0);
    if extra_count(e, object) != 0 {
        loop {
            let extra = extra_at(e, object, index as u16 as u32);
            if e.vcall(extra, SLOT_EXTRA_STREAMABLE, &args![]).bool() {
                let size = e.mem.u16(array + 0x0a) as u32;
                let capacity = e.mem.u16(array + 8) as u32;
                if size >= capacity {
                    let grow_by = e.mem.u16(array + 0x0e) as u32;
                    e.call(USHORT_ARRAY_GROW, &args![array, grow_by + size]);
                }
                fn_00a5b870(e, Ptr::new(array), size, Ptr::new(index_slot));
            }
            index = index.wrapping_add(1);
            e.mem.set_u32(index_slot, index);
            if (index as u16) >= extra_count(e, object) {
                break;
            }
        }
    }
}

// Translated from 00a5c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::IsEqual` (Xbox PDB): false unless the `NiObject` part says
/// equal and the two names are the same handle. Then each object's list of
/// extra data entries that answer true to slot 0x8c is collected as indices
/// into two local arrays (`0109b2d4`, built empty on the stack); the lists
/// must have the same length and the entries at equal places must be equal
/// (slot 0x5c). The controller chains must have the same length
/// (`00a6cd00`) and be pairwise equal (slot 0x5c). The two arrays' storage
/// is freed on every way out (the second array first).
pub fn ni_object_net_is_equal(
    e: &mut Engine,
    this: Ptr<NiObjectNET>,
    other: Ptr<NiObjectNET>,
) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiObjectNET::m_kName) != e.get(other, NiObjectNET::m_kName) {
        return false;
    }
    // Locals: array of `this` (+0), array of `other` (+0x10) and the stack
    // word holding the running index (+0x20).
    e.with_stack(0x24, |e, locals| {
        let mine = locals.addr();
        let theirs = locals.addr() + 0x10;
        let index_slot = locals.addr() + 0x20;
        init_index_array(e, mine);
        init_index_array(e, theirs);
        collect_streamed_indices(e, this, mine, index_slot);
        collect_streamed_indices(e, other, theirs, index_slot);
        let equal = streamed_lists_equal(e, this, other, mine, theirs);
        let storage = e.mem.u32(theirs + 4);
        free(e, storage);
        let storage = e.mem.u32(mine + 4);
        free(e, storage);
        equal
    })
}

/// The comparison part of `IsEqual` once the index lists are collected.
fn streamed_lists_equal(
    e: &mut Engine,
    this: Ptr<NiObjectNET>,
    other: Ptr<NiObjectNET>,
    mine: u32,
    theirs: u32,
) -> bool {
    if e.mem.u16(mine + 0x0a) != e.mem.u16(theirs + 0x0a) {
        return false;
    }
    let mut at: u16 = 0;
    if at < e.mem.u16(mine + 0x0a) {
        loop {
            let my_indices = e.mem.u32(mine + 4);
            let their_indices = e.mem.u32(theirs + 4);
            let mine_index = e.mem.u16(my_indices.wrapping_add(at as u32 * 2)) as u32;
            let their_index = e.mem.u16(their_indices.wrapping_add(at as u32 * 2)) as u32;
            let mine_extra = extra_at(e, this, mine_index);
            let their_extra = extra_at(e, other, their_index);
            if !e
                .vcall(mine_extra, SLOT_IS_EQUAL, &args![their_extra])
                .bool()
            {
                return false;
            }
            at = at.wrapping_add(1);
            if at >= e.mem.u16(mine + 0x0a) {
                break;
            }
        }
    }
    let mut mine_controller = e.get(this, NiObjectNET::m_spControllers).addr();
    let mut their_controller = e.get(other, NiObjectNET::m_spControllers).addr();
    let mine_length = if mine_controller != 0 {
        e.call(CONTROLLER_CHAIN_LENGTH, &args![mine_controller])
            .u32()
    } else {
        0
    };
    let their_length = if their_controller != 0 {
        e.call(CONTROLLER_CHAIN_LENGTH, &args![their_controller])
            .u32()
    } else {
        0
    };
    if mine_length != their_length {
        return false;
    }
    while mine_controller != 0 && their_controller != 0 {
        if !e
            .vcall(mine_controller, SLOT_IS_EQUAL, &args![their_controller])
            .bool()
        {
            return false;
        }
        mine_controller = e.mem.u32(mine_controller + CONTROLLER_NEXT);
        their_controller = e.mem.u32(their_controller + CONTROLLER_NEXT);
    }
    true
}

/// Appends `text` to the viewer strings array (a string array: capacity at
/// +8, size at +0xA, grow-by at +0xE): grows it when it is full, then stores
/// the pointer at the old size through the element store.
fn append_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        let size = e.mem.u16(strings.addr() + 0x0a) as u32;
        let capacity = e.mem.u16(strings.addr() + 8) as u32;
        if size >= capacity {
            let grow_by = e.mem.u16(strings.addr() + 0x0e) as u32;
            e.call(STRINGS_GROW, &args![strings, grow_by + size]);
        }
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a5ca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObjectNET::GetViewerStrings` (Xbox PDB): the `NiObject` part, then the
/// class name (the first word of the `NiRTTI` at `011f4304`, through
/// `NiGetViewerString`) and the pair `"m_kName"` / the name handle (through
/// `NiGetViewerString_ov11`) are appended to the array.
pub fn ni_object_net_get_viewer_strings(e: &mut Engine, this: Ptr<NiObjectNET>, strings: Ptr) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_OBJECT_NET_RTTI);
    let line = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    append_viewer_string(e, strings, line);
    let name = e.get(this, NiObjectNET::m_kName);
    let line = e
        .call(NI_GET_VIEWER_STRING_PAIR, &args![NAME_FIELD_TEXT, name])
        .u32();
    append_viewer_string(e, strings, line);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5b870, fn_00a5b870(Ptr<NiTArray>, u32, Ptr)),
        entry!(
            0x00a5b8f0,
            ni_object_net_process_clone(Ptr<NiObjectNET>, Ptr)
        ),
        entry!(0x00a5b950, fn_00a5b950(Ptr<NiObjectNET>, Ptr)),
        entry!(
            0x00a5b990,
            ni_object_net_remove_extra_data_at(Ptr<NiObjectNET>, u16)
        ),
        entry!(0x00a5ba40, fn_00a5ba40(Ptr<NiObjectNET>, Ptr) -> bool),
        entry!(
            0x00a5bba0,
            ni_object_net_set_extra_data_size(Ptr<NiObjectNET>, u16) -> bool
        ),
        entry!(
            0x00a5bc40,
            ni_object_net_add_extra_data(Ptr<NiObjectNET>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5bca0,
            ni_object_net_add_extra_data_ov2(Ptr<NiObjectNET>, Ptr) -> bool
        ),
        entry!(
            0x00a5bdd0,
            ni_object_net_get_extra_data(Ptr<NiObjectNET>, Ptr) -> Ptr
        ),
        entry!(
            0x00a5be90,
            ni_object_net_remove_extra_data(Ptr<NiObjectNET>, Ptr) -> bool
        ),
        entry!(0x00a5bf50, fn_00a5bf50(Ptr<NiObjectNET>, u16) -> bool),
        entry!(0x00a5bfa0, fn_00a5bfa0(Ptr<NiObjectNET>)),
        entry!(0x00a5c000, fn_00a5c000(Ptr<NiObjectNET>, Ptr)),
        entry!(
            0x00a5c050,
            ni_object_net_remove_all_controllers(Ptr<NiObjectNET>)
        ),
        entry!(0x00a5c080, ni_object_net_load_binary(Ptr<NiObjectNET>, Ptr)),
        entry!(0x00a5c0e0, ni_object_net_link_object(Ptr<NiObjectNET>, Ptr)),
        entry!(
            0x00a5c180,
            ni_object_net_register_streamables(Ptr<NiObjectNET>, Ptr) -> bool
        ),
        entry!(
            0x00a5c200,
            fn_00a5c200(Ptr<NiObjectNET>) -> Ptr<NiObjectNET>
        ),
        entry!(0x00a5c230, ni_object_net_get_rtti() -> Ptr),
        entry!(0x00a5c240, fn_00a5c240(Ptr<NiObjectNET>)),
        entry!(
            0x00a5c290,
            fn_00a5c290(Ptr<NiObjectNET>, Ptr<NiObjectNET>, Ptr)
        ),
        entry!(0x00a5c400, fn_00a5c400(Ptr<NiObjectNET>, Ptr)),
        entry!(
            0x00a5c480,
            ni_object_net_remove_controller(Ptr<NiObjectNET>, Ptr)
        ),
        entry!(
            0x00a5c570,
            ni_object_net_get_controller(Ptr<NiObjectNET>, Ptr) -> Ptr
        ),
        entry!(0x00a5c5b0, ni_object_net_save_binary(Ptr<NiObjectNET>, Ptr)),
        entry!(
            0x00a5c6a0,
            ni_object_net_post_link_object(Ptr<NiObjectNET>, Ptr)
        ),
        entry!(0x00a5c780, fn_00a5c780(Ptr, u8) -> Ptr),
        entry!(
            0x00a5c7b0,
            fn_00a5c7b0(Ptr<NiObjectNET>, u8) -> Ptr<NiObjectNET>
        ),
        entry!(
            0x00a5c820,
            ni_object_net_is_equal(Ptr<NiObjectNET>, Ptr<NiObjectNET>) -> bool
        ),
        entry!(
            0x00a5ca80,
            ni_object_net_get_viewer_strings(Ptr<NiObjectNET>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The empty string's handle, and some pooled-string handles that are only
    /// compared (never dereferenced) in the tests that use them.
    const EMPTY: u32 = 0x5000_0000;
    const NAME_A: u32 = 0x5000_0100;
    const NAME_B: u32 = 0x5000_0200;
    const NAME_C: u32 = 0x5000_0300;
    const NAME_D: u32 = 0x5000_0400;
    const NAME_E: u32 = 0x5000_0500;
    /// Targets of the fake vtable slots.
    const DELETE_DOUBLE: u32 = 0x7000_0004;
    const TRUE_DOUBLE: u32 = 0x7000_0100;
    const FALSE_DOUBLE: u32 = 0x7000_0101;
    const LOG_DOUBLE: u32 = 0x7000_0102;
    const EQUAL_DOUBLE: u32 = 0x7000_0103;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the platform pieces this unit calls stood in for, the
    /// empty-string handle in place and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, EMPTY);
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
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            ret(a[0])
        });
        e.register(NAME_ADDRESS, |_, a| ret(a[0] + 8));
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(TRUE_DOUBLE, |_, _| ret(1));
        e.register(FALSE_DOUBLE, |_, _| ret(0));
        e.register(LOG_DOUBLE, |_, _| Ret::default());
        e.register(EQUAL_DOUBLE, |e, a| {
            ret((e.mem.u32(a[0] + 8) == e.mem.u32(a[1] + 8)) as u32)
        });
        e.call_log = Some(vec![]);
        e
    }

    fn plain(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// The arguments of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// An object with its own vtable (slot 4 is `DeleteThis`, a double that
    /// only logs), reference count 1 and the given word at +8; `slots` fill
    /// the rest of the table.
    fn object(e: &mut Engine, word_at_8: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + SLOT_DELETE_THIS, DELETE_DOUBLE);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 4, 1);
        e.mem.set_u32(object + 8, word_at_8);
        object
    }

    fn net(e: &mut Engine) -> Ptr<NiObjectNET> {
        e.new_object::<NiObjectNET>()
    }

    fn set_extras(e: &mut Engine, this: Ptr<NiObjectNET>, list: &[u32], capacity: u16) {
        let block = e.mem.alloc(capacity as u32 * 4);
        for (i, extra) in list.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *extra);
        }
        e.set(this, NiObjectNET::m_ppkExtra, Ptr::new(block));
        e.set(this, NiObjectNET::m_usExtraDataSize, list.len() as u16);
        e.set(this, NiObjectNET::m_usMaxSize, capacity);
    }

    /// The first `count` words of the extra data array.
    fn extras(e: &Engine, this: Ptr<NiObjectNET>, count: u32) -> Vec<u32> {
        let base = e.get(this, NiObjectNET::m_ppkExtra).addr();
        (0..count).map(|i| e.mem.u32(base + 4 * i)).collect()
    }

    fn count_of(e: &Engine, object: u32) -> u32 {
        e.mem.u32(object + 4)
    }

    /// A pooled string: reference count 1 eight bytes before the characters.
    fn pooled(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(8 + text.len() as u32 + 1);
        e.mem.set_u32(block, 1);
        e.mem.set_cstr(block + 8, text.as_bytes());
        block + 8
    }

    fn handle_slot(e: &mut Engine, handle: u32) -> Ptr {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, handle);
        Ptr::new(slot)
    }

    /// A controller: reference count 1, next controller at +0x30.
    fn controller(e: &mut Engine, next: u32, slots: &[(u32, u32)]) -> u32 {
        let controller = object(e, 0, slots);
        e.mem.set_u32(controller + CONTROLLER_NEXT, next);
        controller
    }

    // -- 00a5b870

    fn word_array(e: &mut Engine, capacity: u16) -> Ptr<NiTArray> {
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(capacity as u32 * 2);
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, capacity);
        array
    }

    fn store_word(e: &mut Engine, array: Ptr<NiTArray>, index: u32, value: u16) {
        let slot = e.mem.alloc(4);
        e.mem.set_u16(slot, value);
        fn_00a5b870(e, array, index, Ptr::new(slot));
    }

    fn array_state(e: &Engine, array: Ptr<NiTArray>) -> (u16, u16) {
        (
            e.get(array, NiTArray::m_usSize),
            e.get(array, NiTArray::m_usESize),
        )
    }

    #[test]
    fn element_store_tracks_size_and_non_zero_count() {
        let mut e = engine();
        let array = word_array(&mut e, 8);
        let base = e.get(array, NiTArray::m_pBase);
        // Past the end with a value: size follows, one non-zero entry.
        store_word(&mut e, array, 3, 7);
        assert_eq!(array_state(&e, array), (4, 1));
        assert_eq!(e.mem.u16(base + 6), 7);
        // Non-zero over non-zero: the count stays.
        store_word(&mut e, array, 3, 9);
        assert_eq!(array_state(&e, array), (4, 1));
        assert_eq!(e.mem.u16(base + 6), 9);
        // Zero over non-zero: one fewer.
        store_word(&mut e, array, 3, 0);
        assert_eq!(array_state(&e, array), (4, 0));
        // Non-zero over zero inside the size: one more.
        store_word(&mut e, array, 1, 5);
        assert_eq!(array_state(&e, array), (4, 1));
        // Zero over zero: unchanged.
        store_word(&mut e, array, 2, 0);
        assert_eq!(array_state(&e, array), (4, 1));
        // Past the end with zero: the size grows, the count does not.
        store_word(&mut e, array, 6, 0);
        assert_eq!(array_state(&e, array), (7, 1));
    }

    // -- 00a5b8f0

    #[test]
    fn process_clone_visits_entries_and_the_first_controller() {
        let mut e = engine();
        plain(&mut e, &[NI_OBJECT_PROCESS_CLONE]);
        let slots = [(SLOT_PROCESS_CLONE, LOG_DOUBLE)];
        let a = object(&mut e, NAME_A, &slots);
        let b = object(&mut e, NAME_B, &slots);
        let c = controller(&mut e, 0, &slots);
        let n = net(&mut e);
        set_extras(&mut e, n, &[a, 0, b], 4);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(c));
        e.call(0x00a5_b8f0, &args![n, 0x1234u32]);
        assert_eq!(
            calls(&e, NI_OBJECT_PROCESS_CLONE),
            vec![vec![n.addr(), 0x1234]]
        );
        assert_eq!(
            calls(&e, LOG_DOUBLE),
            vec![vec![a, 0x1234], vec![b, 0x1234], vec![c, 0x1234]]
        );
        // Nothing to visit: only the base class runs.
        let mut e = engine();
        plain(&mut e, &[NI_OBJECT_PROCESS_CLONE]);
        let n = net(&mut e);
        ni_object_net_process_clone(&mut e, n, Ptr::new(1));
        assert!(calls(&e, LOG_DOUBLE).is_empty());
    }

    // -- 00a5b950

    #[test]
    fn name_store_counts_the_new_string_and_releases_the_old() {
        let mut e = engine();
        let old = pooled(&mut e, "old");
        let new = pooled(&mut e, "new");
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_kName, old);
        let slot = handle_slot(&mut e, new);
        e.call(0x00a5_b950, &args![n, slot]);
        assert_eq!(e.get(n, NiObjectNET::m_kName), new);
        assert_eq!(e.mem.u32(new - 8), 2);
        assert_eq!(e.mem.u32(old - 8), 0);
        // The same handle again changes nothing.
        fn_00a5b950(&mut e, n, slot);
        assert_eq!(e.mem.u32(new - 8), 2);
        // The empty string is stored without touching any count (its count
        // word is not mapped, so touching it would panic).
        let slot = handle_slot(&mut e, EMPTY);
        fn_00a5b950(&mut e, n, slot);
        assert_eq!(e.get(n, NiObjectNET::m_kName), EMPTY);
        assert_eq!(e.mem.u32(new - 8), 1);
    }

    // -- 00a5b990

    #[test]
    fn remove_at_releases_shifts_and_clears() {
        let mut e = engine();
        let a = object(&mut e, NAME_A, &[]);
        let b = object(&mut e, NAME_B, &[]);
        let c = object(&mut e, NAME_C, &[]);
        let n = net(&mut e);
        set_extras(&mut e, n, &[a, b, c], 4);
        e.call(0x00a5_b990, &args![n, 1u32]);
        assert_eq!(extras(&e, n, 3), vec![a, c, 0]);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 2);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![b]]);
        // An entry that is still referenced elsewhere is not deleted.
        e.mem.set_u32(c + 4, 2);
        ni_object_net_remove_extra_data_at(&mut e, n, 1);
        assert_eq!(extras(&e, n, 2), vec![a, 0]);
        assert_eq!(count_of(&e, c), 1);
        assert_eq!(calls(&e, DELETE_DOUBLE).len(), 1);
        // Out of range: nothing happens (but the lock is taken).
        ni_object_net_remove_extra_data_at(&mut e, n, 5);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 1);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 3);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 3);
    }

    // -- 00a5ba40

    #[test]
    fn sorted_add_creates_grows_orders_and_rejects_duplicates() {
        let mut e = engine();
        let n = net(&mut e);
        let a = object(&mut e, NAME_A, &[]);
        let b = object(&mut e, NAME_B, &[]);
        let c = object(&mut e, NAME_C, &[]);
        // Null: false, nothing locked.
        assert!(!fn_00a5ba40(&mut e, n, Ptr::NULL));
        assert!(calls(&e, ENTER_CRITICAL_SECTION).is_empty());
        // First entry: a one-slot array.
        assert!(e.call(0x00a5_ba40, &args![n, b]).bool());
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 1);
        assert_eq!(extras(&e, n, 1), vec![b]);
        assert_eq!(count_of(&e, b), 2);
        // Full: capacity becomes 3; the smaller name moves in front.
        assert!(fn_00a5ba40(&mut e, n, Ptr::new(a)));
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 3);
        assert_eq!(extras(&e, n, 3), vec![a, b, 0]);
        assert_eq!(calls(&e, MEMCPY).len(), 1);
        assert!(fn_00a5ba40(&mut e, n, Ptr::new(c)));
        assert_eq!(extras(&e, n, 3), vec![a, b, c]);
        // A second object of an existing name is taken out again.
        let duplicate = object(&mut e, NAME_B, &[]);
        assert!(!fn_00a5ba40(&mut e, n, Ptr::new(duplicate)));
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 3);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 7);
        assert_eq!(extras(&e, n, 7), vec![a, b, c, 0, 0, 0, 0]);
        assert_eq!(count_of(&e, duplicate), 1);
        assert!(calls(&e, DELETE_DOUBLE).is_empty());
    }

    // -- 00a5bba0

    #[test]
    fn set_size_allocates_then_copies() {
        let mut e = engine();
        let n = net(&mut e);
        assert!(!ni_object_net_set_extra_data_size(&mut e, n, 0));
        assert!(calls(&e, ENTER_CRITICAL_SECTION).is_empty());
        assert!(e.call(0x00a5_bba0, &args![n, 4u32]).bool());
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 4);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 0);
        let first = e.get(n, NiObjectNET::m_ppkExtra).addr();
        assert_eq!(e.mem.block_size(first), Some(16));
        // Two entries, then a bigger array: copied, old block freed.
        e.mem.set_u32(first, 0x11);
        e.mem.set_u32(first + 4, 0x22);
        e.set(n, NiObjectNET::m_usExtraDataSize, 2);
        assert!(ni_object_net_set_extra_data_size(&mut e, n, 6));
        let second = e.get(n, NiObjectNET::m_ppkExtra).addr();
        assert_ne!(first, second);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.mem.block_size(second), Some(24));
        assert_eq!(extras(&e, n, 2), vec![0x11, 0x22]);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 6);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 2);
    }

    // -- 00a5bc40

    #[test]
    fn add_with_name_branches() {
        let mut e = engine();
        let name = pooled(&mut e, "wanted");
        let slot = handle_slot(&mut e, name);
        let empty_slot = handle_slot(&mut e, EMPTY);
        let n = net(&mut e);
        let unnamed = object(&mut e, EMPTY, &[]);
        // The empty name and a null object answer false.
        assert!(!ni_object_net_add_extra_data(
            &mut e,
            n,
            empty_slot,
            Ptr::new(unnamed)
        ));
        assert!(!ni_object_net_add_extra_data(&mut e, n, slot, Ptr::NULL));
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 0);
        // An unnamed object takes the name and is added.
        assert!(e.call(0x00a5_bc40, &args![n, slot, unnamed]).bool());
        assert_eq!(e.mem.u32(unnamed + 8), name);
        assert_eq!(e.mem.u32(name - 8), 2);
        assert_eq!(extras(&e, n, 1), vec![unnamed]);
        // An object with another name is refused, one with this name added.
        let other = object(&mut e, NAME_A, &[]);
        let n2 = net(&mut e);
        assert!(!ni_object_net_add_extra_data(
            &mut e,
            n2,
            slot,
            Ptr::new(other)
        ));
        let same = object(&mut e, name, &[]);
        assert!(ni_object_net_add_extra_data(
            &mut e,
            n2,
            slot,
            Ptr::new(same)
        ));
        assert_eq!(extras(&e, n2, 1), vec![same]);
    }

    // -- 00a5bca0

    fn rtti_for(e: &mut Engine, class_name: Option<&str>) -> u32 {
        let rtti = e.mem.alloc(8);
        if let Some(text) = class_name {
            let characters = e.mem.alloc(text.len() as u32 + 1);
            e.mem.set_cstr(characters, text.as_bytes());
            e.mem.set_u32(rtti, characters);
        }
        rtti
    }

    /// An extra data object whose `GetRTTI` (slot 8) answers a record naming
    /// `class_name`; `double` is a free address for that slot's target.
    fn extra_of_class(e: &mut Engine, name: u32, class_name: Option<&str>, double: u32) -> u32 {
        let rtti = rtti_for(e, class_name);
        e.register_double(double, move |_, _| ret(rtti));
        object(e, name, &[(SLOT_GET_RTTI, double)])
    }

    /// Stand-ins for the formatting and string functions `AddExtraData_ov2`
    /// builds a name with, and the unmapped word a null old name makes
    /// `SetName` decrement (see `fn_00a5b950`).
    fn naming_doubles(e: &mut Engine) {
        e.map(0xffff_f000, 0x1000);
        // The text `strstr` looks for (the exe's `.rdata` string).
        e.map(EXTRA_DATA_TEXT, 16);
        e.mem.set_cstr(EXTRA_DATA_TEXT, b"ExtraData");
        e.register(NI_SPRINTF, |e, a| {
            let text = format!("ED{:03}", a[3]);
            e.mem.set_cstr(a[0], text.as_bytes());
            ret(text.len() as u32)
        });
        e.register(STRSTR, |e, a| {
            let haystack = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            match haystack
                .windows(needle.len())
                .position(|part| part == needle.as_slice())
            {
                Some(at) => ret(a[0] + at as u32),
                None => ret(0),
            }
        });
        e.register(STRNCPY_S, |e, a| {
            let text = e.mem.bytes(a[2], a[3]);
            e.mem.set_cstr(a[0], &text);
            ret(a[0])
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            ret(a[0])
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            let text = e.mem.cstr(a[1]);
            let block = e.mem.alloc(8 + text.len() as u32 + 1);
            e.mem.set_u32(block, 1);
            e.mem.set_cstr(block + 8, &text);
            e.mem.set_u32(a[0], block + 8);
            ret(a[0])
        });
    }

    #[test]
    fn add_without_a_name_makes_one_from_the_class_name() {
        let mut e = engine();
        naming_doubles(&mut e);
        let n = net(&mut e);
        assert!(!ni_object_net_add_extra_data_ov2(&mut e, n, Ptr::NULL));
        // The class name has "ExtraData" after a prefix: the prefix is kept.
        let x1 = extra_of_class(&mut e, 0, Some("NiFooExtraData"), 0x7000_0200);
        assert!(e.call(0x00a5_bca0, &args![n, x1]).bool());
        let name = e.mem.u32(x1 + 8);
        assert_eq!(e.mem.cstr(name), b"NiFooED000");
        assert_eq!(e.mem.u32(name - 8), 1);
        // "ExtraData" at the start, or not present: only the counter text.
        let x2 = extra_of_class(&mut e, 0, Some("ExtraDataBar"), 0x7000_0201);
        assert!(ni_object_net_add_extra_data_ov2(&mut e, n, Ptr::new(x2)));
        assert_eq!(e.mem.cstr(e.mem.u32(x2 + 8)), b"ED001");
        let x3 = extra_of_class(&mut e, 0, Some("Plain"), 0x7000_0202);
        assert!(ni_object_net_add_extra_data_ov2(&mut e, n, Ptr::new(x3)));
        assert_eq!(e.mem.cstr(e.mem.u32(x3 + 8)), b"ED002");
        // A name that is already set, and a class without a name, are not
        // touched.
        let x4 = extra_of_class(&mut e, NAME_D, Some("Named"), 0x7000_0203);
        assert!(ni_object_net_add_extra_data_ov2(&mut e, n, Ptr::new(x4)));
        assert_eq!(e.mem.u32(x4 + 8), NAME_D);
        let x5 = extra_of_class(&mut e, 0, None, 0x7000_0204);
        assert!(ni_object_net_add_extra_data_ov2(&mut e, n, Ptr::new(x5)));
        assert_eq!(e.mem.u32(x5 + 8), 0);
        assert_eq!(calls(&e, NI_SPRINTF).len(), 3);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 5);
    }

    // -- 00a5bdd0 / 00a5be90

    fn five_named(e: &mut Engine) -> (Ptr<NiObjectNET>, Vec<u32>) {
        let names = [NAME_A, NAME_B, NAME_C, NAME_D, NAME_E];
        let list: Vec<u32> = names.iter().map(|name| object(e, *name, &[])).collect();
        let n = net(e);
        set_extras(e, n, &list, 5);
        (n, list)
    }

    #[test]
    fn get_finds_every_entry_by_binary_search() {
        let mut e = engine();
        let (n, list) = five_named(&mut e);
        for (name, object) in [NAME_A, NAME_B, NAME_C, NAME_D, NAME_E].iter().zip(&list) {
            let slot = handle_slot(&mut e, *name);
            assert_eq!(
                ni_object_net_get_extra_data(&mut e, n, slot).addr(),
                *object
            );
        }
        let slot = handle_slot(&mut e, 0x5000_0250);
        assert!(e.call(0x00a5_bdd0, &args![n, slot]).ptr::<()>().is_null());
        // The empty name answers null without the lock.
        let locks = calls(&e, ENTER_CRITICAL_SECTION).len();
        let slot = handle_slot(&mut e, EMPTY);
        assert!(ni_object_net_get_extra_data(&mut e, n, slot).is_null());
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), locks);
        // No entries: not found.
        let empty = net(&mut e);
        let slot = handle_slot(&mut e, NAME_A);
        assert!(ni_object_net_get_extra_data(&mut e, empty, slot).is_null());
    }

    #[test]
    fn remove_by_name_removes_the_matching_entry() {
        let mut e = engine();
        let (n, list) = five_named(&mut e);
        let slot = handle_slot(&mut e, NAME_C);
        assert!(e.call(0x00a5_be90, &args![n, slot]).bool());
        assert_eq!(
            extras(&e, n, 5),
            vec![list[0], list[1], list[3], list[4], 0]
        );
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![list[2]]]);
        // Gone now.
        assert!(!ni_object_net_remove_extra_data(&mut e, n, slot));
        // The empty name: false, the lock was taken and given back.
        let locks = calls(&e, ENTER_CRITICAL_SECTION).len();
        let slot = handle_slot(&mut e, EMPTY);
        assert!(!ni_object_net_remove_extra_data(&mut e, n, slot));
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), locks + 1);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), locks + 1);
        // No entries: false without the lock.
        let empty = net(&mut e);
        let locks = calls(&e, ENTER_CRITICAL_SECTION).len();
        assert!(!ni_object_net_remove_extra_data(&mut e, empty, slot));
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), locks);
    }

    // -- 00a5bf50 / 00a5bfa0

    #[test]
    fn checked_remove_by_index() {
        let mut e = engine();
        let (n, list) = five_named(&mut e);
        assert!(e.call(0x00a5_bf50, &args![n, 0u32]).bool());
        assert_eq!(extras(&e, n, 2), vec![list[1], list[2]]);
        assert!(!fn_00a5bf50(&mut e, n, 4));
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 4);
    }

    #[test]
    fn remove_all_goes_from_the_last_and_frees_the_array() {
        let mut e = engine();
        let a = object(&mut e, NAME_A, &[]);
        let b = object(&mut e, NAME_B, &[]);
        let c = object(&mut e, NAME_C, &[]);
        e.mem.set_u32(a + 4, 2);
        let n = net(&mut e);
        set_extras(&mut e, n, &[a, b, c], 4);
        let block = e.get(n, NiObjectNET::m_ppkExtra).addr();
        e.call(0x00a5_bfa0, &args![n]);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![c], vec![b]]);
        assert_eq!(count_of(&e, a), 1);
        assert_eq!(e.mem.block_size(block), None);
        assert!(e.get(n, NiObjectNET::m_ppkExtra).is_null());
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 0);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 0);
        // With no entries it frees nothing and clears again.
        fn_00a5bfa0(&mut e, n);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 0);
    }

    // -- 00a5c000 / 00a5c050 / 00a5c400

    #[test]
    fn first_controller_assignment() {
        let mut e = engine();
        let old = controller(&mut e, 0, &[]);
        let new = controller(&mut e, 0, &[]);
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(old));
        e.call(0x00a5_c000, &args![n, new]);
        assert_eq!(e.get(n, NiObjectNET::m_spControllers).addr(), new);
        assert_eq!(count_of(&e, new), 2);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![old]]);
        // The same controller again: nothing changes.
        fn_00a5c000(&mut e, n, Ptr::new(new));
        assert_eq!(count_of(&e, new), 2);
        // Null releases and clears.
        fn_00a5c000(&mut e, n, Ptr::NULL);
        assert!(e.get(n, NiObjectNET::m_spControllers).is_null());
        assert_eq!(count_of(&e, new), 1);
    }

    #[test]
    fn remove_all_controllers_releases_the_head() {
        let mut e = engine();
        let c = controller(&mut e, 0, &[]);
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(c));
        e.call(0x00a5_c050, &args![n]);
        assert!(e.get(n, NiObjectNET::m_spControllers).is_null());
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![c]]);
        // None: nothing to do.
        ni_object_net_remove_all_controllers(&mut e, n);
        assert_eq!(calls(&e, DELETE_DOUBLE).len(), 1);
    }

    #[test]
    fn controller_goes_to_the_head_of_the_chain() {
        let mut e = engine();
        let first = controller(&mut e, 0, &[]);
        let second = controller(&mut e, 0, &[]);
        let n = net(&mut e);
        // Null does nothing.
        fn_00a5c400(&mut e, n, Ptr::NULL);
        assert!(e.get(n, NiObjectNET::m_spControllers).is_null());
        e.call(0x00a5_c400, &args![n, first]);
        assert_eq!(e.get(n, NiObjectNET::m_spControllers).addr(), first);
        assert_eq!(count_of(&e, first), 2);
        // The new head points to the old one: the old head is counted by the
        // new head's next pointer and no longer by the object.
        fn_00a5c400(&mut e, n, Ptr::new(second));
        assert_eq!(e.get(n, NiObjectNET::m_spControllers).addr(), second);
        assert_eq!(e.mem.u32(second + CONTROLLER_NEXT), first);
        assert_eq!(count_of(&e, first), 2);
        assert_eq!(count_of(&e, second), 2);
        assert!(calls(&e, DELETE_DOUBLE).is_empty());
    }

    // -- 00a5c080

    fn stream(e: &mut Engine, version: u32) -> Ptr {
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        Ptr::new(stream)
    }

    fn stream_doubles(e: &mut Engine) {
        plain(
            e,
            &[
                NI_OBJECT_LOAD_BINARY,
                STREAM_LOAD_FIXED_STRING,
                STREAM_LOAD_C_STRING_AS_FIXED_STRING,
                STREAM_READ_MULTIPLE_LINK_IDS,
                STREAM_READ_LINK_ID,
                LINK_STUB,
            ],
        );
    }

    #[test]
    fn load_binary_follows_the_stream_version() {
        for (version, old_text, reads_multiple) in [
            (0x0200_0000u32, true, false),
            (0x0a00_0000, true, true),
            (0x1401_0001, false, true),
        ] {
            let mut e = engine();
            stream_doubles(&mut e);
            let n = net(&mut e);
            let s = stream(&mut e, version);
            e.call(0x00a5_c080, &args![n, s]);
            let old = calls(&e, STREAM_LOAD_C_STRING_AS_FIXED_STRING);
            let new = calls(&e, STREAM_LOAD_FIXED_STRING);
            if old_text {
                assert_eq!(old, vec![vec![s.addr(), n.addr() + 8]]);
                assert!(new.is_empty());
            } else {
                assert_eq!(new, vec![vec![s.addr(), n.addr() + 8]]);
                assert!(old.is_empty());
            }
            assert_eq!(
                calls(&e, NI_OBJECT_LOAD_BINARY),
                vec![vec![n.addr(), s.addr()]]
            );
            assert_eq!(
                calls(&e, STREAM_READ_MULTIPLE_LINK_IDS).len(),
                reads_multiple as usize
            );
            assert_eq!(
                calls(&e, STREAM_READ_LINK_ID).len(),
                if reads_multiple { 1 } else { 2 }
            );
        }
    }

    // -- 00a5c0e0

    #[test]
    fn link_object_old_and_new_formats() {
        // New format: two links become entries, then the controller.
        let mut e = engine();
        stream_doubles(&mut e);
        let a = object(&mut e, NAME_A, &[]);
        let b = object(&mut e, NAME_B, &[]);
        let c = controller(&mut e, 0, &[]);
        let mut ids = vec![a, b, c];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(ids.remove(0))
        });
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| ret(2));
        let n = net(&mut e);
        let s = stream(&mut e, 0x1401_0001);
        e.call(0x00a5_c0e0, &args![n, s]);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 2);
        assert_eq!(extras(&e, n, 2), vec![a, b]);
        assert_eq!(e.get(n, NiObjectNET::m_spControllers).addr(), c);
        assert_eq!(count_of(&e, c), 2);
        // Old format: the first link is parked at +0x10, then the controller.
        let mut e = engine();
        stream_doubles(&mut e);
        let mut ids = vec![0x1234u32, 0];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(ids.remove(0))
        });
        let n = net(&mut e);
        let s = stream(&mut e, 0x0400_0000);
        ni_object_net_link_object(&mut e, n, s);
        assert_eq!(e.get(n, NiObjectNET::m_ppkExtra).addr(), 0x1234);
        assert!(e.get(n, NiObjectNET::m_spControllers).is_null());
        assert!(calls(&e, STREAM_GET_NUMBER_OF_LINK_IDS).is_empty());
    }

    // -- 00a5c180

    #[test]
    fn register_streamables_asks_before_registering() {
        let mut e = engine();
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| ret(1));
        let register = (SLOT_REGISTER_STREAMABLES, LOG_DOUBLE);
        let yes = object(
            &mut e,
            NAME_A,
            &[register, (SLOT_EXTRA_STREAMABLE, TRUE_DOUBLE)],
        );
        let no = object(
            &mut e,
            NAME_B,
            &[register, (SLOT_EXTRA_STREAMABLE, FALSE_DOUBLE)],
        );
        let c = controller(&mut e, 0, &[register]);
        let n = net(&mut e);
        set_extras(&mut e, n, &[yes, 0, no], 3);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(c));
        let accepting = object(
            &mut e,
            0,
            &[(SLOT_STREAM_REGISTER_FIXED_STRING, TRUE_DOUBLE)],
        );
        assert!(e.call(0x00a5_c180, &args![n, accepting]).bool());
        assert_eq!(
            calls(&e, LOG_DOUBLE),
            vec![vec![yes, accepting], vec![c, accepting]]
        );
        assert_eq!(calls(&e, TRUE_DOUBLE)[0], vec![accepting, n.addr() + 8]);
        // The name registration answers false: nothing else happens.
        let refusing = object(
            &mut e,
            0,
            &[(SLOT_STREAM_REGISTER_FIXED_STRING, FALSE_DOUBLE)],
        );
        let before = calls(&e, LOG_DOUBLE).len();
        assert!(!ni_object_net_register_streamables(
            &mut e,
            n,
            Ptr::new(refusing)
        ));
        assert_eq!(calls(&e, LOG_DOUBLE).len(), before);
        // The base class answers false.
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| ret(0));
        assert!(!ni_object_net_register_streamables(
            &mut e,
            n,
            Ptr::new(accepting)
        ));
    }

    // -- 00a5c200 / 00a5c230 / 00a5c240 / 00a5c7b0 / 00a5c780

    #[test]
    fn constructor_sets_the_empty_state() {
        let mut e = engine();
        plain(&mut e, &[NI_OBJECT_CONSTRUCT]);
        let n = net(&mut e);
        e.mem.write(n.addr() + 4, &[0xff; 0x14]);
        let result = e.call(0x00a5_c200, &args![n]).ptr::<NiObjectNET>();
        assert_eq!(result, n);
        assert_eq!(e.mem.u32(n.addr()), NI_OBJECT_NET_VTABLE);
        assert_eq!(e.get(n, NiObjectNET::m_kName), EMPTY);
        assert!(e.get(n, NiObjectNET::m_spControllers).is_null());
        assert!(e.get(n, NiObjectNET::m_ppkExtra).is_null());
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 0);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 0);
        assert_eq!(calls(&e, NI_OBJECT_CONSTRUCT), vec![vec![n.addr()]]);
    }

    #[test]
    fn rtti_is_the_class_record() {
        let mut e = engine();
        assert_eq!(e.call(0x00a5_c230, &args![]).u32(), 0x011f_4304);
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        plain(&mut e, &[NI_OBJECT_DESTRUCT]);
        let name = pooled(&mut e, "name");
        let a = object(&mut e, NAME_A, &[]);
        let c = controller(&mut e, 0, &[]);
        let n = net(&mut e);
        set_extras(&mut e, n, &[a], 1);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(c));
        e.set(n, NiObjectNET::m_kName, name);
        e.call(0x00a5_c240, &args![n]);
        assert_eq!(e.mem.u32(n.addr()), NI_OBJECT_NET_VTABLE);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![a], vec![c]]);
        assert_eq!(e.mem.u32(name - 8), 0);
        assert_eq!(calls(&e, NI_OBJECT_DESTRUCT), vec![vec![n.addr()]]);
        assert_eq!(e.get(n, NiObjectNET::m_usMaxSize), 0);
    }

    #[test]
    fn deleting_destructors_honour_the_flag() {
        let mut e = engine();
        plain(&mut e, &[NI_OBJECT_DESTRUCT, OPERATOR_DELETE_SIZED]);
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_kName, EMPTY);
        let result = e.call(0x00a5_c7b0, &args![n, 0u32]).ptr::<NiObjectNET>();
        assert_eq!(result, n);
        assert!(calls(&e, OPERATOR_DELETE_SIZED).is_empty());
        assert_eq!(calls(&e, NI_OBJECT_DESTRUCT).len(), 1);
        fn_00a5c7b0(&mut e, n, 1);
        assert_eq!(calls(&e, OPERATOR_DELETE_SIZED), vec![vec![n.addr(), 0x18]]);
    }

    #[test]
    fn array_deleting_destructor_frees_the_elements() {
        let mut e = engine();
        plain(&mut e, &[MEMORY_MANAGER_DEALLOCATE]);
        let array = e.mem.alloc(0x10);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(array + 4, elements);
        let result = e.call(0x00a5_c780, &args![array, 0u32]).u32();
        assert_eq!(result, array);
        assert_eq!(e.mem.u32(array), USHORT_ARRAY_VTABLE);
        assert_eq!(e.mem.block_size(elements), None);
        assert!(calls(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        // Bit 0 also gives the object back (the other bits are ignored).
        let elements = e.mem.alloc(8);
        e.mem.set_u32(array + 4, elements);
        fn_00a5c780(&mut e, Ptr::new(array), 3);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    // -- 00a5c290

    fn copy_doubles(e: &mut Engine) {
        plain(e, &[NI_OBJECT_COPY_MEMBERS]);
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(a[0])
        });
        e.register(FIXED_STRING_CREATE, |e, a| {
            let text = e.mem.cstr(a[0]);
            let block = e.mem.alloc(8 + text.len() as u32 + 1);
            e.mem.set_u32(block, 1);
            e.mem.set_cstr(block + 8, &text);
            ret(block + 8)
        });
    }

    fn cloning_process(e: &mut Engine, copy_type: u32, append: u8) -> Ptr {
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(process + PROCESS_COPY_TYPE, copy_type);
        e.mem.set_u8(process + PROCESS_APPEND_CHAR, append);
        Ptr::new(process)
    }

    #[test]
    fn copy_members_copy_types() {
        // Exact: the clone shares the name.
        let mut e = engine();
        copy_doubles(&mut e);
        let name = pooled(&mut e, "thing");
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_kName, name);
        let clone = net(&mut e);
        e.set(clone, NiObjectNET::m_kName, EMPTY);
        let process = cloning_process(&mut e, COPY_EXACT, b'#');
        e.call(0x00a5_c290, &args![n, clone, process]);
        assert_eq!(e.get(clone, NiObjectNET::m_kName), name);
        assert_eq!(e.mem.u32(name - 8), 2);
        assert_eq!(
            calls(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![n.addr(), clone.addr(), process.addr()]]
        );
        // Unique: a new name with the append character.
        let process = cloning_process(&mut e, COPY_UNIQUE, b'#');
        let clone2 = net(&mut e);
        e.set(clone2, NiObjectNET::m_kName, EMPTY);
        fn_00a5c290(&mut e, n, clone2, process);
        let made = e.get(clone2, NiObjectNET::m_kName);
        assert_eq!(e.mem.cstr(made), b"thing#");
        assert_eq!(e.mem.u32(made - 8), 1);
        assert_eq!(e.mem.u32(name - 8), 2);
        // Another copy type: the name is not copied.
        let process = cloning_process(&mut e, 3, b'#');
        let clone3 = net(&mut e);
        e.set(clone3, NiObjectNET::m_kName, EMPTY);
        fn_00a5c290(&mut e, n, clone3, process);
        assert_eq!(e.get(clone3, NiObjectNET::m_kName), EMPTY);
        // A null name is not copied either.
        let nameless = net(&mut e);
        let process = cloning_process(&mut e, COPY_UNIQUE, b'#');
        let clone4 = net(&mut e);
        e.set(clone4, NiObjectNET::m_kName, EMPTY);
        fn_00a5c290(&mut e, nameless, clone4, process);
        assert_eq!(e.get(clone4, NiObjectNET::m_kName), EMPTY);
    }

    #[test]
    fn copy_members_clones_entries_and_controllers() {
        let mut e = engine();
        copy_doubles(&mut e);
        let copy = object(&mut e, NAME_A, &[]);
        e.register_double(0x7000_0300, move |_, _| ret(copy));
        let copy_controller = controller(&mut e, 0, &[]);
        e.register_double(0x7000_0301, move |_, _| ret(copy_controller));
        let cloneable = object(
            &mut e,
            NAME_B,
            &[
                (SLOT_EXTRA_CLONEABLE, TRUE_DOUBLE),
                (SLOT_CREATE_CLONE, 0x7000_0300),
            ],
        );
        let skipped = object(&mut e, NAME_C, &[(SLOT_EXTRA_CLONEABLE, FALSE_DOUBLE)]);
        let source_controller = controller(&mut e, 0, &[(SLOT_CREATE_CLONE, 0x7000_0301)]);
        let n = net(&mut e);
        set_extras(&mut e, n, &[cloneable, 0, skipped], 3);
        e.set(n, NiObjectNET::m_kName, EMPTY);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(source_controller));
        let clone = net(&mut e);
        e.set(clone, NiObjectNET::m_kName, EMPTY);
        let process = cloning_process(&mut e, 0, 0);
        fn_00a5c290(&mut e, n, clone, process);
        assert_eq!(extras(&e, clone, 1), vec![copy]);
        assert_eq!(e.get(clone, NiObjectNET::m_usExtraDataSize), 1);
        assert_eq!(
            e.get(clone, NiObjectNET::m_spControllers).addr(),
            copy_controller
        );
        assert_eq!(count_of(&e, copy_controller), 2);
        assert_eq!(
            calls(&e, 0x7000_0300),
            vec![vec![cloneable, process.addr()]]
        );
    }

    // -- 00a5c480

    fn controller_doubles(e: &mut Engine) {
        e.register(CONTROLLER_HEAD_ASSIGN, |e, a| {
            assign_reference(e, a[0], a[1]);
            ret(a[0])
        });
        e.register(CONTROLLER_NEXT_ASSIGN, |e, a| {
            assign_reference(e, a[0] + CONTROLLER_NEXT, a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_DESTRUCT, |e, a| {
            let held = e.mem.u32(a[0]);
            if held != 0 {
                release(e, held);
            }
            Ret::default()
        });
    }

    fn chain(e: &mut Engine) -> (Ptr<NiObjectNET>, [u32; 3]) {
        let third = controller(e, 0, &[]);
        let second = controller(e, third, &[]);
        let first = controller(e, second, &[]);
        let n = net(e);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(first));
        (n, [first, second, third])
    }

    #[test]
    fn remove_controller_head_middle_and_absent() {
        // The head: the chain moves up and the controller is deleted.
        let mut e = engine();
        controller_doubles(&mut e);
        let (n, [first, second, third]) = chain(&mut e);
        e.call(0x00a5_c480, &args![n, first]);
        assert_eq!(e.get(n, NiObjectNET::m_spControllers).addr(), second);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![first]]);
        assert_eq!(e.mem.u32(first + CONTROLLER_NEXT), 0);
        assert_eq!(count_of(&e, second), 1);
        assert_eq!(e.mem.u32(second + CONTROLLER_NEXT), third);
        // A controller further down: the one before takes its next pointer.
        let mut e = engine();
        controller_doubles(&mut e);
        let (n, [first, second, third]) = chain(&mut e);
        ni_object_net_remove_controller(&mut e, n, Ptr::new(second));
        assert_eq!(e.mem.u32(first + CONTROLLER_NEXT), third);
        assert_eq!(e.mem.u32(second + CONTROLLER_NEXT), 0);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![second]]);
        assert_eq!(count_of(&e, third), 1);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
        // Not in the chain, null, and no controllers at all: nothing.
        let mut e = engine();
        controller_doubles(&mut e);
        let (n, [first, ..]) = chain(&mut e);
        let stranger = controller(&mut e, 0, &[]);
        ni_object_net_remove_controller(&mut e, n, Ptr::new(stranger));
        ni_object_net_remove_controller(&mut e, n, Ptr::NULL);
        let bare = net(&mut e);
        ni_object_net_remove_controller(&mut e, bare, Ptr::new(first));
        assert_eq!(e.get(n, NiObjectNET::m_spControllers).addr(), first);
        assert_eq!(count_of(&e, stranger), 1);
        assert!(calls(&e, CONTROLLER_HEAD_ASSIGN).is_empty());
        assert!(calls(&e, CONTROLLER_NEXT_ASSIGN).is_empty());
    }

    // -- 00a5c570

    #[test]
    fn get_controller_matches_the_type_or_a_base_type() {
        let mut e = engine();
        let base_rtti = e.mem.alloc(8);
        let derived_rtti = e.mem.alloc(8);
        e.mem.set_u32(derived_rtti + 4, base_rtti);
        let other_rtti = e.mem.alloc(8);
        e.register_double(0x7000_0400, move |_, _| ret(other_rtti));
        e.register_double(0x7000_0401, move |_, _| ret(derived_rtti));
        let second = controller(&mut e, 0, &[(SLOT_GET_RTTI, 0x7000_0401)]);
        let first = controller(&mut e, second, &[(SLOT_GET_RTTI, 0x7000_0400)]);
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(first));
        let found = e.call(0x00a5_c570, &args![n, base_rtti]).ptr::<()>();
        assert_eq!(found.addr(), second);
        assert_eq!(
            ni_object_net_get_controller(&mut e, n, Ptr::new(derived_rtti)).addr(),
            second
        );
        assert_eq!(
            ni_object_net_get_controller(&mut e, n, Ptr::new(other_rtti)).addr(),
            first
        );
        let unrelated = e.mem.alloc(8);
        assert!(ni_object_net_get_controller(&mut e, n, Ptr::new(unrelated)).is_null());
        let bare = net(&mut e);
        assert!(ni_object_net_get_controller(&mut e, bare, Ptr::new(base_rtti)).is_null());
    }

    // -- 00a5c5b0

    #[test]
    fn save_binary_writes_the_streamed_entries() {
        let mut e = engine();
        plain(&mut e, &[LINK_STUB, STREAM_SAVE_FIXED_STRING]);
        // The output object: write function pointer at +0xC, position at +4.
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 4, 100);
        e.mem.set_u32(output + 0x0c, 0x7000_0500);
        e.register_double(0x7000_0500, |e, a| {
            // (output, &count, 4, &four, 1): keep what was written.
            let written = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 8, written);
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            ret(4)
        });
        let s = e.mem.alloc(0x300);
        e.mem.set_u32(s + STREAM_OUTPUT, output);
        let stream_table = e.mem.alloc(0x100);
        e.mem
            .set_u32(stream_table + SLOT_STREAM_SAVE_LINK_ID, LOG_DOUBLE);
        e.mem.set_u32(s, stream_table);
        let yes1 = object(&mut e, NAME_A, &[(SLOT_EXTRA_STREAMABLE, TRUE_DOUBLE)]);
        let no = object(&mut e, NAME_B, &[(SLOT_EXTRA_STREAMABLE, FALSE_DOUBLE)]);
        let yes2 = object(&mut e, NAME_C, &[(SLOT_EXTRA_STREAMABLE, TRUE_DOUBLE)]);
        let second = controller(&mut e, 0, &[(SLOT_CONTROLLER_SAVEABLE, TRUE_DOUBLE)]);
        let first = controller(&mut e, second, &[(SLOT_CONTROLLER_SAVEABLE, FALSE_DOUBLE)]);
        let n = net(&mut e);
        set_extras(&mut e, n, &[yes1, no, yes2], 3);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(first));
        e.call(0x00a5_c5b0, &args![n, s]);
        assert_eq!(e.mem.u32(output + 8), 2);
        assert_eq!(e.mem.u32(output + 4), 104);
        assert_eq!(
            calls(&e, STREAM_SAVE_FIXED_STRING),
            vec![vec![s, n.addr() + 8]]
        );
        assert_eq!(
            calls(&e, LOG_DOUBLE),
            vec![vec![s, yes1], vec![s, yes2], vec![s, second]]
        );
        // No controller that wants saving: the link of null is saved.
        let bare = net(&mut e);
        set_extras(&mut e, bare, &[], 1);
        ni_object_net_save_binary(&mut e, bare, Ptr::new(s));
        assert_eq!(calls(&e, LOG_DOUBLE).last().unwrap(), &vec![s, 0]);
        assert_eq!(e.mem.u32(output + 8), 0);
        assert_eq!(e.mem.u32(output + 4), 108);
    }

    // -- 00a5c6a0

    #[test]
    fn post_link_removes_vertex_weights_entries() {
        let mut e = engine();
        plain(&mut e, &[LINK_STUB]);
        e.register_double(0x7000_0600, |_, _| ret(VERT_WEIGHTS_EXTRA_DATA_RTTI));
        let other_rtti = e.mem.alloc(8);
        e.register_double(0x7000_0601, move |_, _| ret(other_rtti));
        let weights1 = object(&mut e, NAME_A, &[(SLOT_GET_RTTI, 0x7000_0600)]);
        let keep = object(&mut e, NAME_B, &[(SLOT_GET_RTTI, 0x7000_0601)]);
        let weights2 = object(&mut e, NAME_C, &[(SLOT_GET_RTTI, 0x7000_0600)]);
        let n = net(&mut e);
        set_extras(&mut e, n, &[weights1, keep, weights2], 3);
        let s = stream(&mut e, 0x1401_0001);
        e.call(0x00a5_c6a0, &args![n, s]);
        assert_eq!(extras(&e, n, 3), vec![keep, 0, 0]);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 1);
        assert_eq!(
            calls(&e, DELETE_DOUBLE),
            vec![vec![weights1], vec![weights2]]
        );
    }

    #[test]
    fn post_link_old_format_turns_the_chain_into_entries() {
        let mut e = engine();
        plain(&mut e, &[LINK_STUB]);
        naming_doubles(&mut e);
        e.register(CLEAR_WORD_AT_8, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        // A chain of two objects linked through the word at +8.
        let last = extra_of_class(&mut e, 0, Some("TwoExtraData"), 0x7000_0700);
        let first = extra_of_class(&mut e, last, Some("OneExtraData"), 0x7000_0701);
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_ppkExtra, Ptr::new(first));
        let s = stream(&mut e, 0x0400_0000);
        ni_object_net_post_link_object(&mut e, n, s);
        assert_eq!(e.get(n, NiObjectNET::m_usExtraDataSize), 2);
        assert_eq!(e.mem.cstr(e.mem.u32(first + 8)), b"OneED000");
        assert_eq!(e.mem.cstr(e.mem.u32(last + 8)), b"TwoED001");
        assert_eq!(calls(&e, CLEAR_WORD_AT_8), vec![vec![first], vec![last]]);
    }

    // -- 00a5c820

    fn is_equal_doubles(e: &mut Engine) {
        e.register(NI_OBJECT_IS_EQUAL, |_, _| ret(1));
        e.register(USHORT_ARRAY_GROW, |e, a| {
            let (array, size) = (a[0], a[1]);
            let old = e.mem.u32(array + 4);
            let used = e.mem.u16(array + 0x0a) as u32;
            let block = e.mem.alloc(size * 2);
            if old != 0 {
                let data = e.mem.bytes(old, used * 2);
                e.mem.write(block, &data);
            }
            e.mem.free(old);
            e.mem.set_u32(array + 4, block);
            e.mem.set_u16(array + 8, size as u16);
            Ret::default()
        });
        e.register(CONTROLLER_CHAIN_LENGTH, |e, a| {
            let mut length = 0;
            let mut at = a[0];
            while at != 0 {
                length += 1;
                at = e.mem.u32(at + CONTROLLER_NEXT);
            }
            ret(length)
        });
    }

    fn equal_object(e: &mut Engine) -> Ptr<NiObjectNET> {
        let streamed = [
            (SLOT_EXTRA_STREAMABLE, TRUE_DOUBLE),
            (SLOT_IS_EQUAL, EQUAL_DOUBLE),
        ];
        let not_streamed = [
            (SLOT_EXTRA_STREAMABLE, FALSE_DOUBLE),
            (SLOT_IS_EQUAL, EQUAL_DOUBLE),
        ];
        let list = [
            object(e, NAME_A, &streamed),
            object(e, NAME_B, &not_streamed),
            object(e, NAME_C, &streamed),
        ];
        let second = controller(e, 0, &[(SLOT_IS_EQUAL, EQUAL_DOUBLE)]);
        e.mem.set_u32(second + 8, NAME_D);
        let first = controller(e, second, &[(SLOT_IS_EQUAL, EQUAL_DOUBLE)]);
        e.mem.set_u32(first + 8, NAME_E);
        let n = net(e);
        set_extras(e, n, &list, 3);
        e.set(n, NiObjectNET::m_kName, NAME_A);
        e.set(n, NiObjectNET::m_spControllers, Ptr::new(first));
        n
    }

    /// Two equal objects: entries (streamed, plain, streamed) with the same
    /// names and a controller chain of two.
    fn equal_pair(e: &mut Engine) -> (Ptr<NiObjectNET>, Ptr<NiObjectNET>) {
        let first = equal_object(e);
        let second = equal_object(e);
        (first, second)
    }

    fn assert_arrays_freed(e: &Engine) {
        let frees = calls(e, FREE);
        assert_eq!(frees.len(), 2);
        for call in frees {
            assert_eq!(e.mem.block_size(call[0]), None);
        }
    }

    #[test]
    fn is_equal_compares_streamed_entries_and_controllers() {
        let mut e = engine();
        is_equal_doubles(&mut e);
        let (n, other) = equal_pair(&mut e);
        assert!(e.call(0x00a5_c820, &args![n, other]).bool());
        assert_arrays_freed(&e);

        // The base class says no: no array is built.
        let mut e = engine();
        is_equal_doubles(&mut e);
        e.register(NI_OBJECT_IS_EQUAL, |_, _| ret(0));
        let (n, other) = equal_pair(&mut e);
        assert!(!ni_object_net_is_equal(&mut e, n, other));
        assert!(calls(&e, FREE).is_empty());

        // Different names.
        let mut e = engine();
        is_equal_doubles(&mut e);
        let (n, other) = equal_pair(&mut e);
        e.set(other, NiObjectNET::m_kName, NAME_B);
        assert!(!ni_object_net_is_equal(&mut e, n, other));
        assert!(calls(&e, FREE).is_empty());

        // A different number of streamed entries.
        let mut e = engine();
        is_equal_doubles(&mut e);
        let (n, other) = equal_pair(&mut e);
        let last = extras(&e, other, 3)[2];
        let table = e.mem.u32(last);
        e.mem.set_u32(table + SLOT_EXTRA_STREAMABLE, FALSE_DOUBLE);
        assert!(!ni_object_net_is_equal(&mut e, n, other));
        assert_arrays_freed(&e);

        // Entries at equal places that differ.
        let mut e = engine();
        is_equal_doubles(&mut e);
        let (n, other) = equal_pair(&mut e);
        let changed = extras(&e, other, 3)[2];
        e.mem.set_u32(changed + 8, NAME_E);
        assert!(!ni_object_net_is_equal(&mut e, n, other));
        assert_arrays_freed(&e);

        // Controller chains of different length.
        let mut e = engine();
        is_equal_doubles(&mut e);
        let (n, other) = equal_pair(&mut e);
        let head = e.get(other, NiObjectNET::m_spControllers).addr();
        e.mem.set_u32(head + CONTROLLER_NEXT, 0);
        assert!(!ni_object_net_is_equal(&mut e, n, other));
        assert_arrays_freed(&e);

        // Controllers of equal count that differ.
        let mut e = engine();
        is_equal_doubles(&mut e);
        let (n, other) = equal_pair(&mut e);
        let head = e.get(other, NiObjectNET::m_spControllers).addr();
        let tail = e.mem.u32(head + CONTROLLER_NEXT);
        e.mem.set_u32(tail + 8, NAME_A);
        assert!(!ni_object_net_is_equal(&mut e, n, other));
        assert_arrays_freed(&e);

        // No controllers on either side and nothing streamed: equal.
        let mut e = engine();
        is_equal_doubles(&mut e);
        let n = net(&mut e);
        let other = net(&mut e);
        e.set(n, NiObjectNET::m_kName, NAME_A);
        e.set(other, NiObjectNET::m_kName, NAME_A);
        assert!(ni_object_net_is_equal(&mut e, n, other));
        assert_arrays_freed(&e);
    }

    // -- 00a5ca80

    #[test]
    fn viewer_strings_append_the_class_name_and_the_name() {
        let mut e = engine();
        plain(&mut e, &[NI_OBJECT_GET_VIEWER_STRINGS]);
        e.map(NI_OBJECT_NET_RTTI, 8);
        e.mem.set_u32(NI_OBJECT_NET_RTTI, 0x4444);
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0x1111));
        e.register(NI_GET_VIEWER_STRING_PAIR, |_, _| ret(0x2222));
        e.register(STRINGS_GROW, |e, a| {
            let (array, size) = (a[0], a[1]);
            let old = e.mem.u32(array + 4);
            let used = e.mem.u16(array + 0x0a) as u32;
            let block = e.mem.alloc(size * 4);
            if used != 0 {
                let data = e.mem.bytes(old, used * 4);
                e.mem.write(block, &data);
            }
            e.mem.free(old);
            e.mem.set_u32(array + 4, block);
            e.mem.set_u16(array + 8, size as u16);
            Ret::default()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0x0a, (a[1] + 1) as u16);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0x0e, 4);
        let n = net(&mut e);
        e.set(n, NiObjectNET::m_kName, NAME_A);
        e.call(0x00a5_ca80, &args![n, strings]);
        assert_eq!(e.mem.u16(strings + 0x0a), 2);
        let base = e.mem.u32(strings + 4);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(base + 4), 0x2222);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x4444]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_PAIR),
            vec![vec![NAME_FIELD_TEXT, NAME_A]]
        );
        // Storage was grown once (capacity 0 to 4); the second string fitted.
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 4]]);
    }
}
