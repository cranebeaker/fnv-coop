//! DDS tests against textures assembled byte by byte. (The block decoders
//! were also checked to match Pillow's output exactly on DXT1/3/5, BC5 and
//! uncompressed files.)

use dds::{Dds, Error, Format};

const FOURCC: u32 = 0x4;
const RGB: u32 = 0x40;
const ALPHA_PIXELS: u32 = 0x1;
const LUMINANCE: u32 = 0x2_0000;
const CUBEMAP_ALL: u32 = 0x200 | 0xFC00;

#[derive(Default, Clone, Copy)]
struct Pf {
    flags: u32,
    four_cc: [u8; 4],
    bits: u32,
    masks: [u32; 4],
}

fn fourcc(code: &[u8; 4]) -> Pf {
    Pf {
        flags: FOURCC,
        four_cc: *code,
        ..Pf::default()
    }
}

fn dds(width: u32, height: u32, mips: u32, pf: Pf, caps2: u32, data: &[u8]) -> Vec<u8> {
    let mut h = vec![0u8; 124];
    let mut put = |at: usize, v: u32| h[at..at + 4].copy_from_slice(&v.to_le_bytes());
    put(0, 124);
    put(4, 0x1007 | if mips > 1 { 0x2_0000 } else { 0 });
    put(8, height);
    put(12, width);
    put(24, mips);
    put(72, 32);
    put(76, pf.flags);
    put(84, pf.bits);
    for (i, m) in pf.masks.iter().enumerate() {
        put(88 + 4 * i, *m);
    }
    put(104, 0x1000);
    put(108, caps2);
    h[80..84].copy_from_slice(&pf.four_cc);
    let mut out = b"DDS ".to_vec();
    out.extend(h);
    out.extend(data);
    out
}

/// A BC1 block from two RGB565 colors and 16 two-bit indices.
fn bc1_block(c0: u16, c1: u16, indices: [u8; 16]) -> Vec<u8> {
    let mut bits = 0u32;
    for (i, &ix) in indices.iter().enumerate() {
        bits |= u32::from(ix) << (2 * i);
    }
    let mut v = c0.to_le_bytes().to_vec();
    v.extend(c1.to_le_bytes());
    v.extend(bits.to_le_bytes());
    v
}

/// An interpolated-alpha block from two endpoints and 16 three-bit indices.
fn alpha_block(a0: u8, a1: u8, indices: [u8; 16]) -> Vec<u8> {
    let mut bits = 0u64;
    for (i, &ix) in indices.iter().enumerate() {
        bits |= u64::from(ix) << (3 * i);
    }
    let mut v = vec![a0, a1];
    v.extend(&bits.to_le_bytes()[..6]);
    v
}

const RED: u16 = 0xF800;
const BLUE: u16 = 0x001F;

fn pixel(rgba: &[u8], width: usize, x: usize, y: usize) -> [u8; 4] {
    let o = (y * width + x) * 4;
    rgba[o..o + 4].try_into().unwrap()
}

#[test]
fn decodes_bc1_four_color_blocks() {
    let mut idx = [0u8; 16];
    idx[1] = 1;
    idx[2] = 2;
    idx[3] = 3;
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"DXT1"), 0, &bc1_block(RED, BLUE, idx))).unwrap();
    assert_eq!(tex.format(), Format::Bc1);
    let rgba = tex.decode_rgba(0, 0).unwrap();
    assert_eq!(pixel(&rgba, 4, 0, 0), [255, 0, 0, 255]);
    assert_eq!(pixel(&rgba, 4, 1, 0), [0, 0, 255, 255]);
    assert_eq!(pixel(&rgba, 4, 2, 0), [170, 0, 85, 255]);
    assert_eq!(pixel(&rgba, 4, 3, 0), [85, 0, 170, 255]);
}

#[test]
fn decodes_bc1_transparent_mode() {
    // color0 <= color1 selects three colors plus transparent black.
    let mut idx = [0u8; 16];
    idx[0] = 2;
    idx[1] = 3;
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"DXT1"), 0, &bc1_block(BLUE, RED, idx))).unwrap();
    let rgba = tex.decode_rgba(0, 0).unwrap();
    assert_eq!(pixel(&rgba, 4, 0, 0), [127, 0, 127, 255]);
    assert_eq!(pixel(&rgba, 4, 1, 0), [0, 0, 0, 0]);
}

#[test]
fn decodes_bc2_explicit_alpha() {
    let mut block = Vec::new();
    let alpha: u64 = 0xF | (0x8 << 4); // pixel 0 alpha 15, pixel 1 alpha 8
    block.extend(alpha.to_le_bytes());
    block.extend(bc1_block(BLUE, RED, [0; 16])); // four-color mode regardless of order
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"DXT3"), 0, &block)).unwrap();
    let rgba = tex.decode_rgba(0, 0).unwrap();
    assert_eq!(pixel(&rgba, 4, 0, 0), [0, 0, 255, 255]);
    assert_eq!(pixel(&rgba, 4, 1, 0)[3], 136);
    assert_eq!(pixel(&rgba, 4, 2, 0)[3], 0);
}

#[test]
fn decodes_bc3_alpha_in_both_modes() {
    let mut idx = [0u8; 16];
    idx[1] = 1;
    idx[2] = 2;
    idx[3] = 7;
    let mut eight = alpha_block(255, 0, idx);
    eight.extend(bc1_block(RED, BLUE, [0; 16]));
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"DXT5"), 0, &eight)).unwrap();
    let a: Vec<u8> = tex
        .decode_rgba(0, 0)
        .unwrap()
        .chunks(4)
        .map(|p| p[3])
        .collect();
    assert_eq!(&a[..4], &[255, 0, 218, 36]);

    let mut idx = [0u8; 16];
    idx[0] = 2;
    idx[1] = 6;
    idx[2] = 7;
    let mut six = alpha_block(0, 255, idx);
    six.extend(bc1_block(RED, BLUE, [0; 16]));
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"DXT5"), 0, &six)).unwrap();
    let a: Vec<u8> = tex
        .decode_rgba(0, 0)
        .unwrap()
        .chunks(4)
        .map(|p| p[3])
        .collect();
    assert_eq!(&a[..3], &[51, 0, 255]);
}

#[test]
fn decodes_bc4_and_bc5() {
    let mut idx = [0u8; 16];
    idx[1] = 1;
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"ATI1"), 0, &alpha_block(200, 40, idx))).unwrap();
    let rgba = tex.decode_rgba(0, 0).unwrap();
    assert_eq!(pixel(&rgba, 4, 0, 0), [200, 200, 200, 255]);
    assert_eq!(pixel(&rgba, 4, 1, 0), [40, 40, 40, 255]);

    // A flat normal: X and Y at the midpoint, so Z is rebuilt as 1.0.
    let mut block = alpha_block(128, 128, [0; 16]);
    block.extend(alpha_block(128, 128, [0; 16]));
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"ATI2"), 0, &block)).unwrap();
    assert_eq!(
        pixel(&tex.decode_rgba(0, 0).unwrap(), 4, 0, 0),
        [128, 128, 255, 255]
    );
}

#[test]
fn clips_blocks_at_odd_sizes() {
    // 5x3 needs 2x1 blocks; the second block's extra pixels are dropped.
    let mut data = bc1_block(RED, BLUE, [0; 16]);
    data.extend(bc1_block(RED, BLUE, [1; 16]));
    let tex = Dds::parse(dds(5, 3, 1, fourcc(b"DXT1"), 0, &data)).unwrap();
    let rgba = tex.decode_rgba(0, 0).unwrap();
    assert_eq!(rgba.len(), 5 * 3 * 4);
    assert_eq!(pixel(&rgba, 5, 3, 2), [255, 0, 0, 255]);
    assert_eq!(pixel(&rgba, 5, 4, 2), [0, 0, 255, 255]);
}

#[test]
fn decodes_uncompressed_layouts() {
    let argb = Pf {
        flags: RGB | ALPHA_PIXELS,
        bits: 32,
        masks: [0x00FF_0000, 0xFF00, 0xFF, 0xFF00_0000],
        ..Pf::default()
    };
    let tex = Dds::parse(dds(1, 1, 1, argb, 0, &[10, 20, 30, 40])).unwrap();
    assert_eq!(tex.format().name(), "A8R8G8B8");
    assert_eq!(tex.decode_rgba(0, 0).unwrap(), [30, 20, 10, 40]);

    let r565 = Pf {
        flags: RGB,
        bits: 16,
        masks: [0xF800, 0x07E0, 0x1F, 0],
        ..Pf::default()
    };
    let tex = Dds::parse(dds(1, 1, 1, r565, 0, &0xF81Fu16.to_le_bytes())).unwrap();
    assert_eq!(tex.format().name(), "R5G6B5");
    assert_eq!(tex.decode_rgba(0, 0).unwrap(), [255, 0, 255, 255]);

    let a8l8 = Pf {
        flags: LUMINANCE | ALPHA_PIXELS,
        bits: 16,
        masks: [0xFF, 0, 0, 0xFF00],
        ..Pf::default()
    };
    let tex = Dds::parse(dds(1, 1, 1, a8l8, 0, &[90, 200])).unwrap();
    assert_eq!(tex.format().name(), "A8L8");
    assert_eq!(tex.decode_rgba(0, 0).unwrap(), [90, 90, 90, 200]);
}

#[test]
fn lays_out_mip_chains() {
    // 16x8 DXT1: levels 16x8, 8x4, 4x2, 2x1, 1x1 -> 8, 2, 1, 1, 1 blocks.
    let sizes = [64usize, 16, 8, 8, 8];
    let mut data = Vec::new();
    for (level, size) in sizes.iter().enumerate() {
        data.extend(vec![level as u8; *size]);
    }
    let tex = Dds::parse(dds(16, 8, 5, fourcc(b"DXT1"), 0, &data)).unwrap();
    assert_eq!(tex.mip_count(), 5);
    for (level, size) in sizes.iter().enumerate() {
        assert_eq!(tex.level_size(level), *size);
        let bytes = tex.level_data(0, level).unwrap();
        assert!(bytes.iter().all(|&b| b == level as u8));
    }
    assert_eq!(tex.level_dimensions(3), (2, 1));
    assert!(tex.level_data(0, 5).is_err());
}

#[test]
fn keeps_the_levels_that_are_present() {
    // Header claims 5 levels, data only holds the first 3.
    let tex = Dds::parse(dds(16, 8, 5, fourcc(b"DXT1"), 0, &[0; 64 + 16 + 8])).unwrap();
    assert_eq!(tex.declared_mip_count(), 5);
    assert_eq!(tex.mip_count(), 3);
}

#[test]
fn lays_out_cube_map_faces() {
    // Six faces of 4x4 DXT1 with two levels each: face = 8 + 8 bytes.
    let mut data = Vec::new();
    for face in 0..6u8 {
        data.extend(vec![face * 10; 8]);
        data.extend(vec![face * 10 + 1; 8]);
    }
    let tex = Dds::parse(dds(4, 4, 2, fourcc(b"DXT1"), CUBEMAP_ALL, &data)).unwrap();
    assert!(tex.is_cube_map());
    assert_eq!(tex.face_count(), 6);
    assert!(tex.level_data(5, 1).unwrap().iter().all(|&b| b == 51));
    assert!(tex.level_data(2, 0).unwrap().iter().all(|&b| b == 20));
}

#[test]
fn reads_dx10_headers() {
    let mut ext = 71u32.to_le_bytes().to_vec(); // BC1_UNORM
    ext.extend([3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
    ext.extend(bc1_block(RED, BLUE, [0; 16]));
    let tex = Dds::parse(dds(4, 4, 1, fourcc(b"DX10"), 0, &ext)).unwrap();
    assert_eq!(tex.format(), Format::Bc1);
    assert_eq!(
        pixel(&tex.decode_rgba(0, 0).unwrap(), 4, 0, 0),
        [255, 0, 0, 255]
    );
}

#[test]
fn rejects_bad_input() {
    assert!(matches!(
        Dds::parse(b"BSA\0".to_vec()),
        Err(Error::NotADds { .. })
    ));
    assert!(matches!(
        Dds::parse(b"DDS short".to_vec()),
        Err(Error::Malformed(_))
    ));
    let too_little = dds(8, 8, 1, fourcc(b"DXT5"), 0, &[0; 20]);
    let err = Dds::parse(too_little).err().unwrap();
    assert!(err.to_string().contains("needs 64 bytes"), "{err}");
    assert!(matches!(
        Dds::parse(dds(4, 4, 1, fourcc(b"BC7X"), 0, &[0; 16])),
        Err(Error::Unsupported(_))
    ));
    assert!(matches!(
        Dds::parse(dds(4, 4, 1, fourcc(b"DXT1"), 0x20_0000, &[0; 8])),
        Err(Error::Unsupported(_))
    ));
    assert!(Dds::parse(dds(0, 4, 1, fourcc(b"DXT1"), 0, &[0; 8])).is_err());
    assert!(Dds::parse(dds(4, 4, 1, fourcc(b"DX10"), 0, &[])).is_err());
}

/// Decodes a PNG written by `write_rgba` back to RGBA, undoing the filters.
fn decode_png(png: &[u8], width: usize) -> Vec<u8> {
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    let (mut pos, mut idat) = (8, Vec::new());
    while pos < png.len() {
        let len = u32::from_be_bytes(png[pos..pos + 4].try_into().unwrap()) as usize;
        if &png[pos + 4..pos + 8] == b"IDAT" {
            idat.extend(&png[pos + 8..pos + 8 + len]);
        }
        pos += 12 + len;
    }
    let raw = esm::inflate::zlib_decompress(&idat, None).unwrap();
    let row = width * 4;
    let mut out: Vec<u8> = Vec::new();
    for (y, line) in raw.chunks(row + 1).enumerate() {
        let start = out.len();
        for i in 0..row {
            let a = if i >= 4 { out[start + i - 4] } else { 0 };
            let b = if y > 0 { out[start + i - row] } else { 0 };
            let c = if i >= 4 && y > 0 {
                out[start + i - row - 4]
            } else {
                0
            };
            let predicted = match line[0] {
                0 => 0,
                1 => a,
                2 => b,
                3 => ((u16::from(a) + u16::from(b)) / 2) as u8,
                4 => {
                    let p = i16::from(a) + i16::from(b) - i16::from(c);
                    let (pa, pb, pc) = (
                        (p - i16::from(a)).abs(),
                        (p - i16::from(b)).abs(),
                        (p - i16::from(c)).abs(),
                    );
                    if pa <= pb && pa <= pc {
                        a
                    } else if pb <= pc {
                        b
                    } else {
                        c
                    }
                }
                other => panic!("bad filter type {other}"),
            };
            out.push(line[1 + i].wrapping_add(predicted));
        }
    }
    out
}

#[test]
fn writes_png_that_round_trips() {
    let rgba: Vec<u8> = (0..5 * 3 * 4).map(|i| (i * 7) as u8).collect();
    let mut png = Vec::new();
    dds::png::write_rgba(&mut png, 5, 3, &rgba).unwrap();
    assert_eq!(decode_png(&png, 5), rgba);
    assert!(dds::png::write_rgba(&mut Vec::new(), 5, 3, &rgba[..10]).is_err());
}

#[test]
fn compresses_images_with_smooth_areas() {
    // A 400x300 picture: gradients, flat areas and noise.
    let (w, h) = (400usize, 300usize);
    let mut seed = 12345u32;
    let mut rgba = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            let noise = (seed >> 24) as u8;
            let px = if y < 100 {
                [x as u8, y as u8, 128, 255]
            } else if y < 200 {
                [30, 60, 90, 255]
            } else {
                [noise, noise / 2, 200, 255]
            };
            rgba.extend(px);
        }
    }
    let mut png = Vec::new();
    dds::png::write_rgba(&mut png, w as u32, h as u32, &rgba).unwrap();
    assert_eq!(decode_png(&png, w), rgba);
    assert!(
        png.len() < rgba.len() / 3,
        "{} bytes for {}",
        png.len(),
        rgba.len()
    );
}
