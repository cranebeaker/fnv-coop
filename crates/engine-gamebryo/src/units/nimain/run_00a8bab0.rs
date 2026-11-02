//! `NiMain/run_00a8bab0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The code address `00a8bd20` stores at +4 (`00a8b8c0`).
const FUNCTION_00A8B8C0: u32 = 0x00a8_b8c0;
/// The `double`s the nibble table is scaled with (`01015a38`, divided by; `0101e568`,
/// multiplied by) and the `float` 2^32 the unsigned conversion adds (`01099378`).
const DIVISOR_01015A38: u32 = 0x0101_5a38;
const FACTOR_0101E568: u32 = 0x0101_e568;
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// One entry of the nibble table: `FILD` of the nibble (plus 2^32 when negative, which a
/// nibble never is), `FDIV ST0,ST2` by the divisor, `FMUL ST0,ST1` by the factor (bytes
/// `D8 F2`, `D8 C9`), `FISTP` with the rounding control set to chop; the integer's low byte.
fn scaled_nibble(e: &Engine, nibble: u32, divisor: f64, factor: f64) -> u8 {
    let mut value = f64::from(nibble as i32);
    if (nibble as i32) < 0 {
        // Unreachable: a nibble is never negative; the exe still tests it.
        value = x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)));
    }
    let scaled = x87_mul(x87_div(value, divisor), factor);
    crate::x87::fistp_chop(scaled) as u8
}

// Translated from 00a8bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a8bab0`), `thiscall`, no stack words: clears +0, +8 and
/// +0xc, stores the code address `00a8b8c0` at +4, and fills the 256 byte pairs from +0x10:
/// for each index, its low nibble then its high nibble, each divided by the `double` at
/// `01015a38`, multiplied by the one at `0101e568` and truncated (both `double`s loaded once
/// on the x87 stack). Answers `this`.
pub fn fn_00a8bd20(e: &mut Engine, this: Ptr) -> Ptr {
    let divisor = x87_load64(e.mem.u64(DIVISOR_01015A38));
    let factor = x87_load64(e.mem.u64(FACTOR_0101E568));
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base.wrapping_add(4), FUNCTION_00A8B8C0);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    for index in 0..0x100u32 {
        let pair = base.wrapping_add(0x10 + 2 * index);
        let low = scaled_nibble(e, index & 0xf, divisor, factor);
        e.mem.set_u8(pair, low);
        let high = scaled_nibble(e, (index >> 4) & 0xf, divisor, factor);
        e.mem.set_u8(pair.wrapping_add(1), high);
    }
    this
}

/// The three bytes the decoder makes of a 16-bit colour: `(c >> 8) & 0xf8`, `(c >> 3) &
/// 0xfe` and the low byte shifted left by 3 (8 bits), as the exe computes them (the middle
/// mask keeps one bit more than the 6-bit field).
fn colour_bytes(colour: u16) -> [u8; 3] {
    [
        ((colour >> 8) as u8) & 0xf8,
        ((colour >> 3) as u8) & 0xfe,
        (colour as u8) << 3,
    ]
}

// Translated from 00a8bab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a8bab0 - the /GS cookie check (00ec408c) is not modelled
/// Unnamed in the engine map (`run_00a8bab0`, `thiscall`, the output; `RET 4`): decodes one
/// colour block from the read pointer at +8 into 16 four-byte pixels at `output`. Reads two
/// little-endian 16-bit colours (the pointer advanced by 2 after each), makes each into
/// three bytes ([`colour_bytes`]) and alpha 0xff, and builds a four-entry palette: when the
/// first colour is greater (unsigned) the two others are `(second + 2·first) / 3` and
/// `(first + 2·second) / 3` per byte with alpha 0xff, otherwise both are the halves
/// `(first + second) >> 1`, the third alpha 0xff and the fourth 0. Then four rows: each
/// byte at the pointer (advanced by 1 after each row) gives four 2-bit palette indices, low
/// bits first, one per pixel.
pub fn fn_00a8bab0(e: &mut Engine, this: Ptr, output: Ptr) {
    let reader = this.addr() + 8;
    let at = e.mem.u32(reader);
    let first = u16::from(e.mem.u8(at.wrapping_add(1)))
        .wrapping_mul(0x100)
        .wrapping_add(u16::from(e.mem.u8(at)));
    let at = at.wrapping_add(2);
    e.mem.set_u32(reader, at);
    let second = u16::from(e.mem.u8(at.wrapping_add(1)))
        .wrapping_mul(0x100)
        .wrapping_add(u16::from(e.mem.u8(at)));
    e.mem.set_u32(reader, at.wrapping_add(2));
    let a = colour_bytes(first);
    let b = colour_bytes(second);
    let mut palette = [0u8; 16];
    palette[..3].copy_from_slice(&a);
    palette[3] = 0xff;
    palette[4..7].copy_from_slice(&b);
    palette[7] = 0xff;
    palette[0xb] = 0xff;
    if first > second {
        for i in 0..3 {
            let (x, y) = (u32::from(a[i]), u32::from(b[i]));
            palette[8 + i] = ((y + 2 * x) / 3) as u8;
            palette[0xc + i] = ((x + 2 * y) / 3) as u8;
        }
        palette[0xf] = 0xff;
    } else {
        for i in 0..3 {
            let half = ((u32::from(b[i]) + u32::from(a[i])) >> 1) as u8;
            palette[8 + i] = half;
            palette[0xc + i] = half;
        }
        palette[0xf] = 0;
    }
    let mut out = output.addr();
    for _ in 0..4 {
        let indices = e.mem.u8(e.mem.u32(reader));
        for pixel in 0..4u32 {
            let entry = 4 * usize::from((indices >> (2 * pixel)) & 3);
            for byte in 0..4u32 {
                let value = palette[entry + byte as usize];
                e.mem.set_u8(out.wrapping_add(4 * pixel + byte), value);
            }
        }
        let next = e.mem.u32(reader).wrapping_add(1);
        e.mem.set_u32(reader, next);
        out = out.wrapping_add(0x10);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_bd20, fn_00a8bd20(Ptr) -> Ptr),
        entry!(0x00a8_bab0, fn_00a8bab0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    #[test]
    fn fn_00a8bd20_fills_the_scaled_nibble_pairs() {
        let mut e = Engine::new();
        e.map(DIVISOR_01015A38, 8);
        e.map(FACTOR_0101E568, 8);
        e.map(TWO_TO_32_01099378, 4);
        e.set_global(DIVISOR_01015A38, 15.0f64);
        e.set_global(FACTOR_0101E568, 255.0f64);
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this, 0xffff_ffff);
        assert_eq!(e.call(0x00a8_bd20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), FUNCTION_00A8B8C0);
        let expected = |n: u32| (f64::from(n) / 15.0 * 255.0).trunc() as u8;
        for index in [0u32, 0x21, 0xd7, 0xff] {
            let pair = this + 0x10 + 2 * index;
            assert_eq!(e.mem.u8(pair), expected(index & 0xf));
            assert_eq!(e.mem.u8(pair + 1), expected(index >> 4));
        }
        assert_eq!(e.mem.u16(this + 0x10 + 2 * 0xff), 0xffff);
        assert_eq!(e.mem.u16(this + 0x10), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8bd20_fills_the_nibble_table() {
        let mut e = Engine::new();
        e.map(DIVISOR_01015A38, 8);
        e.mem.set_u64(DIVISOR_01015A38, 15f64.to_bits());
        e.map(FACTOR_0101E568, 8);
        e.mem.set_u64(FACTOR_0101E568, 255f64.to_bits());
        e.map(TWO_TO_32_01099378, 4);
        e.mem.set_u32(TWO_TO_32_01099378, 0x4f80_0000);
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this, 9);
        assert_eq!(e.call(0x00a8_bd20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), FUNCTION_00A8B8C0);
        for index in 0u32..0x100 {
            assert_eq!(
                e.mem.u8(this + 0x10 + 2 * index),
                ((index & 0xf) * 17) as u8
            );
            assert_eq!(e.mem.u8(this + 0x11 + 2 * index), ((index >> 4) * 17) as u8);
        }
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;

    fn decode(first: u16, second: u16, rows: [u8; 4]) -> (Vec<u8>, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(8);
        e.mem.write(data, &first.to_le_bytes());
        e.mem.write(data + 2, &second.to_le_bytes());
        e.mem.write(data + 4, &rows);
        e.mem.set_u32(this + 8, data);
        let output = e.mem.alloc(0x40);
        e.call(0x00a8_bab0, &args![this, output]);
        (e.mem.bytes(output, 0x40), e.mem.u32(this + 8), data)
    }

    #[test]
    fn four_colour_blocks_interpolate_by_thirds() {
        // 0xf81f: bytes f8, (0xf81f >> 3) & 0xfe = 0x02, 0x1f << 3 = 0xf8; 0x07e0: 0, 0xfc, 0.
        let (pixels, reader, data) = decode(0xf81f, 0x07e0, [0b1110_0100, 0, 0xff, 0x55]);
        assert_eq!(reader, data + 8);
        assert_eq!(&pixels[..4], &[0xf8, 0x02, 0xf8, 0xff]);
        assert_eq!(&pixels[4..8], &[0x00, 0xfc, 0x00, 0xff]);
        // (second + 2 first) / 3 and (first + 2 second) / 3.
        assert_eq!(&pixels[8..0xc], &[0xa5, 0x55, 0xa5, 0xff]);
        assert_eq!(&pixels[0xc..0x10], &[0x52, 0xa8, 0x52, 0xff]);
        assert_eq!(&pixels[0x10..0x14], &[0xf8, 0x02, 0xf8, 0xff]);
        assert_eq!(&pixels[0x20..0x24], &[0x52, 0xa8, 0x52, 0xff]);
        assert_eq!(&pixels[0x30..0x34], &[0x00, 0xfc, 0x00, 0xff]);
    }

    #[test]
    fn three_colour_blocks_average_and_clear_the_last_alpha() {
        let (pixels, _, _) = decode(0x07e0, 0xf81f, [0b1110_0100, 0, 0, 0]);
        assert_eq!(&pixels[..4], &[0x00, 0xfc, 0x00, 0xff]);
        assert_eq!(&pixels[8..0xc], &[0x7c, 0x7f, 0x7c, 0xff]);
        assert_eq!(&pixels[0xc..0x10], &[0x7c, 0x7f, 0x7c, 0x00]);
        // Equal colours take the same path.
        let (pixels, _, _) = decode(0x1234, 0x1234, [0xff, 0, 0, 0]);
        assert_eq!(pixels[3], 0);
    }
}
