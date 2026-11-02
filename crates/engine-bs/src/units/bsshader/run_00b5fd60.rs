//! `BSShader/run_00b5fd60` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::{NiTListItem, NiTPointerList};

/// The list-item pool's lock (`011c5f80`: the `this` of the lock enter `0040fbf0`, `thiscall`
/// with one word, and the lock's owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, and the head of the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00b602d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b5fd60`): an `NiTPointerList` remove-head, `thiscall`, no
/// stack words. The head item's next item becomes the head (its previous cleared; the tail
/// cleared instead when there is none); the item's element (+8) is taken and cleared, the item
/// goes back to the item pool inline (the pool's lock entered through `0040fbf0` on `011c5f80`
/// with 0, the item's +4 cleared, the item pushed on the free list at `011c5f58`, the lock
/// left: its count at `011c5f84` down by one and the owner word cleared at zero) and the
/// list's count goes down by one. Answers the element.
pub fn fn_00b602d0(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let item: Ptr<NiTListItem> = Ptr::new(e.get(this, NiTPointerList::m_pkHead));
    let next = e.get(item, NiTListItem::m_pkNext);
    e.set(this, NiTPointerList::m_pkHead, next);
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(4), 0);
    } else {
        e.set(this, NiTPointerList::m_pkTail, 0);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.set(item, NiTListItem::m_element, 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.set(item, NiTListItem::m_pkPrev, 0);
    let head = e.mem.u32(NODE_POOL_FREE_LIST);
    e.set(item, NiTListItem::m_pkNext, head);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, item.addr());
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
    let items = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, items);
    element
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_assign, ni_pointer_clear, release_reference, IMPORT_INTERLOCKED_DECREMENT,
    IMPORT_INTERLOCKED_INCREMENT, MEMORY_ALLOCATE, MEMORY_MANAGER,
};

/// The two locks `fn_00b5fd60` enters (`0040fbf0` with 0 on each) and their entry counts,
/// which it decrements at the end (the owner word cleared at zero).
const LIGHT_LOCK_011F9EA0: u32 = 0x011f_9ea0;
const LIGHT_LOCK_COUNT_011F9EA4: u32 = 0x011f_9ea4;
const OBJECT_LOCK_011F9EC0: u32 = 0x011f_9ec0;
const OBJECT_LOCK_COUNT_011F9EC4: u32 = 0x011f_9ec4;
/// The callees of `fn_00b5fd60` (engine map unit `shadowscenenode.cpp`; `thiscall` with one
/// stack word unless said otherwise).
const LIGHT_UPDATE_00B5CD00: u32 = 0x00b5_cd00;
const LIGHT_UPDATE_00B5ECA0: u32 = 0x00b5_eca0;
const LIGHT_UPDATE_00B5D930: u32 = 0x00b5_d930;
const LIGHT_UPDATE_00B5D300: u32 = 0x00b5_d300;
/// `ShadowSceneNode::RemoveQueuedLight_ov2` (engine map), `thiscall`, one stack word (the
/// address of an `NiPointer`).
const REMOVE_QUEUED_LIGHT: u32 = 0x00b5_ef90;
/// `ShadowSceneNode::UpdateQueuedObjectLightingStart` (engine map), `cdecl`, four words.
const UPDATE_QUEUED_OBJECT_LIGHTING_START: u32 = 0x00b5_dac0;
/// `00b6d4c0` (unnamed, `thiscall` on the object `011f91a8` holds, one stack word).
const RELEASE_00B6D4C0: u32 = 0x00b6_d4c0;
const HOLDER_011F91A8: u32 = 0x011f_91a8;
/// `00c5b8d0` (unnamed, engine map unit `bsportalgraph`, `thiscall`, no stack words).
const PORTAL_GRAPH_00C5B8D0: u32 = 0x00c5_b8d0;
/// `NiTPointerListBase<..>::RemoveAll` (engine map, `00e74d40`, `thiscall`, no stack words).
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// The 150.0 (`010231d0`) the object distance is compared with, and the default bound
/// (`011f4288`) used when an object has none at `+0x20`.
const DISTANCE_LIMIT_010231D0: u32 = 0x0102_31d0;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;

/// The element (`+8`) and the next item (`+0`) of a list item.
fn item_parts(e: &Engine, item: u32) -> (u32, u32) {
    (e.mem.u32(item.wrapping_add(8)), e.mem.u32(item))
}

/// The last argument of `UpdateQueuedObjectLightingStart` for an object: 1 when its vtable
/// slot +0xc answers non-zero and 150.0 is above the float at `+0xc` of its bound (`+0x20`,
/// or `011f4288` when null; `FCOMP` with `TEST AH,0x41`, so not when unordered), else 0.
fn near_flag(e: &mut Engine, object: u32) -> u32 {
    if e.vcall(object, 0xc, &args![]).u32() == 0 {
        return 0;
    }
    let mut bound = e.mem.u32(object.wrapping_add(0x20));
    if bound == 0 {
        bound = DEFAULT_BOUND_011F4288;
    }
    let limit = e.global::<f32>(DISTANCE_LIMIT_010231D0);
    let value = e.mem.f32(bound.wrapping_add(0xc));
    u32::from(limit > value)
}

// Translated from 00b5fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a shadow scene node, no stack words): processes
/// the node's queues. Enters the lock `011f9ea0` (`0040fbf0` with 0); for each light of the list
/// at `+0xe4` calls `00b5cd00` (byte `+0xec` of the light set) or `00b5eca0` with it, for
/// each of the list at `+0xfc` `00b5d930` (byte `+0xfc` set) or `00b5d300`; for each light of the
/// list at `+0xf0` keeps it in a local `NiPointer` (count up) and, when its byte `+0xec` is set,
/// first calls `00b6d4c0` on the object `011f91a8` holds with the light's `+0x10c` and clears that
/// `NiPointer`; then `RemoveQueuedLight_ov2` with the local, which is released after. Enters the
/// lock `011f9ec0`; for each object of the lists at `+0x114` and `+0x108` calls
/// `UpdateQueuedObjectLightingStart` (object, node, 1 for `+0x114` and 0 for `+0x108`, the near
/// flag). Then `00c5b8d0` on `+0x1e0` when that is not null, `RemoveAll` on the five lists, the
/// byte `+0x1fc` set to 1, and both locks left (their counts down by one, the owner word cleared
/// at zero; the object lock first).
// Translation incomplete: 00b5fd60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the local NiPointers) is not translated.
pub fn fn_00b5fd60(e: &mut Engine, this: Ptr) {
    let node = this.addr();
    e.call(LOCK_ENTER, &args![LIGHT_LOCK_011F9EA0, 0u32]);
    let mut item = e.mem.u32(node + 0xe4);
    while item != 0 {
        let (light, next) = item_parts(e, item);
        item = next;
        if light != 0 {
            let callee = if e.mem.u8(light + 0xec) != 0 {
                LIGHT_UPDATE_00B5CD00
            } else {
                LIGHT_UPDATE_00B5ECA0
            };
            e.call(callee, &args![node, light]);
        }
    }
    let mut item = e.mem.u32(node + 0xfc);
    while item != 0 {
        let (light, next) = item_parts(e, item);
        item = next;
        if light != 0 {
            let callee = if e.mem.u8(light + 0xfc) != 0 {
                LIGHT_UPDATE_00B5D930
            } else {
                LIGHT_UPDATE_00B5D300
            };
            e.call(callee, &args![node, light]);
        }
    }
    let mut item = e.mem.u32(node + 0xf0);
    while item != 0 {
        let (light, next) = item_parts(e, item);
        item = next;
        if light == 0 {
            continue;
        }
        let shadowed = e.mem.u8(light + 0xec) != 0;
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), light);
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light + 4]);
            if shadowed {
                let held = e.mem.u32(light + 0x10c);
                let holder = e.mem.u32(HOLDER_011F91A8);
                e.call(RELEASE_00B6D4C0, &args![holder, held]);
                ni_pointer_clear(e, light + 0x10c);
            }
            e.call(REMOVE_QUEUED_LIGHT, &args![node, local]);
            let kept = e.mem.u32(local.addr());
            if kept != 0 {
                release_reference(e, kept);
            }
        });
    }
    e.call(LOCK_ENTER, &args![OBJECT_LOCK_011F9EC0, 0u32]);
    for (list, first) in [(0x114u32, 1u32), (0x108, 0)] {
        let mut item = e.mem.u32(node + list);
        while item != 0 {
            let (object, next) = item_parts(e, item);
            item = next;
            if object != 0 {
                let near = near_flag(e, object);
                e.call(
                    UPDATE_QUEUED_OBJECT_LIGHTING_START,
                    &args![object, node, first, near],
                );
            }
        }
    }
    let graph = e.mem.u32(node + 0x1e0);
    if graph != 0 {
        e.call(PORTAL_GRAPH_00C5B8D0, &args![graph]);
    }
    for list in [0xe4u32, 0xf0, 0xfc, 0x108, 0x114] {
        e.call(LIST_REMOVE_ALL, &args![node + list]);
    }
    e.mem.set_u8(node + 0x1fc, 1);
    for (lock, count) in [
        (OBJECT_LOCK_011F9EC0, OBJECT_LOCK_COUNT_011F9EC4),
        (LIGHT_LOCK_011F9EA0, LIGHT_LOCK_COUNT_011F9EA4),
    ] {
        let left = e.mem.u32(count).wrapping_sub(1);
        e.mem.set_u32(count, left);
        if left == 0 {
            e.mem.set_u32(lock, 0);
        }
    }
}

/// The four node pointers `fn_00b60040` walks (`011f91c8` up to `011f91d8`).
const NODES_011F91C8: u32 = 0x011f_91c8;
const NODES_END_011F91D8: u32 = 0x011f_91d8;

// Translated from 00b60040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): runs `fn_00b5fd60` on each non-null node
/// of the four words at `011f91c8`.
pub fn fn_00b60040(e: &mut Engine) {
    let mut slot = NODES_011F91C8;
    while slot < NODES_END_011F91D8 {
        let node = e.mem.u32(slot);
        if node != 0 {
            fn_00b5fd60(e, Ptr::new(node));
        }
        slot += 4;
    }
}

/// `00af4250` (unnamed, engine map unit `archive.cpp`, `cdecl`, three words: a source, the
/// buffer and its size).
const NAME_INTO_00AF4250: u32 = 0x00af_4250;

// Translated from 00b60060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): fills a 0x104-byte stack buffer
/// through `00af4250` (the word `+0x28`, the buffer, 0x104), then calls vtable slot +0x60 of the
/// object at `+0x30` with the buffer and 0.
// Translation incomplete: 00b60060 - the /GS cookie check (00ec408c) of its stack buffer is not translated.
pub fn fn_00b60060(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x104, |e, buffer| {
        let source = e.mem.u32(base + 0x28);
        e.call(NAME_INTO_00AF4250, &args![source, buffer, 0x104u32]);
        e.vcall(base + 0x30, 0x60, &args![buffer, 0u32]);
    });
}

// Translated from 00b600e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a constructor body): clears the
/// word `+0` and the floats `+8`, `+0xc`, `+0x10`, `+0x14`, sets the float `+0x18` to 1.0 and
/// answers `this`.
pub fn fn_00b600e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_f32(base + 8, 0.0);
    e.mem.set_u32(base, 0);
    e.mem.set_f32(base + 0xc, 0.0);
    e.mem.set_f32(base + 0x10, 0.0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x18, 1.0);
    this
}

/// The four floats `fn_00b60120` sets (`011f9ee0`..`011f9eec`).
const RANGE_011F9EE0: u32 = 0x011f_9ee0;

// Translated from 00b60120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four floats `a`, `b`, `c`, `d`): stores `a`, `b - a`,
/// `c` and `d - c` at `011f9ee0`, `011f9ee4`, `011f9ee8` and `011f9eec` (x87: each through
/// `FLD`/`FST`, the differences through `FSUBR`).
pub fn fn_00b60120(e: &mut Engine, a: f32, b: f32, c: f32, d: f32) {
    let first = x87_load32(a.to_bits());
    e.mem.set_u32(RANGE_011F9EE0, x87_store32(first));
    let span = x87_sub(x87_m32(b.to_bits()), first);
    e.mem.set_u32(RANGE_011F9EE0 + 4, x87_store32(span));
    let second = x87_load32(c.to_bits());
    e.mem.set_u32(RANGE_011F9EE0 + 8, x87_store32(second));
    let span = x87_sub(x87_m32(d.to_bits()), second);
    e.mem.set_u32(RANGE_011F9EE0 + 0xc, x87_store32(span));
}

/// `00b6f460`, where `fn_00b60150` jumps (itself a jump to `00ba8a70`, which jumps to `00a950c0`;
/// the chain ends in a `thiscall` body with one stack word).
const LINK_OBJECT_00B6F460: u32 = 0x00b6_f460;

// Translated from 00b60150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a vtable thunk (`010adef8`) that jumps to `00b6f460` with its
/// `this` and its one stack word (the stream).
pub fn fn_00b60150(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(LINK_OBJECT_00B6F460, &args![this, stream]);
}

/// `BSShaderProperty::PostLinkObject` (engine map, `00ba8be0`, `thiscall`, one stack word).
const SHADER_PROPERTY_POST_LINK_OBJECT: u32 = 0x00ba_8be0;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words).
const GET_TEXTURE: u32 = 0x00b5_5840;

// Translated from 00b60240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, the stream): runs
/// `BSShaderProperty::PostLinkObject` with it, then, when the word `+0x88` is not null, calls
/// `BSShaderManager::GetTexture` with (that word, 1, `this + 0x84`, 1, 0).
pub fn fn_00b60240(e: &mut Engine, this: Ptr, stream: u32) {
    let base = this.addr();
    e.call(SHADER_PROPERTY_POST_LINK_OBJECT, &args![base, stream]);
    let name = e.mem.u32(base + 0x88);
    if name != 0 {
        e.call(GET_TEXTURE, &args![name, 1u32, base + 0x84, 1u32, 0u32]);
    }
}

// Translated from 00b60280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiPointer<T>::operator=(T*)` (`thiscall` on the pointer, one
/// stack word) for objects whose reference count sits at `+8`: when the pointer does not
/// already hold the object, the old one (when not null) is released (`InterlockedDecrement`
/// on its `+8`, deleted through vtable slot 0 with 1 at zero), the object stored and its count
/// at `+8` incremented when it is not null. Answers the pointer.
pub fn fn_00b60280(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    let slot = this.addr();
    let old = e.mem.u32(slot);
    if old == object {
        return this;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 8]).u32();
        if left == 0 {
            e.vcall(old, 0, &args![1u32]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 8]);
    }
    this
}

// Translated from 00b60330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): answers 1 when the byte `011f9427` is 0,
/// the word `011f95e4` is not 0, bit 0x20 of the byte `011f91d8` is set and the word
/// `011f91c0` is at least 2 (signed), else 0.
pub fn fn_00b60330(e: &mut Engine) -> u32 {
    let enabled = e.global::<u8>(0x011f_9427) == 0
        && e.global::<u32>(0x011f_95e4) != 0
        && e.global::<u8>(0x011f_91d8) & 0x20 != 0
        && e.global::<i32>(0x011f_91c0) >= 2;
    u32::from(enabled)
}

/// `00b601a0` (unnamed, `thiscall`, no stack words): what `fn_00b60360` runs under both
/// vtables (it empties the map's buckets).
const MAP_CLEAR_00B601A0: u32 = 0x00b6_01a0;
/// The two vtables `fn_00b60360` stores (`010ade50`, then `010ade18`).
const VTABLE_010ADE50: u32 = 0x010a_de50;
const VTABLE_010ADE18: u32 = 0x010a_de18;
/// `00aa10f0` (`cdecl`, one word): the Ni free.
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00b60360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a destructor): stores the vtable
/// `010ade50`, runs `00b601a0`, stores the vtable `010ade18`, runs `00b601a0` again and frees
/// the word `+8` (`00aa10f0`).
// Translation incomplete: 00b60360 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b60360(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADE50);
    e.call(MAP_CLEAR_00B601A0, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADE18);
    e.call(MAP_CLEAR_00B601A0, &args![base]);
    let buckets = e.mem.u32(base + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00b603d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): allocates 0xc bytes from the memory
/// manager (`00aa3e40` on `011f6238`) and answers the block.
pub fn fn_00b603d0(e: &mut Engine) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32()
}

/// The vtable `fn_00b60420` stores (`010adf84`) and the base destructor it ends with
/// (`00b580a0`, unnamed, `thiscall`, no stack words).
const VTABLE_010ADF84: u32 = 0x010a_df84;
const MAP_BASE_DESTRUCT_00B580A0: u32 = 0x00b5_80a0;

// Translated from 00b60420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the destructor of a string-keyed
/// map): stores the vtable `010adf84`; when the byte `+0x10` (the map owns its keys) is set,
/// frees the key (`+4`) of every item of every bucket (the bucket count `+4`, the buckets
/// `+8`, each item's next at `+0`) through `00aa10f0`. Then runs the base destructor `00b580a0`.
// Translation incomplete: 00b60420 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b60420(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADF84);
    if e.mem.u8(base + 0x10) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(base + 4) {
            let buckets = e.mem.u32(base + 8);
            let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let key = e.mem.u32(item + 4);
                item = e.mem.u32(item);
                e.call(NI_FREE, &args![key]);
            }
            bucket += 1;
        }
    }
    e.call(MAP_BASE_DESTRUCT_00B580A0, &args![base]);
}

/// `00aa1070` (`cdecl`, one word): the Ni allocate; `_strcpy_s` (`00ec65a6`, `cdecl`).
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const STRCPY_S: u32 = 0x00ec_65a6;

// Translated from 00b604b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a string-keyed map, three stack words: an item,
/// a key string and an `NiPointer` passed by value): when the map's byte `+0x10` is set the
/// item's key (`+4`) becomes a copy of the string (`strlen + 1` bytes from `00aa1070`, filled by
/// `_strcpy_s`), otherwise the string itself; the value is assigned to the item's `NiPointer` at
/// `+8`, and the by-value argument is released (its destructor) at the end.
// Translation incomplete: 00b604b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the by-value NiPointer) is not translated.
pub fn fn_00b604b0(e: &mut Engine, this: Ptr, item: Ptr, key: u32, value: u32) {
    let item = item.addr();
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let size = e.mem.cstr(key).len() as u32 + 1;
        let copy = e.call(NI_ALLOCATE_00AA1070, &args![size]).u32();
        e.mem.set_u32(item + 4, copy);
        e.call(STRCPY_S, &args![copy, size, key]);
    } else {
        e.mem.set_u32(item + 4, key);
    }
    ni_pointer_assign(e, item + 8, value);
    if value != 0 {
        release_reference(e, value);
    }
}

/// The callees of `fn_00b60640` and `fn_00b606f0` (engine map unit `queuedfiles` for the
/// `00c3c...` ones; all `thiscall`): the base constructor `00c3ce60` (one word), the stream
/// constructor `00a66150` (unit `nistream.cpp`, none), `00c3cee0` (one word), `00c3cf60` (one
/// word), `BSStream::~BSStream` `0043d090` (none) and the base destructor `00c3cea0` (none).
const QUEUED_BASE_CONSTRUCT_00C3CE60: u32 = 0x00c3_ce60;
const STREAM_CONSTRUCT_00A66150: u32 = 0x00a6_6150;
const QUEUED_SET_00C3CEE0: u32 = 0x00c3_cee0;
const QUEUED_SET_00C3CF60: u32 = 0x00c3_cf60;
const BS_STREAM_DESTRUCT: u32 = 0x0043_d090;
const QUEUED_BASE_DESTRUCT_00C3CEA0: u32 = 0x00c3_cea0;
/// The vtables `fn_00b60640` stores: `010ade6c` on the object, `01016904` on its stream at
/// `+0x30`.
const VTABLE_010ADE6C: u32 = 0x010a_de6c;
const STREAM_VTABLE_01016904: u32 = 0x0101_6904;

// Translated from 00b60640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word; a queued-file constructor): runs
/// `00c3ce60` with 2, stores the vtable `010ade6c`, constructs the stream at `+0x30`
/// (`00a66150`), stores `01016904` on it and clears its words `+0x5c8` and `+0x5c4`, then calls
/// `00c3cee0` with the word and `00c3cf60` with 0. Answers `this`.
// Translation incomplete: 00b60640 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the stream) is not translated.
pub fn fn_00b60640(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    let base = this.addr();
    e.call(QUEUED_BASE_CONSTRUCT_00C3CE60, &args![base, 2u32]);
    let stream = base + 0x30;
    e.mem.set_u32(base, VTABLE_010ADE6C);
    e.call(STREAM_CONSTRUCT_00A66150, &args![stream]);
    e.mem.set_u32(stream, STREAM_VTABLE_01016904);
    e.mem.set_u32(stream + 0x5c8, 0);
    e.mem.set_u32(stream + 0x5c4, 0);
    e.call(QUEUED_SET_00C3CEE0, &args![base, value]);
    e.call(QUEUED_SET_00C3CF60, &args![base, 0u32]);
    this
}

// Translated from 00b606e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function pointer `00b57420` stores at `011f9fac`, and vtable
/// slots): reads no register or stack word and answers `true` (`MOV AL,1`).
pub fn fn_00b606e0(_e: &mut Engine) -> bool {
    true
}

// Translated from 00b606f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the destructor matching
/// `fn_00b60640`): runs `BSStream::~BSStream` on `+0x30`, then the base destructor `00c3cea0`.
// Translation incomplete: 00b606f0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stream) is not translated.
pub fn fn_00b606f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(BS_STREAM_DESTRUCT, &args![base + 0x30]);
    e.call(QUEUED_BASE_DESTRUCT_00C3CEA0, &args![base]);
}

// Translated from 00b60750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the address of an `NiPointer` it
/// fills and answers): the object is the first word of the array `+0x250` when the count
/// `+0x258` is not 0, else null; it is stored and, when not null, its count at `+4`
/// incremented.
pub fn fn_00b60750(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let base = this.addr();
    let object = if e.mem.u32(base + 0x258) != 0 {
        let array = e.mem.u32(base + 0x250);
        e.mem.u32(array)
    } else {
        0
    };
    e.mem.set_u32(out.addr(), object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
    out
}

/// `QueuedFileEntry::GetDescription` (Xbox PDB, `00c3d0e0`; `thiscall`, three words: the
/// buffer, its size and the kind text) and the kind text `"grass load task"` (`010ade08`).
const QUEUED_FILE_ENTRY_GET_DESCRIPTION: u32 = 0x00c3_d0e0;
const GRASS_LOAD_TASK_KIND_010ADE08: u32 = 0x010a_de08;
/// Where `fn_00b60160` jumps (`00bc3770`, itself a jump to `00ba8a80`).
const REGISTER_STREAMABLES_00BC3770: u32 = 0x00bc_3770;
/// The base `SaveBinary` (`00bb7160`, `thiscall`, one word: the stream) and the stream's
/// string writer `00a64160` (`thiscall` on the stream, one word: the C string).
const BASE_SAVE_BINARY_00BB7160: u32 = 0x00bb_7160;
const STREAM_SAVE_STRING_00A64160: u32 = 0x00a6_4160;
/// The float `fn_00b60170` truncates (`011fd890`).
const SETTING_011FD890: u32 = 0x011f_d890;
/// The vtable `fn_00b60400` and `fn_00b605e0` store (`010ade34`).
const VTABLE_010ADE34: u32 = 0x010a_de34;
/// The memory manager's deallocation (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

// Translated from 00b600c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: slot +0x10 of the queued-file vtable `010ade6c` (`thiscall`,
/// two words: a buffer and its size). Calls `QueuedFileEntry::GetDescription` on `this` with
/// them and the kind text `"grass load task"` (`010ade08`), and answers its `EAX`.
pub fn fn_00b600c0(e: &mut Engine, this: Ptr, buffer: u32, size: u32) -> u32 {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, GRASS_LOAD_TASK_KIND_010ADE08],
    )
    .u32()
}

// Translated from 00b60100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `TallGrassShaderProperty`'s vtable slot +0xd0 (the vtable
/// `010adeac`, whose `GetRTTI` `00b60d60` answers the record `011f9f00` that the static
/// initializer `00fb9910` names `"TallGrassShaderProperty"`; `SetClampD3D` in the Xbox PDB's
/// slot list; `thiscall`, one word). Stores 0 in `TextureClampMode` (`+0x90` on PC, Xbox PDB
/// +0x98) when the word is 3, otherwise 3.
pub fn fn_00b60100(e: &mut Engine, this: Ptr, mode: u32) {
    let value: u32 = if mode.wrapping_sub(3) != 0 { 3 } else { 0 };
    e.mem.set_u32(this.addr().wrapping_add(0x90), value);
}

// Translated from 00b60160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `TallGrassShaderProperty`'s vtable slot +0x54
/// (`RegisterStreamables` in the Xbox PDB's slot list): a jump to `00bc3770` with `this` and
/// its one stack word (the stream); answers the target's `EAX`.
pub fn fn_00b60160(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(REGISTER_STREAMABLES_00BC3770, &args![this, stream])
        .u32()
}

// Translated from 00b60170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): the float at `011fd890` truncated
/// (`FISTP qword` with the rounding control set to chop), its low word in `EAX`.
pub fn fn_00b60170(e: &mut Engine) -> u32 {
    let value = e.mem.u32(SETTING_011FD890);
    x87_fistp64_chop(X87Ext::load32(value)) as u32
}

// Translated from 00b60210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `TallGrassShaderProperty`'s vtable slot +0x58 (`SaveBinary`
/// in the Xbox PDB's slot list; `thiscall`, one word: the stream). The base `00bb7160` runs
/// with the stream; then the stream writes the C string at `+0x88` (the PC offset of the Xbox
/// PDB's `TextureFilename`, a `BSStringT` whose first word is the characters) through
/// `00a64160`.
pub fn fn_00b60210(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BASE_SAVE_BINARY_00BB7160, &args![this, stream]);
    let name = e.mem.u32(this.addr().wrapping_add(0x88));
    e.call(STREAM_SAVE_STRING_00A64160, &args![stream, name]);
}

/// The map destructor body shared by `fn_00b603e0`, `fn_00b60400`, `fn_00b605a0` and
/// `fn_00b605e0`: the vtable stored, the clean-up `00b601a0`, the buckets (`+8`) freed.
fn map_destroy(e: &mut Engine, this: Ptr, vtable: u32) {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    e.call(MAP_CLEAR_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(NI_FREE, &args![buckets]);
}

/// The scalar deleting tail: with bit 0 of the flags the object goes back to the memory
/// manager (`00aa4060` on `011f6238`).
fn give_back(e: &mut Engine, this: Ptr, flags: u8) {
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
}

// Translated from 00b603e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a map destructor): stores the vtable
/// `010ade18`, runs `00b601a0` and frees the buckets (`+8`, `00aa10f0`).
pub fn fn_00b603e0(e: &mut Engine, this: Ptr) {
    map_destroy(e, this, VTABLE_010ADE18);
}

// Translated from 00b60400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a map destructor): stores the vtable
/// `010ade34`, runs `00b601a0` and frees the buckets (`+8`, `00aa10f0`).
pub fn fn_00b60400(e: &mut Engine, this: Ptr) {
    map_destroy(e, this, VTABLE_010ADE34);
}

// Translated from 00b605a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of `fn_00b603e0`'s map
/// (`thiscall`, one word: the flags; inlined body), the map given back to the memory manager
/// with bit 0 of the flags. Answers `this`.
pub fn fn_00b605a0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    map_destroy(e, this, VTABLE_010ADE18);
    give_back(e, this, flags);
    this
}

// Translated from 00b605e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of `fn_00b60400`'s map
/// (`thiscall`, one word: the flags; inlined body), the map given back to the memory manager
/// with bit 0 of the flags. Answers `this`.
pub fn fn_00b605e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    map_destroy(e, this, VTABLE_010ADE34);
    give_back(e, this, flags);
    this
}

// Translated from 00b60620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of the string-keyed map
/// (`thiscall`, one word: the flags): `fn_00b60420`, then the map given back to the memory
/// manager with bit 0 of the flags. Answers `this`.
pub fn fn_00b60620(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00b60420(e, this);
    give_back(e, this, flags);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_00c0, fn_00b600c0(Ptr, u32, u32) -> u32),
        entry!(0x00b6_0100, fn_00b60100(Ptr, u32)),
        entry!(0x00b6_0160, fn_00b60160(Ptr, u32) -> u32),
        entry!(0x00b6_0170, fn_00b60170() -> u32),
        entry!(0x00b6_0210, fn_00b60210(Ptr, u32)),
        entry!(0x00b6_03e0, fn_00b603e0(Ptr)),
        entry!(0x00b6_0400, fn_00b60400(Ptr)),
        entry!(0x00b6_05a0, fn_00b605a0(Ptr, u8) -> Ptr),
        entry!(0x00b6_05e0, fn_00b605e0(Ptr, u8) -> Ptr),
        entry!(0x00b6_0620, fn_00b60620(Ptr, u8) -> Ptr),
        entry!(0x00b6_02d0, fn_00b602d0(Ptr<NiTPointerList>) -> u32),
        entry!(0x00b5_fd60, fn_00b5fd60(Ptr)),
        entry!(0x00b6_0040, fn_00b60040()),
        entry!(0x00b6_0060, fn_00b60060(Ptr)),
        entry!(0x00b6_00e0, fn_00b600e0(Ptr) -> Ptr),
        entry!(0x00b6_0120, fn_00b60120(f32, f32, f32, f32)),
        entry!(0x00b6_0150, fn_00b60150(Ptr, u32)),
        entry!(0x00b6_0240, fn_00b60240(Ptr, u32)),
        entry!(0x00b6_0280, fn_00b60280(Ptr, u32) -> Ptr),
        entry!(0x00b6_0330, fn_00b60330() -> u32),
        entry!(0x00b6_0360, fn_00b60360(Ptr)),
        entry!(0x00b6_03d0, fn_00b603d0() -> u32),
        entry!(0x00b6_0420, fn_00b60420(Ptr)),
        entry!(0x00b6_04b0, fn_00b604b0(Ptr, Ptr, u32, u32)),
        entry!(0x00b6_0640, fn_00b60640(Ptr, u32) -> Ptr),
        entry!(0x00b6_06e0, fn_00b606e0() -> bool),
        entry!(0x00b6_06f0, fn_00b606f0(Ptr)),
        entry!(0x00b6_0750, fn_00b60750(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A list of two items (elements 10, 20), the pool's lock held by an owner 7.
    fn engine() -> (Engine, u32, [u32; 2]) {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0e10_0000);
        e.mem.set_u32(NODE_POOL_LOCK, 7);
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 4, 0);
        e.mem.set_u32(items[0] + 8, 10);
        e.mem.set_u32(items[1], 0);
        e.mem.set_u32(items[1] + 4, items[0]);
        e.mem.set_u32(items[1] + 8, 20);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[1]);
        e.mem.set_u32(list + 8, 2);
        (e, list, items)
    }

    #[test]
    fn fn_00b602d0_removes_the_head_and_frees_its_item() {
        let (mut e, list, items) = engine();
        assert_eq!(e.call(0x00b6_02d0, &args![list]).u32(), 10);
        assert_eq!(e.mem.u32(list), items[1]);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert_eq!(e.mem.u32(list + 4), items[1]);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(items[0]), 0x0e10_0000);
        assert_eq!(e.mem.u32(items[0] + 4), 0);
        assert_eq!(e.mem.u32(items[0] + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), items[0]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        // The last item: the tail is cleared too.
        assert_eq!(e.call(0x00b6_02d0, &args![list]).u32(), 20);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(items[1]), items[0]);
    }

    #[test]
    fn fn_00b602d0_keeps_a_lock_still_held() {
        let (mut e, list, _) = engine();
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.call(0x00b6_02d0, &args![list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 7);
    }
}

#[cfg(test)]
mod b053_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const STUB: AbiFn = |_, _| Ret::default();

    /// A list of items (next, previous, element) holding `elements`; answers the first item.
    fn list(e: &mut Engine, elements: &[u32]) -> u32 {
        let mut next = 0;
        for element in elements.iter().rev() {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 8, *element);
            next = item;
        }
        next
    }

    fn node_engine() -> Engine {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f32(DISTANCE_LIMIT_010231D0, 150.0);
        for callee in [
            LOCK_ENTER,
            LIGHT_UPDATE_00B5CD00,
            LIGHT_UPDATE_00B5ECA0,
            LIGHT_UPDATE_00B5D930,
            LIGHT_UPDATE_00B5D300,
            REMOVE_QUEUED_LIGHT,
            UPDATE_QUEUED_OBJECT_LIGHTING_START,
            RELEASE_00B6D4C0,
            PORTAL_GRAPH_00C5B8D0,
            LIST_REMOVE_ALL,
        ] {
            e.register(callee, STUB);
        }
        e
    }

    #[test]
    fn fn_00b5fd60_processes_every_queue() {
        let mut e = node_engine();
        let node = e.mem.alloc(0x200);
        let lights: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x110)).collect();
        e.mem.set_u8(lights[0] + 0xec, 1);
        e.mem.set_u8(lights[2] + 0xfc, 1);
        let first = list(&mut e, &[lights[0], 0, lights[1]]);
        e.mem.set_u32(node + 0xe4, first);
        let first = list(&mut e, &[lights[2], lights[3]]);
        e.mem.set_u32(node + 0xfc, first);
        // Removed lights: the first shadowed (its +0x10c held object is released).
        let removed = [e.mem.alloc(0x110), e.mem.alloc(0x110)];
        for light in removed {
            e.mem.set_u32(light, REFERENCE_VTABLE);
            e.mem.set_u32(light + 4, 1);
        }
        e.mem.set_u8(removed[0] + 0xec, 1);
        let held = object(&mut e, 1);
        e.mem.set_u32(removed[0] + 0x10c, held);
        let first = list(&mut e, &[removed[0], 0, removed[1]]);
        e.mem.set_u32(node + 0xf0, first);
        e.mem.set_u32(HOLDER_011F91A8, 0x0e55_0000);
        // Objects: slot +0xc answers 1 for the first two; bounds 100.0 (near) and the default.
        e.register(0x0e72_0000, |_, _| 1u32.into_ret());
        e.register(0x0e72_0004, |_, _| 0u32.into_ret());
        e.put_vtable(0x0e73_0000, &[0, 0, 0, 0x0e72_0000]);
        e.put_vtable(0x0e73_0100, &[0, 0, 0, 0x0e72_0004]);
        let objects: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x24)).collect();
        e.mem.set_u32(objects[0], 0x0e73_0000);
        e.mem.set_u32(objects[1], 0x0e73_0000);
        e.mem.set_u32(objects[2], 0x0e73_0100);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 100.0);
        e.mem.set_u32(objects[0] + 0x20, bound);
        e.mem.set_f32(DEFAULT_BOUND_011F4288 + 0xc, 150.0);
        let first = list(&mut e, &[objects[0], objects[1]]);
        e.mem.set_u32(node + 0x114, first);
        let first = list(&mut e, &[objects[2]]);
        e.mem.set_u32(node + 0x108, first);
        e.mem.set_u32(node + 0x1e0, 0x0e56_0000);
        e.mem.set_u32(LIGHT_LOCK_011F9EA0, 3);
        e.mem.set_u32(LIGHT_LOCK_COUNT_011F9EA4, 1);
        e.mem.set_u32(OBJECT_LOCK_011F9EC0, 4);
        e.mem.set_u32(OBJECT_LOCK_COUNT_011F9EC4, 2);
        e.call(0x00b5_fd60, &args![node]);
        assert_eq!(
            calls(&e, LOCK_ENTER),
            vec![vec![LIGHT_LOCK_011F9EA0, 0], vec![OBJECT_LOCK_011F9EC0, 0]]
        );
        assert_eq!(
            calls(&e, LIGHT_UPDATE_00B5CD00),
            vec![vec![node, lights[0]]]
        );
        assert_eq!(
            calls(&e, LIGHT_UPDATE_00B5ECA0),
            vec![vec![node, lights[1]]]
        );
        assert_eq!(
            calls(&e, LIGHT_UPDATE_00B5D930),
            vec![vec![node, lights[2]]]
        );
        assert_eq!(
            calls(&e, LIGHT_UPDATE_00B5D300),
            vec![vec![node, lights[3]]]
        );
        assert_eq!(calls(&e, RELEASE_00B6D4C0), vec![vec![0x0e55_0000, held]]);
        assert_eq!(e.mem.u32(removed[0] + 0x10c), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(calls(&e, REMOVE_QUEUED_LIGHT).len(), 2);
        // Each removed light was held for the call and released after it.
        assert_eq!(e.mem.u32(removed[0] + 4), 1);
        assert_eq!(e.mem.u32(removed[1] + 4), 1);
        assert_eq!(
            calls(&e, UPDATE_QUEUED_OBJECT_LIGHTING_START),
            vec![
                vec![objects[0], node, 1, 1],
                vec![objects[1], node, 1, 0],
                vec![objects[2], node, 0, 0],
            ]
        );
        assert_eq!(calls(&e, PORTAL_GRAPH_00C5B8D0), vec![vec![0x0e56_0000]]);
        let lists: Vec<u32> = calls(&e, LIST_REMOVE_ALL).iter().map(|w| w[0]).collect();
        assert_eq!(
            lists,
            vec![
                node + 0xe4,
                node + 0xf0,
                node + 0xfc,
                node + 0x108,
                node + 0x114
            ]
        );
        assert_eq!(e.mem.u8(node + 0x1fc), 1);
        assert_eq!(e.mem.u32(LIGHT_LOCK_COUNT_011F9EA4), 0);
        assert_eq!(e.mem.u32(LIGHT_LOCK_011F9EA0), 0);
        assert_eq!(e.mem.u32(OBJECT_LOCK_COUNT_011F9EC4), 1);
        assert_eq!(e.mem.u32(OBJECT_LOCK_011F9EC0), 4);
    }

    #[test]
    fn fn_00b5fd60_near_flag_is_0_for_an_unordered_distance() {
        let mut e = node_engine();
        e.register(0x0e72_0000, |_, _| 1u32.into_ret());
        e.put_vtable(0x0e73_0000, &[0, 0, 0, 0x0e72_0000]);
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, 0x0e73_0000);
        e.mem.set_f32(DEFAULT_BOUND_011F4288 + 0xc, f32::NAN);
        assert_eq!(near_flag(&mut e, object), 0);
        e.mem.set_f32(DEFAULT_BOUND_011F4288 + 0xc, 149.9);
        assert_eq!(near_flag(&mut e, object), 1);
    }

    #[test]
    fn fn_00b60040_runs_each_non_null_node() {
        let mut e = node_engine();
        let nodes = [e.mem.alloc(0x200), e.mem.alloc(0x200)];
        e.mem.set_u32(NODES_011F91C8 + 4, nodes[0]);
        e.mem.set_u32(NODES_011F91C8 + 0xc, nodes[1]);
        e.mem.set_u32(LIGHT_LOCK_COUNT_011F9EA4, 5);
        e.mem.set_u32(OBJECT_LOCK_COUNT_011F9EC4, 5);
        e.call(0x00b6_0040, &args![]);
        let lists: Vec<u32> = calls(&e, LIST_REMOVE_ALL).iter().map(|w| w[0]).collect();
        assert_eq!(lists.len(), 10);
        assert_eq!(lists[0], nodes[0] + 0xe4);
        assert_eq!(lists[5], nodes[1] + 0xe4);
        assert_eq!(e.mem.u32(LIGHT_LOCK_COUNT_011F9EA4), 3);
    }

    #[test]
    fn fn_00b60060_fills_the_buffer_and_passes_it_on() {
        let mut e = rig();
        e.register(NAME_INTO_00AF4250, |e, a| {
            e.mem.set_cstr(a[1], b"name");
            Ret::default()
        });
        e.register(0x0e74_0000, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"name".to_vec());
            Ret::default()
        });
        e.put_vtable(0x0e75_0000, &[0; 0x19]);
        e.mem.set_u32(0x0e75_0060, 0x0e74_0000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, 0x77);
        e.mem.set_u32(this + 0x30, 0x0e75_0000);
        e.call(0x00b6_0060, &args![this]);
        let fill = calls(&e, NAME_INTO_00AF4250);
        assert_eq!(fill.len(), 1);
        assert_eq!((fill[0][0], fill[0][2]), (0x77, 0x104));
        assert_eq!(
            calls(&e, 0x0e74_0000),
            vec![vec![this + 0x30, fill[0][1], 0]]
        );
    }

    #[test]
    fn fn_00b600e0_clears_and_sets_the_scale() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0xcc; 0x1c]);
        assert_eq!(e.call(0x00b6_00e0, &args![this]).u32(), this);
        for offset in [0u32, 8, 0xc, 0x10, 0x14] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert_eq!(e.mem.u32(this + 4), 0xcccc_cccc);
    }

    #[test]
    fn fn_00b60120_stores_the_starts_and_spans() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.call(0x00b6_0120, &args![1.5f32, 4.0f32, -2.0f32, 3.0f32]);
        assert_eq!(e.mem.f32(RANGE_011F9EE0), 1.5);
        assert_eq!(e.mem.f32(RANGE_011F9EE0 + 4), 2.5);
        assert_eq!(e.mem.f32(RANGE_011F9EE0 + 8), -2.0);
        assert_eq!(e.mem.f32(RANGE_011F9EE0 + 0xc), 5.0);
    }

    #[test]
    fn fn_00b60150_forwards_to_the_link() {
        let mut e = rig();
        e.register(LINK_OBJECT_00B6F460, STUB);
        e.call(0x00b6_0150, &args![0x10u32, 0x20u32]);
        assert_eq!(calls(&e, LINK_OBJECT_00B6F460), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn fn_00b60240_loads_the_texture_when_named() {
        let mut e = rig();
        e.register(SHADER_PROPERTY_POST_LINK_OBJECT, STUB);
        e.register(GET_TEXTURE, STUB);
        let this = e.mem.alloc(0x90);
        e.call(0x00b6_0240, &args![this, 0x55u32]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_POST_LINK_OBJECT),
            vec![vec![this, 0x55]]
        );
        assert!(calls(&e, GET_TEXTURE).is_empty());
        e.mem.set_u32(this + 0x88, 0x99);
        e.call(0x00b6_0240, &args![this, 0x55u32]);
        assert_eq!(
            calls(&e, GET_TEXTURE),
            vec![vec![0x99, 1, this + 0x84, 1, 0]]
        );
    }

    #[test]
    fn fn_00b60280_assigns_with_the_count_at_8() {
        let mut e = rig();
        e.register(0x0e76_0000, STUB);
        e.put_vtable(0x0e77_0000, &[0x0e76_0000]);
        let old = e.mem.alloc(0xc);
        e.mem.set_u32(old, 0x0e77_0000);
        e.mem.set_u32(old + 8, 1);
        let new = e.mem.alloc(0xc);
        e.mem.set_u32(new, 0x0e77_0000);
        e.mem.set_u32(new + 8, 4);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        assert_eq!(e.call(0x00b6_0280, &args![slot, new]).u32(), slot);
        assert_eq!(e.mem.u32(slot), new);
        assert_eq!(e.mem.u32(new + 8), 5);
        assert_eq!(calls(&e, 0x0e76_0000), vec![vec![old, 1]]);
        // The same object again: nothing changes.
        e.call(0x00b6_0280, &args![slot, new]);
        assert_eq!(e.mem.u32(new + 8), 5);
        e.call(0x00b6_0280, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.mem.u32(new + 8), 4);
    }

    #[test]
    fn fn_00b60330_needs_every_condition() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.mem.set_u32(0x011f_95e4, 1);
        e.mem.set_u8(0x011f_91d8, 0x20);
        e.mem.set_u32(0x011f_91c0, 2);
        assert_eq!(e.call(0x00b6_0330, &args![]).u32(), 1);
        e.mem.set_i32(0x011f_91c0, -5);
        assert_eq!(e.call(0x00b6_0330, &args![]).u32(), 0);
        e.mem.set_u32(0x011f_91c0, 7);
        e.mem.set_u8(0x011f_9427, 1);
        assert_eq!(e.call(0x00b6_0330, &args![]).u32(), 0);
        e.mem.set_u8(0x011f_9427, 0);
        e.mem.set_u8(0x011f_91d8, 0xdf);
        assert_eq!(e.call(0x00b6_0330, &args![]).u32(), 0);
        e.mem.set_u8(0x011f_91d8, 0x20);
        e.mem.set_u32(0x011f_95e4, 0);
        assert_eq!(e.call(0x00b6_0330, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00b60360_clears_under_both_vtables_then_frees() {
        let mut e = rig();
        e.register(MAP_CLEAR_00B601A0, STUB);
        e.register(NI_FREE, STUB);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x4444);
        e.call(0x00b6_0360, &args![this]);
        assert_eq!(calls(&e, MAP_CLEAR_00B601A0), vec![vec![this], vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADE18);
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x4444]]);
    }

    #[test]
    fn fn_00b603d0_allocates_twelve_bytes() {
        let mut e = rig();
        let block = e.call(0x00b6_03d0, &args![]).u32();
        assert_ne!(block, 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0xc]]);
    }

    #[test]
    fn fn_00b60420_frees_owned_keys() {
        let mut e = rig();
        e.register(NI_FREE, STUB);
        e.register(MAP_BASE_DESTRUCT_00B580A0, STUB);
        let this = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0xc);
        let items = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 4, 0x100);
        e.mem.set_u32(items[1] + 4, 0x101);
        e.mem.set_u32(items[2] + 4, 0x102);
        e.mem.set_u32(buckets, items[0]);
        e.mem.set_u32(buckets + 8, items[2]);
        e.mem.set_u32(this + 4, 3);
        e.mem.set_u32(this + 8, buckets);
        e.mem.set_u8(this + 0x10, 1);
        e.call(0x00b6_0420, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADF84);
        assert_eq!(
            calls(&e, NI_FREE),
            vec![vec![0x100], vec![0x101], vec![0x102]]
        );
        assert_eq!(calls(&e, MAP_BASE_DESTRUCT_00B580A0), vec![vec![this]]);
        // Keys not owned: nothing freed.
        e.mem.set_u8(this + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_0420, &args![this]);
        assert!(calls(&e, NI_FREE).is_empty());
    }

    #[test]
    fn fn_00b604b0_sets_the_key_and_value() {
        let mut e = rig();
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let map = e.mem.alloc(0x14);
        let item = e.mem.alloc(0xc);
        let key = e.mem.alloc(8);
        e.mem.set_cstr(key, b"abc");
        // The caller's by-value pointer holds one reference; the item keeps another.
        let value = object(&mut e, 1);
        e.call(0x00b6_04b0, &args![map, item, key, value]);
        assert_eq!(e.mem.u32(item + 4), key);
        assert_eq!(e.mem.u32(item + 8), value);
        assert_eq!(e.mem.u32(value + 4), 1);
        e.mem.set_u8(map + 0x10, 1);
        let other = object(&mut e, 1);
        e.call(0x00b6_04b0, &args![map, item, key, other]);
        let copy = e.mem.u32(item + 4);
        assert_ne!(copy, key);
        assert_eq!(e.mem.cstr(copy), b"abc".to_vec());
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070), vec![vec![4]]);
        assert_eq!(calls(&e, STRCPY_S), vec![vec![copy, 4, key]]);
        assert_eq!(e.mem.u32(item + 8), other);
        assert_eq!(e.mem.u32(value + 4), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![value]]);
    }

    #[test]
    fn fn_00b60640_constructs_the_queued_file() {
        let mut e = rig();
        for callee in [
            QUEUED_BASE_CONSTRUCT_00C3CE60,
            STREAM_CONSTRUCT_00A66150,
            QUEUED_SET_00C3CEE0,
            QUEUED_SET_00C3CF60,
        ] {
            e.register(callee, STUB);
        }
        let this = e.mem.alloc(0x600);
        e.mem.write(this, &[0xcc; 0x600]);
        assert_eq!(e.call(0x00b6_0640, &args![this, 0x31u32]).u32(), this);
        assert_eq!(
            calls(&e, QUEUED_BASE_CONSTRUCT_00C3CE60),
            vec![vec![this, 2]]
        );
        assert_eq!(
            calls(&e, STREAM_CONSTRUCT_00A66150),
            vec![vec![this + 0x30]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_010ADE6C);
        assert_eq!(e.mem.u32(this + 0x30), STREAM_VTABLE_01016904);
        assert_eq!(e.mem.u32(this + 0x30 + 0x5c8), 0);
        assert_eq!(e.mem.u32(this + 0x30 + 0x5c4), 0);
        assert_eq!(calls(&e, QUEUED_SET_00C3CEE0), vec![vec![this, 0x31]]);
        assert_eq!(calls(&e, QUEUED_SET_00C3CF60), vec![vec![this, 0]]);
    }

    #[test]
    fn fn_00b606e0_answers_true() {
        let mut e = Engine::new();
        assert!(e.call(0x00b6_06e0, &args![]).bool());
    }

    #[test]
    fn fn_00b606f0_destroys_the_stream_then_the_base() {
        let mut e = rig();
        e.register(BS_STREAM_DESTRUCT, STUB);
        e.register(QUEUED_BASE_DESTRUCT_00C3CEA0, STUB);
        e.call(0x00b6_06f0, &args![0x1000u32]);
        let log: Vec<_> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (BS_STREAM_DESTRUCT, vec![0x1030]),
                (QUEUED_BASE_DESTRUCT_00C3CEA0, vec![0x1000]),
            ]
        );
    }

    #[test]
    fn fn_00b60750_hands_out_the_first_object() {
        let mut e = rig();
        let this = e.mem.alloc(0x260);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        assert_eq!(e.call(0x00b6_0750, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0);
        let held = object(&mut e, 2);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, held);
        e.mem.set_u32(this + 0x250, array);
        e.mem.set_u32(this + 0x258, 1);
        e.call(0x00b6_0750, &args![this, out]);
        assert_eq!(e.mem.u32(out), held);
        assert_eq!(e.mem.u32(held + 4), 3);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const STUB: AbiFn = |_, _| Ret::default();

    #[test]
    fn fn_00b600c0_describes_a_grass_load_task() {
        let mut e = rig();
        e.register(QUEUED_FILE_ENTRY_GET_DESCRIPTION, |_, _| {
            0x101u32.into_ret()
        });
        let r = e
            .call(0x00b6_00c0, &args![0x500u32, 0x600u32, 0x40u32])
            .u32();
        assert_eq!(r, 0x101);
        assert_eq!(
            calls(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![0x500, 0x600, 0x40, GRASS_LOAD_TASK_KIND_010ADE08]]
        );
    }

    #[test]
    fn fn_00b60100_maps_3_to_0_and_the_rest_to_3() {
        let mut e = rig();
        let this = e.mem.alloc(0xa0);
        e.call(0x00b6_0100, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0);
        e.call(0x00b6_0100, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x90), 3);
    }

    #[test]
    fn fn_00b60160_jumps_on() {
        let mut e = rig();
        e.register(REGISTER_STREAMABLES_00BC3770, |_, _| 0x0100u32.into_ret());
        assert_eq!(e.call(0x00b6_0160, &args![0x10u32, 0x20u32]).u32(), 0x100);
        assert_eq!(
            calls(&e, REGISTER_STREAMABLES_00BC3770),
            vec![vec![0x10, 0x20]]
        );
    }

    #[test]
    fn fn_00b60170_truncates_the_setting() {
        let mut e = rig();
        e.map(0x011f_d000, 0x1000);
        e.set_global(SETTING_011FD890, 2.75f32);
        assert_eq!(e.call(0x00b6_0170, &args![]).u32(), 2);
        e.set_global(SETTING_011FD890, -2.75f32);
        assert_eq!(e.call(0x00b6_0170, &args![]).i32(), -2);
    }

    #[test]
    fn fn_00b60210_saves_the_base_then_the_name() {
        let mut e = rig();
        e.register(BASE_SAVE_BINARY_00BB7160, STUB);
        e.register(STREAM_SAVE_STRING_00A64160, STUB);
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x88, 0x7777);
        e.call(0x00b6_0210, &args![this, 0x300u32]);
        assert_eq!(
            calls(&e, BASE_SAVE_BINARY_00BB7160),
            vec![vec![this, 0x300]]
        );
        assert_eq!(
            calls(&e, STREAM_SAVE_STRING_00A64160),
            vec![vec![0x300, 0x7777]]
        );
    }

    #[test]
    fn map_destructors_store_their_vtable_clean_and_free() {
        for (addr, vtable) in [
            (0x00b6_03e0u32, VTABLE_010ADE18),
            (0x00b6_0400, VTABLE_010ADE34),
        ] {
            let mut e = rig();
            e.register(MAP_CLEAR_00B601A0, STUB);
            e.register(NI_FREE, STUB);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x4444);
            e.call(addr, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&e, MAP_CLEAR_00B601A0), vec![vec![this]]);
            assert_eq!(calls(&e, NI_FREE), vec![vec![0x4444]]);
        }
    }

    #[test]
    fn map_scalar_deleting_destructors_give_back_on_bit_0() {
        for (addr, vtable) in [
            (0x00b6_05a0u32, VTABLE_010ADE18),
            (0x00b6_05e0, VTABLE_010ADE34),
        ] {
            let mut e = rig();
            e.register(MAP_CLEAR_00B601A0, STUB);
            e.register(NI_FREE, STUB);
            e.register(MEMORY_DEALLOCATE_00AA4060, STUB);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x4444);
            assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
            assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
            assert_eq!(e.mem.u32(this), vtable);
            e.call(addr, &args![this, 1u32]);
            assert_eq!(
                calls(&e, MEMORY_DEALLOCATE_00AA4060),
                vec![vec![MEMORY_MANAGER, this]]
            );
            assert_eq!(calls(&e, NI_FREE).len(), 2);
        }
    }

    #[test]
    fn fn_00b60620_destroys_the_string_map_and_gives_it_back() {
        let mut e = rig();
        e.register(MAP_BASE_DESTRUCT_00B580A0, STUB);
        e.register(MEMORY_DEALLOCATE_00AA4060, STUB);
        let this = e.mem.alloc(0x14);
        e.call(0x00b6_0620, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADF84);
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
        assert_eq!(e.call(0x00b6_0620, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(calls(&e, MAP_BASE_DESTRUCT_00B580A0).len(), 2);
    }
}
