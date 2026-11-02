//! `Havok SDK/hkdataobjectdict.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dc9a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_96a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dc9a20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_96a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dca190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_a0f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dca190(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_a0f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dca720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_a6b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dca720(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_a6b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcb960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_b890`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcb960(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_b890, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcbf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_8080`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcbf90(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_8080, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcc100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_bfc0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcc100(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_bfc0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// ---- Callees outside this file, called by address (first word is `this` for the methods). ----

/// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>` (hkstringmap.cpp, Xbox PDB names):
/// the constructor takes one stack word (an `hkBool`) it never reads.
pub const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
pub const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
pub const STRING_MAP_GET_ITERATOR: u32 = 0x00d8_7410;
pub const STRING_MAP_GET_VALUE: u32 = 0x00d8_7440;
pub const STRING_MAP_GET_NEXT: u32 = 0x00d8_7480;
pub const STRING_MAP_IS_VALID: u32 = 0x00d8_74b0;
pub const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
pub const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
pub const STRING_MAP_REMOVE: u32 = 0x00d8_7820;
/// `hkString::strDup` / `hkString::strFree` (cdecl).
pub const STRING_DUP: u32 = 0x00cb_1410;
pub const STRING_FREE: u32 = 0x00cb_13c0;
/// `hkMapBase<unsigned long,unsigned long>`: `getWithDefault(key, default)`, `findKey(key)`,
/// `remove(index)`, `clearAndDeallocate(allocator)`, and the unnamed map reset at `0x00cb_1a60`.
pub const POINTER_MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
pub const POINTER_MAP_FIND_KEY: u32 = 0x00cb_18f0;
pub const POINTER_MAP_REMOVE: u32 = 0x00cb_19b0;
pub const POINTER_MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
pub const POINTER_MAP_RESET: u32 = 0x00cb_1a60;
/// The one-byte function (`RET`) called on a map after `clearAndDeallocate` (unnamed in the map).
pub const POINTER_MAP_AFTER_CLEAR: u32 = 0x00a2_9680;
/// The multi-map constructor-like routine of this unit (`0x00dc_9e70`, unnamed in the map).
pub const MULTI_MAP_CONSTRUCT: u32 = 0x00dc_9e70;
/// `hkSerializeMultiMap::insert` (`0x00dc_9ec0`; the map names it after another instance).
pub const MULTI_MAP_INSERT: u32 = 0x00dc_9ec0;
/// `hkReferencedObject` reference counting in `hkreferencedobject.cpp` (unnamed in the map):
/// the object is in ECX, i.e. the first word.
pub const ADD_REFERENCE: u32 = 0x00c9_0510;
pub const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArrayUtil` (hkarrayutil.cpp): the growth step `(allocator, array, element size)` and
/// `_reserve(allocator, array, count, element size)`, both cdecl.
pub const ARRAY_GROW: u32 = 0x00c9_0940;
pub const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// C runtime `memcpy(dst, src, n)` and `memset(dst, value, n)` (cdecl).
pub const MEM_COPY: u32 = 0x00cb_1260;
pub const MEM_SET: u32 = 0x00ec_61c0;
/// `hkDataClass` (hkdataobject.cpp, Xbox PDB names); the first word is the `hkDataClass` handle
/// (a block whose first word is the class implementation).
pub const DATA_CLASS_CONSTRUCT: u32 = 0x00db_a650;
pub const DATA_CLASS_DESTRUCT: u32 = 0x00db_a660;
pub const DATA_CLASS_IS_NULL: u32 = 0x00db_a670;
pub const DATA_CLASS_GET_NAME: u32 = 0x00db_a680;
pub const DATA_CLASS_GET_VERSION: u32 = 0x00db_a690;
pub const DATA_CLASS_GET_PARENT: u32 = 0x00db_a6a0;
pub const DATA_CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00db_a6f0;
pub const DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO: u32 = 0x00db_af10;
/// `hkDataObject::hkDataObject(impl)`.
pub const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
/// `hkDataWorldDict::ObjectTracker` methods outside this part of the unit (called with the tracker first).
pub const TRACKER_RETRACT_CASTED_OBJECT: u32 = 0x00dc_ab50;
pub const TRACKER_TRACK_CLASS: u32 = 0x00dc_a820;
pub const TRACKER_FN_00DCAAF0: u32 = 0x00dc_aaf0;
/// `hkDataClassDict` constructor (`this`, world, name, version) and `hkDataClassDict::addMember`.
pub const DATA_CLASS_DICT_CONSTRUCT: u32 = 0x00dc_abb0;

/// Offsets inside the object tracker (`hkDataWorldDict + 8`, 0x88 bytes). The roles are what
/// the code in this unit does with them; the PDB has no layout for it.
pub mod tracker {
    /// `hkCachedHashMap` class name -> class implementation.
    pub const CLASS_MAP: u32 = 0x00;
    /// Multi-map, class -> its derived classes.
    pub const DERIVED_MAP: u32 = 0x10;
    /// Multi-map, class -> the objects of exactly that class.
    pub const OBJECT_MAP: u32 = 0x30;
    /// `hkCachedHashMap` of interned strings (`hkString::strDup` copies, key and value alike).
    pub const INTERNED_NAMES: u32 = 0x4c;
    /// `hkCachedHashMap` name -> small reference-counted record (`internClassNameHandle`).
    pub const CLASS_HANDLES: u32 = 0x5c;
    /// Multi-map, class -> the struct and object arrays of that class.
    pub const ARRAY_MAP: u32 = 0x6c;
}

/// Field offsets of a multi-map (`hkSerializeMultiMap`): an `hkArray` of 8-byte `{value, next}`
/// entries, an `hkMapBase` from key to the first entry, and the head of the free list.
pub mod multi_map {
    pub const ENTRIES: u32 = 0x00;
    pub const MAP: u32 = 0x0c;
    pub const FREE_HEAD: u32 = 0x18;
}

layout! {
    /// `ArrayOfTuplesImplementation::View` (Xbox PDB name; fields are roles read from the code):
    /// a window of one tuple of an array of tuples. It sits embedded at `+0xc` of its
    /// [`ArrayOfTuplesImplementation`], so its `+0xc` points back at that object.
    pub struct ArrayOfTuplesView: 0x14 {
        /// The owning array of tuples.
        0x0c tuples: Ptr<ArrayOfTuplesImplementation>,
        /// Index of the tuple's first element in the element array.
        0x10 first_index: i32,
    }
    /// `ArrayOfTuplesImplementation` (Xbox PDB name; fields are roles read from the code).
    pub struct ArrayOfTuplesImplementation: 0x28 {
        /// The embedded view.
        0x0c view: Inline<ArrayOfTuplesView>,
        /// The array implementation holding every element of every tuple.
        0x20 elements: Ptr,
        /// Elements per tuple.
        0x24 tuple_size: i32,
    }
    /// `hkDataWorldDict` (Xbox PDB name). Only the fields the translated code touches.
    pub struct HkDataWorldDict: 0x0c {
        /// Stored as 1 by the constructor (role unconfirmed).
        0x06 kind: u16,
        /// The object tracker.
        0x08 tracker: Ptr<ObjectTracker>,
    }
    /// The object tracker (`hkDataWorldDict::ObjectTracker`); see [`tracker`].
    pub struct ObjectTracker: 0x88 {
        /// The first object made by `newObject` (what `getContents` hands out).
        0x2c contents: Ptr,
    }
    /// `hkDataClassDict` (Xbox PDB name), 0x40 bytes; fields are roles read from the code.
    pub struct HkDataClassDict: 0x40 {
        0x08 reference_count: i32,
        /// The world the class belongs to (what `ObjectTracker::intern` reads the tracker from).
        0x0c world: Ptr<HkDataWorldDict>,
        /// Written by `setClassVersion`.
        0x14 version: i32,
        /// The parent class (reference counted).
        0x18 parent: Ptr<HkDataClassDict>,
        /// Array of 0x18-byte member records: `{name, class handle, _, _, type, extra}`.
        0x28 members: Ptr,
        0x2c member_count: i32,
    }
    /// `hkDataObjectDict` (Xbox PDB name), 0x1c bytes; fields are roles read from the code.
    pub struct HkDataObjectDict: 0x1c {
        /// 16-bit size of the block, kept for the memory router.
        0x04 memory_size: u16,
        /// Count of references from outside the world.
        0x06 external_count: i16,
        0x08 reference_count: i32,
        0x0c class: Ptr<HkDataClassDict>,
        /// `hkArray` of 0x10-byte value records: `{name, _, value low, value high}`.
        0x10 members: Ptr,
        0x14 member_count: i32,
        0x18 capacity_and_flags: u32,
    }
}

/// The allocator of the memory router (`router + 0x10`).
pub fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The inline `hkArray` destructor: size 0, the buffer returned through the router's allocator
/// (slot `0x10`) unless bit 31 of the capacity word says the array does not own it, then data 0
/// and capacity word `0x8000_0000`. `element_size` is the exe's per-element byte multiplier.
pub fn array_free(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// An empty `hkArray` that owns nothing: data 0, size 0, capacity word `0x8000_0000`.
pub fn array_init_empty(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// First half of `hkArray::pushBack`: when `size` equals the capacity the array grows through
/// `hkArrayUtil` with the router's allocator.
pub fn array_grow_if_full(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, array, element_size]);
    }
}

/// `hkArray<void*>::pushBack` as the exe inlines it.
pub fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    array_grow_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// `hkArray::reserve` growth as the exe inlines it: when the capacity (without its flag bits)
/// is below `wanted`, `hkArrayUtil::_reserve` gets the larger of `wanted` and twice the
/// capacity.
pub fn array_reserve(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let new_capacity = capacity.wrapping_add(capacity).max(wanted);
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// Drops one reference of the counted object at `object` (field `+8`) and runs its deleting
/// destructor (slot `0`, argument `1`) when the count reaches zero.
pub fn release_object(e: &mut Engine, object: u32) {
    let count = e.mem.u32(object + 8).wrapping_sub(1);
    e.mem.set_u32(object + 8, count);
    if count == 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

/// [`release_object`] for a pointer that may be null.
pub fn release_object_if_set(e: &mut Engine, object: u32) {
    if object != 0 {
        release_object(e, object);
    }
}

/// `INC [object+8]`.
pub fn add_object_reference(e: &mut Engine, object: u32) {
    let count = e.mem.u32(object + 8).wrapping_add(1);
    e.mem.set_u32(object + 8, count);
}

/// The inlined string interning: the copy of `name` kept in `map` (the tracker's
/// [`tracker::INTERNED_NAMES`]), made with `hkString::strDup` and inserted when missing.
/// `name` must not be null.
pub fn intern_string(e: &mut Engine, map: u32, name: u32) -> u32 {
    let mut interned = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![map, name, 0u32])
        .u32();
    if interned == 0 {
        interned = e.call(STRING_DUP, &args![name]).u32();
        e.call(STRING_MAP_INSERT, &args![map, interned, interned]);
    }
    interned
}

/// [`intern_string`] through a tracker, with the null name staying null.
pub fn intern_name(e: &mut Engine, tracker_address: u32, name: u32) -> u32 {
    if name == 0 {
        0
    } else {
        intern_string(e, tracker_address + tracker::INTERNED_NAMES, name)
    }
}

/// `hkCachedHashMap::isValid(&out, iterator)`: the flag byte it stores.
pub fn string_map_is_valid(e: &mut Engine, map: u32, iterator: u32) -> bool {
    e.with_stack(4, |e, flag| {
        e.call(STRING_MAP_IS_VALID, &args![map, flag.addr(), iterator]);
        e.mem.u8(flag.addr()) != 0
    })
}

/// The iteration loop the exe inlines: `getIterator`, then while `isValid` calls `body` on the
/// entry's `getValue` and moves on with `getNext`.
pub fn string_map_for_each(e: &mut Engine, map: u32, mut body: impl FnMut(&mut Engine, u32)) {
    let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![map]).u32();
    while string_map_is_valid(e, map, iterator) {
        let value = e.call(STRING_MAP_GET_VALUE, &args![map, iterator]).u32();
        body(e, value);
        iterator = e.call(STRING_MAP_GET_NEXT, &args![map, iterator]).u32();
    }
}

/// The array implementation behind a tuple view (`view.tuples.elements`).
fn view_elements(e: &Engine, this: Ptr<ArrayOfTuplesView>) -> u32 {
    let tuples = e.get(this, ArrayOfTuplesView::tuples);
    e.get(tuples, ArrayOfTuplesImplementation::elements).addr()
}

/// The element index `index` of a tuple view in the element array.
fn view_element_index(e: &Engine, this: Ptr<ArrayOfTuplesView>, index: i32) -> i32 {
    e.get(this, ArrayOfTuplesView::first_index)
        .wrapping_add(index)
}

// Translated from 00dc6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::getType` (Xbox PDB): the type of the owning array of
/// tuples (virtual slot `4` of the object at `this + 0xc`) with the array bit `0x20` toggled.
pub fn array_of_tuples_view_get_type(e: &mut Engine, this: Ptr<ArrayOfTuplesView>) -> u32 {
    let tuples = e.get(this, ArrayOfTuplesView::tuples);
    e.vcall(tuples.addr(), 4, &args![]).u32() ^ 0x20
}

// Translated from 00dc6fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::clear` (Xbox PDB): tail-calls slot `8` of the element
/// array.
pub fn array_of_tuples_view_clear(e: &mut Engine, this: Ptr<ArrayOfTuplesView>) {
    let elements = view_elements(e, this);
    e.vcall(elements, 8, &args![]);
}

// Translated from 00dc6fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::getSize` (Xbox PDB): the number of elements per tuple.
pub fn array_of_tuples_view_get_size(e: &mut Engine, this: Ptr<ArrayOfTuplesView>) -> i32 {
    let tuples = e.get(this, ArrayOfTuplesView::tuples);
    e.get(tuples, ArrayOfTuplesImplementation::tuple_size)
}

// Translated from 00dc6ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setAll` (Xbox PDB): stores `count` bytes from `values`
/// one by one through slot `0x44` of the element array (`setInt`), at the tuple's indices.
pub fn array_of_tuples_view_set_all(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    values: u32,
    count: i32,
) {
    let mut i = 0;
    while i < count {
        let elements = view_elements(e, this);
        let value = u32::from(e.mem.u8(values.wrapping_add(i as u32)));
        let index = view_element_index(e, this, i);
        e.vcall(elements, 0x44, &args![index, value]);
        i += 1;
    }
}

// Translated from 00dc7030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setAll_ov2` (Xbox PDB): stores `count` floats from
/// `values` one by one through slot `0x34` of the element array (`setReal`); each goes
/// through an `FLD`/`FSTP` pair.
pub fn array_of_tuples_view_set_all_ov2(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    values: u32,
    count: i32,
) {
    let mut i = 0;
    while i < count {
        let elements = view_elements(e, this);
        let bits = x87_quiet_bits(e.mem.u32(values.wrapping_add((i as u32).wrapping_mul(4))));
        let index = view_element_index(e, this, i);
        e.vcall(elements, 0x34, &args![index, bits]);
        i += 1;
    }
}

// Translated from 00dc7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asVec` (Xbox PDB): tail-calls slot `0x20` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_vec(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x20, &args![index]).u32()
}

// Translated from 00dc7090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setVec` (Xbox PDB): slot `0x24` of the element array
/// (`setVec`) with the element index and `value`.
pub fn array_of_tuples_view_set_vec(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x24, &args![index, value]);
}

// Translated from 00dc70c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asString` (Xbox PDB): tail-calls slot `0x28` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_string(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x28, &args![index]).u32()
}

// Translated from 00dc70e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setString` (Xbox PDB): slot `0x2c` of the element
/// array (`setString`) with the element index and `value`.
pub fn array_of_tuples_view_set_string(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x2c, &args![index, value]);
}

// Translated from 00dc7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asReal` (Xbox PDB): tail-calls slot `0x30` of the
/// element array with the element index. The callee's `ST0` result is passed through
/// unchanged, hence the `f64` result.
pub fn array_of_tuples_view_as_real(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> f64 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x30, &args![index]).f64()
}

// Translated from 00dc7130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setReal` (Xbox PDB): slot `0x34` of the element array
/// (`setReal`) with the element index and the float, which goes through an `FLD`/`FSTP` pair.
pub fn array_of_tuples_view_set_real(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: f32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x34, &args![index, x87_quiet(value)]);
}

// Translated from 00dc7160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asInt` (Xbox PDB): tail-calls slot `0x40` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_int(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x40, &args![index]).u32()
}

// Translated from 00dc7180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setInt` (Xbox PDB): slot `0x44` of the element array
/// (`setInt`) with the element index and `value`.
pub fn array_of_tuples_view_set_int(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x44, &args![index, value]);
}

// Translated from 00dc71b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asInt64` (Xbox PDB): tail-calls slot `0x48` of the
/// element array with the element index; its `EDX:EAX` result is passed through.
pub fn array_of_tuples_view_as_int64(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u64 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x48, &args![index]).u64()
}

// Translated from 00dc71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setInt64` (Xbox PDB): slot `0x4c` of the element
/// array (`setInt64`) with the element index and the value as two words (low, high).
pub fn array_of_tuples_view_set_int64(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    low: u32,
    high: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x4c, &args![index, low, high]);
}

// Translated from 00dc7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asObject` (Xbox PDB): tail-calls slot `0x50` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_object(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x50, &args![index]).u32()
}

// Translated from 00dc7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setObject` (Xbox PDB): slot `0x54` of the element
/// array (`setObject`) with the element index and `value`.
pub fn array_of_tuples_view_set_object(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x54, &args![index, value]);
}

// Translated from 00dc7250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asArray` (Xbox PDB): tail-calls slot `0x58` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x58, &args![index]).u32()
}

// Translated from 00dc7270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setArray` (Xbox PDB): slot `0x5c` of the element array
/// (`setArray`) with the element index and `value`.
pub fn array_of_tuples_view_set_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x5c, &args![index, value]);
}

// Translated from 00dc89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::getType` (Xbox PDB): the type of the element array (slot `4`
/// of `this + 0x20`) with the array bit `0x20` set.
pub fn array_of_tuples_implementation_get_type(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) -> u32 {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.vcall(elements, 4, &args![]).u32() | 0x20
}

// Translated from 00dc89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::getClass` (Xbox PDB): tail-calls slot `0x18` of the element
/// array; its result is passed through.
pub fn array_of_tuples_implementation_get_class(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) -> u32 {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.vcall(elements, 0x18, &args![]).u32()
}

// Translated from 00dc89f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::clear` (Xbox PDB): tail-calls slot `8` of the element array.
pub fn array_of_tuples_implementation_clear(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.vcall(elements, 8, &args![]);
}

// Translated from 00dc8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::reserve` (Xbox PDB): tail-calls slot `0xc` of the element
/// array with `count * tuple_size` elements.
pub fn array_of_tuples_implementation_reserve(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    count: i32,
) {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.vcall(elements, 0xc, &args![tuple_size.wrapping_mul(count)]);
}

// Translated from 00dc8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::setSize` (Xbox PDB): tail-calls slot `0x10` of the element
/// array with `count * tuple_size` elements.
pub fn array_of_tuples_implementation_set_size(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    count: i32,
) {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.vcall(elements, 0x10, &args![tuple_size.wrapping_mul(count)]);
}

// Translated from 00dc8a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::getSize` (Xbox PDB): the size of the element array (slot
/// `0x14`) divided, signed, by the tuple size. A zero divisor (or `i32::MIN / -1`) is the
/// divide error the `IDIV` raises.
pub fn array_of_tuples_implementation_get_size(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) -> i32 {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    let size = e.vcall(elements, 0x14, &args![]).i32();
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    match size.checked_div(tuple_size) {
        Some(count) => count,
        None => panic!("00dc8a40: IDIV faults (divide error)"),
    }
}

// Translated from 00dc8a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::asArray` (Xbox PDB): points the embedded view at tuple
/// `index` (`+0x1c` = `tuple_size * index`) and returns the view (`this + 0xc`).
pub fn array_of_tuples_implementation_as_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    index: i32,
) -> u32 {
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.mem
        .set_i32(this.addr() + 0x1c, tuple_size.wrapping_mul(index));
    this.addr() + 0xc
}

// Translated from 00dc8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::setArray` (Xbox PDB): points the embedded view at tuple
/// `index`, asks the array `source` for its type (slot `4`) and copies its first `tuple_size`
/// elements into the view, one by one, with the getter and the view's setter of that type:
/// 1 int (`0x40` / `0x44`), 2 int64 (`0x48` / `0x4c`, two words), 3 real (`0x30` / `0x34`, the
/// `ST0` result stored as a float), 4 to 7 vector (`0x20` / `0x24`), 8 and 9 object (`0x50` /
/// `0x54`), 10 string (`0x28` / `0x2c`). Any other type copies nothing.
pub fn array_of_tuples_implementation_set_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    index: i32,
    source: u32,
) {
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.mem
        .set_i32(this.addr() + 0x1c, tuple_size.wrapping_mul(index));
    let kind = e.vcall(source, 4, &args![]).u32().wrapping_sub(1);
    if kind > 9 {
        return;
    }
    let view = this.addr() + 0xc;
    let mut element = 0i32;
    while element < e.get(this, ArrayOfTuplesImplementation::tuple_size) {
        match kind {
            0 => {
                let value = e.vcall(source, 0x40, &args![element]).u32();
                e.vcall(view, 0x44, &args![element, value]);
            }
            1 => {
                let value = e.vcall(source, 0x48, &args![element]).u64();
                e.vcall(
                    view,
                    0x4c,
                    &args![element, value as u32, (value >> 32) as u32],
                );
            }
            2 => {
                let value = e.vcall(source, 0x30, &args![element]).f64();
                e.vcall(view, 0x34, &args![element, (value as f32).to_bits()]);
            }
            3..=6 => {
                let value = e.vcall(source, 0x20, &args![element]).u32();
                e.vcall(view, 0x24, &args![element, value]);
            }
            7 | 8 => {
                let value = e.vcall(source, 0x50, &args![element]).u32();
                e.vcall(view, 0x54, &args![element, value]);
            }
            _ => {
                let value = e.vcall(source, 0x28, &args![element]).u32();
                e.vcall(view, 0x2c, &args![element, value]);
            }
        }
        element += 1;
    }
}

// Translated from 00dc9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::isValid` (Xbox PDB): whether `index` (signed) is below the member count.
pub fn hk_data_object_dict_is_valid(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    index: i32,
) -> bool {
    index < e.get(this, HkDataObjectDict::member_count)
}

// Translated from 00dc9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::getMemberName` (Xbox PDB): the name word of the value record `index`
/// (16 bytes each).
pub fn hk_data_object_dict_get_member_name(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    index: i32,
) -> u32 {
    let records = e.get(this, HkDataObjectDict::members).addr();
    e.mem.u32(records.wrapping_add((index as u32) << 4))
}

// Translated from 00dc9550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::getMemberValue` (Xbox PDB): stores `this` and the name word of the value
/// record `index` at `out` and `out + 4`. `EAX` still holds `out` on return.
pub fn hk_data_object_dict_get_member_value(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    out: u32,
    index: i32,
) -> u32 {
    e.mem.set_u32(out, this.addr());
    let records = e.get(this, HkDataObjectDict::members).addr();
    let name = e.mem.u32(records.wrapping_add((index as u32) << 4));
    e.mem.set_u32(out + 4, name);
    out
}

// Translated from 00dc9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `VecArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_6f4c` belongs to it): `+6` 0, reference count `+8` 0, vtable `0x010d_6f4c`, an empty
/// element array (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`) and the
/// stride `stride` at `+0x18`. Returns `this`.
pub fn vec_array_implementation_construct(e: &mut Engine, this: u32, stride: i32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this, 0x010d_6f4c);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_i32(this + 0x18, stride);
    this
}

// Translated from 00dc9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::getType` (Xbox PDB): the type code for the stride at `+0x18`: 4
/// gives 4, 0xc gives 6, 0x10 gives 7 and anything else gives 0.
pub fn vec_array_implementation_get_type(e: &mut Engine, this: u32) -> u32 {
    match e.mem.u32(this + 0x18) {
        4 => 4,
        0xc => 6,
        0x10 => 7,
        _ => 0,
    }
}

// Translated from 00dc9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::asVec` (Xbox PDB): the address of the vector `index`: the data
/// pointer plus `stride * index * 4` bytes (a signed 32-bit product).
pub fn vec_array_implementation_as_vec(e: &mut Engine, this: u32, index: i32) -> u32 {
    let stride = e.mem.i32(this + 0x18);
    let data = e.mem.u32(this + 0xc);
    data.wrapping_add((stride.wrapping_mul(index) as u32).wrapping_mul(4))
}

// Translated from 00dc9ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::setVec` (Xbox PDB): copies the `stride` floats at `values` to the
/// elements `stride * index + n` of the data, one at a time through an `FLD`/`FSTP` pair.
pub fn vec_array_implementation_set_vec(e: &mut Engine, this: u32, index: i32, values: u32) {
    let mut stride = e.mem.i32(this + 0x18);
    let mut n = 0i32;
    if stride > 0 {
        loop {
            let bits = x87_quiet_bits(e.mem.u32(values.wrapping_add((n as u32).wrapping_mul(4))));
            let data = e.mem.u32(this + 0xc);
            let position = stride.wrapping_mul(index).wrapping_add(n);
            n += 1;
            e.mem
                .set_u32(data.wrapping_add((position as u32).wrapping_mul(4)), bits);
            stride = e.mem.i32(this + 0x18);
            if n >= stride {
                break;
            }
        }
    }
}

// Translated from 00dc9b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::getSize` (Xbox PDB): the element count (`+0x10`) divided, signed,
/// by the stride (`+0x18`). A zero divisor (or `i32::MIN / -1`) is the divide error `IDIV`
/// raises.
pub fn vec_array_implementation_get_size(e: &mut Engine, this: u32) -> i32 {
    let count = e.mem.i32(this + 0x10);
    let stride = e.mem.i32(this + 0x18);
    match count.checked_div(stride) {
        Some(size) => size,
        None => panic!("00dc9b00: IDIV faults (divide error)"),
    }
}

// Translated from 00dc9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::clear` (Xbox PDB): sets the element count (`+0x10`) to 0.
pub fn vec_array_implementation_clear(e: &mut Engine, this: u32) {
    e.mem.set_u32(this + 0x10, 0);
}

// Translated from 00dc9b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::reserve` (Xbox PDB): reserves `stride * count` elements (4 bytes
/// each) of the element array at `+0xc`.
pub fn vec_array_implementation_reserve(e: &mut Engine, this: u32, count: i32) {
    let wanted = e.mem.i32(this + 0x18).wrapping_mul(count);
    array_reserve(e, this + 0xc, wanted, 4);
}

// Translated from 00dc9bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed function (the map gives it only a range) that answers 8 and reads nothing.
pub fn fn_00dc9bc0(_e: &mut Engine) -> u32 {
    8
}

// Translated from 00dc9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<unsigned char,1>::reserve` (Xbox PDB): reserves `count` bytes of
/// the element array at `+0xc`.
pub fn basic_array_implementation_u8_reserve(e: &mut Engine, this: u32, count: i32) {
    array_reserve(e, this + 0xc, count, 1);
}

// Translated from 00dc9d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<__int64,2>::reserve` (Xbox PDB): reserves `count` 8-byte elements
/// of the element array at `+0xc`.
pub fn basic_array_implementation_i64_reserve(e: &mut Engine, this: u32, count: i32) {
    array_reserve(e, this + 0xc, count, 8);
}

// Translated from 00dc9e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<hkDataObjectImpl*,8>::reserve` (Xbox PDB): reserves `count`
/// 4-byte elements of the element array at `+0xc`.
pub fn basic_array_implementation_object_reserve(e: &mut Engine, this: u32, count: i32) {
    array_reserve(e, this + 0xc, count, 4);
}

// Translated from 00dc9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::_scalar_deleting_destructor_` (Xbox PDB): frees the element array
/// at `+0xc` (4-byte elements, unless it does not own its buffer), sets the vtable to
/// `0x010d_62f4` and, when bit 0 of `flags` is set, returns the block to the router's
/// allocator (slot `8`) with the 16-bit size kept at `+4`. Returns `this`.
pub fn vec_array_implementation_deleting_destructor(e: &mut Engine, this: u32, flags: u8) -> u32 {
    array_free(e, this + 0xc, 4);
    e.mem.set_u32(this, 0x010d_62f4);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(this + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dca040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `PointerArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_6fe4` belongs to it): `+6` 0, reference count `+8` 0, an empty element array
/// (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`), vtable
/// `0x010d_6fe4`, and the element class `class` at `+0x18` (one more reference when not
/// null). Returns `this`.
pub fn pointer_array_implementation_construct(e: &mut Engine, this: u32, class: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_6fe4);
    if class != 0 {
        add_object_reference(e, class);
    }
    e.mem.set_u32(this + 0x18, class);
    this
}

// Translated from 00dca080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PointerArrayImplementation::asObject` (Xbox PDB): the pointer at element `index`.
pub fn pointer_array_implementation_as_object(e: &mut Engine, this: u32, index: i32) -> u32 {
    let data = e.mem.u32(this + 0xc);
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00dca090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PointerArrayImplementation::setObject` (Xbox PDB): the new object gains a reference (when
/// not null), the old element loses one (its deleting destructor runs at zero) and the element
/// `index` becomes the new object.
pub fn pointer_array_implementation_set_object(e: &mut Engine, this: u32, index: i32, object: u32) {
    if object != 0 {
        add_object_reference(e, object);
    }
    let data = e.mem.u32(this + 0xc);
    let old = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
    release_object_if_set(e, old);
    let data = e.mem.u32(this + 0xc);
    e.mem
        .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), object);
}

// Translated from 00dca0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `PointerArrayImplementation` (the map gives it only a range; the deleting
/// destructor `0x00dc_a190` runs it): sets the vtable to `0x010d_6fe4`, lets every non-null
/// element lose a reference, releases the element class at `+0x18` (cleared), frees the element
/// array (4-byte elements, unless it does not own its buffer) and sets the vtable to
/// `0x010d_62f4`.
pub fn fn_00dca0f0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010d_6fe4);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0x10) {
        let data = e.mem.u32(this + 0xc);
        let element = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
        release_object_if_set(e, element);
        index += 1;
    }
    let class = e.mem.u32(this + 0x18);
    release_object_if_set(e, class);
    e.mem.set_u32(this + 0x18, 0);
    array_free(e, this + 0xc, 4);
    e.mem.set_u32(this, 0x010d_62f4);
}

// Translated from 00dca1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ByteArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_707c` belongs to it): `+6` 0, reference count `+8` 0, an empty element array
/// (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`) and the vtable
/// `0x010d_707c`. Returns `this`.
pub fn byte_array_implementation_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_707c);
    this
}

// Translated from 00dca1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::asInt` (Xbox PDB): the byte at `index`, zero-extended.
pub fn byte_array_implementation_as_int(e: &mut Engine, this: u32, index: i32) -> u32 {
    let data = e.mem.u32(this + 0xc);
    u32::from(e.mem.u8(data.wrapping_add(index as u32)))
}

// Translated from 00dca1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::setInt` (Xbox PDB): stores the low byte of `value` at `index`.
pub fn byte_array_implementation_set_int(e: &mut Engine, this: u32, index: i32, value: u8) {
    let data = e.mem.u32(this + 0xc);
    e.mem.set_u8(data.wrapping_add(index as u32), value);
}

// Translated from 00dca210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::setAll` (Xbox PDB): reserves `count` bytes, stores `count` as the
/// size and copies `count` bytes from `source` with `memcpy`.
pub fn byte_array_implementation_set_all(e: &mut Engine, this: u32, source: u32, count: i32) {
    let array = this + 0xc;
    array_reserve(e, array, count, 1);
    e.mem.set_i32(array + 4, count);
    let data = e.mem.u32(array);
    e.call(MEM_COPY, &args![data, source, count]);
}

// Translated from 00dca260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `IntArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_7114` belongs to it): `+6` 0, reference count `+8` 0, an empty element array
/// (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`) and the vtable
/// `0x010d_7114`. Returns `this`.
pub fn int_array_implementation_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_7114);
    this
}

// Translated from 00dca280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::asInt` (Xbox PDB): the low word of the 8-byte element `index`.
pub fn int_array_implementation_as_int(e: &mut Engine, this: u32, index: i32) -> u32 {
    let data = e.mem.u32(this + 0xc);
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8)))
}

// Translated from 00dca290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setInt` (Xbox PDB): stores `value` as a sign-extended 64-bit value
/// in the element `index`.
pub fn int_array_implementation_set_int(e: &mut Engine, this: u32, index: i32, value: i32) {
    let data = e.mem.u32(this + 0xc);
    let element = data.wrapping_add((index as u32).wrapping_mul(8));
    e.mem.set_i32(element, value);
    e.mem.set_i32(element + 4, value >> 31);
}

// Translated from 00dca2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::asInt64` (Xbox PDB): the 8-byte element `index` (`EDX:EAX`).
pub fn int_array_implementation_as_int64(e: &mut Engine, this: u32, index: i32) -> u64 {
    let data = e.mem.u32(this + 0xc);
    let element = data.wrapping_add((index as u32).wrapping_mul(8));
    let low = e.mem.u32(element);
    let high = e.mem.u32(element + 4);
    u64::from(low) | (u64::from(high) << 32)
}

// Translated from 00dca2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setInt64` (Xbox PDB): stores the two words `low` and `high` in the
/// element `index`.
pub fn int_array_implementation_set_int64(
    e: &mut Engine,
    this: u32,
    index: i32,
    low: u32,
    high: u32,
) {
    let data = e.mem.u32(this + 0xc);
    let element = data.wrapping_add((index as u32).wrapping_mul(8));
    e.mem.set_u32(element, low);
    e.mem.set_u32(element + 4, high);
}

/// The shared body of the three `IntArrayImplementation::setAll` variants: reserves `count`
/// 8-byte elements, stores `count` as the size and, for each byte of `source`, stores the
/// `(low, high)` words `convert` makes of it in the element of that index.
fn int_array_set_all_with(
    e: &mut Engine,
    this: u32,
    source: u32,
    count: i32,
    convert: impl Fn(u8) -> (u32, u32),
) {
    let array = this + 0xc;
    array_reserve(e, array, count, 8);
    e.mem.set_i32(array + 4, count);
    let mut index = 0i32;
    while index < count {
        let byte = e.mem.u8(source.wrapping_add(index as u32));
        let (low, high) = convert(byte);
        let data = e.mem.u32(array);
        let element = data.wrapping_add((index as u32).wrapping_mul(8));
        e.mem.set_u32(element, low);
        e.mem.set_u32(element + 4, high);
        index += 1;
    }
}

// Translated from 00dca2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll` (Xbox PDB): sets the array to the `count` bytes at `source`
/// read as booleans (non-zero is 1, zero is 0), as 64-bit values.
pub fn int_array_implementation_set_all(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_set_all_with(e, this, source, count, |byte| (u32::from(byte != 0), 0));
}

// Translated from 00dca360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov2` (Xbox PDB): sets the array to the `count` bytes at
/// `source` read as signed, sign-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov2(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_set_all_with(e, this, source, count, |byte| {
        let value = i32::from(byte as i8);
        (value as u32, (value >> 31) as u32)
    });
}

// Translated from 00dca3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov3` (Xbox PDB): sets the array to the `count` bytes at
/// `source` read as unsigned, zero-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov3(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_set_all_with(e, this, source, count, |byte| (u32::from(byte), 0));
}

// ---- Callees outside this file added for `hkDataWorldDict` and the object tracker. ----

/// `hkOstream` (hkostream.cpp): the constructor `(stream, buffer, size)` (unnamed in the map),
/// `operator<<` for a string (unnamed) and for an int (`operator_shl_ov3`), each taking the stream
/// and returning it, and the destructor (unnamed).
pub const OSTREAM_CONSTRUCT: u32 = 0x00cb_1600;
pub const OSTREAM_WRITE_STRING: u32 = 0x00cc_01c0;
pub const OSTREAM_WRITE_INT: u32 = 0x00cc_0280;
pub const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global that holds the object the tracker's warnings go to; its virtual slot `0xc` takes
/// `(severity, id, text, file, line)`.
pub const ERROR_REPORTER: u32 = 0x0126_8360;
/// The fixed texts of that warning: the start `"The object of class "`, the text after the class
/// name, the text after the count (`") from outside the world and ..."`) and the source file name
/// (`".\Data\Dict\hkDataObjectDict.cpp"`).
pub const TEXT_THE_OBJECT_OF_CLASS: u32 = 0x010d_72f8;
pub const TEXT_ABOUT_TO_BE_REMOVED: u32 = 0x010d_7310;
pub const TEXT_FROM_OUTSIDE_THE_WORLD: u32 = 0x010d_7378;
pub const TEXT_SOURCE_FILE: u32 = 0x010d_72d4;
/// The text `"typeName"`: the member name `wrapClass` reads through `hkClassMemberAccessor`.
pub const TEXT_TYPE_NAME: u32 = 0x010d_4790;
/// A global that holds a pointer to the text `"hk.DataObjectType"`: the attribute name `wrapClass`
/// looks for.
pub const DATA_OBJECT_TYPE_NAME: u32 = 0x011b_7bfc;
/// A static object of the kind `internClassNameHandle` makes (vtable `0x010d_6da0`): the default
/// class handle, which code takes a reference of before it knows the real one.
pub const DEFAULT_CLASS_HANDLE: u32 = 0x011b_7c00;
/// `hkClass` and `hkClassMember` accessors (hkclass.cpp, hkclassmember.cpp), `this` first.
pub const HK_CLASS_GET_DESCRIBED_VERSION: u32 = 0x00d4_7120;
pub const HK_CLASS_GET_NAME: u32 = 0x00d4_6d40;
pub const HK_CLASS_GET_PARENT: u32 = 0x00d4_6d50;
pub const HK_CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00d4_6f70;
pub const HK_CLASS_GET_DECLARED_MEMBER: u32 = 0x00d4_6f80;
pub const HK_CLASS_GET_ATTRIBUTE: u32 = 0x00d4_7260;
pub const HK_CLASS_MEMBER_GET_CSTYLE_ARRAY_SIZE: u32 = 0x00d8_6940;
pub const HK_CLASS_MEMBER_GET_CLASS: u32 = 0x00d8_6920;
pub const HK_CLASS_MEMBER_GET_ATTRIBUTE: u32 = 0x00d8_6960;
/// `hkDataObjectUtil::getTypeFromMemberType(type, subtype, class, array size)` (cdecl).
pub const TYPE_FROM_MEMBER_TYPE: u32 = 0x00dc_1c50;
/// `hkClassMemberAccessor::hkClassMemberAccessor_ov2(variant, name)` and
/// `hkClassMemberAccessor::asCstring(index)`.
pub const CLASS_MEMBER_ACCESSOR_CONSTRUCT: u32 = 0x00d8_71a0;
pub const CLASS_MEMBER_ACCESSOR_AS_CSTRING: u32 = 0x00d8_7220;
/// `hkVersionPatchManager::UidFromClassVersion::cache(this = tracker, out, name)`: interns `name`
/// in the tracker's string map and stores the copy at `out`.
pub const UID_FROM_CLASS_VERSION_CACHE: u32 = 0x00dc_8d20;
/// The function `copyClassFromWorld` runs on its class description before it leaves early
/// (unnamed in the map, hkversionpatchmanager.cpp).
pub const DATA_CLASS_CINFO_DESTRUCT: u32 = 0x00db_a490;
/// The string map's lookup by key (unnamed in the map): the slot index of the key, `hashMod + 1`
/// when the key is missing; `isValid` tells which.
pub const STRING_MAP_FIND_ITERATOR: u32 = 0x00d8_7580;
/// `hkDataWorldDict`'s object `this + 8` (the tracker) as the exe reads it.
fn world_tracker(e: &Engine, this: Ptr<HkDataWorldDict>) -> u32 {
    e.get(this, HkDataWorldDict::tracker).addr()
}

/// Runs `body` with two empty `hkArray`s of the translated code's locals (12 bytes each).
fn with_two_arrays<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, u32, u32) -> R) -> R {
    e.with_stack(0x18, |e, block| {
        let first = block.addr();
        let second = first + 0xc;
        body(e, first, second)
    })
}
/// `hkDataObject::Value` accessors (hkdataobject.cpp, Xbox PDB names; the first word is the
/// `Value`), `hkDataArray::~hkDataArray` and `hkDataObject::~hkDataObject` (the first word is the
/// local being destroyed).
pub const DATA_OBJECT_VALUE_AS_INT: u32 = 0x00db_a990;
pub const DATA_OBJECT_VALUE_AS_VEC: u32 = 0x00db_a9a0;
pub const DATA_OBJECT_VALUE_AS_STRING: u32 = 0x00db_a9c0;
pub const DATA_OBJECT_VALUE_AS_OBJECT: u32 = 0x00db_a9d0;
pub const DATA_OBJECT_VALUE_AS_REAL: u32 = 0x00db_aa00;
pub const DATA_OBJECT_VALUE_AS_ARRAY: u32 = 0x00db_afb0;
pub const DATA_ARRAY_DESTRUCT: u32 = 0x00db_ab70;
pub const DATA_OBJECT_DESTRUCT: u32 = 0x00db_a7f0;

// Translated from 00dc72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::getContents` (Xbox PDB): constructs the `hkDataObject` at `out` over the
/// tracker's first object (`tracker + 0x2c`) and returns `out`.
pub fn hk_data_world_dict_get_contents(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    out: u32,
) -> u32 {
    let tracker_ptr = e.get(this, HkDataWorldDict::tracker);
    let contents = e.get(tracker_ptr, ObjectTracker::contents);
    e.call(DATA_OBJECT_CONSTRUCT, &args![out, contents]);
    out
}

// Translated from 00dc72c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::findObject` (Xbox PDB): constructs the `hkDataObject` at `out` over the
/// implementation held by the handle at `handle` and returns `out`. `this` (ECX) is not read.
pub fn hk_data_world_dict_find_object(
    e: &mut Engine,
    _this: Ptr<HkDataWorldDict>,
    out: u32,
    handle: u32,
) -> u32 {
    let implementation = e.mem.u32(handle);
    e.call(DATA_OBJECT_CONSTRUCT, &args![out, implementation]);
    out
}

// Translated from 00dc72e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::setClassVersion` (Xbox PDB): stores `version` in the class implementation
/// held by the handle at `class_handle`. `this` (ECX) is not read.
pub fn hk_data_world_dict_set_class_version(
    e: &mut Engine,
    _this: Ptr<HkDataWorldDict>,
    class_handle: u32,
    version: i32,
) {
    let class: Ptr<HkDataClassDict> = Ptr::new(e.mem.u32(class_handle));
    e.set(class, HkDataClassDict::version, version);
}

// Translated from 00dc72f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::getType` (Xbox PDB): stores the type code `1` at `out` and returns `out`
/// (left in `EAX`). `this` (ECX) is not read.
pub fn hk_data_world_dict_get_type(e: &mut Engine, _this: Ptr<HkDataWorldDict>, out: u32) -> u32 {
    e.mem.set_u32(out, 1);
    out
}

// Translated from 00dc7300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::findClass` (Xbox PDB): the class implementation registered under `name` in
/// the tracker's class map, or 0 (also for a null `name`).
pub fn hk_data_world_dict_find_class(e: &mut Engine, this: Ptr<HkDataWorldDict>, name: u32) -> u32 {
    let tracker_address = world_tracker(e, this);
    if name == 0 {
        return 0;
    }
    e.call(
        STRING_MAP_GET_WITH_DEFAULT,
        &args![tracker_address + tracker::CLASS_MAP, name, 0u32],
    )
    .u32()
}

// Translated from 00dc7320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::renameClass` (Xbox PDB): interns the current name of the class at
/// `class_handle` (`hkDataClass::getName`; a null name stays null) and hands it, with
/// `new_name`, to the tracker's unnamed method `0x00dc_8f20`.
pub fn hk_data_world_dict_rename_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
    new_name: u32,
) {
    let old_name = e.call(DATA_CLASS_GET_NAME, &args![class_handle]).u32();
    let tracker_address = world_tracker(e, this);
    let interned = intern_name(e, tracker_address, old_name);
    let tracker_address = world_tracker(e, this);
    fn_00dc8f20(e, tracker_address, interned, new_name);
}

// Translated from 00dc7390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::findAllClasses` (Xbox PDB): tail-calls `ObjectTracker::findTrackedClasses`
/// with the tracker and the result array `out`.
pub fn hk_data_world_dict_find_all_classes(e: &mut Engine, this: Ptr<HkDataWorldDict>, out: u32) {
    let tracker_address = world_tracker(e, this);
    object_tracker_find_tracked_classes(e, tracker_address, out);
}

// Translated from 00dc73a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::findObjectsByExactClass` (Xbox PDB): empties the result array `out`
/// (size 0), then `ObjectTracker::findTrackedObjectsByBase(name, 0, 1, out)`.
pub fn hk_data_world_dict_find_objects_by_exact_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_name: u32,
    out: u32,
) {
    let tracker_address = world_tracker(e, this);
    e.mem.set_u32(out + 4, 0);
    object_tracker_find_tracked_objects_by_base(e, tracker_address, class_name, 0, 1, out);
}

// Translated from 00dc73c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::findObjectsByBaseClass` (Xbox PDB): empties the result array `out`
/// (size 0), then `ObjectTracker::findTrackedObjectsByBase(name, 1, 1, out)`.
pub fn hk_data_world_dict_find_objects_by_base_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_name: u32,
    out: u32,
) {
    let tracker_address = world_tracker(e, this);
    e.mem.set_u32(out + 4, 0);
    object_tracker_find_tracked_objects_by_base(e, tracker_address, class_name, 1, 1, out);
}

// Translated from 00dc73e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed fragment of this unit (the map gives it only a range): `(EAX >> 2) + 3`, with the
/// input in the register `EAX`, which is the first word here. `assign_ov6` and `asVec` use it to
/// turn a float count into a member type code.
pub fn fn_00dc73e0(_e: &mut Engine, eax: u32) -> u32 {
    (eax >> 2).wrapping_add(3)
}

// Translated from 00dc73f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::renameClassMember` (Xbox PDB): renames the member `old_name` of the class
/// at `class_handle` to `new_name` (both interned, null stays null) in
///  - every tracked object of the class and of its derived classes (the 0x10-byte value
///    records: the first whose name matches),
///  - every struct array of the class (the 8-byte member records: the first match),
///  - the class's own member list (0x18-byte records, searched from the last: the first match;
///    with no match the exe stores at index -1, i.e. 0x18 bytes before the list, and so does
///    this).
pub fn hk_data_world_dict_rename_class_member(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
    old_name: u32,
    new_name: u32,
) {
    let class_address = e.mem.u32(class_handle);
    let tracker_address = world_tracker(e, this);
    let old_interned = intern_name(e, tracker_address, old_name);
    let tracker_address = world_tracker(e, this);
    let new_interned = intern_name(e, tracker_address, new_name);
    with_two_arrays(e, |e, objects, struct_arrays| {
        array_init_empty(e, objects);
        let name = e.call(DATA_CLASS_GET_NAME, &args![class_handle]).u32();
        let tracker_address = world_tracker(e, this);
        object_tracker_find_tracked_objects_by_base(e, tracker_address, name, 1, 0, objects);
        let mut i = 0;
        while i < e.mem.i32(objects + 4) {
            let object = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add((i as u32).wrapping_mul(4)));
            let count = e.mem.i32(object + 0x14);
            if count > 0 {
                let records = e.mem.u32(object + 0x10);
                let mut j = 0;
                while j < count {
                    let record = records.wrapping_add((j as u32).wrapping_mul(0x10));
                    if e.mem.u32(record) == old_interned {
                        e.mem.set_u32(record, new_interned);
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        array_init_empty(e, struct_arrays);
        let tracker_address = world_tracker(e, this);
        object_tracker_get_tracked_struct_arrays(
            e,
            tracker_address,
            class_address,
            1,
            struct_arrays,
        );
        let mut i = 0;
        while i < e.mem.i32(struct_arrays + 4) {
            let array = e.mem.u32(
                e.mem
                    .u32(struct_arrays)
                    .wrapping_add((i as u32).wrapping_mul(4)),
            );
            let count = e.mem.i32(array + 0x10);
            if count > 0 {
                let records = e.mem.u32(array + 0xc);
                let mut j = 0;
                while j < count {
                    let record = records.wrapping_add((j as u32).wrapping_mul(8));
                    if e.mem.u32(record) == old_interned {
                        e.mem.set_u32(record, new_interned);
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        let class: Ptr<HkDataClassDict> = Ptr::new(class_address);
        let member_count = e.get(class, HkDataClassDict::member_count);
        let mut index = member_count.wrapping_sub(1);
        let mut found = -1;
        if index >= 0 {
            let members = e.get(class, HkDataClassDict::members).addr();
            let mut record = members.wrapping_add((index as u32).wrapping_mul(0x18));
            loop {
                if e.mem.u32(record) == old_interned {
                    found = index;
                    break;
                }
                index -= 1;
                record = record.wrapping_sub(0x18);
                if index < 0 {
                    break;
                }
            }
        }
        let members = e.get(class, HkDataClassDict::members).addr();
        let target = members.wrapping_add((found as u32).wrapping_mul(0x18));
        e.mem.set_u32(target, new_interned);
        array_free(e, struct_arrays, 4);
        array_free(e, objects, 4);
    });
}

// Translated from 00dc7600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::removeClassMember` (Xbox PDB): removes the member `member_name` (interned;
/// null stays null) from the class at `class_handle`: from every tracked object of the class
/// and its derived classes (`hkDataObjectDict::removeMember`), from every struct array of the
/// class (`StructArrayImplementation::removeMember`) and from the class itself
/// (`hkDataClassDict::removeMember`).
pub fn hk_data_world_dict_remove_class_member(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
    member_name: u32,
) {
    let class_address = e.mem.u32(class_handle);
    let tracker_address = world_tracker(e, this);
    let interned = intern_name(e, tracker_address, member_name);
    with_two_arrays(e, |e, objects, struct_arrays| {
        array_init_empty(e, objects);
        let name = e.call(DATA_CLASS_GET_NAME, &args![class_handle]).u32();
        let tracker_address = world_tracker(e, this);
        object_tracker_find_tracked_objects_by_base(e, tracker_address, name, 1, 0, objects);
        let mut i = 0;
        while i < e.mem.i32(objects + 4) {
            let object = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add((i as u32).wrapping_mul(4)));
            hk_data_object_dict_remove_member(e, Ptr::new(object), interned);
            i += 1;
        }
        array_init_empty(e, struct_arrays);
        let tracker_address = world_tracker(e, this);
        object_tracker_get_tracked_struct_arrays(
            e,
            tracker_address,
            class_address,
            1,
            struct_arrays,
        );
        let mut i = 0;
        while i < e.mem.i32(struct_arrays + 4) {
            let array = e.mem.u32(
                e.mem
                    .u32(struct_arrays)
                    .wrapping_add((i as u32).wrapping_mul(4)),
            );
            struct_array_implementation_remove_member(e, array, interned);
            i += 1;
        }
        hk_data_class_dict_remove_member(e, Ptr::new(class_address), interned);
        array_free(e, struct_arrays, 4);
        array_free(e, objects, 4);
    });
}

/// The inlined teardown of one multi-map member of the tracker: the index map is cleared
/// (`hkMapBase::clearAndDeallocate` with the router's allocator, then the one-byte routine
/// `0x00a2_9680`), then the entry array (8-byte entries) is freed.
fn multi_map_destroy(e: &mut Engine, multi_map_address: u32) {
    let map = multi_map_address + multi_map::MAP;
    let allocator = router_allocator(e);
    e.call(POINTER_MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(POINTER_MAP_AFTER_CLEAR, &args![map]);
    array_free(e, multi_map_address + multi_map::ENTRIES, 8);
}

/// The warning the tracker's teardown writes through the error reporter (global `0x0126_8360`,
/// slot `0xc`) for an object still referenced from outside the world: an `hkOstream` over a
/// 0x200-byte buffer receives the start text, the class name (slot `8` of the object gives the
/// class, slot `8` of the class its name), the next text, the external count and the closing
/// text; the report carries severity 1, the id `id`, the buffer, the source file name and the
/// source line `line`. The stream is destroyed afterwards.
fn tracker_report_referenced_object(e: &mut Engine, object: u32, id: u32, line: u32) {
    e.with_stack(0x20c, |e, block| {
        let stream = block.addr();
        let buffer = stream + 0xc;
        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
        let class = e.vcall(object, 8, &args![]).u32();
        let count = i32::from(e.mem.i16(object + 6));
        let name = e.vcall(class, 8, &args![]).u32();
        let stream_now = e
            .call(
                OSTREAM_WRITE_STRING,
                &args![stream, TEXT_THE_OBJECT_OF_CLASS],
            )
            .u32();
        let stream_now = e.call(OSTREAM_WRITE_STRING, &args![stream_now, name]).u32();
        let stream_now = e
            .call(
                OSTREAM_WRITE_STRING,
                &args![stream_now, TEXT_ABOUT_TO_BE_REMOVED],
            )
            .u32();
        let stream_now = e.call(OSTREAM_WRITE_INT, &args![stream_now, count]).u32();
        e.call(
            OSTREAM_WRITE_STRING,
            &args![stream_now, TEXT_FROM_OUTSIDE_THE_WORLD],
        );
        let reporter = e.mem.u32(ERROR_REPORTER);
        e.vcall(
            reporter,
            0xc,
            &args![1u32, id, buffer, TEXT_SOURCE_FILE, line],
        );
        e.call(OSTREAM_DESTRUCT, &args![stream]);
    });
}

// Translated from 00dc7750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Teardown of the object tracker (`hkDataWorldDict::ObjectTracker`; the map leaves this
/// body unnamed, it is the tracker's destructor). Two `hkLocalArray`s (data, size, capacity
/// word, local memory: 0x10 bytes each) take 0x800 and 0x100 bytes of the router's stack
/// allocator; then
///  - the derived-class multi-map is cleared and the contents pointer zeroed,
///  - every class of the class map is collected and every object of the object multi-map is
///    collected while its key is removed,
///  - each collected object with a positive external count is reported, and all of them
///    self-destruct (`hkDataObjectDict::selfDestruct`),
///  - each collected object and class loses one reference,
///  - the class-handle records lose a reference, the interned strings are freed,
///  - the stack allocator is given back, the multi-maps and string maps are destroyed and the
///    class map's destructor ends the function.
pub fn fn_00dc7750(e: &mut Engine, this: Ptr<ObjectTracker>) {
    let tracker_address = this.addr();
    e.with_stack(0x20, |e, block| {
        let objects = block.addr();
        let classes = objects + 0x10;
        array_init_empty(e, objects);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let objects_base = e.mem.u32(router);
        e.mem.set_u32(router, objects_base.wrapping_add(0x800));
        e.mem.set_u32(objects, objects_base);
        e.mem.set_u32(objects + 8, 0x8000_0200);
        e.mem.set_u32(objects + 0xc, objects_base);
        array_init_empty(e, classes);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let classes_base = e.mem.u32(router);
        e.mem.set_u32(router, classes_base.wrapping_add(0x100));
        e.mem.set_u32(classes, classes_base);
        e.mem.set_u32(classes + 0xc, classes_base);
        e.mem.set_u32(classes + 8, 0x8000_0040);
        fn_00dc8fc0(e, tracker_address + tracker::DERIVED_MAP);
        e.mem.set_u32(tracker_address + 0x2c, 0);
        let object_map = tracker_address + tracker::OBJECT_MAP;
        string_map_for_each(e, tracker_address + tracker::CLASS_MAP, |e, class| {
            array_push_word(e, classes, class);
            let mut index = fn_00dc8e30(e, object_map, class);
            if index != -1 {
                loop {
                    let entries = e.mem.u32(object_map);
                    let entry = entries.wrapping_add((index as u32).wrapping_mul(8));
                    let object = e.mem.u32(entry);
                    array_push_word(e, objects, object);
                    let entries = e.mem.u32(object_map);
                    index = e
                        .mem
                        .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
                    if index == -1 {
                        break;
                    }
                }
                hk_serialize_multi_map_remove_key(e, object_map, class);
            }
        });
        let mut i = 0;
        while i < e.mem.i32(objects + 4) {
            let object = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add((i as u32).wrapping_mul(4)));
            if e.mem.i16(object + 6) > 0 {
                tracker_report_referenced_object(e, object, 0x760d_3795, 0x6f4);
            }
            hk_data_object_dict_self_destruct(e, Ptr::new(object));
            i += 1;
        }
        let mut i = 0;
        while i < e.mem.i32(objects + 4) {
            let object = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add((i as u32).wrapping_mul(4)));
            release_object(e, object);
            i += 1;
        }
        let mut i = 0;
        while i < e.mem.i32(classes + 4) {
            let class = e
                .mem
                .u32(e.mem.u32(classes).wrapping_add((i as u32).wrapping_mul(4)));
            release_object(e, class);
            i += 1;
        }
        string_map_for_each(e, tracker_address + tracker::CLASS_HANDLES, |e, record| {
            e.call(REMOVE_REFERENCE, &args![record]);
        });
        string_map_for_each(e, tracker_address + tracker::INTERNED_NAMES, |e, text| {
            e.call(STRING_FREE, &args![text]);
        });
        if classes_base == e.mem.u32(classes) {
            e.mem.set_u32(classes + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, classes_base);
        array_free(e, classes, 4);
        if objects_base == e.mem.u32(objects) {
            e.mem.set_u32(objects + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, objects_base);
        array_free(e, objects, 4);
    });
    multi_map_destroy(e, tracker_address + tracker::ARRAY_MAP);
    e.call(
        STRING_MAP_DESTRUCT,
        &args![tracker_address + tracker::CLASS_HANDLES],
    );
    e.call(
        STRING_MAP_DESTRUCT,
        &args![tracker_address + tracker::INTERNED_NAMES],
    );
    multi_map_destroy(e, tracker_address + tracker::OBJECT_MAP);
    multi_map_destroy(e, tracker_address + tracker::DERIVED_MAP);
    e.call(
        STRING_MAP_DESTRUCT,
        &args![tracker_address + tracker::CLASS_MAP],
    );
}

// Translated from 00dc7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::newObject` (Xbox PDB): makes an `hkDataObjectDict` (0x1c bytes from the
/// router's allocator, slot `4`) of the class held by the handle at `class_handle`: vtable
/// `0x010d_6ee4`, no members (`capacity_and_flags` `0x8000_0000`), one reference for the
/// caller and one more on the class. The new object (found through its virtual slot `8`) is
/// put in the tracker's object multi-map and, when the tracker has no contents yet, becomes
/// the contents. Returns the object.
pub fn hk_data_world_dict_new_object(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
) -> u32 {
    let allocator = router_allocator(e);
    let object = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    let object_ptr: Ptr<HkDataObjectDict> = Ptr::new(object);
    e.set(object_ptr, HkDataObjectDict::memory_size, 0x1c);
    e.set(object_ptr, HkDataObjectDict::external_count, 0);
    e.set(object_ptr, HkDataObjectDict::reference_count, 0);
    e.mem.set_u32(object, 0x010d_6ee4);
    let class = e.mem.u32(class_handle);
    if class != 0 {
        add_object_reference(e, class);
    }
    e.mem.set_u32(object + 0xc, class);
    e.mem.set_u32(object + 0x10, 0);
    e.mem.set_u32(object + 0x14, 0);
    e.mem.set_u32(object + 0x18, 0x8000_0000);
    add_object_reference(e, object);
    let tracker_address = world_tracker(e, this);
    let key = e.vcall(object, 8, &args![]).u32();
    e.with_stack(4, |e, held| {
        e.mem.set_u32(held.addr(), object);
        e.call(
            MULTI_MAP_INSERT,
            &args![tracker_address + tracker::OBJECT_MAP, key, held.addr()],
        );
    });
    if e.mem.u32(tracker_address + 0x2c) == 0 {
        e.mem.set_u32(tracker_address + 0x2c, object);
    }
    object
}

// Translated from 00dc7cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::setClassParent` (Xbox PDB): asks the class (slot `0x10`, its parent), then
/// swaps its parent for the class held by `parent_handle` (one reference more on the new one, one
/// less on the old, whose deleting destructor runs at zero) and calls the tracker's
/// `0x00dc_aaf0` with the slot's answer and the class.
pub fn hk_data_world_dict_set_class_parent(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
    parent_handle: u32,
) {
    let class_address = e.mem.u32(class_handle);
    let answer = e.vcall(class_address, 0x10, &args![]).u32();
    let class_address = e.mem.u32(class_handle);
    let parent = e.mem.u32(parent_handle);
    let parent_field = class_address + 0x18;
    if parent != 0 {
        add_object_reference(e, parent);
    }
    let old_parent = e.mem.u32(parent_field);
    release_object_if_set(e, old_parent);
    e.mem.set_u32(parent_field, parent);
    let tracker_address = world_tracker(e, this);
    let class_address = e.mem.u32(class_handle);
    fn_00dcaaf0(e, tracker_address, answer, class_address);
}

// Translated from 00dc7d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::addClassMember` (Xbox PDB): adds a member to the class held by the
/// handle at `class_handle`. The member `name` (interned, null stays null) is first removed
/// when the class already has it (slot `0x20` of the class, given the raw name, answers
/// `-1` for a missing one). Then the class named `class_name` is looked up or made as a
/// handle (`ObjectTracker::internClassNameHandle`), `hkDataClassDict::addMember` records
/// `(name, member_type, handle, extra)` and the handle's reference is dropped.
pub fn hk_data_world_dict_add_class_member(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
    name: u32,
    member_type: u32,
    class_name: u32,
    extra: u32,
) {
    let class_address = e.mem.u32(class_handle);
    let tracker_address = world_tracker(e, this);
    let interned = intern_name(e, tracker_address, name);
    let existing = e.vcall(class_address, 0x20, &args![name]).i32();
    if existing != -1 {
        hk_data_class_dict_remove_member(e, Ptr::new(class_address), interned);
    }
    let tracker_address = world_tracker(e, this);
    e.with_stack(4, |e, handle| {
        object_tracker_intern_class_name_handle(e, tracker_address, handle.addr(), class_name);
        hk_data_class_dict_add_member(
            e,
            Ptr::new(class_address),
            interned,
            member_type,
            handle.addr(),
            extra,
        );
        let held = e.mem.u32(handle.addr());
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
    });
}

// Translated from 00dc7dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::castObject` (Xbox PDB): tail-calls `ObjectTracker::retractCastedObject`
/// with the tracker and the words the two pointers `first` and `second` point at.
pub fn hk_data_world_dict_cast_object(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    first: u32,
    second: u32,
) {
    let tracker_address = world_tracker(e, this);
    let first_word = e.mem.u32(first);
    let second_word = e.mem.u32(second);
    object_tracker_retract_casted_object(e, tracker_address, first_word, second_word);
}

// Translated from 00dc7de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayDict_create` (Xbox PDB; cdecl, three stack words): makes the array
/// implementation for the type code `array_type`. With bit `0x20` set it makes an
/// `ArrayOfTuplesImplementation` (0x28 bytes from the router's allocator: vtable
/// `0x010d_6e4c`, the embedded view with vtable `0x010d_6db4`, the view's back pointer, one
/// reference on the object made for the type without the bit by the recursive call
/// `(type & !0x20, class, 0)`, and `tuple_size` at `+0x24`). Otherwise `array_type & 0xf`
/// picks the class: 1 byte, 2 int, 3 real, 4/6/7 vector arrays of 4/12/16 floats, 8 pointer
/// (given `class`), 9 struct (given `class`), 10 string; every other value (5, 0, 11 and
/// up) answers 0. Each is allocated at its size, which is stored at `+4`, and constructed.
pub fn hk_data_array_dict_create(
    e: &mut Engine,
    array_type: u32,
    class: u32,
    tuple_size: u32,
) -> u32 {
    if array_type & 0x20 != 0 {
        let inner = hk_data_array_dict_create(e, array_type & 0xffff_ffdf, class, 0);
        let allocator = router_allocator(e);
        let object = e.vcall(allocator, 4, &args![0x28u32]).u32();
        e.mem.set_u16(object + 4, 0x28);
        e.mem.set_u16(object + 6, 0);
        e.mem.set_u32(object + 8, 0);
        e.mem.set_u32(object, 0x010d_6e4c);
        e.mem.set_u16(object + 0x12, 0);
        e.mem.set_u32(object + 0x14, 0);
        e.mem.set_u32(object + 0xc, 0x010d_6db4);
        e.mem.set_u32(object + 0x20, inner);
        e.mem.set_u32(object + 0x24, tuple_size);
        add_object_reference(e, inner);
        let count = e.mem.u32(object + 0x14).wrapping_add(1);
        e.mem.set_u32(object + 0x14, count);
        e.mem.set_u32(object + 0x18, object);
        return object;
    }
    let kind = (array_type & 0xf).wrapping_sub(1);
    if kind > 9 {
        return 0;
    }
    let size = match kind {
        0..=2 | 9 => 0x18u32,
        3 | 5..=7 => 0x1c,
        8 => 0x20,
        _ => return 0,
    };
    let allocator = router_allocator(e);
    let object = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(object + 4, size as u16);
    match kind {
        0 => byte_array_implementation_construct(e, object),
        1 => int_array_implementation_construct(e, object),
        2 => real_array_implementation_construct(e, object),
        3 => vec_array_implementation_construct(e, object, 4),
        5 => vec_array_implementation_construct(e, object, 0xc),
        6 => vec_array_implementation_construct(e, object, 0x10),
        7 => pointer_array_implementation_construct(e, object, class),
        8 => struct_array_implementation_construct(e, object, class),
        _ => cstring_array_implementation_construct(e, object),
    }
}

// Translated from 00dc7fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the object tracker (`ECX` = the 0x88-byte block, unnamed in the map): the
/// three string maps (`hkCachedHashMap` constructor, whose stack word is never read; the exe
/// pushes a word with only its low byte set to 0) and the three multi-maps are constructed
/// and the contents pointer cleared. Returns `this`.
pub fn fn_00dc7fd0(e: &mut Engine, this: Ptr<ObjectTracker>) -> Ptr<ObjectTracker> {
    let address = this.addr();
    e.call(
        STRING_MAP_CONSTRUCT,
        &args![address + tracker::CLASS_MAP, 0u32],
    );
    e.call(MULTI_MAP_CONSTRUCT, &args![address + tracker::DERIVED_MAP]);
    e.mem.set_u32(address + 0x2c, 0);
    e.call(MULTI_MAP_CONSTRUCT, &args![address + tracker::OBJECT_MAP]);
    e.call(
        STRING_MAP_CONSTRUCT,
        &args![address + tracker::INTERNED_NAMES, 0u32],
    );
    e.call(
        STRING_MAP_CONSTRUCT,
        &args![address + tracker::CLASS_HANDLES, 0u32],
    );
    e.call(MULTI_MAP_CONSTRUCT, &args![address + tracker::ARRAY_MAP]);
    this
}

// Translated from 00dc8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::hkDataWorldDict` (Xbox PDB): vtable `0x010d_7510`, `+6` = 1, and a new
/// object tracker (0x88 bytes from the router's allocator, slot `4`; a null block leaves a
/// null tracker). Returns `this`.
pub fn hk_data_world_dict_construct(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
) -> Ptr<HkDataWorldDict> {
    e.set(this, HkDataWorldDict::kind, 1);
    e.mem.set_u32(this.addr(), 0x010d_7510);
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x88u32]).u32();
    let tracker_ptr = if block != 0 {
        fn_00dc7fd0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    e.set(this, HkDataWorldDict::tracker, tracker_ptr);
    this
}

// Translated from 00dc8080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::~hkDataWorldDict` (Xbox PDB): with a tracker, runs its teardown and
/// returns the block to the router's allocator (slot `8`, size 0x88); the vtable goes to
/// `0x010d_7510` first and `0x0102_e388` last.
pub fn hk_data_world_dict_destruct(e: &mut Engine, this: Ptr<HkDataWorldDict>) {
    let tracker_ptr = e.get(this, HkDataWorldDict::tracker);
    e.mem.set_u32(this.addr(), 0x010d_7510);
    if !tracker_ptr.is_null() {
        fn_00dc7750(e, tracker_ptr);
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![tracker_ptr.addr(), 0x88u32]);
    }
    e.mem.set_u32(this.addr(), 0x0102_e388);
}

// Translated from 00dc80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::newClass` (Xbox PDB): makes an `hkDataClassDict` (0x40 bytes from the
/// router's allocator, slot `4`) from the description at `description`
/// (`{name, version, parent name, members, member count}`, members 0x10 bytes each:
/// `{name, type, extra, class name}`). The parent, looked up by its raw name in the tracker's
/// class map, gains a reference and replaces the old parent (which loses one). The class gets
/// one reference of its own and is handed to `ObjectTracker::trackClass`. Each member is added
/// with `hkDataClassDict::addMember` under its interned name and the handle made by
/// `internClassNameHandle`, whose reference is dropped again. Returns the class.
pub fn hk_data_world_dict_new_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    description: u32,
) -> u32 {
    let world = this.addr();
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let name = e.mem.u32(description);
    let version = e.mem.u32(description + 4);
    let class = hk_data_class_dict_construct(e, block, world, name, version);
    let parent_name = e.mem.u32(description + 8);
    if parent_name != 0 {
        let tracker_address = world_tracker(e, this);
        let parent = e
            .call(
                STRING_MAP_GET_WITH_DEFAULT,
                &args![tracker_address + tracker::CLASS_MAP, parent_name, 0u32],
            )
            .u32();
        if parent != 0 {
            add_object_reference(e, parent);
        }
        let old_parent = e.mem.u32(class + 0x18);
        release_object_if_set(e, old_parent);
        e.mem.set_u32(class + 0x18, parent);
    }
    add_object_reference(e, class);
    let tracker_address = world_tracker(e, this);
    object_tracker_track_class(e, tracker_address, class);
    let member_count = e.mem.i32(description + 0x10);
    if member_count > 0 {
        e.with_stack(4, |e, handle| {
            let mut index = 0;
            let mut offset = 0u32;
            loop {
                let member = e.mem.u32(description + 0xc).wrapping_add(offset);
                let tracker_address = world_tracker(e, this);
                let member_name = e.mem.u32(member);
                let interned = intern_name(e, tracker_address, member_name);
                let member_type = e.mem.u32(member + 4);
                let extra = e.mem.u32(member + 8);
                let class_name = e.mem.u32(member + 0xc);
                let tracker_address = world_tracker(e, this);
                object_tracker_intern_class_name_handle(
                    e,
                    tracker_address,
                    handle.addr(),
                    class_name,
                );
                hk_data_class_dict_add_member(
                    e,
                    Ptr::new(class),
                    interned,
                    member_type,
                    handle.addr(),
                    extra,
                );
                let held = e.mem.u32(handle.addr());
                if held != 0 {
                    e.call(REMOVE_REFERENCE, &args![held]);
                }
                offset = offset.wrapping_add(0x10);
                index += 1;
                e.mem.set_u32(handle.addr(), 0);
                if index >= member_count {
                    break;
                }
            }
        });
    }
    class
}

// Translated from 00dc8210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::newArray` (Xbox PDB): makes the array for the member description
/// `description` (`+8` flags, `+0xc` class, `+0x10` size or tuple size, `+0` member index) with
/// `hkDataArrayDict_create`. With flag `0x10` the flag is cleared and `+0x10` is the tuple size
/// (the array's size stays 0); otherwise flag `0x20` is cleared and `+0x10` is the size. The
/// array is sized through its slot `0x10`, then slot `0x54` of the implementation held by
/// `object_handle` stores it under the member index. `this` (ECX) is not read. Returns the
/// array.
pub fn hk_data_world_dict_new_array(
    e: &mut Engine,
    _this: Ptr<HkDataWorldDict>,
    object_handle: u32,
    description: u32,
) -> u32 {
    let flags = e.mem.u32(description + 8);
    let (size, tuple_size, flags) = if flags & 0x10 != 0 {
        (0, e.mem.u32(description + 0x10), flags & 0xffff_ffef)
    } else {
        (e.mem.u32(description + 0x10), 0, flags & 0xffff_ffdf)
    };
    let class = e.mem.u32(description + 0xc);
    let array = hk_data_array_dict_create(e, flags, class, tuple_size);
    e.vcall(array, 0x10, &args![size]);
    let implementation = e.mem.u32(object_handle);
    let member_index = e.mem.u32(description);
    e.vcall(implementation, 0x54, &args![member_index, array]);
    array
}

// Translated from 00dc8270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::wrapClass` (Xbox PDB; `__fastcall`, one stack word): makes the
/// `hkDataClassDict` (0x40 bytes from the router's allocator, slot `4`) for the `hkClass` at
/// `hk_class` (name, described version) and
///  - gives it its parent: the class registered in the tracker's class map under the parent
///    class's name, or else the result of wrapping the parent class (recursively); one reference
///    on the new parent, one less on the old,
///  - takes one reference on the class and hands it to `ObjectTracker::trackClass`,
///  - adds every declared member: its name interned, its type from
///    `hkDataObjectUtil::getTypeFromMemberType` (unless the member has flag `0x400`, then 0),
///    its class handle from the member's `hk.DataObjectType` attribute, else the class's
///    attribute of that name (both read through `hkClassMemberAccessor` as `typeName`), else
///    the member class's name (`internClassNameHandle`); with no member class the handle is
///    the static default handle `0x011b_7c00`.
/// Returns the class.
pub fn hk_data_world_dict_wrap_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    hk_class: u32,
) -> u32 {
    let world = this.addr();
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let version = e
        .call(HK_CLASS_GET_DESCRIBED_VERSION, &args![hk_class])
        .u32();
    let name = e.call(HK_CLASS_GET_NAME, &args![hk_class]).u32();
    let class = hk_data_class_dict_construct(e, block, world, name, version);
    let parent_class = e.call(HK_CLASS_GET_PARENT, &args![hk_class]).u32();
    if parent_class != 0 {
        let parent_class = e.call(HK_CLASS_GET_PARENT, &args![hk_class]).u32();
        let parent_name = e.call(HK_CLASS_GET_NAME, &args![parent_class]).u32();
        let tracker_address = world_tracker(e, this);
        let mut parent = 0;
        if parent_name != 0 {
            parent = e
                .call(
                    STRING_MAP_GET_WITH_DEFAULT,
                    &args![tracker_address + tracker::CLASS_MAP, parent_name, 0u32],
                )
                .u32();
        }
        if parent == 0 {
            let parent_class = e.call(HK_CLASS_GET_PARENT, &args![hk_class]).u32();
            parent = hk_data_world_dict_wrap_class(e, this, parent_class);
        }
        if parent != 0 {
            add_object_reference(e, parent);
        }
        let old_parent = e.mem.u32(class + 0x18);
        release_object_if_set(e, old_parent);
        e.mem.set_u32(class + 0x18, parent);
    }
    add_object_reference(e, class);
    let tracker_address = world_tracker(e, this);
    object_tracker_track_class(e, tracker_address, class);
    let count = e
        .call(HK_CLASS_GET_NUM_DECLARED_MEMBERS, &args![hk_class])
        .i32();
    if count <= 0 {
        return class;
    }
    // The locals the exe passes by address: the handle (+0), three out words (+0xc, +0x10,
    // +0x14) and two `hkClassMemberAccessor` objects (+0x24, +0x2c).
    e.with_stack(0x34, |e, locals| {
        let handle_slot = locals.addr();
        let mut index = 0i32;
        loop {
            let member = e
                .call(HK_CLASS_GET_DECLARED_MEMBER, &args![hk_class, index])
                .u32();
            let mut member_type = 0u32;
            e.call(ADD_REFERENCE, &args![DEFAULT_CLASS_HANDLE]);
            let mut handle = DEFAULT_CLASS_HANDLE;
            e.mem.set_u32(handle_slot, handle);
            if e.mem.u16(member + 0x10) & 0x400 != 0x400 {
                let raw_type = u32::from(e.mem.u8(member + 0xc));
                let raw_subtype = u32::from(e.mem.u8(member + 0xd));
                let array_size = e
                    .call(HK_CLASS_MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member])
                    .u32();
                let member_class = e.call(HK_CLASS_MEMBER_GET_CLASS, &args![member]).u32();
                member_type = e
                    .call(
                        TYPE_FROM_MEMBER_TYPE,
                        &args![raw_type, raw_subtype, member_class, array_size],
                    )
                    .u32();
                let member_class = e.call(HK_CLASS_MEMBER_GET_CLASS, &args![member]).u32();
                if member_class != 0 {
                    let attribute_name = e.mem.u32(DATA_OBJECT_TYPE_NAME);
                    let attribute = e
                        .call(
                            HK_CLASS_MEMBER_GET_ATTRIBUTE,
                            &args![member, attribute_name],
                        )
                        .u32();
                    if attribute != 0 {
                        handle = wrap_class_attribute_handle(
                            e,
                            this,
                            attribute,
                            handle_slot + 0x24,
                            handle_slot + 0x0c,
                            handle_slot,
                        );
                    } else {
                        let attribute_name = e.mem.u32(DATA_OBJECT_TYPE_NAME);
                        let member_class = e.call(HK_CLASS_MEMBER_GET_CLASS, &args![member]).u32();
                        let class_attribute = e
                            .call(HK_CLASS_GET_ATTRIBUTE, &args![member_class, attribute_name])
                            .u32();
                        if class_attribute != 0 {
                            handle = wrap_class_attribute_handle(
                                e,
                                this,
                                class_attribute,
                                handle_slot + 0x2c,
                                handle_slot + 0x10,
                                handle_slot,
                            );
                        } else {
                            let member_class =
                                e.call(HK_CLASS_MEMBER_GET_CLASS, &args![member]).u32();
                            let class_name = e.call(HK_CLASS_GET_NAME, &args![member_class]).u32();
                            let tracker_address = world_tracker(e, this);
                            let out = handle_slot + 0x14;
                            let result = object_tracker_intern_class_name_handle(
                                e,
                                tracker_address,
                                out,
                                class_name,
                            );
                            handle = adopt_class_handle(e, result, out, handle_slot);
                        }
                    }
                }
            }
            let member_name = e.mem.u32(member);
            let tracker_address = world_tracker(e, this);
            let interned = intern_name(e, tracker_address, member_name);
            let array_size = e
                .call(HK_CLASS_MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member])
                .u32();
            hk_data_class_dict_add_member(
                e,
                Ptr::new(class),
                interned,
                member_type,
                handle_slot,
                array_size,
            );
            if handle != 0 {
                e.call(REMOVE_REFERENCE, &args![handle]);
            }
            index += 1;
            let count = e
                .call(HK_CLASS_GET_NUM_DECLARED_MEMBERS, &args![hk_class])
                .i32();
            if index >= count {
                break;
            }
        }
    });
    class
}

/// The inlined tail of `wrapClass` after `internClassNameHandle(out, name)` returned `result`:
/// the new handle gains a reference and the static default handle loses the one taken for this
/// member, the handle is kept in `handle_slot`, the out word gives up its own reference and is
/// cleared. Returns the new handle.
fn adopt_class_handle(e: &mut Engine, result: u32, out: u32, handle_slot: u32) -> u32 {
    let handle = e.mem.u32(result);
    if handle != 0 {
        e.call(ADD_REFERENCE, &args![handle]);
    }
    e.call(REMOVE_REFERENCE, &args![DEFAULT_CLASS_HANDLE]);
    let held = e.mem.u32(out);
    let handle = e.mem.u32(result);
    e.mem.set_u32(handle_slot, handle);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(out, 0);
    handle
}

/// The two attribute branches of `wrapClass` (the member's and the member class's): an
/// `hkClassMemberAccessor` over `attribute` named `typeName` yields the class name, from which
/// `internClassNameHandle` makes the handle.
fn wrap_class_attribute_handle(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    attribute: u32,
    accessor: u32,
    out: u32,
    handle_slot: u32,
) -> u32 {
    e.call(
        CLASS_MEMBER_ACCESSOR_CONSTRUCT,
        &args![accessor, attribute, TEXT_TYPE_NAME],
    );
    let text = e
        .call(CLASS_MEMBER_ACCESSOR_AS_CSTRING, &args![accessor, 0u32])
        .u32();
    let class_name = e.mem.u32(text);
    let tracker_address = world_tracker(e, this);
    let result = object_tracker_intern_class_name_handle(e, tracker_address, out, class_name);
    adopt_class_handle(e, result, out, handle_slot)
}

// Translated from 00dc85a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::copyClassFromWorld` (Xbox PDB; two stack words after `this`): makes in
/// `this` the class `class_name` of `source_world`, with its members, and returns it. Both
/// worlds are asked through virtual slot `0x24` (`findClass`):
///  - `this` already has the class: that class is the answer,
///  - the source world's class (`hkDataClass` handle) gives name, version and parent; when it
///    has a parent, `this` is asked once more for the class of the same name and an existing
///    one ends the function (the handles and the description are given back first),
///  - otherwise slot `0xc` of `this` (`newClass`) gets a description with no members, then
///    the source class's member infos (0x14 bytes each, from the router's second allocator,
///    slot `0xc`, zeroed) are read and each member is added with `addClassMember` after
///    copying its class (recursively, by the class handle's name) when it has one.
/// The temporary arrays are freed again before the class is returned.
pub fn hk_data_world_dict_copy_class_from_world(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_name: u32,
    source_world: u32,
) -> u32 {
    let world = this.addr();
    let existing = e.vcall(world, 0x24, &args![class_name]).u32();
    if existing != 0 {
        return existing;
    }
    // The locals of the exe frame: the parent handle (+0x00), the new class handle (+0x0c),
    // the member-info array (+0x14), the description (+0x20: name, version, parent name, then
    // the member array +0x2c) and the source class handle (+0x3c).
    e.with_stack(0x40, |e, frame| {
        let base = frame.addr();
        let parent_handle = base;
        let new_class_handle = base + 0x0c;
        let infos = base + 0x14;
        let description = base + 0x20;
        let members = base + 0x2c;
        let source_class = base + 0x3c;
        let source_impl = e.vcall(source_world, 0x24, &args![class_name]).u32();
        e.call(DATA_CLASS_CONSTRUCT, &args![source_class, source_impl]);
        e.mem.set_u32(members, 0);
        e.mem.set_u32(members + 4, 0);
        e.mem.set_u32(members + 8, 0x8000_0000);
        let name = e.call(DATA_CLASS_GET_NAME, &args![source_class]).u32();
        e.mem.set_u32(description, name);
        let version = e.call(DATA_CLASS_GET_VERSION, &args![source_class]).u32();
        e.mem.set_u32(description + 4, version);
        e.call(DATA_CLASS_GET_PARENT, &args![source_class, parent_handle]);
        if !e.call(DATA_CLASS_IS_NULL, &args![parent_handle]).bool() {
            let parent_name = e.call(DATA_CLASS_GET_NAME, &args![parent_handle]).u32();
            e.mem.set_u32(description + 8, parent_name);
            let own_name = e.mem.u32(description);
            let found = e.vcall(world, 0x24, &args![own_name]).u32();
            if found != 0 {
                e.call(DATA_CLASS_DESTRUCT, &args![parent_handle]);
                e.call(DATA_CLASS_CINFO_DESTRUCT, &args![description]);
                e.call(DATA_CLASS_DESTRUCT, &args![source_class]);
                return found;
            }
        } else {
            e.mem.set_u32(description + 8, 0);
        }
        let new_class = e.vcall(world, 0xc, &args![description]).u32();
        let declared = e
            .call(DATA_CLASS_GET_NUM_DECLARED_MEMBERS, &args![source_class])
            .i32();
        e.mem.set_u32(infos, 0);
        e.mem.set_u32(infos + 4, 0);
        e.mem.set_u32(infos + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let temp_allocator = e.mem.u32(router + 0xc);
        let (data, size, capacity_word) = if declared == 0 {
            (0, 0, 0x8000_0000)
        } else {
            e.mem.set_i32(base + 4, declared.wrapping_mul(20));
            let data = e.vcall(temp_allocator, 0xc, &args![base + 4]).u32();
            let size = e.mem.i32(base + 4) / 20;
            (
                data,
                size,
                if size != 0 { size as u32 } else { 0x8000_0000 },
            )
        };
        e.mem.set_u32(infos, data);
        e.mem.set_i32(infos + 4, size);
        e.mem.set_u32(infos + 8, capacity_word);
        if size > 0 {
            let mut element = data;
            let mut left = size;
            loop {
                if element != 0 {
                    for word in 0..5 {
                        e.mem.set_u32(element + 4 * word, 0);
                    }
                }
                element = element.wrapping_add(0x14);
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
        e.call(
            DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO,
            &args![source_class, infos],
        );
        let info_count = e.mem.i32(infos + 4);
        array_reserve(e, members, info_count, 0x10);
        e.call(DATA_CLASS_CONSTRUCT, &args![new_class_handle, new_class]);
        if e.mem.i32(infos + 4) > 0 {
            let mut index = 0i32;
            let mut offset = 0u32;
            loop {
                let info = e.mem.u32(infos).wrapping_add(offset);
                let mut member_class_name = 0;
                if e.mem.u32(info + 0xc) != 0 {
                    let handle_impl = e.mem.u32(info + 0xc);
                    member_class_name = e.vcall(handle_impl, 8, &args![]).u32();
                    hk_data_world_dict_copy_class_from_world(
                        e,
                        this,
                        member_class_name,
                        source_world,
                    );
                }
                let member_name = e.mem.u32(info);
                let member_type = e.mem.u32(info + 8);
                let extra = e.mem.u32(info + 0x10);
                hk_data_world_dict_add_class_member(
                    e,
                    this,
                    new_class_handle,
                    member_name,
                    member_type,
                    member_class_name,
                    extra,
                );
                index += 1;
                offset = offset.wrapping_add(0x14);
                if index >= e.mem.i32(infos + 4) {
                    break;
                }
            }
        }
        e.call(DATA_CLASS_DESTRUCT, &args![new_class_handle]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let temp_allocator = e.mem.u32(router + 0xc);
        let capacity_word = e.mem.u32(infos + 8);
        e.mem.set_u32(infos + 4, 0);
        if capacity_word & 0x8000_0000 == 0 {
            let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(20);
            let data = e.mem.u32(infos);
            e.vcall(temp_allocator, 0x10, &args![data, bytes]);
        }
        e.call(DATA_CLASS_DESTRUCT, &args![parent_handle]);
        e.mem.set_u32(infos, 0);
        e.mem.set_u32(infos + 8, 0x8000_0000);
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(members + 8);
        e.mem.set_u32(members + 4, 0);
        if capacity_word & 0x8000_0000 == 0 {
            let bytes = (capacity_word & 0x3fff_ffff) << 4;
            let data = e.mem.u32(members);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(members, 0);
        e.mem.set_u32(members + 8, 0x8000_0000);
        e.call(DATA_CLASS_DESTRUCT, &args![source_class]);
        new_class
    })
}

// Translated from 00dc8870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::removeClass` (Xbox PDB): empties the class handle at `class_handle` (its
/// first word becomes that of a null `hkDataClass`), tells the tracker to stop tracking the
/// class it held (`ObjectTracker::untrackClass`) and drops one reference of it (its deleting
/// destructor runs at zero).
pub fn hk_data_world_dict_remove_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldDict>,
    class_handle: u32,
) {
    let class = e.mem.u32(class_handle);
    e.with_stack(4, |e, empty| {
        e.call(DATA_CLASS_CONSTRUCT, &args![empty.addr(), 0u32]);
        let word = e.mem.u32(empty.addr());
        e.mem.set_u32(class_handle, word);
        e.call(DATA_CLASS_DESTRUCT, &args![empty.addr()]);
    });
    let tracker_address = world_tracker(e, this);
    object_tracker_untrack_class(e, tracker_address, class);
    release_object(e, class);
}

// Translated from 00dc88c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::Union::clearArray` (Xbox PDB; cdecl): gives back what `count`
/// 8-byte union values at `array` of type `array_type` own.
///  - A type with bit `0x10` or `0x20` (an array of tuples or an array of arrays): only the
///    first value's object loses a reference (its deleting destructor runs at zero).
///  - Types 4, 6 and 7 (vectors): each non-null pointer is freed through the router's
///    allocator (slot `8`, size `(type - 3) * 16`) and cleared.
///  - Types 8 and 9 (objects): each non-null object loses a reference and the slot is cleared.
///  - Type 10 (strings): each value goes to `hkString::strFree` and is cleared.
///  - Type 5 and every other type: nothing.
pub fn union_clear_array(e: &mut Engine, array_type: u32, array: u32, count: i32) {
    if array_type & 0x30 != 0 {
        let first = e.mem.u32(array);
        release_object_if_set(e, first);
        return;
    }
    match array_type.wrapping_sub(4) {
        0 | 2 | 3 => {
            let mut index = 0;
            while index < count {
                let slot = array.wrapping_add((index as u32).wrapping_mul(8));
                let pointer = e.mem.u32(slot);
                if pointer != 0 {
                    let allocator = router_allocator(e);
                    let size = array_type.wrapping_sub(3) << 4;
                    e.vcall(allocator, 8, &args![pointer, size]);
                    e.mem.set_u32(slot, 0);
                }
                index += 1;
            }
        }
        4 | 5 => {
            let mut index = 0;
            while index < count {
                let slot = array.wrapping_add((index as u32).wrapping_mul(8));
                let object = e.mem.u32(slot);
                if object != 0 {
                    release_object(e, object);
                    e.mem.set_u32(slot, 0);
                }
                index += 1;
            }
        }
        6 => {
            let mut index = 0;
            while index < count {
                let slot = array.wrapping_add((index as u32).wrapping_mul(8));
                let text = e.mem.u32(slot);
                e.call(STRING_FREE, &args![text]);
                e.mem.set_u32(slot, 0);
                index += 1;
            }
        }
        _ => {}
    }
}

// Translated from 00dc8cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::_scalar_deleting_destructor_` (Xbox PDB): sets the vtable to
/// `0x010d_6e4c`, drops one reference of the element array (`+0x20`; its deleting destructor
/// runs at zero), puts the base vtable `0x010d_62f4` at `+0xc` and `+0`, and, when bit 0 of
/// `flags` is set, returns the block to the router's allocator (slot `8`) with the 16-bit size
/// kept at `+4`. Returns `this`.
pub fn array_of_tuples_implementation_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    flags: u8,
) -> Ptr<ArrayOfTuplesImplementation> {
    let address = this.addr();
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.mem.set_u32(address, 0x010d_6e4c);
    release_object(e, elements);
    e.mem.set_u32(address + 0xc, 0x010d_62f4);
    e.mem.set_u32(address, 0x010d_62f4);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(address + 4));
        e.vcall(allocator, 8, &args![address, size]);
    }
    this
}

// Translated from 00dc8d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::internClassNameHandle` (Xbox PDB): sets the word at `out` to
/// the class-handle record of `name` and returns `out`. The word first holds the static default
/// handle `0x011b_7c00` (taking one reference of it; a word that held something would lose
/// one). A non-null `name` is looked up in the tracker's handle map; a missing record is made
/// (0xc bytes from the router's allocator, slot `4`: vtable `0x010d_6da0`, size at `+4`,
/// reference count 1 at `+6`, the interned name at `+8` from `0x00dc_8d20`) and inserted under
/// that interned name. The record gains a reference and replaces the word's content (which
/// loses one).
pub fn object_tracker_intern_class_name_handle(
    e: &mut Engine,
    this: u32,
    out: u32,
    name: u32,
) -> u32 {
    e.mem.set_u32(out, 0);
    e.call(ADD_REFERENCE, &args![DEFAULT_CLASS_HANDLE]);
    let held = e.mem.u32(out);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(out, DEFAULT_CLASS_HANDLE);
    if name == 0 {
        return out;
    }
    let handles = this + tracker::CLASS_HANDLES;
    let mut record = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![handles, name, 0u32])
        .u32();
    if record == 0 {
        let allocator = router_allocator(e);
        record = e.vcall(allocator, 4, &args![0xcu32]).u32();
        e.mem.set_u16(record + 4, 0xc);
        let interned = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), name);
            let result = e
                .call(
                    UID_FROM_CLASS_VERSION_CACHE,
                    &args![this, slot.addr(), name],
                )
                .u32();
            e.mem.u32(result)
        });
        e.mem.set_u16(record + 6, 1);
        e.mem.set_u32(record, 0x010d_6da0);
        e.mem.set_u32(record + 8, interned);
        e.call(STRING_MAP_INSERT, &args![handles, interned, record]);
    }
    e.call(ADD_REFERENCE, &args![record]);
    let held = e.mem.u32(out);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(out, record);
    out
}

// Translated from 00dc8e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of a multi-map (the map gives it only a range): the first entry index of
/// `key`, `-1` when missing (`hkMapBase::getWithDefault` on the index map at `+0xc`).
pub fn fn_00dc8e30(e: &mut Engine, this: u32, key: u32) -> i32 {
    e.call(
        POINTER_MAP_GET_WITH_DEFAULT,
        &args![this + multi_map::MAP, key, 0xffff_ffffu32],
    )
    .i32()
}

// Translated from 00dc8e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getMemberInfoByName` (Xbox PDB): the address of the first 0x18-byte
/// member record whose name word equals `name` (a pointer compare), searching the class and then
/// its parents; 0 when there is none (also for a null class).
pub fn hk_data_class_dict_get_member_info_by_name(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    name: u32,
) -> u32 {
    let mut class = this;
    while !class.is_null() {
        let count = e.get(class, HkDataClassDict::member_count);
        let members = e.get(class, HkDataClassDict::members).addr();
        let mut index = 0;
        while index < count {
            let record = members.wrapping_add((index as u32).wrapping_mul(0x18));
            if e.mem.u32(record) == name {
                return record;
            }
            index += 1;
        }
        class = e.get(class, HkDataClassDict::parent);
    }
    0
}

// Translated from 00dc8ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::removeMember` (Xbox PDB): removes the 8-byte record `{name,
/// array}` whose name word equals `name`: the array loses a reference (its deleting destructor
/// runs at zero), the count drops by one and the later records move down by copying
/// `(count - index) * 8` bytes, a word at a time, from eight bytes further on.
pub fn struct_array_implementation_remove_member(e: &mut Engine, this: u32, name: u32) {
    let count = e.mem.i32(this + 0x10);
    if count <= 0 {
        return;
    }
    let records = e.mem.u32(this + 0xc);
    let mut index = 0i32;
    let mut record = records;
    while e.mem.u32(record) != name {
        index += 1;
        record = record.wrapping_add(8);
        if index >= count {
            return;
        }
    }
    let array = e.mem.u32(record + 4);
    release_object(e, array);
    let remaining = e.mem.i32(this + 0x10).wrapping_sub(1);
    e.mem.set_i32(this + 0x10, remaining);
    let bytes = remaining.wrapping_sub(index).wrapping_mul(8);
    let data = e.mem.u32(this + 0xc);
    let mut destination = data.wrapping_add((index as u32).wrapping_mul(8));
    if bytes > 0 {
        let mut words = ((bytes as u32).wrapping_sub(1) >> 2).wrapping_add(1);
        while words != 0 {
            let word = e.mem.u32(destination.wrapping_add(8));
            e.mem.set_u32(destination, word);
            destination = destination.wrapping_add(4);
            words -= 1;
        }
    }
}

// Translated from 00dc8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of the object tracker (the map gives it only a range): renames the class
/// `old_name` (already interned) to `new_name`, which is interned here (null stays null).
/// The class found under the old name in the class map is remembered. When the interned new
/// name differs from the old one, the class-handle record of the old name moves to the new
/// name (removed, its name word at `+8` set, inserted again). Finally the class map's entry of
/// the old name is removed and the remembered class inserted under the new name.
pub fn fn_00dc8f20(e: &mut Engine, this: u32, old_name: u32, new_name: u32) {
    let class = if old_name == 0 {
        0
    } else {
        e.call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![this + tracker::CLASS_MAP, old_name, 0u32],
        )
        .u32()
    };
    let mut interned = 0;
    if new_name != 0 {
        interned = intern_string(e, this + tracker::INTERNED_NAMES, new_name);
    }
    if old_name != interned {
        let handles = this + tracker::CLASS_HANDLES;
        let record = e
            .call(STRING_MAP_GET_WITH_DEFAULT, &args![handles, old_name, 0u32])
            .u32();
        e.call(STRING_MAP_REMOVE, &args![handles, old_name]);
        e.mem.set_u32(record + 8, interned);
        e.call(STRING_MAP_INSERT, &args![handles, interned, record]);
    }
    e.call(
        STRING_MAP_REMOVE,
        &args![this + tracker::CLASS_MAP, old_name],
    );
    e.call(
        STRING_MAP_INSERT,
        &args![this + tracker::CLASS_MAP, interned, class],
    );
}

// Translated from 00dc8fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of a multi-map (the map gives it only a range): empties it. The entry count
/// at `+4` becomes 0, the index map at `+0xc` is reset (`0x00cb_1a60`) and the free list head
/// at `+0x18` becomes `-1`.
pub fn fn_00dc8fc0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this + 4, 0);
    e.call(POINTER_MAP_RESET, &args![this + multi_map::MAP]);
    e.mem.set_u32(this + multi_map::FREE_HEAD, 0xffff_ffff);
}

// Translated from 00dc8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of a multi-map (the map gives it only a range): finds `key`'s chain of
/// entries and walks it for the first entry whose value equals the word at `value`; that
/// entry is removed by index (`hkSerializeMultiMap::removeByIndex`, given the key and the
/// entry index).
pub fn fn_00dc8fe0(e: &mut Engine, this: u32, key: u32, value: u32) {
    let mut index = fn_00dc8e30(e, this, key);
    if index == -1 {
        return;
    }
    let wanted = e.mem.u32(value);
    let entries = e.mem.u32(this);
    loop {
        let entry = entries.wrapping_add((index as u32).wrapping_mul(8));
        if e.mem.u32(entry) == wanted {
            hk_serialize_multi_map_remove_by_index(e, this, key, index);
            return;
        }
        index = e.mem.i32(entry + 4);
        if index == -1 {
            return;
        }
    }
}

// Translated from 00dc9030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<hkDataClassDict*,hkDataClassDict*>::removeKey` (Xbox PDB): looks `key`
/// up in the index map (`+0xc`, `findKey`), reads the first entry of its chain, removes the key
/// and, when the chain is not empty (`-1`), appends the whole chain to the free list: the last
/// entry's next index becomes the free head (`+0x18`) and the free head becomes the first
/// entry.
pub fn hk_serialize_multi_map_remove_key(e: &mut Engine, this: u32, key: u32) {
    let map = this + multi_map::MAP;
    let found = e.call(POINTER_MAP_FIND_KEY, &args![map, key]).u32();
    let table = e.mem.u32(map);
    let first = e.mem.i32(table.wrapping_add(found.wrapping_mul(8)) + 4);
    e.call(POINTER_MAP_REMOVE, &args![map, found]);
    if first == -1 {
        return;
    }
    let entries = e.mem.u32(this);
    let mut last = first;
    while e
        .mem
        .i32(entries.wrapping_add((last as u32).wrapping_mul(8)) + 4)
        != -1
    {
        last = e
            .mem
            .i32(entries.wrapping_add((last as u32).wrapping_mul(8)) + 4);
    }
    let free_head = e.mem.u32(this + multi_map::FREE_HEAD);
    e.mem.set_u32(
        entries.wrapping_add((last as u32).wrapping_mul(8)) + 4,
        free_head,
    );
    e.mem.set_i32(this + multi_map::FREE_HEAD, first);
}

// Translated from 00dc90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::selfDestruct` (Xbox PDB): for every value record (0x10 bytes) the
/// member info of its name is looked up in the class (and its parents); when there is one, the
/// value (`record + 8`, one entry) is cleared by `Union::clearArray` with the info's type
/// (`+0x10`). Then the member count is set to 0.
pub fn hk_data_object_dict_self_destruct(e: &mut Engine, this: Ptr<HkDataObjectDict>) {
    let mut index = 0;
    while index < e.get(this, HkDataObjectDict::member_count) {
        let records = e.get(this, HkDataObjectDict::members).addr();
        let record = records.wrapping_add((index as u32).wrapping_mul(0x10));
        let class = e.get(this, HkDataObjectDict::class);
        let name = e.mem.u32(record);
        let info = hk_data_class_dict_get_member_info_by_name(e, class, name);
        if info != 0 {
            let member_type = e.mem.u32(info + 0x10);
            union_clear_array(e, member_type, record + 8, 1);
        }
        index += 1;
    }
    e.set(this, HkDataObjectDict::member_count, 0);
}

// Translated from 00dc90f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::removeMember` (Xbox PDB): removes the value record whose name word equals
/// `name`: its value is cleared like in `selfDestruct` (member info of the name in the class),
/// the count drops by one and, unless the record was the last, the last record (16 bytes)
/// moves into its place.
pub fn hk_data_object_dict_remove_member(e: &mut Engine, this: Ptr<HkDataObjectDict>, name: u32) {
    let count = e.get(this, HkDataObjectDict::member_count);
    if count <= 0 {
        return;
    }
    let records = e.get(this, HkDataObjectDict::members).addr();
    let mut index = 0i32;
    let mut record = records;
    while e.mem.u32(record) != name {
        index += 1;
        record = record.wrapping_add(0x10);
        if index >= count {
            return;
        }
    }
    let class = e.get(this, HkDataObjectDict::class);
    let offset = (index as u32).wrapping_mul(0x10);
    let record_name = e.mem.u32(offset.wrapping_add(records));
    let info = hk_data_class_dict_get_member_info_by_name(e, class, record_name);
    if info != 0 {
        let member_type = e.mem.u32(info + 0x10);
        union_clear_array(e, member_type, offset.wrapping_add(records) + 8, 1);
    }
    let remaining = e.get(this, HkDataObjectDict::member_count).wrapping_sub(1);
    e.set(this, HkDataObjectDict::member_count, remaining);
    if remaining != index {
        let data = e.get(this, HkDataObjectDict::members).addr();
        let destination = data.wrapping_add(offset);
        let source = (remaining as u32).wrapping_mul(0x10).wrapping_add(data);
        for half in 0..2 {
            let low = e.mem.u32(source + 8 * half);
            let high = e.mem.u32(source + 8 * half + 4);
            e.mem.set_u32(destination + 8 * half, low);
            e.mem.set_u32(destination + 8 * half + 4, high);
        }
    }
}

// Translated from 00dc9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::getTrackedStructArrays` (Xbox PDB): appends to the
/// `hkArray` `out` every struct array of `class` (the chain of `class` in the array multi-map
/// `+0x6c`); with `include_derived` nonzero it then does the same for every class of
/// `class`'s derived-class chain (`+0x10`), recursively.
pub fn object_tracker_get_tracked_struct_arrays(
    e: &mut Engine,
    this: u32,
    class: u32,
    include_derived: u32,
    out: u32,
) {
    let array_map = this + tracker::ARRAY_MAP;
    let mut index = fn_00dc8e30(e, array_map, class);
    while index != -1 {
        let entries = e.mem.u32(array_map);
        let entry = entries.wrapping_add((index as u32).wrapping_mul(8));
        let array = e.mem.u32(entry);
        array_push_word(e, out, array);
        let entries = e.mem.u32(array_map);
        index = e
            .mem
            .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
    }
    if include_derived != 0 {
        let derived_map = this + tracker::DERIVED_MAP;
        let mut index = fn_00dc8e30(e, derived_map, class);
        while index != -1 {
            let entries = e.mem.u32(derived_map);
            let derived = e
                .mem
                .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
            object_tracker_get_tracked_struct_arrays(e, this, derived, include_derived, out);
            let entries = e.mem.u32(derived_map);
            index = e
                .mem
                .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        }
    }
}

// Translated from 00dc9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::findTrackedClasses` (Xbox PDB): appends every value of the
/// class map (`this` is the string map at the start of the tracker) to the `hkArray` `out`.
pub fn object_tracker_find_tracked_classes(e: &mut Engine, this: u32, out: u32) {
    string_map_for_each(e, this, |e, class| {
        array_push_word(e, out, class);
    });
}

// Translated from 00dc92d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::findTrackedObjectsByBase` (Xbox PDB): appends to the
/// `hkArray` `out` the objects found for the class registered under `class_name` (nothing for
/// a null or unknown name): the objects of exactly that class (the chain in the object
/// multi-map `+0x30`); with `include_arrays` nonzero, every element (slot `0x50`, for indices
/// below the array's size from slot `0x14`) of the arrays of the class (the array multi-map
/// `+0x6c`); with `include_derived` nonzero, the same search by name (slot `8` of the derived
/// class) for each class of the derived-class chain (`+0x10`), recursively.
pub fn object_tracker_find_tracked_objects_by_base(
    e: &mut Engine,
    this: u32,
    class_name: u32,
    include_derived: u32,
    include_arrays: u32,
    out: u32,
) {
    if class_name == 0 {
        return;
    }
    let class = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![this + tracker::CLASS_MAP, class_name, 0u32],
        )
        .u32();
    if class == 0 {
        return;
    }
    let object_map = this + tracker::OBJECT_MAP;
    let mut index = fn_00dc8e30(e, object_map, class);
    while index != -1 {
        let entries = e.mem.u32(object_map);
        let object = e
            .mem
            .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
        array_push_word(e, out, object);
        let entries = e.mem.u32(object_map);
        index = e
            .mem
            .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
    }
    if include_arrays != 0 {
        let array_map = this + tracker::ARRAY_MAP;
        let mut index = fn_00dc8e30(e, array_map, class);
        while index != -1 {
            let entries = e.mem.u32(array_map);
            let array = e
                .mem
                .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
            let mut element = 0i32;
            let mut size = e.vcall(array, 0x14, &args![]).i32();
            while element < size {
                let object = e.vcall(array, 0x50, &args![element]).u32();
                array_push_word(e, out, object);
                element += 1;
                size = e.vcall(array, 0x14, &args![]).i32();
            }
            let entries = e.mem.u32(array_map);
            index = e
                .mem
                .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        }
    }
    if include_derived != 0 {
        let derived_map = this + tracker::DERIVED_MAP;
        let mut index = fn_00dc8e30(e, derived_map, class);
        while index != -1 {
            let entries = e.mem.u32(derived_map);
            let derived = e
                .mem
                .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
            let derived_name = e.vcall(derived, 8, &args![]).u32();
            object_tracker_find_tracked_objects_by_base(
                e,
                this,
                derived_name,
                include_derived,
                include_arrays,
                out,
            );
            let entries = e.mem.u32(derived_map);
            index = e
                .mem
                .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        }
    }
}

// Translated from 00dc9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::removeMember` (Xbox PDB): removes the member record (0x18 bytes) whose name
/// word equals `name`, searching from the last record. With no match the index is `-1` and the
/// exe works on the record 0x18 bytes before the list, and so does this. The record's class
/// handle (`+4`) loses a reference and is cleared, the count drops by one and the later
/// records move down by one (a chunk of 8 bytes at a time). Returns whether the class (or a
/// parent) still has a member of that name.
pub fn hk_data_class_dict_remove_member(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    name: u32,
) -> bool {
    let members = e.get(this, HkDataClassDict::members).addr();
    let mut index = e.get(this, HkDataClassDict::member_count).wrapping_sub(1);
    let mut found = -1;
    if index >= 0 {
        let mut record = members.wrapping_add((index as u32).wrapping_mul(0x18));
        loop {
            if e.mem.u32(record) == name {
                found = index;
                break;
            }
            index -= 1;
            record = record.wrapping_sub(0x18);
            if index < 0 {
                break;
            }
        }
    }
    let members = e.get(this, HkDataClassDict::members).addr();
    let offset = (found as u32).wrapping_mul(0x18);
    let record = members.wrapping_add(offset);
    let handle = e.mem.u32(record + 4);
    if handle != 0 {
        e.call(REMOVE_REFERENCE, &args![handle]);
    }
    e.mem.set_u32(record + 4, 0);
    let remaining = e.get(this, HkDataClassDict::member_count).wrapping_sub(1);
    e.set(this, HkDataClassDict::member_count, remaining);
    let bytes = remaining.wrapping_sub(found).wrapping_mul(0x18);
    let data = e.get(this, HkDataClassDict::members).addr();
    let mut destination = data.wrapping_add(offset);
    if bytes > 0 {
        let mut chunks = ((bytes as u32).wrapping_sub(1) >> 3).wrapping_add(1);
        while chunks != 0 {
            let low = e.mem.u32(destination.wrapping_add(0x18));
            let high = e.mem.u32(destination.wrapping_add(0x1c));
            e.mem.set_u32(destination, low);
            e.mem.set_u32(destination + 4, high);
            destination = destination.wrapping_add(8);
            chunks -= 1;
        }
    }
    hk_data_class_dict_get_member_info_by_name(e, this, name) != 0
}

// Translated from 00dc9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this body `hkDataWorldDict::ObjectTracker::intern`, but it reads the tracker
/// through `this + 0xc` (a world) and then `+8`, so `this` is a class dict whose `+0xc` is its
/// world (`StructArrayImplementation::Object::intern` calls it with the array's class): stores
/// the interned copy of `name` (null stays null) at `out` and returns `out`.
pub fn fn_00dc9570(e: &mut Engine, this: Ptr<HkDataClassDict>, out: u32, name: u32) -> u32 {
    let world = e.get(this, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    let interned = intern_name(e, tracker_address, name);
    e.mem.set_u32(out, interned);
    out
}

// Translated from 00dc95d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::hasMember` (Xbox PDB): interns `name` through the tracker of the
/// class's world (null stays null; the copy is made even when no record matches) and answers
/// whether a value record has exactly that name word.
pub fn hk_data_object_dict_has_member(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
) -> bool {
    let class = e.get(this, HkDataObjectDict::class);
    let world = e.get(class, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    let interned = intern_name(e, tracker_address, name);
    let count = e.get(this, HkDataObjectDict::member_count);
    if count > 0 {
        let mut record = e.get(this, HkDataObjectDict::members).addr();
        let mut index = 0;
        while index < count {
            if e.mem.u32(record) == interned {
                return true;
            }
            index += 1;
            record = record.wrapping_add(0x10);
        }
    }
    false
}

// Translated from 00dc9640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::accessByName` (Xbox PDB): stores `this` at `out` and the interned
/// copy of `name` (through the tracker of the class's world; null stays null) at `out + 4`.
/// `EAX` still holds `out` on return.
pub fn hk_data_object_dict_access_by_name(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    out: u32,
    name: u32,
) -> u32 {
    let class = e.get(this, HkDataObjectDict::class);
    let world = e.get(class, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    let interned = intern_name(e, tracker_address, name);
    e.mem.set_u32(out + 4, interned);
    e.mem.set_u32(out, this.addr());
    out
}

// Translated from 00dc96a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::~hkDataObjectDict` (Xbox PDB): sets the vtable to `0x010d_6ee4`, runs
/// `selfDestruct`, frees the value-record array (16 bytes per element, unless the array does
/// not own its buffer), and drops one reference of the class (its deleting destructor runs at
/// zero). The class pointer is cleared and the vtable becomes `0x010d_62f4`.
pub fn hk_data_object_dict_destruct(e: &mut Engine, this: Ptr<HkDataObjectDict>) {
    let address = this.addr();
    e.mem.set_u32(address, 0x010d_6ee4);
    hk_data_object_dict_self_destruct(e, this);
    array_free(e, address + 0x10, 0x10);
    let class = e.get(this, HkDataObjectDict::class).addr();
    release_object_if_set(e, class);
    e.mem.set_u32(address + 0xc, 0);
    e.mem.set_u32(address, 0x010d_62f4);
}

// Translated from 00dc9710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::_writeAccess` (Xbox PDB): finds the member info of `name` in the class
/// and stores `{info, index}` at `out`. With no info it is `{0, -1}`. Otherwise an existing
/// value record of that name has its value cleared (`Union::clearArray`, one entry, the info's
/// type) and gives its index; a missing one is appended (the array grows by one 0x10-byte
/// element when full): name, value `0`, index = the old count. The third word is not read.
/// `EAX` still holds `out` on return.
pub fn hk_data_object_dict_write_access(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    out: u32,
    name: u32,
    _unused_3: u32,
) -> u32 {
    let class = e.get(this, HkDataObjectDict::class);
    let info = hk_data_class_dict_get_member_info_by_name(e, class, name);
    if info == 0 {
        e.mem.set_u32(out, 0);
        e.mem.set_i32(out + 4, -1);
        return out;
    }
    let count = e.get(this, HkDataObjectDict::member_count);
    let mut index = 0i32;
    if count > 0 {
        let records = e.get(this, HkDataObjectDict::members).addr();
        let mut record = records;
        while index < count {
            if e.mem.u32(record) == name {
                let member_type = e.mem.u32(info + 0x10);
                let value = ((index as u32) << 4).wrapping_add(records) + 8;
                union_clear_array(e, member_type, value, 1);
                e.mem.set_i32(out + 4, index);
                e.mem.set_u32(out, info);
                return out;
            }
            index += 1;
            record = record.wrapping_add(0x10);
        }
    }
    let array = this.addr() + 0x10;
    array_grow_if_full(e, array, 0x10);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let slot = (size << 4).wrapping_add(data);
    if slot != 0 {
        e.mem.set_u32(slot, 0);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let slot = (size << 4).wrapping_add(data);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(slot, name);
    e.mem.set_u32(slot + 8, 0);
    e.mem.set_u32(slot + 0xc, 0);
    e.mem.set_i32(out + 4, count);
    e.mem.set_u32(out, info);
    out
}

/// The value slot (`record + 8`) of the record `{info, index}` at `access` names, as the
/// exe computes it after `_writeAccess`: `index * 0x10 + records + 8`.
fn object_dict_value_slot(e: &Engine, this: Ptr<HkDataObjectDict>, access: u32) -> u32 {
    let index = e.mem.u32(access + 4);
    let records = e.get(this, HkDataObjectDict::members).addr();
    (index << 4).wrapping_add(records) + 8
}

// Translated from 00dc97f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign` (Xbox PDB): stores the object `object` (one more reference when
/// not null) as the value of the member `name`.
pub fn hk_data_object_dict_assign(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    object: u32,
) {
    if object != 0 {
        add_object_reference(e, object);
    }
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, 8);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.set_u32(slot, object);
    });
}

// Translated from 00dc9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov2` (Xbox PDB): stores the array `array` (one more reference)
/// as the value of the member `name`; the array's type (slot `4`) with bit `0x10` set is the
/// unread third word of `_writeAccess`.
pub fn hk_data_object_dict_assign_ov2(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    array: u32,
) {
    add_object_reference(e, array);
    let array_type = e.vcall(array, 4, &args![]).u32();
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, array_type | 0x10);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.set_u32(slot, array);
    });
}

// Translated from 00dc9880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov3` (Xbox PDB): stores a `hkString::strDup` copy of `text` (0 for
/// a null `text`) as the value of the member `name`; the old value is cleared first by
/// `_writeAccess`.
pub fn hk_data_object_dict_assign_ov3(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    text: u32,
) {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, 10);
        let copy = if text == 0 {
            0
        } else {
            e.call(STRING_DUP, &args![text]).u32()
        };
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.set_u32(slot, copy);
    });
}

// Translated from 00dc98d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov4` (Xbox PDB): stores the float `value` (through an
/// `FLD`/`FSTP` pair) as the value of the member `name`.
pub fn hk_data_object_dict_assign_ov4(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    value: f32,
) {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, 3);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.set_u32(slot, x87_quiet(value).to_bits());
    });
}

// Translated from 00dc9910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov5` (Xbox PDB): stores the sign-extended 16-bit `value` shifted
/// left by 16, as the bit pattern of a float, through an `FLD`/`FSTP` pair, as the value of the
/// member `name`.
pub fn hk_data_object_dict_assign_ov5(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    value: i16,
) {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, 3);
        let bits = (i32::from(value) as u32) << 16;
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.set_u32(slot, x87_quiet_bits(bits));
    });
}

// Translated from 00dc9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov6` (Xbox PDB): stores a copy of the `count` floats at `values`
/// (`count * 4` bytes from the router's allocator, slot `4`, filled with `memcpy`) as the value
/// of the member `name`. The unread third word of `_writeAccess` is `fn_00dc73e0(count)`.
pub fn hk_data_object_dict_assign_ov6(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    values: u32,
    count: i32,
) {
    let member_type = fn_00dc73e0(e, count as u32);
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, member_type);
        let allocator = router_allocator(e);
        let bytes = (count as u32).wrapping_mul(4);
        let copy = e.vcall(allocator, 4, &args![bytes]).u32();
        e.call(MEM_COPY, &args![copy, values, bytes]);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.set_u32(slot, copy);
    });
}

// Translated from 00dc99c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov7` (Xbox PDB): after `_writeAccess` the member info's type
/// decides what the 64-bit `value` does to the value record: types 1 and 2 store both words
/// (`+8`, `+0xc`), type 3 stores the value converted to a float (`FILD`/`FSTP`) at `+8`, other
/// types change nothing. With no member info the exe reads the type at address `0x10`.
pub fn hk_data_object_dict_assign_ov7(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    value: u64,
) {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_write_access(e, this, access.addr(), name, 0);
        let info = e.mem.u32(access.addr());
        let member_type = e.mem.i32(info + 0x10);
        let slot = object_dict_value_slot(e, this, access.addr());
        if member_type > 0 {
            if member_type <= 2 {
                e.mem.set_u32(slot, value as u32);
                e.mem.set_u32(slot + 4, (value >> 32) as u32);
            } else if member_type == 3 {
                e.mem.set_u32(slot, (value as i64 as f32).to_bits());
            }
        }
    });
}

/// `setSize` of an array of 4-byte elements at `array` as the exe inlines it: the capacity
/// grows if needed, new elements are zeroed (`REP STOSD`) and the size is stored.
fn word_array_set_size(e: &mut Engine, array: u32, size: i32) {
    array_reserve(e, array, size, 4);
    let old_size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let added = size.wrapping_sub(old_size);
    if added > 0 {
        let mut destination = data.wrapping_add((old_size as u32).wrapping_mul(4));
        for _ in 0..added {
            e.mem.set_u32(destination, 0);
            destination = destination.wrapping_add(4);
        }
    }
    e.mem.set_i32(array + 4, size);
}

// Translated from 00dc9b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::setSize` (Xbox PDB): sets the element count to `stride * count`,
/// growing the array and zeroing the new elements.
pub fn vec_array_implementation_set_size(e: &mut Engine, this: u32, count: i32) {
    let size = e.mem.i32(this + 0x18).wrapping_mul(count);
    word_array_set_size(e, this + 0xc, size);
}

// Translated from 00dc9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<hkDataObjectImpl*,8>::setSize` (Xbox PDB): sets the element count
/// of the 4-byte element array at `+0xc` to `size`, growing it and zeroing the new elements.
pub fn basic_array_implementation_object_set_size(e: &mut Engine, this: u32, size: i32) {
    word_array_set_size(e, this + 0xc, size);
}

// Translated from 00dc9c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::_addMember` (Xbox PDB): appends the 8-byte record `{name,
/// array}` for the member description `description` (`+0` name, `+4` a record whose `+8` is the
/// class name, `+0x10` type, `+0x14` extra) and returns its index; a zero type returns `-1`
/// without changing anything. The class is looked up by the name (slot `4` of the array's class
/// at `+0x18` gives the object whose `+8` is the tracker, whose class map answers; a null name
/// gives 0), the array is made by `hkDataArrayDict_create(type, class, extra)` and gains a
/// reference.
pub fn struct_array_implementation_add_member_impl(
    e: &mut Engine,
    this: u32,
    description: u32,
) -> i32 {
    if e.mem.u32(description + 0x10) == 0 {
        return -1;
    }
    let array = this + 0xc;
    array_grow_if_full(e, array, 8);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let slot = data.wrapping_add(size.wrapping_mul(8));
    if slot != 0 {
        e.mem.set_u32(slot, 0);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let record = data.wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let name = e.mem.u32(description);
    e.mem.set_u32(record, name);
    let class = e.mem.u32(this + 0x18);
    let class_name = e.mem.u32(e.mem.u32(description + 4) + 8);
    let owner = e.vcall(class, 4, &args![]).u32();
    let tracker_address = e.mem.u32(owner + 8);
    let class_impl = if class_name == 0 {
        0
    } else {
        e.call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![tracker_address, class_name, 0u32],
        )
        .u32()
    };
    let member_type = e.mem.u32(description + 0x10);
    let extra = e.mem.u32(description + 0x14);
    let new_array = hk_data_array_dict_create(e, member_type, class_impl, extra);
    e.mem.set_u32(record + 4, new_array);
    add_object_reference(e, new_array);
    e.mem.i32(this + 0x10).wrapping_sub(1)
}

// Translated from 00dc9d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<unsigned char,1>::setSize` (Xbox PDB): sets the byte count of the
/// array at `+0xc` to `size`, growing it; new bytes are zeroed with `memset`.
pub fn basic_array_implementation_u8_set_size(e: &mut Engine, this: u32, size: i32) {
    let array = this + 0xc;
    array_reserve(e, array, size, 1);
    let old_size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let added = size.wrapping_sub(old_size);
    if added > 0 {
        e.call(
            MEM_SET,
            &args![data.wrapping_add(old_size as u32), 0u32, added],
        );
    }
    e.mem.set_i32(array + 4, size);
}

// Translated from 00dc9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<__int64,2>::setSize` (Xbox PDB): sets the element count of the
/// 8-byte element array at `+0xc` to `size`, growing it. The new elements are zeroed the way
/// the exe does it: the first one is stored, then `(8 * added - 5) >> 2` words are copied
/// forward from it to eight bytes after it (`REP MOVSD`), which spreads the zeros.
pub fn basic_array_implementation_i64_set_size(e: &mut Engine, this: u32, size: i32) {
    let array = this + 0xc;
    array_reserve(e, array, size, 8);
    let old_size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let added = size.wrapping_sub(old_size);
    let first = data.wrapping_add((old_size as u32).wrapping_mul(8));
    if added > 0 {
        let words = (added.wrapping_mul(8).wrapping_sub(5) as u32) >> 2;
        e.mem.set_u32(first, 0);
        e.mem.set_u32(first + 4, 0);
        let mut source = first;
        let mut destination = first.wrapping_add(8);
        for _ in 0..words {
            let word = e.mem.u32(source);
            e.mem.set_u32(destination, word);
            source = source.wrapping_add(4);
            destination = destination.wrapping_add(4);
        }
    }
    e.mem.set_i32(array + 4, size);
}

// Translated from 00dc9f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::addMember` (Xbox PDB; four stack words): appends a 0x18-byte member record
/// `{name, class handle, 0, 0, member_type, extra}` to the member array at `+0x28` (growing it
/// by 0x18-byte elements when full). The handle is the word held by the handle at `handle`:
/// a temporary reference holder takes it (the static default handle `0x011b_7c00` is
/// referenced first and released again) and the record takes one reference of its own
/// when not null; the temporary is released at the end.
pub fn hk_data_class_dict_add_member(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    name: u32,
    member_type: u32,
    handle: u32,
    extra: u32,
) {
    e.call(ADD_REFERENCE, &args![DEFAULT_CLASS_HANDLE]);
    let held = e.mem.u32(handle);
    if held != 0 {
        e.call(ADD_REFERENCE, &args![held]);
    }
    e.call(REMOVE_REFERENCE, &args![DEFAULT_CLASS_HANDLE]);
    let held = e.mem.u32(handle);
    let array = this.addr() + 0x28;
    array_grow_if_full(e, array, 0x18);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let record = data.wrapping_add(size.wrapping_mul(0x18));
    if record != 0 {
        e.mem.set_u32(record, name);
        if held != 0 {
            e.call(ADD_REFERENCE, &args![held]);
        }
        e.mem.set_u32(record + 4, held);
        e.mem.set_u32(record + 8, 0);
        e.mem.set_u32(record + 0xc, 0);
        e.mem.set_u32(record + 0x10, member_type);
        e.mem.set_u32(record + 0x14, extra);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
}

/// The loop the exe repeats in `IntArrayImplementation::setAll` (overloads 4 to 7): reserves
/// `count` 8-byte elements, stores `count` as the size and writes each element as the
/// `(low, high)` words `read` makes of the source element `index`.
fn int_array_fill(
    e: &mut Engine,
    this: u32,
    count: i32,
    read: impl Fn(&Engine, i32) -> (u32, u32),
) {
    let array = this + 0xc;
    array_reserve(e, array, count, 8);
    e.mem.set_i32(array + 4, count);
    let mut index = 0i32;
    while index < count {
        let (low, high) = read(e, index);
        let data = e.mem.u32(array);
        let element = data.wrapping_add((index as u32).wrapping_mul(8));
        e.mem.set_u32(element, low);
        e.mem.set_u32(element + 4, high);
        index += 1;
    }
}

// Translated from 00dca420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov4` (Xbox PDB): sets the array to the `count` 16-bit values
/// at `source` read as signed, sign-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov4(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_fill(e, this, count, |e, index| {
        let value = i32::from(
            e.mem
                .i16(source.wrapping_add((index as u32).wrapping_mul(2))),
        );
        (value as u32, (value >> 31) as u32)
    });
}

// Translated from 00dca480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov5` (Xbox PDB): sets the array to the `count` 16-bit values
/// at `source` read as unsigned, zero-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov5(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_fill(e, this, count, |e, index| {
        let value = u32::from(
            e.mem
                .u16(source.wrapping_add((index as u32).wrapping_mul(2))),
        );
        (value, 0)
    });
}

// Translated from 00dca4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov6` (Xbox PDB): sets the array to the `count` 32-bit values
/// at `source` read as signed, sign-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov6(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_fill(e, this, count, |e, index| {
        let value = e
            .mem
            .i32(source.wrapping_add((index as u32).wrapping_mul(4)));
        (value as u32, (value >> 31) as u32)
    });
}

// Translated from 00dca540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov7` (Xbox PDB): sets the array from the `count` 64-bit values
/// at `source`, of which only the low 32 bits are read, sign-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov7(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_fill(e, this, count, |e, index| {
        let value = e
            .mem
            .i32(source.wrapping_add((index as u32).wrapping_mul(8)));
        (value as u32, (value >> 31) as u32)
    });
}

// Translated from 00dca5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `RealArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_71ac` has `RealArrayImplementation::asReal` and `setReal` in its slots): `+6` 0,
/// reference count `+8` 0, an empty element array (`+0xc` data 0, `+0x10` size 0, `+0x14`
/// capacity word `0x8000_0000`) and the vtable. Returns `this`.
pub fn real_array_implementation_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_71ac);
    this
}

// Translated from 00dca5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RealArrayImplementation::asReal` (Xbox PDB): the float at `index`, loaded with `FLD` and
/// handed back in `ST0` (hence the `f64` result).
pub fn real_array_implementation_as_real(e: &mut Engine, this: u32, index: i32) -> f64 {
    let data = e.mem.u32(this + 0xc);
    let bits = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
    f64::from(x87_quiet(f32::from_bits(bits)))
}

// Translated from 00dca5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RealArrayImplementation::setReal` (Xbox PDB): stores the float `value`, through an
/// `FLD`/`FSTP` pair, at `index`.
pub fn real_array_implementation_set_real(e: &mut Engine, this: u32, index: i32, value: f32) {
    let data = e.mem.u32(this + 0xc);
    e.mem.set_u32(
        data.wrapping_add((index as u32).wrapping_mul(4)),
        x87_quiet(value).to_bits(),
    );
}

// Translated from 00dca5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RealArrayImplementation::setAll` (Xbox PDB): reserves `count` floats, stores `count` as the
/// size and copies `count * 4` bytes from `source` with `memcpy`.
pub fn real_array_implementation_set_all(e: &mut Engine, this: u32, source: u32, count: i32) {
    let array = this + 0xc;
    array_reserve(e, array, count, 4);
    let bytes = (count as u32).wrapping_mul(4);
    e.mem.set_i32(array + 4, count);
    let data = e.mem.u32(array);
    e.call(MEM_COPY, &args![data, source, bytes]);
}

// Translated from 00dca650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `CstringArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_7244` has `CstringArrayImplementation::setString` in its slot `0x2c`): `+6` 0,
/// reference count `+8` 0, an empty element array (`+0xc` data 0, `+0x10` size 0, `+0x14`
/// capacity word `0x8000_0000`) and the vtable. Returns `this`.
pub fn cstring_array_implementation_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_7244);
    this
}

// Translated from 00dca670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CstringArrayImplementation::setString` (Xbox PDB): frees the string at `index`
/// (`hkString::strFree`) and stores a `hkString::strDup` copy of `text` (0 for a null `text`).
pub fn cstring_array_implementation_set_string(e: &mut Engine, this: u32, index: i32, text: u32) {
    let data = e.mem.u32(this + 0xc);
    let old = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
    e.call(STRING_FREE, &args![old]);
    let copy = if text != 0 {
        e.call(STRING_DUP, &args![text]).u32()
    } else {
        0
    };
    let data = e.mem.u32(this + 0xc);
    e.mem
        .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), copy);
}

// Translated from 00dca6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CstringArrayImplementation::~CstringArrayImplementation` (Xbox PDB): sets the vtable to
/// `0x010d_7244`, frees every string (`hkString::strFree`), frees the element array (4 bytes per
/// element, unless it does not own its buffer) and puts the base vtable `0x010d_62f4`.
pub fn cstring_array_implementation_destruct(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010d_7244);
    let mut index = 0i32;
    while index < e.mem.i32(this + 0x10) {
        let data = e.mem.u32(this + 0xc);
        let text = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        e.call(STRING_FREE, &args![text]);
        index += 1;
    }
    array_free(e, this + 0xc, 4);
    e.mem.set_u32(this, 0x010d_62f4);
}

/// The tail the deleting destructors of the array implementations share: the element array
/// (`element_size` bytes per element) is freed, the base vtable `0x010d_62f4` is put in and,
/// when bit 0 of `flags` is set, the block goes back to the router's allocator (slot `8`) with
/// the 16-bit size kept at `+4`.
fn array_implementation_deleting_tail(e: &mut Engine, this: u32, flags: u8, element_size: u32) {
    array_free(e, this + 0xc, element_size);
    e.mem.set_u32(this, 0x010d_62f4);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(this + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
}

// Translated from 00dca750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::_scalar_deleting_destructor_` (Xbox PDB): frees the byte array
/// (one byte per element), puts the base vtable `0x010d_62f4` in and, when bit 0 of `flags` is
/// set, returns the block to the router's allocator (slot `8`) with the 16-bit size kept at
/// `+4`. Returns `this`.
pub fn byte_array_implementation_deleting_destructor(e: &mut Engine, this: u32, flags: u8) -> u32 {
    array_implementation_deleting_tail(e, this, flags, 1);
    this
}

// Translated from 00dca7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::_scalar_deleting_destructor_` (Xbox PDB): frees the element array
/// (eight bytes per element), puts the base vtable `0x010d_62f4` in and, when bit 0 of `flags`
/// is set, returns the block to the router's allocator (slot `8`) with the 16-bit size kept at
/// `+4`. Returns `this`.
pub fn int_array_implementation_deleting_destructor(e: &mut Engine, this: u32, flags: u8) -> u32 {
    array_implementation_deleting_tail(e, this, flags, 8);
    this
}

// Translated from 00dca820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::trackClass` (Xbox PDB): asks the class for its parent
/// (slot `0x10`) and its name (slot `8`), and when the class map has no entry for that name
/// yet, inserts the class under the name (asked again) and, when the class has a parent, puts
/// the class in the derived-class multi-map (`+0x10`) under the parent.
pub fn object_tracker_track_class(e: &mut Engine, this: u32, class: u32) {
    let parent = e.vcall(class, 0x10, &args![]).u32();
    let name = e.vcall(class, 8, &args![]).u32();
    let iterator = e.call(STRING_MAP_FIND_ITERATOR, &args![this, name]).u32();
    if !string_map_is_valid(e, this, iterator) {
        let name = e.vcall(class, 8, &args![]).u32();
        e.call(STRING_MAP_INSERT, &args![this, name, class]);
        if parent != 0 {
            e.with_stack(4, |e, held| {
                e.mem.set_u32(held.addr(), class);
                e.call(
                    MULTI_MAP_INSERT,
                    &args![this + tracker::DERIVED_MAP, parent, held.addr()],
                );
            });
        }
    }
}

// Translated from 00dca890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of the object tracker (the map gives it only a range; it is the part of
/// `untrackClass` that destroys the objects of a class): first the same for every class of the
/// derived-class chain of `class` (`+0x10`, recursively). Then the objects of the class
/// registered under `class`'s name (the chain of that class in the object multi-map `+0x30`)
/// are collected in an `hkLocalArray` (0x200 elements from the router's stack allocator); the
/// tracker's contents pointer (`+0x2c`) is cleared when it is one of them, each with a
/// positive external count is reported like in the tracker's teardown (id `0x760d_3794`,
/// source line `0x7b9`), each self-destructs and loses one reference, and the class's key is
/// removed from the object multi-map. The stack allocator is given back and the array's
/// buffer freed unless it is the local memory. Without objects nothing of that happens.
pub fn fn_00dca890(e: &mut Engine, this: u32, class: u32) {
    let name = e.vcall(class, 8, &args![]).u32();
    let registered = if name != 0 {
        e.call(STRING_MAP_GET_WITH_DEFAULT, &args![this, name, 0u32])
            .u32()
    } else {
        0
    };
    let derived_map = this + tracker::DERIVED_MAP;
    let mut index = fn_00dc8e30(e, derived_map, class);
    while index != -1 {
        let entries = e.mem.u32(derived_map);
        let child = e
            .mem
            .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
        fn_00dca890(e, this, child);
        let entries = e.mem.u32(derived_map);
        index = e
            .mem
            .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
    }
    let object_map = this + tracker::OBJECT_MAP;
    let mut index = fn_00dc8e30(e, object_map, registered);
    if index == -1 {
        return;
    }
    e.with_stack(0x10, |e, block| {
        let objects = block.addr();
        array_init_empty(e, objects);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let base = e.mem.u32(router);
        e.mem.set_u32(router, base.wrapping_add(0x800));
        e.mem.set_u32(objects, base);
        e.mem.set_u32(objects + 8, 0x8000_0200);
        e.mem.set_u32(objects + 0xc, base);
        loop {
            let entries = e.mem.u32(object_map);
            let object = e
                .mem
                .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
            array_push_word(e, objects, object);
            let entries = e.mem.u32(object_map);
            index = e
                .mem
                .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            if index == -1 {
                break;
            }
        }
        let mut i = 0;
        while i < e.mem.i32(objects + 4) {
            let object = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add((i as u32).wrapping_mul(4)));
            if e.mem.u32(this + 0x2c) == object {
                e.mem.set_u32(this + 0x2c, 0);
            }
            if e.mem.i16(object + 6) > 0 {
                tracker_report_referenced_object(e, object, 0x760d_3794, 0x7b9);
            }
            hk_data_object_dict_self_destruct(e, Ptr::new(object));
            i += 1;
        }
        let mut i = 0;
        while i < e.mem.i32(objects + 4) {
            let object = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add((i as u32).wrapping_mul(4)));
            release_object(e, object);
            i += 1;
        }
        hk_serialize_multi_map_remove_key(e, object_map, registered);
        if base == e.mem.u32(objects) {
            e.mem.set_u32(objects + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, base);
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(objects + 8);
        e.mem.set_u32(objects + 4, 0);
        if capacity & 0x8000_0000 == 0 {
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            let data = e.mem.u32(objects);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00dcaaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of the object tracker (the map gives it only a range; `setClassParent` calls
/// it): takes `class` out of the derived-class chain of `old_parent` (the first entry of that
/// chain whose value is `class`, removed by index) when `old_parent` is not null, then, when
/// the class's parent (slot `0x10`) is not null, puts `class` in the derived-class multi-map
/// (`+0x10`) under that parent.
pub fn fn_00dcaaf0(e: &mut Engine, this: u32, old_parent: u32, class: u32) {
    let derived_map = this + tracker::DERIVED_MAP;
    if old_parent != 0 {
        let mut index = fn_00dc8e30(e, derived_map, old_parent);
        if index != -1 {
            let entries = e.mem.u32(derived_map);
            loop {
                let entry = entries.wrapping_add((index as u32).wrapping_mul(8));
                if e.mem.u32(entry) == class {
                    hk_serialize_multi_map_remove_by_index(e, derived_map, old_parent, index);
                    break;
                }
                index = e.mem.i32(entry + 4);
                if index == -1 {
                    break;
                }
            }
        }
    }
    let parent = e.vcall(class, 0x10, &args![]).u32();
    if parent != 0 {
        e.with_stack(4, |e, held| {
            e.mem.set_u32(held.addr(), class);
            e.call(MULTI_MAP_INSERT, &args![derived_map, parent, held.addr()]);
        });
    }
}

// Translated from 00dcab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::retractCastedObject` (Xbox PDB): asks the object for its
/// class (slot `8`) and takes the object out of that class's chain in the object multi-map
/// (`+0x30`, the first entry whose value is the object, removed by index), and puts the object
/// in the chain of `new_class` (`hkSerializeMultiMap::insert`). A null old class does nothing.
pub fn object_tracker_retract_casted_object(
    e: &mut Engine,
    this: u32,
    object: u32,
    new_class: u32,
) {
    let mut old_class = e.vcall(object, 8, &args![]).u32();
    let object_map = this + tracker::OBJECT_MAP;
    let mut index = fn_00dc8e30(e, object_map, old_class);
    if old_class == 0 {
        return;
    }
    while index != -1 {
        let entries = e.mem.u32(object_map);
        if e.mem
            .u32(entries.wrapping_add((index as u32).wrapping_mul(8)))
            == object
        {
            hk_serialize_multi_map_remove_by_index(e, object_map, old_class, index);
            e.with_stack(4, |e, held| {
                e.mem.set_u32(held.addr(), object);
                e.call(MULTI_MAP_INSERT, &args![object_map, new_class, held.addr()]);
            });
            old_class = 0;
        }
        let entries = e.mem.u32(object_map);
        index = e
            .mem
            .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        if old_class == 0 {
            break;
        }
    }
}

// Translated from 00dcabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::hkDataClassDict` (Xbox PDB): `+6` 0, reference count `+8` 0, vtable
/// `0x010d_73c8`, the world at `+0xc`, the class-name handle at `+0x10` (made by
/// `internClassNameHandle` of the world's tracker), `version` at `+0x14`, no parent (`+0x18`)
/// and three empty arrays (`+0x1c`, `+0x28`, `+0x34`: data 0, size 0, capacity word
/// `0x8000_0000`). Returns `this`.
pub fn hk_data_class_dict_construct(
    e: &mut Engine,
    this: u32,
    world: u32,
    name: u32,
    version: u32,
) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this, 0x010d_73c8);
    e.mem.set_u32(this + 0xc, world);
    let tracker_address = e.mem.u32(world + 8);
    object_tracker_intern_class_name_handle(e, tracker_address, this + 0x10, name);
    e.mem.set_u32(this + 0x14, version);
    e.mem.set_u32(this + 0x18, 0);
    for array in [0x1cu32, 0x28, 0x34] {
        e.mem.set_u32(this + array, 0);
        e.mem.set_u32(this + array + 4, 0);
        e.mem.set_u32(this + array + 8, 0x8000_0000);
    }
    this
}

// Translated from 00dcac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getName` (Xbox PDB): the name in the class-name handle at `+0x10`
/// (its field `+8`).
pub fn hk_data_class_dict_get_name(e: &mut Engine, this: Ptr<HkDataClassDict>) -> u32 {
    // The class-name handle `+0x10` is not in the class's layout above.
    let handle = e.mem.u32(this.addr() + 0x10);
    e.mem.u32(handle + 8)
}

// Translated from 00dcac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getDeclaredMemberInfo` (Xbox PDB): fills the 0x14-byte member info at
/// `out` from the declared member `index` of this class: the name (`+0`), the class itself
/// (`+4`), the type (`+8`), the class implementation registered under the member's class
/// name (`+0xc`, from the world's tracker; 0 for a null name) and the extra word (`+0x10`).
pub fn hk_data_class_dict_get_declared_member_info(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    index: i32,
    out: u32,
) {
    let members = e.get(this, HkDataClassDict::members).addr();
    let record = members.wrapping_add((index as u32).wrapping_mul(0x18));
    let name = e.mem.u32(record);
    e.mem.set_u32(out, name);
    e.mem.set_u32(out + 4, this.addr());
    let handle = e.mem.u32(record + 4);
    let class_name = e.mem.u32(handle + 8);
    let world = e.get(this, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    if class_name != 0 {
        let class = e
            .call(
                STRING_MAP_GET_WITH_DEFAULT,
                &args![tracker_address, class_name, 0u32],
            )
            .u32();
        e.mem.set_u32(out + 0xc, class);
    } else {
        e.mem.set_u32(out + 0xc, 0);
    }
    let member_type = e.mem.u32(record + 0x10);
    e.mem.set_u32(out + 8, member_type);
    let extra = e.mem.u32(record + 0x14);
    e.mem.set_u32(out + 0x10, extra);
}

// Translated from 00dcac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::isSuperClass` (Xbox PDB): stores at `out` (one byte) whether this class
/// is `other` or one of its ancestors: for `other` and each parent (slot `0x10`) the name (slot
/// `8`) is interned through the world's tracker (null stays null) and compared with this
/// class's own name (the field `+8` of the name handle at `+0x10`). A null `other` gives 0.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hk_data_class_dict_is_super_class(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    out: u32,
    other: u32,
) -> u32 {
    let mut candidate = other;
    while candidate != 0 {
        let name = e.vcall(candidate, 8, &args![]).u32();
        let world = e.get(this, HkDataClassDict::world);
        let tracker_address = world_tracker(e, world);
        let interned = intern_name(e, tracker_address, name);
        let own_handle = e.mem.u32(this.addr() + 0x10);
        if interned == e.mem.u32(own_handle + 8) {
            e.mem.set_u8(out, 1);
            return out;
        }
        candidate = e.vcall(candidate, 0x10, &args![]).u32();
    }
    e.mem.set_u8(out, 0);
    out
}

// Translated from 00dcad10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getDeclaredMemberIndexByName` (Xbox PDB): interns `name` through the
/// world's tracker (null stays null) and gives the index of the first declared member record
/// with that name word, or `-1`.
pub fn hk_data_class_dict_get_declared_member_index_by_name(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    name: u32,
) -> i32 {
    let world = e.get(this, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    let interned = intern_name(e, tracker_address, name);
    let count = e.get(this, HkDataClassDict::member_count);
    let mut index = 0;
    if count > 0 {
        let mut record = e.get(this, HkDataClassDict::members).addr();
        while index < count {
            if e.mem.u32(record) == interned {
                return index;
            }
            index += 1;
            record = record.wrapping_add(0x18);
        }
    }
    -1
}

// Translated from 00dcad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getNumMembers` (Xbox PDB): the declared member count of this class plus
/// those of every ancestor.
pub fn hk_data_class_dict_get_num_members(e: &mut Engine, this: Ptr<HkDataClassDict>) -> i32 {
    let mut total = e.get(this, HkDataClassDict::member_count);
    let mut parent = e.get(this, HkDataClassDict::parent);
    while !parent.is_null() {
        total = total.wrapping_add(e.get(parent, HkDataClassDict::member_count));
        parent = e.get(parent, HkDataClassDict::parent);
    }
    total
}

// Translated from 00dcada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getMemberIndexByName` (Xbox PDB): interns `name` through the world's
/// tracker (null stays null) and searches the class, then its ancestors, for a declared member
/// record with that name word. The index counts the ancestors' members first: the position in
/// the class that has it, minus that class's declared count (slot `0x18`) plus its total
/// (slot `0x24`). `-1` when no class has it.
pub fn hk_data_class_dict_get_member_index_by_name(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    name: u32,
) -> i32 {
    let world = e.get(this, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    let interned = intern_name(e, tracker_address, name);
    let mut class = this;
    loop {
        let count = e.get(class, HkDataClassDict::member_count);
        let mut index = 0;
        if count > 0 {
            let mut record = e.get(class, HkDataClassDict::members).addr();
            while index < count {
                if e.mem.u32(record) == interned {
                    let declared = e.vcall(class.addr(), 0x18, &args![]).i32();
                    let total = e.vcall(class.addr(), 0x24, &args![]).i32();
                    return index.wrapping_sub(declared).wrapping_add(total);
                }
                index += 1;
                record = record.wrapping_add(0x18);
            }
        }
        class = e.get(class, HkDataClassDict::parent);
        if class.is_null() {
            return -1;
        }
    }
}

// Translated from 00dcae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassDict::getMemberInfo` (Xbox PDB): fills the 0x14-byte member info at `out` for
/// the member `index` counted over the ancestors' members first (slot `0x24` gives this
/// class's total): the name (`+0`), this class (`+4`, even when an ancestor has the member),
/// the type (`+8`), the class implementation registered under the member's class name (`+0xc`,
/// 0 for a null name) and the extra word (`+0x10`). An index past the members writes nothing.
pub fn hk_data_class_dict_get_member_info(
    e: &mut Engine,
    this: Ptr<HkDataClassDict>,
    index: i32,
    out: u32,
) {
    let total = e.vcall(this.addr(), 0x24, &args![]).i32();
    let mut owner = this;
    let mut remaining = index
        .wrapping_sub(total)
        .wrapping_add(e.get(owner, HkDataClassDict::member_count));
    while remaining < 0 {
        owner = e.get(owner, HkDataClassDict::parent);
        if owner.is_null() {
            return;
        }
        remaining = remaining.wrapping_add(e.get(owner, HkDataClassDict::member_count));
    }
    let members = e.get(owner, HkDataClassDict::members).addr();
    let record = members.wrapping_add((remaining as u32).wrapping_mul(0x18));
    let name = e.mem.u32(record);
    e.mem.set_u32(out, name);
    e.mem.set_u32(out + 4, this.addr());
    let member_type = e.mem.u32(record + 0x10);
    e.mem.set_u32(out + 8, member_type);
    let handle = e.mem.u32(record + 4);
    let class_name = e.mem.u32(handle + 8);
    let world = e.get(this, HkDataClassDict::world);
    let tracker_address = world_tracker(e, world);
    if class_name != 0 {
        let class = e
            .call(
                STRING_MAP_GET_WITH_DEFAULT,
                &args![tracker_address, class_name, 0u32],
            )
            .u32();
        e.mem.set_u32(out + 0xc, class);
    } else {
        e.mem.set_u32(out + 0xc, 0);
    }
    let extra = e.mem.u32(record + 0x14);
    e.mem.set_u32(out + 0x10, extra);
}

/// The record of the member `index` of `class`, counted over the ancestors' members first
/// (slot `0x24` is the class's total), as `StructArrayImplementation`'s constructor and
/// `addMember` inline it: the first class on the way up whose running index is not negative
/// gives `members + index * 0x18`; with no such class the exe takes the first record of
/// `class` minus 0x18 and so does this.
fn class_member_record(e: &mut Engine, class: u32, index: i32) -> u32 {
    let total = e.vcall(class, 0x24, &args![]).i32();
    let mut remaining = index
        .wrapping_sub(total)
        .wrapping_add(e.mem.i32(class + 0x2c));
    let mut owner = class;
    loop {
        if remaining >= 0 {
            let members = e.mem.u32(owner + 0x28);
            return members.wrapping_add((remaining as u32).wrapping_mul(0x18));
        }
        owner = e.mem.u32(owner + 0x18);
        if owner == 0 {
            return e.mem.u32(class + 0x28).wrapping_sub(0x18);
        }
        remaining = remaining.wrapping_add(e.mem.i32(owner + 0x2c));
    }
}

// Translated from 00dcaeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `StructArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_73fc` has `StructArrayImplementation::clear`, `reserve`, `setSize` and `asObject` in
/// its slots): `+6` 0, reference count `+8` 0, an empty record array (`+0xc` data 0, `+0x10`
/// size 0, `+0x14` capacity word `0x8000_0000`), one more reference on `class`, which is kept at
/// `+0x18`, and `+0x1c` (the size) 0. Each of the class's members (slot `0x24` counts them) is
/// then added with `_addMember`, and the array is put in the array multi-map of the tracker
/// (the object that slot `4` of the class gives, field `+8`; `+0x6c`) under the key of slot
/// `0x18` of the array. Returns `this`.
pub fn struct_array_implementation_construct(e: &mut Engine, this: u32, class: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this, 0x010d_73fc);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    if class != 0 {
        add_object_reference(e, class);
    }
    e.mem.set_u32(this + 0x18, class);
    e.mem.set_u32(this + 0x1c, 0);
    let class_now = e.mem.u32(this + 0x18);
    let total = e.vcall(class_now, 0x24, &args![]).i32();
    let mut index = 0i32;
    if total > 0 {
        loop {
            let class_now = e.mem.u32(this + 0x18);
            let record = class_member_record(e, class_now, index);
            struct_array_implementation_add_member_impl(e, this, record);
            index += 1;
            let class_now = e.mem.u32(this + 0x18);
            let total = e.vcall(class_now, 0x24, &args![]).i32();
            if index >= total {
                break;
            }
        }
    }
    let class_now = e.mem.u32(this + 0x18);
    let owner = e.vcall(class_now, 4, &args![]).u32();
    let tracker_address = e.mem.u32(owner + 8);
    let key = e.vcall(this, 0x18, &args![]).u32();
    e.with_stack(4, |e, held| {
        e.mem.set_u32(held.addr(), this);
        e.call(
            MULTI_MAP_INSERT,
            &args![tracker_address + tracker::ARRAY_MAP, key, held.addr()],
        );
    });
    this
}

// Translated from 00dcaf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayImpl::set` (Xbox PDB; `index` and the two words of an `hkDataObject::Value`
/// are the three stack words): stores the value at `index` through the setter that fits the
/// array's type (slot `4`, `getType`). With bit `0x20` set (an array of tuples) the value
/// read as an `hkDataArray` goes to slot `0x5c` (`setArray`) and the temporary is destroyed.
/// Otherwise `type - 1` picks: 0 and 1 (int) `asInt` to slot `0x44`; 2 (real) `asReal`,
/// through an `FSTP` to a float, to slot `0x34`; 3 to 6 (vectors) `asVec` of `type * 4 - 12`
/// floats to slot `0x24`; 7 and 8 (objects) `asObject` to slot `0x54`, the temporary object
/// destroyed afterwards; 9 (string) `asString` to slot `0x2c`. Any other type stores nothing.
pub fn hk_data_array_impl_set(
    e: &mut Engine,
    this: u32,
    index: i32,
    value_first: u32,
    value_second: u32,
) {
    e.with_stack(0x10, |e, block| {
        let value = block.addr();
        let local = value + 8;
        e.mem.set_u32(value, value_first);
        e.mem.set_u32(value + 4, value_second);
        let array_type = e.vcall(this, 4, &args![]).u8();
        if array_type & 0x20 != 0 {
            let array = e
                .call(DATA_OBJECT_VALUE_AS_ARRAY, &args![value, local])
                .u32();
            let implementation = e.mem.u32(array);
            e.vcall(this, 0x5c, &args![index, implementation]);
            e.call(DATA_ARRAY_DESTRUCT, &args![local]);
            return;
        }
        let kind = e.vcall(this, 4, &args![]).u32().wrapping_sub(1);
        match kind {
            0 | 1 => {
                let int_value = e.call(DATA_OBJECT_VALUE_AS_INT, &args![value]).u32();
                e.vcall(this, 0x44, &args![index, int_value]);
            }
            2 => {
                let real = e.call(DATA_OBJECT_VALUE_AS_REAL, &args![value]).f64();
                e.vcall(this, 0x34, &args![index, real as f32]);
            }
            3..=6 => {
                let array_type = e.vcall(this, 4, &args![]).u32();
                let count = array_type.wrapping_mul(4).wrapping_sub(12);
                let floats = e.call(DATA_OBJECT_VALUE_AS_VEC, &args![value, count]).u32();
                e.vcall(this, 0x24, &args![index, floats]);
            }
            7 | 8 => {
                let object = e
                    .call(DATA_OBJECT_VALUE_AS_OBJECT, &args![value, local])
                    .u32();
                let implementation = e.mem.u32(object);
                e.vcall(this, 0x54, &args![index, implementation]);
                e.call(DATA_OBJECT_DESTRUCT, &args![local]);
            }
            9 => {
                let text = e.call(DATA_OBJECT_VALUE_AS_STRING, &args![value]).u32();
                e.vcall(this, 0x2c, &args![index, text]);
            }
            _ => {}
        }
    });
}

// Translated from 00dcb0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::getHandle` (Xbox PDB; the vtable of `StructArrayImplementation::Object`
/// has the same body in slot 4): stores `this` and 0 at `out` and returns `out`.
pub fn hk_data_object_dict_get_handle(e: &mut Engine, this: u32, out: u32) -> u32 {
    e.mem.set_u32(out, this);
    e.mem.set_u32(out + 4, 0);
    out
}

// Translated from 00dcb0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::asObject` (Xbox PDB): makes a `StructArrayImplementation::Object`
/// (0x14 bytes from the router's allocator, slot `4`: vtable `0x010d_7494`, no references, the
/// array at `+0xc`, `index` at `+0x10`) and returns it.
pub fn struct_array_implementation_as_object(e: &mut Engine, this: u32, index: i32) -> u32 {
    let allocator = router_allocator(e);
    let object = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(object + 4, 0x14);
    e.mem.set_u32(object + 0xc, this);
    e.mem.set_u16(object + 6, 0);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object, 0x010d_7494);
    e.mem.set_i32(object + 0x10, index);
    object
}

// Translated from 00dcb120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::hasMember` (Xbox PDB): interns `name` through slot
/// `0x60` of the object (`intern(out, name)`), then answers whether a record of the array has
/// exactly that name word.
pub fn struct_array_object_has_member(e: &mut Engine, this: u32, name: u32) -> bool {
    e.with_stack(4, |e, interned_slot| {
        e.vcall(this, 0x60, &args![interned_slot.addr(), name]);
        let interned = e.mem.u32(interned_slot.addr());
        let array = e.mem.u32(this + 0xc);
        let count = e.mem.i32(array + 0x10);
        let mut index = 0;
        if count > 0 {
            let mut record = e.mem.u32(array + 0xc);
            while index < count {
                if interned == e.mem.u32(record) {
                    return true;
                }
                index += 1;
                record = record.wrapping_add(8);
            }
        }
        false
    })
}

// Translated from 00dcb170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::isValid` (Xbox PDB): whether `index` (signed) is below
/// the array's record count.
pub fn struct_array_object_is_valid(e: &mut Engine, this: u32, index: i32) -> bool {
    let array = e.mem.u32(this + 0xc);
    index < e.mem.i32(array + 0x10)
}

// Translated from 00dcb190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::getMemberName` (Xbox PDB): the name word of the array's
/// record `index`.
pub fn struct_array_object_get_member_name(e: &mut Engine, this: u32, index: i32) -> u32 {
    let array = e.mem.u32(this + 0xc);
    let records = e.mem.u32(array + 0xc);
    e.mem
        .u32(records.wrapping_add((index as u32).wrapping_mul(8)))
}

// Translated from 00dcb1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::getMemberValue` (Xbox PDB): stores `this` at `out` and
/// the name word of the array's record `index` at `out + 4`; returns `out`.
pub fn struct_array_object_get_member_value(
    e: &mut Engine,
    this: u32,
    out: u32,
    index: i32,
) -> u32 {
    e.mem.set_u32(out, this);
    let array = e.mem.u32(this + 0xc);
    let records = e.mem.u32(array + 0xc);
    let name = e
        .mem
        .u32(records.wrapping_add((index as u32).wrapping_mul(8)));
    e.mem.set_u32(out + 4, name);
    out
}

// Translated from 00dcb1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::setObject` (Xbox PDB): for each record of the array (the count
/// is read again after every one) asks `object` through its slot `0xc` for the value of the
/// record's name (`out`, name) and stores it in the record's array at `index` with
/// `hkDataArrayImpl::set` (the two words the slot answered).
pub fn struct_array_implementation_set_object(e: &mut Engine, this: u32, index: i32, object: u32) {
    e.with_stack(8, |e, value| {
        let mut record_index = 0i32;
        while record_index < e.mem.i32(this + 0x10) {
            let record = e
                .mem
                .u32(this + 0xc)
                .wrapping_add((record_index as u32).wrapping_mul(8));
            let name = e.mem.u32(record);
            let answer = e.vcall(object, 0xc, &args![value.addr(), name]).u32();
            let value_first = e.mem.u32(answer);
            let value_second = e.mem.u32(answer + 4);
            let array = e.mem.u32(record + 4);
            hk_data_array_impl_set(e, array, index, value_first, value_second);
            record_index += 1;
        }
    });
}

// Translated from 00dcb210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::clear` (Xbox PDB): calls slot `8` (`clear`) of the array of each
/// record, then sets the size (`+0x1c`) to 0.
pub fn struct_array_implementation_clear(e: &mut Engine, this: u32) {
    let mut index = 0i32;
    while index < e.mem.i32(this + 0x10) {
        let records = e.mem.u32(this + 0xc);
        let array = e
            .mem
            .u32(records.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        e.vcall(array, 8, &args![]);
        index += 1;
    }
    e.mem.set_u32(this + 0x1c, 0);
}

// Translated from 00dcb250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::reserve` (Xbox PDB): calls slot `0xc` (`reserve`) of the array
/// of each record with `count`.
pub fn struct_array_implementation_reserve(e: &mut Engine, this: u32, count: i32) {
    let mut index = 0i32;
    while index < e.mem.i32(this + 0x10) {
        let records = e.mem.u32(this + 0xc);
        let array = e
            .mem
            .u32(records.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        e.vcall(array, 0xc, &args![count]);
        index += 1;
    }
}

// Translated from 00dcb280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::setSize` (Xbox PDB): stores `size` at `+0x1c`, then calls slot
/// `0x10` (`setSize`) of the array of each record with `size`.
pub fn struct_array_implementation_set_size(e: &mut Engine, this: u32, size: i32) {
    let count = e.mem.i32(this + 0x10);
    e.mem.set_i32(this + 0x1c, size);
    let mut index = 0i32;
    if count > 0 {
        loop {
            let records = e.mem.u32(this + 0xc);
            let array = e
                .mem
                .u32(records.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            e.vcall(array, 0x10, &args![size]);
            index += 1;
            if index >= e.mem.i32(this + 0x10) {
                break;
            }
        }
    }
}

// Translated from 00dcb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::_find` (Xbox PDB): interns `name` through the tracker of the
/// array's class's world (null stays null) and gives the index of the first record with that
/// name word, or `-1`.
pub fn struct_array_implementation_find(e: &mut Engine, this: u32, name: u32) -> i32 {
    let class = e.mem.u32(this + 0x18);
    let world = e.mem.u32(class + 0xc);
    let tracker_address = e.mem.u32(world + 8);
    let interned = intern_name(e, tracker_address, name);
    let count = e.mem.i32(this + 0x10);
    let mut index = 0;
    if count > 0 {
        let mut record = e.mem.u32(this + 0xc);
        while index < count {
            if e.mem.u32(record) == interned {
                return index;
            }
            index += 1;
            record = record.wrapping_add(8);
        }
    }
    -1
}

// Translated from 00dcb320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::intern` (Xbox PDB): interns `name` through the class of
/// the object's array (`fn_00dc9570`) into `out` and returns `out`.
pub fn struct_array_object_intern(e: &mut Engine, this: u32, out: u32, name: u32) -> u32 {
    let array = e.mem.u32(this + 0xc);
    let class = e.mem.u32(array + 0x18);
    fn_00dc9570(e, Ptr::new(class), out, name);
    out
}

/// The array implementation of the record named `name` in the array of a
/// `StructArrayImplementation::Object`, and the object's index: what the object's accessors
/// forward to.
fn struct_object_target(e: &mut Engine, this: u32, name: u32) -> (u32, u32) {
    let array = e.mem.u32(this + 0xc);
    let found = struct_array_implementation_find(e, array, name);
    let array = e.mem.u32(this + 0xc);
    let records = e.mem.u32(array + 0xc);
    let implementation = e
        .mem
        .u32(records.wrapping_add((found as u32).wrapping_mul(8)) + 4);
    let object_index = e.mem.u32(this + 0x10);
    (implementation, object_index)
}

// Translated from 00dcb340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asVec` (Xbox PDB; the second stack word is not read):
/// the vector `asVec(index)` (slot `0x20`) of the array of the record named `name`, at the
/// object's index.
pub fn struct_array_object_as_vec(e: &mut Engine, this: u32, name: u32, _unused_2: u32) -> u32 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    e.vcall(implementation, 0x20, &args![object_index]).u32()
}

// Translated from 00dcb370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asString` (Xbox PDB): tail-calls slot `0x28` (`asString`)
/// of the array of the record named `name` with the object's index.
pub fn struct_array_object_as_string(e: &mut Engine, this: u32, name: u32) -> u32 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    e.vcall(implementation, 0x28, &args![object_index]).u32()
}

// Translated from 00dcb3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asArray` (Xbox PDB): tail-calls slot `0x58` (`asArray`)
/// of the array of the record named `name` with the object's index.
pub fn struct_array_object_as_array(e: &mut Engine, this: u32, name: u32) -> u32 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    e.vcall(implementation, 0x58, &args![object_index]).u32()
}

// Translated from 00dcb3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asInt` (Xbox PDB): `asInt(index)` (slot `0x40`) of the
/// array of the record named `name` at the object's index, sign-extended to 64 bits (`CDQ`).
pub fn struct_array_object_as_int(e: &mut Engine, this: u32, name: u32) -> u64 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    let value = e.vcall(implementation, 0x40, &args![object_index]).u32();
    i64::from(value as i32) as u64
}

// Translated from 00dcb400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asReal` (Xbox PDB): tail-calls slot `0x30` (`asReal`) of
/// the array of the record named `name` with the object's index; the `ST0` result is passed
/// through (hence `f64`).
pub fn struct_array_object_as_real(e: &mut Engine, this: u32, name: u32) -> f64 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    e.vcall(implementation, 0x30, &args![object_index]).f64()
}

// Translated from 00dcb430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asHalf` (Xbox PDB): slot `0x38` of the array of the
/// record named `name`, given `out` and the object's index; returns `out`.
pub fn struct_array_object_as_half(e: &mut Engine, this: u32, out: u32, name: u32) -> u32 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    e.vcall(implementation, 0x38, &args![out, object_index]);
    out
}

// Translated from 00dcb460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::asObject` (Xbox PDB): tail-calls slot `0x50`
/// (`asObject`) of the array of the record named `name` with the object's index.
pub fn struct_array_object_as_object(e: &mut Engine, this: u32, name: u32) -> u32 {
    let (implementation, object_index) = struct_object_target(e, this, name);
    e.vcall(implementation, 0x50, &args![object_index]).u32()
}

// Translated from 00dcb490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::swizzleObjectMember` (Xbox PDB): the word at `+4` of the
/// 8-byte record `index` that `_find(name)` gives, read from `this + 0xc` (the record array's
/// data pointer) the way the exe indexes it.
pub fn struct_array_implementation_swizzle_object_member(
    e: &mut Engine,
    this: u32,
    name: u32,
) -> u32 {
    let found = struct_array_implementation_find(e, this, name);
    let records = e.mem.u32(this + 0xc);
    e.mem
        .u32(records.wrapping_add((found as u32).wrapping_mul(8)) + 4)
}

// Translated from 00dcb4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<hkDataClassDict*,hkDataClassDict*>::removeByIndex` (Xbox PDB): removes
/// the entry `index` of the chain of `key`. When the entry has a successor the successor's
/// value and link move into its place and the successor's slot goes on the free list (`+0x18`);
/// the answer is `index`. For the last entry of the chain the key is looked up in the index map
/// (`+0xc`): when the entry is the chain's head the key's head becomes `-1`; when it follows the
/// head directly the head's link becomes `-1`; otherwise the key's head moves to the head's
/// successor and the head's value is copied into the entry, and the head's slot is the one freed.
/// The freed slot is put on the free list; the answer is `-1`.
pub fn hk_serialize_multi_map_remove_by_index(
    e: &mut Engine,
    this: u32,
    key: u32,
    index: i32,
) -> i32 {
    let entries = e.mem.u32(this);
    let slot = entries.wrapping_add((index as u32).wrapping_mul(8));
    let next = e.mem.i32(slot + 4);
    if next != -1 {
        let successor = entries.wrapping_add((next as u32).wrapping_mul(8));
        let value = e.mem.u32(successor);
        e.mem.set_u32(slot, value);
        let link = e.mem.u32(successor + 4);
        e.mem.set_u32(slot + 4, link);
        let free_head = e.mem.u32(this + multi_map::FREE_HEAD);
        let entries = e.mem.u32(this);
        e.mem.set_u32(
            entries.wrapping_add((next as u32).wrapping_mul(8)) + 4,
            free_head,
        );
        e.mem.set_i32(this + multi_map::FREE_HEAD, next);
        return index;
    }
    let map = this + multi_map::MAP;
    let found = e.call(POINTER_MAP_FIND_KEY, &args![map, key]).u32();
    let table = e.mem.u32(map);
    let head_slot = table.wrapping_add(found.wrapping_mul(8)) + 4;
    let head = e.mem.i32(head_slot);
    let freed = if head == index {
        e.mem.set_i32(head_slot, -1);
        index
    } else {
        let entries = e.mem.u32(this);
        let head_link = entries.wrapping_add((head as u32).wrapping_mul(8)) + 4;
        let following = e.mem.i32(head_link);
        if following == index {
            e.mem.set_i32(head_link, -1);
            index
        } else {
            e.mem.set_i32(head_slot, following);
            let entries = e.mem.u32(this);
            let head_value = e
                .mem
                .u32(entries.wrapping_add((head as u32).wrapping_mul(8)));
            e.mem.set_u32(
                entries.wrapping_add((index as u32).wrapping_mul(8)),
                head_value,
            );
            head
        }
    };
    let free_head = e.mem.u32(this + multi_map::FREE_HEAD);
    let entries = e.mem.u32(this);
    e.mem.set_u32(
        entries.wrapping_add((freed as u32).wrapping_mul(8)) + 4,
        free_head,
    );
    e.mem.set_i32(this + multi_map::FREE_HEAD, freed);
    next
}

// Translated from 00dcb550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::addMember` (Xbox PDB): asks the array's class (slot `0x2c`,
/// `getMemberIndexByName`) for the member `name`, takes that member's record the way the
/// constructor does and adds it with `_addMember`. When a record was added (the answer is not
/// `-1`), the new record's array gets `setSize` (slot `0x10`) of the array's size (slot `0x14`).
/// Returns the new record's index.
pub fn struct_array_implementation_add_member(e: &mut Engine, this: u32, name: u32) -> i32 {
    let class = e.mem.u32(this + 0x18);
    let member_index = e.vcall(class, 0x2c, &args![name]).i32();
    let class = e.mem.u32(this + 0x18);
    let record = class_member_record(e, class, member_index);
    let added = struct_array_implementation_add_member_impl(e, this, record);
    if added != -1 {
        let records = e.mem.u32(this + 0xc);
        let new_array = e
            .mem
            .u32(records.wrapping_add((added as u32).wrapping_mul(8)) + 4);
        let size = e.vcall(this, 0x14, &args![]).u32();
        e.vcall(new_array, 0x10, &args![size]);
    }
    added
}

// Translated from 00dcb5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::_findOrAdd` (Xbox PDB): interns `name` through the tracker of
/// the array's class's world (null stays null); the index of the first record with that name
/// word, or else the index `addMember` gives for the interned name.
pub fn struct_array_implementation_find_or_add(e: &mut Engine, this: u32, name: u32) -> i32 {
    let class = e.mem.u32(this + 0x18);
    let world = e.mem.u32(class + 0xc);
    let tracker_address = e.mem.u32(world + 8);
    let interned = intern_name(e, tracker_address, name);
    let count = e.mem.i32(this + 0x10);
    let mut index = 0;
    if count > 0 {
        let mut record = e.mem.u32(this + 0xc);
        while index < count {
            if e.mem.u32(record) == interned {
                return index;
            }
            index += 1;
            record = record.wrapping_add(8);
        }
    }
    struct_array_implementation_add_member(e, this, interned)
}

/// The array implementation and index an assigning accessor of a
/// `StructArrayImplementation::Object` forwards to: `_findOrAdd(name)` in the object's array,
/// then that record's array and the object's index.
fn struct_object_assign_target(e: &mut Engine, this: u32, name: u32) -> (u32, u32) {
    let array = e.mem.u32(this + 0xc);
    let found = struct_array_implementation_find_or_add(e, array, name);
    let array = e.mem.u32(this + 0xc);
    let records = e.mem.u32(array + 0xc);
    let implementation = e
        .mem
        .u32(records.wrapping_add((found as u32).wrapping_mul(8)) + 4);
    let object_index = e.mem.u32(this + 0x10);
    (implementation, object_index)
}

// Translated from 00dcb650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign` (Xbox PDB): `hkDataArrayImpl::set` on the array
/// of the record `_findOrAdd(name)` gives, at the object's index, with the two words of the
/// value at `value`.
pub fn struct_array_object_assign(e: &mut Engine, this: u32, name: u32, value: u32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    let value_first = e.mem.u32(value);
    let value_second = e.mem.u32(value + 4);
    hk_data_array_impl_set(
        e,
        implementation,
        object_index as i32,
        value_first,
        value_second,
    );
}

// Translated from 00dcb690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov2` (Xbox PDB): slot `0x54` (`setObject`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and `object`.
pub fn struct_array_object_assign_ov2(e: &mut Engine, this: u32, name: u32, object: u32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x54, &args![object_index, object]);
}

// Translated from 00dcb6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov3` (Xbox PDB): slot `0x5c` (`setArray`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and `array`.
pub fn struct_array_object_assign_ov3(e: &mut Engine, this: u32, name: u32, array: u32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x5c, &args![object_index, array]);
}

// Translated from 00dcb6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov4` (Xbox PDB): slot `0x2c` (`setString`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and `text`.
pub fn struct_array_object_assign_ov4(e: &mut Engine, this: u32, name: u32, text: u32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x2c, &args![object_index, text]);
}

// Translated from 00dcb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov5` (Xbox PDB): slot `0x34` (`setReal`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and the float `value`
/// (through an `FLD`/`FSTP` pair).
pub fn struct_array_object_assign_ov5(e: &mut Engine, this: u32, name: u32, value: f32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x34, &args![object_index, x87_quiet(value)]);
}

// Translated from 00dcb760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov6` (Xbox PDB): slot `0x3c` of the array of the
/// record `_findOrAdd(name)` gives, with the object's index and `value`.
pub fn struct_array_object_assign_ov6(e: &mut Engine, this: u32, name: u32, value: u32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x3c, &args![object_index, value]);
}

// Translated from 00dcb790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov7` (Xbox PDB; the third stack word is not
/// read): slot `0x24` (`setVec`) of the array of the record `_findOrAdd(name)` gives, with the
/// object's index and `values`.
pub fn struct_array_object_assign_ov7(
    e: &mut Engine,
    this: u32,
    name: u32,
    values: u32,
    _unused_3: u32,
) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x24, &args![object_index, values]);
}

// Translated from 00dcb7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov8` (Xbox PDB): slot `0x44` (`setInt`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and the sign-extended
/// byte `value`.
pub fn struct_array_object_assign_ov8(e: &mut Engine, this: u32, name: u32, value: i8) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x44, &args![object_index, i32::from(value)]);
}

// Translated from 00dcb7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov9` (Xbox PDB): slot `0x44` (`setInt`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and the sign-extended
/// 16-bit `value`.
pub fn struct_array_object_assign_ov9(e: &mut Engine, this: u32, name: u32, value: i16) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x44, &args![object_index, i32::from(value)]);
}

// Translated from 00dcb820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov10` (Xbox PDB): slot `0x44` (`setInt`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and `value`.
pub fn struct_array_object_assign_ov10(e: &mut Engine, this: u32, name: u32, value: u32) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(implementation, 0x44, &args![object_index, value]);
}

// Translated from 00dcb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StructArrayImplementation::Object::assign_ov11` (Xbox PDB): slot `0x4c` (`setInt64`) of the
/// array of the record `_findOrAdd(name)` gives, with the object's index and the two words of
/// `value`.
pub fn struct_array_object_assign_ov11(e: &mut Engine, this: u32, name: u32, value: u64) {
    let (implementation, object_index) = struct_object_assign_target(e, this, name);
    e.vcall(
        implementation,
        0x4c,
        &args![object_index, value as u32, (value >> 32) as u32],
    );
}

// Translated from 00dcb890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `StructArrayImplementation` (the map gives it only a range; the deleting
/// destructor `00dcb960` calls it): the vtable `0x010d_73fc` is put in, the array of each record
/// loses one reference (its deleting destructor runs at zero), the array is taken out of the
/// tracker's array multi-map (`+0x6c` of the object slot `4` of the class gives, under the key of
/// slot `0x18` of the array: the first entry whose value is this array, removed by index), the
/// class loses a reference and is cleared, the record array is freed (8 bytes per element,
/// unless it does not own its buffer) and the base vtable `0x010d_62f4` is put in.
pub fn fn_00dcb890(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010d_73fc);
    let mut index = 0i32;
    while index < e.mem.i32(this + 0x10) {
        let records = e.mem.u32(this + 0xc);
        let array = e
            .mem
            .u32(records.wrapping_add((index as u32).wrapping_mul(8)) + 4);
        release_object(e, array);
        index += 1;
    }
    let class = e.mem.u32(this + 0x18);
    let owner = e.vcall(class, 4, &args![]).u32();
    let tracker_address = e.mem.u32(owner + 8);
    let key = e.vcall(this, 0x18, &args![]).u32();
    let array_map = tracker_address + tracker::ARRAY_MAP;
    let mut entry = fn_00dc8e30(e, array_map, key);
    while entry != -1 {
        let entries = e.mem.u32(array_map);
        if e.mem
            .u32(entries.wrapping_add((entry as u32).wrapping_mul(8)))
            == this
        {
            hk_serialize_multi_map_remove_by_index(e, array_map, key, entry);
            break;
        }
        entry = e
            .mem
            .i32(entries.wrapping_add((entry as u32).wrapping_mul(8)) + 4);
    }
    let class = e.mem.u32(this + 0x18);
    release_object_if_set(e, class);
    e.mem.set_u32(this + 0x18, 0);
    array_free(e, this + 0xc, 8);
    e.mem.set_u32(this, 0x010d_62f4);
}

/// Releases the class-name handle word of each of the `count` 0x18-byte member records at
/// `records` and clears it (`+4` of each record), as the exe's class teardown loops do.
fn release_member_handles(e: &mut Engine, records: u32, count: i32) {
    let mut handle_slot = records.wrapping_add(4);
    let mut left = count;
    while left != 0 {
        let handle = e.mem.u32(handle_slot);
        if handle != 0 {
            e.call(REMOVE_REFERENCE, &args![handle]);
        }
        e.mem.set_u32(handle_slot, 0);
        handle_slot = handle_slot.wrapping_add(0x18);
        left -= 1;
    }
}

// Translated from 00dcb990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of `hkDataClassDict` (the map gives it only a range; the destructor body and
/// `untrackClass` call it): the class handle of each member record is released and cleared and
/// the member count becomes 0; each of the objects the pointer array at `+0x1c` holds (0x10 bytes:
/// an array of 8-byte elements at `+4`) has its size cleared, its element array freed and the
/// block itself returned to the router's allocator; the pointer count (`+0x20`) becomes 0; and
/// the class-name handle at `+0x10` is replaced by the static default handle (which gains one
/// reference, the old handle losing one).
pub fn fn_00dcb990(e: &mut Engine, this: u32) {
    let member_count = e.mem.i32(this + 0x2c);
    let members = e.mem.u32(this + 0x28);
    if member_count > 0 {
        release_member_handles(e, members, member_count);
    }
    e.mem.set_u32(this + 0x2c, 0);
    let mut index = 0i32;
    while index < e.mem.i32(this + 0x20) {
        let block = e.mem.u32(
            e.mem
                .u32(this + 0x1c)
                .wrapping_add((index as u32).wrapping_mul(4)),
        );
        if block != 0 {
            let allocator = router_allocator(e);
            let capacity = e.mem.u32(block + 0xc);
            e.mem.set_u32(block + 8, 0);
            if capacity & 0x8000_0000 == 0 {
                let bytes = (capacity & 0x3fff_ffff).wrapping_mul(8);
                let data = e.mem.u32(block + 4);
                e.vcall(allocator, 0x10, &args![data, bytes]);
            }
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block + 0xc, 0x8000_0000);
            let allocator = router_allocator(e);
            e.vcall(allocator, 8, &args![block, 0x10u32]);
        }
        index += 1;
    }
    e.mem.set_u32(this + 0x20, 0);
    e.call(ADD_REFERENCE, &args![DEFAULT_CLASS_HANDLE]);
    let old_handle = e.mem.u32(this + 0x10);
    if old_handle != 0 {
        e.call(REMOVE_REFERENCE, &args![old_handle]);
    }
    e.mem.set_u32(this + 0x10, DEFAULT_CLASS_HANDLE);
}

// Translated from 00dcba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::_readAccess` (Xbox PDB): finds the member info of `name` in the class and
/// stores `{info, index}` at `out`; with no info that is `{0, -1}`. With an info, the index of
/// the last value record with that name word is used. A missing record is appended (the array
/// grows by one 0x10-byte element when full) with its name set, and its value made from the
/// info's type: a type with bit `0x10` gets an array made by `hkDataArrayDict_create` (type
/// without the bit, the class the world's slot `0x24` finds for the member's class name, tuple
/// size 0), a type with bit `0x20` the same array sized to the info's extra word, type 8 a null
/// value, any other type a zero value; the array gains a reference. The index is the old count.
/// The third stack word is not read.
pub fn hk_data_object_dict_read_access(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    out: u32,
    name: u32,
    _unused_3: u32,
) -> u32 {
    let class = e.get(this, HkDataObjectDict::class);
    let info = hk_data_class_dict_get_member_info_by_name(e, class, name);
    if info == 0 {
        e.mem.set_u32(out, 0);
        e.mem.set_i32(out + 4, -1);
        return out;
    }
    let count = e.get(this, HkDataObjectDict::member_count);
    let mut found = -1i32;
    let mut index = 0i32;
    if count > 0 {
        let mut record = e.get(this, HkDataObjectDict::members).addr();
        while index < count {
            if e.mem.u32(record) == name {
                found = index;
            }
            index += 1;
            record = record.wrapping_add(0x10);
        }
        if found != -1 {
            e.mem.set_u32(out, info);
            e.mem.set_i32(out + 4, found);
            return out;
        }
    }
    let array = this.addr() + 0x10;
    array_grow_if_full(e, array, 0x10);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let slot = (size << 4).wrapping_add(data);
    if slot != 0 {
        e.mem.set_u32(slot, 0);
    }
    let size = e.mem.u32(array + 4);
    let record = (size << 4).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(record, name);
    let member_type = e.mem.u32(info + 0x10);
    if member_type & 0x10 != 0 {
        let array_object = read_access_make_array(e, this, info, member_type & 0xffff_ffef, 0);
        e.mem.set_u32(record + 8, array_object);
        add_object_reference(e, array_object);
    } else if member_type & 0x20 != 0 {
        let array_object = read_access_make_array(e, this, info, member_type & 0xffff_ffdf, 0);
        e.mem.set_u32(record + 8, array_object);
        let extra = e.mem.u32(info + 0x14);
        e.vcall(array_object, 0x10, &args![extra]);
        add_object_reference(e, array_object);
    } else if member_type == 8 {
        e.mem.set_u32(record + 8, 0);
    } else {
        e.mem.set_u32(record + 8, 0);
        e.mem.set_u32(record + 0xc, 0);
    }
    e.mem.set_u32(out, info);
    e.mem.set_i32(out + 4, count);
    out
}

/// `hkDataArrayDict_create(type, class, tuple size)` for a member info of
/// `_readAccess`: the class is the one the class's world finds (slot `0x24`) for the class name
/// of the info's class handle (`info + 4`, its field `+8`).
fn read_access_make_array(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    info: u32,
    array_type: u32,
    tuple_size: u32,
) -> u32 {
    let class = e.get(this, HkDataObjectDict::class);
    let world = e.get(class, HkDataClassDict::world).addr();
    let handle = e.mem.u32(info + 4);
    let class_name = e.mem.u32(handle + 8);
    let member_class = e.vcall(world, 0x24, &args![class_name]).u32();
    hk_data_array_dict_create(e, array_type, member_class, tuple_size)
}

// Translated from 00dcbbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::asArray` (Xbox PDB): `_readAccess(name)` for an array member (third word
/// `0x10`), then the word at the value slot of the record it names.
pub fn hk_data_object_dict_as_array(e: &mut Engine, this: Ptr<HkDataObjectDict>, name: u32) -> u32 {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, this, access.addr(), name, 0x10);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.u32(slot)
    })
}

// Translated from 00dcbc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::asString` (Xbox PDB): `_readAccess(name)` for a string member (third word
/// `0xa`), then the word at the value slot of the record it names.
pub fn hk_data_object_dict_as_string(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
) -> u32 {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, this, access.addr(), name, 0xa);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.u32(slot)
    })
}

// Translated from 00dcbc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::asInt` (Xbox PDB): `_readAccess(name)` (third word 0), then, when the
/// member info's type (`+0x10`; with no info the exe reads address `0x10`) is 1 or 2, the 64-bit
/// value at the value slot of the record it names; any other type gives 0.
pub fn hk_data_object_dict_as_int(e: &mut Engine, this: Ptr<HkDataObjectDict>, name: u32) -> u64 {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, this, access.addr(), name, 0);
        let info = e.mem.u32(access.addr());
        let member_type = e.mem.u32(info + 0x10);
        if member_type.wrapping_sub(1) <= 1 {
            let slot = object_dict_value_slot(e, this, access.addr());
            let low = e.mem.u32(slot);
            let high = e.mem.u32(slot + 4);
            u64::from(low) | u64::from(high) << 32
        } else {
            0
        }
    })
}

// Translated from 00dcbca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::asObject` (Xbox PDB): `_readAccess(name)` for an object member (third word
/// 8), then the word at the value slot of the record it names.
pub fn hk_data_object_dict_as_object(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
) -> u32 {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, this, access.addr(), name, 8);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.u32(slot)
    })
}

// Translated from 00dcbcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::asVec` (Xbox PDB): `_readAccess(name)` for a vector member (third word
/// `fn_00dc73e0(count)`), then the word at the value slot of the record it names.
pub fn hk_data_object_dict_as_vec(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    count: u32,
) -> u32 {
    let member_type = fn_00dc73e0(e, count);
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, this, access.addr(), name, member_type);
        let slot = object_dict_value_slot(e, this, access.addr());
        e.mem.u32(slot)
    })
}

// Translated from 00dcbd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::asReal` (Xbox PDB): `_readAccess(name)` for a real member (third word 3),
/// then the float at the value slot of the record it names, loaded with `FLD` and handed back in
/// `ST0` (hence the `f64` result).
pub fn hk_data_object_dict_as_real(e: &mut Engine, this: Ptr<HkDataObjectDict>, name: u32) -> f64 {
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, this, access.addr(), name, 3);
        let slot = object_dict_value_slot(e, this, access.addr());
        let bits = e.mem.u32(slot);
        f64::from(x87_quiet(f32::from_bits(bits)))
    })
}

// Translated from 00dcbd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::assign_ov8` (Xbox PDB): copies the value of the member a handle names
/// (`source`: the object and the member name, two words) into the member `name` of this object.
/// The source's `_readAccess` gives the member info and the value words; an info type with bit
/// 3 or 4 (`0x18`) and a non-null first word adds a reference to that word (an object or an
/// array); `_writeAccess(name)` (third word: the info's type) finds or makes the destination
/// record, whose two value words are then set from the source record. The exe also keeps the
/// second value word in a stack local it never reads again.
pub fn hk_data_object_dict_assign_ov8(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    name: u32,
    source: u32,
) {
    let source_object: Ptr<HkDataObjectDict> = Ptr::new(e.mem.u32(source));
    let source_name = e.mem.u32(source + 4);
    e.with_stack(8, |e, access| {
        hk_data_object_dict_read_access(e, source_object, access.addr(), source_name, 0);
        let source_index = e.mem.u32(access.addr() + 4);
        let source_records = e.get(source_object, HkDataObjectDict::members).addr();
        let source_offset = source_index << 4;
        let value_low = e.mem.u32(source_records.wrapping_add(source_offset) + 8);
        let info = e.mem.u32(access.addr());
        if e.mem.u8(info + 0x10) & 0x18 != 0 && value_low != 0 {
            add_object_reference(e, value_low);
        }
        let member_type = e.mem.u32(info + 0x10);
        hk_data_object_dict_write_access(e, this, access.addr(), name, member_type);
        let source_records = e.get(source_object, HkDataObjectDict::members).addr();
        let value_low = e.mem.u32(source_records.wrapping_add(source_offset) + 8);
        let destination_index = e.mem.u32(access.addr() + 4);
        let destination_records = e.get(this, HkDataObjectDict::members).addr();
        let destination = (destination_index << 4).wrapping_add(destination_records);
        e.mem.set_u32(destination + 8, value_low);
        let value_high = e.mem.u32(source_records.wrapping_add(source_offset) + 0xc);
        e.mem.set_u32(destination + 0xc, value_high);
    });
}

// Translated from 00dcbdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldDict::ObjectTracker::untrackClass` (Xbox PDB): asks `class` for its name (slot
/// `8`) and finds the class registered under it; runs `fn_00dca890` on `class` (destroys its
/// objects), takes `class` out of the derived-class chain of the registered class's parent
/// (slot `0x10`; `fn_00dc8fe0`), runs `fn_00dcb990` on `class`, collects the classes of the
/// derived-class chain of `class` (`+0x10`) in an `hkLocalArray` (0x40 elements from the
/// router's stack allocator), runs `untrackClass` on each and lets it lose one reference,
/// removes the key of `class` from the derived-class multi-map, gives the stack allocator back
/// and frees the array's buffer unless it is the local memory; finally the name's handle record
/// leaves the handle map (`+0x5c`) and loses a reference (the exe does not test it for null)
/// and the name leaves the class map.
pub fn object_tracker_untrack_class(e: &mut Engine, this: u32, class: u32) {
    let name = e.vcall(class, 8, &args![]).u32();
    let registered = if name != 0 {
        e.call(STRING_MAP_GET_WITH_DEFAULT, &args![this, name, 0u32])
            .u32()
    } else {
        0
    };
    fn_00dca890(e, this, class);
    let registered_parent = e.vcall(registered, 0x10, &args![]).u32();
    if registered_parent != 0 {
        e.with_stack(4, |e, held| {
            e.mem.set_u32(held.addr(), class);
            fn_00dc8fe0(
                e,
                this + tracker::DERIVED_MAP,
                registered_parent,
                held.addr(),
            );
        });
    }
    fn_00dcb990(e, class);
    let derived_map = this + tracker::DERIVED_MAP;
    e.with_stack(0x10, |e, block| {
        let children = block.addr();
        array_init_empty(e, children);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let base = e.mem.u32(router);
        e.mem.set_u32(router, base.wrapping_add(0x100));
        e.mem.set_u32(children, base);
        e.mem.set_u32(children + 0xc, base);
        e.mem.set_u32(children + 8, 0x8000_0040);
        let mut index = fn_00dc8e30(e, derived_map, class);
        let has_chain = index != -1;
        if has_chain {
            while index != -1 {
                let entries = e.mem.u32(derived_map);
                let child = e
                    .mem
                    .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
                array_push_word(e, children, child);
                let entries = e.mem.u32(derived_map);
                index = e
                    .mem
                    .i32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            }
        }
        let mut i = 0;
        while i < e.mem.i32(children + 4) {
            let child = e
                .mem
                .u32(e.mem.u32(children).wrapping_add((i as u32).wrapping_mul(4)));
            object_tracker_untrack_class(e, this, child);
            let child = e
                .mem
                .u32(e.mem.u32(children).wrapping_add((i as u32).wrapping_mul(4)));
            release_object(e, child);
            i += 1;
        }
        if has_chain {
            hk_serialize_multi_map_remove_key(e, derived_map, class);
        }
        if base == e.mem.u32(children) {
            e.mem.set_u32(children + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, base);
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(children + 8);
        e.mem.set_u32(children + 4, 0);
        if capacity & 0x8000_0000 == 0 {
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            let data = e.mem.u32(children);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
    let handles = this + tracker::CLASS_HANDLES;
    let record = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![handles, name, 0u32])
        .u32();
    e.call(STRING_MAP_REMOVE, &args![handles, name]);
    e.call(REMOVE_REFERENCE, &args![record]);
    e.call(STRING_MAP_REMOVE, &args![this, name]);
}

// Translated from 00dcbfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `hkDataClassDict` (the map gives it only a range; the deleting destructor
/// `00dcc100` calls it): the vtable `0x010d_73c8` is put in, `fn_00dcb990` runs, and the three
/// arrays are emptied: the one at `+0x34` and the member array at `+0x28` (0x18-byte records,
/// each class handle released and cleared first) and the pointer array at `+0x1c` (4 bytes per
/// element), each buffer freed unless the array does not own it. The parent loses a reference
/// (its deleting destructor runs at zero) and the class-name handle (`+0x10`) is released and
/// cleared, and the base vtable `0x010d_62f4` is put in.
pub fn fn_00dcbfc0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010d_73c8);
    fn_00dcb990(e, this);
    for (array, element_size) in [(0x34u32, 0x18u32), (0x28, 0x18)] {
        let count = e.mem.i32(this + array + 4);
        let records = e.mem.u32(this + array);
        if count > 0 {
            release_member_handles(e, records, count);
        }
        e.mem.set_u32(this + array + 4, 0);
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(this + array + 8);
        if capacity & 0x8000_0000 == 0 {
            let data = e.mem.u32(this + array);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(this + array, 0);
        e.mem.set_u32(this + array + 8, 0x8000_0000);
    }
    array_free(e, this + 0x1c, 4);
    let parent = e.mem.u32(this + 0x18);
    release_object_if_set(e, parent);
    e.mem.set_u32(this + 0x18, 0);
    let handle = e.mem.u32(this + 0x10);
    if handle != 0 {
        e.call(REMOVE_REFERENCE, &args![handle]);
    }
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this, 0x010d_62f4);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_9a20, deleting_destructor_00dc9a20(Ptr, u8) -> Ptr),
        entry!(0x00dc_a190, deleting_destructor_00dca190(Ptr, u8) -> Ptr),
        entry!(0x00dc_a720, deleting_destructor_00dca720(Ptr, u8) -> Ptr),
        entry!(0x00dc_b960, deleting_destructor_00dcb960(Ptr, u8) -> Ptr),
        entry!(0x00dc_bf90, deleting_destructor_00dcbf90(Ptr, u8) -> Ptr),
        entry!(0x00dc_c100, deleting_destructor_00dcc100(Ptr, u8) -> Ptr),
        entry!(
            0x00dc_89d0,
            array_of_tuples_implementation_get_type(Ptr<ArrayOfTuplesImplementation>) -> u32
        ),
        entry!(
            0x00dc_89e0,
            array_of_tuples_implementation_get_class(Ptr<ArrayOfTuplesImplementation>) -> u32
        ),
        entry!(
            0x00dc_89f0,
            array_of_tuples_implementation_clear(Ptr<ArrayOfTuplesImplementation>)
        ),
        entry!(
            0x00dc_8a00,
            array_of_tuples_implementation_reserve(Ptr<ArrayOfTuplesImplementation>, i32)
        ),
        entry!(
            0x00dc_8a20,
            array_of_tuples_implementation_set_size(Ptr<ArrayOfTuplesImplementation>, i32)
        ),
        entry!(
            0x00dc_8a40,
            array_of_tuples_implementation_get_size(Ptr<ArrayOfTuplesImplementation>) -> i32
        ),
        entry!(
            0x00dc_8a60,
            array_of_tuples_implementation_as_array(Ptr<ArrayOfTuplesImplementation>, i32) -> u32
        ),
        entry!(
            0x00dc_8a80,
            array_of_tuples_implementation_set_array(Ptr<ArrayOfTuplesImplementation>, i32, u32)
        ),
        entry!(
            0x00dc_9530,
            hk_data_object_dict_is_valid(Ptr<HkDataObjectDict>, i32) -> bool
        ),
        entry!(
            0x00dc_9540,
            hk_data_object_dict_get_member_name(Ptr<HkDataObjectDict>, i32) -> u32
        ),
        entry!(
            0x00dc_9550,
            hk_data_object_dict_get_member_value(Ptr<HkDataObjectDict>, u32, i32) -> u32
        ),
        entry!(0x00dc_9a50, vec_array_implementation_construct(u32, i32) -> u32),
        entry!(0x00dc_9a80, vec_array_implementation_get_type(u32) -> u32),
        entry!(0x00dc_9ab0, vec_array_implementation_as_vec(u32, i32) -> u32),
        entry!(0x00dc_9ad0, vec_array_implementation_set_vec(u32, i32, u32)),
        entry!(0x00dc_9b00, vec_array_implementation_get_size(u32) -> i32),
        entry!(0x00dc_9b10, vec_array_implementation_clear(u32)),
        entry!(0x00dc_9b20, vec_array_implementation_reserve(u32, i32)),
        entry!(0x00dc_9bc0, fn_00dc9bc0() -> u32),
        entry!(0x00dc_9cd0, basic_array_implementation_u8_reserve(u32, i32)),
        entry!(
            0x00dc_9d70,
            basic_array_implementation_i64_reserve(u32, i32)
        ),
        entry!(
            0x00dc_9e20,
            basic_array_implementation_object_reserve(u32, i32)
        ),
        entry!(0x00dc_9fd0, vec_array_implementation_deleting_destructor(u32, u8) -> u32),
        entry!(0x00dc_a040, pointer_array_implementation_construct(u32, u32) -> u32),
        entry!(0x00dc_a080, pointer_array_implementation_as_object(u32, i32) -> u32),
        entry!(
            0x00dc_a090,
            pointer_array_implementation_set_object(u32, i32, u32)
        ),
        entry!(0x00dc_a0f0, fn_00dca0f0(u32)),
        entry!(0x00dc_a1c0, byte_array_implementation_construct(u32) -> u32),
        entry!(0x00dc_a1e0, byte_array_implementation_as_int(u32, i32) -> u32),
        entry!(0x00dc_a1f0, byte_array_implementation_set_int(u32, i32, u8)),
        entry!(
            0x00dc_a210,
            byte_array_implementation_set_all(u32, u32, i32)
        ),
        entry!(0x00dc_a260, int_array_implementation_construct(u32) -> u32),
        entry!(0x00dc_a280, int_array_implementation_as_int(u32, i32) -> u32),
        entry!(0x00dc_a290, int_array_implementation_set_int(u32, i32, i32)),
        entry!(0x00dc_a2b0, int_array_implementation_as_int64(u32, i32) -> u64),
        entry!(
            0x00dc_a2d0,
            int_array_implementation_set_int64(u32, i32, u32, u32)
        ),
        entry!(0x00dc_a2f0, int_array_implementation_set_all(u32, u32, i32)),
        entry!(
            0x00dc_a360,
            int_array_implementation_set_all_ov2(u32, u32, i32)
        ),
        entry!(
            0x00dc_a3c0,
            int_array_implementation_set_all_ov3(u32, u32, i32)
        ),
        entry!(
            0x00dc_6fc0,
            array_of_tuples_view_get_type(Ptr<ArrayOfTuplesView>) -> u32
        ),
        entry!(
            0x00dc_6fd0,
            array_of_tuples_view_clear(Ptr<ArrayOfTuplesView>)
        ),
        entry!(
            0x00dc_6fe0,
            array_of_tuples_view_get_size(Ptr<ArrayOfTuplesView>) -> i32
        ),
        entry!(
            0x00dc_6ff0,
            array_of_tuples_view_set_all(Ptr<ArrayOfTuplesView>, u32, i32)
        ),
        entry!(
            0x00dc_7030,
            array_of_tuples_view_set_all_ov2(Ptr<ArrayOfTuplesView>, u32, i32)
        ),
        entry!(
            0x00dc_7070,
            array_of_tuples_view_as_vec(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7090,
            array_of_tuples_view_set_vec(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_70c0,
            array_of_tuples_view_as_string(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_70e0,
            array_of_tuples_view_set_string(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_7110,
            array_of_tuples_view_as_real(Ptr<ArrayOfTuplesView>, i32) -> f64
        ),
        entry!(
            0x00dc_7130,
            array_of_tuples_view_set_real(Ptr<ArrayOfTuplesView>, i32, f32)
        ),
        entry!(
            0x00dc_7160,
            array_of_tuples_view_as_int(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7180,
            array_of_tuples_view_set_int(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_71b0,
            array_of_tuples_view_as_int64(Ptr<ArrayOfTuplesView>, i32) -> u64
        ),
        entry!(
            0x00dc_71d0,
            array_of_tuples_view_set_int64(Ptr<ArrayOfTuplesView>, i32, u32, u32)
        ),
        entry!(
            0x00dc_7200,
            array_of_tuples_view_as_object(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7220,
            array_of_tuples_view_set_object(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_7250,
            array_of_tuples_view_as_array(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7270,
            array_of_tuples_view_set_array(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_72a0,
            hk_data_world_dict_get_contents(Ptr<HkDataWorldDict>, u32) -> u32
        ),
        entry!(
            0x00dc_72c0,
            hk_data_world_dict_find_object(Ptr<HkDataWorldDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_72e0,
            hk_data_world_dict_set_class_version(Ptr<HkDataWorldDict>, u32, i32)
        ),
        entry!(
            0x00dc_72f0,
            hk_data_world_dict_get_type(Ptr<HkDataWorldDict>, u32) -> u32
        ),
        entry!(
            0x00dc_7300,
            hk_data_world_dict_find_class(Ptr<HkDataWorldDict>, u32) -> u32
        ),
        entry!(
            0x00dc_7320,
            hk_data_world_dict_rename_class(Ptr<HkDataWorldDict>, u32, u32)
        ),
        entry!(
            0x00dc_7390,
            hk_data_world_dict_find_all_classes(Ptr<HkDataWorldDict>, u32)
        ),
        entry!(
            0x00dc_73a0,
            hk_data_world_dict_find_objects_by_exact_class(Ptr<HkDataWorldDict>, u32, u32)
        ),
        entry!(
            0x00dc_73c0,
            hk_data_world_dict_find_objects_by_base_class(Ptr<HkDataWorldDict>, u32, u32)
        ),
        entry!(0x00dc_73e0, fn_00dc73e0(u32) -> u32),
        entry!(
            0x00dc_73f0,
            hk_data_world_dict_rename_class_member(Ptr<HkDataWorldDict>, u32, u32, u32)
        ),
        entry!(
            0x00dc_7600,
            hk_data_world_dict_remove_class_member(Ptr<HkDataWorldDict>, u32, u32)
        ),
        entry!(0x00dc_7750, fn_00dc7750(Ptr<ObjectTracker>)),
        entry!(
            0x00dc_7c40,
            hk_data_world_dict_new_object(Ptr<HkDataWorldDict>, u32) -> u32
        ),
        entry!(
            0x00dc_7cc0,
            hk_data_world_dict_set_class_parent(Ptr<HkDataWorldDict>, u32, u32)
        ),
        entry!(
            0x00dc_7d20,
            hk_data_world_dict_add_class_member(Ptr<HkDataWorldDict>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00dc_7dc0,
            hk_data_world_dict_cast_object(Ptr<HkDataWorldDict>, u32, u32)
        ),
        entry!(0x00dc_7de0, hk_data_array_dict_create(u32, u32, u32) -> u32),
        entry!(
            0x00dc_7fd0,
            fn_00dc7fd0(Ptr<ObjectTracker>) -> Ptr<ObjectTracker>
        ),
        entry!(
            0x00dc_8030,
            hk_data_world_dict_construct(Ptr<HkDataWorldDict>) -> Ptr<HkDataWorldDict>
        ),
        entry!(
            0x00dc_8080,
            hk_data_world_dict_destruct(Ptr<HkDataWorldDict>)
        ),
        entry!(
            0x00dc_80c0,
            hk_data_world_dict_new_class(Ptr<HkDataWorldDict>, u32) -> u32
        ),
        entry!(
            0x00dc_8210,
            hk_data_world_dict_new_array(Ptr<HkDataWorldDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_8270,
            hk_data_world_dict_wrap_class(Ptr<HkDataWorldDict>, u32) -> u32
        ),
        entry!(
            0x00dc_85a0,
            hk_data_world_dict_copy_class_from_world(Ptr<HkDataWorldDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_8870,
            hk_data_world_dict_remove_class(Ptr<HkDataWorldDict>, u32)
        ),
        entry!(0x00dc_88c0, union_clear_array(u32, u32, i32)),
        entry!(
            0x00dc_8cd0,
            array_of_tuples_implementation_deleting_destructor(
                Ptr<ArrayOfTuplesImplementation>,
                u8,
            )
                -> Ptr<ArrayOfTuplesImplementation>
        ),
        entry!(
            0x00dc_8d70,
            object_tracker_intern_class_name_handle(u32, u32, u32) -> u32
        ),
        entry!(0x00dc_8e30, fn_00dc8e30(u32, u32) -> i32),
        entry!(
            0x00dc_8e50,
            hk_data_class_dict_get_member_info_by_name(Ptr<HkDataClassDict>, u32) -> u32
        ),
        entry!(
            0x00dc_8ea0,
            struct_array_implementation_remove_member(u32, u32)
        ),
        entry!(0x00dc_8f20, fn_00dc8f20(u32, u32, u32)),
        entry!(0x00dc_8fc0, fn_00dc8fc0(u32)),
        entry!(0x00dc_8fe0, fn_00dc8fe0(u32, u32, u32)),
        entry!(0x00dc_9030, hk_serialize_multi_map_remove_key(u32, u32)),
        entry!(
            0x00dc_90a0,
            hk_data_object_dict_self_destruct(Ptr<HkDataObjectDict>)
        ),
        entry!(
            0x00dc_90f0,
            hk_data_object_dict_remove_member(Ptr<HkDataObjectDict>, u32)
        ),
        entry!(
            0x00dc_9180,
            object_tracker_get_tracked_struct_arrays(u32, u32, u32, u32)
        ),
        entry!(0x00dc_9240, object_tracker_find_tracked_classes(u32, u32)),
        entry!(
            0x00dc_92d0,
            object_tracker_find_tracked_objects_by_base(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00dc_9470,
            hk_data_class_dict_remove_member(Ptr<HkDataClassDict>, u32) -> bool
        ),
        entry!(
            0x00dc_9570,
            fn_00dc9570(Ptr<HkDataClassDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_95d0,
            hk_data_object_dict_has_member(Ptr<HkDataObjectDict>, u32) -> bool
        ),
        entry!(
            0x00dc_9640,
            hk_data_object_dict_access_by_name(Ptr<HkDataObjectDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_96a0,
            hk_data_object_dict_destruct(Ptr<HkDataObjectDict>)
        ),
        entry!(
            0x00dc_9710,
            hk_data_object_dict_write_access(Ptr<HkDataObjectDict>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dc_97f0,
            hk_data_object_dict_assign(Ptr<HkDataObjectDict>, u32, u32)
        ),
        entry!(
            0x00dc_9830,
            hk_data_object_dict_assign_ov2(Ptr<HkDataObjectDict>, u32, u32)
        ),
        entry!(
            0x00dc_9880,
            hk_data_object_dict_assign_ov3(Ptr<HkDataObjectDict>, u32, u32)
        ),
        entry!(
            0x00dc_98d0,
            hk_data_object_dict_assign_ov4(Ptr<HkDataObjectDict>, u32, f32)
        ),
        entry!(
            0x00dc_9910,
            hk_data_object_dict_assign_ov5(Ptr<HkDataObjectDict>, u32, i16)
        ),
        entry!(
            0x00dc_9950,
            hk_data_object_dict_assign_ov6(Ptr<HkDataObjectDict>, u32, u32, i32)
        ),
        entry!(
            0x00dc_99c0,
            hk_data_object_dict_assign_ov7(Ptr<HkDataObjectDict>, u32, u64)
        ),
        entry!(0x00dc_9b60, vec_array_implementation_set_size(u32, i32)),
        entry!(
            0x00dc_9bd0,
            basic_array_implementation_object_set_size(u32, i32)
        ),
        entry!(0x00dc_9c20, struct_array_implementation_add_member_impl(u32, u32) -> i32),
        entry!(
            0x00dc_9d10,
            basic_array_implementation_u8_set_size(u32, i32)
        ),
        entry!(
            0x00dc_9db0,
            basic_array_implementation_i64_set_size(u32, i32)
        ),
        entry!(
            0x00dc_9f20,
            hk_data_class_dict_add_member(Ptr<HkDataClassDict>, u32, u32, u32, u32)
        ),
        entry!(
            0x00dc_a420,
            int_array_implementation_set_all_ov4(u32, u32, i32)
        ),
        entry!(
            0x00dc_a480,
            int_array_implementation_set_all_ov5(u32, u32, i32)
        ),
        entry!(
            0x00dc_a4e0,
            int_array_implementation_set_all_ov6(u32, u32, i32)
        ),
        entry!(
            0x00dc_a540,
            int_array_implementation_set_all_ov7(u32, u32, i32)
        ),
        entry!(0x00dc_a5a0, real_array_implementation_construct(u32) -> u32),
        entry!(0x00dc_a5c0, real_array_implementation_as_real(u32, i32) -> f64),
        entry!(
            0x00dc_a5d0,
            real_array_implementation_set_real(u32, i32, f32)
        ),
        entry!(
            0x00dc_a5f0,
            real_array_implementation_set_all(u32, u32, i32)
        ),
        entry!(0x00dc_a650, cstring_array_implementation_construct(u32) -> u32),
        entry!(
            0x00dc_a670,
            cstring_array_implementation_set_string(u32, i32, u32)
        ),
        entry!(0x00dc_a6b0, cstring_array_implementation_destruct(u32)),
        entry!(
            0x00dc_a750,
            byte_array_implementation_deleting_destructor(u32, u8) -> u32
        ),
        entry!(
            0x00dc_a7b0,
            int_array_implementation_deleting_destructor(u32, u8) -> u32
        ),
        entry!(0x00dc_a820, object_tracker_track_class(u32, u32)),
        entry!(0x00dc_a890, fn_00dca890(u32, u32)),
        entry!(0x00dc_aaf0, fn_00dcaaf0(u32, u32, u32)),
        entry!(
            0x00dc_ab50,
            object_tracker_retract_casted_object(u32, u32, u32)
        ),
        entry!(0x00dc_abb0, hk_data_class_dict_construct(u32, u32, u32, u32) -> u32),
        entry!(
            0x00dc_ac10,
            hk_data_class_dict_get_name(Ptr<HkDataClassDict>) -> u32
        ),
        entry!(
            0x00dc_ac20,
            hk_data_class_dict_get_declared_member_info(Ptr<HkDataClassDict>, i32, u32)
        ),
        entry!(
            0x00dc_ac80,
            hk_data_class_dict_is_super_class(Ptr<HkDataClassDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_ad10,
            hk_data_class_dict_get_declared_member_index_by_name(Ptr<HkDataClassDict>, u32) -> i32
        ),
        entry!(
            0x00dc_ad80,
            hk_data_class_dict_get_num_members(Ptr<HkDataClassDict>) -> i32
        ),
        entry!(
            0x00dc_ada0,
            hk_data_class_dict_get_member_index_by_name(Ptr<HkDataClassDict>, u32) -> i32
        ),
        entry!(
            0x00dc_ae30,
            hk_data_class_dict_get_member_info(Ptr<HkDataClassDict>, i32, u32)
        ),
        entry!(0x00dc_aeb0, struct_array_implementation_construct(u32, u32) -> u32),
        entry!(0x00dc_af70, hk_data_array_impl_set(u32, i32, u32, u32)),
        entry!(0x00dc_b0d0, hk_data_object_dict_get_handle(u32, u32) -> u32),
        entry!(0x00dc_b0e0, struct_array_implementation_as_object(u32, i32) -> u32),
        entry!(0x00dc_b120, struct_array_object_has_member(u32, u32) -> bool),
        entry!(0x00dc_b170, struct_array_object_is_valid(u32, i32) -> bool),
        entry!(0x00dc_b190, struct_array_object_get_member_name(u32, i32) -> u32),
        entry!(0x00dc_b1a0, struct_array_object_get_member_value(u32, u32, i32) -> u32),
        entry!(
            0x00dc_b1c0,
            struct_array_implementation_set_object(u32, i32, u32)
        ),
        entry!(0x00dc_b210, struct_array_implementation_clear(u32)),
        entry!(0x00dc_b250, struct_array_implementation_reserve(u32, i32)),
        entry!(0x00dc_b280, struct_array_implementation_set_size(u32, i32)),
        entry!(0x00dc_b2b0, struct_array_implementation_find(u32, u32) -> i32),
        entry!(0x00dc_b320, struct_array_object_intern(u32, u32, u32) -> u32),
        entry!(0x00dc_b340, struct_array_object_as_vec(u32, u32, u32) -> u32),
        entry!(0x00dc_b370, struct_array_object_as_string(u32, u32) -> u32),
        entry!(0x00dc_b3a0, struct_array_object_as_array(u32, u32) -> u32),
        entry!(0x00dc_b3d0, struct_array_object_as_int(u32, u32) -> u64),
        entry!(0x00dc_b400, struct_array_object_as_real(u32, u32) -> f64),
        entry!(0x00dc_b430, struct_array_object_as_half(u32, u32, u32) -> u32),
        entry!(0x00dc_b460, struct_array_object_as_object(u32, u32) -> u32),
        entry!(
            0x00dc_b490,
            struct_array_implementation_swizzle_object_member(u32, u32) -> u32
        ),
        entry!(0x00dc_b4b0, hk_serialize_multi_map_remove_by_index(u32, u32, i32) -> i32),
        entry!(0x00dc_b550, struct_array_implementation_add_member(u32, u32) -> i32),
        entry!(0x00dc_b5d0, struct_array_implementation_find_or_add(u32, u32) -> i32),
        entry!(0x00dc_b650, struct_array_object_assign(u32, u32, u32)),
        entry!(0x00dc_b690, struct_array_object_assign_ov2(u32, u32, u32)),
        entry!(0x00dc_b6c0, struct_array_object_assign_ov3(u32, u32, u32)),
        entry!(0x00dc_b6f0, struct_array_object_assign_ov4(u32, u32, u32)),
        entry!(0x00dc_b720, struct_array_object_assign_ov5(u32, u32, f32)),
        entry!(0x00dc_b760, struct_array_object_assign_ov6(u32, u32, u32)),
        entry!(
            0x00dc_b790,
            struct_array_object_assign_ov7(u32, u32, u32, u32)
        ),
        entry!(0x00dc_b7c0, struct_array_object_assign_ov8(u32, u32, i8)),
        entry!(0x00dc_b7f0, struct_array_object_assign_ov9(u32, u32, i16)),
        entry!(0x00dc_b820, struct_array_object_assign_ov10(u32, u32, u32)),
        entry!(0x00dc_b850, struct_array_object_assign_ov11(u32, u32, u64)),
        entry!(0x00dc_b890, fn_00dcb890(u32)),
        entry!(0x00dc_b990, fn_00dcb990(u32)),
        entry!(
            0x00dc_ba40,
            hk_data_object_dict_read_access(Ptr<HkDataObjectDict>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dc_bbf0,
            hk_data_object_dict_as_array(Ptr<HkDataObjectDict>, u32) -> u32
        ),
        entry!(
            0x00dc_bc20,
            hk_data_object_dict_as_string(Ptr<HkDataObjectDict>, u32) -> u32
        ),
        entry!(
            0x00dc_bc50,
            hk_data_object_dict_as_int(Ptr<HkDataObjectDict>, u32) -> u64
        ),
        entry!(
            0x00dc_bca0,
            hk_data_object_dict_as_object(Ptr<HkDataObjectDict>, u32) -> u32
        ),
        entry!(
            0x00dc_bcd0,
            hk_data_object_dict_as_vec(Ptr<HkDataObjectDict>, u32, u32) -> u32
        ),
        entry!(
            0x00dc_bd10,
            hk_data_object_dict_as_real(Ptr<HkDataObjectDict>, u32) -> f64
        ),
        entry!(
            0x00dc_bd40,
            hk_data_object_dict_assign_ov8(Ptr<HkDataObjectDict>, u32, u32)
        ),
        entry!(0x00dc_bdd0, object_tracker_untrack_class(u32, u32)),
        entry!(0x00dc_bfc0, fn_00dcbfc0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dc9a20_frees_only_when_bit_zero_is_set() {
        check(0x00dc_9a20, 0x00dc_96a0);
    }

    #[test]
    fn deleting_destructor_00dca190_frees_only_when_bit_zero_is_set() {
        check(0x00dc_a190, 0x00dc_a0f0);
    }

    #[test]
    fn deleting_destructor_00dca720_frees_only_when_bit_zero_is_set() {
        check(0x00dc_a720, 0x00dc_a6b0);
    }

    #[test]
    fn deleting_destructor_00dcb960_frees_only_when_bit_zero_is_set() {
        check(0x00dc_b960, 0x00dc_b890);
    }

    #[test]
    fn deleting_destructor_00dcbf90_frees_only_when_bit_zero_is_set() {
        check(0x00dc_bf90, 0x00dc_8080);
    }

    #[test]
    fn deleting_destructor_00dcc100_frees_only_when_bit_zero_is_set() {
        check(0x00dc_c100, 0x00dc_bfc0);
    }
}

#[cfg(test)]
mod translated_tests {
    use super::*;

    const TUP: u32 = 0x0900_1000;
    const ELS: u32 = 0x0900_2000;
    const VWS: u32 = 0x0900_3000;
    const SRC: u32 = 0x0900_4000;
    const FREE_SLOT: u32 = 0x0900_5008;
    const BUF_FREE_SLOT: u32 = 0x0900_5010;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn fresh() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument words of every call to `addr`, in order.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(a, _)| *a == addr)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn words(e: &Engine, addr: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(addr + 4 * i)).collect()
    }

    /// A vtable whose slot `s` (every word up to 0x80) is a no-op double at `base + s`.
    fn stub_vtable(e: &mut Engine, base: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        vtable
    }

    fn stub_object(e: &mut Engine, base: u32, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = stub_vtable(e, base);
        e.mem.set_u32(object, vtable);
        object
    }

    fn grow(e: &mut Engine, array: u32, element_size: u32, capacity: u32) {
        let data = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let fresh_block = e.mem.alloc(capacity * element_size);
        for i in 0..size * element_size {
            let byte = e.mem.u8(data + i);
            e.mem.set_u8(fresh_block + i, byte);
        }
        e.mem.set_u32(array, fresh_block);
        e.mem.set_u32(array + 8, capacity);
    }

    /// The memory router double and its allocator (slot 8 at `FREE_SLOT`, slot 0x10 at
    /// `BUF_FREE_SLOT`), with doubles of `hkArrayUtil::_reserve` and `memcpy`.
    fn router(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        e.mem.set_u32(vtable + 0x10, BUF_FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router_block = e.mem.alloc(0x20);
        e.mem.set_u32(router_block + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router_block));
        e.register(FREE_SLOT, |_, _| Ret::default());
        e.register(BUF_FREE_SLOT, |_, _| Ret::default());
        e.register(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[3], a[2]);
            Ret::default()
        });
        e.register(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
        allocator
    }

    /// A tuple array (tuple size 3) with its embedded view; returns `(view, tuples, elements)`.
    fn view_world(e: &mut Engine) -> (u32, u32, u32) {
        let tuples = stub_object(e, TUP, 0x28);
        let elements = stub_object(e, ELS, 0x20);
        let vtable = stub_vtable(e, VWS);
        e.mem.set_u32(tuples + 0xc, vtable);
        e.mem.set_u32(tuples + 0x18, tuples);
        e.mem.set_u32(tuples + 0x20, elements);
        e.mem.set_u32(tuples + 0x24, 3);
        e.mem.set_i32(tuples + 0x1c, 10);
        (tuples + 0xc, tuples, elements)
    }

    #[test]
    fn view_get_type_toggles_the_array_bit() {
        let mut e = fresh();
        let (view, tuples, _) = view_world(&mut e);
        e.register(TUP + 4, |_, _| ret(0x2a));
        assert_eq!(e.call(0x00dc_6fc0, &args![view]).u32(), 0x0a);
        assert_eq!(calls(&e, TUP + 4), vec![vec![tuples]]);
    }

    #[test]
    fn view_clear_calls_slot_8_of_the_elements() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_6fd0, &args![view]);
        assert_eq!(calls(&e, ELS + 8), vec![vec![elements]]);
    }

    #[test]
    fn view_get_size_is_the_tuple_size() {
        let mut e = fresh();
        let (view, _, _) = view_world(&mut e);
        assert_eq!(e.call(0x00dc_6fe0, &args![view]).i32(), 3);
    }

    #[test]
    fn view_set_all_stores_each_byte_through_set_int() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        let values = e.mem.alloc(8);
        e.mem.write(values, &[5, 6, 7]);
        e.call(0x00dc_6ff0, &args![view, values, 3i32]);
        assert_eq!(
            calls(&e, ELS + 0x44),
            vec![
                vec![elements, 10, 5],
                vec![elements, 11, 6],
                vec![elements, 12, 7]
            ]
        );
    }

    #[test]
    fn view_set_all_ov2_stores_each_float_through_set_real() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        let values = e.mem.alloc(8);
        e.mem.set_u32(values, 0x3f80_0000);
        e.mem.set_u32(values + 4, 0x7f80_0001);
        e.call(0x00dc_7030, &args![view, values, 2i32]);
        assert_eq!(
            calls(&e, ELS + 0x34),
            vec![
                vec![elements, 10, 0x3f80_0000],
                vec![elements, 11, 0x7fc0_0001]
            ]
        );
    }

    #[test]
    fn view_as_vec_forwards_slot_0x20() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x20, |_, _| ret(0xabcd));
        assert_eq!(e.call(0x00dc_7070, &args![view, 2i32]).u32(), 0xabcd);
        assert_eq!(calls(&e, ELS + 0x20), vec![vec![elements, 12]]);
    }

    #[test]
    fn view_set_vec_forwards_slot_0x24() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7090, &args![view, 2i32, 0x55u32]);
        assert_eq!(calls(&e, ELS + 0x24), vec![vec![elements, 12, 0x55]]);
    }

    #[test]
    fn view_as_string_forwards_slot_0x28() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x28, |_, _| ret(0x77));
        assert_eq!(e.call(0x00dc_70c0, &args![view, 1i32]).u32(), 0x77);
        assert_eq!(calls(&e, ELS + 0x28), vec![vec![elements, 11]]);
    }

    #[test]
    fn view_set_string_forwards_slot_0x2c() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_70e0, &args![view, 1i32, 0x66u32]);
        assert_eq!(calls(&e, ELS + 0x2c), vec![vec![elements, 11, 0x66]]);
    }

    #[test]
    fn view_as_real_passes_the_st0_result_through() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x30, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00dc_7110, &args![view, 0i32]).f64(), 2.5);
        assert_eq!(calls(&e, ELS + 0x30), vec![vec![elements, 10]]);
    }

    #[test]
    fn view_set_real_passes_the_float_as_quieted_bits() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7130, &args![view, 1i32, 1.5f32]);
        e.call(0x00dc_7130, &args![view, 2i32, f32::from_bits(0x7f80_0001)]);
        assert_eq!(
            calls(&e, ELS + 0x34),
            vec![
                vec![elements, 11, 1.5f32.to_bits()],
                vec![elements, 12, 0x7fc0_0001]
            ]
        );
    }

    #[test]
    fn view_as_int_forwards_slot_0x40() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x40, |_, _| ret(9));
        assert_eq!(e.call(0x00dc_7160, &args![view, 4i32]).u32(), 9);
        assert_eq!(calls(&e, ELS + 0x40), vec![vec![elements, 14]]);
    }

    #[test]
    fn view_set_int_forwards_slot_0x44() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7180, &args![view, 4i32, 9u32]);
        assert_eq!(calls(&e, ELS + 0x44), vec![vec![elements, 14, 9]]);
    }

    #[test]
    fn view_as_int64_passes_both_result_words_through() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x48, |_, _| Ret {
            eax: 1,
            edx: 2,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00dc_71b0, &args![view, 0i32]).u64(), 0x2_0000_0001);
        assert_eq!(calls(&e, ELS + 0x48), vec![vec![elements, 10]]);
    }

    #[test]
    fn view_set_int64_forwards_both_words() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_71d0, &args![view, 0i32, 7u32, 8u32]);
        assert_eq!(calls(&e, ELS + 0x4c), vec![vec![elements, 10, 7, 8]]);
    }

    #[test]
    fn view_as_object_forwards_slot_0x50() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x50, |_, _| ret(0x31));
        assert_eq!(e.call(0x00dc_7200, &args![view, 1i32]).u32(), 0x31);
        assert_eq!(calls(&e, ELS + 0x50), vec![vec![elements, 11]]);
    }

    #[test]
    fn view_set_object_forwards_slot_0x54() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7220, &args![view, 1i32, 0x32u32]);
        assert_eq!(calls(&e, ELS + 0x54), vec![vec![elements, 11, 0x32]]);
    }

    #[test]
    fn view_as_array_forwards_slot_0x58() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x58, |_, _| ret(0x41));
        assert_eq!(e.call(0x00dc_7250, &args![view, 2i32]).u32(), 0x41);
        assert_eq!(calls(&e, ELS + 0x58), vec![vec![elements, 12]]);
    }

    #[test]
    fn view_set_array_forwards_slot_0x5c() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7270, &args![view, 2i32, 0x42u32]);
        assert_eq!(calls(&e, ELS + 0x5c), vec![vec![elements, 12, 0x42]]);
    }

    #[test]
    fn tuples_get_type_sets_the_array_bit() {
        let mut e = fresh();
        let (_, tuples, _) = view_world(&mut e);
        e.register(ELS + 4, |_, _| ret(5));
        assert_eq!(e.call(0x00dc_89d0, &args![tuples]).u32(), 0x25);
    }

    #[test]
    fn tuples_get_class_forwards_slot_0x18() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.register(ELS + 0x18, |_, _| ret(0x99));
        assert_eq!(e.call(0x00dc_89e0, &args![tuples]).u32(), 0x99);
        assert_eq!(calls(&e, ELS + 0x18), vec![vec![elements]]);
    }

    #[test]
    fn tuples_clear_forwards_slot_8() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.call(0x00dc_89f0, &args![tuples]);
        assert_eq!(calls(&e, ELS + 8), vec![vec![elements]]);
    }

    #[test]
    fn tuples_reserve_multiplies_by_the_tuple_size() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.call(0x00dc_8a00, &args![tuples, 3i32]);
        assert_eq!(calls(&e, ELS + 0xc), vec![vec![elements, 9]]);
    }

    #[test]
    fn tuples_set_size_multiplies_by_the_tuple_size() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.call(0x00dc_8a20, &args![tuples, 3i32]);
        assert_eq!(calls(&e, ELS + 0x10), vec![vec![elements, 9]]);
    }

    #[test]
    fn tuples_get_size_divides_by_the_tuple_size() {
        let mut e = fresh();
        let (_, tuples, _) = view_world(&mut e);
        e.register(ELS + 0x14, |_, _| ret(12));
        assert_eq!(e.call(0x00dc_8a40, &args![tuples]).i32(), 4);
    }

    #[test]
    #[should_panic(expected = "IDIV faults")]
    fn tuples_get_size_with_a_zero_tuple_size_faults() {
        let mut e = fresh();
        let (_, tuples, _) = view_world(&mut e);
        e.mem.set_u32(tuples + 0x24, 0);
        e.call(0x00dc_8a40, &args![tuples]);
    }

    #[test]
    fn tuples_as_array_positions_the_view() {
        let mut e = fresh();
        let (view, tuples, _) = view_world(&mut e);
        assert_eq!(e.call(0x00dc_8a60, &args![tuples, 2i32]).u32(), view);
        assert_eq!(e.mem.i32(tuples + 0x1c), 6);
    }

    #[test]
    fn tuples_set_array_copies_ints_and_reals_through_the_view() {
        let mut e = fresh();
        let (view, tuples, _) = view_world(&mut e);
        let source = stub_object(&mut e, SRC, 0x20);
        e.register(SRC + 4, |_, _| ret(1));
        e.register(SRC + 0x40, |_, _| ret(0x33));
        e.call(0x00dc_8a80, &args![tuples, 3i32, source]);
        assert_eq!(e.mem.i32(tuples + 0x1c), 9);
        assert_eq!(
            calls(&e, VWS + 0x44),
            vec![
                vec![view, 0, 0x33],
                vec![view, 1, 0x33],
                vec![view, 2, 0x33]
            ]
        );
        e.register(SRC + 4, |_, _| ret(3));
        e.register(SRC + 0x30, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.call(0x00dc_8a80, &args![tuples, 0i32, source]);
        assert_eq!(
            calls(&e, VWS + 0x34),
            vec![
                vec![view, 0, 0x3fc0_0000],
                vec![view, 1, 0x3fc0_0000],
                vec![view, 2, 0x3fc0_0000]
            ]
        );
        e.register(SRC + 4, |_, _| ret(11));
        e.call(0x00dc_8a80, &args![tuples, 0i32, source]);
        assert_eq!(calls(&e, VWS + 0x44).len(), 3);
    }

    #[test]
    fn object_dict_is_valid_compares_with_the_member_count() {
        let mut e = fresh();
        let object = e.mem.alloc(0x1c);
        e.mem.set_u32(object + 0x14, 2);
        assert!(e.call(0x00dc_9530, &args![object, 1i32]).bool());
        assert!(!e.call(0x00dc_9530, &args![object, 2i32]).bool());
        assert!(e.call(0x00dc_9530, &args![object, -1i32]).bool());
    }

    #[test]
    fn object_dict_get_member_name_reads_the_record_name() {
        let mut e = fresh();
        let object = e.mem.alloc(0x1c);
        let records = e.mem.alloc(0x20);
        e.mem.set_u32(records + 0x10, 0xbeef);
        e.mem.set_u32(object + 0x10, records);
        assert_eq!(e.call(0x00dc_9540, &args![object, 1i32]).u32(), 0xbeef);
    }

    #[test]
    fn object_dict_get_member_value_stores_the_object_and_name() {
        let mut e = fresh();
        let object = e.mem.alloc(0x1c);
        let records = e.mem.alloc(0x20);
        e.mem.set_u32(records + 0x10, 0xbeef);
        e.mem.set_u32(object + 0x10, records);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_9550, &args![object, out, 1i32]).u32(), out);
        assert_eq!(words(&e, out, 2), vec![object, 0xbeef]);
    }

    #[test]
    fn vec_array_construct_makes_an_empty_array_with_the_stride() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x00dc_9a50, &args![this, 12i32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_6f4c);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(e.mem.i32(this + 0x18), 0xc);
    }

    #[test]
    fn vec_array_get_type_maps_the_stride() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        for (stride, expected) in [(4, 4), (0xc, 6), (0x10, 7), (8, 0)] {
            e.mem.set_u32(this + 0x18, stride);
            assert_eq!(e.call(0x00dc_9a80, &args![this]).u32(), expected);
        }
    }

    #[test]
    fn vec_array_as_vec_is_the_address_of_the_vector() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x18, 3);
        assert_eq!(e.call(0x00dc_9ab0, &args![this, 2i32]).u32(), 0x1018);
    }

    #[test]
    fn vec_array_set_vec_copies_the_floats_quieted() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x18, 2);
        let values = e.mem.alloc(8);
        e.mem.set_u32(values, 0x3f80_0000);
        e.mem.set_u32(values + 4, 0x7f80_0001);
        e.call(0x00dc_9ad0, &args![this, 1i32, values]);
        assert_eq!(words(&e, data, 4), vec![0, 0, 0x3f80_0000, 0x7fc0_0001]);
    }

    #[test]
    fn vec_array_get_size_divides_by_the_stride() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 12);
        e.mem.set_u32(this + 0x18, 4);
        assert_eq!(e.call(0x00dc_9b00, &args![this]).i32(), 3);
    }

    #[test]
    #[should_panic(expected = "IDIV faults")]
    fn vec_array_get_size_with_a_zero_stride_faults() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 12);
        e.call(0x00dc_9b00, &args![this]);
    }

    #[test]
    fn vec_array_clear_zeroes_the_size() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 5);
        e.call(0x00dc_9b10, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn vec_array_reserve_asks_for_stride_times_count() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        e.mem.set_u32(this + 0x18, 2);
        e.call(0x00dc_9b20, &args![this, 3i32]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![allocator, this + 0xc, 6, 4]]
        );
    }

    #[test]
    fn unnamed_function_9bc0_answers_8() {
        let mut e = fresh();
        assert_eq!(e.call(0x00dc_9bc0, &args![]).u32(), 8);
    }

    #[test]
    fn basic_array_reserves_use_their_element_sizes() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        e.call(0x00dc_9cd0, &args![this, 5i32]);
        e.call(0x00dc_9d70, &args![this, 20i32]);
        e.call(0x00dc_9e20, &args![this, 40i32]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![
                vec![allocator, this + 0xc, 5, 1],
                vec![allocator, this + 0xc, 20, 8],
                vec![allocator, this + 0xc, 40, 4]
            ]
        );
    }

    #[test]
    fn vec_array_deleting_destructor_frees_the_data_and_the_block() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u16(this + 4, 0x20);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 4);
        assert_eq!(e.call(0x00dc_9fd0, &args![this, 1u8]).u32(), this);
        assert_eq!(calls(&e, BUF_FREE_SLOT), vec![vec![allocator, data, 16]]);
        assert_eq!(calls(&e, FREE_SLOT), vec![vec![allocator, this, 0x20]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
        e.call(0x00dc_9fd0, &args![this, 0u8]);
        assert_eq!(calls(&e, FREE_SLOT).len(), 1);
    }

    #[test]
    fn pointer_array_construct_takes_a_reference_of_the_class() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let class = e.mem.alloc(0x10);
        e.mem.set_u32(class + 8, 1);
        assert_eq!(e.call(0x00dc_a040, &args![this, class]).u32(), this);
        assert_eq!(e.mem.u32(class + 8), 2);
        assert_eq!(e.mem.u32(this + 0x18), class);
        assert_eq!(e.mem.u32(this), 0x010d_6fe4);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
        let other = e.mem.alloc(0x20);
        e.call(0x00dc_a040, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other + 0x18), 0);
    }

    #[test]
    fn pointer_array_as_object_reads_the_element() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 7);
        e.mem.set_u32(data + 4, 8);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a080, &args![this, 1i32]).u32(), 8);
    }

    #[test]
    fn pointer_array_set_object_swaps_the_references() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        let old = stub_object(&mut e, SRC, 0x10);
        let fresh_object = e.mem.alloc(0x10);
        e.mem.set_u32(old + 8, 2);
        e.mem.set_u32(fresh_object + 8, 1);
        e.mem.set_u32(data, old);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a090, &args![this, 0i32, fresh_object]);
        assert_eq!(e.mem.u32(old + 8), 1);
        assert_eq!(e.mem.u32(fresh_object + 8), 2);
        assert_eq!(e.mem.u32(data), fresh_object);
        assert!(calls(&e, SRC).is_empty());
        let newer = e.mem.alloc(0x10);
        e.call(0x00dc_a090, &args![this, 0i32, newer]);
        assert_eq!(e.mem.u32(fresh_object + 8), 1);
        e.mem.set_u32(fresh_object + 8, 1);
        e.mem.set_u32(data, old);
        e.mem.set_u32(old + 8, 1);
        e.call(0x00dc_a090, &args![this, 0i32, 0u32]);
        assert_eq!(calls(&e, SRC), vec![vec![old, 1]]);
        assert_eq!(e.mem.u32(data), 0);
    }

    #[test]
    fn pointer_array_destructor_body_releases_the_elements_and_the_class() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        let element = e.mem.alloc(0x10);
        let class = stub_object(&mut e, SRC, 0x10);
        e.mem.set_u32(element + 8, 2);
        e.mem.set_u32(class + 8, 1);
        e.mem.set_u32(data, element);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x18, class);
        e.call(0x00dc_a0f0, &args![this]);
        assert_eq!(e.mem.u32(element + 8), 1);
        assert_eq!(calls(&e, SRC), vec![vec![class, 1]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&e, BUF_FREE_SLOT), vec![vec![allocator, data, 8]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
    }

    #[test]
    fn byte_array_construct_makes_an_empty_array() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_a1c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_707c);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn byte_array_as_int_reads_a_byte() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.write(data, &[1, 0xfe]);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a1e0, &args![this, 1i32]).u32(), 0xfe);
    }

    #[test]
    fn byte_array_set_int_stores_the_low_byte() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a1f0, &args![this, 1i32, 0xabu8]);
        assert_eq!(e.mem.u8(data + 1), 0xab);
    }

    #[test]
    fn byte_array_set_all_copies_the_bytes() {
        let mut e = fresh();
        router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[1, 2, 3]);
        e.call(0x00dc_a210, &args![this, source, 3i32]);
        assert_eq!(e.mem.i32(this + 0x10), 3);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(
            [e.mem.u8(data), e.mem.u8(data + 1), e.mem.u8(data + 2)],
            [1, 2, 3]
        );
    }

    #[test]
    fn int_array_construct_makes_an_empty_array() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_a260, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_7114);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn int_array_as_int_reads_the_low_word() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 0x1234);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a280, &args![this, 1i32]).u32(), 0x1234);
    }

    #[test]
    fn int_array_set_int_sign_extends() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a290, &args![this, 1i32, -2i32]);
        assert_eq!(words(&e, data + 8, 2), vec![0xffff_fffe, 0xffff_ffff]);
    }

    #[test]
    fn int_array_as_int64_reads_both_words() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 1);
        e.mem.set_u32(data + 12, 2);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a2b0, &args![this, 1i32]).u64(), 0x2_0000_0001);
    }

    #[test]
    fn int_array_set_int64_stores_both_words() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a2d0, &args![this, 1i32, 5u32, 6u32]);
        assert_eq!(words(&e, data + 8, 2), vec![5, 6]);
    }

    /// Runs one of the three `setAll` variants on the bytes `[0, 5, 0x80]` and returns the
    /// six words of the array.
    fn int_set_all(address: u32) -> Vec<u32> {
        let mut e = fresh();
        router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[0, 5, 0x80]);
        e.call(address, &args![this, source, 3i32]);
        assert_eq!(e.mem.i32(this + 0x10), 3);
        let data = e.mem.u32(this + 0xc);
        words(&e, data, 6)
    }

    #[test]
    fn int_array_set_all_stores_booleans() {
        assert_eq!(int_set_all(0x00dc_a2f0), vec![0, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn int_array_set_all_ov2_sign_extends_the_bytes() {
        assert_eq!(
            int_set_all(0x00dc_a360),
            vec![0, 0, 5, 0, 0xffff_ff80, 0xffff_ffff]
        );
    }

    #[test]
    fn int_array_set_all_ov3_zero_extends_the_bytes() {
        assert_eq!(int_set_all(0x00dc_a3c0), vec![0, 0, 5, 0, 0x80, 0]);
    }
}

#[cfg(test)]
mod dict_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    const ALLOCATE: u32 = 0x0900_e004;
    const FREE: u32 = 0x0900_e008;
    const TEMP_ALLOCATE: u32 = 0x0900_e10c;
    const BUFFER_FREE: u32 = 0x0900_e010;
    const TEMP_BUFFER_FREE: u32 = 0x0900_e110;
    const ANY_SLOT: u32 = 0x0900_f000;
    /// Slots of the pointer map's table (`key` word, `value` word), as the doubles lay it out.
    const MAP_SLOTS: u32 = 16;

    /// What the doubles of the string maps hold: for each map address its entries
    /// `(text, key pointer, value)` in insertion order.
    type StringMaps = Rc<RefCell<HashMap<u32, Vec<(String, u32, u32)>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn fresh() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument words of every call to `addr`, in order.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(a, _)| *a == addr)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// A zero-terminated copy of `text` in memory.
    fn text(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        for (i, byte) in text.bytes().enumerate() {
            e.mem.set_u8(block + i as u32, byte);
        }
        block
    }

    fn read_text(e: &Engine, address: u32) -> String {
        let mut out = String::new();
        let mut at = address;
        while e.mem.u8(at) != 0 {
            out.push(char::from(e.mem.u8(at)));
            at += 1;
        }
        out
    }

    fn grow(e: &mut Engine, array: u32, element_size: u32, capacity: u32) {
        let data = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let block = e.mem.alloc(capacity * element_size);
        for i in 0..size * element_size {
            let byte = e.mem.u8(data + i);
            e.mem.set_u8(block + i, byte);
        }
        e.mem.set_u32(array, block);
        e.mem.set_u32(array + 8, capacity);
    }

    /// The memory router (`router + 0` the stack allocator's top, `+0xc` the temporary allocator,
    /// `+0x10` the allocator) with allocators whose allocations come from `Mem` and whose frees
    /// only log, plus doubles of the Havok array growth, `memcpy`, `memset`, `hkString::strDup` /
    /// `strFree` and the reference counting. Returns `(router block, allocator, stack region)`.
    fn basics(e: &mut Engine) -> (u32, u32, u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        e.mem.set_u32(vtable + 8, FREE);
        e.mem.set_u32(vtable + 0x10, BUFFER_FREE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let temp_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(temp_vtable + 0xc, TEMP_ALLOCATE);
        e.mem.set_u32(temp_vtable + 0x10, TEMP_BUFFER_FREE);
        let temp_allocator = e.mem.alloc(8);
        e.mem.set_u32(temp_allocator, temp_vtable);
        let stack = e.mem.alloc(0x2000);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, stack);
        e.mem.set_u32(block + 0xc, temp_allocator);
        e.mem.set_u32(block + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(block));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[1])));
        e.register(TEMP_ALLOCATE, |e, a| {
            let wanted = e.mem.u32(a[1]);
            ret(e.mem.alloc(wanted))
        });
        for address in [FREE, BUFFER_FREE, TEMP_BUFFER_FREE] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ARRAY_GROW, |e, a| {
            let capacity = e.mem.u32(a[1] + 8) & 0x3fff_ffff;
            grow(e, a[1], a[2], (capacity * 2).max(4));
            Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[3], a[2]);
            Ret::default()
        });
        e.register(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
        e.register(MEM_SET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(STRING_DUP, |e, a| {
            let copy = read_text(e, a[0]);
            ret(text(e, &copy))
        });
        for address in [STRING_FREE, ADD_REFERENCE, REMOVE_REFERENCE] {
            e.register(address, |_, _| Ret::default());
        }
        (block, allocator, stack)
    }

    /// Doubles of the string map (`hkCachedHashMap<hkStringMapOperations,...>`): entries are found
    /// by the text of their key, iteration is in insertion order (iterator = position).
    fn string_maps(e: &mut Engine) -> StringMaps {
        let state: StringMaps = Rc::default();
        let maps = Rc::clone(&state);
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |e, a| {
            let key = read_text(e, a[1]);
            let found = maps
                .borrow()
                .get(&a[0])
                .and_then(|m| m.iter().find(|entry| entry.0 == key).map(|entry| entry.2));
            ret(found.unwrap_or(a[2]))
        });
        let maps = Rc::clone(&state);
        e.register_double(STRING_MAP_INSERT, move |e, a| {
            let key = read_text(e, a[1]);
            let mut maps = maps.borrow_mut();
            let entries = maps.entry(a[0]).or_default();
            match entries.iter_mut().find(|entry| entry.0 == key) {
                Some(entry) => {
                    entry.1 = a[1];
                    entry.2 = a[2];
                }
                None => entries.push((key, a[1], a[2])),
            }
            Ret::default()
        });
        let maps = Rc::clone(&state);
        e.register_double(STRING_MAP_REMOVE, move |e, a| {
            let key = read_text(e, a[1]);
            if let Some(entries) = maps.borrow_mut().get_mut(&a[0]) {
                entries.retain(|entry| entry.0 != key);
            }
            Ret::default()
        });
        let maps = Rc::clone(&state);
        e.register_double(STRING_MAP_FIND_ITERATOR, move |e, a| {
            let key = read_text(e, a[1]);
            let maps = maps.borrow();
            let entries = maps.get(&a[0]);
            let position = entries.and_then(|m| m.iter().position(|entry| entry.0 == key));
            ret(position.unwrap_or(entries.map_or(0, Vec::len)) as u32)
        });
        e.register(STRING_MAP_GET_ITERATOR, |_, _| ret(0));
        let maps = Rc::clone(&state);
        e.register_double(STRING_MAP_IS_VALID, move |e, a| {
            let length = maps.borrow().get(&a[0]).map_or(0, Vec::len);
            e.mem.set_u8(a[1], u8::from((a[2] as usize) < length));
            Ret::default()
        });
        let maps = Rc::clone(&state);
        e.register_double(STRING_MAP_GET_VALUE, move |_, a| {
            ret(maps.borrow()[&a[0]][a[1] as usize].2)
        });
        e.register(STRING_MAP_GET_NEXT, |_, a| ret(a[1] + 1));
        for address in [STRING_MAP_CONSTRUCT, STRING_MAP_DESTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        state
    }

    /// Puts an entry `(key text, key pointer, value)` in the string map at `map`.
    fn put_entry(maps: &StringMaps, map: u32, key: &str, pointer: u32, value: u32) {
        maps.borrow_mut()
            .entry(map)
            .or_default()
            .push((key.to_string(), pointer, value));
    }

    /// A string map entry's value by key text.
    fn map_value(maps: &StringMaps, map: u32, key: &str) -> Option<u32> {
        maps.borrow()
            .get(&map)
            .and_then(|m| m.iter().find(|entry| entry.0 == key).map(|entry| entry.2))
    }

    /// The slot of `key` in the pointer map at `map` (its first word is the table).
    fn table_find(e: &Engine, map: u32, key: u32) -> Option<u32> {
        let table = e.mem.u32(map);
        if key == 0 {
            return None;
        }
        (0..MAP_SLOTS).find(|slot| e.mem.u32(table + slot * 8) == key)
    }

    /// Doubles of the pointer map (`hkMapBase<unsigned long,unsigned long>`) the multi-maps keep
    /// their key to first-entry index in: its table is `MAP_SLOTS` slots of `(key, value)` at the
    /// map's first word, an empty slot has key 0.
    fn pointer_maps(e: &mut Engine) {
        e.register(POINTER_MAP_GET_WITH_DEFAULT, |e, a| {
            match table_find(e, a[0], a[1]) {
                Some(slot) => ret(e.mem.u32(e.mem.u32(a[0]) + slot * 8 + 4)),
                None => ret(a[2]),
            }
        });
        e.register(POINTER_MAP_FIND_KEY, |e, a| {
            ret(table_find(e, a[0], a[1]).unwrap_or(MAP_SLOTS))
        });
        e.register(POINTER_MAP_REMOVE, |e, a| {
            let table = e.mem.u32(a[0]);
            e.mem.set_u32(table + a[1] * 8, 0);
            Ret::default()
        });
        for address in [
            POINTER_MAP_RESET,
            POINTER_MAP_CLEAR_AND_DEALLOCATE,
            POINTER_MAP_AFTER_CLEAR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// Makes an empty multi-map at `at` (0x1c bytes): 32 entries of room, an empty pointer-map
    /// table, the free list empty (`-1`).
    fn multi_map_at(e: &mut Engine, at: u32) {
        let entries = e.mem.alloc(0x100);
        e.mem.set_u32(at, entries);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 32);
        let table = e.mem.alloc(8 * MAP_SLOTS);
        e.mem.set_u32(at + multi_map::MAP, table);
        e.mem.set_i32(at + multi_map::FREE_HEAD, -1);
    }

    /// Appends `value` to the chain of `key` of the multi-map `multi`.
    fn chain_add(e: &mut Engine, multi: u32, key: u32, value: u32) {
        let entries = e.mem.u32(multi);
        let size = e.mem.u32(multi + 4);
        e.mem.set_u32(entries + size * 8, value);
        e.mem.set_i32(entries + size * 8 + 4, -1);
        e.mem.set_u32(multi + 4, size + 1);
        let map = multi + multi_map::MAP;
        let table = e.mem.u32(map);
        match table_find(e, map, key) {
            None => {
                let slot = (0..MAP_SLOTS)
                    .find(|slot| e.mem.u32(table + slot * 8) == 0)
                    .expect("a free slot");
                e.mem.set_u32(table + slot * 8, key);
                e.mem.set_u32(table + slot * 8 + 4, size);
            }
            Some(slot) => {
                let mut index = e.mem.u32(table + slot * 8 + 4);
                while e.mem.i32(entries + index * 8 + 4) != -1 {
                    index = e.mem.i32(entries + index * 8 + 4) as u32;
                }
                e.mem.set_u32(entries + index * 8 + 4, size);
            }
        }
    }

    /// The values of the chain of `key` in the multi-map, read through the entries.
    fn chain_values(e: &Engine, multi: u32, key: u32) -> Vec<u32> {
        let map = multi + multi_map::MAP;
        let entries = e.mem.u32(multi);
        let mut out = Vec::new();
        let mut index = table_find(e, map, key)
            .map(|slot| e.mem.u32(e.mem.u32(map) + slot * 8 + 4))
            .filter(|head| *head != u32::MAX);
        while let Some(i) = index {
            out.push(e.mem.u32(entries + i * 8));
            let next = e.mem.i32(entries + i * 8 + 4);
            index = if next == -1 { None } else { Some(next as u32) };
        }
        out
    }

    /// A world `{vtable, +6 kind, +8 tracker}` with its zeroed 0x88-byte tracker; returns
    /// `(world, tracker)`.
    fn make_world(e: &mut Engine) -> (u32, u32) {
        let world = e.mem.alloc(0x0c);
        let tracker = e.mem.alloc(0x88);
        e.mem.set_u32(world + 8, tracker);
        (world, tracker)
    }

    /// A world whose tracker has its three multi-maps made: the derived-class map (`+0x10`), the
    /// object map (`+0x30`) and the array map (`+0x6c`).
    fn make_world_with_maps(e: &mut Engine) -> (u32, u32) {
        let (world, tracker) = make_world(e);
        multi_map_at(e, tracker + tracker::DERIVED_MAP);
        multi_map_at(e, tracker + tracker::OBJECT_MAP);
        multi_map_at(e, tracker + tracker::ARRAY_MAP);
        (world, tracker)
    }

    /// An object `size` bytes big whose vtable slots are the given doubles (and every other slot
    /// a no-op).
    fn object_with(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + slot, ANY_SLOT);
        }
        e.register(ANY_SLOT, |_, _| Ret::default());
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// An array `{data, size, capacity word}` holding `values` (4 bytes each).
    fn word_array(e: &mut Engine, values: &[u32]) -> u32 {
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(4 * values.len().max(1) as u32);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *value);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, values.len() as u32);
        e.mem.set_u32(array + 8, values.len().max(1) as u32);
        array
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    /// An empty `hkArray` that owns nothing.
    fn empty_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        array_init_empty(e, array);
        array
    }

    /// A class dict in `world` with the given `(name, handle, type)` member records (0x18 bytes
    /// each, extra word 0), without parent.
    fn class_dict(e: &mut Engine, world: u32, members: &[(u32, u32, u32)]) -> u32 {
        let class = e.mem.alloc(0x40);
        e.mem.set_u32(class + 8, 1);
        e.mem.set_u32(class + 0xc, world);
        // Padding before the records: a search that finds nothing works on the record before them.
        let data = e.mem.alloc(0x18 + 0x18 * members.len().max(1) as u32) + 0x18;
        for (i, (name, handle, member_type)) in members.iter().enumerate() {
            let record = data + 0x18 * i as u32;
            e.mem.set_u32(record, *name);
            e.mem.set_u32(record + 4, *handle);
            e.mem.set_u32(record + 0x10, *member_type);
        }
        e.mem.set_u32(class + 0x28, data);
        e.mem.set_u32(class + 0x2c, members.len() as u32);
        e.mem.set_u32(class + 0x30, members.len().max(1) as u32);
        class
    }

    /// An `hkDataObjectDict`-shaped object of `class` with the given `(name, value)` records
    /// (0x10 bytes each, the value's first word at `+8`).
    fn dict_object(e: &mut Engine, class: u32, records: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x1c);
        e.mem.set_u32(object + 8, 1);
        e.mem.set_u32(object + 0xc, class);
        let data = e.mem.alloc(0x10 * records.len().max(1) as u32);
        for (i, (name, value)) in records.iter().enumerate() {
            e.mem.set_u32(data + 0x10 * i as u32, *name);
            e.mem.set_u32(data + 0x10 * i as u32 + 8, *value);
        }
        e.mem.set_u32(object + 0x10, data);
        e.mem.set_u32(object + 0x14, records.len() as u32);
        e.mem.set_u32(object + 0x18, records.len().max(1) as u32);
        object
    }

    fn record_names(e: &Engine, object: u32) -> Vec<u32> {
        let data = e.mem.u32(object + 0x10);
        (0..e.mem.u32(object + 0x14))
            .map(|i| e.mem.u32(data + 0x10 * i))
            .collect()
    }

    const SLOT_A: u32 = 0x0900_b000;
    const SLOT_B: u32 = 0x0900_b004;
    const SLOT_C: u32 = 0x0900_b008;
    const SLOT_D: u32 = 0x0900_b00c;
    const SLOT_E: u32 = 0x0900_b010;

    /// An object whose slot 8 answers `name` (a class: its name) and whose slot 0 and 0x10 are
    /// no-ops; `refs` is the reference count at `+8`.
    fn named_object(e: &mut Engine, size: u32, name: u32, refs: u32) -> u32 {
        e.register_double(SLOT_A, move |_, _| ret(name));
        let object = object_with(e, size, &[(8, SLOT_A)]);
        e.mem.set_u32(object + 8, refs);
        object
    }

    #[test]
    fn get_contents_wraps_the_trackers_first_object() {
        let mut e = fresh();
        let (world, tracker) = make_world(&mut e);
        e.mem.set_u32(tracker + 0x2c, 0x1234);
        e.register(DATA_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_72a0, &args![world, out]).u32(), out);
        assert_eq!(calls(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0x1234]]);
    }

    #[test]
    fn find_object_wraps_the_implementation_of_the_handle() {
        let mut e = fresh();
        let (world, _) = make_world(&mut e);
        e.register(DATA_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, 0x5678);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_72c0, &args![world, out, handle]).u32(), out);
        assert_eq!(calls(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0x5678]]);
    }

    #[test]
    fn set_class_version_stores_in_the_implementation() {
        let mut e = fresh();
        let (world, _) = make_world(&mut e);
        let class = e.mem.alloc(0x40);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, class);
        e.call(0x00dc_72e0, &args![world, handle, 7i32]);
        assert_eq!(e.mem.i32(class + 0x14), 7);
    }

    #[test]
    fn get_type_stores_one_and_returns_out() {
        let mut e = fresh();
        let (world, _) = make_world(&mut e);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00dc_72f0, &args![world, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 1);
    }

    #[test]
    fn find_class_looks_the_name_up_in_the_class_map() {
        let mut e = fresh();
        let maps = string_maps(&mut e);
        let (world, tracker) = make_world(&mut e);
        let foo = text(&mut e, "Foo");
        put_entry(&maps, tracker, "Foo", foo, 0xc1a5);
        assert_eq!(e.call(0x00dc_7300, &args![world, foo]).u32(), 0xc1a5);
        let bar = text(&mut e, "Bar");
        assert_eq!(e.call(0x00dc_7300, &args![world, bar]).u32(), 0);
        let lookups = calls(&e, STRING_MAP_GET_WITH_DEFAULT).len();
        assert_eq!(e.call(0x00dc_7300, &args![world, 0u32]).u32(), 0);
        assert_eq!(calls(&e, STRING_MAP_GET_WITH_DEFAULT).len(), lookups);
    }

    #[test]
    fn rename_class_moves_the_class_and_its_handle_to_the_new_name() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let (world, tracker) = make_world(&mut e);
        let old = text(&mut e, "Old");
        e.register_double(DATA_CLASS_GET_NAME, move |_, _| ret(old));
        let record = e.mem.alloc(0x10);
        put_entry(&maps, tracker, "Old", old, 0xc1a5);
        put_entry(&maps, tracker + 0x5c, "Old", old, record);
        let new = text(&mut e, "New");
        e.call(0x00dc_7320, &args![world, 0x1111u32, new]);
        assert_eq!(map_value(&maps, tracker, "Old"), None);
        assert_eq!(map_value(&maps, tracker, "New"), Some(0xc1a5));
        assert_eq!(map_value(&maps, tracker + 0x5c, "Old"), None);
        assert_eq!(map_value(&maps, tracker + 0x5c, "New"), Some(record));
        let interned = maps.borrow()[&(tracker + 0x4c)]
            .iter()
            .find(|entry| entry.0 == "New")
            .map(|entry| entry.1);
        assert_eq!(Some(e.mem.u32(record + 8)), interned);
    }

    #[test]
    fn find_all_classes_collects_the_class_map_values() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let (world, tracker) = make_world(&mut e);
        put_entry(&maps, tracker, "A", 0, 0xa1);
        put_entry(&maps, tracker, "B", 0, 0xb1);
        let out = empty_array(&mut e);
        e.call(0x00dc_7390, &args![world, out]);
        assert_eq!(array_words(&e, out), vec![0xa1, 0xb1]);
    }

    #[test]
    fn find_objects_by_exact_class_takes_the_objects_of_the_class_only() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let foo = text(&mut e, "Foo");
        put_entry(&maps, tracker, "Foo", foo, 0xc1a5);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, 0xc1a5, 0x11);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, 0xc1a5, 0x12);
        chain_add(&mut e, tracker + tracker::DERIVED_MAP, 0xc1a5, 0xd1);
        let out = word_array(&mut e, &[9, 9, 9]);
        e.call(0x00dc_73a0, &args![world, foo, out]);
        assert_eq!(array_words(&e, out), vec![0x11, 0x12]);
    }

    #[test]
    fn find_objects_by_base_class_adds_the_derived_classes() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let foo = text(&mut e, "Foo");
        let derived_name = text(&mut e, "Derived");
        let derived = named_object(&mut e, 0x40, derived_name, 1);
        put_entry(&maps, tracker, "Foo", foo, 0xc1a5);
        put_entry(&maps, tracker, "Derived", derived_name, derived);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, 0xc1a5, 0x11);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, derived, 0x21);
        chain_add(&mut e, tracker + tracker::DERIVED_MAP, 0xc1a5, derived);
        let out = word_array(&mut e, &[9, 9, 9]);
        e.call(0x00dc_73c0, &args![world, foo, out]);
        assert_eq!(array_words(&e, out), vec![0x11, 0x21]);
    }

    #[test]
    fn unnamed_73e0_is_the_count_over_four_plus_three() {
        let mut e = fresh();
        assert_eq!(e.call(0x00dc_73e0, &args![0x10u32]).u32(), 7);
        assert_eq!(e.call(0x00dc_73e0, &args![3u32]).u32(), 3);
    }

    /// A world with a class `C` of members `A` and `B`, an object of `C` with records `B`, `A`
    /// and a struct array of `C` with records `A`, `B`. Returns
    /// `(world, class, object, struct array, name of A, name of B, class handle)`.
    fn member_world(e: &mut Engine) -> (u32, u32, u32, u32, u32, u32, u32) {
        let maps = string_maps(e);
        pointer_maps(e);
        let (world, tracker) = make_world_with_maps(e);
        let (a, b) = (text(e, "A"), text(e, "B"));
        put_entry(&maps, tracker + 0x4c, "A", a, a);
        put_entry(&maps, tracker + 0x4c, "B", b, b);
        let class = class_dict(e, world, &[(a, 0, 8), (b, 0, 8)]);
        let cname = text(e, "C");
        put_entry(&maps, tracker, "C", cname, class);
        e.register_double(DATA_CLASS_GET_NAME, move |_, _| ret(cname));
        let object = dict_object(e, class, &[(b, 0), (a, 0)]);
        chain_add(e, tracker + tracker::OBJECT_MAP, class, object);
        let array = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x10);
        let (first, second) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_u32(first + 8, 2);
        e.mem.set_u32(second + 8, 2);
        e.mem.set_u32(records, a);
        e.mem.set_u32(records + 4, first);
        e.mem.set_u32(records + 8, b);
        e.mem.set_u32(records + 0xc, second);
        e.mem.set_u32(array + 0xc, records);
        e.mem.set_u32(array + 0x10, 2);
        chain_add(e, tracker + tracker::ARRAY_MAP, class, array);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, class);
        (world, class, object, array, a, b, handle)
    }

    #[test]
    fn rename_class_member_renames_in_objects_arrays_and_the_class() {
        let mut e = fresh();
        basics(&mut e);
        let (world, class, object, array, a, b, handle) = member_world(&mut e);
        let (raw_a, raw_z) = (text(&mut e, "A"), text(&mut e, "Z"));
        e.call(0x00dc_73f0, &args![world, handle, raw_a, raw_z]);
        let z = e.mem.u32(e.mem.u32(array + 0xc));
        assert_ne!(z, a);
        assert_eq!(read_text(&e, z), "Z");
        assert_eq!(record_names(&e, object), vec![b, z]);
        assert_eq!(e.mem.u32(e.mem.u32(array + 0xc) + 8), b);
        let members = e.mem.u32(class + 0x28);
        assert_eq!(e.mem.u32(members), z);
        assert_eq!(e.mem.u32(members + 0x18), b);
    }

    #[test]
    fn rename_class_member_without_a_match_writes_before_the_member_list() {
        let mut e = fresh();
        basics(&mut e);
        let (world, class, _, _, _, _, handle) = member_world(&mut e);
        let members = e.mem.u32(class + 0x28);
        let (raw_missing, raw_z) = (text(&mut e, "Missing"), text(&mut e, "Z"));
        e.call(0x00dc_73f0, &args![world, handle, raw_missing, raw_z]);
        let z = e.mem.u32(members - 0x18);
        assert_eq!(read_text(&e, z), "Z");
    }

    #[test]
    fn remove_class_member_removes_from_objects_arrays_and_the_class() {
        let mut e = fresh();
        basics(&mut e);
        let (world, class, object, array, _, b, handle) = member_world(&mut e);
        let raw_a = text(&mut e, "A");
        e.call(0x00dc_7600, &args![world, handle, raw_a]);
        assert_eq!(record_names(&e, object), vec![b]);
        assert_eq!(e.mem.u32(array + 0x10), 1);
        assert_eq!(e.mem.u32(e.mem.u32(array + 0xc)), b);
        assert_eq!(e.mem.u32(class + 0x2c), 1);
        assert_eq!(e.mem.u32(e.mem.u32(class + 0x28)), b);
    }

    #[test]
    fn tracker_teardown_reports_objects_still_referenced_from_outside() {
        let mut e = fresh();
        let (router, _, stack) = basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        let (_, tracker) = make_world_with_maps(&mut e);
        let class_name = text(&mut e, "C");
        let class = named_object(&mut e, 0x40, class_name, 2);
        put_entry(&maps, tracker, "C", class_name, class);
        e.register_double(SLOT_B, move |_, _| ret(class));
        let object = object_with(&mut e, 0x1c, &[(8, SLOT_B)]);
        e.mem.set_u32(object + 8, 2);
        e.mem.set_u16(object + 6, 1);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, class, object);
        e.register(SLOT_C, |_, _| Ret::default());
        let reporter = object_with(&mut e, 0x10, &[(0xc, SLOT_C)]);
        e.mem.set_u32(ERROR_REPORTER, reporter);
        e.register(OSTREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(OSTREAM_WRITE_STRING, |_, a| ret(a[0]));
        e.register(OSTREAM_WRITE_INT, |_, a| ret(a[0]));
        e.register(OSTREAM_DESTRUCT, |_, _| Ret::default());
        e.call(0x00dc_7750, &args![tracker]);
        assert_eq!(calls(&e, OSTREAM_WRITE_INT).len(), 1);
        assert_eq!(calls(&e, OSTREAM_WRITE_INT)[0][1], 1);
        let report = &calls(&e, SLOT_C)[0];
        assert_eq!(report[1..3], [1, 0x760d_3795]);
        assert_eq!(report[4..], [TEXT_SOURCE_FILE, 0x6f4]);
        let writes: Vec<u32> = calls(&e, OSTREAM_WRITE_STRING)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(
            writes,
            vec![
                TEXT_THE_OBJECT_OF_CLASS,
                class_name,
                TEXT_ABOUT_TO_BE_REMOVED,
                TEXT_FROM_OUTSIDE_THE_WORLD
            ]
        );
        assert_eq!(e.mem.u32(router), stack);
        assert_eq!(e.mem.u32(object + 8), 1);
        assert_eq!(e.mem.u32(class + 8), 1);
        for map in [
            tracker,
            tracker + tracker::INTERNED_NAMES,
            tracker + tracker::CLASS_HANDLES,
        ] {
            assert_eq!(
                calls(&e, STRING_MAP_DESTRUCT)
                    .iter()
                    .filter(|c| c[0] == map)
                    .count(),
                1
            );
        }
        assert_eq!(calls(&e, POINTER_MAP_CLEAR_AND_DEALLOCATE).len(), 3);
        assert_eq!(e.mem.u32(tracker + 0x2c), 0);
    }

    #[test]
    fn tracker_teardown_releases_handles_and_frees_the_interned_names() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        let (_, tracker) = make_world_with_maps(&mut e);
        put_entry(&maps, tracker + tracker::CLASS_HANDLES, "H", 0, 0x4a4a);
        put_entry(
            &maps,
            tracker + tracker::INTERNED_NAMES,
            "N",
            0x5b5b,
            0x5b5b,
        );
        e.call(0x00dc_7750, &args![tracker]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x4a4a]]);
        assert_eq!(calls(&e, STRING_FREE), vec![vec![0x5b5b]]);
        assert!(calls(&e, OSTREAM_CONSTRUCT).is_empty());
    }

    #[test]
    fn new_object_makes_an_object_of_the_class_and_tracks_it() {
        let mut e = fresh();
        basics(&mut e);
        let (world, tracker) = make_world(&mut e);
        let seen: Rc<RefCell<Vec<[u32; 3]>>> = Rc::default();
        let record = Rc::clone(&seen);
        e.register_double(MULTI_MAP_INSERT, move |e, a| {
            record.borrow_mut().push([a[0], a[1], e.mem.u32(a[2])]);
            Ret::default()
        });
        // The new object's vtable is the exe's `0x010d_6ee4`: its slot 8 gives the key.
        e.register(SLOT_C, |_, _| ret(0x9999));
        e.mem.set_u32(0x010d_6ee4 + 8, SLOT_C);
        let class = e.mem.alloc(0x40);
        e.mem.set_u32(class + 8, 1);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, class);
        let object = e.call(0x00dc_7c40, &args![world, handle]).u32();
        assert_eq!(e.mem.u16(object + 4), 0x1c);
        assert_eq!(e.mem.u32(object), 0x010d_6ee4);
        assert_eq!(e.mem.u32(object + 0xc), class);
        assert_eq!(e.mem.u32(object + 0x18), 0x8000_0000);
        assert_eq!(e.mem.u32(object + 8), 1);
        assert_eq!(e.mem.u32(class + 8), 2);
        assert_eq!(*seen.borrow(), vec![[tracker + 0x30, 0x9999, object]]);
        assert_eq!(e.mem.u32(tracker + 0x2c), object);
        let second = e.call(0x00dc_7c40, &args![world, handle]).u32();
        assert_ne!(second, object);
        assert_eq!(e.mem.u32(tracker + 0x2c), object);
    }

    /// A class `C` of `world` as a class dict (no members) whose virtual slots are `slots`.
    fn class_with_slots(e: &mut Engine, world: u32, slots: &[(u32, u32)]) -> u32 {
        let class = class_dict(e, world, &[]);
        let prototype = object_with(e, 8, slots);
        let vtable = e.mem.u32(prototype);
        e.mem.set_u32(class, vtable);
        class
    }

    /// Records the `(map, key, word at the value pointer)` of every `hkSerializeMultiMap::insert`.
    fn record_inserts(e: &mut Engine) -> Rc<RefCell<Vec<[u32; 3]>>> {
        let seen: Rc<RefCell<Vec<[u32; 3]>>> = Rc::default();
        let record = Rc::clone(&seen);
        e.register_double(MULTI_MAP_INSERT, move |e, a| {
            record.borrow_mut().push([a[0], a[1], e.mem.u32(a[2])]);
            Ret::default()
        });
        seen
    }

    #[test]
    fn set_class_parent_swaps_the_parent_and_moves_the_derived_entry() {
        let mut e = fresh();
        basics(&mut e);
        pointer_maps(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let derived_map = tracker + tracker::DERIVED_MAP;
        let inserts = record_inserts(&mut e);
        e.register_double(SLOT_B, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(SLOT_D, |_, _| Ret::default());
        let old_parent = object_with(&mut e, 0x40, &[(0, SLOT_D)]);
        e.mem.set_u32(old_parent + 8, 1);
        let new_parent = e.mem.alloc(0x40);
        e.mem.set_u32(new_parent + 8, 1);
        let class = object_with(&mut e, 0x40, &[(0x10, SLOT_B)]);
        e.mem.set_u32(class + 0x18, old_parent);
        chain_add(&mut e, derived_map, old_parent, class);
        let (handle, parent_handle) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(handle, class);
        e.mem.set_u32(parent_handle, new_parent);
        e.call(0x00dc_7cc0, &args![world, handle, parent_handle]);
        assert_eq!(e.mem.u32(class + 0x18), new_parent);
        assert_eq!(e.mem.u32(new_parent + 8), 2);
        assert_eq!(e.mem.u32(old_parent + 8), 0);
        assert_eq!(calls(&e, SLOT_D), vec![vec![old_parent, 1]]);
        assert_eq!(chain_values(&e, derived_map, old_parent), Vec::<u32>::new());
        assert_eq!(*inserts.borrow(), vec![[derived_map, new_parent, class]]);
    }

    #[test]
    fn set_class_parent_without_parents_only_stores() {
        let mut e = fresh();
        basics(&mut e);
        pointer_maps(&mut e);
        let (world, _) = make_world_with_maps(&mut e);
        let inserts = record_inserts(&mut e);
        e.register_double(SLOT_B, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        let class = object_with(&mut e, 0x40, &[(0x10, SLOT_B)]);
        let (handle, parent_handle) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(handle, class);
        e.call(0x00dc_7cc0, &args![world, handle, parent_handle]);
        assert_eq!(e.mem.u32(class + 0x18), 0);
        assert!(inserts.borrow().is_empty());
    }

    /// The doubles `internClassNameHandle` needs: the interning of the name for the handle's
    /// record answers the slot that holds the name itself.
    fn handle_doubles(e: &mut Engine) {
        e.register(UID_FROM_CLASS_VERSION_CACHE, |_, a| ret(a[1]));
    }

    #[test]
    fn add_class_member_adds_a_missing_member_with_a_class_handle() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        handle_doubles(&mut e);
        let (world, tracker) = make_world(&mut e);
        let name = text(&mut e, "m");
        put_entry(&maps, tracker + 0x4c, "m", name, name);
        e.register(SLOT_C, |_, _| ret(0xffff_ffff));
        let class = class_with_slots(&mut e, world, &[(0x20, SLOT_C)]);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, class);
        let kind = text(&mut e, "K");
        e.call(
            0x00dc_7d20,
            &args![world, handle, name, 7u32, kind, 0x55u32],
        );
        assert_eq!(e.mem.u32(class + 0x2c), 1);
        let record = e.mem.u32(class + 0x28);
        assert_eq!(e.mem.u32(record), name);
        assert_eq!(e.mem.u32(record + 0x10), 7);
        assert_eq!(e.mem.u32(record + 0x14), 0x55);
        let class_handle = e.mem.u32(record + 4);
        assert_eq!(e.mem.u32(class_handle + 8), kind);
        assert_eq!(e.mem.u32(class_handle), 0x010d_6da0);
        assert_eq!(map_value(&maps, tracker + 0x5c, "K"), Some(class_handle));
        assert_eq!(calls(&e, SLOT_C), vec![vec![class, name]]);
    }

    #[test]
    fn add_class_member_first_removes_an_existing_member() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        handle_doubles(&mut e);
        let (world, tracker) = make_world(&mut e);
        let name = text(&mut e, "m");
        put_entry(&maps, tracker + 0x4c, "m", name, name);
        e.register(SLOT_C, |_, _| ret(0));
        let class = class_with_slots(&mut e, world, &[(0x20, SLOT_C)]);
        let members = e.mem.alloc(0x18 + 0x18) + 0x18;
        e.mem.set_u32(members, name);
        e.mem.set_u32(class + 0x28, members);
        e.mem.set_u32(class + 0x2c, 1);
        e.mem.set_u32(class + 0x30, 1);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, class);
        let kind = text(&mut e, "K");
        e.call(0x00dc_7d20, &args![world, handle, name, 9u32, kind, 0u32]);
        assert_eq!(e.mem.u32(class + 0x2c), 1);
        assert_eq!(e.mem.u32(e.mem.u32(class + 0x28) + 0x10), 9);
    }

    #[test]
    fn cast_object_moves_the_object_to_the_chain_of_the_new_class() {
        let mut e = fresh();
        basics(&mut e);
        pointer_maps(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let object_map = tracker + tracker::OBJECT_MAP;
        let inserts = record_inserts(&mut e);
        e.register(SLOT_A, |_, _| ret(0xc1a5));
        let object = object_with(&mut e, 0x20, &[(8, SLOT_A)]);
        chain_add(&mut e, object_map, 0xc1a5, 0x7777);
        chain_add(&mut e, object_map, 0xc1a5, object);
        let (first, second) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(first, object);
        e.mem.set_u32(second, 0xd00d);
        e.call(0x00dc_7dc0, &args![world, first, second]);
        assert_eq!(*inserts.borrow(), vec![[object_map, 0xd00d, object]]);
        assert_eq!(chain_values(&e, object_map, 0xc1a5), vec![0x7777]);
    }

    #[test]
    fn cast_object_of_an_object_without_class_does_nothing() {
        let mut e = fresh();
        basics(&mut e);
        pointer_maps(&mut e);
        let (world, _) = make_world_with_maps(&mut e);
        let inserts = record_inserts(&mut e);
        e.register(SLOT_A, |_, _| ret(0));
        let object = object_with(&mut e, 0x20, &[(8, SLOT_A)]);
        let (first, second) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(first, object);
        e.call(0x00dc_7dc0, &args![world, first, second]);
        assert!(inserts.borrow().is_empty());
    }

    /// The array implementation `hkDataArrayDict_create(array_type, class, tuple_size)` makes.
    fn create(e: &mut Engine, array_type: u32, class: u32, tuple_size: u32) -> u32 {
        e.call(0x00dc_7de0, &args![array_type, class, tuple_size])
            .u32()
    }

    #[test]
    fn array_create_picks_the_implementation_by_type() {
        let mut e = fresh();
        basics(&mut e);
        let cases = [
            (1u32, 0x18u32, 0x010d_707cu32),
            (2, 0x18, 0x010d_7114),
            (3, 0x18, 0x010d_71ac),
            (4, 0x1c, 0x010d_6f4c),
            (6, 0x1c, 0x010d_6f4c),
            (7, 0x1c, 0x010d_6f4c),
            (10, 0x18, 0x010d_7244),
        ];
        for (array_type, size, vtable) in cases {
            let array = create(&mut e, array_type, 0, 0);
            assert_ne!(array, 0, "type {array_type}");
            assert_eq!(u32::from(e.mem.u16(array + 4)), size, "type {array_type}");
            assert_eq!(e.mem.u32(array), vtable, "type {array_type}");
        }
        let vector = create(&mut e, 6, 0, 0);
        assert_eq!(e.mem.u32(vector + 0x18), 0xc);
        let vector = create(&mut e, 7, 0, 0);
        assert_eq!(e.mem.u32(vector + 0x18), 0x10);
    }

    #[test]
    fn array_create_with_other_types_answers_zero() {
        let mut e = fresh();
        basics(&mut e);
        for array_type in [0u32, 5, 11, 15, 0x10] {
            assert_eq!(create(&mut e, array_type, 0, 0), 0, "type {array_type}");
        }
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn array_create_makes_pointer_arrays_for_a_class() {
        let mut e = fresh();
        basics(&mut e);
        let class = e.mem.alloc(0x40);
        e.mem.set_u32(class + 8, 1);
        let array = create(&mut e, 8, class, 0);
        assert_eq!(e.mem.u16(array + 4), 0x1c);
        assert_eq!(e.mem.u32(class + 8), 2);
    }

    #[test]
    fn array_create_makes_struct_arrays_for_a_class() {
        let mut e = fresh();
        basics(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let inserts = record_inserts(&mut e);
        e.register_double(SLOT_A, move |_, _| ret(world));
        e.register(SLOT_B, |_, _| ret(0));
        e.register(SLOT_C, |_, _| ret(0xabcd));
        let class = class_with_slots(&mut e, world, &[(4, SLOT_A), (0x24, SLOT_B)]);
        e.mem.set_u32(class + 8, 1);
        e.mem.set_u32(0x010d_73fc + 0x18, SLOT_C);
        let array = create(&mut e, 9, class, 0);
        assert_eq!(e.mem.u16(array + 4), 0x20);
        assert_eq!(e.mem.u32(array), 0x010d_73fc);
        assert_eq!(e.mem.u32(array + 0x18), class);
        assert_eq!(e.mem.u32(class + 8), 2);
        assert_eq!(
            *inserts.borrow(),
            vec![[tracker + tracker::ARRAY_MAP, 0xabcd, array]]
        );
    }

    #[test]
    fn array_create_with_the_tuple_bit_wraps_the_other_array() {
        let mut e = fresh();
        basics(&mut e);
        let tuples = create(&mut e, 0x21, 0, 3);
        assert_eq!(e.mem.u16(tuples + 4), 0x28);
        assert_eq!(e.mem.u32(tuples), 0x010d_6e4c);
        assert_eq!(e.mem.u32(tuples + 0xc), 0x010d_6db4);
        assert_eq!(e.mem.u32(tuples + 0x24), 3);
        assert_eq!(e.mem.u32(tuples + 0x14), 1);
        assert_eq!(e.mem.u32(tuples + 0x18), tuples);
        let inner = e.mem.u32(tuples + 0x20);
        assert_eq!(e.mem.u32(inner), 0x010d_707c);
        assert_eq!(e.mem.u32(inner + 8), 1);
    }

    #[test]
    fn tracker_constructor_makes_its_maps_and_clears_the_contents() {
        let mut e = fresh();
        let tracker = e.mem.alloc(0x88);
        e.mem.set_u32(tracker + 0x2c, 0xdead);
        e.register(STRING_MAP_CONSTRUCT, |_, _| Ret::default());
        e.register(MULTI_MAP_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00dc_7fd0, &args![tracker]).u32(), tracker);
        assert_eq!(
            calls(&e, STRING_MAP_CONSTRUCT),
            vec![
                vec![tracker, 0],
                vec![tracker + 0x4c, 0],
                vec![tracker + 0x5c, 0]
            ]
        );
        assert_eq!(
            calls(&e, MULTI_MAP_CONSTRUCT),
            vec![
                vec![tracker + 0x10],
                vec![tracker + 0x30],
                vec![tracker + 0x6c]
            ]
        );
        assert_eq!(e.mem.u32(tracker + 0x2c), 0);
    }

    #[test]
    fn world_constructor_makes_a_tracker() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        e.register(STRING_MAP_CONSTRUCT, |_, _| Ret::default());
        e.register(MULTI_MAP_CONSTRUCT, |_, _| Ret::default());
        let world = e.mem.alloc(0x0c);
        assert_eq!(e.call(0x00dc_8030, &args![world]).u32(), world);
        assert_eq!(e.mem.u32(world), 0x010d_7510);
        assert_eq!(e.mem.u16(world + 6), 1);
        let tracker = e.mem.u32(world + 8);
        assert_ne!(tracker, 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![allocator, 0x88]]);
        assert_eq!(calls(&e, STRING_MAP_CONSTRUCT)[0], vec![tracker, 0]);
    }

    #[test]
    fn world_constructor_keeps_a_null_tracker_when_nothing_is_allocated() {
        let mut e = fresh();
        basics(&mut e);
        e.register(ALLOCATE, |_, _| ret(0));
        e.register(STRING_MAP_CONSTRUCT, |_, _| Ret::default());
        let world = e.mem.alloc(0x0c);
        e.mem.set_u32(world + 8, 0x1234);
        assert_eq!(e.call(0x00dc_8030, &args![world]).u32(), world);
        assert_eq!(e.mem.u32(world + 8), 0);
        assert!(calls(&e, STRING_MAP_CONSTRUCT).is_empty());
    }

    #[test]
    fn world_destructor_tears_the_tracker_down_and_frees_it() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        string_maps(&mut e);
        pointer_maps(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        e.call(0x00dc_8080, &args![world]);
        assert_eq!(e.mem.u32(world), 0x0102_e388);
        assert_eq!(calls(&e, FREE), vec![vec![allocator, tracker, 0x88]]);
        assert_eq!(calls(&e, STRING_MAP_DESTRUCT).len(), 3);
    }

    #[test]
    fn world_destructor_without_a_tracker_only_sets_the_vtable() {
        let mut e = fresh();
        basics(&mut e);
        let world = e.mem.alloc(0x0c);
        e.call(0x00dc_8080, &args![world]);
        assert_eq!(e.mem.u32(world), 0x0102_e388);
        assert!(calls(&e, FREE).is_empty());
    }

    /// Gives the exe's own class-dict vtable (`0x010d_73c8`, which the code writes into every class
    /// it makes) the two slots `trackClass` asks: slot 8 `getName` (the translation) and slot
    /// `0x10`, the parent, read from `+0x18`.
    fn class_vtable_for_new_classes(e: &mut Engine) {
        e.mem.set_u32(0x010d_73c8 + 8, 0x00dc_ac10);
        e.mem.set_u32(0x010d_73c8 + 0x10, SLOT_D);
        e.register(SLOT_D, |e, a| ret(e.mem.u32(a[0] + 0x18)));
    }

    #[test]
    fn new_class_makes_tracks_and_fills_the_class() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        handle_doubles(&mut e);
        class_vtable_for_new_classes(&mut e);
        let inserts = record_inserts(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let parent = e.mem.alloc(0x40);
        e.mem.set_u32(parent + 8, 1);
        let parent_name = text(&mut e, "Par");
        put_entry(&maps, tracker, "Par", parent_name, parent);
        let (class_name, member_name, kind) =
            (text(&mut e, "Cls"), text(&mut e, "m1"), text(&mut e, "K"));
        let members = e.mem.alloc(0x10);
        e.mem.set_u32(members, member_name);
        e.mem.set_u32(members + 4, 3);
        e.mem.set_u32(members + 8, 9);
        e.mem.set_u32(members + 0xc, kind);
        let description = e.mem.alloc(0x14);
        e.mem.set_u32(description, class_name);
        e.mem.set_u32(description + 4, 4);
        e.mem.set_u32(description + 8, parent_name);
        e.mem.set_u32(description + 0xc, members);
        e.mem.set_u32(description + 0x10, 1);
        let class = e.call(0x00dc_80c0, &args![world, description]).u32();
        assert_eq!(e.mem.u32(class), 0x010d_73c8);
        assert_eq!(e.mem.u32(class + 0xc), world);
        assert_eq!(e.mem.u32(class + 0x14), 4);
        assert_eq!(e.mem.u32(class + 0x18), parent);
        assert_eq!(e.mem.u32(parent + 8), 2);
        assert_eq!(e.mem.u32(class + 8), 1);
        assert_eq!(map_value(&maps, tracker, "Cls"), Some(class));
        assert_eq!(
            *inserts.borrow(),
            vec![[tracker + tracker::DERIVED_MAP, parent, class]]
        );
        assert_eq!(e.mem.u32(class + 0x2c), 1);
        let record = e.mem.u32(class + 0x28);
        assert_eq!(read_text(&e, e.mem.u32(record)), "m1");
        assert_eq!(e.mem.u32(record + 0x10), 3);
        assert_eq!(e.mem.u32(record + 0x14), 9);
        assert_eq!(e.mem.u32(e.mem.u32(record + 4) + 8), kind);
    }

    #[test]
    fn new_class_without_parent_or_members_only_tracks() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        handle_doubles(&mut e);
        class_vtable_for_new_classes(&mut e);
        let inserts = record_inserts(&mut e);
        let (world, tracker) = make_world(&mut e);
        let class_name = text(&mut e, "Solo");
        let description = e.mem.alloc(0x14);
        e.mem.set_u32(description, class_name);
        let class = e.call(0x00dc_80c0, &args![world, description]).u32();
        assert_eq!(e.mem.u32(class + 0x18), 0);
        assert_eq!(e.mem.u32(class + 0x2c), 0);
        assert_eq!(map_value(&maps, tracker, "Solo"), Some(class));
        assert!(inserts.borrow().is_empty());
    }

    #[test]
    fn new_array_sizes_the_array_and_stores_it_under_the_member_index() {
        let mut e = fresh();
        basics(&mut e);
        let array_vtable = 0x010d_707c;
        e.mem.set_u32(array_vtable + 0x10, SLOT_A);
        e.register(SLOT_A, |_, _| Ret::default());
        e.register(SLOT_B, |_, _| Ret::default());
        let implementation = object_with(&mut e, 0x40, &[(0x54, SLOT_B)]);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, implementation);
        let (world, _) = make_world(&mut e);
        let description = e.mem.alloc(0x14);
        e.mem.set_u32(description, 5);
        e.mem.set_u32(description + 8, 0x21);
        e.mem.set_u32(description + 0x10, 7);
        let array = e
            .call(0x00dc_8210, &args![world, handle, description])
            .u32();
        assert_eq!(e.mem.u32(array), array_vtable);
        assert_eq!(calls(&e, SLOT_A), vec![vec![array, 7]]);
        assert_eq!(calls(&e, SLOT_B), vec![vec![implementation, 5, array]]);
    }

    #[test]
    fn new_array_with_the_tuple_flag_gives_a_zero_size() {
        let mut e = fresh();
        basics(&mut e);
        e.mem.set_u32(0x010d_707c + 0x10, SLOT_A);
        e.register(SLOT_A, |_, _| Ret::default());
        e.register(SLOT_B, |_, _| Ret::default());
        let implementation = object_with(&mut e, 0x40, &[(0x54, SLOT_B)]);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, implementation);
        let (world, _) = make_world(&mut e);
        let description = e.mem.alloc(0x14);
        e.mem.set_u32(description + 8, 0x11);
        e.mem.set_u32(description + 0x10, 3);
        let array = e
            .call(0x00dc_8210, &args![world, handle, description])
            .u32();
        assert_eq!(calls(&e, SLOT_A), vec![vec![array, 0]]);
    }

    /// The `hkClass` doubles `wrapClass` reads. Members: `m0` flagged `0x400`; `m1` without a
    /// class; `m2` with a member attribute; `m3` with an attribute of its class; `m4` with just a
    /// class name. Returns `(hk class, members, world, maps)`.
    fn wrap_doubles(e: &mut Engine) -> (u32, Vec<u32>, u32, StringMaps) {
        basics(e);
        let maps = string_maps(e);
        handle_doubles(e);
        class_vtable_for_new_classes(e);
        record_inserts(e);
        let (world, _) = make_world_with_maps(e);
        let hk_class = e.mem.alloc(0x10);
        let mut members = Vec::new();
        for (i, flags) in [0x400u16, 0, 0, 0, 0].into_iter().enumerate() {
            let member = e.mem.alloc(0x20);
            let name = text(e, &format!("m{i}"));
            e.mem.set_u32(member, name);
            e.mem.set_u8(member + 0xc, 2 + i as u8);
            e.mem.set_u8(member + 0xd, 20 + i as u8);
            e.mem.set_u16(member + 0x10, flags);
            members.push(member);
        }
        let member_classes = [0, 0, 0xc2, 0xc3, 0xc4];
        let classes: Vec<(u32, u32)> = members.iter().copied().zip(member_classes).collect();
        e.register_double(HK_CLASS_MEMBER_GET_CLASS, move |_, a| {
            ret(classes.iter().find(|m| m.0 == a[0]).map_or(0, |m| m.1))
        });
        let list = members.clone();
        e.register_double(HK_CLASS_GET_DECLARED_MEMBER, move |_, a| {
            ret(list[a[1] as usize])
        });
        let list = members.clone();
        e.register_double(HK_CLASS_MEMBER_GET_CSTYLE_ARRAY_SIZE, move |_, a| {
            ret(100 + list.iter().position(|m| *m == a[0]).unwrap() as u32)
        });
        e.register(TYPE_FROM_MEMBER_TYPE, |_, a| ret(a[0] * 0x100 + a[1]));
        let member_two = members[2];
        e.register_double(HK_CLASS_MEMBER_GET_ATTRIBUTE, move |_, a| {
            ret(if a[0] == member_two { 0xa2 } else { 0 })
        });
        e.register(HK_CLASS_GET_ATTRIBUTE, |_, a| {
            ret(if a[0] == 0xc3 { 0xa3 } else { 0 })
        });
        e.register(CLASS_MEMBER_ACCESSOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let holders: Vec<(u32, u32)> = [(0xa2, "Attr2"), (0xa3, "Attr3")]
            .into_iter()
            .map(|(attribute, name)| {
                let holder = e.mem.alloc(4);
                let pointer = text(e, name);
                e.mem.set_u32(holder, pointer);
                (attribute, holder)
            })
            .collect();
        e.register_double(CLASS_MEMBER_ACCESSOR_AS_CSTRING, move |e, a| {
            let attribute = e.mem.u32(a[0]);
            ret(holders.iter().find(|h| h.0 == attribute).unwrap().1)
        });
        let class_names: Vec<(u32, u32)> = [(hk_class, "Wrapped"), (0xc4, "Plain4")]
            .into_iter()
            .map(|(class, name)| (class, text(e, name)))
            .collect();
        e.register_double(HK_CLASS_GET_NAME, move |_, a| {
            ret(class_names.iter().find(|c| c.0 == a[0]).map_or(0, |c| c.1))
        });
        e.register(HK_CLASS_GET_DESCRIBED_VERSION, |_, _| ret(6));
        e.register(HK_CLASS_GET_PARENT, |_, _| ret(0));
        e.register(HK_CLASS_GET_NUM_DECLARED_MEMBERS, |_, _| ret(5));
        e.mem.set_u32(DATA_OBJECT_TYPE_NAME, 0x7e57);
        (hk_class, members, world, maps)
    }

    #[test]
    fn wrap_class_adds_every_declared_member_with_its_handle() {
        let mut e = fresh();
        let (hk_class, members, world, maps) = wrap_doubles(&mut e);
        let tracker = e.mem.u32(world + 8);
        let class = e.call(0x00dc_8270, &args![world, hk_class]).u32();
        assert_eq!(e.mem.u32(class + 0x14), 6);
        assert_eq!(
            read_text(&e, e.mem.u32(e.mem.u32(class + 0x10) + 8)),
            "Wrapped"
        );
        assert_eq!(map_value(&maps, tracker, "Wrapped"), Some(class));
        assert_eq!(e.mem.u32(class + 0x2c), 5);
        let records = e.mem.u32(class + 0x28);
        let record = |i: u32| records + 0x18 * i;
        // m0: flag 0x400: type 0, the default handle.
        assert_eq!(e.mem.u32(record(0) + 0x10), 0);
        assert_eq!(e.mem.u32(record(0) + 4), DEFAULT_CLASS_HANDLE);
        assert_eq!(e.mem.u32(record(0) + 0x14), 100);
        // m1: no class: its type, the default handle.
        assert_eq!(e.mem.u32(record(1) + 0x10), 3 * 0x100 + 21);
        assert_eq!(e.mem.u32(record(1) + 4), DEFAULT_CLASS_HANDLE);
        assert_eq!(e.mem.u32(record(1) + 0x14), 101);
        // m2, m3, m4: handles of the class named by the attribute or the class itself.
        for (i, name) in [(2, "Attr2"), (3, "Attr3"), (4, "Plain4")] {
            let handle = e.mem.u32(record(i) + 4);
            assert_ne!(handle, DEFAULT_CLASS_HANDLE, "m{i}");
            assert_eq!(read_text(&e, e.mem.u32(handle + 8)), name);
            assert_eq!(map_value(&maps, tracker + 0x5c, name), Some(handle));
        }
        for (i, member) in members.iter().enumerate() {
            let name = e.mem.u32(*member);
            assert_eq!(
                read_text(&e, e.mem.u32(record(i as u32))),
                read_text(&e, name)
            );
        }
        assert_eq!(
            calls(&e, CLASS_MEMBER_ACCESSOR_CONSTRUCT)
                .iter()
                .map(|c| (c[1], c[2]))
                .collect::<Vec<_>>(),
            vec![(0xa2, TEXT_TYPE_NAME), (0xa3, TEXT_TYPE_NAME)]
        );
    }

    #[test]
    fn wrap_class_wraps_a_missing_parent_and_reuses_a_known_one() {
        let mut e = fresh();
        let (hk_class, _, world, maps) = wrap_doubles(&mut e);
        let tracker = e.mem.u32(world + 8);
        let parent_class = e.mem.alloc(0x10);
        e.register_double(HK_CLASS_GET_PARENT, move |_, a| {
            ret(if a[0] == hk_class { parent_class } else { 0 })
        });
        let names = Rc::new(RefCell::new(HashMap::new()));
        e.register_double(HK_CLASS_GET_NAME, move |e, a| {
            let mut names = names.borrow_mut();
            let pointer = *names.entry(a[0]).or_insert_with(|| {
                let label = if a[0] == parent_class {
                    "Base"
                } else {
                    "Wrapped"
                };
                text(e, label)
            });
            ret(pointer)
        });
        e.register(HK_CLASS_GET_NUM_DECLARED_MEMBERS, |_, _| ret(0));
        let class = e.call(0x00dc_8270, &args![world, hk_class]).u32();
        let parent = map_value(&maps, tracker, "Base").expect("the parent was wrapped");
        assert_eq!(e.mem.u32(class + 0x18), parent);
        assert_eq!(e.mem.u32(parent + 8), 2);
        // A second wrap finds the parent in the class map instead of wrapping it again.
        let again = e.call(0x00dc_8270, &args![world, hk_class]).u32();
        assert_eq!(e.mem.u32(again + 0x18), parent);
        assert_eq!(e.mem.u32(parent + 8), 3);
    }

    /// A world object (`+8` tracker) whose virtual slots are `slots`; returns `(world, tracker)`.
    fn world_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> (u32, u32) {
        let (_, tracker) = make_world(e);
        let world = object_with(e, 0x0c, slots);
        e.mem.set_u32(world + 8, tracker);
        (world, tracker)
    }

    /// The `hkDataClass` handle doubles `copyClassFromWorld` uses: a handle is one word, the
    /// class implementation; `GetName` answers by it. The source class `0x5eed` is called `Src`,
    /// `0x77` (the parent `getParent` puts into the handle when `parent` is set) `Par`; it has one
    /// declared member `mem` of type 5 and extra word 6.
    fn data_class_doubles(e: &mut Engine, parent: bool) {
        let (src, par, member) = (text(e, "Src"), text(e, "Par"), text(e, "mem"));
        e.register(DATA_CLASS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(DATA_CLASS_DESTRUCT, |_, _| Ret::default());
        e.register(DATA_CLASS_CINFO_DESTRUCT, |_, _| Ret::default());
        e.register_double(DATA_CLASS_GET_NAME, move |e, a| {
            ret(if e.mem.u32(a[0]) == 0x77 { par } else { src })
        });
        e.register(DATA_CLASS_GET_VERSION, |_, _| ret(3));
        e.register_double(DATA_CLASS_GET_PARENT, move |e, a| {
            e.mem.set_u32(a[1], if parent { 0x77 } else { 0 });
            ret(a[1])
        });
        e.register(DATA_CLASS_IS_NULL, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
        });
        e.register(DATA_CLASS_GET_NUM_DECLARED_MEMBERS, |_, _| ret(1));
        e.register_double(DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO, move |e, a| {
            let data = e.mem.u32(a[1]);
            e.mem.set_u32(data, member);
            e.mem.set_u32(data + 8, 5);
            e.mem.set_u32(data + 0x10, 6);
            Ret::default()
        });
    }

    #[test]
    fn copy_class_from_world_answers_a_class_the_world_has() {
        let mut e = fresh();
        basics(&mut e);
        e.register(SLOT_A, |_, _| ret(0xe1));
        let (world, _) = world_with_slots(&mut e, &[(0x24, SLOT_A)]);
        let source = object_with(&mut e, 8, &[]);
        let name = text(&mut e, "Src");
        assert_eq!(e.call(0x00dc_85a0, &args![world, name, source]).u32(), 0xe1);
        assert!(calls(&e, DATA_CLASS_CONSTRUCT).is_empty());
    }

    #[test]
    fn copy_class_from_world_stops_when_the_world_has_the_parent_named_class() {
        let mut e = fresh();
        basics(&mut e);
        data_class_doubles(&mut e, true);
        let mut asked = 0;
        e.register_double(SLOT_A, move |_, _| {
            asked += 1;
            ret(if asked == 1 { 0 } else { 0xf00d })
        });
        e.register(SLOT_B, |_, _| ret(0xbad));
        e.register(SLOT_C, |_, _| ret(0x5eed));
        let (world, _) = world_with_slots(&mut e, &[(0x24, SLOT_A), (0xc, SLOT_B)]);
        let source = object_with(&mut e, 8, &[(0x24, SLOT_C)]);
        let name = text(&mut e, "Src");
        assert_eq!(
            e.call(0x00dc_85a0, &args![world, name, source]).u32(),
            0xf00d
        );
        assert_eq!(calls(&e, DATA_CLASS_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, DATA_CLASS_CINFO_DESTRUCT).len(), 1);
        assert!(calls(&e, SLOT_B).is_empty());
    }

    #[test]
    fn copy_class_from_world_makes_the_class_and_adds_its_members() {
        let mut e = fresh();
        basics(&mut e);
        string_maps(&mut e);
        handle_doubles(&mut e);
        data_class_doubles(&mut e, false);
        let new_class = class_with_slots(&mut e, 0, &[(0x20, SLOT_D)]);
        e.register(SLOT_D, |_, _| ret(0xffff_ffff));
        let seen: Rc<RefCell<Vec<[u32; 3]>>> = Rc::default();
        let record = Rc::clone(&seen);
        e.register_double(SLOT_B, move |e, a| {
            let description = a[1];
            record.borrow_mut().push([
                e.mem.u32(description),
                e.mem.u32(description + 4),
                e.mem.u32(description + 8),
            ]);
            ret(new_class)
        });
        e.register(SLOT_A, |_, _| ret(0));
        e.register(SLOT_C, |_, _| ret(0x5eed));
        let (world, _) = world_with_slots(&mut e, &[(0x24, SLOT_A), (0xc, SLOT_B)]);
        let source = object_with(&mut e, 8, &[(0x24, SLOT_C)]);
        let name = text(&mut e, "Src");
        assert_eq!(
            e.call(0x00dc_85a0, &args![world, name, source]).u32(),
            new_class
        );
        let described = seen.borrow()[0];
        assert_eq!(read_text(&e, described[0]), "Src");
        assert_eq!(described[1..], [3, 0]);
        assert_eq!(e.mem.u32(new_class + 0x2c), 1);
        let record = e.mem.u32(new_class + 0x28);
        assert_eq!(read_text(&e, e.mem.u32(record)), "mem");
        assert_eq!(e.mem.u32(record + 0x10), 5);
        assert_eq!(e.mem.u32(record + 0x14), 6);
        assert_eq!(calls(&e, DATA_CLASS_DESTRUCT).len(), 3);
        let temp_frees = calls(&e, TEMP_BUFFER_FREE);
        assert_eq!(temp_frees.len(), 1);
        assert_eq!(temp_frees[0][2], 20);
        assert_eq!(calls(&e, BUFFER_FREE)[0][2], 16);
    }

    /// A class object for the tracker tests: its name word at `+0x30` (slot 8 answers it), its
    /// parent at `+0x18` (slot `0x10` answers it), two references, entered in the class map.
    fn tracked_class(
        e: &mut Engine,
        tracker: u32,
        maps: &StringMaps,
        label: &str,
        parent: u32,
    ) -> u32 {
        e.register(SLOT_A, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(SLOT_D, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(SLOT_B, |_, _| Ret::default());
        let class = object_with(e, 0x40, &[(0, SLOT_B), (8, SLOT_A), (0x10, SLOT_D)]);
        let name = text(e, label);
        e.mem.set_u32(class + 8, 2);
        e.mem.set_u32(class + 0x18, parent);
        e.mem.set_u32(class + 0x30, name);
        put_entry(maps, tracker, label, name, class);
        class
    }

    #[test]
    fn remove_class_empties_the_handle_untracks_the_class_and_releases_it() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        e.register(DATA_CLASS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(DATA_CLASS_DESTRUCT, |_, _| Ret::default());
        let (world, tracker) = make_world_with_maps(&mut e);
        let class = tracked_class(&mut e, tracker, &maps, "Cls", 0);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, class);
        e.call(0x00dc_8870, &args![world, handle]);
        assert_eq!(e.mem.u32(handle), 0);
        assert_eq!(calls(&e, DATA_CLASS_CONSTRUCT)[0][1], 0);
        assert_eq!(calls(&e, DATA_CLASS_DESTRUCT).len(), 1);
        assert_eq!(map_value(&maps, tracker, "Cls"), None);
        assert_eq!(e.mem.u32(class + 8), 1);
    }

    #[test]
    fn untrack_class_untracks_the_derived_classes_and_forgets_the_names() {
        let mut e = fresh();
        let (router, _, stack) = basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        let (_, tracker) = make_world_with_maps(&mut e);
        let derived_map = tracker + tracker::DERIVED_MAP;
        let base = tracked_class(&mut e, tracker, &maps, "P", 0);
        let middle = tracked_class(&mut e, tracker, &maps, "C", base);
        let leaf = tracked_class(&mut e, tracker, &maps, "D", middle);
        chain_add(&mut e, derived_map, base, middle);
        chain_add(&mut e, derived_map, middle, leaf);
        put_entry(&maps, tracker + 0x5c, "C", 0, 0xc0c0);
        put_entry(&maps, tracker + 0x5c, "D", 0, 0xd0d0);
        e.call(0x00dc_bdd0, &args![tracker, middle]);
        assert_eq!(map_value(&maps, tracker, "C"), None);
        assert_eq!(map_value(&maps, tracker, "D"), None);
        assert!(map_value(&maps, tracker, "P").is_some());
        assert_eq!(chain_values(&e, derived_map, base), Vec::<u32>::new());
        assert_eq!(
            calls(&e, REMOVE_REFERENCE),
            vec![vec![0xd0d0], vec![0xc0c0]]
        );
        assert_eq!(e.mem.u32(leaf + 8), 1);
        assert_eq!(e.mem.u32(router), stack);
        assert_eq!(e.mem.u32(middle + 0x10), DEFAULT_CLASS_HANDLE);
    }

    #[test]
    fn untrack_class_destroys_the_objects_of_the_class() {
        let mut e = fresh();
        let (router, _, stack) = basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        e.register(OSTREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(OSTREAM_WRITE_STRING, |_, a| ret(a[0]));
        e.register(OSTREAM_WRITE_INT, |_, a| ret(a[0]));
        e.register(OSTREAM_DESTRUCT, |_, _| Ret::default());
        e.register(SLOT_C, |_, _| Ret::default());
        let (_, tracker) = make_world_with_maps(&mut e);
        let class = tracked_class(&mut e, tracker, &maps, "C", 0);
        e.register_double(SLOT_E, move |_, _| ret(class));
        let reporter = object_with(&mut e, 0x10, &[(0xc, SLOT_C)]);
        e.mem.set_u32(ERROR_REPORTER, reporter);
        let object = object_with(&mut e, 0x1c, &[(8, SLOT_E)]);
        e.mem.set_u32(object + 8, 2);
        e.mem.set_u16(object + 6, 1);
        e.mem.set_u32(tracker + 0x2c, object);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, class, object);
        e.call(0x00dc_bdd0, &args![tracker, class]);
        assert_eq!(e.mem.u32(tracker + 0x2c), 0);
        assert_eq!(e.mem.u32(object + 8), 1);
        let report = &calls(&e, SLOT_C)[0];
        assert_eq!(report[1..3], [1, 0x760d_3794]);
        assert_eq!(report[5], 0x7b9);
        assert_eq!(e.mem.u32(router), stack);
        assert_eq!(
            chain_values(&e, tracker + tracker::OBJECT_MAP, class),
            Vec::<u32>::new()
        );
    }

    #[test]
    fn clear_array_releases_the_first_value_of_tuple_and_array_types() {
        let mut e = fresh();
        basics(&mut e);
        e.register(SLOT_B, |_, _| Ret::default());
        let object = object_with(&mut e, 0x20, &[(0, SLOT_B)]);
        e.mem.set_u32(object + 8, 1);
        let values = e.mem.alloc(0x10);
        e.mem.set_u32(values, object);
        e.call(0x00dc_88c0, &args![0x10u32, values, 2i32]);
        assert_eq!(calls(&e, SLOT_B), vec![vec![object, 1]]);
        e.call(0x00dc_88c0, &args![0x21u32, values, 2i32]);
        assert_eq!(e.mem.u32(object + 8), 4294967295);
        e.mem.set_u32(values, 0);
        e.call(0x00dc_88c0, &args![0x20u32, values, 2i32]);
        assert_eq!(calls(&e, SLOT_B).len(), 1);
    }

    #[test]
    fn clear_array_frees_vector_values_with_their_size() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        for (array_type, size) in [(4u32, 16u32), (6, 48), (7, 64)] {
            let values = e.mem.alloc(0x10);
            e.mem.set_u32(values, 0xaaa0 + array_type);
            e.call(0x00dc_88c0, &args![array_type, values, 2i32]);
            assert_eq!(e.mem.u32(values), 0);
            let freed = calls(&e, FREE);
            assert_eq!(
                freed.last().unwrap(),
                &vec![allocator, 0xaaa0 + array_type, size]
            );
        }
        assert_eq!(calls(&e, FREE).len(), 3);
    }

    #[test]
    fn clear_array_releases_objects_and_frees_strings() {
        let mut e = fresh();
        basics(&mut e);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 8, 3);
        let values = e.mem.alloc(0x10);
        e.mem.set_u32(values, object);
        for array_type in [8u32, 9] {
            e.mem.set_u32(values, object);
            e.call(0x00dc_88c0, &args![array_type, values, 2i32]);
            assert_eq!(e.mem.u32(values), 0);
        }
        assert_eq!(e.mem.u32(object + 8), 1);
        e.mem.set_u32(values, 0x5a5a);
        e.mem.set_u32(values + 8, 0);
        e.call(0x00dc_88c0, &args![10u32, values, 2i32]);
        assert_eq!(calls(&e, STRING_FREE), vec![vec![0x5a5a], vec![0]]);
        assert_eq!(e.mem.u32(values), 0);
        e.mem.set_u32(values, 0x6b6b);
        for array_type in [5u32, 1, 0, 11] {
            e.call(0x00dc_88c0, &args![array_type, values, 2i32]);
        }
        assert_eq!(e.mem.u32(values), 0x6b6b);
    }

    #[test]
    fn tuple_array_destructor_releases_the_elements_and_frees_the_block() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        e.register(SLOT_B, |_, _| Ret::default());
        let elements = object_with(&mut e, 0x20, &[(0, SLOT_B)]);
        e.mem.set_u32(elements + 8, 1);
        let tuples = e.mem.alloc(0x28);
        e.mem.set_u32(tuples + 0x20, elements);
        e.mem.set_u16(tuples + 4, 0x28);
        assert_eq!(e.call(0x00dc_8cd0, &args![tuples, 1u32]).u32(), tuples);
        assert_eq!(calls(&e, SLOT_B), vec![vec![elements, 1]]);
        assert_eq!(e.mem.u32(tuples), 0x010d_62f4);
        assert_eq!(e.mem.u32(tuples + 0xc), 0x010d_62f4);
        assert_eq!(calls(&e, FREE), vec![vec![allocator, tuples, 0x28]]);
        let second = e.mem.alloc(0x28);
        e.mem.set_u32(second + 0x20, elements);
        e.mem.set_u32(elements + 8, 2);
        e.call(0x00dc_8cd0, &args![second, 0u32]);
        assert_eq!(calls(&e, FREE).len(), 1);
    }

    #[test]
    fn intern_class_name_handle_makes_one_record_per_name() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let maps = string_maps(&mut e);
        handle_doubles(&mut e);
        let (_, tracker) = make_world(&mut e);
        let name = text(&mut e, "K");
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00dc_8d70, &args![tracker, out, name]).u32(), out);
        let record = e.mem.u32(out);
        assert_ne!(record, DEFAULT_CLASS_HANDLE);
        assert_eq!(e.mem.u16(record + 4), 0xc);
        assert_eq!(e.mem.u16(record + 6), 1);
        assert_eq!(e.mem.u32(record), 0x010d_6da0);
        assert_eq!(e.mem.u32(record + 8), name);
        assert_eq!(map_value(&maps, tracker + 0x5c, "K"), Some(record));
        assert_eq!(calls(&e, ALLOCATE), vec![vec![allocator, 0xc]]);
        let again = e.mem.alloc(4);
        e.call(0x00dc_8d70, &args![tracker, again, name]);
        assert_eq!(e.mem.u32(again), record);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        assert!(calls(&e, ADD_REFERENCE).contains(&vec![record]));
        assert!(calls(&e, ADD_REFERENCE).contains(&vec![DEFAULT_CLASS_HANDLE]));
    }

    #[test]
    fn intern_class_name_handle_of_no_name_is_the_default_handle() {
        let mut e = fresh();
        basics(&mut e);
        string_maps(&mut e);
        let (_, tracker) = make_world(&mut e);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x4444);
        e.call(0x00dc_8d70, &args![tracker, out, 0u32]);
        assert_eq!(e.mem.u32(out), DEFAULT_CLASS_HANDLE);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![DEFAULT_CLASS_HANDLE]]);
        assert!(calls(&e, STRING_MAP_GET_WITH_DEFAULT).is_empty());
    }

    #[test]
    fn multi_map_first_index_is_minus_one_for_a_missing_key() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        chain_add(&mut e, multi, 0x11, 0xaa);
        chain_add(&mut e, multi, 0x22, 0xbb);
        assert_eq!(e.call(0x00dc_8e30, &args![multi, 0x22u32]).i32(), 1);
        assert_eq!(e.call(0x00dc_8e30, &args![multi, 0x33u32]).i32(), -1);
    }

    #[test]
    fn member_info_by_name_searches_the_class_and_then_its_parents() {
        let mut e = fresh();
        let (parent, child) = (
            class_dict(&mut e, 0, &[(0xb0, 0, 1)]),
            class_dict(&mut e, 0, &[(0xa0, 0, 2)]),
        );
        e.mem.set_u32(child + 0x18, parent);
        let info_of = |e: &mut Engine, name: u32| e.call(0x00dc_8e50, &args![child, name]).u32();
        assert_eq!(info_of(&mut e, 0xa0), e.mem.u32(child + 0x28));
        assert_eq!(info_of(&mut e, 0xb0), e.mem.u32(parent + 0x28));
        assert_eq!(info_of(&mut e, 0xc0), 0);
        assert_eq!(e.call(0x00dc_8e50, &args![0u32, 0xa0u32]).u32(), 0);
    }

    #[test]
    fn struct_array_remove_member_drops_the_record_and_its_array() {
        let mut e = fresh();
        basics(&mut e);
        e.register(SLOT_B, |_, _| Ret::default());
        let arrays: Vec<u32> = (0..3)
            .map(|_| {
                let array = object_with(&mut e, 0x20, &[(0, SLOT_B)]);
                e.mem.set_u32(array + 8, 1);
                array
            })
            .collect();
        let this = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x18);
        for (i, array) in arrays.iter().enumerate() {
            e.mem.set_u32(records + 8 * i as u32, 0xa0 + i as u32);
            e.mem.set_u32(records + 8 * i as u32 + 4, *array);
        }
        e.mem.set_u32(this + 0xc, records);
        e.mem.set_u32(this + 0x10, 3);
        e.call(0x00dc_8ea0, &args![this, 0xa1u32]);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(calls(&e, SLOT_B), vec![vec![arrays[1], 1]]);
        assert_eq!(e.mem.u32(records + 8), 0xa2);
        assert_eq!(e.mem.u32(records + 12), arrays[2]);
        e.call(0x00dc_8ea0, &args![this, 0xffu32]);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00dc_8ea0, &args![this, 0xa0u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn multi_map_clear_empties_the_entries_and_resets_the_free_list() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        chain_add(&mut e, multi, 0x11, 0xaa);
        e.mem.set_u32(multi + multi_map::FREE_HEAD, 7);
        e.call(0x00dc_8fc0, &args![multi]);
        assert_eq!(e.mem.u32(multi + 4), 0);
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), -1);
        assert_eq!(
            calls(&e, POINTER_MAP_RESET),
            vec![vec![multi + multi_map::MAP]]
        );
    }

    #[test]
    fn multi_map_remove_value_takes_the_matching_entry_out_of_the_chain() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        for value in [0xa1, 0xa2, 0xa3] {
            chain_add(&mut e, multi, 0x11, value);
        }
        let wanted = e.mem.alloc(4);
        e.mem.set_u32(wanted, 0xa2);
        e.call(0x00dc_8fe0, &args![multi, 0x11u32, wanted]);
        assert_eq!(chain_values(&e, multi, 0x11), vec![0xa1, 0xa3]);
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), 2);
        e.mem.set_u32(wanted, 0x99);
        e.call(0x00dc_8fe0, &args![multi, 0x11u32, wanted]);
        assert_eq!(chain_values(&e, multi, 0x11), vec![0xa1, 0xa3]);
        e.call(0x00dc_8fe0, &args![multi, 0x44u32, wanted]);
        assert_eq!(chain_values(&e, multi, 0x11), vec![0xa1, 0xa3]);
    }

    #[test]
    fn multi_map_remove_key_puts_the_whole_chain_on_the_free_list() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        chain_add(&mut e, multi, 0x11, 0xa1);
        chain_add(&mut e, multi, 0x11, 0xa2);
        e.mem.set_u32(multi + multi_map::FREE_HEAD, 9);
        e.call(0x00dc_9030, &args![multi, 0x11u32]);
        let entries = e.mem.u32(multi);
        assert_eq!(e.mem.i32(entries + 8 + 4), 9);
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), 0);
        assert_eq!(chain_values(&e, multi, 0x11), Vec::<u32>::new());
        assert_eq!(calls(&e, POINTER_MAP_REMOVE).len(), 1);
    }

    #[test]
    fn multi_map_remove_key_of_an_empty_chain_changes_no_entries() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        let table = e.mem.u32(multi + multi_map::MAP);
        e.mem.set_u32(table, 0x11);
        e.mem.set_u32(table + 4, 0xffff_ffff);
        e.call(0x00dc_9030, &args![multi, 0x11u32]);
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), -1);
        assert_eq!(calls(&e, POINTER_MAP_REMOVE).len(), 1);
    }

    #[test]
    fn object_self_destruct_clears_the_values_of_known_members() {
        let mut e = fresh();
        basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0, 8), (0xb0, 0, 10)]);
        let value = e.mem.alloc(0x20);
        e.mem.set_u32(value + 8, 3);
        let object = dict_object(
            &mut e,
            class,
            &[(0xa0, value), (0xb0, 0x5a5a), (0xc0, 0x7777)],
        );
        e.call(0x00dc_90a0, &args![object]);
        assert_eq!(e.mem.u32(object + 0x14), 0);
        let data = e.mem.u32(object + 0x10);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(value + 8), 2);
        assert_eq!(e.mem.u32(data + 0x10 + 8), 0);
        assert_eq!(calls(&e, STRING_FREE), vec![vec![0x5a5a]]);
        assert_eq!(e.mem.u32(data + 0x20 + 8), 0x7777);
    }

    #[test]
    fn object_remove_member_moves_the_last_record_into_the_gap() {
        let mut e = fresh();
        basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0, 8), (0xb0, 0, 8), (0xc0, 0, 8)]);
        let object = dict_object(&mut e, class, &[(0xa0, 0), (0xb0, 0), (0xc0, 0)]);
        e.call(0x00dc_90f0, &args![object, 0xa0u32]);
        assert_eq!(record_names(&e, object), vec![0xc0, 0xb0]);
        e.call(0x00dc_90f0, &args![object, 0xb0u32]);
        assert_eq!(record_names(&e, object), vec![0xc0]);
        e.call(0x00dc_90f0, &args![object, 0xffu32]);
        assert_eq!(record_names(&e, object), vec![0xc0]);
    }

    #[test]
    fn object_remove_member_of_the_last_record_moves_nothing() {
        let mut e = fresh();
        basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0, 8), (0xb0, 0, 8)]);
        let object = dict_object(&mut e, class, &[(0xa0, 0), (0xb0, 0)]);
        e.call(0x00dc_90f0, &args![object, 0xb0u32]);
        assert_eq!(record_names(&e, object), vec![0xa0]);
    }

    #[test]
    fn tracked_struct_arrays_include_the_derived_classes_on_request() {
        let mut e = fresh();
        basics(&mut e);
        pointer_maps(&mut e);
        let (_, tracker) = make_world_with_maps(&mut e);
        let (array_map, derived_map) =
            (tracker + tracker::ARRAY_MAP, tracker + tracker::DERIVED_MAP);
        chain_add(&mut e, array_map, 0xc1, 0x51);
        chain_add(&mut e, array_map, 0xc1, 0x52);
        chain_add(&mut e, derived_map, 0xc1, 0xc2);
        chain_add(&mut e, array_map, 0xc2, 0x53);
        let out = empty_array(&mut e);
        e.call(0x00dc_9180, &args![tracker, 0xc1u32, 1u32, out]);
        assert_eq!(array_words(&e, out), vec![0x51, 0x52, 0x53]);
        let out = empty_array(&mut e);
        e.call(0x00dc_9180, &args![tracker, 0xc1u32, 0u32, out]);
        assert_eq!(array_words(&e, out), vec![0x51, 0x52]);
    }

    #[test]
    fn tracked_objects_include_the_elements_of_the_arrays_of_the_class() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        pointer_maps(&mut e);
        let (_, tracker) = make_world_with_maps(&mut e);
        let name = text(&mut e, "Foo");
        put_entry(&maps, tracker, "Foo", name, 0xc1);
        chain_add(&mut e, tracker + tracker::OBJECT_MAP, 0xc1, 0x11);
        e.register(SLOT_A, |_, _| ret(2));
        e.register(SLOT_B, |_, a| ret(0xe0 + a[1]));
        let array = object_with(&mut e, 0x20, &[(0x14, SLOT_A), (0x50, SLOT_B)]);
        chain_add(&mut e, tracker + tracker::ARRAY_MAP, 0xc1, array);
        let out = empty_array(&mut e);
        e.call(0x00dc_92d0, &args![tracker, name, 0u32, 1u32, out]);
        assert_eq!(array_words(&e, out), vec![0x11, 0xe0, 0xe1]);
        let out = empty_array(&mut e);
        e.call(0x00dc_92d0, &args![tracker, name, 0u32, 0u32, out]);
        assert_eq!(array_words(&e, out), vec![0x11]);
        let other = text(&mut e, "Nope");
        let out = empty_array(&mut e);
        e.call(0x00dc_92d0, &args![tracker, other, 1u32, 1u32, out]);
        e.call(0x00dc_92d0, &args![tracker, 0u32, 1u32, 1u32, out]);
        assert_eq!(array_words(&e, out), Vec::<u32>::new());
    }

    #[test]
    fn class_remove_member_releases_the_handle_and_closes_the_gap() {
        let mut e = fresh();
        basics(&mut e);
        let parent = class_dict(&mut e, 0, &[(0xb0, 0, 1)]);
        let class = class_dict(
            &mut e,
            0,
            &[(0xa0, 0x1001, 1), (0xb0, 0x1002, 1), (0xc0, 0x1003, 1)],
        );
        e.mem.set_u32(class + 0x18, parent);
        assert_eq!(e.call(0x00dc_9470, &args![class, 0xb0u32]).u32(), 1);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x1002]]);
        assert_eq!(e.mem.u32(class + 0x2c), 2);
        let records = e.mem.u32(class + 0x28);
        assert_eq!(e.mem.u32(records), 0xa0);
        assert_eq!(e.mem.u32(records + 0x18), 0xc0);
        assert_eq!(e.mem.u32(records + 0x18 + 4), 0x1003);
        assert_eq!(e.call(0x00dc_9470, &args![class, 0xa0u32]).u32(), 0);
        assert_eq!(e.mem.u32(class + 0x2c), 1);
        assert_eq!(e.mem.u32(e.mem.u32(class + 0x28)), 0xc0);
    }

    /// A class dict `class` in a world whose interned-name map holds the names `names`
    /// (pointer = its own value), and an object of it.
    fn named_members(e: &mut Engine, names: &[&str]) -> (u32, u32, Vec<u32>) {
        let maps = string_maps(e);
        let (world, tracker) = make_world(e);
        let interned: Vec<u32> = names
            .iter()
            .map(|name| {
                let pointer = text(e, name);
                put_entry(&maps, tracker + 0x4c, name, pointer, pointer);
                pointer
            })
            .collect();
        let class = class_dict(e, world, &[]);
        (class, tracker, interned)
    }

    #[test]
    fn intern_through_a_class_dict_stores_the_interned_name() {
        let mut e = fresh();
        basics(&mut e);
        let (class, _, names) = named_members(&mut e, &["x"]);
        let out = e.mem.alloc(4);
        let raw = text(&mut e, "x");
        assert_eq!(e.call(0x00dc_9570, &args![class, out, raw]).u32(), out);
        assert_eq!(e.mem.u32(out), names[0]);
        e.call(0x00dc_9570, &args![class, out, 0u32]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn object_has_member_compares_interned_name_words() {
        let mut e = fresh();
        basics(&mut e);
        let (class, _, names) = named_members(&mut e, &["a", "b"]);
        let object = dict_object(&mut e, class, &[(names[1], 0)]);
        let (raw_a, raw_b) = (text(&mut e, "a"), text(&mut e, "b"));
        assert!(e.call(0x00dc_95d0, &args![object, raw_b]).bool());
        assert!(!e.call(0x00dc_95d0, &args![object, raw_a]).bool());
        assert!(!e.call(0x00dc_95d0, &args![object, 0u32]).bool());
    }

    #[test]
    fn object_access_by_name_stores_the_object_and_the_interned_name() {
        let mut e = fresh();
        basics(&mut e);
        let (class, _, names) = named_members(&mut e, &["a"]);
        let object = dict_object(&mut e, class, &[]);
        let (out, raw) = (e.mem.alloc(8), text(&mut e, "a"));
        assert_eq!(e.call(0x00dc_9640, &args![object, out, raw]).u32(), out);
        assert_eq!(e.mem.u32(out), object);
        assert_eq!(e.mem.u32(out + 4), names[0]);
        e.call(0x00dc_9640, &args![object, out, 0u32]);
        assert_eq!(e.mem.u32(out + 4), 0);
    }

    #[test]
    fn object_destructor_frees_the_records_and_releases_the_class() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let class = class_dict(&mut e, 0, &[]);
        e.mem.set_u32(class + 8, 2);
        let object = dict_object(&mut e, class, &[(0x1, 0)]);
        let data = e.mem.u32(object + 0x10);
        e.call(0x00dc_96a0, &args![object]);
        assert_eq!(e.mem.u32(object + 0x14), 0);
        assert_eq!(calls(&e, BUFFER_FREE), vec![vec![allocator, data, 0x10]]);
        assert_eq!(e.mem.u32(class + 8), 1);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object), 0x010d_62f4);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        assert_eq!(e.mem.u32(object + 0x18), 0x8000_0000);
    }

    #[test]
    fn write_access_answers_no_info_for_an_unknown_member() {
        let mut e = fresh();
        basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0, 1)]);
        let object = dict_object(&mut e, class, &[]);
        let out = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00dc_9710, &args![object, out, 0xffu32, 1u32])
                .u32(),
            out
        );
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.i32(out + 4), -1);
        assert_eq!(e.mem.u32(object + 0x14), 0);
    }

    #[test]
    fn write_access_clears_an_existing_value_and_appends_a_missing_record() {
        let mut e = fresh();
        basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0, 8), (0xb0, 0, 8)]);
        let held = e.mem.alloc(0x20);
        e.mem.set_u32(held + 8, 5);
        let object = dict_object(&mut e, class, &[(0xa0, held)]);
        let out = e.mem.alloc(8);
        e.call(0x00dc_9710, &args![object, out, 0xa0u32, 8u32]);
        assert_eq!(e.mem.u32(out), e.mem.u32(class + 0x28));
        assert_eq!(e.mem.i32(out + 4), 0);
        assert_eq!(e.mem.u32(held + 8), 4);
        assert_eq!(e.mem.u32(e.mem.u32(object + 0x10) + 8), 0);
        e.call(0x00dc_9710, &args![object, out, 0xb0u32, 8u32]);
        assert_eq!(e.mem.u32(out), e.mem.u32(class + 0x28) + 0x18);
        assert_eq!(e.mem.i32(out + 4), 1);
        assert_eq!(e.mem.u32(object + 0x14), 2);
        assert_eq!(record_names(&e, object), vec![0xa0, 0xb0]);
        let data = e.mem.u32(object + 0x10);
        assert_eq!(e.mem.u32(data + 0x10 + 8), 0);
        assert_eq!(e.mem.u32(data + 0x10 + 0xc), 0);
    }

    /// An object whose class has one member `0xa0` of type `member_type`, and no records yet.
    fn assign_fixture(e: &mut Engine, member_type: u32) -> u32 {
        basics(e);
        let class = class_dict(e, 0, &[(0xa0, 0, member_type)]);
        dict_object(e, class, &[])
    }

    /// The first record's value words.
    fn first_value(e: &Engine, object: u32) -> (u32, u32) {
        let data = e.mem.u32(object + 0x10);
        (e.mem.u32(data + 8), e.mem.u32(data + 0xc))
    }

    #[test]
    fn assign_stores_an_object_and_takes_a_reference() {
        let mut e = fresh();
        let object = assign_fixture(&mut e, 8);
        let value = e.mem.alloc(0x20);
        e.call(0x00dc_97f0, &args![object, 0xa0u32, value]);
        assert_eq!(first_value(&e, object).0, value);
        assert_eq!(e.mem.u32(value + 8), 1);
        let other = assign_fixture(&mut e, 8);
        e.call(0x00dc_97f0, &args![other, 0xa0u32, 0u32]);
        assert_eq!(first_value(&e, other).0, 0);
    }

    #[test]
    fn assign_ov2_stores_an_array_and_takes_a_reference() {
        let mut e = fresh();
        let object = assign_fixture(&mut e, 5);
        e.register(SLOT_A, |_, _| ret(5));
        let array = object_with(&mut e, 0x20, &[(4, SLOT_A)]);
        e.call(0x00dc_9830, &args![object, 0xa0u32, array]);
        assert_eq!(first_value(&e, object).0, array);
        assert_eq!(e.mem.u32(array + 8), 1);
    }

    #[test]
    fn assign_ov3_stores_a_copy_of_the_text() {
        let mut e = fresh();
        let object = assign_fixture(&mut e, 10);
        let raw = text(&mut e, "hello");
        e.call(0x00dc_9880, &args![object, 0xa0u32, raw]);
        let copy = first_value(&e, object).0;
        assert_ne!(copy, raw);
        assert_eq!(read_text(&e, copy), "hello");
        let other = assign_fixture(&mut e, 10);
        e.call(0x00dc_9880, &args![other, 0xa0u32, 0u32]);
        assert_eq!(first_value(&e, other).0, 0);
    }

    #[test]
    fn assign_ov4_stores_the_float_quieted() {
        let mut e = fresh();
        let object = assign_fixture(&mut e, 3);
        e.call(0x00dc_98d0, &args![object, 0xa0u32, 1.5f32]);
        assert_eq!(first_value(&e, object).0, 1.5f32.to_bits());
        let other = assign_fixture(&mut e, 3);
        e.call(
            0x00dc_98d0,
            &args![other, 0xa0u32, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(first_value(&e, other).0, 0x7fc0_0001);
    }

    #[test]
    fn assign_ov5_stores_the_short_shifted_into_the_high_half() {
        let mut e = fresh();
        for (short, bits) in [
            (0x3f80i16, 0x3f80_0000u32),
            (-1, 0xffff_0000),
            (0x7fa0, 0x7fe0_0000),
        ] {
            let object = assign_fixture(&mut e, 3);
            e.call(0x00dc_9910, &args![object, 0xa0u32, short]);
            assert_eq!(first_value(&e, object).0, bits, "{short:#x}");
        }
    }

    #[test]
    fn assign_ov6_stores_a_copy_of_the_floats() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0, 6)]);
        let object = dict_object(&mut e, class, &[]);
        let source = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        e.call(0x00dc_9950, &args![object, 0xa0u32, source, 3i32]);
        let copy = first_value(&e, object).0;
        assert_ne!(copy, source);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(copy + 4 * i)).collect::<Vec<_>>(),
            vec![0x100, 0x101, 0x102]
        );
        assert_eq!(calls(&e, ALLOCATE), vec![vec![allocator, 12]]);
    }

    #[test]
    fn assign_ov7_stores_by_the_type_of_the_member() {
        let mut e = fresh();
        for (member_type, expected) in [
            (1u32, (0x1234_5678u32, 0x9abc_def0u32)),
            (2, (0x1234_5678, 0x9abc_def0)),
            (3, (0.0f32.to_bits(), 0)),
            (8, (0, 0)),
        ] {
            let object = assign_fixture(&mut e, member_type);
            let value = if member_type == 3 {
                5u64
            } else {
                0x9abc_def0_1234_5678
            };
            e.call(0x00dc_99c0, &args![object, 0xa0u32, value]);
            let stored = first_value(&e, object);
            if member_type == 3 {
                assert_eq!(stored, (5.0f32.to_bits(), 0));
            } else {
                assert_eq!(stored, expected, "type {member_type}");
            }
        }
    }

    /// An implementation with an element array at `+0xc` (`data`, `size`, capacity word) of
    /// capacity 16 elements of `element_size` bytes, filled with junk bytes, and `size` elements.
    fn junk_array(e: &mut Engine, element_size: u32, size: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(16 * element_size);
        for i in 0..16 * element_size {
            e.mem.set_u8(data + i, 0xdd);
        }
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, size);
        e.mem.set_u32(this + 0x14, 16);
        this
    }

    #[test]
    fn vec_array_set_size_zeroes_the_new_words() {
        let mut e = fresh();
        basics(&mut e);
        let this = junk_array(&mut e, 4, 2);
        e.mem.set_u32(this + 0x18, 4);
        e.call(0x00dc_9b60, &args![this, 3i32]);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(this + 0x10), 12);
        assert_eq!(e.mem.u32(data + 4), 0xdddd_dddd);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 44), 0);
        assert_eq!(e.mem.u32(data + 48), 0xdddd_dddd);
    }

    #[test]
    fn object_array_set_size_zeroes_the_new_pointers() {
        let mut e = fresh();
        basics(&mut e);
        let this = junk_array(&mut e, 4, 1);
        e.call(0x00dc_9bd0, &args![this, 4i32]);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        assert_eq!(e.mem.u32(data), 0xdddd_dddd);
        assert_eq!(
            (1..4).map(|i| e.mem.u32(data + 4 * i)).collect::<Vec<_>>(),
            vec![0; 3]
        );
        assert_eq!(e.mem.u32(data + 16), 0xdddd_dddd);
        let shrunk = junk_array(&mut e, 4, 8);
        e.call(0x00dc_9bd0, &args![shrunk, 2i32]);
        assert_eq!(e.mem.u32(shrunk + 0x10), 2);
    }

    #[test]
    fn byte_array_set_size_zeroes_the_new_bytes() {
        let mut e = fresh();
        basics(&mut e);
        let this = junk_array(&mut e, 1, 2);
        e.call(0x00dc_9d10, &args![this, 6i32]);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(this + 0x10), 6);
        assert_eq!(e.mem.u8(data + 1), 0xdd);
        assert_eq!(
            (2..6).map(|i| e.mem.u8(data + i)).collect::<Vec<_>>(),
            vec![0; 4]
        );
        assert_eq!(e.mem.u8(data + 6), 0xdd);
        assert_eq!(calls(&e, MEM_SET), vec![vec![data + 2, 0, 4]]);
    }

    #[test]
    fn int64_array_set_size_zeroes_the_new_elements() {
        let mut e = fresh();
        basics(&mut e);
        let this = junk_array(&mut e, 8, 1);
        e.call(0x00dc_9db0, &args![this, 4i32]);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        assert_eq!(e.mem.u32(data + 4), 0xdddd_dddd);
        assert_eq!(
            (2..8).map(|i| e.mem.u32(data + 4 * i)).collect::<Vec<_>>(),
            vec![0; 6]
        );
        assert_eq!(e.mem.u32(data + 32), 0xdddd_dddd);
        e.call(0x00dc_9db0, &args![this, 2i32]);
        assert_eq!(e.mem.u32(this + 0x10), 2);
    }

    /// A struct array of a class whose slot 4 answers an owner with `tracker` at `+8`.
    fn struct_array(e: &mut Engine, tracker: u32) -> u32 {
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner + 8, tracker);
        e.register_double(SLOT_A, move |_, _| ret(owner));
        let class = object_with(e, 0x40, &[(4, SLOT_A)]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, class);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        this
    }

    #[test]
    fn struct_array_add_member_makes_the_members_array() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let (_, tracker) = make_world(&mut e);
        let this = struct_array(&mut e, tracker);
        let (name, kind) = (0xa0u32, text(&mut e, "K"));
        put_entry(&maps, tracker, "K", kind, 0xc1);
        let handle = e.mem.alloc(0x10);
        e.mem.set_u32(handle + 8, kind);
        let description = e.mem.alloc(0x18);
        e.mem.set_u32(description, name);
        e.mem.set_u32(description + 4, handle);
        e.mem.set_u32(description + 0x10, 1);
        assert_eq!(e.call(0x00dc_9c20, &args![this, description]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        let records = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(records), name);
        let array = e.mem.u32(records + 4);
        assert_eq!(e.mem.u32(array), 0x010d_707c);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.call(0x00dc_9c20, &args![this, description]).i32(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 2);
    }

    #[test]
    fn struct_array_add_member_without_a_type_adds_nothing() {
        let mut e = fresh();
        basics(&mut e);
        let this = e.mem.alloc(0x20);
        let description = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00dc_9c20, &args![this, description]).i32(), -1);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn class_add_member_appends_a_record_and_holds_the_handle() {
        let mut e = fresh();
        basics(&mut e);
        let class = class_dict(&mut e, 0, &[(1, 0, 1)]);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, 0x2002);
        e.call(0x00dc_9f20, &args![class, 0xa0u32, 3u32, handle, 7u32]);
        assert_eq!(e.mem.u32(class + 0x2c), 2);
        let record = e.mem.u32(class + 0x28) + 0x18;
        assert_eq!(e.mem.u32(record), 0xa0);
        assert_eq!(e.mem.u32(record + 4), 0x2002);
        assert_eq!(e.mem.u32(record + 8), 0);
        assert_eq!(e.mem.u32(record + 0xc), 0);
        assert_eq!(e.mem.u32(record + 0x10), 3);
        assert_eq!(e.mem.u32(record + 0x14), 7);
        assert_eq!(
            calls(&e, ADD_REFERENCE),
            vec![vec![DEFAULT_CLASS_HANDLE], vec![0x2002], vec![0x2002]]
        );
        assert_eq!(
            calls(&e, REMOVE_REFERENCE),
            vec![vec![DEFAULT_CLASS_HANDLE], vec![0x2002]]
        );
        let empty_handle = e.mem.alloc(4);
        e.call(
            0x00dc_9f20,
            &args![class, 0xb0u32, 4u32, empty_handle, 8u32],
        );
        assert_eq!(e.mem.u32(class + 0x2c), 3);
        assert_eq!(calls(&e, ADD_REFERENCE).len(), 4);
    }

    #[test]
    fn track_class_enters_a_new_class_and_its_derived_entry() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let inserts = record_inserts(&mut e);
        let (_, tracker) = make_world(&mut e);
        let parent = tracked_class(&mut e, tracker, &maps, "P", 0);
        let class = tracked_class(&mut e, tracker, &maps, "C", parent);
        // `tracked_class` entered both; forget `C` to track it afresh.
        maps.borrow_mut()
            .get_mut(&tracker)
            .unwrap()
            .retain(|entry| entry.0 != "C");
        e.call(0x00dc_a820, &args![tracker, class]);
        assert_eq!(map_value(&maps, tracker, "C"), Some(class));
        assert_eq!(
            *inserts.borrow(),
            vec![[tracker + tracker::DERIVED_MAP, parent, class]]
        );
        e.call(0x00dc_a820, &args![tracker, class]);
        assert_eq!(inserts.borrow().len(), 1);
        assert_eq!(maps.borrow()[&tracker].len(), 2);
    }

    #[test]
    fn track_class_of_a_root_class_only_enters_the_name() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let inserts = record_inserts(&mut e);
        let (_, tracker) = make_world(&mut e);
        let class = tracked_class(&mut e, tracker, &maps, "R", 0);
        maps.borrow_mut().get_mut(&tracker).unwrap().clear();
        e.call(0x00dc_a820, &args![tracker, class]);
        assert_eq!(map_value(&maps, tracker, "R"), Some(class));
        assert!(inserts.borrow().is_empty());
    }

    #[test]
    fn class_dict_constructor_fills_the_fields_and_empty_arrays() {
        let mut e = fresh();
        basics(&mut e);
        string_maps(&mut e);
        handle_doubles(&mut e);
        let (world, _) = make_world(&mut e);
        let class = e.mem.alloc(0x40);
        let name = text(&mut e, "Cls");
        assert_eq!(
            e.call(0x00dc_abb0, &args![class, world, name, 9u32]).u32(),
            class
        );
        assert_eq!(e.mem.u32(class), 0x010d_73c8);
        assert_eq!(e.mem.u32(class + 0xc), world);
        assert_eq!(e.mem.u32(class + 0x14), 9);
        assert_eq!(e.mem.u32(class + 0x18), 0);
        assert_eq!(e.mem.u32(e.mem.u32(class + 0x10) + 8), name);
        for array in [0x1cu32, 0x28, 0x34] {
            assert_eq!(e.mem.u32(class + array), 0);
            assert_eq!(e.mem.u32(class + array + 4), 0);
            assert_eq!(e.mem.u32(class + array + 8), 0x8000_0000);
        }
    }

    #[test]
    fn class_dict_get_name_reads_the_name_handle() {
        let mut e = fresh();
        let class = e.mem.alloc(0x40);
        let handle = e.mem.alloc(0x10);
        e.mem.set_u32(handle + 8, 0x4e4e);
        e.mem.set_u32(class + 0x10, handle);
        assert_eq!(e.call(0x00dc_ac10, &args![class]).u32(), 0x4e4e);
    }

    /// A class dict whose vtable has the two slots `getMemberIndexByName` and the struct arrays
    /// use besides the translation's own: slot `0x18` (declared member count, read from `+0x2c`)
    /// and slot `0x24` (the translated `getNumMembers`); `parent` is entered at `+0x18`.
    fn class_with_methods(
        e: &mut Engine,
        world: u32,
        members: &[(u32, u32, u32)],
        parent: u32,
    ) -> u32 {
        e.register(SLOT_C, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        let class = class_with_slots(e, world, &[(0x18, SLOT_C), (0x24, 0x00dc_ad80)]);
        let donor = class_dict(e, world, members);
        for offset in [0x28u32, 0x2c, 0x30] {
            let word = e.mem.u32(donor + offset);
            e.mem.set_u32(class + offset, word);
        }
        e.mem.set_u32(class + 0x18, parent);
        class
    }

    #[test]
    fn declared_member_info_describes_the_member_and_its_class() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let (world, tracker) = make_world(&mut e);
        let kind = text(&mut e, "K");
        put_entry(&maps, tracker, "K", kind, 0xc1);
        let handle = e.mem.alloc(0x10);
        e.mem.set_u32(handle + 8, kind);
        let class = class_dict(&mut e, world, &[(0xa0, 0, 1), (0xb0, handle, 6)]);
        let record = e.mem.u32(class + 0x28) + 0x18;
        e.mem.set_u32(record + 0x14, 0x77);
        let out = e.mem.alloc(0x14);
        e.call(0x00dc_ac20, &args![class, 1i32, out]);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0xb0, class, 6, 0xc1, 0x77]);
    }

    #[test]
    fn super_class_checks_the_names_up_the_chain_of_the_other_class() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let (world, tracker) = make_world(&mut e);
        let base_name = text(&mut e, "Base");
        put_entry(&maps, tracker + 0x4c, "Base", base_name, base_name);
        let class = class_dict(&mut e, world, &[]);
        let handle = e.mem.alloc(0x10);
        e.mem.set_u32(handle + 8, base_name);
        e.mem.set_u32(class + 0x10, handle);
        let base = tracked_class(&mut e, tracker, &maps, "Base", 0);
        let derived = tracked_class(&mut e, tracker, &maps, "Derived", base);
        let unrelated = tracked_class(&mut e, tracker, &maps, "Other", 0);
        let out = e.mem.alloc(4);
        for (other, expected) in [(derived, 1), (base, 1), (unrelated, 0), (0, 0)] {
            e.mem.set_u8(out, 0xee);
            e.call(0x00dc_ac80, &args![class, out, other]);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn fn_00dcac80_answers_its_result_pointer() {
        let mut e = fresh();
        basics(&mut e);
        let class = e.mem.alloc(0x40);
        let out = e.mem.alloc(4);
        // No other class: nothing is compared; EAX is `out` at both RETs (the hkBool result
        // pointer the callers read through).
        assert_eq!(e.call(0x00dc_ac80, &args![class, out, 0u32]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn declared_member_index_finds_the_interned_name() {
        let mut e = fresh();
        basics(&mut e);
        let (class, _, names) = named_members(&mut e, &["a", "b"]);
        let members = e.mem.alloc(0x18 * 2);
        e.mem.set_u32(members, names[0]);
        e.mem.set_u32(members + 0x18, names[1]);
        e.mem.set_u32(class + 0x28, members);
        e.mem.set_u32(class + 0x2c, 2);
        let (raw_a, raw_b, raw_c) = (text(&mut e, "a"), text(&mut e, "b"), text(&mut e, "c"));
        assert_eq!(e.call(0x00dc_ad10, &args![class, raw_a]).i32(), 0);
        assert_eq!(e.call(0x00dc_ad10, &args![class, raw_b]).i32(), 1);
        assert_eq!(e.call(0x00dc_ad10, &args![class, raw_c]).i32(), -1);
        assert_eq!(e.call(0x00dc_ad10, &args![class, 0u32]).i32(), -1);
    }

    #[test]
    fn member_counts_add_up_along_the_parents() {
        let mut e = fresh();
        let grandparent = class_dict(&mut e, 0, &[(1, 0, 1), (2, 0, 1)]);
        let parent = class_dict(&mut e, 0, &[(3, 0, 1)]);
        let child = class_dict(&mut e, 0, &[(4, 0, 1), (5, 0, 1), (6, 0, 1)]);
        e.mem.set_u32(parent + 0x18, grandparent);
        e.mem.set_u32(child + 0x18, parent);
        assert_eq!(e.call(0x00dc_ad80, &args![child]).i32(), 6);
        assert_eq!(e.call(0x00dc_ad80, &args![grandparent]).i32(), 2);
    }

    /// A world with the names `names` interned (pointer = value) and a parent class of the first
    /// two, a child class of the third; returns `(world, tracker, parent, child, names)`.
    fn two_level_classes(e: &mut Engine) -> (u32, u32, u32, u32, Vec<u32>) {
        let maps = string_maps(e);
        let (world, tracker) = make_world(e);
        let names: Vec<u32> = ["p1", "p2", "c1"]
            .iter()
            .map(|name| {
                let pointer = text(e, name);
                put_entry(&maps, tracker + 0x4c, name, pointer, pointer);
                pointer
            })
            .collect();
        let handle = e.mem.alloc(0x10);
        let parent =
            class_with_methods(e, world, &[(names[0], handle, 1), (names[1], handle, 2)], 0);
        let child = class_with_methods(e, world, &[(names[2], handle, 3)], parent);
        (world, tracker, parent, child, names)
    }

    #[test]
    fn member_index_counts_the_ancestors_members_first() {
        let mut e = fresh();
        basics(&mut e);
        let (_, _, _, child, _) = two_level_classes(&mut e);
        for (name, expected) in [("c1", 2), ("p2", 1), ("p1", 0), ("zz", -1)] {
            let raw = text(&mut e, name);
            assert_eq!(
                e.call(0x00dc_ada0, &args![child, raw]).i32(),
                expected,
                "{name}"
            );
        }
        assert_eq!(e.call(0x00dc_ada0, &args![child, 0u32]).i32(), -1);
    }

    #[test]
    fn member_info_counts_the_ancestors_members_first() {
        let mut e = fresh();
        basics(&mut e);
        let (_, _, parent, child, names) = two_level_classes(&mut e);
        let out = e.mem.alloc(0x14);
        for (index, name, member_type) in [(0, names[0], 1), (1, names[1], 2), (2, names[2], 3)] {
            e.call(0x00dc_ae30, &args![child, index, out]);
            assert_eq!(e.mem.u32(out), name, "index {index}");
            assert_eq!(e.mem.u32(out + 4), child);
            assert_eq!(e.mem.u32(out + 8), member_type);
            assert_eq!(e.mem.u32(out + 0xc), 0);
        }
        let untouched = e.mem.alloc(0x14);
        e.call(0x00dc_ae30, &args![child, -1i32, untouched]);
        assert_eq!(e.mem.u32(untouched), 0);
        let _ = parent;
    }

    const STUB_A: u32 = 0x0910_0000;
    const STUB_B: u32 = 0x0910_1000;

    /// An object whose every virtual slot `s` is a double at `base + s` answering `0x99`.
    fn stub_object(e: &mut Engine, base: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            e.register(base + slot, |_, _| ret(0x99));
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A struct array `this` of a class in a world that has `a` and `b` interned, with the records
    /// `{a, first}`, `{b, second}` (the arrays are stub objects `STUB_A`, `STUB_B`), and a view
    /// `Object` of it at index 7. Returns `(this, view, arrays, names)`.
    fn struct_fixture(e: &mut Engine) -> (u32, u32, [u32; 2], [u32; 2]) {
        basics(e);
        let maps = string_maps(e);
        let (world, tracker) = make_world(e);
        let (a, b) = (text(e, "a"), text(e, "b"));
        put_entry(&maps, tracker + 0x4c, "a", a, a);
        put_entry(&maps, tracker + 0x4c, "b", b, b);
        let class = class_dict(e, world, &[]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, class);
        let (first, second) = (stub_object(e, STUB_A, 0x20), stub_object(e, STUB_B, 0x20));
        let records = e.mem.alloc(0x10);
        for (i, (name, array)) in [(a, first), (b, second)].into_iter().enumerate() {
            e.mem.set_u32(records + 8 * i as u32, name);
            e.mem.set_u32(records + 8 * i as u32 + 4, array);
        }
        e.mem.set_u32(this + 0xc, records);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 2);
        let view = e.mem.alloc(0x14);
        e.mem.set_u32(view + 0xc, this);
        e.mem.set_u32(view + 0x10, 7);
        (this, view, [first, second], [a, b])
    }

    /// Calls the view function at `address` with the name `b` and `extra`, and answers what the
    /// array of the record `b` got at `slot`.
    fn forwarded(address: u32, slot: u32, extra: &[u32]) -> (Vec<Vec<u32>>, u32) {
        let mut e = fresh();
        let (_, view, arrays, _) = struct_fixture(&mut e);
        let raw = text(&mut e, "b");
        let mut words = vec![view, raw];
        words.extend_from_slice(extra);
        e.call(address, &words);
        (calls(&e, STUB_B + slot), arrays[1])
    }

    #[test]
    fn view_as_vec_asks_slot_0x20_for_the_object_index() {
        let (got, array) = forwarded(0x00dc_b340, 0x20, &[0]);
        assert_eq!(got, vec![vec![array, 7]]);
    }

    #[test]
    fn view_as_string_asks_slot_0x28() {
        let (got, array) = forwarded(0x00dc_b370, 0x28, &[]);
        assert_eq!(got, vec![vec![array, 7]]);
    }

    #[test]
    fn view_as_array_asks_slot_0x58() {
        let (got, array) = forwarded(0x00dc_b3a0, 0x58, &[]);
        assert_eq!(got, vec![vec![array, 7]]);
    }

    #[test]
    fn view_as_object_asks_slot_0x50() {
        let (got, array) = forwarded(0x00dc_b460, 0x50, &[]);
        assert_eq!(got, vec![vec![array, 7]]);
    }

    #[test]
    fn view_as_int_sign_extends_the_answer() {
        let mut e = fresh();
        let (_, view, arrays, _) = struct_fixture(&mut e);
        e.register(STUB_B + 0x40, |_, _| ret(0xffff_fff0));
        let raw = text(&mut e, "b");
        assert_eq!(
            e.call(0x00dc_b3d0, &args![view, raw]).u64(),
            0xffff_ffff_ffff_fff0
        );
        assert_eq!(calls(&e, STUB_B + 0x40), vec![vec![arrays[1], 7]]);
    }

    #[test]
    fn view_as_real_passes_st0_through() {
        let mut e = fresh();
        let (_, view, arrays, _) = struct_fixture(&mut e);
        e.register(STUB_B + 0x30, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let raw = text(&mut e, "b");
        assert_eq!(e.call(0x00dc_b400, &args![view, raw]).f64(), 2.5);
        assert_eq!(calls(&e, STUB_B + 0x30), vec![vec![arrays[1], 7]]);
    }

    #[test]
    fn view_as_half_hands_the_out_pointer_to_slot_0x38() {
        let mut e = fresh();
        let (_, view, arrays, _) = struct_fixture(&mut e);
        let (out, raw) = (e.mem.alloc(8), text(&mut e, "b"));
        assert_eq!(e.call(0x00dc_b430, &args![view, out, raw]).u32(), out);
        assert_eq!(calls(&e, STUB_B + 0x38), vec![vec![arrays[1], out, 7]]);
    }

    #[test]
    fn view_assign_ov2_stores_an_object_through_slot_0x54() {
        let (got, array) = forwarded(0x00dc_b690, 0x54, &[0xaa]);
        assert_eq!(got, vec![vec![array, 7, 0xaa]]);
    }

    #[test]
    fn view_assign_ov3_stores_an_array_through_slot_0x5c() {
        let (got, array) = forwarded(0x00dc_b6c0, 0x5c, &[0xaa]);
        assert_eq!(got, vec![vec![array, 7, 0xaa]]);
    }

    #[test]
    fn view_assign_ov4_stores_a_string_through_slot_0x2c() {
        let (got, array) = forwarded(0x00dc_b6f0, 0x2c, &[0xaa]);
        assert_eq!(got, vec![vec![array, 7, 0xaa]]);
    }

    #[test]
    fn view_assign_ov5_stores_the_float_quieted_through_slot_0x34() {
        let (got, array) = forwarded(0x00dc_b720, 0x34, &[0x7f80_0001]);
        assert_eq!(got, vec![vec![array, 7, 0x7fc0_0001]]);
        let (got, array) = forwarded(0x00dc_b720, 0x34, &[1.5f32.to_bits()]);
        assert_eq!(got, vec![vec![array, 7, 1.5f32.to_bits()]]);
    }

    #[test]
    fn view_assign_ov6_stores_through_slot_0x3c() {
        let (got, array) = forwarded(0x00dc_b760, 0x3c, &[0xaa]);
        assert_eq!(got, vec![vec![array, 7, 0xaa]]);
    }

    #[test]
    fn view_assign_ov7_stores_the_vector_through_slot_0x24() {
        let (got, array) = forwarded(0x00dc_b790, 0x24, &[0xaa, 0xbb]);
        assert_eq!(got, vec![vec![array, 7, 0xaa]]);
    }

    #[test]
    fn view_assign_ov8_stores_the_sign_extended_byte_through_slot_0x44() {
        let (got, array) = forwarded(0x00dc_b7c0, 0x44, &[0xfd]);
        assert_eq!(got, vec![vec![array, 7, 0xffff_fffd]]);
    }

    #[test]
    fn view_assign_ov9_stores_the_sign_extended_short_through_slot_0x44() {
        let (got, array) = forwarded(0x00dc_b7f0, 0x44, &[0x8001]);
        assert_eq!(got, vec![vec![array, 7, 0xffff_8001]]);
    }

    #[test]
    fn view_assign_ov10_stores_the_int_through_slot_0x44() {
        let (got, array) = forwarded(0x00dc_b820, 0x44, &[0x1234]);
        assert_eq!(got, vec![vec![array, 7, 0x1234]]);
    }

    #[test]
    fn view_assign_ov11_stores_both_words_through_slot_0x4c() {
        let (got, array) = forwarded(0x00dc_b850, 0x4c, &[0x11, 0x22]);
        assert_eq!(got, vec![vec![array, 7, 0x11, 0x22]]);
    }

    #[test]
    fn view_assign_sets_the_value_by_the_type_of_the_array() {
        let mut e = fresh();
        let (_, view, arrays, _) = struct_fixture(&mut e);
        e.register(STUB_B + 4, |_, _| ret(1));
        let seen: Rc<RefCell<Vec<[u32; 2]>>> = Rc::default();
        let record = Rc::clone(&seen);
        e.register_double(DATA_OBJECT_VALUE_AS_INT, move |e, a| {
            record
                .borrow_mut()
                .push([e.mem.u32(a[0]), e.mem.u32(a[0] + 4)]);
            ret(0x55)
        });
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x31);
        e.mem.set_u32(value + 4, 0x32);
        let raw = text(&mut e, "b");
        e.call(0x00dc_b650, &args![view, raw, value]);
        assert_eq!(*seen.borrow(), vec![[0x31, 0x32]]);
        assert_eq!(calls(&e, STUB_B + 0x44), vec![vec![arrays[1], 7, 0x55]]);
    }

    #[test]
    fn view_intern_goes_through_the_class_of_the_array() {
        let mut e = fresh();
        let (_, view, _, names) = struct_fixture(&mut e);
        let (out, raw) = (e.mem.alloc(4), text(&mut e, "a"));
        assert_eq!(e.call(0x00dc_b320, &args![view, out, raw]).u32(), out);
        assert_eq!(e.mem.u32(out), names[0]);
    }

    #[test]
    fn view_has_member_compares_the_interned_name_with_the_records() {
        let mut e = fresh();
        let (this, _, _, names) = struct_fixture(&mut e);
        let interned = names[1];
        e.register_double(SLOT_A, move |e, a| {
            e.mem.set_u32(a[1], interned);
            ret(a[1])
        });
        let view = object_with(&mut e, 0x14, &[(0x60, SLOT_A)]);
        e.mem.set_u32(view + 0xc, this);
        let raw = text(&mut e, "b");
        assert!(e.call(0x00dc_b120, &args![view, raw]).bool());
        assert_eq!(calls(&e, SLOT_A)[0][2], raw);
        e.register(SLOT_A, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            ret(a[1])
        });
        assert!(!e.call(0x00dc_b120, &args![view, raw]).bool());
    }

    #[test]
    fn view_is_valid_compares_with_the_record_count() {
        let mut e = fresh();
        let (_, view, _, _) = struct_fixture(&mut e);
        assert!(e.call(0x00dc_b170, &args![view, 1i32]).bool());
        assert!(!e.call(0x00dc_b170, &args![view, 2i32]).bool());
        assert!(e.call(0x00dc_b170, &args![view, -1i32]).bool());
    }

    #[test]
    fn view_member_name_and_value_read_the_record() {
        let mut e = fresh();
        let (_, view, _, names) = struct_fixture(&mut e);
        assert_eq!(e.call(0x00dc_b190, &args![view, 1i32]).u32(), names[1]);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_b1a0, &args![view, out, 0i32]).u32(), out);
        assert_eq!(e.mem.u32(out), view);
        assert_eq!(e.mem.u32(out + 4), names[0]);
    }

    #[test]
    fn handle_of_an_object_is_the_object_and_zero() {
        let mut e = fresh();
        let out = e.mem.alloc(8);
        e.mem.set_u32(out + 4, 9);
        assert_eq!(e.call(0x00dc_b0d0, &args![0x1234u32, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x1234);
        assert_eq!(e.mem.u32(out + 4), 0);
    }

    #[test]
    fn struct_array_as_object_makes_a_view() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let this = e.mem.alloc(0x20);
        let view = e.call(0x00dc_b0e0, &args![this, 7i32]).u32();
        assert_eq!(calls(&e, ALLOCATE), vec![vec![allocator, 0x14]]);
        assert_eq!(e.mem.u16(view + 4), 0x14);
        assert_eq!(e.mem.u32(view), 0x010d_7494);
        assert_eq!(e.mem.u32(view + 0xc), this);
        assert_eq!(e.mem.u32(view + 0x10), 7);
        assert_eq!(e.mem.u32(view + 8), 0);
    }

    #[test]
    fn struct_array_set_object_stores_each_member_of_the_object() {
        let mut e = fresh();
        let (this, _, arrays, names) = struct_fixture(&mut e);
        for base in [STUB_A, STUB_B] {
            e.register(base + 4, |_, _| ret(1));
        }
        e.register(DATA_OBJECT_VALUE_AS_INT, |_, _| ret(0x55));
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x31);
        e.mem.set_u32(value + 4, 0x32);
        e.register_double(SLOT_B, move |_, _| ret(value));
        let source = object_with(&mut e, 0x20, &[(0xc, SLOT_B)]);
        e.call(0x00dc_b1c0, &args![this, 5i32, source]);
        let asked: Vec<u32> = calls(&e, SLOT_B).iter().map(|call| call[2]).collect();
        assert_eq!(asked, names.to_vec());
        assert_eq!(calls(&e, STUB_A + 0x44), vec![vec![arrays[0], 5, 0x55]]);
        assert_eq!(calls(&e, STUB_B + 0x44), vec![vec![arrays[1], 5, 0x55]]);
    }

    #[test]
    fn struct_array_clear_clears_every_array_and_the_size() {
        let mut e = fresh();
        let (this, _, arrays, _) = struct_fixture(&mut e);
        e.mem.set_u32(this + 0x1c, 5);
        e.call(0x00dc_b210, &args![this]);
        assert_eq!(calls(&e, STUB_A + 8), vec![vec![arrays[0]]]);
        assert_eq!(calls(&e, STUB_B + 8), vec![vec![arrays[1]]]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn struct_array_reserve_reserves_in_every_array() {
        let mut e = fresh();
        let (this, _, arrays, _) = struct_fixture(&mut e);
        e.call(0x00dc_b250, &args![this, 9i32]);
        assert_eq!(calls(&e, STUB_A + 0xc), vec![vec![arrays[0], 9]]);
        assert_eq!(calls(&e, STUB_B + 0xc), vec![vec![arrays[1], 9]]);
    }

    #[test]
    fn struct_array_set_size_sizes_every_array_and_keeps_the_size() {
        let mut e = fresh();
        let (this, _, arrays, _) = struct_fixture(&mut e);
        e.call(0x00dc_b280, &args![this, 9i32]);
        assert_eq!(e.mem.u32(this + 0x1c), 9);
        assert_eq!(calls(&e, STUB_A + 0x10), vec![vec![arrays[0], 9]]);
        assert_eq!(calls(&e, STUB_B + 0x10), vec![vec![arrays[1], 9]]);
        let empty = e.mem.alloc(0x20);
        e.call(0x00dc_b280, &args![empty, 4i32]);
        assert_eq!(e.mem.u32(empty + 0x1c), 4);
    }

    #[test]
    fn struct_array_find_gives_the_record_index_of_the_interned_name() {
        let mut e = fresh();
        let (this, _, _, _) = struct_fixture(&mut e);
        for (name, expected) in [("a", 0), ("b", 1), ("zz", -1)] {
            let raw = text(&mut e, name);
            assert_eq!(
                e.call(0x00dc_b2b0, &args![this, raw]).i32(),
                expected,
                "{name}"
            );
        }
        assert_eq!(e.call(0x00dc_b2b0, &args![this, 0u32]).i32(), -1);
    }

    #[test]
    fn struct_array_swizzle_gives_the_array_of_the_named_record() {
        let mut e = fresh();
        let (this, _, arrays, _) = struct_fixture(&mut e);
        let raw = text(&mut e, "b");
        assert_eq!(e.call(0x00dc_b490, &args![this, raw]).u32(), arrays[1]);
    }

    #[test]
    fn struct_array_find_or_add_finds_an_existing_record() {
        let mut e = fresh();
        let (this, _, _, _) = struct_fixture(&mut e);
        let raw = text(&mut e, "b");
        assert_eq!(e.call(0x00dc_b5d0, &args![this, raw]).i32(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 2);
    }

    /// An empty struct array whose class has the member `c1` (type 1, no class name) and answers
    /// slot `0x2c` (`getMemberIndexByName`) with 0, slot 4 with an owner of the tracker, and whose
    /// own slot `0x14` (the size) says 5. Returns `(this, class, name of c1)`.
    fn adding_fixture(e: &mut Engine) -> (u32, u32, u32) {
        basics(e);
        let maps = string_maps(e);
        let (world, tracker) = make_world(e);
        let name = text(e, "c1");
        put_entry(&maps, tracker + 0x4c, "c1", name, name);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner + 8, tracker);
        e.register_double(SLOT_A, move |_, _| ret(owner));
        e.register(SLOT_B, |_, _| ret(0));
        e.register(SLOT_D, |_, _| ret(5));
        e.register(SLOT_E, |_, _| Ret::default());
        e.mem.set_u32(0x010d_707c + 0x10, SLOT_E);
        let handle = e.mem.alloc(0x10);
        let class = class_with_methods(e, world, &[(name, handle, 1)], 0);
        let vtable = e.mem.u32(class);
        e.mem.set_u32(vtable + 4, SLOT_A);
        e.mem.set_u32(vtable + 0x2c, SLOT_B);
        let this = object_with(e, 0x20, &[(0x14, SLOT_D)]);
        e.mem.set_u32(this + 0x18, class);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        (this, class, name)
    }

    #[test]
    fn struct_array_add_member_adds_the_record_of_the_class_member_and_sizes_it() {
        let mut e = fresh();
        let (this, class, name) = adding_fixture(&mut e);
        let raw = text(&mut e, "c1");
        assert_eq!(e.call(0x00dc_b550, &args![this, raw]).i32(), 0);
        assert_eq!(calls(&e, SLOT_B), vec![vec![class, raw]]);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        let records = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(records), name);
        let array = e.mem.u32(records + 4);
        assert_eq!(e.mem.u32(array), 0x010d_707c);
        assert_eq!(calls(&e, SLOT_E), vec![vec![array, 5]]);
    }

    #[test]
    fn struct_array_find_or_add_adds_a_missing_record() {
        let mut e = fresh();
        let (this, _, name) = adding_fixture(&mut e);
        let raw = text(&mut e, "c1");
        assert_eq!(e.call(0x00dc_b5d0, &args![this, raw]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0xc)), name);
        assert_eq!(e.call(0x00dc_b5d0, &args![this, raw]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 1);
    }

    #[test]
    fn struct_array_constructor_adds_a_record_for_each_class_member() {
        let mut e = fresh();
        basics(&mut e);
        let maps = string_maps(&mut e);
        let inserts = record_inserts(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let kind = text(&mut e, "K");
        put_entry(&maps, tracker, "K", kind, 0xc1);
        let handle = e.mem.alloc(0x10);
        e.mem.set_u32(handle + 8, kind);
        e.register_double(SLOT_A, move |_, _| ret(world));
        e.register(SLOT_E, |_, _| ret(0xabcd));
        let class = class_with_methods(&mut e, world, &[(0xa0, handle, 1), (0xb0, handle, 10)], 0);
        let vtable = e.mem.u32(class);
        e.mem.set_u32(vtable + 4, SLOT_A);
        e.mem.set_u32(class + 8, 1);
        e.mem.set_u32(0x010d_73fc + 0x18, SLOT_E);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_aeb0, &args![this, class]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_73fc);
        assert_eq!(e.mem.u32(this + 0x18), class);
        assert_eq!(e.mem.u32(class + 8), 2);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        let records = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(records), 0xa0);
        assert_eq!(e.mem.u32(records + 8), 0xb0);
        assert_eq!(e.mem.u32(e.mem.u32(records + 4)), 0x010d_707c);
        assert_eq!(e.mem.u32(e.mem.u32(records + 12)), 0x010d_7244);
        assert_eq!(
            *inserts.borrow(),
            vec![[tracker + tracker::ARRAY_MAP, 0xabcd, this]]
        );
    }

    /// A stub array implementation whose `getType` (slot 4) answers `array_type`.
    fn typed_stub(e: &mut Engine, array_type: u32) -> u32 {
        let array = stub_object(e, STUB_A, 0x20);
        e.register_double(STUB_A + 4, move |_, _| ret(array_type));
        array
    }

    /// Doubles that read the two words of an `hkDataObject::Value` at `value`: records them.
    fn record_value_words(e: &mut Engine, address: u32, answer: u32) -> Rc<RefCell<Vec<[u32; 2]>>> {
        let seen: Rc<RefCell<Vec<[u32; 2]>>> = Rc::default();
        let record = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            record
                .borrow_mut()
                .push([e.mem.u32(a[0]), e.mem.u32(a[0] + 4)]);
            ret(answer)
        });
        seen
    }

    #[test]
    fn array_set_with_the_tuple_bit_stores_the_array_value() {
        let mut e = fresh();
        let array = typed_stub(&mut e, 0x21);
        let seen: Rc<RefCell<Vec<[u32; 2]>>> = Rc::default();
        let record = Rc::clone(&seen);
        e.register_double(DATA_OBJECT_VALUE_AS_ARRAY, move |e, a| {
            record
                .borrow_mut()
                .push([e.mem.u32(a[0]), e.mem.u32(a[0] + 4)]);
            e.mem.set_u32(a[1], 0xa77a);
            ret(a[1])
        });
        e.register(DATA_ARRAY_DESTRUCT, |_, _| Ret::default());
        e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
        assert_eq!(*seen.borrow(), vec![[0x31, 0x32]]);
        assert_eq!(calls(&e, STUB_A + 0x5c), vec![vec![array, 3, 0xa77a]]);
        assert_eq!(calls(&e, DATA_ARRAY_DESTRUCT).len(), 1);
    }

    #[test]
    fn array_set_stores_ints_through_slot_0x44() {
        for array_type in [1u32, 2] {
            let mut e = fresh();
            let array = typed_stub(&mut e, array_type);
            let seen = record_value_words(&mut e, DATA_OBJECT_VALUE_AS_INT, 0x55);
            e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
            assert_eq!(*seen.borrow(), vec![[0x31, 0x32]]);
            assert_eq!(calls(&e, STUB_A + 0x44), vec![vec![array, 3, 0x55]]);
        }
    }

    #[test]
    fn array_set_stores_a_real_as_a_float_through_slot_0x34() {
        let mut e = fresh();
        let array = typed_stub(&mut e, 3);
        e.register(DATA_OBJECT_VALUE_AS_REAL, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
        assert_eq!(
            calls(&e, STUB_A + 0x34),
            vec![vec![array, 3, 1.5f32.to_bits()]]
        );
    }

    #[test]
    fn array_set_stores_vectors_of_the_size_of_the_type_through_slot_0x24() {
        for (array_type, count) in [(4u32, 4u32), (5, 8), (6, 12), (7, 16)] {
            let mut e = fresh();
            let array = typed_stub(&mut e, array_type);
            e.register(DATA_OBJECT_VALUE_AS_VEC, |_, _| ret(0xf10a));
            e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
            assert_eq!(
                calls(&e, DATA_OBJECT_VALUE_AS_VEC)[0][1],
                count,
                "type {array_type}"
            );
            assert_eq!(calls(&e, STUB_A + 0x24), vec![vec![array, 3, 0xf10a]]);
        }
    }

    #[test]
    fn array_set_stores_objects_through_slot_0x54_and_destroys_the_temporary() {
        for array_type in [8u32, 9] {
            let mut e = fresh();
            let array = typed_stub(&mut e, array_type);
            e.register(DATA_OBJECT_VALUE_AS_OBJECT, |e, a| {
                e.mem.set_u32(a[1], 0x0b1e);
                ret(a[1])
            });
            e.register(DATA_OBJECT_DESTRUCT, |_, _| Ret::default());
            e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
            assert_eq!(calls(&e, STUB_A + 0x54), vec![vec![array, 3, 0x0b1e]]);
            assert_eq!(calls(&e, DATA_OBJECT_DESTRUCT).len(), 1);
        }
    }

    #[test]
    fn array_set_stores_strings_through_slot_0x2c() {
        let mut e = fresh();
        let array = typed_stub(&mut e, 10);
        e.register(DATA_OBJECT_VALUE_AS_STRING, |_, _| ret(0x7e47));
        e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
        assert_eq!(calls(&e, STUB_A + 0x2c), vec![vec![array, 3, 0x7e47]]);
    }

    #[test]
    fn array_set_ignores_other_types() {
        for array_type in [0u32, 11, 0x1f] {
            let mut e = fresh();
            let array = typed_stub(&mut e, array_type);
            e.call(0x00dc_af70, &args![array, 3i32, 0x31u32, 0x32u32]);
            let log = e.call_log.as_ref().unwrap();
            assert!(log
                .iter()
                .all(|(address, _)| *address == 0x00dc_af70 || *address == STUB_A + 4));
        }
    }

    /// A class `C` whose world answers slot `0x24` with `0xc1a5` and whose member `0xa0` has
    /// `member_type`, the extra word `extra` and a class-name handle naming `K`; the byte array's
    /// `setSize` slot is a double. Returns the class.
    fn read_fixture(e: &mut Engine, member_type: u32, extra: u32) -> u32 {
        basics(e);
        e.register(SLOT_A, |_, _| ret(0xc1a5));
        e.register(SLOT_E, |_, _| Ret::default());
        e.mem.set_u32(0x010d_707c + 0x10, SLOT_E);
        let world = object_with(e, 0x0c, &[(0x24, SLOT_A)]);
        let handle = e.mem.alloc(0x10);
        let kind = text(e, "K");
        e.mem.set_u32(handle + 8, kind);
        let class = class_dict(e, world, &[(0xa0, handle, member_type)]);
        let record = e.mem.u32(class + 0x28);
        e.mem.set_u32(record + 0x14, extra);
        class
    }

    #[test]
    fn read_access_of_an_unknown_member_is_empty() {
        let mut e = fresh();
        let class = read_fixture(&mut e, 8, 0);
        let object = dict_object(&mut e, class, &[]);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0x1111);
        assert_eq!(
            e.call(0x00dc_ba40, &args![object, out, 0xffu32, 0u32])
                .u32(),
            out
        );
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.i32(out + 4), -1);
        assert_eq!(e.mem.u32(object + 0x14), 0);
    }

    #[test]
    fn read_access_finds_the_last_record_of_the_name() {
        let mut e = fresh();
        let class = read_fixture(&mut e, 8, 0);
        let object = dict_object(&mut e, class, &[(0xa0, 1), (0xb0, 2), (0xa0, 3)]);
        let out = e.mem.alloc(8);
        e.call(0x00dc_ba40, &args![object, out, 0xa0u32, 0u32]);
        assert_eq!(e.mem.u32(out), e.mem.u32(class + 0x28));
        assert_eq!(e.mem.i32(out + 4), 2);
        assert_eq!(e.mem.u32(object + 0x14), 3);
    }

    #[test]
    fn read_access_makes_a_missing_object_member_null() {
        let mut e = fresh();
        let class = read_fixture(&mut e, 8, 0);
        let object = dict_object(&mut e, class, &[]);
        let data = e.mem.u32(object + 0x10);
        e.mem.set_u32(data + 8, 0xdddd);
        let out = e.mem.alloc(8);
        e.call(0x00dc_ba40, &args![object, out, 0xa0u32, 0u32]);
        assert_eq!(e.mem.i32(out + 4), 0);
        assert_eq!(e.mem.u32(object + 0x14), 1);
        assert_eq!(e.mem.u32(data), 0xa0);
        assert_eq!(e.mem.u32(data + 8), 0);
    }

    #[test]
    fn read_access_zeroes_a_missing_plain_member() {
        let mut e = fresh();
        let class = read_fixture(&mut e, 3, 0);
        let object = dict_object(&mut e, class, &[]);
        let data = e.mem.u32(object + 0x10);
        e.mem.set_u32(data + 8, 0xdddd);
        e.mem.set_u32(data + 0xc, 0xdddd);
        let out = e.mem.alloc(8);
        e.call(0x00dc_ba40, &args![object, out, 0xa0u32, 0u32]);
        assert_eq!((e.mem.u32(data + 8), e.mem.u32(data + 0xc)), (0, 0));
        assert_eq!(e.mem.u32(out), e.mem.u32(class + 0x28));
    }

    #[test]
    fn read_access_makes_an_array_for_an_array_member() {
        let mut e = fresh();
        let class = read_fixture(&mut e, 0x11, 0);
        let object = dict_object(&mut e, class, &[]);
        let out = e.mem.alloc(8);
        e.call(0x00dc_ba40, &args![object, out, 0xa0u32, 0u32]);
        let array = e.mem.u32(e.mem.u32(object + 0x10) + 8);
        assert_eq!(e.mem.u32(array), 0x010d_707c);
        assert_eq!(e.mem.u32(array + 8), 1);
        let asked = calls(&e, SLOT_A);
        assert_eq!(asked.len(), 1);
        assert_eq!(read_text(&e, asked[0][1]), "K");
        assert!(calls(&e, SLOT_E).is_empty());
    }

    #[test]
    fn read_access_sizes_the_array_of_a_tuple_member_to_the_extra_word() {
        let mut e = fresh();
        let class = read_fixture(&mut e, 0x21, 6);
        let object = dict_object(&mut e, class, &[]);
        let out = e.mem.alloc(8);
        e.call(0x00dc_ba40, &args![object, out, 0xa0u32, 0u32]);
        let array = e.mem.u32(e.mem.u32(object + 0x10) + 8);
        assert_eq!(calls(&e, SLOT_E), vec![vec![array, 6]]);
        assert_eq!(e.mem.u32(array + 8), 1);
    }

    /// Reads member `0xa0` of an object with the single record `{0xa0, value}` and the extra word
    /// `0xcafe` in the record's second value word, through the accessor at `address`.
    fn read_with(address: u32, member_type: u32, value: u32, extra_args: &[u32]) -> (Engine, Ret) {
        let mut e = fresh();
        let class = read_fixture(&mut e, member_type, 0);
        let object = dict_object(&mut e, class, &[(0xa0, value)]);
        let data = e.mem.u32(object + 0x10);
        e.mem.set_u32(data + 0xc, 0xcafe);
        let mut words = vec![object, 0xa0];
        words.extend_from_slice(extra_args);
        let result = e.call(address, &words);
        (e, result)
    }

    #[test]
    fn as_array_reads_the_value_word() {
        let (_, result) = read_with(0x00dc_bbf0, 8, 0xa77a, &[]);
        assert_eq!(result.u32(), 0xa77a);
    }

    #[test]
    fn as_string_reads_the_value_word() {
        let (_, result) = read_with(0x00dc_bc20, 8, 0xa77a, &[]);
        assert_eq!(result.u32(), 0xa77a);
    }

    #[test]
    fn as_object_reads_the_value_word() {
        let (_, result) = read_with(0x00dc_bca0, 8, 0xa77a, &[]);
        assert_eq!(result.u32(), 0xa77a);
    }

    #[test]
    fn as_vec_reads_the_value_word() {
        let (_, result) = read_with(0x00dc_bcd0, 6, 0xa77a, &[3]);
        assert_eq!(result.u32(), 0xa77a);
    }

    #[test]
    fn as_int_reads_both_words_of_an_int_member_and_zero_for_others() {
        let (_, result) = read_with(0x00dc_bc50, 1, 0x1234, &[]);
        assert_eq!(result.u64(), 0xcafe_0000_1234);
        let (_, result) = read_with(0x00dc_bc50, 2, 0x1234, &[]);
        assert_eq!(result.u64(), 0xcafe_0000_1234);
        let (_, result) = read_with(0x00dc_bc50, 3, 0x1234, &[]);
        assert_eq!(result.u64(), 0);
    }

    #[test]
    fn as_real_loads_the_float_quieted() {
        let (_, result) = read_with(0x00dc_bd10, 3, 1.5f32.to_bits(), &[]);
        assert_eq!(result.f64(), 1.5);
        let (_, result) = read_with(0x00dc_bd10, 3, 0x7f80_0001, &[]);
        assert!(result.f64().is_nan());
    }

    #[test]
    fn assign_ov8_copies_an_object_value_and_takes_a_reference() {
        let mut e = fresh();
        let source_class = read_fixture(&mut e, 8, 0);
        let value = e.mem.alloc(0x20);
        e.mem.set_u32(value + 8, 1);
        let source_object = dict_object(&mut e, source_class, &[(0xa0, value)]);
        let target_class = class_dict(&mut e, 0, &[(0xb0, 0, 8)]);
        let target = dict_object(&mut e, target_class, &[]);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, source_object);
        e.mem.set_u32(source + 4, 0xa0);
        e.call(0x00dc_bd40, &args![target, 0xb0u32, source]);
        assert_eq!(e.mem.u32(target + 0x14), 1);
        let data = e.mem.u32(target + 0x10);
        assert_eq!(e.mem.u32(data), 0xb0);
        assert_eq!(e.mem.u32(data + 8), value);
        assert_eq!(e.mem.u32(value + 8), 2);
    }

    #[test]
    fn assign_ov8_copies_both_words_of_a_plain_value() {
        let mut e = fresh();
        let source_class = read_fixture(&mut e, 3, 0);
        let source_object = dict_object(&mut e, source_class, &[(0xa0, 0x3f80_0000)]);
        let source_data = e.mem.u32(source_object + 0x10);
        e.mem.set_u32(source_data + 0xc, 0x1234);
        let target_class = class_dict(&mut e, 0, &[(0xb0, 0, 3)]);
        let target = dict_object(&mut e, target_class, &[]);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, source_object);
        e.mem.set_u32(source + 4, 0xa0);
        e.call(0x00dc_bd40, &args![target, 0xb0u32, source]);
        let data = e.mem.u32(target + 0x10);
        assert_eq!(
            (e.mem.u32(data + 8), e.mem.u32(data + 0xc)),
            (0x3f80_0000, 0x1234)
        );
    }

    #[test]
    fn multi_map_remove_by_index_moves_the_successor_into_the_entry() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        for value in [0xa1, 0xa2, 0xa3] {
            chain_add(&mut e, multi, 0x11, value);
        }
        assert_eq!(e.call(0x00dc_b4b0, &args![multi, 0x11u32, 0i32]).i32(), 0);
        assert_eq!(chain_values(&e, multi, 0x11), vec![0xa2, 0xa3]);
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), 1);
    }

    #[test]
    fn multi_map_remove_by_index_of_the_last_entry_frees_the_head_when_far_away() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        for value in [0xa1, 0xa2, 0xa3] {
            chain_add(&mut e, multi, 0x11, value);
        }
        assert_eq!(e.call(0x00dc_b4b0, &args![multi, 0x11u32, 2i32]).i32(), -1);
        assert_eq!(chain_values(&e, multi, 0x11), vec![0xa2, 0xa1]);
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), 0);
    }

    #[test]
    fn multi_map_remove_by_index_of_head_or_second_entry() {
        let mut e = fresh();
        pointer_maps(&mut e);
        let multi = e.mem.alloc(0x1c);
        multi_map_at(&mut e, multi);
        chain_add(&mut e, multi, 0x11, 0xa1);
        e.call(0x00dc_b4b0, &args![multi, 0x11u32, 0i32]);
        assert_eq!(chain_values(&e, multi, 0x11), Vec::<u32>::new());
        assert_eq!(e.mem.i32(multi + multi_map::FREE_HEAD), 0);
        let other = e.mem.alloc(0x1c);
        multi_map_at(&mut e, other);
        chain_add(&mut e, other, 0x22, 0xb1);
        chain_add(&mut e, other, 0x22, 0xb2);
        assert_eq!(e.call(0x00dc_b4b0, &args![other, 0x22u32, 1i32]).i32(), -1);
        assert_eq!(chain_values(&e, other, 0x22), vec![0xb1]);
        assert_eq!(e.mem.i32(other + multi_map::FREE_HEAD), 1);
    }

    #[test]
    fn struct_array_destructor_body_unregisters_and_frees() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        pointer_maps(&mut e);
        let (world, tracker) = make_world_with_maps(&mut e);
        let array_map = tracker + tracker::ARRAY_MAP;
        e.register_double(SLOT_A, move |_, _| ret(world));
        e.register(SLOT_C, |_, _| ret(0xabcd));
        e.register(SLOT_B, |_, _| Ret::default());
        let class = object_with(&mut e, 0x40, &[(4, SLOT_A)]);
        e.mem.set_u32(class + 8, 2);
        e.mem.set_u32(0x010d_73fc + 0x18, SLOT_C);
        let this = e.mem.alloc(0x20);
        let first = object_with(&mut e, 0x20, &[(0, SLOT_B)]);
        e.mem.set_u32(first + 8, 1);
        let records = e.mem.alloc(0x10);
        e.mem.set_u32(records, 0xa0);
        e.mem.set_u32(records + 4, first);
        e.mem.set_u32(this + 0xc, records);
        e.mem.set_u32(this + 0x10, 1);
        e.mem.set_u32(this + 0x14, 1);
        e.mem.set_u32(this + 0x18, class);
        chain_add(&mut e, array_map, 0xabcd, 0x7777);
        chain_add(&mut e, array_map, 0xabcd, this);
        e.call(0x00dc_b890, &args![this]);
        assert_eq!(calls(&e, SLOT_B), vec![vec![first, 1]]);
        assert_eq!(chain_values(&e, array_map, 0xabcd), vec![0x7777]);
        assert_eq!(e.mem.u32(class + 8), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&e, BUFFER_FREE), vec![vec![allocator, records, 8]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn class_clear_releases_the_handles_and_the_pointer_blocks() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let class = class_dict(&mut e, 0, &[(0xa0, 0x1001, 1), (0xb0, 0x1002, 1)]);
        e.mem.set_u32(class + 0x10, 0x1000);
        let block = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x10);
        e.mem.set_u32(block + 4, elements);
        e.mem.set_u32(block + 8, 2);
        e.mem.set_u32(block + 0xc, 2);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, block);
        e.mem.set_u32(class + 0x1c, pointers);
        e.mem.set_u32(class + 0x20, 1);
        e.mem.set_u32(class + 0x24, 1);
        e.call(0x00dc_b990, &args![class]);
        assert_eq!(e.mem.u32(class + 0x2c), 0);
        let members = e.mem.u32(class + 0x28);
        assert_eq!(
            (e.mem.u32(members + 4), e.mem.u32(members + 0x18 + 4)),
            (0, 0)
        );
        assert_eq!(
            calls(&e, REMOVE_REFERENCE),
            vec![vec![0x1001], vec![0x1002], vec![0x1000]]
        );
        assert_eq!(calls(&e, BUFFER_FREE), vec![vec![allocator, elements, 16]]);
        assert_eq!(calls(&e, FREE), vec![vec![allocator, block, 0x10]]);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 0xc), 0x8000_0000);
        assert_eq!(e.mem.u32(class + 0x20), 0);
        assert_eq!(e.mem.u32(class + 0x10), DEFAULT_CLASS_HANDLE);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![DEFAULT_CLASS_HANDLE]]);
    }

    #[test]
    fn class_destructor_body_frees_the_arrays_and_releases_parent_and_name() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        e.register(SLOT_B, |_, _| Ret::default());
        let parent = object_with(&mut e, 0x40, &[(0, SLOT_B)]);
        e.mem.set_u32(parent + 8, 2);
        let class = class_dict(&mut e, 0, &[(0xa0, 0x1001, 1)]);
        e.mem.set_u32(class + 0x10, 0x1000);
        e.mem.set_u32(class + 0x18, parent);
        let other = e.mem.alloc(0x18 + 0x18);
        e.mem.set_u32(other + 4 + 0x18, 0x1003);
        e.mem.set_u32(class + 0x34, other + 0x18);
        e.mem.set_u32(class + 0x38, 1);
        e.mem.set_u32(class + 0x3c, 1);
        let words = e.mem.alloc(8);
        e.mem.set_u32(class + 0x1c, words);
        e.mem.set_u32(class + 0x20, 0);
        e.mem.set_u32(class + 0x24, 2);
        let (members, member_capacity) = (e.mem.u32(class + 0x28), e.mem.u32(class + 0x30));
        e.call(0x00dc_bfc0, &args![class]);
        assert_eq!(e.mem.u32(class), 0x010d_62f4);
        assert_eq!(
            calls(&e, REMOVE_REFERENCE),
            vec![
                vec![0x1001],
                vec![0x1000],
                vec![0x1003],
                vec![DEFAULT_CLASS_HANDLE]
            ]
        );
        assert_eq!(
            calls(&e, BUFFER_FREE),
            vec![
                vec![allocator, other + 0x18, 0x18],
                vec![allocator, members, member_capacity * 0x18],
                vec![allocator, words, 8]
            ]
        );
        for array in [0x1cu32, 0x28, 0x34] {
            assert_eq!(e.mem.u32(class + array), 0);
            assert_eq!(e.mem.u32(class + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(parent + 8), 1);
        assert_eq!(e.mem.u32(class + 0x18), 0);
        assert_eq!(e.mem.u32(class + 0x10), 0);
    }

    /// An empty array implementation: an element array at `+0xc` that owns nothing.
    fn empty_implementation(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        this
    }

    /// The 8-byte elements `(low, high)` of the implementation's element array.
    fn elements(e: &Engine, this: u32) -> Vec<(u32, u32)> {
        let data = e.mem.u32(this + 0xc);
        (0..e.mem.u32(this + 0x10))
            .map(|i| (e.mem.u32(data + 8 * i), e.mem.u32(data + 8 * i + 4)))
            .collect()
    }

    #[test]
    fn int_array_set_all_ov4_sign_extends_shorts() {
        let mut e = fresh();
        basics(&mut e);
        let this = empty_implementation(&mut e);
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 0xfffe);
        e.mem.set_u16(source + 2, 3);
        e.call(0x00dc_a420, &args![this, source, 2i32]);
        assert_eq!(elements(&e, this), vec![(0xffff_fffe, 0xffff_ffff), (3, 0)]);
    }

    #[test]
    fn int_array_set_all_ov5_zero_extends_shorts() {
        let mut e = fresh();
        basics(&mut e);
        let this = empty_implementation(&mut e);
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 0xfffe);
        e.mem.set_u16(source + 2, 3);
        e.call(0x00dc_a480, &args![this, source, 2i32]);
        assert_eq!(elements(&e, this), vec![(0xfffe, 0), (3, 0)]);
    }

    #[test]
    fn int_array_set_all_ov6_sign_extends_ints() {
        let mut e = fresh();
        basics(&mut e);
        let this = empty_implementation(&mut e);
        let source = e.mem.alloc(8);
        e.mem.set_i32(source, -5);
        e.mem.set_i32(source + 4, 7);
        e.call(0x00dc_a4e0, &args![this, source, 2i32]);
        assert_eq!(elements(&e, this), vec![(0xffff_fffb, 0xffff_ffff), (7, 0)]);
    }

    #[test]
    fn int_array_set_all_ov7_takes_the_low_word_of_each_64_bit_value() {
        let mut e = fresh();
        basics(&mut e);
        let this = empty_implementation(&mut e);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x8000_0000);
        e.mem.set_u32(source + 4, 0x1234);
        e.mem.set_u32(source + 8, 5);
        e.mem.set_u32(source + 12, 9);
        e.call(0x00dc_a540, &args![this, source, 2i32]);
        assert_eq!(elements(&e, this), vec![(0x8000_0000, 0xffff_ffff), (5, 0)]);
        e.call(0x00dc_a540, &args![this, source, 0i32]);
        assert_eq!(elements(&e, this), Vec::new());
    }

    #[test]
    fn real_array_constructor_makes_an_empty_array() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x00dc_a5a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_71ac);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn real_array_as_real_loads_the_float_quieted() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 1.5f32.to_bits());
        e.mem.set_u32(data + 4, 0x7f80_0001);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a5c0, &args![this, 0i32]).f64(), 1.5);
        assert!(e.call(0x00dc_a5c0, &args![this, 1i32]).f64().is_nan());
    }

    #[test]
    fn real_array_set_real_stores_the_float_quieted() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a5d0, &args![this, 1i32, 2.5f32]);
        e.call(0x00dc_a5d0, &args![this, 0i32, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(data + 4), 2.5f32.to_bits());
        assert_eq!(e.mem.u32(data), 0x7fc0_0001);
    }

    #[test]
    fn real_array_set_all_copies_the_floats() {
        let mut e = fresh();
        basics(&mut e);
        let this = empty_implementation(&mut e);
        let source = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        e.call(0x00dc_a5f0, &args![this, source, 3i32]);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(data + 4 * i)).collect::<Vec<_>>(),
            vec![0x100, 0x101, 0x102]
        );
        assert_eq!(calls(&e, MEM_COPY), vec![vec![data, source, 12]]);
    }

    #[test]
    fn cstring_array_constructor_makes_an_empty_array() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_a650, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_7244);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn cstring_array_set_string_replaces_the_copy() {
        let mut e = fresh();
        basics(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 4, 0x2222);
        e.mem.set_u32(this + 0xc, data);
        let raw = text(&mut e, "new");
        e.call(0x00dc_a670, &args![this, 1i32, raw]);
        assert_eq!(calls(&e, STRING_FREE), vec![vec![0x2222]]);
        let copy = e.mem.u32(data + 4);
        assert_ne!(copy, raw);
        assert_eq!(read_text(&e, copy), "new");
        assert_eq!(e.mem.u32(data), 0x1111);
        e.call(0x00dc_a670, &args![this, 0i32, 0u32]);
        assert_eq!(calls(&e, STRING_FREE)[1], vec![0x1111]);
        assert_eq!(e.mem.u32(data), 0);
    }

    #[test]
    fn cstring_array_destructor_frees_every_string_and_the_array() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 4, 0x2222);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 2);
        e.call(0x00dc_a6b0, &args![this]);
        assert_eq!(calls(&e, STRING_FREE), vec![vec![0x1111], vec![0x2222]]);
        assert_eq!(calls(&e, BUFFER_FREE), vec![vec![allocator, data, 8]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn byte_array_deleting_destructor_frees_the_bytes_and_optionally_the_block() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x14, 5);
        e.mem.set_u16(this + 4, 0x18);
        assert_eq!(e.call(0x00dc_a750, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, BUFFER_FREE), vec![vec![allocator, data, 5]]);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        e.call(0x00dc_a750, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![allocator, this, 0x18]]);
        assert_eq!(calls(&e, BUFFER_FREE).len(), 1);
    }

    #[test]
    fn int_array_deleting_destructor_frees_the_elements_and_optionally_the_block() {
        let mut e = fresh();
        let (_, allocator, _) = basics(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x14, 4);
        e.mem.set_u16(this + 4, 0x18);
        assert_eq!(e.call(0x00dc_a7b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, BUFFER_FREE), vec![vec![allocator, data, 32]]);
        assert_eq!(calls(&e, FREE), vec![vec![allocator, this, 0x18]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        let borrowed = e.mem.alloc(0x20);
        e.mem.set_u32(borrowed + 0x14, 0x8000_0004);
        e.call(0x00dc_a7b0, &args![borrowed, 0u32]);
        assert_eq!(calls(&e, BUFFER_FREE).len(), 1);
    }
}
