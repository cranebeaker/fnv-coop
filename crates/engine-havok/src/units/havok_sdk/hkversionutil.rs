//! `Havok SDK/hkversionutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dc0fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionUtil::getCurrentVersion` (Xbox PDB): takes no arguments and returns the constant 0x010c_f830.
/// The constant is the address of a NUL-terminated string in the exe.
pub fn hk_version_util_get_current_version(_e: &mut Engine) -> u32 {
    0x010c_f830
}

/// The memory router (`00c85750`): `+0xc` is the allocator `copyObjects` takes its temporary
/// array from, `+0x10` the one the other functions use (slot `+4` `blockAlloc(size)`, slot
/// `+0x10` `bufFree(block, size)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve(allocator, array, capacity, element size)` (`00c908c0`, cdecl).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkClass`: `getObjectSize` (`00d470d0`), `getNumMembers` (`00d46f00`), `getMember(index)`
/// (`00d46f20`, `RET 4`), `getMemberByName(name)` (`00d46fe0`, `RET 4`), `getDefault(index,
/// stream)` (`00d471c0`, `RET 8`) and `getName` (`00d46d40`, answers the first word), all
/// `thiscall`.
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
const CLASS_GET_NUM_MEMBERS: u32 = 0x00d4_6f00;
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;
const CLASS_GET_MEMBER_BY_NAME: u32 = 0x00d4_6fe0;
const CLASS_GET_DEFAULT: u32 = 0x00d4_71c0;
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// `hkBufferedStreamWriter`: the constructor (`00d274c0`, `thiscall`, `RET 0xc`: buffer, size,
/// a flag word), `seek(offset, mode)` (`00d27380`, `RET 8`) and the destructor (`00d27520`).
const WRITER_CONSTRUCT: u32 = 0x00d2_74c0;
const WRITER_SEEK: u32 = 0x00d2_7380;
const WRITER_DESTRUCT: u32 = 0x00d2_7520;
/// `hkClassMember`: `getClass` (`00d86920`) and `getStructClass` (`00d86910`), `thiscall`.
const MEMBER_GET_CLASS: u32 = 0x00d8_6920;
const MEMBER_GET_STRUCT_CLASS: u32 = 0x00d8_6910;
/// The class record `areMembersCompatible` compares struct class names with (`01268748`) and
/// `hkString::strCmp(a, b)` (`00cb0fa0`, cdecl).
const COMPARED_CLASS: u32 = 0x0126_8748;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// `hkObjectCopier`: the constructor (`00e03120`, `thiscall`, `RET 0xc`: two layouts and a
/// flag word), the destructor (`00e03160`), `areMembersCompatible(a, b)` (`00e032d0`, `RET 8`) and
/// `copyObject(object, class, writer, previous word, relocation info)` (`00e04920`, `RET
/// 0x14`).
const COPIER_CONSTRUCT: u32 = 0x00e0_3120;
const COPIER_DESTRUCT: u32 = 0x00e0_3160;
const COPIER_ARE_MEMBERS_COMPATIBLE: u32 = 0x00e0_32d0;
const COPIER_COPY_OBJECT: u32 = 0x00e0_4920;
/// The vtable `00dc1040` stores (`010d6acc`).
const PACKFILE_COPIER_VTABLE: u32 = 0x010d_6acc;
/// `hkStructureLayout`: the constructor with no arguments (`00dc1670`) and the one taking the
/// layout rules (`00dc1680`, `RET 4`), and the rules the exe passes (`011b7b88`).
const LAYOUT_CONSTRUCT: u32 = 0x00dc_1670;
const LAYOUT_CONSTRUCT_FROM_RULES: u32 = 0x00dc_1680;
const LAYOUT_RULES: u32 = 0x011b_7b88;
/// `00dc1bb0` (`thiscall` on the layout, `RET 0xc`: class, map, a flag of 1) and
/// `00e04d90` (cdecl: the class array and a word).
const LAYOUT_ADD_CLASS: u32 = 0x00dc_1bb0;
const PREPARE_CLASSES: u32 = 0x00e0_4d90;
/// `hkMapBase<ulong, ulong>`: `reserve(allocator, count)` (`00cb1fe0`), `insert(allocator,
/// key, value)` (`00cb1870`, `RET 0xc`), `clearAndDeallocate(allocator)` (`00cb1a90`, `RET 4`)
/// and the destructor `00a29680`, all `thiscall` on the map.
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_DESTRUCT: u32 = 0x00a2_9680;
/// `hkOstream::hkOstream(hkArray<char> &)` (`00cc0630`, `RET 4`), its destructor (`00cc00e0`) and
/// `hkRelocationInfo::~hkRelocationInfo` (`00e028e0`).
const OSTREAM_CONSTRUCT_FROM_ARRAY: u32 = 0x00cc_0630;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
const RELOCATION_INFO_DESTRUCT: u32 = 0x00e0_28e0;
/// `hkString::memCpy(destination, source, count)` (`00cb1260`, cdecl).
const MEM_CPY: u32 = 0x00cb_1260;

// Translated from 00db7a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionUtil::recomputeClassMemberOffsets` (Xbox PDB), `thiscall`, no stack words: the
/// first time (the word at `+0x14` zero) runs `00dc1230` on the pair of words at `+0xc` and
/// `+0x10` and sets the word at `+0x14` to 1.
pub fn hk_version_util_recompute_class_member_offsets(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x14) == 0 {
        let classes = e.mem.u32(base + 0xc);
        let second = e.mem.u32(base + 0x10);
        fn_00dc1230(e, Ptr::new(classes), second);
        e.mem.set_u32(base + 0x14, 1);
    }
}

// Translated from 00dc1040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `first`, `second`): the constructor of the
/// packfile object copier: `hkObjectCopier`'s constructor (`00e03120`) with `first`, `second`
/// and a third stack word of zero (the stack slot the exe zeroes first), then the vtable
/// `010d6acc`. Returns `this`.
pub fn fn_00dc1040(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(COPIER_CONSTRUCT, &args![this, first, second, 0u32]);
    e.mem.set_u32(this.addr(), PACKFILE_COPIER_VTABLE);
    this
}

// Translated from 00dc1070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionUtil::copyDefaults` (Xbox PDB), `cdecl`, three stack words (`buffer`,
/// `old_class`, `new_class`): puts the defaults of the members the new class has and the old
/// one does not into the object at `buffer`. An `hkBufferedStreamWriter` over `buffer` with the
/// size of the new class (`getObjectSize`, `00d470d0`) is made (`00d274c0`, a flag word of
/// zero); then for each member of the new class (the count asked at the start and again after
/// each member) the old class is asked for a member of that name (`getMemberByName`); when it has
/// none the writer is moved to the member's offset (the 16-bit word at `+0x12`, `seek`, mode 0)
/// and the new class's `getDefault(index, writer)` writes it. The writer is destroyed last.
pub fn hk_version_util_copy_defaults(e: &mut Engine, buffer: u32, old_class: Ptr, new_class: Ptr) {
    let size = e.call(CLASS_GET_OBJECT_SIZE, &args![new_class]).u32();
    e.with_stack(0x1c, |e, writer| {
        e.call(WRITER_CONSTRUCT, &args![writer, buffer, size, 0u32]);
        let mut index = 0u32;
        let mut members = e.call(CLASS_GET_NUM_MEMBERS, &args![new_class]).i32();
        if members > 0 {
            loop {
                let member = e.call(CLASS_GET_MEMBER, &args![new_class, index]).u32();
                let name = e.mem.u32(member);
                let found = e
                    .call(CLASS_GET_MEMBER_BY_NAME, &args![old_class, name])
                    .u32();
                if found == 0 {
                    let offset = u32::from(e.mem.u16(member + 0x12));
                    e.call(WRITER_SEEK, &args![writer, offset, 0u32]);
                    e.call(CLASS_GET_DEFAULT, &args![new_class, index, writer]);
                }
                index += 1;
                members = e.call(CLASS_GET_NUM_MEMBERS, &args![new_class]).i32();
                if (index as i32) >= members {
                    break;
                }
            }
        }
        e.call(WRITER_DESTRUCT, &args![writer]);
    });
}

// Translated from 00dc1100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::PackfileObjectCopier::areMembersCompatible` (Xbox PDB), `thiscall`,
/// `RET 8` (`first`, `second`): members are compatible when `hkObjectCopier`'s own test
/// (`00e032d0`, always asked) says so or, with the type bytes at `+0xc` and the subtype
/// bytes at `+0xd` of the two members: the first type is `0x1d` and the second `0x21`; or
/// the first type is `0x1c`, the second `0x14`, the second's subtype `0x19`, the second has a
/// class (`getClass`, `00d86920`) and its struct class's name (`getStructClass`
/// `00d86910`, then `getName`) equals the name of the class record at `01268748` (`strCmp` zero);
/// or the types are equal (or `0x1a` against `0x16`) and the subtypes are equal (or `0x1d`
/// against `0x21`), or the subtypes are `0x1c` against `0x14` with a second class whose struct
/// class has that same name. Answers 0 or 1.
pub fn packfile_object_copier_are_members_compatible(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
) -> u32 {
    let first_type = e.mem.u8(first.addr() + 0xc);
    let second_type = e.mem.u8(second.addr() + 0xc);
    let types_alike = first_type == second_type || (first_type == 0x1a && second_type == 0x16);
    let wide_pair = first_type == 0x1d && second_type == 0x21;
    let mut struct_by_type = false;
    if first_type == 0x1c
        && second_type == 0x14
        && e.mem.u8(second.addr() + 0xd) == 0x19
        && e.call(MEMBER_GET_CLASS, &args![second]).u32() != 0
    {
        struct_by_type = struct_class_matches(e, second);
    }
    let first_sub = e.mem.u8(first.addr() + 0xd);
    let second_sub = e.mem.u8(second.addr() + 0xd);
    let subtypes_alike = first_sub == second_sub || (first_sub == 0x1d && second_sub == 0x21);
    let mut struct_by_subtype = false;
    if first_sub == 0x1c
        && second_sub == 0x14
        && e.call(MEMBER_GET_CLASS, &args![second]).u32() != 0
    {
        struct_by_subtype = struct_class_matches(e, second);
    }
    let copier_says = e
        .call(COPIER_ARE_MEMBERS_COMPATIBLE, &args![this, first, second])
        .u32();
    u32::from(
        copier_says != 0
            || wide_pair
            || struct_by_type
            || (types_alike && (subtypes_alike || struct_by_subtype)),
    )
}

/// Whether the struct class of `member`'s name equals the name of the class record at
/// `01268748`: `getStructClass`, `getName`, `getName` of the record, `strCmp(record name,
/// struct class name)` zero.
fn struct_class_matches(e: &mut Engine, member: Ptr) -> bool {
    let struct_class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
    let name = e.call(CLASS_GET_NAME, &args![struct_class]).u32();
    let record_name = e.call(CLASS_GET_NAME, &args![COMPARED_CLASS]).u32();
    e.call(STRING_COMPARE, &args![record_name, name]).u32() == 0
}

// Translated from 00dc1230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two stack words: `classes`, `extra`): prepares the
/// classes (`00e04d90(classes, extra)`), makes a `hkStructureLayout` (`00dc1670`; its one
/// word is the layout rules pointer, kept in the stack word the exe reuses) and an
/// `hkMapBase<ulong, ulong>` (`{0, 0, -1}`, `reserve(allocator, 0)`, `00cb1fe0`), and calls
/// `00dc1bb0(layout; class, &map, 1)` for each class of the null-terminated array `classes`; the
/// map is cleared and deallocated (`00cb1a90`) and destroyed (`00a29680`) at the end.
pub fn fn_00dc1230(e: &mut Engine, classes: Ptr, extra: u32) {
    e.call(PREPARE_CLASSES, &args![classes, extra]);
    e.with_stack(0x10, |e, frame| {
        let layout = frame.addr();
        let map = frame.addr() + 4;
        e.call(LAYOUT_CONSTRUCT, &args![layout]);
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
        let mut index = 0u32;
        let mut class = e.mem.u32(classes.addr());
        while class != 0 {
            e.call(LAYOUT_ADD_CLASS, &args![layout, class, map, 1u32]);
            class = e.mem.u32(classes.addr().wrapping_add(4 * (index + 1)));
            index += 1;
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(MAP_DESTRUCT, &args![map]);
    });
}

// Translated from 00dc12c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::copySomeObjects` (Xbox PDB, inputs in a register and on the
/// stack, `cdecl` for the stack words). Registers first: `EAX` the array of object numbers
/// (`indices`, data at `+0`, count at `+4`); then the stack words `objects` (a pointer to an
/// array of 8-byte records `(object, class)`), `offsets` (a pointer to an array of 8-byte records
/// the function writes the object's offset into the stream to, keeping the second word) and
/// `sink` (an object whose slot `+0x10` takes `(block, size, 5)` and slot `+0x14` `(second
/// word, address)`). It makes an `hkOstream` over a growing array of bytes, an
/// `hkRelocationInfo` and an `hkMapBase<ulong, ulong>`, and for each index copies the
/// object with a packfile object copier (`00dc1680` twice for the two layouts, `00dc1040`
/// for the copier, `00e04920` for the copy, `00e03160` to destroy it), recording `object ->
/// offset` in the map and the offset in `offsets`. With nothing written the temporaries are
/// destroyed and 0 returned. Otherwise a block of the size written is allocated (the
/// allocator's slot `+4`), the sink told of it (slot `+0x10`, flag 5), the bytes copied
/// into it (`00cb1260`), the offsets of `objects` turned into addresses (the block address
/// added), each pair of the relocation info's first array (local fixups) filled in
/// (`*(block + offset) = value + block`), and each 16-byte entry of its second array
/// (`+0x10`... imports) given to the sink's slot `+0x14` as `(second word, block + first word)`.
/// The temporaries are destroyed and the block returned.
pub fn copy_some_objects(
    e: &mut Engine,
    indices: Ptr,
    objects: Ptr,
    offsets: Ptr,
    sink: Ptr,
) -> u32 {
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let layout_one = f;
        let layout_two = f + 4;
        let pair_value = f + 8;
        let pair_key = f + 0xc;
        let bytes = f + 0x10;
        let map = f + 0x1c;
        let stream = f + 0x28;
        let copier = f + 0x34;
        let relocations = f + 0x4c;
        e.mem.set_u32(bytes, 0);
        e.mem.set_u32(bytes + 4, 0);
        e.mem.set_u32(bytes + 8, 0x8000_0000);
        e.call(OSTREAM_CONSTRUCT_FROM_ARRAY, &args![stream, bytes]);
        for array in 0..4u32 {
            let at = relocations + 0xc * array;
            e.mem.set_u32(at, 0);
            e.mem.set_u32(at + 4, 0);
            e.mem.set_u32(at + 8, 0x8000_0000);
        }
        e.mem.set_u32(relocations + 0x30, 0);
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
        let mut index = 0i32;
        while index < e.mem.i32(indices.addr() + 4) {
            let ray = e.mem.u32(
                e.mem
                    .u32(indices.addr())
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let offset = ray.wrapping_mul(8);
            let record = e.mem.u32(objects.addr()).wrapping_add(offset);
            e.call(
                LAYOUT_CONSTRUCT_FROM_RULES,
                &args![layout_one, LAYOUT_RULES],
            );
            e.call(
                LAYOUT_CONSTRUCT_FROM_RULES,
                &args![layout_two, LAYOUT_RULES],
            );
            fn_00dc1040(e, Ptr::new(copier), layout_two, layout_one);
            let written = e.mem.u32(offsets.addr()).wrapping_add(offset);
            let previous = e.mem.u32(written + 4);
            let size = e.mem.u32(bytes + 4);
            e.mem.set_u32(written, size);
            let object = e.mem.u32(record);
            e.mem.set_u32(pair_value, size);
            e.mem.set_u32(pair_key, object);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_INSERT, &args![map, allocator, object, size]);
            let writer = e.mem.u32(stream + 8);
            let class = e.mem.u32(record + 4);
            e.call(
                COPIER_COPY_OBJECT,
                &args![copier, object, class, writer, previous, relocations],
            );
            e.call(COPIER_DESTRUCT, &args![copier]);
            index += 1;
        }
        let total = e.mem.u32(bytes + 4);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let mut block = 0u32;
        if total != 0 {
            block = e.vcall(allocator, 4, &args![total]).u32();
            e.vcall(sink.addr(), 0x10, &args![block, e.mem.u32(bytes + 4), 5u32]);
            let data = e.mem.u32(bytes);
            e.call(MEM_CPY, &args![block, data, e.mem.u32(bytes + 4)]);
            let mut index = 0i32;
            while index < e.mem.i32(indices.addr() + 4) {
                let ray = e.mem.u32(
                    e.mem
                        .u32(indices.addr())
                        .wrapping_add((index as u32).wrapping_mul(4)),
                );
                let slot = e.mem.u32(offsets.addr()).wrapping_add(ray.wrapping_mul(8));
                let moved = e.mem.u32(slot).wrapping_add(block);
                e.mem.set_u32(slot, moved);
                index += 1;
            }
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(relocations + 4) {
                let entry = e.mem.u32(relocations).wrapping_add(8 * index);
                let value = e.mem.u32(entry + 4).wrapping_add(block);
                let position = e.mem.u32(entry);
                e.mem.set_u32(block.wrapping_add(position), value);
                index += 1;
            }
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(relocations + 0x10) {
                let entry = e.mem.u32(relocations + 0xc).wrapping_add(0x10 * index);
                let first = e.mem.u32(entry);
                let second = e.mem.u32(entry + 4);
                e.vcall(sink.addr(), 0x14, &args![second, first.wrapping_add(block)]);
                index += 1;
            }
        }
        let map_allocator = if total != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            e.mem.u32(router + 0x10)
        } else {
            allocator
        };
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, map_allocator]);
        e.call(MAP_DESTRUCT, &args![map]);
        e.call(RELOCATION_INFO_DESTRUCT, &args![relocations]);
        e.call(OSTREAM_DESTRUCT, &args![stream]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(bytes + 8);
        e.mem.set_u32(bytes + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(bytes);
            e.vcall(allocator, 0x10, &args![data, capacity & 0x3fff_ffff]);
        }
        block
    })
}

// Translated from 00dc15a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionUtil::copyObjects` (Xbox PDB), `cdecl`, three stack words (`records`, `offsets`,
/// `sink`; the first has the object count at `+4`): makes a temporary array of the numbers
/// 0 to count - 1 (an `hkArray<int>` with its data from the memory router's allocator at
/// `+0xc`, `_reserve`d to the count when its capacity, zero, is lower), runs
/// `copySomeObjects` (`00dc12c0`, the array in `EAX`, then `records`, `offsets` and `sink`
/// as the three stack words) and gives the array back (the allocator's slot `+0x10`, the
/// masked capacity times 4) unless its capacity word carries the flag. Returns what
/// `copySomeObjects` returned.
pub fn hk_version_util_copy_objects(e: &mut Engine, records: Ptr, offsets: Ptr, sink: Ptr) -> u32 {
    let count = e.mem.i32(records.addr() + 4);
    e.with_stack(0xc, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
        }
        e.mem.set_u32(array + 4, count as u32);
        let mut index = 0i32;
        while index < e.mem.i32(records.addr() + 4) {
            let data = e.mem.u32(array);
            e.mem.set_u32(
                data.wrapping_add((index as u32).wrapping_mul(4)),
                index as u32,
            );
            index += 1;
        }
        let answer = copy_some_objects(e, Ptr::new(array), records, offsets, sink);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        answer
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc0fb0, hk_version_util_get_current_version() -> u32),
        entry!(
            0x00db7a80,
            hk_version_util_recompute_class_member_offsets(Ptr)
        ),
        entry!(0x00dc1040, fn_00dc1040(Ptr, u32, u32) -> Ptr),
        entry!(0x00dc1070, hk_version_util_copy_defaults(u32, Ptr, Ptr)),
        entry!(0x00dc1100, packfile_object_copier_are_members_compatible(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dc1230, fn_00dc1230(Ptr, u32)),
        entry!(0x00dc12c0, copy_some_objects(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dc15a0, hk_version_util_copy_objects(Ptr, Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hk_version_util_get_current_version_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dc_0fb0, &args![]).u32(), 0x010c_f830);
    }
    use std::cell::{Cell, RefCell};
    use std::collections::HashSet;
    use std::rc::Rc;

    type Events = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    const ROUTER_TABLE_B: u32 = 0x0062_0000;
    const ROUTER_TABLE_C: u32 = 0x0062_1000;
    const SINK_TABLE: u32 = 0x0063_0000;
    const DOUBLES: u32 = 0x0064_0000;

    /// Registers a double at `addr` that appends `(tag, arguments)` to `events` and answers
    /// `answer`.
    fn tag_calls(e: &mut Engine, events: &Events, addr: u32, tag: &'static str, answer: u32) {
        let kept = events.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push((tag, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls(events: &Events, tag: &str) -> Vec<Vec<u32>> {
        events
            .borrow()
            .iter()
            .filter(|(name, _)| *name == tag)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn tags(events: &Events) -> Vec<&'static str> {
        events.borrow().iter().map(|(tag, _)| *tag).collect()
    }

    /// The memory router: `+0x10` is an allocator whose slot 4 allocates (`alloc`) and slot 0x10
    /// frees (`free`); `+0xc` is another whose slot 0x10 frees (`free_temporary`).
    fn install_router(e: &mut Engine, events: &Events) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let allocator_b = e.mem.alloc(8);
        let allocator_c = e.mem.alloc(8);
        e.mem.set_u32(router + 0x10, allocator_b);
        e.mem.set_u32(router + 0xc, allocator_c);
        e.mem.set_u32(allocator_b, ROUTER_TABLE_B);
        e.mem.set_u32(allocator_c, ROUTER_TABLE_C);
        let mut table = vec![0u32; 5];
        table[1] = DOUBLES;
        table[4] = DOUBLES + 0x10;
        e.put_vtable(ROUTER_TABLE_B, &table);
        let mut table = vec![0u32; 5];
        table[4] = DOUBLES + 0x20;
        e.put_vtable(ROUTER_TABLE_C, &table);
        let kept = events.clone();
        e.register_double(DOUBLES, move |e, a| {
            kept.borrow_mut().push(("alloc", a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        tag_calls(e, events, DOUBLES + 0x10, "free", 0);
        tag_calls(e, events, DOUBLES + 0x20, "free_temporary", 0);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (allocator_b, allocator_c)
    }

    #[test]
    fn the_class_offsets_are_recomputed_once() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_router(&mut e, &events);
        tag_calls(&mut e, &events, PREPARE_CLASSES, "prepare", 0);
        tag_calls(&mut e, &events, LAYOUT_CONSTRUCT, "layout", 0);
        tag_calls(&mut e, &events, MAP_RESERVE, "reserve", 0);
        tag_calls(&mut e, &events, LAYOUT_ADD_CLASS, "add_class", 0);
        tag_calls(&mut e, &events, MAP_CLEAR_AND_DEALLOCATE, "clear", 0);
        tag_calls(&mut e, &events, MAP_DESTRUCT, "destruct", 0);
        let classes = e.mem.alloc(16);
        e.mem.set_u32(classes, 0xc1);
        e.mem.set_u32(classes + 4, 0xc2);
        e.mem.set_u32(classes + 8, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, classes);
        e.mem.set_u32(this + 0x10, 7);
        e.call(0x00db_7a80, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(calls(&events, "prepare"), vec![vec![classes, 7]]);
        // The layout is one word, the map follows it; both are the exe's stack words.
        let layout = calls(&events, "layout")[0][0];
        let map = layout + 4;
        assert_eq!(calls(&events, "reserve")[0][0], map);
        assert_eq!(calls(&events, "reserve")[0][2], 0);
        assert_eq!(
            calls(&events, "add_class"),
            vec![vec![layout, 0xc1, map, 1], vec![layout, 0xc2, map, 1]]
        );
        assert_eq!(calls(&events, "clear")[0][0], map);
        assert_eq!(calls(&events, "destruct"), vec![vec![map]]);
        // The second call does nothing.
        events.borrow_mut().clear();
        e.call(0x00db_7a80, &args![this]);
        assert!(events.borrow().is_empty());
    }

    #[test]
    fn the_map_starts_as_zero_zero_minus_one() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_router(&mut e, &events);
        tag_calls(&mut e, &events, PREPARE_CLASSES, "prepare", 0);
        tag_calls(&mut e, &events, LAYOUT_CONSTRUCT, "layout", 0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let kept = seen.clone();
        e.register_double(MAP_RESERVE, move |e, a| {
            *kept.borrow_mut() = vec![e.mem.u32(a[0]), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)];
            Ret::default()
        });
        tag_calls(&mut e, &events, MAP_CLEAR_AND_DEALLOCATE, "clear", 0);
        tag_calls(&mut e, &events, MAP_DESTRUCT, "destruct", 0);
        let classes = e.mem.alloc(8);
        fn_00dc1230(&mut e, Ptr::new(classes), 3);
        assert_eq!(*seen.borrow(), vec![0, 0, 0xffff_ffff]);
        // No classes: no layout calls.
        assert_eq!(
            tags(&events),
            vec!["prepare", "layout", "clear", "destruct"]
        );
    }

    #[test]
    fn the_copier_constructor_passes_a_zero_word_and_stores_its_vtable() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        tag_calls(&mut e, &events, COPIER_CONSTRUCT, "base", 0);
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00dc_1040, &args![this, 0x11u32, 0x22u32]).u32(),
            this
        );
        assert_eq!(calls(&events, "base"), vec![vec![this, 0x11, 0x22, 0]]);
        assert_eq!(e.mem.u32(this), PACKFILE_COPIER_VTABLE);
    }

    #[test]
    fn copy_defaults_writes_the_defaults_of_the_new_members_only() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let old_class = e.mem.alloc(8);
        let new_class = e.mem.alloc(8);
        // Three members: named by words 0xa, 0xb, 0xc; their offsets (+0x12, 16 bits) 4, 8, 12.
        let members = e.mem.alloc(0x60);
        for (index, name) in [0xau32, 0xb, 0xc].iter().enumerate() {
            let member = members + 0x20 * index as u32;
            e.mem.set_u32(member, *name);
            e.mem.set_u16(member + 0x12, 4 * (index as u16 + 1));
        }
        e.register(CLASS_GET_OBJECT_SIZE, |_, _| Ret {
            eax: 0x40,
            ..Ret::default()
        });
        let counts = Rc::new(Cell::new(0u32));
        let kept = counts.clone();
        e.register_double(CLASS_GET_NUM_MEMBERS, move |_, _| {
            kept.set(kept.get() + 1);
            Ret {
                eax: 3,
                ..Ret::default()
            }
        });
        e.register_double(CLASS_GET_MEMBER, move |_, a| Ret {
            eax: members + 0x20 * a[1],
            ..Ret::default()
        });
        let old_has: HashSet<u32> = [0xau32, 0xc].into_iter().collect();
        e.register_double(CLASS_GET_MEMBER_BY_NAME, move |_, a| Ret {
            eax: u32::from(old_has.contains(&a[1])),
            ..Ret::default()
        });
        tag_calls(&mut e, &events, WRITER_CONSTRUCT, "writer", 0);
        tag_calls(&mut e, &events, WRITER_SEEK, "seek", 0);
        tag_calls(&mut e, &events, CLASS_GET_DEFAULT, "default", 0);
        tag_calls(&mut e, &events, WRITER_DESTRUCT, "writer_destruct", 0);
        e.call(0x00dc_1070, &args![0x5000u32, old_class, new_class]);
        let writer = calls(&events, "writer")[0][0];
        assert_eq!(
            calls(&events, "writer"),
            vec![vec![writer, 0x5000, 0x40, 0]]
        );
        // Only member 1 (name 0xb) is missing from the old class.
        assert_eq!(calls(&events, "seek"), vec![vec![writer, 8, 0]]);
        assert_eq!(calls(&events, "default"), vec![vec![new_class, 1, writer]]);
        assert_eq!(calls(&events, "writer_destruct"), vec![vec![writer]]);
        // The member count is asked at the start and after each member.
        assert_eq!(counts.get(), 4);
    }

    #[test]
    fn copy_defaults_with_no_members_only_builds_and_destroys_the_writer() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        e.register(CLASS_GET_OBJECT_SIZE, |_, _| Ret::default());
        e.register(CLASS_GET_NUM_MEMBERS, |_, _| Ret::default());
        tag_calls(&mut e, &events, WRITER_CONSTRUCT, "writer", 0);
        tag_calls(&mut e, &events, WRITER_DESTRUCT, "writer_destruct", 0);
        e.call(0x00dc_1070, &args![0x5000u32, 1u32, 2u32]);
        assert_eq!(tags(&events), vec!["writer", "writer_destruct"]);
    }

    /// Members `(type, subtype)` at fresh blocks, a copier double answering `base`, class
    /// doubles over names stored as C strings and `strCmp` over them.
    fn compat_setup(e: &mut Engine, events: &Events, base: u32, struct_name: &str) {
        tag_calls(e, events, COPIER_ARE_MEMBERS_COMPATIBLE, "base", base);
        e.register(MEMBER_GET_CLASS, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let name = e.mem.alloc(32);
        e.mem.set_cstr(name, struct_name.as_bytes());
        let struct_class = e.mem.alloc(8);
        e.mem.set_u32(struct_class, name);
        e.register_double(MEMBER_GET_STRUCT_CLASS, move |_, _| Ret {
            eax: struct_class,
            ..Ret::default()
        });
        let record_name = e.mem.alloc(32);
        e.mem.set_cstr(record_name, b"hkClass");
        e.map(COMPARED_CLASS, 8);
        e.mem.set_u32(COMPARED_CLASS, record_name);
        e.register(CLASS_GET_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(STRING_COMPARE, |e, a| Ret {
            eax: u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])),
            ..Ret::default()
        });
    }

    fn member(e: &mut Engine, kind: u8, subtype: u8) -> Ptr {
        let at = e.mem.alloc(0x20);
        e.mem.set_u8(at + 0xc, kind);
        e.mem.set_u8(at + 0xd, subtype);
        Ptr::new(at)
    }

    fn compatible(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> u32 {
        e.call(0x00dc_1100, &args![this, first, second]).u32()
    }

    #[test]
    fn members_are_compatible_by_the_copiers_answer_or_by_type_and_subtype() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        compat_setup(&mut e, &events, 0, "hkClass");
        let this = Ptr::new(e.mem.alloc(8));
        // Equal types and subtypes.
        let a = member(&mut e, 5, 6);
        let b = member(&mut e, 5, 6);
        assert_eq!(compatible(&mut e, this, a, b), 1);
        // Equal types, different subtypes.
        let c = member(&mut e, 5, 7);
        assert_eq!(compatible(&mut e, this, a, c), 0);
        // Different types.
        let d = member(&mut e, 9, 6);
        assert_eq!(compatible(&mut e, this, a, d), 0);
        // 0x1a against 0x16 counts as alike types.
        let e1 = member(&mut e, 0x1a, 6);
        let e2 = member(&mut e, 0x16, 6);
        assert_eq!(compatible(&mut e, this, e1, e2), 1);
        // 0x1d against 0x21 as types is compatible on its own; as subtypes it is alike.
        let f1 = member(&mut e, 0x1d, 1);
        let f2 = member(&mut e, 0x21, 2);
        assert_eq!(compatible(&mut e, this, f1, f2), 1);
        let g1 = member(&mut e, 5, 0x1d);
        let g2 = member(&mut e, 5, 0x21);
        assert_eq!(compatible(&mut e, this, g1, g2), 1);
        // The copier's own test is asked every time.
        assert_eq!(calls(&events, "base").len(), 6);
        // And when it says yes, the answer is 1.
        tag_calls(&mut e, &events, COPIER_ARE_MEMBERS_COMPATIBLE, "base", 1);
        assert_eq!(compatible(&mut e, this, a, d), 1);
    }

    #[test]
    fn struct_class_names_decide_the_pointer_member_cases() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        compat_setup(&mut e, &events, 0, "hkClass");
        let this = Ptr::new(e.mem.alloc(8));
        // Type 0x1c against 0x14 with the second's subtype 0x19 and the right struct class name.
        let first = member(&mut e, 0x1c, 3);
        let second = member(&mut e, 0x14, 0x19);
        assert_eq!(compatible(&mut e, this, first, second), 1);
        // The subtype pair 0x1c / 0x14 with equal types and the right name.
        let h1 = member(&mut e, 4, 0x1c);
        let h2 = member(&mut e, 4, 0x14);
        assert_eq!(compatible(&mut e, this, h1, h2), 1);
        // A different struct class name: neither case holds.
        compat_setup(&mut e, &events, 0, "hkOther");
        assert_eq!(compatible(&mut e, this, first, second), 0);
        assert_eq!(compatible(&mut e, this, h1, h2), 0);
    }

    /// The doubles `copySomeObjects` needs: the stream, layouts, copier (each copy appends
    /// `bytes_per_copy` bytes to the stream array and sets relocation entries), map, relocation
    /// info, `memCpy`. Returns the log of the copies' argument words.
    fn install_copy_doubles(
        e: &mut Engine,
        events: &Events,
        bytes_per_copy: u32,
    ) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let stream_array = Rc::new(Cell::new(0u32));
        let kept = stream_array.clone();
        let log = events.clone();
        e.register_double(OSTREAM_CONSTRUCT_FROM_ARRAY, move |e, a| {
            log.borrow_mut().push(("stream", a.to_vec()));
            kept.set(a[1]);
            e.mem.set_u32(a[0] + 8, 0x7a7a_0000);
            Ret::default()
        });
        for (address, tag) in [
            (LAYOUT_CONSTRUCT_FROM_RULES, "layout"),
            (MAP_RESERVE, "map_reserve"),
            (MAP_INSERT, "map_insert"),
            (MAP_CLEAR_AND_DEALLOCATE, "map_clear"),
            (MAP_DESTRUCT, "map_destruct"),
            (COPIER_DESTRUCT, "copier_destruct"),
            (RELOCATION_INFO_DESTRUCT, "relocations_destruct"),
            (OSTREAM_DESTRUCT, "stream_destruct"),
            (COPIER_CONSTRUCT, "copier"),
        ] {
            tag_calls(e, events, address, tag, 0);
        }
        e.register_double(MEM_CPY, {
            let log = events.clone();
            move |e, a| {
                log.borrow_mut().push(("memcpy", a.to_vec()));
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(a[0], &bytes);
                Ret::default()
            }
        });
        let copies = Rc::new(RefCell::new(Vec::new()));
        let kept_copies = copies.clone();
        let array = stream_array.clone();
        e.register_double(COPIER_COPY_OBJECT, move |e, a| {
            kept_copies.borrow_mut().push(a.to_vec());
            if bytes_per_copy == 0 {
                return Ret::default();
            }
            let bytes = array.get();
            let size = e.mem.u32(bytes + 4);
            if e.mem.u32(bytes) == 0 {
                let data = e.mem.alloc(256);
                e.mem.set_u32(bytes, data);
                e.mem.set_u32(bytes + 8, 0x4000_0100);
            }
            let data = e.mem.u32(bytes);
            for byte in 0..bytes_per_copy {
                e.mem
                    .set_u8(data + size + byte, (a[1] as u8).wrapping_add(byte as u8));
            }
            e.mem.set_u32(bytes + 4, size + bytes_per_copy);
            // Relocation info (a[5]): a local fixup (position, value) and an import entry.
            let relocations = a[5];
            let fixups = e.mem.alloc(64);
            e.mem.set_u32(relocations, fixups);
            e.mem.set_u32(relocations + 4, 1);
            e.mem.set_u32(fixups, 4);
            e.mem.set_u32(fixups + 4, 0x10);
            let imports = e.mem.alloc(64);
            e.mem.set_u32(relocations + 0xc, imports);
            e.mem.set_u32(relocations + 0x10, 1);
            e.mem.set_u32(imports, 8);
            e.mem.set_u32(imports + 4, 0x99);
            Ret::default()
        });
        copies
    }

    fn install_sink(e: &mut Engine, events: &Events) -> Ptr {
        let sink = e.mem.alloc(8);
        e.mem.set_u32(sink, SINK_TABLE);
        let mut table = vec![0u32; 6];
        table[4] = DOUBLES + 0x40;
        table[5] = DOUBLES + 0x50;
        e.put_vtable(SINK_TABLE, &table);
        tag_calls(e, events, DOUBLES + 0x40, "sink_block", 0);
        tag_calls(e, events, DOUBLES + 0x50, "sink_import", 0);
        Ptr::new(sink)
    }

    /// `count` objects `0x100 * (i + 1)` with classes `0x200 * (i + 1)`, an offsets array of
    /// as many 8-byte records (second word `0x5000 + i`), and the index array `0..count`.
    fn object_arrays(e: &mut Engine, count: u32) -> (Ptr, Ptr, Ptr) {
        let records = e.mem.alloc(0x10);
        let record_data = e.mem.alloc(8 * count + 8);
        let offsets = e.mem.alloc(0x10);
        let offset_data = e.mem.alloc(8 * count + 8);
        let indices = e.mem.alloc(0x10);
        let index_data = e.mem.alloc(4 * count + 4);
        for index in 0..count {
            e.mem.set_u32(record_data + 8 * index, 0x100 * (index + 1));
            e.mem
                .set_u32(record_data + 8 * index + 4, 0x200 * (index + 1));
            e.mem.set_u32(offset_data + 8 * index + 4, 0x5000 + index);
            e.mem.set_u32(index_data + 4 * index, index);
        }
        e.mem.set_u32(records, record_data);
        e.mem.set_u32(records + 4, count);
        e.mem.set_u32(offsets, offset_data);
        e.mem.set_u32(offsets + 4, count);
        e.mem.set_u32(indices, index_data);
        e.mem.set_u32(indices + 4, count);
        (Ptr::new(records), Ptr::new(offsets), Ptr::new(indices))
    }

    #[test]
    fn copy_some_objects_copies_each_object_and_fixes_the_offsets_up() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let (allocator, _) = install_router(&mut e, &events);
        let copies = install_copy_doubles(&mut e, &events, 16);
        let sink = install_sink(&mut e, &events);
        let (records, offsets, indices) = object_arrays(&mut e, 2);
        let block = e
            .call(0x00dc_12c0, &args![indices, records, offsets, sink])
            .u32();
        assert_ne!(block, 0);
        // Two copies of 16 bytes each: the objects at offsets 0 and 16 of the stream.
        let copies = copies.borrow();
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1..3], [0x100, 0x200]);
        assert_eq!(copies[1][1..3], [0x200, 0x400]);
        // The writer pointer is the stream's word at +8, `previous` the offsets record's old
        // second word.
        assert_eq!(copies[0][3], 0x7a7a_0000);
        assert_eq!(copies[0][4], 0x5000);
        assert_eq!(copies[1][4], 0x5001);
        let offset_data = e.mem.u32(offsets.addr());
        assert_eq!(e.mem.u32(offset_data), block);
        assert_eq!(e.mem.u32(offset_data + 8), block + 16);
        // The allocation and the sink were told: (block, 32 bytes, 5).
        assert_eq!(calls(&events, "alloc"), vec![vec![allocator, 32]]);
        assert_eq!(
            calls(&events, "sink_block"),
            vec![vec![sink.addr(), block, 32, 5]]
        );
        assert_eq!(calls(&events, "memcpy").len(), 1);
        assert_eq!(calls(&events, "memcpy")[0][0], block);
        assert_eq!(calls(&events, "memcpy")[0][2], 32);
        // Local fixups: *(block + 4) = 0x10 + block (written twice, once per copy's entry
        // list, the last copy's list is what is left).
        assert_eq!(e.mem.u32(block + 4), 0x10 + block);
        // The import entry (8, 0x99) goes to the sink as (0x99, block + 8).
        assert_eq!(
            calls(&events, "sink_import"),
            vec![vec![sink.addr(), 0x99, block + 8]]
        );
        // The map got both objects with their offsets.
        let inserts = calls(&events, "map_insert");
        assert_eq!(inserts.len(), 2);
        assert_eq!(inserts[0][2..], [0x100, 0]);
        assert_eq!(inserts[1][2..], [0x200, 16]);
        // The temporaries went in order and the stream's array was given back (flagged: no).
        let order = tags(&events);
        assert_eq!(
            &order[order.len() - 5..],
            &[
                "map_clear",
                "map_destruct",
                "relocations_destruct",
                "stream_destruct",
                "free"
            ]
        );
        assert_eq!(calls(&events, "free").len(), 1);
        assert_eq!(calls(&events, "free")[0][2], 0x100);
    }

    #[test]
    fn copy_some_objects_with_nothing_written_returns_null() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_router(&mut e, &events);
        install_copy_doubles(&mut e, &events, 0);
        let sink = install_sink(&mut e, &events);
        let (records, offsets, indices) = object_arrays(&mut e, 1);
        assert_eq!(
            e.call(0x00dc_12c0, &args![indices, records, offsets, sink])
                .u32(),
            0
        );
        assert!(calls(&events, "alloc").is_empty());
        assert!(calls(&events, "sink_block").is_empty());
        assert_eq!(calls(&events, "copier_destruct").len(), 1);
        // The stream's array carries the flag: nothing is freed.
        assert!(calls(&events, "free").is_empty());
        // With no indices at all the map and stream are still destroyed.
        events.borrow_mut().clear();
        e.mem.set_u32(indices.addr() + 4, 0);
        assert_eq!(
            e.call(0x00dc_12c0, &args![indices, records, offsets, sink])
                .u32(),
            0
        );
        let order = tags(&events);
        assert_eq!(
            &order[order.len() - 4..],
            &[
                "map_clear",
                "map_destruct",
                "relocations_destruct",
                "stream_destruct"
            ]
        );
    }

    #[test]
    fn copy_objects_numbers_the_objects_and_gives_the_temporary_array_back() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_router(&mut e, &events);
        install_copy_doubles(&mut e, &events, 0);
        let sink = install_sink(&mut e, &events);
        let kept = events.clone();
        e.register_double(ARRAY_RESERVE, move |e, a| {
            kept.borrow_mut().push(("reserve", a.to_vec()));
            let data = e.mem.alloc(4 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let (records, offsets, _) = object_arrays(&mut e, 3);
        assert_eq!(e.call(0x00dc_15a0, &args![records, offsets, sink]).u32(), 0);
        let reserves = calls(&events, "reserve");
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][2..], [3, 4]);
        // copySomeObjects ran over the objects in order.
        let objects: Vec<u32> = calls(&events, "stream").iter().map(|c| c[0]).collect();
        assert_eq!(objects.len(), 1);
        // The temporary array (3 words) is freed through the router's +0xc allocator.
        let freed = calls(&events, "free_temporary");
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 12);
        // No objects: no reserve and nothing to free but the zero-capacity block.
        events.borrow_mut().clear();
        e.mem.set_u32(records.addr() + 4, 0);
        e.call(0x00dc_15a0, &args![records, offsets, sink]);
        assert!(calls(&events, "reserve").is_empty());
        assert!(calls(&events, "free_temporary").is_empty());
    }
}
