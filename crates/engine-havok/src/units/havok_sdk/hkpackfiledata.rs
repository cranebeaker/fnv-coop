//! `Havok SDK/hkpackfiledata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkPackfileData` fields as the translated functions use them: the contents pointer at `+8`,
//! its type name at `+0xc`, an open-addressing map at `+0x10` (entries of two words, an empty
//! key is `-1`; data `+0x10`, count `+0x14`, mask `+0x18`) keyed by object pointers, a string
//! hash map at `+0x1c` (count word at `+0x20`), a referenced object at `+0x2c`, a flag word at
//! `+0x30` and four arrays: pointers (`+0x34`, 4-byte elements), chunks (`+0x40`, 12-byte
//! elements), exports (`+0x4c`, 8-byte elements) and imports (`+0x58`, 8-byte elements), each
//! as data, size and a capacity word whose low 30 bits are the capacity (bit 31 set: the data is
//! not owned).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::easyFree` (`00c856c0`, cdecl `(allocator, pointer)`).
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkString::strDup` (`00cb1410`, cdecl `(text)`, returns the copy).
const STRING_DUPLICATE: u32 = 0x00cb_1410;
/// `hkMapBase::getWithDefault` (`00cb1930`, `this` = the map, `(key, default)`, returns the value).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// `hkCachedHashMap::getWithDefault` (`00d87800`, `this` = the string map, `(key, default)`).
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
/// The map clear (`00cb1a60`, `this` = the map) and the string map clear (`00d87760`).
const MAP_CLEAR: u32 = 0x00cb_1a60;
const STRING_MAP_CLEAR: u32 = 0x00d8_7760;
/// `hkCachedHashMap::insert` (`00d874d0`, `this` = the string map, `(key, value)`).
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
/// `hkMapBase::reserve` (`00cb1fe0`, `this` = the map, `(allocator, capacity)`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// The string map constructor (`00d87790`, `this` = the string map, one dword that it ignores)
/// and destructor (`00d877d0`).
const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
/// `hkMapBase::clearAndDeallocate` (`00cb1a90`, `this` = the map, `(allocator)`).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// `hkReferencedObject` add and remove reference (`00c90510`, `00c905b0`, `this` = the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The empty function (`00a29680`) the destructor calls on the map at `+0x10`.
const NO_EFFECT: u32 = 0x00a2_9680;
/// The class-name check (`00d46d80`, `this` = a class, `(out byte, other class)`; the callee
/// cleans the stack and returns the out pointer): it stores 1 in the out byte when a class
/// on the parent chain of the other class has the same name as `this`, 0 otherwise.
const CLASS_CHAIN_CONTAINS: u32 = 0x00d4_6d80;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// The exe global holding the object the constructor asks (slot `0x10`) for the referenced
/// object at `+0x2c` (identity unconfirmed).
const SOURCE_OBJECT_GLOBAL: u32 = 0x0126_8b08;
/// The vtable the constructor installs, and the one the destructor ends with.
const PACKFILE_DATA_VTABLE: u32 = 0x010e_a51c;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The names handed to the collector (pointers to exe strings): "memory", "chunks",
/// "exports" and "imports".
const NAME_MEMORY: u32 = 0x010e_a510;
const NAME_CHUNKS: u32 = 0x010e_29d4;
const NAME_EXPORTS: u32 = 0x010e_a508;
const NAME_IMPORTS: u32 = 0x010e_a500;
/// The "not owned" bit of a capacity word, and the capacity bits.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The slot-`0x10` free of the allocator for one array of this packfile data: when the data is
/// owned (bit 31 clear) it is returned as `data, capacity * element size`.
fn free_owned(e: &mut Engine, allocator: u32, data: u32, capacity_word: u32, element: u32) {
    if capacity_word & NOT_OWNED == 0 {
        let bytes = (capacity_word & CAPACITY_MASK).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00e05110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::setName` (named in the engine map): frees the old name at `+0xc`
/// (`easyFree` with the router's allocator) and stores a copy of `name` (`strDup`).
pub fn hkp_packfile_data_set_name(e: &mut Engine, this: Ptr, name: u32) {
    let old = e.mem.u32(this.addr() + 0xc);
    let allocator = router_allocator(e);
    e.call(EASY_FREE, &args![allocator, old]);
    let copy = e.call(STRING_DUPLICATE, &args![name]).u32();
    e.mem.set_u32(this.addr() + 0xc, copy);
}

// Translated from 00e05140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::getContentsTypeName` (named in the engine map, `__fastcall`): 0 without
/// contents (`+8`), otherwise the value the map at `+0x10` holds for the contents pointer (0
/// when absent).
pub fn hkp_packfile_data_get_contents_type_name(e: &mut Engine, this: Ptr) -> u32 {
    let contents = e.mem.u32(this.addr() + 8);
    if contents == 0 {
        return 0;
    }
    e.call(
        MAP_GET_WITH_DEFAULT,
        &args![this.addr() + 0x10, contents, 0u32],
    )
    .u32()
}

// Translated from 00e05240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__fastcall`): whether the count word at `+0x20` (the string map's) is non-zero.
pub fn fn_00e05240(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x20) != 0
}

/// Adds one array to the collector (slot `0x14`: `3, name, data, size * element, allocated`)
/// when it is owned (capacity word bit 31 clear) or the collector's flag byte (`+0x10`) is
/// set; `allocated` is `getAllocatedSize(data, capacity * element)`. The three fields are
/// `(data, size, capacity word)` offsets.
fn add_array(
    e: &mut Engine,
    collector: u32,
    this: u32,
    fields: (u32, u32, u32),
    element: u32,
    name: u32,
) {
    let (data_field, size_field, capacity_field) = fields;
    let capacity_word = e.mem.u32(this + capacity_field);
    if capacity_word & NOT_OWNED != 0 && e.mem.u8(collector + 0x10) == 0 {
        return;
    }
    let data = e.mem.u32(this + data_field);
    let bytes = (capacity_word & CAPACITY_MASK).wrapping_mul(element);
    let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
    let data = e.mem.u32(this + data_field);
    let used = e.mem.u32(this + size_field).wrapping_mul(element);
    e.vcall(collector, 0x14, &args![3u32, name, data, used, allocated]);
}

// Translated from 00e05250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::calcContentStatistics` (named in the engine map): adds the four arrays to
/// `collector` (see [`add_array`]): "memory" (`+0x34`, 4-byte elements), "chunks" (`+0x40`, 12),
/// "exports" (`+0x4c`, 8) and "imports" (`+0x58`, 8).
/// The second stack word (`RET 8`; callers pass the class record) is not read.
pub fn hkp_packfile_data_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused_2: u32,
) {
    let base = this.addr();
    add_array(e, collector, base, (0x34, 0x38, 0x3c), 4, NAME_MEMORY);
    add_array(e, collector, base, (0x40, 0x44, 0x48), 12, NAME_CHUNKS);
    add_array(e, collector, base, (0x4c, 0x50, 0x54), 8, NAME_EXPORTS);
    add_array(e, collector, base, (0x58, 0x5c, 0x60), 8, NAME_IMPORTS);
}

// Translated from 00e05370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::callDestructors` (named in the engine map, `__fastcall`): when the flag
/// word at `+0x30` is non-zero and the string map's count (`+0x20`) is positive, every used
/// entry of the map at `+0x10` (a key other than `-1`, in index order up to the mask at `+0x18`)
/// looks its value up in the string map (`00d87800`, default 0) and, when that finds a record
/// whose word at `+8` is a function, calls it (cdecl) with the entry's key. Then, always, the
/// contents pointer (`+8`) is cleared, the map at `+0x10` is cleared (`00cb1a60`) and the string
/// map is cleared (`00d87760`).
pub fn hkp_packfile_data_call_destructors(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x30) != 0 && e.mem.i32(base + 0x20) > 0 {
        let mask = e.mem.i32(base + 0x18);
        let mut index = 0i32;
        if mask >= 0 {
            let mut entry = e.mem.u32(base + 0x10);
            loop {
                if e.mem.u32(entry) != 0xffff_ffff {
                    break;
                }
                index += 1;
                entry = entry.wrapping_add(8);
                if index > mask {
                    break;
                }
            }
        }
        if index <= mask {
            loop {
                let entries = e.mem.u32(base + 0x10);
                let value = e
                    .mem
                    .u32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
                let record = e
                    .call(
                        STRING_MAP_GET_WITH_DEFAULT,
                        &args![base + 0x1c, value, 0u32],
                    )
                    .u32();
                if record != 0 {
                    let function = e.mem.u32(record + 8);
                    if function != 0 {
                        let entries = e.mem.u32(base + 0x10);
                        let key = e
                            .mem
                            .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
                        e.call(function, &args![key]);
                    }
                }
                let mask = e.mem.i32(base + 0x18);
                let mut next = index + 1;
                if next <= mask {
                    let mut entry = e
                        .mem
                        .u32(base + 0x10)
                        .wrapping_add((next as u32).wrapping_mul(8));
                    loop {
                        if e.mem.u32(entry) != 0xffff_ffff {
                            break;
                        }
                        next += 1;
                        entry = entry.wrapping_add(8);
                        if next > mask {
                            break;
                        }
                    }
                }
                index = next;
                if index > mask {
                    break;
                }
            }
        }
    }
    e.mem.set_u32(base + 8, 0);
    e.call(MAP_CLEAR, &args![base + 0x10]);
    e.call(STRING_MAP_CLEAR, &args![base + 0x1c]);
}

/// One of the two array copies of `getImportsExports`: copies the `count` 8-byte elements at
/// `source` into `array` (an `hkArray` of 8-byte elements: data, size, capacity word). The
/// router is asked first; when the capacity is below `count` the array is grown to
/// `max(count, 2 * capacity)` (`_reserve`, element size 8). The first `min(size, count)`
/// elements are copied over the existing ones; the others are copied only where the
/// destination address is not zero. The size becomes `count`.
fn copy_into_array(e: &mut Engine, array: u32, source_data_field: u32, this: u32, count: i32) {
    let allocator = router_allocator(e);
    let size = e.mem.i32(array + 4);
    let kept = if count <= size { count } else { size };
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < count {
        let doubled = capacity * 2;
        let grown = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, grown as u32, 8u32]);
    }
    let source = e.mem.u32(this + source_data_field);
    let mut destination = e.mem.u32(array);
    let mut from = source;
    for _ in 0..kept.max(0) {
        copy_pair(e, from, destination);
        from = from.wrapping_add(8);
        destination = destination.wrapping_add(8);
    }
    let offset = (kept as u32).wrapping_mul(8);
    let mut from = e.mem.u32(this + source_data_field).wrapping_add(offset);
    let mut destination = e.mem.u32(array).wrapping_add(offset);
    for _ in 0..(count - kept).max(0) {
        if destination != 0 {
            copy_pair(e, from, destination);
        }
        from = from.wrapping_add(8);
        destination = destination.wrapping_add(8);
    }
    e.mem.set_u32(array + 4, count as u32);
}

/// Copies two words.
fn copy_pair(e: &mut Engine, from: u32, to: u32) {
    let first = e.mem.u32(from);
    e.mem.set_u32(to, first);
    let second = e.mem.u32(from + 4);
    e.mem.set_u32(to + 4, second);
}

// Translated from 00e05410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::getImportsExports` (named in the engine map): copies the imports
/// (`+0x58`, count `+0x5c`) into the array `imports` and then the exports (`+0x4c`, count
/// `+0x50`) into the array `exports`, each as described for [`copy_into_array`].
pub fn hkp_packfile_data_get_imports_exports(
    e: &mut Engine,
    this: Ptr,
    imports: Ptr,
    exports: Ptr,
) {
    let base = this.addr();
    let count = e.mem.i32(base + 0x5c);
    copy_into_array(e, imports.addr(), 0x58, base, count);
    let count = e.mem.i32(base + 0x50);
    copy_into_array(e, exports.addr(), 0x4c, base, count);
}

// Translated from 00e055f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::getContentsPointer` (named in the engine map): returns the contents
/// pointer (`+8`), or 0 when it is missing or a check fails. Without contents the result is 0.
/// When `requested` is non-zero, the type name of the contents ([`hkp_packfile_data_get_contents_type_name`]'s
/// value, from the map at `+0x10`) and `requested` are both turned into classes by slot `0x10`
/// of the object at `+0x2c`, and the first must have the class name of one of the second's
/// parents (`00d46d80`); otherwise the result is 0. Then, when the string map's count (`+0x20`)
/// is zero and `source` is non-zero, every used entry of the map at `+0x10` (see
/// [`fn_00e05950`] for the first index) is handed to slot `0x10` of `source` (`key`, `value`):
/// a non-zero result is inserted into the string map (`value`, result), a zero result makes the
/// whole call return 0 after the walk.
pub fn hkp_packfile_data_get_contents_pointer(
    e: &mut Engine,
    this: Ptr,
    requested: u32,
    source: u32,
) -> u32 {
    let base = this.addr();
    let contents = e.mem.u32(base + 8);
    if contents == 0 {
        return 0;
    }
    let type_name = e
        .call(MAP_GET_WITH_DEFAULT, &args![base + 0x10, contents, 0u32])
        .u32();
    if requested != 0 {
        let lookup = e.mem.u32(base + 0x2c);
        let actual = e.vcall(lookup, 0x10, &args![type_name]).u32();
        let wanted = e.vcall(lookup, 0x10, &args![requested]).u32();
        let found = e.with_stack(4, |e, out| {
            let result = e
                .call(CLASS_CHAIN_CONTAINS, &args![wanted, out, actual])
                .u32();
            e.mem.u8(result)
        });
        if found == 0 {
            return 0;
        }
    }
    if !fn_00e05240(e, this) && source != 0 {
        let mut failed = false;
        let mut index = fn_00e05950(e, Ptr::new(base + 0x10));
        if index <= e.mem.i32(base + 0x18) {
            loop {
                let entries = e.mem.u32(base + 0x10);
                let at = entries.wrapping_add((index as u32).wrapping_mul(8));
                let value = e.mem.u32(at + 4);
                let key = e.mem.u32(at);
                let created = e.vcall(source, 0x10, &args![key, value]).u32();
                if created != 0 {
                    e.call(STRING_MAP_INSERT, &args![base + 0x1c, value, created]);
                } else {
                    failed = true;
                }
                let mask = e.mem.i32(base + 0x18);
                let mut next = index + 1;
                if next <= mask {
                    let mut entry = e
                        .mem
                        .u32(base + 0x10)
                        .wrapping_add((next as u32).wrapping_mul(8));
                    loop {
                        if e.mem.u32(entry) != 0xffff_ffff {
                            break;
                        }
                        next += 1;
                        entry = entry.wrapping_add(8);
                        if next > mask {
                            break;
                        }
                    }
                }
                index = next;
                if index > e.mem.i32(base + 0x18) {
                    break;
                }
            }
            if failed {
                return 0;
            }
        }
    }
    e.mem.u32(base + 8)
}

// Translated from 00e05700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::hkPackfileData` (named in the engine map, `__fastcall`): sets the 16-bit
/// word at `+6` to 1, the vtable (`010ea51c`), the contents pointer and name (`+8`, `+0xc`) to
/// zero, the map at `+0x10` to data 0, size 0, mask `-1` and reserves it (`00cb1fe0`, the
/// router's allocator, capacity 0); constructs the string map at `+0x1c` (`00d87790`, handed a
/// dword of which only the low byte is set; the callee ignores it), zeroes `+0x2c`, sets the
/// flag word `+0x30` to 1 and empties the four arrays (data 0, size 0, capacity word
/// `0x80000000`). Then slot `0x10` of the object at the exe global `01268b08` gives the
/// referenced object: it gets a reference (`00c90510`) when non-zero, the previous object at
/// `+0x2c` (zero here) is released when non-zero (`00c905b0`) and the new one is stored.
/// Returns `this`.
pub fn hkp_packfile_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, PACKFILE_DATA_VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(MAP_RESERVE, &args![base + 0x10, allocator, 0u32]);
    e.call(STRING_MAP_CONSTRUCT, &args![base + 0x1c, 0u32]);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x30, 1);
    for (data, size, capacity) in [
        (0x34, 0x38, 0x3c),
        (0x40, 0x44, 0x48),
        (0x4c, 0x50, 0x54),
        (0x58, 0x5c, 0x60),
    ] {
        e.mem.set_u32(base + data, 0);
        e.mem.set_u32(base + size, 0);
        e.mem.set_u32(base + capacity, NOT_OWNED);
    }
    let source = e.mem.u32(SOURCE_OBJECT_GLOBAL);
    let object = e.vcall(source, 0x10, &args![]).u32();
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    let previous = e.mem.u32(base + 0x2c);
    if previous != 0 {
        e.call(REMOVE_REFERENCE, &args![previous]);
    }
    e.mem.set_u32(base + 0x2c, object);
    this
}

// Translated from 00e057c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileData::~hkPackfileData` (named in the engine map, `__fastcall`): sets the vtable
/// (`010ea51c`) and runs [`hkp_packfile_data_call_destructors`]; frees every pointer of the
/// array at `+0x34` (`easyFree`, router allocator) and every chunk of the array at `+0x40`
/// (allocator slot 8, `(pointer, size)` from the 12-byte elements' first two words); frees the
/// name (`+0xc`, `easyFree`); empties the imports (`+0x58`), exports (`+0x4c`), chunks
/// (`+0x40`) and pointer (`+0x34`) arrays, in that order, freeing owned data through allocator
/// slot `0x10` (`data, capacity * element size`) and resetting each to data 0, size 0,
/// capacity word `0x80000000`; releases the referenced object (`+0x2c`) when set; destructs the
/// string map (`00d877d0`), clears and deallocates the map at `+0x10` (`00cb1a90`, router
/// allocator), calls the empty function (`00a29680`) on it and ends with the vtable `0102e388`.
pub fn hkp_packfile_data_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PACKFILE_DATA_VTABLE);
    hkp_packfile_data_call_destructors(e, this);
    let mut index = 0u32;
    if e.mem.i32(base + 0x38) > 0 {
        loop {
            let array = e.mem.u32(base + 0x34);
            let pointer = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
            let allocator = router_allocator(e);
            e.call(EASY_FREE, &args![allocator, pointer]);
            index += 1;
            if index as i32 >= e.mem.i32(base + 0x38) {
                break;
            }
        }
    }
    let mut index = 0u32;
    let mut offset = 0u32;
    if e.mem.i32(base + 0x44) > 0 {
        loop {
            let chunks = e.mem.u32(base + 0x40);
            let size = e.mem.u32(chunks.wrapping_add(offset) + 4);
            let pointer = e.mem.u32(chunks.wrapping_add(offset));
            let allocator = router_allocator(e);
            e.vcall(allocator, 8, &args![pointer, size]);
            index += 1;
            offset = offset.wrapping_add(0xc);
            if index as i32 >= e.mem.i32(base + 0x44) {
                break;
            }
        }
    }
    let name = e.mem.u32(base + 0xc);
    let allocator = router_allocator(e);
    e.call(EASY_FREE, &args![allocator, name]);
    // Imports, exports, chunks and pointers: data, size and capacity word fields and the
    // element size.
    for (data, size, capacity, element) in [
        (0x58, 0x5c, 0x60, 8),
        (0x4c, 0x50, 0x54, 8),
        (0x40, 0x44, 0x48, 12),
        (0x34, 0x38, 0x3c, 4),
    ] {
        let allocator = router_allocator(e);
        let capacity_word = e.mem.u32(base + capacity);
        e.mem.set_u32(base + size, 0);
        let array = e.mem.u32(base + data);
        free_owned(e, allocator, array, capacity_word, element);
        e.mem.set_u32(base + data, 0);
        e.mem.set_u32(base + capacity, NOT_OWNED);
    }
    let referenced = e.mem.u32(base + 0x2c);
    if referenced != 0 {
        e.call(REMOVE_REFERENCE, &args![referenced]);
    }
    e.mem.set_u32(base + 0x2c, 0);
    e.call(STRING_MAP_DESTRUCT, &args![base + 0x1c]);
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![base + 0x10, allocator]);
    e.call(NO_EFFECT, &args![base + 0x10]);
    e.mem.set_u32(base, BASE_VTABLE);
}

// Translated from 00e05950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__fastcall`, `this` = a map as at `+0x10`: data, size, mask): the index of the
/// first used entry (key other than `-1`), scanning while the index is at most the mask (a
/// negative mask scans nothing); mask + 1 when there is none.
pub fn fn_00e05950(e: &mut Engine, this: Ptr) -> i32 {
    let mask = e.mem.i32(this.addr() + 8);
    let mut index = 0;
    if mask >= 0 {
        let mut entry = e.mem.u32(this.addr());
        loop {
            if e.mem.u32(entry) != 0xffff_ffff {
                break;
            }
            index += 1;
            entry = entry.wrapping_add(8);
            if index > mask {
                break;
            }
        }
    }
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_5110, hkp_packfile_data_set_name(Ptr, u32)),
        entry!(
            0x00e0_5140,
            hkp_packfile_data_get_contents_type_name(Ptr) -> u32
        ),
        entry!(0x00e0_5240, fn_00e05240(Ptr) -> bool),
        entry!(
            0x00e0_5250,
            hkp_packfile_data_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(0x00e0_5370, hkp_packfile_data_call_destructors(Ptr)),
        entry!(
            0x00e0_5410,
            hkp_packfile_data_get_imports_exports(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_55f0,
            hkp_packfile_data_get_contents_pointer(Ptr, u32, u32) -> u32
        ),
        entry!(0x00e0_5700, hkp_packfile_data_construct(Ptr) -> Ptr),
        entry!(0x00e0_57c0, hkp_packfile_data_destruct(Ptr)),
        entry!(0x00e0_5950, fn_00e05950(Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0010;
    const FREE_DATA_DOUBLE: u32 = 0x0900_0020;
    const ARRAY_DOUBLE: u32 = 0x0900_0030;
    const FINISH_DOUBLE: u32 = 0x0900_0040;
    const LOOKUP_DOUBLE: u32 = 0x0900_0050;
    const SOURCE_DOUBLE: u32 = 0x0900_0060;
    const DESTRUCTOR_DOUBLE: u32 = 0x0900_0070;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x0126_8000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of the logged calls among `of`, in order.
    fn order(e: &Engine, of: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| of.contains(a))
            .collect()
    }

    /// A router with an allocator whose slots 8 and `0x10` are doubles; returns the allocator.
    fn install_allocator(e: &mut Engine) -> u32 {
        e.register_double(FREE_BLOCK_DOUBLE, |_, _| Ret::default());
        e.register_double(FREE_DATA_DOUBLE, |_, _| Ret::default());
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 8, FREE_BLOCK_DOUBLE);
        e.mem.set_u32(table + 0x10, FREE_DATA_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    /// Writes an object whose vtable has the given double at slot `0x10`.
    fn object_with_slot_10(e: &mut Engine, target: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, target);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn set_name_frees_the_old_name_and_stores_a_copy() {
        let mut e = engine();
        let allocator = install_allocator(&mut e);
        e.register_double(EASY_FREE, |_, _| Ret::default());
        e.register_double(STRING_DUPLICATE, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        let data = e.mem.alloc(0x70);
        e.mem.set_u32(data + 0xc, 0x5555);
        e.call(0x00e0_5110, &args![data, 0x1234u32]);
        assert_eq!(calls(&e, EASY_FREE), vec![vec![allocator, 0x5555]]);
        assert_eq!(calls(&e, STRING_DUPLICATE), vec![vec![0x1234]]);
        assert_eq!(e.mem.u32(data + 0xc), 0x7777);
        assert_eq!(
            order(&e, &[EASY_FREE, STRING_DUPLICATE]),
            vec![EASY_FREE, STRING_DUPLICATE]
        );
    }

    #[test]
    fn get_contents_type_name_looks_the_contents_up_in_the_map() {
        let mut e = engine();
        e.register_double(MAP_GET_WITH_DEFAULT, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let data = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00e0_5140, &args![data]).u32(), 0);
        assert!(calls(&e, MAP_GET_WITH_DEFAULT).is_empty());
        e.mem.set_u32(data + 8, 0x4000);
        assert_eq!(e.call(0x00e0_5140, &args![data]).u32(), 0x4001);
        assert_eq!(
            calls(&e, MAP_GET_WITH_DEFAULT),
            vec![vec![data + 0x10, 0x4000, 0]]
        );
    }

    #[test]
    fn fn_00e05240_tests_the_count_word() {
        let mut e = engine();
        let data = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00e0_5240, &args![data]).u32() & 0xff, 0);
        e.mem.set_u32(data + 0x20, 3);
        assert_eq!(e.call(0x00e0_5240, &args![data]).u32() & 0xff, 1);
    }

    /// A collector whose slot `0x14` is the array double, and a packfile data with the four
    /// arrays: `(data, size, capacity word)` = memory (0x100, 2, 8), chunks (0x200, 3, 5),
    /// exports (0x300, 4, 6), imports (0x400, 5, 7).
    fn statistics_fixture(e: &mut Engine) -> (u32, u32) {
        e.register_double(ARRAY_DOUBLE, |_, _| Ret::default());
        e.register_double(GET_ALLOCATED_SIZE, |_, a| Ret {
            eax: a[1] + 1000,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x14, ARRAY_DOUBLE);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, table);
        let data = e.mem.alloc(0x70);
        for (i, (array, size, capacity)) in [
            (0x100u32, 2u32, 8u32),
            (0x200, 3, 5),
            (0x300, 4, 6),
            (0x400, 5, 7),
        ]
        .into_iter()
        .enumerate()
        {
            let at = data + 0x34 + 0xc * i as u32;
            e.mem.set_u32(at, array);
            e.mem.set_u32(at + 4, size);
            e.mem.set_u32(at + 8, capacity);
        }
        (data, collector)
    }

    #[test]
    fn calc_content_statistics_adds_the_four_owned_arrays() {
        let mut e = engine();
        let (data, collector) = statistics_fixture(&mut e);
        e.call(0x00e0_5250, &args![data, collector, 0u32]);
        assert_eq!(
            calls(&e, GET_ALLOCATED_SIZE),
            vec![
                vec![0x100, 32],
                vec![0x200, 60],
                vec![0x300, 48],
                vec![0x400, 56]
            ]
        );
        assert_eq!(
            calls(&e, ARRAY_DOUBLE),
            vec![
                vec![collector, 3, NAME_MEMORY, 0x100, 8, 1032],
                vec![collector, 3, NAME_CHUNKS, 0x200, 36, 1060],
                vec![collector, 3, NAME_EXPORTS, 0x300, 32, 1048],
                vec![collector, 3, NAME_IMPORTS, 0x400, 40, 1056],
            ]
        );
    }

    #[test]
    fn calc_content_statistics_skips_unowned_arrays_unless_the_collector_flag_is_set() {
        let mut e = engine();
        let (data, collector) = statistics_fixture(&mut e);
        e.mem.set_u32(data + 0x3c, 0x8000_0008);
        e.mem.set_u32(data + 0x60, 0x8000_0007);
        e.call(0x00e0_5250, &args![data, collector, 0u32]);
        let names: Vec<u32> = calls(&e, ARRAY_DOUBLE).iter().map(|w| w[2]).collect();
        assert_eq!(names, vec![NAME_CHUNKS, NAME_EXPORTS]);
        e.call_log = Some(Vec::new());
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00e0_5250, &args![data, collector, 0u32]);
        let names: Vec<u32> = calls(&e, ARRAY_DOUBLE).iter().map(|w| w[2]).collect();
        assert_eq!(
            names,
            vec![NAME_MEMORY, NAME_CHUNKS, NAME_EXPORTS, NAME_IMPORTS]
        );
        // The capacity bits alone size the allocated block.
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE)[0], vec![0x100, 32]);
    }

    /// A packfile data whose map at `+0x10` has eight entries (mask 7) with keys at 2, 3 and 5
    /// (key `0x100 + index`, value `0x200 + index`); the string-map double returns a record
    /// for each value: 0x200 + 2 a record with a destructor, + 3 none, + 5 a record whose
    /// destructor word is zero.
    fn destructor_fixture(e: &mut Engine) -> u32 {
        e.register_double(DESTRUCTOR_DOUBLE, |_, _| Ret::default());
        e.register_double(MAP_CLEAR, |_, _| Ret::default());
        e.register_double(STRING_MAP_CLEAR, |_, _| Ret::default());
        let with = e.mem.alloc(0x10);
        e.mem.set_u32(with + 8, DESTRUCTOR_DOUBLE);
        let without = e.mem.alloc(0x10);
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: match a[1] {
                0x202 => with,
                0x205 => without,
                _ => 0,
            },
            ..Ret::default()
        });
        let entries = e.mem.alloc(0x40);
        for index in 0..8u32 {
            let used = [2, 3, 5].contains(&index);
            e.mem.set_u32(
                entries + 8 * index,
                if used { 0x100 + index } else { 0xffff_ffff },
            );
            e.mem.set_u32(entries + 8 * index + 4, 0x200 + index);
        }
        let data = e.mem.alloc(0x70);
        e.mem.set_u32(data + 8, 0x4000);
        e.mem.set_u32(data + 0x10, entries);
        e.mem.set_u32(data + 0x14, 3);
        e.mem.set_u32(data + 0x18, 7);
        e.mem.set_u32(data + 0x20, 2);
        e.mem.set_u32(data + 0x30, 1);
        data
    }

    #[test]
    fn call_destructors_calls_the_destructor_of_each_used_entry_then_clears() {
        let mut e = engine();
        let data = destructor_fixture(&mut e);
        e.call(0x00e0_5370, &args![data]);
        assert_eq!(
            calls(&e, STRING_MAP_GET_WITH_DEFAULT),
            vec![
                vec![data + 0x1c, 0x202, 0],
                vec![data + 0x1c, 0x203, 0],
                vec![data + 0x1c, 0x205, 0]
            ]
        );
        assert_eq!(calls(&e, DESTRUCTOR_DOUBLE), vec![vec![0x102]]);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(calls(&e, MAP_CLEAR), vec![vec![data + 0x10]]);
        assert_eq!(calls(&e, STRING_MAP_CLEAR), vec![vec![data + 0x1c]]);
    }

    #[test]
    fn call_destructors_only_clears_without_the_flag_or_the_count() {
        for (flag, count) in [(0u32, 2u32), (1, 0)] {
            let mut e = engine();
            let data = destructor_fixture(&mut e);
            e.mem.set_u32(data + 0x30, flag);
            e.mem.set_u32(data + 0x20, count);
            e.call(0x00e0_5370, &args![data]);
            assert!(calls(&e, STRING_MAP_GET_WITH_DEFAULT).is_empty());
            assert_eq!(e.mem.u32(data + 8), 0);
            assert_eq!(calls(&e, MAP_CLEAR).len(), 1);
            assert_eq!(calls(&e, STRING_MAP_CLEAR).len(), 1);
        }
    }

    #[test]
    fn call_destructors_ignores_an_empty_map_with_a_negative_mask() {
        let mut e = engine();
        let data = destructor_fixture(&mut e);
        e.mem.set_u32(data + 0x18, 0xffff_ffff);
        e.call(0x00e0_5370, &args![data]);
        assert!(calls(&e, STRING_MAP_GET_WITH_DEFAULT).is_empty());
        assert_eq!(calls(&e, MAP_CLEAR).len(), 1);
    }

    /// The `_reserve` double: moves the array into a new block of the requested capacity
    /// (keeping its elements) and sets the capacity word.
    fn install_reserve(e: &mut Engine) {
        e.register_double(ARRAY_RESERVE, |e, a| {
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let block = e.mem.alloc(capacity * element + 8);
            let old = e.mem.u32(array);
            let used = e.mem.u32(array + 4) * element;
            for offset in 0..used {
                let byte = e.mem.u8(old + offset);
                e.mem.set_u8(block + offset, byte);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
    }

    /// A packfile data with two imports `(10, 11), (12, 13)` and three exports
    /// `(20, 21), (22, 23), (24, 25)`; returns it with two empty destination arrays.
    fn imports_exports_fixture(e: &mut Engine) -> (u32, u32, u32) {
        let imports = e.mem.alloc(0x10);
        let exports = e.mem.alloc(0x18);
        for (i, word) in [10u32, 11, 12, 13].into_iter().enumerate() {
            e.mem.set_u32(imports + 4 * i as u32, word);
        }
        for (i, word) in [20u32, 21, 22, 23, 24, 25].into_iter().enumerate() {
            e.mem.set_u32(exports + 4 * i as u32, word);
        }
        let data = e.mem.alloc(0x70);
        e.mem.set_u32(data + 0x58, imports);
        e.mem.set_u32(data + 0x5c, 2);
        e.mem.set_u32(data + 0x4c, exports);
        e.mem.set_u32(data + 0x50, 3);
        let import_array = e.mem.alloc(0x10);
        let export_array = e.mem.alloc(0x10);
        (data, import_array, export_array)
    }

    fn words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..2 * e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn get_imports_exports_grows_the_empty_arrays_and_copies_the_elements() {
        let mut e = engine();
        let allocator = install_allocator(&mut e);
        install_reserve(&mut e);
        let (data, import_array, export_array) = imports_exports_fixture(&mut e);
        e.call(0x00e0_5410, &args![data, import_array, export_array]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![
                vec![allocator, import_array, 2, 8],
                vec![allocator, export_array, 3, 8]
            ]
        );
        assert_eq!(words(&e, import_array), vec![10, 11, 12, 13]);
        assert_eq!(words(&e, export_array), vec![20, 21, 22, 23, 24, 25]);
        assert_eq!(e.mem.u32(import_array + 4), 2);
        assert_eq!(e.mem.u32(export_array + 4), 3);
    }

    #[test]
    fn get_imports_exports_overwrites_existing_elements_and_doubles_the_capacity() {
        let mut e = engine();
        let allocator = install_allocator(&mut e);
        install_reserve(&mut e);
        let (data, import_array, export_array) = imports_exports_fixture(&mut e);
        // The export array holds one element and has room for one: it grows to
        // max(3, 2 * 1) = 3; the import array holds five and has room for five: no growth.
        let old_exports = e.mem.alloc(8);
        e.mem.set_u32(old_exports, 99);
        e.mem.set_u32(export_array, old_exports);
        e.mem.set_u32(export_array + 4, 1);
        e.mem.set_u32(export_array + 8, 1);
        let old_imports = e.mem.alloc(0x28);
        e.mem.set_u32(import_array, old_imports);
        e.mem.set_u32(import_array + 4, 5);
        e.mem.set_u32(import_array + 8, 0x8000_0005);
        e.call(0x00e0_5410, &args![data, import_array, export_array]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![allocator, export_array, 3, 8]]
        );
        assert_eq!(e.mem.u32(import_array), old_imports);
        assert_eq!(words(&e, import_array), vec![10, 11, 12, 13]);
        assert_eq!(words(&e, export_array), vec![20, 21, 22, 23, 24, 25]);
    }

    /// A packfile data for `getContentsPointer`: contents `0x4000`, a lookup object (slot
    /// `0x10` doubles the argument plus `0x10`), the class-chain double reporting `found`,
    /// and two map entries `(key 0x100, value 0x200)` and `(key 0x101, value 0x201)` at
    /// indices 1 and 3 of a map with mask 3.
    fn pointer_fixture(e: &mut Engine, found: u8) -> u32 {
        e.register_double(MAP_GET_WITH_DEFAULT, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register_double(LOOKUP_DOUBLE, |_, a| Ret {
            eax: a[1] + 0x10,
            ..Ret::default()
        });
        e.register_double(CLASS_CHAIN_CONTAINS, move |e, a| {
            e.mem.set_u8(a[1], found);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(STRING_MAP_INSERT, |_, _| Ret::default());
        let lookup = object_with_slot_10(e, LOOKUP_DOUBLE);
        let entries = e.mem.alloc(0x20);
        for index in 0..4u32 {
            let used = index == 1 || index == 3;
            let key = if used { 0x100 + index / 2 } else { 0xffff_ffff };
            e.mem.set_u32(entries + 8 * index, key);
            e.mem.set_u32(entries + 8 * index + 4, 0x200 + index / 2);
        }
        let data = e.mem.alloc(0x70);
        e.mem.set_u32(data + 8, 0x4000);
        e.mem.set_u32(data + 0x10, entries);
        e.mem.set_u32(data + 0x14, 2);
        e.mem.set_u32(data + 0x18, 3);
        e.mem.set_u32(data + 0x2c, lookup);
        data
    }

    #[test]
    fn get_contents_pointer_is_zero_without_contents() {
        let mut e = engine();
        let data = pointer_fixture(&mut e, 1);
        e.mem.set_u32(data + 8, 0);
        assert_eq!(e.call(0x00e0_55f0, &args![data, 0u32, 0u32]).u32(), 0);
        assert!(calls(&e, MAP_GET_WITH_DEFAULT).is_empty());
    }

    #[test]
    fn get_contents_pointer_checks_the_requested_class() {
        let mut e = engine();
        let data = pointer_fixture(&mut e, 1);
        let lookup = e.mem.u32(data + 0x2c);
        let result = e.call(0x00e0_55f0, &args![data, 0x300u32, 0u32]).u32();
        assert_eq!(result, 0x4000);
        // The type name 0x6000 and the request 0x300 are both looked up (slot 0x10); the class
        // of the type name is the `other` argument, the requested class `this`.
        assert_eq!(
            calls(&e, LOOKUP_DOUBLE),
            vec![vec![lookup, 0x6000], vec![lookup, 0x300]]
        );
        let checks = calls(&e, CLASS_CHAIN_CONTAINS);
        assert_eq!(checks.len(), 1);
        assert_eq!((checks[0][0], checks[0][2]), (0x310, 0x6010));
    }

    #[test]
    fn get_contents_pointer_is_zero_when_the_class_check_fails() {
        let mut e = engine();
        let data = pointer_fixture(&mut e, 0);
        assert_eq!(
            e.call(0x00e0_55f0, &args![data, 0x300u32, 0x9000u32]).u32(),
            0
        );
        assert!(calls(&e, STRING_MAP_INSERT).is_empty());
    }

    #[test]
    fn get_contents_pointer_finishes_each_entry_through_the_source() {
        let mut e = engine();
        let data = pointer_fixture(&mut e, 1);
        e.register_double(FINISH_DOUBLE, |_, a| Ret {
            eax: a[1] + a[2],
            ..Ret::default()
        });
        let source = object_with_slot_10(&mut e, FINISH_DOUBLE);
        assert_eq!(
            e.call(0x00e0_55f0, &args![data, 0u32, source]).u32(),
            0x4000
        );
        assert_eq!(
            calls(&e, FINISH_DOUBLE),
            vec![vec![source, 0x100, 0x200], vec![source, 0x101, 0x201]]
        );
        assert_eq!(
            calls(&e, STRING_MAP_INSERT),
            vec![
                vec![data + 0x1c, 0x200, 0x300],
                vec![data + 0x1c, 0x201, 0x302]
            ]
        );
    }

    #[test]
    fn get_contents_pointer_fails_after_the_walk_when_a_source_returns_zero() {
        let mut e = engine();
        let data = pointer_fixture(&mut e, 1);
        e.register_double(FINISH_DOUBLE, |_, a| Ret {
            eax: if a[1] == 0x100 { 0 } else { 5 },
            ..Ret::default()
        });
        let source = object_with_slot_10(&mut e, FINISH_DOUBLE);
        assert_eq!(e.call(0x00e0_55f0, &args![data, 0u32, source]).u32(), 0);
        // The walk still reached the second entry.
        assert_eq!(calls(&e, FINISH_DOUBLE).len(), 2);
        assert_eq!(
            calls(&e, STRING_MAP_INSERT),
            vec![vec![data + 0x1c, 0x201, 5]]
        );
    }

    #[test]
    fn get_contents_pointer_skips_the_walk_without_a_source_or_with_a_count() {
        let mut e = engine();
        let data = pointer_fixture(&mut e, 1);
        e.register_double(FINISH_DOUBLE, |_, _| Ret::default());
        let source = object_with_slot_10(&mut e, FINISH_DOUBLE);
        assert_eq!(e.call(0x00e0_55f0, &args![data, 0u32, 0u32]).u32(), 0x4000);
        e.mem.set_u32(data + 0x20, 4);
        assert_eq!(
            e.call(0x00e0_55f0, &args![data, 0u32, source]).u32(),
            0x4000
        );
        assert!(calls(&e, FINISH_DOUBLE).is_empty());
    }

    #[test]
    fn construct_sets_the_defaults_and_takes_a_reference_to_the_source_object() {
        let mut e = engine();
        let allocator = install_allocator(&mut e);
        for address in [
            MAP_RESERVE,
            STRING_MAP_CONSTRUCT,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        let created = e.mem.alloc(0x10);
        e.register_double(SOURCE_DOUBLE, move |_, _| Ret {
            eax: created,
            ..Ret::default()
        });
        let source = object_with_slot_10(&mut e, SOURCE_DOUBLE);
        e.mem.set_u32(SOURCE_OBJECT_GLOBAL, source);
        let data = e.mem.alloc(0x70);
        for offset in (0..0x70).step_by(4) {
            e.mem.set_u32(data + offset, 0xdead_beef);
        }
        let result = e.call(0x00e0_5700, &args![data]);
        assert_eq!(result.u32(), data);
        assert_eq!(e.mem.u16(data + 6), 1);
        assert_eq!(e.mem.u32(data), PACKFILE_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u32(data + 0x10), 0);
        assert_eq!(e.mem.u32(data + 0x14), 0);
        assert_eq!(e.mem.u32(data + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u32(data + 0x2c), created);
        assert_eq!(e.mem.u32(data + 0x30), 1);
        for (array_data, size, capacity) in [
            (0x34, 0x38, 0x3c),
            (0x40, 0x44, 0x48),
            (0x4c, 0x50, 0x54),
            (0x58, 0x5c, 0x60),
        ] {
            assert_eq!(e.mem.u32(data + array_data), 0);
            assert_eq!(e.mem.u32(data + size), 0);
            assert_eq!(e.mem.u32(data + capacity), 0x8000_0000);
        }
        assert_eq!(
            calls(&e, MAP_RESERVE),
            vec![vec![data + 0x10, allocator, 0]]
        );
        let constructs = calls(&e, STRING_MAP_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        assert_eq!(constructs[0][0], data + 0x1c);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![created]]);
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
    }

    #[test]
    fn construct_skips_the_reference_when_the_source_gives_nothing() {
        let mut e = engine();
        install_allocator(&mut e);
        for address in [
            MAP_RESERVE,
            STRING_MAP_CONSTRUCT,
            ADD_REFERENCE,
            SOURCE_DOUBLE,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        let source = object_with_slot_10(&mut e, SOURCE_DOUBLE);
        e.mem.set_u32(SOURCE_OBJECT_GLOBAL, source);
        let data = e.mem.alloc(0x70);
        e.call(0x00e0_5700, &args![data]);
        assert_eq!(e.mem.u32(data + 0x2c), 0);
        assert!(calls(&e, ADD_REFERENCE).is_empty());
    }

    /// A packfile data to destroy: two pointers `0xa1`, `0xa2` (owned array, capacity 4); two
    /// chunks `(0xb1, 0x10)`, `(0xb2, 0x20)` of 12 bytes (array not owned); a name `0xc1`; owned
    /// exports (data `0x2000`, capacity 5) and imports (data `0x3000`, capacity 6); a referenced
    /// object `0xd1`; the flag word zero so that `callDestructors` only clears. Returns it with
    /// the address of the pointer array.
    fn destruct_fixture(e: &mut Engine) -> (u32, u32) {
        for address in [
            EASY_FREE,
            MAP_CLEAR,
            STRING_MAP_CLEAR,
            STRING_MAP_DESTRUCT,
            MAP_CLEAR_AND_DEALLOCATE,
            NO_EFFECT,
            REMOVE_REFERENCE,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, 0xa1);
        e.mem.set_u32(pointers + 4, 0xa2);
        let chunks = e.mem.alloc(0x18);
        for (i, word) in [0xb1u32, 0x10, 0, 0xb2, 0x20, 0].into_iter().enumerate() {
            e.mem.set_u32(chunks + 4 * i as u32, word);
        }
        let data = e.mem.alloc(0x70);
        e.mem.set_u32(data + 0xc, 0xc1);
        e.mem.set_u32(data + 0x2c, 0xd1);
        e.mem.set_u32(data + 0x30, 0);
        e.mem.set_u32(data + 0x34, pointers);
        e.mem.set_u32(data + 0x38, 2);
        e.mem.set_u32(data + 0x3c, 4);
        e.mem.set_u32(data + 0x40, chunks);
        e.mem.set_u32(data + 0x44, 2);
        e.mem.set_u32(data + 0x48, 0x8000_0002);
        e.mem.set_u32(data + 0x4c, 0x2000);
        e.mem.set_u32(data + 0x50, 3);
        e.mem.set_u32(data + 0x54, 5);
        e.mem.set_u32(data + 0x58, 0x3000);
        e.mem.set_u32(data + 0x5c, 1);
        e.mem.set_u32(data + 0x60, 6);
        (data, pointers)
    }

    #[test]
    fn destruct_frees_everything_it_owns_and_resets_the_arrays() {
        let mut e = engine();
        let allocator = install_allocator(&mut e);
        let (data, pointers) = destruct_fixture(&mut e);
        e.call(0x00e0_57c0, &args![data]);
        assert_eq!(
            calls(&e, EASY_FREE),
            vec![
                vec![allocator, 0xa1],
                vec![allocator, 0xa2],
                vec![allocator, 0xc1]
            ]
        );
        assert_eq!(
            calls(&e, FREE_BLOCK_DOUBLE),
            vec![vec![allocator, 0xb1, 0x10], vec![allocator, 0xb2, 0x20]]
        );
        // Imports, exports, then the pointer array; the chunk array is not owned.
        assert_eq!(
            calls(&e, FREE_DATA_DOUBLE),
            vec![
                vec![allocator, 0x3000, 48],
                vec![allocator, 0x2000, 40],
                vec![allocator, pointers, 16]
            ]
        );
        for (array, size, capacity) in [
            (0x34, 0x38, 0x3c),
            (0x40, 0x44, 0x48),
            (0x4c, 0x50, 0x54),
            (0x58, 0x5c, 0x60),
        ] {
            assert_eq!(e.mem.u32(data + array), 0);
            assert_eq!(e.mem.u32(data + size), 0);
            assert_eq!(e.mem.u32(data + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(data + 0x2c), 0);
        assert_eq!(e.mem.u32(data), BASE_VTABLE);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0xd1]]);
        assert_eq!(calls(&e, STRING_MAP_DESTRUCT), vec![vec![data + 0x1c]]);
        assert_eq!(
            calls(&e, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![data + 0x10, allocator]]
        );
        assert_eq!(calls(&e, NO_EFFECT), vec![vec![data + 0x10]]);
        // callDestructors ran first (it clears the maps), the map teardown came last.
        assert_eq!(
            order(
                &e,
                &[
                    MAP_CLEAR,
                    STRING_MAP_CLEAR,
                    EASY_FREE,
                    STRING_MAP_DESTRUCT,
                    MAP_CLEAR_AND_DEALLOCATE,
                    NO_EFFECT
                ]
            ),
            vec![
                MAP_CLEAR,
                STRING_MAP_CLEAR,
                EASY_FREE,
                EASY_FREE,
                EASY_FREE,
                STRING_MAP_DESTRUCT,
                MAP_CLEAR_AND_DEALLOCATE,
                NO_EFFECT
            ]
        );
    }

    #[test]
    fn destruct_of_an_empty_object_frees_only_the_name_and_skips_the_reference() {
        let mut e = engine();
        let allocator = install_allocator(&mut e);
        let (data, _) = destruct_fixture(&mut e);
        for offset in [0x34, 0x38, 0x40, 0x44, 0x4c, 0x50, 0x58, 0x5c, 0x2c] {
            e.mem.set_u32(data + offset, 0);
        }
        for offset in [0x3c, 0x48, 0x54, 0x60] {
            e.mem.set_u32(data + offset, 0x8000_0000);
        }
        e.call(0x00e0_57c0, &args![data]);
        assert_eq!(calls(&e, EASY_FREE), vec![vec![allocator, 0xc1]]);
        assert!(calls(&e, FREE_BLOCK_DOUBLE).is_empty());
        assert!(calls(&e, FREE_DATA_DOUBLE).is_empty());
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
    }

    #[test]
    fn fn_00e05950_finds_the_first_used_entry() {
        let mut e = engine();
        let entries = e.mem.alloc(0x20);
        for index in 0..4u32 {
            let key = if index == 2 { 7 } else { 0xffff_ffff };
            e.mem.set_u32(entries + 8 * index, key);
        }
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, entries);
        e.mem.set_u32(map + 8, 3);
        assert_eq!(e.call(0x00e0_5950, &args![map]).i32(), 2);
        // No used entry: one past the mask. A negative mask scans nothing.
        e.mem.set_u32(entries + 16, 0xffff_ffff);
        assert_eq!(e.call(0x00e0_5950, &args![map]).i32(), 4);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        assert_eq!(e.call(0x00e0_5950, &args![map]).i32(), 0);
    }
}
