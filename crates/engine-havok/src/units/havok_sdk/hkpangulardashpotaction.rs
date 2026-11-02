//! `Havok SDK/hkpangulardashpotaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpAngularDashpotAction` is a binary action (bodies at `+0x18` and `+0x1c`) holding a
//! rotation quaternion at `+0x20`, a strength at `+0x30` and a damping at `+0x34`.

#[allow(unused_imports)]
use crate::prelude::*;
use std::cmp::Ordering;

/// The `hkpBinaryAction` constructor (`00cf4d80`, `this`, `(body a, body b, user data)`).
const BINARY_ACTION_CONSTRUCT: u32 = 0x00cf_4d80;
/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkQuaternion::setMul` (`00c66320`, `this` = result, `(a, b)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// The quaternion angle (`00c74c20`, `this` = the quaternion, result in `st0`); the name is the
/// one the crate already uses for this address.
const QUATERNION_GET_ANGLE: u32 = 0x00c7_4c20;
/// `hkpEntity::activate` (`00c9c1d0`, `this` = the entity).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The `float` the constructor stores as strength and the one it stores as damping.
const DEFAULT_STRENGTH: u32 = 0x0101_e2bc;
const DEFAULT_DAMPING: u32 = 0x0101_3ea4;
/// The 16 bytes the constructor copies to the rotation quaternion.
const DEFAULT_ROTATION: u32 = 0x010c_71b0;
/// The `double` the step time is scaled by (200.0).
const TIME_SCALE: u32 = 0x0101_79e0;
/// The angle below which the axis term is left out (a `float`).
const ANGLE_THRESHOLD: u32 = 0x0101_7d00;
/// The `XORPS` masks: the sign flips of the first three lanes (the conjugate) and of all
/// four (the opposite impulse).
const CONJUGATE_MASK: u32 = 0x010c_4c00;
const NEGATE_MASK: u32 = 0x010c_44d0;
/// The vtable the constructor installs.
const ANGULAR_DASHPOT_VTABLE: u32 = 0x010d_09f4;
/// Bytes of one action.
const ACTION_SIZE: u32 = 0x40;

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// `XORPS` of the four lanes at `addr` with the four mask words at `mask`.
fn flip_signs(e: &mut Engine, addr: u32, mask: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(addr + 4 * lane) ^ e.mem.u32(mask + 4 * lane);
        e.mem.set_u32(addr + 4 * lane, word);
    }
}

// Translated from 00d4dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAngularDashpotAction::hkpAngularDashpotAction` (named in the engine map): runs the
/// binary action constructor, installs the vtable (`010d09f4`) and sets the rotation quaternion
/// (16 bytes at `+0x20`), strength (`+0x30`) and damping (`+0x34`) to the exe's defaults (the
/// two floats are copied through the x87). Returns `this`.
pub fn hkp_angular_dashpot_action_construct(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    user_data: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        BINARY_ACTION_CONSTRUCT,
        &args![this, body_a, body_b, user_data],
    );
    let strength = x87_quiet(e.mem.f32(DEFAULT_STRENGTH));
    e.mem.set_f32(base + 0x30, strength);
    e.mem.set_u32(base, ANGULAR_DASHPOT_VTABLE);
    let damping = x87_quiet(e.mem.f32(DEFAULT_DAMPING));
    e.mem.set_f32(base + 0x34, damping);
    copy_vec(e, DEFAULT_ROTATION, base + 0x20);
    this
}

// Translated from 00d4dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAngularDashpotAction::clone` (named in the engine map): only when `bodies` holds two
/// entries (its size, `+4`, is 2) and the word at `+4` of `other` is zero; otherwise returns
/// null. Takes `0x40` bytes from the memory router's allocator (slot 4), stores the size in the
/// 16-bit word at `+4`, constructs the copy from the two bodies in `bodies` (an array of two
/// pointers) and this action's user data (`+0x10`), then copies the rotation (16 bytes),
/// strength and damping (both through the x87). Returns the copy.
pub fn hkp_angular_dashpot_action_clone(e: &mut Engine, this: Ptr, bodies: u32, other: u32) -> Ptr {
    if e.mem.u32(bodies + 4) != 2 || e.mem.u32(other + 4) != 0 {
        return Ptr::new(0);
    }
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![ACTION_SIZE]).u32();
    e.mem.set_u16(block + 4, ACTION_SIZE as u16);
    let array = e.mem.u32(bodies);
    let (body_a, body_b) = (e.mem.u32(array), e.mem.u32(array + 4));
    let user_data = e.mem.u32(base + 0x10);
    let copy = hkp_angular_dashpot_action_construct(e, Ptr::new(block), body_a, body_b, user_data);
    copy_vec(e, base + 0x20, block + 0x20);
    let strength = x87_quiet(e.mem.f32(base + 0x30));
    e.mem.set_f32(block + 0x30, strength);
    let damping = x87_quiet(e.mem.f32(base + 0x34));
    e.mem.set_f32(block + 0x34, damping);
    copy
}

// Translated from 00d4dcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAngularDashpotAction::applyAction` (named in the engine map): `step_info + 8` (the step
/// time) is scaled by 200.0 and stored as a `float`. The product of the second body's rotation
/// (`+0x160`) and the action's quaternion, conjugated (the first three lanes' signs flipped),
/// is multiplied with the first body's rotation (`quaternion = a.rotation * conjugate(b.rotation
/// * q)`); the difference of the bodies' vectors at `+0x1c0` (all four lanes) and the
/// quaternion's angle (`00c74c20`) give the impulse
/// `damping * scale * difference + strength * scale * angle * quaternion`, where the angle term is
/// left out (zero) while `0.001 >= angle` holds for an ordered angle (a NaN angle keeps it).
/// The impulse goes to the second body (activated first, then its motion's slot `0x54`, at
/// `+0xe0`) and, with all four signs flipped, to the first body.
pub fn hkp_angular_dashpot_action_apply_action(e: &mut Engine, this: Ptr, step_info: u32) {
    let base = this.addr();
    let scale = (f64::from(e.mem.f32(step_info + 8)) * e.mem.f64(TIME_SCALE)) as f32;
    let body_b = e.mem.u32(base + 0x1c);
    let body_a = e.mem.u32(base + 0x18);
    e.with_stack(0x30, |e, locals| {
        let product = locals.addr();
        let difference = product + 0x10;
        let impulse = product + 0x20;
        e.call(
            QUATERNION_SET_MUL,
            &args![product, body_b + 0x160, base + 0x20],
        );
        copy_vec(e, product, difference);
        flip_signs(e, difference, CONJUGATE_MASK);
        e.call(
            QUATERNION_SET_MUL,
            &args![product, body_a + 0x160, difference],
        );
        for lane in 0..4 {
            let offset = 4 * lane;
            let value = e.mem.f32(body_a + 0x1c0 + offset) - e.mem.f32(body_b + 0x1c0 + offset);
            e.mem.set_f32(difference + offset, value);
        }
        let angle = e.call(QUATERNION_GET_ANGLE, &args![product]).st0 as f32;
        let mut axis_term = [0.0f32; 4];
        let threshold = e.mem.f32(ANGLE_THRESHOLD);
        if !matches!(
            threshold.partial_cmp(&angle),
            Some(Ordering::Greater | Ordering::Equal)
        ) {
            for (lane, term) in axis_term.iter_mut().enumerate() {
                *term = angle * e.mem.f32(product + 4 * lane as u32);
            }
        }
        let strength_term = (f64::from(e.mem.f32(base + 0x30)) * f64::from(scale)) as f32;
        let damping_term = (f64::from(scale) * f64::from(e.mem.f32(base + 0x34))) as f32;
        for (lane, term) in axis_term.iter().enumerate() {
            let offset = 4 * lane as u32;
            let value = damping_term * e.mem.f32(difference + offset) + strength_term * *term;
            e.mem.set_f32(impulse + offset, value);
        }
        e.call(ENTITY_ACTIVATE, &args![body_b]);
        e.vcall(body_b + 0xe0, 0x54, &args![impulse]);
        flip_signs(e, impulse, NEGATE_MASK);
        e.call(ENTITY_ACTIVATE, &args![body_a]);
        e.vcall(body_a + 0xe0, 0x54, &args![impulse]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_dbe0,
            hkp_angular_dashpot_action_construct(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00d4_dc30,
            hkp_angular_dashpot_action_clone(Ptr, u32, u32) -> Ptr
        ),
        entry!(
            0x00d4_dcb0,
            hkp_angular_dashpot_action_apply_action(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    type Operands = Rc<RefCell<Vec<[f32; 4]>>>;

    const ALLOCATE_DOUBLE: u32 = 0x0900_0010;
    const IMPULSE_DOUBLE: u32 = 0x0900_0020;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *value);
        }
    }

    fn floats(e: &Engine, addr: u32) -> [f32; 4] {
        std::array::from_fn(|i| e.mem.f32(addr + 4 * i as u32))
    }

    #[test]
    fn construct_installs_the_defaults() {
        let mut e = engine();
        e.register_double(BINARY_ACTION_CONSTRUCT, |_, _| Ret::default());
        e.mem.set_u32(DEFAULT_STRENGTH, 0x7fa0_0000);
        e.mem.set_f32(DEFAULT_DAMPING, 0.02);
        for (i, word) in [1u32, 2, 0x7fa0_0003, 4].into_iter().enumerate() {
            e.mem.set_u32(DEFAULT_ROTATION + 4 * i as u32, word);
        }
        let action = e.mem.alloc(0x40);
        let result = e.call(0x00d4_dbe0, &args![action, 11u32, 22u32, 33u32]);
        assert_eq!(result.u32(), action);
        assert_eq!(
            calls(&e, BINARY_ACTION_CONSTRUCT),
            vec![vec![action, 11, 22, 33]]
        );
        assert_eq!(e.mem.u32(action), ANGULAR_DASHPOT_VTABLE);
        assert_eq!(e.mem.u32(action + 0x30), 0x7fe0_0000);
        assert_eq!(e.mem.f32(action + 0x34), 0.02);
        let rotation: Vec<u32> = (0..4).map(|i| e.mem.u32(action + 0x20 + 4 * i)).collect();
        assert_eq!(rotation, vec![1, 2, 0x7fa0_0003, 4]);
    }

    /// A router whose allocator slot 4 hands out `block`, and a source action with the given
    /// user data, rotation, strength and damping.
    fn clone_setup(e: &mut Engine, block: u32) -> (u32, u32, u32) {
        e.register_double(BINARY_ACTION_CONSTRUCT, |_, _| Ret::default());
        e.register_double(ALLOCATE_DOUBLE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 4, ALLOCATE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x10, 77);
        set_floats(e, source + 0x20, &[0.1, 0.2, 0.3, 0.4]);
        e.mem.set_u32(source + 0x30, 0x7fa0_0001);
        e.mem.set_f32(source + 0x34, 0.5);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 0x1111);
        e.mem.set_u32(array + 4, 0x2222);
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, array);
        e.mem.set_u32(bodies + 4, 2);
        let other = e.mem.alloc(8);
        (source, bodies, other)
    }

    #[test]
    fn clone_builds_a_copy_from_the_two_bodies() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        let (source, bodies, other) = clone_setup(&mut e, block);
        let result = e.call(0x00d4_dc30, &args![source, bodies, other]);
        assert_eq!(result.u32(), block);
        assert_eq!(e.mem.u16(block + 4), 0x40);
        let takes = calls(&e, ALLOCATE_DOUBLE);
        assert_eq!(takes.len(), 1);
        assert_eq!(takes[0][1], 0x40);
        assert_eq!(
            calls(&e, BINARY_ACTION_CONSTRUCT),
            vec![vec![block, 0x1111, 0x2222, 77]]
        );
        assert_eq!(floats(&e, block + 0x20), [0.1, 0.2, 0.3, 0.4]);
        assert_eq!(e.mem.u32(block + 0x30), 0x7fe0_0001);
        assert_eq!(e.mem.f32(block + 0x34), 0.5);
    }

    #[test]
    fn clone_refuses_other_body_counts_and_a_used_second_argument() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        let (source, bodies, other) = clone_setup(&mut e, block);
        e.mem.set_u32(bodies + 4, 3);
        assert_eq!(e.call(0x00d4_dc30, &args![source, bodies, other]).u32(), 0);
        e.mem.set_u32(bodies + 4, 2);
        e.mem.set_u32(other + 4, 1);
        assert_eq!(e.call(0x00d4_dc30, &args![source, bodies, other]).u32(), 0);
        assert!(calls(&e, ALLOCATE_DOUBLE).is_empty());
        assert!(calls(&e, BINARY_ACTION_CONSTRUCT).is_empty());
    }

    /// Bodies with the motion slot `0x54` double, the exe constants, and an action with
    /// strength 0.5 and damping 0.25. `angle` is what the angle double returns. The set-mul
    /// double writes `(0.5, 1.0, 1.5, 2.0)` for its first call and `(0.25, 0.5, 0.75, 1.0)`
    /// for its second, and records the vector it was given as the second operand.
    fn apply_setup(e: &mut Engine, angle: f64) -> (u32, u32, u32, u32, Operands) {
        let operands = Rc::new(RefCell::new(Vec::new()));
        let recorded = Rc::clone(&operands);
        let mut count = 0;
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            recorded.borrow_mut().push(floats(e, a[2]));
            let values: [f32; 4] = if count == 0 {
                [0.5, 1.0, 1.5, 2.0]
            } else {
                [0.25, 0.5, 0.75, 1.0]
            };
            count += 1;
            set_floats(e, a[0], &values);
            Ret::default()
        });
        e.register_double(QUATERNION_GET_ANGLE, move |_, _| Ret {
            st0: angle,
            ..Ret::default()
        });
        e.register_double(ENTITY_ACTIVATE, |_, _| Ret::default());
        e.register_double(IMPULSE_DOUBLE, |_, _| Ret::default());
        e.mem.set_f64(TIME_SCALE, 200.0);
        e.mem.set_f32(ANGLE_THRESHOLD, 0.001);
        for lane in 0..4 {
            e.mem.set_u32(
                CONJUGATE_MASK + 4 * lane,
                if lane < 3 { 0x8000_0000 } else { 0 },
            );
            e.mem.set_u32(NEGATE_MASK + 4 * lane, 0x8000_0000);
        }
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0x54, IMPULSE_DOUBLE);
        let mut bodies = [0u32; 2];
        for body in &mut bodies {
            *body = e.mem.alloc(0x200);
            e.mem.set_u32(*body + 0xe0, table);
        }
        set_floats(e, bodies[0] + 0x1c0, &[1.0, 2.0, 3.0, 4.0]);
        set_floats(e, bodies[1] + 0x1c0, &[0.5, 0.5, 0.5, 0.5]);
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + 0x18, bodies[0]);
        e.mem.set_u32(action + 0x1c, bodies[1]);
        set_floats(e, action + 0x20, &[9.0, 8.0, 7.0, 6.0]);
        e.mem.set_f32(action + 0x30, 0.5);
        e.mem.set_f32(action + 0x34, 0.25);
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 8, 0.5);
        (action, bodies[0], bodies[1], step, operands)
    }

    #[test]
    fn apply_action_pushes_opposite_impulses_onto_both_bodies() {
        let mut e = engine();
        let (action, first, second, step, operands) = apply_setup(&mut e, 0.5);
        e.call(0x00d4_dcb0, &args![action, step]);
        let multiplies = calls(&e, QUATERNION_SET_MUL);
        assert_eq!(multiplies.len(), 2);
        assert_eq!(multiplies[0][1..], [second + 0x160, action + 0x20]);
        assert_eq!(multiplies[1][1], first + 0x160);
        assert_eq!(multiplies[0][0], multiplies[1][0]);
        // The action's quaternion is passed on as it is; the product comes back conjugated.
        assert_eq!(
            *operands.borrow(),
            vec![[9.0, 8.0, 7.0, 6.0], [-0.5, -1.0, -1.5, 2.0]]
        );
        assert_eq!(
            calls(&e, QUATERNION_GET_ANGLE),
            vec![vec![multiplies[0][0]]]
        );
        // scale 100, difference (0.5, 1.5, 2.5, 3.5), axis term 0.5 * (0.25, 0.5, 0.75, 1.0):
        // 25 * difference + 50 * axis term.
        let pushes = calls(&e, IMPULSE_DOUBLE);
        assert_eq!(pushes.len(), 2);
        assert_eq!(pushes[0][0], second + 0xe0);
        assert_eq!(pushes[1][0], first + 0xe0);
        assert_eq!(pushes[0][1], pushes[1][1]);
        assert_eq!(calls(&e, ENTITY_ACTIVATE), vec![vec![second], vec![first]]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [ENTITY_ACTIVATE, IMPULSE_DOUBLE].contains(a))
            .collect();
        assert_eq!(
            order,
            vec![
                ENTITY_ACTIVATE,
                IMPULSE_DOUBLE,
                ENTITY_ACTIVATE,
                IMPULSE_DOUBLE
            ]
        );
    }

    #[test]
    fn apply_action_impulse_values() {
        let mut e = engine();
        let (action, _, _, step, _) = apply_setup(&mut e, 0.5);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let recorded = Rc::clone(&seen);
        e.register_double(IMPULSE_DOUBLE, move |e, a| {
            recorded.borrow_mut().push(floats(e, a[1]));
            Ret::default()
        });
        e.call(0x00d4_dcb0, &args![action, step]);
        assert_eq!(
            *seen.borrow(),
            vec![[18.75, 50.0, 81.25, 112.5], [-18.75, -50.0, -81.25, -112.5]]
        );
    }

    #[test]
    fn apply_action_leaves_out_the_axis_term_for_a_small_angle_but_not_for_nan() {
        for (angle, small) in [
            (0.001f64, true),
            (0.0005, true),
            (0.01, false),
            (f64::NAN, false),
        ] {
            let mut e = engine();
            let (action, _, _, step, _) = apply_setup(&mut e, angle);
            let seen = Rc::new(RefCell::new(Vec::new()));
            let recorded = Rc::clone(&seen);
            e.register_double(IMPULSE_DOUBLE, move |e, a| {
                recorded.borrow_mut().push(floats(e, a[1]));
                Ret::default()
            });
            e.call(0x00d4_dcb0, &args![action, step]);
            let first = seen.borrow()[0];
            if small {
                // 25 * (0.5, 1.5, 2.5, 3.5) only.
                assert_eq!(first, [12.5, 37.5, 62.5, 87.5], "angle {angle}");
            } else if angle.is_nan() {
                assert!(first.iter().all(|v| v.is_nan()));
            } else {
                assert_ne!(first[0], 12.5);
            }
        }
    }
}
