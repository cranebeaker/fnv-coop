//! The game's conversion of a decoded frame to 32-bit colour.
//!
//! The game copies each movie frame with `BinkCopyToBufferRect` and surface
//! type 3 (32 bits per pixel) into X8R8G8B8 textures, so what it shows is
//! what `binkw32.dll`'s 32-bit blitter makes. On any processor with MMX that
//! is the routine at 0x18025c50 (read as disassembly), using tables that
//! `_YUV_init@4` (0x180174e0) builds:
//!
//! - luma: `pmulhw((max(Y - 16, 0) << 2), 0x4a85)`, the high 16 bits of a
//!   signed 16-bit product (constants at 0x1804f038 and 0x1804f040);
//! - chroma: four tables of `(c - 128) * k / 32768` truncated toward zero,
//!   with k = 66120 for U into blue, -12827 for U into green, -26656 for V into
//!   green and 52291 for V into red;
//! - blue = luma + U term, green = luma + (U term + V term), red = luma + V
//!   term, in 16-bit words, then clamped to 0..255 by adding 0x7f00 with
//!   signed saturation and subtracting it with unsigned saturation (constant
//!   at 0x1804f048);
//! - one chroma sample per 2x2 block of pixels (nearest, no filtering).
//!
//! The crate documentation records the frame-by-frame comparison with the
//! DLL's output.

/// Truncating division toward zero by 32768, as the table builder computes
/// it (`(x + ((x >> 31) & 0x7fff)) >> 15`).
const fn tz15(x: i32) -> i32 {
    (x + ((x >> 31) & 0x7fff)) >> 15
}

const fn table(k: i32) -> [i16; 256] {
    let mut t = [0i16; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = tz15((i as i32 - 128) * k) as i16;
        i += 1;
    }
    t
}

/// U into blue (binkw32.dll 0x1805bba8, step 0x10248).
const U_BLUE: [i16; 256] = table(66120);
/// U into green (0x1805c3a8, step -0x321b).
const U_GREEN: [i16; 256] = table(-12827);
/// V into green (0x1805bfa8, step -0x6820).
const V_GREEN: [i16; 256] = table(-26656);
/// V into red (0x1805c7a8, step 0xcc43).
const V_RED: [i16; 256] = table(52291);

fn luma(y: u8) -> i16 {
    let a = (y.saturating_sub(16) as i16) << 2;
    ((a as i32 * 0x4a85) >> 16) as i16
}

/// `paddsw 0x7f00` then `psubusw 0x7f00`.
fn clamp(x: i16) -> u8 {
    let t = x.saturating_add(0x7f00) as u16;
    t.saturating_sub(0x7f00) as u8
}

/// One pixel as (blue, green, red).
pub(crate) fn bgr(y: u8, u: u8, v: u8) -> [u8; 3] {
    let l = luma(y);
    let g = U_GREEN[u as usize].wrapping_add(V_GREEN[v as usize]);
    [
        clamp(l.wrapping_add(U_BLUE[u as usize])),
        clamp(l.wrapping_add(g)),
        clamp(l.wrapping_add(V_RED[v as usize])),
    ]
}
