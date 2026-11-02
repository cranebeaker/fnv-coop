//! `Havok SDK/hkpprismaticconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use std::cmp::Ordering;

use super::hkpprimitivedrawer::{normalised, normalised_sse};
use super::hkpsphereshape::{
    load_vec, movhlps, movlhps, shufps, store_vec, unpckhps, unpcklps, zip_with, Vec4,
};

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_f8d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4ffd0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_f8d0, &args![this]);
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
/// The `float` tolerance `isValid` passes to `isOrthonormal` (`01017d00`).
const ORTHONORMAL_EPSILON: u32 = 0x0101_7d00;
/// The `float` the constructor starts the lower motor limit with (`0104e148`).
const DEFAULT_LOWER_LIMIT: u32 = 0x0104_e148;
/// The `float` the constructor starts the upper motor limit with (`0104da48`).
const DEFAULT_UPPER_LIMIT: u32 = 0x0104_da48;
/// The vtable the constructor and the destructor set (the class's own).
const PRISMATIC_VTABLE: u32 = 0x010d_0b3c;
/// The base object's vtable the destructor ends with.
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

/// The motor held at `+0xac`.
const MOTOR: u32 = 0xac;
/// The rotation and pivot of body A (`+0x20` three rows, `+0x50`), and of body B (`+0x60` three
/// rows, `+0x90`).
const ROTATION_A: u32 = 0x20;
const PIVOT_A: u32 = 0x50;
const ROTATION_B: u32 = 0x60;
const PIVOT_B: u32 = 0x90;

/// `a x b` the way the code forms it: `(a.yzx * b.zxy) - (a.zxy * b.yzx)` (`SHUFPS` 0xC9 and
/// 0xD2).
pub(crate) fn cross_yzx(a: Vec4, b: Vec4) -> Vec4 {
    let plus = zip_with(shufps(a, a, 0xc9), shufps(b, b, 0xd2), |x, y| x * y);
    let minus = zip_with(shufps(a, a, 0xd2), shufps(b, b, 0xc9), |x, y| x * y);
    zip_with(plus, minus, |x, y| x - y)
}

/// [`cross_yzx`] with the SSE NaN rules. With `a_first` each `MULPS` has the shuffled `a` as
/// its destination (`a.yzx * b.zxy - a.zxy * b.yzx`); otherwise the shuffled `b`
/// (`b.zxy * a.yzx - b.yzx * a.zxy`, the same products).
pub(crate) fn cross_yzx_sse(a: Vec4, b: Vec4, a_first: bool) -> Vec4 {
    use crate::sse::{mulps, subps};
    let (a_yzx, a_zxy) = (shufps(a, a, 0xc9), shufps(a, a, 0xd2));
    let (b_yzx, b_zxy) = (shufps(b, b, 0xc9), shufps(b, b, 0xd2));
    if a_first {
        subps(mulps(a_yzx, b_zxy), mulps(a_zxy, b_yzx))
    } else {
        subps(mulps(b_zxy, a_yzx), mulps(b_yzx, a_zxy))
    }
}

/// The columns of the three rows (`UNPCKLPS`, `UNPCKHPS`, `MOVLHPS`, `MOVHLPS`, `SHUFPS`): the
/// fourth lane of each column is whatever the shuffles leave there.
pub(crate) fn columns(rows: [Vec4; 3]) -> [Vec4; 3] {
    let low = unpcklps(rows[0], rows[1]);
    let high = unpckhps(rows[0], rows[1]);
    let first = movlhps(low, rows[2]);
    let second = shufps(movhlps(first, low), rows[2], 0xd4);
    let third = shufps(high, rows[2], 0xe4);
    [first, second, third]
}

/// `x * columns[0] + y * columns[1] + z * columns[2]` in the order `(y + x) + z`.
pub(crate) fn combine_columns(columns: &[Vec4; 3], v: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        (v[1] * columns[1][lane] + v[0] * columns[0][lane]) + v[2] * columns[2][lane]
    })
}

// Translated from 00d4f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::setMotor`: takes a reference on the new motor (when there is
/// one), releases the old one at `+0xac` (when there is one) and stores the new.
pub fn prismatic_set_motor(e: &mut Engine, this: Ptr, motor: Ptr) {
    let data = this.addr();
    if motor.addr() != 0 {
        e.call(ADD_REFERENCE, &args![motor]);
    }
    let old = e.mem.u32(data + MOTOR);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(data + MOTOR, motor.addr());
}

// Translated from 00d4f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::getConstraintInfo`: the shared `getConstraintInfoUtil` on the
/// atoms at `this + 0x10`, `0xc0` bytes of them, filling `info`.
pub fn prismatic_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0xc0u32, info],
    );
}

// Translated from 00d4f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::getRuntimeInfo`: the first stack word is not used; `info`
/// receives the runtime size `0x48` and the count 8.
pub fn prismatic_get_runtime_info(e: &mut Engine, _this: Ptr, _unused_0: u32, info: Ptr) {
    e.mem.set_u32(info.addr() + 4, 8);
    e.mem.set_u32(info.addr(), 0x48);
}

// Translated from 00d4f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::getType`: the constraint type, 6.
pub fn prismatic_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    6
}

// Translated from 00d4f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (unnamed in the engine map): the vtable is the class's while the motor
/// at `+0xac` is released (when there is one), then it is the base object's.
pub fn destructor_00d4f8d0(e: &mut Engine, this: Ptr) {
    let data = this.addr();
    let motor = e.mem.u32(data + MOTOR);
    e.mem.set_u32(data, PRISMATIC_VTABLE);
    if motor != 0 {
        e.call(REMOVE_REFERENCE, &args![motor]);
    }
    e.mem.set_u32(data, BASE_OBJECT_VTABLE);
}

// Translated from 00d4f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, four stack words that the callee pops): the
/// world-space counterpart of `setInBodySpace`, but that identity is unconfirmed.
/// `transform_a` and `transform_b` are rotations of three rows. The axis is normalised
/// (`V`); a vector perpendicular to it is built from the two of its components that the
/// comparisons of the absolute values (through the x87, so signalling NaNs are quieted) pick,
/// normalised (`P`), and `W = V x P`. The rotation at `+0x20` has the rows `V`, `P`, `W`
/// carried through `transform_a` (each is `x * column 0 + y * column 1 + z * column 2` of the
/// transform's rows), the rotation at `+0x60` the same through `transform_b`, and the pivots at
/// `+0x50` and `+0x90` are the pivot carried into each body's space (`setTransformedInversePos`).
pub fn fn_00d4f8f0(
    e: &mut Engine,
    this: Ptr,
    transform_a: u32,
    transform_b: u32,
    pivot: u32,
    axis: u32,
) {
    let data = this.addr();
    let axis_vector = load_vec(e, axis);
    let unit = normalised(e, axis_vector);
    let quiet: [f32; 3] = [0, 1, 2].map(|lane| x87_quiet(unit[lane]));
    let magnitude = quiet.map(f32::abs);
    let greater = |a: f32, b: f32| a.partial_cmp(&b) == Some(Ordering::Greater);
    // Equal and unordered values count as "not greater" in both comparisons.
    let (smallest, minimum_index, other) = if greater(magnitude[0], magnitude[1]) {
        (magnitude[1], 1usize, 0usize)
    } else {
        (magnitude[0], 0usize, 1usize)
    };
    let chosen = if greater(smallest, magnitude[2]) {
        minimum_index
    } else {
        2
    };
    let mut perpendicular = [0.0f32; 4];
    perpendicular[other] = quiet[chosen];
    perpendicular[chosen] = -quiet[other];
    let perpendicular = normalised(e, perpendicular);
    let third = cross_yzx(unit, perpendicular);
    let basis = [unit, perpendicular, third];
    for (matrix, target) in [(transform_a, ROTATION_A), (transform_b, ROTATION_B)] {
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
        &args![data + PIVOT_A, transform_a, pivot],
    );
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![data + PIVOT_B, transform_b, pivot],
    );
}

// Translated from 00d4fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::setInBodySpace` (`this` in `ECX`, six stack words that the
/// callee pops): copies the pivots to `+0x50` and `+0x90`; body A's axis (normalised) goes
/// to `+0x20`, its second vector (copied, then normalised) to `+0x30` and their cross product
/// to `+0x40`; body B's go to `+0x60`, `+0x70` and `+0x80`.
pub fn prismatic_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: u32,
    pivot_b: u32,
    axis_a: u32,
    axis_b: u32,
    second_a: u32,
    second_b: u32,
) {
    let data = this.addr();
    let pivot = load_vec(e, pivot_a);
    store_vec(e, data + PIVOT_A, pivot);
    let pivot = load_vec(e, pivot_b);
    store_vec(e, data + PIVOT_B, pivot);
    // The SSE NaN rules throughout; body A's cross product has the second vector's shuffles
    // as the `MULPS` destinations, body B's the first vector's.
    for (rows, axis, second, a_first) in [
        (data + ROTATION_A, axis_a, second_a, false),
        (data + ROTATION_B, axis_b, second_b, true),
    ] {
        let vector = load_vec(e, axis);
        store_vec(e, rows, normalised_sse(e, vector));
        let vector = load_vec(e, second);
        store_vec(e, rows + 0x10, vector);
        let vector = load_vec(e, rows + 0x10);
        store_vec(e, rows + 0x10, normalised_sse(e, vector));
        let first = load_vec(e, rows);
        let second = load_vec(e, rows + 0x10);
        store_vec(e, rows + 0x20, cross_yzx_sse(first, second, a_first));
    }
}

// Translated from 00d4fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::isValid`: the byte at `valid` becomes 1 when both rotations
/// (`+0x20` and `+0x60`) are orthonormal to within the tolerance at `01017d00`
/// (`hkRotation::isOrthonormal`) and the limit at `+0xcc` is not below the limit at `+0xc8`
/// (an unordered pair is invalid), else 0. Returns `valid`.
pub fn prismatic_is_valid(e: &mut Engine, this: Ptr, valid: u32) -> u32 {
    let data = this.addr();
    let ok = e.with_stack(4, |e, scratch| {
        let result = scratch.addr();
        let epsilon = x87_quiet(e.global::<f32>(ORTHONORMAL_EPSILON));
        let got = e
            .call(
                IS_ORTHONORMAL,
                &args![data + ROTATION_A, result + 2, epsilon],
            )
            .u32();
        if e.mem.u8(got) == 0 {
            return false;
        }
        let epsilon = x87_quiet(e.global::<f32>(ORTHONORMAL_EPSILON));
        let got = e
            .call(
                IS_ORTHONORMAL,
                &args![data + ROTATION_B, result + 3, epsilon],
            )
            .u32();
        if e.mem.u8(got) == 0 {
            return false;
        }
        let lower = f64::from(x87_quiet(e.mem.f32(data + 0xc8)));
        let upper = f64::from(x87_quiet(e.mem.f32(data + 0xcc)));
        matches!(
            upper.partial_cmp(&lower),
            Some(Ordering::Greater | Ordering::Equal)
        )
    });
    e.mem.set_u8(valid, u8::from(ok));
    valid
}

// Translated from 00d4fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrismaticConstraintData::hkpPrismaticConstraintData` (`this` in `ECX`): the atoms are
/// initialised (`00d4ff60`), both rotations are the identity, the pivots and the motor are
/// empty, the two limits start at the floats at `0104e148` and `0104da48`, and the small flags
/// and sizes are set. Returns `this`.
pub fn prismatic_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let data = this.addr();
    e.mem.set_u16(data + 6, 1);
    e.mem.set_u32(data + 8, 0);
    e.mem.set_u32(data, PRISMATIC_VTABLE);
    fn_00d4ff60(e, Ptr::new(data + 0x10));
    for offset in [0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90] {
        store_vec(e, data + offset, [0.0; 4]);
    }
    for offset in [0x20, 0x34, 0x48, 0x60, 0x74, 0x88] {
        e.mem.set_f32(data + offset, 1.0);
    }
    e.mem.set_u8(data + 0xc6, 0);
    let lower = x87_quiet_bits(e.mem.u32(DEFAULT_LOWER_LIMIT));
    e.mem.set_u32(data + 0xc8, lower);
    e.mem.set_u8(data + 0xb3, 0);
    let upper = x87_quiet_bits(e.mem.u32(DEFAULT_UPPER_LIMIT));
    e.mem.set_u32(data + 0xcc, upper);
    e.mem.set_f32(data + 0xb4, 0.0);
    e.mem.set_u8(data + 0xa2, 0);
    e.mem.set_f32(data + 0xa8, 0.0);
    e.mem.set_u32(data + MOTOR, 0);
    e.mem.set_u8(data + 0xa3, 0);
    e.mem.set_u8(data + 0xba, 0);
    e.mem.set_u16(data + 0xa4, 0x40);
    e.mem.set_u16(data + 0xa6, 0x44);
    e.mem.set_u8(data + 0xbb, 3);
    e.mem.set_u8(data + 0xbe, 1);
    e.mem.set_u8(data + 0xc2, 2);
    this
}

// Translated from 00d4ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The atoms' initialiser (unnamed in the engine map; `this` in `ECX` is the block at the
/// constraint data's `+0x10`): the type words and flags of the constraint atoms. Returns `this`.
pub fn fn_00d4ff60(e: &mut Engine, this: Ptr) -> Ptr {
    let atoms = this.addr();
    e.mem.set_u16(atoms, 2);
    e.mem.set_u16(atoms + 0x90, 0xb);
    e.mem.set_u8(atoms + 0x92, 1);
    e.mem.set_u16(atoms + 0x94, 0);
    e.mem.set_u16(atoms + 0x96, 0);
    e.mem.set_u8(atoms + 0xa2, 1);
    e.mem.set_u16(atoms + 0xa0, 0xa);
    e.mem.set_u16(atoms + 0xa8, 0xd);
    e.mem.set_u16(atoms + 0xac, 7);
    e.mem.set_u16(atoms + 0xb0, 7);
    e.mem.set_u16(atoms + 0xb4, 9);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_f850, prismatic_set_motor(Ptr, Ptr)),
        entry!(0x00d4_f880, prismatic_get_constraint_info(Ptr, u32)),
        entry!(0x00d4_f8a0, prismatic_get_runtime_info(Ptr, u32, Ptr)),
        entry!(0x00d4_f8c0, prismatic_get_type(Ptr) -> u32),
        entry!(0x00d4_f8d0, destructor_00d4f8d0(Ptr)),
        entry!(0x00d4_f8f0, fn_00d4f8f0(Ptr, u32, u32, u32, u32)),
        entry!(
            0x00d4_fba0,
            prismatic_set_in_body_space(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00d4_fdd0, prismatic_is_valid(Ptr, u32) -> u32),
        entry!(0x00d4_fe80, prismatic_constructor(Ptr) -> Ptr),
        entry!(0x00d4_ff60, fn_00d4ff60(Ptr) -> Ptr),
        entry!(0x00d4_ffd0, deleting_destructor_00d4ffd0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d4ffd0_frees_only_when_bit_zero_is_set() {
        check(0x00d4_ffd0, 0x00d4_f8d0);
    }
}

#[cfg(test)]
mod a05_prismatic_tests {
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An engine whose reference counting and helper callees do nothing, with the call log on.
    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_6000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        e.map(0x0104_d000, 0x2000);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        for address in [
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            GET_CONSTRAINT_INFO_UTIL,
            SET_TRANSFORMED_INVERSE_POS,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn vector(e: &mut Engine, v: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec(e, at, v);
        at
    }

    fn assert_near(actual: Vec4, expected: [f32; 3]) {
        for (lane, (a, x)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (a - x).abs() <= 1e-4,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    #[test]
    fn set_motor_references_the_new_motor_and_releases_the_old() {
        let mut e = world();
        let data = e.mem.alloc(0x100);
        e.call(0x00d4_f850, &args![data, 0x1111u32]);
        assert_eq!(e.mem.u32(data + 0xac), 0x1111);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x1111]]);
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
        e.call(0x00d4_f850, &args![data, 0u32]);
        assert_eq!(e.mem.u32(data + 0xac), 0);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x1111]]);
        e.call(0x00d4_f850, &args![data, 0u32]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 1);
    }

    #[test]
    fn constraint_info_runtime_info_and_type() {
        let mut e = world();
        let data = e.mem.alloc(0x100);
        e.call(0x00d4_f880, &args![data, 0x7777u32]);
        assert_eq!(
            calls_to(&e, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![data + 0x10, 0xc0, 0x7777]]
        );
        let info = e.mem.alloc(8);
        e.call(0x00d4_f8a0, &args![data, 0xffffu32, info]);
        assert_eq!(e.mem.u32(info), 0x48);
        assert_eq!(e.mem.u32(info + 4), 8);
        assert_eq!(e.call(0x00d4_f8c0, &args![data]).u32(), 6);
    }

    #[test]
    fn destructor_releases_the_motor_and_ends_as_a_base_object() {
        let mut e = world();
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data + 0xac, 0x2222);
        e.call(0x00d4_f8d0, &args![data]);
        assert_eq!(e.mem.u32(data), BASE_OBJECT_VTABLE);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x2222]]);
        e.mem.set_u32(data + 0xac, 0);
        e.call(0x00d4_f8d0, &args![data]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 1);
    }

    #[test]
    fn world_space_setup_builds_a_basis_from_the_axis() {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let turned = [
            [0.0, -1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let cases = [
            ([0.0, 0.0, 2.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]),
            ([3.0, 0.0, 4.0], [0.8, 0.0, -0.6], [0.0, 1.0, 0.0]),
            (
                [1.0, 1.0, 0.0],
                [
                    -std::f32::consts::FRAC_1_SQRT_2,
                    std::f32::consts::FRAC_1_SQRT_2,
                    0.0,
                ],
                [0.0, 0.0, 1.0],
            ),
        ];
        for (axis, perpendicular, third) in cases {
            let mut e = world();
            let data = e.mem.alloc(0x100);
            let a = e.mem.alloc(0x30);
            let b = e.mem.alloc(0x30);
            for (index, (row_a, row_b)) in identity.into_iter().zip(turned).enumerate() {
                store_vec(&mut e, a + 0x10 * index as u32, row_a);
                store_vec(&mut e, b + 0x10 * index as u32, row_b);
            }
            let pivot = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
            let axis_vector = vector(&mut e, [axis[0], axis[1], axis[2], 0.0]);
            e.call(0x00d4_f8f0, &args![data, a, b, pivot, axis_vector]);
            let length = axis.iter().map(|v| v * v).sum::<f32>().sqrt();
            let unit = axis.map(|v| v / length);
            assert_near(load_vec(&e, data + 0x20), unit);
            assert_near(load_vec(&e, data + 0x30), perpendicular);
            assert_near(load_vec(&e, data + 0x40), third);
            // The second rotation turns (x, y, z) into (-y, x, z) of the same rows.
            let turn = |v: [f32; 3]| [-v[1], v[0], v[2]];
            assert_near(load_vec(&e, data + 0x60), turn(unit));
            assert_near(load_vec(&e, data + 0x70), turn(perpendicular));
            assert_near(load_vec(&e, data + 0x80), turn(third));
            assert_eq!(
                calls_to(&e, SET_TRANSFORMED_INVERSE_POS),
                vec![vec![data + 0x50, a, pivot], vec![data + 0x90, b, pivot]]
            );
        }
    }

    #[test]
    fn body_space_setup_normalises_the_axes_and_crosses_them() {
        let mut e = world();
        let data = e.mem.alloc(0x100);
        let pivot_a = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let pivot_b = vector(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let axis_a = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let axis_b = vector(&mut e, [0.0, 0.0, 5.0, 0.0]);
        let second_a = vector(&mut e, [0.0, 3.0, 0.0, 0.0]);
        let second_b = vector(&mut e, [5.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00d4_fba0,
            &args![data, pivot_a, pivot_b, axis_a, axis_b, second_a, second_b],
        );
        assert_eq!(load_vec(&e, data + 0x50), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, data + 0x90), [5.0, 6.0, 7.0, 8.0]);
        assert_near(load_vec(&e, data + 0x20), [1.0, 0.0, 0.0]);
        assert_near(load_vec(&e, data + 0x30), [0.0, 1.0, 0.0]);
        assert_near(load_vec(&e, data + 0x40), [0.0, 0.0, 1.0]);
        assert_near(load_vec(&e, data + 0x60), [0.0, 0.0, 1.0]);
        assert_near(load_vec(&e, data + 0x70), [1.0, 0.0, 0.0]);
        assert_near(load_vec(&e, data + 0x80), [0.0, 1.0, 0.0]);
        assert_eq!(load_vec(&e, second_a), [0.0, 3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, axis_a), [2.0, 0.0, 0.0, 0.0]);
    }

    /// `isValid` with the two orthonormality answers in `answers` (taken in order) and the
    /// limits at `+0xc8` and `+0xcc`; returns the byte and the `isOrthonormal` calls.
    fn check_valid(answers: [u8; 2], limits: [f32; 2]) -> (u8, Vec<Vec<u32>>) {
        let mut e = world();
        e.mem.set_f32(ORTHONORMAL_EPSILON, 0.001);
        let data = e.mem.alloc(0x100);
        e.mem.set_f32(data + 0xc8, limits[0]);
        e.mem.set_f32(data + 0xcc, limits[1]);
        let queue = Rc::new(RefCell::new(answers.to_vec()));
        e.register_double(IS_ORTHONORMAL, move |e, a| {
            let answer = queue.borrow_mut().remove(0);
            e.mem.set_u8(a[1], answer);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let valid = e.mem.alloc(8);
        e.mem.set_u8(valid, 0xaa);
        let got = e.call(0x00d4_fdd0, &args![data, valid]).u32();
        assert_eq!(got, valid);
        let calls = calls_to(&e, IS_ORTHONORMAL);
        if let Some(first) = calls.first() {
            assert_eq!(first[0], data + 0x20);
            assert_eq!(first[2], 0.001f32.to_bits());
        }
        if let Some(second) = calls.get(1) {
            assert_eq!(second[0], data + 0x60);
            assert_eq!(second[1], calls[0][1] + 1);
        }
        (e.mem.u8(valid), calls)
    }

    #[test]
    fn is_valid_needs_both_rotations_and_ordered_limits() {
        let (valid, calls) = check_valid([1, 1], [-1.0, 1.0]);
        assert_eq!(valid, 1);
        assert_eq!(calls.len(), 2);
        assert_eq!(check_valid([1, 1], [2.0, 2.0]).0, 1);
        assert_eq!(check_valid([1, 1], [2.0, -2.0]).0, 0);
        assert_eq!(check_valid([1, 1], [f32::NAN, 1.0]).0, 0);
        assert_eq!(check_valid([1, 1], [1.0, f32::NAN]).0, 0);
        let (valid, calls) = check_valid([0, 1], [-1.0, 1.0]);
        assert_eq!(valid, 0);
        assert_eq!(calls.len(), 1);
        let (valid, calls) = check_valid([1, 0], [-1.0, 1.0]);
        assert_eq!(valid, 0);
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn atoms_initialiser_sets_the_type_words_and_flags() {
        let mut e = world();
        let atoms = e.mem.alloc(0xc0);
        assert_eq!(e.call(0x00d4_ff60, &args![atoms]).u32(), atoms);
        assert_eq!(e.mem.u16(atoms), 2);
        assert_eq!(e.mem.u16(atoms + 0x90), 0xb);
        assert_eq!(e.mem.u8(atoms + 0x92), 1);
        assert_eq!(e.mem.u8(atoms + 0xa2), 1);
        assert_eq!(e.mem.u16(atoms + 0xa0), 0xa);
        assert_eq!(e.mem.u16(atoms + 0xa8), 0xd);
        assert_eq!(e.mem.u16(atoms + 0xac), 7);
        assert_eq!(e.mem.u16(atoms + 0xb0), 7);
        assert_eq!(e.mem.u16(atoms + 0xb4), 9);
    }

    #[test]
    fn constructor_sets_identity_rotations_empty_pivots_and_the_defaults() {
        let mut e = world();
        e.mem.set_u32(DEFAULT_LOWER_LIMIT, 0xff7f_ffee);
        e.mem.set_u32(DEFAULT_UPPER_LIMIT, 0x7f7f_ffee);
        let data = e.mem.alloc(0xe0);
        for offset in (0..0xe0).step_by(4) {
            e.mem.set_u32(data + offset, 0xcccc_cccc);
        }
        assert_eq!(e.call(0x00d4_fe80, &args![data]).u32(), data);
        assert_eq!(e.mem.u32(data), PRISMATIC_VTABLE);
        assert_eq!(e.mem.u16(data + 6), 1);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u16(data + 0x10), 2);
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        for (index, row) in identity.into_iter().enumerate() {
            assert_eq!(load_vec(&e, data + 0x20 + 0x10 * index as u32), row);
            assert_eq!(load_vec(&e, data + 0x60 + 0x10 * index as u32), row);
        }
        assert_eq!(load_vec(&e, data + 0x50), [0.0; 4]);
        assert_eq!(load_vec(&e, data + 0x90), [0.0; 4]);
        assert_eq!(e.mem.u32(data + 0xc8), 0xff7f_ffee);
        assert_eq!(e.mem.u32(data + 0xcc), 0x7f7f_ffee);
        assert_eq!(e.mem.u32(data + 0xac), 0);
        assert_eq!(e.mem.f32(data + 0xa8), 0.0);
        assert_eq!(e.mem.f32(data + 0xb4), 0.0);
        assert_eq!(
            [
                e.mem.u8(data + 0xa2),
                e.mem.u8(data + 0xa3),
                e.mem.u8(data + 0xb3),
                e.mem.u8(data + 0xba),
                e.mem.u8(data + 0xc6),
            ],
            [0; 5]
        );
        assert_eq!(e.mem.u16(data + 0xa4), 0x40);
        assert_eq!(e.mem.u16(data + 0xa6), 0x44);
        assert_eq!(
            [
                e.mem.u8(data + 0xbb),
                e.mem.u8(data + 0xbe),
                e.mem.u8(data + 0xc2)
            ],
            [3, 1, 2]
        );
    }
}
