//! `Havok SDK/hkpgenericconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

//!
//! A generic constraint's *scheme* is a small program: a list of instruction words (`+0x1c`
//! of the scheme, which is `this + 0x18`), the floats they read (`+0x10`) and the objects some
//! of them call (`+0x28`). `hatchScheme` runs it against a block of "registers" (the *data
//! buffer*): 16-byte vectors from `+0x20`, register `n` at `+0x20 + 16 * n`, the two pivots
//! at `+0x00` and `+0x10`, the rotations at `+0x50` and `+0x80` and the number of rows built
//! so far at `+0xb8`. The instruction routines all take six stack arguments: the address of
//! the instruction cursor, the address of the float cursor, the scheme, the data buffer, the
//! copy of the build-jacobian information and the extra argument.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpgskmanifoldutil::{
    add, cross, inverse_length, load_vec, mul, store_vec, sub, sum3,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// The class's vtable (`010d0b9c`) and the one the base destructor leaves (`0102e388`).
const VTABLE: u32 = 0x010d_0b9c;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The tables of axis indices: `TABLE_A[i]` is the dword at `011b5a90 + 4 * i` and
/// `TABLE_B[i]` the one at `011b5a94 + 4 * i`.
const TABLE_A: u32 = 0x011b_5a90;
const TABLE_B: u32 = 0x011b_5a94;
/// Floats the code reads from the exe: 0.0, 0.5, the epsilon 1.1920929e-7, the vector of
/// sign bits; the doubles pi, 2 * pi, -pi and 1.0.
const ZERO: u32 = 0x0101_1d78;
const HALF: u32 = 0x0101_6248;
const EPSILON: u32 = 0x0104_da4c;
const SIGN_MASK: u32 = 0x010c_44d0;
const PI: u32 = 0x0101_ff40;
const TWO_PI: u32 = 0x0101_ff48;
const MINUS_PI: u32 = 0x0101_ff58;
const ONE: u32 = 0x0101_2070;
/// `_ftol2`: truncates the float in `ST0` (the leading `f64` argument).
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `hkMath_atan2fApproximation` (Xbox PDB; two floats, answers the angle in `ST0`).
const ATAN2: u32 = 0x00cb_ff80;
/// `hkVector4::setTransformedPos` (Xbox PDB; `this` = the output, then `(matrix, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkMatrix3::setMul_ov2` (Xbox PDB; `this` = the output, then `(matrix, matrix)`).
const MATRIX_SET_MUL: u32 = 0x00cd_55b0;
/// `hk1dAngularBilateralConstraintBuildJacobian`, `hk1dLinearBilateralConstraintBuildJacobian`,
/// `hk1dLinearFrictionBuildJacobian`, `hk1dLinearLimitBuildJacobian` and
/// `hk1dAngularFrictionBuildJacobian` (Xbox PDB), each `(block, information, extra)`.
const ANGULAR_BILATERAL: u32 = 0x00dc_ee80;
const LINEAR_BILATERAL: u32 = 0x00dc_f280;
const LINEAR_FRICTION: u32 = 0x00dc_f100;
const LINEAR_LIMIT: u32 = 0x00dc_fa30;
const ANGULAR_FRICTION: u32 = 0x00dc_efb0;
/// Unnamed build-jacobian helpers: `00dcfec0` fills two blocks from four vectors and
/// `00d6cb30` consumes a block `(block, information, extra)`.
const FOUR_VECTOR_BUILD: u32 = 0x00dc_fec0;
const BLOCK_BUILD: u32 = 0x00d6_cb30;
/// `hk1dAngularVelocityMotorBeginJacobian`, `...CommitJacobian`, `hkCalcMotorData` and
/// `hk1dLinearVelocityMotorBeginJacobian`, `...CommitJacobian` (Xbox PDB).
const ANGULAR_MOTOR_BEGIN: u32 = 0x00d6_cd70;
const ANGULAR_MOTOR_COMMIT: u32 = 0x00d6_ce70;
const CALC_MOTOR_DATA: u32 = 0x00dc_fff0;
const LINEAR_MOTOR_BEGIN: u32 = 0x00dd_0260;
const LINEAR_MOTOR_COMMIT: u32 = 0x00dd_03c0;
/// `hkpBridgeConstraintAtom::init` (Xbox PDB; `this` = the atom, then one argument).
const BRIDGE_ATOM_INIT: u32 = 0x00d2_1ce0;
/// `00dd0420` (`this` = the member at `+0x18`, then the flag) and `00c905b0` (`this` = an
/// object): unnamed in the engine map.
const MEMBER_DESTRUCT: u32 = 0x00dd_0420;
const RELEASE_OBJECT: u32 = 0x00c9_05b0;
/// `00d6f2b0` (cdecl `(information, extra, table, 8)`): unnamed in the engine map.
const PREPARE_ROWS: u32 = 0x00d6_f2b0;

/// The six stack arguments every scheme instruction routine takes.
type SchemeStep = fn(&mut Engine, Ptr, Ptr, Ptr, Ptr, Ptr, u32);

/// `*cursor += 4`, then the word the cursor points at: the operand of the instruction.
fn next_operand(e: &mut Engine, cursor: u32) -> u32 {
    let at = e.mem.u32(cursor).wrapping_add(4);
    e.mem.set_u32(cursor, at);
    e.mem.u32(at)
}

/// The address of the 16-byte slot `index + bias` of the vector block at `base`.
fn slot(base: u32, index: u32, bias: u32) -> u32 {
    base.wrapping_add(index.wrapping_add(bias) << 4)
}

/// Register `index` of the data buffer `data`.
fn register(data: u32, index: u32) -> u32 {
    slot(data, index, 2)
}

/// The dword at `base + 4 * index`.
fn table_entry(e: &Engine, base: u32, index: u32) -> u32 {
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// A raw copy of `words` dwords (`MOVAPS` / `MOVSD`).
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

/// `FLD dword` / `FSTP dword`: a float copy that quiets a signalling NaN.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// `data[+0xb8] += by`: the number of rows built.
fn add_rows(e: &mut Engine, data: u32, by: u32) {
    let rows = e.mem.u32(data + 0xb8).wrapping_add(by);
    e.mem.set_u32(data + 0xb8, rows);
}

/// The two sums `z + (y + x)` of `vector(data, b + 8) * axis` and `vector(data, a + 8) * axis`
/// the angle routines hand to the arc tangent, where `a` and `b` are the entries of the two
/// index tables for `index` and `axis` is the vector `a + 5`. The first sum is the one of the
/// vector picked by `TABLE_B`.
fn axis_sums(e: &Engine, data: u32, index: u32) -> (f32, f32) {
    let a = table_entry(e, TABLE_A, index);
    let b = table_entry(e, TABLE_B, index);
    let axis = load_vec(e, slot(data, a, 5));
    let sum_b = sum3(mul(load_vec(e, slot(data, b, 8)), axis));
    let sum_a = sum3(mul(load_vec(e, slot(data, a, 8)), axis));
    (sum_b, sum_a)
}

fn float_to_int(e: &mut Engine, value: f32) -> u32 {
    e.call(FLOAT_TO_INT, &args![f64::from(value)]).u32()
}

// Translated from 00d516f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d5_0c50`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d516f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d5_0c50, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d503a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::getRuntimeInfo` (Xbox PDB), vtable slot: `this`, an unused
/// argument, then `info`. Stores the number of rows `this[+0x20]` at `info + 4` and eight
/// times as many at `info + 0`.
pub fn get_runtime_info(e: &mut Engine, this: Ptr, _unused: u32, info: Ptr) {
    let rows = e.mem.u32(this.addr() + 0x20);
    e.mem.set_u32(info.addr() + 4, rows);
    e.mem.set_u32(info.addr(), rows.wrapping_mul(8));
}

// Translated from 00d503c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d503c0` (no map name, unconfirmed): a scheme instruction (code 7) taking the six
/// stack arguments. Builds the three linear bilateral rows along the x, y and z axes: a
/// 0x30-byte block of the two pivots (copied from `data`) and an axis vector goes to
/// `hk1dLinearBilateralConstraintBuildJacobian` three times, then the row count grows by 3.
pub fn fn_00d503c0(
    e: &mut Engine,
    _cursor: Ptr,
    _floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    e.with_stack(0x30, |e, frame| {
        let block = frame.addr();
        copy_words(e, data, block, 8);
        // The axis vector starts cleared; one lane at a time is set to 1.0.
        e.mem.set_f32(block + 0x20, 1.0);
        e.call(LINEAR_BILATERAL, &args![block, info, extra]);
        e.mem.set_f32(block + 0x20, 0.0);
        e.mem.set_f32(block + 0x24, 1.0);
        e.call(LINEAR_BILATERAL, &args![block, info, extra]);
        e.mem.set_f32(block + 0x24, 0.0);
        e.mem.set_f32(block + 0x28, 1.0);
        e.call(LINEAR_BILATERAL, &args![block, info, extra]);
    });
    add_rows(e, data, 3);
}

// Translated from 00d50450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50450` (no map name, unconfirmed): register inputs first: `base` (ECX) and `index`
/// (ESI). Takes the axis `v` = vector `TABLE_A[index] + 5` of the 16-byte block at `base` and
/// the sums `z + (y + x)` of `vector(TABLE_B[index] + 8) * v` and `vector(TABLE_A[index] + 8)
/// * v`, and answers `hkMath_atan2fApproximation(first, second)` (the angle in `ST0`).
pub fn fn_00d50450(e: &mut Engine, base: Ptr, index: u32) -> f64 {
    let (first, second) = axis_sums(e, base.addr(), index);
    e.call(ATAN2, &args![first.to_bits(), second.to_bits()])
        .f64()
}

// Translated from 00d504f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::getScheme` (Xbox PDB): the scheme, `this + 0x18`.
pub fn get_scheme(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0x18)
}

// Translated from 00d50500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::isValid` (Xbox PDB), vtable slot: sets the byte `out` points at
/// to 1 and answers `out`.
pub fn is_valid(e: &mut Engine, _this: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00d50510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::getType` (Xbox PDB), vtable slot: answers 10.
pub fn get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    10
}

// Translated from 00d50520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::getConstraintInfo` (Xbox PDB), vtable slot: `info + 0x10` gets
/// `this + 0xc` and `info + 0x14` gets 12; the four words at `info + 0` are cleared and then
/// loaded from `this + 0x18`.
pub fn get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    let (this, info) = (this.addr(), info.addr());
    e.mem.set_u32(info + 0x10, this + 0xc);
    for word in 0..4 {
        e.mem.set_u32(info + 4 * word, 0);
    }
    e.mem.set_u32(info + 0x14, 0xc);
    copy_words(e, this + 0x18, info, 4);
}

// Translated from 00d50570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50570` (no map name, unconfirmed): `this` (ECX), then `(start, count, source)`.
/// Copies `count` 16-byte vectors from `source` into the array at `this[+0x28]`, from slot
/// `start` (nothing when `start + count` is not above `start`).
pub fn fn_00d50570(e: &mut Engine, this: Ptr, start: i32, count: i32, source: Ptr) {
    let end = start.wrapping_add(count);
    if start < end {
        let mut offset = (start as u32) << 4;
        let mut from = source.addr();
        for _ in 0..end.wrapping_sub(start) as u32 {
            let array = e.mem.u32(this.addr() + 0x28);
            copy_words(e, from, array.wrapping_add(offset), 4);
            offset = offset.wrapping_add(0x10);
            from = from.wrapping_add(0x10);
        }
    }
}

// Translated from 00d505c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::setLinearFrictionW` (Xbox PDB): scheme instruction 0x15 with the
/// six stack arguments. Reads the register number after the instruction, builds the 0x28-byte
/// block (the difference of the two pivots, register `n`, the next float of the float cursor
/// and the row's entry in the table at `info[+0x64]`), calls
/// `hk1dLinearFrictionBuildJacobian` and counts the row. The block's last two lanes are
/// never read by that function.
pub fn set_linear_friction_w(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let (data, floats) = (data.addr(), floats.addr());
    let table = e.mem.u32(info.addr() + 0x64);
    let index = next_operand(e, cursor.addr());
    e.with_stack(0x30, |e, frame| {
        let block = frame.addr();
        copy_words(e, register(data, index), block + 0x10, 4);
        let first = e.mem.u32(floats);
        let difference = sub(load_vec(e, data), load_vec(e, data + 0x10));
        store_vec(e, block, difference);
        e.mem.set_u32(floats, first.wrapping_add(0x10));
        copy_float(e, first, block + 0x20);
        let rows = e.mem.u32(data + 0xb8);
        e.mem
            .set_u32(block + 0x24, table.wrapping_add(rows.wrapping_mul(8)));
        e.call(LINEAR_FRICTION, &args![block, info, extra]);
    });
    add_rows(e, data, 1);
}

// Translated from 00d50640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::hatchScheme` (Xbox PDB): `this` (ECX; unused here), then
/// `(schema, info, extra)`. Runs the scheme: a copy of `info` (0x80 bytes) and a data buffer
/// of registers live on the frame; the instruction cursor, float cursor and object cursor
/// start at the scheme's `+0x1c`, `+0x10` and `+0x28`. The matrices `info[+0x38]` and
/// `info[+0x3c]` are the two bodies' transforms. Instructions:
///
/// - 0 ends; 1 and 2 put the float-cursor vector through matrix A / B into pivot A / B
///   (`hkVector4::setTransformedPos`) and 3 / 4 into register `n` (multiplied as rows, no
///   translation); 5 copies the vector into register `n`;
/// - 8 and 9 multiply matrix A / B by the 0x30 bytes at the float cursor into the rotation
///   at `+0x50` / `+0x80`; 10 and 11 copy matrix A / B there;
/// - 6, 7, 12-21 call the routines of this unit with the six stack arguments (cursor
///   addresses first): 6 `fn_00d50d00`, 7 `fn_00d503c0`, 12 `fn_00d50d60`, 13 `fn_00d50e10`,
///   14 `setAngularMotorW`, 15 `setLinearMotorW`, 16 `setLinearLimitW`, 17 `fn_00d51170`,
///   18 `fn_00d50ec0`, 19 `fn_00d51060`, 20 `setAngularFrictionW`, 21 `setLinearFrictionW`;
/// - 22 multiplies the copy's word 9 by the next float, 23 sets it from `info[+0x24]`;
/// - 24 calls slot 1 of the object at the object cursor with `(data buffer, operand)` and
///   moves that cursor on;
/// - any other code is skipped.
///
/// The pivots (instructions 1 and 2) are never cleared: schemes set them before the
/// routines that read them.
pub fn hatch_scheme(e: &mut Engine, _this: Ptr, schema: Ptr, info: Ptr, extra: u32) {
    let schema = schema.addr();
    e.with_stack(0x170, |e, frame| {
        let f = frame.addr();
        // The frame: float cursor f+0x10, instruction cursor f+0x14, object cursor f+0x18,
        // data buffer f+0x20 (registers from f+0x40), matrices f+0xd0 / f+0xd4, the copy of
        // info f+0xe0.
        copy_words(e, info.addr(), f + 0xe0, 0x20);
        let matrix_a = e.mem.u32(f + 0x118);
        let matrix_b = e.mem.u32(f + 0x11c);
        e.mem.set_u32(f + 0xd0, matrix_a);
        e.mem.set_u32(f + 0xd4, matrix_b);
        e.mem.set_u32(f + 0xd8, 0);
        let mut next = e.mem.u32(schema + 0x1c);
        let objects = e.mem.u32(schema + 0x28);
        let floats = e.mem.u32(schema + 0x10);
        e.mem.set_u32(f + 0x10, floats);
        e.mem.set_u32(f + 0x18, objects);
        loop {
            e.mem.set_u32(f + 0x14, next);
            let code = e.mem.u32(next);
            let cursor = e.mem.u32(f + 0x10);
            let matrix_a = e.mem.u32(f + 0xd0);
            let matrix_b = e.mem.u32(f + 0xd4);
            match code {
                0 => return,
                1 | 2 => {
                    let (matrix, output) = if code == 1 {
                        (matrix_a, f + 0x20)
                    } else {
                        (matrix_b, f + 0x30)
                    };
                    e.call(SET_TRANSFORMED_POS, &args![output, matrix, cursor]);
                    let moved = e.mem.u32(f + 0x10).wrapping_add(0x10);
                    e.mem.set_u32(f + 0x10, moved);
                }
                3 | 4 => {
                    let matrix = if code == 3 { matrix_a } else { matrix_b };
                    let v = load_vec(e, cursor);
                    let rows = [
                        load_vec(e, matrix),
                        load_vec(e, matrix.wrapping_add(0x10)),
                        load_vec(e, matrix.wrapping_add(0x20)),
                    ];
                    let moved = add(
                        add(mul([v[1]; 4], rows[1]), mul([v[0]; 4], rows[0])),
                        mul([v[2]; 4], rows[2]),
                    );
                    e.mem.set_u32(f + 0x14, next.wrapping_add(4));
                    let target = e.mem.u32(next.wrapping_add(4));
                    store_vec(e, f.wrapping_add(0x40).wrapping_add(target << 4), moved);
                    e.mem.set_u32(f + 0x10, cursor.wrapping_add(0x10));
                }
                5 => {
                    e.mem.set_u32(f + 0x14, next.wrapping_add(4));
                    let target = e.mem.u32(next.wrapping_add(4));
                    let to = f.wrapping_add(0x40).wrapping_add(target << 4);
                    copy_words(e, cursor, to, 4);
                    e.mem.set_u32(f + 0x10, cursor.wrapping_add(0x10));
                }
                8 | 9 => {
                    let (matrix, output) = if code == 8 {
                        (matrix_a, f + 0x70)
                    } else {
                        (matrix_b, f + 0xa0)
                    };
                    e.mem.set_u32(f + 0x10, cursor.wrapping_add(0x30));
                    e.call(MATRIX_SET_MUL, &args![output, matrix, cursor]);
                }
                10 | 11 => {
                    let (matrix, output) = if code == 10 {
                        (matrix_a, f + 0x70)
                    } else {
                        (matrix_b, f + 0xa0)
                    };
                    copy_words(e, matrix, output, 12);
                }
                0x16 => {
                    copy_float(e, cursor, f + 0x1c);
                    e.mem.set_u32(f + 0x10, cursor.wrapping_add(0x10));
                    let scale = f64::from(e.mem.f32(f + 0x104));
                    let by = f64::from(e.mem.f32(f + 0x1c));
                    e.mem.set_f32(f + 0x104, (scale * by) as f32);
                }
                0x17 => {
                    let value = e.mem.u32(info.addr() + 0x24);
                    e.mem.set_u32(f + 0x104, value);
                }
                0x18 => {
                    let list = e.mem.u32(f + 0x18);
                    let object = e.mem.u32(list);
                    e.mem.set_u32(f + 0x14, next.wrapping_add(4));
                    let operand = e.mem.u32(next.wrapping_add(4));
                    e.vcall(object, 4, &args![f + 0x20, operand]);
                    let moved = e.mem.u32(f + 0x18).wrapping_add(4);
                    e.mem.set_u32(f + 0x18, moved);
                }
                _ => {
                    if let Some(step) = scheme_step(code) {
                        step(
                            e,
                            Ptr::new(f + 0x14),
                            Ptr::new(f + 0x10),
                            Ptr::new(schema),
                            Ptr::new(f + 0x20),
                            Ptr::new(f + 0xe0),
                            extra,
                        );
                    }
                }
            }
            next = e.mem.u32(f + 0x14).wrapping_add(4);
        }
    })
}

/// The routine of a scheme instruction that calls into one of this unit's functions.
fn scheme_step(code: u32) -> Option<SchemeStep> {
    let step: SchemeStep = match code {
        6 => fn_00d50d00,
        7 => fn_00d503c0,
        0xc => fn_00d50d60,
        0xd => fn_00d50e10,
        0xe => set_angular_motor_w,
        0xf => set_linear_motor_w,
        0x10 => set_linear_limit_w,
        0x11 => fn_00d51170,
        0x12 => fn_00d50ec0,
        0x13 => fn_00d51060,
        0x14 => set_angular_friction_w,
        0x15 => set_linear_friction_w,
        _ => return None,
    };
    Some(step)
}

// Translated from 00d50bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::buildJacobian` (Xbox PDB), vtable slot: `this`, then `(info,
/// extra)`. Calls `00d6f2b0(info, extra, info[+0x64], 8)` (cdecl) and then runs the scheme
/// (`this + 0x18`) with `hatchScheme`.
pub fn build_jacobian(e: &mut Engine, this: Ptr, info: Ptr, extra: u32) {
    let table = e.mem.u32(info.addr() + 0x64);
    e.call(PREPARE_ROWS, &args![info, extra, table, 8u32]);
    hatch_scheme(e, this, Ptr::new(this.addr() + 0x18), info, extra);
}

// Translated from 00d50c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50c00` (no map name, unconfirmed; `this` = the object, then a flag): sets the
/// vtable, when the flag is set initialises the bridge atom at `this + 0xc` with its own
/// word `+8`, calls `00dd0420` on the member at `this + 0x18`, and when the flag is set
/// initialises the atom with `this`. Answers `this`.
pub fn fn_00d50c00(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    let atom = t + 0xc;
    if flag != 0 {
        let word = e.mem.u32(atom + 8);
        e.call(BRIDGE_ATOM_INIT, &args![atom, word]);
    }
    e.call(MEMBER_DESTRUCT, &args![t + 0x18, flag]);
    if flag != 0 {
        e.call(BRIDGE_ATOM_INIT, &args![atom, t]);
    }
    this
}

// Translated from 00d50c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50c50` (no map name, unconfirmed; the base destructor `deleting_destructor_00d516f0`
/// runs; `this` in ECX): sets the vtable, calls `00c905b0` on each of the `this[+0x50]`
/// objects of the array at `this[+0x4c]`, runs `fn_00d51620` on the member at `this + 0x18`
/// and sets the base class vtable (`0102e388`).
pub fn fn_00d50c50(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(t + 0x50) {
        let list = e.mem.u32(t + 0x4c);
        let object = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
        e.call(RELEASE_OBJECT, &args![object]);
        index += 1;
    }
    fn_00d51620(e, Ptr::new(t + 0x18));
    e.mem.set_u32(t, BASE_VTABLE);
}

// Translated from 00d50c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::hkpGenericConstraintData` (Xbox PDB; `this` in ECX): sets the
/// vtable, `[+6] = 1` and `[+0xc] = 1` (16-bit), clears `+8` and the scheme (`+0x18` to
/// `+0x24`, with `+0x1c` then set to 0x10), makes the four arrays at `+0x28`, `+0x34`,
/// `+0x40` and `+0x4c` empty (pointer, size 0, capacity 0x80000000), and initialises the
/// bridge atom at `this + 0xc` with `this`. Answers `this`.
pub fn hkp_generic_constraint_data(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u16(t + 0xc, 1);
    for array in [0x28, 0x34, 0x40, 0x4c] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, 0x8000_0000);
    }
    for word in [0x18, 0x1c, 0x20, 0x24] {
        e.mem.set_u32(t + word, 0);
    }
    e.mem.set_u32(t + 0x1c, 0x10);
    e.call(BRIDGE_ATOM_INIT, &args![t + 0xc, t]);
    this
}

// Translated from 00d50d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50d00` (no map name, unconfirmed): scheme instruction 6 with the six stack
/// arguments. Reads the register number after the instruction, builds the 0x30-byte block
/// (the two pivots and the register), calls `hk1dLinearBilateralConstraintBuildJacobian` and
/// counts the row.
pub fn fn_00d50d00(
    e: &mut Engine,
    cursor: Ptr,
    _floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    let index = next_operand(e, cursor.addr());
    e.with_stack(0x30, |e, frame| {
        let block = frame.addr();
        copy_words(e, register(data, index), block + 0x20, 4);
        copy_words(e, data, block, 8);
        e.call(LINEAR_BILATERAL, &args![block, info, extra]);
    });
    add_rows(e, data, 1);
}

// Translated from 00d50d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50d60` (no map name, unconfirmed): scheme instruction 12 with the six stack
/// arguments. With `a = TABLE_A[n]` and `b = TABLE_B[n]` for the operand `n`, builds the
/// block `[vector(b + 5), vector(a + 5), vector(n + 8)]` of the data buffer and calls
/// `hk1dAngularBilateralConstraintBuildJacobian`; then the block's first vector becomes the
/// sign-flipped second one and its second vector the first one as the call left it, and
/// the function is called again. Counts two rows.
pub fn fn_00d50d60(
    e: &mut Engine,
    cursor: Ptr,
    _floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    let index = next_operand(e, cursor.addr());
    let a = table_entry(e, TABLE_A, index);
    let b = table_entry(e, TABLE_B, index);
    e.with_stack(0x30, |e, frame| {
        let block = frame.addr();
        copy_words(e, slot(data, a, 5), block + 0x10, 4);
        copy_words(e, slot(data, b, 5), block, 4);
        copy_words(e, slot(data, index, 8), block + 0x20, 4);
        e.call(ANGULAR_BILATERAL, &args![block, info, extra]);
        for lane in 0..4 {
            let second = e.mem.u32(block + 0x10 + 4 * lane);
            let first = e.mem.u32(block + 4 * lane);
            let sign = e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(block + 4 * lane, sign ^ second);
            e.mem.set_u32(block + 0x10 + 4 * lane, first);
        }
        e.call(ANGULAR_BILATERAL, &args![block, info, extra]);
    });
    add_rows(e, data, 2);
}

// Translated from 00d50e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50e10` (no map name, unconfirmed): scheme instruction 13 with the six stack
/// arguments; reads no operand. Three calls of
/// `hk1dAngularBilateralConstraintBuildJacobian` on blocks of the rotation vectors of the
/// data buffer (`+0x50`, `+0x70`, `+0x90`; then `+0x60`, the first block's first vector as the
/// call left it, `+0xa0`; then the second block's first vector, the first block's second
/// vector and `+0x80`), each fed with the vectors the previous call left behind. Counts
/// three rows.
pub fn fn_00d50e10(
    e: &mut Engine,
    _cursor: Ptr,
    _floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    e.with_stack(0x60, |e, frame| {
        let low = frame.addr();
        let high = low + 0x30;
        copy_words(e, data + 0x50, high, 4);
        copy_words(e, data + 0x90, high + 0x20, 4);
        copy_words(e, data + 0x70, high + 0x10, 4);
        e.call(ANGULAR_BILATERAL, &args![high, info, extra]);
        copy_words(e, data + 0x60, low, 4);
        copy_words(e, data + 0xa0, low + 0x20, 4);
        copy_words(e, high, low + 0x10, 4);
        e.call(ANGULAR_BILATERAL, &args![low, info, extra]);
        copy_words(e, data + 0x80, low + 0x20, 4);
        copy_words(e, low, low + 0x10, 4);
        copy_words(e, high + 0x10, low, 4);
        e.call(ANGULAR_BILATERAL, &args![low, info, extra]);
    });
    add_rows(e, data, 3);
}

// Translated from 00d50ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d50ec0` (no map name, unconfirmed): scheme instruction 18 with the six stack
/// arguments. With the operand `n`, `a` = the vector `n + 5` and `b` = the vector `n + 8` of
/// the data buffer, takes the cross product (all four lanes); nothing more happens when the
/// squared length is below the epsilon (`0104da4c`; a NaN length goes on). Otherwise the
/// 0x20-byte block is the cross product scaled by the refined `RSQRTSS` inverse length, the
/// next two floats of the float cursor, the dot product of `a` and `b` (x87, rounded to a
/// float) and 0.5; it goes to `00d6cb30` and the row is counted.
pub fn fn_00d50ec0(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    let index = next_operand(e, cursor.addr());
    let a = load_vec(e, slot(data, index, 5));
    let b = load_vec(e, slot(data, index, 8));
    let normal = cross(a, b);
    let squared = sum3(mul(normal, normal));
    if e.mem.f32(EPSILON) > squared {
        return;
    }
    let dot = (f64::from(a[1]) * f64::from(b[1]) + f64::from(a[0]) * f64::from(b[0]))
        + f64::from(a[2]) * f64::from(b[2]);
    let scale = inverse_length(e, squared);
    e.with_stack(0x20, |e, frame| {
        let block = frame.addr();
        store_vec(e, block, mul([scale; 4], normal));
        let first = e.mem.u32(floats.addr());
        copy_float(e, first, block + 0x10);
        e.mem.set_u32(floats.addr(), first.wrapping_add(0x10));
        copy_float(e, first.wrapping_add(4), block + 0x14);
        e.mem.set_f32(block + 0x18, dot as f32);
        copy_float(e, HALF, block + 0x1c);
        e.call(BLOCK_BUILD, &args![block, info, extra]);
    });
    add_rows(e, data, 1);
}

// Translated from 00d51060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d51060` (no map name, unconfirmed): scheme instruction 19 with the six stack
/// arguments. Reads two operands `m` and `n`, calls `00dcfec0` with the data buffer's vectors
/// `m + 5`, `m + 8`, `n + 5`, `n + 8` and an output block and a float, then builds the
/// 0x20-byte block (that output, the next two floats of the float cursor, the float, 0.5)
/// for `00d6cb30` and counts the row.
pub fn fn_00d51060(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    let m = next_operand(e, cursor.addr());
    let n = next_operand(e, cursor.addr());
    e.with_stack(0x20, |e, frame| {
        let block = frame.addr();
        copy_float(e, HALF, block + 0x1c);
        e.call(
            FOUR_VECTOR_BUILD,
            &args![
                slot(data, m, 5),
                slot(data, m, 8),
                slot(data, n, 5),
                slot(data, n, 8),
                block,
                block + 0x18
            ],
        );
        let first = e.mem.u32(floats.addr());
        copy_float(e, first, block + 0x10);
        e.mem.set_u32(floats.addr(), first.wrapping_add(0x10));
        copy_float(e, first.wrapping_add(4), block + 0x14);
        e.call(BLOCK_BUILD, &args![block, info, extra]);
    });
    add_rows(e, data, 1);
}

// Translated from 00d51100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::setLinearLimitW` (Xbox PDB): scheme instruction 16 with the
/// six stack arguments. Reads the register number, builds the 0x38-byte block (the two
/// pivots, the register, the next two floats of the float cursor), calls
/// `hk1dLinearLimitBuildJacobian` and counts the row.
pub fn set_linear_limit_w(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    let index = next_operand(e, cursor.addr());
    e.with_stack(0x40, |e, frame| {
        let block = frame.addr();
        copy_words(e, register(data, index), block + 0x20, 4);
        copy_words(e, data, block, 8);
        let first = e.mem.u32(floats.addr());
        copy_float(e, first, block + 0x30);
        e.mem.set_u32(floats.addr(), first.wrapping_add(0x10));
        copy_float(e, first.wrapping_add(4), block + 0x34);
        e.call(LINEAR_LIMIT, &args![block, info, extra]);
    });
    add_rows(e, data, 1);
}

// Translated from 00d51170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d51170` (no map name, unconfirmed): scheme instruction 17 with the six stack
/// arguments. Reads the operand `n`; the 0x20-byte block is the data buffer's vector `n + 5`,
/// the next two floats of the float cursor, the angle `fn_00d50450(data, n)` (rounded to a
/// float) and 0.5. The float cursor's third float (the limit) is brought into the angle's
/// turn: with an angle below 0 and a limit above 0 more than pi apart, 2 pi is added to the
/// angle; with an angle above 0 and a limit below 0 more than pi apart, 2 pi is taken off
/// (x87 arithmetic in doubles, constants from the exe). The angle goes to the block and over
/// the limit; the block goes to `00d6cb30` and the row is counted.
pub fn fn_00d51170(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data_address = data.addr();
    let index = next_operand(e, cursor.addr());
    e.with_stack(0x20, |e, frame| {
        let block = frame.addr();
        copy_float(e, HALF, block + 0x1c);
        copy_words(e, slot(data_address, index, 5), block, 4);
        let angle = fn_00d50450(e, data, index);
        e.mem.set_f32(block + 0x18, angle as f32);
        let first = e.mem.u32(floats.addr());
        copy_float(e, first, block + 0x10);
        e.mem.set_u32(floats.addr(), first.wrapping_add(0x10));
        copy_float(e, first.wrapping_add(4), block + 0x14);
        let limit = f64::from(e.mem.f32(first.wrapping_add(8)));
        let pi = e.mem.f64(PI);
        let two_pi = e.mem.f64(TWO_PI);
        let mut turn = f64::from(e.mem.f32(block + 0x18));
        if turn < 0.0 && 0.0 < limit && pi < limit - turn {
            let moved = (turn + two_pi) as f32;
            e.mem.set_f32(block + 0x18, moved);
            turn = f64::from(moved);
        }
        if 0.0 < turn && limit < 0.0 && pi < turn - limit {
            let moved = (turn - two_pi) as f32;
            e.mem.set_f32(block + 0x18, moved);
            turn = f64::from(moved);
        }
        e.mem.set_f32(first.wrapping_add(8), turn as f32);
        e.call(BLOCK_BUILD, &args![block, info, extra]);
    });
    add_rows(e, data_address, 1);
}

// Translated from 00d51270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::setLinearMotorW` (Xbox PDB): scheme instruction 15 with the six
/// stack arguments (`schema` has the motor table: pointer at `+0x34`). Reads the register
/// number `n`, takes the motor whose number is the first float of the float cursor truncated
/// (`_ftol2`), and moves the float cursor on. With a motor: calls
/// `hk1dLinearVelocityMotorBeginJacobian`, fills the data block (the row's two words from
/// `info[+0x64]`, 0, and minus the sum `z + (y + x)` of `(pivot A - pivot B) * register n`),
/// runs `hkCalcMotorData` and `hk1dLinearVelocityMotorCommitJacobian`. The row is counted
/// either way.
pub fn set_linear_motor_w(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let index = next_operand(e, cursor.addr());
        let table = e.mem.u32(info.addr() + 0x64);
        let first = e.mem.u32(floats.addr());
        e.mem.set_u32(f + 0x14, table);
        e.mem.set_u32(floats.addr(), first.wrapping_add(0x10));
        let value = e.mem.f32(first);
        let selector = float_to_int(e, value);
        let motors = e.mem.u32(schema.addr() + 0x34);
        let motor = e.mem.u32(motors.wrapping_add(selector.wrapping_mul(4)));
        copy_words(e, register(data, index), f + 0x20, 4);
        let difference = sub(load_vec(e, data), load_vec(e, data + 0x10));
        let sum = sum3(mul(difference, load_vec(e, f + 0x20)));
        e.mem.set_f32(f + 0x18, sum);
        if motor != 0 {
            let word = e.mem.u32(extra);
            e.mem.set_u32(f + 0x38, 0);
            e.mem.set_u32(f + 0x3c, 0);
            e.mem.set_u32(f + 0x1c, word);
            e.call(
                LINEAR_MOTOR_BEGIN,
                &args![f + 0x20, data + 0x10, info, word, f + 0x30],
            );
            let rows = e.mem.u32(data + 0xb8);
            let entry = e.mem.u32(f + 0x14).wrapping_add(rows.wrapping_mul(8));
            let low = e.mem.u32(entry);
            let high = e.mem.u32(entry.wrapping_add(4));
            e.mem.set_u32(f + 0x38, low);
            e.mem.set_u32(f + 0x40, 0);
            let negated = -e.mem.f32(f + 0x18);
            e.mem.set_f32(f + 0x44, negated);
            e.mem.set_u32(f + 0x3c, high);
            e.mem.set_u32(f + 0x34, info.addr());
            e.call(CALC_MOTOR_DATA, &args![motor, f + 0x30, f + 0x48]);
            let word = e.mem.u32(f + 0x1c);
            e.call(LINEAR_MOTOR_COMMIT, &args![f + 0x48, info, word, extra]);
        }
    });
    add_rows(e, data, 1);
}

// Translated from 00d51390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::setAngularFrictionW` (Xbox PDB): scheme instruction 20 with the
/// six stack arguments. Builds the 16-byte block (the address of the data buffer's vector
/// `n + 5` for the operand `n`, the row's entry in the table at `info[+0x64]`, the next float
/// of the float cursor, 1), calls `hk1dAngularFrictionBuildJacobian` and counts the row.
pub fn set_angular_friction_w(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    _schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    let table = e.mem.u32(info.addr() + 0x64);
    let index = next_operand(e, cursor.addr());
    e.with_stack(0x10, |e, frame| {
        let block = frame.addr();
        e.mem.set_u32(block, slot(data, index, 5));
        let first = e.mem.u32(floats.addr());
        e.mem.set_u32(floats.addr(), first.wrapping_add(0x10));
        copy_float(e, first, block + 8);
        let rows = e.mem.u32(data + 0xb8);
        e.mem
            .set_u32(block + 4, table.wrapping_add(rows.wrapping_mul(8)));
        e.mem.set_u32(block + 0xc, 1);
        e.call(ANGULAR_FRICTION, &args![block, info, extra]);
    });
    add_rows(e, data, 1);
}

// Translated from 00d51410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGenericConstraintData::setAngularMotorW` (Xbox PDB): scheme instruction 14 with the six
/// stack arguments (`schema` has the motor table: pointer at `+0x34`). Reads the operand
/// `n`; the record at the float cursor is `(motor selector, previous angle, turns, _)`, the
/// cursor moving on by 0x10. The motor is the one whose number is the selector truncated
/// (`_ftol2`). The new angle is `hkMath_atan2fApproximation(-s_b, -s_a) + pi` (the sums of
/// `fn_00d50450`), rounded to a float; when it is more than pi below the previous angle (that
/// is `new - previous < -pi`) one turn is added to the turns, when more than pi above one
/// turn is taken off (x87 arithmetic in doubles, constants from the exe). The record becomes
/// `(selector, new angle, turns, 0.0)`. With a motor: `hk1dAngularVelocityMotorBeginJacobian`
/// (vector `n + 5`), the data block (the row's two words from `info[+0x64]`, 0 and minus
/// `turns * 2 pi + angle`), `hkCalcMotorData` and `hk1dAngularVelocityMotorCommitJacobian`.
/// The row is counted either way.
pub fn set_angular_motor_w(
    e: &mut Engine,
    cursor: Ptr,
    floats: Ptr,
    schema: Ptr,
    data: Ptr,
    info: Ptr,
    extra: u32,
) {
    let data = data.addr();
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let table = e.mem.u32(info.addr() + 0x64);
        let index = next_operand(e, cursor.addr());
        let record = e.mem.u32(floats.addr());
        e.mem.set_u32(floats.addr(), record.wrapping_add(0x10));
        copy_float(e, record.wrapping_add(4), f + 0x1c);
        e.mem.set_u32(f + 0x20, index);
        e.mem.set_u32(f + 0x2c, table);
        copy_float(e, record.wrapping_add(8), f + 0x18);
        let value = e.mem.f32(record);
        let selector = float_to_int(e, value);
        let motors = e.mem.u32(schema.addr() + 0x34);
        let motor = e.mem.u32(motors.wrapping_add(selector.wrapping_mul(4)));
        e.mem.set_u32(f + 0x24, motor);
        let (sum_b, sum_a) = axis_sums(e, data, index);
        e.mem.set_f32(f + 0x20, sum_a);
        let angle = e
            .call(ATAN2, &args![(-sum_b).to_bits(), (-sum_a).to_bits()])
            .f64();
        let pi = e.mem.f64(PI);
        let new_angle = (angle + pi) as f32;
        e.mem.set_f32(f + 0x20, new_angle);
        let difference = f64::from(new_angle) - f64::from(e.mem.f32(f + 0x1c));
        let one = e.mem.f64(ONE);
        let turns = f64::from(e.mem.f32(f + 0x18));
        if e.mem.f64(MINUS_PI) > difference {
            e.mem.set_f32(f + 0x18, (turns + one) as f32);
        } else if difference > pi {
            e.mem.set_f32(f + 0x18, (turns - one) as f32);
        }
        e.mem.set_f32(f + 0x1c, new_angle);
        let selector_bits = e.mem.u32(record);
        let turns_bits = e.mem.u32(f + 0x18);
        let zero_bits = e.mem.u32(ZERO);
        e.mem.set_u32(record, selector_bits);
        e.mem.set_u32(record.wrapping_add(4), new_angle.to_bits());
        e.mem.set_u32(record.wrapping_add(8), turns_bits);
        e.mem.set_u32(record.wrapping_add(0xc), zero_bits);
        if motor != 0 {
            let word = e.mem.u32(extra);
            e.mem.set_u32(f + 0x38, 0);
            e.mem.set_u32(f + 0x3c, 0);
            e.call(
                ANGULAR_MOTOR_BEGIN,
                &args![slot(data, index, 5), info, word, f + 0x30],
            );
            let rows = e.mem.u32(data + 0xb8);
            let entry = e.mem.u32(f + 0x2c).wrapping_add(rows.wrapping_mul(8));
            e.mem.set_u32(f + 0x34, info.addr());
            let low = e.mem.u32(entry);
            e.mem.set_u32(f + 0x38, low);
            let high = e.mem.u32(entry.wrapping_add(4));
            e.mem.set_u32(f + 0x40, 0);
            let two_pi = e.mem.f64(TWO_PI);
            let total =
                (f64::from(e.mem.f32(f + 0x18)) * two_pi + f64::from(e.mem.f32(f + 0x20))) as f32;
            e.mem.set_u32(f + 0x3c, high);
            e.mem.set_f32(f + 0x2c, total);
            let negated = -e.mem.f32(f + 0x2c);
            e.mem.set_f32(f + 0x44, negated);
            e.call(CALC_MOTOR_DATA, &args![motor, f + 0x30, f + 0x48]);
            e.call(ANGULAR_MOTOR_COMMIT, &args![f + 0x48, info, word, extra]);
        }
    });
    add_rows(e, data, 1);
}

// Translated from 00d51620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00d51620` (no map name, unconfirmed; `this` in ECX): empties the four arrays at
/// `+0x34`, `+0x28`, `+0x1c` (4-byte elements) and `+0x10` (16-byte elements; each laid out
/// as pointer, size at `+4`, capacity-and-flags at `+8`): the size is cleared, the block is
/// freed through the memory router's allocator (slot 4) with its byte size when bit 31 of
/// the capacity is clear, then the pointer is cleared and the capacity becomes 0x80000000.
pub fn fn_00d51620(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    for (array, shift) in [(0x34, 2), (0x28, 2), (0x1c, 2), (0x10, 4)] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(t + array + 8);
        e.mem.set_u32(t + array + 4, 0);
        if capacity & 0x8000_0000 == 0 {
            let block = e.mem.u32(t + array);
            e.vcall(
                allocator,
                0x10,
                &args![block, (capacity & 0x3fff_ffff) << shift],
            );
        }
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 8, 0x8000_0000);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_03a0, get_runtime_info(Ptr, u32, Ptr)),
        entry!(0x00d5_03c0, fn_00d503c0(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00d5_0450, fn_00d50450(Ptr, u32) -> f64),
        entry!(0x00d5_04f0, get_scheme(Ptr) -> Ptr),
        entry!(0x00d5_0500, is_valid(Ptr, Ptr) -> Ptr),
        entry!(0x00d5_0510, get_type(Ptr) -> u32),
        entry!(0x00d5_0520, get_constraint_info(Ptr, Ptr)),
        entry!(0x00d5_0570, fn_00d50570(Ptr, i32, i32, Ptr)),
        entry!(
            0x00d5_05c0,
            set_linear_friction_w(Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d5_0640, hatch_scheme(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d5_0bd0, build_jacobian(Ptr, Ptr, u32)),
        entry!(0x00d5_0c00, fn_00d50c00(Ptr, u32) -> Ptr),
        entry!(0x00d5_0c50, fn_00d50c50(Ptr)),
        entry!(0x00d5_0c90, hkp_generic_constraint_data(Ptr) -> Ptr),
        entry!(0x00d5_0d00, fn_00d50d00(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00d5_0d60, fn_00d50d60(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00d5_0e10, fn_00d50e10(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00d5_0ec0, fn_00d50ec0(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00d5_1060, fn_00d51060(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d5_1100,
            set_linear_limit_w(Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d5_1170, fn_00d51170(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d5_1270,
            set_linear_motor_w(Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d5_1390,
            set_angular_friction_w(Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d5_1410,
            set_angular_motor_w(Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d5_1620, fn_00d51620(Ptr)),
        entry!(0x00d5_16f0, deleting_destructor_00d516f0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d516f0_frees_only_when_bit_zero_is_set() {
        check(0x00d5_16f0, 0x00d5_0c50);
    }

    // ---- the scheme routines ----

    /// One logged call of a double: its address, its argument words and the dwords found at
    /// its first argument (or, for `watch_info`, the copy's word 9) when it ran.
    #[derive(Clone, Debug, PartialEq)]
    struct Call {
        address: u32,
        args: Vec<u32>,
        block: Vec<u32>,
    }

    type Log = Rc<RefCell<Vec<Call>>>;

    const FREE_DOUBLE: u32 = 0x0900_0031;
    const OBJECT_DOUBLE: u32 = 0x0900_0041;

    /// A double at `address` that logs its arguments and the `words` dwords at its first one.
    fn watch(e: &mut Engine, log: &Log, address: u32, words: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let block = (0..words).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
            log.borrow_mut().push(Call {
                address,
                args: a.to_vec(),
                block,
            });
            Ret::default()
        });
    }

    /// A double that logs its arguments and the word 9 of the information copy it is given as
    /// its second argument.
    fn watch_info(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let block = vec![e.mem.u32(a[1] + 0x24)];
            log.borrow_mut().push(Call {
                address,
                args: a.to_vec(),
                block,
            });
            Ret::default()
        });
    }

    /// A double that logs like `watch` and then writes `rewrite` over the block's start.
    fn watch_and_write(e: &mut Engine, log: &Log, address: u32, words: u32, rewrite: Vec<u32>) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let block = (0..words).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
            log.borrow_mut().push(Call {
                address,
                args: a.to_vec(),
                block,
            });
            for (n, word) in rewrite.iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * n as u32, *word);
            }
            Ret::default()
        });
    }

    fn calls_at(log: &Log, address: u32) -> Vec<Call> {
        log.borrow()
            .iter()
            .filter(|call| call.address == address)
            .cloned()
            .collect()
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|call| call.address).collect()
    }

    fn new_log() -> Log {
        Rc::default()
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    fn put_floats(e: &mut Engine, address: u32, values: &[f32]) {
        for (n, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * n as u32, *value);
        }
    }

    fn put_words(e: &mut Engine, address: u32, values: &[u32]) {
        for (n, value) in values.iter().enumerate() {
            e.mem.set_u32(address + 4 * n as u32, *value);
        }
    }

    fn read_words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|n| e.mem.u32(address + 4 * n)).collect()
    }

    /// The constants the routines read from the exe: 0.5 and 3.0, the epsilon, the sign
    /// mask, pi, 2 pi, -pi and 1.0 (as doubles; pi is the float's value like in the exe) and
    /// the axis index tables (`TABLE_A[1] = 1`, `TABLE_A[2] = 2`, so `TABLE_B[1] = 2`).
    fn setup() -> Engine {
        let mut e = engine();
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f32(0x0101_7718, 3.0);
        e.mem.set_f32(EPSILON, 1.192_092_9e-7);
        put_words(&mut e, SIGN_MASK, &[0x8000_0000; 4]);
        e.mem.set_u64(PI, 0x4009_21fb_6000_0000);
        e.mem.set_u64(TWO_PI, 0x4019_21fb_6000_0000);
        e.mem.set_u64(MINUS_PI, 0xc009_21fb_6000_0000_u64);
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_u32(TABLE_A + 4, 1);
        e.mem.set_u32(TABLE_A + 8, 2);
        e
    }

    fn pi() -> f64 {
        f64::from_bits(0x4009_21fb_6000_0000)
    }

    fn two_pi() -> f64 {
        f64::from_bits(0x4019_21fb_6000_0000)
    }

    /// The pieces of a scheme-instruction call.
    struct Rig {
        ops: u32,
        cursor: u32,
        floats: u32,
        float_values: u32,
        schema: u32,
        data: u32,
        info: u32,
        extra: u32,
        table: u32,
        motors: u32,
    }

    fn rig(e: &mut Engine, ops: &[u32], floats: &[f32]) -> Rig {
        let ops_address = e.mem.alloc(4 * ops.len() as u32 + 4);
        put_words(e, ops_address, ops);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, ops_address);
        let float_values = e.mem.alloc(0x100);
        put_floats(e, float_values, floats);
        let floats_cell = e.mem.alloc(4);
        e.mem.set_u32(floats_cell, float_values);
        let schema = e.mem.alloc(0x60);
        let motors = e.mem.alloc(0x40);
        e.mem.set_u32(schema + 0x34, motors);
        let data = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x64, table);
        let extra = e.mem.alloc(8);
        e.mem.set_u32(extra, 0x77);
        Rig {
            ops: ops_address,
            cursor,
            floats: floats_cell,
            float_values,
            schema,
            data,
            info,
            extra,
            table,
            motors,
        }
    }

    /// Runs the routine at `address` with the rig's six stack arguments.
    fn step(e: &mut Engine, address: u32, r: &Rig) {
        e.call(
            address,
            &args![r.cursor, r.floats, r.schema, r.data, r.info, r.extra],
        );
    }

    fn assert_close(got: &[u32], want: &[f32]) {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            let g = f32::from_bits(*g);
            assert!(
                (g - w).abs() <= 1e-5 * w.abs().max(1e-3),
                "{g} is not close to {w}"
            );
        }
    }

    fn router(e: &mut Engine) -> Log {
        let log = new_log();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        watch(e, &log, FREE_DOUBLE, 0);
        log
    }

    #[test]
    fn get_runtime_info_stores_the_row_count_and_eight_times_it() {
        let mut e = setup();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x20, 6);
        let info = e.mem.alloc(0x20);
        e.call(0x00d5_03a0, &args![this, 0u32, info]);
        assert_eq!(read_words(&e, info, 2), vec![48, 6]);
    }

    #[test]
    fn get_scheme_is_this_plus_0x18() {
        let mut e = setup();
        assert_eq!(e.call(0x00d5_04f0, &args![0x5000u32]).u32(), 0x5018);
    }

    #[test]
    fn is_valid_sets_the_byte_and_answers_the_pointer() {
        let mut e = setup();
        let flag = e.mem.alloc(4);
        assert_eq!(e.call(0x00d5_0500, &args![0x5000u32, flag]).u32(), flag);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(e.mem.u8(flag + 1), 0);
    }

    #[test]
    fn get_type_is_ten() {
        let mut e = setup();
        assert_eq!(e.call(0x00d5_0510, &args![0x5000u32]).u32(), 10);
    }

    #[test]
    fn get_constraint_info_describes_the_atoms() {
        let mut e = setup();
        let this = e.mem.alloc(0x60);
        put_words(&mut e, this + 0x18, &[1, 2, 3, 4]);
        let info = e.mem.alloc(0x20);
        put_words(&mut e, info, &[0xaaaa; 8]);
        e.call(0x00d5_0520, &args![this, info]);
        assert_eq!(read_words(&e, info, 6), vec![1, 2, 3, 4, this + 0xc, 0xc]);
    }

    #[test]
    fn fn_00d50570_copies_vectors_into_the_slots_from_start() {
        let mut e = setup();
        let this = e.mem.alloc(0x60);
        let array = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x28, array);
        let source = e.mem.alloc(0x40);
        put_words(&mut e, source, &(1..=8).collect::<Vec<u32>>());
        e.call(0x00d5_0570, &args![this, 1i32, 2i32, source]);
        assert_eq!(
            read_words(&e, array, 16),
            vec![0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0]
        );
        // A zero or negative count copies nothing.
        e.call(0x00d5_0570, &args![this, 5i32, 0i32, source]);
        e.call(0x00d5_0570, &args![this, 5i32, -3i32, source]);
        assert_eq!(read_words(&e, array + 0x50, 4), vec![0; 4]);
    }

    #[test]
    fn constructor_initialises_the_arrays_and_the_atom() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, BRIDGE_ATOM_INIT, 0);
        let this = e.mem.alloc(0x60);
        put_words(&mut e, this, &[0xdead_beef; 22]);
        assert_eq!(e.call(0x00d5_0c90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!((e.mem.u16(this + 6), e.mem.u16(this + 0xc)), (1, 1));
        assert_eq!(e.mem.u32(this + 8), 0);
        for array in [0x28, 0x34, 0x40, 0x4c] {
            assert_eq!(
                read_words(&e, this + array, 3),
                vec![0, 0, 0x8000_0000],
                "array {array:#x}"
            );
        }
        assert_eq!(read_words(&e, this + 0x18, 4), vec![0, 0x10, 0, 0]);
        let calls = calls_at(&log, BRIDGE_ATOM_INIT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].args, vec![this + 0xc, this]);
    }

    #[test]
    fn fn_00d50c00_initialises_the_atom_around_the_member_call_when_flagged() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, BRIDGE_ATOM_INIT, 0);
        watch(&mut e, &log, MEMBER_DESTRUCT, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x77);
        assert_eq!(e.call(0x00d5_0c00, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(addresses(&log), vec![MEMBER_DESTRUCT]);
        assert_eq!(log.borrow()[0].args, vec![this + 0x18, 0]);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00d5_0c00, &args![this, 1u32]).u32(), this);
        let seen: Vec<(u32, Vec<u32>)> = log
            .borrow()
            .iter()
            .map(|call| (call.address, call.args.clone()))
            .collect();
        assert_eq!(
            seen,
            vec![
                (BRIDGE_ATOM_INIT, vec![this + 0xc, 0x77]),
                (MEMBER_DESTRUCT, vec![this + 0x18, 1]),
                (BRIDGE_ATOM_INIT, vec![this + 0xc, this]),
            ]
        );
    }

    #[test]
    fn fn_00d51620_empties_the_four_arrays() {
        let mut e = setup();
        let frees = router(&mut e);
        let this = e.mem.alloc(0x40);
        // Owned, not owned (bit 31), owned with 4-byte elements, owned with 16-byte elements.
        put_words(&mut e, this + 0x34, &[0x1000, 5, 3]);
        put_words(&mut e, this + 0x28, &[0x2000, 4, 0x8000_0002]);
        put_words(&mut e, this + 0x1c, &[0x3000, 1, 2]);
        put_words(&mut e, this + 0x10, &[0x4000, 7, 1]);
        e.call(0x00d5_1620, &args![this]);
        let freed: Vec<Vec<u32>> = frees
            .borrow()
            .iter()
            .map(|call| call.args[1..].to_vec())
            .collect();
        assert_eq!(
            freed,
            vec![vec![0x1000, 12], vec![0x3000, 8], vec![0x4000, 16]]
        );
        for array in [0x34, 0x28, 0x1c, 0x10] {
            assert_eq!(
                read_words(&e, this + array, 3),
                vec![0, 0, 0x8000_0000],
                "array {array:#x}"
            );
        }
    }

    #[test]
    fn fn_00d50c50_releases_the_objects_and_empties_the_member() {
        let mut e = setup();
        let frees = router(&mut e);
        let log = new_log();
        watch(&mut e, &log, RELEASE_OBJECT, 0);
        let this = e.mem.alloc(0x60);
        let list = e.mem.alloc(8);
        put_words(&mut e, list, &[0xa1, 0xa2]);
        // The object list is the member's array at +0x34 (pointer, size, capacity flags),
        // which is not owned; the member's array at +0x1c is.
        put_words(&mut e, this + 0x4c, &[list, 2, 0x8000_0002]);
        put_words(&mut e, this + 0x18 + 0x1c, &[0x3000, 1, 2]);
        // The member's other two arrays are not owned either.
        e.mem.set_u32(this + 0x30, 0x8000_0000);
        e.mem.set_u32(this + 0x48, 0x8000_0000);
        e.call(0x00d5_0c50, &args![this]);
        let released: Vec<u32> = log.borrow().iter().map(|call| call.args[0]).collect();
        assert_eq!(released, vec![0xa1, 0xa2]);
        assert_eq!(frees.borrow().len(), 1);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // An empty list releases nothing.
        let empty = e.mem.alloc(0x60);
        put_words(&mut e, empty + 0x4c, &[list, 0, 0x8000_0002]);
        for cap in [0x30, 0x3c, 0x48] {
            e.mem.set_u32(empty + cap, 0x8000_0000);
        }
        log.borrow_mut().clear();
        e.call(0x00d5_0c50, &args![empty]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00d503c0_builds_the_three_axis_rows() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_BILATERAL, 12);
        let r = rig(&mut e, &[7], &[]);
        put_floats(&mut e, r.data, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u32(r.data + 0xb8, 10);
        step(&mut e, 0x00d5_03c0, &r);
        let calls = calls_at(&log, LINEAR_BILATERAL);
        assert_eq!(calls.len(), 3);
        let axes = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        for (call, axis) in calls.iter().zip(axes) {
            let mut want = bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
            want.extend(bits(&axis));
            assert_eq!(call.block, want);
            assert_eq!(call.args[1..], [r.info, r.extra]);
        }
        assert_eq!(e.mem.u32(r.data + 0xb8), 13);
    }

    #[test]
    fn fn_00d50450_passes_the_two_axis_sums_to_the_arc_tangent() {
        let mut e = setup();
        let log = new_log();
        let seen = Rc::clone(&log);
        e.register_double(ATAN2, move |_, a| {
            seen.borrow_mut().push(Call {
                address: ATAN2,
                args: a.to_vec(),
                block: Vec::new(),
            });
            Ret {
                st0: 0.75,
                ..Ret::default()
            }
        });
        let base = e.mem.alloc(0x120);
        // Index 1: a = 1, b = 2. The axis is vector 6, the others vectors 10 (b) and 9 (a).
        put_floats(&mut e, base + 0x60, &[1.0, 2.0, 3.0, 4.0]);
        put_floats(&mut e, base + 0xa0, &[5.0, 6.0, 7.0, 8.0]);
        put_floats(&mut e, base + 0x90, &[2.0, 3.0, 4.0, 5.0]);
        let got = e.call(0x00d5_0450, &args![base, 1u32]).f64();
        assert_eq!(got, 0.75);
        let calls = calls_at(&log, ATAN2);
        // 5 * 1 + 6 * 2 + 7 * 3 = 38 and 2 * 1 + 3 * 2 + 4 * 3 = 20.
        assert_eq!(calls[0].args, bits(&[38.0, 20.0]));
    }

    #[test]
    fn set_linear_friction_w_builds_the_block_and_counts_the_row() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_FRICTION, 10);
        // The float is a signalling NaN: the copy quiets it.
        let r = rig(&mut e, &[0x15, 3], &[]);
        e.mem.set_u32(r.float_values, 0x7f80_0001);
        put_floats(&mut e, r.data, &[5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0]);
        put_floats(&mut e, r.data + 0x50, &[9.0, 10.0, 11.0, 12.0]);
        e.mem.set_u32(r.data + 0xb8, 2);
        step(&mut e, 0x00d5_05c0, &r);
        let calls = calls_at(&log, LINEAR_FRICTION);
        assert_eq!(calls.len(), 1);
        let mut want = bits(&[4.0, 4.0, 4.0, 4.0, 9.0, 10.0, 11.0, 12.0]);
        want.extend([0x7fc0_0001, r.table + 16]);
        assert_eq!(calls[0].block, want);
        assert_eq!(calls[0].args[1..], [r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 3);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
        assert_eq!(e.mem.u32(r.cursor), r.ops + 4);
    }

    #[test]
    fn fn_00d50d00_builds_the_pivots_and_register_block() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_BILATERAL, 12);
        let r = rig(&mut e, &[6, 2], &[]);
        let pivots: Vec<f32> = (1..=8).map(|n| n as f32).collect();
        put_floats(&mut e, r.data, &pivots);
        put_floats(&mut e, r.data + 0x40, &[9.0, 10.0, 11.0, 12.0]);
        step(&mut e, 0x00d5_0d00, &r);
        let calls = calls_at(&log, LINEAR_BILATERAL);
        let mut want = bits(&pivots);
        want.extend(bits(&[9.0, 10.0, 11.0, 12.0]));
        assert_eq!(calls[0].block, want);
        assert_eq!(calls[0].args[1..], [r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 1);
        assert_eq!(e.mem.u32(r.cursor), r.ops + 4);
    }

    #[test]
    fn fn_00d50d60_builds_two_blocks_the_second_from_the_first_call() {
        let mut e = setup();
        let log = new_log();
        watch_and_write(
            &mut e,
            &log,
            ANGULAR_BILATERAL,
            12,
            bits(&[100.0, 101.0, 102.0, 103.0]),
        );
        let r = rig(&mut e, &[0xc, 1], &[]);
        // Index 1: a = 1, b = 2: vectors b + 5 = 7, a + 5 = 6, and 1 + 8 = 9.
        put_floats(&mut e, r.data + 0x70, &[1.0, 2.0, 3.0, 4.0]);
        put_floats(&mut e, r.data + 0x60, &[5.0, 6.0, 7.0, 8.0]);
        put_floats(&mut e, r.data + 0x90, &[9.0, 10.0, 11.0, 12.0]);
        step(&mut e, 0x00d5_0d60, &r);
        let calls = calls_at(&log, ANGULAR_BILATERAL);
        assert_eq!(calls.len(), 2);
        let mut first = bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        first.extend(bits(&[9.0, 10.0, 11.0, 12.0]));
        assert_eq!(calls[0].block, first);
        let mut second = bits(&[-5.0, -6.0, -7.0, -8.0, 100.0, 101.0, 102.0, 103.0]);
        second.extend(bits(&[9.0, 10.0, 11.0, 12.0]));
        assert_eq!(calls[1].block, second);
        assert_eq!(calls[1].args, vec![calls[0].args[0], r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 2);
    }

    #[test]
    fn fn_00d50e10_chains_three_blocks_of_rotation_vectors() {
        let mut e = setup();
        let log = new_log();
        // The first call leaves (200..) and the second (300..) in the block's first vector.
        let first_log = Rc::clone(&log);
        let mut count = 0;
        e.register_double(ANGULAR_BILATERAL, move |e, a| {
            count += 1;
            let block = (0..12).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
            first_log.borrow_mut().push(Call {
                address: ANGULAR_BILATERAL,
                args: a.to_vec(),
                block,
            });
            let base = if count == 1 { 200.0 } else { 300.0 };
            put_floats(e, a[0], &[base, base + 1.0, base + 2.0, base + 3.0]);
            Ret::default()
        });
        let r = rig(&mut e, &[0xd], &[]);
        for (offset, base) in [
            (0x50, 10.0),
            (0x60, 20.0),
            (0x70, 30.0),
            (0x80, 40.0),
            (0x90, 50.0),
            (0xa0, 60.0),
        ] {
            put_floats(
                &mut e,
                r.data + offset,
                &[base, base + 1.0, base + 2.0, base + 3.0],
            );
        }
        step(&mut e, 0x00d5_0e10, &r);
        let v = |base: f32| bits(&[base, base + 1.0, base + 2.0, base + 3.0]);
        let expected = [
            [v(10.0), v(30.0), v(50.0)],
            [v(20.0), v(200.0), v(60.0)],
            [v(30.0), v(300.0), v(40.0)],
        ];
        let calls = calls_at(&log, ANGULAR_BILATERAL);
        assert_eq!(calls.len(), 3);
        for (call, want) in calls.iter().zip(expected) {
            assert_eq!(call.block, want.concat());
            assert_eq!(call.args[1..], [r.info, r.extra]);
        }
        assert_eq!(e.mem.u32(r.data + 0xb8), 3);
    }

    #[test]
    fn fn_00d50ec0_scales_the_cross_product_and_adds_the_dot() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, BLOCK_BUILD, 8);
        let r = rig(&mut e, &[0x12, 0], &[0.25, 0.75]);
        put_floats(&mut e, r.data + 0x50, &[1.0, 2.0, 3.0, 0.0]);
        put_floats(&mut e, r.data + 0x80, &[4.0, 5.0, 6.0, 0.0]);
        step(&mut e, 0x00d5_0ec0, &r);
        let calls = calls_at(&log, BLOCK_BUILD);
        assert_eq!(calls.len(), 1);
        // cross = (-3, 6, -3), squared length 54: the inverse length is estimated.
        let scale = 1.0 / 54.0_f32.sqrt();
        assert_close(
            &calls[0].block[..3],
            &[-3.0 * scale, 6.0 * scale, -3.0 * scale],
        );
        assert_eq!(calls[0].block[3], 0);
        assert_eq!(calls[0].block[4..], bits(&[0.25, 0.75, 32.0, 0.5])[..]);
        assert_eq!(calls[0].args[1..], [r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 1);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
    }

    #[test]
    fn fn_00d50ec0_does_nothing_for_parallel_vectors() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, BLOCK_BUILD, 8);
        let r = rig(&mut e, &[0x12, 0], &[0.25, 0.75]);
        put_floats(&mut e, r.data + 0x50, &[1.0, 2.0, 3.0, 0.0]);
        put_floats(&mut e, r.data + 0x80, &[2.0, 4.0, 6.0, 0.0]);
        step(&mut e, 0x00d5_0ec0, &r);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(r.data + 0xb8), 0);
        assert_eq!(e.mem.u32(r.floats), r.float_values);
    }

    #[test]
    fn fn_00d51060_hands_the_four_vectors_to_the_helper() {
        let mut e = setup();
        let log = new_log();
        // The helper fills its output block and its float.
        let seen = Rc::clone(&log);
        e.register_double(FOUR_VECTOR_BUILD, move |e, a| {
            seen.borrow_mut().push(Call {
                address: FOUR_VECTOR_BUILD,
                args: a.to_vec(),
                block: Vec::new(),
            });
            put_floats(e, a[4], &[7.0, 8.0, 9.0, 10.0]);
            e.mem.set_f32(a[5], 11.0);
            Ret::default()
        });
        watch(&mut e, &log, BLOCK_BUILD, 8);
        let r = rig(&mut e, &[0x13, 1, 2], &[0.25, 0.75]);
        step(&mut e, 0x00d5_1060, &r);
        let four = calls_at(&log, FOUR_VECTOR_BUILD);
        let d = r.data;
        assert_eq!(four[0].args[..4], [d + 0x60, d + 0x90, d + 0x70, d + 0xa0]);
        assert_eq!(four[0].args[5], four[0].args[4] + 0x18);
        let built = calls_at(&log, BLOCK_BUILD);
        assert_eq!(
            built[0].block,
            bits(&[7.0, 8.0, 9.0, 10.0, 0.25, 0.75, 11.0, 0.5])
        );
        assert_eq!(built[0].args, vec![four[0].args[4], r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 1);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
        assert_eq!(e.mem.u32(r.cursor), r.ops + 8);
    }

    #[test]
    fn set_linear_limit_w_builds_the_block_with_two_floats() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_LIMIT, 14);
        let r = rig(&mut e, &[0x10, 1], &[0.5, 0.25, 7.0, 8.0]);
        let pivots: Vec<f32> = (1..=8).map(|n| n as f32).collect();
        put_floats(&mut e, r.data, &pivots);
        put_floats(&mut e, r.data + 0x30, &[21.0, 22.0, 23.0, 24.0]);
        step(&mut e, 0x00d5_1100, &r);
        let calls = calls_at(&log, LINEAR_LIMIT);
        let mut want = bits(&pivots);
        want.extend(bits(&[21.0, 22.0, 23.0, 24.0, 0.5, 0.25]));
        assert_eq!(calls[0].block, want);
        assert_eq!(calls[0].args[1..], [r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 1);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
    }

    /// Runs `fn_00d51170` with the arc tangent answering `angle` and `limit` as the third
    /// float; answers the block it built and the limit left in the float record.
    fn limit_turn(angle: f32, limit: f32) -> (Vec<u32>, f32) {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, BLOCK_BUILD, 8);
        e.register_double(ATAN2, move |_, _| Ret {
            st0: f64::from(angle),
            ..Ret::default()
        });
        let r = rig(&mut e, &[0x11, 0], &[0.25, 0.75, limit, 0.0]);
        put_floats(&mut e, r.data + 0x50, &[1.0, 2.0, 3.0, 4.0]);
        step(&mut e, 0x00d5_1170, &r);
        assert_eq!(e.mem.u32(r.data + 0xb8), 1);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
        let block = calls_at(&log, BLOCK_BUILD)[0].block.clone();
        (block, e.mem.f32(r.float_values + 8))
    }

    #[test]
    fn fn_00d51170_keeps_an_angle_that_is_near_the_limit() {
        let (block, limit) = limit_turn(0.5, 0.3);
        assert_eq!(block, bits(&[1.0, 2.0, 3.0, 4.0, 0.25, 0.75, 0.5, 0.5]));
        assert_eq!(limit, 0.5);
        let (block, limit) = limit_turn(-1.0, 1.0);
        assert_eq!(block[6], (-1.0f32).to_bits());
        assert_eq!(limit, -1.0);
    }

    #[test]
    fn fn_00d51170_turns_an_angle_that_is_far_from_the_limit() {
        let (block, limit) = limit_turn(-3.0, 3.0);
        let want = (-3.0f64 + two_pi()) as f32;
        assert_eq!(block[6], want.to_bits());
        assert_eq!(limit, want);
        let (block, limit) = limit_turn(3.0, -3.0);
        let want = (3.0f64 - two_pi()) as f32;
        assert_eq!(block[6], want.to_bits());
        assert_eq!(limit, want);
    }

    /// A rig for the motor routines with `motor` as motor 1 and the float record
    /// `(1.0, previous, turns, 9.9)`, after setting the double of `ftol` to answer 1.
    fn motor_rig(e: &mut Engine, ops: &[u32], motor: u32, previous: f32, turns: f32) -> Rig {
        e.register_double(FLOAT_TO_INT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let r = rig(e, ops, &[1.0, previous, turns, 9.9]);
        e.mem.set_u32(r.motors + 4, motor);
        // Two rows built so far; the row table holds two words per row.
        e.mem.set_u32(r.data + 0xb8, 1);
        put_words(e, r.table + 8, &[0xaaaa, 0xbbbb]);
        r
    }

    #[test]
    fn set_linear_motor_w_runs_the_motor_chain() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_MOTOR_COMMIT, 0);
        let calc_log = Rc::clone(&log);
        e.register_double(CALC_MOTOR_DATA, move |e, a| {
            let block = (0..6).map(|n| e.mem.u32(a[1] + 4 * n)).collect();
            calc_log.borrow_mut().push(Call {
                address: CALC_MOTOR_DATA,
                args: a.to_vec(),
                block,
            });
            Ret::default()
        });
        e.register_double(LINEAR_MOTOR_BEGIN, {
            let log = Rc::clone(&log);
            move |e, a| {
                let block = (0..4).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
                log.borrow_mut().push(Call {
                    address: LINEAR_MOTOR_BEGIN,
                    args: a.to_vec(),
                    block,
                });
                e.mem.set_u32(a[4], 0x1234);
                Ret::default()
            }
        });
        let r = motor_rig(&mut e, &[0xf, 2], 0x5000, 0.0, 0.0);
        put_floats(&mut e, r.data, &[4.0, 6.0, 8.0, 1.0, 1.0, 2.0, 3.0, 1.0]);
        // Register 2.
        put_floats(&mut e, r.data + 0x40, &[1.0, 2.0, 3.0, 4.0]);
        step(&mut e, 0x00d5_1270, &r);
        let begin = calls_at(&log, LINEAR_MOTOR_BEGIN);
        assert_eq!(begin[0].block, bits(&[1.0, 2.0, 3.0, 4.0]));
        assert_eq!(begin[0].args[1..4], [r.data + 0x10, r.info, 0x77]);
        let calc = calls_at(&log, CALC_MOTOR_DATA);
        // (4, 6, 8, 1) - (1, 2, 3, 1) = (3, 4, 5, 0): 3 + 8 + 15 = 26.
        let mut want = vec![0x1234, r.info, 0xaaaa, 0xbbbb, 0];
        want.push((-26.0f32).to_bits());
        assert_eq!(calc[0].block, want);
        assert_eq!(calc[0].args[0], 0x5000);
        assert_eq!(calc[0].args[2], calc[0].args[1] + 0x18);
        let commit = calls_at(&log, LINEAR_MOTOR_COMMIT);
        assert_eq!(commit[0].args[0], calc[0].args[2]);
        assert_eq!(commit[0].args[1..], [r.info, 0x77, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 2);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
    }

    #[test]
    fn set_linear_motor_w_without_a_motor_only_counts_the_row() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_MOTOR_BEGIN, 0);
        let ftol = Rc::clone(&log);
        let r = motor_rig(&mut e, &[0xf, 2], 0, 0.0, 0.0);
        e.register_double(FLOAT_TO_INT, move |_, a| {
            ftol.borrow_mut().push(Call {
                address: FLOAT_TO_INT,
                args: a.to_vec(),
                block: Vec::new(),
            });
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        step(&mut e, 0x00d5_1270, &r);
        let calls = calls_at(&log, FLOAT_TO_INT);
        let value = f64::from_bits(u64::from(calls[0].args[0]) | u64::from(calls[0].args[1]) << 32);
        assert_eq!(value, 1.0);
        assert_eq!(addresses(&log), vec![FLOAT_TO_INT]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 2);
    }

    #[test]
    fn set_angular_friction_w_passes_the_vector_address_and_row_entry() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, ANGULAR_FRICTION, 4);
        let r = rig(&mut e, &[0x14, 2], &[0.75]);
        e.mem.set_u32(r.data + 0xb8, 3);
        step(&mut e, 0x00d5_1390, &r);
        let calls = calls_at(&log, ANGULAR_FRICTION);
        assert_eq!(
            calls[0].block,
            vec![r.data + 0x70, r.table + 24, 0.75f32.to_bits(), 1]
        );
        assert_eq!(calls[0].args[1..], [r.info, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 4);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
    }

    /// Runs `set_angular_motor_w` with the arc tangent answering 0.5 for the vectors set up
    /// below; answers the log, the rig and the float record after the call.
    fn angular_motor(motor: u32, previous: f32, turns: f32) -> (Engine, Log, Rig) {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, ANGULAR_MOTOR_BEGIN, 0);
        watch(&mut e, &log, ANGULAR_MOTOR_COMMIT, 0);
        let calc = Rc::clone(&log);
        e.register_double(CALC_MOTOR_DATA, move |e, a| {
            let block = (0..6).map(|n| e.mem.u32(a[1] + 4 * n)).collect();
            calc.borrow_mut().push(Call {
                address: CALC_MOTOR_DATA,
                args: a.to_vec(),
                block,
            });
            Ret::default()
        });
        let atan = Rc::clone(&log);
        e.register_double(ATAN2, move |_, a| {
            atan.borrow_mut().push(Call {
                address: ATAN2,
                args: a.to_vec(),
                block: Vec::new(),
            });
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        let r = motor_rig(&mut e, &[0xe, 1], motor, previous, turns);
        // Index 1: a = 1, b = 2. Axis (vector 6), vector 10 (b + 8) and vector 9 (a + 8).
        put_floats(&mut e, r.data + 0x60, &[1.0, 2.0, 0.0, 0.0]);
        put_floats(&mut e, r.data + 0xa0, &[3.0, 4.0, 0.0, 0.0]);
        put_floats(&mut e, r.data + 0x90, &[5.0, 6.0, 0.0, 0.0]);
        step(&mut e, 0x00d5_1410, &r);
        (e, log, r)
    }

    #[test]
    fn set_angular_motor_w_keeps_the_turns_for_a_near_angle() {
        let (e, log, r) = angular_motor(0, 3.5, 2.0);
        let atan = calls_at(&log, ATAN2);
        // 3 * 1 + 4 * 2 = 11 and 5 * 1 + 6 * 2 = 17.
        assert_eq!(atan[0].args, bits(&[-11.0, -17.0]));
        let angle = (0.5f64 + pi()) as f32;
        let record = read_words(&e, r.float_values, 4);
        assert_eq!(
            record,
            vec![1.0f32.to_bits(), angle.to_bits(), 2.0f32.to_bits(), 0]
        );
        assert_eq!(addresses(&log), vec![ATAN2]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 2);
        assert_eq!(e.mem.u32(r.floats), r.float_values + 0x10);
    }

    #[test]
    fn set_angular_motor_w_adds_and_removes_turns() {
        // The new angle is 3.6415927: 7.0 is 3.36 above it, 0.1 is 3.54 below it.
        let (e, _, r) = angular_motor(0, 7.0, 2.0);
        assert_eq!(e.mem.f32(r.float_values + 8), 3.0);
        let (e, _, r) = angular_motor(0, 0.1, 2.0);
        assert_eq!(e.mem.f32(r.float_values + 8), 1.0);
    }

    #[test]
    fn set_angular_motor_w_runs_the_motor_chain() {
        let (e, log, r) = angular_motor(0x6000, 0.1, 2.0);
        let angle = (0.5f64 + pi()) as f32;
        let begin = calls_at(&log, ANGULAR_MOTOR_BEGIN);
        assert_eq!(begin[0].args[..3], [r.data + 0x60, r.info, 0x77]);
        let calc = calls_at(&log, CALC_MOTOR_DATA);
        // The turns went 2 -> 1; the data is -(1 * 2 pi + angle).
        let total = (1.0f64 * two_pi() + f64::from(angle)) as f32;
        assert_eq!(calc[0].args[0], 0x6000);
        let want = vec![r.info, 0xaaaa, 0xbbbb, 0, (-total).to_bits()];
        assert_eq!(calc[0].block[1..], want[..]);
        let commit = calls_at(&log, ANGULAR_MOTOR_COMMIT);
        assert_eq!(commit[0].args[0], calc[0].args[2]);
        assert_eq!(commit[0].args[1..], [r.info, 0x77, r.extra]);
        assert_eq!(e.mem.u32(r.data + 0xb8), 2);
    }

    // ---- hatchScheme ----

    /// A scheme for `hatch_scheme`: instruction words, floats, objects, two matrices of
    /// three vectors (A = the first, B = the second) in the information.
    struct Scheme {
        schema: u32,
        info: u32,
        float_values: u32,
        matrix_a: u32,
        matrix_b: u32,
    }

    fn scheme(e: &mut Engine, ops: &[u32], floats: &[f32], objects: &[u32]) -> Scheme {
        let ops_address = e.mem.alloc(4 * ops.len() as u32 + 4);
        put_words(e, ops_address, ops);
        let float_values = e.mem.alloc(0x200);
        put_floats(e, float_values, floats);
        let object_list = e.mem.alloc(4 * objects.len() as u32 + 4);
        put_words(e, object_list, objects);
        let schema = e.mem.alloc(0x60);
        e.mem.set_u32(schema + 0x10, float_values);
        e.mem.set_u32(schema + 0x1c, ops_address);
        e.mem.set_u32(schema + 0x28, object_list);
        let motors = e.mem.alloc(0x40);
        e.mem.set_u32(schema + 0x34, motors);
        let info = e.mem.alloc(0x100);
        let matrix_a = e.mem.alloc(0x30);
        let matrix_b = e.mem.alloc(0x30);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x38, matrix_a);
        e.mem.set_u32(info + 0x3c, matrix_b);
        e.mem.set_u32(info + 0x64, table);
        Scheme {
            schema,
            info,
            float_values,
            matrix_a,
            matrix_b,
        }
    }

    fn hatch(e: &mut Engine, s: &Scheme, extra: u32) {
        e.call(0x00d5_0640, &args![0x5000u32, s.schema, s.info, extra]);
    }

    #[test]
    fn hatch_scheme_stops_at_zero_and_skips_unknown_codes() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_BILATERAL, 0);
        let s = scheme(&mut e, &[0x19, 0xffff_ffff, 0, 6, 0], &[], &[]);
        hatch(&mut e, &s, 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn hatch_scheme_transforms_the_pivots() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, SET_TRANSFORMED_POS, 0);
        let s = scheme(&mut e, &[1, 2, 0], &[], &[]);
        hatch(&mut e, &s, 0);
        let calls = calls_at(&log, SET_TRANSFORMED_POS);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].args[1..], [s.matrix_a, s.float_values]);
        assert_eq!(calls[1].args[1..], [s.matrix_b, s.float_values + 0x10]);
        assert_eq!(calls[1].args[0], calls[0].args[0] + 0x10);
    }

    #[test]
    fn hatch_scheme_fills_registers_by_matrix_and_copy() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, LINEAR_BILATERAL, 12);
        let v0 = [1.0, 2.0, 3.0, 9.0];
        let v1 = [3.0, 5.0, 7.0, 11.0];
        let v2 = [13.0, 14.0, 15.0, 16.0];
        let mut floats = Vec::new();
        floats.extend(v0);
        floats.extend(v1);
        floats.extend(v2);
        // Register 1 = v0 through A, register 2 = v1 through B, register 3 = v2; each is
        // then built into a row with instruction 6.
        let s = scheme(
            &mut e,
            &[3, 1, 4, 2, 5, 3, 6, 1, 6, 2, 6, 3, 0],
            &floats,
            &[],
        );
        put_floats(
            &mut e,
            s.matrix_a,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        );
        put_floats(
            &mut e,
            s.matrix_b,
            &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        );
        hatch(&mut e, &s, 0);
        let calls = calls_at(&log, LINEAR_BILATERAL);
        assert_eq!(calls.len(), 3);
        // (y * row1 + x * row0) + z * row2 per lane, x = 1, y = 2, z = 3.
        assert_eq!(calls[0].block[8..], bits(&[38.0, 44.0, 50.0, 56.0])[..]);
        // B is the identity, so v1 keeps its x, y, z and loses its w.
        assert_eq!(calls[1].block[8..], bits(&[3.0, 5.0, 7.0, 0.0])[..]);
        assert_eq!(calls[2].block[8..], bits(&v2)[..]);
    }

    #[test]
    fn hatch_scheme_multiplies_matrices_and_feeds_the_rotations_to_instruction_13() {
        let mut e = setup();
        let log = new_log();
        let rewrite = Rc::clone(&log);
        e.register_double(MATRIX_SET_MUL, move |e, a| {
            rewrite.borrow_mut().push(Call {
                address: MATRIX_SET_MUL,
                args: a.to_vec(),
                block: Vec::new(),
            });
            // A product is 100.., a B product 200...
            let base = if rewrite.borrow().len() == 1 {
                100.0
            } else {
                200.0
            };
            for n in 0..12 {
                e.mem.set_f32(a[0] + 4 * n, base + n as f32);
            }
            Ret::default()
        });
        watch(&mut e, &log, ANGULAR_BILATERAL, 12);
        let s = scheme(&mut e, &[8, 9, 0xd, 0], &[0.0; 24], &[]);
        hatch(&mut e, &s, 0);
        let muls = calls_at(&log, MATRIX_SET_MUL);
        assert_eq!(muls[0].args[1..], [s.matrix_a, s.float_values]);
        assert_eq!(muls[1].args[1..], [s.matrix_b, s.float_values + 0x30]);
        let v = |base: f32| bits(&[base, base + 1.0, base + 2.0, base + 3.0]);
        let rows = calls_at(&log, ANGULAR_BILATERAL);
        // A rows: 100, 104, 108; B rows: 200, 204, 208.
        assert_eq!(rows[0].block, [v(100.0), v(108.0), v(204.0)].concat());
        assert_eq!(rows[1].block[..4], v(104.0)[..]);
        assert_eq!(rows[1].block[8..], v(208.0)[..]);
        assert_eq!(rows[2].block[..4], v(108.0)[..]);
        assert_eq!(rows[2].block[8..], v(200.0)[..]);
    }

    #[test]
    fn hatch_scheme_copies_the_matrices_for_instruction_13() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, ANGULAR_BILATERAL, 12);
        let s = scheme(&mut e, &[10, 11, 0xd, 0], &[], &[]);
        let a: Vec<f32> = (0..12).map(|n| 10.0 + n as f32).collect();
        let b: Vec<f32> = (0..12).map(|n| 30.0 + n as f32).collect();
        put_floats(&mut e, s.matrix_a, &a);
        put_floats(&mut e, s.matrix_b, &b);
        hatch(&mut e, &s, 0);
        let rows = calls_at(&log, ANGULAR_BILATERAL);
        // First block: A row 0, A row 2, B row 1.
        assert_eq!(
            rows[0].block,
            [bits(&a[0..4]), bits(&a[8..12]), bits(&b[4..8])].concat()
        );
    }

    #[test]
    fn hatch_scheme_scales_and_resets_the_copy_word() {
        let mut e = setup();
        let log = new_log();
        watch_info(&mut e, &log, LINEAR_BILATERAL);
        let ops = [0x16, 6, 0, 0x16, 6, 0, 0x17, 6, 0, 0];
        let s = scheme(&mut e, &ops, &[4.0, 0.0, 0.0, 0.0, 0.5], &[]);
        e.mem.set_f32(s.info + 0x24, 2.0);
        hatch(&mut e, &s, 0);
        let words: Vec<u32> = calls_at(&log, LINEAR_BILATERAL)
            .iter()
            .map(|call| call.block[0])
            .collect();
        // 2 * 4 = 8; the float cursor moves by 0x10 per multiplication, so the next factor
        // is 0.5 (8 * 0.5 = 4); instruction 23 brings back the 2.0 of the information.
        assert_eq!(words, bits(&[8.0, 4.0, 2.0]));
    }

    #[test]
    fn hatch_scheme_calls_the_objects_in_turn() {
        let mut e = setup();
        let log = new_log();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, OBJECT_DOUBLE);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, vtable);
        e.mem.set_u32(second, vtable);
        watch(&mut e, &log, OBJECT_DOUBLE, 0);
        let s = scheme(&mut e, &[0x18, 7, 0x18, 9, 0], &[], &[first, second]);
        hatch(&mut e, &s, 0);
        let calls = calls_at(&log, OBJECT_DOUBLE);
        assert_eq!(calls.len(), 2);
        assert_eq!((calls[0].args[0], calls[0].args[2]), (first, 7));
        assert_eq!((calls[1].args[0], calls[1].args[2]), (second, 9));
        assert_eq!(calls[0].args[1], calls[1].args[1]);
    }

    #[test]
    fn hatch_scheme_dispatches_every_instruction_routine() {
        let mut e = setup();
        let log = new_log();
        for address in [
            LINEAR_BILATERAL,
            ANGULAR_BILATERAL,
            LINEAR_LIMIT,
            BLOCK_BUILD,
            FOUR_VECTOR_BUILD,
            ANGULAR_FRICTION,
            LINEAR_FRICTION,
        ] {
            watch(&mut e, &log, address, 0);
        }
        for address in [FLOAT_TO_INT, ATAN2] {
            let seen = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                seen.borrow_mut().push(Call {
                    address,
                    args: a.to_vec(),
                    block: Vec::new(),
                });
                Ret::default()
            });
        }
        let ops = [
            10, 11, 6, 0, 7, 0xc, 0, 0xd, 0xe, 0, 0xf, 0, 0x10, 0, 0x11, 0, 0x12, 0, 0x13, 0, 0,
            0x14, 0, 0x15, 0, 0,
        ];
        let s = scheme(&mut e, &ops, &[0.0; 40], &[]);
        put_floats(
            &mut e,
            s.matrix_a,
            &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        );
        put_floats(
            &mut e,
            s.matrix_b,
            &[0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        );
        hatch(&mut e, &s, 0);
        let want = vec![
            LINEAR_BILATERAL,
            LINEAR_BILATERAL,
            LINEAR_BILATERAL,
            LINEAR_BILATERAL,
            ANGULAR_BILATERAL,
            ANGULAR_BILATERAL,
            ANGULAR_BILATERAL,
            ANGULAR_BILATERAL,
            ANGULAR_BILATERAL,
            FLOAT_TO_INT,
            ATAN2,
            FLOAT_TO_INT,
            LINEAR_LIMIT,
            ATAN2,
            BLOCK_BUILD,
            BLOCK_BUILD,
            FOUR_VECTOR_BUILD,
            BLOCK_BUILD,
            ANGULAR_FRICTION,
            LINEAR_FRICTION,
        ];
        assert_eq!(addresses(&log), want);
    }

    #[test]
    fn build_jacobian_prepares_the_rows_and_runs_the_scheme() {
        let mut e = setup();
        let log = new_log();
        watch(&mut e, &log, PREPARE_ROWS, 0);
        let this = e.mem.alloc(0x60);
        let ops = e.mem.alloc(8);
        e.mem.set_u32(this + 0x18 + 0x1c, ops);
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x64, 0x9100);
        e.call(0x00d5_0bd0, &args![this, info, 0x42u32]);
        let calls = calls_at(&log, PREPARE_ROWS);
        assert_eq!(calls[0].args, vec![info, 0x42, 0x9100, 8]);
    }
}
