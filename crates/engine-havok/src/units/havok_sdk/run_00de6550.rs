//! `Havok SDK/run_00de6550` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three more constraint drawing functions (the engine map has no names), thiscall on the
//! constraint drawer with four stack words `(constraint, display handler, first word, second
//! word)`, built like the two of `run_00de59f0.rs`, whose `pub(crate)` helpers they use: the
//! frame is addressed as the exe addresses it (relative to the stack pointer after the three
//! saved registers), the vector arithmetic is SSE in `f32`, the x87 products are `f64` rounded
//! to `float` at each store.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00de59f0::{
    add, broadcast, copied_float, cross, draw_arrows, float_product, load, mul, show_limit_shapes,
    squared_length, store, update_limit_shapes, Normaliser, DRAW_BODY_FRAMES, DRAW_PIVOTS,
    RGB_FROM_FLOATS, SET_DISPLAY_HANDLER, SET_ROTATED_DIR, SET_TRANSFORMED_POS,
    UPDATE_COMMON_PARAMETERS, ZERO,
};

/// `_ftol2` (`00ec62c0`, the value passed in `ST0`, here two words).
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// The float `0.07` (`0104eab4`) of the first lane of the spacing vector of `00de6550`, and
/// the double `-0.5` (`01011598`) of its offset.
const SPACING: u32 = 0x0104_eab4;
const HALF_NEGATIVE: u32 = 0x0101_1598;
/// The constants of `00de6d80`: the doubles `-2.0` (`0106d4c8`) and `1.4` as a float widened
/// (`010b7e90`), and the floats `0.8` (`01030ff0`) and `0.3` (`01017870`) of its colours.
const MINUS_TWO: u32 = 0x0106_d4c8;
const SPREAD: u32 = 0x010b_7e90;
const GREY: u32 = 0x0103_0ff0;
const DARK: u32 = 0x0101_7870;

/// The byte at `at`, as the exe's `MOVZX`.
fn byte(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at))
}

// Translated from 00de6550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a row of lines (the engine map has no name), thiscall with four
/// words. After the common set-up (`setDisplayHandler`, `updateCommonParameters` into the
/// blocks at frame `+0x70` / `+0xb0`, the drawer's points at `+0x50` / `+0x40` transformed from
/// the constraint data's `+0x30` / `+0x20`, `drawPivots`, `drawBodyFrames`) it draws two lines
/// with the display handler's slot `0x24` (`data + 0x50` to `data + 0x60` in `0xffc0c0c0`, and
/// `drawer + 0x40` to `data + 0x50` in `0xffffffff`). The count `n` is `_ftol2` of the float at
/// `data + 0x74` when that is above 1.0 (otherwise 1.0; a NaN gives 1.0). The vector `(0.07,
/// 0, 0, 0)` (the float at `0104eab4` and the zero at `01011d78`) is rotated by the second body
/// (`[constraint + 0x18] + 0xf0`) into frame `+0x30`, and for every `i` below `n` a line is
/// drawn in `0xffffffff` from `drawer + 0x50 + (i * v + base)` to `(i * v + base) + data +
/// 0x60`, with `v` the rotated vector and `base = (n - 1) * -0.5 * v`.
pub fn fn_00de6550(
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
            &args![me + 0x50, f + 0x70, data.wrapping_add(0x30)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x40, f + 0xb0, data.wrapping_add(0x20)],
        );
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        e.call(DRAW_BODY_FRAMES, &args![me, first_word, second_word]);
        e.vcall(
            display_handler,
            0x24,
            &args![
                data.wrapping_add(0x50),
                data.wrapping_add(0x60),
                0xffc0_c0c0u32,
                first_word,
                second_word
            ],
        );
        e.vcall(
            display_handler,
            0x24,
            &args![
                me + 0x40,
                data.wrapping_add(0x50),
                0xffff_ffffu32,
                first_word,
                second_word
            ],
        );
        // The count: max(1.0, float) through `FCOM`, then the truncation.
        let limit = x87_load32(e.mem.u32(data.wrapping_add(0x74)));
        let counted = if limit > 1.0 { limit } else { 1.0 };
        let counted = x87_load32(x87_store32(counted));
        let n = e.call(FLOAT_TO_INT, &args![counted]).i32();
        // The spacing vector rotated by the second body.
        let spacing = e.global::<f32>(SPACING);
        let zero = e.global::<f32>(ZERO);
        let second_body = e.mem.u32(constraint.wrapping_add(0x18));
        store(e, f + 0x30, [spacing, zero, zero, zero]);
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x30, second_body.wrapping_add(0xf0), f + 0x30],
        );
        let steps = x87_store32(x87_mul(
            f64::from(n.wrapping_sub(1)),
            e.mem.f64(HALF_NEGATIVE),
        ));
        let base = mul(broadcast(f32::from_bits(steps)), load(e, f + 0x30));
        store(e, f + 0x40, base);
        let mut i: i32 = 0;
        while i < n {
            let index = f32::from_bits(x87_store32(f64::from(i)));
            let base = load(e, f + 0x40);
            let offset = add(mul(broadcast(index), load(e, f + 0x30)), base);
            let point = load(e, me + 0x50);
            store(e, f + 0x50, add(point, offset));
            let end_vector = load(e, data.wrapping_add(0x60));
            store(e, f + 0x60, add(offset, end_vector));
            e.vcall(
                display_handler,
                0x24,
                &args![f + 0x50, f + 0x60, 0xffff_ffffu32, first_word, second_word],
            );
            i = i.wrapping_add(1);
        }
    });
}

// Translated from 00de6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a cone and a plane limit from the packed constraint data of a
/// ragdoll (the engine map has no name), thiscall with four words: the same drawing as
/// `00de59f0` with the constraint data `D` (the pointer at `constraint + 0xc`) and `B = D +
/// 0x10` read through byte indexes. The points are transformed from `D + 0x90` and `D + 0x50`;
/// frame `+0x80` gets `B + ((byte[D + 0xff] + 5) << 4)` rotated; with the bytes `i1 =
/// byte[B + 0x117]` and `i2 = byte[B + 0x118]`: frame `+0x70` the unit cross product of
/// `B + 0x50 + (i1 << 4)` and `B + 0x50 + (i2 << 4)` rotated by the first frame, frame `+0x60`
/// `B + 0x50 + (i2 << 4)` rotated by it, frame `+0x90` the unit cross product of `B + 0x10 +
/// (i2 << 4)` and `B + 0x10 + (i1 << 4)` rotated by the second frame (`drawer + 0x60`) and
/// frame `+0xc0` `B + 0x10 + (byte[B + 0xef] << 4)` rotated by it. The arrows, cones and
/// plane are those of `00de59f0` (`draw_arrows`, `update_limit_shapes`, `show_limit_shapes`),
/// the angles read from `D + 0x11c`, `B + 0x120` and `B + 0x11c`.
pub fn fn_00de6730(
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
            &args![me, constraint, f + 0xd0, f + 0x110],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x50, f + 0xd0, data.wrapping_add(0x90)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x40, f + 0x110, data.wrapping_add(0x50)],
        );
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        let packed = data.wrapping_add(0x10);
        let selected = byte(e, data.wrapping_add(0xff)).wrapping_add(5) << 4;
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x80, me + 0x90, packed.wrapping_add(selected)],
        );
        // The unit normal of the two selected vectors, rotated by the first frame.
        let second_index = byte(e, packed.wrapping_add(0x118)) << 4;
        let first_index = byte(e, packed.wrapping_add(0x117)) << 4;
        let normaliser = Normaliser::read(e);
        let first = load(e, packed.wrapping_add(0x50).wrapping_add(first_index));
        let second = load(e, packed.wrapping_add(0x50).wrapping_add(second_index));
        let normal = cross(first, second);
        let scale = normaliser.scale(squared_length(normal));
        store(e, f + 0x70, mul(broadcast(scale), normal));
        e.call(SET_ROTATED_DIR, &args![f + 0x70, me + 0x90, f + 0x70]);
        let second_index = byte(e, packed.wrapping_add(0x118)) << 4;
        e.call(
            SET_ROTATED_DIR,
            &args![
                f + 0x60,
                me + 0x90,
                packed.wrapping_add(0x50).wrapping_add(second_index)
            ],
        );
        // The unit normal of the two selected vectors of the other row, rotated by the second.
        let first_index = byte(e, packed.wrapping_add(0x117)) << 4;
        let second_index = byte(e, packed.wrapping_add(0x118)) << 4;
        let first = load(e, packed.wrapping_add(0x10).wrapping_add(second_index));
        let second = load(e, packed.wrapping_add(0x10).wrapping_add(first_index));
        let normal = cross(first, second);
        let scale = normaliser.scale(squared_length(normal));
        store(e, f + 0xb0, mul(broadcast(scale), normal));
        e.call(SET_ROTATED_DIR, &args![f + 0x90, me + 0x60, f + 0xb0]);
        let selected = byte(e, packed.wrapping_add(0xef)) << 4;
        e.call(
            SET_ROTATED_DIR,
            &args![
                f + 0xc0,
                me + 0x60,
                packed.wrapping_add(0x10).wrapping_add(selected)
            ],
        );
        draw_arrows(e, me, f, first_word, second_word);
        let first_angle = copied_float(e, data.wrapping_add(0x11c));
        let saved_limit = copied_float(e, packed.wrapping_add(0x11c));
        let second_limit = e.mem.u32(packed.wrapping_add(0x120));
        update_limit_shapes(e, me, f, first_angle, second_limit, saved_limit);
        show_limit_shapes(e, me, f, display_handler, first_word, second_word);
    });
}

// Translated from 00de6d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a box of lines (the engine map has no name), thiscall with four
/// words. After the common set-up (blocks at frame `+0x40` / `+0x80`, the drawer's points at
/// `+0x50` / `+0x40` transformed from the constraint data's `+0x90` / `+0x50`, `drawPivots`,
/// `drawBodyFrames`) it rotates two rows of the data by the first frame (`drawer + 0x90`): row
/// `((byte[D + 0xa2] + 1) % 3) + 6` into frame `+0x30` and row `((byte + 2) % 3) + 6` into `+0x20`
/// (rows are 16 bytes from `D`). With `s = [drawer + 0x1c] * [drawer]` (rounded to a float) it
/// builds, for the two choices of which of the two vectors is `A` and which `B` and for `j` in
/// `-1..=1`, the pairs of points `P + t * A + u * B` with `P = [drawer + 0x50]`, `t = +2s` and
/// `t = -2s` and `u = float(float(j * 1.4) * s)` (the order of the additions is the exe's), 12
/// vectors in all (frame `+0xc0` on), and draws the six pairs as lines with slot `0x24` in
/// the colour of `rgbFromFloats(0.8, 0.8, 0.8, 1)`. The last line, from the saved point
/// `drawer + 0x40` to `drawer + 0x20`, is in `rgbFromFloats(0.3, 0.3, 0.8, 1)`.
pub fn fn_00de6d80(
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
    e.with_stack(0x1c0, |e, frame| {
        let f = frame.addr();
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![me, constraint, f + 0x40, f + 0x80],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x50, f + 0x40, data.wrapping_add(0x90)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![me + 0x40, f + 0x80, data.wrapping_add(0x50)],
        );
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        e.call(DRAW_BODY_FRAMES, &args![me, first_word, second_word]);
        let axis = byte(e, data.wrapping_add(0xa2));
        let row = ((axis.wrapping_add(1) % 3) + 6) << 4;
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x30, me + 0x90, data.wrapping_add(row)],
        );
        let axis = byte(e, data.wrapping_add(0xa2));
        let row = ((axis.wrapping_add(2) % 3) + 6) << 4;
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x20, me + 0x90, data.wrapping_add(row)],
        );
        // The scale: s = [+0x1c] * [+0], twice that, and -2 times it, each stored as a float.
        let product = float_product(e, me + 0x1c, me);
        let product_float = x87_load32(x87_store32(product));
        let twice = f32::from_bits(x87_store32(x87_add(product_float, product_float)));
        let minus_twice = f32::from_bits(x87_store32(x87_mul(e.mem.f64(MINUS_TWO), product_float)));
        let spread = e.mem.f64(SPREAD);
        let point = load(e, me + 0x50);
        // The 12 vectors: for the outer index `o` the pairs at `+0xc0 + o * 0x60 + m * 0x20`.
        for outer in 0..2u32 {
            let (along, across) = if outer == 0 {
                (load(e, f + 0x20), load(e, f + 0x30))
            } else {
                (load(e, f + 0x30), load(e, f + 0x20))
            };
            for step in 0..3u32 {
                let j = f64::from(step as i32 - 1);
                let t = f32::from_bits(x87_store32(x87_mul(x87_mul(j, spread), product_float)));
                let first = add(
                    mul(broadcast(t), across),
                    add(mul(broadcast(twice), along), point),
                );
                let second = add(
                    mul(broadcast(t), across),
                    add(mul(broadcast(minus_twice), along), point),
                );
                let at = f + 0xd0 + outer * 0x60 + step * 0x20;
                store(e, at - 0x10, first);
                store(e, at, second);
            }
        }
        // Six grey lines, one per pair.
        for pair in 0..6u32 {
            let grey = copied_float(e, GREY);
            let colour = e
                .call(RGB_FROM_FLOATS, &args![grey, grey, grey, 0x3f80_0000u32])
                .u32();
            let from = f + 0xc0 + pair * 0x20;
            e.vcall(
                display_handler,
                0x24,
                &args![from, from + 0x10, colour, first_word, second_word],
            );
        }
        // The last line.
        let blue = copied_float(e, GREY);
        let dark = copied_float(e, DARK);
        let colour = e
            .call(RGB_FROM_FLOATS, &args![dark, dark, blue, 0x3f80_0000u32])
            .u32();
        e.vcall(
            display_handler,
            0x24,
            &args![me + 0x40, me + 0x20, colour, first_word, second_word],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_6550, fn_00de6550(Ptr, u32, u32, u32, u32)),
        entry!(0x00de_6730, fn_00de6730(Ptr, u32, u32, u32, u32)),
        entry!(0x00de_6d80, fn_00de6d80(Ptr, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00de59f0::{
        ARRAY_RESERVE, CONE_SET, DISPLAY_ARROW, MEMORY_ROUTER, PLANE_SET, RGB_FROM_CHARS, THREE,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One call of a callee: its address, its argument words and the 16-byte blocks the
    /// listed words point to, read when the call happens.
    pub(crate) type Call = (u32, Vec<u32>, Vec<[u32; 4]>);
    pub(crate) type Log = Rc<RefCell<Vec<Call>>>;

    pub(crate) const LINE_SLOT: u32 = 0x0900_0024;
    pub(crate) const GEOMETRY_SLOT: u32 = 0x0900_0038;
    pub(crate) const ROUTER_CELL: u32 = 0x0126_8100;

    pub(crate) fn block(e: &Engine, at: u32) -> [u32; 4] {
        std::array::from_fn(|lane| e.mem.u32(at + 4 * lane as u32))
    }

    pub(crate) fn bits(v: [f32; 4]) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    pub(crate) fn set_block(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, value) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
    }

    /// A double that logs `(address, words, blocks at the listed word indexes)` and answers
    /// `eax`.
    pub(crate) fn watch(
        e: &mut Engine,
        log: &Log,
        address: u32,
        blocks: &'static [usize],
        eax: u32,
    ) {
        let log = log.clone();
        e.register_double(address, move |e, words| {
            let seen = blocks.iter().map(|i| block(e, words[*i])).collect();
            log.borrow_mut().push((address, words.to_vec(), seen));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    pub(crate) struct Rig {
        pub(crate) e: Engine,
        pub(crate) log: Log,
        pub(crate) drawer: u32,
        pub(crate) constraint: u32,
        pub(crate) data: u32,
        pub(crate) handler: u32,
    }

    /// A drawer with `[+0x1c] = 2`, `[+0] = 3`, the points `+0x20`, `+0x40`, `+0x50`, a
    /// constraint with a second body and a data block, the constants of the exe, doubles for
    /// every callee (the rotation double copies its vector into its result; `_ftol2`
    /// truncates its argument) and a display handler whose slots `0x24` and `0x38` log.
    pub(crate) fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem
            .set_f32(crate::units::havok_sdk::run_00de59f0::HALF, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(0x010c_44d0 + 4 * lane, 0x8000_0000);
        }
        e.mem
            .set_f64(0x0103_0f38, f64::from_bits(0x3ff9_21fb_6000_0000));
        e.mem.set_f32(SPACING, 0.5);
        e.mem.set_f64(HALF_NEGATIVE, -0.5);
        e.mem.set_f64(MINUS_TWO, -2.0);
        e.mem.set_f64(SPREAD, 1.5);
        e.mem.set_f32(GREY, 0.8);
        e.mem.set_f32(DARK, 0.3);
        let log: Log = Rc::default();
        let drawer = e.mem.alloc(0x300);
        e.mem.set_f32(drawer + 0x1c, 2.0);
        e.mem.set_f32(drawer, 3.0);
        e.mem.set_f32(drawer + 0xc, 5.0);
        e.mem.set_f32(drawer + 0x10, 7.0);
        e.mem.set_f32(drawer + 0x14, 11.0);
        e.mem.set_u32(drawer + 0x18, 24);
        set_block(&mut e, drawer + 0x20, [100.0, 200.0, 300.0, 0.0]);
        set_block(&mut e, drawer + 0x40, [10.0, 20.0, 30.0, 0.0]);
        set_block(&mut e, drawer + 0x50, [1.0, 2.0, 3.0, 0.0]);
        let data = e.mem.alloc(0x200);
        let body = e.mem.alloc(0x200);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_u32(constraint + 0x18, body);
        for address in [
            SET_DISPLAY_HANDLER,
            UPDATE_COMMON_PARAMETERS,
            SET_TRANSFORMED_POS,
            DRAW_PIVOTS,
            DRAW_BODY_FRAMES,
        ] {
            watch(&mut e, &log, address, &[], 0);
        }
        {
            let log = log.clone();
            e.register_double(SET_ROTATED_DIR, move |e, words| {
                let seen = block(e, words[2]);
                log.borrow_mut()
                    .push((SET_ROTATED_DIR, words.to_vec(), vec![seen]));
                for (lane, word) in seen.iter().enumerate() {
                    e.mem.set_u32(words[0] + 4 * lane as u32, *word);
                }
                Ret::default()
            });
        }
        e.register(FLOAT_TO_INT, |_, words| {
            let value = f64::from_bits(u64::from(words[0]) | u64::from(words[1]) << 32);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        watch(&mut e, &log, DISPLAY_ARROW, &[2, 3], 0);
        watch(&mut e, &log, CONE_SET, &[4, 5], 0);
        watch(&mut e, &log, PLANE_SET, &[1, 2, 3, 4], 0);
        watch(&mut e, &log, RGB_FROM_CHARS, &[], 0x0c01_0000);
        watch(&mut e, &log, RGB_FROM_FLOATS, &[], 0x0f10_0000);
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(8);
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, allocator_vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(ROUTER_CELL),
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, words| {
            let block = e.mem.alloc(4 * words[2]);
            e.mem.set_u32(words[1], block);
            Ret::default()
        });
        let handler = e.mem.alloc(8);
        let handler_vtable = e.mem.alloc(0x60);
        e.mem.set_u32(handler, handler_vtable);
        e.mem.set_u32(handler_vtable + 0x24, LINE_SLOT);
        e.mem.set_u32(handler_vtable + 0x38, GEOMETRY_SLOT);
        watch(&mut e, &log, LINE_SLOT, &[1, 2], 0);
        watch(&mut e, &log, GEOMETRY_SLOT, &[], 0);
        Rig {
            e,
            log,
            drawer,
            constraint,
            data,
            handler,
        }
    }

    pub(crate) fn calls(r: &Rig, address: u32) -> Vec<Call> {
        r.log
            .borrow()
            .iter()
            .filter(|(a, _, _)| *a == address)
            .cloned()
            .collect()
    }

    pub(crate) fn run(r: &mut Rig, function: u32) {
        r.e.call(
            function,
            &args![r.drawer, r.constraint, r.handler, 0x111u32, 0x222u32],
        );
    }

    // ----- 00de6550 -----

    #[test]
    fn the_row_drawer_sets_up_and_draws_its_two_lines() {
        let mut r = rig();
        let (d, c, data) = (r.drawer, r.constraint, r.data);
        run(&mut r, 0x00de_6550);
        let log = r.log.borrow();
        let frame = log[1].1[2] - 0x70;
        let head: Vec<(u32, Vec<u32>)> = log
            .iter()
            .take(7)
            .map(|(a, w, _)| (*a, w.clone()))
            .collect();
        assert_eq!(
            head,
            vec![
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0x70, frame + 0xb0]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x50, frame + 0x70, data + 0x30]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x40, frame + 0xb0, data + 0x20]
                ),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
                (DRAW_BODY_FRAMES, vec![d, 0x111, 0x222]),
                (
                    LINE_SLOT,
                    vec![
                        r.handler,
                        data + 0x50,
                        data + 0x60,
                        0xffc0_c0c0,
                        0x111,
                        0x222
                    ]
                ),
            ]
        );
        assert_eq!(
            log[7].1,
            vec![r.handler, d + 0x40, data + 0x50, 0xffff_ffff, 0x111, 0x222]
        );
    }

    #[test]
    fn the_row_drawer_spaces_its_lines_along_the_rotated_vector() {
        let mut r = rig();
        let data = r.data;
        // the float above 1.0: the count is its truncation, 3
        r.e.mem.set_f32(data + 0x74, 3.75);
        set_block(&mut r.e, data + 0x60, [1.0, 1.0, 1.0, 0.0]);
        let body = r.e.mem.u32(r.constraint + 0x18);
        run(&mut r, 0x00de_6550);
        let frame = calls(&r, UPDATE_COMMON_PARAMETERS)[0].1[2] - 0x70;
        let rotations = calls(&r, SET_ROTATED_DIR);
        let rotation = &rotations[0];
        assert_eq!(rotation.1, vec![frame + 0x30, body + 0xf0, frame + 0x30]);
        assert_eq!(rotation.2[0], bits([0.5, 0.0, 0.0, 0.0]));
        // base = (3 - 1) * -0.5 * v = -v; the i-th line runs from P + (i * v - v)
        // to (i * v - v) + data[+0x60]
        let lines: Vec<Call> = calls(&r, LINE_SLOT).into_iter().skip(2).collect();
        assert_eq!(lines.len(), 3);
        for (i, line) in lines.iter().enumerate() {
            let minus_one = -1.0f32;
            let step = [0.5f32, 0.0, 0.0, 0.0].map(|lane| i as f32 * lane + minus_one * lane);
            let from: Vec<f32> = [1.0f32, 2.0, 3.0, 0.0]
                .iter()
                .zip(step)
                .map(|(p, s)| p + s)
                .collect();
            let to: Vec<f32> = step
                .iter()
                .zip([1.0f32, 1.0, 1.0, 0.0])
                .map(|(s, e)| s + e)
                .collect();
            assert_eq!(line.1[3..], [0xffff_ffff, 0x111, 0x222]);
            assert_eq!(line.2[0], bits([from[0], from[1], from[2], from[3]]));
            assert_eq!(line.2[1], bits([to[0], to[1], to[2], to[3]]));
        }
    }

    #[test]
    fn a_float_at_or_below_one_or_nan_counts_one_line() {
        for value in [0.25f32, 1.0, f32::NAN] {
            let mut r = rig();
            let data = r.data;
            r.e.mem.set_f32(data + 0x74, value);
            run(&mut r, 0x00de_6550);
            assert_eq!(calls(&r, LINE_SLOT).len(), 3, "value {value}");
        }
    }

    #[test]
    fn a_count_of_zero_draws_no_row() {
        let mut r = rig();
        r.e.register(FLOAT_TO_INT, |_, _| Ret::default());
        run(&mut r, 0x00de_6550);
        assert_eq!(calls(&r, LINE_SLOT).len(), 2);
    }

    // ----- 00de6730 -----

    #[test]
    fn the_packed_cone_drawer_selects_its_vectors_by_the_index_bytes() {
        let mut r = rig();
        let (d, c, data) = (r.drawer, r.constraint, r.data);
        let packed = data + 0x10;
        r.e.mem.set_u8(data + 0xff, 1);
        r.e.mem.set_u8(packed + 0x117, 1);
        r.e.mem.set_u8(packed + 0x118, 2);
        set_block(&mut r.e, packed + 0x60, [1.0, 0.0, 0.0, 0.0]);
        set_block(&mut r.e, packed + 0x70, [0.0, 1.0, 0.0, 0.0]);
        set_block(&mut r.e, packed + 0x20, [0.0, 0.0, 1.0, 0.0]);
        set_block(&mut r.e, packed + 0x30, [1.0, 0.0, 0.0, 0.0]);
        r.e.mem.set_f32(data + 0x11c, 0.7);
        r.e.mem.set_f32(packed + 0x11c, 0.4);
        r.e.mem.set_f32(packed + 0x120, -0.3);
        run(&mut r, 0x00de_6730);
        let log = r.log.borrow();
        let frame = log[1].1[2] - 0xd0;
        let head: Vec<(u32, Vec<u32>)> = log
            .iter()
            .take(5)
            .map(|(a, w, _)| (*a, w.clone()))
            .collect();
        assert_eq!(
            head,
            vec![
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0xd0, frame + 0x110]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x50, frame + 0xd0, data + 0x90]
                ),
                (
                    SET_TRANSFORMED_POS,
                    vec![d + 0x40, frame + 0x110, data + 0x50]
                ),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
            ]
        );
        drop(log);
        let rotations: Vec<Vec<u32>> = calls(&r, SET_ROTATED_DIR)
            .iter()
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(rotations.len(), 5);
        assert_eq!(rotations[0], vec![frame + 0x80, d + 0x90, packed + 0x60]);
        assert_eq!(rotations[1], vec![frame + 0x70, d + 0x90, frame + 0x70]);
        assert_eq!(rotations[2], vec![frame + 0x60, d + 0x90, packed + 0x70]);
        assert_eq!(rotations[3], vec![frame + 0x90, d + 0x60, frame + 0xb0]);
        assert_eq!(
            rotations[4],
            vec![frame + 0xc0, d + 0x60, packed + 0x10 + 16]
        );
        // the normals: x cross y is z, x cross z is -y (the rotation double copies them on)
        let unit = {
            let squared = 1.0f32;
            let r = crate::x87::rsqrt_estimate(squared);
            (0.5 * r) * (3.0 - (squared * r) * r)
        };
        let seen = calls(&r, SET_ROTATED_DIR);
        assert_eq!(seen[1].2[0], bits([0.0, 0.0, unit, 0.0]));
        assert_eq!(seen[3].2[0], bits([0.0, -unit, 0.0, 0.0]));
        // the angles of the cones come from the three floats
        let cones = calls(&r, CONE_SET);
        assert_eq!(cones.len(), 3);
        assert_eq!(cones[0].1[1], 0.7f32.to_bits());
        let half_pi = f64::from_bits(0x3ff9_21fb_6000_0000);
        assert_eq!(
            cones[1].1[1],
            ((half_pi - f64::from(0.3f32)) as f32).to_bits()
        );
        assert_eq!(
            cones[2].1[1],
            ((half_pi - f64::from(0.4f32)) as f32).to_bits()
        );
        assert_eq!(calls(&r, DISPLAY_ARROW).len(), 2);
        assert_eq!(calls(&r, PLANE_SET).len(), 1);
        assert_eq!(calls(&r, GEOMETRY_SLOT).len(), 3);
    }

    // ----- 00de6d80 -----

    #[test]
    fn the_box_drawer_draws_six_grey_lines_and_a_last_one() {
        let mut r = rig();
        let (d, data) = (r.drawer, r.data);
        r.e.mem.set_u8(data + 0xa2, 1);
        // rows ((1 + 1) % 3 + 6) = 8 and ((1 + 2) % 3 + 6) = 6
        set_block(&mut r.e, data + 0x80, [4.0, 5.0, 6.0, 0.0]);
        set_block(&mut r.e, data + 0x60, [1.0, 2.0, 3.0, 0.0]);
        run(&mut r, 0x00de_6d80);
        let frame = calls(&r, UPDATE_COMMON_PARAMETERS)[0].1[2] - 0x40;
        let rotations = calls(&r, SET_ROTATED_DIR);
        assert_eq!(rotations.len(), 2);
        assert_eq!(rotations[0].1, vec![frame + 0x30, d + 0x90, data + 0x80]);
        assert_eq!(rotations[1].1, vec![frame + 0x20, d + 0x90, data + 0x60]);
        let colours: Vec<Vec<u32>> = calls(&r, RGB_FROM_FLOATS)
            .iter()
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(colours.len(), 7);
        let (grey, dark, one) = (0.8f32.to_bits(), 0.3f32.to_bits(), 1.0f32.to_bits());
        assert!(colours[..6]
            .iter()
            .all(|w| *w == vec![grey, grey, grey, one]));
        assert_eq!(colours[6], vec![dark, dark, grey, one]);
        let lines = calls(&r, LINE_SLOT);
        // the two lines of 00de6550 are not drawn here: 6 grey lines and the last one
        assert_eq!(lines.len(), 7);
        // s = 3 * 2 = 6; twice = 12; -twice = -12; spread 1.5; t = float(float(j * 1.5) * 6)
        let point = [1.0f32, 2.0, 3.0, 0.0];
        let a = [[1.0f32, 2.0, 3.0, 0.0], [4.0, 5.0, 6.0, 0.0]];
        let b = [[4.0f32, 5.0, 6.0, 0.0], [1.0, 2.0, 3.0, 0.0]];
        // frame +0x20 holds the data +0x60 row (1, 2, 3), frame +0x30 the data +0x80 row
        let mut index = 0;
        for outer in 0..2 {
            for step in 0..3 {
                let t = ((f64::from(step - 1) * 1.5) * 6.0) as f32;
                for (entry, twice) in [(0, 12.0f32), (1, -12.0)] {
                    let want: Vec<f32> = (0..4)
                        .map(|lane| t * b[outer][lane] + (twice * a[outer][lane] + point[lane]))
                        .collect();
                    let got = lines[index].2[entry];
                    assert_eq!(got, bits([want[0], want[1], want[2], want[3]]));
                }
                assert_eq!(lines[index].1[3], 0x0f10_0000);
                index += 1;
            }
        }
        assert_eq!(
            lines[6].1,
            vec![r.handler, d + 0x40, d + 0x20, 0x0f10_0000, 0x111, 0x222]
        );
    }
}
