//! `Havok SDK/hkpsolvergettoivelocities.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is a single-precision operation, bit for bit as the exe
//! does it.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, sum3, zip_with, Vec4};

/// The solver body copy (`00d72030`, `this` = the destination, one stack word = the source;
/// 0x80 bytes).
const BODY_COPY: u32 = 0x00d7_2030;
/// `hkVector4::setMul3` (`00c7f750`, `this` = the result, `(matrix, vector)`).
const SET_MUL3: u32 = 0x00c7_f750;
/// The size in bytes of each Jacobian schema command, by command byte (`011b9414`).
const COMMAND_SIZES: u32 = 0x011b_9414;

fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// The velocity of one Jacobian row against two bodies: with the row's three vectors at
/// `row`, `row + 0x10`, `row + 0x20`, it is the sum of the first three lanes (`z + (y + x)`)
/// of `(row[+0x20] * b[+0x20] + row[+0x10] * a[+0x20]) + (a[+0x10] - b[+0x10]) * row[0]`.
fn row_velocity(e: &Engine, row: u32, a: u32, b: u32) -> f32 {
    let by_b = mul(load_vec(e, row + 0x20), load_vec(e, b + 0x20));
    let by_a = mul(load_vec(e, row + 0x10), load_vec(e, a + 0x20));
    let linear = zip_with(load_vec(e, a + 0x10), load_vec(e, b + 0x10), |x, y| x - y);
    sum3(add(add(by_b, by_a), mul(linear, load_vec(e, row))))
}

/// The "no more than zero" clamp the x87 does for the rows that push bodies apart: `0.0`
/// when the value is greater than zero (ordered), else the value itself (`FCOM` with the
/// zero on the stack, `C0 | C3` clear only for a greater value).
fn clamp_to_zero(value: f32) -> f32 {
    if value > 0.0 {
        0.0
    } else {
        value
    }
}

// Translated from 00d9f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolveGetToiViolatingConstraintVelocity` (engine map), cdecl `(_unused_1, schema, bodies,
/// limit, out)` (the first word is never read): walks the Jacobian schema and writes, for
/// every row, how fast the two bodies move against it. `schema + 4` and `schema + 6` are
/// the two 16-bit body indices (into the 0x80-byte bodies at `bodies`) and the commands start
/// at `schema + 0x10`; `limit` (signed) is the number of rows that may be written, `out`
/// receives the `float`s, and the number of rows written is returned. Commands (the first byte,
/// sign-extended): `0`, `1`, `0x16`, `0x17`, `0x18` end the walk; `2` jumps by the word at
/// `+4`; `3`, `8`, `9`, `0xa`, `0xc`, `0xd`, `0xe`, `0xf`, `0x19`, `0x1a` (and a run of the same
/// command) are skipped by the size in the table at `011b9414`; `5` and `6` (0x30 bytes) and
/// `7` (0x40 bytes) write one row velocity; `0x10` .. `0x13` (0x30 bytes) one row velocity
/// clamped to at most zero; `0x14` and `0x15` (0x70 bytes) two clamped rows (the second row's
/// vectors at `+0x30`); `0x1b` (0x60 bytes) replaces both bodies with copies on the stack
/// and adds `matrix * [+0x20]` (resp. `[+0x50]`) of each body to its `[+0x10]` (resp. `[+0x40]`)
/// using the two 3x3 matrices at `+0` and `+0x30`. A row that would be written past `limit`
/// ends the walk (the double-row command needs room for two). The commands `4` and `0xb`, and
/// any byte above `0x1b` (including negative ones), make the exe jump to the same
/// instruction for ever (`00d9f739`); they panic here.
pub fn hk_solve_get_toi_violating_constraint_velocity(
    e: &mut Engine,
    _unused_1: u32,
    schema: u32,
    bodies: u32,
    limit: i32,
    out: u32,
) -> u32 {
    e.with_stack(0x180, |e, frame| {
        let frame = frame.addr();
        let first_copy = frame + 0x70;
        let second_copy = frame + 0xf0;
        let mut body_a = u32::from(e.mem.u16(schema + 4))
            .wrapping_mul(0x80)
            .wrapping_add(bodies);
        let mut body_b = u32::from(e.mem.u16(schema + 6))
            .wrapping_mul(0x80)
            .wrapping_add(bodies);
        let mut cursor = schema + 0x10;
        let mut out = out;
        let mut count = 0u32;
        // The 1-based number of the next row; the double-row command needs it and the next.
        let mut slot = 1i32;
        loop {
            let command = i32::from(e.mem.i8(cursor));
            match command {
                0 | 1 | 0x16 | 0x17 | 0x18 => return count,
                2 => cursor = cursor.wrapping_add(e.mem.u32(cursor + 4)),
                3 | 8 | 9 | 0xa | 0xc | 0xd | 0xe | 0xf | 0x19 | 0x1a => loop {
                    let index = i32::from(e.mem.i8(cursor)) as u32;
                    let size = u32::from(e.mem.u8(COMMAND_SIZES.wrapping_add(index)));
                    cursor = cursor.wrapping_add(size);
                    if i32::from(e.mem.i8(cursor)) != command {
                        break;
                    }
                },
                5..=7 | 0x10..=0x13 => {
                    if slot > limit {
                        return count;
                    }
                    let value = row_velocity(e, cursor, body_a, body_b);
                    let value = if command >= 0x10 {
                        clamp_to_zero(value)
                    } else {
                        value
                    };
                    e.mem.set_f32(out, value);
                    count += 1;
                    slot += 1;
                    out = out.wrapping_add(4);
                    cursor = cursor.wrapping_add(if command == 7 { 0x40 } else { 0x30 });
                }
                0x14 | 0x15 => {
                    if slot.wrapping_add(1) > limit {
                        return count;
                    }
                    let first = clamp_to_zero(row_velocity(e, cursor, body_a, body_b));
                    e.mem.set_f32(out, first);
                    let second = clamp_to_zero(row_velocity(e, cursor + 0x30, body_a, body_b));
                    e.mem.set_f32(out + 4, second);
                    count += 2;
                    slot += 2;
                    out = out.wrapping_add(8);
                    cursor = cursor.wrapping_add(0x70);
                }
                0x1b => {
                    e.call(BODY_COPY, &args![first_copy, body_a]);
                    body_a = first_copy;
                    e.call(BODY_COPY, &args![second_copy, body_b]);
                    body_b = second_copy;
                    e.call(
                        SET_MUL3,
                        &args![frame + 0x30, cursor, first_copy + 0x20],
                    );
                    e.call(
                        SET_MUL3,
                        &args![frame + 0x60, cursor + 0x30, second_copy + 0x20],
                    );
                    e.call(
                        SET_MUL3,
                        &args![frame + 0x50, cursor, first_copy + 0x50],
                    );
                    e.call(
                        SET_MUL3,
                        &args![frame + 0x40, cursor + 0x30, second_copy + 0x50],
                    );
                    for (body, product) in [
                        (first_copy + 0x10, frame + 0x30),
                        (first_copy + 0x40, frame + 0x50),
                        (second_copy + 0x10, frame + 0x60),
                        (second_copy + 0x40, frame + 0x40),
                    ] {
                        let sum = add(load_vec(e, body), load_vec(e, product));
                        store_vec(e, body, sum);
                    }
                    cursor = cursor.wrapping_add(0x60);
                }
                _ => panic!(
                    "00d9f739: schema command {command:#x} at {cursor:#010x}: the exe spins here for ever"
                ),
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d9_f6f0,
        hk_solve_get_toi_violating_constraint_velocity(u32, u32, u32, i32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The command sizes the exe keeps at `011b9414`.
    const SIZES: [u8; 28] = [
        16, 16, 16, 16, 255, 48, 48, 64, 64, 64, 80, 64, 32, 48, 48, 64, 48, 48, 48, 48, 112, 112,
        112, 144, 80, 32, 32, 96,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x011b_9000, 0x1000);
        e.call_log = Some(Vec::new());
        for (index, size) in SIZES.into_iter().enumerate() {
            e.mem.set_u8(COMMAND_SIZES + index as u32, size);
        }
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    const ZERO: Vec4 = [0.0; 4];
    const POSITIVE: Vec4 = [0.0, 1.0, 0.0, 0.0];
    const NEGATIVE: Vec4 = [0.0, -1.0, 0.0, 0.0];

    /// Two bodies: body 0 moves at `(0, 3, 0)` with angular `(0, 5, 0)`, body 1 at
    /// `(0, 1, 0)` with angular `(0, 0, 7)`. Returns the base of the two bodies.
    fn bodies(e: &mut Engine) -> u32 {
        let base = e.mem.alloc(0x100);
        store_vec(e, base + 0x10, [0.0, 3.0, 0.0, 0.0]);
        store_vec(e, base + 0x20, [0.0, 5.0, 0.0, 0.0]);
        store_vec(e, base + 0x80 + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + 0x80 + 0x20, [0.0, 0.0, 7.0, 0.0]);
        base
    }

    /// A schema for the two bodies with room for `size` bytes of commands.
    fn schema(e: &mut Engine, size: u32) -> u32 {
        let schema = e.mem.alloc(0x10 + size);
        e.mem.set_u16(schema + 4, 0);
        e.mem.set_u16(schema + 6, 1);
        schema
    }

    /// A command of the kind `kind` at `at`, with the three vectors of its row. The kind
    /// shares the low byte of the first vector, so it is written last.
    fn put_command(e: &mut Engine, at: u32, kind: u8, linear: Vec4, first: Vec4, second: Vec4) {
        store_vec(e, at, linear);
        store_vec(e, at + 0x10, first);
        store_vec(e, at + 0x20, second);
        e.mem.set_u8(at, kind);
    }

    #[test]
    fn rows_are_written_in_command_order_and_clamped_where_asked() {
        let mut e = engine();
        let base = bodies(&mut e);
        let schema = schema(&mut e, 0x180);
        let commands = schema + 0x10;
        // Type 5: 1 * (3 - 1) + 1 * 5 + 1 * 7 = 14.
        put_command(
            &mut e,
            commands,
            5,
            POSITIVE,
            POSITIVE,
            [0.0, 0.0, 1.0, 0.0],
        );
        // Type 0x10 keeps a value below zero; type 0x11 turns a positive one to zero.
        put_command(&mut e, commands + 0x30, 0x10, NEGATIVE, ZERO, ZERO);
        put_command(&mut e, commands + 0x60, 0x11, POSITIVE, ZERO, ZERO);
        // Type 7 is 0x40 bytes long.
        put_command(&mut e, commands + 0x90, 7, POSITIVE, ZERO, ZERO);
        // Two commands of type 3 are skipped, a type 2 jumps ahead by 0x10.
        e.mem.set_u8(commands + 0xd0, 3);
        e.mem.set_u8(commands + 0xe0, 3);
        e.mem.set_u8(commands + 0xf0, 2);
        e.mem.set_u32(commands + 0xf4, 0x10);
        // Type 0x14: two clamped rows, the second one's vectors at +0x30.
        put_command(&mut e, commands + 0x100, 0x14, POSITIVE, ZERO, ZERO);
        put_command(&mut e, commands + 0x130, 0, NEGATIVE, ZERO, ZERO);
        e.mem.set_u8(commands + 0x170, 0);
        let out = e.mem.alloc(0x40);
        let result = e.call(0x00d9_f6f0, &args![0u32, schema, base, 8i32, out]);
        assert_eq!(result.u32(), 6);
        let values: Vec<f32> = (0..6).map(|row| e.mem.f32(out + 4 * row)).collect();
        assert_eq!(values, vec![14.0, -2.0, 0.0, 2.0, 0.0, -2.0]);
        // The limit stops the walk at the first row that would not fit: three rows fit
        // before the type 7 command.
        let result = e.call(0x00d9_f6f0, &args![0u32, schema, base, 3i32, out]);
        assert_eq!(result.u32(), 3);
        // The double-row command needs room for two: with 5 rows only, it is not run.
        let result = e.call(0x00d9_f6f0, &args![0u32, schema, base, 5i32, out]);
        assert_eq!(result.u32(), 4);
        // A limit of zero writes nothing.
        let result = e.call(0x00d9_f6f0, &args![0u32, schema, base, 0i32, out]);
        assert_eq!(result.u32(), 0);
    }

    #[test]
    fn the_transform_command_works_on_copies_of_the_bodies() {
        let mut e = engine();
        let base = bodies(&mut e);
        store_vec(&mut e, base + 0x40, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, base + 0x50, [0.0, 0.0, 2.0, 0.0]);
        let schema = schema(&mut e, 0x100);
        let commands = schema + 0x10;
        // The matrices' first rows carry the scale in lane 1: 2 for the first body, 3 for the
        // second.
        put_command(&mut e, commands, 0x1b, [0.0, 2.0, 0.0, 0.0], ZERO, ZERO);
        store_vec(&mut e, commands + 0x30, [0.0, 3.0, 0.0, 0.0]);
        // Then a row reading the second lanes of the linear velocities.
        put_command(&mut e, commands + 0x60, 5, POSITIVE, ZERO, ZERO);
        e.mem.set_u8(commands + 0x90, 0);
        e.register_double(BODY_COPY, |e, a| {
            for word in 0..0x20 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register_double(SET_MUL3, |e, a| {
            let scale = e.mem.f32(a[1] + 4);
            let vector = load_vec(e, a[2]);
            store_vec(e, a[0], vector.map(|lane| lane * scale));
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        let result = e.call(0x00d9_f6f0, &args![0u32, schema, base, 4i32, out]);
        assert_eq!(result.u32(), 1);
        // The first copy's linear velocity becomes (0, 3 + 2 * 5, 0), the second's
        // (0, 1, 3 * 7): the row sees 13 - 1 in lane 1.
        assert_eq!(e.mem.f32(out), 12.0);
        let copies = calls(&e, BODY_COPY);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1], base);
        assert_eq!(copies[1][1], base + 0x80);
        // The stack copies took the additions; the real bodies did not.
        assert_eq!(load_vec(&e, base + 0x10), [0.0, 3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, base + 0x40), [0.0, 1.0, 0.0, 0.0]);
        // The vector products were asked for with the matrices at +0 and +0x30.
        let products = calls(&e, SET_MUL3);
        assert_eq!(products.len(), 4);
        assert_eq!(products[0][1], commands);
        assert_eq!(products[1][1], commands + 0x30);
        assert_eq!(products[2][1], commands);
        assert_eq!(products[3][1], commands + 0x30);
    }

    #[test]
    #[should_panic(expected = "00d9f739")]
    fn an_unknown_command_hangs_the_exe() {
        let mut e = engine();
        let base = bodies(&mut e);
        let schema = schema(&mut e, 0x10);
        e.mem.set_u8(schema + 0x10, 4);
        let out = e.mem.alloc(0x10);
        e.call(0x00d9_f6f0, &args![0u32, schema, base, 4i32, out]);
    }
}
