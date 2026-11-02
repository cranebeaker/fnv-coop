//! `FaceGen/run_00ac6220` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte set once the random generator is seeded (`011f6c78`).
const SEEDED: u32 = 0x011f_6c78;
/// `_time64` (`00ecadda`, cdecl: a pointer), `srand` (`00ecada6`, cdecl: the seed) and `rand`
/// (`00ecadb8`, cdecl).
const TIME64: u32 = 0x00ec_adda;
const SRAND: u32 = 0x00ec_ada6;
const RAND: u32 = 0x00ec_adb8;
/// `32768.0`, `2.0`, `-1.0` and `-2.0` (`double`s).
const RAND_SCALE: u32 = 0x0103_1140;
const TWO: u32 = 0x0101_1590;
const MINUS_ONE: u32 = 0x0101_a6b0;
const MINUS_TWO: u32 = 0x0106_d4c8;
/// The CRT math functions the normal draws call with a `double` (`00eca920`, whose library
/// match Ghidra cannot tell apart, and `00ec6054`), each answering in `ST0`.
const MATH_00ECA920: u32 = 0x00ec_a920;
const MATH_00EC6054: u32 = 0x00ec_6054;
/// The word `fn_00ac63f0` tests and `fn_00ac6410` sets once (`011f6c88`), and the byte it
/// clears with it (`011f6c8c`).
const ONCE_WORD: u32 = 0x011f_6c88;
const ONCE_BYTE: u32 = 0x011f_6c8c;

/// A `double` argument as its two words, low first.
fn double_words(value: f64) -> [u32; 2] {
    let bits = value.to_bits();
    [bits as u32, (bits >> 32) as u32]
}

// Translated from 00ac6220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, no arguments): seeds the C generator once (`srand` of the
/// low word of `_time64(0)`, the flag at `011f6c78`), then builds a `double` from four `rand()`
/// draws (`acc = rand() + acc * 32768`) and answers `acc / 32768^4` in `ST0`.
pub fn fn_00ac6220(e: &mut Engine) -> f64 {
    if e.mem.u8(SEEDED) == 0 {
        let time = e.call(TIME64, &args![0u32]).u32();
        e.call(SRAND, &args![time]);
        e.mem.set_u8(SEEDED, 1);
    }
    let scale = x87_load64(e.mem.u64(RAND_SCALE));
    let mut sum = 0.0f64;
    let mut i = 0i32;
    while i < 4 {
        let scaled = x87_mul(sum, scale);
        let draw = e.call(RAND, &[]).u32() as i32;
        sum = x87_add(f64::from(draw), scaled);
        i += 1;
    }
    let divisor = x87_mul(x87_mul(x87_mul(scale, scale), scale), scale);
    x87_div(sum, divisor)
}

/// The polar draw both normal functions make: two uniform values mapped to `[-1, 1]`
/// (`* 2 + -1`) until their squared length `s` is at most 1 (`FCOMP`: a NaN ends the loop)
/// and not 0; returns `(s, second value)`.
fn polar_pair(e: &mut Engine) -> (f64, f64) {
    let two: f64 = e.global(TWO);
    let minus_one: f64 = e.global(MINUS_ONE);
    loop {
        let a = x87_add(x87_mul(fn_00ac6220(e), two), minus_one);
        let b = x87_add(x87_mul(fn_00ac6220(e), two), minus_one);
        let s = x87_add(x87_mul(a, a), x87_mul(b, b));
        // FLD1; FCOMP s; TEST AH,5; JNP: again while 1 < s.
        if 1.0 < s {
            continue;
        }
        // FLDZ; FCOMP s; TEST AH,0x44; JNP: again while s == 0.
        if s == 0.0 {
            continue;
        }
        return (s, b);
    }
}

/// `MATH_00EC6054(MATH_00ECA920(s) * -2 / s)`, in `ST0`.
fn polar_factor(e: &mut Engine, s: f64) -> f64 {
    let first = e.call(MATH_00ECA920, &double_words(s)).f64();
    let minus_two: f64 = e.global(MINUS_TWO);
    let ratio = x87_div(x87_mul(first, minus_two), s);
    e.call(MATH_00EC6054, &double_words(ratio)).f64()
}

// Translated from 00ac62b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a block of `double`s, no stack words): a polar
/// draw (`s`, `b`), then `factor(s) * ((1 / this[2]) * b) + this[0]` (the `double`s at +0 and
/// +0x10), in `ST0`.
pub fn fn_00ac62b0(e: &mut Engine, this: Ptr) -> f64 {
    let (s, b) = polar_pair(e);
    let offset = x87_load64(e.mem.u64(this.addr()));
    let inverse = x87_div(1.0, x87_load64(e.mem.u64(this.addr() + 0x10)));
    let scaled = x87_mul(inverse, b);
    let factor = polar_factor(e, s);
    x87_add(x87_mul(factor, scaled), offset)
}

// Translated from 00ac6360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, no arguments): a polar draw (`s`, `b`), then
/// `factor(s) * b`, in `ST0`.
pub fn fn_00ac6360(e: &mut Engine) -> f64 {
    let (s, b) = polar_pair(e);
    let factor = polar_factor(e, s);
    x87_mul(factor, b)
}

// Translated from 00ac63f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): whether the word at `011f6c88` is set.
pub fn fn_00ac63f0(e: &mut Engine) -> bool {
    e.mem.u32(ONCE_WORD) != 0
}

// Translated from 00ac6410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): when the word at `011f6c88` is not set yet,
/// sets it to `value` and clears the byte at `011f6c8c`.
pub fn fn_00ac6410(e: &mut Engine, value: u32) {
    if e.mem.u32(ONCE_WORD) == 0 {
        e.mem.set_u32(ONCE_WORD, value);
        e.mem.set_u8(ONCE_BYTE, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ac_6220, fn_00ac6220() -> f64),
        entry!(0x00ac_62b0, fn_00ac62b0(Ptr) -> f64),
        entry!(0x00ac_6360, fn_00ac6360() -> f64),
        entry!(0x00ac_63f0, fn_00ac63f0() -> bool),
        entry!(0x00ac_6410, fn_00ac6410(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The constants, `_time64` (0x1234), `srand`, and `rand` answering `draws` in turn.
    fn engine(draws: Vec<u32>) -> Engine {
        let mut e = Engine::new();
        for (addr, value) in [
            (RAND_SCALE, 32768.0f64),
            (TWO, 2.0),
            (MINUS_ONE, -1.0),
            (MINUS_TWO, -2.0),
        ] {
            e.map(addr, 8);
            e.mem.set_f64(addr, value);
        }
        e.map(SEEDED, 4);
        e.map(ONCE_WORD, 8);
        e.register(TIME64, |_, _| 0x1234u32.into_ret());
        e.register(SRAND, |_, _| Ret::default());
        let next = std::cell::Cell::new(0usize);
        e.register_double(RAND, move |_, _| {
            let i = next.get();
            next.set(i + 1);
            draws[i % draws.len()].into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn uniform_draw_seeds_once_and_scales() {
        let mut e = engine(vec![0x4000, 0, 0, 0]);
        // 0x4000 * 32768^3 / 32768^4 = 0.5.
        assert_eq!(e.call(0x00ac_6220, &[]).f64(), 0.5);
        assert_eq!(calls(&e, SRAND), vec![vec![0x1234]]);
        assert_eq!(e.mem.u8(SEEDED), 1);
        e.call(0x00ac_6220, &[]);
        assert_eq!(calls(&e, SRAND).len(), 1);
        assert_eq!(calls(&e, RAND).len(), 8);
    }

    /// `rand` draws making uniform values `u` (each from four draws).
    fn uniform(u: &[f64]) -> Vec<u32> {
        let mut out = vec![];
        for v in u {
            let bits = (v * 32768.0f64.powi(4)) as u64;
            for k in (0..4).rev() {
                out.push(((bits >> (15 * k)) & 0x7fff) as u32);
            }
        }
        out
    }

    #[test]
    fn normal_draw_retries_outside_the_disc() {
        // First pair (1, 1) -> s = 2: again; second pair (0.75, 0.5) -> a = 0.5, b = 0: s =
        // 0.25.
        let mut e = engine(uniform(&[
            1.0 - 1.0 / 32768.0f64.powi(4),
            0.999_999,
            0.75,
            0.5,
        ]));
        e.register(MATH_00ECA920, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)),
            ..Ret::default()
        });
        e.register(MATH_00EC6054, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)),
            ..Ret::default()
        });
        let r = e.call(0x00ac_6360, &[]).f64();
        // factor = (0.25 * -2) / 0.25 = -2; times b = 0.
        assert_eq!(r, 0.0);
        assert_eq!(calls(&e, MATH_00ECA920), vec![double_words(0.25).to_vec()]);
        assert_eq!(calls(&e, MATH_00EC6054), vec![double_words(-2.0).to_vec()]);
    }

    #[test]
    fn normal_draw_with_parameters() {
        // One pair (0.75, 0.875): a = 0.5, b = 0.75, s = 0.8125.
        let mut e = engine(uniform(&[0.75, 0.875]));
        e.register(MATH_00ECA920, |_, _| Ret {
            st0: 0.8125,
            ..Ret::default()
        });
        e.register(MATH_00EC6054, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        let block = e.mem.alloc(0x18);
        e.mem.set_f64(block, 10.0);
        e.mem.set_f64(block + 0x10, 4.0);
        // 2 * ((1 / 4) * 0.75) + 10.
        assert_eq!(e.call(0x00ac_62b0, &args![block]).f64(), 10.375);
    }

    #[test]
    fn once_word_is_set_only_once() {
        let mut e = engine(vec![0]);
        assert!(!e.call(0x00ac_63f0, &[]).bool());
        e.mem.set_u8(ONCE_BYTE, 9);
        e.call(0x00ac_6410, &args![0x55u32]);
        assert!(e.call(0x00ac_63f0, &[]).bool());
        assert_eq!((e.mem.u32(ONCE_WORD), e.mem.u8(ONCE_BYTE)), (0x55, 0));
        e.mem.set_u8(ONCE_BYTE, 9);
        e.call(0x00ac_6410, &args![0x66u32]);
        assert_eq!((e.mem.u32(ONCE_WORD), e.mem.u8(ONCE_BYTE)), (0x55, 9));
    }
}
