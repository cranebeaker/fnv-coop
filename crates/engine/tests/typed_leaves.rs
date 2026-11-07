//! The differential test of every typed leaf (`engine_core::typed::leaf`) against its translation: here
//! because the translations live in the crates above the core, and this crate has them all. Real objects
//! in guest memory, every branch, and the stops the exe's reads would give.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Diff, Input, Value};

/// A block of `size` bytes with `value` at `offset`.
fn object_with(e: &mut Engine, size: u32, offset: u32, value: u32) -> u32 {
    let p = e.mem.alloc(size);
    e.mem.set_u32(p + offset, value);
    p
}

/// Words a field getter should hand back whole: top bits, low bits, and what looks like a pointer.
const FIELD_VALUES: &[u32] = &[
    0,
    1,
    0x80,
    0xff,
    0x100,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_ffff,
    0x0123_4567,
];

/// Flag words around each bit a form flag test reads.
const FLAG_WORDS: &[u32] = &[
    0,
    0x20,
    0x800,
    0x0020_0000,
    !0x20,
    !0x800,
    !0x0020_0000,
    0x10,
    0x40,
    0x400,
    0x1000,
    0x0010_0000,
    0x0040_0000,
    u32::MAX,
];

#[test]
fn word_at_0() {
    Diff::new(entry!(0x0055_9450, leaf::word_at_0(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 8, 0, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_4() {
    Diff::new(entry!(0x0072_6070, leaf::word_at_4(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 8, 4, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_8() {
    Diff::new(entry!(0x0044_ddc0, leaf::word_at_8(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 12, 8, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_c() {
    Diff::new(entry!(0x0084_e3a0, leaf::word_at_c(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 16, 0xc, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_20() {
    Diff::new(entry!(0x007a_f430, leaf::word_at_20(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 0x24, 0x20, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_28() {
    Diff::new(entry!(0x0045_cd60, leaf::word_at_28(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 0x2c, 0x28, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_30() {
    Diff::new(entry!(0x0067_1d10, leaf::word_at_30(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 0x34, 0x30, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn word_at_68() {
    Diff::new(entry!(0x008d_8520, leaf::word_at_68(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 0x6c, 0x68, case.word(0))])
        .run()
        .assert_same();
}

/// A field getter on a null or dangling `this` stops, at the same address.
#[test]
fn a_getter_on_a_bad_pointer_stops_alike() {
    Diff::new(entry!(0x0044_ddc0, leaf::word_at_8(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(&[0, 0x10, 0xfff0_0000])])
        .expect_stops(|_| true)
        .run()
        .assert_same();
}

#[test]
fn byte_at_87() {
    Diff::new(entry!(0x0043_7bb0, leaf::byte_at_87(Ptr) -> u8))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(&[0, 1, 0x7f, 0x80, 0xff])])
        .setup(|e, case| {
            let p = e.mem.alloc(0x90);
            // The bytes either side differ, so a read of a word or of its neighbour shows.
            e.mem.set_u32(p + 0x84, 0xa5a5_a5a5);
            e.mem.set_u32(p + 0x88, 0x5a5a_5a5a);
            e.mem.set_u8(p + 0x87, case.word(0) as u8);
            vec![p]
        })
        .run()
        .assert_same();
}

#[test]
fn same_word() {
    Diff::new(entry!(0x0068_15c0, leaf::same_word(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn nothing() {
    Diff::new(entry!(0x0040_fbe0, leaf::nothing()))
        .family(&leaf::LEAVES)
        .run()
        .assert_same();
}

/// Its callers push words of every shape, and it reads none.
#[test]
fn nothing_with_words_pushed() {
    Diff::new(entry!(0x0040_fbe0, leaf::nothing()))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Word, Input::Of(&[0, 0xffff_ffff])])
        .run()
        .assert_same();
}

#[test]
fn zero_of_this() {
    Diff::new(entry!(0x00ac_bb70, leaf::zero_of_this(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn zero() {
    Diff::new(entry!(0x00e7_cd40, leaf::zero(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

/// The words go through `FLD`: a signalling NaN comes out quiet, everything else as it went in.
#[test]
fn point3_new() {
    Diff::new(entry!(0x0041_6870, leaf::point3_new(Ptr, f32, f32, f32) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::F32, Input::F32, Input::F32])
        .max_cases(120)
        .float_sweep(64)
        .setup(|e, case| {
            let p = e.mem.alloc(16);
            // Past the point: not written.
            e.mem.set_u32(p + 12, 0x1234_5678);
            let mut words = vec![p];
            words.extend(case.words());
            words
        })
        .run()
        .assert_same();
}

#[test]
fn world_bound() {
    Diff::new(entry!(0x0043_d450, leaf::world_bound(Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES)])
        .setup(|e, case| vec![object_with(e, 0x24, 0x20, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn array_element() {
    Diff::new(entry!(0x0087_7a30, leaf::array_element(Ptr, u32) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES), Input::Word])
        .setup(|e, case| vec![object_with(e, 8, 4, case.word(0)), case.word(1)])
        .run()
        .assert_same();
}

#[test]
fn form_flag_20() {
    Diff::new(entry!(0x0044_0d80, leaf::form_flag_20(Ptr) -> bool))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FLAG_WORDS)])
        .setup(|e, case| vec![object_with(e, 12, 8, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn form_flag_800() {
    Diff::new(entry!(0x0044_0da0, leaf::form_flag_800(Ptr) -> bool))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FLAG_WORDS)])
        .setup(|e, case| vec![object_with(e, 12, 8, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn form_flag_200000() {
    Diff::new(entry!(0x0057_6d30, leaf::form_flag_200000(Ptr) -> bool))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FLAG_WORDS)])
        .setup(|e, case| vec![object_with(e, 12, 8, case.word(0))])
        .run()
        .assert_same();
}

#[test]
fn flags_at_30_any() {
    Diff::new(entry!(0x0045_6630, leaf::flags_at_30_any(Ptr, u32) -> bool))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FLAG_WORDS), Input::Of(FLAG_WORDS)])
        .setup(|e, case| vec![object_with(e, 0x34, 0x30, case.word(0)), case.word(1)])
        .run()
        .assert_same();
}

#[test]
fn both_words_zero() {
    Diff::new(entry!(0x0082_56d0, leaf::both_words_zero(Ptr) -> bool))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(FIELD_VALUES), Input::Of(FIELD_VALUES)])
        .setup(|e, case| {
            let p = e.mem.alloc(8);
            e.mem.set_u32(p, case.word(0));
            e.mem.set_u32(p + 4, case.word(1));
            vec![p]
        })
        .run()
        .assert_same();
}

/// The word at +4 is read first: with it on an unmapped page the call stops there, whatever +0 holds.
#[test]
fn both_words_zero_reads_the_second_word_first() {
    Diff::new(entry!(0x0082_56d0, leaf::both_words_zero(Ptr) -> bool))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(&[0, 1])])
        .expect_stops(|_| true)
        .setup(|e, case| {
            e.map(0x0300_0000, 0x1000);
            e.mem.set_u32(0x0300_0ffc, case.word(0));
            vec![0x0300_0ffc]
        })
        .run()
        .assert_same();
}

#[test]
fn store_colour() {
    Diff::new(entry!(
        0x004e_20c0,
        leaf::store_colour(u32, u32, u32, u32, u32)
    ))
    .family(&leaf::LEAVES)
    .inputs(&[
        Input::Of(&[0, 1, 15]),
        Input::Of(FIELD_VALUES),
        Input::Of(&[1, 2]),
        Input::Of(&[3, 4]),
        Input::Of(&[5, 6]),
    ])
    .max_cases(64)
    .setup(|e, case| {
        e.map(0x011f_a000, 0x1000);
        case.words()
    })
    .run()
    .assert_same();
}

/// Six floats from the exe's classes into two points: `case.words()` are their bits, with any NaN kept.
fn two_points(e: &mut Engine, case: &engine_core::typed::diff::Case) -> Vec<u32> {
    let words = case.words();
    let (a, b) = (e.mem.alloc(16), e.mem.alloc(16));
    for i in 0..3u32 {
        e.mem.set_u32(a + i * 4, words[i as usize]);
        e.mem.set_u32(b + i * 4, words[i as usize + 3]);
    }
    vec![a, b]
}

#[test]
fn point3_dot() {
    Diff::new(entry!(0x004b_6190, leaf::point3_dot(Ptr, u32) -> f32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::F32; 6])
        .max_cases(300)
        .float_sweep(64)
        .setup(two_points)
        .run()
        .assert_same();
}

#[test]
fn plane_side() {
    Diff::new(entry!(0x0049_da80, leaf::plane_side(Ptr, Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::F32; 7])
        .max_cases(300)
        .float_sweep(64)
        .setup(|e, case| {
            // The double zero both compares read.
            e.map(0x0101_2000, 0x1000);
            let words = case.words();
            let (plane, point) = (e.mem.alloc(16), e.mem.alloc(16));
            for i in 0..4u32 {
                e.mem.set_u32(plane + i * 4, words[i as usize]);
            }
            for i in 0..3u32 {
                e.mem.set_u32(point + i * 4, words[i as usize + 4]);
            }
            vec![plane, point]
        })
        .run()
        .assert_same();
}

/// On, behind, in front, and exactly on the plane; the generated floats rarely land there.
#[test]
fn plane_side_each_way() {
    const X: &[u32] = &[0x3f80_0000, 0xbf80_0000, 0x0000_0000, 0x8000_0000];
    Diff::new(entry!(0x0049_da80, leaf::plane_side(Ptr, Ptr) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(X), Input::Of(X)])
        .setup(|e, case| {
            e.map(0x0101_2000, 0x1000);
            let (plane, point) = (e.mem.alloc(16), e.mem.alloc(16));
            e.mem.set_u32(plane, 0x3f80_0000);
            e.mem.set_u32(plane + 0xc, case.word(0));
            e.mem.set_u32(point, case.word(1));
            vec![plane, point]
        })
        .run()
        .assert_same();
}

#[test]
fn ctrlfp() {
    Diff::new(entry!(0x00ed_5abb, leaf::ctrlfp(u32, u32) -> i32))
        .family(&leaf::LEAVES)
        .inputs(&[
            Input::Of(&[0, 0x7f, 0x27f, 0xc7f, 0x1f7f, 0x0001_8000, u32::MAX]),
            Input::Of(&[0, 0x300, 0xc00, 0xffff, u32::MAX]),
        ])
        .run()
        .assert_same();
}

/// The old word's top bit set: it comes back sign-extended.
#[test]
fn ctrlfp_sign_extends_the_old_word() {
    Diff::new(entry!(0x00ed_5abb, leaf::ctrlfp(u32, u32) -> i32))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(&[0x7f, 0x27f])])
        .setup(|e, case| {
            // The word in the thread block (`x87_control_word`): marked as set, with bit 15 up, which a
            // load can never leave there.
            let at = e.tls() + 0x0e04;
            e.mem.set_u32(at, 0x1_0000 | 0x9f7f);
            vec![case.word(0), 0xffff]
        })
        .run()
        .assert_same();
}

#[test]
fn load_math_control_word() {
    Diff::new(entry!(0x00ed_2795, leaf::load_math_control_word(u32)))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(&[
            0,
            0x7f,
            0x27f,
            0x37f,
            0x0e3e,
            0x0c7f,
            u32::MAX,
        ])])
        .run()
        .assert_same();
}

#[test]
fn raise_status_flags() {
    Diff::new(entry!(0x00ed_5ae4, leaf::raise_status_flags(u32)))
        .family(&leaf::LEAVES)
        .inputs(&[Input::Of(&[0, 1, 4, 8, 0x10, 0x20, 0x3f, 0x100])])
        .setup(|e, case| {
            e.map(0x011c_2000, 0x1000);
            case.words()
        })
        .run()
        .assert_same();
}

#[test]
fn double_exponent() {
    Diff::new(entry!(0x00ed_2808, leaf::double_exponent(u32, u32) -> u32))
        .family(&leaf::LEAVES)
        .inputs(&[
            Input::Of(&[0, u32::MAX]),
            Input::Of(&[
                0,
                1,
                0x3ff0_0000,
                0x7fef_ffff,
                0x7ff0_0000,
                0x7ff8_0000,
                0x7fff_ffff,
                0xfff0_0001,
                u32::MAX,
            ]),
        ])
        .run()
        .assert_same();
}

/// Every way through: the state's slot below and past 5, the flags, and each of the marks already set.
#[test]
fn set_state_slot() {
    Diff::new(entry!(
        0x00e8_90c0,
        leaf::set_state_slot(Ptr, u32, u32, u32, u32)
    ))
    .family(&leaf::LEAVES)
    .inputs(&[
        // State: slots 0, 3, 4, and 5 and 0xffff, which do nothing.
        Input::Of(&[0, 1, 2, 3, 4]),
        // Enable and the second flag, with a word whose low byte is clear.
        Input::Of(&[0, 1, 0xff, 0x100]),
        Input::Of(&[0, 1, 0x100]),
        // Which of +0x80, +0xa4 and +0xb0 are set already.
        Input::Of(&[0, 1, 2, 3, 4, 5, 6, 7]),
    ])
    .setup(|e, case| {
        e.map(0x0126_f000, 0x1000);
        for (i, slot) in [0u16, 3, 4, 5, 0xffff].iter().enumerate() {
            e.mem.set_u16(0x0126_f92c + i as u32 * 2, *slot);
        }
        let this = e.mem.alloc(0x100);
        for (offset, value) in [(0x64, 10u32), (0x88, 20), (0xac, 30)] {
            e.mem.set_u32(this + offset, value);
        }
        let marks = case.word(3);
        for slot in [0, 3, 4] {
            for (bit, mark) in [(1, 0x80), (2, 0xa4), (4, 0xb0)] {
                e.mem.set_u8(this + mark + slot, u8::from(marks & bit != 0));
            }
        }
        vec![this, case.word(0), 0x1234_5678, case.word(1), case.word(2)]
    })
    .run()
    .assert_same();
}

/// What a typed leaf is compared with really is the translation, and the comparison can fail: a wrong
/// function at a leaf's address differs on the inputs that matter.
#[test]
fn a_wrong_leaf_is_found() {
    fn word_at_8_as_word_at_4(e: &mut Engine, this: Ptr) -> u32 {
        e.mem.u32(this.addr() + 4)
    }
    let report = Diff::new(entry!(0x0044_ddc0, word_at_8_as_word_at_4(Ptr) -> u32))
        .inputs(&[Input::Of(&[7])])
        .setup(|e, case| {
            let p = e.mem.alloc(12);
            e.mem.set_u32(p + 8, case.word(0));
            vec![p]
        })
        .run();
    assert!(!report.is_same());
    assert!(matches!(report.first[0].case.0[0], Value::Word(7)));
}
