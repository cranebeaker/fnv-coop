//! `Havok SDK/hkp1dbilateralconstraintbuildjacobian.cpp` (Xbox PDB source unit), subsystem
//! `Havok SDK`: its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions append "jacobian" records to a buffer. Each takes the constraint atoms, the
//! constraint runtime data and the address of a cursor (a pointer to the next free record),
//! writes a record and moves the cursor on. In every record the first word is a header: the
//! first float of the record plus a small constant, with its lowest byte replaced by the record
//! kind (a tag the solver reads). The SSE code is translated lane by lane in `f32`, in the exe's
//! operand order; the x87 code in `f64` rounded to `f32` at each store.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` added to the first float of a record for its header.
const HEADER_ADDEND: u32 = 0x010c_9300;
/// Four floats of 1.0.
const ONES: u32 = 0x010e_c7f0;
/// Three 16-byte vectors (filled in when the game starts).
const AXES: u32 = 0x0126_8370;
/// Builds three records for a ball-socket atom (`00d6c970`, cdecl, four words).
const BUILD_BALL_SOCKET: u32 = 0x00d6_c970;
/// The CRT square root (`00ec6040`, argument in ST0, result in ST0).
const X87_SQRT: u32 = 0x00ec_6040;

type Lanes = [f32; 4];

fn load(e: &Engine, address: u32) -> Lanes {
    [0, 1, 2, 3].map(|lane| e.mem.f32(address + 4 * lane))
}

fn store(e: &mut Engine, address: u32, lanes: Lanes) {
    for (lane, value) in lanes.iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, *value);
    }
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| a[lane] * b[lane])
}

fn add(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| a[lane] + b[lane])
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| a[lane] - b[lane])
}

/// `SHUFPS a, b, control`: two lanes of `a`, then two lanes of `b`.
fn shuffle(a: Lanes, b: Lanes, control: u8) -> Lanes {
    let pick = |source: Lanes, shift: u8| source[usize::from((control >> shift) & 3)];
    [pick(a, 0), pick(a, 2), pick(b, 4), pick(b, 6)]
}

/// One lane of `a` in all four lanes.
fn splat(a: Lanes, lane: usize) -> Lanes {
    [a[lane]; 4]
}

/// The cross product `a x b`, as the exe forms it: `a.yzx * b.zxy - a.zxy * b.yzx`.
fn cross(a: Lanes, b: Lanes) -> Lanes {
    sub(
        mul(shuffle(a, a, 0xc9), shuffle(b, b, 0xd2)),
        mul(shuffle(a, a, 0xd2), shuffle(b, b, 0xc9)),
    )
}

/// A body's three rotation rows (`+0x50`, `+0x60`, `+0x70`) applied to `vector`:
/// `y * row1 + x * row0 + z * row2`.
fn rotate(e: &Engine, body: u32, vector: Lanes) -> Lanes {
    let rows = [0x50, 0x60, 0x70].map(|offset| load(e, body + offset));
    add(
        add(
            mul(splat(vector, 1), rows[1]),
            mul(splat(vector, 0), rows[0]),
        ),
        mul(splat(vector, 2), rows[2]),
    )
}

/// The sum the exe folds out of `v1 * v1 * mass_a + v2 * v2 * mass_b` (the first three lanes)
/// and `mass_a + mass_b` (the fourth lane): `(x2 + x0) + (x3 + x1)` of `[sum0, sum1, sum2,
/// mass3]`.
fn mass_term(e: &Engine, body_a: u32, body_b: u32, first: Lanes, second: Lanes) -> f32 {
    let mass_a = load(e, body_a + 0x30);
    let mass_b = load(e, body_b + 0x30);
    let weighted = add(
        mul(mul(first, first), mass_a),
        mul(mul(second, second), mass_b),
    );
    let masses = add(mass_a, mass_b);
    let high = [weighted[2], masses[2], weighted[3], masses[3]];
    let gathered = shuffle(weighted, high, 0xc4);
    let folded = add(shuffle(gathered, gathered, 0x4e), gathered);
    add(shuffle(folded, folded, 0xb1), folded)[0]
}

/// `z + (y + x)` of the first three lanes (`ADDSS` after two shuffles).
fn lane_sum(lanes: Lanes) -> f32 {
    lanes[2] + (lanes[1] + lanes[0])
}

/// A float loaded through the x87 stack.
fn x87_float(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address))
}

/// A float rounded from an `f64` result of x87 arithmetic.
fn rounded(value: f64) -> f32 {
    value as f32
}

/// The record header: the float at `record` plus the `double` constant, rounded to a float,
/// with its lowest byte replaced by `tag`.
fn finish_header(e: &mut Engine, record: u32, tag: u8) {
    let addend = e.mem.f64(HEADER_ADDEND);
    let first = f64::from(x87_float(e, record));
    e.mem.set_f32(record, rounded(addend + first));
    e.mem.set_u8(record, tag);
}

/// The part of a record that is the same in most builders: the axis at `+0`, the two rotated
/// arms at `+0x10` and `+0x20` (arm `n` is the cross product of the offset from the body's
/// position at `+0x40` to the anchor with the axis, rotated by the body's rows) and, at
/// `+0x2c`, the mass term plus the `double` constant, rounded to a float. Answers that last
/// float.
fn write_arms(
    e: &mut Engine,
    record: u32,
    axis: Lanes,
    anchors: [Lanes; 2],
    bodies: [u32; 2],
) -> f32 {
    store(e, record, axis);
    let first_offset = sub(anchors[0], load(e, bodies[0] + 0x40));
    let second_offset = sub(anchors[1], load(e, bodies[1] + 0x40));
    let first_arm = rotate(e, bodies[0], cross(first_offset, axis));
    let second_arm = rotate(e, bodies[1], cross(axis, second_offset));
    store(e, record + 0x10, first_arm);
    store(e, record + 0x20, second_arm);
    let sum = mass_term(e, bodies[0], bodies[1], first_arm, second_arm);
    let addend = e.mem.f64(HEADER_ADDEND);
    let total = rounded(f64::from(sum) + addend);
    e.mem.set_f32(record + 0x2c, total);
    total
}

/// Stores `numerator / divisor` (x87 division, rounded) at `record + 0x1c`.
fn store_quotient(e: &mut Engine, record: u32, numerator: f32, divisor: f32) {
    e.mem.set_f32(
        record + 0x1c,
        rounded(f64::from(numerator) / f64::from(divisor)),
    );
}

/// The two body pointers kept at `+0x30` and `+0x34` of the runtime data.
fn bodies_of(e: &Engine, data: u32) -> [u32; 2] {
    [e.mem.u32(data + 0x30), e.mem.u32(data + 0x34)]
}

// Translated from 00dcedd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAddVelocityBuildJacobian` (Xbox PDB; cdecl, three words): copies the 16 bytes at `vector`
/// to the cursor's record, stores the word `value` at `+0x10`, gives the record its header
/// with tag 0x1a and moves the cursor on by 0x20.
pub fn hk_add_velocity_build_jacobian(e: &mut Engine, vector: u32, value: u32, cursor: u32) {
    let record = e.mem.u32(cursor);
    let lanes = load(e, vector);
    store(e, record, lanes);
    e.mem.set_u32(record + 0x10, value);
    finish_header(e, record, 0x1a);
    e.mem.set_u32(cursor, record + 0x20);
}

// Translated from 00dcee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSetCenterOfMassBuildJacobian` (Xbox PDB; cdecl, three words): copies the 0x30 bytes at
/// `first` and then the 0x30 bytes at `second` to the cursor's record, gives it the header with
/// tag 0x1b and moves the cursor on by 0x60.
pub fn hk_set_center_of_mass_build_jacobian(e: &mut Engine, first: u32, second: u32, cursor: u32) {
    let record = e.mem.u32(cursor);
    for row in 0..3 {
        let lanes = load(e, first + 0x10 * row);
        store(e, record + 0x10 * row, lanes);
    }
    for row in 0..3 {
        let lanes = load(e, second + 0x10 * row);
        store(e, record + 0x30 + 0x10 * row, lanes);
    }
    finish_header(e, record, 0x1b);
    e.mem.set_u32(cursor, record + 0x60);
}

// Translated from 00dcee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLoadVelocityAccumulators` (Xbox PDB): does nothing (a bare `RET`). Its callers push one
/// word, which is not used.
pub fn hk_load_velocity_accumulators(_e: &mut Engine, _unused_1: u32) {}

// Translated from 00dcf100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dLinearFrictionBuildJacobian` (Xbox PDB; cdecl, three words): `atoms` (three vectors at
/// `+0`, `+0x10`, `+0x20`, the float at `+0x20` and a pointer at `+0x24`), `data` (the floats
/// at `+4`, `+0x20` and `+0x24`, the bodies at `+0x30` and `+0x34`) and the cursor. Writes a
/// 0x40-byte record of tag 9: the arms ([`write_arms`], axis `atoms + 0x10`, both anchors
/// `atoms + 0`), the quotient of the float at `data + 0x24` by the mass term at `+0x1c`, the
/// product of the float at `+4` of the object the pointer at `atoms + 0x24` leads to and the
/// float at `data + 0x20` at `+0xC`, and the product of the float at `atoms + 0x20` and the
/// float at `data + 4` at `+0x30`.
pub fn hk1d_linear_friction_build_jacobian(e: &mut Engine, atoms: u32, data: u32, cursor: u32) {
    let record = e.mem.u32(cursor);
    let bodies = bodies_of(e, data);
    let divisor_source = x87_float(e, data + 0x24);
    let axis = load(e, atoms + 0x10);
    let anchor = load(e, atoms);
    let total = write_arms(e, record, axis, [anchor, anchor], bodies);
    store_quotient(e, record, divisor_source, total);
    let inner = e.mem.u32(atoms + 0x24);
    let product = f64::from(x87_float(e, inner + 4)) * f64::from(x87_float(e, data + 0x20));
    e.mem.set_f32(record + 0xc, rounded(product));
    let record = e.mem.u32(cursor);
    e.mem.set_u32(cursor, record + 0x40);
    let product = f64::from(x87_float(e, atoms + 0x20)) * f64::from(x87_float(e, data + 4));
    e.mem.set_f32(record + 0x30, rounded(product));
    finish_header(e, record, 9);
}

// Translated from 00dcf280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dLinearBilateralConstraintBuildJacobian` (Xbox PDB; cdecl, three words): `atoms`, `data`
/// and the cursor as for [`hk1d_linear_friction_build_jacobian`]. Writes a 0x30-byte record of
/// tag 5: the arms ([`write_arms`], axis `atoms + 0x20`, both anchors `atoms + 0`), the quotient
/// of the float at `data + 0x24` by the mass term at `+0x1c`, and at `+0xC` the float at
/// `data + 0x20` times the dot product of `atoms + 0x20` with `(atoms + 0) - (atoms + 0)` (zero
/// unless a lane is not finite; the exe's own subtraction of a vector from itself), which the
/// exe then overwrites with the float at `data + 0x20` times the dot product of the axis with
/// `(atoms + 0x10) - (atoms + 0)`.
pub fn hk1d_linear_bilateral_constraint_build_jacobian(
    e: &mut Engine,
    atoms: u32,
    data: u32,
    cursor: u32,
) {
    let record = e.mem.u32(cursor);
    let bodies = bodies_of(e, data);
    let numerator = x87_float(e, data + 0x24);
    let factor = x87_float(e, data + 0x20);
    let axis = load(e, atoms + 0x20);
    let anchor = load(e, atoms);
    let total = write_arms(e, record, axis, [anchor, anchor], bodies);
    store_quotient(e, record, numerator, total);
    let self_difference = mul(sub(anchor, anchor), axis);
    let dot = lane_sum(self_difference);
    e.mem
        .set_f32(record + 0xc, rounded(f64::from(dot) * f64::from(factor)));
    let difference = mul(sub(load(e, atoms + 0x10), anchor), axis);
    let dot = lane_sum(difference);
    e.mem
        .set_f32(record + 0xc, rounded(f64::from(factor) * f64::from(dot)));
    let record = e.mem.u32(cursor);
    e.mem.set_u32(cursor, record + 0x30);
    finish_header(e, record, 5);
}

/// The record builder that `fn_00dcf460`, `fn_00dcf5b0` and `fn_00dcf720` share: the arms
/// ([`write_arms`], axis `atoms + 0x20`, both anchors `atoms + 0`), the quotient of the float at
/// `data + 0x24` by the mass term at `+0x1c`, and the float at `data + 0x20` times `scale` at
/// `+0xC`.
fn write_scaled_record(e: &mut Engine, atoms: u32, data: u32, record: u32, scale: f32) {
    let bodies = bodies_of(e, data);
    let numerator = x87_float(e, data + 0x24);
    let axis = load(e, atoms + 0x20);
    let anchor = load(e, atoms);
    let total = write_arms(e, record, axis, [anchor, anchor], bodies);
    store_quotient(e, record, numerator, total);
    let product = f64::from(x87_float(e, data + 0x20)) * f64::from(scale);
    e.mem.set_f32(record + 0xc, rounded(product));
}

// Translated from 00dcf460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00dcf460 - nothing references this code and it reads EDI, ESI and EAX without setting them; they are taken as the first three words, ahead of the stack float.
/// `fn_00dcf460` (unnamed in the Xbox PDB): the same record builder as `fn_00dcf720`, with
/// `atoms`, `data` and the record in `EDI`, `ESI` and `EAX` and only `scale` on the stack.
pub fn fn_00dcf460(e: &mut Engine, atoms: u32, data: u32, record: u32, scale: f32) {
    write_scaled_record(e, atoms, data, record, scale);
}

// Translated from 00dcf5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00dcf5b0` (unnamed in the Xbox PDB; cdecl, four words): `atoms`, `data`, the cursor and a
/// float. Builds the record of `fn_00dcf720` at the cursor ([`write_scaled_record`]), gives it
/// the header with tag 5 and moves the cursor on by 0x30.
pub fn fn_00dcf5b0(e: &mut Engine, atoms: u32, data: u32, cursor: u32, scale: f32) {
    let record = e.mem.u32(cursor);
    write_scaled_record(e, atoms, data, record, scale);
    let record = e.mem.u32(cursor);
    e.mem.set_u32(cursor, record + 0x30);
    finish_header(e, record, 5);
}

// Translated from 00dcf720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00dcf720` (unnamed in the Xbox PDB; cdecl, four words): `atoms`, `data`, the record to
/// fill (not a cursor) and a float. Builds the record ([`write_scaled_record`]); no header and
/// no cursor.
pub fn fn_00dcf720(e: &mut Engine, atoms: u32, data: u32, record: u32, scale: f32) {
    write_scaled_record(e, atoms, data, record, scale);
}

// Translated from 00dcf880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dLinearBilateralConstraintUserTauBuildJacobian` (Xbox PDB; cdecl, three words): `atoms`,
/// `data` and the cursor. Writes a 0x40-byte record of tag 7: the arms ([`write_arms`], axis
/// `atoms + 0x20`, anchors `atoms + 0` and `atoms + 0x10`), 1.0 divided by the mass term at
/// `+0x1c`, the float at `data + 0x20` times the dot product of the axis with
/// `(atoms + 0x10) - (atoms + 0)` at `+0xC`, and the floats at `atoms + 0x30` and `+0x34` at
/// `+0x30` and `+0x34`.
pub fn hk1d_linear_bilateral_constraint_user_tau_build_jacobian(
    e: &mut Engine,
    atoms: u32,
    data: u32,
    cursor: u32,
) {
    let record = e.mem.u32(cursor);
    let bodies = bodies_of(e, data);
    let factor = x87_float(e, data + 0x20);
    let axis = load(e, atoms + 0x20);
    let first_anchor = load(e, atoms);
    let second_anchor = load(e, atoms + 0x10);
    let total = write_arms(e, record, axis, [first_anchor, second_anchor], bodies);
    store_quotient(e, record, 1.0, total);
    let dot = lane_sum(mul(sub(second_anchor, first_anchor), axis));
    e.mem
        .set_f32(record + 0xc, rounded(f64::from(dot) * f64::from(factor)));
    let record = e.mem.u32(cursor);
    e.mem
        .set_u32(record + 0x30, x87_quiet_bits(e.mem.u32(atoms + 0x30)));
    e.mem
        .set_u32(record + 0x34, x87_quiet_bits(e.mem.u32(atoms + 0x34)));
    e.mem.set_u32(cursor, record + 0x40);
    finish_header(e, record, 7);
}

// Translated from 00dcfa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dLinearLimitBuildJacobian` (Xbox PDB; cdecl, three words): `atoms`, `data` and the
/// cursor. Writes a 0x40-byte record of tag 8: the arms ([`write_arms`], axis `atoms + 0x20`,
/// anchors `atoms + 0` and `atoms + 0x10`), the float at `data + 0x24` divided by the mass term
/// at `+0x1c`, the float at `data + 0x20` times the dot product of the axis with
/// `(atoms + 0x10) - (atoms + 0)` at `+0xC`, and, with `negated` the float at `data + 0x20`
/// negated, `(atoms + 0x30) * negated` at `+0x30` and `negated * (atoms + 0x34)` at `+0x34`.
pub fn hk1d_linear_limit_build_jacobian(e: &mut Engine, atoms: u32, data: u32, cursor: u32) {
    let record = e.mem.u32(cursor);
    let bodies = bodies_of(e, data);
    let numerator = x87_float(e, data + 0x24);
    let factor = x87_float(e, data + 0x20);
    let axis = load(e, atoms + 0x20);
    let first_anchor = load(e, atoms);
    let second_anchor = load(e, atoms + 0x10);
    let total = write_arms(e, record, axis, [first_anchor, second_anchor], bodies);
    store_quotient(e, record, numerator, total);
    let dot = lane_sum(mul(sub(second_anchor, first_anchor), axis));
    e.mem
        .set_f32(record + 0xc, rounded(f64::from(dot) * f64::from(factor)));
    let negated = -x87_float(e, data + 0x20);
    let first = x87_float(e, atoms + 0x30);
    let second = x87_float(e, atoms + 0x34);
    let record = e.mem.u32(cursor);
    e.mem.set_u32(cursor, record + 0x40);
    e.mem.set_f32(
        record + 0x30,
        rounded(f64::from(first) * f64::from(negated)),
    );
    e.mem.set_f32(
        record + 0x34,
        rounded(f64::from(negated) * f64::from(second)),
    );
    finish_header(e, record, 8);
}

// Translated from 00dcfc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStabilizedBallSocketConstraintBuildJacobian_noSchema` (Xbox PDB; cdecl, five words):
/// `first` and `second` (anchors), `scale` (a float), `data` (the floats at `+0x20` and
/// `+0x24`, the bodies at `+0x30` and `+0x34`) and the `record` buffer. Runs `00d6c970` on
/// `(first, second, data, record)`, which fills three 0x30-byte records. Takes the floats at
/// `+0xC`, `+0x3C` and `+0x6C` of the buffer as a vector `w`; with `limit` the float at
/// `data + 0x20` times `scale` and `length` the length of `w`, does nothing more unless
/// `limit < length` (a NaN stops it too). Otherwise `r` is the fourth root of `limit / length`
/// (two rounds of the CRT square root, each rounded to a float), the two anchors are
/// interpolated towards the given anchors, `p = (1 - r) * body_position + anchor * r`, and the
/// three records are rebuilt for the three axes kept at 0x01268370 (axis, arms, the mass term
/// plus the `double` constant at `+0x2c`, the float at `data + 0x24` divided by it at `+0x1c`,
/// and at `+0xC` the float at `data + 0x20` times the dot product of the axis with the
/// difference of the two interpolated anchors). The three `+0xC` floats are finally replaced
/// by the saved `w` lanes times `r`.
pub fn hk_stabilized_ball_socket_constraint_build_jacobian_no_schema(
    e: &mut Engine,
    first: u32,
    second: u32,
    scale: f32,
    data: u32,
    record: u32,
) {
    let bodies = bodies_of(e, data);
    e.call(BUILD_BALL_SOCKET, &args![first, second, data, record]);
    let w = [0x0c, 0x3c, 0x6c].map(|offset| x87_float(e, record + offset));
    let limit = rounded(f64::from(x87_float(e, data + 0x20)) * f64::from(scale));
    let squares = [w[0] * w[0], w[1] * w[1], w[2] * w[2]];
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    if !(limit < length) {
        return;
    }
    let mut root = rounded(f64::from(limit) / f64::from(length));
    for _ in 0..2 {
        root = e.call(X87_SQRT, &args![f64::from(root)]).f64() as f32;
    }
    let ones = load(e, ONES);
    let weight = [root; 4];
    let remaining = sub(ones, weight);
    let first_position = add(
        mul(remaining, load(e, bodies[0] + 0x40)),
        mul(load(e, first), weight),
    );
    let second_position = add(
        mul(remaining, load(e, bodies[1] + 0x40)),
        mul(load(e, second), weight),
    );
    let difference = sub(second_position, first_position);
    let addend = e.mem.f64(HEADER_ADDEND);
    for index in 0..3u32 {
        let axis = load(e, AXES + 0x10 * index);
        let numerator = x87_float(e, data + 0x24);
        let factor = x87_float(e, data + 0x20);
        let part = record + 0x30 * index;
        store(e, part, axis);
        let first_offset = sub(first_position, load(e, bodies[0] + 0x40));
        let second_offset = sub(second_position, load(e, bodies[1] + 0x40));
        let first_arm = rotate(e, bodies[0], cross(first_offset, axis));
        let second_arm = rotate(e, bodies[1], cross(axis, second_offset));
        store(e, part + 0x10, first_arm);
        store(e, part + 0x20, second_arm);
        let sum = mass_term(e, bodies[0], bodies[1], first_arm, second_arm);
        let total = rounded(f64::from(sum) + addend);
        e.mem.set_f32(part + 0x2c, total);
        store_quotient(e, part, numerator, total);
        let dot = lane_sum(mul(difference, axis));
        e.mem
            .set_f32(part + 0xc, rounded(f64::from(dot) * f64::from(factor)));
    }
    for (index, offset) in [0x0cu32, 0x3c, 0x6c].into_iter().enumerate() {
        e.mem.set_f32(
            record + offset,
            rounded(f64::from(w[index]) * f64::from(root)),
        );
    }
}

/// The four sign bits (`010c44d0`) the exe negates a vector with (`XORPS`).
const NEGATION_MASK: u32 = 0x010c_44d0;

/// `vector` with every lane's sign flipped (`XORPS` with [`NEGATION_MASK`]).
fn negated(e: &Engine, vector: Lanes) -> Lanes {
    let mask = [0, 1, 2, 3].map(|lane| e.mem.u32(NEGATION_MASK + 4 * lane));
    [0, 1, 2, 3].map(|lane| f32::from_bits(vector[lane].to_bits() ^ mask[lane]))
}

// Translated from 00dcee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A one-row angular record (no engine-map name; cdecl, three words: `block`, the info block and
/// the cursor). `block` holds the axis at `+0x10` and two triples at `+0` and `+0x20`; the info
/// block the bodies at `+0x30`/`+0x34`, floats at `+0x20` and `+0x24`. Writes at the cursor
/// record: the axis carried through body A's rotation rows at `+0`, the negated axis through
/// body B's at `+0x10`, `info[0x24] / (sum + FLT_EPSILON)` at `+0xc` (the sum of the squared
/// terms times the inverse inertias, `z + (y + x)`), `-(b[9] * b[1] + b[0] * b[8] + b[10] * b[2])
/// * info[0x20]` at `+0x1c` (x87), and gives the record the header with tag 0xc. The cursor
/// moves on by 0x20.
pub fn fn_00dcee80(e: &mut Engine, block: u32, info: u32, cursor: u32) {
    let record = e.mem.u32(cursor);
    let axis = load(e, block + 0x10);
    let opposite = negated(e, axis);
    let bodies = bodies_of(e, info);
    let tau = x87_float(e, info + 0x24);
    let first = rotate(e, bodies[0], axis);
    store(e, record, first);
    let second = rotate(e, bodies[1], opposite);
    store(e, record + 0x10, second);
    let first = load(e, record);
    let terms = add(
        mul(mul(first, first), load(e, bodies[0] + 0x30)),
        mul(mul(second, second), load(e, bodies[1] + 0x30)),
    );
    let sum = lane_sum(terms);
    let addend = e.mem.f64(HEADER_ADDEND);
    let denominator = rounded(f64::from(sum) + addend);
    e.mem.set_f32(
        record + 0xc,
        rounded(x87_div(f64::from(tau), f64::from(x87_quiet(denominator)))),
    );
    let stored = x87_store32(x87_add(
        x87_add(
            x87_mul(
                x87_load32(e.mem.u32(block + 0x24)),
                x87_m32(e.mem.u32(block + 4)),
            ),
            x87_mul(
                x87_load32(e.mem.u32(block)),
                x87_m32(e.mem.u32(block + 0x20)),
            ),
        ),
        x87_mul(
            x87_load32(e.mem.u32(block + 0x28)),
            x87_m32(e.mem.u32(block + 8)),
        ),
    ));
    let negative = -x87_load32(stored);
    e.mem.set_f32(
        record + 0x1c,
        f32::from_bits(x87_store32(x87_mul(
            negative,
            x87_m32(e.mem.u32(info + 0x20)),
        ))),
    );
    e.mem.set_u32(cursor, record + 0x20);
    let first_word = f64::from(x87_float(e, record));
    e.mem.set_f32(record, rounded(addend + first_word));
    e.mem.set_u8(record, 0xc);
}

// Translated from 00dcefb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dAngularFrictionBuildJacobian` (Xbox PDB; cdecl, three words: `block`, the info block and
/// the cursor). `block` holds a pointer to `count` axis vectors at `+0`, a pointer to `count`
/// pairs of floats (8 bytes each) at `+4`, a float at `+8` and `count` at `+0xc`; the info block the
/// floats at `+4` and `+8` and the bodies at `+0x30`/`+0x34`. For each axis a 0x30-byte record:
/// the axis through body A's rotation rows at `+0`, the negated axis through body B's at `+0x10`,
/// `1 / (sum + FLT_EPSILON)` at `+0xc` (the sum of the squared terms times the inverse
/// inertias, `z + (y + x)`, rounded to float first), the second float of the pair times
/// `info[8]` at `+0x1c`, `info[4] * block[8]` at `+0x20`, and the header with tag 0xe. With no
/// axis only the cursor is rewritten (unchanged).
pub fn hk1d_angular_friction_build_jacobian(e: &mut Engine, block: u32, info: u32, cursor: u32) {
    let axes = e.mem.u32(block);
    let pairs = e.mem.u32(block + 4);
    let mut record = e.mem.u32(cursor);
    let mut remaining = (e.mem.u32(block + 0xc) as i32).wrapping_sub(1);
    if remaining < 0 {
        e.mem.set_u32(cursor, record);
        return;
    }
    let addend = e.mem.f64(HEADER_ADDEND);
    let (mut axis_at, mut pair_at) = (axes, pairs + 4);
    loop {
        let axis = load(e, axis_at);
        let opposite = negated(e, axis);
        let bodies = bodies_of(e, info);
        let first = rotate(e, bodies[0], axis);
        store(e, record, first);
        let second = rotate(e, bodies[1], opposite);
        store(e, record + 0x10, second);
        let first = load(e, record);
        let terms = add(
            mul(mul(first, first), load(e, bodies[0] + 0x30)),
            mul(mul(second, second), load(e, bodies[1] + 0x30)),
        );
        let sum = lane_sum(terms);
        let denominator = rounded(f64::from(sum) + addend);
        let this_record = record;
        record += 0x30;
        axis_at += 0x10;
        remaining -= 1;
        e.mem.set_f32(
            this_record + 0xc,
            rounded(1.0 / f64::from(x87_quiet(denominator))),
        );
        let pair_second = f64::from(x87_float(e, pair_at));
        pair_at += 8;
        e.mem.set_f32(
            this_record + 0x1c,
            rounded(pair_second * f64::from(x87_float(e, info + 8))),
        );
        e.mem.set_f32(
            this_record + 0x20,
            rounded(f64::from(x87_float(e, info + 4)) * f64::from(x87_float(e, block + 8))),
        );
        finish_header(e, this_record, 0xe);
        if remaining < 0 {
            break;
        }
    }
    e.mem.set_u32(cursor, record);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_ee80, fn_00dcee80(u32, u32, u32)),
        entry!(
            0x00dc_efb0,
            hk1d_angular_friction_build_jacobian(u32, u32, u32)
        ),
        entry!(0x00dcedd0, hk_add_velocity_build_jacobian(u32, u32, u32)),
        entry!(
            0x00dcee10,
            hk_set_center_of_mass_build_jacobian(u32, u32, u32)
        ),
        entry!(0x00dcee70, hk_load_velocity_accumulators(u32)),
        entry!(
            0x00dcf100,
            hk1d_linear_friction_build_jacobian(u32, u32, u32)
        ),
        entry!(
            0x00dcf280,
            hk1d_linear_bilateral_constraint_build_jacobian(u32, u32, u32)
        ),
        entry!(0x00dcf460, fn_00dcf460(u32, u32, u32, f32)),
        entry!(0x00dcf5b0, fn_00dcf5b0(u32, u32, u32, f32)),
        entry!(0x00dcf720, fn_00dcf720(u32, u32, u32, f32)),
        entry!(
            0x00dcf880,
            hk1d_linear_bilateral_constraint_user_tau_build_jacobian(u32, u32, u32)
        ),
        entry!(0x00dcfa30, hk1d_linear_limit_build_jacobian(u32, u32, u32)),
        entry!(
            0x00dcfc00,
            hk_stabilized_ball_socket_constraint_build_jacobian_no_schema(u32, u32, f32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    /// The constants mapped: the header addend 2^-23, four ones and three axes.
    fn engine() -> Engine {
        let mut e = logging_engine();
        e.map(HEADER_ADDEND, 8);
        e.mem.set_f64(HEADER_ADDEND, 1.0 / 8_388_608.0);
        e.map(ONES, 16);
        store(&mut e, ONES, [1.0; 4]);
        e.map(AXES, 48);
        store(&mut e, AXES, [1.0, 0.0, 0.0, 0.0]);
        store(&mut e, AXES + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store(&mut e, AXES + 0x20, [0.0, 0.0, 1.0, 0.0]);
        e
    }

    /// A body at `position` with identity rows and the inverse-mass vector `mass`.
    fn body(e: &mut Engine, position: [f32; 3], mass: Lanes) -> u32 {
        let body = e.mem.alloc(0x80);
        store(e, body + 0x30, mass);
        store(e, body + 0x40, [position[0], position[1], position[2], 0.0]);
        store(e, body + 0x50, [1.0, 0.0, 0.0, 0.0]);
        store(e, body + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store(e, body + 0x70, [0.0, 0.0, 1.0, 0.0]);
        body
    }

    /// Runtime data for two bodies (A at (1, 0, 0), B at (0, 2, 0)) with the floats at
    /// `+0x20` and `+0x24`.
    fn data(e: &mut Engine, at_20: f32, at_24: f32) -> u32 {
        let first = body(e, [1.0, 0.0, 0.0], [1.0, 1.0, 1.0, 2.0]);
        let second = body(e, [0.0, 2.0, 0.0], [1.0, 1.0, 1.0, 3.0]);
        let data = e.mem.alloc(0x40);
        e.mem.set_f32(data + 0x20, at_20);
        e.mem.set_f32(data + 0x24, at_24);
        e.mem.set_u32(data + 0x30, first);
        e.mem.set_u32(data + 0x34, second);
        data
    }

    /// A cursor word pointing at a fresh 0x100-byte buffer; answers (cursor, buffer).
    fn cursor(e: &mut Engine) -> (u32, u32) {
        let buffer = e.mem.alloc(0x100);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, buffer);
        (cursor, buffer)
    }

    /// Atoms: the three vectors, zeroed beyond what the test sets.
    fn atoms(e: &mut Engine, rows: [Lanes; 3]) -> u32 {
        let atoms = e.mem.alloc(0x40);
        for (i, row) in rows.iter().enumerate() {
            store(e, atoms + 0x10 * i as u32, *row);
        }
        atoms
    }

    /// The header bits of a record whose first float was `first`.
    fn header_bits(first: f32, tag: u8) -> u32 {
        let sum = (f64::from(first) + 1.0 / 8_388_608.0) as f32;
        (sum.to_bits() & !0xff) | u32::from(tag)
    }

    #[test]
    fn add_velocity_copies_the_vector_and_the_word() {
        let mut e = engine();
        let (cursor, buffer) = cursor(&mut e);
        let vector = e.mem.alloc(0x10);
        store(&mut e, vector, [2.0, 3.0, 4.0, 5.0]);
        e.call(0x00dc_edd0, &args![vector, 0x1234u32, cursor]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x20);
        assert_eq!(e.mem.u32(buffer), header_bits(2.0, 0x1a));
        assert_eq!(load(&e, buffer)[1..], [3.0, 4.0, 5.0]);
        assert_eq!(e.mem.u32(buffer + 0x10), 0x1234);
    }

    #[test]
    fn set_center_of_mass_copies_both_blocks() {
        let mut e = engine();
        let (cursor, buffer) = cursor(&mut e);
        let first = e.mem.alloc(0x30);
        let second = e.mem.alloc(0x30);
        for row in 0..3u32 {
            store(&mut e, first + 0x10 * row, [row as f32 + 1.0; 4]);
            store(&mut e, second + 0x10 * row, [row as f32 + 10.0; 4]);
        }
        e.call(0x00dc_ee10, &args![first, second, cursor]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x60);
        assert_eq!(e.mem.u32(buffer), header_bits(1.0, 0x1b));
        assert_eq!(load(&e, buffer + 0x20), [3.0; 4]);
        assert_eq!(load(&e, buffer + 0x30), [10.0; 4]);
        assert_eq!(load(&e, buffer + 0x50), [12.0; 4]);
    }

    #[test]
    fn load_velocity_accumulators_does_nothing() {
        let mut e = engine();
        let before = e.mem.alloc(8);
        e.mem.set_u32(before, 0x1234);
        let result = e.call(0x00dc_ee70, &args![before]);
        assert_eq!(result.eax, 0);
        assert_eq!(e.mem.u32(before), 0x1234);
    }

    #[test]
    fn friction_record_has_the_arms_the_quotient_and_the_products() {
        let mut e = engine();
        let (cursor, buffer) = cursor(&mut e);
        let data = data(&mut e, 0.5, 4.0);
        e.mem.set_f32(data + 4, 6.0);
        let atoms = atoms(&mut e, [[0.0; 4], [0.0, 0.0, 1.0, 0.0], [0.0; 4]]);
        // The float at atoms+0x20 and a pointer at atoms+0x24 (the pointed object's +4 is 3).
        let pointed = e.mem.alloc(0x10);
        e.mem.set_f32(pointed + 4, 3.0);
        e.mem.set_f32(atoms + 0x20, 2.0);
        e.mem.set_u32(atoms + 0x24, pointed);
        e.call(0x00dc_f100, &args![atoms, data, cursor]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x40);
        assert_eq!(e.mem.u32(buffer), header_bits(0.0, 9));
        assert_eq!(load(&e, buffer + 0x10)[..3], [0.0, 1.0, 0.0]);
        assert_eq!(load(&e, buffer + 0x20)[..3], [2.0, 0.0, 0.0]);
        // The mass term: (0 + 4) + (5 + 1) = 10.
        assert_eq!(e.mem.f32(buffer + 0x2c), 10.0);
        assert_eq!(e.mem.f32(buffer + 0x1c), 0.4);
        assert_eq!(e.mem.f32(buffer + 0xc), 1.5);
        assert_eq!(e.mem.f32(buffer + 0x30), 12.0);
    }

    #[test]
    fn linear_bilateral_record_has_the_axis_and_the_dot_product() {
        let mut e = engine();
        let (cursor, buffer) = cursor(&mut e);
        let data = data(&mut e, 0.5, 4.0);
        let atoms = atoms(
            &mut e,
            [[0.0; 4], [0.0, 0.0, 2.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
        );
        e.call(0x00dc_f280, &args![atoms, data, cursor]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x30);
        assert_eq!(e.mem.u32(buffer), header_bits(0.0, 5));
        assert_eq!(load(&e, buffer + 0x10)[..3], [0.0, 1.0, 0.0]);
        assert_eq!(load(&e, buffer + 0x20)[..3], [2.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(buffer + 0x2c), 10.0);
        assert_eq!(e.mem.f32(buffer + 0x1c), 0.4);
        // 0.5 * dot((0, 0, 2), axis).
        assert_eq!(e.mem.f32(buffer + 0xc), 1.0);
    }

    #[test]
    fn scaled_records_agree_and_differ_in_the_header_and_cursor() {
        let mut e = engine();
        let data = data(&mut e, 0.5, 4.0);
        let atoms = atoms(&mut e, [[0.0; 4], [0.0; 4], [0.0, 0.0, 1.0, 0.0]]);
        let plain = e.mem.alloc(0x100);
        e.call(0x00dc_f720, &args![atoms, data, plain, 3.0f32]);
        assert_eq!(load(&e, plain + 0x10)[..3], [0.0, 1.0, 0.0]);
        assert_eq!(load(&e, plain + 0x20)[..3], [2.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(plain + 0x2c), 10.0);
        assert_eq!(e.mem.f32(plain + 0x1c), 0.4);
        assert_eq!(e.mem.f32(plain + 0xc), 1.5);
        // No header: the first float is the axis' first lane.
        assert_eq!(e.mem.f32(plain), 0.0);

        let orphan = e.mem.alloc(0x100);
        e.call(0x00dc_f460, &args![atoms, data, orphan, 3.0f32]);
        for offset in [0x0c, 0x1c, 0x2c, 0x10, 0x20] {
            assert_eq!(e.mem.u32(orphan + offset), e.mem.u32(plain + offset));
        }

        let (cursor, buffer) = cursor(&mut e);
        e.call(0x00dc_f5b0, &args![atoms, data, cursor, 3.0f32]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x30);
        assert_eq!(e.mem.u32(buffer), header_bits(0.0, 5));
        for offset in [0x0c, 0x1c, 0x2c, 0x10, 0x20] {
            assert_eq!(e.mem.u32(buffer + offset), e.mem.u32(plain + offset));
        }
    }

    #[test]
    fn user_tau_record_inverts_the_mass_term_and_copies_two_floats() {
        let mut e = engine();
        let (cursor, buffer) = cursor(&mut e);
        let data = data(&mut e, 0.5, 4.0);
        let atoms = atoms(
            &mut e,
            [[0.0; 4], [0.0, 0.0, 2.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
        );
        e.mem.set_f32(atoms + 0x30, 7.0);
        e.mem.set_u32(atoms + 0x34, 0x7f80_0001);
        e.call(0x00dc_f880, &args![atoms, data, cursor]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x40);
        assert_eq!(e.mem.u32(buffer), header_bits(0.0, 7));
        assert_eq!(e.mem.f32(buffer + 0x2c), 10.0);
        assert_eq!(e.mem.f32(buffer + 0x1c), 0.1);
        assert_eq!(e.mem.f32(buffer + 0xc), 1.0);
        assert_eq!(e.mem.f32(buffer + 0x30), 7.0);
        assert_eq!(e.mem.u32(buffer + 0x34), 0x7fc0_0001);
    }

    #[test]
    fn limit_record_negates_the_float_for_the_last_two_products() {
        let mut e = engine();
        let (cursor, buffer) = cursor(&mut e);
        let data = data(&mut e, 0.5, 4.0);
        let atoms = atoms(
            &mut e,
            [[0.0; 4], [0.0, 0.0, 2.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
        );
        e.mem.set_f32(atoms + 0x30, 8.0);
        e.mem.set_f32(atoms + 0x34, 6.0);
        e.call(0x00dc_fa30, &args![atoms, data, cursor]);
        assert_eq!(e.mem.u32(cursor), buffer + 0x40);
        assert_eq!(e.mem.u32(buffer), header_bits(0.0, 8));
        assert_eq!(e.mem.f32(buffer + 0x1c), 0.4);
        assert_eq!(e.mem.f32(buffer + 0xc), 1.0);
        assert_eq!(e.mem.f32(buffer + 0x30), -4.0);
        assert_eq!(e.mem.f32(buffer + 0x34), -3.0);
    }

    /// The ball-socket case: the double for `00d6c970` leaves `w` in the three `+0xC` floats.
    /// The anchors are (0, 0, 0) and (0, 4, 0). Answers the engine and the four words of the call.
    fn ball_socket(w: [f32; 3]) -> (Engine, [u32; 4]) {
        let mut e = engine();
        e.register_double(BUILD_BALL_SOCKET, move |e, a| {
            for (index, offset) in [0x0cu32, 0x3c, 0x6c].into_iter().enumerate() {
                e.mem.set_f32(a[3] + offset, w[index]);
            }
            Ret::default()
        });
        e.register(X87_SQRT, |_, a| {
            let value = f64::from_bits((u64::from(a[1]) << 32) | u64::from(a[0]));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        let data = data(&mut e, 4.0, 5.0);
        let record = e.mem.alloc(0x100);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        store(&mut e, second, [0.0, 4.0, 0.0, 0.0]);
        (e, [first, second, data, record])
    }

    #[test]
    fn ball_socket_rebuilds_the_three_records_for_the_interpolated_anchors() {
        let (mut e, [first, second, data, record]) = ball_socket([0.0, 0.0, 16.0]);
        // limit = 4 * 0.25 = 1, length = 16: the fourth root of 1/16 is 1/2.
        e.call(0x00dc_fc00, &args![first, second, 0.25f32, data, record]);
        assert_eq!(
            calls_to(&e, BUILD_BALL_SOCKET),
            [[first, second, data, record]]
        );
        assert_eq!(calls_to(&e, X87_SQRT).len(), 2);
        // Axes x, y, z.
        assert_eq!(load(&e, record), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load(&e, record + 0x30)[..3], [0.0, 1.0, 0.0]);
        assert_eq!(load(&e, record + 0x60)[..3], [0.0, 0.0, 1.0]);
        assert_eq!(load(&e, record + 0x20)[..3], [0.0, 0.0, 1.0]);
        assert_eq!(load(&e, record + 0x40)[..3], [0.0, 0.0, -0.5]);
        assert_eq!(load(&e, record + 0x50)[..3], [0.0; 3]);
        assert_eq!(load(&e, record + 0x70)[..3], [0.0, 0.5, 0.0]);
        assert_eq!(load(&e, record + 0x80)[..3], [-1.0, 0.0, 0.0]);
        // The mass terms 6, 5.25 and 6.25 and the quotients of the float 5 by them.
        for (record_offset, total) in [(0u32, 6.0f32), (0x30, 5.25), (0x60, 6.25)] {
            assert_eq!(e.mem.f32(record + record_offset + 0x2c), total);
            assert_eq!(
                e.mem.f32(record + record_offset + 0x1c),
                (5.0f64 / f64::from(total)) as f32
            );
        }
        // The +0xC floats end as w times the root.
        assert_eq!(e.mem.f32(record + 0x0c), 0.0);
        assert_eq!(e.mem.f32(record + 0x3c), 0.0);
        assert_eq!(e.mem.f32(record + 0x6c), 8.0);
    }

    #[test]
    fn ball_socket_stops_after_the_helper_when_the_limit_is_not_below_the_length() {
        let (mut e, [first, second, data, record]) = ball_socket([0.0, 0.0, 1.0]);
        e.call(0x00dc_fc00, &args![first, second, 0.25f32, data, record]);
        assert!(calls_to(&e, X87_SQRT).is_empty());
        assert_eq!(e.mem.f32(record + 0x6c), 1.0);
        assert_eq!(load(&e, record), [0.0; 4]);
        // A NaN lane of w makes the length NaN, which stops it too.
        let (mut e, [first, second, data, record]) = ball_socket([f32::NAN, 0.0, 0.0]);
        e.call(0x00dc_fc00, &args![first, second, 0.25f32, data, record]);
        assert!(calls_to(&e, X87_SQRT).is_empty());
        assert!(e.mem.f32(record + 0x0c).is_nan());
    }
}

#[cfg(test)]
mod angular_record_tests {
    use super::*;

    /// An engine with the image constants and two bodies with identity rotation rows and
    /// inverse inertias (1, 1, 1, 0) hanging off an info block.
    fn rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(HEADER_ADDEND, 1.1920929e-7f64);
        for lane in 0..4 {
            e.mem.set_u32(NEGATION_MASK + 4 * lane, 0x8000_0000);
        }
        let info = e.mem.alloc(0x80);
        let mut bodies = [0u32; 2];
        for body in &mut bodies {
            *body = e.mem.alloc(0x100);
            for (row, lanes) in [
                (0x50u32, [1.0f32, 0.0, 0.0, 0.0]),
                (0x60, [0.0, 1.0, 0.0, 0.0]),
                (0x70, [0.0, 0.0, 1.0, 0.0]),
                (0x30, [1.0, 1.0, 1.0, 0.0]),
            ] {
                for (lane, value) in lanes.into_iter().enumerate() {
                    e.mem.set_f32(*body + row + 4 * lane as u32, value);
                }
            }
        }
        e.mem.set_u32(info + 0x30, bodies[0]);
        e.mem.set_u32(info + 0x34, bodies[1]);
        let records = e.mem.alloc(0x100);
        (e, info, records)
    }

    #[test]
    fn the_angular_friction_records_carry_each_axis_through_both_bodies() {
        let (mut e, info, records) = rig();
        e.mem.set_f32(info + 4, 3.0);
        e.mem.set_f32(info + 8, 0.5);
        let axes = e.mem.alloc(0x20);
        for (lane, value) in [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(axes + 4 * lane as u32, value);
        }
        let pairs = e.mem.alloc(0x10);
        for (i, value) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
            e.mem.set_f32(pairs + 4 * i as u32, value);
        }
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, axes);
        e.mem.set_u32(block + 4, pairs);
        e.mem.set_f32(block + 8, 4.0);
        e.mem.set_u32(block + 0xc, 2);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        e.call(0x00dc_efb0, &args![block, info, cursor]);
        assert_eq!(e.mem.u32(cursor), records + 0x60);
        // First axis (1, 0, 0): body A turns it into (1, 0, 0), body B into (-1, 0, 0).
        assert_eq!(e.mem.f32(records + 0x10), -1.0);
        let epsilon = (1.0f64 + 1.1920929e-7) as f32;
        assert_eq!(e.mem.u32(records), epsilon.to_bits() & !0xff | 0xe);
        // 1 / (2 + FLT_EPSILON), the sum rounded to float first.
        let denominator = f64::from((2.0 + 1.1920929e-7f64) as f32);
        assert_eq!(e.mem.f32(records + 0xc), (1.0 / denominator) as f32);
        // The second float of the pair times info[2], and info[1] times block[2].
        assert_eq!(e.mem.f32(records + 0x1c), 20.0 * 0.5);
        assert_eq!(e.mem.f32(records + 0x20), 3.0 * 4.0);
        // Second axis (0, 1, 0): the header float is 0 + FLT_EPSILON, the pair is the next one.
        assert_eq!(e.mem.f32(records + 0x30 + 4), 1.0);
        assert_eq!(e.mem.u32(records + 0x30), 0x3400_000e);
        assert_eq!(e.mem.f32(records + 0x30 + 0x1c), 40.0 * 0.5);
    }

    #[test]
    fn the_angular_friction_without_axes_leaves_the_cursor_alone() {
        let (mut e, info, records) = rig();
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 0xc, 0);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        e.call(0x00dc_efb0, &args![block, info, cursor]);
        assert_eq!(e.mem.u32(cursor), records);
        assert_eq!(e.mem.u32(records), 0);
    }

    #[test]
    fn the_one_row_angular_record_negates_the_axis_for_the_second_body() {
        let (mut e, info, records) = rig();
        e.mem.set_f32(info + 0x20, 0.5);
        e.mem.set_f32(info + 0x24, 6.0);
        let block = e.mem.alloc(0x40);
        for (lane, value) in [0.0f32, 0.0, 1.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(block + 0x10 + 4 * lane as u32, value);
        }
        for (offset, value) in [
            (0u32, 2.0f32),
            (4, 3.0),
            (8, 4.0),
            (0x20, 5.0),
            (0x24, 6.0),
            (0x28, 7.0),
        ] {
            e.mem.set_f32(block + offset, value);
        }
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, records);
        e.call(0x00dc_ee80, &args![block, info, cursor]);
        assert_eq!(e.mem.u32(cursor), records + 0x20);
        assert_eq!(e.mem.f32(records + 8), 1.0);
        assert_eq!(e.mem.f32(records + 0x10 + 8), -1.0);
        // 6 / (2 + FLT_EPSILON).
        assert_eq!(
            e.mem.f32(records + 0xc),
            (6.0 / (2.0 + 1.1920929e-7f64)) as f32
        );
        // -(6 * 3 + 2 * 5 + 7 * 4) * 0.5 = -28.
        assert_eq!(e.mem.f32(records + 0x1c), -28.0);
        assert_eq!(e.mem.u32(records), 0x3400_000c);
    }
}
