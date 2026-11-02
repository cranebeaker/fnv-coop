//! `Havok SDK/hkpmotoraction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMotorAction` (a `hkpUnaryAction` subclass, vtable `010d0944`): the axis (four lanes,
//! normalized) at +0x20, the target speed at +0x30, the gain at +0x34 and the enabled flag
//! (a byte) at +0x38. The entity it acts on is the word at +0x18, the user data at +0x10.

use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// The vtable of `hkpMotorAction`.
const MOTOR_ACTION_VTABLE: u32 = 0x010d_0944;
/// The `hkpUnaryAction` constructor (`this`, `(entity, 0)`).
const UNARY_ACTION_CONSTRUCT: u32 = 0x00d2_b260;
/// The memory router getter; its allocator is the word at +0x10.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// An unnamed thiscall `(first, second)` writing a four-lane vector to `this`
/// (`fn_004b4ab0`); the action calls it with the two matrices of the entity at +0xF0 and
/// +0x1C0.
const UNNAMED_004B4AB0: u32 = 0x004b_4ab0;
/// `hkVector4::setMul3` (`this`, `(matrix, vector)`).
const SET_MUL3: u32 = 0x00c7_f750;
/// `hkVector4::setRotatedDir` (`this`, `(matrix, vector)`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkpEntity::activate` (`this`).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The constant 0.0 the length is compared with.
const ZERO: u32 = 0x0101_1d78;
/// The constant 3.0 of the Newton step.
const THREE: u32 = 0x0101_7718;
/// The constant 0.5 of the Newton step.
const HALF: u32 = 0x0101_6248;

// Translated from 00d4d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotorAction::hkpMotorAction` (Xbox PDB): runs the `hkpUnaryAction` constructor on
/// `this` with `(entity, 0)`, then sets the vtable, copies the sixteen-byte `axis` to +0x20,
/// `target` (a float, copied through the x87) to +0x30, `gain` to +0x34 and the byte 1 to
/// +0x38. The axis is then normalized over all four lanes: the length is
/// `(w² + y²) + (z² + x²)`, the factor `0.5 * estimate * (3 - length * estimate²)` from the
/// `RSQRTSS` estimate, and a zero length gives a zero factor. Returns `this`.
pub fn hkp_motor_action_hkp_motor_action(
    e: &mut Engine,
    this: Ptr,
    entity: u32,
    axis: Ptr,
    target: u32,
    gain: u32,
) -> Ptr {
    let base = this.addr();
    e.call(UNARY_ACTION_CONSTRUCT, &args![this, entity, 0u32]);
    e.mem.set_u32(base, MOTOR_ACTION_VTABLE);
    let mut lanes = [0f32; 4];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = e.mem.f32(axis.addr() + 4 * index as u32);
    }
    let zero = e.mem.f32(ZERO);
    let three = e.mem.f32(THREE);
    let half = e.mem.f32(HALF);
    e.mem.set_u32(base + 0x30, x87_quiet_bits(target));
    e.mem.set_u32(base + 0x34, x87_quiet_bits(gain));
    e.mem.set_u8(base + 0x38, 1);
    let squares = lanes.map(|lane| lane * lane);
    let length = (squares[3] + squares[1]) + (squares[2] + squares[0]);
    let estimate = rsqrt_estimate(length);
    let scaled = length * estimate * estimate;
    let factor = half * estimate * (three - scaled);
    let factor = if length == zero { 0.0 } else { factor };
    for (index, lane) in lanes.into_iter().enumerate() {
        e.mem.set_f32(base + 0x20 + 4 * index as u32, factor * lane);
    }
    this
}

// Translated from 00d4d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotorAction::clone` (Xbox PDB): `first_array` and `second_array` are array objects
/// (data pointer at +0, element count at +4; their identities are unconfirmed). Returns 0
/// unless the first holds exactly one element and the second none. Otherwise allocates 0x40
/// bytes from the memory router's allocator (virtual +4), stores 0x40 as the halfword at +4,
/// constructs a motor action on it for the entity word the first element's pointer points to,
/// with this action's axis (+0x20), target (+0x30) and gain (+0x34), copies the byte at +0x38
/// and the word at +0x10 over, and returns it.
pub fn hkp_motor_action_clone(
    e: &mut Engine,
    this: Ptr,
    first_array: Ptr,
    second_array: Ptr,
) -> u32 {
    let base = this.addr();
    if e.mem.u32(first_array.addr() + 4) != 1 || e.mem.u32(second_array.addr() + 4) != 0 {
        return 0;
    }
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let clone = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(clone + 4, 0x40);
    let elements = e.mem.u32(first_array.addr());
    let entity = e.mem.u32(elements);
    let target = x87_quiet_bits(e.mem.u32(base + 0x30));
    let gain = x87_quiet_bits(e.mem.u32(base + 0x34));
    let result = e
        .call(
            0x00d4_d720,
            &args![clone, entity, base + 0x20, target, gain],
        )
        .u32();
    let enabled = e.mem.u8(base + 0x38);
    e.mem.set_u8(result + 0x38, enabled);
    let user_data = e.mem.u32(base + 0x10);
    e.mem.set_u32(result + 0x10, user_data);
    result
}

// Translated from 00d4d860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotorAction::applyAction` (Xbox PDB): does nothing when the enabled byte at +0x38 is
/// zero. Otherwise, with `body` the entity at +0x18: `fn_004b4ab0` writes a vector from the
/// matrices at `body + 0xF0` and `body + 0x1C0`; the speed error is `target - (z·z' +
/// (y·y' + x·x'))` of the axis against that vector, rounded to `float`, times the gain
/// (rounded again); the axis scaled by it is multiplied by the matrix the object at
/// `body + 0xE0` hands out through its virtual +0x14 (`setMul3`) and rotated by `body +
/// 0xF0` (`setRotatedDir`); the entity is activated; and the object's virtual +0x60 is
/// called with the float at +8 of `step_info` and the vector.
pub fn hkp_motor_action_apply_action(e: &mut Engine, this: Ptr, step_info: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x38) == 0 {
        return;
    }
    let body = e.mem.u32(base + 0x18);
    let object = body + 0xe0;
    e.with_stack(0x70, |e, frame| {
        let frame = frame.addr();
        e.call(
            UNNAMED_004B4AB0,
            &args![frame + 0x20, body + 0xf0, body + 0x1c0],
        );
        let axis: [f32; 4] = std::array::from_fn(|lane| e.mem.f32(base + 0x20 + 4 * lane as u32));
        let velocity: [f32; 4] =
            std::array::from_fn(|lane| e.mem.f32(frame + 0x20 + 4 * lane as u32));
        let products = [
            axis[0] * velocity[0],
            axis[1] * velocity[1],
            axis[2] * velocity[2],
        ];
        let along = products[2] + (products[1] + products[0]);
        let target = e.mem.f32(base + 0x30);
        let error = (f64::from(target) - f64::from(along)) as f32;
        let gain = e.mem.f32(base + 0x34);
        let factor = (f64::from(error) * f64::from(gain)) as f32;
        for (lane, component) in axis.into_iter().enumerate() {
            e.mem
                .set_f32(frame + 0x10 + 4 * lane as u32, factor * component);
        }
        e.vcall(object, 0x14, &args![frame + 0x30]);
        e.call(SET_MUL3, &args![frame + 0x10, frame + 0x30, frame + 0x10]);
        e.call(
            SET_ROTATED_DIR,
            &args![frame + 0x10, body + 0xf0, frame + 0x10],
        );
        let time = x87_quiet_bits(e.mem.u32(step_info.addr() + 8));
        e.call(ENTITY_ACTIVATE, &args![body]);
        e.vcall(object, 0x60, &args![time, frame + 0x10]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_d720,
            hkp_motor_action_hkp_motor_action(Ptr, u32, Ptr, u32, u32) -> Ptr
        ),
        entry!(0x00d4_d7f0, hkp_motor_action_clone(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d4_d860, hkp_motor_action_apply_action(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Maps the float constants the normalization reads.
    fn map_constants(e: &mut Engine) {
        for (address, value) in [(ZERO, 0.0f32), (THREE, 3.0), (HALF, 0.5)] {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
    }

    /// Checks `actual` against `expected` to within four ULPs (the `RSQRTSS` estimate differs
    /// between processors and one Newton step leaves a rounding error).
    fn assert_close(actual: f32, expected: f32) {
        let distance = i64::from(actual.to_bits()) - i64::from(expected.to_bits());
        assert!(distance.abs() <= 4, "{actual} against {expected}");
    }

    fn set_vector(e: &mut Engine, address: u32, lanes: [f32; 4]) {
        for (index, lane) in lanes.into_iter().enumerate() {
            e.mem.set_f32(address + 4 * index as u32, lane);
        }
    }

    fn vector(e: &Engine, address: u32) -> [f32; 4] {
        std::array::from_fn(|index| e.mem.f32(address + 4 * index as u32))
    }

    fn construct(axis: [f32; 4]) -> (Engine, u32) {
        let mut e = Engine::new();
        map_constants(&mut e);
        e.register_double(UNARY_ACTION_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        let source = e.mem.alloc(0x10);
        set_vector(&mut e, source, axis);
        let result = e.call(
            0x00d4_d720,
            &args![this, 0x1234u32, source, 0x7f80_0001u32, 2.5f32.to_bits()],
        );
        assert_eq!(result.u32(), this);
        (e, this)
    }

    #[test]
    fn the_constructor_stores_the_fields_and_normalizes_the_axis() {
        let (e, this) = construct([3.0, 0.0, 4.0, 0.0]);
        assert_eq!(e.mem.u32(this), MOTOR_ACTION_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x34), 2.5);
        assert_eq!(e.mem.u8(this + 0x38), 1);
        let axis = vector(&e, this + 0x20);
        assert_close(axis[0], 0.6);
        assert_eq!(axis[1], 0.0);
        assert_close(axis[2], 0.8);
        assert_eq!(axis[3], 0.0);
        let log = e.call_log.as_ref().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|call| call.0 == UNARY_ACTION_CONSTRUCT)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(calls, [vec![this, 0x1234, 0]]);
    }

    #[test]
    fn the_constructor_normalizes_all_four_lanes() {
        let (e, this) = construct([1.0, 1.0, 1.0, 1.0]);
        for lane in vector(&e, this + 0x20) {
            assert_close(lane, 0.5);
        }
    }

    #[test]
    fn the_constructor_makes_a_zero_axis_zero() {
        let (e, this) = construct([0.0, 0.0, 0.0, 0.0]);
        assert_eq!(vector(&e, this + 0x20), [0.0; 4]);
    }

    /// A router whose allocator hands out zeroed 0x40-byte blocks through virtual +4.
    fn install_allocator(e: &mut Engine) {
        let allocator = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(vtable + 4, 0x0900_0002);
        e.register(0x0900_0002, |e, _| Ret {
            eax: e.mem.alloc(0x40),
            ..Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
    }

    /// An array object with one pointer element.
    fn single_array(e: &mut Engine, count: u32, entity: u32) -> u32 {
        let array = e.mem.alloc(8);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(elements, entity);
        e.mem.set_u32(array, elements);
        e.mem.set_u32(array + 4, count);
        array
    }

    #[test]
    fn clone_builds_a_copy_for_the_entity_of_the_first_array() {
        let mut e = Engine::new();
        map_constants(&mut e);
        install_allocator(&mut e);
        e.register_double(UNARY_ACTION_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        set_vector(&mut e, this + 0x20, [0.0, 0.0, 2.0, 0.0]);
        e.mem.set_f32(this + 0x30, 4.0);
        e.mem.set_f32(this + 0x34, 0.25);
        e.mem.set_u8(this + 0x38, 9);
        e.mem.set_u32(this + 0x10, 0xabcd);
        let first = single_array(&mut e, 1, 0x1234);
        let second = single_array(&mut e, 0, 0);
        let copy = e.call(0x00d4_d7f0, &args![this, first, second]).u32();
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u16(copy + 4), 0x40);
        assert_eq!(e.mem.u32(copy), MOTOR_ACTION_VTABLE);
        assert_eq!(e.mem.f32(copy + 0x30), 4.0);
        assert_eq!(e.mem.f32(copy + 0x34), 0.25);
        assert_eq!(e.mem.u8(copy + 0x38), 9);
        assert_eq!(e.mem.u32(copy + 0x10), 0xabcd);
        assert_close(e.mem.f32(copy + 0x28), 1.0);
        let log = e.call_log.as_ref().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|call| call.0 == UNARY_ACTION_CONSTRUCT)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(calls, [vec![copy, 0x1234, 0]]);
    }

    #[test]
    fn clone_returns_zero_unless_the_arrays_hold_one_and_none() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        let this = e.mem.alloc(0x40);
        let two = single_array(&mut e, 2, 1);
        let one = single_array(&mut e, 1, 1);
        let none = single_array(&mut e, 0, 0);
        assert_eq!(e.call(0x00d4_d7f0, &args![this, two, none]).u32(), 0);
        assert_eq!(e.call(0x00d4_d7f0, &args![this, one, one]).u32(), 0);
    }

    /// A body whose object at +0xE0 has virtual +0x14 (writes a diagonal 2, 3, 4 matrix to
    /// its argument) and +0x60 (records its arguments and the vector), an identity rotation at
    /// +0xF0 and `fn_004b4ab0` writing `(3, 5, 7, 0)`.
    fn apply_setup() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x14, 0x0900_0003);
        e.mem.set_u32(vtable + 0x60, 0x0900_0004);
        e.register(0x0900_0003, |e, a| {
            for (row, scale) in [2.0f32, 3.0, 4.0].into_iter().enumerate() {
                for lane in 0..4u32 {
                    let value = if lane == row as u32 { scale } else { 0.0 };
                    e.mem.set_f32(a[1] + 0x10 * row as u32 + 4 * lane, value);
                }
            }
            Ret::default()
        });
        e.register(UNNAMED_004B4AB0, |e, a| {
            for (lane, value) in [3.0f32, 5.0, 7.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register_double(ENTITY_ACTIVATE, |_, _| Ret::default());
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(body + 0xe0, vtable);
        for row in 0..3u32 {
            e.mem.set_f32(body + 0xf0 + 0x10 * row + 4 * row, 1.0);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, body);
        set_vector(&mut e, this + 0x20, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(this + 0x30, 10.0);
        e.mem.set_f32(this + 0x34, 0.5);
        e.mem.set_u8(this + 0x38, 1);
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 8, 0.016);
        e.call_log = Some(vec![]);
        (e, this, body, step)
    }

    #[test]
    fn apply_action_pushes_the_speed_error_through_the_body_matrices() {
        let (mut e, this, body, step) = apply_setup();
        // The axis is read as it is; the vector is captured by the +0x60 double.
        use std::cell::RefCell;
        use std::rc::Rc;
        let seen: Rc<RefCell<Vec<(u32, [f32; 4])>>> = Rc::new(RefCell::new(vec![]));
        let seen_in_double = Rc::clone(&seen);
        e.register_double(0x0900_0004, move |e, a| {
            let lanes = std::array::from_fn(|lane| e.mem.f32(a[2] + 4 * lane as u32));
            seen_in_double.borrow_mut().push((a[1], lanes));
            Ret::default()
        });
        e.call(0x00d4_d860, &args![this, step]);
        // Error 10 - 3 = 7, times the gain 0.5 = 3.5; the axis scaled is (3.5, 0, 0, 0); the
        // diagonal matrix makes it (7, 0, 0, 0) and the identity rotation keeps it.
        assert_eq!(*seen.borrow(), [(0.016f32.to_bits(), [7.0, 0.0, 0.0, 0.0])]);
        let log = e.call_log.as_ref().unwrap();
        let activated: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ENTITY_ACTIVATE)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(activated, [vec![body]]);
    }

    #[test]
    fn apply_action_does_nothing_when_disabled() {
        let (mut e, this, _, step) = apply_setup();
        e.mem.set_u8(this + 0x38, 0);
        e.call(0x00d4_d860, &args![this, step]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|call| call.0 != ENTITY_ACTIVATE));
    }
}
