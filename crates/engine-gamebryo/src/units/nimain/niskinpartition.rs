//! `NiMain/niskinpartition.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSkinPartition::ms_RTTI` (the class's `NiRTTI` record).
const NI_SKIN_PARTITION_RTTI: u32 = 0x011f_5b1c;

// Translated from 00a820d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_skin_partition_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SKIN_PARTITION_RTTI)
}

// Translated from 00a82c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a824d0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_skin_partition_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a8_24d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiAlloc` (`00aa1070`, cdecl, the size).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiObject`'s constructor and destructor body (`00a5d3a0`, `00a5d3d0`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`) and `NiObject::RegisterStreamables`
/// (`00a5d2f0`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// The base `SaveBinary` this unit calls (`00a89af0`, a bare `RET 4`).
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The renderer singleton pointer (`011f4748`); its virtual `+0x100` is
/// called with the partition before it goes.
const RENDERER: u32 = 0x011f_4748;
/// `MemoryManager::Allocate` / `Deallocate` (`00aa3e40`, `00aa4060`) on the
/// singleton at `011f6238`.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `_vector_constructor_iterator` (`00401050`) and the `Partition`
/// constructor it runs (`00a81ba0`).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
const PARTITION_CONSTRUCT: u32 = 0x00a8_1ba0;
/// The `NiSkinPartition` vtable (its RTTI locator names
/// `.?AVNiSkinPartition@@`).
const NI_SKIN_PARTITION_VTABLE: u32 = 0x0109_e47c;
/// `NiSkinPartition`: `m_uiPartitions` +8, `m_pkPartitions` +0xC (Xbox
/// PDB); a `Partition` is 0x2C bytes.
const PARTITIONS: u32 = 0x08;
const PARTITION_ARRAY: u32 = 0x0c;
const PARTITION_SIZE: u32 = 0x2c;
/// `NiSkinPartition::Partition` (Xbox PDB field names).
const PART_BONES: u32 = 0x04;
const PART_WEIGHTS: u32 = 0x08;
const PART_VERTEX_MAP: u32 = 0x0c;
const PART_BONE_PALETTE: u32 = 0x10;
const PART_TRI_LIST: u32 = 0x14;
const PART_STRIP_LENGTHS: u32 = 0x18;
const PART_VERTICES: u32 = 0x1c;
const PART_TRIANGLES: u32 = 0x1e;
const PART_BONE_COUNT: u32 = 0x20;
const PART_STRIPS: u32 = 0x22;
const PART_BONES_PER_VERTEX: u32 = 0x24;
/// The stream's version (+0xD8) and the first one with the has-array bytes
/// (`10.0.1.18`).
const STREAM_VERSION: u32 = 0xd8;
const VERSION_10_0_1_18: u32 = 0x0a00_0112;
/// `NiStream`'s input (+0x24C, read function +8) and output (+0x250,
/// write function +0xC); both move their position (+4) by what the
/// function answers.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;

/// Reads `bytes` bytes of components of `size` bytes into `buffer`.
fn read(e: &mut Engine, stream: Ptr, buffer: u32, bytes: u32, size: u32) {
    transfer(e, stream, STREAM_INPUT, 8, buffer, bytes, size);
}

/// Writes `bytes` bytes of components of `size` bytes from `buffer`.
fn write(e: &mut Engine, stream: Ptr, buffer: u32, bytes: u32, size: u32) {
    transfer(e, stream, STREAM_OUTPUT, 0xc, buffer, bytes, size);
}

fn transfer(e: &mut Engine, stream: Ptr, side: u32, slot: u32, buffer: u32, bytes: u32, size: u32) {
    let file = e.mem.u32(stream.addr() + side);
    let function = e.mem.u32(file + slot);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        e.call(function, &args![file, buffer, bytes, sizes, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(count));
}

fn u16_at(e: &Engine, address: u32) -> u32 {
    u32::from(e.mem.u16(address))
}

// Translated from 00a81fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::CreateObject` (the class's stream factory; Ghidra made
/// no function of it): a new 0x10-byte object with the `NiObject`
/// constructor, the class's vtable and no partitions array (the count at
/// +8 is left as it is); null when the allocation fails.
pub fn ni_skin_partition_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_SKIN_PARTITION_VTABLE);
    e.mem.set_u32(object + PARTITION_ARRAY, 0);
    Ptr::new(object)
}

// Translated from 00a81fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `NiObject::RegisterStreamables` (`00a5d2f0`) with the same
/// `this` and stream; answers what it answers.
pub fn fn_00a81fe0(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00a824d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinPartition` destructor body: the vtable; the renderer, when
/// there is one, lets go of the partition (its virtual `+0x100`); the
/// partitions array goes through the first item's vector deleting
/// destructor (virtual `+0`, flag 3) when its count word is not 0, else
/// its block (count word included) back to the `MemoryManager`; then the
/// `NiObject` part.
pub fn fn_00a824d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_SKIN_PARTITION_VTABLE);
    let renderer = e.global::<u32>(RENDERER);
    if renderer != 0 {
        e.vcall(renderer, 0x100, &args![this]);
    }
    let parts = e.mem.u32(base + PARTITION_ARRAY);
    if parts != 0 {
        let block = parts.wrapping_sub(4);
        if e.mem.u32(block) != 0 {
            e.vcall(parts, 0, &args![3u32]);
        } else {
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
        }
    }
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a81c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sum of the partition's strip lengths (`m_pusStripLengths`, one
/// `u16` per strip; the exe adds them two by two, then an odd last one).
pub fn fn_00a81c20(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let strips = u16_at(e, base + PART_STRIPS);
    let mut odd = 0u32;
    let mut first = 0u32;
    let mut second = 0u32;
    let mut index = 0u32;
    if strips >= 2 {
        let lengths = e.mem.u32(base + PART_STRIP_LENGTHS);
        let pairs = ((strips - 2) >> 1) + 1;
        for pair in 0..pairs {
            first = first.wrapping_add(u16_at(e, lengths + 4 * pair));
            second = second.wrapping_add(u16_at(e, lengths + 4 * pair + 2));
        }
        index = pairs * 2;
    }
    if index < strips {
        let lengths = e.mem.u32(base + PART_STRIP_LENGTHS);
        odd = u16_at(e, lengths + 2 * index);
    }
    second.wrapping_add(first).wrapping_add(odd)
}

/// The triangle index count: three per triangle without strips, else the
/// strip lengths' sum.
fn tri_list_count(e: &mut Engine, this: Ptr, read_lengths: bool, stream: Ptr) -> u32 {
    let base = this.addr();
    let strips = e.mem.u16(base + PART_STRIPS);
    if strips == 0 {
        return u16_at(e, base + PART_TRIANGLES) * 3;
    }
    let bytes = u32::from(strips) * 2;
    if read_lengths {
        let lengths = e.call(NI_ALLOC, &args![bytes]).u32();
        e.mem.set_u32(base + PART_STRIP_LENGTHS, lengths);
        let bytes = u16_at(e, base + PART_STRIPS) * 2;
        read(e, stream, lengths, bytes, 2);
    } else {
        let lengths = e.mem.u32(base + PART_STRIP_LENGTHS);
        write(e, stream, lengths, bytes, 2);
    }
    fn_00a81c20(e, this)
}

// Translated from 00a82d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::LoadBinary` (by its use): the five counts,
/// the bones (`NiAlloc`), then, each behind a has-array byte from version
/// 10.0.1.18 (always before), the vertex map, the weights (bones per
/// vertex times vertices floats), the strip lengths (when there are
/// strips) and the triangle list; the bone palette's has-byte is read in
/// every version.
pub fn fn_00a82d90(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    for offset in [
        PART_VERTICES,
        PART_TRIANGLES,
        PART_BONE_COUNT,
        PART_STRIPS,
        PART_BONES_PER_VERTEX,
    ] {
        read(e, stream, base + offset, 2, 2);
    }
    let bones = e
        .call(NI_ALLOC, &args![u16_at(e, base + PART_BONE_COUNT) * 2])
        .u32();
    let bytes = u16_at(e, base + PART_BONE_COUNT) * 2;
    e.mem.set_u32(base + PART_BONES, bones);
    read(e, stream, bones, bytes, 2);
    let tagged = e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_0_1_18;
    e.with_stack(4, |e, flags| {
        let flag = |e: &mut Engine, at: u32| {
            if tagged {
                read(e, stream, at, 1, 1);
            } else {
                e.mem.set_u8(at, 1);
            }
            e.mem.u8(at) != 0
        };
        let has = flags.addr();
        if flag(e, has) {
            let map = e
                .call(NI_ALLOC, &args![u16_at(e, base + PART_VERTICES) * 2])
                .u32();
            let bytes = u16_at(e, base + PART_VERTICES) * 2;
            e.mem.set_u32(base + PART_VERTEX_MAP, map);
            read(e, stream, map, bytes, 2);
        }
        let weight_count =
            u16_at(e, base + PART_BONES_PER_VERTEX).wrapping_mul(u16_at(e, base + PART_VERTICES));
        if flag(e, has + 1) {
            let bytes = weight_count.wrapping_mul(4);
            let weights = e.call(NI_ALLOC, &args![bytes]).u32();
            e.mem.set_u32(base + PART_WEIGHTS, weights);
            read(e, stream, weights, bytes, 4);
        }
        let count = tri_list_count(e, this, true, stream);
        if flag(e, has + 2) {
            let bytes = count.wrapping_mul(2);
            let list = e.call(NI_ALLOC, &args![bytes]).u32();
            e.mem.set_u32(base + PART_TRI_LIST, list);
            read(e, stream, list, bytes, 2);
        }
        read(e, stream, has + 3, 1, 1);
        if e.mem.u8(has + 3) != 0 {
            let palette = e.call(NI_ALLOC, &args![weight_count]).u32();
            e.mem.set_u32(base + PART_BONE_PALETTE, palette);
            read(e, stream, palette, weight_count, 1);
        }
    });
}

// Translated from 00a830d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::SaveBinary` (by its use): the five counts,
/// the bones, then for the vertex map, the weights, the triangle list and
/// the bone palette a has-array byte (whether the array is set) followed
/// by the array when set; the strip lengths go before the triangle list's
/// byte when there are strips.
pub fn fn_00a830d0(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    for offset in [
        PART_VERTICES,
        PART_TRIANGLES,
        PART_BONE_COUNT,
        PART_STRIPS,
        PART_BONES_PER_VERTEX,
    ] {
        write(e, stream, base + offset, 2, 2);
    }
    let bones = e.mem.u32(base + PART_BONES);
    write(e, stream, bones, u16_at(e, base + PART_BONE_COUNT) * 2, 2);
    e.with_stack(4, |e, flags| {
        let has = flags.addr();
        let flag = |e: &mut Engine, at: u32, field: u32| {
            let set = e.mem.u32(base + field) != 0;
            e.mem.set_u8(at, u8::from(set));
            write(e, stream, at, 1, 1);
            e.mem.u8(at) != 0
        };
        if flag(e, has, PART_VERTEX_MAP) {
            let map = e.mem.u32(base + PART_VERTEX_MAP);
            write(e, stream, map, u16_at(e, base + PART_VERTICES) * 2, 2);
        }
        let weight_count =
            u16_at(e, base + PART_BONES_PER_VERTEX).wrapping_mul(u16_at(e, base + PART_VERTICES));
        if flag(e, has + 1, PART_WEIGHTS) {
            let weights = e.mem.u32(base + PART_WEIGHTS);
            write(e, stream, weights, weight_count.wrapping_mul(4), 4);
        }
        let count = tri_list_count(e, this, false, stream);
        if flag(e, has + 2, PART_TRI_LIST) {
            let list = e.mem.u32(base + PART_TRI_LIST);
            write(e, stream, list, count.wrapping_mul(2), 2);
        }
        if flag(e, has + 3, PART_BONE_PALETTE) {
            let palette = e.mem.u32(base + PART_BONE_PALETTE);
            write(e, stream, palette, weight_count, 1);
        }
    });
}

// Translated from 00a83c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::LoadBinary` (Xbox PDB): the `NiObject` part, the
/// partition count, the partitions array (a `MemoryManager` block with the
/// count word in front, the `Partition`s built by the vector constructor;
/// all ones on overflow) and each partition's `LoadBinary`.
pub fn ni_skin_partition_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    read(e, stream, base + PARTITIONS, 4, 4);
    let count = e.mem.u32(base + PARTITIONS);
    let bytes = count
        .checked_mul(PARTITION_SIZE)
        .unwrap_or(0xffff_ffff)
        .checked_add(4)
        .unwrap_or(0xffff_ffff);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    let parts = if block != 0 {
        e.mem.set_u32(block, count);
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![block + 4, PARTITION_SIZE, count, PARTITION_CONSTRUCT],
        );
        block + 4
    } else {
        0
    };
    e.mem.set_u32(base + PARTITION_ARRAY, parts);
    let mut index = 0u32;
    while index < e.mem.u32(base + PARTITIONS) {
        let part = e
            .mem
            .u32(base + PARTITION_ARRAY)
            .wrapping_add(index.wrapping_mul(PARTITION_SIZE));
        fn_00a82d90(e, Ptr::new(part), stream);
        index += 1;
    }
}

// Translated from 00a83ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::SaveBinary` (Xbox PDB): the base part, the partition
/// count, then each partition's `SaveBinary`.
pub fn ni_skin_partition_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    write(e, stream, base + PARTITIONS, 4, 4);
    let mut index = 0u32;
    while index < e.mem.u32(base + PARTITIONS) {
        let part = e
            .mem
            .u32(base + PARTITION_ARRAY)
            .wrapping_add(index.wrapping_mul(PARTITION_SIZE));
        fn_00a830d0(e, Ptr::new(part), stream);
        index += 1;
    }
}

/// `NiFree` (`00aa10f0`, cdecl: the block).
const NI_FREE: u32 = 0x00aa_10f0;
/// `NiObject::IsEqual` (`00a5d470`, `__thiscall`, the other object).
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// The `NiSkinPartition` part of `NiObject`'s table: the partitions' vertex, triangle, bone,
/// strip and bones-per-vertex counts, compared by `Partition::IsEqual`.
const PART_COUNTS: [u32; 5] = [
    PART_VERTICES,
    PART_TRIANGLES,
    PART_BONE_COUNT,
    PART_STRIPS,
    PART_BONES_PER_VERTEX,
];
/// The geometry data's virtual +0xac (`__thiscall`, four words: a triangle index and the
/// addresses of its three vertex indices, `u16` each).
const GET_TRIANGLE_SLOT: u32 = 0xac;
/// The vtables of `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, float>`
/// (`0109e50c`) and `NiTPointerMap<unsigned int, float>` (`0109e52c`) (engine map names of
/// their slots).
const NI_T_MAP_BASE_VTABLE: u32 = 0x0109_e50c;
const NI_T_POINTER_MAP_VTABLE: u32 = 0x0109_e52c;
/// The maps' virtuals: `KeyToHashIndex` (+0x4, the key), `IsKeysEqual` (+0x8, two keys),
/// `SetValue` (+0xc: item, key, value) and `NewItem` (+0x14, no words).
const MAP_KEY_TO_HASH_INDEX_SLOT: u32 = 0x4;
const MAP_IS_KEYS_EQUAL_SLOT: u32 = 0x8;
const MAP_SET_VALUE_SLOT: u32 = 0xc;
const MAP_NEW_ITEM_SLOT: u32 = 0x14;
/// `00b601a0` (`__thiscall` on a map, no words; the map's `RemoveAll` shape: every item
/// through its virtuals +0x10 and +0x18, the count cleared).
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// The shared item allocator of `NiTPointerAllocator<unsigned int>`: its lock (`011c5f80`, the
/// owner word, then the nesting count at `011c5f84`), its free list (`011c5f58`), and the
/// lock's enter (`0040fbf0`, `__thiscall` on the lock, one word).
const ALLOCATOR_LOCK: u32 = 0x011c_5f80;
const ALLOCATOR_LOCK_COUNT: u32 = 0x011c_5f84;
const ALLOCATOR_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// `bsearch` (`00ec716d`) and `qsort` (`00ec6f20`) (cdecl), with the comparison `00a81b80`
/// they get for `u16` arrays.
const BSEARCH: u32 = 0x00ec_716d;
const QSORT: u32 = 0x00ec_6f20;
const COMPARE_U16: u32 = 0x00a8_1b80;

/// The inlined `memcmp(a, b, bytes) == 0`: the code compares four bytes at a time, then the
/// last one to three; the answer is the same as comparing byte by byte.
fn same_bytes(e: &Engine, a: u32, b: u32, bytes: u32) -> bool {
    (0..bytes).all(|i| e.mem.u8(a.wrapping_add(i)) == e.mem.u8(b.wrapping_add(i)))
}

// Translated from 00a81c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::IsEqual` (Xbox PDB): the same five counts (vertices,
/// triangles, bones, strips, bones per vertex), then the same bones, vertex map and weights;
/// without strips the same triangle list (three indices per triangle), with strips the same
/// strip lengths and triangle list (their sum); finally both bone palettes null, or both set
/// and the same (one byte per weight).
pub fn ni_skin_partition_partition_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    for offset in PART_COUNTS {
        if e.mem.u16(a + offset) != e.mem.u16(b + offset) {
            return false;
        }
    }
    let vertices = u16_at(e, a + PART_VERTICES);
    let compare = |e: &Engine, field: u32, bytes: u32| {
        same_bytes(e, e.mem.u32(a + field), e.mem.u32(b + field), bytes)
    };
    if !compare(e, PART_BONES, u16_at(e, a + PART_BONE_COUNT) * 2) {
        return false;
    }
    if !compare(e, PART_VERTEX_MAP, vertices * 2) {
        return false;
    }
    let weights = u16_at(e, a + PART_BONES_PER_VERTEX).wrapping_mul(vertices);
    if !compare(e, PART_WEIGHTS, weights.wrapping_mul(4)) {
        return false;
    }
    let strips = u16_at(e, a + PART_STRIPS);
    if strips == 0 {
        if !compare(e, PART_TRI_LIST, u16_at(e, a + PART_TRIANGLES) * 6) {
            return false;
        }
    } else {
        if !compare(e, PART_STRIP_LENGTHS, strips * 2) {
            return false;
        }
        let indices = fn_00a81c20(e, this);
        if !compare(e, PART_TRI_LIST, indices.wrapping_mul(2)) {
            return false;
        }
    }
    let mine = e.mem.u32(a + PART_BONE_PALETTE);
    let theirs = e.mem.u32(b + PART_BONE_PALETTE);
    if (mine == 0) != (theirs == 0) {
        return false;
    }
    mine == 0 || same_bytes(e, mine, theirs, weights)
}

// Translated from 00a81f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiSkinPartition` constructor body (`NiObject`'s
/// constructor `00a5d3a0`, the class's vtable `0109e47c`, no partitions array; the count at
/// +8 is left as it is). Answers `this`.
pub fn fn_00a81f90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_SKIN_PARTITION_VTABLE);
    e.mem.set_u32(this.addr() + PARTITION_ARRAY, 0);
    this
}

// Translated from 00a81ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::IsEqual` (Xbox PDB): `NiObject::IsEqual` (`00a5d470`), the same partition
/// count, and each partition equal to the other's (`Partition::IsEqual`).
pub fn ni_skin_partition_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = e.mem.u32(this.addr() + PARTITIONS);
    if count != e.mem.u32(other.addr() + PARTITIONS) {
        return false;
    }
    let mine = e.mem.u32(this.addr() + PARTITION_ARRAY);
    let theirs = e.mem.u32(other.addr() + PARTITION_ARRAY);
    let mut index = 0u32;
    while index < count {
        let offset = index.wrapping_mul(PARTITION_SIZE);
        let a = Ptr::new(mine.wrapping_add(offset));
        let b = Ptr::new(theirs.wrapping_add(offset));
        if !ni_skin_partition_partition_is_equal(e, a, b) {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00a82050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `SetAt` of the `unsigned int` to `float` map (`__thiscall`:
/// key, value). The bucket (`KeyToHashIndex`, virtual +4) is searched with `IsKeysEqual`
/// (virtual +8); a found item takes the value (+8); otherwise a new item (`NewItem`, virtual
/// +0x14) gets key and value (`SetValue`, virtual +0xc), goes at the front of the bucket and
/// the count (+0xc) goes up. The value passes through the x87 stack (a signalling NaN is
/// quieted).
pub fn fn_00a82050(e: &mut Engine, this: Ptr, key: u32, value: f32) {
    let map = this.addr();
    let bucket = e.vcall(map, MAP_KEY_TO_HASH_INDEX_SLOT, &args![key]).u32();
    let mut item = e
        .mem
        .u32(e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(map, MAP_IS_KEYS_EQUAL_SLOT, &args![key, item_key])
            .bool()
        {
            e.mem.set_u32(item + 8, x87_quiet_bits(value.to_bits()));
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(map, MAP_NEW_ITEM_SLOT, &args![]).u32();
    e.vcall(
        map,
        MAP_SET_VALUE_SLOT,
        &args![item, key, x87_quiet_bits(value.to_bits())],
    );
    let head = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    let first = e.mem.u32(head);
    e.mem.set_u32(item, first);
    let head = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    e.mem.set_u32(head, item);
    let count = e.mem.u32(map + 0xc).wrapping_add(1);
    e.mem.set_u32(map + 0xc, count);
}

// Translated from 00a820e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: normalizes a list of (key, weight) pairs (8 bytes each; the
/// array at +0, the count at +8): the weights are summed into a `float` (four at a time, in
/// the code's order, while at least four remain by a signed count, then the rest), and each
/// weight is multiplied by `1 / sum` (a `float`). Nothing is scaled for an empty list.
pub fn fn_00a820e0(e: &mut Engine, this: Ptr) {
    let list = this.addr();
    let count = e.mem.u32(list + 8);
    // The bits of weight `index` (the array is read again for each one, as the code does).
    let weight = |e: &Engine, index: u32| {
        let pairs = e.mem.u32(list);
        e.mem
            .u32(pairs.wrapping_add(index.wrapping_mul(8)).wrapping_add(4))
    };
    // `FLDZ; FSTP`: the `float` sum on the stack.
    let mut sum = 0u32;
    let mut index = 0u32;
    if (count as i32) >= 4 {
        let groups = ((count - 4) >> 2) + 1;
        for group in 0..groups {
            let first = group * 4;
            sum = x87_store32(x87_add(x87_load32(weight(e, first)), x87_m32(sum)));
            for step in 1..4u32 {
                sum = x87_store32(x87_add(x87_load32(sum), x87_m32(weight(e, first + step))));
            }
        }
        index = groups * 4;
    }
    while index < count {
        sum = x87_store32(x87_add(x87_load32(weight(e, index)), x87_m32(sum)));
        index += 1;
    }
    // `FLD sum; FLD1; FDIVRP`: 1 / sum, stored as a `float` and loaded once for the loop.
    let scale = x87_load32(x87_store32(x87_div(1.0, x87_load32(sum))));
    if count != 0 {
        let mut index = 0u32;
        loop {
            let at = e
                .mem
                .u32(list)
                .wrapping_add(index.wrapping_mul(8))
                .wrapping_add(4);
            let product = x87_mul(scale, x87_m32(e.mem.u32(at)));
            e.mem.set_u32(at, x87_store32(product));
            index += 1;
            if index >= e.mem.u32(list + 8) {
                break;
            }
        }
    }
}

/// Element `index` of a set's `u16` array (the array pointer at +0 read each time).
fn set_u16(e: &Engine, set: u32, index: u32) -> u32 {
    u16_at(e, e.mem.u32(set).wrapping_add(index.wrapping_mul(2)))
}

// Translated from 00a82190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether every element of `other` is in `this`, both sorted
/// sets of `u16` (elements at +0, count at +8): for each element of `other` the scan of
/// `this` moves on past smaller elements; reaching the end, or a different element, answers
/// false. `this`'s count is read once.
pub fn fn_00a82190(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (mine, theirs) = (this.addr(), other.addr());
    let mut at = 0u32;
    let mut index = 0u32;
    if e.mem.u32(theirs + 8) == 0 {
        return true;
    }
    let count = e.mem.u32(mine + 8);
    loop {
        if at < count {
            while set_u16(e, theirs, index) > set_u16(e, mine, at) {
                at += 1;
                if at >= count {
                    break;
                }
            }
        }
        if at == count {
            return false;
        }
        if set_u16(e, mine, at) != set_u16(e, theirs, index) {
            return false;
        }
        index += 1;
        at += 1;
        if index >= e.mem.u32(theirs + 8) {
            return true;
        }
    }
}

// Translated from 00a821f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the key (the low 16 bits of the first word) of every
/// 8-byte item of `other` (items at +0, count at +8, read once) is in `this`, a sorted set of
/// `u16` (elements at +0, count at +8), each looked up with `bsearch` (`00ec716d`, element size
/// 2, comparison `00a81b80`) on a stack copy of the key (the argument's own stack word).
pub fn fn_00a821f0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (mine, theirs) = (this.addr(), other.addr());
    let count = e.mem.u32(theirs + 8);
    e.with_stack(4, |e, key| {
        let mut index = 0u32;
        while index < count {
            let items = e.mem.u32(theirs);
            let value = u16_at(e, items.wrapping_add(index.wrapping_mul(8)));
            e.mem.set_u32(key.addr(), value);
            let base = e.mem.u32(mine);
            let size = e.mem.u32(mine + 8);
            let found = e
                .call(BSEARCH, &args![key, base, size, 2u32, COMPARE_U16])
                .u32();
            if found == 0 {
                return false;
            }
            index += 1;
        }
        true
    })
}

/// One side of the merge iterator `00a82250` steps: the index word at `index`, the current
/// value at `value` and the set at `set`; the next value is the set's element at the new
/// index, or 0xffff past its end.
fn merge_step(e: &mut Engine, iterator: u32, index: u32, value: u32, set: u32) {
    let next = e.mem.u32(iterator + index).wrapping_add(1);
    e.mem.set_u32(iterator + index, next);
    let set = e.mem.u32(iterator + set);
    let current = if next < e.mem.u32(set + 8) {
        e.mem.u16(e.mem.u32(set).wrapping_add(next.wrapping_mul(2)))
    } else {
        0xffff
    };
    e.mem.set_u16(iterator + value, current);
}

// Translated from 00a82250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: one step of a merge of two sorted `u16` sets (`__thiscall` on a
/// stack iterator: the two indices at +0 and +4, the current values at +8 and +0xa, the sets at
/// +0xc and +0x10). Answers the smaller current value and moves past it: the second set alone
/// when its value is smaller, both when they are equal, the first alone otherwise.
pub fn fn_00a82250(e: &mut Engine, this: Ptr) -> u16 {
    let iterator = this.addr();
    let first = e.mem.u16(iterator + 8);
    let second = e.mem.u16(iterator + 0xa);
    if second < first {
        merge_step(e, iterator, 4, 0xa, 0x10);
        return second;
    }
    if second == first {
        merge_step(e, iterator, 4, 0xa, 0x10);
    }
    merge_step(e, iterator, 0, 8, 0xc);
    first
}

// Translated from 00a822d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on a `Partition`: the set of triangles, the
/// geometry data): builds the partition's vertex map. A table of one `u16` per vertex of the
/// data (its count at +8; `00aa1070`) is filled with 0xffff; each listed triangle's three
/// vertices (the data's virtual +0xac) get the next new index the first time they are seen.
/// The partition's vertex count (+0x1c) is the number of new indices and its vertex map
/// (+0x0c, `00aa1070`) maps each new index back to its vertex. Answers the table.
// Translation incomplete: 00a822d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a822d0(e: &mut Engine, this: Ptr, triangles: Ptr, data: Ptr) -> Ptr {
    let base = this.addr();
    let vertices = u16_at(e, data.addr() + 8);
    let table = e.call(NI_ALLOC, &args![vertices * 2]).u32();
    for index in 0..vertices {
        e.mem.set_u16(table + 2 * index, 0xffff);
    }
    let mut next = 0u32;
    e.with_stack(8, |e, corners| {
        let corners = corners.addr();
        let mut triangle = 0u32;
        while triangle < u16_at(e, base + PART_TRIANGLES) {
            let index = set_u16(e, triangles.addr(), triangle);
            e.vcall(
                data.addr(),
                GET_TRIANGLE_SLOT,
                &args![index, corners, corners + 2, corners + 4],
            );
            for corner in 0..3u32 {
                let vertex = u16_at(e, corners + 2 * corner);
                if e.mem.u16(table + 2 * vertex) == 0xffff {
                    e.mem.set_u16(table + 2 * vertex, next as u16);
                    next += 1;
                }
            }
            triangle += 1;
        }
    });
    let map = e.call(NI_ALLOC, &args![next * 2]).u32();
    e.mem.set_u16(base + PART_VERTICES, next as u16);
    e.mem.set_u32(base + PART_VERTEX_MAP, map);
    for vertex in 0..vertices {
        let new = e.mem.u16(table + 2 * vertex);
        if new != 0xffff {
            let map = e.mem.u32(base + PART_VERTEX_MAP);
            e.mem.set_u16(map + 2 * u32::from(new), vertex as u16);
        }
    }
    Ptr::new(table)
}

// Translated from 00a82400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on a `Partition`: the set of triangles, the
/// geometry data, the vertex table `00a822d0` made): the partition's triangle list (+0x14,
/// three `u16` per triangle, `00aa1070`) holds each listed triangle's three vertices (the
/// data's virtual +0xac) through the table.
// Translation incomplete: 00a82400 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a82400(e: &mut Engine, this: Ptr, triangles: Ptr, data: Ptr, table: Ptr) {
    let base = this.addr();
    let list = e
        .call(NI_ALLOC, &args![u16_at(e, base + PART_TRIANGLES) * 6])
        .u32();
    e.mem.set_u32(base + PART_TRI_LIST, list);
    e.with_stack(8, |e, corners| {
        let corners = corners.addr();
        let mut triangle = 0u32;
        let mut offset = 0u32;
        while triangle < u16_at(e, base + PART_TRIANGLES) {
            let index = set_u16(e, triangles.addr(), triangle);
            e.vcall(
                data.addr(),
                GET_TRIANGLE_SLOT,
                &args![index, corners, corners + 2, corners + 4],
            );
            let out = e.mem.u32(base + PART_TRI_LIST).wrapping_add(offset);
            for corner in 0..3u32 {
                let vertex = u16_at(e, corners + 2 * corner);
                let mapped = e.mem.u16(table.addr().wrapping_add(vertex * 2));
                e.mem.set_u16(out + 2 * corner, mapped);
            }
            triangle += 1;
            offset += 6;
        }
    });
}

// Translated from 00a82530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall`, `this` unread: the bone-to-weight map, the
/// address of a triangle's three vertex indices, the per-vertex influence lists of 12 bytes
/// each: (key, weight) pairs at +0, their count at +8): every influence of the three vertices
/// goes into the map (`00a82050`) unless the map already holds the bone with a weight at least
/// as large (or the comparison is unordered).
pub fn fn_00a82530(e: &mut Engine, _unused_0: Ptr, map: Ptr, corners: Ptr, lists: Ptr) {
    let m = map.addr();
    for corner in 0..3u32 {
        let vertex = u16_at(e, corners.addr() + 2 * corner);
        let list = lists.addr().wrapping_add(vertex.wrapping_mul(12));
        let count = e.mem.u32(list + 8);
        let mut index = 0u32;
        while index < count {
            let pair = e.mem.u32(list).wrapping_add(index.wrapping_mul(8));
            let key = e.mem.u32(pair);
            let bucket = e.vcall(m, MAP_KEY_TO_HASH_INDEX_SLOT, &args![key]).u32();
            let mut item = e
                .mem
                .u32(e.mem.u32(m + 8).wrapping_add(bucket.wrapping_mul(4)));
            let mut keep = false;
            while item != 0 {
                let item_key = e.mem.u32(item + 4);
                if e.vcall(m, MAP_IS_KEYS_EQUAL_SLOT, &args![key, item_key])
                    .bool()
                {
                    // `FLD item; FSTP local; FLD local; FLD weight; FCOMPP`: kept when the
                    // weight is not greater (C0 or C3 set; unordered sets both).
                    let held = x87_load32(e.mem.u32(item + 8));
                    let pair = e.mem.u32(list).wrapping_add(index.wrapping_mul(8));
                    let weight = x87_load32(e.mem.u32(pair + 4));
                    keep = !(weight > held);
                    break;
                }
                item = e.mem.u32(item);
            }
            if !keep {
                let pair = e.mem.u32(list).wrapping_add(index.wrapping_mul(8));
                let weight = f32::from_bits(x87_quiet_bits(e.mem.u32(pair + 4)));
                fn_00a82050(e, map, key, weight);
            }
            index += 1;
        }
    }
}

// Translated from 00a82600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, float>::SetValue` (Xbox PDB;
/// `__thiscall`, the map unread): the item's key (+4) and value (+8, through the x87 stack).
pub fn ni_t_map_base_set_value(e: &mut Engine, _unused_0: Ptr, item: Ptr, key: u32, value: f32) {
    e.mem
        .set_u32(item.addr() + 8, x87_quiet_bits(value.to_bits()));
    e.mem.set_u32(item.addr() + 4, key);
}

// Translated from 00a82620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, float>::DeleteItem` (Xbox PDB; `__thiscall`, the map unread):
/// clears the item's value (0.0) and, holding the shared item allocator's lock (`0040fbf0`
/// on `011c5f80` with 0), its key; puts the item at the front of the free list (`011c5f58`)
/// and leaves the lock (its nesting count at `011c5f84` down by one, the owner word cleared at
/// zero).
pub fn ni_t_pointer_map_delete_item(e: &mut Engine, _unused_0: Ptr, item: Ptr) {
    let item = item.addr();
    e.mem.set_u32(item + 8, 0);
    e.call(LOCK_ENTER, &args![ALLOCATOR_LOCK, 0u32]);
    e.mem.set_u32(item + 4, 0);
    let free = e.mem.u32(ALLOCATOR_FREE_LIST);
    e.mem.set_u32(item, free);
    let count = e.mem.u32(ALLOCATOR_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(ALLOCATOR_LOCK_COUNT, count);
    e.mem.set_u32(ALLOCATOR_FREE_LIST, item);
    if count == 0 {
        e.mem.set_u32(ALLOCATOR_LOCK, 0);
    }
}

// Translated from 00a82670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, float>::_scalar_deleting_destructor_`
/// (Xbox PDB): its vtable (`0109e50c`), `RemoveAll` (`00b601a0`), the bucket array freed
/// (`00aa10f0`), and the map itself given back to the `MemoryManager` (`00aa4060` on
/// `011f6238`) when bit 0 of `flags` is set. Answers `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(NI_FREE, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The shared body of the two `Realloc`s: a new array of `size` elements of `stride` bytes
/// (`00aa1070`; none for 0) with the used elements (+8) copied, the old array freed
/// (`00aa10f0`), the new array (+0) and capacity (+4) stored. Nothing when the capacity is
/// already `size`.
fn set_realloc(e: &mut Engine, set: u32, size: u32, stride: u32) {
    if size == e.mem.u32(set + 4) {
        return;
    }
    let array = if size != 0 {
        let array = e.call(NI_ALLOC, &args![size.wrapping_mul(stride)]).u32();
        let mut index = 0u32;
        while index < e.mem.u32(set + 8) {
            let from = e.mem.u32(set).wrapping_add(index * stride);
            let bytes = e.mem.bytes(from, stride);
            e.mem.write(array.wrapping_add(index * stride), &bytes);
            index += 1;
        }
        array
    } else {
        0
    };
    let old = e.mem.u32(set);
    e.call(NI_FREE, &args![old]);
    e.mem.set_u32(set, array);
    e.mem.set_u32(set + 4, size);
}

// Translated from 00a826b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `Realloc` of a set of 8-byte elements (array +0, capacity +4,
/// count +8) to `size` elements; the elements are copied as two words.
pub fn fn_00a826b0(e: &mut Engine, this: Ptr, size: u32) {
    set_realloc(e, this.addr(), size, 8);
}

// Translated from 00a82710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `Realloc` of a set of `u16` (array +0, capacity +4, count +8) to
/// `size` elements.
pub fn fn_00a82710(e: &mut Engine, this: Ptr, size: u32) {
    set_realloc(e, this.addr(), size, 2);
}

/// The stack iterator `00a82250` steps, over `first` and `second` (sorted `u16` sets): both
/// indices 0 and each current value the set's first element (read whatever the count).
fn merge_iterator(e: &mut Engine, iterator: u32, first: u32, second: u32) {
    let a = e.mem.u16(e.mem.u32(first));
    let b = e.mem.u16(e.mem.u32(second));
    e.mem.set_u32(iterator + 4, 0);
    e.mem.set_u32(iterator, 0);
    e.mem.set_u16(iterator + 8, a);
    e.mem.set_u16(iterator + 0xa, b);
    e.mem.set_u32(iterator + 0xc, first);
    e.mem.set_u32(iterator + 0x10, second);
}

// Translated from 00a82770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the size of the union of two sorted `u16` sets: the merge
/// (`00a82250`) is stepped until both current values are 0xffff.
// Translation incomplete: 00a82770 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a82770(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.with_stack(0x14, |e, iterator| {
        let it = iterator.addr();
        merge_iterator(e, it, this.addr(), other.addr());
        let mut count = 0u32;
        while e.mem.u16(it + 8) != 0xffff || e.mem.u16(it + 0xa) != 0xffff {
            fn_00a82250(e, iterator);
            count += 1;
        }
        count
    })
}

// Translated from 00a827f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this` becomes the union of itself and `other` (sorted `u16`
/// sets): a new array of both counts' sum (`00aa1070`) gets the merge's values (`00a82250`)
/// until both current values are 0xffff; the old array is freed and the new one, that
/// capacity and the merged count stored.
// Translation incomplete: 00a827f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a827f0(e: &mut Engine, this: Ptr, other: Ptr) {
    let set = this.addr();
    e.with_stack(0x14, |e, iterator| {
        let it = iterator.addr();
        let capacity = e.mem.u32(other.addr() + 8).wrapping_add(e.mem.u32(set + 8));
        merge_iterator(e, it, set, other.addr());
        let array = e.call(NI_ALLOC, &args![capacity.wrapping_mul(2)]).u32();
        let mut count = 0u32;
        while e.mem.u16(it + 8) != 0xffff || e.mem.u16(it + 0xa) != 0xffff {
            let value = fn_00a82250(e, iterator);
            e.mem.set_u16(array.wrapping_add(count * 2), value);
            count += 1;
        }
        let old = e.mem.u32(set);
        e.call(NI_FREE, &args![old]);
        e.mem.set_u32(set + 4, capacity);
        e.mem.set_u32(set, array);
        e.mem.set_u32(set + 8, count);
    });
}

// Translated from 00a82890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on a sorted `u16` bone set: the address of a
/// triangle's three vertex indices, the per-vertex influence lists of 12 bytes): whether the
/// set holds every bone of the three vertices (`00a821f0` on each one's list).
pub fn fn_00a82890(e: &mut Engine, this: Ptr, corners: Ptr, lists: Ptr) -> bool {
    for corner in 0..3u32 {
        let vertex = u16_at(e, corners.addr() + 2 * corner);
        let list = lists.addr().wrapping_add(vertex.wrapping_mul(12));
        if !fn_00a821f0(e, this, Ptr::new(list)) {
            return false;
        }
    }
    true
}

// Translated from 00a82c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: a set of bone-set pointers (array +0, count +8), the
/// address of a triangle's three vertex indices, the influence lists, a byte per bone set, a
/// byte to match): the first bone set whose byte is the given one and that holds every bone of
/// the triangle (`00a82890`) has its byte set to 1 and its index answered; -1 when none does.
pub fn fn_00a82c90(
    e: &mut Engine,
    sets: Ptr,
    corners: Ptr,
    lists: Ptr,
    used: Ptr,
    flag: u8,
) -> i32 {
    let mut index = 0u32;
    while index < e.mem.u32(sets.addr() + 8) {
        let at = used.addr().wrapping_add(index);
        if e.mem.u8(at) == flag {
            let set = e.mem.u32(e.mem.u32(sets.addr()).wrapping_add(index * 4));
            if fn_00a82890(e, Ptr::new(set), corners, lists) {
                e.mem.set_u8(at, 1);
                return index as i32;
            }
        }
        index += 1;
    }
    -1
}

// Translated from 00a82cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, float>::_scalar_deleting_destructor_` (Xbox PDB): its vtable
/// (`0109e52c`) and `RemoveAll` (`00b601a0`), then the base's vtable (`0109e50c`) and
/// `RemoveAll` again, the bucket array freed (`00aa10f0`), and the map given back to the
/// `MemoryManager` (`00aa4060` on `011f6238`) when bit 0 of `flags` is set. Answers `this`.
pub fn ni_t_pointer_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_T_POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(NI_FREE, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The inlined `Add` of a set (array +0, capacity +4, count +8): when full, grown to twice
/// its capacity (1 from 0) by `realloc`, then `value` stored at the count, which goes up.
fn set_add_u16(e: &mut Engine, set: u32, value: u16) {
    let capacity = e.mem.u32(set + 4);
    if e.mem.u32(set + 8) == capacity {
        let size = if capacity != 0 { capacity * 2 } else { 1 };
        fn_00a82710(e, Ptr::new(set), size);
    }
    let count = e.mem.u32(set + 8);
    e.mem.set_u16(e.mem.u32(set).wrapping_add(count * 2), value);
    e.mem.set_u32(set + 8, count.wrapping_add(1));
}

// Translated from 00a82d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on a `u16` set: a word, an array of words, its
/// count): adds to the set the index of every array word equal to the given one (the set
/// doubling, from 1, through `00a82710` when full).
pub fn fn_00a82d30(e: &mut Engine, this: Ptr, value: u32, words: Ptr, count: u32) {
    let mut index = 0u32;
    while index < count {
        if e.mem.u32(words.addr().wrapping_add(index * 4)) == value {
            set_add_u16(e, this.addr(), index as u16);
        }
        index += 1;
    }
}

/// `-FLT_MAX` (`01015f5c`): what `00a828d0` leaves in a weight it has taken.
const NEGATIVE_MAXIMUM: u32 = 0x0101_5f5c;

// Translated from 00a828d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on a `Partition`, eight words: the set of its
/// triangles, its bone set, the geometry data, the per-vertex influence lists (12 bytes each:
/// (bone, weight) pairs at +0, their count at +8), a "limited" byte, a palette byte, the most
/// bones per vertex (a byte) and the skin's bone count). Fills a partition: its bone count is
/// the bone set's count or the byte, whichever is larger (signed), and its bones (`00aa1070`)
/// are the set's bones padded with 0; its triangle count is the set's. Its vertex map
/// (`00a822d0`) and triangle list (`00a82400`) are made and the vertex table freed.
///
/// Limited: bones per vertex is the byte; for each partition vertex its influences (weight and
/// the bone's index in the partition, through a table of the skin's bone count) are copied
/// to two stack buffers padded with 0, then the largest weight (the first of equal ones) is
/// taken bones-per-vertex times into the weights and the palette, each taken weight replaced
/// by `-FLT_MAX` (`01015f5c`).
///
/// Otherwise: bones per vertex is the bone count; each vertex gets, for each partition bone,
/// its weight for that bone (0 when it has none), padded with 0 to the bone count; with the
/// palette byte the palette lists 0, 1, .. for every vertex.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a828d0(
    e: &mut Engine,
    this: Ptr,
    triangles: Ptr,
    bones: Ptr,
    data: Ptr,
    lists: Ptr,
    limited: u8,
    palette: u8,
    bones_per_vertex: u8,
    bone_count: u32,
) {
    let base = this.addr();
    let set_count = e.mem.u32(bones.addr() + 8);
    let most = u32::from(bones_per_vertex);
    let count = if (set_count as i32) > (most as i32) {
        set_count
    } else {
        most
    };
    e.mem.set_u16(base + PART_BONE_COUNT, count as u16);
    let triangle_count = e.mem.u16(triangles.addr() + 8);
    e.mem.set_u16(base + PART_TRIANGLES, triangle_count);
    let array = e.call(NI_ALLOC, &args![(count & 0xffff) * 2]).u32();
    e.mem.set_u32(base + PART_BONES, array);
    let mut index = 0u32;
    while index < u16_at(e, base + PART_BONE_COUNT) {
        let bone = if index < e.mem.u32(bones.addr() + 8) {
            set_u16(e, bones.addr(), index) as u16
        } else {
            0
        };
        let array = e.mem.u32(base + PART_BONES);
        e.mem.set_u16(array + 2 * index, bone);
        index += 1;
    }
    let table = fn_00a822d0(e, this, triangles, data);
    fn_00a82400(e, this, triangles, data, table);
    e.call(NI_FREE, &args![table]);
    let list_of = |e: &Engine, vertex: u32| {
        let map = e.mem.u32(base + PART_VERTEX_MAP);
        lists
            .addr()
            .wrapping_add(u16_at(e, map + 2 * vertex).wrapping_mul(12))
    };
    if limited != 0 {
        e.mem
            .set_u16(base + PART_BONES_PER_VERTEX, u16::from(bones_per_vertex));
        let weight_count = most.wrapping_mul(u16_at(e, base + PART_VERTICES));
        let weights = e.call(NI_ALLOC, &args![weight_count.wrapping_mul(4)]).u32();
        e.mem.set_u32(base + PART_WEIGHTS, weights);
        let palette_bytes = e.call(NI_ALLOC, &args![weight_count]).u32();
        e.mem.set_u32(base + PART_BONE_PALETTE, palette_bytes);
        let local = e.call(NI_ALLOC, &args![bone_count.wrapping_mul(4)]).u32();
        let mut index = 0u32;
        while index < e.mem.u32(bones.addr() + 8) {
            let bone = set_u16(e, base + PART_BONES, index);
            e.mem.set_u32(local.wrapping_add(bone * 4), index);
            index += 1;
        }
        let mut weight_at = e.mem.u32(base + PART_WEIGHTS);
        let mut palette_at = e.mem.u32(base + PART_BONE_PALETTE);
        let held_weights = e
            .call(
                NI_ALLOC,
                &args![u16_at(e, base + PART_BONES_PER_VERTEX) * 4],
            )
            .u32();
        let held_bones = e
            .call(NI_ALLOC, &args![u16_at(e, base + PART_BONES_PER_VERTEX)])
            .u32();
        if u16_at(e, base + PART_VERTICES) != 0 {
            let taken = x87_store32(x87_load32(e.mem.u32(NEGATIVE_MAXIMUM)));
            let mut vertex = 0u32;
            loop {
                let list = list_of(e, vertex);
                let mut slot = 0u32;
                while slot < e.mem.u32(list + 8) {
                    let pair = e.mem.u32(list).wrapping_add(slot * 8);
                    let weight = x87_quiet_bits(e.mem.u32(pair + 4));
                    e.mem.set_u32(held_weights.wrapping_add(slot * 4), weight);
                    let bone = e.mem.u32(pair);
                    let partition_bone = e.mem.u8(local.wrapping_add(bone.wrapping_mul(4)));
                    e.mem.set_u8(held_bones.wrapping_add(slot), partition_bone);
                    slot += 1;
                }
                while slot < u16_at(e, base + PART_BONES_PER_VERTEX) {
                    e.mem.set_u32(held_weights.wrapping_add(slot * 4), 0);
                    e.mem.set_u8(held_bones.wrapping_add(slot), 0);
                    slot += 1;
                }
                let mut pass = 0u32;
                while pass < u16_at(e, base + PART_BONES_PER_VERTEX) {
                    let width = u16_at(e, base + PART_BONES_PER_VERTEX);
                    let mut best = x87_quiet_bits(e.mem.u32(held_weights));
                    let mut pick = 0u32;
                    let mut at = 1u32;
                    while at < width {
                        let candidate = e.mem.u32(held_weights.wrapping_add(at * 4));
                        // `FLD candidate; FLD best; FCOMPP; TEST AH,5; JP`: taken when best is
                        // below it (C0 alone).
                        if x87_load32(best) < x87_load32(candidate) {
                            best = x87_quiet_bits(candidate);
                            pick = at;
                        }
                        at += 1;
                    }
                    let chosen = e.mem.u32(held_weights.wrapping_add(pick * 4));
                    e.mem.set_u32(weight_at, x87_quiet_bits(chosen));
                    weight_at = weight_at.wrapping_add(4);
                    let bone = e.mem.u8(held_bones.wrapping_add(pick));
                    e.mem.set_u8(palette_at, bone);
                    e.mem.set_u32(held_weights.wrapping_add(pick * 4), taken);
                    palette_at = palette_at.wrapping_add(1);
                    pass += 1;
                }
                vertex += 1;
                if vertex >= u16_at(e, base + PART_VERTICES) {
                    break;
                }
            }
        }
        e.call(NI_FREE, &args![held_weights]);
        e.call(NI_FREE, &args![held_bones]);
        e.call(NI_FREE, &args![local]);
        return;
    }
    let bone_total = e.mem.u16(base + PART_BONE_COUNT);
    e.mem.set_u16(base + PART_BONES_PER_VERTEX, bone_total);
    let weight_count = u32::from(bone_total).wrapping_mul(u16_at(e, base + PART_VERTICES));
    let weights = e.call(NI_ALLOC, &args![weight_count.wrapping_mul(4)]).u32();
    e.mem.set_u32(base + PART_WEIGHTS, weights);
    let mut weight_at = weights;
    if u16_at(e, base + PART_VERTICES) != 0 {
        let mut vertex = 0u32;
        loop {
            let list = list_of(e, vertex);
            let mut index = 0u32;
            while index < e.mem.u32(bones.addr() + 8) {
                let influences = e.mem.u32(list + 8);
                let bone = set_u16(e, bones.addr(), index);
                let pairs = e.mem.u32(list);
                let mut weight = 0u32;
                let mut slot = 0u32;
                while slot < influences {
                    let pair = pairs.wrapping_add(slot * 8);
                    if e.mem.u32(pair) == bone {
                        weight = x87_quiet_bits(e.mem.u32(pair + 4));
                        break;
                    }
                    slot += 1;
                }
                e.mem.set_u32(weight_at, weight);
                weight_at = weight_at.wrapping_add(4);
                index += 1;
            }
            while index < u16_at(e, base + PART_BONE_COUNT) {
                e.mem.set_u32(weight_at, 0);
                weight_at = weight_at.wrapping_add(4);
                index += 1;
            }
            vertex += 1;
            if vertex >= u16_at(e, base + PART_VERTICES) {
                break;
            }
        }
    }
    if palette != 0 {
        let bytes = e.call(NI_ALLOC, &args![weight_count]).u32();
        e.mem.set_u32(base + PART_BONE_PALETTE, bytes);
        let mut at = bytes;
        let mut vertex = 0u32;
        while vertex < u16_at(e, base + PART_VERTICES) {
            let mut bone = 0u32;
            while bone < u16_at(e, base + PART_BONE_COUNT) {
                e.mem.set_u8(at, bone as u8);
                at = at.wrapping_add(1);
                bone += 1;
            }
            vertex += 1;
        }
    }
}

/// Deletes a bone set made by `00a83d40`: its array freed (`00aa10f0`) and the 0x10-byte set
/// given back to the `MemoryManager` (`00aa4060` on `011f6238`); nothing for null.
fn delete_bone_set(e: &mut Engine, set: u32) {
    if set != 0 {
        let array = e.mem.u32(set);
        e.call(NI_FREE, &args![array]);
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, set]);
    }
}

/// Removes element `index` of a set of words (array +0, count +8) by moving the last one
/// into its place.
fn set_remove_word(e: &mut Engine, set: u32, index: u32) {
    let count = e.mem.u32(set + 8).wrapping_sub(1);
    e.mem.set_u32(set + 8, count);
    let array = e.mem.u32(set);
    let last = e.mem.u32(array.wrapping_add(count.wrapping_mul(4)));
    e.mem
        .set_u32(array.wrapping_add(index.wrapping_mul(4)), last);
}

// Translated from 00a839d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall`, `this` unread: the most bones a partition may
/// have (a byte), a set of bone-set pointers (array +0, count +8)): merges bone sets.
/// Repeatedly the pair (i < j) whose union (`00a82770`) has at most the allowed bones and that
/// shares the most bones (the two counts minus the union's size; -1 when the two counts are
/// equal; the first pair of a signed maximum, above -1) is merged: i takes the union
/// (`00a827f0`); when the two sets' words at +0xc differ, j takes it too and the search starts
/// again; otherwise j is deleted (moved over by the last set) and so is every other set
/// contained in i (`00a82190`), and the search starts again. When no pair is left, every set
/// gets a group number in a 50-word stack table: the number of the first earlier set equal to
/// it (each contains the other), or the next new number; then the sets are reordered by their
/// numbers, largest first (a swap of the table words moves the two sets, each only while its
/// index is below the count). The table is the exe's fixed stack array: more than 50 sets
/// would write past it, as in the exe.
pub fn fn_00a839d0(e: &mut Engine, _unused_0: Ptr, most: u8, sets: Ptr) {
    let s = sets.addr();
    let element = |e: &Engine, index: u32| e.mem.u32(e.mem.u32(s).wrapping_add(index * 4));
    loop {
        let mut best_i = 0u32;
        let mut best_j = 0u32;
        let mut best = -1i32;
        let mut i = 0u32;
        while i < e.mem.u32(s + 8) {
            let a = element(e, i);
            let mut j = i + 1;
            while j < e.mem.u32(s + 8) {
                let b = element(e, j);
                let union = fn_00a82770(e, Ptr::new(a), Ptr::new(b));
                if union <= u32::from(most) {
                    let b_count = e.mem.u32(b + 8);
                    let a_count = e.mem.u32(a + 8);
                    let mut shared = a_count.wrapping_sub(union).wrapping_add(b_count);
                    if a_count == b_count {
                        shared = 0xffff_ffff;
                    }
                    if (shared as i32) > best {
                        best_i = i;
                        best_j = j;
                        best = shared as i32;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        if best < 0 {
            break;
        }
        let a = element(e, best_i);
        let b = element(e, best_j);
        fn_00a827f0(e, Ptr::new(a), Ptr::new(b));
        let b = element(e, best_j);
        let a = element(e, best_i);
        if e.mem.u32(a + 0xc) != e.mem.u32(b + 0xc) {
            fn_00a827f0(e, Ptr::new(b), Ptr::new(a));
            continue;
        }
        delete_bone_set(e, b);
        set_remove_word(e, s, best_j);
        let merged = element(e, best_i);
        let mut k = 0u32;
        while k < e.mem.u32(s + 8) {
            let other = element(e, k);
            if other != merged && fn_00a82190(e, Ptr::new(merged), Ptr::new(other)) {
                delete_bone_set(e, other);
                set_remove_word(e, s, k);
            } else {
                k += 1;
            }
        }
    }
    let count = e.mem.u32(s + 8);
    e.with_stack(0xc8, |e, table| {
        let table = table.addr();
        let slot = |index: u32| table.wrapping_add(index.wrapping_mul(4));
        let mut next = 0u32;
        let mut index = 0u32;
        if count != 0 {
            loop {
                if index == 0 {
                    e.mem.set_u32(slot(0), next);
                    next += 1;
                } else {
                    let array = e.mem.u32(s);
                    let mine = e.mem.u32(array.wrapping_add(index * 4));
                    let mut k = 0u32;
                    while k < index {
                        let other = e.mem.u32(array.wrapping_add(k * 4));
                        // The second test's argument is the `ECX` the first call returns with:
                        // `00a82190` never changes it, so it is `mine`.
                        if fn_00a82190(e, Ptr::new(mine), Ptr::new(other))
                            && fn_00a82190(e, Ptr::new(other), Ptr::new(mine))
                        {
                            let group = e.mem.u32(slot(k));
                            e.mem.set_u32(slot(index), group);
                            break;
                        }
                        k += 1;
                    }
                    if k == index {
                        e.mem.set_u32(slot(index), next);
                        next += 1;
                    }
                }
                index += 1;
                if index >= e.mem.u32(s + 8) {
                    break;
                }
            }
        }
        let mut a = 0u32;
        while a < count {
            let mut b = 0u32;
            while b < a {
                let first = e.mem.u32(slot(a));
                let second = e.mem.u32(slot(b));
                if (first as i32) > (second as i32) {
                    e.mem.set_u32(slot(a), second);
                    e.mem.set_u32(slot(b), first);
                    let array = e.mem.u32(s);
                    let moved = e.mem.u32(array.wrapping_add(a * 4));
                    if a < e.mem.u32(s + 8) {
                        let other = e.mem.u32(array.wrapping_add(b * 4));
                        e.mem.set_u32(array.wrapping_add(a * 4), other);
                    }
                    if b < e.mem.u32(s + 8) {
                        let array = e.mem.u32(s);
                        e.mem.set_u32(array.wrapping_add(b * 4), moved);
                    }
                }
                b += 1;
            }
            a += 1;
        }
    });
}

/// `NiObject::GetViewerStrings` (`00a5d6e0`, `__thiscall`, the string array),
/// `NiGetViewerString` (`00a7d330`, cdecl: a C string), the formatted print `00406d00` (cdecl:
/// buffer, size, format, then its arguments), `"Partition %d = %d bones"` (`0109e548`) and
/// the string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`).
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const PRINT_00406D00: u32 = 0x0040_6d00;
const PARTITION_FORMAT: u32 = 0x0109_e548;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a846e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::GetViewerStrings` (Xbox PDB): `NiObject`'s strings (`00a5d6e0`), the class
/// name (`NiGetViewerString` of `ms_RTTI`'s first word, appended through `SetAt`), then for
/// each partition `"Partition <n> = <bones> bones"` (printed into a 0x20-byte stack buffer)
/// appended with `SetAt` written out: the array grown to its size plus `m_usGrowBy` when full,
/// the size moved past the slot (and the non-null count, +0xc, up for a string) when it was
/// at the end, else the non-null count adjusted for the slot's old and new value.
// Translation incomplete: 00a846e0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_skin_partition_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let strings = strings.addr();
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_SKIN_PARTITION_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let grow_by = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.with_stack(0x24, |e, frame| {
        let element = frame.addr();
        let buffer = element + 4;
        e.mem.set_u32(element, text);
        e.call(STRING_ARRAY_SET_AT, &args![strings, size, element]);
        let mut index = 0u32;
        while index < e.mem.u32(this.addr() + PARTITIONS) {
            let parts = e.mem.u32(this.addr() + PARTITION_ARRAY);
            let bones = u16_at(
                e,
                parts.wrapping_add(index * PARTITION_SIZE + PART_BONE_COUNT),
            );
            e.call(
                PRINT_00406D00,
                &args![buffer, 0x20u32, PARTITION_FORMAT, index, bones],
            );
            let text = e.call(NI_GET_VIEWER_STRING, &args![buffer]).u32();
            let at = u32::from(e.mem.u16(strings + 0xa));
            if at >= u32::from(e.mem.u16(strings + 8)) {
                let grow_by = u32::from(e.mem.u16(strings + 0xe));
                e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + at]);
            }
            let slot = e.mem.u32(strings + 4).wrapping_add(at * 4);
            if at >= u32::from(e.mem.u16(strings + 0xa)) {
                e.mem.set_u16(strings + 0xa, (at + 1) as u16);
                if text != 0 {
                    let used = e.mem.u16(strings + 0xc).wrapping_add(1);
                    e.mem.set_u16(strings + 0xc, used);
                }
            } else if text != 0 {
                if e.mem.u32(slot) == 0 {
                    let used = e.mem.u16(strings + 0xc).wrapping_add(1);
                    e.mem.set_u16(strings + 0xc, used);
                }
            } else if e.mem.u32(slot) != 0 {
                let used = e.mem.u16(strings + 0xc).wrapping_add(0xffff);
                e.mem.set_u16(strings + 0xc, used);
            }
            let slot = e.mem.u32(strings + 4).wrapping_add(at * 4);
            e.mem.set_u32(slot, text);
            index += 1;
        }
    });
}

/// `NiGeometryData` helpers (`nigeometrydata.cpp`, unnamed in the engine map): `00a68820`
/// (`__thiscall`, three words: the addresses of two array pointers it fills, a flag; the
/// first array lists the vertices in an order where equal vertices are next to each other,
/// the second gives each vertex's position in it) and `00a66db0` (`__thiscall`, three words:
/// two vertex indices and flags; 0 when the two vertices are the same).
const GEOMETRY_SORTED_VERTICES: u32 = 0x00a6_8820;
const GEOMETRY_COMPARE_VERTICES: u32 = 0x00a6_6db0;
/// `_memset` (`00ec61c0`, cdecl) and the comparison `00a81b30` `qsort` gets for (bone, weight)
/// pairs.
const MEMSET: u32 = 0x00ec_61c0;
const COMPARE_INFLUENCES: u32 = 0x00a8_1b30;
/// `FLT_MAX` (`01016970`).
const FLOAT_MAXIMUM: u32 = 0x0101_6970;

/// Whether two influence lists (pairs at +0, count at +8) have the same count and the same
/// bones in the same order.
fn same_bones(e: &Engine, list: u32, other: u32) -> bool {
    let count = e.mem.u32(list + 8);
    if count != e.mem.u32(other + 8) {
        return false;
    }
    let (a, b) = (e.mem.u32(list), e.mem.u32(other));
    (0..count).all(|i| e.mem.u32(a.wrapping_add(i * 8)) == e.mem.u32(b.wrapping_add(i * 8)))
}

/// A set of words kept in locals (array, capacity, count), as `00a833b0` inlines it.
struct WordSet {
    array: u32,
    capacity: u32,
    count: u32,
}

impl WordSet {
    /// The inlined `Add`: when full, the capacity doubles (1 from 0) into a new array
    /// (`00aa1070`, the words copied; none for 0) and the old one is freed (`00aa10f0`).
    fn add(&mut self, e: &mut Engine, value: u32) {
        if self.count == self.capacity {
            let size = if self.capacity != 0 {
                self.capacity.wrapping_mul(2)
            } else {
                1
            };
            if size != self.capacity {
                let array = if size != 0 {
                    let array = e.call(NI_ALLOC, &args![size.wrapping_mul(4)]).u32();
                    for index in 0..self.count {
                        let word = e.mem.u32(self.array.wrapping_add(index * 4));
                        e.mem.set_u32(array.wrapping_add(index * 4), word);
                    }
                    array
                } else {
                    0
                };
                e.call(NI_FREE, &args![self.array]);
                self.array = array;
                self.capacity = size;
            }
        }
        e.mem
            .set_u32(self.array.wrapping_add(self.count.wrapping_mul(4)), value);
        self.count = self.count.wrapping_add(1);
    }
}

/// The end of `00a833b0`: the three arrays freed and the stack map torn down as its
/// destructor does (`NiTPointerMap`'s vtable and `RemoveAll`, the base's and `RemoveAll`, the
/// buckets freed).
fn finish_00a833b0(e: &mut Engine, map: u32, sorted: u32, positions: u32, touched: u32) {
    e.call(NI_FREE, &args![sorted]);
    e.call(NI_FREE, &args![positions]);
    e.call(NI_FREE, &args![touched]);
    e.mem.set_u32(map, NI_T_POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    e.mem.set_u32(map, NI_T_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let buckets = e.mem.u32(map + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00a833b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on the partition object, four words: the geometry
/// data, the most bones a triangle may use (a byte), the most influences a vertex may keep
/// (a byte), the per-vertex influence lists of 12 bytes: (bone, weight) pairs at +0, count at
/// +8). Cuts the influences down. A stack `NiTPointerMap<unsigned int, float>` (37 buckets)
/// collects, for each triangle with three different vertices (the data's virtual +0xac), the
/// largest weight of each bone of its vertices (`00a82530`). While it holds more bones than
/// allowed, the bone of the smallest weight (the first one strictly below the running
/// minimum, from `FLT_MAX`) is removed from the lists of the triangle's vertices and of every
/// vertex equal to one of them (`00a68820`'s order, `00a66db0` with flag 1) that has the same
/// bones; each list losing it is renormalized (`00a820e0`), and a list left empty ends the
/// function with false. The map is refilled after each removal. Finally every list with more
/// influences than allowed is sorted (`qsort` with `00a81b30`), cut and renormalized. The
/// arrays and the map are freed on both ends; the answer is true.
// Translation incomplete: 00a833b0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a833b0(e: &mut Engine, this: Ptr, data: Ptr, most: u8, keep: u8, lists: Ptr) -> bool {
    let geometry = data.addr();
    let lists = lists.addr();
    let triangles = u16_at(e, geometry + 0x40);
    let vertex_count = u16_at(e, geometry + 8);
    e.with_stack(0x20, |e, frame| {
        let map = frame.addr();
        let sorted_at = map + 0x10;
        let positions_at = map + 0x14;
        let corners = map + 0x18;
        e.mem.set_u32(map, NI_T_MAP_BASE_VTABLE);
        e.mem.set_u32(map + 4, 0x25);
        e.mem.set_u32(map + 0xc, 0);
        let buckets = e.call(NI_ALLOC, &args![0x94u32]).u32();
        let size = e.mem.u32(map + 4).wrapping_mul(4);
        e.mem.set_u32(map + 8, buckets);
        e.call(MEMSET, &args![buckets, 0u32, size]);
        e.mem.set_u32(map, NI_T_POINTER_MAP_VTABLE);
        let mut touched = WordSet {
            array: 0,
            capacity: 0,
            count: 0,
        };
        e.call(
            GEOMETRY_SORTED_VERTICES,
            &args![geometry, sorted_at, positions_at, 1u32],
        );
        let mut triangle = 0u32;
        while triangle < triangles {
            e.vcall(
                geometry,
                GET_TRIANGLE_SLOT,
                &args![triangle, corners, corners + 2, corners + 4],
            );
            let (c0, c1, c2) = (
                e.mem.u16(corners),
                e.mem.u16(corners + 2),
                e.mem.u16(corners + 4),
            );
            if c0 == c1 || c1 == c2 || c2 == c0 {
                triangle += 1;
                continue;
            }
            fn_00a82530(e, this, Ptr::new(map), Ptr::new(corners), Ptr::new(lists));
            let allowed = u32::from(most);
            let mut remaining = e.mem.u32(map + 0xc);
            if remaining > allowed {
                loop {
                    // The first item: the first non-empty bucket's head (0 when none).
                    let mut item = 0u32;
                    let mut bucket = 0u32;
                    while bucket < e.mem.u32(map + 4) {
                        let head = e.mem.u32(e.mem.u32(map + 8).wrapping_add(bucket * 4));
                        if head != 0 {
                            item = head;
                            break;
                        }
                        bucket += 1;
                    }
                    let mut minimum = x87_store32(x87_load32(e.mem.u32(FLOAT_MAXIMUM)));
                    let mut minimum_bone = 0u32;
                    loop {
                        let value = x87_quiet_bits(e.mem.u32(item + 8));
                        let bone = e.mem.u32(item + 4);
                        let mut next = e.mem.u32(item);
                        if next == 0 {
                            let hash = e.vcall(map, MAP_KEY_TO_HASH_INDEX_SLOT, &args![bone]).u32();
                            let mut bucket = hash.wrapping_add(1);
                            while bucket < e.mem.u32(map + 4) {
                                let head = e.mem.u32(e.mem.u32(map + 8).wrapping_add(bucket * 4));
                                if head != 0 {
                                    next = head;
                                    break;
                                }
                                bucket += 1;
                            }
                        }
                        // `FLD value; FLD minimum; FCOMP; TEST AH,0x41`: replaced only when
                        // the minimum is above the value.
                        if x87_load32(minimum) > x87_load32(value) {
                            minimum = value;
                            minimum_bone = bone;
                        }
                        item = next;
                        if item == 0 {
                            break;
                        }
                    }
                    for corner in 0..3u32 {
                        let vertex = u16_at(e, corners + 2 * corner);
                        if touched.capacity == 0 {
                            let array = e.call(NI_ALLOC, &args![4u32]).u32();
                            e.call(NI_FREE, &args![touched.array]);
                            touched.array = array;
                            touched.capacity = 1;
                        }
                        e.mem.set_u32(touched.array, vertex);
                        let list = lists.wrapping_add(vertex.wrapping_mul(12));
                        let positions = e.mem.u32(positions_at);
                        let mut position = u16_at(e, positions.wrapping_add(vertex * 2));
                        touched.count = 1;
                        let mut back = position;
                        while back != 0 {
                            let sorted = e.mem.u32(sorted_at);
                            let other = u16_at(e, sorted.wrapping_add(back * 2 - 2));
                            if e.call(
                                GEOMETRY_COMPARE_VERTICES,
                                &args![geometry, other, vertex, 1u32],
                            )
                            .u32()
                                != 0
                            {
                                break;
                            }
                            let other_list = lists.wrapping_add(other.wrapping_mul(12));
                            if same_bones(e, list, other_list) {
                                touched.add(e, other);
                            }
                            back -= 1;
                        }
                        while position < vertex_count.wrapping_sub(1) {
                            let sorted = e.mem.u32(sorted_at);
                            let other = u16_at(e, sorted.wrapping_add(position * 2 + 2));
                            if e.call(
                                GEOMETRY_COMPARE_VERTICES,
                                &args![geometry, other, vertex, 1u32],
                            )
                            .u32()
                                != 0
                            {
                                break;
                            }
                            let other_list = lists.wrapping_add(other.wrapping_mul(12));
                            if same_bones(e, list, other_list) {
                                touched.add(e, other);
                            }
                            position += 1;
                        }
                        let mut index = 0u32;
                        while index < touched.count {
                            let vertex = e.mem.u32(touched.array.wrapping_add(index * 4));
                            let list = lists.wrapping_add(vertex.wrapping_mul(12));
                            let count = e.mem.u32(list + 8);
                            let pairs = e.mem.u32(list);
                            let found = (0..count).find(|&slot| {
                                e.mem.u32(pairs.wrapping_add(slot * 8)) == minimum_bone
                            });
                            if let Some(slot) = found {
                                let mut at = slot;
                                while at < e.mem.u32(list + 8).wrapping_sub(1) {
                                    let pairs = e.mem.u32(list);
                                    let pair = pairs.wrapping_add(at * 8);
                                    let next_bone = e.mem.u32(pair + 8);
                                    e.mem.set_u32(pair, next_bone);
                                    let next_weight = e.mem.u32(pair + 0xc);
                                    e.mem.set_u32(pair + 4, next_weight);
                                    at += 1;
                                }
                                let left = e.mem.u32(list + 8).wrapping_sub(1);
                                e.mem.set_u32(list + 8, left);
                                if left == 0 {
                                    let sorted = e.mem.u32(sorted_at);
                                    let positions = e.mem.u32(positions_at);
                                    finish_00a833b0(e, map, sorted, positions, touched.array);
                                    return false;
                                }
                                fn_00a820e0(e, Ptr::new(list));
                            }
                            index += 1;
                        }
                    }
                    remaining = remaining.wrapping_sub(1);
                    if remaining == allowed {
                        break;
                    }
                    e.call(MAP_REMOVE_ALL, &args![map]);
                    fn_00a82530(e, this, Ptr::new(map), Ptr::new(corners), Ptr::new(lists));
                }
            }
            e.call(MAP_REMOVE_ALL, &args![map]);
            triangle += 1;
        }
        let limit = u32::from(keep);
        for vertex in 0..vertex_count {
            let list = lists.wrapping_add(vertex * 12);
            let count = e.mem.u32(list + 8);
            if count > limit {
                let pairs = e.mem.u32(list);
                e.call(QSORT, &args![pairs, count, 8u32, COMPARE_INFLUENCES]);
                e.mem.set_u32(list + 8, limit);
                fn_00a820e0(e, Ptr::new(list));
            }
        }
        let sorted = e.mem.u32(sorted_at);
        let positions = e.mem.u32(positions_at);
        finish_00a833b0(e, map, sorted, positions, touched.array);
        true
    })
}

/// `NiTSet<NiNode *, NiTMallocInterface<NiNode *>>::Realloc` (`00a311e0`, engine map name of
/// the folded body in `nibsbonelodcontroller.obj`; `__thiscall`, the new size): the `Realloc`
/// of a set of words.
const WORD_SET_REALLOC: u32 = 0x00a3_11e0;

/// Adds `bone` to a sorted-later bone set (array +0, capacity +4, count +8) unless the set
/// already holds it (a linear search of its count).
fn add_bone(e: &mut Engine, set: u32, bone: u32) {
    let count = e.mem.u32(set + 8);
    let array = e.mem.u32(set);
    if (0..count).any(|i| u16_at(e, array.wrapping_add(i * 2)) == bone) {
        return;
    }
    set_add_u16(e, set, bone as u16);
}

/// The bones of a triangle's three vertices (their influence lists) added to a bone set,
/// which is then sorted (`qsort` with `00a81b80`).
fn add_triangle_bones(e: &mut Engine, set: u32, corners: u32, lists: u32) {
    for corner in 0..3u32 {
        let vertex = u16_at(e, corners + 2 * corner);
        let list = lists.wrapping_add(vertex.wrapping_mul(12));
        let mut index = 0u32;
        while index < e.mem.u32(list + 8) {
            let bone = u16_at(e, e.mem.u32(list).wrapping_add(index * 8));
            add_bone(e, set, bone);
            index += 1;
        }
    }
    let array = e.mem.u32(set);
    let count = e.mem.u32(set + 8);
    e.call(QSORT, &args![array, count, 2u32, COMPARE_U16]);
}

/// A new empty bone set: 0x10 bytes from the `MemoryManager` (`00aa3e40` on `011f6238`), its
/// first three words cleared; 0 when the allocation fails.
fn new_bone_set(e: &mut Engine) -> u32 {
    let set = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    if set != 0 {
        for offset in [0u32, 4, 8] {
            e.mem.set_u32(set + offset, 0);
        }
    }
    set
}

// Translated from 00a83d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall`, `this` unread: the geometry data, the per-vertex
/// influence lists of 12 bytes, the set of bone sets it fills (array +0, capacity +4, count
/// +8), an optional array of one word per vertex). Makes the bone sets of the triangles (the
/// data's triangle count at +0x40, each triangle through its virtual +0xac).
///
/// With the word array: for each corner of each triangle, every set whose word at +0xc is the
/// corner vertex's word gets the bones of the whole triangle (each one once, then sorted with
/// `qsort` and `00a81b80`); when no set has that word a new set (0x10 bytes from the
/// `MemoryManager`) with it gets them and is added (the set of sets doubling inline, from 1).
///
/// Without it: each triangle with three different vertices makes a new set of its bones. A
/// set already holding it (`00a82190`) makes it go (freed); sets it holds are deleted
/// (moved over by the last one); when none held it, it is added (`00a311e0` doubling, from 1).
// Translation incomplete: 00a83d40 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a83d40(e: &mut Engine, _unused_0: Ptr, data: Ptr, lists: Ptr, sets: Ptr, words: Ptr) {
    let geometry = data.addr();
    let (lists, out) = (lists.addr(), sets.addr());
    e.with_stack(8, |e, corners| {
        let corners = corners.addr();
        let triangles = u16_at(e, geometry + 0x40);
        if !words.is_null() {
            let mut triangle = 0u32;
            while triangle < triangles {
                e.vcall(
                    geometry,
                    GET_TRIANGLE_SLOT,
                    &args![triangle, corners, corners + 2, corners + 4],
                );
                let word_of = |e: &Engine, corner: u32| {
                    let vertex = u16_at(e, corners + 2 * corner);
                    e.mem.u32(words.addr().wrapping_add(vertex * 4))
                };
                let tags = [word_of(e, 0), word_of(e, 1), word_of(e, 2)];
                for tag in tags {
                    let mut found = false;
                    let mut index = 0u32;
                    while index < e.mem.u32(out + 8) {
                        let set = e.mem.u32(e.mem.u32(out).wrapping_add(index * 4));
                        if e.mem.u32(set + 0xc) == tag {
                            found = true;
                            add_triangle_bones(e, set, corners, lists);
                        }
                        index += 1;
                    }
                    if found {
                        continue;
                    }
                    let set = new_bone_set(e);
                    e.mem.set_u32(set + 0xc, tag);
                    add_triangle_bones(e, set, corners, lists);
                    let capacity = e.mem.u32(out + 4);
                    if e.mem.u32(out + 8) == capacity {
                        let size = if capacity != 0 { capacity * 2 } else { 1 };
                        set_realloc(e, out, size, 4);
                    }
                    let count = e.mem.u32(out + 8);
                    e.mem.set_u32(e.mem.u32(out).wrapping_add(count * 4), set);
                    e.mem.set_u32(out + 8, count.wrapping_add(1));
                }
                triangle += 1;
                if triangle >= u16_at(e, geometry + 0x40) {
                    break;
                }
            }
            return;
        }
        let mut triangle = 0u32;
        while triangle < triangles {
            e.vcall(
                geometry,
                GET_TRIANGLE_SLOT,
                &args![triangle, corners, corners + 2, corners + 4],
            );
            let (c0, c1, c2) = (
                e.mem.u16(corners),
                e.mem.u16(corners + 2),
                e.mem.u16(corners + 4),
            );
            if c0 != c1 && c1 != c2 && c2 != c0 {
                let set = new_bone_set(e);
                for corner in 0..3u32 {
                    let vertex = u16_at(e, corners + 2 * corner);
                    let list = lists.wrapping_add(vertex.wrapping_mul(12));
                    let count = e.mem.u32(list + 8);
                    for index in 0..count {
                        let bone = u16_at(e, e.mem.u32(list).wrapping_add(index * 8));
                        add_bone(e, set, bone);
                    }
                }
                let array = e.mem.u32(set);
                let count = e.mem.u32(set + 8);
                e.call(QSORT, &args![array, count, 2u32, COMPARE_U16]);
                let mut index = 0u32;
                while index < e.mem.u32(out + 8) {
                    let other = e.mem.u32(e.mem.u32(out).wrapping_add(index * 4));
                    if fn_00a82190(e, Ptr::new(other), Ptr::new(set)) {
                        delete_bone_set(e, set);
                        break;
                    }
                    if fn_00a82190(e, Ptr::new(set), Ptr::new(other)) {
                        delete_bone_set(e, other);
                        set_remove_word(e, out, index);
                    } else {
                        index += 1;
                    }
                }
                let count = e.mem.u32(out + 8);
                if index == count {
                    let capacity = e.mem.u32(out + 4);
                    if count == capacity {
                        let size = if capacity != 0 { capacity * 2 } else { 1 };
                        e.call(WORD_SET_REALLOC, &args![out, size]);
                    }
                    let count = e.mem.u32(out + 8);
                    e.mem.set_u32(e.mem.u32(out).wrapping_add(count * 4), set);
                    e.mem.set_u32(out + 8, count.wrapping_add(1));
                }
            }
            triangle += 1;
        }
    });
}

/// The vtables of `NiTMap<int, int>` (`0101ee00`) and its base
/// `NiTMapBase<DFALL<NiTMapItem<int, int>>, int, int>` (`0101ee20`) (their RTTI names), and the
/// unnamed functions `00a84310` runs on one: `00b57ea0` (`__thiscall`: key, value; the
/// `SetAt` shape), `00a53c30` (`__thiscall`: key, address of the value; the `GetAt` shape) and
/// `00438af0` (`__thiscall`, no words; the `RemoveAll` shape).
const INT_MAP_VTABLE: u32 = 0x0101_ee00;
const INT_MAP_BASE_VTABLE: u32 = 0x0101_ee20;
const INT_MAP_SET_AT: u32 = 0x00b5_7ea0;
const INT_MAP_GET_AT: u32 = 0x00a5_3c30;
const INT_MAP_REMOVE_ALL: u32 = 0x0043_8af0;

/// The stack `NiTMap<int, int>` of `00a84310` torn down as its destructor does.
fn destroy_int_map(e: &mut Engine, map: u32) {
    e.mem.set_u32(map, INT_MAP_VTABLE);
    e.call(INT_MAP_REMOVE_ALL, &args![map]);
    e.mem.set_u32(map, INT_MAP_BASE_VTABLE);
    e.call(INT_MAP_REMOVE_ALL, &args![map]);
    let buckets = e.mem.u32(map + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00a84310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall` on the `NiSkinPartition`, seven words: the
/// geometry data, an unread word, the most bones a partition may have (a byte), the
/// per-vertex influence lists, the set of bone sets, the address of the triangle-to-partition
/// array pointer, an optional array of one word per vertex). Makes the bone sets
/// (`00a83d40`) and merges them (`00a839d0`), then gives each triangle its partition in a new
/// array (`00aa1070`, one word per triangle) stored through the sixth word, with a byte per
/// set (`00aa1070`).
///
/// With the word array: a stack `NiTMap<int, int>` (37 buckets) maps each set's word (+0xc)
/// to its index; a triangle whose three vertex words agree takes that word's set
/// (`00a53c30`), otherwise the set of the smallest of its words (or of the largest when that
/// is above 100), looked up inline; a set of -1 leaves the triangle's entry. A triangle whose
/// three words all differ ends the function with false (the map torn down; the byte array is
/// not freed and the partition count is left).
///
/// Without it: the set bytes are cleared; a triangle with a repeated vertex gets -1, any other
/// the first set with byte 1 that holds its bones (`00a82c90`), else the first with byte 0.
/// Then every set whose byte is still 0 is removed (moved over by the last one, whose
/// triangles are renumbered; the bytes are not moved).
///
/// The partition count (+8) becomes the number of sets, the byte array is freed and the answer
/// is true.
// Translation incomplete: 00a84310 - the /GS stack cookie check (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a84310(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    _unused_2: u32,
    most: u8,
    lists: Ptr,
    sets: Ptr,
    partitions_at: Ptr,
    words: Ptr,
) -> bool {
    let geometry = data.addr();
    let s = sets.addr();
    let triangles = u16_at(e, geometry + 0x40);
    fn_00a83d40(e, this, data, lists, sets, words);
    fn_00a839d0(e, this, most, sets);
    let partitions = e.call(NI_ALLOC, &args![triangles * 4]).u32();
    e.mem.set_u32(partitions_at.addr(), partitions);
    let used = e.call(NI_ALLOC, &args![e.mem.u32(s + 8)]).u32();
    let partition_of = |e: &mut Engine, triangle: u32, value: u32| {
        let array = e.mem.u32(partitions_at.addr());
        e.mem.set_u32(array.wrapping_add(triangle * 4), value);
    };
    // The map (0x10), the found word and the triangle's three 16-bit corners (6 bytes; the
    // exe pushes ESP+0x3c, +0x3e and +0x40 of its `SUB ESP,0x38` frame), so 0x1a bytes.
    let ok = e.with_stack(0x1c, |e, frame| {
        let map = frame.addr();
        let found = map + 0x10;
        let corners = map + 0x14;
        let word_of = |e: &Engine, corner: u32| {
            let vertex = u16_at(e, corners + 2 * corner);
            e.mem.u32(words.addr().wrapping_add(vertex * 4))
        };
        if !words.is_null() {
            e.mem.set_u32(map, INT_MAP_BASE_VTABLE);
            e.mem.set_u32(map + 4, 0x25);
            e.mem.set_u32(map + 0xc, 0);
            let buckets = e.call(NI_ALLOC, &args![0x94u32]).u32();
            let size = e.mem.u32(map + 4).wrapping_mul(4);
            e.mem.set_u32(map + 8, buckets);
            e.call(MEMSET, &args![buckets, 0u32, size]);
            e.mem.set_u32(map, INT_MAP_VTABLE);
            let mut index = 0u32;
            while index < e.mem.u32(s + 8) {
                let set = e.mem.u32(e.mem.u32(s).wrapping_add(index * 4));
                let tag = e.mem.u32(set + 0xc);
                e.call(INT_MAP_SET_AT, &args![map, tag, index]);
                index += 1;
            }
            let mut triangle = 0u32;
            while triangle < triangles {
                e.vcall(
                    geometry,
                    GET_TRIANGLE_SLOT,
                    &args![triangle, corners, corners + 2, corners + 4],
                );
                e.mem.set_u32(found, 0xffff_ffff);
                let (w1, w0) = (word_of(e, 1), word_of(e, 0));
                let w2 = word_of(e, 2);
                if w0 == w1 && w1 == w2 && w0 == w2 {
                    e.call(INT_MAP_GET_AT, &args![map, w0, found]);
                    let value = e.mem.u32(found);
                    if value != 0xffff_ffff {
                        partition_of(e, triangle, value);
                    }
                } else {
                    let low = if (w1 as i32) < (w2 as i32) { w1 } else { w2 };
                    let mut key = if (w0 as i32) < (low as i32) { w0 } else { low };
                    let high = if (w1 as i32) > (w2 as i32) { w1 } else { w2 };
                    let high = if (w0 as i32) > (high as i32) {
                        w0
                    } else {
                        high
                    };
                    if (high as i32) > 100 {
                        key = high;
                    }
                    let bucket = e.vcall(map, MAP_KEY_TO_HASH_INDEX_SLOT, &args![key]).u32();
                    let mut item = e.mem.u32(e.mem.u32(map + 8).wrapping_add(bucket * 4));
                    while item != 0 {
                        let item_key = e.mem.u32(item + 4);
                        if e.vcall(map, MAP_IS_KEYS_EQUAL_SLOT, &args![key, item_key])
                            .bool()
                        {
                            let value = e.mem.u32(item + 8);
                            if value != 0xffff_ffff {
                                partition_of(e, triangle, value);
                            }
                            break;
                        }
                        item = e.mem.u32(item);
                    }
                }
                let (w1, w0) = (word_of(e, 1), word_of(e, 0));
                if w0 != w1 {
                    let w2 = word_of(e, 2);
                    if w1 != w2 && w0 != w2 {
                        destroy_int_map(e, map);
                        return false;
                    }
                }
                triangle += 1;
            }
            destroy_int_map(e, map);
            return true;
        }
        let mut index = 0u32;
        while index < e.mem.u32(s + 8) {
            e.mem.set_u8(used + index, 0);
            index += 1;
        }
        for triangle in 0..triangles {
            e.vcall(
                geometry,
                GET_TRIANGLE_SLOT,
                &args![triangle, corners, corners + 2, corners + 4],
            );
            let (c0, c1, c2) = (
                e.mem.u16(corners),
                e.mem.u16(corners + 2),
                e.mem.u16(corners + 4),
            );
            if c0 == c1 || c1 == c2 || c2 == c0 {
                partition_of(e, triangle, 0xffff_ffff);
                continue;
            }
            let mut set = fn_00a82c90(e, sets, Ptr::new(corners), lists, Ptr::new(used), 1);
            if set < 0 {
                set = fn_00a82c90(e, sets, Ptr::new(corners), lists, Ptr::new(used), 0);
            }
            partition_of(e, triangle, set as u32);
        }
        let mut index = 0u32;
        while index < e.mem.u32(s + 8) {
            if e.mem.u8(used + index) != 0 {
                index += 1;
                continue;
            }
            set_remove_word(e, s, index);
            let last = e.mem.u32(s + 8);
            for triangle in 0..triangles {
                let array = e.mem.u32(partitions_at.addr());
                let at = array.wrapping_add(triangle * 4);
                if e.mem.u32(at) == last {
                    e.mem.set_u32(at, index);
                }
            }
        }
        true
    });
    if !ok {
        return false;
    }
    let count = e.mem.u32(s + 8);
    e.mem.set_u32(this.addr() + PARTITIONS, count);
    e.call(NI_FREE, &args![used]);
    true
}

// Translated from 00a833a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; translated from the disassembled bytes): clears the
/// three words of a set (array, capacity, count) and answers it.
pub fn fn_00a833a0(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0u32, 4, 8] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a820d0, ni_skin_partition_get_rtti() -> Ptr),
        entry!(0x00a82c70, ni_skin_partition_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a81fb0, ni_skin_partition_create_object() -> Ptr),
        entry!(0x00a81fe0, fn_00a81fe0(Ptr, Ptr) -> u32),
        entry!(0x00a824d0, fn_00a824d0(Ptr)),
        entry!(0x00a81c20, fn_00a81c20(Ptr) -> u32),
        entry!(0x00a82d90, fn_00a82d90(Ptr, Ptr)),
        entry!(0x00a830d0, fn_00a830d0(Ptr, Ptr)),
        entry!(0x00a83c30, ni_skin_partition_load_binary(Ptr, Ptr)),
        entry!(0x00a83ce0, ni_skin_partition_save_binary(Ptr, Ptr)),
        entry!(0x00a81c90, ni_skin_partition_partition_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a81f90, fn_00a81f90(Ptr) -> Ptr),
        entry!(0x00a81ff0, ni_skin_partition_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a82050, fn_00a82050(Ptr, u32, f32)),
        entry!(0x00a820e0, fn_00a820e0(Ptr)),
        entry!(0x00a82190, fn_00a82190(Ptr, Ptr) -> bool),
        entry!(0x00a821f0, fn_00a821f0(Ptr, Ptr) -> bool),
        entry!(0x00a82250, fn_00a82250(Ptr) -> u16),
        entry!(0x00a822d0, fn_00a822d0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00a82400, fn_00a82400(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00a82530, fn_00a82530(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00a82600, ni_t_map_base_set_value(Ptr, Ptr, u32, f32)),
        entry!(0x00a82620, ni_t_pointer_map_delete_item(Ptr, Ptr)),
        entry!(0x00a82670, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a826b0, fn_00a826b0(Ptr, u32)),
        entry!(0x00a82710, fn_00a82710(Ptr, u32)),
        entry!(0x00a82770, fn_00a82770(Ptr, Ptr) -> u32),
        entry!(0x00a827f0, fn_00a827f0(Ptr, Ptr)),
        entry!(0x00a82890, fn_00a82890(Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00a828d0,
            fn_00a828d0(Ptr, Ptr, Ptr, Ptr, Ptr, u8, u8, u8, u32)
        ),
        entry!(0x00a82c90, fn_00a82c90(Ptr, Ptr, Ptr, Ptr, u8) -> i32),
        entry!(0x00a82cf0, ni_t_pointer_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a82d30, fn_00a82d30(Ptr, u32, Ptr, u32)),
        entry!(0x00a833a0, fn_00a833a0(Ptr) -> Ptr),
        entry!(0x00a833b0, fn_00a833b0(Ptr, Ptr, u8, u8, Ptr) -> bool),
        entry!(0x00a839d0, fn_00a839d0(Ptr, u8, Ptr)),
        entry!(0x00a83d40, fn_00a83d40(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00a84310,
            fn_00a84310(Ptr, Ptr, u32, u8, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00a846e0, ni_skin_partition_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_skin_partition_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a820d0, &args![]).u32(), NI_SKIN_PARTITION_RTTI);
    }

    #[test]
    fn ni_skin_partition_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a8_24d0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a82c70, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a82c70, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00a8_24d0), vec![vec![this]; 2]);
    }

    fn calls_to_partition(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing_partition(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    type Bytes = std::rc::Rc<std::cell::RefCell<Vec<u8>>>;

    /// A stream whose input reads from `bytes` and whose output appends to
    /// the answered buffer.
    fn stream_with(e: &mut Engine, bytes: Vec<u8>, version: u32) -> (u32, Bytes) {
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, 0x0640_0000);
        let source = std::rc::Rc::new(std::cell::RefCell::new(bytes));
        e.register_double(0x0640_0000, move |e, a| {
            let taken: Vec<u8> = source.borrow_mut().drain(..a[2] as usize).collect();
            e.mem.write(a[1], &taken);
            a[2].into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x0640_0010);
        let written: Bytes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(0x0640_0010, move |e, a| {
            sink.borrow_mut().extend(e.mem.bytes(a[1], a[2]));
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        (stream, written)
    }

    #[test]
    fn create_object_and_register_streamables() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing_partition);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0x101u32.into_ret());
        let object = e.call(0x00a8_1fb0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_SKIN_PARTITION_VTABLE);
        assert_eq!(e.mem.u32(object + PARTITION_ARRAY), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_1fe0, &args![object, 0x55u32]).u32(), 0x101);
        assert_eq!(
            calls_to_partition(&e, NI_OBJECT_REGISTER_STREAMABLES),
            vec![vec![object, 0x55]]
        );
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_1fb0, &args![]).u32(), 0);
    }

    #[test]
    fn destructor_body_releases_the_partitions() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT, nothing_partition);
        e.register(MEMORY_MANAGER_DEALLOCATE, nothing_partition);
        e.register(0x0640_0100, nothing_partition);
        e.register(0x0640_0200, nothing_partition);
        e.map(RENDERER, 4);
        let this = e.mem.alloc(0x10);
        let block = e.mem.alloc(4 + 0x2c);
        e.mem.set_u32(this + PARTITION_ARRAY, block + 4);
        e.call_log = Some(vec![]);
        e.call(0x00a8_24d0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_SKIN_PARTITION_VTABLE);
        assert_eq!(
            calls_to_partition(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls_to_partition(&e, NI_OBJECT_DESTRUCT), vec![vec![this]]);
        // With a count and a renderer: the vector deleting destructor.
        let renderer_vtable = e.mem.alloc(0x104);
        e.mem.set_u32(renderer_vtable + 0x100, 0x0640_0100);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, renderer_vtable);
        e.set_global(RENDERER, renderer);
        let part_vtable = e.mem.alloc(4);
        e.mem.set_u32(part_vtable, 0x0640_0200);
        e.mem.set_u32(block, 1);
        e.mem.set_u32(block + 4, part_vtable);
        e.call(0x00a8_24d0, &args![this]);
        assert_eq!(
            calls_to_partition(&e, 0x0640_0100),
            vec![vec![renderer, this]]
        );
        assert_eq!(
            calls_to_partition(&e, 0x0640_0200),
            vec![vec![block + 4, 3]]
        );
        assert_eq!(calls_to_partition(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
        assert_eq!(calls_to_partition(&e, NI_OBJECT_DESTRUCT).len(), 2);
    }

    #[test]
    fn strip_lengths_sum() {
        let mut e = Engine::new();
        let part = e.mem.alloc(0x2c);
        let lengths = e.mem.alloc(10);
        for (i, v) in [3u16, 4, 5, 6, 7].iter().enumerate() {
            e.mem.set_u16(lengths + 2 * i as u32, *v);
        }
        e.mem.set_u32(part + PART_STRIP_LENGTHS, lengths);
        for (strips, sum) in [(0u16, 0u32), (1, 3), (2, 7), (3, 12), (5, 25)] {
            e.mem.set_u16(part + PART_STRIPS, strips);
            assert_eq!(e.call(0x00a8_1c20, &args![part]).u32(), sum);
        }
    }

    fn partition_bytes(with_flags: bool) -> Vec<u8> {
        let mut bytes = Vec::new();
        // 2 vertices, 1 triangle, 1 bone, 0 strips, 1 bone per vertex.
        for count in [2u16, 1, 1, 0, 1] {
            bytes.extend_from_slice(&count.to_le_bytes());
        }
        bytes.extend_from_slice(&9u16.to_le_bytes());
        if with_flags {
            bytes.push(1);
        }
        bytes.extend_from_slice(&[5, 0, 6, 0]);
        if with_flags {
            bytes.push(1);
        }
        bytes.extend_from_slice(&1.0f32.to_le_bytes());
        bytes.extend_from_slice(&0.5f32.to_le_bytes());
        if with_flags {
            bytes.push(1);
        }
        bytes.extend_from_slice(&[0, 0, 1, 0, 1, 0]);
        bytes.push(1);
        bytes.extend_from_slice(&[0, 0]);
        bytes
    }

    #[test]
    fn partition_load_binary_reads_every_array() {
        for (version, with_flags) in [(VERSION_10_0_1_18, true), (0x0a00_0111, false)] {
            let mut e = Engine::new();
            e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
            let (stream, _) = stream_with(&mut e, partition_bytes(with_flags), version);
            let part = e.mem.alloc(0x2c);
            e.call(0x00a8_2d90, &args![part, stream]);
            assert_eq!(e.mem.u16(part + PART_VERTICES), 2);
            assert_eq!(e.mem.u16(e.mem.u32(part + PART_BONES)), 9);
            assert_eq!(e.mem.u16(e.mem.u32(part + PART_VERTEX_MAP) + 2), 6);
            assert_eq!(e.mem.f32(e.mem.u32(part + PART_WEIGHTS) + 4), 0.5);
            assert_eq!(e.mem.u16(e.mem.u32(part + PART_TRI_LIST) + 4), 1);
            assert_ne!(e.mem.u32(part + PART_BONE_PALETTE), 0);
            assert_eq!(e.mem.u32(part + PART_STRIP_LENGTHS), 0);
            let input = e.mem.u32(stream + STREAM_INPUT);
            assert_eq!(
                e.mem.u32(input + 4) as usize,
                partition_bytes(with_flags).len()
            );
        }
    }

    #[test]
    fn partition_load_binary_with_strips_and_no_arrays() {
        let mut e = Engine::new();
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        let mut bytes = Vec::new();
        for count in [3u16, 0, 0, 2, 0] {
            bytes.extend_from_slice(&count.to_le_bytes());
        }
        bytes.extend_from_slice(&[0, 0]);
        bytes.extend_from_slice(&[3, 0, 1, 0]);
        bytes.extend_from_slice(&[1, 7, 0, 8, 0, 9, 0, 1, 0]);
        bytes.push(0);
        let (stream, _) = stream_with(&mut e, bytes, VERSION_10_0_1_18);
        let part = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        e.call(0x00a8_2d90, &args![part, stream]);
        assert_eq!(e.mem.u32(part + PART_VERTEX_MAP), 0);
        assert_eq!(e.mem.u32(part + PART_WEIGHTS), 0);
        let lengths = e.mem.u32(part + PART_STRIP_LENGTHS);
        assert_eq!(e.mem.u16(lengths), 3);
        assert_eq!(e.mem.u16(e.mem.u32(part + PART_TRI_LIST) + 6), 1);
        assert_eq!(e.mem.u32(part + PART_BONE_PALETTE), 0);
        let allocated: Vec<u32> = calls_to_partition(&e, NI_ALLOC)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(allocated, vec![0, 4, 8]);
    }

    #[test]
    fn partition_save_binary_writes_flags_and_arrays() {
        let mut e = Engine::new();
        let (stream, written) = stream_with(&mut e, vec![], VERSION_10_0_1_18);
        let part = e.mem.alloc(0x2c);
        for (offset, count) in [
            (PART_VERTICES, 2u16),
            (PART_TRIANGLES, 1),
            (PART_BONE_COUNT, 1),
            (PART_STRIPS, 0),
            (PART_BONES_PER_VERTEX, 1),
        ] {
            e.mem.set_u16(part + offset, count);
        }
        let bones = e.mem.alloc(2);
        e.mem.set_u16(bones, 9);
        e.mem.set_u32(part + PART_BONES, bones);
        let weights = e.mem.alloc(8);
        e.mem.set_f32(weights, 1.0);
        e.mem.set_f32(weights + 4, 0.5);
        e.mem.set_u32(part + PART_WEIGHTS, weights);
        let list = e.mem.alloc(6);
        e.mem.set_u16(list + 4, 1);
        e.mem.set_u32(part + PART_TRI_LIST, list);
        e.call(0x00a8_30d0, &args![part, stream]);
        let mut expected = Vec::new();
        for count in [2u16, 1, 1, 0, 1] {
            expected.extend_from_slice(&count.to_le_bytes());
        }
        expected.extend_from_slice(&9u16.to_le_bytes());
        expected.push(0);
        expected.push(1);
        expected.extend_from_slice(&1.0f32.to_le_bytes());
        expected.extend_from_slice(&0.5f32.to_le_bytes());
        expected.push(1);
        expected.extend_from_slice(&[0, 0, 0, 0, 1, 0]);
        expected.push(0);
        assert_eq!(*written.borrow(), expected);
    }

    #[test]
    fn skin_partition_load_and_save() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, nothing_partition);
        e.register(NI_OBJECT_SAVE_BINARY, nothing_partition);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR, nothing_partition);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        let mut bytes = vec![1, 0, 0, 0];
        bytes.extend(partition_bytes(true));
        let (stream, written) = stream_with(&mut e, bytes, VERSION_10_0_1_18);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00a8_3c30, &args![this, stream]);
        let parts = e.mem.u32(this + PARTITION_ARRAY);
        assert_eq!(e.mem.u32(parts - 4), 1);
        assert_eq!(
            calls_to_partition(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x30]]
        );
        assert_eq!(
            calls_to_partition(&e, VECTOR_CONSTRUCTOR),
            vec![vec![parts, 0x2c, 1, PARTITION_CONSTRUCT]]
        );
        assert_eq!(e.mem.u16(parts + PART_VERTICES), 2);
        e.call(0x00a8_3ce0, &args![this, stream]);
        assert_eq!(
            calls_to_partition(&e, NI_OBJECT_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        let written = written.borrow();
        assert_eq!(written[..4], [1, 0, 0, 0]);
        assert_eq!(written.len(), 4 + partition_bytes(true).len());
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const HASH_DOUBLE: u32 = 0x0070_c000;
    const EQUAL_DOUBLE: u32 = 0x0070_c010;
    const NEW_ITEM_DOUBLE: u32 = 0x0070_c020;
    const TRIANGLE_DOUBLE: u32 = 0x0070_c030;

    /// The allocators and the CRT helpers as plain doubles: `NiAlloc` and the
    /// `MemoryManager` allocate (at least one byte), the frees do nothing, `qsort` sorts
    /// `u16` arrays upwards and influence pairs by weight downwards, `bsearch` finds a `u16`.
    fn heap(e: &mut Engine) {
        e.register(NI_ALLOC, |e, a| word(e.mem.alloc(a[0].max(1))));
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            word(e.mem.alloc(a[1].max(1)))
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(QSORT, |e, a| {
            let (base, count, width) = (a[0], a[1], a[2]);
            if width == 2 {
                let mut values: Vec<u16> = (0..count).map(|i| e.mem.u16(base + 2 * i)).collect();
                values.sort_unstable();
                for (i, v) in values.iter().enumerate() {
                    e.mem.set_u16(base + 2 * i as u32, *v);
                }
            } else {
                let mut pairs: Vec<(u32, u32)> = (0..count)
                    .map(|i| (e.mem.u32(base + 8 * i), e.mem.u32(base + 8 * i + 4)))
                    .collect();
                pairs.sort_by(|x, y| {
                    f32::from_bits(y.1)
                        .partial_cmp(&f32::from_bits(x.1))
                        .unwrap()
                });
                for (i, (bone, weight)) in pairs.iter().enumerate() {
                    e.mem.set_u32(base + 8 * i as u32, *bone);
                    e.mem.set_u32(base + 8 * i as u32 + 4, *weight);
                }
            }
            Ret::default()
        });
        e.register(BSEARCH, |e, a| {
            let key = e.mem.u16(a[0]);
            let found = (0..a[2]).find(|i| e.mem.u16(a[1] + 2 * i) == key);
            word(found.map_or(0, |i| a[1] + 2 * i))
        });
    }

    /// A `u16` set (array, capacity, count, then a fourth word).
    fn set(e: &mut Engine, values: &[u16]) -> u32 {
        let set = e.mem.alloc(0x10);
        let array = e.mem.alloc(2 * values.len() as u32 + 2);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u16(array + 2 * i as u32, *v);
        }
        e.mem.set_u32(set, array);
        e.mem.set_u32(set + 4, values.len() as u32);
        e.mem.set_u32(set + 8, values.len() as u32);
        set
    }

    fn set_values(e: &Engine, set: u32) -> Vec<u16> {
        let array = e.mem.u32(set);
        (0..e.mem.u32(set + 8))
            .map(|i| e.mem.u16(array + 2 * i))
            .collect()
    }

    /// A set of words (array, capacity, count).
    fn words(e: &mut Engine, values: &[u32]) -> u32 {
        let set = e.mem.alloc(0xc);
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *v);
        }
        e.mem.set_u32(set, array);
        e.mem.set_u32(set + 4, values.len() as u32);
        e.mem.set_u32(set + 8, values.len() as u32);
        set
    }

    fn word_values(e: &Engine, set: u32) -> Vec<u32> {
        let array = e.mem.u32(set);
        (0..e.mem.u32(set + 8))
            .map(|i| e.mem.u32(array + 4 * i))
            .collect()
    }

    /// Influence lists, 12 bytes per vertex: (bone, weight) pairs, their count.
    fn lists(e: &mut Engine, vertices: &[&[(u32, f32)]]) -> u32 {
        let lists = e.mem.alloc(12 * vertices.len() as u32);
        for (v, pairs) in vertices.iter().enumerate() {
            let array = e.mem.alloc(8 * pairs.len() as u32 + 8);
            for (i, (bone, weight)) in pairs.iter().enumerate() {
                e.mem.set_u32(array + 8 * i as u32, *bone);
                e.mem.set_f32(array + 8 * i as u32 + 4, *weight);
            }
            let list = lists + 12 * v as u32;
            e.mem.set_u32(list, array);
            e.mem.set_u32(list + 4, pairs.len() as u32);
            e.mem.set_u32(list + 8, pairs.len() as u32);
        }
        lists
    }

    fn influences(e: &Engine, lists: u32, vertex: u32) -> Vec<(u32, f32)> {
        let list = lists + 12 * vertex;
        let array = e.mem.u32(list);
        (0..e.mem.u32(list + 8))
            .map(|i| (e.mem.u32(array + 8 * i), e.mem.f32(array + 8 * i + 4)))
            .collect()
    }

    /// Geometry data with `vertices` vertices and the given triangles (virtual +0xac).
    fn geometry(e: &mut Engine, vertices: u16, triangles: &[[u16; 3]]) -> u32 {
        let data = e.mem.alloc(0x50);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(vtable + GET_TRIANGLE_SLOT, TRIANGLE_DOUBLE);
        e.mem.set_u16(data + 8, vertices);
        e.mem.set_u16(data + 0x40, triangles.len() as u16);
        let table = triangles.to_vec();
        e.register_double(TRIANGLE_DOUBLE, move |e, a| {
            let corners = table[a[1] as usize];
            e.mem.set_u16(a[2], corners[0]);
            e.mem.set_u16(a[3], corners[1]);
            e.mem.set_u16(a[4], corners[2]);
            Ret::default()
        });
        data
    }

    /// The `unsigned int` to `float` map's virtuals: the key modulo the bucket count, key
    /// equality, a new zeroed 12-byte item, and the real `SetValue`.
    fn map_doubles(e: &mut Engine) {
        e.register(HASH_DOUBLE, |e, a| word(a[1] % e.mem.u32(a[0] + 4)));
        e.register(EQUAL_DOUBLE, |_, a| word(u32::from(a[1] == a[2])));
        e.register(NEW_ITEM_DOUBLE, |e, _| word(e.mem.alloc(12)));
        e.put_vtable(
            NI_T_POINTER_MAP_VTABLE,
            &[
                0,
                HASH_DOUBLE,
                EQUAL_DOUBLE,
                0x00a8_2600,
                0,
                NEW_ITEM_DOUBLE,
                0,
            ],
        );
        e.register(MAP_REMOVE_ALL, |e, a| {
            let map = a[0];
            for b in 0..e.mem.u32(map + 4) {
                e.mem.set_u32(e.mem.u32(map + 8) + 4 * b, 0);
            }
            e.mem.set_u32(map + 0xc, 0);
            Ret::default()
        });
    }

    fn map(e: &mut Engine, buckets: u32) -> u32 {
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, NI_T_POINTER_MAP_VTABLE);
        e.mem.set_u32(map + 4, buckets);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(map + 8, table);
        map
    }

    fn map_value(e: &Engine, map: u32, key: u32) -> Option<f32> {
        let buckets = e.mem.u32(map + 8);
        let mut item = e.mem.u32(buckets + 4 * (key % e.mem.u32(map + 4)));
        while item != 0 {
            if e.mem.u32(item + 4) == key {
                return Some(e.mem.f32(item + 8));
            }
            item = e.mem.u32(item);
        }
        None
    }

    /// A partition with every array set (vertices 2, triangles 1, bones 2, no strips, 2 bones
    /// per vertex).
    fn partition(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(PARTITION_SIZE);
        e.mem.set_u16(p + PART_VERTICES, 2);
        e.mem.set_u16(p + PART_TRIANGLES, 1);
        e.mem.set_u16(p + PART_BONE_COUNT, 2);
        e.mem.set_u16(p + PART_BONES_PER_VERTEX, 2);
        let fill = |e: &mut Engine, bytes: u32, seed: u8| {
            let block = e.mem.alloc(bytes);
            for i in 0..bytes {
                e.mem.set_u8(block + i, seed.wrapping_add(i as u8));
            }
            block
        };
        let bones = fill(e, 4, 1);
        e.mem.set_u32(p + PART_BONES, bones);
        let map = fill(e, 4, 2);
        e.mem.set_u32(p + PART_VERTEX_MAP, map);
        let weights = fill(e, 16, 3);
        e.mem.set_u32(p + PART_WEIGHTS, weights);
        let list = fill(e, 6, 4);
        e.mem.set_u32(p + PART_TRI_LIST, list);
        let palette = fill(e, 4, 5);
        e.mem.set_u32(p + PART_BONE_PALETTE, palette);
        p
    }

    #[test]
    fn partition_is_equal_compares_counts_and_arrays() {
        let mut e = Engine::new();
        let a = partition(&mut e);
        let b = partition(&mut e);
        assert!(e.call(0x00a8_1c90, &args![a, b]).bool());
        // A count.
        e.mem.set_u16(b + PART_BONES_PER_VERTEX, 1);
        assert!(!e.call(0x00a8_1c90, &args![a, b]).bool());
        e.mem.set_u16(b + PART_BONES_PER_VERTEX, 2);
        // The last weight byte, the last triangle byte, the last palette byte.
        for (field, at) in [
            (PART_WEIGHTS, 15u32),
            (PART_TRI_LIST, 5),
            (PART_BONE_PALETTE, 3),
        ] {
            let block = e.mem.u32(b + field);
            let old = e.mem.u8(block + at);
            e.mem.set_u8(block + at, old ^ 0x80);
            assert!(!e.call(0x00a8_1c90, &args![a, b]).bool());
            e.mem.set_u8(block + at, old);
        }
        // One palette null.
        let palette = e.mem.u32(b + PART_BONE_PALETTE);
        e.mem.set_u32(b + PART_BONE_PALETTE, 0);
        assert!(!e.call(0x00a8_1c90, &args![a, b]).bool());
        e.mem.set_u32(a + PART_BONE_PALETTE, 0);
        assert!(e.call(0x00a8_1c90, &args![a, b]).bool());
        e.mem.set_u32(b + PART_BONE_PALETTE, palette);
        // Strips: the lengths (one strip of 3), then the strip-length sum of indices.
        for p in [a, b] {
            e.mem.set_u16(p + PART_STRIPS, 1);
            let lengths = e.mem.alloc(2);
            e.mem.set_u16(lengths, 3);
            e.mem.set_u32(p + PART_STRIP_LENGTHS, lengths);
            e.mem.set_u32(p + PART_BONE_PALETTE, 0);
        }
        assert!(e.call(0x00a8_1c90, &args![a, b]).bool());
        let list = e.mem.u32(b + PART_TRI_LIST);
        e.mem.set_u8(list + 5, 0x7f);
        assert!(!e.call(0x00a8_1c90, &args![a, b]).bool());
    }

    #[test]
    fn constructor_and_skin_partition_is_equal() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + PARTITION_ARRAY, 0x55);
        assert_eq!(e.call(0x00a8_1f90, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), NI_SKIN_PARTITION_VTABLE);
        assert_eq!(e.mem.u32(object + PARTITION_ARRAY), 0);
        e.register(NI_OBJECT_IS_EQUAL, |_, a| word(u32::from(a[0] != 0x40)));
        let make = |e: &mut Engine| {
            let object = e.mem.alloc(0x10);
            let parts = e.mem.alloc(2 * PARTITION_SIZE);
            e.mem.set_u32(object + PARTITIONS, 2);
            e.mem.set_u32(object + PARTITION_ARRAY, parts);
            object
        };
        let a = make(&mut e);
        let b = make(&mut e);
        assert!(e.call(0x00a8_1ff0, &args![a, b]).bool());
        let second = e.mem.u32(b + PARTITION_ARRAY) + PARTITION_SIZE;
        e.mem.set_u16(second + PART_VERTICES, 1);
        assert!(!e.call(0x00a8_1ff0, &args![a, b]).bool());
        e.mem.set_u32(b + PARTITIONS, 1);
        assert!(!e.call(0x00a8_1ff0, &args![a, b]).bool());
        assert!(!e.call(0x00a8_1ff0, &args![0x40u32, b]).bool());
    }

    #[test]
    fn set_at_updates_or_adds_an_item() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let m = map(&mut e, 5);
        e.call(0x00a8_2050, &args![m, 7u32, 0.5f32]);
        e.call(0x00a8_2050, &args![m, 12u32, 0.25f32]);
        assert_eq!(e.mem.u32(m + 0xc), 2);
        assert_eq!(map_value(&e, m, 7), Some(0.5));
        assert_eq!(map_value(&e, m, 12), Some(0.25));
        // 12 went in front of 7 in bucket 2.
        let head = e.mem.u32(e.mem.u32(m + 8) + 8);
        assert_eq!(e.mem.u32(head + 4), 12);
        e.call(0x00a8_2050, &args![m, 7u32, 2.0f32]);
        assert_eq!(e.mem.u32(m + 0xc), 2);
        assert_eq!(map_value(&e, m, 7), Some(2.0));
        // A signalling NaN is quieted.
        e.call(0x00a8_2050, &args![m, 7u32, 0x7f80_0001u32]);
        let item = e.mem.u32(head);
        assert_eq!(e.mem.u32(item + 8), 0x7fc0_0001);
    }

    #[test]
    fn normalize_scales_weights_to_a_unit_sum() {
        let mut e = Engine::new();
        let l = lists(
            &mut e,
            &[
                &[(1, 1.0), (2, 0.5), (3, 0.25), (4, 0.125), (5, 0.125)],
                &[(6, 3.0), (7, 1.0)],
                &[],
            ],
        );
        e.call(0x00a8_20e0, &args![l]);
        let got: Vec<f32> = influences(&e, l, 0).iter().map(|p| p.1).collect();
        assert_eq!(got, vec![0.5, 0.25, 0.125, 0.0625, 0.0625]);
        e.call(0x00a8_20e0, &args![l + 12]);
        let got: Vec<f32> = influences(&e, l, 1).iter().map(|p| p.1).collect();
        assert_eq!(got, vec![0.75, 0.25]);
        // An empty list is left alone.
        e.call(0x00a8_20e0, &args![l + 24]);
        assert_eq!(e.mem.u32(l + 24 + 8), 0);
    }

    #[test]
    fn subset_tests() {
        let mut e = Engine::new();
        heap(&mut e);
        let big = set(&mut e, &[1, 3, 5, 7]);
        let small = set(&mut e, &[3, 7]);
        let other = set(&mut e, &[3, 4]);
        let past = set(&mut e, &[9]);
        let empty = set(&mut e, &[]);
        assert!(e.call(0x00a8_2190, &args![big, small]).bool());
        assert!(!e.call(0x00a8_2190, &args![big, other]).bool());
        assert!(!e.call(0x00a8_2190, &args![big, past]).bool());
        assert!(!e.call(0x00a8_2190, &args![small, big]).bool());
        assert!(e.call(0x00a8_2190, &args![big, empty]).bool());
        // `00a821f0`: the keys of 8-byte items looked up with `bsearch`.
        let l = lists(&mut e, &[&[(5, 1.0), (1, 1.0)], &[(5, 1.0), (2, 1.0)]]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a8_21f0, &args![big, l]).bool());
        let looked = calls_to(&e, BSEARCH);
        assert_eq!(looked.len(), 2);
        assert_eq!(looked[0][1..], [e.mem.u32(big), 4, 2, COMPARE_U16]);
        assert!(!e.call(0x00a8_21f0, &args![big, l + 12]).bool());
        // `00a82890`: all three corners' lists.
        let corners = e.mem.alloc(6);
        assert!(e.call(0x00a8_2890, &args![big, corners, l]).bool());
        e.mem.set_u16(corners + 4, 1);
        assert!(!e.call(0x00a8_2890, &args![big, corners, l]).bool());
    }

    #[test]
    fn merge_steps_count_and_union() {
        let mut e = Engine::new();
        heap(&mut e);
        // The sets end with the 0xffff the iterator stops at.
        let a = set(&mut e, &[1, 4, 6]);
        let b = set(&mut e, &[2, 4, 9]);
        let it = e.mem.alloc(0x14);
        merge_iterator(&mut e, it, a, b);
        let steps: Vec<u32> = (0..5)
            .map(|_| e.call(0x00a8_2250, &args![it]).u32())
            .collect();
        assert_eq!(steps, vec![1, 2, 4, 6, 9]);
        assert_eq!(e.mem.u16(it + 8), 0xffff);
        assert_eq!(e.mem.u16(it + 0xa), 0xffff);
        assert_eq!(e.call(0x00a8_2770, &args![a, b]).u32(), 5);
        e.call_log = Some(vec![]);
        e.call(0x00a8_27f0, &args![a, b]);
        assert_eq!(set_values(&e, a), vec![1, 2, 4, 6, 9]);
        assert_eq!(e.mem.u32(a + 4), 6);
        assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![12]]);
        assert_eq!(calls_to(&e, NI_FREE).len(), 1);
    }

    #[test]
    fn vertex_map_and_triangle_list_from_triangles() {
        let mut e = Engine::new();
        heap(&mut e);
        let data = geometry(&mut e, 6, &[[0, 1, 2], [5, 3, 4], [4, 5, 2], [2, 3, 1]]);
        let p = e.mem.alloc(PARTITION_SIZE);
        e.mem.set_u16(p + PART_TRIANGLES, 2);
        let triangles = set(&mut e, &[2, 1]);
        let table = e.call(0x00a8_22d0, &args![p, triangles, data]).u32();
        // Triangle 2 (4, 5, 2) then 1 (5, 3, 4): new indices 0..3 for 4, 5, 2, 3.
        let mapped: Vec<u16> = (0..6).map(|v| e.mem.u16(table + 2 * v)).collect();
        assert_eq!(mapped, vec![0xffff, 0xffff, 2, 3, 0, 1]);
        assert_eq!(e.mem.u16(p + PART_VERTICES), 4);
        let map = e.mem.u32(p + PART_VERTEX_MAP);
        let back: Vec<u16> = (0..4).map(|i| e.mem.u16(map + 2 * i)).collect();
        assert_eq!(back, vec![4, 5, 2, 3]);
        e.call(0x00a8_2400, &args![p, triangles, data, table]);
        let list = e.mem.u32(p + PART_TRI_LIST);
        let got: Vec<u16> = (0..6).map(|i| e.mem.u16(list + 2 * i)).collect();
        assert_eq!(got, vec![0, 1, 2, 1, 3, 0]);
    }

    #[test]
    fn influences_go_into_the_map_unless_a_larger_one_is_there() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let m = map(&mut e, 7);
        let l = lists(
            &mut e,
            &[
                &[(1, 0.5), (2, 0.25)],
                &[(1, 0.75)],
                &[(2, 0.25), (3, f32::NAN)],
            ],
        );
        let corners = e.mem.alloc(6);
        for (i, v) in [0u16, 1, 2].iter().enumerate() {
            e.mem.set_u16(corners + 2 * i as u32, *v);
        }
        e.call(0x00a8_2530, &args![0u32, m, corners, l]);
        assert_eq!(map_value(&e, m, 1), Some(0.75));
        assert_eq!(map_value(&e, m, 2), Some(0.25));
        assert!(map_value(&e, m, 3).unwrap().is_nan());
        assert_eq!(e.mem.u32(m + 0xc), 3);
        // A NaN never replaces what is there.
        let l = lists(&mut e, &[&[(1, f32::NAN)], &[], &[]]);
        e.call(0x00a8_2530, &args![0u32, m, corners, l]);
        assert_eq!(map_value(&e, m, 1), Some(0.75));
    }

    #[test]
    fn map_items_and_destructors() {
        let mut e = Engine::new();
        heap(&mut e);
        let item = e.mem.alloc(12);
        e.call(0x00a8_2600, &args![0u32, item, 9u32, 1.5f32]);
        assert_eq!(e.mem.u32(item + 4), 9);
        assert_eq!(e.mem.f32(item + 8), 1.5);
        // `DeleteItem`: back on the free list under the lock.
        e.map(ALLOCATOR_FREE_LIST, 4);
        e.map(ALLOCATOR_LOCK, 8);
        e.mem.set_u32(ALLOCATOR_FREE_LIST, 0x1234);
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(ALLOCATOR_LOCK, 0x99);
            let count = e.mem.u32(ALLOCATOR_LOCK_COUNT) + 1;
            e.mem.set_u32(ALLOCATOR_LOCK_COUNT, count);
            Ret::default()
        });
        e.call(0x00a8_2620, &args![0u32, item]);
        assert_eq!(
            (e.mem.u32(item), e.mem.u32(item + 4), e.mem.u32(item + 8)),
            (0x1234, 0, 0)
        );
        assert_eq!(e.mem.u32(ALLOCATOR_FREE_LIST), item);
        assert_eq!(e.mem.u32(ALLOCATOR_LOCK), 0);
        // A lock still held keeps its owner.
        e.mem.set_u32(ALLOCATOR_LOCK_COUNT, 1);
        e.call(0x00a8_2620, &args![0u32, item]);
        assert_eq!(e.mem.u32(ALLOCATOR_LOCK), 0x99);
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m + 8, 0x4444);
        assert_eq!(e.call(0x00a8_2670, &args![m, 0u32]).u32(), m);
        assert_eq!(e.mem.u32(m), NI_T_MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 0);
        e.call(0x00a8_2670, &args![m, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, m]]
        );
        assert_eq!(e.call(0x00a8_2cf0, &args![m, 1u32]).u32(), m);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 4);
        assert_eq!(e.mem.u32(m), NI_T_MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 2);
    }

    #[test]
    fn reallocs_copy_the_used_elements() {
        let mut e = Engine::new();
        heap(&mut e);
        let s = set(&mut e, &[4, 5, 6]);
        e.call_log = Some(vec![]);
        e.call(0x00a8_2710, &args![s, 3u32]);
        assert_eq!(calls_to(&e, NI_ALLOC).len(), 0);
        e.call(0x00a8_2710, &args![s, 8u32]);
        assert_eq!(set_values(&e, s), vec![4, 5, 6]);
        assert_eq!(e.mem.u32(s + 4), 8);
        assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![16]]);
        e.call(0x00a8_2710, &args![s, 0u32]);
        assert_eq!(e.mem.u32(s), 0);
        let pairs = lists(&mut e, &[&[(1, 0.5), (2, 0.25)]]);
        e.call(0x00a8_26b0, &args![pairs, 4u32]);
        assert_eq!(influences(&e, pairs, 0), vec![(1, 0.5), (2, 0.25)]);
        assert_eq!(e.mem.u32(pairs + 4), 4);
        // `00a833a0` clears the three words.
        assert_eq!(e.call(0x00a8_33a0, &args![pairs]).u32(), pairs);
        assert_eq!(e.mem.u32(pairs + 8), 0);
    }

    #[test]
    fn find_set_and_collect_indices() {
        let mut e = Engine::new();
        heap(&mut e);
        let a = set(&mut e, &[1, 2]);
        let b = set(&mut e, &[1, 2, 3]);
        let sets = words(&mut e, &[a, b]);
        let l = lists(&mut e, &[&[(1, 1.0)], &[(3, 1.0)], &[(2, 1.0)]]);
        let corners = e.mem.alloc(6);
        for (i, v) in [0u16, 1, 2].iter().enumerate() {
            e.mem.set_u16(corners + 2 * i as u32, *v);
        }
        let used = e.mem.alloc(2);
        assert_eq!(
            e.call(0x00a8_2c90, &args![sets, corners, l, used, 0u32])
                .i32(),
            1
        );
        assert_eq!(e.mem.u8(used + 1), 1);
        assert_eq!(
            e.call(0x00a8_2c90, &args![sets, corners, l, used, 0u32])
                .i32(),
            -1
        );
        assert_eq!(
            e.call(0x00a8_2c90, &args![sets, corners, l, used, 1u32])
                .i32(),
            1
        );
        // `00a82d30`: the indices of the equal words, the set doubling from 1.
        let s = set(&mut e, &[]);
        let array = e.mem.alloc(16);
        for (i, w) in [7u32, 3, 7, 7].iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *w);
        }
        e.call(0x00a8_2d30, &args![s, 7u32, array, 4u32]);
        assert_eq!(set_values(&e, s), vec![0, 2, 3]);
        assert_eq!(e.mem.u32(s + 4), 4);
    }

    #[test]
    fn fill_a_limited_partition() {
        let mut e = Engine::new();
        heap(&mut e);
        e.map(NEGATIVE_MAXIMUM, 4);
        e.mem.set_u32(NEGATIVE_MAXIMUM, 0xff7f_ffff);
        let data = geometry(&mut e, 3, &[[0, 1, 2]]);
        let triangles = set(&mut e, &[0]);
        let bones = set(&mut e, &[4, 9]);
        let l = lists(
            &mut e,
            &[&[(9, 0.25), (4, 0.75)], &[(4, 1.0)], &[(9, 0.5), (4, 0.5)]],
        );
        let p = e.mem.alloc(PARTITION_SIZE);
        e.call(
            0x00a8_28d0,
            &args![p, triangles, bones, data, l, 1u32, 0u32, 3u32, 10u32],
        );
        assert_eq!(e.mem.u16(p + PART_BONE_COUNT), 3);
        assert_eq!(e.mem.u16(p + PART_TRIANGLES), 1);
        let partition_bones: Vec<u16> = (0..3)
            .map(|i| e.mem.u16(e.mem.u32(p + PART_BONES) + 2 * i))
            .collect();
        assert_eq!(partition_bones, vec![4, 9, 0]);
        assert_eq!(e.mem.u16(p + PART_BONES_PER_VERTEX), 3);
        let weights: Vec<f32> = (0..9)
            .map(|i| e.mem.f32(e.mem.u32(p + PART_WEIGHTS) + 4 * i))
            .collect();
        assert_eq!(weights, vec![0.75, 0.25, 0.0, 1.0, 0.0, 0.0, 0.5, 0.5, 0.0]);
        let palette: Vec<u8> = (0..9)
            .map(|i| e.mem.u8(e.mem.u32(p + PART_BONE_PALETTE) + i))
            .collect();
        assert_eq!(palette, vec![0, 1, 0, 0, 0, 0, 1, 0, 0]);
    }

    #[test]
    fn fill_an_unlimited_partition_with_a_palette() {
        let mut e = Engine::new();
        heap(&mut e);
        let data = geometry(&mut e, 2, &[[0, 1, 0]]);
        let triangles = set(&mut e, &[0]);
        let bones = set(&mut e, &[4, 9]);
        let l = lists(&mut e, &[&[(9, 0.25), (4, 0.75)], &[(4, 1.0)]]);
        let p = e.mem.alloc(PARTITION_SIZE);
        e.call(
            0x00a8_28d0,
            &args![p, triangles, bones, data, l, 0u32, 1u32, 3u32, 10u32],
        );
        assert_eq!(e.mem.u16(p + PART_BONE_COUNT), 3);
        assert_eq!(e.mem.u16(p + PART_BONES_PER_VERTEX), 3);
        let weights: Vec<f32> = (0..6)
            .map(|i| e.mem.f32(e.mem.u32(p + PART_WEIGHTS) + 4 * i))
            .collect();
        assert_eq!(weights, vec![0.75, 0.25, 0.0, 1.0, 0.0, 0.0]);
        let palette: Vec<u8> = (0..6)
            .map(|i| e.mem.u8(e.mem.u32(p + PART_BONE_PALETTE) + i))
            .collect();
        assert_eq!(palette, vec![0, 1, 2, 0, 1, 2]);
        // Without the palette byte there is none.
        let q = e.mem.alloc(PARTITION_SIZE);
        e.call(
            0x00a8_28d0,
            &args![q, triangles, bones, data, l, 0u32, 0u32, 1u32, 10u32],
        );
        assert_eq!(e.mem.u32(q + PART_BONE_PALETTE), 0);
        assert_eq!(e.mem.u16(q + PART_BONE_COUNT), 2);
    }

    fn cut_engine(order: &[u16]) -> Engine {
        let mut e = Engine::new();
        heap(&mut e);
        map_doubles(&mut e);
        e.map(FLOAT_MAXIMUM, 4);
        e.mem.set_u32(FLOAT_MAXIMUM, 0x7f7f_ffff);
        let order = order.to_vec();
        e.register_double(GEOMETRY_SORTED_VERTICES, move |e, a| {
            let count = order.len() as u32;
            let sorted = e.mem.alloc(2 * count);
            let positions = e.mem.alloc(2 * count);
            for (at, v) in order.iter().enumerate() {
                e.mem.set_u16(sorted + 2 * at as u32, *v);
                e.mem.set_u16(positions + 2 * u32::from(*v), at as u16);
            }
            e.mem.set_u32(a[1], sorted);
            e.mem.set_u32(a[2], positions);
            Ret::default()
        });
        // Vertices 0 and 3 are the same point.
        e.register(GEOMETRY_COMPARE_VERTICES, |_, a| {
            word(u32::from(!(a[1] + a[2] == 3 && a[1] * a[2] == 0)))
        });
        e
    }

    #[test]
    fn cut_influences_down_to_the_allowed_bones() {
        let mut e = cut_engine(&[0, 3, 1, 2]);
        let data = geometry(&mut e, 4, &[[0, 1, 2], [1, 1, 2]]);
        let l = lists(
            &mut e,
            &[
                &[(1, 0.5), (2, 0.5)],
                &[(1, 1.0)],
                &[(3, 0.75), (1, 0.25)],
                &[(1, 0.5), (2, 0.5)],
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a8_33b0, &args![0x10u32, data, 2u32, 1u32, l])
            .bool());
        // Bone 2 (the smallest, 0.5) left vertex 0 and its twin 3; vertex 2 was cut to one.
        assert_eq!(influences(&e, l, 0), vec![(1, 1.0)]);
        assert_eq!(influences(&e, l, 3), vec![(1, 1.0)]);
        assert_eq!(influences(&e, l, 1), vec![(1, 1.0)]);
        assert_eq!(influences(&e, l, 2), vec![(3, 1.0)]);
        assert_eq!(calls_to(&e, QSORT).len(), 1);
        assert_eq!(calls_to(&e, MEMSET)[0][1..], [0, 0x94]);
        // The degenerate triangle was skipped: one fill of the map, one clear after it.
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 3);
    }

    #[test]
    fn cut_influences_fails_on_an_emptied_list() {
        let mut e = cut_engine(&[0, 1, 2]);
        let data = geometry(&mut e, 3, &[[0, 1, 2]]);
        let l = lists(&mut e, &[&[(1, 0.5), (2, 0.5)], &[(2, 1.0)], &[(3, 1.0)]]);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a8_33b0, &args![0x10u32, data, 1u32, 4u32, l])
            .bool());
        assert!(influences(&e, l, 0).is_empty());
        assert_eq!(calls_to(&e, QSORT).len(), 0);
        // The touched array (freed once when first made), the arrays and the buckets are freed.
        assert_eq!(calls_to(&e, NI_FREE).len(), 5);
    }

    #[test]
    fn merge_bone_sets_by_shared_bones() {
        let mut e = Engine::new();
        heap(&mut e);
        let a = set(&mut e, &[1, 2]);
        let b = set(&mut e, &[2, 3]);
        let c = set(&mut e, &[5]);
        let sets = words(&mut e, &[a, b, c]);
        e.call_log = Some(vec![]);
        e.call(0x00a8_39d0, &args![0u32, 3u32, sets]);
        assert_eq!(set_values(&e, a), vec![1, 2, 5]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, c]]
        );
        // Reordered by group number, the later group first.
        assert_eq!(word_values(&e, sets), vec![b, a]);
    }

    #[test]
    fn merge_bone_sets_with_different_tags_keeps_both() {
        let mut e = Engine::new();
        heap(&mut e);
        let a = set(&mut e, &[1, 2]);
        let b = set(&mut e, &[2, 3]);
        let c = set(&mut e, &[5]);
        e.mem.set_u32(c + 0xc, 1);
        let sets = words(&mut e, &[a, b, c]);
        e.call_log = Some(vec![]);
        e.call(0x00a8_39d0, &args![0u32, 3u32, sets]);
        assert_eq!(set_values(&e, a), vec![1, 2, 5]);
        assert_eq!(set_values(&e, c), vec![1, 2, 5]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 0);
        assert_eq!(word_values(&e, sets), vec![b, a, c]);
    }

    fn realloc_words(e: &mut Engine) {
        e.register(WORD_SET_REALLOC, |e, a| {
            let (set, size) = (a[0], a[1]);
            let array = e.mem.alloc(4 * size);
            for i in 0..e.mem.u32(set + 8) {
                let w = e.mem.u32(e.mem.u32(set) + 4 * i);
                e.mem.set_u32(array + 4 * i, w);
            }
            e.mem.set_u32(set, array);
            e.mem.set_u32(set + 4, size);
            Ret::default()
        });
    }

    #[test]
    fn bone_sets_per_triangle() {
        let mut e = Engine::new();
        heap(&mut e);
        realloc_words(&mut e);
        let data = geometry(&mut e, 4, &[[0, 1, 2], [0, 0, 1], [2, 1, 3]]);
        let l = lists(
            &mut e,
            &[&[(1, 1.0)], &[(2, 1.0)], &[(1, 1.0)], &[(4, 1.0)]],
        );
        let sets = words(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00a8_3d40, &args![0u32, data, l, sets, 0u32]);
        let out = word_values(&e, sets);
        assert_eq!(out.len(), 1);
        assert_eq!(set_values(&e, out[0]), vec![1, 2, 4]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 2);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    #[test]
    fn bone_sets_per_vertex_word() {
        let mut e = Engine::new();
        heap(&mut e);
        let data = geometry(&mut e, 4, &[[0, 1, 2], [2, 1, 3]]);
        let l = lists(
            &mut e,
            &[&[(1, 1.0)], &[(2, 1.0)], &[(1, 1.0)], &[(4, 1.0)]],
        );
        let tags = e.mem.alloc(16);
        for (i, t) in [7u32, 7, 9, 9].iter().enumerate() {
            e.mem.set_u32(tags + 4 * i as u32, *t);
        }
        let sets = words(&mut e, &[]);
        e.call(0x00a8_3d40, &args![0u32, data, l, sets, tags]);
        let out = word_values(&e, sets);
        assert_eq!(out.len(), 2);
        assert_eq!((e.mem.u32(out[0] + 0xc), e.mem.u32(out[1] + 0xc)), (7, 9));
        assert_eq!(set_values(&e, out[0]), vec![1, 2, 4]);
        assert_eq!(set_values(&e, out[1]), vec![1, 2, 4]);
        assert_eq!(e.mem.u32(sets + 4), 2);
    }

    #[test]
    fn partition_triangles_without_words() {
        let mut e = Engine::new();
        heap(&mut e);
        realloc_words(&mut e);
        let data = geometry(&mut e, 4, &[[0, 1, 2], [1, 1, 3]]);
        let l = lists(
            &mut e,
            &[&[(1, 1.0)], &[(2, 1.0)], &[(1, 1.0)], &[(3, 1.0)]],
        );
        let sets = words(&mut e, &[]);
        let skin = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        assert!(e
            .call(
                0x00a8_4310,
                &args![skin, data, 0u32, 4u32, l, sets, out, 0u32]
            )
            .bool());
        assert_eq!(e.mem.u32(skin + PARTITIONS), 1);
        let array = e.mem.u32(out);
        assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (0, 0xffff_ffff));
    }

    #[test]
    fn partition_triangles_drops_unused_sets() {
        let mut e = Engine::new();
        heap(&mut e);
        realloc_words(&mut e);
        // Two triangles with unrelated bones make two sets; a third set no triangle uses is
        // dropped and the last set takes its place.
        let data = geometry(&mut e, 6, &[[0, 1, 2], [3, 4, 5]]);
        let l = lists(
            &mut e,
            &[
                &[(1, 1.0)],
                &[(1, 1.0)],
                &[(1, 1.0)],
                &[(2, 1.0)],
                &[(2, 1.0)],
                &[(2, 1.0)],
            ],
        );
        let unused = set(&mut e, &[8, 9]);
        let sets = words(&mut e, &[unused]);
        let skin = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        assert!(e
            .call(
                0x00a8_4310,
                &args![skin, data, 0u32, 1u32, l, sets, out, 0u32]
            )
            .bool());
        let left = word_values(&e, sets);
        assert_eq!(left.len(), 2);
        assert!(!left.contains(&unused));
        let array = e.mem.u32(out);
        let first = e.mem.u32(array);
        let second = e.mem.u32(array + 4);
        assert_eq!(set_values(&e, left[first as usize]), vec![1]);
        assert_eq!(set_values(&e, left[second as usize]), vec![2]);
        assert_eq!(e.mem.u32(skin + PARTITIONS), 2);
    }

    fn int_map_doubles(e: &mut Engine) {
        e.register(HASH_DOUBLE, |e, a| word(a[1] % e.mem.u32(a[0] + 4)));
        e.register(EQUAL_DOUBLE, |_, a| word(u32::from(a[1] == a[2])));
        e.put_vtable(INT_MAP_VTABLE, &[0, HASH_DOUBLE, EQUAL_DOUBLE]);
        e.register(INT_MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(INT_MAP_SET_AT, |e, a| {
            let (map, key, value) = (a[0], a[1], a[2]);
            let item = e.mem.alloc(12);
            let head = e.mem.u32(map + 8) + 4 * (key % e.mem.u32(map + 4));
            e.mem.set_u32(item, e.mem.u32(head));
            e.mem.set_u32(item + 4, key);
            e.mem.set_u32(item + 8, value);
            e.mem.set_u32(head, item);
            Ret::default()
        });
        e.register(INT_MAP_GET_AT, |e, a| {
            let (map, key) = (a[0], a[1]);
            let mut item = e
                .mem
                .u32(e.mem.u32(map + 8) + 4 * (key % e.mem.u32(map + 4)));
            while item != 0 {
                if e.mem.u32(item + 4) == key {
                    e.mem.set_u32(a[2], e.mem.u32(item + 8));
                    return word(1);
                }
                item = e.mem.u32(item);
            }
            word(0)
        });
    }

    #[test]
    fn partition_triangles_by_vertex_words() {
        let mut e = Engine::new();
        heap(&mut e);
        int_map_doubles(&mut e);
        let data = geometry(&mut e, 4, &[[0, 1, 2], [1, 2, 3]]);
        let l = lists(
            &mut e,
            &[&[(1, 1.0)], &[(2, 1.0)], &[(1, 1.0)], &[(3, 1.0)]],
        );
        let tags = e.mem.alloc(16);
        for (i, t) in [5u32, 5, 5, 6].iter().enumerate() {
            e.mem.set_u32(tags + 4 * i as u32, *t);
        }
        let sets = words(&mut e, &[]);
        let skin = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00a8_4310,
                &args![skin, data, 0u32, 4u32, l, sets, out, tags]
            )
            .bool());
        assert_eq!(calls_to(&e, INT_MAP_SET_AT).len(), 2);
        assert_eq!(calls_to(&e, INT_MAP_GET_AT).len(), 1);
        let array = e.mem.u32(out);
        assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (0, 0));
        assert_eq!(e.mem.u32(skin + PARTITIONS), 2);
        // Three different words on one triangle end it with false.
        e.mem.set_u32(tags + 4, 7);
        let sets = words(&mut e, &[]);
        e.mem.set_u32(skin + PARTITIONS, 0);
        assert!(!e
            .call(
                0x00a8_4310,
                &args![skin, data, 0u32, 4u32, l, sets, out, tags]
            )
            .bool());
        assert_eq!(e.mem.u32(skin + PARTITIONS), 0);
    }

    #[test]
    fn get_viewer_strings_lists_each_partition() {
        let mut e = Engine::new();
        e.map(NI_SKIN_PARTITION_RTTI, 8);
        e.mem.set_u32(NI_SKIN_PARTITION_RTTI, 0x0109_e400);
        e.register(NI_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let printed: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(vec![]));
        let log = printed.clone();
        e.register_double(PRINT_00406D00, move |_, a| {
            log.borrow_mut().push(a[1..].to_vec());
            Ret::default()
        });
        // The class name, then a string, then null.
        let mut texts = vec![0x7000u32, 0, 0x9000].into_iter();
        e.register_double(NI_GET_VIEWER_STRING, move |_, _| {
            word(texts.next_back().unwrap())
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let slot = e.mem.u32(a[0] + 4) + 4 * a[1];
            e.mem.set_u32(slot, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            e.mem.set_u16(a[0] + 0xc, e.mem.u16(a[0] + 0xc) + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, array);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 4);
        let skin = e.mem.alloc(0x10);
        let parts = e.mem.alloc(2 * PARTITION_SIZE);
        e.mem.set_u16(parts + PART_BONE_COUNT, 3);
        e.mem.set_u16(parts + PARTITION_SIZE + PART_BONE_COUNT, 4);
        e.mem.set_u32(skin + PARTITIONS, 2);
        e.mem.set_u32(skin + PARTITION_ARRAY, parts);
        e.call_log = Some(vec![]);
        e.call(0x00a8_46e0, &args![skin, strings]);
        let printed = printed.borrow();
        assert_eq!(printed[0][1..], [PARTITION_FORMAT, 0, 3]);
        assert_eq!(printed[1][1..], [PARTITION_FORMAT, 1, 4]);
        let array = e.mem.u32(strings + 4);
        let values: Vec<u32> = (0..3).map(|i| e.mem.u32(array + 4 * i)).collect();
        assert_eq!(values, vec![0x9000, 0, 0x7000]);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        // One non-null string from `SetAt`, one from the partitions.
        assert_eq!(e.mem.u16(strings + 0xc), 2);
        // Full at the second partition: grown by 4.
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 6]]);
    }
}
