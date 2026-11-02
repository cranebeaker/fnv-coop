//! `NiMain/nidynamiceffect.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDynamicEffect::ms_RTTI` (the class's `NiRTTI` record).
const NI_DYNAMIC_EFFECT_RTTI: u32 = 0x011f_4988;

// Translated from 00a69060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_dynamic_effect_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DYNAMIC_EFFECT_RTTI)
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiDynamicEffect`'s vtable (`0109c174`; its RTTI type descriptor reads
/// `.?AVNiDynamicEffect@@`).
const NI_DYNAMIC_EFFECT_VTABLE: u32 = 0x0109_c174;
/// `NiDynamicEffect::ms_iNextIndex` (Xbox PDB static; the word at `011a9b58` the constructor
/// increments).
const NEXT_INDEX_011A9B58: u32 = 0x011a_9b58;

// Translated from 00a69320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::NiDynamicEffect` (Xbox PDB), `thiscall`, no stack words. Runs the
/// `NiAVObject` constructor (`00a5aa20`), stores the vtable `0109c174`, then (PC offsets, the
/// Xbox PDB's less 0x24): `m_ucEffectType` (+0x9d) 0xff, `m_uiPushCount` (+0xa4) 0, `m_bOn`
/// (+0x9c) 1, `m_uiRevID` (+0xa8) 1, the two node lists (+0xac..+0xc3) cleared, and
/// `m_iIndex` (+0xa0) the value `InterlockedIncrement` answers for `ms_iNextIndex`
/// (`011a9b58`). Answers `this`.
pub fn ni_dynamic_effect_ni_dynamic_effect(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_DYNAMIC_EFFECT_VTABLE);
    e.mem.set_u8(base + 0x9d, 0xff);
    e.mem.set_u32(base + 0xa4, 0);
    e.mem.set_u8(base + 0x9c, 1);
    e.mem.set_u32(base + 0xa8, 1);
    for offset in [0xb4u32, 0xac, 0xb0, 0xc0, 0xb8, 0xbc] {
        e.mem.set_u32(base + offset, 0);
    }
    let index = e
        .call(IMPORT_INTERLOCKED_INCREMENT, &args![NEXT_INDEX_011A9B58])
        .u32();
    e.mem.set_u32(base + 0xa0, index);
    this
}

/// The shared list-node pool's lock (`011c5f80`: its owner word; the count is
/// at `011c5f84`), taken with `0040fbf0` (`thiscall`, one word: 0).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
/// The lock's count word (`011c5f84`).
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
/// The pool's free-node head (`011c5f58`).
const NODE_POOL_FREE: u32 = 0x011c_5f58;
/// `0040fbf0` (`thiscall` on the lock, `RET 4`): takes the lock.
const LOCK_TAKE: u32 = 0x0040_fbf0;
/// `0043a080` (no arguments): refills the empty free-node pool.
const NODE_POOL_REFILL: u32 = 0x0043_a080;

/// The inline release of the pool lock: one count less, and the owner cleared
/// when the count reaches 0.
fn release_node_pool(e: &mut Engine) {
    let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
    if count == 0 {
        e.set_global::<u32>(NODE_POOL_LOCK, 0);
    }
}

// Translated from 00a68e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): empties the list
/// at `+0xb8` (head `+0xb8`, tail `+0xbc`, count `+0xc0`; nodes are next,
/// previous, item). While the count is not 0: unlinks the head node (the next
/// node's previous link, or the tail, cleared), clears its item, takes the
/// node pool's lock (`0040fbf0` on `011c5f80`), clears its previous link, puts
/// it on the free list `011c5f58`, releases the lock and counts one down.
pub fn fn_00a68e30(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    while e.mem.u32(t + 0xc0) != 0 {
        let node = e.mem.u32(t + 0xb8);
        let next = e.mem.u32(node);
        e.mem.set_u32(t + 0xb8, next);
        if next != 0 {
            e.mem.set_u32(next + 4, 0);
        } else {
            e.mem.set_u32(t + 0xbc, 0);
        }
        e.mem.set_u32(node + 8, 0);
        e.call(LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
        e.mem.set_u32(node + 4, 0);
        let free = e.global::<u32>(NODE_POOL_FREE);
        e.mem.set_u32(node, free);
        let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
        e.set_global::<u32>(NODE_POOL_FREE, node);
        if count == 0 {
            e.set_global::<u32>(NODE_POOL_LOCK, 0);
        }
        let remaining = e.mem.u32(t + 0xc0).wrapping_sub(1);
        e.mem.set_u32(t + 0xc0, remaining);
    }
}

/// `NiTPointerListBase<..>::RemoveAll` (`00b71300`, `thiscall`, no stack words).
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// `00a5a390` (`niavobject.cpp`, `thiscall`, no stack words): the `NiAVObject`
/// destructor body.
const NI_AV_OBJECT_DESTRUCT: u32 = 0x00a5_a390;

// Translated from 00a69030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiDynamicEffect` destructor body
/// (`thiscall`, no stack words): stores the vtable `0109c174`, empties the
/// list at `+0xb8` (`00a68e30`), runs `RemoveAll` on the lists at `+0xb8` and
/// `+0xac`, then jumps to the `NiAVObject` destructor body `00a5a390`.
pub fn fn_00a69030(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_DYNAMIC_EFFECT_VTABLE);
    fn_00a68e30(e, this);
    e.call(POINTER_LIST_REMOVE_ALL, &args![this.addr() + 0xb8]);
    e.call(POINTER_LIST_REMOVE_ALL, &args![this.addr() + 0xac]);
    e.call(NI_AV_OBJECT_DESTRUCT, &args![this]);
}

/// `NiAVObject::LoadBinary` (`00a5a510`, `thiscall`, the stream).
const NI_AV_OBJECT_LOAD_BINARY: u32 = 0x00a5_a510;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`, `thiscall` on the stream, no
/// stack words).
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;

/// One read through the stream's binary reader (the object at
/// `stream+0x24c`): its function at `+8`, `cdecl`, with the reader, the
/// destination, the size, the address of the count word (set to the size
/// first) and 1; the result is added to the reader's `+4`.
fn read_items(e: &mut Engine, stream: u32, destination: u32, size: u32, count_word: u32) {
    let reader = e.mem.u32(stream + 0x24c);
    let read = e.mem.u32(reader + 8);
    e.mem.set_u32(count_word, size);
    let done = e
        .call(read, &args![reader, destination, size, count_word, 1u32])
        .u32();
    let total = e.mem.u32(reader + 4).wrapping_add(done);
    e.mem.set_u32(reader + 4, total);
}

// Translated from 00a68ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::LoadBinary` (Xbox PDB), `thiscall`, one stack word (the
/// stream). `NiAVObject::LoadBinary`, then by the stream's file version
/// (`stream+0xd8`): from 0x0a010066 one byte (read into the argument's own
/// stack slot) sets `m_bOn` (+0x9c) to whether it is not 0; below 0x04010000 a
/// 4-byte count (into the same slot) and that many 4-byte items (into a stack
/// word, discarded); from 0x0a00010e `NiStream::ReadMultipleLinkIDs`. Reads go
/// through the stream's binary reader (`stream+0x24c`).
pub fn ni_dynamic_effect_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let s = stream.addr();
    e.call(NI_AV_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(12, |e, frame| {
        // The frame: the count word (+0), the discarded item (+4), the
        // argument's slot (+8), which still holds the stream pointer.
        let count_word = frame.addr();
        let item = count_word + 4;
        let slot = count_word + 8;
        e.mem.set_u32(slot, s);
        if e.mem.u32(s + 0xd8) >= 0x0a01_0066 {
            read_items(e, s, slot, 1, count_word);
            let on = e.mem.u8(slot) != 0;
            e.mem.set_u8(this.addr() + 0x9c, u8::from(on));
        }
        if e.mem.u32(s + 0xd8) < 0x0401_0000 {
            read_items(e, s, slot, 4, count_word);
            let mut index: i32 = 0;
            while index < e.mem.u32(slot) as i32 {
                read_items(e, s, item, 4, count_word);
                index += 1;
            }
        }
    });
    if e.mem.u32(s + 0xd8) >= 0x0a00_010e {
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    }
}

/// `NiAVObject::LinkObject` (`00a5a7f0`, `thiscall`, the stream).
const NI_AV_OBJECT_LINK_OBJECT: u32 = 0x00a5_a7f0;
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`, `thiscall` on the stream).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;

// Translated from 00a69070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::LinkObject` (Xbox PDB), `thiscall`, one stack word (the
/// stream). `NiAVObject::LinkObject`, then from file version 0x0a00010e
/// (`stream+0xd8`) the stream's link count (`GetNumberOfLinkIDs`) objects
/// (`GetObjectFromLinkID`); each one that is not null goes to the front of the
/// list at `+0xb8` (head `+0xb8`, tail `+0xbc`, count `+0xc0`) in a node from
/// the shared pool: under the pool lock (`0040fbf0` on `011c5f80`), the free
/// list `011c5f58` (refilled by `0043a080` when empty) gives the node, which is
/// cleared, the lock released, and then filled (item, no previous link, the old
/// head as next).
pub fn ni_dynamic_effect_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(NI_AV_OBJECT_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) < 0x0a00_010e {
        return;
    }
    let mut remaining = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    while remaining != 0 {
        remaining = remaining.wrapping_sub(1);
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if object == 0 {
            continue;
        }
        e.call(LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
        let mut node = e.global::<u32>(NODE_POOL_FREE);
        if node == 0 {
            e.call(NODE_POOL_REFILL, &args![]);
            node = e.global::<u32>(NODE_POOL_FREE);
        }
        let next_free = e.mem.u32(node);
        e.set_global::<u32>(NODE_POOL_FREE, next_free);
        e.mem.set_u32(node + 8, 0);
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, 0);
        release_node_pool(e);
        e.mem.set_u32(node + 8, object);
        e.mem.set_u32(node + 4, 0);
        let head = e.mem.u32(t + 0xb8);
        e.mem.set_u32(node, head);
        if head != 0 {
            e.mem.set_u32(head + 4, node);
        } else {
            e.mem.set_u32(t + 0xbc, node);
        }
        let count = e.mem.u32(t + 0xc0).wrapping_add(1);
        e.mem.set_u32(t + 0xc0, count);
        e.mem.set_u32(t + 0xb8, node);
    }
}

/// `NiAVObject::IsEqual` (`00a5a260`, the other object), `NiAVObject::SaveBinary`
/// (`00a5a900`, the stream) and `NiAVObject::GetViewerStrings` (`00a5ad00`, the string
/// array), all `thiscall`.
const NI_AV_OBJECT_IS_EQUAL: u32 = 0x00a5_a260;
const NI_AV_OBJECT_SAVE_BINARY: u32 = 0x00a5_a900;
const NI_AV_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_ad00;
/// The sized delete (`00aa1460`, `cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiGetViewerString` (`00a7d330`, the class name) and `NiGetViewerString_ov10`
/// (`00a7d5a0`, a name and an object), both `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_OBJECT: u32 = 0x00a7_d5a0;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The formats `"affected node[%d]"` and `"unaffected node[%d]"`.
const FORMAT_AFFECTED_NODE: u32 = 0x0109_c264;
const FORMAT_UNAFFECTED_NODE: u32 = 0x0109_c250;
/// The two node lists (`NiTPointerList`: head, tail, count; nodes next +0, previous +4,
/// item +8): the affected nodes at +0xac and the unaffected nodes at +0xb8, by the names
/// `GetViewerStrings` gives them.
const AFFECTED_NODES: u32 = 0xac;
const UNAFFECTED_NODES: u32 = 0xb8;

/// Appends `text` to a viewer-string array with the set-at inlined (`NiTArray::SetAt`):
/// the array grows to its step (+0xe) plus its size (+0xa) when the size has reached the
/// capacity (+0x8); after the grow the size is read again; storing at or past it makes the
/// size the index plus one and counts a non-null string in the 16-bit used count (+0xc);
/// storing below it adjusts the used count by the change between the old element and
/// `text`. The string goes into the elements (+0x4) at the old size.
fn push_viewer_string_inline(e: &mut Engine, strings: Ptr, text: u32) {
    let base = strings.addr();
    let index = u32::from(e.mem.u16(base + 0xa));
    if index >= u32::from(e.mem.u16(base + 8)) {
        let step = u32::from(e.mem.u16(base + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + index]);
    }
    let size = u32::from(e.mem.u16(base + 0xa));
    if index >= size {
        e.mem.set_u16(base + 0xa, (index + 1) as u16);
        if text != 0 {
            let used = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, used.wrapping_add(1));
        }
    } else {
        let old = e.mem.u32(e.mem.u32(base + 4).wrapping_add(4 * index));
        if text != 0 {
            if old == 0 {
                let used = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, used.wrapping_add(1));
            }
        } else if old != 0 {
            let used = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, used.wrapping_add(0xffff));
        }
    }
    let elements = e.mem.u32(base + 4);
    e.mem.set_u32(elements.wrapping_add(4 * index), text);
}

/// The two lists' node walk of `IsEqual`: from this list's head, each node's item and the
/// other list's node's item must both be null or both not null (the items themselves are
/// not compared); the other list is followed in step without its own end check.
fn same_items(e: &Engine, mine: u32, theirs: u32) -> bool {
    let mut node = e.mem.u32(mine);
    let mut other = e.mem.u32(theirs);
    while node != 0 {
        let item = e.mem.u32(node + 8);
        let next = e.mem.u32(node);
        let other_item = e.mem.u32(other.wrapping_add(8));
        other = e.mem.u32(other);
        if (item != 0) != (other_item != 0) {
            return false;
        }
        node = next;
    }
    true
}

// Translated from 00a68d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::IsEqual` (Xbox PDB): `NiAVObject::IsEqual`, then for each node list
/// (+0xac, +0xb8) the counts must match and, when the count is not 0 and this list has a
/// head, the nodes must have items in the same places (null or not null).
pub fn ni_dynamic_effect_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_AV_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let t = this.addr();
    let o = other.addr();
    for list in [AFFECTED_NODES, UNAFFECTED_NODES] {
        let count = e.mem.u32(t + list + 8);
        if count != e.mem.u32(o.wrapping_add(list + 8)) {
            return false;
        }
        if count != 0 && !same_items(e, t + list, o.wrapping_add(list)) {
            return false;
        }
    }
    true
}

// Translated from 00a68f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::SaveBinary` (Xbox PDB): `NiAVObject::SaveBinary`, then through the
/// stream's output (`m_pkOstr`, its write function, the position advanced by each result)
/// the on byte (+0x9c, 1 byte, written from the stream argument's stack word) and the
/// unaffected-node count (+0xc0, 4 bytes from a local); when that local is above 0 (signed)
/// each node of the list at +0xb8, from its tail back through the previous links, has its
/// item saved through the stream's virtual slot +0x30.
pub fn ni_dynamic_effect_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(NI_AV_OBJECT_SAVE_BINARY, &args![this, stream]);
    let on = e.mem.u8(t + 0x9c);
    let count = e.mem.u32(t + UNAFFECTED_NODES + 8);
    let saved = e.with_stack(0xc, |e, locals| {
        // locals: +0 the stream argument's word, +4 the count local, +8 the size local.
        let word = locals.addr();
        e.mem.set_u8(word, on);
        write(e, stream, word, 1, word + 8);
        e.mem.set_u32(word + 4, count);
        write(e, stream, word + 4, 4, word + 8);
        e.mem.u32(word + 4)
    });
    if (saved as i32) <= 0 {
        return;
    }
    let mut node = e.mem.u32(t + UNAFFECTED_NODES + 4);
    while node != 0 {
        let item = e.mem.u32(node + 8);
        node = e.mem.u32(node + 4);
        e.vcall(stream.addr(), 0x30, &args![item]);
    }
}

/// One write through the stream's output: its write function (`m_pfnWrite` +0xc of
/// `m_pkOstr` +0x250, `cdecl`: (output, data, bytes, address of the size word, 1)) with the
/// size word at `size_word` set to `bytes`; the position (+4) advanced by the result.
fn write(e: &mut Engine, stream: Ptr, data: u32, bytes: u32, size_word: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(size_word, bytes);
    let done = e
        .call(function, &args![output, data, bytes, size_word, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a69140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a69140 - the /GS cookie check (00ec408c) guarding the 0x40-byte text buffer is not modeled
/// `NiDynamicEffect::GetViewerStrings` (Xbox PDB): `NiAVObject::GetViewerStrings`, the
/// class name; then, only when the affected-node count (+0xb4) is not 0, one string per
/// node of the affected list (+0xac) and then of the unaffected list (+0xb8), each named
/// `"affected node[%d]"` / `"unaffected node[%d]"` (`NiSprintf` into a 0x40-byte stack
/// buffer, one index counting through both lists) with the node's item
/// (`NiGetViewerString_ov10`), appended with the array's set-at inlined.
pub fn ni_dynamic_effect_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let t = this.addr();
    e.call(NI_AV_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_DYNAMIC_EFFECT_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
    if e.mem.u32(t + AFFECTED_NODES + 8) == 0 {
        return;
    }
    let mut index = 0u32;
    for (list, format) in [
        (AFFECTED_NODES, FORMAT_AFFECTED_NODE),
        (UNAFFECTED_NODES, FORMAT_UNAFFECTED_NODE),
    ] {
        let mut node = e.mem.u32(t + list);
        while node != 0 {
            let item = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            let text = e.with_stack(0x40, |e, buffer| {
                e.call(NI_SPRINTF, &args![buffer, 0x40u32, format, index]);
                e.call(VIEWER_STRING_OBJECT, &args![buffer, item]).u32()
            });
            push_viewer_string_inline(e, strings, text);
            index = index.wrapping_add(1);
        }
    }
}

// Translated from 00a69390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::_scalar_deleting_destructor_` (Xbox PDB): the destructor body
/// (`00a69030`: vtable, the unaffected list emptied, both lists' `RemoveAll`, the
/// `NiAVObject` destructor) inlined, then the sized delete of 0xc4 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_dynamic_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00a69030(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xc4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_9320, ni_dynamic_effect_ni_dynamic_effect(Ptr) -> Ptr),
        entry!(0x00a69060, ni_dynamic_effect_get_rtti() -> Ptr),
        entry!(0x00a6_8e30, fn_00a68e30(Ptr)),
        entry!(0x00a6_9030, fn_00a69030(Ptr)),
        entry!(0x00a6_8ea0, ni_dynamic_effect_load_binary(Ptr, Ptr)),
        entry!(0x00a6_9070, ni_dynamic_effect_link_object(Ptr, Ptr)),
        entry!(0x00a6_8d80, ni_dynamic_effect_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a6_8f80, ni_dynamic_effect_save_binary(Ptr, Ptr)),
        entry!(0x00a6_9140, ni_dynamic_effect_get_viewer_strings(Ptr, Ptr)),
        entry!(
            0x00a6_9390,
            ni_dynamic_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dynamic_effect_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a69060, &args![]).u32(), NI_DYNAMIC_EFFECT_RTTI);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn ni_dynamic_effect_ni_dynamic_effect_sets_the_fields_and_takes_the_next_index() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.map(0x011a_9000, 0x1000);
        e.set_global::<u32>(NEXT_INDEX_011A9B58, 41);
        let this = e.mem.alloc(0xd0);
        for offset in (0x9cu32..0xc4).step_by(4) {
            e.mem.set_u32(this + offset, 0x5555_5555);
        }
        assert_eq!(e.call(0x00a6_9320, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DYNAMIC_EFFECT_VTABLE);
        assert_eq!(e.mem.u8(this + 0x9c), 1);
        assert_eq!(e.mem.u8(this + 0x9d), 0xff);
        assert_eq!(e.mem.u8(this + 0x9e), 0x55);
        assert_eq!(e.mem.u32(this + 0xa0), 42);
        assert_eq!(e.global::<u32>(NEXT_INDEX_011A9B58), 42);
        assert_eq!(e.mem.u32(this + 0xa4), 0);
        assert_eq!(e.mem.u32(this + 0xa8), 1);
        for offset in (0xacu32..0xc4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    fn pool(e: &mut Engine) {
        e.map(NODE_POOL_FREE, 4);
        e.map(NODE_POOL_LOCK, 8);
        e.register(LOCK_TAKE, |e, _| {
            let count = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
            e.set_global::<u32>(NODE_POOL_LOCK, 0x77);
            Ret::default()
        });
    }

    #[test]
    fn fn_00a68e30_returns_every_node_to_the_pool() {
        let mut e = Engine::new();
        pool(&mut e);
        e.set_global::<u32>(NODE_POOL_FREE, 0x5550);
        let this = e.mem.alloc(0xc4);
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        e.mem.set_u32(a, b);
        e.mem.set_u32(a + 8, 0x111);
        e.mem.set_u32(b + 4, a);
        e.mem.set_u32(b + 8, 0x222);
        e.mem.set_u32(this + 0xb8, a);
        e.mem.set_u32(this + 0xbc, b);
        e.mem.set_u32(this + 0xc0, 2);
        e.call(0x00a6_8e30, &args![this]);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE), b);
        assert_eq!(e.mem.u32(b), a);
        assert_eq!(e.mem.u32(a), 0x5550);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(b + 8)), (0, 0));
        assert_eq!(e.global::<u32>(NODE_POOL_LOCK), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_LOCK_COUNT), 0);
    }

    #[test]
    fn fn_00a69030_runs_the_parts_in_order() {
        let mut e = Engine::new();
        e.register(POINTER_LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        e.call(0x00a6_9030, &args![this]);
        assert_eq!(e.mem.u32(this), NI_DYNAMIC_EFFECT_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (POINTER_LIST_REMOVE_ALL, vec![this + 0xb8]),
                (POINTER_LIST_REMOVE_ALL, vec![this + 0xac]),
                (NI_AV_OBJECT_DESTRUCT, vec![this]),
            ]
        );
    }

    const READ: u32 = 0x0123_6000;

    /// A stream at `version` whose reader answers `values` in turn (each a
    /// word written to the destination; answers the size).
    fn stream(e: &mut Engine, version: u32, values: Vec<u32>) -> u32 {
        let mut values = values.into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            let v = values.next().unwrap();
            if a[2] == 1 {
                e.mem.set_u8(a[1], v as u8);
            } else {
                e.mem.set_u32(a[1], v);
            }
            a[2].into_ret()
        });
        let reader = e.mem.alloc(0x10);
        e.mem.set_u32(reader + 8, READ);
        let s = e.mem.alloc(0x250);
        e.mem.set_u32(s + 0x24c, reader);
        e.mem.set_u32(s + 0xd8, version);
        s
    }

    fn load(version: u32, values: Vec<u32>) -> (Engine, u32, u32, Vec<u32>) {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        let s = stream(&mut e, version, values);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u8(this + 0x9c, 7);
        e.call_log = Some(vec![]);
        e.call(0x00a6_8ea0, &args![this, s]);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|c| c.0)
            .collect();
        (e, this, s, log)
    }

    #[test]
    fn ni_dynamic_effect_load_binary_new_files_read_the_on_flag() {
        let (e, this, s, log) = load(0x1402_0007, vec![2]);
        assert_eq!(e.mem.u8(this + 0x9c), 1);
        let reader = e.mem.u32(s + 0x24c);
        assert_eq!(e.mem.u32(reader + 4), 1);
        assert_eq!(
            log,
            vec![
                0x00a6_8ea0,
                NI_AV_OBJECT_LOAD_BINARY,
                READ,
                STREAM_READ_MULTIPLE_LINK_IDS
            ]
        );
        let (e, this, _, _) = load(0x0a01_0066, vec![0]);
        assert_eq!(e.mem.u8(this + 0x9c), 0);
    }

    #[test]
    fn ni_dynamic_effect_load_binary_old_files_skip_the_counted_items() {
        let (e, this, s, log) = load(0x0400_0002, vec![2, 9, 9]);
        assert_eq!(e.mem.u8(this + 0x9c), 7);
        let reader = e.mem.u32(s + 0x24c);
        assert_eq!(e.mem.u32(reader + 4), 12);
        assert_eq!(
            log,
            vec![0x00a6_8ea0, NI_AV_OBJECT_LOAD_BINARY, READ, READ, READ]
        );
        let (_, _, _, log) = load(0x0a00_0100, vec![]);
        assert_eq!(log, vec![0x00a6_8ea0, NI_AV_OBJECT_LOAD_BINARY]);
    }

    #[test]
    fn ni_dynamic_effect_link_object_puts_linked_objects_in_front() {
        let mut e = Engine::new();
        pool(&mut e);
        let spare = e.mem.alloc(12);
        let refill_node = e.mem.alloc(12);
        e.set_global::<u32>(NODE_POOL_FREE, spare);
        e.register(NI_AV_OBJECT_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| 3u32.into_ret());
        let mut objects = vec![0x300u32, 0, 0x100].into_iter();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            objects.next().unwrap().into_ret()
        });
        e.register_double(NODE_POOL_REFILL, move |e, _| {
            e.set_global::<u32>(NODE_POOL_FREE, refill_node);
            Ret::default()
        });
        let s = e.mem.alloc(0x250);
        e.mem.set_u32(s + 0xd8, 0x0a00_010e);
        let this = e.mem.alloc(0xc4);
        e.call(0x00a6_9070, &args![this, s]);
        assert_eq!(e.mem.u32(this + 0xc0), 2);
        let head = e.mem.u32(this + 0xb8);
        assert_eq!(head, refill_node);
        assert_eq!(e.mem.u32(this + 0xbc), spare);
        assert_eq!(e.mem.u32(head + 8), 0x100);
        assert_eq!(e.mem.u32(head), spare);
        assert_eq!(e.mem.u32(spare + 4), head);
        assert_eq!(e.mem.u32(spare + 8), 0x300);
        assert_eq!(e.global::<u32>(NODE_POOL_LOCK_COUNT), 0);

        e.call_log = Some(vec![]);
        e.mem.set_u32(s + 0xd8, 0x0a00_010d);
        e.call(0x00a6_9070, &args![this, s]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Builds the list at `list` of `object` from `items` (head to tail).
    fn list(e: &mut Engine, object: u32, list: u32, items: &[u32]) {
        let mut previous = 0u32;
        for item in items {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 4, previous);
            e.mem.set_u32(node + 8, *item);
            if previous == 0 {
                e.mem.set_u32(object + list, node);
            } else {
                e.mem.set_u32(previous, node);
            }
            previous = node;
        }
        e.mem.set_u32(object + list + 4, previous);
        e.mem.set_u32(object + list + 8, items.len() as u32);
    }

    #[test]
    fn is_equal_compares_counts_and_item_places() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc4);
        let other = e.mem.alloc(0xc4);
        list(&mut e, this, AFFECTED_NODES, &[1, 0, 3]);
        list(&mut e, other, AFFECTED_NODES, &[7, 0, 9]);
        list(&mut e, this, UNAFFECTED_NODES, &[5]);
        list(&mut e, other, UNAFFECTED_NODES, &[6]);
        assert!(e.call(0x00a6_8d80, &args![this, other]).bool());
        list(&mut e, other, UNAFFECTED_NODES, &[0]);
        assert!(!e.call(0x00a6_8d80, &args![this, other]).bool());
        list(&mut e, other, UNAFFECTED_NODES, &[6, 6]);
        assert!(!e.call(0x00a6_8d80, &args![this, other]).bool());
        list(&mut e, other, UNAFFECTED_NODES, &[6]);
        list(&mut e, other, AFFECTED_NODES, &[7, 8, 9]);
        assert!(!e.call(0x00a6_8d80, &args![this, other]).bool());
        // A count with no head is not walked.
        list(&mut e, other, AFFECTED_NODES, &[7, 0, 9]);
        e.mem.set_u32(this + AFFECTED_NODES, 0);
        assert!(e.call(0x00a6_8d80, &args![this, other]).bool());
        e.register(NI_AV_OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a6_8d80, &args![this, this]).bool());
    }

    #[test]
    fn save_binary_writes_on_and_count_then_links_from_the_tail() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let first = e.mem.u32(a[1]);
            e.mem.set_u32(0x0e70_0000 + 4 * a[2], first);
            a[2].into_ret()
        });
        e.map(0x0e70_0000, 0x1000);
        let mut slots = vec![0u32; 0x10];
        slots[0x30 / 4] = 0x0e00_0030;
        e.put_vtable(0x0e10_0000, &slots);
        e.register(0x0e00_0030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream, 0x0e10_0000);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u8(this + 0x9c, 1);
        list(&mut e, this, UNAFFECTED_NODES, &[0x11, 0x22, 0x33]);
        e.call(0x00a6_8f80, &args![this, stream]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u8(0x0e70_0004), 1);
        assert_eq!(e.mem.u32(0x0e70_0010), 3);
        assert_eq!(e.mem.u32(output + 4), 5);
        assert_eq!(
            calls_to(&e, 0x0e00_0030),
            vec![vec![stream, 0x33], vec![stream, 0x22], vec![stream, 0x11]]
        );
        // A count of 0 (or below, as a signed word) saves no links.
        e.mem.set_u32(this + UNAFFECTED_NODES + 8, 0x8000_0000);
        e.call(0x00a6_8f80, &args![this, stream]);
        assert_eq!(calls_to(&e, 0x0e00_0030).len(), 3);
    }

    #[test]
    fn get_viewer_strings_names_both_lists_with_one_index() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_DYNAMIC_EFFECT_RTTI, 0x0123_0000);
        for double in [
            NI_AV_OBJECT_GET_VIEWER_STRINGS,
            NI_SPRINTF,
            STRINGS_GROW,
            STRINGS_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_OBJECT, |_, a| (0x0e10_0000 + a[1]).into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc4);
        list(&mut e, this, AFFECTED_NODES, &[0x11, 0x22]);
        list(&mut e, this, UNAFFECTED_NODES, &[0x33]);
        let strings = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, elements);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00a6_9140, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 0]);
        let printed: Vec<(u32, u32, u32)> = calls_to(&e, NI_SPRINTF)
            .iter()
            .map(|c| (c[1], c[2], c[3]))
            .collect();
        assert_eq!(
            printed,
            vec![
                (0x40, FORMAT_AFFECTED_NODE, 0),
                (0x40, FORMAT_AFFECTED_NODE, 1),
                (0x40, FORMAT_UNAFFECTED_NODE, 2)
            ]
        );
        let items: Vec<u32> = calls_to(&e, VIEWER_STRING_OBJECT)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(items, vec![0x11, 0x22, 0x33]);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert_eq!(e.mem.u32(elements + 8), 0x0e10_0033);
        // No affected nodes: neither list is listed.
        e.mem.set_u32(this + AFFECTED_NODES + 8, 0);
        e.call(0x00a6_9140, &args![this, strings]);
        assert_eq!(calls_to(&e, NI_SPRINTF).len(), 3);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = Engine::new();
        for double in [POINTER_LIST_REMOVE_ALL, NI_AV_OBJECT_DESTRUCT, SIZED_DELETE] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc4);
        assert_eq!(e.call(0x00a6_9390, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DYNAMIC_EFFECT_VTABLE);
        assert_eq!(
            calls_to(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![this + 0xb8], vec![this + 0xac]]
        );
        assert_eq!(calls_to(&e, NI_AV_OBJECT_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0xc4]]);
        e.call(0x00a6_9390, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }
}
