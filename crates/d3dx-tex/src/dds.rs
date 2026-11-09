//! Reading a DDS file, as D3DX's DDS loader does (`d3dx9_38.dll` 0x621052, called from its image
//! reader 0x622d99, which tries the image file formats in turn).
//!
//! What it checks and gives:
//! - at least 4 + 124 bytes and the magic `DDS `;
//! - width (+16), height (+12); depth (+24) only when `DDSD_DEPTH` is set, 0 counting as 1;
//! - a cube map when all six face bits of `dwCaps2` are set (the `DDSCAPS2_CUBEMAP` bit itself is
//!   not looked at); some face bits but not all: refused;
//! - the mip count (+28), 0 counting as 1, whatever `DDSD_MIPMAPCOUNT` says;
//! - the pixel format by D3DX's table ([`PIXEL_FORMATS`], `pfSize` must be 32; with
//!   `DDPF_FOURCC` set every other flag is dropped);
//! - every level of every face must be in the data, by the loader's own 32-bit sizes, or the
//!   file is refused.
//!
//! A refused file is tried by D3DX's other image readers and ends as `D3DXERR_INVALIDDATA`
//! ([`crate::D3DXERR_INVALIDDATA`]) for anything that is not one of those formats.

use crate::format;

/// One row of D3DX's DDS pixel format table (0x787758): the `D3DFORMAT`, the `DDPF_*` flags, the
/// FourCC, the bit count and the red, green, blue and alpha masks.
pub struct PixelFormat {
    pub format: u32,
    pub flags: u32,
    pub fourcc: u32,
    pub bits: u32,
    pub masks: [u32; 4],
}

const fn pf(format: u32, flags: u32, fourcc: u32, bits: u32, masks: [u32; 4]) -> PixelFormat {
    PixelFormat {
        format,
        flags,
        fourcc,
        bits,
        masks,
    }
}

const fn cc(f: u32) -> PixelFormat {
    pf(f, 4, f, 0, [0; 4])
}

/// D3DX's table, in its order (the first match wins).
pub const PIXEL_FORMATS: &[PixelFormat] = &[
    pf(format::R8G8B8, 0x40, 0, 24, [0xff_0000, 0xff00, 0xff, 0]),
    pf(
        format::A8R8G8B8,
        0x41,
        0,
        32,
        [0xff_0000, 0xff00, 0xff, 0xff00_0000],
    ),
    pf(format::X8R8G8B8, 0x40, 0, 32, [0xff_0000, 0xff00, 0xff, 0]),
    pf(format::R5G6B5, 0x40, 0, 16, [0xf800, 0x7e0, 0x1f, 0]),
    pf(format::X1R5G5B5, 0x40, 0, 16, [0x7c00, 0x3e0, 0x1f, 0]),
    pf(format::A1R5G5B5, 0x41, 0, 16, [0x7c00, 0x3e0, 0x1f, 0x8000]),
    pf(format::A4R4G4B4, 0x41, 0, 16, [0xf00, 0xf0, 0xf, 0xf000]),
    pf(format::R3G3B2, 0x40, 0, 8, [0xe0, 0x1c, 0x3, 0]),
    pf(format::A8, 0x2, 0, 8, [0, 0, 0, 0xff]),
    pf(format::A8R3G3B2, 0x41, 0, 16, [0xe0, 0x1c, 0x3, 0xff00]),
    pf(format::X4R4G4B4, 0x40, 0, 16, [0xf00, 0xf0, 0xf, 0]),
    pf(
        format::A2B10G10R10,
        0x41,
        0,
        32,
        [0x3ff0_0000, 0xffc00, 0x3ff, 0xc000_0000],
    ),
    pf(
        format::A8B8G8R8,
        0x41,
        0,
        32,
        [0xff, 0xff00, 0xff_0000, 0xff00_0000],
    ),
    pf(format::X8B8G8R8, 0x40, 0, 32, [0xff, 0xff00, 0xff_0000, 0]),
    pf(format::G16R16, 0x40, 0, 32, [0xffff, 0xffff_0000, 0, 0]),
    pf(
        format::A2R10G10B10,
        0x41,
        0,
        32,
        [0x3ff, 0xffc00, 0x3ff0_0000, 0xc000_0000],
    ),
    pf(format::A8P8, 0x21, 0, 16, [0, 0, 0, 0xff00]),
    pf(format::P8, 0x20, 0, 8, [0; 4]),
    pf(format::L8, 0x2_0000, 0, 8, [0xff, 0, 0, 0]),
    pf(format::A8L8, 0x2_0001, 0, 16, [0xff, 0, 0, 0xff00]),
    pf(format::A4L4, 0x2_0001, 0, 8, [0xf, 0, 0, 0xf0]),
    pf(format::L16, 0x2_0000, 0, 16, [0xffff, 0, 0, 0]),
    pf(format::V8U8, 0x8_0000, 0, 16, [0xff, 0xff00, 0, 0]),
    pf(format::L6V5U5, 0x4_0000, 0, 16, [0x1f, 0x3e0, 0xfc00, 0]),
    pf(
        format::X8L8V8U8,
        0x4_0000,
        0,
        32,
        [0xff, 0xff00, 0xff_0000, 0],
    ),
    pf(
        format::Q8W8V8U8,
        0x8_0000,
        0,
        32,
        [0xff, 0xff00, 0xff_0000, 0xff00_0000],
    ),
    pf(format::V16U16, 0x8_0000, 0, 32, [0xffff, 0xffff_0000, 0, 0]),
    pf(
        format::A2W10V10U10,
        0x8_0001,
        0,
        32,
        [0x3ff0_0000, 0xffc00, 0x3ff, 0xc000_0000],
    ),
    cc(u32::from_le_bytes(*b"UYVY")),
    cc(u32::from_le_bytes(*b"RGBG")),
    cc(u32::from_le_bytes(*b"YUY2")),
    cc(u32::from_le_bytes(*b"GRGB")),
    cc(format::DXT1),
    cc(format::DXT2),
    cc(format::DXT3),
    cc(format::DXT4),
    cc(format::DXT5),
    pf(70, 0x400, 0, 16, [0, 0xffff, 0, 0]),
    cc(format::L16 + 1),
    cc(format::A16B16G16R16),
    cc(format::Q16W16V16U16),
    cc(format::R16F),
    cc(format::G16R16F),
    cc(format::A16B16G16R16F),
    cc(format::R32F),
    cc(format::G32R32F),
    cc(format::A32B32G32R32F),
    cc(117),
];

/// `D3DRESOURCETYPE`: a texture, a volume texture, a cube texture.
pub const RTYPE_TEXTURE: u32 = 3;
pub const RTYPE_VOLUME: u32 = 4;
pub const RTYPE_CUBE: u32 = 5;

/// One level of one face in the file: its size and where its bytes are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    /// Offset of the level's bytes in the file.
    pub offset: usize,
    /// Bytes per row (of blocks, for the block formats).
    pub pitch: usize,
    /// Bytes per slice.
    pub slice: usize,
}

/// A DDS file as D3DX reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    /// The file's level count (its mip count, 0 counting as 1).
    pub mip_levels: u32,
    /// The pixel format's `D3DFORMAT` (from [`PIXEL_FORMATS`]).
    pub format: u32,
    /// [`RTYPE_TEXTURE`], [`RTYPE_VOLUME`] or [`RTYPE_CUBE`].
    pub resource_type: u32,
    /// 1, or 6 for a cube map.
    pub faces: u32,
    /// The levels kept per face: the file's, at most down to 1x1 (a mip count past the whole
    /// chain only adds 1x1 levels D3DX reads but never loads).
    pub stored: u32,
    /// Face by face, level by level (`stored` each).
    pub levels: Vec<Level>,
    /// The loader's 32-bit arithmetic wrapped for a level: it took a size the file holds for a
    /// larger level, and D3DX's create calls then read past the file (measured: the 32-bit DLL
    /// crashes). The image info is the header's, as D3DX gives it.
    pub past_file: bool,
}

fn word(d: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(d[at..at + 4].try_into().unwrap())
}

/// A level's bytes as D3DX's DDS loader counts them (0x621052): the row, the slice and the whole
/// level (`d` slices), each in its 32-bit arithmetic, which wraps (`(w + 3) >> 2` too); and
/// whether that differs from the true size (the arithmetic wrapped: the loader then reads a size
/// the file may hold while the level is larger).
fn level_size(format: u32, bits: u32, w: u32, h: u32, d: u32) -> (u32, u32, u32, bool) {
    let (w64, h64) = (u128::from(w), u128::from(h));
    let ((row, slice), (true_row, true_slice)) = match format {
        format::DXT1 | format::DXT2 | format::DXT3 | format::DXT4 | format::DXT5 => {
            let shift = if format == format::DXT1 { 3 } else { 4 };
            let row = (w.wrapping_add(3) >> 2) << shift;
            let true_row = ((w64 + 3) >> 2) << shift;
            (
                (row, (h.wrapping_add(3) >> 2).wrapping_mul(row)),
                (true_row, ((h64 + 3) >> 2) * true_row),
            )
        }
        f if f == u32::from_le_bytes(*b"UYVY")
            || f == u32::from_le_bytes(*b"YUY2")
            || f == u32::from_le_bytes(*b"RGBG")
            || f == u32::from_le_bytes(*b"GRGB") =>
        {
            let row = (w.wrapping_add(1) >> 1) << 2;
            let true_row = ((w64 + 1) >> 1) << 2;
            ((row, row.wrapping_mul(h)), (true_row, true_row * h64))
        }
        _ => {
            let row = (bits >> 3).wrapping_mul(w);
            let true_row = u128::from(bits >> 3) * w64;
            ((row, row.wrapping_mul(h)), (true_row, true_row * h64))
        }
    };
    let size = slice.wrapping_mul(d);
    let true_size = true_slice * u128::from(d);
    let wrapped = u128::from(row) != true_row
        || u128::from(slice) != true_slice
        || u128::from(size) != true_size;
    (row, slice, size, wrapped)
}

/// The bits per pixel of the formats in D3DX's DDS table; 0 for the block formats.
fn format_bits(format: u32) -> u32 {
    match format {
        format::R8G8B8 => 24,
        format::A8R8G8B8
        | format::X8R8G8B8
        | format::A2B10G10R10
        | format::A8B8G8R8
        | format::X8B8G8R8
        | format::G16R16
        | format::A2R10G10B10
        | format::X8L8V8U8
        | format::Q8W8V8U8
        | format::V16U16
        | format::A2W10V10U10
        | format::G16R16F
        | format::R32F
        | 82 => 32,
        format::R5G6B5
        | format::X1R5G5B5
        | format::A1R5G5B5
        | format::A4R4G4B4
        | format::A8R3G3B2
        | format::X4R4G4B4
        | format::A8P8
        | format::A8L8
        | format::V8U8
        | format::L6V5U5
        | format::L16
        | format::R16F
        | 70
        | 117 => 16,
        format::R3G3B2 | format::A8 | format::P8 | format::L8 | format::A4L4 => 8,
        format::A16B16G16R16 | format::Q16W16V16U16 | format::A16B16G16R16F | format::G32R32F => 64,
        format::A32B32G32R32F => 128,
        _ => 0,
    }
}

/// The most levels a face may have: the 32-bit DLL makes a node for each level it reads and
/// overflows its stack on many (measured, `data/headers_32bit.tsv`: 2^16 levels of no bytes work
/// on a 1 MB stack, 2^18 crash, in the image info call already). Past this a file is refused
/// where the original would crash.
const MAX_LEVELS: u64 = 1 << 16;

/// Reads `data` as D3DX's DDS loader does; `None` when it refuses the file (D3DX then answers
/// `D3DXERR_INVALIDDATA`, its other image readers refusing it too).
///
/// Every size is the loader's own, in its 32-bit arithmetic, checked against the data left
/// (measured against the 32-bit DLL, `data/headers_32bit.tsv`): a level of 2^31 to 2^32 bytes is
/// not in the file; one past 2^32 wraps to a size that may be ([`Image::past_file`]); a side of 0
/// gives levels of no bytes, which every file holds. A mip count past the whole chain must still
/// have its 1x1 levels in the data, as D3DX reads them; only the chain's levels are kept. Past
/// [`MAX_LEVELS`] levels a face is refused (the DLL overflows its stack there).
pub fn parse(data: &[u8]) -> Option<Image> {
    if data.len() < 4 || word(data, 0) != u32::from_le_bytes(*b"DDS ") || data.len() - 4 < 0x7c {
        return None;
    }
    let width = word(data, 16);
    let height = word(data, 12);
    let flags = word(data, 8);
    let depth = if flags & 0x80_0000 == 0 {
        1
    } else {
        word(data, 24).max(1)
    };
    let caps2 = word(data, 112);
    let faces: u32 = match caps2 & 0xfc00 {
        0xfc00 => 6,
        0 => 1,
        _ => return None,
    };
    let mips = word(data, 28).max(1);
    let mut pf_flags = word(data, 80);
    if pf_flags & 4 != 0 {
        pf_flags = 4;
    }
    let (pf_size, fourcc, bits) = (word(data, 76), word(data, 84), word(data, 88));
    let masks = [92, 96, 100, 104].map(|a| word(data, a));
    let entry = PIXEL_FORMATS.iter().find(|e| {
        !(pf_size != 0x20
            || e.flags != pf_flags
            || (pf_flags & 4 != 0 && fourcc != e.fourcc)
            || (pf_flags & 0xc4462 != 0 && bits != e.bits)
            || (pf_flags & 0xe4040 != 0 && masks[0] != e.masks[0])
            || (pf_flags & 0xc4440 != 0 && masks[1] != e.masks[1])
            || (pf_flags & 0xc4040 != 0 && masks[2] != e.masks[2])
            || (pf_flags & 0x80003 != 0 && masks[3] != e.masks[3]))
    })?;
    let format = entry.format;
    let resource_type = if flags & 0x80_0000 != 0 {
        RTYPE_VOLUME
    } else if faces == 6 {
        RTYPE_CUBE
    } else {
        RTYPE_TEXTURE
    };
    // A palette of 256 colours follows the header for the palette formats.
    let mut at = 128u64;
    let mut left = (data.len() - 128) as u64;
    if format == format::P8 || format == format::A8P8 {
        if left < 0x400 {
            return None;
        }
        at += 0x400;
        left -= 0x400;
    }
    let bits = format_bits(format);
    // The whole chain: down to 1x1x1.
    let chain = 32 - width.max(height).max(depth).max(1).leading_zeros();
    let stored = mips.min(chain);
    let mut levels = Vec::new();
    let mut past_file = false;
    for _ in 0..faces {
        let (mut w, mut h, mut d) = (width, height, depth);
        let mut l = 0u32;
        while l < mips {
            let (pitch, slice, size, wrapped) = level_size(format, bits, w, h, d);
            let size64 = u64::from(size);
            let level = Level {
                width: w,
                height: h,
                depth: d,
                offset: at as usize,
                pitch: pitch as usize,
                slice: slice as usize,
            };
            if w <= 1 && h <= 1 && d <= 1 {
                // Every level from here on is this one (a side of 0 stays 0, one of 1 stays 1):
                // the loader makes a node for each until the data runs out, the refused one too.
                let rest = u64::from(mips - l);
                let fit = if size == 0 { rest } else { left / size64 };
                let made = u64::from(l) + if fit >= rest { rest } else { fit + 1 };
                if made > MAX_LEVELS || fit < rest {
                    return None;
                }
                past_file |= wrapped;
                if l < stored {
                    levels.push(level);
                }
                at += size64 * rest;
                left -= size64 * rest;
                break;
            }
            if size64 > left {
                return None;
            }
            past_file |= wrapped;
            if l < stored {
                levels.push(level);
            }
            at += size64;
            left -= size64;
            // The loader's halving: a side of 1 stays 1, and one of 0 stays 0.
            w = if w == 1 { 1 } else { w >> 1 };
            h = if h == 1 { 1 } else { h >> 1 };
            d = if d == 1 { 1 } else { d >> 1 };
            l += 1;
        }
    }
    Some(Image {
        width,
        height,
        depth,
        mip_levels: mips,
        format,
        resource_type,
        faces,
        stored,
        levels,
        past_file,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(w: u32, h: u32, mips: u32, fourcc: &[u8; 4]) -> Vec<u8> {
        let mut d = vec![0u8; 128];
        let mut put = |at: usize, v: u32| d[at..at + 4].copy_from_slice(&v.to_le_bytes());
        put(0, u32::from_le_bytes(*b"DDS "));
        put(8, 0x1007);
        put(12, h);
        put(16, w);
        put(28, mips);
        put(76, 32);
        put(80, 4);
        put(84, u32::from_le_bytes(*fourcc));
        d
    }

    /// A mip count past the chain needs its 1x1 levels in the data and keeps only the chain.
    #[test]
    fn mip_counts_past_the_chain_are_checked_not_kept() {
        let mut d = header(8, 8, 6, b"DXT1");
        d.extend(vec![0u8; 32 + 8 + 8 + 8 * 3]);
        let img = parse(&d).expect("8x8: 4 levels, then 2 more 1x1 ones");
        assert_eq!((img.mip_levels, img.stored, img.levels.len()), (6, 4, 4));
        d.pop();
        assert!(parse(&d).is_none(), "one byte short of the last 1x1 level");
        let mut d = header(4, 4, u32::MAX, b"DXT1");
        d.extend([0u8; 64]);
        assert!(parse(&d).is_none(), "4 billion levels are not in 64 bytes");
    }

    /// The loader's sizes wrap as its 32-bit arithmetic does, and a side of 0 gives levels of no
    /// bytes (measured answers: `data/headers_32bit.tsv`, `tests/dll_answers.rs`).
    #[test]
    fn sizes_wrap_as_the_loaders_and_sides_of_0_hold_nothing() {
        // 2 GiB of DXT1 blocks is a size the loader counts too, and not in the data.
        let mut d = header(0x1_0000, 0x1_0000, 1, b"DXT1");
        d.extend([0u8; 256]);
        assert!(parse(&d).is_none());
        // 8 GiB wraps to 0: the loader takes it, and the level is past the file.
        let mut d = header(0x2_0000, 0x2_0000, 1, b"DXT1");
        d.extend([0u8; 256]);
        assert!(parse(&d).expect("a wrapped size of 0").past_file);
        // `(w + 3) >> 2` wraps too.
        assert_eq!(level_size(format::DXT5, 0, u32::MAX, 4, 1), (0, 0, 0, true));
        assert_eq!(
            level_size(format::A8R8G8B8, 32, 65536, 65536, 1),
            (1 << 18, 0, 0, true)
        );
        // A side of 0: four levels of no bytes, in a file with none.
        let d = header(8, 0, 4, b"DXT1");
        let img = parse(&d).expect("levels of no bytes");
        assert_eq!((img.mip_levels, img.past_file), (4, false));
    }

    /// Past `MAX_LEVELS` levels a face is refused, where the DLL overflows its stack.
    #[test]
    fn too_many_levels_are_refused() {
        assert!(parse(&header(8, 0, 1 << 16, b"DXT1")).is_some());
        assert!(parse(&header(8, 0, (1 << 16) + 1, b"DXT1")).is_none());
    }
}
