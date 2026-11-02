//! `Havok SDK/run_00c90b80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor `00c90b80` fills an object with 0xe4 bytes of defaults (vtable
//! `0102f0ec`; the class is unconfirmed: its first vector is `(0, -9.8, 0, 0)`, so it looks
//! like a set of world construction values). The defaults are `float` constants read from
//! the exe's data; the ones the code moves with `FLD`/`FSTP` come back through the x87 stack,
//! which quiets a signalling NaN (`x87_quiet_bits`), and the ones it moves with `MOVSS`/
//! `UNPCKLPS` keep their bits.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable word the constructor and the second function write (`0102f0ec`).
const OBJECT_VTABLE: u32 = 0x0102_f0ec;
/// `00c905b0` (`this` only): run on a pointer field that is not null when the constructor
/// resets it. The constructor zeroes `+0x58`, `+0x5c` and `+0x6c` at its start, so these
/// calls never happen.
const RELEASE_FIELD: u32 = 0x00c9_05b0;

/// Copies the `float` at `source` to `destination` through the x87 stack (`FLD`/`FSTP`).
fn copy_float(e: &mut Engine, source: u32, destination: u32) {
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(destination, bits);
}

/// Runs `00c905b0` on the pointer stored at `field` when it is not null, then clears it.
fn release_field(e: &mut Engine, field: u32) {
    let pointer = e.mem.u32(field);
    if pointer != 0 {
        e.call(RELEASE_FIELD, &args![pointer]);
    }
    e.mem.set_u32(field, 0);
}

// Translated from 00c90b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: writes the defaults of the object (see the module note) and returns `this`.
/// Vectors (`MOVAPS`), with `A`, `B`, `C`, `D` the floats at `01011d78`, `010c6370`, `01059ac8`
/// and `01013d84`: `+0x10` = `(A, B, A, A)`, `+0x30` = `(C, C, C, A)` and `+0x40` = `(D, D, D,
/// A)` (the `UNPCKLPS` sequences leave exactly these lanes; the register lanes they never
/// define do not reach the result). Floats at `+0x78`, `+0x88`, `+0x90`, `+0x94`, `+0x54`,
/// `+0xac`, `+0xc0` and `+0xc4` (one source: `FST` then `FSTP`), `+0xc8`, `+0xcc`, `+0xa8`,
/// `+0x60`, `+0x68`, `+0x9c` and `+0xdc` are copied from the exe constants through the x87 stack
/// (`FLD`/`FSTP`); `1.0` goes to `+0x24` and `+0x7c`.
pub fn fn_00c90b80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, OBJECT_VTABLE);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x6c, 0);
    // Vectors, lane by lane (MOVSS loads keep the bits).
    let zero = e.mem.u32(0x0101_1d78);
    let down = e.mem.u32(0x010c_6370);
    for (lane, word) in [zero, down, zero, zero].into_iter().enumerate() {
        e.mem.set_u32(base + 0x10 + 4 * lane as u32, word);
    }
    e.mem.set_u8(base + 0xd2, 1);
    e.mem.set_u32(base + 0x20, 0x400);
    e.mem.set_u8(base + 0x29, 0);
    let low = e.mem.u32(0x0105_9ac8);
    for (lane, word) in [low, low, low, zero].into_iter().enumerate() {
        e.mem.set_u32(base + 0x30 + 4 * lane as u32, word);
    }
    let high = e.mem.u32(0x0101_3d84);
    for (lane, word) in [high, high, high, zero].into_iter().enumerate() {
        e.mem.set_u32(base + 0x40 + 4 * lane as u32, word);
    }
    release_field(e, base + 0x58);
    release_field(e, base + 0x5c);
    copy_float(e, 0x0101_8180, base + 0x78);
    e.mem.set_u32(base + 0x70, 0);
    e.mem.set_u32(base + 0x64, 0xfa);
    e.mem.set_u32(base + 0x7c, 0x3f80_0000);
    e.mem.set_u32(base + 0x84, 1);
    e.mem.set_u32(base + 0x24, 0x3f80_0000);
    e.mem.set_u32(base + 0x80, 4);
    copy_float(e, 0x0104_da48, base + 0x88);
    e.mem.set_u8(base + 0x8c, 0);
    copy_float(e, 0x010c_636c, base + 0x90);
    copy_float(e, 0x010c_6368, base + 0x94);
    copy_float(e, 0x0101_e2bc, base + 0x54);
    e.mem.set_u8(base + 0x98, 0);
    e.mem.set_u8(base + 0x28, 0);
    copy_float(e, 0x0101_df44, base + 0xac);
    e.mem.set_u8(base + 0x52, 0);
    e.mem.set_u8(base + 0x50, 0);
    e.mem.set_u8(base + 0x51, 0);
    e.mem.set_u8(base + 0x53, 1);
    // One FLD, stored twice (FST, then FSTP).
    let three = x87_quiet_bits(e.mem.u32(0x0101_7718));
    e.mem.set_u8(base + 0xb4, 1);
    e.mem.set_u32(base + 0xc0, three);
    e.mem.set_u32(base + 0xb0, 4);
    e.mem.set_u32(base + 0xc4, three);
    e.mem.set_u32(base + 0xb8, 8);
    copy_float(e, 0x0101_e570, base + 0xc8);
    e.mem.set_u32(base + 0xbc, 1);
    copy_float(e, 0x0101_7868, base + 0xcc);
    copy_float(e, 0x0102_d934, base + 0xa8);
    copy_float(e, 0x0102_226c, base + 0x60);
    copy_float(e, 0x0102_09a0, base + 0x68);
    copy_float(e, 0x0101_3ea4, base + 0x9c);
    e.mem.set_u8(base + 0x99, 0);
    e.mem.set_u32(base + 0xa0, 0x14);
    e.mem.set_u8(base + 0xd0, 1);
    e.mem.set_u8(base + 0xa7, 1);
    e.mem.set_u32(base + 0xd4, 0x40);
    e.mem.set_u8(base + 0xa4, 0);
    e.mem.set_u8(base + 0xa5, 0);
    e.mem.set_u8(base + 0xa6, 0);
    e.mem.set_u8(base + 0x74, 2);
    e.mem.set_u8(base + 0x75, 0);
    copy_float(e, 0x0101_41cc, base + 0xdc);
    e.mem.set_u8(base + 0xd1, 2);
    release_field(e, base + 0x6c);
    e.mem.set_u8(base + 0xd8, 1);
    e.mem.set_u8(base + 0xd9, 0);
    e.mem.set_u8(base + 0xe0, 0);
    this
}

// Translated from 00c90db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `x` the float argument: the four lanes at `+0x30` are set to `x * -0.5` and the four
/// at `+0x40` to `x * 0.5` (doubles read from `01011598` and `01011588`; x87 products stored as
/// floats, the x87 choice of NaN). The class is the one `00c90b80` builds (unconfirmed).
pub fn fn_00c90db0(e: &mut Engine, this: Ptr, x: f32) {
    let base = this.addr();
    let value = x87_load32(x.to_bits());
    let minus_half: f64 = e.global(0x0101_1598);
    let half: f64 = e.global(0x0101_1588);
    let low = x87_store32(x87_mul(minus_half, value));
    let high = x87_store32(x87_mul(value, half));
    for lane in 0..4 {
        e.mem.set_u32(base + 0x30 + 4 * lane, low);
        e.mem.set_u32(base + 0x40 + 4 * lane, high);
    }
}

// Translated from 00c90e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable word `0102f0ec` and, when `flag` is not zero and the float at `+0x24`
/// equals `0.0` (the x87 compare against the double at `01012060`; a NaN does not), sets it
/// to `1.0`. Returns `this`. The class is the one `00c90b80` builds (unconfirmed).
pub fn fn_00c90e00(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, OBJECT_VTABLE);
    if flag != 0 {
        let value = x87_load32(e.mem.u32(base + 0x24));
        let zero: f64 = e.global(0x0101_2060);
        if value == zero {
            e.mem.set_u32(base + 0x24, 0x3f80_0000);
        }
    }
    this
}

// Translated from 00c90e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (its only reference is the vtable slot at `010c6394`; the class-name string
/// `hkpContactMgrFactory` follows that table, the class is unconfirmed) that reads nothing and returns 0; the `RET 0x24` takes nine words.
pub fn fn_00c90e30(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_0b80, fn_00c90b80(Ptr) -> Ptr),
        entry!(0x00c9_0db0, fn_00c90db0(Ptr, f32)),
        entry!(0x00c9_0e00, fn_00c90e00(Ptr, u32) -> Ptr),
        entry!(
            0x00c9_0e30,
            fn_00c90e30(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// The exe constants the constructor reads, with a signalling NaN at one of them.
    const SOURCES: [u32; 18] = [
        0x0101_1d78,
        0x010c_6370,
        0x0105_9ac8,
        0x0101_3d84,
        0x0101_8180,
        0x0104_da48,
        0x010c_636c,
        0x010c_6368,
        0x0101_e2bc,
        0x0101_df44,
        0x0101_7718,
        0x0101_e570,
        0x0101_7868,
        0x0102_d934,
        0x0102_226c,
        0x0102_09a0,
        0x0101_3ea4,
        0x0101_41cc,
    ];

    /// Every default lands where the exe puts it: the whole 0xe8-byte image is compared with
    /// the expected one (a fill byte marks what the constructor leaves alone).
    #[test]
    fn constructor_writes_every_default_and_nothing_else() {
        let mut e = engine();
        // Distinct patterns per source; the one for +0x78 is a signalling NaN.
        for (i, &at) in SOURCES.iter().enumerate() {
            e.mem.set_u32(at, 0x3f00_0000 + 0x1111 * (i as u32 + 1));
        }
        e.mem.set_u32(0x0101_8180, 0x7f80_0001);
        let source = |e: &Engine, at: u32| e.mem.u32(at);
        let this = e.mem.alloc(0xe8);
        e.mem.write(this, &[0xaa; 0xe8]);
        let got = e.call(0x00c9_0b80, &args![this]).u32();
        assert_eq!(got, this);
        let mut image = [0xaau8; 0xe8];
        let mut put32 = |at: usize, value: u32| {
            image[at..at + 4].copy_from_slice(&value.to_le_bytes());
        };
        put32(0, 0x0102_f0ec);
        put32(0x58, 0);
        put32(0x5c, 0);
        put32(0x6c, 0);
        let zero = source(&e, 0x0101_1d78);
        let down = source(&e, 0x010c_6370);
        let low = source(&e, 0x0105_9ac8);
        let high = source(&e, 0x0101_3d84);
        for (lane, word) in [zero, down, zero, zero].into_iter().enumerate() {
            put32(0x10 + 4 * lane, word);
        }
        for (lane, word) in [low, low, low, zero].into_iter().enumerate() {
            put32(0x30 + 4 * lane, word);
        }
        for (lane, word) in [high, high, high, zero].into_iter().enumerate() {
            put32(0x40 + 4 * lane, word);
        }
        for (offset, at) in [
            (0x78, 0x0101_8180),
            (0x88, 0x0104_da48),
            (0x90, 0x010c_636c),
            (0x94, 0x010c_6368),
            (0x54, 0x0101_e2bc),
            (0xac, 0x0101_df44),
            (0xc0, 0x0101_7718),
            (0xc4, 0x0101_7718),
            (0xc8, 0x0101_e570),
            (0xcc, 0x0101_7868),
            (0xa8, 0x0102_d934),
            (0x60, 0x0102_226c),
            (0x68, 0x0102_09a0),
            (0x9c, 0x0101_3ea4),
            (0xdc, 0x0101_41cc),
        ] {
            put32(offset, x87_quiet_bits(source(&e, at)));
        }
        for (offset, word) in [
            (0x20, 0x400),
            (0x24, 0x3f80_0000),
            (0x64, 0xfa),
            (0x70, 0),
            (0x7c, 0x3f80_0000),
            (0x80, 4),
            (0x84, 1),
            (0xa0, 0x14),
            (0xb0, 4),
            (0xb8, 8),
            (0xbc, 1),
            (0xd4, 0x40),
        ] {
            put32(offset, word);
        }
        // Byte fields; the 16-bit word at +6 is 1.
        image[6] = 1;
        image[7] = 0;
        for (offset, value) in [
            (0x28, 0),
            (0x29, 0),
            (0x50, 0),
            (0x51, 0),
            (0x52, 0),
            (0x53, 1),
            (0x74, 2),
            (0x75, 0),
            (0x8c, 0),
            (0x98, 0),
            (0x99, 0),
            (0xa4, 0),
            (0xa5, 0),
            (0xa6, 0),
            (0xa7, 1),
            (0xb4, 1),
            (0xd0, 1),
            (0xd1, 2),
            (0xd2, 1),
            (0xd8, 1),
            (0xd9, 0),
            (0xe0, 0),
        ] {
            image[offset] = value;
        }
        assert_eq!(e.mem.bytes(this, 0xe8), image.to_vec());
        // The signalling NaN came back quiet.
        assert_eq!(e.mem.u32(this + 0x78), 0x7fc0_0001);
    }

    /// The box half-extents are `x * -0.5` and `x * 0.5` in all four lanes each.
    #[test]
    fn half_extents_are_minus_and_plus_half_of_the_argument() {
        let mut e = engine();
        e.mem.set_f64(0x0101_1598, -0.5);
        e.mem.set_f64(0x0101_1588, 0.5);
        let this = e.mem.alloc(0x60);
        e.call(0x00c9_0db0, &args![this, 3.0f32]);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0x30 + 4 * lane), -1.5);
            assert_eq!(e.mem.f32(this + 0x40 + 4 * lane), 1.5);
        }
        // A signalling NaN is quieted by the x87 load and comes out as a quiet NaN.
        e.call(0x00c9_0db0, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x30) & 0x7fc0_0000, 0x7fc0_0000);
        assert_eq!(e.mem.u32(this + 0x4c) & 0x7fc0_0000, 0x7fc0_0000);
    }

    /// The vtable word is always written; with a flag a zero (not a NaN) float at +0x24
    /// becomes 1.0.
    #[test]
    fn flag_turns_a_zero_float_into_one() {
        let mut e = engine();
        e.mem.set_f64(0x0101_2060, 0.0);
        let this = e.mem.alloc(0x40);
        let cases = [
            (0u32, 0x0000_0000u32, 0x0000_0000u32),
            (1, 0x0000_0000, 0x3f80_0000),
            (1, 0x8000_0000, 0x3f80_0000),
            (1, 0x4000_0000, 0x4000_0000),
            (1, 0x7fc0_0000, 0x7fc0_0000),
            (0, 0x7f80_0001, 0x7f80_0001),
        ];
        for (flag, before, after) in cases {
            e.mem.set_u32(this, 0);
            e.mem.set_u32(this + 0x24, before);
            assert_eq!(e.call(0x00c9_0e00, &args![this, flag]).u32(), this);
            assert_eq!(e.mem.u32(this), 0x0102_f0ec);
            assert_eq!(
                e.mem.u32(this + 0x24),
                after,
                "flag {flag} before {before:#x}"
            );
        }
    }

    /// The virtual method that does nothing returns 0 whatever it is given.
    #[test]
    fn the_nine_word_method_returns_zero() {
        let mut e = engine();
        assert_eq!(
            e.call(
                0x00c9_0e30,
                &args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32]
            )
            .u32(),
            0
        );
    }
}
