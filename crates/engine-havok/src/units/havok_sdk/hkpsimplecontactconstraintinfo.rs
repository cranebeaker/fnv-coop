//! `Havok SDK/hkpsimplecontactconstraintinfo.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, andnps, andps, cmpeqss, fcom_status, load_vec, mulps, mulss, rsqrtss, shufps,
    sqrtss, store_vec, subps, subss, unpckhps, xorps, Vec4,
};

/// The double `2^-23` (`FLD qword`) the jacobian builder adds to the first float of each
/// schema it writes and to each effective mass.
const EPSILON_010C9300: u32 = 0x010c_9300;
/// The float `-2^-23`: the largest value the builder leaves in a contact point's
/// `m_internalDataA`.
const NEGATIVE_EPSILON_010C8440: u32 = 0x010c_8440;
/// A table of floats indexed by a contact point's `m_friction` and `m_maxImpulse` bytes (both
/// `hkUFloat8` in the Xbox PDB).
const UFLOAT8_TABLE_010C7948: u32 = 0x010c_7948;
/// Two float globals of the recovery term (in `.data`; 1.0 and 0.05 in the exe image).
const RECOVERY_SCALE_011B6924: u32 = 0x011b_6924;
const RECOVERY_LIMIT_011B6928: u32 = 0x011b_6928;
/// The double (0.99) that scales the coupling of a contact point with the previous one.
const COUPLING_SCALE_0106D508: u32 = 0x0106_d508;
/// The float zero the normalizer compares a squared length with.
const ZERO_01011D78: u32 = 0x0101_1d78;
/// Doubles 0.9999, 0.1 and 0.9 of the averaged-normal checks.
const NEARLY_UNIT_010CC5D0: u32 = 0x010c_c5d0;
const ONE_TENTH_0101FFA0: u32 = 0x0101_ffa0;
const NINE_TENTHS_0106B9E8: u32 = 0x0106_b9e8;
/// Floats 3.0 and 0.5 of the Newton step after `RSQRTSS`.
const THREE_01017718: u32 = 0x0101_7718;
const HALF_01016248: u32 = 0x0101_6248;
/// A table of 16-byte axis vectors in `.data` (zero in the exe image, filled at run time),
/// indexed by the atom's `m_info.m_index`.
const AXIS_TABLE_01268370: u32 = 0x0126_8370;
/// Sixteen-byte masks: every lane `0x7fffffff` (absolute value), every lane `0x80000000`
/// (negation).
const ABS_MASK_01017D10: u32 = 0x0101_7d10;
const SIGN_MASK_010C44D0: u32 = 0x010c_44d0;
/// The float `1e-6`: the smallest average distance of the contact points from their centre.
const MIN_RADIUS_010959F4: u32 = 0x0109_59f4;

/// A value stored by `FSTP dword` and loaded again by `FLD dword`.
fn round_trip(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `FLD dword` of the float at `address`.
fn fld(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// The float at `address` as the memory operand of an x87 arithmetic instruction.
fn m32(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// `FSTP dword` of `value` to `address`.
fn fstp(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_u32(address, x87_store32(value));
}

/// `JP` after `FNSTSW AX; TEST AH,5`: an even number of `C0` and `C2` (`ST0` above the
/// operand, equal to it, or unordered).
fn parity_even(status: u8) -> bool {
    matches!(status & 5, 0 | 5)
}

/// `x + y + z` of a register in lane 0 (`SHUFPS 0x55; ADDSS; SHUFPS 0xaa; ADDSS`).
fn sum3(v: Vec4) -> Vec4 {
    let yx = addss(shufps(v, v, 0x55), v);
    addss(shufps(v, v, 0xaa), yx)
}

/// A vector times a velocity accumulator's `m_scratch123` rotation (Xbox PDB, rows at +0x50,
/// +0x60, +0x70) in the exe's order: `v.y * row1 + v.x * row0`, then `+ v.z * row2`.
fn rotate(e: &Engine, accumulator: u32, v: Vec4) -> Vec4 {
    let sum = addps(
        mulps(
            shufps(v, v, 0x55),
            load_vec(e, accumulator.wrapping_add(0x60)),
        ),
        mulps(
            shufps(v, v, 0x00),
            load_vec(e, accumulator.wrapping_add(0x50)),
        ),
    );
    addps(
        sum,
        mulps(
            shufps(v, v, 0xaa),
            load_vec(e, accumulator.wrapping_add(0x70)),
        ),
    )
}

/// `m_invMasses` (Xbox PDB `hkpVelocityAccumulator` +0x30) of an accumulator.
fn inverse_masses(e: &Engine, accumulator: u32) -> Vec4 {
    load_vec(e, accumulator.wrapping_add(0x30))
}

/// The effective-mass sum of one row: the two angular terms (`first + second`) with lane 3
/// replaced by the sum of the two inverse masses (lane 3 of `masses_a + masses_b`), added
/// across the four lanes (`UNPCKHPS; SHUFPS 0xc4; SHUFPS 0x4e; ADDPS; SHUFPS 0xb1; ADDPS`).
fn mass_sum(first: Vec4, second: Vec4, masses_a: Vec4, masses_b: Vec4) -> f32 {
    let terms = addps(first, second);
    let masses = addps(masses_a, masses_b);
    let high = unpckhps(terms, masses);
    let joined = shufps(terms, high, 0xc4);
    let halves = addps(shufps(joined, joined, 0x4e), joined);
    addps(shufps(halves, halves, 0xb1), halves)[0]
}

/// `RSQRTSS` and one Newton step on lane 0 of `sum`: `0.5 * r * (3 - x * r * r)`, or 0 when
/// the squared length equals the float at `01011d78` (lane 0 of `zero`); the other lanes come
/// out zero.
fn reciprocal_length(e: &Engine, sum: Vec4, zero: Vec4) -> Vec4 {
    let estimate = rsqrtss(sum, sum);
    let zero_register = [zero[0], 0.0, 0.0, 0.0];
    let product = mulss(mulss(sum, estimate), estimate);
    let mask = cmpeqss(sum, zero_register);
    let three = [e.mem.f32(THREE_01017718), 0.0, 0.0, 0.0];
    let difference = subss(three, product);
    let half = [e.mem.f32(HALF_01016248), 0.0, 0.0, 0.0];
    let factor = mulss(mulss(half, estimate), difference);
    andnps(mask, factor)
}

/// The x87 block that couples two rows with coupling term `k`:
/// `det = loaded_first * loaded_second - k * k`, `inv = virt_mass / det` (stored as a float),
/// then `(loaded_first * inv, loaded_second * inv, inv * -k)`. `loaded_first` is the mass the
/// exe loads first.
fn pair_terms(k: f64, virt_mass: f64, loaded_first: f64, loaded_second: f64) -> (f64, f64, f64) {
    let k_squared = x87_mul(k, k);
    let product = x87_mul(loaded_first, loaded_second);
    let det = x87_sub(product, k_squared);
    let inverse = round_trip(x87_div(virt_mass, det));
    (
        x87_mul(loaded_first, inverse),
        x87_mul(loaded_second, inverse),
        x87_mul(inverse, -k),
    )
}

/// One friction row at `row` along `direction` through the contact centre `centre`: the
/// direction (+0x0), the angular parts of body A (+0x10) and body B (+0x20), the effective
/// mass (+0x2c), `m_virtMassFactor / mass` (+0x1c) and the atom float at `atom_field` times
/// `m_frictionRhsFactor` (+0xc).
#[allow(clippy::too_many_arguments)] // the registers and stack slots the exe's block reads
fn friction_row(
    e: &mut Engine,
    row: u32,
    direction: Vec4,
    centre: Vec4,
    body_a: u32,
    body_b: u32,
    query: u32,
    epsilon: f64,
    atom_field: u32,
) {
    // hkpConstraintQueryIn::m_virtMassFactor (Xbox PDB) +0x24, kept in a stack float.
    let virt_mass_bits = x87_quiet_bits(e.mem.u32(query + 0x24));
    store_vec(e, row, direction);
    let dir_d2 = shufps(direction, direction, 0xd2);
    let dir_c9 = shufps(direction, direction, 0xc9);
    // hkpVelocityAccumulator::m_scratch0 (Xbox PDB) +0x40: the body's centre.
    let ra = subps(centre, load_vec(e, body_a.wrapping_add(0x40)));
    let rb = subps(centre, load_vec(e, body_b.wrapping_add(0x40)));
    let cross_a = subps(
        mulps(shufps(ra, ra, 0xc9), dir_d2),
        mulps(shufps(ra, ra, 0xd2), dir_c9),
    );
    let cross_b = subps(
        mulps(shufps(rb, rb, 0xd2), dir_c9),
        mulps(shufps(rb, rb, 0xc9), dir_d2),
    );
    let angular_a = rotate(e, body_a, cross_a);
    store_vec(e, row + 0x10, angular_a);
    let angular_b = rotate(e, body_b, cross_b);
    store_vec(e, row + 0x20, angular_b);
    let masses_b = inverse_masses(e, body_b);
    let masses_a = inverse_masses(e, body_a);
    let sum = mass_sum(
        mulps(mulps(angular_a, angular_a), masses_a),
        mulps(mulps(angular_b, angular_b), masses_b),
        masses_a,
        masses_b,
    );
    let mass = round_trip(x87_add(x87_load32(sum.to_bits()), epsilon));
    fstp(e, row + 0x2c, mass);
    fstp(e, row + 0x1c, x87_div(x87_m32(virt_mass_bits), mass));
    // hkpConstraintQueryIn::m_frictionRhsFactor (Xbox PDB) +0x28.
    let rhs = x87_mul(fld(e, atom_field), m32(e, query + 0x28));
    fstp(e, row + 0xc, rhs);
}

// Translated from 00d72090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleContactConstraintDataAddALittleSurfaceViscosity` (Xbox PDB), cdecl: clears the
/// floats at +0xc, +0x14 and +0x1c of `data` (the exe stores the x87 zero with `FST`).
pub fn hk_simple_contact_constraint_data_add_a_little_surface_viscosity(e: &mut Engine, data: Ptr) {
    let base = data.addr();
    e.mem.set_f32(base + 0xc, 0.0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x1c, 0.0);
}

// Translated from 00d72190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleContactConstraintDataBuildJacobian` (Xbox PDB), cdecl `(atom, query,
/// write_header, schema_cursor)`: writes the solver schemas of a
/// `hkpSimpleContactConstraintAtom` (Xbox PDB) at `*schema_cursor` and advances the cursor.
///
/// - `write_header != 0`: a 0x10-byte header (type byte 1, the property striding, the two
///   accumulator indices of the `hkpConstraintQueryIn`, the address of the contact point
///   properties, `m_constraintInstance`).
/// - Each contact point (a do-while loop: it runs once even with no points): a 0x30-byte
///   row along its normal (type 0x10, or 0x12 with the point's `m_maxImpulse` table value at
///   +0x2c when that byte is nonzero), with the point's recovery term clamped into
///   `m_internalDataA` (at most `-2^-23`). A point whose `m_flags` has bit 1 set is instead
///   coupled with the previous row (that row's type becomes 0x14, this row takes 0x40
///   bytes).
/// - When the average of the `m_friction` table values times the sum of `m_impulseApplied`
///   over the points is above zero: two friction rows (type 0x16, 0x70 bytes) along the
///   tangents of the averaged normal, the tangent axis picked by `m_info.m_index` (reset
///   from the normal's largest component when nearly parallel); with more than one point
///   and bit 2 of `m_info.m_flags` set, the average distance of the points from their centre
///   is computed into the atom's +0x14 (stopping when below `1e-6`) and the bit cleared;
///   with more than one point, a final normal-twist row (type 0x17, 0x90 bytes in all).
///
/// The first float of each row has `2^-23` added (x87) before its low byte is overwritten
/// with the type, as the exe does. Every `FCOM` test follows the exe's flag tests, NaN
/// included.
pub fn hk_simple_contact_constraint_data_build_jacobian(
    e: &mut Engine,
    atom: Ptr,
    query: Ptr,
    write_header: u8,
    schema_cursor: Ptr,
) {
    let atom = atom.addr();
    let query = query.addr();
    let cursor = schema_cursor.addr();
    // hkpSimpleContactConstraintAtom (Xbox PDB): m_numContactPoints +0x4,
    // m_numReservedContactPoints +0x6, m_contactPointPropertiesStriding +0xa; the contact
    // points (0x20 bytes each: position, then normal with the distance in w) from +0x30, the
    // properties after the reserved points.
    let count = u32::from(e.mem.u16(atom + 4));
    let inv_count_bits = x87_store32(x87_div(1.0, f64::from(count)));
    let reserved = u32::from(e.mem.u16(atom + 6));
    let striding = e.mem.u8(atom + 0xa);
    let properties = (reserved << 5).wrapping_add(atom).wrapping_add(0x30);
    let mut schema = e.mem.u32(cursor);
    // hkpConstraintQueryIn::m_bodyB / m_bodyA (Xbox PDB) +0x34 / +0x30.
    let body_b = e.mem.u32(query + 0x34);
    let body_a = e.mem.u32(query + 0x30);
    let mut position_sum: Vec4 = [0.0; 4];
    let mut normal_sum: Vec4 = [0.0; 4];
    let mut table_sum_bits = x87_store32(0.0);
    let mut impulse_sum_bits = x87_store32(0.0);
    let epsilon = x87_load64(e.mem.u64(EPSILON_010C9300));
    if write_header != 0 {
        // hkpConstraintQueryIn (Xbox PDB): m_accumulatorBIndex +0x70, m_accumulatorAIndex
        // +0x6c, m_constraintInstance +0x60.
        let index_b = e.mem.u32(query + 0x70);
        let first = x87_add(fld(e, schema), epsilon);
        let index_a = e.mem.u32(query + 0x6c);
        let instance = e.mem.u32(query + 0x60);
        e.mem.set_u16(schema.wrapping_add(4), index_a as u16);
        fstp(e, schema, first);
        e.mem.set_u16(schema.wrapping_add(6), index_b as u16);
        e.mem.set_u32(schema.wrapping_add(0xc), instance);
        e.mem.set_u8(schema, 1);
        e.mem.set_u32(schema.wrapping_add(8), properties);
        e.mem.set_u8(schema.wrapping_add(1), striding);
        schema = schema.wrapping_add(0x10);
    }
    let mut ceiling = fld(e, NEGATIVE_EPSILON_010C8440);
    let first_normal = atom + 0x40;
    let mut point = first_normal;
    // hkpContactPointProperties (Xbox PDB), addressed at its m_friction (+0xc):
    // m_impulseApplied -0xc, m_internalSolverData -0x8, m_maxImpulse +0x2, m_flags +0x3,
    // m_internalDataA +0x4.
    let mut property = properties.wrapping_add(0xc);
    let mut index: i32 = 0;
    loop {
        let paired = e.mem.u8(property.wrapping_add(3)) & 2;
        let position = load_vec(e, point.wrapping_sub(0x10));
        let normal = load_vec(e, point);
        let ra = subps(position, load_vec(e, body_a.wrapping_add(0x40)));
        let rb = subps(position, load_vec(e, body_b.wrapping_add(0x40)));
        let n_c9 = shufps(normal, normal, 0xc9);
        let n_d2 = shufps(normal, normal, 0xd2);
        let cross_a = subps(
            mulps(shufps(ra, ra, 0xc9), n_d2),
            mulps(shufps(ra, ra, 0xd2), n_c9),
        );
        let cross_b = subps(
            mulps(shufps(rb, rb, 0xd2), n_c9),
            mulps(shufps(rb, rb, 0xc9), n_d2),
        );
        let angular_a = rotate(e, body_a, cross_a);
        let angular_b = rotate(e, body_b, cross_b);
        let masses_b = inverse_masses(e, body_b);
        let masses_a = inverse_masses(e, body_a);
        store_vec(e, schema.wrapping_add(0x20), angular_b);
        let sum = mass_sum(
            mulps(mulps(angular_b, angular_b), masses_b),
            mulps(mulps(angular_a, angular_a), masses_a),
            masses_a,
            masses_b,
        );
        store_vec(e, schema, normal);
        let mass = round_trip(x87_add(x87_load32(sum.to_bits()), epsilon));
        store_vec(e, schema.wrapping_add(0x10), angular_a);
        fstp(e, schema.wrapping_add(0x2c), mass);
        // hkpConstraintQueryIn::m_virtMassFactor (Xbox PDB) +0x24.
        let inverse_mass = x87_div(m32(e, query + 0x24), mass);
        fstp(e, schema.wrapping_add(0x1c), inverse_mass);
        let friction = u32::from(e.mem.u8(property));
        let table_value = fld(e, UFLOAT8_TABLE_010C7948 + friction * 4);
        table_sum_bits = x87_store32(x87_add(table_value, x87_m32(table_sum_bits)));
        let impulse = fld(e, property.wrapping_sub(0xc));
        impulse_sum_bits = x87_store32(x87_add(impulse, x87_m32(impulse_sum_bits)));
        let internal = fld(e, property.wrapping_add(4));
        let distance = fld(e, point.wrapping_add(0xc));
        let gap = round_trip(x87_sub(distance, internal));
        let target = round_trip(x87_sub(-fld(e, property.wrapping_sub(8)), gap));
        // hkpConstraintQueryIn::m_frameDeltaTime (Xbox PDB) +0xc.
        let allowed = round_trip(x87_mul(
            fld(e, query + 0xc),
            m32(e, RECOVERY_SCALE_011B6924),
        ));
        let limit = round_trip(x87_mul(-fld(e, RECOVERY_LIMIT_011B6928), internal));
        let clamp = if fcom_status(allowed, limit) & 0x41 == 0 {
            limit
        } else {
            allowed
        };
        let mut lower = round_trip(x87_add(internal, clamp));
        let mut rest = round_trip(x87_sub(gap, clamp));
        let reach = x87_add(allowed, x87_add(clamp, clamp));
        if !parity_even(fcom_status(reach, target)) {
            lower = round_trip(x87_sub(lower, target));
            rest = round_trip(x87_add(target, rest));
        }
        if parity_even(fcom_status(lower, ceiling)) {
            lower = ceiling;
        }
        fstp(e, property.wrapping_add(4), lower);
        position_sum = addps(position_sum, position);
        // hkpConstraintQueryIn::m_rhsFactor (Xbox PDB) +0x20.
        let rhs = x87_mul(-rest, m32(e, query + 0x20));
        normal_sum = addps(normal_sum, normal);
        fstp(e, schema.wrapping_add(0xc), rhs);
        if paired == 0 {
            let maximum = u32::from(e.mem.u8(property.wrapping_add(2)));
            let row = schema;
            schema = schema.wrapping_add(0x30);
            let first = x87_add(fld(e, row), epsilon);
            fstp(e, row, first);
            e.mem.set_u8(row, 0x10);
            if maximum != 0 {
                let first = x87_add(fld(e, row), epsilon);
                fstp(e, row, first);
                e.mem.set_u8(row, 0x12);
                let limit = fld(e, UFLOAT8_TABLE_010C7948 + maximum * 4);
                fstp(e, row.wrapping_add(0x2c), limit);
            }
        } else {
            // Coupled with the previous row (0x30 bytes before this one).
            let current = load_vec(e, schema);
            let previous = load_vec(e, schema.wrapping_sub(0x30));
            let previous_mass = fld(e, schema.wrapping_sub(4));
            let previous_b = load_vec(e, schema.wrapping_sub(0x10));
            let current_mass = fld(e, schema.wrapping_add(0x2c));
            let previous_a = load_vec(e, schema.wrapping_sub(0x20));
            let masses_a = inverse_masses(e, body_a);
            let masses_b = inverse_masses(e, body_b);
            let normals = mulps(previous, current);
            let terms = addps(
                mulps(
                    mulps(load_vec(e, schema.wrapping_add(0x20)), previous_b),
                    masses_b,
                ),
                mulps(
                    mulps(load_vec(e, schema.wrapping_add(0x10)), previous_a),
                    masses_a,
                ),
            );
            let linear = addps(
                mulps(shufps(masses_a, masses_a, 0xff), normals),
                mulps(shufps(masses_b, masses_b, 0xff), normals),
            );
            let terms = addps(terms, linear);
            let k = x87_load32(sum3(terms)[0].to_bits());
            let k = round_trip(x87_mul(
                k,
                f64::from_bits(e.mem.u64(COUPLING_SCALE_0106D508)),
            ));
            let (previous_term, current_term, coupling) =
                pair_terms(k, fld(e, query + 0x24), current_mass, previous_mass);
            let row = schema;
            schema = schema.wrapping_add(0x40);
            fstp(e, row.wrapping_sub(4), previous_term);
            fstp(e, row.wrapping_add(0x2c), current_term);
            fstp(e, row.wrapping_add(0x30), coupling);
            let first = x87_add(fld(e, row.wrapping_sub(0x30)), epsilon);
            fstp(e, row.wrapping_sub(0x30), first);
            e.mem.set_u8(row.wrapping_sub(0x30), 0x14);
            ceiling = fld(e, NEGATIVE_EPSILON_010C8440);
        }
        property = property.wrapping_add(u32::from(striding));
        point = point.wrapping_add(0x20);
        index += 1;
        if index >= count as i32 {
            break;
        }
    }

    let inv_count = x87_load32(inv_count_bits);
    let product = x87_mul(x87_load32(table_sum_bits), x87_m32(impulse_sum_bits));
    let mut friction_bits = x87_store32(x87_mul(product, inv_count));
    if parity_even(fcom_status(0.0, x87_m32(friction_bits))) {
        e.mem.set_u32(cursor, schema);
        return;
    }
    let zero = [e.mem.f32(ZERO_01011D78), 0.0, 0.0, 0.0];
    let inv_count_squared = round_trip(x87_mul(inv_count, inv_count));
    let squares = mulps(normal_sum, normal_sum);
    let length_squared = x87_load32(sum3(squares)[0].to_bits());
    let point_one = x87_load64(e.mem.u64(ONE_TENTH_0101FFA0));
    let averaged = x87_mul(inv_count_squared, length_squared);
    let nearly_unit = f64::from_bits(e.mem.u64(NEARLY_UNIT_010CC5D0));
    let normal: Vec4 = if fcom_status(averaged, nearly_unit) & 0x41 == 0 {
        load_vec(e, first_normal)
    } else if !parity_even(fcom_status(point_one, length_squared)) {
        let factor = reciprocal_length(e, sum3(squares), zero);
        mulps(shufps(factor, factor, 0), normal_sum)
    } else {
        let first = load_vec(e, first_normal);
        let first_length = x87_load32(sum3(mulps(first, first))[0].to_bits());
        let nine_tenths = f64::from_bits(e.mem.u64(NINE_TENTHS_0106B9E8));
        if parity_even(fcom_status(first_length, nine_tenths)) {
            first
        } else {
            // The zero vector the exe keeps at [ESP+0x90] (stored at entry) with 1.0 in y; the
            // friction product becomes 0.
            friction_bits = x87_store32(0.0);
            [0.0, 1.0, 0.0, 0.0]
        }
    };

    let n_c9 = shufps(normal, normal, 0xc9);
    let n_d2 = shufps(normal, normal, 0xd2);
    let axis_at =
        |e: &Engine, index: u16| load_vec(e, (u32::from(index) << 4) + AXIS_TABLE_01268370);
    // hkpSimpleContactConstraintAtom::m_info.m_index (Xbox PDB) +0x12.
    let axis = axis_at(e, e.mem.u16(atom + 0x12));
    let mut tangent = subps(
        mulps(shufps(axis, axis, 0xc9), n_d2),
        mulps(shufps(axis, axis, 0xd2), n_c9),
    );
    let tangent_length = x87_load32(sum3(mulps(tangent, tangent))[0].to_bits());
    if fcom_status(tangent_length, point_one) & 0x41 != 0 {
        // The axis is nearly parallel to the normal: pick another from the normal's components.
        let magnitude = andps(normal, load_vec(e, ABS_MASK_01017D10));
        let x = x87_load32(magnitude[0].to_bits());
        let y = x87_load32(magnitude[1].to_bits());
        let z = x87_load32(magnitude[2].to_bits());
        let new_index: u16 = if fcom_status(y, x) & 0x41 != 0 {
            if fcom_status(z, y) & 0x41 == 0 {
                1
            } else {
                2
            }
        } else if fcom_status(z, x) & 0x41 != 0 {
            2
        } else {
            0
        };
        e.mem.set_u16(atom + 0x12, new_index);
        fstp(e, atom + 0x1c, 0.0);
        fstp(e, atom + 0x24, 0.0);
        let axis = axis_at(e, new_index);
        tangent = subps(
            mulps(shufps(axis, axis, 0xc9), n_d2),
            mulps(shufps(axis, axis, 0xd2), n_c9),
        );
    }
    let factor = reciprocal_length(e, sum3(mulps(tangent, tangent)), zero);
    let first_tangent = mulps(shufps(factor, factor, 0), tangent);
    let t1_c9 = shufps(first_tangent, first_tangent, 0xc9);
    let t1_d2 = shufps(first_tangent, first_tangent, 0xd2);
    let second_tangent = subps(mulps(t1_d2, n_c9), mulps(t1_c9, n_d2));
    let inv_count_register = [f32::from_bits(inv_count_bits), 0.0, 0.0, 0.0];
    let centre = mulps(
        shufps(inv_count_register, inv_count_register, 0),
        position_sum,
    );

    let row = schema;
    friction_row(
        e,
        row,
        second_tangent,
        centre,
        body_a,
        body_b,
        query,
        epsilon,
        atom + 0x1c,
    );
    friction_row(
        e,
        row.wrapping_add(0x30),
        first_tangent,
        centre,
        body_a,
        body_b,
        query,
        epsilon,
        atom + 0x24,
    );
    // The coupling of the two friction rows.
    let first_direction = load_vec(e, row);
    let second_direction = load_vec(e, row.wrapping_add(0x30));
    let first_mass = fld(e, row.wrapping_add(0x2c));
    let first_a = load_vec(e, row.wrapping_add(0x10));
    let first_b = load_vec(e, row.wrapping_add(0x20));
    let masses_a = inverse_masses(e, body_a);
    let masses_b = inverse_masses(e, body_b);
    let directions = mulps(second_direction, first_direction);
    let terms = addps(
        mulps(
            mulps(load_vec(e, row.wrapping_add(0x40)), first_a),
            masses_a,
        ),
        mulps(
            mulps(load_vec(e, row.wrapping_add(0x50)), first_b),
            masses_b,
        ),
    );
    let linear = addps(
        mulps(shufps(masses_a, masses_a, 0xff), directions),
        mulps(shufps(masses_b, masses_b, 0xff), directions),
    );
    let k = x87_load32(sum3(addps(terms, linear))[0].to_bits());
    let second_mass = fld(e, row.wrapping_add(0x5c));
    let (first_term, second_term, coupling) =
        pair_terms(k, fld(e, query + 0x24), second_mass, first_mass);
    fstp(e, row.wrapping_add(0x2c), first_term);
    fstp(e, row.wrapping_add(0x5c), second_term);
    let coupling = round_trip(coupling);
    // hkpConstraintQueryIn::m_invNumStepsTimesMicroSteps (Xbox PDB) +0x18.
    let scaled = round_trip(x87_mul(fld(e, query + 0x18), x87_m32(friction_bits)));
    fstp(e, row.wrapping_add(0x60), coupling);
    e.mem.set_u32(row.wrapping_add(0x68), atom + 0x18);
    fstp(e, row.wrapping_add(0x64), scaled);
    let first = x87_add(fld(e, row), epsilon);
    fstp(e, row, first);
    e.mem.set_u8(row, 0x16);
    if count as i32 <= 1 {
        e.mem.set_u32(cursor, row.wrapping_add(0x70));
        return;
    }

    // hkpSimpleContactConstraintAtom::m_info.m_flags (Xbox PDB) +0x10; its bit 2 asks for the
    // average distance of the points from their centre, kept in the float at +0x14.
    if e.mem.u8(atom + 0x10) & 4 != 0 {
        let minimum = fld(e, MIN_RADIUS_010959F4);
        loop {
            fstp(e, atom + 0x14, 0.0);
            let mut position = atom + 0x30;
            let mut remaining = count as i32 - 1;
            while remaining >= 0 {
                let total = fld(e, atom + 0x14);
                let offset = subps(load_vec(e, position), centre);
                let lengths = sum3(mulps(offset, offset));
                let length = sqrtss(lengths, lengths)[0];
                fstp(e, atom + 0x14, x87_add(total, x87_m32(length.to_bits())));
                position = position.wrapping_add(0x20);
                remaining -= 1;
            }
            let average = round_trip(x87_mul(fld(e, atom + 0x14), inv_count));
            fstp(e, atom + 0x14, average);
            if !parity_even(fcom_status(average, minimum)) {
                e.mem.set_u32(cursor, row.wrapping_add(0x70));
                return;
            }
            let flags = e.mem.u16(atom + 0x10) & 0xfffb;
            e.mem.set_u16(atom + 0x10, flags);
            // The exe tests the bit it has just cleared, so this never repeats.
            if e.mem.u8(atom + 0x10) & 4 == 0 {
                break;
            }
        }
    }

    // The twist row about the normal.
    let inverse_radius_bits = x87_store32(x87_div(1.0, fld(e, atom + 0x14)));
    let negated = xorps(normal, load_vec(e, SIGN_MASK_010C44D0));
    let twist = row.wrapping_add(0x60);
    let angular_a = rotate(e, body_a, normal);
    let virt_mass = fld(e, query + 0x24);
    store_vec(e, twist, angular_a);
    let angular_b = rotate(e, body_b, negated);
    let stored_a = load_vec(e, twist);
    store_vec(e, twist.wrapping_add(0x10), angular_b);
    let terms = addps(
        mulps(mulps(stored_a, stored_a), inverse_masses(e, body_a)),
        mulps(mulps(angular_b, angular_b), inverse_masses(e, body_b)),
    );
    let denominator_bits = x87_store32(x87_add(x87_load32(sum3(terms)[0].to_bits()), epsilon));
    fstp(
        e,
        twist.wrapping_add(0xc),
        x87_div(virt_mass, x87_m32(denominator_bits)),
    );
    let rhs = x87_mul(fld(e, atom + 0x2c), m32(e, query + 0x28));
    fstp(e, row.wrapping_add(0x7c), rhs);
    let scaled_mass = x87_mul(
        fld(e, twist.wrapping_add(0xc)),
        x87_m32(inverse_radius_bits),
    );
    fstp(e, twist.wrapping_add(0xc), scaled_mass);
    let radius_bits = x87_store32(fld(e, atom + 0x14));
    e.mem.set_u32(row.wrapping_add(0x8c), atom + 0x18);
    fstp(e, row.wrapping_add(0x80), coupling);
    fstp(e, row.wrapping_add(0x84), scaled);
    e.mem.set_u32(row.wrapping_add(0x88), radius_bits);
    let first = x87_add(epsilon, m32(e, row));
    fstp(e, row, first);
    e.mem.set_u8(row, 0x17);
    e.mem.set_u32(cursor, row.wrapping_add(0x90));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d7_2090,
            hk_simple_contact_constraint_data_add_a_little_surface_viscosity(Ptr)
        ),
        entry!(
            0x00d7_2190,
            hk_simple_contact_constraint_data_build_jacobian(Ptr, Ptr, u8, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_viscosity_terms_are_cleared() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x30);
        for offset in [0xc, 0x10, 0x14, 0x1c] {
            e.mem.set_f32(data + offset, 5.0);
        }
        e.call(0x00d7_2090, &args![data]);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u32(data + 0x14), 0);
        assert_eq!(e.mem.u32(data + 0x1c), 0);
        // The float between the cleared ones is left alone.
        assert_eq!(e.mem.f32(data + 0x10), 5.0);
    }

    /// The exe's constants the builder reads (values from FalloutNV.exe 1.4.0.525), and
    /// test axes in the run-time table at `01268370`.
    fn map_constants(e: &mut Engine, axes: [Vec4; 3]) {
        let doubles: [(u32, u64); 5] = [
            (EPSILON_010C9300, 0x3e80_0000_0000_0000),
            (COUPLING_SCALE_0106D508, 0x3fef_ae14_8000_0000),
            (NEARLY_UNIT_010CC5D0, 0x3fef_ff2e_4000_0000),
            (ONE_TENTH_0101FFA0, 0x3fb9_9999_a000_0000),
            (NINE_TENTHS_0106B9E8, 0x3fec_cccc_c000_0000),
        ];
        for (address, bits) in doubles {
            e.mem.map(address, 8);
            e.mem.set_u64(address, bits);
        }
        let floats: [(u32, u32); 7] = [
            (NEGATIVE_EPSILON_010C8440, 0xb400_0000),
            (RECOVERY_SCALE_011B6924, 0x3f80_0000),
            (RECOVERY_LIMIT_011B6928, 0x3d4c_cccd),
            (ZERO_01011D78, 0),
            (THREE_01017718, 0x4040_0000),
            (HALF_01016248, 0x3f00_0000),
            (MIN_RADIUS_010959F4, 0x3586_37bd),
        ];
        for (address, bits) in floats {
            e.mem.map(address, 4);
            e.mem.set_u32(address, bits);
        }
        e.mem.map(UFLOAT8_TABLE_010C7948, 16);
        for (slot, bits) in [0, 0x3c23_d70a, 0x3ca3_d70a, 0x3cf5_c28f]
            .into_iter()
            .enumerate()
        {
            e.mem
                .set_u32(UFLOAT8_TABLE_010C7948 + 4 * slot as u32, bits);
        }
        for (address, lane) in [
            (ABS_MASK_01017D10, 0x7fff_ffff),
            (SIGN_MASK_010C44D0, 0x8000_0000),
        ] {
            e.mem.map(address, 16);
            for offset in [0, 4, 8, 12] {
                e.mem.set_u32(address + offset, lane);
            }
        }
        e.mem.map(AXIS_TABLE_01268370, 0x30);
        for (slot, axis) in axes.into_iter().enumerate() {
            store_vec(e, AXIS_TABLE_01268370 + 0x10 * slot as u32, axis);
        }
    }

    const X_AXIS: Vec4 = [1.0, 0.0, 0.0, 0.0];
    const Y_AXIS: Vec4 = [0.0, 1.0, 0.0, 0.0];
    const Z_AXIS: Vec4 = [0.0, 0.0, 1.0, 0.0];

    /// A velocity accumulator at the origin: inverse masses 1, identity `m_scratch123`.
    fn body(e: &mut Engine) -> u32 {
        let body = e.mem.alloc(0x80);
        store_vec(e, body + 0x30, [1.0; 4]);
        store_vec(e, body + 0x50, X_AXIS);
        store_vec(e, body + 0x60, Y_AXIS);
        store_vec(e, body + 0x70, Z_AXIS);
        body
    }

    /// An `hkpConstraintQueryIn` over two bodies: frame delta time 0.1, the other factors 1.
    fn query(e: &mut Engine) -> u32 {
        let query = e.mem.alloc(0x80);
        let (body_a, body_b) = (body(e), body(e));
        e.mem.set_f32(query + 0xc, 0.1);
        for offset in [0x18, 0x20, 0x24, 0x28] {
            e.mem.set_f32(query + offset, 1.0);
        }
        e.mem.set_u32(query + 0x30, body_a);
        e.mem.set_u32(query + 0x34, body_b);
        e.mem.set_u32(query + 0x60, 0x1234_5678);
        e.mem.set_u32(query + 0x6c, 3);
        e.mem.set_u32(query + 0x70, 4);
        query
    }

    /// A contact atom with `points` (position, normal with distance) and their properties
    /// (striding 0x14, the properties after `points.len()` reserved points).
    fn atom(e: &mut Engine, points: &[(Vec4, Vec4)]) -> u32 {
        let atom = e.mem.alloc(0x200);
        e.mem.set_u16(atom + 4, points.len() as u16);
        e.mem.set_u16(atom + 6, points.len() as u16);
        e.mem.set_u8(atom + 0xa, 0x14);
        for (slot, (position, normal)) in points.iter().enumerate() {
            store_vec(e, atom + 0x30 + 0x20 * slot as u32, *position);
            store_vec(e, atom + 0x40 + 0x20 * slot as u32, *normal);
        }
        atom
    }

    fn property(atom: u32, count: u32, slot: u32) -> u32 {
        atom + 0x30 + count * 0x20 + 0x14 * slot
    }

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= 2.0 * f32::EPSILON * expected.abs().max(1.0)
    }

    #[test]
    fn a_point_without_friction_writes_the_header_and_one_normal_row() {
        let mut e = Engine::new();
        map_constants(&mut e, [X_AXIS, Y_AXIS, Z_AXIS]);
        let query = query(&mut e);
        let atom = atom(&mut e, &[([1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.5])]);
        let props = property(atom, 1, 0);
        e.mem.set_f32(props + 0x10, 0.25);
        let schemas = e.mem.alloc(0x100);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, schemas);
        e.call(0x00d7_2190, &args![atom, query, 1u8, cursor]);

        // Header: type 1, striding, the float bytes 2-3 of 0 + 2^-23, indices, properties,
        // instance.
        assert_eq!(e.mem.u8(schemas), 1);
        assert_eq!(e.mem.u8(schemas + 1), 0x14);
        assert_eq!(e.mem.u16(schemas + 2), 0x3400);
        assert_eq!(e.mem.u16(schemas + 4), 3);
        assert_eq!(e.mem.u16(schemas + 6), 4);
        assert_eq!(e.mem.u32(schemas + 8), atom + 0x50);
        assert_eq!(e.mem.u32(schemas + 0xc), 0x1234_5678);

        let row = schemas + 0x10;
        assert_eq!(e.mem.u32(row), 0x3400_0010);
        assert_eq!(e.mem.f32(row + 8), 1.0);
        assert_eq!(load_vec(&e, row + 0x10)[..3], [0.0, -1.0, 0.0]);
        assert_eq!(load_vec(&e, row + 0x20)[..3], [0.0, 1.0, 0.0]);
        // Mass 1 + 1 (angular) + 1 + 1 (linear), plus 2^-23 rounded away.
        assert_eq!(e.mem.f32(row + 0x2c), 4.0);
        assert_eq!(e.mem.f32(row + 0x1c), 0.25);
        // Recovery: the gap 0.5 - 0.25 less the clamped limit -0.05 * 0.25.
        let rest = 0.25f32 + f32::from_bits(0x3d4c_cccd) * 0.25;
        assert_eq!(e.mem.f32(row + 0xc), -rest);
        assert_eq!(e.mem.u32(props + 0x10), 0xb400_0000);
        // No friction: the cursor ends after the one row.
        assert_eq!(e.mem.u32(cursor), row + 0x30);
    }

    #[test]
    fn coupled_points_with_friction_write_friction_and_twist_rows() {
        let mut e = Engine::new();
        map_constants(&mut e, [X_AXIS, Y_AXIS, Z_AXIS]);
        let query = query(&mut e);
        let normal = [0.0, 0.0, 1.0, 0.5];
        let atom = atom(
            &mut e,
            &[
                ([1.0, 0.0, 0.0, 0.0], normal),
                ([-1.0, 0.0, 0.0, 0.0], normal),
            ],
        );
        e.mem.set_u16(atom + 0x10, 4);
        let (first, second) = (property(atom, 2, 0), property(atom, 2, 1));
        for props in [first, second] {
            e.mem.set_f32(props, 1.0);
            e.mem.set_u8(props + 0xc, 2);
        }
        e.mem.set_u8(first + 0xe, 1);
        e.mem.set_f32(first + 0x10, 0.25);
        e.mem.set_u8(second + 0xf, 2);
        e.mem.set_f32(second + 4, -10.0);
        let schemas = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, schemas);
        e.call(0x00d7_2190, &args![atom, query, 0u8, cursor]);

        // The first point's row, then coupled with the second.
        assert_eq!(e.mem.u8(schemas), 0x14);
        let inverse = (1.0 / (4.0 * f64::from(f32::from_bits(0x3c23_d70a)))) as f32;
        assert_eq!(e.mem.f32(schemas + 0x2c), 4.0 * inverse);
        let second_row = schemas + 0x30;
        assert_eq!(
            e.mem.f32(second_row + 0x2c),
            (f64::from(f32::from_bits(0x3c23_d70a)) * f64::from(inverse)) as f32
        );
        assert_eq!(e.mem.u32(second_row + 0x30), 0x8000_0000);
        // The second point's recovery reaches its target: -10 + ... and rhs -10.
        assert_eq!(e.mem.f32(second + 0x10), -9.5);
        assert_eq!(e.mem.f32(second_row + 0xc), -10.0);

        // Friction rows along the tangents of the first normal (axis 0 is x).
        let row = schemas + 0x70;
        assert!(near(e.mem.f32(row + 0x34), -1.0));
        assert_eq!(e.mem.f32(row + 0x1c), 0.5);
        assert_eq!(e.mem.f32(row + 0x4c), 0.5);
        // Coupled: det 2 * 2, inverse 0.25.
        assert_eq!(e.mem.f32(row + 0x2c), 0.5);
        assert_eq!(e.mem.f32(row + 0x5c), 0.5);
        let friction = f32::from_bits(0x3ca3_d70a) + f32::from_bits(0x3ca3_d70a);
        assert_eq!(e.mem.f32(row + 0x84), friction);
        assert_eq!(e.mem.u32(row + 0x80), 0x8000_0000);
        // The points' average distance from their centre, and the cleared request bit.
        assert_eq!(e.mem.f32(atom + 0x14), 1.0);
        assert_eq!(e.mem.u16(atom + 0x10), 0);
        // The twist row.
        assert_eq!(load_vec(&e, row + 0x60)[..3], [0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, row + 0x70)[..3], [0.0, 0.0, -1.0]);
        assert_eq!(e.mem.f32(row + 0x6c), 0.5);
        assert_eq!(e.mem.f32(row + 0x88), 1.0);
        assert_eq!(e.mem.u32(row + 0x8c), atom + 0x18);
        assert_eq!(e.mem.u8(row), 0x17);
        assert_eq!(e.mem.u32(cursor), row + 0x90);
    }

    #[test]
    fn the_tangent_axis_and_the_normal_fall_back() {
        // A short normal is normalized; the axis parallel to it is replaced by axis 1.
        let mut e = Engine::new();
        map_constants(&mut e, [Z_AXIS, Y_AXIS, X_AXIS]);
        let query = query(&mut e);
        let atom = atom(&mut e, &[([0.0; 4], [0.0, 0.0, 0.5, 0.5])]);
        let props = property(atom, 1, 0);
        e.mem.set_f32(props, 1.0);
        e.mem.set_u8(props + 0xc, 1);
        e.mem.set_f32(atom + 0x1c, 7.0);
        e.mem.set_f32(atom + 0x24, 7.0);
        let schemas = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, schemas);
        e.call(0x00d7_2190, &args![atom, query, 0u8, cursor]);
        let row = schemas + 0x30;
        assert_eq!(e.mem.u16(atom + 0x12), 1);
        assert_eq!(e.mem.u32(atom + 0x1c), 0);
        assert_eq!(e.mem.u32(atom + 0x24), 0);
        // The first tangent lies along x (axis 1 crossed with the normal), the second along y.
        assert!(near(e.mem.f32(row + 0x30), 1.0));
        assert!(near(e.mem.f32(row + 4), 1.0));
        assert_eq!(e.mem.u8(row), 0x16);
        assert_eq!(e.mem.u32(row + 0x68), atom + 0x18);
        assert_eq!(e.mem.f32(row + 0x64), f32::from_bits(0x3c23_d70a));
        assert_eq!(e.mem.u32(cursor), row + 0x70);

        // A normal too short to normalize: the y axis stands in and the friction term is 0.
        let mut e = Engine::new();
        map_constants(&mut e, [Z_AXIS, Y_AXIS, X_AXIS]);
        let query = super::tests::query(&mut e);
        let atom = super::tests::atom(&mut e, &[([0.0; 4], [0.0, 0.0, 0.2, 0.5])]);
        let props = property(atom, 1, 0);
        e.mem.set_f32(props, 1.0);
        e.mem.set_u8(props + 0xc, 1);
        let schemas = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, schemas);
        e.call(0x00d7_2190, &args![atom, query, 0u8, cursor]);
        let row = schemas + 0x30;
        assert_eq!(e.mem.u16(atom + 0x12), 0);
        assert!(near(e.mem.f32(row + 0x30), -1.0));
        assert!(near(e.mem.f32(row + 8), 1.0));
        assert_eq!(e.mem.u32(row + 0x64), 0);
        assert_eq!(e.mem.u32(cursor), row + 0x70);
    }
}
