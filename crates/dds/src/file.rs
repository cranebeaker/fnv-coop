//! DDS header parsing and pixel access.

use crate::bc;
use crate::error::{Error, Result};

const MAGIC: &[u8; 4] = b"DDS ";
const HEADER_LEN: usize = 4 + 124;
const DX10_HEADER_LEN: usize = 20;

const DDSD_MIPMAPCOUNT: u32 = 0x2_0000;
const DDSD_DEPTH: u32 = 0x80_0000;
const DDPF_ALPHAPIXELS: u32 = 0x1;
const DDPF_ALPHA: u32 = 0x2;
const DDPF_FOURCC: u32 = 0x4;
const DDPF_RGB: u32 = 0x40;
const DDPF_LUMINANCE: u32 = 0x2_0000;
const DDSCAPS2_CUBEMAP: u32 = 0x200;
const DDSCAPS2_CUBEMAP_FACES: [u32; 6] = [0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000];
const DDSCAPS2_VOLUME: u32 = 0x20_0000;
const MAX_DIMENSION: u32 = 16_384;

/// Bit layout of an uncompressed format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelMasks {
    pub bits: u32,
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
    /// Single luminance channel (stored in the red mask) shown as grey.
    pub luminance: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// DXT1: RGB with optional 1-bit alpha, 8 bytes per 4x4 block.
    Bc1,
    /// DXT3 (DXT2 if premultiplied): explicit 4-bit alpha.
    Bc2 {
        premultiplied: bool,
    },
    /// DXT5 (DXT4 if premultiplied): interpolated alpha.
    Bc3 {
        premultiplied: bool,
    },
    /// ATI1: one channel.
    Bc4,
    /// ATI2: two channels, used for normal maps.
    Bc5,
    Uncompressed(PixelMasks),
}

impl Format {
    pub fn is_block_compressed(&self) -> bool {
        !matches!(self, Format::Uncompressed(_))
    }

    /// Bytes per 4x4 block, or per pixel for uncompressed formats.
    fn unit_size(&self) -> usize {
        match self {
            Format::Bc1 | Format::Bc4 => 8,
            Format::Bc2 { .. } | Format::Bc3 { .. } | Format::Bc5 => 16,
            Format::Uncompressed(m) => (m.bits / 8) as usize,
        }
    }

    /// A short name such as `DXT5` or `A8R8G8B8`.
    pub fn name(&self) -> String {
        match *self {
            Format::Bc1 => "DXT1".into(),
            Format::Bc2 { premultiplied } => (if premultiplied { "DXT2" } else { "DXT3" }).into(),
            Format::Bc3 { premultiplied } => (if premultiplied { "DXT4" } else { "DXT5" }).into(),
            Format::Bc4 => "ATI1 (BC4)".into(),
            Format::Bc5 => "ATI2 (BC5)".into(),
            Format::Uncompressed(m) => {
                let known = [
                    (32, 0x00FF_0000, 0xFF00, 0xFF, 0xFF00_0000, "A8R8G8B8"),
                    (32, 0x00FF_0000, 0xFF00, 0xFF, 0, "X8R8G8B8"),
                    (32, 0xFF, 0xFF00, 0x00FF_0000, 0xFF00_0000, "A8B8G8R8"),
                    (32, 0xFF, 0xFF00, 0x00FF_0000, 0, "X8B8G8R8"),
                    (24, 0x00FF_0000, 0xFF00, 0xFF, 0, "R8G8B8"),
                    (16, 0xF800, 0x07E0, 0x1F, 0, "R5G6B5"),
                    (16, 0x7C00, 0x03E0, 0x1F, 0x8000, "A1R5G5B5"),
                    (16, 0x7C00, 0x03E0, 0x1F, 0, "X1R5G5B5"),
                    (16, 0x0F00, 0x00F0, 0x0F, 0xF000, "A4R4G4B4"),
                ];
                if m.luminance {
                    return (if m.a != 0 {
                        format!("A{0}L{0}", m.bits / 2)
                    } else {
                        format!("L{}", m.bits)
                    })
                    .to_string();
                }
                if m.r == 0 && m.g == 0 && m.b == 0 {
                    return format!("A{}", m.bits);
                }
                known
                    .iter()
                    .find(|k| (k.0, k.1, k.2, k.3, k.4) == (m.bits, m.r, m.g, m.b, m.a))
                    .map_or_else(|| format!("{}-bit RGB", m.bits), |k| k.5.to_string())
            }
        }
    }
}

/// A parsed DDS texture.
pub struct Dds {
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    format: Format,
    /// Mip levels declared in the header.
    declared_mips: usize,
    /// Mip levels fully present in the data (may be fewer than declared).
    mips: usize,
    faces: usize,
    data_start: usize,
}

fn u32_at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

impl Dds {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let mut found = [0u8; 4];
        let n = bytes.len().min(4);
        found[..n].copy_from_slice(&bytes[..n]);
        if &found != MAGIC {
            return Err(Error::NotADds { found });
        }
        if bytes.len() < HEADER_LEN {
            return Err(Error::Malformed(format!(
                "the file is {} bytes, shorter than the {HEADER_LEN}-byte header",
                bytes.len()
            )));
        }
        let h = &bytes[4..HEADER_LEN];
        let flags = u32_at(h, 4);
        let height = u32_at(h, 8);
        let width = u32_at(h, 12);
        let declared_mips = if flags & DDSD_MIPMAPCOUNT != 0 {
            u32_at(h, 24).max(1)
        } else {
            1
        } as usize;
        let pf_flags = u32_at(h, 76);
        let four_cc = [h[80], h[81], h[82], h[83]];
        let masks = PixelMasks {
            bits: u32_at(h, 84),
            r: u32_at(h, 88),
            g: u32_at(h, 92),
            b: u32_at(h, 96),
            a: if pf_flags & (DDPF_ALPHAPIXELS | DDPF_ALPHA) != 0 {
                u32_at(h, 100)
            } else {
                0
            },
            luminance: pf_flags & DDPF_LUMINANCE != 0,
        };
        let caps2 = u32_at(h, 108);

        if width == 0 || height == 0 || width > MAX_DIMENSION || height > MAX_DIMENSION {
            return Err(Error::Malformed(format!("unusable size {width}x{height}")));
        }
        if caps2 & DDSCAPS2_VOLUME != 0 || (flags & DDSD_DEPTH != 0 && u32_at(h, 20) > 1) {
            return Err(Error::Unsupported("volume (3D) textures".into()));
        }

        let mut data_start = HEADER_LEN;
        let format = if pf_flags & DDPF_FOURCC != 0 {
            match &four_cc {
                b"DXT1" => Format::Bc1,
                b"DXT2" => Format::Bc2 {
                    premultiplied: true,
                },
                b"DXT3" => Format::Bc2 {
                    premultiplied: false,
                },
                b"DXT4" => Format::Bc3 {
                    premultiplied: true,
                },
                b"DXT5" => Format::Bc3 {
                    premultiplied: false,
                },
                b"ATI1" | b"BC4U" => Format::Bc4,
                b"ATI2" | b"BC5U" => Format::Bc5,
                b"DX10" => {
                    let ext = bytes
                        .get(HEADER_LEN..HEADER_LEN + DX10_HEADER_LEN)
                        .ok_or_else(|| {
                            Error::Malformed("the DX10 extension header is missing".into())
                        })?;
                    data_start += DX10_HEADER_LEN;
                    dx10_format(u32_at(ext, 0))?
                }
                other => {
                    return Err(Error::Unsupported(format!(
                        "compression format {:?}",
                        String::from_utf8_lossy(other)
                    )))
                }
            }
        } else if pf_flags & (DDPF_RGB | DDPF_LUMINANCE | DDPF_ALPHA) != 0 {
            if !matches!(masks.bits, 8 | 16 | 24 | 32) {
                return Err(Error::Unsupported(format!("{}-bit pixels", masks.bits)));
            }
            Format::Uncompressed(fix_luminance_masks(masks))
        } else {
            return Err(Error::Unsupported(format!(
                "pixel format flags {pf_flags:#x}"
            )));
        };

        let faces = if caps2 & DDSCAPS2_CUBEMAP != 0 {
            DDSCAPS2_CUBEMAP_FACES
                .iter()
                .filter(|&&f| caps2 & f != 0)
                .count()
                .max(1)
        } else {
            1
        };

        let mut dds = Dds {
            bytes,
            width,
            height,
            format,
            declared_mips,
            mips: 0,
            faces,
            data_start,
        };
        // Count the mip levels actually present for every face.
        let available = dds.bytes.len() - data_start;
        let mut chain = 0usize;
        for level in 0..declared_mips.min(32) {
            let next = chain + dds.level_size(level);
            if next * faces > available {
                break;
            }
            chain = next;
            dds.mips = level + 1;
        }
        if dds.mips == 0 {
            return Err(Error::Malformed(format!(
                "the {}x{} {} image needs {} bytes but only {available} follow the header",
                width,
                height,
                format.name(),
                dds.level_size(0) * faces
            )));
        }
        Ok(dds)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> Format {
        self.format
    }

    /// Mip levels present in the data.
    pub fn mip_count(&self) -> usize {
        self.mips
    }

    /// Mip levels the header claims; more than [`Dds::mip_count`] means the
    /// file is missing its smallest levels.
    pub fn declared_mip_count(&self) -> usize {
        self.declared_mips
    }

    /// 6 for a full cube map, 1 for an ordinary texture.
    pub fn face_count(&self) -> usize {
        self.faces
    }

    pub fn is_cube_map(&self) -> bool {
        self.faces > 1
    }

    pub fn level_dimensions(&self, level: usize) -> (u32, u32) {
        let shift = level.min(31) as u32;
        ((self.width >> shift).max(1), (self.height >> shift).max(1))
    }

    /// Size in bytes of one mip level of one face.
    pub fn level_size(&self, level: usize) -> usize {
        let (w, h) = self.level_dimensions(level);
        let (w, h) = (w as usize, h as usize);
        if self.format.is_block_compressed() {
            w.div_ceil(4) * h.div_ceil(4) * self.format.unit_size()
        } else {
            w * h * self.format.unit_size()
        }
    }

    /// Raw bytes of one face's mip level, ready for GPU upload in
    /// block-compressed formats. Faces are stored one after another, each
    /// with its whole mip chain.
    pub fn level_data(&self, face: usize, level: usize) -> Result<&[u8]> {
        if face >= self.faces || level >= self.mips {
            return Err(Error::Malformed(format!(
                "face {face}, level {level} requested; the texture has {} faces and {} levels",
                self.faces, self.mips
            )));
        }
        let chain: usize = (0..self.mips).map(|l| self.level_size(l)).sum();
        let before: usize = (0..level).map(|l| self.level_size(l)).sum();
        let start = self.data_start + face * chain + before;
        Ok(&self.bytes[start..start + self.level_size(level)])
    }

    /// Decodes one face's mip level to RGBA, 4 bytes per pixel, rows top
    /// to bottom.
    pub fn decode_rgba(&self, face: usize, level: usize) -> Result<Vec<u8>> {
        let data = self.level_data(face, level)?;
        let (w, h) = self.level_dimensions(level);
        let (w, h) = (w as usize, h as usize);
        let mut out = vec![0u8; w * h * 4];
        match self.format {
            Format::Uncompressed(masks) => decode_uncompressed(data, &masks, &mut out),
            format => {
                let decode: fn(&[u8]) -> bc::Block = match format {
                    Format::Bc1 => bc::bc1,
                    Format::Bc2 { .. } => bc::bc2,
                    Format::Bc3 { .. } => bc::bc3,
                    Format::Bc4 => bc::bc4,
                    _ => bc::bc5,
                };
                let unit = format.unit_size();
                let blocks_wide = w.div_ceil(4);
                for (i, chunk) in data.chunks_exact(unit).enumerate() {
                    let (bx, by) = (i % blocks_wide * 4, i / blocks_wide * 4);
                    let block = decode(chunk);
                    for (p, px) in block.iter().enumerate() {
                        let (x, y) = (bx + p % 4, by + p / 4);
                        if x < w && y < h {
                            let o = (y * w + x) * 4;
                            out[o..o + 4].copy_from_slice(px);
                        }
                    }
                }
            }
        }
        Ok(out)
    }
}

fn dx10_format(dxgi: u32) -> Result<Format> {
    let rgba = |r, g, b, a| {
        Format::Uncompressed(PixelMasks {
            bits: 32,
            r,
            g,
            b,
            a,
            luminance: false,
        })
    };
    // Typeless formats are read as their unsigned-normalized versions.
    Ok(match dxgi {
        70..=72 => Format::Bc1,
        73..=75 => Format::Bc2 {
            premultiplied: false,
        },
        76..=78 => Format::Bc3 {
            premultiplied: false,
        },
        79 | 80 => Format::Bc4,
        82 | 83 => Format::Bc5,
        81 | 84 => {
            return Err(Error::Unsupported(
                "signed BC4/BC5 textures (DXGI formats 81 and 84)".into(),
            ))
        }
        28 | 29 => rgba(0xFF, 0xFF00, 0x00FF_0000, 0xFF00_0000),
        87 | 91 => rgba(0x00FF_0000, 0xFF00, 0xFF, 0xFF00_0000),
        88 | 93 => rgba(0x00FF_0000, 0xFF00, 0xFF, 0),
        other => {
            return Err(Error::Unsupported(format!(
                "DXGI format {other} in a DX10 header"
            )))
        }
    })
}

/// Some writers store luminance masks that don't fit the pixel size. The
/// standard layouts are L8 (luminance in byte 0) and A8L8 (alpha in byte 1),
/// so masks outside the pixel fall back to those.
fn fix_luminance_masks(mut m: PixelMasks) -> PixelMasks {
    if !m.luminance {
        return m;
    }
    let fits = |mask: u32| m.bits >= 32 || mask >> m.bits == 0;
    if m.r == 0 || !fits(m.r) {
        m.r = 0xFF;
    }
    if m.a != 0 && !fits(m.a) {
        m.a = if m.bits >= 16 { 0xFF00 } else { 0 };
    }
    m
}

/// Scales a masked channel value to 0-255.
fn channel(pixel: u32, mask: u32) -> Option<u8> {
    if mask == 0 {
        return None;
    }
    let shift = mask.trailing_zeros();
    let max = u64::from(mask >> shift);
    let value = u64::from((pixel & mask) >> shift);
    Some(((value * 255 + max / 2) / max) as u8)
}

fn decode_uncompressed(data: &[u8], m: &PixelMasks, out: &mut [u8]) {
    let step = (m.bits / 8) as usize;
    for (px, chunk) in out.chunks_exact_mut(4).zip(data.chunks_exact(step)) {
        let mut raw = [0u8; 4];
        raw[..step].copy_from_slice(chunk);
        let pixel = u32::from_le_bytes(raw);
        let a = channel(pixel, m.a).unwrap_or(255);
        if m.luminance {
            let l = channel(pixel, m.r).unwrap_or(0);
            px.copy_from_slice(&[l, l, l, a]);
        } else if m.r == 0 && m.g == 0 && m.b == 0 {
            // Alpha-only textures: show the alpha as grey.
            px.copy_from_slice(&[a, a, a, 255]);
        } else {
            px.copy_from_slice(&[
                channel(pixel, m.r).unwrap_or(0),
                channel(pixel, m.g).unwrap_or(0),
                channel(pixel, m.b).unwrap_or(0),
                a,
            ]);
        }
    }
}
