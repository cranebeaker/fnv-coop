//! `Havok SDK/hkpcollisioncallbackutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both callbacks are profiled (`RDTSC` time stamps from the host clock) and are otherwise
//! complete: neither has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// Profiling labels: `"TtCollLtUtil"` (added), `"TtCollLfUtil"` (removed) and the shared end
/// label.
const LABEL_ADDED: u32 = 0x010c_d2e8;
const LABEL_REMOVED: u32 = 0x010c_d2f8;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpSimpleContactConstraintData::getSimpleConstraintContactMgr` (`this` = the constraint
/// data, no arguments).
const GET_CONTACT_MANAGER: u32 = 0x00d9_2820;
/// `hkpWorldCallbackUtil::fireContactConstraintAddedCallback` and
/// `hkpEntityCallbackUtil::fireContactConstraintAddedCallback` (cdecl: listener owner,
/// event).
const WORLD_FIRE_ADDED: u32 = 0x00d0_0d40;
const ENTITY_FIRE_ADDED: u32 = 0x00cf_ffb0;
/// The same for `fireContactConstraintRemovedCallback`.
const WORLD_FIRE_REMOVED: u32 = 0x00d0_0de0;
const ENTITY_FIRE_REMOVED: u32 = 0x00d0_0060;

/// The body of both callbacks: a profile record with `label`; when the constraint data
/// (`[constraint + 0xC]`) answers type `0xB` in its slot `0x20`, builds the 16-byte event
/// `(kind, entity A, entity B, contact manager)` for the two entities (`+0x14`, `+0x18`) and
/// the contact manager of the data, and fires it to the world of entity A (`[A + 8]`) with
/// kind 2, to entity A with kind 0 and to entity B with kind 1 (one buffer, its first word
/// rewritten between the calls); then the end record.
fn fire_callbacks(e: &mut Engine, constraint: Ptr, label: u32, world_fire: u32, entity_fire: u32) {
    profile_record(e, label);
    let data = e.mem.u32(constraint.addr() + 0xc);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    if kind == 0xb {
        let manager = e.call(GET_CONTACT_MANAGER, &args![data]).u32();
        let entity_a = e.mem.u32(constraint.addr() + 0x14);
        let entity_b = e.mem.u32(constraint.addr() + 0x18);
        e.with_stack(0x10, |e, event| {
            e.mem.set_u32(event.addr(), 2);
            e.mem.set_u32(event.addr() + 4, entity_a);
            e.mem.set_u32(event.addr() + 8, entity_b);
            e.mem.set_u32(event.addr() + 0xc, manager);
            let world = e.mem.u32(entity_a + 8);
            e.call(world_fire, &args![world, event]);
            e.mem.set_u32(event.addr(), 0);
            e.call(entity_fire, &args![entity_a, event]);
            e.mem.set_u32(event.addr(), 1);
            e.call(entity_fire, &args![entity_b, event]);
        });
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d0ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionCallbackUtil::constraintAddedCallback` (Xbox PDB): `constraint` is the
/// constraint instance. For a contact constraint (data type `0xB`) fires the contact
/// constraint added callback to the world and to both entities (see `fire_callbacks`).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_collision_callback_util_constraint_added_callback(
    e: &mut Engine,
    _this: u32,
    constraint: Ptr,
) {
    fire_callbacks(
        e,
        constraint,
        LABEL_ADDED,
        WORLD_FIRE_ADDED,
        ENTITY_FIRE_ADDED,
    );
}

// Translated from 00d0af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionCallbackUtil::constraintRemovedCallback` (Xbox PDB): as the added callback,
/// with the contact constraint removed callbacks.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_collision_callback_util_constraint_removed_callback(
    e: &mut Engine,
    _this: u32,
    constraint: Ptr,
) {
    fire_callbacks(
        e,
        constraint,
        LABEL_REMOVED,
        WORLD_FIRE_REMOVED,
        ENTITY_FIRE_REMOVED,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d0_ae80,
            hkp_collision_callback_util_constraint_added_callback(u32, Ptr)
        ),
        entry!(
            0x00d0_af60,
            hkp_collision_callback_util_constraint_removed_callback(u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    const TYPE_DOUBLE: u32 = 0x0900_0020;
    /// The events seen by the fire doubles: (callee, owner, the four event words).
    type Events = Rc<RefCell<Vec<(u32, u32, Vec<u32>)>>>;

    struct Scene {
        e: Engine,
        log: Log,
        events: Events,
        constraint: u32,
        entity_a: u32,
        entity_b: u32,
        stream: u32,
        records: u32,
    }

    /// A constraint whose data answers `kind`, between two entities (the first in a world),
    /// with doubles over the contact manager getter (answering `0x3333`) and the four fire
    /// functions that note the event.
    fn scene(kind: u32) -> Scene {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let world = e.mem.alloc(0x20);
        let entity_a = e.mem.alloc(0x20);
        e.mem.set_u32(entity_a + 8, world);
        let entity_b = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, TYPE_DOUBLE);
        log_double(&mut e, &log, TYPE_DOUBLE, kind);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, vtable);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_u32(constraint + 0x14, entity_a);
        e.mem.set_u32(constraint + 0x18, entity_b);
        log_double(&mut e, &log, GET_CONTACT_MANAGER, 0x3333);
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        for callee in [
            WORLD_FIRE_ADDED,
            ENTITY_FIRE_ADDED,
            WORLD_FIRE_REMOVED,
            ENTITY_FIRE_REMOVED,
        ] {
            let seen = Rc::clone(&events);
            e.register_double(callee, move |e, a| {
                let words = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
                seen.borrow_mut().push((callee, a[0], words));
                Ret::default()
            });
        }
        Scene {
            e,
            log,
            events,
            constraint,
            entity_a,
            entity_b,
            stream,
            records,
        }
    }

    /// The labels of the profile records written since `records`.
    fn labels(s: &Scene) -> Vec<u32> {
        let end = s.e.mem.u32(s.stream + 4);
        (s.records..end)
            .step_by(12)
            .map(|record| s.e.mem.u32(record))
            .collect()
    }

    #[test]
    fn contact_constraint_fires_world_then_both_entities_00d0ae80() {
        let mut s = scene(0xb);
        s.e.call(0x00d0_ae80, &[VCALL_THIS, s.constraint]);
        let world = s.e.mem.u32(s.entity_a + 8);
        assert_eq!(
            *s.events.borrow(),
            vec![
                (
                    WORLD_FIRE_ADDED,
                    world,
                    vec![2, s.entity_a, s.entity_b, 0x3333]
                ),
                (
                    ENTITY_FIRE_ADDED,
                    s.entity_a,
                    vec![0, s.entity_a, s.entity_b, 0x3333]
                ),
                (
                    ENTITY_FIRE_ADDED,
                    s.entity_b,
                    vec![1, s.entity_a, s.entity_b, 0x3333]
                ),
            ]
        );
        assert_eq!(labels(&s), vec![LABEL_ADDED, LABEL_END]);
        assert_eq!(calls(&s.log, GET_CONTACT_MANAGER).len(), 1);
    }

    #[test]
    fn other_constraint_types_only_profile_00d0ae80() {
        let mut s = scene(0xa);
        s.e.call(0x00d0_ae80, &[VCALL_THIS, s.constraint]);
        assert!(s.events.borrow().is_empty());
        assert!(calls(&s.log, GET_CONTACT_MANAGER).is_empty());
        assert_eq!(labels(&s), vec![LABEL_ADDED, LABEL_END]);
    }

    #[test]
    fn removed_contact_constraint_fires_the_removed_callbacks_00d0af60() {
        let mut s = scene(0xb);
        s.e.call(0x00d0_af60, &[VCALL_THIS, s.constraint]);
        let world = s.e.mem.u32(s.entity_a + 8);
        assert_eq!(
            *s.events.borrow(),
            vec![
                (
                    WORLD_FIRE_REMOVED,
                    world,
                    vec![2, s.entity_a, s.entity_b, 0x3333]
                ),
                (
                    ENTITY_FIRE_REMOVED,
                    s.entity_a,
                    vec![0, s.entity_a, s.entity_b, 0x3333]
                ),
                (
                    ENTITY_FIRE_REMOVED,
                    s.entity_b,
                    vec![1, s.entity_a, s.entity_b, 0x3333]
                ),
            ]
        );
        assert_eq!(labels(&s), vec![LABEL_REMOVED, LABEL_END]);
    }

    #[test]
    fn removed_callback_ignores_other_types_00d0af60() {
        let mut s = scene(0);
        s.e.call(0x00d0_af60, &[VCALL_THIS, s.constraint]);
        assert!(s.events.borrow().is_empty());
        assert_eq!(labels(&s), vec![LABEL_REMOVED, LABEL_END]);
    }
}
