//! `Havok SDK/hkpreorientaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpReorientAction` (Xbox PDB; vtable `010e6834`, 0x50 bytes, derived from
//! `hkpUnaryAction`, whose entity pointer is at `+0x18`) keeps the axis in the entity's body
//! space at `+0x20`, the axis it should turn towards at `+0x30`, the strength at `+0x40` and
//! the damping at `+0x44`. The entity's rotation quaternion is at `+0x160`, its angular
//! velocity at `+0x1C0` and its motion (an object with a vtable) at `+0xE0`.
//!
//! The SSE code normalises with `RSQRTSS` (the host's estimate, refined once the way the code
//! does); the x87 code quiets a signalling NaN at each load.
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, shufps, store_vec, sum3, zip_with, Vec4,
};

/// `hkpUnaryAction::hkpUnaryAction` (`00d2b260`, `this`, then the entity and a word).
const UNARY_ACTION_CONSTRUCT: u32 = 0x00d2_b260;
/// The memory router (`00c85750`): its `+0x10` is the allocator (slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpEntity::activate` (`00c9c1d0`, `this` = the entity).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `acos` (`00eca1d0`, the argument in ST0 as a leading `f64`).
const ACOS: u32 = 0x00ec_a1d0;
/// The vtable of the action.
const VTABLE: u32 = 0x010e_6834;
/// Four `-0.5` (the `(w * w - 0.5)` term), four `0x7fffffff` (the absolute-value mask), the
/// float `1.0` the dot product is compared with and the float pi.
const MINUS_HALF_VECTOR: u32 = 0x010c_71e0;
const ABS_MASK: u32 = 0x0101_7d10;
const ONE_F32: u32 = 0x0101_2068;
const PI_F32: u32 = 0x0102_b3c8;

// Translated from 00e444f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReorientAction::hkpReorientAction` (Xbox PDB), `RET 0x14`: the base constructor
/// (`hkpUnaryAction`, with the entity and 0), the vtable, the 16 bytes at `axis_a` to `+0x20`,
/// the 16 bytes at `axis_b` to `+0x30` and the two floats (through the x87 stack, so a
/// signalling NaN is quieted) to `+0x40` and `+0x44`. Answers `this`.
pub fn hkp_reorient_action_construct(
    e: &mut Engine,
    this: Ptr,
    entity: u32,
    axis_a: Ptr,
    axis_b: Ptr,
    strength: f32,
    damping: f32,
) -> Ptr {
    let me = this.addr();
    e.call(UNARY_ACTION_CONSTRUCT, &args![this, entity, 0u32]);
    e.mem.set_u32(me, VTABLE);
    copy_vec(e, axis_a.addr(), me + 0x20);
    e.mem.set_f32(me + 0x40, x87_quiet(strength));
    copy_vec(e, axis_b.addr(), me + 0x30);
    e.mem.set_f32(me + 0x44, x87_quiet(damping));
    this
}

// Translated from 00e44540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReorientAction::clone` (Xbox PDB), `RET 8`: answers 0 unless the array at `first` has
/// exactly one entry (`+4` = 1) and the one at `second` none (`+4` = 0). Otherwise a 0x50-byte
/// block comes from the memory router's allocator (slot 4), its size field (`+4`, 16 bits) is
/// set to 0x50, and it is constructed as a copy (the entity is the first word of the first
/// word of `first`; both axes and both floats are this action's) with `+0x10` copied over.
pub fn hkp_reorient_action_clone(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    if e.mem.u32(first.addr() + 4) != 1 || e.mem.u32(second.addr() + 4) != 0 {
        return Ptr::new(0);
    }
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x50u32]).u32();
    e.mem.set_u16(block + 4, 0x50);
    let strength = x87_quiet(e.mem.f32(me + 0x40));
    let damping = x87_quiet(e.mem.f32(me + 0x44));
    let entity_list = e.mem.u32(first.addr());
    let entity = e.mem.u32(entity_list);
    let copy = hkp_reorient_action_construct(
        e,
        Ptr::new(block),
        entity,
        Ptr::new(me + 0x20),
        Ptr::new(me + 0x30),
        strength,
        damping,
    );
    let word = e.mem.u32(me + 0x10);
    e.mem.set_u32(copy.addr() + 0x10, word);
    copy
}

/// Which of the three lanes is the largest, the way the x87 compares pick it: lane 1 beats lane
/// 0 only when strictly greater (a NaN never wins), and lane 2 beats the leader only when
/// strictly greater.
fn largest_lane(v: Vec4) -> usize {
    if v[1] > v[0] {
        if v[2] > v[1] {
            2
        } else {
            1
        }
    } else if v[2] > v[0] {
        2
    } else {
        0
    }
}

/// The quaternion rotation the code computes for `v`: with `c = v x q` (lane 3 is
/// `v3 * q3 - v3 * q3`), `2 * (c * w + ((w * w + k) * v + (v . q) * q))` per lane, `k` being the
/// four floats at `010c71e0` (-0.5) and `v . q` the sum of the first three products, added
/// `z + (y + x)`.
fn rotate_by_quaternion(e: &Engine, q: Vec4, v: Vec4) -> Vec4 {
    let k = load_vec(e, MINUS_HALF_VECTOR);
    let dot = sum3(zip_with(v, q, |a, b| a * b));
    let w = q[3];
    let cross = zip_with(
        zip_with(shufps(v, v, 0xd2), shufps(q, q, 0xc9), |a, b| a * b),
        zip_with(shufps(v, v, 0xc9), shufps(q, q, 0xd2), |a, b| a * b),
        |a, b| a - b,
    );
    std::array::from_fn(|lane| {
        let sum = cross[lane] * w + ((w * w + k[lane]) * v[lane] + dot * q[lane]);
        sum + sum
    })
}

// Translated from 00e445b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReorientAction::applyAction` (Xbox PDB), `RET 4` (the step info; its float at `+0xC` is
/// the time step): turns the entity's body-space axes (`+0x20` and `+0x30`) by its rotation
/// (`R1` and `R0` below), takes the part of `+0x30` perpendicular to `R1` and normalised
/// (`RSQRTSS`, one refinement, 0 when the squared length is 0), the angle between it and `R0`
/// (`acos` of their dot product, or pi / 0 when its absolute value is not below 1.0; a NaN
/// takes the `acos` path), the sign of that angle from the signs of the largest-magnitude
/// lanes of their cross product and of `R1`, then activates the entity and applies, through
/// slot `0x54` of the motion at `entity + 0xE0`, the vector `strength * angle * dt * R1 -
/// damping * dt * angularVelocity` (the products through the x87 stack, in `f64`).
pub fn hkp_reorient_action_apply_action(e: &mut Engine, this: Ptr, info: Ptr) {
    let me = this.addr();
    let entity = e.mem.u32(me + 0x18);
    let target = load_vec(e, me + 0x30);
    let rotation = load_vec(e, entity + 0x160);
    let rotated_target = rotate_by_quaternion(e, rotation, target);
    let axis = load_vec(e, me + 0x20);
    let rotation = load_vec(e, entity + 0x160);
    let rotated_axis = rotate_by_quaternion(e, rotation, axis);
    let target = load_vec(e, me + 0x30);
    let along = sum3(zip_with(rotated_axis, target, |a, b| a * b));
    // `FLD; FCHS; FSTP`: the sign bit flips.
    let negative_along = -along;
    let perpendicular = zip_with(
        zip_with([negative_along; 4], rotated_axis, |a, b| a * b),
        target,
        |a, b| a + b,
    );
    let squared = zip_with(perpendicular, perpendicular, |a, b| a * b);
    let scale = inverse_length(e, sum3(squared));
    let direction = zip_with([scale; 4], perpendicular, |a, b| a * b);
    let dot = sum3(zip_with(direction, rotated_target, |a, b| a * b));
    // `FABS; FSTP; FLD; FCOMP`: below 1.0 or unordered goes to `acos`.
    let one = e.global::<f32>(ONE_F32);
    let mut angle = if !(dot.abs() >= one) {
        e.call(ACOS, &args![f64::from(dot)]).f32()
    } else {
        // `FLDZ; FCOM; TEST AH,5; JP`: pi unless zero is below the dot product (a NaN has
        // already gone to `acos` above).
        let unordered = dot.is_nan();
        let zero_below = 0.0 < dot || unordered;
        if zero_below == unordered {
            x87_quiet(e.global::<f32>(PI_F32))
        } else {
            0.0
        }
    };
    let cross = zip_with(
        zip_with(
            shufps(direction, direction, 0xd2),
            shufps(rotated_target, rotated_target, 0xc9),
            |a, b| a * b,
        ),
        zip_with(
            shufps(direction, direction, 0xc9),
            shufps(rotated_target, rotated_target, 0xd2),
            |a, b| a * b,
        ),
        |a, b| a - b,
    );
    let mask = load_bits(e, ABS_MASK);
    let magnitude = |v: Vec4| -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
    };
    let cross_lane = cross[largest_lane(magnitude(cross))];
    let axis_lane = rotated_axis[largest_lane(magnitude(rotated_axis))];
    if (cross_lane < 0.0) != (axis_lane < 0.0) {
        angle = -angle;
    }
    let time_step = e.mem.f32(info.addr() + 0xc);
    let damping = x87_quiet(e.mem.f32(me + 0x44));
    let damping_step = (f64::from(damping) * f64::from(time_step)) as f32;
    let angular_velocity = load_vec(e, entity + 0x1c0);
    let strength = x87_quiet(e.mem.f32(me + 0x40));
    let turn = ((f64::from(strength) * f64::from(angle)) * f64::from(time_step)) as f32;
    let turn_part = zip_with([turn; 4], rotated_axis, |a, b| a * b);
    let damping_part = zip_with([damping_step; 4], angular_velocity, |a, b| a * b);
    let applied = zip_with(turn_part, damping_part, |a, b| a - b);
    e.call(ENTITY_ACTIVATE, &args![entity]);
    e.with_stack(0x10, |e, vector| {
        store_vec(e, vector.addr(), applied);
        e.vcall(entity + 0xe0, 0x54, &args![vector]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_44f0,
            hkp_reorient_action_construct(Ptr, u32, Ptr, Ptr, f32, f32) -> Ptr
        ),
        entry!(0x00e4_4540, hkp_reorient_action_clone(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00e4_45b0, hkp_reorient_action_apply_action(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot-4 allocator function of the test router, and the motion's slot-`0x54` function.
    const ALLOCATE: u32 = 0x0070_0004;
    const MOTION_APPLY: u32 = 0x0070_0054;

    fn set_constants(e: &mut Engine) {
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        for lane in 0..4 {
            e.set_global(MINUS_HALF_VECTOR + 4 * lane, -0.5f32);
            e.set_global(ABS_MASK + 4 * lane, 0x7fff_ffffu32);
        }
        e.set_global(ONE_F32, 1.0f32);
        e.set_global(PI_F32, std::f32::consts::PI);
    }

    #[test]
    fn construct_copies_the_axes_and_quiets_the_floats_00e444f0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, UNARY_ACTION_CONSTRUCT, 0);
        let this = e.mem.alloc(0x50);
        e.mem.write(this, &[0xcd; 0x50]);
        let [axis_a, axis_b] = [0; 2].map(|_| e.mem.alloc(0x10));
        store_vec(&mut e, axis_a, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, axis_b, [5.0, 6.0, 7.0, 8.0]);
        let entity = 0x1234_5678u32;
        let strength = 0x7f80_0001u32;
        let damping = 0.75f32.to_bits();
        let result = e.call(
            0x00e4_44f0,
            &args![this, entity, axis_a, axis_b, strength, damping],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(
            calls(&log, UNARY_ACTION_CONSTRUCT),
            vec![vec![this, entity, 0]]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(load_vec(&e, this + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.u32(this + 0x40), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x44), 0.75);
        // The base constructor (a double here) owns the rest.
        assert_eq!(e.mem.u32(this + 0x10), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 0x48), 0xcdcd_cdcd);
    }

    /// The memory router with an allocator whose slot 4 answers `block`.
    fn router_double(
        e: &mut Engine,
        log: &crate::units::havok_sdk::hkpbvagent::test_support::Log,
        block: u32,
    ) {
        let allocator_table = e.mem.alloc(0x10);
        e.mem.set_u32(allocator_table + 4, ALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, 0x00c8_5750, router);
        log_double(e, log, ALLOCATE, block);
    }

    #[test]
    fn clone_builds_a_copy_when_the_lists_have_one_and_no_entries_00e44540() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, UNARY_ACTION_CONSTRUCT, 0);
        let block = e.mem.alloc(0x50);
        e.mem.write(block, &[0xcd; 0x50]);
        router_double(&mut e, &log, block);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x10, 0xfeed);
        store_vec(&mut e, this + 0x20, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, this + 0x30, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_f32(this + 0x40, 0.5);
        e.mem.set_u32(this + 0x44, 0x7f80_0003);
        let entity = 0x4545_4545u32;
        let entity_cell = e.mem.alloc(4);
        e.mem.set_u32(entity_cell, entity);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, entity_cell);
        e.mem.set_u32(first + 4, 1);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 4, 0);
        assert_eq!(
            e.call(0x00e4_4540, &args![this, first, second]).u32(),
            block
        );
        assert_eq!(calls(&log, ALLOCATE).len(), 1);
        assert_eq!(calls(&log, ALLOCATE)[0][1..], [0x50]);
        assert_eq!(
            calls(&log, UNARY_ACTION_CONSTRUCT),
            vec![vec![block, entity, 0]]
        );
        assert_eq!(e.mem.u16(block + 4), 0x50);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0x10), 0xfeed);
        assert_eq!(load_vec(&e, block + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, block + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.f32(block + 0x40), 0.5);
        assert_eq!(e.mem.u32(block + 0x44), 0x7fc0_0003);
    }

    #[test]
    fn clone_answers_zero_unless_the_lists_fit_00e44540() {
        for (first_count, second_count) in [(0u32, 0u32), (2, 0), (1, 1), (1, 7)] {
            let mut e = engine();
            let log = new_log();
            router_double(&mut e, &log, 0x1000);
            let this = e.mem.alloc(0x50);
            let first = e.mem.alloc(0x10);
            e.mem.set_u32(first + 4, first_count);
            let second = e.mem.alloc(0x10);
            e.mem.set_u32(second + 4, second_count);
            assert_eq!(e.call(0x00e4_4540, &args![this, first, second]).u32(), 0);
            assert!(calls(&log, 0x00c8_5750).is_empty());
        }
    }

    #[test]
    fn the_largest_lane_follows_the_x87_compares() {
        assert_eq!(largest_lane([1.0, 2.0, 3.0, 0.0]), 2);
        assert_eq!(largest_lane([1.0, 3.0, 2.0, 0.0]), 1);
        assert_eq!(largest_lane([3.0, 1.0, 2.0, 0.0]), 0);
        assert_eq!(largest_lane([3.0, 2.0, 1.0, 0.0]), 0);
        // Ties keep the earlier lane.
        assert_eq!(largest_lane([1.0, 2.0, 2.0, 0.0]), 1);
        assert_eq!(largest_lane([2.0, 2.0, 2.0, 0.0]), 0);
        // A NaN never wins and never loses to the next lane's compare.
        assert_eq!(largest_lane([f32::NAN, 1.0, 2.0, 0.0]), 0);
        assert_eq!(largest_lane([1.0, f32::NAN, 2.0, 0.0]), 2);
        assert_eq!(largest_lane([1.0, 2.0, f32::NAN, 0.0]), 1);
    }

    /// What the motion's slot `0x54` was called with: (activations so far, the motion, the vector).
    type Applied = Rc<RefCell<Vec<(usize, u32, Vec4)>>>;

    /// An entity with the rotation `q` and the angular velocity `angular_velocity`, whose motion
    /// logs the vectors it is given; `ENTITY_ACTIVATE` is a double that logs the entity.
    fn entity(
        e: &mut Engine,
        q: Vec4,
        angular_velocity: Vec4,
    ) -> (u32, Applied, Rc<RefCell<Vec<u32>>>) {
        let entity = e.mem.alloc(0x200);
        store_vec(e, entity + 0x160, q);
        store_vec(e, entity + 0x1c0, angular_velocity);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(table + 0x54, MOTION_APPLY);
        e.mem.set_u32(entity + 0xe0, table);
        let activations = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&activations);
        e.register_double(ENTITY_ACTIVATE, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        let applied: Applied = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&applied);
        let seen = Rc::clone(&activations);
        e.register_double(MOTION_APPLY, move |e, a| {
            let count = seen.borrow().len();
            sink.borrow_mut().push((count, a[0], load_vec(e, a[1])));
            Ret::default()
        });
        (entity, applied, activations)
    }

    /// An action on `entity` with axes `axis` (`+0x20`) and `target` (`+0x30`), strength
    /// and damping.
    fn action(
        e: &mut Engine,
        entity: u32,
        axis: Vec4,
        target: Vec4,
        strength: f32,
        damping: f32,
    ) -> u32 {
        let action = e.mem.alloc(0x50);
        e.mem.set_u32(action + 0x18, entity);
        store_vec(e, action + 0x20, axis);
        store_vec(e, action + 0x30, target);
        e.mem.set_f32(action + 0x40, strength);
        e.mem.set_f32(action + 0x44, damping);
        action
    }

    fn step_info(e: &mut Engine, time_step: f32) -> u32 {
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0xc, time_step);
        info
    }

    fn assert_near(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            let tolerance = 1e-4 * expected[lane].abs().max(1.0);
            assert!(
                (actual[lane] - expected[lane]).abs() <= tolerance,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    const QUARTER_TURN_X: Vec4 = [0.707_106_77, 0.0, 0.0, 0.707_106_77];

    #[test]
    fn a_quarter_turn_flips_the_angle_when_the_cross_product_opposes_the_axis_00e445b0() {
        let mut e = engine();
        set_constants(&mut e);
        let (entity, applied, activations) = entity(&mut e, QUARTER_TURN_X, [1.0, 2.0, 3.0, 0.0]);
        let this = action(
            &mut e,
            entity,
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            2.0,
            0.25,
        );
        let info = step_info(&mut e, 0.5);
        e.call(0x00e4_45b0, &args![this, info]);
        assert_eq!(*activations.borrow(), vec![entity]);
        let applied = applied.borrow();
        assert_eq!(applied.len(), 1);
        // Called on the motion, after the activation.
        assert_eq!((applied[0].0, applied[0].1), (1, entity + 0xe0));
        // The cross product points along -x, the axis along +x: the angle is negated.
        let turn = -2.0 * std::f32::consts::FRAC_PI_2 * 0.5;
        // R1 is (1, 0, 0, 1) (lane 3: 2 * (axis . q) * w).
        assert_near(applied[0].2, [turn - 0.125, -0.25, -0.375, turn]);
    }

    #[test]
    fn a_quarter_turn_keeps_the_angle_when_the_cross_product_follows_the_axis_00e445b0() {
        let mut e = engine();
        set_constants(&mut e);
        let (entity, applied, _) = entity(&mut e, QUARTER_TURN_X, [1.0, 2.0, 3.0, 0.0]);
        let this = action(
            &mut e,
            entity,
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            2.0,
            0.25,
        );
        let info = step_info(&mut e, 0.5);
        e.call(0x00e4_45b0, &args![this, info]);
        // The cross product points along -x like the axis: the angle stays positive.
        let turn = 2.0 * std::f32::consts::FRAC_PI_2 * 0.5;
        // R1 is (-1, 0, 0, -1).
        assert_near(applied.borrow()[0].2, [-turn - 0.125, -0.25, -0.375, -turn]);
    }

    #[test]
    fn a_dot_product_at_or_below_minus_one_gives_pi_00e445b0() {
        let mut e = engine();
        set_constants(&mut e);
        // A half turn about x: the target (0, 1, 1) is carried to (0, -1, -1).
        let (entity, applied, _) = entity(&mut e, [1.0, 0.0, 0.0, 0.0], [0.0; 4]);
        let this = action(
            &mut e,
            entity,
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 1.0, 0.0],
            1.0,
            0.0,
        );
        let info = step_info(&mut e, 1.0);
        e.call(0x00e4_45b0, &args![this, info]);
        let applied = applied.borrow();
        assert_near(applied[0].2, [std::f32::consts::PI, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn a_dot_product_at_or_above_one_gives_no_turn_00e445b0() {
        let mut e = engine();
        set_constants(&mut e);
        let (entity, applied, _) = entity(&mut e, [0.0, 0.0, 0.0, 1.0], [1.0, 2.0, 3.0, 0.0]);
        let this = action(
            &mut e,
            entity,
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            5.0,
            0.5,
        );
        let info = step_info(&mut e, 0.5);
        e.call(0x00e4_45b0, &args![this, info]);
        assert_near(applied.borrow()[0].2, [-0.25, -0.5, -0.75, 0.0]);
    }

    #[test]
    fn a_nan_target_takes_the_acos_path_and_gives_nan_00e445b0() {
        let mut e = engine();
        // The exe's sqrt/acos error path (a NaN or negative argument) sets errno, a word of
        // the thread's data, which the bare engine does not have: doubled.
        let errno = e.mem.alloc(4);
        e.register_double(0x00ec_85e3, move |_, _| errno.into_ret());
        set_constants(&mut e);
        let (entity, applied, _) = entity(&mut e, QUARTER_TURN_X, [0.0; 4]);
        let this = action(
            &mut e,
            entity,
            [1.0, 0.0, 0.0, 0.0],
            [f32::NAN, 0.0, 1.0, 0.0],
            1.0,
            0.0,
        );
        let info = step_info(&mut e, 1.0);
        e.call(0x00e4_45b0, &args![this, info]);
        assert!(applied.borrow()[0].2.iter().all(|lane| lane.is_nan()));
    }

    #[test]
    fn a_target_along_the_rotated_axis_has_no_perpendicular_part_00e445b0() {
        // The perpendicular part is zero: its length is zero, the scale is zero, the
        // direction is zero and the dot product is zero, so the angle is a quarter turn.
        let mut e = engine();
        set_constants(&mut e);
        let (entity, applied, _) = entity(&mut e, [0.0, 0.0, 0.0, 1.0], [0.0; 4]);
        let this = action(
            &mut e,
            entity,
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            2.0,
            0.0,
        );
        let info = step_info(&mut e, 1.0);
        e.call(0x00e4_45b0, &args![this, info]);
        let quarter = std::f32::consts::FRAC_PI_2;
        // R1 = (0, 0, 1, 0); the cross product is zero so the signs agree (both not negative).
        assert_near(applied.borrow()[0].2, [0.0, 0.0, 2.0 * quarter, 0.0]);
    }
}
