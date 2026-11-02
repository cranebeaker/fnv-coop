//! `NiMain/nitristripsdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

layout! {
    /// `NiTriStripsData` (Xbox PDB), 0x50 bytes: only the fields used so far.
    pub struct NiTriStripsData: 0x50 {
        /// `m_pusStripLists` (Xbox PDB): `u16*`, the strips' vertex indices.
        0x4c m_pusStripLists: Ptr,
    }
}

// Translated from 00a75f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetStripLists` (map name): the strip index array.
pub fn ni_tri_strips_data_get_strip_lists(e: &mut Engine, this: Ptr<NiTriStripsData>) -> Ptr {
    e.get(this, NiTriStripsData::m_pusStripLists)
}

// Translated from 00a75f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x011f_4a70.
pub fn ni_tri_strips_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x011f_4a70
}

/// `NiTriBasedGeomData::m_usTriangles` (Xbox PDB) +0x40.
const TRIANGLES: u32 = 0x40;
/// `NiTriStripsData::m_usStrips` (Xbox PDB) +0x44.
const STRIPS: u32 = 0x44;
/// `NiTriStripsData::m_pusStripLengths` (Xbox PDB) +0x48.
const STRIP_LENGTHS: u32 = 0x48;
/// `NiTriStripsData::m_pusStripLists` (Xbox PDB) +0x4C.
const STRIP_LISTS: u32 = 0x4c;
/// `NiTriStripsData`'s vtable (RTTI `.?AVNiTriStripsData@@`).
const NI_TRI_STRIPS_DATA_VTABLE: u32 = 0x0109_d8ac;
/// `NiGeometryData::GetGroup` (`00a669c0`): the class's static
/// `m_pkGroup` (`NiObjectGroup*`).
const GEOMETRY_DATA_GET_GROUP: u32 = 0x00a6_69c0;
/// `NiObjectGroup::m_pvFree` (Xbox PDB) +8: the next free byte.
const GROUP_FREE: u32 = 0x8;
/// `NiObjectGroup::m_uiRefCount` (Xbox PDB) +0xC.
const GROUP_REF_COUNT: u32 = 0xc;
/// The `NiObjectGroup` destructor body (`00a8a670`, `thiscall`).
const OBJECT_GROUP_DESTRUCTOR: u32 = 0x00a8_a670;
/// The sized delete (`00aa1460`, `cdecl (pointer, size)`).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The Gamebryo allocator's free (`00aa10f0`, `cdecl (pointer)`).
const NI_FREE: u32 = 0x00aa_10f0;
/// The Gamebryo allocator's malloc (`00aa1070`, `cdecl (size)`).
const NI_ALLOC: u32 = 0x00aa_1070;
/// The parent destructor (`00a8a990`, `thiscall`), reached by the tail jump.
const PARENT_DESTRUCTOR: u32 = 0x00a8_a990;
/// `NiTriBasedGeomData::LoadBinary` (`00a8aa30`, `RET 4`).
const PARENT_LOAD_BINARY: u32 = 0x00a8_aa30;
/// `NiTriBasedGeomData::SaveBinary` (`00a8aa70`, `RET 4`).
const PARENT_SAVE_BINARY: u32 = 0x00a8_aa70;
/// `NiObject::GetGroup` (Xbox PDB vtable slot +0x80).
const GET_GROUP_SLOT: u32 = 0x80;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// `NiStream::m_pkIstr` (Xbox PDB) +0x24C: the stream read from.
const STREAM_INPUT: u32 = 0x24c;
/// `NiStream::m_pkOstr` (Xbox PDB) +0x250: the stream written to.
const STREAM_OUTPUT: u32 = 0x250;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB) +4.
const BINARY_STREAM_POSITION: u32 = 0x4;
/// `NiBinaryStream::m_pfnRead` (Xbox PDB) +8, `cdecl (stream, buffer, size,
/// &component size, component count)`, answers the bytes read.
const BINARY_STREAM_READ: u32 = 0x8;
/// `NiBinaryStream::m_pfnWrite` (Xbox PDB) +0xC, same form as the read.
const BINARY_STREAM_WRITE: u32 = 0xc;
/// The first file version that stores whether the strip lists are present.
const HAS_LISTS_FLAG_VERSION: u32 = 0x0a00_0111;

/// One `m_pfnRead`/`m_pfnWrite` call through the stream at `binary`'s
/// function pointer at `slot`, with the component size word at
/// `component_size` set to `width` first, and the position advanced by the
/// bytes it answers.
fn transfer(
    e: &mut Engine,
    binary: u32,
    slot: u32,
    buffer: u32,
    size: u32,
    component_size: u32,
    width: u32,
) {
    let function = e.mem.u32(binary.wrapping_add(slot));
    e.mem.set_u32(component_size, width);
    let got = e
        .call(function, &args![binary, buffer, size, component_size, 1u32])
        .u32();
    let position = binary.wrapping_add(BINARY_STREAM_POSITION);
    let advanced = e.mem.u32(position).wrapping_add(got);
    e.mem.set_u32(position, advanced);
}

/// The allocation of `size` bytes `LoadBinary` makes for an array: from the
/// object's group (vtable +0x80) when it has one (the group's free pointer
/// moves on by `size` and its count goes up), else `NiAlloc`.
fn allocate_array(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    if e.vcall(this.addr(), GET_GROUP_SLOT, &args![]).u32() == 0 {
        return e.call(NI_ALLOC, &args![size]).u32();
    }
    let group = e.vcall(this.addr(), GET_GROUP_SLOT, &args![]).u32();
    let block = e.mem.u32(group.wrapping_add(GROUP_FREE));
    let count = e.mem.u32(group.wrapping_add(GROUP_REF_COUNT));
    e.mem
        .set_u32(group.wrapping_add(GROUP_REF_COUNT), count.wrapping_add(1));
    e.mem
        .set_u32(group.wrapping_add(GROUP_FREE), block.wrapping_add(size));
    block
}

// Translated from 00a75f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::GetActiveTriangleCount` (Xbox PDB, emitted in this
/// unit): `m_usTriangles` (+0x40).
pub fn ni_tri_based_geom_data_get_active_triangle_count(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr().wrapping_add(TRIANGLES))
}

// Translated from 00a75e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetTriangleIndices` (Xbox PDB): the three vertex
/// indices of triangle `triangle` (its low 16 bits). It walks the strips
/// (each of length `L` holds `L - 2` triangles, all counts 16-bit) to the
/// strip holding it, then reads the indices at its position `i` in that
/// strip: `i`, `i + 1`, `i + 2`, the first two swapped when `i` is odd
/// (the strip's alternating winding). No bounds checks: the exe reads past
/// the arrays for an index beyond the last strip.
pub fn ni_tri_strips_data_get_triangle_indices(
    e: &mut Engine,
    this: Ptr,
    triangle: u32,
    first: Ptr,
    second: Ptr,
    third: Ptr,
) {
    let mut list = e.mem.u32(this.addr().wrapping_add(STRIP_LISTS));
    let lengths = e.mem.u32(this.addr().wrapping_add(STRIP_LENGTHS));
    let mut index = triangle;
    let mut length = u32::from(e.mem.u16(lengths));
    let mut triangles = length.wrapping_sub(2) & 0xffff;
    let mut strip = 0u32;
    while index & 0xffff >= triangles {
        index = index.wrapping_sub(triangles);
        list = list.wrapping_add(length.wrapping_mul(2));
        strip = strip.wrapping_add(1);
        length = u32::from(e.mem.u16(lengths.wrapping_add((strip & 0xffff) * 2)));
        triangles = length.wrapping_sub(2) & 0xffff;
    }
    let at = list.wrapping_add((index & 0xffff) * 2);
    let (a, b) = if index & 1 != 0 {
        (e.mem.u16(at.wrapping_add(2)), e.mem.u16(at))
    } else {
        (e.mem.u16(at), e.mem.u16(at.wrapping_add(2)))
    };
    e.mem.set_u16(first.addr(), a);
    e.mem.set_u16(second.addr(), b);
    let c = e.mem.u16(at.wrapping_add(4));
    e.mem.set_u16(third.addr(), c);
}

// Translated from 00a76040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::~NiTriStripsData` (Xbox PDB): puts the class's vtable
/// back; without a geometry-data group (`GetGroup`) frees both strip arrays
/// with `NiFree`, with one releases the group once per array (each release
/// that brings its count to 0 destroys it and frees its 0x10 bytes; the
/// exe asks `GetGroup` again for the second release, even after the first
/// freed it), then runs the parent destructor.
pub fn ni_tri_strips_data_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TRI_STRIPS_DATA_VTABLE);
    if e.call(GEOMETRY_DATA_GET_GROUP, &args![this]).u32() == 0 {
        let lengths = e.mem.u32(this.addr().wrapping_add(STRIP_LENGTHS));
        e.call(NI_FREE, &args![lengths]);
        let lists = e.mem.u32(this.addr().wrapping_add(STRIP_LISTS));
        e.call(NI_FREE, &args![lists]);
    } else {
        for _ in 0..2 {
            let group = e.call(GEOMETRY_DATA_GET_GROUP, &args![this]).u32();
            let count = e
                .mem
                .u32(group.wrapping_add(GROUP_REF_COUNT))
                .wrapping_sub(1);
            e.mem.set_u32(group.wrapping_add(GROUP_REF_COUNT), count);
            if count == 0 {
                e.call(OBJECT_GROUP_DESTRUCTOR, &args![group]);
                e.call(SIZED_DELETE, &args![group, 0x10u32]);
            }
        }
    }
    e.call(PARENT_DESTRUCTOR, &args![this]);
}

// Translated from 00a761a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then the sized delete of 0x50 bytes when bit 0 of `flags`
/// is set. Answers `this`.
pub fn ni_tri_strips_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    ni_tri_strips_data_destructor(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x50u32]);
    }
    this
}

// Translated from 00a761c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::LoadBinary` (Xbox PDB): `NiTriBasedGeomData::LoadBinary`,
/// then the strip count (2 bytes into `m_usStrips`); with strips, the
/// lengths array (`2 * strips` bytes, 2-byte components) into a new array,
/// then, from version `0x0A000111` on, a 1-byte "has lists" flag (earlier
/// files always have them); with lists, the index array of
/// `(2 * strips + m_usTriangles) & 0xFFFF` indices (2 bytes each) into a new
/// array. The arrays come from the object's group when it has one
/// (vtable +0x80), else `NiAlloc`. The flag and the first component size
/// live in the stack word of the stream argument, the other component sizes
/// in a local.
pub fn ni_tri_strips_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PARENT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(8, |e, frame| {
        // +0: the stream argument's stack word; +4: the local.
        let argument = frame.addr();
        let local = argument + 4;
        e.mem.set_u32(argument, stream.addr());
        e.mem.set_u32(local, this.addr());
        let strips_at = this.addr().wrapping_add(STRIPS);
        let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
        transfer(e, input, BINARY_STREAM_READ, strips_at, 2, argument, 2);
        if e.mem.u16(strips_at) == 0 {
            return;
        }
        let size = u32::from(e.mem.u16(strips_at)) * 2;
        let lengths = allocate_array(e, this, size);
        e.mem
            .set_u32(this.addr().wrapping_add(STRIP_LENGTHS), lengths);
        let size = u32::from(e.mem.u16(strips_at)) * 2;
        let lengths = e.mem.u32(this.addr().wrapping_add(STRIP_LENGTHS));
        let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
        transfer(e, input, BINARY_STREAM_READ, lengths, size, local, 2);
        if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) < HAS_LISTS_FLAG_VERSION {
            e.mem.set_u8(argument, 1);
        } else {
            let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
            transfer(e, input, BINARY_STREAM_READ, argument, 1, local, 1);
            if e.mem.u8(argument) == 0 {
                return;
            }
        }
        let count = e
            .mem
            .u16(strips_at)
            .wrapping_mul(2)
            .wrapping_add(e.mem.u16(this.addr().wrapping_add(TRIANGLES)));
        let size = u32::from(count) * 2;
        let lists = allocate_array(e, this, size);
        e.mem.set_u32(this.addr().wrapping_add(STRIP_LISTS), lists);
        let lists = e.mem.u32(this.addr().wrapping_add(STRIP_LISTS));
        let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
        transfer(e, input, BINARY_STREAM_READ, lists, size, local, 2);
    });
}

// Translated from 00a76340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::SaveBinary` (Xbox PDB): `NiTriBasedGeomData::SaveBinary`,
/// then through the output stream's `m_pfnWrite`: the strip count (2
/// bytes); with strips, the lengths (`2 * strips` bytes), a 1-byte flag
/// saying whether the index array is non-null, and with it the indices
/// (`(2 * strips + m_usTriangles) & 0xFFFF` of 2 bytes). The flag and the
/// first component size live in the stream argument's stack word, the
/// other component sizes in a local.
pub fn ni_tri_strips_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PARENT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(8, |e, frame| {
        let argument = frame.addr();
        let local = argument + 4;
        e.mem.set_u32(argument, stream.addr());
        e.mem.set_u32(local, this.addr());
        let strips_at = this.addr().wrapping_add(STRIPS);
        let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
        transfer(e, output, BINARY_STREAM_WRITE, strips_at, 2, argument, 2);
        let strips = e.mem.u16(strips_at);
        if strips == 0 {
            return;
        }
        let lengths = e.mem.u32(this.addr().wrapping_add(STRIP_LENGTHS));
        let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
        transfer(
            e,
            output,
            BINARY_STREAM_WRITE,
            lengths,
            u32::from(strips) * 2,
            local,
            2,
        );
        let has_lists = e.mem.u32(this.addr().wrapping_add(STRIP_LISTS)) != 0;
        let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
        e.mem.set_u8(argument, u8::from(has_lists));
        transfer(e, output, BINARY_STREAM_WRITE, argument, 1, local, 1);
        if e.mem.u8(argument) == 0 {
            return;
        }
        let count = e
            .mem
            .u16(strips_at)
            .wrapping_mul(2)
            .wrapping_add(e.mem.u16(this.addr().wrapping_add(TRIANGLES)));
        let lists = e.mem.u32(this.addr().wrapping_add(STRIP_LISTS));
        let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
        transfer(
            e,
            output,
            BINARY_STREAM_WRITE,
            lists,
            u32::from(count) * 2,
            local,
            2,
        );
    });
}

/// `NiTriStripsData::ms_RTTI` (`011f4a70`): its first word is the class name.
const NI_TRI_STRIPS_DATA_RTTI: u32 = 0x011f_4a70;
/// `NiTriBasedGeomData::IsEqual` (`00a8a9c0`) and `GetViewerStrings` (`00a8aab0`), and
/// `NiGeometryData::GetBlockAllocationSize` (`00a66f90`): thiscall.
const TRI_BASED_GEOM_DATA_IS_EQUAL: u32 = 0x00a8_a9c0;
const TRI_BASED_GEOM_DATA_GET_VIEWER_STRINGS: u32 = 0x00a8_aab0;
const GEOMETRY_DATA_GET_BLOCK_ALLOCATION_SIZE: u32 = 0x00a6_6f90;
/// `NiGetViewerString` (cdecl, the class name), `NiGetViewerString_ov6` (cdecl, a name and
/// a 16-bit value, `"%s = %hu"`) and `NiGetViewerString_ov10` (cdecl, a name and a pointer,
/// `"%s = %p"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_U16: u32 = 0x00a7_d490;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// The strings array's `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_usStrips"`, `"m_pusStripLengths"` and `"m_pusStripLists"`.
const TEXT_STRIPS: u32 = 0x0109_d984;
const TEXT_STRIP_LENGTHS: u32 = 0x0109_d970;
const TEXT_STRIP_LISTS: u32 = 0x0109_d960;

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

/// The number of strip-list entries: twice the strip count plus the triangle count, in
/// 16 bits.
fn strip_list_length(e: &Engine, this: u32) -> u16 {
    let strips = e.mem.u16(this + STRIPS);
    strips
        .wrapping_add(strips)
        .wrapping_add(e.mem.u16(this + TRIANGLES))
}

// Translated from 00a75f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetStripData` (Xbox PDB; thiscall, `RET 0x10`, four out pointers): the
/// strip count (16 bits), the strip lengths (+0x48), the strip lists (+0x4C) and the list
/// length (twice the strips plus the triangles, in 16 bits).
pub fn ni_tri_strips_data_get_strip_data(
    e: &mut Engine,
    this: Ptr,
    strips: u32,
    strip_lengths: u32,
    strip_lists: u32,
    list_length: u32,
) {
    let base = this.addr();
    let count = e.mem.u16(base + STRIPS);
    e.mem.set_u16(strips, count);
    let lengths = e.mem.u32(base + STRIP_LENGTHS);
    e.mem.set_u32(strip_lengths, lengths);
    let lists = e.mem.u32(base + STRIP_LISTS);
    e.mem.set_u32(strip_lists, lists);
    let length = strip_list_length(e, base);
    e.mem.set_u32(list_length, u32::from(length));
}

// Translated from 00a75f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::IsEqual` (Xbox PDB; thiscall, `RET 4`): `NiTriBasedGeomData::IsEqual`,
/// the same strip count, every strip length the same, and every strip-list entry (twice
/// the strips plus this object's triangles, 16 bits) the same.
pub fn ni_tri_strips_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if e.call(TRI_BASED_GEOM_DATA_IS_EQUAL, &args![this, other])
        .u32() as u8
        == 0
    {
        return false;
    }
    let (mine, theirs) = (this.addr(), other.addr());
    let strips = e.mem.u16(mine + STRIPS);
    if strips != e.mem.u16(theirs + STRIPS) {
        return false;
    }
    let count = u32::from(strips);
    if count != 0 {
        let a = e.mem.u32(mine + STRIP_LENGTHS);
        let b = e.mem.u32(theirs + STRIP_LENGTHS);
        for i in 0..count {
            if e.mem.u16(a.wrapping_add(i * 2)) != e.mem.u16(b.wrapping_add(i * 2)) {
                return false;
            }
        }
    }
    let count = u32::from(strip_list_length(e, mine));
    if count != 0 {
        let a = e.mem.u32(mine + STRIP_LISTS);
        let b = e.mem.u32(theirs + STRIP_LISTS);
        for i in 0..count {
            if e.mem.u16(a.wrapping_add(i * 2)) != e.mem.u16(b.wrapping_add(i * 2)) {
                return false;
            }
        }
    }
    true
}

// Translated from 00a76020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetBlockAllocationSize` (Xbox PDB; thiscall, no arguments):
/// `NiGeometryData::GetBlockAllocationSize` plus two bytes for each strip length and each
/// strip-list entry.
pub fn ni_tri_strips_data_get_block_allocation_size(e: &mut Engine, this: Ptr) -> u32 {
    let strips = u32::from(e.mem.u16(this.addr() + STRIPS));
    let length = u32::from(strip_list_length(e, this.addr()));
    let own = (length + strips) * 2;
    e.call(GEOMETRY_DATA_GET_BLOCK_ALLOCATION_SIZE, &args![this])
        .u32()
        .wrapping_add(own)
}

/// Drops one reference to the object's group (vtable +0x80), destroying it
/// (`00a8a670`, then the 0x10-byte sized delete) when none are left.
fn release_group(e: &mut Engine, this: Ptr) {
    let group = e.vcall(this.addr(), GET_GROUP_SLOT, &args![]).u32();
    let count = e.mem.u32(group + GROUP_REF_COUNT).wrapping_sub(1);
    e.mem.set_u32(group + GROUP_REF_COUNT, count);
    if count == 0 {
        e.call(OBJECT_GROUP_DESTRUCTOR, &args![group]);
        e.call(SIZED_DELETE, &args![group, 0x10u32]);
    }
}

// Translated from 00a760c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body replaces the strips (thiscall, `RET 0xc`: the strip
/// count, the strip lengths, the strip lists). A strip array that changes is let go: with
/// a group (vtable +0x80) by dropping one group reference for it, otherwise with
/// `NiFree`. Then the strip count (16 bits), both arrays and a triangle count of the sum
/// of each strip's length less 2 (16 bits) are stored.
pub fn fn_00a760c0(e: &mut Engine, this: Ptr, strips: u32, lengths: u32, lists: u32) {
    let base = this.addr();
    if e.vcall(base, GET_GROUP_SLOT, &args![]).u32() != 0 {
        if lengths != e.mem.u32(base + STRIP_LENGTHS) {
            release_group(e, this);
        }
        if lists != e.mem.u32(base + STRIP_LISTS) {
            release_group(e, this);
        }
    } else {
        let old = e.mem.u32(base + STRIP_LENGTHS);
        if lengths != old {
            e.call(NI_FREE, &args![old]);
        }
        let old = e.mem.u32(base + STRIP_LISTS);
        if lists != old {
            e.call(NI_FREE, &args![old]);
        }
    }
    let strips = strips as u16;
    e.mem.set_u16(base + STRIPS, strips);
    e.mem.set_u32(base + STRIP_LENGTHS, lengths);
    e.mem.set_u32(base + STRIP_LISTS, lists);
    e.mem.set_u16(base + TRIANGLES, 0);
    for i in 0..u32::from(strips) {
        let length = e.mem.u16(lengths.wrapping_add(i * 2)).wrapping_sub(2);
        let triangles = e.mem.u16(base + TRIANGLES).wrapping_add(length);
        e.mem.set_u16(base + TRIANGLES, triangles);
    }
}

// Translated from 00a76430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriBasedGeomData::GetViewerStrings`, then the class name (the first word of
/// `ms_RTTI`), `"m_usStrips"` with the strip count, and `"m_pusStripLengths"` and
/// `"m_pusStripLists"` with the two array addresses, each held in the argument's stack
/// word and added at the array's end.
pub fn ni_tri_strips_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr<NiTArray>) {
    e.call(
        TRI_BASED_GEOM_DATA_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_TRI_STRIPS_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let count = u32::from(e.mem.u16(this.addr() + STRIPS));
        let text = e.call(VIEWER_STRING_U16, &args![TEXT_STRIPS, count]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        for (name, field) in [
            (TEXT_STRIP_LENGTHS, STRIP_LENGTHS),
            (TEXT_STRIP_LISTS, STRIP_LISTS),
        ] {
            let pointer = e.mem.u32(this.addr() + field);
            let text = e.call(VIEWER_STRING_POINTER, &args![name, pointer]).u32();
            e.mem.set_u32(slot.addr(), text);
            add_viewer_string(e, strings, slot.addr());
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a75f20, ni_tri_based_geom_data_get_active_triangle_count(Ptr) -> u16),
        entry!(
            0x00a75e40,
            ni_tri_strips_data_get_triangle_indices(Ptr, u32, Ptr, Ptr, Ptr)
        ),
        entry!(0x00a76040, ni_tri_strips_data_destructor(Ptr)),
        entry!(0x00a761a0, ni_tri_strips_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a761c0, ni_tri_strips_data_load_binary(Ptr, Ptr)),
        entry!(0x00a76340, ni_tri_strips_data_save_binary(Ptr, Ptr)),
        entry!(0x00a75f30, ni_tri_strips_data_get_r_t_t_i() -> u32),
        entry!(
            0x00a75f10,
            ni_tri_strips_data_get_strip_lists(Ptr<NiTriStripsData>) -> Ptr
        ),
        entry!(
            0x00a7_5f40,
            ni_tri_strips_data_get_strip_data(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00a7_5f80, ni_tri_strips_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a7_6020,
            ni_tri_strips_data_get_block_allocation_size(Ptr) -> u32
        ),
        entry!(0x00a7_60c0, fn_00a760c0(Ptr, u32, u32, u32)),
        entry!(
            0x00a7_6430,
            ni_tri_strips_data_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_strip_lists_pointer() {
        let mut e = Engine::new();
        let data: Ptr<NiTriStripsData> = e.new_object();
        e.set(
            data,
            NiTriStripsData::m_pusStripLists,
            Ptr::<()>::new(0x7000),
        );
        assert_eq!(
            e.call(0x00a7_5f10, &args![data]).ptr::<()>(),
            Ptr::<()>::new(0x7000)
        );
    }

    #[test]
    fn ni_tri_strips_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7_5f30, &args![]).u32(), 0x011f_4a70);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Stand-in stream functions at made-up addresses.
    const FAKE_READ: u32 = 0x7100_0000;
    const FAKE_WRITE: u32 = 0x7100_0010;
    /// A vtable whose `GetGroup` (+0x80) answers the word at 0x7300_0000.
    const VTABLE: u32 = 0x7000_0000;
    const GROUP_ANSWER: u32 = 0x7300_0000;
    const GET_GROUP: u32 = 0x7200_0000;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &[0; 0x84 / 4]);
        e.mem.set_u32(VTABLE + GET_GROUP_SLOT, GET_GROUP);
        e.map(GROUP_ANSWER, 4);
        e.register(GET_GROUP, |e, _| Ret {
            eax: e.mem.u32(GROUP_ANSWER),
            ..Ret::default()
        });
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_active_triangle_count_reads_the_triangles() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x50);
        e.mem.set_u16(data + 0x40, 0x1234);
        assert_eq!(e.call(0x00a75f20, &args![data]).u16(), 0x1234);
    }

    #[test]
    fn get_triangle_indices_walks_the_strips_and_flips_odd_triangles() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x50);
        let lengths = e.mem.alloc(8);
        let lists = e.mem.alloc(0x20);
        // Two strips: 4 indices (2 triangles), 5 indices (3 triangles).
        e.mem.set_u16(lengths, 4);
        e.mem.set_u16(lengths + 2, 5);
        for i in 0..9u16 {
            e.mem.set_u16(lists + 2 * u32::from(i), 100 + i);
        }
        e.mem.set_u32(data + 0x48, lengths);
        e.mem.set_u32(data + 0x4c, lists);
        let out = e.mem.alloc(6);
        // (triangle, expected): the first strip, then the second; only the
        // low 16 bits of the triangle count.
        for (triangle, expected) in [
            (0u32, [100u16, 101, 102]),
            (1, [102, 101, 103]),
            (2, [104, 105, 106]),
            (3, [106, 105, 107]),
            (0x1_0004, [106, 107, 108]),
        ] {
            e.call(0x00a75e40, &args![data, triangle, out, out + 2, out + 4]);
            let got = [e.mem.u16(out), e.mem.u16(out + 2), e.mem.u16(out + 4)];
            assert_eq!(got, expected, "triangle {triangle:#x}");
        }
    }

    #[test]
    fn destructor_frees_the_arrays_without_a_group() {
        let mut e = Engine::new();
        e.register(GEOMETRY_DATA_GET_GROUP, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(PARENT_DESTRUCTOR, |_, _| Ret::default());
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(data + 0x48, 0x1111);
        e.mem.set_u32(data + 0x4c, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x00a76040, &args![data]);
        assert_eq!(e.mem.u32(data), NI_TRI_STRIPS_DATA_VTABLE);
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x1111], vec![0x2222]]);
        assert_eq!(calls(&e, PARENT_DESTRUCTOR), vec![vec![data]]);
    }

    #[test]
    fn destructor_releases_the_group_twice() {
        for (count, destroyed) in [(3u32, 0usize), (2, 1)] {
            let mut e = Engine::new();
            let group = e.mem.alloc(0x10);
            e.mem.set_u32(group + 0xc, count);
            e.map(GROUP_ANSWER, 4);
            e.mem.set_u32(GROUP_ANSWER, group);
            e.register(GEOMETRY_DATA_GET_GROUP, |e, _| Ret {
                eax: e.mem.u32(GROUP_ANSWER),
                ..Ret::default()
            });
            e.register(OBJECT_GROUP_DESTRUCTOR, |_, _| Ret::default());
            e.register(SIZED_DELETE, |_, _| Ret::default());
            e.register(PARENT_DESTRUCTOR, |_, _| Ret::default());
            let data = e.mem.alloc(0x50);
            e.call_log = Some(vec![]);
            e.call(0x00a76040, &args![data]);
            assert_eq!(e.mem.u32(group + 0xc), count - 2);
            assert_eq!(calls(&e, OBJECT_GROUP_DESTRUCTOR).len(), destroyed);
            assert_eq!(calls(&e, SIZED_DELETE), vec![vec![group, 0x10]; destroyed]);
            assert!(calls(&e, NI_FREE).is_empty());
            assert_eq!(calls(&e, PARENT_DESTRUCTOR), vec![vec![data]]);
        }
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(GEOMETRY_DATA_GET_GROUP, |_, _| Ret::default());
            e.register(NI_FREE, |_, _| Ret::default());
            e.register(PARENT_DESTRUCTOR, |_, _| Ret::default());
            e.register(SIZED_DELETE, |_, _| Ret::default());
            let data = e.mem.alloc(0x50);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a761a0, &args![data, flags]).u32(), data);
            let deletes = calls(&e, SIZED_DELETE);
            if flags == 1 {
                assert_eq!(deletes, vec![vec![data, 0x50]]);
            } else {
                assert!(deletes.is_empty());
            }
            assert_eq!(calls(&e, PARENT_DESTRUCTOR).len(), 1);
        }
    }

    /// A stream whose read answers successive byte strings, one per call,
    /// logging (size, component size, count) of each.
    fn reading_stream(e: &mut Engine, version: u32, chunks: Vec<Vec<u8>>, log: Log) -> u32 {
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, FAKE_READ);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, input);
        let mut next = chunks.into_iter();
        e.register_double(FAKE_READ, move |e, a| {
            log.borrow_mut().push(vec![a[2], e.mem.u32(a[3]), a[4]]);
            let chunk = next.next().unwrap();
            e.mem.write(a[1], &chunk);
            Ret {
                eax: chunk.len() as u32,
                ..Ret::default()
            }
        });
        stream
    }

    fn allocating(e: &mut Engine) {
        e.register(NI_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
    }

    #[test]
    fn load_binary_reads_strips_lengths_flag_and_lists() {
        let mut e = engine();
        e.register(PARENT_LOAD_BINARY, |_, _| Ret::default());
        allocating(&mut e);
        let log: Log = Rc::default();
        let stream = reading_stream(
            &mut e,
            0x0a00_0111,
            vec![vec![2, 0], vec![3, 0, 4, 0], vec![1], vec![9; 10]],
            log.clone(),
        );
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(data, VTABLE);
        e.mem.set_u16(data + 0x40, 1);
        e.call(0x00a761c0, &args![data, stream]);
        assert_eq!(e.mem.u16(data + 0x44), 2);
        let lengths = e.mem.u32(data + 0x48);
        assert_eq!((e.mem.u16(lengths), e.mem.u16(lengths + 2)), (3, 4));
        let lists = e.mem.u32(data + 0x4c);
        assert_eq!(e.mem.u8(lists + 9), 9);
        assert_eq!(
            *log.borrow(),
            vec![vec![2, 2, 1], vec![4, 2, 1], vec![1, 1, 1], vec![10, 2, 1]]
        );
        assert_eq!(e.mem.u32(e.mem.u32(stream + 0x24c) + 4), 2 + 4 + 1 + 10);
    }

    #[test]
    fn load_binary_without_lists_from_a_group_or_without_strips() {
        // A file with the flag cleared: no index array.
        let mut e = engine();
        e.register(PARENT_LOAD_BINARY, |_, _| Ret::default());
        allocating(&mut e);
        let log: Log = Rc::default();
        let stream = reading_stream(
            &mut e,
            0x0a00_0200,
            vec![vec![1, 0], vec![3, 0], vec![0]],
            log.clone(),
        );
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(data, VTABLE);
        e.call(0x00a761c0, &args![data, stream]);
        assert_eq!(e.mem.u32(data + 0x4c), 0);
        assert_eq!(log.borrow().len(), 3);

        // An old file (no flag) whose arrays come from the object's group.
        let mut e = engine();
        e.register(PARENT_LOAD_BINARY, |_, _| Ret::default());
        let group = e.mem.alloc(0x10);
        let pool = e.mem.alloc(0x40);
        e.mem.set_u32(group + 8, pool);
        e.mem.set_u32(group + 0xc, 1);
        e.mem.set_u32(GROUP_ANSWER, group);
        let log: Log = Rc::default();
        let stream = reading_stream(
            &mut e,
            0x0a00_0110,
            vec![vec![1, 0], vec![5, 0], vec![7; 4]],
            log.clone(),
        );
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(data, VTABLE);
        e.call(0x00a761c0, &args![data, stream]);
        assert_eq!(e.mem.u32(data + 0x48), pool);
        assert_eq!(e.mem.u32(data + 0x4c), pool + 2);
        assert_eq!(e.mem.u32(group + 8), pool + 2 + 4);
        assert_eq!(e.mem.u32(group + 0xc), 3);
        assert_eq!(
            *log.borrow(),
            vec![vec![2, 2, 1], vec![2, 2, 1], vec![4, 2, 1]]
        );

        // No strips: only the count is read.
        let mut e = engine();
        e.register(PARENT_LOAD_BINARY, |_, _| Ret::default());
        let log: Log = Rc::default();
        let stream = reading_stream(&mut e, 0x0a00_0200, vec![vec![0, 0]], log.clone());
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(data, VTABLE);
        e.call(0x00a761c0, &args![data, stream]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn save_binary_writes_what_is_there() {
        for has_lists in [true, false] {
            let mut e = Engine::new();
            e.register(PARENT_SAVE_BINARY, |_, _| Ret::default());
            let log: Log = Rc::default();
            let seen = log.clone();
            e.register_double(FAKE_WRITE, move |e, a| {
                let first = e.mem.u8(a[1]);
                seen.borrow_mut()
                    .push(vec![a[1], a[2], e.mem.u32(a[3]), a[4], u32::from(first)]);
                Ret {
                    eax: a[2],
                    ..Ret::default()
                }
            });
            let stream = e.mem.alloc(0x5c4);
            let output = e.mem.alloc(0x10);
            e.mem.set_u32(output + 0xc, FAKE_WRITE);
            e.mem.set_u32(stream + 0x250, output);
            let data = e.mem.alloc(0x50);
            e.mem.set_u16(data + 0x40, 3);
            e.mem.set_u16(data + 0x44, 2);
            let lengths = e.mem.alloc(4);
            let lists = if has_lists { e.mem.alloc(14) } else { 0 };
            e.mem.set_u32(data + 0x48, lengths);
            e.mem.set_u32(data + 0x4c, lists);
            e.call(0x00a76340, &args![data, stream]);
            let log = log.borrow();
            assert_eq!(log[0][..4], [data + 0x44, 2, 2, 1]);
            assert_eq!(log[1][..4], [lengths, 4, 2, 1]);
            assert_eq!(log[2][1..], [1, 1, 1, u32::from(has_lists)]);
            if has_lists {
                assert_eq!(log[3][..4], [lists, 14, 2, 1]);
                assert_eq!(e.mem.u32(output + 4), 2 + 4 + 1 + 14);
            } else {
                assert_eq!(log.len(), 3);
            }
        }
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    const OBJECT_VTABLE: u32 = 0x0710_0000;
    const GET_GROUP_DOUBLE: u32 = 0x0720_0000;
    /// Where the group double finds the group it answers (0 for none).
    const GROUP_HOLDER: u32 = 0x0730_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn u16_array(e: &mut Engine, values: &[u16]) -> u32 {
        let block = e.mem.alloc(2 * values.len() as u32 + 2);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u16(block + 2 * i as u32, *v);
        }
        block
    }

    /// Strips data with the given lengths and lists (triangles: the sum of each length
    /// less 2).
    fn strips_data(e: &mut Engine, lengths: &[u16], lists: &[u16]) -> u32 {
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, OBJECT_VTABLE);
        e.mem.set_u16(this + STRIPS, lengths.len() as u16);
        let triangles: u16 = lengths.iter().map(|l| l - 2).sum();
        e.mem.set_u16(this + TRIANGLES, triangles);
        let lengths = u16_array(e, lengths);
        e.mem.set_u32(this + STRIP_LENGTHS, lengths);
        let lists = u16_array(e, lists);
        e.mem.set_u32(this + STRIP_LISTS, lists);
        this
    }

    fn object_vtable(e: &mut Engine) {
        let mut slots = vec![0u32; 0x84 / 4];
        slots[GET_GROUP_SLOT as usize / 4] = GET_GROUP_DOUBLE;
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.map(GROUP_HOLDER, 4);
        e.register(GET_GROUP_DOUBLE, |e, _| e.mem.u32(GROUP_HOLDER).into_ret());
    }

    #[test]
    fn strip_data_gives_the_arrays_and_the_list_length() {
        let mut e = Engine::new();
        let this = strips_data(&mut e, &[4, 5], &[0; 9]);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out, 0xffff_ffff);
        e.call(0x00a7_5f40, &args![this, out, out + 4, out + 8, out + 0xc]);
        assert_eq!(e.mem.u32(out), 0xffff_0002);
        assert_eq!(e.mem.u32(out + 4), e.mem.u32(this + STRIP_LENGTHS));
        assert_eq!(e.mem.u32(out + 8), e.mem.u32(this + STRIP_LISTS));
        assert_eq!(e.mem.u32(out + 0xc), 2 * 2 + 5);
        // The length is kept to 16 bits.
        e.mem.set_u16(this + TRIANGLES, 0xfffe);
        e.call(0x00a7_5f40, &args![this, out, out + 4, out + 8, out + 0xc]);
        assert_eq!(e.mem.u32(out + 0xc), 2);
    }

    #[test]
    fn is_equal_compares_strip_lengths_and_lists() {
        let mut e = Engine::new();
        e.register(TRI_BASED_GEOM_DATA_IS_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        let a = strips_data(&mut e, &[3, 4], &[1, 2, 3, 4, 5, 6, 7]);
        let b = strips_data(&mut e, &[3, 4], &[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(e.call(0x00a7_5f80, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u32(a + 4, 1);
        assert_eq!(e.call(0x00a7_5f80, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(a + 4, 0);
        let lists = e.mem.u32(b + STRIP_LISTS);
        e.mem.set_u16(lists + 12, 8);
        assert_eq!(e.call(0x00a7_5f80, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u16(lists + 12, 7);
        let lengths = e.mem.u32(b + STRIP_LENGTHS);
        e.mem.set_u16(lengths + 2, 9);
        assert_eq!(e.call(0x00a7_5f80, &args![a, b]).u32() & 0xff, 0);
        let c = strips_data(&mut e, &[3], &[1, 2, 3]);
        assert_eq!(e.call(0x00a7_5f80, &args![a, c]).u32() & 0xff, 0);
    }

    #[test]
    fn block_allocation_size_adds_two_bytes_per_entry() {
        let mut e = Engine::new();
        e.register(GEOMETRY_DATA_GET_BLOCK_ALLOCATION_SIZE, |_, _| {
            0x100u32.into_ret()
        });
        let this = strips_data(&mut e, &[3, 5], &[0; 8]);
        // Two lengths and 2 * 2 + 4 list entries.
        assert_eq!(e.call(0x00a7_6020, &args![this]).u32(), 0x100 + 2 * (2 + 8));
    }

    #[test]
    fn replacing_strips_frees_changed_arrays_and_counts_triangles() {
        let mut e = Engine::new();
        object_vtable(&mut e);
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(OBJECT_GROUP_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = strips_data(&mut e, &[3], &[0; 3]);
        let old_lengths = e.mem.u32(this + STRIP_LENGTHS);
        let old_lists = e.mem.u32(this + STRIP_LISTS);
        let lengths = u16_array(&mut e, &[4, 6, 1]);
        e.call_log = Some(vec![]);
        e.call(0x00a7_60c0, &args![this, 0x1_0003u32, lengths, old_lists]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![old_lengths]]);
        assert_eq!(e.mem.u16(this + STRIPS), 3);
        assert_eq!(e.mem.u32(this + STRIP_LENGTHS), lengths);
        // 2 + 4 + 0xffff, in 16 bits.
        assert_eq!(e.mem.u16(this + TRIANGLES), 5);
        // With a group: each changed array drops one reference; the last one destroys it.
        let group = e.mem.alloc(0x10);
        e.mem.set_u32(group + GROUP_REF_COUNT, 2);
        e.mem.set_u32(GROUP_HOLDER, group);
        let other = u16_array(&mut e, &[0; 3]);
        e.call_log = Some(vec![]);
        e.call(0x00a7_60c0, &args![this, 0u32, other, other]);
        assert_eq!(e.mem.u32(group + GROUP_REF_COUNT), 0);
        assert_eq!(calls_to(&e, OBJECT_GROUP_DESTRUCTOR), vec![vec![group]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![group, 0x10]]);
        assert!(calls_to(&e, NI_FREE).is_empty());
        assert_eq!(e.mem.u16(this + TRIANGLES), 0);
    }

    #[test]
    fn viewer_strings_add_the_class_the_count_and_both_arrays() {
        let mut e = Engine::new();
        e.map(NI_TRI_STRIPS_DATA_RTTI, 4);
        e.set_global(NI_TRI_STRIPS_DATA_RTTI, 0x0104_0000u32);
        e.register(TRI_BASED_GEOM_DATA_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_U16, |_, a| {
            assert_eq!(a[0], TEXT_STRIPS);
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
        e.mem.set_u16(strings + 0xe, 3);
        let this = strips_data(&mut e, &[3, 3], &[0; 6]);
        let lengths = e.mem.u32(this + STRIP_LENGTHS);
        let lists = e.mem.u32(this + STRIP_LISTS);
        e.call(0x00a7_6430, &args![this, strings]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                0x0104_0001,
                0x0500_0002,
                TEXT_STRIP_LENGTHS ^ lengths,
                TEXT_STRIP_LISTS ^ lists
            ]
        );
    }
}
