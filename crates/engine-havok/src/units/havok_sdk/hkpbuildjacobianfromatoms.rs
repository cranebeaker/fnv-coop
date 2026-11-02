//! `Havok SDK/hkpbuildjacobianfromatoms.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The solver's Jacobian builders. Each function appends fixed-size records to the Jacobian
//! stream behind a cursor (a pointer to the write position, advanced in place). The Xbox PDB has
//! no record of the Havok classes, so the field names below are unconfirmed; the code addresses
//! the constraint "info" block (`this` of the atom builders: `+0x20` the weight (inverse time
//! step), `+0x24` the slope, `+0x30`/`+0x34` the two bodies, `+0x50` the breaking limit, `+0x60`
//! the constraint id, `+0x68` the pointer to the list of broken constraints) and the body's
//! solver data (`+0x20` angular velocity, `+0x30` the inverse inertia, `+0x40` the centre of
//! mass, `+0x50`/`+0x60`/`+0x70` the rotation rows). Functions that take values in registers
//! list them first, in register order (EAX, ECX, EDX, ESI, EDI), then the stack words.

use super::hkpboxmotion::{add, cross, mul, splat, sub};
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, shufps, store_bits, store_vec, sum3, unpckhps,
    Vec4, SIGN_MASK,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` 1.1920929e-07 (`FLT_EPSILON`) the builders add to the first word of a record and
/// to the effective mass.
const EPSILON: u32 = 0x010c_9300;
/// The `double` 3.4028234663852886e38 (`FLT_MAX`) the limit is compared with.
const MAX_FLOAT: u32 = 0x0104_e138;
/// Three unit vectors, the axes of the linear builders (16 bytes each).
const AXIS_TABLE: u32 = 0x0126_8370;
/// `InterlockedExchangeAdd` import slot (stdcall: the counter, the addend).
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;
/// Unnamed (thiscall, two stack words): `this` receives the vector made of the body's
/// angular term; the words are the body's rotation rows (`+0x50`) and angular velocity (`+0x20`).
const ROTATE_ANGULAR: u32 = 0x004b_4ab0;
/// `hkSimpleContactConstraintDataAddALittleSurfaceViscosity` (cdecl, one word).
const ADD_SURFACE_VISCOSITY: u32 = 0x00d7_2090;
/// `hkSimpleContactConstraintDataBuildJacobian` (cdecl: atom, info copy, flag, cursor).
const BUILD_CONTACT_JACOBIAN: u32 = 0x00d7_2190;
/// `hkSolverBuildJacobianFromAtomsNotContact` (cdecl: atom, id, info copy, cursor).
const BUILD_NOT_CONTACT_JACOBIAN: u32 = 0x00d6_f460;
/// `hk1dLinearBilateralConstraintUserTauBuildJacobian` (cdecl: input block, two words).
const LINEAR_BILATERAL_USER_TAU: u32 = 0x00dc_f880;
/// `hk1dLinearLimitBuildJacobian` (cdecl: input block, two words).
const LINEAR_LIMIT: u32 = 0x00dc_fa30;
/// `hk1dAngularFrictionBuildJacobian` (cdecl: input block, info, cursor).
const ANGULAR_FRICTION: u32 = 0x00dc_efb0;

/// The 16-byte write position of the first float of a record: adds `FLT_EPSILON` through the
/// x87 stack (the old word is what the stream held before, converted to a double).
fn add_epsilon(e: &mut Engine, record: u32) {
    let epsilon: f64 = e.global(EPSILON);
    let old = x87_quiet(e.mem.f32(record));
    e.mem.set_f32(record, (f64::from(old) + epsilon) as f32);
}

/// `(c.x * r0 + c.y * r1) + c.z * r2`: the vector `c` carried through three rows (the code adds
/// the `y` and `x` parts first; the sum is commutative, so the order is not observable).
fn rotate_rows(c: Vec4, rows: [Vec4; 3]) -> Vec4 {
    add(
        add(mul(splat(c, 0), rows[0]), mul(splat(c, 1), rows[1])),
        mul(splat(c, 2), rows[2]),
    )
}

/// The three rotation rows `+0x50`, `+0x60`, `+0x70` of a body's solver data.
fn body_rows(e: &Engine, body: u32) -> [Vec4; 3] {
    [
        load_vec(e, body + 0x50),
        load_vec(e, body + 0x60),
        load_vec(e, body + 0x70),
    ]
}

/// The effective-mass sum of the axis builders: lanes `x`, `y`, `z` of `acc` and lane `w` of
/// `mass`, added as `(y + w) + (x + z)` (`UNPCKHPS`, `SHUFPS`, two pair-wise adds).
fn pair_sum(acc: Vec4, mass: Vec4) -> f32 {
    let unpacked = unpckhps(acc, mass);
    let merged = shufps(acc, unpacked, 0xc4);
    let swapped = shufps(merged, merged, 0x4e);
    let pairs = add(swapped, merged);
    let shifted = shufps(pairs, pairs, 0xb1);
    add(shifted, pairs)[0]
}

/// Negates all four lanes (`XORPS` with the sign mask at `010c44d0`).
fn negate(e: &Engine, v: Vec4) -> Vec4 {
    let mask = load_bits(e, SIGN_MASK);
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// One axis record of the linear builders (the common part of `fn_00d6c630` and
/// `fn_00d6c970`): the axis (16 bytes) at `+0`, the two angular terms at `+0x10` and `+0x20`,
/// the effective mass at `+0x2c`, `slope / mass` at `+0x1c` and `(b - a) . axis * weight` at
/// `+0xc`.
fn axis_record(e: &mut Engine, info: u32, a: u32, b: u32, record: u32, axis: u32) {
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let slope = x87_quiet(e.mem.f32(info + 0x24));
    let weight = x87_quiet(e.mem.f32(info + 0x20));
    let axis_vector = load_vec(e, axis);
    store_vec(e, record, axis_vector);
    let from_a = sub(load_vec(e, a), load_vec(e, body_a + 0x40));
    let from_b = sub(load_vec(e, b), load_vec(e, body_b + 0x40));
    let axis_vector = load_vec(e, axis);
    let torque_a = cross(from_a, axis_vector);
    let torque_b = cross(axis_vector, from_b);
    let rows_a = body_rows(e, body_a);
    let rows_b = body_rows(e, body_b);
    let angular_a = rotate_rows(torque_a, rows_a);
    store_vec(e, record + 0x10, angular_a);
    let angular_b = rotate_rows(torque_b, rows_b);
    store_vec(e, record + 0x20, angular_b);
    let inverse_a = load_vec(e, body_a + 0x30);
    let inverse_b = load_vec(e, body_b + 0x30);
    let terms = add(
        mul(mul(angular_b, angular_b), inverse_b),
        mul(mul(angular_a, angular_a), inverse_a),
    );
    let sum = pair_sum(terms, add(inverse_a, inverse_b));
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    e.mem.set_f32(record + 0x2c, total);
    e.mem
        .set_f32(record + 0x1c, (f64::from(slope) / f64::from(total)) as f32);
    let distance = sum3(mul(sub(load_vec(e, b), load_vec(e, a)), axis_vector));
    e.mem.set_f32(
        record + 0xc,
        (f64::from(distance) * f64::from(weight)) as f32,
    );
}

/// The angular pair of the angular builders: `vector` carried through body A's rows into
/// `record + 0`, the negated vector through body B's rows into `record + 0x10`. Returns the sum
/// of the squared terms times the inverse inertias, `z + (y + x)`.
fn angular_pair(e: &mut Engine, info: u32, vector: u32, record: u32) -> f32 {
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let v = load_vec(e, vector);
    let negated = negate(e, v);
    let rows_a = body_rows(e, body_a);
    let rows_b = body_rows(e, body_b);
    let angular_a = rotate_rows(v, rows_a);
    store_vec(e, record, angular_a);
    let angular_b = rotate_rows(negated, rows_b);
    store_vec(e, record + 0x10, angular_b);
    let terms = add(
        mul(mul(angular_a, angular_a), load_vec(e, body_a + 0x30)),
        mul(mul(angular_b, angular_b), load_vec(e, body_b + 0x30)),
    );
    sum3(terms)
}

// Translated from 00d6c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromViscousSurfaceModifier` (engine map): `source` (EAX) is the 0x80-byte info
/// block; the stack words are the atom and the cursor. Backs the cursor up by 0x10, copies the
/// info block to the stack, walks the atom chain (`+0x14`, while the type word is at least 0x17)
/// and adds a little surface viscosity to the contact data behind the node (`node + 0x10`), then
/// builds the Jacobian of the next atom: a contact atom (type 0x16) through
/// `hkSimpleContactConstraintDataBuildJacobian` (with the flag 1; the word the exe pushes has
/// unset upper bytes the callee never reads), any other through
/// `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`.
pub fn build_jacobian_from_viscous_surface_modifier(
    e: &mut Engine,
    source: Ptr,
    atom: Ptr,
    cursor: Ptr,
) {
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    e.with_stack(0x80, |e, copy| {
        for word in 0..0x20 {
            let value = e.mem.u32(source.addr() + 4 * word);
            e.mem.set_u32(copy.addr() + 4 * word, value);
        }
        let id = u32::from(e.mem.u16(atom.addr() + 0x12));
        let mut node = atom.addr();
        loop {
            node = e.mem.u32(node + 0x14);
            if e.mem.u16(node) < 0x17 {
                break;
            }
        }
        e.call(ADD_SURFACE_VISCOSITY, &args![node + 0x10]);
        let next = e.mem.u32(atom.addr() + 0x14);
        if e.mem.u16(next) == 0x16 {
            e.call(BUILD_CONTACT_JACOBIAN, &args![next, copy, 1u32, cursor]);
        } else {
            e.call(BUILD_NOT_CONTACT_JACOBIAN, &args![next, id, copy, cursor]);
        }
    });
}

// Translated from 00d6c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Three linear-limit records (no engine-map name). `info` (EDI) is the info block; the stack
/// words are the two world points `a` and `b`, the cursor, the limit and a scale. Makes each
/// body's angular term through the unnamed rotate function, forms the position error
/// `(b - a) + (rb x ob) * h - (ra x oa) * h` with `h = (1 / weight) * scale`, and when the
/// list at `info + 0x68` exists and `info + 0x50` minus the squared error is negative appends
/// the constraint id at `info + 0x60` to it (an `InterlockedExchangeAdd` slot, at most 0x80
/// entries). Then each axis gets a record from `axis_record`: the `+0xc` word is replaced by
/// the error component times the weight, the first float gets `FLT_EPSILON` added and the tag
/// byte is 5 when `limit` equals `FLT_MAX` and 6 otherwise (then `+0x2c` holds the limit).
pub fn fn_00d6c630(e: &mut Engine, info: Ptr, a: Ptr, b: Ptr, cursor: Ptr, limit: f32, scale: f32) {
    let info = info.addr();
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let mut record = e.mem.u32(cursor.addr());
    let weight_reciprocal = 1.0f64 / f64::from(e.mem.f32(info + 0x20));
    let step = (weight_reciprocal * f64::from(scale)) as f32;
    let a_point = load_vec(e, a.addr());
    let b_point = load_vec(e, b.addr());
    let from_a = sub(a_point, load_vec(e, body_a + 0x40));
    let from_b = sub(b_point, load_vec(e, body_b + 0x40));
    let angular_a = rotated_angular(e, body_a);
    let angular_b = rotated_angular(e, body_b);
    let steps = [step; 4];
    let moved = add(mul(cross(from_b, angular_b), steps), sub(b_point, a_point));
    let error = sub(moved, mul(cross(from_a, angular_a), steps));
    let list = e.mem.u32(info + 0x68);
    if list != 0 {
        let squared = sum3(mul(error, error));
        let margin = e.mem.f32(info + 0x50) - squared;
        if margin.to_bits() >> 31 != 0 {
            let old = e.call(INTERLOCKED_EXCHANGE_ADD, &args![list, 1u32]).u32();
            if old < 0x80 {
                let id = e.mem.u32(info + 0x60);
                e.mem.set_u32(list + 4 + old * 4, id);
            }
        }
    }
    let max_float: f64 = e.global(MAX_FLOAT);
    for axis in 0..3u32 {
        axis_record(
            e,
            info,
            a.addr(),
            b.addr(),
            record,
            AXIS_TABLE + 0x10 * axis,
        );
        let scaled = (f64::from(error[axis as usize]) * f64::from(e.mem.f32(info + 0x20))) as f32;
        e.mem.set_f32(record + 0xc, scaled);
        add_epsilon(e, record);
        if f64::from(limit) == max_float {
            e.mem.set_u8(record, 5);
        } else {
            e.mem.set_u8(record, 6);
            e.mem.set_f32(record + 0x2c, x87_quiet(limit));
        }
        record += 0x30;
    }
    e.mem.set_u32(cursor.addr(), record);
}

/// The unnamed rotate function (`004b4ab0`) on a body's rotation rows and angular velocity.
fn rotated_angular(e: &mut Engine, body: u32) -> Vec4 {
    e.with_stack(0x10, |e, out| {
        e.call(ROTATE_ANGULAR, &args![out, body + 0x50, body + 0x20]);
        load_vec(e, out.addr())
    })
}

// Translated from 00d6c970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Three linear records without an error term (no engine-map name; cdecl: the two world points,
/// the info block, the output record): for each axis of the table at `01268370` the same
/// `axis_record` as `fn_00d6c630` (axis, angular terms, effective mass, `slope / mass`,
/// `(b - a) . axis * weight`), records 0x30 bytes apart, with no tag byte.
pub fn fn_00d6c970(e: &mut Engine, a: Ptr, b: Ptr, info: Ptr, record: Ptr) {
    for axis in 0..3u32 {
        axis_record(
            e,
            info.addr(),
            a.addr(),
            b.addr(),
            record.addr() + 0x30 * axis,
            AXIS_TABLE + 0x10 * axis,
        );
    }
}

// Translated from 00d6cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular record (no engine-map name; cdecl: the input block `vector`, the info block and
/// the cursor). The input block holds a vector at `+0` and floats at `+0x10`, `+0x14`, `+0x18`
/// and `+0x1c` (the last is `k`). Writes `vector` carried through body A's rows at `+0` and
/// `-vector` through body B's at `+0x10`; `+0xc` is `k / (mass + FLT_EPSILON)`; with
/// `m = -info[2]`: `+0x1c` is `input[6] * m`, `+0x20` is `input[4] * m * k`, `+0x24` is
/// `m * input[5] * k` and `+0x28` is `k`; the first float gets `FLT_EPSILON` added and the tag
/// byte 0xd. The cursor moves on by 0x30.
pub fn fn_00d6cb30(e: &mut Engine, input: Ptr, info: Ptr, cursor: Ptr) {
    let (input, info) = (input.addr(), info.addr());
    let k = x87_quiet(e.mem.f32(input + 0x1c));
    let record = e.mem.u32(cursor.addr());
    let sum = angular_pair(e, info, input, record);
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    e.mem
        .set_f32(record + 0xc, (f64::from(k) / f64::from(total)) as f32);
    let m = -x87_quiet(e.mem.f32(info + 8));
    let fourth = x87_quiet(e.mem.f32(input + 0x10));
    let fifth = x87_quiet(e.mem.f32(input + 0x14));
    let sixth = e.mem.f32(input + 0x18);
    e.mem
        .set_f32(record + 0x1c, (f64::from(sixth) * f64::from(m)) as f32);
    e.mem.set_u32(cursor.addr(), record + 0x30);
    let first = (f64::from(fourth) * f64::from(m)) * f64::from(k);
    e.mem.set_f32(record + 0x20, first as f32);
    let second = (f64::from(m) * f64::from(fifth)) * f64::from(k);
    e.mem.set_f32(record + 0x24, second as f32);
    e.mem.set_f32(record + 0x28, k);
    add_epsilon(e, record);
    e.mem.set_u8(record, 0xd);
}

// Translated from 00d6cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular-velocity-motor begin step with registers (no engine-map name): `info` (ECX), the
/// output `record` (EDX), `vector` (ESI) and the stack word `out`. Same body as
/// `hk1d_angular_velocity_motor_begin_jacobian`: the angular pair at `record + 0`/`+0x10`, then
/// `1 / (sum + FLT_EPSILON)` rounded to `float` at `record + 0xc` and at `*out`.
pub fn fn_00d6cc80(e: &mut Engine, info: Ptr, record: Ptr, vector: Ptr, out: Ptr) {
    angular_motor_begin(e, info.addr(), vector.addr(), record.addr(), out.addr());
}

fn angular_motor_begin(e: &mut Engine, info: u32, vector: u32, record: u32, out: u32) {
    let sum = angular_pair(e, info, vector, record);
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    let inverse = (1.0f64 / f64::from(total)) as f32;
    e.mem.set_f32(record + 0xc, inverse);
    e.mem.set_f32(out, inverse);
}

// Translated from 00d6cd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dAngularVelocityMotorBeginJacobian` (engine map; cdecl: the axis vector, the info block,
/// the output record and a float output): the angular pair at `record + 0`/`+0x10`, then
/// `1 / (sum + FLT_EPSILON)` rounded to `float` at `record + 0xc` and at `*out`.
pub fn hk1d_angular_velocity_motor_begin_jacobian(
    e: &mut Engine,
    vector: Ptr,
    info: Ptr,
    record: Ptr,
    out: Ptr,
) {
    angular_motor_begin(e, info.addr(), vector.addr(), record.addr(), out.addr());
}

// Translated from 00d6ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dAngularVelocityMotorCommitJacobian` (engine map; cdecl: the motor block, the motor
/// state, the begin record and the cursor): `begin + 0x1c = state[2] * block[0]`; at the cursor
/// a new record with the tag 0xf and `FLT_EPSILON` added to its first float, `+0x20 = block[2] *
/// state[1]`, `+0x24 = block[3] * state[1]`, `+0x28`, `+0x2c` and `+0x30` the floats
/// `block[1]`, `block[4]` and `block[5]` (copied through the x87 stack). The cursor moves on
/// by 0x40. The first word of the new record keeps the stream's earlier bytes apart from the
/// tag.
pub fn hk1d_angular_velocity_motor_commit_jacobian(
    e: &mut Engine,
    block: Ptr,
    state: Ptr,
    begin: Ptr,
    cursor: Ptr,
) {
    let (block, state) = (block.addr(), state.addr());
    let product = f64::from(e.mem.f32(state + 8)) * f64::from(e.mem.f32(block));
    e.mem.set_f32(begin.addr() + 0x1c, product as f32);
    let record = e.mem.u32(cursor.addr());
    add_epsilon(e, record);
    e.mem.set_u8(record, 0xf);
    e.mem.set_u32(cursor.addr(), record + 0x40);
    let scale = f64::from(e.mem.f32(state + 4));
    e.mem.set_f32(
        record + 0x20,
        (f64::from(e.mem.f32(block + 8)) * scale) as f32,
    );
    e.mem.set_f32(
        record + 0x24,
        (f64::from(e.mem.f32(block + 0xc)) * scale) as f32,
    );
    for (to, from) in [(0x28, 4), (0x2c, 0x10), (0x30, 0x14)] {
        let value = x87_quiet(e.mem.f32(block + from));
        e.mem.set_f32(record + to, value);
    }
}

/// Builds the input block of a one-dimensional linear builder on the stack and calls `callee`
/// with it and the two stack words (`fn_00d6d220`, `fn_00d6d280`). The block: `+0` the vector at
/// `other + 0x30`, `+0x10` the vector at `bodies + 0x30`, `+0x20` the vector at
/// `bodies + (atom.byte[2] << 4)`, `+0x30` and `+0x34` the floats at `atom + 4` and `atom + 8`
/// (copied through the x87 stack).
fn linear_input_block(
    e: &mut Engine,
    atom: u32,
    bodies: u32,
    other: u32,
    callee: u32,
    tail: [u32; 2],
) {
    e.with_stack(0x40, |e, block| {
        let block = block.addr();
        let first = load_vec(e, other + 0x30);
        store_vec(e, block, first);
        let second = load_vec(e, bodies + 0x30);
        store_vec(e, block + 0x10, second);
        let axis = u32::from(e.mem.u8(atom + 2)) << 4;
        let third = load_vec(e, bodies + axis);
        store_vec(e, block + 0x20, third);
        let tau = x87_quiet(e.mem.f32(atom + 4));
        e.mem.set_f32(block + 0x30, tau);
        let limit = x87_quiet(e.mem.f32(atom + 8));
        e.mem.set_f32(block + 0x34, limit);
        e.call(callee, &args![block, tail[0], tail[1]]);
    });
}

// Translated from 00d6d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A user-tau bilateral linear builder (no engine-map name). Registers: the atom (EAX) and the
/// body pair (ECX); then the stack words `a`, `other` and `b`. Builds the 0x38-byte input block
/// (see `linear_input_block`) and calls `hk1dLinearBilateralConstraintUserTauBuildJacobian`
/// with the block, `a` and `b`.
pub fn fn_00d6d220(e: &mut Engine, atom: Ptr, bodies: Ptr, a: u32, other: Ptr, b: u32) {
    linear_input_block(
        e,
        atom.addr(),
        bodies.addr(),
        other.addr(),
        LINEAR_BILATERAL_USER_TAU,
        [a, b],
    );
}

// Translated from 00d6d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A linear-limit builder (no engine-map name), the same input block as `fn_00d6d220`.
/// Registers: the atom (EAX) and the body pair (ECX); then the stack words `a`, `other` and
/// `b`. Calls `hk1dLinearLimitBuildJacobian` with the block, `a` and `b`.
pub fn fn_00d6d280(e: &mut Engine, atom: Ptr, bodies: Ptr, a: u32, other: Ptr, b: u32) {
    linear_input_block(
        e,
        atom.addr(),
        bodies.addr(),
        other.addr(),
        LINEAR_LIMIT,
        [a, b],
    );
}

// Translated from 00d6d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular friction record (no engine-map name). Registers: the atom (ECX), the cursor
/// pointer (EDX) and the info block (ESI); stack word: the rows base. When `atom.byte[2]` is
/// not 0 and the float at `atom + 8` is not 0 (a NaN counts as not 0), calls
/// `hk1dAngularFrictionBuildJacobian` with a stack block `{rows + (atom.byte[3] << 4), info
/// [0x64], friction, atom.byte[4]}`, the info block and the cursor pointer. Otherwise appends a
/// record: the byte `atom.byte[4]` at `+4`, `FLT_EPSILON` added to the first float and the tag
/// 3; the cursor moves on by 0x10.
pub fn fn_00d6d990(e: &mut Engine, atom: Ptr, cursor: Ptr, info: Ptr, rows: u32) {
    let atom = atom.addr();
    if e.mem.u8(atom + 2) != 0 && 0.0 != f64::from(e.mem.f32(atom + 8)) {
        e.with_stack(0x10, |e, block| {
            let base = rows.wrapping_add(u32::from(e.mem.u8(atom + 3)) << 4);
            e.mem.set_u32(block.addr(), base);
            let friction = x87_quiet(e.mem.f32(atom + 8));
            e.mem.set_f32(block.addr() + 8, friction);
            let group = u32::from(e.mem.u8(atom + 4));
            e.mem.set_u32(block.addr() + 0xc, group);
            let word = e.mem.u32(info.addr() + 0x64);
            e.mem.set_u32(block.addr() + 4, word);
            e.call(ANGULAR_FRICTION, &args![block, info, cursor]);
        });
        return;
    }
    let record = e.mem.u32(cursor.addr());
    let group = e.mem.u8(atom + 4);
    e.mem.set_u8(record + 4, group);
    add_epsilon(e, record);
    e.mem.set_u8(record, 3);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

/// The one-constraint-header record of `fn_00d6f2b0` and `fn_00d6ea30`: the first float gets
/// `FLT_EPSILON` added, `info.word[0x70]` (low 16 bits) goes to `+6`, `info.word[0x6c]` (low 16)
/// to `+4`, `info.word[0x60]` to `+0xc`, `value` to `+8`, `flag` to `+1` and the tag 1 to `+0`.
fn header_record(e: &mut Engine, info: u32, cursor: Ptr, value: f32, flag: u8) {
    let record = e.mem.u32(cursor.addr());
    add_epsilon(e, record);
    let high = e.mem.u32(info + 0x70) as u16;
    e.mem.set_u16(record + 6, high);
    let id = e.mem.u32(info + 0x60);
    e.mem.set_u32(record + 0xc, id);
    let low = e.mem.u32(info + 0x6c) as u16;
    e.mem.set_u16(record + 4, low);
    e.mem.set_u32(record + 8, value.to_bits());
    e.mem.set_u8(record, 1);
    e.mem.set_u8(record + 1, flag);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

// Translated from 00d6f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A header record (no engine-map name; cdecl: the info block, the cursor pointer, a float and
/// a byte). See `header_record`. The float word is copied as bits.
pub fn fn_00d6f2b0(e: &mut Engine, info: Ptr, cursor: Ptr, value: f32, flag: u8) {
    header_record(e, info.addr(), cursor, value, flag);
}

// Translated from 00d6ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same header record as `fn_00d6f2b0`, with the info block (ECX) and the cursor pointer
/// (EDX) in registers and the float and the byte on the stack.
pub fn fn_00d6ea30(e: &mut Engine, info: Ptr, cursor: Ptr, value: f32, flag: u8) {
    header_record(e, info.addr(), cursor, value, flag);
}

/// `hkMath_atan2fApproximation` (engine map; cdecl: two floats, the result in ST0).
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkCalcMotorData` (engine map; cdecl: the motor, an input block, an output block).
const CALC_MOTOR_DATA: u32 = 0x00dc_fff0;
/// The `float` 1.1920929e-07 the axis builders compare a squared length with.
const SMALL_FLOAT: u32 = 0x0104_da4c;
/// Two byte tables indexed by an axis number (the offsets of the other two axes in a row of
/// 16-byte vectors).
const NEXT_AXIS_TABLE: u32 = 0x010d_33c1;
const THIRD_AXIS_TABLE: u32 = 0x010d_33c2;
/// The `double`s -1.0, 0.5, pi and 2 pi (the exe's truncated values) the angle code uses.
const MINUS_ONE: u32 = 0x0101_a6b0;
const HALF: u32 = 0x0101_1588;
const PI: u32 = 0x0101_ff40;
const TWO_PI: u32 = 0x0101_ff48;
/// The `double` 1e-16 below which a summed axis is taken as zero (`fn_00d6d450`).
const TINY: u32 = 0x010d_33d0;
/// The `double` the angle limit is scaled by in `fn_00d6d720`.
const LIMIT_SCALE: u32 = 0x0103_c7c8;
/// `float` tables indexed by a byte of the atom: the multiplier (`+4` per entry from
/// `010d33e0`), the offset (`010d33d8`) and the axis sign (`010d33e4` going down).
const ANGLE_SCALE_TABLE: u32 = 0x010d_33e0;
const ANGLE_OFFSET_TABLE: u32 = 0x010d_33d8;
const AXIS_SIGN_TABLE: u32 = 0x010d_33e4;

/// Moves `angle` (a `float`) by whole turns of 2 pi until it lies within pi of `limit`, the way
/// the two x87 loops do: each step is added or subtracted in double precision and stored as a
/// `float`; a comparison with NaN is false.
fn wrap_angle(e: &Engine, angle: f32, limit: f32) -> f32 {
    let pi: f64 = e.global(PI);
    let two_pi: f64 = e.global(TWO_PI);
    let limit = f64::from(limit);
    let mut angle = angle;
    if pi < limit - f64::from(angle) {
        loop {
            angle = (f64::from(angle) + two_pi) as f32;
            let again = limit - f64::from(angle) > pi;
            if !again {
                break;
            }
        }
    }
    if f64::from(angle) - limit > pi {
        loop {
            angle = (f64::from(angle) - two_pi) as f32;
            let again = f64::from(angle) - limit > pi;
            if !again {
                break;
            }
        }
    }
    angle
}

// Translated from 00d6d2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular-limit record (no engine-map name; cdecl). Registers: the atom (ECX) and the
/// output record (ESI); stack words: the info block, two bases of rows of 16-byte vectors.
/// Returns 1 when the atom's byte `+2` is 0 and writes nothing. Otherwise (`axis = atom.byte[3]`)
/// the record gets row `axis` of the first base at `+0`, the atom's floats `+4` and `+8` at
/// `+0x10` and `+0x14`, the angle between the two bases about that axis
/// (`hkMath_atan2fApproximation(dot, cross . row)`, with the other two axes taken from the
/// byte tables at `010d33c1` and `010d33c2`) at `+0x18`, moved by whole turns to within pi of
/// `-info[0x64][1]` (the limit), and `info.float[0x40] * atom.float[3]` at `+0x1c`. Returns 0.
/// (The turn loops run forever on an infinite angle, as in the exe.)
pub fn fn_00d6d2e0(
    e: &mut Engine,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
) -> u32 {
    let (atom, record, info) = (atom.addr(), record.addr(), info.addr());
    if e.mem.u8(atom + 2) == 0 {
        return 1;
    }
    let axis = u32::from(e.mem.u8(atom + 3));
    let first = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis));
    let second = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis));
    let product =
        f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(x87_quiet(e.mem.f32(atom + 0xc)));
    e.mem.set_f32(record + 0x1c, product as f32);
    let axis_row = axis << 4;
    let row = load_vec(e, rows_a.addr() + axis_row);
    store_vec(e, record, row);
    let other_a = load_vec(e, rows_a.addr() + first);
    let tau = x87_quiet(e.mem.f32(atom + 4));
    e.mem.set_f32(record + 0x10, tau);
    let other_b = load_vec(e, rows_b.addr() + second);
    let axis_b = load_vec(e, rows_b.addr() + axis_row);
    let limit_value = x87_quiet(e.mem.f32(atom + 8));
    e.mem.set_f32(record + 0x14, limit_value);
    let dot = sum3(mul(other_b, other_a));
    let turned = sum3(mul(cross(other_b, axis_b), other_a));
    let angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(dot).to_bits(), x87_quiet(turned).to_bits()],
        )
        .f32();
    let limits = e.mem.u32(info + 0x64);
    let scale: f64 = e.global(MINUS_ONE);
    let limit = (f64::from(x87_quiet(e.mem.f32(limits + 4))) * scale) as f32;
    let wrapped = wrap_angle(e, angle, limit);
    e.mem.set_f32(record + 0x18, wrapped);
    0
}

// Translated from 00d6d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular record about the sum of two rows (no engine-map name; cdecl). Registers: the
/// first base (EDX), the atom (ESI) and the output record (EDI); stack words: the info block
/// and the second base. With `axis = atom.byte[3]` and `other = atom.byte[4]`, `n` is the
/// row `axis` of the second base added to that of the first, normalised when its length is
/// above `1e-16` (and then `half` is half the length; otherwise `n` is the second base's row
/// itself and `half` is 0). The record gets row `axis` of the first base at `+0` (replaced
/// by `n` at the end), the atom's floats `+8` and `+0xc` at `+0x10` and `+0x14`, the angle
/// `hkMath_atan2fApproximation(a . t, a . (t x n))` with `t = n x b` (`a` and `b` the row
/// `other` of the two bases) at `+0x18` and `half * half * info.float[0x40] *
/// atom.float[4]` at `+0x1c`. Returns 0.
pub fn fn_00d6d450(
    e: &mut Engine,
    base_a: Ptr,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    base_b: Ptr,
) -> u32 {
    let (base_a, atom, record, info, base_b) = (
        base_a.addr(),
        atom.addr(),
        record.addr(),
        info.addr(),
        base_b.addr(),
    );
    let axis_row = u32::from(e.mem.u8(atom + 3)) << 4;
    let first = load_vec(e, base_a + axis_row);
    store_vec(e, record, first);
    let tau = x87_quiet(e.mem.f32(atom + 8));
    e.mem.set_f32(record + 0x10, tau);
    let limit_value = x87_quiet(e.mem.f32(atom + 0xc));
    e.mem.set_f32(record + 0x14, limit_value);
    let axis_row = u32::from(e.mem.u8(atom + 3)) << 4;
    let other_row = u32::from(e.mem.u8(atom + 4)) << 4;
    let axis_b = load_vec(e, base_b + axis_row);
    let axis_a = load_vec(e, base_a + axis_row);
    let sum = add(axis_a, axis_b);
    let length = sum3(mul(sum, sum)).sqrt();
    let tiny: f64 = e.global(TINY);
    let (half, normal) = if f64::from(x87_quiet(length)) > tiny {
        let half_constant: f64 = e.global(HALF);
        let half = (half_constant * f64::from(length)) as f32;
        let inverse = (1.0f64 / f64::from(length)) as f32;
        (half, mul([inverse; 4], sum))
    } else {
        (0.0f32, axis_b)
    };
    let other_b = load_vec(e, base_b + other_row);
    let tangent = cross(normal, other_b);
    let binormal = cross(tangent, normal);
    let other_a = load_vec(e, base_a + other_row);
    let along = sum3(mul(other_a, tangent));
    let across = sum3(mul(other_a, binormal));
    let angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(along).to_bits(), x87_quiet(across).to_bits()],
        )
        .f32();
    e.mem.set_f32(record + 0x18, angle);
    store_vec(e, record, normal);
    let weight =
        f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(x87_quiet(e.mem.f32(atom + 0x10)));
    let half = f64::from(x87_quiet(half));
    e.mem
        .set_f32(record + 0x1c, ((weight * half) * half) as f32);
    0
}

/// The common part of `fn_00d6d5d0` and `fn_00d6d720`: the axes `a` (row `atom.byte[3]` of the
/// first base) and `b` (row `atom.byte[4]` of the second); returns `None` (the functions return
/// 1) when the squared length of `a x b` is below `FLT_EPSILON`, else writes the record and
/// returns the angle written at `+0x18`: `atan2(|a x b|, a . b)` scaled and offset by the
/// tables at `010d33e0` and `010d33d8` (index `atom.byte[5]`), with `(a x b) / |a x b|` times
/// the axis sign (`010d33e4`, going down) at `+0`, the atom's floats `+8` and `+0xc` at
/// `+0x10` and `+0x14` and `info.float[0x40] * atom.float[4]` at `+0x1c`.
fn axis_angle_record(
    e: &mut Engine,
    atom: u32,
    record: u32,
    info: u32,
    rows_a: u32,
    rows_b: u32,
) -> Option<f32> {
    let a_row = (u32::from(e.mem.u8(atom + 3)) << 4).wrapping_add(rows_a);
    let b_row = (u32::from(e.mem.u8(atom + 4)) << 4).wrapping_add(rows_b);
    let a = load_vec(e, a_row);
    let b = load_vec(e, b_row);
    let turned = cross(a, b);
    let squared = sum3(mul(turned, turned));
    let small: f32 = e.global(SMALL_FLOAT);
    if x87_quiet(squared) < x87_quiet(small) {
        return None;
    }
    let index = u32::from(e.mem.u8(atom + 5));
    let dot = sum3(mul(b, a));
    let length = e
        .call(X87_SQRT, &args![f64::from(x87_quiet(squared))])
        .f32();
    let angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(length).to_bits(), x87_quiet(dot).to_bits()],
        )
        .f64();
    let scale = f64::from(e.global::<f32>(ANGLE_SCALE_TABLE + 4 * index));
    let offset = f64::from(e.global::<f32>(ANGLE_OFFSET_TABLE + 4 * index));
    let value = (angle * scale + offset) as f32;
    let sign = x87_quiet(e.global::<f32>(AXIS_SIGN_TABLE.wrapping_sub(4 * index)));
    let weight =
        f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(x87_quiet(e.mem.f32(atom + 0x10)));
    e.mem.set_f32(record + 0x1c, weight as f32);
    let inverse = (1.0f64 / f64::from(x87_quiet(length))) as f32;
    e.mem.set_f32(record + 0x18, x87_quiet(value));
    let scaled = mul(mul([inverse; 4], turned), [sign; 4]);
    store_vec(e, record, scaled);
    let tau = x87_quiet(e.mem.f32(atom + 8));
    e.mem.set_f32(record + 0x10, tau);
    let limit_value = x87_quiet(e.mem.f32(atom + 0xc));
    e.mem.set_f32(record + 0x14, limit_value);
    Some(value)
}

// Translated from 00d6d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular record from two axes (no engine-map name; cdecl). Registers: the atom (ESI) and
/// the output record (EDI); stack words: the info block and the two bases. See
/// `axis_angle_record`; returns 1 for parallel axes and otherwise 0.
pub fn fn_00d6d5d0(
    e: &mut Engine,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
) -> u32 {
    let written = axis_angle_record(
        e,
        atom.addr(),
        record.addr(),
        info.addr(),
        rows_a.addr(),
        rows_b.addr(),
    );
    u32::from(written.is_none())
}

// Translated from 00d6d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same record as `fn_00d6d5d0` followed by a limit update (no engine-map name; cdecl; the
/// same registers and stack words). Returns 1 for parallel axes, 0 when `atom.byte[6] <= 1`
/// (nothing more), else, with `limits = info.word[0x19]` and `slot = atom.byte[6]`: with the
/// angle `v` just written, `a = limits[slot]`, `c = -limits[1]`, `m = info.float[3]`, the new
/// bound `M = max(0.5 * m, a * K)` (`K` the double at `0103c7c8`; NaN picks `0.5 * m`) and the
/// differences `x = v - a` and `y = c - x`; `limits[slot]` becomes `a - M` (but at least
/// `FLT_EPSILON`) and the float at `+0x18` of the record `x + M`, both reduced by `y` /
/// raised by it when `0.5 * (0.5 * m + 2 M) < -y` and `c` is not 0.
pub fn fn_00d6d720(
    e: &mut Engine,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
) -> u32 {
    let (atom, record, info) = (atom.addr(), record.addr(), info.addr());
    let Some(angle) = axis_angle_record(e, atom, record, info, rows_a.addr(), rows_b.addr()) else {
        return 1;
    };
    let slot = u32::from(e.mem.u8(atom + 6));
    if slot <= 1 {
        return 0;
    }
    let limits = e.mem.u32(info + 0x64);
    let a = x87_quiet(e.mem.f32(limits + slot));
    let c = -x87_quiet(e.mem.f32(limits + 4));
    let x = (f64::from(angle) - f64::from(a)) as f32;
    let y = (f64::from(c) - f64::from(x)) as f32;
    let half_constant: f64 = e.global(HALF);
    let m = x87_quiet(e.mem.f32(info + 0xc));
    let half_m = (half_constant * f64::from(m)) as f32;
    let scale: f64 = e.global(LIMIT_SCALE);
    let a_scaled = (f64::from(a) * scale) as f32;
    let bound = if half_m < a_scaled { a_scaled } else { half_m };
    let mut low = (f64::from(a) - f64::from(bound)) as f32;
    let mut high = (f64::from(x) + f64::from(bound)) as f32;
    let sum = f64::from(half_m) + (f64::from(bound) + f64::from(bound));
    let blend = (half_constant * sum) as f32;
    if blend < -y && f64::from(c) != 0.0 {
        low = (f64::from(low) - f64::from(y)) as f32;
        high = (f64::from(y) + f64::from(high)) as f32;
    }
    let small: f32 = e.global(SMALL_FLOAT);
    low = if x87_quiet(small) < low {
        low
    } else {
        x87_quiet(small)
    };
    e.mem.set_f32(limits + slot, low);
    e.mem.set_f32(record + 0x18, high);
    0
}

/// Asks `hkCalcMotorData` for the six floats of a motor: the input block is `{inverse, info,
/// limits[0], limits[1], remaining, passed}` (the limits as raw words) and the output has six
/// words, of which the code reads `1..=5`.
fn calc_motor_data(e: &mut Engine, motor: u32, block: [u32; 6]) -> [f32; 6] {
    e.with_stack(0x18, |e, input| {
        for (index, word) in block.into_iter().enumerate() {
            e.mem.set_u32(input.addr() + 4 * index as u32, word);
        }
        e.with_stack(0x18, |e, output| {
            e.call(CALC_MOTOR_DATA, &args![motor, input, output]);
            std::array::from_fn(|word| e.mem.f32(output.addr() + 4 * word as u32))
        })
    })
}

/// Completes the 0x40-byte tag-15 record at the cursor from the motor data: `begin + 0x1c`
/// becomes `info.float[2] * out[3]`; the record gets `FLT_EPSILON` added to its first float,
/// the tag 15, `out[2]` and `out[3]` times `info.float[1]` at `+0x20` and `+0x24` and `out[1]`,
/// `out[4]` and `out[5]` at `+0x28`, `+0x2c` and `+0x30`; the cursor moves on by 0x40.
fn commit_motor_record(e: &mut Engine, info: u32, cursor: Ptr, begin: u32, out: [f32; 6]) {
    let weight = f64::from(x87_quiet(e.mem.f32(info + 8))) * f64::from(out[3]);
    e.mem.set_f32(begin + 0x1c, weight as f32);
    let record = e.mem.u32(cursor.addr());
    add_epsilon(e, record);
    e.mem.set_u8(record, 0xf);
    let tau = f64::from(x87_quiet(e.mem.f32(info + 4)));
    e.mem
        .set_f32(record + 0x20, (f64::from(out[2]) * tau) as f32);
    e.mem
        .set_f32(record + 0x24, (f64::from(out[3]) * tau) as f32);
    e.mem.set_u32(cursor.addr(), record + 0x40);
    e.mem.set_f32(record + 0x28, x87_quiet(out[1]));
    e.mem.set_f32(record + 0x2c, x87_quiet(out[4]));
    e.mem.set_f32(record + 0x30, x87_quiet(out[5]));
}

/// The motor atoms' placeholder record: `{tag 3, byte[4] = marker}` and the cursor on by 0x10.
fn placeholder_record(e: &mut Engine, cursor: Ptr, marker: u8) {
    let record = e.mem.u32(cursor.addr());
    e.mem.set_u8(record + 4, marker);
    add_epsilon(e, record);
    e.mem.set_u8(record, 3);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

// Translated from 00d6da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromAngMotorAtom` (engine map; cdecl). Registers: the info block (ESI) and
/// the atom (EDI); stack words: two bases of rows of 16-byte vectors and the cursor pointer.
/// `flag` and `value` are the byte and the float at `info.word[0x19] + atom.short[2]` and
/// `+ atom.short[3]`. Without a motor (`atom.word[4]`) or with the atom's byte `+2` 0 it
/// appends the placeholder record (`{tag 3, byte[4] = 1}`). Otherwise it takes the angle of the
/// two bases about row `atom.byte[3]` (`hkMath_atan2fApproximation(a . b, a . (b x c))`),
/// moved by whole turns to within pi of `-info.float[atom.short[4] + 4]` when that offset is
/// not 0 (`wrap_angle`), stores it in `value` when `flag` is 0, makes the angular pair and its
/// inverse effective mass in the record at the cursor, asks `hkCalcMotorData` (the motor,
/// `{1 / mass, info, limits[0], limits[1], atom.float[3] - value, value - angle}`) and
/// completes the 0x40-byte record from the answer (`commit_motor_record`); `value` and `flag`
/// are set to `atom.float[3]` and 1 when the two floats differ or `flag` is 0.
pub fn build_jacobian_from_ang_motor_atom(
    e: &mut Engine,
    info: Ptr,
    atom: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
    cursor: Ptr,
) {
    let (info, atom) = (info.addr(), atom.addr());
    let limits = e.mem.u32(info + 0x64);
    let flag = limits.wrapping_add(e.mem.u16(atom + 4) as i16 as i32 as u32);
    let value = limits.wrapping_add(e.mem.u16(atom + 6) as i16 as i32 as u32);
    let motor = e.mem.u32(atom + 0x10);
    if motor == 0 || e.mem.u8(atom + 2) == 0 {
        placeholder_record(e, cursor, 1);
        return;
    }
    let axis = u32::from(e.mem.u8(atom + 3));
    let first = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis)) + rows_a.addr();
    let second = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis)) + rows_b.addr();
    let a = load_vec(e, first);
    let b = load_vec(e, second);
    let dot = sum3(mul(a, b));
    let c = load_vec(e, (axis << 4) + rows_b.addr());
    let turned = sum3(mul(cross(b, c), a));
    let mut angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(dot).to_bits(), x87_quiet(turned).to_bits()],
        )
        .f32();
    let offset = e.mem.u16(atom + 8);
    if offset != 0 {
        let at = limits.wrapping_add(offset as i16 as i32 as u32);
        let scale: f64 = e.global(MINUS_ONE);
        let bound = (f64::from(x87_quiet(e.mem.f32(at + 4))) * scale) as f32;
        angle = wrap_angle(e, angle, bound);
    }
    if e.mem.u8(flag) == 0 {
        e.mem.set_f32(value, x87_quiet(angle));
    }
    let record = e.mem.u32(cursor.addr());
    let axis_vector = (axis << 4) + rows_a.addr();
    let inverse = angular_inverse(e, info, axis_vector, record);
    let bounds = [e.mem.u32(limits), e.mem.u32(limits + 4)];
    let target = x87_quiet(e.mem.f32(atom + 0xc));
    let current = x87_quiet(e.mem.f32(value));
    let remaining = (f64::from(target) - f64::from(current)) as f32;
    let current = x87_quiet(e.mem.f32(value));
    let passed = (f64::from(current) - f64::from(angle)) as f32;
    let block = [
        inverse.to_bits(),
        info,
        bounds[0],
        bounds[1],
        remaining.to_bits(),
        passed.to_bits(),
    ];
    let out = calc_motor_data(e, motor, block);
    commit_motor_record(e, info, cursor, record, out);
    let now = e.mem.f32(value);
    let wanted = e.mem.f32(atom + 0xc);
    // `FUCOMPP`: only equal, ordered values keep the update away.
    if now != wanted || e.mem.u8(flag) == 0 {
        e.mem.set_f32(value, x87_quiet(wanted));
        e.mem.set_u8(flag, 1);
    }
}

/// The angular pair of `vector` into `record`, then `1 / (sum + FLT_EPSILON)` rounded to a
/// `float` stored at `record + 0xc` and returned (the begin step of the motor builders).
fn angular_inverse(e: &mut Engine, info: u32, vector: u32, record: u32) -> f32 {
    let sum = angular_pair(e, info, vector, record);
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    let inverse = (1.0f64 / f64::from(x87_quiet(total))) as f32;
    e.mem.set_f32(record + 0xc, inverse);
    inverse
}

/// `hkMatrix3::setMulMat3Mat3` (engine map; cdecl: the result matrix, two matrices).
const SET_MUL_MAT3_MAT3: u32 = 0x00cd_5530;

/// The unit axis of a ragdoll cone limit: `sum` times `1 / |sum|` when the length of its first
/// three lanes is above `1e-16`, else `fallback`.
fn limit_axis(e: &Engine, sum: Vec4, fallback: Vec4) -> Vec4 {
    let length = x87_quiet(sum3(mul(sum, sum)).sqrt());
    let tiny: f64 = e.global(TINY);
    if f64::from(length) > tiny {
        let inverse = (1.0f64 / f64::from(length)) as f32;
        mul([inverse; 4], sum)
    } else {
        fallback
    }
}

/// `hkMath_atan2fApproximation(t . v, (t x axis) . v)` with `t = axis x w`, the angle of the
/// vector `v` about `axis` measured from `w`.
fn angle_about(e: &mut Engine, axis: Vec4, w: Vec4, v: Vec4) -> f32 {
    let tangent = cross(axis, w);
    let binormal = cross(tangent, axis);
    let along = sum3(mul(tangent, v));
    let across = sum3(mul(binormal, v));
    e.call(
        ATAN2_APPROXIMATION,
        &args![x87_quiet(along).to_bits(), x87_quiet(across).to_bits()],
    )
    .f32()
}

// Translated from 00d6dd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromRagdollMotorAtom` (engine map; cdecl). Registers: the atom (EAX) and the
/// info block (EDI); stack words: two bases of rows of 16-byte vectors and the cursor pointer.
/// With the atom's byte `+2` 0 it appends the placeholder record (`{tag 3, byte[4] = 3}`) and
/// stops. Otherwise it multiplies the info block's matrix (`info.word[0xf]`) and the atom's
/// matrix (`+0x10`) with `hkMatrix3::setMulMat3Mat3` and, for the three motors at `atom + 0x40`
/// (axis `p` = 0, 1, 2 and `q` = 1, 0, 0): a motor of 0 appends the placeholder record
/// (`{tag 3, byte[4] = 1}`); otherwise the angle about the axis of the rows `p` of the two
/// bases (`limit_axis`, `angle_about` with the rows `q`) is the first angle and, about the
/// axis of the product's row `p` added to the second base's, the second angle. The first
/// angle goes to `value[i]` when `flag[i]` is 0 (`flag` and `value` as for
/// `build_jacobian_from_ang_motor_atom`, one byte and one float further per motor). The
/// angular pair of the second axis and `hkCalcMotorData` (limits `info.word[0x19][2 i]`,
/// `[2 i + 1]`, the second angle minus `value[i]`, `value[i]` minus the first angle) make the
/// 0x40-byte record (`commit_motor_record`); `value[i]` is replaced by the second angle when
/// they differ.
pub fn build_jacobian_from_ragdoll_motor_atom(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    let limits = e.mem.u32(info + 0x64);
    let flags = limits.wrapping_add(e.mem.u16(atom + 4) as i16 as i32 as u32);
    let values = limits.wrapping_add(e.mem.u16(atom + 6) as i16 as i32 as u32);
    if e.mem.u8(atom + 2) == 0 {
        placeholder_record(e, cursor, 3);
        return;
    }
    let matrix_word = e.mem.u32(info + 0x3c);
    e.with_stack(0x30, |e, product| {
        e.call(SET_MUL_MAT3_MAT3, &args![product, matrix_word, atom + 0x10]);
        for motor_index in 0..3u32 {
            let axis = motor_index;
            let other = [1u32, 0, 0][motor_index as usize];
            let motor = e.mem.u32(atom + 0x40 + 4 * motor_index);
            if motor == 0 {
                placeholder_record(e, cursor, 1);
                continue;
            }
            let flag = flags + motor_index;
            let value = values + 4 * motor_index;
            let axis_row = axis << 4;
            let other_row = other << 4;
            let (rows_a, rows_b) = (rows_a.addr(), rows_b.addr());
            let b_axis = load_vec(e, rows_b + axis_row);
            let a_axis = load_vec(e, rows_a + axis_row);
            let first_axis = limit_axis(e, add(a_axis, b_axis), b_axis);
            let b_other = load_vec(e, rows_b + other_row);
            let a_other = load_vec(e, rows_a + other_row);
            let first_angle = angle_about(e, first_axis, b_other, a_other);
            let product_axis = load_vec(e, product.addr() + axis_row);
            let second_axis = limit_axis(e, add(product_axis, b_axis), b_axis);
            let product_other = load_vec(e, product.addr() + other_row);
            let second_angle = angle_about(e, second_axis, b_other, product_other);
            if e.mem.u8(flag) == 0 {
                e.mem.set_f32(value, x87_quiet(first_angle));
                e.mem.set_u8(flag, 1);
            }
            let record = e.mem.u32(cursor.addr());
            let inverse = e.with_stack(0x10, |e, vector| {
                store_vec(e, vector.addr(), second_axis);
                angular_inverse(e, info, vector.addr(), record)
            });
            let bounds = [
                e.mem.u32(limits + 8 * motor_index),
                e.mem.u32(limits + 8 * motor_index + 4),
            ];
            let current = x87_quiet(e.mem.f32(value));
            let remaining = (f64::from(second_angle) - f64::from(current)) as f32;
            let current = x87_quiet(e.mem.f32(value));
            let passed = (f64::from(current) - f64::from(first_angle)) as f32;
            let block = [
                inverse.to_bits(),
                info,
                bounds[0],
                bounds[1],
                remaining.to_bits(),
                passed.to_bits(),
            ];
            let out = calc_motor_data(e, motor, block);
            commit_motor_record(e, info, cursor, record, out);
            if second_angle != e.mem.f32(value) {
                e.mem.set_f32(value, x87_quiet(second_angle));
            }
        }
    });
}

/// `hk1dLinearFrictionBuildJacobian` (engine map; cdecl: an input block, the info block, the
/// cursor pointer).
const LINEAR_FRICTION: u32 = 0x00dc_f100;
/// `hkAddVelocityBuildJacobian` (engine map; cdecl: a vector, a flag, the cursor pointer).
const ADD_VELOCITY: u32 = 0x00dc_edd0;
/// Unnamed (cdecl: two vectors, the cursor pointer): appends a tag-25 record of the two vectors.
const PUSH_VECTOR_PAIR: u32 = 0x00dc_ed90;
/// `hkSetCenterOfMassBuildJacobian` (engine map; cdecl: two matrices, the cursor pointer).
const SET_CENTER_OF_MASS: u32 = 0x00dc_ee10;
/// Unnamed (thiscall, one stack word, `RET 4`): copies a body's solver data like
/// `fn_00d71f90`, but the vectors at `+0x10..+0x80` as 16-byte rows (bytes `1..4` and the word
/// at `+0xc` stay as they were).
const COPY_BODY_DATA: u32 = 0x00d7_2030;
/// `hk1dLinearVelocityMotorBeginJacobian` and `hk1dLinearVelocityMotorCommitJacobian` (engine
/// map; cdecl).
const LINEAR_VELOCITY_BEGIN: u32 = 0x00dd_0260;
const LINEAR_VELOCITY_COMMIT: u32 = 0x00dd_03c0;
/// `hkVector4::setRotatedDir` (thiscall: the result, a matrix, a vector), `hkVector4::setMul3`
/// (the same body), `hkMatrix3::setCrossSkewSymmetric` (thiscall: the matrix, a vector),
/// `hkMatrix3::setMulInverse` (thiscall: the result, two matrices) and the unnamed row scaler
/// (thiscall: the matrix, a vector whose first lane scales every row).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
const SET_MUL3: u32 = 0x00c7_f750;
const SET_CROSS_SKEW_SYMMETRIC: u32 = 0x00cd_4cc0;
const SET_MUL_INVERSE: u32 = 0x00cd_5610;
const SCALE_ROWS: u32 = 0x00cd_4c70;
/// The `float` (-1.0) the center-of-mass changer scales the rows with.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// Sign bits in the first three lanes only.
const SIGN_MASK_XYZ: u32 = 0x010c_4c00;

// Translated from 00d6e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A linear friction record (no engine-map name; cdecl). Registers: the atom (ECX), the info
/// block (EDX), the cursor pointer (ESI) and a base of 16-byte rows (EDI); stack word: the
/// bodies block. When the atom's byte `+2` is not 0 and its float `+4` is above 0 it calls
/// `hk1dLinearFrictionBuildJacobian` with a stack block `{bodies.vector[0x30], rows[atom.byte[3]],
/// atom.float[1], info.word[0x19]}`, the info block and the cursor pointer; otherwise it
/// appends the placeholder record (`{tag 3, byte[4] = 1}`).
pub fn fn_00d6e280(e: &mut Engine, atom: Ptr, info: Ptr, cursor: Ptr, rows: Ptr, bodies: Ptr) {
    let (atom, info) = (atom.addr(), info.addr());
    if e.mem.u8(atom + 2) != 0 && 0.0 < f64::from(e.mem.f32(atom + 4)) {
        let friction = x87_quiet(e.mem.f32(atom + 4));
        let body_vector = load_vec(e, bodies.addr() + 0x30);
        let row = (u32::from(e.mem.u8(atom + 3)) << 4).wrapping_add(rows.addr());
        let limits = e.mem.u32(info + 0x64);
        e.with_stack(0x30, |e, block| {
            let block_address = block.addr();
            store_vec(e, block_address, body_vector);
            let row_vector = load_vec(e, row);
            store_vec(e, block_address + 0x10, row_vector);
            e.mem.set_f32(block_address + 0x20, friction);
            e.mem.set_u32(block_address + 0x24, limits);
            e.call(LINEAR_FRICTION, &args![block, info, cursor]);
        });
        return;
    }
    placeholder_record(e, cursor, 1);
}

/// The 0x40-byte tag-11 record of `fn_00d6e300` and `fn_00d6e6b0` (a length constraint between
/// two points with an offset and a second weight: no engine-map name, so the identity is
/// unconfirmed). `points` are `p0` (the first body's arm), `p1` (the second body's arm), `p2`
/// and `p3` (the far ends). With `d1 = p2 - p0` and `d2 = p1 - p3` the unit directions `n1`
/// of `d1` and `n2` of `d2` (the first x axis, the second its negative, when the squared
/// length is not above `FLT_EPSILON`; the rsqrt-based `inverse_length` otherwise) fill `+0`
/// and `+0x10` (`scale * n2`); `+0x20` and `+0x30` are the arms' cross products rotated into the
/// two bodies, `+0x2c` is `info.float[9] / (mass terms)`, `+0xc` is
/// `((scale * |d2| + |d1|) - offset) * info.float[2]`. The first float gets `FLT_EPSILON` added,
/// the tag 11 is set and the cursor moves on by 0x40.
fn spring_record(
    e: &mut Engine,
    info: u32,
    cursor: Ptr,
    points: [Vec4; 4],
    offset: f32,
    scale: f32,
) {
    let [p0, p1, p2, p3] = points;
    let first_difference = sub(p2, p0);
    let second_difference = sub(p1, p3);
    let record = e.mem.u32(cursor.addr());
    let small = x87_quiet(e.global::<f32>(SMALL_FLOAT));
    let axis_x = load_vec(e, AXIS_TABLE);
    let first_squared = sum3(mul(first_difference, first_difference));
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let first = if small < first_squared {
        mul([inverse_length(e, first_squared); 4], first_difference)
    } else {
        axis_x
    };
    let second_squared = sum3(mul(second_difference, second_difference));
    let second = if small < second_squared {
        mul([inverse_length(e, second_squared); 4], second_difference)
    } else {
        negate(e, axis_x)
    };
    let info_weight = x87_quiet(e.mem.f32(info + 8));
    let info_numerator = x87_quiet(e.mem.f32(info + 0x24));
    let scaled = mul([scale; 4], second);
    store_vec(e, record + 0x10, scaled);
    store_vec(e, record, first);
    let arm_a = sub(p0, load_vec(e, body_a + 0x40));
    let arm_b = sub(p1, load_vec(e, body_b + 0x40));
    let turn_a = cross(arm_a, first);
    let turn_b = cross(scaled, arm_b);
    let rows_a = body_rows(e, body_a);
    let angular_a = rotate_rows(turn_a, rows_a);
    store_vec(e, record + 0x20, angular_a);
    let rows_b = body_rows(e, body_b);
    let angular_b = rotate_rows(turn_b, rows_b);
    store_vec(e, record + 0x30, angular_b);
    let inverse_a = load_vec(e, body_a + 0x30);
    let inverse_b = load_vec(e, body_b + 0x30);
    let terms = add(
        mul(mul(angular_a, angular_a), inverse_a),
        mul(mul(angular_b, angular_b), inverse_b),
    );
    let inertia = sum3(terms);
    let scale_squared = (f64::from(scale) * f64::from(scale)) as f32;
    let mass_a = e.mem.f32(body_a + 0x3c);
    let mass_b = e.mem.f32(body_b + 0x3c);
    let epsilon: f64 = e.global(EPSILON);
    let mass_term =
        ((f64::from(scale_squared) * f64::from(mass_b) + f64::from(mass_a)) + epsilon) as f32;
    let denominator = (f64::from(mass_term) + f64::from(inertia)) as f32;
    let first_length = sum3(mul(first_difference, first_difference)).sqrt();
    let second_length = sum3(mul(second_difference, second_difference)).sqrt();
    e.mem.set_f32(
        record + 0x2c,
        (f64::from(info_numerator) / f64::from(denominator)) as f32,
    );
    let reach = (f64::from(scale) * f64::from(second_length) + f64::from(first_length)) as f32;
    let excess = (f64::from(reach) - f64::from(offset)) as f32;
    e.mem.set_f32(
        record + 0xc,
        (f64::from(excess) * f64::from(info_weight)) as f32,
    );
    let next = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), next.wrapping_add(0x40));
    add_epsilon(e, next);
    e.mem.set_u8(next, 0xb);
}

// Translated from 00d6e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// A tag-11 length record from one input block (no engine-map name; cdecl). Register: the
/// input block (EDI): the points at `+0`, `+0x10`, `+0x20`, `+0x30` and the floats `+0x40`
/// (the offset) and `+0x44` (the scale); stack words: the info block and the cursor pointer.
/// See `spring_record`.
pub fn fn_00d6e300(e: &mut Engine, block: Ptr, info: Ptr, cursor: Ptr) {
    let block = block.addr();
    let points = [
        load_vec(e, block),
        load_vec(e, block + 0x10),
        load_vec(e, block + 0x20),
        load_vec(e, block + 0x30),
    ];
    let offset = x87_quiet(e.mem.f32(block + 0x40));
    let scale = x87_quiet(e.mem.f32(block + 0x44));
    spring_record(e, info.addr(), cursor, points, offset, scale);
}

// Translated from 00d6e6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same tag-11 length record from three input blocks (no engine-map name; cdecl).
/// Registers: `first` (EAX), `second` (ECX) and the info block (EDI); stack words: `third` and
/// the cursor pointer. The points are `second[0x30]`, `third[0x30]`, `first[0x10]` and
/// `first[0x20]`; the offset and the scale are the floats `first[0x30]` and `first[0x34]`.
pub fn fn_00d6e6b0(e: &mut Engine, first: Ptr, second: Ptr, info: Ptr, third: Ptr, cursor: Ptr) {
    let (first, second, third) = (first.addr(), second.addr(), third.addr());
    let offset = x87_quiet(e.mem.f32(first + 0x30));
    let points = [
        load_vec(e, second + 0x30),
        load_vec(e, third + 0x30),
        load_vec(e, first + 0x10),
        load_vec(e, first + 0x20),
    ];
    let scale = x87_quiet(e.mem.f32(first + 0x34));
    spring_record(e, info.addr(), cursor, points, offset, scale);
}

// Translated from 00d6ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A modifier that lowers the contact points' values (no engine-map name; cdecl). Register:
/// the info block (EDX); stack words: the atom and the cursor pointer. Backs the cursor up by
/// 0x10, copies the info block to the stack with `float[9]`, `float[0x10]` and `float[0x11]`
/// multiplied by the atom's `float[8]`, walks the atom chain (`+0x14`, while the type is at least
/// 0x17) to a node with `count = word[2]`, `skip = word[3]` and `stride = byte[0xa]` and, for
/// each of the `count` entries (`entry j` has its float at `node + 0x4c + 0x20 j`), takes
/// `u = atom.float[9] * info.float[1] * info.float[0] / atom.float[8] + entry` and
/// lowers the float at `node + 0x30 + (skip << 5) + stride j + 0x10` to `u` when `u` is smaller.
/// Then builds the Jacobian of the next atom: a contact atom (type 0x16) through
/// `hkSimpleContactConstraintDataBuildJacobian` (the flag is `count & 0xff00 | 1`), any other
/// through `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`.
pub fn fn_00d6ea70(e: &mut Engine, info: Ptr, atom: Ptr, cursor: Ptr) {
    let (info, atom) = (info.addr(), atom.addr());
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    let scale = x87_quiet(e.mem.f32(atom + 0x20));
    let id = u32::from(e.mem.u16(atom + 0x12));
    let numerator = x87_quiet(e.mem.f32(info + 0x24));
    e.with_stack(0x80, |e, copy| {
        let copy = copy.addr();
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        e.mem.set_f32(
            copy + 0x24,
            (f64::from(numerator) * f64::from(scale)) as f32,
        );
        let first = f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(scale);
        e.mem.set_f32(copy + 0x40, first as f32);
        let second = f64::from(scale) * f64::from(x87_quiet(e.mem.f32(info + 0x44)));
        e.mem.set_f32(copy + 0x44, second as f32);
        let mut node = atom;
        loop {
            node = e.mem.u32(node + 0x14);
            if e.mem.u16(node) < 0x17 {
                break;
            }
        }
        let count = u32::from(e.mem.u16(node + 4));
        let skip = u32::from(e.mem.u16(node + 6));
        let stride = u32::from(e.mem.u8(node + 0xa));
        let targets = node.wrapping_add(0x30).wrapping_add(skip << 5);
        for entry in 0..count {
            let value = x87_quiet(e.mem.f32(node + 0x4c + 0x20 * entry));
            let atom_numerator = f64::from(x87_quiet(e.mem.f32(atom + 0x24)));
            let first_factor = f64::from(x87_quiet(e.mem.f32(info + 4)));
            let second_factor = f64::from(x87_quiet(e.mem.f32(info)));
            let divisor = f64::from(x87_quiet(e.mem.f32(atom + 0x20)));
            let quotient = (((atom_numerator * first_factor) * second_factor) / divisor) as f32;
            let sum = f64::from(quotient) + f64::from(value);
            let target = targets
                .wrapping_add(stride.wrapping_mul(entry))
                .wrapping_add(0x10);
            if f64::from(x87_quiet(e.mem.f32(target))) > sum {
                e.mem.set_f32(target, sum as f32);
            }
        }
        let next = e.mem.u32(atom + 0x14);
        if e.mem.u16(next) == 0x16 {
            let flag = (count & 0xff00) | 1;
            e.call(BUILD_CONTACT_JACOBIAN, &args![next, copy, flag, cursor]);
        } else {
            e.call(BUILD_NOT_CONTACT_JACOBIAN, &args![next, id, copy, cursor]);
        }
    });
}

// Translated from 00d6ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A modifier that scales the bodies' vectors around one Jacobian (no engine-map name; cdecl).
/// Register: the atom (EAX); stack words: the info block and the cursor pointer. Copies the
/// info block and, with `fn_00d71f90`, both bodies' solver data to the stack; the copy's body
/// pointers (`+0x30`, `+0x34`) point to those copies, whose vectors at `+0x30` are multiplied
/// by the atom's vectors at `+0x20` and `+0x30`. Appends the tag-25 record of the two scaled
/// vectors (`00dced90`), then the Jacobian of the next atom built from the info copy (type
/// 0x16: `hkSimpleContactConstraintDataBuildJacobian` with the flag `id & 0xff00 | 1`; any
/// other: `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`), then the
/// tag-25 record of the bodies' own vectors at `+0x30`.
pub fn fn_00d6ecb0(e: &mut Engine, atom: Ptr, info: Ptr, cursor: Ptr) {
    let (atom, info) = (atom.addr(), info.addr());
    // The exe's locals in one block at its own offsets: the second body copy at `+0x30`, the
    // first at `+0xb0` and the info copy at `+0x130`.
    e.with_stack(0x1b0, |e, frame| {
        let frame = frame.addr();
        let (copy_b, copy_a, info_copy) = (frame + 0x30, frame + 0xb0, frame + 0x130);
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(info_copy + 4 * word, value);
        }
        let first_scale = load_vec(e, atom + 0x20);
        let next = e.mem.u32(atom + 0x14);
        let second_scale = load_vec(e, atom + 0x30);
        let id = u32::from(e.mem.u16(atom + 0x12));
        let body_a = e.mem.u32(info + 0x30);
        fn_00d71f90(e, Ptr::new(copy_a), Ptr::new(body_a));
        let body_b = e.mem.u32(info + 0x34);
        fn_00d71f90(e, Ptr::new(copy_b), Ptr::new(body_b));
        let scaled_a = mul(load_vec(e, copy_a + 0x30), first_scale);
        store_vec(e, copy_a + 0x30, scaled_a);
        e.mem.set_u32(info_copy + 0x30, copy_a);
        let scaled_b = mul(load_vec(e, copy_b + 0x30), second_scale);
        store_vec(e, copy_b + 0x30, scaled_b);
        e.mem.set_u32(info_copy + 0x34, copy_b);
        e.call(
            PUSH_VECTOR_PAIR,
            &args![copy_a + 0x30, copy_b + 0x30, cursor],
        );
        if e.mem.u16(next) == 0x16 {
            let flag = (id & 0xffff_ff00) | 1;
            e.call(
                BUILD_CONTACT_JACOBIAN,
                &args![next, info_copy, flag, cursor],
            );
        } else {
            e.call(
                BUILD_NOT_CONTACT_JACOBIAN,
                &args![next, id, info_copy, cursor],
            );
        }
        let body_a = e.mem.u32(info + 0x30);
        let body_b = e.mem.u32(info + 0x34);
        e.call(
            PUSH_VECTOR_PAIR,
            &args![body_a + 0x30, body_b + 0x30, cursor],
        );
    });
}

// Translated from 00d71f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a body's solver data (no engine-map name; thiscall, one stack word, `RET 4`): `this`
/// gets the byte at `+0`, the words at `+4` and `+0x10..+0x50`, the float at `+8` (through
/// the x87 stack, so a signalling NaN is quieted) and the 16-byte rows at `+0x50`, `+0x60`
/// and `+0x70` of `source`; bytes `1..4` and the word at `+0xc` stay as they were. Returns
/// `this`.
pub fn fn_00d71f90(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    let byte = e.mem.u8(from);
    e.mem.set_u8(to, byte);
    let word = e.mem.u32(from + 4);
    e.mem.set_u32(to + 4, word);
    let value = x87_quiet(e.mem.f32(from + 8));
    e.mem.set_f32(to + 8, value);
    for offset in (0x10..0x50).step_by(4) {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0x50, 0x60, 0x70] {
        copy_vec(e, from + offset, to + offset);
    }
    this
}

// Translated from 00d6f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromMovingSurfaceModifier` (engine map; cdecl). Register: the atom (EAX); stack
/// words: the info block and the cursor pointer. Copies the info block to the stack and
/// replaces the last record (the cursor backs up by 0x10) by a header record whose value word is
/// the address after the next atom's entries (`node + 0x30 + (word[3] << 5)`) and whose flag
/// is the byte `node[0xa]`. A local copy of body B (when `info.word[0x1b]` is 0, else of body
/// A; `00d72030`) replaces that body in the original info block, which keeps the stack
/// pointer; the atom's vector at `+0x20` (negated in its first three lanes for B) loses its
/// part along the node's vector at `+0x40` and is negated, `y`; the velocity record for `y`
/// (`hkAddVelocityBuildJacobian`), the copy's vector at `+0x10` plus `y`, the node's Jacobian
/// (`hkSimpleContactConstraintDataBuildJacobian`, flag `info.word[0x18] & 0xffffff00`, from the
/// info copy) and the velocity record for `-y` follow.
pub fn build_jacobian_from_moving_surface_modifier(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    let velocity = load_vec(e, atom + 0x20);
    e.with_stack(0x80, |e, copy| {
        let copy = copy.addr();
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        let node = e.mem.u32(atom + 0x14);
        let position = e.mem.u32(cursor.addr());
        e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
        let skip = u32::from(e.mem.u16(node + 6));
        let flag_byte = e.mem.u8(node + 0xa);
        let targets = skip.wrapping_shl(5).wrapping_add(0x30).wrapping_add(node);
        let identifier = e.mem.u32(info + 0x60);
        header_record(e, info, cursor, f32::from_bits(targets), flag_byte);
        let second_body = e.mem.u32(info + 0x6c) == 0;
        e.with_stack(0x80, |e, buffer| {
            let buffer_address = buffer.addr();
            let mut direction = velocity;
            if second_body {
                let body = e.mem.u32(info + 0x34);
                e.call(COPY_BODY_DATA, &args![buffer, body]);
                e.mem.set_u32(info + 0x34, buffer_address);
                let mask = load_bits(e, SIGN_MASK_XYZ);
                direction = std::array::from_fn(|lane| {
                    f32::from_bits(velocity[lane].to_bits() ^ mask[lane])
                });
            } else {
                let body = e.mem.u32(info + 0x30);
                e.call(COPY_BODY_DATA, &args![buffer, body]);
                e.mem.set_u32(info + 0x30, buffer_address);
            }
            let normal = load_vec(e, node + 0x40);
            let along = sum3(mul(normal, direction));
            let rest = sub(direction, mul([along; 4], normal));
            let negated = negate(e, rest);
            e.with_stack(0x10, |e, vector| {
                store_vec(e, vector.addr(), negated);
                e.call(ADD_VELOCITY, &args![vector, u32::from(second_body), cursor]);
                let body_vector = load_vec(e, buffer_address + 0x10);
                let pushed = load_vec(e, vector.addr());
                store_vec(e, buffer_address + 0x10, add(body_vector, pushed));
                let flag = identifier & 0xffff_ff00;
                e.call(BUILD_CONTACT_JACOBIAN, &args![node, copy, flag, cursor]);
                let pushed = load_vec(e, vector.addr());
                let opposite = negate(e, pushed);
                store_vec(e, vector.addr(), opposite);
                e.call(ADD_VELOCITY, &args![vector, u32::from(second_body), cursor]);
            });
        });
    });
}

// Translated from 00d6f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromLinMotorAtom` (engine map; cdecl). Register: the atom (ESI); stack words:
/// the info block, two bases (`first`, `second`) of 16-byte rows and the cursor pointer.
/// `flag` and `value` are the byte and the float at `info.word[0x19] + atom.short[2]` and
/// `+ atom.short[3]`. Without a motor (`atom.word[3]`) or with the atom's byte `+2` 0 it appends
/// the placeholder record (`{tag 3, byte[4] = 1}`). Otherwise `value` becomes `atom.float[2]`
/// when `flag` is 0, `hk1dLinearVelocityMotorBeginJacobian` makes the begin record from the
/// row `atom.byte[3]` of `second` and `first[0x30]`, `hkCalcMotorData` (the motor, `{begin,
/// info, limits[0], limits[1], atom.float[2] - value, value - (first[0x30] - second[0x30]) .
/// row}`) and `hk1dLinearVelocityMotorCommitJacobian` complete it; `value` and `flag` are
/// set to `atom.float[2]` and 1 when `atom.float[2]` differs from `value` or `flag` is 0.
pub fn build_jacobian_from_lin_motor_atom(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    first: Ptr,
    second: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    let (first, second) = (first.addr(), second.addr());
    let limits = e.mem.u32(info + 0x64);
    let flag = limits.wrapping_add(e.mem.u16(atom + 4) as i16 as i32 as u32);
    let value = limits.wrapping_add(e.mem.u16(atom + 6) as i16 as i32 as u32);
    let motor = e.mem.u32(atom + 0xc);
    if e.mem.u8(atom + 2) == 0 || motor == 0 {
        placeholder_record(e, cursor, 1);
        return;
    }
    if e.mem.u8(flag) == 0 {
        let target = x87_quiet(e.mem.f32(atom + 8));
        e.mem.set_f32(value, target);
    }
    let record = e.mem.u32(cursor.addr());
    let axis_row = u32::from(e.mem.u8(atom + 3)) << 4;
    e.with_stack(0x18, |e, block| {
        let block_address = block.addr();
        // The block's words 2 and 3 are zero before the call; the exe leaves the others unset.
        e.mem.set_u32(block_address + 8, 0);
        e.mem.set_u32(block_address + 0xc, 0);
        e.call(
            LINEAR_VELOCITY_BEGIN,
            &args![second + axis_row, first + 0x30, info, record, block],
        );
        e.mem.set_u32(block_address + 4, info);
        let target = x87_quiet(e.mem.f32(atom + 8));
        let current = x87_quiet(e.mem.f32(value));
        let remaining = (f64::from(target) - f64::from(current)) as f32;
        let bounds = [e.mem.u32(limits), e.mem.u32(limits + 4)];
        e.mem.set_u32(block_address + 8, bounds[0]);
        e.mem.set_f32(block_address + 0x10, remaining);
        let current = x87_quiet(e.mem.f32(value));
        e.mem.set_u32(block_address + 0xc, bounds[1]);
        let relative = sub(load_vec(e, first + 0x30), load_vec(e, second + 0x30));
        let axis = load_vec(e, second + axis_row);
        let along = sum3(mul(relative, axis));
        let passed = (f64::from(current) - f64::from(along)) as f32;
        e.mem.set_f32(block_address + 0x14, passed);
        e.with_stack(0x18, |e, out| {
            e.call(CALC_MOTOR_DATA, &args![motor, block, out]);
            e.call(LINEAR_VELOCITY_COMMIT, &args![out, info, record, cursor]);
        });
    });
    let now = e.mem.f32(value);
    let wanted = e.mem.f32(atom + 8);
    if wanted != now || e.mem.u8(flag) == 0 {
        e.mem.set_f32(value, x87_quiet(wanted));
        e.mem.set_u8(flag, 1);
    }
}

// Translated from 00d6edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromCenterOfMassChangerModifier` (engine map; cdecl). Stack words: the atom,
/// the info block and the cursor pointer. Copies the info block and, with `fn_00d71f90`, both
/// bodies' solver data to the stack; the copy's body pointers (`+0x30`, `+0x34`) point to those
/// copies. The atom's vectors at `+0x20` and `+0x30` are turned by the matrices `info.word[0xe]`
/// and `info.word[0xf]` (`hkVector4::setRotatedDir`) and added to the body copies' vectors at
/// `+0x40`. The skew-symmetric matrices of the two turned vectors
/// (`hkMatrix3::setCrossSkewSymmetric`) are multiplied by the inverse of each body copy's
/// rotation rows at `+0x50` (`hkMatrix3::setMulInverse`) into `m1` and `m2`. Copies of `m1` and
/// `m2` with the rows scaled by the `float` at `01012054` (-1.0) are applied to each body
/// copy's vector at `+0x20` (`hkVector4::setMul3`) and added to its vector at `+0x10`. The
/// center-of-mass record of the scaled copies (`hkSetCenterOfMassBuildJacobian`), the Jacobian
/// of the next atom built from the info copy (type 0x16:
/// `hkSimpleContactConstraintDataBuildJacobian` with the flag `id & 0xff00 | 1`; any other:
/// `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`) and the
/// center-of-mass record of `m1` and `m2` follow.
pub fn build_jacobian_from_center_of_mass_changer_modifier(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    // One block for all of the exe's locals, at the exe's own offsets (the block start is the
    // stack pointer of the body): the id at `+0xc`, the scale vector at `+0x10`, `m2` at `+0x20`,
    // the turned vectors at `+0x50` and `+0x60`, `m1` at `+0x70`, a scratch matrix at `+0xa0`,
    // the scaled copies at `+0xd0` and `+0x110`, the products at `+0x100` and `+0x140`, the two
    // body copies at `+0x150` and `+0x1d0` and the info copy at `+0x250`.
    e.with_stack(0x2d0, |e, frame| {
        let frame = frame.addr();
        let (turned_a, turned_b) = (frame + 0x60, frame + 0x50);
        let (m1, m2, scratch) = (frame + 0x70, frame + 0x20, frame + 0xa0);
        let (scaled_1, scaled_2, scale) = (frame + 0xd0, frame + 0x110, frame + 0x10);
        let (product_1, product_2) = (frame + 0x140, frame + 0x100);
        let (copy_a, copy_b, info_copy) = (frame + 0x150, frame + 0x1d0, frame + 0x250);
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(info_copy + 4 * word, value);
        }
        let id = u32::from(e.mem.u16(atom + 0x12));
        let next = e.mem.u32(atom + 0x14);
        let rotation_a = e.mem.u32(info + 0x38);
        e.call(SET_ROTATED_DIR, &args![turned_a, rotation_a, atom + 0x20]);
        let rotation_b = e.mem.u32(info + 0x3c);
        e.call(SET_ROTATED_DIR, &args![turned_b, rotation_b, atom + 0x30]);
        let body_a = e.mem.u32(info + 0x30);
        fn_00d71f90(e, Ptr::new(copy_a), Ptr::new(body_a));
        let body_b = e.mem.u32(info + 0x34);
        fn_00d71f90(e, Ptr::new(copy_b), Ptr::new(body_b));
        let position_a = add(load_vec(e, copy_a + 0x40), load_vec(e, turned_a));
        store_vec(e, copy_a + 0x40, position_a);
        let position_b = add(load_vec(e, copy_b + 0x40), load_vec(e, turned_b));
        e.mem.set_u32(info_copy + 0x34, copy_b);
        store_vec(e, copy_b + 0x40, position_b);
        e.mem.set_u32(info_copy + 0x30, copy_a);
        e.call(SET_CROSS_SKEW_SYMMETRIC, &args![m1, turned_a]);
        e.call(SET_CROSS_SKEW_SYMMETRIC, &args![m2, turned_b]);
        for (matrix, body_copy) in [(m1, copy_a), (m2, copy_b)] {
            for row in 0..3 {
                copy_vec(e, matrix + 0x10 * row, scratch + 0x10 * row);
            }
            e.call(SET_MUL_INVERSE, &args![matrix, scratch, body_copy + 0x50]);
        }
        let minus_one = e.global::<u32>(MINUS_ONE_FLOAT);
        for (matrix, scaled) in [(m1, scaled_1), (m2, scaled_2)] {
            for row in 0..3 {
                copy_vec(e, matrix + 0x10 * row, scaled + 0x10 * row);
            }
            store_bits(e, scale, [minus_one, 0, 0, 0]);
            e.call(SCALE_ROWS, &args![scaled, scale]);
        }
        e.call(SET_MUL3, &args![product_1, scaled_1, copy_a + 0x20]);
        e.call(SET_MUL3, &args![product_2, scaled_2, copy_b + 0x20]);
        let linear_a = add(load_vec(e, copy_a + 0x10), load_vec(e, product_1));
        store_vec(e, copy_a + 0x10, linear_a);
        let linear_b = add(load_vec(e, copy_b + 0x10), load_vec(e, product_2));
        store_vec(e, copy_b + 0x10, linear_b);
        e.call(SET_CENTER_OF_MASS, &args![scaled_1, scaled_2, cursor]);
        if e.mem.u16(next) == 0x16 {
            let flag = (id & 0xffff_ff00) | 1;
            e.call(
                BUILD_CONTACT_JACOBIAN,
                &args![next, info_copy, flag, cursor],
            );
        } else {
            e.call(
                BUILD_NOT_CONTACT_JACOBIAN,
                &args![next, id, info_copy, cursor],
            );
        }
        e.call(SET_CENTER_OF_MASS, &args![m1, m2, cursor]);
    });
}

/// The table of floats (`010c7948`) indexed by the byte at `atom + 3`: the scale of the error
/// term of the linear-limit records.
const ATOM_SCALE_TABLE: u32 = 0x010c_7948;

// Translated from 00d6ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Three linear-limit records for an atom (no engine-map name; `atom` is passed in EAX, the info
/// block in EDI, and the stack words are `a`, `b` (the points are the vectors at their +0x30) and the cursor). The body is
/// the same code as `fn_00d6c630` (which has no atom: its limit and scale are stack words):
/// here the limit is the float at `atom + 4` (loaded through the x87 stack, so a signalling NaN
/// comes out quiet) and the scale the float of the table at `010c7948` indexed by the byte at
/// `atom + 3`; the step is `(1 / weight) * scale`, so the translation of that function does the
/// rest.
pub fn fn_00d6ced0(e: &mut Engine, atom: Ptr, info: Ptr, a: Ptr, b: Ptr, cursor: Ptr) {
    let limit = x87_quiet(e.mem.f32(atom.addr() + 4));
    let kind = u32::from(e.mem.u8(atom.addr() + 3));
    let scale = e.mem.f32(ATOM_SCALE_TABLE + kind * 4);
    let (a, b) = (Ptr::new(a.addr() + 0x30), Ptr::new(b.addr() + 0x30));
    fn_00d6c630(e, info, a, b, cursor, limit, scale);
}

// Translated from 00d6f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverBuildJacobianFromAtomsNotContact` (Xbox PDB name per the engine map; cdecl: the atom
/// stream, its byte length, the constraint info block and the cursor pointer). It copies the
/// info block to the stack (the `schema` the 1D builders read), copies both bodies' rotation
/// blocks (`info.word[0xe]`, `info.word[0xf]`), writes the stream's opening record (tag 1, flag
/// 8), then walks the atoms (the type word at `atom + 0`) and appends the Jacobian records of
/// each, advancing the byte-pointer counters stored in the schema (`+0x64` and `+0x74`) as it
/// goes. The modifier atoms (0x17 to 0x1c) build their child's Jacobian through this function
/// or the contact builder by address. The exe keeps four constants on the x87 stack across the
/// loop (1.0, 0.0, the float at `0104da4c` and the double `FLT_EPSILON`); the translation reads
/// them where an atom uses them.
pub fn hk_solver_build_jacobian_from_atoms_not_contact(
    e: &mut Engine,
    atoms: Ptr,
    size: u32,
    info: Ptr,
    cursor: Ptr,
) {
    // The 0x80-byte schema copy and the two bodies' 0x40-byte rotation blocks live in one frame
    // at the exe's own offsets (schema `+0x60`, body from `info.word[0xf]` at `+0x180`, body from
    // `info.word[0xe]` at `+0x2b0`), so the records' pointer arithmetic lands where the exe's
    // does.
    let end = atoms.addr().wrapping_add(size);
    let mut atom = atoms.addr();
    e.with_stack(0x880, |e, frame| {
        let schema = frame.addr() + 0x60;
        let rows_a = frame.addr() + 0x180;
        let rows_b = frame.addr() + 0x2b0;
        for word in 0..0x20 {
            let value = e.mem.u32(info.addr() + 4 * word);
            e.mem.set_u32(schema + 4 * word, value);
        }
        let body_b = e.mem.u32(schema + 0x38);
        for word in 0..0x10 {
            let value = e.mem.u32(body_b + 4 * word);
            e.mem.set_u32(rows_b + 4 * word, value);
        }
        let body_a = e.mem.u32(schema + 0x3c);
        for word in 0..0x10 {
            let value = e.mem.u32(body_a + 4 * word);
            e.mem.set_u32(rows_a + 4 * word, value);
        }

        // The opening record: three counters' current values, `FLT_EPSILON` added to the first
        // float, the tag 1 and the flag 8.
        let record = e.mem.u32(cursor.addr());
        e.mem.set_u32(record + 8, e.mem.u32(schema + 0x64));
        e.mem.set_u16(record + 4, e.mem.u32(schema + 0x6c) as u16);
        e.mem.set_u16(record + 6, e.mem.u32(schema + 0x70) as u16);
        add_epsilon(e, record);
        e.mem.set_u8(record, 1);
        e.mem.set_u32(record + 0xc, e.mem.u32(schema + 0x60));
        e.mem.set_u8(record + 1, 8);
        e.mem.set_u32(cursor.addr(), record + 0x10);

        if end <= atom {
            return;
        }
        loop {
            let mut kind = e.mem.u16(atom);
            // Type 0 realigns the atom pointer up to the next 16 bytes and re-reads.
            while kind == 0 {
                atom = (atom + 0xf) & !0xf;
                kind = e.mem.u16(atom);
            }
            if kind <= 0x1c {
                dispatch_atom(e, &mut atom, kind, schema, rows_a, rows_b, cursor);
            }
            // Types 0x15, 0x16 and anything above 0x1c are no-ops that do not advance the atom
            // pointer (the exe loops on them; a well-formed stream never contains them).
            if end <= atom {
                return;
            }
        }
    });
}

/// Appends the records of one atom of type `kind` (1..=0x1c) and advances the atom pointer and
/// the schema counters. The counters are byte pointers in the schema (`+0x64` and `+0x74`;
/// the opening record also reads `+0x60`, `+0x6c` and `+0x70`).
fn dispatch_atom(
    e: &mut Engine,
    atom: &mut u32,
    kind: u16,
    schema: u32,
    rows_a: u32,
    rows_b: u32,
    cursor: Ptr,
) {
    let advance = |e: &mut Engine, offset: u32, by: u32| {
        let value = e.mem.u32(schema + offset).wrapping_add(by);
        e.mem.set_u32(schema + offset, value);
    };
    match kind {
        1 => {
            // A modifier whose node holds a function pointer: back the cursor up by 0x10 and call
            // `(atom.word[1])(atom.word[2], schema, cursor)`.
            let record = e.mem.u32(cursor.addr());
            e.mem.set_u32(cursor.addr(), record.wrapping_sub(0x10));
            let callee = e.mem.u32(*atom + 4);
            let argument = e.mem.u32(*atom + 8);
            e.call(callee, &args![argument, schema, cursor.addr()]);
            advance(e, 0x64, 0xc_3500);
            advance(e, 0x74, 0xc_3500);
            *atom += 0xc;
        }
        2 => {
            // Each body's rotation turns four of the atom's rows; the fourth row also gains the
            // body's translation (`body.row[3]`).
            let matrix_b = body_matrix(e, schema + 0x38);
            transform_rows(e, matrix_b, *atom + 0x10, rows_b);
            let translation_b = load_vec(e, e.mem.u32(schema + 0x38) + 0x30);
            let base_b = load_vec(e, rows_b + 0x30);
            store_vec(e, rows_b + 0x30, add(base_b, translation_b));
            let matrix_a = body_matrix(e, schema + 0x3c);
            transform_rows(e, matrix_a, *atom + 0x50, rows_a);
            let translation_a = load_vec(e, e.mem.u32(schema + 0x3c) + 0x30);
            let base_a = load_vec(e, rows_a + 0x30);
            store_vec(e, rows_a + 0x30, add(base_a, translation_a));
            *atom += 0x90;
        }
        3 => {
            // The atom's two points turned and translated by the two bodies, into each body
            // block's fourth row.
            let point_b = load_vec(e, *atom + 0x10);
            let turned_b = rotate_rows(point_b, body_matrix(e, schema + 0x38));
            let translation_b = load_vec(e, e.mem.u32(schema + 0x38) + 0x30);
            store_vec(e, rows_b + 0x30, add(turned_b, translation_b));
            let point_a = load_vec(e, *atom + 0x20);
            let turned_a = rotate_rows(point_a, body_matrix(e, schema + 0x3c));
            let translation_a = load_vec(e, e.mem.u32(schema + 0x3c) + 0x30);
            store_vec(e, rows_a + 0x30, add(turned_a, translation_a));
            *atom += 0x30;
        }
        4 => {
            // Like type 2 but no translation, and the second body turns the atom's rows at
            // `+0x40..+0x80` (the first turns `+0x10..+0x50`).
            let base = *atom;
            let matrix_b = body_matrix(e, schema + 0x38);
            transform_rows(e, matrix_b, base + 0x10, rows_b);
            let matrix_a = body_matrix(e, schema + 0x3c);
            transform_rows(e, matrix_a, base + 0x40, rows_a);
            *atom = base + 0x70;
        }
        5 => {
            // Three linear-limit records for the atom's limits, shared with `fn_00d6ced0`.
            fn_00d6ced0(
                e,
                Ptr::new(*atom),
                Ptr::new(schema),
                Ptr::new(rows_b),
                Ptr::new(rows_a),
                cursor,
            );
            advance(e, 0x64, 0x18);
            advance(e, 0x74, 0x18);
            *atom += 8;
        }
        6 => {
            atom_distance(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 8;
        }
        7..=9 => {
            atom_linear(e, *atom, kind, schema - 0x60, cursor);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += if kind == 7 { 4 } else { 0xc };
        }
        0xa => {
            atom_linear_friction(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 8;
        }
        0xb => {
            atom_linear_motor(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 0x10;
        }
        0xc => {
            atom_angular_pair(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 0x10);
            advance(e, 0x74, 0x10);
            *atom += 4;
        }
        0xd => {
            atom_angular_run(e, *atom, schema - 0x60, cursor);
            let count = u32::from(e.mem.u8(*atom + 3));
            *atom += 4;
            advance(e, 0x64, count * 8);
            advance(e, 0x74, count * 8);
        }
        0xe => {
            let written = atom_angle_limit(e, *atom, schema - 0x60);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 0x10;
            if written {
                angle_record(e, schema - 0x60, cursor);
            } else {
                placeholder_loaded(e, cursor);
            }
        }
        0xf => {
            atom_angle_cone(e, *atom, schema - 0x60);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 0x14;
            angle_record(e, schema - 0x60, cursor);
        }
        0x10 => {
            let written = atom_angle_twist(e, *atom, schema - 0x60);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 0x14;
            if written {
                angle_record(e, schema - 0x60, cursor);
            } else {
                placeholder_loaded(e, cursor);
            }
        }
        0x12 => {
            atom_angular_motor(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 0x14;
        }
        0x13 => {
            atom_ragdoll_motor(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 0x18);
            advance(e, 0x74, 0x18);
            *atom += 0x50;
        }
        0x11 => {
            atom_angular_friction(e, *atom, schema - 0x60, cursor);
            let count = u32::from(e.mem.u8(*atom + 4));
            *atom += 0xc;
            advance(e, 0x64, count * 8);
            advance(e, 0x74, count * 8);
        }
        0x1b => {
            // A container trailer: back the cursor up by 0x10 and step over the atom.
            let record = e.mem.u32(cursor.addr());
            e.mem.set_u32(cursor.addr(), record.wrapping_sub(0x10));
            *atom += 0x20;
        }
        0x14 => {
            atom_length(e, *atom, schema - 0x60, cursor);
            advance(e, 0x64, 8);
            advance(e, 0x74, 8);
            *atom += 0x40;
        }
        0x17 => {
            atom_contact_lowering(e, *atom, schema - 0x60, cursor);
            *atom += 0x30;
        }
        0x18 => {
            atom_body_scaling(e, *atom, schema - 0x60, cursor);
            *atom += 0x40;
        }
        0x19 => {
            atom_surface_viscosity(e, *atom, schema - 0x60, cursor);
            *atom += 0x20;
        }
        0x1a => {
            atom_moving_surface(e, *atom, schema - 0x60, cursor);
            *atom += 0x30;
        }
        0x1c => {
            atom_centre_of_mass(e, *atom, schema - 0x60, cursor);
            *atom += 0x40;
        }
        // Types 0x15 and 0x16 (and those above 0x1c, which the loop already skips) are no-ops
        // that do not advance the atom pointer, as in the exe.
        _ => {}
    }
}

/// The three rotation rows of a body matrix (`+0`, `+0x10`, `+0x20` from the pointer at
/// `pointer_slot` in the schema).
fn body_matrix(e: &Engine, pointer_slot: u32) -> [Vec4; 3] {
    let base = e.mem.u32(pointer_slot);
    [
        load_vec(e, base),
        load_vec(e, base + 0x10),
        load_vec(e, base + 0x20),
    ]
}

/// Turns four consecutive atom rows (`source + 0`, `+0x10`, `+0x20`, `+0x30`) by `matrix` and
/// stores them in the body block (`rows + 0`, `+0x10`, `+0x20`, `+0x30`).
fn transform_rows(e: &mut Engine, matrix: [Vec4; 3], source: u32, rows: u32) {
    for row in 0..4u32 {
        let turned = rotate_rows(load_vec(e, source + 0x10 * row), matrix);
        store_vec(e, rows + 0x10 * row, turned);
    }
}

/// `fn_00dcf5b0` (cdecl: an input block, the info block, the cursor pointer, a float scale).
const SCALED_LINEAR_RECORD: u32 = 0x00dc_f5b0;
/// `hk1dLinearBilateralConstraintBuildJacobian` (cdecl: an input block, the info block, the
/// cursor pointer).
const LINEAR_BILATERAL: u32 = 0x00dc_f280;
/// `fn_00dcee80`, the angular bilateral builder (cdecl: an input block, the info block, the
/// cursor pointer).
const ANGULAR_BILATERAL: u32 = 0x00dc_ee80;
/// The byte tables of row offsets the angular atoms index by axis (`010d33c0`, then the next
/// and third axes at `010d33c1` and `010d33c2`).
const AXIS_OFFSET_TABLE: u32 = 0x010d_33c0;

// Frame offsets of the dispatcher (from its stack pointer): the schema copy, the body block
// copied from `info.word[0xf]` (`ROWS_A`, its fourth row `PIVOT_A`) and the one from
// `info.word[0xe]` (`ROWS_B`, `PIVOT_B`).
const SCHEMA: u32 = 0x60;
const ROWS_A: u32 = 0x180;
const PIVOT_A: u32 = 0x1b0;
const ROWS_B: u32 = 0x2b0;
const PIVOT_B: u32 = 0x2e0;

/// `FLD dword` of the float at `address`.
fn fld(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A float memory operand of an x87 arithmetic instruction.
fn m32(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// `FSTP dword` to `address`.
fn fstp(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_u32(address, x87_store32(value));
}

/// The x87 `FLD qword` of `FLT_EPSILON` (the constant the dispatcher keeps on its stack).
fn epsilon_on_stack(e: &Engine) -> f64 {
    x87_load64(e.mem.u64(EPSILON))
}

/// The `AH` status bits of an x87 compare of `a` with `b`: `C0` (0x01) for less, `C3` (0x40)
/// for equal, all of `C0`, `C2`, `C3` for unordered.
fn compare_status(a: f64, b: f64) -> u8 {
    if a.is_nan() || b.is_nan() {
        0x45
    } else if a < b {
        0x01
    } else if a == b {
        0x40
    } else {
        0
    }
}

/// `TEST AH, mask` then `JP`: whether the masked status bits have even parity.
fn parity_even(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

/// The placeholder record of the dispatcher: `FLT_EPSILON` (from the x87 stack) plus the
/// stream's first float, `marker` at `+4`, the tag 3; the cursor moves on by 0x10.
fn placeholder_from_stack(e: &mut Engine, cursor: Ptr, marker: u8) {
    let record = e.mem.u32(cursor.addr());
    let sum = x87_add(epsilon_on_stack(e), m32(e, record));
    e.mem.set_u8(record + 4, marker);
    fstp(e, record, sum);
    e.mem.set_u8(record, 3);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

/// Atom type 6 (`00d6f9df`): the two pivots and their unit direction (the first axis of the
/// table at `01268370` when the distance is not above the float at `0104da4c`, then the
/// distance counts as 0), and `fn_00dcf5b0` with the scale `atom.float[1] - distance`.
fn atom_distance(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let pivot_b = load_vec(e, fp + PIVOT_B);
    let pivot_a = load_vec(e, fp + PIVOT_A);
    store_vec(e, fp + 0x300, pivot_b);
    store_vec(e, fp + 0x310, pivot_a);
    let difference = sub(pivot_b, pivot_a);
    let squares = mul(difference, difference);
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    let loaded = x87_load32(length.to_bits());
    e.mem.set_u32(fp + 0x30, length.to_bits());
    let small = fld(e, SMALL_FLOAT);
    if parity_even(compare_status(loaded, small), 0x41) {
        let inverse = f32::from_bits(x87_store32(x87_div(1.0, loaded)));
        store_vec(e, fp + 0x320, mul([inverse; 4], difference));
    } else {
        copy_vec(e, AXIS_TABLE, fp + 0x320);
        e.mem.set_u32(fp + 0x30, x87_store32(0.0));
    }
    let scale = x87_store32(x87_sub(fld(e, atom + 4), m32(e, fp + 0x30)));
    e.call(
        SCALED_LINEAR_RECORD,
        &args![fp + 0x300, fp + SCHEMA, cursor.addr(), scale],
    );
}

/// Atom types 7, 8 and 9 (`00d6fac2`, `00d6fb2d`, `00d6fbac`): the input block `{pivot B,
/// pivot A, row atom.byte[2] of block A}` (types 8 and 9 add the atom's floats `+4` and `+8` at
/// `+0x30` and `+0x34`) for the bilateral, user-tau bilateral and limit builders.
fn atom_linear(e: &mut Engine, atom: u32, kind: u16, fp: u32, cursor: Ptr) {
    let block = fp + 0x1c0;
    let row = u32::from(e.mem.u8(atom + 2)) << 4;
    copy_vec(e, fp + PIVOT_B, block);
    if kind != 7 {
        let tau = fld(e, atom + 4);
        fstp(e, block + 0x30, tau);
        let limit = fld(e, atom + 8);
        fstp(e, block + 0x34, limit);
    }
    copy_vec(e, fp + PIVOT_A, block + 0x10);
    copy_vec(e, fp + ROWS_A + row, block + 0x20);
    let callee = match kind {
        7 => LINEAR_BILATERAL,
        8 => LINEAR_BILATERAL_USER_TAU,
        _ => LINEAR_LIMIT,
    };
    e.call(callee, &args![block, fp + SCHEMA, cursor.addr()]);
}

/// Atom type 0xa (`00d6fc2b`): with `atom.byte[2]` set and `0 < atom.float[1]`, the friction
/// block `{pivot B, row atom.byte[3] of block A, atom.float[1], schema.word[0x19]}` for
/// `hk1dLinearFrictionBuildJacobian`; otherwise the placeholder record (marker 1).
fn atom_linear_friction(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let enabled = e.mem.u8(atom + 2) != 0;
    if enabled && compare_status(0.0, m32(e, atom + 4)) & 0x05 == 0x01 {
        let row = u32::from(e.mem.u8(atom + 3)) << 4;
        let block = fp + 0x1c0;
        let friction = fld(e, atom + 4);
        let counter = e.mem.u32(fp + SCHEMA + 0x64);
        fstp(e, block + 0x20, friction);
        copy_vec(e, fp + PIVOT_B, block);
        copy_vec(e, fp + ROWS_A + row, block + 0x10);
        e.mem.set_u32(block + 0x24, counter);
        e.call(LINEAR_FRICTION, &args![block, fp + SCHEMA, cursor.addr()]);
    } else {
        placeholder_from_stack(e, cursor, 1);
    }
}

/// Atom type 0xb (`00d6fcd6`): a linear velocity motor. `flag` and `value` are the byte and the
/// float at `counter + atom.short[2]` and `+ atom.short[3]` (`counter` the schema's `+0x64`).
/// Without `atom.byte[2]` or a motor (`atom.word[3]`) it appends the placeholder record
/// (marker 1). Otherwise `value` becomes `atom.float[2]` when `flag` is 0; the begin builder gets
/// row `atom.byte[3]` of block A, pivot B, the schema, the record and the motor block at
/// `+0x140` (`{.., schema, counter[0], counter[1], target - value, value - (B - A) . row}`);
/// `hkCalcMotorData` and the commit builder follow, and `value`/`flag` are set to the target
/// and 1 unless the two floats are equal and `flag` is set.
fn atom_linear_motor(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let flag = (e.mem.u16(atom + 4) as i16 as i32 as u32).wrapping_add(counter);
    let value = (e.mem.u16(atom + 6) as i16 as i32 as u32).wrapping_add(counter);
    e.mem.set_u32(fp + 0x30, flag);
    let motor = e.mem.u32(atom + 0xc);
    if e.mem.u8(atom + 2) == 0 || motor == 0 {
        placeholder_from_stack(e, cursor, 1);
        return;
    }
    e.mem.set_u32(fp + 0x10, motor);
    if e.mem.u8(flag) == 0 {
        let target = fld(e, atom + 8);
        fstp(e, value, target);
    }
    let record = e.mem.u32(cursor.addr());
    let block = fp + 0x140;
    fstp(e, block + 8, 0.0);
    fstp(e, block + 0xc, 0.0);
    e.mem.set_u32(fp + 0x3c, record);
    let row = u32::from(e.mem.u8(atom + 3)) << 4;
    e.call(
        LINEAR_VELOCITY_BEGIN,
        &args![fp + ROWS_A + row, fp + PIVOT_B, fp + SCHEMA, record, block],
    );
    let pivot_a = load_vec(e, fp + PIVOT_A);
    let remaining = x87_sub(fld(e, atom + 8), m32(e, value));
    let pivot_b = load_vec(e, fp + PIVOT_B);
    e.mem.set_u32(block + 4, fp + SCHEMA);
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let first = e.mem.u32(counter);
    e.mem.set_u32(block + 8, first);
    let second = e.mem.u32(counter + 4);
    fstp(e, block + 0x10, remaining);
    let current = fld(e, value);
    let row = u32::from(e.mem.u8(atom + 3)) << 4;
    let products = mul(sub(pivot_b, pivot_a), load_vec(e, fp + ROWS_A + row));
    let along = products[2] + (products[1] + products[0]);
    e.mem.set_u32(block + 0xc, second);
    e.mem.set_u32(fp + 0x264, along.to_bits());
    let passed = x87_sub(current, m32(e, fp + 0x264));
    fstp(e, block + 0x14, passed);
    e.call(CALC_MOTOR_DATA, &args![motor, block, fp + 0x200]);
    e.call(
        LINEAR_VELOCITY_COMMIT,
        &args![fp + 0x200, fp + SCHEMA, record, cursor.addr()],
    );
    let now = fld(e, value);
    let target = fld(e, atom + 8);
    let status = compare_status(target, now);
    if parity_even(status, 0x44) || e.mem.u8(flag) == 0 {
        let target = fld(e, atom + 8);
        fstp(e, value, target);
        e.mem.set_u8(flag, 1);
    }
}

/// Atom type 0xc (`00d6fe69`): two angular bilateral records about row `axis = atom.byte[2]`
/// of block B, the first with the other two rows of block A (`next`, `third` from the tables
/// at `010d33c1` and `010d33c2`) as `{third, next}`, the second as `{-next, third}`.
fn atom_angular_pair(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let axis = u32::from(e.mem.u8(atom + 2));
    let next = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis));
    let third = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis));
    let block = fp + 0x300;
    copy_vec(e, fp + ROWS_B + (axis << 4), block);
    let next_row = fp + ROWS_A + next;
    let third_row = fp + ROWS_A + third;
    copy_vec(e, next_row, block + 0x20);
    e.mem.set_u32(fp + 0x10, third_row);
    copy_vec(e, third_row, block + 0x10);
    e.call(ANGULAR_BILATERAL, &args![block, fp + SCHEMA, cursor.addr()]);
    copy_vec(e, third_row, block + 0x20);
    let negated = negate(e, load_vec(e, next_row));
    store_vec(e, block + 0x10, negated);
    e.call(ANGULAR_BILATERAL, &args![block, fp + SCHEMA, cursor.addr()]);
}

/// Atom type 0xd (`00d6ff29`): for `i` from `atom.byte[2]` while `i < atom.byte[2] +
/// atom.byte[3]` (re-read each step) an angular bilateral record from the block `{row c1 of
/// block B, row c0 of block B, row c2 of block A}`, `c0`, `c1`, `c2` the bytes at
/// `010d33c0 + i`, `+1`, `+2`.
fn atom_angular_run(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let block = fp + 0x1c0;
    e.mem.set_u32(fp + 0x30, atom + 3);
    let mut index = u32::from(e.mem.u8(atom + 2));
    let mut limit = index + u32::from(e.mem.u8(atom + 3));
    while (index as i32) < (limit as i32) {
        let first = u32::from(e.mem.u8(AXIS_OFFSET_TABLE + index));
        let second = u32::from(e.mem.u8(AXIS_OFFSET_TABLE + 1 + index));
        copy_vec(e, fp + ROWS_B + first, block + 0x10);
        let third = u32::from(e.mem.u8(AXIS_OFFSET_TABLE + 2 + index));
        copy_vec(e, fp + ROWS_B + second, block);
        copy_vec(e, fp + ROWS_A + third, block + 0x20);
        e.call(ANGULAR_BILATERAL, &args![block, fp + SCHEMA, cursor.addr()]);
        limit = u32::from(e.mem.u8(atom + 2)) + u32::from(e.mem.u8(atom + 3));
        index += 1;
    }
}

/// Atom type 0x11 (`00d70784`): with `atom.byte[2]` set and `atom.float[2]` not 0 (a NaN counts
/// as not 0), the angular friction block `{&row atom.byte[3] of block B, schema.word[0x19],
/// atom.float[2], atom.byte[4]}`; otherwise the placeholder record with the marker
/// `atom.byte[4]`.
fn atom_angular_friction(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let enabled = e.mem.u8(atom + 2) != 0;
    if enabled && parity_even(compare_status(0.0, m32(e, atom + 8)), 0x44) {
        let row = u32::from(e.mem.u8(atom + 3)) << 4;
        let marker = u32::from(e.mem.u8(atom + 4));
        let block = fp + 0x360;
        let friction = fld(e, atom + 8);
        fstp(e, block + 8, friction);
        let counter = e.mem.u32(fp + SCHEMA + 0x64);
        e.mem.set_u32(block, fp + ROWS_B + row);
        e.mem.set_u32(block + 0xc, marker);
        e.mem.set_u32(block + 4, counter);
        e.call(ANGULAR_FRICTION, &args![block, fp + SCHEMA, cursor.addr()]);
    } else {
        let marker = e.mem.u8(atom + 4);
        placeholder_from_stack(e, cursor, marker);
    }
}

/// The `double` at `address` as an x87 arithmetic memory operand.
fn m64(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// `FSTP dword` then `FLD dword`: the value rounded through a float.
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `hkMath_atan2fApproximation` on two floats (each copied through the x87 stack), the result
/// in `ST0`.
fn atan2_of(e: &mut Engine, first: f32, second: f32) -> f64 {
    let words = [
        x87_quiet_bits(first.to_bits()),
        x87_quiet_bits(second.to_bits()),
    ];
    e.call(ATAN2_APPROXIMATION, &args![words[0], words[1]])
        .f64()
}

/// The dispatcher's turn loops (`00d7010a` and `00d70955`): the float angle at `angle_at` is
/// moved by whole turns of 2 pi (the doubles at `0101ff40`, `0101ff48`) until it lies within pi
/// of `limit`, each step stored as a float and loaded again. Returns the angle as the x87 stack
/// holds it at the end.
fn wrap_turns(e: &mut Engine, angle_at: u32, limit: f64) -> f64 {
    let pi = x87_load64(e.mem.u64(PI));
    let two_pi = x87_load64(e.mem.u64(TWO_PI));
    let mut angle = fld(e, angle_at);
    if compare_status(pi, x87_sub(limit, angle)) & 0x05 == 0x01 {
        loop {
            fstp(e, angle_at, x87_add(angle, two_pi));
            angle = fld(e, angle_at);
            if compare_status(x87_sub(limit, angle), pi) != 0 {
                break;
            }
        }
    }
    while compare_status(x87_sub(angle, limit), pi) & 0x41 == 0 {
        fstp(e, angle_at, x87_sub(angle, two_pi));
        angle = fld(e, angle_at);
    }
    angle
}

/// The placeholder record of the angle atoms (`00d7019e`): the stream's first float loaded,
/// `FLT_EPSILON` (the double) added, the marker 1 at `+4` and the tag 3; the cursor moves on
/// by 0x10.
fn placeholder_loaded(e: &mut Engine, cursor: Ptr) {
    let record = e.mem.u32(cursor.addr());
    let first = fld(e, record);
    e.mem.set_u8(record + 4, 1);
    let sum = x87_add(first, m64(e, EPSILON));
    fstp(e, record, sum);
    e.mem.set_u8(record, 3);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

/// The angle record shared by atom types 0xe, 0xf and 0x10 (`00d70646`), from the frame's
/// input block: the axis `v` (`+0x120`), two floats `a1`, `a2` (`+0x130`, `+0x134`), the angle
/// (`+0x138`) and the weight `k` (`+0x13c`). The record gets the angular pair of `v` (`+0`,
/// `+0x10`), `k / (sum + FLT_EPSILON)` at `+0xc`, with `c = -schema.float[2]`: `angle * c` at
/// `+0x1c`, `(a1 * c) * k` at `+0x20`, `(c * a2) * k` at `+0x24`, `k` at `+0x28`; the first
/// float gets `FLT_EPSILON` added and the tag 0xd. The cursor moves on by 0x30.
fn angle_record(e: &mut Engine, fp: u32, cursor: Ptr) {
    let weight = fld(e, fp + 0x13c);
    let record = e.mem.u32(cursor.addr());
    let sum = angular_pair(e, fp + SCHEMA, fp + 0x120, record);
    e.mem.set_u32(fp + 0x298, sum.to_bits());
    let epsilon = epsilon_on_stack(e);
    let mass = through_float(x87_add(x87_load32(sum.to_bits()), epsilon));
    fstp(e, fp + 0x10, mass);
    fstp(e, record + 0xc, x87_div(weight, mass));
    let negated = -fld(e, fp + SCHEMA + 8);
    fstp(e, fp + 0x10, negated);
    let angle = fld(e, fp + 0x138);
    let c = fld(e, fp + 0x10);
    fstp(e, record + 0x1c, x87_mul(angle, c));
    let record = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), record + 0x30);
    let first = x87_mul(x87_mul(fld(e, fp + 0x130), c), weight);
    fstp(e, record + 0x20, first);
    let second = x87_mul(x87_mul(c, m32(e, fp + 0x134)), weight);
    fstp(e, record + 0x24, second);
    fstp(e, record + 0x28, weight);
    let sum = x87_add(epsilon, m32(e, record));
    fstp(e, record, sum);
    e.mem.set_u8(record, 0xd);
}

/// Atom type 0xe (`00d6ffe2`): with `atom.byte[2]` set, the angle about row `axis =
/// atom.byte[3]` of block B: `atan2(third_A . next_B, (third_A x row_A) . next_B)` (`next`,
/// `third` from the tables at `010d33c1`, `010d33c2`), moved by whole turns to within pi of
/// `-counter.float[1]`; the input block of the angle record gets row `axis` of block B, the
/// atom's floats `+4`, `+8` and `atom.float[3] * schema.float[0x10]`. Returns false (the
/// placeholder) without `atom.byte[2]`.
fn atom_angle_limit(e: &mut Engine, atom: u32, fp: u32) -> bool {
    if e.mem.u8(atom + 2) == 0 {
        return false;
    }
    let axis = u32::from(e.mem.u8(atom + 3));
    let weight = x87_mul(fld(e, atom + 0xc), m32(e, fp + SCHEMA + 0x40));
    let next = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis));
    let next_b = load_vec(e, fp + ROWS_B + next);
    fstp(e, fp + 0x13c, weight);
    let third = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis));
    let first = fld(e, atom + 4);
    fstp(e, fp + 0x130, first);
    let second = fld(e, atom + 8);
    let third_a = load_vec(e, fp + ROWS_A + third);
    fstp(e, fp + 0x134, second);
    let dot = sum3(mul(third_a, next_b));
    copy_vec(e, fp + ROWS_B + (axis << 4), fp + 0x120);
    let row_a = load_vec(e, fp + ROWS_A + (axis << 4));
    let turned = sum3(mul(cross(third_a, row_a), next_b));
    e.mem.set_u32(fp + 0x238, turned.to_bits());
    e.mem.set_u32(fp + 0x260, dot.to_bits());
    let angle = atan2_of(e, dot, turned);
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    fstp(e, fp + 0x138, angle);
    let limit = x87_mul(fld(e, counter + 4), m64(e, MINUS_ONE));
    fstp(e, fp + 0x10, limit);
    let limit = fld(e, fp + 0x10);
    wrap_turns(e, fp + 0x138, limit);
    true
}

/// Atom type 0xf (`00d7048a`): the angle about `n`, row `axis = atom.byte[3]` of block B plus
/// that of block A, normalised when its length is above the double `1e-16` (`010d33d0`; then
/// `half` is half the length, else `n` is block A's row and `half` 0):
/// `atan2(w . t, w . (t x n))` with `t = n x b`, `b` and `w` the rows `atom.byte[4]` of blocks A
/// and B. The input block of the angle record gets `n`, the atom's floats `+8`, `+0xc` and the
/// weight `(atom.float[4] * schema.float[0x10]) * half * half`.
fn atom_angle_cone(e: &mut Engine, atom: u32, fp: u32) {
    let other = u32::from(e.mem.u8(atom + 4)) << 4;
    let first = fld(e, atom + 8);
    fstp(e, fp + 0x130, first);
    let axis = u32::from(e.mem.u8(atom + 3)) << 4;
    let second = fld(e, atom + 0xc);
    fstp(e, fp + 0x134, second);
    let row_a = load_vec(e, fp + ROWS_A + axis);
    let sum = add(load_vec(e, fp + ROWS_B + axis), row_a);
    let length = sum3(mul(sum, sum)).sqrt();
    e.mem.set_u32(fp + 0x278, length.to_bits());
    let loaded = x87_load32(length.to_bits());
    let normal = if compare_status(loaded, m64(e, TINY)) & 0x41 == 0 {
        let half = x87_mul(loaded, m64(e, HALF));
        fstp(e, fp + 0x178, half);
        let inverse = x87_store32(x87_div(1.0, loaded));
        e.mem.set_u32(fp + 0x10, inverse);
        mul([f32::from_bits(inverse); 4], sum)
    } else {
        fstp(e, fp + 0x178, 0.0);
        row_a
    };
    let b = load_vec(e, fp + ROWS_A + other);
    let tangent = cross(normal, b);
    let binormal = cross(tangent, normal);
    store_vec(e, fp + 0x3a0, normal);
    let w = load_vec(e, fp + ROWS_B + other);
    let along = sum3(mul(w, tangent));
    let across = sum3(mul(w, binormal));
    e.mem.set_u32(fp + 0x288, across.to_bits());
    e.mem.set_u32(fp + 0x290, along.to_bits());
    let angle = atan2_of(e, along, across);
    fstp(e, fp + 0x138, angle);
    let weight = x87_mul(fld(e, atom + 0x10), m32(e, fp + SCHEMA + 0x40));
    let half = fld(e, fp + 0x178);
    let weight = x87_mul(x87_mul(weight, half), half);
    store_vec(e, fp + 0x120, normal);
    fstp(e, fp + 0x13c, weight);
}

/// Atom type 0x10 (`00d701c2`): `c = a x b` with `a` row `atom.byte[3]` of block B and `b` row
/// `atom.byte[4]` of block A. Returns false (the placeholder) when `|c|^2` is below the float at
/// `0104da4c`. Otherwise the angle `atan2(|c|, a . b)` (the CRT square root `00ec6040`) scaled
/// and offset by the tables at `010d33e0`/`010d33d8` (index `atom.byte[5]`); the input block
/// gets `(c / |c|) * sign` (`010d33e4` going down), the atom's floats `+8`, `+0xc`, the angle and
/// `atom.float[4] * schema.float[0x10]`. With a limit slot `s = atom.byte[6] > 1`, the limit
/// `a = counter.float[s]` and `c = -counter.float[1]`, `x = angle - a`, `y = c - x`, the bound
/// `M = max(schema.float[3] * 0.5, a * K)` (`K` the double at `0103c7c8`; NaN picks the first),
/// `counter.float[s]` becomes `a - M` and the angle `x + M`, both moved by `y` when
/// `0.5 * (0.5 m + 2 M) < -y` and `c` is not 0; the slot keeps at least the small float.
fn atom_angle_twist(e: &mut Engine, atom: u32, fp: u32) -> bool {
    let axis = u32::from(e.mem.u8(atom + 3)) << 4;
    let other = u32::from(e.mem.u8(atom + 4)) << 4;
    let a = load_vec(e, fp + ROWS_B + axis);
    let b = load_vec(e, fp + ROWS_A + other);
    let turned = cross(a, b);
    store_vec(e, fp + 0x490, turned);
    let squared = sum3(mul(turned, turned));
    e.mem.set_u32(fp + 0x268, squared.to_bits());
    let loaded = x87_load32(squared.to_bits());
    let small = fld(e, SMALL_FLOAT);
    if !parity_even(compare_status(loaded, small), 0x05) {
        return false;
    }
    let index = u32::from(e.mem.u8(atom + 5));
    let dot = sum3(mul(a, b));
    e.mem.set_u32(fp + 0x270, dot.to_bits());
    let root = e.call(X87_SQRT, &args![loaded]).f64();
    fstp(e, fp + 0x30, root);
    let length = f32::from_bits(e.mem.u32(fp + 0x30));
    let angle = atan2_of(e, length, dot);
    let scaled = x87_mul(angle, m32(e, ANGLE_SCALE_TABLE + 4 * index));
    let index = u32::from(e.mem.u8(atom + 5));
    let value = x87_add(scaled, m32(e, ANGLE_OFFSET_TABLE + 4 * index));
    let sign_at = AXIS_SIGN_TABLE.wrapping_sub(4 * index);
    let slot = u32::from(e.mem.u8(atom + 6));
    fstp(e, fp + 0x5c, value);
    let sign = fld(e, sign_at);
    fstp(e, fp + 0x3c, sign);
    let sign = f32::from_bits(e.mem.u32(fp + 0x3c));
    let weight = x87_mul(fld(e, atom + 0x10), m32(e, fp + SCHEMA + 0x40));
    fstp(e, fp + 0x13c, weight);
    let inverse = x87_div(1.0, fld(e, fp + 0x30));
    fstp(e, fp + 0x10, inverse);
    let value = fld(e, fp + 0x5c);
    fstp(e, fp + 0x138, value);
    let inverse = f32::from_bits(e.mem.u32(fp + 0x10));
    let first = fld(e, atom + 8);
    fstp(e, fp + 0x130, first);
    let second = fld(e, atom + 0xc);
    let axis_vector = mul(mul([inverse; 4], load_vec(e, fp + 0x490)), [sign; 4]);
    fstp(e, fp + 0x134, second);
    store_vec(e, fp + 0x120, axis_vector);
    if slot > 1 {
        twist_limit(e, fp, slot, value);
    }
    true
}

/// The limit update of atom type 0x10 (`00d7036d`); `value` is the angle on the x87 stack.
fn twist_limit(e: &mut Engine, fp: u32, slot: u32, value: f64) {
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let slot_at = counter.wrapping_add(slot);
    let limit = fld(e, slot_at);
    fstp(e, fp + 0x2c, limit);
    let negated = -fld(e, counter + 4);
    fstp(e, fp + 0x10, negated);
    let a = fld(e, fp + 0x2c);
    fstp(e, fp + 0x34, x87_sub(value, a));
    let c = fld(e, fp + 0x10);
    let x = fld(e, fp + 0x34);
    fstp(e, fp + 0x3c, x87_sub(c, x));
    let half = x87_load64(e.mem.u64(HALF));
    let half_m = x87_mul(fld(e, fp + SCHEMA + 0xc), half);
    fstp(e, fp + 0x30, half_m);
    fstp(e, fp + 0x10, x87_mul(a, m64(e, LIMIT_SCALE)));
    let scaled = fld(e, fp + 0x10);
    let half_m = fld(e, fp + 0x30);
    if parity_even(compare_status(half_m, scaled), 0x05) {
        fstp(e, fp + 0x30, half_m);
    } else {
        fstp(e, fp + 0x30, scaled);
    }
    let bound = fld(e, fp + 0x30);
    fstp(e, fp + 0x2c, x87_sub(a, bound));
    fstp(e, fp + 0x34, x87_add(x, bound));
    let y = fld(e, fp + 0x3c);
    let blend = x87_mul(half, x87_add(half_m, x87_add(bound, bound)));
    fstp(e, fp + 0x10, blend);
    let blend = fld(e, fp + 0x10);
    if !parity_even(compare_status(blend, -y), 0x05) && parity_even(compare_status(0.0, c), 0x44) {
        let low = x87_sub(fld(e, fp + 0x2c), y);
        fstp(e, fp + 0x2c, low);
        let high = x87_add(y, m32(e, fp + 0x34));
        fstp(e, fp + 0x34, high);
    }
    let small = fld(e, SMALL_FLOAT);
    if compare_status(small, m32(e, fp + 0x2c)) & 0x05 != 0x01 {
        fstp(e, fp + 0x2c, small);
    }
    let low = fld(e, fp + 0x2c);
    fstp(e, slot_at, low);
    let high = fld(e, fp + 0x34);
    fstp(e, fp + 0x138, high);
}

/// The second record of the motor atoms (tag 0xf, 0x40 bytes) from the motor data at `+0x200`
/// of the frame: the first float plus `FLT_EPSILON`, `out[2]` and `out[3]` times
/// `schema.float[1]` at `+0x20`, `+0x24` (`tau_first`: the exe loads `schema.float[1]` first
/// for `+0x20`), and `out[1]`, `out[4]`, `out[5]` at `+0x28`, `+0x2c`, `+0x30`.
fn motor_commit_record(e: &mut Engine, fp: u32, cursor: Ptr, tau_first: bool) {
    let second = e.mem.u32(cursor.addr());
    let sum = x87_add(fld(e, second), m64(e, EPSILON));
    fstp(e, second, sum);
    e.mem.set_u8(second, 0xf);
    let first = if tau_first {
        let tau = fld(e, fp + SCHEMA + 4);
        e.mem.set_u32(cursor.addr(), second + 0x40);
        x87_mul(tau, m32(e, fp + 0x208))
    } else {
        let out = fld(e, fp + 0x208);
        e.mem.set_u32(cursor.addr(), second + 0x40);
        x87_mul(out, m32(e, fp + SCHEMA + 4))
    };
    fstp(e, second + 0x20, first);
    let product = x87_mul(fld(e, fp + 0x20c), m32(e, fp + SCHEMA + 4));
    fstp(e, second + 0x24, product);
    for (to, from) in [(0x28, 0x204), (0x2c, 0x210), (0x30, 0x214)] {
        let value = fld(e, fp + from);
        fstp(e, second + to, value);
    }
}

/// The begin part of the motor atoms: the angular pair of the axis at `axis_at` into the record
/// at the cursor (the sum stored at `sum_at`) and `1 / (sum + FLT_EPSILON)` at `record + 0xc`
/// and the frame's `+0x140`, each step rounded through the float at `scratch`. Returns the
/// record.
fn motor_begin(
    e: &mut Engine,
    fp: u32,
    cursor: Ptr,
    axis_at: u32,
    sum_at: u32,
    scratch: u32,
) -> u32 {
    let record = e.mem.u32(cursor.addr());
    let sum = angular_pair(e, fp + SCHEMA, axis_at, record);
    e.mem.set_u32(sum_at, sum.to_bits());
    let mass = through_float(x87_add(x87_load32(sum.to_bits()), m64(e, EPSILON)));
    fstp(e, scratch, mass);
    let inverse = through_float(x87_div(1.0, mass));
    fstp(e, scratch, inverse);
    fstp(e, record + 0xc, inverse);
    fstp(e, fp + 0x140, inverse);
    record
}

/// Atom type 0x12 (`00d7081a`): an angular motor. `flag`, `value` are the byte and the float at
/// `counter + atom.short[2]`, `+ atom.short[3]`. Without a motor (`atom.word[4]`) or
/// `atom.byte[2]` the placeholder record (marker 1). Otherwise the angle about row `axis =
/// atom.byte[3]`: `atan2(next_B . third_A, (third_A x row_A) . next_B)`, moved by whole turns to
/// within pi of `-float at counter + atom.short[4] + 4` when that offset is not 0; `value` takes
/// it when `flag` is 0. The angular pair of row `axis` of block A, `hkCalcMotorData` (`{1 /
/// mass, schema, counter[0], counter[1], target - value, value - angle}`) and the two records
/// follow; `value`/`flag` become the target and 1 unless they are equal and `flag` is set.
fn atom_angular_motor(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let value = (e.mem.u16(atom + 6) as i16 as i32 as u32).wrapping_add(counter);
    e.mem.set_u32(fp + 0x34, value);
    let motor = e.mem.u32(atom + 0x10);
    let flag = (e.mem.u16(atom + 4) as i16 as i32 as u32).wrapping_add(counter);
    e.mem.set_u32(fp + 0x30, flag);
    e.mem.set_u32(fp + 0x5c, motor);
    if motor == 0 || e.mem.u8(atom + 2) == 0 {
        placeholder_from_stack(e, cursor, 1);
        return;
    }
    let axis = u32::from(e.mem.u8(atom + 3));
    let next = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis));
    let third = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis));
    let third_a = load_vec(e, fp + ROWS_A + third);
    let next_b = load_vec(e, fp + ROWS_B + next);
    let dot = sum3(mul(next_b, third_a));
    let row_a = load_vec(e, fp + ROWS_A + (axis << 4));
    e.mem.set_u32(fp + 0x2fc, dot.to_bits());
    let turned = sum3(mul(cross(third_a, row_a), next_b));
    e.mem.set_u32(fp + 0x2ac, turned.to_bits());
    let angle = atan2_of(e, dot, turned);
    fstp(e, fp + 0x2c, angle);
    let offset = e.mem.u16(atom + 8);
    let angle = if offset != 0 {
        let at = (offset as i16 as i32 as u32)
            .wrapping_add(e.mem.u32(fp + SCHEMA + 0x64))
            .wrapping_add(4);
        let limit = x87_mul(fld(e, at), m64(e, MINUS_ONE));
        fstp(e, fp + 0x10, limit);
        let limit = fld(e, fp + 0x10);
        wrap_turns(e, fp + 0x2c, limit)
    } else {
        fld(e, fp + 0x2c)
    };
    if e.mem.u8(flag) == 0 {
        fstp(e, value, angle);
    }
    fstp(e, fp + 0x148, 0.0);
    fstp(e, fp + 0x14c, 0.0);
    let axis_at = fp + ROWS_A + (axis << 4);
    e.mem.set_u32(fp + 0x10, axis_at);
    let record = motor_begin(e, fp, cursor, axis_at, fp + 0x23c, fp + 0x10);
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let target = fld(e, atom + 0xc);
    e.mem.set_u32(fp + 0x144, fp + SCHEMA);
    let bound = e.mem.u32(counter);
    e.mem.set_u32(fp + 0x148, bound);
    let bound = e.mem.u32(counter + 4);
    let remaining = x87_sub(target, m32(e, value));
    e.mem.set_u32(fp + 0x14c, bound);
    fstp(e, fp + 0x150, remaining);
    let passed = x87_sub(m32(e, value), angle);
    fstp(e, fp + 0x154, passed);
    e.call(CALC_MOTOR_DATA, &args![motor, fp + 0x140, fp + 0x200]);
    let weight = x87_mul(fld(e, fp + 0x200), m32(e, fp + SCHEMA + 8));
    fstp(e, record + 0x1c, weight);
    motor_commit_record(e, fp, cursor, true);
    let now = fld(e, value);
    let wanted = fld(e, atom + 0xc);
    if parity_even(compare_status(wanted, now), 0x44) || e.mem.u8(flag) == 0 {
        let wanted = fld(e, atom + 0xc);
        fstp(e, value, wanted);
        e.mem.set_u8(flag, 1);
    }
}

/// Atom type 0x13 (`00d70bff`): a ragdoll motor. Without `atom.byte[2]` the placeholder record
/// (marker 3). Otherwise the product of block A's matrix (`schema.word[0xf]`) and the atom's
/// (`+0x10`, `hkMatrix3::setMulMat3Mat3`) at the frame's `+0x1c0`, then for the three motors at
/// `atom + 0x40` (axes 0, 1, 2 with the other axes 1, 0, 0 from a frame table): a motor of 0
/// appends the placeholder (marker 1); otherwise the first angle (about block B's row plus block
/// A's, with block A's other row and block B's) and the second (about the product's row plus
/// block A's, with block A's other row and the product's); `values[i]` and `flags[i]` take the
/// first angle and 1 when the flag is 0; the angular pair of the second axis, `hkCalcMotorData`
/// (`counter[2 i]`, `counter[2 i + 1]`, second - value, value - first) and the two records
/// follow, and `values[i]` takes the second angle when they differ.
fn atom_ragdoll_motor(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let flags = (e.mem.u16(atom + 4) as i16 as i32 as u32).wrapping_add(counter);
    let values = (e.mem.u16(atom + 6) as i16 as i32 as u32).wrapping_add(counter);
    e.mem.set_u32(fp + 0x34, values);
    if e.mem.u8(atom + 2) == 0 {
        placeholder_from_stack(e, cursor, 3);
        return;
    }
    let matrix = e.mem.u32(fp + SCHEMA + 0x3c);
    e.call(SET_MUL_MAT3_MAT3, &args![fp + 0x1c0, matrix, atom + 0x10]);
    fstp(e, fp + 0x148, 0.0);
    fstp(e, fp + 0x14c, 0.0);
    e.mem.set_u32(fp + 0x144, fp + SCHEMA);
    for (index, (axis, other)) in [(0u32, 1u32), (1, 0), (2, 0)].into_iter().enumerate() {
        e.mem.set_u32(fp + 0x100 + 4 * index as u32, axis);
        e.mem.set_u32(fp + 0x110 + 4 * index as u32, other);
    }
    e.mem.set_u32(fp + 0x3c, 0);
    e.mem.set_u32(fp + 0x5c, flags);
    e.mem.set_u32(fp + 0x30, atom + 0x40);
    e.mem.set_u32(fp + 0x2c, 0);
    loop {
        let motor = e.mem.u32(e.mem.u32(fp + 0x30));
        e.mem.set_u32(fp + 0x38, motor);
        if motor == 0 {
            placeholder_loaded(e, cursor);
        } else {
            ragdoll_motor_axis(e, fp, cursor, motor);
        }
        let flag = e.mem.u32(fp + 0x5c).wrapping_add(1);
        e.mem.set_u32(fp + 0x5c, flag);
        let next = e.mem.u32(fp + 0x30).wrapping_add(4);
        e.mem.set_u32(fp + 0x30, next);
        let slot = e.mem.u32(fp + 0x2c).wrapping_add(4);
        e.mem.set_u32(fp + 0x2c, slot);
        let pair = e.mem.u32(fp + 0x3c).wrapping_add(8);
        e.mem.set_u32(fp + 0x3c, pair);
        if (pair as i32) >= 0x18 {
            break;
        }
    }
}

/// The unit axis `sum / |sum|` of the ragdoll motor when `|sum|` is above the double `1e-16`
/// (the length stored at `length_at`, the inverse at `inverse_at`), else `fallback`.
fn ragdoll_axis(
    e: &mut Engine,
    sum: Vec4,
    fallback: Vec4,
    length_at: u32,
    inverse_at: u32,
) -> Vec4 {
    let length = sum3(mul(sum, sum)).sqrt();
    e.mem.set_u32(length_at, length.to_bits());
    let loaded = x87_load32(length.to_bits());
    if compare_status(loaded, m64(e, TINY)) & 0x41 == 0 {
        let inverse = x87_store32(x87_div(1.0, loaded));
        e.mem.set_u32(inverse_at, inverse);
        mul([f32::from_bits(inverse); 4], sum)
    } else {
        fallback
    }
}

/// One motor of atom type 0x13 (`00d70cc2`..`00d71124`); the frame holds the motor's slot
/// (`+0x2c`), pair offset (`+0x3c`), flag pointer (`+0x5c`) and the values base (`+0x34`).
fn ragdoll_motor_axis(e: &mut Engine, fp: u32, cursor: Ptr, motor: u32) {
    let slot = e.mem.u32(fp + 0x2c);
    let other = e.mem.u32(fp + 0x110 + slot) << 4;
    let axis = e.mem.u32(fp + 0x100 + slot) << 4;
    let row_a = load_vec(e, fp + ROWS_A + axis);
    let sum = add(load_vec(e, fp + ROWS_B + axis), row_a);
    e.mem.set_u32(fp + 0x40, axis);
    let other_a = fp + ROWS_A + other;
    e.mem.set_u32(fp + 0x10, other_a);
    let normal = ragdoll_axis(e, sum, row_a, fp + 0x280, fp + 0x27c);
    let b = load_vec(e, other_a);
    let tangent = cross(normal, b);
    let binormal = cross(tangent, normal);
    let w = load_vec(e, fp + ROWS_B + other);
    let along = sum3(mul(w, tangent));
    let across = sum3(mul(w, binormal));
    e.mem.set_u32(fp + 0x244, across.to_bits());
    e.mem.set_u32(fp + 0x29c, along.to_bits());
    let first_angle = atan2_of(e, along, across);
    fstp(e, fp + 0x25c, first_angle);
    let axis = e.mem.u32(fp + 0x40);
    let row_a = load_vec(e, fp + ROWS_A + axis);
    let sum = add(load_vec(e, fp + 0x1c0 + axis), row_a);
    let normal = ragdoll_axis(e, sum, row_a, fp + 0x284, fp + 0x40);
    let product_other = fp + 0x1c0 + other;
    let b = load_vec(e, e.mem.u32(fp + 0x10));
    store_vec(e, fp + 0x380, normal);
    let tangent = cross(normal, b);
    let binormal = cross(tangent, normal);
    let w = load_vec(e, product_other);
    let along = sum3(mul(w, tangent));
    let across = sum3(mul(w, binormal));
    e.mem.set_u32(fp + 0x254, across.to_bits());
    e.mem.set_u32(fp + 0x2a0, along.to_bits());
    let second_angle = atan2_of(e, along, across);
    fstp(e, fp + 0x17c, second_angle);
    let flag = e.mem.u32(fp + 0x5c);
    let first = fld(e, fp + 0x25c);
    if e.mem.u8(flag) == 0 {
        let value_at = e.mem.u32(fp + 0x34).wrapping_add(e.mem.u32(fp + 0x2c));
        fstp(e, value_at, first);
        e.mem.set_u8(flag, 1);
    }
    let record = motor_begin(e, fp, cursor, fp + 0x380, fp + 0x28c, fp + 0x40);
    let pair = e.mem.u32(fp + 0x3c);
    let counter = e.mem.u32(fp + SCHEMA + 0x64);
    let second = fld(e, fp + 0x17c);
    let bound = e.mem.u32(pair.wrapping_add(counter));
    e.mem.set_u32(fp + 0x148, bound);
    let bound = e.mem.u32(pair.wrapping_add(counter).wrapping_add(4));
    let value_at = e.mem.u32(fp + 0x2c).wrapping_add(e.mem.u32(fp + 0x34));
    e.mem.set_u32(fp + 0x14c, bound);
    fstp(e, fp + 0x150, x87_sub(second, m32(e, value_at)));
    fstp(e, fp + 0x154, x87_sub(m32(e, value_at), first));
    e.call(CALC_MOTOR_DATA, &args![motor, fp + 0x140, fp + 0x200]);
    let weight = x87_mul(fld(e, fp + 0x200), m32(e, fp + SCHEMA + 8));
    let value_at = e.mem.u32(fp + 0x2c).wrapping_add(e.mem.u32(fp + 0x34));
    fstp(e, record + 0x1c, weight);
    motor_commit_record(e, fp, cursor, false);
    let now = fld(e, value_at);
    let second = fld(e, fp + 0x17c);
    if parity_even(compare_status(second, now), 0x44) {
        fstp(e, value_at, second);
    }
}

/// The unit direction of a length atom: `difference * inverse_length(|difference|^2)` when the
/// float at `0104da4c` is below the squared length, else `fallback` (the squared length is
/// stored at `squared_at`).
fn length_direction(e: &mut Engine, difference: Vec4, fallback: Vec4, squared_at: u32) -> Vec4 {
    let squares = mul(difference, difference);
    let squared = squares[2] + (squares[1] + squares[0]);
    e.mem.set_u32(squared_at, squared.to_bits());
    let small = fld(e, SMALL_FLOAT);
    if parity_even(compare_status(small, m32(e, squared_at)), 0x05) {
        fallback
    } else {
        mul([inverse_length(e, squared); 4], difference)
    }
}

/// Atom type 0x14 (`00d711ab`): a length record between pivot B and the atom's point
/// `atom.vector[1]` and between pivot A and `atom.vector[2]`, with the offset `atom.float[0xc]`
/// and the scale `s = atom.float[0xd]`. `n1` is the unit direction of `p1 - pivot B` (the first
/// axis of `01268370` when too short), `n2` that of `pivot A - p2` (the negated first axis);
/// the record gets `n1`, `s * n2`, the arms' cross products turned by the bodies' rows
/// (`(pivot B - centre A) x n1`, `(s n2) x (pivot A - centre B)` as `cross` forms them),
/// `schema.float[9] / (s^2 mass_B + mass_A + FLT_EPSILON + inertia)` at `+0x2c` and
/// `((s |pivot A - p2| + |p1 - pivot B|) - offset) * schema.float[2]` at `+0xc`; the first float
/// gets `FLT_EPSILON` added and the tag 0xb. The cursor moves on by 0x40.
fn atom_length(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let far_b = load_vec(e, atom + 0x20);
    let near = load_vec(e, atom + 0x10);
    let pivot_a = load_vec(e, fp + PIVOT_A);
    let offset = fld(e, atom + 0x30);
    let record = e.mem.u32(cursor.addr());
    fstp(e, fp + 0x570, offset);
    let scale = fld(e, atom + 0x34);
    store_vec(e, fp + 0x560, far_b);
    fstp(e, fp + 0x574, scale);
    let first_difference = sub(near, load_vec(e, fp + PIVOT_B));
    let second_difference = sub(pivot_a, far_b);
    let axis = load_vec(e, AXIS_TABLE);
    let first = length_direction(e, first_difference, axis, fp + 0x2a4);
    let negated_axis = negate(e, axis);
    let second = length_direction(e, second_difference, negated_axis, fp + 0x26c);
    let body_a = e.mem.u32(fp + SCHEMA + 0x30);
    let scale = fld(e, fp + 0x574);
    let body_b = e.mem.u32(fp + SCHEMA + 0x34);
    fstp(e, fp + 0x5f4, scale);
    let weight = fld(e, fp + SCHEMA + 8);
    let scale_lanes = [f32::from_bits(e.mem.u32(fp + 0x5f4)); 4];
    let pivot_b = load_vec(e, fp + PIVOT_B);
    fstp(e, fp + 0x5f8, weight);
    let pivot_a = load_vec(e, fp + PIVOT_A);
    let numerator = fld(e, fp + SCHEMA + 0x24);
    store_vec(e, record, first);
    fstp(e, fp + 0x5fc, numerator);
    let scaled = mul(scale_lanes, second);
    store_vec(e, record + 0x10, scaled);
    store_vec(e, fp + 0x590, pivot_b);
    let arm_a = sub(pivot_b, load_vec(e, body_a + 0x40));
    let arm_b = sub(pivot_a, load_vec(e, body_b + 0x40));
    let turn_a = cross(arm_a, first);
    let turn_b = cross(scaled, arm_b);
    let angular_a = rotate_rows(turn_a, body_rows(e, body_a));
    store_vec(e, record + 0x20, angular_a);
    let angular_b = rotate_rows(turn_b, body_rows(e, body_b));
    store_vec(e, record + 0x30, angular_b);
    let terms = add(
        mul(mul(angular_b, angular_b), load_vec(e, body_b + 0x30)),
        mul(mul(angular_a, angular_a), load_vec(e, body_a + 0x30)),
    );
    let inertia = sum3(terms);
    e.mem.set_u32(fp + 0x294, inertia.to_bits());
    let epsilon = epsilon_on_stack(e);
    let numerator = fld(e, fp + 0x5fc);
    fstp(e, fp + 0x38, x87_mul(scale, scale));
    let first_length = {
        let d = sub(near, load_vec(e, fp + 0x590));
        let s = mul(d, d);
        (s[2] + (s[1] + s[0])).sqrt()
    };
    e.mem.set_u32(fp + 0x2a8, first_length.to_bits());
    let mass = x87_mul(fld(e, fp + 0x38), m32(e, body_b + 0x3c));
    let mass = x87_add(x87_add(mass, m32(e, body_a + 0x3c)), epsilon);
    fstp(e, fp + 0x38, mass);
    let denominator = x87_add(fld(e, fp + 0x38), m32(e, fp + 0x294));
    fstp(e, fp + 0x38, denominator);
    let quotient = x87_div(numerator, m32(e, fp + 0x38));
    let second_length = {
        let d = sub(pivot_a, load_vec(e, fp + 0x560));
        let s = mul(d, d);
        (s[2] + (s[1] + s[0])).sqrt()
    };
    e.mem.set_u32(fp + 0x274, second_length.to_bits());
    fstp(e, record + 0x2c, quotient);
    let reach = x87_add(x87_mul(scale, m32(e, fp + 0x274)), m32(e, fp + 0x2a8));
    fstp(e, fp + 0x38, reach);
    let excess = x87_sub(fld(e, fp + 0x38), m32(e, fp + 0x570));
    fstp(e, fp + 0x38, excess);
    let error = x87_mul(fld(e, fp + 0x38), m32(e, fp + 0x5f8));
    fstp(e, record + 0xc, error);
    let record = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), record + 0x40);
    let sum = x87_add(epsilon, m32(e, record));
    fstp(e, record, sum);
    e.mem.set_u8(record, 0xb);
}

/// Copies the schema (`0x80` bytes) to the frame at `to`.
fn copy_schema(e: &mut Engine, fp: u32, to: u32) {
    for word in 0..0x20 {
        let value = e.mem.u32(fp + SCHEMA + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The first atom below a modifier: the chain through `+0x14` while the type is at least 0x17.
fn modifier_target(e: &Engine, atom: u32) -> u32 {
    let mut node = atom;
    loop {
        node = e.mem.u32(node + 0x14);
        if e.mem.u16(node) < 0x17 {
            return node;
        }
    }
}

/// The modifiers' child Jacobian: a contact atom (type 0x16) through
/// `hkSimpleContactConstraintDataBuildJacobian` with the frame word at `flag_at` (its low byte
/// set to `flag` first; the other bytes are whatever the frame holds), any other through
/// `hkSolverBuildJacobianFromAtomsNotContact` with `id`.
fn child_jacobian(
    e: &mut Engine,
    next: u32,
    id: u32,
    copy: u32,
    flag_at: u32,
    flag: u8,
    cursor: Ptr,
) {
    if e.mem.u16(next) == 0x16 {
        e.mem.set_u8(flag_at, flag);
        let word = e.mem.u32(flag_at);
        e.call(
            BUILD_CONTACT_JACOBIAN,
            &args![next, copy, word, cursor.addr()],
        );
    } else {
        e.call(
            BUILD_NOT_CONTACT_JACOBIAN,
            &args![next, id, copy, cursor.addr()],
        );
    }
}

/// One entry of atom type 0x17: `u = atom.float[9] * schema.float[1] * schema.float[0] /
/// atom.float[8] + entry` (each step through a float), stored at `target` when smaller.
fn lower_target(e: &mut Engine, atom: u32, fp: u32, entry: u32, target: u32) {
    let value = fld(e, entry);
    fstp(e, fp + 0x2c, value);
    let product = x87_mul(fld(e, atom + 0x24), m32(e, fp + SCHEMA + 4));
    let product = x87_mul(product, m32(e, fp + SCHEMA));
    let quotient = x87_div(product, m32(e, atom + 0x20));
    fstp(e, fp + 0x34, quotient);
    let lowered = x87_add(fld(e, fp + 0x34), m32(e, fp + 0x2c));
    if compare_status(fld(e, target), lowered) & 0x41 == 0 {
        fstp(e, target, lowered);
    }
}

/// Atom type 0x17 (`00d7189d`): backs the cursor up by 0x10, copies the schema to the frame's
/// `+0x600` with `float[9]`, `float[0x10]` and `float[0x11]` multiplied by `atom.float[8]`, lowers
/// the contact values of the atom's target (see `lower_target`; `count = word[2]`, the targets
/// `node + 0x30 + (word[3] << 5) + 0x10`, `stride = byte[0xa]`, four entries per pass, then the
/// rest) and builds the child Jacobian of `atom.word[5]` from the copy.
fn atom_contact_lowering(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    let scale = fld(e, atom + 0x20);
    fstp(e, fp + 0x38, scale);
    let copy = fp + 0x600;
    copy_schema(e, fp, copy);
    let scale = fld(e, fp + 0x38);
    fstp(e, copy + 0x24, x87_mul(scale, m32(e, fp + SCHEMA + 0x24)));
    fstp(e, copy + 0x40, x87_mul(scale, m32(e, fp + SCHEMA + 0x40)));
    let id = u32::from(e.mem.u16(atom + 0x12));
    e.mem.set_u32(fp + 0x38, id);
    fstp(e, copy + 0x44, x87_mul(scale, m32(e, fp + SCHEMA + 0x44)));
    let node = modifier_target(e, atom);
    let count = u32::from(e.mem.u16(node + 4));
    let skip = u32::from(e.mem.u16(node + 6));
    let stride = u32::from(e.mem.u8(node + 0xa));
    e.mem.set_u32(fp + 0x3c, count);
    let mut target = (skip << 5).wrapping_add(node).wrapping_add(0x30);
    let mut done = 0u32;
    if (count as i32) >= 4 {
        let groups = ((count - 4) >> 2) + 1;
        done = groups * 4;
        let mut entry = node + 0x6c;
        for _ in 0..groups {
            for step in 0..4u32 {
                let at = entry.wrapping_sub(0x20).wrapping_add(0x20 * step);
                lower_target(e, atom, fp, at, target.wrapping_add(0x10));
                target = target.wrapping_add(stride);
            }
            entry = entry.wrapping_add(0x80);
        }
    }
    e.mem.set_u32(fp + 0x30, done);
    if (done as i32) < (count as i32) {
        let mut entry = (done << 5).wrapping_add(node).wrapping_add(0x4c);
        target = target.wrapping_add(0x10);
        for _ in done..count {
            lower_target(e, atom, fp, entry, target);
            target = target.wrapping_add(stride);
            entry = entry.wrapping_add(0x20);
        }
    }
    let next = e.mem.u32(atom + 0x14);
    child_jacobian(e, next, id, copy, fp + 0x240, 1, cursor);
}

/// Atom type 0x18 (`00d71b99`): copies the schema to `+0x700` and both bodies' solver data
/// (`fn_00d71f90`, to `+0x4b0` and `+0x3b0`) with their vectors at `+0x30` multiplied by the
/// atom's vectors at `+0x20` and `+0x30`; the copy's bodies point to them. The vector-pair
/// record of the scaled vectors (`00dced90`), the child Jacobian of `atom.word[5]` and the
/// vector-pair record of the schema bodies' own vectors at `+0x30` follow.
fn atom_body_scaling(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let copy = fp + 0x700;
    copy_schema(e, fp, copy);
    let next = e.mem.u32(atom + 0x14);
    let id = u32::from(e.mem.u16(atom + 0x12));
    copy_vec(e, atom + 0x20, fp + 0x340);
    copy_vec(e, atom + 0x30, fp + 0x330);
    let (copy_a, copy_b) = (fp + 0x4b0, fp + 0x3b0);
    let body_a = e.mem.u32(fp + SCHEMA + 0x30);
    fn_00d71f90(e, Ptr::new(copy_a), Ptr::new(body_a));
    let body_b = e.mem.u32(fp + SCHEMA + 0x34);
    fn_00d71f90(e, Ptr::new(copy_b), Ptr::new(body_b));
    let scaled_a = mul(load_vec(e, copy_a + 0x30), load_vec(e, fp + 0x340));
    e.mem.set_u32(copy + 0x30, copy_a);
    store_vec(e, copy_a + 0x30, scaled_a);
    let scaled_b = mul(load_vec(e, copy_b + 0x30), load_vec(e, fp + 0x330));
    e.mem.set_u32(copy + 0x34, copy_b);
    store_vec(e, copy_b + 0x30, scaled_b);
    e.call(
        PUSH_VECTOR_PAIR,
        &args![copy_a + 0x30, copy_b + 0x30, cursor.addr()],
    );
    child_jacobian(e, next, id, copy, fp + 0x250, 1, cursor);
    let body_b = e.mem.u32(fp + SCHEMA + 0x34);
    let body_a = e.mem.u32(fp + SCHEMA + 0x30);
    e.call(
        PUSH_VECTOR_PAIR,
        &args![body_a + 0x30, body_b + 0x30, cursor.addr()],
    );
}

/// Atom type 0x19 (`00d71afa`): backs the cursor up by 0x10, copies the schema to `+0x780`, adds
/// a little surface viscosity to the target's contact data (`node + 0x10`) and builds the child
/// Jacobian of `atom.word[5]` from the copy.
fn atom_surface_viscosity(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    let copy = fp + 0x780;
    copy_schema(e, fp, copy);
    let id = u32::from(e.mem.u16(atom + 0x12));
    let node = modifier_target(e, atom);
    e.call(ADD_SURFACE_VISCOSITY, &args![node + 0x10]);
    let next = e.mem.u32(atom + 0x14);
    child_jacobian(e, next, id, copy, fp + 0x248, 1, cursor);
}

/// Atom type 0x1a (`00d71d0e`): a moving surface. The atom's velocity `+0x20` goes to the
/// frame's `+0xe0`; the schema is copied to `+0x800`; the cursor backs up by 0x10 and the header
/// record of the schema (`fn_00d6ea30`, value `node + 0x30 + (word[3] << 5)`, flag
/// `node.byte[0xa]`, `node = atom.word[5]`) replaces the last record. A copy (`00d72030`, at
/// `+0x3b0`) of body B (when `schema.word[0x1b]` is 0; the velocity negated in its first three
/// lanes) or else of body A replaces that body in the dispatcher's schema from here on. With
/// the node's normal `+0x40`, `y = -(v - (n . v) n)`: the velocity record of `y`, the copy's
/// vector `+0x10` plus `y`, the node's contact Jacobian (flag word low byte 0) and the velocity
/// record of `-y` follow.
fn atom_moving_surface(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    copy_vec(e, atom + 0x20, fp + 0xe0);
    let copy = fp + 0x800;
    copy_schema(e, fp, copy);
    let node = e.mem.u32(atom + 0x14);
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    let skip = u32::from(e.mem.u16(node + 6));
    let flag = e.mem.u8(node + 0xa);
    let value = (skip << 5).wrapping_add(node).wrapping_add(0x30);
    fn_00d6ea30(
        e,
        Ptr::new(fp + SCHEMA),
        cursor,
        f32::from_bits(value),
        flag,
    );
    let second_body = e.mem.u32(fp + SCHEMA + 0x6c) == 0;
    e.mem.set_u8(fp + 0x5b, u8::from(second_body));
    let body_copy = fp + 0x3b0;
    let velocity = if second_body {
        let body = e.mem.u32(fp + SCHEMA + 0x34);
        e.call(COPY_BODY_DATA, &args![body_copy, body]);
        let mask = load_bits(e, SIGN_MASK_XYZ);
        let velocity = load_vec(e, fp + 0xe0);
        let flipped: Vec4 =
            std::array::from_fn(|lane| f32::from_bits(velocity[lane].to_bits() ^ mask[lane]));
        e.mem.set_u32(fp + SCHEMA + 0x34, body_copy);
        store_vec(e, fp + 0xe0, flipped);
        flipped
    } else {
        let body = e.mem.u32(fp + SCHEMA + 0x30);
        e.call(COPY_BODY_DATA, &args![body_copy, body]);
        let velocity = load_vec(e, fp + 0xe0);
        e.mem.set_u32(fp + SCHEMA + 0x30, body_copy);
        velocity
    };
    copy_vec(e, node + 0x40, fp + 0x350);
    let normal = load_vec(e, fp + 0x350);
    let flag_word = u32::from(second_body);
    let along = sum3(mul(normal, velocity));
    e.mem.set_u32(fp + 0x38, flag_word);
    let rest = sub(velocity, mul([along; 4], normal));
    let pushed = negate(e, rest);
    store_vec(e, fp + 0xf0, pushed);
    e.call(ADD_VELOCITY, &args![fp + 0xf0, flag_word, cursor.addr()]);
    let moved = add(load_vec(e, fp + 0xf0), load_vec(e, body_copy + 0x10));
    e.mem.set_u8(fp + 0x258, 0);
    let contact_flag = e.mem.u32(fp + 0x258);
    store_vec(e, body_copy + 0x10, moved);
    e.call(
        BUILD_CONTACT_JACOBIAN,
        &args![node, copy, contact_flag, cursor.addr()],
    );
    let flag_word = e.mem.u32(fp + 0x38);
    let opposite = negate(e, load_vec(e, fp + 0xf0));
    store_vec(e, fp + 0xf0, opposite);
    e.call(ADD_VELOCITY, &args![fp + 0xf0, flag_word, cursor.addr()]);
}

/// Atom type 0x1c (`00d71578`): a centre-of-mass changer. Copies the schema to `+0x680`; the
/// atom's vectors `+0x20` and `+0x30` turned by the matrices `schema.word[0xe]` and
/// `schema.word[0xf]` (`hkVector4::setRotatedDir`, to `+0x370` and `+0x390`); body copies
/// (`fn_00d71f90`, A at `+0x4b0`, B at `+0x3b0`) with the turned vectors added to their
/// `+0x40` vectors, the copy's bodies pointing to them; `m1`, `m2` (`+0x140`, `+0x200`) the
/// skew-symmetric matrices of the turned vectors times the inverse of each copy's rows at
/// `+0x50` (`hkMatrix3::setMulInverse`, from a scratch copy at `+0x300`); copies of `m1`, `m2`
/// (`+0x1c0`, `+0x440`) with the rows scaled by `(-1, 0, 0, 0)` (the float at `01012054`) applied
/// to each copy's `+0x20` vector (`hkVector4::setMul3`, to `+0x4a0` and `+0x430`) and added to its
/// `+0x10` vector. The centre-of-mass record of the scaled copies, the child Jacobian of
/// `atom.word[5]` and the centre-of-mass record of `m1`, `m2` follow.
fn atom_centre_of_mass(e: &mut Engine, atom: u32, fp: u32, cursor: Ptr) {
    let copy = fp + 0x680;
    copy_schema(e, fp, copy);
    let next = e.mem.u32(atom + 0x14);
    let id = u32::from(e.mem.u16(atom + 0x12));
    let (turned_a, turned_b) = (fp + 0x370, fp + 0x390);
    let matrix_a = e.mem.u32(fp + SCHEMA + 0x38);
    e.call(SET_ROTATED_DIR, &args![turned_a, matrix_a, atom + 0x20]);
    let matrix_b = e.mem.u32(fp + SCHEMA + 0x3c);
    e.call(SET_ROTATED_DIR, &args![turned_b, matrix_b, atom + 0x30]);
    let (copy_a, copy_b) = (fp + 0x4b0, fp + 0x3b0);
    let body_a = e.mem.u32(fp + SCHEMA + 0x30);
    fn_00d71f90(e, Ptr::new(copy_a), Ptr::new(body_a));
    let body_b = e.mem.u32(fp + SCHEMA + 0x34);
    fn_00d71f90(e, Ptr::new(copy_b), Ptr::new(body_b));
    let moved_a = add(load_vec(e, turned_a), load_vec(e, copy_a + 0x40));
    store_vec(e, copy_a + 0x40, moved_a);
    let moved_b = add(load_vec(e, turned_b), load_vec(e, copy_b + 0x40));
    e.mem.set_u32(copy + 0x34, copy_b);
    store_vec(e, copy_b + 0x40, moved_b);
    e.mem.set_u32(copy + 0x30, copy_a);
    let (m1, m2, scratch) = (fp + 0x140, fp + 0x200, fp + 0x300);
    e.call(SET_CROSS_SKEW_SYMMETRIC, &args![m1, turned_a]);
    e.call(SET_CROSS_SKEW_SYMMETRIC, &args![m2, turned_b]);
    for (matrix, body_copy) in [(m1, copy_a), (m2, copy_b)] {
        for row in 0..3 {
            copy_vec(e, matrix + 0x10 * row, scratch + 0x10 * row);
        }
        e.call(SET_MUL_INVERSE, &args![matrix, scratch, body_copy + 0x50]);
    }
    let minus_one = e.mem.u32(MINUS_ONE_FLOAT);
    let (scaled_1, scaled_2) = (fp + 0x1c0, fp + 0x440);
    for (matrix, scaled, factor) in [(m1, scaled_1, fp + 0x480), (m2, scaled_2, fp + 0x470)] {
        for row in 0..3 {
            copy_vec(e, matrix + 0x10 * row, scaled + 0x10 * row);
        }
        store_bits(e, factor, [minus_one, 0, 0, 0]);
        e.call(SCALE_ROWS, &args![scaled, factor]);
    }
    let (product_a, product_b) = (fp + 0x4a0, fp + 0x430);
    e.call(SET_MUL3, &args![product_a, scaled_1, copy_a + 0x20]);
    e.call(SET_MUL3, &args![product_b, scaled_2, copy_b + 0x20]);
    let linear_a = add(load_vec(e, copy_a + 0x10), load_vec(e, product_a));
    store_vec(e, copy_a + 0x10, linear_a);
    let linear_b = add(load_vec(e, copy_b + 0x10), load_vec(e, product_b));
    store_vec(e, copy_b + 0x10, linear_b);
    e.call(
        SET_CENTER_OF_MASS,
        &args![scaled_1, scaled_2, cursor.addr()],
    );
    child_jacobian(e, next, id, copy, fp + 0x234, 1, cursor);
    e.call(SET_CENTER_OF_MASS, &args![m1, m2, cursor.addr()]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d6_f460,
            hk_solver_build_jacobian_from_atoms_not_contact(Ptr, u32, Ptr, Ptr)
        ),
        entry!(0x00d6_ced0, fn_00d6ced0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d6_c5a0,
            build_jacobian_from_viscous_surface_modifier(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d6_c630, fn_00d6c630(Ptr, Ptr, Ptr, Ptr, f32, f32)),
        entry!(0x00d6_c970, fn_00d6c970(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_cb30, fn_00d6cb30(Ptr, Ptr, Ptr)),
        entry!(0x00d6_cc80, fn_00d6cc80(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d6_cd70,
            hk1d_angular_velocity_motor_begin_jacobian(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_ce70,
            hk1d_angular_velocity_motor_commit_jacobian(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d6_d220, fn_00d6d220(Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x00d6_d280, fn_00d6d280(Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x00d6_d990, fn_00d6d990(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d6_ea30, fn_00d6ea30(Ptr, Ptr, f32, u8)),
        entry!(0x00d6_f2b0, fn_00d6f2b0(Ptr, Ptr, f32, u8)),
        entry!(0x00d6_d2e0, fn_00d6d2e0(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d6_d450, fn_00d6d450(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d6_d5d0, fn_00d6d5d0(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d6_d720, fn_00d6d720(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d6_da00,
            build_jacobian_from_ang_motor_atom(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_dd60,
            build_jacobian_from_ragdoll_motor_atom(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d6_e280, fn_00d6e280(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_e300, fn_00d6e300(Ptr, Ptr, Ptr)),
        entry!(0x00d6_e6b0, fn_00d6e6b0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_ea70, fn_00d6ea70(Ptr, Ptr, Ptr)),
        entry!(0x00d6_ecb0, fn_00d6ecb0(Ptr, Ptr, Ptr)),
        entry!(
            0x00d6_edf0,
            build_jacobian_from_center_of_mass_changer_modifier(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_f0c0,
            build_jacobian_from_moving_surface_modifier(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_f300,
            build_jacobian_from_lin_motor_atom(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d7_1f90, fn_00d71f90(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range, `FLT_EPSILON` and `FLT_MAX` as doubles, the three
    /// axes and the all-lanes sign mask.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, f64::from(f32::EPSILON));
        e.set_global(MAX_FLOAT, f64::from(f32::MAX));
        for axis in 0..3u32 {
            let mut vector = [0.0f32; 4];
            vector[axis as usize] = 1.0;
            store_vec(&mut e, AXIS_TABLE + 0x10 * axis, vector);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e
    }

    /// A body: inverse inertia at `+0x30`, centre of mass at `+0x40`, identity rotation rows.
    fn body(e: &mut Engine, position: Vec4, inverse: Vec4, angular: Vec4) -> u32 {
        let body = e.mem.alloc(0x80);
        store_vec(e, body + 0x20, angular);
        store_vec(e, body + 0x30, inverse);
        store_vec(e, body + 0x40, position);
        store_vec(e, body + 0x50, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, body + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, body + 0x70, [0.0, 0.0, 1.0, 0.0]);
        body
    }

    /// An info block with weight 2, slope 5 and two bodies (inverse inertias 1 and 2, w lanes
    /// 0.5; B turning about z when `spin` is set).
    fn info(e: &mut Engine, spin: bool) -> u32 {
        let first = body(e, [0.0; 4], [1.0, 1.0, 1.0, 0.5], [0.0; 4]);
        let angular = if spin { [0.0, 0.0, 1.0, 0.0] } else { [0.0; 4] };
        let second = body(e, [0.0; 4], [2.0, 2.0, 2.0, 0.5], angular);
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 0x20, 2.0);
        e.mem.set_f32(info + 0x24, 5.0);
        e.mem.set_u32(info + 0x30, first);
        e.mem.set_u32(info + 0x34, second);
        info
    }

    fn point(e: &mut Engine, value: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, value);
        address
    }

    fn cell(e: &mut Engine, value: u32) -> u32 {
        let address = e.mem.alloc(8);
        e.mem.set_u32(address, value);
        address
    }

    #[test]
    fn plain_axis_records_carry_the_axis_the_angular_terms_and_the_mass() {
        let mut e = engine();
        let info = info(&mut e, false);
        let a = point(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = point(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let record = e.mem.alloc(0x90);
        fn_00d6c970(
            &mut e,
            Ptr::new(a),
            Ptr::new(b),
            Ptr::new(info),
            Ptr::new(record),
        );
        let first = record;
        assert_eq!(e.mem.f32(first + 0xc), 4.0);
        assert_eq!(e.mem.u32(first + 0x2c), 0x3f80_0001);
        assert_eq!(
            e.mem.f32(first + 0x1c),
            (5.0f64 / f64::from(f32::from_bits(0x3f80_0001))) as f32
        );
        let second = record + 0x30;
        assert_eq!(load_vec(&e, second + 0x10)[..3], [0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, second + 0x20)[..3], [0.0, 0.0, -3.0]);
        assert_eq!(e.mem.f32(second + 0x2c), 20.0);
        assert_eq!(e.mem.f32(second + 0x1c), 0.25);
        assert_eq!(e.mem.f32(second + 0xc), 0.0);
        let third = record + 0x60;
        assert_eq!(load_vec(&e, third + 0x10)[..3], [0.0, -1.0, 0.0]);
        assert_eq!(load_vec(&e, third + 0x20)[..3], [0.0, 3.0, 0.0]);
    }

    fn rotate_double(e: &mut Engine) {
        e.register_double(ROTATE_ANGULAR, |e, a| {
            let vector = load_vec(e, a[2]);
            store_vec(e, a[0], vector);
            Ret::default()
        });
    }

    fn counter_double(e: &mut Engine) {
        e.register_double(INTERLOCKED_EXCHANGE_ADD, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old + a[1]);
            Ret {
                eax: old,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn limit_records_add_the_rotation_error_and_tag_the_limit() {
        let mut e = engine();
        rotate_double(&mut e);
        counter_double(&mut e);
        let info = info(&mut e, true);
        e.mem.set_f32(info + 0x50, 1.0);
        e.mem.set_u32(info + 0x60, 0x1234);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0x68, list);
        let a = point(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = point(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let record = e.mem.alloc(0x90);
        let cursor = cell(&mut e, record);
        fn_00d6c630(
            &mut e,
            Ptr::new(info),
            Ptr::new(a),
            Ptr::new(b),
            Ptr::new(cursor),
            f32::MAX,
            4.0,
        );
        assert_eq!(e.mem.u32(cursor), record + 0x90);
        assert_eq!(e.mem.u32(list), 1);
        assert_eq!(e.mem.u32(list + 4), 0x1234);
        assert_eq!(e.mem.f32(record + 0xc), 4.0);
        assert_eq!(e.mem.f32(record + 0x3c), -12.0);
        assert_eq!(e.mem.f32(record + 0x6c), 0.0);
        assert_eq!(e.mem.u32(record), 0x3f80_0005);
        assert_eq!(e.mem.u32(record + 0x30), 0x3400_0005);
        assert_eq!(e.mem.u8(record + 0x60), 5);
        e.mem.set_f32(info + 0x50, 100.0);
        e.mem.set_u32(cursor, record);
        fn_00d6c630(
            &mut e,
            Ptr::new(info),
            Ptr::new(a),
            Ptr::new(b),
            Ptr::new(cursor),
            7.5,
            4.0,
        );
        assert_eq!(e.mem.u32(list), 1);
        assert_eq!(e.mem.u8(record), 6);
        assert_eq!(e.mem.f32(record + 0x2c), 7.5);
        assert_eq!(e.mem.f32(record + 0x5c), 7.5);
    }

    #[test]
    fn the_angular_record_scales_the_motor_terms() {
        let mut e = engine();
        let info = info(&mut e, false);
        e.mem.set_f32(info + 8, 1.5);
        let input = e.mem.alloc(0x40);
        store_vec(&mut e, input, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(input + 0x10, 2.0);
        e.mem.set_f32(input + 0x14, 3.0);
        e.mem.set_f32(input + 0x18, 4.0);
        e.mem.set_f32(input + 0x1c, 0.5);
        let record = e.mem.alloc(0x40);
        let cursor = cell(&mut e, record);
        fn_00d6cb30(&mut e, Ptr::new(input), Ptr::new(info), Ptr::new(cursor));
        assert_eq!(e.mem.u32(cursor), record + 0x30);
        assert_eq!(e.mem.u32(record), 0x3f80_000d);
        assert_eq!(load_vec(&e, record + 0x10)[..3], [-1.0, 0.0, 0.0]);
        let total = (3.0f64 + f64::from(f32::EPSILON)) as f32;
        assert_eq!(e.mem.f32(record + 0xc), (0.5f64 / f64::from(total)) as f32);
        assert_eq!(e.mem.f32(record + 0x1c), -6.0);
        assert_eq!(e.mem.f32(record + 0x20), -1.5);
        assert_eq!(e.mem.f32(record + 0x24), -2.25);
        assert_eq!(e.mem.f32(record + 0x28), 0.5);
    }

    #[test]
    fn both_angular_motor_begin_functions_store_the_inverse_mass() {
        let mut e = engine();
        let info = info(&mut e, false);
        let vector = point(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let total = (3.0f64 + f64::from(f32::EPSILON)) as f32;
        let inverse = (1.0f64 / f64::from(total)) as f32;
        for (index, registers) in [true, false].into_iter().enumerate() {
            let record = e.mem.alloc(0x20);
            let out = e.mem.alloc(8);
            if registers {
                fn_00d6cc80(
                    &mut e,
                    Ptr::new(info),
                    Ptr::new(record),
                    Ptr::new(vector),
                    Ptr::new(out),
                );
            } else {
                hk1d_angular_velocity_motor_begin_jacobian(
                    &mut e,
                    Ptr::new(vector),
                    Ptr::new(info),
                    Ptr::new(record),
                    Ptr::new(out),
                );
            }
            assert_eq!(
                load_vec(&e, record)[..3],
                [1.0, 0.0, 0.0],
                "variant {index}"
            );
            assert_eq!(load_vec(&e, record + 0x10)[..3], [-1.0, 0.0, 0.0]);
            assert_eq!(e.mem.f32(record + 0xc), inverse);
            assert_eq!(e.mem.f32(out), inverse);
        }
    }

    #[test]
    fn the_commit_record_copies_the_motor_terms() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        for (index, value) in [2.0f32, 7.0, 3.0, 4.0, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(block + 4 * index as u32, value);
        }
        let state = e.mem.alloc(0x10);
        e.mem.set_f32(state + 4, 0.5);
        e.mem.set_f32(state + 8, 10.0);
        let begin = e.mem.alloc(0x40);
        let record = e.mem.alloc(0x40);
        e.mem.set_f32(record, 1.0);
        let cursor = cell(&mut e, record);
        hk1d_angular_velocity_motor_commit_jacobian(
            &mut e,
            Ptr::new(block),
            Ptr::new(state),
            Ptr::new(begin),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.f32(begin + 0x1c), 20.0);
        assert_eq!(e.mem.u32(cursor), record + 0x40);
        assert_eq!(e.mem.u32(record), 0x3f80_000f);
        let tail: Vec<f32> = [0x20, 0x24, 0x28, 0x2c, 0x30]
            .into_iter()
            .map(|offset| e.mem.f32(record + offset))
            .collect();
        assert_eq!(tail, vec![1.5, 2.0, 7.0, 8.0, 9.0]);
    }

    #[test]
    fn the_linear_input_blocks_gather_the_vectors_and_floats() {
        for (callee, builder) in [
            (
                LINEAR_BILATERAL_USER_TAU,
                fn_00d6d220 as fn(&mut Engine, Ptr, Ptr, u32, Ptr, u32),
            ),
            (LINEAR_LIMIT, fn_00d6d280),
        ] {
            let mut e = engine();
            let log: Log = Rc::default();
            let sink = Rc::clone(&log);
            e.register_double(callee, move |e, a| {
                let mut words = a[1..3].to_vec();
                words.extend((0..14).map(|n| e.mem.u32(a[0] + 4 * n)));
                sink.borrow_mut().push((callee, words));
                Ret::default()
            });
            let bodies = e.mem.alloc(0x80);
            store_vec(&mut e, bodies + 0x30, [1.0, 2.0, 3.0, 4.0]);
            store_vec(&mut e, bodies + 0x20, [9.0, 9.5, 9.25, 9.75]);
            let other = e.mem.alloc(0x40);
            store_vec(&mut e, other + 0x30, [5.0, 6.0, 7.0, 8.0]);
            let atom = e.mem.alloc(0x10);
            e.mem.set_u8(atom + 2, 2);
            e.mem.set_f32(atom + 4, 0.25);
            e.mem.set_f32(atom + 8, 0.75);
            builder(
                &mut e,
                Ptr::new(atom),
                Ptr::new(bodies),
                0x111,
                Ptr::new(other),
                0x222,
            );
            let floats = [
                5.0f32, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 9.0, 9.5, 9.25, 9.75, 0.25, 0.75,
            ];
            let mut expected = vec![0x111, 0x222];
            expected.extend(floats.iter().map(|value| value.to_bits()));
            assert_eq!(*log.borrow(), vec![(callee, expected)]);
        }
    }

    #[test]
    fn friction_calls_the_builder_for_a_nonzero_value_and_otherwise_writes_a_record() {
        let mut e = engine();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(ANGULAR_FRICTION, move |e, a| {
            let mut words = a[1..3].to_vec();
            words.extend((0..4).map(|n| e.mem.u32(a[0] + 4 * n)));
            sink.borrow_mut().push((a[0], words));
            Ret::default()
        });
        let atom = e.mem.alloc(0x10);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 1);
        e.mem.set_u8(atom + 4, 6);
        e.mem.set_f32(atom + 8, 0.3);
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, 0xabcd);
        let record = e.mem.alloc(0x20);
        let cursor = cell(&mut e, record);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        {
            let calls = log.borrow();
            assert_eq!(calls.len(), 1);
            assert_eq!(
                calls[0].1,
                vec![info, cursor, 0x4000_0010, 0xabcd, 0.3f32.to_bits(), 6]
            );
        }
        assert_eq!(e.mem.u32(cursor), record);
        e.mem.set_u32(atom + 8, 0x7fa0_0000);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        assert_eq!(log.borrow()[1].1[4], 0x7fe0_0000);
        e.mem.set_f32(atom + 8, 0.0);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u32(record), 0x3400_0003);
        assert_eq!(e.mem.u8(record + 4), 6);
        e.mem.set_u8(atom + 2, 0);
        e.mem.set_f32(atom + 8, 1.0);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(cursor), record + 0x20);
    }

    #[test]
    fn both_header_records_hold_the_ids_the_value_and_the_flag() {
        let mut e = engine();
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x60, 0xaa);
        e.mem.set_u32(info + 0x6c, 0x1111_2222);
        e.mem.set_u32(info + 0x70, 0x3333_4444);
        for plain in [true, false] {
            let record = e.mem.alloc(0x20);
            e.mem.set_f32(record, 2.0);
            let cursor = cell(&mut e, record);
            if plain {
                fn_00d6f2b0(&mut e, Ptr::new(info), Ptr::new(cursor), 1.5, 7);
            } else {
                fn_00d6ea30(&mut e, Ptr::new(info), Ptr::new(cursor), 1.5, 7);
            }
            assert_eq!(e.mem.u32(cursor), record + 0x10);
            assert_eq!(e.mem.u8(record), 1);
            assert_eq!(e.mem.u8(record + 1), 7);
            assert_eq!(e.mem.u16(record + 4), 0x2222);
            assert_eq!(e.mem.u16(record + 6), 0x4444);
            assert_eq!(e.mem.u32(record + 8), 1.5f32.to_bits());
            assert_eq!(e.mem.u32(record + 0xc), 0xaa);
        }
    }

    #[test]
    fn the_viscosity_modifier_builds_the_next_atom_from_a_copy_of_the_info_block() {
        for contact in [false, true] {
            let mut e = engine();
            let log: Log = Rc::default();
            let source = e.mem.alloc(0x80);
            for word in 0..0x20 {
                e.mem.set_u32(source + 4 * word, 0x100 + word);
            }
            for (callee, tag) in [
                (ADD_SURFACE_VISCOSITY, 1u32),
                (BUILD_CONTACT_JACOBIAN, 2),
                (BUILD_NOT_CONTACT_JACOBIAN, 3),
            ] {
                let sink = Rc::clone(&log);
                e.register_double(callee, move |e, a| {
                    let mut words = vec![tag];
                    words.extend_from_slice(&a[..4.min(a.len())]);
                    if tag != 1 {
                        let copy = if tag == 2 { a[1] } else { a[2] };
                        words.push((0..0x20).all(|n| e.mem.u32(copy + 4 * n) == 0x100 + n) as u32);
                    }
                    sink.borrow_mut().push((callee, words));
                    Ret::default()
                });
            }
            let atom = e.mem.alloc(0x20);
            let first = e.mem.alloc(0x40);
            let second = e.mem.alloc(0x40);
            e.mem.set_u16(atom + 0x12, 0x77);
            e.mem.set_u32(atom + 0x14, first);
            e.mem.set_u16(first, if contact { 0x16 } else { 0x30 });
            e.mem.set_u32(first + 0x14, second);
            e.mem.set_u16(second, 5);
            let cursor = cell(&mut e, 0x5000);
            build_jacobian_from_viscous_surface_modifier(
                &mut e,
                Ptr::new(source),
                Ptr::new(atom),
                Ptr::new(cursor),
            );
            assert_eq!(e.mem.u32(cursor), 0x4ff0);
            let calls = log.borrow();
            if contact {
                assert_eq!(calls[0], (ADD_SURFACE_VISCOSITY, vec![1, first + 0x10]));
                assert_eq!(calls[1].0, BUILD_CONTACT_JACOBIAN);
                assert_eq!(calls[1].1[1], first);
                assert_eq!((calls[1].1[3], calls[1].1[4]), (1, cursor));
                assert_eq!(calls[1].1[5], 1);
            } else {
                assert_eq!(calls[0], (ADD_SURFACE_VISCOSITY, vec![1, second + 0x10]));
                assert_eq!(calls[1].0, BUILD_NOT_CONTACT_JACOBIAN);
                assert_eq!((calls[1].1[1], calls[1].1[2]), (first, 0x77));
                assert_eq!(calls[1].1[4], cursor);
                assert_eq!(calls[1].1[5], 1);
            }
        }
    }
}

#[cfg(test)]
mod continuation_tests {
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range and the constants the translated functions read.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, f64::from(f32::EPSILON));
        e.set_global(SMALL_FLOAT, f32::EPSILON);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(PI, f64::from_bits(0x4009_21fb_6000_0000));
        e.set_global(TWO_PI, f64::from_bits(0x4019_21fb_6000_0000));
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(TINY, 1e-16f64);
        e.set_global(LIMIT_SCALE, 0.25f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(
                SIGN_MASK_XYZ + 4 * lane,
                if lane < 3 { 0x8000_0000 } else { 0 },
            );
        }
        for axis in 0..3u32 {
            let mut vector = [0.0f32; 4];
            vector[axis as usize] = 1.0;
            store_vec(&mut e, AXIS_TABLE + 0x10 * axis, vector);
        }
        for (index, offset) in [0x10u8, 0x20, 0x00, 0x10].into_iter().enumerate() {
            e.mem.set_u8(NEXT_AXIS_TABLE + index as u32, offset);
        }
        // Index 1 of the angle tables: offset 0.5, sign -1.0 and scale 2.0.
        e.mem.set_f32(ANGLE_OFFSET_TABLE + 4, 0.5);
        e.mem.set_f32(AXIS_SIGN_TABLE - 4, -1.0);
        e.mem.set_f32(ANGLE_SCALE_TABLE + 4, 2.0);
        e
    }

    fn cell(e: &mut Engine, value: u32) -> u32 {
        let address = e.mem.alloc(8);
        e.mem.set_u32(address, value);
        address
    }

    /// Three rows of 16 bytes: the identity.
    fn identity(e: &mut Engine) -> u32 {
        let rows = e.mem.alloc(0x30);
        for axis in 0..3u32 {
            let mut row = [0.0f32; 4];
            row[axis as usize] = 1.0;
            store_vec(e, rows + 0x10 * axis, row);
        }
        rows
    }

    /// A body: inverse inertia (`w` 0.5) at `+0x30`, position at `+0x40`, identity rotation.
    fn body(e: &mut Engine, inverse: [f32; 3], position: Vec4) -> u32 {
        let body = e.mem.alloc(0x80);
        store_vec(e, body + 0x30, [inverse[0], inverse[1], inverse[2], 0.5]);
        store_vec(e, body + 0x40, position);
        for axis in 0..3u32 {
            let mut row = [0.0f32; 4];
            row[axis as usize] = 1.0;
            store_vec(e, body + 0x50 + 0x10 * axis, row);
        }
        body
    }

    /// An info block with weight 2, slope 5, motor factors 0.5 (`+4`) and 1.5 (`+8`) and two
    /// bodies (inverse inertias 1 and 2).
    fn info(e: &mut Engine) -> u32 {
        let first = body(e, [1.0; 3], [0.0; 4]);
        let second = body(e, [2.0; 3], [0.0; 4]);
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 4, 0.5);
        e.mem.set_f32(info + 8, 1.5);
        e.mem.set_f32(info + 0x20, 2.0);
        e.mem.set_f32(info + 0x24, 5.0);
        e.mem.set_u32(info + 0x30, first);
        e.mem.set_u32(info + 0x34, second);
        info
    }

    /// Logs the argument words of every call to `address`.
    fn spy(e: &mut Engine, address: u32, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    /// The argument words of the calls to `address`, in order.
    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// `hkMath_atan2fApproximation` returning `angle` and logging its two words.
    fn atan2_double(e: &mut Engine, log: &Log, angle: f64) {
        let sink = Rc::clone(log);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            sink.borrow_mut().push((ATAN2_APPROXIMATION, a.to_vec()));
            Ret {
                st0: angle,
                ..Ret::default()
            }
        });
    }

    /// `hkCalcMotorData` writing `out` and logging the input block's six words.
    fn motor_double(e: &mut Engine, log: &Log, out: [f32; 6]) {
        let sink = Rc::clone(log);
        e.register_double(CALC_MOTOR_DATA, move |e, a| {
            let mut words = vec![a[0]];
            words.extend((0..6).map(|n| e.mem.u32(a[1] + 4 * n)));
            sink.borrow_mut().push((CALC_MOTOR_DATA, words));
            for (index, value) in out.into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * index as u32, value);
            }
            Ret::default()
        });
    }

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1e-5
    }

    #[test]
    fn the_angular_limit_record_wraps_the_angle_into_the_limit() {
        let two_pi = f64::from_bits(0x4019_21fb_6000_0000);
        for (angle, expected) in [(0.5f64, 0.5f32), (5.0, (5.0 - two_pi) as f32)] {
            let mut e = engine();
            let log = Log::default();
            atan2_double(&mut e, &log, angle);
            let info = e.mem.alloc(0x80);
            e.mem.set_f32(info + 0x40, 2.0);
            let limits = e.mem.alloc(0x20);
            e.mem.set_f32(limits + 4, 1.0);
            e.mem.set_u32(info + 0x64, limits);
            let atom = e.mem.alloc(0x20);
            e.mem.set_u8(atom + 2, 1);
            e.mem.set_f32(atom + 4, 0.25);
            e.mem.set_f32(atom + 8, 0.75);
            e.mem.set_f32(atom + 0xc, 3.0);
            let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
            let record = e.mem.alloc(0x40);
            let result = fn_00d6d2e0(
                &mut e,
                Ptr::new(atom),
                Ptr::new(record),
                Ptr::new(info),
                Ptr::new(rows_a),
                Ptr::new(rows_b),
            );
            assert_eq!(result, 0);
            assert_eq!(
                calls(&log, ATAN2_APPROXIMATION),
                vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
            );
            assert_eq!(load_vec(&e, record)[..3], [1.0, 0.0, 0.0]);
            assert_eq!(e.mem.f32(record + 0x10), 0.25);
            assert_eq!(e.mem.f32(record + 0x14), 0.75);
            assert_eq!(e.mem.f32(record + 0x18), expected);
            assert_eq!(e.mem.f32(record + 0x1c), 6.0);
        }
    }

    #[test]
    fn the_angular_limit_record_is_skipped_without_the_limit_byte() {
        let mut e = engine();
        let atom = e.mem.alloc(0x20);
        let (info, record) = (e.mem.alloc(0x80), e.mem.alloc(0x40));
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let result = fn_00d6d2e0(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 1);
        assert_eq!(e.mem.u32(record + 0x1c), 0);
    }

    #[test]
    fn the_summed_axis_record_normalises_the_sum_or_keeps_the_second_axis() {
        for (second_axis, normal, half_weight) in [
            ([1.0f32, 0.0, 0.0], [1.0f32, 0.0, 0.0], 8.0f32),
            ([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.0),
        ] {
            let mut e = engine();
            let log = Log::default();
            atan2_double(&mut e, &log, 0.5);
            let info = e.mem.alloc(0x80);
            e.mem.set_f32(info + 0x40, 2.0);
            let atom = e.mem.alloc(0x20);
            e.mem.set_u8(atom + 3, 0);
            e.mem.set_u8(atom + 4, 1);
            e.mem.set_f32(atom + 8, 0.25);
            e.mem.set_f32(atom + 0xc, 0.75);
            e.mem.set_f32(atom + 0x10, 4.0);
            let (base_a, base_b) = (identity(&mut e), identity(&mut e));
            store_vec(
                &mut e,
                base_b,
                [second_axis[0], second_axis[1], second_axis[2], 0.0],
            );
            let record = e.mem.alloc(0x40);
            let result = fn_00d6d450(
                &mut e,
                Ptr::new(base_a),
                Ptr::new(atom),
                Ptr::new(record),
                Ptr::new(info),
                Ptr::new(base_b),
            );
            assert_eq!(result, 0);
            assert_eq!(
                calls(&log, ATAN2_APPROXIMATION),
                vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
            );
            assert_eq!(load_vec(&e, record)[..3], normal);
            assert_eq!(e.mem.f32(record + 0x10), 0.25);
            assert_eq!(e.mem.f32(record + 0x14), 0.75);
            assert_eq!(e.mem.f32(record + 0x18), 0.5);
            assert_eq!(e.mem.f32(record + 0x1c), half_weight);
        }
    }

    /// The doubles and data of the two axis-angle records: the CRT square root, the angle
    /// 2.0 and two bases whose axes `atom.byte[3]` / `atom.byte[4]` are x and y (or both x).
    fn angle_record_setup(parallel: bool) -> (Engine, Log, [u32; 5]) {
        let mut e = engine();
        let log = Log::default();
        atan2_double(&mut e, &log, 2.0);
        e.register_double(X87_SQRT, |_, a| {
            let mut next = 0;
            Ret {
                st0: f64::take(a, &mut next).sqrt(),
                ..Ret::default()
            }
        });
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 0x40, 2.0);
        e.mem.set_f32(info + 0xc, 0.5);
        let limits = e.mem.alloc(0x20);
        e.mem.set_f32(limits + 4, -3.0);
        e.mem.set_f32(limits + 8, 4.0);
        e.mem.set_u32(info + 0x64, limits);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u8(atom + 3, 0);
        e.mem.set_u8(atom + 4, if parallel { 0 } else { 1 });
        e.mem.set_u8(atom + 5, 1);
        e.mem.set_f32(atom + 8, 0.25);
        e.mem.set_f32(atom + 0xc, 0.75);
        e.mem.set_f32(atom + 0x10, 4.0);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x40);
        (e, log, [atom, record, info, rows_a, rows_b])
    }

    #[test]
    fn the_two_axis_record_scales_the_angle_and_signs_the_axis() {
        let (mut e, log, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        let result = fn_00d6d5d0(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 0);
        assert_eq!(
            calls(&log, ATAN2_APPROXIMATION),
            vec![vec![1.0f32.to_bits(), 0.0f32.to_bits()]]
        );
        assert_eq!(load_vec(&e, record)[..3], [0.0, 0.0, -1.0]);
        assert_eq!(e.mem.f32(record + 0x10), 0.25);
        assert_eq!(e.mem.f32(record + 0x14), 0.75);
        assert_eq!(e.mem.f32(record + 0x18), 4.5);
        assert_eq!(e.mem.f32(record + 0x1c), 8.0);
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(true);
        let result = fn_00d6d5d0(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 1);
        assert_eq!(e.mem.u32(record + 0x18), 0);
    }

    #[test]
    fn the_limit_variant_lowers_the_bound_by_the_angle_overshoot() {
        // slot 0: nothing more than the record.
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        let result = fn_00d6d720(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 0);
        assert_eq!(e.mem.f32(record + 0x18), 4.5);
        let limits = e.mem.u32(info + 0x64);
        assert_eq!(e.mem.f32(limits + 8), 4.0);
        // slot 8, limit 3 (the record's angle stays below the blend): bound 3 and -1.
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        e.mem.set_u8(atom + 6, 8);
        let limits = e.mem.u32(info + 0x64);
        fn_00d6d720(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        // a = 4, x = 4.5 - 4 = 0.5, c = 3, y = 2.5, M = max(0.25, 1) = 1.
        assert_eq!(e.mem.f32(limits + 8), 3.0);
        assert_eq!(e.mem.f32(record + 0x18), 1.5);
        // limit -3 -> c = -(-3) = 3 above; with limits[1] = 5, c = -5, y = -5.5: the blend
        // 0.5 * (0.25 + 2) = 1.125 is below 5.5, so both bounds move by y.
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        e.mem.set_u8(atom + 6, 8);
        let limits = e.mem.u32(info + 0x64);
        e.mem.set_f32(limits + 4, 5.0);
        fn_00d6d720(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(e.mem.f32(limits + 8), 8.5);
        assert_eq!(e.mem.f32(record + 0x18), -4.0);
    }

    /// An angular motor atom: the flag byte at `limits + 0x10`, the value at `limits + 0x14`,
    /// target 3.0, a motor.
    fn motor_atom(e: &mut Engine) -> u32 {
        let atom = e.mem.alloc(0x60);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 0);
        e.mem.set_u16(atom + 4, 0x10);
        e.mem.set_u16(atom + 6, 0x14);
        e.mem.set_f32(atom + 0xc, 3.0);
        e.mem.set_u32(atom + 0x10, 0xabc);
        atom
    }

    #[test]
    fn the_angular_motor_atom_builds_the_motor_record_from_the_motor_data() {
        let mut e = engine();
        let log = Log::default();
        atan2_double(&mut e, &log, 0.75);
        motor_double(&mut e, &log, [0.0, 7.0, 2.0, 4.0, 8.0, 9.0]);
        let info = info(&mut e);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits, 0x1111);
        e.mem.set_u32(limits + 4, 0x2222);
        e.mem.set_u32(info + 0x64, limits);
        let atom = motor_atom(&mut e);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        build_jacobian_from_ang_motor_atom(
            &mut e,
            Ptr::new(info),
            Ptr::new(atom),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(
            calls(&log, ATAN2_APPROXIMATION),
            vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        let total = (3.0f64 + f64::from(f32::EPSILON)) as f32;
        let inverse = (1.0f64 / f64::from(total)) as f32;
        // remaining = 3.0 - 0.75, passed = 0.75 - 0.75.
        assert_eq!(
            calls(&log, CALC_MOTOR_DATA),
            vec![vec![
                0xabc,
                inverse.to_bits(),
                info,
                0x1111,
                0x2222,
                2.25f32.to_bits(),
                0.0f32.to_bits()
            ]]
        );
        assert_eq!(e.mem.u32(cursor), record + 0x40);
        assert_eq!(e.mem.u8(record), 0xf);
        assert_eq!(e.mem.f32(record + 0xc), inverse);
        assert_eq!(e.mem.f32(record + 0x1c), 6.0);
        assert_eq!(e.mem.f32(record + 0x20), 1.0);
        assert_eq!(e.mem.f32(record + 0x24), 2.0);
        assert_eq!(e.mem.f32(record + 0x28), 7.0);
        assert_eq!(e.mem.f32(record + 0x2c), 8.0);
        assert_eq!(e.mem.f32(record + 0x30), 9.0);
        assert_eq!(e.mem.f32(limits + 0x14), 3.0);
        assert_eq!(e.mem.u8(limits + 0x10), 1);
    }

    #[test]
    fn the_angular_motor_atom_without_a_motor_appends_the_placeholder() {
        let mut e = engine();
        let info = info(&mut e);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x64, limits);
        let atom = motor_atom(&mut e);
        e.mem.set_u32(atom + 0x10, 0);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        build_jacobian_from_ang_motor_atom(
            &mut e,
            Ptr::new(info),
            Ptr::new(atom),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
    }

    #[test]
    fn the_ragdoll_motor_atom_builds_a_record_per_motor() {
        let mut e = engine();
        let log = Log::default();
        atan2_double(&mut e, &log, 0.5);
        motor_double(&mut e, &log, [0.0, 7.0, 2.0, 4.0, 8.0, 9.0]);
        // The product matrix is the identity.
        e.register_double(SET_MUL_MAT3_MAT3, |e, a| {
            for axis in 0..3u32 {
                let mut row = [0.0f32; 4];
                row[axis as usize] = 1.0;
                store_vec(e, a[0] + 0x10 * axis, row);
            }
            Ret::default()
        });
        let info = info(&mut e);
        let limits = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, limits);
        let atom = e.mem.alloc(0x60);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u16(atom + 4, 0x20);
        e.mem.set_u16(atom + 6, 0x30);
        e.mem.set_u32(atom + 0x40, 0xaaa);
        e.mem.set_u32(atom + 0x48, 0xbbb);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x100);
        let cursor = cell(&mut e, record);
        build_jacobian_from_ragdoll_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u32(cursor), record + 0x90);
        assert_eq!(e.mem.u8(record), 0xf);
        assert_eq!(e.mem.u8(record + 0x40), 3);
        assert_eq!(e.mem.u8(record + 0x44), 1);
        assert_eq!(e.mem.u8(record + 0x50), 0xf);
        let motors: Vec<u32> = calls(&log, CALC_MOTOR_DATA)
            .iter()
            .map(|words| words[0])
            .collect();
        assert_eq!(motors, vec![0xaaa, 0xbbb]);
        assert_eq!(calls(&log, ATAN2_APPROXIMATION).len(), 4);
        // flag and value of the motors 0 and 2.
        assert_eq!(e.mem.u8(limits + 0x20), 1);
        assert_eq!(e.mem.u8(limits + 0x22), 1);
        assert_eq!(e.mem.f32(limits + 0x30), 0.5);
        assert_eq!(e.mem.f32(limits + 0x38), 0.5);
        assert_eq!(e.mem.f32(record + 0x1c), 6.0);
        // Without the atom's byte `+2` only the placeholder with the marker 3 is made.
        e.mem.set_u8(atom + 2, 0);
        e.mem.set_u32(cursor, record);
        build_jacobian_from_ragdoll_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 3);
    }

    #[test]
    fn the_friction_atom_passes_a_block_or_appends_the_placeholder() {
        let mut e = engine();
        let log = Log::default();
        let sink = Rc::clone(&log);
        e.register_double(LINEAR_FRICTION, move |e, a| {
            let mut words = vec![a[1], a[2]];
            words.extend((0..10).map(|n| e.mem.u32(a[0] + 4 * n)));
            sink.borrow_mut().push((LINEAR_FRICTION, words));
            Ret::default()
        });
        let bodies = e.mem.alloc(0x40);
        store_vec(&mut e, bodies + 0x30, [1.0, 2.0, 3.0, 4.0]);
        let rows = e.mem.alloc(0x40);
        store_vec(&mut e, rows + 0x10, [5.0, 6.0, 7.0, 8.0]);
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, 0x7777);
        let atom = e.mem.alloc(0x10);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 1);
        e.mem.set_f32(atom + 4, 0.5);
        let record = e.mem.alloc(0x20);
        let cursor = cell(&mut e, record);
        fn_00d6e280(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(cursor),
            Ptr::new(rows),
            Ptr::new(bodies),
        );
        let expected: Vec<u32> = [info, cursor]
            .into_iter()
            .chain([1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.5].map(f32::to_bits))
            .chain([0x7777])
            .collect();
        assert_eq!(calls(&log, LINEAR_FRICTION), vec![expected]);
        assert_eq!(e.mem.u32(cursor), record);
        // A friction of 0 is not above 0: the placeholder.
        e.mem.set_f32(atom + 4, 0.0);
        fn_00d6e280(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(cursor),
            Ptr::new(rows),
            Ptr::new(bodies),
        );
        assert_eq!(calls(&log, LINEAR_FRICTION).len(), 1);
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
    }

    /// The points `p0..p3` of the length records: the arm of the first body (0, 1, 0), the
    /// origin, and the far ends (3, 1, 0) and (0, -4, 0). The bodies sit at the origin with
    /// the inverse masses 0.5, and the offset and the scale are 1 and 2.
    fn spring_points() -> [Vec4; 4] {
        [
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [3.0, 1.0, 0.0, 0.0],
            [0.0, -4.0, 0.0, 0.0],
        ]
    }

    #[test]
    fn the_length_record_holds_the_directions_arms_and_the_stretch() {
        let mut e = engine();
        let points = spring_points();
        let info = info(&mut e);
        let block = e.mem.alloc(0x50);
        for (index, point) in points.into_iter().enumerate() {
            store_vec(&mut e, block + 0x10 * index as u32, point);
        }
        e.mem.set_f32(block + 0x40, 1.0);
        e.mem.set_f32(block + 0x44, 2.0);
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        fn_00d6e300(&mut e, Ptr::new(block), Ptr::new(info), Ptr::new(cursor));
        assert_eq!(e.mem.u32(cursor), record + 0x40);
        assert_eq!(e.mem.u8(record), 0xb);
        let first = load_vec(&e, record);
        assert!(near(first[1], 0.0) && near(first[2], 0.0));
        let second = load_vec(&e, record + 0x10);
        assert!(near(second[0], 0.0) && near(second[1], 2.0) && near(second[2], 0.0));
        let arm = load_vec(&e, record + 0x20);
        assert!(near(arm[0], 0.0) && near(arm[1], 0.0) && near(arm[2], -1.0));
        assert_eq!(load_vec(&e, record + 0x30)[..3], [0.0, 0.0, 0.0]);
        // mass terms 2 * 2 * 0.5 + 0.5 + FLT_EPSILON, inertia 1: 5 / 3.5.
        assert!(near(e.mem.f32(record + 0x2c), 5.0 / 3.5));
        // ((2 * 4 + 3) - 1) * 1.5.
        assert_eq!(e.mem.f32(record + 0xc), 15.0);
    }

    #[test]
    fn the_length_record_of_three_blocks_matches_the_one_block_form() {
        let mut e = engine();
        let points = spring_points();
        let info = info(&mut e);
        let block = e.mem.alloc(0x50);
        for (index, point) in points.into_iter().enumerate() {
            store_vec(&mut e, block + 0x10 * index as u32, point);
        }
        e.mem.set_f32(block + 0x40, 1.0);
        e.mem.set_f32(block + 0x44, 2.0);
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        fn_00d6e300(&mut e, Ptr::new(block), Ptr::new(info), Ptr::new(cursor));
        let expected: Vec<u32> = (0..0x10).map(|n| e.mem.u32(record + 4 * n)).collect();
        let first = e.mem.alloc(0x40);
        store_vec(&mut e, first + 0x10, points[2]);
        store_vec(&mut e, first + 0x20, points[3]);
        e.mem.set_f32(first + 0x30, 1.0);
        e.mem.set_f32(first + 0x34, 2.0);
        let second = e.mem.alloc(0x40);
        store_vec(&mut e, second + 0x30, points[0]);
        let third = e.mem.alloc(0x40);
        store_vec(&mut e, third + 0x30, points[1]);
        let other = e.mem.alloc(0x80);
        let other_cursor = cell(&mut e, other);
        fn_00d6e6b0(
            &mut e,
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(info),
            Ptr::new(third),
            Ptr::new(other_cursor),
        );
        let actual: Vec<u32> = (0..0x10).map(|n| e.mem.u32(other + 4 * n)).collect();
        assert_eq!(actual, expected);
        assert_eq!(e.mem.u32(other_cursor), other + 0x40);
    }

    #[test]
    fn the_length_record_falls_back_to_the_x_axis_for_coincident_points() {
        let mut e = engine();
        let info = info(&mut e);
        let block = e.mem.alloc(0x50);
        e.mem.set_f32(block + 0x44, 3.0);
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        fn_00d6e300(&mut e, Ptr::new(block), Ptr::new(info), Ptr::new(cursor));
        assert_eq!(load_vec(&e, record + 0x10)[..3], [-3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, record)[1..3], [0.0, 0.0]);
        assert_eq!(e.mem.f32(record + 0xc), 0.0);
    }

    /// An atom chain for the modifiers: the atom, with the next node of type `kind`.
    fn modifier_atom(e: &mut Engine, kind: u16) -> (u32, u32) {
        let node = e.mem.alloc(0x100);
        e.mem.set_u16(node, kind);
        let atom = e.mem.alloc(0x80);
        e.mem.set_u16(atom + 0x12, 0x1234);
        e.mem.set_u32(atom + 0x14, node);
        (atom, node)
    }

    #[test]
    fn the_lowering_modifier_scales_the_info_copy_and_caps_the_node_entries() {
        for (kind, contact) in [(0x16u16, true), (0x10, false)] {
            let mut e = engine();
            let log = Log::default();
            for callee in [BUILD_CONTACT_JACOBIAN, BUILD_NOT_CONTACT_JACOBIAN] {
                let sink = Rc::clone(&log);
                e.register_double(callee, move |e, a| {
                    let copy = if callee == BUILD_CONTACT_JACOBIAN {
                        a[1]
                    } else {
                        a[2]
                    };
                    let mut words = a.to_vec();
                    words.extend([0x24, 0x40, 0x44].map(|offset| e.mem.u32(copy + offset)));
                    sink.borrow_mut().push((callee, words));
                    Ret::default()
                });
            }
            let info = e.mem.alloc(0x80);
            for (offset, value) in [
                (0, 2.0f32),
                (4, 3.0),
                (0x24, 5.0),
                (0x40, 7.0),
                (0x44, 11.0),
            ] {
                e.mem.set_f32(info + offset, value);
            }
            let (atom, node) = modifier_atom(&mut e, kind);
            e.mem.set_f32(atom + 0x20, 0.5);
            e.mem.set_f32(atom + 0x24, 0.25);
            e.mem.set_u16(node + 4, 2);
            e.mem.set_u16(node + 6, 1);
            e.mem.set_u8(node + 0xa, 0x20);
            e.mem.set_f32(node + 0x4c, 1.0);
            e.mem.set_f32(node + 0x6c, 10.0);
            e.mem.set_f32(node + 0x60, 100.0);
            e.mem.set_f32(node + 0x80, 5.0);
            let record = e.mem.alloc(0x40);
            let cursor = cell(&mut e, record + 0x10);
            fn_00d6ea70(&mut e, Ptr::new(info), Ptr::new(atom), Ptr::new(cursor));
            assert_eq!(e.mem.u32(cursor), record);
            // 0.25 * 3 * 2 / 0.5 = 3: the entries 3 + 1 and 3 + 10.
            assert_eq!(e.mem.f32(node + 0x60), 4.0);
            assert_eq!(e.mem.f32(node + 0x80), 5.0);
            let scaled = [2.5f32, 3.5, 5.5].map(f32::to_bits);
            if contact {
                let [call] = calls(&log, BUILD_CONTACT_JACOBIAN).try_into().unwrap();
                assert_eq!([call[0], call[2], call[3]], [node, 1, cursor]);
                assert_eq!(call[4..], scaled);
                assert!(calls(&log, BUILD_NOT_CONTACT_JACOBIAN).is_empty());
            } else {
                let [call] = calls(&log, BUILD_NOT_CONTACT_JACOBIAN).try_into().unwrap();
                assert_eq!([call[0], call[1], call[3]], [node, 0x1234, cursor]);
                assert_eq!(call[4..], scaled);
            }
        }
    }

    #[test]
    fn the_scaling_modifier_wraps_the_jacobian_between_two_vector_records() {
        let mut e = engine();
        let log = Log::default();
        let sink = Rc::clone(&log);
        e.register_double(PUSH_VECTOR_PAIR, move |e, a| {
            let mut words = a.to_vec();
            for pointer in &a[..2] {
                words.extend((0..4).map(|n| e.mem.u32(pointer + 4 * n)));
            }
            sink.borrow_mut().push((PUSH_VECTOR_PAIR, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BUILD_CONTACT_JACOBIAN, move |e, a| {
            let mut words = a.to_vec();
            words.extend([e.mem.u32(a[1] + 0x30), e.mem.u32(a[1] + 0x34)]);
            sink.borrow_mut().push((BUILD_CONTACT_JACOBIAN, words));
            Ret::default()
        });
        let info = e.mem.alloc(0x80);
        let body_a = e.mem.alloc(0x80);
        let body_b = e.mem.alloc(0x80);
        store_vec(&mut e, body_a + 0x30, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, body_b + 0x30, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u32(info + 0x30, body_a);
        e.mem.set_u32(info + 0x34, body_b);
        let (atom, node) = modifier_atom(&mut e, 0x16);
        e.mem.set_u16(atom + 0x12, 0x0207);
        store_vec(&mut e, atom + 0x20, [2.0; 4]);
        store_vec(&mut e, atom + 0x30, [3.0; 4]);
        let cursor = cell(&mut e, 0x5000);
        fn_00d6ecb0(&mut e, Ptr::new(atom), Ptr::new(info), Ptr::new(cursor));
        let pairs = calls(&log, PUSH_VECTOR_PAIR);
        assert_eq!(pairs.len(), 2);
        let words = |values: [f32; 8]| values.map(f32::to_bits).to_vec();
        assert_eq!(pairs[0][2], cursor);
        assert_eq!(
            pairs[0][3..],
            words([2.0, 4.0, 6.0, 8.0, 15.0, 18.0, 21.0, 24.0])[..]
        );
        assert_eq!(pairs[1][..3], [body_a + 0x30, body_b + 0x30, cursor]);
        assert_eq!(
            pairs[1][3..],
            words([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])[..]
        );
        let [contact] = calls(&log, BUILD_CONTACT_JACOBIAN).try_into().unwrap();
        assert_eq!([contact[0], contact[2], contact[3]], [node, 0x0201, cursor]);
        assert_eq!(contact[4], pairs[0][0] - 0x30);
        assert_eq!(contact[5], pairs[0][1] - 0x30);
        assert_eq!(load_vec(&e, body_a + 0x30), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn the_body_copy_leaves_the_flag_bytes_and_quiets_the_float() {
        let mut e = engine();
        let source = e.mem.alloc(0x80);
        let target = e.mem.alloc(0x80);
        for word in 0..0x20 {
            e.mem.set_u32(source + 4 * word, 0x1000_0000 + word);
            e.mem.set_u32(target + 4 * word, 0xaaaa_aaaa);
        }
        e.mem.set_u32(source + 8, 0x7f80_0001);
        let result = fn_00d71f90(&mut e, Ptr::new(target), Ptr::new(source));
        assert_eq!(result.addr(), target);
        assert_eq!(e.mem.u8(target), 0);
        assert_eq!(e.mem.u32(target + 4), 0x1000_0001);
        assert_eq!(e.mem.u32(target + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(target) >> 8, 0xaa_aaaa);
        assert_eq!(e.mem.u32(target + 0xc), 0xaaaa_aaaa);
        for word in 4..0x20 {
            assert_eq!(e.mem.u32(target + 4 * word), 0x1000_0000 + word, "{word}");
        }
    }

    #[test]
    fn the_moving_surface_modifier_replaces_one_body_and_brackets_the_jacobian() {
        for second_body in [true, false] {
            let mut e = engine();
            let log = Log::default();
            e.register_double(COPY_BODY_DATA, |e, a| {
                for word in 0..0x20 {
                    let value = e.mem.u32(a[1] + 4 * word);
                    e.mem.set_u32(a[0] + 4 * word, value);
                }
                Ret::default()
            });
            let sink = Rc::clone(&log);
            e.register_double(ADD_VELOCITY, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|n| e.mem.u32(a[0] + 4 * n)));
                sink.borrow_mut().push((ADD_VELOCITY, words));
                Ret::default()
            });
            let sink = Rc::clone(&log);
            e.register_double(BUILD_CONTACT_JACOBIAN, move |e, a| {
                let mut words = a.to_vec();
                words.extend([e.mem.u32(a[1] + 0x30), e.mem.u32(a[1] + 0x34)]);
                sink.borrow_mut().push((BUILD_CONTACT_JACOBIAN, words));
                Ret::default()
            });
            let info = e.mem.alloc(0x80);
            let body_a = e.mem.alloc(0x80);
            let body_b = e.mem.alloc(0x80);
            store_vec(&mut e, body_a + 0x10, [10.0; 4]);
            store_vec(&mut e, body_b + 0x10, [20.0; 4]);
            e.mem.set_u32(info + 0x30, body_a);
            e.mem.set_u32(info + 0x34, body_b);
            e.mem.set_u32(info + 0x60, 0x1234_5678);
            e.mem.set_u32(info + 0x6c, u32::from(!second_body));
            let (atom, node) = modifier_atom(&mut e, 0x16);
            store_vec(&mut e, atom + 0x20, [1.0, 2.0, 3.0, 4.0]);
            e.mem.set_u16(node + 6, 1);
            e.mem.set_u8(node + 0xa, 3);
            store_vec(&mut e, node + 0x40, [0.0, 1.0, 0.0, 0.0]);
            let record = e.mem.alloc(0x40);
            let cursor = cell(&mut e, record + 0x10);
            build_jacobian_from_moving_surface_modifier(
                &mut e,
                Ptr::new(atom),
                Ptr::new(info),
                Ptr::new(cursor),
            );
            // The header record replaces the last record.
            assert_eq!(e.mem.u32(cursor), record + 0x10);
            assert_eq!(e.mem.u8(record), 1);
            assert_eq!(e.mem.u8(record + 1), 3);
            assert_eq!(e.mem.u32(record + 8), node + 0x50);
            assert_eq!(e.mem.u32(record + 0xc), 0x1234_5678);
            let (kept, replaced) = if second_body {
                (info + 0x30, info + 0x34)
            } else {
                (info + 0x34, info + 0x30)
            };
            assert_eq!(e.mem.u32(kept), if second_body { body_a } else { body_b });
            let buffer = e.mem.u32(replaced);
            assert_ne!(buffer, body_a);
            assert_ne!(buffer, body_b);
            // Direction: x, y, z negated for the second body; its part along y is removed.
            let sign = if second_body { -1.0 } else { 1.0 };
            let rest = [sign * 1.0, 0.0, sign * 3.0, 4.0];
            let negated = rest.map(|lane| -lane);
            let velocities = calls(&log, ADD_VELOCITY);
            assert_eq!(velocities.len(), 2);
            let flag = u32::from(second_body);
            assert_eq!(velocities[0][1], flag);
            assert_eq!(velocities[0][3..], negated.map(f32::to_bits));
            assert_eq!(velocities[1][1], flag);
            assert_eq!(velocities[1][3..], rest.map(f32::to_bits));
            let original = if second_body { 20.0 } else { 10.0 };
            let moved = load_vec(&e, buffer + 0x10);
            assert_eq!(moved, negated.map(|lane| lane + original));
            let [contact] = calls(&log, BUILD_CONTACT_JACOBIAN).try_into().unwrap();
            assert_eq!(
                [contact[0], contact[2], contact[3]],
                [node, 0x1234_5600, cursor]
            );
            // The copy handed on still has both original bodies.
            assert_eq!(contact[4..], [body_a, body_b]);
        }
    }

    #[test]
    fn the_linear_motor_atom_builds_the_begin_block_and_commits() {
        let mut e = engine();
        let log = Log::default();
        let sink = Rc::clone(&log);
        e.register_double(LINEAR_VELOCITY_BEGIN, move |e, a| {
            sink.borrow_mut().push((LINEAR_VELOCITY_BEGIN, a.to_vec()));
            let seen = [e.mem.u32(a[4] + 8), e.mem.u32(a[4] + 0xc)];
            sink.borrow_mut().push((0, seen.to_vec()));
            e.mem.set_f32(a[4], 0.125);
            Ret::default()
        });
        motor_double(&mut e, &log, [0.0, 7.0, 2.0, 4.0, 8.0, 9.0]);
        spy(&mut e, LINEAR_VELOCITY_COMMIT, &log);
        let info = e.mem.alloc(0x80);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits, 0x1111);
        e.mem.set_u32(limits + 4, 0x2222);
        e.mem.set_u32(info + 0x64, limits);
        let first = e.mem.alloc(0x40);
        store_vec(&mut e, first + 0x30, [4.0, 0.0, 0.0, 0.0]);
        let second = e.mem.alloc(0x40);
        store_vec(&mut e, second + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, second + 0x10, [1.0, 0.0, 0.0, 0.0]);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 1);
        e.mem.set_u16(atom + 4, 0x10);
        e.mem.set_u16(atom + 6, 0x14);
        e.mem.set_f32(atom + 8, 2.5);
        e.mem.set_u32(atom + 0xc, 0xbeef);
        let record = e.mem.alloc(0x40);
        let cursor = cell(&mut e, record);
        build_jacobian_from_lin_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(cursor),
        );
        let [begin] = calls(&log, LINEAR_VELOCITY_BEGIN).try_into().unwrap();
        assert_eq!(begin[..4], [second + 0x10, first + 0x30, info, record]);
        // The begin function sees zeros in the block's words 2 and 3.
        assert_eq!(calls(&log, 0), vec![vec![0, 0]]);
        // remaining = 2.5 - 2.5, passed = 2.5 - (4 - 1) . x.
        let [motor] = calls(&log, CALC_MOTOR_DATA).try_into().unwrap();
        assert_eq!(motor[0], 0xbeef);
        assert_eq!(motor[1], 0.125f32.to_bits());
        assert_eq!(
            motor[2..],
            [info, 0x1111, 0x2222, 0.0f32.to_bits(), (-0.5f32).to_bits()]
        );
        let [commit] = calls(&log, LINEAR_VELOCITY_COMMIT).try_into().unwrap();
        assert_eq!(commit[1..], [info, record, cursor]);
        assert_eq!(e.mem.f32(limits + 0x14), 2.5);
        assert_eq!(e.mem.u8(limits + 0x10), 1);
        // No motor: the placeholder.
        e.mem.set_u32(atom + 0xc, 0);
        build_jacobian_from_lin_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
        assert_eq!(e.mem.u32(cursor), record + 0x10);
    }

    #[test]
    fn the_center_of_mass_changer_moves_the_body_copies_and_brackets_the_jacobian() {
        let mut e = engine();
        let log = Log::default();
        e.register_double(SET_ROTATED_DIR, |e, a| {
            copy_vec(e, a[2], a[0]);
            Ret::default()
        });
        e.register_double(SET_CROSS_SKEW_SYMMETRIC, |e, a| {
            let source = load_vec(e, a[1]);
            for row in 0..3 {
                store_vec(e, a[0] + 0x10 * row, [source[row as usize]; 4]);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SET_MUL_INVERSE, move |e, a| {
            sink.borrow_mut().push((SET_MUL_INVERSE, a.to_vec()));
            for row in 0..3 {
                copy_vec(e, a[1] + 0x10 * row, a[0] + 0x10 * row);
            }
            Ret::default()
        });
        e.register_double(SCALE_ROWS, |e, a| {
            let factor = e.mem.f32(a[1]);
            for row in 0..3 {
                let scaled = load_vec(e, a[0] + 0x10 * row).map(|lane| lane * factor);
                store_vec(e, a[0] + 0x10 * row, scaled);
            }
            Ret::default()
        });
        e.register_double(SET_MUL3, |e, a| {
            copy_vec(e, a[1], a[0]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SET_CENTER_OF_MASS, move |e, a| {
            let mut words = a.to_vec();
            words.extend(a[..2].iter().map(|matrix| e.mem.u32(*matrix)));
            sink.borrow_mut().push((SET_CENTER_OF_MASS, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BUILD_NOT_CONTACT_JACOBIAN, move |e, a| {
            let mut words = a.to_vec();
            for buffer in [e.mem.u32(a[2] + 0x30), e.mem.u32(a[2] + 0x34)] {
                words.push(buffer);
                for offset in [0x10, 0x40] {
                    words.extend((0..4).map(|n| e.mem.u32(buffer + offset + 4 * n)));
                }
            }
            sink.borrow_mut().push((BUILD_NOT_CONTACT_JACOBIAN, words));
            Ret::default()
        });
        let info = e.mem.alloc(0x80);
        let body_a = e.mem.alloc(0x80);
        let body_b = e.mem.alloc(0x80);
        store_vec(&mut e, body_a + 0x10, [10.0; 4]);
        store_vec(&mut e, body_b + 0x10, [20.0; 4]);
        e.mem.set_u32(info + 0x30, body_a);
        e.mem.set_u32(info + 0x34, body_b);
        e.mem.set_u32(info + 0x38, 0x1111);
        e.mem.set_u32(info + 0x3c, 0x2222);
        let (atom, node) = modifier_atom(&mut e, 0x10);
        store_vec(&mut e, atom + 0x20, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, atom + 0x30, [4.0, 5.0, 6.0, 0.0]);
        let cursor = cell(&mut e, 0x6000);
        build_jacobian_from_center_of_mass_changer_modifier(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(cursor),
        );
        let order: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                SET_MUL_INVERSE,
                SET_MUL_INVERSE,
                SET_CENTER_OF_MASS,
                BUILD_NOT_CONTACT_JACOBIAN,
                SET_CENTER_OF_MASS
            ]
        );
        // setMulInverse: the skew matrices and the body copies' rotation rows.
        let inverses = calls(&log, SET_MUL_INVERSE);
        let [not_contact] = calls(&log, BUILD_NOT_CONTACT_JACOBIAN).try_into().unwrap();
        assert_eq!(
            [not_contact[0], not_contact[1], not_contact[3]],
            [node, 0x1234, cursor]
        );
        // The words after the four arguments: the copy's body, its vectors at +0x10 and +0x40.
        let (buffer_a, buffer_b) = (not_contact[4], not_contact[13]);
        assert_eq!(inverses[0][2], buffer_a + 0x50);
        assert_eq!(inverses[1][2], buffer_b + 0x50);
        // Linear velocities: 10 and 20 plus the negated first rows; positions: 0 plus the
        // turned vectors.
        let words = |values: [f32; 4]| values.map(f32::to_bits).to_vec();
        assert_eq!(not_contact[5..9], words([9.0; 4])[..]);
        assert_eq!(not_contact[9..13], words([1.0, 2.0, 3.0, 0.0])[..]);
        assert_eq!(not_contact[14..18], words([16.0; 4])[..]);
        assert_eq!(not_contact[18..], words([4.0, 5.0, 6.0, 0.0])[..]);
        // The record of the negated matrices (first floats -1 and -4), then the record of the
        // products with the inverse rotations (1 and 4).
        let records = calls(&log, SET_CENTER_OF_MASS);
        assert_eq!(records[0][2], cursor);
        assert_eq!(records[0][3..], [(-1.0f32).to_bits(), (-4.0f32).to_bits()]);
        assert_eq!(records[1][..3], [inverses[0][0], inverses[1][0], cursor]);
        assert_eq!(records[1][3..], [1.0f32.to_bits(), 4.0f32.to_bits()]);
    }
}

#[cfg(test)]
mod atom_limit_tests {
    use super::*;

    /// An engine with the image constants and the callees of the linear-limit builders doubled.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, 1.1920929e-7f64);
        e.set_global(MAX_FLOAT, f64::from(f32::MAX));
        for (i, axis) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            for (lane, value) in axis.into_iter().enumerate() {
                e.mem
                    .set_f32(AXIS_TABLE + 16 * i as u32 + 4 * lane as u32, value);
            }
        }
        // `004b4ab0` gives the angular term (1, 0, 0, 0) for both bodies.
        e.register(ROTATE_ANGULAR, |e, a| {
            for (lane, value) in [1.0f32, 0.0, 0.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e
    }

    struct Scene {
        atom: u32,
        info: u32,
        a: u32,
        b: u32,
        cursor: u32,
        records: u32,
    }

    fn scene(e: &mut Engine, limit: f32, kind: u8) -> Scene {
        let atom = e.mem.alloc(0x10);
        e.mem.set_u8(atom + 3, kind);
        e.mem.set_f32(atom + 4, limit);
        e.mem.set_f32(ATOM_SCALE_TABLE + 4 * u32::from(kind), 0.25);
        let (body_a, body_b) = (e.mem.alloc(0x100), e.mem.alloc(0x100));
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 0x20, 2.0);
        e.mem.set_f32(info + 0x24, 1.0);
        e.mem.set_u32(info + 0x30, body_a);
        e.mem.set_u32(info + 0x34, body_b);
        // Body A's centre of mass is one above the origin; body B's is at the point b.
        e.mem.set_f32(body_a + 0x44, 1.0);
        e.mem.set_f32(body_b + 0x40, 1.0);
        let (a, b) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        e.mem.set_f32(b + 0x30, 1.0);
        let records = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        Scene {
            atom,
            info,
            a,
            b,
            cursor,
            records,
        }
    }

    #[test]
    fn the_atom_supplies_the_limit_and_the_scale_of_three_records() {
        let mut e = engine();
        // Body A's centre of mass is at (0, 1, 0) (the odd offset above), so `ra` = (0, -1, 0).
        let s = scene(&mut e, f32::MAX, 3);
        e.call(0x00d6_ced0, &args![s.atom, s.info, s.a, s.b, s.cursor]);
        // Three records of 0x30 bytes; the limit is FLT_MAX, so the tag is 5.
        assert_eq!(e.mem.u32(s.cursor), s.records + 0x90);
        for record in 0..3 {
            assert_eq!(e.mem.u8(s.records + 0x30 * record), 5);
        }
        // error = (b - a) - h * (ra x w) with ra x w = (0, 0, 1), h = scale / weight = 0.125:
        // z component -0.125, times the weight 2 = -0.25 = -(table scale).
        assert_eq!(e.mem.f32(s.records + 0x60 + 0xc), -0.25);
        // The x component of the error is b - a = 1, times 2.
        assert_eq!(e.mem.f32(s.records + 0xc), 2.0);
    }

    #[test]
    fn a_finite_limit_is_stored_in_each_record_with_tag_six() {
        let mut e = engine();
        let s = scene(&mut e, 12.5, 0);
        e.call(0x00d6_ced0, &args![s.atom, s.info, s.a, s.b, s.cursor]);
        for record in 0..3 {
            assert_eq!(e.mem.u8(s.records + 0x30 * record), 6);
            assert_eq!(e.mem.f32(s.records + 0x30 * record + 0x2c), 12.5);
        }
    }
}

#[cfg(test)]
mod atom_dispatch_tests {
    use super::*;

    /// A minimal engine: the exe's data page and `FLT_EPSILON` as a double (the opening record
    /// adds it).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, 1.1920929e-7f64);
        e
    }

    /// A body with identity rotation rows and no translation.
    fn identity_body(e: &mut Engine) -> u32 {
        let base = e.mem.alloc(0x40);
        store_vec(e, base, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, base + 0x30, [0.0, 0.0, 0.0, 0.0]);
        base
    }

    #[test]
    fn the_opening_record_carries_the_counters_the_tag_and_the_flag() {
        let mut e = engine();
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x60, 0x1111_1111);
        e.mem.set_u32(info + 0x64, 0x2222_2222);
        e.mem.set_u32(info + 0x6c, 0x3333_3333);
        e.mem.set_u32(info + 0x70, 0x4444_4444);
        let (body_b, body_a) = (identity_body(&mut e), identity_body(&mut e));
        e.mem.set_u32(info + 0x38, body_b);
        e.mem.set_u32(info + 0x3c, body_a);
        // One type-2 atom (0x90 bytes): it transforms the body blocks in the frame and writes
        // nothing to the cursor, so only the opening record and the single step are observable.
        let atom = e.mem.alloc(0x90);
        e.mem.set_u16(atom, 2);
        let records = e.mem.alloc(0x100);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        e.call(0x00d6_f460, &args![atom, 0x90u32, info, cursor]);
        assert_eq!(e.mem.u32(records + 8), 0x2222_2222);
        assert_eq!(e.mem.u16(records + 4), 0x3333);
        assert_eq!(e.mem.u16(records + 6), 0x4444);
        assert_eq!(e.mem.u32(records + 0xc), 0x1111_1111);
        assert_eq!(e.mem.u8(records), 1);
        assert_eq!(e.mem.u8(records + 1), 8);
        // Type 2 appends nothing: the cursor is one record past the start.
        assert_eq!(e.mem.u32(cursor), records + 0x10);
    }

    #[test]
    fn the_modifier_atom_backs_the_cursor_up_and_calls_the_node_function() {
        let mut e = engine();
        let info = e.mem.alloc(0x80);
        let body = identity_body(&mut e);
        e.mem.set_u32(info + 0x38, body);
        e.mem.set_u32(info + 0x3c, body);
        let atom = e.mem.alloc(0xc);
        e.mem.set_u16(atom, 1);
        e.mem.set_u32(atom + 4, 0x00b0_0000);
        e.mem.set_u32(atom + 8, 0xdead_beef);
        let records = e.mem.alloc(0x100);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        e.register(0x00b0_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_f460, &args![atom, 0xcu32, info, cursor]);
        let calls: Vec<&(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00b0_0000)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], 0xdead_beef);
        assert_eq!(calls[0].1[2], cursor);
        // The opening record advanced the cursor by 0x10; the modifier backs it up by 0x10.
        assert_eq!(e.mem.u32(cursor), records);
    }
}

#[cfg(test)]
mod atom_one_d_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Calls seen by a double: the address, the argument words and the 0x40 bytes behind the
    /// first argument (the input block, read while the dispatcher's frame is alive).
    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>, [u32; 0x10])>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, 1.1920929e-7f64);
        e.set_global(SMALL_FLOAT, 1.1920929e-7f32);
        for axis in 0..3u32 {
            let mut vector = [0.0f32; 4];
            vector[axis as usize] = 1.0;
            store_vec(&mut e, AXIS_TABLE + 0x10 * axis, vector);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        for (index, offset) in [0u8, 0x10, 0x20, 0, 0x10, 0x20, 0, 0x10, 0x20]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u8(AXIS_OFFSET_TABLE + index as u32, offset);
        }
        e
    }

    fn capture(e: &mut Engine, seen: &Seen, address: u32) {
        let log = seen.clone();
        e.register_double(address, move |e, a| {
            // `hkCalcMotorData` takes the motor first and its input block second.
            let at = if address == CALC_MOTOR_DATA {
                a[1]
            } else {
                a[0]
            };
            let block = std::array::from_fn(|word| e.mem.u32(at + 4 * word as u32));
            log.borrow_mut().push((address, a.to_vec(), block));
            Ret::default()
        });
    }

    fn calls(seen: &Seen, address: u32) -> Vec<(Vec<u32>, [u32; 0x10])> {
        seen.borrow()
            .iter()
            .filter(|(at, _, _)| *at == address)
            .map(|(_, words, block)| (words.clone(), *block))
            .collect()
    }

    fn lanes(block: &[u32; 0x10], first: usize) -> Vec<f32> {
        (first..first + 4)
            .map(|w| f32::from_bits(block[w]))
            .collect()
    }

    struct Scene {
        info: u32,
        atoms: u32,
        records: u32,
        cursor: u32,
        counter: u32,
    }

    fn body(e: &mut Engine, pivot: Vec4) -> u32 {
        let base = e.mem.alloc(0x40);
        store_vec(e, base, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, base + 0x30, pivot);
        base
    }

    /// Two bodies with identity rotation rows, pivot A at `a` and pivot B at `b` (each body's
    /// fourth row), the counter block at `info + 0x64`, and room for the atoms.
    fn scene(e: &mut Engine, a: Vec4, b: Vec4) -> Scene {
        let (body_a, body_b) = (body(e, a), body(e, b));
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x3c, body_a);
        e.mem.set_u32(info + 0x38, body_b);
        let counter = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x64, counter);
        let atoms = e.mem.alloc(0x100);
        let records = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        Scene {
            info,
            atoms,
            records,
            cursor,
            counter,
        }
    }

    fn run(e: &mut Engine, s: &Scene, size: u32) {
        e.call(0x00d6_f460, &args![s.atoms, size, s.info, s.cursor]);
    }

    #[test]
    fn the_distance_atom_hands_the_pivots_their_direction_and_the_remaining_length() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, SCALED_LINEAR_RECORD);
        let s = scene(&mut e, [0.0; 4], [3.0, 4.0, 0.0, 0.0]);
        e.mem.set_u16(s.atoms, 6);
        e.mem.set_f32(s.atoms + 4, 7.0);
        run(&mut e, &s, 8);
        let done = calls(&seen, SCALED_LINEAR_RECORD);
        assert_eq!(done.len(), 1);
        let (words, block) = &done[0];
        assert_eq!(f32::from_bits(words[3]), 2.0);
        assert_eq!(lanes(block, 0), [3.0, 4.0, 0.0, 0.0]);
        let inverse = (1.0f64 / 5.0) as f32;
        assert_eq!(lanes(block, 8), [inverse * 3.0, inverse * 4.0, 0.0, 0.0]);
    }

    #[test]
    fn coincident_pivots_take_the_first_axis_and_a_zero_distance() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, SCALED_LINEAR_RECORD);
        let s = scene(&mut e, [1.0; 4], [1.0; 4]);
        e.mem.set_u16(s.atoms, 6);
        e.mem.set_f32(s.atoms + 4, 7.0);
        run(&mut e, &s, 8);
        let (words, block) = &calls(&seen, SCALED_LINEAR_RECORD)[0];
        assert_eq!(f32::from_bits(words[3]), 7.0);
        assert_eq!(lanes(block, 8), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn linear_atoms_pass_the_pivots_and_a_row_and_advance_the_counter() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, LINEAR_BILATERAL);
        capture(&mut e, &seen, LINEAR_LIMIT);
        capture(&mut e, &seen, LINEAR_FRICTION);
        let s = scene(&mut e, [1.0, 2.0, 3.0, 0.0], [5.0, 6.0, 7.0, 0.0]);
        // A bilateral atom on row 1, a limit atom (tau 0.5, limit 9) on row 2, then a friction
        // atom whose block carries the counter after both advanced it by 8.
        e.mem.set_u16(s.atoms, 7);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u16(s.atoms + 4, 9);
        e.mem.set_u8(s.atoms + 6, 2);
        e.mem.set_f32(s.atoms + 8, 0.5);
        e.mem.set_f32(s.atoms + 0xc, 9.0);
        e.mem.set_u16(s.atoms + 0x10, 0xa);
        e.mem.set_u8(s.atoms + 0x12, 1);
        e.mem.set_f32(s.atoms + 0x14, 0.25);
        run(&mut e, &s, 0x18);
        let (_, block) = &calls(&seen, LINEAR_BILATERAL)[0];
        assert_eq!(lanes(block, 0), [5.0, 6.0, 7.0, 0.0]);
        assert_eq!(lanes(block, 4), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lanes(block, 8), [0.0, 1.0, 0.0, 0.0]);
        let (_, block) = &calls(&seen, LINEAR_LIMIT)[0];
        assert_eq!(lanes(block, 8), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(f32::from_bits(block[0xc]), 0.5);
        assert_eq!(f32::from_bits(block[0xd]), 9.0);
        let (_, block) = &calls(&seen, LINEAR_FRICTION)[0];
        assert_eq!(f32::from_bits(block[8]), 0.25);
        assert_eq!(block[9], s.counter + 0x10);
    }

    #[test]
    fn a_friction_atom_without_friction_appends_the_placeholder() {
        let mut e = engine();
        let s = scene(&mut e, [0.0; 4], [0.0; 4]);
        e.mem.set_u16(s.atoms, 0xa);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_f32(s.atoms + 4, 0.0);
        run(&mut e, &s, 8);
        let record = s.records + 0x10;
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
        assert_eq!(e.mem.u32(s.cursor), s.records + 0x20);
    }

    #[test]
    fn the_angular_pair_atom_builds_two_records_the_second_with_the_next_row_negated() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, ANGULAR_BILATERAL);
        let s = scene(&mut e, [0.0; 4], [0.0; 4]);
        e.mem.set_u16(s.atoms, 0xc);
        e.mem.set_u8(s.atoms + 2, 0);
        e.mem.set_u8(NEXT_AXIS_TABLE, 0x10);
        e.mem.set_u8(THIRD_AXIS_TABLE, 0x20);
        run(&mut e, &s, 4);
        let done = calls(&seen, ANGULAR_BILATERAL);
        assert_eq!(done.len(), 2);
        assert_eq!(lanes(&done[0].1, 0), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(lanes(&done[0].1, 4), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(lanes(&done[0].1, 8), [0.0, 1.0, 0.0, 0.0]);
        let negated: Vec<u32> = (4..8).map(|w| done[1].1[w]).collect();
        assert_eq!(
            negated,
            [0x8000_0000, 0xbf80_0000, 0x8000_0000, 0x8000_0000]
        );
        assert_eq!(lanes(&done[1].1, 8), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn the_angular_run_atom_builds_one_record_per_count() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, ANGULAR_BILATERAL);
        let s = scene(&mut e, [0.0; 4], [0.0; 4]);
        e.mem.set_u16(s.atoms, 0xd);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u8(s.atoms + 3, 2);
        run(&mut e, &s, 4);
        let done = calls(&seen, ANGULAR_BILATERAL);
        assert_eq!(done.len(), 2);
        // i = 1: c0 = 0x10, c1 = 0x20, c2 = 0.
        assert_eq!(lanes(&done[0].1, 0), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(lanes(&done[0].1, 4), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(lanes(&done[0].1, 8), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn an_angular_friction_atom_without_friction_keeps_its_marker() {
        let mut e = engine();
        let s = scene(&mut e, [0.0; 4], [0.0; 4]);
        e.mem.set_u16(s.atoms, 0x11);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u8(s.atoms + 4, 3);
        e.mem.set_f32(s.atoms + 8, 0.0);
        run(&mut e, &s, 0xc);
        let record = s.records + 0x10;
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 3);
    }

    #[test]
    fn the_linear_motor_atom_seeds_the_value_and_commits_through_the_motor() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, LINEAR_VELOCITY_BEGIN);
        capture(&mut e, &seen, CALC_MOTOR_DATA);
        capture(&mut e, &seen, LINEAR_VELOCITY_COMMIT);
        let s = scene(&mut e, [0.0; 4], [2.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(s.counter, 10.0);
        e.mem.set_f32(s.counter + 4, 20.0);
        e.mem.set_u16(s.atoms, 0xb);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u8(s.atoms + 3, 0);
        e.mem.set_u16(s.atoms + 4, 0x20);
        e.mem.set_u16(s.atoms + 6, 0x24);
        e.mem.set_f32(s.atoms + 8, 1.5);
        e.mem.set_u32(s.atoms + 0xc, 0x0123_4560);
        run(&mut e, &s, 0x10);
        let (words, block) = &calls(&seen, CALC_MOTOR_DATA)[0];
        assert_eq!(words[0], 0x0123_4560);
        assert_eq!(f32::from_bits(block[2]), 10.0);
        assert_eq!(f32::from_bits(block[3]), 20.0);
        // target - value = 0; value - (B - A) . row 0 = 1.5 - 2.
        assert_eq!(f32::from_bits(block[4]), 0.0);
        assert_eq!(f32::from_bits(block[5]), -0.5);
        assert_eq!(calls(&seen, LINEAR_VELOCITY_COMMIT).len(), 1);
        assert_eq!(e.mem.f32(s.counter + 0x24), 1.5);
        assert_eq!(e.mem.u8(s.counter + 0x20), 1);
    }
}

#[cfg(test)]
mod atom_angle_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>, [u32; 8])>>>;

    /// The exe's pi and 2 pi (`0101ff40`, `0101ff48`: the float values widened).
    const PI_VALUE: f64 = std::f32::consts::PI as f64;
    const TWO_PI_VALUE: f64 = (2.0 * std::f32::consts::PI) as f64;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, 1.1920929e-7f64);
        e.set_global(SMALL_FLOAT, 1.1920929e-7f32);
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(PI, PI_VALUE);
        e.set_global(TWO_PI, TWO_PI_VALUE);
        e.set_global(TINY, 1e-16f64);
        e.set_global(LIMIT_SCALE, 0.05f64);
        e.set_global(ANGLE_SCALE_TABLE, 1.0f32);
        e.set_global(ANGLE_OFFSET_TABLE, 0.0f32);
        e.set_global(AXIS_SIGN_TABLE, 1.0f32);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        for (index, offset) in [0u8, 0x10, 0x20, 0, 0x10, 0x20, 0, 0x10, 0x20]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u8(AXIS_OFFSET_TABLE + index as u32, offset);
        }
        e.register(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        e
    }

    /// `hkMath_atan2fApproximation` answers `angle`; the calls are recorded.
    fn atan2_double(e: &mut Engine, seen: &Seen, angle: f64) {
        let log = seen.clone();
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            log.borrow_mut()
                .push((ATAN2_APPROXIMATION, a.to_vec(), [0; 8]));
            Ret {
                st0: angle,
                ..Ret::default()
            }
        });
    }

    /// `hkCalcMotorData` records its input block and answers `out[k] = k + 1`.
    fn motor_double(e: &mut Engine, seen: &Seen) {
        let log = seen.clone();
        e.register_double(CALC_MOTOR_DATA, move |e, a| {
            let block = std::array::from_fn(|word| e.mem.u32(a[1] + 4 * word as u32));
            log.borrow_mut().push((CALC_MOTOR_DATA, a.to_vec(), block));
            for word in 0..6u32 {
                e.mem.set_f32(a[2] + 4 * word, (word + 1) as f32);
            }
            Ret::default()
        });
    }

    fn calls(seen: &Seen, address: u32) -> Vec<(Vec<u32>, [u32; 8])> {
        seen.borrow()
            .iter()
            .filter(|(at, _, _)| *at == address)
            .map(|(_, words, block)| (words.clone(), *block))
            .collect()
    }

    struct Scene {
        info: u32,
        atoms: u32,
        records: u32,
        cursor: u32,
        counter: u32,
    }

    fn rows(e: &mut Engine, base: u32) {
        store_vec(e, base, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + 0x20, [0.0, 0.0, 1.0, 0.0]);
    }

    /// Identity rotation blocks for both bodies (pivots at the origin), two solver bodies with
    /// identity rows at `+0x50` and unit inverse inertia, `schema.float[1] = 0.5`,
    /// `schema.float[2] = 2`, `schema.float[3] = 4` and `schema.float[0x10] = 2`.
    fn scene(e: &mut Engine) -> Scene {
        let info = e.mem.alloc(0x80);
        for slot in [0x38u32, 0x3c] {
            let block = e.mem.alloc(0x40);
            rows(e, block);
            e.mem.set_u32(info + slot, block);
        }
        for slot in [0x30u32, 0x34] {
            let body = e.mem.alloc(0x80);
            store_vec(e, body + 0x30, [1.0, 1.0, 1.0, 0.0]);
            rows(e, body + 0x50);
            e.mem.set_u32(info + slot, body);
        }
        e.mem.set_f32(info + 4, 0.5);
        e.mem.set_f32(info + 8, 2.0);
        e.mem.set_f32(info + 0xc, 4.0);
        e.mem.set_f32(info + 0x40, 2.0);
        let counter = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x64, counter);
        let atoms = e.mem.alloc(0x100);
        let records = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        Scene {
            info,
            atoms,
            records,
            cursor,
            counter,
        }
    }

    fn run(e: &mut Engine, s: &Scene, size: u32) {
        e.call(0x00d6_f460, &args![s.atoms, size, s.info, s.cursor]);
    }

    #[test]
    fn the_angle_limit_atom_wraps_the_angle_and_writes_the_angle_record() {
        let mut e = engine();
        let seen = Seen::default();
        atan2_double(&mut e, &seen, 4.0);
        let s = scene(&mut e);
        // Limit -counter.float[1] = -0.5: 4 - (-0.5) is above pi, so one turn comes off.
        e.mem.set_f32(s.counter + 4, 0.5);
        e.mem.set_u16(s.atoms, 0xe);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u8(s.atoms + 3, 0);
        e.mem.set_f32(s.atoms + 4, 0.25);
        e.mem.set_f32(s.atoms + 8, 0.75);
        e.mem.set_f32(s.atoms + 0xc, 3.0);
        run(&mut e, &s, 0x10);
        let angle = (4.0f64 - TWO_PI_VALUE) as f32;
        let record = s.records + 0x10;
        assert_eq!(e.mem.u8(record), 0xd);
        assert_eq!(e.mem.f32(record + 0x1c), (f64::from(angle) * -2.0) as f32);
        // k = 3 * 2 = 6; the angular sum is 1 + 1 = 2.
        assert_eq!(e.mem.f32(record + 0x28), 6.0);
        let mass = (2.0f64 + 1.1920929e-7) as f32;
        assert_eq!(e.mem.f32(record + 0xc), (6.0 / f64::from(mass)) as f32);
        assert_eq!(e.mem.f32(record + 0x20), (0.25f64 * -2.0 * 6.0) as f32);
        assert_eq!(e.mem.f32(record + 0x24), (-2.0f64 * 0.75 * 6.0) as f32);
        assert_eq!(e.mem.u32(s.cursor), record + 0x30);
    }

    #[test]
    fn a_disabled_angle_limit_atom_appends_the_placeholder() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u16(s.atoms, 0xe);
        run(&mut e, &s, 0x10);
        let record = s.records + 0x10;
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
        assert_eq!(e.mem.u32(s.cursor), record + 0x10);
    }

    #[test]
    fn the_cone_atom_weights_by_the_square_of_half_the_summed_axis() {
        let mut e = engine();
        let seen = Seen::default();
        atan2_double(&mut e, &seen, 0.5);
        let s = scene(&mut e);
        e.mem.set_u16(s.atoms, 0xf);
        e.mem.set_u8(s.atoms + 3, 0);
        e.mem.set_u8(s.atoms + 4, 1);
        e.mem.set_f32(s.atoms + 0x10, 3.0);
        run(&mut e, &s, 0x14);
        // The summed axis (2, 0, 0) has length 2, half 1: the weight is 3 * 2 * 1 * 1.
        let record = s.records + 0x10;
        assert_eq!(e.mem.f32(record + 0x28), 6.0);
        assert_eq!(e.mem.f32(record + 0x1c), -1.0);
        // t = n x b with n = x and b = y is z, so w = y gives atan2(0, y . (z x x)) = atan2(0, 1).
        let (words, _) = &calls(&seen, ATAN2_APPROXIMATION)[0];
        assert_eq!(f32::from_bits(words[0]), 0.0);
        assert_eq!(f32::from_bits(words[1]).abs(), 1.0);
    }

    #[test]
    fn parallel_twist_axes_append_the_placeholder() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u16(s.atoms, 0x10);
        e.mem.set_u8(s.atoms + 3, 0);
        e.mem.set_u8(s.atoms + 4, 0);
        run(&mut e, &s, 0x14);
        let record = s.records + 0x10;
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u32(s.cursor), record + 0x10);
    }

    #[test]
    fn the_twist_atom_with_a_limit_slot_moves_the_slot_and_the_angle() {
        let mut e = engine();
        let seen = Seen::default();
        atan2_double(&mut e, &seen, 1.0);
        let s = scene(&mut e);
        // a = counter.float[2] = 0.25, c = -counter.float[1] = -0.5.
        e.mem.set_f32(s.counter + 4, 0.5);
        e.mem.set_f32(s.counter + 8, 0.25);
        e.mem.set_u16(s.atoms, 0x10);
        e.mem.set_u8(s.atoms + 3, 0);
        e.mem.set_u8(s.atoms + 4, 1);
        e.mem.set_u8(s.atoms + 6, 8);
        e.mem.set_f32(s.atoms + 0x10, 1.0);
        run(&mut e, &s, 0x14);
        // value 1: x = 0.75, y = -1.25, M = max(4 * 0.5, 0.25 * 0.05) = 2;
        // 0.5 * (2 + 4) = 3 is not below 1.25, so the slot is 0.25 - 2 (at least the small
        // float) and the angle 0.75 + 2.
        assert_eq!(e.mem.f32(s.counter + 8), 1.1920929e-7);
        let record = s.records + 0x10;
        assert_eq!(e.mem.f32(record + 0x1c), -5.5);
    }

    #[test]
    fn the_angular_motor_atom_commits_two_records_and_updates_the_value() {
        let mut e = engine();
        let seen = Seen::default();
        atan2_double(&mut e, &seen, 0.5);
        motor_double(&mut e, &seen);
        let s = scene(&mut e);
        e.mem.set_f32(s.counter, 10.0);
        e.mem.set_f32(s.counter + 4, 20.0);
        e.mem.set_u16(s.atoms, 0x12);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u16(s.atoms + 4, 0x20);
        e.mem.set_u16(s.atoms + 6, 0x24);
        e.mem.set_f32(s.atoms + 0xc, 1.5);
        e.mem.set_u32(s.atoms + 0x10, 0x0123_4560);
        run(&mut e, &s, 0x14);
        let (words, block) = &calls(&seen, CALC_MOTOR_DATA)[0];
        assert_eq!(words[0], 0x0123_4560);
        assert_eq!(f32::from_bits(block[2]), 10.0);
        assert_eq!(f32::from_bits(block[3]), 20.0);
        // The value starts as the angle 0.5: target - value = 1, value - angle = 0.
        assert_eq!(f32::from_bits(block[4]), 1.0);
        assert_eq!(f32::from_bits(block[5]), 0.0);
        let first = s.records + 0x10;
        assert_eq!(e.mem.f32(first + 0x1c), 2.0);
        let second = first;
        assert_eq!(e.mem.u8(second), 0xf);
        assert_eq!(e.mem.f32(second + 0x20), 1.5);
        assert_eq!(e.mem.f32(second + 0x24), 2.0);
        assert_eq!(e.mem.f32(second + 0x28), 2.0);
        assert_eq!(e.mem.u32(s.cursor), second + 0x40);
        assert_eq!(e.mem.f32(s.counter + 0x24), 1.5);
        assert_eq!(e.mem.u8(s.counter + 0x20), 1);
    }

    #[test]
    fn the_ragdoll_motor_atom_builds_a_record_pair_per_motor() {
        let mut e = engine();
        let seen = Seen::default();
        atan2_double(&mut e, &seen, 0.5);
        motor_double(&mut e, &seen);
        e.register(SET_MUL_MAT3_MAT3, |e, a| {
            rows(e, a[0]);
            Ret::default()
        });
        let s = scene(&mut e);
        e.mem.set_u16(s.atoms, 0x13);
        e.mem.set_u8(s.atoms + 2, 1);
        e.mem.set_u16(s.atoms + 4, 0x40);
        e.mem.set_u16(s.atoms + 6, 0x50);
        // Motors for axes 0 and 2; axis 1 has none.
        e.mem.set_u32(s.atoms + 0x40, 0x0123_4560);
        e.mem.set_u32(s.atoms + 0x48, 0x0123_4570);
        run(&mut e, &s, 0x50);
        let motors = calls(&seen, CALC_MOTOR_DATA);
        assert_eq!(motors.len(), 2);
        assert_eq!(motors[1].0[0], 0x0123_4570);
        // Records: one of 0x40 bytes, the placeholder, one of 0x40 bytes.
        let placeholder = s.records + 0x10 + 0x40;
        assert_eq!(e.mem.u8(placeholder), 3);
        assert_eq!(e.mem.u32(s.cursor), placeholder + 0x10 + 0x40);
        // The first and third flags are set, the second untouched.
        assert_eq!(e.mem.u8(s.counter + 0x40), 1);
        assert_eq!(e.mem.u8(s.counter + 0x41), 0);
        assert_eq!(e.mem.u8(s.counter + 0x42), 1);
    }
}

#[cfg(test)]
mod atom_container_tests {
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Calls seen by the doubles: the address, the argument words and the first 0x20 words
    /// behind the first two arguments (read while the dispatcher's frame is alive).
    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, 1.1920929e-7f64);
        e.set_global(SMALL_FLOAT, 1.1920929e-7f32);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        for axis in 0..3u32 {
            let mut vector = [0.0f32; 4];
            vector[axis as usize] = 1.0;
            store_vec(&mut e, AXIS_TABLE + 0x10 * axis, vector);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            let xyz = if lane < 3 { 0x8000_0000 } else { 0 };
            e.mem.set_u32(SIGN_MASK_XYZ + 4 * lane, xyz);
        }
        e
    }

    fn capture(e: &mut Engine, seen: &Seen, address: u32) {
        let log = seen.clone();
        e.register_double(address, move |e, a| {
            let mut words = Vec::new();
            for argument in a.iter().take(2) {
                for word in 0..0x20u32 {
                    let at = argument.wrapping_add(4 * word);
                    words.push(if e.mem.is_mapped(at) {
                        e.mem.u32(at)
                    } else {
                        0
                    });
                }
            }
            log.borrow_mut().push((address, a.to_vec(), words));
            Ret::default()
        });
    }

    fn calls(seen: &Seen, address: u32) -> Vec<(Vec<u32>, Vec<u32>)> {
        seen.borrow()
            .iter()
            .filter(|(at, _, _)| *at == address)
            .map(|(_, words, block)| (words.clone(), block.clone()))
            .collect()
    }

    fn order(seen: &Seen) -> Vec<u32> {
        seen.borrow().iter().map(|(at, _, _)| *at).collect()
    }

    struct Scene {
        info: u32,
        atoms: u32,
        records: u32,
        cursor: u32,
        body_a: u32,
        body_b: u32,
    }

    fn rows(e: &mut Engine, base: u32) {
        store_vec(e, base, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + 0x20, [0.0, 0.0, 1.0, 0.0]);
    }

    /// Identity rotation blocks (pivots at the origin), solver bodies with identity rows at
    /// `+0x50`, unit inverse inertia and mass 1 (`+0x3c`), `schema.float[2] = 2` and
    /// `schema.float[9] = 3`.
    fn scene(e: &mut Engine) -> Scene {
        let info = e.mem.alloc(0x80);
        for slot in [0x38u32, 0x3c] {
            let block = e.mem.alloc(0x40);
            rows(e, block);
            e.mem.set_u32(info + slot, block);
        }
        let mut bodies = [0u32; 2];
        for (index, slot) in [0x30u32, 0x34].into_iter().enumerate() {
            let body = e.mem.alloc(0x80);
            store_vec(e, body + 0x30, [1.0, 1.0, 1.0, 1.0]);
            rows(e, body + 0x50);
            e.mem.set_u32(info + slot, body);
            bodies[index] = body;
        }
        e.mem.set_f32(info + 8, 2.0);
        e.mem.set_f32(info + 0x24, 3.0);
        let counter = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x64, counter);
        let atoms = e.mem.alloc(0x400);
        let records = e.mem.alloc(0x400);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        Scene {
            info,
            atoms,
            records,
            cursor,
            body_a: bodies[0],
            body_b: bodies[1],
        }
    }

    fn run(e: &mut Engine, s: &Scene, size: u32) {
        e.call(0x00d6_f460, &args![s.atoms, size, s.info, s.cursor]);
    }

    #[test]
    fn the_length_atom_writes_a_tag_eleven_record() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u16(s.atoms, 0x14);
        store_vec(&mut e, s.atoms + 0x10, [3.0, 4.0, 0.0, 0.0]);
        e.mem.set_f32(s.atoms + 0x30, 1.0);
        e.mem.set_f32(s.atoms + 0x34, 2.0);
        run(&mut e, &s, 0x40);
        let record = s.records + 0x10;
        assert_eq!(e.mem.u8(record), 0xb);
        let inverse = inverse_length(&e, 25.0);
        assert_eq!(e.mem.f32(record + 4), inverse * 4.0);
        // The second direction is too short: the negated first axis, times the scale 2.
        assert_eq!(e.mem.u32(record + 0x10), 0xc000_0000);
        assert_eq!(e.mem.u32(record + 0x14), 0x8000_0000);
        // ((2 * 0 + 5) - 1) * schema.float[2].
        assert_eq!(e.mem.f32(record + 0xc), 8.0);
        assert_eq!(e.mem.u32(s.cursor), record + 0x40);
    }

    /// A target node of type 0x16 with `count` entries (entry `j` is `j`), all targets at 100.
    fn contact_node(e: &mut Engine, count: u16) -> u32 {
        let node = e.mem.alloc(0x200);
        e.mem.set_u16(node, 0x16);
        e.mem.set_u16(node + 4, count);
        e.mem.set_u8(node + 0xa, 0x20);
        for entry in 0..u32::from(count) {
            e.mem.set_f32(node + 0x4c + 0x20 * entry, entry as f32);
            e.mem.set_f32(node + 0x40 + 0x20 * entry, 100.0);
        }
        node
    }

    #[test]
    fn the_lowering_modifier_lowers_every_entry_and_builds_the_contact_jacobian() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, BUILD_CONTACT_JACOBIAN);
        let s = scene(&mut e);
        e.mem.set_f32(s.info, 1.0);
        e.mem.set_f32(s.info + 4, 1.0);
        let node = contact_node(&mut e, 5);
        e.mem.set_u16(s.atoms, 0x17);
        e.mem.set_u32(s.atoms + 0x14, node);
        e.mem.set_f32(s.atoms + 0x20, 2.0);
        e.mem.set_f32(s.atoms + 0x24, 4.0);
        // Entry 1's target is already low.
        e.mem.set_f32(node + 0x60, 0.5);
        run(&mut e, &s, 0x30);
        // u = 4 * 1 * 1 / 2 + j = 2 + j.
        assert_eq!(e.mem.f32(node + 0x40), 2.0);
        assert_eq!(e.mem.f32(node + 0x60), 0.5);
        assert_eq!(e.mem.f32(node + 0xc0), 6.0);
        let (words, block) = &calls(&seen, BUILD_CONTACT_JACOBIAN)[0];
        assert_eq!(words[0], node);
        assert_eq!(words[2] & 0xff, 1);
        // The copy's float[9] is 3 * 2.
        assert_eq!(f32::from_bits(block[0x20 + 9]), 6.0);
        // The cursor backed up over the opening record.
        assert_eq!(words[3], s.cursor);
        assert_eq!(e.mem.u32(s.cursor), s.records);
    }

    #[test]
    fn the_scaling_modifier_brackets_the_child_with_two_vector_pairs() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, PUSH_VECTOR_PAIR);
        capture(&mut e, &seen, BUILD_CONTACT_JACOBIAN);
        let s = scene(&mut e);
        let node = contact_node(&mut e, 0);
        e.mem.set_u16(s.atoms, 0x18);
        e.mem.set_u32(s.atoms + 0x14, node);
        store_vec(&mut e, s.atoms + 0x20, [2.0, 3.0, 4.0, 5.0]);
        store_vec(&mut e, s.atoms + 0x30, [6.0, 7.0, 8.0, 9.0]);
        run(&mut e, &s, 0x40);
        assert_eq!(
            order(&seen),
            [PUSH_VECTOR_PAIR, BUILD_CONTACT_JACOBIAN, PUSH_VECTOR_PAIR]
        );
        let pairs = calls(&seen, PUSH_VECTOR_PAIR);
        let scaled: Vec<f32> = pairs[0].1[..4].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(scaled, [2.0, 3.0, 4.0, 5.0]);
        let scaled: Vec<f32> = pairs[0].1[0x20..0x24]
            .iter()
            .map(|w| f32::from_bits(*w))
            .collect();
        assert_eq!(scaled, [6.0, 7.0, 8.0, 9.0]);
        assert_eq!(pairs[1].0[..2], [s.body_a + 0x30, s.body_b + 0x30]);
    }

    #[test]
    fn the_viscosity_modifier_recurses_into_a_non_contact_child() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, ADD_SURFACE_VISCOSITY);
        let s = scene(&mut e);
        // The child stream is one type-2 atom (0x90 bytes, the id word is its length).
        let child = e.mem.alloc(0x90);
        e.mem.set_u16(child, 2);
        e.mem.set_u16(s.atoms, 0x19);
        e.mem.set_u16(s.atoms + 0x12, 0x90);
        e.mem.set_u32(s.atoms + 0x14, child);
        run(&mut e, &s, 0x20);
        let (words, _) = &calls(&seen, ADD_SURFACE_VISCOSITY)[0];
        assert_eq!(words[0], child + 0x10);
        // Backed up over the opening record, the child wrote its own opening record there.
        assert_eq!(e.mem.u32(s.cursor), s.records + 0x10);
        assert_eq!(e.mem.u8(s.records), 1);
    }

    #[test]
    fn the_moving_surface_modifier_pushes_the_tangential_velocity_both_ways() {
        let mut e = engine();
        let seen = Seen::default();
        capture(&mut e, &seen, ADD_VELOCITY);
        capture(&mut e, &seen, BUILD_CONTACT_JACOBIAN);
        e.register(COPY_BODY_DATA, |e, a| {
            for word in 0..0x20 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        let s = scene(&mut e);
        let node = contact_node(&mut e, 0);
        store_vec(&mut e, node + 0x40, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_u16(s.atoms, 0x1a);
        e.mem.set_u32(s.atoms + 0x14, node);
        store_vec(&mut e, s.atoms + 0x20, [1.0, 2.0, 3.0, 0.0]);
        run(&mut e, &s, 0x30);
        assert_eq!(
            order(&seen),
            [ADD_VELOCITY, BUILD_CONTACT_JACOBIAN, ADD_VELOCITY]
        );
        let pushes = calls(&seen, ADD_VELOCITY);
        // schema.word[0x1b] is 0: body B is copied and the velocity flipped in x, y, z.
        assert_eq!(pushes[0].0[1], 1);
        let first: Vec<f32> = pushes[0].1[..3]
            .iter()
            .map(|w| f32::from_bits(*w))
            .collect();
        assert_eq!(first, [1.0, 2.0, 0.0]);
        let second: Vec<f32> = pushes[1].1[..3]
            .iter()
            .map(|w| f32::from_bits(*w))
            .collect();
        assert_eq!(second, [-1.0, -2.0, -0.0]);
        let (words, _) = &calls(&seen, BUILD_CONTACT_JACOBIAN)[0];
        assert_eq!(words[2] & 0xff, 0);
        // The header record replaced the opening one: value word `node + 0x30`, tag 1.
        assert_eq!(e.mem.u32(s.records + 8), node + 0x30);
    }

    #[test]
    fn the_centre_of_mass_changer_calls_its_helpers_in_the_exe_order() {
        let mut e = engine();
        let seen = Seen::default();
        for address in [
            SET_ROTATED_DIR,
            SET_CROSS_SKEW_SYMMETRIC,
            SET_MUL_INVERSE,
            SCALE_ROWS,
            SET_MUL3,
            SET_CENTER_OF_MASS,
            BUILD_CONTACT_JACOBIAN,
        ] {
            capture(&mut e, &seen, address);
        }
        let s = scene(&mut e);
        let node = contact_node(&mut e, 0);
        e.mem.set_u16(s.atoms, 0x1c);
        e.mem.set_u32(s.atoms + 0x14, node);
        run(&mut e, &s, 0x40);
        assert_eq!(
            order(&seen),
            [
                SET_ROTATED_DIR,
                SET_ROTATED_DIR,
                SET_CROSS_SKEW_SYMMETRIC,
                SET_CROSS_SKEW_SYMMETRIC,
                SET_MUL_INVERSE,
                SET_MUL_INVERSE,
                SCALE_ROWS,
                SCALE_ROWS,
                SET_MUL3,
                SET_MUL3,
                SET_CENTER_OF_MASS,
                BUILD_CONTACT_JACOBIAN,
                SET_CENTER_OF_MASS,
            ]
        );
        let scales = calls(&seen, SCALE_ROWS);
        assert_eq!(scales[0].1[0x20], (-1.0f32).to_bits());
    }
}
