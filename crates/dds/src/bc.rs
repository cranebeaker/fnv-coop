//! Block-compression (BC1-BC5) decoders. Each block covers 4x4 pixels and
//! decodes to 16 RGBA pixels in row-major order.

pub type Block = [[u8; 4]; 16];

fn expand565(c: u16) -> [u8; 3] {
    let r = ((c >> 11) & 31) as u8;
    let g = ((c >> 5) & 63) as u8;
    let b = (c & 31) as u8;
    [
        (r << 3) | (r >> 2),
        (g << 2) | (g >> 4),
        (b << 3) | (b >> 2),
    ]
}

/// BC1 color block (8 bytes). `force_four_color` is set for the color half
/// of BC2/BC3 blocks, which never use the 3-color + transparent mode.
fn color_block(b: &[u8], force_four_color: bool) -> Block {
    let c0 = u16::from_le_bytes([b[0], b[1]]);
    let c1 = u16::from_le_bytes([b[2], b[3]]);
    let (p0, p1) = (expand565(c0), expand565(c1));
    let mix = |w0: u16, w1: u16, div: u16| -> [u8; 4] {
        let ch = |i: usize| ((u16::from(p0[i]) * w0 + u16::from(p1[i]) * w1) / div) as u8;
        [ch(0), ch(1), ch(2), 255]
    };
    let palette: [[u8; 4]; 4] = if c0 > c1 || force_four_color {
        [
            [p0[0], p0[1], p0[2], 255],
            [p1[0], p1[1], p1[2], 255],
            mix(2, 1, 3),
            mix(1, 2, 3),
        ]
    } else {
        [
            [p0[0], p0[1], p0[2], 255],
            [p1[0], p1[1], p1[2], 255],
            mix(1, 1, 2),
            [0, 0, 0, 0],
        ]
    };
    let indices = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    let mut out = [[0u8; 4]; 16];
    for (i, px) in out.iter_mut().enumerate() {
        *px = palette[((indices >> (2 * i)) & 3) as usize];
    }
    out
}

/// BC4-style interpolated single channel (8 bytes), used for BC3 alpha,
/// BC4 and each half of BC5.
fn channel_block(b: &[u8]) -> [u8; 16] {
    let (a0, a1) = (u16::from(b[0]), u16::from(b[1]));
    let mut palette = [0u16; 8];
    palette[0] = a0;
    palette[1] = a1;
    if a0 > a1 {
        for i in 1..7u16 {
            palette[usize::from(i) + 1] = ((7 - i) * a0 + i * a1) / 7;
        }
    } else {
        for i in 1..5u16 {
            palette[usize::from(i) + 1] = ((5 - i) * a0 + i * a1) / 5;
        }
        palette[6] = 0;
        palette[7] = 255;
    }
    let mut bits = 0u64;
    for (i, &byte) in b[2..8].iter().enumerate() {
        bits |= u64::from(byte) << (8 * i);
    }
    let mut out = [0u8; 16];
    for (i, v) in out.iter_mut().enumerate() {
        *v = palette[((bits >> (3 * i)) & 7) as usize] as u8;
    }
    out
}

pub fn bc1(b: &[u8]) -> Block {
    color_block(b, false)
}

/// BC2: 4-bit explicit alpha, then a color block.
pub fn bc2(b: &[u8]) -> Block {
    let mut out = color_block(&b[8..16], true);
    let alpha = u64::from_le_bytes(b[0..8].try_into().expect("8 bytes"));
    for (i, px) in out.iter_mut().enumerate() {
        let a = ((alpha >> (4 * i)) & 15) as u8;
        px[3] = a * 17;
    }
    out
}

/// BC3: interpolated alpha, then a color block.
pub fn bc3(b: &[u8]) -> Block {
    let mut out = color_block(&b[8..16], true);
    for (px, a) in out.iter_mut().zip(channel_block(&b[0..8])) {
        px[3] = a;
    }
    out
}

/// BC4: one channel, shown as grey.
pub fn bc4(b: &[u8]) -> Block {
    let mut out = [[0u8; 4]; 16];
    for (px, v) in out.iter_mut().zip(channel_block(b)) {
        *px = [v, v, v, 255];
    }
    out
}

/// BC5: two channels (a normal map's X and Y). Z is rebuilt so the result
/// looks like an ordinary normal map.
pub fn bc5(b: &[u8]) -> Block {
    let (red, green) = (channel_block(&b[0..8]), channel_block(&b[8..16]));
    let mut out = [[0u8; 4]; 16];
    for i in 0..16 {
        let x = f32::from(red[i]) / 127.5 - 1.0;
        let y = f32::from(green[i]) / 127.5 - 1.0;
        let z = (1.0 - x * x - y * y).max(0.0).sqrt();
        out[i] = [red[i], green[i], ((z + 1.0) * 127.5).round() as u8, 255];
    }
    out
}
