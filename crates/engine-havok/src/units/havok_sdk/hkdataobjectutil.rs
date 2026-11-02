//! `Havok SDK/hkdataobjectutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDataObjectUtil` converts between the data-object model (`hkDataObject`, `hkDataArray`,
//! `hkDataClass`, handles into a world) and native Havok objects. Two anonymous helper classes of
//! the source file do the work: `Copier` (data objects to native objects: it keeps the class
//! registry, the `hkPackfileData` the native blocks go into, the map from an object's handle to
//! its native copy and the handles in progress) and `Cloner` (data objects of one world copied
//! into another world). The Xbox PDB has no layout for either, so [`Copier`] and [`Cloner`] are
//! roles read from the code.
//!
//! The unit's hash maps are instances of `hkMapBase` whose empty slots have first word
//! `0xffffffff`: 12-byte slots (handle to one word, the copier's native object map) and 16-byte
//! slots (handle to a handle, the cloner's object map), with the layout and hash of `hkmap.rs`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkarrayutil::HkArrayBase;
use crate::units::havok_sdk::hkmap::HkMapBase;

/// The memory router (`00c85750`): `+0xc` is the allocator of array growth and `+0x10` the one
/// of the blocks and the maps.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// the growth step next to it (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkReferencedObject`'s reference counting (`00c90510` adds, `00c905b0` removes; the object is
/// the first word).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkString::memSet` (cdecl `(destination, value, bytes)`), `strDup` (cdecl), `strCmp` (cdecl).
const MEM_SET: u32 = 0x00cb_1280;
const STRING_DUP: u32 = 0x00cb_1410;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// `hkMapBase<unsigned long,unsigned long>` (hkmap instance, Xbox PDB names): `reserve`,
/// `getWithDefault`, `findOrInsertKey` and `clearAndDeallocate`, and `00cb1870`, which the
/// resource builder calls with an allocator, a native object and its class name.
const POINTER_MAP_RESERVE: u32 = 0x00cb_1fe0;
const POINTER_MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const POINTER_MAP_FIND_OR_INSERT_KEY: u32 = 0x00cb_1f40;
const POINTER_MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const POINTER_MAP_INSERT_00CB1870: u32 = 0x00cb_1870;
/// The map clear-and-deallocate (`00d5f640`) the deep copy ends with.
const OBJECT_MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00d5_f640;
/// `hkClass` (hkclass.cpp, Xbox PDB names): member count and member, name, parent, object size,
/// whether it has a vtable, described version, and `00d46d80` (`out`, other class: whether the
/// object is a base class of the other, an `hkBool` written to `out`).
const CLASS_GET_NUM_MEMBERS: u32 = 0x00d4_6f00;
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;
const CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00d4_6f70;
const CLASS_GET_DECLARED_MEMBER: u32 = 0x00d4_6f80;
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const CLASS_GET_PARENT: u32 = 0x00d4_6d50;
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
const CLASS_HAS_VTABLE: u32 = 0x00d4_70f0;
const CLASS_GET_DESCRIBED_VERSION: u32 = 0x00d4_7120;
const CLASS_IS_SUPER_CLASS: u32 = 0x00d4_6d80;
/// `hkClassMember` (hkclassmember.cpp, Xbox PDB names): class, C-style array size, attribute by
/// name, size in bytes; and `00d86bc0`, which `00dc1bf0` calls on a member.
const MEMBER_GET_CLASS: u32 = 0x00d8_6920;
const MEMBER_GET_CSTYLE_ARRAY_SIZE: u32 = 0x00d8_6940;
const MEMBER_GET_ATTRIBUTE: u32 = 0x00d8_6960;
const MEMBER_GET_SIZE_IN_BYTES: u32 = 0x00d8_6980;
const MEMBER_CALL_00D86BC0: u32 = 0x00d8_6bc0;
/// `hkPackfileData` (hkpackfiledata.cpp): the constructor, and `00e05100`, which gives the
/// finished package its root object and class name.
const PACKFILE_DATA_CONSTRUCT: u32 = 0x00e0_5700;
const PACKFILE_DATA_SET_CONTENTS: u32 = 0x00e0_5100;
/// The resource the object-with-registry conversion returns: `00e04f10` constructs it from the
/// root pair, `00e04eb0` and `00e04ee0` hand it the registry and the second argument.
const RESOURCE_CONSTRUCT: u32 = 0x00e0_4f10;
const RESOURCE_SET_REGISTRY: u32 = 0x00e0_4eb0;
const RESOURCE_SET_ARGUMENT: u32 = 0x00e0_4ee0;
/// `hkOstream` over a character buffer (`00cb1600` constructs it, `00cc01c0` appends a string,
/// `00cc00e0` destroys it) and the error reporter global (`01268360`; its slot `0xc` reports).
const STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_APPEND: u32 = 0x00cc_01c0;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_REPORTER: u32 = 0x0126_8360;
/// `hkDataClass`, `hkDataObject`, `hkDataArray` and their `Value`s (hkdataobject.cpp).
const DATA_CLASS_CONSTRUCT: u32 = 0x00db_a650;
const DATA_CLASS_DESTRUCT: u32 = 0x00db_a660;
const DATA_CLASS_IS_NULL: u32 = 0x00db_a670;
const DATA_CLASS_GET_NAME: u32 = 0x00db_a680;
const DATA_CLASS_GET_VERSION: u32 = 0x00db_a690;
const DATA_CLASS_GET_PARENT: u32 = 0x00db_a6a0;
const DATA_CLASS_GET_WORLD: u32 = 0x00db_a6c0;
const DATA_CLASS_GET_NUM_MEMBERS: u32 = 0x00db_a730;
const DATA_CLASS_GET_ALL_MEMBER_INFO: u32 = 0x00db_aed0;
const DATA_CLASS_GET_CINFO: u32 = 0x00db_afe0;
const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
const DATA_OBJECT_CONSTRUCT_COPY: u32 = 0x00db_a780;
const DATA_OBJECT_DESTRUCT: u32 = 0x00db_a7f0;
const DATA_OBJECT_GET_HANDLE: u32 = 0x00db_a810;
const DATA_OBJECT_GET_CLASS: u32 = 0x00db_a850;
const DATA_OBJECT_MEMBER_VALUE: u32 = 0x00db_a870;
const DATA_OBJECT_MEMBER_VALUE_OV2: u32 = 0x00db_a890;
const DATA_OBJECT_HAS_MEMBER: u32 = 0x00db_a910;
const DATA_OBJECT_IS_NULL: u32 = 0x00db_a980;
const OBJECT_VALUE_GET_TYPE: u32 = 0x00db_a920;
const OBJECT_VALUE_AS_INT: u32 = 0x00db_a990;
const OBJECT_VALUE_AS_VEC: u32 = 0x00db_a9a0;
const OBJECT_VALUE_AS_STRING: u32 = 0x00db_a9c0;
const OBJECT_VALUE_AS_OBJECT: u32 = 0x00db_a9d0;
const OBJECT_VALUE_AS_REAL: u32 = 0x00db_aa00;
const OBJECT_VALUE_SET_INT: u32 = 0x00db_aa30;
const OBJECT_VALUE_SET_OBJECT: u32 = 0x00db_aa50;
const OBJECT_VALUE_SET_REAL: u32 = 0x00db_aab0;
const OBJECT_VALUE_SET_VEC: u32 = 0x00db_ab10;
const OBJECT_VALUE_SET_STRING: u32 = 0x00db_ab30;
const OBJECT_VALUE_AS_ARRAY: u32 = 0x00db_afb0;
const DATA_ARRAY_CONSTRUCT: u32 = 0x00db_ab50;
const DATA_ARRAY_DESTRUCT: u32 = 0x00db_ab70;
const DATA_ARRAY_ELEMENT: u32 = 0x00db_ab90;
const DATA_ARRAY_ELEMENT_OV2: u32 = 0x00db_abb0;
const DATA_ARRAY_SET_SIZE: u32 = 0x00db_abe0;
const DATA_ARRAY_GET_SIZE: u32 = 0x00db_ac00;
const DATA_ARRAY_GET_TYPE: u32 = 0x00db_ac10;
const DATA_ARRAY_GET_CLASS: u32 = 0x00db_ac20;
const DATA_ARRAY_SWIZZLE_OBJECT_MEMBER: u32 = 0x00db_aea0;
const ARRAY_VALUE_GET_TYPE: u32 = 0x00db_ac40;
const ARRAY_VALUE_AS_VEC: u32 = 0x00db_ac50;
const ARRAY_VALUE_AS_STRING: u32 = 0x00db_ac70;
const ARRAY_VALUE_AS_REAL: u32 = 0x00db_ac80;
const ARRAY_VALUE_AS_INT: u32 = 0x00db_ac90;
const ARRAY_VALUE_AS_OBJECT: u32 = 0x00db_aca0;
const ARRAY_VALUE_AS_ARRAY: u32 = 0x00db_acd0;
const ARRAY_VALUE_SET_OBJECT: u32 = 0x00db_ad00;
const ARRAY_VALUE_SET_REAL: u32 = 0x00db_ad20;
const ARRAY_VALUE_SET_VEC: u32 = 0x00db_ad60;
const ARRAY_VALUE_SET_INT: u32 = 0x00db_ada0;
const ARRAY_VALUE_SET_STRING: u32 = 0x00db_adc0;
/// The matrix and quaternion converters the copy calls on the destination (`this`) with the
/// source floats: `(destination, source)`.
const CONVERT_MATRIX_3: u32 = 0x00c6_cd00;
const CONVERT_MATRIX_3_B: u32 = 0x00c6_cd30;
const CONVERT_MATRIX_4: u32 = 0x00c6_cd60;
/// The table `getTypeFromMemberType` indexes by member type (`010d6b98`, one word per type).
const MEMBER_TYPE_TABLE: u32 = 0x010d_6b98;
/// The messages of the unfinished-virtual-object report and its source file.
const MESSAGE_HEAD: u32 = 0x010d_6c48;
const MESSAGE_TAIL: u32 = 0x010d_6c70;
const SOURCE_FILE: u32 = 0x010d_6c24;
/// The name of the member attribute that names a data object's class (`010cda84`).
const ATTRIBUTE_NAME: u32 = 0x010c_da84;
/// The class whose derived classes `findOrCopy` counts references for (`01268748`, a global
/// `hkClass`).
const BASE_CLASS: u32 = 0x0126_8748;
/// The hash multiplier `-0x61c8864f` of the maps.
const HASH_MULTIPLIER: u32 = 0x9e37_79b1;
/// Bit 31 of a map's element count or an array's capacity word: the storage is not owned.
const NOT_OWNED: u32 = 0x8000_0000;
/// The size bits of an array's capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The size of an `hkDataClass::MemberInfo` record.
const MEMBER_INFO_SIZE: u32 = 0x14;
/// A slot of the copier's native object map (key low, key high, value) and of the cloner's
/// object map (key low, key high, value low, value high).
const SMALL_SLOT_SIZE: u32 = 12;
const LARGE_SLOT_SIZE: u32 = 16;

layout! {
    /// `hkClassMember` (Xbox PDB), 0x18 bytes: the fields the conversion reads.
    pub struct HkClassMember: 0x18 {
        /// `m_name` (Xbox PDB).
        0x00 m_name: u32,
        /// `m_enum` (Xbox PDB).
        0x08 m_enum: u32,
        /// `m_type` (Xbox PDB).
        0x0c m_type: u8,
        /// `m_subtype` (Xbox PDB).
        0x0d m_subtype: u8,
        /// `m_flags` (Xbox PDB).
        0x10 m_flags: u16,
        /// `m_offset` (Xbox PDB).
        0x12 m_offset: u16,
    }

    /// `Copier` (hkdataobjectutil.cpp; no Xbox PDB layout, roles read from the code), 0x30
    /// bytes.
    pub struct Copier: 0x30 {
        /// The class registry (a reference-counted object whose slot `0x10` maps a class name
        /// to its `hkClass`).
        0x00 registry: Ptr,
        /// The `hkPackfileData` the native blocks and strings are added to.
        0x04 packfile: Ptr,
        /// The constructor's second argument (1 for `toObjectWithRegistry`, 0 for
        /// `toResourceWithRegistry`).
        0x08 flag: u32,
        /// Native object to reference count (`hkMapBase<unsigned long,unsigned long>`).
        0x0c native_references: Inline<HkMapBase>,
        /// Object handle to native object, 12-byte slots, `0xffffffff` when empty.
        0x18 objects: Inline<HkMapBase>,
        /// The handles of the objects being copied (two words each).
        0x24 in_progress: Inline<HkArrayBase>,
    }

    /// `Cloner` (hkdataobjectutil.cpp; no Xbox PDB layout, roles read from the code), 0x14
    /// bytes.
    pub struct Cloner: 0x14 {
        /// The world the copies are made in (slot `0xc` defines a class, `0x10` makes an
        /// object, `0x24` finds a class by name, `0x28` gets an object from its handle).
        0x00 destination: Ptr,
        /// The world the copies are made from.
        0x04 source: Ptr,
        /// Source object handle to destination object handle, 16-byte slots, `0xffffffff` when
        /// empty.
        0x08 objects: Inline<HkMapBase>,
    }

    /// `hkDataClass::MemberInfo` (the records of `getAllMemberInfo`, 0x14 bytes, roles read from
    /// the code).
    pub struct MemberInfo: 0x14 {
        /// The member's name (a C string).
        0x00 name: u32,
        /// The member's type word: the low four bits are the type, bit 0x10 an array, bit 0x20 a
        /// tuple.
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

/// Copies `bytes` bytes (the code's aligned vector moves, which fault on a misaligned address:
/// the memory model has no alignment fault).
fn copy_bytes(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    let data = e.mem.bytes(source, bytes);
    e.mem.write(destination, &data);
}

/// The home slot of a key before masking.
fn hash(key_low: u32) -> u32 {
    (key_low >> 4).wrapping_mul(HASH_MULTIPLIER)
}

/// `hkArray::reserve` as the code inlines it for `findDestClass` and `getClassMemberInfos`: when
/// the capacity is below `wanted`, `_reserve` is called with `wanted`, or twice the capacity if
/// that is larger (`at_least` is the smallest request `findDestClass` makes).
fn array_reserve(e: &mut Engine, array: Ptr<HkArrayBase>, wanted: i32, element_size: i32) {
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

/// Zeroes records `from..to` of the member-info array at `data` (0x14 bytes each), skipping an
/// address of 0 (the placement-new null check).
fn zero_member_infos(e: &mut Engine, data: u32, from: i32, to: i32) {
    for index in from..to {
        let record = data.wrapping_add((index as u32).wrapping_mul(MEMBER_INFO_SIZE));
        if record != 0 {
            for word in 0..5 {
                e.mem.set_u32(record + 4 * word, 0);
            }
        }
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
    zero_member_infos(e, data, 0, size);
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

// ---- The maps with 12-byte slots (handle to one word) ----

// Translated from 00dc35e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. The first used slot of a 12-byte-slot map: the index of the first
/// slot whose first word is not `0xffffffff`, or `m_hashMod + 1` when there is none (0 when
/// `m_hashMod` is negative).
pub fn fn_00dc35e0(e: &mut Engine, this: Ptr<HkMapBase>) -> i32 {
    let last = e.get(this, HkMapBase::m_hashMod);
    let mut index = 0i32;
    if last >= 0 {
        let table = e.get(this, HkMapBase::m_elem);
        loop {
            let slot = table.wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE));
            if e.mem.u32(slot) != 0xffff_ffff {
                break;
            }
            index += 1;
            if index > last {
                break;
            }
        }
    }
    index
}

// Translated from 00dc3600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `insert` of the copier's map (key: two words, value: one word,
/// empty slots have key word `0xffffffff`): first grows the table (`00dc3bf0`, to `2 * m_hashMod
/// + 2` slots) when twice the element count exceeds `m_hashMod`; then stores the pair at the
/// first free slot of the probe sequence or over the slot with an equal key, the element count
/// rising for a new key.
pub fn fn_00dc3600(
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
        fn_00dc3bf0(e, this, allocator, slots);
    }
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;
    let table = e.get(this, HkMapBase::m_elem);
    let mut slot = hash(key_low) & mask;
    let mut added = 1u32;
    loop {
        let address = table.wrapping_add(slot.wrapping_mul(SMALL_SLOT_SIZE));
        if e.mem.u32(address) == 0xffff_ffff {
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
    let address = table.wrapping_add(slot.wrapping_mul(SMALL_SLOT_SIZE));
    e.mem.set_u32(address, key_low);
    e.mem.set_u32(address + 4, key_high);
    e.mem.set_u32(address + 8, value);
}

// Translated from 00dc36b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<hkDataObject::Handle,void *,Copier::HandleOps>::getWithDefault` (engine map name):
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
        let mut slot = hash(key_low) & mask;
        loop {
            let address = table.wrapping_add(slot.wrapping_mul(SMALL_SLOT_SIZE));
            if e.mem.u32(address) == 0xffff_ffff {
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

// Translated from 00dc38f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Empties a 12-byte-slot map: the first word of every slot
/// (`m_hashMod + 1` of them) becomes `0xffffffff`, and the element count is cleared except the
/// not-owned flag.
pub fn fn_00dc38f0(e: &mut Engine, this: Ptr<HkMapBase>) {
    let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let mut offset = 0u32;
    for _ in 0..slots.max(0) {
        let table = e.get(this, HkMapBase::m_elem);
        e.mem.set_u32(table.wrapping_add(offset), 0xffff_ffff);
        offset = offset.wrapping_add(SMALL_SLOT_SIZE);
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word & NOT_OWNED);
}

// Translated from 00dc3b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Empties a 12-byte-slot map (`00dc38f0`) and, unless its slots are
/// not owned, returns them to `allocator` (vtable slot 8, the slots and their size in bytes).
pub fn fn_00dc3b60(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr) {
    fn_00dc38f0(e, this);
    if e.get(this, HkMapBase::m_numElems) & NOT_OWNED == 0 {
        let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1) as u32;
        let table = e.get(this, HkMapBase::m_elem);
        e.vcall(
            allocator.addr(),
            8,
            &args![table, slots.wrapping_mul(SMALL_SLOT_SIZE)],
        );
    }
}

// Translated from 00dc3bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `resizeTable` of a 12-byte-slot map: gives the map `max(slots,
/// 8)` slots from `allocator` (vtable slot 4, the size in bytes), marks them empty, clears the
/// element count (and the not-owned flag), inserts every element of the old table again
/// (`00dc3600`) and, if the old slots were owned, returns them to the allocator (vtable slot 8,
/// the slots and their size).
pub fn fn_00dc3bf0(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, slots: i32) {
    let slots = if slots > 8 { slots } else { 8 };
    let flags = e.get(this, HkMapBase::m_numElems) & NOT_OWNED;
    let old_table = e.get(this, HkMapBase::m_elem);
    let old_slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let bytes = (slots as u32).wrapping_mul(SMALL_SLOT_SIZE);
    let table = e.vcall(allocator.addr(), 4, &args![bytes]).u32();
    e.set(this, HkMapBase::m_elem, table);
    for index in 0..slots as u32 {
        let table = e.get(this, HkMapBase::m_elem);
        e.mem.set_u32(
            table.wrapping_add(index.wrapping_mul(SMALL_SLOT_SIZE)),
            0xffff_ffff,
        );
    }
    e.set(this, HkMapBase::m_numElems, 0);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1));
    for index in 0..old_slots.max(0) as u32 {
        let slot = old_table.wrapping_add(index.wrapping_mul(SMALL_SLOT_SIZE));
        if e.mem.u32(slot) != 0xffff_ffff {
            let (low, high, value) = (e.mem.u32(slot), e.mem.u32(slot + 4), e.mem.u32(slot + 8));
            fn_00dc3600(e, this, allocator, low, high, value);
        }
    }
    if flags == 0 {
        e.vcall(
            allocator.addr(),
            8,
            &args![old_table, (old_slots as u32).wrapping_mul(SMALL_SLOT_SIZE)],
        );
    }
}

// Translated from 00dc3cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `reserve` of a 12-byte-slot map: the smallest power of two that
/// is at least 8 and at least twice `count`, handed to `resizeTable` (`00dc3bf0`).
pub fn fn_00dc3cc0(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, count: i32) {
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
    fn_00dc3bf0(e, this, allocator, slots);
}

// ---- The maps with 16-byte slots (handle to handle) ----

// Translated from 00dc3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `insert` of the cloner's map (key: two words, value: two words,
/// empty slots have key word `0xffffffff`): first grows the table (`00dc39b0`, to `2 * m_hashMod
/// + 2` slots) when twice the element count exceeds `m_hashMod`; then stores the entry at the
/// first free slot of the probe sequence or over the slot with an equal key, the element count
/// rising for a new key.
pub fn fn_00dc3740(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    key_low: u32,
    key_high: u32,
    value_low: u32,
    value_high: u32,
) {
    let count = (e.get(this, HkMapBase::m_numElems) & 0x7fff_ffff) as i32;
    let hash_mod = e.get(this, HkMapBase::m_hashMod);
    if count.wrapping_add(count) > hash_mod {
        let slots = hash_mod.wrapping_add(hash_mod).wrapping_add(2);
        fn_00dc39b0(e, this, allocator, slots);
    }
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;
    let table = e.get(this, HkMapBase::m_elem);
    let mut slot = hash(key_low) & mask;
    let mut added = 1u32;
    loop {
        let address = table.wrapping_add(slot.wrapping_mul(LARGE_SLOT_SIZE));
        if e.mem.u32(address) == 0xffff_ffff {
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
    let address = table.wrapping_add(slot.wrapping_mul(LARGE_SLOT_SIZE));
    e.mem.set_u32(address, key_low);
    e.mem.set_u32(address + 4, key_high);
    let table = e.get(this, HkMapBase::m_elem);
    let address = table.wrapping_add(slot.wrapping_mul(LARGE_SLOT_SIZE));
    e.mem.set_u32(address + 8, value_low);
    e.mem.set_u32(address + 0xc, value_high);
}

// Translated from 00dc3920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<hkDataObject::Handle,hkDataObject::Handle,Copier::HandleOps>::findKey` (engine map
/// name): the slot index holding the key (two words), or `m_hashMod + 1` when it is not in the
/// map (also when `m_hashMod` is not positive).
pub fn hk_map_base_find_key(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key_low: u32,
    key_high: u32,
) -> u32 {
    let mask = e.get(this, HkMapBase::m_hashMod);
    if mask > 0 {
        let table = e.get(this, HkMapBase::m_elem);
        let mask = mask as u32;
        let mut slot = hash(key_low) & mask;
        loop {
            let address = table.wrapping_add(slot.wrapping_mul(LARGE_SLOT_SIZE));
            if e.mem.u32(address) == 0xffff_ffff {
                break;
            }
            if e.mem.u32(address) == key_low && e.mem.u32(address + 4) == key_high {
                return slot;
            }
            slot = slot.wrapping_add(1) & mask;
        }
    }
    (mask as u32).wrapping_add(1)
}

// Translated from 00dc3980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Empties a 16-byte-slot map: the first word of every slot
/// (`m_hashMod + 1` of them) becomes `0xffffffff`, and the element count is cleared except the
/// not-owned flag.
pub fn fn_00dc3980(e: &mut Engine, this: Ptr<HkMapBase>) {
    let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let mut offset = 0u32;
    for _ in 0..slots.max(0) {
        let table = e.get(this, HkMapBase::m_elem);
        e.mem.set_u32(table.wrapping_add(offset), 0xffff_ffff);
        offset = offset.wrapping_add(LARGE_SLOT_SIZE);
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word & NOT_OWNED);
}

// Translated from 00dc39b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `resizeTable` of a 16-byte-slot map: gives the map `max(slots,
/// 8)` slots from `allocator` (vtable slot 4, the size in bytes), marks them empty, clears the
/// element count (and the not-owned flag), inserts every element of the old table again
/// (`00dc3740`) and, if the old slots were owned, returns them to the allocator (vtable slot 8,
/// the slots and their size).
pub fn fn_00dc39b0(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, slots: i32) {
    let slots = if slots > 8 { slots } else { 8 };
    let flags = e.get(this, HkMapBase::m_numElems) & NOT_OWNED;
    let old_table = e.get(this, HkMapBase::m_elem);
    let old_slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let bytes = (slots as u32).wrapping_mul(LARGE_SLOT_SIZE);
    let table = e.vcall(allocator.addr(), 4, &args![bytes]).u32();
    e.set(this, HkMapBase::m_elem, table);
    for index in 0..slots as u32 {
        let table = e.get(this, HkMapBase::m_elem);
        e.mem.set_u32(
            table.wrapping_add(index.wrapping_mul(LARGE_SLOT_SIZE)),
            0xffff_ffff,
        );
    }
    e.set(this, HkMapBase::m_numElems, 0);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1));
    for index in 0..old_slots.max(0) as u32 {
        let slot = old_table.wrapping_add(index.wrapping_mul(LARGE_SLOT_SIZE));
        if e.mem.u32(slot) != 0xffff_ffff {
            let words = (
                e.mem.u32(slot),
                e.mem.u32(slot + 4),
                e.mem.u32(slot + 8),
                e.mem.u32(slot + 0xc),
            );
            fn_00dc3740(e, this, allocator, words.0, words.1, words.2, words.3);
        }
    }
    if flags == 0 {
        e.vcall(
            allocator.addr(),
            8,
            &args![old_table, (old_slots as u32).wrapping_mul(LARGE_SLOT_SIZE)],
        );
    }
}

// Translated from 00dc3b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<hkDataObject::Handle,hkDataObject::Handle,Copier::HandleOps>::get` (engine map
/// name): looks the key (two words) up (`findKey`); when it is in the map (the index is not past
/// `m_hashMod`) its two value words are written to `out` and 0 is returned, otherwise 1.
pub fn hk_map_base_get(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key_low: u32,
    key_high: u32,
    out: Ptr,
) -> u32 {
    let index = hk_map_base_find_key(e, this, key_low, key_high);
    if index as i32 <= e.get(this, HkMapBase::m_hashMod) {
        let table = e.get(this, HkMapBase::m_elem);
        let slot = table.wrapping_add(index.wrapping_mul(LARGE_SLOT_SIZE));
        let (low, high) = (e.mem.u32(slot + 8), e.mem.u32(slot + 0xc));
        e.mem.set_u32(out.addr(), low);
        e.mem.set_u32(out.addr() + 4, high);
        return 0;
    }
    1
}

// Translated from 00dc3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. `reserve` of a 16-byte-slot map: the smallest power of two that
/// is at least 8 and at least twice `count`, handed to `resizeTable` (`00dc39b0`).
pub fn fn_00dc3bd0(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, count: i32) {
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
    fn_00dc39b0(e, this, allocator, slots);
}

// Translated from 00dc3710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Copies the 0x40 bytes (a 4x4 matrix) at `source` to `this`
/// (four aligned vector moves; the exe's `MOVAPS` faults on a misaligned address, which the
/// memory model does not model).
pub fn fn_00dc3710(e: &mut Engine, this: Ptr, source: Ptr) {
    copy_bytes(e, this.addr(), source.addr(), 0x40);
}

// Translated from 00dc3ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::addChunk` (engine map name): appends the chunk record `(pointer, size,
/// kind)` to the 12-byte-record array at `this + 0x40`, growing it first (`00c90940` with the
/// router's allocator at `+0x10`) when it is full.
pub fn hk_packfile_data_add_chunk(e: &mut Engine, this: Ptr, pointer: u32, size: u32, kind: u32) {
    let chunks: Ptr<HkArrayBase> = Ptr::new(this.addr().wrapping_add(0x40));
    let allocator = router_allocator(e, 0x10);
    let count = e.get(chunks, HkArrayBase::m_size);
    let capacity = e.get(chunks, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
    if count as u32 == capacity {
        e.call(ARRAY_GROW, &args![allocator, chunks, 0xcu32]);
    }
    let count = e.get(chunks, HkArrayBase::m_size);
    let data = e.get(chunks, HkArrayBase::m_data).addr();
    let record = data.wrapping_add((count as u32).wrapping_mul(12));
    if record != 0 {
        e.mem.set_u32(record, pointer);
        e.mem.set_u32(record + 4, size);
        e.mem.set_u32(record + 8, kind);
    }
    e.set(chunks, HkArrayBase::m_size, count.wrapping_add(1));
}

// ---- hkClassMember helpers ----

// Translated from 00dc1bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. For a member of type 0x18 or 0x1f (`m_type`) whose `m_enum` is
/// set, calls `00d86bc0` on the member with `target` and the integer in `value`
/// (`hkDataObject::Value::asInt`) and answers 0; a member of either type without an enum also
/// answers 0, any other type 1. `_unused_4` is a word every caller pushes and the code never
/// reads.
pub fn fn_00dc1bf0(
    e: &mut Engine,
    target: u32,
    member: Ptr<HkClassMember>,
    value: Ptr,
    _unused_4: u32,
) -> u32 {
    let kind = e.get(member, HkClassMember::m_type);
    if kind == 0x18 || kind == 0x1f {
        if e.get(member, HkClassMember::m_enum) != 0 {
            let number = e.call(OBJECT_VALUE_AS_INT, &args![value]).u32();
            e.call(MEMBER_CALL_00D86BC0, &args![member, target, number]);
        }
        return 0;
    }
    1
}

// Translated from 00dc1c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectUtil::getTypeFromMemberType` (engine map name): the data-object type of a class
/// member of type `member_type` and subtype `subtype` (for the types 0x18 and 0x1f the subtype
/// takes the place of the type and is then 0). The type comes from the table at `010d6b98`; a
/// table value of 0x10 (struct) is refined by the subtype: from the table again, ORed with 0x10,
/// except subtype 4 (0x11) and subtype 0x19, which is 0x19 for a class (`class`) whose members
/// have none of the types 0x16, 0x1a and 0x1b and 0x18 otherwise. A nonzero `array` adds the
/// tuple bit 0x20. (The exe's cdecl: `(type, subtype, class, array)`.)
pub fn hk_data_object_util_get_type_from_member_type(
    e: &mut Engine,
    member_type: u32,
    subtype: u32,
    class: Ptr,
    array: u32,
) -> u32 {
    let (kind, sub) = if member_type == 0x18 || member_type == 0x1f {
        (subtype, 0)
    } else {
        (member_type, subtype)
    };
    let mut result: u32 = e.global(MEMBER_TYPE_TABLE.wrapping_add(kind.wrapping_mul(4)));
    if result == 0x10 && sub <= 0x21 {
        if sub == 4 {
            result = 0x11;
        } else if sub == 0x19 {
            let mut plain = true;
            let mut index = 0i32;
            let mut count = e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32();
            if count > 0 {
                loop {
                    let member = e.call(CLASS_GET_MEMBER, &args![class, index]).u32();
                    let member_kind = u32::from(e.mem.u8(member + 0xc));
                    if member_kind == 0x16 || (member_kind > 0x19 && member_kind <= 0x1b) {
                        plain = false;
                        break;
                    }
                    index += 1;
                    count = e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32();
                    if index >= count {
                        break;
                    }
                }
            }
            result = u32::from(plain) + 0x18;
        } else {
            let value: u32 = e.global(MEMBER_TYPE_TABLE.wrapping_add(sub.wrapping_mul(4)));
            result = value | 0x10;
        }
    }
    if array != 0 {
        result |= 0x20;
    }
    result
}

// ---- Copier: data objects to native objects ----

// Translated from 00dc3a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::getMemberClassAndCheck` (engine map name): the `hkClass` to build for the object
/// `object` held by the member `member`. A null object has none (0). The object's own class is
/// found by name through the registry (slot `0x10`); a member without a class gets it, a member
/// with a class and no matching registered class gets none, and a member whose class has the
/// same name as the object's gets the member's class. When the names differ the object's class
/// wins if the member's class is a base class of it (`00d46d80`); otherwise the class named by
/// the member's `hk.DataObjectType` attribute is looked up in the registry and gives the object's
/// class when it is its base, else that attribute's class (or the member's, without the
/// attribute).
pub fn copier_get_member_class_and_check(
    e: &mut Engine,
    this: Ptr<Copier>,
    member: Ptr<HkClassMember>,
    object: Ptr,
) -> u32 {
    if e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() != 0 {
        return 0;
    }
    let member_class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
    e.with_stack(8, |e, frame| {
        let handle = frame.addr();
        let base_out = frame.addr() + 4;
        let class = e.call(DATA_OBJECT_GET_CLASS, &args![object, handle]).u32();
        let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
        e.call(DATA_CLASS_DESTRUCT, &args![handle]);
        let registry = e.get(this, Copier::registry).addr();
        let object_class = if name != 0 {
            e.vcall(registry, 0x10, &args![name]).u32()
        } else {
            0
        };
        if member_class == 0 {
            return object_class;
        }
        if object_class == 0 {
            return 0;
        }
        let member_class_name = e.call(CLASS_GET_NAME, &args![member_class]).u32();
        if e.call(STRING_COMPARE, &args![member_class_name, name])
            .u32()
            == 0
        {
            return member_class;
        }
        let is_base = e
            .call(
                CLASS_IS_SUPER_CLASS,
                &args![member_class, base_out, object_class],
            )
            .u32();
        if e.mem.u8(is_base) != 0 {
            return object_class;
        }
        let attribute = e
            .call(MEMBER_GET_ATTRIBUTE, &args![member, ATTRIBUTE_NAME])
            .u32();
        if attribute == 0 {
            return member_class;
        }
        let first = e.mem.u32(attribute);
        let attribute_name = e.mem.u32(first);
        let attribute_class = e.vcall(registry, 0x10, &args![attribute_name]).u32();
        let is_base = e
            .call(
                CLASS_IS_SUPER_CLASS,
                &args![attribute_class, base_out, object_class],
            )
            .u32();
        if e.mem.u8(is_base) != 0 {
            object_class
        } else {
            attribute_class
        }
    })
}

// Translated from 00dc4f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::Copier` (engine map name): stores the class registry (adding a reference when it
/// is not null), no package, and `flag`; empties the native reference map (`+0xc`, reserved for
/// 0 elements, `00cb1fe0`), the object map (`+0x18`, reserved for 0 elements, `00dc3cc0`) and the
/// in-progress array (not owned). Returns `this`.
pub fn copier_constructor(
    e: &mut Engine,
    this: Ptr<Copier>,
    registry: Ptr,
    flag: u32,
) -> Ptr<Copier> {
    if registry.addr() != 0 {
        e.call(ADD_REFERENCE, &args![registry]);
    }
    e.set(this, Copier::registry, registry);
    e.set(this, Copier::packfile, Ptr::NULL);
    e.set(this, Copier::flag, flag);
    let references = this.at(Copier::native_references);
    e.set(references, HkMapBase::m_elem, 0);
    e.set(references, HkMapBase::m_numElems, 0);
    e.set(references, HkMapBase::m_hashMod, -1);
    let allocator = router_allocator(e, 0x10);
    e.call(POINTER_MAP_RESERVE, &args![references, allocator, 0u32]);
    let objects = this.at(Copier::objects);
    e.set(objects, HkMapBase::m_elem, 0);
    e.set(objects, HkMapBase::m_numElems, 0);
    e.set(objects, HkMapBase::m_hashMod, -1);
    let allocator = router_allocator(e, 0x10);
    fn_00dc3cc0(e, objects, Ptr::new(allocator), 0);
    let in_progress = this.at(Copier::in_progress);
    e.set(in_progress, HkArrayBase::m_data, Ptr::NULL);
    e.set(in_progress, HkArrayBase::m_size, 0);
    e.set(in_progress, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    this
}

// Translated from 00dc43a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::~Copier` (engine map name): empties the in-progress array and returns its storage
/// (the router's allocator at `+0x10`, slot `0x10`, 8 bytes per element) unless not owned,
/// empties the object map (`00dc3b60`) and the native reference map (`00cb1a90`), and releases
/// the package and the registry (`00c905b0`).
pub fn copier_destructor(e: &mut Engine, this: Ptr<Copier>) {
    let in_progress = this.at(Copier::in_progress);
    let allocator = router_allocator(e, 0x10);
    let capacity = e.get(in_progress, HkArrayBase::m_capacityAndFlags);
    e.set(in_progress, HkArrayBase::m_size, 0);
    if capacity as i32 >= 0 {
        let data = e.get(in_progress, HkArrayBase::m_data);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(8)],
        );
    }
    e.set(in_progress, HkArrayBase::m_data, Ptr::NULL);
    e.set(in_progress, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
    let allocator = router_allocator(e, 0x10);
    fn_00dc3b60(e, this.at(Copier::objects), Ptr::new(allocator));
    let allocator = router_allocator(e, 0x10);
    e.call(
        POINTER_MAP_CLEAR_AND_DEALLOCATE,
        &args![this.at(Copier::native_references), allocator],
    );
    let packfile = e.get(this, Copier::packfile);
    if packfile.addr() != 0 {
        e.call(REMOVE_REFERENCE, &args![packfile]);
    }
    e.set(this, Copier::packfile, Ptr::NULL);
    let registry = e.get(this, Copier::registry);
    if registry.addr() != 0 {
        e.call(REMOVE_REFERENCE, &args![registry]);
    }
    e.set(this, Copier::registry, Ptr::NULL);
}

// Translated from 00dc4430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::findOrCopy` (engine map name): writes to `out` the pair (native object, `hkClass`)
/// for the data object `object`: (0, 0) for a null object, an object whose class is null or has
/// no name, or an object already in progress (when the copier's flag and `flag` are both set,
/// the handles being copied are tracked in `+0x24`). Otherwise the class comes from the registry
/// by name; a native copy already in the object map is returned, else a block of the class's
/// object size is allocated (router allocator `+0x10`, slot 4), added to the package as a chunk
/// of kind 6 and zeroed, entered in the object map, and filled (`fillNativeMembers`): when that
/// answers 1 the pair is (0, 0), otherwise the pair is the block and the class. While tracking,
/// an object of a class with a vtable that derives from the class at `01268748` also has its
/// native reference count (`+0xc` map) raised, and the in-progress entry is removed again.
pub fn copier_find_or_copy(e: &mut Engine, this: Ptr<Copier>, out: Ptr, object: Ptr, flag: u32) {
    let pair = e.with_stack(0x30, |e, frame| {
        let class_a = frame.addr();
        let class_b = frame.addr() + 4;
        let class_c = frame.addr() + 8;
        let handle = frame.addr() + 0x10;
        let handle_2 = frame.addr() + 0x18;
        let bool_out = frame.addr() + 0x20;
        let tracking = e.get(this, Copier::flag) != 0 && flag != 0;
        if e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() != 0 {
            return (0u32, 0u32);
        }
        let first = e.call(DATA_OBJECT_GET_CLASS, &args![object, class_a]).u32();
        let mut invalid = true;
        if e.call(DATA_CLASS_IS_NULL, &args![first]).u32() == 0 {
            let second = e.call(DATA_OBJECT_GET_CLASS, &args![object, class_b]).u32();
            invalid = e.call(DATA_CLASS_GET_NAME, &args![second]).u32() == 0;
            e.call(DATA_CLASS_DESTRUCT, &args![class_b]);
        }
        e.call(DATA_CLASS_DESTRUCT, &args![class_a]);
        if invalid {
            return (0, 0);
        }
        let in_progress = this.at(Copier::in_progress);
        if tracking {
            let found = e.call(DATA_OBJECT_GET_HANDLE, &args![object, handle]).u32();
            let count = e.get(in_progress, HkArrayBase::m_size);
            if count > 0 {
                let data = e.get(in_progress, HkArrayBase::m_data).addr();
                let (low, high) = (e.mem.u32(found), e.mem.u32(found + 4));
                for index in 0..count as u32 {
                    let entry = data.wrapping_add(index.wrapping_mul(8));
                    if e.mem.u32(entry) == low && e.mem.u32(entry + 4) == high {
                        return (0, 0);
                    }
                }
            }
            let found = e.call(DATA_OBJECT_GET_HANDLE, &args![object, handle]).u32();
            let allocator = router_allocator(e, 0x10);
            let size = e.get(in_progress, HkArrayBase::m_size);
            let capacity = e.get(in_progress, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
            if size as u32 == capacity {
                e.call(ARRAY_GROW, &args![allocator, in_progress, 8u32]);
            }
            let size = e.get(in_progress, HkArrayBase::m_size);
            let data = e.get(in_progress, HkArrayBase::m_data).addr();
            let entry = data.wrapping_add((size as u32).wrapping_mul(8));
            if entry != 0 {
                let (low, high) = (e.mem.u32(found), e.mem.u32(found + 4));
                e.mem.set_u32(entry, low);
                e.mem.set_u32(entry + 4, high);
            }
            e.set(in_progress, HkArrayBase::m_size, size.wrapping_add(1));
        }
        let registry = e.get(this, Copier::registry).addr();
        let class_ret = e.call(DATA_OBJECT_GET_CLASS, &args![object, class_a]).u32();
        let class_name = e.call(DATA_CLASS_GET_NAME, &args![class_ret]).u32();
        let mut class = e.vcall(registry, 0x10, &args![class_name]).u32();
        e.call(DATA_CLASS_DESTRUCT, &args![class_a]);
        let found = e.call(DATA_OBJECT_GET_HANDLE, &args![object, handle]).u32();
        let (low, high) = (e.mem.u32(found), e.mem.u32(found + 4));
        let objects = this.at(Copier::objects);
        let mut native = hk_map_base_get_with_default(e, objects, low, high, 0);
        // Whether the block (found or new) goes on to the reference counting.
        let mut counted = true;
        if native == 0 {
            let data_class = e.call(DATA_OBJECT_GET_CLASS, &args![object, class_c]).u32();
            e.call(DATA_CLASS_GET_VERSION, &args![data_class]);
            e.call(CLASS_GET_DESCRIBED_VERSION, &args![class]);
            e.call(DATA_CLASS_DESTRUCT, &args![class_c]);
            let size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32();
            let allocator = router_allocator(e, 0x10);
            native = e.vcall(allocator, 4, &args![size]).u32();
            let packfile = e.get(this, Copier::packfile);
            hk_packfile_data_add_chunk(e, packfile, native, size, 6);
            let object_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32();
            e.call(MEM_SET, &args![native, 0u32, object_size]);
            let found = e
                .call(DATA_OBJECT_GET_HANDLE, &args![object, handle_2])
                .u32();
            let (low, high) = (e.mem.u32(found), e.mem.u32(found + 4));
            let allocator = router_allocator(e, 0x10);
            fn_00dc3600(e, objects, Ptr::new(allocator), low, high, native);
            if copier_fill_native_members(e, this, native, object) == 1 {
                native = 0;
                class = 0;
                counted = false;
            } else if native == 0 {
                counted = false;
            }
        }
        if counted {
            if !tracking {
                return (native, class);
            }
            let vtable_ptr = e.call(CLASS_HAS_VTABLE, &args![class, bool_out]).u32();
            if e.mem.u8(vtable_ptr) != 0 {
                let derived = e
                    .call(
                        CLASS_IS_SUPER_CLASS,
                        &args![BASE_CLASS, bool_out + 4, class],
                    )
                    .u32();
                if e.mem.u8(derived) != 0 {
                    let allocator = router_allocator(e, 0x10);
                    let references = this.at(Copier::native_references);
                    let index = e
                        .call(
                            POINTER_MAP_FIND_OR_INSERT_KEY,
                            &args![references, allocator, native, 0u32],
                        )
                        .u32();
                    let table = e.get(references, HkMapBase::m_elem);
                    let slot = table.wrapping_add(index.wrapping_mul(8)).wrapping_add(4);
                    let count = e.mem.u32(slot);
                    e.mem.set_u32(slot, count.wrapping_add(1));
                }
            }
        }
        if tracking {
            let size = e.get(in_progress, HkArrayBase::m_size);
            e.set(in_progress, HkArrayBase::m_size, size.wrapping_sub(1));
        }
        (native, class)
    });
    e.mem.set_u32(out.addr(), pair.0);
    e.mem.set_u32(out.addr() + 4, pair.1);
}

/// The accessors a native member fill takes the member's value through: the `hkDataArray::Value`
/// set or the `hkDataObject::Value` set (same six, other addresses).
struct SimpleAccessors {
    as_int: u32,
    as_real: u32,
    as_vec: u32,
    as_object: u32,
    as_string: u32,
}

const ARRAY_ACCESSORS: SimpleAccessors = SimpleAccessors {
    as_int: ARRAY_VALUE_AS_INT,
    as_real: ARRAY_VALUE_AS_REAL,
    as_vec: ARRAY_VALUE_AS_VEC,
    as_object: ARRAY_VALUE_AS_OBJECT,
    as_string: ARRAY_VALUE_AS_STRING,
};

const OBJECT_ACCESSORS: SimpleAccessors = SimpleAccessors {
    as_int: OBJECT_VALUE_AS_INT,
    as_real: OBJECT_VALUE_AS_REAL,
    as_vec: OBJECT_VALUE_AS_VEC,
    as_object: OBJECT_VALUE_AS_OBJECT,
    as_string: OBJECT_VALUE_AS_STRING,
};

/// Appends the string `copy` to the package's string array (`+0x34` of the package, growth
/// through the router's allocator at `+0x10`).
fn push_package_string(e: &mut Engine, this: Ptr<Copier>, copy: u32) {
    let packfile = e.get(this, Copier::packfile).addr();
    let strings: Ptr<HkArrayBase> = Ptr::new(packfile.wrapping_add(0x34));
    let allocator = router_allocator(e, 0x10);
    let size = e.get(strings, HkArrayBase::m_size);
    let capacity = e.get(strings, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
    if size as u32 == capacity {
        e.call(ARRAY_GROW, &args![allocator, strings, 4u32]);
    }
    let size = e.get(strings, HkArrayBase::m_size);
    let data = e.get(strings, HkArrayBase::m_data).addr();
    e.mem
        .set_u32(data.wrapping_add((size as u32).wrapping_mul(4)), copy);
    e.set(strings, HkArrayBase::m_size, size.wrapping_add(1));
}

/// The body of `Copier::fillNativeSimpleMember` for either kind of `Value`: stores the member
/// value at `value` into the native field at `target`, by the member's subtype (`1` a boolean
/// byte, 2 to 4 a byte, 5 and 6 a 16-bit word, 7, 8 and 0x1e a 32-bit word, 9 and 10 a 64-bit
/// word with the sign extended, 0xb a float, 0xc and 0xd four floats, 0xe, 0xf and 0x10 a 3x3
/// matrix through the converters, 0x11 a 4x4 matrix copy, 0x12 another matrix converter, 0x14 a
/// pointer to the object's native copy (`findOrCopy`), 0x19 an embedded struct
/// (`fillNativeMembers`), 0x1c a pointer and class pair, 0x1d a duplicated string added to the
/// package, 0x20 the high half of a float, 0x21 a duplicated string with bit 0 set when the
/// copier's flag is set). Answers 0, or 1 for a failure or a subtype not handled.
fn fill_native_simple_member(
    e: &mut Engine,
    this: Ptr<Copier>,
    accessors: &SimpleAccessors,
    target: u32,
    subtype: u32,
    value: Ptr,
    member_class: u32,
    flag: u32,
) -> u32 {
    match subtype {
        1 => {
            let number = e.call(accessors.as_int, &args![value]).u32();
            e.mem.set_u8(target, u8::from(number != 0));
            0
        }
        2..=4 => {
            let number = e.call(accessors.as_int, &args![value]).u32();
            e.mem.set_u8(target, number as u8);
            0
        }
        5 | 6 => {
            let number = e.call(accessors.as_int, &args![value]).u32();
            e.mem.set_u16(target, number as u16);
            0
        }
        7 | 8 | 0x1e => {
            let number = e.call(accessors.as_int, &args![value]).u32();
            e.mem.set_u32(target, number);
            0
        }
        9 | 10 => {
            let number = e.call(accessors.as_int, &args![value]).i32();
            e.mem.set_i32(target, number);
            e.mem.set_i32(target + 4, number >> 31);
            0
        }
        0xb => {
            let real = e.call(accessors.as_real, &args![value]).f32();
            e.mem.set_f32(target, x87_quiet(real));
            0
        }
        0xc | 0xd => {
            let vector = e.call(accessors.as_vec, &args![value, 4u32]).u32();
            copy_bytes(e, target, vector, 16);
            0
        }
        0xe => {
            let vector = e.call(accessors.as_vec, &args![value, 0xcu32]).u32();
            e.call(CONVERT_MATRIX_3, &args![target, vector]);
            0
        }
        0xf | 0x10 => {
            let vector = e.call(accessors.as_vec, &args![value, 0xcu32]).u32();
            e.call(CONVERT_MATRIX_3_B, &args![target, vector]);
            0
        }
        0x11 => {
            let vector = e.call(accessors.as_vec, &args![value, 0x10u32]).u32();
            fn_00dc3710(e, Ptr::new(target), Ptr::new(vector));
            0
        }
        0x12 => {
            let vector = e.call(accessors.as_vec, &args![value, 0x10u32]).u32();
            e.call(CONVERT_MATRIX_4, &args![target, vector]);
            0
        }
        0x14 => {
            if member_class == 0 {
                return 0;
            }
            e.with_stack(0xc, |e, frame| {
                let object = frame.addr();
                let pair = frame.addr() + 4;
                e.call(accessors.as_object, &args![value, object]);
                if e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() != 0 {
                    e.call(DATA_OBJECT_DESTRUCT, &args![object]);
                    return 0;
                }
                copier_find_or_copy(e, this, Ptr::new(pair), Ptr::new(object), flag);
                let native = e.mem.u32(pair);
                if native == 0 {
                    e.call(DATA_OBJECT_DESTRUCT, &args![object]);
                    return 1;
                }
                e.mem.set_u32(target, native);
                e.call(DATA_OBJECT_DESTRUCT, &args![object]);
                0
            })
        }
        0x19 => {
            if member_class == 0 {
                return 0;
            }
            e.with_stack(4, |e, object| {
                let converted = e.call(accessors.as_object, &args![value, object]);
                let failed = copier_fill_native_members(e, this, target, converted.ptr()) == 1;
                e.call(DATA_OBJECT_DESTRUCT, &args![object]);
                u32::from(failed)
            })
        }
        0x1c => {
            if e.get(this, Copier::flag) != 0 {
                return 1;
            }
            e.with_stack(0xc, |e, frame| {
                let object = frame.addr();
                let pair = frame.addr() + 4;
                e.call(accessors.as_object, &args![value, object]);
                if e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() == 0 {
                    copier_find_or_copy(e, this, Ptr::new(pair), Ptr::new(object), flag);
                    let native = e.mem.u32(pair);
                    if native == 0 {
                        e.call(DATA_OBJECT_DESTRUCT, &args![object]);
                        return 1;
                    }
                    let class = e.mem.u32(pair + 4);
                    e.mem.set_u32(target, native);
                    e.mem.set_u32(target + 4, class);
                }
                e.call(DATA_OBJECT_DESTRUCT, &args![object]);
                0
            })
        }
        0x1d => {
            if e.get(this, Copier::flag) != 0 {
                return 1;
            }
            let mut copy = 0u32;
            if e.call(accessors.as_string, &args![value]).u32() != 0 {
                let string = e.call(accessors.as_string, &args![value]).u32();
                copy = e.call(STRING_DUP, &args![string]).u32();
                push_package_string(e, this, copy);
            }
            e.mem.set_u32(target, copy);
            0
        }
        0x20 => {
            let real = e.call(accessors.as_real, &args![value]).f32();
            e.mem
                .set_u16(target, (x87_quiet(real).to_bits() >> 16) as u16);
            0
        }
        0x21 => {
            let mut copy = 0u32;
            if e.call(accessors.as_string, &args![value]).u32() != 0 {
                let string = e.call(accessors.as_string, &args![value]).u32();
                copy = e.call(STRING_DUP, &args![string]).u32();
                push_package_string(e, this, copy);
            }
            e.mem.set_u32(target, copy);
            if e.get(this, Copier::flag) != 0 {
                e.mem.set_u32(target, copy | 1);
            }
            0
        }
        _ => 1,
    }
}

// Translated from 00dc4a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::fillNativeSimpleMember<hkDataArray::Value>` (engine map name): stores the element
/// `value` of an array into the native field at `target`, by the member's `subtype` (see
/// `fill_native_simple_member`; the accessors are `hkDataArray::Value`'s). `member_class` is the
/// member's class and `flag` the member's flag word; answers 0 or 1.
pub fn copier_fill_native_simple_member_array_value(
    e: &mut Engine,
    this: Ptr<Copier>,
    target: u32,
    subtype: u32,
    value: Ptr,
    member_class: u32,
    flag: u32,
) -> u32 {
    fill_native_simple_member(
        e,
        this,
        &ARRAY_ACCESSORS,
        target,
        subtype,
        value,
        member_class,
        flag,
    )
}

// Translated from 00dc4fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::fillNativeSimpleMember<hkDataObject::Value>` (engine map name): stores the member
/// value `value` of an object into the native field at `target`, by the member's `subtype` (see
/// `fill_native_simple_member`; the accessors are `hkDataObject::Value`'s). `member_class` is the
/// member's class and `flag` the member's flag word; answers 0 or 1.
pub fn copier_fill_native_simple_member_object_value(
    e: &mut Engine,
    this: Ptr<Copier>,
    target: u32,
    subtype: u32,
    value: Ptr,
    member_class: u32,
    flag: u32,
) -> u32 {
    fill_native_simple_member(
        e,
        this,
        &OBJECT_ACCESSORS,
        target,
        subtype,
        value,
        member_class,
        flag,
    )
}

// Translated from 00dc2d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::fillNativeMembers` (engine map name): fills the native struct at `target` from the
/// data object `object`, member by member: the object's class is found by name through the
/// registry (slot `0x10`) and its declared members are walked, then those of each parent class.
/// A member the object does not have is skipped (an `hkArray` member, type 0x16, gets the
/// capacity word `0x80000000`). By the member's type: 1 to 0x12, 0x19, 0x1c to 0x1e, 0x20 and
/// 0x21 simple values (one, or each element of a C-style array), 0x14 pointers (strings of
/// subtype 2, objects of subtype 0x19), 0x16 and 0x1a arrays (`copyIntoNativeArray`), 0x18 and
/// 0x1f enumerations (`00dc1bf0`), 0x1b a pointer array with its class and count; the types
/// 0x13, 0x15 and 0x17 and any above 0x21 stop the copy with 1, as does any step answering 1.
/// Answers 0 when everything was copied.
pub fn copier_fill_native_members(
    e: &mut Engine,
    this: Ptr<Copier>,
    target: u32,
    object: Ptr,
) -> u32 {
    const CLASS_TEMP: u32 = 0x00;
    const VALUE: u32 = 0x08;
    const ARRAY_TEMP: u32 = 0x10;
    const OBJECT_TEMP: u32 = 0x14;
    const ELEMENT: u32 = 0x18;
    const ELEMENT_2: u32 = 0x20;
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let mut result = 0u32;
        let class_ret = e
            .call(DATA_OBJECT_GET_CLASS, &args![object, f + CLASS_TEMP])
            .u32();
        let class_name = e.call(DATA_CLASS_GET_NAME, &args![class_ret]).u32();
        e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS_TEMP]);
        let registry = e.get(this, Copier::registry).addr();
        let mut class = e.vcall(registry, 0x10, &args![class_name]).u32();
        while class != 0 {
            let mut member_number = 0i32;
            if e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32() > 0 {
                loop {
                    let member = e
                        .call(CLASS_GET_DECLARED_MEMBER, &args![class, member_number])
                        .u32();
                    let member_ptr: Ptr<HkClassMember> = Ptr::new(member);
                    let field =
                        target.wrapping_add(u32::from(e.get(member_ptr, HkClassMember::m_offset)));
                    let name = e.get(member_ptr, HkClassMember::m_name);
                    let member_type = u32::from(e.get(member_ptr, HkClassMember::m_type));
                    if e.call(DATA_OBJECT_HAS_MEMBER, &args![object, name]).u32() == 0 {
                        if member_type == 0x16 {
                            e.mem.set_u32(field + 8, 0x8000_0000);
                        }
                    } else {
                        let value = f + VALUE;
                        e.call(DATA_OBJECT_MEMBER_VALUE, &args![object, value, name]);
                        let mut array_size =
                            e.call(MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member]).i32();
                        if array_size == 0 {
                            array_size = 1;
                        }
                        let element_size =
                            e.call(MEMBER_GET_SIZE_IN_BYTES, &args![member]).i32() / array_size;
                        let flag = u32::from((!e.get(member_ptr, HkClassMember::m_flags) >> 9) & 1);
                        match member_type {
                            1..=0x12 | 0x19 | 0x1c | 0x1d | 0x1e | 0x20 | 0x21 => {
                                if array_size == 1 {
                                    let member_class =
                                        e.call(MEMBER_GET_CLASS, &args![member]).u32();
                                    result = fill_native_simple_member(
                                        e,
                                        this,
                                        &OBJECT_ACCESSORS,
                                        field,
                                        member_type,
                                        Ptr::new(value),
                                        member_class,
                                        flag,
                                    );
                                } else {
                                    e.call(OBJECT_VALUE_AS_ARRAY, &args![value, f + ARRAY_TEMP]);
                                    let mut element_number = 0i32;
                                    let mut destination = field;
                                    let mut size =
                                        e.call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_TEMP]).i32();
                                    if size > 0 {
                                        loop {
                                            if result != 0 {
                                                break;
                                            }
                                            let member_class =
                                                e.call(MEMBER_GET_CLASS, &args![member]).u32();
                                            let element = e
                                                .call(
                                                    DATA_ARRAY_ELEMENT,
                                                    &args![
                                                        f + ARRAY_TEMP,
                                                        f + ELEMENT,
                                                        element_number
                                                    ],
                                                )
                                                .u32();
                                            let done = fill_native_simple_member(
                                                e,
                                                this,
                                                &ARRAY_ACCESSORS,
                                                destination,
                                                member_type,
                                                Ptr::new(element),
                                                member_class,
                                                flag,
                                            );
                                            element_number += 1;
                                            destination =
                                                destination.wrapping_add(element_size as u32);
                                            result = done;
                                            size = e
                                                .call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_TEMP])
                                                .i32();
                                            if element_number >= size {
                                                break;
                                            }
                                        }
                                    }
                                    e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_TEMP]);
                                }
                            }
                            0x14 => {
                                let subtype =
                                    u32::from(e.get(member_ptr, HkClassMember::m_subtype));
                                if subtype == 2 {
                                    if array_size == 1 {
                                        result = fill_native_simple_member(
                                            e,
                                            this,
                                            &OBJECT_ACCESSORS,
                                            field,
                                            0x1d,
                                            Ptr::new(value),
                                            0,
                                            flag,
                                        );
                                    } else {
                                        e.call(
                                            OBJECT_VALUE_AS_ARRAY,
                                            &args![value, f + ARRAY_TEMP],
                                        );
                                        let mut element_number = 0i32;
                                        let mut destination = field;
                                        let size = e
                                            .call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_TEMP])
                                            .i32();
                                        if size > 0 {
                                            loop {
                                                if result != 0 {
                                                    break;
                                                }
                                                let element = e
                                                    .call(
                                                        DATA_ARRAY_ELEMENT,
                                                        &args![
                                                            f + ARRAY_TEMP,
                                                            f + ELEMENT,
                                                            element_number
                                                        ],
                                                    )
                                                    .u32();
                                                let done = fill_native_simple_member(
                                                    e,
                                                    this,
                                                    &ARRAY_ACCESSORS,
                                                    destination,
                                                    0x1d,
                                                    Ptr::new(element),
                                                    0,
                                                    flag,
                                                );
                                                destination =
                                                    destination.wrapping_add(element_size as u32);
                                                result = done;
                                                element_number += 1;
                                                let size = e
                                                    .call(
                                                        DATA_ARRAY_GET_SIZE,
                                                        &args![f + ARRAY_TEMP],
                                                    )
                                                    .i32();
                                                if element_number >= size {
                                                    break;
                                                }
                                            }
                                        }
                                        e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_TEMP]);
                                    }
                                } else if subtype == 0x19 {
                                    if array_size == 1 {
                                        let object_ptr = e
                                            .call(
                                                OBJECT_VALUE_AS_OBJECT,
                                                &args![value, f + OBJECT_TEMP],
                                            )
                                            .u32();
                                        let member_class = copier_get_member_class_and_check(
                                            e,
                                            this,
                                            member_ptr,
                                            Ptr::new(object_ptr),
                                        );
                                        result = fill_native_simple_member(
                                            e,
                                            this,
                                            &OBJECT_ACCESSORS,
                                            field,
                                            member_type,
                                            Ptr::new(value),
                                            member_class,
                                            flag,
                                        );
                                        e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                                    } else {
                                        e.call(
                                            OBJECT_VALUE_AS_ARRAY,
                                            &args![value, f + ARRAY_TEMP],
                                        );
                                        let mut element_number = 0i32;
                                        let mut destination = field;
                                        let size = e
                                            .call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_TEMP])
                                            .i32();
                                        if size > 0 {
                                            loop {
                                                if result != 0 {
                                                    break;
                                                }
                                                let first = e
                                                    .call(
                                                        DATA_ARRAY_ELEMENT,
                                                        &args![
                                                            f + ARRAY_TEMP,
                                                            f + ELEMENT,
                                                            element_number
                                                        ],
                                                    )
                                                    .u32();
                                                let object_ptr = e
                                                    .call(
                                                        ARRAY_VALUE_AS_OBJECT,
                                                        &args![first, f + OBJECT_TEMP],
                                                    )
                                                    .u32();
                                                let member_class =
                                                    copier_get_member_class_and_check(
                                                        e,
                                                        this,
                                                        member_ptr,
                                                        Ptr::new(object_ptr),
                                                    );
                                                let second = e
                                                    .call(
                                                        DATA_ARRAY_ELEMENT,
                                                        &args![
                                                            f + ARRAY_TEMP,
                                                            f + ELEMENT_2,
                                                            element_number
                                                        ],
                                                    )
                                                    .u32();
                                                result = fill_native_simple_member(
                                                    e,
                                                    this,
                                                    &ARRAY_ACCESSORS,
                                                    destination,
                                                    member_type,
                                                    Ptr::new(second),
                                                    member_class,
                                                    flag,
                                                );
                                                e.call(
                                                    DATA_OBJECT_DESTRUCT,
                                                    &args![f + OBJECT_TEMP],
                                                );
                                                element_number += 1;
                                                destination =
                                                    destination.wrapping_add(element_size as u32);
                                                let size = e
                                                    .call(
                                                        DATA_ARRAY_GET_SIZE,
                                                        &args![f + ARRAY_TEMP],
                                                    )
                                                    .i32();
                                                if element_number >= size {
                                                    break;
                                                }
                                            }
                                        }
                                        e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_TEMP]);
                                    }
                                }
                            }
                            0x16 | 0x1a => {
                                if e.get(this, Copier::flag) != 0 {
                                    let subtype =
                                        u32::from(e.get(member_ptr, HkClassMember::m_subtype));
                                    if member_type == 0x1a || subtype == 0x1c || subtype == 0x1d {
                                        return 1;
                                    }
                                }
                                let array = e
                                    .call(OBJECT_VALUE_AS_ARRAY, &args![value, f + ARRAY_TEMP])
                                    .u32();
                                result = copier_copy_into_native_array(
                                    e,
                                    this,
                                    Ptr::new(field),
                                    member_ptr,
                                    Ptr::new(array),
                                );
                                e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_TEMP]);
                            }
                            0x18 | 0x1f => {
                                result = fn_00dc1bf0(e, field, member_ptr, Ptr::new(value), class);
                            }
                            0x1b => {
                                if e.get(this, Copier::flag) != 0 {
                                    return 1;
                                }
                                e.call(OBJECT_VALUE_AS_ARRAY, &args![value, f + ARRAY_TEMP]);
                                let first = e
                                    .call(
                                        DATA_ARRAY_ELEMENT,
                                        &args![f + ARRAY_TEMP, f + ELEMENT, 0i32],
                                    )
                                    .u32();
                                let object_ptr = e
                                    .call(ARRAY_VALUE_AS_OBJECT, &args![first, f + OBJECT_TEMP])
                                    .u32();
                                let member_class = copier_get_member_class_and_check(
                                    e,
                                    this,
                                    member_ptr,
                                    Ptr::new(object_ptr),
                                );
                                e.mem.set_u32(field, member_class);
                                e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                                let count =
                                    e.call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_TEMP]).i32();
                                e.mem.set_i32(field + 8, count);
                                let class_of_members = e.mem.u32(field);
                                let object_size = e
                                    .call(CLASS_GET_OBJECT_SIZE, &args![class_of_members])
                                    .i32();
                                let total = e.mem.i32(field + 8).wrapping_mul(object_size);
                                let allocator = router_allocator(e, 0x10);
                                let (block, granted) = e.with_stack(4, |e, request| {
                                    e.mem.set_i32(request.addr(), total);
                                    let block = e.vcall(allocator, 0xc, &args![request]).u32();
                                    (block, e.mem.i32(request.addr()))
                                });
                                e.mem.set_u32(field + 4, block);
                                e.call(MEM_SET, &args![block, 0u32, granted]);
                                let packfile = e.get(this, Copier::packfile);
                                let chunk_size = e.mem.i32(field + 8).wrapping_mul(object_size);
                                hk_packfile_data_add_chunk(
                                    e,
                                    packfile,
                                    block,
                                    chunk_size as u32,
                                    6,
                                );
                                let mut index = 0i32;
                                let mut offset = 0u32;
                                if e.mem.i32(field + 8) > 0 {
                                    loop {
                                        if result != 0 {
                                            break;
                                        }
                                        let native = e.mem.u32(field + 4).wrapping_add(offset);
                                        let element = e
                                            .call(
                                                DATA_ARRAY_ELEMENT,
                                                &args![f + ARRAY_TEMP, f + ELEMENT_2, index],
                                            )
                                            .u32();
                                        let converted = e.call(
                                            ARRAY_VALUE_AS_OBJECT,
                                            &args![element, f + OBJECT_TEMP],
                                        );
                                        result = copier_fill_native_members(
                                            e,
                                            this,
                                            native,
                                            converted.ptr(),
                                        );
                                        e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                                        offset = offset.wrapping_add(object_size as u32);
                                        index += 1;
                                        if index >= e.mem.i32(field + 8) {
                                            break;
                                        }
                                    }
                                }
                                e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_TEMP]);
                            }
                            _ => return 1,
                        }
                        if result == 1 {
                            return 1;
                        }
                    }
                    member_number += 1;
                    if member_number >= e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32()
                    {
                        break;
                    }
                }
            }
            class = e.call(CLASS_GET_PARENT, &args![class]).u32();
        }
        result
    })
}

/// The block every simple case of `copyIntoNativeArray` starts with: `dest.size` becomes the
/// array's size, a block of that many elements of `element_size` bytes is taken from the router's
/// allocator at `+0x10` (slot `0xc` is given the byte count and leaves the granted one), and
/// `dest` gets the block and the granted bytes over `element_size` as its size.
fn alloc_elements(e: &mut Engine, dest: Ptr<HkArrayBase>, array: Ptr, element_size: u32) {
    let count = e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32();
    e.set(dest, HkArrayBase::m_size, count);
    let allocator = router_allocator(e, 0x10);
    let (block, granted) = e.with_stack(4, |e, request| {
        e.mem
            .set_i32(request.addr(), count.wrapping_mul(element_size as i32));
        let block = e.vcall(allocator, 0xc, &args![request]).u32();
        (block, e.mem.i32(request.addr()))
    });
    e.set(dest, HkArrayBase::m_size, granted / element_size as i32);
    e.set(dest, HkArrayBase::m_data, Ptr::new(block));
}

/// How an element of a simple subtype is stored in the native array.
#[derive(Clone, Copy)]
enum NativeStore {
    Bool,
    Byte,
    Word,
    Dword,
    Qword,
    Float,
    Half,
    Vector,
    Matrix { convert: u32, floats: u32 },
    Matrix4,
}

/// The element size and the way of storing it for the simple subtypes of `copyIntoNativeArray`.
fn native_store(subtype: u32) -> Option<(u32, NativeStore)> {
    Some(match subtype {
        1 => (1, NativeStore::Bool),
        2..=4 => (1, NativeStore::Byte),
        5 | 6 => (2, NativeStore::Word),
        7 | 8 | 0x1e => (4, NativeStore::Dword),
        9 | 10 => (8, NativeStore::Qword),
        0xb => (4, NativeStore::Float),
        0xc | 0xd => (16, NativeStore::Vector),
        0xe => (
            0x30,
            NativeStore::Matrix {
                convert: CONVERT_MATRIX_3,
                floats: 0xc,
            },
        ),
        0xf | 0x10 => (
            0x30,
            NativeStore::Matrix {
                convert: CONVERT_MATRIX_3_B,
                floats: 0xc,
            },
        ),
        0x11 => (0x40, NativeStore::Matrix4),
        0x12 => (
            0x40,
            NativeStore::Matrix {
                convert: CONVERT_MATRIX_4,
                floats: 0x10,
            },
        ),
        0x20 => (2, NativeStore::Half),
        _ => return None,
    })
}

/// Stores the array element `element` (an `hkDataArray::Value`) as element `index` of the native
/// array `dest`.
fn store_native_element(
    e: &mut Engine,
    store: NativeStore,
    element: u32,
    dest: Ptr<HkArrayBase>,
    index: i32,
    element_size: u32,
) {
    let offset = (index as u32).wrapping_mul(element_size);
    match store {
        NativeStore::Bool => {
            let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem
                .set_u8(data.wrapping_add(offset), u8::from(number != 0));
        }
        NativeStore::Byte => {
            let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem.set_u8(data.wrapping_add(offset), number as u8);
        }
        NativeStore::Word => {
            let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem.set_u16(data.wrapping_add(offset), number as u16);
        }
        NativeStore::Dword => {
            let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem.set_u32(data.wrapping_add(offset), number);
        }
        NativeStore::Qword => {
            let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).i32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem.set_i32(data.wrapping_add(offset), number);
            e.mem
                .set_i32(data.wrapping_add(offset).wrapping_add(4), number >> 31);
        }
        NativeStore::Float => {
            let real = e.call(ARRAY_VALUE_AS_REAL, &args![element]).f32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem.set_f32(data.wrapping_add(offset), x87_quiet(real));
        }
        NativeStore::Half => {
            let real = e.call(ARRAY_VALUE_AS_REAL, &args![element]).f32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.mem.set_u16(
                data.wrapping_add(offset),
                (x87_quiet(real).to_bits() >> 16) as u16,
            );
        }
        NativeStore::Vector => {
            let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, 4u32]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            copy_bytes(e, data.wrapping_add(offset), vector, 16);
        }
        NativeStore::Matrix { convert, floats } => {
            let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, floats]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            e.call(convert, &args![data.wrapping_add(offset), vector]);
        }
        NativeStore::Matrix4 => {
            let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, 0x10u32]).u32();
            let data = e.get(dest, HkArrayBase::m_data).addr();
            fn_00dc3710(e, Ptr::new(data.wrapping_add(offset)), Ptr::new(vector));
        }
    }
}

/// The element loop of the string subtypes (0x1d and 0x21) of `copyIntoNativeArray`: while the
/// result is still 0, each element goes through `fillNativeSimpleMember` into its 4-byte slot of
/// `dest`.
fn copy_native_array_strings(
    e: &mut Engine,
    this: Ptr<Copier>,
    dest: Ptr<HkArrayBase>,
    array: Ptr,
    subtype: u32,
    flag: u32,
    value: u32,
    result: &mut u32,
) {
    let mut index = 0i32;
    if e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() > 0 {
        loop {
            if *result != 0 {
                break;
            }
            let data = e.get(dest, HkArrayBase::m_data).addr();
            let target = data.wrapping_add((index as u32).wrapping_mul(4));
            let element = e
                .call(DATA_ARRAY_ELEMENT_OV2, &args![array, value, index])
                .u32();
            *result = fill_native_simple_member(
                e,
                this,
                &ARRAY_ACCESSORS,
                target,
                subtype,
                Ptr::new(element),
                0,
                flag,
            );
            index += 1;
            if index >= e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() {
                break;
            }
        }
    }
}

// Translated from 00dc1d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::copyIntoNativeArray` (engine map name): copies the data array `array` into the
/// native `hkArray` at `dest` for the member `member`. An empty array only sets the capacity word
/// of an `hkArray` member (type 0x16) to `0x80000000` and answers 0. Otherwise by the member's
/// subtype: the simple subtypes (1 to 0x12, 0x1e, 0x20) take a block of the array's size from the
/// router's allocator and store each element (see `native_store`), 0x14 and 0x1c store the
/// native copies of the objects (`findOrCopy`, 0x1c with their classes), 0x19 allocates the
/// structs of the class `getMemberClassAndCheck` finds for the first element and fills each
/// (`fillNativeMembers`), 0x1d and 0x21 store duplicated strings (0x1d fills its array twice:
/// the exe falls through from its loop into the body of 0x21, which allocates and fills again);
/// subtype 0 does nothing and the others answer 1. The block is then added to the package as a
/// chunk of kind `0x18` (size times the element size) and, for an `hkArray` member, the capacity
/// word becomes the size (with bit 31 set unless the copier's flag is set). Answers the result of
/// the last element, 0 when all went well.
pub fn copier_copy_into_native_array(
    e: &mut Engine,
    this: Ptr<Copier>,
    dest: Ptr<HkArrayBase>,
    member: Ptr<HkClassMember>,
    array: Ptr,
) -> u32 {
    const VALUE: u32 = 0x00;
    const OBJECT_TEMP: u32 = 0x08;
    const PAIR: u32 = 0x10;
    if e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() == 0 {
        if e.get(member, HkClassMember::m_type) == 0x16 {
            e.set(dest, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        }
        return 0;
    }
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        let flag = u32::from((!e.get(member, HkClassMember::m_flags) >> 9) & 1);
        let subtype = u32::from(e.get(member, HkClassMember::m_subtype));
        if subtype > 0x21 {
            return 1;
        }
        let mut result = 0u32;
        let mut element_size = 0i32;
        let add_chunk = match subtype {
            0 => false,
            0x14 | 0x1c => {
                let size = if subtype == 0x14 { 4 } else { 8 };
                alloc_elements(e, dest, array, size);
                element_size = size as i32;
                let count = e.get(dest, HkArrayBase::m_size);
                let data = e.get(dest, HkArrayBase::m_data).addr();
                e.call(MEM_SET, &args![data, 0u32, count.wrapping_mul(size as i32)]);
                let mut index = 0i32;
                if e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() > 0 {
                    loop {
                        let element = e
                            .call(DATA_ARRAY_ELEMENT_OV2, &args![array, f + VALUE, index])
                            .u32();
                        e.call(ARRAY_VALUE_AS_OBJECT, &args![element, f + OBJECT_TEMP]);
                        if e.call(DATA_OBJECT_IS_NULL, &args![f + OBJECT_TEMP]).u32() == 0 {
                            copier_find_or_copy(
                                e,
                                this,
                                Ptr::new(f + PAIR),
                                Ptr::new(f + OBJECT_TEMP),
                                flag,
                            );
                            let native = e.mem.u32(f + PAIR);
                            if native == 0 {
                                result = 1;
                            } else {
                                let data = e.get(dest, HkArrayBase::m_data).addr();
                                let slot = data.wrapping_add((index as u32).wrapping_mul(size));
                                e.mem.set_u32(slot, native);
                                if subtype == 0x1c {
                                    let class = e.mem.u32(f + PAIR + 4);
                                    e.mem.set_u32(slot + 4, class);
                                }
                            }
                        }
                        e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                        index += 1;
                        if index >= e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() {
                            break;
                        }
                    }
                }
                true
            }
            0x19 => {
                let first = e
                    .call(DATA_ARRAY_ELEMENT_OV2, &args![array, f + VALUE, 0i32])
                    .u32();
                let object = e
                    .call(ARRAY_VALUE_AS_OBJECT, &args![first, f + OBJECT_TEMP])
                    .u32();
                let class = copier_get_member_class_and_check(e, this, member, Ptr::new(object));
                e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                if class == 0 {
                    false
                } else {
                    let count = e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32();
                    e.set(dest, HkArrayBase::m_size, count);
                    let object_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
                    let total = object_size.wrapping_mul(e.get(dest, HkArrayBase::m_size));
                    let allocator = router_allocator(e, 0x10);
                    let block = e.with_stack(4, |e, request| {
                        e.mem.set_i32(request.addr(), total);
                        e.vcall(allocator, 0xc, &args![request]).u32()
                    });
                    e.set(dest, HkArrayBase::m_data, Ptr::new(block));
                    element_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
                    let object_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
                    let count = e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32();
                    let data = e.get(dest, HkArrayBase::m_data).addr();
                    e.call(MEM_SET, &args![data, 0u32, object_size.wrapping_mul(count)]);
                    let mut index = 0i32;
                    if e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() > 0 {
                        loop {
                            if result != 0 {
                                break;
                            }
                            let size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
                            let data = e.get(dest, HkArrayBase::m_data).addr();
                            let target = data.wrapping_add(size.wrapping_mul(index) as u32);
                            let element = e
                                .call(DATA_ARRAY_ELEMENT_OV2, &args![array, f + VALUE, index])
                                .u32();
                            let object =
                                e.call(ARRAY_VALUE_AS_OBJECT, &args![element, f + OBJECT_TEMP]);
                            result = copier_fill_native_members(e, this, target, object.ptr());
                            e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                            index += 1;
                            if index >= e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() {
                                break;
                            }
                        }
                    }
                    element_size > 0
                }
            }
            0x1d => {
                alloc_elements(e, dest, array, 4);
                copy_native_array_strings(
                    e,
                    this,
                    dest,
                    array,
                    subtype,
                    flag,
                    f + VALUE,
                    &mut result,
                );
                // The exe falls through into the body of subtype 0x21.
                alloc_elements(e, dest, array, 4);
                element_size = 4;
                copy_native_array_strings(
                    e,
                    this,
                    dest,
                    array,
                    subtype,
                    flag,
                    f + VALUE,
                    &mut result,
                );
                true
            }
            0x21 => {
                alloc_elements(e, dest, array, 4);
                element_size = 4;
                copy_native_array_strings(
                    e,
                    this,
                    dest,
                    array,
                    subtype,
                    flag,
                    f + VALUE,
                    &mut result,
                );
                true
            }
            _ => match native_store(subtype) {
                Some((size, store)) => {
                    alloc_elements(e, dest, array, size);
                    element_size = size as i32;
                    let mut index = 0i32;
                    if e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() > 0 {
                        loop {
                            let element = e
                                .call(DATA_ARRAY_ELEMENT_OV2, &args![array, f + VALUE, index])
                                .u32();
                            store_native_element(e, store, element, dest, index, size);
                            index += 1;
                            if index >= e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32() {
                                break;
                            }
                        }
                    }
                    true
                }
                None => return 1,
            },
        };
        if add_chunk {
            let size = e.get(dest, HkArrayBase::m_size);
            let data = e.get(dest, HkArrayBase::m_data).addr();
            let packfile = e.get(this, Copier::packfile);
            hk_packfile_data_add_chunk(
                e,
                packfile,
                data,
                size.wrapping_mul(element_size) as u32,
                0x18,
            );
        }
        if e.get(member, HkClassMember::m_type) == 0x16 {
            let size = e.get(dest, HkArrayBase::m_size);
            e.set(dest, HkArrayBase::m_capacityAndFlags, size as u32);
            if e.get(this, Copier::flag) == 0 {
                e.set(
                    dest,
                    HkArrayBase::m_capacityAndFlags,
                    (size as u32) | NOT_OWNED,
                );
            }
        }
        result
    })
}

/// The next used slot of a 12-byte-slot map at or after `from` (`m_hashMod + 1`-bounded): the
/// first index whose slot's first word is not `0xffffffff`.
fn next_used_slot(e: &mut Engine, map: Ptr<HkMapBase>, from: i32) -> i32 {
    let last = e.get(map, HkMapBase::m_hashMod);
    let mut index = from;
    if index <= last {
        let table = e.get(map, HkMapBase::m_elem);
        loop {
            let slot = table.wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE));
            if e.mem.u32(slot) != 0xffff_ffff {
                break;
            }
            index += 1;
            if index > last {
                break;
            }
        }
    }
    index
}

// Translated from 00dc4710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Copier::toObject` (engine map name): converts the data object `object` into a native object.
/// A new `hkPackfileData` (0x64 bytes from the router's allocator, `00e05700`) replaces the
/// copier's package; the object is copied (`findOrCopy`, tracking handles); when that gives no
/// native object the package is released and 0 is answered. Otherwise, for every native object
/// the copier made: when its reference count (`+0xc` map) is above 0 the object's `+4` is set to
/// the class's object size and `+6` to the count; then each object is finished through
/// `registry_argument`'s slot `0x10` (native object, class name) and, if the class has a vtable
/// and nothing came back, "Cannot finish virtual object of class ..." is reported through the
/// error reporter. The answer is a resource built from the copy (`00e04f10`, given the copier's
/// registry and `registry_argument`); the package's `+0x30`, `+0x38` and `+0x44` are cleared and
/// it is released.
pub fn copier_to_object(
    e: &mut Engine,
    this: Ptr<Copier>,
    object: Ptr,
    registry_argument: Ptr,
) -> u32 {
    const PAIR: u32 = 0x00;
    const CLASS: u32 = 0x08;
    const KEY: u32 = 0x10;
    const OBJECT_TEMP: u32 = 0x18;
    const CLASS_TEMP: u32 = 0x1c;
    const BOOL_OUT: u32 = 0x20;
    const STREAM: u32 = 0x30;
    const BUFFER: u32 = 0x40;
    let allocator = router_allocator(e, 0x10);
    let block = e.vcall(allocator, 4, &args![0x64u32]).u32();
    e.mem.set_u16(block + 4, 0x64);
    let package = e.call(PACKFILE_DATA_CONSTRUCT, &args![block]).u32();
    let old = e.get(this, Copier::packfile).addr();
    if old != 0 && old != package {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.set(this, Copier::packfile, Ptr::new(package));
    e.with_stack(0x250, |e, frame| {
        let f = frame.addr();
        copier_find_or_copy(e, this, Ptr::new(f + PAIR), object, 1);
        let class_ret = e
            .call(DATA_OBJECT_GET_CLASS, &args![object, f + CLASS])
            .u32();
        let world = e.call(DATA_CLASS_GET_WORLD, &args![class_ret]).u32();
        e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS]);
        if e.mem.u32(f + PAIR) == 0 {
            let package = e.get(this, Copier::packfile).addr();
            e.mem.set_u32(package + 0x30, 0);
            let package = e.get(this, Copier::packfile).addr();
            if package != 0 {
                e.call(REMOVE_REFERENCE, &args![package]);
            }
            e.set(this, Copier::packfile, Ptr::NULL);
            return 0;
        }
        let objects = this.at(Copier::objects);
        // First every native object's header gets its size and reference count.
        let mut index = next_used_slot(e, objects, 0);
        while index <= e.get(objects, HkMapBase::m_hashMod) {
            let table = e.get(objects, HkMapBase::m_elem);
            let slot = table.wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE));
            let (key_low, key_high) = (e.mem.u32(slot), e.mem.u32(slot + 4));
            e.mem.set_u32(f + KEY, key_low);
            e.mem.set_u32(f + KEY + 4, key_high);
            e.vcall(world, 0x28, &args![f + OBJECT_TEMP, f + KEY]);
            let class_ret = e
                .call(
                    DATA_OBJECT_GET_CLASS,
                    &args![f + OBJECT_TEMP, f + CLASS_TEMP],
                )
                .u32();
            let class_name = e.call(DATA_CLASS_GET_NAME, &args![class_ret]).u32();
            e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS_TEMP]);
            let registry = e.get(this, Copier::registry).addr();
            let class = e.vcall(registry, 0x10, &args![class_name]).u32();
            let table = e.get(objects, HkMapBase::m_elem);
            let native = e.mem.u32(
                table
                    .wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE))
                    .wrapping_add(8),
            );
            let references = this.at(Copier::native_references);
            let count = e
                .call(
                    POINTER_MAP_GET_WITH_DEFAULT,
                    &args![references, native, 0u32],
                )
                .u32() as u16;
            if (count as i16) > 0 {
                let size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32();
                e.mem.set_u16(native + 4, size as u16);
                e.mem.set_u16(native + 6, count);
            }
            e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
            index = next_used_slot(e, objects, index + 1);
        }
        // Then every native object is finished.
        let mut index = next_used_slot(e, objects, 0);
        while index <= e.get(objects, HkMapBase::m_hashMod) {
            let table = e.get(objects, HkMapBase::m_elem);
            let slot = table.wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE));
            let (key_low, key_high) = (e.mem.u32(slot), e.mem.u32(slot + 4));
            e.mem.set_u32(f + KEY, key_low);
            e.mem.set_u32(f + KEY + 4, key_high);
            e.vcall(world, 0x28, &args![f + OBJECT_TEMP, f + KEY]);
            let class_ret = e
                .call(
                    DATA_OBJECT_GET_CLASS,
                    &args![f + OBJECT_TEMP, f + CLASS_TEMP],
                )
                .u32();
            let class_name = e.call(DATA_CLASS_GET_NAME, &args![class_ret]).u32();
            e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS_TEMP]);
            let registry = e.get(this, Copier::registry).addr();
            let class = e.vcall(registry, 0x10, &args![class_name]).u32();
            let table = e.get(objects, HkMapBase::m_elem);
            let native = e.mem.u32(
                table
                    .wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE))
                    .wrapping_add(8),
            );
            let name = e.call(CLASS_GET_NAME, &args![class]).u32();
            let finished = e
                .vcall(registry_argument.addr(), 0x10, &args![native, name])
                .u32();
            let has_vtable = e.call(CLASS_HAS_VTABLE, &args![class, f + BOOL_OUT]).u32();
            if e.mem.u8(has_vtable) != 0 && finished == 0 {
                e.call(STREAM_CONSTRUCT, &args![f + STREAM, f + BUFFER, 0x200u32]);
                let name = e.call(CLASS_GET_NAME, &args![class]).u32();
                let first = e
                    .call(STREAM_APPEND, &args![f + STREAM, MESSAGE_HEAD])
                    .u32();
                let second = e.call(STREAM_APPEND, &args![first, name]).u32();
                e.call(STREAM_APPEND, &args![second, MESSAGE_TAIL]);
                let reporter = e.global::<u32>(ERROR_REPORTER);
                e.vcall(
                    reporter,
                    0xc,
                    &args![1u32, 0x7c30_3834u32, f + BUFFER, SOURCE_FILE, 0x133u32],
                );
                e.call(STREAM_DESTRUCT, &args![f + STREAM]);
            }
            e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
            index = next_used_slot(e, objects, index + 1);
        }
        let allocator = router_allocator(e, 0x10);
        let resource_block = e.vcall(allocator, 4, &args![0x18u32]).u32();
        e.mem.set_u16(resource_block + 4, 0x18);
        let resource = e
            .call(RESOURCE_CONSTRUCT, &args![resource_block, f + PAIR])
            .u32();
        let registry = e.get(this, Copier::registry);
        e.call(RESOURCE_SET_REGISTRY, &args![resource, registry]);
        e.call(RESOURCE_SET_ARGUMENT, &args![resource, registry_argument]);
        let package = e.get(this, Copier::packfile).addr();
        e.mem.set_u32(package + 0x30, 0);
        e.mem.set_u32(package + 0x38, 0);
        e.mem.set_u32(package + 0x44, 0);
        let package = e.get(this, Copier::packfile).addr();
        if package != 0 {
            e.call(REMOVE_REFERENCE, &args![package]);
        }
        e.set(this, Copier::packfile, Ptr::NULL);
        resource
    })
}

// Translated from 00dc32c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectUtil::toObjectWithRegistry` (engine map name): builds a `Copier` on the stack over
/// the class registry `registry` (flag 1), converts `object` with `registry_argument`
/// (`Copier::toObject`) and destroys the copier. Answers the conversion's result.
pub fn hk_data_object_util_to_object_with_registry(
    e: &mut Engine,
    object: Ptr,
    registry: Ptr,
    registry_argument: Ptr,
) -> u32 {
    e.with_stack(0x30, |e, block| {
        let copier: Ptr<Copier> = block.cast();
        copier_constructor(e, copier, registry, 1);
        let result = copier_to_object(e, copier, object, registry_argument);
        copier_destructor(e, copier);
        result
    })
}

// Translated from 00dc3300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectUtil::toResourceWithRegistry` (engine map name): builds a `Copier` on the stack
/// over `registry` (flag 0) and copies `object` (`findOrCopy`, no tracking) into a new
/// `hkPackfileData` (0x64 bytes from the router's allocator). A null object, or a copy that
/// yields no native object or no class, gives 0. Otherwise every native object the copier made
/// is registered in the package's name map (`00cb1870`, with the class name found for it) and
/// the package gets the root object and its class name (`00e05100`); the package is returned
/// with a reference added, and the copier's own reference is released.
pub fn hk_data_object_util_to_resource_with_registry(
    e: &mut Engine,
    object: Ptr,
    registry: Ptr,
) -> u32 {
    const PAIR: u32 = 0x00;
    const CLASS: u32 = 0x08;
    const KEY: u32 = 0x10;
    const OBJECT_TEMP: u32 = 0x18;
    const CLASS_TEMP: u32 = 0x1c;
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let copier_block = f + 0x40;
        let copier: Ptr<Copier> = Ptr::new(copier_block);
        copier_constructor(e, copier, registry, 0);
        if e.call(DATA_OBJECT_IS_NULL, &args![object]).u32() != 0 {
            copier_destructor(e, copier);
            return 0;
        }
        let allocator = router_allocator(e, 0x10);
        let block = e.vcall(allocator, 4, &args![0x64u32]).u32();
        e.mem.set_u16(block + 4, 0x64);
        let package = e.call(PACKFILE_DATA_CONSTRUCT, &args![block]).u32();
        let old = e.get(copier, Copier::packfile).addr();
        if old != 0 && old != package {
            e.call(REMOVE_REFERENCE, &args![old]);
        }
        e.set(copier, Copier::packfile, Ptr::new(package));
        copier_find_or_copy(e, copier, Ptr::new(f + PAIR), object, 0);
        let (native, class) = (e.mem.u32(f + PAIR), e.mem.u32(f + PAIR + 4));
        if native == 0 || class == 0 {
            let old = e.get(copier, Copier::packfile).addr();
            if old != 0 {
                e.call(REMOVE_REFERENCE, &args![old]);
            }
            e.set(copier, Copier::packfile, Ptr::NULL);
            copier_destructor(e, copier);
            return 0;
        }
        let class_ret = e
            .call(DATA_OBJECT_GET_CLASS, &args![object, f + CLASS])
            .u32();
        let world = e.call(DATA_CLASS_GET_WORLD, &args![class_ret]).u32();
        e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS]);
        let objects = copier.at(Copier::objects);
        let mut index = fn_00dc35e0(e, objects);
        while index <= e.get(objects, HkMapBase::m_hashMod) {
            let table = e.get(objects, HkMapBase::m_elem);
            let slot = table.wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE));
            let (key_low, key_high) = (e.mem.u32(slot), e.mem.u32(slot + 4));
            e.mem.set_u32(f + KEY, key_low);
            e.mem.set_u32(f + KEY + 4, key_high);
            e.vcall(world, 0x28, &args![f + OBJECT_TEMP, f + KEY]);
            let class_ret = e
                .call(
                    DATA_OBJECT_GET_CLASS,
                    &args![f + OBJECT_TEMP, f + CLASS_TEMP],
                )
                .u32();
            let class_name = e.call(DATA_CLASS_GET_NAME, &args![class_ret]).u32();
            e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS_TEMP]);
            let registry = e.get(copier, Copier::registry).addr();
            let hk_class = e.vcall(registry, 0x10, &args![class_name]).u32();
            let table = e.get(objects, HkMapBase::m_elem);
            let native_object = e.mem.u32(
                table
                    .wrapping_add((index as u32).wrapping_mul(SMALL_SLOT_SIZE))
                    .wrapping_add(8),
            );
            let package = e.get(copier, Copier::packfile).addr();
            let name = e.call(CLASS_GET_NAME, &args![hk_class]).u32();
            let allocator = router_allocator(e, 0x10);
            e.call(
                POINTER_MAP_INSERT_00CB1870,
                &args![package.wrapping_add(0x10), allocator, native_object, name],
            );
            e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
            index = next_used_slot(e, objects, index + 1);
        }
        let package = e.get(copier, Copier::packfile).addr();
        let class_name = e.call(CLASS_GET_NAME, &args![class]).u32();
        e.call(
            PACKFILE_DATA_SET_CONTENTS,
            &args![package, native, class_name],
        );
        let package = e.get(copier, Copier::packfile).addr();
        e.call(ADD_REFERENCE, &args![package]);
        let old = e.get(copier, Copier::packfile).addr();
        if old != 0 {
            e.call(REMOVE_REFERENCE, &args![old]);
        }
        e.set(copier, Copier::packfile, Ptr::NULL);
        copier_destructor(e, copier);
        package
    })
}

// ---- Cloner: data objects into another world ----

/// The accessors a simple value copy goes through: the `Value`'s getters on the source and the
/// setters on the destination (`hkDataArray::Value` or `hkDataObject::Value`).
struct CloneAccessors {
    get_type: u32,
    as_int: u32,
    as_real: u32,
    as_vec: u32,
    as_object: u32,
    as_string: u32,
    set_int: u32,
    set_real: u32,
    set_vec: u32,
    set_object: u32,
    set_string: u32,
}

const ARRAY_CLONE: CloneAccessors = CloneAccessors {
    get_type: ARRAY_VALUE_GET_TYPE,
    as_int: ARRAY_VALUE_AS_INT,
    as_real: ARRAY_VALUE_AS_REAL,
    as_vec: ARRAY_VALUE_AS_VEC,
    as_object: ARRAY_VALUE_AS_OBJECT,
    as_string: ARRAY_VALUE_AS_STRING,
    set_int: ARRAY_VALUE_SET_INT,
    set_real: ARRAY_VALUE_SET_REAL,
    set_vec: ARRAY_VALUE_SET_VEC,
    set_object: ARRAY_VALUE_SET_OBJECT,
    set_string: ARRAY_VALUE_SET_STRING,
};

const OBJECT_CLONE: CloneAccessors = CloneAccessors {
    get_type: OBJECT_VALUE_GET_TYPE,
    as_int: OBJECT_VALUE_AS_INT,
    as_real: OBJECT_VALUE_AS_REAL,
    as_vec: OBJECT_VALUE_AS_VEC,
    as_object: OBJECT_VALUE_AS_OBJECT,
    as_string: OBJECT_VALUE_AS_STRING,
    set_int: OBJECT_VALUE_SET_INT,
    set_real: OBJECT_VALUE_SET_REAL,
    set_vec: OBJECT_VALUE_SET_VEC,
    set_object: OBJECT_VALUE_SET_OBJECT,
    set_string: OBJECT_VALUE_SET_STRING,
};

/// The body of `Cloner::copySimpleValue` for either kind of `Value`: sets the destination
/// `Value` (the two words `destination_implementation` and `destination_index`, handed to the
/// setters as a block) from the source value `source` by its type: 1 and 2 an integer, 3 a real,
/// 4 to 7 a vector of `type * 4 - 12` floats, 8 and 9 a copy of the object (`copyObject`), 10 a
/// string; any other type does nothing.
fn copy_simple_value(
    e: &mut Engine,
    this: Ptr<Cloner>,
    accessors: &CloneAccessors,
    destination_implementation: u32,
    destination_index: u32,
    source: Ptr,
) {
    e.with_stack(0x10, |e, frame| {
        let destination = frame.addr();
        let object_a = frame.addr() + 8;
        let object_b = frame.addr() + 0xc;
        e.mem.set_u32(destination, destination_implementation);
        e.mem.set_u32(destination + 4, destination_index);
        match e.call(accessors.get_type, &args![source]).u32() {
            1 | 2 => {
                let number = e.call(accessors.as_int, &args![source]).u32();
                e.call(accessors.set_int, &args![destination, number]);
            }
            3 => {
                let real = e.call(accessors.as_real, &args![source]).f32();
                e.call(accessors.set_real, &args![destination, x87_quiet(real)]);
            }
            4..=7 => {
                let floats = e
                    .call(accessors.get_type, &args![source])
                    .i32()
                    .wrapping_mul(4)
                    .wrapping_sub(12);
                let vector = e.call(accessors.as_vec, &args![source, floats]).u32();
                e.call(accessors.set_vec, &args![destination, vector, floats]);
            }
            8 | 9 => {
                let object = e.call(accessors.as_object, &args![source, object_a]).u32();
                let copy = cloner_copy_object(e, this, Ptr::new(object_b), Ptr::new(object));
                e.call(accessors.set_object, &args![destination, copy]);
                e.call(DATA_OBJECT_DESTRUCT, &args![object_b]);
                e.call(DATA_OBJECT_DESTRUCT, &args![object_a]);
            }
            10 => {
                let string = e.call(accessors.as_string, &args![source]).u32();
                e.call(accessors.set_string, &args![destination, string]);
            }
            _ => {}
        }
    });
}

// Translated from 00dc37f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::copySimpleValue<hkDataArray::Value>` (engine map name): sets the destination array
/// element (the `hkDataArray::Value` passed by value as `destination_implementation` and
/// `destination_index`) from the source element `source` (see `copy_simple_value`).
pub fn cloner_copy_simple_value_array_value(
    e: &mut Engine,
    this: Ptr<Cloner>,
    destination_implementation: u32,
    destination_index: u32,
    source: Ptr,
) {
    copy_simple_value(
        e,
        this,
        &ARRAY_CLONE,
        destination_implementation,
        destination_index,
        source,
    );
}

// Translated from 00dc5ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::copySimpleValue<hkDataObject::Value>` (engine map name): sets the destination member
/// value (the `hkDataObject::Value` passed by value as `destination_implementation` and
/// `destination_index`) from the source value `source` (see `copy_simple_value`).
pub fn cloner_copy_simple_value_object_value(
    e: &mut Engine,
    this: Ptr<Cloner>,
    destination_implementation: u32,
    destination_index: u32,
    source: Ptr,
) {
    copy_simple_value(
        e,
        this,
        &OBJECT_CLONE,
        destination_implementation,
        destination_index,
        source,
    );
}

// Translated from 00dc3d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::copyArray` (engine map name): copies `count` elements of the data array `source` into
/// the data array `destination` by the array's type: 1 and 2 integers, 3 reals, 4 to 7 vectors of
/// `type * 4 - 12` floats, 8 object copies (`copyObject`, after sizing the destination), 9 structs
/// (the destination is sized, then for every member of the class whose type word has a type, the
/// member's arrays are swizzled out of both arrays and copied: members with the array or tuple
/// bit (0x30) copy each element's own array recursively with the size of the destination's array
/// (type bit 0x10) or the tuple size, other members copy each element through `copySimpleValue`),
/// 10 strings; other types do nothing.
pub fn cloner_copy_array(
    e: &mut Engine,
    this: Ptr<Cloner>,
    destination: Ptr,
    source: Ptr,
    count: i32,
) {
    const VALUE_SOURCE: u32 = 0x00;
    const VALUE_DESTINATION: u32 = 0x08;
    const OBJECT_TEMP: u32 = 0x10;
    const OBJECT_COPY: u32 = 0x14;
    const CLASS: u32 = 0x18;
    const MEMBERS: u32 = 0x20;
    const ARRAY_A: u32 = 0x30;
    const ARRAY_B: u32 = 0x34;
    const ARRAY_C: u32 = 0x38;
    const ARRAY_D: u32 = 0x3c;
    const ARRAY_E: u32 = 0x40;
    const VALUE_A: u32 = 0x48;
    const VALUE_B: u32 = 0x50;
    const VALUE_C: u32 = 0x58;
    const VALUE_X: u32 = 0x60;
    const VALUE_Y: u32 = 0x68;
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let mut lifetime_flag = 0u32;
        match e.call(DATA_ARRAY_GET_TYPE, &args![source]).u32() {
            1 | 2 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(
                            DATA_ARRAY_ELEMENT_OV2,
                            &args![source, f + VALUE_SOURCE, index],
                        )
                        .u32();
                    let number = e.call(ARRAY_VALUE_AS_INT, &args![element]).u32();
                    let target = e
                        .call(
                            DATA_ARRAY_ELEMENT,
                            &args![destination, f + VALUE_DESTINATION, index],
                        )
                        .u32();
                    e.call(ARRAY_VALUE_SET_INT, &args![target, number]);
                }
            }
            3 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(
                            DATA_ARRAY_ELEMENT_OV2,
                            &args![source, f + VALUE_SOURCE, index],
                        )
                        .u32();
                    let real = e.call(ARRAY_VALUE_AS_REAL, &args![element]).f32();
                    let target = e
                        .call(
                            DATA_ARRAY_ELEMENT,
                            &args![destination, f + VALUE_DESTINATION, index],
                        )
                        .u32();
                    e.call(ARRAY_VALUE_SET_REAL, &args![target, x87_quiet(real)]);
                }
            }
            4..=7 => {
                let floats = e
                    .call(DATA_ARRAY_GET_TYPE, &args![source])
                    .i32()
                    .wrapping_mul(4)
                    .wrapping_sub(12);
                for index in 0..count.max(0) {
                    let element = e
                        .call(
                            DATA_ARRAY_ELEMENT_OV2,
                            &args![source, f + VALUE_SOURCE, index],
                        )
                        .u32();
                    let vector = e.call(ARRAY_VALUE_AS_VEC, &args![element, floats]).u32();
                    let target = e
                        .call(
                            DATA_ARRAY_ELEMENT,
                            &args![destination, f + VALUE_DESTINATION, index],
                        )
                        .u32();
                    e.call(ARRAY_VALUE_SET_VEC, &args![target, vector, floats]);
                }
            }
            8 => {
                e.call(DATA_ARRAY_SET_SIZE, &args![destination, count]);
                for index in 0..count.max(0) {
                    let element = e
                        .call(
                            DATA_ARRAY_ELEMENT_OV2,
                            &args![source, f + VALUE_SOURCE, index],
                        )
                        .u32();
                    let object = e
                        .call(ARRAY_VALUE_AS_OBJECT, &args![element, f + OBJECT_TEMP])
                        .u32();
                    let copy =
                        cloner_copy_object(e, this, Ptr::new(f + OBJECT_COPY), Ptr::new(object));
                    let target = e
                        .call(
                            DATA_ARRAY_ELEMENT,
                            &args![destination, f + VALUE_DESTINATION, index],
                        )
                        .u32();
                    e.call(ARRAY_VALUE_SET_OBJECT, &args![target, copy]);
                    e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_COPY]);
                    e.call(DATA_OBJECT_DESTRUCT, &args![f + OBJECT_TEMP]);
                }
            }
            9 => {
                e.call(DATA_ARRAY_SET_SIZE, &args![destination, count]);
                e.call(DATA_ARRAY_GET_CLASS, &args![source, f + CLASS]);
                let member_count = e.call(DATA_CLASS_GET_NUM_MEMBERS, &args![f + CLASS]).i32();
                let members: Ptr<HkArrayBase> = Ptr::new(f + MEMBERS);
                let size = member_info_array_new(e, members, member_count);
                let data = e.get(members, HkArrayBase::m_data).addr();
                zero_member_infos(e, data, size, member_count);
                e.set(members, HkArrayBase::m_size, member_count);
                e.call(DATA_CLASS_GET_ALL_MEMBER_INFO, &args![f + CLASS, members]);
                if member_count > 0 {
                    let mut offset = 0u32;
                    for _ in 0..member_count {
                        let data = e.get(members, HkArrayBase::m_data).addr();
                        let member: Ptr<MemberInfo> = Ptr::new(data.wrapping_add(offset));
                        let type_word = e.get(member, MemberInfo::type_word);
                        if type_word & 0xf != 0 {
                            let name = e.get(member, MemberInfo::name);
                            e.call(
                                DATA_ARRAY_SWIZZLE_OBJECT_MEMBER,
                                &args![destination, f + ARRAY_A, name],
                            );
                            e.call(
                                DATA_ARRAY_SWIZZLE_OBJECT_MEMBER,
                                &args![source, f + ARRAY_B, name],
                            );
                            if type_word & 0x30 != 0 {
                                for index in 0..count.max(0) {
                                    let sub_count: i32;
                                    if type_word & 0x10 != 0 {
                                        lifetime_flag |= 1;
                                        let element = e
                                            .call(
                                                DATA_ARRAY_ELEMENT,
                                                &args![f + ARRAY_A, f + VALUE_A, index],
                                            )
                                            .u32();
                                        let array = e
                                            .call(
                                                ARRAY_VALUE_AS_ARRAY,
                                                &args![element, f + ARRAY_C],
                                            )
                                            .u32();
                                        sub_count =
                                            e.call(DATA_ARRAY_GET_SIZE, &args![array]).i32();
                                    } else {
                                        sub_count = e.get(member, MemberInfo::tuple_size) as i32;
                                    }
                                    if lifetime_flag & 1 != 0 {
                                        lifetime_flag &= !1;
                                        e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_C]);
                                    }
                                    let element = e
                                        .call(
                                            DATA_ARRAY_ELEMENT,
                                            &args![f + ARRAY_A, f + VALUE_B, index],
                                        )
                                        .u32();
                                    e.call(ARRAY_VALUE_AS_ARRAY, &args![element, f + ARRAY_D]);
                                    let element = e
                                        .call(
                                            DATA_ARRAY_ELEMENT,
                                            &args![f + ARRAY_B, f + VALUE_C, index],
                                        )
                                        .u32();
                                    let source_array = e
                                        .call(ARRAY_VALUE_AS_ARRAY, &args![element, f + ARRAY_E])
                                        .u32();
                                    cloner_copy_array(
                                        e,
                                        this,
                                        Ptr::new(f + ARRAY_D),
                                        Ptr::new(source_array),
                                        sub_count,
                                    );
                                    e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_E]);
                                    e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_D]);
                                }
                            } else {
                                for index in 0..count.max(0) {
                                    let target = e
                                        .call(
                                            DATA_ARRAY_ELEMENT,
                                            &args![f + ARRAY_A, f + VALUE_X, index],
                                        )
                                        .u32();
                                    let element = e
                                        .call(
                                            DATA_ARRAY_ELEMENT,
                                            &args![f + ARRAY_B, f + VALUE_Y, index],
                                        )
                                        .u32();
                                    let (implementation, element_index) =
                                        (e.mem.u32(target), e.mem.u32(target + 4));
                                    cloner_copy_simple_value_array_value(
                                        e,
                                        this,
                                        implementation,
                                        element_index,
                                        Ptr::new(element),
                                    );
                                }
                            }
                            e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_B]);
                            e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_A]);
                        }
                        offset = offset.wrapping_add(MEMBER_INFO_SIZE);
                    }
                }
                member_info_array_free(e, members);
                e.set(members, HkArrayBase::m_data, Ptr::NULL);
                e.set(members, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
                e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS]);
            }
            10 => {
                for index in 0..count.max(0) {
                    let element = e
                        .call(
                            DATA_ARRAY_ELEMENT_OV2,
                            &args![source, f + VALUE_SOURCE, index],
                        )
                        .u32();
                    let string = e.call(ARRAY_VALUE_AS_STRING, &args![element]).u32();
                    let target = e
                        .call(
                            DATA_ARRAY_ELEMENT,
                            &args![destination, f + VALUE_DESTINATION, index],
                        )
                        .u32();
                    e.call(ARRAY_VALUE_SET_STRING, &args![target, string]);
                }
            }
            _ => {}
        }
    });
}

// Translated from 00dc4210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::copyValue` (engine map name): copies the member `member` (a `MemberInfo`) of the data
/// object `source` into the object `destination`. A tuple member (type word bit 0x20) makes the
/// destination's array through the destination world (class's world, slot `0x14`, given the
/// object and the member) sized to the tuple size, and copies the source's array into it
/// (`copyArray`); an array member (bit 0x10) does the same with the source array's size; any
/// other member copies its value (`copySimpleValue`).
pub fn cloner_copy_value(
    e: &mut Engine,
    this: Ptr<Cloner>,
    destination: Ptr,
    source: Ptr,
    member: Ptr<MemberInfo>,
) {
    const VALUE: u32 = 0x00;
    const VALUE_2: u32 = 0x08;
    const CLASS: u32 = 0x10;
    const ARRAY_D: u32 = 0x14;
    const ARRAY_T: u32 = 0x18;
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        let type_word = e.get(member, MemberInfo::type_word);
        let name = e.get(member, MemberInfo::name);
        if type_word & 0x30 != 0 {
            let value = e
                .call(
                    DATA_OBJECT_MEMBER_VALUE_OV2,
                    &args![source, f + VALUE, name],
                )
                .u32();
            e.call(OBJECT_VALUE_AS_ARRAY, &args![value, f + ARRAY_T]);
            let class = e
                .call(DATA_OBJECT_GET_CLASS, &args![destination, f + CLASS])
                .u32();
            let world = e.call(DATA_CLASS_GET_WORLD, &args![class]).u32();
            let implementation = e.vcall(world, 0x14, &args![destination, member]).u32();
            e.call(DATA_ARRAY_CONSTRUCT, &args![f + ARRAY_D, implementation]);
            e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS]);
            if type_word & 0x20 != 0 {
                let tuple_size = e.get(member, MemberInfo::tuple_size);
                e.call(DATA_ARRAY_SET_SIZE, &args![f + ARRAY_D, tuple_size]);
                cloner_copy_array(
                    e,
                    this,
                    Ptr::new(f + ARRAY_D),
                    Ptr::new(f + ARRAY_T),
                    tuple_size as i32,
                );
            } else {
                let size = e.call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_T]).i32();
                e.call(DATA_ARRAY_SET_SIZE, &args![f + ARRAY_D, size]);
                let size = e.call(DATA_ARRAY_GET_SIZE, &args![f + ARRAY_T]).i32();
                cloner_copy_array(e, this, Ptr::new(f + ARRAY_D), Ptr::new(f + ARRAY_T), size);
            }
            e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_D]);
            e.call(DATA_ARRAY_DESTRUCT, &args![f + ARRAY_T]);
            return;
        }
        e.call(
            DATA_OBJECT_MEMBER_VALUE,
            &args![destination, f + VALUE, name],
        );
        let source_value = e
            .call(
                DATA_OBJECT_MEMBER_VALUE_OV2,
                &args![source, f + VALUE_2, name],
            )
            .u32();
        let (implementation, index) = (e.mem.u32(f + VALUE), e.mem.u32(f + VALUE + 4));
        cloner_copy_simple_value_object_value(
            e,
            this,
            implementation,
            index,
            Ptr::new(source_value),
        );
    });
}

// Translated from 00dc5440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::findDestClass` (engine map name): the class of the destination world named `name`
/// (slot `0x24` of the destination world), defining it first when the world does not have it:
/// the chain of classes from `name` up through its parents (names from the source world's
/// classes, `0x24` of the source world) that the destination lacks is collected as class
/// descriptions (`getCinfo`), defined parent first (destination slot `0xc`), and the classes the
/// source class's members refer to are found in turn. Answers the destination class (0 when the
/// destination cannot be given one).
pub fn cloner_find_dest_class(e: &mut Engine, this: Ptr<Cloner>, name: u32) -> u32 {
    let destination = e.get(this, Cloner::destination).addr();
    let source = e.get(this, Cloner::source).addr();
    let existing = e.vcall(destination, 0x24, &args![name]).u32();
    if existing != 0 {
        return existing;
    }
    const CLASS_TEMP: u32 = 0x10;
    const PARENT: u32 = 0x14;
    const PARENT_2: u32 = 0x18;
    const MEMBER: u32 = 0x1c;
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        let records: Ptr<HkArrayBase> = Ptr::new(f);
        e.set(records, HkArrayBase::m_data, Ptr::NULL);
        e.set(records, HkArrayBase::m_size, 0);
        e.set(records, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        // hkArray::reserve(10) of 0x18-byte records.
        let allocator = router_allocator(e, 0xc);
        let capacity = (e.get(records, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK) as i32;
        if capacity < 10 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if doubled > 10 { doubled } else { 10 };
            e.call(ARRAY_RESERVE, &args![allocator, records, wanted, 0x18i32]);
        }
        // The classes the destination lacks, from `name` upwards.
        let mut current = name;
        while current != 0 {
            if e.vcall(destination, 0x24, &args![current]).u32() != 0 {
                break;
            }
            let source_class = e.vcall(source, 0x24, &args![current]).u32();
            e.call(DATA_CLASS_CONSTRUCT, &args![f + CLASS_TEMP, source_class]);
            let allocator = router_allocator(e, 0xc);
            let size = e.get(records, HkArrayBase::m_size);
            let capacity = e.get(records, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
            if size as u32 == capacity {
                e.call(ARRAY_GROW, &args![allocator, records, 0x18u32]);
            }
            let size = e.get(records, HkArrayBase::m_size);
            let data = e.get(records, HkArrayBase::m_data).addr();
            let record = data.wrapping_add((size as u32).wrapping_mul(0x18));
            if record != 0 {
                e.mem.set_u32(record + 0xc, 0);
                e.mem.set_u32(record + 0x10, 0);
                e.mem.set_u32(record + 0x14, NOT_OWNED);
            }
            let size = e.get(records, HkArrayBase::m_size);
            let data = e.get(records, HkArrayBase::m_data).addr();
            let record = data.wrapping_add((size as u32).wrapping_mul(0x18));
            e.set(records, HkArrayBase::m_size, size.wrapping_add(1));
            e.call(DATA_CLASS_GET_CINFO, &args![f + CLASS_TEMP, record]);
            let parent = e
                .call(DATA_CLASS_GET_PARENT, &args![f + CLASS_TEMP, f + PARENT])
                .u32();
            let mut next_name = 0u32;
            if e.call(DATA_CLASS_IS_NULL, &args![parent]).u32() == 0 {
                let parent_2 = e
                    .call(DATA_CLASS_GET_PARENT, &args![f + CLASS_TEMP, f + PARENT_2])
                    .u32();
                next_name = e.call(DATA_CLASS_GET_NAME, &args![parent_2]).u32();
                e.call(DATA_CLASS_DESTRUCT, &args![f + PARENT_2]);
            }
            e.call(DATA_CLASS_DESTRUCT, &args![f + PARENT]);
            e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS_TEMP]);
            current = next_name;
        }
        // Parent first.
        let mut index = e.get(records, HkArrayBase::m_size).wrapping_sub(1);
        while index >= 0 {
            let data = e.get(records, HkArrayBase::m_data).addr();
            let record = data.wrapping_add((index as u32).wrapping_mul(0x18));
            e.vcall(destination, 0xc, &args![record]);
            index -= 1;
        }
        // The classes of the source class's members.
        let source_class = e.vcall(source, 0x24, &args![name]).u32();
        let mut member_number = 0i32;
        if e.vcall(source_class, 0x24, &args![]).i32() > 0 {
            loop {
                for word in 0..5 {
                    e.mem.set_u32(f + MEMBER + 4 * word, 0);
                }
                e.vcall(source_class, 0x28, &args![member_number, f + MEMBER]);
                let member_class = e.mem.u32(f + MEMBER + 0xc);
                if member_class != 0 {
                    let member_class_name = e.vcall(member_class, 8, &args![]).u32();
                    cloner_find_dest_class(e, this, member_class_name);
                }
                member_number += 1;
                if member_number >= e.vcall(source_class, 0x24, &args![]).i32() {
                    break;
                }
            }
        }
        let result = e.vcall(destination, 0x24, &args![name]).u32();
        // The records' member arrays, then the record array.
        let allocator = router_allocator(e, 0xc);
        let count = e.get(records, HkArrayBase::m_size);
        if count > 0 {
            let mut offset = 0u32;
            for _ in 0..count {
                let member_allocator = router_allocator(e, 0x10);
                let data = e.get(records, HkArrayBase::m_data).addr();
                let record = data.wrapping_add(offset);
                let capacity = e.mem.u32(record + 0x14);
                e.mem.set_u32(record + 0x10, 0);
                if capacity as i32 >= 0 {
                    let members = e.mem.u32(record + 0xc);
                    e.vcall(
                        member_allocator,
                        0x10,
                        &args![members, (capacity & CAPACITY_MASK) << 4],
                    );
                }
                e.mem.set_u32(record + 0xc, 0);
                e.mem.set_u32(record + 0x14, NOT_OWNED);
                offset = offset.wrapping_add(0x18);
            }
        }
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
        result
    })
}

// Translated from 00dc56f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::getClassMemberInfos` (engine map name): fills the array `out` with the member records
/// of the destination class handle `destination_class` (`getAllMemberInfo`) and completes them
/// from the source class handle `source_class`: for each member of the source class that has a
/// class while the destination's record has none, the record gets the destination class found by
/// the source class's name (`findDestClass`). Both arrays are sized (zeroed records) first.
pub fn cloner_get_class_member_infos(
    e: &mut Engine,
    this: Ptr<Cloner>,
    out: Ptr<HkArrayBase>,
    destination_class: Ptr,
    source_class: Ptr,
) {
    e.with_stack(0xc, |e, block| {
        let source_members: Ptr<HkArrayBase> = block.cast();
        e.set(source_members, HkArrayBase::m_data, Ptr::NULL);
        e.set(source_members, HkArrayBase::m_size, 0);
        e.set(source_members, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        let source_count = e
            .call(DATA_CLASS_GET_NUM_MEMBERS, &args![source_class])
            .i32();
        array_reserve(e, source_members, source_count, MEMBER_INFO_SIZE as i32);
        let size = e.get(source_members, HkArrayBase::m_size);
        let data = e.get(source_members, HkArrayBase::m_data).addr();
        zero_member_infos(e, data, size, source_count);
        e.set(source_members, HkArrayBase::m_size, source_count);
        e.call(
            DATA_CLASS_GET_ALL_MEMBER_INFO,
            &args![source_class, source_members],
        );
        let destination_count = e
            .call(DATA_CLASS_GET_NUM_MEMBERS, &args![destination_class])
            .i32();
        array_reserve(e, out, destination_count, MEMBER_INFO_SIZE as i32);
        let size = e.get(out, HkArrayBase::m_size);
        let data = e.get(out, HkArrayBase::m_data).addr();
        zero_member_infos(e, data, size, destination_count);
        e.set(out, HkArrayBase::m_size, destination_count);
        e.call(
            DATA_CLASS_GET_ALL_MEMBER_INFO,
            &args![destination_class, out],
        );
        let mut index = 0i32;
        let mut offset = 0u32;
        if e.call(DATA_CLASS_GET_NUM_MEMBERS, &args![source_class])
            .i32()
            > 0
        {
            loop {
                let source_data = e.get(source_members, HkArrayBase::m_data).addr();
                let source_member_class = e
                    .mem
                    .u32(source_data.wrapping_add(offset).wrapping_add(0xc));
                if source_member_class != 0 {
                    let data = e.get(out, HkArrayBase::m_data).addr();
                    let record = data.wrapping_add(offset);
                    if e.mem.u32(record + 0xc) == 0 {
                        let class_name = e.vcall(source_member_class, 8, &args![]).u32();
                        let found = cloner_find_dest_class(e, this, class_name);
                        e.mem.set_u32(record + 0xc, found);
                    }
                }
                index += 1;
                offset = offset.wrapping_add(MEMBER_INFO_SIZE);
                if index
                    >= e.call(DATA_CLASS_GET_NUM_MEMBERS, &args![source_class])
                        .i32()
                {
                    break;
                }
            }
        }
        member_info_array_free(e, source_members);
    });
}

// Translated from 00dc58a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::copyObjectMembers` (engine map name): copies every member of the data object
/// `source` into `destination`: the member records of the destination's class completed from
/// the source's class (`getClassMemberInfos`), each member copied by `copyValue`.
pub fn cloner_copy_object_members(
    e: &mut Engine,
    this: Ptr<Cloner>,
    destination: Ptr,
    source: Ptr,
) {
    e.with_stack(0x14, |e, frame| {
        let destination_class = frame.addr();
        let source_class = frame.addr() + 4;
        let records: Ptr<HkArrayBase> = Ptr::new(frame.addr() + 8);
        e.call(
            DATA_OBJECT_GET_CLASS,
            &args![destination, destination_class],
        );
        e.set(records, HkArrayBase::m_data, Ptr::NULL);
        e.set(records, HkArrayBase::m_size, 0);
        e.set(records, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        let source_ret = e
            .call(DATA_OBJECT_GET_CLASS, &args![source, source_class])
            .u32();
        cloner_get_class_member_infos(
            e,
            this,
            records,
            Ptr::new(destination_class),
            Ptr::new(source_ret),
        );
        e.call(DATA_CLASS_DESTRUCT, &args![source_class]);
        let mut index = 0i32;
        let mut offset = 0u32;
        if e.get(records, HkArrayBase::m_size) > 0 {
            loop {
                let data = e.get(records, HkArrayBase::m_data).addr();
                cloner_copy_value(
                    e,
                    this,
                    destination,
                    source,
                    Ptr::new(data.wrapping_add(offset)),
                );
                index += 1;
                offset = offset.wrapping_add(MEMBER_INFO_SIZE);
                if index >= e.get(records, HkArrayBase::m_size) {
                    break;
                }
            }
        }
        member_info_array_free(e, records);
        e.set(records, HkArrayBase::m_data, Ptr::NULL);
        e.set(records, HkArrayBase::m_capacityAndFlags, NOT_OWNED);
        e.call(DATA_CLASS_DESTRUCT, &args![destination_class]);
    });
}

// Translated from 00dc5970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Cloner::copyObject` (engine map name): builds in `out` the destination world's copy of the
/// data object `source`. A null source gives a null object. An object already copied (its handle
/// in the cloner's map) is fetched from the destination world by the stored handle (slot `0x28`).
/// Otherwise the class is found or defined in the destination (`findDestClass`), a new object of
/// it is made (destination slot `0x10`), the pair of handles is entered in the map (`00dc3740`,
/// through the router's allocator at `+0x10`) before the members are copied
/// (`copyObjectMembers`), so that cycles end. Returns `out`.
pub fn cloner_copy_object(e: &mut Engine, this: Ptr<Cloner>, out: Ptr, source: Ptr) -> Ptr {
    if e.call(DATA_OBJECT_IS_NULL, &args![source]).u32() != 0 {
        e.call(DATA_OBJECT_CONSTRUCT, &args![out, 0u32]);
        return out;
    }
    const HANDLE: u32 = 0x00;
    const STORED: u32 = 0x08;
    const CLASS: u32 = 0x10;
    const NEW_CLASS: u32 = 0x14;
    const NEW_OBJECT: u32 = 0x18;
    const NEW_HANDLE: u32 = 0x1c;
    const SOURCE_HANDLE: u32 = 0x24;
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let handle = e
            .call(DATA_OBJECT_GET_HANDLE, &args![source, f + HANDLE])
            .u32();
        let (low, high) = (e.mem.u32(handle), e.mem.u32(handle + 4));
        let objects = this.at(Cloner::objects);
        let destination = e.get(this, Cloner::destination).addr();
        if hk_map_base_get(e, objects, low, high, Ptr::new(f + STORED)) == 0 {
            e.vcall(destination, 0x28, &args![out, f + STORED]);
            return out;
        }
        let class_ret = e
            .call(DATA_OBJECT_GET_CLASS, &args![source, f + CLASS])
            .u32();
        let class_name = e.call(DATA_CLASS_GET_NAME, &args![class_ret]).u32();
        let destination_class = cloner_find_dest_class(e, this, class_name);
        e.call(
            DATA_CLASS_CONSTRUCT,
            &args![f + NEW_CLASS, destination_class],
        );
        e.call(DATA_CLASS_DESTRUCT, &args![f + CLASS]);
        let implementation = e.vcall(destination, 0x10, &args![f + NEW_CLASS]).u32();
        e.call(
            DATA_OBJECT_CONSTRUCT,
            &args![f + NEW_OBJECT, implementation],
        );
        let new_handle = e
            .call(
                DATA_OBJECT_GET_HANDLE,
                &args![f + NEW_OBJECT, f + NEW_HANDLE],
            )
            .u32();
        let (new_low, new_high) = (e.mem.u32(new_handle), e.mem.u32(new_handle + 4));
        let source_handle = e
            .call(DATA_OBJECT_GET_HANDLE, &args![source, f + SOURCE_HANDLE])
            .u32();
        let (key_low, key_high) = (e.mem.u32(source_handle), e.mem.u32(source_handle + 4));
        let allocator = router_allocator(e, 0x10);
        fn_00dc3740(
            e,
            objects,
            Ptr::new(allocator),
            key_low,
            key_high,
            new_low,
            new_high,
        );
        cloner_copy_object_members(e, this, Ptr::new(f + NEW_OBJECT), source);
        e.call(DATA_OBJECT_CONSTRUCT_COPY, &args![out, f + NEW_OBJECT]);
        e.call(DATA_OBJECT_DESTRUCT, &args![f + NEW_OBJECT]);
        e.call(DATA_CLASS_DESTRUCT, &args![f + NEW_CLASS]);
        out
    })
}

// Translated from 00dc3550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectUtil::deepCopyWorld` (engine map name): copies the contents of the world `source`
/// into the world `destination`: a `Cloner` is built on the stack over the two worlds with an
/// object map reserved for 0 elements (`00dc3bd0`), the source world's slot `0x1c` gives its
/// contents object, which is copied with `copyObject`, and the object map is emptied
/// (`00d5f640`). Answers 0.
pub fn hk_data_object_util_deep_copy_world(e: &mut Engine, destination: Ptr, source: Ptr) -> u32 {
    e.with_stack(0x20, |e, block| {
        let f = block.addr();
        let cloner: Ptr<Cloner> = block.cast();
        e.set(cloner, Cloner::destination, destination);
        e.set(cloner, Cloner::source, source);
        let objects = cloner.at(Cloner::objects);
        e.set(objects, HkMapBase::m_elem, 0);
        e.set(objects, HkMapBase::m_numElems, 0);
        e.set(objects, HkMapBase::m_hashMod, -1);
        let allocator = router_allocator(e, 0x10);
        fn_00dc3bd0(e, objects, Ptr::new(allocator), 0);
        let contents = e.vcall(source.addr(), 0x1c, &args![f + 0x14]).u32();
        cloner_copy_object(e, cloner, Ptr::new(f + 0x18), Ptr::new(contents));
        e.call(DATA_OBJECT_DESTRUCT, &args![f + 0x18]);
        e.call(DATA_OBJECT_DESTRUCT, &args![f + 0x14]);
        let allocator = router_allocator(e, 0x10);
        e.call(OBJECT_MAP_CLEAR_AND_DEALLOCATE, &args![objects, allocator]);
    });
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dc1bf0,
            fn_00dc1bf0(u32, Ptr<HkClassMember>, Ptr, u32) -> u32
        ),
        entry!(
            0x00dc1c50,
            hk_data_object_util_get_type_from_member_type(u32, u32, Ptr, u32) -> u32
        ),
        entry!(
            0x00dc1d30,
            copier_copy_into_native_array(
                Ptr<Copier>,
                Ptr<HkArrayBase>,
                Ptr<HkClassMember>,
                Ptr,
            ) -> u32
        ),
        entry!(
            0x00dc2d60,
            copier_fill_native_members(Ptr<Copier>, u32, Ptr) -> u32
        ),
        entry!(
            0x00dc32c0,
            hk_data_object_util_to_object_with_registry(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00dc3300,
            hk_data_object_util_to_resource_with_registry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00dc3550,
            hk_data_object_util_deep_copy_world(Ptr, Ptr) -> u32
        ),
        entry!(0x00dc35e0, fn_00dc35e0(Ptr<HkMapBase>) -> i32),
        entry!(0x00dc3600, fn_00dc3600(Ptr<HkMapBase>, Ptr, u32, u32, u32)),
        entry!(
            0x00dc36b0,
            hk_map_base_get_with_default(Ptr<HkMapBase>, u32, u32, u32) -> u32
        ),
        entry!(0x00dc3710, fn_00dc3710(Ptr, Ptr)),
        entry!(
            0x00dc3740,
            fn_00dc3740(Ptr<HkMapBase>, Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00dc37f0,
            cloner_copy_simple_value_array_value(Ptr<Cloner>, u32, u32, Ptr)
        ),
        entry!(0x00dc38f0, fn_00dc38f0(Ptr<HkMapBase>)),
        entry!(
            0x00dc3920,
            hk_map_base_find_key(Ptr<HkMapBase>, u32, u32) -> u32
        ),
        entry!(0x00dc3980, fn_00dc3980(Ptr<HkMapBase>)),
        entry!(0x00dc39b0, fn_00dc39b0(Ptr<HkMapBase>, Ptr, i32)),
        entry!(
            0x00dc3a80,
            copier_get_member_class_and_check(Ptr<Copier>, Ptr<HkClassMember>, Ptr) -> u32
        ),
        entry!(0x00dc3b60, fn_00dc3b60(Ptr<HkMapBase>, Ptr)),
        entry!(
            0x00dc3b90,
            hk_map_base_get(Ptr<HkMapBase>, u32, u32, Ptr) -> u32
        ),
        entry!(0x00dc3bd0, fn_00dc3bd0(Ptr<HkMapBase>, Ptr, i32)),
        entry!(0x00dc3bf0, fn_00dc3bf0(Ptr<HkMapBase>, Ptr, i32)),
        entry!(0x00dc3cc0, fn_00dc3cc0(Ptr<HkMapBase>, Ptr, i32)),
        entry!(0x00dc3ce0, hk_packfile_data_add_chunk(Ptr, u32, u32, u32)),
        entry!(0x00dc3d40, cloner_copy_array(Ptr<Cloner>, Ptr, Ptr, i32)),
        entry!(
            0x00dc4210,
            cloner_copy_value(Ptr<Cloner>, Ptr, Ptr, Ptr<MemberInfo>)
        ),
        entry!(0x00dc43a0, copier_destructor(Ptr<Copier>)),
        entry!(0x00dc4430, copier_find_or_copy(Ptr<Copier>, Ptr, Ptr, u32)),
        entry!(0x00dc4710, copier_to_object(Ptr<Copier>, Ptr, Ptr) -> u32),
        entry!(
            0x00dc4a80,
            copier_fill_native_simple_member_array_value(
                Ptr<Copier>,
                u32,
                u32,
                Ptr,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00dc4f20,
            copier_constructor(Ptr<Copier>, Ptr, u32) -> Ptr<Copier>
        ),
        entry!(
            0x00dc4fa0,
            copier_fill_native_simple_member_object_value(
                Ptr<Copier>,
                u32,
                u32,
                Ptr,
                u32,
                u32,
            ) -> u32
        ),
        entry!(0x00dc5440, cloner_find_dest_class(Ptr<Cloner>, u32) -> u32),
        entry!(
            0x00dc56f0,
            cloner_get_class_member_infos(Ptr<Cloner>, Ptr<HkArrayBase>, Ptr, Ptr)
        ),
        entry!(
            0x00dc58a0,
            cloner_copy_object_members(Ptr<Cloner>, Ptr, Ptr)
        ),
        entry!(0x00dc5970, cloner_copy_object(Ptr<Cloner>, Ptr, Ptr) -> Ptr),
        entry!(
            0x00dc5ab0,
            cloner_copy_simple_value_object_value(Ptr<Cloner>, u32, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Shared<T> = Rc<RefCell<T>>;

    /// The allocator at `router + 0xc` (array growth and frees) and the one at `router + 0x10`
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
        // The pages of the member type table (`010d6b98`), the error report's messages and the
        // global holding the error reporter (`01268360`, `01268748`).
        e.map(0x010d_6000, 0x1000);
        e.map(0x0126_8000, 0x1000);
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
    /// capacity as asked or doubled, at least 4) and for `memSet`.
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

    // ---- the maps ----

    fn new_map(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(12);
        e.mem.set_i32(map + 8, -1);
        map
    }

    #[test]
    fn the_first_used_slot_is_found_from_the_start() {
        let mut e = engine();
        let map = e.mem.alloc(12);
        let slots = e.mem.alloc(96);
        for n in 0..8 {
            e.mem.set_u32(slots + 12 * n, 0xffff_ffff);
        }
        e.mem.set_u32(map, slots);
        e.mem.set_i32(map + 8, 7);
        // No slot used: one past the last.
        assert_eq!(e.call(0x00dc_35e0, &args![map]).i32(), 8);
        e.mem.set_u32(slots + 12 * 5, 0x30);
        assert_eq!(e.call(0x00dc_35e0, &args![map]).i32(), 5);
        e.mem.set_u32(slots, 0);
        assert_eq!(e.call(0x00dc_35e0, &args![map]).i32(), 0);
        // A negative mask: 0.
        e.mem.set_i32(map + 8, -1);
        assert_eq!(e.call(0x00dc_35e0, &args![map]).i32(), 0);
    }

    #[test]
    fn the_small_map_inserts_looks_up_clears_and_resizes() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        // The first insert grows the empty table to 8 slots of 12 bytes, empty marked -1.
        e.call(0x00dc_3600, &args![map, blocks, 0x100u32, 1u32, 11u32]);
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 4), vec![vec![blocks, 96]]);
        assert_eq!(e.mem.i32(map + 8), 7);
        assert_eq!(e.mem.u32(map + 4), 1);
        let get = |e: &mut Engine, high: u32, default: u32| {
            e.call(0x00dc_36b0, &args![map, 0x100u32, high, default])
                .u32()
        };
        assert_eq!(get(&mut e, 1, 99), 11);
        assert_eq!(get(&mut e, 2, 99), 99);
        // A key already in the map is replaced.
        e.call(0x00dc_3600, &args![map, blocks, 0x100u32, 1u32, 12u32]);
        assert_eq!(e.mem.u32(map + 4), 1);
        assert_eq!(get(&mut e, 1, 99), 12);
        // Keys with the same hash probe on; the fifth makes the table 16 slots wide.
        for high in 2..=5u32 {
            e.call(0x00dc_3600, &args![map, blocks, 0x100u32, high, high * 10]);
        }
        assert_eq!(e.mem.i32(map + 8), 15);
        for high in 2..=5u32 {
            assert_eq!(get(&mut e, high, 0), high * 10);
        }
        // Clearing keeps the table but empties it; clear-and-free also returns it.
        let table = e.mem.u32(map);
        e.call(0x00dc_38f0, &args![map]);
        assert_eq!(e.mem.u32(map + 4), 0);
        assert_eq!(get(&mut e, 2, 0), 0);
        for slot in 0..16 {
            assert_eq!(e.mem.u32(table + 12 * slot), 0xffff_ffff);
        }
        e.call(0x00dc_3b60, &args![map, blocks]);
        assert_eq!(
            calls(&e, ALLOCATOR_BLOCKS + 8).last(),
            Some(&vec![blocks, table, 192])
        );
        // A mask that is not positive finds nothing.
        e.mem.set_i32(map + 8, 0);
        assert_eq!(get(&mut e, 1, 77), 77);
    }

    #[test]
    fn the_small_map_resizes_by_the_requested_count() {
        for (count, slots) in [(0, 8u32), (4, 8), (5, 16), (8, 16), (9, 32)] {
            let mut e = engine();
            let (_, blocks) = install_router(&mut e);
            let map = new_map(&mut e);
            e.call(0x00dc_3cc0, &args![map, blocks, count]);
            assert_eq!(
                calls(&e, ALLOCATOR_BLOCKS + 4),
                vec![vec![blocks, slots * 12]],
                "count {count}"
            );
            assert_eq!(e.mem.i32(map + 8), slots as i32 - 1);
        }
        // `resizeTable` raises fewer than 8 slots to 8 and does not free a not-owned table.
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        e.call(0x00dc_3bf0, &args![map, blocks, 3i32]);
        assert_eq!(e.mem.i32(map + 8), 7);
        e.call(0x00dc_3600, &args![map, blocks, 0x40u32, 1u32, 5u32]);
        e.mem.set_u32(map + 4, 0x8000_0001);
        let frees = calls(&e, ALLOCATOR_BLOCKS + 8).len();
        e.call(0x00dc_3bf0, &args![map, blocks, 16i32]);
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 8).len(), frees);
        assert_eq!(e.mem.u32(map + 4), 1, "the flag is gone with the old table");
        assert_eq!(
            e.call(0x00dc_36b0, &args![map, 0x40u32, 1u32, 0u32]).u32(),
            5
        );
    }

    #[test]
    fn the_large_map_holds_two_value_words() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        e.call(
            0x00dc_3740,
            &args![map, blocks, 0x100u32, 1u32, 0xaau32, 0xbbu32],
        );
        assert_eq!(calls(&e, ALLOCATOR_BLOCKS + 4), vec![vec![blocks, 128]]);
        let out = e.mem.alloc(8);
        // Found: the two words are written and 0 answered; not found: 1.
        assert_eq!(
            e.call(0x00dc_3b90, &args![map, 0x100u32, 1u32, out]).u32(),
            0
        );
        assert_eq!([e.mem.u32(out), e.mem.u32(out + 4)], [0xaa, 0xbb]);
        e.mem.set_u32(out, 0);
        assert_eq!(
            e.call(0x00dc_3b90, &args![map, 0x100u32, 2u32, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(out), 0, "left alone");
        // The slot of a key, or the mask plus one.
        let slot = e.call(0x00dc_3920, &args![map, 0x100u32, 1u32]).u32();
        assert!(slot < 8);
        assert_eq!(e.call(0x00dc_3920, &args![map, 0x100u32, 2u32]).u32(), 8);
        // Equal keys are replaced; colliding keys probe on; the fifth entry makes 16 slots.
        e.call(
            0x00dc_3740,
            &args![map, blocks, 0x100u32, 1u32, 0xccu32, 0xddu32],
        );
        assert_eq!(e.mem.u32(map + 4), 1);
        for high in 2..=5u32 {
            e.call(
                0x00dc_3740,
                &args![map, blocks, 0x100u32, high, high, high + 1],
            );
        }
        assert_eq!(e.mem.i32(map + 8), 15);
        for high in 2..=5u32 {
            assert_eq!(
                e.call(0x00dc_3b90, &args![map, 0x100u32, high, out]).u32(),
                0
            );
            assert_eq!([e.mem.u32(out), e.mem.u32(out + 4)], [high, high + 1]);
        }
        assert_eq!(
            e.call(0x00dc_3b90, &args![map, 0x100u32, 1u32, out]).u32(),
            0
        );
        assert_eq!([e.mem.u32(out), e.mem.u32(out + 4)], [0xcc, 0xdd]);
        // Clearing leaves the table, marked empty.
        let table = e.mem.u32(map);
        e.call(0x00dc_3980, &args![map]);
        assert_eq!(e.mem.u32(map + 4), 0);
        for slot in 0..16 {
            assert_eq!(e.mem.u32(table + 16 * slot), 0xffff_ffff);
        }
        // A mask that is not positive finds nothing: the mask plus one.
        e.mem.set_i32(map + 8, 0);
        assert_eq!(e.call(0x00dc_3920, &args![map, 0x100u32, 1u32]).u32(), 1);
    }

    #[test]
    fn the_large_map_resizes_by_the_requested_count() {
        for (count, slots) in [(0, 8u32), (5, 16), (9, 32)] {
            let mut e = engine();
            let (_, blocks) = install_router(&mut e);
            let map = new_map(&mut e);
            e.call(0x00dc_3bd0, &args![map, blocks, count]);
            assert_eq!(
                calls(&e, ALLOCATOR_BLOCKS + 4),
                vec![vec![blocks, slots * 16]],
                "count {count}"
            );
        }
        // `resizeTable` moves the elements, and frees the old table.
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        let map = new_map(&mut e);
        e.call(0x00dc_3740, &args![map, blocks, 0x40u32, 1u32, 7u32, 8u32]);
        let old = e.mem.u32(map);
        e.call(0x00dc_39b0, &args![map, blocks, 3i32]);
        assert_eq!(e.mem.i32(map + 8), 7, "at least 8 slots");
        assert_eq!(
            calls(&e, ALLOCATOR_BLOCKS + 8).last(),
            Some(&vec![blocks, old, 128])
        );
        let out = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00dc_3b90, &args![map, 0x40u32, 1u32, out]).u32(),
            0
        );
        assert_eq!([e.mem.u32(out), e.mem.u32(out + 4)], [7, 8]);
    }

    #[test]
    fn a_chunk_is_appended_after_growing_the_chunk_array() {
        let mut e = engine();
        let (_, blocks) = install_router(&mut e);
        install_arrays(&mut e);
        let package = e.mem.alloc(0x80);
        e.call(0x00dc_3ce0, &args![package, 0x1000u32, 0x20u32, 6u32]);
        e.call(0x00dc_3ce0, &args![package, 0x2000u32, 0x30u32, 0x18u32]);
        assert_eq!(e.mem.u32(package + 0x44), 2);
        let data = e.mem.u32(package + 0x40);
        assert_eq!(e.mem.u32(data), 0x1000);
        assert_eq!(e.mem.u32(data + 4), 0x20);
        assert_eq!(e.mem.u32(data + 8), 6);
        assert_eq!(e.mem.u32(data + 12), 0x2000);
        assert_eq!(e.mem.u32(data + 20), 0x18);
        // The array grew once, to 4 chunks of 12 bytes, through the allocator at `+0x10`.
        assert_eq!(calls(&e, ARRAY_GROW).len(), 1);
        assert_eq!(calls(&e, ARRAY_GROW)[0][2..], [12]);
        assert_eq!(calls(&e, ARRAY_GROW)[0][0], blocks);
    }

    // ---- class member helpers ----

    #[test]
    fn an_enum_member_is_set_through_the_member_and_other_types_are_refused() {
        let mut e = engine();
        e.register(OBJECT_VALUE_AS_INT, |_, _| ret(5));
        e.register_double(MEMBER_CALL_00D86BC0, |_, _| Ret::default());
        let member = e.mem.alloc(0x18);
        let value = e.mem.alloc(8);
        let call = |e: &mut Engine, kind: u8, member_enum: u32| {
            e.mem.set_u8(member + 0xc, kind);
            e.mem.set_u32(member + 8, member_enum);
            e.call(0x00dc_1bf0, &args![0x4000u32, member, value, 0x1111u32])
                .u32()
        };
        // Type 0x18 with an enum: `00d86bc0(member, target, integer)`, answer 0.
        assert_eq!(call(&mut e, 0x18, 0x77), 0);
        assert_eq!(
            calls(&e, MEMBER_CALL_00D86BC0),
            vec![vec![member, 0x4000, 5]]
        );
        // Type 0x1f the same; without an enum nothing is called, the answer is still 0.
        assert_eq!(call(&mut e, 0x1f, 0x77), 0);
        assert_eq!(calls(&e, MEMBER_CALL_00D86BC0).len(), 2);
        assert_eq!(call(&mut e, 0x18, 0), 0);
        assert_eq!(calls(&e, MEMBER_CALL_00D86BC0).len(), 2);
        // Another type: 1.
        assert_eq!(call(&mut e, 7, 0x77), 1);
        assert_eq!(calls(&e, MEMBER_CALL_00D86BC0).len(), 2);
    }

    /// The member type table at `010d6b98` (one word per type).
    const MEMBER_TYPES: [u32; 35] = [
        0, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 3, 4, 4, 6, 6, 6, 7, 7, 0, 8, 0, 0x10, 0x10, 0, 9, 0x10,
        0x19, 8, 0xa, 2, 0, 3, 0xa, 0,
    ];

    #[test]
    fn the_data_type_comes_from_the_member_type_and_subtype() {
        let mut e = engine();
        for (n, value) in MEMBER_TYPES.iter().enumerate() {
            e.set_global(MEMBER_TYPE_TABLE + 4 * n as u32, *value);
        }
        // A class whose members have the given types (the byte at +0xc of each record).
        let members: Shared<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
        let types = Rc::clone(&members);
        e.register_double(CLASS_GET_NUM_MEMBERS, move |_, _| {
            ret(types.borrow().len() as u32)
        });
        let records = Rc::clone(&members);
        e.register_double(CLASS_GET_MEMBER, move |e, a| {
            let record = e.mem.alloc(0x18);
            e.mem.set_u8(record + 0xc, records.borrow()[a[1] as usize]);
            ret(record)
        });
        let get = |e: &mut Engine, kind: u32, subtype: u32, array: u32| {
            e.call(0x00dc_1c50, &args![kind, subtype, 0x5000u32, array])
                .u32()
        };
        // Plain types come from the table.
        assert_eq!(get(&mut e, 1, 0, 0), 1);
        assert_eq!(get(&mut e, 0xb, 0, 0), 3);
        // Types 0x18 and 0x1f take the subtype as the type: here 0xb.
        assert_eq!(get(&mut e, 0x18, 0xb, 0), 3);
        assert_eq!(get(&mut e, 0x1f, 0xb, 0), 3);
        // The tuple bit.
        assert_eq!(get(&mut e, 1, 0, 1), 0x21);
        // Type 0x16 is a struct (0x10): the subtype refines it.
        assert_eq!(get(&mut e, 0x16, 7, 0), 0x12);
        assert_eq!(get(&mut e, 0x16, 4, 0), 0x11);
        assert_eq!(get(&mut e, 0x16, 0x22, 0), 0x10);
        assert_eq!(get(&mut e, 0x16, 0, 5), 0x30);
        // Subtype 0x19: 0x19 when no member is of type 0x16, 0x1a or 0x1b, else 0x18.
        *members.borrow_mut() = vec![7, 8];
        assert_eq!(get(&mut e, 0x16, 0x19, 0), 0x19);
        *members.borrow_mut() = vec![];
        assert_eq!(get(&mut e, 0x16, 0x19, 0), 0x19);
        for kind in [0x16u8, 0x1a, 0x1b] {
            *members.borrow_mut() = vec![7, kind];
            assert_eq!(get(&mut e, 0x16, 0x19, 0), 0x18, "member type {kind:#x}");
        }
        *members.borrow_mut() = vec![0x1c, 0x19];
        assert_eq!(get(&mut e, 0x16, 0x19, 0), 0x19);
    }

    #[test]
    fn a_matrix_copy_moves_64_bytes() {
        let mut e = engine();
        let source = e.mem.alloc(0x40);
        let destination = e.mem.alloc(0x40);
        let pattern: Vec<u8> = (0..0x40).collect();
        e.mem.write(source, &pattern);
        e.call(0x00dc_3710, &args![destination, source]);
        assert_eq!(e.mem.bytes(destination, 0x40), pattern);
    }

    // ---- storing member values in native structs ----

    /// The accessors of one kind of `Value` as doubles over fixed answers. `vector` is the
    /// pointer `asVec` answers, `object` the id `asObject` writes (0 is null).
    struct SimpleValue {
        integer: u32,
        real: f32,
        vector: u32,
        object: u32,
        string: u32,
    }

    fn install_simple(e: &mut Engine, accessors: &SimpleAccessors, value: &SimpleValue) {
        let integer = value.integer;
        e.register_double(accessors.as_int, move |_, _| ret(integer));
        let real = value.real;
        e.register_double(accessors.as_real, move |_, _| Ret {
            st0: f64::from(real),
            ..Ret::default()
        });
        let vector = value.vector;
        e.register_double(accessors.as_vec, move |_, _| ret(vector));
        let object = value.object;
        e.register_double(accessors.as_object, move |e, a| {
            e.mem.set_u32(a[1], object);
            ret(a[1])
        });
        let string = value.string;
        e.register_double(accessors.as_string, move |_, _| ret(string));
        e.register(DATA_OBJECT_IS_NULL, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
        });
        e.register(DATA_OBJECT_DESTRUCT, |_, _| Ret::default());
    }

    /// A copier block (flag at +8) over a package block whose string array is at +0x34.
    fn copier_block(e: &mut Engine, flag: u32) -> (u32, u32) {
        let copier = e.mem.alloc(0x30);
        let package = e.mem.alloc(0x80);
        e.mem.set_u32(copier + 4, package);
        e.mem.set_u32(copier + 8, flag);
        (copier, package)
    }

    fn check_simple(address: u32, accessors: &SimpleAccessors) {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        let floats = e.mem.alloc(0x40);
        for n in 0..16 {
            e.mem.set_f32(floats + 4 * n, 1.0 + n as f32);
        }
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"txt");
        let value = SimpleValue {
            integer: 0xffff_ff85,
            real: 1.25,
            vector: floats,
            object: 0,
            string: text,
        };
        install_simple(&mut e, accessors, &value);
        e.register_double(STRING_DUP, |e, a| {
            let copy = e.mem.alloc(8);
            let text = e.mem.cstr(a[0]);
            e.mem.set_cstr(copy, &text);
            ret(copy)
        });
        e.register_double(CONVERT_MATRIX_3, |_, _| Ret::default());
        e.register_double(CONVERT_MATRIX_3_B, |_, _| Ret::default());
        e.register_double(CONVERT_MATRIX_4, |_, _| Ret::default());
        let (copier, package) = copier_block(&mut e, 0);
        let target = e.mem.alloc(0x40);
        let value_ptr = 0x6000u32;
        let fill = |e: &mut Engine, subtype: u32, member_class: u32| {
            e.call(
                address,
                &args![copier, target, subtype, value_ptr, member_class, 0u32],
            )
            .u32()
        };
        // 1: a boolean byte. 2, 3 and 4: the low byte. 5 and 6: the low word.
        assert_eq!(fill(&mut e, 1, 0), 0);
        assert_eq!(e.mem.u8(target), 1);
        assert_eq!(fill(&mut e, 2, 0), 0);
        assert_eq!(e.mem.u8(target), 0x85);
        e.mem.set_u8(target, 0);
        assert_eq!(fill(&mut e, 3, 0), 0);
        assert_eq!(e.mem.u8(target), 0x85);
        assert_eq!(fill(&mut e, 4, 0), 0);
        e.mem.set_u8(target, 0);
        assert_eq!(fill(&mut e, 5, 0), 0);
        assert_eq!(e.mem.u16(target), 0xff85);
        e.mem.set_u16(target, 0);
        assert_eq!(fill(&mut e, 6, 0), 0);
        assert_eq!(e.mem.u16(target), 0xff85);
        // 7, 8 and 0x1e: the word. 9 and 10: the word and its sign.
        for subtype in [7, 8, 0x1e] {
            e.mem.set_u32(target, 0);
            assert_eq!(fill(&mut e, subtype, 0), 0);
            assert_eq!(e.mem.u32(target), 0xffff_ff85);
        }
        for subtype in [9, 10] {
            e.mem.set_u64(target, 0);
            assert_eq!(fill(&mut e, subtype, 0), 0);
            assert_eq!(e.mem.u64(target), 0xffff_ffff_ffff_ff85);
        }
        // 0xb: a float; 0x20: the high half of a float.
        assert_eq!(fill(&mut e, 0xb, 0), 0);
        assert_eq!(e.mem.f32(target), 1.25);
        e.mem.set_u16(target, 0);
        assert_eq!(fill(&mut e, 0x20, 0), 0);
        assert_eq!(e.mem.u16(target), (1.25f32.to_bits() >> 16) as u16);
        // 0xc and 0xd: four floats copied; the vector is asked for with 4.
        e.mem.write(target, &[0; 16]);
        assert_eq!(fill(&mut e, 0xc, 0), 0);
        assert_eq!(e.mem.bytes(target, 16), e.mem.bytes(floats, 16));
        e.mem.write(target, &[0; 16]);
        assert_eq!(fill(&mut e, 0xd, 0), 0);
        assert_eq!(e.mem.bytes(target, 16), e.mem.bytes(floats, 16));
        // 0xe, 0xf, 0x10 and 0x12 hand the floats to the matrix converters; 0x11 copies 64 bytes.
        assert_eq!(fill(&mut e, 0xe, 0), 0);
        assert_eq!(calls(&e, CONVERT_MATRIX_3), vec![vec![target, floats]]);
        assert_eq!(fill(&mut e, 0xf, 0), 0);
        assert_eq!(fill(&mut e, 0x10, 0), 0);
        assert_eq!(calls(&e, CONVERT_MATRIX_3_B).len(), 2);
        assert_eq!(fill(&mut e, 0x12, 0), 0);
        assert_eq!(calls(&e, CONVERT_MATRIX_4), vec![vec![target, floats]]);
        e.mem.write(target, &[0; 0x40]);
        assert_eq!(fill(&mut e, 0x11, 0), 0);
        assert_eq!(e.mem.bytes(target, 0x40), e.mem.bytes(floats, 0x40));
        // 0x1d: a string copy is made and put in the package's strings; the copier's flag
        // refuses it.
        e.mem.set_u32(target, 0);
        assert_eq!(fill(&mut e, 0x1d, 0), 0);
        let copy = e.mem.u32(target);
        assert_ne!(copy, 0);
        assert_eq!(e.mem.cstr(copy), b"txt");
        assert_eq!(e.mem.u32(package + 0x38), 1);
        assert_eq!(e.mem.u32(e.mem.u32(package + 0x34)), copy);
        // 0x21: the same, with bit 0 set when the copier's flag is set.
        assert_eq!(fill(&mut e, 0x21, 0), 0);
        assert_eq!(e.mem.u32(package + 0x38), 2);
        assert_eq!(e.mem.u32(target) & 1, 0);
        e.mem.set_u32(copier + 8, 1);
        assert_eq!(fill(&mut e, 0x21, 0), 0);
        assert_eq!(e.mem.u32(target) & 1, 1);
        e.mem.set_u32(target, 0xdead);
        assert_eq!(fill(&mut e, 0x1d, 0), 1);
        assert_eq!(e.mem.u32(target), 0xdead);
        // Pointers need a class; a null object stores nothing; unknown subtypes answer 1.
        assert_eq!(fill(&mut e, 0x14, 0), 0);
        assert_eq!(fill(&mut e, 0x14, 0x77), 0);
        assert_eq!(fill(&mut e, 0x19, 0), 0);
        assert_eq!(fill(&mut e, 0x1c, 0), 1);
        e.mem.set_u32(copier + 8, 0);
        assert_eq!(fill(&mut e, 0x1c, 0), 0);
        assert_eq!(e.mem.u32(target), 0xdead);
        for subtype in [0, 0x13, 0x15, 0x1a, 0x1b, 0x1f, 0x22] {
            assert_eq!(fill(&mut e, subtype, 0), 1, "subtype {subtype:#x}");
        }
        // A string that is null stores a null.
        install_simple(&mut e, accessors, &SimpleValue { string: 0, ..value });
        e.mem.set_u32(target, 0xdead);
        assert_eq!(fill(&mut e, 0x1d, 0), 0);
        assert_eq!(e.mem.u32(target), 0);
    }

    #[test]
    fn array_elements_are_stored_into_native_members_by_subtype() {
        check_simple(0x00dc_4a80, &ARRAY_ACCESSORS);
    }

    #[test]
    fn object_values_are_stored_into_native_members_by_subtype() {
        check_simple(0x00dc_4fa0, &OBJECT_ACCESSORS);
    }

    // ---- A small data-object world and class registry the copier walks ----

    /// A value of an object member or an array element.
    #[derive(Clone, Debug, PartialEq)]
    enum Val {
        Int(i32),
        Real(f32),
        Text(u32),
        Object(u32),
        Array(u32),
        /// A pointer to floats in game memory.
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
        /// For an array made by swizzling a member out of an array of structs: the object and
        /// member index each element stands for (empty for ordinary arrays).
        view: Vec<(u32, u32)>,
    }

    /// Classes, objects and arrays; an id is the index plus one (0 is null).
    #[derive(Default)]
    struct Model {
        classes: Vec<ClassModel>,
        objects: Vec<ObjectModel>,
        arrays: Vec<ArrayModel>,
    }

    impl Model {
        fn member_index(&self, object: u32, name: u32) -> Option<u32> {
            let class = self.objects[object as usize - 1].class;
            let members = &self.classes[class as usize - 1].members;
            members
                .iter()
                .position(|m| m.name == name)
                .map(|n| n as u32)
        }

        fn value(&self, container: u32, index: u32, in_array: bool) -> Val {
            if in_array {
                let array = &self.arrays[container as usize - 1];
                if let Some(&(object, member)) = array.view.get(index as usize) {
                    return self.objects[object as usize - 1].values[member as usize].clone();
                }
                array.elements[index as usize].clone()
            } else {
                self.objects[container as usize - 1].values[index as usize].clone()
            }
        }

        fn set_value(&mut self, container: u32, index: u32, in_array: bool, value: Val) {
            if in_array {
                let array = &mut self.arrays[container as usize - 1];
                if let Some(&(object, member)) = array.view.get(index as usize) {
                    self.objects[object as usize - 1].values[member as usize] = value;
                } else {
                    array.elements[index as usize] = value;
                }
            } else {
                self.objects[container as usize - 1].values[index as usize] = value;
            }
        }
    }

    /// Doubles for the `hkDataObject`, `hkDataClass`, `hkDataArray` and `Value` reads over
    /// `model`. A handle is a word holding the implementation id; a `Value` is two words, the
    /// container id and the member or element index.
    fn install_model(e: &mut Engine, model: &Shared<Model>) {
        e.register(DATA_OBJECT_IS_NULL, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
        });
        e.register(DATA_OBJECT_GET_HANDLE, |e, a| {
            let implementation = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], implementation);
            e.mem.set_u32(a[1] + 4, 0);
            ret(a[1])
        });
        e.register(DATA_OBJECT_DESTRUCT, |_, _| Ret::default());
        let m = Rc::clone(model);
        e.register_double(DATA_OBJECT_GET_CLASS, move |e, a| {
            let id = e.mem.u32(a[0]);
            let class = m.borrow().objects[id as usize - 1].class;
            e.mem.set_u32(a[1], class);
            ret(a[1])
        });
        for address in [DATA_OBJECT_MEMBER_VALUE, DATA_OBJECT_MEMBER_VALUE_OV2] {
            let m = Rc::clone(model);
            e.register_double(address, move |e, a| {
                let id = e.mem.u32(a[0]);
                let index = m.borrow().member_index(id, a[2]).unwrap();
                e.mem.set_u32(a[1], id);
                e.mem.set_u32(a[1] + 4, index);
                ret(a[1])
            });
        }
        let m = Rc::clone(model);
        e.register_double(DATA_OBJECT_HAS_MEMBER, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(u32::from(m.borrow().member_index(id, a[1]).is_some()))
        });
        for in_array in [false, true] {
            let (as_int, as_real, as_object, as_string, as_array, as_vec) = if in_array {
                (
                    ARRAY_VALUE_AS_INT,
                    ARRAY_VALUE_AS_REAL,
                    ARRAY_VALUE_AS_OBJECT,
                    ARRAY_VALUE_AS_STRING,
                    ARRAY_VALUE_AS_ARRAY,
                    ARRAY_VALUE_AS_VEC,
                )
            } else {
                (
                    OBJECT_VALUE_AS_INT,
                    OBJECT_VALUE_AS_REAL,
                    OBJECT_VALUE_AS_OBJECT,
                    OBJECT_VALUE_AS_STRING,
                    OBJECT_VALUE_AS_ARRAY,
                    OBJECT_VALUE_AS_VEC,
                )
            };
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
        e.register(DATA_CLASS_IS_NULL, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
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
        let m = Rc::clone(model);
        e.register_double(DATA_CLASS_GET_NUM_MEMBERS, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().classes[id as usize - 1].members.len() as u32)
        });
        let m = Rc::clone(model);
        e.register_double(DATA_CLASS_GET_ALL_MEMBER_INFO, move |e, a| {
            let id = e.mem.u32(a[0]);
            let data = e.mem.u32(a[1]);
            let model = m.borrow();
            for (n, member) in model.classes[id as usize - 1].members.iter().enumerate() {
                let record = data + 0x14 * n as u32;
                e.mem.set_u32(record, member.name);
                e.mem.set_u32(record + 8, member.type_word);
                e.mem.set_u32(record + 0xc, member.class);
                e.mem.set_u32(record + 0x10, member.tuple);
            }
            Ret::default()
        });
        let m = Rc::clone(model);
        e.register_double(DATA_ARRAY_GET_SIZE, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().arrays[id as usize - 1].elements.len() as u32)
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
    }

    /// A native class (the `hkClass` of the registry): its declared members' pointers.
    struct NativeClass {
        name: u32,
        parent: u32,
        members: Vec<u32>,
        size: u32,
    }

    /// The registry, the native classes and members, and the package under construction.
    #[derive(Default)]
    struct Natives {
        classes: HashMap<u32, NativeClass>,
        by_name: HashMap<u32, u32>,
        sizes_in_bytes: HashMap<u32, i32>,
        next: u32,
    }

    struct Fixture {
        e: Engine,
        model: Shared<Model>,
        natives: Shared<Natives>,
        copier: u32,
        package: u32,
        references: Shared<HashMap<u32, i32>>,
    }

    /// The registry object (vtable slot `0x10` is `getClassByName`).
    const REGISTRY_SLOT: u32 = 0x0900_0500;
    /// The object the finished native objects are handed to (slot `0x10`).
    const FINISHER_SLOT: u32 = 0x0900_0600;

    fn fixture(flag: u32) -> Fixture {
        let mut e = engine();
        install_router(&mut e);
        install_arrays(&mut e);
        let model: Shared<Model> = Rc::new(RefCell::new(Model::default()));
        install_model(&mut e, &model);
        let natives: Shared<Natives> = Rc::new(RefCell::new(Natives {
            next: 0x0a00_0000,
            ..Natives::default()
        }));
        // `hkClass` and `hkClassMember`.
        let n = Rc::clone(&natives);
        e.register_double(CLASS_GET_NAME, move |_, a| {
            ret(n.borrow().classes[&a[0]].name)
        });
        let n = Rc::clone(&natives);
        e.register_double(CLASS_GET_PARENT, move |_, a| {
            ret(n.borrow().classes[&a[0]].parent)
        });
        let n = Rc::clone(&natives);
        e.register_double(CLASS_GET_NUM_DECLARED_MEMBERS, move |_, a| {
            ret(n.borrow().classes[&a[0]].members.len() as u32)
        });
        let n = Rc::clone(&natives);
        e.register_double(CLASS_GET_DECLARED_MEMBER, move |_, a| {
            ret(n.borrow().classes[&a[0]].members[a[1] as usize])
        });
        let n = Rc::clone(&natives);
        e.register_double(CLASS_GET_OBJECT_SIZE, move |_, a| {
            ret(n.borrow().classes[&a[0]].size)
        });
        e.register(CLASS_GET_DESCRIBED_VERSION, |_, _| Ret::default());
        e.register(CLASS_HAS_VTABLE, |e, a| {
            e.mem.set_u8(a[1], 0);
            ret(a[1])
        });
        e.register(CLASS_IS_SUPER_CLASS, |e, a| {
            e.mem.set_u8(a[1], 0);
            ret(a[1])
        });
        e.register(MEMBER_GET_CLASS, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(MEMBER_GET_CSTYLE_ARRAY_SIZE, |e, a| {
            ret(i32::from(e.mem.i16(a[0] + 0xe)) as u32)
        });
        let n = Rc::clone(&natives);
        e.register_double(MEMBER_GET_SIZE_IN_BYTES, move |_, a| {
            ret(n.borrow().sizes_in_bytes[&a[0]] as u32)
        });
        e.register(MEMBER_GET_ATTRIBUTE, |_, _| ret(0));
        e.register_double(MEMBER_CALL_00D86BC0, |_, _| Ret::default());
        // The registry and the finisher.
        let n = Rc::clone(&natives);
        e.register_double(REGISTRY_SLOT, move |_, a| {
            ret(n.borrow().by_name.get(&a[1]).copied().unwrap_or(0))
        });
        let registry_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(registry_vtable + 0x10, REGISTRY_SLOT);
        let registry = e.mem.alloc(8);
        e.mem.set_u32(registry, registry_vtable);
        // Reference counting.
        let references: Shared<HashMap<u32, i32>> = Rc::new(RefCell::new(HashMap::new()));
        let counts = Rc::clone(&references);
        e.register_double(ADD_REFERENCE, move |_, a| {
            *counts.borrow_mut().entry(a[0]).or_insert(0) += 1;
            Ret::default()
        });
        let counts = Rc::clone(&references);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            *counts.borrow_mut().entry(a[0]).or_insert(0) -= 1;
            Ret::default()
        });
        // The package, strings, the pointer maps and the matrix converters.
        e.register_double(STRING_DUP, |e, a| {
            let copy = e.mem.alloc(16);
            let text = e.mem.cstr(a[0]);
            e.mem.set_cstr(copy, &text);
            ret(copy)
        });
        e.register(STRING_COMPARE, |e, a| {
            ret(u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])))
        });
        e.register(POINTER_MAP_RESERVE, |_, _| Ret::default());
        e.register(POINTER_MAP_CLEAR_AND_DEALLOCATE, |_, _| Ret::default());
        e.register(POINTER_MAP_GET_WITH_DEFAULT, |_, a| ret(a[2]));
        let package = e.mem.alloc(0x80);
        let copier = e.mem.alloc(0x30);
        e.call(0x00dc_4f20, &args![copier, registry, flag]);
        e.mem.set_u32(copier + 4, package);
        e.call_log = Some(Vec::new());
        Fixture {
            e,
            model,
            natives,
            copier,
            package,
            references,
        }
    }

    impl Fixture {
        fn text(&mut self, text: &str) -> u32 {
            let block = self.e.mem.alloc(text.len() as u32 + 1);
            self.e.mem.set_cstr(block, text.as_bytes());
            block
        }

        fn handle(&mut self, id: u32) -> u32 {
            let block = self.e.mem.alloc(8);
            self.e.mem.set_u32(block, id);
            block
        }

        fn member(&mut self, name: u32, type_word: u32, class: u32, tuple: u32) -> MemberModel {
            MemberModel {
                name,
                type_word,
                class,
                tuple,
            }
        }

        fn class(&mut self, name: u32, members: Vec<MemberModel>) -> u32 {
            let mut model = self.model.borrow_mut();
            model.classes.push(ClassModel {
                name,
                parent: 0,
                version: 1,
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
                view: Vec::new(),
            });
            model.arrays.len() as u32
        }

        /// A native `hkClassMember`: name, class, enum, type, subtype, C array size, flags,
        /// offset; `size_in_bytes` is what `getSizeInBytes` answers.
        #[allow(clippy::too_many_arguments)]
        fn native_member(
            &mut self,
            name: u32,
            kind: u8,
            subtype: u8,
            array_size: i16,
            offset: u16,
            class: u32,
            member_enum: u32,
            size_in_bytes: i32,
        ) -> u32 {
            let member = self.e.mem.alloc(0x18);
            self.e.mem.set_u32(member, name);
            self.e.mem.set_u32(member + 4, class);
            self.e.mem.set_u32(member + 8, member_enum);
            self.e.mem.set_u8(member + 0xc, kind);
            self.e.mem.set_u8(member + 0xd, subtype);
            self.e.mem.set_i16(member + 0xe, array_size);
            self.e.mem.set_u16(member + 0x12, offset);
            self.natives
                .borrow_mut()
                .sizes_in_bytes
                .insert(member, size_in_bytes);
            member
        }

        /// A native `hkClass` registered under `name`.
        fn native_class(&mut self, name: u32, parent: u32, members: Vec<u32>, size: u32) -> u32 {
            let mut natives = self.natives.borrow_mut();
            natives.next += 0x100;
            let class = natives.next;
            natives.classes.insert(
                class,
                NativeClass {
                    name,
                    parent,
                    members,
                    size,
                },
            );
            natives.by_name.insert(name, class);
            class
        }
    }

    #[test]
    fn native_members_are_filled_by_their_type() {
        let mut f = fixture(0);
        let names: Vec<u32> = [
            "flag", "count", "ratio", "items", "name", "absent", "three", "kind",
        ]
        .iter()
        .map(|n| f.text(n))
        .collect();
        let title = f.text("Title");
        let class_name = f.text("N");
        // The data object has every member but "absent".
        let data_members = [
            (0usize, 1u32),
            (1, 7),
            (2, 3),
            (3, 0x11),
            (4, 10),
            (6, 0x11),
            (7, 1),
        ];
        let members: Vec<MemberModel> = data_members
            .iter()
            .map(|&(n, kind)| f.member(names[n], kind, 0, 0))
            .collect();
        let class = f.class(class_name, members);
        let items = f.array(1, 0, vec![Val::Int(10), Val::Int(20), Val::Int(30)]);
        let three = f.array(1, 0, vec![Val::Int(1), Val::Int(2), Val::Int(3)]);
        let object = f.object(
            class,
            vec![
                Val::Int(1),
                Val::Int(5),
                Val::Real(2.5),
                Val::Array(items),
                Val::Text(title),
                Val::Array(three),
                Val::Int(9),
            ],
        );
        // The native class: bool, int, float, hkArray of ints, string, a missing hkArray, a
        // C array of three ints and an enum.
        let natives = [
            f.native_member(names[0], 1, 0, 0, 0x00, 0, 0, 1),
            f.native_member(names[1], 7, 0, 0, 0x04, 0, 0, 4),
            f.native_member(names[2], 0xb, 0, 0, 0x08, 0, 0, 4),
            f.native_member(names[3], 0x16, 7, 0, 0x10, 0, 0, 12),
            f.native_member(names[4], 0x14, 2, 0, 0x1c, 0, 0, 4),
            f.native_member(names[5], 0x16, 7, 0, 0x28, 0, 0, 12),
            f.native_member(names[6], 7, 0, 3, 0x38, 0, 0, 12),
            f.native_member(names[7], 0x18, 7, 0, 0x44, 0, 0x77, 4),
        ];
        f.native_class(class_name, 0, natives.to_vec(), 0x50);
        let target = f.e.mem.alloc(0x60);
        let handle = f.handle(object);
        let copier = f.copier;
        let got = f.e.call(0x00dc_2d60, &args![copier, target, handle]).u32();
        assert_eq!(got, 0);
        let e = &f.e;
        assert_eq!(e.mem.u8(target), 1);
        assert_eq!(e.mem.u32(target + 4), 5);
        assert_eq!(e.mem.f32(target + 8), 2.5);
        // The hkArray: three ints in a new block, size 3, the capacity word 3 with bit 31.
        let (data, size, capacity) = (
            e.mem.u32(target + 0x10),
            e.mem.u32(target + 0x14),
            e.mem.u32(target + 0x18),
        );
        assert_eq!(size, 3);
        assert_eq!(capacity, 0x8000_0003);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [10, 20, 30]
        );
        // The string was duplicated into the package.
        let copy = e.mem.u32(target + 0x1c);
        assert_eq!(e.mem.cstr(copy), b"Title");
        assert_eq!(e.mem.u32(f.package + 0x38), 1);
        // The member the object lacks: an hkArray gets the empty capacity word.
        assert_eq!(e.mem.u32(target + 0x30), 0x8000_0000);
        // The C array: three elements 4 bytes apart.
        assert_eq!(
            [
                e.mem.u32(target + 0x38),
                e.mem.u32(target + 0x3c),
                e.mem.u32(target + 0x40)
            ],
            [1, 2, 3]
        );
        // The enum member went through `00d86bc0` with the member, its field and the integer.
        assert_eq!(
            calls(e, MEMBER_CALL_00D86BC0),
            vec![vec![natives[7], target + 0x44, 9]]
        );
        // The hkArray block was added to the package as a chunk of kind 0x18, 3 * 4 bytes.
        let chunks = e.mem.u32(f.package + 0x40);
        assert_eq!(e.mem.u32(f.package + 0x44), 1);
        assert_eq!(
            [
                e.mem.u32(chunks),
                e.mem.u32(chunks + 4),
                e.mem.u32(chunks + 8)
            ],
            [data, 12, 0x18]
        );
    }

    #[test]
    fn filling_stops_at_a_type_it_cannot_copy_and_follows_parent_classes() {
        let mut f = fixture(0);
        let count = f.text("count");
        let odd = f.text("odd");
        let parent_name = f.text("Base");
        let child_name = f.text("Derived");
        let data_members = vec![f.member(count, 7, 0, 0), f.member(odd, 7, 0, 0)];
        let class = f.class(child_name, data_members);
        let object = f.object(class, vec![Val::Int(5), Val::Int(6)]);
        // The member `count` belongs to the parent class; the child's `odd` has the type 0x15,
        // which cannot be copied.
        let base_member = f.native_member(count, 7, 0, 0, 0x04, 0, 0, 4);
        let base = f.native_class(parent_name, 0, vec![base_member], 8);
        let odd_member = f.native_member(odd, 0x15, 0, 0, 0x08, 0, 0, 4);
        f.native_class(child_name, base, vec![odd_member], 0x10);
        let target = f.e.mem.alloc(0x20);
        let handle = f.handle(object);
        let copier = f.copier;
        let got = f.e.call(0x00dc_2d60, &args![copier, target, handle]).u32();
        assert_eq!(got, 1);
        // The parent was not reached: the child's member stopped the copy first.
        assert_eq!(f.e.mem.u32(target + 4), 0);
        // Without the odd member the parent's member is filled.
        let mut f = fixture(0);
        let count = f.text("count");
        let parent_name = f.text("Base");
        let child_name = f.text("Derived");
        let data_members = vec![f.member(count, 7, 0, 0)];
        let class = f.class(child_name, data_members);
        let object = f.object(class, vec![Val::Int(5)]);
        let base_member = f.native_member(count, 7, 0, 0, 0x04, 0, 0, 4);
        let base = f.native_class(parent_name, 0, vec![base_member], 8);
        f.native_class(child_name, base, vec![], 0x10);
        let target = f.e.mem.alloc(0x20);
        let handle = f.handle(object);
        let copier = f.copier;
        assert_eq!(
            f.e.call(0x00dc_2d60, &args![copier, target, handle]).u32(),
            0
        );
        assert_eq!(f.e.mem.u32(target + 4), 5);
        // A class the registry does not know: nothing to fill.
        let unknown = f.text("Unknown");
        let class = f.class(unknown, vec![]);
        let object = f.object(class, vec![]);
        let handle = f.handle(object);
        assert_eq!(
            f.e.call(0x00dc_2d60, &args![copier, target, handle]).u32(),
            0
        );
    }

    /// The slot of the world that gets an object from its handle (`0x28`): writes the handle's
    /// low word (the object id) as the object handle.
    const WORLD_GET_OBJECT_SLOT: u32 = 0x0900_0700;

    /// A world the data objects live in: `getWorld` of any class answers it.
    fn install_world(f: &mut Fixture) -> u32 {
        let vtable = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(vtable + 0x28, WORLD_GET_OBJECT_SLOT);
        let world = f.e.mem.alloc(8);
        f.e.mem.set_u32(world, vtable);
        f.e.register(WORLD_GET_OBJECT_SLOT, |e, a| {
            let id = e.mem.u32(a[2]);
            e.mem.set_u32(a[1], id);
            ret(a[1])
        });
        f.e.register_double(DATA_CLASS_GET_WORLD, move |_, _| ret(world));
        world
    }

    /// The two-class world of the copier tests: a `Child` with an integer `v`, and a `Root` with
    /// a pointer `child` to a `Child`. Returns (root object, child object, root `hkClass`,
    /// child `hkClass`).
    fn child_and_root(f: &mut Fixture) -> (u32, u32, u32, u32) {
        let child_name = f.text("Child");
        let root_name = f.text("Root");
        let v = f.text("v");
        let child_member = f.text("child");
        let data_v = f.member(v, 1, 0, 0);
        let child_class = f.class(child_name, vec![data_v]);
        let data_child = f.member(child_member, 8, child_class, 0);
        let root_class = f.class(root_name, vec![data_child]);
        let child = f.object(child_class, vec![Val::Int(42)]);
        let root = f.object(root_class, vec![Val::Object(child)]);
        // Objects have a header (the vtable, then the size and the reference count at +4 and +6),
        // so the members start at +8.
        let native_v = f.native_member(v, 7, 0, 0, 8, 0, 0, 4);
        let child_hk = f.native_class(child_name, 0, vec![native_v], 0x10);
        let native_child = f.native_member(child_member, 0x14, 0x19, 0, 8, child_hk, 0, 4);
        let root_hk = f.native_class(root_name, 0, vec![native_child], 0x10);
        (root, child, root_hk, child_hk)
    }

    /// The pair `findOrCopy` writes for `object`.
    fn find_or_copy(f: &mut Fixture, object: u32, flag: u32) -> (u32, u32) {
        let handle = f.handle(object);
        let out = f.e.mem.alloc(8);
        let copier = f.copier;
        f.e.call(0x00dc_4430, &args![copier, out, handle, flag]);
        (f.e.mem.u32(out), f.e.mem.u32(out + 4))
    }

    #[test]
    fn an_object_is_copied_once_with_the_objects_it_points_to() {
        let mut f = fixture(1);
        let (root, child, root_hk, child_hk) = child_and_root(&mut f);
        let (native, class) = find_or_copy(&mut f, root, 1);
        assert_eq!(class, root_hk);
        let child_native = f.e.mem.u32(native + 8);
        assert_ne!(child_native, 0);
        assert_eq!(f.e.mem.u32(child_native + 8), 42);
        // Two 0x10-byte blocks from the blocks allocator, added as chunks of kind 6.
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 4).len(), 2);
        let chunks = f.e.mem.u32(f.package + 0x40);
        assert_eq!(f.e.mem.u32(f.package + 0x44), 2);
        assert_eq!(
            [f.e.mem.u32(chunks + 4), f.e.mem.u32(chunks + 8)],
            [0x10, 6]
        );
        // Both objects are in the copier's map; nothing is left in progress.
        assert_eq!(f.e.mem.u32(f.copier + 0x18 + 4), 2);
        assert_eq!(f.e.mem.u32(f.copier + 0x28), 0);
        let _ = (child, child_hk);
        // Asking again finds the copy in the map: the same pair, nothing new made.
        let allocations = calls(&f.e, ALLOCATOR_BLOCKS + 4).len();
        assert_eq!(find_or_copy(&mut f, root, 1), (native, root_hk));
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 4).len(), allocations);
    }

    #[test]
    fn an_object_that_points_back_to_itself_cannot_be_copied() {
        let mut f = fixture(1);
        let name = f.text("Loop");
        let next = f.text("next");
        let data_next = f.member(next, 8, 0, 0);
        let class = f.class(name, vec![data_next]);
        let object = f.object(class, vec![Val::Object(1)]);
        let native_next = f.native_member(next, 0x14, 0x19, 0, 0, 0, 0, 4);
        let hk = f.native_class(name, 0, vec![native_next], 8);
        // The member class of a self-referencing pointer is the class itself.
        f.e.mem.set_u32(native_next + 4, hk);
        let (native, class) = find_or_copy(&mut f, object, 1);
        // The inner copy finds the object in progress: no pointer, and the outer one fails.
        assert_eq!((native, class), (0, 0));
        assert_eq!(f.e.mem.u32(f.copier + 0x28), 0, "nothing left in progress");
        // Without tracking (the call's flag is 0) the map breaks the loop instead.
        let mut f = fixture(1);
        let name = f.text("Loop");
        let next = f.text("next");
        let data_next = f.member(next, 8, 0, 0);
        let class = f.class(name, vec![data_next]);
        let object = f.object(class, vec![Val::Object(1)]);
        let native_next = f.native_member(next, 0x14, 0x19, 0, 0, 0, 0, 4);
        let hk = f.native_class(name, 0, vec![native_next], 8);
        f.e.mem.set_u32(native_next + 4, hk);
        // The member's flag word has bit 9 set, so its flag argument (the inverse) is 0.
        f.e.mem.set_u16(native_next + 0x10, 0x0200);
        let (native, class) = find_or_copy(&mut f, object, 0);
        assert_ne!(native, 0);
        assert_eq!(class, hk);
        assert_eq!(f.e.mem.u32(native), native, "the pointer refers to itself");
    }

    #[test]
    fn null_objects_and_unnamed_classes_have_no_copy() {
        let mut f = fixture(1);
        assert_eq!(find_or_copy(&mut f, 0, 1), (0, 0));
        let class = f.class(0, vec![]);
        let object = f.object(class, vec![]);
        assert_eq!(find_or_copy(&mut f, object, 1), (0, 0));
        // A class the registry does not know has no native class: the pair is (block, 0)... the
        // exe sizes the block with a null class and so this case needs a registered class.
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 4).len(), 0);
    }

    #[test]
    fn copies_of_classes_with_a_vtable_count_their_references() {
        let mut f = fixture(1);
        let (root, _, root_hk, _) = child_and_root(&mut f);
        // Every class has a vtable and derives from the base class at `01268748`.
        f.e.register(CLASS_HAS_VTABLE, |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(a[1])
        });
        f.e.register(CLASS_IS_SUPER_CLASS, |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(a[1])
        });
        let references = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(f.copier + 0xc, references);
        f.e.register_double(POINTER_MAP_FIND_OR_INSERT_KEY, |_, _| ret(1));
        let (native, class) = find_or_copy(&mut f, root, 1);
        assert_eq!(class, root_hk);
        // `findOrInsertKey(map, allocator, native, 0)` for the child and the root; each slot's
        // count (the word after the key, 8 bytes per slot) went up.
        let keys: Vec<u32> = calls(&f.e, POINTER_MAP_FIND_OR_INSERT_KEY)
            .iter()
            .map(|w| w[2])
            .collect();
        assert_eq!(keys, vec![f.e.mem.u32(native + 8), native]);
        assert_eq!(f.e.mem.u32(references + 8 + 4), 2);
    }

    #[test]
    fn the_member_class_is_checked_against_the_class_of_the_object() {
        let mut f = fixture(0);
        let (_, child, _, child_hk) = child_and_root(&mut f);
        let other_name = f.text("Other");
        let other_hk = f.native_class(other_name, 0, vec![], 4);
        let member = f.native_member(other_name, 0x14, 0x19, 0, 0, 0, 0, 4);
        let handle = f.handle(child);
        let copier = f.copier;
        let check = |f: &mut Fixture, member_class: u32, object: u32| {
            f.e.mem.set_u32(member + 4, member_class);
            f.e.call(0x00dc_3a80, &args![copier, member, object]).u32()
        };
        // A null object has no class.
        let null = f.handle(0);
        assert_eq!(check(&mut f, child_hk, null), 0);
        // A member without a class takes the class of the object found by name.
        assert_eq!(check(&mut f, 0, handle), child_hk);
        // The same class name: the member's class.
        assert_eq!(check(&mut f, child_hk, handle), child_hk);
        // A different member class that is not a base of the object's: the member's class (no
        // attribute) ... and when the base check says it is a base, the object's class.
        assert_eq!(check(&mut f, other_hk, handle), other_hk);
        f.e.register(CLASS_IS_SUPER_CLASS, |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(a[1])
        });
        assert_eq!(check(&mut f, other_hk, handle), child_hk);
        // A class name the registry does not know: nothing when the member has a class.
        let stranger = f.text("Stranger");
        let class = f.class(stranger, vec![]);
        let strange_object = f.object(class, vec![]);
        let strange = f.handle(strange_object);
        assert_eq!(check(&mut f, child_hk, strange), 0);
        assert_eq!(check(&mut f, 0, strange), 0);
    }

    #[test]
    fn the_attribute_names_the_class_a_data_object_stands_for() {
        let mut f = fixture(0);
        let (_, child, _, child_hk) = child_and_root(&mut f);
        let other_name = f.text("Other");
        let other_hk = f.native_class(other_name, 0, vec![], 4);
        let member = f.native_member(other_name, 0x14, 0x19, 0, 0, other_hk, 0, 4);
        let handle = f.handle(child);
        let copier = f.copier;
        // The attribute `hk.DataObjectType` is a record whose first word points at the class
        // name; with a class that is not a base of the object's, its class is the answer.
        let child_name = f.natives.borrow().classes[&child_hk].name;
        let name_slot = f.e.mem.alloc(8);
        f.e.mem.set_u32(name_slot, child_name);
        let attribute = f.e.mem.alloc(8);
        f.e.mem.set_u32(attribute, name_slot);
        f.e.register_double(MEMBER_GET_ATTRIBUTE, move |_, _| ret(attribute));
        let got = f.e.call(0x00dc_3a80, &args![copier, member, handle]).u32();
        assert_eq!(got, child_hk);
        assert_eq!(
            calls(&f.e, MEMBER_GET_ATTRIBUTE),
            vec![vec![member, ATTRIBUTE_NAME]]
        );
    }

    #[test]
    fn the_copier_holds_and_releases_its_registry_and_package() {
        let mut f = fixture(1);
        let registry = f.e.mem.u32(f.copier);
        assert_eq!(f.references.borrow()[&registry], 1);
        // The constructor's own state: no package, the flag, three empty containers.
        let copier = f.e.mem.alloc(0x30);
        f.e.call(0x00dc_4f20, &args![copier, 0u32, 7u32]);
        assert_eq!(f.e.mem.u32(copier + 4), 0);
        assert_eq!(f.e.mem.u32(copier + 8), 7);
        assert_eq!(f.e.mem.i32(copier + 0xc + 8), -1);
        assert_eq!(f.e.mem.i32(copier + 0x18 + 8), 7);
        assert_eq!(f.e.mem.u32(copier + 0x2c), 0x8000_0000);
        // Destroying: the package and the registry lose a reference, and the pointers go.
        let package = f.e.mem.u32(f.copier + 4);
        f.references.borrow_mut().insert(package, 1);
        let copier = f.copier;
        f.e.call(0x00dc_43a0, &args![copier]);
        assert_eq!(f.references.borrow()[&registry], 0);
        assert_eq!(f.references.borrow()[&package], 0);
        assert_eq!(f.e.mem.u32(copier), 0);
        assert_eq!(f.e.mem.u32(copier + 4), 0);
        assert_eq!(calls(&f.e, POINTER_MAP_CLEAR_AND_DEALLOCATE).len(), 1);
    }

    // ---- copyIntoNativeArray ----

    /// A member record (type 0x16, the given subtype) and an empty destination `hkArray`.
    fn array_member(f: &mut Fixture, subtype: u8) -> (u32, u32) {
        let member = f.e.mem.alloc(0x18);
        f.e.mem.set_u8(member + 0xc, 0x16);
        f.e.mem.set_u8(member + 0xd, subtype);
        let dest = f.e.mem.alloc(12);
        (member, dest)
    }

    /// Copies the data array into a native array through `copyIntoNativeArray`.
    fn copy_into(f: &mut Fixture, subtype: u8, elements: Vec<Val>) -> (u32, u32, u32) {
        let (member, dest) = array_member(f, subtype);
        let array = f.array(1, 0, elements);
        let handle = f.handle(array);
        let copier = f.copier;
        let result =
            f.e.call(0x00dc_1d30, &args![copier, dest, member, handle])
                .u32();
        (result, dest, f.e.mem.u32(dest))
    }

    #[test]
    fn simple_arrays_become_native_arrays_of_the_member_s_element_size() {
        let mut f = fixture(0);
        f.e.register_double(CONVERT_MATRIX_3, |e, a| {
            e.mem.set_u32(a[0], 0xc0de);
            Ret::default()
        });
        // A boolean array: one byte each, 0 or 1.
        let (result, dest, data) =
            copy_into(&mut f, 1, vec![Val::Int(1), Val::Int(0), Val::Int(7)]);
        assert_eq!(result, 0);
        assert_eq!(f.e.mem.bytes(data, 3), vec![1, 0, 1]);
        // The size is the element count and the capacity word the size with bit 31 (the copier's
        // flag is 0: the storage belongs to the package).
        assert_eq!(f.e.mem.u32(dest + 4), 3);
        assert_eq!(f.e.mem.u32(dest + 8), 0x8000_0003);
        // The block went to the package as a chunk of kind 0x18, 3 bytes.
        let chunks = f.e.mem.u32(f.package + 0x40);
        assert_eq!(
            [
                f.e.mem.u32(chunks),
                f.e.mem.u32(chunks + 4),
                f.e.mem.u32(chunks + 8)
            ],
            [data, 3, 0x18]
        );
        // Bytes, words, doublewords and quadwords.
        let (_, _, data) = copy_into(&mut f, 2, vec![Val::Int(0x101), Val::Int(2)]);
        assert_eq!(f.e.mem.bytes(data, 2), vec![1, 2]);
        let (_, _, data) = copy_into(&mut f, 5, vec![Val::Int(0x10002), Val::Int(3)]);
        assert_eq!(f.e.mem.bytes(data, 4), vec![2, 0, 3, 0]);
        let (_, _, data) = copy_into(&mut f, 7, vec![Val::Int(-1), Val::Int(2)]);
        assert_eq!([f.e.mem.u32(data), f.e.mem.u32(data + 4)], [0xffff_ffff, 2]);
        let (_, _, data) = copy_into(&mut f, 0x1e, vec![Val::Int(9)]);
        assert_eq!(f.e.mem.u32(data), 9);
        let (_, _, data) = copy_into(&mut f, 9, vec![Val::Int(-2)]);
        assert_eq!(f.e.mem.u64(data), 0xffff_ffff_ffff_fffe);
        // Floats, and floats stored as their high halves.
        let (_, _, data) = copy_into(&mut f, 0xb, vec![Val::Real(1.5), Val::Real(-2.0)]);
        assert_eq!([f.e.mem.f32(data), f.e.mem.f32(data + 4)], [1.5, -2.0]);
        let (_, _, data) = copy_into(&mut f, 0x20, vec![Val::Real(1.0), Val::Real(-1.0)]);
        assert_eq!([f.e.mem.u16(data), f.e.mem.u16(data + 2)], [0x3f80, 0xbf80]);
        // Four floats each, 16 bytes apart.
        let a = floats(&mut f.e, &[1.0, 2.0, 3.0, 4.0]);
        let b = floats(&mut f.e, &[5.0, 6.0, 7.0, 8.0]);
        let (_, _, data) = copy_into(&mut f, 0xc, vec![Val::Vector(a), Val::Vector(b)]);
        assert_eq!(f.e.mem.f32(data + 16), 5.0);
        // A 3x3 matrix through the converter, 0x30 bytes apart.
        let (_, _, data) = copy_into(&mut f, 0xe, vec![Val::Vector(a), Val::Vector(b)]);
        assert_eq!(f.e.mem.u32(data), 0xc0de);
        assert_eq!(f.e.mem.u32(data + 0x30), 0xc0de);
        // A 4x4 matrix copy, 0x40 bytes apart.
        let sixteen: Vec<f32> = (0..16).map(|n| n as f32).collect();
        let matrix = floats(&mut f.e, &sixteen);
        let (_, _, data) = copy_into(&mut f, 0x11, vec![Val::Vector(matrix)]);
        assert_eq!(f.e.mem.f32(data + 4 * 5), 5.0);
    }

    /// Four floats, sixteen floats, in game memory.
    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (n, value) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * n as u32, *value);
        }
        block
    }

    #[test]
    fn an_empty_array_and_unknown_subtypes_are_dealt_with_before_any_allocation() {
        let mut f = fixture(0);
        // An empty array of an hkArray member: the capacity word says "nothing, not owned".
        let (result, dest, _) = copy_into(&mut f, 7, vec![]);
        assert_eq!(result, 0);
        assert_eq!(f.e.mem.u32(dest + 8), 0x8000_0000);
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 0xc).len(), 0);
        // Another member type keeps its capacity word.
        let member = f.e.mem.alloc(0x18);
        f.e.mem.set_u8(member + 0xc, 0x1a);
        let dest = f.e.mem.alloc(12);
        f.e.mem.set_u32(dest + 8, 0x1234);
        let array = f.array(1, 0, vec![]);
        let handle = f.handle(array);
        let copier = f.copier;
        f.e.call(0x00dc_1d30, &args![copier, dest, member, handle]);
        assert_eq!(f.e.mem.u32(dest + 8), 0x1234);
        // Subtypes above 0x21 and the unhandled ones answer 1; subtype 0 does nothing.
        for subtype in [0x13u8, 0x15, 0x18, 0x1a, 0x1b, 0x1f, 0x22, 0xff] {
            let (result, _, _) = copy_into(&mut f, subtype, vec![Val::Int(1)]);
            assert_eq!(result, 1, "subtype {subtype:#x}");
        }
        let (result, dest, data) = copy_into(&mut f, 0, vec![Val::Int(1)]);
        assert_eq!(result, 0);
        assert_eq!(data, 0);
        assert_eq!(f.e.mem.u32(dest + 8), 0x8000_0000);
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 0xc).len(), 0);
    }

    #[test]
    fn the_copier_flag_leaves_the_capacity_word_without_bit_31() {
        let mut f = fixture(1);
        let (_, dest, _) = copy_into(&mut f, 7, vec![Val::Int(1), Val::Int(2)]);
        assert_eq!(f.e.mem.u32(dest + 8), 2);
    }

    #[test]
    fn string_arrays_are_filled_twice_for_subtype_0x1d_and_once_for_0x21() {
        let mut f = fixture(0);
        let a = f.text("a");
        let b = f.text("b");
        // 0x1d: the exe falls from its loop into the body of 0x21: two blocks, four copies.
        let (result, dest, data) = copy_into(&mut f, 0x1d, vec![Val::Text(a), Val::Text(b)]);
        assert_eq!(result, 0);
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 0xc).len(), 2);
        assert_eq!(calls(&f.e, STRING_DUP).len(), 4);
        assert_eq!(
            f.e.mem.u32(f.package + 0x38),
            4,
            "four strings in the package"
        );
        assert_eq!(f.e.mem.cstr(f.e.mem.u32(data)), b"a");
        assert_eq!(f.e.mem.cstr(f.e.mem.u32(data + 4)), b"b");
        // The package's last two strings are the copies the array holds.
        let strings = f.e.mem.u32(f.package + 0x34);
        assert_eq!(f.e.mem.u32(strings + 8), f.e.mem.u32(data));
        assert_eq!(f.e.mem.u32(dest + 4), 2);
        // The chunk is the second block, 2 * 4 bytes.
        let chunks = f.e.mem.u32(f.package + 0x40);
        assert_eq!(
            [
                f.e.mem.u32(chunks),
                f.e.mem.u32(chunks + 4),
                f.e.mem.u32(chunks + 8)
            ],
            [data, 8, 0x18]
        );
        // 0x21: one block, two copies (bit 0 is only set with the copier's flag).
        let mut f = fixture(0);
        let a = f.text("a");
        let (_, _, data) = copy_into(&mut f, 0x21, vec![Val::Text(a)]);
        assert_eq!(calls(&f.e, ALLOCATOR_BLOCKS + 0xc).len(), 1);
        assert_eq!(calls(&f.e, STRING_DUP).len(), 1);
        assert_eq!(f.e.mem.u32(data) & 1, 0);
        // With the copier's flag subtype 0x1d is refused: the first element stops the loop, but
        // the array's block still becomes a chunk.
        let mut f = fixture(1);
        let a = f.text("a");
        let b = f.text("b");
        let (result, _, _) = copy_into(&mut f, 0x1d, vec![Val::Text(a), Val::Text(b)]);
        assert_eq!(result, 1);
        assert_eq!(calls(&f.e, STRING_DUP).len(), 0);
        assert_eq!(f.e.mem.u32(f.package + 0x44), 1);
    }

    #[test]
    fn object_arrays_are_copied_as_pointers_and_struct_arrays_in_place() {
        let mut f = fixture(1);
        let (root, child, _, child_hk) = child_and_root(&mut f);
        // 0x14: an array of pointers to the copies; a null element stays 0.
        let (result, _, data) = copy_into(&mut f, 0x14, vec![Val::Object(child), Val::Object(0)]);
        assert_eq!(result, 0);
        let native = f.e.mem.u32(data);
        assert_ne!(native, 0);
        assert_eq!(f.e.mem.u32(native + 8), 42);
        assert_eq!(f.e.mem.u32(data + 4), 0);
        // 0x1c: pointer and class pairs, 8 bytes each.
        let (result, _, data) = copy_into(&mut f, 0x1c, vec![Val::Object(child)]);
        assert_eq!(result, 0);
        assert_eq!(f.e.mem.u32(f.e.mem.u32(data) + 8), 42);
        assert_eq!(f.e.mem.u32(data + 4), child_hk);
        // 0x19: structs laid out in the block, each filled in place: the member class comes
        // from the first element's class name.
        let mut f = fixture(0);
        let (_, child, _, child_hk) = {
            let world = child_and_root(&mut f);
            (world.0, world.1, world.2, world.3)
        };
        let second = f.object(1, vec![Val::Int(43)]);
        let (member, dest) = array_member(&mut f, 0x19);
        let array = f.array(9, 1, vec![Val::Object(child), Val::Object(second)]);
        let handle = f.handle(array);
        let copier = f.copier;
        let result =
            f.e.call(0x00dc_1d30, &args![copier, dest, member, handle])
                .u32();
        assert_eq!(result, 0);
        let data = f.e.mem.u32(dest);
        assert_eq!([f.e.mem.u32(data + 8), f.e.mem.u32(data + 0x18)], [42, 43]);
        // The block was asked for by the object size (2 * 0x10) and added as a chunk of kind 0x18.
        let chunks = f.e.mem.u32(f.package + 0x40);
        assert_eq!(
            [f.e.mem.u32(chunks + 4), f.e.mem.u32(chunks + 8)],
            [0x20, 0x18]
        );
        let _ = (root, child_hk);
    }

    // ---- toObject and the resource builders ----

    /// Doubles for what the conversions hand their results to: the package, the resource, the
    /// finisher (slot `0x10` of the object passed in; answers `finished`), the reference count
    /// map (every object has `count` references) and the error report. Returns the finisher
    /// object, the pairs given to the resource constructor and the report log.
    struct Conversion {
        finisher: u32,
        pairs: Shared<Vec<(u32, u32)>>,
        reports: Shared<Vec<Vec<u32>>>,
        messages: Shared<Vec<u32>>,
    }

    fn install_conversion(f: &mut Fixture, finished: u32, count: u32) -> Conversion {
        install_world(f);
        f.e.register(PACKFILE_DATA_CONSTRUCT, |_, a| ret(a[0]));
        let pairs: Shared<Vec<(u32, u32)>> = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&pairs);
        f.e.register_double(RESOURCE_CONSTRUCT, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            ret(a[0])
        });
        f.e.register(RESOURCE_SET_REGISTRY, |_, _| Ret::default());
        f.e.register(RESOURCE_SET_ARGUMENT, |_, _| Ret::default());
        f.e.register_double(POINTER_MAP_GET_WITH_DEFAULT, move |_, _| ret(count));
        let finisher_vtable = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(finisher_vtable + 0x10, FINISHER_SLOT);
        let finisher = f.e.mem.alloc(8);
        f.e.mem.set_u32(finisher, finisher_vtable);
        f.e.register_double(FINISHER_SLOT, move |_, _| ret(finished));
        // The error report: a stream chain and the reporter's slot `0xc`.
        let messages: Shared<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
        let appended = Rc::clone(&messages);
        f.e.register_double(STREAM_CONSTRUCT, |_, _| Ret::default());
        f.e.register_double(STREAM_APPEND, move |_, a| {
            appended.borrow_mut().push(a[1]);
            ret(a[0])
        });
        f.e.register_double(STREAM_DESTRUCT, |_, _| Ret::default());
        let reports: Shared<Vec<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let logged = Rc::clone(&reports);
        f.e.register_double(0x0900_0800, move |_, a| {
            logged.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let reporter_vtable = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(reporter_vtable + 0xc, 0x0900_0800);
        let reporter = f.e.mem.alloc(8);
        f.e.mem.set_u32(reporter, reporter_vtable);
        f.e.set_global(ERROR_REPORTER, reporter);
        Conversion {
            finisher,
            pairs,
            reports,
            messages,
        }
    }

    #[test]
    fn converting_an_object_builds_a_resource_and_sets_the_headers_of_counted_objects() {
        let mut f = fixture(1);
        let (root, _, root_hk, child_hk) = child_and_root(&mut f);
        let conversion = install_conversion(&mut f, 1, 3);
        let handle = f.handle(root);
        let (copier, finisher) = (f.copier, conversion.finisher);
        let old_package = f.package;
        let resource =
            f.e.call(0x00dc_4710, &args![copier, handle, finisher])
                .u32();
        // The resource block (0x18 bytes, its size kept at +4) was given the root's pair.
        assert_ne!(resource, 0);
        assert_eq!(f.e.mem.u16(resource + 4), 0x18);
        let pairs = conversion.pairs.borrow().clone();
        assert_eq!(pairs.len(), 1);
        let (native, class) = pairs[0];
        assert_eq!(class, root_hk);
        // Every counted object has its size and count in its header (+4, +6).
        let child_native = f.e.mem.u32(native + 8);
        for object in [native, child_native] {
            assert_eq!(f.e.mem.u16(object + 4), 0x10, "the class's object size");
            assert_eq!(f.e.mem.u16(object + 6), 3, "the reference count");
        }
        // Both objects were finished, with their class names, and the copier let go of its
        // package (the old one too).
        let names: Vec<u32> = calls(&f.e, FINISHER_SLOT).iter().map(|w| w[2]).collect();
        let root_name = f.natives.borrow().classes[&root_hk].name;
        let child_name = f.natives.borrow().classes[&child_hk].name;
        assert_eq!(names.len(), 2);
        assert!(names.contains(&root_name) && names.contains(&child_name));
        assert_eq!(f.e.mem.u32(copier + 4), 0);
        assert!(calls(&f.e, REMOVE_REFERENCE)
            .iter()
            .any(|w| w[0] == old_package));
        assert!(conversion.reports.borrow().is_empty());
    }

    #[test]
    fn a_native_object_with_a_vtable_that_cannot_be_finished_is_reported() {
        let mut f = fixture(1);
        let (root, _, _, child_hk) = child_and_root(&mut f);
        f.e.register(CLASS_HAS_VTABLE, |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(a[1])
        });
        let conversion = install_conversion(&mut f, 0, 0);
        let handle = f.handle(root);
        let (copier, finisher) = (f.copier, conversion.finisher);
        f.e.call(0x00dc_4710, &args![copier, handle, finisher]);
        // One report for each of the two objects: severity 1, the id, the message buffer, the
        // source file and the line 0x133.
        let reports = conversion.reports.borrow().clone();
        assert_eq!(reports.len(), 2);
        for report in &reports {
            assert_eq!(report[1], 1);
            assert_eq!(report[2], 0x7c30_3834);
            assert_eq!(report[4], SOURCE_FILE);
            assert_eq!(report[5], 0x133);
        }
        // The message is the head, the class name and the tail.
        let messages = conversion.messages.borrow().clone();
        assert_eq!(messages.len(), 6);
        assert_eq!(messages[0], MESSAGE_HEAD);
        assert_eq!(messages[2], MESSAGE_TAIL);
        let _ = child_hk;
    }

    #[test]
    fn converting_gives_up_when_the_object_cannot_be_copied() {
        let mut f = fixture(1);
        let name = f.text("Loop");
        let next = f.text("next");
        let data_next = f.member(next, 8, 0, 0);
        let class = f.class(name, vec![data_next]);
        let object = f.object(class, vec![Val::Object(1)]);
        let native_next = f.native_member(next, 0x14, 0x19, 0, 0, 0, 0, 4);
        let hk = f.native_class(name, 0, vec![native_next], 8);
        f.e.mem.set_u32(native_next + 4, hk);
        let conversion = install_conversion(&mut f, 1, 0);
        let handle = f.handle(object);
        let (copier, finisher) = (f.copier, conversion.finisher);
        let result =
            f.e.call(0x00dc_4710, &args![copier, handle, finisher])
                .u32();
        assert_eq!(result, 0);
        assert_eq!(f.e.mem.u32(copier + 4), 0, "the package is released");
        assert!(conversion.pairs.borrow().is_empty());
        assert_eq!(calls(&f.e, FINISHER_SLOT).len(), 0);
    }

    #[test]
    fn converting_with_a_registry_uses_a_copier_built_on_the_stack() {
        let mut f = fixture(1);
        let (root, _, root_hk, _) = child_and_root(&mut f);
        let conversion = install_conversion(&mut f, 1, 0);
        let registry = f.e.mem.u32(f.copier);
        let handle = f.handle(root);
        let before = f.references.borrow()[&registry];
        let resource =
            f.e.call(0x00dc_32c0, &args![handle, registry, conversion.finisher])
                .u32();
        assert_ne!(resource, 0);
        assert_eq!(conversion.pairs.borrow()[0].1, root_hk);
        // The stack copier took a reference to the registry and gave it back.
        assert_eq!(f.references.borrow()[&registry], before);
        assert_eq!(
            calls(&f.e, ADD_REFERENCE)
                .iter()
                .filter(|w| w[0] == registry)
                .count(),
            1
        );
    }

    #[test]
    fn a_resource_names_the_objects_and_sets_the_package_contents() {
        let mut f = fixture(0);
        let (root, _, root_hk, child_hk) = child_and_root(&mut f);
        install_world(&mut f);
        f.e.register(PACKFILE_DATA_CONSTRUCT, |_, a| ret(a[0]));
        f.e.register_double(PACKFILE_DATA_SET_CONTENTS, |_, _| Ret::default());
        f.e.register_double(POINTER_MAP_INSERT_00CB1870, |_, _| Ret::default());
        let registry = f.e.mem.u32(f.copier);
        let handle = f.handle(root);
        let package = f.e.call(0x00dc_3300, &args![handle, registry]).u32();
        assert_ne!(package, 0);
        assert_eq!(f.e.mem.u16(package + 4), 0x64);
        // The package got the root object and its class name.
        let root_name = f.natives.borrow().classes[&root_hk].name;
        let child_name = f.natives.borrow().classes[&child_hk].name;
        let contents = calls(&f.e, PACKFILE_DATA_SET_CONTENTS);
        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0][0], package);
        assert_eq!(contents[0][2], root_name);
        // Every native object is entered with its class name in the package's map at +0x10.
        let inserted = calls(&f.e, POINTER_MAP_INSERT_00CB1870);
        assert_eq!(inserted.len(), 2);
        let mut names: Vec<u32> = inserted.iter().map(|w| w[3]).collect();
        names.sort_unstable();
        let mut expected = vec![root_name, child_name];
        expected.sort_unstable();
        assert_eq!(names, expected);
        assert!(inserted.iter().all(|w| w[0] == package + 0x10));
        assert!(inserted.iter().any(|w| w[2] == contents[0][1]));
        // One reference was added for the caller, the copier's own was released.
        assert_eq!(
            calls(&f.e, ADD_REFERENCE)
                .iter()
                .filter(|w| w[0] == package)
                .count(),
            1
        );
        assert_eq!(
            calls(&f.e, REMOVE_REFERENCE)
                .iter()
                .filter(|w| w[0] == package)
                .count(),
            1
        );
    }

    #[test]
    fn a_resource_is_not_made_of_a_null_object_or_a_failed_copy() {
        let mut f = fixture(0);
        install_world(&mut f);
        f.e.register(PACKFILE_DATA_CONSTRUCT, |_, a| ret(a[0]));
        f.e.register_double(PACKFILE_DATA_SET_CONTENTS, |_, _| Ret::default());
        f.e.register_double(POINTER_MAP_INSERT_00CB1870, |_, _| Ret::default());
        let registry = f.e.mem.u32(f.copier);
        let null = f.handle(0);
        assert_eq!(f.e.call(0x00dc_3300, &args![null, registry]).u32(), 0);
        assert_eq!(calls(&f.e, PACKFILE_DATA_CONSTRUCT).len(), 0);
        // A class with a member type that cannot be copied (0x15): the copy fails, the package
        // is released and nothing is set.
        let name = f.text("Bad");
        let member_name = f.text("odd");
        let data_member = f.member(member_name, 7, 0, 0);
        let class = f.class(name, vec![data_member]);
        let object = f.object(class, vec![Val::Int(1)]);
        let native_member = f.native_member(member_name, 0x15, 0, 0, 0, 0, 0, 4);
        f.native_class(name, 0, vec![native_member], 8);
        let handle = f.handle(object);
        assert_eq!(f.e.call(0x00dc_3300, &args![handle, registry]).u32(), 0);
        assert_eq!(calls(&f.e, PACKFILE_DATA_CONSTRUCT).len(), 1);
        // The package and, with each of the two copiers, the registry are released.
        let removed = calls(&f.e, REMOVE_REFERENCE);
        assert_eq!(removed.iter().filter(|w| w[0] == registry).count(), 2);
        assert_eq!(removed.iter().filter(|w| w[0] != registry).count(), 1);
        assert_eq!(calls(&f.e, PACKFILE_DATA_SET_CONTENTS).len(), 0);
    }

    // ---- The Cloner: data objects into another world ----

    /// A class id is also the address of a word holding the class vtable (a class
    /// implementation): slot 8 answers the name, `0x24` the number of members, `0x28` a member's
    /// record. The classes of the cloner tests therefore have ids that are multiples of 8, from
    /// `0x1000` (a page the test maps for them).
    const CLASS_NAME_SLOT: u32 = 0x0900_0a08;
    const CLASS_COUNT_SLOT: u32 = 0x0900_0a24;
    const CLASS_MEMBER_SLOT: u32 = 0x0900_0a28;
    /// The source world: slot `0x1c` answers its contents object, `0x24` a class by name.
    const SOURCE_CONTENTS_SLOT: u32 = 0x0900_0b1c;
    const SOURCE_FIND_SLOT: u32 = 0x0900_0b24;
    /// The destination world: slot `0xc` defines a class, `0x10` makes an object, `0x14` an
    /// array, `0x24` finds a class by name and `0x28` gets an object from its handle.
    const DESTINATION_DEFINE_SLOT: u32 = 0x0900_0c0c;
    const DESTINATION_OBJECT_SLOT: u32 = 0x0900_0c10;
    const DESTINATION_ARRAY_SLOT: u32 = 0x0900_0c14;
    const DESTINATION_FIND_SLOT: u32 = 0x0900_0c24;
    const DESTINATION_GET_OBJECT_SLOT: u32 = 0x0900_0c28;

    #[derive(Default)]
    struct CloneState {
        /// The ids of the classes the destination world has (defined or given by the test).
        made: Vec<u32>,
        /// The names of the classes the destination defined, in order.
        defined: Vec<Vec<u8>>,
        /// The source world's contents object.
        contents: u32,
    }

    struct Worlds {
        source: u32,
        destination: u32,
        vtable: u32,
        cloner: u32,
        state: Shared<CloneState>,
    }

    fn push_spaced_class(model: &mut Model, class: ClassModel) -> u32 {
        while model.classes.len() < 0xfff || model.classes.len() % 8 != 7 {
            model.classes.push(ClassModel {
                name: 0,
                parent: 0,
                version: 0,
                members: Vec::new(),
            });
        }
        model.classes.push(class);
        model.classes.len() as u32
    }

    /// The class of the source or destination world named `name` (0 when there is none).
    fn find_named(e: &Engine, model: &Model, made: &[u32], in_destination: bool, name: u32) -> u32 {
        let wanted = e.mem.cstr(name);
        (1..=model.classes.len() as u32)
            .filter(|id| made.contains(id) == in_destination)
            .find(|id| {
                let class = model.classes[*id as usize - 1].name;
                class != 0 && e.mem.cstr(class) == wanted
            })
            .unwrap_or(0)
    }

    fn world_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for &(slot, double) in slots {
            e.mem.set_u32(vtable + slot, double);
        }
        let world = e.mem.alloc(8);
        e.mem.set_u32(world, vtable);
        world
    }

    /// The doubles of two worlds over the fixture's data model, the class implementations, the
    /// value setters and the array operations the `Cloner` reaches. Returns the worlds and a
    /// `Cloner` (0x14 bytes) over them with its object map reserved for no elements.
    fn install_clone(f: &mut Fixture) -> Worlds {
        let state: Shared<CloneState> = Rc::new(RefCell::new(CloneState::default()));
        let model = Rc::clone(&f.model);
        let e = &mut f.e;
        e.map(0x1000, 0x2000);
        // The class implementations.
        let vtable = e.mem.alloc(0x40);
        for (slot, double) in [
            (8, CLASS_NAME_SLOT),
            (0x24, CLASS_COUNT_SLOT),
            (0x28, CLASS_MEMBER_SLOT),
        ] {
            e.mem.set_u32(vtable + slot, double);
        }
        let m = Rc::clone(&model);
        e.register_double(CLASS_NAME_SLOT, move |_, a| {
            ret(m.borrow().classes[a[0] as usize - 1].name)
        });
        let m = Rc::clone(&model);
        e.register_double(CLASS_COUNT_SLOT, move |_, a| {
            ret(m.borrow().classes[a[0] as usize - 1].members.len() as u32)
        });
        let m = Rc::clone(&model);
        e.register_double(CLASS_MEMBER_SLOT, move |e, a| {
            let model = m.borrow();
            let member = &model.classes[a[0] as usize - 1].members[a[1] as usize];
            e.mem.set_u32(a[2], member.name);
            e.mem.set_u32(a[2] + 8, member.type_word);
            e.mem.set_u32(a[2] + 0xc, member.class);
            e.mem.set_u32(a[2] + 0x10, member.tuple);
            Ret::default()
        });
        // The worlds.
        let source = world_object(e, &[(0x1c, SOURCE_CONTENTS_SLOT), (0x24, SOURCE_FIND_SLOT)]);
        let destination = world_object(
            e,
            &[
                (0xc, DESTINATION_DEFINE_SLOT),
                (0x10, DESTINATION_OBJECT_SLOT),
                (0x14, DESTINATION_ARRAY_SLOT),
                (0x24, DESTINATION_FIND_SLOT),
                (0x28, DESTINATION_GET_OBJECT_SLOT),
            ],
        );
        let s = Rc::clone(&state);
        e.register_double(SOURCE_CONTENTS_SLOT, move |e, a| {
            e.mem.set_u32(a[1], s.borrow().contents);
            ret(a[1])
        });
        for (slot, in_destination) in [(SOURCE_FIND_SLOT, false), (DESTINATION_FIND_SLOT, true)] {
            let (m, s) = (Rc::clone(&model), Rc::clone(&state));
            e.register_double(slot, move |e, a| {
                let made = s.borrow().made.clone();
                ret(find_named(e, &m.borrow(), &made, in_destination, a[1]))
            });
        }
        let (m, s) = (Rc::clone(&model), Rc::clone(&state));
        e.register_double(DESTINATION_DEFINE_SLOT, move |e, a| {
            let token = e.mem.u32(a[1]);
            let (name, parent, members) = {
                let model = m.borrow();
                let class = &model.classes[token as usize - 1];
                let parent = if class.parent == 0 {
                    0
                } else {
                    let parent_name = model.classes[class.parent as usize - 1].name;
                    find_named(e, &model, &s.borrow().made, true, parent_name)
                };
                let members: Vec<MemberModel> = class
                    .members
                    .iter()
                    .map(|member| MemberModel {
                        name: member.name,
                        type_word: member.type_word,
                        class: 0,
                        tuple: member.tuple,
                    })
                    .collect();
                (class.name, parent, members)
            };
            let id = push_spaced_class(
                &mut m.borrow_mut(),
                ClassModel {
                    name,
                    parent,
                    version: 1,
                    members,
                },
            );
            e.mem.set_u32(id, vtable);
            let mut state = s.borrow_mut();
            state.made.push(id);
            state.defined.push(e.mem.cstr(name));
            Ret::default()
        });
        let m = Rc::clone(&model);
        e.register_double(DESTINATION_OBJECT_SLOT, move |e, a| {
            let class = e.mem.u32(a[1]);
            let mut model = m.borrow_mut();
            let count = model.classes[class as usize - 1].members.len();
            model.objects.push(ObjectModel {
                class,
                values: vec![Val::Int(0); count],
            });
            ret(model.objects.len() as u32)
        });
        let m = Rc::clone(&model);
        e.register_double(DESTINATION_ARRAY_SLOT, move |e, a| {
            // The array becomes the value of the object's member.
            let (object, name) = (e.mem.u32(a[1]), e.mem.u32(a[2]));
            let kind = e.mem.u32(a[2] + 8) & 0xf;
            let mut model = m.borrow_mut();
            model.arrays.push(ArrayModel {
                kind,
                class: 0,
                elements: Vec::new(),
                view: Vec::new(),
            });
            let array = model.arrays.len() as u32;
            let index = model.member_index(object, name).unwrap();
            model.objects[object as usize - 1].values[index as usize] = Val::Array(array);
            ret(array)
        });
        e.register(DESTINATION_GET_OBJECT_SLOT, |e, a| {
            let id = e.mem.u32(a[2]);
            e.mem.set_u32(a[1], id);
            ret(a[1])
        });
        // The handles and the classes of the two worlds.
        e.register(DATA_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(DATA_OBJECT_CONSTRUCT_COPY, |e, a| {
            let id = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], id);
            ret(a[0])
        });
        e.register(DATA_ARRAY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(DATA_CLASS_GET_CINFO, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], id);
            ret(a[1])
        });
        let s = Rc::clone(&state);
        e.register_double(DATA_CLASS_GET_WORLD, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(if s.borrow().made.contains(&id) {
                destination
            } else {
                source
            })
        });
        e.register_double(OBJECT_MAP_CLEAR_AND_DEALLOCATE, |_, _| Ret::default());
        // The arrays.
        let m = Rc::clone(&model);
        e.register_double(DATA_ARRAY_GET_TYPE, move |e, a| {
            let id = e.mem.u32(a[0]);
            ret(m.borrow().arrays[id as usize - 1].kind)
        });
        let m = Rc::clone(&model);
        e.register_double(DATA_ARRAY_GET_CLASS, move |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem
                .set_u32(a[1], m.borrow().arrays[id as usize - 1].class);
            ret(a[1])
        });
        let m = Rc::clone(&model);
        e.register_double(DATA_ARRAY_SET_SIZE, move |e, a| {
            let id = e.mem.u32(a[0]) as usize;
            let mut model = m.borrow_mut();
            let (kind, class) = (model.arrays[id - 1].kind, model.arrays[id - 1].class);
            let wanted = a[1] as usize;
            while model.arrays[id - 1].elements.len() < wanted {
                let value = if kind == 9 {
                    let count = model.classes[class as usize - 1].members.len();
                    model.objects.push(ObjectModel {
                        class,
                        values: vec![Val::Int(0); count],
                    });
                    Val::Object(model.objects.len() as u32)
                } else {
                    Val::Int(0)
                };
                model.arrays[id - 1].elements.push(value);
            }
            model.arrays[id - 1].elements.truncate(wanted);
            Ret::default()
        });
        let m = Rc::clone(&model);
        e.register_double(DATA_ARRAY_SWIZZLE_OBJECT_MEMBER, move |e, a| {
            let id = e.mem.u32(a[0]) as usize;
            let mut model = m.borrow_mut();
            let view: Vec<(u32, u32)> = model.arrays[id - 1]
                .elements
                .iter()
                .map(|element| match element {
                    Val::Object(object) => (*object, model.member_index(*object, a[2]).unwrap()),
                    _ => panic!("not an array of structs"),
                })
                .collect();
            model.arrays.push(ArrayModel {
                kind: 0,
                class: 0,
                elements: vec![Val::Int(0); view.len()],
                view,
            });
            e.mem.set_u32(a[1], model.arrays.len() as u32);
            ret(a[1])
        });
        // The value types and setters, for object members and array elements.
        for (get_type, in_array) in [(ARRAY_VALUE_GET_TYPE, true), (OBJECT_VALUE_GET_TYPE, false)] {
            let m = Rc::clone(&model);
            e.register_double(get_type, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                ret(match m.borrow().value(container, index, in_array) {
                    Val::Int(_) => 2,
                    Val::Real(_) => 3,
                    Val::Vector(_) => 4,
                    Val::Object(_) => 8,
                    Val::Text(_) => 10,
                    Val::Array(_) => 0,
                })
            });
        }
        for in_array in [false, true] {
            let (set_int, set_real, set_vec, set_object, set_string) = if in_array {
                (
                    ARRAY_VALUE_SET_INT,
                    ARRAY_VALUE_SET_REAL,
                    ARRAY_VALUE_SET_VEC,
                    ARRAY_VALUE_SET_OBJECT,
                    ARRAY_VALUE_SET_STRING,
                )
            } else {
                (
                    OBJECT_VALUE_SET_INT,
                    OBJECT_VALUE_SET_REAL,
                    OBJECT_VALUE_SET_VEC,
                    OBJECT_VALUE_SET_OBJECT,
                    OBJECT_VALUE_SET_STRING,
                )
            };
            let m = Rc::clone(&model);
            e.register_double(set_int, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                m.borrow_mut()
                    .set_value(container, index, in_array, Val::Int(a[1] as i32));
                Ret::default()
            });
            let m = Rc::clone(&model);
            e.register_double(set_real, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                m.borrow_mut().set_value(
                    container,
                    index,
                    in_array,
                    Val::Real(f32::from_bits(a[1])),
                );
                Ret::default()
            });
            let m = Rc::clone(&model);
            e.register_double(set_vec, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                m.borrow_mut()
                    .set_value(container, index, in_array, Val::Vector(a[1]));
                Ret::default()
            });
            let m = Rc::clone(&model);
            e.register_double(set_object, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                let object = e.mem.u32(a[1]);
                m.borrow_mut()
                    .set_value(container, index, in_array, Val::Object(object));
                Ret::default()
            });
            let m = Rc::clone(&model);
            e.register_double(set_string, move |e, a| {
                let (container, index) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                m.borrow_mut()
                    .set_value(container, index, in_array, Val::Text(a[1]));
                Ret::default()
            });
        }
        // The cloner, with its object map reserved for no elements as `deepCopyWorld` does.
        let cloner = e.mem.alloc(0x14);
        e.mem.set_u32(cloner, destination);
        e.mem.set_u32(cloner + 4, source);
        let objects: Ptr<HkMapBase> = Ptr::new(cloner + 8);
        e.set(objects, HkMapBase::m_elem, 0);
        e.set(objects, HkMapBase::m_numElems, 0);
        e.set(objects, HkMapBase::m_hashMod, -1);
        let allocator = router_allocator(e, 0x10);
        e.call(0x00dc_3bd0, &args![cloner + 8, allocator, 0u32]);
        Worlds {
            source,
            destination,
            vtable,
            cloner,
            state,
        }
    }

    impl Worlds {
        /// A class of the source world.
        fn source_class(
            &self,
            f: &mut Fixture,
            name: u32,
            parent: u32,
            members: Vec<MemberModel>,
        ) -> u32 {
            let id = push_spaced_class(
                &mut f.model.borrow_mut(),
                ClassModel {
                    name,
                    parent,
                    version: 1,
                    members,
                },
            );
            f.e.mem.set_u32(id, self.vtable);
            id
        }

        /// A class the destination world already has.
        fn destination_class(&self, f: &mut Fixture, name: u32, members: Vec<MemberModel>) -> u32 {
            let id = self.source_class(f, name, 0, members);
            self.state.borrow_mut().made.push(id);
            id
        }

        fn defined(&self) -> Vec<String> {
            self.state
                .borrow()
                .defined
                .iter()
                .map(|name| String::from_utf8(name.clone()).unwrap())
                .collect()
        }
    }

    fn object_values(f: &Fixture, id: u32) -> Vec<Val> {
        f.model.borrow().objects[id as usize - 1].values.clone()
    }

    fn array_values(f: &Fixture, id: u32) -> Vec<Val> {
        f.model.borrow().arrays[id as usize - 1].elements.clone()
    }

    /// A `Value` (container id and index) in memory.
    fn value_at(f: &mut Fixture, container: u32, index: u32) -> u32 {
        let block = f.e.mem.alloc(8);
        f.e.mem.set_u32(block, container);
        f.e.mem.set_u32(block + 4, index);
        block
    }

    #[test]
    fn simple_values_are_copied_by_their_type() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let names: Vec<u32> = ["i", "r", "v", "s", "a"]
            .iter()
            .map(|n| f.text(n))
            .collect();
        let members: Vec<MemberModel> = names.iter().map(|&n| f.member(n, 2, 0, 0)).collect();
        let class_name = f.text("Simple");
        let class = f.class(class_name, members);
        let vector = floats(&mut f.e, &[1.0, 2.0, 3.0, 4.0]);
        let text = f.text("hello");
        let sub = f.array(2, 0, vec![]);
        let source = f.object(
            class,
            vec![
                Val::Int(7),
                Val::Real(1.5),
                Val::Vector(vector),
                Val::Text(text),
                Val::Array(sub),
            ],
        );
        let destination = f.object(class, vec![Val::Int(0); 5]);
        // The object member kind (`copySimpleValue<hkDataObject::Value>`).
        for index in 0..5 {
            let value = value_at(&mut f, source, index);
            f.e.call(0x00dc_5ab0, &args![w.cloner, destination, index, value]);
        }
        // An array member has no simple type: it is left as it was.
        assert_eq!(
            object_values(&f, destination),
            vec![
                Val::Int(7),
                Val::Real(1.5),
                Val::Vector(vector),
                Val::Text(text),
                Val::Int(0)
            ]
        );
        // A vector of type 4 is four floats.
        let sets = calls(&f.e, OBJECT_VALUE_SET_VEC);
        assert_eq!(sets.len(), 1);
        assert_eq!([sets[0][1], sets[0][2]], [vector, 4]);
        // The array element kind (`copySimpleValue<hkDataArray::Value>`).
        let source = f.array(2, 0, vec![Val::Int(-3), Val::Real(0.25), Val::Text(text)]);
        let destination = f.array(2, 0, vec![Val::Int(0); 3]);
        for index in 0..3 {
            let value = value_at(&mut f, source, index);
            f.e.call(0x00dc_37f0, &args![w.cloner, destination, index, value]);
        }
        assert_eq!(
            array_values(&f, destination),
            vec![Val::Int(-3), Val::Real(0.25), Val::Text(text)]
        );
    }

    #[test]
    fn arrays_of_simple_values_are_copied_element_by_element() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let copy = |f: &mut Fixture, kind: u32, elements: Vec<Val>, count: i32| -> Vec<Val> {
            let source = f.array(kind, 0, elements.clone());
            let destination = f.array(kind, 0, vec![Val::Int(0); elements.len()]);
            let (d, s) = (f.handle(destination), f.handle(source));
            f.e.call(0x00dc_3d40, &args![w.cloner, d, s, count]);
            array_values(f, destination)
        };
        // Integers (kinds 1 and 2): only `count` elements are copied.
        let got = copy(&mut f, 2, vec![Val::Int(5), Val::Int(-6), Val::Int(7)], 2);
        assert_eq!(got, vec![Val::Int(5), Val::Int(-6), Val::Int(0)]);
        let got = copy(&mut f, 1, vec![Val::Int(1), Val::Int(0)], 2);
        assert_eq!(got, vec![Val::Int(1), Val::Int(0)]);
        // Reals.
        let got = copy(&mut f, 3, vec![Val::Real(2.5), Val::Real(-0.5)], 2);
        assert_eq!(got, vec![Val::Real(2.5), Val::Real(-0.5)]);
        // Vectors of 4 to 16 floats (kinds 4 to 7).
        let a = floats(&mut f.e, &[1.0, 2.0, 3.0, 4.0]);
        let b = floats(&mut f.e, &[5.0, 6.0, 7.0, 8.0]);
        let got = copy(&mut f, 5, vec![Val::Vector(a), Val::Vector(b)], 2);
        assert_eq!(got, vec![Val::Vector(a), Val::Vector(b)]);
        let sets = calls(&f.e, ARRAY_VALUE_SET_VEC);
        assert_eq!(sets.iter().map(|w| w[2]).collect::<Vec<_>>(), vec![8, 8]);
        // Strings.
        let text = f.text("x");
        let got = copy(&mut f, 10, vec![Val::Text(text)], 1);
        assert_eq!(got, vec![Val::Text(text)]);
        // Other kinds do nothing, and nothing is read from an empty copy.
        let before = calls(&f.e, DATA_ARRAY_ELEMENT_OV2).len();
        let got = copy(&mut f, 11, vec![Val::Int(1)], 1);
        assert_eq!(got, vec![Val::Int(0)]);
        let got = copy(&mut f, 2, vec![Val::Int(1)], 0);
        assert_eq!(got, vec![Val::Int(0)]);
        assert_eq!(calls(&f.e, DATA_ARRAY_ELEMENT_OV2).len(), before);
    }

    #[test]
    fn arrays_of_objects_are_copied_as_new_objects_of_the_destination() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let (name, member) = (f.text("Leaf"), f.text("v"));
        let data_v = f.member(member, 2, 0, 0);
        let leaf = w.source_class(&mut f, name, 0, vec![data_v]);
        let one = f.object(leaf, vec![Val::Int(11)]);
        let two = f.object(leaf, vec![Val::Int(22)]);
        let source = f.array(
            8,
            0,
            vec![Val::Object(one), Val::Object(0), Val::Object(two)],
        );
        let destination = f.array(8, 0, vec![]);
        let (d, s) = (f.handle(destination), f.handle(source));
        f.e.call(0x00dc_3d40, &args![w.cloner, d, s, 3i32]);
        // The destination array was sized, null stays null, the others are new objects of the
        // class the destination had to define.
        let values = array_values(&f, destination);
        assert_eq!(values.len(), 3);
        assert_eq!(values[1], Val::Object(0));
        let (Val::Object(first), Val::Object(third)) = (values[0].clone(), values[2].clone())
        else {
            panic!("not objects: {values:?}");
        };
        assert!(first != 0 && third != 0 && first != third && first != one);
        assert_eq!(object_values(&f, first), vec![Val::Int(11)]);
        assert_eq!(object_values(&f, third), vec![Val::Int(22)]);
        assert_eq!(w.defined(), vec!["Leaf".to_string()]);
        let class = f.model.borrow().objects[first as usize - 1].class;
        assert!(w.state.borrow().made.contains(&class));
    }

    #[test]
    fn arrays_of_structs_are_copied_member_by_member_through_swizzled_arrays() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let names: Vec<u32> = ["n", "t", "a", "skipped"]
            .iter()
            .map(|n| f.text(n))
            .collect();
        let members = vec![
            f.member(names[0], 2, 0, 0),
            f.member(names[1], 0x22, 0, 2),
            f.member(names[2], 0x12, 0, 0),
            f.member(names[3], 0, 0, 0),
        ];
        let class_name = f.text("Item");
        let class = f.class(class_name, members);
        let ints = |values: &[i32]| -> Vec<Val> { values.iter().map(|v| Val::Int(*v)).collect() };
        let make = |f: &mut Fixture, n: i32, tuple: [i32; 2], list: &[i32]| -> u32 {
            let tuple = f.array(2, 0, ints(&tuple));
            let list = f.array(2, 0, ints(list));
            f.object(
                class,
                vec![
                    Val::Int(n),
                    Val::Array(tuple),
                    Val::Array(list),
                    Val::Int(9),
                ],
            )
        };
        let s0 = make(&mut f, 1, [10, 11], &[100]);
        let s1 = make(&mut f, 2, [20, 21], &[200, 201]);
        // The destination items have arrays of the sizes the copy follows.
        let d0 = make(&mut f, 0, [0, 0], &[0]);
        let d1 = make(&mut f, 0, [0, 0], &[0, 0]);
        let source = f.array(9, class, vec![Val::Object(s0), Val::Object(s1)]);
        let destination = f.array(9, class, vec![Val::Object(d0), Val::Object(d1)]);
        let (d, s) = (f.handle(destination), f.handle(source));
        f.e.call(0x00dc_3d40, &args![w.cloner, d, s, 2i32]);
        let sub_values = |f: &Fixture, object: u32, member: usize| -> Vec<Val> {
            match object_values(f, object)[member].clone() {
                Val::Array(id) => array_values(f, id),
                other => panic!("not an array: {other:?}"),
            }
        };
        assert_eq!(object_values(&f, d0)[0], Val::Int(1));
        assert_eq!(object_values(&f, d1)[0], Val::Int(2));
        assert_eq!(sub_values(&f, d0, 1), ints(&[10, 11]));
        assert_eq!(sub_values(&f, d1, 1), ints(&[20, 21]));
        assert_eq!(sub_values(&f, d0, 2), ints(&[100]));
        assert_eq!(sub_values(&f, d1, 2), ints(&[200, 201]));
        // The member without a type was not touched, and only the three typed members were
        // swizzled out of both arrays.
        assert_eq!(object_values(&f, d0)[3], Val::Int(9));
        assert_eq!(calls(&f.e, DATA_ARRAY_SWIZZLE_OBJECT_MEMBER).len(), 6);
        // The destination array was sized to the count first.
        assert_eq!(calls(&f.e, DATA_ARRAY_SET_SIZE)[0][1], 2);
    }

    // ---- copyValue, class lookup and object copies ----

    /// A `MemberInfo` record: name, type word, class and tuple size.
    fn member_info(f: &mut Fixture, name: u32, type_word: u32, class: u32, tuple: u32) -> u32 {
        let record = f.e.mem.alloc(0x14);
        f.e.mem.set_u32(record, name);
        f.e.mem.set_u32(record + 8, type_word);
        f.e.mem.set_u32(record + 0xc, class);
        f.e.mem.set_u32(record + 0x10, tuple);
        record
    }

    #[test]
    fn a_plain_member_is_copied_as_a_simple_value() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let (class_name, member) = (f.text("Plain"), f.text("count"));
        let data = f.member(member, 2, 0, 0);
        let class = f.class(class_name, vec![data]);
        let source = f.object(class, vec![Val::Int(12)]);
        let destination = f.object(class, vec![Val::Int(0)]);
        let (d, s) = (f.handle(destination), f.handle(source));
        let info = member_info(&mut f, member, 2, 0, 0);
        f.e.call(0x00dc_4210, &args![w.cloner, d, s, info]);
        assert_eq!(object_values(&f, destination), vec![Val::Int(12)]);
    }

    #[test]
    fn array_and_tuple_members_make_an_array_in_the_destination_world() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let (class_name, list, triple) = (f.text("Holder"), f.text("list"), f.text("triple"));
        let members = vec![f.member(list, 0x12, 0, 0), f.member(triple, 0x22, 0, 3)];
        let source_class = f.class(class_name, members);
        let members = vec![f.member(list, 0x12, 0, 0), f.member(triple, 0x22, 0, 3)];
        let destination_class = w.destination_class(&mut f, class_name, members);
        let list_values = f.array(2, 0, vec![Val::Int(4), Val::Int(5)]);
        let triple_values = f.array(
            2,
            0,
            vec![Val::Int(7), Val::Int(8), Val::Int(9), Val::Int(10)],
        );
        let source = f.object(
            source_class,
            vec![Val::Array(list_values), Val::Array(triple_values)],
        );
        let destination = f.object(destination_class, vec![Val::Int(0), Val::Int(0)]);
        let (d, s) = (f.handle(destination), f.handle(source));
        // An array member: the destination's array has the source array's size.
        let info = member_info(&mut f, list, 0x12, 0, 0);
        f.e.call(0x00dc_4210, &args![w.cloner, d, s, info]);
        let Val::Array(made) = object_values(&f, destination)[0].clone() else {
            panic!("no array was made");
        };
        assert_eq!(array_values(&f, made), vec![Val::Int(4), Val::Int(5)]);
        assert_eq!(f.model.borrow().arrays[made as usize - 1].kind, 2);
        // A tuple member: the tuple size, not the source array's size.
        let info = member_info(&mut f, triple, 0x22, 0, 3);
        f.e.call(0x00dc_4210, &args![w.cloner, d, s, info]);
        let Val::Array(made) = object_values(&f, destination)[1].clone() else {
            panic!("no array was made");
        };
        assert_eq!(
            array_values(&f, made),
            vec![Val::Int(7), Val::Int(8), Val::Int(9)]
        );
        // Both were asked of the destination world through its class.
        assert_eq!(calls(&f.e, DATA_CLASS_GET_WORLD).len(), 2);
    }

    #[test]
    fn a_missing_class_is_defined_parent_first_with_the_classes_its_members_use() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let names: Vec<u32> = ["Base", "Derived", "Other", "x", "y", "o"]
            .iter()
            .map(|n| f.text(n))
            .collect();
        let other = w.source_class(&mut f, names[2], 0, vec![]);
        let base_member = f.member(names[3], 2, 0, 0);
        let base = w.source_class(&mut f, names[0], 0, vec![base_member]);
        let derived_members = vec![f.member(names[4], 2, 0, 0), f.member(names[5], 8, other, 0)];
        let derived = w.source_class(&mut f, names[1], base, derived_members);
        let found = f.e.call(0x00dc_5440, &args![w.cloner, names[1]]).u32();
        // The base was defined first, then the class, then the class of its member.
        assert_eq!(w.defined(), vec!["Base", "Derived", "Other"]);
        assert_ne!(found, 0);
        assert!(w.state.borrow().made.contains(&found));
        let model = f.model.borrow();
        assert_eq!(model.classes[found as usize - 1].name, names[1]);
        assert_ne!(model.classes[found as usize - 1].parent, 0);
        let _ = derived;
        drop(model);
        // Asking again finds the class: nothing more is defined.
        let again = f.e.call(0x00dc_5440, &args![w.cloner, names[1]]).u32();
        assert_eq!(again, found);
        assert_eq!(w.defined().len(), 3);
        // A class that is there already is answered at once.
        let present = w.destination_class(&mut f, names[2], vec![]);
        let before = calls(&f.e, SOURCE_FIND_SLOT).len();
        assert_eq!(
            f.e.call(0x00dc_5440, &args![w.cloner, names[2]]).u32(),
            again_other(&f, &w, names[2], present)
        );
        assert_eq!(calls(&f.e, SOURCE_FIND_SLOT).len(), before);
    }

    /// The destination's class named `name`: the first one found by the world.
    fn again_other(f: &Fixture, w: &Worlds, name: u32, _present: u32) -> u32 {
        let made = w.state.borrow().made.clone();
        find_named(&f.e, &f.model.borrow(), &made, true, name)
    }

    #[test]
    fn member_records_get_the_destination_classes_the_source_members_have() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let names: Vec<u32> = ["Src", "Dst", "A", "B", "p", "q", "r"]
            .iter()
            .map(|n| f.text(n))
            .collect();
        let class_a = w.source_class(&mut f, names[2], 0, vec![]);
        let class_b = w.source_class(&mut f, names[3], 0, vec![]);
        let source_members = vec![
            f.member(names[4], 2, 0, 0),
            f.member(names[5], 8, class_a, 0),
            f.member(names[6], 8, class_b, 0),
        ];
        let source = w.source_class(&mut f, names[0], 0, source_members);
        // The destination's class lacks the classes of `q`, has one for `r` already.
        let preset = w.destination_class(&mut f, names[3], vec![]);
        let destination_members = vec![
            f.member(names[4], 2, 0, 0),
            f.member(names[5], 8, 0, 0),
            f.member(names[6], 8, preset, 0),
        ];
        let destination = w.destination_class(&mut f, names[1], destination_members);
        let (source_handle, destination_handle) = (f.handle(source), f.handle(destination));
        let out = f.e.mem.alloc(12);
        f.e.mem.set_u32(out + 8, NOT_OWNED);
        f.e.call(
            0x00dc_56f0,
            &args![w.cloner, out, destination_handle, source_handle],
        );
        assert_eq!(f.e.mem.u32(out + 4), 3);
        let data = f.e.mem.u32(out);
        let classes: Vec<u32> = (0..3).map(|n| f.e.mem.u32(data + 0x14 * n + 0xc)).collect();
        assert_eq!(classes[0], 0);
        assert_eq!(classes[2], preset);
        // `q` got the class defined for it; `r` kept its own and caused no definition.
        assert_eq!(w.defined(), vec!["A"]);
        let made = w.state.borrow().made.clone();
        assert_eq!(classes[1], *made.last().unwrap());
        // The records are the destination's members.
        assert_eq!(f.e.mem.u32(data + 0x14), names[5]);
    }

    /// The source class `Shape` (an integer `size` and a `Part` pointer `part`) and its object.
    struct Shapes {
        class: u32,
        size: u32,
        part: u32,
        object: u32,
        inner: u32,
    }

    fn shapes(f: &mut Fixture, w: &Worlds) -> Shapes {
        let names: Vec<u32> = ["Shape", "Part", "size", "part", "weight"]
            .iter()
            .map(|n| f.text(n))
            .collect();
        let weight = f.member(names[4], 2, 0, 0);
        let part = w.source_class(f, names[1], 0, vec![weight]);
        let members = vec![f.member(names[2], 2, 0, 0), f.member(names[3], 8, part, 0)];
        let class = w.source_class(f, names[0], 0, members);
        let inner = f.object(part, vec![Val::Int(99)]);
        let object = f.object(class, vec![Val::Int(5), Val::Object(inner)]);
        Shapes {
            class,
            size: names[2],
            part: names[3],
            object,
            inner,
        }
    }

    #[test]
    fn an_object_has_every_member_copied_into_a_destination_object() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let shape = shapes(&mut f, &w);
        // The destination object: a new `Shape` the destination defines for the copy.
        let source_handle = f.handle(shape.object);
        let class =
            f.e.call(
                0x00dc_5440,
                &args![
                    w.cloner,
                    f.model.borrow().classes[shape.class as usize - 1].name
                ],
            )
            .u32();
        let destination = f.object(class, vec![Val::Int(0), Val::Int(0)]);
        let destination_handle = f.handle(destination);
        f.e.call(
            0x00dc_58a0,
            &args![w.cloner, destination_handle, source_handle],
        );
        let values = object_values(&f, destination);
        assert_eq!(values[0], Val::Int(5));
        // The part was copied as a new object of a class the destination got for it.
        let Val::Object(part) = values[1].clone() else {
            panic!("no object: {values:?}");
        };
        assert!(part != 0 && part != shape.inner);
        assert_eq!(object_values(&f, part), vec![Val::Int(99)]);
        assert_eq!(w.defined(), vec!["Shape", "Part"]);
        let _ = (shape.size, shape.part);
    }

    #[test]
    fn copying_an_object_makes_one_destination_object_for_each_source_object() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let shape = shapes(&mut f, &w);
        let source = f.handle(shape.object);
        let out = f.e.mem.alloc(8);
        let result = f.e.call(0x00dc_5970, &args![w.cloner, out, source]).u32();
        assert_eq!(result, out);
        let copy = f.e.mem.u32(out);
        assert!(copy != 0 && copy != shape.object);
        let values = object_values(&f, copy);
        assert_eq!(values[0], Val::Int(5));
        assert!(matches!(values[1], Val::Object(part) if part != 0 && part != shape.inner));
        assert_eq!(calls(&f.e, DESTINATION_OBJECT_SLOT).len(), 2);
        // The copy's class is the destination's class of that name.
        let class = f.model.borrow().objects[copy as usize - 1].class;
        assert!(w.state.borrow().made.contains(&class));
        // The same source object again: the stored copy is fetched from the destination world.
        let out_again = f.e.mem.alloc(8);
        f.e.call(0x00dc_5970, &args![w.cloner, out_again, source]);
        assert_eq!(f.e.mem.u32(out_again), copy);
        assert_eq!(calls(&f.e, DESTINATION_OBJECT_SLOT).len(), 2);
        assert_eq!(calls(&f.e, DESTINATION_GET_OBJECT_SLOT).len(), 1);
        // The map holds the two source objects.
        assert_eq!(f.e.mem.u32(w.cloner + 8 + 4), 2);
    }

    #[test]
    fn copying_a_null_object_makes_a_null_object() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let null = f.handle(0);
        let out = f.e.mem.alloc(8);
        f.e.mem.set_u32(out, 0x1234);
        assert_eq!(
            f.e.call(0x00dc_5970, &args![w.cloner, out, null]).u32(),
            out
        );
        assert_eq!(f.e.mem.u32(out), 0);
        assert_eq!(calls(&f.e, DESTINATION_OBJECT_SLOT).len(), 0);
    }

    #[test]
    fn an_object_that_points_to_itself_is_copied_once() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let (class_name, member) = (f.text("Loop"), f.text("next"));
        let data = f.member(member, 8, 0, 0);
        let class = w.source_class(&mut f, class_name, 0, vec![data]);
        let object = f.object(class, vec![Val::Object(0)]);
        f.model.borrow_mut().objects[object as usize - 1].values[0] = Val::Object(object);
        let source = f.handle(object);
        let out = f.e.mem.alloc(8);
        f.e.call(0x00dc_5970, &args![w.cloner, out, source]);
        let copy = f.e.mem.u32(out);
        assert!(copy != 0 && copy != object);
        assert_eq!(object_values(&f, copy), vec![Val::Object(copy)]);
        assert_eq!(calls(&f.e, DESTINATION_OBJECT_SLOT).len(), 1);
    }

    #[test]
    fn deep_copying_a_world_copies_its_contents_and_empties_the_object_map() {
        let mut f = fixture(0);
        let w = install_clone(&mut f);
        let shape = shapes(&mut f, &w);
        w.state.borrow_mut().contents = shape.object;
        let result = f.e.call(0x00dc_3550, &args![w.destination, w.source]).u32();
        assert_eq!(result, 0);
        assert_eq!(calls(&f.e, SOURCE_CONTENTS_SLOT).len(), 1);
        assert_eq!(calls(&f.e, DESTINATION_OBJECT_SLOT).len(), 2);
        assert_eq!(w.defined(), vec!["Shape", "Part"]);
        // The cloner built on the stack has its object map emptied last.
        let cleared = calls(&f.e, OBJECT_MAP_CLEAR_AND_DEALLOCATE);
        assert_eq!(cleared.len(), 1);
        assert_ne!(cleared[0][0], w.cloner + 8);
        // A world without contents gives a null copy, and the map is emptied again.
        w.state.borrow_mut().contents = 0;
        assert_eq!(
            f.e.call(0x00dc_3550, &args![w.destination, w.source]).u32(),
            0
        );
        assert_eq!(calls(&f.e, DESTINATION_OBJECT_SLOT).len(), 2);
        assert_eq!(calls(&f.e, OBJECT_MAP_CLEAR_AND_DEALLOCATE).len(), 2);
    }
}
