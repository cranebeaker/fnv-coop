//! `NiMain/niobject.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiObject` (Xbox PDB, 8 bytes) is `NiRefObject` (vtable, `m_uiRefCount`
//! at +4) with the streaming and cloning interface. Vtable slots used here
//! (Xbox PDB `NiObject`): `+0x04` `DeleteThis`, `+0x08` `GetRTTI`, `+0x48`
//! `CreateClone`, `+0x68` `ProcessClone`, `+0x84` `SetGroup`.
//!
//! Only `CreateDeepCopy` (`00a5d510`) has a `/GS` cookie check; none of the
//! others has an exception frame or a cookie check.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiRefObject, NiTArray, NiTPointerMap};

/// Import slots of `InterlockedIncrement` / `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::ms_uiObjects` (Xbox PDB static): the live object count the
/// constructor and destructor change.
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The vtable of `NiRefObject` (RTTI `.?AVNiRefObject@@`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// The vtable of `NiObject` (RTTI `.?AVNiObject@@`).
const NI_OBJECT_VTABLE: u32 = 0x0109_b4f4;

/// `NiObject` vtable slots (Xbox PDB).
const DELETE_THIS_SLOT: u32 = 0x04;
const GET_RTTI_SLOT: u32 = 0x08;
const CREATE_CLONE_SLOT: u32 = 0x48;
const PROCESS_CLONE_SLOT: u32 = 0x68;
const SET_GROUP_SLOT: u32 = 0x84;
/// `NiStream` vtable slot `+0x24` (`RegisterSaveObject` in Gamebryo's
/// interface; the Xbox PDB slot name is not confirmed).
const STREAM_REGISTER_SLOT: u32 = 0x24;

/// `NiTMapBase` vtable slots (Xbox PDB `NiTMapBase<..>`): `KeyToHashIndex`
/// (+0x04), `IsKeysEqual` (+0x08), `SetValue` (+0x0c), `NewItem` (+0x14).
const MAP_KEY_TO_HASH_INDEX_SLOT: u32 = 0x04;
const MAP_IS_KEYS_EQUAL_SLOT: u32 = 0x08;
const MAP_SET_VALUE_SLOT: u32 = 0x0c;
const MAP_NEW_ITEM_SLOT: u32 = 0x14;
/// A map item (`NiTMapItem`, Xbox PDB): `m_pkNext` +0, `m_key` +4, `m_val` +8.
const ITEM_NEXT: u32 = 0x0;
const ITEM_KEY: u32 = 0x4;
const ITEM_VALUE: u32 = 0x8;

/// `00a53c30` (`thiscall` on an `NiTPointerMap<NiObject*, NiObject*>`,
/// `RET 8`): looks up its first word as the key; when found writes the
/// value at the address in its second word and returns true in `AL`.
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `00b57ea0` (`thiscall` on an `NiTPointerMap<NiObject*, NiObject*>`,
/// `RET 8`): sets the value (second word) of the key (first word).
const CLONE_MAP_SET_AT: u32 = 0x00b5_7ea0;
/// `004ad0c0` (`thiscall`, `RET 4`): the `NiCloningProcess` constructor
/// (allocates its two maps and sets the copy type and append character;
/// the one word goes to both map constructors, 0x101 here).
const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d0c0;
/// `NiCloningProcess` (Xbox PDB), 0x10 bytes: `m_pkCloneMap` +0,
/// `m_pkProcessMap` +4, `m_eCopyType` +8, `m_cAppendChar` +0xc.
const CLONING_PROCESS_SIZE: u32 = 0x10;
const CLONE_MAP: u32 = 0x0;
const PROCESS_MAP: u32 = 0x4;
/// The word `NiObject::Clone()` gives its cloning process's constructor.
const CLONE_MAP_BUCKETS: u32 = 0x101;

/// `NiStream` (Xbox PDB, 0x5c4 bytes; PC offsets as the code uses them).
const NI_STREAM_SIZE: u32 = 0x5c4;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xd8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// `NiStream::m_kTopObjects.m_pBase` (Xbox PDB; `m_kTopObjects` at +0x21c).
const STREAM_TOP_OBJECTS: u32 = 0x220;
/// `NiStream::m_pkIstr` (Xbox PDB) +0x24c: the `NiBinaryStream*` read from.
const STREAM_INPUT: u32 = 0x24c;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB) +4.
const BINARY_STREAM_POSITION: u32 = 0x4;
/// `NiBinaryStream::m_pfnRead` (Xbox PDB) +8: a `cdecl` function pointer
/// `(stream, buffer, size, &component size, component count)` returning the
/// number of bytes read.
const BINARY_STREAM_READ: u32 = 0x8;
/// The file versions (`[0x05000006, 0x0a010072)`) whose objects carry a
/// group id.
const GROUP_ID_FIRST_VERSION: u32 = 0x0500_0006;
const GROUP_ID_END_VERSION: u32 = 0x0a01_0072;

/// `NiStream` constructor (`00a66150`, `thiscall`, plain `RET`).
const STREAM_CONSTRUCT: u32 = 0x00a6_6150;
/// `NiStream::InsertObject` (engine map, `00a66370`, `RET 4`).
const STREAM_INSERT_OBJECT: u32 = 0x00a6_6370;
/// `NiStream::Save_ov2` (engine map, `00a635e0`, `RET 8`): saves to a new
/// buffer; writes its address and size at the two words' addresses.
const STREAM_SAVE_TO_BUFFER: u32 = 0x00a6_35e0;
/// `NiStream::Load` (engine map, `00a632f0`, `RET 8`): loads from a buffer
/// and its size.
const STREAM_LOAD_FROM_BUFFER: u32 = 0x00a6_32f0;
/// `NiStream` destructor (`00a65300`, `thiscall`, plain `RET`).
const STREAM_DESTRUCT: u32 = 0x00a6_5300;
/// `NiStream::GetGroupFromID` (engine map, `00a634e0`, `RET 4`).
const STREAM_GET_GROUP_FROM_ID: u32 = 0x00a6_34e0;
/// `00aa10f0` (`cdecl`, one word): frees a block from the Ni allocator.
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a5d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::NiObject` (unnamed in the engine map; the body is the inlined
/// `NiRefObject` constructor and `NiObject`'s vtable): stores the
/// `NiRefObject` vtable, clears `m_uiRefCount`, increments
/// `NiRefObject::ms_uiObjects` (`011f4400`) through `InterlockedIncrement`,
/// then stores the `NiObject` vtable. Returns `this`.
pub fn ni_object_construct(e: &mut Engine, this: Ptr<NiRefObject>) -> Ptr<NiRefObject> {
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.set(this, NiRefObject::m_uiRefCount, 0);
    e.call(INTERLOCKED_INCREMENT, &args![REF_OBJECT_COUNT]);
    e.mem.set_u32(this.addr(), NI_OBJECT_VTABLE);
    this
}

// Translated from 00a5d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::~NiObject` (unnamed in the engine map; the inlined
/// `NiRefObject` destructor): stores the `NiRefObject` vtable and
/// decrements `NiRefObject::ms_uiObjects` (`011f4400`) through
/// `InterlockedDecrement`.
pub fn ni_object_destruct(e: &mut Engine, this: Ptr<NiRefObject>) {
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![REF_OBJECT_COUNT]);
}

// Translated from 00a5d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::Clone` (Xbox PDB, `NiObject* Clone(NiCloningProcess&)`):
/// `CreateClone` (vtable `+0x48`) with the cloning process, then
/// `ProcessClone` (`+0x68`) with it; returns the clone.
pub fn ni_object_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e
        .vcall(this.addr(), CREATE_CLONE_SLOT, &args![cloning])
        .u32();
    e.vcall(this.addr(), PROCESS_CLONE_SLOT, &args![cloning]);
    Ptr::new(clone)
}

// Translated from 00a5d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::RegisterStreamables` (Xbox PDB): a tail call of the stream's
/// vtable slot `+0x24` with `this`; returns its `AL`.
pub fn ni_object_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.vcall(stream.addr(), STREAM_REGISTER_SLOT, &args![this])
        .bool()
}

// Translated from 00a5d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::GetStreamableRTTI` (Xbox PDB): a tail call of `GetRTTI`
/// (vtable `+0x08`).
pub fn ni_object_get_streamable_rtti(e: &mut Engine, this: Ptr) -> Ptr {
    e.vcall(this.addr(), GET_RTTI_SLOT, &args![]).ptr()
}

// Translated from 00a5d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiObject*, bool>::SetAt`
/// (Xbox PDB): hashes `key` (`KeyToHashIndex`, vtable `+0x04`) and walks the
/// bucket's items; for the first whose key `IsKeysEqual` (`+0x08`) says is
/// equal it stores the low byte of `value` as the item's value. Without one
/// it takes a new item (`NewItem`, `+0x14`), fills it (`SetValue`, `+0x0c`,
/// with the whole `value` word), puts it at the head of the bucket and
/// counts it.
pub fn ni_t_map_base_set_at(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: u32) {
    let map = this.addr();
    let index = e.vcall(map, MAP_KEY_TO_HASH_INDEX_SLOT, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(ITEM_KEY));
        if e.vcall(map, MAP_IS_KEYS_EQUAL_SLOT, &args![key, item_key])
            .bool()
        {
            e.mem.set_u8(item.wrapping_add(ITEM_VALUE), value as u8);
            return;
        }
        item = e.mem.u32(item.wrapping_add(ITEM_NEXT));
    }
    let item = e.vcall(map, MAP_NEW_ITEM_SLOT, &args![]).u32();
    e.vcall(map, MAP_SET_VALUE_SLOT, &args![item, key, value]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    e.mem.set_u32(item.wrapping_add(ITEM_NEXT), head);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.mem
        .set_u32(table.wrapping_add(index.wrapping_mul(4)), item);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 00a5d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the clone of `this` in
/// a cloning process. Looks `this` up in the process's clone map
/// (`m_pkCloneMap`, `00a53c30`, the found value written into the argument's
/// own stack word) and returns it when found; otherwise returns
/// `CreateClone` (vtable `+0x48`) with the process.
pub fn fn_00a5d3f0(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone_map = e.mem.u32(cloning.addr().wrapping_add(CLONE_MAP));
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cloning.addr());
        let found = e
            .call(CLONE_MAP_GET_AT, &args![clone_map, this, slot])
            .bool();
        found.then(|| e.mem.u32(slot.addr()))
    });
    match found {
        Some(clone) => Ptr::new(clone),
        None => e
            .vcall(this.addr(), CREATE_CLONE_SLOT, &args![cloning])
            .ptr(),
    }
}

// Translated from 00a5d430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::ProcessClone` (Xbox PDB): marks `this` processed in the
/// cloning process's `m_pkProcessMap` (`SetAt(this, true)`).
pub fn ni_object_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    let process_map = e.mem.u32(cloning.addr().wrapping_add(PROCESS_MAP));
    ni_t_map_base_set_at(e, Ptr::new(process_map), this.addr(), 1);
}

// Translated from 00a5d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `NiObject::CopyMembers`
/// by the jump `00aa1000` makes to it, not confirmed): records `clone` as
/// the clone of `this` in the cloning process's `m_pkCloneMap`
/// (`00b57ea0(map, this, clone)`).
pub fn fn_00a5d450(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    let clone_map = e.mem.u32(cloning.addr().wrapping_add(CLONE_MAP));
    e.call(CLONE_MAP_SET_AT, &args![clone_map, this, clone]);
}

/// `NiPointer` release of `object` (non-null): `InterlockedDecrement` of
/// its count, `DeleteThis` (vtable `+0x04`) when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, DELETE_THIS_SLOT, &args![]);
    }
}

// Translated from 00a5d510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a5d510 - the /GS cookie check (00ec408c) on the stack frame is not translated
/// `NiObject::CreateDeepCopy` (Xbox PDB, `NiPointer<NiObject>` result at
/// `result`): builds an `NiStream` on the stack (`00a66150`), inserts
/// `this` (`InsertObject`), saves it to a buffer (`Save_ov2`, buffer and
/// size into two stack words cleared first) and loads that buffer back
/// (`Load`). The first top object (`m_kTopObjects.m_pBase[0]`) is held
/// (counted when non-null), the buffer freed (`00aa10f0`), the object
/// assigned to the `NiPointer` at `result` (when it differs: the old one
/// released, the new one counted) and the held reference released; then
/// the stream's destructor (`00a65300`). The function leaves no result in
/// `EAX`, so nothing is returned.
pub fn ni_object_create_deep_copy(e: &mut Engine, this: Ptr, result: Ptr) {
    // The frame: buffer +0, size +4, the stream at +8.
    e.with_stack(8 + NI_STREAM_SIZE, |e, frame| {
        let buffer = frame.addr();
        let size = buffer + 4;
        let stream = buffer + 8;
        e.call(STREAM_CONSTRUCT, &args![stream]);
        e.call(STREAM_INSERT_OBJECT, &args![stream, this]);
        e.mem.set_u32(buffer, 0);
        e.mem.set_u32(size, 0);
        e.call(STREAM_SAVE_TO_BUFFER, &args![stream, buffer, size]);
        let data = e.mem.u32(buffer);
        let length = e.mem.u32(size);
        e.call(STREAM_LOAD_FROM_BUFFER, &args![stream, data, length]);
        let top_objects = e.mem.u32(stream + STREAM_TOP_OBJECTS);
        let object = e.mem.u32(top_objects);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
        let data = e.mem.u32(buffer);
        e.call(NI_FREE, &args![data]);
        let old = e.mem.u32(result.addr());
        if old != object {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(result.addr(), object);
            if object != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
            }
        }
        if object != 0 {
            release(e, object);
        }
        e.call(STREAM_DESTRUCT, &args![stream]);
    });
}

// Translated from 00a5d610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::LoadBinary` (Xbox PDB): for files of version `0x05000006` up
/// to (not including) `0x0a010072`, reads the 4-byte group id through the
/// input stream's `m_pfnRead` (`(istr, &id, 4, &component size = 4, 1)`;
/// the component size is the argument's own stack word, overwritten with
/// 4), advances the stream's position by the bytes read, and gives `SetGroup`
/// (vtable `+0x84`) the group `NiStream::GetGroupFromID` returns for the id.
/// Other versions read nothing.
pub fn ni_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let version = e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION));
    if !(GROUP_ID_FIRST_VERSION..GROUP_ID_END_VERSION).contains(&version) {
        return;
    }
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    let group_id = e.with_stack(8, |e, frame| {
        let id = frame.addr();
        let component_size = id + 4;
        e.mem.set_u32(component_size, 4);
        let bytes = e
            .call(read, &args![input, id, 4u32, component_size, 1u32])
            .u32();
        let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
        e.mem.set_u32(
            input.wrapping_add(BINARY_STREAM_POSITION),
            position.wrapping_add(bytes),
        );
        e.mem.u32(id)
    });
    let group = e
        .call(STREAM_GET_GROUP_FROM_ID, &args![stream, group_id])
        .u32();
    e.vcall(this.addr(), SET_GROUP_SLOT, &args![group]);
}

// Translated from 00a5d680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::Clone_ov2` (Xbox PDB, `NiObject* Clone()`): a cloning process
/// on the stack (`004ad0c0` with 0x101), `CreateClone` (vtable `+0x48`) and
/// `ProcessClone` (`+0x68`) with it, then the process's two maps
/// (`m_pkCloneMap`, `m_pkProcessMap`) deleted through their vtable slot 0
/// with the flag 1 when non-null. Returns the clone.
pub fn ni_object_clone_ov2(e: &mut Engine, this: Ptr) -> Ptr {
    e.with_stack(CLONING_PROCESS_SIZE, |e, cloning| {
        e.call(
            CLONING_PROCESS_CONSTRUCT,
            &args![cloning, CLONE_MAP_BUCKETS],
        );
        let clone = e
            .vcall(this.addr(), CREATE_CLONE_SLOT, &args![cloning])
            .u32();
        e.vcall(this.addr(), PROCESS_CLONE_SLOT, &args![cloning]);
        for offset in [CLONE_MAP, PROCESS_MAP] {
            let map = e.mem.u32(cloning.addr() + offset);
            if map != 0 {
                e.vcall(map, 0, &args![1u32]);
            }
        }
        Ptr::new(clone)
    })
}

/// The sized delete (`00aa1460`, `cdecl`: pointer, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiObject::ms_RTTI` (`011f4418`, the record `NiObject`'s `GetRTTI` answers): its first
/// word is the class name.
const NI_OBJECT_RTTI: u32 = 0x011f_4418;
/// Viewer string builders (`cdecl`): `NiGetViewerString` (`00a7d330`, the class name),
/// `NiGetViewerString_ov10` (`00a7d5a0`, a label and a pointer) and `NiGetViewerString_ov8`
/// (`00a7d510`, a label and an unsigned value), each answering a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
/// `NiTArray<char *>` resize (`0096ad30`, one word: the new size) and `SetAt` (`0096ae90`:
/// index, address of the value).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The labels `"this"` (`010408ec`) and `"m_uiRefCount"` (`0109b580`).
const TEXT_THIS: u32 = 0x0104_08ec;
const TEXT_REF_COUNT: u32 = 0x0109_b580;

// Translated from 00a5d470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::IsEqual` (Xbox PDB): false for a null `other`; otherwise whether the class
/// names of the two objects' RTTI records (`GetRTTI`, vtable `+0x08`, first `other`'s then
/// this one's; the name is the record's first word) are the same string (compared byte by
/// byte, inlined `strcmp`).
pub fn ni_object_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    let other_rtti = e.vcall(other.addr(), GET_RTTI_SLOT, &args![]).u32();
    let mut theirs = e.mem.u32(other_rtti);
    let own_rtti = e.vcall(this.addr(), GET_RTTI_SLOT, &args![]).u32();
    let mut ours = e.mem.u32(own_rtti);
    loop {
        let byte = e.mem.u8(ours);
        if byte != e.mem.u8(theirs) {
            return false;
        }
        if byte == 0 {
            return true;
        }
        ours = ours.wrapping_add(1);
        theirs = theirs.wrapping_add(1);
    }
}

// Translated from 00a5d4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::_scalar_deleting_destructor_` (Xbox PDB): the inlined destructor (the
/// `NiRefObject` vtable, `NiRefObject::ms_uiObjects` decremented through
/// `InterlockedDecrement`), then the sized delete (`00aa1460`) of 8 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_object_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![REF_OBJECT_COUNT]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 8u32]);
    }
    this
}

/// Adds the string in the word at `slot` to the viewer strings (`NiTArray<char *>`): grown by
/// its grow-by count when full (`0096ad30`), then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = e.get(strings, NiTArray::m_usSize) as u32;
    if index >= e.get(strings, NiTArray::m_usMaxSize) as u32 {
        let grow = e.get(strings, NiTArray::m_usGrowBy) as u32;
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a5d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::GetViewerStrings` (Xbox PDB): adds the class name (`NiObject`'s RTTI name,
/// `00a7d330`), `"this"` with the object's address (`00a7d5a0`) and `"m_uiRefCount"` with
/// the reference count (`00a7d510`).
pub fn ni_object_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr<NiTArray>) {
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_OBJECT_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let text = e.call(VIEWER_STRING_POINTER, &args![TEXT_THIS, this]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        // `NiRefObject::m_uiRefCount` (Xbox PDB) +4.
        let count = e.mem.u32(this.addr() + 4);
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_REF_COUNT, count])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_d3a0,
            ni_object_construct(Ptr<NiRefObject>) -> Ptr<NiRefObject>
        ),
        entry!(0x00a5_d3d0, ni_object_destruct(Ptr<NiRefObject>)),
        entry!(0x00a5_d2c0, ni_object_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_d2f0, ni_object_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a5_d310, ni_object_get_streamable_rtti(Ptr) -> Ptr),
        entry!(
            0x00a5_d320,
            ni_t_map_base_set_at(Ptr<NiTPointerMap>, u32, u32)
        ),
        entry!(0x00a5_d3f0, fn_00a5d3f0(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_d430, ni_object_process_clone(Ptr, Ptr)),
        entry!(0x00a5_d450, fn_00a5d450(Ptr, Ptr, Ptr)),
        entry!(0x00a5_d510, ni_object_create_deep_copy(Ptr, Ptr)),
        entry!(0x00a5_d610, ni_object_load_binary(Ptr, Ptr)),
        entry!(0x00a5_d680, ni_object_clone_ov2(Ptr) -> Ptr),
        entry!(0x00a5_d470, ni_object_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a5_d4e0,
            ni_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a5_d6e0,
            ni_object_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const VTABLE: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Registers a logging double returning `eax` at `address`.
    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    /// The target the test vtable gives slot `offset`.
    fn slot(offset: u32) -> u32 {
        0x00ee_0000 + offset
    }

    /// An object whose vtable (at `VTABLE`) points each slot at `slot(..)`.
    fn object(e: &mut Engine) -> u32 {
        if !e.mem.is_mapped(VTABLE) {
            let slots: Vec<u32> = (0..0x40).map(|i| slot(i * 4)).collect();
            e.put_vtable(VTABLE, &slots);
        }
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p, VTABLE);
        p
    }

    fn counters(e: &mut Engine) {
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
    }

    #[test]
    fn constructor_and_destructor_count_the_objects() {
        let mut e = Engine::new();
        counters(&mut e);
        e.map(REF_OBJECT_COUNT, 4);
        e.mem.set_u32(REF_OBJECT_COUNT, 5);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x77);
        assert_eq!(e.call(0x00a5_d3a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(REF_OBJECT_COUNT), 6);
        e.call(0x00a5_d3d0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(REF_OBJECT_COUNT), 5);
    }

    #[test]
    fn clone_creates_then_processes() {
        let mut e = Engine::new();
        let log = Log::default();
        let this = object(&mut e);
        logged(&mut e, &log, slot(CREATE_CLONE_SLOT), 0x4444);
        logged(&mut e, &log, slot(PROCESS_CLONE_SLOT), 0x9999);
        assert_eq!(e.call(0x00a5_d2c0, &args![this, 0x1234u32]).u32(), 0x4444);
        assert_eq!(
            *log.borrow(),
            [
                (slot(CREATE_CLONE_SLOT), vec![this, 0x1234]),
                (slot(PROCESS_CLONE_SLOT), vec![this, 0x1234]),
            ]
        );
    }

    #[test]
    fn register_streamables_and_get_streamable_rtti_tail_call_their_slots() {
        let mut e = Engine::new();
        let log = Log::default();
        let stream = object(&mut e);
        let this = object(&mut e);
        logged(&mut e, &log, slot(STREAM_REGISTER_SLOT), 0x0000_0101);
        logged(&mut e, &log, slot(GET_RTTI_SLOT), 0x0123_4567);
        assert!(e.call(0x00a5_d2f0, &args![this, stream]).bool());
        assert_eq!(e.call(0x00a5_d310, &args![this]).u32(), 0x0123_4567);
        assert_eq!(
            *log.borrow(),
            [
                (slot(STREAM_REGISTER_SLOT), vec![stream, this]),
                (slot(GET_RTTI_SLOT), vec![this]),
            ]
        );
    }

    /// A map with 4 buckets; `KeyToHashIndex` is `key % 4`, `IsKeysEqual`
    /// compares the words, `NewItem` allocates 12 bytes, `SetValue` stores
    /// key and value.
    fn map(e: &mut Engine) -> u32 {
        let map = object(e);
        let table = e.mem.alloc(16);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, table);
        e.register(slot(MAP_KEY_TO_HASH_INDEX_SLOT), |_, a| ret(a[1] % 4));
        e.register(slot(MAP_IS_KEYS_EQUAL_SLOT), |_, a| {
            ret((a[1] == a[2]) as u32)
        });
        e.register(slot(MAP_NEW_ITEM_SLOT), |e, _| ret(e.mem.alloc(12)));
        e.register(slot(MAP_SET_VALUE_SLOT), |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        map
    }

    #[test]
    fn set_at_adds_new_keys_at_the_bucket_head_and_updates_existing_ones() {
        let mut e = Engine::new();
        let map = map(&mut e);
        let table = e.mem.u32(map + 8);
        e.call(0x00a5_d320, &args![map, 5u32, 1u32]);
        e.call(0x00a5_d320, &args![map, 9u32, 0x0000_0201u32]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        let head = e.mem.u32(table + 4);
        assert_eq!(e.mem.u32(head + 4), 9);
        assert_eq!(e.mem.u32(head + 8), 0x0201);
        let second = e.mem.u32(head);
        assert_eq!(e.mem.u32(second + 4), 5);
        assert_eq!(e.mem.u32(second + 8), 1);
        assert_eq!(e.mem.u32(second), 0);
        // An existing key: only the value byte changes, nothing is added.
        e.call(0x00a5_d320, &args![map, 5u32, 0x0000_ff00u32]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert_eq!(e.mem.u32(second + 8), 0);
        assert_eq!(e.mem.u32(table + 4), head);
    }

    #[test]
    fn process_clone_marks_the_object_in_the_process_map() {
        let mut e = Engine::new();
        let map = map(&mut e);
        let cloning = e.mem.alloc(0x10);
        e.mem.set_u32(cloning + 4, map);
        let this = 0x0000_0102u32;
        e.call(0x00a5_d430, &args![this, cloning]);
        let table = e.mem.u32(map + 8);
        let item = e.mem.u32(table + 8);
        assert_eq!(e.mem.u32(item + 4), this);
        assert_eq!(e.mem.u32(item + 8), 1);
        assert_eq!(e.mem.u32(map + 0xc), 1);
    }

    #[test]
    fn clone_lookup_returns_the_recorded_clone_or_creates_one() {
        let mut e = Engine::new();
        let log = Log::default();
        let recorded = object(&mut e);
        let fresh = object(&mut e);
        let cloning = e.mem.alloc(0x10);
        e.mem.set_u32(cloning, 0x5000);
        logged(&mut e, &log, slot(CREATE_CLONE_SLOT), 0x7777);
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            assert_eq!(a[0], 0x5000);
            if a[1] == recorded {
                e.mem.set_u32(a[2], 0x6666);
                ret(1)
            } else {
                ret(0x100)
            }
        });
        assert_eq!(e.call(0x00a5_d3f0, &args![recorded, cloning]).u32(), 0x6666);
        assert!(log.borrow().is_empty());
        assert_eq!(e.call(0x00a5_d3f0, &args![fresh, cloning]).u32(), 0x7777);
        assert_eq!(
            *log.borrow(),
            [(slot(CREATE_CLONE_SLOT), vec![fresh, cloning])]
        );
    }

    #[test]
    fn copy_records_the_clone_in_the_clone_map() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, CLONE_MAP_SET_AT, 0);
        let cloning = e.mem.alloc(0x10);
        e.mem.set_u32(cloning, 0x5000);
        e.call(0x00a5_d450, &args![0x1000u32, 0x2000u32, cloning]);
        assert_eq!(
            *log.borrow(),
            [(CLONE_MAP_SET_AT, vec![0x5000, 0x1000, 0x2000])]
        );
    }

    /// The outcome of a deep copy: the engine, the calls, the result
    /// pointer's address, the loaded object and the old one.
    struct DeepCopy {
        e: Engine,
        log: Log,
        result: u32,
        loaded: u32,
        old: u32,
    }

    /// A deep copy whose load produces an object with count `loaded_count`
    /// (none for `None`) while the result holds one with `old_count` (or the
    /// loaded one itself when `same`).
    fn deep_copy(loaded_count: Option<u32>, old_count: Option<u32>, same: bool) -> DeepCopy {
        let mut e = Engine::new();
        let log = Log::default();
        counters(&mut e);
        let counted = |e: &mut Engine, count: Option<u32>| {
            count.map_or(0, |count| {
                let p = object(e);
                e.mem.set_u32(p + 4, count);
                p
            })
        };
        let loaded = counted(&mut e, loaded_count);
        let old = if same {
            loaded
        } else {
            counted(&mut e, old_count)
        };
        logged(&mut e, &log, slot(DELETE_THIS_SLOT), 0);
        logged(&mut e, &log, STREAM_CONSTRUCT, 0);
        logged(&mut e, &log, STREAM_INSERT_OBJECT, 0);
        logged(&mut e, &log, STREAM_DESTRUCT, 0);
        logged(&mut e, &log, NI_FREE, 0);
        let l = log.clone();
        e.register_double(STREAM_SAVE_TO_BUFFER, move |e, a| {
            let words = vec![e.mem.u32(a[1]), e.mem.u32(a[2])];
            l.borrow_mut().push((STREAM_SAVE_TO_BUFFER, words));
            e.mem.set_u32(a[1], 0xb0f0);
            e.mem.set_u32(a[2], 0x44);
            Ret::default()
        });
        let l = log.clone();
        e.register_double(STREAM_LOAD_FROM_BUFFER, move |e, a| {
            l.borrow_mut()
                .push((STREAM_LOAD_FROM_BUFFER, a[1..].to_vec()));
            let top = e.mem.alloc(4);
            e.mem.set_u32(top, loaded);
            e.mem.set_u32(a[0] + STREAM_TOP_OBJECTS, top);
            Ret::default()
        });
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, old);
        e.call(0x00a5_d510, &args![0x1230u32, result]);
        DeepCopy {
            e,
            log,
            result,
            loaded,
            old,
        }
    }

    #[test]
    fn create_deep_copy_saves_loads_and_assigns_the_first_top_object() {
        let d = deep_copy(Some(0), Some(1), false);
        assert_eq!(d.e.mem.u32(d.result), d.loaded);
        assert_eq!(d.e.mem.u32(d.loaded + 4), 1);
        assert_eq!(d.e.mem.u32(d.old + 4), 0);
        let log = d.log.borrow();
        let stream = log[0].1[0];
        assert_eq!(
            *log,
            [
                (STREAM_CONSTRUCT, vec![stream]),
                (STREAM_INSERT_OBJECT, vec![stream, 0x1230]),
                (STREAM_SAVE_TO_BUFFER, vec![0, 0]),
                (STREAM_LOAD_FROM_BUFFER, vec![0xb0f0, 0x44]),
                (NI_FREE, vec![0xb0f0]),
                (slot(DELETE_THIS_SLOT), vec![d.old]),
                (STREAM_DESTRUCT, vec![stream]),
            ]
        );
    }

    #[test]
    fn create_deep_copy_keeps_the_same_object_and_handles_none() {
        let d = deep_copy(Some(1), None, true);
        assert_eq!(d.e.mem.u32(d.result), d.loaded);
        assert_eq!(d.e.mem.u32(d.loaded + 4), 1);
        assert!(!d
            .log
            .borrow()
            .iter()
            .any(|(a, _)| *a == slot(DELETE_THIS_SLOT)));
        let d = deep_copy(None, None, false);
        assert_eq!(d.e.mem.u32(d.result), 0);
        assert!(!d
            .log
            .borrow()
            .iter()
            .any(|(a, _)| *a == slot(DELETE_THIS_SLOT)));
    }

    #[test]
    fn create_deep_copy_deletes_a_loaded_object_nobody_else_holds() {
        // The result already holds the loaded object without counting it:
        // nothing is assigned and the held reference drops it to zero.
        let d = deep_copy(Some(0), None, true);
        assert!(d
            .log
            .borrow()
            .iter()
            .any(|entry| *entry == (slot(DELETE_THIS_SLOT), vec![d.loaded])));
    }

    /// `LoadBinary` on a stream of `version` whose read function hands out
    /// the group id 0x33.
    fn load_fixture(version: u32) -> (Engine, Log, [u32; 3]) {
        let mut e = Engine::new();
        let log = Log::default();
        let this = object(&mut e);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + 4, 0x100);
        e.mem.set_u32(input + 8, 0x00ef_0000);
        let l = log.clone();
        e.register_double(0x00ef_0000, move |e, a| {
            let words = vec![a[0], a[2], e.mem.u32(a[3]), a[4]];
            l.borrow_mut().push((0x00ef_0000, words));
            e.mem.set_u32(a[1], 0x33);
            ret(4)
        });
        logged(&mut e, &log, STREAM_GET_GROUP_FROM_ID, 0x6060);
        logged(&mut e, &log, slot(SET_GROUP_SLOT), 0);
        e.call(0x00a5_d610, &args![this, stream]);
        (e, log, [this, stream, input])
    }

    #[test]
    fn load_binary_reads_the_group_id_for_versions_in_range() {
        for version in [0x0500_0006u32, 0x0a01_0071] {
            let (e, log, [this, stream, input]) = load_fixture(version);
            assert_eq!(
                *log.borrow(),
                [
                    (0x00ef_0000, vec![input, 4, 4, 1]),
                    (STREAM_GET_GROUP_FROM_ID, vec![stream, 0x33]),
                    (slot(SET_GROUP_SLOT), vec![this, 0x6060]),
                ]
            );
            assert_eq!(e.mem.u32(input + 4), 0x104);
        }
    }

    #[test]
    fn load_binary_reads_nothing_for_other_versions() {
        for version in [0x0500_0005u32, 0x0a01_0072] {
            let (e, log, [_, _, input]) = load_fixture(version);
            assert!(log.borrow().is_empty());
            assert_eq!(e.mem.u32(input + 4), 0x100);
        }
    }

    #[test]
    fn clone_without_a_process_makes_and_frees_one() {
        let mut e = Engine::new();
        let log = Log::default();
        let this = object(&mut e);
        let clone_map = object(&mut e);
        let l = log.clone();
        e.register_double(CLONING_PROCESS_CONSTRUCT, move |e, a| {
            l.borrow_mut().push((CLONING_PROCESS_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0], clone_map);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        logged(&mut e, &log, slot(CREATE_CLONE_SLOT), 0x4444);
        logged(&mut e, &log, slot(PROCESS_CLONE_SLOT), 0);
        logged(&mut e, &log, slot(0), 0);
        assert_eq!(e.call(0x00a5_d680, &args![this]).u32(), 0x4444);
        let log = log.borrow();
        let cloning = log[0].1[0];
        assert_eq!(
            *log,
            [
                (CLONING_PROCESS_CONSTRUCT, vec![cloning, 0x101]),
                (slot(CREATE_CLONE_SLOT), vec![this, cloning]),
                (slot(PROCESS_CLONE_SLOT), vec![this, cloning]),
                (slot(0), vec![clone_map, 1]),
            ]
        );
    }

    /// An object whose `GetRTTI` slot answers a record named `name`.
    fn named(e: &mut Engine, name: &[u8], index: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, vtable);
        let text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.write(text, name);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti, text);
        // Each object gets its own `GetRTTI` double at an address of its own.
        let getter = 0x0310_0000 + 0x10 * index;
        e.mem.set_u32(vtable + GET_RTTI_SLOT, getter);
        e.register_double(getter, move |_, _| ret(rtti));
        p
    }

    #[test]
    fn is_equal_compares_the_class_names() {
        let mut e = Engine::new();
        let a = named(&mut e, b"NiNode", 0);
        let b = named(&mut e, b"NiNode", 1);
        let c = named(&mut e, b"NiNodeX", 2);
        let d = named(&mut e, b"NiNodf", 3);
        assert!(e.call(0x00a5_d470, &args![a, b]).bool());
        assert!(!e.call(0x00a5_d470, &args![a, c]).bool());
        assert!(!e.call(0x00a5_d470, &args![c, a]).bool());
        assert!(!e.call(0x00a5_d470, &args![a, d]).bool());
        assert!(!e.call(0x00a5_d470, &args![a, 0u32]).bool());
    }

    #[test]
    fn scalar_deleting_destructor_counts_down_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        counters(&mut e);
        e.map(REF_OBJECT_COUNT & !0xfff, 0x1000);
        e.mem.set_u32(REF_OBJECT_COUNT, 5);
        let log = Log::default();
        logged(&mut e, &log, SIZED_DELETE, 0);
        let this = object(&mut e);
        assert_eq!(e.call(0x00a5_d4e0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(REF_OBJECT_COUNT), 4);
        assert!(log.borrow().is_empty());
        assert_eq!(e.call(0x00a5_d4e0, &args![this, 0x101u32]).u32(), this);
        assert_eq!(e.mem.u32(REF_OBJECT_COUNT), 3);
        assert_eq!(*log.borrow(), [(SIZED_DELETE, vec![this, 8])]);
    }

    #[test]
    fn viewer_strings_add_the_name_the_address_and_the_count_growing_when_full() {
        let mut e = Engine::new();
        e.map(NI_OBJECT_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(NI_OBJECT_RTTI, 0x0123_4567);
        let this = object(&mut e);
        e.mem.set_u32(this + 4, 7);
        let strings: Ptr<NiTArray> = e.new_object();
        e.set(strings, NiTArray::m_usMaxSize, 2);
        e.set(strings, NiTArray::m_usSize, 1);
        e.set(strings, NiTArray::m_usGrowBy, 4);
        let log = Log::default();
        logged(&mut e, &log, VIEWER_STRING_NAME, 0x11);
        logged(&mut e, &log, VIEWER_STRING_POINTER, 0x22);
        logged(&mut e, &log, VIEWER_STRING_UNSIGNED, 0x33);
        logged(&mut e, &log, STRING_ARRAY_SET_SIZE, 0);
        let l = log.clone();
        e.register_double(STRING_ARRAY_SET_AT, move |e, a| {
            l.borrow_mut()
                .push((STRING_ARRAY_SET_AT, vec![a[0], a[1], e.mem.u32(a[2])]));
            // The array grows by one used entry per add.
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            ret(0)
        });
        e.call(0x00a5_d6e0, &args![this, strings]);
        let s = strings.addr();
        assert_eq!(
            *log.borrow(),
            [
                (VIEWER_STRING_NAME, vec![0x0123_4567]),
                (STRING_ARRAY_SET_AT, vec![s, 1, 0x11]),
                (VIEWER_STRING_POINTER, vec![TEXT_THIS, this]),
                (STRING_ARRAY_SET_SIZE, vec![s, 6]),
                (STRING_ARRAY_SET_AT, vec![s, 2, 0x22]),
                (VIEWER_STRING_UNSIGNED, vec![TEXT_REF_COUNT, 7]),
                (STRING_ARRAY_SET_SIZE, vec![s, 7]),
                (STRING_ARRAY_SET_AT, vec![s, 3, 0x33]),
            ]
        );
    }
}
