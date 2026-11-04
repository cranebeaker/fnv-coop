//! `(unplaced)/run_00e673f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e67fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: clears one word in each of two arrays of
/// records inside the object. For each of the `[this + 0x10]` records of
/// 0x50 bytes starting at `this + 0xfb4`, the first word is set to 0; then,
/// for each of the `[this + 8]` records of 0x40 bytes starting at
/// `this + 0x50`, the first word is set to 0. The counts are read again
/// after every record, as the code does.
pub fn fn_00e67fd0(e: &mut Engine, this: Ptr) {
    let mut index = 0i32;
    if e.mem.i32(this.addr() + 0x10) > 0 {
        let mut record = this.addr() + 0xfb4;
        loop {
            e.mem.set_u32(record, 0);
            index += 1;
            record += 0x50;
            if index >= e.mem.i32(this.addr() + 0x10) {
                break;
            }
        }
    }
    index = 0;
    if e.mem.i32(this.addr() + 8) > 0 {
        let mut record = this.addr() + 0x50;
        loop {
            e.mem.set_u32(record, 0);
            index += 1;
            record += 0x40;
            if index >= e.mem.i32(this.addr() + 8) {
                break;
            }
        }
    }
}

layout! {
    /// The PC `NiDX9SourceTextureData` (RTTI `.?AVNiDX9SourceTextureData@@`), 0x84 bytes (its
    /// deleting destructor frees 0x84). Names from the Xbox PDB's `NiXenonSourceTextureData`;
    /// the offsets are the PC code's (from +0x60 on they sit 4 lower than the Xbox class's).
    pub struct NiDX9SourceTextureData: 0x84 {
        /// `m_pkTexture` (Xbox PDB): the `NiTexture` this data belongs to.
        0x08 m_pkTexture: u32,
        /// `m_uiWidth` (Xbox PDB).
        0x0c m_uiWidth: u32,
        /// `m_uiHeight` (Xbox PDB).
        0x10 m_uiHeight: u32,
        /// `m_pkRenderer` (Xbox PDB): the renderer (its virtual +0xa0 answers the device
        /// capabilities, +0x288 holds the Direct3D device).
        0x60 m_pkRenderer: u32,
        /// The Direct3D texture (`m_pkD3DTexture` in the Xbox PDB).
        0x64 m_pkD3DTexture: u32,
        /// `m_uiLevels` (Xbox PDB): the mipmap level count.
        0x68 m_uiLevels: u32,
        /// `m_bReplacementData` (Xbox PDB).
        0x6c m_bReplacementData: u8,
        /// `m_bMipmap` (Xbox PDB).
        0x6d m_bMipmap: u8,
        /// `m_uiFormattedSize` (Xbox PDB).
        0x70 m_uiFormattedSize: u32,
        /// `m_spPalette` (Xbox PDB): an `NiPointer<NiPalette>`.
        0x74 m_spPalette: u32,
        /// `m_uiLevelsSkipped` (Xbox PDB).
        0x78 m_uiLevelsSkipped: u32,
        /// `m_uiSourceRevID` (Xbox PDB).
        0x7c m_uiSourceRevID: u32,
        /// `m_uiPalRevID` (Xbox PDB).
        0x80 m_uiPalRevID: u32,
    }
}

/// `NiDX9SourceTextureData`'s virtual table.
const SOURCE_TEXTURE_DATA_VTABLE: u32 = 0x010e_d37c;
/// `NiTexture::RendererData::m_kPixelFormat` (Xbox PDB) +0x14: an `NiPixelFormat` of 0x44
/// bytes (0x11 words); its `m_ucBitsPerPixel` is at +0x15.
const PIXEL_FORMAT: u32 = 0x14;
const PIXEL_FORMAT_WORDS: u32 = 0x11;
const BITS_PER_PIXEL: u32 = 0x15;
/// `NiTexture::m_pkRendererData` (Xbox PDB) +0x24.
const TEXTURE_RENDERER_DATA: u32 = 0x24;
/// `NiRefObject::m_uiRefCount` (Xbox PDB) +0x4 and the `DeleteThis` virtual (+0x4).
const REF_COUNT: u32 = 0x4;
const DELETE_THIS_SLOT: u32 = 0x4;
/// `NiPalette::m_uiRevID`: the word at +0x10 of a palette that the code copies as its
/// revision.
const PALETTE_REVISION: u32 = 0x10;

/// `NiPixelData` fields (Xbox PDB names, the PC code's offsets): `m_kPixelFormat` +0x08 (its
/// `m_eFormat` at +0x0c), `m_spPalette` +0x4c, `m_pucPixels` +0x50, `m_puiWidth` +0x54,
/// `m_puiHeight` +0x58, `m_puiOffsetInBytes` +0x5c, `m_uiMipmapLevels` +0x60,
/// `m_uiPixelStride` +0x64, `m_uiRevID` +0x68, `m_uiFaces` +0x6c.
const PIXEL_DATA_FORMAT: u32 = 0x08;
const PIXEL_DATA_FORMAT_KIND: u32 = 0x0c;
const PIXEL_DATA_PALETTE: u32 = 0x4c;
const PIXEL_DATA_PIXELS: u32 = 0x50;
const PIXEL_DATA_WIDTHS: u32 = 0x54;
const PIXEL_DATA_HEIGHTS: u32 = 0x58;
const PIXEL_DATA_OFFSETS: u32 = 0x5c;
const PIXEL_DATA_LEVELS: u32 = 0x60;
const PIXEL_DATA_STRIDE: u32 = 0x64;
const PIXEL_DATA_REVISION: u32 = 0x68;
const PIXEL_DATA_FACES: u32 = 0x6c;
/// `NiPixelFormat::FORMAT_DXT1` .. `FORMAT_DXT5` (Xbox PDB enum values 4..6).
const FORMAT_DXT1: i32 = 4;
const FORMAT_DXT5: i32 = 6;

/// `NiPersistentSrcTextureRendererData` fields (Xbox PDB, same offsets on PC): the pixel
/// format at +0x14, `m_puiWidth` +0x64, `m_puiHeight` +0x68, `m_puiOffsetInBytes` +0x6c,
/// `m_uiMipmapLevels` +0x74, `m_uiFaces` +0x7c, `m_spPalette` +0x90.
const PERSISTENT_WIDTHS: u32 = 0x64;
const PERSISTENT_HEIGHTS: u32 = 0x68;
const PERSISTENT_OFFSETS: u32 = 0x6c;
const PERSISTENT_LEVELS: u32 = 0x74;
const PERSISTENT_FACES: u32 = 0x7c;
const PERSISTENT_PALETTE: u32 = 0x90;

/// `NiSourceTexture` fields (Xbox PDB, same offsets on PC): `m_kFormatPrefs` +0x18 (its
/// `m_eMipMapped` at +0x20), `m_kPlatformFilename` +0x34, `m_spPersistentSrcRendererData`
/// +0x38, `m_spSrcPixelData` +0x3c, `m_bLoadDirectToRendererHint` +0x41, `m_pFile` +0x44.
const SOURCE_FORMAT_PREFS: u32 = 0x18;
const SOURCE_MIPMAPPED: u32 = 0x20;
const SOURCE_PLATFORM_FILENAME: u32 = 0x34;
const SOURCE_PERSISTENT_DATA: u32 = 0x38;
const SOURCE_PIXEL_DATA: u32 = 0x3c;
const SOURCE_LOAD_DIRECT_HINT: u32 = 0x41;
const SOURCE_FILE: u32 = 0x44;
/// `NiSourceCubeMap::ms_RTTI` (the record `nisourcecubemap.rs` names).
const NI_SOURCE_CUBE_MAP_RTTI: u32 = 0x011f_4454;
/// `NiRTTI`'s base-class pointer (+4).
const RTTI_BASE: u32 = 0x4;

/// The renderer's fields the code reads: the Direct3D device (+0x288), the table of pixel
/// formats it starts at +0x700 and the index (+0x880) of the one used for replacement data.
const RENDERER_DEVICE: u32 = 0x288;
const RENDERER_FORMATS: u32 = 0x700;
const RENDERER_REPLACEMENT_FORMAT_INDEX: u32 = 0x880;
/// The renderer's virtual +0xa0 (`__thiscall`, no words): device capability bits; bit 8
/// allows textures whose sides are not powers of two, bit 4 allows them conditionally.
const RENDERER_CAPS_SLOT: u32 = 0xa0;
const CAPS_NONPOW2: u32 = 0x8;
const CAPS_NONPOW2_CONDITIONAL: u32 = 0x4;
/// The texture data's virtual +0xb0 (`__thiscall`, one word: a Direct3D texture), the one
/// `00e68a80` initialises the data from ([`fn_00e68530`] in this class's table).
const INIT_FROM_D3D_TEXTURE_SLOT: u32 = 0xb0;
/// The source texture's virtual +0xa8 (`__thiscall`, no words), called after a texture's data
/// is made when the byte at `011f4448` is set.
const SOURCE_AFTER_CREATE_SLOT: u32 = 0xa8;
/// `NiObject::GetRTTI` (virtual +0x8).
const GET_RTTI_SLOT: u32 = 0x8;

/// Direct3D 9 COM methods (`__stdcall`, `this` first), by their slot in the interface:
/// `IDirect3DResource9::GetType` (+0x28), `IDirect3DBaseTexture9::GetLevelCount` (+0x34),
/// `IDirect3DTexture9::GetLevelDesc` (+0x44) and `GetSurfaceLevel` (+0x48),
/// `IDirect3DSurface9::GetDesc` (+0x30), `LockRect` (+0x34) and `UnlockRect` (+0x38),
/// `IUnknown::Release` (+0x8), `IDirect3DDevice9::CreateTexture` (+0x5c) and
/// `IDirect3D9::CheckDeviceFormat` (+0x28).
const D3D_GET_TYPE: u32 = 0x28;
const D3D_GET_LEVEL_COUNT: u32 = 0x34;
const D3D_GET_LEVEL_DESC: u32 = 0x44;
const D3D_GET_SURFACE_LEVEL: u32 = 0x48;
const D3D_SURFACE_GET_DESC: u32 = 0x30;
const D3D_SURFACE_LOCK_RECT: u32 = 0x34;
const D3D_SURFACE_UNLOCK_RECT: u32 = 0x38;
const D3D_RELEASE: u32 = 0x8;
const D3D_CREATE_TEXTURE: u32 = 0x5c;
const D3D_CHECK_DEVICE_FORMAT: u32 = 0x28;
/// `D3DRESOURCETYPE`: texture (3), volume texture (4), cube texture (5).
const D3DRTYPE_TEXTURE: u32 = 3;
const D3DRTYPE_VOLUMETEXTURE: u32 = 4;
const D3DRTYPE_CUBETEXTURE: u32 = 5;
/// `D3DFMT_A8P8` (0x28) and `D3DFMT_P8` (0x29): palettized formats the code refuses.
const D3DFMT_A8P8: u32 = 0x28;
const D3DFMT_P8: u32 = 0x29;
/// The four-character code `DXT1` and the others the code accepts (`DXT2`..`DXT5`).
const FOURCC_DXT1: u32 = 0x3154_5844;
const FOURCC_DXT2: u32 = 0x3254_5844;
const FOURCC_DXT3: u32 = 0x3354_5844;
const FOURCC_DXT4: u32 = 0x3454_5844;
const FOURCC_DXT5: u32 = 0x3554_5844;
/// `D3DUSAGE_DYNAMIC`.
const D3DUSAGE_DYNAMIC: u32 = 0x200;
/// `D3DERR_INVALIDCALL`.
const D3DERR_INVALIDCALL: u32 = 0x8876_086c;
/// `D3DXIFF_DDS`.
const D3DXIFF_DDS: u32 = 4;
/// The bump-map formats a texture whose pixel layout is 4 must have: `D3DFMT_V8U8` (0x3c),
/// `L6V5U5` (0x3d), `X8L8V8U8` (0x3e), `Q8W8V8U8` (0x3f), `V16U16` (0x40), `A2W10V10U10`
/// (0x43), `Q16W16V16U16` (0x6e) and `CxV8U8` (0x75), in the order the code tests them.
const BUMP_FORMATS: [u32; 8] = [0x3c, 0x3f, 0x40, 0x6e, 0x75, 0x3d, 0x3e, 0x43];

/// Globals: the number of mipmap levels to skip (`0126efc0`), the DDS skip count (`0126efc4`)
/// and minimum level count (`0126efc8`), the byte that makes textures dynamic (`0126f0d0`), the
/// `IDirect3D9` object (`0126f0d8`), the renderer global whose +0x5c4/+0x5c8/+0x5d0 words are
/// the adapter, device type and adapter format (`011f4748`), and three bytes that allow
/// loading straight to the renderer (`011f4509`), mipmapping by default (`011f4508`) and the
/// source texture's call after creation (`011f4448`).
const SKIP_LEVELS: u32 = 0x0126_efc0;
const DDS_SKIP_LEVELS: u32 = 0x0126_efc4;
const DDS_MIN_LEVELS: u32 = 0x0126_efc8;
const DYNAMIC_TEXTURES: u32 = 0x0126_f0d0;
const DIRECT3D: u32 = 0x0126_f0d8;
const RENDERER_GLOBAL: u32 = 0x011f_4748;
const LOAD_DIRECT_TO_RENDERER: u32 = 0x011f_4509;
const MIPMAP_BY_DEFAULT: u32 = 0x011f_4508;
const CALL_AFTER_CREATE: u32 = 0x011f_4448;
/// The memory manager `011f6238` (`Allocate` `00aa3e40`, `Deallocate` `00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;

/// Format strings the code passes to the empty debug print (`.rdata`).
const CREATE_SURF_FAILED: u32 = 0x010e_d180;
const GET_DESC_FAILED: u32 = 0x010e_d218;
const GET_SURFACE_LEVEL_FAILED: u32 = 0x010e_d1c8;
const COPY_GET_SURFACE_LEVEL_FAILED: u32 = 0x010e_d438;
const TEXTURE_NOT_FOUND: u32 = 0x010e_d4dc;
/// The extensions `00e68a80` loads (`.bmp`, `.tga`, `.dds`).
const EXTENSION_BMP: u32 = 0x0107_f794;
const EXTENSION_TGA: u32 = 0x0109_f4a4;
const EXTENSION_DDS: u32 = 0x0101_aa24;

// Callees outside this unit.
/// `00e8a260` (`__thiscall`, two words: texture, renderer) and `00e8a2b0` (`__thiscall`): the
/// constructor and destructor bodies of the `NiDX9TextureData` base (`NiXenonRenderer`
/// subsystem, unnamed).
const TEXTURE_DATA_CONSTRUCTOR: u32 = 0x00e8_a260;
const TEXTURE_DATA_DESTRUCTOR: u32 = 0x00e8_a2b0;
/// `00e8a300` (`__thiscall`, three words: source format, format preferences, the renderer's
/// format table): answers the pixel format to use, or 0.
const FIND_CLOSEST_FORMAT: u32 = 0x00e8_a300;
/// `00e8abf0` (cdecl, two words: texture, renderer): the cube map's own creation.
const CREATE_CUBE_MAP_DATA: u32 = 0x00e8_abf0;
/// `00e7bd20` (cdecl, one word: an `NiPixelFormat`): its Direct3D format.
const D3D_FORMAT_OF: u32 = 0x00e7_bd20;
/// `00e7c1e0` (cdecl, two words: a Direct3D format, an `NiPixelFormat` to fill).
const FILL_PIXEL_FORMAT: u32 = 0x00e7_c1e0;
/// `NiXenonErrorString` (`00e89d50`, cdecl, one word: an `HRESULT`).
const ERROR_STRING: u32 = 0x00e8_9d50;
/// `00a29680`: an empty debug print (cdecl, any words).
const DEBUG_PRINT: u32 = 0x00a2_9680;
/// `_memcpy` (`00ec44d0`, cdecl, three words).
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiPixelFormat::NiPixelFormat` (`00a7cca0`, `__thiscall`, no words; answers `this`).
const PIXEL_FORMAT_CONSTRUCTOR: u32 = 0x00a7_cca0;
/// `NiPointer<T>::operator=`'s shape (`00a5f860`, `__thiscall` on the slot, one word).
const POINTER_ASSIGN: u32 = 0x00a5_f860;
/// `00a62a10` (cdecl, three words: texture, code, 1; `NiMain/nitexture.cpp`): the replacement
/// pixel data for a texture that failed.
const REPLACEMENT_DATA: u32 = 0x00a6_2a10;
/// `00aa13e0` (cdecl, one word: a size), `00aa1070` (cdecl allocate, one word), `00aa10f0`
/// (cdecl free, one word) and `00aa1460` (cdecl sized delete, two words).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `NiImageConverter::GetImageConverter` (`00a76640`), and the converter's virtuals +0x0c
/// (read a file: name, 0, 0) and +0x14 (convert: source, format, source, mipmap flag).
const GET_IMAGE_CONVERTER: u32 = 0x00a7_6640;
const CONVERTER_READ_SLOT: u32 = 0x0c;
const CONVERTER_CONVERT_SLOT: u32 = 0x14;
/// `InterlockedIncrement`/`InterlockedDecrement` import slots (stdcall: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `_strcpy_s` (`00ec65a6`, cdecl: buffer, size, source), `NiPath::Standardize` (`00aa2a00`,
/// cdecl), `NiFilename::NiFilename` (`00aa27d0`, `__thiscall`, one word), `NiStricmp`
/// (`00aa1360`, cdecl, two words) and `NiFile::GetFile` (`00aa1a20`, cdecl: name, mode, buffer
/// size).
const STRCPY_S: u32 = 0x00ec_65a6;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const FILENAME_CONSTRUCTOR: u32 = 0x00aa_27d0;
const NI_STRICMP: u32 = 0x00aa_1360;
const FILE_GET_FILE: u32 = 0x00aa_1a20;
/// `00462d80` (`__thiscall` on a file, two words: buffer, size): reads the bytes.
const FILE_READ: u32 = 0x0046_2d80;
/// The file's virtuals: scalar deleting destructor (+0x0), `operator bool` (+0x4) and its size
/// (+0x1c).
const FILE_DELETE_SLOT: u32 = 0x0;
const FILE_GOOD_SLOT: u32 = 0x4;
const FILE_SIZE_SLOT: u32 = 0x1c;
/// The D3DX import thunks (`__stdcall`): `D3DXCreateVolumeTextureFromFileInMemory`
/// (`00ee6e10`), `D3DXCreateCubeTextureFromFileInMemory` (`00ee6e16`),
/// `D3DXCreateTextureFromFileInMemory` (`00ee6e1c`) and `D3DXGetImageInfoFromFileInMemory`
/// (`00ee6e22`).
const D3DX_CREATE_VOLUME_TEXTURE: u32 = 0x00ee_6e10;
const D3DX_CREATE_CUBE_TEXTURE: u32 = 0x00ee_6e16;
const D3DX_CREATE_TEXTURE: u32 = 0x00ee_6e1c;
const D3DX_GET_IMAGE_INFO: u32 = 0x00ee_6e22;

/// Whether `value` is a non-zero power of two (`x & (x - 1) == 0`).
fn is_power_of_two(value: u32) -> bool {
    value != 0 && value & value.wrapping_sub(1) == 0
}

/// `NiPointer` release as the code inlines it: the count at +4 goes down and at zero the
/// object deletes itself (virtual +4).
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

/// Prints `format` with an `HRESULT`'s error string through the empty debug print; `extra`
/// words go before the string.
fn print_error(e: &mut Engine, format: u32, extra: &[u32], result: u32) {
    let text = e.call(ERROR_STRING, &args![result]).u32();
    let mut words = vec![format];
    words.extend_from_slice(extra);
    words.push(text);
    e.call(DEBUG_PRINT, &words);
}

/// Copies an `NiPixelFormat` (0x11 words) from `from` into the texture data's +0x14.
fn copy_pixel_format(e: &mut Engine, this: u32, from: u32) {
    for i in 0..PIXEL_FORMAT_WORDS {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(this + PIXEL_FORMAT + 4 * i, word);
    }
}

// Translated from 00e68810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, no words): answers `this`. Identical code folding shares it among
/// many virtual tables (this class's included).
pub fn fn_00e68810(_e: &mut Engine, this: Ptr) -> Ptr {
    this
}

// Translated from 00e687f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, no words; a virtual of `NiDX9SourceTextureData` and other tables):
/// the word at +0x68, the mipmap level count of this class.
pub fn fn_00e687f0(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>) -> u32 {
    e.get(this, NiDX9SourceTextureData::m_uiLevels)
}

// Translated from 00e68800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, no words; a virtual of `NiDX9SourceTextureData` and other tables):
/// the word at +0x64, the Direct3D texture of this class.
pub fn fn_00e68800(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>) -> u32 {
    e.get(this, NiDX9SourceTextureData::m_pkD3DTexture)
}

// Translated from 00e687a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData` constructor (unnamed in the map; `__thiscall`, two words:
/// texture, renderer): the base constructor (`00e8a260`), this class's virtual table, no
/// replacement data, no mipmap, and the size, palette, skipped levels and revisions zero.
/// Answers `this`.
pub fn fn_00e687a0(
    e: &mut Engine,
    this: Ptr<NiDX9SourceTextureData>,
    texture: Ptr,
    renderer: Ptr,
) -> Ptr<NiDX9SourceTextureData> {
    e.call(TEXTURE_DATA_CONSTRUCTOR, &args![this, texture, renderer]);
    e.set(this, NiDX9SourceTextureData::m_bReplacementData, 0);
    e.set(this, NiDX9SourceTextureData::m_bMipmap, 0);
    e.set(this, NiDX9SourceTextureData::m_uiFormattedSize, 0);
    e.mem.set_u32(this.addr(), SOURCE_TEXTURE_DATA_VTABLE);
    e.set(this, NiDX9SourceTextureData::m_spPalette, 0);
    e.set(this, NiDX9SourceTextureData::m_uiLevelsSkipped, 0);
    e.set(this, NiDX9SourceTextureData::m_uiSourceRevID, 0);
    e.set(this, NiDX9SourceTextureData::m_uiPalRevID, 0);
    this
}

// Translated from 00e68760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, one word: a palette): takes a palette. False for a null palette;
/// otherwise the palette's revision (+0x10) is stored and the palette is assigned to
/// `m_spPalette` (`00a5f860`) when it is another one. Answers whether anything changed.
pub fn fn_00e68760(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>, palette: Ptr) -> bool {
    let mut changed = false;
    if palette.is_null() {
        return changed;
    }
    let revision = e.mem.u32(palette.addr() + PALETTE_REVISION);
    if e.get(this, NiDX9SourceTextureData::m_uiPalRevID) != revision {
        changed = true;
    }
    e.set(this, NiDX9SourceTextureData::m_uiPalRevID, revision);
    let slot = this.addr() + 0x74;
    if e.mem.u32(slot) != palette.addr() {
        e.call(POINTER_ASSIGN, &args![slot, palette]);
        changed = true;
    }
    changed
}

// Translated from 00e68830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData` destructor body (unnamed in the map): this class's virtual table,
/// the palette's reference given up (it deletes itself at zero), then the base destructor
/// (`00e8a2b0`, a tail jump).
pub fn fn_00e68830(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>) {
    e.mem.set_u32(this.addr(), SOURCE_TEXTURE_DATA_VTABLE);
    let palette = e.get(this, NiDX9SourceTextureData::m_spPalette);
    release(e, palette);
    e.call(TEXTURE_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 00e68ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData`'s scalar deleting destructor (unnamed in the map; slot 0 of its
/// table): the destructor body as [`fn_00e68830`] inlines it, then the sized delete
/// (`00aa1460`) of 0x84 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00e68ea0(
    e: &mut Engine,
    this: Ptr<NiDX9SourceTextureData>,
    flags: u32,
) -> Ptr<NiDX9SourceTextureData> {
    e.mem.set_u32(this.addr(), SOURCE_TEXTURE_DATA_VTABLE);
    let palette = e.get(this, NiDX9SourceTextureData::m_spPalette);
    release(e, palette);
    e.call(TEXTURE_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x84u32]);
    }
    this
}

// Translated from 00e68530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, one word: a Direct3D texture; virtual +0xb0 of this class): takes
/// its sizes and format from a Direct3D texture. The level count (`GetLevelCount`) is stored,
/// with "mipmapped" when it is not 1, and the formatted size starts at 0. For a texture
/// (`GetType` 3), a cube texture (5) or a volume texture (4) every level's description
/// (`GetLevelDesc`) is read: a failure answers false; at level 0 the width and height are
/// taken, a palettized format (`D3DFMT_A8P8`, `D3DFMT_P8`) answers false, and the pixel format
/// is reset (`NiPixelFormat` on a stack temporary, copied to +0x14) and filled from the
/// Direct3D format (`00e7c1e0`). Each level adds `bits * width * height (* depth) / 8` to the
/// size; a cube texture's total is then multiplied by 6. The Direct3D texture is stored at
/// +0x64 and the answer is true (also for any other resource type).
pub fn fn_00e68530(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>, texture: Ptr) -> bool {
    let data = this.addr();
    let texture = texture.addr();
    e.with_stack(0x64, |e, frame| {
        let desc = frame.addr();
        let pixel_format = desc + 0x20;
        let levels = e.vcall(texture, D3D_GET_LEVEL_COUNT, &args![]).u32();
        e.set(this, NiDX9SourceTextureData::m_uiLevels, levels);
        e.set(
            this,
            NiDX9SourceTextureData::m_bMipmap,
            u8::from(levels != 1),
        );
        e.set(this, NiDX9SourceTextureData::m_uiFormattedSize, 0);
        let kind = e.vcall(texture, D3D_GET_TYPE, &args![]).u32();
        // `D3DSURFACE_DESC` keeps width and height at +0x18/+0x1c; `D3DVOLUME_DESC` keeps
        // width, height and depth at +0x10/+0x14/+0x18.
        let (width_at, height_at, depth_at) = match kind {
            D3DRTYPE_TEXTURE | D3DRTYPE_CUBETEXTURE => (0x18, 0x1c, None),
            D3DRTYPE_VOLUMETEXTURE => (0x10, 0x14, Some(0x18)),
            _ => {
                e.set(this, NiDX9SourceTextureData::m_pkD3DTexture, texture);
                return true;
            }
        };
        let mut level = 0u32;
        while level < e.get(this, NiDX9SourceTextureData::m_uiLevels) {
            let result = e
                .vcall(texture, D3D_GET_LEVEL_DESC, &args![level, desc])
                .u32();
            if (result as i32) < 0 {
                return false;
            }
            if level == 0 {
                let height = e.mem.u32(desc + height_at);
                let width = e.mem.u32(desc + width_at);
                e.set(this, NiDX9SourceTextureData::m_uiHeight, height);
                e.set(this, NiDX9SourceTextureData::m_uiWidth, width);
                let format = e.mem.u32(desc);
                if format == D3DFMT_A8P8 || format == D3DFMT_P8 {
                    return false;
                }
                let reset = e.call(PIXEL_FORMAT_CONSTRUCTOR, &args![pixel_format]).u32();
                copy_pixel_format(e, data, reset);
                e.call(FILL_PIXEL_FORMAT, &args![format, data + PIXEL_FORMAT]);
            }
            let bits = u32::from(e.mem.u8(data + BITS_PER_PIXEL));
            // `IMUL`s, so the product wraps as the 32-bit one does in any order.
            let mut product = bits.wrapping_mul(e.mem.u32(desc + height_at));
            if let Some(depth_at) = depth_at {
                product = product.wrapping_mul(e.mem.u32(desc + depth_at));
            }
            let product = product.wrapping_mul(e.mem.u32(desc + width_at));
            let size = e
                .get(this, NiDX9SourceTextureData::m_uiFormattedSize)
                .wrapping_add(product >> 3);
            e.set(this, NiDX9SourceTextureData::m_uiFormattedSize, size);
            level += 1;
        }
        e.set(this, NiDX9SourceTextureData::m_pkD3DTexture, texture);
        if kind == D3DRTYPE_CUBETEXTURE {
            let size = e.get(this, NiDX9SourceTextureData::m_uiFormattedSize);
            e.set(
                this,
                NiDX9SourceTextureData::m_uiFormattedSize,
                size.wrapping_mul(6),
            );
        }
        true
    })
}

// Translated from 00e68150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData::CopyDataToSurfaceLevel` (named by the messages it prints; cdecl,
/// four words: pixel data, level, surface, face). Reads the surface's description and locks
/// it (a failure of either prints its message with the error string and returns). The rows
/// of the level are `stride * width` bytes, and as many as the level's height, at most the
/// surface's; for a compressed pixel data format (DXT1..DXT5) the pitch counts block rows (a
/// quarter), a row is `(width + 3) / 2` (DXT1 surface) or `width + 3` rounded down to 4 bytes,
/// and the height is rounded up to 4. The source is `offsets[levels] * face + offsets[level]`
/// into the pixels. When a row is exactly the pitch one copy does it all; otherwise each row
/// copies the smaller of the two, the destination moving by the surface's pitch. The surface is
/// unlocked.
pub fn fn_00e68150(e: &mut Engine, pixel_data: Ptr, level: u32, surface: Ptr, face: u32) {
    let pixel_data = pixel_data.addr();
    let surface = surface.addr();
    e.with_stack(0x2c, |e, frame| {
        let frame = frame.addr();
        let locked = frame + 4;
        let desc = frame + 0xc;
        let result = e.vcall(surface, D3D_SURFACE_GET_DESC, &args![desc]).u32();
        if (result as i32) < 0 {
            print_error(e, GET_DESC_FAILED, &[], result);
            return;
        }
        let result = e
            .vcall(surface, D3D_SURFACE_LOCK_RECT, &args![locked, 0u32, 0u32])
            .u32();
        if (result as i32) < 0 {
            print_error(e, GET_SURFACE_LEVEL_FAILED, &[], result);
            return;
        }
        let widths = e.mem.u32(pixel_data + PIXEL_DATA_WIDTHS);
        let mut pitch = e.mem.u32(locked);
        let width = e.mem.u32(widths + 4 * level);
        let heights = e.mem.u32(pixel_data + PIXEL_DATA_HEIGHTS);
        let mut height = e.mem.u32(heights + 4 * level);
        let surface_height = e.mem.u32(desc + 0x1c);
        let mut row = e
            .mem
            .u32(pixel_data + PIXEL_DATA_STRIDE)
            .wrapping_mul(width);
        if height > surface_height {
            height = surface_height;
        }
        let format = e.mem.i32(pixel_data + PIXEL_DATA_FORMAT_KIND);
        if (FORMAT_DXT1..=FORMAT_DXT5).contains(&format) {
            pitch >>= 2;
            row = width.wrapping_add(3);
            if e.mem.u32(desc) == FOURCC_DXT1 {
                row = (row >> 1) & 0x07ff_fffe;
            } else {
                row &= 0x0fff_fffc;
            }
            height = height.wrapping_add(3) & 0x0fff_fffc;
        }
        let offsets = e.mem.u32(pixel_data + PIXEL_DATA_OFFSETS);
        let levels = e.mem.u32(pixel_data + PIXEL_DATA_LEVELS);
        let face_size = e.mem.u32(offsets + 4 * levels);
        let source = face_size
            .wrapping_mul(face)
            .wrapping_add(e.mem.u32(offsets + 4 * level))
            .wrapping_add(e.mem.u32(pixel_data + PIXEL_DATA_PIXELS));
        let bits = e.mem.u32(locked + 4);
        if row == pitch {
            e.call(MEMCPY, &args![bits, source, height.wrapping_mul(row)]);
        } else {
            let copy = if row > pitch { pitch } else { row };
            e.mem.set_u32(frame, copy);
            let (mut to, mut from) = (bits, source);
            for _ in 0..height {
                let copy = e.mem.u32(frame);
                e.call(MEMCPY, &args![to, from, copy]);
                to = to.wrapping_add(e.mem.u32(locked));
                from = from.wrapping_add(row);
            }
        }
        e.vcall(surface, D3D_SURFACE_UNLOCK_RECT, &args![]);
    });
}

// Translated from 00e68010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData::CreateSurf` (named by its message; `__thiscall`, one word: pixel
/// data). The Direct3D format comes from the pixel format at +0x14 (`00e7bd20`), the sizes from
/// the pixel data's level 0. The level count is the pixel data's when both sides are powers of
/// two or the renderer allows other sizes (capability bit 8), else 1. The skipped levels are
/// the global at `0126efc0`, at most `levels - 1`; the sides are halved (rounding up) once per
/// skipped level. Then the device's `CreateTexture(width, height, levels - skipped, usage,
/// format, pool, &texture, 0)` runs, dynamic (usage `0x200`, pool 0) when the byte at `0126f0d0`
/// is set, managed (usage 0, pool 1) otherwise. On success with a texture it is stored at +0x64
/// and the answer is true; otherwise the error is printed, +0x64 is cleared and the answer is
/// false.
pub fn fn_00e68010(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>, pixel_data: Ptr) -> bool {
    let data = this.addr();
    let pixel_data = pixel_data.addr();
    e.with_stack(8, |e, frame| {
        let texture_out = frame.addr();
        let format_at = texture_out + 4;
        let format = e.call(D3D_FORMAT_OF, &args![data + PIXEL_FORMAT]).u32();
        e.mem.set_u32(format_at, format);
        let width = e.mem.u32(e.mem.u32(pixel_data + PIXEL_DATA_WIDTHS));
        e.set(this, NiDX9SourceTextureData::m_uiWidth, width);
        let height = e.mem.u32(e.mem.u32(pixel_data + PIXEL_DATA_HEIGHTS));
        e.set(this, NiDX9SourceTextureData::m_uiHeight, height);
        let mut full_chain = is_power_of_two(width) && is_power_of_two(height);
        if !full_chain {
            let renderer = e.get(this, NiDX9SourceTextureData::m_pkRenderer);
            let caps = e.vcall(renderer, RENDERER_CAPS_SLOT, &args![]).u32();
            full_chain = caps & CAPS_NONPOW2 != 0;
        }
        let levels = if full_chain {
            e.mem.u32(pixel_data + PIXEL_DATA_LEVELS)
        } else {
            1
        };
        e.set(this, NiDX9SourceTextureData::m_uiLevels, levels);
        let mut skipped = e.mem.u32(SKIP_LEVELS);
        let levels = e.get(this, NiDX9SourceTextureData::m_uiLevels);
        if skipped > levels.wrapping_sub(1) {
            skipped = levels.wrapping_sub(1);
        }
        if skipped != 0 {
            let mut width = e.get(this, NiDX9SourceTextureData::m_uiWidth);
            let mut height = e.get(this, NiDX9SourceTextureData::m_uiHeight);
            for _ in 0..skipped {
                if width & 1 != 0 {
                    width = width.wrapping_add(1);
                }
                width >>= 1;
                if height & 1 != 0 {
                    height = height.wrapping_add(1);
                }
                height >>= 1;
            }
            e.set(this, NiDX9SourceTextureData::m_uiWidth, width);
            e.set(this, NiDX9SourceTextureData::m_uiHeight, height);
        }
        e.set(this, NiDX9SourceTextureData::m_uiLevelsSkipped, skipped);
        let renderer = e.get(this, NiDX9SourceTextureData::m_pkRenderer);
        let device = e.mem.u32(renderer + RENDERER_DEVICE);
        let dynamic = e.mem.u8(DYNAMIC_TEXTURES);
        let pool = u32::from(dynamic == 0);
        let usage = if dynamic != 0 { D3DUSAGE_DYNAMIC } else { 0 };
        let format = e.mem.u32(format_at);
        e.mem.set_u32(texture_out, 0);
        let width = e.get(this, NiDX9SourceTextureData::m_uiWidth);
        let height = e.get(this, NiDX9SourceTextureData::m_uiHeight);
        let result = e
            .vcall(
                device,
                D3D_CREATE_TEXTURE,
                &args![
                    width,
                    height,
                    levels.wrapping_sub(skipped),
                    usage,
                    format,
                    pool,
                    texture_out,
                    0u32
                ],
            )
            .u32();
        let texture = e.mem.u32(texture_out);
        if (result as i32) >= 0 && texture != 0 {
            e.set(this, NiDX9SourceTextureData::m_pkD3DTexture, texture);
            return true;
        }
        print_error(e, CREATE_SURF_FAILED, &[], result);
        e.set(this, NiDX9SourceTextureData::m_pkD3DTexture, 0);
        false
    })
}

// Translated from 00e68870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData::CopyDataToSurface` (named by its message; `__thiscall`, one word:
/// pixel data). Nothing without a Direct3D texture or for a cube or volume texture. Otherwise
/// the level count becomes the pixel data's, and each level from the skipped ones on gets its
/// surface (`GetSurfaceLevel`), its data copied ([`fn_00e68150`] with face 0) and the surface
/// released; a failed `GetSurfaceLevel` prints the level and the error and stops.
pub fn fn_00e68870(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>, pixel_data: Ptr) {
    let texture = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture);
    if texture == 0 {
        return;
    }
    if e.vcall(texture, D3D_GET_TYPE, &args![]).u32() == D3DRTYPE_CUBETEXTURE {
        return;
    }
    let texture = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture);
    if e.vcall(texture, D3D_GET_TYPE, &args![]).u32() == D3DRTYPE_VOLUMETEXTURE {
        return;
    }
    let texture = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture);
    let levels = e.mem.u32(pixel_data.addr() + PIXEL_DATA_LEVELS);
    e.set(this, NiDX9SourceTextureData::m_uiLevels, levels);
    let skipped = e.get(this, NiDX9SourceTextureData::m_uiLevelsSkipped);
    if levels.wrapping_sub(skipped) == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let mut level = 0u32;
        loop {
            let result = e
                .vcall(texture, D3D_GET_SURFACE_LEVEL, &args![level, slot])
                .u32();
            if (result as i32) < 0 {
                print_error(e, COPY_GET_SURFACE_LEVEL_FAILED, &[level], result);
                return;
            }
            let surface = e.mem.u32(slot);
            let skipped = e.get(this, NiDX9SourceTextureData::m_uiLevelsSkipped);
            fn_00e68150(
                e,
                pixel_data,
                skipped.wrapping_add(level),
                Ptr::new(surface),
                0,
            );
            let surface = e.mem.u32(slot);
            e.vcall(surface, D3D_RELEASE, &args![]);
            level += 1;
            let levels = e.get(this, NiDX9SourceTextureData::m_uiLevels);
            let skipped = e.get(this, NiDX9SourceTextureData::m_uiLevelsSkipped);
            if level >= levels.wrapping_sub(skipped) {
                return;
            }
        }
    });
}

// Translated from 00e68a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e68a80 - the /GS stack cookie (stored at frame +0x54c and checked through 00ec408c) is not translated
/// Unnamed (`__thiscall`, four words: file name, renderer, format preferences, file):
/// creates the Direct3D texture straight from a texture file. False without a renderer or
/// device. Without a file the name is copied (`_strcpy_s`, 0x104 bytes), standardized and
/// split (`NiFilename`); only `.bmp`, `.tga` and `.dds` are opened (`NiFile::GetFile` with mode
/// 0 and buffer 0x8000). A file that is not good is deleted and the answer is false. Its bytes
/// are read into a new buffer (`00aa1070`, `00462d80`) and the file is deleted. The image
/// description (`D3DXGetImageInfoFromFileInMemory`) must succeed, and with pixel layout 4 the
/// format must be a bump-map one; otherwise the buffer is freed and the answer is false. For a
/// texture: a DDS file whose pixel format has a DXT four-character code first drops its top
/// mipmap levels, keeping `levels - [0126efc4]` but at least `[0126efc8]` of them (or all when
/// there are no more than that): each dropped level moves the data on by its block bytes and
/// halves the sides; then a new buffer (`MemoryManager::Allocate`) gets the 0x80-byte header with
/// the new sides, level count and linear size, and the remaining data, and the old buffer is
/// freed (`Deallocate`). Note that the size passed on is still the file's. Then
/// `D3DXCreateTextureFromFileInMemory`, `...CubeTexture...` or `...VolumeTexture...` by resource
/// type (any other is `D3DERR_INVALIDCALL`); the buffer is freed (`00aa10f0`). A failure or no
/// texture answers false; otherwise the data takes the texture (virtual +0xb0), keeps the
/// image's first width and height, and releases the texture when that refused it. Answers what
/// the virtual answered.
pub fn fn_00e68a80(
    e: &mut Engine,
    this: Ptr,
    file_name: Ptr,
    renderer: Ptr,
    format_prefs: Ptr,
    file: Ptr,
) -> bool {
    let this = this.addr();
    e.with_stack(0x550, |e, frame| {
        let frame = frame.addr();
        let volume_out = frame;
        let size_slot = frame + 4;
        let texture_out = frame + 0x14;
        let info = frame + 0x20;
        let filename = frame + 0x44;
        let path = frame + 0x448;
        if renderer.is_null() {
            return false;
        }
        let device = e.mem.u32(renderer.addr() + RENDERER_DEVICE);
        if device == 0 {
            return false;
        }
        let mut file = file.addr();
        if file == 0 {
            e.call(STRCPY_S, &args![path, 0x104u32, file_name]);
            e.call(PATH_STANDARDIZE, &args![path]);
            e.call(FILENAME_CONSTRUCTOR, &args![filename, path]);
            // `NiFilename`'s extension, +0x103 into it.
            let extension = filename + 0x103;
            let mut known = false;
            for name in [EXTENSION_BMP, EXTENSION_TGA, EXTENSION_DDS] {
                if e.call(NI_STRICMP, &args![extension, name]).u32() == 0 {
                    known = true;
                    break;
                }
            }
            if !known {
                return false;
            }
            file = e.call(FILE_GET_FILE, &args![path, 0u32, 0x8000u32]).u32();
            if file == 0 {
                return false;
            }
        }
        if !e.vcall(file, FILE_GOOD_SLOT, &args![]).bool() {
            e.vcall(file, FILE_DELETE_SLOT, &args![1u32]);
            return false;
        }
        let size = e.vcall(file, FILE_SIZE_SLOT, &args![]).u32();
        e.mem.set_u32(size_slot, size);
        if size == 0 {
            e.vcall(file, FILE_DELETE_SLOT, &args![1u32]);
            return false;
        }
        let mut buffer = e.call(ALLOCATE, &args![size]).u32();
        e.call(FILE_READ, &args![file, buffer, size]);
        e.vcall(file, FILE_DELETE_SLOT, &args![1u32]);
        let result = e
            .call(D3DX_GET_IMAGE_INFO, &args![buffer, size, info])
            .u32();
        let format = e.mem.u32(info + 0x10);
        if (result as i32) < 0
            || (e.mem.u32(format_prefs.addr()) == 4 && !BUMP_FORMATS.contains(&format))
        {
            e.call(FREE, &args![buffer]);
            return false;
        }
        let first_height = e.mem.u32(info + 4);
        let first_width = e.mem.u32(info);
        let kind = e.mem.u32(info + 0x14);
        let mut texture = 0u32;
        let result = match kind {
            D3DRTYPE_TEXTURE => {
                e.mem.set_u32(texture_out, 0);
                let mut size = size;
                if e.mem.u32(info + 0x18) == D3DXIFF_DDS {
                    let skip = e.mem.u32(DDS_SKIP_LEVELS);
                    let minimum = e.mem.u32(DDS_MIN_LEVELS);
                    // The DDS header's pixel format flags (+0x50) and four-character code (+0x54).
                    if e.mem.u32(buffer + 0x50) & 4 != 0 {
                        let divisor = match e.mem.u32(buffer + 0x54) {
                            FOURCC_DXT1 => Some(2u32),
                            FOURCC_DXT2 | FOURCC_DXT3 | FOURCC_DXT4 | FOURCC_DXT5 => Some(1),
                            _ => None,
                        };
                        if let Some(divisor) = divisor {
                            size =
                                dds_drop_levels(e, &mut buffer, size, info, skip, minimum, divisor);
                        }
                    }
                }
                let result = e
                    .call(
                        D3DX_CREATE_TEXTURE,
                        &args![device, buffer, size, texture_out],
                    )
                    .u32();
                texture = e.mem.u32(texture_out);
                result
            }
            D3DRTYPE_CUBETEXTURE => {
                e.mem.set_u32(size_slot, 0);
                let result = e
                    .call(
                        D3DX_CREATE_CUBE_TEXTURE,
                        &args![device, buffer, size, size_slot],
                    )
                    .u32();
                texture = e.mem.u32(size_slot);
                result
            }
            D3DRTYPE_VOLUMETEXTURE => {
                e.mem.set_u32(volume_out, 0);
                let result = e
                    .call(
                        D3DX_CREATE_VOLUME_TEXTURE,
                        &args![device, buffer, size, volume_out],
                    )
                    .u32();
                texture = e.mem.u32(volume_out);
                result
            }
            _ => D3DERR_INVALIDCALL,
        };
        e.call(FREE, &args![buffer]);
        if (result as i32) < 0 || texture == 0 {
            return false;
        }
        let taken = e
            .vcall(this, INIT_FROM_D3D_TEXTURE_SLOT, &args![texture])
            .bool();
        e.mem.set_u32(this + 0xc, first_width);
        e.mem.set_u32(this + 0x10, first_height);
        if !taken {
            e.vcall(texture, D3D_RELEASE, &args![]);
        }
        taken
    })
}

/// The mipmap dropping of [`fn_00e68a80`] for a DXT DDS file (`divisor` 2 for DXT1, else 1):
/// answers the size the code passes on, which is the file's size again whether or not it
/// replaced the buffer.
fn dds_drop_levels(
    e: &mut Engine,
    buffer: &mut u32,
    size: u32,
    info: u32,
    skip: u32,
    minimum: u32,
    divisor: u32,
) -> u32 {
    let levels = e.mem.u32(*buffer + 0x1c);
    let mut data = *buffer + 0x80;
    let mut keep = levels.wrapping_sub(skip);
    if keep <= minimum {
        keep = minimum;
    }
    if levels <= skip {
        keep = levels;
        if levels >= minimum {
            keep = minimum;
        }
    }
    let mut remaining = levels;
    let mut bytes = size;
    let mut width = e.mem.u32(info);
    if keep < levels {
        let mut count = levels - keep;
        loop {
            let blocks_wide = if width >> 2 >= 1 { width >> 2 } else { 1 };
            let height = e.mem.u32(info + 4);
            let blocks_high = if height >> 2 >= 1 { height >> 2 } else { 1 };
            let level_bytes = (blocks_high.wrapping_mul(blocks_wide) << 4) / divisor;
            data = data.wrapping_add(level_bytes);
            bytes = bytes.wrapping_sub(level_bytes);
            width = if width >> 1 >= 1 { width >> 1 } else { 1 };
            let half = height >> 1;
            e.mem.set_u32(info + 4, if half >= 1 { half } else { 1 });
            remaining = remaining.wrapping_sub(1);
            count -= 1;
            if count == 0 {
                break;
            }
        }
        e.mem.set_u32(info, width);
    }
    if remaining != e.mem.u32(*buffer + 0x1c) {
        let fresh = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
            .u32();
        e.call(MEMCPY, &args![fresh, *buffer, 0x80u32]);
        let width = e.mem.u32(info);
        let height = e.mem.u32(info + 4);
        // The DDS header's height (+0x0c), width (+0x10), linear size (+0x14), level count
        // (+0x1c).
        e.mem.set_u32(fresh + 0x10, width);
        e.mem.set_u32(fresh + 0xc, height);
        e.mem.set_u32(fresh + 0x1c, remaining);
        e.mem.set_u32(fresh + 0x14, height.wrapping_mul(width));
        e.call(MEMCPY, &args![fresh + 0x80, data, bytes.wrapping_sub(0x80)]);
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, *buffer]);
        *buffer = fresh;
    }
    size
}

/// `NV_FACE_CHECK` (diagnostic only, not exe behaviour): notes the file name of the source
/// texture `texture` under the Direct3D texture its renderer data `data` holds (+0x64), for the
/// real-device side's draw lines; textures made from pixel data are named by that.
fn face_check_name(e: &Engine, texture: u32, data: u32) {
    use crate::units::fallout_shared_facegen::bsfacegenmodel::face_check;
    if !face_check::enabled() || data == 0 {
        return;
    }
    let d3d = e.mem.u32(data.wrapping_add(0x64));
    if d3d == 0 {
        return;
    }
    let file = e.mem.u32(texture.wrapping_add(SOURCE_PLATFORM_FILENAME));
    let name = if file != 0 {
        String::from_utf8_lossy(&e.mem.cstr(file)).into_owned()
    } else if e.mem.u32(texture.wrapping_add(SOURCE_PIXEL_DATA)) != 0 {
        format!("(pixel data of {texture:#x})")
    } else {
        format!("(texture {texture:#x})")
    };
    engine_core::units::platform::FACE_CHECK_NAMES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(d3d, name);
}

// Translated from 00e68ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9SourceTextureData::Create`'s shape (unnamed in the map; cdecl, two words: source
/// texture, renderer): makes a source texture's renderer data. A cube map (its RTTI chain
/// reaches `NiSourceCubeMap`'s) is made by `00e8abf0`. Otherwise a new 0x84-byte object
/// (`00aa13e0`) is constructed as [`fn_00e687a0`] does, and its pixel format comes from one of
/// three sources:
/// - Without persistent data or pixel data: when loading straight to the renderer is allowed
///   (byte `011f4509`), the texture hints it and there is a renderer, [`fn_00e68a80`] loads the
///   platform file name and on success the texture takes the data. Otherwise the image
///   converter reads the file (virtual +0x0c).
/// - The pixel data (held by a reference while this runs) or, failing all, replacement data:
///   with no pixel data either, `00a62a10(texture, 0x80000005, 1)` gives replacement data and
///   "Failed to find texture" is printed. With pixel data, the format is chosen by `00e8a300`
///   (from the data's format, the texture's preferences or, for a non-power-of-two image with
///   pixel layout 3 on a renderer that allows it conditionally, a copy of them with layout 2,
///   and the renderer's formats); a renderer that allows neither gives replacement data with
///   code `0x80000006`, a failed choice with `0x80000004`.
/// - Persistent data: the format is chosen the same way from the persistent data's format
///   (failures as above), its Direct3D format (`00e7bd20`) must exist and pass the device's
///   `CheckDeviceFormat`, each failure giving replacement data with `0x80000004`.
///
/// With replacement data the format is the renderer's replacement format and there is no
/// mipmap. The data then takes the sizes, revision and format; mipmapping follows the
/// texture's preference (1, or 2 when `011f4508` is set); the pixel data is converted (unless
/// it is replacement data) and copied in ([`fn_00e68010`] when there is no Direct3D texture
/// yet, then [`fn_00e68870`]), with its level count, palette and size. Persistent data is
/// used as it is (`00e682b0`, `00e68920`). Finally the texture takes the data, the texture's
/// virtual +0xa8 runs when `011f4448` is set, and the held pixel data is released. Answers the
/// new data.
pub fn fn_00e68ef0(e: &mut Engine, texture: Ptr, renderer: Ptr) -> Ptr {
    let texture = texture.addr();
    let renderer = renderer.addr();
    if texture != 0 {
        let mut rtti = e.vcall(texture, GET_RTTI_SLOT, &args![]).u32();
        while rtti != 0 {
            if rtti == NI_SOURCE_CUBE_MAP_RTTI {
                return e
                    .call(CREATE_CUBE_MAP_DATA, &args![texture, renderer])
                    .ptr();
            }
            rtti = e.mem.u32(rtti + RTTI_BASE);
        }
    }
    e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let chosen_at = frame;
        let formats = renderer.wrapping_add(RENDERER_FORMATS);
        e.mem.set_u32(frame + 8, formats);
        let object = e.call(NEW_OBJECT, &args![0x84u32]).u32();
        let data = object;
        if data != 0 {
            e.call(TEXTURE_DATA_CONSTRUCTOR, &args![data, texture, renderer]);
            e.mem.set_u32(data, SOURCE_TEXTURE_DATA_VTABLE);
            e.mem.set_u8(data + 0x6c, 0);
            e.mem.set_u8(data + 0x6d, 0);
            for offset in [0x70, 0x74, 0x78, 0x7c, 0x80] {
                e.mem.set_u32(data + offset, 0);
            }
        }
        let this: Ptr<NiDX9SourceTextureData> = Ptr::new(data);
        let persistent = e.mem.u32(texture + SOURCE_PERSISTENT_DATA);
        let prefs = texture + SOURCE_FORMAT_PREFS;
        let mut pixel_data = 0u32;
        if persistent == 0 {
            pixel_data = e.mem.u32(texture + SOURCE_PIXEL_DATA);
            if pixel_data == 0 {
                let name = e.mem.u32(texture + SOURCE_PLATFORM_FILENAME);
                if e.mem.u8(LOAD_DIRECT_TO_RENDERER) != 0
                    && e.mem.u8(texture + SOURCE_LOAD_DIRECT_HINT) != 0
                    && renderer != 0
                {
                    let source_file = e.mem.u32(texture + SOURCE_FILE);
                    let loaded = fn_00e68a80(
                        e,
                        Ptr::new(data),
                        Ptr::new(name),
                        Ptr::new(renderer),
                        Ptr::new(prefs),
                        Ptr::new(source_file),
                    );
                    if loaded {
                        let owner = e.get(this, NiDX9SourceTextureData::m_pkTexture);
                        e.mem.set_u32(owner + TEXTURE_RENDERER_DATA, data);
                        face_check_name(e, texture, data);
                        return this.cast();
                    }
                }
                let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
                pixel_data = e
                    .vcall(converter, CONVERTER_READ_SLOT, &args![name, 0u32, 0u32])
                    .u32();
            }
        }
        e.mem.set_u32(chosen_at, 0);
        // The reference this function holds on the pixel data until it returns.
        let held = pixel_data;
        e.mem.set_u32(frame + 0x18, held);
        if held != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![held + REF_COUNT]);
        }
        let replacement = |e: &mut Engine, code: u32| -> u32 {
            let data = e.call(REPLACEMENT_DATA, &args![texture, code, 1u32]).u32();
            e.mem.set_u8(object + 0x6c, 1);
            data
        };
        if persistent != 0 {
            let width = e.mem.u32(e.mem.u32(persistent + PERSISTENT_WIDTHS));
            let height = e.mem.u32(e.mem.u32(persistent + PERSISTENT_HEIGHTS));
            let mut allowed = is_power_of_two(width) && is_power_of_two(height);
            if !allowed {
                let caps = e.vcall(renderer, RENDERER_CAPS_SLOT, &args![]).u32();
                allowed = caps & CAPS_NONPOW2 != 0;
                if !allowed {
                    let caps = e.vcall(renderer, RENDERER_CAPS_SLOT, &args![]).u32();
                    allowed = caps & CAPS_NONPOW2_CONDITIONAL != 0;
                }
            }
            if allowed {
                let formats = e.mem.u32(frame + 8);
                let chosen = e
                    .call(
                        FIND_CLOSEST_FORMAT,
                        &args![data, persistent + PIXEL_FORMAT, prefs, formats],
                    )
                    .u32();
                e.mem.set_u32(chosen_at, chosen);
                if chosen == 0 {
                    pixel_data = replacement(e, 0x8000_0004);
                }
            } else {
                pixel_data = replacement(e, 0x8000_0006);
            }
            let device_format = e
                .call(D3D_FORMAT_OF, &args![persistent + PIXEL_FORMAT])
                .u32();
            e.mem.set_u32(frame + 0xc, device_format);
            if device_format == 0 {
                pixel_data = replacement(e, 0x8000_0004);
            }
            let global = e.mem.u32(RENDERER_GLOBAL);
            let device_type = e.mem.u32(global + 0x5c8);
            let adapter_format = e.mem.u32(global + 0x5d0);
            let adapter = e.mem.u32(global + 0x5c4);
            let direct3d = e.mem.u32(DIRECT3D);
            e.mem.set_u32(frame + 0x10, device_type);
            e.mem.set_u32(frame + 0x14, adapter);
            let result = e
                .vcall(
                    direct3d,
                    D3D_CHECK_DEVICE_FORMAT,
                    &args![
                        adapter,
                        device_type,
                        adapter_format,
                        0u32,
                        D3DRTYPE_TEXTURE,
                        device_format
                    ],
                )
                .u32();
            if (result as i32) < 0 {
                pixel_data = replacement(e, 0x8000_0004);
            }
        } else if pixel_data == 0 {
            pixel_data = e
                .call(REPLACEMENT_DATA, &args![texture, 0x8000_0005u32, 1u32])
                .u32();
            let name = e.mem.u32(texture + SOURCE_PLATFORM_FILENAME);
            e.call(DEBUG_PRINT, &args![TEXTURE_NOT_FOUND, name]);
            e.mem.set_u8(object + 0x6c, 1);
        } else {
            let width = e.mem.u32(e.mem.u32(pixel_data + PIXEL_DATA_WIDTHS));
            let height = e.mem.u32(e.mem.u32(pixel_data + PIXEL_DATA_HEIGHTS));
            let mut prefs_used = Some(prefs);
            if !(is_power_of_two(width) && is_power_of_two(height)) {
                let caps = e.vcall(renderer, RENDERER_CAPS_SLOT, &args![]).u32();
                if caps & CAPS_NONPOW2 == 0 {
                    let caps = e.vcall(renderer, RENDERER_CAPS_SLOT, &args![]).u32();
                    if caps & CAPS_NONPOW2_CONDITIONAL == 0 {
                        prefs_used = None;
                    } else if e.mem.u32(prefs) == 3 {
                        // A copy of the preferences with pixel layout 2 (frame +0x1c).
                        let copy = frame + 0x1c;
                        let layout = e.mem.u32(prefs);
                        let alpha = e.mem.u32(prefs + 4);
                        let mipmap = e.mem.u32(prefs + 8);
                        e.mem.set_u32(copy, layout);
                        e.mem.set_u32(copy + 4, alpha);
                        e.mem.set_u32(copy + 8, mipmap);
                        e.mem.set_u32(copy, 2);
                        prefs_used = Some(copy);
                    }
                }
            }
            match prefs_used {
                Some(prefs_used) => {
                    let formats = e.mem.u32(frame + 8);
                    let chosen = e
                        .call(
                            FIND_CLOSEST_FORMAT,
                            &args![data, pixel_data + PIXEL_DATA_FORMAT, prefs_used, formats],
                        )
                        .u32();
                    e.mem.set_u32(chosen_at, chosen);
                    if chosen == 0 {
                        pixel_data = replacement(e, 0x8000_0004);
                    }
                }
                None => {
                    pixel_data = replacement(e, 0x8000_0006);
                }
            }
        }
        let replaced = e.mem.u8(object + 0x6c) != 0;
        if replaced {
            let index = e.mem.u32(renderer + RENDERER_REPLACEMENT_FORMAT_INDEX);
            let formats = e.mem.u32(frame + 8);
            let format = e.mem.u32(formats.wrapping_add(index.wrapping_mul(4)));
            e.mem.set_u32(chosen_at, format);
        }
        if !replaced && persistent != 0 {
            copy_pixel_format(e, data, persistent + PIXEL_FORMAT);
            let width = e.mem.u32(e.mem.u32(persistent + PERSISTENT_WIDTHS));
            e.set(this, NiDX9SourceTextureData::m_uiWidth, width);
            let height = e.mem.u32(e.mem.u32(persistent + PERSISTENT_HEIGHTS));
            e.set(this, NiDX9SourceTextureData::m_uiHeight, height);
            let levels = e.mem.u32(persistent + PERSISTENT_LEVELS);
            e.set(this, NiDX9SourceTextureData::m_uiLevels, levels);
            e.set(
                this,
                NiDX9SourceTextureData::m_bMipmap,
                u8::from(levels != 1),
            );
            e.set(this, NiDX9SourceTextureData::m_bReplacementData, 0);
            e.set(this, NiDX9SourceTextureData::m_pkRenderer, renderer);
            let offsets = e.mem.u32(persistent + PERSISTENT_OFFSETS);
            let levels = e.mem.u32(persistent + PERSISTENT_LEVELS);
            let size = e
                .mem
                .u32(offsets.wrapping_add(levels.wrapping_mul(4)))
                .wrapping_mul(e.mem.u32(persistent + PERSISTENT_FACES));
            e.set(this, NiDX9SourceTextureData::m_uiFormattedSize, size);
            let palette = e.mem.u32(persistent + PERSISTENT_PALETTE);
            if palette != 0 {
                let revision = e.mem.u32(palette + PALETTE_REVISION);
                e.set(this, NiDX9SourceTextureData::m_uiPalRevID, revision);
                if e.mem.u32(data + 0x74) != palette {
                    e.call(POINTER_ASSIGN, &args![data + 0x74, palette]);
                }
            }
            let mut use_data = true;
            if e.get(this, NiDX9SourceTextureData::m_pkD3DTexture) == 0 {
                let format = e.mem.u32(frame + 0xc);
                e.call(0x00e6_82b0, &args![data, persistent, format]);
                use_data = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture) != 0;
            }
            if use_data {
                e.call(0x00e6_8920, &args![data, persistent]);
            }
        } else {
            let revision = e.mem.u32(pixel_data + PIXEL_DATA_REVISION);
            e.set(this, NiDX9SourceTextureData::m_uiSourceRevID, revision);
            let format = e.mem.u32(chosen_at);
            copy_pixel_format(e, data, format);
            let width = e.mem.u32(e.mem.u32(pixel_data + PIXEL_DATA_WIDTHS));
            e.set(this, NiDX9SourceTextureData::m_uiWidth, width);
            let height = e.mem.u32(e.mem.u32(pixel_data + PIXEL_DATA_HEIGHTS));
            e.set(this, NiDX9SourceTextureData::m_uiHeight, height);
            let mipmap = if replaced {
                false
            } else {
                match e.mem.u32(texture + SOURCE_MIPMAPPED) {
                    1 => true,
                    2 => e.mem.u8(MIPMAP_BY_DEFAULT) != 0,
                    _ => false,
                }
            };
            e.set(this, NiDX9SourceTextureData::m_bMipmap, u8::from(mipmap));
            let converted = if replaced {
                pixel_data
            } else {
                let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
                let format = e.mem.u32(chosen_at);
                e.vcall(
                    converter,
                    CONVERTER_CONVERT_SLOT,
                    &args![pixel_data, format, pixel_data, u32::from(mipmap)],
                )
                .u32()
            };
            let mut copy = true;
            if e.get(this, NiDX9SourceTextureData::m_pkD3DTexture) == 0 {
                fn_00e68010(e, this, Ptr::new(converted));
                copy = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture) != 0;
            }
            if copy {
                fn_00e68870(e, this, Ptr::new(converted));
            }
            let levels = e.mem.u32(converted + PIXEL_DATA_LEVELS);
            e.set(this, NiDX9SourceTextureData::m_uiLevels, levels);
            let palette = e.mem.u32(pixel_data + PIXEL_DATA_PALETTE);
            if palette != 0 {
                let revision = e.mem.u32(palette + PALETTE_REVISION);
                let same = e.mem.u32(data + 0x74) == palette;
                e.set(this, NiDX9SourceTextureData::m_uiPalRevID, revision);
                if !same {
                    e.call(POINTER_ASSIGN, &args![data + 0x74, palette]);
                }
            }
            let offsets = e.mem.u32(converted + PIXEL_DATA_OFFSETS);
            let levels = e.mem.u32(converted + PIXEL_DATA_LEVELS);
            let size = e
                .mem
                .u32(offsets.wrapping_add(levels.wrapping_mul(4)))
                .wrapping_mul(e.mem.u32(converted + PIXEL_DATA_FACES));
            e.set(this, NiDX9SourceTextureData::m_uiFormattedSize, size);
            // A reference taken and given up again on the converted data (an `NiPointer`
            // temporary): it deletes itself when nobody else holds it.
            e.call(INTERLOCKED_INCREMENT, &args![converted + REF_COUNT]);
            release(e, converted);
        }
        let owner = e.get(this, NiDX9SourceTextureData::m_pkTexture);
        e.mem.set_u32(owner + TEXTURE_RENDERER_DATA, data);
        face_check_name(e, texture, data);
        if e.mem.u8(CALL_AFTER_CREATE) != 0 {
            e.vcall(texture, SOURCE_AFTER_CREATE_SLOT, &args![]);
        }
        let held = e.mem.u32(frame + 0x18);
        release(e, held);
        this.cast()
    })
}

// ---- n081: the triangle accessors, the convex hull builder and four texture-data methods ----

/// `00e63af0` (unnamed; thiscall on a hull face: a point, the hull, the hull + 4): classifies
/// a face for the point; `00e67660` reads 1 (visible: recurse), 2 (horizon) and 4 (stop).
const HULL_CLASSIFY_FACE: u32 = 0x00e6_3af0;
/// The normalizer's constants (SSE): the squared length that gives scale 0 (`01011d78`),
/// and the Newton step's 3 (`01017718`) and 0.5 (`01016248`).
const HULL_ZERO: u32 = 0x0101_1d78;
const HULL_THREE: u32 = 0x0101_7718;
const HULL_HALF: u32 = 0x0101_6248;
/// The float a removed face's plane distance is set to (`0104da48`; the exe holds a value
/// just under `FLT_MAX`), also the start of `00e67c50`'s best distance.
const HULL_FAR: u32 = 0x0104_da48;
/// `00e67ae0`'s two doubles (`010eb190`, `0106b9e8`).
const HULL_EDGE_SCALE: u32 = 0x010e_b190;
const HULL_PLANE_SCALE: u32 = 0x0106_b9e8;
/// `00e67c50`'s constants: the cosine limit (`0102295c`), the epsilon (`0104da4c`) and the
/// double 0 (`01012060`).
const HULL_COSINE_LIMIT: u32 = 0x0102_295c;
const HULL_EPSILON: u32 = 0x0104_da4c;
const HULL_DOUBLE_ZERO: u32 = 0x0101_2060;
/// The initial tetrahedron's edge table (`010ed0f0`): per edge three words, the vertex's
/// byte offset from +0x20 and the twin's face and edge offsets from +0xf80.
const HULL_EDGE_TABLE: u32 = 0x010e_d0f0;
/// The CRT square root `00ec6040` (argument and result in `ST0`).
const HULL_SQRT: u32 = 0x00ec_6040;

/// The hull builder's layout (offsets as the PC code uses them): vertex records (0x40 bytes,
/// the point at +0, the horizon-edge link at +0x30) from +0x20 (their count at +8), the
/// free-face stack (count +0xc, at most 100 pointers from +0xde0), and face records (0x50
/// bytes: the plane normal at +0, three half-edges of 0x10 bytes at +0x10 (vertex, next,
/// twin, face), the plane distance at +0x40, a mark at +0x44) from +0xf70 (count +0x10).
const HULL_FREE_COUNT: u32 = 0x0c;
const HULL_FACE_COUNT: u32 = 0x10;
const HULL_FREE_FACES: u32 = 0xde0;
const HULL_FACES: u32 = 0xf70;
const FACE_DISTANCE: u32 = 0x40;
const FACE_MARK: u32 = 0x44;

/// `NiPersistentSrcTextureRendererData` fields `00e683d0` reads besides the ones above (PC
/// offsets): the pixel-format kind (+0x18), the pixels (+0x60) and the pixel stride (+0x78).
const PERSISTENT_FORMAT_KIND: u32 = 0x18;
const PERSISTENT_PIXELS: u32 = 0x60;
const PERSISTENT_STRIDE: u32 = 0x78;
/// The messages of `00e682b0`, `00e683d0` and `00e68920`.
const CREATE_SURF_PERSISTENT_FAILED: u32 = 0x010e_d260;
const PERSISTENT_GET_DESC_FAILED: u32 = 0x010e_d328;
const PERSISTENT_LOCK_FAILED: u32 = 0x010e_d2d0;
const PERSISTENT_GET_SURFACE_LEVEL_FAILED: u32 = 0x010e_d488;

type Vector4 = crate::sse::Xmm;

/// Four lanes of the 16-byte vector at `addr`.
fn hull_load(e: &Engine, addr: u32) -> Vector4 {
    std::array::from_fn(|lane| e.mem.f32(addr.wrapping_add(4 * lane as u32)))
}

/// Stores four lanes at `addr` (`MOVAPS`, bits kept).
fn hull_store(e: &mut Engine, addr: u32, v: Vector4) {
    for (lane, x) in v.iter().enumerate() {
        e.mem
            .set_u32(addr.wrapping_add(4 * lane as u32), x.to_bits());
    }
}

/// `SHUFPS v,v,imm` with one register as both operands.
fn hull_shuffle(v: Vector4, imm: u8) -> Vector4 {
    std::array::from_fn(|lane| v[usize::from((imm >> (2 * lane)) & 3)])
}

/// The inlined three-lane sum `v[2] + (v[1] + v[0])` (`ADDSS`).
fn hull_sum3(v: Vector4) -> f32 {
    use crate::sse::add;
    add(v[2], add(v[1], v[0]))
}

/// The inlined cross product `b.zxy * a.yzx - b.yzx * a.zxy` (`SHUFPS` 0xd2 / 0xc9).
fn hull_cross(a: Vector4, b: Vector4) -> Vector4 {
    use crate::sse::{mulps, subps};
    subps(
        mulps(hull_shuffle(b, 0xd2), hull_shuffle(a, 0xc9)),
        mulps(hull_shuffle(b, 0xc9), hull_shuffle(a, 0xd2)),
    )
}

/// The inlined normalizing scale of `v` (only lane 0 of the exe's register matters): the
/// `RSQRTSS` estimate `r` of `l² = sum3(v * v)` refined once, `0.5 * r * (3 - l² r r)`, and
/// 0 when `l²` equals the constant at `01011d78` (`CMPEQSS`/`ANDNPS`). The constants are
/// read in the exe's order: zero first.
fn hull_unit_scale(e: &Engine, length2: f32, zero: f32) -> f32 {
    use crate::sse::{compare, mul, sub};
    let estimate = crate::x87::rsqrt_estimate(length2);
    let scaled = mul(mul(length2, estimate), estimate);
    let three = e.mem.f32(HULL_THREE);
    let half = e.mem.f32(HULL_HALF);
    let step = mul(mul(half, estimate), sub(three, scaled));
    if compare(length2, zero, 0) {
        0.0
    } else {
        step
    }
}

/// `v` scaled to unit length as the exe does it (`MULPS` broadcast first).
fn hull_normalize(e: &Engine, v: Vector4) -> Vector4 {
    use crate::sse::mulps;
    let zero = e.mem.f32(HULL_ZERO);
    let scale = hull_unit_scale(e, hull_sum3(mulps(v, v)), zero);
    mulps([scale; 4], v)
}

// Translated from 00e673f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the triangle count of a geometry: the index count (+0x10, or
/// +0x1c when that is 0) divided by 3 (signed, toward zero) when the signed type byte at +8
/// is 1, else 0.
pub fn fn_00e673f0(e: &mut Engine, this: Ptr) -> i32 {
    let this = this.addr();
    let mut count = e.mem.i32(this + 0x10);
    if count == 0 {
        count = e.mem.i32(this + 0x1c);
    }
    if e.mem.i8(this + 8) == 1 {
        count / 3
    } else {
        0
    }
}

// Translated from 00e67420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the three vertex indices of triangle `triangle`. With a
/// 16-bit index count (+0x10) the indices are words at +0xc, else with a 32-bit count
/// (+0x1c) words of 4 bytes at +0x18; the signed type byte at +8 picks a list (1:
/// `3 * triangle`) or a strip (2: `triangle`) start. Answers true; any other case stores
/// -1 in all three and answers false.
pub fn fn_00e67420(
    e: &mut Engine,
    this: Ptr,
    triangle: u32,
    first: Ptr,
    second: Ptr,
    third: Ptr,
) -> bool {
    let this = this.addr();
    let outs = [first.addr(), second.addr(), third.addr()];
    let kind = e.mem.i8(this + 8);
    let (array_at, size, start) = if e.mem.u32(this + 0x10) != 0 {
        match kind {
            1 => (0x0c, 2u32, triangle.wrapping_mul(3)),
            2 => (0x0c, 2, triangle),
            _ => (0, 0, 0),
        }
    } else if e.mem.u32(this + 0x1c) != 0 {
        match kind {
            1 => (0x18, 4, triangle.wrapping_mul(3)),
            2 => (0x18, 4, triangle),
            _ => (0, 0, 0),
        }
    } else {
        (0, 0, 0)
    };
    if size == 0 {
        for out in outs {
            e.mem.set_u32(out, 0xffff_ffff);
        }
        return false;
    }
    for (k, out) in outs.iter().enumerate() {
        let base = e.mem.u32(this + array_at);
        let at = base.wrapping_add(start.wrapping_add(k as u32).wrapping_mul(size));
        let index = if size == 2 {
            u32::from(e.mem.u16(at))
        } else {
            e.mem.u32(at)
        };
        e.mem.set_u32(*out, index);
    }
    true
}

// Translated from 00e67570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a hull face's plane from its three half-edges' points
/// (vertex pointers at +0x10, +0x20, +0x30): the normal `cross(p0 - p1, p1 - p2)` (SSE),
/// stored, then normalized (see `hull_normalize`) and stored again at +0; the distance
/// `normal · p0` (SSE sum, through a float local and the x87 stack) at +0x40.
pub fn fn_00e67570(e: &mut Engine, this: Ptr) {
    use crate::sse::{mulps, subps};
    let face = this.addr();
    let p1 = hull_load(e, e.mem.u32(face + 0x20));
    let p0_at = e.mem.u32(face + 0x10);
    let p0 = hull_load(e, p0_at);
    let p2 = hull_load(e, e.mem.u32(face + 0x30));
    let normal = hull_cross(subps(p0, p1), subps(p1, p2));
    let unit = hull_normalize(e, normal);
    hull_store(e, face, normal);
    hull_store(e, face, unit);
    let p0 = hull_load(e, p0_at);
    let distance = hull_sum3(mulps(unit, p0));
    e.mem
        .set_u32(face + FACE_DISTANCE, x87_quiet(distance).to_bits());
}

// Translated from 00e67660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (recursive): removes `face` and the faces it reaches that
/// `point` sees. For each of its three edges the twin's face, when unmarked, is classified
/// (`00e63af0` with the point, the hull and the hull + 4; the answer becomes its mark):
/// 1 recurses into it (a recursion answering 2 stops the loop), 4 stops with 2; a face
/// marked 2 (now or before) is on the horizon: the twin edge is linked from its vertex
/// (+0x30), `*count` is incremented and `*edge` set to it. Then the face's distance becomes
/// the float at `0104da48` and it goes on the free stack; a full stack (100) answers 2.
/// Otherwise answers the last recursion's result (0 without one).
pub fn fn_00e67660(e: &mut Engine, this: Ptr, face: Ptr, point: Ptr, count: Ptr, edge: Ptr) -> u32 {
    let hull = this.addr();
    let face_at = face.addr();
    let mut result = 0u32;
    for k in 0..3u32 {
        let twin = e.mem.u32(face_at + 0x18 + 0x10 * k);
        let other = e.mem.u32(twin + 0xc);
        let mut mark = e.mem.u32(other + FACE_MARK);
        if mark == 0 {
            mark = e
                .call(HULL_CLASSIFY_FACE, &args![other, point, hull, hull + 4])
                .u32();
            e.mem.set_u32(other + FACE_MARK, mark);
            if mark == 1 {
                result = fn_00e67660(e, this, Ptr::new(other), point, count, edge);
                if result == 2 {
                    break;
                }
                continue;
            }
            if mark == 4 {
                result = 2;
                break;
            }
        }
        if mark == 2 {
            let vertex = e.mem.u32(twin);
            e.mem.set_u32(vertex + 0x30, twin);
            let n = e.mem.u32(count.addr());
            e.mem.set_u32(count.addr(), n.wrapping_add(1));
            e.mem.set_u32(edge.addr(), twin);
        }
    }
    let far = x87_quiet_bits(e.mem.u32(HULL_FAR));
    e.mem.set_u32(face_at + FACE_DISTANCE, far);
    let free = e.mem.i32(hull + HULL_FREE_COUNT);
    if free >= 100 {
        return 2;
    }
    e.mem
        .set_u32(hull + HULL_FREE_FACES + 4 * free as u32, face_at);
    e.mem.set_i32(hull + HULL_FREE_COUNT, free + 1);
    result
}

// Translated from 00e67740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds `count` new faces from the horizon, starting at
/// `start` and joining each horizon edge to `point`. The faces come from the free stack
/// (top first) and then from new records (+0x10 grows); each gets its three half-edges
/// (`point` on the third, twins with the horizon edge and the neighbours in the ring),
/// its normal (`cross(vB - vA, vA - point)`, normalized) and distance. A zero squared
/// normal length (x87 compare) or a missing horizon edge answers 2. The next horizon edge
/// is the link at +0x30 of the edge's end vertex (cleared). Then all marks are cleared
/// (`00e67fd0`'s loops) and the answer is 2 unless the walk came back to `start` (0).
pub fn fn_00e67740(e: &mut Engine, this: Ptr, count: i32, start: Ptr, point: Ptr) -> u32 {
    use crate::sse::{mulps, subps};
    let hull = this.addr();
    let point = point.addr();
    e.with_stack(0x1c0, |e, frame| {
        let faces = frame.addr() + 0x30;
        // Take faces from the free stack.
        let mut top = e.mem.i32(hull + HULL_FREE_COUNT).wrapping_sub(1);
        let mut taken = 0u32;
        let mut remaining = count;
        if count != 0 {
            let mut slot = (hull + HULL_FREE_FACES).wrapping_add((top as u32).wrapping_mul(4));
            loop {
                if top < 0 {
                    break;
                }
                let face = e.mem.u32(slot);
                let free = e.mem.u32(hull + HULL_FREE_COUNT);
                e.mem.set_u32(hull + HULL_FREE_COUNT, free.wrapping_sub(1));
                e.mem.set_u32(faces + 4 * taken, face);
                remaining -= 1;
                taken += 1;
                top -= 1;
                slot = slot.wrapping_sub(4);
                if remaining == 0 {
                    break;
                }
            }
        }
        if remaining > 0 {
            let records = e.mem.i32(hull + HULL_FACE_COUNT);
            let grown = records.wrapping_add(remaining);
            let mut offset = (records as u32).wrapping_mul(0x50);
            for _ in 0..remaining {
                e.mem.set_u32(faces + 4 * taken, hull + HULL_FACES + offset);
                offset = offset.wrapping_add(0x50);
                taken += 1;
            }
            e.mem.set_i32(hull + HULL_FACE_COUNT, grown);
        }
        // Build the ring.
        let mut edge = start.addr();
        if count > 0 {
            let zero = e.mem.f32(HULL_ZERO);
            let mut index = 0i32;
            loop {
                if edge == 0 {
                    return 2;
                }
                let face = e.mem.u32(faces + 4 * index as u32);
                let next_index = index + 1;
                let next = if next_index < count {
                    e.mem.u32(faces + 4 * next_index as u32)
                } else {
                    e.mem.u32(faces)
                };
                e.mem.set_u32(face + 0x1c, face);
                e.mem.set_u32(face + 0x2c, face);
                e.mem.set_u32(face + 0x3c, face);
                e.mem.set_u32(face + 0x14, face + 0x20);
                e.mem.set_u32(face + 0x24, face + 0x30);
                e.mem.set_u32(face + 0x34, face + 0x10);
                let end_vertex = e.mem.u32(e.mem.u32(edge + 4));
                let start_vertex = e.mem.u32(edge);
                e.mem.set_u32(face + 0x10, end_vertex);
                e.mem.set_u32(face + 0x30, point);
                e.mem.set_u32(face + 0x20, start_vertex);
                e.mem.set_u32(edge + 8, face + 0x10);
                e.mem.set_u32(face + 0x18, edge);
                e.mem.set_u32(next + 0x28, face + 0x30);
                e.mem.set_u32(face + 0x38, next + 0x20);
                let b = hull_load(e, end_vertex);
                let a = hull_load(e, start_vertex);
                let p = hull_load(e, point);
                let normal = hull_cross(subps(b, a), subps(a, p));
                let length2 = hull_sum3(mulps(normal, normal));
                if f64::from(length2) == 0.0 {
                    return 2;
                }
                let scale = hull_unit_scale(e, length2, zero);
                let unit = mulps([scale; 4], normal);
                let distance = hull_sum3(mulps(b, unit));
                e.mem
                    .set_u32(face + FACE_DISTANCE, x87_quiet(distance).to_bits());
                hull_store(e, face, unit);
                let vertex = e.mem.u32(e.mem.u32(edge + 4));
                edge = e.mem.u32(vertex + 0x30);
                e.mem.set_u32(vertex + 0x30, 0);
                index = next_index;
                if index >= count {
                    break;
                }
            }
        }
        fn_00e67fd0(e, this);
        if start.addr() != edge {
            2
        } else {
            0
        }
    })
}

// Translated from 00e679d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the initial tetrahedron: four face records (+0x10 = 4),
/// each with its half-edge ring, mark 0, and per edge its face, the vertex at
/// `this + 0x20 + table[0]` and the twin at `this + 0xf80 + table[1] + table[2]` from the
/// table at `010ed0f0`. The fourth face's plane is computed (`00e67570`); when vertex 0
/// (+0x20) lies in front of it (`(v0 - v1) · normal > 0`, x87 compare) vertices 0 and 1
/// (+0x20, +0x60) are swapped on six edges; then all four planes are computed.
pub fn fn_00e679d0(e: &mut Engine, this: Ptr) {
    use crate::sse::{mulps, subps};
    let hull = this.addr();
    e.mem.set_u32(hull + HULL_FACE_COUNT, 4);
    let mut table = HULL_EDGE_TABLE;
    for face_index in 0..4u32 {
        let face = hull + HULL_FACES + 0x50 * face_index;
        e.mem.set_u32(face + 0x14, face + 0x20);
        e.mem.set_u32(face + 0x24, face + 0x30);
        e.mem.set_u32(face + FACE_MARK, 0);
        e.mem.set_u32(face + 0x34, face + 0x10);
        for k in 0..3u32 {
            let edge = face + 0x10 + 0x10 * k;
            e.mem.set_u32(edge + 0xc, face);
            let vertex = e.mem.u32(table).wrapping_add(hull).wrapping_add(0x20);
            e.mem.set_u32(edge, vertex);
            let twin = e
                .mem
                .u32(table + 8)
                .wrapping_add(e.mem.u32(table + 4))
                .wrapping_add(hull)
                .wrapping_add(0xf80);
            e.mem.set_u32(edge + 8, twin);
            table += 0xc;
        }
    }
    let last = hull + 0x1060;
    fn_00e67570(e, Ptr::new(last));
    let v1 = hull_load(e, hull + 0x60);
    let v0 = hull_load(e, hull + 0x20);
    let normal = hull_load(e, last);
    let side = hull_sum3(mulps(subps(v0, v1), normal));
    if 0.0 < f64::from(side) {
        let (first, second) = (hull + 0x60, hull + 0x20);
        e.mem.set_u32(hull + 0x1020, first);
        e.mem.set_u32(hull + 0xfd0, first);
        e.mem.set_u32(hull + 0xf80, first);
        e.mem.set_u32(hull + 0x1070, second);
        e.mem.set_u32(hull + 0xfe0, second);
        e.mem.set_u32(hull + 0xfa0, second);
    }
    for face_index in 0..4u32 {
        fn_00e67570(e, Ptr::new(hull + HULL_FACES + 0x50 * face_index));
    }
}

// Translated from 00e67ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks from `face` across edges to the face to work on. Marks
/// are cleared first (`00e67fd0`); each face reached is marked 3. For its edges other than
/// the one it was entered by, with `d = edge` and `c = cross(d, normal)` (SSE), an edge
/// qualifies when `l² * scale > (c · v0) * |c · v0|` (x87, the doubles at `010eb190` and
/// `0106b9e8`): the walk crosses it when the start face's distance is not above 0, or its
/// distance times the second double is not above the neighbour's (NaN crossing). It stops
/// on a face without a crossing (answered) or on reaching a face already marked 3 (that
/// face is answered).
pub fn fn_00e67ae0(e: &mut Engine, this: Ptr, face: Ptr) -> Ptr {
    use crate::sse::{mulps, subps};
    fn_00e67fd0(e, this);
    let edge_scale = x87_load64(e.mem.u64(HULL_EDGE_SCALE));
    let plane_scale = x87_load64(e.mem.u64(HULL_PLANE_SCALE));
    let start = face.addr();
    let mut current = start;
    let mut entered = 0u32;
    loop {
        e.mem.set_u32(current + FACE_MARK, 3);
        let mut crossing = None;
        for k in 0..3u32 {
            let edge = current + 0x10 + 0x10 * k;
            if edge == entered {
                continue;
            }
            let v0 = hull_load(e, e.mem.u32(edge));
            let v1 = hull_load(e, e.mem.u32(e.mem.u32(edge + 4)));
            let d = subps(v1, v0);
            let length2 = hull_sum3(mulps(d, d));
            let normal = hull_load(e, current);
            let c = subps(
                mulps(hull_shuffle(d, 0xc9), hull_shuffle(normal, 0xd2)),
                mulps(hull_shuffle(d, 0xd2), hull_shuffle(normal, 0xc9)),
            );
            let side = x87_load32(hull_sum3(mulps(c, v0)).to_bits());
            let magnitude = x87_store32(side.abs());
            let weighted = x87_mul(side, x87_m32(magnitude));
            let limit = x87_load32(x87_store32(x87_mul(
                x87_load32(length2.to_bits()),
                edge_scale,
            )));
            if !(limit > weighted) {
                continue;
            }
            let twin = e.mem.u32(edge + 8);
            let neighbour = e.mem.u32(twin + 0xc);
            let start_distance = e.mem.u32(start + FACE_DISTANCE);
            let crosses = if !(0.0 < x87_m32(start_distance)) {
                true
            } else {
                let theirs = x87_load32(e.mem.u32(neighbour + FACE_DISTANCE));
                let ours = x87_mul(x87_load32(start_distance), plane_scale);
                !(ours > theirs)
            };
            if crosses {
                crossing = Some((k, neighbour));
                break;
            }
        }
        let Some((k, neighbour)) = crossing else {
            return Ptr::new(current);
        };
        let done = e.mem.u32(neighbour + FACE_MARK) == 3;
        entered = e.mem.u32(current + 0x18 + 0x10 * k);
        current = neighbour;
        if done {
            return Ptr::new(current);
        }
    }
}

// Translated from 00e67c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words; nothing references it): finds the closest
/// pair of planes of two hulls and the direction between them. For each face of `first`
/// (last to first) whose squared distance does not exceed the best, and each face of
/// `second` from its last down to index 1 likewise, with `c` the normals' cosine (SSE) not
/// below the limit at `0102295c`: with `a`, `b` the two distances, `p = a - c b` and
/// `q = b - c a` (x87, stored as floats) the pair's squared distance and weights are, when
/// both exceed the epsilon at `0104da4c`, from the 2x2 solve (`q / p` or `p / q`, whichever
/// is below 1), else `a²` with (1, 0) when `b > 0`, else `b²` with (0, 1) when `a` exceeds
/// the epsilon, else `-c²` with (0.5, 0.5). A smaller value replaces the best, its weights
/// and the two faces (`*first_face`, `*second_face`). At the end `direction` gets the
/// normalized `wb * nB + wa * nA` (SSE) and its fourth lane the CRT square root of the best
/// (0 when negative).
pub fn fn_00e67c50(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    direction: Ptr,
    first_face: Ptr,
    second_face: Ptr,
) {
    use crate::sse::{addps, mulps};
    let store = |value: f64| x87_load32(x87_store32(value));
    let mut best = store(x87_load32(e.mem.u32(HULL_FAR)));
    let mut weight_a = 0.0f64;
    let mut weight_b = 0.0f64;
    let one = 1.0f64;
    let zero = 0.0f64;
    let first = first.addr();
    let second = second.addr();
    let mut index_a = e.mem.i32(first + HULL_FACE_COUNT).wrapping_sub(1);
    while index_a >= 0 {
        let face_a = first + HULL_FACES + 0x50 * index_a as u32;
        let distance_a = x87_load32(e.mem.u32(face_a + FACE_DISTANCE));
        let square_a = x87_mul(distance_a, distance_a);
        let count_b = e.mem.i32(second + HULL_FACE_COUNT).wrapping_sub(1);
        if !(best < square_a) && count_b >= 1 {
            let mut index_b = count_b;
            while index_b != 0 {
                let face_b = second + HULL_FACES + 0x50 * index_b as u32;
                let distance_b = x87_load32(e.mem.u32(face_b + FACE_DISTANCE));
                let square_b = x87_mul(distance_b, distance_b);
                if !(best < square_b) {
                    let limit = x87_load32(e.mem.u32(HULL_COSINE_LIMIT));
                    let normal_a = hull_load(e, face_a);
                    let normal_b = hull_load(e, face_b);
                    let c = x87_load32(hull_sum3(mulps(normal_b, normal_a)).to_bits());
                    if !(c < limit) {
                        let a = store(x87_load32(e.mem.u32(face_a + FACE_DISTANCE)));
                        let b = store(x87_load32(e.mem.u32(face_b + FACE_DISTANCE)));
                        let p = store(x87_sub(a, x87_mul(c, b)));
                        let q = store(x87_sub(b, x87_mul(c, a)));
                        let epsilon = x87_load32(e.mem.u32(HULL_EPSILON));
                        let (value, wa, wb) = if p > epsilon {
                            if q > epsilon {
                                let (x, w, u, v) = if q < p {
                                    let r = store(x87_div(q, p));
                                    let denominator = x87_add(x87_mul(c, r), one);
                                    let u = store(x87_div(one, denominator));
                                    let v = store(x87_mul(u, r));
                                    let w = store(x87_div(a, denominator));
                                    (store(x87_mul(r, w)), w, u, v)
                                } else {
                                    let r = store(x87_div(p, q));
                                    let denominator = x87_add(x87_mul(c, r), one);
                                    let v = store(x87_div(one, denominator));
                                    let u = store(x87_mul(r, v));
                                    let x = store(x87_div(b, denominator));
                                    (x, store(x87_mul(r, x)), u, v)
                                };
                                let y = store(x87_add(x87_mul(c, x), w));
                                let c2 = x87_mul(c, c);
                                let spread = x87_sub(one, c2);
                                let xs = x87_mul(spread, x);
                                let xx = x87_mul(x, xs);
                                let yy = x87_mul(y, y);
                                (store(x87_add(yy, xx)), u, v)
                            } else if b > zero {
                                (store(x87_mul(a, a)), one, store(zero))
                            } else {
                                let half = x87_load32(e.mem.u32(HULL_HALF));
                                (store(x87_mul(c, -c)), store(half), store(half))
                            }
                        } else if epsilon < a {
                            (store(x87_mul(b, b)), store(zero), one)
                        } else {
                            let half = x87_load32(e.mem.u32(HULL_HALF));
                            (store(x87_mul(c, -c)), store(half), store(half))
                        };
                        if best > value {
                            weight_a = wa;
                            e.mem.set_u32(first_face.addr(), face_a);
                            weight_b = wb;
                            e.mem.set_u32(second_face.addr(), face_b);
                            best = value;
                        }
                    }
                }
                index_b -= 1;
            }
        }
        index_a -= 1;
    }
    let face_a = e.mem.u32(first_face.addr());
    let scaled_a = mulps(
        [f32::from_bits(x87_store32(weight_a)); 4],
        hull_load(e, face_a),
    );
    hull_store(e, direction.addr(), scaled_a);
    let face_b = e.mem.u32(second_face.addr());
    let zero_bits = e.mem.f32(HULL_ZERO);
    let sum = addps(
        mulps(
            [f32::from_bits(x87_store32(weight_b)); 4],
            hull_load(e, face_b),
        ),
        scaled_a,
    );
    let scale = hull_unit_scale(e, hull_sum3(mulps(sum, sum)), zero_bits);
    hull_store(e, direction.addr(), mulps([scale; 4], sum));
    let floor = f64::from_bits(e.mem.u64(HULL_DOUBLE_ZERO));
    let clamped = if best < floor { zero } else { best };
    let root = e
        .call(HULL_SQRT, &args![x87_load32(x87_store32(clamped))])
        .f64();
    let root = x87_store32(x87_load32(x87_store32(root)));
    e.mem.set_u32(direction.addr() + 0xc, root);
}

// Translated from 00e682b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the persistent-data counterpart of `00e68010`; one call site
/// in `00e68ef0`): creates the Direct3D texture for persistent data in the Direct3D format
/// `format`. The level count is the data's (+0x74) when both sides are powers of two or the
/// renderer allows other sizes (capability bit 8), else 1; the rest is `00e68010`'s: mipmap
/// flag, skipped levels and halved sides, `CreateTexture` (dynamic or managed). Success with
/// a texture stores it at +0x64 and answers true; otherwise the message `010ed260` is
/// printed with the format and the error, +0x64 cleared, false.
pub fn fn_00e682b0(
    e: &mut Engine,
    this: Ptr<NiDX9SourceTextureData>,
    persistent: Ptr,
    format: u32,
) -> bool {
    let width = e.get(this, NiDX9SourceTextureData::m_uiWidth);
    let height = e.get(this, NiDX9SourceTextureData::m_uiHeight);
    let mut full_chain = is_power_of_two(width) && is_power_of_two(height);
    if !full_chain {
        let renderer = e.get(this, NiDX9SourceTextureData::m_pkRenderer);
        let caps = e.vcall(renderer, RENDERER_CAPS_SLOT, &args![]).u32();
        full_chain = caps & CAPS_NONPOW2 != 0;
    }
    let levels = if full_chain {
        e.mem.u32(persistent.addr() + PERSISTENT_LEVELS)
    } else {
        1
    };
    e.set(this, NiDX9SourceTextureData::m_uiLevels, levels);
    e.set(
        this,
        NiDX9SourceTextureData::m_bMipmap,
        u8::from(levels != 1),
    );
    let mut skipped = e.mem.u32(SKIP_LEVELS);
    if skipped > levels.wrapping_sub(1) {
        skipped = levels.wrapping_sub(1);
    }
    if skipped != 0 {
        let mut width = e.get(this, NiDX9SourceTextureData::m_uiWidth);
        let mut height = e.get(this, NiDX9SourceTextureData::m_uiHeight);
        for _ in 0..skipped {
            if width & 1 != 0 {
                width = width.wrapping_add(1);
            }
            width >>= 1;
            if height & 1 != 0 {
                height = height.wrapping_add(1);
            }
            height >>= 1;
        }
        e.set(this, NiDX9SourceTextureData::m_uiWidth, width);
        e.set(this, NiDX9SourceTextureData::m_uiHeight, height);
    }
    e.set(this, NiDX9SourceTextureData::m_uiLevelsSkipped, skipped);
    e.with_stack(4, |e, texture_out| {
        let texture_out = texture_out.addr();
        let renderer = e.get(this, NiDX9SourceTextureData::m_pkRenderer);
        let device = e.mem.u32(renderer + RENDERER_DEVICE);
        let dynamic = e.mem.u8(DYNAMIC_TEXTURES);
        let pool = u32::from(dynamic == 0);
        let usage = if dynamic != 0 { D3DUSAGE_DYNAMIC } else { 0 };
        let height = e.get(this, NiDX9SourceTextureData::m_uiHeight);
        e.mem.set_u32(texture_out, 0);
        let width = e.get(this, NiDX9SourceTextureData::m_uiWidth);
        let result = e
            .vcall(
                device,
                D3D_CREATE_TEXTURE,
                &args![
                    width,
                    height,
                    levels.wrapping_sub(skipped),
                    usage,
                    format,
                    pool,
                    texture_out,
                    0u32
                ],
            )
            .u32();
        let texture = e.mem.u32(texture_out);
        if (result as i32) >= 0 && texture != 0 {
            e.set(this, NiDX9SourceTextureData::m_pkD3DTexture, texture);
            return true;
        }
        print_error(e, CREATE_SURF_PERSISTENT_FAILED, &[format], result);
        e.set(this, NiDX9SourceTextureData::m_pkD3DTexture, 0);
        false
    })
}

// Translated from 00e683d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words: persistent data, level, surface, face; the
/// persistent-data counterpart of `00e68150`): reads the surface's description and locks it
/// (failures print `010ed328` / `010ed2d0` with the error and return), then copies the level
/// into it with `00e68150`'s rules (row = stride (+0x78) * width, height at most the
/// surface's, compressed kinds 4..6 by block rows), from
/// `offsets[levels] * face + offsets[level]` into the pixels (+0x60); the surface is
/// unlocked.
pub fn fn_00e683d0(e: &mut Engine, persistent: Ptr, level: u32, surface: Ptr, face: u32) {
    let data = persistent.addr();
    let surface = surface.addr();
    e.with_stack(0x2c, |e, frame| {
        let frame = frame.addr();
        let locked = frame + 4;
        let desc = frame + 0xc;
        let result = e.vcall(surface, D3D_SURFACE_GET_DESC, &args![desc]).u32();
        if (result as i32) < 0 {
            print_error(e, PERSISTENT_GET_DESC_FAILED, &[], result);
            return;
        }
        let result = e
            .vcall(surface, D3D_SURFACE_LOCK_RECT, &args![locked, 0u32, 0u32])
            .u32();
        if (result as i32) < 0 {
            print_error(e, PERSISTENT_LOCK_FAILED, &[], result);
            return;
        }
        let widths = e.mem.u32(data + PERSISTENT_WIDTHS);
        let mut pitch = e.mem.u32(locked);
        let width = e.mem.u32(widths.wrapping_add(4 * level));
        let heights = e.mem.u32(data + PERSISTENT_HEIGHTS);
        let mut height = e.mem.u32(heights.wrapping_add(4 * level));
        let surface_height = e.mem.u32(desc + 0x1c);
        let mut row = e.mem.u32(data + PERSISTENT_STRIDE).wrapping_mul(width);
        if height > surface_height {
            height = surface_height;
        }
        let kind = e.mem.i32(data + PERSISTENT_FORMAT_KIND);
        if (FORMAT_DXT1..=FORMAT_DXT5).contains(&kind) {
            pitch >>= 2;
            row = width.wrapping_add(3);
            if e.mem.u32(desc) == FOURCC_DXT1 {
                row = (row >> 1) & 0x07ff_fffe;
            } else {
                row &= 0x0fff_fffc;
            }
            height = height.wrapping_add(3) & 0x0fff_fffc;
        }
        let offsets = e.mem.u32(data + PERSISTENT_OFFSETS);
        let levels = e.mem.u32(data + PERSISTENT_LEVELS);
        let face_size = e.mem.u32(offsets.wrapping_add(4 * levels));
        let source = face_size
            .wrapping_mul(face)
            .wrapping_add(e.mem.u32(offsets.wrapping_add(4 * level)))
            .wrapping_add(e.mem.u32(data + PERSISTENT_PIXELS));
        let bits = e.mem.u32(locked + 4);
        if row == pitch {
            e.call(MEMCPY, &args![bits, source, height.wrapping_mul(row)]);
        } else {
            let copy = if row > pitch { pitch } else { row };
            e.mem.set_u32(frame, copy);
            let (mut to, mut from) = (bits, source);
            for _ in 0..height {
                let copy = e.mem.u32(frame);
                e.call(MEMCPY, &args![to, from, copy]);
                to = to.wrapping_add(e.mem.u32(locked));
                from = from.wrapping_add(row);
            }
        }
        e.vcall(surface, D3D_SURFACE_UNLOCK_RECT, &args![]);
    });
}

// Translated from 00e687e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word of six classes): stores the word at +0x64 (the
/// Direct3D texture slot of `NiDX9SourceTextureData`).
pub fn fn_00e687e0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x64, value);
}

// Translated from 00e68820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word): answers the word at +0x74
/// (`NiDX9SourceTextureData::m_spPalette`).
pub fn fn_00e68820(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x74)
}

// Translated from 00e68920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the persistent-data counterpart of `00e68870`): nothing
/// without a Direct3D texture or for a cube or volume texture; otherwise each level from
/// the skipped ones on (count `levels - skipped`, re-read) gets its surface
/// (`GetSurfaceLevel`), the persistent data copied in (`00e683d0`, face 0) and the surface
/// released; a failed `GetSurfaceLevel` prints `010ed488` with the level and the error and
/// stops.
pub fn fn_00e68920(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>, persistent: Ptr) {
    let texture = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture);
    if texture == 0 {
        return;
    }
    if e.vcall(texture, D3D_GET_TYPE, &args![]).u32() == D3DRTYPE_CUBETEXTURE {
        return;
    }
    let texture = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture);
    if e.vcall(texture, D3D_GET_TYPE, &args![]).u32() == D3DRTYPE_VOLUMETEXTURE {
        return;
    }
    let levels = e.get(this, NiDX9SourceTextureData::m_uiLevels);
    let texture = e.get(this, NiDX9SourceTextureData::m_pkD3DTexture);
    let skipped = e.get(this, NiDX9SourceTextureData::m_uiLevelsSkipped);
    if levels.wrapping_sub(skipped) == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let mut level = 0u32;
        loop {
            let result = e
                .vcall(texture, D3D_GET_SURFACE_LEVEL, &args![level, slot])
                .u32();
            if (result as i32) < 0 {
                print_error(e, PERSISTENT_GET_SURFACE_LEVEL_FAILED, &[level], result);
                return;
            }
            let surface = e.mem.u32(slot);
            let skipped = e.get(this, NiDX9SourceTextureData::m_uiLevelsSkipped);
            fn_00e683d0(
                e,
                persistent,
                skipped.wrapping_add(level),
                Ptr::new(surface),
                0,
            );
            let surface = e.mem.u32(slot);
            e.vcall(surface, D3D_RELEASE, &args![]);
            let levels = e.get(this, NiDX9SourceTextureData::m_uiLevels);
            let skipped = e.get(this, NiDX9SourceTextureData::m_uiLevelsSkipped);
            level += 1;
            if level >= levels.wrapping_sub(skipped) {
                return;
            }
        }
    });
}

// Translated from 00e689d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word of `NiDX9SourceTextureData`): refreshes the
/// texture from its source pixel data. Nothing for replacement data (+0x6c) or a texture
/// without pixel data (`NiSourceTexture` +0x3c). A palette on the pixel data (+0x4c) whose
/// revision (+0x10) or pointer differs from the stored ones (+0x80, +0x74: assigned through
/// `00a5f860`) marks a change; with no change and the same pixel revision (+0x68 against
/// +0x7c) nothing more happens. Otherwise the image converter (`00a76640`, vtable +0x14 with
/// the pixel data, the pixel format at +0x14, the pixel data and the mipmap byte) gives new
/// pixel data, the revision is stored, the data is copied to the surface (`00e68870`) while
/// the texture (+8) is set, and the new data's reference is taken and released (deleted at
/// zero through vtable +4).
pub fn fn_00e689d0(e: &mut Engine, this: Ptr<NiDX9SourceTextureData>) {
    let data = this.addr();
    if e.get(this, NiDX9SourceTextureData::m_bReplacementData) != 0 {
        return;
    }
    let texture = e.get(this, NiDX9SourceTextureData::m_pkTexture);
    let pixel_data = e.mem.u32(texture + SOURCE_PIXEL_DATA);
    if pixel_data == 0 {
        return;
    }
    let palette = e.mem.u32(pixel_data + PIXEL_DATA_PALETTE);
    let mut changed = false;
    if palette != 0 {
        let revision = e.mem.u32(palette + PALETTE_REVISION);
        if e.get(this, NiDX9SourceTextureData::m_uiPalRevID) != revision {
            changed = true;
        }
        let stored = e.get(this, NiDX9SourceTextureData::m_spPalette);
        e.set(this, NiDX9SourceTextureData::m_uiPalRevID, revision);
        if stored != palette {
            e.call(POINTER_ASSIGN, &args![data + 0x74, palette]);
            changed = true;
        }
    }
    let revision = e.mem.u32(pixel_data + PIXEL_DATA_REVISION);
    if revision == e.get(this, NiDX9SourceTextureData::m_uiSourceRevID) && !changed {
        return;
    }
    let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
    let mipmap = u32::from(e.get(this, NiDX9SourceTextureData::m_bMipmap));
    let converted = e
        .vcall(
            converter,
            CONVERTER_CONVERT_SLOT,
            &args![pixel_data, data + PIXEL_FORMAT, pixel_data, mipmap],
        )
        .u32();
    let texture_set = e.get(this, NiDX9SourceTextureData::m_pkTexture) != 0;
    let revision = e.mem.u32(pixel_data + PIXEL_DATA_REVISION);
    e.set(this, NiDX9SourceTextureData::m_uiSourceRevID, revision);
    if texture_set {
        fn_00e68870(e, this, Ptr::new(converted));
    }
    if converted != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![converted + REF_COUNT]);
        release(e, converted);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e67fd0, fn_00e67fd0(Ptr)),
        entry!(0x00e68810, fn_00e68810(Ptr) -> Ptr),
        entry!(0x00e687f0, fn_00e687f0(Ptr<NiDX9SourceTextureData>) -> u32),
        entry!(0x00e68800, fn_00e68800(Ptr<NiDX9SourceTextureData>) -> u32),
        entry!(
            0x00e687a0,
            fn_00e687a0(Ptr<NiDX9SourceTextureData>, Ptr, Ptr) -> Ptr<NiDX9SourceTextureData>
        ),
        entry!(
            0x00e68760,
            fn_00e68760(Ptr<NiDX9SourceTextureData>, Ptr) -> bool
        ),
        entry!(0x00e68830, fn_00e68830(Ptr<NiDX9SourceTextureData>)),
        entry!(
            0x00e68ea0,
            fn_00e68ea0(Ptr<NiDX9SourceTextureData>, u32) -> Ptr<NiDX9SourceTextureData>
        ),
        entry!(
            0x00e68530,
            fn_00e68530(Ptr<NiDX9SourceTextureData>, Ptr) -> bool
        ),
        entry!(0x00e68150, fn_00e68150(Ptr, u32, Ptr, u32)),
        entry!(
            0x00e68010,
            fn_00e68010(Ptr<NiDX9SourceTextureData>, Ptr) -> bool
        ),
        entry!(0x00e68870, fn_00e68870(Ptr<NiDX9SourceTextureData>, Ptr)),
        entry!(0x00e68a80, fn_00e68a80(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e68ef0, fn_00e68ef0(Ptr, Ptr) -> Ptr),
        entry!(0x00e6_73f0, fn_00e673f0(Ptr) -> i32),
        entry!(0x00e6_7420, fn_00e67420(Ptr, u32, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e6_7570, fn_00e67570(Ptr)),
        entry!(0x00e6_7660, fn_00e67660(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e6_7740, fn_00e67740(Ptr, i32, Ptr, Ptr) -> u32),
        entry!(0x00e6_79d0, fn_00e679d0(Ptr)),
        entry!(0x00e6_7ae0, fn_00e67ae0(Ptr, Ptr) -> Ptr),
        entry!(0x00e6_7c50, fn_00e67c50(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00e6_82b0,
            fn_00e682b0(Ptr<NiDX9SourceTextureData>, Ptr, u32) -> bool
        ),
        entry!(0x00e6_83d0, fn_00e683d0(Ptr, u32, Ptr, u32)),
        entry!(0x00e6_87e0, fn_00e687e0(Ptr, u32)),
        entry!(0x00e6_8820, fn_00e68820(Ptr) -> u32),
        entry!(0x00e6_8920, fn_00e68920(Ptr<NiDX9SourceTextureData>, Ptr)),
        entry!(0x00e6_89d0, fn_00e689d0(Ptr<NiDX9SourceTextureData>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e67fd0_clears_the_first_word_of_each_record() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1200);
        e.mem.set_i32(this + 0x10, 3);
        e.mem.set_i32(this + 8, 2);
        for offset in (0..0x1200).step_by(4) {
            e.mem.set_u32(this + offset, 0x5555_5555);
        }
        e.mem.set_i32(this + 0x10, 3);
        e.mem.set_i32(this + 8, 2);
        e.call(0x00e6_7fd0, &args![Ptr::<()>::new(this)]);
        let cleared: Vec<u32> = (0..0x1200u32)
            .step_by(4)
            .filter(|offset| e.mem.u32(this + offset) == 0)
            .collect();
        assert_eq!(cleared, vec![0x50, 0x90, 0xfb4, 0x1004, 0x1054]);
        // A zero count touches nothing.
        e.mem.set_i32(this + 0x10, 0);
        e.mem.set_i32(this + 8, -1);
        e.mem.set_u32(this + 0x50, 7);
        e.call(0x00e6_7fd0, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this + 0x50), 7);
    }

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

    /// An object whose virtual table sends each listed slot to an address (doubles are
    /// registered there by the test).
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        object
    }

    /// Pixel data with one face: the level sizes, 1 byte per pixel and offsets of each level.
    fn pixel_data(e: &mut Engine, sizes: &[(u32, u32)]) -> u32 {
        let pd = e.mem.alloc(0x80);
        let widths = e.mem.alloc(4 * sizes.len() as u32 + 4);
        let heights = e.mem.alloc(4 * sizes.len() as u32 + 4);
        let offsets = e.mem.alloc(4 * sizes.len() as u32 + 4);
        let mut offset = 0;
        for (i, &(w, h)) in sizes.iter().enumerate() {
            e.mem.set_u32(widths + 4 * i as u32, w);
            e.mem.set_u32(heights + 4 * i as u32, h);
            e.mem.set_u32(offsets + 4 * i as u32, offset);
            offset += w * h;
        }
        e.mem.set_u32(offsets + 4 * sizes.len() as u32, offset);
        let pixels = e.mem.alloc(offset.max(4) * 2);
        e.mem.set_u32(pd + PIXEL_DATA_WIDTHS, widths);
        e.mem.set_u32(pd + PIXEL_DATA_HEIGHTS, heights);
        e.mem.set_u32(pd + PIXEL_DATA_OFFSETS, offsets);
        e.mem.set_u32(pd + PIXEL_DATA_PIXELS, pixels);
        e.mem.set_u32(pd + PIXEL_DATA_LEVELS, sizes.len() as u32);
        e.mem.set_u32(pd + PIXEL_DATA_STRIDE, 1);
        e.mem.set_u32(pd + PIXEL_DATA_FACES, 1);
        pd
    }

    fn quiet_prints(e: &mut Engine) {
        e.register(ERROR_STRING, |_, a| word(a[0] ^ 0xffff_0000));
        e.register(DEBUG_PRINT, |_, _| Ret::default());
    }

    #[test]
    fn fn_00e68810_and_the_field_getters() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x64, 0x1111);
        e.mem.set_u32(this + 0x68, 7);
        assert_eq!(e.call(0x00e68810, &args![this]).u32(), this);
        assert_eq!(e.call(0x00e687f0, &args![this]).u32(), 7);
        assert_eq!(e.call(0x00e68800, &args![this]).u32(), 0x1111);
    }

    #[test]
    fn fn_00e687a0_constructs_the_data() {
        let mut e = Engine::new();
        e.register(TEXTURE_DATA_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x84);
        for off in (0..0x84).step_by(4) {
            e.mem.set_u32(this + off, 0xaaaa_aaaa);
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e687a0, &args![this, 0x500u32, 0x600u32]).u32(),
            this
        );
        assert_eq!(
            calls_to(&e, TEXTURE_DATA_CONSTRUCTOR),
            vec![vec![this, 0x500, 0x600]]
        );
        assert_eq!(e.mem.u32(this), SOURCE_TEXTURE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x500);
        assert_eq!(e.mem.u32(this + 0x6c), 0xaaaa_0000);
        for off in [0x70, 0x74, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00e68760_takes_a_palette() {
        let mut e = Engine::new();
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x84);
        let palette = e.mem.alloc(0x20);
        e.mem.set_u32(palette + 0x10, 3);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e68760, &args![this, 0u32]).bool());
        assert!(e.call(0x00e68760, &args![this, palette]).bool());
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this + 0x74, palette]]
        );
        assert_eq!(e.mem.u32(this + 0x80), 3);
        // The same palette and revision: nothing changed.
        assert!(!e.call(0x00e68760, &args![this, palette]).bool());
        // A new revision only.
        e.mem.set_u32(palette + 0x10, 4);
        assert!(e.call(0x00e68760, &args![this, palette]).bool());
        assert_eq!(calls_to(&e, POINTER_ASSIGN).len(), 1);
        assert_eq!(e.mem.u32(this + 0x80), 4);
    }

    const DELETE_DOUBLE: u32 = 0x0073_0000;

    fn counting_interlocked(e: &mut Engine) {
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
    fn destructors_release_the_palette() {
        let mut e = Engine::new();
        counting_interlocked(&mut e);
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(TEXTURE_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x84);
        let palette = object_with(&mut e, &[(DELETE_THIS_SLOT, DELETE_DOUBLE)]);
        e.mem.set_u32(palette + 4, 2);
        e.mem.set_u32(this + 0x74, palette);
        e.call_log = Some(vec![]);
        e.call(0x00e68830, &args![this]);
        assert_eq!(e.mem.u32(this), SOURCE_TEXTURE_DATA_VTABLE);
        assert_eq!(e.mem.u32(palette + 4), 1);
        assert!(calls_to(&e, DELETE_DOUBLE).is_empty());
        assert_eq!(calls_to(&e, TEXTURE_DATA_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(e.call(0x00e68ea0, &args![this, 0u32]).u32(), this);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![palette]]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.mem.set_u32(this + 0x74, 0);
        e.call(0x00e68ea0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x84]]);
    }

    const LEVEL_COUNT: u32 = 0x0074_0000;
    const GET_TYPE: u32 = 0x0074_0010;
    const LEVEL_DESC: u32 = 0x0074_0020;

    /// A Direct3D texture of `kind` with `levels` levels; level `n` is `64 >> n` wide and
    /// `32 >> n` high (depth 2 for a volume), format `format`.
    fn d3d_texture(e: &mut Engine, kind: u32, levels: u32, format: u32) -> u32 {
        e.register_double(LEVEL_COUNT, move |_, _| word(levels));
        e.register_double(GET_TYPE, move |_, _| word(kind));
        e.register_double(LEVEL_DESC, move |e, a| {
            let (level, desc) = (a[1], a[2]);
            e.mem.set_u32(desc, format);
            if kind == D3DRTYPE_VOLUMETEXTURE {
                e.mem.set_u32(desc + 0x10, 64 >> level);
                e.mem.set_u32(desc + 0x14, 32 >> level);
                e.mem.set_u32(desc + 0x18, 2);
            } else {
                e.mem.set_u32(desc + 0x18, 64 >> level);
                e.mem.set_u32(desc + 0x1c, 32 >> level);
            }
            word(if level == 9 { 0x8000_0000 } else { 0 })
        });
        object_with(
            e,
            &[
                (D3D_GET_LEVEL_COUNT, LEVEL_COUNT),
                (D3D_GET_TYPE, GET_TYPE),
                (D3D_GET_LEVEL_DESC, LEVEL_DESC),
            ],
        )
    }

    fn pixel_format_doubles(e: &mut Engine) {
        e.register(PIXEL_FORMAT_CONSTRUCTOR, |e, a| {
            for i in 0..0x11 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            word(a[0])
        });
        // 16 bits per pixel.
        e.register(FILL_PIXEL_FORMAT, |e, a| {
            e.mem.set_u8(a[1] + 1, 16);
            Ret::default()
        });
    }

    #[test]
    fn fn_00e68530_reads_a_texture() {
        let mut e = Engine::new();
        pixel_format_doubles(&mut e);
        let tex = d3d_texture(&mut e, D3DRTYPE_TEXTURE, 2, 0x15);
        let this = e.mem.alloc(0x84);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e68530, &args![this, tex]).bool());
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (64, 32));
        assert_eq!(e.mem.u32(this + 0x68), 2);
        assert_eq!(e.mem.u8(this + 0x6d), 1);
        // (64 * 32 + 32 * 16) * 16 / 8.
        assert_eq!(e.mem.u32(this + 0x70), 0x1400);
        assert_eq!(e.mem.u32(this + 0x64), tex);
        assert_eq!(
            calls_to(&e, FILL_PIXEL_FORMAT),
            vec![vec![0x15, this + 0x14]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0x101);
    }

    #[test]
    fn fn_00e68530_cube_volume_and_failures() {
        let mut e = Engine::new();
        pixel_format_doubles(&mut e);
        let this = e.mem.alloc(0x84);
        let cube = d3d_texture(&mut e, D3DRTYPE_CUBETEXTURE, 1, 0x15);
        assert!(e.call(0x00e68530, &args![this, cube]).bool());
        assert_eq!(e.mem.u32(this + 0x70), 64 * 32 * 2 * 6);
        assert_eq!(e.mem.u8(this + 0x6d), 0);
        let volume = d3d_texture(&mut e, D3DRTYPE_VOLUMETEXTURE, 1, 0x15);
        assert!(e.call(0x00e68530, &args![this, volume]).bool());
        assert_eq!(e.mem.u32(this + 0x70), 64 * 32 * 2 * 2);
        // A palettized format fails after taking the sides; +0x64 stays.
        e.mem.set_u32(this + 0x64, 0);
        let palettized = d3d_texture(&mut e, D3DRTYPE_TEXTURE, 1, D3DFMT_P8);
        assert!(!e.call(0x00e68530, &args![this, palettized]).bool());
        assert_eq!(e.mem.u32(this + 0x64), 0);
        // A failing description (level 9) fails.
        let deep = d3d_texture(&mut e, D3DRTYPE_TEXTURE, 10, 0x15);
        assert!(!e.call(0x00e68530, &args![this, deep]).bool());
        // Another resource type: just stored.
        let other = d3d_texture(&mut e, 1, 3, 0x15);
        assert!(e.call(0x00e68530, &args![this, other]).bool());
        assert_eq!(e.mem.u32(this + 0x64), other);
        assert_eq!(e.mem.u32(this + 0x70), 0);
    }

    const SURFACE_DESC: u32 = 0x0075_0000;
    const SURFACE_LOCK: u32 = 0x0075_0010;
    const SURFACE_UNLOCK: u32 = 0x0075_0020;

    /// A surface of height `height` and format `format`; its lock answers `bits` with
    /// `pitch`. `fail` 1 fails the description, 2 the lock.
    fn surface(e: &mut Engine, height: u32, format: u32, bits: u32, pitch: u32, fail: u32) -> u32 {
        e.register_double(SURFACE_DESC, move |e, a| {
            e.mem.set_u32(a[1], format);
            e.mem.set_u32(a[1] + 0x1c, height);
            word(if fail == 1 { 0x8000_0001 } else { 0 })
        });
        e.register_double(SURFACE_LOCK, move |e, a| {
            e.mem.set_u32(a[1], pitch);
            e.mem.set_u32(a[1] + 4, bits);
            word(if fail == 2 { 0x8000_0002 } else { 0 })
        });
        e.register(SURFACE_UNLOCK, |_, _| Ret::default());
        object_with(
            e,
            &[
                (D3D_SURFACE_GET_DESC, SURFACE_DESC),
                (D3D_SURFACE_LOCK_RECT, SURFACE_LOCK),
                (D3D_SURFACE_UNLOCK_RECT, SURFACE_UNLOCK),
            ],
        )
    }

    #[test]
    fn fn_00e68150_copies_rows() {
        let mut e = Engine::new();
        quiet_prints(&mut e);
        e.register(MEMCPY, |_, _| Ret::default());
        let pd = pixel_data(&mut e, &[(8, 4), (4, 2)]);
        let pixels = e.mem.u32(pd + PIXEL_DATA_PIXELS);
        // Pitch equals the row (8): one copy of 4 rows.
        let s = surface(&mut e, 16, 0x15, 0x9000, 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e68150, &args![pd, 0u32, s, 0u32]);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, pixels, 32]]);
        assert_eq!(calls_to(&e, SURFACE_UNLOCK), vec![vec![s]]);
        // Level 1 on face 1, pitch 16, surface height 1: one row of the 4 bytes.
        let s = surface(&mut e, 1, 0x15, 0x9000, 16, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e68150, &args![pd, 1u32, s, 1u32]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x9000, pixels + 40 + 32, 4]]
        );
        // A failed description or lock prints and returns without unlocking.
        let s = surface(&mut e, 1, 0x15, 0x9000, 16, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e68150, &args![pd, 0u32, s, 0u32]);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![GET_DESC_FAILED, 0x7fff_0001]]
        );
        let s = surface(&mut e, 1, 0x15, 0x9000, 16, 2);
        e.call_log = Some(vec![]);
        e.call(0x00e68150, &args![pd, 0u32, s, 0u32]);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![GET_SURFACE_LEVEL_FAILED, 0x7fff_0002]]
        );
        assert!(calls_to(&e, SURFACE_UNLOCK).is_empty());
    }

    #[test]
    fn fn_00e68150_counts_compressed_blocks() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        let pd = pixel_data(&mut e, &[(8, 6)]);
        e.mem.set_i32(pd + PIXEL_DATA_FORMAT_KIND, FORMAT_DXT1);
        let pixels = e.mem.u32(pd + PIXEL_DATA_PIXELS);
        // DXT1 surface: a row is (8 + 3) / 2 & !1 = 4 bytes, the pitch 16 / 4 = 4, height 8.
        let s = surface(&mut e, 16, FOURCC_DXT1, 0x9000, 16, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e68150, &args![pd, 0u32, s, 0u32]);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, pixels, 32]]);
        // Another four-character code: a row is 11 & !3 = 8 bytes against a pitch of 4: eight
        // rows of 4 bytes, the destination moving by the whole pitch.
        let s = surface(&mut e, 16, FOURCC_DXT5, 0x9000, 16, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e68150, &args![pd, 0u32, s, 0u32]);
        let copies = calls_to(&e, MEMCPY);
        assert_eq!(copies.len(), 8);
        assert_eq!(copies[1], vec![0x9010, pixels + 8, 4]);
    }

    const RENDERER_CAPS: u32 = 0x0076_0000;
    const CREATE_TEXTURE: u32 = 0x0076_0010;

    /// A renderer with capability bits `caps` and a device whose `CreateTexture` answers
    /// `result` and stores `texture`.
    fn renderer(e: &mut Engine, caps: u32, result: u32, texture: u32) -> u32 {
        e.register_double(RENDERER_CAPS, move |_, _| word(caps));
        e.register_double(CREATE_TEXTURE, move |e, a| {
            e.mem.set_u32(a[7], texture);
            word(result)
        });
        let device = object_with(e, &[(D3D_CREATE_TEXTURE, CREATE_TEXTURE)]);
        let renderer = e.mem.alloc(0x900);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(renderer, vtable);
        e.mem.set_u32(vtable + RENDERER_CAPS_SLOT, RENDERER_CAPS);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        renderer
    }

    fn globals(e: &mut Engine) {
        e.map(SKIP_LEVELS, 0x200);
        e.map(DYNAMIC_TEXTURES, 0x10);
    }

    #[test]
    fn fn_00e68010_creates_the_texture() {
        let mut e = Engine::new();
        globals(&mut e);
        quiet_prints(&mut e);
        e.register(D3D_FORMAT_OF, |_, _| word(0x15));
        let r = renderer(&mut e, 0, 0, 0x7700);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x60, r);
        let pd = pixel_data(&mut e, &[(64, 32), (32, 16), (16, 8)]);
        e.mem.set_u32(SKIP_LEVELS, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e68010, &args![this, pd]).bool());
        let create = calls_to(&e, CREATE_TEXTURE);
        // 3 levels, one skipped: 32 x 16, 2 levels, managed.
        assert_eq!(&create[0][1..7], &[32, 16, 2, 0, 0x15, 1]);
        assert_eq!(create[0][8], 0);
        assert_eq!(e.mem.u32(this + 0x78), 1);
        assert_eq!(e.mem.u32(this + 0x64), 0x7700);
        assert!(calls_to(&e, RENDERER_CAPS).is_empty());
    }

    #[test]
    fn fn_00e68010_handles_odd_sides_dynamic_textures_and_failure() {
        let mut e = Engine::new();
        globals(&mut e);
        quiet_prints(&mut e);
        e.register(D3D_FORMAT_OF, |_, _| word(0x15));
        let r = renderer(&mut e, 0, 0x8000_0003, 0);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x60, r);
        e.mem.set_u32(this + 0x64, 0x55);
        let pd = pixel_data(&mut e, &[(30, 7), (15, 3)]);
        e.mem.set_u32(SKIP_LEVELS, 4);
        e.mem.set_u8(DYNAMIC_TEXTURES, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e68010, &args![this, pd]).bool());
        // Not powers of two and no capability: one level, nothing skipped.
        let create = calls_to(&e, CREATE_TEXTURE);
        assert_eq!(&create[0][1..7], &[30, 7, 1, D3DUSAGE_DYNAMIC, 0x15, 0]);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![CREATE_SURF_FAILED, 0x7fff_0003]]
        );
        // With the capability: both levels, one skipped, odd sides round up.
        let r = renderer(&mut e, CAPS_NONPOW2, 0, 0x7700);
        e.mem.set_u32(this + 0x60, r);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e68010, &args![this, pd]).bool());
        let create = calls_to(&e, CREATE_TEXTURE);
        assert_eq!(&create[0][1..4], &[15, 4, 1]);
    }

    const SURFACE_LEVEL: u32 = 0x0077_0000;
    const RELEASE_DOUBLE: u32 = 0x0077_0010;

    #[test]
    fn fn_00e68870_copies_each_level() {
        let mut e = Engine::new();
        quiet_prints(&mut e);
        e.register(RELEASE_DOUBLE, |_, _| Ret::default());
        // Every surface fails its description, so 00e68150 only prints.
        let s = surface(&mut e, 1, 0x15, 0, 0, 1);
        let vtable = e.mem.u32(s);
        e.mem.set_u32(vtable + D3D_RELEASE, RELEASE_DOUBLE);
        e.register_double(SURFACE_LEVEL, move |e, a| {
            e.mem.set_u32(a[2], s);
            word(if a[1] == 2 { 0x8000_0009 } else { 0 })
        });
        e.register_double(GET_TYPE, |_, _| word(D3DRTYPE_TEXTURE));
        let tex = object_with(
            &mut e,
            &[
                (D3D_GET_TYPE, GET_TYPE),
                (D3D_GET_SURFACE_LEVEL, SURFACE_LEVEL),
            ],
        );
        let this = e.mem.alloc(0x84);
        let pd = pixel_data(&mut e, &[(8, 8), (4, 4), (2, 2)]);
        e.call_log = Some(vec![]);
        e.call(0x00e68870, &args![this, pd]);
        assert!(calls_to(&e, SURFACE_LEVEL).is_empty());
        e.mem.set_u32(this + 0x64, tex);
        e.mem.set_u32(this + 0x78, 1);
        e.call(0x00e68870, &args![this, pd]);
        // Levels 0 and 1 (3 levels, one skipped); each surface released.
        let levels: Vec<u32> = calls_to(&e, SURFACE_LEVEL).iter().map(|c| c[1]).collect();
        assert_eq!(levels, vec![0, 1]);
        assert_eq!(calls_to(&e, RELEASE_DOUBLE).len(), 2);
        assert_eq!(e.mem.u32(this + 0x68), 3);
        // No skipped levels: level 2 fails and is printed.
        e.mem.set_u32(this + 0x78, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e68870, &args![this, pd]);
        let prints = calls_to(&e, DEBUG_PRINT);
        assert_eq!(
            prints.last().unwrap(),
            &vec![COPY_GET_SURFACE_LEVEL_FAILED, 2, 0x7fff_0009]
        );
        // A cube texture is left alone.
        e.register_double(GET_TYPE, |_, _| word(D3DRTYPE_CUBETEXTURE));
        e.call_log = Some(vec![]);
        e.call(0x00e68870, &args![this, pd]);
        assert!(calls_to(&e, SURFACE_LEVEL).is_empty());
    }

    const FILE_GOOD: u32 = 0x0078_0000;
    const FILE_SIZE: u32 = 0x0078_0010;
    const FILE_DELETE: u32 = 0x0078_0020;
    const INIT_FROM_D3D: u32 = 0x0078_0030;

    /// A renderer with a device, a file of `size` bytes holding `bytes` at its start, and the
    /// doubles `00e68a80` needs; `kind` and `format` are the image description's.
    fn load_setup(e: &mut Engine, size: u32, kind: u32, file_format: u32) -> (u32, u32, u32) {
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + RENDERER_DEVICE, 0xd3d);
        e.register_double(FILE_GOOD, |_, _| word(1));
        e.register_double(FILE_SIZE, move |_, _| word(size));
        e.register(FILE_DELETE, |_, _| Ret::default());
        let file = object_with(
            e,
            &[
                (FILE_GOOD_SLOT, FILE_GOOD),
                (FILE_SIZE_SLOT, FILE_SIZE),
                (FILE_DELETE_SLOT, FILE_DELETE),
            ],
        );
        let buffer = e.mem.alloc(size + 0x100);
        e.register_double(ALLOCATE, move |_, _| word(buffer));
        e.register(FILE_READ, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register_double(D3DX_GET_IMAGE_INFO, move |e, a| {
            e.mem.set_u32(a[2], 64);
            e.mem.set_u32(a[2] + 4, 32);
            e.mem.set_u32(a[2] + 0x10, 0x15);
            e.mem.set_u32(a[2] + 0x14, kind);
            e.mem.set_u32(a[2] + 0x18, file_format);
            word(0)
        });
        for creator in [
            D3DX_CREATE_TEXTURE,
            D3DX_CREATE_CUBE_TEXTURE,
            D3DX_CREATE_VOLUME_TEXTURE,
        ] {
            e.register(creator, |e, a| {
                e.mem.set_u32(a[3], 0x7e70);
                word(0)
            });
        }
        let this = object_with(e, &[(INIT_FROM_D3D_TEXTURE_SLOT, INIT_FROM_D3D)]);
        e.register(INIT_FROM_D3D, |_, _| word(1));
        (renderer, file, this)
    }

    #[test]
    fn fn_00e68a80_loads_a_file_into_a_texture() {
        let mut e = Engine::new();
        let (renderer, file, this) = load_setup(&mut e, 0x40, D3DRTYPE_TEXTURE, 1);
        let prefs = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e68a80, &args![this, 0u32, 0u32, prefs, file])
            .bool());
        assert!(e
            .call(0x00e68a80, &args![this, 0u32, renderer, prefs, file])
            .bool());
        let buffer = calls_to(&e, FILE_READ)[0][1];
        assert_eq!(calls_to(&e, FILE_READ), vec![vec![file, buffer, 0x40]]);
        assert_eq!(calls_to(&e, FILE_DELETE), vec![vec![file, 1]]);
        let create = calls_to(&e, D3DX_CREATE_TEXTURE);
        assert_eq!(&create[0][..3], &[0xd3d, buffer, 0x40]);
        assert_eq!(calls_to(&e, INIT_FROM_D3D), vec![vec![this, 0x7e70]]);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (64, 32));
        assert_eq!(calls_to(&e, FREE), vec![vec![buffer]]);
        // Pixel layout 4 needs a bump-map format: 0x15 is refused.
        e.mem.set_u32(prefs, 4);
        assert!(!e
            .call(0x00e68a80, &args![this, 0u32, renderer, prefs, file])
            .bool());
        assert_eq!(calls_to(&e, FREE).len(), 2);
    }

    #[test]
    fn fn_00e68a80_other_types_refusals_and_file_names() {
        let mut e = Engine::new();
        let (renderer, file, this) = load_setup(&mut e, 0x40, D3DRTYPE_CUBETEXTURE, 1);
        let prefs = e.mem.alloc(0xc);
        // The data refuses the texture: it is released.
        e.register(INIT_FROM_D3D, |_, _| word(0));
        e.register(0x0078_0040, |_, _| Ret::default());
        e.register(D3DX_CREATE_CUBE_TEXTURE, |e, a| {
            let texture = e.mem.alloc(8);
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(texture, vtable);
            e.mem.set_u32(vtable + D3D_RELEASE, 0x0078_0040);
            e.mem.set_u32(a[3], texture);
            word(0)
        });
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e68a80, &args![this, 0u32, renderer, prefs, file])
            .bool());
        assert_eq!(calls_to(&e, 0x0078_0040).len(), 1);
        // No file: the name is split and an unknown extension refused.
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        e.register(FILENAME_CONSTRUCTOR, |_, _| Ret::default());
        e.register(NI_STRICMP, |_, _| word(1));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e68a80, &args![this, 0x4444u32, renderer, prefs, 0u32])
            .bool());
        let compared: Vec<u32> = calls_to(&e, NI_STRICMP).iter().map(|c| c[1]).collect();
        assert_eq!(compared, vec![EXTENSION_BMP, EXTENSION_TGA, EXTENSION_DDS]);
        assert_eq!(calls_to(&e, STRCPY_S)[0][1..], [0x104, 0x4444]);
        // `.tga` matches and the file opens.
        e.register(NI_STRICMP, |_, a| word(u32::from(a[1] != EXTENSION_TGA)));
        e.register_double(FILE_GET_FILE, move |_, _| word(file));
        e.register(INIT_FROM_D3D, |_, _| word(1));
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e68a80, &args![this, 0x4444u32, renderer, prefs, 0u32])
            .bool());
        assert_eq!(calls_to(&e, FILE_GET_FILE)[0][1..], [0, 0x8000]);
        // A bad file is deleted.
        e.register_double(FILE_GOOD, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e68a80, &args![this, 0u32, renderer, prefs, file])
            .bool());
        assert_eq!(calls_to(&e, FILE_DELETE), vec![vec![file, 1]]);
    }

    #[test]
    fn fn_00e68a80_drops_dds_levels() {
        let mut e = Engine::new();
        e.map(DDS_SKIP_LEVELS, 8);
        e.mem.set_u32(DDS_SKIP_LEVELS, 1);
        e.mem.set_u32(DDS_MIN_LEVELS, 1);
        let (renderer, file, this) = load_setup(&mut e, 0x800, D3DRTYPE_TEXTURE, D3DXIFF_DDS);
        let prefs = e.mem.alloc(0xc);
        let buffer = e.call(ALLOCATE, &args![0u32]).u32();
        e.mem.set_u32(buffer + 0x50, 4);
        e.mem.set_u32(buffer + 0x54, FOURCC_DXT1);
        e.mem.set_u32(buffer + 0x1c, 3);
        let fresh = e.mem.alloc(0x200);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| word(fresh));
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e68a80, &args![this, 0u32, renderer, prefs, file])
            .bool());
        // One level of 16 x 8 blocks, 16 bytes each, halved for DXT1: 0x400 bytes dropped.
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x400]]
        );
        assert_eq!(e.mem.u32(fresh + 0x10), 32);
        assert_eq!(e.mem.u32(fresh + 0xc), 16);
        assert_eq!(e.mem.u32(fresh + 0x1c), 2);
        assert_eq!(e.mem.u32(fresh + 0x14), 512);
        let copies = calls_to(&e, MEMCPY);
        assert_eq!(copies[0], vec![fresh, buffer, 0x80]);
        assert_eq!(copies[1], vec![fresh + 0x80, buffer + 0x80 + 0x400, 0x380]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, buffer]]
        );
        // The new buffer, with the file's size still.
        let create = calls_to(&e, D3DX_CREATE_TEXTURE);
        assert_eq!(&create[0][1..3], &[fresh, 0x800]);
        assert_eq!(calls_to(&e, FREE), vec![vec![fresh]]);
        // The data keeps the image's first sides.
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (64, 32));
    }

    const GET_RTTI: u32 = 0x0079_0000;
    const CONVERTER_READ: u32 = 0x0079_0010;
    const CONVERTER_CONVERT: u32 = 0x0079_0020;
    const AFTER_CREATE: u32 = 0x0079_0030;

    /// A source texture (RTTI chain `rtti`), a renderer, the new object `00aa13e0` answers
    /// (whose base constructor stores the texture and a cube Direct3D texture so the copy to
    /// the surfaces stops at once), and the converter.
    fn create_setup(e: &mut Engine, rtti: u32) -> (u32, u32, u32) {
        counting_interlocked(e);
        e.map(LOAD_DIRECT_TO_RENDERER, 0x10);
        e.map(CALL_AFTER_CREATE, 0x10);
        e.map(RENDERER_GLOBAL, 4);
        e.map(DIRECT3D, 4);
        e.register_double(GET_RTTI, move |_, _| word(rtti));
        e.register(AFTER_CREATE, |_, _| Ret::default());
        let texture = object_with(
            e,
            &[
                (GET_RTTI_SLOT, GET_RTTI),
                (SOURCE_AFTER_CREATE_SLOT, AFTER_CREATE),
            ],
        );
        let renderer = e.mem.alloc(0x900);
        let renderer_vtable = e.mem.alloc(0x200);
        e.mem.set_u32(renderer, renderer_vtable);
        e.mem
            .set_u32(renderer_vtable + RENDERER_CAPS_SLOT, RENDERER_CAPS);
        let object = e.mem.alloc(0x84);
        e.register_double(NEW_OBJECT, move |_, _| word(object));
        e.register_double(GET_TYPE, |_, _| word(D3DRTYPE_CUBETEXTURE));
        let d3d = object_with(e, &[(D3D_GET_TYPE, GET_TYPE)]);
        e.register_double(TEXTURE_DATA_CONSTRUCTOR, move |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            e.mem.set_u32(a[0] + 0x64, d3d);
            Ret::default()
        });
        let converter = object_with(
            e,
            &[
                (CONVERTER_READ_SLOT, CONVERTER_READ),
                (CONVERTER_CONVERT_SLOT, CONVERTER_CONVERT),
            ],
        );
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| word(converter));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        (texture, renderer, object)
    }

    #[test]
    fn fn_00e68ef0_hands_cube_maps_on() {
        let mut e = Engine::new();
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, NI_SOURCE_CUBE_MAP_RTTI);
        let (texture, renderer, _) = create_setup(&mut e, rtti);
        e.register(CREATE_CUBE_MAP_DATA, |_, _| word(0xc0be));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e68ef0, &args![texture, renderer]).u32(), 0xc0be);
        assert_eq!(
            calls_to(&e, CREATE_CUBE_MAP_DATA),
            vec![vec![texture, renderer]]
        );
        assert!(calls_to(&e, NEW_OBJECT).is_empty());
    }

    #[test]
    fn fn_00e68ef0_converts_pixel_data() {
        let mut e = Engine::new();
        let (texture, renderer, object) = create_setup(&mut e, 0);
        let pd = pixel_data(&mut e, &[(8, 8), (4, 4)]);
        e.mem.set_u32(pd + 4, 1);
        e.mem.set_u32(pd + PIXEL_DATA_REVISION, 5);
        let palette = e.mem.alloc(0x20);
        e.mem.set_u32(palette + 0x10, 9);
        e.mem.set_u32(pd + PIXEL_DATA_PALETTE, palette);
        e.mem.set_u32(texture + SOURCE_PIXEL_DATA, pd);
        e.mem.set_u32(texture + SOURCE_MIPMAPPED, 1);
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format, 0xf0f0);
        e.register_double(FIND_CLOSEST_FORMAT, move |_, _| word(format));
        // The conversion answers a copy with its own count of 1 (deleted at the end).
        let converted = pixel_data(&mut e, &[(8, 8), (4, 4)]);
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        let converted_vtable = e.mem.alloc(8);
        e.mem.set_u32(converted, converted_vtable);
        e.mem
            .set_u32(converted_vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(converted + 4, 0);
        e.register_double(CONVERTER_CONVERT, move |_, _| word(converted));
        e.mem.set_u8(CALL_AFTER_CREATE, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e68ef0, &args![texture, renderer]).u32(), object);
        assert_eq!(
            calls_to(&e, FIND_CLOSEST_FORMAT),
            vec![vec![object, pd + 8, texture + 0x18, renderer + 0x700]]
        );
        let convert = &calls_to(&e, CONVERTER_CONVERT)[0];
        assert_eq!(&convert[1..], &[pd, format, pd, 1]);
        assert_eq!(e.mem.u32(object + 0x14), 0xf0f0);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (8, 8));
        assert_eq!(e.mem.u32(object + 0x7c), 5);
        assert_eq!(e.mem.u8(object + 0x6d), 1);
        assert_eq!(e.mem.u32(object + 0x68), 2);
        assert_eq!(e.mem.u32(object + 0x74), palette);
        assert_eq!(e.mem.u32(object + 0x80), 9);
        assert_eq!(e.mem.u32(object + 0x70), 80);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![converted]]);
        assert_eq!(e.mem.u32(texture + TEXTURE_RENDERER_DATA), object);
        assert_eq!(calls_to(&e, AFTER_CREATE), vec![vec![texture]]);
        // The held pixel data is back at its count.
        assert_eq!(e.mem.u32(pd + 4), 1);
    }

    #[test]
    fn fn_00e68ef0_falls_back_to_replacement_data() {
        let mut e = Engine::new();
        let (texture, renderer, object) = create_setup(&mut e, 0);
        quiet_prints(&mut e);
        let replacement = pixel_data(&mut e, &[(4, 4)]);
        e.mem.set_u32(replacement + 4, 1);
        e.register_double(REPLACEMENT_DATA, move |_, _| word(replacement));
        let formats = renderer + RENDERER_FORMATS;
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(formats + 8, format);
        e.mem
            .set_u32(renderer + RENDERER_REPLACEMENT_FORMAT_INDEX, 2);
        // No data and no file: the converter reads nothing.
        e.register(CONVERTER_READ, |_, _| word(0));
        e.mem.set_u32(texture + SOURCE_PLATFORM_FILENAME, 0x4242);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e68ef0, &args![texture, renderer]).u32(), object);
        assert_eq!(
            calls_to(&e, REPLACEMENT_DATA),
            vec![vec![texture, 0x8000_0005, 1]]
        );
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![TEXTURE_NOT_FOUND, 0x4242]]
        );
        assert_eq!(calls_to(&e, CONVERTER_READ)[0][1..], [0x4242, 0, 0]);
        assert_eq!(e.mem.u8(object + 0x6c), 1);
        assert_eq!(e.mem.u8(object + 0x6d), 0);
        assert!(calls_to(&e, CONVERTER_CONVERT).is_empty());
        assert_eq!(e.mem.u32(object + 0x68), 1);
        assert_eq!(e.mem.u32(object + 0x70), 16);
        // A non-power-of-two image on a renderer that allows neither: code 0x80000006.
        let odd = pixel_data(&mut e, &[(6, 4)]);
        e.mem.set_u32(odd + 4, 1);
        e.mem.set_u32(texture + SOURCE_PIXEL_DATA, odd);
        e.register_double(RENDERER_CAPS, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x00e68ef0, &args![texture, renderer]);
        assert_eq!(
            calls_to(&e, REPLACEMENT_DATA),
            vec![vec![texture, 0x8000_0006, 1]]
        );
        assert_eq!(calls_to(&e, RENDERER_CAPS).len(), 2);
    }

    #[test]
    fn fn_00e68ef0_loads_straight_to_the_renderer_and_copies_layout_3_preferences() {
        let mut e = Engine::new();
        let (texture, renderer, object) = create_setup(&mut e, 0);
        // Loading straight: no device on the renderer, so 00e68a80 refuses and the
        // converter reads the file.
        e.mem.set_u8(LOAD_DIRECT_TO_RENDERER, 1);
        e.mem.set_u8(texture + SOURCE_LOAD_DIRECT_HINT, 1);
        let pd = pixel_data(&mut e, &[(6, 4)]);
        e.mem.set_u32(pd + 4, 1);
        e.register_double(CONVERTER_READ, move |_, _| word(pd));
        e.register_double(RENDERER_CAPS, |_, _| word(CAPS_NONPOW2_CONDITIONAL));
        e.mem.set_u32(texture + SOURCE_FORMAT_PREFS, 3);
        e.mem.set_u32(texture + SOURCE_FORMAT_PREFS + 4, 7);
        e.mem.set_u32(texture + SOURCE_FORMAT_PREFS + 8, 8);
        let format = e.mem.alloc(0x44);
        e.register_double(FIND_CLOSEST_FORMAT, move |e, a| {
            assert_eq!(
                (e.mem.u32(a[2]), e.mem.u32(a[2] + 4), e.mem.u32(a[2] + 8)),
                (2, 7, 8)
            );
            word(format)
        });
        e.register_double(CONVERTER_CONVERT, move |_, _| word(pd));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e68ef0, &args![texture, renderer]).u32(), object);
        assert_eq!(calls_to(&e, CONVERTER_READ).len(), 1);
        assert_ne!(calls_to(&e, FIND_CLOSEST_FORMAT)[0][2], texture + 0x18);
        assert_eq!(e.mem.u8(object + 0x6c), 0);
        // The converted data is the held one: two references taken, two given up.
        assert_eq!(e.mem.u32(pd + 4), 1);
    }

    const CHECK_FORMAT: u32 = 0x007a_0000;

    #[test]
    fn fn_00e68ef0_uses_persistent_data() {
        let mut e = Engine::new();
        let (texture, renderer, object) = create_setup(&mut e, 0);
        let persistent = e.mem.alloc(0x94);
        let widths = e.mem.alloc(4);
        e.mem.set_u32(widths, 16);
        let heights = e.mem.alloc(4);
        e.mem.set_u32(heights, 8);
        let offsets = e.mem.alloc(8);
        e.mem.set_u32(offsets + 4, 0x80);
        e.mem.set_u32(persistent + PERSISTENT_WIDTHS, widths);
        e.mem.set_u32(persistent + PERSISTENT_HEIGHTS, heights);
        e.mem.set_u32(persistent + PERSISTENT_OFFSETS, offsets);
        e.mem.set_u32(persistent + PERSISTENT_LEVELS, 1);
        e.mem.set_u32(persistent + PERSISTENT_FACES, 6);
        e.mem.set_u32(persistent + 0x14, 0xabcd);
        e.mem.set_u32(texture + SOURCE_PERSISTENT_DATA, persistent);
        let format = e.mem.alloc(0x44);
        e.register_double(FIND_CLOSEST_FORMAT, move |_, _| word(format));
        e.register(D3D_FORMAT_OF, |_, _| word(0x15));
        e.register(CHECK_FORMAT, |_, _| word(0));
        let direct3d = object_with(&mut e, &[(D3D_CHECK_DEVICE_FORMAT, CHECK_FORMAT)]);
        e.mem.set_u32(DIRECT3D, direct3d);
        let global = e.mem.alloc(0x600);
        e.mem.set_u32(global + 0x5c4, 1);
        e.mem.set_u32(global + 0x5c8, 2);
        e.mem.set_u32(global + 0x5d0, 3);
        e.mem.set_u32(RENDERER_GLOBAL, global);
        e.register(0x00e6_8920, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e68ef0, &args![texture, renderer]).u32(), object);
        assert_eq!(
            calls_to(&e, CHECK_FORMAT),
            vec![vec![direct3d, 1, 2, 3, 0, 3, 0x15]]
        );
        assert_eq!(e.mem.u32(object + 0x14), 0xabcd);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (16, 8));
        assert_eq!(e.mem.u32(object + 0x60), renderer);
        assert_eq!(e.mem.u32(object + 0x70), 0x300);
        assert_eq!(e.mem.u8(object + 0x6d), 0);
        // The Direct3D texture exists already (the base constructor's), so only 00e68920.
        assert_eq!(calls_to(&e, 0x00e6_8920), vec![vec![object, persistent]]);
        // A failed format check gives replacement data instead.
        e.mem.set_u32(renderer + RENDERER_FORMATS, format);
        let replacement = pixel_data(&mut e, &[(4, 4)]);
        e.mem.set_u32(replacement + 4, 1);
        e.register_double(REPLACEMENT_DATA, move |_, _| word(replacement));
        e.register(CHECK_FORMAT, |_, _| word(0x8000_0000));
        e.call_log = Some(vec![]);
        e.call(0x00e68ef0, &args![texture, renderer]);
        assert_eq!(
            calls_to(&e, REPLACEMENT_DATA),
            vec![vec![texture, 0x8000_0004, 1]]
        );
        assert_eq!(e.mem.u8(object + 0x6c), 1);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (4, 4));
    }
}

#[cfg(test)]
mod n081_tests {
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

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        object
    }

    fn quiet_prints(e: &mut Engine) {
        e.register(ERROR_STRING, |_, a| word(a[0] ^ 0xffff_0000));
        e.register(DEBUG_PRINT, |_, _| Ret::default());
    }

    fn constants(e: &mut Engine) {
        e.map(0x0101_0000, 0x0010_0000);
        e.mem.set_f32(HULL_ZERO, 0.0);
        e.mem.set_f32(HULL_THREE, 3.0);
        e.mem.set_f32(HULL_HALF, 0.5);
        e.mem.set_u32(HULL_FAR, 0x7f7f_ffee);
        e.mem.set_f32(HULL_COSINE_LIMIT, -0.5);
        e.mem.set_u32(HULL_EPSILON, 0x3400_0000);
        e.mem.set_f64(HULL_DOUBLE_ZERO, 0.0);
    }

    fn close(a: [f32; 4], b: [f32; 4]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 2e-3)
    }

    #[test]
    fn triangle_count_and_indices() {
        let mut e = Engine::new();
        let g = e.mem.alloc(0x20);
        for (kind, count16, count32, want) in [
            (1i8, 9u32, 0u32, 3i32),
            (1, 0, 7, 2),
            (2, 9, 0, 0),
            (1, 0xffff_fff9, 0, -2),
        ] {
            e.mem.set_i8(g + 8, kind);
            e.mem.set_u32(g + 0x10, count16);
            e.mem.set_u32(g + 0x1c, count32);
            assert_eq!(e.call(0x00e6_73f0, &args![g]).i32(), want);
        }
        let words = e.mem.alloc(16);
        for i in 0..8u32 {
            e.mem.set_u16(words + 2 * i, (10 + i) as u16);
        }
        let dwords = e.mem.alloc(32);
        for i in 0..8u32 {
            e.mem.set_u32(dwords + 4 * i, 20 + i);
        }
        e.mem.set_u32(g + 0xc, words);
        e.mem.set_u32(g + 0x18, dwords);
        let outs = e.mem.alloc(12);
        let get = |e: &mut Engine, t: u32| {
            let ok = e
                .call(0x00e6_7420, &args![g, t, outs, outs + 4, outs + 8])
                .bool();
            (
                ok,
                [e.mem.u32(outs), e.mem.u32(outs + 4), e.mem.u32(outs + 8)],
            )
        };
        e.mem.set_u32(g + 0x10, 6);
        e.mem.set_i8(g + 8, 1);
        assert_eq!(get(&mut e, 1), (true, [13, 14, 15]));
        e.mem.set_i8(g + 8, 2);
        assert_eq!(get(&mut e, 1), (true, [11, 12, 13]));
        e.mem.set_i8(g + 8, 3);
        assert_eq!(get(&mut e, 1), (false, [u32::MAX; 3]));
        e.mem.set_u32(g + 0x10, 0);
        e.mem.set_u32(g + 0x1c, 6);
        e.mem.set_i8(g + 8, 1);
        assert_eq!(get(&mut e, 1), (true, [23, 24, 25]));
        e.mem.set_i8(g + 8, 2);
        assert_eq!(get(&mut e, 2), (true, [22, 23, 24]));
        e.mem.set_u32(g + 0x1c, 0);
        assert_eq!(get(&mut e, 0), (false, [u32::MAX; 3]));
    }

    /// A hull block with vertex records at +0x20 (stride 0x40) holding `points`.
    fn hull(e: &mut Engine, points: &[[f32; 4]]) -> u32 {
        let hull = e.mem.alloc(0x1200);
        for (i, p) in points.iter().enumerate() {
            hull_store(e, hull + 0x20 + 0x40 * i as u32, *p);
        }
        hull
    }

    #[test]
    fn face_plane_from_its_three_points() {
        let mut e = Engine::new();
        constants(&mut e);
        let h = hull(
            &mut e,
            &[
                [0.0, 0.0, 2.0, 0.0],
                [1.0, 0.0, 2.0, 0.0],
                [0.0, 1.0, 2.0, 0.0],
            ],
        );
        let face = h + HULL_FACES;
        for k in 0..3u32 {
            e.mem.set_u32(face + 0x10 + 0x10 * k, h + 0x20 + 0x40 * k);
        }
        e.call(0x00e6_7570, &args![face]);
        assert!(close(hull_load(&e, face), [0.0, 0.0, 1.0, 0.0]));
        assert!((e.mem.f32(face + 0x40) - 2.0).abs() < 2e-3);
    }

    #[test]
    fn tetrahedron_edges_and_the_orientation_swap() {
        let mut e = Engine::new();
        constants(&mut e);
        let table: [u32; 36] = [
            0x00, 0xa0, 0x20, 0x80, 0xf0, 0x00, 0x40, 0x50, 0x00, 0x00, 0x00, 0x20, 0x40, 0xf0,
            0x20, 0xc0, 0xa0, 0x00, 0x00, 0x50, 0x20, 0xc0, 0xf0, 0x10, 0x80, 0x00, 0x00, 0x40,
            0x00, 0x10, 0x80, 0xa0, 0x10, 0xc0, 0x50, 0x10,
        ];
        for (i, w) in table.iter().enumerate() {
            e.mem.set_u32(HULL_EDGE_TABLE + 4 * i as u32, *w);
        }
        for (v0, swapped) in [
            ([0.0f32, 0.0, 0.0, 0.0], false),
            ([2.0, 2.0, 2.0, 0.0], true),
        ] {
            let h = hull(
                &mut e,
                &[
                    v0,
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                ],
            );
            e.call(0x00e6_79d0, &args![h]);
            assert_eq!(e.mem.u32(h + 0x10), 4);
            // Face 0, edge 1: its face, and the twin at +0xf80 + 0xf0.
            let f0 = h + HULL_FACES;
            assert_eq!(e.mem.u32(f0 + 0x2c), f0);
            assert_eq!(e.mem.u32(f0 + 0x28), h + 0xf80 + 0xf0);
            assert_eq!(e.mem.u32(f0 + 0x14), f0 + 0x20);
            assert_eq!(e.mem.u32(f0 + 0x34), f0 + 0x10);
            let first = if swapped { h + 0x60 } else { h + 0x20 };
            assert_eq!(e.mem.u32(h + 0xf80), first);
            let second = if swapped { h + 0x20 } else { h + 0x60 };
            assert_eq!(e.mem.u32(h + 0x1070), second);
            if !swapped {
                // Face 3 is (v1, v2, v3): normal along (1, 1, 1).
                let n = hull_load(&e, h + 0x1060);
                let s = 1.0 / 3.0f32.sqrt();
                assert!(close(n, [s, s, s, 0.0]), "{n:?}");
            }
        }
    }

    #[test]
    fn remove_faces_marks_the_horizon_and_frees_faces() {
        let mut e = Engine::new();
        constants(&mut e);
        let h = hull(&mut e, &[]);
        let f0 = e.mem.alloc(0x50);
        let a = e.mem.alloc(0x50);
        let b = e.mem.alloc(0x50);
        let c = e.mem.alloc(0x50);
        let x = e.mem.alloc(0x50);
        e.mem.set_u32(x + FACE_MARK, 3);
        e.mem.set_u32(f0 + FACE_MARK, 3);
        e.mem.set_u32(c + FACE_MARK, 2);
        let twin_of = |e: &mut Engine, face: u32| {
            let twin = e.mem.alloc(0x10);
            let vertex = e.mem.alloc(0x40);
            e.mem.set_u32(twin, vertex);
            e.mem.set_u32(twin + 0xc, face);
            twin
        };
        let t0 = twin_of(&mut e, a);
        let t1 = twin_of(&mut e, b);
        let t2 = twin_of(&mut e, c);
        for (k, t) in [t0, t1, t2].iter().enumerate() {
            e.mem.set_u32(f0 + 0x18 + 0x10 * k as u32, *t);
        }
        for k in 0..3u32 {
            let t = twin_of(&mut e, x);
            e.mem.set_u32(b + 0x18 + 0x10 * k, t);
        }
        e.register_double(HULL_CLASSIFY_FACE, move |_, args| {
            word(if args[0] == a { 2 } else { 1 })
        });
        let counter = e.mem.alloc(4);
        let edge = e.mem.alloc(4);
        let point = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x00e6_7660, &args![h, f0, point, counter, edge])
            .u32();
        assert_eq!(r, 0);
        assert_eq!(
            calls_to(&e, HULL_CLASSIFY_FACE),
            vec![vec![a, point, h, h + 4], vec![b, point, h, h + 4]]
        );
        assert_eq!(e.mem.u32(counter), 2);
        assert_eq!(e.mem.u32(edge), t2);
        assert_eq!(e.mem.u32(e.mem.u32(t0) + 0x30), t0);
        assert_eq!(e.mem.u32(a + FACE_MARK), 2);
        assert_eq!(e.mem.u32(b + FACE_MARK), 1);
        assert_eq!(e.mem.u32(h + 0xc), 2);
        assert_eq!(e.mem.u32(h + 0xde0), b);
        assert_eq!(e.mem.u32(h + 0xde4), f0);
        assert_eq!(e.mem.u32(f0 + 0x40), 0x7f7f_ffee);
        // A full free stack answers 2; a face answering 4 stops with 2.
        e.mem.set_u32(h + 0xc, 100);
        e.mem.set_u32(a + FACE_MARK, 0);
        e.mem.set_u32(b + FACE_MARK, 3);
        assert_eq!(
            e.call(0x00e6_7660, &args![h, f0, point, counter, edge])
                .u32(),
            2
        );
        e.mem.set_u32(h + 0xc, 0);
        e.mem.set_u32(a + FACE_MARK, 0);
        e.register(HULL_CLASSIFY_FACE, |_, _| word(4));
        assert_eq!(
            e.call(0x00e6_7660, &args![h, f0, point, counter, edge])
                .u32(),
            2
        );
        assert_eq!(e.mem.u32(h + 0xc), 1);
    }

    #[test]
    fn new_faces_join_the_horizon_to_the_point() {
        let mut e = Engine::new();
        constants(&mut e);
        let h = hull(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
            ],
        );
        let (va, vb, p) = (h + 0x20, h + 0x60, h + 0xa0);
        let next = e.mem.alloc(0x10);
        e.mem.set_u32(next, vb);
        let start = e.mem.alloc(0x10);
        e.mem.set_u32(start, va);
        e.mem.set_u32(start + 4, next);
        // The walk returns to the start: answer 0.
        e.mem.set_u32(vb + 0x30, start);
        let face = e.mem.alloc(0x50);
        e.mem.set_u32(h + 0xde0, face);
        e.mem.set_u32(h + 0xc, 1);
        assert_eq!(e.call(0x00e6_7740, &args![h, 1u32, start, p]).u32(), 0);
        assert_eq!(e.mem.u32(h + 0xc), 0);
        assert_eq!(e.mem.u32(face + 0x10), vb);
        assert_eq!(e.mem.u32(face + 0x20), va);
        assert_eq!(e.mem.u32(face + 0x30), p);
        assert_eq!(e.mem.u32(start + 8), face + 0x10);
        assert_eq!(e.mem.u32(face + 0x18), start);
        assert_eq!(e.mem.u32(face + 0x28), face + 0x30);
        assert_eq!(e.mem.u32(face + 0x38), face + 0x20);
        assert_eq!(e.mem.u32(vb + 0x30), 0);
        assert!(close(hull_load(&e, face), [0.0, 0.0, -1.0, 0.0]));
        // Not back at the start: 2; new records come from +0xf70.
        assert_eq!(e.call(0x00e6_7740, &args![h, 1u32, start, p]).u32(), 2);
        assert_eq!(e.mem.u32(h + 0x10), 1);
        assert_eq!(e.mem.u32(h + 0xf70 + 0x10), vb);
        // A degenerate face (the point on the edge's line) answers 2.
        assert_eq!(e.call(0x00e6_7740, &args![h, 1u32, start, vb]).u32(), 2);
        // No start edge answers 2.
        assert_eq!(e.call(0x00e6_7740, &args![h, 1u32, 0u32, p]).u32(), 2);
    }

    #[test]
    fn walk_stops_or_crosses_to_a_visited_neighbour() {
        let mut e = Engine::new();
        constants(&mut e);
        let h = hull(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
            ],
        );
        let face = h + HULL_FACES;
        let neighbour = h + HULL_FACES + 0x50;
        e.mem.set_u32(h + 0x10, 2);
        hull_store(&mut e, face, [0.0, 0.0, 1.0, 0.0]);
        hull_store(&mut e, neighbour, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f32(face + 0x40, -1.0);
        let twin = e.mem.alloc(0x10);
        e.mem.set_u32(twin + 0xc, neighbour);
        for f in [face, neighbour] {
            for k in 0..3u32 {
                let edge = f + 0x10 + 0x10 * k;
                e.mem.set_u32(edge, h + 0x20 + 0x40 * k);
                e.mem.set_u32(edge + 4, f + 0x10 + 0x10 * ((k + 1) % 3));
                e.mem.set_u32(edge + 8, twin);
            }
        }
        e.mem.set_u32(face + 0x44, 7);
        e.mem.set_u32(neighbour + 0x44, 7);
        // A very negative edge scale: no edge qualifies, the face itself is answered.
        e.mem.set_f64(HULL_EDGE_SCALE, -1e30);
        e.mem.set_f64(HULL_PLANE_SCALE, 0.9);
        assert_eq!(e.call(0x00e6_7ae0, &args![h, face]).u32(), face);
        assert_eq!(e.mem.u32(face + 0x44), 3);
        assert_eq!(e.mem.u32(neighbour + 0x44), 0);
        // A huge one: an edge qualifies and, the start's distance not being above 0, the walk
        // crosses into the neighbour (marked 3), whose own crossing leads back to itself
        // (the shared twin's face), already marked: the neighbour is answered.
        e.mem.set_f64(HULL_EDGE_SCALE, 1e30);
        assert_eq!(e.call(0x00e6_7ae0, &args![h, face]).u32(), neighbour);
        assert_eq!(e.mem.u32(neighbour + 0x44), 3);
    }

    /// A hull with two face records (the inner loop only reaches the second): normal and
    /// distance of each.
    fn two_faces(e: &mut Engine, faces: [([f32; 4], f32); 2]) -> u32 {
        let h = e.mem.alloc(0x1200);
        e.mem.set_u32(h + 0x10, 2);
        for (i, (n, d)) in faces.iter().enumerate() {
            let f = h + HULL_FACES + 0x50 * i as u32;
            hull_store(e, f, *n);
            e.mem.set_f32(f + 0x40, *d);
        }
        h
    }

    #[test]
    fn closest_planes_of_two_hulls() {
        let up = [0.0f32, 0.0, 1.0, 0.0];
        let side = [1.0f32, 0.0, 0.0, 0.0];
        let down = [0.0f32, 0.0, -1.0, 0.0];
        let s = 1.0 / 1.25f32.sqrt();
        for (b_normal, a_distance, b_distance, want_w, want_dir) in [
            // Same normal, p = 1 - 2 <= eps, a > eps: b² = 4 with weights (0, 1).
            (up, 1.0f32, 2.0f32, 2.0f32, up),
            // Perpendicular: both above eps, q >= p: 5 with weights (0.5, 1).
            (side, 1.0, 2.0, 5.0f32.sqrt(), [s, 0.0, 0.5 * s, 0.0]),
            // Both distances 0: -c² = -1, clamped to 0 for the root.
            (up, 0.0, 0.0, 0.0, up),
        ] {
            let mut e = Engine::new();
            constants(&mut e);
            e.register(HULL_SQRT, |_, a| Ret {
                st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
                ..Ret::default()
            });
            let first = two_faces(&mut e, [(down, 9.0), (up, a_distance)]);
            let second = two_faces(&mut e, [(down, 9.0), (b_normal, b_distance)]);
            let out = e.mem.alloc(16);
            let faces = e.mem.alloc(8);
            e.call(0x00e6_7c50, &args![first, second, out, faces, faces + 4]);
            assert_eq!(e.mem.u32(faces), first + HULL_FACES + 0x50);
            assert_eq!(e.mem.u32(faces + 4), second + HULL_FACES + 0x50);
            let v = hull_load(&e, out);
            assert!(close([v[0], v[1], v[2], 0.0], want_dir), "{v:?}");
            assert!((v[3] - want_w).abs() < 1e-5, "{v:?}");
        }
        // Opposite normals (cosine -1, below the limit) never pair: the face words stay.
        let mut e = Engine::new();
        constants(&mut e);
        e.register(HULL_SQRT, |_, _| Ret::default());
        let first = two_faces(&mut e, [(up, 9.0), (up, 1.0)]);
        let second = two_faces(&mut e, [(down, 9.0), (down, 1.0)]);
        let out = e.mem.alloc(16);
        let faces = e.mem.alloc(8);
        e.mem.set_u32(faces, first + HULL_FACES);
        e.mem.set_u32(faces + 4, second + HULL_FACES);
        e.call(0x00e6_7c50, &args![first, second, out, faces, faces + 4]);
        assert_eq!(e.mem.u32(faces), first + HULL_FACES);
        assert_eq!(e.mem.u32(faces + 4), second + HULL_FACES);
    }

    const RENDERER_CAPS: u32 = 0x0076_0000;
    const CREATE_TEXTURE: u32 = 0x0076_0010;

    fn renderer(e: &mut Engine, caps: u32, result: u32, texture: u32) -> u32 {
        e.register_double(RENDERER_CAPS, move |_, _| word(caps));
        e.register_double(CREATE_TEXTURE, move |e, a| {
            e.mem.set_u32(a[7], texture);
            word(result)
        });
        let device = object_with(e, &[(D3D_CREATE_TEXTURE, CREATE_TEXTURE)]);
        let renderer = e.mem.alloc(0x900);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(renderer, vtable);
        e.mem.set_u32(vtable + RENDERER_CAPS_SLOT, RENDERER_CAPS);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        renderer
    }

    #[test]
    fn persistent_create_surf() {
        let mut e = Engine::new();
        e.map(SKIP_LEVELS, 0x200);
        e.map(DYNAMIC_TEXTURES, 0x10);
        quiet_prints(&mut e);
        let persistent = e.mem.alloc(0x80);
        e.mem.set_u32(persistent + 0x74, 3);
        // Sides not powers of two, without the capability: one level.
        let r = renderer(&mut e, 0, 0, 0x7700);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x60, r);
        e.mem.set_u32(this + 0xc, 48);
        e.mem.set_u32(this + 0x10, 32);
        e.mem.set_u32(SKIP_LEVELS, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e6_82b0, &args![this, persistent, 0x15u32])
            .bool());
        let create = calls_to(&e, CREATE_TEXTURE);
        assert_eq!(&create[0][1..7], &[48, 32, 1, 0, 0x15, 1]);
        assert_eq!(e.mem.u8(this + 0x6d), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0x7700);
        // Powers of two, dynamic, a failure: printed with the format.
        let r = renderer(&mut e, 0, 0x8000_0005, 0);
        e.mem.set_u32(this + 0x60, r);
        e.mem.set_u32(this + 0xc, 64);
        e.mem.set_u32(this + 0x10, 32);
        e.mem.set_u8(DYNAMIC_TEXTURES, 1);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e6_82b0, &args![this, persistent, 0x15u32])
            .bool());
        let create = calls_to(&e, CREATE_TEXTURE);
        assert_eq!(&create[0][1..7], &[32, 16, 2, D3DUSAGE_DYNAMIC, 0x15, 0]);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![CREATE_SURF_PERSISTENT_FAILED, 0x15, 0x7fff_0005]]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x78), 1);
    }

    const SURFACE_DESC: u32 = 0x0075_0000;
    const SURFACE_LOCK: u32 = 0x0075_0010;
    const SURFACE_UNLOCK: u32 = 0x0075_0020;

    fn surface(e: &mut Engine, height: u32, bits: u32, pitch: u32, fail: u32) -> u32 {
        e.register_double(SURFACE_DESC, move |e, a| {
            e.mem.set_u32(a[1], 0x15);
            e.mem.set_u32(a[1] + 0x1c, height);
            word(if fail == 1 { 0x8000_0001 } else { 0 })
        });
        e.register_double(SURFACE_LOCK, move |e, a| {
            e.mem.set_u32(a[1], pitch);
            e.mem.set_u32(a[1] + 4, bits);
            word(if fail == 2 { 0x8000_0002 } else { 0 })
        });
        e.register(SURFACE_UNLOCK, |_, _| Ret::default());
        object_with(
            e,
            &[
                (D3D_SURFACE_GET_DESC, SURFACE_DESC),
                (D3D_SURFACE_LOCK_RECT, SURFACE_LOCK),
                (D3D_SURFACE_UNLOCK_RECT, SURFACE_UNLOCK),
            ],
        )
    }

    /// Persistent data with one face of two levels (8 x 4, 4 x 2), 1 byte per pixel.
    fn persistent(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(0x80);
        let widths = e.mem.alloc(8);
        let heights = e.mem.alloc(8);
        let offsets = e.mem.alloc(12);
        e.mem.set_u32(widths, 8);
        e.mem.set_u32(widths + 4, 4);
        e.mem.set_u32(heights, 4);
        e.mem.set_u32(heights + 4, 2);
        e.mem.set_u32(offsets + 4, 32);
        e.mem.set_u32(offsets + 8, 40);
        e.mem.set_u32(p + PERSISTENT_WIDTHS, widths);
        e.mem.set_u32(p + PERSISTENT_HEIGHTS, heights);
        e.mem.set_u32(p + PERSISTENT_OFFSETS, offsets);
        e.mem.set_u32(p + PERSISTENT_LEVELS, 2);
        e.mem.set_u32(p + PERSISTENT_PIXELS, 0x6000);
        e.mem.set_u32(p + PERSISTENT_STRIDE, 1);
        p
    }

    #[test]
    fn persistent_copy_to_surface_level() {
        let mut e = Engine::new();
        quiet_prints(&mut e);
        e.register(MEMCPY, |_, _| Ret::default());
        let p = persistent(&mut e);
        let s = surface(&mut e, 16, 0x9000, 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_83d0, &args![p, 0u32, s, 0u32]);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, 0x6000, 32]]);
        assert_eq!(calls_to(&e, SURFACE_UNLOCK), vec![vec![s]]);
        let s = surface(&mut e, 1, 0x9000, 16, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_83d0, &args![p, 1u32, s, 1u32]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x9000, 0x6000 + 40 + 32, 4]]
        );
        for (fail, message) in [
            (1u32, PERSISTENT_GET_DESC_FAILED),
            (2, PERSISTENT_LOCK_FAILED),
        ] {
            let s = surface(&mut e, 1, 0x9000, 16, fail);
            e.call_log = Some(vec![]);
            e.call(0x00e6_83d0, &args![p, 0u32, s, 0u32]);
            assert_eq!(
                calls_to(&e, DEBUG_PRINT),
                vec![vec![message, 0x7fff_0000 | fail]]
            );
            assert!(calls_to(&e, SURFACE_UNLOCK).is_empty());
        }
    }

    #[test]
    fn texture_slot_setter_and_palette_getter() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x84);
        e.call(0x00e6_87e0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x64), 0x1234);
        e.mem.set_u32(this + 0x74, 0x5678);
        assert_eq!(e.call(0x00e6_8820, &args![this]).u32(), 0x5678);
    }

    const GET_TYPE: u32 = 0x0074_0000;
    const GET_SURFACE: u32 = 0x0074_0010;
    const RELEASE_SURFACE: u32 = 0x0074_0020;

    #[test]
    fn persistent_copy_to_surface_runs_each_level() {
        let mut e = Engine::new();
        quiet_prints(&mut e);
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(RELEASE_SURFACE, |_, _| Ret::default());
        let surface_object = surface(&mut e, 16, 0x9000, 8, 0);
        let vtable = e.mem.u32(surface_object);
        e.mem.set_u32(vtable + D3D_RELEASE, RELEASE_SURFACE);
        e.register_double(GET_SURFACE, move |e, a| {
            e.mem.set_u32(a[2], surface_object);
            word(if a[1] == 1 { 0x8000_0009 } else { 0 })
        });
        let p = persistent(&mut e);
        for (kind, copies) in [(D3DRTYPE_TEXTURE, 1usize), (D3DRTYPE_CUBETEXTURE, 0)] {
            e.register_double(GET_TYPE, move |_, _| word(kind));
            let texture = object_with(
                &mut e,
                &[
                    (D3D_GET_TYPE, GET_TYPE),
                    (D3D_GET_SURFACE_LEVEL, GET_SURFACE),
                ],
            );
            let this = e.mem.alloc(0x84);
            e.mem.set_u32(this + 0x64, texture);
            e.mem.set_u32(this + 0x68, 2);
            e.call_log = Some(vec![]);
            e.call(0x00e6_8920, &args![this, p]);
            assert_eq!(calls_to(&e, MEMCPY).len(), copies);
            if copies == 1 {
                // Level 1's surface fails: printed with the level.
                assert_eq!(
                    calls_to(&e, DEBUG_PRINT),
                    vec![vec![PERSISTENT_GET_SURFACE_LEVEL_FAILED, 1, 0x7fff_0009]]
                );
                assert_eq!(calls_to(&e, RELEASE_SURFACE).len(), 1);
            }
        }
    }

    #[test]
    fn update_converts_only_on_a_change() {
        const CONVERT: u32 = 0x0073_0014;
        let mut e = Engine::new();
        let converter = object_with(&mut e, &[(CONVERTER_CONVERT_SLOT, CONVERT)]);
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| word(converter));
        let converted = e.mem.alloc(0x80);
        let converted_vtable = e.mem.alloc(8);
        e.mem.set_u32(converted, converted_vtable);
        e.mem.set_u32(converted_vtable + 4, 0x0073_0100);
        e.register(0x0073_0100, |_, _| Ret::default());
        e.register_double(CONVERT, move |_, _| word(converted));
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            word(n)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            word(n)
        });
        let pixel_data = e.mem.alloc(0x80);
        e.mem.set_u32(pixel_data + PIXEL_DATA_REVISION, 5);
        let source = e.mem.alloc(0x50);
        e.mem.set_u32(source + SOURCE_PIXEL_DATA, pixel_data);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 8, source);
        e.mem.set_u32(this + 0x7c, 5);
        // Same revision, no palette: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e6_89d0, &args![this]);
        assert!(calls_to(&e, CONVERT).is_empty());
        // A palette with a new revision: converted (no surface yet: the texture data has no
        // Direct3D texture), the new data's reference taken and released (deleted).
        let palette = e.mem.alloc(0x20);
        e.mem.set_u32(palette + PALETTE_REVISION, 9);
        e.mem.set_u32(pixel_data + PIXEL_DATA_PALETTE, palette);
        e.mem.set_u8(this + 0x6d, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e6_89d0, &args![this]);
        assert_eq!(
            calls_to(&e, CONVERT),
            vec![vec![converter, pixel_data, this + 0x14, pixel_data, 1]]
        );
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this + 0x74, palette]]
        );
        assert_eq!(e.mem.u32(this + 0x80), 9);
        assert_eq!(calls_to(&e, 0x0073_0100), vec![vec![converted]]);
        // Replacement data: nothing at all.
        e.mem.set_u8(this + 0x6c, 1);
        e.mem.set_u32(this + 0x7c, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_89d0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
