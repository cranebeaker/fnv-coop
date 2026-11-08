//! Direct3D 9 formats (`D3DFORMAT`) as wgpu formats: how a texture of each is made, how its
//! uploaded bytes are laid out and changed, and how sampling fills the components the format
//! does not have (`fix`, applied by the converted shaders).

/// `D3DFORMAT` values the game can use.
pub mod d3dfmt {
    pub const A8R8G8B8: u32 = 21;
    pub const X8R8G8B8: u32 = 22;
    pub const R5G6B5: u32 = 23;
    pub const X1R5G5B5: u32 = 24;
    pub const A1R5G5B5: u32 = 25;
    pub const A4R4G4B4: u32 = 26;
    pub const A8: u32 = 28;
    pub const X4R4G4B4: u32 = 30;
    pub const A2B10G10R10: u32 = 31;
    pub const A8B8G8R8: u32 = 32;
    pub const X8B8G8R8: u32 = 33;
    pub const G16R16: u32 = 34;
    pub const A16B16G16R16: u32 = 36;
    pub const L8: u32 = 50;
    pub const A8L8: u32 = 51;
    pub const V8U8: u32 = 60;
    pub const Q8W8V8U8: u32 = 63;
    pub const D16_LOCKABLE: u32 = 70;
    pub const D32: u32 = 71;
    pub const D24S8: u32 = 75;
    pub const D24X8: u32 = 77;
    pub const D16: u32 = 80;
    pub const L16: u32 = 81;
    pub const D32F_LOCKABLE: u32 = 82;
    pub const R16F: u32 = 111;
    pub const G16R16F: u32 = 112;
    pub const A16B16G16R16F: u32 = 113;
    pub const R32F: u32 = 114;
    pub const G32R32F: u32 = 115;
    pub const A32B32G32R32F: u32 = 116;
    pub const fn fourcc(s: &[u8; 4]) -> u32 {
        u32::from_le_bytes(*s)
    }
    pub const DXT1: u32 = fourcc(b"DXT1");
    pub const DXT2: u32 = fourcc(b"DXT2");
    pub const DXT3: u32 = fourcc(b"DXT3");
    pub const DXT4: u32 = fourcc(b"DXT4");
    pub const DXT5: u32 = fourcc(b"DXT5");
    pub const ATI1: u32 = fourcc(b"ATI1");
    pub const ATI2: u32 = fourcc(b"ATI2");
    pub const INTZ: u32 = fourcc(b"INTZ");
    pub const NULL: u32 = fourcc(b"NULL");
}

/// How sampling fills a format's missing components (the converted shaders' `fixup` modes).
pub mod fix {
    /// As sampled.
    pub const NONE: u32 = 0;
    /// (r, 1, 1, 1): one-channel float formats.
    pub const R: u32 = 1;
    /// (r, g, 1, 1): two-channel formats.
    pub const RG: u32 = 2;
    /// Alpha 1: X8R8G8B8 and the other formats without alpha.
    pub const OPAQUE: u32 = 3;
    /// (0, 0, 0, r): A8 stored in one channel.
    pub const ALPHA: u32 = 4;
    /// (r, r, r, 1): L8, L16.
    pub const LUMINANCE: u32 = 5;
    /// (r, r, r, g): A8L8.
    pub const LUMINANCE_ALPHA: u32 = 6;
}

/// How the bytes the game uploads become the texture's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Upload {
    /// As they are.
    Copy,
    /// 16-bit pixels widened to B8G8R8A8 (`R5G6B5` and the 1-5-5-5 and 4-4-4-4 formats).
    Widen16(u32),
}

/// A Direct3D 9 format on wgpu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Format {
    pub d3d: u32,
    pub wgpu: wgpu::TextureFormat,
    pub upload: Upload,
    pub fix: u32,
    /// Bytes per pixel, or per 4x4 block for block-compressed formats.
    pub bytes: u32,
    pub block: bool,
    pub depth: bool,
    /// Has no alpha: render targets keep alpha at 1 (writes masked, clears at 1).
    pub opaque: bool,
}

/// The wgpu format of a Direct3D 9 one, or `None` for one the layer does not provide.
pub fn format(d3d: u32) -> Option<Format> {
    use d3dfmt::*;
    use wgpu::TextureFormat as T;
    let f = |wgpu, upload, fix, bytes, block, depth, opaque| {
        Some(Format {
            d3d,
            wgpu,
            upload,
            fix,
            bytes,
            block,
            depth,
            opaque,
        })
    };
    let c = Upload::Copy;
    match d3d {
        A8R8G8B8 => f(T::Bgra8Unorm, c, fix::NONE, 4, false, false, false),
        X8R8G8B8 => f(T::Bgra8Unorm, c, fix::OPAQUE, 4, false, false, true),
        A8B8G8R8 => f(T::Rgba8Unorm, c, fix::NONE, 4, false, false, false),
        X8B8G8R8 => f(T::Rgba8Unorm, c, fix::OPAQUE, 4, false, false, true),
        R5G6B5 | X1R5G5B5 | X4R4G4B4 => f(
            T::Bgra8Unorm,
            Upload::Widen16(d3d),
            fix::OPAQUE,
            2,
            false,
            false,
            true,
        ),
        A1R5G5B5 | A4R4G4B4 => f(
            T::Bgra8Unorm,
            Upload::Widen16(d3d),
            fix::NONE,
            2,
            false,
            false,
            false,
        ),
        A8 => f(T::R8Unorm, c, fix::ALPHA, 1, false, false, false),
        L8 => f(T::R8Unorm, c, fix::LUMINANCE, 1, false, false, true),
        A8L8 => f(T::Rg8Unorm, c, fix::LUMINANCE_ALPHA, 2, false, false, false),
        L16 => f(T::R16Unorm, c, fix::LUMINANCE, 2, false, false, true),
        V8U8 => f(T::Rg8Snorm, c, fix::RG, 2, false, false, true),
        Q8W8V8U8 => f(T::Rgba8Snorm, c, fix::NONE, 4, false, false, false),
        A2B10G10R10 => f(T::Rgb10a2Unorm, c, fix::NONE, 4, false, false, false),
        G16R16 => f(T::Rg16Unorm, c, fix::RG, 4, false, false, true),
        A16B16G16R16 => f(T::Rgba16Unorm, c, fix::NONE, 8, false, false, false),
        R16F => f(T::R16Float, c, fix::R, 2, false, false, true),
        G16R16F => f(T::Rg16Float, c, fix::RG, 4, false, false, true),
        A16B16G16R16F => f(T::Rgba16Float, c, fix::NONE, 8, false, false, false),
        R32F => f(T::R32Float, c, fix::R, 4, false, false, true),
        G32R32F => f(T::Rg32Float, c, fix::RG, 8, false, false, true),
        A32B32G32R32F => f(T::Rgba32Float, c, fix::NONE, 16, false, false, false),
        DXT1 => f(T::Bc1RgbaUnorm, c, fix::NONE, 8, true, false, false),
        DXT2 | DXT3 => f(T::Bc2RgbaUnorm, c, fix::NONE, 16, true, false, false),
        DXT4 | DXT5 => f(T::Bc3RgbaUnorm, c, fix::NONE, 16, true, false, false),
        ATI1 => f(T::Bc4RUnorm, c, fix::LUMINANCE, 8, true, false, true),
        ATI2 => f(T::Bc5RgUnorm, c, fix::RG, 16, true, false, true),
        D16 | D16_LOCKABLE => f(T::Depth16Unorm, c, fix::R, 2, false, true, true),
        D24X8 => f(T::Depth24Plus, c, fix::R, 4, false, true, true),
        D24S8 | INTZ => f(T::Depth24PlusStencil8, c, fix::R, 4, false, true, true),
        D32 | D32F_LOCKABLE => f(T::Depth32Float, c, fix::R, 4, false, true, true),
        _ => None,
    }
}

/// The bytes of a level `width` x `height` as the game lays them out (rows packed): bytes per
/// row and rows (block rows for block-compressed formats).
pub fn level_layout(f: &Format, width: u32, height: u32) -> (u32, u32) {
    if f.block {
        (width.div_ceil(4) * f.bytes, height.div_ceil(4))
    } else {
        (width * f.bytes, height)
    }
}

/// The game's bytes of one level as the texture takes them (`Upload::Widen16` widens 16-bit
/// pixels to B8G8R8A8; the rest are copied).
pub fn convert<'a>(f: &Format, bytes: &'a [u8]) -> std::borrow::Cow<'a, [u8]> {
    match f.upload {
        Upload::Copy => std::borrow::Cow::Borrowed(bytes),
        Upload::Widen16(d3d) => {
            let mut out = Vec::with_capacity(bytes.len() * 2);
            for p in bytes.chunks_exact(2) {
                let v = u16::from_le_bytes([p[0], p[1]]) as u32;
                let (b, g, r, a) = match d3d {
                    d3dfmt::R5G6B5 => (
                        scale(v & 0x1F, 5),
                        scale((v >> 5) & 0x3F, 6),
                        scale(v >> 11, 5),
                        255,
                    ),
                    d3dfmt::X1R5G5B5 | d3dfmt::A1R5G5B5 => (
                        scale(v & 0x1F, 5),
                        scale((v >> 5) & 0x1F, 5),
                        scale((v >> 10) & 0x1F, 5),
                        if d3d == d3dfmt::X1R5G5B5 || v & 0x8000 != 0 {
                            255
                        } else {
                            0
                        },
                    ),
                    _ => (
                        scale(v & 0xF, 4),
                        scale((v >> 4) & 0xF, 4),
                        scale((v >> 8) & 0xF, 4),
                        if d3d == d3dfmt::X4R4G4B4 {
                            255
                        } else {
                            scale(v >> 12, 4)
                        },
                    ),
                };
                out.extend_from_slice(&[b as u8, g as u8, r as u8, a as u8]);
            }
            std::borrow::Cow::Owned(out)
        }
    }
}

/// An `n`-bit unsigned normalized value as 8 bits (bit replication, exact at 0 and the top).
fn scale(v: u32, n: u32) -> u32 {
    (v * 255 + ((1 << n) - 1) / 2) / ((1 << n) - 1)
}

/// The texture's bytes per row and rows once converted (what `write_texture` is given).
pub fn uploaded_layout(f: &Format, width: u32, height: u32) -> (u32, u32) {
    let (row, rows) = level_layout(f, width, height);
    match f.upload {
        Upload::Widen16(_) => (row * 2, rows),
        Upload::Copy => (row, rows),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_bit_pixels_widen_exactly_at_the_ends() {
        let f = format(d3dfmt::R5G6B5).unwrap();
        let white = 0xFFFFu16.to_le_bytes();
        let red = 0xF800u16.to_le_bytes();
        let bytes = [white, red].concat();
        assert_eq!(&*convert(&f, &bytes), &[255, 255, 255, 255, 0, 0, 255, 255]);
        let a = format(d3dfmt::A4R4G4B4).unwrap();
        assert_eq!(&*convert(&a, &0x7F00u16.to_le_bytes()), &[0, 0, 255, 119]);
    }

    #[test]
    fn block_formats_lay_out_by_block_rows() {
        let f = format(d3dfmt::DXT5).unwrap();
        assert_eq!(level_layout(&f, 256, 128), (64 * 16, 32));
        assert_eq!(level_layout(&f, 2, 1), (16, 1));
        let d = format(d3dfmt::DXT1).unwrap();
        assert_eq!(level_layout(&d, 8, 8), (16, 2));
    }

    #[test]
    fn the_back_buffer_format_has_no_alpha() {
        let f = format(d3dfmt::X8R8G8B8).unwrap();
        assert!(f.opaque);
        assert_eq!(f.wgpu, wgpu::TextureFormat::Bgra8Unorm);
        assert_eq!(f.fix, fix::OPAQUE);
    }
}
