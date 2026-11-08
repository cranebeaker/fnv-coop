//! The differential test of every typed function of the C runtime family (`engine_typed::crt`)
//! against its translation, under both control words (the family's row is `Threads::Both`). The
//! exe's constants the routines read are written where the exe has them; strings and buffers sit in
//! a page whose next page is unmapped, so a scan that read past the page of a byte the exe reads
//! would stop where the translation doesn't.
//!
//! What stands in: `__getptd_noexit`, `_getptd`, `__raise_exc_ex`, `___libm_error_support`,
//! `__cintrindisp2` and the locale-aware case routines, each logging its words; everything else runs
//! translated.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Case, Diff, Input};
use engine_typed::crt::{self, math, mem, string};

/// The most calls one run makes here is a few dozen.
const MAX_CALLS: usize = 2_000;
/// Swept cases per float test.
const FLOAT_SWEEP: usize = 48;

/// The log the stand-ins write (a count, then `(tag, word)` pairs).
const LOG: u32 = 0x0300_0000;
/// The page the strings and source buffers sit in; the page after it is unmapped.
const PAGE: u32 = 0x0500_0000;
/// A writable page for destinations.
const OUT: u32 = 0x0502_0000;

/// The thread data block `__getptd_noexit` and `_getptd` hand back.
const PTD: u32 = 0x0504_0000;

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

fn log(e: &mut Engine, tag: u32, word: u32) {
    let n = e.mem.u32(LOG);
    e.mem.set_u32(LOG + 4 + 8 * n, tag);
    e.mem.set_u32(LOG + 8 + 8 * n, word);
    e.mem.set_u32(LOG, n + 1);
}

/// The double's two words.
fn words(x: f64) -> [u32; 2] {
    let b = x.to_bits();
    [b as u32, (b >> 32) as u32]
}

/// The strings: (offset in [`PAGE`], bytes without the zero). The last ends on the page's last byte.
const STRINGS: &[(u32, &[u8])] = &[
    (0x000, b""),
    (0x010, b"Hello"),
    (0x020, b"hello"),
    (0x030, b"HELLO world"),
    (0x050, b"hello World!"),
    (0x070, b"help"),
    (0x080, b"\x80\xff\x7f bytes"),
    (0x0a0, b"abc[\\]^_`{AZaz@"),
    (
        0x100,
        b"meshes\\Characters\\_Male\\Skeleton.NIF and a long tail past a scan chunk of sixty-four",
    ),
    (
        0x200,
        b"MESHES\\characters\\_male\\skeleton.nif and a long tail past a scan chunk of sixty-four",
    ),
    (0x300, b"ZEBRA zone"),
    (0x320, b"zebra ZONE"),
    (0xffb, b"tail"),
];

/// The strings' addresses.
const S: &[u32] = &[
    PAGE,
    PAGE + 0x10,
    PAGE + 0x20,
    PAGE + 0x30,
    PAGE + 0x50,
    PAGE + 0x70,
    PAGE + 0x80,
    PAGE + 0xa0,
    PAGE + 0x100,
    PAGE + 0x200,
    PAGE + 0x300,
    PAGE + 0x320,
    PAGE + 0xffb,
];

/// The control words a saved word is taken from: the two threads', the plain one, other precisions
/// and rounding modes, the precision exception unmasked, everything unmasked.
const CONTROL_WORDS: &[u32] = &[
    0x027f, 0x007f, 0x037f, 0x0c7f, 0x087f, 0x047f, 0x005f, 0x0040,
];

/// The exe's constants these routines read, the pages they live on, and the stand-ins.
fn world(e: &mut Engine) {
    e.map(LOG, 0x2000);
    for page in [
        0x0101_2000,
        0x010f_1000,
        0x010f_2000,
        0x011c_1000,
        0x011c_2000,
        0x0126_f000,
        0x0127_0000,
    ] {
        e.map(page, 0x1000);
    }
    e.map(PAGE, 0x1000);
    e.map(OUT, 0x1000);
    e.map(PTD, 0x1000);
    // The extended indefinite NaN, the doubles 1.0 and 0.0, +infinity and the largest double.
    e.mem.set_u64(0x011c_1d00, 0xc000_0000_0000_0000);
    e.mem.set_u16(0x011c_1d08, 0xffff);
    e.mem.set_u64(0x010f_2a90, 1.0f64.to_bits());
    e.mem.set_u64(0x0101_2060, 0);
    e.mem.set_u64(0x011c_24f8, f64::INFINITY.to_bits());
    e.mem.set_u64(0x011c_2508, f64::MAX.to_bits());
    // `floor`'s SSE2 constants.
    e.mem.set_u64(0x010f_1f30, 1.0f64.to_bits());
    e.mem.set_u64(0x010f_1f40, 0x433);
    e.mem.set_u64(0x010f_1f50, (-1.0f64).to_bits());
    e.mem.set_u64(0x010f_1f60, (-0.0f64).to_bits());
    e.mem.set_u64(0x010f_1f70, 0x7ff);
    // `floor`'s x87 body: its round-down control word.
    e.mem.set_u16(0x011c_2408, 0x173f);
    for (offset, text) in STRINGS {
        e.mem.write(PAGE + offset, text);
        e.mem.set_u8(PAGE + offset + text.len() as u32, 0);
    }
    for i in 0..0x40u32 {
        e.mem.set_u8(OUT + i, 0x40 + i as u8);
    }
    e.register(0x00ed_1165, |e, _| {
        log(e, 0x1165, 0);
        ret(PTD)
    });
    e.register(0x00ed_11de, |e, _| {
        log(e, 0x11de, 0);
        ret(PTD)
    });
    e.register(0x00ed_5255, |e, a| {
        for w in a {
            log(e, 0x5255, *w);
        }
        ret(0)
    });
    e.register(0x00ed_429f, |e, a| {
        log(e, 0x429f, a[3]);
        let slot = a[0];
        let v = e.mem.u64(slot) ^ 1;
        e.mem.set_u64(slot, v);
        ret(0)
    });
    e.register(0x00ed_6070, |e, a| {
        log(e, 0x6070, a[0]);
        Ret {
            st0: f64::from_bits(u64::from(a[1]) | u64::from(a[2]) << 32)
                - f64::from_bits(u64::from(a[3]) | u64::from(a[4]) << 32),
            ..Ret::default()
        }
    });
    // `__decode_pointer` (it reads the thread's slots through the kernel): no handler. And
    // `__invoke_watson`, which the invalid-parameter report ends in without a handler (it ends the
    // process): logged instead.
    e.register(0x00ed_0f92, |e, a| {
        log(e, 0x0f92, a[0]);
        ret(0)
    });
    e.register(0x00ec_7b08, |e, a| {
        log(e, 0x7b08, a[0]);
        ret(0)
    });
    // The locale-aware case routines: each logs its first word and answers a mix of two.
    e.register(0x00ec_6695, |e, a| locale_stand_in(e, 0x6695, a));
    e.register(0x00ec_a6de, |e, a| locale_stand_in(e, 0xa6de, a));
    e.register(0x00ec_680f, |e, a| locale_stand_in(e, 0x680f, a));
    e.register(0x00ec_7dce, |e, a| locale_stand_in(e, 0x7dce, a));
}

/// The globals a case sets: the math-error switch, `_matherr`'s skip, the two SSE2 switches and
/// `__locale_changed`, from one word's bits.
fn globals(e: &mut Engine, bits: u32) {
    e.mem.set_u32(0x0126_fd90, bits & 1);
    e.mem.set_u32(0x011c_2b60, bits >> 1 & 1);
    e.mem.set_u32(0x0127_0a64, bits >> 2 & 1);
    e.mem.set_u32(0x0127_0a6c, bits >> 3 & 1);
    e.mem.set_u32(0x0126_fdf4, bits >> 4 & 1);
}

const GLOBALS: &[u32] = &[0, 1, 2, 4, 8, 0xc, 0x10, 0x1f];

trait Common {
    fn common(self) -> Diff;
}

impl Common for Diff {
    fn common(self) -> Diff {
        self.family(&crt::CRT)
            .family(&leaf::LEAVES)
            .max_calls(MAX_CALLS)
            .max_cases(160)
    }
}

/// A setup from a function of the case: the world, the globals from the case's last word, the words.
fn with(
    f: impl Fn(&mut Engine, &Case) -> Vec<u32> + 'static,
) -> impl Fn(&mut Engine, &Case) -> Vec<u32> {
    move |e, case| {
        world(e);
        globals(e, case.word(case.0.len() - 1));
        f(e, case)
    }
}

// ---- The square roots and the error machinery ----

#[test]
fn ci_sqrt() {
    Diff::new(entry!(0x00ec_6040, math::ci_sqrt(f64) -> f64))
        .common()
        .inputs(&[Input::F64, Input::Of(GLOBALS)])
        .float_sweep(FLOAT_SWEEP)
        .setup(with(|_, case| words(case.f64(0)).to_vec()))
        .run()
        .assert_same();
}

#[test]
fn sqrt() {
    Diff::new(entry!(0x00ec_6054, math::sqrt(f64) -> f64))
        .common()
        .inputs(&[Input::F64, Input::Of(GLOBALS)])
        .float_sweep(FLOAT_SWEEP)
        .setup(with(|_, case| words(case.f64(0)).to_vec()))
        .run()
        .assert_same();
}

#[test]
fn sqrt_body() {
    Diff::new(entry!(
        0x00ec_605d,
        math::sqrt_body(bool, f64, u32, u32) -> f64
    ))
    .common()
    .inputs(&[Input::F64, Input::Of(CONTROL_WORDS), Input::Of(GLOBALS)])
    .float_sweep(FLOAT_SWEEP)
    .setup(with(|e, case| {
        engine_core::units::crt::set_x87_control_word(e, case.word(1) as u16);
        let [low, high] = words(case.f64(0));
        let zf = u32::from(high & 0x7ff0_0000 == 0x7ff0_0000);
        vec![zf, low, high, low, high]
    }))
    .run()
    .assert_same();
}

#[test]
fn nan_error_type() {
    Diff::new(entry!(
        0x00ed_27ac,
        math::nan_error_type(u32, f64) -> math::EaxSt0
    ))
    .common()
    .inputs(&[
        Input::Of(&[0x7ff8_0000, 0x7ff0_0001, 0xfff4_0000, 0]),
        Input::F64,
        Input::Of(&[0]),
    ])
    .setup(with(|_, case| {
        let [low, high] = words(case.f64(1));
        vec![case.word(0), low, high]
    }))
    .run()
    .assert_same();
}

#[test]
fn fload_with_fb() {
    Diff::new(entry!(
        0x00ed_27c5,
        math::fload_with_fb(u32, Ptr) -> math::EaxSt0
    ))
    .common()
    .inputs(&[Input::F64, Input::Of(&[0])])
    .setup(with(|e, case| {
        e.mem.set_u64(OUT + 8, case.f64(0).to_bits());
        vec![0x1234, OUT + 8]
    }))
    .run()
    .assert_same();
}

#[test]
fn math_exit() {
    Diff::new(entry!(
        0x00ed_282b,
        math::math_exit(u32, u32, f64, u32, u32, u32, u32) -> f64
    ))
    .common()
    .inputs(&[
        Input::Of(&[5, 0x12]),
        Input::F64,
        Input::Of(CONTROL_WORDS),
        Input::Of(GLOBALS),
    ])
    .float_sweep(FLOAT_SWEEP)
    .setup(with(|_, case| {
        let [low, high] = words(case.f64(1));
        vec![
            case.word(0),
            0x011c_1740,
            low,
            high,
            case.word(2),
            0,
            2,
            0x4000_0000,
        ]
    }))
    .run()
    .assert_same();
}

#[test]
fn start_one_arg_error_handling() {
    Diff::new(entry!(
        0x00ed_2737,
        math::start_one_arg_error_handling(u32, u32, u32, f64, u32, u32, u32, u32) -> f64
    ))
    .common()
    .inputs(&[
        Input::Of(&[8, 1, 2, 3, 4, 5, 6, 7, 9]),
        Input::Of(&[5, 0x10, 0x16, 0x1d]),
        Input::F64,
        Input::Of(CONTROL_WORDS),
        Input::Of(GLOBALS),
    ])
    .max_cases(240)
    .float_sweep(FLOAT_SWEEP)
    .setup(with(|_, case| {
        let [low, high] = words(case.f64(2));
        vec![
            case.word(0),
            0x011c_1740,
            case.word(1),
            low,
            high,
            case.word(3),
            0,
            1,
            0x4000_0000,
        ]
    }))
    .run()
    .assert_same();
}

#[test]
fn except_87() {
    Diff::new(entry!(0x00ee_1b04, math::except_87(u32, Ptr, Ptr)))
        .common()
        .inputs(&[
            Input::Of(&[5, 0x10, 0x16, 0x1d]),
            Input::Of(&[8, 1, 2, 3, 4, 5, 6, 7, 0]),
            Input::F64,
            Input::Of(CONTROL_WORDS),
            Input::Of(GLOBALS),
        ])
        .max_cases(240)
        .float_sweep(FLOAT_SWEEP)
        .setup(with(|e, case| {
            let exception = OUT + 0x100;
            e.mem.set_u32(exception, case.word(1));
            e.mem.set_u32(exception + 4, 0x011c_1740);
            e.mem.set_u64(exception + 8, 2.5f64.to_bits());
            e.mem.set_u64(exception + 0x10, case.f64(2).to_bits());
            e.mem.set_u64(exception + 0x18, (-case.f64(2)).to_bits());
            e.mem.set_u32(OUT + 0x140, case.word(3));
            vec![case.word(0), exception, OUT + 0x140]
        }))
        .run()
        .assert_same();
}

/// The flags of every branch of `__handle_exc`, alone and together.
const HANDLE_FLAGS: &[u32] = &[0x10, 8, 4, 1, 2, 0x12, 0x11, 0x18, 0x1f, 0, 0x20];
/// Control words for `__handle_exc`: each mask bit, each rounding mode.
const HANDLE_CONTROLS: &[u32] = &[0x3f, 0x01, 0x04, 0x08, 0x10, 0x20, 0x43f, 0x83f, 0xc3f, 0];

#[test]
fn handle_exc() {
    Diff::new(entry!(
        0x00ed_5554,
        math::handle_exc(u32, Ptr, u32) -> i32
    ))
    .common()
    .inputs(&[
        Input::Of(HANDLE_FLAGS),
        Input::F64,
        Input::Of(HANDLE_CONTROLS),
        Input::Of(&[0]),
    ])
    .max_cases(300)
    .float_sweep(FLOAT_SWEEP)
    .setup(with(|e, case| {
        e.mem.set_u64(OUT + 0x20, case.f64(1).to_bits());
        vec![case.word(0), OUT + 0x20, case.word(2)]
    }))
    .run()
    .assert_same();
}

/// `__handle_exc`'s underflow path over tiny results: denormals and normals at the edges where the
/// shift starts and where the result is lost.
#[test]
fn handle_exc_underflow() {
    Diff::new(entry!(
        0x00ed_5554,
        math::handle_exc(u32, Ptr, u32) -> i32
    ))
    .common()
    .inputs(&[
        Input::Of(&[2, 0x12]),
        Input::Of(&[
            0x0000_0000,
            0x0000_0001,
            0x0001_0000,
            0x0010_0000,
            0x0010_0001,
            0x0020_0000,
            0x8008_0000,
            0x3000_0000,
            0x2000_0000,
            0x1ff0_0000,
            0x6000_0000,
            0x5ff0_0003,
        ]),
        Input::Of(&[0, 1, 0x8000_0001, 0xffff_ffff]),
        Input::Of(&[0]),
    ])
    .setup(with(|e, case| {
        e.mem.set_u32(OUT + 0x20, case.word(2));
        e.mem.set_u32(OUT + 0x24, case.word(1));
        vec![case.word(0), OUT + 0x20, 0x10]
    }))
    .run()
    .assert_same();
}

#[test]
fn raise_exc() {
    Diff::new(entry!(
        0x00ed_5531,
        math::raise_exc(u32, u32, u32, u32, u32, u32) -> u32
    ))
    .common()
    .inputs(&[Input::Word, Input::Of(&[1, 0x10]), Input::Of(&[0])])
    .setup(with(|_, case| vec![case.word(0), 2, case.word(1), 4, 5, 6]))
    .run()
    .assert_same();
}

#[test]
fn errcode() {
    Diff::new(entry!(0x00ed_5763, math::errcode(u32) -> i32))
        .common()
        .inputs(&[Input::Of(&[
            0, 1, 2, 3, 4, 8, 0x10, 0x20, 0x22, 0x0f, 0x3f, 0x100, 0xff,
        ])])
        .run()
        .assert_same();
}

#[test]
fn set_errno_from_matherr() {
    Diff::new(entry!(0x00ed_5736, math::set_errno_from_matherr(i32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 2, 3, 4, 8, 0xffff_ffff, 0x8000_0000]),
            Input::Of(&[0]),
        ])
        .setup(with(|_, case| vec![case.word(0)]))
        .run()
        .assert_same();
}

#[test]
fn default_matherr() {
    Diff::new(entry!(0x00ee_0a3a, math::default_matherr(Ptr) -> u32))
        .common()
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn decomp() {
    Diff::new(entry!(0x00ed_59e1, math::decomp(f64, Ptr) -> f64))
        .common()
        .inputs(&[Input::F64, Input::Of(&[0])])
        .float_sweep(FLOAT_SWEEP)
        .setup(with(|_, case| {
            let [low, high] = words(case.f64(0));
            vec![low, high, OUT + 0x30]
        }))
        .run()
        .assert_same();
}

#[test]
fn set_exp() {
    Diff::new(entry!(0x00ed_5955, math::set_exp(f64, i32) -> f64))
        .common()
        .inputs(&[
            Input::F64,
            Input::Of(&[0, 1, 0xffff_ffff, 0x400, 0xffff_fc02, 0x7fff_ffff]),
        ])
        .float_sweep(FLOAT_SWEEP)
        .run()
        .assert_same();
}

#[test]
fn errno() {
    Diff::new(entry!(0x00ec_85e3, math::errno() -> Ptr))
        .common()
        .inputs(&[Input::Of(&[0, 1])])
        .setup(|e, case| {
            world(e);
            if case.word(0) == 0 {
                e.register(0x00ed_1165, |_, _| ret(0));
            }
            vec![]
        })
        .run()
        .assert_same();
}

// ---- Conversions and integer arithmetic ----

#[test]
fn ftol2_sse() {
    Diff::new(entry!(0x00ec_62c0, math::ftol2_sse(f64) -> u64))
        .common()
        .inputs(&[Input::F64, Input::Of(&[0, 8])])
        .float_sweep(FLOAT_SWEEP)
        .setup(with(|_, case| words(case.f64(0)).to_vec()))
        .run()
        .assert_same();
}

#[test]
fn ftol2_x87() {
    Diff::new(entry!(0x00ec_62f6, math::ftol2_x87(f64) -> u64))
        .common()
        .inputs(&[Input::F64])
        .float_sweep(FLOAT_SWEEP)
        .run()
        .assert_same();
}

#[test]
fn floor() {
    Diff::new(entry!(0x00ec_6940, math::floor(f64) -> f64))
        .common()
        .translation_calls_plainly(&[0x00ed_4563])
        .inputs(&[Input::F64, Input::Of(&[4, 0])])
        .float_sweep(FLOAT_SWEEP)
        .setup(with(|_, case| words(case.f64(0)).to_vec()))
        .run()
        .assert_same();
}

#[test]
fn isnan() {
    Diff::new(entry!(0x00ec_75b1, math::isnan(u32, u32) -> u32))
        .common()
        .inputs(&[Input::F64])
        .float_sweep(FLOAT_SWEEP)
        .run()
        .assert_same();
}

#[test]
fn finite() {
    Diff::new(entry!(0x00ec_7595, math::finite(u32, u32) -> u32))
        .common()
        .inputs(&[Input::F64])
        .float_sweep(FLOAT_SWEEP)
        .run()
        .assert_same();
}

#[test]
fn ci_fmod() {
    Diff::new(entry!(0x00ec_913a, math::ci_fmod(f64, f64) -> f64))
        .common()
        .inputs(&[Input::F64, Input::F64, Input::Of(&[0])])
        .setup(with(|_, case| {
            let mut w = words(case.f64(0)).to_vec();
            w.extend(words(case.f64(1)));
            w
        }))
        .run()
        .assert_same();
}

/// 64-bit operands as word pairs: zero, small, high words set, the sign bit, all ones.
const LOW_WORDS: &[u32] = &[0, 1, 7, 0x8000_0000, 0xffff_ffff, 0x1234_5678];
const HIGH_WORDS: &[u32] = &[0, 1, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0x0001_0000];

#[test]
fn allmul() {
    Diff::new(entry!(0x00ec_c300, math::allmul(u64, u64) -> u64))
        .common()
        .inputs(&[
            Input::Of(LOW_WORDS),
            Input::Of(HIGH_WORDS),
            Input::Of(LOW_WORDS),
            Input::Of(HIGH_WORDS),
        ])
        .run()
        .assert_same();
}

#[test]
fn alldiv() {
    Diff::new(entry!(0x00ec_c250, math::alldiv(u64, u64) -> u64))
        .common()
        .inputs(&[
            Input::Of(LOW_WORDS),
            Input::Of(HIGH_WORDS),
            Input::Of(&[1, 7, 0xffff_ffff, 0x8000_0000, 0]),
            Input::Of(&[0, 1, 0xffff_ffff, 0x8000_0000]),
        ])
        .expect_stops(|case| case.word(2) == 0 && case.word(3) == 0)
        .run()
        .assert_same();
}

#[test]
fn aulldiv() {
    Diff::new(entry!(
        0x00ec_c590,
        math::aulldiv(u32, u32, u32, u32) -> u64
    ))
    .common()
    .inputs(&[
        Input::Of(LOW_WORDS),
        Input::Of(HIGH_WORDS),
        Input::Of(&[1, 7, 0xffff_ffff, 0x8000_0000, 3]),
        Input::Of(&[0, 1, 0xffff_ffff, 0x8000_0000, 0x7fff_ffff, 2]),
    ])
    .max_cases(400)
    .run()
    .assert_same();
}

#[test]
fn rand() {
    Diff::new(entry!(0x00ec_adb8, math::rand() -> i32))
        .common()
        .inputs(&[Input::Word])
        .setup(|e, case| {
            world(e);
            e.mem.set_u32(PTD + 0x14, case.word(0));
            vec![]
        })
        .run()
        .assert_same();
}

#[test]
fn srand() {
    Diff::new(entry!(0x00ec_ada6, math::srand(u32)))
        .common()
        .inputs(&[Input::Word])
        .setup(|e, case| {
            world(e);
            vec![case.word(0)]
        })
        .run()
        .assert_same();
}

// ---- Memory ----

/// Offsets into the pages: aligned, odd, and near each other for overlaps.
const OFFSETS: &[u32] = &[0, 1, 3, 4, 0x10, 0x11, 0x20, 0x40, 0x41, 0x50, 0x84];
/// Counts: none, short, around the dword and 16-byte steps, the SSE2 limit and blocks.
const COUNTS: &[u32] = &[
    0, 1, 3, 4, 5, 7, 8, 15, 16, 33, 0xff, 0x100, 0x101, 0x180, 0x1ff, 0x203,
];

/// A source page of patterned bytes and the destination page; `(dst, src)` from two offsets, both in
/// [`OUT`] when `same` (overlapping copies), else the destination in [`OUT`] and the source in
/// [`PAGE`].
fn buffers(e: &mut Engine, dst_off: u32, src_off: u32, same: bool) -> (u32, u32) {
    for i in 0..0x400u32 {
        e.mem.set_u8(OUT + i, (i * 7 + 3) as u8);
        e.mem.set_u8(PAGE + i, (i * 13 + 5) as u8);
    }
    let dst = OUT + 0x200 + dst_off;
    let src = if same { OUT + 0x200 } else { PAGE } + src_off;
    (dst, src)
}

fn copy_inputs() -> [Input; 5] {
    [
        Input::Of(OFFSETS),
        Input::Of(OFFSETS),
        Input::Of(COUNTS),
        Input::Flag,
        Input::Of(&[0, 8]),
    ]
}

#[test]
fn memcpy() {
    Diff::new(entry!(0x00ec_44d0, mem::memcpy(u32, u32, u32) -> u32))
        .common()
        .inputs(&copy_inputs())
        .max_cases(400)
        .setup(with(|e, case| {
            let (dst, src) = buffers(e, case.word(0), case.word(1), case.word(3) == 1);
            vec![dst, src, case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn memmove() {
    Diff::new(entry!(0x00ec_7230, mem::memmove(u32, u32, u32) -> u32))
        .common()
        .inputs(&copy_inputs())
        .max_cases(400)
        .setup(with(|e, case| {
            let (dst, src) = buffers(e, case.word(0), case.word(1), case.word(3) == 1);
            vec![dst, src, case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn memset() {
    Diff::new(entry!(0x00ec_61c0, mem::memset(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(OFFSETS),
            Input::Of(&[0, 0xab, 0x1ff, 0x100, 0xffff_ff00]),
            Input::Of(COUNTS),
            Input::Of(&[0, 8]),
        ])
        .max_cases(300)
        .setup(with(|e, case| {
            let (dst, _) = buffers(e, case.word(0), 0, true);
            vec![dst, case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn vec_memcpy() {
    Diff::new(entry!(0x00ed_17f4, mem::vec_memcpy(u32, u32, u32) -> u32))
        .common()
        .inputs(&copy_inputs())
        .max_cases(400)
        .setup(with(|e, case| {
            let (dst, src) = buffers(e, case.word(0), case.word(1), case.word(3) == 1);
            vec![dst, src, case.word(2) | 0x80]
        }))
        .run()
        .assert_same();
}

#[test]
fn vec_memcpy_blocks() {
    Diff::new(entry!(0x00ed_176d, mem::vec_memcpy_blocks(u32, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x10, 0x20, 0x40, 0x50, 0x80, 0x90]),
            Input::Of(&[0, 0x10, 0x40]),
            Input::Of(&[0x80, 0x100, 0x17f, 0x180]),
            Input::Flag,
            Input::Of(&[0]),
        ])
        .setup(with(|e, case| {
            let (dst, src) = buffers(e, case.word(0), case.word(1), case.word(3) == 1);
            vec![
                dst - 0x200 * case.word(3),
                src - 0x200 * case.word(3),
                case.word(2),
            ]
        }))
        .run()
        .assert_same();
}

#[test]
fn fastzero_i() {
    Diff::new(entry!(0x00ed_2c7e, mem::fastzero_i(u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x10, 0x40]),
            Input::Of(&[0x80, 0x100, 0x17f, 0x1ff]),
            Input::Of(&[0]),
        ])
        .setup(with(|e, case| {
            let (dst, _) = buffers(e, case.word(0), 0, true);
            vec![dst - 0x200, case.word(1)]
        }))
        .run()
        .assert_same();
}

#[test]
fn vec_memzero() {
    Diff::new(entry!(0x00ed_2cd5, mem::vec_memzero(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(OFFSETS),
            Input::Of(&[0x80, 0x100, 0x101, 0x17f, 0x1f0, 0x10]),
            Input::Of(&[0]),
        ])
        .setup(with(|e, case| {
            let (dst, _) = buffers(e, case.word(0), 0, true);
            vec![dst - 0x200, 0x55, case.word(1)]
        }))
        .run()
        .assert_same();
}

/// Two buffers for `memcmp`: equal but for one byte at an offset from the case; the second sits at
/// the end of [`PAGE`], so a read past the first difference there would stop.
fn compare_buffers(e: &mut Engine, at: u32, differ: u32) -> (u32, u32) {
    let a = OUT + 0x300;
    let b = PAGE + 0x1000 - 0x40;
    for i in 0..0x40 {
        e.mem.set_u8(a + i, (i * 5) as u8);
        e.mem.set_u8(b + i, (i * 5) as u8);
    }
    if differ != 0 {
        e.mem.set_u8(a + at, differ as u8);
    }
    (a, b)
}

#[test]
fn memcmp() {
    Diff::new(entry!(0x00ec_4835, mem::memcmp(u32, u32, u32) -> i32))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 3, 4, 5, 0x1f, 0x20, 0x3f]),
            Input::Of(&[0, 0xff, 0x01, 0x80]),
            Input::Of(&[0, 1, 4, 5, 8, 0x20, 0x21, 0x40]),
            Input::Of(&[0]),
        ])
        .max_cases(256)
        .setup(with(|e, case| {
            let (a, b) = compare_buffers(e, case.word(0), case.word(1));
            vec![a, b, case.word(2)]
        }))
        .run()
        .assert_same();
}

/// `memcmp` with a count past the end of the second buffer's page: the difference before it ends
/// the compare, as in the exe.
#[test]
fn memcmp_stops_at_the_difference() {
    Diff::new(entry!(0x00ec_4835, mem::memcmp(u32, u32, u32) -> i32))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x10, 0x3c, 0x3f]),
            Input::Of(&[0xff, 0x01]),
            Input::Of(&[0x41, 0x100]),
            Input::Of(&[0]),
        ])
        .setup(with(|e, case| {
            let (a, b) = compare_buffers(e, case.word(0), case.word(1));
            vec![a, b, case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn memchr() {
    Diff::new(entry!(0x00ec_a2e0, mem::memchr(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 3, 0x40, 0xfbf]),
            Input::Of(&[0x6c, 0x48, 0x00, 0x175, 0x99]),
            Input::Of(&[0, 1, 4, 5, 0x10, 0x30, 0x41]),
            Input::Of(&[0]),
        ])
        .max_cases(256)
        .setup(with(|_, case| {
            vec![PAGE + case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn memcpy_wrapper() {
    Diff::new(entry!(
        0x0040_1460,
        mem::memcpy_wrapper(Ptr, Ptr, u32) -> Ptr
    ))
    .common()
    .inputs(&copy_inputs())
    .setup(with(|e, case| {
        let (dst, src) = buffers(e, case.word(0), case.word(1), case.word(3) == 1);
        vec![dst, src, case.word(2)]
    }))
    .run()
    .assert_same();
}

#[test]
fn memset_wrapper() {
    Diff::new(entry!(
        0x0040_3d30,
        mem::memset_wrapper(Ptr, i32, u32) -> Ptr
    ))
    .common()
    .inputs(&[
        Input::Of(OFFSETS),
        Input::Of(&[0, 0xab, 0xffff_ffff]),
        Input::Of(COUNTS),
        Input::Of(&[0, 8]),
    ])
    .setup(with(|e, case| {
        let (dst, _) = buffers(e, case.word(0), 0, true);
        vec![dst, case.word(1), case.word(2)]
    }))
    .run()
    .assert_same();
}

#[test]
fn memcpy_s() {
    Diff::new(entry!(
        0x00ec_7c66,
        mem::memcpy_s(Ptr, u32, Ptr, u32) -> u32
    ))
    .common()
    .translation_calls_plainly(&[0x00ec_85e3, 0x00ec_7c30])
    .inputs(&[
        Input::Of(&[1, 0]),
        Input::Of(&[0, 4, 0x10, 0x200]),
        Input::Of(&[1, 0]),
        Input::Of(&[0, 1, 4, 0x10, 0x11, 0x180]),
        Input::Of(&[0, 8]),
    ])
    .setup(with(|e, case| {
        let (dst, src) = buffers(e, 0, 0, false);
        vec![
            dst * case.word(0),
            case.word(1),
            src * case.word(2),
            case.word(3),
        ]
    }))
    .run()
    .assert_same();
}

// ---- Strings ----

#[test]
fn strlen() {
    Diff::new(entry!(0x00ec_6130, string::strlen(u32) -> u32))
        .common()
        .inputs(&[Input::Of(S), Input::Of(&[0])])
        .setup(with(|_, case| vec![case.word(0)]))
        .run()
        .assert_same();
}

#[test]
fn strcmp() {
    Diff::new(entry!(0x00ec_6da0, string::strcmp(u32, u32) -> i32))
        .common()
        .inputs(&[Input::Of(S), Input::Of(S), Input::Of(&[0])])
        .max_cases(200)
        .setup(with(|_, case| vec![case.word(0), case.word(1)]))
        .run()
        .assert_same();
}

#[test]
fn strcpy() {
    Diff::new(entry!(0x00ec_6370, string::strcpy(u32, u32) -> u32))
        .common()
        .inputs(&[
            // Below, inside and above the sources: just above one, the exe's dword steps smear and stop.
            Input::Of(&[
                OUT,
                OUT + 1,
                OUT + 0x3f,
                PAGE + 0xf,
                PAGE + 0x2f,
                PAGE + 0x11,
                PAGE + 0x31,
                PAGE + 0x33,
                PAGE + 0x102,
            ]),
            Input::Of(S),
            Input::Of(&[0]),
        ])
        .max_cases(200)
        .setup(with(|_, case| vec![case.word(0), case.word(1)]))
        .run()
        .assert_same();
}

#[test]
fn strchr() {
    Diff::new(entry!(0x00ec_7690, string::strchr(u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(S),
            Input::Of(&[0x6c, 0x48, 0x00, 0x16c, 0x99, 0x5c, 0x6c69]),
            Input::Of(&[0]),
        ])
        .max_cases(200)
        .setup(with(|_, case| vec![case.word(0), case.word(1)]))
        .run()
        .assert_same();
}

#[test]
fn strncmp() {
    Diff::new(entry!(0x00ec_8a19, string::strncmp(u32, u32, u32) -> i32))
        .common()
        .inputs(&[
            Input::Of(S),
            Input::Of(S),
            Input::Of(&[0, 1, 3, 4, 5, 0x50, 0xffff_ffff]),
            Input::Of(&[0]),
        ])
        .max_cases(300)
        .setup(with(|_, case| {
            vec![case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

/// Characters around the case ranges, and words with high bits.
const CHARACTERS: &[u32] = &[
    0x40,
    0x41,
    0x4d,
    0x5a,
    0x5b,
    0x60,
    0x61,
    0x7a,
    0x7b,
    0,
    0xc1,
    0x141,
    0xffff_ff41,
];

#[test]
fn tolower() {
    Diff::new(entry!(0x00ec_67aa, string::tolower(u32) -> u32))
        .common()
        .inputs(&[Input::Of(CHARACTERS), Input::Of(&[0, 0x10])])
        .setup(with(|_, case| vec![case.word(0)]))
        .run()
        .assert_same();
}

#[test]
fn toupper() {
    Diff::new(entry!(0x00ec_a7f4, string::toupper(u32) -> u32))
        .common()
        .inputs(&[Input::Of(CHARACTERS), Input::Of(&[0, 0x10])])
        .setup(with(|_, case| vec![case.word(0)]))
        .run()
        .assert_same();
}

/// String addresses with a null.
const S_OR_NULL: &[u32] = &[
    PAGE + 0x10,
    PAGE + 0x20,
    PAGE + 0x30,
    PAGE + 0x50,
    PAGE + 0x100,
    PAGE + 0x200,
    PAGE + 0xffb,
    PAGE + 0xa0,
    PAGE + 0x300,
    PAGE + 0x320,
    0,
];

#[test]
fn stricmp() {
    Diff::new(entry!(0x00ec_68e4, string::stricmp(u32, u32) -> i32))
        .common()
        .inputs(&[
            Input::Of(S_OR_NULL),
            Input::Of(S_OR_NULL),
            Input::Of(&[0, 0x10]),
        ])
        .max_cases(200)
        .setup(with(|_, case| vec![case.word(0), case.word(1)]))
        .run()
        .assert_same();
}

#[test]
fn ascii_stricmp() {
    Diff::new(entry!(
        0x00ec_67d6,
        string::ascii_stricmp(Ptr, Ptr) -> i32
    ))
    .common()
    .inputs(&[Input::Of(S), Input::Of(S), Input::Of(&[0])])
    .max_cases(200)
    .setup(with(|_, case| vec![case.word(0), case.word(1)]))
    .run()
    .assert_same();
}

#[test]
fn strnicmp() {
    Diff::new(entry!(0x00ec_7ec0, string::strnicmp(u32, u32, u32) -> i32))
        .common()
        .inputs(&[
            Input::Of(S_OR_NULL),
            Input::Of(S_OR_NULL),
            Input::Of(&[0, 3, 6, 0x60, 0x7fff_ffff, 0x8000_0000]),
            Input::Of(&[0, 0x10]),
        ])
        .max_cases(300)
        .setup(with(|_, case| {
            vec![case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn ascii_strnicmp() {
    Diff::new(entry!(
        0x00ed_5bc0,
        string::ascii_strnicmp(u32, u32, u32) -> i32
    ))
    .common()
    .inputs(&[
        Input::Of(S),
        Input::Of(S),
        Input::Of(&[0, 1, 3, 6, 0x50, 0xffff_ffff]),
        Input::Of(&[0]),
    ])
    .max_cases(300)
    .setup(with(|_, case| {
        vec![case.word(0), case.word(1), case.word(2)]
    }))
    .run()
    .assert_same();
}

#[test]
fn strcat_s() {
    Diff::new(entry!(0x00ec_6bd2, string::strcat_s(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(&[1, 0, 2]),
            Input::Of(&[0, 1, 3, 6, 0x10, 0x60]),
            Input::Of(S_OR_NULL),
            Input::Of(&[0]),
        ])
        .max_cases(200)
        .setup(with(|e, case| {
            // An empty destination, one holding "ab", or none.
            e.mem.write(OUT + 0x80, b"ab\0");
            let dst = match case.word(0) {
                0 => 0,
                1 => OUT + 0x82,
                _ => OUT + 0x80,
            };
            if dst == OUT + 0x82 {
                e.mem.set_u8(dst, 0);
            }
            vec![dst, case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn strncpy_s() {
    Diff::new(entry!(
        0x00ec_8d5f,
        string::strncpy_s(u32, u32, u32, u32) -> u32
    ))
    .common()
    .inputs(&[
        Input::Of(&[1, 0]),
        Input::Of(&[0, 1, 3, 6, 0x10, 0x60]),
        Input::Of(S_OR_NULL),
        Input::Of(&[0, 1, 3, 5, 6, 0x60, 0xffff_ffff]),
        Input::Of(&[0]),
    ])
    .max_cases(300)
    .setup(with(|_, case| {
        vec![
            (OUT + 0x80) * case.word(0),
            case.word(1),
            case.word(2),
            case.word(3),
        ]
    }))
    .run()
    .assert_same();
}

#[test]
fn strcspn() {
    Diff::new(entry!(0x00ec_dbb0, string::strcspn(u32, u32) -> u32))
        .common()
        .inputs(&[Input::Of(S), Input::Of(S), Input::Of(&[0])])
        .max_cases(200)
        .setup(with(|_, case| vec![case.word(0), case.word(1)]))
        .run()
        .assert_same();
}

#[test]
fn byteswap_ulong() {
    Diff::new(entry!(0x00ec_3bad, string::byteswap_ulong(u32) -> u32))
        .common()
        .inputs(&[Input::Of(&[
            0,
            1,
            0x1122_3344,
            0xff00_ff00,
            0x8000_0001,
            u32::MAX,
        ])])
        .run()
        .assert_same();
}

#[test]
fn type_info_equal() {
    Diff::new(entry!(
        0x00ec_40cc,
        string::type_info_equal(u32, u32) -> u32
    ))
    .common()
    .inputs(&[Input::Of(S), Input::Of(S), Input::Of(&[0])])
    .max_cases(120)
    .setup(with(|_, case| vec![case.word(0) - 9, case.word(1) - 9]))
    .run()
    .assert_same();
}

#[test]
fn find_complete_object() {
    Diff::new(entry!(
        0x00ec_40ec,
        string::find_complete_object(u32) -> u32
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 8, 0x10]),
        Input::Of(&[0, 4, 0x20]),
        Input::Of(&[0]),
    ])
    .setup(with(|e, case| {
        // A locator with the offset and displacement of the case; the word at the displacement.
        let locator = OUT + 0x400;
        e.mem.set_u32(locator + 4, case.word(0));
        e.mem.set_u32(locator + 8, case.word(1));
        let vtable = OUT + 0x420;
        e.mem.set_u32(vtable - 4, locator);
        let object = OUT + 0x480;
        e.mem.set_u32(object, vtable);
        if case.word(1) != 0 {
            e.mem.set_u32(object - case.word(1), 0x18);
        }
        vec![object]
    }))
    .run()
    .assert_same();
}

#[test]
fn pmd_to_offset() {
    Diff::new(entry!(0x00ec_4179, string::pmd_to_offset(u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(&[0, 4, 0x10]),
            Input::Of(&[0xffff_ffff, 0, 8, 0x8000_0000]),
            Input::Of(&[0, 4]),
            Input::Of(&[0]),
        ])
        .setup(with(|e, case| {
            let pmd = OUT + 0x500;
            e.mem.set_u32(pmd, case.word(0));
            e.mem.set_u32(pmd + 4, case.word(1));
            e.mem.set_u32(pmd + 8, case.word(2));
            let object = OUT + 0x540;
            let table = OUT + 0x5c0;
            if case.word(1) != 0xffff_ffff && case.word(1) != 0x8000_0000 {
                e.mem.set_u32(object + case.word(1), table);
            }
            e.mem.set_u32(table, 0x30);
            e.mem.set_u32(table + 4, 0x44);
            vec![pmd, object]
        }))
        .run()
        .assert_same();
}

/// `memchr` finding its byte before the end of the page with a count past it: no read past the page.
#[test]
fn memchr_stops_at_the_byte() {
    Diff::new(entry!(0x00ec_a2e0, mem::memchr(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(&[0xff0, 0xffb]),
            Input::Of(&[0x6c, 0x74, 0x00]),
            Input::Of(&[0x100, 0x20]),
            Input::Of(&[0]),
        ])
        .setup(with(|_, case| {
            vec![PAGE + case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

fn locale_stand_in(e: &mut Engine, tag: u32, a: &[u32]) -> Ret {
    log(e, tag, a[0]);
    ret(a[0] ^ a[1] ^ 0x5a)
}

/// A page-sized block for the long copies.
const BIG: u32 = 0x0506_0000;

/// Copies longer than one of the typed version's runs (0x1000 bytes), overlapping from above and
/// from below by a little and by more than a run.
#[test]
fn memmove_long() {
    Diff::new(entry!(0x00ec_7230, mem::memmove(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 4, 0x10, 0x100, 0x1001]),
            Input::Of(&[0x1800, 0x2001]),
            Input::Flag,
            Input::Of(&[0, 8]),
        ])
        .setup(with(|e, case| {
            e.map(BIG, 0x6000);
            let bytes: Vec<u8> = (0..0x6000u32).map(|i| (i * 11 + i / 251) as u8).collect();
            e.mem.write(BIG, &bytes);
            let (a, b) = (BIG + 0x1000, BIG + 0x1000 + case.word(0));
            let (dst, src) = if case.word(2) == 1 { (b, a) } else { (a, b) };
            vec![dst, src, case.word(1)]
        }))
        .run()
        .assert_same();
}

/// Pages around 2 GB: addresses whose signed remainder by 16 differs from their low bits.
const HIGH: u32 = 0x7fff_f000;
/// Addresses there: `7ffffff1` and `80000001` have the same low bits and remainders 1 and -15.
const HIGH_ADDRESSES: &[u32] = &[
    0x7fff_fff1,
    0x8000_0001,
    0x8000_0010,
    0x8000_0100,
    0x7fff_ff00,
    0x8000_000f,
];

fn high_pages(e: &mut Engine) {
    e.map(HIGH, 0x3000);
    let bytes: Vec<u8> = (0..0x3000u32).map(|i| (i * 7 + i / 253) as u8).collect();
    e.mem.write(HIGH, &bytes);
}

#[test]
fn vec_memcpy_high() {
    Diff::new(entry!(0x00ed_17f4, mem::vec_memcpy(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(HIGH_ADDRESSES),
            Input::Of(HIGH_ADDRESSES),
            Input::Of(&[0x80, 0x100, 0x1f0, 0x203]),
            Input::Of(&[8]),
        ])
        .setup(with(|e, case| {
            high_pages(e);
            vec![case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn memcpy_high() {
    Diff::new(entry!(0x00ec_44d0, mem::memcpy(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(HIGH_ADDRESSES),
            Input::Of(HIGH_ADDRESSES),
            Input::Of(&[0x100, 0x1f0, 0x203, 0x10]),
            Input::Of(&[8, 0]),
        ])
        .setup(with(|e, case| {
            high_pages(e);
            vec![case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}

#[test]
fn vec_memzero_high() {
    Diff::new(entry!(0x00ed_2cd5, mem::vec_memzero(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(HIGH_ADDRESSES),
            Input::Of(&[0x80, 0x101, 0x1f0, 0x100]),
            Input::Of(&[8]),
        ])
        .setup(with(|e, case| {
            high_pages(e);
            vec![case.word(0), 0, case.word(1)]
        }))
        .run()
        .assert_same();
}

#[test]
fn memset_high() {
    Diff::new(entry!(0x00ec_61c0, mem::memset(u32, u32, u32) -> u32))
        .common()
        .inputs(&[
            Input::Of(HIGH_ADDRESSES),
            Input::Of(&[0, 0x5a]),
            Input::Of(&[0x100, 0x1f0, 0x10]),
            Input::Of(&[8]),
        ])
        .setup(with(|e, case| {
            high_pages(e);
            vec![case.word(0), case.word(1), case.word(2)]
        }))
        .run()
        .assert_same();
}
