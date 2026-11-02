//! `Havok SDK/hkpunaryaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpUnaryAction` (Xbox PDB, 0x1C bytes) is an `hkpAction` (`m_world` `+8`, `m_island` `+0xC`,
//! `m_userData` `+0x10`, `m_name` `+0x14`, an `hkStringPtr`) acting on one entity (`m_entity`
//! `+0x18`), which it keeps a reference to. Its vtable is `010ce528`; the `hkpAction` base class
//! vtable is `010cc2fc` and the `hkReferencedObject` one `0102e388`.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpUnaryAction` (Xbox PDB), 0x1C bytes.
    pub struct HkpUnaryAction: 0x1c {
        /// `m_world` (Xbox PDB).
        0x08 m_world: Ptr,
        /// `m_userData` (Xbox PDB).
        0x10 m_user_data: u32,
        /// `m_entity` (Xbox PDB).
        0x18 m_entity: Ptr,
    }
}

/// `hkpWorld::removeActionImmediately` (Xbox PDB, `this` = the world, then the action).
const REMOVE_ACTION_IMMEDIATELY: u32 = 0x00c9_7510;
/// Two `hkpworld.cpp` functions the map gives no name: `00c976a0` (`stdcall`: the action and
/// the entity it is being detached from; the world the caller puts in ECX is not read) and
/// `00c92bd0` (`this` = the world, then the action and its new entity).
const WORLD_DETACH_ENTITY: u32 = 0x00c9_76a0;
const WORLD_ATTACH_ENTITY: u32 = 0x00c9_2bd0;
/// `hkReferencedObject` reference counting: add (`00c90510`) and remove (`00c905b0`), `this` =
/// the object (the map gives them no name).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkStringPtr::hkStringPtr` and `hkStringPtr::~hkStringPtr` (Xbox PDB, `this` = the string).
const STRING_PTR_CONSTRUCTOR: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
/// The memory router (`00c85750`, no arguments): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The `hkarrayutil.cpp` function that makes room for one more element (`00c90940`, cdecl
/// `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The vtables: the unary action's, the `hkpAction`'s and the `hkReferencedObject`'s.
const UNARY_ACTION_VTABLE: u32 = 0x010c_e528;
const ACTION_VTABLE: u32 = 0x010c_c2fc;
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00d2b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUnaryAction::entityRemovedCallback` (Xbox PDB): when the action is in a world, removes
/// it from it at once (`hkpWorld::removeActionImmediately`, a tail jump with the action as
/// the stack word). The function ends `RET 4`: the entity word is not read.
pub fn entity_removed_callback(e: &mut Engine, this: Ptr<HkpUnaryAction>, _unused_1: u32) {
    let world = e.get(this, HkpUnaryAction::m_world);
    if world.addr() != 0 {
        e.call(REMOVE_ACTION_IMMEDIATELY, &args![world, this]);
    }
}

// Translated from 00d2b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the action's entity (the map gives it no name; that it is a `setEntity` is
/// unconfirmed): when there is an old entity, it is detached from the world (when the action
/// is in one, `00c976a0` with the action and the old entity), its reference is removed
/// (`00c905b0`) and the field cleared. Then the new entity is stored, a reference added
/// (`00c90510`, also for a null entity, which the exe does without a check) and, when the
/// action is in a world, `00c92bd0` is told about it (the action and the new entity).
pub fn fn_00d2b210(e: &mut Engine, this: Ptr<HkpUnaryAction>, entity: Ptr) {
    let old = e.get(this, HkpUnaryAction::m_entity);
    if old.addr() != 0 {
        let world = e.get(this, HkpUnaryAction::m_world);
        if world.addr() != 0 {
            // 00d2b223: `00c976a0` is stdcall (RET 8) and does not read ECX.
            e.call(WORLD_DETACH_ENTITY, &args![this, old]);
        }
        let old = e.get(this, HkpUnaryAction::m_entity);
        e.call(REMOVE_REFERENCE, &args![old]);
        e.set(this, HkpUnaryAction::m_entity, Ptr::new(0));
    }
    e.set(this, HkpUnaryAction::m_entity, entity);
    e.call(ADD_REFERENCE, &args![entity]);
    let world = e.get(this, HkpUnaryAction::m_world);
    if world.addr() != 0 {
        let entity = e.get(this, HkpUnaryAction::m_entity);
        e.call(WORLD_ATTACH_ENTITY, &args![world, this, entity]);
    }
}

// Translated from 00d2b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUnaryAction::hkpUnaryAction` (Xbox PDB), `(this, entity, user data)`: the reference count
/// `+6` = 1, the `hkpAction` vtable at `+0`, no world and no island (`+8`, `+0xC`), the user
/// data (`+0x10`), the name string constructed (`+0x14`), then the unary action's vtable and
/// the entity (`+0x18`) with a reference added (`00c90510`) when it is not null. Returns
/// `this`.
pub fn hkp_unary_action_hkp_unary_action(
    e: &mut Engine,
    this: Ptr<HkpUnaryAction>,
    entity: Ptr,
    user_data: u32,
) -> Ptr<HkpUnaryAction> {
    e.set(this, HkpUnaryAction::m_user_data, user_data);
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), ACTION_VTABLE);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.call(STRING_PTR_CONSTRUCTOR, &args![this.addr() + 0x14]);
    e.mem.set_u32(this.addr(), UNARY_ACTION_VTABLE);
    e.set(this, HkpUnaryAction::m_entity, entity);
    if entity.addr() != 0 {
        e.call(ADD_REFERENCE, &args![entity]);
    }
    this
}

// Translated from 00d2b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUnaryAction::~hkpUnaryAction` (Xbox PDB): sets the unary action's vtable, removes the
/// reference of the entity (`00c905b0`) and clears the field when there is one, sets the
/// `hkpAction` vtable, destroys the name string (`+0x14`) and sets the `hkReferencedObject`
/// vtable.
pub fn hkp_unary_action_destructor(e: &mut Engine, this: Ptr<HkpUnaryAction>) {
    let entity = e.get(this, HkpUnaryAction::m_entity);
    e.mem.set_u32(this.addr(), UNARY_ACTION_VTABLE);
    if entity.addr() != 0 {
        e.call(REMOVE_REFERENCE, &args![entity]);
        e.set(this, HkpUnaryAction::m_entity, Ptr::new(0));
    }
    e.mem.set_u32(this.addr(), ACTION_VTABLE);
    e.call(STRING_PTR_DESTRUCTOR, &args![this.addr() + 0x14]);
    e.mem.set_u32(this.addr(), REFERENCED_OBJECT_VTABLE);
}

// Translated from 00d2b2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUnaryAction::getEntities` (Xbox PDB), `(this, entities)`: appends the action's entity
/// to the `hkArray` at `entities` (data `+0`, size `+4`, capacity and flags `+8`). The memory
/// router is asked first; when the size equals the capacity (`+8` masked with `3FFFFFFF`) the
/// array is grown through `00c90940` with the router's allocator.
pub fn get_entities(e: &mut Engine, this: Ptr<HkpUnaryAction>, entities: Ptr) {
    let array = entities.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let entity = e.get(this, HkpUnaryAction::m_entity);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), entity.addr());
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_b1f0,
            entity_removed_callback(Ptr<HkpUnaryAction>, u32)
        ),
        entry!(0x00d2_b210, fn_00d2b210(Ptr<HkpUnaryAction>, Ptr)),
        entry!(
            0x00d2_b260,
            hkp_unary_action_hkp_unary_action(Ptr<HkpUnaryAction>, Ptr, u32) -> Ptr<HkpUnaryAction>
        ),
        entry!(
            0x00d2_b2b0,
            hkp_unary_action_destructor(Ptr<HkpUnaryAction>)
        ),
        entry!(0x00d2_b2f0, get_entities(Ptr<HkpUnaryAction>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};

    /// An action with the given world and entity.
    fn action(e: &mut Engine, world: u32, entity: u32) -> u32 {
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 8, world);
        e.mem.set_u32(this + 0x18, entity);
        this
    }

    #[test]
    fn entity_removal_takes_the_action_out_of_its_world() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_ACTION_IMMEDIATELY, 0);
        let this = action(&mut e, 0x5000, 0x6000);
        e.call(0x00d2_b1f0, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, REMOVE_ACTION_IMMEDIATELY),
            vec![vec![0x5000, this]]
        );
        // Without a world nothing happens.
        let loose = action(&mut e, 0, 0x6000);
        e.call(0x00d2_b1f0, &args![loose, 0x6000u32]);
        assert_eq!(calls(&log, REMOVE_ACTION_IMMEDIATELY).len(), 1);
    }

    #[test]
    fn setting_the_entity_swaps_references_and_tells_the_world() {
        let mut e = engine();
        let log = new_log();
        for address in [
            WORLD_DETACH_ENTITY,
            WORLD_ATTACH_ENTITY,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        let this = action(&mut e, 0x5000, 0x6000);
        e.call(0x00d2_b210, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0x7000);
        assert_eq!(calls(&log, WORLD_DETACH_ENTITY), vec![vec![this, 0x6000]]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x7000]]);
        assert_eq!(
            calls(&log, WORLD_ATTACH_ENTITY),
            vec![vec![0x5000, this, 0x7000]]
        );
    }

    #[test]
    fn setting_the_entity_without_a_world_or_old_entity_only_references_it() {
        let mut e = engine();
        let log = new_log();
        for address in [
            WORLD_DETACH_ENTITY,
            WORLD_ATTACH_ENTITY,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        let this = action(&mut e, 0, 0);
        e.call(0x00d2_b210, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0x7000);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x7000]]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert!(calls(&log, WORLD_DETACH_ENTITY).is_empty());
        assert!(calls(&log, WORLD_ATTACH_ENTITY).is_empty());
        // An old entity without a world is released but not detached.
        let held = action(&mut e, 0, 0x6000);
        e.call(0x00d2_b210, &args![held, 0u32]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert!(calls(&log, WORLD_DETACH_ENTITY).is_empty());
        // The reference is added to a null entity as well (the exe does not check).
        assert_eq!(calls(&log, ADD_REFERENCE)[1], vec![0]);
    }

    #[test]
    fn construction_fills_the_header_and_references_the_entity() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, STRING_PTR_CONSTRUCTOR, 0);
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0xaa; 0x1c]);
        let result = e
            .call(0x00d2_b260, &args![this, 0x6000u32, 0x1234u32])
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_e528);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x1234);
        assert_eq!(e.mem.u32(this + 0x18), 0x6000);
        assert_eq!(calls(&log, STRING_PTR_CONSTRUCTOR), vec![vec![this + 0x14]]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x6000]]);
        // A null entity gets no reference.
        e.call(0x00d2_b260, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }

    #[test]
    fn destruction_releases_the_entity_and_the_name() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        log_double(&mut e, &log, STRING_PTR_DESTRUCTOR, 0);
        let this = action(&mut e, 0x5000, 0x6000);
        e.call(0x00d2_b2b0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(calls(&log, STRING_PTR_DESTRUCTOR), vec![vec![this + 0x14]]);
        // Without an entity nothing is released.
        let bare = action(&mut e, 0, 0);
        e.call(0x00d2_b2b0, &args![bare]);
        assert_eq!(calls(&log, REMOVE_REFERENCE).len(), 1);
        assert_eq!(calls(&log, STRING_PTR_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn getting_the_entities_appends_the_entity() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, ARRAY_RESERVE_MORE, 0);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x7777_0000);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = action(&mut e, 0, 0x6000);
        let data = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 0x8000_0004);
        e.call(0x00d2_b2f0, &args![this, array]);
        assert_eq!(e.mem.u32(data + 4), 0x6000);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert!(calls(&log, ARRAY_RESERVE_MORE).is_empty());
        // A full array is grown first (the capacity ignores the flag bits).
        e.mem.set_u32(array + 4, 4);
        e.call(0x00d2_b2f0, &args![this, array]);
        assert_eq!(
            calls(&log, ARRAY_RESERVE_MORE),
            vec![vec![0x7777_0000, array, 4]]
        );
        assert_eq!(e.mem.u32(data + 16), 0x6000);
        assert_eq!(e.mem.u32(array + 4), 5);
    }
}
