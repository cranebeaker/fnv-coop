//! `NiMain/bssegmentedtrishape.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSSegmentedTriShape::ms_RTTI` (the class's `NiRTTI` record).
const BS_SEGMENTED_TRI_SHAPE_RTTI: u32 = 0x011f_5b40;

// Translated from 00a86fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_segmented_tri_shape_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_SEGMENTED_TRI_SHAPE_RTTI)
}

/// `BSSegmentedTriShape`'s vtable (written by its destructor).
const BS_SEGMENTED_TRI_SHAPE_VTABLE: u32 = 0x0109_e834;
/// The `MemoryManager` singleton and its `Allocate` / `Deallocate` (`thiscall` on it, one
/// word), and the sized delete (`cdecl`, `(block, size)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiTriShape`'s destructor body (`thiscall`), `NiTriShape::OnlyRenderImmediate`
/// (`thiscall`, `(renderer)`) and the base `LoadBinary` (`00a98ea0`, `thiscall`,
/// `(stream)`; the map names it `NiParticleMeshes::LoadBinary`, a folded name).
const TRI_SHAPE_DESTRUCTOR: u32 = 0x00a8_9760;
const TRI_SHAPE_ONLY_RENDER_IMMEDIATE: u32 = 0x00a7_4600;
const BASE_LOAD_BINARY: u32 = 0x00a9_8ea0;

layout! {
    /// `BSSegmentedTriShape` (Xbox PDB), 0xD0 bytes on PC (0x100 on the Xbox: its fields sit
    /// 0x2C lower here).
    pub struct BSSegmentedTriShape: 0xD0 {
        /// `NiGeometry::m_spModelData` (Xbox PDB +0xDC): `NiGeometryData*`.
        0xB8 m_spModelData: u32,
        /// `pSegments` (Xbox PDB): `Segment*`, 0x14 bytes each.
        0xC4 pSegments: u32,
        /// `iNumSegments` (Xbox PDB).
        0xC8 iNumSegments: u32,
        /// `bSegmentsChanged` (Xbox PDB).
        0xCC bSegmentsChanged: u8,
    }

    /// `BSSegmentedTriShape::Segment` (Xbox PDB), 0x14 bytes.
    pub struct Segment: 0x14 {
        /// `iStartIndex` (Xbox PDB).
        0x00 iStartIndex: u32,
        /// `iNumPrimitives` (Xbox PDB).
        0x04 iNumPrimitives: u32,
        /// `bEnabled` (Xbox PDB).
        0x08 bEnabled: u8,
        /// `iNumPrimitivesMerged` (Xbox PDB).
        0x0C iNumPrimitivesMerged: u32,
        /// `bEnabledPostMerge` (Xbox PDB).
        0x10 bEnabledPostMerge: u8,
    }
}

/// Segment `index` of the shape's array (read through the array pointer each time, as the
/// exe does).
fn segment(e: &Engine, this: Ptr<BSSegmentedTriShape>, index: u32) -> Ptr<Segment> {
    let segments = e.get(this, BSSegmentedTriShape::pSegments);
    Ptr::new(segments.wrapping_add(index.wrapping_mul(0x14)))
}

/// Reads `size` bytes from the stream's input (`NiStream::m_pkIstr` +0x24C) into
/// `destination`: the input's read function (`NiBinaryStream::m_pfnRead` +0x08) is called
/// as `function(input, destination, size, &count, 1)` with `count` set to `size`, and the
/// byte count it answers is added to the input's position (+0x04).
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(0x24c));
    let function = e.mem.u32(input.wrapping_add(8));
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        e.call(function, &args![input, destination, size, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(input.wrapping_add(4));
    e.mem
        .set_u32(input.wrapping_add(4), position.wrapping_add(done));
}

// Translated from 00a86ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::EnableSegment` (Xbox PDB): the segment is enabled when it has
/// primitives, and the segments are marked changed.
pub fn bs_segmented_tri_shape_enable_segment(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    index: u32,
) {
    let segment = segment(e, this, index);
    let enabled = e.get(segment, Segment::iNumPrimitives) != 0;
    e.set(segment, Segment::bEnabled, u8::from(enabled));
    e.set(this, BSSegmentedTriShape::bSegmentsChanged, 1);
}

// Translated from 00a87020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::DisableSegment` (Xbox PDB): the segment is disabled and the
/// segments are marked changed.
pub fn bs_segmented_tri_shape_disable_segment(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    index: u32,
) {
    let segment = segment(e, this, index);
    e.set(segment, Segment::bEnabled, 0);
    e.set(this, BSSegmentedTriShape::bSegmentsChanged, 1);
}

// Translated from 00a87040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::MergeEnabledSegments` (Xbox PDB): when the segments changed, from
/// the last segment backwards each one's post-merge state is set. The last: enabled as
/// `bEnabled` with its own primitive count when it has primitives, else disabled. Each
/// earlier one is merged with the next (the next's post-merge flag cleared, this one
/// enabled, its merged count its own plus the next's merged count, and without primitives
/// its start index taken from the next) when both are enabled (the next by its post-merge
/// flag), or when it has no primitives and the next is enabled; otherwise it keeps
/// `bEnabled` and its own count when it has primitives, or is disabled. The changed flag is
/// cleared.
pub fn bs_segmented_tri_shape_merge_enabled_segments(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
) {
    if e.get(this, BSSegmentedTriShape::bSegmentsChanged) == 0 {
        return;
    }
    let count = e.get(this, BSSegmentedTriShape::iNumSegments);
    let last = segment(e, this, count.wrapping_sub(1));
    let primitives = e.get(last, Segment::iNumPrimitives);
    if primitives == 0 {
        e.set(last, Segment::bEnabledPostMerge, 0);
    } else {
        e.set(last, Segment::iNumPrimitivesMerged, primitives);
        let enabled = e.get(last, Segment::bEnabled);
        e.set(last, Segment::bEnabledPostMerge, enabled);
    }
    let mut index = e
        .get(this, BSSegmentedTriShape::iNumSegments)
        .wrapping_sub(2) as i32;
    while index >= 0 {
        let current = segment(e, this, index as u32);
        let next = segment(e, this, index as u32 + 1);
        let enabled = e.get(current, Segment::bEnabled);
        let primitives = e.get(current, Segment::iNumPrimitives);
        let merge = if enabled != 0 && e.get(next, Segment::bEnabledPostMerge) != 0 {
            true
        } else {
            primitives == 0 && e.get(next, Segment::bEnabledPostMerge) != 0
        };
        if merge {
            e.set(next, Segment::bEnabledPostMerge, 0);
            let current = segment(e, this, index as u32);
            e.set(current, Segment::bEnabledPostMerge, 1);
            let next = segment(e, this, index as u32 + 1);
            let merged = e
                .get(next, Segment::iNumPrimitivesMerged)
                .wrapping_add(e.get(current, Segment::iNumPrimitives));
            e.set(current, Segment::iNumPrimitivesMerged, merged);
            if e.get(current, Segment::iNumPrimitives) == 0 {
                let start = e.get(next, Segment::iStartIndex);
                e.set(current, Segment::iStartIndex, start);
            }
        } else if primitives != 0 {
            e.set(current, Segment::bEnabledPostMerge, enabled);
            let current = segment(e, this, index as u32);
            let own = e.get(current, Segment::iNumPrimitives);
            e.set(current, Segment::iNumPrimitivesMerged, own);
        } else {
            e.set(current, Segment::bEnabledPostMerge, 0);
        }
        index -= 1;
    }
    e.set(this, BSSegmentedTriShape::bSegmentsChanged, 0);
}

// Translated from 00a87130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::OnlyRenderImmediate` (Xbox PDB), `(renderer)`: the enabled
/// segments are merged, then `NiTriShape::OnlyRenderImmediate` runs with the same argument.
pub fn bs_segmented_tri_shape_only_render_immediate(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    renderer: Ptr,
) {
    bs_segmented_tri_shape_merge_enabled_segments(e, this);
    e.call(TRI_SHAPE_ONLY_RENDER_IMMEDIATE, &args![this, renderer]);
}

// Translated from 00a87290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::_scalar_deleting_destructor_` (Xbox PDB): the class's vtable, the
/// segments freed (`MemoryManager::Deallocate`), `NiTriShape`'s destructor body, then the
/// sized delete of 0xD0 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn bs_segmented_tri_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    flags: u32,
) -> Ptr<BSSegmentedTriShape> {
    let segments = e.get(this, BSSegmentedTriShape::pSegments);
    e.mem.set_u32(this.addr(), BS_SEGMENTED_TRI_SHAPE_VTABLE);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, segments]);
    e.call(TRI_SHAPE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xd0u32]);
    }
    this
}

// Translated from 00a872d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::EnableAllSegments` (Xbox PDB): the first segment becomes one
/// enabled merged segment covering all the model data's triangles
/// (`NiTriBasedGeomData::m_usTriangles`, +0x40), every later one is disabled after the
/// merge and enabled when it has primitives, and the changed flag is cleared.
pub fn bs_segmented_tri_shape_enable_all_segments(e: &mut Engine, this: Ptr<BSSegmentedTriShape>) {
    let first = segment(e, this, 0);
    e.set(first, Segment::bEnabledPostMerge, 1);
    let data = e.get(this, BSSegmentedTriShape::m_spModelData);
    let triangles = u32::from(e.mem.u16(data.wrapping_add(0x40)));
    let first = segment(e, this, 0);
    e.set(first, Segment::iNumPrimitivesMerged, triangles);
    let mut index = 1u32;
    if e.get(this, BSSegmentedTriShape::iNumSegments) > 1 {
        loop {
            let current = segment(e, this, index);
            e.set(current, Segment::bEnabledPostMerge, 0);
            let current = segment(e, this, index);
            if e.get(current, Segment::iNumPrimitives) != 0 {
                e.set(current, Segment::bEnabled, 1);
            }
            index += 1;
            if index >= e.get(this, BSSegmentedTriShape::iNumSegments) {
                break;
            }
        }
    }
    e.set(this, BSSegmentedTriShape::bSegmentsChanged, 0);
}

// Translated from 00a873b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::LoadBinary` (Xbox PDB): the base `LoadBinary`, the old segments
/// freed when present, the segment count (4 bytes) read and `count * 0x14` bytes allocated
/// (all ones when the product overflows); for each segment a byte read into a local and
/// dropped, the start index and primitive count (4 bytes each), the segment disabled, and
/// its merged count its primitive count (both the merged count and the start index 0
/// without primitives). The segments are marked changed.
pub fn bs_segmented_tri_shape_load_binary(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    stream: Ptr,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let old = e.get(this, BSSegmentedTriShape::pSegments);
    if old != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
    }
    stream_read(e, stream, this.addr() + 0xc8, 4);
    let count = e.get(this, BSSegmentedTriShape::iNumSegments);
    let bytes = count.checked_mul(0x14).unwrap_or(u32::MAX);
    let segments = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    e.set(this, BSSegmentedTriShape::pSegments, segments);
    let mut index = 0u32;
    if e.get(this, BSSegmentedTriShape::iNumSegments) != 0 {
        loop {
            e.with_stack(4, |e, local| stream_read(e, stream, local.addr(), 1));
            let current = segment(e, this, index);
            stream_read(e, stream, current.addr(), 4);
            let current = segment(e, this, index);
            stream_read(e, stream, current.addr() + 4, 4);
            let current = segment(e, this, index);
            e.set(current, Segment::bEnabled, 0);
            let current = segment(e, this, index);
            let primitives = e.get(current, Segment::iNumPrimitives);
            e.set(current, Segment::iNumPrimitivesMerged, primitives);
            if primitives == 0 {
                let current = segment(e, this, index);
                e.set(current, Segment::iStartIndex, 0);
            }
            index += 1;
            if index >= e.get(this, BSSegmentedTriShape::iNumSegments) {
                break;
            }
        }
    }
    e.set(this, BSSegmentedTriShape::bSegmentsChanged, 1);
}

/// `operator new` (`cdecl`, `(size)`) and the `NiTriShape` default constructor (`00a744a0`,
/// `thiscall`, no arguments).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_44a0;

// Translated from 00a87140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSegmentedTriShape` factory (Ghidra made no function here and the map has no row;
/// placed in this unit because it builds this class, its vtable `0109e834` confirmed by the
/// RTTI): a 0xD0-byte block (`operator new`) constructed as an `NiTriShape` (`00a744a0`),
/// then the class's vtable, no segments, count 0, not changed. Null when the allocation
/// fails.
pub fn fn_00a87140(e: &mut Engine) -> Ptr<BSSegmentedTriShape> {
    let block = e.call(OPERATOR_NEW, &args![0xd0u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let this = Ptr::<BSSegmentedTriShape>::new(block);
    e.call(TRI_SHAPE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BS_SEGMENTED_TRI_SHAPE_VTABLE);
    e.set(this, BSSegmentedTriShape::pSegments, 0);
    e.set(this, BSSegmentedTriShape::iNumSegments, 0);
    e.set(this, BSSegmentedTriShape::bSegmentsChanged, 0);
    this
}

/// `NiTriShape::RenderImmediate` (`00a745c0`, thiscall: renderer), the member copy
/// (`00a744c0`, thiscall on the source: clone, cloning process), the base `SaveBinary`
/// (`00a71d10`, thiscall: stream; the map names it `NiParticleMeshes::SaveBinary`, a folded
/// name) and `NiTriShape::GetViewerStrings` (`00a74690`, thiscall: string array).
const TRI_SHAPE_RENDER_IMMEDIATE: u32 = 0x00a7_45c0;
const TRI_SHAPE_COPY_MEMBERS: u32 = 0x00a7_44c0;
const BASE_SAVE_BINARY: u32 = 0x00a7_1d10;
const TRI_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00a7_4690;
/// The string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`), `NiGetViewerString`
/// (`00a7d330`) and `NiGetViewerString_ov8` (`00a7d510`, label and unsigned word).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
/// `"Num Segments"`, `"Enabled Segment"`, `"Disabled Segment"`, `"Empty Segments"`.
const TEXT_NUM_SEGMENTS: u32 = 0x0109_e95c;
const TEXT_ENABLED_SEGMENT: u32 = 0x0109_e94c;
const TEXT_DISABLED_SEGMENT: u32 = 0x0109_e938;
const TEXT_EMPTY_SEGMENTS: u32 = 0x0109_e928;

/// One write through the stream's output (`NiStream` +0x250, write function at +0xc,
/// cdecl: output, source, size, element sizes, element count; the answer added to the
/// output's position at +4) of one element whose size is in the stack word `sizes`.
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, size: u32, sizes: u32) {
    let output = e.mem.u32(stream.addr().wrapping_add(0x250));
    let function = e.mem.u32(output.wrapping_add(0xc));
    e.mem.set_u32(sizes, size);
    let done = e
        .call(function, &args![output, source, size, sizes, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(output.wrapping_add(4), position);
}

/// Adds the string word at `slot` at the array's end (`SetSize` when full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(
            STRING_ARRAY_SET_SIZE,
            &args![strings, grow.wrapping_add(index)],
        );
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a87120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::RenderImmediate` (Xbox PDB), `(renderer)`: the enabled segments
/// are merged, then a jump to `NiTriShape::RenderImmediate` (`00a745c0`) with the same
/// argument (the merge keeps `ECX`).
pub fn bs_segmented_tri_shape_render_immediate(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    renderer: Ptr,
) {
    bs_segmented_tri_shape_merge_enabled_segments(e, this);
    e.call(TRI_SHAPE_RENDER_IMMEDIATE, &args![this, renderer]);
}

// Translated from 00a871a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape`'s member copy (unnamed in the engine map; thiscall on the source,
/// `RET 8`: clone, cloning process): the base copy (`00a744c0`), the clone's old segments
/// given back (`MemoryManager::Deallocate`, the pointer not cleared), then with source
/// segments their count and changed flag and a new array (`count * 0x14` bytes, all ones
/// when the product overflows) filled word by word; without them the clone gets no
/// segments and a zero count.
pub fn fn_00a871a0(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    clone: Ptr<BSSegmentedTriShape>,
    cloning: Ptr,
) {
    e.call(TRI_SHAPE_COPY_MEMBERS, &args![this, clone, cloning]);
    let old = e.get(clone, BSSegmentedTriShape::pSegments);
    if old != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
    }
    if e.get(this, BSSegmentedTriShape::pSegments) == 0 {
        e.set(clone, BSSegmentedTriShape::pSegments, 0);
        e.set(clone, BSSegmentedTriShape::iNumSegments, 0);
        return;
    }
    let count = e.get(this, BSSegmentedTriShape::iNumSegments);
    e.set(clone, BSSegmentedTriShape::iNumSegments, count);
    let changed = e.get(this, BSSegmentedTriShape::bSegmentsChanged);
    e.set(clone, BSSegmentedTriShape::bSegmentsChanged, changed);
    let count = e.get(this, BSSegmentedTriShape::iNumSegments);
    let bytes = count.checked_mul(0x14).unwrap_or(u32::MAX);
    let segments = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    e.set(clone, BSSegmentedTriShape::pSegments, segments);
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < e.get(this, BSSegmentedTriShape::iNumSegments) {
        let source = e
            .get(this, BSSegmentedTriShape::pSegments)
            .wrapping_add(offset);
        let target = e
            .get(clone, BSSegmentedTriShape::pSegments)
            .wrapping_add(offset);
        for word in [0u32, 4, 8, 0xc, 0x10] {
            let value = e.mem.u32(source.wrapping_add(word));
            e.mem.set_u32(target.wrapping_add(word), value);
        }
        index = index.wrapping_add(1);
        offset = offset.wrapping_add(0x14);
    }
}

// Translated from 00a87340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::CreateClone` (Xbox PDB; thiscall, `RET 4`: cloning process): a
/// 0xd0-byte block (`operator new`) constructed as an `NiTriShape` (`00a744a0`) with the
/// class's vtable, no segments, count 0 and not changed, then the members copied into it
/// ([`fn_00a871a0`]). When the allocation fails the copy still runs, into null, and the
/// answer is null.
pub fn bs_segmented_tri_shape_create_clone(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    cloning: Ptr,
) -> Ptr<BSSegmentedTriShape> {
    let block = e.call(OPERATOR_NEW, &args![0xd0u32]).u32();
    if block == 0 {
        fn_00a871a0(e, this, Ptr::new(0), cloning);
        return Ptr::new(0);
    }
    let clone = Ptr::<BSSegmentedTriShape>::new(block);
    e.call(TRI_SHAPE_CONSTRUCTOR, &args![clone]);
    e.mem.set_u32(block, BS_SEGMENTED_TRI_SHAPE_VTABLE);
    e.set(clone, BSSegmentedTriShape::pSegments, 0);
    e.set(clone, BSSegmentedTriShape::iNumSegments, 0);
    e.set(clone, BSSegmentedTriShape::bSegmentsChanged, 0);
    fn_00a871a0(e, this, clone, cloning);
    clone
}

// Translated from 00a87520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::SaveBinary` (Xbox PDB; thiscall, `RET 4`; the counterpart of
/// [`bs_segmented_tri_shape_load_binary`]): the base `SaveBinary` (`00a71d10`), the segment
/// count (4 bytes), then per segment a zero byte (the low byte of the argument's stack
/// word; its element size in a local word), the start index and the primitive count (4
/// bytes each, the element size in the argument word).
pub fn bs_segmented_tri_shape_save_binary(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    stream: Ptr,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    // The local word (+0) and the argument word (+4).
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = frame.addr() + 4;
        e.mem.set_u32(argument, stream.addr());
        stream_write(e, stream, this.addr() + 0xc8, 4, argument);
        let mut index = 0u32;
        let mut offset = 0u32;
        while index < e.get(this, BSSegmentedTriShape::iNumSegments) {
            e.mem.set_u8(argument, 0);
            stream_write(e, stream, argument, 1, local);
            let segments = e.get(this, BSSegmentedTriShape::pSegments);
            stream_write(e, stream, segments.wrapping_add(offset), 4, argument);
            let segments = e.get(this, BSSegmentedTriShape::pSegments);
            stream_write(
                e,
                stream,
                segments.wrapping_add(offset).wrapping_add(4),
                4,
                argument,
            );
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(0x14);
        }
    });
}

// Translated from 00a87620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriShape::GetViewerStrings` (`00a74690`), the class name (the first word of
/// `ms_RTTI`, `011f5b40`) and `"Num Segments"`; with a segment array, each segment with
/// primitives as `"Enabled Segment"` or `"Disabled Segment"` with its index, then
/// `"Empty Segments"` with the number of segments without primitives. Each string is added
/// through `SetSize`/`SetAt`; the segment strings are held in a local word, the others in
/// the argument word (which holds the running segment offset in between).
pub fn bs_segmented_tri_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSSegmentedTriShape>,
    strings: Ptr,
) {
    let list = strings.addr();
    e.call(TRI_SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    // The empty count (+0), the segment string (+4) and the argument word (+8).
    e.with_stack(0xc, |e, frame| {
        let empty = frame.addr();
        let local = frame.addr() + 4;
        let argument = frame.addr() + 8;
        let class_name = e.global::<u32>(BS_SEGMENTED_TRI_SHAPE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(argument, text);
        add_viewer_string(e, list, argument);
        let count = e.get(this, BSSegmentedTriShape::iNumSegments);
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_NUM_SEGMENTS, count])
            .u32();
        e.mem.set_u32(argument, text);
        add_viewer_string(e, list, argument);
        if e.get(this, BSSegmentedTriShape::pSegments) == 0 {
            return;
        }
        e.mem.set_u32(empty, 0);
        let mut index = 0u32;
        if e.get(this, BSSegmentedTriShape::iNumSegments) != 0 {
            e.mem.set_u32(argument, 0);
            loop {
                let current = e
                    .mem
                    .u32(argument)
                    .wrapping_add(e.get(this, BSSegmentedTriShape::pSegments));
                if e.mem.u32(current.wrapping_add(4)) == 0 {
                    let count = e.mem.u32(empty).wrapping_add(1);
                    e.mem.set_u32(empty, count);
                } else {
                    let label = if e.mem.u8(current.wrapping_add(8)) != 0 {
                        TEXT_ENABLED_SEGMENT
                    } else {
                        TEXT_DISABLED_SEGMENT
                    };
                    let text = e.call(VIEWER_STRING_UNSIGNED, &args![label, index]).u32();
                    e.mem.set_u32(local, text);
                    add_viewer_string(e, list, local);
                }
                let offset = e.mem.u32(argument).wrapping_add(0x14);
                e.mem.set_u32(argument, offset);
                index = index.wrapping_add(1);
                if index >= e.get(this, BSSegmentedTriShape::iNumSegments) {
                    break;
                }
            }
        }
        let count = e.mem.u32(empty);
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_EMPTY_SEGMENTS, count])
            .u32();
        e.mem.set_u32(argument, text);
        add_viewer_string(e, list, argument);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_7120,
            bs_segmented_tri_shape_render_immediate(Ptr<BSSegmentedTriShape>, Ptr)
        ),
        entry!(
            0x00a8_71a0,
            fn_00a871a0(Ptr<BSSegmentedTriShape>, Ptr<BSSegmentedTriShape>, Ptr)
        ),
        entry!(
            0x00a8_7340,
            bs_segmented_tri_shape_create_clone(
                Ptr<BSSegmentedTriShape>,
                Ptr,
            ) -> Ptr<BSSegmentedTriShape>
        ),
        entry!(
            0x00a8_7520,
            bs_segmented_tri_shape_save_binary(Ptr<BSSegmentedTriShape>, Ptr)
        ),
        entry!(
            0x00a8_7620,
            bs_segmented_tri_shape_get_viewer_strings(Ptr<BSSegmentedTriShape>, Ptr)
        ),
        entry!(0x00a86fe0, bs_segmented_tri_shape_get_rtti() -> Ptr),
        entry!(
            0x00a86ff0,
            bs_segmented_tri_shape_enable_segment(Ptr<BSSegmentedTriShape>, u32)
        ),
        entry!(
            0x00a87020,
            bs_segmented_tri_shape_disable_segment(Ptr<BSSegmentedTriShape>, u32)
        ),
        entry!(
            0x00a87040,
            bs_segmented_tri_shape_merge_enabled_segments(Ptr<BSSegmentedTriShape>)
        ),
        entry!(
            0x00a87130,
            bs_segmented_tri_shape_only_render_immediate(Ptr<BSSegmentedTriShape>, Ptr)
        ),
        entry!(
            0x00a87290,
            bs_segmented_tri_shape_scalar_deleting_destructor(
                Ptr<BSSegmentedTriShape>,
                u32,
            )
                -> Ptr<BSSegmentedTriShape>
        ),
        entry!(
            0x00a872d0,
            bs_segmented_tri_shape_enable_all_segments(Ptr<BSSegmentedTriShape>)
        ),
        entry!(
            0x00a873b0,
            bs_segmented_tri_shape_load_binary(Ptr<BSSegmentedTriShape>, Ptr)
        ),
        entry!(0x00a87140, fn_00a87140() -> Ptr<BSSegmentedTriShape>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_segmented_tri_shape_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a86fe0, &args![]).u32(),
            BS_SEGMENTED_TRI_SHAPE_RTTI
        );
    }

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A shape with segments of the given (start, primitives, enabled) values.
    fn shape(e: &mut Engine, segments: &[(u32, u32, u8)]) -> Ptr<BSSegmentedTriShape> {
        let this = e.new_object::<BSSegmentedTriShape>();
        let array = e.mem.alloc(0x14 * segments.len() as u32);
        for (i, (start, primitives, enabled)) in segments.iter().enumerate() {
            let s = Ptr::<Segment>::new(array + 0x14 * i as u32);
            e.set(s, Segment::iStartIndex, *start);
            e.set(s, Segment::iNumPrimitives, *primitives);
            e.set(s, Segment::bEnabled, *enabled);
        }
        e.set(this, BSSegmentedTriShape::pSegments, array);
        e.set(
            this,
            BSSegmentedTriShape::iNumSegments,
            segments.len() as u32,
        );
        this
    }

    /// (merged count, post-merge flag, start) of every segment.
    fn merged(e: &Engine, this: Ptr<BSSegmentedTriShape>) -> Vec<(u32, u8, u32)> {
        (0..e.get(this, BSSegmentedTriShape::iNumSegments))
            .map(|i| {
                let s = segment(e, this, i);
                (
                    e.get(s, Segment::iNumPrimitivesMerged),
                    e.get(s, Segment::bEnabledPostMerge),
                    e.get(s, Segment::iStartIndex),
                )
            })
            .collect()
    }

    #[test]
    fn enable_and_disable_segment_mark_the_segments_changed() {
        let mut e = Engine::new();
        let this = shape(&mut e, &[(0, 3, 0), (3, 0, 0)]);
        e.call(0x00a86ff0, &args![this, 0u32]);
        assert_eq!(e.get(segment(&e, this, 0), Segment::bEnabled), 1);
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 1);
        e.call(0x00a86ff0, &args![this, 1u32]);
        assert_eq!(e.get(segment(&e, this, 1), Segment::bEnabled), 0);
        e.set(this, BSSegmentedTriShape::bSegmentsChanged, 0);
        e.call(0x00a87020, &args![this, 0u32]);
        assert_eq!(e.get(segment(&e, this, 0), Segment::bEnabled), 0);
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 1);
    }

    #[test]
    fn merge_joins_enabled_neighbours_and_empty_segments() {
        let mut e = Engine::new();
        // 0: enabled 2, 1: enabled 3, 2: empty, 3: enabled 4, 4: disabled 5.
        let this = shape(
            &mut e,
            &[(0, 2, 1), (2, 3, 1), (9, 0, 0), (5, 4, 1), (9, 5, 0)],
        );
        e.call(0x00a87040, &args![this]);
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 0);
        // Nothing changed: no work.
        let before = merged(&e, this);
        assert_eq!(
            before,
            vec![(0, 0, 0), (0, 0, 2), (0, 0, 9), (0, 0, 5), (0, 0, 9)]
        );
        e.set(this, BSSegmentedTriShape::bSegmentsChanged, 1);
        e.call(0x00a87040, &args![this]);
        assert_eq!(
            merged(&e, this),
            vec![
                (2 + 3 + 4, 1, 0),
                (3 + 4, 0, 2),
                (4, 0, 5),
                (4, 0, 5),
                (5, 0, 9)
            ]
        );
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 0);
        // An empty last segment is disabled; a disabled segment with primitives keeps its
        // own count; an empty one before a disabled one is disabled.
        let other = shape(&mut e, &[(0, 2, 0), (2, 0, 0), (2, 0, 1)]);
        e.set(other, BSSegmentedTriShape::bSegmentsChanged, 1);
        e.call(0x00a87040, &args![other]);
        assert_eq!(merged(&e, other), vec![(2, 0, 0), (0, 0, 2), (0, 0, 2)]);
    }

    #[test]
    fn only_render_immediate_merges_then_renders() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TRI_SHAPE_ONLY_RENDER_IMMEDIATE, |_, _| Ret::default());
        let this = shape(&mut e, &[(0, 2, 1)]);
        e.set(this, BSSegmentedTriShape::bSegmentsChanged, 1);
        e.call(0x00a87130, &args![this, 0x7777u32]);
        assert_eq!(merged(&e, this), vec![(2, 1, 0)]);
        assert_eq!(
            calls_at(&e, TRI_SHAPE_ONLY_RENDER_IMMEDIATE),
            vec![vec![this.addr(), 0x7777]]
        );
    }

    #[test]
    fn destructor_frees_the_segments_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for addr in [MEMORY_DEALLOCATE, TRI_SHAPE_DESTRUCTOR, SIZED_DELETE] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = shape(&mut e, &[(0, 2, 1)]);
        let array = e.get(this, BSSegmentedTriShape::pSegments);
        assert_eq!(e.call(0x00a87290, &args![this, 2u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BS_SEGMENTED_TRI_SHAPE_VTABLE);
        assert_eq!(
            calls_at(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
        assert!(calls_at(&e, SIZED_DELETE).is_empty());
        e.call(0x00a87290, &args![this, 1u32]);
        assert_eq!(calls_at(&e, SIZED_DELETE), vec![vec![this.addr(), 0xd0]]);
        assert_eq!(calls_at(&e, TRI_SHAPE_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn enable_all_segments_covers_every_triangle_with_the_first() {
        let mut e = Engine::new();
        let this = shape(&mut e, &[(0, 2, 0), (2, 3, 0), (5, 0, 0)]);
        let data = e.mem.alloc(0x44);
        e.mem.set_u16(data + 0x40, 77);
        e.set(this, BSSegmentedTriShape::m_spModelData, data);
        e.set(this, BSSegmentedTriShape::bSegmentsChanged, 1);
        e.set(segment(&e, this, 1), Segment::bEnabledPostMerge, 1);
        e.call(0x00a872d0, &args![this]);
        assert_eq!(merged(&e, this), vec![(77, 1, 0), (0, 0, 2), (0, 0, 5)]);
        let enabled: Vec<u8> = (0..3)
            .map(|i| e.get(segment(&e, this, i), Segment::bEnabled))
            .collect();
        assert_eq!(enabled, vec![0, 1, 0]);
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 0);
    }

    #[test]
    fn load_binary_reads_the_segments() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        // Reads answer, in order: count 2; then per segment a byte, start, primitives.
        let mut values = vec![0u32, 7, 1, 3, 5, 0xff, 2].into_iter();
        let stream = e.mem.alloc(0x250);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0090_0300);
        e.register_double(0x0090_0300, move |e, a| {
            let value = values.next_back().unwrap();
            match a[2] {
                1 => e.mem.set_u8(a[1], value as u8),
                _ => e.mem.set_u32(a[1], value),
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let this = shape(&mut e, &[(0, 1, 1)]);
        let old = e.get(this, BSSegmentedTriShape::pSegments);
        e.call(0x00a873b0, &args![this, stream]);
        assert_eq!(
            calls_at(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, old]]
        );
        assert_eq!(
            calls_at(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x28]]
        );
        assert_eq!(e.get(this, BSSegmentedTriShape::iNumSegments), 2);
        assert_eq!(merged(&e, this), vec![(3, 0, 5), (0, 0, 0)]);
        assert_eq!(e.get(segment(&e, this, 0), Segment::iNumPrimitives), 3);
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 1);
        assert_eq!(e.mem.u32(input + 4), 4 + 9 + 9);
    }

    #[test]
    fn factory_constructs_an_empty_segmented_shape() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(TRI_SHAPE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0xc4, 0xdead);
            e.mem.set_u8(a[0] + 0xcc, 1);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let made = e.call(0x00a87140, &args![]).u32();
        let this = Ptr::<BSSegmentedTriShape>::new(made);
        assert_eq!(e.mem.u32(made), BS_SEGMENTED_TRI_SHAPE_VTABLE);
        assert_eq!(e.get(this, BSSegmentedTriShape::pSegments), 0);
        assert_eq!(e.get(this, BSSegmentedTriShape::bSegmentsChanged), 0);
        assert_eq!(calls_at(&e, OPERATOR_NEW), vec![vec![0xd0]]);
        assert_eq!(calls_at(&e, TRI_SHAPE_CONSTRUCTOR), vec![vec![made]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a87140, &args![]).u32(), 0);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A shape with `count` segments whose words are `0x100 * segment + word index`.
    fn shape(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(0xd0);
        let segments = e.mem.alloc(0x14 * count.max(1));
        for i in 0..count * 5 {
            e.mem.set_u32(segments + 4 * i, 0x100 * (i / 5) + i % 5);
        }
        e.mem.set_u32(this + 0xc4, segments);
        e.mem.set_u32(this + 0xc8, count);
        this
    }

    #[test]
    fn render_immediate_merges_then_renders() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_RENDER_IMMEDIATE, |e, a| {
            // The merge ran first: the changed flag is clear.
            assert_eq!(e.mem.u8(a[0] + 0xcc), 0);
            Ret::default()
        });
        let this = shape(&mut e, 1);
        e.mem.set_u8(this + 0xcc, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a8_7120, &args![this, 0x5000u32]);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_RENDER_IMMEDIATE),
            vec![vec![this, 0x5000]]
        );
    }

    #[test]
    fn copy_members_copies_the_segments_or_clears_them() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let this = shape(&mut e, 2);
        e.mem.set_u8(this + 0xcc, 1);
        let clone = shape(&mut e, 1);
        let old = e.mem.u32(clone + 0xc4);
        e.call_log = Some(vec![]);
        e.call(0x00a8_71a0, &args![this, clone, 0x6000u32]);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_COPY_MEMBERS),
            vec![vec![this, clone, 0x6000]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, old]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x28]]
        );
        let copy = e.mem.u32(clone + 0xc4);
        assert_ne!(copy, old);
        assert_eq!(e.mem.u32(clone + 0xc8), 2);
        assert_eq!(e.mem.u8(clone + 0xcc), 1);
        let source = e.mem.u32(this + 0xc4);
        assert_eq!(e.mem.bytes(copy, 0x28), e.mem.bytes(source, 0x28));
        // A source without segments.
        e.mem.set_u32(this + 0xc4, 0);
        e.call(0x00a8_71a0, &args![this, clone, 0x6000u32]);
        assert_eq!((e.mem.u32(clone + 0xc4), e.mem.u32(clone + 0xc8)), (0, 0));
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
    }

    #[test]
    fn create_clone_builds_an_empty_shape_and_copies_into_it() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(TRI_SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.set_u32(block + 0xc4, 0);
            block.into_ret()
        });
        let this = shape(&mut e, 1);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a8_7340, &args![this, 0x6000u32]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0xd0]]);
        assert_eq!(calls_to(&e, TRI_SHAPE_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(e.mem.u32(clone), BS_SEGMENTED_TRI_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(clone + 0xc8), 1);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_COPY_MEMBERS),
            vec![vec![this, clone, 0x6000]]
        );
        // A failed allocation still copies, into null.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.mem.set_u32(this + 0xc4, 0);
        e.mem.map(0, 0x100);
        assert_eq!(e.call(0x00a8_7340, &args![this, 0x6000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_COPY_MEMBERS)[1],
            vec![this, 0, 0x6000]
        );
    }

    #[test]
    fn save_binary_writes_the_count_and_each_segment() {
        const WRITE: u32 = 0x0380_0000;
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            (value << 8 | a[2]).into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, output);
        let this = shape(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a8_7520, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let segments = e.mem.u32(this + 0xc4);
        let writes: Vec<(u32, u32)> = calls_to(&e, WRITE).iter().map(|c| (c[1], c[2])).collect();
        assert_eq!(writes.len(), 7);
        assert_eq!(writes[0], (this + 0xc8, 4));
        assert_eq!(writes[1].1, 1);
        assert_eq!(writes[2], (segments, 4));
        assert_eq!(writes[3], (segments + 4, 4));
        assert_eq!(writes[5], (segments + 0x14, 4));
        assert_eq!(writes[6], (segments + 0x18, 4));
    }

    #[test]
    fn get_viewer_strings_lists_segments_and_counts_empty_ones() {
        let mut e = Engine::new();
        e.mem.map(BS_SEGMENTED_TRI_SHAPE_RTTI, 4);
        e.mem.set_u32(BS_SEGMENTED_TRI_SHAPE_RTTI, 0x0109_0000);
        e.register(TRI_SHAPE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            for i in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let value = e.mem.u32(old + 4 * i);
                e.mem.set_u32(base + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        let this = shape(&mut e, 3);
        let segments = e.mem.u32(this + 0xc4);
        // Segment 0 enabled, 1 empty, 2 disabled.
        e.mem.set_u8(segments + 8, 1);
        e.mem.set_u32(segments + 0x14 + 4, 0);
        e.mem.set_u8(segments + 0x28 + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_7620, &args![this, strings]);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let base = e.mem.u32(strings + 4);
        let shown: Vec<u32> = (0..u32::from(e.mem.u16(strings + 0xa)))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect();
        assert_eq!(
            shown,
            vec![
                0x0109_0001,
                TEXT_NUM_SEGMENTS + 3,
                TEXT_ENABLED_SEGMENT,
                TEXT_DISABLED_SEGMENT + 2,
                TEXT_EMPTY_SEGMENTS + 1
            ]
        );
        // Without a segment array only the first two strings.
        e.mem.set_u32(this + 0xc4, 0);
        e.call(0x00a8_7620, &args![this, strings]);
        assert_eq!(e.mem.u16(strings + 0xa), 7);
    }
}
