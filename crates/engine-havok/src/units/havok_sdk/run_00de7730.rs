//! `Havok SDK/run_00de7730` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three more constraint drawing functions (the engine map has no names), thiscall on the
//! constraint drawer with four stack words `(constraint, display handler, first word, second
//! word)`, built like the ones of `run_00de59f0.rs` and `run_00de6550.rs` (whose `pub(crate)`
//! helpers they use): the frame is addressed as the exe addresses it, relative to the stack
//! pointer after the three saved registers, the vector arithmetic is SSE in `f32`, the x87
//! products are `f64` rounded to `float` at each store. All three start with the set-up of
//! the other drawers: `setDisplayHandler`, `updateCommonParameters`, the drawer's points `+0x50`
//! and `+0x40` transformed from the constraint data's `+0x90` and `+0x50`, the pivots and the
//! body frames, and draw the line `drawer + 0x40` to `drawer + 0x50` in `0xffff0000`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00de59f0::{
    add, broadcast, copied_float, float_product, load, mul, new_stack_array, pushed_float,
    release_pointers, reserve_pointers, squared_length, store, Normaliser, DRAW_BODY_FRAMES,
    DRAW_PIVOTS, HALF, HALF_DOUBLE, MINUS_THREE_QUARTERS, RGB_FROM_FLOATS, SEMICIRCLE_SET,
    SET_DISPLAY_HANDLER, SET_ROTATED_DIR, SET_TRANSFORMED_POS, THREE_QUARTERS,
    UPDATE_COMMON_PARAMETERS,
};

/// The double `-0.5` (`01011598`), the double (about `3.40282e38`) `00de7730` compares the
/// float at `data + 0xcc` with (`0104e138`) and the double `00de7a70` compares an angle span
/// with (`0106ab98`).
const MINUS_HALF_DOUBLE: u32 = 0x0101_1598;
const UNSET_LIMIT: u32 = 0x0104_e138;
const SPAN_LIMIT: u32 = 0x0106_ab98;

/// `rgbFromFloats(0.0, 0.5, 1.0, 1.0)`, the colour of the extent lines: the zero and the ones
/// come from `FLDZ` / `FLD1`, the half from the float at `01016248`.
fn extent_colour(e: &mut Engine) -> u32 {
    let half = copied_float(e, HALF);
    e.call(
        RGB_FROM_FLOATS,
        &args![0u32, half, 0x3f80_0000u32, 0x3f80_0000u32],
    )
    .u32()
}

/// The display handler's slot `0x24` (a line, `(from, to, colour, first word, second
/// word)`) from the vtable pointer `vtable` read earlier.
fn draw_line(
    e: &mut Engine,
    vtable: u32,
    handler: u32,
    from: u32,
    to: u32,
    colour: u32,
    words: (u32, u32),
) {
    let slot = e.mem.u32(vtable.wrapping_add(0x24));
    e.call(slot, &args![handler, from, to, colour, words.0, words.1]);
}

// Translated from 00de7730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a limited range along one axis (the engine map has no name),
/// thiscall with four words. After the common set-up (blocks at frame `+0x60` / `+0xa0`) it
/// rotates `data + 0x60` into frame `+0x40` and `data + 0x70` into `+0x50` by the first frame
/// (`drawer + 0x90`), draws the red line and then, with `x = [data + 0xcc]` and `y = [data +
/// 0xc8]` (floats through the x87 stack): when `x` is not equal to the double at `0104e138`
/// (a NaN is not equal) the line from `y * A + P` to `x * A + P`, with `A` the vector at
/// frame `+0x40` and `P = [drawer + 0x50]`; when it is equal the line from `-s * A + P` to
/// `s * A + P` with `s = [drawer + 0x1c] * [drawer]` rounded to a float; both in the colour of
/// `rgbFromFloats(0, 0.5, 1, 1)`. Then two lines along `B` (frame `+0x50`) in `0xffffffff`: from
/// `X + h * B` to `X + nh * B` with `h = 0.5 * s` and `nh = -0.5 * s` (each rounded to a
/// float), once for `X = y * A + P` and once for `X = x * A + P`.
pub fn fn_00de7730(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    first_word: u32,
    second_word: u32,
) {
    let me = this.addr();
    let data = e.mem.u32(constraint.wrapping_add(0xc));
    let words = (first_word, second_word);
    e.call(
        SET_DISPLAY_HANDLER,
        &args![me.wrapping_add(0xc0), display_handler],
    );
    e.with_stack(0x120, |e, frame| {
        let f = frame.addr();
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![me, constraint, f + 0x60, f + 0xa0],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x50, f + 0x60, data.wrapping_add(0x90)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x40, f + 0xa0, data.wrapping_add(0x50)],
        );
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        e.call(DRAW_BODY_FRAMES, &args![me, first_word, second_word]);
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x40, me + 0x90, data.wrapping_add(0x60)],
        );
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x50, me + 0x90, data.wrapping_add(0x70)],
        );
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
        let limit = x87_load32(e.mem.u32(data.wrapping_add(0xcc)));
        let unset = e.mem.f64(UNSET_LIMIT);
        let axis = load(e, f + 0x40);
        let point = load(e, me + 0x50);
        if limit != unset {
            let high = copied_float(e, data.wrapping_add(0xcc));
            let low = copied_float(e, data.wrapping_add(0xc8));
            let first = add(mul(broadcast(f32::from_bits(high)), axis), point);
            store(e, f + 0x20, first);
            let second = add(mul(broadcast(f32::from_bits(low)), axis), point);
            store(e, f + 0x30, second);
            let vtable = e.mem.u32(display_handler);
            let colour = extent_colour(e);
            draw_line(
                e,
                vtable,
                display_handler,
                f + 0x30,
                f + 0x20,
                colour,
                words,
            );
        } else {
            let product = float_product(e, me + 0x1c, me);
            let positive_bits = x87_store32(product);
            let negative_bits = x87_store32(-x87_load32(positive_bits));
            let first = add(mul(broadcast(f32::from_bits(positive_bits)), axis), point);
            store(e, f + 0x20, first);
            let second = add(mul(broadcast(f32::from_bits(negative_bits)), axis), point);
            store(e, f + 0x30, second);
            let vtable = e.mem.u32(display_handler);
            let colour = extent_colour(e);
            draw_line(
                e,
                vtable,
                display_handler,
                f + 0x30,
                f + 0x20,
                colour,
                words,
            );
        }
        // Two lines along B through the points at the two ends of the range.
        for edge in [0xc8u32, 0xcc] {
            let offset = copied_float(e, data.wrapping_add(edge));
            let axis = load(e, f + 0x40);
            let across = load(e, f + 0x50);
            let centre = add(
                mul(broadcast(f32::from_bits(offset)), axis),
                load(e, me + 0x50),
            );
            let product = float_product(e, me + 0x1c, me);
            let product_float = x87_load32(x87_store32(product));
            let half = x87_store32(x87_mul(e.mem.f64(HALF_DOUBLE), product_float));
            let minus_half = x87_store32(x87_mul(product_float, e.mem.f64(MINUS_HALF_DOUBLE)));
            store(
                e,
                f + 0x20,
                add(mul(broadcast(f32::from_bits(half)), across), centre),
            );
            store(
                e,
                f + 0x30,
                add(mul(broadcast(f32::from_bits(minus_half)), across), centre),
            );
            e.vcall(
                display_handler,
                0x24,
                &args![f + 0x20, f + 0x30, 0xffff_ffffu32, first_word, second_word],
            );
        }
    });
}

// Translated from 00de7a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a limited hinge constraint (the engine map has no name), thiscall with four words.
/// After the common set-up (blocks at frame `+0x70` / `+0xb0`) it rotates `data + 0x60` into
/// frame `+0x50` and `data + 0x80` into `+0x60` by the first frame and draws the red line. With
/// `s = [drawer] * [drawer + 0x1c]` (rounded to a float) it draws, in the colour of
/// `rgbFromFloats(0, 0.5, 1, 1)`, the line from `-0.75 s A + P` to `0.75 s A + P` (`A` frame
/// `+0x50`, `P = [drawer + 0x50]`), and, after rotating `data + 0x20` by the second frame
/// (`drawer + 0x60`) into frame `+0x40`, the line between the points `0.75 s B + Q` and `-0.75
/// s B + Q` (`B` the rotated vector, `Q = [drawer + 0x40]`). When the byte at `data + 0xc2` is
/// set and `p - q` (floats `[data + 0xc8]` and `[data + 0xc4]`, subtracted on the x87 stack)
/// is below the double at `0106ab98` the half circle at `drawer + 0xd0` is set (`00de4950`:
/// radius `[+4] * [+0x1c]`, start angle `q`, end angle `p`, sides `[+0x18]`, centre `P`, axis
/// frame `+0x50`, start direction frame `+0x60`) and drawn through slot `0x38` in white (one
/// local array of one pointer). Then the line from `P` along `0.5 s` of frame `+0x60` in
/// `0xffffff00`, and, after rotating `data + 0x40` by the second frame, the line from `Q` along
/// `0.75 s` times that vector made a unit vector (the host's `RSQRTSS`) in `0xffffff00`.
pub fn fn_00de7a70(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    first_word: u32,
    second_word: u32,
) {
    let me = this.addr();
    let data = e.mem.u32(constraint.wrapping_add(0xc));
    let words = (first_word, second_word);
    e.call(
        SET_DISPLAY_HANDLER,
        &args![me.wrapping_add(0xc0), display_handler],
    );
    e.with_stack(0x120, |e, frame| {
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
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x50, me + 0x90, data.wrapping_add(0x60)],
        );
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x60, me + 0x90, data.wrapping_add(0x80)],
        );
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
        // The line along the first rotated vector.
        let product = float_product(e, me, me + 0x1c);
        let product_float = x87_load32(x87_store32(product));
        let axis = load(e, f + 0x50);
        let point = load(e, me + 0x50);
        let plus = x87_store32(x87_mul(e.mem.f64(THREE_QUARTERS), product_float));
        let minus = x87_store32(x87_mul(product_float, e.mem.f64(MINUS_THREE_QUARTERS)));
        store(
            e,
            f + 0x40,
            add(mul(broadcast(f32::from_bits(plus)), axis), point),
        );
        store(
            e,
            f + 0x30,
            add(mul(broadcast(f32::from_bits(minus)), axis), point),
        );
        let vtable = e.mem.u32(display_handler);
        let colour = extent_colour(e);
        draw_line(
            e,
            vtable,
            display_handler,
            f + 0x30,
            f + 0x40,
            colour,
            words,
        );
        // The line along the second rotated vector, through the point `drawer + 0x40`.
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x40, me + 0x60, data.wrapping_add(0x20)],
        );
        let product = float_product(e, me, me + 0x1c);
        let product_float = x87_load32(x87_store32(product));
        let axis = load(e, f + 0x40);
        let anchor = load(e, me + 0x40);
        let vtable = e.mem.u32(display_handler);
        let plus = x87_store32(x87_mul(e.mem.f64(THREE_QUARTERS), product_float));
        let minus = x87_store32(x87_mul(product_float, e.mem.f64(MINUS_THREE_QUARTERS)));
        store(
            e,
            f + 0x30,
            add(mul(broadcast(f32::from_bits(plus)), axis), anchor),
        );
        store(
            e,
            f + 0x20,
            add(mul(broadcast(f32::from_bits(minus)), axis), anchor),
        );
        let colour = extent_colour(e);
        draw_line(
            e,
            vtable,
            display_handler,
            f + 0x20,
            f + 0x30,
            colour,
            words,
        );
        // The half circle of the range.
        if e.mem.u8(data.wrapping_add(0xc2)) != 0 {
            let end_angle = copied_float(e, data.wrapping_add(0xc8));
            let start_angle = copied_float(e, data.wrapping_add(0xc4));
            let span = x87_sub(x87_load32(end_angle), x87_load32(start_angle));
            if span < e.mem.f64(SPAN_LIMIT) {
                let sides = e.mem.u32(me + 0x18);
                let radius = pushed_float(float_product(e, me + 4, me + 0x1c));
                e.call(
                    SEMICIRCLE_SET,
                    &args![
                        me + 0xd0,
                        radius,
                        start_angle,
                        end_angle,
                        sides,
                        me + 0x50,
                        f + 0x50,
                        f + 0x60
                    ],
                );
                new_stack_array(e, f + 0x20);
                reserve_pointers(e, f + 0x20, 1);
                let array_data = e.mem.u32(f + 0x20);
                e.mem.set_u32(f + 0x24, 1);
                e.mem.set_u32(array_data, me + 0xd0);
                e.vcall(
                    display_handler,
                    0x38,
                    &args![f + 0x20, 0xffff_ffffu32, first_word, second_word],
                );
                release_pointers(e, f + 0x20);
            }
        }
        // The line from the point along the third rotated vector.
        let product = float_product(e, me, me + 0x1c);
        let product_float = x87_load32(x87_store32(product));
        let point = load(e, me + 0x50);
        let vtable = e.mem.u32(display_handler);
        let half = x87_store32(x87_mul(product_float, e.mem.f64(HALF_DOUBLE)));
        store(e, f + 0x20, point);
        store(e, f + 0x30, point);
        let third = load(e, f + 0x60);
        let end = add(mul(broadcast(f32::from_bits(half)), third), point);
        store(e, f + 0x20, end);
        draw_line(
            e,
            vtable,
            display_handler,
            f + 0x30,
            f + 0x20,
            0xffff_ff00,
            words,
        );
        // The line along the unit vector of `data + 0x40` rotated by the second frame.
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x40, me + 0x60, data.wrapping_add(0x40)],
        );
        let normaliser = Normaliser::read(e);
        let rotated = load(e, f + 0x40);
        let product = float_product(e, me, me + 0x1c);
        let product_float = x87_load32(x87_store32(product));
        let length = x87_store32(x87_mul(product_float, e.mem.f64(THREE_QUARTERS)));
        let scale = normaliser.scale(squared_length(rotated));
        let anchor = load(e, me + 0x40);
        let unit = mul(broadcast(scale), rotated);
        store(e, f + 0x40, unit);
        store(e, f + 0x30, anchor);
        store(e, f + 0x20, anchor);
        let end = add(mul(broadcast(f32::from_bits(length)), unit), anchor);
        let vtable = e.mem.u32(display_handler);
        store(e, f + 0x20, end);
        draw_line(
            e,
            vtable,
            display_handler,
            f + 0x30,
            f + 0x20,
            0xffff_ff00,
            words,
        );
    });
}

// Translated from 00de7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a hinge constraint (the engine map has no name), thiscall with four words. After the
/// common set-up (blocks at frame `+0x50` / `+0x90`) it draws the body frames, the pivots and
/// the red line, rotates `data + 0x60` by the first frame into frame `+0x40` and draws, in
/// the colour of `rgbFromFloats(0, 0.5, 1, 1)`, the line from `-0.75 s A + P` to `0.75 s A +
/// P` with `s = [drawer + 0x1c] * [drawer]` (rounded to a float), `A` the rotated vector and
/// `P = [drawer + 0x50]`.
pub fn fn_00de7f20(
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
    e.with_stack(0x120, |e, frame| {
        let f = frame.addr();
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![me, constraint, f + 0x50, f + 0x90],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x50, f + 0x50, data.wrapping_add(0x90)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x40, f + 0x90, data.wrapping_add(0x50)],
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
            &args![f + 0x40, me + 0x90, data.wrapping_add(0x60)],
        );
        let product = float_product(e, me + 0x1c, me);
        let product_float = x87_load32(x87_store32(product));
        let axis = load(e, f + 0x40);
        let point = load(e, me + 0x50);
        let plus = x87_store32(x87_mul(e.mem.f64(THREE_QUARTERS), product_float));
        let minus = x87_store32(x87_mul(product_float, e.mem.f64(MINUS_THREE_QUARTERS)));
        store(
            e,
            f + 0x20,
            add(mul(broadcast(f32::from_bits(plus)), axis), point),
        );
        store(
            e,
            f + 0x30,
            add(mul(broadcast(f32::from_bits(minus)), axis), point),
        );
        let vtable = e.mem.u32(display_handler);
        let colour = extent_colour(e);
        draw_line(
            e,
            vtable,
            display_handler,
            f + 0x30,
            f + 0x20,
            colour,
            (first_word, second_word),
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_7730, fn_00de7730(Ptr, u32, u32, u32, u32)),
        entry!(0x00de_7a70, fn_00de7a70(Ptr, u32, u32, u32, u32)),
        entry!(0x00de_7f20, fn_00de7f20(Ptr, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00de59f0::ARRAY_RESERVE;
    use crate::units::havok_sdk::run_00de6550::tests::{
        bits, calls, rig, run, set_block, watch, Rig, GEOMETRY_SLOT, LINE_SLOT, ROUTER_CELL,
    };

    /// The sentinel float of `00de7730`, `0x7f7fffee`.
    const SENTINEL: f32 = f32::from_bits(0x7f7f_ffee);

    fn constants(r: &mut Rig) {
        r.e.mem.set_f64(THREE_QUARTERS, 0.75);
        r.e.mem.set_f64(MINUS_THREE_QUARTERS, -0.75);
        r.e.mem.set_f64(HALF_DOUBLE, 0.5);
        r.e.mem.set_f64(UNSET_LIMIT, f64::from(SENTINEL));
        r.e.mem.set_f64(SPAN_LIMIT, 100.0);
    }

    /// `scalar * vector + base`, lane by lane in `f32`.
    fn axpy(scalar: f32, vector: [f32; 4], base: [f32; 4]) -> [u32; 4] {
        bits(std::array::from_fn(|lane| {
            scalar * vector[lane] + base[lane]
        }))
    }

    fn frame_of(r: &Rig, offset: u32) -> u32 {
        calls(r, UPDATE_COMMON_PARAMETERS)[0].1[2] - offset
    }

    // ----- 00de7f20 -----

    #[test]
    fn the_hinge_drawer_draws_one_extent_line() {
        let mut r = rig();
        constants(&mut r);
        let (d, c, data) = (r.drawer, r.constraint, r.data);
        set_block(&mut r.e, data + 0x60, [1.0, 2.0, 3.0, 0.0]);
        run(&mut r, 0x00de_7f20);
        let frame = frame_of(&r, 0x50);
        let log = r.log.borrow().clone();
        let order: Vec<(u32, Vec<u32>)> = log.iter().map(|c| (c.0, c.1.clone())).collect();
        assert_eq!(
            order[..9],
            [
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0x50, frame + 0x90]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x50, frame + 0x50, data + 0x90]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x40, frame + 0x90, data + 0x50]
                ),
                (DRAW_BODY_FRAMES, vec![d, 0x111, 0x222]),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
                (
                    LINE_SLOT,
                    vec![r.handler, d + 0x40, d + 0x50, 0xffff_0000, 0x111, 0x222]
                ),
                (SET_ROTATED_DIR, vec![frame + 0x40, d + 0x90, data + 0x60]),
                (
                    RGB_FROM_FLOATS,
                    vec![0, 0.5f32.to_bits(), 0x3f80_0000, 0x3f80_0000]
                ),
            ]
        );
        // s = 6; 0.75 * 6 = 4.5: the line from -4.5 A + P to 4.5 A + P
        let point = [1.0f32, 2.0, 3.0, 0.0];
        let axis = [1.0f32, 2.0, 3.0, 0.0];
        let line = &log[9];
        assert_eq!(line.0, LINE_SLOT);
        assert_eq!(
            line.1,
            vec![
                r.handler,
                frame + 0x30,
                frame + 0x20,
                0x0f10_0000,
                0x111,
                0x222
            ]
        );
        assert_eq!(line.2[0], axpy(-4.5, axis, point));
        assert_eq!(line.2[1], axpy(4.5, axis, point));
        assert_eq!(log.len(), 10);
    }

    // ----- 00de7730 -----

    fn limit_rig(low: f32, high: f32) -> Rig {
        let mut r = rig();
        constants(&mut r);
        let data = r.data;
        set_block(&mut r.e, data + 0x60, [1.0, 0.0, 0.0, 0.0]);
        set_block(&mut r.e, data + 0x70, [0.0, 1.0, 0.0, 0.0]);
        r.e.mem.set_f32(data + 0xc8, low);
        r.e.mem.set_f32(data + 0xcc, high);
        r
    }

    #[test]
    fn the_range_drawer_draws_the_limits_when_the_range_is_set() {
        let mut r = limit_rig(-1.0, 2.0);
        let (d, c, data) = (r.drawer, r.constraint, r.data);
        run(&mut r, 0x00de_7730);
        let frame = frame_of(&r, 0x60);
        let log = r.log.borrow().clone();
        let order: Vec<(u32, Vec<u32>)> = log.iter().map(|c| (c.0, c.1.clone())).collect();
        assert_eq!(
            order[..9],
            [
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0x60, frame + 0xa0]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x50, frame + 0x60, data + 0x90]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x40, frame + 0xa0, data + 0x50]
                ),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
                (DRAW_BODY_FRAMES, vec![d, 0x111, 0x222]),
                (SET_ROTATED_DIR, vec![frame + 0x40, d + 0x90, data + 0x60]),
                (SET_ROTATED_DIR, vec![frame + 0x50, d + 0x90, data + 0x70]),
                (
                    LINE_SLOT,
                    vec![r.handler, d + 0x40, d + 0x50, 0xffff_0000, 0x111, 0x222]
                ),
            ]
        );
        let point = [1.0f32, 2.0, 3.0, 0.0];
        let axis = [1.0f32, 0.0, 0.0, 0.0];
        let across = [0.0f32, 1.0, 0.0, 0.0];
        // the extent line: from y * A + P to x * A + P
        let lines = calls(&r, LINE_SLOT);
        assert_eq!(lines.len(), 4);
        assert_eq!(
            lines[1].1,
            vec![
                r.handler,
                frame + 0x30,
                frame + 0x20,
                0x0f10_0000,
                0x111,
                0x222
            ]
        );
        assert_eq!(lines[1].2[0], axpy(-1.0, axis, point));
        assert_eq!(lines[1].2[1], axpy(2.0, axis, point));
        // s = 6: h = 3, nh = -3; the centres are y * A + P and x * A + P
        for (line, centre_scalar) in [(&lines[2], -1.0f32), (&lines[3], 2.0)] {
            let centre: [f32; 4] =
                std::array::from_fn(|lane| centre_scalar * axis[lane] + point[lane]);
            assert_eq!(line.1[0..2], [r.handler, frame + 0x20]);
            assert_eq!(line.1[2..], [frame + 0x30, 0xffff_ffff, 0x111, 0x222]);
            assert_eq!(line.2[0], axpy(3.0, across, centre));
            assert_eq!(line.2[1], axpy(-3.0, across, centre));
        }
    }

    #[test]
    fn the_range_drawer_uses_the_scale_when_the_range_is_unset() {
        let mut r = limit_rig(-1.0, SENTINEL);
        run(&mut r, 0x00de_7730);
        let frame = frame_of(&r, 0x60);
        let point = [1.0f32, 2.0, 3.0, 0.0];
        let axis = [1.0f32, 0.0, 0.0, 0.0];
        let lines = calls(&r, LINE_SLOT);
        assert_eq!(lines.len(), 4);
        // s = 6 with the sign flipped for the far end
        assert_eq!(lines[1].1[1..3], [frame + 0x30, frame + 0x20]);
        assert_eq!(lines[1].2[0], axpy(-6.0, axis, point));
        assert_eq!(lines[1].2[1], axpy(6.0, axis, point));
    }

    #[test]
    fn a_nan_limit_counts_as_not_equal_to_the_sentinel() {
        let mut r = limit_rig(-1.0, f32::NAN);
        run(&mut r, 0x00de_7730);
        let lines = calls(&r, LINE_SLOT);
        assert_eq!(lines.len(), 4);
        // the first extent line is the one of the set range: its second end is the NaN lanes
        assert!(f32::from_bits(lines[1].2[1][0]).is_nan());
    }

    // ----- 00de7a70 -----

    fn hinge_rig() -> Rig {
        let mut r = rig();
        constants(&mut r);
        let drawer = r.drawer;
        r.e.mem.set_f32(drawer + 4, 13.0);
        watch(&mut r.e, &r.log, SEMICIRCLE_SET, &[5, 6, 7], 0);
        let data = r.data;
        set_block(&mut r.e, data + 0x60, [1.0, 0.0, 0.0, 0.0]);
        set_block(&mut r.e, data + 0x80, [0.0, 0.0, 1.0, 0.0]);
        set_block(&mut r.e, data + 0x20, [0.0, 1.0, 0.0, 0.0]);
        set_block(&mut r.e, data + 0x40, [0.0, 3.0, 4.0, 0.0]);
        r.e.mem.set_f32(data + 0xc8, 3.0);
        r.e.mem.set_f32(data + 0xc4, 1.0);
        r
    }

    #[test]
    fn the_limited_hinge_drawer_draws_five_lines_without_a_range() {
        let mut r = hinge_rig();
        let (d, c, data) = (r.drawer, r.constraint, r.data);
        run(&mut r, 0x00de_7a70);
        let frame = frame_of(&r, 0x70);
        let log = r.log.borrow().clone();
        let order: Vec<(u32, Vec<u32>)> = log.iter().map(|c| (c.0, c.1.clone())).collect();
        assert_eq!(
            order[..9],
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
                (SET_ROTATED_DIR, vec![frame + 0x50, d + 0x90, data + 0x60]),
                (SET_ROTATED_DIR, vec![frame + 0x60, d + 0x90, data + 0x80]),
                (
                    LINE_SLOT,
                    vec![r.handler, d + 0x40, d + 0x50, 0xffff_0000, 0x111, 0x222]
                ),
            ]
        );
        let lines = calls(&r, LINE_SLOT);
        assert_eq!(lines.len(), 5);
        let point = [1.0f32, 2.0, 3.0, 0.0];
        let anchor = [10.0f32, 20.0, 30.0, 0.0];
        // s = 6: the extent line along A = x, from -4.5 A + P to 4.5 A + P
        let axis = [1.0f32, 0.0, 0.0, 0.0];
        assert_eq!(lines[1].1[1..3], [frame + 0x30, frame + 0x40]);
        assert_eq!(lines[1].2[0], axpy(-4.5, axis, point));
        assert_eq!(lines[1].2[1], axpy(4.5, axis, point));
        // the line along B = y through Q, from the point with -4.5 to the one with 4.5
        let across = [0.0f32, 1.0, 0.0, 0.0];
        assert_eq!(lines[2].1[1..3], [frame + 0x20, frame + 0x30]);
        assert_eq!(lines[2].2[0], axpy(-4.5, across, anchor));
        assert_eq!(lines[2].2[1], axpy(4.5, across, anchor));
        // the line from P along 3 (0.5 * 6) of frame +0x60 = z
        let third = [0.0f32, 0.0, 1.0, 0.0];
        assert_eq!(
            lines[3].1[1..],
            [frame + 0x30, frame + 0x20, 0xffff_ff00, 0x111, 0x222]
        );
        assert_eq!(lines[3].2[0], bits(point));
        assert_eq!(lines[3].2[1], axpy(3.0, third, point));
        // the line from Q along 4.5 of the unit vector of (0, 3, 4)
        let squared = 25.0f32;
        let estimate = crate::x87::rsqrt_estimate(squared);
        let scale = (0.5 * estimate) * (3.0 - (squared * estimate) * estimate);
        let unit = [0.0, 3.0 * scale, 4.0 * scale, 0.0];
        assert_eq!(
            lines[4].1[1..],
            [frame + 0x30, frame + 0x20, 0xffff_ff00, 0x111, 0x222]
        );
        assert_eq!(lines[4].2[0], bits(anchor));
        assert_eq!(lines[4].2[1], axpy(4.5, unit, anchor));
        assert!(calls(&r, SEMICIRCLE_SET).is_empty());
    }

    #[test]
    fn a_small_range_adds_the_half_circle() {
        let mut r = hinge_rig();
        let (d, data) = (r.drawer, r.data);
        r.e.mem.set_u8(data + 0xc2, 1);
        run(&mut r, 0x00de_7a70);
        let frame = frame_of(&r, 0x70);
        let circle = calls(&r, SEMICIRCLE_SET);
        assert_eq!(circle.len(), 1);
        // radius [+4] * [+0x1c] = 26, start q = 1, end p = 3, 24 sides
        assert_eq!(
            circle[0].1,
            vec![
                d + 0xd0,
                26.0f32.to_bits(),
                1.0f32.to_bits(),
                3.0f32.to_bits(),
                24,
                d + 0x50,
                frame + 0x50,
                frame + 0x60
            ]
        );
        let shows = calls(&r, GEOMETRY_SLOT);
        assert_eq!(shows.len(), 1);
        assert_eq!(shows[0].1[2..], [0xffff_ffff, 0x111, 0x222]);
        assert_eq!(calls(&r, LINE_SLOT).len(), 5);
    }

    #[test]
    fn a_range_that_is_too_wide_or_not_a_number_adds_nothing() {
        for high in [500.0f32, f32::NAN] {
            let mut r = hinge_rig();
            let data = r.data;
            r.e.mem.set_u8(data + 0xc2, 1);
            r.e.mem.set_f32(data + 0xc8, high);
            run(&mut r, 0x00de_7a70);
            assert!(calls(&r, SEMICIRCLE_SET).is_empty(), "end {high}");
            assert!(calls(&r, GEOMETRY_SLOT).is_empty());
        }
    }

    #[test]
    fn the_half_circle_array_is_reserved_and_freed() {
        let mut r = hinge_rig();
        let data = r.data;
        r.e.mem.set_u8(data + 0xc2, 1);
        // a reserve that gives the array a block and clears its flag bit: it is freed
        r.e.register(ARRAY_RESERVE, |e, words| {
            let block = e.mem.alloc(4 * words[2]);
            e.mem.set_u32(words[1], block);
            e.mem.set_u32(words[1] + 8, words[2]);
            Ret::default()
        });
        let allocator = {
            let router = r.e.mem.u32(ROUTER_CELL);
            r.e.mem.u32(router + 0x10)
        };
        let vtable = r.e.mem.u32(allocator);
        r.e.mem.set_u32(vtable + 0x10, 0x0900_0910);
        let log = r.log.clone();
        r.e.register_double(0x0900_0910, move |_, words| {
            log.borrow_mut()
                .push((0x0900_0910, words.to_vec(), Vec::new()));
            Ret::default()
        });
        run(&mut r, 0x00de_7a70);
        let frees = calls(&r, 0x0900_0910);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1[2], 4);
    }
}
