//! `Havok SDK/hkstatisticscollectorutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The helpers walk the members of an `hkClass` and report them to an `hkStatisticsCollector`
//! (RTTI vtable `010cce98`). The collector's slots they use are `0xc` (`(name, object, flag)`,
//! for objects of a class with a vtable), `0x10` (`(class, name, object, flag)`, for the others),
//! `0x14` (`addChunk`: kind, name, address, used bytes, allocated bytes), `0x18` (begin a group,
//! one name) and `0x1c` (end it). A class member record is read at these offsets: `+0` the name
//! (a C string pointer), `+0xc` the member type byte, `+0xd` the array item type byte, `+0x10` a
//! 16-bit flag word (bit 9: the member is skipped) and `+0x12` the 16-bit offset of the member's
//! data inside the object. The member types used: `0x14` pointer, `0x16` array, `0x19` struct,
//! `0x1a` simple array, `0x1b` homogeneous array, `0x1c` variant, `0x1d` C string, `0x21`
//! `hkStringPtr` (the code's names for them are not known; they are described by what the code
//! does with them).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`); its `+0x10` is the allocator whose slot `0x2c` answers the size
/// of an allocation.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkString::strLen` and `strCmp` (cdecl).
const STRING_LEN: u32 = 0x00cb_10d0;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// `hkClass`: `getNumDeclaredMembers`, `getDeclaredMember` (index), `getParent`, `getObjectSize`,
/// `hasVtable` (`this`, a `bool` result pointer; answers it); all `thiscall` (Xbox PDB names).
const CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00d4_6f70;
const CLASS_GET_DECLARED_MEMBER: u32 = 0x00d4_6f80;
const CLASS_GET_PARENT: u32 = 0x00d4_6d50;
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
const CLASS_HAS_VTABLE: u32 = 0x00d4_70f0;
/// `hkClassMember`: `getClassMemberTypeProperties` (cdecl, the type byte; answers a record whose
/// byte `+0` is a type and whose 16-bit word `+8` is a size), `getStructClass` and
/// `getCstyleArraySize` (`thiscall` on the member).
const MEMBER_GET_TYPE_PROPERTIES: u32 = 0x00d8_68a0;
const MEMBER_GET_STRUCT_CLASS: u32 = 0x00d8_6910;
const MEMBER_GET_CSTYLE_ARRAY_SIZE: u32 = 0x00d8_6940;
/// `hkClassMemberAccessor`: the constructor (`this`, object address, member), `asCstring`,
/// `asStringPtr`, `asSimpleArray` and `asHomogeneousArray` (`this`, index; each answers the address
/// of the element). An accessor is two words.
const ACCESSOR_CONSTRUCTOR: u32 = 0x00d8_7180;
const ACCESSOR_AS_CSTRING: u32 = 0x00d8_7220;
const ACCESSOR_AS_STRING_PTR: u32 = 0x00d8_7230;
const ACCESSOR_AS_SIMPLE_ARRAY: u32 = 0x00d8_7340;
const ACCESSOR_AS_HOMOGENEOUS_ARRAY: u32 = 0x00d8_7350;
/// `hkStatisticsCollector::addStringPtr` (`thiscall`: name, string pointer).
const COLLECTOR_ADD_STRING_PTR: u32 = 0x00d0_7030;
/// The class record the `hasContents` walk stops at (`01268748`).
const STOP_CLASS: u32 = 0x0126_8748;
/// The strings used: "name" (the member `_findObjectName` looks for), "unnamed" and "array".
const NAME_MEMBER: u32 = 0x0109_4398;
const UNNAMED: u32 = 0x010c_c458;
const ARRAY_GROUP_NAME: u32 = 0x010c_c460;

/// The collector slots and the chunk kinds.
const SLOT_OBJECT_WITH_VTABLE: u32 = 0xc;
const SLOT_OBJECT_WITH_CLASS: u32 = 0x10;
const SLOT_ADD_CHUNK: u32 = 0x14;
const SLOT_BEGIN_GROUP: u32 = 0x18;
const SLOT_END_GROUP: u32 = 0x1c;
const CHUNK_KIND_NORMAL: u32 = 1;
/// Kind 3 is also the kind `hkStatisticsCollector::addStringPtr` reports with (its name is unconfirmed).
const CHUNK_KIND_BLOCK: u32 = 3;

/// The member types the walks above care about.
const MEMBER_POINTER: u8 = 0x14;
const MEMBER_ARRAY: u8 = 0x16;
const MEMBER_STRUCT: u8 = 0x19;
const MEMBER_SIMPLE_ARRAY: u8 = 0x1a;
const MEMBER_HOMOGENEOUS_ARRAY: u8 = 0x1b;
const MEMBER_VARIANT: u8 = 0x1c;
const MEMBER_CSTRING: u8 = 0x1d;
const MEMBER_STRING_PTR: u8 = 0x21;

/// The allocated size of `ptr`/`size` as the collector reports it: `size` for a null pointer, else
/// the router's allocator's slot `0x2c` (`hkStatisticsCollector::getAllocatedSize`, which these
/// functions run inline).
fn allocated_size(e: &mut Engine, ptr: u32, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 0x2c, &args![ptr, size]).u32()
}

// Translated from 00cf5fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_addCstring` (Xbox PDB), cdecl, `(member, string, collector)`:
/// `member`'s first word is the member name. For a non-null `string`: its length plus 1, the
/// allocation's real size (the router's allocator slot `0x2c`, arguments `(string, length + 1)`)
/// and the collector's `addChunk` with `(1, name, string, length + 1, allocated size)`.
pub fn hk_statistics_collector_util_add_cstring(
    e: &mut Engine,
    member: u32,
    string: u32,
    collector: u32,
) {
    if string == 0 {
        return;
    }
    let length = e.call(STRING_LEN, &args![string]).u32().wrapping_add(1);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let allocated = e.vcall(allocator, 0x2c, &args![string, length]).u32();
    let name = e.mem.u32(member);
    e.vcall(
        collector,
        SLOT_ADD_CHUNK,
        &args![CHUNK_KIND_NORMAL, name, string, length, allocated],
    );
}

// Translated from 00cf6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_addObject` (Xbox PDB), cdecl, `(name, object, class, collector)`:
/// nothing for a null `object`. A class with a vtable (`hkClass::hasVtable`, result in a local) calls
/// the collector's slot `0xc` with `(name, object, 0)`; any other calls slot `0x10` with
/// `(class, name, object, 0)`.
pub fn hk_statistics_collector_util_add_object(
    e: &mut Engine,
    name: u32,
    object: u32,
    class: u32,
    collector: u32,
) {
    add_object_with_flag(e, name, object, class, collector, 0);
}

/// The shared body of `_addObject` (flag 0) and `_addObjectContents` (flag 1).
fn add_object_with_flag(
    e: &mut Engine,
    name: u32,
    object: u32,
    class: u32,
    collector: u32,
    flag: u32,
) {
    if object == 0 {
        return;
    }
    // Both exe functions (00cf6030, 00cf60d0) push two registers and hand `hasVtable` the
    // `object` argument's own slot (`LEA EAX,[ESP+0x10]`): +0x10 of a frame that holds the two
    // pushes, the return address and the four arguments.
    let has_vtable = e.with_stack(0x1c, |e, frame| {
        let frame = frame.addr();
        for (slot, word) in [name, object, class, collector].into_iter().enumerate() {
            e.mem.set_u32(frame + 0xc + slot as u32 * 4, word);
        }
        let at = e.call(CLASS_HAS_VTABLE, &args![class, frame + 0x10]).u32();
        e.mem.u8(at) != 0
    });
    if has_vtable {
        e.vcall(
            collector,
            SLOT_OBJECT_WITH_VTABLE,
            &args![name, object, flag],
        );
    } else {
        e.vcall(
            collector,
            SLOT_OBJECT_WITH_CLASS,
            &args![class, name, object, flag],
        );
    }
}

// Translated from 00cf6080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_addArrayData` (Xbox PDB), cdecl, six words `(name, data, size,
/// capacity, element size, collector)`; the decompiler shows five. Asks the router's allocator
/// (slot `0x2c`) for the real size of `data` given `capacity * element size` bytes, then calls the
/// collector's `addChunk` with `(3, name, data, size * element size, allocated size)`.
pub fn hk_statistics_collector_util_add_array_data(
    e: &mut Engine,
    name: u32,
    data: u32,
    size: i32,
    capacity: i32,
    element_size: i32,
    collector: u32,
) {
    let bytes = capacity.wrapping_mul(element_size);
    let allocated = allocated_size(e, data, bytes as u32);
    let used = size.wrapping_mul(element_size);
    e.vcall(
        collector,
        SLOT_ADD_CHUNK,
        &args![CHUNK_KIND_BLOCK, name, data, used, allocated],
    );
}

// Translated from 00cf60d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_addObjectContents` (Xbox PDB), cdecl, `(name, object, class,
/// collector)`: as `_addObject` but with the flag word 1.
pub fn hk_statistics_collector_util_add_object_contents(
    e: &mut Engine,
    name: u32,
    object: u32,
    class: u32,
    collector: u32,
) {
    add_object_with_flag(e, name, object, class, collector, 1);
}

// Translated from 00cf6170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::hasContents` (Xbox PDB), cdecl, `(out, class)`: writes 1 at the
/// byte `out` when `class` or an ancestor has declared members, walking up through the parents
/// and stopping at a null class or the class record at `01268748` (not examined); else 0.
/// Answers `out`.
pub fn hk_statistics_collector_util_has_contents(e: &mut Engine, out: u32, class: u32) -> u32 {
    let mut class = class;
    while class != 0 && class != STOP_CLASS {
        if e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32() > 0 {
            e.mem.set_u8(out, 1);
            return out;
        }
        class = e.call(CLASS_GET_PARENT, &args![class]).u32();
    }
    e.mem.set_u8(out, 0);
    out
}

// Translated from 00cf61c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_getArrayElementSize` (Xbox PDB), cdecl, `(member)`: -1 when the
/// member's item type byte (`+0xd`) is 0. Otherwise looks up the type's properties
/// (`hkClassMember::getClassMemberTypeProperties`): a positive 16-bit size at `+8` is the answer;
/// else for the struct type (`0x19`, byte `+0`) the member's struct class's object size
/// (`hkClass::getObjectSize`, run as the tail of the function); else -1.
pub fn hk_statistics_collector_util_get_array_element_size(e: &mut Engine, member: u32) -> i32 {
    let item_type = e.mem.u8(member + 0xd);
    if item_type == 0 {
        return -1;
    }
    let properties = e
        .call(MEMBER_GET_TYPE_PROPERTIES, &args![u32::from(item_type)])
        .u32();
    let size = e.mem.i16(properties + 8);
    if size > 0 {
        return i32::from(size);
    }
    if e.mem.u8(properties) == MEMBER_STRUCT {
        let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
        return e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
    }
    -1
}

// Translated from 00cf6210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_findObjectName` (engine map name), register input `EBX` = `class`, one stack word `object`,
/// plain `RET`. Looks through the class's declared members for the one named "name"
/// (`01094398`, compared with `hkString::strCmp`) and reads the object's value of that member
/// through a class member accessor: for a C string member (type `0x1d`) the pointer, for an
/// `hkStringPtr` member (type `0x21`) its text with the flag bit cleared. A null name is the
/// string "unnamed" (`010cc458`). Answers 0 when there is no such member (or it has another type).
pub fn find_object_name(e: &mut Engine, class: u32, object: u32) -> u32 {
    let count = e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32();
    let mut index: i32 = 0;
    if count > 0 {
        loop {
            let member = e
                .call(CLASS_GET_DECLARED_MEMBER, &args![class, index])
                .u32();
            let member_name = e.mem.u32(member);
            if e.call(STRING_COMPARE, &args![member_name, NAME_MEMBER])
                .u32()
                == 0
            {
                // The exe's frame (`SUB ESP,0x8`, three pushes): 0x14 bytes, the accessor at
                // +0xc (00cf6249).
                let found = e.with_stack(0x14, |e, frame| {
                    let accessor = frame.addr() + 0xc;
                    e.call(ACCESSOR_CONSTRUCTOR, &args![accessor, object, member]);
                    let member_type = e.mem.u8(member + 0xc);
                    if member_type == MEMBER_CSTRING {
                        let slot = e.call(ACCESSOR_AS_CSTRING, &args![accessor, 0u32]).u32();
                        return Some(e.mem.u32(slot));
                    }
                    if member_type == MEMBER_STRING_PTR {
                        let slot = e.call(ACCESSOR_AS_STRING_PTR, &args![accessor, 0u32]).u32();
                        return Some(e.mem.u32(slot) & 0xffff_fffe);
                    }
                    None
                });
                if let Some(name) = found {
                    return if name != 0 { name } else { UNNAMED };
                }
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    0
}

// Translated from 00cf62b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Cdecl, `(out, member)`: writes 1 at the byte `out` when the member is
/// simple, else 0; answers `out`. A member with flag bit 9 (`+0x10`) is simple; otherwise by the
/// member type byte (`+0xc`): types 1 to `0x12`, `0x15`, `0x18`, `0x1e`, `0x1f` and `0x20` are
/// simple, a struct (`0x19`) is simple when its struct class is (`isClassSimple`, which writes
/// `out` itself), every other type is not.
pub fn fn_00cf62b0(e: &mut Engine, out: u32, member: u32) -> u32 {
    if (e.mem.u16(member + 0x10) >> 9) & 1 != 0 {
        e.mem.set_u8(out, 1);
        return out;
    }
    let member_type = e.mem.u8(member + 0xc);
    match member_type {
        1..=0x12 | 0x15 | 0x18 | 0x1e | 0x1f | 0x20 => e.mem.set_u8(out, 1),
        MEMBER_STRUCT => {
            let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
            hk_statistics_collector_util_is_class_simple(e, out, class);
        }
        _ => e.mem.set_u8(out, 0),
    }
    out
}

// Translated from 00cf6330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Cdecl, `(out, class)`: writes 1 at the byte `out` when every declared member of
/// `class` is simple (`00cf62b0`; the first one that is not gives 0), else 0; answers `out`.
pub fn fn_00cf6330(e: &mut Engine, out: u32, class: u32) -> u32 {
    let count = e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32();
    let mut index: i32 = 0;
    // The exe's frame (`PUSH ECX`, three pushes): 0x10 bytes, the member result byte at +0xf
    // (`LEA EAX,[ESP+0x13]` after a push, 00cf6350).
    let simple = e.with_stack(0x10, |e, frame| {
        let scratch = frame.addr() + 0xf;
        if count > 0 {
            loop {
                let member = e
                    .call(CLASS_GET_DECLARED_MEMBER, &args![class, index])
                    .u32();
                let at = fn_00cf62b0(e, scratch, member);
                if e.mem.u8(at) == 0 {
                    return false;
                }
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
        true
    });
    e.mem.set_u8(out, u8::from(simple));
    out
}

// Translated from 00cf6380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::isClassSimple` (Xbox PDB), cdecl, `(out, class)`: writes 1 at the
/// byte `out` when `class` and all its ancestors have only simple members (`00cf6330` on each,
/// the result going to the stack slot that held `class`), else 0; answers `out`.
pub fn hk_statistics_collector_util_is_class_simple(e: &mut Engine, out: u32, class: u32) -> u32 {
    let mut class = class;
    // The exe's frame (one push) taken with the arguments: the result goes to the `class`
    // argument's own slot (`LEA EAX,[ESP+0xc]`, 00cf6385), +0xc past the push and the return
    // address.
    let simple = e.with_stack(0x10, |e, frame| {
        let frame = frame.addr();
        let scratch = frame + 0xc;
        e.mem.set_u32(frame + 8, out);
        e.mem.set_u32(scratch, class);
        loop {
            let at = fn_00cf6330(e, scratch, class);
            if e.mem.u8(at) == 0 {
                return false;
            }
            class = e.call(CLASS_GET_PARENT, &args![class]).u32();
            if class == 0 {
                return true;
            }
        }
    });
    e.mem.set_u8(out, u8::from(simple));
    out
}

// Translated from 00cf63c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_addHomogeneousArrayContents` (Xbox PDB), cdecl, `(name, class,
/// data, count, collector)`: nothing for a count of 0 or a simple class (`isClassSimple`).
/// Otherwise reports each of `count` elements of `class`'s object size starting at `data`: through
/// slot `0xc` with `(name, element, 1)` when the class has a vtable, else through slot `0x10`
/// with `(class, name, element, 1)`.
pub fn hk_statistics_collector_util_add_homogeneous_array_contents(
    e: &mut Engine,
    name: u32,
    class: u32,
    data: u32,
    count: i32,
    collector: u32,
) {
    if count == 0 {
        return;
    }
    // The exe's frame (four pushes at most) taken with the arguments: both results go to the
    // `count` argument's own slot (`LEA EAX,[ESP+0x18]` after two pushes, 00cf63d2; `LEA ECX,
    // [ESP+0x20]` after four, 00cf63f2), +0x20 past the pushes and the return address.
    let frame_result = e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let result = frame + 0x20;
        for (slot, word) in [name, class, data, count as u32, collector]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(frame + 0x14 + slot as u32 * 4, word);
        }
        let at = hk_statistics_collector_util_is_class_simple(e, result, class);
        if e.mem.u8(at) != 0 {
            return None;
        }
        let element_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32();
        let at = e.call(CLASS_HAS_VTABLE, &args![class, result]).u32();
        Some((element_size, e.mem.u8(at) != 0))
    });
    let Some((element_size, has_vtable)) = frame_result else {
        return;
    };
    let mut element = data;
    let slot = if has_vtable {
        SLOT_OBJECT_WITH_VTABLE
    } else {
        SLOT_OBJECT_WITH_CLASS
    };
    let mut remaining = count;
    while remaining > 0 {
        if has_vtable {
            e.vcall(collector, slot, &args![name, element, 1u32]);
        } else {
            e.vcall(collector, slot, &args![class, name, element, 1u32]);
        }
        element = element.wrapping_add(element_size);
        remaining -= 1;
    }
}

// Translated from 00cf6460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::_addArrayContents` (Xbox PDB), cdecl, `(member, data, count,
/// collector)`: nothing for a count of 0 or less. By the member's item type byte (`+0xd`): pointers
/// (`0x14`) to a struct class: each of the `count` pointers in `data` is reported
/// (`_addObject`) under its object's name (`_findObjectName`, else the member's name); struct
/// (`0x19`): `_addHomogeneousArrayContents` with the member's name and struct class; C strings
/// (`0x1d`): `_addCstring` for each; `hkStringPtr`s (`0x21`): the collector's `addStringPtr` for
/// each consecutive one; every other type: nothing. Members with no struct class report
/// nothing.
pub fn hk_statistics_collector_util_add_array_contents(
    e: &mut Engine,
    member: u32,
    data: u32,
    count: i32,
    collector: u32,
) {
    if count <= 0 {
        return;
    }
    let item_type = e.mem.u8(member + 0xd);
    match item_type {
        MEMBER_POINTER => {
            let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
            if class == 0 {
                return;
            }
            for index in 0..count as u32 {
                let object = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
                let mut name = find_object_name(e, class, object);
                if name == 0 {
                    name = e.mem.u32(member);
                }
                let object = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
                hk_statistics_collector_util_add_object(e, name, object, class, collector);
            }
        }
        MEMBER_STRUCT => {
            let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
            if class != 0 {
                let name = e.mem.u32(member);
                hk_statistics_collector_util_add_homogeneous_array_contents(
                    e, name, class, data, count, collector,
                );
            }
        }
        MEMBER_CSTRING => {
            for index in 0..count as u32 {
                let string = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
                hk_statistics_collector_util_add_cstring(e, member, string, collector);
            }
        }
        MEMBER_STRING_PTR => {
            let mut string_ptr = data;
            for _ in 0..count {
                let name = e.mem.u32(member);
                e.call(
                    COLLECTOR_ADD_STRING_PTR,
                    &args![collector, name, string_ptr],
                );
                string_ptr = string_ptr.wrapping_add(4);
            }
        }
        _ => {}
    }
}

/// Reports one member of a class's declared members: the body of `addClassContents`' loop.
fn add_member_contents(
    e: &mut Engine,
    frame: u32,
    object: u32,
    class: u32,
    collector: u32,
    index: i32,
) {
    let member = e
        .call(CLASS_GET_DECLARED_MEMBER, &args![class, index])
        .u32();
    if (e.mem.u16(member + 0x10) >> 9) & 1 != 0 {
        return;
    }
    let member_type = e.mem.u8(member + 0xc);
    match member_type {
        MEMBER_STRUCT => {
            let name = e.mem.u32(member);
            let offset = u32::from(e.mem.u16(member + 0x12));
            let address = offset.wrapping_add(object);
            let struct_class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
            hk_statistics_collector_util_add_object_contents(
                e,
                name,
                address,
                struct_class,
                collector,
            );
        }
        MEMBER_POINTER => {
            if e.mem.u8(member + 0xd) != MEMBER_STRUCT {
                return;
            }
            let name = e.mem.u32(member);
            let offset = u32::from(e.mem.u16(member + 0x12));
            let pointer = e.mem.u32(offset.wrapping_add(object));
            let struct_class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
            hk_statistics_collector_util_add_object(e, name, pointer, struct_class, collector);
        }
        MEMBER_VARIANT => {
            let offset = u32::from(e.mem.u16(member + 0x12));
            let address = offset.wrapping_add(object);
            let pointer = e.mem.u32(address);
            if pointer == 0 {
                return;
            }
            let class_word = e.mem.u32(address + 4);
            if class_word == 0 {
                return;
            }
            let name = e.mem.u32(member);
            hk_statistics_collector_util_add_object(e, name, pointer, class_word, collector);
        }
        MEMBER_CSTRING => {
            let accessor = frame + 0x24;
            e.call(ACCESSOR_CONSTRUCTOR, &args![accessor, object, member]);
            let size = e.call(MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member]).i32();
            if size <= 1 {
                let slot = e.call(ACCESSOR_AS_CSTRING, &args![accessor, 0u32]).u32();
                let string = e.mem.u32(slot);
                hk_statistics_collector_util_add_cstring(e, member, string, collector);
            } else {
                for index in 0..size {
                    let slot = e.call(ACCESSOR_AS_CSTRING, &args![accessor, index]).u32();
                    let string = e.mem.u32(slot);
                    hk_statistics_collector_util_add_cstring(e, member, string, collector);
                }
            }
        }
        MEMBER_STRING_PTR => {
            let accessor = frame + 0x2c;
            e.call(ACCESSOR_CONSTRUCTOR, &args![accessor, object, member]);
            let size = e.call(MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member]).i32();
            if size <= 1 {
                let name = e.mem.u32(member);
                let slot = e.call(ACCESSOR_AS_STRING_PTR, &args![accessor, 0u32]).u32();
                e.call(COLLECTOR_ADD_STRING_PTR, &args![collector, name, slot]);
            } else {
                for index in 0..size {
                    let name = e.mem.u32(member);
                    let slot = e
                        .call(ACCESSOR_AS_STRING_PTR, &args![accessor, index])
                        .u32();
                    e.call(COLLECTOR_ADD_STRING_PTR, &args![collector, name, slot]);
                }
            }
        }
        MEMBER_ARRAY => {
            let array = object.wrapping_add(u32::from(e.mem.u16(member + 0x12)));
            let flags = e.mem.u32(array + 8);
            if (flags as i32) < 0 && e.mem.u8(collector + 0x10) == 0 {
                return;
            }
            let size = e.mem.u32(array + 4);
            let capacity = flags & 0x3fff_ffff;
            let element_size = hk_statistics_collector_util_get_array_element_size(e, member);
            if element_size < 0 {
                return;
            }
            let pointer_array = e.mem.u8(member + 0xd) == MEMBER_POINTER;
            let mut name = e.mem.u32(member);
            if pointer_array {
                e.vcall(collector, SLOT_BEGIN_GROUP, &args![name]);
                name = ARRAY_GROUP_NAME;
            }
            let data = e.mem.u32(array);
            hk_statistics_collector_util_add_array_data(
                e,
                name,
                data,
                size as i32,
                capacity as i32,
                element_size,
                collector,
            );
            let data = e.mem.u32(array);
            hk_statistics_collector_util_add_array_contents(
                e,
                member,
                data,
                size as i32,
                collector,
            );
            if pointer_array {
                e.vcall(collector, SLOT_END_GROUP, &args![]);
            }
        }
        MEMBER_SIMPLE_ARRAY => {
            let accessor = frame + 0x3c;
            e.call(ACCESSOR_CONSTRUCTOR, &args![accessor, object, member]);
            let slot = e
                .call(ACCESSOR_AS_SIMPLE_ARRAY, &args![accessor, 0u32])
                .u32();
            let data = e.mem.u32(slot);
            let size = e.mem.i32(slot + 4);
            let element_size = hk_statistics_collector_util_get_array_element_size(e, member);
            if element_size < 0 {
                return;
            }
            let name = e.mem.u32(member);
            e.vcall(collector, SLOT_BEGIN_GROUP, &args![name]);
            let name = e.mem.u32(member);
            hk_statistics_collector_util_add_array_data(
                e,
                name,
                data,
                size,
                size,
                element_size,
                collector,
            );
            hk_statistics_collector_util_add_array_contents(e, member, data, size, collector);
            e.vcall(collector, SLOT_END_GROUP, &args![]);
        }
        MEMBER_HOMOGENEOUS_ARRAY => {
            let accessor = frame + 0x44;
            e.call(ACCESSOR_CONSTRUCTOR, &args![accessor, object, member]);
            let slot = e
                .call(ACCESSOR_AS_HOMOGENEOUS_ARRAY, &args![accessor, 0u32])
                .u32();
            let size = e.mem.i32(slot + 8);
            let class = e.mem.u32(slot);
            let data = e.mem.u32(slot + 4);
            if size <= 0 || class == 0 || data == 0 {
                return;
            }
            let name = e.mem.u32(member);
            e.vcall(collector, SLOT_BEGIN_GROUP, &args![name]);
            let name = e.mem.u32(member);
            let element_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
            hk_statistics_collector_util_add_array_data(
                e,
                name,
                data,
                size,
                size,
                element_size,
                collector,
            );
            let name = e.mem.u32(member);
            hk_statistics_collector_util_add_homogeneous_array_contents(
                e, name, class, data, size, collector,
            );
            e.vcall(collector, SLOT_END_GROUP, &args![]);
        }
        _ => {}
    }
}

// Translated from 00cf6580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::addClassContents` (Xbox PDB), cdecl, `(object, class, collector)`:
/// reports the contents of `object` that `class` declares, member by member (nothing for a null
/// `object`; members with flag bit 9 are skipped). A pointer (`0x14`) to a struct reports the object it points
/// to (`_addObject`); a struct (`0x19`) its contents inline (`_addObjectContents`); a variant
/// (`0x1c`, an object and a class word, both set) the object (`_addObject`); a C string (`0x1d`)
/// each string of the member's array (`getCstyleArraySize`) with `_addCstring`; an `hkStringPtr`
/// (`0x21`) each through the collector's `addStringPtr`; an array (`0x16`, only when it owns its
/// storage or the collector's flag byte `+0x10` is set) its data (`_addArrayData`) and contents
/// (`_addArrayContents`), a pointer array inside a group opened and closed through the
/// collector's slots `0x18` and `0x1c` with the group name "array"; a simple array (`0x1a`,
/// data and size) and a homogeneous array (`0x1b`, class, data and size, all non-zero) likewise
/// inside a group. The other member types report nothing.
pub fn hk_statistics_collector_util_add_class_contents(
    e: &mut Engine,
    object: u32,
    class: u32,
    collector: u32,
) {
    if object == 0 {
        return;
    }
    let count = e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32();
    if count <= 0 {
        return;
    }
    // The exe's frame (`AND ESP,0xfffffff8`, `SUB ESP,0x4c`, three pushes): 0x58 bytes from an
    // 8-byte boundary; each member type's accessor has its own place: a C string's at +0x24
    // (00cf65ed), an `hkStringPtr`'s at +0x2c (00cf6655), a simple array's at +0x3c (00cf675f)
    // and a homogeneous array's at +0x44 (00cf67d2).
    e.with_stack(0x58 + 8, |e, raw| {
        let frame = (raw.addr() + 7) & !7;
        let mut index: i32 = 0;
        loop {
            add_member_contents(e, frame, object, class, collector, index);
            index += 1;
            if index >= count {
                break;
            }
        }
    });
}

// Translated from 00cf6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollectorUtil::addClassContentsAll` (Xbox PDB), cdecl, `(object, class,
/// collector)`: nothing for a null `object`; otherwise `addClassContents` for `class` and then
/// for each ancestor, up to the root.
pub fn hk_statistics_collector_util_add_class_contents_all(
    e: &mut Engine,
    object: u32,
    class: u32,
    collector: u32,
) {
    if object == 0 {
        return;
    }
    let mut class = class;
    loop {
        hk_statistics_collector_util_add_class_contents(e, object, class, collector);
        class = e.call(CLASS_GET_PARENT, &args![class]).u32();
        if class == 0 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf5fe0,
            hk_statistics_collector_util_add_cstring(u32, u32, u32)
        ),
        entry!(
            0x00cf6030,
            hk_statistics_collector_util_add_object(u32, u32, u32, u32)
        ),
        entry!(
            0x00cf6080,
            hk_statistics_collector_util_add_array_data(u32, u32, i32, i32, i32, u32)
        ),
        entry!(
            0x00cf60d0,
            hk_statistics_collector_util_add_object_contents(u32, u32, u32, u32)
        ),
        entry!(0x00cf6170, hk_statistics_collector_util_has_contents(u32, u32) -> u32),
        entry!(0x00cf61c0, hk_statistics_collector_util_get_array_element_size(u32) -> i32),
        entry!(0x00cf6210, find_object_name(u32, u32) -> u32),
        entry!(0x00cf62b0, fn_00cf62b0(u32, u32) -> u32),
        entry!(0x00cf6330, fn_00cf6330(u32, u32) -> u32),
        entry!(0x00cf6380, hk_statistics_collector_util_is_class_simple(u32, u32) -> u32),
        entry!(
            0x00cf63c0,
            hk_statistics_collector_util_add_homogeneous_array_contents(u32, u32, u32, i32, u32)
        ),
        entry!(
            0x00cf6460,
            hk_statistics_collector_util_add_array_contents(u32, u32, i32, u32)
        ),
        entry!(
            0x00cf6580,
            hk_statistics_collector_util_add_class_contents(u32, u32, u32)
        ),
        entry!(
            0x00cf6920,
            hk_statistics_collector_util_add_class_contents_all(u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the class, member, string and router functions as doubles over simple
    /// records, and a collector object (flag byte at `+0x10`) whose slots log `(slot, arguments)`.
    ///
    /// Class record: `+0` member count, `+4` parent, `+8` pointer to the member pointers, `+0xc`
    /// object size, `+0x10` has-vtable byte. Member record: the real layout (name `+0`, type `+0xc`,
    /// item type `+0xd`, flags `+0x10`, offset `+0x12`) plus the struct class at `+0x14` and the
    /// C-style array size at `+0x18`. The allocator's slot `0x2c` answers `size + 0x10` and is
    /// logged as slot `0x2c`.
    fn world() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0109_4000, 0x1000);
        e.mem.set_cstr(NAME_MEMBER, b"name");
        e.register(CLASS_GET_NUM_DECLARED_MEMBERS, |e, a| {
            answer(e.mem.u32(a[0]))
        });
        e.register(CLASS_GET_DECLARED_MEMBER, |e, a| {
            let members = e.mem.u32(a[0] + 8);
            answer(e.mem.u32(members + 4 * a[1]))
        });
        e.register(CLASS_GET_PARENT, |e, a| answer(e.mem.u32(a[0] + 4)));
        e.register(CLASS_GET_OBJECT_SIZE, |e, a| answer(e.mem.u32(a[0] + 0xc)));
        e.register(CLASS_HAS_VTABLE, |e, a| {
            let flag = e.mem.u8(a[0] + 0x10);
            e.mem.set_u8(a[1], flag);
            answer(a[1])
        });
        e.register(MEMBER_GET_STRUCT_CLASS, |e, a| {
            answer(e.mem.u32(a[0] + 0x14))
        });
        e.register(MEMBER_GET_CSTYLE_ARRAY_SIZE, |e, a| {
            answer(e.mem.u32(a[0] + 0x18))
        });
        // The type properties: a 16-byte record per type; struct (0x19) has no size, the others 4.
        let properties = e.mem.alloc(0x400);
        for kind in 0..0x40u32 {
            let record = properties + 0x10 * kind;
            e.mem.set_u8(record, kind as u8);
            e.mem.set_u16(record + 8, if kind == 0x19 { 0 } else { 4 });
        }
        e.register_double(MEMBER_GET_TYPE_PROPERTIES, move |_, a| {
            answer(properties + 0x10 * a[0])
        });
        e.register(STRING_LEN, |e, a| answer(e.mem.cstr(a[0]).len() as u32));
        e.register(STRING_COMPARE, |e, a| {
            answer(u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])))
        });
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        // The collector: slots 0xc, 0x10, 0x14, 0x18, 0x1c.
        let mut slots = Vec::new();
        for slot in [0xcu32, 0x10, 0x14, 0x18, 0x1c] {
            let l = Rc::clone(&log);
            e.register_double(0x0e00_0000 + slot, move |_, a| {
                l.borrow_mut().push((slot, a[1..].to_vec()));
                Ret::default()
            });
            slots.push((slot, 0x0e00_0000 + slot));
        }
        let collector = object_with_slots(&mut e, &slots);
        let l = Rc::clone(&log);
        e.register_double(COLLECTOR_ADD_STRING_PTR, move |_, a| {
            l.borrow_mut().push((0xffff, a[1..].to_vec()));
            Ret::default()
        });
        // The router's allocator: slot 0x2c answers size + 0x10.
        let l = Rc::clone(&log);
        e.register_double(0x0e10_002c, move |_, a| {
            l.borrow_mut().push((0x2c, a[1..].to_vec()));
            answer(a[2] + 0x10)
        });
        let allocator = object_with_slots(&mut e, &[(0x2c, 0x0e10_002c)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| answer(router));
        (e, log, collector)
    }

    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(at, s.as_bytes());
        at
    }

    struct Member {
        kind: u8,
        item: u8,
        flags: u16,
        offset: u16,
        struct_class: u32,
        array_size: u32,
    }

    const PLAIN: Member = Member {
        kind: 1,
        item: 0,
        flags: 0,
        offset: 0,
        struct_class: 0,
        array_size: 1,
    };

    fn member(e: &mut Engine, name: &str, m: Member) -> u32 {
        let record = e.mem.alloc(0x20);
        let name = text(e, name);
        e.mem.set_u32(record, name);
        e.mem.set_u8(record + 0xc, m.kind);
        e.mem.set_u8(record + 0xd, m.item);
        e.mem.set_u16(record + 0x10, m.flags);
        e.mem.set_u16(record + 0x12, m.offset);
        e.mem.set_u32(record + 0x14, m.struct_class);
        e.mem.set_u32(record + 0x18, m.array_size);
        record
    }

    fn class(e: &mut Engine, members: &[u32], parent: u32, size: u32, vtable: bool) -> u32 {
        let record = e.mem.alloc(0x20);
        let list = e.mem.alloc(4 * members.len().max(1) as u32);
        for (i, m) in members.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *m);
        }
        e.mem.set_u32(record, members.len() as u32);
        e.mem.set_u32(record + 4, parent);
        e.mem.set_u32(record + 8, list);
        e.mem.set_u32(record + 0xc, size);
        e.mem.set_u8(record + 0x10, u8::from(vtable));
        record
    }

    fn slot_calls(log: &Log, slot: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == slot)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn add_cstring_reports_the_string_with_its_length_and_allocated_size() {
        let (mut e, log, collector) = world();
        let name = text(&mut e, "title");
        let member_record = e.mem.alloc(0x20);
        e.mem.set_u32(member_record, name);
        let string = text(&mut e, "hello");
        e.call(0x00cf_5fe0, &args![member_record, string, collector]);
        // The allocator is asked about (string, 6); addChunk gets (1, name, string, 6, 0x16).
        assert_eq!(slot_calls(&log, 0x2c), vec![vec![string, 6]]);
        assert_eq!(slot_calls(&log, 0x14), vec![vec![1, name, string, 6, 0x16]]);
        // A null string reports nothing.
        e.call(0x00cf_5fe0, &args![member_record, 0u32, collector]);
        assert_eq!(slot_calls(&log, 0x14).len(), 1);
    }

    #[test]
    fn locals_sit_at_the_exe_frame_offsets() {
        let (mut e, _, collector) = world();
        // Where the called function's own (outermost) `size`-byte frame lands on the stack; its
        // bytes stay as the call left them until another local takes their place.
        let next_frame = |e: &mut Engine, size: u32| e.next_stack_local(size);
        let seen: Rc<RefCell<Vec<u32>>> = Rc::default();
        let s = Rc::clone(&seen);
        e.register_double(CLASS_HAS_VTABLE, move |e, a| {
            s.borrow_mut().push(a[1]);
            e.mem.set_u8(a[1], 0);
            answer(a[1])
        });
        let plain = class(&mut e, &[], 0, 8, false);
        // 00cf6030 / 00cf60d0: `hasVtable` gets the `object` argument's slot, +0x10 of 0x1c.
        let frame = next_frame(&mut e, 0x1c);
        e.call(0x00cf_6030, &args![0xaau32, 0x1000u32, plain, collector]);
        assert_eq!(seen.borrow()[0], frame + 0x10);
        let frame = next_frame(&mut e, 0x1c);
        e.call(0x00cf_60d0, &args![0xaau32, 0x1000u32, plain, collector]);
        assert_eq!(seen.borrow()[1], frame + 0x10);
        // 00cf63c0: the `count` argument's slot, +0x20 of 0x28.
        let complex_member = member(
            &mut e,
            "c",
            Member {
                kind: 0x1d,
                ..PLAIN
            },
        );
        let complex = class(&mut e, &[complex_member], 0, 0x10, false);
        let frame = next_frame(&mut e, 0x28);
        e.call(
            0x00cf_63c0,
            &args![0xaau32, complex, 0x1000u32, 1i32, collector],
        );
        assert_eq!(seen.borrow()[2], frame + 0x20);
        // 00cf6380: 00cf6330's answer lands in the `class` argument's slot, +0xc of 0x10.
        let out = e.mem.alloc(4);
        let frame = next_frame(&mut e, 0x10);
        e.call(0x00cf_6380, &args![out, plain]);
        assert_eq!(e.mem.u8(frame + 0xc), 1);
        // 00cf6210: the accessor at +0xc of 0x14 (its first word the member's address).
        let object = e.mem.alloc(0x40);
        let name = member(
            &mut e,
            "name",
            Member {
                kind: 0x1d,
                offset: 0x10,
                ..PLAIN
            },
        );
        let owner = class(&mut e, &[name], 0, 0x40, false);
        let frame = next_frame(&mut e, 0x14);
        e.call(0x00cf_6210, &args![owner, object]);
        assert_eq!(e.mem.u32(frame + 0xc), object + 0x10);
        // 00cf6580: a C string member's accessor at +0x24 of the 8-aligned 0x58 bytes.
        let text_member = member(
            &mut e,
            "t",
            Member {
                kind: 0x1d,
                offset: 0x20,
                array_size: 1,
                ..PLAIN
            },
        );
        let owner = class(&mut e, &[text_member], 0, 0x60, false);
        let object = e.mem.alloc(0x60);
        let frame = (next_frame(&mut e, 0x60) + 7) & !7;
        e.call(0x00cf_6580, &args![object, owner, collector]);
        assert_eq!(e.mem.u32(frame + 0x24), object + 0x20);
    }

    #[test]
    fn add_object_and_contents_choose_the_slot_by_the_vtable() {
        let (mut e, log, collector) = world();
        let with = class(&mut e, &[], 0, 8, true);
        let without = class(&mut e, &[], 0, 8, false);
        e.call(0x00cf_6030, &args![0xaau32, 0x1000u32, with, collector]);
        e.call(0x00cf_6030, &args![0xaau32, 0x1000u32, without, collector]);
        e.call(0x00cf_60d0, &args![0xaau32, 0x2000u32, with, collector]);
        e.call(0x00cf_60d0, &args![0xaau32, 0x2000u32, without, collector]);
        assert_eq!(
            slot_calls(&log, 0xc),
            vec![vec![0xaa, 0x1000, 0], vec![0xaa, 0x2000, 1]]
        );
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![
                vec![without, 0xaa, 0x1000, 0],
                vec![without, 0xaa, 0x2000, 1]
            ]
        );
        // A null object reports nothing.
        e.call(0x00cf_6030, &args![0xaau32, 0u32, with, collector]);
        e.call(0x00cf_60d0, &args![0xaau32, 0u32, without, collector]);
        assert_eq!(log.borrow().len(), 4);
    }

    #[test]
    fn add_array_data_reports_used_bytes_and_asks_for_the_capacity() {
        let (mut e, log, collector) = world();
        e.call(
            0x00cf_6080,
            &args![0xaau32, 0x4000u32, 3i32, 8i32, 4i32, collector],
        );
        // The allocation size query uses capacity * element size; the chunk size * element size.
        assert_eq!(slot_calls(&log, 0x2c), vec![vec![0x4000, 32]]);
        assert_eq!(
            slot_calls(&log, 0x14),
            vec![vec![3, 0xaa, 0x4000, 12, 0x30]]
        );
    }

    #[test]
    fn has_contents_looks_up_the_parents_but_not_past_the_stop_class() {
        let (mut e, _, _) = world();
        let out = e.mem.alloc(4);
        let empty = class(&mut e, &[], 0, 8, false);
        assert_eq!(e.call(0x00cf_6170, &args![out, empty]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
        let m = member(&mut e, "x", PLAIN);
        let full_parent = class(&mut e, &[m], 0, 8, false);
        let child = class(&mut e, &[], full_parent, 8, false);
        e.mem.set_u8(out, 0xaa);
        assert_eq!(e.call(0x00cf_6170, &args![out, child]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        // A null class, and the stop class record itself, give 0 without being examined.
        e.call(0x00cf_6170, &args![out, 0u32]);
        assert_eq!(e.mem.u8(out), 0);
        e.mem.set_u8(out, 5);
        e.call(0x00cf_6170, &args![out, STOP_CLASS]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn get_array_element_size_by_item_type() {
        let (mut e, _, _) = world();
        let none = member(&mut e, "a", PLAIN);
        assert_eq!(e.call(0x00cf_61c0, &args![none]).i32(), -1);
        // A type with a size (4 in the doubled table).
        let sized = member(&mut e, "b", Member { item: 5, ..PLAIN });
        assert_eq!(e.call(0x00cf_61c0, &args![sized]).i32(), 4);
        // The struct type has no size of its own: the struct class's object size.
        let inner = class(&mut e, &[], 0, 0x24, false);
        let structured = member(
            &mut e,
            "c",
            Member {
                item: 0x19,
                struct_class: inner,
                ..PLAIN
            },
        );
        assert_eq!(e.call(0x00cf_61c0, &args![structured]).i32(), 0x24);
        // A negative size is not a size: it falls through to -1 for a non-struct type. The
        // properties of type 0x30 are set to -2.
        let properties = e.call(MEMBER_GET_TYPE_PROPERTIES, &args![0x30u32]).u32();
        e.mem.set_i16(properties + 8, -2);
        let odd = member(
            &mut e,
            "d",
            Member {
                item: 0x30,
                ..PLAIN
            },
        );
        assert_eq!(e.call(0x00cf_61c0, &args![odd]).i32(), -1);
    }

    #[test]
    fn find_object_name_reads_the_name_member_through_an_accessor() {
        let (mut e, _, _) = world();
        // A C string member "name" at offset 0x10 of the object.
        let object = e.mem.alloc(0x40);
        let title = text(&mut e, "Crate");
        e.mem.set_u32(object + 0x10, title);
        let other = member(
            &mut e,
            "weight",
            Member {
                kind: 0x1d,
                offset: 0x14,
                ..PLAIN
            },
        );
        let name = member(
            &mut e,
            "name",
            Member {
                kind: 0x1d,
                offset: 0x10,
                ..PLAIN
            },
        );
        let owner = class(&mut e, &[other, name], 0, 0x40, false);
        assert_eq!(e.call(0x00cf_6210, &args![owner, object]).u32(), title);
        // A null C string is "unnamed".
        e.mem.set_u32(object + 0x10, 0);
        assert_eq!(e.call(0x00cf_6210, &args![owner, object]).u32(), UNNAMED);
        // An hkStringPtr member: the text with the flag bit cleared, "unnamed" when null.
        let string_name = member(
            &mut e,
            "name",
            Member {
                kind: 0x21,
                offset: 0x18,
                ..PLAIN
            },
        );
        let owner = class(&mut e, &[string_name], 0, 0x40, false);
        e.mem.set_u32(object + 0x18, title | 1);
        assert_eq!(e.call(0x00cf_6210, &args![owner, object]).u32(), title);
        e.mem.set_u32(object + 0x18, 1);
        assert_eq!(e.call(0x00cf_6210, &args![owner, object]).u32(), UNNAMED);
        // A member named "name" of another type is passed over; no member: 0.
        let wrong_type = member(
            &mut e,
            "name",
            Member {
                kind: 0x05,
                offset: 0x10,
                ..PLAIN
            },
        );
        let owner = class(&mut e, &[wrong_type, other], 0, 0x40, false);
        assert_eq!(e.call(0x00cf_6210, &args![owner, object]).u32(), 0);
        let empty = class(&mut e, &[], 0, 0x40, false);
        assert_eq!(e.call(0x00cf_6210, &args![empty, object]).u32(), 0);
    }

    #[test]
    fn simplicity_of_members_classes_and_their_ancestors() {
        let (mut e, _, _) = world();
        let out = e.mem.alloc(4);
        let simple = |e: &mut Engine, out: u32, m: u32| {
            assert_eq!(e.call(0x00cf_62b0, &args![out, m]).u32(), out);
            e.mem.u8(out) == 1
        };
        for kind in [1u8, 0x12, 0x15, 0x18, 0x1e, 0x1f, 0x20] {
            let m = member(&mut e, "x", Member { kind, ..PLAIN });
            assert!(simple(&mut e, out, m), "type {kind:#x}");
        }
        for kind in [0u8, 0x13, 0x14, 0x16, 0x1a, 0x1b, 0x1c, 0x1d, 0x21, 0x40] {
            let m = member(&mut e, "x", Member { kind, ..PLAIN });
            assert!(!simple(&mut e, out, m), "type {kind:#x}");
        }
        // Flag bit 9 makes any member simple.
        let flagged = member(
            &mut e,
            "x",
            Member {
                kind: 0x1d,
                flags: 0x200,
                ..PLAIN
            },
        );
        assert!(simple(&mut e, out, flagged));
        // A struct member is simple when its class is.
        let plain = member(&mut e, "p", PLAIN);
        let simple_class = class(&mut e, &[plain], 0, 4, false);
        let complex_member = member(
            &mut e,
            "c",
            Member {
                kind: 0x1d,
                ..PLAIN
            },
        );
        let complex_class = class(&mut e, &[plain, complex_member], 0, 4, false);
        let in_simple = member(
            &mut e,
            "s",
            Member {
                kind: 0x19,
                struct_class: simple_class,
                ..PLAIN
            },
        );
        let in_complex = member(
            &mut e,
            "s",
            Member {
                kind: 0x19,
                struct_class: complex_class,
                ..PLAIN
            },
        );
        assert!(simple(&mut e, out, in_simple));
        assert!(!simple(&mut e, out, in_complex));
        // `00cf6330`: all declared members; `isClassSimple`: the ancestors too.
        assert_eq!(e.call(0x00cf_6330, &args![out, simple_class]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00cf_6330, &args![out, complex_class]);
        assert_eq!(e.mem.u8(out), 0);
        let empty = class(&mut e, &[], 0, 4, false);
        e.call(0x00cf_6330, &args![out, empty]);
        assert_eq!(e.mem.u8(out), 1);
        let child_of_simple = class(&mut e, &[plain], simple_class, 4, false);
        let child_of_complex = class(&mut e, &[plain], complex_class, 4, false);
        assert_eq!(e.call(0x00cf_6380, &args![out, child_of_simple]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00cf_6380, &args![out, child_of_complex]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn homogeneous_array_contents_reports_each_element_of_a_complex_class() {
        let (mut e, log, collector) = world();
        let plain = member(&mut e, "p", PLAIN);
        let complex_member = member(
            &mut e,
            "c",
            Member {
                kind: 0x1d,
                ..PLAIN
            },
        );
        let simple_class = class(&mut e, &[plain], 0, 8, false);
        let with_vtable = class(&mut e, &[complex_member], 0, 0x10, true);
        let without = class(&mut e, &[complex_member], 0, 0x10, false);
        // A simple class and a count of 0 report nothing.
        e.call(
            0x00cf_63c0,
            &args![0xaau32, simple_class, 0x1000u32, 3i32, collector],
        );
        e.call(
            0x00cf_63c0,
            &args![0xaau32, without, 0x1000u32, 0i32, collector],
        );
        e.call(
            0x00cf_63c0,
            &args![0xaau32, without, 0x1000u32, -2i32, collector],
        );
        assert!(log.borrow().is_empty());
        e.call(
            0x00cf_63c0,
            &args![0xaau32, with_vtable, 0x1000u32, 3i32, collector],
        );
        assert_eq!(
            slot_calls(&log, 0xc),
            vec![
                vec![0xaa, 0x1000, 1],
                vec![0xaa, 0x1010, 1],
                vec![0xaa, 0x1020, 1]
            ]
        );
        e.call(
            0x00cf_63c0,
            &args![0xaau32, without, 0x2000u32, 2i32, collector],
        );
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![
                vec![without, 0xaa, 0x2000, 1],
                vec![without, 0xaa, 0x2010, 1]
            ]
        );
    }

    #[test]
    fn array_contents_by_item_type() {
        let (mut e, log, collector) = world();
        // A struct class whose objects are named by their "name" C string.
        let name_member = member(
            &mut e,
            "name",
            Member {
                kind: 0x1d,
                offset: 0,
                ..PLAIN
            },
        );
        let node = class(&mut e, &[name_member], 0, 8, false);
        let first = e.mem.alloc(8);
        let first_name = text(&mut e, "first");
        e.mem.set_u32(first, first_name);
        let second = e.mem.alloc(8);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, first);
        e.mem.set_u32(pointers + 4, second);
        let list = member(
            &mut e,
            "nodes",
            Member {
                kind: 0x16,
                item: 0x14,
                struct_class: node,
                ..PLAIN
            },
        );
        e.call(0x00cf_6460, &args![list, pointers, 2i32, collector]);
        // Both objects are reported under the object's own name; the second one's name is null:
        // "unnamed".
        let reported = slot_calls(&log, 0x10);
        assert_eq!(reported.len(), 2);
        assert_eq!(reported[0], vec![node, first_name, first, 0]);
        assert_eq!(reported[1][0], node);
        assert_eq!(reported[1][1], UNNAMED);
        // A pointer array whose class has no "name" member reports under the array member's name.
        log.borrow_mut().clear();
        let plain = member(&mut e, "p", PLAIN);
        let anonymous = class(&mut e, &[plain], 0, 8, false);
        let pointer_list = member(
            &mut e,
            "anon",
            Member {
                kind: 0x16,
                item: 0x14,
                struct_class: anonymous,
                ..PLAIN
            },
        );
        let anon_name = e.mem.u32(pointer_list);
        e.call(0x00cf_6460, &args![pointer_list, pointers, 1i32, collector]);
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![vec![anonymous, anon_name, first, 0]]
        );
        // A pointer array without a struct class, other item types and a count of 0 report nothing.
        log.borrow_mut().clear();
        let classless = member(
            &mut e,
            "x",
            Member {
                kind: 0x16,
                item: 0x14,
                ..PLAIN
            },
        );
        e.call(0x00cf_6460, &args![classless, pointers, 1i32, collector]);
        let ints = member(
            &mut e,
            "y",
            Member {
                kind: 0x16,
                item: 0x05,
                ..PLAIN
            },
        );
        e.call(0x00cf_6460, &args![ints, pointers, 2i32, collector]);
        e.call(0x00cf_6460, &args![pointer_list, pointers, 0i32, collector]);
        e.call(
            0x00cf_6460,
            &args![pointer_list, pointers, -1i32, collector],
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn array_contents_of_structs_strings_and_string_pointers() {
        let (mut e, log, collector) = world();
        // Struct items: homogeneous reporting with the member's name and the struct class.
        let complex_member = member(
            &mut e,
            "c",
            Member {
                kind: 0x1d,
                ..PLAIN
            },
        );
        let inner = class(&mut e, &[complex_member], 0, 0x10, false);
        let structs = member(
            &mut e,
            "items",
            Member {
                kind: 0x16,
                item: 0x19,
                struct_class: inner,
                ..PLAIN
            },
        );
        let name = e.mem.u32(structs);
        e.call(0x00cf_6460, &args![structs, 0x5000u32, 2i32, collector]);
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![vec![inner, name, 0x5000, 1], vec![inner, name, 0x5010, 1]]
        );
        // Struct items without a struct class: nothing.
        log.borrow_mut().clear();
        let classless = member(
            &mut e,
            "z",
            Member {
                kind: 0x16,
                item: 0x19,
                ..PLAIN
            },
        );
        e.call(0x00cf_6460, &args![classless, 0x5000u32, 2i32, collector]);
        assert!(log.borrow().is_empty());
        // C strings: one `addChunk` each; a null string reports nothing.
        let strings = member(
            &mut e,
            "labels",
            Member {
                kind: 0x16,
                item: 0x1d,
                ..PLAIN
            },
        );
        let labels_name = e.mem.u32(strings);
        let (a, b) = (text(&mut e, "ab"), text(&mut e, "cde"));
        let table = e.mem.alloc(12);
        e.mem.set_u32(table, a);
        e.mem.set_u32(table + 4, 0);
        e.mem.set_u32(table + 8, b);
        e.call(0x00cf_6460, &args![strings, table, 3i32, collector]);
        assert_eq!(
            slot_calls(&log, 0x14),
            vec![
                vec![1, labels_name, a, 3, 0x13],
                vec![1, labels_name, b, 4, 0x14]
            ]
        );
        // hkStringPtr items: the collector's `addStringPtr` with each consecutive pointer.
        log.borrow_mut().clear();
        let pointers_member = member(
            &mut e,
            "names",
            Member {
                kind: 0x16,
                item: 0x21,
                ..PLAIN
            },
        );
        let names_name = e.mem.u32(pointers_member);
        e.call(
            0x00cf_6460,
            &args![pointers_member, 0x6000u32, 3i32, collector],
        );
        assert_eq!(
            slot_calls(&log, 0xffff),
            vec![
                vec![names_name, 0x6000],
                vec![names_name, 0x6004],
                vec![names_name, 0x6008]
            ]
        );
    }

    /// Runs `addClassContents` over a class of the single member `m` and a 0x60-byte object
    /// prepared by `setup`; returns the member's name pointer and the object.
    fn run_member(
        e: &mut Engine,
        collector: u32,
        m: Member,
        setup: impl FnOnce(&mut Engine, u32),
    ) -> (u32, u32) {
        let record = member(e, "m", m);
        let owner = class(e, &[record], 0, 0x60, false);
        let object = e.mem.alloc(0x60);
        setup(e, object);
        e.call(0x00cf_6580, &args![object, owner, collector]);
        (e.mem.u32(record), object)
    }

    /// A struct class with a complex member (so that its objects are reported one by one).
    fn complex_struct(e: &mut Engine, vtable: bool) -> u32 {
        let complex = member(
            e,
            "c",
            Member {
                kind: 0x1d,
                ..PLAIN
            },
        );
        class(e, &[complex], 0, 0x10, vtable)
    }

    #[test]
    fn class_contents_struct_pointer_and_variant_members() {
        let (mut e, log, collector) = world();
        let inner = complex_struct(&mut e, false);
        // An inline struct: `_addObjectContents` (flag 1) on the address inside the object.
        let (name, object) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x19,
                offset: 0x10,
                struct_class: inner,
                ..PLAIN
            },
            |_, _| {},
        );
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![vec![inner, name, object + 0x10, 1]]
        );
        // A pointer member to a struct: `_addObject` on the pointer's value; a null one, and a
        // pointer member to anything else, report nothing.
        log.borrow_mut().clear();
        let pointed = e.mem.alloc(8);
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x14,
                item: 0x19,
                offset: 0x14,
                struct_class: inner,
                ..PLAIN
            },
            |e, object| e.mem.set_u32(object + 0x14, pointed),
        );
        assert_eq!(slot_calls(&log, 0x10), vec![vec![inner, name, pointed, 0]]);
        log.borrow_mut().clear();
        run_member(
            &mut e,
            collector,
            Member {
                kind: 0x14,
                item: 0x19,
                offset: 0x14,
                struct_class: inner,
                ..PLAIN
            },
            |_, _| {},
        );
        run_member(
            &mut e,
            collector,
            Member {
                kind: 0x14,
                item: 0x05,
                offset: 0x14,
                struct_class: inner,
                ..PLAIN
            },
            |e, object| e.mem.set_u32(object + 0x14, pointed),
        );
        assert!(log.borrow().is_empty());
        // A variant: its object and class words, both set.
        let variant = |e: &mut Engine, object: u32, class: u32| {
            let (name, _) = run_member(
                e,
                collector,
                Member {
                    kind: 0x1c,
                    offset: 0x18,
                    ..PLAIN
                },
                |e, base| {
                    e.mem.set_u32(base + 0x18, object);
                    e.mem.set_u32(base + 0x1c, class);
                },
            );
            name
        };
        let name = variant(&mut e, pointed, inner);
        assert_eq!(slot_calls(&log, 0x10), vec![vec![inner, name, pointed, 0]]);
        log.borrow_mut().clear();
        variant(&mut e, 0, inner);
        variant(&mut e, pointed, 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn class_contents_skips_flagged_members_unknown_types_and_null_objects() {
        let (mut e, log, collector) = world();
        let inner = complex_struct(&mut e, false);
        run_member(
            &mut e,
            collector,
            Member {
                kind: 0x19,
                flags: 0x200,
                struct_class: inner,
                ..PLAIN
            },
            |_, _| {},
        );
        for kind in [0x15u8, 0x17, 0x18, 0x1e, 0x1f, 0x20, 0x22, 0x01] {
            run_member(&mut e, collector, Member { kind, ..PLAIN }, |_, _| {});
        }
        assert!(log.borrow().is_empty());
        // A null object, and a class without members, report nothing.
        let record = member(
            &mut e,
            "m",
            Member {
                kind: 0x19,
                struct_class: inner,
                ..PLAIN
            },
        );
        let owner = class(&mut e, &[record], 0, 0x60, false);
        e.call(0x00cf_6580, &args![0u32, owner, collector]);
        let empty = class(&mut e, &[], 0, 0x60, false);
        e.call(0x00cf_6580, &args![0x1000u32, empty, collector]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn class_contents_c_strings_and_string_pointers() {
        let (mut e, log, collector) = world();
        let hello = text(&mut e, "hello");
        let (a, b, c) = (text(&mut e, "a"), text(&mut e, "bb"), text(&mut e, "ccc"));
        // One C string (array size 1): one chunk.
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x1d,
                offset: 0x20,
                array_size: 1,
                ..PLAIN
            },
            |e, object| e.mem.set_u32(object + 0x20, hello),
        );
        assert_eq!(slot_calls(&log, 0x14), vec![vec![1, name, hello, 6, 0x16]]);
        // Three of them (array size 3): the accessor addresses the words one after another.
        log.borrow_mut().clear();
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x1d,
                offset: 0x20,
                array_size: 3,
                ..PLAIN
            },
            |e, object| {
                e.mem.set_u32(object + 0x20, a);
                e.mem.set_u32(object + 0x24, b);
                e.mem.set_u32(object + 0x28, c);
            },
        );
        assert_eq!(
            slot_calls(&log, 0x14),
            vec![
                vec![1, name, a, 2, 0x12],
                vec![1, name, b, 3, 0x13],
                vec![1, name, c, 4, 0x14]
            ]
        );
        // hkStringPtr members: one, then two consecutive ones.
        log.borrow_mut().clear();
        let (name, object) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x21,
                offset: 0x30,
                array_size: 1,
                ..PLAIN
            },
            |_, _| {},
        );
        assert_eq!(slot_calls(&log, 0xffff), vec![vec![name, object + 0x30]]);
        log.borrow_mut().clear();
        let (name, object) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x21,
                offset: 0x30,
                array_size: 2,
                ..PLAIN
            },
            |_, _| {},
        );
        assert_eq!(
            slot_calls(&log, 0xffff),
            vec![vec![name, object + 0x30], vec![name, object + 0x34]]
        );
    }

    #[test]
    fn class_contents_of_a_dynamic_array() {
        let (mut e, log, collector) = world();
        // An array of ints (item type 5, element size 4) with size 3 and capacity 8.
        let data = e.mem.alloc(16);
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x16,
                item: 0x05,
                offset: 0x28,
                ..PLAIN
            },
            |e, object| {
                e.mem.set_u32(object + 0x28, data);
                e.mem.set_u32(object + 0x2c, 3);
                e.mem.set_u32(object + 0x30, 8);
            },
        );
        // The allocator is asked about `data` with 8 * 4 bytes; the chunk is 3 * 4 bytes.
        assert_eq!(slot_calls(&log, 0x2c), vec![vec![data, 32]]);
        assert_eq!(slot_calls(&log, 0x14), vec![vec![3, name, data, 12, 0x30]]);
        // No group around a plain array.
        assert!(slot_calls(&log, 0x18).is_empty());
        // Storage the array does not own is skipped unless the collector's flag byte is set.
        log.borrow_mut().clear();
        let not_owned = |e: &mut Engine, object: u32| {
            e.mem.set_u32(object + 0x28, data);
            e.mem.set_u32(object + 0x2c, 3);
            e.mem.set_u32(object + 0x30, 0x8000_0008);
        };
        let array = Member {
            kind: 0x16,
            item: 0x05,
            offset: 0x28,
            ..PLAIN
        };
        run_member(&mut e, collector, array, not_owned);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(collector + 0x10, 1);
        let array = Member {
            kind: 0x16,
            item: 0x05,
            offset: 0x28,
            ..PLAIN
        };
        run_member(&mut e, collector, array, not_owned);
        assert_eq!(slot_calls(&log, 0x14).len(), 1);
        // An item type without a size: nothing.
        log.borrow_mut().clear();
        run_member(
            &mut e,
            collector,
            Member {
                kind: 0x16,
                item: 0,
                offset: 0x28,
                ..PLAIN
            },
            |e, object| e.mem.set_u32(object + 0x30, 8),
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn class_contents_of_a_pointer_array_is_grouped_under_the_array_name() {
        let (mut e, log, collector) = world();
        let inner = complex_struct(&mut e, false);
        let object_a = e.mem.alloc(8);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, object_a);
        e.mem.set_u32(pointers + 4, 0);
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x16,
                item: 0x14,
                offset: 0x28,
                struct_class: inner,
                ..PLAIN
            },
            |e, object| {
                e.mem.set_u32(object + 0x28, pointers);
                e.mem.set_u32(object + 0x2c, 2);
                e.mem.set_u32(object + 0x30, 2);
            },
        );
        let calls = log.borrow().clone();
        // Begin (the member's name), the array chunk named "array" (kind 3, 2 pointers of 4 bytes),
        // the allocator query before it, one object (the null pointer reports nothing), end.
        assert_eq!(calls[0], (0x18, vec![name]));
        assert_eq!(calls[1], (0x2c, vec![pointers, 8]));
        assert_eq!(
            calls[2],
            (0x14, vec![3, ARRAY_GROUP_NAME, pointers, 8, 0x18])
        );
        // The pointed object has no "name" member: it is reported under the member's name.
        assert_eq!(calls[3], (0x10, vec![inner, name, object_a, 0]));
        assert_eq!(calls[4], (0x1c, vec![]));
        assert_eq!(calls.len(), 5);
    }

    #[test]
    fn class_contents_of_simple_and_homogeneous_arrays_are_grouped() {
        let (mut e, log, collector) = world();
        let data = e.mem.alloc(16);
        // A simple array: (data, size 2) of ints.
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x1a,
                item: 0x05,
                offset: 0x34,
                ..PLAIN
            },
            |e, object| {
                e.mem.set_u32(object + 0x34, data);
                e.mem.set_u32(object + 0x38, 2);
            },
        );
        let calls = log.borrow().clone();
        assert_eq!(calls[0], (0x18, vec![name]));
        assert_eq!(calls[1], (0x2c, vec![data, 8]));
        assert_eq!(calls[2], (0x14, vec![3, name, data, 8, 0x18]));
        assert_eq!(calls[3], (0x1c, vec![]));
        assert_eq!(calls.len(), 4);
        // An item type with no size: nothing, not even the group.
        log.borrow_mut().clear();
        run_member(
            &mut e,
            collector,
            Member {
                kind: 0x1a,
                item: 0,
                offset: 0x34,
                ..PLAIN
            },
            |e, object| e.mem.set_u32(object + 0x38, 2),
        );
        assert!(log.borrow().is_empty());
        // A homogeneous array: (class, data, size 2); its class has a vtable and complex members.
        log.borrow_mut().clear();
        let inner = complex_struct(&mut e, true);
        let (name, _) = run_member(
            &mut e,
            collector,
            Member {
                kind: 0x1b,
                offset: 0x3c,
                ..PLAIN
            },
            |e, object| {
                e.mem.set_u32(object + 0x3c, inner);
                e.mem.set_u32(object + 0x40, data);
                e.mem.set_u32(object + 0x44, 2);
            },
        );
        let calls = log.borrow().clone();
        assert_eq!(calls[0], (0x18, vec![name]));
        assert_eq!(calls[1], (0x2c, vec![data, 0x20]));
        assert_eq!(calls[2], (0x14, vec![3, name, data, 0x20, 0x30]));
        assert_eq!(calls[3], (0xc, vec![name, data, 1]));
        assert_eq!(calls[4], (0xc, vec![name, data + 0x10, 1]));
        assert_eq!(calls[5], (0x1c, vec![]));
        assert_eq!(calls.len(), 6);
        // A missing class, data or size: nothing.
        for parts in [(0, data, 2), (inner, 0, 2), (inner, data, 0)] {
            log.borrow_mut().clear();
            run_member(
                &mut e,
                collector,
                Member {
                    kind: 0x1b,
                    offset: 0x3c,
                    ..PLAIN
                },
                |e, object| {
                    e.mem.set_u32(object + 0x3c, parts.0);
                    e.mem.set_u32(object + 0x40, parts.1);
                    e.mem.set_u32(object + 0x44, parts.2);
                },
            );
            assert!(log.borrow().is_empty());
        }
    }

    #[test]
    fn class_contents_all_walks_the_ancestors() {
        let (mut e, log, collector) = world();
        let inner = complex_struct(&mut e, false);
        let parent_member = member(
            &mut e,
            "parent_part",
            Member {
                kind: 0x19,
                offset: 0x10,
                struct_class: inner,
                ..PLAIN
            },
        );
        let child_member = member(
            &mut e,
            "child_part",
            Member {
                kind: 0x19,
                offset: 0x20,
                struct_class: inner,
                ..PLAIN
            },
        );
        let parent = class(&mut e, &[parent_member], 0, 0x60, false);
        let child = class(&mut e, &[child_member], parent, 0x60, false);
        let object = e.mem.alloc(0x60);
        e.call(0x00cf_6920, &args![object, child, collector]);
        let parent_name = e.mem.u32(parent_member);
        let child_name = e.mem.u32(child_member);
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![
                vec![inner, child_name, object + 0x20, 1],
                vec![inner, parent_name, object + 0x10, 1]
            ]
        );
        // A null object reports nothing at all.
        log.borrow_mut().clear();
        e.call(0x00cf_6920, &args![0u32, child, collector]);
        assert!(log.borrow().is_empty());
    }
}
