//! `NiMain/nitrishapedata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `NiTriShapeData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_SHAPE_DATA_RTTI: u32 = 0x011f_4a88;

// Translated from 00a7b860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_shape_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_SHAPE_DATA_RTTI)
}

// Translated from 00a7b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7b8c0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x58 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_tri_shape_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_b8c0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x58u32]);
    }
    this
}

layout! {
    /// `NiTriShapeData` (Xbox PDB), an `NiTriBasedGeomData`; 0x58 bytes on PC as on the
    /// Xbox (the sized delete of `00a7b920`). The fields its constructor sets, at the
    /// Xbox PDB's offsets, which the PC constructor writes.
    pub struct NiTriShapeData: 0x58 {
        /// `m_uiTriListLength` (Xbox PDB).
        0x44 m_uiTriListLength: u32,
        /// `m_pusTriList` (Xbox PDB): `u16*`.
        0x48 m_pusTriList: u32,
        /// `m_pkSharedNormals` (Xbox PDB): `NiTriShapeData::SharedNormalArray*`.
        0x4C m_pkSharedNormals: u32,
        /// `m_usSharedNormalsArraySize` (Xbox PDB).
        0x50 m_usSharedNormalsArraySize: u16,
        /// `m_pkSNAMemoryBlocks` (Xbox PDB): `NiTriShapeData::SNAMemBlock*`.
        0x54 m_pkSNAMemoryBlocks: u32,
    }
}

/// `NiTriBasedGeomData::NiTriBasedGeomData` (engine map, `00a8a970`): `thiscall`, no stack
/// words.
const NI_TRI_BASED_GEOM_DATA_CONSTRUCT: u32 = 0x00a8_a970;
/// `NiTriShapeData`'s vtable (`0109dac4`, RTTI `.?AVNiTriShapeData@@`).
const NI_TRI_SHAPE_DATA_VTABLE: u32 = 0x0109_dac4;

// Translated from 00a7b690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::NiTriShapeData_ov2` (Xbox PDB). `thiscall`, no stack words: runs the
/// `NiTriBasedGeomData` constructor (`00a8a970`), clears the triangle list length, the
/// triangle list, the shared normals and their memory blocks, puts this class's vtable
/// (`0109dac4`) at +0 and clears the 16-bit shared normals count, in the exe's order.
/// Answers `this`.
pub fn ni_tri_shape_data_ni_tri_shape_data_ov2(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
) -> Ptr<NiTriShapeData> {
    e.call(NI_TRI_BASED_GEOM_DATA_CONSTRUCT, &args![this]);
    e.set(this, NiTriShapeData::m_uiTriListLength, 0);
    e.set(this, NiTriShapeData::m_pusTriList, 0);
    e.set(this, NiTriShapeData::m_pkSharedNormals, 0);
    e.set(this, NiTriShapeData::m_pkSNAMemoryBlocks, 0);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_DATA_VTABLE);
    e.set(this, NiTriShapeData::m_usSharedNormalsArraySize, 0);
    this
}

/// `00aa1070` / `00aa10f0` (`cdecl`, one word each, unnamed in the engine map): the Ni
/// allocator's allocate and free (through the allocator object at `011f6080`).
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The memory manager (`011f6238`) and its allocate / free (`00aa3e40` / `00aa4060`,
/// `thiscall`, one stack word each).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `NiTriBasedGeomData::NiTriBasedGeomData` with data (`00a8a920`, `thiscall`, `RET 0x20`),
/// its destructor body `00a8a990` and `NiTriBasedGeomData::LoadBinary` / `SaveBinary`
/// (engine map, `00a8aa30` / `00a8aa70`, `RET 4`).
const NI_TRI_BASED_GEOM_DATA_CONSTRUCT_WITH_DATA: u32 = 0x00a8_a920;
const NI_TRI_BASED_GEOM_DATA_DESTRUCT_00A8A990: u32 = 0x00a8_a990;
const NI_TRI_BASED_GEOM_DATA_LOAD_BINARY: u32 = 0x00a8_aa30;
const NI_TRI_BASED_GEOM_DATA_SAVE_BINARY: u32 = 0x00a8_aa70;
/// `00a7b880` (`thiscall`, `RET 4`, this unit, not translated here): frees a chain of
/// shared-normal memory blocks (`SNAMemBlock`, 0x14 bytes, the next at +0x10).
const MEMORY_BLOCK_DELETE_00A7B880: u32 = 0x00a7_b880;
/// `00401050` (`stdcall`: array, element size, count, constructor) and the shared-normal
/// array element constructor `00a7b870` handed to it.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
const SHARED_NORMAL_ARRAY_CONSTRUCT: u32 = 0x00a7_b870;
/// `m_usTriangles` (Xbox PDB `NiTriBasedGeomData`, +0x40).
const TRIANGLES: u32 = 0x40;
/// `NiStream`: `m_uiNifFileVersion` (+0xd8), `m_pkIstr` (+0x24c), `m_pkOstr` (+0x250);
/// `NiBinaryStream`: the position (+4), `m_pfnRead` (+8), `m_pfnWrite` (+0xc).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;

/// One transfer through the stream's input (`read`) or output stream: `(binary, buffer,
/// bytes, size_slot, 1)` with the element size written into `size_slot` first, the position
/// advanced by the result.
fn transfer(
    e: &mut Engine,
    stream: u32,
    read: bool,
    buffer: u32,
    bytes: u32,
    size_slot: u32,
    size: u32,
) {
    let (stream_slot, function_slot) = if read {
        (STREAM_INPUT, 8)
    } else {
        (STREAM_OUTPUT, 0xc)
    };
    let binary = e.mem.u32(stream + stream_slot);
    let function = e.mem.u32(binary.wrapping_add(function_slot));
    e.mem.set_u32(size_slot, size);
    let moved = e
        .call(function, &args![binary, buffer, bytes, size_slot, 1u32])
        .u32();
    let position = e.mem.u32(binary.wrapping_add(4)).wrapping_add(moved);
    e.mem.set_u32(binary.wrapping_add(4), position);
}

/// A new `SNAMemBlock` (0x14 bytes from the memory manager) of `entries` 16-bit entries
/// (`00aa1070`): the block's start and cursor (+0, +4), size and free count (+8, +0xc),
/// next (+0x10) cleared. Null when the allocation fails.
fn new_memory_block(e: &mut Engine, entries: u32) -> u32 {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if block == 0 {
        return 0;
    }
    let memory = e
        .call(NI_ALLOCATE_00AA1070, &args![entries.wrapping_mul(2)])
        .u32();
    e.mem.set_u32(block, memory);
    e.mem.set_u32(block + 4, memory);
    e.mem.set_u32(block + 8, entries);
    e.mem.set_u32(block + 0xc, entries);
    e.mem.set_u32(block + 0x10, 0);
    block
}

// Translated from 00a7b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::NiTriShapeData` (Xbox PDB), `thiscall`, `RET 0x24`: the
/// `NiTriBasedGeomData` constructor `00a8a920` with the first eight words, then
/// `m_pusTriList` (+0x48) is the ninth, the shared normals and their memory blocks are
/// cleared, the vtable `0109dac4` is stored, `m_uiTriListLength` (+0x44) is three times the
/// triangle count (+0x40) and the shared normals count (+0x50) is cleared. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn ni_tri_shape_data_ni_tri_shape_data(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
    vertex_count: u32,
    vertices: u32,
    normals: u32,
    colors: u32,
    textures: u32,
    texture_sets: u32,
    data_flags: u32,
    triangle_count: u32,
    triangles: u32,
) -> Ptr<NiTriShapeData> {
    e.call(
        NI_TRI_BASED_GEOM_DATA_CONSTRUCT_WITH_DATA,
        &args![
            this,
            vertex_count,
            vertices,
            normals,
            colors,
            textures,
            texture_sets,
            data_flags,
            triangle_count
        ],
    );
    let length = u32::from(e.mem.u16(this.addr() + TRIANGLES)) * 3;
    e.set(this, NiTriShapeData::m_pusTriList, triangles);
    e.set(this, NiTriShapeData::m_pkSharedNormals, 0);
    e.set(this, NiTriShapeData::m_pkSNAMemoryBlocks, 0);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_DATA_VTABLE);
    e.set(this, NiTriShapeData::m_uiTriListLength, length);
    e.set(this, NiTriShapeData::m_usSharedNormalsArraySize, 0);
    this
}

// Translated from 00a7b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitrishapedata.cpp`; `thiscall`, `RET 8`): replaces the
/// triangle list: the old one is freed (`00aa10f0`) when it is set and differs, then the
/// triangle count (+0x40, 16 bits), `m_pusTriList` and `m_uiTriListLength` (three times the
/// count) are stored.
pub fn fn_00a7b6c0(e: &mut Engine, this: Ptr<NiTriShapeData>, triangle_count: u32, triangles: u32) {
    let old = e.get(this, NiTriShapeData::m_pusTriList);
    if triangles != old && old != 0 {
        e.call(NI_FREE_00AA10F0, &args![old]);
    }
    e.mem
        .set_u16(this.addr() + TRIANGLES, triangle_count as u16);
    e.set(this, NiTriShapeData::m_pusTriList, triangles);
    e.set(
        this,
        NiTriShapeData::m_uiTriListLength,
        u32::from(triangle_count as u16) * 3,
    );
}

// Translated from 00a7b720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::GetTriangleIndices` (Xbox PDB), `thiscall`, `RET 0x10`: the three
/// 16-bit indices of triangle `triangle` (low 16 bits) of `m_pusTriList` into `first`,
/// `second` and `third`.
pub fn ni_tri_shape_data_get_triangle_indices(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
    triangle: u32,
    first: Ptr,
    second: Ptr,
    third: Ptr,
) {
    let index = u32::from(triangle as u16) * 3;
    for (offset, out) in [(0u32, first), (1, second), (2, third)] {
        let list = e.get(this, NiTriShapeData::m_pusTriList);
        let value = e.mem.u16(list.wrapping_add((index + offset) * 2));
        e.mem.set_u16(out.addr(), value);
    }
}

// Translated from 00a7b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::GetTriList` (Xbox PDB): `m_pusTriList` (+0x48).
pub fn ni_tri_shape_data_get_tri_list(e: &mut Engine, this: Ptr<NiTriShapeData>) -> u32 {
    e.get(this, NiTriShapeData::m_pusTriList)
}

// Translated from 00a7b870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the element constructor `LoadBinary` hands to `00401050`
/// for the shared normals, `thiscall`, no stack words): clears the 16-bit count (+0) and the
/// pointer (+4) of one 8-byte `SharedNormalArray` entry. Answers `this`.
pub fn fn_00a7b870(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 00a7b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::~NiTriShapeData` (Xbox PDB; `thiscall`, no stack words): stores the
/// vtable `0109dac4`, frees `m_pusTriList` (`00aa10f0`) when set; with memory blocks
/// (+0x54) frees the first block's memory, deletes the chain after it (`00a7b880` with 1)
/// and frees the block (`00aa4060`); frees `m_pkSharedNormals` (+0x4c, null included) and
/// ends in the `NiTriBasedGeomData` destructor body `00a8a990`.
pub fn ni_tri_shape_data_destructor(e: &mut Engine, this: Ptr<NiTriShapeData>) {
    let list = e.get(this, NiTriShapeData::m_pusTriList);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_DATA_VTABLE);
    if list != 0 {
        e.call(NI_FREE_00AA10F0, &args![list]);
    }
    let blocks = e.get(this, NiTriShapeData::m_pkSNAMemoryBlocks);
    if blocks != 0 {
        let memory = e.mem.u32(blocks);
        e.call(NI_FREE_00AA10F0, &args![memory]);
        let next = e.mem.u32(blocks + 0x10);
        if next != 0 {
            e.call(MEMORY_BLOCK_DELETE_00A7B880, &args![next, 1u32]);
        }
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, blocks]);
    }
    let shared = e.get(this, NiTriShapeData::m_pkSharedNormals);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, shared]);
    e.call(NI_TRI_BASED_GEOM_DATA_DESTRUCT_00A8A990, &args![this]);
}

// Translated from 00a7b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::LoadBinary` (Xbox PDB), `thiscall`, `RET 4`. After
/// `NiTriBasedGeomData::LoadBinary`: `m_uiTriListLength`; from 10.0.1.17 a presence byte
/// (read into the stream argument's own slot; always present before); a present, non-empty
/// list is allocated (`00aa1070`, two bytes per index) and read. Then the shared normals
/// count (+0x50); when not zero the array (8 bytes per entry, constructed with `00a7b870`)
/// and a first memory block of that many entries, then per entry a 16-bit count and, when
/// not zero, that many indices taken from the first block with room (blocks chained at
/// +0x10; a new block of twice the head block's size goes in front when none has room) and
/// read there; the entry keeps the count and pointer (both 0 for an empty entry).
pub fn ni_tri_shape_data_load_binary(e: &mut Engine, this: Ptr<NiTriShapeData>, stream: Ptr) {
    let base = this.addr();
    let stream = stream.addr();
    e.call(NI_TRI_BASED_GEOM_DATA_LOAD_BINARY, &args![this, stream]);
    // Locals: +0 the stream argument's own slot, +4 the entry count read, +8 the element
    // size of the first reads, +0xc the loop index, +0x10 the element size in the loop.
    e.with_stack(0x14, |e, locals| {
        let l = locals.addr();
        let argument = l;
        e.mem.set_u32(argument, stream);
        transfer(e, stream, true, base + 0x44, 4, l + 8, 4);
        if e.mem.u32(stream + STREAM_VERSION) >= 0x0a00_0111 {
            transfer(e, stream, true, argument, 1, l + 8, 1);
        } else {
            e.mem.set_u8(argument, 1);
        }
        if e.mem.u8(argument) != 0 {
            let length = e.get(this, NiTriShapeData::m_uiTriListLength);
            if length != 0 {
                let list = e
                    .call(NI_ALLOCATE_00AA1070, &args![length.wrapping_mul(2)])
                    .u32();
                let bytes = e
                    .get(this, NiTriShapeData::m_uiTriListLength)
                    .wrapping_mul(2);
                e.set(this, NiTriShapeData::m_pusTriList, list);
                transfer(e, stream, true, list, bytes, l + 8, 2);
            }
        }
        transfer(e, stream, true, base + 0x50, 2, l + 8, 2);
        let count = u32::from(e.get(this, NiTriShapeData::m_usSharedNormalsArraySize));
        if count == 0 {
            return;
        }
        let array = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![
                    MEMORY_MANAGER_OBJECT,
                    count.checked_mul(8).unwrap_or(u32::MAX)
                ],
            )
            .u32();
        let array = if array != 0 {
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![array, 8u32, count, SHARED_NORMAL_ARRAY_CONSTRUCT],
            );
            array
        } else {
            0
        };
        e.set(this, NiTriShapeData::m_pkSharedNormals, array);
        let entries = u32::from(e.get(this, NiTriShapeData::m_usSharedNormalsArraySize));
        let first = new_memory_block(e, entries);
        e.set(this, NiTriShapeData::m_pkSNAMemoryBlocks, first);

        let mut index = 0u32;
        while (index as u16) < e.get(this, NiTriShapeData::m_usSharedNormalsArraySize) {
            transfer(e, stream, true, l + 4, 2, l + 0x10, 2);
            let mut entry_count = e.mem.u16(l + 4);
            let mut cursor = 0u32;
            if entry_count != 0 {
                let mut block = e.get(this, NiTriShapeData::m_pkSNAMemoryBlocks);
                while block != 0 && u32::from(entry_count) > e.mem.u32(block + 0xc) {
                    block = e.mem.u32(block + 0x10);
                }
                if block == 0 {
                    let head = e.get(this, NiTriShapeData::m_pkSNAMemoryBlocks);
                    let size = e.mem.u32(head + 8).wrapping_mul(2);
                    block = new_memory_block(e, size);
                    let head = e.get(this, NiTriShapeData::m_pkSNAMemoryBlocks);
                    e.mem.set_u32(block + 0x10, head);
                    entry_count = e.mem.u16(l + 4);
                    e.set(this, NiTriShapeData::m_pkSNAMemoryBlocks, block);
                }
                cursor = e.mem.u32(block + 4);
                let free = e.mem.u32(block + 0xc).wrapping_sub(u32::from(entry_count));
                e.mem.set_u32(block + 0xc, free);
                let taken = u32::from(e.mem.u16(l + 4));
                e.mem.set_u32(block + 4, cursor.wrapping_add(taken * 2));
                let bytes = u32::from(e.mem.u16(l + 4)) * 2;
                transfer(e, stream, true, cursor, bytes, l + 0x10, 2);
                entry_count = e.mem.u16(l + 4);
            }
            let shared = e.get(this, NiTriShapeData::m_pkSharedNormals);
            let slot = shared.wrapping_add(u32::from(index as u16) * 8);
            let (stored_count, stored_pointer) = if entry_count != 0 && cursor != 0 {
                (entry_count, cursor)
            } else {
                (0, 0)
            };
            e.mem.set_u32(slot + 4, stored_pointer);
            index = index.wrapping_add(1);
            e.mem.set_u16(slot, stored_count);
        }
    });
}

// Translated from 00a7bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::SaveBinary` (Xbox PDB), `thiscall`, `RET 4`. After
/// `NiTriBasedGeomData::SaveBinary`: `m_uiTriListLength`, a presence byte for
/// `m_pusTriList` (in the stream argument's own slot) and the list when present, the shared
/// normals count, then per entry its 16-bit count (from a local word) and that many indices.
pub fn ni_tri_shape_data_save_binary(e: &mut Engine, this: Ptr<NiTriShapeData>, stream: Ptr) {
    let base = this.addr();
    let stream = stream.addr();
    e.call(NI_TRI_BASED_GEOM_DATA_SAVE_BINARY, &args![this, stream]);
    // Locals: +0 the stream argument's own slot, +4 the entry count written, +8 the element
    // size, +0xc the element size in the loop.
    e.with_stack(0x10, |e, locals| {
        let l = locals.addr();
        let argument = l;
        e.mem.set_u32(argument, stream);
        transfer(e, stream, false, base + 0x44, 4, l + 8, 4);
        let has_list = e.get(this, NiTriShapeData::m_pusTriList) != 0;
        e.mem.set_u8(argument, u8::from(has_list));
        transfer(e, stream, false, argument, 1, l + 8, 1);
        if e.mem.u8(argument) != 0 {
            let bytes = e
                .get(this, NiTriShapeData::m_uiTriListLength)
                .wrapping_mul(2);
            let list = e.get(this, NiTriShapeData::m_pusTriList);
            transfer(e, stream, false, list, bytes, l + 8, 2);
        }
        transfer(e, stream, false, base + 0x50, 2, l + 8, 2);
        let mut index = 0u32;
        while (index as u16) < e.get(this, NiTriShapeData::m_usSharedNormalsArraySize) {
            let offset = u32::from(index as u16) * 8;
            let shared = e.get(this, NiTriShapeData::m_pkSharedNormals);
            let count = u32::from(e.mem.u16(shared.wrapping_add(offset)));
            e.mem.set_u32(l + 4, count);
            transfer(e, stream, false, l + 4, 2, l + 0xc, 2);
            let count = e.mem.u16(l + 4);
            if count != 0 {
                let shared = e.get(this, NiTriShapeData::m_pkSharedNormals);
                let indices = e.mem.u32(shared.wrapping_add(offset) + 4);
                transfer(e, stream, false, indices, u32::from(count) * 2, l + 0xc, 2);
            }
            index = index.wrapping_add(1);
        }
    });
}

/// `NiTriBasedGeomData::IsEqual` (`00a8a9c0`) and `GetViewerStrings` (`00a8aab0`),
/// thiscall, one word each.
const TRI_BASED_GEOM_DATA_IS_EQUAL: u32 = 0x00a8_a9c0;
const TRI_BASED_GEOM_DATA_GET_VIEWER_STRINGS: u32 = 0x00a8_aab0;
/// `NiGetViewerString` (cdecl, the class name), `NiGetViewerString_ov8` (cdecl, a name and
/// an unsigned value, `"%s = %u"`) and `NiGetViewerString_ov10` (cdecl, a name and a
/// pointer, `"%s = %p"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// The strings array `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_uiTriListLength"` and `"m_pusTriList"`.
const TEXT_TRI_LIST_LENGTH: u32 = 0x0109_db88;
const TEXT_TRI_LIST: u32 = 0x0109_db78;
/// `NiTriShapeData::SNAMemBlock` (0x14 bytes): its memory (+0x00, freed with `NiFree`)
/// and the next block (+0x10).
const BLOCK_MEMORY: u32 = 0x00;
const BLOCK_NEXT: u32 = 0x10;

/// Adds the string word at `slot` at the end of the strings array (`SetSize` to the size
/// plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a7b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body sets the triangles (thiscall, `RET 8`: the triangle
/// count and the index list): the count (16 bits, +0x40), three times it as the list length
/// and the list, without freeing the old one.
pub fn fn_00a7b700(e: &mut Engine, this: Ptr<NiTriShapeData>, triangles: u32, list: u32) {
    let triangles = triangles as u16;
    e.mem.set_u16(this.addr() + TRIANGLES, triangles);
    e.set(
        this,
        NiTriShapeData::m_uiTriListLength,
        u32::from(triangles) * 3,
    );
    e.set(this, NiTriShapeData::m_pusTriList, list);
}

// Translated from 00a7b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::GetStripData` (Xbox PDB; thiscall, `RET 0x10`, four out pointers): the
/// triangle count as the strip count (16 bits), no strip lengths (0), the index list and
/// three times the triangle count as the list length.
pub fn ni_tri_shape_data_get_strip_data(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
    strips: u32,
    strip_lengths: u32,
    tri_list: u32,
    tri_list_length: u32,
) {
    let triangles = e.mem.u16(this.addr() + TRIANGLES);
    e.mem.set_u16(strips, triangles);
    e.mem.set_u32(strip_lengths, 0);
    let list = e.get(this, NiTriShapeData::m_pusTriList);
    e.mem.set_u32(tri_list, list);
    let triangles = u32::from(e.mem.u16(this.addr() + TRIANGLES));
    e.mem.set_u32(tri_list_length, triangles * 3);
}

// Translated from 00a7b7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::IsEqual` (Xbox PDB; thiscall, `RET 4`): `NiTriBasedGeomData::IsEqual`,
/// the same list length, and the two index lists equal over twice the length in bytes (the
/// code inline `memcmp`: four bytes at a time, then up to three single bytes).
pub fn ni_tri_shape_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
    other: Ptr<NiTriShapeData>,
) -> bool {
    if e.call(TRI_BASED_GEOM_DATA_IS_EQUAL, &args![this, other])
        .u32() as u8
        == 0
    {
        return false;
    }
    let length = e.get(this, NiTriShapeData::m_uiTriListLength);
    if length != e.get(other, NiTriShapeData::m_uiTriListLength) {
        return false;
    }
    let mut theirs = e.get(other, NiTriShapeData::m_pusTriList);
    let mut mine = e.get(this, NiTriShapeData::m_pusTriList);
    let mut bytes = length.wrapping_add(length);
    while bytes >= 4 {
        if e.mem.u32(mine) != e.mem.u32(theirs) {
            return false;
        }
        bytes -= 4;
        theirs = theirs.wrapping_add(4);
        mine = mine.wrapping_add(4);
    }
    for i in 0..bytes {
        if e.mem.u8(theirs.wrapping_add(i)) != e.mem.u8(mine.wrapping_add(i)) {
            return false;
        }
    }
    true
}

// Translated from 00a7b880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is the scalar deleting destructor of a shared-normal
/// memory block (`NiTriShapeData::SNAMemBlock`; thiscall, `RET 4`): frees the block memory
/// (`NiFree`), deletes the next block the same way (flags 1) when there is one, then gives
/// the block back to the memory manager (`00aa4060` on `011f6238`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_00a7b880(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let memory = e.mem.u32(this.addr() + BLOCK_MEMORY);
    e.call(NI_FREE_00AA10F0, &args![memory]);
    let next = e.mem.u32(this.addr() + BLOCK_NEXT);
    if next != 0 {
        fn_00a7b880(e, Ptr::new(next), 1);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00a7bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriBasedGeomData::GetViewerStrings`, then the class name (the first word of
/// `ms_RTTI`), `"m_uiTriListLength"` with the list length and `"m_pusTriList"` with the
/// list address, each held in the argument stack word and added at the array end.
pub fn ni_tri_shape_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
    strings: Ptr<NiTArray>,
) {
    e.call(
        TRI_BASED_GEOM_DATA_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_TRI_SHAPE_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let length = e.get(this, NiTriShapeData::m_uiTriListLength);
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_TRI_LIST_LENGTH, length])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let list = e.get(this, NiTriShapeData::m_pusTriList);
        let text = e
            .call(VIEWER_STRING_POINTER, &args![TEXT_TRI_LIST, list])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a7_b630,
            ni_tri_shape_data_ni_tri_shape_data(
                Ptr<NiTriShapeData>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<NiTriShapeData>
        ),
        entry!(0x00a7_b6c0, fn_00a7b6c0(Ptr<NiTriShapeData>, u32, u32)),
        entry!(
            0x00a7_b720,
            ni_tri_shape_data_get_triangle_indices(Ptr<NiTriShapeData>, u32, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a7_b850,
            ni_tri_shape_data_get_tri_list(Ptr<NiTriShapeData>) -> u32
        ),
        entry!(0x00a7_b870, fn_00a7b870(Ptr) -> Ptr),
        entry!(
            0x00a7_b8c0,
            ni_tri_shape_data_destructor(Ptr<NiTriShapeData>)
        ),
        entry!(
            0x00a7_b940,
            ni_tri_shape_data_load_binary(Ptr<NiTriShapeData>, Ptr)
        ),
        entry!(
            0x00a7_bbe0,
            ni_tri_shape_data_save_binary(Ptr<NiTriShapeData>, Ptr)
        ),
        entry!(0x00a7b860, ni_tri_shape_data_get_rtti() -> Ptr),
        entry!(0x00a7b920, ni_tri_shape_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_b690,
            ni_tri_shape_data_ni_tri_shape_data_ov2(Ptr<NiTriShapeData>) -> Ptr<NiTriShapeData>
        ),
        entry!(0x00a7_b700, fn_00a7b700(Ptr<NiTriShapeData>, u32, u32)),
        entry!(
            0x00a7_b760,
            ni_tri_shape_data_get_strip_data(Ptr<NiTriShapeData>, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_b7e0,
            ni_tri_shape_data_is_equal(Ptr<NiTriShapeData>, Ptr<NiTriShapeData>) -> bool
        ),
        entry!(0x00a7_b880, fn_00a7b880(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_bd60,
            ni_tri_shape_data_get_viewer_strings(Ptr<NiTriShapeData>, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_shape_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7b860, &args![]).u32(), NI_TRI_SHAPE_DATA_RTTI);
    }

    #[test]
    fn ni_tri_shape_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_b8c0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a7b920, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7b920, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x58]]);
        assert_eq!(calls_to(&e, 0x00a7_b8c0), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_tri_shape_data_ov2_runs_the_base_then_clears_its_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        for offset in (0..0x58).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.register(NI_TRI_BASED_GEOM_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0101_0101);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_b690, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_TRI_BASED_GEOM_DATA_CONSTRUCT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_DATA_VTABLE);
        for offset in [0x44, 0x48, 0x4c, 0x54] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        // Only the low half of the word at +0x50 is the 16-bit count.
        assert_eq!(e.mem.u32(this + 0x50), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0x40), 0xdead_beef);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const READ_DOUBLE: u32 = 0x0e73_0000;
    const WRITE_DOUBLE: u32 = 0x0e73_0010;

    type Log = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for double in [
            NI_TRI_BASED_GEOM_DATA_DESTRUCT_00A8A990,
            NI_TRI_BASED_GEOM_DATA_LOAD_BINARY,
            NI_TRI_BASED_GEOM_DATA_SAVE_BINARY,
            NI_FREE_00AA10F0,
            MEMORY_MANAGER_FREE,
            MEMORY_BLOCK_DELETE_00A7B880,
            VECTOR_CONSTRUCTOR_ITERATOR,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(NI_TRI_BASED_GEOM_DATA_CONSTRUCT_WITH_DATA, |e, a| {
            e.mem.set_u16(a[0] + 0x40, a[8] as u16);
            a[0].into_ret()
        });
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn data(e: &mut Engine) -> Ptr<NiTriShapeData> {
        e.new_object()
    }

    #[test]
    fn constructor_with_data_sets_the_list() {
        let mut e = engine();
        let this = data(&mut e);
        e.set(this, NiTriShapeData::m_pkSharedNormals, 9);
        e.set(this, NiTriShapeData::m_usSharedNormalsArraySize, 9);
        let words = args![
            this,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32,
            7u32,
            0x1_0005u32,
            0x900u32
        ];
        assert_eq!(e.call(0x00a7_b630, &words).u32(), this.addr());
        assert_eq!(
            calls(&e, NI_TRI_BASED_GEOM_DATA_CONSTRUCT_WITH_DATA),
            vec![vec![this.addr(), 1, 2, 3, 4, 5, 6, 7, 0x1_0005]]
        );
        assert_eq!(e.get(this, NiTriShapeData::m_pusTriList), 0x900);
        assert_eq!(e.get(this, NiTriShapeData::m_uiTriListLength), 15);
        assert_eq!(e.get(this, NiTriShapeData::m_pkSharedNormals), 0);
        assert_eq!(e.get(this, NiTriShapeData::m_usSharedNormalsArraySize), 0);
        assert_eq!(e.mem.u32(this.addr()), NI_TRI_SHAPE_DATA_VTABLE);
    }

    #[test]
    fn replacing_the_list_frees_a_different_old_one() {
        let mut e = engine();
        let this = data(&mut e);
        e.set(this, NiTriShapeData::m_pusTriList, 0x800);
        e.call(0x00a7_b6c0, &args![this, 0x1_0004u32, 0x800u32]);
        assert!(calls(&e, NI_FREE_00AA10F0).is_empty());
        assert_eq!(e.mem.u16(this.addr() + 0x40), 4);
        assert_eq!(e.get(this, NiTriShapeData::m_uiTriListLength), 12);
        e.call(0x00a7_b6c0, &args![this, 2u32, 0x900u32]);
        assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![0x800]]);
        assert_eq!(e.get(this, NiTriShapeData::m_pusTriList), 0x900);
        e.set(this, NiTriShapeData::m_pusTriList, 0);
        e.call(0x00a7_b6c0, &args![this, 2u32, 0x900u32]);
        assert_eq!(calls(&e, NI_FREE_00AA10F0).len(), 1);
    }

    #[test]
    fn triangle_indices_and_list() {
        let mut e = engine();
        let this = data(&mut e);
        let list = e.mem.alloc(0x20);
        for i in 0..9u16 {
            e.mem.set_u16(list + 2 * u32::from(i), 100 + i);
        }
        e.set(this, NiTriShapeData::m_pusTriList, list);
        let out = e.mem.alloc(8);
        e.call(
            0x00a7_b720,
            &args![this, 0x1_0002u32, out, out + 2, out + 4],
        );
        assert_eq!(
            (e.mem.u16(out), e.mem.u16(out + 2), e.mem.u16(out + 4)),
            (106, 107, 108)
        );
        assert_eq!(e.call(0x00a7_b850, &args![this]).u32(), list);
    }

    #[test]
    fn shared_normal_entry_constructor_clears_it() {
        let mut e = engine();
        let entry = e.mem.alloc(8);
        e.mem.write(entry, &[0xff; 8]);
        assert_eq!(e.call(0x00a7_b870, &args![entry]).u32(), entry);
        assert_eq!(e.mem.bytes(entry, 8), vec![0, 0, 0xff, 0xff, 0, 0, 0, 0]);
    }

    #[test]
    fn destructor_frees_lists_and_blocks() {
        let mut e = engine();
        let this = data(&mut e);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, 0x4000);
        e.mem.set_u32(block + 0x10, 0x4100);
        e.set(this, NiTriShapeData::m_pusTriList, 0x3000);
        e.set(this, NiTriShapeData::m_pkSNAMemoryBlocks, block);
        e.set(this, NiTriShapeData::m_pkSharedNormals, 0x3500);
        e.call(0x00a7_b8c0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_TRI_SHAPE_DATA_VTABLE);
        assert_eq!(
            calls(&e, NI_FREE_00AA10F0),
            vec![vec![0x3000], vec![0x4000]]
        );
        assert_eq!(
            calls(&e, MEMORY_BLOCK_DELETE_00A7B880),
            vec![vec![0x4100, 1]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![
                vec![MEMORY_MANAGER_OBJECT, block],
                vec![MEMORY_MANAGER_OBJECT, 0x3500]
            ]
        );
        assert_eq!(
            calls(&e, NI_TRI_BASED_GEOM_DATA_DESTRUCT_00A8A990),
            vec![vec![this.addr()]]
        );
        // Nothing set: only the free of the (null) shared normals and the base.
        let other = data(&mut e);
        e.call(0x00a7_b8c0, &args![other]);
        assert_eq!(calls(&e, NI_FREE_00AA10F0).len(), 2);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE)[2],
            vec![MEMORY_MANAGER_OBJECT, 0]
        );
    }

    /// A stream whose reads serve `chunks` and whose writes log; both log (bytes, element
    /// size, and for writes the first 16-bit word of the buffer).
    fn make_stream(e: &mut Engine, version: u32, chunks: Vec<Vec<u8>>) -> (u32, Log) {
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, READ_DOUBLE);
        e.mem.set_u32(binary + 0xc, WRITE_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        let log: Log = Rc::default();
        let reads = log.clone();
        let mut chunks = chunks;
        e.register_double(READ_DOUBLE, move |e, a| {
            reads.borrow_mut().push((a[2], e.mem.u32(a[3]), 0));
            let chunk = chunks.remove(0);
            assert_eq!(chunk.len() as u32, a[2]);
            e.mem.write(a[1], &chunk);
            a[2].into_ret()
        });
        let writes = log.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            let first = u32::from(e.mem.u16(a[1]));
            writes.borrow_mut().push((a[2], e.mem.u32(a[3]), first));
            a[2].into_ret()
        });
        (stream, log)
    }

    #[test]
    fn load_binary_reads_list_and_shared_normals() {
        let mut e = engine();
        let chunks = vec![
            vec![6, 0, 0, 0],                         // list length
            vec![1],                                  // list present
            vec![1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0], // list
            vec![3, 0],                               // three shared-normal entries
            vec![2, 0],                               // entry 0: two indices
            vec![7, 0, 8, 0],
            vec![0, 0], // entry 1: empty
            vec![3, 0], // entry 2: three, more than the first block has left
            vec![9, 0, 10, 0, 11, 0],
        ];
        let (stream, log) = make_stream(&mut e, 0x1404_0000, chunks);
        let this = data(&mut e);
        e.call(0x00a7_b940, &args![this, stream]);
        let list = e.get(this, NiTriShapeData::m_pusTriList);
        assert_eq!(e.mem.u16(list + 10), 6);
        let shared = e.get(this, NiTriShapeData::m_pkSharedNormals);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![shared, 8, 3, SHARED_NORMAL_ARRAY_CONSTRUCT]]
        );
        // Entry 0 from the first block (3 entries), entry 2 from a new block of 6 in front.
        let first = e.mem.u32(shared + 4);
        assert_eq!(e.mem.u16(shared), 2);
        assert_eq!((e.mem.u16(first), e.mem.u16(first + 2)), (7, 8));
        assert_eq!((e.mem.u16(shared + 8), e.mem.u32(shared + 12)), (0, 0));
        let head = e.get(this, NiTriShapeData::m_pkSNAMemoryBlocks);
        assert_eq!((e.mem.u32(head + 8), e.mem.u32(head + 0xc)), (6, 3));
        let old = e.mem.u32(head + 0x10);
        assert_eq!((e.mem.u32(old + 8), e.mem.u32(old + 0xc)), (3, 1));
        assert_eq!(e.mem.u16(shared + 16), 3);
        let third = e.mem.u32(shared + 20);
        assert_eq!(third, e.mem.u32(head));
        assert_eq!(e.mem.u16(third + 4), 11);
        let sizes: Vec<(u32, u32)> = log.borrow().iter().map(|r| (r.0, r.1)).collect();
        assert_eq!(
            sizes,
            vec![
                (4, 4),
                (1, 1),
                (12, 2),
                (2, 2),
                (2, 2),
                (4, 2),
                (2, 2),
                (2, 2),
                (6, 2)
            ]
        );
    }

    #[test]
    fn load_binary_of_old_files_always_has_a_list() {
        let mut e = engine();
        let chunks = vec![vec![0, 0, 0, 0], vec![0, 0]];
        let (stream, log) = make_stream(&mut e, 0x0a00_0110, chunks);
        let this = data(&mut e);
        e.call(0x00a7_b940, &args![this, stream]);
        // An empty list: no allocation, no list read; no shared normals.
        assert_eq!(log.borrow().len(), 2);
        assert!(calls(&e, NI_ALLOCATE_00AA1070).is_empty());
        assert_eq!(e.get(this, NiTriShapeData::m_pkSharedNormals), 0);
    }

    #[test]
    fn save_binary_writes_list_and_entries() {
        let mut e = engine();
        let (stream, log) = make_stream(&mut e, 0x1404_0000, vec![]);
        let this = data(&mut e);
        let list = e.mem.alloc(8);
        e.mem.set_u16(list, 0x55);
        e.set(this, NiTriShapeData::m_uiTriListLength, 3);
        e.set(this, NiTriShapeData::m_pusTriList, list);
        let shared = e.mem.alloc(16);
        let indices = e.mem.alloc(8);
        e.mem.set_u16(indices, 0x77);
        e.mem.set_u16(shared, 2);
        e.mem.set_u32(shared + 4, indices);
        e.set(this, NiTriShapeData::m_pkSharedNormals, shared);
        e.set(this, NiTriShapeData::m_usSharedNormalsArraySize, 2);
        e.call(0x00a7_bbe0, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            vec![
                (4, 4, 3),
                (1, 1, 1),
                (6, 2, 0x55),
                (2, 2, 2),
                (2, 2, 2),
                (4, 2, 0x77),
                (2, 2, 0)
            ]
        );
        assert_eq!(
            calls(&e, NI_TRI_BASED_GEOM_DATA_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Shape data whose list holds `indices` (length: their count).
    fn shape_data(e: &mut Engine, indices: &[u16]) -> u32 {
        let this = e.mem.alloc(0x58);
        let list = e.mem.alloc(2 * indices.len() as u32 + 4);
        for (i, v) in indices.iter().enumerate() {
            e.mem.set_u16(list + 2 * i as u32, *v);
        }
        e.mem.set_u32(this + 0x44, indices.len() as u32);
        e.mem.set_u32(this + 0x48, list);
        this
    }

    #[test]
    fn setting_triangles_stores_count_length_and_list() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        e.call(0x00a7_b700, &args![this, 0x7_0005u32, 0x0abc_0000u32]);
        assert_eq!(e.mem.u16(this + TRIANGLES), 5);
        assert_eq!(e.mem.u32(this + 0x44), 15);
        assert_eq!(e.mem.u32(this + 0x48), 0x0abc_0000);
    }

    #[test]
    fn strip_data_is_the_triangle_list() {
        let mut e = Engine::new();
        let this = shape_data(&mut e, &[0; 6]);
        e.mem.set_u16(this + TRIANGLES, 2);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out, 0xffff_ffff);
        e.mem.set_u32(out + 4, 0xffff_ffff);
        e.call(0x00a7_b760, &args![this, out, out + 4, out + 8, out + 0xc]);
        assert_eq!(e.mem.u32(out), 0xffff_0002);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(out + 8), e.mem.u32(this + 0x48));
        assert_eq!(e.mem.u32(out + 0xc), 6);
    }

    #[test]
    fn is_equal_compares_the_lists_byte_for_byte() {
        let mut e = Engine::new();
        e.register(TRI_BASED_GEOM_DATA_IS_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        // Three entries: one four-byte step and two single bytes.
        let a = shape_data(&mut e, &[1, 2, 0x0304]);
        let b = shape_data(&mut e, &[1, 2, 0x0304]);
        assert_eq!(e.call(0x00a7_b7e0, &args![a, b]).u32() & 0xff, 1);
        let lb = e.mem.u32(b + 0x48);
        e.mem.set_u8(lb + 5, 9);
        assert_eq!(e.call(0x00a7_b7e0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u8(lb + 5, 3);
        e.mem.set_u16(lb + 2, 7);
        assert_eq!(e.call(0x00a7_b7e0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u16(lb + 2, 2);
        e.mem.set_u32(b + 0x44, 2);
        assert_eq!(e.call(0x00a7_b7e0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b + 0x44, 3);
        e.mem.set_u32(a + 4, 1);
        assert_eq!(e.call(0x00a7_b7e0, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn memory_block_delete_frees_the_whole_chain() {
        let mut e = Engine::new();
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let second = e.mem.alloc(0x14);
        e.mem.set_u32(second + BLOCK_MEMORY, 0x0b00_0000);
        let first = e.mem.alloc(0x14);
        e.mem.set_u32(first + BLOCK_MEMORY, 0x0a00_0000);
        e.mem.set_u32(first + BLOCK_NEXT, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_b880, &args![first, 0u32]).u32(), first);
        assert_eq!(
            calls_to(&e, NI_FREE_00AA10F0),
            vec![vec![0x0a00_0000], vec![0x0b00_0000]]
        );
        // Only the next block is given back: the first was called with flags 0.
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, second]]
        );
    }

    #[test]
    fn viewer_strings_add_the_class_length_and_list() {
        let mut e = Engine::new();
        e.map(NI_TRI_SHAPE_DATA_RTTI, 4);
        e.set_global(NI_TRI_SHAPE_DATA_RTTI, 0x0104_0000u32);
        e.register(TRI_BASED_GEOM_DATA_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| {
            assert_eq!(a[0], TEXT_TRI_LIST_LENGTH);
            (0x0500_0000 + a[1]).into_ret()
        });
        e.register(VIEWER_STRING_POINTER, |_, a| (a[0] ^ a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 0xe, 2);
        let this = shape_data(&mut e, &[0; 3]);
        let list = e.mem.u32(this + 0x48);
        e.call(0x00a7_bd60, &args![this, strings]);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(words, vec![0x0104_0001, 0x0500_0003, TEXT_TRI_LIST ^ list]);
    }
}
