//! `Havok SDK/hkplimitedhingeconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::cross;
use crate::units::havok_sdk::hkpbvagent::LARGE_FLOAT;
use crate::units::havok_sdk::hkphingeconstraintdata::{
    beside, normalised, rotate_rows, CONSTRAINT_INFO_UTIL, IS_ORTHONORMAL, ORTHONORMAL_TOLERANCE,
    U_FLOAT8_ASSIGN,
};
use crate::units::havok_sdk::hkpprimitivedrawer::normalised_sse;
use crate::units::havok_sdk::hkpprismaticconstraintdata::cross_yzx_sse;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cb9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cb_8b40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cb9220(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cb_8b40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::addReference` and `removeReference` (`00c90510`, `00c905b0`; `this` =
/// the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Vtable of the constraint data and of the `hkReferencedObject` base it ends as.
const VTABLE: u32 = 0x010c_89e4;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The floats the constructor loads: -pi and +pi (the default angular limits).
const MINUS_PI: u32 = 0x0106_b648;
const PLUS_PI: u32 = 0x0102_b3c8;

// Translated from 00cb8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::setMaxLinearImpulse` (Xbox PDB): the float at `+0xD8`.
pub fn hkp_limited_hinge_constraint_data_set_max_linear_impulse(
    e: &mut Engine,
    this: Ptr,
    impulse: f32,
) {
    e.mem.set_f32(this.addr() + 0xd8, x87_quiet(impulse));
}

// Translated from 00cb8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::getMaxLinearImpulse` (Xbox PDB).
pub fn hkp_limited_hinge_constraint_data_get_max_linear_impulse(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xd8))
}

// Translated from 00cb8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::setBodyToNotify` (Xbox PDB): the mask byte at `+0xD6` is
/// `1 << index` (the shift count is the low five bits and the shift is 8 bits wide). Answers
/// `this` (`EAX`, left over from the code).
pub fn hkp_limited_hinge_constraint_data_set_body_to_notify(
    e: &mut Engine,
    this: Ptr,
    index: u32,
) -> Ptr {
    let shift = index & 0x1f;
    let mask = if shift < 8 { 1u8 << shift } else { 0 };
    e.mem.set_u8(this.addr() + 0xd6, mask);
    this
}

// Translated from 00cb8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::getNotifiedBodyIndex` (Xbox PDB): the mask byte at `+0xD6`
/// shifted right once.
pub fn hkp_limited_hinge_constraint_data_get_notified_body_index(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + 0xd6)) >> 1
}

// Translated from 00cb8ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::setMotor` (Xbox PDB): takes a reference on the new motor
/// when there is one, releases the one held at `+0xB0` when there is one, and stores the new
/// pointer there.
pub fn hkp_limited_hinge_constraint_data_set_motor(e: &mut Engine, this: Ptr, motor: Ptr) {
    if motor.addr() != 0 {
        e.call(ADD_REFERENCE, &args![motor]);
    }
    let held = e.mem.u32(this.addr() + 0xb0);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(this.addr() + 0xb0, motor.addr());
}

// Translated from 00cb8af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::getConstraintInfo` (Xbox PDB): `getConstraintInfoUtil` over
/// the atoms at `+0x10`, `0xCC` bytes of them.
pub fn hkp_limited_hinge_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    e.call(
        CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0xccu32, info],
    );
}

// Translated from 00cb8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::getRuntimeInfo` (Xbox PDB): the first word is not read;
/// `info` gets `{0x50, 8}` (`+0` the size in bytes, `+4` the number of solver results).
pub fn hkp_limited_hinge_constraint_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    info: Ptr,
) {
    e.mem.set_u32(info.addr() + 4, 8);
    e.mem.set_u32(info.addr(), 0x50);
}

// Translated from 00cb8b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::getType` (Xbox PDB): 2.
pub fn hkp_limited_hinge_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    2
}

// Translated from 00cb8b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body that the deleting destructor (`00cb9220`) runs (unnamed in the map;
/// unconfirmed): the vtable is set, the motor at `+0xB0` is released when there is one, and
/// the `hkReferencedObject` vtable is left in place.
pub fn fn_00cb8b40(e: &mut Engine, this: Ptr) {
    let motor = e.mem.u32(this.addr() + 0xb0);
    e.mem.set_u32(this.addr(), VTABLE);
    if motor != 0 {
        e.call(REMOVE_REFERENCE, &args![motor]);
    }
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00cb8b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::setInWorldSpace` (Xbox PDB): the axis (copied word by word)
/// is normalised into the first row; the second is the normalised vector built beside that row
/// ([`beside`]) and the third the cross product of the two. `rotate_rows` puts them, rotated by
/// each transform, at `+0x20/+0x30/+0x40` (body A) and `+0x60/+0x70/+0x80` (body B), with the
/// pivot at `+0x50` and `+0x90`.
pub fn hkp_limited_hinge_constraint_data_set_in_world_space(
    e: &mut Engine,
    this: Ptr,
    transform_a: Ptr,
    transform_b: Ptr,
    pivot: Ptr,
    axis: Ptr,
) {
    let first = normalised(e, load_vec(e, axis.addr()));
    let second = normalised(e, beside(first));
    let third = cross(first, second);
    e.with_stack(0x30, |e, frame| {
        let rows = [0, 0x10, 0x20].map(|row| frame.addr() + row);
        for (at, row) in rows.into_iter().zip([first, second, third]) {
            store_vec(e, at, row);
        }
        rotate_rows(e, this.addr(), [transform_a, transform_b], pivot, rows);
    });
}

// Translated from 00cb8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::setInBodySpace` (Xbox PDB): the pivots are copied to `+0x50`
/// and `+0x90`; for each body the axis is normalised into the first row (`+0x20`, `+0x60`),
/// the perpendicular (the fifth and sixth words; their meaning is unconfirmed) is normalised
/// into the second (`+0x30`, `+0x70`) and the third (`+0x40`, `+0x80`) is the cross product of
/// the two.
pub fn hkp_limited_hinge_constraint_data_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: Ptr,
    pivot_b: Ptr,
    axis_a: Ptr,
    axis_b: Ptr,
    perpendicular_a: Ptr,
    perpendicular_b: Ptr,
) {
    let me = this.addr();
    for (row, axis, perpendicular, pivot_at, pivot) in [
        (0x20, axis_a, perpendicular_a, 0x50, pivot_a),
        (0x60, axis_b, perpendicular_b, 0x90, pivot_b),
    ] {
        // The SSE NaN rules: `MULPS` with the first vector's shuffles as the destinations.
        let first = normalised_sse(e, load_vec(e, axis.addr()));
        store_vec(e, me + row, first);
        let second = normalised_sse(e, load_vec(e, perpendicular.addr()));
        store_vec(e, me + row + 0x10, second);
        store_vec(e, me + row + 0x20, cross_yzx_sse(first, second, true));
        let point = load_vec(e, pivot.addr());
        store_vec(e, me + pivot_at, point);
    }
}

// Translated from 00cb8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::setMotorActive` (Xbox PDB): the byte at `+0xA2` is `active`
/// (its low byte) and the byte at `+0xB6` is 1 when it is 0. When `instance` has a pointer at
/// `+0x30` whose `+0x1C` is set, the four floats there (the solver results of the
/// instance; meaning unconfirmed) are zeroed.
pub fn hkp_limited_hinge_constraint_data_set_motor_active(
    e: &mut Engine,
    this: Ptr,
    instance: Ptr,
    active: u8,
) {
    e.mem.set_u8(this.addr() + 0xa2, active);
    e.mem.set_u8(this.addr() + 0xb6, u8::from(active == 0));
    if instance.addr() == 0 {
        return;
    }
    let internal = e.mem.u32(instance.addr() + 0x30);
    if internal == 0 {
        return;
    }
    let results = e.mem.u32(internal + 0x1c);
    if results == 0 {
        return;
    }
    for lane in 0..4 {
        e.mem.set_f32(results + 4 * lane, 0.0);
    }
}

// Translated from 00cb9020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::isValid` (Xbox PDB): `isOrthonormal` (tolerance from
/// `01017d00`) of the rows at `+0x20` and `+0x60`, and when the byte at `+0xA2` is set also the
/// byte at `+0xC2` must be set. The byte at `result` is 1 when all hold and 0 as soon as one
/// fails. Answers `result` (`EAX`).
pub fn hkp_limited_hinge_constraint_data_is_valid(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let me = this.addr();
    let tolerance = x87_quiet_bits(e.global::<u32>(ORTHONORMAL_TOLERANCE));
    let valid = e.with_stack(4, |e, flags| {
        let first = e
            .call(
                IS_ORTHONORMAL,
                &args![me + 0x20, flags.addr() + 2, tolerance],
            )
            .u32();
        if e.mem.u8(first) == 0 {
            return false;
        }
        let second = e
            .call(
                IS_ORTHONORMAL,
                &args![me + 0x60, flags.addr() + 3, tolerance],
            )
            .u32();
        if e.mem.u8(second) == 0 {
            return false;
        }
        e.mem.u8(me + 0xa2) == 0 || e.mem.u8(me + 0xc2) != 0
    });
    e.mem.set_u8(result.addr(), u8::from(valid));
    result
}

// Translated from 00cb9090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::hkpLimitedHingeConstraintData` (Xbox PDB): reference count
/// 1, `+8` zero, the vtable, the atoms (`+0x10`), identity rows with zero pivots, and the
/// atom fields the constructor sets: the angular limits -pi (`+0xC4`) and +pi (`+0xC8`), 1.0
/// at `+0xCC`, zero at `+0xAC` and `+0xBC`, the words `0x10` (`+0xA8`), `0x40` (`+0xA4`) and
/// `0x44` (`+0xA6`), the byte 1 at `+0xB8` and zero bytes at `+0xA2`, `+0xA3`, `+0xB7`,
/// `+0xC3` and `+0xD2` and a null motor (`+0xB0`). Answers `this`.
pub fn hkp_limited_hinge_constraint_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me, VTABLE);
    hkp_limited_hinge_constraint_data_atoms_construct(e, Ptr::new(me + 0x10));
    for row in [0x20, 0x60] {
        for offset in [0, 0x10, 0x20, 0x30] {
            store_vec(e, me + row + offset, [0.0; 4]);
        }
        for diagonal in [0, 0x14, 0x28] {
            e.mem.set_f32(me + row + diagonal, 1.0);
        }
    }
    let minus_pi = x87_quiet_bits(e.global::<u32>(MINUS_PI));
    let plus_pi = x87_quiet_bits(e.global::<u32>(PLUS_PI));
    e.mem.set_u8(me + 0xd2, 0);
    e.mem.set_u32(me + 0xc4, minus_pi);
    e.mem.set_u8(me + 0xc3, 0);
    e.mem.set_u8(me + 0xb7, 0);
    e.mem.set_u32(me + 0xc8, plus_pi);
    e.mem.set_u16(me + 0xa8, 0x10);
    e.mem.set_f32(me + 0xcc, 1.0);
    e.mem.set_u8(me + 0xb8, 1);
    e.mem.set_u16(me + 0xa4, 0x40);
    e.mem.set_f32(me + 0xbc, 0.0);
    e.mem.set_u16(me + 0xa6, 0x44);
    e.mem.set_u8(me + 0xa2, 0);
    e.mem.set_f32(me + 0xac, 0.0);
    e.mem.set_u32(me + 0xb0, 0);
    e.mem.set_u8(me + 0xa3, 0);
    this
}

// Translated from 00cb9170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::Atoms::Atoms` (Xbox PDB): sets the atom headers and flags:
/// 16-bit words 2 at `+0`, `0x12` at `+0x90`, 0 at `+0x94` and `+0x96`, `0x11` at `+0xA4`, `0xE`
/// at `+0xB0`, `0xC` at `+0xC0` and 5 at `+0xC4`; bytes 1 at `+0x92`, `+0xA6`, `+0xA8` and
/// `+0xB2` and 0 at `+0xC6`; the `hkUFloat8` at `+0xC7` set to 1.0 and the large float
/// (`0104da48`) at `+0xC8`. Answers `this`.
pub fn hkp_limited_hinge_constraint_data_atoms_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me, 2);
    e.mem.set_u16(me + 0x90, 0x12);
    e.mem.set_u8(me + 0x92, 1);
    e.mem.set_u16(me + 0x94, 0);
    e.mem.set_u16(me + 0x96, 0);
    e.mem.set_u8(me + 0xa6, 1);
    e.mem.set_u8(me + 0xa8, 1);
    e.mem.set_u16(me + 0xa4, 0x11);
    e.mem.set_u16(me + 0xb0, 0xe);
    e.mem.set_u8(me + 0xb2, 1);
    e.mem.set_u16(me + 0xc0, 0xc);
    e.mem.set_u16(me + 0xc4, 5);
    e.mem.set_u8(me + 0xc6, 0);
    e.with_stack(4, |e, one| {
        e.mem.set_f32(one.addr(), 1.0);
        e.call(U_FLOAT8_ASSIGN, &args![me + 0xc7, one]);
    });
    let large = x87_quiet_bits(e.global::<u32>(LARGE_FLOAT));
    e.mem.set_u32(me + 0xc8, large);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cb_8a70,
            hkp_limited_hinge_constraint_data_set_max_linear_impulse(Ptr, f32)
        ),
        entry!(
            0x00cb_8a80,
            hkp_limited_hinge_constraint_data_get_max_linear_impulse(Ptr) -> f32
        ),
        entry!(
            0x00cb_8a90,
            hkp_limited_hinge_constraint_data_set_body_to_notify(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cb_8ab0,
            hkp_limited_hinge_constraint_data_get_notified_body_index(Ptr) -> u32
        ),
        entry!(
            0x00cb_8ac0,
            hkp_limited_hinge_constraint_data_set_motor(Ptr, Ptr)
        ),
        entry!(
            0x00cb_8af0,
            hkp_limited_hinge_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00cb_8b10,
            hkp_limited_hinge_constraint_data_get_runtime_info(Ptr, u32, Ptr)
        ),
        entry!(0x00cb_8b30, hkp_limited_hinge_constraint_data_get_type(Ptr) -> u32),
        entry!(0x00cb_8b40, fn_00cb8b40(Ptr)),
        entry!(
            0x00cb_8b60,
            hkp_limited_hinge_constraint_data_set_in_world_space(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00cb_8da0,
            hkp_limited_hinge_constraint_data_set_in_body_space(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00cb_8fe0,
            hkp_limited_hinge_constraint_data_set_motor_active(Ptr, Ptr, u8)
        ),
        entry!(
            0x00cb_9020,
            hkp_limited_hinge_constraint_data_is_valid(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cb_9090,
            hkp_limited_hinge_constraint_data_construct(Ptr) -> Ptr
        ),
        entry!(
            0x00cb_9170,
            hkp_limited_hinge_constraint_data_atoms_construct(Ptr) -> Ptr
        ),
        entry!(0x00cb_9220, deleting_destructor_00cb9220(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00cb9220_frees_only_when_bit_zero_is_set() {
        check(0x00cb_9220, 0x00cb_8b40);
    }
}

#[cfg(test)]
mod limited_hinge_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use crate::units::havok_sdk::hkphingeconstraintdata::{
        IS_ORTHONORMAL, ORTHONORMAL_TOLERANCE, ROTATE_VECTOR, SET_TRANSFORMED_INVERSE_POS,
        U_FLOAT8_ASSIGN,
    };
    use crate::units::havok_sdk::hkpsphereshape::{Vec4, HALF_F32, THREE_F32, ZERO_F32};
    use std::rc::Rc;

    /// The constants the normaliser reads from the exe data.
    fn set_constants(e: &mut Engine) {
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
    }

    fn assert_close(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-5,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    fn dot(a: Vec4, b: Vec4) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    /// A double at `address` that logs its arguments and the four words of the vector at
    /// argument 2 as the call sees them.
    fn vector_double(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            log.borrow_mut().push((address, words));
            Ret::default()
        });
    }

    fn vector_words(words: &[u32]) -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(words[3 + lane]))
    }

    #[test]
    fn max_linear_impulse_is_stored_and_read_through_the_x87_stack_00cb8a70() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.call(0x00cb_8a70, &args![this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0xd8), 2.5);
        assert_eq!(e.call(0x00cb_8a80, &args![this]).f32(), 2.5);
        e.call(0x00cb_8a70, &args![this, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xd8), 0xffc0_0001);
    }

    #[test]
    fn body_to_notify_sets_the_mask_and_the_index_reads_it_back_00cb8a90() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        for (index, mask) in [(0u32, 1u8), (1, 2), (7, 0x80), (8, 0), (33, 2)] {
            assert_eq!(e.call(0x00cb_8a90, &args![this, index]).u32(), this);
            assert_eq!(e.mem.u8(this + 0xd6), mask, "index {index}");
        }
        e.mem.set_u8(this + 0xd6, 0x80);
        assert_eq!(e.call(0x00cb_8ab0, &args![this]).u32(), 0x40);
        e.mem.set_u8(this + 0xd6, 1);
        assert_eq!(e.call(0x00cb_8ab0, &args![this]).u32(), 0);
    }

    #[test]
    fn set_motor_swaps_the_references_00cb8ac0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x100);
        e.call(0x00cb_8ac0, &args![this, 0x4000u32]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x4000]]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(this + 0xb0), 0x4000);
        e.call(0x00cb_8ac0, &args![this, 0x5000u32]);
        let order: Vec<(u32, u32)> = log.borrow().iter().map(|(a, w)| (*a, w[0])).collect();
        assert_eq!(
            order,
            [
                (ADD_REFERENCE, 0x4000),
                (ADD_REFERENCE, 0x5000),
                (REMOVE_REFERENCE, 0x4000)
            ]
        );
        e.call(0x00cb_8ac0, &args![this, 0u32]);
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 2);
        assert_eq!(calls(&log, REMOVE_REFERENCE).len(), 2);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
    }

    #[test]
    fn constraint_info_goes_through_the_util_over_the_atoms_00cb8af0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, CONSTRAINT_INFO_UTIL, 0);
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(0x20);
        e.call(0x00cb_8af0, &args![this, info]);
        assert_eq!(
            calls(&log, CONSTRAINT_INFO_UTIL),
            vec![vec![this + 0x10, 0xcc, info]]
        );
    }

    #[test]
    fn runtime_info_is_fifty_bytes_and_eight_results_00cb8b10() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(8);
        e.mem.write(info, &[0xff; 8]);
        e.call(0x00cb_8b10, &args![this, 0x1234u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x50, 8));
    }

    #[test]
    fn the_type_is_two_00cb8b30() {
        let mut e = engine();
        assert_eq!(e.call(0x00cb_8b30, &args![0u32]).u32(), 2);
    }

    #[test]
    fn the_destructor_body_releases_the_motor_and_ends_as_the_base_00cb8b40() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x100);
        e.call(0x00cb_8b40, &args![this]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        e.mem.set_u32(this + 0xb0, 0x6000);
        e.call(0x00cb_8b40, &args![this]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn world_space_setup_rotates_the_normalised_axis_and_two_built_rows_00cb8b60() {
        let mut e = engine();
        set_constants(&mut e);
        let log = new_log();
        vector_double(&mut e, &log, ROTATE_VECTOR);
        vector_double(&mut e, &log, SET_TRANSFORMED_INVERSE_POS);
        let this = e.mem.alloc(0x100);
        let axis = e.mem.alloc(0x10);
        store_vec(&mut e, axis, [0.0, 0.0, 2.0, 0.5]);
        let pivot = e.mem.alloc(0x10);
        let (transform_a, transform_b) = (0x1000u32, 0x2000u32);
        e.call(
            0x00cb_8b60,
            &args![this, transform_a, transform_b, pivot, axis],
        );
        let rotations = calls(&log, ROTATE_VECTOR);
        let outputs: Vec<u32> = rotations.iter().map(|c| c[0]).collect();
        assert_eq!(
            outputs,
            [0x20, 0x30, 0x40, 0x60, 0x70, 0x80].map(|o| this + o)
        );
        let transforms: Vec<u32> = rotations.iter().map(|c| c[1]).collect();
        assert_eq!(
            transforms,
            [
                transform_a,
                transform_a,
                transform_a,
                transform_b,
                transform_b,
                transform_b
            ]
        );
        for half in [0, 3] {
            assert_close(vector_words(&rotations[half]), [0.0, 0.0, 1.0, 0.25]);
            assert_close(vector_words(&rotations[half + 1]), [0.0, 1.0, 0.0, 0.0]);
            assert_close(vector_words(&rotations[half + 2]), [-1.0, 0.0, 0.0, 0.0]);
        }
        let pivots = calls(&log, SET_TRANSFORMED_INVERSE_POS);
        let heads: Vec<[u32; 3]> = pivots.iter().map(|c| [c[0], c[1], c[2]]).collect();
        assert_eq!(
            heads,
            [
                [this + 0x50, transform_a, pivot],
                [this + 0x90, transform_b, pivot]
            ]
        );
    }

    #[test]
    fn world_space_rows_are_orthonormal_for_a_slanted_axis_00cb8b60() {
        let mut e = engine();
        set_constants(&mut e);
        let log = new_log();
        vector_double(&mut e, &log, ROTATE_VECTOR);
        vector_double(&mut e, &log, SET_TRANSFORMED_INVERSE_POS);
        let this = e.mem.alloc(0x100);
        let axis = e.mem.alloc(0x10);
        store_vec(&mut e, axis, [3.0, 2.0, 1.0, 0.0]);
        let pivot = e.mem.alloc(0x10);
        e.call(0x00cb_8b60, &args![this, 0x1000u32, 0x2000u32, pivot, axis]);
        let rotations = calls(&log, ROTATE_VECTOR);
        let first = vector_words(&rotations[0]);
        let second = vector_words(&rotations[1]);
        let third = vector_words(&rotations[2]);
        let root = 14f32.sqrt();
        assert_close(first, [3.0 / root, 2.0 / root, 1.0 / root, 0.0]);
        assert!((dot(second, second) - 1.0).abs() < 1e-5);
        assert!(dot(first, second).abs() < 1e-5);
        assert_close(third, cross(first, second));
    }

    #[test]
    fn body_space_setup_normalises_both_rows_and_crosses_them_00cb8da0() {
        let mut e = engine();
        set_constants(&mut e);
        let this = e.mem.alloc(0x100);
        let [pivot_a, pivot_b, axis_a, axis_b, perpendicular_a, perpendicular_b] =
            [0; 6].map(|_| e.mem.alloc(0x10));
        store_vec(&mut e, pivot_a, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, pivot_b, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, axis_a, [0.0, 0.0, 2.0, 0.5]);
        store_vec(&mut e, perpendicular_a, [0.0, 3.0, 0.0, 0.0]);
        store_vec(&mut e, axis_b, [0.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, perpendicular_b, [2.0, 0.0, 0.0, 1.0]);
        e.call(
            0x00cb_8da0,
            &args![
                this,
                pivot_a,
                pivot_b,
                axis_a,
                axis_b,
                perpendicular_a,
                perpendicular_b
            ],
        );
        assert_eq!(load_vec(&e, this + 0x50), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x90), [5.0, 6.0, 7.0, 8.0]);
        assert_close(load_vec(&e, this + 0x20), [0.0, 0.0, 1.0, 0.25]);
        assert_close(load_vec(&e, this + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_close(load_vec(&e, this + 0x40), [-1.0, 0.0, 0.0, 0.0]);
        assert_close(load_vec(&e, this + 0x60), [0.0, 1.0, 0.0, 0.0]);
        assert_close(load_vec(&e, this + 0x70), [1.0, 0.0, 0.0, 0.5]);
        // (0, 1, 0) x (1, 0, 0) = (0, 0, -1)
        assert_close(load_vec(&e, this + 0x80), [0.0, 0.0, -1.0, 0.0]);
    }

    #[test]
    fn motor_active_sets_the_flags_and_clears_the_results_00cb8fe0() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let results = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_f32(results + 4 * lane, 9.0);
        }
        let internal = e.mem.alloc(0x40);
        let instance = e.mem.alloc(0x40);
        // No instance, no internal data and no results leave the floats alone.
        e.call(0x00cb_8fe0, &args![this, 0u32, 5u32]);
        assert_eq!((e.mem.u8(this + 0xa2), e.mem.u8(this + 0xb6)), (5, 0));
        e.call(0x00cb_8fe0, &args![this, instance, 0u32]);
        assert_eq!((e.mem.u8(this + 0xa2), e.mem.u8(this + 0xb6)), (0, 1));
        e.mem.set_u32(instance + 0x30, internal);
        e.call(0x00cb_8fe0, &args![this, instance, 1u32]);
        assert_eq!(e.mem.f32(results), 9.0);
        e.mem.set_u32(internal + 0x1c, results);
        e.call(0x00cb_8fe0, &args![this, instance, 0x100u32]);
        assert_eq!((e.mem.u8(this + 0xa2), e.mem.u8(this + 0xb6)), (0, 1));
        for lane in 0..4 {
            assert_eq!(e.mem.f32(results + 4 * lane), 0.0);
        }
    }

    /// An `isOrthonormal` double that answers the bytes of `answers` in turn.
    fn orthonormal_double(e: &mut Engine, log: &Log, answers: Vec<u8>) {
        let log = Rc::clone(log);
        let mut answers = answers.into_iter();
        e.register_double(IS_ORTHONORMAL, move |e, a| {
            log.borrow_mut().push((IS_ORTHONORMAL, a.to_vec()));
            e.mem.set_u8(a[1], answers.next().unwrap());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn valid_needs_orthonormal_rows_and_an_enabled_second_flag_when_motor_on_00cb9020() {
        // (answers, byte at +0xA2, byte at +0xC2, expected, asked)
        for (answers, motor, limit, expected, asked) in [
            (vec![1u8, 1], 0u8, 0u8, 1u8, 2usize),
            (vec![1, 1], 1, 0, 0, 2),
            (vec![1, 1], 1, 1, 1, 2),
            (vec![1, 0], 0, 0, 0, 2),
            (vec![0, 1], 0, 0, 0, 1),
        ] {
            let mut e = engine();
            e.set_global(ORTHONORMAL_TOLERANCE, 0x3a83_126fu32);
            let log = new_log();
            orthonormal_double(&mut e, &log, answers);
            let this = e.mem.alloc(0x100);
            e.mem.set_u8(this + 0xa2, motor);
            e.mem.set_u8(this + 0xc2, limit);
            let result = e.mem.alloc(4);
            e.mem.set_u8(result, 0x55);
            assert_eq!(e.call(0x00cb_9020, &args![this, result]).u32(), result);
            assert_eq!(e.mem.u8(result), expected);
            let seen = calls(&log, IS_ORTHONORMAL);
            assert_eq!(seen.len(), asked);
            assert_eq!((seen[0][0], seen[0][2]), (this + 0x20, 0x3a83_126f));
            if asked == 2 {
                assert_eq!((seen[1][0], seen[1][2]), (this + 0x60, 0x3a83_126f));
            }
        }
    }

    #[test]
    fn atoms_set_the_headers_flags_and_the_unit_ufloat8_00cb9170() {
        let mut e = engine();
        e.set_global(LARGE_FLOAT, 0x7f7f_ffeeu32);
        let log = new_log();
        let sink = Rc::clone(&log);
        e.register_double(U_FLOAT8_ASSIGN, move |e, a| {
            sink.borrow_mut()
                .push((U_FLOAT8_ASSIGN, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = e.mem.alloc(0xe0);
        e.mem.write(this, &[0xcd; 0xe0]);
        assert_eq!(e.call(0x00cb_9170, &args![this]).u32(), this);
        assert_eq!(
            calls(&log, U_FLOAT8_ASSIGN),
            vec![vec![this + 0xc7, 1.0f32.to_bits()]]
        );
        for (offset, word) in [
            (0u32, 2u16),
            (0x90, 0x12),
            (0x94, 0),
            (0x96, 0),
            (0xa4, 0x11),
            (0xb0, 0xe),
            (0xc0, 0xc),
            (0xc4, 5),
        ] {
            assert_eq!(e.mem.u16(this + offset), word, "word at {offset:#x}");
        }
        for (offset, byte) in [(0x92u32, 1u8), (0xa6, 1), (0xa8, 1), (0xb2, 1), (0xc6, 0)] {
            assert_eq!(e.mem.u8(this + offset), byte, "byte at {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xc8), 0x7f7f_ffee);
        // Bytes the function does not write keep their value.
        assert_eq!(e.mem.u16(this + 2), 0xcdcd);
        assert_eq!(e.mem.u8(this + 0x93), 0xcd);
        assert_eq!(e.mem.u8(this + 0xa7), 0xcd);
        assert_eq!(e.mem.u8(this + 0xb3), 0xcd);
    }

    #[test]
    fn construct_sets_identity_rows_the_limits_and_the_flags_00cb9090() {
        let mut e = engine();
        e.set_global(LARGE_FLOAT, 0x7f7f_ffeeu32);
        e.set_global(MINUS_PI, (-std::f32::consts::PI).to_bits());
        e.set_global(PLUS_PI, std::f32::consts::PI.to_bits());
        e.register_double(U_FLOAT8_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcd; 0x100]);
        assert_eq!(e.call(0x00cb_9090, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 0x10), 2);
        for row in [0x20, 0x60] {
            assert_eq!(load_vec(&e, this + row), [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, this + row + 0x10), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, this + row + 0x20), [0.0, 0.0, 1.0, 0.0]);
            assert_eq!(load_vec(&e, this + row + 0x30), [0.0; 4]);
        }
        assert_eq!(e.mem.f32(this + 0xc4), -std::f32::consts::PI);
        assert_eq!(e.mem.f32(this + 0xc8), std::f32::consts::PI);
        assert_eq!(e.mem.f32(this + 0xcc), 1.0);
        assert_eq!((e.mem.f32(this + 0xac), e.mem.f32(this + 0xbc)), (0.0, 0.0));
        assert_eq!(
            (
                e.mem.u16(this + 0xa8),
                e.mem.u16(this + 0xa4),
                e.mem.u16(this + 0xa6)
            ),
            (0x10, 0x40, 0x44)
        );
        for (offset, byte) in [
            (0xa2u32, 0u8),
            (0xa3, 0),
            (0xb7, 0),
            (0xb8, 1),
            (0xc3, 0),
            (0xd2, 0),
        ] {
            assert_eq!(e.mem.u8(this + offset), byte, "byte at {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u8(this + 0xd3), 0xcd);
    }
}
