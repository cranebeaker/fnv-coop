//! `Havok SDK/hkpsimplecollisionresponse.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is a single-precision operation, bit for bit as the exe
//! does it. The scalar parts that go through the x87 stack compute in `double`, load through
//! `x87_quiet` and round to `float` where the exe stores.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsimpleconstraintutil::{add, cross, mul, rows_times, splat, sub};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_vec, movhlps, movlhps, shufps, store_vec, sum3, unpckhps,
    unpcklps,
};

/// `hkMatrix3::transpose` (`00cd4b60`, `this` = the matrix).
const MATRIX_TRANSPOSE: u32 = 0x00cd_4b60;
/// `hkMatrix3::setMul` (`00cd55b0`, `this` = the result, `(first, second)`, `RET 8`).
const MATRIX_SET_MUL: u32 = 0x00cd_55b0;
/// `hkSimpleConstraintUtil_InitInfo` (`00df5950`, cdecl `(body_a, body_b, rotation, info)`).
const INIT_INFO: u32 = 0x00df_5950;
/// `hkSimpleConstraintUtil_GetPointVelocity` (`00df5e80`, cdecl `(info, first, second, out)`).
const GET_POINT_VELOCITY: u32 = 0x00df_5e80;
/// `hkSimpleConstraintUtil_Collide` (`00df5f80`, cdecl `(info, params, first, second)`, the
/// result in ST0).
const COLLIDE: u32 = 0x00df_5f80;
/// The vector of ones (`010ec7f0`).
const UNIT_VECTOR: u32 = 0x010e_c7f0;
/// The body type byte value (at `+0xe8` of a rigid body, `+8` of the body objects
/// `solveSingleContact` takes) for which the velocities are not written back.
const FIXED_TYPE: u8 = 5;

/// The absolute value of one component of the three-float `normal`, loaded through the x87
/// stack (`FLD`, `FABS`, `FSTP`).
fn axis_magnitude(e: &Engine, normal: u32, axis: u32) -> f32 {
    x87_quiet(e.mem.f32(normal + 4 * axis)).abs()
}

/// Stores the three rows of the contact basis at `rotation`: the vector at `normal` (all four
/// lanes), a unit vector perpendicular to it and the cross product of the two. The
/// perpendicular is built from the two components of the normal that are not the smallest in
/// magnitude: with their axes `low` (0 or 1) and `high`, it has the normal's `high` component
/// at `low` and the negated `low` component at `high`, zeros elsewhere, and is scaled by the
/// refined `RSQRTSS` of its squared length (`0` when that is zero).
fn store_contact_basis(e: &mut Engine, normal: u32, rotation: u32) {
    let first = axis_magnitude(e, normal, 0);
    let second = axis_magnitude(e, normal, 1);
    let third = axis_magnitude(e, normal, 2);
    // `FCOMP` falls through to the update only when `first > second` (an unordered compare
    // keeps the first axis); the same holds for the third compare below.
    let (mut least, mut low, mut smallest) = (first, 1u32, 0u32);
    if first > second {
        least = second;
        low = 0;
        smallest = 1;
    }
    let high = if least > third { smallest } else { 2 };
    let mut tangent = [0.0f32; 4];
    tangent[low as usize] = x87_quiet(e.mem.f32(normal + 4 * high));
    tangent[high as usize] = -x87_quiet(e.mem.f32(normal + 4 * low));
    let scale = inverse_length(e, sum3(mul(tangent, tangent)));
    let unit = mul(splat(scale), tangent);
    let normal_vector = load_vec(e, normal);
    store_vec(e, rotation, normal_vector);
    store_vec(e, rotation + 0x10, unit);
    store_vec(e, rotation + 0x20, cross(normal_vector, unit));
}

// Translated from 00d9ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleCollisionResponse::solveSingleContact2` (engine map), cdecl with eight words
/// `(handler, contact, params, body_a, body_b, record_a, record_b, result_out)`.
///
/// `contact` has the contact position at `+0` and the contact normal at `+0x10`; `handler` is
/// an object whose virtual slots `0x40` and `0x44` are called with it as `this`; `params` is
/// the friction block of `hkSimpleConstraintUtil_Collide`; `record_a` and `record_b` are the
/// per-body solver records (matrices at `+0x50 ..`, the velocity vector at `+0x10`).
///
/// Per body this builds a 0x60-byte block on the stack: the contact position minus the
/// record's `+0x40` vector, the product matrix from `setMul` (the record's matrix at `+0x50`
/// transposed in place, its columns scaled by the record's `+0x30` vector, times the matrix
/// again), the record's `+0x30` vector, and the pointer `body + 0xf0`; next to it goes a
/// two-vector pair (the record's `+0x10` vector and a vector from the record's `+0x20`
/// against its matrix). It then makes the contact basis (`store_contact_basis`), calls
/// slot `0x40` (five words), `hkSimpleConstraintUtil_InitInfo`, `Collide` (its result is stored
/// at `result_out`) and slot `0x44` (six words, the result as a float word). The pair vectors
/// the slots leave are stored at `record + 0x10` and, for a body whose type byte at `+0xe8`
/// is not 5, at `body + 0x1b0` and `body + 0x1c0`; last each record's `+0x20` vector becomes
/// the second pair vector times the record's matrix.
#[allow(clippy::too_many_arguments)]
pub fn hk_simple_collision_response_solve_single_contact2(
    e: &mut Engine,
    handler: u32,
    contact: u32,
    params: u32,
    body_a: u32,
    body_b: u32,
    record_a: u32,
    record_b: u32,
    result_out: u32,
) {
    e.with_stack(0x380, |e, frame| {
        let base = frame.addr();
        for (index, (body, record)) in [(body_a, record_a), (body_b, record_b)]
            .into_iter()
            .enumerate()
        {
            let index = index as u32;
            let block = base + 0x100 + 0x60 * index;
            let pair = base + 0x30 + 0x20 * index;
            copy_vec(e, record + 0x30, block + 0x40);
            let relative = sub(load_vec(e, contact), load_vec(e, record + 0x40));
            store_vec(e, block, relative);
            copy_vec(e, record + 0x10, pair);
            // The record's matrix (three vectors) goes to the stack and is transposed there.
            let row_a = load_vec(e, record + 0x50);
            let row_b = load_vec(e, record + 0x60);
            let row_c = load_vec(e, record + 0x70);
            let scales = load_vec(e, record + 0x20);
            store_vec(e, base + 0xa0, row_a);
            store_vec(e, base + 0xb0, row_b);
            store_vec(e, base + 0xc0, row_c);
            let low = unpcklps(row_a, row_b);
            let columns = movlhps(low, row_c);
            let high = unpckhps(row_a, row_b);
            let second_column = shufps(movhlps(columns, low), row_c, 0xd4);
            let third_column = shufps(high, row_c, 0xe4);
            let combined = add(
                add(
                    mul(second_column, splat(scales[1])),
                    mul(splat(scales[0]), columns),
                ),
                mul(third_column, splat(scales[2])),
            );
            store_vec(e, pair + 0x10, combined);
            e.call(MATRIX_TRANSPOSE, &args![base + 0xa0]);
            let factors = load_vec(e, record + 0x30);
            let transposed_a = load_vec(e, base + 0xa0);
            let transposed_b = load_vec(e, base + 0xb0);
            let transposed_c = load_vec(e, base + 0xc0);
            store_vec(e, base + 0xd0, mul(splat(factors[0]), transposed_a));
            store_vec(e, base + 0xe0, mul(splat(factors[1]), transposed_b));
            store_vec(e, base + 0xf0, mul(splat(factors[2]), transposed_c));
            e.call(
                MATRIX_SET_MUL,
                &args![block + 0x10, base + 0xd0, record + 0x50],
            );
            e.mem.set_u32(block + 0x50, body + 0xf0);
        }
        store_contact_basis(e, contact + 0x10, base + 0x70);
        e.vcall(
            handler,
            0x40,
            &args![
                contact,
                base + 0x100,
                base + 0x30,
                base + 0x160,
                base + 0x50
            ],
        );
        e.call(
            INIT_INFO,
            &args![base + 0x100, base + 0x160, base + 0x70, base + 0x1c0],
        );
        let result = e
            .call(
                COLLIDE,
                &args![base + 0x1c0, params, base + 0x30, base + 0x50],
            )
            .f32();
        e.mem.set_f32(result_out, x87_quiet(result));
        e.vcall(
            handler,
            0x44,
            &args![
                contact,
                x87_quiet(result),
                base + 0x100,
                base + 0x30,
                base + 0x160,
                base + 0x50
            ],
        );
        copy_vec(e, base + 0x30, record_a + 0x10);
        if e.mem.u8(body_a + 0xe8) != FIXED_TYPE {
            copy_vec(e, base + 0x30, body_a + 0x1b0);
            copy_vec(e, base + 0x40, body_a + 0x1c0);
        }
        copy_vec(e, base + 0x50, record_b + 0x10);
        if e.mem.u8(body_b + 0xe8) != FIXED_TYPE {
            copy_vec(e, base + 0x50, body_b + 0x1b0);
            copy_vec(e, base + 0x60, body_b + 0x1c0);
        }
        for (record, pair_vector) in [(record_a, base + 0x40), (record_b, base + 0x60)] {
            let vector = load_vec(e, pair_vector);
            let rows = [
                load_vec(e, record + 0x50),
                load_vec(e, record + 0x60),
                load_vec(e, record + 0x70),
            ];
            let transformed = rows_times(vector, rows[0], rows[1], rows[2]);
            store_vec(e, record + 0x20, transformed);
        }
    });
}

// Translated from 00d9f250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleCollisionResponse::solveSingleContact` (engine map), cdecl with seven words
/// `(contact, time, params, body_a, body_b, handler, result_out)`; `time` is a `float`.
///
/// `contact` has the contact position at `+0` and the normal at `+0x10`; the bodies are
/// objects with a type byte at `+8` and virtual slots `0x28` (fills a block from a
/// position), `0x40` and `0x44`; `handler` has virtual slots `0x38` and `0x3c`; `result_out`
/// receives three floats.
///
/// For each body `f = (time - body[+0x5c]) * body[+0x6c]` (through the x87 stack) blends
/// `body[+0x50]` and `body[+0x60]`; the contact position minus that is stored at the start of
/// a 0x60-byte block (`body[+0xc0]` at `+0x40`, the pointer `body + 0x10` at `+0x50`) and
/// slot `0x28` fills the block's `+0x10 ..`. The body's `+0xd0` and `+0xe0` vectors go to a
/// two-vector record, and `f` times `body[+0x60] - body[+0x50]` and `f` times
/// `body[+0xa0]` (using `body[+0x6c]`, not the blend) to a second pair. The contact basis is
/// stored (`store_contact_basis`), handler slot `0x38` (five words), `InitInfo`, `Collide`
/// (the result is stored at `result_out`) and handler slot `0x3c` (six words, the result as
/// a float word) follow, then `GetPointVelocity` twice, whose first lanes go to
/// `result_out + 4` and `result_out + 8`. Last, each body whose type byte at `+8` is not 5
/// gets its two record vectors through its virtual slots `0x40` and `0x44`.
#[allow(clippy::too_many_arguments)]
pub fn hk_simple_collision_response_solve_single_contact(
    e: &mut Engine,
    contact: u32,
    time: f32,
    params: u32,
    body_a: u32,
    body_b: u32,
    handler: u32,
    result_out: u32,
) {
    e.with_stack(0x380, |e, frame| {
        let base = frame.addr();
        let block_a = base + 0x150;
        let block_b = base + 0xf0;
        prepare_block(e, contact, time, body_a, block_a);
        e.mem.set_u32(block_a + 0x50, body_a + 0x10);
        prepare_block(e, contact, time, body_b, block_b);
        e.mem.set_u32(block_b + 0x50, body_b + 0x10);
        // The two-vector records (`base + 0x40`, `base + 0x20`) come from the bodies.
        copy_vec(e, body_a + 0xd0, base + 0x40);
        copy_vec(e, body_a + 0xe0, base + 0x50);
        copy_vec(e, body_b + 0xd0, base + 0x20);
        copy_vec(e, body_b + 0xe0, base + 0x30);
        let factor_a = x87_quiet(e.mem.f32(body_a + 0x6c));
        let spin_a = sub(load_vec(e, body_a + 0x60), load_vec(e, body_a + 0x50));
        store_vec(e, base + 0xd0, mul(splat(factor_a), spin_a));
        let rate_a = load_vec(e, body_a + 0xa0);
        store_vec(e, base + 0xe0, mul(splat(factor_a), rate_a));
        let factor_b = x87_quiet(e.mem.f32(body_b + 0x6c));
        let spin_b = sub(load_vec(e, body_b + 0x60), load_vec(e, body_b + 0x50));
        store_vec(e, base + 0x90, mul(splat(factor_b), spin_b));
        let rate_b = load_vec(e, body_b + 0xa0);
        store_vec(e, base + 0xa0, mul(splat(factor_b), rate_b));
        store_contact_basis(e, contact + 0x10, base + 0x60);
        e.vcall(
            handler,
            0x38,
            &args![contact, block_a, base + 0x40, block_b, base + 0x20],
        );
        let info = base + 0x1b0;
        e.call(INIT_INFO, &args![block_a, block_b, base + 0x60, info]);
        let result = e
            .call(COLLIDE, &args![info, params, base + 0x40, base + 0x20])
            .f32();
        e.mem.set_f32(result_out, x87_quiet(result));
        e.vcall(
            handler,
            0x3c,
            &args![
                contact,
                x87_quiet(result),
                block_a,
                base + 0x40,
                block_b,
                base + 0x20
            ],
        );
        e.call(
            GET_POINT_VELOCITY,
            &args![info, base + 0xd0, base + 0x20, base + 0xb0],
        );
        e.call(
            GET_POINT_VELOCITY,
            &args![info, base + 0x40, base + 0x90, base + 0xc0],
        );
        let first_velocity = x87_quiet(e.mem.f32(base + 0xb0));
        e.mem.set_f32(result_out + 4, first_velocity);
        let second_velocity = x87_quiet(e.mem.f32(base + 0xc0));
        e.mem.set_f32(result_out + 8, second_velocity);
        if e.mem.u8(body_a + 8) != FIXED_TYPE {
            e.vcall(body_a, 0x40, &args![base + 0x40]);
            e.vcall(body_a, 0x44, &args![base + 0x50]);
        }
        if e.mem.u8(body_b + 8) != FIXED_TYPE {
            e.vcall(body_b, 0x40, &args![base + 0x20]);
            e.vcall(body_b, 0x44, &args![base + 0x30]);
        }
    });
}

/// The per-body block of `solveSingleContact` at `block` (0x60 bytes): `f = (time -
/// body[+0x5c]) * body[+0x6c]` is the x87 product, `body[+0xc0]` goes to `block + 0x40`, the
/// contact position minus `(1 - f) * body[+0x50] + body[+0x60] * f` to `block`, and
/// virtual slot `0x28` of the body fills `block + 0x10`.
fn prepare_block(e: &mut Engine, contact: u32, time: f32, body: u32, block: u32) {
    let elapsed = f64::from(x87_quiet(time)) - f64::from(x87_quiet(e.mem.f32(body + 0x5c)));
    let blend = (elapsed * f64::from(x87_quiet(e.mem.f32(body + 0x6c)))) as f32;
    copy_vec(e, body + 0xc0, block + 0x40);
    let ones = load_vec(e, UNIT_VECTOR);
    let before = mul(sub(ones, splat(blend)), load_vec(e, body + 0x50));
    let after = mul(load_vec(e, body + 0x60), splat(blend));
    let relative = sub(load_vec(e, contact), add(before, after));
    store_vec(e, block, relative);
    e.vcall(body, 0x28, &args![block + 0x10]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_ee50,
            hk_simple_collision_response_solve_single_contact2(
                u32, u32, u32, u32, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00d9_f250,
            hk_simple_collision_response_solve_single_contact(u32, f32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type WordLog = Rc<RefCell<Vec<Vec<u32>>>>;
    /// An argument index and the number of words to keep from the address it holds.
    type Span = (usize, u32);

    /// Doubles for the handler's two virtual slots and the body's three.
    const HANDLER_FIRST_DOUBLE: u32 = 0x0900_7001;
    const HANDLER_SECOND_DOUBLE: u32 = 0x0900_7002;
    const BODY_FILL_DOUBLE: u32 = 0x0900_7010;
    const BODY_FIRST_DOUBLE: u32 = 0x0900_7011;
    const BODY_SECOND_DOUBLE: u32 = 0x0900_7012;
    const HANDLER_VTABLE: u32 = 0x0900_7100;
    const BODY_VTABLE: u32 = 0x0900_7200;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        store_vec(&mut e, UNIT_VECTOR, [1.0; 4]);
        e
    }

    fn vtable(e: &mut Engine, at: u32, entries: &[(u32, u32)]) {
        let mut slots = vec![0u32; 0x12];
        for &(offset, target) in entries {
            slots[(offset / 4) as usize] = target;
        }
        e.put_vtable(at, &slots);
    }

    fn words(e: &Engine, addr: u32, count: u32) -> Vec<u32> {
        (0..count).map(|word| e.mem.u32(addr + 4 * word)).collect()
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    fn near(actual: &[u32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        for (word, wanted) in actual.iter().zip(expected) {
            let value = f32::from_bits(*word);
            assert!(
                (value - wanted).abs() < 1e-4,
                "{value} is not near {wanted}"
            );
        }
    }

    /// Registers a double that keeps every call's argument words followed by the words at the
    /// addresses in `spans` (read before the double acts), then runs `act`.
    fn recorder(
        e: &mut Engine,
        addr: u32,
        spans: &'static [Span],
        act: impl Fn(&mut Engine, &[u32]) -> Ret + 'static,
    ) -> WordLog {
        let log: WordLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&log);
        e.register_double(addr, move |e, a| {
            let mut entry = a.to_vec();
            for &(index, count) in spans {
                entry.extend(words(e, a[index], count));
            }
            kept.borrow_mut().push(entry);
            act(e, a)
        });
        log
    }

    fn object(e: &mut Engine, size: u32, vtable_at: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, vtable_at);
        block
    }

    /// A solver record: the rows `[1,2,3,4]`, `[5,6,7,8]`, `[9,10,11,12]` at `+0x50 ..`, the
    /// given `+0x10`, `+0x20`, `+0x30` and `+0x40` vectors.
    fn record(e: &mut Engine, vectors: [[f32; 4]; 4]) -> u32 {
        let block = e.mem.alloc(0x80);
        for (index, vector) in vectors.into_iter().enumerate() {
            store_vec(e, block + 0x10 * (index as u32 + 1), vector);
        }
        store_vec(e, block + 0x50, [1.0, 2.0, 3.0, 4.0]);
        store_vec(e, block + 0x60, [5.0, 6.0, 7.0, 8.0]);
        store_vec(e, block + 0x70, [9.0, 10.0, 11.0, 12.0]);
        block
    }

    #[test]
    fn solve_single_contact2_builds_blocks_runs_the_handler_and_applies_results() {
        let mut e = engine();
        e.register_double(MATRIX_TRANSPOSE, |e, a| {
            e.mem.set_f32(a[0], 7.0);
            Ret::default()
        });
        let set_mul = recorder(&mut e, MATRIX_SET_MUL, &[(1, 12)], |e, a| {
            e.mem.set_f32(a[0], 11.0);
            Ret::default()
        });
        let first = recorder(
            &mut e,
            HANDLER_FIRST_DOUBLE,
            &[(2, 24), (3, 8), (4, 24), (5, 8)],
            |e, a| {
                store_vec(e, a[3], [1.0, 2.0, 3.0, 4.0]);
                store_vec(e, a[3] + 0x10, [5.0, 6.0, 7.0, 8.0]);
                store_vec(e, a[5], [9.0, 10.0, 11.0, 12.0]);
                store_vec(e, a[5] + 0x10, [13.0, 14.0, 15.0, 16.0]);
                Ret::default()
            },
        );
        let init = recorder(&mut e, INIT_INFO, &[(2, 12)], |_, _| Ret::default());
        let collide = recorder(&mut e, COLLIDE, &[(2, 8), (3, 8)], |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        let second = recorder(&mut e, HANDLER_SECOND_DOUBLE, &[], |_, _| Ret::default());
        vtable(
            &mut e,
            HANDLER_VTABLE,
            &[(0x40, HANDLER_FIRST_DOUBLE), (0x44, HANDLER_SECOND_DOUBLE)],
        );
        let handler = object(&mut e, 8, HANDLER_VTABLE);
        let contact = e.mem.alloc(0x20);
        store_vec(&mut e, contact, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, contact + 0x10, [0.6, 0.0, 0.8, 0.0]);
        let body_a = e.mem.alloc(0x200);
        let body_b = e.mem.alloc(0x200);
        e.mem.set_u8(body_b + 0xe8, 5);
        let record_a = record(
            &mut e,
            [
                [0.5; 4],
                [2.0, 3.0, 4.0, 0.0],
                [2.0, 3.0, 4.0, 0.0],
                [0.25, 0.5, 0.75, 0.0],
            ],
        );
        let record_b = record(
            &mut e,
            [
                [0.25; 4],
                [1.0, 0.0, 0.0, 0.0],
                [1.0, 1.0, 1.0, 0.0],
                [1.0, 1.0, 1.0, 0.0],
            ],
        );
        let out = e.mem.alloc(4);
        let params = 0x1111_0000u32;
        e.call(
            0x00d9_ee50,
            &args![handler, contact, params, body_a, body_b, record_a, record_b, out],
        );

        // The two matrix products: the transposed rows (the double sets lane 0 to 7) scaled
        // by the record's factors.
        let set_mul = set_mul.borrow();
        assert_eq!(set_mul.len(), 2);
        assert_eq!(
            set_mul[0][3..15],
            bits(&[14.0, 4.0, 6.0, 8.0, 15.0, 18.0, 21.0, 24.0, 36.0, 40.0, 44.0, 48.0])[..]
        );
        assert_eq!(
            set_mul[1][3..15],
            bits(&[7.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0])[..]
        );
        assert_eq!(set_mul[0][2], record_a + 0x50);
        assert_eq!(set_mul[1][2], record_b + 0x50);

        // The handler's first call: the contact, the two blocks and the two pairs.
        let first = first.borrow();
        assert_eq!(first.len(), 1);
        let call = &first[0];
        assert_eq!(call[..2], [handler, contact]);
        let (block_a, pair_a, block_b, pair_b) = (call[2], call[3], call[4], call[5]);
        assert_eq!(pair_b, pair_a + 0x20);
        assert_eq!(block_b, block_a + 0x60);
        assert_eq!(set_mul[0][0], block_a + 0x10);
        assert_eq!(set_mul[1][0], block_b + 0x10);
        let block_a_words = &call[6..30];
        assert_eq!(
            block_a_words[..8],
            bits(&[0.75, 1.5, 2.25, 0.0, 11.0, 0.0, 0.0, 0.0])[..]
        );
        assert_eq!(block_a_words[16..20], bits(&[2.0, 3.0, 4.0, 0.0])[..]);
        assert_eq!(block_a_words[20], body_a + 0xf0);
        assert_eq!(
            call[30..38],
            bits(&[0.5, 0.5, 0.5, 0.5, 20.0, 56.0, 92.0, 104.0])[..]
        );
        let block_b_words = &call[38..62];
        assert_eq!(
            block_b_words[..8],
            bits(&[0.0, 1.0, 2.0, 0.0, 11.0, 0.0, 0.0, 0.0])[..]
        );
        assert_eq!(block_b_words[20], body_b + 0xf0);
        assert_eq!(
            call[62..70],
            bits(&[0.25, 0.25, 0.25, 0.25, 1.0, 5.0, 9.0, 10.0])[..]
        );

        // The basis handed to the init routine: the normal, a perpendicular and their cross
        // product.
        let init = init.borrow();
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][..2], [block_a, block_b]);
        near(
            &init[0][4..16],
            &[0.6, 0.0, 0.8, 0.0, 0.8, 0.0, -0.6, 0.0, 0.0, 1.0, 0.0, 0.0],
        );
        let info = init[0][3];

        // Collide gets the pairs the handler filled in; its result is stored and passed on.
        let collide = collide.borrow();
        assert_eq!(collide.len(), 1);
        assert_eq!(collide[0][..4], [info, params, pair_a, pair_b]);
        assert_eq!(
            collide[0][4..12],
            bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])[..]
        );
        assert_eq!(
            collide[0][12..20],
            bits(&[9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0])[..]
        );
        assert_eq!(e.mem.f32(out), 0.25);
        assert_eq!(
            second.borrow()[..],
            [vec![
                handler,
                contact,
                0.25f32.to_bits(),
                block_a,
                pair_a,
                block_b,
                pair_b
            ]]
        );

        // The results: the first pair vectors go to the records, and to the bodies whose type
        // byte is not 5.
        assert_eq!(words(&e, record_a + 0x10, 4), bits(&[1.0, 2.0, 3.0, 4.0]));
        assert_eq!(
            words(&e, record_b + 0x10, 4),
            bits(&[9.0, 10.0, 11.0, 12.0])
        );
        assert_eq!(words(&e, body_a + 0x1b0, 4), bits(&[1.0, 2.0, 3.0, 4.0]));
        assert_eq!(words(&e, body_a + 0x1c0, 4), bits(&[5.0, 6.0, 7.0, 8.0]));
        assert_eq!(words(&e, body_b + 0x1b0, 8), vec![0; 8]);
        // Each record's `+0x20` vector: the second pair vector times the record's rows.
        assert_eq!(
            floats(&words(&e, record_a + 0x20, 4)),
            [98.0, 116.0, 134.0, 152.0]
        );
        assert_eq!(
            floats(&words(&e, record_b + 0x20, 4)),
            [218.0, 260.0, 302.0, 344.0]
        );
    }

    #[test]
    fn solve_single_contact_blends_the_bodies_and_reports_the_velocities() {
        let mut e = engine();
        let fill = recorder(&mut e, BODY_FILL_DOUBLE, &[], |e, a| {
            e.mem.set_f32(a[1], 33.0);
            Ret::default()
        });
        let body_first = recorder(&mut e, BODY_FIRST_DOUBLE, &[(1, 4)], |_, _| Ret::default());
        let body_second = recorder(&mut e, BODY_SECOND_DOUBLE, &[(1, 4)], |_, _| Ret::default());
        vtable(
            &mut e,
            BODY_VTABLE,
            &[
                (0x28, BODY_FILL_DOUBLE),
                (0x40, BODY_FIRST_DOUBLE),
                (0x44, BODY_SECOND_DOUBLE),
            ],
        );
        vtable(
            &mut e,
            HANDLER_VTABLE,
            &[(0x38, HANDLER_FIRST_DOUBLE), (0x3c, HANDLER_SECOND_DOUBLE)],
        );
        let first = recorder(
            &mut e,
            HANDLER_FIRST_DOUBLE,
            &[(2, 24), (3, 8), (4, 24), (5, 8)],
            |_, _| Ret::default(),
        );
        let init = recorder(&mut e, INIT_INFO, &[(2, 12)], |_, _| Ret::default());
        let collide = recorder(&mut e, COLLIDE, &[], |_, _| Ret {
            st0: 0.125,
            ..Ret::default()
        });
        let second = recorder(&mut e, HANDLER_SECOND_DOUBLE, &[], |e, a| {
            store_vec(e, a[4], [1.0, 2.0, 3.0, 4.0]);
            store_vec(e, a[4] + 0x10, [5.0, 6.0, 7.0, 8.0]);
            store_vec(e, a[6], [9.0, 10.0, 11.0, 12.0]);
            store_vec(e, a[6] + 0x10, [13.0, 14.0, 15.0, 16.0]);
            Ret::default()
        });
        let velocity_calls = Rc::new(Cell::new(0u32));
        let counted = Rc::clone(&velocity_calls);
        let velocity = recorder(
            &mut e,
            GET_POINT_VELOCITY,
            &[(1, 8), (2, 8)],
            move |e, a| {
                e.mem.set_f32(a[3], 3.5 + counted.get() as f32);
                counted.set(counted.get() + 1);
                Ret::default()
            },
        );
        let handler = object(&mut e, 8, HANDLER_VTABLE);
        let contact = e.mem.alloc(0x20);
        store_vec(&mut e, contact, [10.0, 20.0, 30.0, 0.0]);
        store_vec(&mut e, contact + 0x10, [0.8, 0.6, 0.0, 0.0]);
        let body_a = object(&mut e, 0x100, BODY_VTABLE);
        let body_b = object(&mut e, 0x100, BODY_VTABLE);
        e.mem.set_u8(body_b + 8, 5);
        let setup = [
            (body_a, [1.0, 2.0, 3.0], [3.0, 2.0, 1.0], 1.0, 1.0),
            (body_b, [0.0, 0.0, 0.0], [2.0, 2.0, 2.0], 0.5, 0.5),
        ];
        for (body, from, to, stamp, scale) in setup {
            store_vec(&mut e, body + 0x50, [from[0], from[1], from[2], 0.0]);
            store_vec(&mut e, body + 0x60, [to[0], to[1], to[2], 0.0]);
            e.mem.set_f32(body + 0x5c, stamp);
            e.mem.set_f32(body + 0x6c, scale);
        }
        store_vec(&mut e, body_a + 0xa0, [0.5, 1.0, 1.5, 0.0]);
        store_vec(&mut e, body_b + 0xa0, [4.0, 4.0, 4.0, 0.0]);
        store_vec(&mut e, body_a + 0xc0, [7.0, 8.0, 9.0, 0.0]);
        store_vec(&mut e, body_b + 0xc0, [1.0, 1.0, 1.0, 0.0]);
        store_vec(&mut e, body_a + 0xd0, [10.0, 11.0, 12.0, 13.0]);
        store_vec(&mut e, body_a + 0xe0, [14.0, 15.0, 16.0, 17.0]);
        store_vec(&mut e, body_b + 0xd0, [18.0, 19.0, 20.0, 21.0]);
        store_vec(&mut e, body_b + 0xe0, [22.0, 23.0, 24.0, 25.0]);
        let out = e.mem.alloc(12);
        let params = 0x2222_0000u32;
        e.call(
            0x00d9_f250,
            &args![contact, 1.5f32, params, body_a, body_b, handler, out],
        );

        // Both bodies were asked to fill their blocks (`body + 0x10 .. `), and the blocks hold
        // the contact position minus the blend of the body's two positions.
        assert_eq!(fill.borrow().len(), 2);
        let first_call = first.borrow()[0].clone();
        assert_eq!(first_call[..2], [handler, contact]);
        let (block_a, record_a, block_b, record_b) =
            (first_call[2], first_call[3], first_call[4], first_call[5]);
        assert_eq!(block_a, block_b + 0x60);
        assert_eq!(fill.borrow()[0][..], [body_a, block_a + 0x10]);
        assert_eq!(fill.borrow()[1][..], [body_b, block_b + 0x10]);
        // The fourth lane is the blend of the bodies' `+0x5c` and `+0x6c` words (the fourth lanes
        // of their position vectors) taken off the contact's zero.
        let words_a = &first_call[6..30];
        assert_eq!(
            words_a[..8],
            bits(&[8.0, 18.0, 28.0, -1.0, 33.0, 0.0, 0.0, 0.0])[..]
        );
        assert_eq!(words_a[16..20], bits(&[7.0, 8.0, 9.0, 0.0])[..]);
        assert_eq!(words_a[20], body_a + 0x10);
        let words_b = &first_call[38..62];
        assert_eq!(
            words_b[..8],
            bits(&[9.0, 19.0, 29.0, -0.5, 33.0, 0.0, 0.0, 0.0])[..]
        );
        assert_eq!(words_b[20], body_b + 0x10);
        // The records start as the bodies' `+0xd0` and `+0xe0` vectors.
        assert_eq!(
            first_call[30..38],
            bits(&[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0])[..]
        );
        assert_eq!(
            first_call[62..70],
            bits(&[18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0])[..]
        );

        // The basis: the normal (0.8, 0.6, 0), whose smallest axis is the third.
        let init_call = init.borrow()[0].clone();
        assert_eq!(init_call[..2], [block_a, block_b]);
        near(
            &init_call[4..16],
            &[0.8, 0.6, 0.0, 0.0, 0.6, -0.8, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0],
        );
        assert_eq!(
            collide.borrow()[0][..4],
            [init_call[3], params, record_a, record_b]
        );
        assert_eq!(
            second.borrow()[0][..],
            [
                handler,
                contact,
                0.125f32.to_bits(),
                block_a,
                record_a,
                block_b,
                record_b
            ]
        );

        // The two velocity queries: the scaled spin pairs and the records the handler set.
        {
            let velocity = velocity.borrow();
            assert_eq!(velocity.len(), 2);
            assert_eq!(velocity[0][..3], [init_call[3], record_a + 0x90, record_b]);
            assert_eq!(
                velocity[0][4..12],
                bits(&[2.0, 0.0, -2.0, 0.0, 0.5, 1.0, 1.5, 0.0])[..]
            );
            assert_eq!(
                velocity[0][12..20],
                bits(&[9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0])[..]
            );
            assert_eq!(velocity[1][1..3], [record_a, record_a + 0x50]);
            assert_eq!(
                velocity[1][4..12],
                bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])[..]
            );
            assert_eq!(
                velocity[1][12..20],
                bits(&[1.0, 1.0, 1.0, 0.0, 2.0, 2.0, 2.0, 0.0])[..]
            );
            assert_eq!(floats(&words(&e, out, 3)), [0.125, 3.5, 4.5]);
        }

        // Only the body whose type byte is not 5 gets its velocities.
        assert_eq!(body_first.borrow().len(), 1);
        assert_eq!(body_first.borrow()[0][0], body_a);
        assert_eq!(body_first.borrow()[0][2..], bits(&[1.0, 2.0, 3.0, 4.0])[..]);
        assert_eq!(
            body_second.borrow()[0][2..],
            bits(&[5.0, 6.0, 7.0, 8.0])[..]
        );
        assert_eq!(body_second.borrow().len(), 1);

        // With both bodies live, and the normal's first component the smaller one, the other
        // branch of the perpendicular is taken.
        e.mem.set_u8(body_b + 8, 0);
        store_vec(&mut e, contact + 0x10, [0.6, 0.8, 0.0, 0.0]);
        e.call(
            0x00d9_f250,
            &args![contact, 1.5f32, params, body_a, body_b, handler, out],
        );
        near(
            &init.borrow()[1][4..16],
            &[0.6, 0.8, 0.0, 0.0, -0.8, 0.6, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        );
        let body_first = body_first.borrow();
        assert_eq!(body_first.len(), 3);
        assert_eq!(body_first[2][0], body_b);
        assert_eq!(body_first[2][2..], bits(&[9.0, 10.0, 11.0, 12.0])[..]);
        assert_eq!(
            body_second.borrow()[2][2..],
            bits(&[13.0, 14.0, 15.0, 16.0])[..]
        );
    }
}
