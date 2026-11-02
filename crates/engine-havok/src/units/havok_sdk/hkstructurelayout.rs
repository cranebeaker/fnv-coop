//! `Havok SDK/hkstructurelayout.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStructureLayout` is one word, its `LayoutRules` (Xbox PDB: `m_bytesInPointer`,
//! `m_littleEndian`, `m_reusePaddingOptimization`, `m_emptyBaseClassOptimization`, one byte
//! each). The three free functions here (engine map names `computeMemberSize`,
//! `getLayoutAlignment` and `retargetClassInplace`) lay a class out for a target: they read
//! the rules through a pointer (their second argument for the first and third; the pointer
//! size for the second).
//!
//! The class and member records are `hkClass` and `hkClassMember`: a member is `0x18` bytes
//! with its type byte at `+0xc`, the sub-type byte of an array at `+0xd`, the signed C-style
//! array size at `+0xe`, the flags word at `+0x10` and the byte offset at `+0x12`; the member's
//! struct class is the word at `+4`. The class accessors are called by address.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStructureLayout`'s default rules word (a global at `011b7b88`).
const DEFAULT_LAYOUT_RULES: u32 = 0x011b_7b88;
/// `hkClassMember`: `getSizeInBytes` (`00d86980`), `getCstyleArraySize` (`00d86940`),
/// `getStructClass` (`00d86910`) and `getAlignment` (`00d86a90`); all thiscall on the member,
/// no stack words.
const MEMBER_GET_SIZE_IN_BYTES: u32 = 0x00d8_6980;
const MEMBER_GET_CSTYLE_ARRAY_SIZE: u32 = 0x00d8_6940;
const MEMBER_GET_STRUCT_CLASS: u32 = 0x00d8_6910;
const MEMBER_GET_ALIGNMENT: u32 = 0x00d8_6a90;
/// `hkClass`, thiscall on the class: `getObjectSize` (`00d470d0`), `getNumMembers`
/// (`00d46f00`), `getMember` (`00d46f20`, one stack word `index`), `getMember` overload 2
/// (`00d46f60`, the same), `getNumDeclaredMembers` (`00d46f70`), `getDeclaredMember`
/// (`00d46f80`, `index`), `getParent` (`00d46d50`), the unnamed `00d46e20` (the word at `+0xc`,
/// which the layout code uses as a count of pointer-sized slots: the implemented interfaces)
/// and the object size setter `00d470e0` (one stack word `size`).
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
const CLASS_GET_NUM_MEMBERS: u32 = 0x00d4_6f00;
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;
const CLASS_GET_MEMBER_OV2: u32 = 0x00d4_6f60;
const CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00d4_6f70;
const CLASS_GET_DECLARED_MEMBER: u32 = 0x00d4_6f80;
const CLASS_GET_PARENT: u32 = 0x00d4_6d50;
const CLASS_SLOT_COUNT: u32 = 0x00d4_6e20;
const CLASS_SET_OBJECT_SIZE: u32 = 0x00d4_70e0;
/// `hkMapBase` (the pointer map of classes already laid out), thiscall on the map: `insert`
/// (`00cb1870`, `(allocator, key, value)`) and `findKey` (`00cb18f0`, `(key)`, answering the
/// slot or `hashMod + 1` when absent; `hashMod` is the word at map `+8`).
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// The calling thread's memory router (`00c85750`): its word at `+0x10` is the heap allocator,
/// the one at `+0xc` the temporary allocator.
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// An array insert (`00d10710`, thiscall on the array, three stack words `(allocator, index,
/// pointer to the value)`).
const ARRAY_INSERT: u32 = 0x00d1_0710;
/// The `hkOstream` the error message is built in: the constructor over a buffer (`00cb1600`,
/// thiscall, `(buffer, size)`), `operator<<` of a C string (`00cc01c0`, thiscall, `(text)`)
/// and the destructor (`00cc00e0`, thiscall).
const OSTREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
/// The global holding the error handler (`01268360`): its vtable slot `+0xc` reports a message
/// `(level, id, text, file, line)` and answers non-zero to ask for a break.
const ERROR_HANDLER: u32 = 0x0126_8360;
/// The message text (`"Unknown class member type in structureLayout::getMemberSize()."`) and
/// the file name (`".\Util\hkStructureLayout.cpp"`) strings, and the report's id and line.
const UNKNOWN_MEMBER_TYPE_MESSAGE: u32 = 0x010d_6b54;
const SOURCE_FILE_NAME: u32 = 0x010d_6b34;
const UNKNOWN_MEMBER_TYPE_ID: u32 = 0x50a1_8b58;
const UNKNOWN_MEMBER_TYPE_LINE: u32 = 0xc3;
/// The stream object and the buffer behind it: the object is `0xc` bytes, the buffer `0x200`.
const OSTREAM_FRAME: u32 = 0x20c;
const OSTREAM_OBJECT_SIZE: u32 = 0xc;
const OSTREAM_BUFFER_SIZE: u32 = 0x200;
/// `hkArray` capacity word of an array with no storage that must not be deallocated.
const EMPTY_ARRAY_CAPACITY: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

layout! {
    /// `hkStructureLayout` (Xbox PDB), one word: its `LayoutRules`.
    pub struct HkStructureLayout: 0x4 {
        /// `m_rules` (Xbox PDB): the four rule bytes as one word.
        0x00 m_rules: u32,
    }
}

/// The type byte of the member at `member` (`+0xc`).
fn type_byte(e: &Engine, member: u32) -> u32 {
    u32::from(e.mem.u8(member.wrapping_add(0xc)))
}

/// The calling thread's heap allocator (router `+0x10`) and temporary allocator (`+0xc`).
fn allocators(e: &mut Engine) -> (u32, u32) {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    (e.mem.u32(router + 0x10), e.mem.u32(router + 0xc))
}

// Translated from 00dc1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStructureLayout::hkStructureLayout` (Xbox PDB), `__fastcall`: the rules become the
/// default rules word (the global at `011b7b88`). Returns `this`.
pub fn hk_structure_layout_hk_structure_layout(
    e: &mut Engine,
    this: Ptr<HkStructureLayout>,
) -> Ptr<HkStructureLayout> {
    let rules: u32 = e.global(DEFAULT_LAYOUT_RULES);
    e.set(this, HkStructureLayout::m_rules, rules);
    this
}

// Translated from 00dc1680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStructureLayout::hkStructureLayout` copy constructor (Xbox PDB, overload 2), thiscall,
/// one stack word `other`: copies the rules word. Returns `this`.
pub fn hk_structure_layout_hk_structure_layout_ov2(
    e: &mut Engine,
    this: Ptr<HkStructureLayout>,
    other: Ptr<HkStructureLayout>,
) -> Ptr<HkStructureLayout> {
    let rules = e.get(other, HkStructureLayout::m_rules);
    e.set(this, HkStructureLayout::m_rules, rules);
    this
}

/// The size of a struct or array member: the `hkClassMember::getCstyleArraySize` count
/// (`00d86940`, a signed 16-bit value) times `element_size`, or `element_size` alone when
/// the count is 0. The exe calls the getter a second time to use its result.
fn sized_by_array(e: &mut Engine, member: u32, element_size: i32) -> u32 {
    let count = e.call(MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member]).i32();
    if count != 0 {
        let count = e.call(MEMBER_GET_CSTYLE_ARRAY_SIZE, &args![member]).i32();
        return count.wrapping_mul(element_size) as u32;
    }
    element_size as u32
}

// Translated from 00dc1690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `computeMemberSize` (engine map name), cdecl `(member, rules, type)`: the byte size of a
/// class member of the given type, with `rules` pointing at the layout rules (its first byte
/// is the pointer size, `ptr` below):
///
/// - types `1..=0x12`, `0x18`, `0x1f`, `0x20`: `hkClassMember::getSizeInBytes` (`00d86980`);
/// - `0x13`: the same function again for the member's sub-type (the byte at `member + 0xd`);
/// - `0x14`, `0x15`, `0x1d`, `0x1e`, `0x21`: `ptr`, times the C-style array size when it is
///   not 0;
/// - `0x16`: `ptr + 8`; `0x1a`: `ptr + 4`; `0x17`: 0; `0x1b`: `ptr * 2 + 4`; `0x1c`: `ptr * 2`;
/// - `0x19`: the object size of the member's struct class (`getStructClass`, `getObjectSize`),
///   times the C-style array size when it is not 0.
///
/// Any other type (0 or above `0x21`) builds the message "Unknown class member type in
/// structureLayout::getMemberSize()." in a `0x200`-byte `hkOstream` and reports it through the
/// error handler (slot `+0xc` of the global at `01268360`, `(3, 0x50a18b58, text, file,
/// 0xc3)`); the exe executes `INT3` (`00dc17eb`) when the handler answers non-zero, which
/// panics here; otherwise it returns 0.
pub fn compute_member_size(e: &mut Engine, member: Ptr, rules: Ptr, kind: u32) -> u32 {
    match kind {
        1..=0x12 | 0x18 | 0x1f | 0x20 => e.call(MEMBER_GET_SIZE_IN_BYTES, &args![member]).u32(),
        0x13 => {
            let sub_type = u32::from(e.mem.u8(member.addr() + 0xd));
            compute_member_size(e, member, rules, sub_type)
        }
        0x14 | 0x15 | 0x1d | 0x1e | 0x21 => {
            let pointer_size = i32::from(e.mem.u8(rules.addr()));
            sized_by_array(e, member.addr(), pointer_size)
        }
        0x16 => 4 + 4 + u32::from(e.mem.u8(rules.addr())),
        0x17 => 0,
        0x19 => {
            let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
            let object_size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
            sized_by_array(e, member.addr(), object_size)
        }
        0x1a => 4 + u32::from(e.mem.u8(rules.addr())),
        0x1b => u32::from(e.mem.u8(rules.addr())) * 2 + 4,
        0x1c => u32::from(e.mem.u8(rules.addr())) * 2,
        _ => unknown_member_type(e),
    }
}

/// The default case of `computeMemberSize`: the report, then 0 (or the `INT3`).
fn unknown_member_type(e: &mut Engine) -> u32 {
    let broke = e.with_stack(OSTREAM_FRAME, |e, stream| {
        let buffer = stream.addr() + OSTREAM_OBJECT_SIZE;
        e.call(
            OSTREAM_CONSTRUCTOR,
            &args![stream, buffer, OSTREAM_BUFFER_SIZE],
        );
        e.call(
            OSTREAM_WRITE_TEXT,
            &args![stream, UNKNOWN_MEMBER_TYPE_MESSAGE],
        );
        let handler: u32 = e.global(ERROR_HANDLER);
        let broke = e
            .vcall(
                handler,
                0xc,
                &args![
                    3u32,
                    UNKNOWN_MEMBER_TYPE_ID,
                    buffer,
                    SOURCE_FILE_NAME,
                    UNKNOWN_MEMBER_TYPE_LINE
                ],
            )
            .u32()
            != 0;
        if !broke {
            e.call(OSTREAM_DESTRUCTOR, &args![stream]);
        }
        broke
    });
    assert!(!broke, "INT3 at 00dc17eb (computeMemberSize)");
    0
}

// Translated from 00dc1850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getLayoutAlignment` (engine map name), cdecl `(member, type, pointer size)`: the
/// alignment of a class member of the given type, then raised to at least 16 when the flags
/// word (`member + 0x10`, masked with `0x180`) is exactly `0x100`, and to at least 8 when the
/// masked flags are anything else non-zero (`0x80` or `0x180`):
///
/// - types `0..=0x12`, `0x18`, `0x1f`, `0x20`: `hkClassMember::getAlignment` (`00d86a90`);
/// - `0x13`: the same for the sub-type (the byte at `member + 0xd`);
/// - `0x14`, `0x15`, `0x16`, `0x1a..=0x1e`, `0x21`: the pointer size;
/// - `0x19`: the largest alignment among the struct class's members (`getNumMembers`,
///   `getMember`), at least 1;
/// - `0x17`, and any type above `0x21`: -1.
pub fn get_layout_alignment(
    e: &mut Engine,
    member: Ptr,
    member_type: u32,
    pointer_size: i32,
) -> i32 {
    let mut alignment = -1i32;
    if member_type <= 0x21 {
        match member_type {
            0..=0x12 | 0x18 | 0x1f | 0x20 => {
                alignment = e.call(MEMBER_GET_ALIGNMENT, &args![member]).i32();
            }
            0x13 => {
                let sub_type = u32::from(e.mem.u8(member.addr() + 0xd));
                alignment = get_layout_alignment(e, member, sub_type, pointer_size);
            }
            0x14 | 0x15 | 0x16 | 0x1a..=0x1e | 0x21 => alignment = pointer_size,
            0x19 => {
                let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
                let mut largest = 1i32;
                let mut index = 0i32;
                let mut count = e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32();
                if count > 0 {
                    loop {
                        let inner = e.call(CLASS_GET_MEMBER, &args![class, index]).u32();
                        let inner_type = type_byte(e, inner);
                        let found =
                            get_layout_alignment(e, Ptr::new(inner), inner_type, pointer_size);
                        if found > largest {
                            largest = found;
                        }
                        index += 1;
                        count = e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32();
                        if index >= count {
                            break;
                        }
                    }
                }
                alignment = largest;
            }
            // `0x17` and anything else in range: no alignment (the -1 stays).
            _ => {}
        }
    }
    let flags = u32::from(e.mem.u16(member.addr() + 0x10)) & 0x180;
    if flags != 0 {
        let minimum = u32::from(flags == 0x100) as i32 * 8 + 8;
        if minimum > alignment {
            alignment = minimum;
        }
    }
    alignment
}

/// `value` rounded up to a multiple of `alignment` the way the code does it with `IDIV`:
/// when `value % alignment` is not 0, `alignment - remainder` is added. (A zero alignment is
/// the divide fault of the exe; here it panics.)
fn round_up(value: i32, alignment: i32) -> i32 {
    let remainder = value % alignment;
    if remainder != 0 {
        value.wrapping_add(alignment.wrapping_sub(remainder))
    } else {
        value
    }
}

// Translated from 00dc1960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `retargetClassInplace` (engine map name), cdecl `(class, rules, map, recurse)`: lays the
/// class out for the rules and rewrites its member offsets and object size in place.
///
/// The class is first entered into `map` (`insert(allocator, class, 0)`, heap allocator);
/// when `recurse` is non-zero every member that has a struct class (`member + 4`) not yet in
/// the map (`findKey` above `hashMod`) is retargeted first, recursively. The class and its
/// parents are collected base first (`getParent`, each inserted at index 0 of a local array
/// with the temporary allocator). Then, walking the classes with one running byte offset and
/// one largest alignment (starting at 1, kept across classes): each pointer-sized interface
/// slot (`00d46e20` of them; the offset is aligned to the pointer size and advanced by it);
/// each declared member: when the offset is 0 for a class after the first and the rules'
/// fourth byte (the empty-base optimization) is 0 the offset starts at 1, the offset is
/// aligned to the member's alignment (`getLayoutAlignment`), written to the member's 16-bit
/// offset when different, and advanced by `computeMemberSize`. The class's object size is the
/// offset rounded up to the largest alignment (1 for an empty class), set when different; the
/// running offset continues from the rounded value unless the rules' third byte (the padding
/// reuse optimization) is non-zero. The divisions are signed, as the exe's.
pub fn retarget_class_inplace(e: &mut Engine, class: Ptr, rules: Ptr, map: Ptr, recurse: u32) {
    let (heap_allocator, _) = allocators(e);
    e.call(MAP_INSERT, &args![map, heap_allocator, class, 0u32]);
    if recurse != 0 {
        let count = e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32();
        for index in 0..count {
            let member = e.call(CLASS_GET_MEMBER_OV2, &args![class, index]).u32();
            if e.mem.u32(member + 4) != 0 {
                let struct_class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
                let slot = e.call(MAP_FIND_KEY, &args![map, struct_class]).i32();
                if slot > e.mem.i32(map.addr() + 8) {
                    let struct_class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
                    retarget_class_inplace(e, Ptr::new(struct_class), rules, map, recurse);
                }
            }
        }
    }
    e.with_stack(0xc, |e, chain| {
        let chain_address = chain.addr();
        e.mem.set_u32(chain_address, 0);
        e.mem.set_u32(chain_address + 4, 0);
        e.mem.set_u32(chain_address + 8, EMPTY_ARRAY_CAPACITY);
        // The class and its parents, base class first.
        let mut current = class.addr();
        while current != 0 {
            let (_, temp_allocator) = allocators(e);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), current);
                e.call(ARRAY_INSERT, &args![chain, temp_allocator, 0u32, slot]);
            });
            current = e.call(CLASS_GET_PARENT, &args![current]).u32();
        }
        let pointer_size = i32::from(e.mem.u8(rules.addr()));
        let mut offset = 0i32;
        let mut largest = 1i32;
        let mut class_index = 0i32;
        if e.mem.i32(chain_address + 4) > 0 {
            loop {
                let data = e.mem.u32(chain_address);
                let current = e.mem.u32(data + 4 * class_index as u32);
                let mut slot = 0i32;
                let mut slots = e.call(CLASS_SLOT_COUNT, &args![current]).i32();
                if slots > 0 {
                    loop {
                        offset = round_up(offset, pointer_size).wrapping_add(pointer_size);
                        if pointer_size > largest {
                            largest = pointer_size;
                        }
                        slot += 1;
                        slots = e.call(CLASS_SLOT_COUNT, &args![current]).i32();
                        if slot >= slots {
                            break;
                        }
                    }
                }
                let mut member_index = 0i32;
                let mut declared = e
                    .call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![current])
                    .i32();
                if declared > 0 {
                    loop {
                        let member = e
                            .call(CLASS_GET_DECLARED_MEMBER, &args![current, member_index])
                            .u32();
                        if offset == 0 && class_index != 0 && e.mem.u8(rules.addr() + 3) == 0 {
                            offset = 1;
                        }
                        let kind = type_byte(e, member);
                        let alignment =
                            get_layout_alignment(e, Ptr::new(member), kind, pointer_size);
                        if alignment > largest {
                            largest = alignment;
                        }
                        offset = round_up(offset, alignment);
                        if u32::from(e.mem.u16(member + 0x12)) != offset as u32 {
                            e.mem.set_u16(member + 0x12, offset as u16);
                        }
                        let size = compute_member_size(e, Ptr::new(member), rules, kind);
                        offset = offset.wrapping_add(size as i32);
                        member_index += 1;
                        declared = e
                            .call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![current])
                            .i32();
                        if member_index >= declared {
                            break;
                        }
                    }
                }
                let rounded = round_up(offset, largest);
                let object_size = if rounded == 0 { 1 } else { rounded };
                if e.call(CLASS_GET_OBJECT_SIZE, &args![current]).i32() != object_size {
                    e.call(CLASS_SET_OBJECT_SIZE, &args![current, object_size]);
                }
                if e.mem.u8(rules.addr() + 2) == 0 {
                    offset = rounded;
                }
                class_index += 1;
                if class_index >= e.mem.i32(chain_address + 4) {
                    break;
                }
            }
        }
        let (_, temp_allocator) = allocators(e);
        let capacity_word = e.mem.u32(chain_address + 8);
        e.mem.set_u32(chain_address + 4, 0);
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(chain_address);
            e.vcall(
                temp_allocator,
                0x10,
                &args![data, (capacity_word & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dc_1670,
            hk_structure_layout_hk_structure_layout(
                Ptr<HkStructureLayout>,
            ) -> Ptr<HkStructureLayout>
        ),
        entry!(
            0x00dc_1680,
            hk_structure_layout_hk_structure_layout_ov2(
                Ptr<HkStructureLayout>,
                Ptr<HkStructureLayout>,
            ) -> Ptr<HkStructureLayout>
        ),
        entry!(0x00dc_1690, compute_member_size(Ptr, Ptr, u32) -> u32),
        entry!(0x00dc_1850, get_layout_alignment(Ptr, u32, i32) -> i32),
        entry!(0x00dc_1960, retarget_class_inplace(Ptr, Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const HEAP_ALLOCATOR: u32 = 0x1111_0000;
    const FREE_SLOT: u32 = 0x0900_0060;
    /// Rules words: pointer size 4, little endian, and the two optimization bytes.
    const RULES_PLAIN: [u8; 4] = [4, 1, 0, 0];

    type Frees = Rc<RefCell<Vec<(u32, u32)>>>;

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Writes a member record: type, sub-type, C array size, flags, the offset (`0xffff`
    /// until laid out), and for the doubles a size byte at `+0x14` and an alignment byte
    /// at `+0x15`; the struct class at `+4`.
    #[allow(clippy::too_many_arguments)]
    fn set_member(
        e: &mut Engine,
        at: u32,
        kind: u8,
        sub: u8,
        array: i16,
        flags: u16,
        size: u8,
        align: u8,
        class: u32,
    ) {
        e.mem.set_u32(at + 4, class);
        e.mem.set_u8(at + 0xc, kind);
        e.mem.set_u8(at + 0xd, sub);
        e.mem.set_i16(at + 0xe, array);
        e.mem.set_u16(at + 0x10, flags);
        e.mem.set_u16(at + 0x12, 0xffff);
        e.mem.set_u8(at + 0x14, size);
        e.mem.set_u8(at + 0x15, align);
    }

    /// A plain member of `kind` 5 with the given size and alignment.
    fn plain_member(e: &mut Engine, at: u32, size: u8, align: u8) {
        set_member(e, at, 5, 0, 0, 0, size, align, 0);
    }

    /// A class record: parent at `+4`, object size at `+8`, interface count at `+0xc`, the
    /// declared member table at `+0x18` and its count at `+0x1c`. Returns the class and the
    /// table.
    fn make_class(e: &mut Engine, parent: u32, interfaces: u32, members: u32) -> (u32, u32) {
        let class = e.mem.alloc(0x20);
        let table = e.mem.alloc(0x18 * members.max(1));
        e.mem.set_u32(class + 4, parent);
        e.mem.set_u32(class + 8, 0xdead);
        e.mem.set_u32(class + 0xc, interfaces);
        e.mem.set_u32(class + 0x18, table);
        e.mem.set_u32(class + 0x1c, members);
        (class, table)
    }

    fn rules(e: &mut Engine, bytes: [u8; 4]) -> u32 {
        let at = e.mem.alloc(4);
        e.mem.write(at, &bytes);
        at
    }

    /// The class accessors over the test records above, and the member accessors.
    fn install_class_doubles(e: &mut Engine) {
        e.register(MEMBER_GET_SIZE_IN_BYTES, |e, a| Ret {
            eax: u32::from(e.mem.u8(a[0] + 0x14)),
            ..Ret::default()
        });
        e.register(MEMBER_GET_ALIGNMENT, |e, a| Ret {
            eax: u32::from(e.mem.u8(a[0] + 0x15)),
            ..Ret::default()
        });
        e.register(MEMBER_GET_CSTYLE_ARRAY_SIZE, |e, a| Ret {
            eax: i32::from(e.mem.i16(a[0] + 0xe)) as u32,
            ..Ret::default()
        });
        e.register(MEMBER_GET_STRUCT_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(CLASS_GET_OBJECT_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(CLASS_SET_OBJECT_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(CLASS_GET_NUM_DECLARED_MEMBERS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        e.register(CLASS_GET_DECLARED_MEMBER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18) + a[1] * 0x18,
            ..Ret::default()
        });
        e.register(CLASS_GET_PARENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(CLASS_SLOT_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(CLASS_GET_NUM_MEMBERS, |e, a| {
            let mut total = 0;
            let mut class = a[0];
            while class != 0 {
                total += e.mem.u32(class + 0x1c);
                class = e.mem.u32(class + 4);
            }
            Ret {
                eax: total,
                ..Ret::default()
            }
        });
        for address in [CLASS_GET_MEMBER, CLASS_GET_MEMBER_OV2] {
            // The base class's members come first.
            e.register(address, |e, a| {
                let mut chain = vec![];
                let mut class = a[0];
                while class != 0 {
                    chain.push(class);
                    class = e.mem.u32(class + 4);
                }
                let mut index = a[1];
                for class in chain.into_iter().rev() {
                    let count = e.mem.u32(class + 0x1c);
                    if index < count {
                        return Ret {
                            eax: e.mem.u32(class + 0x18) + index * 0x18,
                            ..Ret::default()
                        };
                    }
                    index -= count;
                }
                Ret::default()
            });
        }
    }

    /// The router (heap allocator word `HEAP_ALLOCATOR`, temporary allocator an object whose
    /// slot `0x10` logs `(block, bytes)`), the map doubles (the keys are stored behind the
    /// map's `hashMod` word) and the array insert.
    fn install_services(e: &mut Engine) -> Frees {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let temp = e.mem.alloc(8);
        e.mem.set_u32(temp, vtable);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, HEAP_ALLOCATOR);
        e.mem.set_u32(router + 0xc, temp);
        answer(e, GET_MEMORY_ROUTER, router);
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&frees);
        e.register_double(FREE_SLOT, move |_, a| {
            seen.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        e.register(MAP_INSERT, |e, a| {
            let count = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0x10 + 4 * count, a[2]);
            e.mem.set_u32(a[0] + 0xc, count + 1);
            Ret::default()
        });
        e.register(MAP_FIND_KEY, |e, a| {
            let count = e.mem.u32(a[0] + 0xc);
            for index in 0..count {
                if e.mem.u32(a[0] + 0x10 + 4 * index) == a[1] {
                    return Ret {
                        eax: index,
                        ..Ret::default()
                    };
                }
            }
            Ret {
                eax: e.mem.u32(a[0] + 8) + 1,
                ..Ret::default()
            }
        });
        e.register(ARRAY_INSERT, |e, a| {
            let (array, index, value) = (a[0], a[2], e.mem.u32(a[3]));
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let block = e.mem.alloc((size + 1) * 4);
            let mut words: Vec<u32> = (0..size).map(|i| e.mem.u32(old + 4 * i)).collect();
            words.insert(index as usize, value);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(block + 4 * i as u32, *word);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 4, size + 1);
            e.mem.set_u32(array + 8, size + 1);
            Ret::default()
        });
        frees
    }

    fn map_record(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(0x40);
        e.mem.set_u32(map + 8, 15);
        map
    }

    #[test]
    fn constructors_store_the_default_rules_and_copy_them() {
        let mut e = Engine::new();
        e.map(0x011b_7000, 0x1000);
        e.set_global(DEFAULT_LAYOUT_RULES, 0x0100_0104u32);
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x00dc_1670, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0100_0104);
        let copy = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0001_0208);
        assert_eq!(e.call(0x00dc_1680, &args![copy, this]).u32(), copy);
        assert_eq!(e.mem.u32(copy), 0x0001_0208);
    }

    fn size_of_member(e: &mut Engine, member: u32, rules: u32, kind: u32) -> u32 {
        e.call(0x00dc_1690, &args![member, rules, kind]).u32()
    }

    #[test]
    fn member_size_by_type() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        let rules = rules(&mut e, RULES_PLAIN);
        let member = e.mem.alloc(0x18);
        // The size the member reports for the plain types.
        set_member(&mut e, member, 5, 0, 0, 0, 12, 4, 0);
        for kind in [1u32, 5, 0x12, 0x18, 0x1f, 0x20] {
            assert_eq!(
                size_of_member(&mut e, member, rules, kind),
                12,
                "type {kind:x}"
            );
        }
        // 0x13: the sub-type's size.
        set_member(&mut e, member, 0x13, 0x1c, 0, 0, 12, 4, 0);
        assert_eq!(size_of_member(&mut e, member, rules, 0x13), 8);
        // Pointer-sized types, with and without a C array size.
        e.call_log = Some(vec![]);
        for kind in [0x14u32, 0x15, 0x1d, 0x1e, 0x21] {
            set_member(&mut e, member, 0x14, 0, 0, 0, 0, 0, 0);
            assert_eq!(
                size_of_member(&mut e, member, rules, kind),
                4,
                "type {kind:x}"
            );
            set_member(&mut e, member, 0x14, 0, 3, 0, 0, 0, 0);
            assert_eq!(
                size_of_member(&mut e, member, rules, kind),
                12,
                "type {kind:x}"
            );
        }
        // The getter runs twice when the size is not 0 (once otherwise).
        assert_eq!(calls_to(&e, MEMBER_GET_CSTYLE_ARRAY_SIZE).len(), 5 * 3);
        // The fixed forms.
        assert_eq!(size_of_member(&mut e, member, rules, 0x16), 12);
        assert_eq!(size_of_member(&mut e, member, rules, 0x1a), 8);
        assert_eq!(size_of_member(&mut e, member, rules, 0x17), 0);
        assert_eq!(size_of_member(&mut e, member, rules, 0x1b), 12);
        assert_eq!(size_of_member(&mut e, member, rules, 0x1c), 8);
        // A larger pointer size scales them.
        let wide = self::rules(&mut e, [8, 1, 0, 0]);
        assert_eq!(size_of_member(&mut e, member, wide, 0x16), 16);
        assert_eq!(size_of_member(&mut e, member, wide, 0x1c), 16);
    }

    #[test]
    fn struct_member_size_is_the_class_size_times_the_array_size() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        let rules = rules(&mut e, RULES_PLAIN);
        let (class, _) = make_class(&mut e, 0, 0, 0);
        e.mem.set_u32(class + 8, 20);
        let member = e.mem.alloc(0x18);
        set_member(&mut e, member, 0x19, 0, 0, 0, 0, 0, class);
        assert_eq!(size_of_member(&mut e, member, rules, 0x19), 20);
        set_member(&mut e, member, 0x19, 0, 2, 0, 0, 0, class);
        assert_eq!(size_of_member(&mut e, member, rules, 0x19), 40);
    }

    #[test]
    fn unknown_member_types_report_through_the_error_handler_and_answer_zero() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        install_class_doubles(&mut e);
        let rules = rules(&mut e, RULES_PLAIN);
        let handler_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(handler_vtable + 0xc, 0x0900_0070);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(handler, handler_vtable);
        e.set_global(ERROR_HANDLER, handler);
        answer(&mut e, 0x0900_0070, 0);
        answer(&mut e, OSTREAM_CONSTRUCTOR, 0);
        answer(&mut e, OSTREAM_WRITE_TEXT, 0);
        answer(&mut e, OSTREAM_DESTRUCTOR, 0);
        let member = e.mem.alloc(0x18);
        for kind in [0u32, 0x22, 0x40] {
            e.call_log = Some(vec![]);
            assert_eq!(size_of_member(&mut e, member, rules, kind), 0);
            let constructed = calls_to(&e, OSTREAM_CONSTRUCTOR);
            assert_eq!(constructed.len(), 1);
            let (stream, buffer) = (constructed[0][0], constructed[0][1]);
            assert_eq!(buffer, stream + 0xc);
            assert_eq!(constructed[0][2], 0x200);
            assert_eq!(
                calls_to(&e, OSTREAM_WRITE_TEXT),
                vec![vec![stream, UNKNOWN_MEMBER_TYPE_MESSAGE]]
            );
            assert_eq!(
                calls_to(&e, 0x0900_0070),
                vec![vec![
                    handler,
                    3,
                    0x50a1_8b58,
                    buffer,
                    SOURCE_FILE_NAME,
                    0xc3
                ]]
            );
            assert_eq!(calls_to(&e, OSTREAM_DESTRUCTOR), vec![vec![stream]]);
        }
    }

    #[test]
    #[should_panic(expected = "INT3 at 00dc17eb")]
    fn an_unknown_member_type_breaks_when_the_handler_asks_for_it() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let rules = rules(&mut e, RULES_PLAIN);
        let handler_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(handler_vtable + 0xc, 0x0900_0070);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(handler, handler_vtable);
        e.set_global(ERROR_HANDLER, handler);
        answer(&mut e, 0x0900_0070, 1);
        answer(&mut e, OSTREAM_CONSTRUCTOR, 0);
        answer(&mut e, OSTREAM_WRITE_TEXT, 0);
        answer(&mut e, OSTREAM_DESTRUCTOR, 0);
        let member = e.mem.alloc(0x18);
        size_of_member(&mut e, member, rules, 0x30);
    }

    fn alignment_of(e: &mut Engine, member: u32, kind: u32) -> i32 {
        e.call(0x00dc_1850, &args![member, kind, 4i32]).i32()
    }

    #[test]
    fn alignment_by_type() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        let member = e.mem.alloc(0x18);
        set_member(&mut e, member, 5, 0, 0, 0, 0, 8, 0);
        for kind in [0u32, 1, 0x12, 0x18, 0x1f, 0x20] {
            assert_eq!(alignment_of(&mut e, member, kind), 8, "type {kind:x}");
        }
        set_member(&mut e, member, 0x13, 0x05, 0, 0, 0, 2, 0);
        assert_eq!(alignment_of(&mut e, member, 0x13), 2);
        // The pointer size (4 here) for the pointer-like types.
        for kind in [0x14u32, 0x15, 0x16, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x21] {
            assert_eq!(alignment_of(&mut e, member, kind), 4, "type {kind:x}");
        }
        // No alignment: -1, also above 0x21.
        assert_eq!(alignment_of(&mut e, member, 0x17), -1);
        assert_eq!(alignment_of(&mut e, member, 0x22), -1);
        assert_eq!(alignment_of(&mut e, member, 0xffff_ffff), -1);
    }

    #[test]
    fn alignment_flags_raise_it_to_eight_or_sixteen() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        let member = e.mem.alloc(0x18);
        for (flags, align, expected) in [
            (0x000u16, 4u8, 4),
            (0x080, 4, 8),
            (0x100, 4, 16),
            (0x180, 4, 8),
            (0x080, 32, 32),
            (0x100, 32, 32),
            (0x180, 2, 8),
            // Bits outside 0x180 are ignored.
            (0x07f, 2, 2),
        ] {
            set_member(&mut e, member, 5, 0, 0, flags, 0, align, 0);
            assert_eq!(
                alignment_of(&mut e, member, 5),
                expected,
                "{flags:x}/{align}"
            );
        }
        // The flags apply to a type with no alignment too (-1 is below 8).
        set_member(&mut e, member, 0x17, 0, 0, 0x080, 0, 0, 0);
        assert_eq!(alignment_of(&mut e, member, 0x17), 8);
    }

    #[test]
    fn struct_alignment_is_the_largest_member_alignment() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        let (class, table) = make_class(&mut e, 0, 0, 3);
        plain_member(&mut e, table, 4, 2);
        plain_member(&mut e, table + 0x18, 4, 8);
        plain_member(&mut e, table + 0x30, 4, 4);
        let member = e.mem.alloc(0x18);
        set_member(&mut e, member, 0x19, 0, 0, 0, 0, 0, class);
        assert_eq!(alignment_of(&mut e, member, 0x19), 8);
        // A class with no members aligns to 1.
        let (empty, _) = make_class(&mut e, 0, 0, 0);
        set_member(&mut e, member, 0x19, 0, 0, 0, 0, 0, empty);
        assert_eq!(alignment_of(&mut e, member, 0x19), 1);
    }

    fn retarget(e: &mut Engine, class: u32, rules: u32, map: u32, recurse: u32) {
        e.call(0x00dc_1960, &args![class, rules, map, recurse]);
    }

    fn offset_of(e: &Engine, member: u32) -> u16 {
        e.mem.u16(member + 0x12)
    }

    #[test]
    fn retarget_lays_out_a_class_chain_base_first() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        let frees = install_services(&mut e);
        let (base, base_table) = make_class(&mut e, 0, 0, 2);
        plain_member(&mut e, base_table, 4, 4);
        plain_member(&mut e, base_table + 0x18, 1, 1);
        let (derived, derived_table) = make_class(&mut e, base, 0, 1);
        plain_member(&mut e, derived_table, 8, 8);
        let rules = rules(&mut e, RULES_PLAIN);
        let map = map_record(&mut e);
        e.call_log = Some(vec![]);
        retarget(&mut e, derived, rules, map, 0);
        // Base: 0 and 4, size 5 rounded to 8. The next class starts at 8 (no padding reuse).
        assert_eq!(offset_of(&e, base_table), 0);
        assert_eq!(offset_of(&e, base_table + 0x18), 4);
        assert_eq!(e.mem.u32(base + 8), 8);
        assert_eq!(offset_of(&e, derived_table), 8);
        assert_eq!(e.mem.u32(derived + 8), 16);
        // Only the class passed in is entered in the map, with the heap allocator.
        assert_eq!(
            calls_to(&e, MAP_INSERT),
            vec![vec![map, HEAP_ALLOCATOR, derived, 0]]
        );
        // The local chain array held two classes and is freed through the temporary
        // allocator with its size in bytes.
        let frees = frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, 8);
    }

    #[test]
    fn retarget_reuses_padding_when_the_rules_say_so() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        install_services(&mut e);
        let (base, base_table) = make_class(&mut e, 0, 0, 2);
        plain_member(&mut e, base_table, 4, 4);
        plain_member(&mut e, base_table + 0x18, 1, 1);
        let (derived, derived_table) = make_class(&mut e, base, 0, 1);
        plain_member(&mut e, derived_table, 2, 2);
        let rules = rules(&mut e, [4, 1, 1, 0]);
        let map = map_record(&mut e);
        retarget(&mut e, derived, rules, map, 0);
        // The derived member continues at 5 (aligned to 2: 6), not at the rounded 8.
        assert_eq!(e.mem.u32(base + 8), 8);
        assert_eq!(offset_of(&e, derived_table), 6);
        assert_eq!(e.mem.u32(derived + 8), 8);
    }

    #[test]
    fn retarget_gives_each_interface_a_pointer_slot() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        install_services(&mut e);
        let (class, table) = make_class(&mut e, 0, 1, 1);
        plain_member(&mut e, table, 4, 4);
        let rules = rules(&mut e, RULES_PLAIN);
        let map = map_record(&mut e);
        retarget(&mut e, class, rules, map, 0);
        assert_eq!(offset_of(&e, table), 4);
        assert_eq!(e.mem.u32(class + 8), 8);
    }

    #[test]
    fn retarget_starts_a_class_after_an_empty_base_at_one_unless_optimized() {
        for (fourth_rule, expected) in [(0u8, 1u16), (1, 0)] {
            let mut e = Engine::new();
            install_class_doubles(&mut e);
            install_services(&mut e);
            let (base, _) = make_class(&mut e, 0, 0, 0);
            let (derived, table) = make_class(&mut e, base, 0, 1);
            plain_member(&mut e, table, 1, 1);
            let rules = rules(&mut e, [4, 1, 0, fourth_rule]);
            let map = map_record(&mut e);
            retarget(&mut e, derived, rules, map, 0);
            // An empty class still gets object size 1.
            assert_eq!(e.mem.u32(base + 8), 1);
            assert_eq!(offset_of(&e, table), expected);
        }
    }

    #[test]
    fn retarget_leaves_an_unchanged_offset_and_size_alone() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        install_services(&mut e);
        let (class, table) = make_class(&mut e, 0, 0, 1);
        plain_member(&mut e, table, 4, 4);
        e.mem.set_u16(table + 0x12, 0);
        e.mem.set_u32(class + 8, 4);
        let rules = rules(&mut e, RULES_PLAIN);
        let map = map_record(&mut e);
        e.call_log = Some(vec![]);
        retarget(&mut e, class, rules, map, 0);
        assert!(calls_to(&e, CLASS_SET_OBJECT_SIZE).is_empty());
        assert_eq!(offset_of(&e, table), 0);
    }

    #[test]
    fn retarget_recurses_into_struct_members_not_yet_in_the_map() {
        let mut e = Engine::new();
        install_class_doubles(&mut e);
        install_services(&mut e);
        let (inner, inner_table) = make_class(&mut e, 0, 0, 1);
        plain_member(&mut e, inner_table, 4, 4);
        let (outer, outer_table) = make_class(&mut e, 0, 0, 1);
        set_member(&mut e, outer_table, 0x19, 0, 0, 0, 0, 0, inner);
        let rules = rules(&mut e, RULES_PLAIN);
        let map = map_record(&mut e);
        e.call_log = Some(vec![]);
        retarget(&mut e, outer, rules, map, 1);
        let inserted: Vec<u32> = calls_to(&e, MAP_INSERT).iter().map(|c| c[2]).collect();
        assert_eq!(inserted, vec![outer, inner]);
        assert_eq!(e.mem.u32(inner + 8), 4);
        assert_eq!(offset_of(&e, outer_table), 0);
        assert_eq!(e.mem.u32(outer + 8), 4);
        // A second call finds the struct class in the map and does not enter it again.
        e.call_log = Some(vec![]);
        retarget(&mut e, outer, rules, map, 1);
        let inserted: Vec<u32> = calls_to(&e, MAP_INSERT).iter().map(|c| c[2]).collect();
        assert_eq!(inserted, vec![outer]);
        // Without the recurse flag the members are not looked at.
        let other_map = map_record(&mut e);
        e.call_log = Some(vec![]);
        retarget(&mut e, outer, rules, other_map, 0);
        assert!(calls_to(&e, MAP_FIND_KEY).is_empty());
    }
}
