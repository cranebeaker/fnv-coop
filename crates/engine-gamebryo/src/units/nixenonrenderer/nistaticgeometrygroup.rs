//! `NiXenonRenderer/nistaticgeometrygroup.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometryGroup::AddObject` (engine map; thiscall, one word;
/// `nigeometrygroup.cpp`).
const GEOMETRY_GROUP_ADD_OBJECT: u32 = 0x00e9_8730;
/// `NiGeometryGroup::RemoveObject` (engine map; thiscall, one word;
/// `nigeometrygroup.cpp`).
const GEOMETRY_GROUP_REMOVE_OBJECT: u32 = 0x00e9_8750;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiGeometryBufferData::NiGeometryBufferData` (engine map; thiscall, no
/// stack words), run on a 0x54-byte block.
const BUFFER_DATA_CONSTRUCTOR: u32 = 0x00e9_47c0;
/// The record `00e948f0` looks for in the geometry's chain (each record's
/// word +4 is the next; the shape of an `NiRTTI` base-class chain, not
/// confirmed).
const RECORD_011F4A88: u32 = 0x011f_4a88;
/// The record `00e948f0` passes to the check `0043b300`.
const RECORD_011F4A70: u32 = 0x011f_4a70;
/// The check `0043b300` (`fallout shared/modelloader.cpp`; cdecl: record,
/// object; answers in AL).
const CHECK_0043B300: u32 = 0x0043_b300;
/// The cap on the geometry's low flag bit (`011bf558`, a word).
const FLAG_CAP_011BF558: u32 = 0x011b_f558;

/// The removal both `RemoveObject` overloads share: takes the buffer data
/// in `geometry`'s word `slot` out of the group, deletes it (its virtual
/// slot 0 with 1) and clears the word.
fn remove_buffer_data(e: &mut Engine, this: Ptr, geometry: Ptr, slot: u32) {
    let data = e.mem.u32(geometry.addr() + slot);
    if data != 0 {
        e.call(GEOMETRY_GROUP_REMOVE_OBJECT, &[this.addr(), data]);
        e.vcall(data, 0, &[1]);
        e.mem.set_u32(geometry.addr() + slot, 0);
    }
}

// Translated from 00e94740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::RemoveObject` (Xbox PDB; thiscall, one word):
/// for the geometry's buffer data at +0x34, when set: removes it from the
/// group (`NiGeometryGroup::RemoveObject`), deletes it and clears +0x34.
pub fn ni_static_geometry_group_remove_object(e: &mut Engine, this: Ptr, geometry: Ptr) {
    remove_buffer_data(e, this, geometry, 0x34);
}

// Translated from 00e94d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::RemoveObject` (Xbox PDB, the engine map's
/// `_ov2`; thiscall, one word): the same for the buffer data at +0x28 of
/// its argument.
pub fn ni_static_geometry_group_remove_object_ov2(e: &mut Engine, this: Ptr, part: Ptr) {
    remove_buffer_data(e, this, part, 0x28);
}

/// A new buffer data (0x54 bytes, constructed), or 0 when the allocation
/// fails.
fn new_buffer_data(e: &mut Engine) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x54]).u32();
    if block != 0 {
        e.call(BUFFER_DATA_CONSTRUCTOR, &[block]).u32()
    } else {
        0
    }
}

// Translated from 00e948f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, three words; the second is never
/// read): gives `geometry` (or, when `part` is set, `part`) a new buffer
/// data and adds it to the group. Nothing happens when the target already
/// has one (`part` +0x28, `geometry` +0x34).
///
/// For `part`, the data's +0x3C is 5 when its 16-bit +0x22 is not 0,
/// else 4. For `geometry`, +0x3C becomes 4 when the record `011f4a88` is in
/// the chain the geometry's virtual slot +8 returns, else 5 when
/// `0043b300(011f4a70, geometry)` answers true (otherwise the constructor's
/// value stays). The data's +0x04 gets the geometry's flags: bit 22 when
/// its +0x28 is set, bit 23 when +0x24 is set, bit 21 when +0x24 is set and
/// the 16-bit +0x0C has a bit of 0xF000, and in bit 24 up its bit 0 capped
/// by the word at `011bf558`. Then `NiGeometryGroup::AddObject`. A failed
/// allocation leaves the data null and the exe writes through it anyway.
pub fn fn_00e948f0(e: &mut Engine, this: Ptr, geometry: Ptr, _unused_1: u32, part: Ptr) {
    let geometry = geometry.addr();
    let part = part.addr();
    let data;
    if part != 0 {
        if e.mem.u32(part + 0x28) != 0 {
            return;
        }
        data = new_buffer_data(e);
        let kind = u32::from(e.mem.u16(part + 0x22) != 0) + 4;
        e.mem.set_u32(data.wrapping_add(0x3c), kind);
        e.mem.set_u32(part + 0x28, data);
    } else {
        if e.mem.u32(geometry + 0x34) != 0 {
            return;
        }
        data = new_buffer_data(e);
        let mut record = e.vcall(geometry, 8, &[]).u32();
        let mut found = false;
        while record != 0 {
            if record == RECORD_011F4A88 {
                found = true;
                break;
            }
            record = e.mem.u32(record + 4);
        }
        if found {
            e.mem.set_u32(data.wrapping_add(0x3c), 4);
        } else if e.call(CHECK_0043B300, &[RECORD_011F4A70, geometry]).u8() != 0 {
            e.mem.set_u32(data.wrapping_add(0x3c), 5);
        }
        e.mem.set_u32(geometry + 0x34, data);
    }
    let has_28 = e.mem.u32(geometry + 0x28) != 0;
    let has_24 = e.mem.u32(geometry + 0x24) != 0;
    let high_flags = has_24 && e.mem.u16(geometry + 0xc) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(geometry + 0xc)) & 1;
    let cap = e.mem.u32(FLAG_CAP_011BF558);
    if low > cap {
        low = cap;
    }
    let mut bits = 0;
    if has_28 {
        bits = 0x40_0000;
    }
    if has_24 {
        bits |= 0x80_0000;
    }
    if high_flags {
        bits |= 0x20_0000;
    }
    e.mem.set_u32(data.wrapping_add(4), (low << 24) | bits);
    e.call(GEOMETRY_GROUP_ADD_OBJECT, &[this.addr(), data]);
}

/// The vtables of this unit's classes, named by their RTTI records: `NiStaticGeometryGroup`
/// (`010f0fd4`), `NiTPointerMap<unsigned int,NiVBSet*>` (`010f0fb4`), its base `NiTMapBase`
/// (`010f0f94`), `NiTArray<NiVBBlock*,NiTMallocInterface<NiVBBlock*>>` (`010f0f84`); and the
/// vtable `00e94b00` gives a set's block array (`010f1000`).
const STATIC_GROUP_VTABLE: u32 = 0x010f_0fd4;
const SET_MAP_VTABLE: u32 = 0x010f_0fb4;
const SET_MAP_BASE_VTABLE: u32 = 0x010f_0f94;
const BLOCK_ARRAY_VTABLE: u32 = 0x010f_0f84;
const SET_BLOCK_ARRAY_VTABLE: u32 = 0x010f_1000;
/// `NiStaticGeometryGroup::m_kSets` (Xbox PDB) +0x0C: an `NiTPointerMap` (vtable, hash size
/// +4, bucket table +8, count +0xC; each item: next +0, key +4, value +8).
const SETS: u32 = 0x0c;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00aa1070` (`cdecl`, the size) and `00aa10f0` (`cdecl`, the block): allocate and free a
/// block; `00aa13e0` (`cdecl`, the size) allocates an object; `00aa1460` (`cdecl`: block,
/// size) is the sized delete; `memset` (`00ec61c0`, `cdecl`).
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
const NI_NEW: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const MEMSET: u32 = 0x00ec_61c0;
/// The `NiGeometryGroup` constructor (`00e986f0`) and destructor body (`00e98710`) (both
/// `thiscall`, no stack words; unnamed in the engine map).
const GEOMETRY_GROUP_CONSTRUCT: u32 = 0x00e9_86f0;
const GEOMETRY_GROUP_DESTRUCT: u32 = 0x00e9_8710;
/// The map's removal of a key (`00e91fd0`, `thiscall`, one word; a folded body the engine map
/// puts in `nid3dshaderfactory.cpp`), its emptying (`00b601a0`, `thiscall`, no stack words),
/// its lookup (`00a53c30`, `thiscall`: key, address of the value; answers in `AL`) and its
/// insertion (`00b57ea0`, `thiscall`: key, value).
const MAP_REMOVE: u32 = 0x00e9_1fd0;
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
const MAP_GET_AT: u32 = 0x00a5_3c30;
const MAP_SET_AT: u32 = 0x00b5_7ea0;
/// The set's destructor body (`00e943c0`, `thiscall`), its search for a block with room
/// (`00e941d0`, `thiscall`, the size), its new block (`00e94580`, `thiscall`, the size), its
/// list insertion (`00e94220`, `thiscall`, the block) and its start-up hook (`00e941a0`,
/// `cdecl`, the set); the block's chip allocation (`00e98660`, `thiscall`, the size). All
/// unnamed in the engine map.
const SET_DESTRUCT: u32 = 0x00e9_43c0;
const SET_FIND_BLOCK: u32 = 0x00e9_41d0;
const SET_NEW_BLOCK: u32 = 0x00e9_4580;
const SET_INSERT_BLOCK: u32 = 0x00e9_4220;
const SET_REGISTER_00E941A0: u32 = 0x00e9_41a0;
const BLOCK_ALLOCATE_CHIP: u32 = 0x00e9_8660;
/// `NiVBSet::FreeChip` (`00e94490`, `nivbset.cpp`; `thiscall`: block index, chip index).
const SET_FREE_CHIP: u32 = 0x00e9_4490;
/// `NiVBSet::ms_uiDefBlockSize` and `ms_uiDefFreeSpaceThreshold` by the Xbox PDB's order of
/// the statics (`011c0ce0`, `011c0ce4`; read by the set constructor).
const DEFAULT_BLOCK_SIZE: u32 = 0x011c_0ce0;
const DEFAULT_FREE_SPACE_THRESHOLD: u32 = 0x011c_0ce4;

// Translated from 00e94650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiVBSet`, two words: the size and a byte
/// flag): unless the flag is set, looks for a block with room (`00e941d0`); without one (or
/// with the flag) makes a new block (`00e94580`), answering 0 when that fails. Takes a chip of
/// the size from the block (`00e98660`) and, when the block's free bytes (+0x24) are above
/// the set's `m_uiFreeSpaceThreshold` (+0x30), puts it back in the set's list (`00e94220`).
/// Answers the chip.
pub fn fn_00e94650(e: &mut Engine, this: Ptr, size: u32, new_block: u8) -> u32 {
    let set = this.addr();
    let mut block = 0;
    if new_block == 0 {
        block = e.call(SET_FIND_BLOCK, &[set, size]).u32();
    }
    if block == 0 {
        block = e.call(SET_NEW_BLOCK, &[set, size]).u32();
        if block == 0 {
            return 0;
        }
    }
    let chip = e.call(BLOCK_ALLOCATE_CHIP, &[block, size]).u32();
    if e.mem.u32(block + 0x24) > e.mem.u32(set + 0x30) {
        e.call(SET_INSERT_BLOCK, &[set, block]);
    }
    chip
}

/// The map's first item: the head of its first non-empty bucket, or 0.
fn first_item(e: &Engine, map: u32) -> u32 {
    let size = e.mem.u32(map + 4);
    let table = e.mem.u32(map + 8);
    let mut bucket = 0;
    while bucket < size {
        let item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        if item != 0 {
            return item;
        }
        bucket += 1;
    }
    0
}

// Translated from 00e946b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::Purge` (Xbox PDB, engine map), `thiscall`, no stack words: walks
/// the set map (+0x0C) item by item. For each item it first finds the next one (the next item
/// in the bucket, or the head of the next non-empty bucket after the item's own, which the
/// map's slot +4 computes from the key), then removes the key from the map (`00e91fd0`) and,
/// when the item held a set, destroys it (`00e943c0`) and frees it (0x34 bytes).
pub fn ni_static_geometry_group_purge(e: &mut Engine, this: Ptr) {
    let map = this.addr() + SETS;
    let mut item = first_item(e, map);
    while item != 0 {
        let key = e.mem.u32(item + 4);
        let set = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if item == 0 {
            let mut bucket = e.vcall(map, 4, &[key]).u32().wrapping_add(1);
            while bucket < e.mem.u32(map + 4) {
                let table = e.mem.u32(map + 8);
                item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
                if item != 0 {
                    break;
                }
                bucket += 1;
            }
        }
        e.call(MAP_REMOVE, &[map, key]);
        if set != 0 {
            e.call(SET_DESTRUCT, &[set]);
            e.call(NI_SIZED_DELETE, &[set, 0x34]);
        }
    }
}

// Translated from 00e94770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::ReleaseChip` (Xbox PDB, engine map), `thiscall`, two words (the
/// buffer data and the stream): for a stream below the data's count (+0x20) with a chip
/// (+0x28), a chip with a vertex buffer (+0x08) is given back to its block's set
/// (`NiVBSet::FreeChip` on the block's `m_pkSet` +0 with the block's index +4 and the chip's
/// index +0); then the chip slot is cleared (the stream checked against the count again).
pub fn ni_static_geometry_group_release_chip(e: &mut Engine, _this: Ptr, data: Ptr, stream: u32) {
    let data = data.addr();
    if stream >= e.mem.u32(data + 0x20) {
        return;
    }
    let chips = e.mem.u32(data + 0x28);
    let chip = e.mem.u32(chips.wrapping_add(stream.wrapping_mul(4)));
    if chip == 0 {
        return;
    }
    if e.mem.u32(chip + 8) != 0 {
        let block = e.mem.u32(chip + 4);
        let chip_index = e.mem.u32(chip);
        let block_index = e.mem.u32(block.wrapping_add(4));
        let set = e.mem.u32(block);
        e.call(SET_FREE_CHIP, &[set, block_index, chip_index]);
    }
    if stream < e.mem.u32(data + 0x20) {
        let chips = e.mem.u32(data + 0x28);
        e.mem.set_u32(chips.wrapping_add(stream.wrapping_mul(4)), 0);
    }
}

// Translated from 00e94890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall`
/// with one stack word (the flags): installs the class's vtable, purges the group, empties
/// the set map under its own vtable and again under its base's (`00b601a0`), frees the
/// bucket table (`00aa10f0`), runs the `NiGeometryGroup` destructor body (`00e98710`), and
/// when bit 0 of the flags is set frees the group (0x1c bytes, `00aa1460`). Answers `this`.
pub fn ni_static_geometry_group_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let group = this.addr();
    e.mem.set_u32(group, STATIC_GROUP_VTABLE);
    ni_static_geometry_group_purge(e, this);
    let map = group + SETS;
    e.mem.set_u32(map, SET_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &[map]);
    e.mem.set_u32(map, SET_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &[map]);
    let table = e.mem.u32(map + 8);
    e.call(FREE_BLOCK, &[table]);
    e.call(GEOMETRY_GROUP_DESTRUCT, &[group]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &[group, 0x1c]);
    }
    this
}

// Translated from 00e94a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::AddObject` (Xbox PDB, engine map), `thiscall`, one word: when the
/// object's +0x1C is 0, makes a buffer data (0x54 bytes from the memory manager,
/// constructed), sets its +0x3C to 4 and its +0x04 (`m_uiFlags`) to `0x01400000`, and adds it
/// to the group (`NiGeometryGroup::AddObject`). The exe does not store the data in the object,
/// and with a failed allocation writes through the null pointer.
pub fn ni_static_geometry_group_add_object(e: &mut Engine, this: Ptr, object: Ptr) {
    if e.mem.u32(object.addr() + 0x1c) != 0 {
        return;
    }
    let data = new_buffer_data(e);
    e.mem.set_u32(data.wrapping_add(0x3c), 4);
    e.mem.set_u32(data.wrapping_add(4), 0x0140_0000);
    e.call(GEOMETRY_GROUP_ADD_OBJECT, &[this.addr(), data]);
}

// Translated from 00e94a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<NiVBBlock*,NiTMallocInterface<NiVBBlock*>>` scalar deleting destructor (its
/// vtable `010f0f84`'s slot 0; unnamed in the engine map), `thiscall` with one stack word (the
/// flags): installs the vtable, frees the element block (+0x04, `00aa10f0`) and, when bit 0 of
/// the flags is set, frees the array through the memory manager. Answers `this`.
pub fn fn_00e94a80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let array = this.addr();
    let elements = e.mem.u32(array + 4);
    e.mem.set_u32(array, BLOCK_ARRAY_VTABLE);
    e.call(FREE_BLOCK, &[elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, array]);
    }
    this
}

// Translated from 00e94ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiStaticGeometryGroup` constructor (unnamed in the engine map; `thiscall`, no stack
/// words): the `NiGeometryGroup` constructor (`00e986f0`), the class's vtable, then the set
/// map at +0x0C: the base map vtable, 37 buckets, count 0, a bucket table of 37 words
/// (`00aa1070`, zeroed with `memset`), and the map's own vtable. Answers `this`.
pub fn fn_00e94ab0(e: &mut Engine, this: Ptr) -> Ptr {
    let group = this.addr();
    e.call(GEOMETRY_GROUP_CONSTRUCT, &[group]);
    e.mem.set_u32(group, STATIC_GROUP_VTABLE);
    let map = group + SETS;
    e.mem.set_u32(map, SET_MAP_BASE_VTABLE);
    e.mem.set_u32(map + 4, 0x25);
    e.mem.set_u32(map + 0xc, 0);
    let table = e.call(ALLOCATE_BLOCK, &[0x94]).u32();
    let bytes = e.mem.u32(map + 4).wrapping_mul(4);
    e.mem.set_u32(map + 8, table);
    e.call(MEMSET, &[table, 0, bytes]);
    e.mem.set_u32(map, SET_MAP_VTABLE);
    this
}

// Translated from 00e94b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiVBSet` constructor (unnamed in the engine map; `thiscall`, no stack words; Xbox PDB
/// layout): the block array at +0x1C (vtable `010f1000`, data +0x20 null, 16-bit sizes +0x24
/// and +0x26 and +0x28 zero, grow-by +0x2A one), `m_uiBlockSize` (+0x0C) from `011c0ce0`, the
/// other words zero, the array emptied (each of its +0x26 used slots cleared, none here), the
/// block list (+0x2C) null and `m_uiFreeSpaceThreshold` (+0x30) from `011c0ce4`. Answers
/// `this`.
pub fn fn_00e94b00(e: &mut Engine, this: Ptr) -> Ptr {
    let set = this.addr();
    e.mem.set_u32(set + 0x1c, SET_BLOCK_ARRAY_VTABLE);
    e.mem.set_u16(set + 0x24, 0);
    e.mem.set_u16(set + 0x26, 0);
    e.mem.set_u16(set + 0x2a, 1);
    e.mem.set_u16(set + 0x28, 0);
    e.mem.set_u32(set + 0x20, 0);
    let block_size = e.global::<u32>(DEFAULT_BLOCK_SIZE);
    e.mem.set_u32(set + 0xc, block_size);
    for offset in [4, 8, 0, 0x10, 0x14, 0x18] {
        e.mem.set_u32(set + offset, 0);
    }
    // The inlined array emptying: every used slot cleared, then the sizes.
    if e.mem.u16(set + 0x26) != 0 {
        let mut slot: u16 = 0;
        loop {
            let elements = e.mem.u32(set + 0x20);
            e.mem.set_u32(elements.wrapping_add(u32::from(slot) * 4), 0);
            slot = slot.wrapping_add(1);
            if slot >= e.mem.u16(set + 0x26) {
                break;
            }
        }
    }
    e.mem.set_u16(set + 0x26, 0);
    e.mem.set_u16(set + 0x28, 0);
    e.mem.set_u32(set + 0x2c, 0);
    let threshold = e.global::<u32>(DEFAULT_FREE_SPACE_THRESHOLD);
    e.mem.set_u32(set + 0x30, threshold);
    this
}

// Translated from 00e94b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words: block size, device, FVF, flags, index):
/// allocates (0x34 bytes, `00aa13e0`) and constructs an `NiVBSet` (0 when the allocation
/// fails), sets its block size (+0x0C), keeps the device (+0x10, referenced through its slot
/// +4; a null device releases a held one and clears the field), its FVF (+0x04), flags (+0x08)
/// and index (+0x00), and hands it to `00e941a0`. Answers the set.
pub fn fn_00e94b80(
    e: &mut Engine,
    block_size: u32,
    device: u32,
    fvf: u32,
    flags: u32,
    index: u32,
) -> u32 {
    let block = e.call(NI_NEW, &[0x34]).u32();
    if block == 0 {
        return 0;
    }
    let set = fn_00e94b00(e, Ptr::new(block)).addr();
    if set == 0 {
        return 0;
    }
    e.mem.set_u32(set + 0xc, block_size);
    if device != 0 {
        e.mem.set_u32(set + 0x10, device);
        e.vcall(device, 4, &[]);
    } else {
        let held = e.mem.u32(set + 0x10);
        if held != 0 {
            e.vcall(held, 8, &[]);
        }
        e.mem.set_u32(set + 0x10, 0);
    }
    e.mem.set_u32(set + 4, fvf);
    e.mem.set_u32(set + 8, flags);
    e.mem.set_u32(set, index);
    e.call(SET_REGISTER_00E941A0, &[set]);
    set
}

// Translated from 00e94c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the group makers
/// `NiD3DGeometryGroupManager` picks from): allocates 0x1c bytes (`00aa13e0`) and constructs
/// an `NiStaticGeometryGroup` in them (`00e94ab0`, a tail jump); 0 when the allocation fails.
pub fn fn_00e94c00(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &[0x1c]).u32();
    if block == 0 {
        return 0;
    }
    fn_00e94ab0(e, Ptr::new(block)).addr()
}

// Translated from 00e94c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::CreateChip` (Xbox PDB, engine map), `thiscall`, two words (the
/// buffer data and the stream). The stream's stride (`m_puiVertexStride`, 0 for a stream past
/// the count) is kept in the data argument's slot. The set key is the data's FVF (+0x0C), or
/// the stride with bit 31 when the FVF is 0, with bit 30 for software processing (+0x14),
/// which also makes the set flags `0x80000000` (else 0). A key the map does not hold, or holds
/// with a null set, gets a new set (`00e94b80`: 0x10000, the group's device +0x08, the key,
/// the flags, 0) stored under it. The set then gives a chip of `m_uiMaxVertCount` (+0x1C)
/// times the stride (`00e94650`, a new block forced for more than one stream), and the data's
/// `m_uiBaseVertexIndex` (+0x38) becomes the chip's offset (+0x0C) divided by the stride
/// (`DIV`: a zero stride faults as in the exe). Answers the chip.
pub fn ni_static_geometry_group_create_chip(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    stream: u32,
) -> u32 {
    let group = this.addr();
    let data = data.addr();
    let mut key = e.mem.u32(data + 0xc);
    let stride = if stream < e.mem.u32(data + 0x20) {
        let strides = e.mem.u32(data + 0x24);
        e.mem.u32(strides.wrapping_add(stream.wrapping_mul(4)))
    } else {
        0
    };
    if key == 0 {
        key = stride | 0x8000_0000;
    }
    let mut set_flags = 0;
    if e.mem.u8(data + 0x14) != 0 {
        key |= 0x4000_0000;
        set_flags = 0x8000_0000;
    }
    let map = group + SETS;
    let set = e.with_stack(4, |e, found| {
        let found = found.addr();
        e.mem.set_u32(found, 0);
        let held = e.call(MAP_GET_AT, &[map, key, found]).bool();
        let set = e.mem.u32(found);
        if held && set != 0 {
            return set;
        }
        let device = e.mem.u32(group + 8);
        let set = fn_00e94b80(e, 0x10000, device, key, set_flags, 0);
        e.call(MAP_SET_AT, &[map, key, set]);
        set
    });
    let streams = e.mem.u32(data + 0x20);
    let size = e.mem.u32(data + 0x1c).wrapping_mul(stride);
    let chip = fn_00e94650(e, Ptr::new(set), size, u8::from(streams > 1));
    let offset = e.mem.u32(chip.wrapping_add(0xc));
    e.mem.set_u32(data + 0x38, offset / stride);
    chip
}

// Translated from 00e94db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::RemoveObject` (Xbox PDB, the engine map's `_ov3`; `thiscall`, one
/// word): the same as the other two for the buffer data at +0x1C of its argument.
pub fn ni_static_geometry_group_remove_object_ov3(e: &mut Engine, this: Ptr, object: Ptr) {
    remove_buffer_data(e, this, object, 0x1c);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_4650, fn_00e94650(Ptr, u32, u8) -> u32),
        entry!(0x00e9_46b0, ni_static_geometry_group_purge(Ptr)),
        entry!(
            0x00e9_4770,
            ni_static_geometry_group_release_chip(Ptr, Ptr, u32)
        ),
        entry!(
            0x00e9_4890,
            ni_static_geometry_group_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e9_4a30, ni_static_geometry_group_add_object(Ptr, Ptr)),
        entry!(0x00e9_4a80, fn_00e94a80(Ptr, u32) -> Ptr),
        entry!(0x00e9_4ab0, fn_00e94ab0(Ptr) -> Ptr),
        entry!(0x00e9_4b00, fn_00e94b00(Ptr) -> Ptr),
        entry!(0x00e9_4b80, fn_00e94b80(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e9_4c00, fn_00e94c00() -> u32),
        entry!(
            0x00e9_4c20,
            ni_static_geometry_group_create_chip(Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_4db0,
            ni_static_geometry_group_remove_object_ov3(Ptr, Ptr)
        ),
        entry!(
            0x00e9_4740,
            ni_static_geometry_group_remove_object(Ptr, Ptr)
        ),
        entry!(
            0x00e9_4d00,
            ni_static_geometry_group_remove_object_ov2(Ptr, Ptr)
        ),
        entry!(0x00e9_48f0, fn_00e948f0(Ptr, Ptr, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0066_0000;
    const GET_CHAIN: u32 = 0x0066_0008;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.register(GEOMETRY_GROUP_ADD_OBJECT, |_, _| Ret::default());
        e.register(GEOMETRY_GROUP_REMOVE_OBJECT, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(BUFFER_DATA_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x3c, 0x77);
            a[0].into_ret()
        });
        e.register(CHECK_0043B300, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn buffer_data(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, DELETE);
        let data = e.mem.alloc(0x54);
        e.mem.set_u32(data, vtable);
        data
    }

    #[test]
    fn remove_object_deletes_the_data_at_34() {
        let mut e = engine();
        let data = buffer_data(&mut e);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0x34, data);
        e.call(0x00e9_4740, &[0x9000, geometry]);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_REMOVE_OBJECT),
            vec![vec![0x9000, data]]
        );
        assert_eq!(calls_to(&e, DELETE), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(geometry + 0x34), 0);
        e.call(0x00e9_4740, &[0x9000, geometry]);
        assert_eq!(calls_to(&e, DELETE).len(), 1);
    }

    #[test]
    fn remove_object_ov2_deletes_the_data_at_28() {
        let mut e = engine();
        let data = buffer_data(&mut e);
        let part = e.mem.alloc(0x40);
        e.mem.set_u32(part + 0x28, data);
        e.mem.set_u32(part + 0x34, 0x1234);
        e.call(0x00e9_4d00, &[0x9000, part]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(part + 0x28), 0);
        assert_eq!(e.mem.u32(part + 0x34), 0x1234);
    }

    fn geometry(e: &mut Engine, chain: u32) -> u32 {
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, GET_CHAIN);
        e.register_double(GET_CHAIN, move |_, _| chain.into_ret());
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u32(geometry, vtable);
        geometry
    }

    #[test]
    fn add_finds_the_record_and_sets_the_flags() {
        let mut e = engine();
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, RECORD_011F4A88);
        let geometry = geometry(&mut e, other);
        e.mem.set_u32(geometry + 0x24, 1);
        e.mem.set_u32(geometry + 0x28, 1);
        e.mem.set_u16(geometry + 0xc, 0x1001);
        e.mem.set_u32(FLAG_CAP_011BF558, 1);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0xdead, 0]);
        let data = e.mem.u32(geometry + 0x34);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
        assert_eq!(e.mem.u32(data + 4), 0x01e0_0000);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_ADD_OBJECT),
            vec![vec![0x9000, data]]
        );
        assert!(calls_to(&e, CHECK_0043B300).is_empty());
        // Already has data: nothing more.
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, 0]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_ADD_OBJECT).len(), 1);
    }

    #[test]
    fn add_without_the_record_asks_the_check_and_caps_the_bit() {
        let mut e = engine();
        e.register(CHECK_0043B300, |_, _| 0x0101u32.into_ret());
        let geometry = geometry(&mut e, 0);
        e.mem.set_u16(geometry + 0xc, 0xf001);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, 0]);
        let data = e.mem.u32(geometry + 0x34);
        assert_eq!(
            calls_to(&e, CHECK_0043B300),
            vec![vec![RECORD_011F4A70, geometry]]
        );
        assert_eq!(e.mem.u32(data + 0x3c), 5);
        // +0x24 clear: no flag bits; the cap (0) clears bit 0.
        assert_eq!(e.mem.u32(data + 4), 0);
    }

    #[test]
    fn add_for_a_part_uses_its_word_22() {
        let mut e = engine();
        let geometry = geometry(&mut e, 0);
        e.mem.set_u32(geometry + 0x28, 1);
        let part = e.mem.alloc(0x2c);
        e.mem.set_u16(part + 0x22, 3);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, part]);
        let data = e.mem.u32(part + 0x28);
        assert_eq!(e.mem.u32(data + 0x3c), 5);
        assert_eq!(e.mem.u32(data + 4), 0x40_0000);
        assert_eq!(e.mem.u32(geometry + 0x34), 0);
        assert!(calls_to(&e, GET_CHAIN).is_empty());
        let part = e.mem.alloc(0x2c);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, part]);
        let data = e.mem.u32(part + 0x28);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
    }

    /// Registers no-op doubles for `addresses`.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn set_chip_uses_a_found_or_a_new_block() {
        let mut e = engine();
        let set = e.mem.alloc(0x34);
        e.mem.set_u32(set + 0x30, 0x100);
        let found = e.mem.alloc(0x44);
        let fresh = e.mem.alloc(0x44);
        e.mem.set_u32(found + 0x24, 0x200);
        e.register_double(SET_FIND_BLOCK, move |_, _| found.into_ret());
        e.register_double(SET_NEW_BLOCK, move |_, _| fresh.into_ret());
        e.register(BLOCK_ALLOCATE_CHIP, |_, a| (a[0] + 1).into_ret());
        quiet(&mut e, &[SET_INSERT_BLOCK]);
        // Found, with more free bytes than the threshold: back into the list.
        assert_eq!(e.call(0x00e9_4650, &[set, 0x40, 0]).u32(), found + 1);
        assert_eq!(calls_to(&e, BLOCK_ALLOCATE_CHIP), vec![vec![found, 0x40]]);
        assert_eq!(calls_to(&e, SET_INSERT_BLOCK), vec![vec![set, found]]);
        assert!(calls_to(&e, SET_NEW_BLOCK).is_empty());
        // The flag forces a new block; at the threshold it stays out of the list.
        e.mem.set_u32(fresh + 0x24, 0x100);
        assert_eq!(e.call(0x00e9_4650, &[set, 0x40, 1]).u32(), fresh + 1);
        assert_eq!(calls_to(&e, SET_FIND_BLOCK).len(), 1);
        assert_eq!(calls_to(&e, SET_NEW_BLOCK), vec![vec![set, 0x40]]);
        assert_eq!(calls_to(&e, SET_INSERT_BLOCK).len(), 1);
        // Nothing found and no new block: 0.
        e.register(SET_FIND_BLOCK, |_, _| Ret::default());
        e.register(SET_NEW_BLOCK, |_, _| Ret::default());
        assert_eq!(e.call(0x00e9_4650, &[set, 0x40, 0]).u32(), 0);
        assert_eq!(calls_to(&e, BLOCK_ALLOCATE_CHIP).len(), 2);
    }

    /// A group whose set map has 3 buckets: bucket 0 holds keys 10 (set `sets[0]`) then 11 (no
    /// set), bucket 2 key 12 (set `sets[1]`). The map's slot +4 hashes 10 and 11 to 0, 12 to 2.
    fn group_with_sets(e: &mut Engine) -> (u32, [u32; 2]) {
        let group = e.mem.alloc(0x1c);
        let map = group + SETS;
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0066_0104);
        e.register(0x0066_0104, |_, a| {
            u32::from(a[1] == 12).wrapping_mul(2).into_ret()
        });
        e.mem.set_u32(map, vtable);
        let table = e.mem.alloc(12);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, table);
        let sets = [e.mem.alloc(0x34), e.mem.alloc(0x34)];
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(12)).collect();
        for (item, (key, set)) in items.iter().zip([(10, sets[0]), (11, 0), (12, sets[1])]) {
            e.mem.set_u32(item + 4, key);
            e.mem.set_u32(item + 8, set);
        }
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(table, items[0]);
        e.mem.set_u32(table + 8, items[2]);
        (group, sets)
    }

    #[test]
    fn purge_removes_every_key_and_frees_the_sets() {
        let mut e = engine();
        quiet(&mut e, &[MAP_REMOVE, SET_DESTRUCT, NI_SIZED_DELETE]);
        let (group, sets) = group_with_sets(&mut e);
        e.call(0x00e9_46b0, &[group]);
        let map = group + SETS;
        assert_eq!(
            calls_to(&e, MAP_REMOVE),
            vec![vec![map, 10], vec![map, 11], vec![map, 12]]
        );
        assert_eq!(
            calls_to(&e, SET_DESTRUCT),
            vec![vec![sets[0]], vec![sets[1]]]
        );
        assert_eq!(
            calls_to(&e, NI_SIZED_DELETE),
            vec![vec![sets[0], 0x34], vec![sets[1], 0x34]]
        );
        // The hash is asked only where a bucket ends.
        assert_eq!(
            calls_to(&e, 0x0066_0104),
            vec![vec![map, 11], vec![map, 12]]
        );
    }

    #[test]
    fn purge_of_an_empty_map_does_nothing() {
        let mut e = engine();
        let group = e.mem.alloc(0x1c);
        e.call(0x00e9_46b0, &[group]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn release_chip_frees_and_clears_the_stream() {
        let mut e = engine();
        quiet(&mut e, &[SET_FREE_CHIP]);
        let block = e.mem.alloc(0x44);
        e.mem.set_u32(block, 0x5100);
        e.mem.set_u32(block + 4, 3);
        let chip = e.mem.alloc(0x20);
        e.mem.set_u32(chip, 7);
        e.mem.set_u32(chip + 4, block);
        e.mem.set_u32(chip + 8, 0x5200);
        let bare = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x54);
        let chips = e.mem.alloc(8);
        e.mem.set_u32(chips, chip);
        e.mem.set_u32(chips + 4, bare);
        e.mem.set_u32(data + 0x20, 2);
        e.mem.set_u32(data + 0x28, chips);
        e.call(0x00e9_4770, &[0x9000, data, 0]);
        assert_eq!(calls_to(&e, SET_FREE_CHIP), vec![vec![0x5100, 3, 7]]);
        assert_eq!(e.mem.u32(chips), 0);
        // No vertex buffer: only cleared; past the count: nothing.
        e.call(0x00e9_4770, &[0x9000, data, 1]);
        assert_eq!(calls_to(&e, SET_FREE_CHIP).len(), 1);
        assert_eq!(e.mem.u32(chips + 4), 0);
        e.call(0x00e9_4770, &[0x9000, data, 2]);
        assert_eq!(calls_to(&e, SET_FREE_CHIP).len(), 1);
    }

    #[test]
    fn deleting_destructor_purges_and_empties_the_map() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MAP_REMOVE_ALL,
                FREE_BLOCK,
                GEOMETRY_GROUP_DESTRUCT,
                NI_SIZED_DELETE,
            ],
        );
        let group = e.mem.alloc(0x1c);
        e.mem.set_u32(group + SETS + 8, 0x7700);
        assert_eq!(e.call(0x00e9_4890, &[group, 1]).u32(), group);
        assert_eq!(e.mem.u32(group), STATIC_GROUP_VTABLE);
        assert_eq!(e.mem.u32(group + SETS), SET_MAP_BASE_VTABLE);
        let map = group + SETS;
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map], vec![map]]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![0x7700]]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_DESTRUCT), vec![vec![group]]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![group, 0x1c]]);
        e.call(0x00e9_4890, &[group, 0]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
    }

    #[test]
    fn add_object_makes_a_data_for_an_empty_object() {
        let mut e = engine();
        let object = e.mem.alloc(0x20);
        e.call(0x00e9_4a30, &[0x9000, object]);
        let add = &calls_to(&e, GEOMETRY_GROUP_ADD_OBJECT)[0];
        let data = add[1];
        assert_eq!(add[0], 0x9000);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
        assert_eq!(e.mem.u32(data + 4), 0x0140_0000);
        assert_eq!(e.mem.u32(object + 0x1c), 0);
        e.mem.set_u32(object + 0x1c, data);
        e.call(0x00e9_4a30, &[0x9000, object]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_ADD_OBJECT).len(), 1);
    }

    #[test]
    fn block_array_deleting_destructor_frees_its_elements() {
        let mut e = engine();
        quiet(&mut e, &[FREE_BLOCK, MEMORY_FREE]);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x6600);
        assert_eq!(e.call(0x00e9_4a80, &[array, 1]).u32(), array);
        assert_eq!(e.mem.u32(array), BLOCK_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![0x6600]]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, array]]);
        e.call(0x00e9_4a80, &[array, 0]);
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 1);
    }

    /// Doubles for the group constructor's callees: the base constructor, the allocator and a
    /// real `memset`.
    fn constructor_engine() -> Engine {
        let mut e = engine();
        quiet(&mut e, &[GEOMETRY_GROUP_CONSTRUCT]);
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            let fill = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &fill);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn constructor_builds_the_set_map() {
        let mut e = constructor_engine();
        let group = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00e9_4ab0, &[group]).u32(), group);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_CONSTRUCT), vec![vec![group]]);
        assert_eq!(e.mem.u32(group), STATIC_GROUP_VTABLE);
        assert_eq!(e.mem.u32(group + 0xc), SET_MAP_VTABLE);
        assert_eq!(e.mem.u32(group + 0x10), 37);
        assert_eq!(e.mem.u32(group + 0x18), 0);
        let table = e.mem.u32(group + 0x14);
        assert_eq!(calls_to(&e, ALLOCATE_BLOCK), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 0x94]]);
    }

    #[test]
    fn maker_allocates_and_constructs_a_group() {
        let mut e = constructor_engine();
        let group = e.call(0x00e9_4c00, &[]).u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x1c]]);
        assert_eq!(e.mem.u32(group), STATIC_GROUP_VTABLE);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00e9_4c00, &[]).u32(), 0);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_CONSTRUCT).len(), 1);
    }

    #[test]
    fn set_constructor_clears_the_set() {
        let mut e = engine();
        e.map(0x011c_0000, 0x1000);
        e.set_global(DEFAULT_BLOCK_SIZE, 0x8000u32);
        e.set_global(DEFAULT_FREE_SPACE_THRESHOLD, 0x40u32);
        let set = e.mem.alloc(0x34);
        for offset in (0..0x34).step_by(4) {
            e.mem.set_u32(set + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00e9_4b00, &[set]).u32(), set);
        assert_eq!(e.mem.u32(set + 0x1c), SET_BLOCK_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(set + 0xc), 0x8000);
        assert_eq!(e.mem.u32(set + 0x30), 0x40);
        assert_eq!(e.mem.u16(set + 0x2a), 1);
        for offset in [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x2c] {
            let value = if offset == 0x28 {
                u32::from(e.mem.u16(set + 0x28))
            } else {
                e.mem.u32(set + offset)
            };
            assert_eq!(value, 0, "{offset:#x}");
        }
    }

    #[test]
    fn set_maker_keeps_the_device_and_the_words() {
        let mut e = constructor_engine();
        e.map(0x011c_0000, 0x1000);
        quiet(&mut e, &[SET_REGISTER_00E941A0, 0x0066_0204]);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0066_0204);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let set = e
            .call(0x00e9_4b80, &[0x10000, device, 0x142, 0x8000_0000, 5])
            .u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(e.mem.u32(set + 0xc), 0x10000);
        assert_eq!(e.mem.u32(set + 0x10), device);
        assert_eq!(calls_to(&e, 0x0066_0204), vec![vec![device]]);
        assert_eq!(
            [e.mem.u32(set + 4), e.mem.u32(set + 8), e.mem.u32(set)],
            [0x142, 0x8000_0000, 5]
        );
        assert_eq!(calls_to(&e, SET_REGISTER_00E941A0), vec![vec![set]]);
        // Without a device the field stays clear; without memory the answer is 0.
        let set = e.call(0x00e9_4b80, &[0x10000, 0, 0x142, 0, 5]).u32();
        assert_eq!(e.mem.u32(set + 0x10), 0);
        assert_eq!(calls_to(&e, 0x0066_0204).len(), 1);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00e9_4b80, &[0x10000, device, 0, 0, 0]).u32(), 0);
    }

    /// A group with a device, and a buffer data with two streams (strides 0x20 and 0x10), 0x40
    /// vertices and FVF `fvf`.
    fn chip_setup(e: &mut Engine, fvf: u32) -> (u32, u32) {
        let device_vtable = e.mem.alloc(8);
        e.mem.set_u32(device_vtable + 4, 0x0066_0204);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, device_vtable);
        let group = e.mem.alloc(0x1c);
        e.mem.set_u32(group + 8, device);
        let data = e.mem.alloc(0x54);
        let strides = e.mem.alloc(8);
        e.mem.set_u32(strides, 0x20);
        e.mem.set_u32(strides + 4, 0x10);
        e.mem.set_u32(data + 0xc, fvf);
        e.mem.set_u32(data + 0x1c, 0x40);
        e.mem.set_u32(data + 0x20, 2);
        e.mem.set_u32(data + 0x24, strides);
        // The set's chip comes from a found block whose chip sits at offset 0x300.
        let block = e.mem.alloc(0x44);
        let chip = e.mem.alloc(0x20);
        e.mem.set_u32(chip + 0xc, 0x300);
        e.register_double(SET_FIND_BLOCK, move |_, _| block.into_ret());
        e.register_double(SET_NEW_BLOCK, move |_, _| block.into_ret());
        e.register_double(BLOCK_ALLOCATE_CHIP, move |_, _| chip.into_ret());
        quiet(e, &[MAP_SET_AT, SET_REGISTER_00E941A0, 0x0066_0204]);
        (group, data)
    }

    #[test]
    fn create_chip_uses_the_held_set() {
        let mut e = engine();
        let (group, data) = chip_setup(&mut e, 0x142);
        let set = e.mem.alloc(0x34);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], set);
            1u32.into_ret()
        });
        let chip = e.call(0x00e9_4c20, &[group, data, 1]).u32();
        let get = &calls_to(&e, MAP_GET_AT)[0];
        assert_eq!(get[..2], [group + SETS, 0x142]);
        assert!(calls_to(&e, MAP_SET_AT).is_empty());
        // 0x40 vertices of stride 0x10, a new block forced (two streams).
        assert_eq!(calls_to(&e, SET_NEW_BLOCK)[0], vec![set, 0x400]);
        assert_eq!(e.mem.u32(data + 0x38), 0x300 / 0x10);
        assert_eq!(e.mem.u32(chip + 0xc), 0x300);
    }

    #[test]
    fn create_chip_makes_a_set_for_a_new_key() {
        let mut e = constructor_engine();
        e.map(0x011c_0000, 0x1000);
        let (group, data) = chip_setup(&mut e, 0);
        e.mem.set_u8(data + 0x14, 1);
        e.mem.set_u32(data + 0x20, 1);
        e.register(MAP_GET_AT, |_, _| Ret::default());
        e.call(0x00e9_4c20, &[group, data, 0]);
        let key = 0x20 | 0x8000_0000 | 0x4000_0000;
        let set_at = &calls_to(&e, MAP_SET_AT)[0];
        assert_eq!(set_at[..2], [group + SETS, key]);
        let set = set_at[2];
        assert_eq!(e.mem.u32(set + 4), key);
        assert_eq!(e.mem.u32(set + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(set + 0xc), 0x10000);
        assert_eq!(e.mem.u32(set + 0x10), e.mem.u32(group + 8));
        // One stream: the set looks for a block first.
        assert_eq!(calls_to(&e, SET_FIND_BLOCK), vec![vec![set, 0x800]]);
        assert_eq!(e.mem.u32(data + 0x38), 0x300 / 0x20);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn create_chip_past_the_streams_divides_by_a_zero_stride() {
        let mut e = engine();
        let (group, data) = chip_setup(&mut e, 0x142);
        let set = e.mem.alloc(0x34);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], set);
            1u32.into_ret()
        });
        e.call(0x00e9_4c20, &[group, data, 2]);
    }

    #[test]
    fn remove_object_ov3_deletes_the_data_at_1c() {
        let mut e = engine();
        let data = buffer_data(&mut e);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x1c, data);
        e.call(0x00e9_4db0, &[0x9000, object]);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_REMOVE_OBJECT),
            vec![vec![0x9000, data]]
        );
        assert_eq!(calls_to(&e, DELETE), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(object + 0x1c), 0);
        e.call(0x00e9_4db0, &[0x9000, object]);
        assert_eq!(calls_to(&e, DELETE).len(), 1);
    }
}
