//! `Havok SDK/hkpconvexradiusviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexRadiusViewer` (name from the engine map; the class is not in the Xbox PDB's type list)
//! is a visual debugger viewer that shows the convex radius of every entity of the world. It has
//! several bases, so its functions take the address of the base they belong to as `this`: the
//! viewer itself at `V`, the entity listener at `V + 0x28` (vtable `010d20ac`) and the world's
//! post simulation listener at `V + 0x2C` (vtable `010d209c`). Read off the code (names
//! descriptive, unconfirmed): `V + 0x18` is the display handler, `V + 0x30`/`+0x34`/`+0x38` an
//! `hkArray` of entries `{key, hkArray<void*> ids}` (one per entity key), and `V + 0x3C`/`+0x40`
//! two colour words copied from the globals `01269018` and `01269014`. The process tag the
//! viewer was registered under is the global `01269010`.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) have no
//! /GS cookie and no exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};

/// The process tag the viewer is registered under (`hkProcessFactory::registerProcess`).
const PROCESS_TAG: u32 = 0x0126_9010;
/// The two colour words copied into a new viewer (to `+0x3C` and `+0x40`).
const COLOUR_AT_40: u32 = 0x0126_9014;
const COLOUR_AT_3C: u32 = 0x0126_9018;
/// The `hkProcessFactory` instance (`01269cd0`) and its `registerProcess` (`00dd9380`, `RET 8`:
/// `(name, creation function)`).
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
/// The process name `"Convex Radius"` and the function that makes the viewer (`00d68e00`).
const PROCESS_NAME: u32 = 0x010d_2070;
const CREATE_VIEWER: u32 = 0x00d6_8e00;
/// The profiling labels: `"TthkpConvexRadiusViewer"` and the end label `"Et"`.
const LABEL_VIEWER: u32 = 0x010d_2080;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpWorld::getCurrentTime` (`00c91020`, answers it in `ST0`).
const GET_CURRENT_TIME: u32 = 0x00c9_1020;
/// `hkpMotion::approxTransformAt` (`00c86490`, `RET 8`): `this` = the motion, `(time, out
/// transform)`.
const APPROX_TRANSFORM_AT: u32 = 0x00c8_6490;
/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The grow step of `hkArray::pushBack` (`00c90940`, cdecl): `(allocator, array, element size)`.
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkpWorldViewerBase`'s constructor (`00ddccb0`, `RET 4`), `hkpConvexRadiusBuilder`'s setup
/// record (`00de3050`) and constructor (`00de3060`, `RET 4`: the setup record), and its
/// `buildDisplayGeometries` (`00de4420`, `RET 8`: `(shape, display array)`).
const VIEWER_BASE_CONSTRUCTOR: u32 = 0x00dd_ccb0;
const BUILDER_SETUP: u32 = 0x00de_3050;
const BUILDER_CONSTRUCTOR: u32 = 0x00de_3060;
const BUILD_DISPLAY_GEOMETRIES: u32 = 0x00de_4420;
/// The string stream the error message is built in (`00cb1600`, `RET 8`: `(buffer, size)`), its
/// append (`00cc01c0`, `RET 4`) and its destructor (`00cc00e0`).
const STRING_STREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
const STRING_STREAM_APPEND: u32 = 0x00cc_01c0;
const STRING_STREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
/// The error reporter object (global `01268360`; `vtable + 0xC` reports `(0, -1, text, file,
/// line)`), the message, and the source file name and line the report names.
const ERROR_REPORTER: u32 = 0x0126_8360;
const ERROR_MESSAGE: u32 = 0x010d_196c;
const ERROR_FILE: u32 = 0x010d_2104;
const ERROR_LINE: u32 = 0xa7;
/// The vtables the constructor installs, in the order it writes them.
const VTABLE_ENTITY_LISTENER_BASE: u32 = 0x0102_f13c;
const VTABLE_POST_SIMULATION_LISTENER_BASE: u32 = 0x010c_cf7c;
const VTABLE_VIEWER: u32 = 0x010d_20f8;
const VTABLE_SECOND_BASE: u32 = 0x010d_20d8;
const VTABLE_THIRD_BASE: u32 = 0x010d_20c8;
const VTABLE_ENTITY_LISTENER: u32 = 0x010d_20ac;
const VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010d_209c;
/// The capacity word of an empty `hkArray` that owns nothing.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
/// The mask of the capacity bits of an `hkArray`'s capacity-and-flags word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
pub(crate) fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream (`TlsGetValue`
/// of the index at `0126835c`) while its cursor (`+4`) is below its limit (`+0xC`); a record is 12
/// bytes.
pub(crate) fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// `hkArray<void*>::pushBack`: the router, room for one more through `00c90940` when the size
/// equals the capacity, then the element.
fn push_back_pointer(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + size * 4, value);
    e.mem.set_u32(array + 4, size + 1);
}

// Translated from 00d68970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::postSimulationCallback` (Xbox PDB): between two profiling records
/// (`TthkpConvexRadiusViewer` and the end label), for every entity of every simulation island of
/// `world` (the arrays at `world + 0x28` and each island's at `+0x48`), asks the motion at
/// `entity + 0xE0` for its transform at the world's current time and hands it to the display
/// handler (`vtable + 0x10`: `(transform, entity + 0x13, process tag)`). `this` is the post
/// simulation listener base (`V + 0x2C`), the handler is at `this - 0x14`.
pub fn hkp_convex_radius_viewer_post_simulation_callback(e: &mut Engine, this: Ptr, world: Ptr) {
    let (this, world) = (this.addr(), world.addr());
    profile_record(e, LABEL_VIEWER);
    e.with_stack(0x40, |e, transform| {
        let transform = transform.addr();
        let mut island = 0;
        while island < e.mem.i32(world + 0x2c) {
            let islands = e.mem.u32(world + 0x28);
            let entities = e.mem.u32(islands + island as u32 * 4) + 0x48;
            let mut index = 0;
            while index < e.mem.i32(entities + 4) {
                let entity = e.mem.u32(e.mem.u32(entities) + index as u32 * 4);
                let time = e.call(GET_CURRENT_TIME, &args![world]).f32();
                let time = x87_quiet(time);
                e.call(APPROX_TRANSFORM_AT, &args![entity + 0xe0, time, transform]);
                let handler = e.mem.u32(this - 0x14);
                let tag: u32 = e.global(PROCESS_TAG);
                e.vcall(handler, 0x10, &args![transform, entity + 0x13, tag]);
                index += 1;
            }
            island += 1;
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d68a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the entry of the viewer (`this` = `V`) whose key (the first word of the entry the
/// array at `+0x30` points to) is `key`: its index, or -1.
pub fn fn_00d68a70(e: &mut Engine, this: Ptr, key: u32) -> i32 {
    let this = this.addr();
    let size = e.mem.i32(this + 0x34);
    let data = e.mem.u32(this + 0x30);
    for index in 0..size.max(0) {
        let entry = e.mem.u32(data + index as u32 * 4);
        if e.mem.u32(entry) == key {
            return index;
        }
    }
    -1
}

// Translated from 00d68aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// What the viewer does when an entity goes away (the Xbox PDB has no name for this body; it is
/// the counterpart of `entityAddedCallback`, unconfirmed): if the entity has a shape (`+0x10`)
/// and an entry for its key (`+8`), the display handler (`this - 0x10`, `vtable + 0x14`) is told
/// to remove the geometry `(entity + 0x13, process tag, 0)`, and that id is taken out of the
/// entry's list by moving the last one into its place. `this` is the entity listener base (`V +
/// 0x28`).
pub fn fn_00d68aa0(e: &mut Engine, this: Ptr, entity: Ptr) {
    let (this, entity) = (this.addr(), entity.addr());
    if e.mem.u32(entity + 0x10) == 0 {
        return;
    }
    let key = e.mem.u32(entity + 8);
    let index = fn_00d68a70(e, Ptr::new(this - 0x28), key);
    if index < 0 {
        return;
    }
    let entry = e.mem.u32(e.mem.u32(this + 8) + index as u32 * 4);
    let handler = e.mem.u32(this - 0x10);
    let tag: u32 = e.global(PROCESS_TAG);
    let id = entity + 0x13;
    e.vcall(handler, 0x14, &args![id, tag, 0u32]);
    let count = e.mem.i32(entry + 8);
    let data = e.mem.u32(entry + 4);
    for position in 0..count.max(0) {
        if e.mem.u32(data + position as u32 * 4) == id {
            let last = count - 1;
            e.mem.set_i32(entry + 8, last);
            if last != position {
                let moved = e.mem.u32(data + last as u32 * 4);
                e.mem.set_u32(data + position as u32 * 4, moved);
            }
            return;
        }
    }
}

// Translated from 00d68b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes every geometry of the viewer's entry number `index`: the display handler (`V + 0x18`,
/// `vtable + 0x14`) is told `(id, process tag, 0)` for each id of the entry's list, then the
/// memory router is asked for (nothing is done with its answer: the capacity test after it is
/// always false) and the list is emptied.
pub fn fn_00d68b20(e: &mut Engine, this: Ptr, index: u32) {
    let this = this.addr();
    let entry = e.mem.u32(e.mem.u32(this + 0x30) + index * 4);
    let mut position = 0;
    while position < e.mem.i32(entry + 8) {
        let handler = e.mem.u32(this + 0x18);
        let tag: u32 = e.global(PROCESS_TAG);
        let id = e.mem.u32(e.mem.u32(entry + 4) + position as u32 * 4);
        e.vcall(handler, 0x14, &args![id, tag, 0u32]);
        position += 1;
    }
    e.call(MEMORY_ROUTER, &args![]);
    e.mem.set_u32(entry + 8, 0);
}

// Translated from 00d68b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's constructor (`hkpWorldViewerBase`'s constructor on `this` with `process_list`,
/// then the vtables of the viewer and its bases, an empty entry array at `+0x30` and the two
/// colour words). Answers `this`.
pub fn fn_00d68b90(e: &mut Engine, this: Ptr, process_list: u32) -> Ptr {
    let this = this.addr();
    e.call(VIEWER_BASE_CONSTRUCTOR, &args![this, process_list]);
    let colour_3c: u32 = e.global(COLOUR_AT_3C);
    let colour_40: u32 = e.global(COLOUR_AT_40);
    e.mem.set_u32(this + 0x28, VTABLE_ENTITY_LISTENER_BASE);
    e.mem
        .set_u32(this + 0x2c, VTABLE_POST_SIMULATION_LISTENER_BASE);
    e.mem.set_u32(this, VTABLE_VIEWER);
    e.mem.set_u32(this + 8, VTABLE_SECOND_BASE);
    e.mem.set_u32(this + 0x20, VTABLE_THIRD_BASE);
    e.mem.set_u32(this + 0x28, VTABLE_ENTITY_LISTENER);
    e.mem.set_u32(this + 0x2c, VTABLE_POST_SIMULATION_LISTENER);
    e.mem.set_u32(this + 0x30, 0);
    e.mem.set_u32(this + 0x34, 0);
    e.mem.set_u32(this + 0x38, EMPTY_CAPACITY);
    e.mem.set_u32(this + 0x3c, colour_3c);
    e.mem.set_u32(this + 0x40, colour_40);
    Ptr::new(this)
}

// Translated from 00d68c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::entityAddedCallback` (Xbox PDB): for an entity with a shape (`+0x10`)
/// and an entry for its key (`+8`), builds the display geometries of the shape
/// (`hkpConvexRadiusBuilder::buildDisplayGeometries`, a builder made from the setup record);
/// every display of type 6 with no geometry (`+8`) is reported as an error ("Unable to build
/// display geometry from hkpShape geometry data", through the reporter at `01268360`) and
/// dropped. If any remain, the id `entity + 0x13` is added to the entry's list, the display
/// handler (`V + 0x18`) is given the displays (`vtable + 4`: `(array, entity + 0xF0, id, tag,
/// 0)`) and the colour (`vtable + 0xC`: the word at `V + 0x3C` for a motion type byte (`+0xE8`)
/// of 5, else the one at `V + 0x40`); the displays are then deleted (virtual destructor with flag
/// 1) and their array freed. `this` is the entity listener base (`V + 0x28`).
pub fn hkp_convex_radius_viewer_entity_added_callback(e: &mut Engine, this: Ptr, entity: Ptr) {
    let (this, entity) = (this.addr(), entity.addr());
    let shape = e.mem.u32(entity + 0x10);
    if shape == 0 {
        return;
    }
    let key = e.mem.u32(entity + 8);
    let index = fn_00d68a70(e, Ptr::new(this - 0x28), key);
    if index < 0 {
        return;
    }
    let entry = e.mem.u32(e.mem.u32(this + 8) + index as u32 * 4);
    // The frame: the display array at +0x10, the setup record at +0x20, the message stream at
    // +0x24, the builder at +0x30 and the message buffer at +0x40.
    e.with_stack(0x240, |e, frame| {
        let frame = frame.addr();
        let displays = frame + 0x10;
        let setup = frame + 0x20;
        let stream = frame + 0x24;
        let builder = frame + 0x30;
        let buffer = frame + 0x40;
        e.mem.set_u32(displays, 0);
        e.mem.set_u32(displays + 4, 0);
        e.mem.set_u32(displays + 8, EMPTY_CAPACITY);
        e.call(BUILDER_SETUP, &args![setup]);
        e.call(BUILDER_CONSTRUCTOR, &args![builder, setup]);
        e.call(BUILD_DISPLAY_GEOMETRIES, &args![builder, shape, displays]);
        let mut position = e.mem.i32(displays + 4) - 1;
        while position >= 0 {
            let display = e.mem.u32(e.mem.u32(displays) + position as u32 * 4);
            if e.mem.u32(display + 0x50) == 6 && e.mem.u32(display + 8) == 0 {
                e.call(STRING_STREAM_CONSTRUCTOR, &args![stream, buffer, 0x200u32]);
                e.call(STRING_STREAM_APPEND, &args![stream, ERROR_MESSAGE]);
                let reporter = e.mem.u32(ERROR_REPORTER);
                let arguments = args![0u32, -1i32, buffer, ERROR_FILE, ERROR_LINE];
                e.vcall(reporter, 0xc, &arguments);
                e.call(STRING_STREAM_DESTRUCTOR, &args![stream]);
                let last = e.mem.i32(displays + 4) - 1;
                e.mem.set_i32(displays + 4, last);
                if last != position {
                    let data = e.mem.u32(displays);
                    let moved = e.mem.u32(data + last as u32 * 4);
                    e.mem.set_u32(data + position as u32 * 4, moved);
                }
            }
            position -= 1;
        }
        if e.mem.i32(displays + 4) > 0 {
            let id = entity + 0x13;
            push_back_pointer(e, entry + 4, id);
            let handler = e.mem.u32(this - 0x10);
            let tag: u32 = e.global(PROCESS_TAG);
            e.vcall(handler, 4, &args![displays, entity + 0xf0, id, tag, 0u32]);
            let handler = e.mem.u32(this - 0x10);
            let colour = if e.mem.u8(entity + 0xe8) == 5 {
                e.mem.u32(this + 0x14)
            } else {
                e.mem.u32(this + 0x18)
            };
            let tag: u32 = e.global(PROCESS_TAG);
            e.vcall(handler, 0xc, &args![colour, id, tag]);
        }
        for position in 0..e.mem.i32(displays + 4) {
            let display = e.mem.u32(e.mem.u32(displays) + position as u32 * 4);
            if display != 0 {
                e.vcall(display, 0, &args![1u32]);
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(displays + 8);
        e.mem.set_u32(displays + 4, 0);
        if capacity & 0x8000_0000 == 0 {
            let data = e.mem.u32(displays);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK) << 2],
            );
        }
    });
}

// Translated from 00d68e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::registerViewer` (Xbox PDB): registers the process `"Convex Radius"`
/// with the process factory (`01269cd0`, creation function `00d68e00`) and keeps the tag it
/// answers in `01269010`.
pub fn hkp_convex_radius_viewer_register_viewer(e: &mut Engine) {
    let factory = e.mem.u32(PROCESS_FACTORY);
    let tag = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, CREATE_VIEWER],
        )
        .u32();
    e.set_global(PROCESS_TAG, tag);
}

// Translated from 00d68e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x44 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d68b90` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d68e46.
pub fn fn_00d68e00(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x44]).u32();
    e.mem.set_u16(block + 4, 0x44);
    let object = e.call(0x00d6_8b90, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d68970,
            hkp_convex_radius_viewer_post_simulation_callback(Ptr, Ptr)
        ),
        entry!(0x00d68a70, fn_00d68a70(Ptr, u32) -> i32),
        entry!(0x00d68aa0, fn_00d68aa0(Ptr, Ptr)),
        entry!(0x00d68b20, fn_00d68b20(Ptr, u32)),
        entry!(0x00d68b90, fn_00d68b90(Ptr, u32) -> Ptr),
        entry!(
            0x00d68c00,
            hkp_convex_radius_viewer_entity_added_callback(Ptr, Ptr)
        ),
        entry!(0x00d68e40, hkp_convex_radius_viewer_register_viewer()),
        entry!(0x00d6_8e00, fn_00d68e00(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use std::rc::Rc;

    /// Objects whose vtable slots log (slot `s` at `base + s`).
    const HANDLER: u32 = 0x0900_0000;
    const ALLOCATOR: u32 = 0x0910_0000;
    const REPORTER: u32 = 0x0920_0000;
    const DISPLAY: u32 = 0x0930_0000;
    const TAG: u32 = 0x77;

    fn setup() -> (Engine, Log) {
        let mut e = engine();
        e.set_global(PROCESS_TAG, TAG);
        (e, new_log())
    }

    /// A viewer with a display handler at `V + 0x18`; answers `(viewer, handler)`.
    fn viewer(e: &mut Engine, log: &Log) -> (u32, u32) {
        let handler = logging_object(e, log, HANDLER);
        let viewer = e.mem.alloc(0x50);
        e.mem.set_u32(viewer + 0x18, handler);
        (viewer, handler)
    }

    /// The entries of the viewer's array at `+0x30`: each `{key, ids data, ids size, ids
    /// capacity}`.
    fn entry(e: &mut Engine, key: u32, ids: &[u32]) -> u32 {
        let entry = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x20);
        for (index, id) in ids.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *id);
        }
        e.mem.set_u32(entry, key);
        e.mem.set_u32(entry + 4, data);
        e.mem.set_u32(entry + 8, ids.len() as u32);
        e.mem.set_u32(entry + 0xc, 8);
        entry
    }

    fn set_entries(e: &mut Engine, viewer: u32, entries: &[u32]) {
        let data = e.mem.alloc(0x20);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entry);
        }
        e.mem.set_u32(viewer + 0x30, data);
        e.mem.set_u32(viewer + 0x34, entries.len() as u32);
    }

    #[test]
    fn post_simulation_callback_hands_every_entity_transform_to_the_handler_00d68970() {
        let (mut e, log) = setup();
        let (stream, records) = monitor(&mut e, 0x100);
        let (viewer, handler) = viewer(&mut e, &log);
        // Two islands holding one and two entities.
        let entities: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x100)).collect();
        let world = e.mem.alloc(0x40);
        let islands = e.mem.alloc(8);
        e.mem.set_u32(world + 0x28, islands);
        e.mem.set_u32(world + 0x2c, 2);
        for (island, members) in [&entities[..1], &entities[1..]].into_iter().enumerate() {
            let record = e.mem.alloc(0x60);
            let data = e.mem.alloc(8);
            for (index, member) in members.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *member);
            }
            e.mem.set_u32(record + 0x48, data);
            e.mem.set_u32(record + 0x4c, members.len() as u32);
            e.mem.set_u32(islands + 4 * island as u32, record);
        }
        e.register(GET_CURRENT_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        log_double(&mut e, &log, APPROX_TRANSFORM_AT, 0);
        e.call(0x00d6_8970, &args![viewer + 0x2c, world]);
        let moves = calls(&log, APPROX_TRANSFORM_AT);
        assert_eq!(moves.len(), 3);
        for (call, entity) in moves.iter().zip(&entities) {
            assert_eq!((call[0], call[1]), (entity + 0xe0, 2.5f32.to_bits()));
        }
        let updates = calls(&log, HANDLER + 0x10);
        assert_eq!(updates.len(), 3);
        for ((call, entity), mv) in updates.iter().zip(&entities).zip(&moves) {
            assert_eq!(call, &vec![handler, mv[2], entity + 0x13, TAG]);
        }
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_VIEWER, LABEL_END]
        );
    }

    #[test]
    fn lookup_finds_the_entry_of_a_key_00d68a70() {
        let (mut e, _) = setup();
        let viewer = e.mem.alloc(0x50);
        let first = entry(&mut e, 0x10, &[]);
        let second = entry(&mut e, 0x20, &[]);
        set_entries(&mut e, viewer, &[first, second]);
        assert_eq!(e.call(0x00d6_8a70, &args![viewer, 0x20u32]).i32(), 1);
        assert_eq!(e.call(0x00d6_8a70, &args![viewer, 0x10u32]).i32(), 0);
        assert_eq!(e.call(0x00d6_8a70, &args![viewer, 0x30u32]).i32(), -1);
        e.mem.set_u32(viewer + 0x34, 0);
        assert_eq!(e.call(0x00d6_8a70, &args![viewer, 0x10u32]).i32(), -1);
    }

    #[test]
    fn removing_an_entity_takes_its_id_out_of_the_entry_00d68aa0() {
        let (mut e, log) = setup();
        let (viewer, handler) = viewer(&mut e, &log);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, 0x20);
        e.mem.set_u32(entity + 0x10, 0x1234);
        let id = entity + 0x13;
        let listed = entry(&mut e, 0x20, &[0x111, id, 0x333]);
        let other = entry(&mut e, 0x10, &[]);
        set_entries(&mut e, viewer, &[other, listed]);
        // `this` is the entity listener base, whose handler is 0x10 below it.
        let this = viewer + 0x28;
        e.mem.set_u32(this - 0x10, handler);
        e.call(0x00d6_8aa0, &args![this, entity]);
        assert_eq!(calls(&log, HANDLER + 0x14), vec![vec![handler, id, TAG, 0]]);
        // The last id took its place.
        assert_eq!(e.mem.u32(listed + 8), 2);
        let data = e.mem.u32(listed + 4);
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (0x111, 0x333));
        // An entity without a shape, or without an entry, does nothing.
        e.mem.set_u32(entity + 0x10, 0);
        e.call(0x00d6_8aa0, &args![this, entity]);
        e.mem.set_u32(entity + 0x10, 0x1234);
        e.mem.set_u32(entity + 8, 0x99);
        e.call(0x00d6_8aa0, &args![this, entity]);
        assert_eq!(calls(&log, HANDLER + 0x14).len(), 1);
    }

    #[test]
    fn clearing_an_entry_removes_all_its_geometries_00d68b20() {
        let (mut e, log) = setup();
        let (viewer, handler) = viewer(&mut e, &log);
        let router = e.mem.alloc(0x20);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        let first = entry(&mut e, 0x10, &[0xa1, 0xa2]);
        let second = entry(&mut e, 0x20, &[0xb1]);
        set_entries(&mut e, viewer, &[first, second]);
        e.call(0x00d6_8b20, &args![viewer, 0u32]);
        assert_eq!(
            calls(&log, HANDLER + 0x14),
            vec![vec![handler, 0xa1, TAG, 0], vec![handler, 0xa2, TAG, 0]]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(second + 8), 1);
        assert_eq!(calls(&log, MEMORY_ROUTER).len(), 1);
    }

    #[test]
    fn constructor_installs_the_vtables_and_colours_00d68b90() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, VIEWER_BASE_CONSTRUCTOR, 0);
        e.set_global(COLOUR_AT_3C, 0xc3c3u32);
        e.set_global(COLOUR_AT_40, 0xc4c4u32);
        let this = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00d6_8b90, &args![this, 0x5555u32]).u32(), this);
        assert_eq!(
            calls(&log, VIEWER_BASE_CONSTRUCTOR),
            vec![vec![this, 0x5555]]
        );
        let words: Vec<u32> = [0, 8, 0x20, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![
                VTABLE_VIEWER,
                VTABLE_SECOND_BASE,
                VTABLE_THIRD_BASE,
                VTABLE_ENTITY_LISTENER,
                VTABLE_POST_SIMULATION_LISTENER,
                0,
                0,
                EMPTY_CAPACITY,
                0xc3c3,
                0xc4c4
            ]
        );
    }

    /// Display objects (a logging vtable at `DISPLAY`, the destructor at slot 0) of the given
    /// `(type, geometry)`.
    fn display(e: &mut Engine, log: &Log, kind: u32, geometry: u32) -> u32 {
        // The display is larger than the logging object: its type is at +0x50.
        let logging = logging_object(e, log, DISPLAY);
        let object = e.mem.alloc(0x60);
        let vtable = e.mem.u32(logging);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x50, kind);
        e.mem.set_u32(object + 8, geometry);
        object
    }

    #[test]
    fn entity_added_keeps_good_displays_and_reports_bad_ones_00d68c00() {
        let (mut e, log) = setup();
        let (viewer, handler) = viewer(&mut e, &log);
        let entity = e.mem.alloc(0x100);
        e.mem.set_u32(entity + 8, 0x20);
        e.mem.set_u32(entity + 0x10, 0x4321);
        let motion_type_5 = [5u8, 3];
        let ids = entry(&mut e, 0x20, &[]);
        e.mem.set_u32(ids + 0xc, 0);
        set_entries(&mut e, viewer, &[ids]);
        let this = viewer + 0x28;
        e.mem.set_u32(this - 0x10, handler);
        e.mem.set_u32(this + 0x14, 0xc3c3);
        e.mem.set_u32(this + 0x18, 0xc4c4);
        // The builder makes a good display and a bad one (type 6 without geometry).
        let good = display(&mut e, &log, 1, 0);
        let bad = display(&mut e, &log, 6, 0);
        let notes = Rc::clone(&log);
        e.register_double(BUILD_DISPLAY_GEOMETRIES, move |e, a| {
            notes
                .borrow_mut()
                .push((BUILD_DISPLAY_GEOMETRIES, a.to_vec()));
            let data = e.mem.alloc(8);
            e.mem.set_u32(data, good);
            e.mem.set_u32(data + 4, bad);
            e.mem.set_u32(a[2], data);
            e.mem.set_u32(a[2] + 4, 2);
            e.mem.set_u32(a[2] + 8, 2);
            Ret::default()
        });
        for address in [
            BUILDER_SETUP,
            BUILDER_CONSTRUCTOR,
            STRING_STREAM_CONSTRUCTOR,
            STRING_STREAM_APPEND,
            STRING_STREAM_DESTRUCTOR,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        let reporter = logging_object(&mut e, &log, REPORTER);
        e.mem.set_u32(ERROR_REPORTER, reporter);
        let router = e.mem.alloc(0x20);
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        let notes = Rc::clone(&log);
        e.register_double(ARRAY_GROW, move |e, a| {
            notes.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        for motion in motion_type_5 {
            e.mem.set_u8(entity + 0xe8, motion);
            log.borrow_mut().clear();
            e.mem.set_u32(ids + 4, 0);
            e.mem.set_u32(ids + 8, 0);
            e.mem.set_u32(ids + 0xc, 0);
            e.call(0x00d6_8c00, &args![this, entity]);
            let id = entity + 0x13;
            // The builder was handed the shape and the display array.
            let built = &calls(&log, BUILD_DISPLAY_GEOMETRIES)[0];
            assert_eq!(built[1], 0x4321);
            // The bad display was reported through the stream and dropped.
            let report = &calls(&log, REPORTER + 0xc)[0];
            assert_eq!(report[1..4], [0, 0xffff_ffff, report[3]]);
            assert_eq!((report[4], report[5]), (ERROR_FILE, ERROR_LINE));
            assert_eq!(calls(&log, STRING_STREAM_APPEND)[0][1], ERROR_MESSAGE);
            assert_eq!(calls(&log, STRING_STREAM_DESTRUCTOR).len(), 1);
            // The id was added, the handler was given the displays and the colour.
            assert_eq!(e.mem.u32(ids + 8), 1);
            assert_eq!(e.mem.u32(e.mem.u32(ids + 4)), id);
            let added = &calls(&log, HANDLER + 4)[0];
            assert_eq!(&added[2..], &[entity + 0xf0, id, TAG, 0]);
            let colour = if motion == 5 { 0xc3c3 } else { 0xc4c4 };
            assert_eq!(
                calls(&log, HANDLER + 0xc),
                vec![vec![handler, colour, id, TAG]]
            );
            // The surviving display was deleted, the array freed.
            assert_eq!(calls(&log, DISPLAY), vec![vec![good, 1]]);
            assert_eq!(calls(&log, ALLOCATOR + 0x10).len(), 1);
            assert_eq!(calls(&log, ALLOCATOR + 0x10)[0][2], 8);
        }
    }

    #[test]
    fn registering_the_viewer_keeps_the_process_tag_00d68e40() {
        let (mut e, log) = setup();
        e.mem.set_u32(PROCESS_FACTORY, 0x6000);
        let notes = Rc::clone(&log);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            notes.borrow_mut().push((REGISTER_PROCESS, a.to_vec()));
            Ret {
                eax: 0x42,
                ..Ret::default()
            }
        });
        e.call(0x00d6_8e40, &args![]);
        assert_eq!(
            calls(&log, REGISTER_PROCESS),
            vec![vec![0x6000, PROCESS_NAME, CREATE_VIEWER]]
        );
        assert_eq!(e.mem.u32(PROCESS_TAG), 0x42);
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
    fn fn_00d68e00_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_8b90, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_8e00, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x44]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x44);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d68e00_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_8b90, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_8e00, &args![0x1234u32]).u32(), 0);
    }
}
