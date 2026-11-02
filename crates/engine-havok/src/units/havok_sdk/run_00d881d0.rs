//! `Havok SDK/run_00d881d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` constants the layout scales with: `-0.6f` (`010d3d58`), `1/64` (`010d3d50`) and
/// `-0.4f` (`010d3d48`), each exactly a `float` widened.
const MINUS_SIXTH_TENTHS: u32 = 0x010d_3d58;
const ONE_SIXTY_FOURTH: u32 = 0x010d_3d50;
const MINUS_FOUR_TENTHS: u32 = 0x010d_3d48;

/// `FISTP` with the rounding control the code sets (`0xc00`: toward zero) to a 64-bit integer,
/// of which the exe keeps the low 32 bits.
fn fistp_truncate_low(value: f64) -> u32 {
    (value as i64) as u32
}

// Translated from 00d881d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(first, second, third, start, size, out)`: lays out a buffer of `size` bytes (cut to a
/// whole number `n` of 16-byte units toward zero) behind `start` into the record `out` of 0x50
/// bytes: `first`, `second` and `third` at `+0`, `+4` and `+8`; the aligned start `a =
/// (start + 15) & !15` at `+0xc` and `+0x14`; `b = (a + n * 10 / 64) & !15` at `+0x10` and
/// `+0x18`; `c = (b + n * 4 / 64) & !15` at `+0x1c`, `+0x20`, `+0x28` and `+0x2c`; `d = (c -
/// trunc(-0.6f * float(n * 40) / 64)) & !15` at `+0x24`, `+0x30`, `+0x38` and `+0x3c`;
/// `f = (d - trunc(float(n * 40) * -0.4f / 64)) & !15` at `+0x34`, `+0x40`, `+0x48` and `+0x4c`;
/// and `start + n` at `+0x44`. The divisions by 64 are the compiler's round-toward-zero shifts.
/// The two truncations are `FISTP` with the rounding control set to truncate (the original
/// control word is put back afterwards, so nothing of it outlives the call); the products are
/// exact (each factor has at most 24 significant bits) so the host's `f64` agrees.
pub fn fn_00d881d0(
    e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    start: u32,
    size: u32,
    out: u32,
) {
    e.mem.set_u32(out + 4, second);
    e.mem.set_u32(out, first);
    // `size & 0x8000000f`, then the remainder the compiler makes of it (the sign of `size`).
    let remainder = (size as i32) % 16;
    e.mem.set_u32(out + 8, third);
    let n = (size as i32).wrapping_sub(remainder);
    let aligned_start = start.wrapping_add(0xf) & 0xffff_fff0;
    let times_ten = n.wrapping_mul(10);
    let scaled_ten = (times_ten.wrapping_add((times_ten >> 31) & 0x3f)) >> 6;
    let b = (scaled_ten as u32).wrapping_add(aligned_start) & 0xffff_fff0;
    let times_four = n.wrapping_mul(4);
    let scaled_four = (times_four.wrapping_add((times_four >> 31) & 0x3f)) >> 6;
    let c = (scaled_four as u32).wrapping_add(b) & 0xffff_fff0;
    e.mem.set_u32(out + 0x20, c);
    e.mem.set_u32(out + 0x1c, c);
    e.mem.set_u32(out + 0x10, b);
    e.mem.set_u32(out + 0x18, b);
    e.mem.set_u32(out + 0xc, aligned_start);
    // `FILD` of `n * 40` then `FSTP`/`FLD` as a float.
    let forty = f64::from(n.wrapping_mul(40) as f32);
    let scale: f64 = e.mem.f64(MINUS_SIXTH_TENTHS);
    let sixty_fourth: f64 = e.mem.f64(ONE_SIXTY_FOURTH);
    let first_shift = fistp_truncate_low(x87_mul(x87_mul(scale, forty), sixty_fourth));
    let d = c.wrapping_sub(first_shift) & 0xffff_fff0;
    e.mem.set_u32(out + 0x30, d);
    e.mem.set_u32(out + 0x24, d);
    let second_scale: f64 = e.mem.f64(MINUS_FOUR_TENTHS);
    let second_shift = fistp_truncate_low(x87_mul(sixty_fourth, x87_mul(forty, second_scale)));
    let f = d.wrapping_sub(second_shift) & 0xffff_fff0;
    e.mem.set_u32(out + 0x14, aligned_start);
    let stored_c = e.mem.u32(out + 0x20);
    e.mem.set_u32(out + 0x44, start.wrapping_add(n as u32));
    e.mem.set_u32(out + 0x28, stored_c);
    e.mem.set_u32(out + 0x2c, stored_c);
    let stored_d = e.mem.u32(out + 0x30);
    e.mem.set_u32(out + 0x34, f);
    e.mem.set_u32(out + 0x40, f);
    e.mem.set_u32(out + 0x48, f);
    e.mem.set_u32(out + 0x4c, f);
    e.mem.set_u32(out + 0x38, stored_d);
    e.mem.set_u32(out + 0x3c, stored_d);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d8_81d0,
        fn_00d881d0(u32, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        // -0.6f, 1/64 and -0.4f as the exe's doubles.
        e.mem.set_f64(MINUS_SIXTH_TENTHS, f64::from(-0.6f32));
        e.mem.set_f64(ONE_SIXTY_FOURTH, 0.015625);
        e.mem.set_f64(MINUS_FOUR_TENTHS, f64::from(-0.4f32));
        e
    }

    fn run(e: &mut Engine, start: u32, size: i32) -> Vec<u32> {
        let out = e.mem.alloc(0x50);
        e.call(
            0x00d8_81d0,
            &args![0xaau32, 0xbbu32, 0xccu32, start, size as u32, out],
        );
        (0..0x14).map(|word| e.mem.u32(out + 4 * word)).collect()
    }

    /// The layout worked out with integer arithmetic only: the truncations of the two scaled
    /// products are exact quotients of whole numbers here.
    fn expected(start: i32, size: i32) -> Vec<u32> {
        let n = size - size % 16;
        let a = (start + 15) & !15;
        let b = (a + n * 10 / 64) & !15;
        let c = (b + n * 4 / 64) & !15;
        let forty = f64::from((n * 40) as f32);
        let d = (c - (f64::from(-0.6f32) * forty / 64.0) as i32) & !15;
        let f = (d - (forty * f64::from(-0.4f32) / 64.0) as i32) & !15;
        let mut words = vec![0u32; 0x14];
        words[0] = 0xaa;
        words[1] = 0xbb;
        words[2] = 0xcc;
        for (index, value) in [
            (3, a),
            (5, a),
            (4, b),
            (6, b),
            (7, c),
            (8, c),
            (10, c),
            (11, c),
            (9, d),
            (12, d),
            (14, d),
            (15, d),
            (13, f),
            (16, f),
            (18, f),
            (19, f),
            (17, start + n),
        ] {
            words[index] = value as u32;
        }
        words
    }

    #[test]
    fn the_layout_follows_the_alignments_and_the_scaled_offsets() {
        let mut e = engine();
        for (start, size) in [
            (0x1000, 0x400),
            (0x1003, 0x1234),
            (100, 1000),
            (0, 0),
            (0x2008, 15),
        ] {
            assert_eq!(
                run(&mut e, start, size),
                expected(start as i32, size),
                "{start:#x} {size:#x}"
            );
        }
    }

    #[test]
    fn a_negative_size_is_cut_toward_zero() {
        let mut e = engine();
        // -50 % 16 = -2 as in C: n = -48.
        let words = run(&mut e, 0x1000, -50);
        assert_eq!(words[17], 0x1000u32.wrapping_sub(48));
        assert_eq!(words, expected(0x1000, -50));
    }

    #[test]
    fn nothing_beyond_the_record_is_written() {
        let mut e = engine();
        let out = e.mem.alloc(0x60);
        e.mem.set_u32(out + 0x50, 0x1234_5678);
        e.call(
            0x00d8_81d0,
            &args![1u32, 2u32, 3u32, 0x100u32, 0x100u32, out],
        );
        assert_eq!(e.mem.u32(out + 0x50), 0x1234_5678);
    }
}
