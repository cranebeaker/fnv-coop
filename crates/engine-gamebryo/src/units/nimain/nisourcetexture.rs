//! `NiMain/nisourcetexture.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a60160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a5fca0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x48 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_source_texture_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_fca0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

layout! {
    /// `NiSourceTexture` (Xbox PDB), 0x48 bytes (the sized delete frees 0x48); the PC code
    /// uses the same offsets.
    pub struct NiSourceTexture: 0x48 {
        /// `NiTexture::m_kFormatPrefs.m_ePixelLayout` (Xbox PDB).
        0x18 m_ePixelLayout: u32,
        /// `NiTexture::m_kFormatPrefs.m_eAlphaFmt` (Xbox PDB).
        0x1c m_eAlphaFmt: u32,
        /// `NiTexture::m_kFormatPrefs.m_eMipMapped` (Xbox PDB).
        0x20 m_eMipMapped: u32,
        /// `NiTexture::m_pkRendererData` (Xbox PDB).
        0x24 m_pkRendererData: u32,
        /// `m_kFilename` (Xbox PDB): an `NiFixedString` handle.
        0x30 m_kFilename: u32,
        /// `m_kPlatformFilename` (Xbox PDB): an `NiFixedString` handle.
        0x34 m_kPlatformFilename: u32,
        /// `m_spPersistentSrcRendererData` (Xbox PDB).
        0x38 m_spPersistentSrcRendererData: u32,
        /// `m_spSrcPixelData` (Xbox PDB).
        0x3c m_spSrcPixelData: u32,
        /// `m_bStatic` (Xbox PDB).
        0x40 m_bStatic: u8,
        /// `m_bLoadDirectToRendererHint` (Xbox PDB).
        0x41 m_bLoadDirectToRendererHint: u8,
        /// `m_bSrcRendererDataIsPersistent` (Xbox PDB).
        0x42 m_bSrcRendererDataIsPersistent: u8,
        /// `cLooked` (Xbox PDB).
        0x43 cLooked: u8,
        /// `m_pFile` (Xbox PDB).
        0x44 m_pFile: u32,
    }
}

/// `NiSourceTexture::ms_RTTI` (the address `GetRTTI` answers).
const NI_SOURCE_TEXTURE_RTTI: u32 = 0x011f_444c;
/// The virtual tables of `NiTexture` and `NiSourceTexture`.
const NI_TEXTURE_VTABLE: u32 = 0x0109_b944;
const NI_SOURCE_TEXTURE_VTABLE: u32 = 0x0109_b9ec;
/// The handle of the empty `NiFixedString` (`0109b220`); a non-empty handle points 8 bytes
/// past its reference count.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const FIXED_STRING_COUNT: u32 = 8;
/// `NiRenderer::ms_pkRenderer` (`011f4748`) and the renderer's virtual +0x13c (`__thiscall`,
/// one word: the texture), which makes the renderer data.
const RENDERER_INSTANCE: u32 = 0x011f_4748;
const RENDERER_CREATE_SOURCE_TEXTURE_DATA_SLOT: u32 = 0x13c;
/// The byte (`011f4448`) that makes a texture call its virtual +0xa8 after its renderer data
/// is made; the byte (`011a9594`) that makes textures load their pixels at once (virtual
/// +0xac); the critical section (`011f4800`) that load runs in; and the byte (`011f4780`)
/// `LoadBinary` sets before it looks for the file.
const CALL_AFTER_CREATE: u32 = 0x011f_4448;
const LOAD_AT_ONCE: u32 = 0x011a_9594;
const LOAD_CRITICAL_SECTION: u32 = 0x011f_4800;
const LOOKING_FOR_FILE: u32 = 0x011f_4780;
/// The texture's virtuals: scalar deleting destructor (+0x0), the call after creation
/// (+0xa8), the pixel load (+0xac).
const DELETING_DESTRUCTOR_SLOT: u32 = 0x0;
const AFTER_CREATE_SLOT: u32 = 0xa8;
const LOAD_PIXELS_SLOT: u32 = 0xac;
/// `NiRefObject::m_uiRefCount` (+0x4) and `DeleteThis` (virtual +0x4).
const REF_COUNT: u32 = 0x4;
const DELETE_THIS_SLOT: u32 = 0x4;
/// Renderer data fields the size getters read: width (+0xc) and height (+0x10); the
/// persistent data's width/height tables (+0x64, +0x68); the pixel data's (+0x54, +0x58).
const RENDERER_DATA_WIDTH: u32 = 0xc;
const RENDERER_DATA_HEIGHT: u32 = 0x10;
const PERSISTENT_WIDTHS: u32 = 0x64;
const PERSISTENT_HEIGHTS: u32 = 0x68;
const PIXEL_DATA_WIDTHS: u32 = 0x54;
const PIXEL_DATA_HEIGHTS: u32 = 0x58;
/// `NiStream`'s file version (+0xd8) and input stream (+0x24c); the input stream's read
/// function (+0x8: cdecl, stream, buffer, bytes, component sizes, component count) and
/// position (+0x4); `NiStream`'s virtual +0x28 (one word: a texture) that `LoadBinary`
/// calls on a texture found by name.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_POSITION: u32 = 0x4;
const STREAM_FOUND_TEXTURE_SLOT: u32 = 0x28;

// Callees outside this unit.
/// `00a5c200` and `00a5c240`: the `NiObjectNET` constructor and destructor bodies.
const NI_OBJECT_NET_CONSTRUCTOR: u32 = 0x00a5_c200;
const NI_OBJECT_NET_DESTRUCTOR: u32 = 0x00a5_c240;
/// `NiTexture::~NiTexture` (`00a5f800`, the part that leaves the texture list).
const NI_TEXTURE_DESTRUCTOR: u32 = 0x00a5_f800;
/// `00a5f7b0` (`__thiscall`): adds the texture to the list of all textures.
const TEXTURE_LIST_ADD: u32 = 0x00a5_f7b0;
/// `00a5f860` (`__thiscall` on a pointer slot, one word): `NiPointer<T>::operator=`'s shape.
const POINTER_ASSIGN: u32 = 0x00a5_f860;
/// `00aa13e0` (cdecl, a size): the object allocation.
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// `NiStandardizeFilePath` (`00a57500`, cdecl: a fixed string) and
/// `NiImageConverter::ConvertFilenameToPlatformSpecific` (`00a76960`, cdecl: source,
/// destination fixed strings).
const STANDARDIZE_FILE_PATH: u32 = 0x00a5_7500;
const CONVERT_FILENAME: u32 = 0x00a7_6960;
/// `00a5b690` (cdecl, one word: a C string): the fixed-string handle for it (one reference).
const FIXED_STRING_FROM: u32 = 0x00a5_b690;
/// `00a2e750` (`__thiscall` on a fixed string, one word: a C string): its assignment.
const FIXED_STRING_ASSIGN: u32 = 0x00a2_e750;
/// The base class's `LoadBinary` (`00a47350`) and `LinkObject` (`00a47360`), `__thiscall`
/// with the stream (the map names them after `NiSequenceStreamHelper`, folded bodies).
const BASE_LOAD_BINARY: u32 = 0x00a4_7350;
const BASE_LINK_OBJECT: u32 = 0x00a4_7360;
/// `NiStream` methods (`__thiscall` on the stream): `GetNumberOfLinkIDs` (`00a63460`),
/// `GetObjectFromLinkID` (`00a63430`), `LoadCStringAsFixedString` (`00a64ea0`),
/// `LoadFixedString` (`00a64df0`), `SetNumberOfLinkIDs` (`00a64700`), the link-id read
/// `00a645b0`, `00a636f0` (one word: the platform file name), `00a636b0` (the next candidate
/// path into a buffer of a size; answers whether there was one), `GetTexture` (`00a633d0`:
/// name, `NiPointer` out) and `SetTexture` (`00a633b0`).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_SET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_4700;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_00A636F0: u32 = 0x00a6_36f0;
const STREAM_NEXT_PATH: u32 = 0x00a6_36b0;
const STREAM_GET_TEXTURE: u32 = 0x00a6_33d0;
const STREAM_SET_TEXTURE: u32 = 0x00a6_33b0;
/// `NiFile::Access` (`00aa1a30`, cdecl: name, mode).
const FILE_ACCESS: u32 = 0x00aa_1a30;
/// `InterlockedIncrement`/`InterlockedDecrement`, `EnterCriticalSection`/
/// `LeaveCriticalSection` import slots (stdcall, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The stream versions `LoadBinary` tests.
const VERSION_10_0_1_4: u32 = 0x0a00_0104;
const VERSION_20_1_0_1: u32 = 0x1401_0001;
const VERSION_20_2_0_4: u32 = 0x1402_0004;
const VERSION_3_3_0_12: u32 = 0x0303_000c;
const VERSION_5_0_0_1: u32 = 0x0500_0001;
const VERSION_10_1_0_103: u32 = 0x0a01_0067;

/// A reference given up as the code inlines it (deleted at the last one).
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object + REF_COUNT])
            .u32();
        if count == 0 {
            e.vcall(object, DELETE_THIS_SLOT, &args![]);
        }
    }
}

/// `m_spSrcPixelData`/`m_spPersistentSrcRendererData` released and cleared when set.
fn release_and_clear(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(slot, 0);
    }
}

/// A fixed-string handle's reference given up (nothing for the empty handle).
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(
            INTERLOCKED_DECREMENT,
            &args![handle.wrapping_sub(FIXED_STRING_COUNT)],
        );
    }
}

/// `m_kFilename = name` as the code inlines it, then the path standardized and the platform
/// file name made from it.
fn set_filename_inline(e: &mut Engine, this: Ptr<NiSourceTexture>, name: u32) {
    let slot = this.addr() + 0x30;
    if e.mem.u32(slot) != name {
        if name != e.mem.u32(EMPTY_FIXED_STRING) {
            e.call(
                INTERLOCKED_INCREMENT,
                &args![name.wrapping_sub(FIXED_STRING_COUNT)],
            );
        }
        let old = e.mem.u32(slot);
        release_fixed_string(e, old);
        e.mem.set_u32(slot, name);
    }
    e.call(STANDARDIZE_FILE_PATH, &args![slot]);
    e.call(CONVERT_FILENAME, &args![slot, this.addr() + 0x34]);
}

/// The end of both `Create`s: when textures load at once (`011a9594`) the pixel load runs
/// (virtual +0xac) and a texture that fails it is deleted (null is answered).
fn load_at_once(e: &mut Engine, texture: Ptr<NiSourceTexture>) -> Ptr<NiSourceTexture> {
    if e.mem.u8(LOAD_AT_ONCE) != 0 && !e.vcall(texture.addr(), LOAD_PIXELS_SLOT, &args![]).bool() {
        e.vcall(texture.addr(), DELETING_DESTRUCTOR_SLOT, &args![1u32]);
        return Ptr::new(0);
    }
    texture
}

/// A new texture (`00aa13e0` of 0x48 bytes and the constructor), or address 0 when the
/// allocation fails (the code goes on with it).
fn new_texture(e: &mut Engine) -> Ptr<NiSourceTexture> {
    let memory = e.call(NEW_OBJECT, &args![0x48u32]).u32();
    if memory != 0 {
        ni_source_texture_ni_source_texture(e, Ptr::new(memory))
    } else {
        Ptr::new(0)
    }
}

/// Copies the three format-preference words into the texture.
fn set_prefs(e: &mut Engine, texture: Ptr<NiSourceTexture>, prefs: u32) {
    let layout = e.mem.u32(prefs);
    e.set(texture, NiSourceTexture::m_ePixelLayout, layout);
    let alpha = e.mem.u32(prefs + 4);
    e.set(texture, NiSourceTexture::m_eAlphaFmt, alpha);
    let mipmap = e.mem.u32(prefs + 8);
    e.set(texture, NiSourceTexture::m_eMipMapped, mipmap);
}

// Translated from 00a5f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::CreateRendererData` (Xbox PDB): without a renderer the file is deleted
/// and cleared and the answer is false. With renderer data already the file is deleted and
/// cleared (when there is one) and the answer is true. Otherwise the renderer makes the data
/// (virtual +0x13c): a failure answers false (the file stays); on success the texture's
/// virtual +0xa8 runs when `011f4448` is set, the texture has data and is static, the file
/// pointer is cleared (not deleted) and the answer is true.
pub fn ni_source_texture_create_renderer_data(e: &mut Engine, this: Ptr<NiSourceTexture>) -> bool {
    let renderer = e.mem.u32(RENDERER_INSTANCE);
    if renderer == 0 {
        let file = e.get(this, NiSourceTexture::m_pFile);
        if file != 0 {
            e.vcall(file, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
            e.set(this, NiSourceTexture::m_pFile, 0);
        }
        return false;
    }
    if e.get(this, NiSourceTexture::m_pkRendererData) != 0 {
        let file = e.get(this, NiSourceTexture::m_pFile);
        if file != 0 {
            e.vcall(file, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
            e.set(this, NiSourceTexture::m_pFile, 0);
        }
        return true;
    }
    if !e
        .vcall(
            renderer,
            RENDERER_CREATE_SOURCE_TEXTURE_DATA_SLOT,
            &args![this],
        )
        .bool()
    {
        return false;
    }
    if e.mem.u8(CALL_AFTER_CREATE) != 0
        && e.get(this, NiSourceTexture::m_pkRendererData) != 0
        && e.get(this, NiSourceTexture::m_bStatic) != 0
    {
        e.vcall(this.addr(), AFTER_CREATE_SLOT, &args![]);
    }
    e.set(this, NiSourceTexture::m_pFile, 0);
    true
}

// Translated from 00a5f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::GetWidth` (Xbox PDB): from the renderer data, else the persistent data's
/// level 0, else the pixel data's level 0, else 0.
pub fn ni_source_texture_get_width(e: &mut Engine, this: Ptr<NiSourceTexture>) -> u32 {
    let data = e.get(this, NiSourceTexture::m_pkRendererData);
    if data != 0 {
        return e.mem.u32(data + RENDERER_DATA_WIDTH);
    }
    let persistent = e.get(this, NiSourceTexture::m_spPersistentSrcRendererData);
    if persistent != 0 {
        return e.mem.u32(e.mem.u32(persistent + PERSISTENT_WIDTHS));
    }
    let pixels = e.get(this, NiSourceTexture::m_spSrcPixelData);
    if pixels != 0 {
        return e.mem.u32(e.mem.u32(pixels + PIXEL_DATA_WIDTHS));
    }
    0
}

// Translated from 00a5f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::GetHeight` (Xbox PDB): as [`ni_source_texture_get_width`] for the height.
pub fn ni_source_texture_get_height(e: &mut Engine, this: Ptr<NiSourceTexture>) -> u32 {
    let data = e.get(this, NiSourceTexture::m_pkRendererData);
    if data != 0 {
        return e.mem.u32(data + RENDERER_DATA_HEIGHT);
    }
    let persistent = e.get(this, NiSourceTexture::m_spPersistentSrcRendererData);
    if persistent != 0 {
        return e.mem.u32(e.mem.u32(persistent + PERSISTENT_HEIGHTS));
    }
    let pixels = e.get(this, NiSourceTexture::m_spSrcPixelData);
    if pixels != 0 {
        return e.mem.u32(e.mem.u32(pixels + PIXEL_DATA_HEIGHTS));
    }
    0
}

// Translated from 00a5f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::LinkObject` (Xbox PDB; `__thiscall`, one word: the stream): the base
/// class's link, then, when the stream has link ids, the linked object becomes the
/// persistent data (files from 20.2.0.4 on whose texture is marked persistent; the pixel data
/// is released and cleared) or the pixel data (the persistent data released and cleared).
pub fn ni_source_texture_link_object(e: &mut Engine, this: Ptr<NiSourceTexture>, stream: Ptr) {
    let base = this.addr();
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32() == 0 {
        return;
    }
    let persistent = e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_20_2_0_4
        && e.get(this, NiSourceTexture::m_bSrcRendererDataIsPersistent) != 0;
    let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    if persistent {
        e.call(POINTER_ASSIGN, &args![base + 0x38, object]);
        release_and_clear(e, base + 0x3c);
    } else {
        e.call(POINTER_ASSIGN, &args![base + 0x3c, object]);
        release_and_clear(e, base + 0x38);
    }
}

// Translated from 00a5fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, one word it does not read): when textures load at once
/// (`011a9594`), runs the texture's pixel load (virtual +0xac) inside the load critical
/// section (`011f4800`; the leave is a tail jump that takes the stack word's place).
pub fn fn_00a5fa50(e: &mut Engine, this: Ptr<NiSourceTexture>, _unused_1: u32) {
    if e.mem.u8(LOAD_AT_ONCE) == 0 {
        return;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![LOAD_CRITICAL_SECTION]);
    e.vcall(this.addr(), LOAD_PIXELS_SLOT, &args![]);
    e.call(LEAVE_CRITICAL_SECTION, &args![LOAD_CRITICAL_SECTION]);
}

// Translated from 00a5fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::SetFilename` (Xbox PDB; `__thiscall`, one word: an `NiFixedString`): the
/// file name takes the handle (a reference on the new one, the old one's given up), then the
/// path is standardized (`NiStandardizeFilePath`) and the platform file name made from it
/// (`ConvertFilenameToPlatformSpecific`).
pub fn ni_source_texture_set_filename(e: &mut Engine, this: Ptr<NiSourceTexture>, name: Ptr) {
    let handle = e.mem.u32(name.addr());
    set_filename_inline(e, this, handle);
}

// Translated from 00a5fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::NiSourceTexture` (Xbox PDB): the `NiObjectNET` constructor, the
/// `NiTexture` part inline (its table, preferences 6/3/2, no renderer data, added to the texture
/// list by `00a5f7b0`), then the class's table, both file names the empty string (copied
/// without a reference), no persistent or pixel data, static and loading straight to the
/// renderer, not persistent, not looked at, and no file. Answers `this`.
pub fn ni_source_texture_ni_source_texture(
    e: &mut Engine,
    this: Ptr<NiSourceTexture>,
) -> Ptr<NiSourceTexture> {
    e.call(NI_OBJECT_NET_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), NI_TEXTURE_VTABLE);
    e.set(this, NiSourceTexture::m_ePixelLayout, 6);
    e.set(this, NiSourceTexture::m_eAlphaFmt, 3);
    e.set(this, NiSourceTexture::m_eMipMapped, 2);
    e.set(this, NiSourceTexture::m_pkRendererData, 0);
    e.call(TEXTURE_LIST_ADD, &args![this]);
    e.mem.set_u32(this.addr(), NI_SOURCE_TEXTURE_VTABLE);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.set(this, NiSourceTexture::m_kFilename, empty);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.set(this, NiSourceTexture::m_kPlatformFilename, empty);
    e.set(this, NiSourceTexture::m_spPersistentSrcRendererData, 0);
    e.set(this, NiSourceTexture::m_spSrcPixelData, 0);
    e.set(this, NiSourceTexture::m_bStatic, 1);
    e.set(this, NiSourceTexture::m_bLoadDirectToRendererHint, 1);
    e.set(this, NiSourceTexture::m_bSrcRendererDataIsPersistent, 0);
    e.set(this, NiSourceTexture::cLooked, 0);
    e.set(this, NiSourceTexture::m_pFile, 0);
    this
}

// Translated from 00a5fc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::GetRTTI` (Xbox PDB): the class's `NiRTTI` record.
pub fn ni_source_texture_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SOURCE_TEXTURE_RTTI)
}

// Translated from 00a5fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::GetFilename` (Xbox PDB): the address of `m_kFilename`.
pub fn ni_source_texture_get_filename(_e: &mut Engine, this: Ptr<NiSourceTexture>) -> Ptr {
    Ptr::new(this.addr() + 0x30)
}

// Translated from 00a5fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSourceTexture` destructor body (unnamed in the map): the class's table; the pixel
/// and persistent data released and cleared, then released again if set (the members'
/// destructors); both file names' references given up; then `NiTexture`'s table, its renderer
/// data deleted (virtual +0 with 1), `NiTexture::~NiTexture` and the `NiObjectNET` destructor
/// (a tail jump).
pub fn fn_00a5fca0(e: &mut Engine, this: Ptr<NiSourceTexture>) {
    let base = this.addr();
    e.mem.set_u32(base, NI_SOURCE_TEXTURE_VTABLE);
    release_and_clear(e, base + 0x3c);
    release_and_clear(e, base + 0x38);
    let pixels = e.get(this, NiSourceTexture::m_spSrcPixelData);
    release(e, pixels);
    let persistent = e.get(this, NiSourceTexture::m_spPersistentSrcRendererData);
    release(e, persistent);
    let platform = e.get(this, NiSourceTexture::m_kPlatformFilename);
    release_fixed_string(e, platform);
    let name = e.get(this, NiSourceTexture::m_kFilename);
    release_fixed_string(e, name);
    let data = e.get(this, NiSourceTexture::m_pkRendererData);
    e.mem.set_u32(base, NI_TEXTURE_VTABLE);
    if data != 0 {
        e.vcall(data, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
    }
    e.call(NI_TEXTURE_DESTRUCTOR, &args![this]);
    e.call(NI_OBJECT_NET_DESTRUCTOR, &args![this]);
}

// Translated from 00a5fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::Create` (Xbox PDB; cdecl, four words: file name, format preferences,
/// load-straight hint, persistent flag): a new texture with the preferences, both flags (low
/// bytes) and the file name set as [`ni_source_texture_set_filename`] does; with loading at
/// once a texture whose pixels do not load is deleted and null answered.
pub fn ni_source_texture_create(
    e: &mut Engine,
    name: Ptr,
    prefs: Ptr,
    load_direct: u32,
    persistent: u32,
) -> Ptr<NiSourceTexture> {
    let texture = new_texture(e);
    set_prefs(e, texture, prefs.addr());
    e.set(
        texture,
        NiSourceTexture::m_bLoadDirectToRendererHint,
        load_direct as u8,
    );
    e.set(
        texture,
        NiSourceTexture::m_bSrcRendererDataIsPersistent,
        persistent as u8,
    );
    let handle = e.mem.u32(name.addr());
    set_filename_inline(e, texture, handle);
    load_at_once(e, texture)
}

// Translated from 00a5ff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::Create` (Xbox PDB `Create_ov2`; cdecl, three words: pixel data, a C
/// string name or null, format preferences): a new texture with the preferences and the pixel
/// data (as an `NiPointer`); with a name, its fixed string (`00a5b690`, held in the third
/// argument's stack word) is set as the file name ([`ni_source_texture_set_filename`]) and
/// its reference given up. Loading at once as the other `Create`.
pub fn ni_source_texture_create_ov2(
    e: &mut Engine,
    pixel_data: Ptr,
    name: Ptr,
    prefs: Ptr,
) -> Ptr<NiSourceTexture> {
    let texture = new_texture(e);
    set_prefs(e, texture, prefs.addr());
    let slot = texture.addr() + 0x3c;
    let old = e.mem.u32(slot);
    if old != pixel_data.addr() {
        release(e, old);
        e.mem.set_u32(slot, pixel_data.addr());
        if !pixel_data.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![pixel_data.addr() + REF_COUNT]);
        }
    }
    if !name.is_null() {
        let handle = e.call(FIXED_STRING_FROM, &args![name]).u32();
        e.with_stack(4, |e, temporary| {
            e.mem.set_u32(temporary.addr(), handle);
            ni_source_texture_set_filename(e, texture, temporary);
        });
        release_fixed_string(e, handle);
    }
    load_at_once(e, texture)
}

// Translated from 00a60070 (no function in Ghidra; disassembled bytes, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::CreateObject`'s shape (no words; the stream factory): a new 0x48-byte
/// texture constructed by [`ni_source_texture_ni_source_texture`] (a tail jump), or null.
pub fn fn_00a60070(e: &mut Engine) -> Ptr<NiSourceTexture> {
    let memory = e.call(NEW_OBJECT, &args![0x48u32]).u32();
    if memory == 0 {
        return Ptr::new(0);
    }
    ni_source_texture_ni_source_texture(e, Ptr::new(memory))
}

/// One read through the input stream: `bytes` into `buffer`, with the component sizes at
/// `sizes` (one component); the position moves on by what was read.
fn read(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, sizes: u32) {
    let input = e.mem.u32(stream + STREAM_INPUT);
    let function = e.mem.u32(input + BINARY_STREAM_READ);
    let got = e
        .call(function, &args![input, buffer, bytes, sizes, 1u32])
        .u32();
    let position = e.mem.u32(input + BINARY_STREAM_POSITION).wrapping_add(got);
    e.mem.set_u32(input + BINARY_STREAM_POSITION, position);
}

// Translated from 00a60180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a60180 - the /GS stack cookie (stored at frame +0x118 and checked through 00ec408c) is not translated
/// `NiSourceTexture::LoadBinary` (Xbox PDB; `__thiscall`, one word: the stream). The base
/// class's load, a reference held on the texture while it runs. Files before 10.0.1.4 read an
/// "external" byte: external textures load their file name as a C string and drop the pixel
/// data (no link ids); internal ones clear the file name and read a byte saying whether a
/// link id (the pixel data) follows. Later files read the external byte, the file name (a C
/// string before 20.1.0.1, a fixed string after) and one link id; the old files always drop
/// the persistent data. A non-empty name gets its platform name.
/// External: the stream is told the platform name (`00a636f0`) and `011f4780` is set; the
/// candidate paths (`00a636b0`, 0x104 bytes) are tried against the stream's textures
/// (`GetTexture`); a texture found becomes the platform name's source and the stream's
/// virtual +0x28 runs on it; failing that, the first candidate that exists on disk
/// (`NiFile::Access`) becomes the platform name and the texture registers itself
/// (`SetTexture`). Internal with a platform name: a texture of that name already in the stream
/// goes to virtual +0x28, otherwise this one registers. The found texture's reference is given
/// up. Then the format preferences (layout, mipmap, alpha: words), with old layouts mapped
/// (4 before 3.3.0.12 and 5 before 5.0.0.1 become 6), the static byte, from 10.1.0.103 the
/// load-straight byte and from 20.2.0.4 the persistent byte. The held reference is given up.
pub fn ni_source_texture_load_binary(e: &mut Engine, this: Ptr<NiSourceTexture>, stream: Ptr) {
    let base = this.addr();
    let stream = stream.addr();
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(INTERLOCKED_INCREMENT, &args![base + REF_COUNT]);
    e.with_stack(0x11c, |e, frame| {
        let frame = frame.addr();
        // The frame's slots (`ESP`-relative in the exe): two bytes read from the stream
        // (+0x03, +0x12), the found texture / component size word (+0x04), the "external"
        // flag (+0x0b), a second size/value word (+0x0c), the external byte of old files
        // (+0x13) and the path buffer (+0x14, 0x104 bytes).
        let byte_03 = frame + 0x03;
        let found = frame + 0x04;
        let external = frame + 0x0b;
        let word_0c = frame + 0x0c;
        let byte_12 = frame + 0x12;
        let old_external = frame + 0x13;
        let path = frame + 0x14;
        let version = e.mem.u32(stream + STREAM_VERSION);
        e.mem.set_u8(external, 0);
        e.mem.set_u32(found, 1);
        let name_slot = base + 0x30;
        if version < VERSION_10_0_1_4 {
            read(e, stream, old_external, 1, found);
            if e.mem.u8(old_external) != 0 {
                e.call(
                    STREAM_LOAD_C_STRING_AS_FIXED_STRING,
                    &args![stream, name_slot],
                );
                release_and_clear(e, base + 0x3c);
                e.mem.set_u8(external, 1);
                e.call(STREAM_SET_NUMBER_OF_LINK_IDS, &args![stream, 0u32]);
            } else {
                let old = e.mem.u32(name_slot);
                if old != 0 {
                    let handle = e.call(FIXED_STRING_FROM, &args![0u32]).u32();
                    e.mem.set_u32(name_slot, handle);
                    release_fixed_string(e, old);
                }
                e.mem.set_u32(found, 1);
                read(e, stream, byte_03, 1, found);
                if e.mem.u8(byte_03) == 0 {
                    e.call(STREAM_SET_NUMBER_OF_LINK_IDS, &args![stream, 0u32]);
                } else {
                    e.call(STREAM_SET_NUMBER_OF_LINK_IDS, &args![stream, 1u32]);
                    e.call(STREAM_READ_LINK_ID, &args![stream]);
                }
            }
            release_and_clear(e, base + 0x38);
        } else {
            read(e, stream, external, 1, found);
            if version < VERSION_20_1_0_1 {
                e.call(
                    STREAM_LOAD_C_STRING_AS_FIXED_STRING,
                    &args![stream, name_slot],
                );
            } else {
                e.call(STREAM_LOAD_FIXED_STRING, &args![stream, name_slot]);
            }
            e.call(STREAM_SET_NUMBER_OF_LINK_IDS, &args![stream, 1u32]);
            e.call(STREAM_READ_LINK_ID, &args![stream]);
        }
        if e.mem.u32(name_slot) != 0 {
            e.call(CONVERT_FILENAME, &args![name_slot, base + 0x34]);
        }
        let platform_slot = base + 0x34;
        let mut release_found = true;
        if e.mem.u8(external) != 0 {
            let platform = e.mem.u32(platform_slot);
            e.call(STREAM_00A636F0, &args![stream, platform]);
            e.mem.set_u8(LOOKING_FOR_FILE, 1);
            e.mem.set_u32(found, 0);
            loop {
                let more = e
                    .call(STREAM_NEXT_PATH, &args![stream, path, 0x104u32])
                    .bool();
                if !more {
                    break;
                }
                e.call(STREAM_GET_TEXTURE, &args![stream, path, found]);
                if e.mem.u32(found) != 0 {
                    break;
                }
            }
            if e.mem.u32(found) != 0 {
                let old = e.mem.u32(platform_slot);
                if old != path {
                    let handle = e.call(FIXED_STRING_FROM, &args![path]).u32();
                    e.mem.set_u32(platform_slot, handle);
                    release_fixed_string(e, old);
                }
                let texture = e.mem.u32(found);
                e.vcall(stream, STREAM_FOUND_TEXTURE_SLOT, &args![texture]);
            } else {
                e.mem.set_u8(LOOKING_FOR_FILE, 1);
                let mut more = e
                    .call(STREAM_NEXT_PATH, &args![stream, path, 0x104u32])
                    .bool();
                while more {
                    if e.call(FILE_ACCESS, &args![path, 0u32]).bool() {
                        e.call(FIXED_STRING_ASSIGN, &args![platform_slot, path]);
                        e.call(STREAM_SET_TEXTURE, &args![stream, this]);
                        break;
                    }
                    more = e
                        .call(STREAM_NEXT_PATH, &args![stream, path, 0x104u32])
                        .bool();
                }
            }
        } else {
            let platform = e.mem.u32(platform_slot);
            if platform == e.mem.u32(EMPTY_FIXED_STRING) {
                release_found = false;
            } else {
                e.mem.set_u32(found, 0);
                e.call(STREAM_GET_TEXTURE, &args![stream, platform, found]);
                let texture = e.mem.u32(found);
                if texture == 0 {
                    e.call(STREAM_SET_TEXTURE, &args![stream, this]);
                } else {
                    e.vcall(stream, STREAM_FOUND_TEXTURE_SLOT, &args![texture]);
                }
            }
        }
        if release_found {
            let texture = e.mem.u32(found);
            release(e, texture);
        }
        // The format preferences: three words through the two scratch words.
        e.mem.set_u32(found, 4);
        read(e, stream, word_0c, 4, found);
        let layout = e.mem.u32(word_0c);
        e.set(this, NiSourceTexture::m_ePixelLayout, layout);
        e.mem.set_u32(word_0c, 4);
        read(e, stream, found, 4, word_0c);
        let mipmap = e.mem.u32(found);
        e.set(this, NiSourceTexture::m_eMipMapped, mipmap);
        e.mem.set_u32(word_0c, 4);
        read(e, stream, found, 4, word_0c);
        let alpha = e.mem.u32(found);
        e.set(this, NiSourceTexture::m_eAlphaFmt, alpha);
        let version = e.mem.u32(stream + STREAM_VERSION);
        let old_layout = if version < VERSION_3_3_0_12 {
            Some(4)
        } else if version < VERSION_5_0_0_1 {
            Some(5)
        } else {
            None
        };
        if let Some(old_layout) = old_layout {
            if e.get(this, NiSourceTexture::m_ePixelLayout) == old_layout {
                e.set(this, NiSourceTexture::m_ePixelLayout, 6);
            }
        }
        e.mem.set_u32(word_0c, 1);
        read(e, stream, byte_12, 1, word_0c);
        let value = u8::from(e.mem.u8(byte_12) != 0);
        e.set(this, NiSourceTexture::m_bStatic, value);
        if e.mem.u32(stream + STREAM_VERSION) >= VERSION_10_1_0_103 {
            e.mem.set_u32(word_0c, 1);
            read(e, stream, byte_03, 1, word_0c);
            let value = u8::from(e.mem.u8(byte_03) != 0);
            e.set(this, NiSourceTexture::m_bLoadDirectToRendererHint, value);
        }
        if e.mem.u32(stream + STREAM_VERSION) >= VERSION_20_2_0_4 {
            e.mem.set_u32(word_0c, 1);
            read(e, stream, byte_03, 1, word_0c);
            let value = u8::from(e.mem.u8(byte_03) != 0);
            e.set(this, NiSourceTexture::m_bSrcRendererDataIsPersistent, value);
        }
    });
    release(e, base);
}

/// The base class's `RegisterStreamables` (`00a62130`), `IsEqual` (`00a62140`) and
/// `SaveBinary` (`00a5d7b0`), `__thiscall` with the stream or the other object (the map names
/// them after `NiSequenceStreamHelper`, folded 5-byte bodies).
const BASE_REGISTER_STREAMABLES: u32 = 0x00a6_2130;
const BASE_IS_EQUAL: u32 = 0x00a6_2140;
const BASE_SAVE_BINARY: u32 = 0x00a5_d7b0;
/// `NiStream`'s virtual +0x20 (`RegisterFixedString`'s slot: one word, the fixed string's
/// address) and +0x30 (one word: the object whose link id is saved).
const STREAM_REGISTER_FIXED_STRING_SLOT: u32 = 0x20;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// `NiObject`'s virtual +0x54 (`RegisterStreamables`, one word: the stream) and +0x5c
/// (`IsEqual`, one word: the other object).
const OBJECT_REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const OBJECT_IS_EQUAL_SLOT: u32 = 0x5c;
/// `NiImageConverter::GetImageConverter` (`00a76640`, no words) and the converter's virtual
/// +0xc (three words: the platform file name, the old pixel data, 0) that reads a file's
/// pixel data.
const GET_IMAGE_CONVERTER: u32 = 0x00a7_6640;
const CONVERTER_READ_FILE_SLOT: u32 = 0xc;
/// `00c663d0` (`__thiscall` on a pointer slot, one word: the address of the new pointer): a
/// reference-counted pointer assignment (the map puts it in `BSHavok/bhkworld.obj`).
const POINTER_ASSIGN_FROM: u32 = 0x00c6_63d0;
/// `NiFile::GetFile` (`00aa1a20`, cdecl: name, mode, buffer size).
const FILE_GET_FILE: u32 = 0x00aa_1a20;
/// The texture's virtual +0x9c (no words: the address of its file name) and the renderer
/// data's byte +0x5b (the state `Upgrade` and `Degrade` test) and virtuals +0x8c and +0x90
/// (one word: the texture).
const GET_FILENAME_SLOT: u32 = 0x9c;
const RENDERER_DATA_STATE: u32 = 0x5b;
const RENDERER_DATA_UPGRADE_SLOT: u32 = 0x8c;
const RENDERER_DATA_DEGRADE_SLOT: u32 = 0x90;
/// `NiStream`'s output stream (+0x250) and its write function (+0xc: cdecl, stream, buffer,
/// bytes, component sizes, component count); the bytes written are added to its position
/// (+0x4).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiStream::SaveFixedString` (`00a641f0`, `__thiscall` on the stream, one word: the fixed
/// string's address).
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// `NiTexture::GetViewerStrings` (`00a62a40`, `__thiscall`, the string array),
/// `NiGetViewerString` (`00a7d330`, cdecl: a name) and `NiGetViewerString_ov11` (`00a7d5e0`,
/// cdecl: a label and a fixed string), and the string array's `SetSize` (`0096ad30`,
/// `__thiscall`: the new size) and `SetAt` (`0096ae90`, `__thiscall`: index, address of the
/// element).
const NI_TEXTURE_GET_VIEWER_STRINGS: u32 = 0x00a6_2a40;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FIXED: u32 = 0x00a7_d5e0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"File:"`.
const FILE_LABEL: u32 = 0x0109_ba9c;

/// One write through the output stream: `bytes` from `buffer`, one component of `size`
/// bytes (the size is a word on the stack); the position moves on by what was written.
fn write(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, size: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let function = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        let written = e
            .call(function, &args![output, buffer, bytes, sizes, 1u32])
            .u32();
        let position = e
            .mem
            .u32(output.wrapping_add(BINARY_STREAM_POSITION))
            .wrapping_add(written);
        e.mem
            .set_u32(output.wrapping_add(BINARY_STREAM_POSITION), position);
    });
}

/// One byte written through the output stream from a stack copy.
fn write_byte(e: &mut Engine, stream: u32, value: u8) {
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), value);
        write(e, stream, byte.addr(), 1, 1);
    });
}

/// The inlined `NiTArray<char *>::Add`: grows the array to its size plus `m_usGrowBy` (+0xe)
/// when the size (+0xa) has reached the capacity (+0x8), then stores `value` at the old size.
fn add_viewer_string(e: &mut Engine, strings: u32, value: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    let capacity = u32::from(e.mem.u16(strings.wrapping_add(0x8)));
    if size >= capacity {
        let grow_by = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), value);
        e.call(STRING_ARRAY_SET_AT, &args![strings, size, element]);
    });
}

// Translated from 00a5fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::RegisterStreamables` (Xbox PDB): false when the base class's
/// registration (`00a62130`) is; otherwise registers the file name with the stream (its
/// virtual +0x20) and answers the persistent renderer data's registration (virtual +0x54),
/// or else the pixel data's, or true when the texture has neither.
pub fn ni_source_texture_register_streamables(
    e: &mut Engine,
    this: Ptr<NiSourceTexture>,
    stream: Ptr,
) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    e.vcall(
        stream.addr(),
        STREAM_REGISTER_FIXED_STRING_SLOT,
        &args![this.addr() + 0x30],
    );
    let mut data = e.get(this, NiSourceTexture::m_spPersistentSrcRendererData);
    if data == 0 {
        data = e.get(this, NiSourceTexture::m_spSrcPixelData);
        if data == 0 {
            return true;
        }
    }
    e.vcall(data, OBJECT_REGISTER_STREAMABLES_SLOT, &args![stream])
        .bool()
}

/// One pointer of `IsEqual`: both null, or both set and the first's virtual `IsEqual`
/// (+0x5c) true for the second.
fn source_equal(e: &mut Engine, mine: u32, other: u32) -> bool {
    if mine != 0 {
        if other == 0 {
            return false;
        }
    } else if other != 0 {
        return false;
    }
    mine == 0 || e.vcall(mine, OBJECT_IS_EQUAL_SLOT, &args![other]).bool()
}

// Translated from 00a5fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::IsEqual` (Xbox PDB): the base class's comparison (`00a62140`), the same
/// file name handle, equal pixel data and persistent renderer data (each both null or equal
/// by the first one's virtual `IsEqual`), the same pixel layout, mipmap and alpha
/// preferences and the same static, load-straight and persistent bytes.
pub fn ni_source_texture_is_equal(
    e: &mut Engine,
    this: Ptr<NiSourceTexture>,
    other: Ptr<NiSourceTexture>,
) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiSourceTexture::m_kFilename) != e.get(other, NiSourceTexture::m_kFilename) {
        return false;
    }
    let mine = e.get(this, NiSourceTexture::m_spSrcPixelData);
    let theirs = e.get(other, NiSourceTexture::m_spSrcPixelData);
    if !source_equal(e, mine, theirs) {
        return false;
    }
    let mine = e.get(this, NiSourceTexture::m_spPersistentSrcRendererData);
    let theirs = e.get(other, NiSourceTexture::m_spPersistentSrcRendererData);
    if !source_equal(e, mine, theirs) {
        return false;
    }
    e.get(this, NiSourceTexture::m_ePixelLayout) == e.get(other, NiSourceTexture::m_ePixelLayout)
        && e.get(this, NiSourceTexture::m_eMipMapped) == e.get(other, NiSourceTexture::m_eMipMapped)
        && e.get(this, NiSourceTexture::m_eAlphaFmt) == e.get(other, NiSourceTexture::m_eAlphaFmt)
        && e.get(this, NiSourceTexture::m_bStatic) == e.get(other, NiSourceTexture::m_bStatic)
        && e.get(this, NiSourceTexture::m_bLoadDirectToRendererHint)
            == e.get(other, NiSourceTexture::m_bLoadDirectToRendererHint)
        && e.get(this, NiSourceTexture::m_bSrcRendererDataIsPersistent)
            == e.get(other, NiSourceTexture::m_bSrcRendererDataIsPersistent)
}

// Translated from 00a5ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::LoadPixelDataFromFile` (Xbox PDB): for a texture with a file name (not
/// the empty handle `0109b220`) and neither pixel data nor persistent renderer data, the
/// image converter (`00a76640`) reads the platform file (its virtual +0xc with the platform
/// name, the pixel data and 0). The result, held by a stack reference, becomes the pixel data
/// (`00c663d0`) unless it is already that pointer; the stack reference is then given up.
pub fn ni_source_texture_load_pixel_data_from_file(e: &mut Engine, this: Ptr<NiSourceTexture>) {
    if e.get(this, NiSourceTexture::m_kFilename) == e.mem.u32(EMPTY_FIXED_STRING) {
        return;
    }
    if e.get(this, NiSourceTexture::m_spSrcPixelData) != 0
        || e.get(this, NiSourceTexture::m_spPersistentSrcRendererData) != 0
    {
        return;
    }
    let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
    let pixel_data = e.get(this, NiSourceTexture::m_spSrcPixelData);
    let platform_name = e.get(this, NiSourceTexture::m_kPlatformFilename);
    let data = e
        .vcall(
            converter,
            CONVERTER_READ_FILE_SLOT,
            &args![platform_name, pixel_data, 0u32],
        )
        .u32();
    e.with_stack(4, |e, held| {
        e.mem.set_u32(held.addr(), data);
        if data != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![data + REF_COUNT]);
        }
        let current = e.get(this, NiSourceTexture::m_spSrcPixelData);
        if current == 0 || data != current {
            e.call(POINTER_ASSIGN_FROM, &args![this.addr() + 0x3c, held]);
        }
    });
    release(e, data);
}

// Translated from 00a60090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::Upgrade` (Xbox PDB): when the renderer data's state byte (+0x5b) is 2 or
/// 3, opens the texture's file (its virtual +0x9c gives the name; `NiFile::GetFile` with mode
/// 0 and a 0x8000-byte buffer) into `m_pFile`; when that succeeds the renderer data's virtual
/// +0x8c runs with the texture, the file is deleted (its virtual 0 with 1) and `m_pFile`
/// cleared.
pub fn ni_source_texture_upgrade(e: &mut Engine, this: Ptr<NiSourceTexture>) {
    let data = e.get(this, NiSourceTexture::m_pkRendererData);
    if data == 0 {
        return;
    }
    let state = e.mem.u8(data + RENDERER_DATA_STATE);
    if state != 2 && state != 3 {
        return;
    }
    let name_slot = e.vcall(this.addr(), GET_FILENAME_SLOT, &args![]).u32();
    let name = e.mem.u32(name_slot);
    let file = e.call(FILE_GET_FILE, &args![name, 0u32, 0x8000u32]).u32();
    e.set(this, NiSourceTexture::m_pFile, file);
    if file == 0 {
        return;
    }
    let data = e.get(this, NiSourceTexture::m_pkRendererData);
    e.vcall(data, RENDERER_DATA_UPGRADE_SLOT, &args![this]);
    let file = e.get(this, NiSourceTexture::m_pFile);
    if file != 0 {
        e.vcall(file, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
    }
    e.set(this, NiSourceTexture::m_pFile, 0);
}

// Translated from 00a60100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::Degrade` (Xbox PDB): when the renderer data's state byte (+0x5b) is 4,
/// its virtual +0x90 runs with the texture.
pub fn ni_source_texture_degrade(e: &mut Engine, this: Ptr<NiSourceTexture>) {
    let data = e.get(this, NiSourceTexture::m_pkRendererData);
    if data != 0 && e.mem.u8(data + RENDERER_DATA_STATE) == 4 {
        e.vcall(data, RENDERER_DATA_DEGRADE_SLOT, &args![this]);
    }
}

// Translated from 00a60600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::SaveBinary` (Xbox PDB): the base class's save (`00a5d7b0`), then the
/// "external" byte (1 when there is neither persistent renderer data nor pixel data; the
/// code keeps it in the low byte of its stream argument's stack word), the file name
/// (`SaveFixedString`), the link id of the persistent renderer data when the persistent byte
/// is set or else of the pixel data (the stream's virtual +0x30), the pixel layout, mipmap
/// and alpha words, and the static, load-straight and persistent bytes.
pub fn ni_source_texture_save_binary(e: &mut Engine, this: Ptr<NiSourceTexture>, stream: Ptr) {
    let stream = stream.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let external = e.get(this, NiSourceTexture::m_spPersistentSrcRendererData) == 0
        && e.get(this, NiSourceTexture::m_spSrcPixelData) == 0;
    write_byte(e, stream, u8::from(external));
    e.call(STREAM_SAVE_FIXED_STRING, &args![stream, this.addr() + 0x30]);
    let linked = if e.get(this, NiSourceTexture::m_bSrcRendererDataIsPersistent) != 0 {
        e.get(this, NiSourceTexture::m_spPersistentSrcRendererData)
    } else {
        e.get(this, NiSourceTexture::m_spSrcPixelData)
    };
    e.vcall(stream, STREAM_SAVE_LINK_ID_SLOT, &args![linked]);
    write(e, stream, this.addr() + 0x18, 4, 4);
    write(e, stream, this.addr() + 0x20, 4, 4);
    write(e, stream, this.addr() + 0x1c, 4, 4);
    let value = e.get(this, NiSourceTexture::m_bStatic);
    write_byte(e, stream, value);
    let value = e.get(this, NiSourceTexture::m_bLoadDirectToRendererHint);
    write_byte(e, stream, value);
    let value = e.get(this, NiSourceTexture::m_bSrcRendererDataIsPersistent);
    write_byte(e, stream, value);
}

// Translated from 00a60770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceTexture::GetViewerStrings` (Xbox PDB): `NiTexture`'s strings (`00a62a40`), then
/// the class name (the first word of `ms_RTTI`, through `NiGetViewerString`) and `"File:"`
/// with the platform file name (`NiGetViewerString_ov11`), each appended to the array.
pub fn ni_source_texture_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiSourceTexture>,
    strings: Ptr,
) {
    let strings = strings.addr();
    e.call(NI_TEXTURE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_SOURCE_TEXTURE_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add_viewer_string(e, strings, text);
    let platform_name = e.get(this, NiSourceTexture::m_kPlatformFilename);
    let text = e
        .call(
            NI_GET_VIEWER_STRING_FIXED,
            &args![FILE_LABEL, platform_name],
        )
        .u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a60160, ni_source_texture_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a5f8b0,
            ni_source_texture_create_renderer_data(Ptr<NiSourceTexture>) -> bool
        ),
        entry!(
            0x00a5f940,
            ni_source_texture_get_width(Ptr<NiSourceTexture>) -> u32
        ),
        entry!(
            0x00a5f970,
            ni_source_texture_get_height(Ptr<NiSourceTexture>) -> u32
        ),
        entry!(
            0x00a5f9a0,
            ni_source_texture_link_object(Ptr<NiSourceTexture>, Ptr)
        ),
        entry!(0x00a5fa50, fn_00a5fa50(Ptr<NiSourceTexture>, u32)),
        entry!(
            0x00a5fba0,
            ni_source_texture_set_filename(Ptr<NiSourceTexture>, Ptr)
        ),
        entry!(
            0x00a5fc10,
            ni_source_texture_ni_source_texture(Ptr<NiSourceTexture>) -> Ptr<NiSourceTexture>
        ),
        entry!(0x00a5fc80, ni_source_texture_get_rtti() -> Ptr),
        entry!(
            0x00a5fc90,
            ni_source_texture_get_filename(Ptr<NiSourceTexture>) -> Ptr
        ),
        entry!(0x00a5fca0, fn_00a5fca0(Ptr<NiSourceTexture>)),
        entry!(0x00a5fd70, ni_source_texture_create(Ptr, Ptr, u32, u32) -> Ptr<NiSourceTexture>),
        entry!(0x00a5ff10, ni_source_texture_create_ov2(Ptr, Ptr, Ptr) -> Ptr<NiSourceTexture>),
        entry!(0x00a60070, fn_00a60070() -> Ptr<NiSourceTexture>),
        entry!(
            0x00a60180,
            ni_source_texture_load_binary(Ptr<NiSourceTexture>, Ptr)
        ),
        entry!(
            0x00a5fa90,
            ni_source_texture_register_streamables(Ptr<NiSourceTexture>, Ptr) -> bool
        ),
        entry!(
            0x00a5fae0,
            ni_source_texture_is_equal(Ptr<NiSourceTexture>, Ptr<NiSourceTexture>) -> bool
        ),
        entry!(
            0x00a5ffe0,
            ni_source_texture_load_pixel_data_from_file(Ptr<NiSourceTexture>)
        ),
        entry!(0x00a60090, ni_source_texture_upgrade(Ptr<NiSourceTexture>)),
        entry!(0x00a60100, ni_source_texture_degrade(Ptr<NiSourceTexture>)),
        entry!(
            0x00a60600,
            ni_source_texture_save_binary(Ptr<NiSourceTexture>, Ptr)
        ),
        entry!(
            0x00a60770,
            ni_source_texture_get_viewer_strings(Ptr<NiSourceTexture>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_source_texture_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_fca0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a60160, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a60160, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x48]]);
        assert_eq!(calls_to(&e, 0x00a5_fca0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    use std::cell::RefCell;
    use std::collections::VecDeque;
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

    const DELETE_DOUBLE: u32 = 0x0070_7000;
    const DESTROY_DOUBLE: u32 = 0x0070_7010;
    const LOAD_PIXELS_DOUBLE: u32 = 0x0070_7020;
    const AFTER_CREATE_DOUBLE: u32 = 0x0070_7030;
    const RENDERER_DOUBLE: u32 = 0x0070_7040;
    const READ_DOUBLE: u32 = 0x0070_7050;
    const FOUND_DOUBLE: u32 = 0x0070_7060;

    /// Doubles for the callees outside the unit; the empty fixed string is handle 0.
    fn doubles(e: &mut Engine) {
        e.map(EMPTY_FIXED_STRING, 4);
        e.map(RENDERER_INSTANCE, 4);
        e.map(CALL_AFTER_CREATE, 4);
        e.map(LOAD_AT_ONCE, 4);
        e.map(LOOKING_FOR_FILE, 4);
        for addr in [
            NI_OBJECT_NET_CONSTRUCTOR,
            NI_OBJECT_NET_DESTRUCTOR,
            NI_TEXTURE_DESTRUCTOR,
            TEXTURE_LIST_ADD,
            STANDARDIZE_FILE_PATH,
            CONVERT_FILENAME,
            DELETE_DOUBLE,
            DESTROY_DOUBLE,
            AFTER_CREATE_DOUBLE,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            BASE_LOAD_BINARY,
            BASE_LINK_OBJECT,
            STREAM_LOAD_C_STRING_AS_FIXED_STRING,
            STREAM_LOAD_FIXED_STRING,
            STREAM_SET_NUMBER_OF_LINK_IDS,
            STREAM_READ_LINK_ID,
            STREAM_00A636F0,
            STREAM_SET_TEXTURE,
            FIXED_STRING_ASSIGN,
            FOUND_DOUBLE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // A fixed string: a block whose count is at the handle - 8.
        e.register(FIXED_STRING_FROM, |e, a| {
            if a[0] == 0 {
                return word(0);
            }
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, 1);
            word(block + 8)
        });
    }

    /// A reference-counted object (count `count`) whose `DeleteThis` and deleting destructor
    /// are doubles.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable, DESTROY_DOUBLE);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(object + REF_COUNT, count);
        object
    }

    /// A texture built by the constructor, with a test virtual table holding the pixel load,
    /// the call after creation and the deleting destructor.
    fn texture(e: &mut Engine, loads: bool) -> u32 {
        let t = e.mem.alloc(0x48);
        ni_source_texture_ni_source_texture(e, Ptr::new(t));
        give_table(e, t, loads);
        t
    }

    fn give_table(e: &mut Engine, t: u32, loads: bool) {
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable, DESTROY_DOUBLE);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem
            .set_u32(vtable + AFTER_CREATE_SLOT, AFTER_CREATE_DOUBLE);
        e.mem.set_u32(vtable + LOAD_PIXELS_SLOT, LOAD_PIXELS_DOUBLE);
        e.register_double(LOAD_PIXELS_DOUBLE, move |_, _| word(u32::from(loads)));
        e.mem.set_u32(t, vtable);
    }

    #[test]
    fn constructor_getters_and_factory() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = e.mem.alloc(0x48);
        for off in (0..0x48).step_by(4) {
            e.mem.set_u32(t + off, 0x1111_1111);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5fc10, &args![t]).u32(), t);
        assert_eq!(calls_to(&e, TEXTURE_LIST_ADD), vec![vec![t]]);
        assert_eq!(e.mem.u32(t), NI_SOURCE_TEXTURE_VTABLE);
        let words: Vec<u32> = (0x18..0x48).step_by(4).map(|o| e.mem.u32(t + o)).collect();
        assert_eq!(
            words,
            vec![6, 3, 2, 0, 0x1111_1111, 0x1111_1111, 0, 0, 0, 0, 0x0101, 0]
        );
        assert_eq!(e.call(0x00a5fc80, &args![]).u32(), NI_SOURCE_TEXTURE_RTTI);
        assert_eq!(e.call(0x00a5fc90, &args![t]).u32(), t + 0x30);
        assert_eq!(e.call(0x00a5f940, &args![t]).u32(), 0);
        assert_eq!(e.call(0x00a5f970, &args![t]).u32(), 0);
        let pixels = e.mem.alloc(0x74);
        let sizes = e.mem.alloc(8);
        e.mem.set_u32(sizes, 64);
        e.mem.set_u32(sizes + 4, 32);
        e.mem.set_u32(pixels + 0x54, sizes);
        e.mem.set_u32(pixels + 0x58, sizes + 4);
        e.mem.set_u32(t + 0x3c, pixels);
        assert_eq!(e.call(0x00a5f940, &args![t]).u32(), 64);
        assert_eq!(e.call(0x00a5f970, &args![t]).u32(), 32);
        let persistent = e.mem.alloc(0x94);
        e.mem.set_u32(persistent + 0x64, sizes + 4);
        e.mem.set_u32(persistent + 0x68, sizes);
        e.mem.set_u32(t + 0x38, persistent);
        assert_eq!(e.call(0x00a5f940, &args![t]).u32(), 32);
        assert_eq!(e.call(0x00a5f970, &args![t]).u32(), 64);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 0xc, 7);
        e.mem.set_u32(data + 0x10, 9);
        e.mem.set_u32(t + 0x24, data);
        assert_eq!(e.call(0x00a5f940, &args![t]).u32(), 7);
        assert_eq!(e.call(0x00a5f970, &args![t]).u32(), 9);
        let fresh = e.mem.alloc(0x48);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        assert_eq!(e.call(0x00a60070, &args![]).u32(), fresh);
        assert_eq!(e.mem.u32(fresh), NI_SOURCE_TEXTURE_VTABLE);
        e.register(NEW_OBJECT, |_, _| word(0));
        assert_eq!(e.call(0x00a60070, &args![]).u32(), 0);
    }

    #[test]
    fn create_renderer_data_paths() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let file = counted(&mut e, 1);
        e.mem.set_u32(t + 0x44, file);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5f8b0, &args![t]).bool());
        assert_eq!(calls_to(&e, DESTROY_DOUBLE), vec![vec![file, 1]]);
        assert_eq!(e.mem.u32(t + 0x44), 0);
        let renderer = counted(&mut e, 1);
        let vtable = e.mem.u32(renderer);
        e.mem.set_u32(
            vtable + RENDERER_CREATE_SOURCE_TEXTURE_DATA_SLOT,
            RENDERER_DOUBLE,
        );
        e.mem.set_u32(RENDERER_INSTANCE, renderer);
        // The renderer fails: false, the file stays.
        e.register(RENDERER_DOUBLE, |_, _| word(0));
        e.mem.set_u32(t + 0x44, file);
        assert!(!e.call(0x00a5f8b0, &args![t]).bool());
        assert_eq!(e.mem.u32(t + 0x44), file);
        // It succeeds and makes data: the call after creation runs, the file is forgotten.
        e.register(RENDERER_DOUBLE, |e, a| {
            e.mem.set_u32(a[1] + 0x24, 0xda7a);
            word(1)
        });
        e.mem.set_u8(CALL_AFTER_CREATE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a5f8b0, &args![t]).bool());
        assert_eq!(calls_to(&e, AFTER_CREATE_DOUBLE), vec![vec![t]]);
        assert_eq!(e.mem.u32(t + 0x44), 0);
        assert!(calls_to(&e, DESTROY_DOUBLE).is_empty());
        // Data already there: the file is deleted.
        e.mem.set_u32(t + 0x44, file);
        assert!(e.call(0x00a5f8b0, &args![t]).bool());
        assert_eq!(calls_to(&e, DESTROY_DOUBLE), vec![vec![file, 1]]);
        assert!(e.call(0x00a5f8b0, &args![t]).bool());
    }

    #[test]
    fn link_object_takes_pixel_or_persistent_data() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let stream = e.mem.alloc(0x250);
        let linked = counted(&mut e, 1);
        e.register_double(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(1));
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| word(linked));
        let old = counted(&mut e, 1);
        e.mem.set_u32(t + 0x38, old);
        e.call(0x00a5f9a0, &args![t, stream]);
        assert_eq!((e.mem.u32(t + 0x3c), e.mem.u32(t + 0x38)), (linked, 0));
        // Persistent from 20.2.0.4 on.
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_20_2_0_4);
        e.mem.set_u8(t + 0x42, 1);
        e.call(0x00a5f9a0, &args![t, stream]);
        assert_eq!((e.mem.u32(t + 0x38), e.mem.u32(t + 0x3c)), (linked, 0));
        // No link ids: nothing changes.
        e.register_double(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x00a5f9a0, &args![t, stream]);
        assert!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).is_empty());
        assert_eq!(calls_to(&e, BASE_LINK_OBJECT), vec![vec![t, stream]]);
    }

    #[test]
    fn fn_00a5fa50_loads_in_the_critical_section() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x00a5fa50, &args![t, 0u32]);
        assert!(calls_to(&e, LOAD_PIXELS_DOUBLE).is_empty());
        e.mem.set_u8(LOAD_AT_ONCE, 1);
        e.call(0x00a5fa50, &args![t, 0u32]);
        assert_eq!(calls_to(&e, LOAD_PIXELS_DOUBLE), vec![vec![t]]);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![LOAD_CRITICAL_SECTION]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![LOAD_CRITICAL_SECTION]]
        );
    }

    #[test]
    fn file_names_and_create() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let name = e.call(FIXED_STRING_FROM, &args![0x500u32]).u32();
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, name);
        e.call_log = Some(vec![]);
        e.call(0x00a5fba0, &args![t, holder]);
        assert_eq!(e.mem.u32(t + 0x30), name);
        assert_eq!(e.mem.u32(name - 8), 2);
        assert_eq!(calls_to(&e, STANDARDIZE_FILE_PATH), vec![vec![t + 0x30]]);
        assert_eq!(
            calls_to(&e, CONVERT_FILENAME),
            vec![vec![t + 0x30, t + 0x34]]
        );
        // Another name: the old one gives up its reference.
        let other = e.call(FIXED_STRING_FROM, &args![0x600u32]).u32();
        e.mem.set_u32(holder, other);
        e.call(0x00a5fba0, &args![t, holder]);
        assert_eq!((e.mem.u32(name - 8), e.mem.u32(other - 8)), (1, 2));
        // Create: preferences, flags and the name.
        let fresh = e.mem.alloc(0x48);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        let prefs = e.mem.alloc(0xc);
        e.mem.set_u32(prefs, 1);
        e.mem.set_u32(prefs + 4, 2);
        e.mem.set_u32(prefs + 8, 3);
        let created = e
            .call(0x00a5fd70, &args![holder, prefs, 0x100u32, 1u32])
            .u32();
        assert_eq!(created, fresh);
        assert_eq!(
            (
                e.mem.u32(fresh + 0x18),
                e.mem.u32(fresh + 0x1c),
                e.mem.u32(fresh + 0x20)
            ),
            (1, 2, 3)
        );
        assert_eq!((e.mem.u8(fresh + 0x41), e.mem.u8(fresh + 0x42)), (0, 1));
        assert_eq!(e.mem.u32(fresh + 0x30), other);
        // Loading at once with a failing load: deleted, null.
        e.mem.set_u8(LOAD_AT_ONCE, 1);
        let fresh = e.mem.alloc(0x48);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        e.register_double(TEXTURE_LIST_ADD, |_, _| Ret::default());
        e.register_double(STANDARDIZE_FILE_PATH, move |e, _| {
            // The constructor's table is the exe's: give the texture the test one here.
            give_table(e, fresh, false);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a5fd70, &args![holder, prefs, 1u32, 0u32]).u32(),
            0
        );
        assert_eq!(calls_to(&e, DESTROY_DOUBLE), vec![vec![fresh, 1]]);
    }

    #[test]
    fn create_from_pixel_data() {
        let mut e = Engine::new();
        doubles(&mut e);
        let fresh = e.mem.alloc(0x48);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        let pixels = counted(&mut e, 0);
        let prefs = e.mem.alloc(0xc);
        e.mem.set_u32(prefs, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5ff10, &args![pixels, 0u32, prefs]).u32(), fresh);
        assert_eq!(
            (e.mem.u32(fresh + 0x3c), e.mem.u32(pixels + 4)),
            (pixels, 1)
        );
        assert!(calls_to(&e, FIXED_STRING_FROM).is_empty());
        let fresh = e.mem.alloc(0x48);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        assert_eq!(
            e.call(0x00a5ff10, &args![pixels, 0x700u32, prefs]).u32(),
            fresh
        );
        let name = e.mem.u32(fresh + 0x30);
        assert_ne!(name, 0);
        // The temporary's reference is given up: only the texture's is left.
        assert_eq!(e.mem.u32(name - 8), 1);
        assert_eq!(e.mem.u32(pixels + 4), 2);
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let pixels = counted(&mut e, 1);
        let persistent = counted(&mut e, 2);
        let data = counted(&mut e, 1);
        e.mem.set_u32(t + 0x3c, pixels);
        e.mem.set_u32(t + 0x38, persistent);
        e.mem.set_u32(t + 0x24, data);
        let name = e.call(FIXED_STRING_FROM, &args![0x500u32]).u32();
        e.mem.set_u32(t + 0x30, name);
        e.call_log = Some(vec![]);
        e.call(0x00a5fca0, &args![t]);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![pixels]]);
        assert_eq!(e.mem.u32(persistent + 4), 1);
        assert_eq!((e.mem.u32(t + 0x38), e.mem.u32(t + 0x3c)), (0, 0));
        assert_eq!(e.mem.u32(name - 8), 0);
        assert_eq!(e.mem.u32(t), NI_TEXTURE_VTABLE);
        assert_eq!(calls_to(&e, DESTROY_DOUBLE), vec![vec![data, 1]]);
        assert_eq!(calls_to(&e, NI_TEXTURE_DESTRUCTOR), vec![vec![t]]);
        assert_eq!(calls_to(&e, NI_OBJECT_NET_DESTRUCTOR), vec![vec![t]]);
    }

    type Reads = Rc<RefCell<VecDeque<u32>>>;

    /// A stream of `version` whose input reads the queued values (each read takes one value
    /// and writes its low `bytes` bytes).
    fn stream(e: &mut Engine, version: u32, values: &[u32]) -> (u32, u32, Reads) {
        let stream = counted(e, 1);
        let vtable = e.mem.u32(stream);
        e.mem
            .set_u32(vtable + STREAM_FOUND_TEXTURE_SLOT, FOUND_DOUBLE);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let queue: Reads = Rc::new(RefCell::new(values.iter().copied().collect()));
        let reads = queue.clone();
        e.register_double(READ_DOUBLE, move |e, a| {
            let value = reads.borrow_mut().pop_front().expect("a value to read");
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, (value >> (8 * i)) as u8);
            }
            word(a[2])
        });
        (stream, input, queue)
    }

    #[test]
    fn load_binary_of_an_old_internal_texture() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let persistent = counted(&mut e, 1);
        e.mem.set_u32(t + 0x38, persistent);
        // External 0, link byte 1, layout 4, mipmap 1, alpha 2, static 0.
        let (s, input, queue) = stream(&mut e, 0x0303_0000, &[0, 1, 4, 1, 2, 0]);
        e.call_log = Some(vec![]);
        e.call(0x00a60180, &args![t, s]);
        assert!(queue.borrow().is_empty());
        assert_eq!(
            calls_to(&e, STREAM_SET_NUMBER_OF_LINK_IDS),
            vec![vec![s, 1]]
        );
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID), vec![vec![s]]);
        assert_eq!(e.mem.u32(t + 0x38), 0);
        // Layout 4 before 3.3.0.12 becomes 6.
        assert_eq!(
            (
                e.mem.u32(t + 0x18),
                e.mem.u32(t + 0x1c),
                e.mem.u32(t + 0x20)
            ),
            (6, 2, 1)
        );
        assert_eq!(e.mem.u8(t + 0x40), 0);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 1 + 1 + 12 + 1);
        // The empty platform name: no texture search.
        assert!(calls_to(&e, STREAM_SET_TEXTURE).is_empty());
        assert_eq!(e.mem.u32(t + 4), 0);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY), vec![vec![t, s]]);
    }

    #[test]
    fn load_binary_of_an_old_external_texture_found_on_disk() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let pixels = counted(&mut e, 1);
        e.mem.set_u32(t + 0x3c, pixels);
        // External 1, then layout 5, mipmap, alpha, static 1, load-straight 0.
        let (s, _, queue) = stream(&mut e, 0x0400_0000, &[1, 5, 0, 0, 1]);
        // Two candidates for the stream's textures, then none; then candidates for the disk.
        let mut asked = 0;
        e.register_double(STREAM_NEXT_PATH, move |_, _| {
            asked += 1;
            word(u32::from(asked != 3))
        });
        e.register(STREAM_GET_TEXTURE, |_, _| Ret::default());
        let mut checks = 0;
        e.register_double(FILE_ACCESS, move |_, _| {
            checks += 1;
            word(u32::from(checks == 1))
        });
        e.call_log = Some(vec![]);
        e.call(0x00a60180, &args![t, s]);
        assert!(queue.borrow().is_empty());
        assert_eq!(e.mem.u32(t + 0x3c), 0);
        assert_eq!(
            calls_to(&e, STREAM_SET_NUMBER_OF_LINK_IDS),
            vec![vec![s, 0]]
        );
        assert_eq!(e.mem.u8(LOOKING_FOR_FILE), 1);
        assert_eq!(calls_to(&e, STREAM_GET_TEXTURE).len(), 2);
        assert_eq!(calls_to(&e, FIXED_STRING_ASSIGN).len(), 1);
        assert_eq!(calls_to(&e, STREAM_SET_TEXTURE), vec![vec![s, t]]);
        // Layout 5 before 5.0.0.1 becomes 6; the static byte reads true.
        assert_eq!(e.mem.u32(t + 0x18), 6);
        assert_eq!(e.mem.u8(t + 0x40), 1);
    }

    #[test]
    fn load_binary_of_a_new_texture_finds_it_in_the_stream() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        // External 0, layout, mipmap, alpha, static, load-straight 1, persistent 1.
        let (s, _, queue) = stream(&mut e, VERSION_20_2_0_4, &[0, 3, 2, 1, 1, 1, 1]);
        let platform = e.call(FIXED_STRING_FROM, &args![0x900u32]).u32();
        e.register(CONVERT_FILENAME, |e, a| {
            let handle = e.mem.u32(0x0070_7100);
            e.mem.set_u32(a[1], handle);
            Ret::default()
        });
        e.map(0x0070_7100, 4);
        e.mem.set_u32(0x0070_7100, platform);
        e.register(STREAM_LOAD_FIXED_STRING, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            Ret::default()
        });
        let found = counted(&mut e, 2);
        e.register_double(STREAM_GET_TEXTURE, move |e, a| {
            e.mem.set_u32(a[2], found);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a60180, &args![t, s]);
        assert!(queue.borrow().is_empty());
        assert_eq!(calls_to(&e, STREAM_GET_TEXTURE)[0][1], platform);
        assert_eq!(calls_to(&e, FOUND_DOUBLE), vec![vec![s, found]]);
        assert_eq!(e.mem.u32(found + 4), 1);
        assert!(calls_to(&e, STREAM_SET_TEXTURE).is_empty());
        assert_eq!(
            calls_to(&e, STREAM_SET_NUMBER_OF_LINK_IDS),
            vec![vec![s, 1]]
        );
        assert_eq!(
            (
                e.mem.u32(t + 0x18),
                e.mem.u32(t + 0x1c),
                e.mem.u32(t + 0x20)
            ),
            (3, 1, 2)
        );
        assert_eq!(
            (e.mem.u8(t + 0x40), e.mem.u8(t + 0x41), e.mem.u8(t + 0x42)),
            (1, 1, 1)
        );
    }

    #[test]
    fn load_binary_of_a_new_external_texture_found_by_path() {
        let mut e = Engine::new();
        doubles(&mut e);
        let t = texture(&mut e, true);
        let (s, _, _) = stream(&mut e, VERSION_10_1_0_103, &[1, 0, 0, 0, 0, 0]);
        e.register_double(STREAM_NEXT_PATH, |_, _| word(1));
        let found = counted(&mut e, 2);
        let mut tries = 0;
        e.register_double(STREAM_GET_TEXTURE, move |e, a| {
            tries += 1;
            if tries == 2 {
                e.mem.set_u32(a[2], found);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a60180, &args![t, s]);
        // Loaded as a C string before 20.1.0.1.
        assert_eq!(
            calls_to(&e, STREAM_LOAD_C_STRING_AS_FIXED_STRING),
            vec![vec![s, t + 0x30]]
        );
        assert_eq!(calls_to(&e, STREAM_GET_TEXTURE).len(), 2);
        let path = calls_to(&e, STREAM_GET_TEXTURE)[0][1];
        assert_eq!(calls_to(&e, FIXED_STRING_FROM), vec![vec![path]]);
        assert_ne!(e.mem.u32(t + 0x34), 0);
        assert_eq!(calls_to(&e, FOUND_DOUBLE), vec![vec![s, found]]);
        assert_eq!(e.mem.u32(found + 4), 1);
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

    /// An object whose virtual table has `slots` (byte offset, target).
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        object
    }

    const DATA_DOUBLE: u32 = 0x0070_8000;
    const STREAM_DOUBLE: u32 = 0x0070_8010;
    const WRITE_DOUBLE: u32 = 0x0070_8020;
    const CONVERTER_DOUBLE: u32 = 0x0070_8030;
    const DELETE_DOUBLE: u32 = 0x0070_8040;

    type Writes = Rc<RefCell<Vec<(Vec<u8>, u32, u32)>>>;

    /// A stream whose output records each write (the bytes, the byte count, the size word)
    /// and answers the byte count.
    fn stream(e: &mut Engine) -> (u32, Writes) {
        let stream = object(e, 0x260, &[(0x20, STREAM_DOUBLE), (0x30, STREAM_DOUBLE)]);
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE_DOUBLE);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let writes: Writes = Rc::new(RefCell::new(vec![]));
        let log = writes.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            log.borrow_mut().push((bytes, a[2], e.mem.u32(a[3])));
            word(a[2])
        });
        e.register(STREAM_DOUBLE, |_, _| Ret::default());
        (stream, writes)
    }

    #[test]
    fn register_streamables_asks_the_base_then_the_source() {
        let mut e = Engine::new();
        e.register(BASE_REGISTER_STREAMABLES, |_, a| {
            word(u32::from(a[1] != 0x99))
        });
        e.register(DATA_DOUBLE, |_, a| word(0x100 | (a[1] & 0xff)));
        let (s, _) = stream(&mut e);
        let t = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5_fa90, &args![t, 0x99u32]).bool());
        assert_eq!(calls_to(&e, STREAM_DOUBLE).len(), 0);
        // Neither source: true after the file name is registered.
        assert!(e.call(0x00a5_fa90, &args![t, s]).bool());
        assert_eq!(calls_to(&e, STREAM_DOUBLE), vec![vec![s, t + 0x30]]);
        // The pixel data alone answers its registration.
        let pixels = object(&mut e, 0x10, &[(0x54, DATA_DOUBLE)]);
        e.mem.set_u32(t + 0x3c, pixels);
        assert_eq!(e.call(0x00a5_fa90, &args![t, s]).u32() & 0xff, s & 0xff);
        // The persistent data comes first.
        let persistent = object(&mut e, 0x10, &[(0x54, DATA_DOUBLE)]);
        e.mem.set_u32(t + 0x38, persistent);
        e.call(0x00a5_fa90, &args![t, s]);
        let asked = calls_to(&e, DATA_DOUBLE);
        assert_eq!(asked, vec![vec![pixels, s], vec![persistent, s]]);
    }

    #[test]
    fn is_equal_compares_sources_and_preferences() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |_, _| word(1));
        e.register(DATA_DOUBLE, |e, a| {
            word(u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[1] + 8)))
        });
        let a = e.mem.alloc(0x48);
        let b = e.mem.alloc(0x48);
        for t in [a, b] {
            e.mem.set_u32(t + 0x30, 0x1234);
            e.mem.set_u32(t + 0x18, 2);
            e.mem.set_u32(t + 0x1c, 1);
            e.mem.set_u32(t + 0x20, 3);
            e.mem.set_u8(t + 0x40, 1);
        }
        assert!(e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u8(b + 0x42, 1);
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u8(b + 0x42, 0);
        e.mem.set_u32(b + 0x1c, 4);
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u32(b + 0x1c, 1);
        // Pixel data on one side only.
        let pixels_a = object(&mut e, 0x10, &[(0x5c, DATA_DOUBLE)]);
        e.mem.set_u32(a + 0x3c, pixels_a);
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
        let pixels_b = object(&mut e, 0x10, &[(0x5c, DATA_DOUBLE)]);
        e.mem.set_u32(b + 0x3c, pixels_b);
        assert!(e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u32(pixels_b + 8, 5);
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u32(pixels_b + 8, 0);
        // Persistent data on the other side only.
        e.mem.set_u32(b + 0x38, pixels_b);
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u32(b + 0x38, 0);
        e.mem.set_u32(b + 0x30, 0x5678);
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
        e.mem.set_u32(b + 0x30, 0x1234);
        e.register(BASE_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_fae0, &args![a, b]).bool());
    }

    fn reference_counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
    }

    #[test]
    fn load_pixel_data_from_file_reads_through_the_converter() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0e0e);
        reference_counting(&mut e);
        let converter = object(&mut e, 0x10, &[(0xc, CONVERTER_DOUBLE)]);
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| word(converter));
        let pixels = object(&mut e, 0x10, &[(0x4, DELETE_DOUBLE)]);
        e.register_double(CONVERTER_DOUBLE, move |_, _| word(pixels));
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        // The assignment takes its reference as `00c663d0` does.
        e.register(POINTER_ASSIGN_FROM, |e, a| {
            let new = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], new);
            e.mem.set_u32(new + 4, e.mem.u32(new + 4) + 1);
            Ret::default()
        });
        let t = e.mem.alloc(0x48);
        e.mem.set_u32(t + 0x30, 0x0e0e);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ffe0, &args![t]);
        assert_eq!(calls_to(&e, GET_IMAGE_CONVERTER).len(), 0);
        e.mem.set_u32(t + 0x30, 0x2000);
        e.mem.set_u32(t + 0x34, 0x3000);
        e.mem.set_u32(t + 0x38, 0x10);
        e.call(0x00a5_ffe0, &args![t]);
        assert_eq!(calls_to(&e, GET_IMAGE_CONVERTER).len(), 0);
        e.mem.set_u32(t + 0x38, 0);
        e.call(0x00a5_ffe0, &args![t]);
        assert_eq!(
            calls_to(&e, CONVERTER_DOUBLE),
            vec![vec![converter, 0x3000, 0, 0]]
        );
        assert_eq!(calls_to(&e, POINTER_ASSIGN_FROM)[0][0], t + 0x3c);
        assert_eq!(e.mem.u32(t + 0x3c), pixels);
        assert_eq!(e.mem.u32(pixels + 4), 1);
        assert_eq!(calls_to(&e, DELETE_DOUBLE).len(), 0);
        // With pixel data already there nothing more is read.
        e.call(0x00a5_ffe0, &args![t]);
        assert_eq!(calls_to(&e, CONVERTER_DOUBLE).len(), 1);
        // A converter answering null still goes through the assignment, and gives nothing up.
        e.mem.set_u32(t + 0x3c, 0);
        e.register(CONVERTER_DOUBLE, |_, _| word(0));
        e.register(POINTER_ASSIGN_FROM, |_, _| Ret::default());
        e.call(0x00a5_ffe0, &args![t]);
        assert_eq!(calls_to(&e, POINTER_ASSIGN_FROM).len(), 2);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn load_pixel_data_from_file_deletes_data_nobody_kept() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING, 4);
        reference_counting(&mut e);
        let converter = object(&mut e, 0x10, &[(0xc, CONVERTER_DOUBLE)]);
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| word(converter));
        let pixels = object(&mut e, 0x10, &[(0x4, DELETE_DOUBLE)]);
        e.register_double(CONVERTER_DOUBLE, move |_, _| word(pixels));
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN_FROM, |_, _| Ret::default());
        let t = e.mem.alloc(0x48);
        e.mem.set_u32(t + 0x30, 0x2000);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ffe0, &args![t]);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![pixels]]);
    }

    #[test]
    fn upgrade_and_degrade_test_the_renderer_data_state() {
        let mut e = Engine::new();
        const NAME_DOUBLE: u32 = 0x0070_8050;
        let data = object(&mut e, 0x60, &[(0x8c, DATA_DOUBLE), (0x90, DATA_DOUBLE)]);
        e.register(DATA_DOUBLE, |_, _| Ret::default());
        let t = object(&mut e, 0x48, &[(0x9c, NAME_DOUBLE)]);
        let name_slot = e.mem.alloc(4);
        e.mem.set_u32(name_slot, 0x4444);
        e.register_double(NAME_DOUBLE, move |_, _| word(name_slot));
        let file = object(&mut e, 0x10, &[(0, DELETE_DOUBLE)]);
        e.register_double(FILE_GET_FILE, move |_, _| word(file));
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // No renderer data.
        e.call(0x00a6_0090, &args![t]);
        e.call(0x00a6_0100, &args![t]);
        assert_eq!(calls_to(&e, NAME_DOUBLE).len(), 0);
        e.mem.set_u32(t + 0x24, data);
        e.mem.set_u8(data + 0x5b, 1);
        e.call(0x00a6_0090, &args![t]);
        assert_eq!(calls_to(&e, FILE_GET_FILE).len(), 0);
        for state in [2u8, 3] {
            e.mem.set_u8(data + 0x5b, state);
            e.mem.set_u32(t + 0x44, 0x7777);
            e.call(0x00a6_0090, &args![t]);
            assert_eq!(e.mem.u32(t + 0x44), 0);
        }
        assert_eq!(calls_to(&e, FILE_GET_FILE)[0], vec![0x4444, 0, 0x8000]);
        assert_eq!(
            calls_to(&e, DATA_DOUBLE),
            vec![vec![data, t], vec![data, t]]
        );
        assert_eq!(
            calls_to(&e, DELETE_DOUBLE),
            vec![vec![file, 1], vec![file, 1]]
        );
        // A file that does not open: the null stays and nothing upgrades.
        e.register(FILE_GET_FILE, |_, _| word(0));
        e.call(0x00a6_0090, &args![t]);
        assert_eq!(calls_to(&e, DATA_DOUBLE).len(), 2);
        // Degrade runs on state 4 only.
        e.call(0x00a6_0100, &args![t]);
        assert_eq!(calls_to(&e, DATA_DOUBLE).len(), 2);
        e.mem.set_u8(data + 0x5b, 4);
        e.call(0x00a6_0100, &args![t]);
        assert_eq!(calls_to(&e, DATA_DOUBLE)[2], vec![data, t]);
        e.call(0x00a6_0090, &args![t]);
        assert_eq!(calls_to(&e, FILE_GET_FILE).len(), 3);
    }

    #[test]
    fn save_binary_writes_the_fields_in_order() {
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_FIXED_STRING, |_, _| Ret::default());
        let (s, writes) = stream(&mut e);
        let t = e.mem.alloc(0x48);
        e.mem.set_u32(t + 0x18, 0x11);
        e.mem.set_u32(t + 0x1c, 0x22);
        e.mem.set_u32(t + 0x20, 0x33);
        e.mem.set_u8(t + 0x40, 1);
        e.mem.set_u8(t + 0x41, 0);
        e.mem.set_u8(t + 0x42, 1);
        e.mem.set_u32(t + 0x38, 0x5000);
        e.mem.set_u32(t + 0x3c, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0600, &args![t, s]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![t, s]]);
        assert_eq!(
            calls_to(&e, STREAM_SAVE_FIXED_STRING),
            vec![vec![s, t + 0x30]]
        );
        assert_eq!(calls_to(&e, STREAM_DOUBLE), vec![vec![s, 0x5000]]);
        let expected: Vec<(Vec<u8>, u32, u32)> = vec![
            (vec![0], 1, 1),
            (vec![0x11, 0, 0, 0], 4, 4),
            (vec![0x33, 0, 0, 0], 4, 4),
            (vec![0x22, 0, 0, 0], 4, 4),
            (vec![1], 1, 1),
            (vec![0], 1, 1),
            (vec![1], 1, 1),
        ];
        assert_eq!(*writes.borrow(), expected);
        let output = e.mem.u32(s + STREAM_OUTPUT);
        assert_eq!(e.mem.u32(output + 4), 16);
        // External (no sources, not persistent): byte 1 and the pixel data's link id.
        e.mem.set_u32(t + 0x38, 0);
        e.mem.set_u32(t + 0x3c, 0);
        e.mem.set_u8(t + 0x42, 0);
        writes.borrow_mut().clear();
        e.call(0x00a6_0600, &args![t, s]);
        assert_eq!(writes.borrow()[0], (vec![1], 1, 1));
        assert_eq!(calls_to(&e, STREAM_DOUBLE)[1], vec![s, 0]);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_and_file() {
        let mut e = Engine::new();
        e.map(NI_SOURCE_TEXTURE_RTTI, 8);
        e.mem.set_u32(NI_SOURCE_TEXTURE_RTTI, 0x0123_4567);
        e.register(NI_TEXTURE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| word(a[0] + 1));
        e.register(NI_GET_VIEWER_STRING_FIXED, |_, a| word(a[1] + 2));
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 3);
        let t = e.mem.alloc(0x48);
        e.mem.set_u32(t + 0x34, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0770, &args![t, strings]);
        assert_eq!(
            calls_to(&e, NI_TEXTURE_GET_VIEWER_STRINGS),
            vec![vec![t, strings]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![FILE_LABEL, 0x7000]]
        );
        assert_eq!(e.mem.u32(base), 0x0123_4568);
        assert_eq!(e.mem.u32(base + 4), 0x7002);
        // Full after the first: grown to 1 + 3.
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 4]]);
    }
}
