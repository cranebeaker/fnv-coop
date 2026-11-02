//! `Havok SDK/hkpdashpotaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpDashpotAction` is not in the Xbox PDB, so the layout is read off the code and the
//! meanings are descriptive only (unconfirmed). It is a binary action (the base constructor is
//! `00cf4d80`): `+0x18` and `+0x1c` are the two entities, `+0x20` and `+0x30` a point in each
//! entity's space, `+0x40` and `+0x44` two `float`s (a spring factor and a damping factor), and
//! `+0x50` the impulse the last step applied. An entity keeps its transform at `+0xf0`, its
//! motion at `+0xe0` (a sub-object with a vtable) and the motion's linear velocity at `+0x1b0`.
//! SSE code computes lane by lane in `f32`; x87 code rounds at each `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{load_vec, profile_record, store_vec, zip_with, PROFILE_END};

/// The action's vtable and the one the base constructor (`00cf4d80`) leaves before it.
const VTABLE: u32 = 0x010d_0994;
/// `hkpBinaryAction` constructor (Ghidra name; `this` in `ECX`, three stack words).
const BINARY_ACTION_CONSTRUCTOR: u32 = 0x00cf_4d80;
/// The memory router (`00c85750`): its `+0x10` is the allocator, whose slot `4` allocates.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkVector4::setTransformedPos` (Ghidra name; `this` = out, `(transform, point)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkpEntity::activate` (Ghidra name; `this` in `ECX`).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The `float`s the constructor copies into `+0x40` and `+0x44`.
const DEFAULT_SPRING: u32 = 0x0101_e2bc;
const DEFAULT_DAMPING: u32 = 0x0101_3ea4;
/// The `double` the step time is multiplied by, and the `float` -1.0.
const TIME_SCALE: u32 = 0x010d_09b8;
const MINUS_ONE: u32 = 0x0101_2054;
/// The begin label of `applyAction`'s profiling record.
const PROFILE_BEGIN: u32 = 0x010d_09c0;
/// Slot `0x50` of the motion's vtable (an impulse at a point) and the entity offsets.
const SLOT_APPLY_IMPULSE: u32 = 0x50;
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_TRANSFORM: u32 = 0xf0;
const ENTITY_LINEAR_VELOCITY: u32 = 0x1b0;
/// The size of the action object.
const ACTION_SIZE: u32 = 0x60;

// Translated from 00d4d980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDashpotAction::hkpDashpotAction` (Ghidra name): `ECX` = object, three stack words that
/// go unchanged to the binary action constructor. Then the `float`s at `0101e2bc` and
/// `01013ea4` are copied into `+0x40` and `+0x44`, the vtable is set and both points
/// (`+0x20`, `+0x30`) are zeroed. Returns `this`.
pub fn hkp_dashpot_action_constructor(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) -> Ptr {
    let me = this.addr();
    e.call(
        BINARY_ACTION_CONSTRUCTOR,
        &args![this, first, second, third],
    );
    let spring = e.mem.u32(DEFAULT_SPRING);
    e.mem.set_u32(me + 0x40, x87_quiet_bits(spring));
    e.mem.set_u32(me, VTABLE);
    let damping = e.mem.u32(DEFAULT_DAMPING);
    e.mem.set_u32(me + 0x44, x87_quiet_bits(damping));
    store_vec(e, me + 0x20, [0.0; 4]);
    store_vec(e, me + 0x30, [0.0; 4]);
    this
}

// Translated from 00d4d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDashpotAction::clone` (Ghidra name): `ECX` = action, two stack words, each the address
/// of an array header (data pointer at `+0`, size at `+4`). Returns 0 unless the first array has
/// exactly 2 entries and the second none. Otherwise it allocates 0x60 bytes through the memory
/// router's allocator (slot `4`), stores 0x60 in the 16-bit word at `+4`, builds a new action
/// from the first array's first two entries and the user word at `+0x10`, and copies the points
/// (`+0x20`, `+0x30`), the two `float`s (`+0x40`, `+0x44`) and the impulse (`+0x50`) over.
/// Returns the new action.
pub fn hkp_dashpot_action_clone(e: &mut Engine, this: Ptr, entities: u32, others: u32) -> u32 {
    let me = this.addr();
    if e.mem.u32(entities + 4) != 2 || e.mem.u32(others + 4) != 0 {
        return 0;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![ACTION_SIZE]).u32();
    e.mem.set_u16(block + 4, ACTION_SIZE as u16);
    let data = e.mem.u32(entities);
    let (first, second) = (e.mem.u32(data), e.mem.u32(data + 4));
    let user = e.mem.u32(me + 0x10);
    hkp_dashpot_action_constructor(e, Ptr::new(block), first, second, user);
    for offset in [0x20, 0x30, 0x50] {
        let lanes = load_vec(e, me + offset);
        store_vec(e, block + offset, lanes);
    }
    for offset in [0x40, 0x44] {
        let bits = e.mem.u32(me + offset);
        e.mem.set_u32(block + offset, x87_quiet_bits(bits));
    }
    block
}

// Translated from 00d4da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDashpotAction::applyAction` (Ghidra name): `ECX` = action, one stack word, the step info
/// (`m_deltaTime` at `+8`). Between a begin and an end profiling record: the scaled time `t` is
/// the step time times the `double` at `010d09b8`, rounded to a `float`; both points are moved to
/// world space with `setTransformedPos` (transform at entity `+0xf0`); the impulse stored at
/// `+0x50` is `(damping * t) * (velocity A - velocity B) + (spring * t) * (world A - world B)`
/// (every product and sum a `float`, the vector ones in all four lanes); entity A is activated
/// and its motion's slot `0x50` is called with `-1 * impulse` and world A, then entity B with
/// the impulse and world B.
pub fn hkp_dashpot_action_apply_action(e: &mut Engine, this: Ptr, step_info: Ptr) {
    profile_record(e, PROFILE_BEGIN);
    let me = this.addr();
    let delta = e.mem.f32(step_info.addr() + 8);
    let (entity_a, entity_b) = (e.mem.u32(me + 0x18), e.mem.u32(me + 0x1c));
    let time_scale: f64 = e.global(TIME_SCALE);
    let scaled = (f64::from(delta) * time_scale) as f32;
    e.with_stack(0x30, |e, block| {
        let (world_b, world_a, reaction) = (block.addr(), block.addr() + 0x10, block.addr() + 0x20);
        e.call(
            SET_TRANSFORMED_POS,
            &args![world_a, entity_a + ENTITY_TRANSFORM, me + 0x20],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![world_b, entity_b + ENTITY_TRANSFORM, me + 0x30],
        );
        let spring = e.mem.f32(me + 0x40);
        let spring_step = (f64::from(spring) * f64::from(scaled)) as f32;
        let velocity_a = load_vec(e, entity_a + ENTITY_LINEAR_VELOCITY);
        let velocity_b = load_vec(e, entity_b + ENTITY_LINEAR_VELOCITY);
        let relative_velocity = zip_with(velocity_a, velocity_b, |p, q| p - q);
        let damping = e.mem.f32(me + 0x44);
        let damping_step = (f64::from(scaled) * f64::from(damping)) as f32;
        let separation = zip_with(load_vec(e, world_a), load_vec(e, world_b), |p, q| p - q);
        let pull = separation.map(|lane| spring_step * lane);
        store_vec(e, me + 0x50, pull);
        let drag = relative_velocity.map(|lane| damping_step * lane);
        let impulse = zip_with(drag, pull, |p, q| p + q);
        store_vec(e, me + 0x50, impulse);
        let minus_one: f32 = e.global(MINUS_ONE);
        store_vec(e, reaction, impulse.map(|lane| minus_one * lane));
        e.call(ENTITY_ACTIVATE, &args![entity_a]);
        e.vcall(
            entity_a + ENTITY_MOTION,
            SLOT_APPLY_IMPULSE,
            &args![reaction, world_a],
        );
        e.call(ENTITY_ACTIVATE, &args![entity_b]);
        e.vcall(
            entity_b + ENTITY_MOTION,
            SLOT_APPLY_IMPULSE,
            &args![me + 0x50, world_b],
        );
    });
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4d980,
            hkp_dashpot_action_constructor(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x00d4d9d0, hkp_dashpot_action_clone(Ptr, u32, u32) -> u32),
        entry!(0x00d4da60, hkp_dashpot_action_apply_action(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn constructor_chains_to_the_base_and_sets_the_defaults() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, BINARY_ACTION_CONSTRUCTOR, 0);
        e.mem.set_f32(DEFAULT_SPRING, 0.1);
        e.mem.set_f32(DEFAULT_DAMPING, 0.01);
        let this = e.mem.alloc(0x60);
        for offset in [0x20, 0x30] {
            store_vec(&mut e, this + offset, [1.0; 4]);
        }
        let returned = e
            .call(0x00d4_d980, &args![this, 0x11u32, 0x22u32, 0x33u32])
            .u32();
        assert_eq!(returned, this);
        assert_eq!(
            calls(&log, BINARY_ACTION_CONSTRUCTOR),
            vec![vec![this, 0x11, 0x22, 0x33]]
        );
        assert_eq!(e.mem.u32(this), 0x010d_0994);
        assert_eq!(e.mem.f32(this + 0x40), 0.1);
        assert_eq!(e.mem.f32(this + 0x44), 0.01);
        assert_eq!(load_vec(&e, this + 0x20), [0.0; 4]);
        assert_eq!(load_vec(&e, this + 0x30), [0.0; 4]);
    }

    /// An allocator whose slot `4` logs its call and hands out one fixed block, behind a
    /// memory router double. Returns the allocator.
    fn allocator_giving(e: &mut Engine, log: &Log, block: u32) -> u32 {
        let allocator = logging_object(e, log, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push((0x0900_0004, a.to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        allocator
    }

    #[test]
    fn clone_needs_two_entities_and_no_others() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        let header = e.mem.alloc(0x10);
        let none = e.mem.alloc(0x10);
        e.mem.set_u32(header + 4, 3);
        assert_eq!(e.call(0x00d4_d9d0, &args![this, header, none]).u32(), 0);
        e.mem.set_u32(header + 4, 2);
        e.mem.set_u32(none + 4, 1);
        assert_eq!(e.call(0x00d4_d9d0, &args![this, header, none]).u32(), 0);
    }

    #[test]
    fn clone_builds_a_copy_from_the_new_entities() {
        let mut e = engine();
        let log = new_log();
        let block = e.mem.alloc(0x60);
        let allocator = allocator_giving(&mut e, &log, block);
        log_double(&mut e, &log, BINARY_ACTION_CONSTRUCTOR, 0);
        e.mem.set_f32(DEFAULT_SPRING, 0.1);
        e.mem.set_f32(DEFAULT_DAMPING, 0.01);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x10, 0x77);
        store_vec(&mut e, this + 0x20, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, this + 0x30, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_f32(this + 0x40, 3.5);
        e.mem.set_f32(this + 0x44, 4.5);
        store_vec(&mut e, this + 0x50, [9.0, 10.0, 11.0, 12.0]);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0xa1);
        e.mem.set_u32(data + 4, 0xb2);
        let entities = e.mem.alloc(0x10);
        e.mem.set_u32(entities, data);
        e.mem.set_u32(entities + 4, 2);
        let others = e.mem.alloc(0x10);
        let copy = e.call(0x00d4_d9d0, &args![this, entities, others]).u32();
        assert_eq!(copy, block);
        assert_eq!(calls(&log, 0x0900_0004), vec![vec![allocator, 0x60]]);
        assert_eq!(
            calls(&log, BINARY_ACTION_CONSTRUCTOR),
            vec![vec![block, 0xa1, 0xb2, 0x77]]
        );
        assert_eq!(e.mem.u16(block + 4), 0x60);
        assert_eq!(load_vec(&e, block + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, block + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.f32(block + 0x40), 3.5);
        assert_eq!(e.mem.f32(block + 0x44), 4.5);
        assert_eq!(load_vec(&e, block + 0x50), [9.0, 10.0, 11.0, 12.0]);
    }

    /// What a motion double saw: the motion, then the impulse and the point it was given.
    type Impulses = Rc<RefCell<Vec<(u32, Vec4, Vec4)>>>;

    /// An entity with a transform whose first vector is `(x, 0, 0, 0)`, the given linear
    /// velocity, and a motion at `+0xe0` whose slot `0x50` records into `seen`.
    fn entity(e: &mut Engine, seen: &Impulses, x: f32, velocity: Vec4) -> u32 {
        let entity = e.mem.alloc(0x200);
        store_vec(e, entity + ENTITY_TRANSFORM, [x, 0.0, 0.0, 0.0]);
        store_vec(e, entity + ENTITY_LINEAR_VELOCITY, velocity);
        let vtable = e.mem.alloc(0x60);
        let double = 0x0a00_0000 + entity;
        e.mem.set_u32(vtable + 0x50, double);
        e.mem.set_u32(entity + ENTITY_MOTION, vtable);
        let sink = Rc::clone(seen);
        e.register_double(double, move |e, a| {
            sink.borrow_mut()
                .push((a[0], load_vec(e, a[1]), load_vec(e, a[2])));
            Ret::default()
        });
        entity
    }

    #[test]
    fn apply_action_pushes_the_entities_apart_by_spring_and_damping() {
        let mut e = engine();
        let log = new_log();
        let seen = Impulses::default();
        let (stream, records) = monitor(&mut e, 0x40);
        e.set_global(TIME_SCALE, 150.0f64);
        e.set_global(MINUS_ONE, -1.0f32);
        log_double(&mut e, &log, ENTITY_ACTIVATE, 0);
        let sink = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            sink.borrow_mut().push((SET_TRANSFORMED_POS, a.to_vec()));
            let moved = zip_with(load_vec(e, a[1]), load_vec(e, a[2]), |t, p| t + p);
            store_vec(e, a[0], moved);
            Ret::default()
        });
        let a = entity(&mut e, &seen, 10.0, [1.0, 2.0, 3.0, 4.0]);
        let b = entity(&mut e, &seen, 4.0, [0.5; 4]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x18, a);
        e.mem.set_u32(this + 0x1c, b);
        store_vec(&mut e, this + 0x20, [1.0, 1.0, 1.0, 0.0]);
        store_vec(&mut e, this + 0x30, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(this + 0x40, 2.0);
        e.mem.set_f32(this + 0x44, 4.0);
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 8, 0.5);
        e.call(0x00d4_da60, &args![this, step]);
        let impulse = [1050.0, 600.0, 900.0, 1050.0];
        assert_eq!(load_vec(&e, this + 0x50), impulse);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(
            seen[0],
            (
                a + ENTITY_MOTION,
                impulse.map(|lane| -lane),
                [11.0, 1.0, 1.0, 0.0]
            )
        );
        assert_eq!(seen[1], (b + ENTITY_MOTION, impulse, [5.0, 0.0, 0.0, 0.0]));
        assert_eq!(calls(&log, ENTITY_ACTIVATE), vec![vec![a], vec![b]]);
        assert_eq!(calls(&log, SET_TRANSFORMED_POS).len(), 2);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![PROFILE_BEGIN, PROFILE_END]
        );
    }
}
