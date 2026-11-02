//! `Havok SDK/hkinspectprocess.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `hkInspectProcess` (the Havok visual debugger's inspect process) as the code uses it: the
// object has two parts. The process proper has the output archive at `+0x14`, the array of
// top-level objects (8-byte entries `{object, class}`: data `+0x20`, size `+0x24`, capacity
// word `+0x28`) and the context at `+0x2c` (whose word at `+0x4c`, when not null, is an
// object that maps an object to its real class through its virtual slot `0x10`). The
// methods marked "sub-object" are called with the pointer `+8` into the same object and
// subtract 8 to reach the process proper (`+8` has the input reader at `+8` of itself, the
// array of top-level objects at `+0x18` and `+0x1c`).

/// Callees outside this file. `hkOArchive` members (`thiscall` on the archive): `write32u`
/// (`00d60960`, one word), the one-byte write (`00d605d0`, one word), `write64u` (`00d609a0`,
/// low and high words) and `writeRaw` (`00d607b0`, data and byte count). `hkClass::hasVtable`
/// (`00d470f0`, `thiscall` on the class with a pointer to a byte that receives the answer;
/// answers that pointer).
const WRITE_U32: u32 = 0x00d6_0960;
const WRITE_BYTE: u32 = 0x00d6_05d0;
const WRITE_U64: u32 = 0x00d6_09a0;
const WRITE_RAW: u32 = 0x00d6_07b0;
const CLASS_HAS_VTABLE: u32 = 0x00d4_70f0;
/// The memory router getter (`00c85750`; its words at `+0xc` and `+0x10` are allocators), the
/// array growth `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, count, element
/// size) and `_reserveMore` (`00c90940`, cdecl: allocator, array, element size).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkClassMember::getArrayMemberSize` (`00d86b40`), `getStructClass` (`00d86910`) and
/// `hkClass::getMember` (`00d46f20`, one word) with its overload (`00d46f60`, one word).
const GET_ARRAY_MEMBER_SIZE: u32 = 0x00d8_6b40;
const GET_STRUCT_CLASS: u32 = 0x00d8_6910;
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;
const CLASS_GET_MEMBER_OV2: u32 = 0x00d4_6f60;
/// Reference counting of the context (`00c90510` add, `00c905b0` remove) and its listener
/// lists (`00cd13c0` add: callback, owner; `00cd1140` remove: callback), and the callback
/// the process registers (`00ddb5d0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const ADD_LISTENER: u32 = 0x00cd_13c0;
const REMOVE_LISTENER: u32 = 0x00cd_1140;
const LISTENER_CALLBACK: u32 = 0x00dd_b5d0;
/// `hkProcessFactory::registerProcess` (`00dd9380`, `thiscall` on the factory: name,
/// creation function), the factory pointer (`01269cd0`), the slot the process id goes to
/// (`01269cd4`), the process name (`010d7dd4`) and the creation function (`00ddb710`).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY_GLOBAL: u32 = 0x0126_9cd0;
const PROCESS_ID_GLOBAL: u32 = 0x0126_9cd4;
const PROCESS_NAME: u32 = 0x010d_7dd4;
const PROCESS_CREATOR: u32 = 0x00dd_b710;
/// The reader's `hkIArchive::readArrayGeneric` (`00dd8fe0`: buffer, size, count) and
/// `readRaw` (`00dd90d0`: buffer, count), `hkString::memCpy` (`00cb1260`, cdecl:
/// destination, source, count), `hkObjectSerialize::writeObject` (`00e0bd40`, cdecl),
/// `hkStructureLayout::hkStructureLayout_ov2` (`00dc1680`: the layout rules), the
/// stream writer's constructor (`00cc06f0`: array, flag; the engine map names it
/// `nullTerminate`) and destructor (`00cc0780`), and the layout rules (`010d6ae8`).
const READ_ARRAY_GENERIC: u32 = 0x00dd_8fe0;
const READ_RAW: u32 = 0x00dd_90d0;
const MEM_COPY: u32 = 0x00cb_1260;
const SERIALIZE_WRITE_OBJECT: u32 = 0x00e0_bd40;
const STRUCTURE_LAYOUT_CONSTRUCTOR: u32 = 0x00dc_1680;
const STREAM_WRITER_CONSTRUCTOR: u32 = 0x00cc_06f0;
const STREAM_WRITER_DESTRUCTOR: u32 = 0x00cc_0780;
const LAYOUT_RULES: u32 = 0x010d_6ae8;
/// The string and the four bytes `init` writes (`0126cae8`, `011b7b88` to `011b7b8b`), and the
/// object the writer treats specially (`0126cae8`).
const INIT_STRING: u32 = 0x0126_cae8;
const INIT_BYTES: u32 = 0x011b_7b88;
/// Virtual tables the constructor and the destructor store (`010d1508`, `010d7e0c`,
/// `010d7dec`, `0102e388`).
const BASE_VTABLE_AT_8: u32 = 0x010d_1508;
const PROCESS_VTABLE: u32 = 0x010d_7e0c;
const PROCESS_VTABLE_AT_8: u32 = 0x010d_7dec;
const BASE_VTABLE: u32 = 0x0102_e388;
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Gives back the data of the `hkArray` at `array` the way the inline code does: the router
/// is asked, the size is cleared and, unless the capacity word has the not-owned bit, the
/// data is freed through the router's allocator (slot `0x10`: data, `capacity * element_size`
/// bytes).
fn empty_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Appends `{object, class}` to the 8-byte entry array at `array` (growing it with
/// `_reserveMore` when full).
fn push_entry(e: &mut Engine, array: u32, object: u32, class: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let entry = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(entry, object);
    e.mem.set_u32(entry + 4, class);
}

/// The index of the first entry of the 8-byte entry array at `array` (data `+0`, size `+4`)
/// whose first word is `object`, or the size when there is none.
fn find_entry(e: &Engine, array: u32, object: u32) -> i32 {
    let size = e.mem.i32(array + 4);
    let mut index = 0i32;
    if size > 0 {
        let mut entry = e.mem.u32(array);
        loop {
            if e.mem.u32(entry) == object {
                break;
            }
            index += 1;
            entry = entry.wrapping_add(8);
            if index >= size {
                break;
            }
        }
    }
    index
}

/// Removes entry `index` by copying the last entry over it (the size falls by one first).
fn swap_remove_entry(e: &mut Engine, array: u32, index: i32) {
    let size = e.mem.u32(array + 4).wrapping_sub(1);
    e.mem.set_u32(array + 4, size);
    if size as i32 != index {
        let data = e.mem.u32(array);
        let last = data.wrapping_add(size.wrapping_mul(8));
        let target = data.wrapping_add((index as u32).wrapping_mul(8));
        for word in 0..2 {
            let value = e.mem.u32(last + 4 * word);
            e.mem.set_u32(target + 4 * word, value);
        }
    }
}

/// Asks whether `class` has a virtual table (`hkClass::hasVtable`, whose answer byte goes
/// to a local; the exe uses its argument slot) and, when it does, the context's object
/// (`+0x4c`, virtual slot `0x10`) for the real class of `object`; `class` is kept when
/// there is none.
fn resolve_class(e: &mut Engine, process: u32, object: u32, class: u32) -> u32 {
    let context = e.mem.u32(process + 0x2c);
    if e.mem.u32(context + 0x4c) == 0 {
        return class;
    }
    let has_vtable = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), class);
        let answer = e.call(CLASS_HAS_VTABLE, &args![class, slot]).u32();
        e.mem.u8(answer) != 0
    });
    if !has_vtable {
        return class;
    }
    let registry = e.mem.u32(e.mem.u32(process + 0x2c) + 0x4c);
    let found = e.vcall(registry, 0x10, &args![object]).u32();
    if found == 0 {
        class
    } else {
        found
    }
}

// Translated from 00ddb310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::addTopLevelObject` (Xbox PDB), `thiscall`, `RET 0xc` (`object`, `class`,
/// `extra`): writes the command 0x19 (`write32u`) to the archive at `+0x14`, resolves the
/// class (when the context at `+0x2c` has its object at `+0x4c` and the class has a virtual
/// table, that object's slot `0x10` answers the real class of `object`, `class` when it
/// answers 0), then the byte 0x21 and the three values `object`, the class and `extra` as
/// 64-bit words. Answers 0x19.
pub fn hk_inspect_process_add_top_level_object(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
    extra: u32,
) -> u32 {
    let at = this.addr();
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_U32, &args![archive, 0x19u32]);
    let resolved = resolve_class(e, at, object, class);
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_BYTE, &args![archive, 0x21u32]);
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_U64, &args![archive, object, 0u32]);
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_U64, &args![archive, resolved, 0u32]);
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_U64, &args![archive, extra, 0u32]);
    0x19
}

// Translated from 00ddb3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::init` (Xbox PDB), `thiscall` on the sub-object (`this` is the process
/// plus 8): nothing when the word at `+0x24` is 0. Otherwise writes the command 0xd, the byte
/// 0x20, the 64-bit word `0126cae8` and the four bytes at `011b7b88` to the archive at `+0xc`,
/// then walks the 12-byte entries of the array at the context's `+0x28` (that `+0x24` object's
/// data and its size at `+0x2c`; the size and data are re-read every step) calling
/// `addTopLevelObject` on the process proper (`this - 8`) with each entry's three words.
pub fn hk_inspect_process_init(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let context = e.mem.u32(at + 0x24);
    if context == 0 {
        return;
    }
    let archive = e.mem.u32(at + 0xc);
    e.call(WRITE_U32, &args![archive, 0xdu32]);
    let archive = e.mem.u32(at + 0xc);
    e.call(WRITE_BYTE, &args![archive, 0x20u32]);
    let archive = e.mem.u32(at + 0xc);
    e.call(WRITE_U64, &args![archive, INIT_STRING, 0u32]);
    for offset in 0..4 {
        let byte = u32::from(e.mem.u8(INIT_BYTES + offset));
        let archive = e.mem.u32(at + 0xc);
        e.call(WRITE_BYTE, &args![archive, byte]);
    }
    let list = e.mem.u32(at + 0x24) + 0x28;
    let mut index = 0i32;
    if e.mem.i32(list + 4) > 0 {
        let mut offset = 0u32;
        loop {
            let data = e.mem.u32(list);
            let entry = data.wrapping_add(offset);
            let object = e.mem.u32(entry);
            let class = e.mem.u32(entry + 4);
            let extra = e.mem.u32(entry + 8);
            hk_inspect_process_add_top_level_object(e, Ptr::new(at - 8), object, class, extra);
            index += 1;
            offset = offset.wrapping_add(0xc);
            if index >= e.mem.i32(list + 4) {
                break;
            }
        }
    }
}

// Translated from 00ddb450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_FindMemberItem` (Xbox PDB). Inputs are in registers, so the register values come first:
/// `member` (`EAX`, an `hkClassMember`), `address` (`EDX`), `kind` (`ECX`, a member type
/// number); then the stack words `path` (a pointer to a record whose first word points at an
/// array of 16-bit path steps) and `start` (the index of the step before the first to use,
/// the code reads `steps[start + 1]` next). Each step moves on by the current kind (a table
/// of 32 kinds: kinds 1 to 13, 24 and 30 to 32 end the walk and answer the
/// address; kinds 14 to 18 add `step * 16` and become kind 12; kinds 22 and 26 are arrays:
/// the address becomes the pointer stored there plus `step` times
/// `hkClassMember::getArrayMemberSize`, and the kind becomes the member's byte at `+0xd`;
/// kind 25 is a struct: the member must be non-null with a non-null struct class (`+4`), the
/// next member is `hkClass::getMember(step)`, its offset (`+0x12`) is added to the address
/// and its kind is its byte at `+0xc`; every other kind answers 0). Kind 0 or above 32
/// answers 0.
pub fn find_member_item(
    e: &mut Engine,
    member: u32,
    address: u32,
    kind: u32,
    path: u32,
    start: u32,
) -> u32 {
    let mut member = member;
    let mut address = address;
    let mut kind = kind;
    let mut offset = start.wrapping_add(start);
    loop {
        let index = kind.wrapping_sub(1);
        if index > 0x1f {
            return 0;
        }
        // The compiler's switch on the member kind less 1 (`0..=0x1f`): the case of each kind.
        let case = match index {
            0..=12 | 23 | 29..=31 => 0,
            13..=17 => 1,
            18 | 20 | 22 | 26..=28 => 5,
            19 => 2,
            21 | 25 => 3,
            24 => 4,
            _ => unreachable!(),
        };
        match case {
            0 => return address,
            1 => {
                let steps = e.mem.u32(path);
                let step = u32::from(e.mem.u16(steps.wrapping_add(offset).wrapping_add(2)));
                address = (step << 4).wrapping_add(address);
                kind = 0xc;
                offset = offset.wrapping_add(2);
            }
            3 => {
                let steps = e.mem.u32(path);
                let step = u32::from(e.mem.u16(steps.wrapping_add(offset).wrapping_add(2)));
                let size = e.call(GET_ARRAY_MEMBER_SIZE, &args![member]).u32();
                let pointer = e.mem.u32(address);
                address = size.wrapping_mul(step).wrapping_add(pointer);
                kind = u32::from(e.mem.u8(member + 0xd));
                offset = offset.wrapping_add(2);
            }
            4 => {
                if member == 0 || e.mem.u32(member + 4) == 0 {
                    return 0;
                }
                let class = e.call(GET_STRUCT_CLASS, &args![member]).u32();
                if class == 0 {
                    return 0;
                }
                let steps = e.mem.u32(path);
                let step = u32::from(e.mem.u16(steps.wrapping_add(offset).wrapping_add(2)));
                member = e.call(CLASS_GET_MEMBER, &args![class, step]).u32();
                let member_offset = u32::from(e.mem.u16(member + 0x12));
                kind = u32::from(e.mem.u8(member + 0xc));
                offset = offset.wrapping_add(2);
                address = address.wrapping_add(member_offset);
            }
            _ => return 0,
        }
    }
}

// Translated from 00ddb550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::removeTopLevelObject` (Xbox PDB), `thiscall`, `RET 4` (`object`):
/// writes the command 9 (`write32u`), the byte 0x22 and `object` as a 64-bit word to the
/// archive at `+0x14`, then removes the entry of `object` from the top-level array (`+0x20`,
/// size `+0x24`) by copying the last entry over it (nothing when it is not there). Answers 9.
pub fn hk_inspect_process_remove_top_level_object(e: &mut Engine, this: Ptr, object: u32) -> u32 {
    let at = this.addr();
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_U32, &args![archive, 9u32]);
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_BYTE, &args![archive, 0x22u32]);
    let archive = e.mem.u32(at + 0x14);
    e.call(WRITE_U64, &args![archive, object, 0u32]);
    let size = e.mem.i32(at + 0x24);
    if size > 0 {
        let index = find_entry(e, at + 0x20, object);
        if index < size {
            swap_remove_entry(e, at + 0x20, index);
        }
    }
    9
}

// Translated from 00ddb600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `hkInspectProcess` (unnamed in the engine map; the class is the one
/// `00ddb670` and the process registration belong to), `thiscall`, `RET 4` (`contexts`, an
/// `hkArray` of context pointers): stores 1 in the word at `+6` and the byte at `+0xc`, the
/// virtual tables (`010d7e0c` at `+0`, `010d7dec` at `+8`), clears the top-level array
/// (`+0x20`, `+0x24`, capacity word `0x80000000` at `+0x28`) and takes the first context
/// (the word at `+4` of the object the first array element points to; 0 when the array is
/// empty) into `+0x2c`. A context gets a reference (`00c90510`) and the process as a listener
/// (`00cd13c0`: callback `00ddb5d0`, the process). Answers `this`.
pub fn fn_00ddb600(e: &mut Engine, this: Ptr, contexts: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u8(at + 0xc, 1);
    e.mem.set_u32(at + 8, BASE_VTABLE_AT_8);
    e.mem.set_u32(at, PROCESS_VTABLE);
    e.mem.set_u32(at + 8, PROCESS_VTABLE_AT_8);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u32(at + 0x24, 0);
    e.mem.set_u32(at + 0x28, NOT_OWNED);
    let context = if e.mem.i32(contexts + 4) > 0 {
        let first = e.mem.u32(e.mem.u32(contexts));
        e.mem.u32(first + 4)
    } else {
        0
    };
    e.mem.set_u32(at + 0x2c, context);
    if context != 0 {
        e.call(ADD_REFERENCE, &args![context]);
        let context = e.mem.u32(at + 0x2c);
        e.call(ADD_LISTENER, &args![context, LISTENER_CALLBACK, this]);
    }
    this
}

// Translated from 00ddb670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `hkInspectProcess` (unnamed in the engine map), `thiscall`: stores the
/// virtual tables (`010d7e0c`, `010d7dec`); when the context (`+0x2c`) is not null, removes
/// every top-level object of the context (its 12-byte entries at `+0x28`, count at `+0x2c`,
/// first word of each; `removeTopLevelObject`), removes the listener (`00cd1140`) and the
/// reference (`00c905b0`). Frees the top-level array (`+0x20`; 8-byte entries) and finally
/// stores the virtual tables `010d1508` at `+8` and `0102e388` at `+0`.
pub fn fn_00ddb670(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let context = e.mem.u32(at + 0x2c);
    e.mem.set_u32(at, PROCESS_VTABLE);
    e.mem.set_u32(at + 8, PROCESS_VTABLE_AT_8);
    if context != 0 {
        let mut index = 0i32;
        if e.mem.i32(context + 0x2c) > 0 {
            let mut offset = 0u32;
            loop {
                let entries = e.mem.u32(context + 0x28);
                let object = e.mem.u32(entries.wrapping_add(offset));
                hk_inspect_process_remove_top_level_object(e, this, object);
                index += 1;
                offset = offset.wrapping_add(0xc);
                if index >= e.mem.i32(context + 0x2c) {
                    break;
                }
            }
        }
        let current = e.mem.u32(at + 0x2c);
        e.call(REMOVE_LISTENER, &args![current, LISTENER_CALLBACK]);
        let current = e.mem.u32(at + 0x2c);
        e.call(REMOVE_REFERENCE, &args![current]);
    }
    empty_array(e, at + 0x20, 8);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u32(at + 0x28, NOT_OWNED);
    e.mem.set_u32(at + 8, BASE_VTABLE_AT_8);
    e.mem.set_u32(at, BASE_VTABLE);
}

// Translated from 00ddb750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::registerProcess` (Xbox PDB; a static initializer): registers the process
/// with the factory at `01269cd0` (`hkProcessFactory::registerProcess`: the name `010d7dd4`
/// and the creation function `00ddb710`) and keeps the answer in the word at `01269cd4`.
pub fn hk_inspect_process_register_process(e: &mut Engine) {
    let factory = e.global::<u32>(PROCESS_FACTORY_GLOBAL);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATOR],
        )
        .u32();
    e.set_global(PROCESS_ID_GLOBAL, id);
}

// Translated from 00ddb770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::writeObject` (Xbox PDB), `thiscall`, `RET 0x10` (`object`, `class`,
/// `flag`, `visited`; `flag`'s low byte is the recursion flag, the other bytes are what the
/// caller's stack held): serialises `object` and, with the flag set, its children. The class
/// is resolved as in `addTopLevelObject`. With the flag set, `visited` (an array of 8-byte
/// `{object, class}` entries) is searched for `object`: when found the answer is 0; otherwise
/// the entry is appended. The object is then written (`hkObjectSerialize::writeObject`,
/// `00e0bd40`, with a stream writer over a local `hkArray<char>`, a layout built from the
/// rules at `010d6ae8` and a local array of 0x18-byte child records). When that writes
/// exactly the bytes the array holds and at least 1, the archive at `+0x14` gets the length
/// plus 1 (`write32u`), the byte 0x24 and the bytes, and with the flag set every child
/// record's pair (words at `+8` and `+0x10`) is written recursively with the flag low byte 1
/// (the high bytes of that word are the child object's); the answer is the sum of the
/// lengths plus 1. Any other outcome answers -1. The local arrays are freed either way.
pub fn hk_inspect_process_write_object(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
    flag: u32,
    visited: u32,
) -> u32 {
    let at = this.addr();
    let recursion = flag & 0xff != 0;
    e.with_stack(4, |e, layout| {
        e.call(STRUCTURE_LAYOUT_CONSTRUCTOR, &args![layout, LAYOUT_RULES]);
        // The child list: data, size, capacity word.
        e.with_stack(0xc, |e, children| {
            let list = children.addr();
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 8, NOT_OWNED);
            let resolved = resolve_class(e, at, object, class);
            if recursion {
                let index = find_entry(e, visited, object);
                let size = e.mem.i32(visited + 4);
                if index < size {
                    empty_array(e, list, 0x18);
                    return 0;
                }
                push_entry(e, visited, object, resolved);
            }
            // The character array the writer fills, and the writer over it.
            e.with_stack(0xc, |e, text| {
                let characters = text.addr();
                e.mem.set_u32(characters, 0);
                e.mem.set_u32(characters + 4, 0);
                e.mem.set_u32(characters + 8, NOT_OWNED);
                e.with_stack(0x14, |e, writer| {
                    e.call(STREAM_WRITER_CONSTRUCTOR, &args![writer, characters, 1u32]);
                    let written = e
                        .call(
                            SERIALIZE_WRITE_OBJECT,
                            &args![writer, layout, object, 0u32, object, resolved, children],
                        )
                        .i32();
                    let text_size = e.mem.i32(characters + 4);
                    let result = if written == text_size && written >= 1 {
                        let mut total = (written + 1) as u32;
                        let archive = e.mem.u32(at + 0x14);
                        e.call(WRITE_U32, &args![archive, total]);
                        let archive = e.mem.u32(at + 0x14);
                        e.call(WRITE_BYTE, &args![archive, 0x24u32]);
                        let data = e.mem.u32(characters);
                        let archive = e.mem.u32(at + 0x14);
                        e.call(WRITE_RAW, &args![archive, data, text_size as u32]);
                        if recursion && e.mem.i32(list + 4) > 0 {
                            let mut index = 0i32;
                            let mut offset = 0u32;
                            loop {
                                let records = e.mem.u32(list);
                                let child = e.mem.u32(records.wrapping_add(offset) + 8);
                                let child_class = e.mem.u32(records.wrapping_add(offset) + 0x10);
                                let child_flag = (child & 0xffff_ff00) | 1;
                                let sub = hk_inspect_process_write_object(
                                    e,
                                    this,
                                    child,
                                    child_class,
                                    child_flag,
                                    visited,
                                );
                                total = total.wrapping_add(sub);
                                index += 1;
                                offset = offset.wrapping_add(0x18);
                                if index >= e.mem.i32(list + 4) {
                                    break;
                                }
                            }
                        }
                        total
                    } else {
                        u32::MAX
                    };
                    e.call(STREAM_WRITER_DESTRUCTOR, &args![writer]);
                    empty_array(e, characters, 1);
                    e.mem.set_u32(characters, 0);
                    e.mem.set_u32(characters + 8, NOT_OWNED);
                    empty_array(e, list, 0x18);
                    result
                })
            })
        })
    })
}

// Translated from 00ddba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::step` (Xbox PDB), `thiscall` on the sub-object (`this` is the process
/// plus 8), `RET 4` (the time step, never read): for each top-level entry (the array at
/// `+0x18`, size `+0x1c`, re-read every step) calls `writeObject` on the process proper
/// (`this - 8`) with the entry's object and class, a flag word whose low byte is 0 (the high
/// bytes are those of the address of the local visited array) and that local array of 8-byte
/// entries, whose size is cleared before each call. The array is freed at the end. The code
/// keeps a growth step in front of every call that its own test (a masked capacity word is
/// never negative) makes unreachable; it is translated below as a branch that does not run.
pub fn hk_inspect_process_step(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let at = this.addr();
    e.with_stack(0xc, |e, visited| {
        let list = visited.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, NOT_OWNED);
        let count = e.mem.i32(at + 0x1c);
        if count > 0 {
            let mut index = 0u32;
            loop {
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                let capacity = e.mem.u32(list + 8) & CAPACITY_MASK;
                if (capacity as i32) < 0 {
                    // Never taken: the masked capacity is not negative.
                    let doubled = capacity.wrapping_add(capacity);
                    let keep = u32::from((doubled as i32) <= 0).wrapping_sub(1);
                    e.call(ARRAY_RESERVE, &args![allocator, list, doubled & keep, 8u32]);
                }
                e.mem.set_u32(list + 4, 0);
                let entries = e.mem.u32(at + 0x18);
                let entry = entries.wrapping_add(index.wrapping_mul(8));
                let object = e.mem.u32(entry);
                let class = e.mem.u32(entry + 4);
                hk_inspect_process_write_object(
                    e,
                    Ptr::new(at - 8),
                    object,
                    class,
                    list & 0xffff_ff00,
                    list,
                );
                index += 1;
                if (index as i32) >= e.mem.i32(at + 0x1c) {
                    break;
                }
            }
        }
        empty_array(e, list, 8);
    });
}

// Translated from 00ddbae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInspectProcess::consumeCommand` (Xbox PDB), `thiscall` on the sub-object (`this` is
/// the process plus 8: the reader is its word at `+8`, the top-level array at `+0x18`),
/// `RET 4` (`command`, a byte). Command 0x25 reads a 64-bit word, another 64-bit word, a
/// 16-bit count `n` and `n` 16-bit path steps into a local array (allocated through the
/// router's allocator slot `0xc`, then grown to `n` entries), then a 32-bit count `m` and
/// `m` bytes (`readRaw`) into another local array; when both 64-bit words are non-zero, `n`
/// and `m` are positive, it resolves the member the steps lead to (the first step through
/// `hkClass::getMember_ov2` on the second word as a class, then `_FindMemberItem` from the
/// first word plus the member's offset) and copies the `m` bytes there (`hkString::memCpy`);
/// both local arrays are freed. Command 0x23 reads two 64-bit words and a byte `flags`;
/// when both words are non-zero it writes the object (`writeObject` on the process proper
/// with the recursion flag set when the second word is the string at `0126cae8` or bit 1 of
/// `flags` is set, and a local visited array) and then, by bit 0 of `flags`, adds the object
/// to the top-level array (when it is not there) or removes it (when it is); the visited
/// array is freed. Every other command does nothing.
pub fn hk_inspect_process_consume_command(e: &mut Engine, this: Ptr, command: u8) {
    let at = this.addr();
    if command == 0x23 {
        consume_object_command(e, at);
    } else if command == 0x25 {
        consume_member_command(e, at);
    }
}

/// Reads `count` items of `size` bytes into `buffer` from the reader at `process + 8`
/// (`hkIArchive::readArrayGeneric`).
fn read_items(e: &mut Engine, process: u32, buffer: u32, size: u32, count: u32) {
    let reader = e.mem.u32(process + 8);
    e.call(READ_ARRAY_GENERIC, &args![reader, buffer, size, count]);
}

/// Reads a 64-bit word at the reader into a local and answers its low and high words.
fn read_u64(e: &mut Engine, process: u32) -> (u32, u32) {
    e.with_stack(8, |e, buffer| {
        read_items(e, process, buffer.addr(), 8, 1);
        (e.mem.u32(buffer.addr()), e.mem.u32(buffer.addr() + 4))
    })
}

fn consume_member_command(e: &mut Engine, at: u32) {
    let (object, object_high) = read_u64(e, at);
    let (class, class_high) = read_u64(e, at);
    let steps = e.with_stack(4, |e, buffer| {
        read_items(e, at, buffer.addr(), 2, 1);
        u32::from(e.mem.u16(buffer.addr()))
    });
    // The path array: allocated through the router's allocator (slot 0xc takes a pointer to
    // the byte count and answers the block; the count is rounded in place).
    e.with_stack(0xc, |e, path| {
        let path_array = path.addr();
        e.mem.set_u32(path_array, 0);
        e.mem.set_u32(path_array + 4, 0);
        e.mem.set_u32(path_array + 8, NOT_OWNED);
        let mut capacity = steps;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let data = if steps != 0 {
            e.with_stack(4, |e, bytes| {
                e.mem.set_u32(bytes.addr(), steps.wrapping_add(steps));
                let block = e.vcall(allocator, 0xc, &args![bytes]).u32();
                let rounded = e.mem.i32(bytes.addr());
                capacity = (rounded.wrapping_sub(rounded >> 31) >> 1) as u32;
                block
            })
        } else {
            0
        };
        let flags = if capacity == 0 { NOT_OWNED } else { capacity };
        e.mem.set_u32(path_array, data);
        e.mem.set_u32(path_array + 4, capacity);
        e.mem.set_u32(path_array + 8, flags);
        grow_array(e, path_array, steps, 2);
        e.mem.set_u32(path_array + 4, steps);
        let mut index = 0u32;
        if (steps as i32) > 0 {
            loop {
                let step = e.with_stack(4, |e, buffer| {
                    read_items(e, at, buffer.addr(), 2, 1);
                    e.mem.u16(buffer.addr())
                });
                let data = e.mem.u32(path_array);
                e.mem
                    .set_u16(data.wrapping_add(index.wrapping_mul(2)), step);
                index += 1;
                if index >= steps {
                    break;
                }
            }
        }
        let value_size = e.with_stack(4, |e, buffer| {
            read_items(e, at, buffer.addr(), 4, 1);
            e.mem.u32(buffer.addr())
        });
        e.with_stack(0xc, |e, value| {
            let value_array = value.addr();
            e.mem.set_u32(value_array, 0);
            e.mem.set_u32(value_array + 4, 0);
            e.mem.set_u32(value_array + 8, NOT_OWNED);
            let mut capacity = value_size;
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let data = if value_size != 0 {
                e.with_stack(4, |e, bytes| {
                    e.mem.set_u32(bytes.addr(), value_size);
                    let block = e.vcall(allocator, 0xc, &args![bytes]).u32();
                    capacity = e.mem.u32(bytes.addr());
                    block
                })
            } else {
                0
            };
            let flags = if capacity == 0 { NOT_OWNED } else { capacity };
            e.mem.set_u32(value_array, data);
            e.mem.set_u32(value_array + 4, capacity);
            e.mem.set_u32(value_array + 8, flags);
            grow_array(e, value_array, value_size, 1);
            let reader = e.mem.u32(at + 8);
            let buffer = e.mem.u32(value_array);
            e.mem.set_u32(value_array + 4, value_size);
            e.call(READ_RAW, &args![reader, buffer, value_size]);
            let object_set = object_high > 0 || object > 0;
            let class_set = class_high > 0 || class > 0;
            if object_set && class_set && (e.mem.i32(path_array + 4) > 0) && value_size > 0 {
                let first = u32::from(e.mem.u16(e.mem.u32(path_array)));
                let member = e.call(CLASS_GET_MEMBER_OV2, &args![class, first]).u32();
                let offset = u32::from(e.mem.u16(member + 0x12));
                let kind = u32::from(e.mem.u8(member + 0xc));
                let found =
                    find_member_item(e, member, offset.wrapping_add(object), kind, path_array, 0);
                if found != 0 {
                    let source = e.mem.u32(value_array);
                    e.call(MEM_COPY, &args![found, source, value_size]);
                }
            }
            empty_array(e, value_array, 1);
            e.mem.set_u32(value_array, 0);
            e.mem.set_u32(value_array + 8, NOT_OWNED);
        });
        empty_array(e, path_array, 2);
    });
}

/// `hkArray::reserve` growth as the inline code does it: when the capacity (masked) is below
/// `wanted`, `_reserve` with the router's allocator, twice the capacity or `wanted` when that
/// is larger, and `element_size` bytes per entry.
fn grow_array(e: &mut Engine, array: u32, wanted: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted as i32 {
        let mut grown = capacity.wrapping_add(capacity);
        if (wanted as i32) >= grown {
            grown = wanted as i32;
        }
        e.call(ARRAY_RESERVE, &args![allocator, array, grown, element_size]);
    }
}

fn consume_object_command(e: &mut Engine, at: u32) {
    let (object, object_high) = read_u64(e, at);
    let (class, class_high) = read_u64(e, at);
    let flags = e.with_stack(4, |e, buffer| {
        read_items(e, at, buffer.addr(), 1, 1);
        e.mem.u8(buffer.addr())
    });
    if !(object_high > 0 || object > 0) || !(class_high > 0 || class > 0) {
        return;
    }
    e.with_stack(0xc, |e, visited| {
        let list = visited.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, NOT_OWNED);
        let recursion = class == INIT_STRING || (flags & 2) != 0;
        hk_inspect_process_write_object(
            e,
            Ptr::new(at - 8),
            object,
            class,
            (list & 0xffff_ff00) | u32::from(recursion),
            list,
        );
        let top_level = at + 0x18;
        let size = e.mem.i32(at + 0x1c);
        let index = find_entry(e, top_level, object);
        let present = index != size;
        if flags & 1 != 0 {
            if !present {
                push_entry(e, top_level, object, class);
            }
        } else if present {
            swap_remove_entry(e, top_level, index);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(list + 8);
        e.mem.set_u32(list + 4, 0);
        if (capacity as i32) >= 0 {
            let bytes = (capacity & CAPACITY_MASK).wrapping_mul(8);
            let data = e.mem.u32(list);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00ddb5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words `first`, `second`, `flag`, `fourth`, `target`;
/// descriptions, not confirmed; code in two registration functions stores its address, at
/// `00ddb6ae` and `00ddb656`): a forwarder. With a non-zero low byte of `flag` it runs `00ddb310`
/// on `target` (`thiscall`, three stack words `(first, second, fourth)`); with zero it runs
/// `00ddb550` on `target` (`thiscall`, one stack word, `first`). Answers what the callee
/// answers.
pub fn fn_00ddb5d0(
    e: &mut Engine,
    first: u32,
    second: u32,
    flag: u8,
    fourth: u32,
    target: Ptr,
) -> u32 {
    if flag != 0 {
        e.call(0x00dd_b310, &args![target, first, second, fourth])
            .u32()
    } else {
        e.call(0x00dd_b550, &args![target, first]).u32()
    }
}

// Translated from 00ddb710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `argument`; the code in `registerProcess` at
/// `00ddb756` stores its address): the create function of a server process. Allocates 0x30 bytes
/// from the memory router's allocator (virtual slot 4), stores the size in the 16-bit word at
/// `+4` and runs `00ddb600` on the block (`thiscall`, one stack word, `argument`). Answers that
/// object plus 8, or 0 when `00ddb600` answers null.
pub fn fn_00ddb710(e: &mut Engine, argument: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(block.wrapping_add(4), 0x30);
    let object = e.call(0x00dd_b600, &args![block, argument]).u32();
    if object == 0 {
        return 0;
    }
    object.wrapping_add(8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ddb5d0, fn_00ddb5d0(u32, u32, u8, u32, Ptr) -> u32),
        entry!(0x00ddb710, fn_00ddb710(u32) -> u32),
        entry!(0x00ddb310, hk_inspect_process_add_top_level_object(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ddb3a0, hk_inspect_process_init(Ptr)),
        entry!(0x00ddb450, find_member_item(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ddb550, hk_inspect_process_remove_top_level_object(Ptr, u32) -> u32),
        entry!(0x00ddb600, fn_00ddb600(Ptr, u32) -> Ptr),
        entry!(0x00ddb670, fn_00ddb670(Ptr)),
        entry!(0x00ddb750, hk_inspect_process_register_process()),
        entry!(0x00ddb770, hk_inspect_process_write_object(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00ddba20, hk_inspect_process_step(Ptr, u32)),
        entry!(0x00ddbae0, hk_inspect_process_consume_command(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn calls_of_00ddb5d0(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00ddb5d0_with_the_flag_runs_00ddb310_with_three_words() {
        let mut e = Engine::new();
        e.register(0x00dd_b310, |_, _| Ret {
            eax: 0x31,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00dd_b5d0, &args![1u32, 2u32, 0x101u32, 4u32, 0x5000u32])
            .u32();
        assert_eq!(answer, 0x31);
        assert_eq!(
            calls_of_00ddb5d0(&e, 0x00dd_b310),
            vec![vec![0x5000, 1, 2, 4]]
        );
    }

    #[test]
    fn fn_00ddb5d0_without_the_flag_runs_00ddb550_with_one_word() {
        let mut e = Engine::new();
        e.register(0x00dd_b550, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        // Only the low byte of the third word is the flag.
        let answer = e
            .call(0x00dd_b5d0, &args![1u32, 2u32, 0x100u32, 4u32, 0x5000u32])
            .u32();
        assert_eq!(answer, 0x55);
        assert_eq!(calls_of_00ddb5d0(&e, 0x00dd_b550), vec![vec![0x5000, 1]]);
    }

    /// A router whose allocator hands out `block` (mapped) from slot 4.
    fn router_for_00ddb710(e: &mut Engine, block: u32) {
        e.map(block, 0x100);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| {
            assert_eq!(a[1], 0x30);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_00ddb710_answers_the_object_plus_eight() {
        let mut e = Engine::new();
        router_for_00ddb710(&mut e, 0x0800_0000);
        e.register(0x00dd_b600, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00dd_b710, &args![0x77u32]).u32();
        assert_eq!(answer, 0x0800_0008);
        assert_eq!(e.mem.u16(0x0800_0004), 0x30);
        assert_eq!(
            calls_of_00ddb5d0(&e, 0x00dd_b600),
            vec![vec![0x0800_0000, 0x77]]
        );
    }

    #[test]
    fn fn_00ddb710_answers_zero_when_the_constructor_answers_null() {
        let mut e = Engine::new();
        router_for_00ddb710(&mut e, 0x0800_0000);
        e.register(0x00dd_b600, |_, _| Ret::default());
        assert_eq!(e.call(0x00dd_b710, &args![0x77u32]).u32(), 0);
    }

    const ARCHIVE: u32 = 0x4400_6000;
    const ALLOCATOR_VTABLE: u32 = 0x4400_6100;
    const FREE_TARGET: u32 = 0x4400_6001;
    const ALLOC_TARGET: u32 = 0x4400_6002;
    const REGISTRY_VTABLE: u32 = 0x4400_6200;
    const REGISTRY_TARGET: u32 = 0x4400_6003;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Frees = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An engine whose archive functions record their calls (address, arguments).
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        for address in [WRITE_U32, WRITE_BYTE, WRITE_U64, WRITE_RAW] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log)
    }

    /// A router whose allocator (`+0x10`) frees through `FREE_TARGET` (logged in the returned
    /// list as (data, bytes)) and allocates through `ALLOC_TARGET`.
    fn memory(e: &mut Engine) -> Frees {
        let frees: Frees = Rc::default();
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, 0, ALLOC_TARGET, FREE_TARGET]);
        let sink = Rc::clone(&frees);
        e.register_double(FREE_TARGET, move |_, a| {
            sink.borrow_mut().push(a[1..].to_vec());
            Ret::default()
        });
        e.register(ALLOC_TARGET, |e, a| Ret {
            eax: e.mem.alloc(e.mem.u32(a[1])),
            ..Ret::default()
        });
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let data = e.mem.alloc(8 * 8);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        frees
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A process object (0x60 bytes) whose archive is `ARCHIVE` and whose context (`+0x2c`)
    /// has the given object at `+0x4c`.
    fn process(e: &mut Engine, registry: u32) -> u32 {
        let process = e.mem.alloc(0x60);
        let context = e.mem.alloc(0x60);
        e.mem.set_u32(context + 0x4c, registry);
        e.mem.set_u32(process + 0x14, ARCHIVE);
        e.mem.set_u32(process + 0x2c, context);
        process
    }

    #[test]
    fn a_top_level_object_is_announced_with_its_real_class() {
        let (mut e, log) = engine();
        let process = process(&mut e, 0);
        assert_eq!(
            e.call(0x00dd_b310, &args![process, 0x100u32, 0x200u32, 0x300u32])
                .u32(),
            0x19
        );
        assert_eq!(calls(&log, WRITE_U32), vec![vec![ARCHIVE, 0x19]]);
        assert_eq!(calls(&log, WRITE_BYTE), vec![vec![ARCHIVE, 0x21]]);
        assert_eq!(
            calls(&log, WRITE_U64),
            vec![
                vec![ARCHIVE, 0x100, 0],
                vec![ARCHIVE, 0x200, 0],
                vec![ARCHIVE, 0x300, 0]
            ]
        );
        // With a registry and a class that has a virtual table, its answer replaces the class
        // (and the class is kept when it answers 0).
        let registry = e.mem.alloc(8);
        e.mem.set_u32(registry, REGISTRY_VTABLE);
        e.put_vtable(REGISTRY_VTABLE, &[0, 0, 0, 0, REGISTRY_TARGET]);
        e.register(REGISTRY_TARGET, |_, a| Ret {
            eax: if a[1] == 0x100 { 0x9999 } else { 0 },
            ..Ret::default()
        });
        e.register(CLASS_HAS_VTABLE, |e, a| {
            e.mem.set_u8(a[1], u8::from(a[0] != 0x777));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let process = self::process(&mut e, registry);
        log.borrow_mut().clear();
        e.call(0x00dd_b310, &args![process, 0x100u32, 0x200u32, 0x300u32]);
        assert_eq!(calls(&log, WRITE_U64)[1], vec![ARCHIVE, 0x9999, 0]);
        log.borrow_mut().clear();
        e.call(0x00dd_b310, &args![process, 0x101u32, 0x200u32, 0x300u32]);
        assert_eq!(calls(&log, WRITE_U64)[1], vec![ARCHIVE, 0x200, 0]);
        log.borrow_mut().clear();
        e.call(0x00dd_b310, &args![process, 0x100u32, 0x777u32, 0x300u32]);
        assert_eq!(calls(&log, WRITE_U64)[1], vec![ARCHIVE, 0x777, 0]);
    }

    #[test]
    fn init_announces_every_top_level_object() {
        let (mut e, log) = engine();
        e.map(0x011b_7000, 0x1000);
        for (offset, byte) in [1u8, 2, 3, 4].into_iter().enumerate() {
            e.mem.set_u8(INIT_BYTES + offset as u32, byte);
        }
        let process = process(&mut e, 0);
        let sub_object = process + 8;
        // The sub-object's `+0x24` is the process's `+0x2c` (the context); its archive is the
        // word at `+0xc`.
        let context = e.mem.u32(sub_object + 0x24);
        let entries = e.mem.alloc(0x18);
        for (i, value) in [0x10u32, 0x11, 0x12, 0x20, 0x21, 0x22]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(entries + 4 * i as u32, value);
        }
        e.mem.set_u32(context + 0x28, entries);
        e.mem.set_u32(context + 0x2c, 2);
        e.call(0x00dd_b3a0, &args![sub_object]);
        assert_eq!(
            calls(&log, WRITE_U32),
            vec![vec![ARCHIVE, 0xd], vec![ARCHIVE, 0x19], vec![ARCHIVE, 0x19]]
        );
        let bytes: Vec<u32> = calls(&log, WRITE_BYTE).iter().map(|c| c[1]).collect();
        assert_eq!(bytes, vec![0x20, 1, 2, 3, 4, 0x21, 0x21]);
        assert_eq!(calls(&log, WRITE_U64)[0], vec![ARCHIVE, INIT_STRING, 0]);
        assert_eq!(calls(&log, WRITE_U64)[1], vec![ARCHIVE, 0x10, 0]);
        assert_eq!(calls(&log, WRITE_U64)[4], vec![ARCHIVE, 0x20, 0]);
        // A null context writes nothing.
        e.mem.set_u32(sub_object + 0x24, 0);
        log.borrow_mut().clear();
        e.call(0x00dd_b3a0, &args![sub_object]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_member_walk_follows_the_kind_table() {
        let mut e = Engine::new();
        e.register(GET_ARRAY_MEMBER_SIZE, |_, _| Ret {
            eax: 8,
            ..Ret::default()
        });
        e.register(GET_STRUCT_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let second_member = e.mem.alloc(0x20);
        e.mem.set_u16(second_member + 0x12, 0x20);
        e.mem.set_u8(second_member + 0xc, 24);
        e.register_double(CLASS_GET_MEMBER, move |_, a| {
            assert_eq!(a[1], 3);
            Ret {
                eax: second_member,
                ..Ret::default()
            }
        });
        let steps = e.mem.alloc(8);
        for (i, step) in [9u16, 3, 2].into_iter().enumerate() {
            e.mem.set_u16(steps + 2 * i as u32, step);
        }
        let path = e.mem.alloc(4);
        e.mem.set_u32(path, steps);
        let member = e.mem.alloc(0x20);
        e.mem.set_u32(member + 4, 0x5555);
        e.mem.set_u8(member + 0xd, 1);
        let walk = |e: &mut Engine, member: u32, address: u32, kind: u32| {
            e.call(0x00dd_b450, &args![member, address, kind, path, 0u32])
                .u32()
        };
        // Kinds that end the walk answer the address.
        assert_eq!(walk(&mut e, member, 0x4000, 1), 0x4000);
        assert_eq!(walk(&mut e, member, 0x4000, 24), 0x4000);
        assert_eq!(walk(&mut e, member, 0x4000, 32), 0x4000);
        // Kind 14: the second step (3) times 16 is added and the kind becomes 12.
        assert_eq!(walk(&mut e, member, 0x4000, 14), 0x4030);
        // Kind 22: the pointer stored at the address plus step (3) times the member size (8);
        // the next kind is the member's byte at +0xd (1).
        e.map(0x4000, 0x1000);
        e.mem.set_u32(0x4100, 0x7000);
        assert_eq!(walk(&mut e, member, 0x4100, 22), 0x7000 + 24);
        // Kind 25: the struct's member 3 adds its offset (0x20) and ends on kind 24.
        assert_eq!(walk(&mut e, member, 0x4000, 25), 0x4020);
        // A null member or a member without a struct class fails; so do failing kinds.
        assert_eq!(walk(&mut e, 0, 0x4000, 25), 0);
        e.mem.set_u32(member + 4, 0);
        assert_eq!(walk(&mut e, member, 0x4000, 25), 0);
        assert_eq!(walk(&mut e, member, 0x4000, 19), 0);
        assert_eq!(walk(&mut e, member, 0x4000, 20), 0);
        assert_eq!(walk(&mut e, member, 0x4000, 0), 0);
        assert_eq!(walk(&mut e, member, 0x4000, 33), 0);
    }

    #[test]
    fn removing_a_top_level_object_swaps_the_last_entry_in() {
        let (mut e, log) = engine();
        let process = process(&mut e, 0);
        let data = e.mem.alloc(0x18);
        for (i, value) in [0x10u32, 1, 0x20, 2, 0x30, 3].into_iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, value);
        }
        e.mem.set_u32(process + 0x20, data);
        e.mem.set_u32(process + 0x24, 3);
        assert_eq!(e.call(0x00dd_b550, &args![process, 0x10u32]).u32(), 9);
        assert_eq!(calls(&log, WRITE_U32), vec![vec![ARCHIVE, 9]]);
        assert_eq!(calls(&log, WRITE_BYTE), vec![vec![ARCHIVE, 0x22]]);
        assert_eq!(calls(&log, WRITE_U64), vec![vec![ARCHIVE, 0x10, 0]]);
        assert_eq!(e.mem.u32(process + 0x24), 2);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [0x30, 3, 0x20]
        );
        // The last entry removes without a copy; an absent object changes nothing.
        e.call(0x00dd_b550, &args![process, 0x20u32]);
        assert_eq!(e.mem.u32(process + 0x24), 1);
        e.call(0x00dd_b550, &args![process, 0x99u32]);
        assert_eq!(e.mem.u32(process + 0x24), 1);
        e.mem.set_u32(process + 0x24, 0);
        assert_eq!(e.call(0x00dd_b550, &args![process, 0x30u32]).u32(), 9);
    }

    #[test]
    fn the_constructor_registers_with_the_context_and_the_destructor_undoes_it() {
        let (mut e, log) = engine();
        let frees = memory(&mut e);
        e.register_double(ADD_REFERENCE, {
            let sink = Rc::clone(&log);
            move |_, a| {
                sink.borrow_mut().push((ADD_REFERENCE, a.to_vec()));
                Ret::default()
            }
        });
        for address in [ADD_LISTENER, REMOVE_LISTENER, REMOVE_REFERENCE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        // contexts -> data -> first element -> object whose +4 is the context.
        let context = e.mem.alloc(0x60);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder + 4, context);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, holder);
        let contexts = e.mem.alloc(12);
        e.mem.set_u32(contexts, data);
        e.mem.set_u32(contexts + 4, 1);
        let process = e.mem.alloc(0x60);
        e.mem.write(process, &[0xcd; 0x60]);
        assert_eq!(
            e.call(0x00dd_b600, &args![process, contexts]).u32(),
            process
        );
        assert_eq!(e.mem.u16(process + 6), 1);
        assert_eq!(e.mem.u8(process + 0xc), 1);
        assert_eq!(e.mem.u32(process), PROCESS_VTABLE);
        assert_eq!(e.mem.u32(process + 8), PROCESS_VTABLE_AT_8);
        assert_eq!(e.mem.u32(process + 0x2c), context);
        assert_eq!(
            (
                e.mem.u32(process + 0x20),
                e.mem.u32(process + 0x24),
                e.mem.u32(process + 0x28)
            ),
            (0, 0, NOT_OWNED)
        );
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![context]]);
        assert_eq!(
            calls(&log, ADD_LISTENER),
            vec![vec![context, LISTENER_CALLBACK, process]]
        );
        // An empty context array leaves the context null and registers nothing.
        e.mem.set_u32(contexts + 4, 0);
        let other = e.mem.alloc(0x60);
        e.call(0x00dd_b600, &args![other, contexts]);
        assert_eq!(e.mem.u32(other + 0x2c), 0);
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 1);
        // The destructor removes the context's objects, the listener and the reference.
        e.mem.set_u32(process + 0x14, ARCHIVE);
        let entries = e.mem.alloc(0x18);
        for (i, value) in [0x10u32, 0, 0, 0x20, 0, 0].into_iter().enumerate() {
            e.mem.set_u32(entries + 4 * i as u32, value);
        }
        e.mem.set_u32(context + 0x28, entries);
        e.mem.set_u32(context + 0x2c, 2);
        let top = e.mem.alloc(0x20);
        e.mem.set_u32(process + 0x20, top);
        e.mem.set_u32(process + 0x24, 0);
        e.mem.set_u32(process + 0x28, 4);
        log.borrow_mut().clear();
        e.call(0x00dd_b670, &args![process]);
        let removed: Vec<u32> = calls(&log, WRITE_U64).iter().map(|c| c[1]).collect();
        assert_eq!(removed, vec![0x10, 0x20]);
        assert_eq!(
            calls(&log, REMOVE_LISTENER),
            vec![vec![context, LISTENER_CALLBACK]]
        );
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![context]]);
        assert_eq!(*frees.borrow(), vec![vec![top, 32]]);
        assert_eq!(e.mem.u32(process), BASE_VTABLE);
        assert_eq!(e.mem.u32(process + 8), BASE_VTABLE_AT_8);
        assert_eq!(e.mem.u32(process + 0x28), NOT_OWNED);
    }

    #[test]
    fn the_process_is_registered_with_the_factory() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.set_global(PROCESS_FACTORY_GLOBAL, 0x1234u32);
        e.register(REGISTER_PROCESS, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00dd_b750, &args![]);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (
                REGISTER_PROCESS,
                vec![0x1234, PROCESS_NAME, PROCESS_CREATOR]
            )
        );
        assert_eq!(e.global::<u32>(PROCESS_ID_GLOBAL), 7);
    }

    /// Doubles for the serialization of one object: the writer constructor remembers the
    /// character array, the serializer fills it with `size` bytes and, for object 0x100,
    /// adds one child record (object 0x200, class 0x300); `size_answer` is what it answers.
    fn serializer(e: &mut Engine, size_answer: Option<i32>) {
        e.register(STRUCTURE_LAYOUT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(STREAM_WRITER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STREAM_WRITER_DESTRUCTOR, |_, _| Ret::default());
        e.register_double(SERIALIZE_WRITE_OBJECT, move |e, a| {
            let characters = e.mem.u32(a[0]);
            let data = e.mem.alloc(8);
            e.mem.set_u32(characters, data);
            e.mem.set_u32(characters + 4, 5);
            e.mem.set_u32(characters + 8, 8);
            if a[2] == 0x100 {
                let records = e.mem.alloc(0x30);
                e.mem.set_u32(records + 8, 0x200);
                e.mem.set_u32(records + 0x10, 0x300);
                e.mem.set_u32(a[6], records);
                e.mem.set_u32(a[6] + 4, 1);
                e.mem.set_u32(a[6] + 8, 2);
            }
            Ret {
                eax: size_answer.unwrap_or(5) as u32,
                ..Ret::default()
            }
        });
    }

    fn visited_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 8);
        array
    }

    #[test]
    fn an_object_and_its_children_are_written_once() {
        let (mut e, log) = engine();
        let frees = memory(&mut e);
        serializer(&mut e, None);
        let process = process(&mut e, 0);
        let visited = visited_array(&mut e);
        let total = e
            .call(
                0x00dd_b770,
                &args![process, 0x100u32, 0x1u32, 0x0abc_de01u32, visited],
            )
            .u32();
        // 6 for the object (5 bytes + 1) and 6 for its child.
        assert_eq!(total, 12);
        assert_eq!(
            calls(&log, WRITE_U32),
            vec![vec![ARCHIVE, 6], vec![ARCHIVE, 6]]
        );
        assert_eq!(
            calls(&log, WRITE_BYTE),
            vec![vec![ARCHIVE, 0x24], vec![ARCHIVE, 0x24]]
        );
        assert_eq!(calls(&log, WRITE_RAW).len(), 2);
        assert_eq!(calls(&log, WRITE_RAW)[0][2], 5);
        // Both objects were recorded as visited, with their classes.
        assert_eq!(e.mem.u32(visited + 4), 2);
        let data = e.mem.u32(visited);
        assert_eq!(
            [
                e.mem.u32(data),
                e.mem.u32(data + 4),
                e.mem.u32(data + 8),
                e.mem.u32(data + 12)
            ],
            [0x100, 1, 0x200, 0x300]
        );
        // The character arrays (8 bytes each) and the child list (2 records of 0x18) are freed.
        let freed = frees.borrow();
        assert_eq!(freed.iter().filter(|f| f[1] == 8).count(), 2);
        assert_eq!(freed.iter().filter(|f| f[1] == 0x30).count(), 1);
        drop(freed);
        // The same object again, recursion on: already visited, nothing is written.
        log.borrow_mut().clear();
        let again = e
            .call(
                0x00dd_b770,
                &args![process, 0x100u32, 0x1u32, 1u32, visited],
            )
            .u32();
        assert_eq!(again, 0);
        assert!(calls(&log, WRITE_U32).is_empty());
        // With the flag clear the object is written without the visited check or children.
        let single = e
            .call(
                0x00dd_b770,
                &args![process, 0x100u32, 0x1u32, 0x0abc_de00u32, visited],
            )
            .u32();
        assert_eq!(single, 6);
        assert_eq!(e.mem.u32(visited + 4), 2);
    }

    #[test]
    fn a_serializer_that_disagrees_with_the_array_gives_minus_one() {
        let (mut e, log) = engine();
        memory(&mut e);
        serializer(&mut e, Some(4));
        let process = process(&mut e, 0);
        let visited = visited_array(&mut e);
        let result = e
            .call(
                0x00dd_b770,
                &args![process, 0x300u32, 0x1u32, 0u32, visited],
            )
            .u32();
        assert_eq!(result, u32::MAX);
        assert!(log.borrow().is_empty());
        serializer(&mut e, Some(0));
        let result = e
            .call(
                0x00dd_b770,
                &args![process, 0x300u32, 0x1u32, 0u32, visited],
            )
            .u32();
        assert_eq!(result, u32::MAX);
    }

    #[test]
    fn the_step_writes_every_top_level_object_without_recursion() {
        let (mut e, _) = engine();
        memory(&mut e);
        e.register(STRUCTURE_LAYOUT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(STREAM_WRITER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(STREAM_WRITER_DESTRUCTOR, |_, _| Ret::default());
        let seen: Log = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(SERIALIZE_WRITE_OBJECT, move |_, a| {
            sink.borrow_mut().push((0, a.to_vec()));
            Ret::default()
        });
        let process = process(&mut e, 0);
        let sub_object = process + 8;
        let entries = e.mem.alloc(0x10);
        for (i, value) in [0x10u32, 0x11, 0x20, 0x21].into_iter().enumerate() {
            e.mem.set_u32(entries + 4 * i as u32, value);
        }
        e.mem.set_u32(sub_object + 0x18, entries);
        e.mem.set_u32(sub_object + 0x1c, 2);
        e.call(0x00dd_ba20, &args![sub_object, 0u32]);
        let calls = seen.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].1[2..6], [0x10, 0, 0x10, 0x11]);
        assert_eq!(calls[1].1[2..6], [0x20, 0, 0x20, 0x21]);
        drop(calls);
        // No objects: nothing is written.
        e.mem.set_u32(sub_object + 0x1c, 0);
        e.call(0x00dd_ba20, &args![sub_object, 0u32]);
        assert_eq!(seen.borrow().len(), 2);
    }

    /// A reader double that answers each read from a script of byte vectors.
    fn scripted_reader(e: &mut Engine, script: Vec<Vec<u8>>) {
        let queue = Rc::new(RefCell::new(VecDeque::from(script)));
        let source = Rc::clone(&queue);
        e.register_double(READ_ARRAY_GENERIC, move |e, a| {
            let item = source.borrow_mut().pop_front().expect("script exhausted");
            assert_eq!(item.len() as u32, a[2] * a[3]);
            e.mem.write(a[1], &item);
            Ret::default()
        });
        let source = Rc::clone(&queue);
        e.register_double(READ_RAW, move |e, a| {
            let item = source.borrow_mut().pop_front().expect("script exhausted");
            assert_eq!(item.len() as u32, a[2]);
            e.mem.write(a[1], &item);
            Ret::default()
        });
    }

    #[test]
    fn the_object_command_adds_or_removes_the_top_level_entry() {
        let (mut e, log) = engine();
        memory(&mut e);
        serializer(&mut e, Some(0));
        let process = process(&mut e, 0);
        let sub_object = process + 8;
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(sub_object + 0x18, data);
        e.mem.set_u32(sub_object + 0x1c, 0);
        e.mem.set_u32(sub_object + 0x20, 4);
        let reader = e.mem.alloc(8);
        e.mem.set_u32(sub_object + 8, reader);
        let words = |object: u64, class: u64, flags: u8| {
            vec![
                object.to_le_bytes().to_vec(),
                class.to_le_bytes().to_vec(),
                vec![flags],
            ]
        };
        // Bit 0 adds; the object is written first (the serializer answers 0, so -1).
        scripted_reader(&mut e, words(0x100, 0x1, 1));
        e.call(0x00dd_bae0, &args![sub_object, 0x23u32]);
        assert_eq!(e.mem.u32(sub_object + 0x1c), 1);
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (0x100, 1));
        // Adding it again changes nothing.
        scripted_reader(&mut e, words(0x100, 0x1, 1));
        e.call(0x00dd_bae0, &args![sub_object, 0x23u32]);
        assert_eq!(e.mem.u32(sub_object + 0x1c), 1);
        // Bit 0 clear removes it.
        scripted_reader(&mut e, words(0x100, 0x1, 0));
        e.call(0x00dd_bae0, &args![sub_object, 0x23u32]);
        assert_eq!(e.mem.u32(sub_object + 0x1c), 0);
        // A zero object or class does nothing at all (no entry, no serialization).
        let before = log.borrow().len();
        scripted_reader(&mut e, words(0, 0x1, 1));
        e.call(0x00dd_bae0, &args![sub_object, 0x23u32]);
        scripted_reader(&mut e, words(0x100, 0, 1));
        e.call(0x00dd_bae0, &args![sub_object, 0x23u32]);
        assert_eq!(e.mem.u32(sub_object + 0x1c), 0);
        assert_eq!(log.borrow().len(), before);
        // Other commands do nothing.
        e.call(0x00dd_bae0, &args![sub_object, 0x24u32]);
    }

    #[test]
    fn the_member_command_writes_the_value_into_the_object() {
        let (mut e, _) = engine();
        let frees = memory(&mut e);
        let process = process(&mut e, 0);
        let sub_object = process + 8;
        let reader = e.mem.alloc(8);
        e.mem.set_u32(sub_object + 8, reader);
        let member = e.mem.alloc(0x20);
        e.mem.set_u16(member + 0x12, 0x10);
        e.mem.set_u8(member + 0xc, 1);
        e.register_double(CLASS_GET_MEMBER_OV2, move |_, a| {
            assert_eq!(a[..], [0x5000, 7]);
            Ret {
                eax: member,
                ..Ret::default()
            }
        });
        let copies: Log = Rc::default();
        let sink = Rc::clone(&copies);
        e.register_double(MEM_COPY, move |e, a| {
            sink.borrow_mut()
                .push((0, vec![a[0], a[2], e.mem.u32(a[1])]));
            Ret::default()
        });
        // Object 0x4000, class 0x5000, 2 steps (7 and 9), 4 value bytes.
        scripted_reader(
            &mut e,
            vec![
                0x4000u64.to_le_bytes().to_vec(),
                0x5000u64.to_le_bytes().to_vec(),
                2u16.to_le_bytes().to_vec(),
                7u16.to_le_bytes().to_vec(),
                9u16.to_le_bytes().to_vec(),
                4u32.to_le_bytes().to_vec(),
                0x1122_3344u32.to_le_bytes().to_vec(),
            ],
        );
        e.call(0x00dd_bae0, &args![sub_object, 0x25u32]);
        // Kind 1 ends the walk at once: the address is the object plus the member offset.
        assert_eq!(*copies.borrow(), vec![(0, vec![0x4010, 4, 0x1122_3344])]);
        // The 2-entry step array (4 bytes) and the 4-byte value array are freed.
        assert_eq!(frees.borrow().len(), 2);
        assert_eq!(frees.borrow()[0][1], 4);
        assert_eq!(frees.borrow()[1][1], 4);
        // A zero object address copies nothing.
        scripted_reader(
            &mut e,
            vec![
                0u64.to_le_bytes().to_vec(),
                0x5000u64.to_le_bytes().to_vec(),
                0u16.to_le_bytes().to_vec(),
                0u32.to_le_bytes().to_vec(),
                vec![],
            ],
        );
        e.call(0x00dd_bae0, &args![sub_object, 0x25u32]);
        assert_eq!(copies.borrow().len(), 1);
    }
}
