//! `Havok SDK/run_00de71d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! One more constraint drawing function (the engine map has no name), built like the ones of
//! `run_00de59f0.rs`, `run_00de6550.rs` and `run_00de7730.rs`, whose `pub(crate)` helpers it
//! uses: thiscall on the constraint drawer with four stack words `(constraint, display handler,
//! first word, second word)`, the frame addressed as the exe addresses it (relative to the stack
//! pointer after the three saved registers), SSE vector arithmetic in `f32`, x87 products in
//! `f64` rounded to `float` at each store.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00de59f0::{
    add, broadcast, copied_float, float_product, load, mul, store, DRAW_BODY_FRAMES, DRAW_PIVOTS,
    HALF_DOUBLE, SET_DISPLAY_HANDLER, SET_ROTATED_DIR, SET_TRANSFORMED_POS,
    UPDATE_COMMON_PARAMETERS,
};

/// The doubles `-0.5` (`01011598`), `0.1` (`0101ffa0`, a float widened) and `-0.1` (`0106b170`).
const MINUS_HALF_DOUBLE: u32 = 0x0101_1598;
const TENTH: u32 = 0x0101_ffa0;
const MINUS_TENTH: u32 = 0x0106_b170;

/// The scale `s = [drawer + 0x1c] * [drawer]` rounded to a float and read back (`FSTP`/`FLD`):
/// the x87 value the drawing steps multiply.
fn drawing_scale(e: &Engine, me: u32) -> f64 {
    let product = float_product(e, me + 0x1c, me);
    x87_load32(x87_store32(product))
}

/// The two points of a line along `A` (frame `+0x40`) through the point `centre` (a vector):
/// `u * A + centre` and `v * A + centre` for the floats `u` and `v`, as the exe forms them.
fn along(e: &Engine, f: u32, scalar: u32, centre: [f32; 4]) -> [f32; 4] {
    add(
        mul(broadcast(f32::from_bits(scalar)), load(e, f + 0x40)),
        centre,
    )
}

// Translated from 00de71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a pair of limit marks (the engine map has no name), thiscall with
/// four words. After the common set-up (blocks at frame `+0x70` / `+0xb0`) it draws the body
/// frames, the pivots and the red line, rotates `data + 0x60` by the first frame (`drawer +
/// 0x90`) into frame `+0x60`, and with `s = [drawer + 0x1c] * [drawer]` (rounded to a float)
/// draws the line from `-0.5 s V + P` to `0.5 s V + P` (`V` frame `+0x60`, `P = [drawer + 0x50]`)
/// in `0xff008000`. Then, with `W` = `data + 0x70` rotated into frame `+0x40` and the floats
/// `c2 = [data + 0xa8]`, `c1 = [data + 0xa4]` (through the x87 stack), two lines in
/// `0xffffffff`, one for each `c`: from `m * W + X` to `n * W + X` with `X = c * V + P`, `m =
/// 0.1 s` and `n = -0.1 s` (rounded to floats). Then `data + 0x110` rotated by the first frame
/// gives a line in `0xffffff00` from `-0.5 s W + P` to `0.5 s W + P`, and `data + 0xd0` rotated
/// by the second frame (`drawer + 0x60`) a line in `0xff0000ff` the same way around the
/// point `[drawer + 0x40]`.
pub fn fn_00de71d0(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    first_word: u32,
    second_word: u32,
) {
    let me = this.addr();
    let data = e.mem.u32(constraint.wrapping_add(0xc));
    e.call(
        SET_DISPLAY_HANDLER,
        &args![me.wrapping_add(0xc0), display_handler],
    );
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![me, constraint, f + 0x70, f + 0xb0],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x50, f + 0x70, data.wrapping_add(0x90)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x40, f + 0xb0, data.wrapping_add(0x50)],
        );
        e.call(DRAW_BODY_FRAMES, &args![me, first_word, second_word]);
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        e.vcall(
            display_handler,
            0x24,
            &args![
                me + 0x40,
                me + 0x50,
                0xffff_0000u32,
                first_word,
                second_word
            ],
        );
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x60, me + 0x90, data.wrapping_add(0x60)],
        );
        // The line along V through P.
        let scale = drawing_scale(e, me);
        let direction = load(e, f + 0x60);
        let point = load(e, me + 0x50);
        let minus_half = x87_store32(x87_mul(e.mem.f64(MINUS_HALF_DOUBLE), scale));
        let half = x87_store32(x87_mul(scale, e.mem.f64(HALF_DOUBLE)));
        store(
            e,
            f + 0x30,
            add(mul(broadcast(f32::from_bits(minus_half)), direction), point),
        );
        store(
            e,
            f + 0x40,
            add(mul(broadcast(f32::from_bits(half)), direction), point),
        );
        e.vcall(
            display_handler,
            0x24,
            &args![f + 0x30, f + 0x40, 0xff00_8000u32, first_word, second_word],
        );
        // The pairs of marks along W.
        let second_mark = copied_float(e, data.wrapping_add(0xa4));
        let first_mark = copied_float(e, data.wrapping_add(0xa8));
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x40, me + 0x90, data.wrapping_add(0x70)],
        );
        for mark in [first_mark, second_mark] {
            let scale = drawing_scale(e, me);
            let direction = load(e, f + 0x60);
            let across = load(e, f + 0x40);
            let point = load(e, me + 0x50);
            let long = x87_store32(x87_mul(e.mem.f64(TENTH), scale));
            let short = x87_store32(x87_mul(scale, e.mem.f64(MINUS_TENTH)));
            let centre = add(mul(broadcast(f32::from_bits(mark)), direction), point);
            store(
                e,
                f + 0x30,
                add(mul(broadcast(f32::from_bits(long)), across), centre),
            );
            store(
                e,
                f + 0x50,
                add(mul(broadcast(f32::from_bits(short)), across), centre),
            );
            e.vcall(
                display_handler,
                0x24,
                &args![f + 0x30, f + 0x50, 0xffff_ffffu32, first_word, second_word],
            );
        }
        // The two lines of the other vectors.
        for (vector, rotation, colour, anchor) in [
            (0x110u32, me + 0x90, 0xffff_ff00u32, me + 0x50),
            (0xd0, me + 0x60, 0xff00_00ff, me + 0x40),
        ] {
            e.call(
                SET_ROTATED_DIR,
                &args![f + 0x40, rotation, data.wrapping_add(vector)],
            );
            let scale = drawing_scale(e, me);
            let anchor_point = load(e, anchor);
            let minus_half = x87_store32(x87_mul(e.mem.f64(MINUS_HALF_DOUBLE), scale));
            let half = x87_store32(x87_mul(scale, e.mem.f64(HALF_DOUBLE)));
            store(e, f + 0x50, along(e, f, minus_half, anchor_point));
            store(e, f + 0x30, along(e, f, half, anchor_point));
            e.vcall(
                display_handler,
                0x24,
                &args![f + 0x50, f + 0x30, colour, first_word, second_word],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00de_71d0, fn_00de71d0(Ptr, u32, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00de6550::tests::{
        bits, calls, rig, run, set_block, LINE_SLOT,
    };

    /// `scalar * vector + base`, lane by lane in `f32`.
    fn axpy(scalar: f32, vector: [f32; 4], base: [f32; 4]) -> [u32; 4] {
        bits(std::array::from_fn(|lane| {
            scalar * vector[lane] + base[lane]
        }))
    }

    #[test]
    fn the_mark_drawer_draws_the_axis_line_the_marks_and_two_frame_lines() {
        let mut r = rig();
        let (d, c, data) = (r.drawer, r.constraint, r.data);
        r.e.mem.set_f64(HALF_DOUBLE, 0.5);
        r.e.mem.set_f64(TENTH, f64::from(0.1f32));
        r.e.mem.set_f64(MINUS_TENTH, f64::from(-0.1f32));
        set_block(&mut r.e, data + 0x60, [1.0, 0.0, 0.0, 0.0]);
        set_block(&mut r.e, data + 0x70, [0.0, 1.0, 0.0, 0.0]);
        set_block(&mut r.e, data + 0x110, [0.0, 0.0, 1.0, 0.0]);
        set_block(&mut r.e, data + 0xd0, [1.0, 1.0, 0.0, 0.0]);
        r.e.mem.set_f32(data + 0xa8, 2.0);
        r.e.mem.set_f32(data + 0xa4, -1.0);
        run(&mut r, 0x00de_71d0);
        let frame = calls(&r, UPDATE_COMMON_PARAMETERS)[0].1[2] - 0x70;
        let log = r.log.borrow().clone();
        let order: Vec<(u32, Vec<u32>)> = log.iter().map(|c| (c.0, c.1.clone())).collect();
        assert_eq!(
            order[..8],
            [
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0x70, frame + 0xb0]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x50, frame + 0x70, data + 0x90]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x40, frame + 0xb0, data + 0x50]
                ),
                (DRAW_BODY_FRAMES, vec![d, 0x111, 0x222]),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
                (
                    LINE_SLOT,
                    vec![r.handler, d + 0x40, d + 0x50, 0xffff_0000, 0x111, 0x222]
                ),
                (SET_ROTATED_DIR, vec![frame + 0x60, d + 0x90, data + 0x60]),
            ]
        );
        drop(log);
        let rotations: Vec<Vec<u32>> = calls(&r, SET_ROTATED_DIR)
            .iter()
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(rotations.len(), 4);
        assert_eq!(rotations[1], vec![frame + 0x40, d + 0x90, data + 0x70]);
        assert_eq!(rotations[2], vec![frame + 0x40, d + 0x90, data + 0x110]);
        assert_eq!(rotations[3], vec![frame + 0x40, d + 0x60, data + 0xd0]);
        let lines = calls(&r, LINE_SLOT);
        assert_eq!(lines.len(), 6);
        let point = [1.0f32, 2.0, 3.0, 0.0];
        let anchor = [10.0f32, 20.0, 30.0, 0.0];
        let v = [1.0f32, 0.0, 0.0, 0.0];
        let w = [0.0f32, 1.0, 0.0, 0.0];
        // s = 6: the axis line runs from -3 V + P to 3 V + P
        assert_eq!(
            lines[1].1[1..],
            [frame + 0x30, frame + 0x40, 0xff00_8000, 0x111, 0x222]
        );
        assert_eq!(lines[1].2[0], axpy(-3.0, v, point));
        assert_eq!(lines[1].2[1], axpy(3.0, v, point));
        // the marks: X = c * V + P for c = 2 then -1; m = float(0.1 * 6), n = float(6 * -0.1)
        let m = (f64::from(0.1f32) * 6.0) as f32;
        let n = (6.0 * f64::from(-0.1f32)) as f32;
        for (line, mark) in [(&lines[2], 2.0f32), (&lines[3], -1.0)] {
            let x: [f32; 4] = std::array::from_fn(|lane| mark * v[lane] + point[lane]);
            assert_eq!(
                line.1[1..],
                [frame + 0x30, frame + 0x50, 0xffff_ffff, 0x111, 0x222]
            );
            assert_eq!(line.2[0], axpy(m, w, x));
            assert_eq!(line.2[1], axpy(n, w, x));
        }
        // the frame lines: -3 and +3 times the rotated vector around P and around Q
        let z = [0.0f32, 0.0, 1.0, 0.0];
        assert_eq!(
            lines[4].1[1..],
            [frame + 0x50, frame + 0x30, 0xffff_ff00, 0x111, 0x222]
        );
        assert_eq!(lines[4].2[0], axpy(-3.0, z, point));
        assert_eq!(lines[4].2[1], axpy(3.0, z, point));
        let diagonal = [1.0f32, 1.0, 0.0, 0.0];
        assert_eq!(
            lines[5].1[1..],
            [frame + 0x50, frame + 0x30, 0xff00_00ff, 0x111, 0x222]
        );
        assert_eq!(lines[5].2[0], axpy(-3.0, diagonal, anchor));
        assert_eq!(lines[5].2[1], axpy(3.0, diagonal, anchor));
    }
}
