//! `Havok SDK/hkpconstraintdrawer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkColor::rgbFromFloats` (`00cabb30`, cdecl, four floats; returns the colour).
const RGB_FROM_FLOATS: u32 = 0x00ca_bb30;
/// The primitive drawer's two calls (`ECX` = the drawer at `this + 0xc0`): `00e10bf0`
/// (two points, a scale, a colour and two pass-through words; no name known) and
/// `hkpPrimitiveDrawer::displayArrow` (`00e10d50`: a transform, the two ends, a colour, a
/// scale and two pass-through words).
const DRAW_POINT_PAIR: u32 = 0x00e1_0bf0;
const DISPLAY_ARROW: u32 = 0x00e1_0d50;
/// The float at `01016248` (the green part of the second arrow colour) and the double at
/// `01018a90` (0.2, the arrow length factor).
const ORANGE_GREEN: u32 = 0x0101_6248;
const ARROW_FACTOR: u32 = 0x0101_8a90;

/// Copies the four words at `from` to `to` (the `MOVAPS` copies; the bits are not looked at).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00de44c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintDrawer::drawPivots` (map name): two calls of `00e10bf0` on the drawer at
/// `this + 0xc0`, for the points `this + 0x40` / `this + 0x60` in red (`0xffff0000`) and
/// `this + 0x50` / `this + 0x90` in white (`0xffffffff`), each with the scale
/// `[this + 0x1c] * [this + 8]` (x87 product, float) and the two pass-through words.
pub fn draw_pivots(e: &mut Engine, this: Ptr, first_word: u32, second_word: u32) {
    let at = this.addr();
    for (from, to, colour) in [(0x40, 0x60, 0xffff_0000u32), (0x50, 0x90, 0xffff_ffffu32)] {
        let product = x87_mul(
            x87_load32(e.mem.u32(at + 0x1c)),
            x87_load32(e.mem.u32(at + 8)),
        );
        let scale = x87_quiet_bits(x87_store32(product));
        e.call(
            DRAW_POINT_PAIR,
            &args![
                at + 0xc0,
                at + from,
                at + to,
                scale,
                colour,
                first_word,
                second_word
            ],
        );
    }
}

// Translated from 00de4530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintDrawer::drawBodyFrames` (map name): builds three colours with
/// `hkColor::rgbFromFloats` (1, 1, 0, 1), (1, the float at `01016248`, 0, 1) and (1, 0, 1, 1),
/// copies the six vectors `this + 0x60` to `this + 0xb0` (the two frames' axes) and draws six
/// arrows with
/// `displayArrow` (`00e10d50` on the drawer at `this + 0xc0`): around the transform
/// `this + 0x20` the arrows `0x60 -> 0x70` (`0xffff0000`), `0x70 -> 0x80` (`0xff008000`) and
/// `0x80 -> 0x60` (`0xff0000ff`); around `this + 0x30` the arrows `0x90 -> 0xa0`,
/// `0xa0 -> 0xb0` and `0xb0 -> 0x90` in the three colours. Each arrow's scale is
/// `[this + 0x1c] * [this + 0xc]` (float) times the double at `01018a90` (0.2), stored as a
/// float.
pub fn draw_body_frames(e: &mut Engine, this: Ptr, first_word: u32, second_word: u32) {
    let at = this.addr();
    let one = 0x3f80_0000u32;
    let yellow = e.call(RGB_FROM_FLOATS, &args![one, one, 0u32, one]).u32();
    let orange_green = x87_quiet_bits(e.mem.u32(ORANGE_GREEN));
    let orange = e
        .call(RGB_FROM_FLOATS, &args![one, orange_green, 0u32, one])
        .u32();
    let magenta = e.call(RGB_FROM_FLOATS, &args![one, 0u32, one, one]).u32();
    e.with_stack(0x60, |e, locals| {
        // The frame vectors in the order the code stores them: 0x60, 0x70, 0x80, 0x90, 0xa0,
        // 0xb0 of the drawer become locals 0, 1, 2, 3, 4, 5.
        let local = |index: u32| locals.addr() + 0x10 * index;
        for index in 0..6 {
            copy_vector(e, at + 0x60 + 0x10 * index, local(index));
        }
        let arrows = [
            (0x20, 0, 1, 0xffff_0000),
            (0x20, 1, 2, 0xff00_8000),
            (0x20, 2, 0, 0xff00_00ff),
            (0x30, 3, 4, yellow),
            (0x30, 4, 5, orange),
            (0x30, 5, 3, magenta),
        ];
        for (transform, from, to, colour) in arrows {
            let product = x87_mul(
                x87_load32(e.mem.u32(at + 0x1c)),
                x87_load32(e.mem.u32(at + 0xc)),
            );
            let stored = x87_store32(product);
            let fifth: f64 = e.global(ARROW_FACTOR);
            let scale = x87_quiet_bits(x87_store32(x87_mul(x87_load32(stored), fifth)));
            e.call(
                DISPLAY_ARROW,
                &args![
                    at + 0xc0,
                    at + transform,
                    local(from),
                    local(to),
                    colour,
                    scale,
                    first_word,
                    second_word
                ],
            );
        }
    });
}

// Translated from 00de47d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float `scale` (through the x87 stack, which quiets a signalling NaN) at
/// `this + 0x1c`, the scale `drawPivots` and `drawBodyFrames` multiply with. The class is
/// the constraint drawer; the name is not known.
pub fn set_scale(e: &mut Engine, this: Ptr, scale: u32) {
    e.mem.set_u32(this.addr() + 0x1c, x87_quiet_bits(scale));
}

// Translated from 00de47e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintDrawer::updateCommonParameters` (map name): copies the 64 bytes at `+0xf0` of
/// the constraint's second body (`[constraint + 0x18]`) to `first_out` and those of the first
/// body (`[constraint + 0x14]`) to `second_out` (four vectors each: a transform), then keeps
/// vectors 0 to 2 of `first_out` at `this + 0x90`, of `second_out` at `this + 0x60`, and
/// vector 3 of `first_out` at `this + 0x30` and of `second_out` at `this + 0x20`.
pub fn update_common_parameters(
    e: &mut Engine,
    this: Ptr,
    constraint: Ptr,
    first_out: Ptr,
    second_out: Ptr,
) {
    let at = this.addr();
    let second_body = e.mem.u32(constraint.addr() + 0x18);
    let first_body = e.mem.u32(constraint.addr() + 0x14);
    for index in 0..4 {
        copy_vector(
            e,
            second_body + 0xf0 + 0x10 * index,
            first_out.addr() + 0x10 * index,
        );
    }
    for index in 0..4 {
        copy_vector(
            e,
            first_body + 0xf0 + 0x10 * index,
            second_out.addr() + 0x10 * index,
        );
    }
    for index in 0..3 {
        copy_vector(e, first_out.addr() + 0x10 * index, at + 0x90 + 0x10 * index);
    }
    for index in 0..3 {
        copy_vector(
            e,
            second_out.addr() + 0x10 * index,
            at + 0x60 + 0x10 * index,
        );
    }
    copy_vector(e, first_out.addr() + 0x30, at + 0x30);
    copy_vector(e, second_out.addr() + 0x30, at + 0x20);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_44c0, draw_pivots(Ptr, u32, u32)),
        entry!(0x00de_4530, draw_body_frames(Ptr, u32, u32)),
        entry!(0x00de_47d0, set_scale(Ptr, u32)),
        entry!(0x00de_47e0, update_common_parameters(Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range mapped and the drawer's float and double constants.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(ORANGE_GREEN, 0.5f32.to_bits());
        e.mem.set_f64(ARROW_FACTOR, 0.2);
        e
    }

    /// A drawer with the scale `4.0` at `+0x1c` and `0.5` / `0.25` at `+8` / `+0xc`, and word
    /// `0x100 + n` at the vectors from `+0x20` on.
    fn drawer(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x100);
        for word in 8..0x30 {
            e.mem.set_u32(this + 4 * word, 0x100 + word);
        }
        e.mem.set_f32(this + 0x1c, 4.0);
        e.mem.set_f32(this + 8, 0.5);
        e.mem.set_f32(this + 0xc, 0.25);
        this
    }

    #[test]
    fn draw_pivots_draws_two_point_pairs_scaled_by_the_product() {
        let mut e = rig();
        let this = drawer(&mut e);
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(DRAW_POINT_PAIR, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00de_44c0, &args![this, 0xaaaau32, 0xbbbbu32]);
        let scale = 2.0f32.to_bits();
        assert_eq!(
            *log.borrow(),
            [
                vec![
                    this + 0xc0,
                    this + 0x40,
                    this + 0x60,
                    scale,
                    0xffff_0000,
                    0xaaaa,
                    0xbbbb
                ],
                vec![
                    this + 0xc0,
                    this + 0x50,
                    this + 0x90,
                    scale,
                    0xffff_ffff,
                    0xaaaa,
                    0xbbbb
                ],
            ]
        );
    }

    #[test]
    fn draw_pivots_quiets_a_signalling_scale() {
        let mut e = rig();
        let this = drawer(&mut e);
        e.mem.set_u32(this + 0x1c, 0x7f80_0001);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(DRAW_POINT_PAIR, move |_, a| {
            sink.borrow_mut().push(a[3]);
            Ret::default()
        });
        e.call(0x00de_44c0, &args![this, 0u32, 0u32]);
        assert_eq!(*seen.borrow(), [0x7fc0_0001, 0x7fc0_0001]);
    }

    #[test]
    fn draw_body_frames_draws_six_arrows_between_copies_of_the_axes() {
        let mut e = rig();
        let this = drawer(&mut e);
        let colours = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&colours);
        e.register_double(RGB_FROM_FLOATS, move |_, a| {
            let mut calls = sink.borrow_mut();
            calls.push(a.to_vec());
            Ret {
                eax: 0xc000_0000 + calls.len() as u32,
                ..Ret::default()
            }
        });
        let arrows = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&arrows);
        e.register_double(DISPLAY_ARROW, move |e, a| {
            // The two ends are copies of the drawer's vectors: their first words tell which.
            sink.borrow_mut().push((
                a[0],
                a[1],
                e.mem.u32(a[2]),
                e.mem.u32(a[3]),
                a[4],
                a[5],
                a[6],
                a[7],
            ));
            Ret::default()
        });
        e.call(0x00de_4530, &args![this, 0xaaaau32, 0xbbbbu32]);
        let one = 1.0f32.to_bits();
        let half = 0.5f32.to_bits();
        assert_eq!(
            *colours.borrow(),
            [
                vec![one, one, 0, one],
                vec![one, half, 0, one],
                vec![one, 0, one, one]
            ]
        );
        // 4 * 0.25 = 1.0, times 0.2 in double precision, rounded to a float.
        let scale = ((1.0f64 * 0.2) as f32).to_bits();
        let word = |offset: u32| 0x100 + offset / 4;
        let drawer_at = this + 0xc0;
        let [yellow, orange, magenta] = [0xc000_0001, 0xc000_0002, 0xc000_0003];
        assert_eq!(
            *arrows.borrow(),
            [
                (
                    drawer_at,
                    this + 0x20,
                    word(0x60),
                    word(0x70),
                    0xffff_0000,
                    scale,
                    0xaaaa,
                    0xbbbb
                ),
                (
                    drawer_at,
                    this + 0x20,
                    word(0x70),
                    word(0x80),
                    0xff00_8000,
                    scale,
                    0xaaaa,
                    0xbbbb
                ),
                (
                    drawer_at,
                    this + 0x20,
                    word(0x80),
                    word(0x60),
                    0xff00_00ff,
                    scale,
                    0xaaaa,
                    0xbbbb
                ),
                (
                    drawer_at,
                    this + 0x30,
                    word(0x90),
                    word(0xa0),
                    yellow,
                    scale,
                    0xaaaa,
                    0xbbbb
                ),
                (
                    drawer_at,
                    this + 0x30,
                    word(0xa0),
                    word(0xb0),
                    orange,
                    scale,
                    0xaaaa,
                    0xbbbb
                ),
                (
                    drawer_at,
                    this + 0x30,
                    word(0xb0),
                    word(0x90),
                    magenta,
                    scale,
                    0xaaaa,
                    0xbbbb
                ),
            ]
        );
    }

    #[test]
    fn set_scale_stores_the_float_and_quiets_a_signalling_nan() {
        let mut e = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00de_47d0, &args![this, 1.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x1c), 1.5);
        e.call(0x00de_47d0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x7fc0_0001);
    }

    #[test]
    fn update_common_parameters_copies_the_body_frames() {
        let mut e = rig();
        let this = e.mem.alloc(0x100);
        let first_body = e.mem.alloc(0x140);
        let second_body = e.mem.alloc(0x140);
        for word in 0..16 {
            e.mem.set_u32(first_body + 0xf0 + 4 * word, 0xa00 + word);
            e.mem.set_u32(second_body + 0xf0 + 4 * word, 0xb00 + word);
        }
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0x14, first_body);
        e.mem.set_u32(constraint + 0x18, second_body);
        let first_out = e.mem.alloc(0x40);
        let second_out = e.mem.alloc(0x40);
        e.call(0x00de_47e0, &args![this, constraint, first_out, second_out]);
        // The second body goes to the first output, the first body to the second one.
        for word in 0..16 {
            assert_eq!(e.mem.u32(first_out + 4 * word), 0xb00 + word);
            assert_eq!(e.mem.u32(second_out + 4 * word), 0xa00 + word);
        }
        for word in 0..12 {
            assert_eq!(e.mem.u32(this + 0x90 + 4 * word), 0xb00 + word);
            assert_eq!(e.mem.u32(this + 0x60 + 4 * word), 0xa00 + word);
        }
        for word in 0..4 {
            assert_eq!(e.mem.u32(this + 0x30 + 4 * word), 0xb0c + word);
            assert_eq!(e.mem.u32(this + 0x20 + 4 * word), 0xa0c + word);
        }
    }
}
