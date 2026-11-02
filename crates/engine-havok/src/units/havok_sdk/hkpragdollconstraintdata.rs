//! `Havok SDK/hkpragdollconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpprimitivedrawer::{normalised, normalised_sse, rotate_by_quaternion};
use super::hkpprismaticconstraintdata::{columns, combine_columns, cross_yzx, cross_yzx_sse};
use super::hkpsphereshape::{copy_vec, load_vec, store_vec};
use std::cmp::Ordering;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cdf530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cd_f1f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cdf530(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cd_f1f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::addReference` (`00c90510`, `this` = the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl `(atoms, size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkVector4::setTransformedInversePos` (`00c7f690`, `this` = the result, `(transform, vector)`,
/// `RET 8`).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// `hkRotation::isOrthonormal` (`00cb2e70`, `this` = the rotation, `(result byte, epsilon)`,
/// `RET 8`); it returns the pointer to the result byte.
const IS_ORTHONORMAL: u32 = 0x00cb_2e70;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the quaternion, `(axis, angle)`,
/// `RET 8`).
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// `hkMatrix3::setMul` (`00cd55b0`, `this` = the result, `(first, second)`, `RET 8`).
const MATRIX_SET_MUL: u32 = 0x00cd_55b0;
/// `hkUFloat8::operator=` (`00ca9360`, `this` = the value, a pointer to the float, `RET 4`).
const UFLOAT8_ASSIGN: u32 = 0x00ca_9360;
/// The `float` tolerance `isValid` passes to `isOrthonormal` (`01017d00`).
const ORTHONORMAL_EPSILON: u32 = 0x0101_7d00;
/// The `double` `0.5`.
const HALF_F64: u32 = 0x0101_1588;
/// The `double` the float at `+0x118` must equal for `isValid` (`01024108`, `-100.0`).
const EXPECTED_AT_118: u32 = 0x0102_4108;
/// The `double` the float at `+0x11c` must stay below for `isValid` (`0101ff40`, pi).
const LIMIT_AT_11C: u32 = 0x0101_ff40;
/// The floats the constructor copies into `+0x104`, `+0x108`, `+0x10c` (and `+0x120`, `+0x134`),
/// `+0x118`, `+0x12c` and `+0x130`.
const CONSTRUCTOR_FLOAT_104: u32 = 0x010c_b764;
const CONSTRUCTOR_FLOAT_108: u32 = 0x010c_b760;
const CONSTRUCTOR_FLOAT_10C: u32 = 0x0103_0ff0;
const CONSTRUCTOR_FLOAT_118: u32 = 0x0102_4100;
const CONSTRUCTOR_FLOAT_12C: u32 = 0x010c_b75c;
const CONSTRUCTOR_FLOAT_130: u32 = 0x0103_13d4;
/// The float the atoms' constructor copies into `+0x12c` (`0104da48`).
const ATOMS_FLOAT_12C: u32 = 0x0104_da48;
/// The class's vtable.
const RAGDOLL_VTABLE: u32 = 0x010c_b71c;
/// The base object's vtable.
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

/// The motors: the three words from `+0xe0` (twist, plane, cone).
const MOTORS: u32 = 0xe0;

// Translated from 00cde960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::setMaxLinearImpulse`: copies the float to `+0x13c`.
pub fn ragdoll_set_max_linear_impulse(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x13c, x87_quiet(value));
}

// Translated from 00cde970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getMaxLinearImpulse`: the float at `+0x13c`, in `ST0`.
pub fn ragdoll_get_max_linear_impulse(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x13c))
}

// Translated from 00cde980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::setBodyToNotify`: the byte at `+0x13a` becomes `1 << index` (the
/// shift count is the low byte of `index` taken modulo 32, and the result is one byte).
pub fn ragdoll_set_body_to_notify(e: &mut Engine, this: Ptr, index: u32) {
    let count = (index & 0xff) & 0x1f;
    e.mem.set_u8(this.addr() + 0x13a, (1u32 << count) as u8);
}

// Translated from 00cde9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getNotifiedBodyIndex`: the byte at `+0x13a` shifted right once.
pub fn ragdoll_get_notified_body_index(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + 0x13a)) >> 1
}

// Translated from 00cde9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getConstraintInfo`: the shared `getConstraintInfoUtil` on the
/// atoms at `this + 0x10`, `0x130` bytes of them, filling `info`.
pub fn ragdoll_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x130u32, info],
    );
}

// Translated from 00cde9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getRuntimeInfo`: the first stack word is not used; `info`
/// receives the runtime size `0x80` and the count 12.
pub fn ragdoll_get_runtime_info(e: &mut Engine, _this: Ptr, _unused_0: u32, info: Ptr) {
    e.mem.set_u32(info.addr() + 4, 0xc);
    e.mem.set_u32(info.addr(), 0x80);
}

// Translated from 00cde9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::setMaxFrictionTorque`: copies the float to `+0xf8`.
pub fn ragdoll_set_max_friction_torque(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0xf8, x87_quiet(value));
}

// Translated from 00cdea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word whose low byte is the flag): the byte at
/// `+0x116` becomes `0x38` when the flag is not zero and 0 otherwise.
pub fn fn_00cdea00(e: &mut Engine, this: Ptr, flag: u8) {
    let value = if flag != 0 { 0x38 } else { 0 };
    e.mem.set_u8(this.addr() + 0x116, value);
}

// Translated from 00cdea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getTwistMotor`: the word at `+0xe0`.
pub fn ragdoll_get_twist_motor(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + MOTORS))
}

// Translated from 00cdea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, two stack words that the callee pops): takes a
/// reference on the new motor (when there is one), releases the old one in slot `index`
/// (the words from `+0xe0`, when there is one) and stores the new.
pub fn fn_00cdea30(e: &mut Engine, this: Ptr, index: u32, motor: Ptr) {
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(MOTORS);
    if motor.addr() != 0 {
        e.call(ADD_REFERENCE, &args![motor]);
    }
    let old = e.mem.u32(slot);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(slot, motor.addr());
}

// Translated from 00cdea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word that the callee pops): `00cdea30` for
/// the motor in slot 0 (`+0xe0`, the twist motor).
pub fn fn_00cdea70(e: &mut Engine, this: Ptr, motor: Ptr) {
    fn_00cdea30(e, this, 0, motor);
}

// Translated from 00cdea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getConeMotor`: the word at `+0xe8`.
pub fn ragdoll_get_cone_motor(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + MOTORS + 8))
}

// Translated from 00cdea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word that the callee pops): `00cdea30` for
/// the motor in slot 2 (`+0xe8`, the cone motor).
pub fn fn_00cdea90(e: &mut Engine, this: Ptr, motor: Ptr) {
    fn_00cdea30(e, this, 2, motor);
}

// Translated from 00cdeaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getPlaneMotor`: the word at `+0xe4`.
pub fn ragdoll_get_plane_motor(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + MOTORS + 4))
}

// Translated from 00cdeab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word that the callee pops): `00cdea30` for
/// the motor in slot 1 (`+0xe4`, the plane motor).
pub fn fn_00cdeab0(e: &mut Engine, this: Ptr, motor: Ptr) {
    fn_00cdea30(e, this, 1, motor);
}

// Translated from 00cdead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::getType`: the constraint type, 7.
pub fn ragdoll_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    7
}

// Translated from 00cdeae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, five stack words that the callee pops): the
/// world-space counterpart of `setInBodySpace`, but that identity is unconfirmed. The two axes
/// are normalised (`V`, `W`) and `U = V x W`. The rotation at `+0x20` has the rows `V`, `W`, `U`
/// carried through `transform_a` (each is `x * column 0 + y * column 1 + z * column 2` of the
/// transform's rows), the rotation at `+0x60` the same through `transform_b`, the pivots at
/// `+0x50` and `+0x90` are the pivot carried into each body's space
/// (`setTransformedInversePos`), and the rows at `+0x60` are copied to `+0xb0`.
pub fn fn_00cdeae0(
    e: &mut Engine,
    this: Ptr,
    transform_a: u32,
    transform_b: u32,
    pivot: u32,
    axis_a: u32,
    axis_b: u32,
) {
    let data = this.addr();
    let first = load_vec(e, axis_a);
    let first = normalised(e, first);
    let second = load_vec(e, axis_b);
    let second = normalised(e, second);
    let basis = [first, second, cross_yzx(first, second)];
    for (matrix, target) in [(transform_a, 0x20), (transform_b, 0x60)] {
        let rows = [
            load_vec(e, matrix),
            load_vec(e, matrix + 0x10),
            load_vec(e, matrix + 0x20),
        ];
        let table = columns(rows);
        for (index, vector) in basis.into_iter().enumerate() {
            store_vec(
                e,
                data + target + 0x10 * index as u32,
                combine_columns(&table, vector),
            );
        }
    }
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![data + 0x50, transform_a, pivot],
    );
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![data + 0x90, transform_b, pivot],
    );
    for row in 0..3 {
        copy_vec(e, data + 0x60 + 0x10 * row, data + 0xb0 + 0x10 * row);
    }
}

// Translated from 00cded30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::setInBodySpace` (`this` in `ECX`, six stack words that the
/// callee pops): body A's axis (normalised) goes to `+0x20`, its second vector (copied, then
/// normalised) to `+0x30` and their cross product to `+0x40`; the pivot to `+0x50`; body B's go
/// to `+0x60`, `+0x70`, `+0x80` and the pivot to `+0x90`; the rows of body B are copied to
/// `+0xb0`.
pub fn ragdoll_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: u32,
    pivot_b: u32,
    second_a: u32,
    second_b: u32,
    axis_a: u32,
    axis_b: u32,
) {
    let data = this.addr();
    // The SSE NaN rules throughout; body A's cross product has the second vector's shuffles
    // as the `MULPS` destinations, body B's the first vector's.
    for (rows, axis, second, pivot, pivot_target, a_first) in [
        (data + 0x20, axis_a, second_a, pivot_a, data + 0x50, false),
        (data + 0x60, axis_b, second_b, pivot_b, data + 0x90, true),
    ] {
        let vector = load_vec(e, axis);
        let unit = normalised_sse(e, vector);
        store_vec(e, rows, unit);
        copy_vec(e, second, rows + 0x10);
        let vector = load_vec(e, rows + 0x10);
        let unit = normalised_sse(e, vector);
        store_vec(e, rows + 0x10, unit);
        let first = load_vec(e, rows);
        let second = load_vec(e, rows + 0x10);
        store_vec(e, rows + 0x20, cross_yzx_sse(first, second, a_first));
        let pivot = load_vec(e, pivot);
        store_vec(e, pivot_target, pivot);
    }
    for row in 0..3 {
        copy_vec(e, data + 0x60 + 0x10 * row, data + 0xb0 + 0x10 * row);
    }
}

// Translated from 00cdef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, two floats that the callee pops, `first` and
/// `second`): the quaternion about the row at `+0x70` by `-(first + second) / 2` turns the row
/// at `+0x60`; the row at `+0x80` becomes the cross product of the turned row and the row at
/// `+0x70`; and the float at `+0x11c` becomes `(second - first) / 2`.
pub fn fn_00cdef90(e: &mut Engine, this: Ptr, first: f32, second: f32) {
    let data = this.addr();
    e.with_stack(0x18, |e, frame| {
        let quaternion = frame.addr() + 8;
        let scratch = frame.addr() + 4;
        let half: f64 = e.global(HALF_F64);
        let first = f64::from(x87_quiet(first));
        let second = f64::from(x87_quiet(second));
        e.mem.set_f32(scratch, ((first + second) * half) as f32);
        let angle = -x87_quiet(e.mem.f32(scratch));
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![quaternion, data + 0x70, angle],
        );
        let difference = ((second - first) * half) as f32;
        let row = load_vec(e, data + 0x60);
        let turn = load_vec(e, quaternion);
        let turned = rotate_by_quaternion(e, turn, row);
        store_vec(e, data + 0x60, turned);
        let turned = load_vec(e, data + 0x60);
        let axis = load_vec(e, data + 0x70);
        store_vec(e, data + 0x80, cross_yzx(turned, axis));
        e.mem.set_f32(data + 0x11c, difference);
    });
}

// Translated from 00cdf090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::isValid` (`this` in `ECX`, one stack word that the callee pops):
/// the byte at `valid` becomes 1 when the rotations at `+0x20` and `+0x60` are orthonormal to
/// within the tolerance at `01017d00` (`hkRotation::isOrthonormal`), the float at `+0x118`
/// equals `-100.0`, the float at `+0x11c` is not below zero and below pi, the float at `+0x130`
/// is not below the float at `+0x12c` and the float at `+0x108` is not below the float at
/// `+0x104` (an unordered value fails every test), else 0. Returns `valid`.
pub fn ragdoll_is_valid(e: &mut Engine, this: Ptr, valid: u32) -> u32 {
    let data = this.addr();
    let ok = e.with_stack(4, |e, scratch| {
        e.mem.set_u8(valid, 1);
        for rotation in [data + 0x20, data + 0x60] {
            let epsilon = x87_quiet(e.global::<f32>(ORTHONORMAL_EPSILON));
            let got = e
                .call(IS_ORTHONORMAL, &args![rotation, scratch.addr(), epsilon])
                .u32();
            if e.mem.u8(got) == 0 {
                return false;
            }
        }
        let float_at = |e: &Engine, offset: u32| f64::from(x87_quiet(e.mem.f32(data + offset)));
        let expected: f64 = e.global(EXPECTED_AT_118);
        if float_at(e, 0x118) != expected {
            return false;
        }
        let angle = float_at(e, 0x11c);
        if !matches!(
            0.0f64.partial_cmp(&angle),
            Some(Ordering::Less | Ordering::Equal)
        ) {
            return false;
        }
        let limit: f64 = e.global(LIMIT_AT_11C);
        if angle.partial_cmp(&limit) != Some(Ordering::Less) {
            return false;
        }
        let not_below = |e: &Engine, high: u32, low: u32| {
            matches!(
                float_at(e, high).partial_cmp(&float_at(e, low)),
                Some(Ordering::Greater | Ordering::Equal)
            )
        };
        not_below(e, 0x130, 0x12c) && not_below(e, 0x108, 0x104)
    });
    e.mem.set_u8(valid, u8::from(ok));
    valid
}

// Translated from 00cdf190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word that the callee pops): copies the three
/// rows at `+0x20` to the three vectors at `out`.
pub fn fn_00cdf190(e: &mut Engine, this: Ptr, out: u32) {
    for row in 0..3 {
        copy_vec(e, this.addr() + 0x20 + 0x10 * row, out + 0x10 * row);
    }
}

// Translated from 00cdf1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word that the callee pops): copies the three
/// rows at `+0x60` to the three vectors at `out`.
pub fn fn_00cdf1c0(e: &mut Engine, this: Ptr, out: u32) {
    for row in 0..3 {
        copy_vec(e, this.addr() + 0x60 + 0x10 * row, out + 0x10 * row);
    }
}

// Translated from 00cdf1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): the destructor body. The vtable is the class's
/// while the three motors from `+0xe0` are released (each when there is one), then it is the
/// base object's.
pub fn fn_00cdf1f0(e: &mut Engine, this: Ptr) {
    let data = this.addr();
    e.mem.set_u32(data, RAGDOLL_VTABLE);
    for slot in 0..3 {
        let motor = e.mem.u32(data + MOTORS + 4 * slot);
        if motor != 0 {
            e.call(REMOVE_REFERENCE, &args![motor]);
        }
    }
    e.mem.set_u32(data, BASE_OBJECT_VTABLE);
}

// Translated from 00cdf230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::setTarget` (`this` in `ECX`, one word that the callee pops): the
/// rows at `+0xb0` become the product (`hkMatrix3::setMul`) of the rows at `+0x60` and the
/// matrix.
pub fn ragdoll_set_target(e: &mut Engine, this: Ptr, matrix: u32) {
    let data = this.addr();
    e.call(MATRIX_SET_MUL, &args![data + 0xb0, data + 0x60, matrix]);
}

// Translated from 00cdf290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::setMotorsActive` (`this` in `ECX`, two stack words that the callee
/// pops): the byte at `+0xa2` becomes `active`, the byte at `+0xf2` its negation (1 when
/// `active` is zero); then, when `instance`, its word at `+0x30` and that one's word at `+0x1c`
/// all exist, the twelve floats from there are zeroed.
pub fn ragdoll_set_motors_active(e: &mut Engine, this: Ptr, instance: u32, active: u8) {
    let data = this.addr();
    e.mem.set_u8(data + 0xa2, active);
    e.mem.set_u8(data + 0xf2, u8::from(active == 0));
    if instance == 0 {
        return;
    }
    let runtime = e.mem.u32(instance + 0x30);
    if runtime == 0 {
        return;
    }
    let block = e.mem.u32(runtime + 0x1c);
    if block == 0 {
        return;
    }
    for lane in 0..12 {
        e.mem.set_f32(block + 4 * lane, 0.0);
    }
}

// Translated from 00cdf2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::hkpRagdollConstraintData` (`this` in `ECX`): the atoms are
/// initialised (`00cdf460` on `this + 0x10`), the rotations at `+0x20` and `+0x60` and the rows
/// at `+0xb0` are the identity, the pivots and the motors are empty, `+0xa4` and `+0xa6` are
/// `0x60` and `0x64`, the flag byte at `+0xf4` is 3, the flag at `+0x116` is set with
/// `00cdea00(1)`, and the floats at `+0xf8`, `+0x104`, `+0x108`, `+0x10c`, `+0x118`, `+0x11c`,
/// `+0x120`, `+0x12c`, `+0x130` and `+0x134` and the small flag bytes get their starting values.
/// Returns `this`.
pub fn ragdoll_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let data = this.addr();
    e.mem.set_u16(data + 6, 1);
    e.mem.set_u32(data + 8, 0);
    e.mem.set_u32(data, RAGDOLL_VTABLE);
    fn_00cdf460(e, Ptr::new(data + 0x10));
    for offset in [0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90] {
        store_vec(e, data + offset, [0.0; 4]);
    }
    for offset in [0x20, 0x34, 0x48, 0x60, 0x74, 0x88] {
        e.mem.set_f32(data + offset, 1.0);
    }
    e.mem.set_u8(data + 0xf3, 0);
    e.mem.set_f32(data + 0xf8, 0.0);
    e.mem.set_u8(data + 0xf4, 3);
    e.mem.set_u8(data + 0xa2, 0);
    e.mem.set_u16(data + 0xa4, 0x60);
    e.mem.set_u16(data + 0xa6, 0x64);
    for offset in [0xb0, 0xc0, 0xd0] {
        store_vec(e, data + offset, [0.0; 4]);
    }
    for offset in [0xb0, 0xc4, 0xd8] {
        e.mem.set_f32(data + offset, 1.0);
    }
    e.mem.set_u32(data + MOTORS, 0);
    copy_quiet(e, CONSTRUCTOR_FLOAT_104, data + 0x104);
    e.mem.set_u32(data + MOTORS + 4, 0);
    copy_quiet(e, CONSTRUCTOR_FLOAT_108, data + 0x108);
    e.mem.set_u32(data + MOTORS + 8, 0);
    e.mem.set_u8(data + 0xff, 0);
    e.mem.set_u8(data + 0x100, 1);
    copy_quiet(e, CONSTRUCTOR_FLOAT_10C, data + 0x10c);
    e.mem.set_u8(data + 0x113, 0);
    copy_quiet(e, CONSTRUCTOR_FLOAT_118, data + 0x118);
    e.mem.set_u8(data + 0x114, 0);
    e.mem.set_u8(data + 0x115, 0);
    e.mem.set_f32(data + 0x11c, 1.0);
    copy_quiet(e, CONSTRUCTOR_FLOAT_10C, data + 0x120);
    fn_00cdea00(e, this, 1);
    copy_quiet(e, CONSTRUCTOR_FLOAT_12C, data + 0x12c);
    e.mem.set_u8(data + 0x127, 0);
    e.mem.set_u8(data + 0x128, 1);
    copy_quiet(e, CONSTRUCTOR_FLOAT_130, data + 0x130);
    e.mem.set_u8(data + 0x129, 1);
    copy_quiet(e, CONSTRUCTOR_FLOAT_10C, data + 0x134);
    this
}

// Translated from 00cdf460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollConstraintData::Atoms::Atoms` (`this` in `ECX` is the atoms block): the atoms'
/// type words and flags; the byte-sized float at `+0x12b` is set to `1.0` with `hkUFloat8`'s
/// assignment, and the float at `+0x12c` is copied from `0104da48`. Returns `this`.
pub fn fn_00cdf460(e: &mut Engine, this: Ptr) -> Ptr {
    let atoms = this.addr();
    e.with_stack(4, |e, one| {
        e.mem.set_f32(one.addr(), 1.0);
        e.mem.set_u16(atoms, 2);
        e.mem.set_u16(atoms + 0x90, 0x13);
        e.mem.set_u8(atoms + 0x92, 1);
        e.mem.set_u16(atoms + 0x96, 0);
        e.mem.set_u16(atoms + 0x94, 0);
        e.mem.set_u8(atoms + 0xe2, 1);
        e.mem.set_u8(atoms + 0xe4, 1);
        e.mem.set_u16(atoms + 0xe0, 0x11);
        e.mem.set_u8(atoms + 0xee, 1);
        e.mem.set_u16(atoms + 0xec, 0xf);
        e.mem.set_u8(atoms + 0x106, 0);
        e.mem.set_u8(atoms + 0x102, 1);
        e.mem.set_u16(atoms + 0x100, 0x10);
        e.mem.set_u8(atoms + 0x11a, 0);
        e.mem.set_u8(atoms + 0x116, 1);
        e.mem.set_u16(atoms + 0x114, 0x10);
        e.mem.set_u8(atoms + 0x12a, 0);
        e.mem.set_u16(atoms + 0x128, 5);
        e.call(UFLOAT8_ASSIGN, &args![atoms + 0x12b, one.addr()]);
    });
    copy_quiet(e, ATOMS_FLOAT_12C, atoms + 0x12c);
    this
}

/// Copies the float at `from` to `to` through the x87 stack (a quiet copy).
fn copy_quiet(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_e960, ragdoll_set_max_linear_impulse(Ptr, f32)),
        entry!(0x00cd_e970, ragdoll_get_max_linear_impulse(Ptr) -> f32),
        entry!(0x00cd_e980, ragdoll_set_body_to_notify(Ptr, u32)),
        entry!(0x00cd_e9a0, ragdoll_get_notified_body_index(Ptr) -> u32),
        entry!(0x00cd_e9b0, ragdoll_get_constraint_info(Ptr, u32)),
        entry!(0x00cd_e9d0, ragdoll_get_runtime_info(Ptr, u32, Ptr)),
        entry!(0x00cd_e9f0, ragdoll_set_max_friction_torque(Ptr, f32)),
        entry!(0x00cd_ea00, fn_00cdea00(Ptr, u8)),
        entry!(0x00cd_ea20, ragdoll_get_twist_motor(Ptr) -> Ptr),
        entry!(0x00cd_ea30, fn_00cdea30(Ptr, u32, Ptr)),
        entry!(0x00cd_ea70, fn_00cdea70(Ptr, Ptr)),
        entry!(0x00cd_ea80, ragdoll_get_cone_motor(Ptr) -> Ptr),
        entry!(0x00cd_ea90, fn_00cdea90(Ptr, Ptr)),
        entry!(0x00cd_eaa0, ragdoll_get_plane_motor(Ptr) -> Ptr),
        entry!(0x00cd_eab0, fn_00cdeab0(Ptr, Ptr)),
        entry!(0x00cd_ead0, ragdoll_get_type(Ptr) -> u32),
        entry!(0x00cd_eae0, fn_00cdeae0(Ptr, u32, u32, u32, u32, u32)),
        entry!(
            0x00cd_ed30,
            ragdoll_set_in_body_space(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00cd_ef90, fn_00cdef90(Ptr, f32, f32)),
        entry!(0x00cd_f090, ragdoll_is_valid(Ptr, u32) -> u32),
        entry!(0x00cd_f190, fn_00cdf190(Ptr, u32)),
        entry!(0x00cd_f1c0, fn_00cdf1c0(Ptr, u32)),
        entry!(0x00cd_f1f0, fn_00cdf1f0(Ptr)),
        entry!(0x00cd_f230, ragdoll_set_target(Ptr, u32)),
        entry!(0x00cd_f290, ragdoll_set_motors_active(Ptr, u32, u8)),
        entry!(0x00cd_f2f0, ragdoll_constructor(Ptr) -> Ptr),
        entry!(0x00cd_f460, fn_00cdf460(Ptr) -> Ptr),
        entry!(0x00cd_f530, deleting_destructor_00cdf530(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00cdf530_frees_only_when_bit_zero_is_set() {
        check(0x00cd_f530, 0x00cd_f1f0);
    }
}

#[cfg(test)]
mod a05_ragdoll_tests {
    use super::super::hkpsphereshape::{Vec4, HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const ADD_TOKEN: u32 = 1;
    const REMOVE_TOKEN: u32 = 2;

    /// The reference counting calls made, as `(token, object)`.
    type RefLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// An engine with every constant page the functions read mapped, the constants the vector
    /// helpers need set, and the call log on.
    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
            0x0102_4000,
            0x0103_0000,
            0x0103_1000,
            0x0104_d000,
            0x010c_7000,
            0x010c_b000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f64(HALF_F64, 0.5);
        for lane in 0..4 {
            e.mem.set_f32(0x010c_71e0 + 4 * lane, -0.5);
        }
        e.mem.set_f32(ORTHONORMAL_EPSILON, 0.001);
        e.mem.set_f64(EXPECTED_AT_118, -100.0);
        e.mem.set_f64(LIMIT_AT_11C, std::f64::consts::PI);
        e.call_log = Some(vec![]);
        e
    }

    /// Doubles that log the reference counting calls.
    fn count_references(e: &mut Engine) -> RefLog {
        let log = RefLog::default();
        let sink = log.clone();
        e.register_double(ADD_REFERENCE, move |_, a| {
            sink.borrow_mut().push((ADD_TOKEN, a[0]));
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            sink.borrow_mut().push((REMOVE_TOKEN, a[0]));
            Ret::default()
        });
        log
    }

    fn vector(e: &mut Engine, v: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec(e, at, v);
        at
    }

    fn near(actual: Vec4, expected: [f32; 3]) {
        for (lane, (a, x)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (a - x).abs() <= 1e-4,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn scalars_flags_and_types() {
        let mut e = world();
        let data = e.mem.alloc(0x140);
        e.call(0x00cd_e960, &args![data, 2.5f32]);
        assert_eq!(e.mem.f32(data + 0x13c), 2.5);
        assert_eq!(e.call(0x00cd_e970, &args![data]).f64(), 2.5);
        // A signalling NaN is quieted by the copy through the x87 stack.
        e.call(0x00cd_e960, &args![data, f32::from_bits(0x7fa0_0000)]);
        assert_eq!(e.mem.u32(data + 0x13c), 0x7fe0_0000);
        e.mem.set_u32(data + 0x13c, 0x7fa0_0001);
        assert!(e.call(0x00cd_e970, &args![data]).f64().is_nan());
        e.call(0x00cd_e9f0, &args![data, -1.5f32]);
        assert_eq!(e.mem.f32(data + 0xf8), -1.5);
        assert_eq!(e.call(0x00cd_ead0, &args![data]).u32(), 7);
    }

    #[test]
    fn body_to_notify_is_a_bit_and_reads_back_shifted() {
        let mut e = world();
        let data = e.mem.alloc(0x140);
        // (index word, byte stored): only the low byte counts, taken modulo 32 for the shift
        // count, and the result is cut to one byte.
        for (index, stored) in [
            (0u32, 1u8),
            (3, 8),
            (7, 0x80),
            (8, 0),
            (0x23, 8),
            (0x103, 8),
            (0xffff_ff1f, 0),
        ] {
            e.mem.set_u8(data + 0x13a, 0xaa);
            e.call(0x00cd_e980, &args![data, index]);
            assert_eq!(e.mem.u8(data + 0x13a), stored, "{index:#x}");
        }
        e.mem.set_u8(data + 0x13a, 0x80);
        assert_eq!(e.call(0x00cd_e9a0, &args![data]).u32(), 0x40);
        e.mem.set_u8(data + 0x13a, 1);
        assert_eq!(e.call(0x00cd_e9a0, &args![data]).u32(), 0);
    }

    #[test]
    fn motor_flag_only_looks_at_the_low_byte() {
        let mut e = world();
        let data = e.mem.alloc(0x140);
        e.call(0x00cd_ea00, &args![data, 1u32]);
        assert_eq!(e.mem.u8(data + 0x116), 0x38);
        e.call(0x00cd_ea00, &args![data, 0x100u32]);
        assert_eq!(e.mem.u8(data + 0x116), 0);
        e.call(0x00cd_ea00, &args![data, 0x80u32]);
        assert_eq!(e.mem.u8(data + 0x116), 0x38);
    }

    #[test]
    fn constraint_and_runtime_info() {
        let mut e = world();
        e.register(GET_CONSTRAINT_INFO_UTIL, |_, _| Ret::default());
        let data = e.mem.alloc(0x140);
        let info = e.mem.alloc(0x10);
        e.call(0x00cd_e9b0, &args![data, info]);
        assert_eq!(
            calls_to(&e, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![data + 0x10, 0x130, info]]
        );
        e.call(0x00cd_e9d0, &args![data, 5u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x80, 0xc));
    }

    #[test]
    fn motors_are_referenced_released_and_read_back() {
        let mut e = world();
        let log = count_references(&mut e);
        let data = e.mem.alloc(0x140);
        let (twist, plane, cone): (u32, u32, u32) = (0x1000_0001, 0x1000_0002, 0x1000_0003);
        e.call(0x00cd_ea70, &args![data, twist]);
        e.call(0x00cd_eab0, &args![data, plane]);
        e.call(0x00cd_ea90, &args![data, cone]);
        assert_eq!(
            *log.borrow(),
            vec![(ADD_TOKEN, twist), (ADD_TOKEN, plane), (ADD_TOKEN, cone)]
        );
        assert_eq!(e.call(0x00cd_ea20, &args![data]).u32(), twist);
        assert_eq!(e.call(0x00cd_eaa0, &args![data]).u32(), plane);
        assert_eq!(e.call(0x00cd_ea80, &args![data]).u32(), cone);
        assert_eq!(
            [0xe0, 0xe4, 0xe8].map(|offset| e.mem.u32(data + offset)),
            [twist, plane, cone]
        );
        // Replacing: the new motor gets its reference before the old one is released.
        log.borrow_mut().clear();
        e.call(0x00cd_ea30, &args![data, 1u32, 0x1000_0009u32]);
        assert_eq!(
            *log.borrow(),
            vec![(ADD_TOKEN, 0x1000_0009), (REMOVE_TOKEN, plane)]
        );
        assert_eq!(e.mem.u32(data + 0xe4), 0x1000_0009);
        // Clearing releases only; clearing an empty slot does nothing.
        log.borrow_mut().clear();
        e.call(0x00cd_ea70, &args![data, 0u32]);
        assert_eq!(*log.borrow(), vec![(REMOVE_TOKEN, twist)]);
        assert_eq!(e.mem.u32(data + 0xe0), 0);
        log.borrow_mut().clear();
        e.call(0x00cd_ea70, &args![data, 0u32]);
        assert!(log.borrow().is_empty());
        // Filling an empty slot only takes the reference.
        e.call(0x00cd_ea70, &args![data, 0x1000_0007u32]);
        assert_eq!(*log.borrow(), vec![(ADD_TOKEN, 0x1000_0007)]);
        assert_eq!(e.mem.u32(data + 0xe0), 0x1000_0007);
    }

    #[test]
    fn set_in_body_space_normalises_and_completes_each_basis() {
        let mut e = world();
        let data = e.mem.alloc(0x100);
        let pivot_a = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let pivot_b = vector(&mut e, [-1.0, -2.0, -3.0, -4.0]);
        let second_a = vector(&mut e, [0.0, 0.0, 2.0, 0.0]);
        let second_b = vector(&mut e, [0.0, 0.0, 5.0, 0.0]);
        let axis_a = vector(&mut e, [0.0, 3.0, 0.0, 0.0]);
        let axis_b = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00cd_ed30,
            &args![data, pivot_a, pivot_b, second_a, second_b, axis_a, axis_b],
        );
        near(load_vec(&e, data + 0x20), [0.0, 1.0, 0.0]);
        near(load_vec(&e, data + 0x30), [0.0, 0.0, 1.0]);
        near(load_vec(&e, data + 0x40), [1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, data + 0x50), [1.0, 2.0, 3.0, 4.0]);
        near(load_vec(&e, data + 0x60), [1.0, 0.0, 0.0]);
        near(load_vec(&e, data + 0x70), [0.0, 0.0, 1.0]);
        near(load_vec(&e, data + 0x80), [0.0, -1.0, 0.0]);
        assert_eq!(load_vec(&e, data + 0x90), [-1.0, -2.0, -3.0, -4.0]);
        for row in 0..3 {
            assert_eq!(
                load_vec(&e, data + 0xb0 + 0x10 * row),
                load_vec(&e, data + 0x60 + 0x10 * row)
            );
        }
        // The sources are not changed.
        assert_eq!(load_vec(&e, second_a), [0.0, 0.0, 2.0, 0.0]);
    }

    #[test]
    fn set_in_world_space_carries_the_basis_through_each_transform() {
        let mut e = world();
        e.register(SET_TRANSFORMED_INVERSE_POS, |_, _| Ret::default());
        let data = e.mem.alloc(0x100);
        // A quarter turn about z for body A, the identity for body B.
        let turn = e.mem.alloc(0x30);
        let identity = e.mem.alloc(0x30);
        let rows = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let unit = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        for (index, (turned, plain)) in rows.into_iter().zip(unit).enumerate() {
            let [x, y, z] = turned;
            store_vec(&mut e, turn + 0x10 * index as u32, [x, y, z, 7.0]);
            let [x, y, z] = plain;
            store_vec(&mut e, identity + 0x10 * index as u32, [x, y, z, 7.0]);
        }
        let pivot = vector(&mut e, [5.0, 6.0, 7.0, 0.0]);
        let axis_a = vector(&mut e, [4.0, 0.0, 0.0, 0.0]);
        let axis_b = vector(&mut e, [0.0, 2.0, 0.0, 0.0]);
        e.call(
            0x00cd_eae0,
            &args![data, turn, identity, pivot, axis_a, axis_b],
        );
        // Body A: the columns of the quarter turn, (0,1,0), (-1,0,0) and (0,0,1).
        near(load_vec(&e, data + 0x20), [0.0, 1.0, 0.0]);
        near(load_vec(&e, data + 0x30), [-1.0, 0.0, 0.0]);
        near(load_vec(&e, data + 0x40), [0.0, 0.0, 1.0]);
        // Body B: the basis itself, x, y, and x cross y.
        near(load_vec(&e, data + 0x60), [1.0, 0.0, 0.0]);
        near(load_vec(&e, data + 0x70), [0.0, 1.0, 0.0]);
        near(load_vec(&e, data + 0x80), [0.0, 0.0, 1.0]);
        for row in 0..3 {
            assert_eq!(
                load_vec(&e, data + 0xb0 + 0x10 * row),
                load_vec(&e, data + 0x60 + 0x10 * row)
            );
        }
        assert_eq!(
            calls_to(&e, SET_TRANSFORMED_INVERSE_POS),
            vec![
                vec![data + 0x50, turn, pivot],
                vec![data + 0x90, identity, pivot]
            ]
        );
    }

    #[test]
    fn twist_limit_turns_the_plane_row_and_stores_half_the_difference() {
        let mut e = world();
        e.register_double(QUATERNION_SET_AXIS_ANGLE, |e, a| {
            let axis = load_vec(e, a[1]);
            let (sin, cos) = (f64::from(f32::from_bits(a[2])) / 2.0).sin_cos();
            let sin = sin as f32;
            store_vec(
                e,
                a[0],
                [axis[0] * sin, axis[1] * sin, axis[2] * sin, cos as f32],
            );
            Ret::default()
        });
        let data = e.mem.alloc(0x140);
        store_vec(&mut e, data + 0x60, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, data + 0x70, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00cd_ef90, &args![data, 0.2f32, 0.6f32]);
        let angle = -(((f64::from(0.2f32) + f64::from(0.6f32)) * 0.5) as f32);
        let quaternion = calls_to(&e, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(quaternion.len(), 1);
        assert_eq!(quaternion[0][1], data + 0x70);
        assert_eq!(f32::from_bits(quaternion[0][2]), angle);
        let (sin, cos) = (-0.4f32).sin_cos();
        near(load_vec(&e, data + 0x60), [cos, sin, 0.0]);
        near(load_vec(&e, data + 0x70), [0.0, 0.0, 1.0]);
        near(load_vec(&e, data + 0x80), [sin, -cos, 0.0]);
        let half_difference = ((f64::from(0.6f32) - f64::from(0.2f32)) * 0.5) as f32;
        assert_eq!(e.mem.f32(data + 0x11c), half_difference);
    }

    /// Doubles for `isOrthonormal` that report the rotation at `bad` as not orthonormal.
    fn orthonormal_check(e: &mut Engine, bad: Rc<Cell<u32>>) {
        e.register_double(IS_ORTHONORMAL, move |e, a| {
            e.mem.set_u8(a[1], u8::from(a[0] != bad.get()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    /// An object whose values pass every test of `isValid`.
    fn valid_object(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x140);
        e.mem.set_f32(data + 0x118, -100.0);
        e.mem.set_f32(data + 0x11c, 1.0);
        e.mem.set_f32(data + 0x12c, 1.0);
        e.mem.set_f32(data + 0x130, 2.0);
        e.mem.set_f32(data + 0x104, -3.0);
        e.mem.set_f32(data + 0x108, 3.0);
        data
    }

    #[test]
    fn is_valid_checks_both_rotations_then_every_range() {
        let mut e = world();
        let bad = Rc::new(Cell::new(0));
        orthonormal_check(&mut e, bad.clone());
        let data = valid_object(&mut e);
        let valid = e.mem.alloc(8);
        let run = |e: &mut Engine| {
            e.mem.set_u8(valid, 0xaa);
            let got = e.call(0x00cd_f090, &args![data, valid]).u32();
            assert_eq!(got, valid);
            e.mem.u8(valid)
        };
        assert_eq!(run(&mut e), 1);
        let calls = calls_to(&e, IS_ORTHONORMAL);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][0], data + 0x20);
        assert_eq!(calls[1][0], data + 0x60);
        assert_eq!(f32::from_bits(calls[0][2]), 0.001);
        assert_eq!(f32::from_bits(calls[1][2]), 0.001);
        assert_eq!(calls[0][1], calls[1][1]);

        // The second rotation fails: both were asked.
        e.call_log = Some(vec![]);
        bad.set(data + 0x60);
        assert_eq!(run(&mut e), 0);
        assert_eq!(calls_to(&e, IS_ORTHONORMAL).len(), 2);
        // The first rotation fails: the second is not asked.
        e.call_log = Some(vec![]);
        bad.set(data + 0x20);
        assert_eq!(run(&mut e), 0);
        assert_eq!(calls_to(&e, IS_ORTHONORMAL).len(), 1);
        bad.set(0);

        // 3.14 is a test value just below pi, not pi itself.

        #[allow(clippy::approx_constant)]
        let cases: [(u32, f32, u8); 21] = [
            (0x118, -100.0, 1),
            (0x118, -99.0, 0),
            (0x118, f32::NAN, 0),
            (0x11c, 0.0, 1),
            (0x11c, -0.5, 0),
            (0x11c, 3.14, 1),
            (0x11c, 3.2, 0),
            // The float pi is a little above the double pi.
            (0x11c, std::f32::consts::PI, 0),
            (0x11c, f32::NAN, 0),
            (0x130, 1.0, 1),
            (0x130, 0.5, 0),
            (0x130, f32::NAN, 0),
            (0x12c, 2.0, 1),
            (0x12c, 2.5, 0),
            (0x12c, f32::NAN, 0),
            (0x108, -3.0, 1),
            (0x108, -3.5, 0),
            (0x108, f32::NAN, 0),
            (0x104, 3.0, 1),
            (0x104, 3.5, 0),
            (0x104, f32::NAN, 0),
        ];
        for (offset, value, expected) in cases {
            let old = e.mem.f32(data + offset);
            e.mem.set_f32(data + offset, value);
            assert_eq!(run(&mut e), expected, "{offset:#x} {value}");
            e.mem.set_f32(data + offset, old);
        }
    }

    #[test]
    fn rotation_rows_are_copied_out() {
        let mut e = world();
        let data = e.mem.alloc(0x140);
        for index in 0..6u32 {
            let base = index as f32 * 10.0;
            let row_at = 0x20 + 0x10 * index + if index < 3 { 0 } else { 0x10 };
            store_vec(
                &mut e,
                data + row_at,
                [base, base + 1.0, base + 2.0, base + 3.0],
            );
        }
        let out = e.mem.alloc(0x30);
        e.call(0x00cd_f190, &args![data, out]);
        for row in 0..3u32 {
            let base = row as f32 * 10.0;
            assert_eq!(
                load_vec(&e, out + 0x10 * row),
                [base, base + 1.0, base + 2.0, base + 3.0]
            );
        }
        let out = e.mem.alloc(0x30);
        e.call(0x00cd_f1c0, &args![data, out]);
        for row in 0..3u32 {
            let base = (row + 3) as f32 * 10.0;
            assert_eq!(
                load_vec(&e, out + 0x10 * row),
                [base, base + 1.0, base + 2.0, base + 3.0]
            );
        }
    }

    #[test]
    fn destructor_body_releases_each_motor_and_restores_the_base_vtable() {
        let mut e = world();
        let log = count_references(&mut e);
        let data = e.mem.alloc(0x140);
        e.mem.set_u32(data + 0xe0, 0x1000_0001);
        e.mem.set_u32(data + 0xe8, 0x1000_0003);
        e.call(0x00cd_f1f0, &args![data]);
        assert_eq!(
            *log.borrow(),
            vec![(REMOVE_TOKEN, 0x1000_0001), (REMOVE_TOKEN, 0x1000_0003)]
        );
        assert_eq!(e.mem.u32(data), BASE_OBJECT_VTABLE);
        log.borrow_mut().clear();
        e.mem.set_u32(data + 0xe4, 0x1000_0002);
        e.call(0x00cd_f1f0, &args![data]);
        assert_eq!(
            *log.borrow(),
            vec![
                (REMOVE_TOKEN, 0x1000_0001),
                (REMOVE_TOKEN, 0x1000_0002),
                (REMOVE_TOKEN, 0x1000_0003)
            ]
        );
    }

    #[test]
    fn set_target_multiplies_the_second_rotation_by_the_matrix() {
        let mut e = world();
        e.register(MATRIX_SET_MUL, |_, _| Ret::default());
        let data = e.mem.alloc(0x140);
        e.call(0x00cd_f230, &args![data, 0x1234_0000u32]);
        assert_eq!(
            calls_to(&e, MATRIX_SET_MUL),
            vec![vec![data + 0xb0, data + 0x60, 0x1234_0000]]
        );
    }

    #[test]
    fn motors_active_clears_the_runtime_floats_when_the_chain_exists() {
        let mut e = world();
        let data = e.mem.alloc(0x140);
        let block = e.mem.alloc(0x40);
        let inner = e.mem.alloc(0x40);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x30, inner);
        e.mem.set_u32(inner + 0x1c, block);
        let fill = |e: &mut Engine| {
            for lane in 0..14 {
                e.mem.set_f32(block + 4 * lane, 7.0);
            }
        };
        fill(&mut e);
        e.call(0x00cd_f290, &args![data, instance, 1u32]);
        assert_eq!((e.mem.u8(data + 0xa2), e.mem.u8(data + 0xf2)), (1, 0));
        for lane in 0..14 {
            let expected = if lane < 12 { 0.0 } else { 7.0 };
            assert_eq!(e.mem.f32(block + 4 * lane), expected, "{lane}");
        }
        // Only the low byte of the flag counts.
        fill(&mut e);
        e.call(0x00cd_f290, &args![data, instance, 0x100u32]);
        assert_eq!((e.mem.u8(data + 0xa2), e.mem.u8(data + 0xf2)), (0, 1));
        assert_eq!(e.mem.f32(block), 0.0);
        // Missing links leave the floats alone, and no instance only sets the flags.
        fill(&mut e);
        e.mem.set_u32(inner + 0x1c, 0);
        e.call(0x00cd_f290, &args![data, instance, 1u32]);
        e.mem.set_u32(instance + 0x30, 0);
        e.call(0x00cd_f290, &args![data, instance, 1u32]);
        e.call(0x00cd_f290, &args![data, 0u32, 0u32]);
        assert_eq!((e.mem.u8(data + 0xa2), e.mem.u8(data + 0xf2)), (0, 1));
        assert_eq!(e.mem.f32(block), 7.0);
    }

    /// The flag bytes and halves the atoms' constructor sets, as `(offset, value)`.
    const ATOM_BYTES: [(u32, u8); 11] = [
        (0x92, 1),
        (0xe2, 1),
        (0xe4, 1),
        (0xee, 1),
        (0x106, 0),
        (0x102, 1),
        (0x11a, 0),
        (0x116, 1),
        (0x12a, 0),
        (0x0, 2),
        (0x90, 0x13),
    ];
    const ATOM_HALVES: [(u32, u16); 6] = [
        (0x96, 0),
        (0x94, 0),
        (0xe0, 0x11),
        (0xec, 0xf),
        (0x100, 0x10),
        (0x114, 0x10),
    ];

    /// Doubles for the unsigned-float assignment that remember the value it was given.
    fn assignment(e: &mut Engine) -> Rc<Cell<f32>> {
        let seen = Rc::new(Cell::new(f32::NAN));
        let sink = seen.clone();
        e.register_double(UFLOAT8_ASSIGN, move |e, a| {
            sink.set(e.mem.f32(a[1]));
            Ret::default()
        });
        seen
    }

    fn dirty(e: &mut Engine, size: u32) -> u32 {
        let at = e.mem.alloc(size);
        for offset in 0..size {
            e.mem.set_u8(at + offset, 0xaa);
        }
        at
    }

    #[test]
    fn atoms_constructor_sets_the_types_flags_and_the_unsigned_float() {
        let mut e = world();
        let seen = assignment(&mut e);
        e.mem.set_u32(ATOMS_FLOAT_12C, 0x7fa0_0000);
        let atoms = dirty(&mut e, 0x140);
        assert_eq!(e.call(0x00cd_f460, &args![atoms]).u32(), atoms);
        for (offset, value) in ATOM_BYTES {
            assert_eq!(e.mem.u8(atoms + offset), value, "{offset:#x}");
        }
        assert_eq!(e.mem.u16(atoms + 0x128), 5);
        for (offset, value) in ATOM_HALVES {
            assert_eq!(e.mem.u16(atoms + offset), value, "{offset:#x}");
        }
        assert_eq!(seen.get(), 1.0);
        assert_eq!(
            calls_to(&e, UFLOAT8_ASSIGN)
                .iter()
                .map(|a| a[0])
                .collect::<Vec<_>>(),
            vec![atoms + 0x12b]
        );
        // The copied float is quieted.
        assert_eq!(e.mem.u32(atoms + 0x12c), 0x7fe0_0000);
    }

    #[test]
    fn constructor_builds_identity_rotations_and_default_limits() {
        let mut e = world();
        assignment(&mut e);
        e.mem.set_u32(CONSTRUCTOR_FLOAT_104, 0x7fa0_0000);
        e.mem.set_f32(CONSTRUCTOR_FLOAT_108, 0.5);
        e.mem.set_f32(CONSTRUCTOR_FLOAT_10C, 1.0e6);
        e.mem.set_f32(CONSTRUCTOR_FLOAT_118, -3.5);
        e.mem.set_f32(CONSTRUCTOR_FLOAT_12C, 0.125);
        e.mem.set_f32(CONSTRUCTOR_FLOAT_130, 9.0);
        e.mem.set_f32(ATOMS_FLOAT_12C, 0.75);
        let data = dirty(&mut e, 0x140);
        assert_eq!(e.call(0x00cd_f2f0, &args![data]).u32(), data);
        assert_eq!(e.mem.u16(data + 6), 1);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data), RAGDOLL_VTABLE);
        // The atoms are built at +0x10.
        assert_eq!(e.mem.u16(data + 0x10), 2);
        assert_eq!(e.mem.u16(data + 0x10 + 0x90), 0x13);
        assert_eq!(e.mem.f32(data + 0x10 + 0x12c), 0.75);
        // The rotations and the pivots.
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        for (base, rows) in [(0x20, identity), (0x60, identity), (0xb0, identity)] {
            for (index, row) in rows.into_iter().enumerate() {
                assert_eq!(load_vec(&e, data + base + 0x10 * index as u32), row);
            }
        }
        assert_eq!(load_vec(&e, data + 0x50), [0.0; 4]);
        assert_eq!(load_vec(&e, data + 0x90), [0.0; 4]);
        assert_eq!(
            (
                e.mem.u8(data + 0xa2),
                e.mem.u16(data + 0xa4),
                e.mem.u16(data + 0xa6)
            ),
            (0, 0x60, 0x64)
        );
        // The motors and the flag bytes.
        assert_eq!(
            [0xe0, 0xe4, 0xe8].map(|offset| e.mem.u32(data + offset)),
            [0, 0, 0]
        );
        let bytes = [
            (0xf3, 0),
            (0xf4, 3),
            (0xff, 0),
            (0x100, 1),
            (0x113, 0),
            (0x114, 0),
            (0x115, 0),
            (0x116, 0x38),
            (0x127, 0),
            (0x128, 1),
            (0x129, 1),
        ];
        for (offset, value) in bytes {
            assert_eq!(e.mem.u8(data + offset), value, "{offset:#x}");
        }
        // The limits: quiet copies of the constants, and the plain values.
        assert_eq!(e.mem.f32(data + 0xf8), 0.0);
        assert_eq!(e.mem.u32(data + 0x104), 0x7fe0_0000);
        assert_eq!(e.mem.f32(data + 0x108), 0.5);
        for offset in [0x10c, 0x120, 0x134] {
            assert_eq!(e.mem.f32(data + offset), 1.0e6, "{offset:#x}");
        }
        assert_eq!(e.mem.f32(data + 0x118), -3.5);
        assert_eq!(e.mem.f32(data + 0x11c), 1.0);
        assert_eq!(e.mem.f32(data + 0x12c), 0.125);
        assert_eq!(e.mem.f32(data + 0x130), 9.0);
    }
}
