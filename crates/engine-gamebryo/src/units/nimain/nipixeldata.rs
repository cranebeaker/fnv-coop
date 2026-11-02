//! `NiMain/nipixeldata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPixelData::ms_RTTI` (the class's `NiRTTI` record).
const NI_PIXEL_DATA_RTTI: u32 = 0x011f_4a90;

// Translated from 00a7c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_pixel_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PIXEL_DATA_RTTI)
}

// Translated from 00a7c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7c390`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x74 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_pixel_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_c390;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x74u32]);
    }
    this
}

layout! {
    /// `NiPixelData` (Xbox PDB), 0x74 bytes (the sized delete frees 0x74); the PC code uses
    /// the same offsets.
    pub struct NiPixelData: 0x74 {
        /// `m_kPixelFormat` (Xbox PDB): an `NiPixelFormat` of 0x44 bytes.
        0x08 m_kPixelFormat: u32,
        /// `m_spPalette` (Xbox PDB).
        0x4c m_spPalette: u32,
        /// `m_pucPixels` (Xbox PDB).
        0x50 m_pucPixels: u32,
        /// `m_puiWidth` (Xbox PDB).
        0x54 m_puiWidth: u32,
        /// `m_puiHeight` (Xbox PDB).
        0x58 m_puiHeight: u32,
        /// `m_puiOffsetInBytes` (Xbox PDB).
        0x5c m_puiOffsetInBytes: u32,
        /// `m_uiMipmapLevels` (Xbox PDB).
        0x60 m_uiMipmapLevels: u32,
        /// `m_uiPixelStride` (Xbox PDB).
        0x64 m_uiPixelStride: u32,
        /// `m_uiRevID` (Xbox PDB).
        0x68 m_uiRevID: u32,
        /// `m_uiFaces` (Xbox PDB).
        0x6c m_uiFaces: u32,
        /// `bNoConvert` (Xbox PDB).
        0x70 bNoConvert: u8,
    }
}

/// `NiPixelData`'s virtual table.
const NI_PIXEL_DATA_VTABLE: u32 = 0x0109_dbb4;
/// `NiPixelFormat` (Xbox PDB): its 0x11 words, `m_ucBitsPerPixel` (+0x01) and `m_eFormat`
/// (+0x04; `FORMAT_DXT1` .. `FORMAT_DXT5` are 4..6).
const PIXEL_FORMAT_WORDS: u32 = 0x11;
const FORMAT_BITS_PER_PIXEL: u32 = 0x01;
const FORMAT_KIND: u32 = 0x04;
const FORMAT_DXT1: i32 = 4;
const FORMAT_DXT5: i32 = 6;
/// The constructor's stack frame (0xc4 bytes): the bytes per pixel (+0x00), the offsets of the
/// levels (+0x04, one more than the levels), their widths (+0x44) and heights (+0x84), room
/// for 16 levels each.
const FRAME_SIZE: u32 = 0xc4;
const FRAME_OFFSETS: u32 = 0x04;
const FRAME_WIDTHS: u32 = 0x44;
const FRAME_HEIGHTS: u32 = 0x84;

// Callees outside this unit.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `NiPixelFormat::NiPixelFormat` (`00a7cca0`, `__thiscall`).
const PIXEL_FORMAT_CONSTRUCTOR: u32 = 0x00a7_cca0;
/// `NiPixelData::AllocateData` (`00a8f010`, `__thiscall`: levels, faces, bytes per face).
const ALLOCATE_DATA: u32 = 0x00a8_f010;
/// `00a8f060` (`__thiscall`, `nipixeldata_xenon.cpp`): frees the pixel buffers.
const FREE_DATA: u32 = 0x00a8_f060;
/// `_memcpy` (`00ec44d0`, cdecl, three words).
const MEMCPY: u32 = 0x00ec_44d0;
/// `InterlockedDecrement` import slot; `DeleteThis` (virtual +0x4).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const DELETE_THIS_SLOT: u32 = 0x4;

// Translated from 00a7be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two words: width, height; the shape of
/// `NiPixelData::ComputeMipmapLevels`): 0 when either side is 0, else the larger of the two
/// sides' bit lengths (the number of halvings to 1, plus one).
pub fn fn_00a7be30(_e: &mut Engine, width: u32, height: u32) -> u32 {
    if width == 0 || height == 0 {
        return 0;
    }
    let width_levels = 32 - width.leading_zeros();
    let height_levels = 32 - height.leading_zeros();
    if height_levels < width_levels {
        width_levels
    } else {
        height_levels
    }
}

// Translated from 00a7c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::NiPixelData` (Xbox PDB `NiPixelData_ov2`; `__thiscall`, five words: width,
/// height, format, mipmap levels, faces). The `NiObject` constructor, the class's virtual
/// table, the pixel format reset (`NiPixelFormat::NiPixelFormat`) and then copied from
/// `format`, no palette, `bNoConvert` cleared. Zero levels become the full chain
/// ([`fn_00a7be30`]). Each level's side is the previous one halved (0 read as 1); its size is
/// `stride * width * height` for an uncompressed format (`stride` = bits per pixel / 8),
/// `width * height / 2` when that stride is 0, and for a DXT format (stride 0) the 4x4 blocks
/// times 8 (DXT1) or 16 bytes. The offsets accumulate on the stack with the sides;
/// `AllocateData(levels, faces, total)` makes the buffers and the sides and offsets are copied
/// in (`_memcpy`). The revision becomes 1. Answers `this`.
pub fn ni_pixel_data_ni_pixel_data(
    e: &mut Engine,
    this: Ptr<NiPixelData>,
    width: u32,
    height: u32,
    format: Ptr,
    levels: u32,
    faces: u32,
) -> Ptr<NiPixelData> {
    let base = this.addr();
    let format = format.addr();
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_PIXEL_DATA_VTABLE);
    e.call(PIXEL_FORMAT_CONSTRUCTOR, &args![base + 0x8]);
    e.set(this, NiPixelData::m_spPalette, 0);
    e.set(this, NiPixelData::bNoConvert, 0);
    for i in 0..PIXEL_FORMAT_WORDS {
        let word = e.mem.u32(format + 4 * i);
        e.mem.set_u32(base + 0x8 + 4 * i, word);
    }
    let levels = if levels == 0 {
        fn_00a7be30(e, width, height)
    } else {
        levels
    };
    e.set(this, NiPixelData::m_uiMipmapLevels, levels);
    e.set(this, NiPixelData::m_uiFaces, faces);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let offsets = frame + FRAME_OFFSETS;
        let widths = frame + FRAME_WIDTHS;
        let heights = frame + FRAME_HEIGHTS;
        let slot = |table: u32, i: u32| table.wrapping_add(i.wrapping_mul(4));
        let kind = e.mem.i32(format + FORMAT_KIND);
        e.mem.set_u32(offsets, 0);
        let (mut w, mut h) = (width, height);
        if (FORMAT_DXT1..=FORMAT_DXT5).contains(&kind) {
            e.set(this, NiPixelData::m_uiPixelStride, 0);
            let block = if e.mem.i32(format + FORMAT_KIND) != FORMAT_DXT1 {
                16
            } else {
                8
            };
            let mut i = 0;
            while i < e.get(this, NiPixelData::m_uiMipmapLevels) {
                let side_w = if w != 0 { w } else { 1 };
                e.mem.set_u32(slot(widths, i), side_w);
                let side_h = if h != 0 { h } else { 1 };
                e.mem.set_u32(slot(heights, i), side_h);
                let blocks =
                    ((side_h.wrapping_add(3) & !3).wrapping_mul(side_w.wrapping_add(3) & !3)) >> 4;
                let next = blocks
                    .wrapping_mul(block)
                    .wrapping_add(e.mem.u32(slot(offsets, i)));
                e.mem.set_u32(slot(offsets, i + 1), next);
                i += 1;
                w >>= 1;
                h >>= 1;
            }
        } else {
            let stride = u32::from(e.mem.u8(format + FORMAT_BITS_PER_PIXEL)) >> 3;
            e.mem.set_u32(frame, stride);
            e.set(this, NiPixelData::m_uiPixelStride, stride);
            let mut i = 0;
            while i < e.get(this, NiPixelData::m_uiMipmapLevels) {
                let side_w = if w != 0 { w } else { 1 };
                e.mem.set_u32(slot(widths, i), side_w);
                let side_h = if h != 0 { h } else { 1 };
                e.mem.set_u32(slot(heights, i), side_h);
                let size = if stride != 0 {
                    e.mem.u32(frame).wrapping_mul(side_h).wrapping_mul(side_w)
                } else {
                    side_h.wrapping_mul(side_w) >> 1
                };
                let next = size.wrapping_add(e.mem.u32(slot(offsets, i)));
                e.mem.set_u32(slot(offsets, i + 1), next);
                i += 1;
                w >>= 1;
                h >>= 1;
            }
        }
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let total = e.mem.u32(slot(offsets, levels));
        e.call(ALLOCATE_DATA, &args![this, levels, faces, total]);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let bytes = levels.wrapping_mul(4);
        let to = e.get(this, NiPixelData::m_puiWidth);
        e.call(MEMCPY, &args![to, widths, bytes]);
        let to = e.get(this, NiPixelData::m_puiHeight);
        e.call(MEMCPY, &args![to, heights, bytes]);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let to = e.get(this, NiPixelData::m_puiOffsetInBytes);
        e.call(
            MEMCPY,
            &args![to, offsets, levels.wrapping_mul(4).wrapping_add(4)],
        );
    });
    e.set(this, NiPixelData::m_uiRevID, 1);
    this
}

/// The palette's reference given up (deleted at the last one).
fn release_palette(e: &mut Engine, palette: u32) {
    let count = e.call(INTERLOCKED_DECREMENT, &args![palette + 4]).u32();
    if count == 0 {
        e.vcall(palette, DELETE_THIS_SLOT, &args![]);
    }
}

// Translated from 00a7c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPixelData` destructor body (unnamed in the map): the class's virtual table, the
/// palette released and cleared, the buffers freed (`00a8f060`), then the palette pointer
/// released again if anything put one back (without clearing it, as the inlined `NiPointer`
/// destructor does), and the `NiObject` destructor (a tail jump).
pub fn fn_00a7c390(e: &mut Engine, this: Ptr<NiPixelData>) {
    e.mem.set_u32(this.addr(), NI_PIXEL_DATA_VTABLE);
    let palette = e.get(this, NiPixelData::m_spPalette);
    if palette != 0 {
        release_palette(e, palette);
        e.set(this, NiPixelData::m_spPalette, 0);
    }
    e.call(FREE_DATA, &args![this]);
    let palette = e.get(this, NiPixelData::m_spPalette);
    if palette != 0 {
        release_palette(e, palette);
    }
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

/// `00a89af0` (`thiscall`, the stream; unnamed in the engine map, a 3-byte body that both
/// `LinkObject` and `SaveBinary` call first: the `NiObject` base's).
const OBJECT_STREAM_BASE: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (Xbox PDB, `thiscall`, no stack word).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiObject::RegisterStreamables` (Xbox PDB, `thiscall`, the stream).
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// `NiObject::IsEqual` (Xbox PDB, `thiscall`, the other object).
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// `00a765c0` (`thiscall`, the other pixel format; the engine map names it
/// `NiPixelFormat::operator==` in `nixenonrenderer.cpp`): the exe treats a nonzero answer
/// as the formats differing.
const PIXEL_FORMAT_COMPARE: u32 = 0x00a7_65c0;
/// `NiPalette::NiPalette` (Xbox PDB, `thiscall`, the palette to copy).
const PALETTE_COPY_CONSTRUCTOR: u32 = 0x00a8_b080;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject::LoadBinary` (Xbox PDB, `thiscall`, the stream).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a7ced0` and `00a7d1a0` (`thiscall` on the pixel format, the stream; unnamed in the
/// engine map): the pixel format's load and save.
const PIXEL_FORMAT_LOAD: u32 = 0x00a7_ced0;
const PIXEL_FORMAT_SAVE: u32 = 0x00a7_d1a0;
/// `00a645b0` (`thiscall` on the stream, no stack word; unnamed in the engine map): reads
/// the next link id (the palette's).
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream`'s file version (+0xd8), its input (+0x24c, read function at +8) and output
/// (+0x250, write function at +0xc); a file's position is at +4.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// The first file version that stores the face count (10.3.0.6).
const VERSION_WITH_FACES: u32 = 0x0a03_0006;
/// `NiObject::GetViewerStrings` (Xbox PDB, `thiscall`, the string array).
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word) and its `unsigned int` overload
/// `NiGetViewerString_ov8` (`cdecl`, name and value).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_UINT: u32 = 0x00a7_d510;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"Width"`, `"Height"`, `"m_uiMipmapLevels"`, `"m_uiFaces"`, `"TotalSizeInBytes"`,
/// `"FaceSizeInBytes"`.
const NAME_WIDTH: u32 = 0x0109_dc8c;
const NAME_HEIGHT: u32 = 0x0109_dc84;
const NAME_MIPMAP_LEVELS: u32 = 0x0109_dc70;
const NAME_FACES: u32 = 0x0109_dc64;
const NAME_TOTAL_SIZE: u32 = 0x0109_dc50;
const NAME_FACE_SIZE: u32 = 0x0109_dc40;

// Translated from 00a7be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::LinkObject` (Xbox PDB): the base's link step (`00a89af0`), then the palette
/// becomes the stream's object for the next link id (`GetObjectFromLinkID`), assigned as a
/// counted pointer (the old one released, the new one counted).
pub fn ni_pixel_data_link_object(e: &mut Engine, this: Ptr<NiPixelData>, stream: Ptr) {
    e.call(OBJECT_STREAM_BASE, &args![this, stream]);
    let palette = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    crate::units::nimain::nimainsdm::assign_ref_counted(e, this.addr() + 0x4c, palette);
}

// Translated from 00a7bee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::RegisterStreamables` (Xbox PDB): false when `NiObject`'s refuses;
/// otherwise the palette (when set) registers itself (virtual +0x54, the stream) and the
/// answer is true.
pub fn ni_pixel_data_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPixelData>,
    stream: Ptr,
) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let palette = e.get(this, NiPixelData::m_spPalette);
    if palette != 0 {
        e.vcall(palette, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a7bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::IsEqual` (Xbox PDB): `NiObject::IsEqual`, the pixel formats (`00a765c0`
/// answering zero), the palettes (this one's `IsEqual`, virtual +0x5c, when it has one),
/// the level count and stride, each level's width, height and offset, the last offset, the
/// face count, then the pixel bytes of the first `offset[levels]` bytes (words while four
/// or more remain, then up to three bytes) must all match.
pub fn ni_pixel_data_is_equal(e: &mut Engine, this: Ptr<NiPixelData>, other: Ptr) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = this.addr();
    let theirs = other.addr();
    if e.call(PIXEL_FORMAT_COMPARE, &args![mine + 8, theirs + 8])
        .bool()
    {
        return false;
    }
    let palette = e.get(this, NiPixelData::m_spPalette);
    if palette != 0 {
        let other_palette = e.mem.u32(theirs + 0x4c);
        if !e.vcall(palette, 0x5c, &args![other_palette]).bool() {
            return false;
        }
    }
    let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
    if levels != e.mem.u32(theirs + 0x60) {
        return false;
    }
    if e.get(this, NiPixelData::m_uiPixelStride) != e.mem.u32(theirs + 0x64) {
        return false;
    }
    let width_gap = e
        .mem
        .u32(theirs + 0x54)
        .wrapping_sub(e.get(this, NiPixelData::m_puiWidth));
    let mut i = 0u32;
    while i < levels {
        let widths = e.get(this, NiPixelData::m_puiWidth).wrapping_add(4 * i);
        if e.mem.u32(widths) != e.mem.u32(width_gap.wrapping_add(widths)) {
            return false;
        }
        let height = e.get(this, NiPixelData::m_puiHeight).wrapping_add(4 * i);
        if e.mem.u32(height) != e.mem.u32(e.mem.u32(theirs + 0x58).wrapping_add(4 * i)) {
            return false;
        }
        let offset = e
            .get(this, NiPixelData::m_puiOffsetInBytes)
            .wrapping_add(4 * i);
        if e.mem.u32(offset) != e.mem.u32(e.mem.u32(theirs + 0x5c).wrapping_add(4 * i)) {
            return false;
        }
        i += 1;
    }
    let offsets = e.get(this, NiPixelData::m_puiOffsetInBytes);
    let other_offsets = e.mem.u32(theirs + 0x5c);
    let last = levels.wrapping_mul(4);
    if e.mem.u32(last.wrapping_add(offsets)) != e.mem.u32(last.wrapping_add(other_offsets)) {
        return false;
    }
    if e.get(this, NiPixelData::m_uiFaces) != e.mem.u32(theirs + 0x6c) {
        return false;
    }
    let mut size = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
    let mut theirs = e.mem.u32(theirs + 0x50);
    let mut mine = e.get(this, NiPixelData::m_pucPixels);
    while size >= 4 {
        if e.mem.u32(mine) != e.mem.u32(theirs) {
            return false;
        }
        size -= 4;
        theirs = theirs.wrapping_add(4);
        mine = mine.wrapping_add(4);
    }
    if size == 0 {
        return true;
    }
    if e.mem.u8(theirs) != e.mem.u8(mine) {
        return false;
    }
    if size <= 1 {
        return true;
    }
    if e.mem.u8(theirs + 1) != e.mem.u8(mine + 1) {
        return false;
    }
    if size <= 2 {
        return true;
    }
    e.mem.u8(theirs + 2) == e.mem.u8(mine + 2)
}

/// The copy constructor's stack frame (0xc0 bytes): the offsets of the levels (+0x00, one
/// more than the levels), their widths (+0x40) and heights (+0x80), room for 16 levels each.
const COPY_FRAME_SIZE: u32 = 0xc0;
const COPY_FRAME_WIDTHS: u32 = 0x40;
const COPY_FRAME_HEIGHTS: u32 = 0x80;

// Translated from 00a7c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::NiPixelData` (Xbox PDB; the copy constructor, named `_copy` here beside the
/// sized constructor above): the `NiObject`
/// constructor, the class's vtable, the pixel format reset (`NiPixelFormat::NiPixelFormat`)
/// and then copied (0x11 words) from `other`, no palette, `bNoConvert` cleared. When
/// `other` has a palette, a copy of it (0x24 bytes from `00aa13e0`, `NiPalette`'s copy
/// constructor; null when the allocation fails) is assigned as a counted pointer. The level
/// count, faces and stride are copied; the levels' offsets, heights and widths go through
/// the stack frame (`levels & 0x3fffffff` words each, as `REP MOVSD` copies them), the
/// total (`other`'s `offset[levels]`) is stored after the offsets, `AllocateData(levels,
/// faces, total)` makes the buffers, the tables are copied in (`_memcpy`) and so are the
/// `total` pixel bytes. The revision becomes 1. Answers `this`.
pub fn ni_pixel_data_ni_pixel_data_copy(
    e: &mut Engine,
    this: Ptr<NiPixelData>,
    other: Ptr,
) -> Ptr<NiPixelData> {
    let base = this.addr();
    let other = other.addr();
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_PIXEL_DATA_VTABLE);
    e.call(PIXEL_FORMAT_CONSTRUCTOR, &args![base + 0x8]);
    e.set(this, NiPixelData::m_spPalette, 0);
    e.set(this, NiPixelData::bNoConvert, 0);
    for i in 0..PIXEL_FORMAT_WORDS {
        let word = e.mem.u32(other + 0x8 + 4 * i);
        e.mem.set_u32(base + 0x8 + 4 * i, word);
    }
    let other_palette = e.mem.u32(other + 0x4c);
    if other_palette != 0 {
        let block = e.call(ALLOCATE, &args![0x24u32]).u32();
        let palette = if block == 0 {
            0
        } else {
            let palette = e.mem.u32(other + 0x4c);
            e.call(PALETTE_COPY_CONSTRUCTOR, &args![block, palette])
                .u32()
        };
        crate::units::nimain::nimainsdm::assign_ref_counted(e, base + 0x4c, palette);
    }
    let levels = e.mem.u32(other + 0x60);
    e.set(this, NiPixelData::m_uiMipmapLevels, levels);
    let faces = e.mem.u32(other + 0x6c);
    e.set(this, NiPixelData::m_uiFaces, faces);
    let stride = e.mem.u32(other + 0x64);
    e.set(this, NiPixelData::m_uiPixelStride, stride);
    e.with_stack(COPY_FRAME_SIZE, |e, frame| {
        let offsets = frame.addr();
        let widths = offsets + COPY_FRAME_WIDTHS;
        let heights = offsets + COPY_FRAME_HEIGHTS;
        if levels != 0 {
            let words = levels.wrapping_mul(4) >> 2;
            for (table, to) in [(0x5cu32, offsets), (0x58, heights), (0x54, widths)] {
                let from = e.mem.u32(other + table);
                for i in 0..words {
                    let word = e.mem.u32(from.wrapping_add(4 * i));
                    e.mem.set_u32(to.wrapping_add(4 * i), word);
                }
            }
        }
        let other_offsets = e.mem.u32(other + 0x5c);
        let total = e
            .mem
            .u32(other_offsets.wrapping_add(levels.wrapping_mul(4)));
        let faces = e.get(this, NiPixelData::m_uiFaces);
        e.mem
            .set_u32(offsets.wrapping_add(levels.wrapping_mul(4)), total);
        e.call(ALLOCATE_DATA, &args![this, levels, faces, total]);
        let bytes = e.get(this, NiPixelData::m_uiMipmapLevels).wrapping_mul(4);
        let to = e.get(this, NiPixelData::m_puiWidth);
        e.call(MEMCPY, &args![to, widths, bytes]);
        let to = e.get(this, NiPixelData::m_puiHeight);
        e.call(MEMCPY, &args![to, heights, bytes]);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let to = e.get(this, NiPixelData::m_puiOffsetInBytes);
        e.call(
            MEMCPY,
            &args![to, offsets, levels.wrapping_mul(4).wrapping_add(4)],
        );
        let from = e.mem.u32(other + 0x50);
        let to = e.get(this, NiPixelData::m_pucPixels);
        e.call(MEMCPY, &args![to, from, total]);
    });
    e.set(this, NiPixelData::m_uiRevID, 1);
    this
}

// Translated from 00a7c3f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiPixelData`, `0109dc94`): a new pixel data (0x74 bytes from
/// `00aa13e0`; the `NiObject` constructor, the vtable, the pixel format reset, the palette,
/// pixels, tables, level count, faces and stride cleared, the revision 1; `bNoConvert` is
/// left as allocated), or null when the allocation fails.
pub fn fn_00a7c3f0(e: &mut Engine) -> Ptr<NiPixelData> {
    let block = e.call(ALLOCATE, &args![0x74u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let this: Ptr<NiPixelData> = Ptr::new(block);
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(block, NI_PIXEL_DATA_VTABLE);
    e.call(PIXEL_FORMAT_CONSTRUCTOR, &args![block + 0x8]);
    e.set(this, NiPixelData::m_spPalette, 0);
    e.set(this, NiPixelData::m_pucPixels, 0);
    e.set(this, NiPixelData::m_puiWidth, 0);
    e.set(this, NiPixelData::m_puiHeight, 0);
    e.set(this, NiPixelData::m_puiOffsetInBytes, 0);
    e.set(this, NiPixelData::m_uiMipmapLevels, 0);
    e.set(this, NiPixelData::m_uiFaces, 0);
    e.set(this, NiPixelData::m_uiPixelStride, 0);
    e.set(this, NiPixelData::m_uiRevID, 1);
    this
}

/// One read or write through the stream's input (read function at +8) or output (write
/// function at +0xc): `function(file, at, size, count, 1)` with the word at `count` set to
/// `count_value` first, the answer added to the file's position.
fn stream_transfer(
    e: &mut Engine,
    stream: Ptr,
    write: bool,
    at: u32,
    size: u32,
    count: u32,
    count_value: u32,
) {
    let (file_slot, function_slot) = if write {
        (STREAM_OUTPUT, 0xc)
    } else {
        (STREAM_INPUT, 8)
    };
    let file = e.mem.u32(stream.addr().wrapping_add(file_slot));
    let function = e.mem.u32(file.wrapping_add(function_slot));
    e.mem.set_u32(count, count_value);
    let done = e.call(function, &args![file, at, size, count, 1u32]).u32();
    let position = e.mem.u32(file.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(file.wrapping_add(4), position);
}

// Translated from 00a7c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the pixel format's load
/// (`00a7ced0`), the palette's link id (`00a645b0`), then from the stream's input: the
/// level count (+0x60) and stride (+0x64), each level's width, height and offset into the
/// stack frame (laid out as the constructor's: the byte-count word at +0, offsets at +4,
/// widths at +0x44, heights at +0x84), the offset after the last level, and the face count
/// (+0x6c; 1 for files older than 10.3.0.6). `AllocateData(levels, faces, offset[levels])`
/// makes the buffers, the tables are copied in (`_memcpy`) and the pixels are read as
/// `offset[levels] * faces` bytes with a byte count of 1.
pub fn ni_pixel_data_load_binary(e: &mut Engine, this: Ptr<NiPixelData>, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(PIXEL_FORMAT_LOAD, &args![this.addr() + 8, stream]);
    e.call(READ_LINK_ID, &args![stream]);
    let base = this.addr();
    e.with_stack(FRAME_SIZE, |e, frame| {
        let count = frame.addr();
        let offsets = count + FRAME_OFFSETS;
        let widths = count + FRAME_WIDTHS;
        let heights = count + FRAME_HEIGHTS;
        stream_transfer(e, stream, false, base + 0x60, 4, count, 4);
        stream_transfer(e, stream, false, base + 0x64, 4, count, 4);
        let mut i = 0u32;
        while i < e.get(this, NiPixelData::m_uiMipmapLevels) {
            let at = |table: u32| table.wrapping_add(i.wrapping_mul(4));
            stream_transfer(e, stream, false, at(widths), 4, count, 4);
            stream_transfer(e, stream, false, at(heights), 4, count, 4);
            stream_transfer(e, stream, false, at(offsets), 4, count, 4);
            i += 1;
        }
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let last = offsets.wrapping_add(levels.wrapping_mul(4));
        stream_transfer(e, stream, false, last, 4, count, 4);
        if e.mem.u32(stream.addr() + STREAM_VERSION) < VERSION_WITH_FACES {
            e.set(this, NiPixelData::m_uiFaces, 1);
        } else {
            stream_transfer(e, stream, false, base + 0x6c, 4, count, 4);
        }
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let total = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
        let faces = e.get(this, NiPixelData::m_uiFaces);
        e.call(ALLOCATE_DATA, &args![this, levels, faces, total]);
        let bytes = e.get(this, NiPixelData::m_uiMipmapLevels).wrapping_mul(4);
        let to = e.get(this, NiPixelData::m_puiWidth);
        e.call(MEMCPY, &args![to, widths, bytes]);
        let to = e.get(this, NiPixelData::m_puiHeight);
        e.call(MEMCPY, &args![to, heights, bytes]);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let to = e.get(this, NiPixelData::m_puiOffsetInBytes);
        e.call(
            MEMCPY,
            &args![to, offsets, levels.wrapping_mul(4).wrapping_add(4)],
        );
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let table = e.get(this, NiPixelData::m_puiOffsetInBytes);
        let size = e
            .mem
            .u32(table.wrapping_add(levels.wrapping_mul(4)))
            .wrapping_mul(e.get(this, NiPixelData::m_uiFaces));
        let pixels = e.get(this, NiPixelData::m_pucPixels);
        stream_transfer(e, stream, false, pixels, size, count, 1);
    });
}

// Translated from 00a7c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::SaveBinary` (Xbox PDB): the base's step (`00a89af0`), the pixel format's
/// save (`00a7d1a0`), the palette's link id (the stream's virtual +0x30), then to the
/// stream's output: the level count and stride, each level's width, height and offset, the
/// offset after the last level, the face count, and the `offset[levels] * faces` pixel
/// bytes (byte count 1). The byte-count word is the stream argument's stack word, except
/// inside the level loop (which keeps its index there), where it is the function's own
/// local.
pub fn ni_pixel_data_save_binary(e: &mut Engine, this: Ptr<NiPixelData>, stream: Ptr) {
    e.call(OBJECT_STREAM_BASE, &args![this, stream]);
    e.call(PIXEL_FORMAT_SAVE, &args![this.addr() + 8, stream]);
    let palette = e.get(this, NiPixelData::m_spPalette);
    e.vcall(stream.addr(), 0x30, &args![palette]);
    let base = this.addr();
    e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let local = argument + 4;
        stream_transfer(e, stream, true, base + 0x60, 4, argument, 4);
        stream_transfer(e, stream, true, base + 0x64, 4, argument, 4);
        let mut i = 0u32;
        while i < e.get(this, NiPixelData::m_uiMipmapLevels) {
            for table in [
                NiPixelData::m_puiWidth,
                NiPixelData::m_puiHeight,
                NiPixelData::m_puiOffsetInBytes,
            ] {
                let at = e.get(this, table).wrapping_add(i.wrapping_mul(4));
                stream_transfer(e, stream, true, at, 4, local, 4);
            }
            i += 1;
        }
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let last = e
            .get(this, NiPixelData::m_puiOffsetInBytes)
            .wrapping_add(levels.wrapping_mul(4));
        stream_transfer(e, stream, true, last, 4, argument, 4);
        stream_transfer(e, stream, true, base + 0x6c, 4, argument, 4);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let table = e.get(this, NiPixelData::m_puiOffsetInBytes);
        let size = e
            .mem
            .u32(table.wrapping_add(levels.wrapping_mul(4)))
            .wrapping_mul(e.get(this, NiPixelData::m_uiFaces));
        let pixels = e.get(this, NiPixelData::m_pucPixels);
        stream_transfer(e, stream, true, pixels, size, argument, 1);
    });
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array: when
/// the count (`m_usSize`, +0xa) is not below the capacity (`m_usMaxSize`, +8) the array first
/// grows to count + `m_usGrowBy` (+0xe, `0096ad30`); then `0096ae90` stores it at the old
/// count.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a7c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`, then seven
/// strings appended to `strings`: the class name (the first word of `NiPixelData::ms_RTTI`,
/// `011f4a90`), and through `NiGetViewerString_ov8` the first level's width and height, the
/// level count, the face count, `offset[levels] * faces` and `offset[levels]`. Each string
/// pointer is kept in the `strings` argument's stack word, whose address the array's set-at
/// receives.
pub fn ni_pixel_data_get_viewer_strings(e: &mut Engine, this: Ptr<NiPixelData>, strings: Ptr) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_PIXEL_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        let face_size = |e: &Engine| {
            let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
            let table = e.get(this, NiPixelData::m_puiOffsetInBytes);
            e.mem.u32(table.wrapping_add(levels.wrapping_mul(4)))
        };
        for item in 0..6 {
            // Each value is read right before its call, as the exe reads it.
            let (name, value) = match item {
                0 => (NAME_WIDTH, e.mem.u32(e.get(this, NiPixelData::m_puiWidth))),
                1 => (
                    NAME_HEIGHT,
                    e.mem.u32(e.get(this, NiPixelData::m_puiHeight)),
                ),
                2 => (
                    NAME_MIPMAP_LEVELS,
                    e.get(this, NiPixelData::m_uiMipmapLevels),
                ),
                3 => (NAME_FACES, e.get(this, NiPixelData::m_uiFaces)),
                4 => (
                    NAME_TOTAL_SIZE,
                    face_size(e).wrapping_mul(e.get(this, NiPixelData::m_uiFaces)),
                ),
                _ => (NAME_FACE_SIZE, face_size(e)),
            };
            let text = e.call(GET_VIEWER_STRING_UINT, &args![name, value]).u32();
            e.mem.set_u32(slot, text);
            append_viewer_string(e, strings, slot);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a7_be80,
            ni_pixel_data_link_object(Ptr<NiPixelData>, Ptr)
        ),
        entry!(
            0x00a7_bee0,
            ni_pixel_data_register_streamables(Ptr<NiPixelData>, Ptr) -> bool
        ),
        entry!(
            0x00a7_bf10,
            ni_pixel_data_is_equal(Ptr<NiPixelData>, Ptr) -> bool
        ),
        entry!(
            0x00a7_c030,
            ni_pixel_data_ni_pixel_data_copy(Ptr<NiPixelData>, Ptr) -> Ptr<NiPixelData>
        ),
        entry!(0x00a7_c3f0, fn_00a7c3f0() -> Ptr<NiPixelData>),
        entry!(
            0x00a7_c470,
            ni_pixel_data_load_binary(Ptr<NiPixelData>, Ptr)
        ),
        entry!(
            0x00a7_c670,
            ni_pixel_data_save_binary(Ptr<NiPixelData>, Ptr)
        ),
        entry!(
            0x00a7_c820,
            ni_pixel_data_get_viewer_strings(Ptr<NiPixelData>, Ptr)
        ),
        entry!(0x00a7be30, fn_00a7be30(u32, u32) -> u32),
        entry!(
            0x00a7c190,
            ni_pixel_data_ni_pixel_data(
                Ptr<NiPixelData>,
                u32,
                u32,
                Ptr,
                u32,
                u32,
            ) -> Ptr<NiPixelData>
        ),
        entry!(0x00a7c390, fn_00a7c390(Ptr<NiPixelData>)),
        entry!(0x00a7c020, ni_pixel_data_get_rtti() -> Ptr),
        entry!(0x00a7c450, ni_pixel_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_pixel_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7c020, &args![]).u32(), NI_PIXEL_DATA_RTTI);
    }

    #[test]
    fn ni_pixel_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_c390, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a7c450, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7c450, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x74]]);
        assert_eq!(calls_to(&e, 0x00a7_c390), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00a7be30_counts_levels() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7be30, &args![0u32, 8u32]).u32(), 0);
        assert_eq!(e.call(0x00a7be30, &args![8u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00a7be30, &args![1u32, 1u32]).u32(), 1);
        assert_eq!(e.call(0x00a7be30, &args![256u32, 16u32]).u32(), 9);
        assert_eq!(e.call(0x00a7be30, &args![3u32, 1000u32]).u32(), 10);
    }

    /// Doubles for the constructor's callees: `AllocateData` makes three tables of 16 words
    /// and the copies are real.
    fn constructor_doubles(e: &mut Engine) {
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PIXEL_FORMAT_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(ALLOCATE_DATA, |e, a| {
            let tables = e.mem.alloc(0xc0);
            e.mem.set_u32(a[0] + 0x54, tables);
            e.mem.set_u32(a[0] + 0x58, tables + 0x40);
            e.mem.set_u32(a[0] + 0x5c, tables + 0x80);
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
    }

    fn table(e: &Engine, data: u32, at: u32, count: u32) -> Vec<u32> {
        let table = e.mem.u32(data + at);
        (0..count).map(|i| e.mem.u32(table + 4 * i)).collect()
    }

    fn format(e: &mut Engine, bits: u8, kind: i32) -> u32 {
        let f = e.mem.alloc(0x44);
        e.mem.set_u8(f + 1, bits);
        e.mem.set_i32(f + 4, kind);
        e.mem.set_u32(f + 0x40, 0xabc);
        f
    }

    #[test]
    fn constructor_lays_out_uncompressed_levels() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        let data = e.mem.alloc(0x74);
        let f = format(&mut e, 32, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a7c190, &args![data, 8u32, 2u32, f, 0u32, 1u32])
                .u32(),
            data
        );
        assert_eq!(e.mem.u32(data), NI_PIXEL_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x48), 0xabc);
        assert_eq!(e.mem.u32(data + 0x60), 4);
        assert_eq!(e.mem.u32(data + 0x64), 4);
        assert_eq!(e.mem.u32(data + 0x68), 1);
        assert_eq!(e.mem.u32(data + 0x6c), 1);
        assert_eq!(table(&e, data, 0x54, 4), vec![8, 4, 2, 1]);
        assert_eq!(table(&e, data, 0x58, 4), vec![2, 1, 1, 1]);
        assert_eq!(table(&e, data, 0x5c, 5), vec![0, 64, 80, 88, 92]);
        assert_eq!(calls_to(&e, ALLOCATE_DATA), vec![vec![data, 4, 1, 92]]);
        // Four bits per pixel (stride 0): half a byte each.
        let f = format(&mut e, 4, 2);
        e.call(0x00a7c190, &args![data, 4u32, 4u32, f, 2u32, 6u32]);
        assert_eq!(e.mem.u32(data + 0x64), 0);
        assert_eq!(table(&e, data, 0x5c, 3), vec![0, 8, 10]);
        assert_eq!(e.mem.u32(data + 0x6c), 6);
    }

    #[test]
    fn constructor_lays_out_compressed_levels() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        let data = e.mem.alloc(0x74);
        let dxt1 = format(&mut e, 4, FORMAT_DXT1);
        e.call(0x00a7c190, &args![data, 8u32, 8u32, dxt1, 0u32, 1u32]);
        assert_eq!(e.mem.u32(data + 0x64), 0);
        // 2x2 blocks, then 1 block, 1 block, 1 block: 8 bytes each.
        assert_eq!(table(&e, data, 0x5c, 5), vec![0, 32, 40, 48, 56]);
        let dxt5 = format(&mut e, 8, FORMAT_DXT5);
        e.call(0x00a7c190, &args![data, 8u32, 4u32, dxt5, 2u32, 1u32]);
        assert_eq!(table(&e, data, 0x5c, 3), vec![0, 32, 48]);
        assert_eq!(table(&e, data, 0x58, 2), vec![4, 2]);
    }

    const DELETE_DOUBLE: u32 = 0x0070_6000;

    #[test]
    fn destructor_releases_the_palette_and_frees_the_buffers() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE_DATA, |_, _| Ret::default());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(0x74);
        let palette = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(palette, vtable);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(palette + 4, 1);
        e.mem.set_u32(data + 0x4c, palette);
        e.call_log = Some(vec![]);
        e.call(0x00a7c390, &args![data]);
        assert_eq!(e.mem.u32(data), NI_PIXEL_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x4c), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![palette]]);
        assert_eq!(calls_to(&e, FREE_DATA), vec![vec![data]]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![data]]);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::engine_with_counting;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A counted object whose vtable slot +4 is a recording double, and the log of it.
    fn counted(e: &mut Engine, count: u32, vtable: u32, delete: u32) -> u32 {
        e.put_vtable(vtable, &[0, delete]);
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn link_object_assigns_the_linked_palette() {
        let mut e = engine_with_counting();
        e.register(OBJECT_STREAM_BASE, |_, _| Ret::default());
        e.register(0x0300_1000, |_, _| Ret::default());
        let old = counted(&mut e, 1, 0x0300_0000, 0x0300_1000);
        let new = counted(&mut e, 0, 0x0300_0000, 0x0300_1000);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| new.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + 0x4c, old);
        e.call(0x00a7_be80, &args![this, 0x4444u32]);
        assert_eq!(calls_to(&e, OBJECT_STREAM_BASE), vec![vec![this, 0x4444]]);
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID), vec![vec![0x4444]]);
        assert_eq!(e.mem.u32(this + 0x4c), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![old]]);
    }

    #[test]
    fn register_streamables_passes_the_stream_to_the_palette() {
        let mut e = Engine::new();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 0x100u32.into_ret());
        e.register(0x0300_1000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x74);
        let palette = e.mem.alloc(8);
        e.put_vtable(0x0300_0000, &[0; 0x16]);
        e.mem.set_u32(0x0300_0054, 0x0300_1000);
        e.mem.set_u32(palette, 0x0300_0000);
        e.mem.set_u32(this + 0x4c, palette);
        assert!(!e.call(0x00a7_bee0, &args![this, 0x4444u32]).bool());
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a7_bee0, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![palette, 0x4444]]);
        e.mem.set_u32(this + 0x4c, 0);
        assert!(e.call(0x00a7_bee0, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, 0x0300_1000).len(), 1);
    }

    /// A pixel data with two levels (widths 4, 2; heights 2, 1; offsets 0, 8, 10) and
    /// `size` pixel bytes `0..size`.
    fn pixels(e: &mut Engine, size: u32) -> u32 {
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + 0x60, 2);
        e.mem.set_u32(this + 0x64, 1);
        e.mem.set_u32(this + 0x6c, 1);
        let tables = e.mem.alloc(0x30);
        for (i, v) in [4u32, 2, 2, 1, 0, 8, size].iter().enumerate() {
            let at = if i < 2 {
                tables + 4 * i as u32
            } else if i < 4 {
                tables + 0x10 + 4 * (i as u32 - 2)
            } else {
                tables + 0x20 + 4 * (i as u32 - 4)
            };
            e.mem.set_u32(at, *v);
        }
        e.mem.set_u32(this + 0x54, tables);
        e.mem.set_u32(this + 0x58, tables + 0x10);
        e.mem.set_u32(this + 0x5c, tables + 0x20);
        let data = e.mem.alloc(size.max(1));
        for i in 0..size {
            e.mem.set_u8(data + i, i as u8);
        }
        e.mem.set_u32(this + 0x50, data);
        this
    }

    fn equal_engine() -> Engine {
        let mut e = Engine::new();
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(PIXEL_FORMAT_COMPARE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn is_equal_compares_the_tables_and_the_pixels() {
        let mut e = equal_engine();
        for size in [8u32, 10, 11] {
            let a = pixels(&mut e, size);
            let b = pixels(&mut e, size);
            assert!(e.call(0x00a7_bf10, &args![a, b]).bool(), "{size}");
            // The last byte differs.
            let data = e.mem.u32(b + 0x50);
            e.mem.set_u8(data + size - 1, 0xff);
            assert!(!e.call(0x00a7_bf10, &args![a, b]).bool(), "{size}");
        }
        assert_eq!(
            calls_to(&e, PIXEL_FORMAT_COMPARE)[0][1] - calls_to(&e, PIXEL_FORMAT_COMPARE)[0][0],
            calls_to(&e, OBJECT_IS_EQUAL)[0][1] - calls_to(&e, OBJECT_IS_EQUAL)[0][0]
        );
    }

    #[test]
    fn is_equal_refuses_any_difference() {
        let mut e = equal_engine();
        // Each field or table entry changed in turn.
        let changes: [(u32, Option<u32>); 9] = [
            (0x60, None),
            (0x64, None),
            (0x6c, None),
            (0x54, Some(4)),
            (0x58, Some(0)),
            (0x5c, Some(4)),
            (0x5c, Some(8)),
            (0x54, Some(0)),
            (0x58, Some(4)),
        ];
        for (field, entry) in changes {
            let a = pixels(&mut e, 10);
            let b = pixels(&mut e, 10);
            match entry {
                None => {
                    let v = e.mem.u32(b + field);
                    e.mem.set_u32(b + field, v + 1);
                }
                Some(at) => {
                    let table = e.mem.u32(b + field);
                    let v = e.mem.u32(table + at);
                    e.mem.set_u32(table + at, v + 1);
                }
            }
            assert!(!e.call(0x00a7_bf10, &args![a, b]).bool(), "{field:x}");
        }
        let a = pixels(&mut e, 10);
        let b = pixels(&mut e, 10);
        e.register(PIXEL_FORMAT_COMPARE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00a7_bf10, &args![a, b]).bool());
        e.register(PIXEL_FORMAT_COMPARE, |_, _| 0u32.into_ret());
        e.register(OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_bf10, &args![a, b]).bool());
    }

    #[test]
    fn is_equal_asks_the_palette() {
        let mut e = equal_engine();
        let a = pixels(&mut e, 8);
        let b = pixels(&mut e, 8);
        let palette = e.mem.alloc(8);
        e.put_vtable(0x0300_0000, &[0; 0x18]);
        e.mem.set_u32(0x0300_005c, 0x0300_1000);
        e.mem.set_u32(palette, 0x0300_0000);
        e.mem.set_u32(a + 0x4c, palette);
        e.mem.set_u32(b + 0x4c, 0x7777);
        e.register(0x0300_1000, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_bf10, &args![a, b]).bool());
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![palette, 0x7777]]);
        e.register(0x0300_1000, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a7_bf10, &args![a, b]).bool());
    }

    fn copy_engine() -> Engine {
        let mut e = engine_with_counting();
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PIXEL_FORMAT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ALLOCATE_DATA, |e, a| {
            let tables = e.mem.alloc(0xc0);
            e.mem.set_u32(a[0] + 0x54, tables);
            e.mem.set_u32(a[0] + 0x58, tables + 0x40);
            e.mem.set_u32(a[0] + 0x5c, tables + 0x80);
            let data = e.mem.alloc(a[3].max(1));
            e.mem.set_u32(a[0] + 0x50, data);
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register(PALETTE_COPY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn table(e: &Engine, data: u32, at: u32, count: u32) -> Vec<u32> {
        let table = e.mem.u32(data + at);
        (0..count).map(|i| e.mem.u32(table + 4 * i)).collect()
    }

    #[test]
    fn copy_constructor_copies_everything_and_the_palette() {
        let mut e = copy_engine();
        let other = pixels(&mut e, 10);
        for i in 0..PIXEL_FORMAT_WORDS {
            e.mem.set_u32(other + 8 + 4 * i, 0x100 + i);
        }
        let palette = e.mem.alloc(0x24);
        e.mem.set_u32(other + 0x4c, palette);
        let this = e.mem.alloc(0x74);
        e.mem.set_u8(this + 0x70, 1);
        let back = e.call(0x00a7_c030, &args![this, other]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), NI_PIXEL_DATA_VTABLE);
        assert_eq!(e.mem.u8(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 8 + 0x40), 0x110);
        let copy = calls_to(&e, PALETTE_COPY_CONSTRUCTOR);
        assert_eq!(copy[0][1], palette);
        assert_eq!(e.mem.u32(this + 0x4c), copy[0][0]);
        assert_eq!(e.mem.u32(copy[0][0] + 4), 1);
        assert_eq!(calls_to(&e, ALLOCATE_DATA), vec![vec![this, 2, 1, 10]]);
        assert_eq!(table(&e, this, 0x54, 2), vec![4, 2]);
        assert_eq!(table(&e, this, 0x58, 2), vec![2, 1]);
        assert_eq!(table(&e, this, 0x5c, 3), vec![0, 8, 10]);
        let data = e.mem.u32(this + 0x50);
        assert_eq!(e.mem.bytes(data, 10), (0..10).collect::<Vec<u8>>());
        assert_eq!(e.mem.u32(this + 0x64), 1);
        assert_eq!(e.mem.u32(this + 0x68), 1);
    }

    #[test]
    fn copy_constructor_without_palette_or_with_a_failed_allocation() {
        let mut e = copy_engine();
        let other = pixels(&mut e, 8);
        let this = e.mem.alloc(0x74);
        e.call(0x00a7_c030, &args![this, other]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        e.mem.set_u32(other + 0x4c, 0x5555);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x74);
        e.call(0x00a7_c030, &args![this, other]);
        assert!(calls_to(&e, PALETTE_COPY_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u32(this + 0x4c), 0);
    }

    #[test]
    fn create_object_builds_an_empty_pixel_data() {
        let mut e = copy_engine();
        let object = e.call(0x00a7_c3f0, &args![]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x74]]);
        assert_eq!(e.mem.u32(object), NI_PIXEL_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, PIXEL_FORMAT_CONSTRUCTOR),
            vec![vec![object + 8]]
        );
        assert_eq!(e.mem.u32(object + 0x68), 1);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_c3f0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCTOR).len(), 1);
    }

    type Transfers = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

    /// A stream whose input reads the next of `words` (or `size` bytes of 0xab) and whose
    /// output records each write (address, size, count address, count value).
    fn stream(e: &mut Engine, version: u32, words: Vec<u32>) -> (u32, Transfers) {
        let log: Transfers = Rc::default();
        let reads = log.clone();
        let mut queue = words.into_iter();
        e.register_double(0x0700_0008, move |e, a| {
            reads.borrow_mut().push((a[1], a[2], a[3], e.mem.u32(a[3])));
            if a[2] == 4 {
                let word = queue.next().unwrap();
                e.mem.set_u32(a[1], word);
            } else {
                for i in 0..a[2] {
                    e.mem.set_u8(a[1] + i, 0xab);
                }
            }
            a[2].into_ret()
        });
        let writes = log.clone();
        e.register_double(0x0700_000c, move |e, a| {
            writes
                .borrow_mut()
                .push((a[1], a[2], a[3], e.mem.u32(a[3])));
            a[2].into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0700_0008);
        e.mem.set_u32(file + 0xc, 0x0700_000c);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + STREAM_INPUT, file);
        e.mem.set_u32(stream + STREAM_OUTPUT, file);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        (stream, log)
    }

    #[test]
    fn load_binary_reads_the_levels_faces_and_pixels() {
        let mut e = copy_engine();
        for double in [OBJECT_LOAD_BINARY, PIXEL_FORMAT_LOAD, READ_LINK_ID] {
            e.register(double, |_, _| Ret::default());
        }
        // Levels 2, stride 1, (4, 2, 0), (2, 1, 8), last 10, faces 2.
        let (stream, log) = stream(
            &mut e,
            VERSION_WITH_FACES,
            vec![2, 1, 4, 2, 0, 2, 1, 8, 10, 2],
        );
        let this = e.mem.alloc(0x74);
        e.call(0x00a7_c470, &args![this, stream]);
        assert_eq!(
            calls_to(&e, PIXEL_FORMAT_LOAD),
            vec![vec![this + 8, stream]]
        );
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
        assert_eq!(e.mem.u32(this + 0x60), 2);
        assert_eq!(e.mem.u32(this + 0x64), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 2);
        assert_eq!(calls_to(&e, ALLOCATE_DATA), vec![vec![this, 2, 2, 10]]);
        assert_eq!(table(&e, this, 0x54, 2), vec![4, 2]);
        assert_eq!(table(&e, this, 0x58, 2), vec![2, 1]);
        assert_eq!(table(&e, this, 0x5c, 3), vec![0, 8, 10]);
        let log = log.borrow();
        assert_eq!(log.len(), 11);
        let pixels = e.mem.u32(this + 0x50);
        assert_eq!((log[10].0, log[10].1, log[10].3), (pixels, 20, 1));
        // One byte-count word for every read.
        assert!(log.iter().all(|r| r.2 == log[0].2));
        assert!(log[..10].iter().all(|r| r.3 == 4 && r.1 == 4));
    }

    #[test]
    fn load_binary_of_an_old_file_has_one_face() {
        let mut e = copy_engine();
        for double in [OBJECT_LOAD_BINARY, PIXEL_FORMAT_LOAD, READ_LINK_ID] {
            e.register(double, |_, _| Ret::default());
        }
        let (stream, log) = stream(&mut e, VERSION_WITH_FACES - 1, vec![0, 3, 6]);
        let this = e.mem.alloc(0x74);
        e.call(0x00a7_c470, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x6c), 1);
        assert_eq!(calls_to(&e, ALLOCATE_DATA), vec![vec![this, 0, 1, 6]]);
        assert_eq!(log.borrow().len(), 4);
        assert_eq!(log.borrow()[3].1, 6);
    }

    #[test]
    fn save_binary_writes_the_levels_faces_and_pixels() {
        let mut e = Engine::new();
        for double in [OBJECT_STREAM_BASE, PIXEL_FORMAT_SAVE] {
            e.register(double, |_, _| Ret::default());
        }
        let (stream, log) = stream(&mut e, 0, vec![]);
        e.put_vtable(0x0300_0000, &[0; 0xd]);
        e.mem.set_u32(0x0300_0030, 0x0300_1000);
        e.register(0x0300_1000, |_, _| Ret::default());
        e.mem.set_u32(stream, 0x0300_0000);
        e.call_log = Some(vec![]);
        let this = pixels(&mut e, 10);
        e.mem.set_u32(this + 0x6c, 3);
        e.mem.set_u32(this + 0x4c, 0x5555);
        e.call(0x00a7_c670, &args![this, stream]);
        assert_eq!(
            calls_to(&e, PIXEL_FORMAT_SAVE),
            vec![vec![this + 8, stream]]
        );
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![stream, 0x5555]]);
        let log = log.borrow();
        let widths = e.mem.u32(this + 0x54);
        let heights = e.mem.u32(this + 0x58);
        let offsets = e.mem.u32(this + 0x5c);
        let written: Vec<(u32, u32)> = log.iter().map(|w| (w.0, w.1)).collect();
        assert_eq!(
            written,
            vec![
                (this + 0x60, 4),
                (this + 0x64, 4),
                (widths, 4),
                (heights, 4),
                (offsets, 4),
                (widths + 4, 4),
                (heights + 4, 4),
                (offsets + 4, 4),
                (offsets + 8, 4),
                (this + 0x6c, 4),
                (e.mem.u32(this + 0x50), 30),
            ]
        );
        // The loop uses its own byte-count word.
        assert_ne!(log[2].2, log[0].2);
        assert!(log[2..8].iter().all(|w| w.2 == log[2].2));
        assert!(log[8..].iter().all(|w| w.2 == log[0].2));
        assert_eq!(log[10].3, 1);
    }

    #[test]
    fn viewer_strings_list_the_sizes() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_PIXEL_DATA_RTTI, 0x0109_d000);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(GET_VIEWER_STRING_UINT, |_, a| (0x6000 + a[1]).into_ret());
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 0xe, 8);
        let this = pixels(&mut e, 10);
        e.mem.set_u32(this + 0x6c, 3);
        e.call(0x00a7_c820, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_d000]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_UINT),
            vec![
                vec![NAME_WIDTH, 4],
                vec![NAME_HEIGHT, 2],
                vec![NAME_MIPMAP_LEVELS, 2],
                vec![NAME_FACES, 3],
                vec![NAME_TOTAL_SIZE, 30],
                vec![NAME_FACE_SIZE, 10],
            ]
        );
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 8]]);
        let held: Vec<u32> = (0..7).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(
            held,
            vec![0x5000, 0x6004, 0x6002, 0x6002, 0x6003, 0x601e, 0x600a]
        );
    }
}
