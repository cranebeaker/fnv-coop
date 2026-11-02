//! `NiMain/nipersistentsrctexturerendererdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a98480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a98240`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x94 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_persistent_src_texture_renderer_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_8240;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x94u32]);
    }
    this
}

/// `00a63520` (`cdecl`, one word; unit `NiMain/nistream.cpp`).
const UNREGISTER_NAME: u32 = 0x00a6_3520;
/// `00a64900` (`cdecl`, two words; engine map `NiStream::RegisterLoader`).
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// The class-name string both calls pass.
const CLASS_NAME: u32 = 0x0109_f0a0;
/// The creation function `00a98460` registers (`00a982d0`).
const CREATE_FUNCTION: u32 = 0x00a9_82d0;

// Translated from 00a98460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nipersistentsrctexturerendererdata.cpp`, no arguments):
/// passes the class-name string at `0109f0a0` to `00a63520`, then registers it with the
/// creation function `00a982d0` (`NiStream::RegisterLoader`, `00a64900`).
pub fn fn_00a98460(e: &mut Engine) {
    e.call(UNREGISTER_NAME, &args![CLASS_NAME]);
    e.call(REGISTER_LOADER, &args![CLASS_NAME, CREATE_FUNCTION]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type D = NiPersistentSrcTextureRendererData;
    vec![
        entry!(0x00a98480, ni_persistent_src_texture_renderer_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_8460, fn_00a98460()),
        entry!(
            0x00a9_8090,
            ni_persistent_src_texture_renderer_data_allocate_data(Ptr<D>, u32, u32, u32)
        ),
        entry!(
            0x00a9_80e0,
            ni_persistent_src_texture_renderer_data_free_data(Ptr<D>)
        ),
        entry!(
            0x00a9_8110,
            ni_persistent_src_texture_renderer_data_link_object(Ptr<D>, Ptr)
        ),
        entry!(
            0x00a9_8170,
            ni_persistent_src_texture_renderer_data_register_streamables(Ptr<D>, Ptr) -> bool
        ),
        entry!(
            0x00a9_82f0,
            ni_persistent_src_texture_renderer_data_is_equal(Ptr<D>, Ptr) -> bool
        ),
        entry!(
            0x00a9_84b0,
            ni_persistent_src_texture_renderer_data_save_binary(Ptr<D>, Ptr)
        ),
        entry!(
            0x00a9_8740,
            ni_persistent_src_texture_renderer_data_get_viewer_strings(Ptr<D>, Ptr)
        ),
        entry!(
            0x00e9_17e0,
            ni_persistent_src_texture_renderer_data_load_binary(Ptr<D>, Ptr)
        ),
    ]
}

layout! {
    /// `NiPersistentSrcTextureRendererData` (Xbox PDB), 0x94 bytes (the sized delete frees
    /// 0x94); its own fields at the PC offsets the code uses. The `NiTexture::RendererData`
    /// base holds the texture (+8), width (+0xc), height (+0x10) and pixel format (+0x14).
    pub struct NiPersistentSrcTextureRendererData: 0x94 {
        /// `m_pucPixels` (Xbox PDB).
        0x60 m_pucPixels: u32,
        /// `m_puiWidth` (Xbox PDB).
        0x64 m_puiWidth: u32,
        /// `m_puiHeight` (Xbox PDB).
        0x68 m_puiHeight: u32,
        /// `m_puiOffsetInBytes` (Xbox PDB).
        0x6c m_puiOffsetInBytes: u32,
        /// `m_uiPadOffsetInBytes` (Xbox PDB).
        0x70 m_uiPadOffsetInBytes: u32,
        /// `m_uiMipmapLevels` (Xbox PDB).
        0x74 m_uiMipmapLevels: u32,
        /// `m_uiPixelStride` (Xbox PDB).
        0x78 m_uiPixelStride: u32,
        /// `m_uiFaces` (Xbox PDB).
        0x7c m_uiFaces: u32,
        /// `m_eTargetPlatform` (Xbox PDB).
        0x80 m_eTargetPlatform: u32,
        /// `m_pucPristinePixels` (Xbox PDB).
        0x84 m_pucPristinePixels: u32,
        /// `m_uiPristineMaxOffsetInBytes` (Xbox PDB).
        0x88 m_uiPristineMaxOffsetInBytes: u32,
        /// `m_uiPristinePadOffsetInBytes` (Xbox PDB).
        0x8c m_uiPristinePadOffsetInBytes: u32,
        /// `m_spPalette` (Xbox PDB).
        0x90 m_spPalette: u32,
    }
}

/// `NiPersistentSrcTextureRendererData::ms_RTTI` (the record `IsEqual` looks for in the
/// other object's RTTI chain, and whose first word is the class name).
const NI_PERSISTENT_SRC_TEXTURE_RENDERER_DATA_RTTI: u32 = 0x011f_5f04;
/// `011f5f00`: a byte that, when set, keeps a pristine copy of the pixels on load and saves
/// it instead of the pixels (`ms_bToolModeStreaming` in the Xbox PDB, by its use; not
/// confirmed).
const TOOL_MODE_STREAMING: u32 = 0x011f_5f00;
/// `00aa1070` and `00aa10f0` (`cdecl`, one word): the Ni allocation and free.
const NEW_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `00a89af0` (`thiscall`, the stream; a 3-byte body the `LinkObject`s and `SaveBinary`s
/// call first: the `NiObject` base's).
const OBJECT_STREAM_BASE: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (Xbox PDB) and `00a645b0` (`thiscall` on the stream, no
/// stack word): the next link id's object, and the reading of a link id.
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiObject::RegisterStreamables`, `IsEqual`, `LoadBinary` and `GetViewerStrings` (Xbox
/// PDB, `thiscall`, one stack word each).
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `00a765c0` (`thiscall`, the other pixel format): the exe treats a nonzero answer as the
/// formats differing; `00a7ced0` and `00a7d1a0`: the pixel format's load and save.
const PIXEL_FORMAT_COMPARE: u32 = 0x00a7_65c0;
const PIXEL_FORMAT_LOAD: u32 = 0x00a7_ced0;
const PIXEL_FORMAT_SAVE: u32 = 0x00a7_d1a0;
/// CRT `memcpy` (`00ec44d0`).
const MEMCPY: u32 = 0x00ec_44d0;
/// The import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStream`'s file version (+0xd8), input (+0x24c, read function at +8) and output
/// (+0x250, write function at +0xc); a file's position is at +4.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// The first file version that stores the pad offset (20.2.0.6).
const VERSION_WITH_PAD: u32 = 0x1402_0006;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word), its `unsigned int` overload `_ov8`
/// and its string overload `_ov11` (`cdecl`, name and value).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_UINT: u32 = 0x00a7_d510;
const GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// `0096ad30` (set-size), `0096ae90` (set-at) and `0096a610` (`thiscall`, a pointer to the
/// element: the add the exe uses for the platform string of a known platform) on the
/// viewer-string array.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
const ARRAY_ADD: u32 = 0x0096_a610;
/// The viewer-string names and values: `"Width"`, `"Height"`, `"MipmapLevels"`,
/// `"Faces"`, `"TotalSizeInBytes"`, `"FaceSizeInBytes"`, `"TargetPlatform"` and the
/// platform texts `"NI_ANY - Generic"` (0), `"NI_XENON"` (1), `"NI_PS3"` (2), `"NI_DX9"` (3)
/// and `"UNKNOWN!"`.
const NAME_WIDTH: u32 = 0x0109_dc8c;
const NAME_HEIGHT: u32 = 0x0109_dc84;
const NAME_MIPMAP_LEVELS: u32 = 0x010a_09a8;
const NAME_FACES: u32 = 0x010a_09a0;
const NAME_TOTAL_SIZE: u32 = 0x0109_dc50;
const NAME_FACE_SIZE: u32 = 0x0109_dc40;
const NAME_TARGET_PLATFORM: u32 = 0x010a_0974;
const PLATFORM_ANY: u32 = 0x010a_0984;
const PLATFORM_XENON: u32 = 0x010a_0960;
const PLATFORM_PS3: u32 = 0x010a_0958;
const PLATFORM_DX9: u32 = 0x010a_096c;
const PLATFORM_UNKNOWN: u32 = 0x010a_094c;

/// Drops one reference of the counted object at `object` (`InterlockedDecrement` of its
/// count at +4; at zero its virtual +4).
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
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

// Translated from 00a98090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::AllocateData` (Xbox PDB): one block (`00aa1070`) of
/// `faces * round_up_4(face_size)` pixel bytes followed by the widths, heights and offsets
/// (`levels` words each, one more for the offsets), its parts stored at +0x60 .. +0x6c.
pub fn ni_persistent_src_texture_renderer_data_allocate_data(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    levels: u32,
    faces: u32,
    face_size: u32,
) {
    type D = NiPersistentSrcTextureRendererData;
    let pixel_bytes = faces.wrapping_mul(face_size.wrapping_add(3) & !3);
    let size = pixel_bytes
        .wrapping_add(levels.wrapping_mul(12))
        .wrapping_add(4);
    let block = e.call(NEW_BLOCK, &args![size]).u32();
    e.set(this, D::m_pucPixels, block);
    let widths = block.wrapping_add(pixel_bytes);
    e.set(this, D::m_puiWidth, widths);
    let heights = widths.wrapping_add(levels.wrapping_mul(4));
    let offsets = heights.wrapping_add(levels.wrapping_mul(4));
    e.set(this, D::m_puiHeight, heights);
    e.set(this, D::m_puiOffsetInBytes, offsets);
}

// Translated from 00a980e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::FreeData` (Xbox PDB): with a pixel block, frees it
/// (`00aa10f0`) and clears the four pointers.
pub fn ni_persistent_src_texture_renderer_data_free_data(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
) {
    type D = NiPersistentSrcTextureRendererData;
    let block = e.get(this, D::m_pucPixels);
    if block != 0 {
        e.call(FREE_BLOCK, &args![block]);
        e.set(this, D::m_puiOffsetInBytes, 0);
        e.set(this, D::m_puiHeight, 0);
        e.set(this, D::m_puiWidth, 0);
        e.set(this, D::m_pucPixels, 0);
    }
}

// Translated from 00a98110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::LinkObject` (Xbox PDB): the base's link step
/// (`00a89af0`), then the palette becomes the stream's object for the next link id
/// (`GetObjectFromLinkID`), assigned as a counted pointer.
pub fn ni_persistent_src_texture_renderer_data_link_object(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    stream: Ptr,
) {
    e.call(OBJECT_STREAM_BASE, &args![this, stream]);
    let palette = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let slot = this.addr() + 0x90;
    let old = e.mem.u32(slot);
    if old != palette {
        release(e, old);
        e.mem.set_u32(slot, palette);
        if palette != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![palette.wrapping_add(4)]);
        }
    }
}

// Translated from 00a98170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::RegisterStreamables` (Xbox PDB): false when
/// `NiObject`'s refuses; otherwise the palette (when set) registers itself (virtual +0x54)
/// and the answer is true.
pub fn ni_persistent_src_texture_renderer_data_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    stream: Ptr,
) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let palette = e.get(this, NiPersistentSrcTextureRendererData::m_spPalette);
    if palette != 0 {
        e.vcall(palette, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a982f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::IsEqual` (Xbox PDB): false for a null `other` or one
/// whose RTTI chain (its `GetRTTI`, virtual +8, then each record's base at +4) does not reach
/// `011f5f04`. Then `NiObject::IsEqual`, the pixel formats (`00a765c0` answering zero), the
/// palettes (this one's `IsEqual`, virtual +0x5c, when it has one), the level count and
/// stride, each level's width, height and offset, the last offset, the pad offset, the face
/// count and the target platform, and finally the first `offset[levels]` pixel bytes (words
/// while four or more remain, then up to three bytes) must all match.
pub fn ni_persistent_src_texture_renderer_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    other: Ptr,
) -> bool {
    type D = NiPersistentSrcTextureRendererData;
    let theirs = other.addr();
    if theirs == 0 {
        return false;
    }
    let mut rtti = e.vcall(theirs, 8, &args![]).u32();
    loop {
        if rtti == 0 {
            return false;
        }
        if rtti == NI_PERSISTENT_SRC_TEXTURE_RENDERER_DATA_RTTI {
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = this.addr();
    if e.call(PIXEL_FORMAT_COMPARE, &args![mine + 0x14, theirs + 0x14])
        .bool()
    {
        return false;
    }
    let palette = e.get(this, D::m_spPalette);
    if palette != 0 {
        let other_palette = e.mem.u32(theirs + 0x90);
        if !e.vcall(palette, 0x5c, &args![other_palette]).bool() {
            return false;
        }
    }
    let levels = e.get(this, D::m_uiMipmapLevels);
    if levels != e.mem.u32(theirs + 0x74) {
        return false;
    }
    if e.get(this, D::m_uiPixelStride) != e.mem.u32(theirs + 0x78) {
        return false;
    }
    let mut i = 0u32;
    if levels != 0 {
        let widths = e.get(this, D::m_puiWidth);
        let gap = e.mem.u32(theirs + 0x64).wrapping_sub(widths);
        loop {
            let at = widths.wrapping_add(4 * i);
            if e.mem.u32(at) != e.mem.u32(at.wrapping_add(gap)) {
                return false;
            }
            let heights = e.get(this, D::m_puiHeight);
            let other_heights = e.mem.u32(theirs + 0x68);
            if e.mem.u32(heights.wrapping_add(4 * i))
                != e.mem.u32(other_heights.wrapping_add(4 * i))
            {
                return false;
            }
            let offsets = e.get(this, D::m_puiOffsetInBytes);
            let other_offsets = e.mem.u32(theirs + 0x6c);
            if e.mem.u32(offsets.wrapping_add(4 * i))
                != e.mem.u32(other_offsets.wrapping_add(4 * i))
            {
                return false;
            }
            i += 1;
            if i >= e.get(this, D::m_uiMipmapLevels) {
                break;
            }
        }
    }
    let levels = e.get(this, D::m_uiMipmapLevels);
    let offsets = e.get(this, D::m_puiOffsetInBytes);
    let other_offsets = e.mem.u32(theirs + 0x6c);
    let last = levels.wrapping_mul(4);
    if e.mem.u32(last.wrapping_add(offsets)) != e.mem.u32(last.wrapping_add(other_offsets)) {
        return false;
    }
    for field in [0x70u32, 0x7c, 0x80] {
        if e.mem.u32(mine + field) != e.mem.u32(theirs + field) {
            return false;
        }
    }
    // The loop's level register holds the level count (read again by the loop).
    let mut size = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
    let mut their_pixels = e.mem.u32(theirs + 0x60);
    let mut my_pixels = e.get(this, D::m_pucPixels);
    while size >= 4 {
        if e.mem.u32(my_pixels) != e.mem.u32(their_pixels) {
            return false;
        }
        size -= 4;
        their_pixels = their_pixels.wrapping_add(4);
        my_pixels = my_pixels.wrapping_add(4);
    }
    if size == 0 {
        return true;
    }
    if e.mem.u8(their_pixels) != e.mem.u8(my_pixels) {
        return false;
    }
    if size <= 1 {
        return true;
    }
    if e.mem.u8(their_pixels + 1) != e.mem.u8(my_pixels + 1) {
        return false;
    }
    if size <= 2 {
        return true;
    }
    e.mem.u8(their_pixels + 2) == e.mem.u8(my_pixels + 2)
}

// Translated from 00a984b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::SaveBinary` (Xbox PDB): the base's step
/// (`00a89af0`); the pristine flag (kept in the low byte of the stream argument's stack
/// word) is set when tool-mode streaming (`011f5f00`) is on and the pristine max offset,
/// pad offset and pixels are all present. Then the pixel format (`00a7d1a0`), the palette's
/// link id (the stream's virtual +0x30), and to the stream's output: the level count and
/// stride (a local as the byte-count word, which then holds the level index), each level's
/// width, height and offset (a second local as the byte-count word), the pristine max and
/// pad offsets or the last offset and the pad offset, the face count, the platform, and the
/// pristine pixels (`max * faces` bytes) or the pixels (`offset[levels] * faces` bytes) with
/// the argument's word (now 1) as the byte-count word.
pub fn ni_persistent_src_texture_renderer_data_save_binary(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    stream: Ptr,
) {
    type D = NiPersistentSrcTextureRendererData;
    let me = this.addr();
    e.call(OBJECT_STREAM_BASE, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        let first = frame.addr();
        let second = first + 4;
        let argument = first + 8;
        e.mem.set_u32(argument, stream.addr());
        let pristine = e.mem.u8(TOOL_MODE_STREAMING) != 0
            && e.get(this, D::m_uiPristineMaxOffsetInBytes) != 0
            && e.get(this, D::m_uiPristinePadOffsetInBytes) != 0
            && e.get(this, D::m_pucPristinePixels) != 0;
        e.mem.set_u8(argument, u8::from(pristine));
        e.call(PIXEL_FORMAT_SAVE, &args![me + 0x14, stream]);
        let palette = e.get(this, D::m_spPalette);
        e.vcall(stream.addr(), 0x30, &args![palette]);
        stream_transfer(e, stream, true, me + 0x74, 4, first, 4);
        stream_transfer(e, stream, true, me + 0x78, 4, first, 4);
        e.mem.set_u32(first, 0);
        if e.get(this, D::m_uiMipmapLevels) != 0 {
            loop {
                let i = e.mem.u32(first);
                for table in [D::m_puiWidth, D::m_puiHeight, D::m_puiOffsetInBytes] {
                    let at = e.get(this, table).wrapping_add(i.wrapping_mul(4));
                    stream_transfer(e, stream, true, at, 4, second, 4);
                }
                let next = e.mem.u32(first).wrapping_add(1);
                e.mem.set_u32(first, next);
                if next >= e.get(this, D::m_uiMipmapLevels) {
                    break;
                }
            }
        }
        let pristine = e.mem.u8(argument) != 0;
        e.mem.set_u32(second, 4);
        if pristine {
            stream_transfer(e, stream, true, me + 0x88, 4, second, 4);
            stream_transfer(e, stream, true, me + 0x8c, 4, second, 4);
        } else {
            let levels = e.get(this, D::m_uiMipmapLevels);
            let last = e
                .get(this, D::m_puiOffsetInBytes)
                .wrapping_add(levels.wrapping_mul(4));
            stream_transfer(e, stream, true, last, 4, second, 4);
            stream_transfer(e, stream, true, me + 0x70, 4, second, 4);
        }
        stream_transfer(e, stream, true, me + 0x7c, 4, second, 4);
        stream_transfer(e, stream, true, me + 0x80, 4, second, 4);
        let pristine = e.mem.u8(argument) != 0;
        if pristine {
            let pixels = e.get(this, D::m_pucPristinePixels);
            let size = e
                .get(this, D::m_uiPristineMaxOffsetInBytes)
                .wrapping_mul(e.get(this, D::m_uiFaces));
            stream_transfer(e, stream, true, pixels, size, argument, 1);
        } else {
            let levels = e.get(this, D::m_uiMipmapLevels);
            let offsets = e.get(this, D::m_puiOffsetInBytes);
            let size = e
                .mem
                .u32(offsets.wrapping_add(levels.wrapping_mul(4)))
                .wrapping_mul(e.get(this, D::m_uiFaces));
            let pixels = e.get(this, D::m_pucPixels);
            stream_transfer(e, stream, true, pixels, size, argument, 1);
        }
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

// Translated from 00a98740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::GetViewerStrings` (Xbox PDB):
/// `NiObject::GetViewerStrings`, the class name (the first word of the record at
/// `011f5f04`), the first level's width and height, the level count, the face count,
/// `offset[levels] * faces` and `offset[levels]` (`NiGetViewerString_ov8`), then
/// `"TargetPlatform"` with the platform's text (`NiGetViewerString_ov11`; the compiler's
/// switch on the platform as `match` arms): for platforms 0 to 3 added through `0096a610`,
/// for any other (`"UNKNOWN!"`) appended like the others. Each string pointer is kept in the
/// `strings` argument's stack word, whose address the array receives.
pub fn ni_persistent_src_texture_renderer_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    strings: Ptr,
) {
    type D = NiPersistentSrcTextureRendererData;
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_PERSISTENT_SRC_TEXTURE_RENDERER_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        let face_size = |e: &Engine| {
            let levels = e.get(this, D::m_uiMipmapLevels);
            let offsets = e.get(this, D::m_puiOffsetInBytes);
            e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)))
        };
        for item in 0..6 {
            // Each value is read right before its call, as the exe reads it.
            let (name, value) = match item {
                0 => (NAME_WIDTH, e.mem.u32(e.get(this, D::m_puiWidth))),
                1 => (NAME_HEIGHT, e.mem.u32(e.get(this, D::m_puiHeight))),
                2 => (NAME_MIPMAP_LEVELS, e.get(this, D::m_uiMipmapLevels)),
                3 => (NAME_FACES, e.get(this, D::m_uiFaces)),
                4 => (
                    NAME_TOTAL_SIZE,
                    face_size(e).wrapping_mul(e.get(this, D::m_uiFaces)),
                ),
                _ => (NAME_FACE_SIZE, face_size(e)),
            };
            let text = e.call(GET_VIEWER_STRING_UINT, &args![name, value]).u32();
            e.mem.set_u32(slot, text);
            append_viewer_string(e, strings, slot);
        }
        let platform = match e.get(this, D::m_eTargetPlatform) {
            0 => Some(PLATFORM_ANY),
            1 => Some(PLATFORM_XENON),
            2 => Some(PLATFORM_PS3),
            3 => Some(PLATFORM_DX9),
            _ => None,
        };
        match platform {
            Some(text) => {
                let text = e
                    .call(GET_VIEWER_STRING_TEXT, &args![NAME_TARGET_PLATFORM, text])
                    .u32();
                e.mem.set_u32(slot, text);
                e.call(ARRAY_ADD, &args![strings, slot]);
            }
            None => {
                let text = e
                    .call(
                        GET_VIEWER_STRING_TEXT,
                        &args![NAME_TARGET_PLATFORM, PLATFORM_UNKNOWN],
                    )
                    .u32();
                e.mem.set_u32(slot, text);
                append_viewer_string(e, strings, slot);
            }
        }
    });
}

/// The load's stack frame (0xc8 bytes): two byte-count words (+0, +4; the second also
/// takes the platform read), the levels' offsets (+8, one more than the levels), widths
/// (+0x48) and heights (+0x88), room for 16 levels each.
const LOAD_FRAME_SIZE: u32 = 0xc8;
const LOAD_FRAME_OFFSETS: u32 = 0x08;
const LOAD_FRAME_WIDTHS: u32 = 0x48;
const LOAD_FRAME_HEIGHTS: u32 = 0x88;

// Translated from 00e917e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the
/// pixel format (`00a7ced0`), the palette's link id (`00a645b0`), then from the stream's
/// input: the level count and stride, each level's width, height and offset into the stack
/// frame (the exe moves the byte-count word between its two locals, keeping the input in
/// the other), the offset after the last level, the pad offset (read for files from
/// 20.2.0.6 on, else the last offset), the face count and the platform. `AllocateData`
/// (virtual +0x98 with levels, faces and the last offset) makes the buffers, the tables are
/// copied in (`_memcpy`) and the pixels read (`offset[levels] * faces` bytes, byte count 1).
/// With tool-mode streaming (`011f5f00`) a pristine copy of the pixels is kept (`00aa1070`,
/// `_memcpy`) with its max and pad offsets.
pub fn ni_persistent_src_texture_renderer_data_load_binary(
    e: &mut Engine,
    this: Ptr<NiPersistentSrcTextureRendererData>,
    stream: Ptr,
) {
    type D = NiPersistentSrcTextureRendererData;
    let me = this.addr();
    let s = stream.addr();
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(PIXEL_FORMAT_LOAD, &args![me + 0x14, stream]);
    e.call(READ_LINK_ID, &args![stream]);
    e.with_stack(LOAD_FRAME_SIZE, |e, frame| {
        let low = frame.addr();
        let high = low + 4;
        let offsets = low + LOAD_FRAME_OFFSETS;
        let widths = low + LOAD_FRAME_WIDTHS;
        let heights = low + LOAD_FRAME_HEIGHTS;
        stream_transfer(e, stream, false, me + 0x74, 4, high, 4);
        stream_transfer(e, stream, false, me + 0x78, 4, high, 4);
        let mut i = 0u32;
        while i < e.get(this, D::m_uiMipmapLevels) {
            let at = |table: u32| table.wrapping_add(i.wrapping_mul(4));
            let input = e.mem.u32(s + STREAM_INPUT);
            e.mem.set_u32(low, input);
            stream_transfer(e, stream, false, at(widths), 4, high, 4);
            let input = e.mem.u32(s + STREAM_INPUT);
            e.mem.set_u32(high, input);
            stream_transfer(e, stream, false, at(heights), 4, low, 4);
            let input = e.mem.u32(s + STREAM_INPUT);
            e.mem.set_u32(high, input);
            stream_transfer(e, stream, false, at(offsets), 4, low, 4);
            i += 1;
        }
        let levels = e.get(this, D::m_uiMipmapLevels);
        let last = offsets.wrapping_add(levels.wrapping_mul(4));
        stream_transfer(e, stream, false, last, 4, low, 4);
        if e.mem.u32(s + STREAM_VERSION) >= VERSION_WITH_PAD {
            e.set(this, D::m_uiPadOffsetInBytes, 0);
            stream_transfer(e, stream, false, me + 0x70, 4, low, 4);
        } else {
            let levels = e.get(this, D::m_uiMipmapLevels);
            let pad = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
            e.set(this, D::m_uiPadOffsetInBytes, pad);
        }
        stream_transfer(e, stream, false, me + 0x7c, 4, low, 4);
        stream_transfer(e, stream, false, high, 4, low, 4);
        let platform = e.mem.u32(high);
        let levels = e.get(this, D::m_uiMipmapLevels);
        e.set(this, D::m_eTargetPlatform, platform);
        let total = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
        let faces = e.get(this, D::m_uiFaces);
        e.vcall(me, 0x98, &args![levels, faces, total]);
        let bytes = e.get(this, D::m_uiMipmapLevels).wrapping_mul(4);
        let to = e.get(this, D::m_puiWidth);
        e.call(MEMCPY, &args![to, widths, bytes]);
        let to = e.get(this, D::m_puiHeight);
        e.call(MEMCPY, &args![to, heights, bytes]);
        let levels = e.get(this, D::m_uiMipmapLevels);
        let to = e.get(this, D::m_puiOffsetInBytes);
        e.call(
            MEMCPY,
            &args![to, offsets, levels.wrapping_mul(4).wrapping_add(4)],
        );
        let levels = e.get(this, D::m_uiMipmapLevels);
        let table = e.get(this, D::m_puiOffsetInBytes);
        let size = e
            .mem
            .u32(table.wrapping_add(levels.wrapping_mul(4)))
            .wrapping_mul(e.get(this, D::m_uiFaces));
        let pixels = e.get(this, D::m_pucPixels);
        stream_transfer(e, stream, false, pixels, size, low, 1);
    });
    if e.mem.u8(TOOL_MODE_STREAMING) != 0 {
        let levels = e.get(this, D::m_uiMipmapLevels);
        let table = e.get(this, D::m_puiOffsetInBytes);
        let face_size = e.mem.u32(table.wrapping_add(levels.wrapping_mul(4)));
        let pad = e.get(this, D::m_uiPadOffsetInBytes);
        let size = face_size.wrapping_mul(e.get(this, D::m_uiFaces));
        e.set(this, D::m_uiPristineMaxOffsetInBytes, face_size);
        e.set(this, D::m_uiPristinePadOffsetInBytes, pad);
        let copy = e.call(NEW_BLOCK, &args![size]).u32();
        e.set(this, D::m_pucPristinePixels, copy);
        let pixels = e.get(this, D::m_pucPixels);
        e.call(MEMCPY, &args![copy, pixels, size]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_persistent_src_texture_renderer_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_8240, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a98480, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a98480, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x94]]);
        assert_eq!(calls_to(&e, 0x00a9_8240), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a98460_reregisters_the_loader() {
        let mut e = Engine::new();
        e.register(UNREGISTER_NAME, |_, _| Ret::default());
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a9_8460, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (UNREGISTER_NAME, vec![CLASS_NAME]),
                (REGISTER_LOADER, vec![CLASS_NAME, CREATE_FUNCTION]),
            ]
        );
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

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.register(NEW_BLOCK, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A renderer data with two levels (widths 4, 2; heights 2, 1; offsets 0, 8, 10), the
    /// given pixel bytes, faces 1, stride 1, pad 12, platform 1.
    fn data(e: &mut Engine, size: u32) -> u32 {
        let this = e.mem.alloc(0x94);
        e.mem.set_u32(this + 0x74, 2);
        e.mem.set_u32(this + 0x78, 1);
        e.mem.set_u32(this + 0x7c, 1);
        e.mem.set_u32(this + 0x70, 12);
        e.mem.set_u32(this + 0x80, 1);
        let tables = e.mem.alloc(0x30);
        for (i, v) in [4u32, 2, 2, 1, 0, 8, size].iter().enumerate() {
            let at = match i {
                0 | 1 => tables + 4 * i as u32,
                2 | 3 => tables + 0x10 + 4 * (i as u32 - 2),
                _ => tables + 0x20 + 4 * (i as u32 - 4),
            };
            e.mem.set_u32(at, *v);
        }
        e.mem.set_u32(this + 0x64, tables);
        e.mem.set_u32(this + 0x68, tables + 0x10);
        e.mem.set_u32(this + 0x6c, tables + 0x20);
        let pixels = e.mem.alloc(size.max(1));
        for i in 0..size {
            e.mem.set_u8(pixels + i, i as u8);
        }
        e.mem.set_u32(this + 0x60, pixels);
        this
    }

    #[test]
    fn allocate_and_free_data() {
        let mut e = engine();
        let this = e.mem.alloc(0x94);
        e.call(0x00a9_8090, &args![this, 3u32, 2u32, 10u32]);
        // Two faces of 12 bytes (10 rounded up), then 3 + 3 + 4 words.
        assert_eq!(calls_to(&e, NEW_BLOCK), vec![vec![24 + 36 + 4]]);
        let block = e.mem.u32(this + 0x60);
        assert_eq!(e.mem.u32(this + 0x64), block + 24);
        assert_eq!(e.mem.u32(this + 0x68), block + 36);
        assert_eq!(e.mem.u32(this + 0x6c), block + 48);
        e.call(0x00a9_80e0, &args![this]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![block]]);
        assert_eq!(e.mem.bytes(this + 0x60, 16), vec![0; 16]);
        e.call(0x00a9_80e0, &args![this]);
        assert_eq!(calls_to(&e, FREE_BLOCK).len(), 1);
    }

    /// A counted object (count `count`) whose vtable holds `slots`.
    fn counted(e: &mut Engine, count: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x94);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn link_object_and_register_streamables_handle_the_palette() {
        let mut e = engine();
        e.register(OBJECT_STREAM_BASE, |_, _| Ret::default());
        e.register(0x0300_1000, |_, _| Ret::default());
        e.register(0x0300_2000, |_, _| Ret::default());
        let palette = counted(&mut e, 0, &[(0x54, 0x0300_2000)]);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| palette.into_ret());
        let this = e.mem.alloc(0x94);
        let old = counted(&mut e, 1, &[(4, 0x0300_1000)]);
        e.mem.set_u32(this + 0x90, old);
        e.call(0x00a9_8110, &args![this, 0x4444u32]);
        assert_eq!(e.mem.u32(this + 0x90), palette);
        assert_eq!(e.mem.u32(palette + 4), 1);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![old]]);
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a9_8170, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![palette, 0x4444]]);
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_8170, &args![this, 0x4444u32]).bool());
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0x90, 0);
        assert!(e.call(0x00a9_8170, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, 0x0300_2000).len(), 1);
    }

    /// `other`'s vtable answers an RTTI chain ending in the class's record.
    fn with_rtti(e: &mut Engine, other: u32, direct: bool) {
        e.map(0x011f_5000, 0x1000);
        let derived = e.mem.alloc(8);
        e.mem
            .set_u32(derived + 4, NI_PERSISTENT_SRC_TEXTURE_RENDERER_DATA_RTTI);
        let answer = if direct {
            NI_PERSISTENT_SRC_TEXTURE_RENDERER_DATA_RTTI
        } else {
            derived
        };
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0300_3000);
        e.mem.set_u32(other, vtable);
        e.register_double(0x0300_3000, move |_, _| answer.into_ret());
    }

    #[test]
    fn is_equal_checks_the_class_and_every_field() {
        let mut e = engine();
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(PIXEL_FORMAT_COMPARE, |_, _| 0u32.into_ret());
        let a = data(&mut e, 10);
        let b = data(&mut e, 10);
        with_rtti(&mut e, b, false);
        assert!(e.call(0x00a9_82f0, &args![a, b]).bool());
        assert!(!e.call(0x00a9_82f0, &args![a, 0u32]).bool());
        // Each field or table entry changed in turn.
        let fields: [(u32, Option<u32>); 9] = [
            (0x74, None),
            (0x78, None),
            (0x70, None),
            (0x7c, None),
            (0x80, None),
            (0x64, Some(4)),
            (0x68, Some(0)),
            (0x6c, Some(4)),
            (0x6c, Some(8)),
        ];
        for (field, entry) in fields {
            let c = data(&mut e, 10);
            with_rtti(&mut e, c, true);
            match entry {
                None => {
                    let v = e.mem.u32(c + field);
                    e.mem.set_u32(c + field, v + 1);
                }
                Some(at) => {
                    let table = e.mem.u32(c + field);
                    let v = e.mem.u32(table + at);
                    e.mem.set_u32(table + at, v + 1);
                }
            }
            assert!(!e.call(0x00a9_82f0, &args![a, c]).bool(), "{field:x}");
        }
        // A pixel byte in the tail.
        let c = data(&mut e, 10);
        with_rtti(&mut e, c, true);
        let pixels = e.mem.u32(c + 0x60);
        e.mem.set_u8(pixels + 9, 0xff);
        assert!(!e.call(0x00a9_82f0, &args![a, c]).bool());
        // An object of another class.
        let d = data(&mut e, 10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0300_4000);
        e.mem.set_u32(d, vtable);
        e.register(0x0300_4000, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_82f0, &args![a, d]).bool());
    }

    type Transfers = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

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
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x0300_5000);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(stream + STREAM_INPUT, file);
        e.mem.set_u32(stream + STREAM_OUTPUT, file);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.register(0x0300_5000, |_, _| Ret::default());
        (stream, log)
    }

    #[test]
    fn save_binary_writes_the_pixels_or_the_pristine_copy() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        for double in [OBJECT_STREAM_BASE, PIXEL_FORMAT_SAVE] {
            e.register(double, |_, _| Ret::default());
        }
        let (stream, log) = stream(&mut e, 0, vec![]);
        let this = data(&mut e, 10);
        e.call(0x00a9_84b0, &args![this, stream]);
        let widths = e.mem.u32(this + 0x64);
        let offsets = e.mem.u32(this + 0x6c);
        let written: Vec<(u32, u32)> = log.borrow().iter().map(|w| (w.0, w.1)).collect();
        assert_eq!(written.len(), 13);
        assert_eq!(written[2], (widths, 4));
        assert_eq!(written[8], (offsets + 8, 4));
        assert_eq!(written[9], (this + 0x70, 4));
        assert_eq!(written[12], (e.mem.u32(this + 0x60), 10));
        assert_eq!(log.borrow()[12].3, 1);
        assert_ne!(log.borrow()[0].2, log.borrow()[2].2);
        // With tool-mode streaming and a pristine copy, the pristine values are written.
        e.mem.set_u8(TOOL_MODE_STREAMING, 1);
        let pristine = e.mem.alloc(32);
        e.mem.set_u32(this + 0x84, pristine);
        e.mem.set_u32(this + 0x88, 16);
        e.mem.set_u32(this + 0x8c, 20);
        log.borrow_mut().clear();
        e.call(0x00a9_84b0, &args![this, stream]);
        let written: Vec<(u32, u32)> = log.borrow().iter().map(|w| (w.0, w.1)).collect();
        assert_eq!(written[8], (this + 0x88, 4));
        assert_eq!(written[9], (this + 0x8c, 4));
        assert_eq!(written[12], (pristine, 16));
    }

    #[test]
    fn load_binary_reads_the_levels_and_keeps_a_pristine_copy() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        for double in [OBJECT_LOAD_BINARY, PIXEL_FORMAT_LOAD, READ_LINK_ID] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(0x0300_6000, |e, a| {
            ni_persistent_src_texture_renderer_data_allocate_data(
                e,
                Ptr::new(a[0]),
                a[1],
                a[2],
                a[3],
            );
            Ret::default()
        });
        // Levels 2, stride 1, (4, 2, 0), (2, 1, 8), last 10, pad 12, faces 1, platform 3.
        let (stream, log) = stream(
            &mut e,
            VERSION_WITH_PAD,
            vec![2, 1, 4, 2, 0, 2, 1, 8, 10, 12, 1, 3],
        );
        e.mem.set_u8(TOOL_MODE_STREAMING, 1);
        let this = counted(&mut e, 1, &[(0x98, 0x0300_6000)]);
        e.call(0x00e9_17e0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x74), 2);
        assert_eq!(e.mem.u32(this + 0x70), 12);
        assert_eq!(e.mem.u32(this + 0x7c), 1);
        assert_eq!(e.mem.u32(this + 0x80), 3);
        let offsets = e.mem.u32(this + 0x6c);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(offsets + 4 * i))
                .collect::<Vec<_>>(),
            vec![0, 8, 10]
        );
        let pixels = e.mem.u32(this + 0x60);
        assert_eq!(e.mem.u8(pixels + 9), 0xab);
        let log = log.borrow();
        assert_eq!(log.len(), 13);
        assert_eq!((log[12].0, log[12].1, log[12].3), (pixels, 10, 1));
        // The pristine copy.
        let pristine = e.mem.u32(this + 0x84);
        assert_eq!(e.mem.u8(pristine + 9), 0xab);
        assert_eq!(e.mem.u32(this + 0x88), 10);
        assert_eq!(e.mem.u32(this + 0x8c), 12);
    }

    #[test]
    fn load_binary_of_an_old_file_takes_the_last_offset_as_pad() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        for double in [OBJECT_LOAD_BINARY, PIXEL_FORMAT_LOAD, READ_LINK_ID] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(0x0300_6000, |e, a| {
            ni_persistent_src_texture_renderer_data_allocate_data(
                e,
                Ptr::new(a[0]),
                a[1],
                a[2],
                a[3],
            );
            Ret::default()
        });
        let (stream, log) = stream(&mut e, VERSION_WITH_PAD - 1, vec![0, 1, 6, 2, 0]);
        let this = counted(&mut e, 1, &[(0x98, 0x0300_6000)]);
        e.call(0x00e9_17e0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x70), 6);
        assert_eq!(e.mem.u32(this + 0x7c), 2);
        assert_eq!(log.borrow().len(), 6);
        assert_eq!(log.borrow()[5].1, 12);
        assert_eq!(e.mem.u32(this + 0x84), 0);
    }

    #[test]
    fn viewer_strings_name_the_platform() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        e.mem
            .set_u32(NI_PERSISTENT_SRC_TEXTURE_RENDERER_DATA_RTTI, 0x011f_5100);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(GET_VIEWER_STRING_UINT, |_, a| (0x6000 + a[1]).into_ret());
        e.register(GET_VIEWER_STRING_TEXT, |_, _| 0x7000u32.into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.register(ARRAY_ADD, |_, _| Ret::default());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 16);
        let this = data(&mut e, 10);
        for (platform, text) in [
            (0u32, PLATFORM_ANY),
            (1, PLATFORM_XENON),
            (2, PLATFORM_PS3),
            (3, PLATFORM_DX9),
        ] {
            e.mem.set_u32(this + 0x80, platform);
            e.mem.set_u16(strings + 0xa, 0);
            e.call_log = Some(vec![]);
            e.call(0x00a9_8740, &args![this, strings]);
            assert_eq!(
                calls_to(&e, GET_VIEWER_STRING_TEXT),
                vec![vec![NAME_TARGET_PLATFORM, text]]
            );
            assert_eq!(calls_to(&e, ARRAY_ADD).len(), 1);
            assert_eq!(e.mem.u16(strings + 0xa), 7);
        }
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_UINT),
            vec![
                vec![NAME_WIDTH, 4],
                vec![NAME_HEIGHT, 2],
                vec![NAME_MIPMAP_LEVELS, 2],
                vec![NAME_FACES, 1],
                vec![NAME_TOTAL_SIZE, 10],
                vec![NAME_FACE_SIZE, 10],
            ]
        );
        e.mem.set_u32(this + 0x80, 9);
        e.mem.set_u16(strings + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_8740, &args![this, strings]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_TEXT),
            vec![vec![NAME_TARGET_PLATFORM, PLATFORM_UNKNOWN]]
        );
        assert!(calls_to(&e, ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u16(strings + 0xa), 8);
    }
}
