//! The texture loading the game asks of D3DX9 (`d3dx9_38.dll`), translated: the game calls
//! `D3DXGetImageInfoFromFileInMemory`, then `D3DXCreateTextureFromFileInMemory` or
//! `D3DXCreateCubeTextureFromFileInMemory` by the image's resource type (`FalloutNV.exe`
//! 00e68a80), always with D3DX's defaults. This crate gives the texture D3DX makes for those
//! calls (level count, each level's format, size and bytes, rows packed) without D3DX or any
//! device, byte for byte as the 32-bit DLL makes it on the calling thread: the DLL computes in
//! that thread's x87 precision, and the same file gives other bytes at 24 bits (the thread that
//! made the Direct3D device, the game's main thread) than at 53 bits (a thread as Windows starts
//! it: the game's loading threads, `BSTaskThread` 00c42da0, which made about four in five of
//! the texture calls in the recordings of Doc's house). The caller says which ([`Precision`]).
//!
//! The pieces, each translated from the 32-bit DLL (internal; `dxt`, `filter`, `num` are public
//! only for the verification tools): reading a DDS file (`dds`), a format's pixels to and from
//! D3DX's working colour (`pixel`), D3DX's DXT1/3/5 block decoder and encoder (`dxt`), its box
//! filter for the mip levels it makes (`filter`), what the create calls do with all of it
//! (`load`), and the DLL's arithmetic: x87 at either precision, what it keeps in registers and
//! what it stores, its operation order, and the truncating rounding its encoders run under
//! (`num`).
//!
//! Checked (2026-11-08): all 21,776 textures of the game's archives give the bytes the 32-bit DLL
//! gives, at both precisions (`scripts/d3dx-tex-verify` against the DLL's answers captured with
//! `scripts/d3dx-texture-capture`: on the device's thread, and with `thread` on a new thread at
//! 53 bits), with the processor's truncation and with the software one. At 53 bits 1,873 of them
//! differ from the 24-bit bytes. The tests compare with the DLL's answers for synthetic files and
//! blocks at both precisions (`data/`).
//!
//! What D3DX also does and this crate does not (yet) is [`Error::NotProvided`]: image files
//! other than DDS, volume textures, DDS pixel formats other than DXT1/3/5, A8R8G8B8, X8R8G8B8,
//! R8G8B8 and A4R4G4B4, and a top level D3DX resamples (a size that is not a power of two, a
//! DXT side that is not a multiple of 4; its triangle filter), each with its kind
//! ([`Untranslated`]). None of the game's textures needs them.
//!
//! Speed (2026-11-08, all the game's textures, `d3dx-tex-verify --time` against the 64-bit DLL
//! creating on a real device and reading back): on one core about 10% faster than the 64-bit DLL
//! at single precision and about 15% slower at double (each double-precision store is a
//! conversion); with a large level's DXT blocks split across cores ([`set_max_threads`], a
//! budget shared by every loading thread) about twice as fast at either. The bytes are the same.
//! Malformed files get D3DX's own answers (its 32-bit size arithmetic, measured on such headers),
//! and a refusal where the 32-bit DLL crashes, before anything is allocated from their headers.

pub(crate) mod dds;
#[doc(hidden)]
pub mod dxt;
#[doc(hidden)]
pub mod filter;
pub mod load;
#[doc(hidden)]
pub mod num;
pub(crate) mod pixel;
#[doc(hidden)]
pub mod synthetic;

pub use filter::set_max_threads;
pub use load::{
    create_cube_texture_from_file_in_memory, create_texture_from_file_in_memory,
    get_image_info_from_file_in_memory, Caps, Error, ImageInfo, Surface, Texture, Untranslated,
};
pub use num::Precision;

/// `D3DXERR_INVALIDDATA`.
pub const D3DXERR_INVALIDDATA: u32 = 0x8876_0b59;
/// `D3DERR_INVALIDCALL`.
pub const D3DERR_INVALIDCALL: u32 = 0x8876_086c;
/// `D3DERR_NOTAVAILABLE`.
pub const D3DERR_NOTAVAILABLE: u32 = 0x8876_086a;
/// `E_OUTOFMEMORY`.
pub const E_OUTOFMEMORY: u32 = 0x8007_000e;

/// `D3DFORMAT` values this crate names.
pub mod format {
    pub const UNKNOWN: u32 = 0;
    pub const R8G8B8: u32 = 20;
    pub const A8R8G8B8: u32 = 21;
    pub const X8R8G8B8: u32 = 22;
    pub const R5G6B5: u32 = 23;
    pub const X1R5G5B5: u32 = 24;
    pub const A1R5G5B5: u32 = 25;
    pub const A4R4G4B4: u32 = 26;
    pub const R3G3B2: u32 = 27;
    pub const A8: u32 = 28;
    pub const A8R3G3B2: u32 = 29;
    pub const X4R4G4B4: u32 = 30;
    pub const A2B10G10R10: u32 = 31;
    pub const A8B8G8R8: u32 = 32;
    pub const X8B8G8R8: u32 = 33;
    pub const G16R16: u32 = 34;
    pub const A2R10G10B10: u32 = 35;
    pub const A16B16G16R16: u32 = 36;
    pub const A8P8: u32 = 40;
    pub const P8: u32 = 41;
    pub const L8: u32 = 50;
    pub const A8L8: u32 = 51;
    pub const A4L4: u32 = 52;
    pub const V8U8: u32 = 60;
    pub const L6V5U5: u32 = 61;
    pub const X8L8V8U8: u32 = 62;
    pub const Q8W8V8U8: u32 = 63;
    pub const V16U16: u32 = 64;
    pub const A2W10V10U10: u32 = 67;
    pub const L16: u32 = 81;
    pub const Q16W16V16U16: u32 = 110;
    pub const R16F: u32 = 111;
    pub const G16R16F: u32 = 112;
    pub const A16B16G16R16F: u32 = 113;
    pub const R32F: u32 = 114;
    pub const G32R32F: u32 = 115;
    pub const A32B32G32R32F: u32 = 116;
    pub const DXT1: u32 = u32::from_le_bytes(*b"DXT1");
    pub const DXT2: u32 = u32::from_le_bytes(*b"DXT2");
    pub const DXT3: u32 = u32::from_le_bytes(*b"DXT3");
    pub const DXT4: u32 = u32::from_le_bytes(*b"DXT4");
    pub const DXT5: u32 = u32::from_le_bytes(*b"DXT5");

    /// Whether `f` is one of the DXT block formats.
    pub(crate) fn is_dxt(f: u32) -> bool {
        matches!(f, DXT1 | DXT2 | DXT3 | DXT4 | DXT5)
    }

    /// The block side in pixels and the bytes per block (per pixel for the others), for the
    /// formats this crate reads or makes; `None` for others.
    pub(crate) fn layout(f: u32) -> Option<(u32, u32)> {
        Some(match f {
            DXT1 => (4, 8),
            DXT2 | DXT3 | DXT4 | DXT5 => (4, 16),
            R8G8B8 => (1, 3),
            A8R8G8B8 | X8R8G8B8 | A2B10G10R10 | A8B8G8R8 | X8B8G8R8 | G16R16 | A2R10G10B10
            | X8L8V8U8 | Q8W8V8U8 | V16U16 | A2W10V10U10 | G16R16F | R32F => (1, 4),
            R5G6B5 | X1R5G5B5 | A1R5G5B5 | A4R4G4B4 | A8R3G3B2 | X4R4G4B4 | A8P8 | A8L8 | V8U8
            | L6V5U5 | L16 | R16F => (1, 2),
            R3G3B2 | A8 | P8 | L8 | A4L4 => (1, 1),
            A16B16G16R16 | Q16W16V16U16 | A16B16G16R16F | G32R32F => (1, 8),
            A32B32G32R32F => (1, 16),
            _ => return None,
        })
    }

    /// Bytes of a `w` x `h` level of `f`, rows packed.
    pub(crate) fn level_size(f: u32, w: u32, h: u32) -> usize {
        let (block, bytes) = layout(f).expect("a format with a known layout");
        w.div_ceil(block) as usize * h.div_ceil(block) as usize * bytes as usize
    }
}
