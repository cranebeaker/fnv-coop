//! `Havok SDK/hkprigidbodycentreofmassviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The viewer object (`this`) has its world viewer base at the start, the physics context at
//! `+0x24`, an entity listener sub-object at `+0x28`, a post-simulation listener sub-object at
//! `+0x2C` and the array of watched bodies (data, size, capacity words) at `+0x30`. None of the
//! functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_vec, profile_record, store_vec, zip_with, PROFILE_END, ZERO_F32,
};

/// The memory router (`00c85750`): its `+0x10` is the allocator (slot 4 allocates, slot `0x10`
/// frees).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpMotion::getMass` (`00c86430`, thiscall on the motion, the mass in `ST0`).
const GET_MASS: u32 = 0x00c8_6430;
/// The array growth (`00c90940`, cdecl: allocator, array, element size).
const GROW_ARRAY: u32 = 0x00c9_0940;
/// The world's listener lists (thiscall on the world, one word, `RET 4`): the entity listener
/// and the post-simulation listener are added (`00c93820`, `00c93920`) or removed (`00c917a0`,
/// `00c918e0`).
const ADD_ENTITY_LISTENER: u32 = 0x00c9_3820;
const ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
const REMOVE_ENTITY_LISTENER: u32 = 0x00c9_17a0;
const REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;
/// The world viewer base's constructor (`00ddccb0`, thiscall, one word) and destructor
/// (`00ddcc60`, thiscall).
const VIEWER_BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
const VIEWER_BASE_DESTRUCT: u32 = 0x00dd_cc60;
/// `hkProcessFactory::registerProcess` (`00dd9380`, thiscall on the factory: name, creator;
/// `RET 8`) and the factory (`01269cd0`).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
/// The process id the registration answers (`01269020`).
const PROCESS_ID: u32 = 0x0126_9020;
/// The creator function registered with the factory, and the process name `Center of Mass`.
const CREATOR: u32 = 0x00d6_a140;
const PROCESS_NAME: u32 = 0x010d_21f0;
/// The scale of the axis lines (`011b6194`, a float).
const AXIS_SCALE: u32 = 0x011b_6194;

/// The profiling labels: the callback, the line set-up and the drawing (the end label is
/// `PROFILE_END`).
const LABEL_VIEWER: u32 = 0x010d_2224;
const LABEL_MASS_AND_LINES: u32 = 0x010d_2210;
const LABEL_DISPLAY_LINES: u32 = 0x010d_2200;

/// The vtables the constructor and the destructor store.
const VTABLE_BASE_SELF: u32 = 0x010d_22a8;
const VTABLE_BASE_VIEWER: u32 = 0x010d_2288;
const VTABLE_WORLD_LISTENER: u32 = 0x010d_2278;
const VTABLE_ENTITY_LISTENER: u32 = 0x010d_225c;
const VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010d_224c;
/// The base-class vtables that the constructor and destructor store first.
const VTABLE_ENTITY_LISTENER_BASE: u32 = 0x0102_f13c;
const VTABLE_POST_SIMULATION_LISTENER_BASE: u32 = 0x010c_cf7c;

/// Calls slot `slot` of the listener at `listener` with each entity of the array at
/// `island + 0x48` (the count is `+0x4C`); the array and the count are read again at every step.
fn for_each_entity(e: &mut Engine, listener: u32, island: u32, slot: u32) {
    let mut index = 0;
    while index < e.mem.i32(island + 0x4c) {
        let entity = e
            .mem
            .u32(e.mem.u32(island + 0x48) + (index as u32).wrapping_mul(4));
        e.vcall(listener, slot, &args![entity]);
        index += 1;
    }
}

/// Calls slot `slot` of the entity listener (`this + 0x28`) for every entity of the world's
/// active islands (array `+0x28`, count `+0x2C`), its inactive islands (array `+0x34`, count
/// `+0x38`) and, when there is one, its fixed island (`+0x20`).
fn for_each_world_entity(e: &mut Engine, this: u32, world: u32, slot: u32) {
    let listener = this.wrapping_add(0x28);
    for (array, count) in [(0x28, 0x2c), (0x34, 0x38)] {
        let mut index = 0;
        while index < e.mem.i32(world + count) {
            let island = e
                .mem
                .u32(e.mem.u32(world + array) + (index as u32).wrapping_mul(4));
            for_each_entity(e, listener, island, slot);
            index += 1;
        }
    }
    let fixed = e.mem.u32(world + 0x20);
    if fixed != 0 {
        for_each_entity(e, listener, fixed, slot);
    }
}

/// A listener sub-object address of the viewer: `this + offset`, or 0 for a null viewer.
fn sub_object(this: u32, offset: u32) -> u32 {
    if this == 0 {
        0
    } else {
        this + offset
    }
}

// Translated from 00d69b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's world removal (thiscall with the world, `RET 4`): the world drops the viewer's
/// entity listener (`this + 0x28`, `00c917a0`) and post-simulation listener (`this + 0x2C`,
/// `00c918e0`) (both 0 for a null `this`), then slot 8 of the entity listener is called with
/// each entity of the world's islands (see `for_each_world_entity`).
pub fn fn_00d69b20(e: &mut Engine, this: Ptr, world: Ptr) {
    let me = this.addr();
    e.call(REMOVE_ENTITY_LISTENER, &args![world, sub_object(me, 0x28)]);
    e.call(
        REMOVE_POST_SIMULATION_LISTENER,
        &args![world, sub_object(me, 0x2c)],
    );
    for_each_world_entity(e, me, world.addr(), 8);
}

// Translated from 00d69c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's world addition (thiscall with the world, `RET 4`): the world takes the viewer's
/// entity listener (`this + 0x28`, `00c93820`) and post-simulation listener (`this + 0x2C`,
/// `00c93920`, `hkpWorld::addWorldPostSimulationListener` in the Xbox build) (both 0 for a null
/// `this`), then slot 4 of the entity listener is called with each entity of the world's
/// islands (see `for_each_world_entity`).
pub fn fn_00d69c10(e: &mut Engine, this: Ptr, world: Ptr) {
    let me = this.addr();
    e.call(ADD_ENTITY_LISTENER, &args![world, sub_object(me, 0x28)]);
    e.call(
        ADD_POST_SIMULATION_LISTENER,
        &args![world, sub_object(me, 0x2c)],
    );
    for_each_world_entity(e, me, world.addr(), 4);
}

// Translated from 00d69d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBodyCentreOfMassViewer::postSimulationCallback` (Xbox PDB), `RET 4` with one unread stack word, fastcall on the
/// post-simulation listener sub-object (`this`: the body array's data at `+4`, its size at
/// `+8`; the display handler is the object at `this - 0x14`). For each body whose mass
/// (`hkpMotion::getMass` of the motion at `+0xE0`; skipped only when equal to zero, so a NaN is
/// drawn) is not zero, three axis lines are drawn from the centre of mass (`+0x140`) to the
/// centre of mass plus the scaled (`011b6194`) rotation columns (`+0xF0`, `+0x100`, `+0x110`),
/// through slot `0x24` of the display handler with `(centre, end point, colour, the body's
/// transform at +0x10, process id)`: red for the first column, green for the second, blue for
/// the third (ARGB `ffff0000`, `ff008000`, `ff0000ff`). The code keeps the four vectors in its
/// stack frame in the order centre, third end point, first end point, second end point; each
/// body writes a profiling record pair around the set-up and another around the drawing.
pub fn hkp_rigid_body_centre_of_mass_viewer_post_simulation_callback(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) {
    let me = this.addr();
    profile_record(e, LABEL_VIEWER);
    // The frame: the centre at +0, then the end points of the third, first and second axes.
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let zero = f64::from(e.global::<f32>(ZERO_F32));
        let mut index = 0;
        while index < e.mem.i32(me + 8) {
            let slot = e
                .mem
                .u32(me + 4)
                .wrapping_add((index as u32).wrapping_mul(4));
            let body = e.mem.u32(slot);
            let mass = e.call(GET_MASS, &args![body.wrapping_add(0xe0)]).f64();
            if mass != zero {
                profile_record(e, LABEL_MASS_AND_LINES);
                let body = e.mem.u32(
                    e.mem
                        .u32(me + 4)
                        .wrapping_add((index as u32).wrapping_mul(4)),
                );
                let centre = load_vec(e, body + 0x140);
                store_vec(e, frame, centre);
                let scale: f32 = e.global(AXIS_SCALE);
                let broadcast = [scale; 4];
                let first = zip_with(broadcast, load_vec(e, body + 0xf0), |s, c| s * c);
                let first = zip_with(first, centre, |a, b| a + b);
                store_vec(e, frame + 0x20, first);
                let second = zip_with(broadcast, load_vec(e, body + 0x100), |s, c| s * c);
                let second = zip_with(second, centre, |a, b| a + b);
                store_vec(e, frame + 0x30, second);
                let third = zip_with(broadcast, load_vec(e, body + 0x110), |s, c| s * c);
                let third = zip_with(third, centre, |a, b| a + b);
                store_vec(e, frame + 0x10, third);
                profile_record(e, PROFILE_END);
                profile_record(e, LABEL_DISPLAY_LINES);
                let body = e.mem.u32(
                    e.mem
                        .u32(me + 4)
                        .wrapping_add((index as u32).wrapping_mul(4)),
                );
                let transform = body.wrapping_add(0x10);
                for (end_point, colour) in [
                    (frame + 0x20, 0xffff_0000u32),
                    (frame + 0x30, 0xff00_8000),
                    (frame + 0x10, 0xff00_00ff),
                ] {
                    let process: u32 = e.global(PROCESS_ID);
                    let handler = e.mem.u32(me.wrapping_sub(0x14));
                    e.vcall(
                        handler,
                        0x24,
                        &args![frame, end_point, colour, transform, process],
                    );
                }
                profile_record(e, PROFILE_END);
            }
            index += 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d69f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the viewer to every world of its context (fastcall; `this` is the viewer plus 8, the
/// context is at `this + 0x1C` and may be null): `fn_00d69c10` on the viewer (`this - 8`) with
/// each world of the context's array (data `+0x6C`, count `+0x70`, both read again at every
/// step).
pub fn fn_00d69f60(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let mut context = e.mem.u32(me + 0x1c);
    if context == 0 {
        return;
    }
    let mut index = 0;
    while index < e.mem.i32(context + 0x70) {
        let world = e
            .mem
            .u32(e.mem.u32(context + 0x6c) + (index as u32).wrapping_mul(4));
        fn_00d69c10(e, Ptr::new(me.wrapping_sub(8)), Ptr::new(world));
        context = e.mem.u32(me + 0x1c);
        index += 1;
    }
}

// Translated from 00d69fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The thunk of `fn_00d69b20` for the sub-object at the viewer plus `0x20` (thiscall with the
/// world, `RET 4`): `this - 0x20`.
pub fn fn_00d69fa0(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d69b20(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d69fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The thunk of `fn_00d69c10` for the sub-object at the viewer plus `0x20` (thiscall with the
/// world, `RET 4`): `this - 0x20`.
pub fn fn_00d69fb0(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d69c10(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d69fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the first entry equal to `value` from the pointer array whose data is at
/// `this + 8` and size at `this + 0xC` (thiscall, `RET 4`): the size shrinks and the last entry
/// moves into the freed place (unless the entry was the last one). The size is read once for
/// the search.
pub fn fn_00d69fc0(e: &mut Engine, this: Ptr, value: u32) {
    let me = this.addr();
    let size = e.mem.i32(me + 0xc);
    if size <= 0 {
        return;
    }
    let data = e.mem.u32(me + 8);
    let mut index: i32 = 0;
    loop {
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == value {
            break;
        }
        index += 1;
        if index >= size {
            return;
        }
    }
    if index < 0 {
        return;
    }
    let last = e.mem.i32(me + 0xc).wrapping_sub(1);
    e.mem.set_i32(me + 0xc, last);
    if last != index {
        let data = e.mem.u32(me + 8);
        let moved = e.mem.u32(data.wrapping_add((last as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
    }
}

// Translated from 00d6a000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `value` to the pointer array whose data is at `this + 8`, size at `this + 0xC` and
/// capacity (low 30 bits of `+0x10`) after it (thiscall, `RET 4`): a full array grows through
/// `00c90940` with the memory router's allocator first.
pub fn fn_00d6a000(e: &mut Engine, this: Ptr, value: u32) {
    let array = this.addr() + 8;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(GROW_ARRAY, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem
        .set_u32(array + 4, e.mem.u32(array + 4).wrapping_add(1));
}

// Translated from 00d6a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's constructor (thiscall with the context, `RET 4`; answers `this`): the world
/// viewer base is constructed with the context, the listener vtables are stored (the entity
/// and post-simulation listener ones first as the base's, then as the viewer's), and the body
/// array (`+0x30`) starts empty with the capacity word `0x80000000`.
pub fn fn_00d6a040(e: &mut Engine, this: Ptr, context: u32) -> Ptr {
    let me = this.addr();
    e.call(VIEWER_BASE_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(me + 0x28, VTABLE_ENTITY_LISTENER_BASE);
    e.mem
        .set_u32(me + 0x2c, VTABLE_POST_SIMULATION_LISTENER_BASE);
    e.mem.set_u32(me, VTABLE_BASE_SELF);
    e.mem.set_u32(me + 8, VTABLE_BASE_VIEWER);
    e.mem.set_u32(me + 0x20, VTABLE_WORLD_LISTENER);
    e.mem.set_u32(me + 0x28, VTABLE_ENTITY_LISTENER);
    e.mem.set_u32(me + 0x2c, VTABLE_POST_SIMULATION_LISTENER);
    e.mem.set_u32(me + 0x30, 0);
    e.mem.set_u32(me + 0x34, 0);
    e.mem.set_u32(me + 0x38, 0x8000_0000);
    this
}

// Translated from 00d6a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's destructor (fastcall): the vtables are stored, the viewer leaves every world of
/// its context (`+0x24`, read before the stores; `fn_00d69b20` for each world of the array at
/// `+0x6C`, count `+0x70`), the body array is emptied and its storage freed (allocator slot
/// `0x10`, data and `capacity * 4` bytes, unless the capacity word is negative), the base-class
/// listener vtables are stored and the world viewer base's destructor (`00ddcc60`) finishes.
pub fn fn_00d6a0a0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let context = e.mem.u32(me + 0x24);
    e.mem.set_u32(me, VTABLE_BASE_SELF);
    e.mem.set_u32(me + 8, VTABLE_BASE_VIEWER);
    e.mem.set_u32(me + 0x20, VTABLE_WORLD_LISTENER);
    e.mem.set_u32(me + 0x28, VTABLE_ENTITY_LISTENER);
    e.mem.set_u32(me + 0x2c, VTABLE_POST_SIMULATION_LISTENER);
    if context != 0 {
        let mut current = context;
        let mut index = 0;
        while index < e.mem.i32(current + 0x70) {
            current = e.mem.u32(me + 0x24);
            let world = e
                .mem
                .u32(e.mem.u32(current + 0x6c) + (index as u32).wrapping_mul(4));
            fn_00d69b20(e, this, Ptr::new(world));
            current = e.mem.u32(me + 0x24);
            index += 1;
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.i32(me + 0x38);
    e.mem.set_u32(me + 0x34, 0);
    if capacity >= 0 {
        let bytes = ((capacity as u32) & 0x3fff_ffff).wrapping_mul(4);
        let data = e.mem.u32(me + 0x30);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x30, 0);
    e.mem.set_u32(me + 0x38, 0x8000_0000);
    e.mem
        .set_u32(me + 0x2c, VTABLE_POST_SIMULATION_LISTENER_BASE);
    e.mem.set_u32(me + 0x28, VTABLE_ENTITY_LISTENER_BASE);
    e.call(VIEWER_BASE_DESTRUCT, &args![this]);
}

// Translated from 00d6a180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBodyCentreOfMassViewer::registerViewer` (Xbox PDB): registers the process
/// `Center of Mass` (creator `00d6a140`) with the process factory (`01269cd0`) and keeps the
/// id in `01269020`.
pub fn hkp_rigid_body_centre_of_mass_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(REGISTER_PROCESS, &args![factory, PROCESS_NAME, CREATOR])
        .u32();
    e.set_global(PROCESS_ID, id);
}

// Translated from 00d6a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x3c bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d6a040` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d6a186.
pub fn fn_00d6a140(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x3c]).u32();
    e.mem.set_u16(block + 4, 0x3c);
    let object = e.call(0x00d6_a040, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_9b20, fn_00d69b20(Ptr, Ptr)),
        entry!(0x00d6_9c10, fn_00d69c10(Ptr, Ptr)),
        entry!(
            0x00d6_9d00,
            hkp_rigid_body_centre_of_mass_viewer_post_simulation_callback(Ptr, u32)
        ),
        entry!(0x00d6_9f60, fn_00d69f60(Ptr)),
        entry!(0x00d6_9fa0, fn_00d69fa0(Ptr, Ptr)),
        entry!(0x00d6_9fb0, fn_00d69fb0(Ptr, Ptr)),
        entry!(0x00d6_9fc0, fn_00d69fc0(Ptr, u32)),
        entry!(0x00d6_a000, fn_00d6a000(Ptr, u32)),
        entry!(0x00d6_a040, fn_00d6a040(Ptr, u32) -> Ptr),
        entry!(0x00d6_a0a0, fn_00d6a0a0(Ptr)),
        entry!(
            0x00d6_a180,
            hkp_rigid_body_centre_of_mass_viewer_register_viewer()
        ),
        entry!(0x00d6_a140, fn_00d6a140(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Slots 4 and 8 of the test entity listener, slot `0x24` of the display handler and slot
    /// `0x10` of the allocator.
    const ENTITY_ADDED: u32 = 0x0074_0004;
    const ENTITY_REMOVED: u32 = 0x0074_0008;
    const DRAW: u32 = 0x0075_0024;
    const FREE: u32 = 0x0076_0010;

    /// Every call the doubles saw, in order.
    fn trace(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.borrow().clone()
    }

    /// Doubles for the world's listener lists and the viewer's entity listener slots.
    fn listener_doubles(e: &mut Engine, log: &Log) {
        for address in [
            ADD_ENTITY_LISTENER,
            ADD_POST_SIMULATION_LISTENER,
            REMOVE_ENTITY_LISTENER,
            REMOVE_POST_SIMULATION_LISTENER,
            ENTITY_ADDED,
            ENTITY_REMOVED,
        ] {
            log_double(e, log, address, 0);
        }
    }

    /// A viewer (0x80 bytes) whose entity listener sub-object (`+0x28`) has the slots 4 and 8
    /// of the test listener.
    fn viewer(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, ENTITY_ADDED);
        e.mem.set_u32(table + 8, ENTITY_REMOVED);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x28, table);
        this
    }

    fn words(e: &mut Engine, values: &[u32]) -> u32 {
        let data = e.mem.alloc(0x40);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *value);
        }
        data
    }

    /// An island whose entity array holds `entities`.
    fn island(e: &mut Engine, entities: &[u32]) -> u32 {
        let island = e.mem.alloc(0x60);
        let data = words(e, entities);
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_u32(island + 0x4c, entities.len() as u32);
        island
    }

    /// A world with the given active and inactive islands and fixed island.
    fn world(e: &mut Engine, active: &[u32], inactive: &[u32], fixed: u32) -> u32 {
        let world = e.mem.alloc(0x80);
        for (array, count, islands) in [(0x28, 0x2c, active), (0x34, 0x38, inactive)] {
            let data = words(e, islands);
            e.mem.set_u32(world + array, data);
            e.mem.set_u32(world + count, islands.len() as u32);
        }
        e.mem.set_u32(world + 0x20, fixed);
        world
    }

    /// Checks a world walk (`00d69b20` or `00d69c10`, or a thunk of them at `offset`).
    fn check_world_walk(address: u32, offset: u32, world_calls: [u32; 2], slot_function: u32) {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let this = viewer(&mut e);
        let active_a = island(&mut e, &[0x11, 0x12]);
        let active_b = island(&mut e, &[]);
        let active_c = island(&mut e, &[0x13]);
        let inactive = island(&mut e, &[0x21]);
        let fixed = island(&mut e, &[0x31, 0x32]);
        let world = world(&mut e, &[active_a, active_b, active_c], &[inactive], fixed);
        e.call(address, &args![this + offset, world]);
        let listener = this + 0x28;
        let mut expected = vec![
            (world_calls[0], vec![world, this + 0x28]),
            (world_calls[1], vec![world, this + 0x2c]),
        ];
        for entity in [0x11, 0x12, 0x13, 0x21, 0x31, 0x32] {
            expected.push((slot_function, vec![listener, entity]));
        }
        assert_eq!(trace(&log), expected);
    }

    #[test]
    fn the_world_removal_walks_every_island_00d69b20() {
        check_world_walk(
            0x00d6_9b20,
            0,
            [REMOVE_ENTITY_LISTENER, REMOVE_POST_SIMULATION_LISTENER],
            ENTITY_REMOVED,
        );
    }

    #[test]
    fn the_world_addition_walks_every_island_00d69c10() {
        check_world_walk(
            0x00d6_9c10,
            0,
            [ADD_ENTITY_LISTENER, ADD_POST_SIMULATION_LISTENER],
            ENTITY_ADDED,
        );
    }

    #[test]
    fn the_removal_thunk_adjusts_this_by_0x20_00d69fa0() {
        check_world_walk(
            0x00d6_9fa0,
            0x20,
            [REMOVE_ENTITY_LISTENER, REMOVE_POST_SIMULATION_LISTENER],
            ENTITY_REMOVED,
        );
    }

    #[test]
    fn the_addition_thunk_adjusts_this_by_0x20_00d69fb0() {
        check_world_walk(
            0x00d6_9fb0,
            0x20,
            [ADD_ENTITY_LISTENER, ADD_POST_SIMULATION_LISTENER],
            ENTITY_ADDED,
        );
    }

    #[test]
    fn a_world_without_a_fixed_island_or_entities_only_touches_the_listeners_00d69c10() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let this = viewer(&mut e);
        let empty = island(&mut e, &[]);
        let world = world(&mut e, &[empty], &[], 0);
        e.call(0x00d6_9c10, &args![this, world]);
        assert_eq!(
            trace(&log),
            vec![
                (ADD_ENTITY_LISTENER, vec![world, this + 0x28]),
                (ADD_POST_SIMULATION_LISTENER, vec![world, this + 0x2c]),
            ]
        );
    }

    #[test]
    fn a_null_viewer_passes_null_listeners_00d69b20() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let world = world(&mut e, &[], &[], 0);
        e.call(0x00d6_9b20, &args![0u32, world]);
        assert_eq!(
            trace(&log),
            vec![
                (REMOVE_ENTITY_LISTENER, vec![world, 0]),
                (REMOVE_POST_SIMULATION_LISTENER, vec![world, 0]),
            ]
        );
    }

    #[test]
    fn a_null_viewer_passes_null_listeners_00d69c10() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let world = world(&mut e, &[], &[], 0);
        e.call(0x00d6_9c10, &args![0u32, world]);
        assert_eq!(
            trace(&log),
            vec![
                (ADD_ENTITY_LISTENER, vec![world, 0]),
                (ADD_POST_SIMULATION_LISTENER, vec![world, 0]),
            ]
        );
    }

    /// A context (0x80 bytes) with the given worlds at `+0x6C` / `+0x70`.
    fn context(e: &mut Engine, worlds: &[u32]) -> u32 {
        let context = e.mem.alloc(0x80);
        let data = words(e, worlds);
        e.mem.set_u32(context + 0x6c, data);
        e.mem.set_u32(context + 0x70, worlds.len() as u32);
        context
    }

    #[test]
    fn the_viewer_joins_every_world_of_its_context_00d69f60() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let this = viewer(&mut e);
        let first = world(&mut e, &[], &[], 0);
        let second = world(&mut e, &[], &[], 0);
        let context = context(&mut e, &[first, second]);
        e.mem.set_u32(this + 0x24, context);
        e.call(0x00d6_9f60, &args![this + 8]);
        assert_eq!(
            trace(&log),
            vec![
                (ADD_ENTITY_LISTENER, vec![first, this + 0x28]),
                (ADD_POST_SIMULATION_LISTENER, vec![first, this + 0x2c]),
                (ADD_ENTITY_LISTENER, vec![second, this + 0x28]),
                (ADD_POST_SIMULATION_LISTENER, vec![second, this + 0x2c]),
            ]
        );
    }

    #[test]
    fn a_viewer_without_a_context_or_worlds_joins_nothing_00d69f60() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let this = viewer(&mut e);
        e.call(0x00d6_9f60, &args![this + 8]);
        let none = context(&mut e, &[]);
        e.mem.set_u32(this + 0x24, none);
        e.call(0x00d6_9f60, &args![this + 8]);
        assert!(trace(&log).is_empty());
    }

    /// An array object with `values` at `+8` (data), `+0xC` (size) and `+0x10` (capacity).
    fn array_object(e: &mut Engine, values: &[u32]) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = words(e, values);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, values.len() as u32);
        e.mem.set_u32(this + 0x10, 8);
        this
    }

    fn array_values(e: &Engine, this: u32) -> Vec<u32> {
        let data = e.mem.u32(this + 8);
        (0..e.mem.u32(this + 0xc))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn removing_an_entry_moves_the_last_one_into_its_place_00d69fc0() {
        for (values, value, expected) in [
            (vec![1, 2, 3, 4], 2, vec![1, 4, 3]),
            (vec![1, 2, 3, 4], 1, vec![4, 2, 3]),
            (vec![1, 2, 3, 4], 4, vec![1, 2, 3]),
            (vec![7], 7, vec![]),
            (vec![1, 2, 3, 2], 2, vec![1, 2, 3]),
            (vec![1, 2, 3], 9, vec![1, 2, 3]),
            (vec![], 9, vec![]),
        ] {
            let mut e = engine();
            let this = array_object(&mut e, &values);
            e.call(0x00d6_9fc0, &args![this, value]);
            assert_eq!(array_values(&e, this), expected, "{values:?} - {value}");
        }
    }

    /// The router (its allocator is `allocator`) and the array growth double, which moves the
    /// entries to a new buffer of room for 16 entries.
    fn growth_doubles(e: &mut Engine, log: &Log, allocator: u32) {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, MEMORY_ROUTER, router);
        let seen = Rc::clone(log);
        e.register_double(GROW_ARRAY, move |e, a| {
            seen.borrow_mut().push((GROW_ARRAY, a.to_vec()));
            let array = a[1];
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let new = e.mem.alloc(0x40);
            for index in 0..size {
                let word = e.mem.u32(old + 4 * index);
                e.mem.set_u32(new + 4 * index, word);
            }
            e.mem.set_u32(array, new);
            e.mem.set_u32(array + 8, 16);
            Ret::default()
        });
    }

    #[test]
    fn appending_with_room_does_not_grow_the_array_00d6a000() {
        let mut e = engine();
        let log = new_log();
        growth_doubles(&mut e, &log, 0xa110);
        let this = array_object(&mut e, &[1, 2]);
        e.call(0x00d6_a000, &args![this, 0x55u32]);
        assert_eq!(array_values(&e, this), vec![1, 2, 0x55]);
        assert!(calls(&log, GROW_ARRAY).is_empty());
        assert_eq!(calls(&log, MEMORY_ROUTER).len(), 1);
    }

    #[test]
    fn appending_to_a_full_array_grows_it_first_00d6a000() {
        let mut e = engine();
        let log = new_log();
        growth_doubles(&mut e, &log, 0xa110);
        let this = array_object(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8]);
        // The flag bits above the capacity do not count.
        e.mem.set_u32(this + 0x10, 0x8000_0008);
        e.call(0x00d6_a000, &args![this, 0x55u32]);
        assert_eq!(array_values(&e, this), vec![1, 2, 3, 4, 5, 6, 7, 8, 0x55]);
        assert_eq!(calls(&log, GROW_ARRAY), vec![vec![0xa110, this + 8, 4]]);
        assert_eq!(e.mem.u32(this + 0x10), 16);
    }

    #[test]
    fn the_constructor_builds_the_base_and_stores_the_vtables_00d6a040() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, VIEWER_BASE_CONSTRUCT, 0);
        let this = e.mem.alloc(0x80);
        e.mem.write(this, &[0xcd; 0x80]);
        assert_eq!(e.call(0x00d6_a040, &args![this, 0x1234u32]).u32(), this);
        assert_eq!(calls(&log, VIEWER_BASE_CONSTRUCT), vec![vec![this, 0x1234]]);
        assert_eq!(e.mem.u32(this), 0x010d_22a8);
        assert_eq!(e.mem.u32(this + 8), 0x010d_2288);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_2278);
        assert_eq!(e.mem.u32(this + 0x28), 0x010d_225c);
        assert_eq!(e.mem.u32(this + 0x2c), 0x010d_224c);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 0x3c), 0xcdcd_cdcd);
    }

    /// The router with an allocator whose slot `0x10` is the logging `FREE`.
    fn free_doubles(e: &mut Engine, log: &Log) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, MEMORY_ROUTER, router);
        log_double(e, log, FREE, 0);
        log_double(e, log, VIEWER_BASE_DESTRUCT, 0);
        allocator
    }

    #[test]
    fn the_destructor_leaves_the_worlds_and_frees_the_array_00d6a0a0() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        let allocator = free_doubles(&mut e, &log);
        let this = viewer(&mut e);
        let first = world(&mut e, &[], &[], 0);
        let second = world(&mut e, &[], &[], 0);
        let context = context(&mut e, &[first, second]);
        e.mem.set_u32(this + 0x24, context);
        e.mem.set_u32(this + 0x30, 0x7000);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x4000_0005);
        e.call(0x00d6_a0a0, &args![this]);
        assert_eq!(
            trace(&log)
                .into_iter()
                .filter(|(address, _)| *address != MEMORY_ROUTER)
                .collect::<Vec<_>>(),
            vec![
                (REMOVE_ENTITY_LISTENER, vec![first, this + 0x28]),
                (REMOVE_POST_SIMULATION_LISTENER, vec![first, this + 0x2c]),
                (REMOVE_ENTITY_LISTENER, vec![second, this + 0x28]),
                (REMOVE_POST_SIMULATION_LISTENER, vec![second, this + 0x2c]),
                (FREE, vec![allocator, 0x7000, 20]),
                (VIEWER_BASE_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x010d_22a8);
        assert_eq!(e.mem.u32(this + 8), 0x010d_2288);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_2278);
        assert_eq!(e.mem.u32(this + 0x28), 0x0102_f13c);
        assert_eq!(e.mem.u32(this + 0x2c), 0x010c_cf7c);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn the_destructor_keeps_storage_it_does_not_own_and_skips_a_missing_context_00d6a0a0() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log);
        free_doubles(&mut e, &log);
        let this = viewer(&mut e);
        e.mem.set_u32(this + 0x30, 0x7000);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x8000_0000);
        e.call(0x00d6_a0a0, &args![this]);
        assert!(calls(&log, FREE).is_empty());
        assert!(calls(&log, REMOVE_ENTITY_LISTENER).is_empty());
        assert_eq!(calls(&log, VIEWER_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
    }

    #[test]
    fn registering_the_viewer_keeps_the_process_id_00d6a180() {
        let mut e = engine();
        let log = new_log();
        e.set_global(PROCESS_FACTORY, 0xfac0u32);
        log_double(&mut e, &log, REGISTER_PROCESS, 0x4a);
        e.call(0x00d6_a180, &args![]);
        assert_eq!(
            calls(&log, REGISTER_PROCESS),
            vec![vec![0xfac0, 0x010d_21f0, 0x00d6_a140]]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x4a);
    }

    /// The draw calls the display handler saw: (centre, end point, colour, transform, id).
    type Draws = Rc<RefCell<Vec<(Vec4, Vec4, u32, u32, u32, u32)>>>;

    /// A body with the centre of mass, three rotation columns and a distinct transform address.
    fn body(e: &mut Engine, centre: Vec4, columns: [Vec4; 3]) -> u32 {
        let body = e.mem.alloc(0x180);
        store_vec(e, body + 0x140, centre);
        for (index, column) in columns.iter().enumerate() {
            store_vec(e, body + 0xf0 + 0x10 * index as u32, *column);
        }
        body
    }

    #[test]
    fn the_callback_draws_the_three_axes_of_each_body_with_mass_00d69d00() {
        let mut e = engine();
        e.set_global(AXIS_SCALE, 2.0f32);
        e.set_global(PROCESS_ID, 0x77u32);
        let (stream, records) = monitor(&mut e, 0x200);
        let columns = [
            [1.0, 0.0, 0.0, 5.0],
            [0.0, 1.0, 0.0, 6.0],
            [0.0, 0.0, 1.0, 7.0],
        ];
        let centre = [1.0, 2.0, 3.0, 4.0];
        let bodies: Vec<u32> = (0..4).map(|_| body(&mut e, centre, columns)).collect();
        let masses = Rc::new(RefCell::new(vec![
            (bodies[0] + 0xe0, 2.0f64),
            (bodies[1] + 0xe0, 0.0),
            (bodies[2] + 0xe0, f64::NAN),
            (bodies[3] + 0xe0, -1.0),
        ]));
        let asked = Rc::clone(&masses);
        e.register_double(GET_MASS, move |_, a| {
            let found = asked
                .borrow()
                .iter()
                .find(|(motion, _)| *motion == a[0])
                .copied();
            Ret {
                st0: found.map(|(_, mass)| mass).unwrap(),
                ..Ret::default()
            }
        });
        let draws: Draws = Rc::default();
        let seen = Rc::clone(&draws);
        e.register_double(DRAW, move |e, a| {
            seen.borrow_mut().push((
                load_vec(e, a[1]),
                load_vec(e, a[2]),
                a[2] - a[1],
                a[3],
                a[4],
                a[5],
            ));
            Ret::default()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x24, DRAW);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(handler, table);
        let block = e.mem.alloc(0x40);
        let this = block + 0x20;
        e.mem.set_u32(this - 0x14, handler);
        let array = words(&mut e, &bodies);
        e.mem.set_u32(this + 4, array);
        e.mem.set_u32(this + 8, 4);
        e.call(0x00d6_9d00, &args![this, 0u32]);
        let ends = [
            [3.0, 2.0, 3.0, 14.0],
            [1.0, 4.0, 3.0, 16.0],
            [1.0, 2.0, 5.0, 18.0],
        ];
        let mut expected = Vec::new();
        for body in [bodies[0], bodies[2], bodies[3]] {
            for (end, offset, colour) in [
                (ends[0], 0x20, 0xffff_0000u32),
                (ends[1], 0x30, 0xff00_8000),
                (ends[2], 0x10, 0xff00_00ff),
            ] {
                expected.push((centre, end, offset, colour, body + 0x10, 0x77));
            }
        }
        assert_eq!(*draws.borrow(), expected);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![
                LABEL_VIEWER,
                LABEL_MASS_AND_LINES,
                PROFILE_END,
                LABEL_DISPLAY_LINES,
                PROFILE_END,
                LABEL_MASS_AND_LINES,
                PROFILE_END,
                LABEL_DISPLAY_LINES,
                PROFILE_END,
                LABEL_MASS_AND_LINES,
                PROFILE_END,
                LABEL_DISPLAY_LINES,
                PROFILE_END,
                PROFILE_END,
            ]
        );
    }

    #[test]
    fn the_callback_without_bodies_only_writes_the_outer_records_00d69d00() {
        let mut e = engine();
        let (stream, records) = monitor(&mut e, 0x100);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0x24, 0);
        e.call(0x00d6_9d00, &args![block + 0x20, 0u32]);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_VIEWER, PROFILE_END]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00d6a140_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_a040, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_a140, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x3c]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x3c);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d6a140_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_a040, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_a140, &args![0x1234u32]).u32(), 0);
    }
}
