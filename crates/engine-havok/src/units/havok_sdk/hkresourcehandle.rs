//! `Havok SDK/hkresourcehandle.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e2f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e2_ea40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e2f210(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e2_ea40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e2f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e2_ec00`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e2f240(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e2_ec00, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::addReference` and `removeReference` (the locked variants), `this` in
/// `ECX`.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `00c908c0` (cdecl: allocator, array, capacity, element size) and `00c90940` (cdecl:
/// allocator, array, element size, one more element).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkStringPtr`, `this` in `ECX`: the default constructor (`00cb0a60`), the constructor from
/// text (`00cb0a70`, one word), the constructor from a text with a copy (`00cb0b80`, one
/// word), the destructor (`00cb0c00`) and `operator=` (`00cb0c10`, one word).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_CONSTRUCT_FROM_TEXT: u32 = 0x00cb_0a70;
const STRING_PTR_CONSTRUCT_COPY: u32 = 0x00cb_0b80;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// `hkString::strCmp` (`00cb0fa0`, cdecl, 0 when equal), `strChr` (`00cb1140`, cdecl `(text,
/// char)`) and `memCpy` (`00cb1260`, cdecl `(dest, source, size)`).
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const STRING_FIND_CHAR: u32 = 0x00cb_1140;
const MEMORY_COPY: u32 = 0x00cb_1260;
/// `hkCachedHashMap<hkStringMapOperations>::getWithDefault` (`00d87800`, `this` = the map,
/// `(key, default)`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
/// `hkRefVariant::getClass` (`00d477a0`) and `set` (`00d47770`), `this` = the variant.
const REF_VARIANT_GET_CLASS: u32 = 0x00d4_77a0;
const REF_VARIANT_SET: u32 = 0x00d4_7770;
/// `hkClass` pieces, `this` = the class in `ECX`: `getName` (`00d46d40`), `getMemberByName`
/// (`00d46fe0`, one word) and the derivation test `00d46d80` (`(out byte, other class)`, returns
/// `out`, a byte that is nonzero when `other` is `this` or derives from it).
const CLASS_NAME: u32 = 0x00d4_6d40;
const CLASS_MEMBER_BY_NAME: u32 = 0x00d4_6fe0;
const CLASS_CHAIN_CONTAINS: u32 = 0x00d4_6d80;
/// `hkClassMember::getClass` (`00d86920`, the word at `+4`), `hkClassMemberAccessor`'s
/// constructor (`00d87180`, `this` = the accessor record, `(object, member)`, returns `this`),
/// `getClassMember` (`00d87370`, the word at `+4`) and `asPointer` (`00d87210`, one word: the
/// address of pointer number `index` from the accessor's first word).
const MEMBER_CLASS: u32 = 0x00d8_6920;
const ACCESSOR_CONSTRUCT: u32 = 0x00d8_7180;
const ACCESSOR_CLASS_MEMBER: u32 = 0x00d8_7370;
const ACCESSOR_AS_POINTER: u32 = 0x00d8_7210;
/// The operator `delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `hkCachedHashMap`'s destructor (`00d877d0`, `this` = the map).
const MAP_DESTRUCTOR: u32 = 0x00d8_77d0;
/// The error-report pieces: the string stream's constructor (`00cb1600`, `this` = the 0xC-byte
/// stream, `(buffer, size)`), its text append (`00cc01c0`) and destructor (`00cc00e0`), the
/// logger object's pointer (`01268360`, its slot `0xC` reports `(level, id, text, file,
/// line)`) and the source file name (`".\ResourceDatabase\hkResourceHandle.cpp"`).
const STREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
const STREAM_APPEND: u32 = 0x00cc_01c0;
const STREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
const LOGGER_GLOBAL: u32 = 0x0126_8360;
const RESOURCE_FILE_NAME: u32 = 0x010e_b1c4;
/// The message texts of the error reports.
const TEXT_CANNOT_FIND_MEMBER: u32 = 0x010e_b1ac;
const TEXT_NOT_A_STRUCT: u32 = 0x010e_b1ec;
const TEXT_CLASS_MISMATCH: u32 = 0x010e_b20c;
const TEXT_CLASS_MISMATCH_MIDDLE: u32 = 0x010e_b234;
const TEXT_CANNOT_PARENT: u32 = 0x010e_b23c;
const TEXT_PARENT_TO: u32 = 0x010e_b24c;
const TEXT_CIRCULAR: u32 = 0x010e_b254;
/// `hkStringPtr`'s `operator<<` on the output stream (`00cc05a0`, `this` = the stream, one
/// word) and the empty name the handle's `getName` gives (`0x010eb1a4`).
const STREAM_APPEND_STRING_PTR: u32 = 0x00cc_05a0;
const EMPTY_NAME: u32 = 0x010e_b1a4;
/// Vtables: `hkResourceMap`'s purecall one (`0x010eb19c`), the memory resource handle's
/// (`0x010eb288`), the memory resource container's (`0x010eb2c4`) and the base (`0x0102e388`).
const RESOURCE_MAP_VTABLE: u32 = 0x010e_b19c;
const MEMORY_HANDLE_VTABLE: u32 = 0x010e_b288;
const MEMORY_CONTAINER_VTABLE: u32 = 0x010e_b2c4;
const BASE_VTABLE: u32 = 0x0102_e388;

/// The allocator object: the word at `+0x10` of the memory router's structure (`+0xC` for the
/// temporary one, see [`temporary_allocator`]).
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The router's allocator at `+0xC`, which `findResourceByName` uses for its name buffer.
fn temporary_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0xc)
}

/// Makes room for one more element of `element_size` bytes in the array (data at `+0`, size at
/// `+4`, capacity at `+8` with flag bits in the top two): when the size equals the capacity,
/// calls `00c90940`.
fn grow_if_full(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

/// Appends a pointer to the array of 4-byte elements at `array` (see [`grow_if_full`]).
fn push_pointer(e: &mut Engine, array: u32, value: u32) {
    grow_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    e.mem.set_u32(slot, value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Gives the storage of an empty-able array of `element_size`-byte elements back through the
/// allocator's slot `0x10` when the capacity word is not negative.
fn free_storage(e: &mut Engine, allocator: u32, data: u32, capacity: u32, element_size: u32) {
    if (capacity as i32) >= 0 {
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(element_size)],
        );
    }
}

/// The error report: a 0x200-byte string stream gets the texts (each call's result is the
/// stream for the next), the logger's slot `0xC` reports `(1, id, buffer, file, line)` (its
/// result is not used) and the stream is destroyed.
fn report_error(e: &mut Engine, texts: impl FnOnce(&mut Engine) -> Vec<u32>, id: u32, line: u32) {
    e.with_stack(0x20c, |e, stream| {
        let buffer = stream.addr() + 0xc;
        e.call(STREAM_CONSTRUCTOR, &args![stream, buffer, 0x200u32]);
        let mut current = stream.addr();
        for text in texts(e) {
            current = e.call(STREAM_APPEND, &args![current, text]).u32();
        }
        let logger = e.mem.u32(LOGGER_GLOBAL);
        e.vcall(
            logger,
            0xc,
            &args![1u32, id, buffer, RESOURCE_FILE_NAME, line],
        );
        e.call(STREAM_DESTRUCTOR, &args![stream]);
    });
}

/// Runs the derivation test `00d46d80` (`this` = `class`) on `other` and returns its byte.
fn class_derives(e: &mut Engine, class: u32, other: u32) -> u8 {
    e.with_stack(4, |e, out| {
        let result = e
            .call(CLASS_CHAIN_CONTAINS, &args![class, out, other])
            .u32();
        e.mem.u8(result)
    })
}

// Translated from 00e2def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkResourceMap::_scalar_deleting_destructor_` (Xbox PDB), `(flags)`: installs the purecall
/// vtable `0x010eb19c` and, with bit 0 of `flags`, calls the operator `delete` (`00401030`) on
/// `this`. Returns `this`.
pub fn hk_resource_map_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), RESOURCE_MAP_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00e2df10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::setName` (Xbox PDB), `(name)`: a tail jump to `hkStringPtr::operator=`
/// (`00cb0c10`) on the string at `this + 0xC` with the one word; returns its result.
pub fn hk_memory_resource_handle_set_name(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    e.call(STRING_PTR_ASSIGN, &args![this.addr() + 0xc, name])
        .u32()
}

// Translated from 00e2df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::getClass` (Xbox PDB): a tail jump to `hkRefVariant::getClass`
/// (`00d477a0`) on the variant at `this + 8`; returns its result.
pub fn hk_memory_resource_handle_get_class(e: &mut Engine, this: Ptr) -> u32 {
    e.call(REF_VARIANT_GET_CLASS, &args![this.addr() + 8]).u32()
}

// Translated from 00e2df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::setObject` (Xbox PDB), `(object, _unused)`: a tail jump to
/// `hkRefVariant::set` (`00d47770`, two stack words) on the variant at `this + 8`.
pub fn hk_memory_resource_handle_set_object(e: &mut Engine, this: Ptr, object: u32, unused_2: u32) {
    e.call(REF_VARIANT_SET, &args![this.addr() + 8, object, unused_2]);
}

// Translated from 00e2df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::getName` (Xbox PDB), one unused stack word: the name text (the
/// word at `+0xC` without its low flag bit) or the empty name at `0x010eb1a4` when it is null.
pub fn hk_memory_resource_handle_get_name(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let name = e.mem.u32(this.addr() + 0xc) & 0xffff_fffe;
    if name == 0 {
        EMPTY_NAME
    } else {
        name
    }
}

// Translated from 00e2df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::getObject` (Xbox PDB): the object pointer at `+8`.
pub fn hk_memory_resource_handle_get_object(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00e2df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::getName` (Xbox PDB), one unused stack word: the name word at
/// `+8` without its low flag bit.
pub fn hk_memory_resource_container_get_name(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    e.mem.u32(this.addr() + 8) & 0xffff_fffe
}

// Translated from 00e2df70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::getNumContainers` (Xbox PDB): the container count at `+0x20`.
pub fn hk_memory_resource_container_get_num_containers(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x20)
}

// Translated from 00e2df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkContainerResourceMap::findObjectByName` (Xbox PDB), `(name, out class)`: clears `*out_class`
/// when it is not null; looks `name` up in the map at `this + 4` (`getWithDefault`, default 0)
/// and, when found, stores the object's class (its virtual slot `0x1C`) in `*out_class` when
/// that is not null, then calls the object's virtual slot `0x18` (no arguments).
///
/// Answers the object pointer: the lookup's 0 when nothing was found (the exe returns with it
/// still in `EAX`), else slot `0x18`'s `EAX`, which the exe ends on.
/// `hkResourceHandle::tryToResolveLinks` (`00e2e4c0`, 00e2e52a) reads it from this slot and skips
/// a link whose answer is null.
pub fn hk_container_resource_map_find_object_by_name(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    out_class: Ptr,
) -> u32 {
    if out_class.addr() != 0 {
        e.mem.set_u32(out_class.addr(), 0);
    }
    let object = e
        .call(MAP_GET_WITH_DEFAULT, &args![this.addr() + 4, name, 0u32])
        .u32();
    if object == 0 {
        return object;
    }
    if out_class.addr() != 0 {
        let class = e.vcall(object, 0x1c, &args![]).u32();
        e.mem.set_u32(out_class.addr(), class);
    }
    e.vcall(object, 0x18, &args![]).u32()
}

// Translated from 00e2dfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::findContainerByName` (Xbox PDB), `(name, after)`: the search
/// starts after the container `after` in the array at `+0x1C` (count at `+0x20`) when it is
/// not null and in the array, else at the first one. Returns the first container from there
/// whose name (`+8` without its flag bit) equals `name` (`hkString::strCmp`), or the first
/// one when `name` is null, or 0.
pub fn hk_memory_resource_container_find_container_by_name(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    after: u32,
) -> u32 {
    let mut index: i32 = 0;
    while after != 0 && index < e.mem.u32(this.addr() + 0x20) as i32 {
        let slot = e
            .mem
            .u32(this.addr() + 0x1c)
            .wrapping_add((index as u32).wrapping_mul(4));
        index += 1;
        if e.mem.u32(slot) == after {
            break;
        }
    }
    while index < e.mem.u32(this.addr() + 0x20) as i32 {
        let list = e.mem.u32(this.addr() + 0x1c);
        let container = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
        if name == 0 {
            return container;
        }
        let container_name = e.mem.u32(container + 8) & 0xffff_fffe;
        if e.call(STRING_COMPARE, &args![name, container_name]).i32() == 0 {
            return container;
        }
        index += 1;
    }
    0
}

// Translated from 00e2e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::addExternalLink` (Xbox PDB), `(a, b)`: appends an 8-byte record of
/// two `hkStringPtr`s to the array at `this + 0x10` (size at `+0x14`, capacity at `+0x18`),
/// making room with `00c90940`: both are constructed (`00cb0a60`) when the slot address is
/// not null, the size grows, and then `a` and `b` are assigned to them (`00cb0c10`).
pub fn hk_memory_resource_handle_add_external_link(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let array = this.addr() + 0x10;
    grow_if_full(e, array, 8);
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    if slot != 0 {
        e.call(STRING_PTR_CONSTRUCT, &args![slot]);
        e.call(STRING_PTR_CONSTRUCT, &args![slot + 4]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(STRING_PTR_ASSIGN, &args![slot, a]);
    e.call(STRING_PTR_ASSIGN, &args![slot + 4, b]);
}

/// The `Cannot find member : <name>` report of `getExternalLinks` (id `0xf032edf1`, line
/// 0x92).
fn report_cannot_find_member(e: &mut Engine, name: u32) {
    report_error(
        e,
        |_| vec![TEXT_CANNOT_FIND_MEMBER, name],
        0xf032_edf1,
        0x92,
    );
}

// Translated from 00e2e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::getExternalLinks` (Xbox PDB), `(out array)`: the array at `out`
/// (0x10-byte elements) is emptied and made big enough for the link count (`this + 0x14`).
/// Each link (8 bytes at `this + 0x10`: the member path and the target name, both with a
/// flag bit) becomes a record `(path, target name, accessor data, accessor member)`: the path
/// is split at `.`, each component but the last is found in the current class and must be a
/// struct member (type 0x19; the object address moves by its offset and the class by its
/// class), and the last is found in the final class; the accessor
/// (`hkClassMemberAccessor`, `00d87180`) is made on the object address. A missing member or a
/// non-struct component reports an error ("Member is not of type struct : <component>"
/// id `0xf032edfe` line 0x80, then "Cannot find member : <rest>" id `0xf032edf1` line 0x92)
/// and drops the record again.
pub fn hk_memory_resource_handle_get_external_links(e: &mut Engine, this: Ptr, out: Ptr) {
    let out = out.addr();
    e.mem.set_u32(out + 4, 0);
    let links = e.mem.u32(this.addr() + 0x14) as i32;
    let allocator = heap_allocator(e);
    let capacity = (e.mem.u32(out + 8) & 0x3fff_ffff) as i32;
    if capacity < links {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if links < doubled { doubled } else { links };
        e.call(ARRAY_RESERVE, &args![allocator, out, wanted, 0x10u32]);
    }
    e.with_stack(0x300, |e, frame| {
        let component = frame.addr();
        let temporary = frame.addr() + 0x2f8;
        let mut index: i32 = 0;
        if e.mem.u32(this.addr() + 0x14) as i32 <= 0 {
            return;
        }
        loop {
            let size = e.mem.u32(out + 4);
            let record = size.wrapping_mul(0x10).wrapping_add(e.mem.u32(out));
            e.mem.set_u32(out + 4, size.wrapping_add(1));
            let table = e.mem.u32(this.addr() + 0x10);
            let mut path = e
                .mem
                .u32(table.wrapping_add((index as u32).wrapping_mul(8)))
                & 0xffff_fffe;
            let mut object = e.mem.u32(this.addr() + 8);
            let mut class = e.call(REF_VARIANT_GET_CLASS, &args![this.addr() + 8]).u32();
            let mut failed = false;
            let mut dot = e.call(STRING_FIND_CHAR, &args![path, 0x2eu32]).u32();
            while dot != 0 {
                let length = dot.wrapping_sub(path);
                e.call(MEMORY_COPY, &args![component, path, length]);
                e.mem.set_u8(component.wrapping_add(length), 0);
                path = path.wrapping_add(length).wrapping_add(1);
                let member = e.call(CLASS_MEMBER_BY_NAME, &args![class, component]).u32();
                if member == 0 {
                    failed = true;
                    break;
                }
                if e.mem.u8(member + 0xc) != 0x19 {
                    report_error(e, |_| vec![TEXT_NOT_A_STRUCT, component], 0xf032_edfe, 0x80);
                    failed = true;
                    break;
                }
                class = e.call(MEMBER_CLASS, &args![member]).u32();
                object = object.wrapping_add(u32::from(e.mem.u16(member + 0x12)));
                dot = e.call(STRING_FIND_CHAR, &args![path, 0x2eu32]).u32();
            }
            if !failed {
                let member = e.call(CLASS_MEMBER_BY_NAME, &args![class, path]).u32();
                if member == 0 {
                    failed = true;
                } else {
                    e.mem.set_u32(record, path);
                    let accessor = e
                        .call(ACCESSOR_CONSTRUCT, &args![temporary, object, member])
                        .u32();
                    let data = e.mem.u32(accessor);
                    e.mem.set_u32(record + 8, data);
                    let accessor_member = e.mem.u32(accessor + 4);
                    e.mem.set_u32(record + 0xc, accessor_member);
                    let table = e.mem.u32(this.addr() + 0x10);
                    let target = e.mem.u32(
                        table
                            .wrapping_add((index as u32).wrapping_mul(8))
                            .wrapping_add(4),
                    ) & 0xffff_fffe;
                    e.mem.set_u32(record + 4, target);
                }
            }
            if failed {
                let size = e.mem.u32(out + 4);
                e.mem.set_u32(out + 4, size.wrapping_sub(1));
                report_cannot_find_member(e, path);
            }
            index += 1;
            if index >= e.mem.u32(this.addr() + 0x14) as i32 {
                break;
            }
        }
    });
}

// Translated from 00e2e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map, `this` = a container, `(array)`: appends to the array of
/// pointers at `array` (making room with `00c90940`) every resource of the container and of
/// the containers below it: first, for each container found by virtual slot `0x34`
/// (`findContainerByName(0, previous)`, starting from 0), this function on that container,
/// then every resource found by virtual slot `0x20` (`findResourceByName(0, 0, previous)`).
pub fn fn_00e2e2e0(e: &mut Engine, this: Ptr, array: Ptr) {
    let mut child = e.vcall(this.addr(), 0x34, &args![0u32, 0u32]).u32();
    while child != 0 {
        fn_00e2e2e0(e, Ptr::new(child), array);
        child = e.vcall(this.addr(), 0x34, &args![0u32, child]).u32();
    }
    let mut resource = e.vcall(this.addr(), 0x20, &args![0u32, 0u32, 0u32]).u32();
    while resource != 0 {
        push_pointer(e, array.addr(), resource);
        resource = e
            .vcall(this.addr(), 0x20, &args![0u32, 0u32, resource])
            .u32();
    }
}

// Translated from 00e2e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map, `this` = a container, `(array)`: appends `this`, then (this function
/// on each of its containers, found through virtual slot `0x34`) the containers below it, to
/// the array of pointers at `array`.
pub fn fn_00e2e380(e: &mut Engine, this: Ptr, array: Ptr) {
    push_pointer(e, array.addr(), this.addr());
    let mut child = e.vcall(this.addr(), 0x34, &args![0u32, 0u32]).u32();
    while child != 0 {
        fn_00e2e380(e, Ptr::new(child), array);
        child = e.vcall(this.addr(), 0x34, &args![0u32, child]).u32();
    }
}

// Translated from 00e2e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::clearExternalLinks` (Xbox PDB): destroys both `hkStringPtr`s of every
/// link record (the second field first, `00cb0c00`) and sets the count at `+0x14` to 0.
pub fn hk_memory_resource_handle_clear_external_links(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0x14) as i32;
    let mut link = e.mem.u32(this.addr() + 0x10);
    if count > 0 {
        let mut remaining = count;
        while remaining != 0 {
            e.call(STRING_PTR_DESTRUCT, &args![link + 4]);
            e.call(STRING_PTR_DESTRUCT, &args![link]);
            link = link.wrapping_add(8);
            remaining -= 1;
        }
    }
    e.mem.set_u32(this.addr() + 0x14, 0);
}

// Translated from 00e2e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::removeExternalLink` (Xbox PDB), `(name)`: finds the first link
/// whose first string (`+0` without its flag bit) equals `name` (`strCmp`), destroys its two
/// `hkStringPtr`s, lowers the count and copies the last record into its place when it was not
/// the last.
pub fn hk_memory_resource_handle_remove_external_link(e: &mut Engine, this: Ptr, name: u32) {
    let mut index: i32 = 0;
    if e.mem.u32(this.addr() + 0x14) as i32 <= 0 {
        return;
    }
    loop {
        let table = e.mem.u32(this.addr() + 0x10);
        let link_name = e
            .mem
            .u32(table.wrapping_add((index as u32).wrapping_mul(8)))
            & 0xffff_fffe;
        if e.call(STRING_COMPARE, &args![link_name, name]).i32() == 0 {
            break;
        }
        index += 1;
        if index >= e.mem.u32(this.addr() + 0x14) as i32 {
            return;
        }
    }
    let offset = (index as u32).wrapping_mul(8);
    let link = e.mem.u32(this.addr() + 0x10).wrapping_add(offset);
    e.call(STRING_PTR_DESTRUCT, &args![link + 4]);
    e.call(STRING_PTR_DESTRUCT, &args![link]);
    let count = e.mem.u32(this.addr() + 0x14).wrapping_sub(1);
    e.mem.set_u32(this.addr() + 0x14, count);
    if count as i32 != index {
        let table = e.mem.u32(this.addr() + 0x10);
        let last = table.wrapping_add(count.wrapping_mul(8));
        let slot = table.wrapping_add(offset);
        for word in 0..2 {
            let value = e.mem.u32(last + 4 * word);
            e.mem.set_u32(slot + 4 * word, value);
        }
    }
}

/// Handles one link of `tryToResolveLinks`: the resolver (virtual slot 0, `(target name, &class)`)
/// finds the target; returns whether the whole run stops (the target is this handle's own
/// object).
fn resolve_one_link(
    e: &mut Engine,
    this: Ptr,
    resolver: Ptr,
    element: u32,
    found_class_slot: u32,
) -> bool {
    let target = e.mem.u32(element + 4);
    let object = e
        .vcall(resolver.addr(), 0, &args![target, found_class_slot])
        .u32();
    if object == 0 {
        return false;
    }
    let accessor = element + 8;
    let member = e.call(ACCESSOR_CLASS_MEMBER, &args![accessor]).u32();
    let member_class = e.call(MEMBER_CLASS, &args![member]).u32();
    let found_class = e.mem.u32(found_class_slot);
    if class_derives(e, member_class, found_class) == 0 && found_class != member_class {
        report_error(
            e,
            |e| {
                let member_class_name = e.call(CLASS_NAME, &args![member_class]).u32();
                let found_class_name = e.call(CLASS_NAME, &args![found_class]).u32();
                vec![
                    TEXT_CLASS_MISMATCH,
                    found_class_name,
                    TEXT_CLASS_MISMATCH_MIDDLE,
                    member_class_name,
                ]
            },
            0xf034_ed21,
            0xb2,
        );
        return false;
    }
    let own_object = e.vcall(this.addr(), 0x18, &args![]).u32();
    if object == own_object {
        return true;
    }
    let pointer = e.call(ACCESSOR_AS_POINTER, &args![accessor, 0u32]).u32();
    e.mem.set_u32(pointer, object);
    let name = e.mem.u32(member);
    e.vcall(this.addr(), 0x28, &args![name]);
    false
}

// Translated from 00e2e4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkResourceHandle::tryToResolveLinks` (Xbox PDB), `(resolver)`: gets the external links
/// (virtual slot `0x2C`, into an array on the stack) and, from the last to the first, asks
/// the resolver (its virtual slot 0: `(target name, &class)`) for the target. For a found
/// target the member's class (`getClassMember`, `getClass`) must be the target's class or
/// one of its ancestors (`00d46d80`), else "Class mismatch, cannot resolve link: ..." is
/// reported (id `0xf034ed21`, line 0xb2) and the link is skipped; a match stops the whole
/// run when the target is this handle's own object (virtual slot `0x18`), otherwise stores
/// the target through the accessor (`asPointer(0)`) and calls virtual slot `0x28` with the
/// member's name. The array's storage is given back at the end.
pub fn hk_resource_handle_try_to_resolve_links(e: &mut Engine, this: Ptr, resolver: Ptr) {
    e.with_stack(0x10, |e, links| {
        let links = links.addr();
        e.mem.set_u32(links, 0);
        e.mem.set_u32(links + 4, 0);
        e.mem.set_u32(links + 8, 0x8000_0000);
        e.vcall(this.addr(), 0x2c, &args![links]);
        let mut index = (e.mem.u32(links + 4) as i32).wrapping_sub(1);
        if index >= 0 {
            let mut offset = (index as u32) << 4;
            loop {
                let element = e.mem.u32(links).wrapping_add(offset);
                let stop = e.with_stack(4, |e, class_slot| {
                    resolve_one_link(e, this, resolver, element, class_slot.addr())
                });
                if stop {
                    break;
                }
                index -= 1;
                offset = offset.wrapping_sub(0x10);
                if index < 0 {
                    break;
                }
            }
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(links + 8);
        e.mem.set_u32(links + 4, 0);
        let data = e.mem.u32(links);
        free_storage(e, allocator, data, capacity, 0x10);
    });
}

/// Finds the index of `item` in the array of pointers `count` long at `data`, or -1.
fn index_of_pointer(e: &Engine, data: u32, count: i32, item: u32) -> i32 {
    for index in 0..count.max(0) {
        if e.mem.u32(data.wrapping_add(4 * index as u32)) == item {
            return index;
        }
    }
    -1
}

/// Shifts the pointers after element `index` of the array at `data` one place down, as
/// `destroyResource` and `parentTo` do: `(size - index) * 4` bytes, in 4-byte steps.
fn shift_down(e: &mut Engine, data: u32, index: i32, size: i32) {
    let bytes = size.wrapping_sub(index).wrapping_mul(4);
    let mut slot = data.wrapping_add((index as u32).wrapping_mul(4));
    if bytes > 0 {
        let mut steps = (((bytes - 1) as u32) >> 2) + 1;
        while steps != 0 {
            let next = e.mem.u32(slot.wrapping_add(4));
            e.mem.set_u32(slot, next);
            slot = slot.wrapping_add(4);
            steps -= 1;
        }
    }
}

// Translated from 00e2e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::destroyResource` (Xbox PDB), `(resource)`: holds a reference on
/// the resource (when not null) while it looks it up in the array of resources at `+0x10`
/// (count at `+0x14`); drops that reference; and, when it was in the array, removes the
/// array's own reference, clears the slot, lowers the count and shifts the later entries one
/// place down.
pub fn hk_memory_resource_container_destroy_resource(e: &mut Engine, this: Ptr, resource: Ptr) {
    let resource = resource.addr();
    if resource != 0 {
        e.call(ADD_REFERENCE, &args![resource]);
    }
    let count = e.mem.u32(this.addr() + 0x14) as i32;
    let data = e.mem.u32(this.addr() + 0x10);
    let index = index_of_pointer(e, data, count, resource);
    if resource != 0 {
        e.call(REMOVE_REFERENCE, &args![resource]);
    }
    if index > -1 {
        let data = e.mem.u32(this.addr() + 0x10);
        let slot = data.wrapping_add((index as u32).wrapping_mul(4));
        let held = e.mem.u32(slot);
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
        e.mem.set_u32(slot, 0);
        let size = e.mem.u32(this.addr() + 0x14).wrapping_sub(1);
        e.mem.set_u32(this.addr() + 0x14, size);
        let data = e.mem.u32(this.addr() + 0x10);
        shift_down(e, data, index, size as i32);
    }
}

// Translated from 00e2e710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::destroyContainer` (Xbox PDB), `(container)`: the same for the
/// array of containers at `+0x1C` (count at `+0x20`), but the removed slot is filled with the
/// last entry (when it was not the last) instead of shifting.
pub fn hk_memory_resource_container_destroy_container(e: &mut Engine, this: Ptr, container: Ptr) {
    let container = container.addr();
    if container != 0 {
        e.call(ADD_REFERENCE, &args![container]);
    }
    let count = e.mem.u32(this.addr() + 0x20) as i32;
    let data = e.mem.u32(this.addr() + 0x1c);
    let index = index_of_pointer(e, data, count, container);
    if container != 0 {
        e.call(REMOVE_REFERENCE, &args![container]);
    }
    if index > -1 {
        let data = e.mem.u32(this.addr() + 0x1c);
        let offset = (index as u32).wrapping_mul(4);
        let held = e.mem.u32(data.wrapping_add(offset));
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
        e.mem.set_u32(data.wrapping_add(offset), 0);
        let size = e.mem.u32(this.addr() + 0x20).wrapping_sub(1);
        e.mem.set_u32(this.addr() + 0x20, size);
        if size as i32 != index {
            let data = e.mem.u32(this.addr() + 0x1c);
            let last = e.mem.u32(data.wrapping_add(size.wrapping_mul(4)));
            e.mem.set_u32(data.wrapping_add(offset), last);
        }
    }
}

// Translated from 00e2e790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::parentTo` (Xbox PDB), `(new parent)`: walks `new parent` and
/// its parents (virtual slot `0x24`); when `this` is among them reports "Cannot parent '<this
/// name>' to '<parent name>' as this would create a circular dependency " (id `0xabba4554`,
/// line 0x198) and returns 1. Otherwise holds two references on `this`, removes it from the
/// children array (`+0x1C`, count `+0x20`) of its old parent (`this + 0xC`; the old array
/// reference is dropped and later entries shift down, also when it was not found, with the
/// index -1), takes a reference, appends `this` to the new parent's children array (making room
/// with `00c90940`, a reference for the slot), drops the extra references, sets `this + 0xC` to
/// the new parent and returns 0.
pub fn hk_memory_resource_container_parent_to(e: &mut Engine, this: Ptr, new_parent: Ptr) -> u32 {
    let me = this.addr();
    let mut cursor = new_parent.addr();
    while cursor != 0 {
        if cursor == me {
            e.with_stack(0x20c, |e, stream| {
                let buffer = stream.addr() + 0xc;
                e.call(STREAM_CONSTRUCTOR, &args![stream, buffer, 0x200u32]);
                let mut current = e
                    .call(STREAM_APPEND, &args![stream, TEXT_CANNOT_PARENT])
                    .u32();
                current = e
                    .call(STREAM_APPEND_STRING_PTR, &args![current, me + 8])
                    .u32();
                current = e.call(STREAM_APPEND, &args![current, TEXT_PARENT_TO]).u32();
                current = e
                    .call(
                        STREAM_APPEND_STRING_PTR,
                        &args![current, new_parent.addr() + 8],
                    )
                    .u32();
                e.call(STREAM_APPEND, &args![current, TEXT_CIRCULAR]);
                let logger = e.mem.u32(LOGGER_GLOBAL);
                e.vcall(
                    logger,
                    0xc,
                    &args![1u32, 0xabba_4554u32, buffer, RESOURCE_FILE_NAME, 0x198u32],
                );
                e.call(STREAM_DESTRUCTOR, &args![stream]);
            });
            return 1;
        }
        cursor = e.vcall(cursor, 0x24, &args![]).u32();
    }
    e.call(ADD_REFERENCE, &args![this]);
    if me != 0 {
        e.call(ADD_REFERENCE, &args![this]);
    }
    let old_parent = e.mem.u32(me + 0xc);
    let count = e.mem.u32(old_parent + 0x20) as i32;
    let data = e.mem.u32(old_parent + 0x1c);
    let index = index_of_pointer(e, data, count, me);
    e.call(REMOVE_REFERENCE, &args![this]);
    let old_parent = e.mem.u32(me + 0xc);
    let array = old_parent + 0x1c;
    let data = e.mem.u32(array);
    let slot = data.wrapping_add((index as u32).wrapping_mul(4));
    let held = e.mem.u32(slot);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(slot, 0);
    let size = e.mem.u32(array + 4).wrapping_sub(1);
    e.mem.set_u32(array + 4, size);
    let data = e.mem.u32(array);
    shift_down(e, data, index, size as i32);
    e.call(ADD_REFERENCE, &args![this]);
    let array = new_parent.addr() + 0x1c;
    grow_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    if slot != 0 {
        e.call(ADD_REFERENCE, &args![this]);
        e.mem.set_u32(slot, me);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(REMOVE_REFERENCE, &args![this]);
    e.mem.set_u32(me + 0xc, new_parent.addr());
    e.call(REMOVE_REFERENCE, &args![this]);
    0
}

// Translated from 00e2e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkResourceContainer::tryToResolveLinks` (Xbox PDB), `(resolver)`: collects every resource
/// of the container tree into an array on the stack ([`fn_00e2e2e0`]) and calls each one's
/// virtual slot `0x34` (`tryToResolveLinks`) with the resolver; the array's storage is given
/// back at the end.
pub fn hk_resource_container_try_to_resolve_links(e: &mut Engine, this: Ptr, resolver: Ptr) {
    e.with_stack(0x10, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        fn_00e2e2e0(e, this, Ptr::new(array));
        let mut index: i32 = 0;
        while index < e.mem.u32(array + 4) as i32 {
            let resource = e.mem.u32(e.mem.u32(array).wrapping_add(4 * index as u32));
            e.vcall(resource, 0x34, &args![resolver]);
            index += 1;
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        let data = e.mem.u32(array);
        free_storage(e, allocator, data, capacity, 4);
    });
}

// Translated from 00e2e9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::hkMemoryResourceHandle` (Xbox PDB): reference count 1 (`+6`),
/// vtable `0x010eb288`, the object `+8` 0, the name at `+0xC` constructed (`00cb0a60`), the
/// links array at `+0x10` empty (size `+0x14` 0, capacity `+0x18` `0x80000000`). Returns `this`.
pub fn hk_memory_resource_handle_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), MEMORY_HANDLE_VTABLE);
    e.mem.set_u32(this.addr() + 8, 0);
    e.call(STRING_PTR_CONSTRUCT, &args![this.addr() + 0xc]);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    e.mem.set_u32(this.addr() + 0x18, 0x8000_0000);
    this
}

// Translated from 00e2ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the handle's constructor variant that only installs the vtable
/// `0x010eb288` and constructs the name at `+0xC` from the given text word (`00cb0a70`).
/// Returns `this`.
pub fn fn_00e2ea20(e: &mut Engine, this: Ptr, text: u32) -> Ptr {
    e.mem.set_u32(this.addr(), MEMORY_HANDLE_VTABLE);
    e.call(
        STRING_PTR_CONSTRUCT_FROM_TEXT,
        &args![this.addr() + 0xc, text],
    );
    this
}

// Translated from 00e2ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceHandle::~hkMemoryResourceHandle` (Xbox PDB): installs the vtable
/// `0x010eb288`, destroys both `hkStringPtr`s of every link record (second field first), empties
/// the links array (count 0, storage given back through the allocator's slot `0x10` unless the
/// capacity word is negative, capacity `0x80000000`), destroys the name at `+0xC`, removes the
/// reference on the object at `+8` (when set) and leaves the base vtable.
pub fn hk_memory_resource_handle_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MEMORY_HANDLE_VTABLE);
    let allocator = heap_allocator(e);
    let count = e.mem.u32(this.addr() + 0x14) as i32;
    let mut link = e.mem.u32(this.addr() + 0x10);
    if count > 0 {
        let mut remaining = count;
        while remaining != 0 {
            e.call(STRING_PTR_DESTRUCT, &args![link + 4]);
            e.call(STRING_PTR_DESTRUCT, &args![link]);
            link = link.wrapping_add(8);
            remaining -= 1;
        }
    }
    let capacity = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u32(this.addr() + 0x14, 0);
    let data = e.mem.u32(this.addr() + 0x10);
    free_storage(e, allocator, data, capacity, 8);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x18, 0x8000_0000);
    e.call(STRING_PTR_DESTRUCT, &args![this.addr() + 0xc]);
    let object = e.mem.u32(this.addr() + 8);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00e2eb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::hkMemoryResourceContainer` (Xbox PDB), `(name)`: reference count
/// 1 (`+6`), vtable `0x010eb2c4`, the name at `+8` constructed from `name` (`00cb0b80`, which
/// copies it), the parent `+0xC`, and the two arrays at `+0x10` (resources) and `+0x1C`
/// (containers) empty (size 0, capacity `0x80000000`). Returns `this`.
pub fn hk_memory_resource_container_constructor(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), MEMORY_CONTAINER_VTABLE);
    e.call(STRING_PTR_CONSTRUCT_COPY, &args![this.addr() + 8, name]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    e.mem.set_u32(this.addr() + 0x18, 0x8000_0000);
    e.mem.set_u32(this.addr() + 0x1c, 0);
    e.mem.set_u32(this.addr() + 0x20, 0);
    e.mem.set_u32(this.addr() + 0x24, 0x8000_0000);
    this
}

// Translated from 00e2ebc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the container's constructor variant that installs the vtable
/// `0x010eb2c4`, constructs the name at `+8` from the text word (`00cb0a70`) and, when that is
/// not null, sets the parent (`+0xC`) of every container in the array at `+0x1C` to `this`.
/// Returns `this`.
pub fn fn_00e2ebc0(e: &mut Engine, this: Ptr, text: u32) -> Ptr {
    e.mem.set_u32(this.addr(), MEMORY_CONTAINER_VTABLE);
    e.call(
        STRING_PTR_CONSTRUCT_FROM_TEXT,
        &args![this.addr() + 8, text],
    );
    if text != 0 && e.mem.u32(this.addr() + 0x20) as i32 > 0 {
        let mut index: u32 = 0;
        loop {
            let list = e.mem.u32(this.addr() + 0x1c);
            let child = e.mem.u32(list.wrapping_add(4 * index));
            index += 1;
            e.mem.set_u32(child + 0xc, this.addr());
            if index as i32 >= e.mem.u32(this.addr() + 0x20) as i32 {
                break;
            }
        }
    }
    this
}

/// Drops the reference on each of the `count` pointers at `data` and clears them (the loop of
/// the container's destructor).
fn release_pointers(e: &mut Engine, data: u32, count: i32) {
    let mut index: u32 = 0;
    while (index as i32) < count {
        let slot = data.wrapping_add(4 * index);
        let held = e.mem.u32(slot);
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
        e.mem.set_u32(slot, 0);
        index += 1;
    }
}

// Translated from 00e2ec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::~hkMemoryResourceContainer` (Xbox PDB): installs the vtable
/// `0x010eb2c4`; for each of the two arrays (containers at `+0x1C`, then resources at `+0x10`)
/// drops the reference on every entry and clears it, sets the count to 0, gives the storage back
/// through the allocator's slot `0x10` unless the capacity word is negative and resets data and
/// capacity (`0x80000000`); then destroys the name at `+8` and leaves the base vtable.
pub fn hk_memory_resource_container_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MEMORY_CONTAINER_VTABLE);
    let allocator = heap_allocator(e);
    let count = e.mem.u32(this.addr() + 0x20) as i32;
    let data = e.mem.u32(this.addr() + 0x1c);
    release_pointers(e, data, count);
    let capacity = e.mem.u32(this.addr() + 0x24);
    e.mem.set_u32(this.addr() + 0x20, 0);
    let data = e.mem.u32(this.addr() + 0x1c);
    free_storage(e, allocator, data, capacity, 4);
    e.mem.set_u32(this.addr() + 0x1c, 0);
    e.mem.set_u32(this.addr() + 0x24, 0x8000_0000);
    let allocator = heap_allocator(e);
    let count = e.mem.u32(this.addr() + 0x14) as i32;
    let data = e.mem.u32(this.addr() + 0x10);
    release_pointers(e, data, count);
    let capacity = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u32(this.addr() + 0x14, 0);
    let data = e.mem.u32(this.addr() + 0x10);
    free_storage(e, allocator, data, capacity, 4);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x18, 0x8000_0000);
    e.call(STRING_PTR_DESTRUCT, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00e2ed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::createResource` (Xbox PDB), `(name, object, class)`: makes a
/// memory resource handle (a 0x1c-byte block from the allocator's slot 4 with its size word set,
/// constructed by `00e2e9e0`), names it (virtual slot `0x14`, `name`), gives it the object
/// (virtual slot `0x20`, `(object, class)`), takes a reference, appends it to the resources array
/// (`+0x10`, count `+0x14`, making room with `00c90940`; a reference for the slot when the slot
/// address is not null) and drops two references. Returns the handle.
pub fn hk_memory_resource_container_create_resource(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    object: u32,
    class: u32,
) -> Ptr {
    let allocator = heap_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    let handle = hk_memory_resource_handle_constructor(e, Ptr::new(block)).addr();
    e.vcall(handle, 0x14, &args![name]);
    e.vcall(handle, 0x20, &args![object, class]);
    e.call(ADD_REFERENCE, &args![handle]);
    let array = this.addr() + 0x10;
    grow_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    if slot != 0 {
        e.call(ADD_REFERENCE, &args![handle]);
        e.mem.set_u32(slot, handle);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(REMOVE_REFERENCE, &args![handle]);
    e.call(REMOVE_REFERENCE, &args![handle]);
    Ptr::new(handle)
}

/// Whether the name of `resource` (virtual slot `0x10` into a string buffer that starts in place
/// with 128 bytes, a one-byte empty string, from the router's second allocator) equals
/// `name`; the buffer is given back afterwards.
fn resource_name_matches(e: &mut Engine, resource: u32, name: u32) -> bool {
    e.with_stack(0x90, |e, frame| {
        let record = frame.addr();
        let in_place = record + 0x10;
        e.mem.set_u32(record, in_place);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, 0x8000_0080);
        let allocator = temporary_allocator(e);
        let capacity = (e.mem.u32(record + 8) & 0x3fff_ffff) as i32;
        if capacity < 1 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if doubled > 1 { doubled } else { 1 };
            e.call(ARRAY_RESERVE, &args![allocator, record, wanted, 1u32]);
        }
        e.mem.set_u32(record + 4, 1);
        let data = e.mem.u32(record);
        e.mem.set_u8(data, 0);
        e.mem.set_u8(data, 0);
        let text = e.vcall(resource, 0x10, &args![record]).u32();
        let equal = e.call(STRING_COMPARE, &args![name, text]).i32() == 0;
        let allocator = temporary_allocator(e);
        let capacity = e.mem.u32(record + 8);
        e.mem.set_u32(record + 4, 0);
        let data = e.mem.u32(record);
        free_storage(e, allocator, data, capacity, 1);
        equal
    })
}

// Translated from 00e2edb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::findResourceByName` (Xbox PDB), `(name, class, after)`: the
/// search starts after the resource `after` in the array at `+0x10` (count `+0x14`) when it
/// is not null and in the array, else at the first. A resource matches when `name` is null or
/// the name its virtual slot `0x10` writes into a 128-byte in-place string buffer (made with the
/// router's second allocator) equals `name` (`strCmp`), and `class` is null, equals the
/// resource's class (virtual slot `0x1C`) or is one of its ancestors (`00d46d80`). A
/// resource that matches by name but not by class reports "Class mismatch, cannot resolve
/// link: ..." (id `0xf034ed22`, line 0x138) and ends the search with 0 (when `name` is null it
/// is skipped instead). Returns the resource or 0.
pub fn hk_memory_resource_container_find_resource_by_name(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    class: u32,
    after: u32,
) -> u32 {
    let mut index: i32 = 0;
    while after != 0 && index < e.mem.u32(this.addr() + 0x14) as i32 {
        let slot = e
            .mem
            .u32(this.addr() + 0x10)
            .wrapping_add((index as u32).wrapping_mul(4));
        index += 1;
        if e.mem.u32(slot) == after {
            break;
        }
    }
    while index < e.mem.u32(this.addr() + 0x14) as i32 {
        let list = e.mem.u32(this.addr() + 0x10);
        let resource = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
        let named = name == 0 || resource_name_matches(e, resource, name);
        if named {
            if class == 0 {
                return resource;
            }
            let resource_class = e.vcall(resource, 0x1c, &args![]).u32();
            if class == resource_class {
                return resource;
            }
            if class_derives(e, class, resource_class) != 0 {
                return resource;
            }
            if name != 0 {
                report_error(
                    e,
                    |e| {
                        let resource_class_name = e.call(CLASS_NAME, &args![resource_class]).u32();
                        let class_name = e.call(CLASS_NAME, &args![class]).u32();
                        vec![
                            TEXT_CLASS_MISMATCH,
                            class_name,
                            TEXT_CLASS_MISMATCH_MIDDLE,
                            resource_class_name,
                        ]
                    },
                    0xf034_ed22,
                    0x138,
                );
                return 0;
            }
        }
        index += 1;
    }
    0
}

// Translated from 00e2efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryResourceContainer::createContainer` (Xbox PDB), `(name)`: returns the container
/// named `name` that virtual slot `0x34` (`findContainerByName(name, 0)`) finds, else makes one
/// (a 0x28-byte block from the allocator's slot 4 with its size word set, constructed by
/// `00e2eb70`), takes a reference, appends it to the containers array (`+0x1C`, count `+0x20`; a
/// reference for the slot when the slot address and the container are not null), drops
/// the extra reference, sets its parent (`+0xC`) to `this`, drops a reference again and
/// returns it.
pub fn hk_memory_resource_container_create_container(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let existing = e.vcall(this.addr(), 0x34, &args![name, 0u32]).u32();
    if existing != 0 {
        return existing;
    }
    let allocator = heap_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x28u32]).u32();
    e.mem.set_u16(block + 4, 0x28);
    let container = hk_memory_resource_container_constructor(e, Ptr::new(block), name).addr();
    if container != 0 {
        e.call(ADD_REFERENCE, &args![container]);
    }
    let array = this.addr() + 0x1c;
    grow_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    if slot != 0 {
        if container != 0 {
            e.call(ADD_REFERENCE, &args![container]);
        }
        e.mem.set_u32(slot, container);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    if container != 0 {
        e.call(REMOVE_REFERENCE, &args![container]);
    }
    e.mem.set_u32(container + 0xc, this.addr());
    e.call(REMOVE_REFERENCE, &args![container]);
    container
}

// Translated from 00e2f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkContainerResourceMap::_scalar_deleting_destructor_` (Xbox PDB), `(flags)`: runs the
/// destructor of the map at `this + 4` (`00d877d0`), installs the purecall vtable `0x010eb19c`
/// and, with bit 0 of `flags`, calls the operator `delete` (`00401030`) on `this`. Returns
/// `this`.
pub fn hk_container_resource_map_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(MAP_DESTRUCTOR, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), RESOURCE_MAP_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e2_f210, deleting_destructor_00e2f210(Ptr, u8) -> Ptr),
        entry!(0x00e2_f240, deleting_destructor_00e2f240(Ptr, u8) -> Ptr),
        entry!(0x00e2_def0, hk_resource_map_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_df10, hk_memory_resource_handle_set_name(Ptr, u32) -> u32),
        entry!(0x00e2_df20, hk_memory_resource_handle_get_class(Ptr) -> u32),
        entry!(
            0x00e2_df30,
            hk_memory_resource_handle_set_object(Ptr, u32, u32)
        ),
        entry!(0x00e2_df40, hk_memory_resource_handle_get_name(Ptr, u32) -> u32),
        entry!(0x00e2_df50, hk_memory_resource_handle_get_object(Ptr) -> u32),
        entry!(0x00e2_df60, hk_memory_resource_container_get_name(Ptr, u32) -> u32),
        entry!(0x00e2_df70, hk_memory_resource_container_get_num_containers(Ptr) -> u32),
        entry!(
            0x00e2_df80,
            hk_container_resource_map_find_object_by_name(Ptr, u32, Ptr) -> u32
        ),
        entry!(0x00e2_dfd0, hk_memory_resource_container_find_container_by_name(Ptr, u32, u32) -> u32),
        entry!(
            0x00e2_e040,
            hk_memory_resource_handle_add_external_link(Ptr, u32, u32)
        ),
        entry!(
            0x00e2_e0b0,
            hk_memory_resource_handle_get_external_links(Ptr, Ptr)
        ),
        entry!(0x00e2_e2e0, fn_00e2e2e0(Ptr, Ptr)),
        entry!(0x00e2_e380, fn_00e2e380(Ptr, Ptr)),
        entry!(
            0x00e2_e3f0,
            hk_memory_resource_handle_clear_external_links(Ptr)
        ),
        entry!(
            0x00e2_e430,
            hk_memory_resource_handle_remove_external_link(Ptr, u32)
        ),
        entry!(
            0x00e2_e4c0,
            hk_resource_handle_try_to_resolve_links(Ptr, Ptr)
        ),
        entry!(
            0x00e2_e670,
            hk_memory_resource_container_destroy_resource(Ptr, Ptr)
        ),
        entry!(
            0x00e2_e710,
            hk_memory_resource_container_destroy_container(Ptr, Ptr)
        ),
        entry!(0x00e2_e790, hk_memory_resource_container_parent_to(Ptr, Ptr) -> u32),
        entry!(
            0x00e2_e960,
            hk_resource_container_try_to_resolve_links(Ptr, Ptr)
        ),
        entry!(0x00e2_e9e0, hk_memory_resource_handle_constructor(Ptr) -> Ptr),
        entry!(0x00e2_ea20, fn_00e2ea20(Ptr, u32) -> Ptr),
        entry!(0x00e2_ea40, hk_memory_resource_handle_destructor(Ptr)),
        entry!(0x00e2_eb70, hk_memory_resource_container_constructor(Ptr, u32) -> Ptr),
        entry!(0x00e2_ebc0, fn_00e2ebc0(Ptr, u32) -> Ptr),
        entry!(0x00e2_ec00, hk_memory_resource_container_destructor(Ptr)),
        entry!(0x00e2_ed00, hk_memory_resource_container_create_resource(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00e2_edb0, hk_memory_resource_container_find_resource_by_name(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e2_efe0, hk_memory_resource_container_create_container(Ptr, u32) -> u32),
        entry!(0x00e2_f1e0, hk_container_resource_map_deleting_destructor(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e2f210_frees_only_when_bit_zero_is_set() {
        check(0x00e2_f210, 0x00e2_ea40);
    }

    #[test]
    fn deleting_destructor_00e2f240_frees_only_when_bit_zero_is_set() {
        check(0x00e2_f240, 0x00e2_ec00);
    }
}

#[cfg(test)]
mod handle_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0001;
    const FREE_BLOCK_SLOT: u32 = 0x0900_0002;
    const FREE_ARRAY_SLOT: u32 = 0x0900_0003;
    const TEMP_FREE_ARRAY_SLOT: u32 = 0x0900_0004;
    const LOGGER_SLOT: u32 = 0x0900_0005;

    struct World {
        e: Engine,
        log: Log,
    }

    /// A router (heap allocator at +0x10, temporary one at +0xC), the reference-count
    /// doubles, the string-stream doubles and the logger; everything logs into `log`.
    fn world() -> World {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            seen.borrow_mut().push(format!("allocate:{:x}", a[1]));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_BLOCK_SLOT, move |_, a| {
            seen.borrow_mut().push(format!("free-block:{:x}", a[2]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_ARRAY_SLOT, move |_, a| {
            seen.borrow_mut().push(format!("free-array:{:x}", a[2]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(TEMP_FREE_ARRAY_SLOT, move |_, a| {
            seen.borrow_mut().push(format!("temp-free:{:x}", a[2]));
            Ret::default()
        });
        let heap_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(heap_vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(heap_vtable + 8, FREE_BLOCK_SLOT);
        e.mem.set_u32(heap_vtable + 0x10, FREE_ARRAY_SLOT);
        let heap = e.mem.alloc(0x10);
        e.mem.set_u32(heap, heap_vtable);
        let temp_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(temp_vtable + 0x10, TEMP_FREE_ARRAY_SLOT);
        let temp = e.mem.alloc(0x10);
        e.mem.set_u32(temp, temp_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.mem.set_u32(router + 0xc, temp);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ADD_REFERENCE, move |_, a| {
            seen.borrow_mut().push(format!("add:{:x}", a[0]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            seen.borrow_mut().push(format!("remove:{:x}", a[0]));
            Ret::default()
        });
        // The reserve-more double grows an array to twice its capacity (at least 4 elements).
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            seen.borrow_mut().push(format!("more:{:x}", a[2]));
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff).max(2) * 2;
            let size = e.mem.u32(a[1] + 4);
            let block = e.mem.alloc(capacity * a[2]);
            let old = e.mem.u32(a[1]);
            if old != 0 {
                let bytes = e.mem.bytes(old, size * a[2]);
                e.mem.write(block, &bytes);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
        // The string pointer records: `record` holds the text pointer.
        let seen = Rc::clone(&log);
        e.register_double(STRING_PTR_CONSTRUCT, move |e, a| {
            seen.borrow_mut().push(format!("string-new:{:x}", a[0]));
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(STRING_PTR_ASSIGN, move |e, a| {
            seen.borrow_mut()
                .push(format!("string-assign:{:x}:{:x}", a[0], a[1]));
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STRING_PTR_DESTRUCT, move |_, a| {
            seen.borrow_mut().push(format!("string-drop:{:x}", a[0]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(STRING_PTR_CONSTRUCT_FROM_TEXT, move |e, a| {
            seen.borrow_mut()
                .push(format!("string-text:{:x}:{:x}", a[0], a[1]));
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(STRING_PTR_CONSTRUCT_COPY, move |e, a| {
            seen.borrow_mut()
                .push(format!("string-copy:{:x}:{:x}", a[0], a[1]));
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // The error stream pieces and the logger.
        let seen = Rc::clone(&log);
        e.register_double(STREAM_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut().push("stream".to_string());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STREAM_APPEND, move |e, a| {
            let text = if a[1] < 0x2000_0000 {
                format!("<{:x}>", a[1])
            } else {
                String::from_utf8_lossy(&e.mem.cstr(a[1])).into_owned()
            };
            seen.borrow_mut().push(format!("text:{text}"));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STREAM_APPEND_STRING_PTR, move |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(e.mem.u32(a[1]))).into_owned();
            seen.borrow_mut().push(format!("name:{text}"));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STREAM_DESTRUCTOR, move |_, _| {
            seen.borrow_mut().push("stream-end".to_string());
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(LOGGER_SLOT, move |_, a| {
            seen.borrow_mut()
                .push(format!("report:{:x}:{:x}:{:x}", a[1], a[2], a[5]));
            Ret::default()
        });
        let logger_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(logger_vtable + 0xc, LOGGER_SLOT);
        let logger = e.mem.alloc(0x10);
        e.mem.set_u32(logger, logger_vtable);
        e.mem.set_u32(LOGGER_GLOBAL, logger);
        World { e, log }
    }

    /// An object whose virtual slots are the given `(slot, double address)` pairs.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A pointer array record `(data, size, capacity)` holding `items`; returns the record's
    /// address.
    fn pointers(e: &mut Engine, items: &[u32], capacity: u32) -> u32 {
        let data = e.mem.alloc(4 * capacity.max(1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, data);
        e.mem.set_u32(record + 4, items.len() as u32);
        e.mem.set_u32(record + 8, capacity);
        record
    }

    fn text(e: &mut Engine, bytes: &[u8]) -> u32 {
        let block = e.mem.alloc(bytes.len() as u32 + 1);
        e.mem.set_cstr(block, bytes);
        block
    }

    fn strings(log: &Log) -> Vec<String> {
        log.borrow().clone()
    }

    /// The next entry after `previous` (the first when `previous` is 0) of the pointer array
    /// record at `record`.
    fn next_in(e: &Engine, record: u32, previous: u32) -> u32 {
        let list = e.mem.u32(record);
        let count = e.mem.u32(record + 4);
        for i in 0..count {
            let item = e.mem.u32(list + 4 * i);
            if previous == 0 {
                return item;
            }
            if item == previous && i + 1 < count {
                return e.mem.u32(list + 4 * (i + 1));
            }
        }
        0
    }

    /// `findContainerByName(this, name, previous)` over the children record at `this + 0x30`.
    fn find_container_double(e: &mut Engine, a: &[u32]) -> Ret {
        let children = e.mem.u32(a[0] + 0x30);
        Ret {
            eax: next_in(e, children, a[2]),
            ..Ret::default()
        }
    }

    /// `findResourceByName(this, name, class, previous)` over the resources record at
    /// `this + 0x38`.
    fn find_resource_double(e: &mut Engine, a: &[u32]) -> Ret {
        let resources = e.mem.u32(a[0] + 0x38);
        Ret {
            eax: next_in(e, resources, a[3]),
            ..Ret::default()
        }
    }

    #[test]
    fn deleting_destructors_install_the_purecall_vtable() {
        let mut w = world();
        w.e.register_double(OPERATOR_DELETE, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut().push(format!("delete:{:x}", a[0]));
                Ret::default()
            }
        });
        w.e.register_double(MAP_DESTRUCTOR, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut().push(format!("map-drop:{:x}", a[0]));
                Ret::default()
            }
        });
        let this = w.e.mem.alloc(0x20);
        assert_eq!(w.e.call(0x00e2_def0, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(w.e.mem.u32(this), RESOURCE_MAP_VTABLE);
        assert!(strings(&w.log).is_empty());
        w.e.call(0x00e2_def0, &args![this, 1u32]);
        assert_eq!(strings(&w.log), vec![format!("delete:{this:x}")]);
        w.log.borrow_mut().clear();
        w.e.call(0x00e2_f1e0, &args![this, 1u32]);
        assert_eq!(
            strings(&w.log),
            vec![
                format!("map-drop:{:x}", this + 4),
                format!("delete:{this:x}")
            ]
        );
    }

    #[test]
    fn handle_accessors_and_tail_calls() {
        let mut w = world();
        let this = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(this + 8, 0x1234);
        w.e.mem.set_u32(this + 0xc, 0);
        // Empty name: the placeholder.
        assert_eq!(w.e.call(0x00e2_df40, &args![this, 0u32]).u32(), EMPTY_NAME);
        w.e.mem.set_u32(this + 0xc, 0x5001);
        assert_eq!(w.e.call(0x00e2_df40, &args![this, 0u32]).u32(), 0x5000);
        assert_eq!(w.e.call(0x00e2_df50, &args![this]).u32(), 0x1234);
        // setName assigns the string at +0xC and returns the assignment's result.
        assert_eq!(
            w.e.call(0x00e2_df10, &args![this, 0x7700u32]).u32(),
            this + 0xc
        );
        assert_eq!(w.e.mem.u32(this + 0xc), 0x7700);
        // getClass and setObject forward to the variant at +8.
        w.e.register(REF_VARIANT_GET_CLASS, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(w.e.call(0x00e2_df20, &args![this]).u32(), this + 9);
        w.e.register_double(REF_VARIANT_SET, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut()
                    .push(format!("set:{:x}:{:x}:{:x}", a[0], a[1], a[2]));
                Ret::default()
            }
        });
        w.e.call(0x00e2_df30, &args![this, 0xaau32, 0xbbu32]);
        assert!(strings(&w.log).contains(&format!("set:{:x}:aa:bb", this + 8)));
    }

    #[test]
    fn container_accessors() {
        let mut w = world();
        let this = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(this + 8, 0x9003);
        w.e.mem.set_u32(this + 0x20, 4);
        assert_eq!(w.e.call(0x00e2_df60, &args![this, 0u32]).u32(), 0x9002);
        assert_eq!(w.e.call(0x00e2_df70, &args![this]).u32(), 4);
    }

    #[test]
    fn find_object_by_name_clears_then_fills_the_class() {
        let mut w = world();
        w.e.register(MAP_GET_WITH_DEFAULT, |e, a| {
            assert_eq!(a[2], 0);
            Ret {
                eax: e.mem.u32(a[1]),
                ..Ret::default()
            }
        });
        w.e.register(0x0900_0010, |_, _| Ret {
            eax: 0xc1a5,
            ..Ret::default()
        });
        w.e.register_double(0x0900_0011, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut().push(format!("hold:{:x}", a[0]));
                Ret::default()
            }
        });
        let found = object(&mut w.e, &[(0x1c, 0x0900_0010), (0x18, 0x0900_0011)]);
        let this = w.e.mem.alloc(0x20);
        let key = w.e.mem.alloc(4);
        w.e.mem.set_u32(key, found);
        let out = w.e.mem.alloc(4);
        w.e.mem.set_u32(out, 0xdead);
        w.e.call(0x00e2_df80, &args![this, key, out]);
        assert_eq!(w.e.mem.u32(out), 0xc1a5);
        assert_eq!(strings(&w.log), vec![format!("hold:{found:x}")]);
        // A missing object leaves the class cleared and does not call anything.
        w.log.borrow_mut().clear();
        w.e.mem.set_u32(key, 0);
        w.e.call(0x00e2_df80, &args![this, key, out]);
        assert_eq!(w.e.mem.u32(out), 0);
        assert!(strings(&w.log).is_empty());
        // A null out pointer is allowed.
        w.e.mem.set_u32(key, found);
        w.e.call(0x00e2_df80, &args![this, key, 0u32]);
        assert_eq!(strings(&w.log), vec![format!("hold:{found:x}")]);
    }

    #[test]
    fn find_object_by_name_answers_the_object_pointer() {
        let mut w = world();
        w.e.register(MAP_GET_WITH_DEFAULT, |e, a| e.mem.u32(a[1]).into_ret());
        w.e.register(0x0900_0010, |_, _| 0xc1a5u32.into_ret());
        w.e.register(0x0900_0011, |_, _| 0x0bad_0b1eu32.into_ret());
        let found = object(&mut w.e, &[(0x1c, 0x0900_0010), (0x18, 0x0900_0011)]);
        let this = w.e.mem.alloc(0x20);
        let key = w.e.mem.alloc(4);
        w.e.mem.set_u32(key, found);
        let answer = w.e.call(0x00e2_df80, &args![this, key, 0u32]).u32();
        assert_eq!(answer, 0x0bad_0b1e);
        w.e.mem.set_u32(key, 0);
        assert_eq!(w.e.call(0x00e2_df80, &args![this, key, 0u32]).u32(), 0);
    }

    #[test]
    fn find_container_by_name_searches_after_the_given_container() {
        let mut w = world();
        w.e.register(STRING_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: u32::from(x != y),
                ..Ret::default()
            }
        });
        let mut containers = Vec::new();
        for name in [&b"a"[..], b"b", b"a"] {
            let container = w.e.mem.alloc(0x20);
            let name = text(&mut w.e, name);
            w.e.mem.set_u32(container + 8, name | 1);
            containers.push(container);
        }
        let record = pointers(&mut w.e, &containers, 3);
        let this = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(this + 0x1c, w.e.mem.u32(record));
        w.e.mem.set_u32(this + 0x20, 3);
        let a = text(&mut w.e, b"a");
        let found = |w: &mut World, name: u32, after: u32| {
            w.e.call(0x00e2_dfd0, &args![this, name, after]).u32()
        };
        assert_eq!(found(&mut w, a, 0), containers[0]);
        assert_eq!(found(&mut w, a, containers[0]), containers[2]);
        assert_eq!(found(&mut w, a, containers[2]), 0);
        // No name: the first after `after`.
        assert_eq!(found(&mut w, 0, containers[0]), containers[1]);
        // An unknown `after` starts from the beginning: after an unknown container the
        // index stays at the end.
        assert_eq!(found(&mut w, a, 0x1111), 0);
    }

    #[test]
    fn add_external_link_constructs_and_assigns_two_strings() {
        let mut w = world();
        let this = w.e.mem.alloc(0x40);
        // Full array of one: room is made first.
        let data = w.e.mem.alloc(8);
        w.e.mem.set_u32(this + 0x10, data);
        w.e.mem.set_u32(this + 0x14, 1);
        w.e.mem.set_u32(this + 0x18, 1);
        w.e.call(0x00e2_e040, &args![this, 0xa0u32, 0xb0u32]);
        let new_data = w.e.mem.u32(this + 0x10);
        assert_eq!(w.e.mem.u32(this + 0x14), 2);
        assert_eq!(
            strings(&w.log),
            vec![
                "more:8".to_string(),
                format!("string-new:{:x}", new_data + 8),
                format!("string-new:{:x}", new_data + 12),
                format!("string-assign:{:x}:a0", new_data + 8),
                format!("string-assign:{:x}:b0", new_data + 12),
            ]
        );
        assert_eq!(w.e.mem.u32(new_data + 8), 0xa0);
        assert_eq!(w.e.mem.u32(new_data + 12), 0xb0);
    }

    #[test]
    fn clear_and_remove_external_links() {
        let mut w = world();
        w.e.register(STRING_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: u32::from(x != y),
                ..Ret::default()
            }
        });
        let this = w.e.mem.alloc(0x40);
        let data = w.e.mem.alloc(0x18);
        let names: Vec<u32> = [&b"one"[..], b"two", b"three"]
            .iter()
            .map(|n| text(&mut w.e, n))
            .collect();
        for (i, name) in names.iter().enumerate() {
            w.e.mem.set_u32(data + 8 * i as u32, *name | 1);
            w.e.mem.set_u32(data + 8 * i as u32 + 4, 0x100 + i as u32);
        }
        w.e.mem.set_u32(this + 0x10, data);
        w.e.mem.set_u32(this + 0x14, 3);
        // Removing the first link moves the last one into its place.
        let one = text(&mut w.e, b"one");
        w.e.call(0x00e2_e430, &args![this, one]);
        assert_eq!(w.e.mem.u32(this + 0x14), 2);
        assert_eq!(w.e.mem.u32(data), names[2] | 1);
        assert_eq!(w.e.mem.u32(data + 4), 0x102);
        assert_eq!(
            strings(&w.log),
            vec![
                format!("string-drop:{:x}", data + 4),
                format!("string-drop:{data:x}")
            ]
        );
        // An unknown name changes nothing.
        w.log.borrow_mut().clear();
        let unknown = text(&mut w.e, b"zzz");
        w.e.call(0x00e2_e430, &args![this, unknown]);
        assert_eq!(w.e.mem.u32(this + 0x14), 2);
        assert!(strings(&w.log).is_empty());
        // Removing the last one just drops it.
        let two = text(&mut w.e, b"two");
        w.e.call(0x00e2_e430, &args![this, two]);
        assert_eq!(w.e.mem.u32(this + 0x14), 1);
        // Clearing drops both strings of each remaining link, second field first.
        w.log.borrow_mut().clear();
        w.e.call(0x00e2_e3f0, &args![this]);
        assert_eq!(w.e.mem.u32(this + 0x14), 0);
        assert_eq!(
            strings(&w.log),
            vec![
                format!("string-drop:{:x}", data + 4),
                format!("string-drop:{data:x}")
            ]
        );
    }

    #[test]
    fn external_links_split_the_path_at_dots() {
        let mut w = world();
        // Classes: `outer` has a struct member `inner` (class `inner_class`, offset 0x10)
        // and `inner_class` has a member `value` (offset 4).
        let outer = w.e.mem.alloc(0x10);
        let inner_class = w.e.mem.alloc(0x10);
        let inner_member = w.e.mem.alloc(0x20);
        w.e.mem.set_u8(inner_member + 0xc, 0x19);
        w.e.mem.set_u16(inner_member + 0x12, 0x10);
        w.e.mem.set_u32(inner_member + 4, inner_class);
        let value_member = w.e.mem.alloc(0x20);
        w.e.mem.set_u8(value_member + 0xc, 1);
        w.e.mem.set_u16(value_member + 0x12, 4);
        let plain_member = w.e.mem.alloc(0x20);
        w.e.mem.set_u8(plain_member + 0xc, 1);
        w.e.register_double(REF_VARIANT_GET_CLASS, move |_, _| Ret {
            eax: outer,
            ..Ret::default()
        });
        w.e.register_double(CLASS_MEMBER_BY_NAME, move |e, a| {
            let name = String::from_utf8_lossy(&e.mem.cstr(a[1])).into_owned();
            let eax = match (a[0] == outer, a[0] == inner_class, name.as_str()) {
                (true, _, "inner") => inner_member,
                (true, _, "plain") => plain_member,
                (_, true, "value") => value_member,
                _ => 0,
            };
            Ret {
                eax,
                ..Ret::default()
            }
        });
        w.e.register(MEMBER_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        // The accessor constructor stores (object, member) in its record and returns it.
        w.e.register(ACCESSOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        w.e.register(STRING_FIND_CHAR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            let found = bytes.iter().position(|b| u32::from(*b) == a[1]);
            Ret {
                eax: found.map_or(0, |i| a[0] + i as u32),
                ..Ret::default()
            }
        });
        w.e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // The handle: object at +8, three links.
        let this = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(this + 8, 0x4000);
        let data = w.e.mem.alloc(0x18);
        let paths = [&b"inner.value"[..], b"plain", b"missing.x"];
        let targets = [&b"t0"[..], b"t1", b"t2"];
        for i in 0..3 {
            let path = text(&mut w.e, paths[i]);
            let target = text(&mut w.e, targets[i]);
            w.e.mem.set_u32(data + 8 * i as u32, path | 1);
            w.e.mem.set_u32(data + 8 * i as u32 + 4, target | 1);
        }
        w.e.mem.set_u32(this + 0x10, data);
        w.e.mem.set_u32(this + 0x14, 3);
        let out = pointers(&mut w.e, &[], 4);
        let out_data = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(out, out_data);
        w.e.mem.set_u32(out + 4, 9);
        w.e.call(0x00e2_e0b0, &args![this, out]);
        // Two records are left: the third link was dropped.
        assert_eq!(w.e.mem.u32(out + 4), 2);
        let first_path = w.e.mem.u32(data) & !1;
        // "inner.value": the component after the dot is the member name.
        let record0_path = w.e.mem.u32(out_data);
        assert_eq!(
            String::from_utf8_lossy(&w.e.mem.cstr(record0_path)),
            "value"
        );
        assert_eq!(record0_path, first_path + 6);
        assert_eq!(w.e.mem.u32(out_data + 4), w.e.mem.u32(data + 4) & !1);
        // The accessor was made on object + the struct member's offset.
        assert_eq!(w.e.mem.u32(out_data + 8), 0x4010);
        assert_eq!(w.e.mem.u32(out_data + 0xc), value_member);
        // "plain": the whole path.
        assert_eq!(w.e.mem.u32(out_data + 0x10), w.e.mem.u32(data + 8) & !1);
        assert_eq!(w.e.mem.u32(out_data + 0x18), 0x4000);
        assert_eq!(w.e.mem.u32(out_data + 0x1c), plain_member);
        // The third (first component unknown) reports once.
        let log = strings(&w.log);
        assert!(log.contains(&"text:x".to_string()));
        assert!(log.contains(&"report:1:f032edf1:92".to_string()));
    }

    #[test]
    fn collectors_walk_the_container_tree() {
        let mut w = world();
        // A container tree: root has children c1 and c2 (found by slot 0x34 with
        // (name, previous)) and resource r_root; c1 has resource r1.
        w.e.register(0x0900_0020, find_container_double);
        w.e.register(0x0900_0021, find_resource_double);
        let slots = [(0x34, 0x0900_0020), (0x20, 0x0900_0021)];
        let c1 = object(&mut w.e, &slots);
        let c2 = object(&mut w.e, &slots);
        let root = object(&mut w.e, &slots);
        let kids = pointers(&mut w.e, &[c1, c2], 2);
        let none = pointers(&mut w.e, &[], 1);
        w.e.mem.set_u32(root + 0x30, kids);
        w.e.mem.set_u32(c1 + 0x30, none);
        w.e.mem.set_u32(c2 + 0x30, none);
        let resources = pointers(&mut w.e, &[0xe1, 0xe2], 2);
        let r1 = pointers(&mut w.e, &[0xe3], 1);
        let empty = pointers(&mut w.e, &[], 1);
        w.e.mem.set_u32(root + 0x38, resources);
        w.e.mem.set_u32(c1 + 0x38, r1);
        w.e.mem.set_u32(c2 + 0x38, empty);
        // Resources: children first, then the container's own.
        let array = pointers(&mut w.e, &[], 8);
        w.e.call(0x00e2_e2e0, &args![root, array]);
        let list = w.e.mem.u32(array);
        let found: Vec<u32> = (0..w.e.mem.u32(array + 4))
            .map(|i| w.e.mem.u32(list + 4 * i))
            .collect();
        assert_eq!(found, vec![0xe3, 0xe1, 0xe2]);
        // Containers: the container itself, then the ones below it.
        let array = pointers(&mut w.e, &[], 8);
        w.e.call(0x00e2_e380, &args![root, array]);
        let list = w.e.mem.u32(array);
        let found: Vec<u32> = (0..w.e.mem.u32(array + 4))
            .map(|i| w.e.mem.u32(list + 4 * i))
            .collect();
        assert_eq!(found, vec![root, c1, c2]);
        // A full array makes room first.
        let full = pointers(&mut w.e, &[7], 1);
        w.e.call(0x00e2_e380, &args![c1, full]);
        assert!(strings(&w.log).contains(&"more:4".to_string()));
        assert_eq!(w.e.mem.u32(full + 4), 2);
    }

    const GET_OBJECT_SLOT: u32 = 0x0900_0040;
    const REMOVE_LINK_SLOT: u32 = 0x0900_0041;
    const GET_LINKS_SLOT: u32 = 0x0900_0042;
    const FIND_OBJECT_SLOT: u32 = 0x0900_0043;
    const GET_PARENT_SLOT: u32 = 0x0900_0044;
    const RESOLVE_SLOT: u32 = 0x0900_0045;
    const SET_NAME_SLOT: u32 = 0x0900_0046;
    const SET_OBJECT_SLOT: u32 = 0x0900_0047;
    const GET_NAME_SLOT: u32 = 0x0900_0048;
    const GET_CLASS_SLOT: u32 = 0x0900_0049;
    const FIND_CONTAINER_SLOT: u32 = 0x0900_004a;

    /// Doubles for the class pieces `tryToResolveLinks` uses: the accessor's member is the
    /// word at `+4`, a member's class the word at `+4`, a class' name the word at `+0`, and a
    /// class derives from another when it is that class or its parent word (`+4`) is.
    fn install_class_doubles(e: &mut Engine) {
        e.register(ACCESSOR_CLASS_MEMBER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(MEMBER_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(CLASS_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CLASS_CHAIN_CONTAINS, |e, a| {
            let found = a[2] == a[0] || e.mem.u32(a[2] + 4) == a[0];
            e.mem.set_u8(a[1], u8::from(found));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(ACCESSOR_AS_POINTER, |e, a| Ret {
            eax: e.mem.u32(a[0]) + 4 * a[1],
            ..Ret::default()
        });
    }

    /// A handle for `tryToResolveLinks` whose own object is `own`; its links (two records of
    /// 0x10 bytes: path, target name, accessor object, accessor member) are written by the
    /// virtual slot `0x2C` double.
    fn resolving_handle(w: &mut World, own: u32, links: [[u32; 4]; 2]) -> u32 {
        w.e.register_double(GET_OBJECT_SLOT, move |_, _| Ret {
            eax: own,
            ..Ret::default()
        });
        w.e.register_double(REMOVE_LINK_SLOT, {
            let seen = Rc::clone(&w.log);
            move |e, a| {
                let name = String::from_utf8_lossy(&e.mem.cstr(a[1])).into_owned();
                seen.borrow_mut().push(format!("unlink:{name}"));
                Ret::default()
            }
        });
        w.e.register_double(GET_LINKS_SLOT, move |e, a| {
            let data = e.mem.alloc(0x20);
            for (i, record) in links.iter().enumerate() {
                for (word, value) in record.iter().enumerate() {
                    e.mem
                        .set_u32(data + 0x10 * i as u32 + 4 * word as u32, *value);
                }
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 2);
            Ret::default()
        });
        object(
            &mut w.e,
            &[
                (0x18, GET_OBJECT_SLOT),
                (0x28, REMOVE_LINK_SLOT),
                (0x2c, GET_LINKS_SLOT),
            ],
        )
    }

    /// A link target whose name is its address, found by a resolver double that returns
    /// the object and class kept at +0 / +4 of the target name record.
    fn resolver(w: &mut World) -> u32 {
        w.e.register(FIND_OBJECT_SLOT, |e, a| {
            if a[1] == 0 {
                return Ret::default();
            }
            e.mem.set_u32(a[2], e.mem.u32(a[1] + 4));
            Ret {
                eax: e.mem.u32(a[1]),
                ..Ret::default()
            }
        });
        object(&mut w.e, &[(0, FIND_OBJECT_SLOT)])
    }

    #[test]
    fn try_to_resolve_links_stores_targets_and_stops_at_its_own_object() {
        let mut w = world();
        install_class_doubles(&mut w.e);
        let base_class = w.e.mem.alloc(8);
        let derived_class = w.e.mem.alloc(8);
        w.e.mem.set_u32(derived_class + 4, base_class);
        // Two members whose class is `base_class`, each with a pointer cell the accessor
        // data points at; the member's first word is its name.
        let mut members = Vec::new();
        let mut cells = Vec::new();
        for name in [&b"first"[..], b"second"] {
            let member = w.e.mem.alloc(8);
            let name = text(&mut w.e, name);
            w.e.mem.set_u32(member, name);
            w.e.mem.set_u32(member + 4, base_class);
            let cell = w.e.mem.alloc(8);
            members.push(member);
            cells.push(cell);
        }
        // Targets: a record per name with the object and the class it is found with.
        let target = |w: &mut World, object: u32, class: u32| {
            let record = w.e.mem.alloc(8);
            w.e.mem.set_u32(record, object);
            w.e.mem.set_u32(record + 4, class);
            record
        };
        let own = w.e.mem.alloc(8);
        let other = w.e.mem.alloc(8);
        let target_a = target(&mut w, other, derived_class);
        let target_b = target(&mut w, own, base_class);
        let handle = resolving_handle(
            &mut w,
            own,
            [
                [0, target_b, cells[0], members[0]],
                [0, target_a, cells[1], members[1]],
            ],
        );
        let resolver = resolver(&mut w);
        w.e.call(0x00e2_e4c0, &args![handle, resolver]);
        // The last link is handled first: its target derives from the member's class, so
        // it is stored and unlinked; the first link's target is the handle's own object,
        // which stops the run without storing.
        assert_eq!(w.e.mem.u32(cells[1]), other);
        assert_eq!(w.e.mem.u32(cells[0]), 0);
        assert_eq!(
            strings(&w.log),
            vec!["unlink:second".to_string(), "free-array:20".to_string()]
        );
    }

    #[test]
    fn try_to_resolve_links_reports_a_class_mismatch_and_goes_on() {
        let mut w = world();
        install_class_doubles(&mut w.e);
        let member_class = w.e.mem.alloc(8);
        let other_class = w.e.mem.alloc(8);
        let name_a = text(&mut w.e, b"MemberClass");
        let name_b = text(&mut w.e, b"OtherClass");
        w.e.mem.set_u32(member_class, name_a);
        w.e.mem.set_u32(other_class, name_b);
        let member = w.e.mem.alloc(8);
        let member_name = text(&mut w.e, b"m");
        w.e.mem.set_u32(member, member_name);
        w.e.mem.set_u32(member + 4, member_class);
        let cell = w.e.mem.alloc(8);
        let found = w.e.mem.alloc(8);
        let record = w.e.mem.alloc(8);
        w.e.mem.set_u32(record, found);
        w.e.mem.set_u32(record + 4, other_class);
        let own = w.e.mem.alloc(8);
        let handle = resolving_handle(
            &mut w,
            own,
            [[0, record, cell, member], [0, 0, cell, member]],
        );
        let resolver = resolver(&mut w);
        w.e.call(0x00e2_e4c0, &args![handle, resolver]);
        assert_eq!(w.e.mem.u32(cell), 0);
        assert_eq!(
            strings(&w.log),
            vec![
                "stream".to_string(),
                format!("text:<{TEXT_CLASS_MISMATCH:x}>"),
                "text:OtherClass".to_string(),
                format!("text:<{TEXT_CLASS_MISMATCH_MIDDLE:x}>"),
                "text:MemberClass".to_string(),
                "report:1:f034ed21:b2".to_string(),
                "stream-end".to_string(),
                "free-array:20".to_string(),
            ]
        );
    }

    #[test]
    fn destroy_resource_and_container_remove_one_entry() {
        let mut w = world();
        let this = w.e.mem.alloc(0x40);
        let resources = pointers(&mut w.e, &[0x1000, 0x2000, 0x3000], 4);
        w.e.mem.set_u32(this + 0x10, w.e.mem.u32(resources));
        w.e.mem.set_u32(this + 0x14, 3);
        w.e.call(0x00e2_e670, &args![this, 0x2000u32]);
        let data = w.e.mem.u32(this + 0x10);
        assert_eq!(w.e.mem.u32(this + 0x14), 2);
        assert_eq!((w.e.mem.u32(data), w.e.mem.u32(data + 4)), (0x1000, 0x3000));
        assert_eq!(
            strings(&w.log),
            vec![
                "add:2000".to_string(),
                "remove:2000".to_string(),
                "remove:2000".to_string()
            ]
        );
        // An unknown resource only takes and drops its temporary reference.
        w.log.borrow_mut().clear();
        w.e.call(0x00e2_e670, &args![this, 0x9000u32]);
        assert_eq!(w.e.mem.u32(this + 0x14), 2);
        assert_eq!(
            strings(&w.log),
            vec!["add:9000".to_string(), "remove:9000".to_string()]
        );
        // Containers: the last entry moves into the removed slot.
        w.log.borrow_mut().clear();
        let containers = pointers(&mut w.e, &[0xa000, 0xb000, 0xc000], 4);
        w.e.mem.set_u32(this + 0x1c, w.e.mem.u32(containers));
        w.e.mem.set_u32(this + 0x20, 3);
        w.e.call(0x00e2_e710, &args![this, 0xa000u32]);
        let data = w.e.mem.u32(this + 0x1c);
        assert_eq!(w.e.mem.u32(this + 0x20), 2);
        assert_eq!((w.e.mem.u32(data), w.e.mem.u32(data + 4)), (0xc000, 0xb000));
        assert_eq!(
            strings(&w.log),
            vec![
                "add:a000".to_string(),
                "remove:a000".to_string(),
                "remove:a000".to_string()
            ]
        );
    }

    #[test]
    fn parent_to_moves_a_container_between_parents() {
        let mut w = world();
        w.e.register(GET_PARENT_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let slots = [(0x24, GET_PARENT_SLOT)];
        let me = object(&mut w.e, &slots);
        let old_parent = object(&mut w.e, &slots);
        let new_parent = object(&mut w.e, &slots);
        let old_children = pointers(&mut w.e, &[0x7000, me, 0x8000], 4);
        w.e.mem
            .set_u32(old_parent + 0x1c, w.e.mem.u32(old_children));
        w.e.mem.set_u32(old_parent + 0x20, 3);
        w.e.mem.set_u32(me + 0xc, old_parent);
        let got = w.e.call(0x00e2_e790, &args![me, new_parent]).u32();
        assert_eq!(got, 0);
        // Removed from the old parent (later entries shift down), added to the new one.
        let data = w.e.mem.u32(old_parent + 0x1c);
        assert_eq!(w.e.mem.u32(old_parent + 0x20), 2);
        assert_eq!((w.e.mem.u32(data), w.e.mem.u32(data + 4)), (0x7000, 0x8000));
        let new_data = w.e.mem.u32(new_parent + 0x1c);
        assert_eq!(w.e.mem.u32(new_parent + 0x20), 1);
        assert_eq!(w.e.mem.u32(new_data), me);
        assert_eq!(w.e.mem.u32(me + 0xc), new_parent);
        let me_hex = format!("{me:x}");
        let expected: Vec<String> = [
            "add", "add", "remove", "remove", "add", "more", "add", "remove", "remove",
        ]
        .iter()
        .map(|step| {
            if *step == "more" {
                "more:4".to_string()
            } else {
                format!("{step}:{me_hex}")
            }
        })
        .collect();
        // The slot of the old parent held `me` too, so its own reference is dropped second.
        assert_eq!(strings(&w.log), expected);
    }

    #[test]
    fn parent_to_refuses_a_circular_dependency() {
        let mut w = world();
        w.e.register(GET_PARENT_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let slots = [(0x24, GET_PARENT_SLOT)];
        let me = object(&mut w.e, &slots);
        let child = object(&mut w.e, &slots);
        // `child`'s parent is `me`, so `me` cannot be parented to `child`.
        w.e.mem.set_u32(child + 0xc, me);
        let me_name = text(&mut w.e, b"me");
        let child_name = text(&mut w.e, b"child");
        w.e.mem.set_u32(me + 8, me_name);
        w.e.mem.set_u32(child + 8, child_name);
        let got = w.e.call(0x00e2_e790, &args![me, child]).u32();
        assert_eq!(got, 1);
        assert_eq!(
            strings(&w.log),
            vec![
                "stream".to_string(),
                format!("text:<{TEXT_CANNOT_PARENT:x}>"),
                "name:me".to_string(),
                format!("text:<{TEXT_PARENT_TO:x}>"),
                "name:child".to_string(),
                format!("text:<{TEXT_CIRCULAR:x}>"),
                "report:1:abba4554:198".to_string(),
                "stream-end".to_string(),
            ]
        );
    }

    #[test]
    fn container_try_to_resolve_links_visits_every_resource() {
        let mut w = world();
        w.e.register(0x0900_0020, find_container_double);
        w.e.register(0x0900_0021, find_resource_double);
        w.e.register_double(RESOLVE_SLOT, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut().push(format!("resolve:{:x}", a[1]));
                Ret::default()
            }
        });
        let root = object(&mut w.e, &[(0x34, 0x0900_0020), (0x20, 0x0900_0021)]);
        let first = object(&mut w.e, &[(0x34, RESOLVE_SLOT)]);
        let second = object(&mut w.e, &[(0x34, RESOLVE_SLOT)]);
        let none = pointers(&mut w.e, &[], 1);
        let resources = pointers(&mut w.e, &[first, second], 2);
        w.e.mem.set_u32(root + 0x30, none);
        w.e.mem.set_u32(root + 0x38, resources);
        w.e.call(0x00e2_e960, &args![root, 0x5555u32]);
        assert_eq!(
            strings(&w.log),
            vec![
                "more:4".to_string(),
                "resolve:5555".to_string(),
                "resolve:5555".to_string(),
                "free-array:10".to_string()
            ]
        );
    }

    #[test]
    fn handle_constructors_and_destructor() {
        let mut w = world();
        let this = w.e.mem.alloc(0x40);
        // The plain constructor.
        assert_eq!(w.e.call(0x00e2_e9e0, &args![this]).u32(), this);
        assert_eq!(w.e.mem.u16(this + 6), 1);
        assert_eq!(w.e.mem.u32(this), MEMORY_HANDLE_VTABLE);
        assert_eq!(w.e.mem.u32(this + 0x18), 0x8000_0000);
        assert_eq!(
            strings(&w.log),
            vec![format!("string-new:{:x}", this + 0xc)]
        );
        // The variant that builds the name from text.
        w.log.borrow_mut().clear();
        let other = w.e.mem.alloc(0x40);
        assert_eq!(w.e.call(0x00e2_ea20, &args![other, 0xabcdu32]).u32(), other);
        assert_eq!(
            strings(&w.log),
            vec![format!("string-text:{:x}:abcd", other + 0xc)]
        );
        // The destructor: two links, owned storage, an object.
        w.log.borrow_mut().clear();
        let data = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(this + 8, 0x6000);
        w.e.mem.set_u32(this + 0x10, data);
        w.e.mem.set_u32(this + 0x14, 2);
        w.e.mem.set_u32(this + 0x18, 2);
        w.e.call(0x00e2_ea40, &args![this]);
        assert_eq!(
            strings(&w.log),
            vec![
                format!("string-drop:{:x}", data + 4),
                format!("string-drop:{data:x}"),
                format!("string-drop:{:x}", data + 12),
                format!("string-drop:{:x}", data + 8),
                "free-array:10".to_string(),
                format!("string-drop:{:x}", this + 0xc),
                "remove:6000".to_string(),
            ]
        );
        assert_eq!(w.e.mem.u32(this), BASE_VTABLE);
        assert_eq!(w.e.mem.u32(this + 8), 0);
        assert_eq!(w.e.mem.u32(this + 0x18), 0x8000_0000);
    }

    #[test]
    fn container_constructors_and_destructor() {
        let mut w = world();
        let this = w.e.mem.alloc(0x40);
        assert_eq!(w.e.call(0x00e2_eb70, &args![this, 0x4242u32]).u32(), this);
        assert_eq!(w.e.mem.u16(this + 6), 1);
        assert_eq!(w.e.mem.u32(this), MEMORY_CONTAINER_VTABLE);
        assert_eq!(w.e.mem.u32(this + 0x18), 0x8000_0000);
        assert_eq!(w.e.mem.u32(this + 0x24), 0x8000_0000);
        assert_eq!(
            strings(&w.log),
            vec![format!("string-copy:{:x}:4242", this + 8)]
        );
        // The variant: the children get this as their parent.
        w.log.borrow_mut().clear();
        let other = w.e.mem.alloc(0x40);
        let child_a = w.e.mem.alloc(0x20);
        let child_b = w.e.mem.alloc(0x20);
        let children = pointers(&mut w.e, &[child_a, child_b], 2);
        w.e.mem.set_u32(other + 0x1c, w.e.mem.u32(children));
        w.e.mem.set_u32(other + 0x20, 2);
        assert_eq!(w.e.call(0x00e2_ebc0, &args![other, 0x9999u32]).u32(), other);
        assert_eq!(
            (w.e.mem.u32(child_a + 0xc), w.e.mem.u32(child_b + 0xc)),
            (other, other)
        );
        // A null text leaves them alone.
        w.e.mem.set_u32(child_a + 0xc, 0);
        w.e.call(0x00e2_ebc0, &args![other, 0u32]);
        assert_eq!(w.e.mem.u32(child_a + 0xc), 0);
        // The destructor drops both arrays' references and the name.
        w.log.borrow_mut().clear();
        let resources = pointers(&mut w.e, &[0x5000, 0], 2);
        w.e.mem.set_u32(this + 0x10, w.e.mem.u32(resources));
        w.e.mem.set_u32(this + 0x14, 2);
        w.e.mem.set_u32(this + 0x18, 2);
        let kids = pointers(&mut w.e, &[0x6000], 1);
        w.e.mem.set_u32(this + 0x1c, w.e.mem.u32(kids));
        w.e.mem.set_u32(this + 0x20, 1);
        w.e.mem.set_u32(this + 0x24, 1);
        w.e.call(0x00e2_ec00, &args![this]);
        assert_eq!(
            strings(&w.log),
            vec![
                "remove:6000".to_string(),
                "free-array:4".to_string(),
                "remove:5000".to_string(),
                "free-array:8".to_string(),
                format!("string-drop:{:x}", this + 8),
            ]
        );
        assert_eq!(w.e.mem.u32(this), BASE_VTABLE);
        assert_eq!((w.e.mem.u32(this + 0x14), w.e.mem.u32(this + 0x20)), (0, 0));
    }

    #[test]
    fn create_resource_names_it_and_appends_it() {
        let mut w = world();
        w.e.map(0x010e_b000, 0x1000);
        w.e.register_double(SET_NAME_SLOT, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut().push(format!("name:{:x}", a[1]));
                Ret::default()
            }
        });
        w.e.register_double(SET_OBJECT_SLOT, {
            let seen = Rc::clone(&w.log);
            move |_, a| {
                seen.borrow_mut()
                    .push(format!("object:{:x}:{:x}", a[1], a[2]));
                Ret::default()
            }
        });
        w.e.mem.set_u32(MEMORY_HANDLE_VTABLE + 0x14, SET_NAME_SLOT);
        w.e.mem
            .set_u32(MEMORY_HANDLE_VTABLE + 0x20, SET_OBJECT_SLOT);
        let this = w.e.mem.alloc(0x40);
        let handle =
            w.e.call(0x00e2_ed00, &args![this, 0x111u32, 0x222u32, 0x333u32])
                .u32();
        assert_ne!(handle, 0);
        assert_eq!(w.e.mem.u16(handle + 4), 0x1c);
        assert_eq!(w.e.mem.u32(this + 0x14), 1);
        assert_eq!(w.e.mem.u32(w.e.mem.u32(this + 0x10)), handle);
        assert_eq!(
            strings(&w.log),
            vec![
                "allocate:1c".to_string(),
                format!("string-new:{:x}", handle + 0xc),
                "name:111".to_string(),
                "object:222:333".to_string(),
                format!("add:{handle:x}"),
                "more:4".to_string(),
                format!("add:{handle:x}"),
                format!("remove:{handle:x}"),
                format!("remove:{handle:x}"),
            ]
        );
    }

    #[test]
    fn find_resource_by_name_checks_the_name_and_the_class() {
        let mut w = world();
        w.e.register(STRING_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: u32::from(x != y),
                ..Ret::default()
            }
        });
        install_class_doubles(&mut w.e);
        // The name double returns the text kept at +0x30 and the class double the class at
        // +0x34 of the resource.
        w.e.register(GET_NAME_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        w.e.register(GET_CLASS_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        let base = w.e.mem.alloc(8);
        let base_name = text(&mut w.e, b"Base");
        w.e.mem.set_u32(base, base_name);
        let derived = w.e.mem.alloc(8);
        let derived_name = text(&mut w.e, b"Derived");
        w.e.mem.set_u32(derived, derived_name);
        w.e.mem.set_u32(derived + 4, base);
        let unrelated = w.e.mem.alloc(8);
        let unrelated_name = text(&mut w.e, b"Other");
        w.e.mem.set_u32(unrelated, unrelated_name);
        let slots = [(0x10, GET_NAME_SLOT), (0x1c, GET_CLASS_SLOT)];
        let mut resources = Vec::new();
        for (name, class) in [(&b"a"[..], unrelated), (b"b", derived)] {
            let resource = object(&mut w.e, &slots);
            let name = text(&mut w.e, name);
            w.e.mem.set_u32(resource + 0x30, name);
            w.e.mem.set_u32(resource + 0x34, class);
            resources.push(resource);
        }
        let record = pointers(&mut w.e, &resources, 2);
        let this = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(this + 0x10, w.e.mem.u32(record));
        w.e.mem.set_u32(this + 0x14, 2);
        let name_b = text(&mut w.e, b"b");
        let name_a = text(&mut w.e, b"a");
        let find = |w: &mut World, name: u32, class: u32, after: u32| {
            w.e.call(0x00e2_edb0, &args![this, name, class, after])
                .u32()
        };
        // By name only.
        assert_eq!(find(&mut w, name_b, 0, 0), resources[1]);
        // By name with an ancestor class: found.
        assert_eq!(find(&mut w, name_b, base, 0), resources[1]);
        // No name: the first resource whose class matches.
        assert_eq!(find(&mut w, 0, base, 0), resources[1]);
        // After the second one nothing is left.
        assert_eq!(find(&mut w, name_b, 0, resources[1]), 0);
        assert!(strings(&w.log).iter().all(|l| !l.starts_with("report")));
        // A name match with a class that is not an ancestor reports and gives up.
        assert_eq!(find(&mut w, name_a, base, 0), 0);
        assert_eq!(
            strings(&w.log),
            vec![
                "stream".to_string(),
                format!("text:<{TEXT_CLASS_MISMATCH:x}>"),
                "text:Base".to_string(),
                format!("text:<{TEXT_CLASS_MISMATCH_MIDDLE:x}>"),
                "text:Other".to_string(),
                "report:1:f034ed22:138".to_string(),
                "stream-end".to_string(),
            ]
        );
    }

    #[test]
    fn create_container_returns_an_existing_one_or_makes_a_new_one() {
        let mut w = world();
        w.e.register(FIND_CONTAINER_SLOT, |e, a| Ret {
            // The name word is the container itself when it is known.
            eax: e.mem.u32(a[0] + 0x30) * u32::from(a[1] == 0x77),
            ..Ret::default()
        });
        let this = object(&mut w.e, &[(0x34, FIND_CONTAINER_SLOT)]);
        w.e.mem.set_u32(this + 0x30, 0x4321);
        // A known name gives the existing container and logs nothing.
        assert_eq!(w.e.call(0x00e2_efe0, &args![this, 0x77u32]).u32(), 0x4321);
        assert!(strings(&w.log).is_empty());
        // An unknown one builds a container and adds it to the array at +0x1C.
        let container = w.e.call(0x00e2_efe0, &args![this, 0x88u32]).u32();
        assert_eq!(w.e.mem.u16(container + 4), 0x28);
        assert_eq!(w.e.mem.u32(container + 0xc), this);
        assert_eq!(w.e.mem.u32(this + 0x20), 1);
        assert_eq!(w.e.mem.u32(w.e.mem.u32(this + 0x1c)), container);
        assert_eq!(
            strings(&w.log),
            vec![
                "allocate:28".to_string(),
                format!("string-copy:{:x}:88", container + 8),
                format!("add:{container:x}"),
                "more:4".to_string(),
                format!("add:{container:x}"),
                format!("remove:{container:x}"),
                format!("remove:{container:x}"),
            ]
        );
    }
}
