//! `BSShader/run_00b60a50` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa13e0` (`cdecl`, one word): the object allocation; `00aa1460`
/// (`cdecl`, `(block, size)`): the sized free; `00aa1070` (`cdecl`, one
/// word): the allocation the hash maps take their buckets from.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
const NI_ALLOC: u32 = 0x00aa_1070;
/// The memory manager (`011f6238`) and its `Allocate` (`00aa3e40`) and
/// `Deallocate` (`00aa4060`), `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `memset` (`00ec61c0`, `cdecl`) and `_atexit` (`00ec658f`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
const ATEXIT: u32 = 0x00ec_658f;
/// `NiRefObject`'s vtable and `NiRefObject::ms_uiObjects` (`011f4400`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The thread block's word at +0x2B4 (the allocation context), 0x1C while
/// this unit's caches change.
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;

/// The vtables of this unit's classes (no RTTI): the 0x38-byte entry
/// `010ade9c` (an `NiRefObject`), the 0x98-byte `010adea8` (built on the
/// `BSShaderLightingProperty` base `00b71720`), and the hash maps
/// `010ade18` (the base vtable the map constructors store first),
/// `010af6b0` (the first cache map), `010ade34`/`010adf84` (the second
/// cache map) and `010ade50` (the map inside a 0x40-byte cache record).
const VTABLE_010ADE9C: u32 = 0x010a_de9c;
const VTABLE_010ADEA8: u32 = 0x010a_dea8;
const MAP_BASE_VTABLE_010ADE18: u32 = 0x010a_de18;
const MAP_VTABLE_010AF6B0: u32 = 0x010a_f6b0;
const MAP_BASE_VTABLE_010ADE34: u32 = 0x010a_de34;
const MAP_VTABLE_010ADF84: u32 = 0x010a_df84;
const MAP_VTABLE_010ADE50: u32 = 0x010a_de50;

/// The first cache map (`011f9f08`: vtable, hash size +4, buckets +8, count
/// +0xC), its guard word (`011f9f18`, bit 0) and its `atexit` destructor
/// (`00fdcc80`); the second (`011f9f1c`, the byte at `011f9f2c`, guard
/// `011f9f30`, destructor `00fdcc90`).
const CACHE_MAP: u32 = 0x011f_9f08;
const CACHE_MAP_GUARD: u32 = 0x011f_9f18;
const CACHE_MAP_DESTRUCTOR: u32 = 0x00fd_cc80;
const SECOND_MAP: u32 = 0x011f_9f1c;
const SECOND_MAP_BYTE: u32 = 0x011f_9f2c;
const SECOND_MAP_GUARD: u32 = 0x011f_9f30;
const SECOND_MAP_DESTRUCTOR: u32 = 0x00fd_cc90;
/// The two counters the cache keeps (`011f9ef4`: cached elements;
/// `011f9ef8`).
const ELEMENT_COUNT: u32 = 0x011f_9ef4;
const COUNTER_011F9EF8: u32 = 0x011f_9ef8;

/// The `BSShaderLightingProperty` base constructor (`00b71720`), destructor
/// (`00b71410`) and `00b70260` (unnamed, `thiscall`, one word: the stream).
const LIGHTING_CONSTRUCTOR: u32 = 0x00b7_1720;
const LIGHTING_DESTRUCTOR: u32 = 0x00b7_1410;
const LIGHTING_LOAD_00B70260: u32 = 0x00b7_0260;
/// `NiStream::LoadCString` (`00a64ce0`, `thiscall`, the address of a
/// `char *`), the string assignment `004037f0` (`thiscall`, `(text,
/// length)`) and the free `00aa10f0` (`cdecl`).
const LOAD_CSTRING: u32 = 0x00a6_4ce0;
const STRING_SET: u32 = 0x0040_37f0;
const FREE: u32 = 0x00aa_10f0;
/// `00b606f0` (unnamed, `thiscall`): the destructor `00b60c90` runs.
const DESTRUCTOR_00B606F0: u32 = 0x00b6_06f0;
/// The map helpers: the item allocation `0043a010` (`thiscall` on the
/// map's allocator +0xC), the clone map lookup `00a53c30` (`thiscall`,
/// `(key, &value)`), `GetNext` of the plain maps `00b57f20` and of the
/// `NiPointer` map `00e6e630` (`thiscall`, `(&position, &key, &value)`),
/// `RemoveAll` `00b601a0` and `RemoveAt` `00e91fd0` (one word), the map
/// destructor body `00b60360`.
const ALLOCATE_ITEM: u32 = 0x0043_a010;
const MAP_LOOKUP: u32 = 0x00a5_3c30;
const MAP_GET_NEXT: u32 = 0x00b5_7f20;
const POINTER_MAP_GET_NEXT: u32 = 0x00e6_e630;
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
const MAP_REMOVE_AT: u32 = 0x00e9_1fd0;
const MAP_DESTRUCTOR: u32 = 0x00b6_0360;
/// The list helpers: `RemoveHead` `00b602d0`, `RemoveTail` `00e7d5b0`,
/// `RemoveAll` `00b71300` (each `thiscall`, no stack words).
const LIST_REMOVE_HEAD: u32 = 0x00b6_02d0;
const LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;
const LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// The list-item pool's lock (`011c5f80`, entered through `0040fbf0` with
/// 0), its entry count (`011c5f84`) and free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// The cached element's destructor `00bd8f90` and its test `00bd8670`
/// (`thiscall`, one word; non-zero keeps the element).
const ELEMENT_DESTRUCTOR: u32 = 0x00bd_8f90;
const ELEMENT_TEST: u32 = 0x00bd_8670;

/// An `NiPointer`'s release: the count at +4 decremented, the object
/// deleted through its vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An `NiPointer` member's release and clear.
fn release_and_clear(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(member, 0);
    }
}

/// An `NiPointer` member's release without clearing.
fn release_member(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
    }
}

/// The first item of a hash map (`+4` size, `+8` buckets): the head of the
/// first non-empty bucket, 0 when all are empty.
fn first_item(e: &Engine, map: u32) -> u32 {
    let size = e.mem.u32(map + 4);
    let buckets = e.mem.u32(map + 8);
    for index in 0..size {
        let head = e.mem.u32(buckets + 4 * index);
        if head != 0 {
            return head;
        }
    }
    0
}

/// The list item's return to the pool, inlined: the lock entered, the
/// item's element and previous link cleared, the item pushed on the free
/// list, the lock left (its count down by one, the owner cleared at 0).
fn free_item(e: &mut Engine, item: u32) {
    e.mem.set_u32(item + 8, 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(item + 4, 0);
    let free = e.mem.u32(NODE_POOL_FREE_LIST);
    e.mem.set_u32(item, free);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, item);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
}

/// The function-local static hash map shape `00b61170` and `00b61890`
/// build on their first call: the base vtable, the hash size, the count 0,
/// the buckets (`00aa1070`, zeroed with `memset`), the final vtable and the
/// `atexit` registration of its destructor.
fn build_static_map(e: &mut Engine, map: u32, base_vtable: u32, vtable: u32, destructor: u32) {
    e.mem.set_u32(map, base_vtable);
    e.mem.set_u32(map + 4, 0x25);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(NI_ALLOC, &args![0x94u32]).u32();
    let size = e.mem.u32(map + 4);
    e.mem.set_u32(map + 8, buckets);
    e.call(MEMSET, &args![buckets, 0u32, size.wrapping_mul(4)]);
    e.mem.set_u32(map, vtable);
    e.call(ATEXIT, &args![destructor]);
}

// Translated from 00b60b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010ade9c` entry: an `NiRefObject`
/// (vtable, count 0, `ms_uiObjects` incremented), its vtable, and the
/// pointers +8 and +0x14 to +0x24 cleared. Returns `this`.
pub fn fn_00b60b10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    e.mem.set_u32(base, VTABLE_010ADE9C);
    for offset in [0x8u32, 0x14, 0x18, 0x1c, 0x20, 0x24] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00b60b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010ade9c` entry's destructor: the object at +0x24 (its
/// count at +8) released, deleted through its slot 0 with 1 when the count
/// reaches 0; the `NiPointer`s +0x20, +0x1C, +0x18, +0x14 and +8 released;
/// then `NiRefObject`'s vtable and `ms_uiObjects` decremented.
// Translation incomplete: 00b60b50 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27b9f, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b60b50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let object = e.mem.u32(base + 0x24);
    if object != 0 {
        let left = e.call(INTERLOCKED_DECREMENT, &args![object + 8]).u32();
        if left == 0 && object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
    }
    for offset in [0x20u32, 0x1c, 0x18, 0x14, 0x8] {
        release_member(e, base + offset);
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
}

// Translated from 00b60c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a scalar deleting destructor: `00b606f0`, then the memory
/// manager's `Deallocate` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00b60c90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_00B606F0, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b60cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010adea8` class: the
/// `BSShaderLightingProperty` base constructor, the vtable, the words
/// +0x80, +0x84, +0x88 and the `u16`s +0x8C, +0x8E cleared, the pointer at
/// +0x80 released and cleared when set (it is null here), +0x7C cleared and
/// `iShaderPropertyType` 4. Returns `this`.
// Translation incomplete: 00b60cb0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27bf2, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b60cb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(LIGHTING_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, VTABLE_010ADEA8);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x88, 0);
    e.mem.set_u16(base + 0x8c, 0);
    e.mem.set_u16(base + 0x8e, 0);
    release_and_clear(e, base + 0x80);
    e.mem.set_u32(base + 0x7c, 0);
    e.mem.set_u32(base + 0x1c, 4);
    this
}

// Translated from 00b60d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010adea8` class's destructor: its vtable, the pointer at
/// +0x80 released and cleared, +0x7C cleared, the buffer at +0x88 given
/// back to the memory manager when set and cleared with the `u16`s +0x8E
/// and +0x8C, then the member destructors of +0x84 and +0x80 and the
/// `BSShaderLightingProperty` base destructor.
// Translation incomplete: 00b60d80 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27c42, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b60d80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADEA8);
    release_and_clear(e, base + 0x80);
    e.mem.set_u32(base + 0x7c, 0);
    let buffer = e.mem.u32(base + 0x88);
    if buffer != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    }
    e.mem.set_u32(base + 0x88, 0);
    e.mem.set_u16(base + 0x8e, 0);
    e.mem.set_u16(base + 0x8c, 0);
    release_member(e, base + 0x84);
    release_member(e, base + 0x80);
    e.call(LIGHTING_DESTRUCTOR, &args![this]);
}

// Translated from 00b60f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010ade9c` entry's scalar deleting destructor: its
/// destructor, then the 0x38-byte free when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00b60f30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b60b50(e, this);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x38u32]);
    }
    this
}

// Translated from 00b60f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, no arguments): a new 0x98-byte `010adea8` object from
/// `00aa13e0` through its constructor; 0 when the allocation fails.
// Translation incomplete: 00b60f50 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27cb2, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b60f50(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x98u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00b60cb0(e, Ptr::new(block)).addr()
}

// Translated from 00b60fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `010adea8` class's vtable slot 19, where `LoadBinary`
/// sits): `00b70260` with the stream, then a string read from the stream
/// (`NiStream::LoadCString` into a word of the argument's stack slot,
/// cleared first) assigned to the string at +0x88 (`004037f0` with length
/// 0) and freed (`00aa10f0`).
pub fn fn_00b60fc0(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(LIGHTING_LOAD_00B70260, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        e.call(LOAD_CSTRING, &args![stream, slot]);
        let text = e.mem.u32(slot.addr());
        e.call(STRING_SET, &args![this.addr() + 0x88, text, 0u32]);
        let text = e.mem.u32(slot.addr());
        e.call(FREE, &args![text]);
    });
}

// Translated from 00b61010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a hash map's `SetAt` with an `NiPointer` value passed by value
/// (`thiscall`, `RET 8`). The key's bucket (slot +4) is searched with the
/// key compare (slot +8); a found item gets the key (unless the byte at
/// +0x10 is set) and the value (the old one released, the new one's count
/// incremented when they differ); otherwise a new item from the map's
/// allocator (+0xC, `0043a010`) is filled by slot +0xC (key and a copy of
/// the value, its count incremented for the copy), linked at the bucket's
/// head, and the count (+0xC) goes up. The value parameter is released at
/// the end (its destructor).
// Translation incomplete: 00b61010 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27cd8, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b61010(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let base = this.addr();
    let index = e.vcall(base, 4, &args![key]).u32();
    let buckets = e.mem.u32(base + 8);
    let mut item = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(base, 8, &args![key, item_key]).bool() {
            if e.mem.u8(base + 0x10) == 0 {
                e.mem.set_u32(item + 4, key);
            }
            let old = e.mem.u32(item + 8);
            if old != value {
                if old != 0 {
                    release(e, old);
                }
                e.mem.set_u32(item + 8, value);
                if value != 0 {
                    e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
                }
            }
            if value != 0 {
                release(e, value);
            }
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.call(ALLOCATE_ITEM, &args![base + 0xc]).u32();
    if value != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
    }
    e.vcall(base, 0xc, &args![item, key, value]);
    let buckets = e.mem.u32(base + 8);
    let slot = buckets.wrapping_add(index.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(item, head);
    let buckets = e.mem.u32(base + 8);
    e.mem
        .set_u32(buckets.wrapping_add(index.wrapping_mul(4)), item);
    let count = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0xc, count.wrapping_add(1));
    if value != 0 {
        release(e, value);
    }
}

// Translated from 00b61170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, no arguments): the first cache map, a function-local
/// static at `011f9f08` built on the first call (hash size 0x25, vtable
/// `010af6b0`, destructor `00fdcc80` given to `atexit`). Returns its
/// address.
// Translation incomplete: 00b61170 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27d0e, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b61170(e: &mut Engine) -> u32 {
    if e.global::<u8>(CACHE_MAP_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(CACHE_MAP_GUARD);
        e.set_global(CACHE_MAP_GUARD, guard | 1);
        build_static_map(
            e,
            CACHE_MAP,
            MAP_BASE_VTABLE_010ADE18,
            MAP_VTABLE_010AF6B0,
            CACHE_MAP_DESTRUCTOR,
        );
    }
    CACHE_MAP
}

// Translated from 00b61220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010adea8` class's scalar deleting destructor: its
/// destructor, then the 0x98-byte free when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00b61220(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b60d80(e, this);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x98u32]);
    }
    this
}

// Translated from 00b61250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the destructor of a 0x40-byte cache record (`00b617f0`'s): the
/// `NiPointer` at +0x2C released and cleared, its map (+0x30) emptied,
/// every element of its list (+0x20) destroyed (`00bd8f90`) and given back
/// to the memory manager, the list emptied; then the same release and
/// emptying again, the map's destructor body (`00b60360`), the member
/// destructor of +0x2C and the list emptied once more.
// Translation incomplete: 00b61250 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27d51, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b61250(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_and_clear(e, base + 0x2c);
    e.call(MAP_REMOVE_ALL, &args![base + 0x30]);
    let mut item = e.mem.u32(base + 0x20);
    while item != 0 {
        let element = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if element != 0 {
            e.call(ELEMENT_DESTRUCTOR, &args![element]);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, element]);
        }
    }
    e.call(LIST_REMOVE_ALL, &args![base + 0x20]);
    release_and_clear(e, base + 0x2c);
    e.call(MAP_REMOVE_ALL, &args![base + 0x30]);
    e.call(MAP_DESTRUCTOR, &args![base + 0x30]);
    release_member(e, base + 0x2c);
    e.call(LIST_REMOVE_ALL, &args![base + 0x20]);
}

/// Unlinks `item` from `list` as `00b61370` and `00b61570` do in the
/// middle of the list (neither head nor tail): its neighbours linked
/// together, the item returned to the pool, the list's count down by one.
fn unlink_middle(e: &mut Engine, list: u32, item: u32) {
    let previous = e.mem.u32(item + 4);
    let next = e.mem.u32(item);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    free_item(e, item);
    let count = e.mem.u32(list + 8);
    e.mem.set_u32(list + 8, count.wrapping_sub(1));
}

/// Destroys a cached element and gives it back to the memory manager; the
/// element count (`011f9ef4`) goes down by one.
fn destroy_element(e: &mut Engine, element: u32) {
    e.call(ELEMENT_DESTRUCTOR, &args![element]);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, element]);
    let count = e.global::<u32>(ELEMENT_COUNT);
    e.set_global(ELEMENT_COUNT, count.wrapping_sub(1));
}

// Translated from 00b61370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): with the allocation context at 0x1C (put
/// back at the end), for every record of the first cache map whose map
/// (+0x30) holds `key`: each element of the record's list (+0x20) for which
/// `00bd8670` with `key` answers 0 is unlinked (`RemoveHead`, `RemoveTail`
/// or the inlined middle removal), destroyed and freed (the element count
/// down by one); then `key` is removed from the record's map.
// Translation incomplete: 00b61370 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27d78, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b61370(e: &mut Engine, key: u32) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1c);
    let map = fn_00b61170(e);
    let first = first_item(e, map);
    e.with_stack(0x10, |e, frame| {
        let (position, slot_key, value, found) = (
            frame.addr(),
            frame.addr() + 4,
            frame.addr() + 8,
            frame.addr() + 0xc,
        );
        e.mem.set_u32(position, first);
        e.mem.set_u32(value, 0);
        let map = fn_00b61170(e);
        if e.mem.u32(map + 0xc) == 0 || first == 0 {
            return;
        }
        loop {
            let map = fn_00b61170(e);
            e.call(MAP_GET_NEXT, &args![map, position, slot_key, value]);
            let record = e.mem.u32(value);
            if record != 0 && e.call(MAP_LOOKUP, &args![record + 0x30, key, found]).bool() {
                let list = record + 0x20;
                let mut item = e.mem.u32(list);
                while item != 0 {
                    let element = e.mem.u32(item + 8);
                    let current = item;
                    item = e.mem.u32(item);
                    if element == 0 || e.call(ELEMENT_TEST, &args![element, key]).u32() != 0 {
                        continue;
                    }
                    if current == e.mem.u32(list) {
                        e.call(LIST_REMOVE_HEAD, &args![list]);
                    } else if current == e.mem.u32(list + 4) {
                        e.call(LIST_REMOVE_TAIL, &args![list]);
                    } else {
                        unlink_middle(e, list, current);
                    }
                    destroy_element(e, element);
                }
                e.call(MAP_REMOVE_AT, &args![record + 0x30, key]);
            }
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
}

// Translated from 00b61570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words): with the allocation context at 0x1C (put
/// back at the end), for every record of the first cache map, each element
/// of its list (+0x20) whose words +0x3C and +0x40 are `first` and `second`
/// has its `u16` +0x12 keys (the array at +0x18) removed from the record's
/// map (+0x30), is unlinked (the inlined head removal, `RemoveTail`, or the
/// inlined middle removal), destroyed and freed (the element count down by
/// one).
// Translation incomplete: 00b61570 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27da8, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b61570(e: &mut Engine, first: u32, second: u32) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1c);
    let map = fn_00b61170(e);
    let start = first_item(e, map);
    e.with_stack(0xc, |e, frame| {
        let (position, slot_key, value) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        e.mem.set_u32(position, start);
        e.mem.set_u32(value, 0);
        let map = fn_00b61170(e);
        if e.mem.u32(map + 0xc) == 0 || start == 0 {
            return;
        }
        loop {
            let map = fn_00b61170(e);
            e.call(MAP_GET_NEXT, &args![map, position, slot_key, value]);
            let record = e.mem.u32(value);
            if record != 0 {
                let mut item = e.mem.u32(record + 0x20);
                while item != 0 {
                    let next = e.mem.u32(item);
                    let element = e.mem.u32(item + 8);
                    if element != 0
                        && first == e.mem.u32(element + 0x3c)
                        && second == e.mem.u32(element + 0x40)
                    {
                        let keys = i32::from(e.mem.u16(element + 0x12));
                        let array = e.mem.u32(element + 0x18);
                        let mut index = 0i32;
                        while index < keys {
                            let key = e.mem.u32(array + 4 * index as u32);
                            let record = e.mem.u32(value);
                            e.call(MAP_REMOVE_AT, &args![record + 0x30, key]);
                            index += 1;
                        }
                        let record = e.mem.u32(value);
                        let list = record + 0x20;
                        let head = e.mem.u32(list);
                        if item == head {
                            let after = e.mem.u32(head);
                            e.mem.set_u32(list, after);
                            if after != 0 {
                                e.mem.set_u32(after + 4, 0);
                            } else {
                                e.mem.set_u32(list + 4, 0);
                            }
                            free_item(e, head);
                            let count = e.mem.u32(list + 8);
                            e.mem.set_u32(list + 8, count.wrapping_sub(1));
                        } else if item == e.mem.u32(list + 4) {
                            e.call(LIST_REMOVE_TAIL, &args![list]);
                        } else {
                            unlink_middle(e, list, item);
                        }
                        destroy_element(e, element);
                    }
                    item = next;
                }
            }
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
}

// Translated from 00b617b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words): whether the first cache map has a record
/// for `key` whose own map (+0x30) holds `inner` (the found value lands in
/// the same local word). Answers the last lookup's result.
pub fn fn_00b617b0(e: &mut Engine, key: u32, inner: u32) -> bool {
    e.with_stack(4, |e, slot| {
        let map = fn_00b61170(e);
        if !e.call(MAP_LOOKUP, &args![map, key, slot]).bool() {
            return false;
        }
        let record = e.mem.u32(slot.addr());
        e.call(MAP_LOOKUP, &args![record + 0x30, inner, slot])
            .bool()
    })
}

// Translated from 00b617f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of a 0x40-byte cache record: +0 and the floats
/// +8..+0x14 cleared, +0x18 1.0, +0x20..+0x2C cleared, and the map at
/// +0x30 (hash size 0x65, count 0, buckets from `00aa1070` zeroed with
/// `memset`, vtable `010ade50` after the base one). Returns `this`.
// Translation incomplete: 00b617f0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27de6, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b617f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 1.0f32.to_bits());
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x30, MAP_BASE_VTABLE_010ADE18);
    e.mem.set_u32(base + 0x34, 0x65);
    e.mem.set_u32(base + 0x3c, 0);
    let buckets = e.call(NI_ALLOC, &args![0x194u32]).u32();
    let size = e.mem.u32(base + 0x34);
    e.mem.set_u32(base + 0x38, buckets);
    e.call(MEMSET, &args![buckets, 0u32, size.wrapping_mul(4)]);
    e.mem.set_u32(base + 0x30, MAP_VTABLE_010ADE50);
    this
}

// Translated from 00b61890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, no arguments): the second cache map, a function-local
/// static at `011f9f1c` built on the first call (hash size 0x25, the byte
/// +0x10 set, vtable `010adf84`, destructor `00fdcc90` given to `atexit`).
/// Returns its address.
// Translation incomplete: 00b61890 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27e1e, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b61890(e: &mut Engine) -> u32 {
    if e.global::<u8>(SECOND_MAP_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(SECOND_MAP_GUARD);
        e.set_global(SECOND_MAP_GUARD, guard | 1);
        e.mem.set_u32(SECOND_MAP, MAP_BASE_VTABLE_010ADE34);
        e.mem.set_u32(SECOND_MAP + 4, 0x25);
        e.mem.set_u32(SECOND_MAP + 0xc, 0);
        let buckets = e.call(NI_ALLOC, &args![0x94u32]).u32();
        let size = e.mem.u32(SECOND_MAP + 4);
        e.mem.set_u32(SECOND_MAP + 8, buckets);
        e.call(MEMSET, &args![buckets, 0u32, size.wrapping_mul(4)]);
        e.set_global(SECOND_MAP_BYTE, 1u8);
        e.mem.set_u32(SECOND_MAP, MAP_VTABLE_010ADF84);
        e.call(ATEXIT, &args![SECOND_MAP_DESTRUCTOR]);
    }
    SECOND_MAP
}

// Translated from 00b61940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words): `00b617b0` for the key at +4 of `object`
/// (the found value lands in the first argument's stack slot).
pub fn fn_00b61940(e: &mut Engine, object: u32, inner: u32) -> bool {
    let key = e.mem.u32(object + 4);
    e.with_stack(4, |e, slot| {
        let map = fn_00b61170(e);
        if !e.call(MAP_LOOKUP, &args![map, key, slot]).bool() {
            return false;
        }
        let record = e.mem.u32(slot.addr());
        e.call(MAP_LOOKUP, &args![record + 0x30, inner, slot])
            .bool()
    })
}

// Translated from 00b62970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, no arguments): destroys every record of the first
/// cache map (`00b61250`, then the memory manager's `Deallocate`), empties
/// the map and clears the counters `011f9ef4` and `011f9ef8`.
pub fn fn_00b62970(e: &mut Engine) {
    let map = fn_00b61170(e);
    let first = first_item(e, map);
    e.with_stack(0xc, |e, frame| {
        let (position, slot_key, value) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        e.mem.set_u32(position, first);
        e.mem.set_u32(value, 0);
        let map = fn_00b61170(e);
        if e.mem.u32(map + 0xc) == 0 || first == 0 {
            return;
        }
        loop {
            let map = fn_00b61170(e);
            e.call(MAP_GET_NEXT, &args![map, position, slot_key, value]);
            let record = e.mem.u32(value);
            if record != 0 {
                fn_00b61250(e, Ptr::new(record));
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, record]);
            }
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
    let map = fn_00b61170(e);
    e.call(MAP_REMOVE_ALL, &args![map]);
    e.set_global(ELEMENT_COUNT, 0u32);
    e.set_global(COUNTER_011F9EF8, 0u32);
}

// Translated from 00b62a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, no arguments): `00b62970`, then for every entry of the
/// second cache map whose byte +0x32 is clear: its `NiPointer` +8 released
/// and cleared, its buffer +0xC given back to the memory manager (also
/// when null) and cleared, its `NiPointer`s +0x14, +0x1C, +0x18 and +0x20
/// released and cleared; then the map emptied. The map's iteration value
/// is an `NiPointer` local, released at the end.
// Translation incomplete: 00b62a20 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f273e8, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b62a20(e: &mut Engine) {
    fn_00b62970(e);
    let map = fn_00b61890(e);
    let first = first_item(e, map);
    e.with_stack(0xc, |e, frame| {
        let (position, slot_key, value) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        e.mem.set_u32(position, first);
        e.mem.set_u32(value, 0);
        let mut entry = 0u32;
        let map = fn_00b61890(e);
        if e.mem.u32(map + 0xc) != 0 {
            if first != 0 {
                loop {
                    let map = fn_00b61890(e);
                    e.call(POINTER_MAP_GET_NEXT, &args![map, position, slot_key, value]);
                    entry = e.mem.u32(value);
                    if entry != 0 && e.mem.u8(entry + 0x32) == 0 {
                        release_and_clear(e, entry + 8);
                        let buffer = e.mem.u32(entry + 0xc);
                        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
                        e.mem.set_u32(entry + 0xc, 0);
                        for offset in [0x14u32, 0x1c, 0x18, 0x20] {
                            release_and_clear(e, entry + offset);
                        }
                    }
                    if e.mem.u32(position) == 0 {
                        break;
                    }
                }
            }
            let map = fn_00b61890(e);
            e.call(MAP_REMOVE_ALL, &args![map]);
        }
        if entry != 0 {
            release(e, entry);
        }
    });
}

/// The word `00b61980` tests first (`011f91c0`: 0 makes nothing; below 2 gives 0x50
/// instances, else 0xE4), the byte that hands loading to the background loader
/// (`011ad86c`), the loader's owner (`01202d98`, its vtable slot +0x48 takes a loader) and the
/// hook called with a finished entry's key (`011f91e8`, `cdecl`, may be null).
const INSTANCING_LEVEL: u32 = 0x011f_91c0;
const BACKGROUND_LOADING: u32 = 0x011a_d86c;
const LOADER_OWNER: u32 = 0x0120_2d98;
const ENTRY_HOOK: u32 = 0x011f_91e8;
/// The second cache map's lookup (`00e92760`, `thiscall`: key, the out `NiPointer`), the
/// out-of-line `NiPointer` destructors (`00b66f20` for the entry, `0045cec0`), the `NiPointer`
/// assignments from a raw pointer (`0066b0d0`) and from another `NiPointer` (`00c663d0`), all
/// `thiscall`.
const ENTRY_LOOKUP: u32 = 0x00e9_2760;
const ENTRY_RELEASE: u32 = 0x00b6_6f20;
const POINTER_RELEASE: u32 = 0x0045_cec0;
const POINTER_SET: u32 = 0x0066_b0d0;
const POINTER_COPY: u32 = 0x00c6_63d0;
/// The loader (0x600 bytes from the memory manager): its constructor (`00b60640`, `thiscall`:
/// the key), the entry's loader assignment (`00b60280`, `thiscall`: the loader; its count at
/// +8) and its first loaded object (`00b60750`, `thiscall`: the out `NiPointer`).
const LOADER_CONSTRUCT: u32 = 0x00b6_0640;
const LOADER_ASSIGN: u32 = 0x00b6_0280;
const LOADER_FIRST_OBJECT: u32 = 0x00b6_0750;
/// The archive path of a key (`00af4250`, `cdecl`: key, buffer, 0x104), `BSStream`'s
/// constructor (`0043cfd0`), `BSStream::Load` (`00c3a8a0`: the path and 0) and destructor
/// (`0043d090`), `thiscall`; the stream's top objects (+0x220) and their count (+0x228).
const ARCHIVE_PATH: u32 = 0x00af_4250;
const STREAM_CONSTRUCT: u32 = 0x0043_cfd0;
const STREAM_LOAD: u32 = 0x00c3_a8a0;
const STREAM_DESTRUCT: u32 = 0x0043_d090;
/// The cast of a loaded object to `NiAVObject` (`00653290`, `thiscall`: the RTTI; the object
/// or 0), `NiAVObject`'s and `NiBillboardNode`'s RTTI (`011f4280`, `011f4aa0`).
const DYNAMIC_CAST: u32 = 0x0065_3290;
const AV_OBJECT_RTTI: u32 = 0x011f_4280;
const BILLBOARD_NODE_RTTI: u32 = 0x011f_4aa0;
/// `NiTriShapeData::GetTriList` (`00a7b850`) and `NiTriStripsData::GetStripLists`
/// (`00a75f10`), `thiscall`; the vector constructor iterator (`00401050`, `stdcall`: array,
/// element size, count, constructor) with the colour constructor `004a7800`;
/// `NiTriShapeDynamicData::NiTriShapeDynamicData` (`00a816a0`, 0x5C bytes, eleven words) and
/// the strips' dynamic data constructor (`00a92340`, 0x54 bytes, thirteen words).
const TRI_LIST: u32 = 0x00a7_b850;
const STRIP_LISTS: u32 = 0x00a7_5f10;
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const COLOR_CONSTRUCT: u32 = 0x004a_7800;
const SHAPE_DATA_CONSTRUCT: u32 = 0x00a8_16a0;
const STRIPS_DATA_CONSTRUCT: u32 = 0x00a9_2340;
/// `NiAVObject::GetProperty` (`00a59d30`, `thiscall`: the property type), the property base
/// constructor (`00a5c200`, `thiscall`) and `NiAlphaProperty`'s vtable (RTTI).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_CONSTRUCT: u32 = 0x00a5_c200;
const ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;

/// The frame of `00b61980`: the entry `NiPointer` (+0x24), the loaded object's (+0x50), a
/// temporary `NiPointer` (+0x38), the texture `NiPointer` (+0x28), the stream (+0xBC) and the
/// path buffer (+0x68C, 0x104 bytes).
const SOURCE_FRAME: u32 = 0x790;
const FRAME_ENTRY: u32 = 0x24;
const FRAME_TEXTURE: u32 = 0x28;
const FRAME_TEMPORARY: u32 = 0x38;
const FRAME_OBJECT: u32 = 0x50;
const FRAME_STREAM: u32 = 0xbc;
const FRAME_PATH: u32 = 0x68c;

/// An array's byte size as the code computes it (`MUL`, `SETO`, `NEG`, `OR`): all ones on
/// overflow.
fn array_bytes(count: u32, size: u32) -> u32 {
    let bytes = u64::from(count) * u64::from(size);
    if bytes > u64::from(u32::MAX) {
        u32::MAX
    } else {
        bytes as u32
    }
}

/// A memory manager allocation.
fn allocate(e: &mut Engine, bytes: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32()
}

/// The instance count kept below `0xFFFF / per_instance` (`IDIV`, the quotient's low half): a
/// limit not above the count replaces it.
fn limit_instances(instances: u32, per_instance: u32) -> u32 {
    let limit = (0xffff_i32 / per_instance as i32) as u32 & 0xffff;
    if (instances as u16) < limit as u16 {
        instances
    } else {
        limit
    }
}

/// An inlined `NiPointer` assignment (the count at +4, released through slot +4); the new
/// object's count is raised only when it is not null if `checked`.
fn assign_counted(e: &mut Engine, member: u32, new: u32, checked: bool) {
    let old = e.mem.u32(member);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(member, new);
    if new != 0 || !checked {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

/// The background loading of an entry: a 0x600-byte loader from the memory manager
/// (`00b60640` with the key; null when the allocation fails) set as the entry's loader (+0x24,
/// `00b60280`) and handed to the owner's slot +0x48.
fn start_loader(e: &mut Engine, entry: u32, descriptor: u32) {
    let block = allocate(e, 0x600);
    let loader = if block == 0 {
        0
    } else {
        let key = e.mem.u32(descriptor);
        e.call(LOADER_CONSTRUCT, &args![block, key]).u32()
    };
    e.call(LOADER_ASSIGN, &args![entry + 0x24, loader]);
    let owner = e.mem.u32(LOADER_OWNER);
    let loader = e.mem.u32(entry + 0x24);
    e.vcall(owner, 0x48, &args![loader]);
}

/// The first geometry under `root`: each child (+0xA0, the count `u16` +0xA6) asked with its
/// slot +0x18. The code never moves past `root`: without such a child it scans `root` again
/// and again (it does not return), and a null `root` gives 0.
fn find_geometry(e: &mut Engine, root: u32) -> u32 {
    let mut found = 0u32;
    loop {
        if root == 0 {
            return found;
        }
        let mut index = 0u32;
        if 0 < e.mem.u16(root + 0xa6) {
            loop {
                let children = e.mem.u32(root + 0xa0);
                let child = e.mem.u32(children.wrapping_add(4 * index));
                if child != 0 {
                    found = e.vcall(child, 0x18, &[]).u32();
                    if found != 0 {
                        return found;
                    }
                }
                index = index.wrapping_add(1);
                if index >= u32::from(e.mem.u16(root + 0xa6)) {
                    break;
                }
            }
        }
        if found != 0 {
            return found;
        }
    }
}

/// What the instanced geometry builders hand to the entry: the dynamic data (or 0), the
/// per-vertex instance numbers (floats), the vertex count, the instance count, the `u16`
/// stored at +0x34 and whether the geometry is strips.
struct Instanced {
    data: u32,
    instance_numbers: u32,
    vertices: u32,
    instances: u32,
    count: u32,
    strips: u8,
}

/// The per-instance arrays: positions and normals (12 bytes a vertex), colours (16, built by
/// the vector constructor; 0 when the allocation fails), texture coordinates (8) and instance
/// numbers (4).
struct Arrays {
    positions: u32,
    normals: u32,
    colors: u32,
    uvs: u32,
    numbers: u32,
}

fn allocate_arrays(e: &mut Engine, count: u32) -> Arrays {
    let positions = allocate(e, array_bytes(count, 0xc));
    let normals = allocate(e, array_bytes(count, 0xc));
    let block = allocate(e, array_bytes(count, 0x10));
    let colors = if block == 0 {
        0
    } else {
        e.call(
            VECTOR_CONSTRUCT,
            &args![block, 0x10u32, count, COLOR_CONSTRUCT],
        );
        block
    };
    let uvs = allocate(e, array_bytes(count, 8));
    let numbers = allocate(e, array_bytes(count, 4));
    Arrays {
        positions,
        normals,
        colors,
        uvs,
        numbers,
    }
}

/// Copies `words` words from `source` to `destination` (`MOV` by `MOV`).
fn copy_words(e: &mut Engine, destination: u32, source: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// The instanced copy of an `NiTriShape`'s data: every instance gets the vertices, normals,
/// colours and texture coordinates (+0x20..+0x2C of the data) and its number as a float, and
/// the triangle list (+0x44 indices) offset by its first vertex.
fn build_shape(e: &mut Engine, geometry: u32, instances: u32) -> Instanced {
    let data = e.mem.u32(geometry + 0xb8);
    let vertices = u32::from(e.mem.u16(data + 8));
    let source_vertices = e.mem.u32(data + 0x20);
    let index_total = e.mem.u32(data + 0x44);
    let triangles = e.call(TRI_LIST, &args![data]).u32();
    let instances = limit_instances(instances, vertices);
    let indices = index_total & 0xffff;
    let instances = limit_instances(instances, indices) & 0xffff;
    let count = instances.wrapping_mul(vertices);
    let arrays = allocate_arrays(e, count);
    let index_buffer = allocate(e, array_bytes(indices.wrapping_mul(instances), 2));
    let source_colors = e.mem.u32(data + 0x28);
    let source_uvs = e.mem.u32(data + 0x2c);
    let source_normals = e.mem.u32(data + 0x24);
    let mut written = 0u32;
    let mut base = 0u32;
    if (instances as i32) > 0 {
        let mut instance = 0u32;
        let mut at = 0u32;
        loop {
            if (vertices as i32) > 0 {
                let number = x87_store32(f64::from(instance as i32));
                for vertex in 0..vertices {
                    let slot = at + vertex;
                    copy_words(
                        e,
                        arrays.positions + 0xc * slot,
                        source_vertices + 0xc * vertex,
                        3,
                    );
                    copy_words(
                        e,
                        arrays.normals + 0xc * slot,
                        source_normals + 0xc * vertex,
                        3,
                    );
                    copy_words(
                        e,
                        arrays.colors + 0x10 * slot,
                        source_colors + 0x10 * vertex,
                        4,
                    );
                    copy_words(e, arrays.uvs + 8 * slot, source_uvs + 8 * vertex, 2);
                    e.mem.set_u32(arrays.numbers + 4 * slot, number);
                }
            }
            let mut index = 0u32;
            while (index as i32) < indices as i32 {
                let value = e.mem.u16(triangles + 2 * index).wrapping_add(base as u16);
                e.mem.set_u16(index_buffer + 2 * written, value);
                index += 1;
                written += 1;
            }
            at = at.wrapping_add(vertices);
            instance = instance.wrapping_add(1);
            base = base.wrapping_add(vertices);
            if (instance as i32) >= instances as i32 {
                break;
            }
        }
    }
    let triangle_count = ((indices as i32) / 3) as u32;
    let block = e.call(NI_NEW, &args![0x5cu32]).u32();
    let shape = if block == 0 {
        0
    } else {
        e.call(
            SHAPE_DATA_CONSTRUCT,
            &args![
                block,
                count,
                arrays.positions,
                arrays.normals,
                arrays.colors,
                arrays.uvs,
                1u32,
                0u32,
                triangle_count.wrapping_mul(instances),
                index_buffer,
                count,
                0u32
            ],
        )
        .u32()
    };
    Instanced {
        data: shape,
        instance_numbers: arrays.numbers,
        vertices,
        instances,
        count: triangle_count & 0xffff,
        strips: 0,
    }
}

/// The instanced copy of an `NiTriStrips`' data (one strip, its length the first `u16` of
/// +0x48): every instance gets the arrays copied (`memcpy`) and its number as a float, its
/// strip offset by its first vertex, and all but the last are joined to the next by repeating
/// the strip's last index and the next one's first (once more for an odd length). The index
/// count goes into a 2-byte block from the memory manager.
fn build_strips(e: &mut Engine, geometry: u32, instances: u32) -> Instanced {
    let data = e.mem.u32(geometry + 0xb8);
    let vertices = u32::from(e.mem.u16(data + 8));
    let lengths = e.mem.u32(data + 0x48);
    let source_vertices = e.mem.u32(data + 0x20);
    let strip = e.call(STRIP_LISTS, &args![data]).u32();
    let instances = limit_instances(instances, vertices);
    let length = u32::from(e.mem.u16(lengths));
    // `AND 0x80000001` and its sign fix-up (`DEC`, `OR`, `INC`), never taken for a
    // zero-extended `u16`: the length's oddness.
    let span = length + 2 + u32::from(length & 1 != 0);
    let instances = limit_instances(instances, span) & 0xffff;
    let count = instances.wrapping_mul(vertices);
    let arrays = allocate_arrays(e, count);
    let length = e.mem.u16(lengths);
    let stride = u32::from((u16::from(length & 1 != 0) + 2).wrapping_add(length));
    let index_count = instances
        .wrapping_sub(1)
        .wrapping_mul(stride)
        .wrapping_add(u32::from(length));
    let index_buffer = allocate(e, array_bytes(index_count, 2));
    let source_normals = e.mem.u32(data + 0x24);
    let source_colors = e.mem.u32(data + 0x28);
    let source_uvs = e.mem.u32(data + 0x2c);
    let mut written = 0u32;
    if (instances as i32) > 0 {
        let last = instances - 1;
        let mut instance = 0u32;
        let mut at = 0u32;
        loop {
            e.call(
                MEMCPY,
                &args![arrays.positions + 0xc * at, source_vertices, 0xc * vertices],
            );
            e.call(
                MEMCPY,
                &args![arrays.normals + 0xc * at, source_normals, 0xc * vertices],
            );
            e.call(
                MEMCPY,
                &args![arrays.colors + 0x10 * at, source_colors, 0x10 * vertices],
            );
            e.call(
                MEMCPY,
                &args![arrays.uvs + 8 * at, source_uvs, 8 * vertices],
            );
            if (vertices as i32) > 0 {
                let number = x87_store32(f64::from(instance as i32));
                for vertex in 0..vertices {
                    e.mem.set_u32(arrays.numbers + 4 * (at + vertex), number);
                }
            }
            if 0 < e.mem.u16(lengths) {
                let first = instance.wrapping_mul(vertices) as u16;
                let mut index = 0u32;
                loop {
                    let value = e.mem.u16(strip + 2 * index).wrapping_add(first);
                    e.mem.set_u16(index_buffer + 2 * written, value);
                    index += 1;
                    written += 1;
                    if (index as i32) >= i32::from(e.mem.u16(lengths)) {
                        break;
                    }
                }
            }
            if (instance as i32) < last as i32 {
                let length = u32::from(e.mem.u16(lengths));
                let here = (instance & 0xffff).wrapping_mul(vertices) as u16;
                let closing =
                    here.wrapping_add(e.mem.u16(strip.wrapping_add(2 * length).wrapping_sub(2)));
                written += 1;
                e.mem.set_u16(index_buffer + 2 * written - 2, closing);
                let next = ((instance & 0xffff) + 1).wrapping_mul(vertices) as u16;
                let opening = e.mem.u16(strip).wrapping_add(next);
                e.mem.set_u16(index_buffer + 2 * written, opening);
                written += 1;
                // The same never-taken sign fix-up before the oddness test.
                if e.mem.u16(lengths) & 1 == 1 {
                    let again = e.mem.u16(strip).wrapping_add(next);
                    e.mem.set_u16(index_buffer + 2 * written, again);
                    written += 1;
                }
            }
            at = at.wrapping_add(vertices);
            instance = instance.wrapping_add(1);
            if (instance as i32) >= instances as i32 {
                break;
            }
        }
    }
    let total = allocate(e, 2);
    e.mem.set_u16(total, written as u16);
    let length = e.mem.u16(lengths);
    // Once more the never-taken sign fix-up.
    let joined = if length & 1 == 1 {
        length.wrapping_add(3)
    } else {
        length.wrapping_add(2)
    };
    let block = e.call(NI_NEW, &args![0x54u32]).u32();
    let strips = if block == 0 {
        0
    } else {
        e.call(
            STRIPS_DATA_CONSTRUCT,
            &args![
                block,
                count,
                arrays.positions,
                arrays.normals,
                arrays.colors,
                arrays.uvs,
                1u32,
                0u32,
                written.wrapping_sub(2),
                1u32,
                total,
                index_buffer,
                count,
                0u32
            ],
        )
        .u32()
    };
    Instanced {
        data: strips,
        instance_numbers: arrays.numbers,
        vertices,
        instances,
        count: u32::from(joined),
        strips: 1,
    }
}

/// A new `NiAlphaProperty` (0x1C bytes: the property base, its vtable, flags 0xEC, test
/// reference 0), or 0 when the allocation fails.
fn new_alpha_property(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(PROPERTY_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, ALPHA_PROPERTY_VTABLE);
    e.mem.set_u16(block + 0x18, 0xec);
    e.mem.set_u8(block + 0x1a, 0);
    block
}

/// The entry's fields from the geometry and its instanced data: the texture of a type-4
/// texturing property (property 3, its slot +0xB4), the geometry's alpha property (or a new
/// one) and a second new one with the same test reference and flags `0x1201`/`0x1200` (after
/// masks `0xF3FF`/`0xF3FE`), property 4, the bound's radius (+0x1C of the data) and whether
/// the root is an `NiBillboardNode`; then the entry hook; the locals are released.
fn finish_entry(
    e: &mut Engine,
    frame: u32,
    entry: u32,
    geometry: u32,
    root: u32,
    built: &Instanced,
    descriptor: u32,
) -> u32 {
    let texture_slot = frame + FRAME_TEXTURE;
    e.mem.set_u32(texture_slot, 0);
    if e.call(GET_PROPERTY, &args![geometry, 3u32]).u32() != 0 {
        let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
        if e.mem.u32(property + 0x1c) == 4 {
            let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            if property != 0 {
                let texture = e.vcall(property, 0xb4, &[]).u32();
                if texture != 0 {
                    e.mem.set_u32(texture_slot, texture);
                    e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
                }
            }
        }
    }
    let found = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
    let mut reference = 0u8;
    let alpha = if found != 0 {
        reference = e.mem.u8(found + 0x1a);
        found
    } else {
        new_alpha_property(e)
    };
    e.mem.set_u8(alpha + 0x1a, reference);
    let flags = (e.mem.u16(alpha + 0x18) & 0xf3ff) | 0x1201;
    e.mem.set_u16(alpha + 0x18, flags);
    let second = new_alpha_property(e);
    e.mem.set_u8(second + 0x1a, reference);
    let flags = (e.mem.u16(second + 0x18) & 0xf3fe) | 0x1200;
    e.mem.set_u16(second + 0x18, flags);
    let material = e.call(GET_PROPERTY, &args![geometry, 4u32]).u32();
    let data = e.mem.u32(geometry + 0xb8);
    let radius = e.mem.u32(data + 0x1c);
    e.mem.set_u8(entry + 0x30, built.strips);
    assign_counted(e, entry + 8, built.data, true);
    e.mem.set_u32(entry + 0xc, built.instance_numbers);
    e.mem.set_u32(entry + 0x10, built.vertices);
    let texture = e.mem.u32(texture_slot);
    assign_counted(e, entry + 0x14, texture, true);
    assign_counted(e, entry + 0x1c, second, false);
    assign_counted(e, entry + 0x18, alpha, false);
    assign_counted(e, entry + 0x20, material, true);
    e.mem.set_u32(entry + 0x28, x87_quiet_bits(radius));
    e.mem.set_u32(entry + 0x2c, built.instances);
    e.mem.set_u16(entry + 0x34, built.count as u16);
    e.mem.set_u8(entry + 0x31, 0);
    if root != 0 {
        let mut rtti = e.vcall(root, 8, &[]).u32();
        let mut billboard = false;
        while rtti != 0 {
            if rtti == BILLBOARD_NODE_RTTI {
                billboard = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
        if billboard && root != 0 {
            e.mem.set_u8(entry + 0x31, 1);
        }
    }
    let hook = e.mem.u32(ENTRY_HOOK);
    let key = e.mem.u32(descriptor);
    if hook != 0 {
        e.call(hook, &args![key]);
    }
    let texture = e.mem.u32(texture_slot);
    if texture != 0 {
        release(e, texture);
    }
    let object = e.mem.u32(frame + FRAME_OBJECT);
    release(e, object);
    release(e, entry);
    entry
}

/// The way out without an entry: the object and entry `NiPointer`s destroyed; 0.
fn source_failed(e: &mut Engine, frame: u32) -> u32 {
    e.call(POINTER_RELEASE, &args![frame + FRAME_OBJECT]);
    e.call(ENTRY_RELEASE, &args![frame + FRAME_ENTRY]);
    0
}

/// `00b61980`'s body over its frame (see there).
fn source_in_frame(e: &mut Engine, frame: u32, descriptor: u32, key: u32) -> u32 {
    let entry_slot = frame + FRAME_ENTRY;
    let object_slot = frame + FRAME_OBJECT;
    e.mem.set_u32(entry_slot, 0);
    let map = fn_00b61890(e);
    let found = e.call(ENTRY_LOOKUP, &args![map, key, entry_slot]).u8() != 0;
    let entry = if found {
        e.mem.u32(entry_slot)
    } else {
        let block = e.call(NI_NEW, &args![0x38u32]).u32();
        let made = if block == 0 {
            0
        } else {
            fn_00b60b10(e, Ptr::new(block)).addr()
        };
        e.call(POINTER_ASSIGN, &args![entry_slot, made]);
        let entry = e.mem.u32(entry_slot);
        e.mem.set_u8(entry + 0x32, 1);
        let loader = e.mem.u32(entry + 0x24);
        if loader != 0 {
            let left = e.call(INTERLOCKED_DECREMENT, &args![loader + 8]).u32();
            if left == 0 && loader != 0 {
                e.vcall(loader, 0, &args![1u32]);
            }
            e.mem.set_u32(entry + 0x24, 0);
        }
        // The entry passed by value: its copy's count.
        e.call(INTERLOCKED_INCREMENT, &args![entry + 4]);
        let key = e.mem.u32(descriptor);
        let map = fn_00b61890(e);
        fn_00b61010(e, Ptr::new(map), key, entry);
        if e.mem.u8(BACKGROUND_LOADING) != 0 {
            start_loader(e, entry, descriptor);
            e.call(ENTRY_RELEASE, &args![entry_slot]);
            return 0;
        }
        entry
    };
    if e.mem.u8(entry + 0x32) == 0 {
        release(e, entry);
        return entry;
    }
    e.mem.set_u32(object_slot, 0);
    let entry = e.mem.u32(entry_slot);
    if e.mem.u8(BACKGROUND_LOADING) != 0 && e.mem.u32(entry + 0x24) == 0 {
        start_loader(e, entry, descriptor);
        e.call(POINTER_RELEASE, &args![object_slot]);
        e.call(ENTRY_RELEASE, &args![entry_slot]);
        return 0;
    }
    let loader = e.mem.u32(entry + 0x24);
    if loader != 0 {
        if (e.mem.u32(loader + 0xc) as i32) < 4 {
            return source_failed(e, frame);
        }
        let temporary = frame + FRAME_TEMPORARY;
        let first = e.call(LOADER_FIRST_OBJECT, &args![loader, temporary]).u32();
        e.call(POINTER_COPY, &args![object_slot, first]);
        e.call(POINTER_RELEASE, &args![temporary]);
        e.call(LOADER_ASSIGN, &args![entry + 0x24, 0u32]);
    } else {
        let path = frame + FRAME_PATH;
        let stream = frame + FRAME_STREAM;
        let key = e.mem.u32(descriptor);
        e.call(ARCHIVE_PATH, &args![key, path, 0x104u32]);
        e.call(STREAM_CONSTRUCT, &args![stream]);
        e.call(STREAM_LOAD, &args![stream, path, 0u32]);
        if e.mem.u32(stream + 0x228) != 0 {
            let objects = e.mem.u32(stream + 0x220);
            let first = e.mem.u32(objects);
            e.call(POINTER_SET, &args![object_slot, first]);
        }
        e.call(STREAM_DESTRUCT, &args![stream]);
    }
    let object = e.mem.u32(object_slot);
    if object == 0 {
        return source_failed(e, frame);
    }
    e.mem.set_u8(entry + 0x32, 0);
    let cast = e.call(DYNAMIC_CAST, &args![object, AV_OBJECT_RTTI]).u32();
    let root = e.vcall(cast, 0xc, &[]).u32();
    let geometry = find_geometry(e, root);
    let strips = e.vcall(geometry, 0x20, &[]).u32() != 0;
    let instances = if (e.mem.u32(INSTANCING_LEVEL) as i32) < 2 {
        0x50
    } else {
        0xe4
    };
    let built = if strips {
        build_strips(e, geometry, instances)
    } else {
        build_shape(e, geometry, instances)
    };
    finish_entry(e, frame, entry, geometry, root, &built, descriptor)
}

// Translated from 00b61980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word: a descriptor whose first word is its key): with the
/// allocation context at 0x1C (put back at the end), the instanced geometry entry of the
/// descriptor's model. Nothing (0) while `011f91c0` is 0 or for a null key. The second cache
/// map's entry for the key is looked up (`00e92760`) or made (a 0x38-byte `00b60b10` entry,
/// marked unbuilt (+0x32), its loader cleared, stored with `00b61010`); with background
/// loading (`011ad86c`) a new entry gets a loader and the answer is 0. A built entry is
/// answered as it is. Otherwise the model comes from the entry's loader (0 while it has fewer
/// than 4 at +0xC) or is loaded now (`BSStream::Load` of the key's archive path); its first
/// geometry (`NiAVObject` cast, slot +0xC, then the children's slot +0x18) is copied 0x50
/// times (0xE4 when `011f91c0` is 2 or more; fewer when the indices would pass 0xFFFF) into
/// dynamic shape (slot +0x20 zero) or strips data, and the entry gets the data and the
/// geometry's properties. Answers the entry, its count as it was (the local reference
/// released).
// Translation incomplete: 00b61980 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27f0d, state variable) and the /GS cookie check (00ec408c) are not translated.
pub fn fn_00b61980(e: &mut Engine, descriptor: u32) -> u32 {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1c);
    let answer = if e.mem.u32(INSTANCING_LEVEL) == 0 || e.mem.u32(descriptor) == 0 {
        0
    } else {
        let key = e.mem.u32(descriptor);
        e.with_stack(SOURCE_FRAME, |e, frame| {
            source_in_frame(e, frame.addr(), descriptor, key)
        })
    };
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
    answer
}

/// `00a5f860` (unnamed, `thiscall` on an `NiPointer`, one word): its
/// assignment. `memcpy` (`00ec44d0`). The plain map's `SetAt` (`00b57ea0`,
/// `thiscall`, `(key, value)`). The cached element's constructor
/// (`00bd8ae0`, `thiscall`, four words), its source registration
/// (`00bd86a0`, one word), point add (`00bd8550`, two words) and finish
/// (`00bd8750`, none). The list's `AddTail` (`00b58570`, one word).
const POINTER_ASSIGN: u32 = 0x00a5_f860;
const MEMCPY: u32 = 0x00ec_44d0;
const MAP_SET_AT: u32 = 0x00b5_7ea0;
const ELEMENT_CONSTRUCTOR: u32 = 0x00bd_8ae0;
const ELEMENT_REGISTER: u32 = 0x00bd_86a0;
const ELEMENT_ADD_POINT: u32 = 0x00bd_8550;
const ELEMENT_FINISH: u32 = 0x00bd_8750;
const LIST_ADD_TAIL: u32 = 0x00b5_8570;
/// `0057ddb0` (unnamed, `cdecl`, one word: a point): the random seed of a
/// point; `srand` (`00ecada6`) and `rand` (`00ecadb8`), `cdecl`; `_ftol2`
/// (`00ec62c0`, the value in ST0, here a leading `f64`).
const POINT_SEED: u32 = 0x0057_ddb0;
const SRAND: u32 = 0x00ec_ada6;
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
/// The `double`s 65534.0 (`010adfa0`), 0.5 (`01011588`), 32767.0
/// (`01029790`), 100.0 (`01017a40`) and 0.0 (`01012060`), and the `float`
/// 0.99 (`0106682c`) a scale of at least 1 becomes.
const RAND_SPAN_010ADFA0: u32 = 0x010a_dfa0;
const HALF_01011588: u32 = 0x0101_1588;
const RAND_MAX_01029790: u32 = 0x0102_9790;
const HUNDRED_01017A40: u32 = 0x0101_7a40;
const ZERO_01012060: u32 = 0x0101_2060;
const SCALE_LIMIT_0106682C: u32 = 0x0106_682c;

// Translated from 00b62bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, five words): with the allocation context at 0x1C (put
/// back at the end), the cached element of `descriptor` for `(first,
/// second)`. `*element` is cleared and `00b61980` asked for the
/// descriptor's source (0 without one: answers 0). The first cache map's
/// record for the descriptor's key (+4) is found, or made (a 0x40-byte
/// record from the memory manager, `00b617f0`, its `NiPointer` +0x2C set to
/// the source, the descriptor's first 0x20 bytes copied in, `SetAt`). An
/// element of the record's list (+0x20) with room (`u16` +0x10 above +0x12)
/// and the same words +0x3C/+0x40 is taken; when there is none, or the one
/// found is full, a new 0x48-byte element (`00bd8ae0` with the source,
/// `first`, `second`, 0; null when the allocation fails) replaces it (a
/// full one is destroyed and freed), the element count goes up, the element
/// registers the descriptor (`00bd86a0`), is added at the list's tail and
/// `owner`'s vtable slot +0xDC gets its +4 word and 1. Answers the
/// element's free room (`u16` +0x10 minus +0x12). The record lands in the
/// `element` argument's stack slot (a local word here).
// Translation incomplete: 00b62bd0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27f5e, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00b62bd0(
    e: &mut Engine,
    descriptor: u32,
    first: u32,
    second: u32,
    element: u32,
    owner: u32,
) -> u32 {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1c);
    e.mem.set_u32(element, 0);
    let source = fn_00b61980(e, descriptor);
    if source == 0 {
        e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
        return 0;
    }
    let found = e.with_stack(4, |e, slot| {
        let key = e.mem.u32(descriptor + 4);
        let map = fn_00b61170(e);
        if e.call(MAP_LOOKUP, &args![map, key, slot]).bool() {
            return e.mem.u32(slot.addr());
        }
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
            .u32();
        e.mem.set_u32(slot.addr(), block);
        let record = if block == 0 {
            0
        } else {
            fn_00b617f0(e, Ptr::new(block)).addr()
        };
        e.call(POINTER_ASSIGN, &args![record + 0x2c, source]);
        e.call(MEMCPY, &args![record, descriptor, 0x20u32]);
        let key = e.mem.u32(descriptor + 4);
        let map = fn_00b61170(e);
        e.call(MAP_SET_AT, &args![map, key, record]);
        record
    });
    let list = found + 0x20;
    let mut item = e.mem.u32(found + 0x20);
    let mut current = e.mem.u32(element);
    let mut make = false;
    if current == 0 {
        // `CMP EDI,0; JZ` at the end of each step tests the register that
        // holds `*element` (0 here): the walk always goes on.
        while item != 0 {
            let candidate = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if e.mem.u16(candidate + 0x12) != e.mem.u16(candidate + 0x10)
                && first == e.mem.u32(candidate + 0x3c)
                && second == e.mem.u32(candidate + 0x40)
            {
                e.mem.set_u32(element, candidate);
                break;
            }
        }
        current = e.mem.u32(element);
        if current == 0 {
            make = true;
        }
    }
    if !make {
        let room = e
            .mem
            .u16(current + 0x10)
            .wrapping_sub(e.mem.u16(current + 0x12));
        if room == 0 {
            if current != 0 {
                e.call(ELEMENT_DESTRUCTOR, &args![current]);
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, current]);
            }
            make = true;
        }
    }
    if make {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x48u32])
            .u32();
        let made = if block == 0 {
            0
        } else {
            e.call(
                ELEMENT_CONSTRUCTOR,
                &args![block, source, first, second, 0u32],
            )
            .u32()
        };
        e.mem.set_u32(element, made);
        let count = e.global::<u32>(ELEMENT_COUNT);
        e.set_global(ELEMENT_COUNT, count.wrapping_add(1));
        let made = e.mem.u32(element);
        e.call(ELEMENT_REGISTER, &args![made, descriptor]);
        e.call(LIST_ADD_TAIL, &args![list, element]);
        let made = e.mem.u32(element);
        let word = e.mem.u32(made + 4);
        e.vcall(owner, 0xdc, &args![word, 1u32]);
    }
    let current = e.mem.u32(element);
    let room = e
        .mem
        .u16(current + 0x10)
        .wrapping_sub(e.mem.u16(current + 0x12));
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
    u32::from(room)
}

// Translated from 00b62de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, eight words): with the allocation context at 0x1C (put
/// back at the end), adds `count` (`u16`) points (`points`, 12 bytes each,
/// with their `scales`) to cached elements of `descriptor`: the counter
/// `011f9ef8` grows by `count`; while points are left, an element with room
/// is taken (`00b62bd0`; none stops the function) and gets as many points as
/// it has room for (`00bd8550` with `key` and a 16-byte copy: the point and
/// a fourth word made of two random draws seeded from the point
/// (`0057ddb0`, `srand`, `rand`): the floor of `(2 * rand / 32767 - 1) *
/// descriptor[+0xC] * 100` plus the scale `((rand / 65534 - 1) *
/// descriptor[+0x10] * 0.5 + 1 - descriptor[+0x10]) * scale`, the scale
/// kept in `0..0.99`), then finished (`00bd8750`). Then `owner`'s vtable
/// slot +0xBC and `key` set in the record's map (+0x30) under itself. The
/// x87 operations follow the exe instruction by instruction; the element
/// lands in the `count` argument's stack slot (a local word here).
// Translation incomplete: 00b62de0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27f88, state variable) and the security-cookie bookkeeping are not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b62de0(
    e: &mut Engine,
    key: u32,
    first: u32,
    second: u32,
    owner: u32,
    descriptor: u32,
    points: u32,
    scales: u32,
    count: u32,
) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1c);
    let mut remaining = count as u16;
    let total = e
        .global::<u32>(COUNTER_011F9EF8)
        .wrapping_add(u32::from(remaining));
    e.set_global(COUNTER_011F9EF8, total);
    let completed = e.with_stack(0x14, |e, frame| {
        let slot = frame.addr();
        let copy = frame.addr() + 4;
        let mut index: u32 = 0;
        while remaining > 0 {
            e.mem.set_u32(slot, 0);
            let room = fn_00b62bd0(e, descriptor, first, second, slot, owner) as u16;
            if e.mem.u32(slot) == 0 {
                return false;
            }
            let room = i32::from(room);
            let mut added = 0i32;
            while added < room && remaining != 0 {
                let point = points + index * 0xc;
                let seed = e.call(POINT_SEED, &args![point]).u32();
                e.call(SRAND, &args![seed]);
                for word in 0..3 {
                    let value = x87_quiet_bits(e.mem.u32(point + 4 * word));
                    e.mem.set_u32(copy + 4 * word, value);
                }
                let draw = e.call(RAND, &[]).i32();
                let span = f64::from_bits(e.global::<u64>(RAND_SPAN_010ADFA0));
                let shifted = x87_store32(x87_sub(x87_div(f64::from(draw), span), 1.0));
                let spread = e.mem.u32(descriptor + 0x10);
                let half = f64::from_bits(e.global::<u64>(HALF_01011588));
                let varied = x87_mul(x87_mul(x87_load32(spread), half), x87_m32(shifted));
                let base = x87_sub(1.0, x87_load32(spread));
                let factor = x87_add(base, varied);
                let scale = e.mem.u32(scales + 4 * index);
                let mut scaled = x87_store32(x87_mul(factor, x87_m32(scale)));
                // `FCOM` with 1.0 (`TEST AH,1`), then with 0.0 (`TEST
                // AH,0x41`): the scale stays in 0..0.99.
                if !(x87_load32(scaled) < 1.0) {
                    scaled = x87_quiet_bits(e.global::<u32>(SCALE_LIMIT_0106682C));
                } else if 0.0 > x87_load32(scaled) {
                    scaled = 0;
                }
                let draw = e.call(RAND, &[]).i32();
                let maximum = f64::from_bits(e.global::<u64>(RAND_MAX_01029790));
                let unit = x87_div(f64::from(draw), maximum);
                let doubled = x87_add(unit, unit);
                let signed = x87_store32(x87_sub(doubled, 1.0));
                let hundred = f64::from_bits(e.global::<u64>(HUNDRED_01017A40));
                let angle = x87_store32(x87_mul(
                    x87_mul(x87_load32(e.mem.u32(descriptor + 0xc)), x87_m32(signed)),
                    hundred,
                ));
                let angle = x87_load32(angle);
                let whole = e.call(FTOL, &args![angle]).i32();
                let whole = x87_load32(x87_store32(f64::from(whole)));
                let zero = f64::from_bits(e.global::<u64>(ZERO_01012060));
                // `FCOMP double 0.0; TEST AH,1`: one less below 0 (or NaN).
                let fraction = x87_sub(angle, whole);
                let floor = if !(fraction >= zero) {
                    x87_sub(whole, 1.0)
                } else {
                    whole
                };
                let floor = x87_store32(floor);
                let fourth = x87_store32(x87_add(x87_load32(floor), x87_m32(scaled)));
                e.mem.set_u32(copy + 0xc, fourth);
                let target = e.mem.u32(slot);
                e.call(ELEMENT_ADD_POINT, &args![target, copy, key]);
                added += 1;
                remaining = remaining.wrapping_sub(1);
                index += 1;
            }
            let target = e.mem.u32(slot);
            e.call(ELEMENT_FINISH, &args![target]);
        }
        true
    });
    if completed {
        e.vcall(owner, 0xbc, &[]);
        e.with_stack(4, |e, slot| {
            let record_key = e.mem.u32(descriptor + 4);
            let map = fn_00b61170(e);
            e.call(MAP_LOOKUP, &args![map, record_key, slot]);
            let record = e.mem.u32(slot.addr());
            e.call(MAP_SET_AT, &args![record + 0x30, key, key]);
        });
    }
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
}

/// `00b702b0` (unnamed; `thiscall`, two words): the base class's copy `00b60a50` runs first.
const BASE_COPY_00B702B0: u32 = 0x00b7_02b0;

/// The `NiPointer` at `from + offset` copied to `to + offset` when they differ: the old one
/// released (count at +4, slot +4 at 0), the new one's count at +4 incremented.
fn copy_pointer(e: &mut Engine, from: u32, to: u32, offset: u32) {
    let old = e.mem.u32(to + offset);
    if old == e.mem.u32(from + offset) {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    let new = e.mem.u32(from + offset);
    e.mem.set_u32(to + offset, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
    }
}

// Translated from 00b60a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, two words): copies the `010adea8` object `this` into `clone`: the
/// base's `00b702b0(clone, cloning)`, the `NiPointer`s +0x80 and +0x84 (reference counts kept),
/// then the words +0x90, +0x94 and +0x7c.
pub fn fn_00b60a50(e: &mut Engine, this: Ptr, clone: Ptr, cloning: u32) {
    e.call(BASE_COPY_00B702B0, &args![this, clone, cloning]);
    let from = this.addr();
    let to = clone.addr();
    copy_pointer(e, from, to, 0x80);
    copy_pointer(e, from, to, 0x84);
    for offset in [0x90u32, 0x94, 0x7c] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

// Translated from 00b60c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a hash map's scalar deleting destructor (`thiscall`, one word): the map destructor
/// body `00b60360`, then the memory manager's `Deallocate` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00b60c70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAP_DESTRUCTOR, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// What `00b60d60` answers: the address `011f9f00` (no RTTI locator for the vtable; the word
/// is the class's `NiRTTI` by its place in the vtable, unconfirmed).
const RECORD_011F9F00: u32 = 0x011f_9f00;

// Translated from 00b60d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 2 of the vtable `010adea8`, `thiscall`
/// without reading `this`: answers the address `011f9f00`.
pub fn fn_00b60d60(_e: &mut Engine, _this: Ptr) -> u32 {
    RECORD_011F9F00
}

// Translated from 00b60d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 54 of the vtable `010adea8`, `thiscall`, one
/// word: stores it at +0x90.
pub fn fn_00b60d70(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x90, value);
}

// Translated from 00b60e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b60e80 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f27c7a, state variable) and the security-cookie bookkeeping are not translated.
/// Unnamed (`thiscall`, one word): the `010adea8` object's clone. With the thread's allocation
/// context (+0x2b4) set to 0x1c, a 0x98-byte block from `00aa13e0` is constructed (`00b60cb0`;
/// null when the allocation fails) and `this` copied into it (`00b60a50` with `cloning`); the
/// context is restored and the clone answered.
pub fn fn_00b60e80(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1c);
    let block = e.call(NI_NEW, &args![0x98u32]).u32();
    let clone = if block != 0 {
        fn_00b60cb0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    fn_00b60a50(e, this, Ptr::new(clone), cloning);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
    clone
}

// Translated from 00b63070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, no stack words): calls `this`'s vtable slot +0xa8, then jumps to its
/// slot +0xac (a tail call: what that slot answers in EAX, EDX and ST0 is this function's
/// result).
pub fn fn_00b63070(e: &mut Engine, this: Ptr) -> Ret {
    e.vcall(this.addr(), 0xa8, &args![]);
    e.vcall(this.addr(), 0xac, &args![])
}

/// The uniform-form entry of [`fn_00b63070`]: `this`; the tail call's registers pass through.
fn fn_00b63070_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00b63070(e, Ptr::new(a[0]))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_0a50, fn_00b60a50(Ptr, Ptr, u32)),
        entry!(0x00b6_0c70, fn_00b60c70(Ptr, u32) -> Ptr),
        entry!(0x00b6_0d60, fn_00b60d60(Ptr) -> u32),
        entry!(0x00b6_0d70, fn_00b60d70(Ptr, u32)),
        entry!(0x00b6_0e80, fn_00b60e80(Ptr, u32) -> u32),
        (0x00b6_3070, fn_00b63070_entry as AbiFn),
        entry!(0x00b6_0b10, fn_00b60b10(Ptr) -> Ptr),
        entry!(0x00b6_0b50, fn_00b60b50(Ptr)),
        entry!(0x00b6_0c90, fn_00b60c90(Ptr, u32) -> Ptr),
        entry!(0x00b6_0cb0, fn_00b60cb0(Ptr) -> Ptr),
        entry!(0x00b6_0d80, fn_00b60d80(Ptr)),
        entry!(0x00b6_0f30, fn_00b60f30(Ptr, u32) -> Ptr),
        entry!(0x00b6_0f50, fn_00b60f50() -> u32),
        entry!(0x00b6_0fc0, fn_00b60fc0(Ptr, u32)),
        entry!(0x00b6_1010, fn_00b61010(Ptr, u32, u32)),
        entry!(0x00b6_1170, fn_00b61170() -> u32),
        entry!(0x00b6_1220, fn_00b61220(Ptr, u32) -> Ptr),
        entry!(0x00b6_1250, fn_00b61250(Ptr)),
        entry!(0x00b6_1370, fn_00b61370(u32)),
        entry!(0x00b6_1570, fn_00b61570(u32, u32)),
        entry!(0x00b6_17b0, fn_00b617b0(u32, u32) -> bool),
        entry!(0x00b6_17f0, fn_00b617f0(Ptr) -> Ptr),
        entry!(0x00b6_1890, fn_00b61890() -> u32),
        entry!(0x00b6_1940, fn_00b61940(u32, u32) -> bool),
        entry!(0x00b6_1980, fn_00b61980(u32) -> u32),
        entry!(0x00b6_2970, fn_00b62970()),
        entry!(0x00b6_2a20, fn_00b62a20()),
        entry!(0x00b6_2bd0, fn_00b62bd0(u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00b6_2de0,
            fn_00b62de0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    /// Interlocked doubles on real counts, allocation and free doubles,
    /// the static map pages mapped, a log.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        for allocate in [NI_NEW, NI_ALLOC] {
            e.register(allocate, |e, a| e.mem.alloc(a[0]).into_ret());
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        for quiet in [NI_DELETE, MEMORY_DEALLOCATE, MEMSET, ATEXIT, LOCK_ENTER] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x011c_5000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// A reference-counted object (count at +4) whose delete slot (+4) is
    /// the double `0x0040_9004`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        if !e.mem.is_mapped(0x0090_9000) {
            e.put_vtable(0x0090_9000, &[0x0040_9000, 0x0040_9004]);
            e.register(0x0040_9000, |_, _| Ret::default());
            e.register(0x0040_9004, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x0090_9000);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn entry_constructor_and_destructor() {
        let mut e = rig();
        let this: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.mem.set_u32(this.addr() + 0x14, 0xdead);
        e.call(0x00b6_0b10, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010ADE9C);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT), 1);
        // +0x24 counts at +8 and is deleted through slot 0 with 1.
        let special = counted(&mut e, 0);
        e.mem.set_u32(special + 8, 1);
        e.mem.set_u32(this.addr() + 0x24, special);
        let pointer = counted(&mut e, 1);
        e.mem.set_u32(this.addr() + 0x8, pointer);
        e.call(0x00b6_0f30, &args![this, 1u32]);
        assert_eq!(calls_to(&e, 0x0040_9000), vec![vec![special, 1]]);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![pointer]]);
        assert_eq!(e.mem.u32(this.addr()), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT), 0);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x38]]);
    }

    #[test]
    fn scalar_destructor_00b60c90_frees_through_the_memory_manager() {
        let mut e = rig();
        e.register(DESTRUCTOR_00B606F0, |_, _| Ret::default());
        e.call(0x00b6_0c90, &args![0x5000u32, 0u32]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00b6_0c90, &args![0x5000u32, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x5000]]
        );
    }

    #[test]
    fn lighting_class_constructor_destructor_and_factory() {
        let mut e = rig();
        e.register(LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        let object = e.call(0x00b6_0f50, &[]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), VTABLE_010ADEA8);
        assert_eq!(e.mem.u32(object + 0x1c), 4);
        let pointer = counted(&mut e, 2);
        e.mem.set_u32(object + 0x80, pointer);
        e.mem.set_u32(object + 0x88, 0x7777);
        e.mem.set_u16(object + 0x8c, 3);
        e.call(0x00b6_1220, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x80), 0);
        assert_eq!(e.mem.u32(pointer + 4), 1);
        assert_eq!(e.mem.u16(object + 0x8c), 0);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7777]]
        );
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![object, 0x98]]);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00b6_0f50, &[]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_string() {
        let mut e = rig();
        e.register(LIGHTING_LOAD_00B70260, |_, _| Ret::default());
        e.register(LOAD_CSTRING, |e, a| {
            e.mem.set_u32(a[1], 0x00aa_0000);
            Ret::default()
        });
        e.register(STRING_SET, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call(0x00b6_0fc0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_to(&e, LIGHTING_LOAD_00B70260),
            vec![vec![0x5000, 0x6000]]
        );
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![0x5088, 0x00aa_0000, 0]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x00aa_0000]]);
    }

    /// A map whose hash is the key modulo 4, compare is equality, and the
    /// fill slot stores the key and value (+4, +8).
    fn pointer_map(e: &mut Engine) -> Ptr {
        e.put_vtable(0x0090_a000, &[0, 0x0040_a004, 0x0040_a008, 0x0040_a00c]);
        e.register(0x0040_a004, |_, a| (a[1] % 4).into_ret());
        e.register(0x0040_a008, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x0040_a00c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(ALLOCATE_ITEM, |e, _| e.mem.alloc(0xc).into_ret());
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map, 0x0090_a000);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, buckets);
        Ptr::new(map)
    }

    #[test]
    fn set_at_adds_then_replaces_the_value() {
        let mut e = rig();
        let map = pointer_map(&mut e);
        let first = counted(&mut e, 0);
        e.call(0x00b6_1010, &args![map, 6u32, first]);
        let buckets = e.mem.u32(map.addr() + 8);
        let item = e.mem.u32(buckets + 8);
        assert_eq!(e.mem.u32(item + 4), 6);
        assert_eq!(e.mem.u32(item + 8), first);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
        // The parameter's copy kept one count (the double stores it).
        assert_eq!(e.mem.u32(first + 4), 0);
        let second = counted(&mut e, 0);
        e.call(0x00b6_1010, &args![map, 6u32, second]);
        assert_eq!(e.mem.u32(item + 8), second);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
    }

    #[test]
    fn static_maps_are_built_once() {
        let mut e = rig();
        assert_eq!(e.call(0x00b6_1170, &[]).u32(), CACHE_MAP);
        assert_eq!(e.mem.u32(CACHE_MAP), MAP_VTABLE_010AF6B0);
        assert_eq!(e.mem.u32(CACHE_MAP + 4), 0x25);
        assert_ne!(e.mem.u32(CACHE_MAP + 8), 0);
        e.call(0x00b6_1170, &[]);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![CACHE_MAP_DESTRUCTOR]]);
        assert_eq!(e.call(0x00b6_1890, &[]).u32(), SECOND_MAP);
        assert_eq!(e.mem.u32(SECOND_MAP), MAP_VTABLE_010ADF84);
        assert_eq!(e.mem.u8(SECOND_MAP_BYTE), 1);
        e.call(0x00b6_1890, &[]);
        assert_eq!(calls_to(&e, ATEXIT).len(), 2);
    }

    #[test]
    fn record_constructor_and_destructor() {
        let mut e = rig();
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.call(0x00b6_17f0, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0x18), 1.0);
        assert_eq!(e.mem.u32(this.addr() + 0x30), MAP_VTABLE_010ADE50);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 0x65);
        assert_eq!(calls_to(&e, MEMSET)[0][2], 0x194);
        for quiet in [
            MAP_REMOVE_ALL,
            MAP_DESTRUCTOR,
            LIST_REMOVE_ALL,
            ELEMENT_DESTRUCTOR,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        let element = e.mem.alloc(0x48);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, element);
        e.mem.set_u32(this.addr() + 0x20, item);
        let source = counted(&mut e, 1);
        e.mem.set_u32(this.addr() + 0x2c, source);
        e.call(0x00b6_1250, &args![this]);
        assert_eq!(calls_to(&e, ELEMENT_DESTRUCTOR), vec![vec![element]]);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![source]]);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL).len(), 2);
        assert_eq!(e.mem.u32(this.addr() + 0x2c), 0);
    }

    /// One record in the first cache map (bucket 0), its list holding the
    /// given elements; the lookup double finds `key` in the record's map.
    fn cache_with_record(e: &mut Engine, elements: &[u32]) -> u32 {
        let map = e.call(0x00b6_1170, &[]).u32();
        let record = e.mem.alloc(0x40);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, record);
        let buckets = e.mem.u32(map + 8);
        e.mem.set_u32(buckets, item);
        e.mem.set_u32(map + 0xc, 1);
        e.register(MAP_GET_NEXT, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[3], e.mem.u32(item + 8));
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let mut previous = 0u32;
        for element in elements {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 8, *element);
            e.mem.set_u32(node + 4, previous);
            if previous == 0 {
                e.mem.set_u32(record + 0x20, node);
            } else {
                e.mem.set_u32(previous, node);
            }
            previous = node;
        }
        e.mem.set_u32(record + 0x24, previous);
        e.mem.set_u32(record + 0x28, elements.len() as u32);
        record
    }

    #[test]
    fn removing_a_key_drops_the_elements_that_do_not_keep_it() {
        let mut e = rig();
        let kept = e.mem.alloc(0x48);
        let dropped_head = e.mem.alloc(0x48);
        let dropped_middle = e.mem.alloc(0x48);
        let dropped_tail = e.mem.alloc(0x48);
        let record = cache_with_record(
            &mut e,
            &[dropped_head, kept, dropped_middle, kept, dropped_tail],
        );
        e.register(MAP_LOOKUP, |_, _| 1u32.into_ret());
        e.register_double(ELEMENT_TEST, move |_, a| u32::from(a[0] == kept).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let head = e.mem.u32(a[0]);
            let next = e.mem.u32(head);
            e.mem.set_u32(a[0], next);
            e.mem.set_u32(next + 4, 0);
            Ret::default()
        });
        for quiet in [LIST_REMOVE_TAIL, ELEMENT_DESTRUCTOR, MAP_REMOVE_AT] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.set_global(ELEMENT_COUNT, 5u32);
        e.call(0x00b6_1370, &args![0x77u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_DESTRUCTOR),
            vec![vec![dropped_head], vec![dropped_middle], vec![dropped_tail]]
        );
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD).len(), 1);
        assert_eq!(calls_to(&e, LIST_REMOVE_TAIL).len(), 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT), 2);
        assert_eq!(calls_to(&e, MAP_REMOVE_AT), vec![vec![record + 0x30, 0x77]]);
        // The middle item went back to the pool.
        assert_ne!(e.mem.u32(NODE_POOL_FREE_LIST), 0);
        assert_eq!(e.mem.u32(record + 0x28), 4);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 0);
    }

    #[test]
    fn removing_a_pair_drops_matching_elements_and_their_keys() {
        let mut e = rig();
        let matching = e.mem.alloc(0x48);
        e.mem.set_u32(matching + 0x3c, 1);
        e.mem.set_u32(matching + 0x40, 2);
        e.mem.set_u16(matching + 0x12, 2);
        let keys = e.mem.alloc(8);
        e.mem.set_u32(keys, 0x51);
        e.mem.set_u32(keys + 4, 0x52);
        e.mem.set_u32(matching + 0x18, keys);
        let other = e.mem.alloc(0x48);
        let record = cache_with_record(&mut e, &[matching, other]);
        for quiet in [LIST_REMOVE_TAIL, ELEMENT_DESTRUCTOR, MAP_REMOVE_AT] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.call(0x00b6_1570, &args![1u32, 2u32]);
        assert_eq!(
            calls_to(&e, MAP_REMOVE_AT),
            vec![vec![record + 0x30, 0x51], vec![record + 0x30, 0x52]]
        );
        assert_eq!(calls_to(&e, ELEMENT_DESTRUCTOR), vec![vec![matching]]);
        // The head was unlinked inline: the second item is the head now.
        let head = e.mem.u32(record + 0x20);
        assert_eq!(e.mem.u32(head + 8), other);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(e.mem.u32(record + 0x28), 1);
    }

    #[test]
    fn nested_lookups() {
        let mut e = rig();
        let record = e.mem.alloc(0x40);
        e.register_double(MAP_LOOKUP, move |e, a| {
            if a[0] == CACHE_MAP {
                if a[1] != 5 {
                    return 0u32.into_ret();
                }
                e.mem.set_u32(a[2], record);
                return 1u32.into_ret();
            }
            u32::from(a[1] == 9).into_ret()
        });
        assert!(e.call(0x00b6_17b0, &args![5u32, 9u32]).bool());
        assert!(!e.call(0x00b6_17b0, &args![5u32, 8u32]).bool());
        assert!(!e.call(0x00b6_17b0, &args![4u32, 9u32]).bool());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 5);
        assert!(e.call(0x00b6_1940, &args![object, 9u32]).bool());
        assert_eq!(calls_to(&e, MAP_LOOKUP).last().unwrap()[0], record + 0x30);
    }

    #[test]
    fn clearing_the_caches() {
        let mut e = rig();
        for quiet in [MAP_REMOVE_ALL, MAP_DESTRUCTOR, LIST_REMOVE_ALL] {
            e.register(quiet, |_, _| Ret::default());
        }
        let record = cache_with_record(&mut e, &[]);
        e.set_global(ELEMENT_COUNT, 3u32);
        e.set_global(COUNTER_011F9EF8, 4u32);
        // The second map: one entry with byte +0x32 clear.
        let map = e.call(0x00b6_1890, &[]).u32();
        let entry = counted(&mut e, 1);
        let held = counted(&mut e, 1);
        e.mem.set_u32(entry + 0x14, held);
        e.mem.set_u32(entry + 0xc, 0x4321);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, entry);
        let buckets = e.mem.u32(map + 8);
        e.mem.set_u32(buckets + 4, item);
        e.mem.set_u32(map + 0xc, 1);
        e.register(POINTER_MAP_GET_NEXT, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[3], e.mem.u32(item + 8));
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.call(0x00b6_2a20, &[]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, record]));
        assert!(calls_to(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, 0x4321]));
        assert_eq!(e.global::<u32>(ELEMENT_COUNT), 0);
        assert_eq!(e.global::<u32>(COUNTER_011F9EF8), 0);
        assert_eq!(e.mem.u32(entry + 0x14), 0);
        assert_eq!(e.mem.u32(held + 4), 0);
        // The iteration's local reference to the entry is released at the end.
        assert_eq!(e.mem.u32(entry + 4), 0);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 4);
    }

    #[test]
    fn element_lookup_makes_a_record_and_an_element() {
        let mut e = rig();
        let source = e.mem.alloc(0x40);
        serve_source(&mut e, source);
        e.register(MAP_LOOKUP, |_, _| 0u32.into_ret());
        for quiet in [
            POINTER_ASSIGN,
            MEMCPY,
            MAP_SET_AT,
            ELEMENT_REGISTER,
            LIST_ADD_TAIL,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.register(ELEMENT_CONSTRUCTOR, |e, a| {
            e.mem.set_u16(a[0] + 0x10, 8);
            e.mem.set_u32(a[0] + 4, 0x4444);
            a[0].into_ret()
        });
        e.register(0x0040_b0dc, |_, _| Ret::default());
        e.put_vtable(0x0090_b000, &[0; 0xe0 / 4]);
        e.mem.set_u32(0x0090_b000 + 0xdc, 0x0040_b0dc);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0090_b000);
        let descriptor = e.mem.alloc(0x20);
        e.mem.set_u32(descriptor, 0x77);
        e.mem.set_u32(descriptor + 4, 0x99);
        let element = e.mem.alloc(4);
        let room = e
            .call(0x00b6_2bd0, &args![descriptor, 1u32, 2u32, element, owner])
            .u32();
        assert_eq!(room, 8);
        let made = e.mem.u32(element);
        assert_eq!(
            calls_to(&e, ELEMENT_CONSTRUCTOR),
            vec![vec![made, source, 1, 2, 0]]
        );
        assert_eq!(calls_to(&e, 0x0040_b0dc), vec![vec![owner, 0x4444, 1]]);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT), 1);
        assert_eq!(calls_to(&e, MEMCPY)[0][1..], [descriptor, 0x20]);
        // No source: 0 and nothing made.
        e.set_global(INSTANCING_LEVEL, 0u32);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b6_2bd0, &args![descriptor, 1u32, 2u32, element, owner])
                .u32(),
            0
        );
        assert!(calls_to(&e, ELEMENT_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn adding_points_fills_elements_with_jittered_scales() {
        let mut e = rig();
        e.map(0x010a_d000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_9000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0106_6000, 0x1000);
        e.set_global(RAND_SPAN_010ADFA0, 65534.0f64);
        e.set_global(HALF_01011588, 0.5f64);
        e.set_global(RAND_MAX_01029790, 32767.0f64);
        e.set_global(HUNDRED_01017A40, 100.0f64);
        e.set_global(ZERO_01012060, 0.0f64);
        e.set_global(SCALE_LIMIT_0106682C, 0.99f32);
        // The element source and the record (its list empty): each new
        // element has room for 2 points; adding one fills a place.
        let source = e.mem.alloc(0x40);
        serve_source(&mut e, source);
        let record = e.mem.alloc(0x40);
        e.register_double(MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], record);
            1u32.into_ret()
        });
        e.register(ELEMENT_CONSTRUCTOR, |e, a| {
            e.mem.set_u16(a[0] + 0x10, 2);
            a[0].into_ret()
        });
        e.register(ELEMENT_REGISTER, |_, _| Ret::default());
        e.register(LIST_ADD_TAIL, |_, _| Ret::default());
        e.register(0x0040_c0dc, |_, _| Ret::default());
        e.register(POINT_SEED, |_, _| 7u32.into_ret());
        e.register(SRAND, |_, _| Ret::default());
        e.register_double(RAND, |_, _| 32767u32.into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(ELEMENT_FINISH, |_, _| Ret::default());
        e.register(MAP_SET_AT, |_, _| Ret::default());
        let points_added = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = points_added.clone();
        e.register_double(ELEMENT_ADD_POINT, move |e, a| {
            seen.borrow_mut()
                .push([e.mem.f32(a[1]), e.mem.f32(a[1] + 0xc)]);
            let used = e.mem.u16(a[0] + 0x12);
            e.mem.set_u16(a[0] + 0x12, used + 1);
            Ret::default()
        });
        e.register(0x0040_c0bc, |_, _| Ret::default());
        e.put_vtable(0x0090_c000, &[0; 0xe0 / 4]);
        e.mem.set_u32(0x0090_c000 + 0xbc, 0x0040_c0bc);
        e.mem.set_u32(0x0090_c000 + 0xdc, 0x0040_c0dc);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0090_c000);
        let descriptor = e.mem.alloc(0x20);
        e.mem.set_u32(descriptor, 0x77);
        e.mem.set_f32(descriptor + 0xc, 0.5);
        e.mem.set_f32(descriptor + 0x10, 0.0);
        let points = e.mem.alloc(3 * 0xc);
        for i in 0..3u32 {
            e.mem.set_f32(points + 0xc * i, i as f32);
        }
        let scales = e.mem.alloc(3 * 4);
        for (i, scale) in [0.5f32, 2.0, -1.0].iter().enumerate() {
            e.mem.set_f32(scales + 4 * i as u32, *scale);
        }
        e.call(
            0x00b6_2de0,
            &args![0x31u32, 1u32, 2u32, owner, descriptor, points, scales, 3u32],
        );
        // rand = 32767: the angle (2 - 1) * 0.5 * 100 = 50, floor 50; the
        // scale factor (32767/65534 - 1) * 0 * 0.5 + 1 - 0 = 1, so the
        // scales 0.5, 2 (kept below 1: 0.99) and -1 (raised to 0).
        let added = points_added.borrow();
        assert_eq!(added.len(), 3);
        assert_eq!(added[0], [0.0, 50.5]);
        assert_eq!(added[1], [1.0, 50.99]);
        assert_eq!(added[2], [2.0, 50.0]);
        assert_eq!(calls_to(&e, ELEMENT_FINISH).len(), 2);
        assert_eq!(calls_to(&e, 0x0040_c0bc).len(), 1);
        assert_eq!(e.global::<u32>(COUNTER_011F9EF8), 3);
        // No element source: stops before the owner's slot.
        e.set_global(INSTANCING_LEVEL, 0u32);
        e.call_log = Some(vec![]);
        e.call(
            0x00b6_2de0,
            &args![0x31u32, 1u32, 2u32, owner, descriptor, points, scales, 3u32],
        );
        assert!(calls_to(&e, 0x0040_c0bc).is_empty());
    }

    /// `00b61980` answers `source` for any key: instancing on, the lookup finding `source`
    /// built (+0x32 clear; its count 2, raised for the local reference the lookup fills).
    fn serve_source(e: &mut Engine, source: u32) {
        e.map(0x011a_d000, 0x1000);
        e.set_global(INSTANCING_LEVEL, 1u32);
        e.mem.set_u32(source + 4, 2);
        e.mem.set_u8(source + 0x32, 0);
        e.register_double(ENTRY_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], source);
            let count = e.mem.u32(source + 4) + 1;
            e.mem.set_u32(source + 4, count);
            1u32.into_ret()
        });
    }

    /// Doubles for `00b61980`: instancing level 1, a descriptor with key 0x77, the lookup
    /// finding an unbuilt entry (count 2, no loader), the model loaded by the stream (its top
    /// object a node whose one child is a geometry of `vertices` vertices; slot +0x20 answers
    /// `strips`), the dynamic data constructors answering their block, no properties.
    struct Model {
        descriptor: u32,
        entry: u32,
        geometry: u32,
        data: u32,
        root: u32,
    }

    fn model(e: &mut Engine, vertices: u16, strips: bool) -> Model {
        e.map(0x011a_d000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global(INSTANCING_LEVEL, 1u32);
        let descriptor = e.mem.alloc(0x20);
        e.mem.set_u32(descriptor, 0x77);
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry, 0x0090_9000);
        e.mem.set_u32(entry + 4, 2);
        e.mem.set_u8(entry + 0x32, 1);
        counted(e, 0);
        e.register_double(ENTRY_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], entry);
            let count = e.mem.u32(entry + 4) + 1;
            e.mem.set_u32(entry + 4, count);
            1u32.into_ret()
        });
        // The node: slot +0xC (as root), slot +8 (its RTTI); one child, slot +0x18.
        let root = e.mem.alloc(0xb0);
        let child = e.mem.alloc(8);
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x50);
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, child);
        e.mem.set_u32(root + 0xa0, children);
        e.mem.set_u16(root + 0xa6, 1);
        e.put_vtable(
            0x0090_a000,
            &[0x0040_a000, 0x0040_a004, 0x0040_a008, 0x0040_a00c],
        );
        e.put_vtable(
            0x0090_a100,
            &[0, 0, 0, 0, 0, 0, 0x0040_a018, 0, 0x0040_a020],
        );
        e.mem.set_u32(root, 0x0090_a000);
        e.mem.set_u32(child, 0x0090_a100);
        e.mem.set_u32(geometry, 0x0090_a100);
        e.register(0x0040_a000, |_, _| Ret::default());
        e.register(0x0040_a004, |_, _| Ret::default());
        e.register(0x0040_a008, |_, _| BILLBOARD_NODE_RTTI.into_ret());
        e.register_double(0x0040_a00c, move |_, _| root.into_ret());
        e.register_double(0x0040_a018, move |_, _| geometry.into_ret());
        e.register_double(0x0040_a020, move |_, _| u32::from(strips).into_ret());
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u16(data + 8, vertices);
        e.mem.set_f32(data + 0x1c, 4.5);
        for (offset, words) in [(0x20u32, 3u32), (0x24, 3), (0x28, 4), (0x2c, 2)] {
            let block = e.mem.alloc(4 * words * u32::from(vertices));
            for i in 0..words * u32::from(vertices) {
                e.mem.set_u32(block + 4 * i, offset << 8 | i);
            }
            e.mem.set_u32(data + offset, block);
        }
        for quiet in [
            ARCHIVE_PATH,
            STREAM_CONSTRUCT,
            STREAM_DESTRUCT,
            POINTER_RELEASE,
            ENTRY_RELEASE,
            VECTOR_CONSTRUCT,
            PROPERTY_CONSTRUCT,
            GET_PROPERTY,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.register_double(STREAM_LOAD, move |e, a| {
            let objects = e.mem.alloc(4);
            e.mem.set_u32(objects, root);
            e.mem.set_u32(a[0] + 0x220, objects);
            e.mem.set_u32(a[0] + 0x228, 1);
            Ret::default()
        });
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(SHAPE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STRIPS_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        Model {
            descriptor,
            entry,
            geometry,
            data,
            root,
        }
    }

    #[test]
    fn element_source_needs_instancing_and_a_key() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        let descriptor = e.mem.alloc(4);
        e.mem.set_u32(descriptor, 0x77);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 5);
        assert_eq!(fn_00b61980(&mut e, descriptor), 0);
        e.set_global(INSTANCING_LEVEL, 1u32);
        e.mem.set_u32(descriptor, 0);
        assert_eq!(fn_00b61980(&mut e, descriptor), 0);
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 5);
        // A built entry is answered as it is (its local reference released).
        let source = e.mem.alloc(0x38);
        serve_source(&mut e, source);
        e.mem.set_u32(descriptor, 0x77);
        assert_eq!(fn_00b61980(&mut e, descriptor), source);
        assert_eq!(e.mem.u32(source + 4), 2);
    }

    #[test]
    fn element_source_starts_a_background_loader_for_a_new_entry() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global(INSTANCING_LEVEL, 1u32);
        e.set_global(BACKGROUND_LOADING, 1u8);
        e.mem.set_u32(SECOND_MAP_GUARD, 1);
        e.register(ENTRY_LOOKUP, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(ENTRY_RELEASE, |_, _| Ret::default());
        // The map's slots: hash 0, no item found; the new item's fill.
        e.put_vtable(0x0090_d000, &[0, 0x0040_d004, 0x0040_d008, 0x0040_d00c]);
        e.mem.set_u32(SECOND_MAP, 0x0090_d000);
        e.mem.set_u32(SECOND_MAP + 4, 1);
        let buckets = e.mem.alloc(4);
        e.mem.set_u32(SECOND_MAP + 8, buckets);
        e.register(0x0040_d004, |_, _| Ret::default());
        e.register(0x0040_d008, |_, _| Ret::default());
        e.register(0x0040_d00c, |_, _| Ret::default());
        e.register(ALLOCATE_ITEM, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(LOADER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LOADER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0040_e048, |_, _| Ret::default());
        e.put_vtable(0x0090_e000, &[0; 0x4c / 4]);
        e.mem.set_u32(0x0090_e000 + 0x48, 0x0040_e048);
        let owner = e.mem.alloc(4);
        e.mem.set_u32(owner, 0x0090_e000);
        e.set_global(LOADER_OWNER, owner);
        let descriptor = e.mem.alloc(4);
        e.mem.set_u32(descriptor, 0x77);
        assert_eq!(fn_00b61980(&mut e, descriptor), 0);
        let made = calls_to(&e, POINTER_ASSIGN)[0][1];
        assert_eq!(e.mem.u32(made), VTABLE_010ADE9C);
        assert_eq!(e.mem.u8(made + 0x32), 1);
        let loader = calls_to(&e, LOADER_CONSTRUCT)[0].clone();
        assert_eq!(loader[1], 0x77);
        assert_eq!(e.mem.u32(made + 0x24), loader[0]);
        assert_eq!(calls_to(&e, 0x0040_e048), vec![vec![owner, loader[0]]]);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE)[0],
            vec![MEMORY_MANAGER, 0x600]
        );
    }

    #[test]
    fn element_source_waits_for_a_loader_with_too_little() {
        let mut e = rig();
        let m = model(&mut e, 2, false);
        let loader = e.mem.alloc(0x10);
        e.mem.set_u32(loader + 0xc, 3);
        e.mem.set_u32(m.entry + 0x24, loader);
        assert_eq!(fn_00b61980(&mut e, m.descriptor), 0);
        assert_eq!(calls_to(&e, ENTRY_RELEASE).len(), 1);
        assert_eq!(calls_to(&e, POINTER_RELEASE).len(), 1);
        assert!(calls_to(&e, STREAM_LOAD).is_empty());
    }

    #[test]
    fn element_source_instances_a_shape() {
        let mut e = rig();
        let m = model(&mut e, 2, false);
        e.mem.set_u32(m.data + 0x44, 3);
        let triangles = e.mem.alloc(6);
        for (i, v) in [0u16, 1, 1].iter().enumerate() {
            e.mem.set_u16(triangles + 2 * i as u32, *v);
        }
        e.register_double(TRI_LIST, move |_, _| triangles.into_ret());
        assert_eq!(fn_00b61980(&mut e, m.descriptor), m.entry);
        let entry = m.entry;
        assert_eq!(e.mem.u8(entry + 0x32), 0);
        assert_eq!(e.mem.u8(entry + 0x30), 0);
        assert_eq!(e.mem.u8(entry + 0x31), 1);
        assert_eq!(e.mem.u32(entry + 0x10), 2);
        assert_eq!(e.mem.u32(entry + 0x2c), 0x50);
        assert_eq!(e.mem.u16(entry + 0x34), 1);
        assert_eq!(e.mem.f32(entry + 0x28), 4.5);
        let shape = &calls_to(&e, SHAPE_DATA_CONSTRUCT)[0];
        assert_eq!(e.mem.u32(entry + 8), shape[0]);
        assert_eq!(shape[1], 0xa0);
        assert_eq!(&shape[6..9], &[1, 0, 0x50]);
        assert_eq!(shape[11], 0);
        // Instance 1: the vertices and normals again, numbered 1.0, indices from vertex 2.
        let (positions, normals, indices) = (shape[2], shape[3], shape[9]);
        assert_eq!(e.mem.u32(positions + 0x18), 0x2000);
        assert_eq!(e.mem.u32(normals + 0x1c), 0x2401);
        let numbers = e.mem.u32(entry + 0xc);
        assert_eq!(e.mem.f32(numbers + 8), 1.0);
        assert_eq!(e.mem.u16(indices + 6), 2);
        assert_eq!(e.mem.u16(indices + 0xa), 3);
        // Two new alpha properties (none on the geometry).
        let alpha = e.mem.u32(entry + 0x18);
        let second = e.mem.u32(entry + 0x1c);
        assert_eq!(e.mem.u32(alpha), ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(alpha + 0x18), 0x12ed);
        assert_eq!(e.mem.u16(second + 0x18), 0x12ec);
        assert_eq!(e.mem.u32(alpha + 4), 1);
        // The entry's local reference released: back to 2.
        assert_eq!(e.mem.u32(entry + 4), 2);
        assert_eq!(e.mem.u32(m.root), 0x0090_a000);
        assert_eq!(e.mem.u32(m.geometry), 0x0090_a100);
    }

    #[test]
    fn element_source_instances_strips_joined() {
        let mut e = rig();
        let m = model(&mut e, 3, true);
        let lengths = e.mem.alloc(2);
        e.mem.set_u16(lengths, 3);
        e.mem.set_u32(m.data + 0x48, lengths);
        let strip = e.mem.alloc(6);
        for (i, v) in [0u16, 1, 2].iter().enumerate() {
            e.mem.set_u16(strip + 2 * i as u32, *v);
        }
        e.register_double(STRIP_LISTS, move |_, _| strip.into_ret());
        assert_eq!(fn_00b61980(&mut e, m.descriptor), m.entry);
        let entry = m.entry;
        assert_eq!(e.mem.u8(entry + 0x30), 1);
        assert_eq!(e.mem.u16(entry + 0x34), 6);
        assert_eq!(e.mem.u32(entry + 0x2c), 0x50);
        let strips = &calls_to(&e, STRIPS_DATA_CONSTRUCT)[0];
        assert_eq!(strips[1], 0xf0);
        assert_eq!(strips[8], 80 * 3 + 79 * 3 - 2);
        assert_eq!(e.mem.u16(strips[10]), 80 * 3 + 79 * 3);
        let indices = strips[11];
        let first: Vec<u16> = (0..9).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(first, vec![0, 1, 2, 2, 3, 3, 3, 4, 5]);
        // The arrays copied per instance.
        assert_eq!(e.mem.u32(strips[2] + 0x24), 0x2000);
        assert_eq!(e.mem.u32(strips[4] + 0x30), 0x2800);
    }
}

#[cfg(test)]
mod n070_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const DELETE_SLOT: u32 = 0x0e30_0000;

    /// A reference-counted object (vtable slot +4 `DELETE_SLOT`) with count `count`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_SLOT);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        for target in [
            BASE_COPY_00B702B0,
            DELETE_SLOT,
            MAP_DESTRUCTOR,
            MEMORY_DEALLOCATE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00b60a50_copies_pointers_and_words() {
        let mut e = rig();
        let this = e.mem.alloc(0x98);
        let clone = e.mem.alloc(0x98);
        let shared = counted(&mut e, 2);
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 1);
        // +0x80: the same pointer on both sides (untouched); +0x84: old replaced by new.
        e.mem.set_u32(this + 0x80, shared);
        e.mem.set_u32(clone + 0x80, shared);
        e.mem.set_u32(this + 0x84, new);
        e.mem.set_u32(clone + 0x84, old);
        for (offset, word) in [(0x90u32, 9u32), (0x94, 10), (0x7c, 11)] {
            e.mem.set_u32(this + offset, word);
        }
        e.call(0x00b6_0a50, &args![this, clone, 1u32]);
        assert_eq!(calls(&e, BASE_COPY_00B702B0), vec![vec![this, clone, 1]]);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![old]]);
        assert_eq!(e.mem.u32(clone + 0x84), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.mem.u32(clone + 0x90), 9);
        assert_eq!(e.mem.u32(clone + 0x94), 10);
        assert_eq!(e.mem.u32(clone + 0x7c), 11);
    }

    #[test]
    fn fn_00b60a50_clears_to_a_null_pointer() {
        let mut e = rig();
        let this = e.mem.alloc(0x98);
        let clone = e.mem.alloc(0x98);
        let old = counted(&mut e, 3);
        e.mem.set_u32(clone + 0x80, old);
        e.call(0x00b6_0a50, &args![this, clone, 0u32]);
        assert_eq!(e.mem.u32(old + 4), 2);
        assert!(calls(&e, DELETE_SLOT).is_empty());
        assert_eq!(e.mem.u32(clone + 0x80), 0);
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn fn_00b60c70_frees_only_with_bit_0() {
        let mut e = rig();
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00b6_0c70, &args![map, 0x100u32]).u32(), map);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00b6_0c70, &args![map, 1u32]);
        assert_eq!(calls(&e, MAP_DESTRUCTOR), vec![vec![map], vec![map]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
    }

    #[test]
    fn fn_00b60d60_and_00b60d70() {
        let mut e = rig();
        let this = e.mem.alloc(0x98);
        assert_eq!(e.call(0x00b6_0d60, &args![this]).u32(), 0x011f_9f00);
        e.call(0x00b6_0d70, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0x55);
    }

    #[test]
    fn fn_00b60e80_clones_under_the_allocation_context() {
        let mut e = rig();
        e.register(LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        e.map(0x0e33_0000, 0x10);
        e.register(NI_NEW, |e, a| {
            let tls = e.tls();
            let context = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
            e.mem.set_u32(0x0e33_0000, context);
            e.mem.alloc(a[0]).into_ret()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 7);
        let this = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x94, 0x66);
        let clone = e.call(0x00b6_0e80, &args![this, 1u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE_010ADEA8);
        assert_eq!(e.mem.u32(clone + 0x94), 0x66);
        assert_eq!(calls(&e, BASE_COPY_00B702B0), vec![vec![this, clone, 1]]);
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 7);
        // The block was asked for under context 0x1c.
        assert_eq!(e.mem.u32(0x0e33_0000), 0x1c);
        assert_eq!(calls(&e, NI_NEW), vec![vec![0x98]]);
    }

    #[test]
    fn fn_00b60e80_copies_into_null_when_the_allocation_fails() {
        let mut e = rig();
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x98);
        e.register(BASE_COPY_00B702B0, |_, _| Ret::default());
        // The copy into null reads the clone's pointer at +0x80: map page 0.
        e.map(0, 0x1000);
        assert_eq!(e.call(0x00b6_0e80, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls(&e, BASE_COPY_00B702B0), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn fn_00b63070_calls_two_slots_and_passes_the_second_result() {
        let mut e = rig();
        e.register(0x0e31_0000, |_, _| Ret::default());
        e.register(0x0e32_0000, |_, _| Ret {
            eax: 0x1234,
            edx: 5,
            st0: 2.5,
        });
        let vtable = e.mem.alloc(0xb0);
        e.mem.set_u32(vtable + 0xa8, 0x0e31_0000);
        e.mem.set_u32(vtable + 0xac, 0x0e32_0000);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        let r = e.call(0x00b6_3070, &args![this]);
        assert_eq!((r.eax, r.edx, r.st0), (0x1234, 5, 2.5));
        assert_eq!(calls(&e, 0x0e31_0000), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0e32_0000), vec![vec![this]]);
    }
}
