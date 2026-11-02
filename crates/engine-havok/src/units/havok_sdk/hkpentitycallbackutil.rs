//! `Havok SDK/hkpentitycallbackutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every function takes the `hkpEntity` as its only (cdecl) argument. An entity keeps its
//! `hkpEntityListener`s in `m_extendedListeners` (+0x214, Xbox PDB `hkpEntity::
//! ExtendedListeners`), whose `m_entityListeners` `hkSmallArray` has its data pointer at +8
//! and its 16-bit size at +0xC; the listeners are called from the last to the first, null
//! ones skipped. The listener callbacks are vtable slots `0x04` (`entityAddedCallback`),
//! `0x08` (`entityRemovedCallback`) and `0x10` (`entitySetMotionTypeCallback`, Xbox PDB
//! `hkpEntityListener`).
//!
//! Each callback is wrapped in Havok's profiling records (`RDTSC` time stamps appended to the
//! thread's monitor stream, found with `TlsGetValue` of the index at `0126835c`); the engine
//! has no model of `RDTSC`, so the host clock stands in for the time stamps.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtentAddCb"`, `"TtentRemCb"`, `"TtsetMotionTypeCb"` (the `Tt` is the
/// record type) and the end label.
const LABEL_ADDED: u32 = 0x010c_c9dc;
const LABEL_REMOVED: u32 = 0x010c_c9e8;
const LABEL_SET_MOTION_TYPE: u32 = 0x010c_ca04;
const LABEL_END: u32 = 0x010c_41f8;
/// Profiling labels of the functions below (`"TtsetShapeCb"`, `"TtentDelCb"`, `"TtcpCb"`,
/// `"TtNewCollCb"`, `"TtFinCollCb"`, `"TtcpAddCb"`, `"TtcpRemCb"`, `"TtcpProCb"`).
const LABEL_SHAPE_SET: u32 = 0x010c_c9f4;
const LABEL_DELETED: u32 = 0x010c_ca18;
const LABEL_CONTACT_POINT: u32 = 0x010c_ca24;
const LABEL_CONSTRAINT_ADDED: u32 = 0x010c_ca2c;
const LABEL_CONSTRAINT_REMOVED: u32 = 0x010c_ca38;
const LABEL_POINT_ADDED: u32 = 0x010c_ca44;
const LABEL_POINT_REMOVED: u32 = 0x010c_ca50;
const LABEL_PROCESS: u32 = 0x010c_ca5c;
/// `hkpEntity::sortConstraintsSlavesDeterministically` (Xbox PDB, `00c9c860`, `this` only).
const SORT_CONSTRAINTS_SLAVES: u32 = 0x00c9_c860;

layout! {
    /// `hkpEntity` (Xbox PDB), as far as these functions read it (PC offsets).
    pub struct HkpEntityCallbacks: 0x220 {
        /// `m_constraintsMaster` (Xbox PDB, `hkSmallArray<hkConstraintInternal>`): data. The
        /// first word of an element is its `hkpConstraintInstance*`.
        0x0ac m_constraintsMaster: Ptr,
        /// Size of `m_constraintsMaster` (16 bits).
        0x0b0 m_constraintsMasterSize: u16,
        /// `m_constraintsSlave` (Xbox PDB, `hkArray<hkpConstraintInstance*>`): data.
        0x0b4 m_constraintsSlave: Ptr,
        /// Size of `m_constraintsSlave`.
        0x0b8 m_constraintsSlaveSize: i32,
        /// `m_actions` (Xbox PDB, `hkSmallArray<hkpAction*>`): data.
        0x208 m_actions: Ptr,
        /// Size of `m_actions` (16 bits).
        0x20c m_actionsSize: u16,
        /// `m_extendedListeners` (Xbox PDB): `ExtendedListeners*` or null.
        0x214 m_extendedListeners: Ptr,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cff9b0, cleanup_null_pointers(Ptr)),
        entry!(
            0x00cffcf0,
            hkp_entity_callback_util_fire_entity_shape_set(Ptr<HkpEntityCallbacks>)
        ),
        entry!(
            0x00cffe50,
            hkp_entity_callback_util_fire_entity_deleted(Ptr<HkpEntityCallbacks>)
        ),
        entry!(
            0x00cfff00,
            hkp_entity_callback_util_fire_contact_point_callback_internal(
                Ptr<HkpEntityCallbacks>,
                u32,
            )
        ),
        entry!(
            0x00cfffb0,
            hkp_entity_callback_util_fire_contact_constraint_added_callback(
                Ptr<HkpEntityCallbacks>,
                u32,
            )
        ),
        entry!(
            0x00d00060,
            hkp_entity_callback_util_fire_contact_constraint_removed_callback(
                Ptr<HkpEntityCallbacks>,
                u32,
            )
        ),
        entry!(
            0x00d00110,
            hkp_entity_callback_util_fire_contact_point_added_internal(
                Ptr<HkpEntityCallbacks>,
                u32,
            )
        ),
        entry!(
            0x00d001c0,
            hkp_entity_callback_util_fire_contact_point_removed_internal(
                Ptr<HkpEntityCallbacks>,
                u32,
            )
        ),
        entry!(
            0x00d00270,
            hkp_entity_callback_util_fire_contact_process_internal(Ptr<HkpEntityCallbacks>, u32)
        ),
        entry!(
            0x00cffa00,
            hkp_entity_callback_util_fire_entity_added(Ptr<HkpEntityCallbacks>)
        ),
        entry!(
            0x00cffab0,
            hkp_entity_callback_util_fire_entity_removed(Ptr<HkpEntityCallbacks>)
        ),
        entry!(
            0x00cffda0,
            hkp_entity_callback_util_fire_entity_set_motion_type(Ptr<HkpEntityCallbacks>)
        ),
    ]
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Calls vtable slot `slot` of every entity listener, last first, each between a record
/// `label` and an end record.
fn fire_listeners(e: &mut Engine, entity: Ptr<HkpEntityCallbacks>, label: u32, slot: u32) {
    let extended = e
        .get(entity, HkpEntityCallbacks::m_extendedListeners)
        .addr();
    if extended == 0 {
        return;
    }
    let mut index = i32::from(e.mem.u16(extended + 0xc)) - 1;
    while index >= 0 {
        let data = e.mem.u32(extended + 8);
        let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if listener != 0 {
            profile_record(e, label);
            e.vcall(listener, slot, &args![entity]);
            profile_record(e, LABEL_END);
        }
        index -= 1;
    }
}

/// One element of a list of objects that all get the same callback while the list is not
/// empty: each call is expected to remove itself from the list. `first` reads the object
/// at the start of the list, `pending` says whether the list still has any.
fn drain_callbacks(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    slot: u32,
    pending: impl Fn(&Engine) -> bool,
    first: impl Fn(&Engine) -> u32,
) {
    while pending(e) {
        profile_record(e, LABEL_REMOVED);
        let object = first(e);
        e.vcall(object, slot, &args![entity]);
        profile_record(e, LABEL_END);
    }
}

// Translated from 00cff9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cleanupNullPointers` (Xbox PDB): takes the `hkSmallArray<T*>` (data +0, 16-bit size +4) in
/// the register `EAX` (here the argument `array`; no stack argument). Walking from the last
/// entry to the first, every null entry shrinks the size by one and the entries after it are
/// moved down by one place, so the array ends without nulls (entries past the new size keep
/// stale values).
pub fn cleanup_null_pointers(e: &mut Engine, array: Ptr) {
    let array = array.addr();
    let mut index = i32::from(e.mem.u16(array + 4)) - 1;
    while index >= 0 {
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == 0 {
            let size = e.mem.u16(array + 4);
            e.mem.set_u16(array + 4, size.wrapping_add(0xffff));
            let mut at = index;
            if index < i32::from(e.mem.u16(array + 4)) {
                loop {
                    let data = e.mem.u32(array);
                    let place = data.wrapping_add((at as u32).wrapping_mul(4));
                    let next = e.mem.u32(place.wrapping_add(4));
                    e.mem.set_u32(place, next);
                    at += 1;
                    if at >= i32::from(e.mem.u16(array + 4)) {
                        break;
                    }
                }
            }
        }
        index -= 1;
    }
}

/// Calls vtable slot `slot` (with `argument`) of every listener of the small array at
/// `entity + 0x200` (the collision listeners; data +0, 16-bit size +4), last first, null ones
/// skipped, each between a profiling record `label` and an end record; then removes the nulls
/// (`cleanupNullPointers`).
fn fire_collision_listeners(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    label: u32,
    slot: u32,
    argument: u32,
) {
    let array = entity.addr() + 0x200;
    let mut index = i32::from(e.mem.u16(array + 4)) - 1;
    while index >= 0 {
        let at = (index as u32).wrapping_mul(4);
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add(at)) != 0 {
            profile_record(e, label);
            let data = e.mem.u32(array);
            let listener = e.mem.u32(data.wrapping_add(at));
            e.vcall(listener, slot, &args![argument]);
            profile_record(e, LABEL_END);
        }
        index -= 1;
    }
    cleanup_null_pointers(e, Ptr::new(array));
}

// Translated from 00cffcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntityShapeSet` (Xbox PDB): `entityShapeSetCallback` (slot
/// `0x0c`; name unconfirmed) of every entity listener.
pub fn hkp_entity_callback_util_fire_entity_shape_set(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
) {
    fire_listeners(e, entity, LABEL_SHAPE_SET, 0xc);
}

// Translated from 00cffe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntityDeleted` (Xbox PDB): `entityDeletedCallback` (slot `0x14`;
/// name unconfirmed) of every entity listener.
pub fn hkp_entity_callback_util_fire_entity_deleted(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
) {
    fire_listeners(e, entity, LABEL_DELETED, 0x14);
}

// Translated from 00cfff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireContactPointCallbackInternal` (Xbox PDB): slot `0x00` of every
/// collision listener of the entity, with `event`, then `cleanupNullPointers`.
pub fn hkp_entity_callback_util_fire_contact_point_callback_internal(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    event: u32,
) {
    fire_collision_listeners(e, entity, LABEL_CONTACT_POINT, 0x0, event);
}

// Translated from 00cfffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireContactConstraintAddedCallback` (Xbox PDB): slot `0x04` of every
/// collision listener of the entity, with `event`, then `cleanupNullPointers`.
pub fn hkp_entity_callback_util_fire_contact_constraint_added_callback(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    event: u32,
) {
    fire_collision_listeners(e, entity, LABEL_CONSTRAINT_ADDED, 0x4, event);
}

// Translated from 00d00060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireContactConstraintRemovedCallback` (Xbox PDB): slot `0x08` of
/// every collision listener of the entity, with `event`, then `cleanupNullPointers`.
pub fn hkp_entity_callback_util_fire_contact_constraint_removed_callback(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    event: u32,
) {
    fire_collision_listeners(e, entity, LABEL_CONSTRAINT_REMOVED, 0x8, event);
}

// Translated from 00d00110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireContactPointAddedInternal` (Xbox PDB): stores `entity` in the
/// word at `event + 0xc`, then calls slot `0x10` of every collision listener with `event` and
/// runs `cleanupNullPointers`.
pub fn hkp_entity_callback_util_fire_contact_point_added_internal(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    event: u32,
) {
    e.mem.set_u32(event + 0xc, entity.addr());
    fire_collision_listeners(e, entity, LABEL_POINT_ADDED, 0x10, event);
}

// Translated from 00d001c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireContactPointRemovedInternal` (Xbox PDB): stores `entity` in the
/// word at `event + 0x10`, then calls slot `0x14` of every collision listener with `event` and
/// runs `cleanupNullPointers`.
pub fn hkp_entity_callback_util_fire_contact_point_removed_internal(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    event: u32,
) {
    e.mem.set_u32(event + 0x10, entity.addr());
    fire_collision_listeners(e, entity, LABEL_POINT_REMOVED, 0x14, event);
}

// Translated from 00d00270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireContactProcessInternal` (Xbox PDB): stores `entity` in the word
/// at `event + 8`, then calls slot `0x18` of every collision listener with `event` and runs
/// `cleanupNullPointers`.
pub fn hkp_entity_callback_util_fire_contact_process_internal(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    event: u32,
) {
    e.mem.set_u32(event + 8, entity.addr());
    fire_collision_listeners(e, entity, LABEL_PROCESS, 0x18, event);
}

// Translated from 00cffa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntityAdded` (Xbox PDB): `entityAddedCallback` (slot `0x04`) of
/// every entity listener.
pub fn hkp_entity_callback_util_fire_entity_added(e: &mut Engine, entity: Ptr<HkpEntityCallbacks>) {
    fire_listeners(e, entity, LABEL_ADDED, 0x4);
}

// Translated from 00cffda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntitySetMotionType` (Xbox PDB): `entitySetMotionTypeCallback`
/// (slot `0x10`) of every entity listener.
pub fn hkp_entity_callback_util_fire_entity_set_motion_type(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
) {
    fire_listeners(e, entity, LABEL_SET_MOTION_TYPE, 0x10);
}

// Translated from 00cffab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntityRemoved` (Xbox PDB): `entityRemovedCallback` (slot
/// `0x08`) of every entity listener, then, each between `"TtentRemCb"` records:
///
/// - while the master constraint list (16-bit size at +0xB0) is not empty, slot `0x10`
///   (`entityRemovedCallback` of `hkpConstraintInstance`, Xbox PDB) of its first constraint,
///   which removes it from the list;
/// - `hkpEntity::sortConstraintsSlavesDeterministically` (`00c9c860`);
/// - the same for the slave constraint list (32-bit size at +0xB8);
/// - while the action list (16-bit size at +0x20C) is not empty, slot `0x18`
///   (`entityRemovedCallback` of `hkpAction`, Xbox PDB) of its first action.
pub fn hkp_entity_callback_util_fire_entity_removed(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
) {
    fire_listeners(e, entity, LABEL_REMOVED, 0x8);
    let element = |e: &Engine, data: Ptr| e.mem.u32(data.addr());
    drain_callbacks(
        e,
        entity,
        0x10,
        |e| e.get(entity, HkpEntityCallbacks::m_constraintsMasterSize) != 0,
        |e| element(e, e.get(entity, HkpEntityCallbacks::m_constraintsMaster)),
    );
    e.call(SORT_CONSTRAINTS_SLAVES, &args![entity]);
    drain_callbacks(
        e,
        entity,
        0x10,
        |e| e.get(entity, HkpEntityCallbacks::m_constraintsSlaveSize) != 0,
        |e| element(e, e.get(entity, HkpEntityCallbacks::m_constraintsSlave)),
    );
    drain_callbacks(
        e,
        entity,
        0x18,
        |e| e.get(entity, HkpEntityCallbacks::m_actionsSize) != 0,
        |e| element(e, e.get(entity, HkpEntityCallbacks::m_actions)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTENER_VTABLE: u32 = 0x0200_0000;
    const OBJECT_VTABLE: u32 = 0x0200_0040;
    const ON_ADDED: u32 = 0x0200_0100;
    const ON_REMOVED: u32 = 0x0200_0104;
    const ON_MOTION: u32 = 0x0200_0108;
    const ON_CONSTRAINT: u32 = 0x0200_010c;
    const ON_ACTION: u32 = 0x0200_0114;

    struct Rig {
        e: Engine,
        entity: Ptr<HkpEntityCallbacks>,
        stream: u32,
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        e.put_vtable(LISTENER_VTABLE, &[0, ON_ADDED, ON_REMOVED, 0, ON_MOTION]);
        e.put_vtable(OBJECT_VTABLE, &[0, 0, 0, 0, ON_CONSTRAINT, 0, ON_ACTION]);
        for addr in [ON_ADDED, ON_REMOVED, ON_MOTION, SORT_CONSTRAINTS_SLAVES] {
            e.register(addr, |_, _| Ret::default());
        }
        // Each removal callback removes its object from the entity's list, as the real ones
        // do: the constraint slot serves the master list while it has entries, then the slaves.
        e.register(ON_CONSTRAINT, |e, a| {
            let masters = e.mem.u16(a[1] + 0xb0);
            if masters > 0 {
                e.mem.set_u16(a[1] + 0xb0, masters - 1);
            } else {
                let slaves = e.mem.i32(a[1] + 0xb8);
                e.mem.set_i32(a[1] + 0xb8, slaves - 1);
            }
            Ret::default()
        });
        e.register(ON_ACTION, |e, a| {
            let size = e.mem.u16(a[1] + 0x20c);
            e.mem.set_u16(a[1] + 0x20c, size - 1);
            Ret::default()
        });
        let stream = e.mem.alloc(0x200);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0x1f0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, _| stream.into_ret());
        let entity = e.new_object::<HkpEntityCallbacks>();
        Rig { e, entity, stream }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Gives the entity three listeners (the middle one null).
    fn add_listeners(r: &mut Rig) -> [u32; 2] {
        let listeners = [r.e.mem.alloc(4), r.e.mem.alloc(4)];
        for l in listeners {
            r.e.mem.set_u32(l, LISTENER_VTABLE);
        }
        let extended = r.e.mem.alloc(0x10);
        let data = r.e.mem.alloc(12);
        r.e.mem.set_u32(data, listeners[0]);
        r.e.mem.set_u32(data + 8, listeners[1]);
        r.e.mem.set_u32(extended + 8, data);
        r.e.mem.set_u16(extended + 0xc, 3);
        r.e.set(
            r.entity,
            HkpEntityCallbacks::m_extendedListeners,
            Ptr::new(extended),
        );
        listeners
    }

    fn records(r: &Rig) -> Vec<u32> {
        let cursor = r.e.mem.u32(r.stream + 4);
        (r.stream + 0x40..cursor)
            .step_by(12)
            .map(|at| r.e.mem.u32(at))
            .collect()
    }

    #[test]
    fn added_and_motion_type_call_listeners_last_first_skipping_null() {
        let mut r = rig();
        let listeners = add_listeners(&mut r);
        r.e.call(0x00cf_fa00, &args![r.entity]);
        let seen = calls(&r.e, ON_ADDED);
        assert_eq!(
            seen,
            vec![
                vec![listeners[1], r.entity.addr()],
                vec![listeners[0], r.entity.addr()]
            ]
        );
        assert_eq!(
            records(&r),
            vec![LABEL_ADDED, LABEL_END, LABEL_ADDED, LABEL_END]
        );
        r.e.call(0x00cf_fda0, &args![r.entity]);
        assert_eq!(calls(&r.e, ON_MOTION).len(), 2);
        assert_eq!(records(&r)[4], LABEL_SET_MOTION_TYPE);
    }

    #[test]
    fn no_extended_listeners_means_no_calls() {
        let mut r = rig();
        r.e.call(0x00cf_fa00, &args![r.entity]);
        assert!(calls(&r.e, ON_ADDED).is_empty());
        assert!(records(&r).is_empty());
    }

    #[test]
    fn removed_drains_constraints_then_sorts_then_actions() {
        let mut r = rig();
        let listeners = add_listeners(&mut r);
        // Two master constraints (the pointer array holds the instances), one action, and
        // slave constraints that remove themselves through their own slot.
        let instance = r.e.mem.alloc(4);
        r.e.mem.set_u32(instance, OBJECT_VTABLE);
        let masters = r.e.mem.alloc(8);
        r.e.mem.set_u32(masters, instance);
        r.e.set(
            r.entity,
            HkpEntityCallbacks::m_constraintsMaster,
            Ptr::new(masters),
        );
        r.e.set(r.entity, HkpEntityCallbacks::m_constraintsMasterSize, 2);
        let action = r.e.mem.alloc(4);
        r.e.mem.set_u32(action, OBJECT_VTABLE);
        let actions = r.e.mem.alloc(4);
        r.e.mem.set_u32(actions, action);
        r.e.set(r.entity, HkpEntityCallbacks::m_actions, Ptr::new(actions));
        r.e.set(r.entity, HkpEntityCallbacks::m_actionsSize, 1);
        // The slaves: one entry, called through the constraint slot once the master list is
        // empty (the double then lowers the slave count).
        let slave = r.e.mem.alloc(4);
        r.e.mem.set_u32(slave, OBJECT_VTABLE);
        let slaves = r.e.mem.alloc(4);
        r.e.mem.set_u32(slaves, slave);
        r.e.set(
            r.entity,
            HkpEntityCallbacks::m_constraintsSlave,
            Ptr::new(slaves),
        );
        r.e.set(r.entity, HkpEntityCallbacks::m_constraintsSlaveSize, 1);
        let entity_word = r.entity.addr();
        r.e.call(0x00cf_fab0, &args![r.entity]);

        assert_eq!(calls(&r.e, ON_REMOVED).len(), 2);
        assert_eq!(calls(&r.e, ON_REMOVED)[0], vec![listeners[1], entity_word]);
        // Two master callbacks, then the sort, then one slave callback, then the action.
        let order: Vec<u32> =
            r.e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .filter(|a| [ON_CONSTRAINT, SORT_CONSTRAINTS_SLAVES, ON_ACTION].contains(a))
                .collect();
        assert_eq!(
            order,
            vec![
                ON_CONSTRAINT,
                ON_CONSTRAINT,
                SORT_CONSTRAINTS_SLAVES,
                ON_CONSTRAINT,
                ON_ACTION
            ]
        );
        assert_eq!(calls(&r.e, ON_CONSTRAINT)[0], vec![instance, entity_word]);
        assert_eq!(calls(&r.e, ON_ACTION), vec![vec![action, entity_word]]);
        assert_eq!(r.e.get(r.entity, HkpEntityCallbacks::m_actionsSize), 0);
        assert_eq!(
            r.e.get(r.entity, HkpEntityCallbacks::m_constraintsSlaveSize),
            0
        );
        // Records: two listener calls, then 2 + 1 constraint calls and 1 action call.
        assert_eq!(records(&r).len(), 2 * (2 + 3 + 1));
    }

    const FULL_VTABLE: u32 = 0x0200_0080;
    const SLOT_DOUBLES: u32 = 0x0200_0200;

    /// The rig with a vtable of seven slots whose doubles sit at `SLOT_DOUBLES + 4 * slot`.
    fn full_rig() -> Rig {
        let mut r = rig();
        let slots: Vec<u32> = (0..7).map(|n| SLOT_DOUBLES + n * 4).collect();
        r.e.put_vtable(FULL_VTABLE, &slots);
        for slot in slots {
            r.e.register(slot, |_, _| Ret::default());
        }
        r
    }

    #[test]
    fn shape_set_and_deleted_call_their_slots_last_first() {
        let mut r = full_rig();
        let listeners = add_listeners(&mut r);
        for listener in listeners {
            r.e.mem.set_u32(listener, FULL_VTABLE);
        }
        let entity = r.entity.addr();
        r.e.call(0x00cf_fcf0, &args![r.entity]);
        assert_eq!(
            calls(&r.e, SLOT_DOUBLES + 0xc),
            vec![vec![listeners[1], entity], vec![listeners[0], entity]]
        );
        assert_eq!(
            records(&r),
            vec![LABEL_SHAPE_SET, LABEL_END, LABEL_SHAPE_SET, LABEL_END]
        );
        r.e.call(0x00cf_fe50, &args![r.entity]);
        assert_eq!(
            calls(&r.e, SLOT_DOUBLES + 0x14),
            vec![vec![listeners[1], entity], vec![listeners[0], entity]]
        );
        assert_eq!(records(&r)[4], LABEL_DELETED);
        // Without an extended block nothing is called.
        let mut r = full_rig();
        r.e.call(0x00cf_fe50, &args![r.entity]);
        assert!(records(&r).is_empty());
    }

    #[test]
    fn contact_callbacks_call_collision_listeners_then_drop_the_nulls() {
        // (function, label, vtable slot, event word that receives the entity)
        let table = [
            (0x00cf_ff00, LABEL_CONTACT_POINT, 0x0, None),
            (0x00cf_ffb0, LABEL_CONSTRAINT_ADDED, 0x4, None),
            (0x00d0_0060, LABEL_CONSTRAINT_REMOVED, 0x8, None),
            (0x00d0_0110, LABEL_POINT_ADDED, 0x10, Some(0xc)),
            (0x00d0_01c0, LABEL_POINT_REMOVED, 0x14, Some(0x10)),
            (0x00d0_0270, LABEL_PROCESS, 0x18, Some(0x8)),
        ];
        for (address, label, slot, event_word) in table {
            let mut r = full_rig();
            let listeners = [r.e.mem.alloc(4), r.e.mem.alloc(4)];
            for listener in listeners {
                r.e.mem.set_u32(listener, FULL_VTABLE);
            }
            let entity = r.entity.addr();
            let data = r.e.mem.alloc(16);
            r.e.mem.set_u32(data, listeners[0]);
            r.e.mem.set_u32(data + 8, listeners[1]);
            r.e.mem.set_u32(entity + 0x200, data);
            r.e.mem.set_u16(entity + 0x204, 3);
            let event = r.e.mem.alloc(0x20);
            r.e.call(address, &args![r.entity, event]);
            assert_eq!(
                calls(&r.e, SLOT_DOUBLES + slot),
                vec![vec![listeners[1], event], vec![listeners[0], event]]
            );
            assert_eq!(records(&r), vec![label, LABEL_END, label, LABEL_END]);
            assert_eq!(r.e.mem.u16(entity + 0x204), 2);
            assert_eq!(r.e.mem.u32(data), listeners[0]);
            assert_eq!(r.e.mem.u32(data + 4), listeners[1]);
            if let Some(word) = event_word {
                assert_eq!(r.e.mem.u32(event + word), entity);
            }
        }
    }

    #[test]
    fn contact_point_added_stores_the_entity_even_without_listeners() {
        let mut r = full_rig();
        let event = r.e.mem.alloc(0x20);
        r.e.call(0x00d0_0110, &args![r.entity, event]);
        assert_eq!(r.e.mem.u32(event + 0xc), r.entity.addr());
        assert!(records(&r).is_empty());
    }

    #[test]
    fn cleanup_removes_every_null_and_keeps_the_order() {
        let mut r = rig();
        let array = r.e.mem.alloc(8);
        let data = r.e.mem.alloc(20);
        for (index, value) in [1u32, 0, 0, 2, 0].into_iter().enumerate() {
            r.e.mem.set_u32(data + index as u32 * 4, value);
        }
        r.e.mem.set_u32(array, data);
        r.e.mem.set_u16(array + 4, 5);
        r.e.call(0x00cf_f9b0, &args![array]);
        assert_eq!(r.e.mem.u16(array + 4), 2);
        assert_eq!(r.e.mem.u32(data), 1);
        assert_eq!(r.e.mem.u32(data + 4), 2);
        // An empty array stays empty.
        r.e.mem.set_u16(array + 4, 0);
        r.e.call(0x00cf_f9b0, &args![array]);
        assert_eq!(r.e.mem.u16(array + 4), 0);
        // A single null empties the array.
        r.e.mem.set_u32(data, 0);
        r.e.mem.set_u16(array + 4, 1);
        r.e.call(0x00cf_f9b0, &args![array]);
        assert_eq!(r.e.mem.u16(array + 4), 0);
    }
}
