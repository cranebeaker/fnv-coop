//! `Havok SDK/hkbinarytagfilewriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit writes an `hkDataObject` graph as a binary tagfile (`hkBinaryTagfileWriter::save`):
//! the magic words, a version, then the object graph through a `Writer` (an anonymous class of
//! the source file: the Xbox PDB has no layout for it, so [`TagfileWriter`] and its parts are
//! roles read from the code). Integers are written as variable-length numbers (`fn_00dbe790`:
//! seven bits per byte, the sign in the lowest bit), strings and classes are numbered the first
//! time they are written and referred to by number afterwards, and an object's members are
//! preceded by one flag per member saying whether the member is worth writing.
//!
//! Containers of the unit's own instances (`hkMapBase` from a handle to an index, with 12-byte
//! slots whose first word is 0 when the slot is empty) are written out here; the shared layouts
//! of `hkArray` and `hkMapBase` are those of `hkarrayutil.rs` and `hkmap.rs`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkarrayutil::HkArrayBase;
use crate::units::havok_sdk::hkmap::HkMapBase;

/// The memory router (`00c85750`): `+0xc` is the allocator of array growth and `+0x10` the one
/// of the small blocks and the maps.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// the growth step next to it (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `memset` of the C runtime (`00ec61c0`, cdecl `(destination, value, bytes)`).
const MEM_SET: u32 = 0x00ec_61c0;
/// `hkOArchive` (hkoarchive.cpp): the constructor `(this, stream, flag word)`, the destructor and
/// the writers `(this, ...)`.
const ARCHIVE_CONSTRUCT: u32 = 0x00d6_0830;
const ARCHIVE_DESTRUCT: u32 = 0x00d6_0590;
const WRITE_BYTE: u32 = 0x00d6_05d0;
const WRITE_RAW: u32 = 0x00d6_07b0;
const WRITE_32: u32 = 0x00d6_0940;
const WRITE_FLOAT_32: u32 = 0x00d6_09c0;
const WRITE_ARRAY_FLOAT_32: u32 = 0x00d6_0b00;
/// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>` (hkstringmap.cpp): the
/// constructor takes one stack word it ignores.
const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
/// `hkString::strLen` (cdecl).
const STRING_LENGTH: u32 = 0x00cb_10d0;
/// `hkDataClass` (hkdataobject.cpp, Xbox PDB names): the handle is one word, the implementation.
const DATA_CLASS_CONSTRUCT: u32 = 0x00db_a650;
const DATA_CLASS_DESTRUCT: u32 = 0x00db_a660;
const DATA_CLASS_GET_NAME: u32 = 0x00db_a680;
const DATA_CLASS_GET_VERSION: u32 = 0x00db_a690;
const DATA_CLASS_GET_PARENT: u32 = 0x00db_a6a0;
const DATA_CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00db_a6f0;
/// Not named by the engine map: `(class, index, record)` fills a zeroed 0x14-byte record with
/// the member of that index (the record's `+8` word is the member's type word).
const DATA_CLASS_MEMBER_AT: u32 = 0x00db_a720;
const DATA_CLASS_GET_NUM_MEMBERS: u32 = 0x00db_a730;
/// Not named by the engine map: `(class, member selector)` gives the member's index.
const DATA_CLASS_MEMBER_INDEX: u32 = 0x00db_a740;
const DATA_CLASS_GET_ALL_MEMBER_INFO: u32 = 0x00db_aed0;
const DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO: u32 = 0x00db_af10;
/// `hkDataObject` (hkdataobject.cpp): the handle is one word, the implementation.
const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
const DATA_OBJECT_CONSTRUCT_COPY: u32 = 0x00db_a780;
const DATA_OBJECT_DESTRUCT: u32 = 0x00db_a7f0;
const DATA_OBJECT_GET_HANDLE: u32 = 0x00db_a810;
const DATA_OBJECT_GET_CLASS: u32 = 0x00db_a850;
/// `hkDataObject::operator[]` second overload.
const DATA_OBJECT_MEMBER_VALUE: u32 = 0x00db_a890;
/// The member iteration of `hkDataObject` (roles from the callers, not named by the map):
/// start, is the iterator valid, next, the member selector, the member's value.
const DATA_OBJECT_ITERATOR_START: u32 = 0x00db_a8b0;
const DATA_OBJECT_ITERATOR_IS_VALID: u32 = 0x00db_a8c0;
const DATA_OBJECT_ITERATOR_NEXT: u32 = 0x00db_a8d0;
const DATA_OBJECT_ITERATOR_MEMBER: u32 = 0x00db_a8e0;
const DATA_OBJECT_ITERATOR_VALUE: u32 = 0x00db_a8f0;
const DATA_OBJECT_IS_NULL: u32 = 0x00db_a980;
/// `hkDataObject::Value` accessors.
const OBJECT_VALUE_GET_TYPE: u32 = 0x00db_a920;
const OBJECT_VALUE_AS_INT: u32 = 0x00db_a990;
const OBJECT_VALUE_AS_VEC: u32 = 0x00db_a9a0;
const OBJECT_VALUE_AS_STRING: u32 = 0x00db_a9c0;
const OBJECT_VALUE_AS_OBJECT: u32 = 0x00db_a9d0;
const OBJECT_VALUE_AS_REAL: u32 = 0x00db_aa00;
const OBJECT_VALUE_AS_ARRAY: u32 = 0x00db_afb0;
/// `hkDataArray` (hkdataobject.cpp): destructor, `operator[]` (two overloads, `(array, out,
/// index)`), size, type, class and the member swizzle `(array, out, member)`.
const DATA_ARRAY_DESTRUCT: u32 = 0x00db_ab70;
const DATA_ARRAY_ELEMENT: u32 = 0x00db_ab90;
const DATA_ARRAY_ELEMENT_OV2: u32 = 0x00db_abb0;
const DATA_ARRAY_GET_SIZE: u32 = 0x00db_ac00;
const DATA_ARRAY_GET_TYPE: u32 = 0x00db_ac10;
const DATA_ARRAY_GET_CLASS: u32 = 0x00db_ac20;
const DATA_ARRAY_SWIZZLE_OBJECT_MEMBER: u32 = 0x00db_aea0;
/// `hkDataArray::Value` accessors.
const ARRAY_VALUE_GET_TYPE: u32 = 0x00db_ac40;
const ARRAY_VALUE_AS_VEC: u32 = 0x00db_ac50;
const ARRAY_VALUE_AS_STRING: u32 = 0x00db_ac70;
const ARRAY_VALUE_AS_REAL: u32 = 0x00db_ac80;
const ARRAY_VALUE_AS_INT: u32 = 0x00db_ac90;
const ARRAY_VALUE_AS_OBJECT: u32 = 0x00db_aca0;
const ARRAY_VALUE_AS_ARRAY: u32 = 0x00db_acd0;
/// Not named by the engine map: `(this, other)` says whether the two objects are the same
/// (called with the listener's result and the original).
const DATA_OBJECT_ACCEPTED: u32 = 0x00db_af90;
/// The `0.0f` the code compares reals with (`01011d78`).
const ZERO_FLOAT: u32 = 0x0101_1d78;
/// The string both string maps start with (`01011584`: its first byte is 0, the empty string).
const EMPTY_STRING: u32 = 0x0101_1584;
/// The default of an object record (`010d6848`, six words).
const DEFAULT_OBJECT_RECORD: u32 = 0x010d_6848;
/// The hash multiplier `-0x61c8864f` of the maps.
const HASH_MULTIPLIER: u32 = 0x9e37_79b1;
/// A slot of the handle map is three words: key low, key high, value.
const SLOT_SIZE: u32 = 12;
/// Bit 31 of a map's element count or an array's capacity word: the storage is not owned.
const NOT_OWNED: u32 = 0x8000_0000;
/// The size bits of an array's capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The size of an `hkDataClass::MemberInfo` record.
const MEMBER_INFO_SIZE: u32 = 0x14;

layout! {
    /// An `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>` (hkstringmap.cpp):
    /// 0x10 bytes; only its address is used here.
    pub struct StringMap: 0x10 {
    }

    /// `Writer::ListenerCache` (hkbinarytagfilewriter.cpp; no Xbox PDB layout, roles read from
    /// the code), 0x1c bytes: the listener called for objects not seen before, the map from an
    /// object's handle to the index in `objects`, and the cached objects (heap-allocated
    /// `hkDataObject`s).
    pub struct ListenerCache: 0x1c {
        /// The listener (an object whose slot `0xc` makes the replacement object), or 0.
        0x00 listener: Ptr,
        /// Handle (two words) to index in `objects`, 12-byte slots, 0 when empty.
        0x04 handles: Inline<HkMapBase>,
        /// The cached `hkDataObject`s (pointers).
        0x10 objects: Inline<HkArrayBase>,
    }

    /// `Writer` (hkbinarytagfilewriter.cpp; no Xbox PDB layout, roles read from the code), 0x74
    /// bytes: an `hkOArchive` (0x10 bytes), two string maps, the object counter, the map from
    /// an object's handle to its record and the records, the bytes of member flags and the
    /// listener cache.
    pub struct TagfileWriter: 0x74 {
        /// The strings written so far, by pointer, with their number.
        0x10 strings: Inline<StringMap>,
        /// The classes written so far, by name, with their number.
        0x20 classes: Inline<StringMap>,
        /// The number given to the next object written with a reference count above one.
        0x30 next_object_number: u32,
        /// Handle (two words) to the index of its record in `records`.
        0x34 object_indices: Inline<HkMapBase>,
        /// The member flags of every object, one byte per member, packed as bytes of 8 flags
        /// (`MemberWriteFlags::toBytes`).
        0x40 flag_bytes: Inline<HkArrayBase>,
        /// The object records ([`ObjectRecord`]).
        0x4c records: Inline<HkArrayBase>,
        /// The listener cache.
        0x58 cache: Inline<ListenerCache>,
    }

    /// One record of the writer's object list (0x18 bytes, roles read from the code).
    pub struct ObjectRecord: 0x18 {
        /// Set to 1 once the object has been written in full.
        0x00 written: u32,
        /// The number given to the object when it is referred to more than once (-1 before).
        0x04 number: i32,
        /// The offset of the object's flag bytes in the writer's `flag_bytes`.
        0x08 flags_offset: u32,
        /// How many times the object has been met.
        0x0c references: u32,
        /// The object's handle (two words).
        0x10 handle_low: u32,
        0x14 handle_high: u32,
    }

    /// `hkDataClass::MemberInfo` (the records of `getAllMemberInfo`, 0x14 bytes, roles read from
    /// the code).
    pub struct MemberInfo: 0x14 {
        /// The member's name (a C string).
        0x00 name: u32,
        /// The member's type word: the low four bits are the type, bit 0x20 says a tuple.
        0x08 type_word: u32,
        /// The member's class (the implementation of an `hkDataClass`), or 0.
        0x0c class: u32,
        /// The tuple size.
        0x10 tuple_size: u32,
    }
}

/// The router's allocator at byte `field` of the router (`0xc` or `0x10`).
fn router_allocator(e: &mut Engine, field: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + field)
}

/// `hkArray::reserve` as the code inlines it, for `wanted` elements: when the capacity is
/// smaller, `_reserve` is called with the larger of `wanted` and twice the capacity.
fn array_ensure_capacity(e: &mut Engine, array: Ptr<HkArrayBase>, wanted: i32, element_size: i32) {
    let allocator = router_allocator(e, 0xc);
    let capacity = (e.get(array, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// The inline construction of `hkArray<MemberInfo>(count)`: an empty array, then (for a count
/// other than 0) a block from the router's allocator (slot `0xc` is given the byte count and
/// leaves the granted one), the capacity and size becoming the granted bytes over 0x14, every
/// record zeroed. Returns the size.
fn member_info_array_new(e: &mut Engine, array: Ptr<HkArrayBase>, count: i32) -> i32 {
    e.set(array, HkArrayBase::m_data, Ptr::NULL);
    e.set(array, HkArrayBase::m_size, 0);
    e.set(array, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let allocator = router_allocator(e, 0xc);
    let mut size = 0i32;
    let mut data = 0u32;
    let mut capacity = NOT_OWNED;
    if count != 0 {
        let granted = e.with_stack(4, |e, request| {
            e.mem
                .set_i32(request.addr(), count.wrapping_mul(MEMBER_INFO_SIZE as i32));
            let block = e.vcall(allocator, 0xc, &args![request]).u32();
            (block, e.mem.i32(request.addr()))
        });
        data = granted.0;
        size = granted.1 / MEMBER_INFO_SIZE as i32;
        capacity = if size == 0 { NOT_OWNED } else { size as u32 };
    }
    e.set(array, HkArrayBase::m_data, Ptr::new(data));
    e.set(array, HkArrayBase::m_size, size);
    e.set(array, HkArrayBase::m_capacityAndFlags, capacity);
    // The elements are constructed in place: each is zeroed unless its address is null.
    for index in 0..size.max(0) as u32 {
        let record = data.wrapping_add(index.wrapping_mul(MEMBER_INFO_SIZE));
        if record != 0 {
            for word in 0..5 {
                e.mem.set_u32(record + 4 * word, 0);
            }
        }
    }
    size
}

/// The inline destruction of an `hkArray<MemberInfo>`: size 0, the storage returned to the
/// router's allocator (slot `0x10`, with its byte count) unless the capacity word says it is not
/// owned.
fn member_info_array_free(e: &mut Engine, array: Ptr<HkArrayBase>) {
    let allocator = router_allocator(e, 0xc);
    let capacity = e.get(array, HkArrayBase::m_capacityAndFlags);
    e.set(array, HkArrayBase::m_size, 0);
    if capacity as i32 >= 0 {
        let data = e.get(array, HkArrayBase::m_data);
        e.vcall(
            allocator,
            0x10,
            &args![
                data,
                (capacity & CAPACITY_MASK).wrapping_mul(MEMBER_INFO_SIZE)
            ],
        );
    }
}

/// The bytes of the member flags in `count` flags: `count` rounded up to 8, over 8 (the exe's
/// signed `(n + 7 & ~7) / 8`).
fn flag_byte_count(count: i32) -> i32 {
    let rounded = count.wrapping_add(7) & !7;
    rounded.wrapping_add((rounded >> 31) & 7) >> 3
}

/// A `Value`'s accessors: `hkDataObject::Value` and `hkDataArray::Value` have the same six
/// with different addresses.
struct ValueAccessors {
    get_type: u32,
    as_int: u32,
    as_real: u32,
    as_object: u32,
    as_string: u32,
    as_array: u32,
}

const OBJECT_VALUE: ValueAccessors = ValueAccessors {
    get_type: OBJECT_VALUE_GET_TYPE,
    as_int: OBJECT_VALUE_AS_INT,
    as_real: OBJECT_VALUE_AS_REAL,
    as_object: OBJECT_VALUE_AS_OBJECT,
    as_string: OBJECT_VALUE_AS_STRING,
    as_array: OBJECT_VALUE_AS_ARRAY,
};

const ARRAY_VALUE: ValueAccessors = ValueAccessors {
    get_type: ARRAY_VALUE_GET_TYPE,
    as_int: ARRAY_VALUE_AS_INT,
    as_real: ARRAY_VALUE_AS_REAL,
    as_object: ARRAY_VALUE_AS_OBJECT,
    as_string: ARRAY_VALUE_AS_STRING,
    as_array: ARRAY_VALUE_AS_ARRAY,
};

/// The size of the array a `Value` holds, with the temporary array destroyed.
fn value_array_size(e: &mut Engine, accessors: &ValueAccessors, value: Ptr) -> u32 {
    e.with_stack(4, |e, array| {
        let array_ptr = e.call(accessors.as_array, &args![value, array]).u32();
        let size = e.call(DATA_ARRAY_GET_SIZE, &args![array_ptr]).u32();
        e.call(DATA_ARRAY_DESTRUCT, &args![array]);
        size
    })
}

/// `Writer::worthWriting` for either kind of `Value`: whether the value is worth writing
/// (nonzero integer, nonzero real, a vector or a quaternion or a matrix always, a non-null
/// object, a non-null string, a non-empty array; an array-typed value (type 0x10 or above)
/// returns its size). The type is asked twice, as the exe does.
fn worth_writing(e: &mut Engine, accessors: &ValueAccessors, value: Ptr) -> u32 {
    let first_type = e.call(accessors.get_type, &args![value]).i32();
    if first_type >= 0x10 {
        return value_array_size(e, accessors, value);
    }
    let second_type = e.call(accessors.get_type, &args![value]).u32();
    if second_type > 0x10 {
        return 0;
    }
    match second_type {
        1 | 2 => u32::from(e.call(accessors.as_int, &args![value]).u32() != 0),
        3 => {
            let real = e.call(accessors.as_real, &args![value]).f32();
            let zero: f32 = e.global(ZERO_FLOAT);
            u32::from(real != zero)
        }
        4 | 6 | 7 => 1,
        8 | 9 => e.with_stack(4, |e, object| {
            let object_ptr = e.call(accessors.as_object, &args![value, object]).u32();
            let null = e.call(DATA_OBJECT_IS_NULL, &args![object_ptr]).u32();
            e.call(DATA_OBJECT_DESTRUCT, &args![object]);
            u32::from(null == 0)
        }),
        10 => u32::from(e.call(accessors.as_string, &args![value]).u32() != 0),
        // A type of exactly 0x10 is taken by the first test; the case is reached only when the
        // second `getType` answers differently from the first.
        0x10 => u32::from(value_array_size(e, accessors, value) != 0),
        _ => 0,
    }
}

// Translated from 00dbe790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map (the writer's variable-length integer). Writes the 64-bit
/// integer (`low`, `high`) to the archive `this`: a negative number (`high` below zero) becomes
/// `(-n * 2) | 1`, any other `n * 2`; then seven bits at a time, lowest first, every byte but
/// the last with bit 7 set, each through `hkOArchive`'s byte writer (`00d605d0`). The last byte
/// is passed as the word `low` with its low byte replaced (the writer reads only that byte).
pub fn fn_00dbe790(e: &mut Engine, this: Ptr, low: u32, high: i32) {
    let mut value = (u64::from(high as u32) << 32) | u64::from(low);
    if high < 0 {
        value = value.wrapping_neg();
        value = (value << 1) | 1;
    } else {
        value <<= 1;
    }
    let mut byte = (value as u8) & 0x7f;
    value >>= 7;
    while value != 0 {
        e.call(WRITE_BYTE, &args![this, u32::from(byte | 0x80)]);
        byte = (value as u8) & 0x7f;
        value >>= 7;
    }
    e.call(
        WRITE_BYTE,
        &args![this, (low & 0xffff_ff00) | u32::from(byte)],
    );
}

// Translated from 00dbe810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `insert` of the writer's handle map (key: two words, value: one
/// word, empty slots have key word 0): first grows the table (`00dbec60`, to `2 * m_hashMod + 2`
/// slots) when twice the element count exceeds `m_hashMod`; then stores the pair at the first
/// free slot of the probe sequence or over the slot with an equal key, the element count rising
/// for a new key.
pub fn fn_00dbe810(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    key_low: u32,
    key_high: u32,
    value: u32,
) {
    let count = (e.get(this, HkMapBase::m_numElems) & 0x7fff_ffff) as i32;
    let hash_mod = e.get(this, HkMapBase::m_hashMod);
    if count.wrapping_add(count) > hash_mod {
        let slots = hash_mod.wrapping_add(hash_mod).wrapping_add(2);
        fn_00dbec60(e, this, allocator, slots);
    }
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;
    let table = e.get(this, HkMapBase::m_elem);
    let mut slot = (key_low >> 4).wrapping_mul(HASH_MULTIPLIER) & mask;
    let mut added = 1u32;
    loop {
        let address = table.wrapping_add(slot.wrapping_mul(SLOT_SIZE));
        if e.mem.u32(address) == 0 {
            break;
        }
        if e.mem.u32(address) == key_low && e.mem.u32(address + 4) == key_high {
            added = 0;
            break;
        }
        slot = slot.wrapping_add(1) & mask;
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word.wrapping_add(added));
    let address = table.wrapping_add(slot.wrapping_mul(SLOT_SIZE));
    e.mem.set_u32(address, key_low);
    e.mem.set_u32(address + 4, key_high);
    e.mem.set_u32(address + 8, value);
}

// Translated from 00dbe8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<hkDataObject::Handle,int,Writer::HandleOps>::getWithDefault` (engine map name):
/// the value of the key (two words) or `default` when the key is not in the map (also when
/// `m_hashMod` is not positive).
pub fn hk_map_base_get_with_default(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key_low: u32,
    key_high: u32,
    default: u32,
) -> u32 {
    let mask = e.get(this, HkMapBase::m_hashMod);
    if mask > 0 {
        let table = e.get(this, HkMapBase::m_elem);
        let mask = mask as u32;
        let mut slot = (key_low >> 4).wrapping_mul(HASH_MULTIPLIER) & mask;
        loop {
            let address = table.wrapping_add(slot.wrapping_mul(SLOT_SIZE));
            if e.mem.u32(address) == 0 {
                break;
            }
            if e.mem.u32(address) == key_low && e.mem.u32(address + 4) == key_high {
                return e.mem.u32(address + 8);
            }
            slot = slot.wrapping_add(1) & mask;
        }
    }
    default
}

// Translated from 00dbe920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Empties the map: the first word of every slot (`m_hashMod + 1`
/// of them) becomes 0, and the element count is cleared except the not-owned flag.
pub fn fn_00dbe920(e: &mut Engine, this: Ptr<HkMapBase>) {
    let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let mut offset = 0u32;
    for _ in 0..slots.max(0) {
        let table = e.get(this, HkMapBase::m_elem);
        e.mem.set_u32(table.wrapping_add(offset), 0);
        offset = offset.wrapping_add(SLOT_SIZE);
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word & NOT_OWNED);
}

// Translated from 00dbe950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Empties the map (`00dbe920`) and, unless its slots are not
/// owned, returns them to `allocator` (vtable slot 8, the slots and their size in bytes).
pub fn fn_00dbe950(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr) {
    fn_00dbe920(e, this);
    if e.get(this, HkMapBase::m_numElems) & NOT_OWNED == 0 {
        let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1) as u32;
        let table = e.get(this, HkMapBase::m_elem);
        e.vcall(
            allocator.addr(),
            8,
            &args![table, slots.wrapping_mul(SLOT_SIZE)],
        );
    }
}

// Translated from 00dbe980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::worthWriting<hkDataObject::Value>` (engine map name): whether the member value
/// at `value` is worth writing, by its type: a nonzero integer (types 1 and 2), a nonzero real
/// (3, a NaN counts), always for 4, 6 and 7 (vectors), a non-null object (8 and 9), a non-null
/// string (10); a value of an array type (0x10 and above) answers its size. `_this` is the
/// writer, which the code never reads.
pub fn writer_worth_writing_data_object_value(e: &mut Engine, _this: Ptr, value: Ptr) -> u32 {
    worth_writing(e, &OBJECT_VALUE, value)
}

// Translated from 00dbeab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::worthWriting<hkDataArray::Value>` (engine map name): the same decisions as
/// `00dbe980` for an element of an array (`hkDataArray::Value` accessors). `_this` is the
/// writer, which the code never reads.
pub fn writer_worth_writing_data_array_value(e: &mut Engine, _this: Ptr, value: Ptr) -> u32 {
    worth_writing(e, &ARRAY_VALUE, value)
}

// Translated from 00dbebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::ListenerCache::getCachedDataObject` (engine map name): builds in `out` the object to
/// write for `object`: when there is a listener and the object is not null and its handle is in
/// the cache, a copy of the cached object (`hkDataObject` constructor from its implementation,
/// which takes a reference); otherwise a copy of `object`. Returns `out`.
pub fn writer_listener_cache_get_cached_data_object(
    e: &mut Engine,
    this: Ptr<ListenerCache>,
    out: Ptr,
    object: Ptr,
) -> Ptr {
    if e.get(this, ListenerCache::listener).addr() != 0
        && e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() == 0
    {
        let found = e.with_stack(8, |e, handle| {
            e.call(DATA_OBJECT_GET_HANDLE, &args![object, handle]);
            let (low, high) = (e.mem.u32(handle.addr()), e.mem.u32(handle.addr() + 4));
            hk_map_base_get_with_default(e, this.at(ListenerCache::handles), low, high, 0xffff_ffff)
        });
        if found != 0xffff_ffff {
            let objects = this.at(ListenerCache::objects);
            let data = e.get(objects, HkArrayBase::m_data).addr();
            let cached = e.mem.u32(data.wrapping_add(found.wrapping_mul(4)));
            let implementation = e.mem.u32(cached);
            e.call(DATA_OBJECT_CONSTRUCT, &args![out, implementation]);
            return out;
        }
    }
    e.call(DATA_OBJECT_CONSTRUCT_COPY, &args![out, object]);
    out
}

// Translated from 00dbec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `resizeTable` of the writer's handle map: gives the map
/// `max(slots, 8)` slots from `allocator` (vtable slot 4, the size in bytes), marks them empty
/// (first word 0), clears the element count (and the not-owned flag), inserts every element of
/// the old table again (`00dbe810`) and, if the old slots were owned, returns them to the
/// allocator (vtable slot 8, the slots and their size).
pub fn fn_00dbec60(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, slots: i32) {
    let slots = if slots > 8 { slots } else { 8 };
    let flags = e.get(this, HkMapBase::m_numElems) & NOT_OWNED;
    let old_table = e.get(this, HkMapBase::m_elem);
    let old_slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let bytes = (slots as u32).wrapping_mul(SLOT_SIZE);
    let table = e.vcall(allocator.addr(), 4, &args![bytes]).u32();
    e.set(this, HkMapBase::m_elem, table);
    for index in 0..slots as u32 {
        let table = e.get(this, HkMapBase::m_elem);
        e.mem
            .set_u32(table.wrapping_add(index.wrapping_mul(SLOT_SIZE)), 0);
    }
    e.set(this, HkMapBase::m_numElems, 0);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1));
    for index in 0..old_slots.max(0) as u32 {
        let slot = old_table.wrapping_add(index.wrapping_mul(SLOT_SIZE));
        if e.mem.u32(slot) != 0 {
            let (low, high, value) = (e.mem.u32(slot), e.mem.u32(slot + 4), e.mem.u32(slot + 8));
            fn_00dbe810(e, this, allocator, low, high, value);
        }
    }
    if flags == 0 {
        e.vcall(
            allocator.addr(),
            8,
            &args![old_table, (old_slots as u32).wrapping_mul(SLOT_SIZE)],
        );
    }
}

// Translated from 00dbed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::writeString` (engine map name): writes `string` to the archive as a number. A null
/// string is the number -1; a string written before is its number negated; a new string is
/// written as its length followed by its bytes (`hkOArchive::writeRaw`) and entered in the
/// string map with the number `m_numElems + 1`.
pub fn writer_write_string(e: &mut Engine, this: Ptr<TagfileWriter>, string: u32) {
    if string == 0 {
        fn_00dbe790(e, this.cast(), 0xffff_ffff, -1);
        return;
    }
    let strings = this.at(TagfileWriter::strings).addr();
    let number = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![strings, string, 0xffff_ffffu32],
        )
        .i32();
    if number == -1 {
        let length = e.call(STRING_LENGTH, &args![string]).i32();
        fn_00dbe790(e, this.cast(), length as u32, length >> 31);
        e.call(WRITE_RAW, &args![this, string, length]);
        let count = e.mem.u32(strings + 4);
        e.call(
            STRING_MAP_INSERT,
            &args![strings, string, count.wrapping_add(1)],
        );
    } else {
        let negated = number.wrapping_neg();
        fn_00dbe790(e, this.cast(), negated as u32, negated >> 31);
    }
}

// Translated from 00dbedb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `reserve` of the writer's handle map: the smallest power of two
/// that is at least 8 and at least twice `count`, handed to `resizeTable` (`00dbec60`).
pub fn fn_00dbedb0(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, count: i32) {
    let wanted = count.wrapping_add(count);
    let mut slots = 8i32;
    if wanted > slots {
        loop {
            slots = slots.wrapping_add(slots);
            if slots >= wanted {
                break;
            }
        }
    }
    fn_00dbec60(e, this, allocator, slots);
}

// Translated from 00dbedd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::ListenerCache::callListenerAndCache` (engine map name): builds in `out` the object to
/// write for `object`. With a listener and a non-null object: a copy of the cached object when
/// the handle is cached; otherwise the listener (slot `0xc`, given a temporary object and
/// `object`) makes a replacement and `00dbaf90` says whether it is accepted; an accepted one is
/// put in the cache (the handle mapped to the index, a heap copy appended to `objects`) and
/// `out` becomes a copy of it; a rejected one, no listener or a null object gives a copy of
/// `object`. Returns `out`.
pub fn writer_listener_cache_call_listener_and_cache(
    e: &mut Engine,
    this: Ptr<ListenerCache>,
    out: Ptr,
    object: Ptr,
) -> Ptr {
    let listener = e.get(this, ListenerCache::listener).addr();
    if listener != 0 && e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() == 0 {
        let accepted = e.with_stack(0xc, |e, block| {
            let handle = block.addr();
            let replacement = block.addr() + 8;
            e.call(DATA_OBJECT_GET_HANDLE, &args![object, handle]);
            let (low, high) = (e.mem.u32(handle), e.mem.u32(handle + 4));
            let handles = this.at(ListenerCache::handles);
            let found = hk_map_base_get_with_default(e, handles, low, high, 0xffff_ffff);
            if found != 0xffff_ffff {
                let objects = this.at(ListenerCache::objects);
                let data = e.get(objects, HkArrayBase::m_data).addr();
                let cached = e.mem.u32(data.wrapping_add(found.wrapping_mul(4)));
                let implementation = e.mem.u32(cached);
                e.call(DATA_OBJECT_CONSTRUCT, &args![out, implementation]);
                return true;
            }
            e.vcall(listener, 0xc, &args![replacement, object]);
            if e.call(DATA_OBJECT_ACCEPTED, &args![replacement, object])
                .u32()
                & 0xff
                != 0
            {
                let objects = this.at(ListenerCache::objects);
                let count = e.get(objects, HkArrayBase::m_size);
                let allocator = router_allocator(e, 0x10);
                fn_00dbe810(e, handles, Ptr::new(allocator), low, high, count as u32);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let heap = e.mem.u32(router + 0x10);
                let copy = e.vcall(heap, 4, &args![4u32]).u32();
                let element = if copy != 0 {
                    e.call(DATA_OBJECT_CONSTRUCT_COPY, &args![copy, replacement])
                        .u32()
                } else {
                    0
                };
                let grow_allocator = router_allocator(e, 0xc);
                let size = e.get(objects, HkArrayBase::m_size);
                let capacity = e.get(objects, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
                if size as u32 == capacity {
                    e.call(ARRAY_GROW, &args![grow_allocator, objects, 4u32]);
                }
                let size = e.get(objects, HkArrayBase::m_size);
                let data = e.get(objects, HkArrayBase::m_data).addr();
                e.set(objects, HkArrayBase::m_size, size.wrapping_add(1));
                e.mem
                    .set_u32(data.wrapping_add((size as u32).wrapping_mul(4)), element);
                e.call(DATA_OBJECT_CONSTRUCT_COPY, &args![out, replacement]);
                e.call(DATA_OBJECT_DESTRUCT, &args![replacement]);
                return true;
            }
            e.call(DATA_OBJECT_DESTRUCT, &args![replacement]);
            false
        });
        if accepted {
            return out;
        }
    }
    e.call(DATA_OBJECT_CONSTRUCT_COPY, &args![out, object]);
    out
}

// Translated from 00dbef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::MemberWriteFlags::MemberWriteFlags` (engine map name): an `hkArray<char>` of
/// `count` flags, all zero. The array starts empty (not owned), is made `count` rounded up to a
/// multiple of 8 bytes long and zero-filled, then its size becomes `count`. Returns `this`.
pub fn writer_member_write_flags_constructor(
    e: &mut Engine,
    this: Ptr<HkArrayBase>,
    count: i32,
) -> Ptr<HkArrayBase> {
    e.set(this, HkArrayBase::m_data, Ptr::NULL);
    e.set(this, HkArrayBase::m_size, 0);
    e.set(this, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let rounded = count.wrapping_add(7) & !7;
    array_ensure_capacity(e, this, rounded, 1);
    let size = e.get(this, HkArrayBase::m_size);
    let data = e.get(this, HkArrayBase::m_data).addr();
    let padding = rounded.wrapping_sub(size);
    if padding > 0 {
        e.call(
            MEM_SET,
            &args![data.wrapping_add(size as u32), 0u32, padding],
        );
    }
    e.set(this, HkArrayBase::m_size, rounded);
    array_ensure_capacity(e, this, count, 1);
    e.set(this, HkArrayBase::m_size, count);
    this
}

// Translated from 00dbefc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::MemberWriteFlags::fromBytes` (engine map name): fills the flags from `count` bytes
/// at `bytes`, eight flags per byte, lowest bit first, each flag 0 or 1. The array is first made
/// as long as its size rounded up to 8 (zero-filled); afterwards its size is restored.
pub fn writer_member_write_flags_from_bytes(
    e: &mut Engine,
    this: Ptr<HkArrayBase>,
    bytes: u32,
    count: i32,
) {
    let size = e.get(this, HkArrayBase::m_size);
    let rounded = size.wrapping_add(7) & !7;
    array_ensure_capacity(e, this, rounded, 1);
    let current = e.get(this, HkArrayBase::m_size);
    let data = e.get(this, HkArrayBase::m_data).addr();
    let padding = rounded.wrapping_sub(current);
    if padding > 0 {
        e.call(
            MEM_SET,
            &args![data.wrapping_add(current as u32), 0u32, padding],
        );
    }
    e.set(this, HkArrayBase::m_size, rounded);
    let mut flag_offset = 0u32;
    for index in 0..count.max(0) as u32 {
        let byte = e.mem.u8(bytes.wrapping_add(index));
        let data = e.get(this, HkArrayBase::m_data).addr();
        for bit in 0..8u32 {
            e.mem.set_u8(
                data.wrapping_add(flag_offset).wrapping_add(bit),
                (byte >> bit) & 1,
            );
        }
        flag_offset = flag_offset.wrapping_add(8);
    }
    array_ensure_capacity(e, this, size, 1);
    e.set(this, HkArrayBase::m_size, size);
}

// Translated from 00dbf0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::MemberWriteFlags::toBytes` (engine map name): packs the flags into bytes at `out`,
/// eight flags per byte with the first in bit 0 (each flag byte is combined as it stands: bytes
/// above 1 spill into the neighbours' bits). The array is first made as long as its size
/// rounded up to 8 (zero-filled); afterwards its size is restored.
pub fn writer_member_write_flags_to_bytes(e: &mut Engine, this: Ptr<HkArrayBase>, out: u32) {
    let size = e.get(this, HkArrayBase::m_size);
    let rounded = size.wrapping_add(7) & !7;
    array_ensure_capacity(e, this, rounded, 1);
    let current = e.get(this, HkArrayBase::m_size);
    let data = e.get(this, HkArrayBase::m_data).addr();
    let padding = rounded.wrapping_sub(current);
    if padding > 0 {
        e.call(
            MEM_SET,
            &args![data.wrapping_add(current as u32), 0u32, padding],
        );
    }
    e.set(this, HkArrayBase::m_size, rounded);
    let mut flag_offset = 0u32;
    let mut out = out;
    if rounded > 0 {
        loop {
            let data = e
                .get(this, HkArrayBase::m_data)
                .addr()
                .wrapping_add(flag_offset);
            // The eight flag bytes combined from the last to the first, in 8-bit arithmetic.
            let mut packed = e.mem.u8(data.wrapping_add(7));
            for bit in (0..7u32).rev() {
                packed = packed.wrapping_add(packed) | e.mem.u8(data.wrapping_add(bit));
            }
            e.mem.set_u8(out, packed);
            out = out.wrapping_add(1);
            flag_offset = flag_offset.wrapping_add(8);
            if (flag_offset as i32) >= e.get(this, HkArrayBase::m_size) {
                break;
            }
        }
    }
    array_ensure_capacity(e, this, size, 1);
    e.set(this, HkArrayBase::m_size, size);
}

// Translated from 00dbf1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::writeClass` (engine map name): writes the class whose handle is at `class` and
/// returns its number. A null class is number 0; a class written before is the number in the
/// class map; a new one first writes its parent class, then is entered in the map with the
/// number `this + 0x24` (the writer's own counter field), and is written as the tag 2, its name
/// (`writeString`), version, the parent's number, the count of declared members and, for each,
/// its name, type word, tuple size (when the type word has bit 0x20) and class name (for types 8
/// and 9: the class's name, or -1), followed by the member classes written in turn. Returns the
/// number the class was entered with.
pub fn writer_write_class(e: &mut Engine, this: Ptr<TagfileWriter>, class: Ptr) -> i32 {
    if e.mem.u32(class.addr()) == 0 {
        return 0;
    }
    let classes = this.at(TagfileWriter::classes).addr();
    let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
    let known = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![classes, name, 0xffff_ffffu32],
        )
        .i32();
    if known >= 0 {
        return known;
    }
    let parent_number = e.with_stack(4, |e, parent| {
        let parent_class = e.call(DATA_CLASS_GET_PARENT, &args![class, parent]);
        let number = writer_write_class(e, this, parent_class.ptr());
        e.call(DATA_CLASS_DESTRUCT, &args![parent]);
        number
    });
    let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
    let known = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![classes, name, 0xffff_ffffu32],
        )
        .i32();
    if known != -1 {
        return known;
    }
    let number = e.mem.u32(this.addr() + 0x24);
    let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
    e.call(STRING_MAP_INSERT, &args![classes, name, number]);
    let member_count = e
        .call(DATA_CLASS_GET_NUM_DECLARED_MEMBERS, &args![class])
        .i32();
    e.with_stack(0xc, |e, members| {
        let members: Ptr<HkArrayBase> = members.cast();
        member_info_array_new(e, members, member_count);
        e.call(
            DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO,
            &args![class, members],
        );
        fn_00dbe790(e, this.cast(), 2, 0);
        let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
        writer_write_string(e, this, name);
        let version = e.call(DATA_CLASS_GET_VERSION, &args![class]).i32();
        fn_00dbe790(e, this.cast(), version as u32, version >> 31);
        fn_00dbe790(e, this.cast(), parent_number as u32, parent_number >> 31);
        fn_00dbe790(e, this.cast(), member_count as u32, member_count >> 31);
        if member_count > 0 {
            let mut offset = 0u32;
            for _ in 0..member_count {
                let data = e.get(members, HkArrayBase::m_data).addr();
                let member: Ptr<MemberInfo> = Ptr::new(data.wrapping_add(offset));
                let member_name = e.get(member, MemberInfo::name);
                writer_write_string(e, this, member_name);
                let type_word = e.get(member, MemberInfo::type_word);
                fn_00dbe790(e, this.cast(), type_word, (type_word as i32) >> 31);
                if type_word & 0x20 != 0 {
                    let tuple_size = e.get(member, MemberInfo::tuple_size);
                    fn_00dbe790(e, this.cast(), tuple_size, (tuple_size as i32) >> 31);
                }
                let kind = type_word & 0xf;
                if kind == 9 || kind == 8 {
                    let member_class = e.get(member, MemberInfo::class);
                    if member_class != 0 {
                        e.with_stack(4, |e, handle| {
                            e.call(DATA_CLASS_CONSTRUCT, &args![handle, member_class]);
                            let class_name = e.call(DATA_CLASS_GET_NAME, &args![handle]).u32();
                            writer_write_string(e, this, class_name);
                            e.call(DATA_CLASS_DESTRUCT, &args![handle]);
                        });
                    } else {
                        fn_00dbe790(e, this.cast(), 0xffff_ffff, -1);
                    }
                }
                offset = offset.wrapping_add(MEMBER_INFO_SIZE);
            }
            let mut offset = 0u32;
            for _ in 0..member_count {
                let data = e.get(members, HkArrayBase::m_data).addr();
                let member_class = e.mem.u32(data.wrapping_add(offset).wrapping_add(0xc));
                if member_class != 0 {
                    e.with_stack(4, |e, handle| {
                        e.call(DATA_CLASS_CONSTRUCT, &args![handle, member_class]);
                        writer_write_class(e, this, handle);
                        e.call(DATA_CLASS_DESTRUCT, &args![handle]);
                    });
                }
                offset = offset.wrapping_add(MEMBER_INFO_SIZE);
            }
        }
        member_info_array_free(e, members);
    });
    number as i32
}

/// The address of record `index` of the writer's object records.
fn record_at(e: &Engine, this: Ptr<TagfileWriter>, index: u32) -> Ptr<ObjectRecord> {
    let records = this.at(TagfileWriter::records);
    let data = e.get(records, HkArrayBase::m_data).addr();
    Ptr::new(data.wrapping_add(index.wrapping_mul(0x18)))
}

// Translated from 00dbf450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. The writer's first pass over `object`: asks the listener cache
/// for the object to write; a null object does nothing. An object whose handle already has a
/// record only has its reference count (`+0xc`) raised. A new one gets a record (when `flag` is
/// 0, as every caller passes it): its handle is mapped to the record's index, a record made of
/// the six words at `010d6848` is appended to the record array and holds the handle. Then the
/// class is written (`writeClass`), a flag is set for every member worth writing
/// (`worthWriting`), and the objects the members hold are scanned in turn: an object member,
/// every element of an object array (types 0x18 and 0x28), and for struct arrays (types 0x19
/// and 0x29) every object member of the struct. With `flag` 0 the packed flags are appended to
/// the writer's flag bytes and the record's `+8` is the offset they went to.
pub fn fn_00dbf450(e: &mut Engine, this: Ptr<TagfileWriter>, object: Ptr, flag: u32) {
    // The frame the exe keeps these temporaries in: a handle for the cached object, class
    // handles, hkDataObject/hkDataArray temporaries, `Value`s, the handle words, a member record
    // and the flag array.
    const OBJECT: u32 = 0x00;
    const CLASS: u32 = 0x04;
    const ARRAY_CLASS: u32 = 0x08;
    const OBJECT_TEMP: u32 = 0x0c;
    const ARRAY_TEMP: u32 = 0x10;
    const ARRAY_TEMP_2: u32 = 0x14;
    const VALUE: u32 = 0x18;
    const HANDLE: u32 = 0x20;
    const HANDLE_2: u32 = 0x28;
    const ELEMENT: u32 = 0x30;
    const ELEMENT_2: u32 = 0x38;
    const RECORD: u32 = 0x40;
    const FLAGS: u32 = 0x58;
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let cached = frame + OBJECT;
        let cache = this.at(TagfileWriter::cache);
        writer_listener_cache_call_listener_and_cache(e, cache, Ptr::new(cached), object);
        if e.call(DATA_OBJECT_IS_NULL, &args![cached]).u32() != 0 {
            e.call(DATA_OBJECT_DESTRUCT, &args![cached]);
            return;
        }
        let handle = e
            .call(DATA_OBJECT_GET_HANDLE, &args![cached, frame + HANDLE])
            .u32();
        let (low, high) = (e.mem.u32(handle), e.mem.u32(handle + 4));
        let object_indices = this.at(TagfileWriter::object_indices);
        let record_index = hk_map_base_get_with_default(e, object_indices, low, high, 0xffff_ffff);
        if record_index != 0xffff_ffff {
            let record = record_at(e, this, record_index);
            let references = e.get(record, ObjectRecord::references);
            e.set(record, ObjectRecord::references, references.wrapping_add(1));
            e.call(DATA_OBJECT_DESTRUCT, &args![cached]);
            return;
        }
        let mut record_index = 0xffff_ffffu32;
        if flag == 0 {
            record_index = e.get(object_indices, HkMapBase::m_numElems) & 0x7fff_ffff;
            let handle = e
                .call(DATA_OBJECT_GET_HANDLE, &args![cached, frame + HANDLE_2])
                .u32();
            let (low, high) = (e.mem.u32(handle), e.mem.u32(handle + 4));
            let allocator = router_allocator(e, 0x10);
            fn_00dbe810(
                e,
                object_indices,
                Ptr::new(allocator),
                low,
                high,
                record_index,
            );
            let records = this.at(TagfileWriter::records);
            let grow_allocator = router_allocator(e, 0xc);
            let size = e.get(records, HkArrayBase::m_size);
            let capacity = e.get(records, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
            if size as u32 == capacity {
                e.call(ARRAY_GROW, &args![grow_allocator, records, 0x18u32]);
            }
            let size = e.get(records, HkArrayBase::m_size);
            let data = e.get(records, HkArrayBase::m_data).addr();
            let new_record = data.wrapping_add((size as u32).wrapping_mul(0x18));
            e.set(records, HkArrayBase::m_size, size.wrapping_add(1));
            for word in 0..6 {
                let value: u32 = e.global(DEFAULT_OBJECT_RECORD + 4 * word);
                e.mem.set_u32(new_record + 4 * word, value);
            }
            let size = e.get(records, HkArrayBase::m_size);
            let data = e.get(records, HkArrayBase::m_data).addr();
            let last: Ptr<ObjectRecord> = Ptr::new(
                data.wrapping_add((size as u32).wrapping_mul(0x18))
                    .wrapping_sub(0x18),
            );
            let handle = e
                .call(DATA_OBJECT_GET_HANDLE, &args![cached, frame + HANDLE])
                .u32();
            let (low, high) = (e.mem.u32(handle), e.mem.u32(handle + 4));
            e.set(last, ObjectRecord::handle_low, low);
            e.set(last, ObjectRecord::handle_high, high);
        }
        let class = frame + CLASS;
        e.call(DATA_OBJECT_GET_CLASS, &args![cached, class]);
        writer_write_class(e, this, Ptr::new(class));
        let member_count = e.call(DATA_CLASS_GET_NUM_MEMBERS, &args![class]).i32();
        let flags: Ptr<HkArrayBase> = Ptr::new(frame + FLAGS);
        writer_member_write_flags_constructor(e, flags, member_count);
        let value = frame + VALUE;
        let mut iterator = e.call(DATA_OBJECT_ITERATOR_START, &args![cached]).u32();
        while e
            .call(DATA_OBJECT_ITERATOR_IS_VALID, &args![cached, iterator])
            .u32()
            != 0
        {
            e.call(DATA_OBJECT_ITERATOR_VALUE, &args![cached, value, iterator]);
            if writer_worth_writing_data_object_value(e, this.cast(), Ptr::new(value)) != 0 {
                let member = e
                    .call(DATA_OBJECT_ITERATOR_MEMBER, &args![cached, iterator])
                    .u32();
                let member_index = e.call(DATA_CLASS_MEMBER_INDEX, &args![class, member]).u32();
                let flag_data = e.get(flags, HkArrayBase::m_data).addr();
                e.mem.set_u8(flag_data.wrapping_add(member_index), 1);
                let kind = e.call(OBJECT_VALUE_GET_TYPE, &args![value]).u32() & 0xf;
                if kind == 8 || kind == 9 {
                    match e
                        .call(OBJECT_VALUE_GET_TYPE, &args![value])
                        .u32()
                        .wrapping_sub(8)
                    {
                        0 | 1 => {
                            let temp = frame + OBJECT_TEMP;
                            let scanned = e.call(OBJECT_VALUE_AS_OBJECT, &args![value, temp]);
                            fn_00dbf450(e, this, scanned.ptr(), 0);
                            e.call(DATA_OBJECT_DESTRUCT, &args![temp]);
                        }
                        0x10 | 0x20 => {
                            let temp = frame + ARRAY_TEMP;
                            let array = e.call(OBJECT_VALUE_AS_ARRAY, &args![value, temp]).u32();
                            let size = e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32();
                            e.call(DATA_ARRAY_DESTRUCT, &args![temp]);
                            for index in 0..size.max(0) {
                                let array = e
                                    .call(
                                        OBJECT_VALUE_AS_ARRAY,
                                        &args![value, frame + ARRAY_TEMP_2],
                                    )
                                    .u32();
                                let element = e
                                    .call(DATA_ARRAY_ELEMENT, &args![array, frame + ELEMENT, index])
                                    .u32();
                                let scanned = e.call(
                                    ARRAY_VALUE_AS_OBJECT,
                                    &args![element, frame + OBJECT_TEMP],
                                );
                                fn_00dbf450(e, this, scanned.ptr(), 0);
                                e.call(DATA_OBJECT_DESTRUCT, &args![frame + OBJECT_TEMP]);
                                e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP_2]);
                            }
                        }
                        0x11 | 0x21 => {
                            let array_class = frame + ARRAY_CLASS;
                            let array = e
                                .call(OBJECT_VALUE_AS_ARRAY, &args![value, frame + ARRAY_TEMP])
                                .u32();
                            e.call(DATA_ARRAY_GET_CLASS, &args![array, array_class]);
                            e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP]);
                            writer_write_class(e, this, Ptr::new(array_class));
                            let mut member_number = 0i32;
                            let total = e
                                .call(DATA_CLASS_GET_NUM_MEMBERS, &args![array_class])
                                .i32();
                            if total > 0 {
                                loop {
                                    let record = frame + RECORD;
                                    for word in 0..5 {
                                        e.mem.set_u32(record + 4 * word, 0);
                                    }
                                    e.call(
                                        DATA_CLASS_MEMBER_AT,
                                        &args![array_class, member_number, record],
                                    );
                                    if e.mem.u32(record + 8) == 8 {
                                        let name = e.mem.u32(record);
                                        let source = e
                                            .call(
                                                OBJECT_VALUE_AS_ARRAY,
                                                &args![value, frame + ARRAY_TEMP],
                                            )
                                            .u32();
                                        e.call(
                                            DATA_ARRAY_SWIZZLE_OBJECT_MEMBER,
                                            &args![source, frame + ARRAY_TEMP_2, name],
                                        );
                                        e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP]);
                                        let size = e
                                            .call(DATA_ARRAY_GET_SIZE, &args![frame + ARRAY_TEMP_2])
                                            .i32();
                                        for index in 0..size.max(0) {
                                            let element = e
                                                .call(
                                                    DATA_ARRAY_ELEMENT,
                                                    &args![
                                                        frame + ARRAY_TEMP_2,
                                                        frame + ELEMENT_2,
                                                        index
                                                    ],
                                                )
                                                .u32();
                                            let scanned = e.call(
                                                ARRAY_VALUE_AS_OBJECT,
                                                &args![element, frame + OBJECT_TEMP],
                                            );
                                            fn_00dbf450(e, this, scanned.ptr(), 0);
                                            e.call(
                                                DATA_OBJECT_DESTRUCT,
                                                &args![frame + OBJECT_TEMP],
                                            );
                                        }
                                        e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP_2]);
                                    }
                                    member_number += 1;
                                    let total = e
                                        .call(DATA_CLASS_GET_NUM_MEMBERS, &args![array_class])
                                        .i32();
                                    if member_number >= total {
                                        break;
                                    }
                                }
                            }
                            e.call(DATA_CLASS_DESTRUCT, &args![array_class]);
                        }
                        _ => {}
                    }
                }
            }
            iterator = e
                .call(DATA_OBJECT_ITERATOR_NEXT, &args![cached, iterator])
                .u32();
        }
        if flag == 0 {
            let record = record_at(e, this, record_index);
            let bytes = this.at(TagfileWriter::flag_bytes);
            let offset = e.get(bytes, HkArrayBase::m_size) as u32;
            e.set(record, ObjectRecord::flags_offset, offset);
            let flag_size = e.get(flags, HkArrayBase::m_size);
            let byte_count = flag_byte_count(flag_size);
            let allocator = router_allocator(e, 0xc);
            let old_size = e.get(bytes, HkArrayBase::m_size);
            let wanted = old_size.wrapping_add(byte_count);
            let capacity = (e.get(bytes, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK) as i32;
            if capacity < wanted {
                let doubled = capacity.wrapping_add(capacity);
                let new_capacity = if wanted >= doubled { wanted } else { doubled };
                e.call(ARRAY_RESERVE, &args![allocator, bytes, new_capacity, 1i32]);
            }
            let data = e.get(bytes, HkArrayBase::m_data).addr();
            e.set(
                bytes,
                HkArrayBase::m_size,
                old_size.wrapping_add(byte_count),
            );
            writer_member_write_flags_to_bytes(e, flags, data.wrapping_add(old_size as u32));
        }
        // The flag array's storage is returned to the router's allocator and the array emptied.
        let allocator = router_allocator(e, 0xc);
        let capacity = e.get(flags, HkArrayBase::m_capacityAndFlags);
        e.set(flags, HkArrayBase::m_size, 0);
        if capacity as i32 >= 0 {
            let data = e.get(flags, HkArrayBase::m_data);
            e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
        }
        e.set(flags, HkArrayBase::m_data, Ptr::NULL);
        e.set(flags, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        e.call(DATA_CLASS_DESTRUCT, &args![class]);
        e.call(DATA_OBJECT_DESTRUCT, &args![cached]);
    });
}

// Translated from 00dbf930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::_writeArrayItems` (engine map name): writes `count` elements of the array `array` to
/// the archive. An array of tuples (type bit 0x20) writes every element's own array in turn.
/// Otherwise by the array's type: 1 one byte each, 2 each as a variable-length integer, 3 each
/// as a float (`writeFloat32`), 4 first the width (3, or 4 when a fourth float of some element
/// is nonzero) then every vector with that many floats (`writeArrayFloat32`), 6 and 7 every
/// vector, matrix or quaternion with `type * 4 - 12` floats, 8 every object (`writeObject`),
/// 9 (structs) the member flags as raw bytes followed by the arrays of every flagged member
/// (each swizzled out of the array and written the same way), 10 every string.
pub fn writer_write_array_items(e: &mut Engine, this: Ptr<TagfileWriter>, array: Ptr, count: i32) {
    const ARRAY_TEMP: u32 = 0x00;
    const VALUE: u32 = 0x08;
    const OBJECT_TEMP: u32 = 0x18;
    const CLASS: u32 = 0x1c;
    const MEMBERS: u32 = 0x20;
    const FLAGS: u32 = 0x30;
    const BYTES: u32 = 0x40;
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        if e.call(DATA_ARRAY_GET_TYPE, &args![array]).u32() & 0x20 != 0 {
            for index in 0..count.max(0) {
                let element = e
                    .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                    .u32();
                e.call(ARRAY_VALUE_AS_ARRAY, &args![element, frame + ARRAY_TEMP]);
                let size = e
                    .call(DATA_ARRAY_GET_SIZE, &args![frame + ARRAY_TEMP])
                    .i32();
                writer_write_array_items(e, this, Ptr::new(frame + ARRAY_TEMP), size);
                e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP]);
            }
            return;
        }
        let kind = e.call(DATA_ARRAY_GET_TYPE, &args![array]).u32();
        if kind > 10 {
            return;
        }
        match kind {
            1 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).u32();
                    e.call(WRITE_BYTE, &args![this, number]);
                }
            }
            2 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).i32();
                    fn_00dbe790(e, this.cast(), number as u32, number >> 31);
                }
            }
            3 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let real = e.call(ARRAY_VALUE_AS_REAL, &args![element]).f32();
                    e.call(WRITE_FLOAT_32, &args![this, x87_quiet(real)]);
                }
            }
            4 => {
                let mut width = 3i32;
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, 4u32]).u32();
                    // FLDZ / FCOMP: anything but an exact zero (a NaN included) counts.
                    if e.mem.f32(vector + 0xc) != 0.0 {
                        width = 4;
                        break;
                    }
                }
                fn_00dbe790(e, this.cast(), width as u32, width >> 31);
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, 4u32]).u32();
                    e.call(WRITE_ARRAY_FLOAT_32, &args![this, vector, width]);
                }
            }
            6 | 7 => {
                let floats = e
                    .call(DATA_ARRAY_GET_TYPE, &args![array])
                    .i32()
                    .wrapping_mul(4)
                    .wrapping_sub(12);
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, floats]).u32();
                    e.call(WRITE_ARRAY_FLOAT_32, &args![this, vector, floats]);
                }
            }
            8 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let object = e
                        .call(ARRAY_VALUE_AS_OBJECT, &args![element, frame + OBJECT_TEMP])
                        .ptr();
                    writer_write_object(e, this, object);
                    e.call(DATA_OBJECT_DESTRUCT, &args![frame + OBJECT_TEMP]);
                }
            }
            9 => {
                let class = frame + CLASS;
                let class_ptr = e.call(DATA_ARRAY_GET_CLASS, &args![array, class]).u32();
                let member_count = e.call(DATA_CLASS_GET_NUM_MEMBERS, &args![class_ptr]).i32();
                e.call(DATA_CLASS_DESTRUCT, &args![class]);
                let members: Ptr<HkArrayBase> = Ptr::new(frame + MEMBERS);
                member_info_array_new(e, members, member_count);
                let class_ptr = e.call(DATA_ARRAY_GET_CLASS, &args![array, class]).u32();
                e.call(DATA_CLASS_GET_ALL_MEMBER_INFO, &args![class_ptr, members]);
                e.call(DATA_CLASS_DESTRUCT, &args![class]);
                let flags: Ptr<HkArrayBase> = Ptr::new(frame + FLAGS);
                writer_member_write_flags_constructor(e, flags, member_count);
                // The members that hold something in some element are flagged.
                let mut member_offset = 0u32;
                for member_number in 0..member_count.max(0) {
                    let data = e.get(members, HkArrayBase::m_data).addr();
                    let member: Ptr<MemberInfo> = Ptr::new(data.wrapping_add(member_offset));
                    if e.get(member, MemberInfo::type_word) != 0 {
                        let name = e.get(member, MemberInfo::name);
                        e.call(
                            DATA_ARRAY_SWIZZLE_OBJECT_MEMBER,
                            &args![array, frame + ARRAY_TEMP, name],
                        );
                        let mut index = 0i32;
                        let mut size = e
                            .call(DATA_ARRAY_GET_SIZE, &args![frame + ARRAY_TEMP])
                            .i32();
                        while index < size {
                            let element = e
                                .call(
                                    DATA_ARRAY_ELEMENT,
                                    &args![frame + ARRAY_TEMP, frame + VALUE, index],
                                )
                                .u32();
                            let worth = writer_worth_writing_data_array_value(
                                e,
                                this.cast(),
                                Ptr::new(element),
                            );
                            if worth != 0 {
                                let flag_data = e.get(flags, HkArrayBase::m_data).addr();
                                e.mem
                                    .set_u8(flag_data.wrapping_add(member_number as u32), 1);
                                break;
                            }
                            index += 1;
                            size = e
                                .call(DATA_ARRAY_GET_SIZE, &args![frame + ARRAY_TEMP])
                                .i32();
                        }
                        e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP]);
                    }
                    member_offset = member_offset.wrapping_add(MEMBER_INFO_SIZE);
                }
                // The flags go out as raw bytes.
                let bytes: Ptr<HkArrayBase> = Ptr::new(frame + BYTES);
                e.set(bytes, HkArrayBase::m_data, Ptr::NULL);
                e.set(bytes, HkArrayBase::m_size, 0);
                e.set(bytes, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
                let byte_count = flag_byte_count(e.get(flags, HkArrayBase::m_size));
                array_ensure_capacity(e, bytes, byte_count, 1);
                let data = e.get(bytes, HkArrayBase::m_data).addr();
                e.set(bytes, HkArrayBase::m_size, byte_count);
                writer_member_write_flags_to_bytes(e, flags, data);
                e.call(WRITE_RAW, &args![this, data, byte_count]);
                let allocator = router_allocator(e, 0xc);
                let capacity = e.get(bytes, HkArrayBase::m_capacityAndFlags);
                e.set(bytes, HkArrayBase::m_size, 0);
                if capacity as i32 >= 0 {
                    let data = e.get(bytes, HkArrayBase::m_data);
                    e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
                }
                // The arrays of the flagged members follow.
                let mut member_offset = 0u32;
                for member_number in 0..member_count.max(0) {
                    let flag_data = e.get(flags, HkArrayBase::m_data).addr();
                    if e.mem.u8(flag_data.wrapping_add(member_number as u32)) != 0 {
                        let data = e.get(members, HkArrayBase::m_data).addr();
                        let name = e.mem.u32(data.wrapping_add(member_offset));
                        e.call(
                            DATA_ARRAY_SWIZZLE_OBJECT_MEMBER,
                            &args![array, frame + ARRAY_TEMP, name],
                        );
                        writer_write_array_items(e, this, Ptr::new(frame + ARRAY_TEMP), count);
                        e.call(DATA_ARRAY_DESTRUCT, &args![frame + ARRAY_TEMP]);
                    }
                    member_offset = member_offset.wrapping_add(MEMBER_INFO_SIZE);
                }
                // The flag array and the member records go back to the allocator.
                let allocator = router_allocator(e, 0xc);
                let capacity = e.get(flags, HkArrayBase::m_capacityAndFlags);
                e.set(flags, HkArrayBase::m_size, 0);
                if capacity as i32 >= 0 {
                    let data = e.get(flags, HkArrayBase::m_data);
                    e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
                }
                e.set(flags, HkArrayBase::m_data, Ptr::NULL);
                e.set(flags, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
                member_info_array_free(e, members);
            }
            10 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(DATA_ARRAY_ELEMENT_OV2, &args![array, frame + VALUE, index])
                        .u32();
                    let string = e.call(ARRAY_VALUE_AS_STRING, &args![element]).u32();
                    writer_write_string(e, this, string);
                }
            }
            _ => {}
        }
    });
}

// Translated from 00dbfef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::writeBinaryValue` (engine map name): writes the value at `value` (an
/// `hkDataObject::Value`) of the member record `member` (`MemberInfo`). A tuple type (bit 0x20)
/// writes its array's items; an array type (bit 0x10) writes the array's size, for an array of
/// structs (type 9) whose member has no class (`member + 0xc` is 0) the class's number from
/// the class map, then the items; otherwise by type: 1 a byte, 2 an integer, 3 a float, 4, 6
/// and 7 `type * 4 - 12` floats, 8 and 9 an object (`writeObject`), 10 a string.
pub fn writer_write_binary_value(
    e: &mut Engine,
    this: Ptr<TagfileWriter>,
    value: Ptr,
    member: Ptr<MemberInfo>,
) {
    e.with_stack(0xc, |e, frame| {
        let array_temp = frame.addr();
        let class_temp = frame.addr() + 4;
        let object_temp = frame.addr() + 8;
        if e.call(OBJECT_VALUE_GET_TYPE, &args![value]).u32() & 0x20 != 0 {
            e.call(OBJECT_VALUE_AS_ARRAY, &args![value, array_temp]);
            let size = e.call(DATA_ARRAY_GET_SIZE, &args![array_temp]).i32();
            writer_write_array_items(e, this, Ptr::new(array_temp), size);
            e.call(DATA_ARRAY_DESTRUCT, &args![array_temp]);
            return;
        }
        if e.call(OBJECT_VALUE_GET_TYPE, &args![value]).u32() & 0x10 != 0 {
            e.call(OBJECT_VALUE_AS_ARRAY, &args![value, array_temp]);
            let size = e.call(DATA_ARRAY_GET_SIZE, &args![array_temp]).i32();
            fn_00dbe790(e, this.cast(), size as u32, size >> 31);
            if e.call(DATA_ARRAY_GET_TYPE, &args![array_temp]).u32() == 9
                && e.get(member, MemberInfo::class) == 0
            {
                e.call(DATA_ARRAY_GET_CLASS, &args![array_temp, class_temp]);
                let name = e.call(DATA_CLASS_GET_NAME, &args![class_temp]).u32();
                let classes = this.at(TagfileWriter::classes).addr();
                let number = e
                    .call(STRING_MAP_GET_WITH_DEFAULT, &args![classes, name, 0u32])
                    .i32();
                fn_00dbe790(e, this.cast(), number as u32, number >> 31);
                e.call(DATA_CLASS_DESTRUCT, &args![class_temp]);
            }
            writer_write_array_items(e, this, Ptr::new(array_temp), size);
            e.call(DATA_ARRAY_DESTRUCT, &args![array_temp]);
            return;
        }
        match e.call(OBJECT_VALUE_GET_TYPE, &args![value]).u32() {
            1 => {
                let number = e.call(OBJECT_VALUE_AS_INT, &args![value]).u32();
                e.call(WRITE_BYTE, &args![this, number]);
            }
            2 => {
                let number = e.call(OBJECT_VALUE_AS_INT, &args![value]).i32();
                fn_00dbe790(e, this.cast(), number as u32, number >> 31);
            }
            3 => {
                let real = e.call(OBJECT_VALUE_AS_REAL, &args![value]).f32();
                e.call(WRITE_FLOAT_32, &args![this, x87_quiet(real)]);
            }
            4 | 6 | 7 => {
                let floats = e
                    .call(OBJECT_VALUE_GET_TYPE, &args![value])
                    .i32()
                    .wrapping_mul(4)
                    .wrapping_sub(12);
                let vector = e.call(OBJECT_VALUE_AS_VEC, &args![value, floats]).u32();
                e.call(WRITE_ARRAY_FLOAT_32, &args![this, vector, floats]);
            }
            8 | 9 => {
                let object = e
                    .call(OBJECT_VALUE_AS_OBJECT, &args![value, object_temp])
                    .ptr();
                writer_write_object(e, this, object);
                e.call(DATA_OBJECT_DESTRUCT, &args![object_temp]);
            }
            10 => {
                let string = e.call(OBJECT_VALUE_AS_STRING, &args![value]).u32();
                writer_write_string(e, this, string);
            }
            _ => {}
        }
    });
}

// Translated from 00dc00b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::writeObject` (engine map name): writes the object at `object` (the listener cache
/// may replace it). A null object is the tag 6. An object with a number (its record's `+4` not
/// negative) is the tag 5 and the number. Otherwise the tag is 4 for an object met more than
/// once (it gets the writer's next object number, kept in the record) or 3 for one met once,
/// then the class's number (`writeClass`); the object's flag bytes are written raw (taken from
/// the writer's flag bytes at the record's offset and unpacked into a flag array), and every
/// flagged member is written (`writeBinaryValue`).
pub fn writer_write_object(e: &mut Engine, this: Ptr<TagfileWriter>, object: Ptr) {
    const CACHED: u32 = 0x00;
    const CLASS: u32 = 0x04;
    const HANDLE: u32 = 0x08;
    const VALUE: u32 = 0x10;
    const FLAGS: u32 = 0x18;
    const MEMBERS: u32 = 0x28;
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let cached = frame + CACHED;
        let cache = this.at(TagfileWriter::cache);
        writer_listener_cache_get_cached_data_object(e, cache, Ptr::new(cached), object);
        if e.call(DATA_OBJECT_IS_NULL, &args![cached]).u32() != 0 {
            fn_00dbe790(e, this.cast(), 6, 0);
            e.call(DATA_OBJECT_DESTRUCT, &args![cached]);
            return;
        }
        let handle = e
            .call(DATA_OBJECT_GET_HANDLE, &args![cached, frame + HANDLE])
            .u32();
        let (low, high) = (e.mem.u32(handle), e.mem.u32(handle + 4));
        let object_indices = this.at(TagfileWriter::object_indices);
        let index = hk_map_base_get_with_default(e, object_indices, low, high, 0xffff_ffff);
        let record = record_at(e, this, index);
        if e.get(record, ObjectRecord::number) >= 0 {
            fn_00dbe790(e, this.cast(), 5, 0);
            let number = e.get(record, ObjectRecord::number);
            fn_00dbe790(e, this.cast(), number as u32, number >> 31);
            e.call(DATA_OBJECT_DESTRUCT, &args![cached]);
            return;
        }
        let class = frame + CLASS;
        e.call(DATA_OBJECT_GET_CLASS, &args![cached, class]);
        let class_number = writer_write_class(e, this, Ptr::new(class));
        if (e.get(record, ObjectRecord::references) as i32) > 1 {
            fn_00dbe790(e, this.cast(), 4, 0);
            let next = e.get(this, TagfileWriter::next_object_number);
            e.set(record, ObjectRecord::number, next as i32);
            e.set(
                this,
                TagfileWriter::next_object_number,
                next.wrapping_add(1),
            );
        } else {
            fn_00dbe790(e, this.cast(), 3, 0);
        }
        e.set(record, ObjectRecord::written, 1);
        fn_00dbe790(e, this.cast(), class_number as u32, class_number >> 31);
        let member_count = e.call(DATA_CLASS_GET_NUM_MEMBERS, &args![class]).i32();
        let flags: Ptr<HkArrayBase> = Ptr::new(frame + FLAGS);
        writer_member_write_flags_constructor(e, flags, member_count);
        let bytes = this.at(TagfileWriter::flag_bytes);
        let flag_data = e
            .get(record, ObjectRecord::flags_offset)
            .wrapping_add(e.get(bytes, HkArrayBase::m_data).addr());
        let byte_count = flag_byte_count(e.get(flags, HkArrayBase::m_size));
        e.call(WRITE_RAW, &args![this, flag_data, byte_count]);
        writer_member_write_flags_from_bytes(e, flags, flag_data, byte_count);
        // The flag block is never returned: its capacity word is built with bit 31 set, which
        // the exe tests (and so skips) all the same.
        let allocator = router_allocator(e, 0xc);
        let marked = byte_count as u32 | NOT_OWNED;
        if marked as i32 >= 0 {
            e.vcall(allocator, 0x10, &args![flag_data, marked & CAPACITY_MASK]);
        }
        let members: Ptr<HkArrayBase> = Ptr::new(frame + MEMBERS);
        member_info_array_new(e, members, member_count);
        e.call(DATA_CLASS_GET_ALL_MEMBER_INFO, &args![class, members]);
        let mut member_offset = 0u32;
        for member_number in 0..member_count.max(0) {
            let flag_array = e.get(flags, HkArrayBase::m_data).addr();
            if e.mem.u8(flag_array.wrapping_add(member_number as u32)) != 0 {
                let data = e.get(members, HkArrayBase::m_data).addr();
                let member = data.wrapping_add(member_offset);
                let name = e.mem.u32(member);
                e.call(
                    DATA_OBJECT_MEMBER_VALUE,
                    &args![cached, frame + VALUE, name],
                );
                writer_write_binary_value(e, this, Ptr::new(frame + VALUE), Ptr::new(member));
            }
            member_offset = member_offset.wrapping_add(MEMBER_INFO_SIZE);
        }
        member_info_array_free(e, members);
        e.set(members, HkArrayBase::m_data, Ptr::NULL);
        e.set(members, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        let allocator = router_allocator(e, 0xc);
        let capacity = e.get(flags, HkArrayBase::m_capacityAndFlags);
        e.set(flags, HkArrayBase::m_size, 0);
        if capacity & NOT_OWNED == 0 {
            let data = e.get(flags, HkArrayBase::m_data);
            e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
        }
        e.set(flags, HkArrayBase::m_data, Ptr::NULL);
        e.set(flags, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        e.call(DATA_CLASS_DESTRUCT, &args![class]);
        e.call(DATA_OBJECT_DESTRUCT, &args![cached]);
    });
}

// Translated from 00dc0380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. The constructor of `Writer::ListenerCache`: stores the listener,
/// empties the handle map (no storage, `m_hashMod` -1) and reserves it for 0 elements
/// (`00dbedb0`, through the router's allocator at `+0x10`), and empties the object array (not
/// owned). Returns `this`.
pub fn fn_00dc0380(e: &mut Engine, this: Ptr<ListenerCache>, listener: Ptr) -> Ptr<ListenerCache> {
    e.set(this, ListenerCache::listener, listener);
    let handles = this.at(ListenerCache::handles);
    e.set(handles, HkMapBase::m_elem, 0);
    e.set(handles, HkMapBase::m_numElems, 0);
    e.set(handles, HkMapBase::m_hashMod, -1);
    let allocator = router_allocator(e, 0x10);
    fn_00dbedb0(e, handles, Ptr::new(allocator), 0);
    let objects = this.at(ListenerCache::objects);
    e.set(objects, HkArrayBase::m_data, Ptr::NULL);
    e.set(objects, HkArrayBase::m_size, 0);
    e.set(objects, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    this
}

// Translated from 00dc03d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::ListenerCache::~ListenerCache` (engine map name): destroys every cached object (its
/// `hkDataObject` destructor, then the 4-byte block returned through the router's allocator at
/// `+0x10`, slot 8), empties the array and returns its storage (allocator at `+0xc`, slot
/// `0x10`) unless it is not owned, and empties the handle map (`00dbe950`).
pub fn writer_listener_cache_destructor(e: &mut Engine, this: Ptr<ListenerCache>) {
    let objects = this.at(ListenerCache::objects);
    let mut index = 0i32;
    while index < e.get(objects, HkArrayBase::m_size) {
        let data = e.get(objects, HkArrayBase::m_data).addr();
        let element = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if element != 0 {
            e.call(DATA_OBJECT_DESTRUCT, &args![element]);
            let allocator = router_allocator(e, 0x10);
            e.vcall(allocator, 8, &args![element, 4u32]);
        }
        index += 1;
    }
    let allocator = router_allocator(e, 0xc);
    let capacity = e.get(objects, HkArrayBase::m_capacityAndFlags);
    e.set(objects, HkArrayBase::m_size, 0);
    if capacity as i32 >= 0 {
        let data = e.get(objects, HkArrayBase::m_data);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
    e.set(objects, HkArrayBase::m_data, Ptr::NULL);
    e.set(objects, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let allocator = router_allocator(e, 0x10);
    fn_00dbe950(e, this.at(ListenerCache::handles), Ptr::new(allocator));
}

// Translated from 00dc0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::~Writer` (engine map name): destroys the listener cache, returns the records' and
/// the flag bytes' storage to the router's allocator at `+0xc` (slot `0x10`, sizes `0x18` and 1
/// per element) unless not owned, empties the object map (`00dbe950`), destructs the two string
/// maps (`00d877d0`) and ends with the `hkOArchive` destructor (`00d60590`).
pub fn writer_destructor(e: &mut Engine, this: Ptr<TagfileWriter>) {
    writer_listener_cache_destructor(e, this.at(TagfileWriter::cache));
    let records = this.at(TagfileWriter::records);
    let allocator = router_allocator(e, 0xc);
    let capacity = e.get(records, HkArrayBase::m_capacityAndFlags);
    e.set(records, HkArrayBase::m_size, 0);
    if capacity as i32 >= 0 {
        let data = e.get(records, HkArrayBase::m_data);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(0x18)],
        );
    }
    e.set(records, HkArrayBase::m_data, Ptr::NULL);
    e.set(records, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let bytes = this.at(TagfileWriter::flag_bytes);
    let allocator = router_allocator(e, 0xc);
    let capacity = e.get(bytes, HkArrayBase::m_capacityAndFlags);
    e.set(bytes, HkArrayBase::m_size, 0);
    if capacity as i32 >= 0 {
        let data = e.get(bytes, HkArrayBase::m_data);
        e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
    }
    e.set(bytes, HkArrayBase::m_data, Ptr::NULL);
    e.set(bytes, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let allocator = router_allocator(e, 0x10);
    fn_00dbe950(
        e,
        this.at(TagfileWriter::object_indices),
        Ptr::new(allocator),
    );
    e.call(STRING_MAP_DESTRUCT, &args![this.at(TagfileWriter::classes)]);
    e.call(STRING_MAP_DESTRUCT, &args![this.at(TagfileWriter::strings)]);
    e.call(ARCHIVE_DESTRUCT, &args![this]);
}

// Translated from 00dc0500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Writer::Writer` (engine map name): constructs the `hkOArchive` over `stream`, the two string
/// maps, the object counter (0), the object map (empty, reserved for 0 elements), the empty flag
/// byte and record arrays and the listener cache over `listener`; both string maps start with
/// the empty string (`01011584`, number 0). The words the code passes for the archive's flag
/// and the string maps' ignored argument are the `this` and `stream` words with the low byte
/// cleared. Returns `this`.
pub fn writer_constructor(
    e: &mut Engine,
    this: Ptr<TagfileWriter>,
    stream: Ptr,
    listener: Ptr,
) -> Ptr<TagfileWriter> {
    e.call(
        ARCHIVE_CONSTRUCT,
        &args![this, stream, this.addr() & 0xffff_ff00],
    );
    let ignored = stream.addr() & 0xffff_ff00;
    let strings = this.at(TagfileWriter::strings);
    e.call(STRING_MAP_CONSTRUCT, &args![strings, ignored]);
    let classes = this.at(TagfileWriter::classes);
    e.call(STRING_MAP_CONSTRUCT, &args![classes, ignored]);
    e.set(this, TagfileWriter::next_object_number, 0);
    let object_indices = this.at(TagfileWriter::object_indices);
    e.set(object_indices, HkMapBase::m_elem, 0);
    e.set(object_indices, HkMapBase::m_numElems, 0);
    e.set(object_indices, HkMapBase::m_hashMod, -1);
    let allocator = router_allocator(e, 0x10);
    fn_00dbedb0(e, object_indices, Ptr::new(allocator), 0);
    let bytes = this.at(TagfileWriter::flag_bytes);
    e.set(bytes, HkArrayBase::m_data, Ptr::NULL);
    e.set(bytes, HkArrayBase::m_size, 0);
    e.set(bytes, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let records = this.at(TagfileWriter::records);
    e.set(records, HkArrayBase::m_data, Ptr::NULL);
    e.set(records, HkArrayBase::m_size, 0);
    e.set(records, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    fn_00dc0380(e, this.at(TagfileWriter::cache), listener);
    e.call(STRING_MAP_INSERT, &args![classes, EMPTY_STRING, 0u32]);
    e.call(STRING_MAP_INSERT, &args![strings, EMPTY_STRING, 0u32]);
    this
}

// Translated from 00dc05b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryTagfileWriter::save` (engine map name): writes `object` to the archive stream
/// `stream` as a binary tagfile through a `Writer` built on the stack (`listener` may replace
/// objects): the magic words `0xcab00d1e` and `0xd011face`, the numbers 1 and 0, the first pass
/// over the object (`00dbf450`), the object itself (`writeObject`), the stream's slot `0x14`
/// (called with no arguments), then the writer is destroyed. Returns 0.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_binary_tagfile_writer_save(
    e: &mut Engine,
    _this: u32,
    object: Ptr,
    stream: Ptr,
    listener: Ptr,
) -> u32 {
    e.with_stack(0x74, |e, block| {
        let writer: Ptr<TagfileWriter> = block.cast();
        writer_constructor(e, writer, stream, listener);
        e.call(WRITE_32, &args![writer, 0xcab0_0d1eu32]);
        e.call(WRITE_32, &args![writer, 0xd011_faceu32]);
        fn_00dbe790(e, writer.cast(), 1, 0);
        fn_00dbe790(e, writer.cast(), 0, 0);
        fn_00dbf450(e, writer, object, 0);
        writer_write_object(e, writer, object);
        e.vcall(stream.addr(), 0x14, &args![]);
        writer_destructor(e, writer);
    });
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dbe790, fn_00dbe790(Ptr, u32, i32)),
        entry!(0x00dbe810, fn_00dbe810(Ptr<HkMapBase>, Ptr, u32, u32, u32)),
        entry!(
            0x00dbe8c0,
            hk_map_base_get_with_default(Ptr<HkMapBase>, u32, u32, u32) -> u32
        ),
        entry!(0x00dbe920, fn_00dbe920(Ptr<HkMapBase>)),
        entry!(0x00dbe950, fn_00dbe950(Ptr<HkMapBase>, Ptr)),
        entry!(
            0x00dbe980,
            writer_worth_writing_data_object_value(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00dbeab0,
            writer_worth_writing_data_array_value(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00dbebe0,
            writer_listener_cache_get_cached_data_object(Ptr<ListenerCache>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00dbec60, fn_00dbec60(Ptr<HkMapBase>, Ptr, i32)),
        entry!(0x00dbed30, writer_write_string(Ptr<TagfileWriter>, u32)),
        entry!(0x00dbedb0, fn_00dbedb0(Ptr<HkMapBase>, Ptr, i32)),
        entry!(
            0x00dbedd0,
            writer_listener_cache_call_listener_and_cache(Ptr<ListenerCache>, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00dbef20,
            writer_member_write_flags_constructor(Ptr<HkArrayBase>, i32) -> Ptr<HkArrayBase>
        ),
        entry!(
            0x00dbefc0,
            writer_member_write_flags_from_bytes(Ptr<HkArrayBase>, u32, i32)
        ),
        entry!(
            0x00dbf0d0,
            writer_member_write_flags_to_bytes(Ptr<HkArrayBase>, u32)
        ),
        entry!(
            0x00dbf1a0,
            writer_write_class(Ptr<TagfileWriter>, Ptr) -> i32
        ),
        entry!(0x00dbf450, fn_00dbf450(Ptr<TagfileWriter>, Ptr, u32)),
        entry!(
            0x00dbf930,
            writer_write_array_items(Ptr<TagfileWriter>, Ptr, i32)
        ),
        entry!(
            0x00dbfef0,
            writer_write_binary_value(Ptr<TagfileWriter>, Ptr, Ptr<MemberInfo>)
        ),
        entry!(0x00dc00b0, writer_write_object(Ptr<TagfileWriter>, Ptr)),
        entry!(
            0x00dc0380,
            fn_00dc0380(Ptr<ListenerCache>, Ptr) -> Ptr<ListenerCache>
        ),
        entry!(
            0x00dc03d0,
            writer_listener_cache_destructor(Ptr<ListenerCache>)
        ),
        entry!(0x00dc0460, writer_destructor(Ptr<TagfileWriter>)),
        entry!(
            0x00dc0500,
            writer_constructor(Ptr<TagfileWriter>, Ptr, Ptr) -> Ptr<TagfileWriter>
        ),
        entry!(0x00dc05b0, hk_binary_tagfile_writer_save(u32, Ptr, Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Shared<T> = Rc<RefCell<T>>;

    /// The allocator at `router + 0xc` (growth and frees) and the one at `router + 0x10`
    /// (blocks and maps): their vtable slots are doubles at these addresses plus the slot.
    const ALLOCATOR_GROWTH: u32 = 0x0900_0100;
    const ALLOCATOR_BLOCKS: u32 = 0x0900_0200;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        // The page of `0.0f` (`01011d78`) and the empty string (`01011584`), and the page of the
        // default object record (`010d6848`).
        e.map(0x0101_1000, 0x1000);
        e.map(0x010d_6000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument words of every call to `address`, in order.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(a, _)| *a == address)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Installs the memory router and its two allocators over the engine's heap (slot 4 allocates
    /// the size, slot 8 and slot `0x10` free the pointer, slot `0xc` allocates the size the
    /// pointed-to word holds and leaves it). Returns the allocator objects (growth, blocks).
    fn install_router(e: &mut Engine) -> (u32, u32) {
        let mut objects = [0u32; 2];
        for (n, base) in [ALLOCATOR_GROWTH, ALLOCATOR_BLOCKS].into_iter().enumerate() {
            let vtable = e.mem.alloc(0x20);
            for slot in [4u32, 8, 0xc, 0x10] {
                e.mem.set_u32(vtable + slot, base + slot);
            }
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            objects[n] = object;
            e.register(base + 4, |e, a| ret(e.mem.alloc(a[1])));
            e.register(base + 8, |e, a| {
                e.mem.free(a[1]);
                Ret::default()
            });
            e.register(base + 0xc, |e, a| {
                let size = e.mem.u32(a[1]);
                ret(e.mem.alloc(size))
            });
            e.register(base + 0x10, |e, a| {
                e.mem.free(a[1]);
                Ret::default()
            });
        }
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, objects[0]);
        e.mem.set_u32(router + 0x10, objects[1]);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        (objects[0], objects[1])
    }

    /// Moves an `hkArray` to a new block of `capacity` elements (the old elements copied).
    fn reallocate(e: &mut Engine, array: u32, capacity: u32, element_size: u32) {
        let old = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let data = e.mem.alloc(capacity * element_size);
        if old != 0 {
            let bytes = e.mem.bytes(old, size * element_size);
            e.mem.write(data, &bytes);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Doubles for `hkArrayUtil::_reserve` and the growth step (new block, old elements copied,
    /// capacity as asked or doubled, at least 4) and for `memset`.
    fn install_arrays(e: &mut Engine) {
        e.register(ARRAY_RESERVE, |e, a| {
            reallocate(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register(ARRAY_GROW, |e, a| {
            let capacity = (e.mem.u32(a[1] + 8) & CAPACITY_MASK) * 2;
            reallocate(e, a[1], capacity.max(4), a[2]);
            Ret::default()
        });
        e.register(MEM_SET, |e, a| {
            let zeroes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &zeroes);
            Ret::default()
        });
    }

    /// Records what the archive receives: bytes from the byte writer (low byte of the word), raw
    /// blocks from `writeRaw`, floats from `writeFloat32` and `writeArrayFloat32` (as bit
    /// patterns), 32-bit words from `write32`.
    fn install_archive(e: &mut Engine) -> Shared<Vec<u8>> {
        let stream = Rc::new(RefCell::new(Vec::new()));
        let bytes = Rc::clone(&stream);
        e.register_double(WRITE_BYTE, move |_, a| {
            bytes.borrow_mut().push(a[1] as u8);
            Ret::default()
        });
        let raw = Rc::clone(&stream);
        e.register_double(WRITE_RAW, move |e, a| {
            let block = e.mem.bytes(a[1], a[2]);
            raw.borrow_mut().extend(block);
            Ret::default()
        });
        let words = Rc::clone(&stream);
        e.register_double(WRITE_32, move |_, a| {
            words.borrow_mut().extend(a[1].to_le_bytes());
            Ret::default()
        });
        let floats = Rc::clone(&stream);
        e.register_double(WRITE_FLOAT_32, move |_, a| {
            floats.borrow_mut().extend(a[1].to_le_bytes());
            Ret::default()
        });
        let float_array = Rc::clone(&stream);
        e.register_double(WRITE_ARRAY_FLOAT_32, move |e, a| {
            for n in 0..a[2] {
                let bits = e.mem.u32(a[1] + 4 * n);
                float_array.borrow_mut().extend(bits.to_le_bytes());
            }
            Ret::default()
        });
        stream
    }

    fn varint_bytes(low: u32, high: i32) -> Vec<u32> {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let inner = Rc::clone(&log);
        e.register_double(WRITE_BYTE, move |_, a| {
            inner.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.call(0x00db_e790, &args![0x1000u32, low, high]);
        let words = log.borrow().clone();
        words
    }

    #[test]
    fn variable_length_integers_are_seven_bits_per_byte_with_the_sign_in_bit_zero() {
        assert_eq!(varint_bytes(0, 0), vec![0]);
        assert_eq!(varint_bytes(1, 0), vec![2]);
        assert_eq!(varint_bytes(200, 0), vec![0x90, 3]);
        // -1 is (1 * 2) | 1; the exe passes the last byte in the word `low` with its low byte replaced.
        assert_eq!(varint_bytes(0xffff_ffff, -1), vec![0xffff_ff03]);
        // -200 is 400 | 1; the last word keeps the upper bytes of `low`.
        assert_eq!(varint_bytes(0xffff_ff38, -1), vec![0x91, 0xffff_ff03]);
        assert_eq!(varint_bytes(0x1_0000, 0), vec![0x80, 0x80, 0x1_0008]);
        // A value above 32 bits: 2^32 * 2 = 2^33.
        assert_eq!(varint_bytes(0, 1), vec![0x80, 0x80, 0x80, 0x80, 0x20]);
    }

    fn new_map(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(12);
        e.mem.set_i32(map + 8, -1);
        map
    }

    #[test]
    fn insert_grows_the_empty_table_and_replaces_equal_keys() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        e.call(0x00db_e810, &args![map, blocks, 0x100u32, 1u32, 11u32]);
        assert_eq!(e.mem.u32(map + 4), 1);
        assert_eq!(e.mem.i32(map + 8), 7);
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 4), vec![vec![blocks, 96]]);
        let get = |e: &mut Engine, high: u32, default: u32| {
            e.call(0x00db_e8c0, &args![map, 0x100u32, high, default])
                .u32()
        };
        assert_eq!(get(&mut e, 1, 99), 11);
        assert_eq!(get(&mut e, 2, 99), 99);
        e.call(0x00db_e810, &args![map, blocks, 0x100u32, 1u32, 12u32]);
        assert_eq!(e.mem.u32(map + 4), 1);
        assert_eq!(get(&mut e, 1, 99), 12);
    }

    #[test]
    fn insert_probes_past_equal_hashes_and_grows_when_half_full() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        // The same low word hashes to the same slot; the high word tells the keys apart.
        for high in 1..=4u32 {
            e.call(0x00db_e810, &args![map, blocks, 0x100u32, high, high * 10]);
        }
        assert_eq!(e.mem.i32(map + 8), 7);
        let old = e.mem.u32(map);
        e.call(0x00db_e810, &args![map, blocks, 0x100u32, 5u32, 50u32]);
        assert_eq!(e.mem.i32(map + 8), 15);
        assert_eq!(e.mem.u32(map + 4), 5);
        for high in 1..=5u32 {
            let value = e.call(0x00db_e8c0, &args![map, 0x100u32, high, 0u32]).u32();
            assert_eq!(value, high * 10);
        }
        // The old table (8 slots of 12 bytes) went back to the allocator.
        assert_eq!(
            calls(&e, ALLOCATOR_BLOCKS + 8).last(),
            Some(&vec![blocks, old, 96])
        );
    }

    #[test]
    fn lookup_without_a_positive_mask_answers_the_default() {
        let mut e = engine();
        let map = e.mem.alloc(12);
        let slots = e.mem.alloc(24);
        e.mem.set_u32(slots, 0x100);
        e.mem.set_u32(slots + 4, 1);
        e.mem.set_u32(slots + 8, 5);
        e.mem.set_u32(map, slots);
        e.mem.set_i32(map + 8, 0);
        let value = e
            .call(0x00db_e8c0, &args![map, 0x100u32, 1u32, 77u32])
            .u32();
        assert_eq!(value, 77);
    }

    #[test]
    fn clear_empties_every_slot_and_keeps_the_not_owned_flag() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        e.call(0x00db_e810, &args![map, blocks, 0x100u32, 1u32, 11u32]);
        let table = e.mem.u32(map);
        e.mem.set_u32(map + 4, 0x8000_0001);
        e.call(0x00db_e920, &args![map]);
        assert_eq!(e.mem.u32(map + 4), 0x8000_0000);
        for slot in 0..8 {
            assert_eq!(e.mem.u32(table + 12 * slot), 0);
        }
    }

    #[test]
    fn clear_and_free_returns_owned_slots_only() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        e.call(0x00db_e810, &args![map, blocks, 0x100u32, 1u32, 11u32]);
        let table = e.mem.u32(map);
        e.call(0x00db_e950, &args![map, blocks]);
        assert_eq!(e.mem.u32(map + 4), 0);
        assert_eq!(
            calls(&e, ALLOCATOR_BLOCKS + 8).last(),
            Some(&vec![blocks, table, 96])
        );
        // Not owned: emptied, but not freed.
        let before = calls(&e, ALLOCATOR_BLOCKS + 8).len();
        e.mem.set_u32(map + 4, 0x8000_0000);
        e.call(0x00db_e950, &args![map, blocks]);
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 8).len(), before);
    }

    #[test]
    fn resize_moves_every_element_and_frees_the_old_table() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        for high in 1..=3u32 {
            e.call(
                0x00db_e810,
                &args![map, blocks, 0x40u32 * high, high, high + 100],
            );
        }
        let old = e.mem.u32(map);
        e.call(0x00db_ec60, &args![map, blocks, 20i32]);
        assert_eq!(e.mem.i32(map + 8), 19);
        assert_eq!(e.mem.u32(map + 4), 3);
        for high in 1..=3u32 {
            let value = e
                .call(0x00db_e8c0, &args![map, 0x40u32 * high, high, 0u32])
                .u32();
            assert_eq!(value, high + 100);
        }
        assert_eq!(
            calls(&e, ALLOCATOR_BLOCKS + 8).last(),
            Some(&vec![blocks, old, 96])
        );
        // Fewer than 8 slots are raised to 8; a not-owned table is not freed.
        e.mem.set_u32(map + 4, 0x8000_0000 | 3);
        let frees = calls(&e, ALLOCATOR_BLOCKS + 8).len();
        e.call(0x00db_ec60, &args![map, blocks, 3i32]);
        assert_eq!(e.mem.i32(map + 8), 7);
        assert_eq!(e.mem.u32(map + 4), 3);
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 8).len(), frees);
    }

    #[test]
    fn reserve_picks_the_power_of_two_slot_count() {
        for (count, slots) in [(0, 8u32), (4, 8), (5, 16), (8, 16), (9, 32)] {
            let mut e = engine();
            let (_, blocks) = install_router(&mut e);
            let map = new_map(&mut e);
            e.call(0x00db_edb0, &args![map, blocks, count]);
            let allocations = calls(&e, ALLOCATOR_BLOCKS + 4);
            assert_eq!(allocations, vec![vec![blocks, slots * 12]], "count {count}");
            assert_eq!(e.mem.i32(map + 8), slots as i32 - 1);
        }
    }

    /// Doubles for one kind of `Value`: its six accessors and the object and array helpers they
    /// hand results to. `types` is what successive `getType` calls answer (the last one repeats).
    fn install_value(
        e: &mut Engine,
        accessors: &ValueAccessors,
        types: Vec<u32>,
        integer: u32,
        real: f32,
        object_null: bool,
        string: u32,
        array_size: i32,
    ) {
        let types = Rc::new(RefCell::new(types));
        let sequence = Rc::clone(&types);
        e.register_double(accessors.get_type, move |_, _| {
            let mut types = sequence.borrow_mut();
            let next = if types.len() > 1 {
                types.remove(0)
            } else {
                types[0]
            };
            ret(next)
        });
        e.register_double(accessors.as_int, move |_, _| ret(integer));
        e.register_double(accessors.as_real, move |_, _| Ret {
            st0: f64::from(real),
            ..Ret::default()
        });
        e.register_double(accessors.as_object, move |e, a| {
            e.mem.set_u32(a[1], if object_null { 0 } else { 1 });
            ret(a[1])
        });
        e.register_double(accessors.as_string, move |_, _| ret(string));
        e.register_double(accessors.as_array, move |_, a| ret(a[1]));
        e.register(DATA_OBJECT_IS_NULL, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
        });
        e.register(DATA_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register_double(DATA_ARRAY_GET_SIZE, move |_, _| ret(array_size as u32));
        e.register(DATA_ARRAY_DESTRUCT, |_, _| Ret::default());
    }

    fn worth(
        address: u32,
        accessors: &ValueAccessors,
        types: &[u32],
        integer: u32,
        real: f32,
        object_null: bool,
        string: u32,
        array_size: i32,
    ) -> u32 {
        let mut e = engine();
        install_value(
            &mut e,
            accessors,
            types.to_vec(),
            integer,
            real,
            object_null,
            string,
            array_size,
        );
        e.call(address, &args![0u32, 0x5000u32]).u32()
    }

    fn check_worth(address: u32, accessors: &ValueAccessors) {
        let w = |types: &[u32], integer, real, null, string, size| {
            worth(address, accessors, types, integer, real, null, string, size)
        };
        // Integers: types 1 and 2.
        assert_eq!(w(&[1], 0, 0.0, true, 0, 0), 0);
        assert_eq!(w(&[1], 5, 0.0, true, 0, 0), 1);
        assert_eq!(w(&[2], 3, 0.0, true, 0, 0), 1);
        // Reals: 0 and -0 are not worth writing, a NaN is.
        assert_eq!(w(&[3], 0, 0.0, true, 0, 0), 0);
        assert_eq!(w(&[3], 0, -0.0, true, 0, 0), 0);
        assert_eq!(w(&[3], 0, 0.5, true, 0, 0), 1);
        assert_eq!(w(&[3], 0, f32::NAN, true, 0, 0), 1);
        // Vectors always; type 5 and the unlisted types never.
        for kind in [4, 6, 7] {
            assert_eq!(w(&[kind], 0, 0.0, true, 0, 0), 1, "type {kind}");
        }
        for kind in [0, 5, 11, 15] {
            assert_eq!(w(&[kind], 9, 1.0, false, 9, 9), 0, "type {kind}");
        }
        // Objects: non-null only.
        for kind in [8, 9] {
            assert_eq!(w(&[kind], 0, 0.0, true, 0, 0), 0);
            assert_eq!(w(&[kind], 0, 0.0, false, 0, 0), 1);
        }
        // Strings: non-null only.
        assert_eq!(w(&[10], 0, 0.0, true, 0, 0), 0);
        assert_eq!(w(&[10], 0, 0.0, true, 0x1234, 0), 1);
        // Array types answer the array's size.
        assert_eq!(w(&[0x10], 0, 0.0, true, 0, 7), 7);
        assert_eq!(w(&[0x12], 0, 0.0, true, 0, 3), 3);
        assert_eq!(w(&[0x12], 0, 0.0, true, 0, 0), 0);
        // A type that reads 0xf the first time and 0x10 the second takes the case-0x10 branch.
        assert_eq!(w(&[0xf, 0x10], 0, 0.0, true, 0, 4), 1);
        assert_eq!(w(&[0xf, 0x10], 0, 0.0, true, 0, 0), 0);
        // A negative type is below 0x10 as a signed number, above it as the table index.
        assert_eq!(w(&[0xffff_ffff], 1, 1.0, false, 1, 1), 0);
    }

    #[test]
    fn worth_writing_an_object_value_goes_by_its_type() {
        check_worth(0x00db_e980, &OBJECT_VALUE);
    }

    #[test]
    fn worth_writing_an_array_value_goes_by_its_type() {
        check_worth(0x00db_eab0, &ARRAY_VALUE);
    }

    /// A cache with a listener, one cached object (handle `0x30`, implementation `0xabc`).
    fn cached_cache(e: &mut Engine, blocks: u32, listener: u32) -> u32 {
        let cache = e.mem.alloc(0x1c);
        e.mem.set_u32(cache, listener);
        let handles = cache + 4;
        e.mem.set_i32(handles + 8, -1);
        e.call(0x00db_e810, &args![handles, blocks, 0x30u32, 0u32, 1u32]);
        let objects = e.mem.alloc(8);
        let cached = e.mem.alloc(4);
        e.mem.set_u32(cached, 0xabc);
        e.mem.set_u32(objects + 4, cached);
        e.mem.set_u32(cache + 0x10, objects);
        e.mem.set_u32(cache + 0x14, 2);
        e.mem.set_u32(cache + 0x18, 2);
        cache
    }

    /// Doubles for the `hkDataObject` handle functions on one-word handles holding the
    /// implementation: the handle words are (implementation, 0).
    fn install_object_handles(e: &mut Engine) {
        e.register(DATA_OBJECT_IS_NULL, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
        });
        e.register(DATA_OBJECT_GET_HANDLE, |e, a| {
            let implementation = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], implementation);
            e.mem.set_u32(a[1] + 4, 0);
            ret(a[1])
        });
        e.register(DATA_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(DATA_OBJECT_CONSTRUCT_COPY, |e, a| {
            let implementation = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], implementation);
            ret(a[0])
        });
        e.register(DATA_OBJECT_DESTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn a_cached_object_is_rebuilt_from_the_cached_implementation() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        install_object_handles(&mut e);
        let cache = cached_cache(&mut e, blocks, 0x77);
        let out = e.mem.alloc(4);
        let object = e.mem.alloc(4);
        // Cached handle: the cached object's implementation, through the constructor.
        e.mem.set_u32(object, 0x30);
        let got = e.call(0x00db_ebe0, &args![cache, out, object]).u32();
        assert_eq!(got, out);
        assert_eq!(e.mem.u32(out), 0xabc);
        assert_eq!(calls(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0xabc]]);
        // A handle that is not cached, a null object and no listener: a copy of the object.
        e.mem.set_u32(object, 0x31);
        e.call(0x00db_ebe0, &args![cache, out, object]);
        assert_eq!(e.mem.u32(out), 0x31);
        e.mem.set_u32(object, 0);
        e.call(0x00db_ebe0, &args![cache, out, object]);
        assert_eq!(e.mem.u32(out), 0);
        e.mem.set_u32(object, 0x30);
        e.mem.set_u32(cache, 0);
        e.call(0x00db_ebe0, &args![cache, out, object]);
        assert_eq!(e.mem.u32(out), 0x30);
        assert_eq!(calls(&e, DATA_OBJECT_CONSTRUCT_COPY).len(), 3);
    }

    /// A listener whose slot `0xc` makes the replacement implementation `0x99`; `0xdbaf90`
    /// answers `accepted`.
    fn install_listener(e: &mut Engine, accepted: bool) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_0300);
        e.register(0x0900_0300, |e, a| {
            e.mem.set_u32(a[1], 0x99);
            Ret::default()
        });
        let listener = e.mem.alloc(8);
        e.mem.set_u32(listener, vtable);
        e.register_double(DATA_OBJECT_ACCEPTED, move |_, _| ret(u32::from(accepted)));
        listener
    }

    fn empty_cache(e: &mut Engine, listener: u32) -> u32 {
        let cache = e.mem.alloc(0x1c);
        e.call(0x00dc_0380, &args![cache, listener]);
        cache
    }

    #[test]
    fn the_listener_replacement_is_cached_when_accepted() {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        install_object_handles(&mut e);
        let listener = install_listener(&mut e, true);
        let cache = empty_cache(&mut e, listener);
        let out = e.mem.alloc(4);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x42);
        e.call(0x00db_edd0, &args![cache, out, object]);
        assert_eq!(e.mem.u32(out), 0x99, "the replacement");
        // The handle went into the map with index 0 and a heap copy of the replacement into the
        // object array; the array grew once.
        let found = e
            .call(
                0x00db_e8c0,
                &args![cache + 4, 0x42u32, 0u32, 0xffff_ffffu32],
            )
            .u32();
        assert_eq!(found, 0);
        assert_eq!(e.mem.u32(cache + 0x14), 1);
        let element = e.mem.u32(e.mem.u32(cache + 0x10));
        assert_eq!(e.mem.u32(element), 0x99);
        assert_eq!(calls(&e, ARRAY_GROW).len(), 1);
        assert_eq!(calls(&e, 0x0900_0300).len(), 1);
        // The next time the handle is found: no second listener call, same result.
        e.mem.set_u32(out, 0);
        e.call(0x00db_edd0, &args![cache, out, object]);
        assert_eq!(e.mem.u32(out), 0x99);
        assert_eq!(calls(&e, 0x0900_0300).len(), 1);
    }

    #[test]
    fn a_rejected_replacement_is_dropped_and_the_object_copied() {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        install_object_handles(&mut e);
        let listener = install_listener(&mut e, false);
        let cache = empty_cache(&mut e, listener);
        let out = e.mem.alloc(4);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x42);
        e.call(0x00db_edd0, &args![cache, out, object]);
        assert_eq!(e.mem.u32(out), 0x42);
        assert_eq!(e.mem.u32(cache + 0x14), 0);
        // The replacement temporary was destroyed.
        assert_eq!(calls(&e, DATA_OBJECT_DESTRUCT).len(), 1);
        // No listener, or a null object: a copy as well, the listener not asked.
        e.mem.set_u32(cache, 0);
        e.call(0x00db_edd0, &args![cache, out, object]);
        assert_eq!(calls(&e, 0x0900_0300).len(), 1);
    }

    #[test]
    fn write_string_numbers_each_string_once() {
        let mut e = engine();
        install_object_handles(&mut e);
        let stream = install_archive(&mut e);
        let numbers: Shared<HashMap<u32, u32>> = Rc::new(RefCell::new(HashMap::new()));
        let lookup = Rc::clone(&numbers);
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |_, a| {
            ret(lookup.borrow().get(&a[1]).copied().unwrap_or(a[2]))
        });
        let insert = Rc::clone(&numbers);
        e.register_double(STRING_MAP_INSERT, move |_, a| {
            insert.borrow_mut().insert(a[1], a[2]);
            Ret::default()
        });
        e.register(STRING_LENGTH, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        let writer = e.mem.alloc(0x74);
        e.mem.set_u32(writer + 0x14, 4);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abc");
        // The null string is number -1.
        e.call(0x00db_ed30, &args![writer, 0u32]);
        assert_eq!(*stream.borrow(), vec![3]);
        // A new string: its length, its bytes, and the number one above the map's count.
        stream.borrow_mut().clear();
        e.call(0x00db_ed30, &args![writer, text]);
        assert_eq!(*stream.borrow(), vec![6, b'a', b'b', b'c']);
        assert_eq!(
            calls(&e, STRING_MAP_INSERT),
            vec![vec![writer + 0x10, text, 5]]
        );
        // A string written before: its number negated (-3 is (3 * 2) | 1).
        stream.borrow_mut().clear();
        numbers.borrow_mut().insert(text, 3);
        e.call(0x00db_ed30, &args![writer, text]);
        assert_eq!(*stream.borrow(), vec![7]);
    }

    #[test]
    fn member_flags_are_rounded_to_whole_bytes() {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        let flags = e.mem.alloc(12);
        let got = e.call(0x00db_ef20, &args![flags, 10i32]).u32();
        assert_eq!(got, flags);
        assert_eq!(e.mem.i32(flags + 4), 10);
        let data = e.mem.u32(flags);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(flags + 8), 16);
        // One reserve for 16 flags, and the 16 flags cleared.
        assert_eq!(calls(&e, ARRAY_RESERVE).len(), 1);
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][2..], [16, 1]);
        assert_eq!(calls(&e, MEM_SET), vec![vec![data, 0, 16]]);
        // Nothing to reserve or clear for no flags.
        let empty = e.mem.alloc(12);
        e.call(0x00db_ef20, &args![empty, 0i32]);
        assert_eq!(e.mem.i32(empty + 4), 0);
        assert_eq!(calls(&e, ARRAY_RESERVE).len(), 1);
    }

    fn flag_array(e: &mut Engine, flags: &[u8]) -> u32 {
        let array = e.mem.alloc(12);
        e.call(0x00db_ef20, &args![array, flags.len() as i32]);
        let data = e.mem.u32(array);
        e.mem.write(data, flags);
        array
    }

    #[test]
    fn flags_unpack_from_bytes_lowest_bit_first() {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        let array = flag_array(&mut e, &[0; 10]);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[0xa5, 0xff]);
        e.call(0x00db_efc0, &args![array, source, 2i32]);
        let data = e.mem.u32(array);
        assert_eq!(
            e.mem.bytes(data, 16),
            vec![1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        // The size is what it was.
        assert_eq!(e.mem.i32(array + 4), 10);
        // Only the first byte when the count is 1; the rest stay as they were.
        let array = flag_array(&mut e, &[0; 10]);
        e.call(0x00db_efc0, &args![array, source, 1i32]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.bytes(data, 10), vec![1, 0, 1, 0, 0, 1, 0, 1, 0, 0]);
    }

    #[test]
    fn flags_pack_into_bytes_and_combine_whole_flag_bytes() {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        let array = flag_array(&mut e, &[1, 1, 0, 0, 0, 0, 0, 1, 1, 0]);
        let out = e.mem.alloc(8);
        e.call(0x00db_f0d0, &args![array, out]);
        assert_eq!(e.mem.bytes(out, 2), vec![0x83, 0x01]);
        assert_eq!(e.mem.i32(array + 4), 10, "size restored");
        // A flag byte above 1 spills into its neighbours: bit 0 given 3, bit 1 given 3 (6).
        let array = flag_array(&mut e, &[3, 0, 0, 0, 0, 0, 0, 0]);
        e.call(0x00db_f0d0, &args![array, out]);
        assert_eq!(e.mem.u8(out), 3);
        let array = flag_array(&mut e, &[0, 3, 0, 0, 0, 0, 0, 0]);
        e.call(0x00db_f0d0, &args![array, out]);
        assert_eq!(e.mem.u8(out), 6);
        // The 8-bit arithmetic wraps: a 2 in bit 7 is shifted out.
        let array = flag_array(&mut e, &[0, 0, 0, 0, 0, 0, 0, 2]);
        e.call(0x00db_f0d0, &args![array, out]);
        assert_eq!(e.mem.u8(out), 0);
        // Nothing to pack for no flags.
        e.mem.set_u8(out, 0x55);
        let array = flag_array(&mut e, &[]);
        e.call(0x00db_f0d0, &args![array, out]);
        assert_eq!(e.mem.u8(out), 0x55);
    }

    // ---- A small data-object world the writer's functions walk ----

    /// A value of an object member or an array element.
    #[derive(Clone)]
    enum Val {
        Int(i32),
        Real(f32),
        Text(u32),
        Object(u32),
        Array(u32),
        /// A pointer to four floats in game memory.
        Vector(u32),
    }

    struct MemberModel {
        name: u32,
        type_word: u32,
        class: u32,
        tuple: u32,
    }

    struct ClassModel {
        name: u32,
        parent: u32,
        version: i32,
        members: Vec<MemberModel>,
    }

    struct ObjectModel {
        class: u32,
        values: Vec<Val>,
    }

    struct ArrayModel {
        kind: u32,
        class: u32,
        elements: Vec<Val>,
    }

    /// Classes, objects and arrays; an id is the index plus one (0 is null).
    #[derive(Default)]
    struct Model {
        classes: Vec<ClassModel>,
        objects: Vec<ObjectModel>,
        arrays: Vec<ArrayModel>,
    }

    impl Model {
        fn member_index(&self, object: u32, name: u32) -> u32 {
            let class = self.objects[object as usize - 1].class;
            let members = &self.classes[class as usize - 1].members;
            members.iter().position(|m| m.name == name).unwrap() as u32
        }

        fn value(&self, container: u32, index: u32, in_array: bool) -> Val {
            if in_array {
                self.arrays[container as usize - 1].elements[index as usize].clone()
            } else {
                self.objects[container as usize - 1].values[index as usize].clone()
            }
        }
    }

    fn put_member_info(e: &mut Engine, member: &MemberModel, record: u32) {
        e.mem.set_u32(record, member.name);
        e.mem.set_u32(record + 8, member.type_word);
        e.mem.set_u32(record + 0xc, member.class);
        e.mem.set_u32(record + 0x10, member.tuple);
    }

    /// Doubles for the `hkDataObject`, `hkDataClass`, `hkDataArray` and `Value` functions over
    /// `model`. A handle is a word holding the implementation id; a `Value` is two words, the
    /// container id and the member or element index.
    fn install_model(e: &mut Engine, model: &Shared<Model>) {
        install_object_handles(e);
        let m = Rc::clone(model);
        e.register_double(DATA_OBJECT_GET_CLASS, move |e, a| {
            let id = e.mem.u32(a[0]);
            let class = m.borrow().objects[id as usize - 1].class;
            e.mem.set_u32(a[1], class);
            ret(a[1])
        });
        for address in [0x00db_a870, DATA_OBJECT_MEMBER_VALUE] {
            let m = Rc::clone(model);
            e.register_double(address, move |e, a| {
                let id = e.mem.u32(a[0]);
                let index = m.borrow().member_index(id, a[2]);
                e.mem.set_u32(a[1], id);
                e.mem.set_u32(a[1] + 4, index);
                ret(a[1])
            });
        }
        let m = Rc::clone(model);
        let member_count = move |e: &Engine, object: u32| -> u32 {
            let id = e.mem.u32(object);
            let model = m.borrow();
            let class = model.objects[id as usize - 1].class;
            model.classes[class as usize - 1].members.len() as u32
        };
        e.register(DATA_OBJECT_ITERATOR_START, |_, _| ret(0));
        let counter = member_count.clone();
        e.register_double(DATA_OBJECT_ITERATOR_IS_VALID, move |e, a| {
            ret(u32::from(a[1] < counter(e, a[0])))
        });
        e.register(DATA_OBJECT_ITERATOR_NEXT, |_, a| ret(a[1] + 1));
        e.register(DATA_OBJECT_ITERATOR_MEMBER, |_, a| ret(a[1]));
        e.register(DATA_OBJECT_ITERATOR_VALUE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], id);
            e.mem.set_u32(a[1] + 4, a[2]);
            ret(a[1])
        });
        // The `Value` accessors of objects and of arrays.
        for in_array in [false, true] {
            let (get_type, as_int, as_real, as_object, as_string, as_array, as_vec) = if in_array {
                (
                    ARRAY_VALUE_GET_TYPE,
                    ARRAY_VALUE_AS_INT,
                    ARRAY_VALUE_AS_REAL,
                    ARRAY_VALUE_AS_OBJECT,
                    ARRAY_VALUE_AS_STRING,
                    ARRAY_VALUE_AS_ARRAY,
                    ARRAY_VALUE_AS_VEC,
                )
            } else {
                (
                    OBJECT_VALUE_GET_TYPE,
                    OBJECT_VALUE_AS_INT,
                    OBJECT_VALUE_AS_REAL,
                    OBJECT_VALUE_AS_OBJECT,
                    OBJECT_VALUE_AS_STRING,
                    OBJECT_VALUE_AS_ARRAY,
                    OBJECT_VALUE_AS_VEC,
                )
            };
            let m = Rc::clone(model);
            e.register_double(get_type, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                let model = m.borrow();
                if in_array {
                    ret(model.arrays[container as usize - 1].kind)
                } else {
                    let class = model.objects[container as usize - 1].class;
                    ret(model.classes[class as usize - 1].members[index as usize].type_word)
                }
            });
            let m = Rc::clone(model);
            e.register_double(as_int, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                match m.borrow().value(container, index, in_array) {
                    Val::Int(number) => ret(number as u32),
                    _ => panic!("not an integer"),
                }
            });
            let m = Rc::clone(model);
            e.register_double(as_real, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                match m.borrow().value(container, index, in_array) {
                    Val::Real(real) => Ret {
                        st0: f64::from(real),
                        ..Ret::default()
                    },
                    _ => panic!("not a real"),
                }
            });
            let m = Rc::clone(model);
            e.register_double(as_string, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                match m.borrow().value(container, index, in_array) {
                    Val::Text(text) => ret(text),
                    _ => panic!("not a string"),
                }
            });
            let m = Rc::clone(model);
            e.register_double(as_object, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                match m.borrow().value(container, index, in_array) {
                    Val::Object(id) => e.mem.set_u32(a[1], id),
                    _ => panic!("not an object"),
                }
                ret(a[1])
            });
            let m = Rc::clone(model);
            e.register_double(as_array, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                match m.borrow().value(container, index, in_array) {
                    Val::Array(id) => e.mem.set_u32(a[1], id),
                    _ => panic!("not an array"),
                }
                ret(a[1])
            });
            let m = Rc::clone(model);
            e.register_double(as_vec, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                match m.borrow().value(container, index, in_array) {
                    Val::Vector(pointer) => ret(pointer),
                    _ => panic!("not a vector"),
                }
            });
        }
        // Classes.
        e.register(DATA_CLASS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(DATA_CLASS_DESTRUCT, |_, _| Ret::default());
        let m = Rc::clone(model);
        e.register_double(DATA_CLASS_GET_NAME, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().classes[id as usize - 1].name)
        });
        let m = Rc::clone(model);
        e.register_double(DATA_CLASS_GET_VERSION, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().classes[id as usize - 1].version as u32)
        });
        let m = Rc::clone(model);
        e.register_double(DATA_CLASS_GET_PARENT, move |e, a| {
            let id = e.mem.u32(a[0]);
            let parent = m.borrow().classes[id as usize - 1].parent;
            e.mem.set_u32(a[1], parent);
            ret(a[1])
        });
        for address in [
            DATA_CLASS_GET_NUM_DECLARED_MEMBERS,
            DATA_CLASS_GET_NUM_MEMBERS,
        ] {
            let m = Rc::clone(model);
            e.register_double(address, move |e, a| {
                let id = e.mem.u32(a[0]);
                ret(m.borrow().classes[id as usize - 1].members.len() as u32)
            });
        }
        for address in [
            DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO,
            DATA_CLASS_GET_ALL_MEMBER_INFO,
        ] {
            let m = Rc::clone(model);
            e.register_double(address, move |e, a| {
                let id = e.mem.u32(a[0]);
                let data = e.mem.u32(a[1]);
                let model = m.borrow();
                for (n, member) in model.classes[id as usize - 1].members.iter().enumerate() {
                    put_member_info(e, member, data + 0x14 * n as u32);
                }
                Ret::default()
            });
        }
        let m = Rc::clone(model);
        e.register_double(DATA_CLASS_MEMBER_AT, move |e, a| {
            let id = e.mem.u32(a[0]);
            let model = m.borrow();
            put_member_info(
                e,
                &model.classes[id as usize - 1].members[a[1] as usize],
                a[2],
            );
            Ret::default()
        });
        e.register(DATA_CLASS_MEMBER_INDEX, |_, a| ret(a[1]));
        // Arrays.
        let m = Rc::clone(model);
        e.register_double(DATA_ARRAY_GET_SIZE, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().arrays[id as usize - 1].elements.len() as u32)
        });
        let m = Rc::clone(model);
        e.register_double(DATA_ARRAY_GET_TYPE, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().arrays[id as usize - 1].kind)
        });
        let m = Rc::clone(model);
        e.register_double(DATA_ARRAY_GET_CLASS, move |e, a| {
            let id = e.mem.u32(a[0]);
            let class = m.borrow().arrays[id as usize - 1].class;
            e.mem.set_u32(a[1], class);
            ret(a[1])
        });
        e.register(DATA_ARRAY_DESTRUCT, |_, _| Ret::default());
        for address in [DATA_ARRAY_ELEMENT, DATA_ARRAY_ELEMENT_OV2] {
            e.register(address, |e, a| {
                let id = e.mem.u32(a[0]);
                e.mem.set_u32(a[1], id);
                e.mem.set_u32(a[1] + 4, a[2]);
                ret(a[1])
            });
        }
        // The member of every element of a struct array, as an array of its own: its kind is
        // the member's type word; the id of the new array is returned in `out`.
        let m = Rc::clone(model);
        e.register_double(DATA_ARRAY_SWIZZLE_OBJECT_MEMBER, move |e, a| {
            let id = e.mem.u32(a[0]);
            let mut model = m.borrow_mut();
            let mut elements = Vec::new();
            let mut kind = 0;
            for element in model.arrays[id as usize - 1].elements.clone() {
                let Val::Object(object) = element else {
                    panic!("not an object")
                };
                let index = model.member_index(object, a[2]);
                let class = model.objects[object as usize - 1].class;
                kind = model.classes[class as usize - 1].members[index as usize].type_word;
                elements.push(model.objects[object as usize - 1].values[index as usize].clone());
            }
            model.arrays.push(ArrayModel {
                kind,
                class: 0,
                elements,
            });
            e.mem.set_u32(a[1], model.arrays.len() as u32);
            ret(a[1])
        });
    }

    /// The writer under test with its archive, string maps and a model.
    struct Fixture {
        e: Engine,
        model: Shared<Model>,
        writer: u32,
        stream: Shared<Vec<u8>>,
    }

    fn fixture() -> Fixture {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        let stream = install_archive(&mut e);
        let model: Shared<Model> = Rc::new(RefCell::new(Model::default()));
        install_model(&mut e, &model);
        // The string maps: a number per key; an insert raises the element count at `+4`.
        let numbers: Shared<HashMap<(u32, u32), u32>> = Rc::new(RefCell::new(HashMap::new()));
        let lookup = Rc::clone(&numbers);
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |_, a| {
            ret(lookup.borrow().get(&(a[0], a[1])).copied().unwrap_or(a[2]))
        });
        let insert = Rc::clone(&numbers);
        e.register_double(STRING_MAP_INSERT, move |e, a| {
            insert.borrow_mut().insert((a[0], a[1]), a[2]);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        e.register(STRING_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        e.register(STRING_MAP_DESTRUCT, |_, _| Ret::default());
        e.register(ARCHIVE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ARCHIVE_DESTRUCT, |_, _| Ret::default());
        e.register(STRING_LENGTH, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        for (n, value) in [0u32, 0xffff_ffff, 0xffff_ffff, 1, 0, 0]
            .into_iter()
            .enumerate()
        {
            e.set_global(DEFAULT_OBJECT_RECORD + 4 * n as u32, value);
        }
        let stream_object = e.mem.alloc(8);
        let writer = e.mem.alloc(0x74);
        e.call(0x00dc_0500, &args![writer, stream_object, 0u32]);
        e.call_log = Some(Vec::new());
        stream.borrow_mut().clear();
        Fixture {
            e,
            model,
            writer,
            stream,
        }
    }

    impl Fixture {
        fn text(&mut self, text: &str) -> u32 {
            let block = self.e.mem.alloc(text.len() as u32 + 1);
            self.e.mem.set_cstr(block, text.as_bytes());
            block
        }

        /// A handle block holding `id`.
        fn handle(&mut self, id: u32) -> u32 {
            let block = self.e.mem.alloc(8);
            self.e.mem.set_u32(block, id);
            block
        }

        fn member(&mut self, name: &str, type_word: u32, class: u32, tuple: u32) -> MemberModel {
            MemberModel {
                name: self.text(name),
                type_word,
                class,
                tuple,
            }
        }

        fn class(
            &mut self,
            name: &str,
            parent: u32,
            version: i32,
            members: Vec<MemberModel>,
        ) -> u32 {
            let name = self.text(name);
            let mut model = self.model.borrow_mut();
            model.classes.push(ClassModel {
                name,
                parent,
                version,
                members,
            });
            model.classes.len() as u32
        }

        fn object(&mut self, class: u32, values: Vec<Val>) -> u32 {
            let mut model = self.model.borrow_mut();
            model.objects.push(ObjectModel { class, values });
            model.objects.len() as u32
        }

        fn array(&mut self, kind: u32, class: u32, elements: Vec<Val>) -> u32 {
            let mut model = self.model.borrow_mut();
            model.arrays.push(ArrayModel {
                kind,
                class,
                elements,
            });
            model.arrays.len() as u32
        }

        /// The bytes written since the last call.
        fn written(&mut self) -> Vec<u8> {
            std::mem::take(&mut *self.stream.borrow_mut())
        }
    }

    #[test]
    fn a_class_is_written_once_after_its_parent_and_numbered() {
        let mut f = fixture();
        let parent = f.class("A", 0, 1, vec![]);
        let x = f.member("x", 1, 0, 0);
        let o = f.member("o", 8, parent, 0);
        let t = f.member("t", 0x21, 0, 4);
        let child = f.class("B", parent, 3, vec![x, o, t]);
        let handle = f.handle(child);
        let number = f.e.call(0x00db_f1a0, &args![f.writer, handle]).i32();
        // The classes map held the empty string, so A is 1 and B is 2.
        assert_eq!(number, 2);
        assert_eq!(
            f.written(),
            vec![
                // A: tag 2, its name (new: length 1, "A"), version 1, parent 0, no members.
                4, 2, b'A', 2, 0, 0, //
                // B: tag 2, name, version 3, parent 1, three members.
                4, 2, b'B', 6, 2, 6, //
                // x: name, type 1.
                2, b'x', 2, //
                // o: name, type 8 (object), its class's name: "A" again, as -2.
                2, b'o', 16, 5, //
                // t: name, type 0x21 (a tuple), tuple size 4.
                2, b't', 0x42, 8,
            ]
        );
        // Written before: only the number comes back, nothing is written.
        let again = f.e.call(0x00db_f1a0, &args![f.writer, handle]).i32();
        assert_eq!(again, 2);
        assert!(f.written().is_empty());
        // A null class is number 0.
        let null = f.handle(0);
        assert_eq!(f.e.call(0x00db_f1a0, &args![f.writer, null]).i32(), 0);
    }

    #[test]
    fn saving_an_object_writes_the_magic_the_class_and_the_flagged_members() {
        let mut f = fixture();
        let n = f.member("n", 1, 0, 0);
        let z = f.member("z", 1, 0, 0);
        let r = f.member("r", 3, 0, 0);
        let class = f.class("C", 0, 1, vec![n, z, r]);
        let object = f.object(class, vec![Val::Int(5), Val::Int(0), Val::Real(2.5)]);
        let handle = f.handle(object);
        let stream_vtable = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(stream_vtable + 0x14, 0x0900_0400);
        f.e.register(0x0900_0400, |_, _| Ret::default());
        let stream_object = f.e.mem.alloc(8);
        f.e.mem.set_u32(stream_object, stream_vtable);
        let result =
            f.e.call(0x00dc_05b0, &args![VCALL_THIS, handle, stream_object, 0u32])
                .u32();
        assert_eq!(result, 0);
        assert_eq!(
            f.written(),
            vec![
                // The magic words and the numbers 1 and 0.
                0x1e, 0x0d, 0xb0, 0xca, 0xce, 0xfa, 0x11, 0xd0, 2, 0, //
                // The class, written by the first pass: tag 2, "C", version 1, parent 0, three
                // members (name and type each).
                4, 2, b'C', 2, 0, 6, 2, b'n', 2, 2, b'z', 2, 2, b'r', 6, //
                // The object: tag 3 (met once), class 1, the flags (members 0 and 2) as one
                // byte, then member 0 as a byte and member 2 as a float (2.5).
                6, 2, 5, 5, 0, 0, 0x20, 0x40,
            ]
        );
        assert_eq!(calls(&f.e, 0x0900_0400).len(), 1, "the stream is flushed");
        assert_eq!(calls(&f.e, ARCHIVE_DESTRUCT).len(), 1);
        assert_eq!(calls(&f.e, STRING_MAP_DESTRUCT).len(), 2);
    }

    #[test]
    fn an_object_met_twice_is_numbered_and_then_referred_to() {
        let mut f = fixture();
        let v = f.member("v", 1, 0, 0);
        let leaf_class = f.class("K", 0, 1, vec![v]);
        let a = f.member("a", 8, leaf_class, 0);
        let b = f.member("b", 8, leaf_class, 0);
        let parent_class = f.class("P", 0, 1, vec![a, b]);
        let leaf = f.object(leaf_class, vec![Val::Int(7)]);
        let parent = f.object(parent_class, vec![Val::Object(leaf), Val::Object(leaf)]);
        let handle = f.handle(parent);
        let writer = f.writer;
        // The first pass registers both objects; the leaf is met twice.
        f.e.call(0x00db_f450, &args![writer, handle, 0u32]);
        // The first pass writes the classes: P with its two object members (the class of the
        // member named by its string, which is a number the second time), then K.
        assert_eq!(
            f.written(),
            vec![
                4, 2, b'P', 2, 0, 4, 2, b'a', 16, 2, b'K', 2, b'b', 16, 9, //
                4, 9, 2, 0, 2, 2, b'v', 2,
            ]
        );
        let index =
            f.e.call(
                0x00db_e8c0,
                &args![writer + 0x34, leaf, 0u32, 0xffff_ffffu32],
            )
            .u32();
        let records = f.e.mem.u32(writer + 0x4c);
        assert_eq!(f.e.mem.u32(writer + 0x50), 2, "two records");
        let record = records + 0x18 * index;
        assert_eq!(f.e.mem.u32(record + 0xc), 2, "the leaf was met twice");
        assert_eq!(
            f.e.mem.u32(record + 0x10),
            leaf,
            "the record holds the handle"
        );
        // The second pass writes the parent (tag 3), the leaf once in full (tag 4, number 0)
        // and the second reference as tag 5 and the number.
        f.e.call(0x00dc_00b0, &args![writer, handle]);
        assert_eq!(f.written(), vec![6, 2, 3, 8, 4, 1, 7, 10, 0]);
        assert_eq!(
            f.e.mem.u32(writer + 0x30),
            1,
            "one object number was given out"
        );
    }

    /// Four or sixteen floats in game memory.
    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (n, value) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * n as u32, *value);
        }
        block
    }

    /// Writes the first `count` elements of `array` and returns the bytes.
    fn array_items(f: &mut Fixture, array: u32, count: i32) -> Vec<u8> {
        let handle = f.handle(array);
        f.e.call(0x00db_f930, &args![f.writer, handle, count]);
        f.written()
    }

    #[test]
    fn byte_integer_and_float_arrays_write_each_element() {
        let mut f = fixture();
        let bytes = f.array(1, 0, vec![Val::Int(3), Val::Int(0x1ff)]);
        assert_eq!(array_items(&mut f, bytes, 2), vec![3, 0xff]);
        let integers = f.array(2, 0, vec![Val::Int(1), Val::Int(-1), Val::Int(300)]);
        assert_eq!(array_items(&mut f, integers, 3), vec![2, 3, 0xd8, 4]);
        let reals = f.array(3, 0, vec![Val::Real(1.5), Val::Real(-2.0)]);
        assert_eq!(
            array_items(&mut f, reals, 2),
            [1.5f32.to_le_bytes(), (-2.0f32).to_le_bytes()].concat()
        );
        // Only `count` elements are written; a count of 0 or less writes none.
        assert_eq!(array_items(&mut f, bytes, 1), vec![3]);
        assert!(array_items(&mut f, bytes, 0).is_empty());
        assert!(array_items(&mut f, bytes, -1).is_empty());
        // Types 0, 5 and above 10 write nothing.
        for kind in [0, 5, 11, 0x10] {
            let nothing = f.array(kind, 0, vec![Val::Int(1)]);
            assert!(array_items(&mut f, nothing, 1).is_empty(), "type {kind}");
        }
    }

    #[test]
    fn vector_arrays_are_three_floats_wide_unless_a_fourth_is_used() {
        let mut f = fixture();
        let a = floats(&mut f.e, &[1.0, 2.0, 3.0, 0.0]);
        let b = floats(&mut f.e, &[4.0, 5.0, 6.0, 0.0]);
        let narrow = f.array(4, 0, vec![Val::Vector(a), Val::Vector(b)]);
        let mut expected = vec![6u8];
        for value in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0] {
            expected.extend(value.to_le_bytes());
        }
        assert_eq!(array_items(&mut f, narrow, 2), expected);
        // A fourth float that is not 0 (a NaN counts) makes every vector four floats wide.
        let c = floats(&mut f.e, &[7.0, 8.0, 9.0, f32::NAN]);
        let wide = f.array(4, 0, vec![Val::Vector(a), Val::Vector(c)]);
        let mut expected = vec![8u8];
        for value in [1.0f32, 2.0, 3.0, 0.0, 7.0, 8.0, 9.0] {
            expected.extend(value.to_le_bytes());
        }
        expected.extend(f32::NAN.to_le_bytes());
        assert_eq!(array_items(&mut f, wide, 2), expected);
        // Types 6 and 7 write `type * 4 - 12` floats per element.
        let sixteen: Vec<f32> = (0..16).map(|n| n as f32).collect();
        let matrix = floats(&mut f.e, &sixteen);
        for (kind, count) in [(6u32, 12usize), (7, 16)] {
            let array = f.array(kind, 0, vec![Val::Vector(matrix)]);
            let expected: Vec<u8> = sixteen[..count]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            assert_eq!(array_items(&mut f, array, 1), expected, "type {kind}");
        }
    }

    #[test]
    fn string_object_and_tuple_arrays() {
        let mut f = fixture();
        let hi = f.text("hi");
        let strings = f.array(10, 0, vec![Val::Text(hi), Val::Text(0)]);
        // "hi" is new (length 2, the bytes), then the null string (number -1).
        assert_eq!(array_items(&mut f, strings, 2), vec![4, b'h', b'i', 3]);
        // A null object is the tag 6.
        let objects = f.array(8, 0, vec![Val::Object(0)]);
        assert_eq!(array_items(&mut f, objects, 1), vec![12]);
        // A tuple array writes each element's own array: here two byte arrays.
        let first = f.array(1, 0, vec![Val::Int(9), Val::Int(8)]);
        let second = f.array(1, 0, vec![Val::Int(7)]);
        let tuples = f.array(0x21, 0, vec![Val::Array(first), Val::Array(second)]);
        // Each element array is written with its own size.
        assert_eq!(array_items(&mut f, tuples, 2), vec![9, 8, 7]);
    }

    #[test]
    fn struct_arrays_write_the_flags_then_the_arrays_of_the_flagged_members() {
        let mut f = fixture();
        let i = f.member("i", 1, 0, 0);
        let j = f.member("j", 1, 0, 0);
        let k = f.member("k", 0, 0, 0);
        let class = f.class("S", 0, 1, vec![i, j, k]);
        let first = f.object(class, vec![Val::Int(5), Val::Int(0), Val::Int(0)]);
        let second = f.object(class, vec![Val::Int(0), Val::Int(0), Val::Int(0)]);
        let structs = f.array(9, class, vec![Val::Object(first), Val::Object(second)]);
        // Member i holds something (5), j never does and k has no type: the flag byte is 0b1,
        // then the array of i: [5, 0].
        assert_eq!(array_items(&mut f, structs, 2), vec![1, 5, 0]);
        // With a zero i everywhere only the flag byte (no members) is written.
        let empty_first = f.object(class, vec![Val::Int(0), Val::Int(0), Val::Int(0)]);
        let nothing = f.array(
            9,
            class,
            vec![Val::Object(empty_first), Val::Object(second)],
        );
        assert_eq!(array_items(&mut f, nothing, 2), vec![0]);
    }

    /// An object with one member of each of the value types; returns (the object's handle, the
    /// member records block).
    fn value_object(f: &mut Fixture) -> (u32, u32) {
        let hi = f.text("v");
        let vector = floats(&mut f.e, &[1.0, 2.0, 3.0, 4.0]);
        let matrix = floats(&mut f.e, &(0..16).map(|n| n as f32).collect::<Vec<_>>());
        let ints = f.array(1, 0, vec![Val::Int(1), Val::Int(2)]);
        let tuple = f.array(1, 0, vec![Val::Int(6), Val::Int(5)]);
        let kinds = [
            (1u32, Val::Int(7)),
            (2, Val::Int(-3)),
            (3, Val::Real(0.5)),
            (4, Val::Vector(vector)),
            (7, Val::Vector(matrix)),
            (8, Val::Object(0)),
            (10, Val::Text(hi)),
            (0x11, Val::Array(ints)),
            (0x21, Val::Array(tuple)),
            (5, Val::Int(1)),
        ];
        let mut members = Vec::new();
        let mut values = Vec::new();
        for (n, (type_word, value)) in kinds.into_iter().enumerate() {
            members.push(f.member(&format!("m{n}"), type_word, 0, 0));
            values.push(value);
        }
        let class = f.class("V", 0, 1, members);
        let object = f.object(class, values);
        let records = f.e.mem.alloc(0x14 * 10);
        (object, records)
    }

    /// Writes member `index` of the object as a binary value, with a member record whose class
    /// is `member_class`.
    fn binary_value(f: &mut Fixture, object: u32, index: u32, member_class: u32) -> Vec<u8> {
        let value = f.e.mem.alloc(8);
        f.e.mem.set_u32(value, object);
        f.e.mem.set_u32(value + 4, index);
        let member = f.e.mem.alloc(0x14);
        f.e.mem.set_u32(member + 0xc, member_class);
        f.e.call(0x00db_fef0, &args![f.writer, value, member]);
        f.written()
    }

    #[test]
    fn binary_values_are_written_by_type() {
        let mut f = fixture();
        let (object, _) = value_object(&mut f);
        // 1: a byte; 2: an integer (-3 is 7); 3: a float.
        assert_eq!(binary_value(&mut f, object, 0, 0), vec![7]);
        assert_eq!(binary_value(&mut f, object, 1, 0), vec![7]);
        assert_eq!(
            binary_value(&mut f, object, 2, 0),
            0.5f32.to_le_bytes().to_vec()
        );
        // 4 and 7: `type * 4 - 12` floats (4 and 16).
        let four: Vec<u8> = [1.0f32, 2.0, 3.0, 4.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        assert_eq!(binary_value(&mut f, object, 3, 0), four);
        let sixteen: Vec<u8> = (0..16).flat_map(|n| (n as f32).to_le_bytes()).collect();
        assert_eq!(binary_value(&mut f, object, 4, 0), sixteen);
        // 8: an object (here null, the tag 6); 10: a string.
        assert_eq!(binary_value(&mut f, object, 5, 0), vec![12]);
        assert_eq!(binary_value(&mut f, object, 6, 0), vec![2, b'v']);
        // 0x11 (an array): its size, then its items; 0x21 (a tuple): only its items.
        assert_eq!(binary_value(&mut f, object, 7, 0), vec![4, 1, 2]);
        assert_eq!(binary_value(&mut f, object, 8, 0), vec![6, 5]);
        // Type 5 writes nothing.
        assert!(binary_value(&mut f, object, 9, 0).is_empty());
    }

    #[test]
    fn a_struct_array_without_a_member_class_names_its_class() {
        let mut f = fixture();
        let i = f.member("i", 1, 0, 0);
        let class = f.class("S", 0, 1, vec![i]);
        // Written once so that the class has a number (1).
        let handle = f.handle(class);
        f.e.call(0x00db_f1a0, &args![f.writer, handle]);
        f.written();
        let element = f.object(class, vec![Val::Int(4)]);
        let structs = f.array(9, class, vec![Val::Object(element)]);
        let holder_member = f.member("s", 0x19, 0, 0);
        let holder_class = f.class("H", 0, 1, vec![holder_member]);
        let holder = f.object(holder_class, vec![Val::Array(structs)]);
        // The array's size 1, the class number 1, then the items: the flag byte and member i.
        assert_eq!(binary_value(&mut f, holder, 0, 0), vec![2, 2, 1, 4]);
        // With a member class the number is not written.
        assert_eq!(binary_value(&mut f, holder, 0, class), vec![2, 1, 4]);
    }

    #[test]
    fn writing_an_object_with_a_number_refers_to_it() {
        let mut f = fixture();
        let n = f.member("n", 1, 0, 0);
        let class = f.class("C", 0, 1, vec![n]);
        let object = f.object(class, vec![Val::Int(1)]);
        let handle = f.handle(object);
        let writer = f.writer;
        f.e.call(0x00db_f450, &args![writer, handle, 0u32]);
        f.written();
        // The record gets number 3: the tag 5 and the number.
        let index =
            f.e.call(
                0x00db_e8c0,
                &args![writer + 0x34, object, 0u32, 0xffff_ffffu32],
            )
            .u32();
        let record = f.e.mem.u32(writer + 0x4c) + 0x18 * index;
        f.e.mem.set_u32(record + 4, 3);
        f.e.call(0x00dc_00b0, &args![writer, handle]);
        assert_eq!(f.written(), vec![10, 6]);
        // A null object is the tag 6.
        let null = f.handle(0);
        f.e.call(0x00dc_00b0, &args![writer, null]);
        assert_eq!(f.written(), vec![12]);
    }

    #[test]
    fn the_first_pass_registers_objects_and_flags_their_members() {
        let mut f = fixture();
        let n = f.member("n", 1, 0, 0);
        let z = f.member("z", 1, 0, 0);
        let class = f.class("C", 0, 1, vec![n, z]);
        let object = f.object(class, vec![Val::Int(3), Val::Int(0)]);
        let handle = f.handle(object);
        let writer = f.writer;
        f.e.call(0x00db_f450, &args![writer, handle, 0u32]);
        // One record from the default words, holding the object's handle; the record's flag
        // offset is 0 and the flag byte 0b1.
        assert_eq!(f.e.mem.u32(writer + 0x50), 1);
        let record = f.e.mem.u32(writer + 0x4c);
        assert_eq!(
            [
                f.e.mem.u32(record),
                f.e.mem.u32(record + 4),
                f.e.mem.u32(record + 8),
                f.e.mem.u32(record + 0xc),
                f.e.mem.u32(record + 0x10),
                f.e.mem.u32(record + 0x14),
            ],
            [0, 0xffff_ffff, 0, 1, object, 0]
        );
        assert_eq!(f.e.mem.u32(writer + 0x44), 1, "one flag byte");
        assert_eq!(f.e.mem.u8(f.e.mem.u32(writer + 0x40)), 1);
        // A second object adds its flag byte after the first.
        let both = f.object(class, vec![Val::Int(1), Val::Int(1)]);
        let other = f.handle(both);
        f.e.call(0x00db_f450, &args![writer, other, 0u32]);
        let second = f.e.mem.u32(writer + 0x4c) + 0x18;
        assert_eq!(
            f.e.mem.u32(second + 8),
            1,
            "the second flag byte is at offset 1"
        );
        assert_eq!(f.e.mem.u8(f.e.mem.u32(writer + 0x40) + 1), 3);
        // With the flag set nothing is registered, but the class is still written.
        f.written();
        let third = f.object(class, vec![Val::Int(1), Val::Int(1)]);
        let third_handle = f.handle(third);
        f.e.call(0x00db_f450, &args![writer, third_handle, 1u32]);
        assert_eq!(f.e.mem.u32(writer + 0x50), 2);
        assert_eq!(f.e.mem.u32(writer + 0x44), 2);
        // A null object does nothing.
        let null = f.handle(0);
        f.e.call(0x00db_f450, &args![writer, null, 0u32]);
        assert_eq!(f.e.mem.u32(writer + 0x50), 2);
    }

    #[test]
    fn the_first_pass_follows_object_arrays_and_struct_arrays() {
        let mut f = fixture();
        let v = f.member("v", 1, 0, 0);
        let leaf_class = f.class("K", 0, 1, vec![v]);
        let leaf_a = f.object(leaf_class, vec![Val::Int(1)]);
        let leaf_b = f.object(leaf_class, vec![Val::Int(2)]);
        // An object array member (type 0x18) holding both leaves.
        let leaves = f.array(8, 0, vec![Val::Object(leaf_a), Val::Object(leaf_b)]);
        let list = f.member("list", 0x18, leaf_class, 0);
        // A struct array member (0x19): its struct has an object member naming a third leaf.
        let leaf_c = f.object(leaf_class, vec![Val::Int(3)]);
        let link = f.member("link", 8, leaf_class, 0);
        let struct_class = f.class("S", 0, 1, vec![link]);
        let element = f.object(struct_class, vec![Val::Object(leaf_c)]);
        let structs = f.array(9, struct_class, vec![Val::Object(element)]);
        let table = f.member("table", 0x19, struct_class, 0);
        let holder_class = f.class("H", 0, 1, vec![list, table]);
        let holder = f.object(holder_class, vec![Val::Array(leaves), Val::Array(structs)]);
        let handle = f.handle(holder);
        let writer = f.writer;
        f.e.call(0x00db_f450, &args![writer, handle, 0u32]);
        // The holder and the three leaves.
        assert_eq!(f.e.mem.u32(writer + 0x50), 4);
        for leaf in [leaf_a, leaf_b, leaf_c] {
            let found =
                f.e.call(
                    0x00db_e8c0,
                    &args![writer + 0x34, leaf, 0u32, 0xffff_ffffu32],
                )
                .u32();
            assert_ne!(found, 0xffff_ffff, "leaf {leaf}");
        }
    }

    #[test]
    fn the_cache_constructor_empties_the_map_and_the_array() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let cache = e.mem.alloc(0x1c);
        let got = e.call(0x00dc_0380, &args![cache, 0x77u32]).u32();
        assert_eq!(got, cache);
        assert_eq!(e.mem.u32(cache), 0x77);
        // The map reserved for 0 elements: 8 slots from the blocks allocator.
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 4), vec![vec![blocks, 96]]);
        assert_eq!(e.mem.i32(cache + 4 + 8), 7);
        assert_eq!(e.mem.u32(cache + 4 + 4), 0);
        assert_eq!(
            [
                e.mem.u32(cache + 0x10),
                e.mem.u32(cache + 0x14),
                e.mem.u32(cache + 0x18)
            ],
            [0, 0, 0x8000_0000]
        );
    }

    #[test]
    fn the_cache_destructor_destroys_every_cached_object_and_returns_the_storage() {
        let mut e = engine();
        let (growth, blocks) = install_router(&mut e);
        install_arrays(&mut e);
        install_object_handles(&mut e);
        let listener = install_listener(&mut e, true);
        let cache = empty_cache(&mut e, listener);
        let out = e.mem.alloc(4);
        for handle in [0x42u32, 0x43] {
            let object = e.mem.alloc(4);
            e.mem.set_u32(object, handle);
            e.call(0x00db_edd0, &args![cache, out, object]);
        }
        assert_eq!(e.mem.u32(cache + 0x14), 2);
        let data = e.mem.u32(cache + 0x10);
        let capacity = e.mem.u32(cache + 0x18);
        let elements = [e.mem.u32(data), e.mem.u32(data + 4)];
        let table = e.mem.u32(cache + 4);
        let destructs = calls(&e, DATA_OBJECT_DESTRUCT).len();
        e.call(0x00dc_03d0, &args![cache]);
        // Each element: destroyed, then its 4-byte block freed through the blocks allocator.
        let freed: Vec<Vec<u32>> = calls(&e, ALLOCATOR_BLOCKS + 8);
        for element in elements {
            assert!(
                freed.contains(&vec![blocks, element, 4]),
                "element {element:x}"
            );
        }
        assert_eq!(calls(&e, DATA_OBJECT_DESTRUCT).len(), destructs + 2);
        // The array storage (capacity * 4 bytes) through the growth allocator's slot 0x10.
        assert_eq!(
            calls(&e, ALLOCATOR_GROWTH + 0x10),
            vec![vec![growth, data, capacity * 4]]
        );
        assert_eq!(
            [
                e.mem.u32(cache + 0x10),
                e.mem.u32(cache + 0x14),
                e.mem.u32(cache + 0x18)
            ],
            [0, 0, 0x8000_0000]
        );
        // The map's slots go back too (8 slots of 12 bytes).
        assert!(freed.contains(&vec![blocks, table, 96]));
    }

    #[test]
    fn the_writer_constructor_builds_the_parts_and_the_destructor_takes_them_down() {
        let mut f = fixture();
        let writer = f.writer;
        // The constructor ran in `fixture`; its log was cleared, so build another writer.
        let stream = f.e.mem.alloc(8);
        let other = f.e.mem.alloc(0x74);
        let got = f.e.call(0x00dc_0500, &args![other, stream, 0x55u32]).u32();
        assert_eq!(got, other);
        assert_eq!(
            calls(&f.e, ARCHIVE_CONSTRUCT),
            vec![vec![other, stream, other & 0xffff_ff00]]
        );
        let ignored = stream & 0xffff_ff00;
        assert_eq!(
            calls(&f.e, STRING_MAP_CONSTRUCT),
            vec![vec![other + 0x10, ignored], vec![other + 0x20, ignored]]
        );
        // Both maps start with the empty string as number 0 (classes first).
        assert_eq!(
            calls(&f.e, STRING_MAP_INSERT),
            vec![
                vec![other + 0x20, EMPTY_STRING, 0],
                vec![other + 0x10, EMPTY_STRING, 0]
            ]
        );
        assert_eq!(
            f.e.mem.u32(other + 0x30),
            0,
            "no object number given out yet"
        );
        assert_eq!(f.e.mem.u32(other + 0x58), 0x55, "the listener");
        assert_eq!(
            f.e.mem.i32(other + 0x34 + 8),
            7,
            "the object map has 8 slots"
        );
        for array in [other + 0x40, other + 0x4c] {
            assert_eq!(
                [
                    f.e.mem.u32(array),
                    f.e.mem.u32(array + 4),
                    f.e.mem.u32(array + 8)
                ],
                [0, 0, 0x8000_0000]
            );
        }
        // After some work the destructor returns the records and the flag bytes.
        let n = f.member("n", 1, 0, 0);
        let class = f.class("C", 0, 1, vec![n]);
        let object = f.object(class, vec![Val::Int(1)]);
        let handle = f.handle(object);
        f.e.call(0x00db_f450, &args![writer, handle, 0u32]);
        let records = f.e.mem.u32(writer + 0x4c);
        let record_capacity = f.e.mem.u32(writer + 0x54);
        let bytes = f.e.mem.u32(writer + 0x40);
        let byte_capacity = f.e.mem.u32(writer + 0x48);
        let table = f.e.mem.u32(writer + 0x34);
        f.e.call_log = Some(Vec::new());
        f.e.call(0x00dc_0460, &args![writer]);
        let freed = calls(&f.e, ALLOCATOR_GROWTH + 0x10);
        assert!(freed
            .iter()
            .any(|w| w[1..] == [records, record_capacity * 0x18]));
        assert!(freed.iter().any(|w| w[1..] == [bytes, byte_capacity]));
        assert!(calls(&f.e, ALLOCATOR_BLOCKS + 8)
            .iter()
            .any(|w| w[1..] == [table, 96]));
        assert_eq!(
            calls(&f.e, STRING_MAP_DESTRUCT),
            vec![vec![writer + 0x20], vec![writer + 0x10]]
        );
        assert_eq!(calls(&f.e, ARCHIVE_DESTRUCT), vec![vec![writer]]);
        for array in [writer + 0x40, writer + 0x4c] {
            assert_eq!(
                [
                    f.e.mem.u32(array),
                    f.e.mem.u32(array + 4),
                    f.e.mem.u32(array + 8)
                ],
                [0, 0, 0x8000_0000]
            );
        }
    }
}
