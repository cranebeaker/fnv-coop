//! `Havok SDK/hkpshapedisplayviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, store_bits, PROFILE_END};

/// `TthkpShapeDisplayViewer`, the label of the callback's timer record.
const LABEL_VIEWER: u32 = 0x010d_1a70;
/// The id `hkProcessFactory::registerProcess` gave the viewer ("Shapes"), kept at `01268fe0`.
const PROCESS_ID: u32 = 0x0126_8fe0;
/// `hkpWorldObject::getProperty` (`00c6d840`, thiscall on the object: output pair, key, flag,
/// `RET 0xc`; answers the output pair, whose first word is the value) and `removeProperty`
/// (`00cd2a50`, `(output pair, key)`).
const GET_PROPERTY: u32 = 0x00c6_d840;
const REMOVE_PROPERTY: u32 = 0x00cd_2a50;
/// The memory router (`00c85750`): its `+0x10` word is the allocator.
const ROUTER: u32 = 0x00c8_5750;
/// `hkMapBase<unsigned long, unsigned long>` methods: `getWithDefault` (`00cb1930`, thiscall:
/// key, default, `RET 8`), insert (`00cb1870`: allocator, key, value, `RET 0xC`), remove
/// (`00cb1fb0`: key, `RET 4`) and reserve (`00cb1fe0`: allocator, count, `RET 8`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_REMOVE: u32 = 0x00cb_1fb0;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// The array growth helpers (cdecl): `00c90940` (allocator, array, element size) and
/// `00c908c0` (allocator, array, new capacity, element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
const ARRAY_RESIZE: u32 = 0x00c9_08c0;
/// `hkpMotion::approxTransformAt` (`00c86490`, thiscall: time, output) and
/// `hkpWorld::getCurrentTime` (`00c91020`, thiscall, answers a `float` in `st0`).
const APPROX_TRANSFORM_AT: u32 = 0x00c8_6490;
const GET_CURRENT_TIME: u32 = 0x00c9_1020;
/// `hkpWorldViewerBase::hkpWorldViewerBase` (`00ddccb0`, thiscall, one word `RET 4`) and
/// `hkString::strCmp` (`00cb0fa0`, cdecl).
const VIEWER_BASE_CONSTRUCTOR: u32 = 0x00dd_ccb0;
const STR_CMP: u32 = 0x00cb_0fa0;
/// `hkProcessFactory::registerProcess` (`00dd9380`, thiscall on the factory at `01269cd0`:
/// name, creation function, `RET 8`).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
/// `hkpWorld::addEntityListener` and `addWorldPostSimulationListener` (thiscall on the world,
/// one word, `RET 4`).
const ADD_ENTITY_LISTENER: u32 = 0x00c9_3820;
const ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
/// The builder pieces: environment constructor (`00ddfb70`, fastcall), builder constructor
/// (`00ddfb80`, one word), `buildDisplayGeometries` (`00de0f90`, shape, array) and the
/// display convex constructor (`00de1d20`, one word, answers the object).
const BUILDER_ENVIRONMENT: u32 = 0x00dd_fb70;
const BUILDER_CONSTRUCTOR: u32 = 0x00dd_fb80;
const BUILD_DISPLAY_GEOMETRIES: u32 = 0x00de_0f90;
const DISPLAY_CONVEX_CONSTRUCTOR: u32 = 0x00de_1d20;
/// The string buffer helpers (`00cb1600` constructor: buffer, capacity; `00cc01c0` append:
/// text; `00cc00e0` destructor) and the diagnostic sink at `01268360` (virtual slot `0xC`:
/// kind, id, text, file, line).
const STRING_BUFFER_CONSTRUCTOR: u32 = 0x00cb_1600;
const STRING_BUFFER_APPEND: u32 = 0x00cc_01c0;
const STRING_BUFFER_DESTRUCTOR: u32 = 0x00cc_00e0;
const DIAGNOSTIC_SINK: u32 = 0x0126_8360;
/// The message text and the source file name the diagnostic carries.
const BUILD_FAILED_TEXT: u32 = 0x010d_196c;
const SOURCE_FILE_TEXT: u32 = 0x010d_1b18;
/// `hkReferencedObject::removeReference` as the exe calls it on the shape property (`00c905b0`,
/// thiscall on the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The name the viewer looks its options object up by (`ShapeDisplayViewerOptions`).
const OPTIONS_NAME: u32 = 0x010d_1a88;
/// The registered creation function for the "Shapes" process and its name.
const CREATE_VIEWER: u32 = 0x00d6_5910;
const PROCESS_NAME: u32 = 0x010c_41b0;

/// Property keys of an `hkpWorldObject` the viewer reads (the 16-byte records at `+0x78`,
/// count at `+0x7C`; the value is the word at `+8` of a record).
const KEY_DISPLAY_ID: u32 = 0x1134;
const KEY_1135: u32 = 0x1135;

/// Index of the first property record of `object` with `key`.
fn find_property(e: &Engine, object: u32, key: u32) -> Option<u32> {
    let count = e.mem.i32(object + 0x7c);
    let mut index = 0;
    if count > 0 {
        let records = e.mem.u32(object + 0x78);
        loop {
            if e.mem.u32(records + 16 * index) == key {
                return Some(index);
            }
            index += 1;
            if index as i32 >= count {
                break;
            }
        }
    }
    None
}

/// The allocator (`+0x10`) of the memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Appends `value` to the `hkArray` at `array` (data `+0`, size `+4`, capacity and flags
/// `+8`): the router is asked first, the array grows by 4-byte elements when it is full.
fn push_back(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + 4 * size, value);
    e.mem.set_u32(array + 4, size + 1);
}

/// Tells the entity listener at `listener` about every entity of the simulation island
/// `island` (the entity array is at `+0x48`, count at `+0x4C`): slot 4, one entity each.
fn announce_island(e: &mut Engine, listener: u32, island: u32) {
    let mut index = 0;
    while index < e.mem.i32(island + 0x4c) {
        let data = e.mem.u32(island + 0x48);
        let entity = e.mem.u32(data + 4 * index as u32);
        e.vcall(listener, 4, &args![entity]);
        index += 1;
    }
}

// Translated from 00d652c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Registers in: `EAX` = `object`, an `hkpWorldObject` (its property records at
/// `+0x78`/`+0x7C`). Two stack words, popped by the caller: `use_first` (a byte) and `out`
/// (a byte pointer). Reads the values of the properties `0x1135`, `0x1235` and `0x1234` and
/// sets `*out` to 0. With `use_first` set and a record `0x1234` present, answers its value,
/// or, when that value is 0, sets `*out` to 1 and answers 0. Otherwise the same for the
/// record `0x1235`, then for `0x1135`. Without any of the records the answer is 0.
pub fn fn_00d652c0(e: &mut Engine, object: Ptr, use_first: u8, out: Ptr) -> u32 {
    let object = object.addr();
    let out = out.addr();
    let (a_1135, a_1235, a_1234) = e.with_stack(0x10, |e, frame| {
        let first = frame.addr();
        let second = frame.addr() + 8;
        let found = e
            .call(GET_PROPERTY, &args![object, first, 0x1135u32, 0u32])
            .u32();
        let value_1135 = e.mem.u32(found);
        e.mem.set_u32(first, value_1135);
        let found = e
            .call(GET_PROPERTY, &args![object, second, 0x1235u32, 0u32])
            .u32();
        let value_1235 = e.mem.u32(found);
        let found = e
            .call(GET_PROPERTY, &args![object, second, 0x1234u32, 0u32])
            .u32();
        let value_1234 = e.mem.u32(found);
        (e.mem.u32(first), value_1235, value_1234)
    });
    e.mem.set_u8(out, 0);
    if use_first != 0 && find_property(e, object, 0x1234).is_some() {
        if a_1234 != 0 {
            return a_1234;
        }
        e.mem.set_u8(out, 1);
        return a_1234;
    }
    if find_property(e, object, 0x1235).is_some() {
        if a_1235 != 0 {
            return a_1235;
        }
        e.mem.set_u8(out, 1);
        return 0;
    }
    if find_property(e, object, 0x1135).is_some() {
        if a_1135 != 0 {
            return a_1135;
        }
        e.mem.set_u8(out, 1);
        return 0;
    }
    0
}

// Translated from 00d653f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayViewer::synchronizeTransforms` (Xbox PDB), thiscall with two stack words
/// (`RET 8`): the display `handler` and the `world`. The time is the `float` at `+0x68` when it
/// is above 0 (an unordered compare takes the other way), otherwise the world's current time
/// (`00c91020`, through the x87 stack). Handler slot `0x48` begins the update; every entity of
/// the world's first island array (data `+0x28`, count `+0x2C`) and, through the entity array
/// of each island at `+0x48`, every entity gets its transform at that time
/// (`hkpMotion::approxTransformAt` on `+0xE0`), handed to handler slot `0x10` with the entity's
/// display id (the second word of the `0x1134` record, or `entity + 0x10` without one) and the
/// process id. Slot `0x4C` ends the update.
pub fn hkp_shape_display_viewer_synchronize_transforms(
    e: &mut Engine,
    this: Ptr,
    handler: Ptr,
    world: Ptr,
) {
    let this = this.addr();
    let handler = handler.addr();
    let world = world.addr();
    let stored = f32::from_bits(e.mem.u32(this + 0x68));
    let time_bits = if 0.0 < stored {
        x87_quiet_bits(stored.to_bits())
    } else {
        let now = e.call(GET_CURRENT_TIME, &args![world]).st0;
        (now as f32).to_bits()
    };
    e.vcall(handler, 0x48, &args![]);
    e.with_stack(0x50, |e, transform| {
        let transform = transform.addr();
        let mut island_index = 0;
        while island_index < e.mem.i32(world + 0x2c) {
            let islands = e.mem.u32(world + 0x28);
            let island = e.mem.u32(islands + 4 * island_index as u32);
            let entities = island + 0x48;
            let mut entity_index = 0;
            while entity_index < e.mem.i32(entities + 4) {
                let data = e.mem.u32(entities);
                let entity = e.mem.u32(data + 4 * entity_index as u32);
                let record = match find_property(e, entity, KEY_DISPLAY_ID) {
                    Some(_) => match find_property(e, entity, KEY_DISPLAY_ID) {
                        Some(found) => {
                            let records = e.mem.u32(entity + 0x78);
                            e.mem.u32(records + 16 * found + 8)
                        }
                        None => 0,
                    },
                    None => entity + 0x10,
                };
                e.call(
                    APPROX_TRANSFORM_AT,
                    &args![entity + 0xe0, time_bits, transform],
                );
                let process_id: u32 = e.global(PROCESS_ID);
                e.vcall(handler, 0x10, &args![transform, record, process_id]);
                entity_index += 1;
            }
            island_index += 1;
        }
    });
    e.vcall(handler, 0x4c, &args![]);
}

// Translated from 00d65520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayViewer::postSimulationCallback` (Xbox PDB), thiscall with one stack word
/// (`RET 4`); `this` is the post-simulation listener embedded at `+0x2C` of the viewer. Writes
/// the timer record `TthkpShapeDisplayViewer`; when the viewer's flag byte (`+0x34` of the
/// listener, `+0x60` of the viewer) is set, virtual slot `0x10` of the viewer is called with
/// the word at `listener - 0x14` and the argument. Writes the end record.
pub fn hkp_shape_display_viewer_post_simulation_callback(e: &mut Engine, this: Ptr, world: u32) {
    let this = this.addr();
    profile_record(e, LABEL_VIEWER);
    if e.mem.u8(this + 0x34) != 0 {
        let target = e.mem.u32(this - 0x14);
        e.vcall(this - 0x2c, 0x10, &args![target, world]);
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00d655e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall with one stack word (`RET 4`): the index of the first entry of the
/// pointer array at `+0x30` (count `+0x34`) whose first word equals `key`, or -1.
pub fn fn_00d655e0(e: &mut Engine, this: Ptr, key: u32) -> i32 {
    let this = this.addr();
    let count = e.mem.i32(this + 0x34);
    let mut index = 0;
    if count > 0 {
        let entries = e.mem.u32(this + 0x30);
        loop {
            let entry = e.mem.u32(entries + 4 * index as u32);
            if e.mem.u32(entry) == key {
                return index;
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    -1
}

// Translated from 00d65610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall with one stack word (`RET 4`); `this` is the sub-object embedded at
/// `+0x2C` of the viewer (the display handler is the word at `this - 0x14`). Calls virtual slot
/// `0x10` of the display handler with `(body + 0xF0, display id of the body, process id)`.
pub fn fn_00d65610(e: &mut Engine, this: Ptr, body: Ptr) {
    let handler = e.mem.u32(this.addr() - 0x14);
    let vtable = e.mem.u32(handler);
    let process_id: u32 = e.global(PROCESS_ID);
    let id = fn_00d66180(e, body);
    let target = e.mem.u32(vtable + 0x10);
    e.call(target, &args![handler, body.addr() + 0xf0, id, process_id]);
}

// Translated from 00d65640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall with one stack word (`RET 4`); `this` is the entity listener embedded at
/// `+0x28` of the viewer. Looks up the body's list (`00d655e0` on `this - 0x28` with the word
/// at `body + 8`); without one it ends. Hands the body's display id and shape to virtual slot
/// `0x14` of the display handler at `this - 0x10`, drops the use counts of the shape in the two
/// maps (`this + 0x14`, `this + 0x20`) when the option byte at `+0x39` is set, and removes the
/// id from the body's list (the last entry takes its place).
pub fn fn_00d65640(e: &mut Engine, this: Ptr, body: Ptr) {
    let this = this.addr();
    let body = body.addr();
    let use_first = e.mem.u32(this - 0x18) == 0 && e.mem.u32(this - 0x14) == 0;
    let mut shape = e.with_stack(4, |e, out| {
        fn_00d652c0(e, Ptr::new(body), u8::from(use_first), out)
    });
    let key = e.mem.u32(body + 8);
    let index = fn_00d655e0(e, Ptr::new(this - 0x28), key);
    if index < 0 {
        return;
    }
    let lists = e.mem.u32(this + 8);
    let list = e.mem.u32(lists + 4 * index as u32);
    if shape == 0 {
        shape = e.mem.u32(body + 0x10);
    }
    let id = fn_00d66180(e, Ptr::new(body));
    let display = e.mem.u32(this - 0x10);
    let process_id: u32 = e.global(PROCESS_ID);
    e.vcall(display, 0x14, &args![id, process_id, shape]);
    if e.mem.u8(this + 0x39) != 0 && shape != 0 {
        let map = this + 0x20;
        let count = e.call(MAP_GET_WITH_DEFAULT, &args![map, shape, 0u32]).u32();
        let remaining = count.wrapping_sub(1);
        if remaining == 0 {
            e.call(MAP_REMOVE, &args![this + 0x14, shape]);
            e.call(MAP_REMOVE, &args![map, shape]);
        } else {
            let allocator = router_allocator(e);
            e.call(MAP_INSERT, &args![map, allocator, shape, remaining]);
        }
    }
    let size = e.mem.i32(list + 8);
    let mut at = 0;
    let mut found = false;
    if size > 0 {
        let data = e.mem.u32(list + 4);
        loop {
            if e.mem.u32(data + 4 * at as u32) == id {
                found = true;
                break;
            }
            at += 1;
            if at >= size {
                break;
            }
        }
    }
    if found && at >= 0 {
        let last = size - 1;
        e.mem.set_i32(list + 8, last);
        if last != at {
            let data = e.mem.u32(list + 4);
            let moved = e.mem.u32(data + 4 * last as u32);
            e.mem.set_u32(data + 4 * at as u32, moved);
        }
    }
}

// Translated from 00d65750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall with one stack word (`RET 4`): empties the list number `index` of the
/// viewer (the pointer array at `+0x30`). Every item `i` of the list (array at `+4`, count at
/// `+8`) is handed to virtual slot `0x14` of the display handler at `+0x18` as
/// `(item, process id, first word of the item)`; then the list's size becomes 0. The array
/// shrink the exe has after the router call (capacity word negative after masking) can not
/// happen and is kept as written.
pub fn fn_00d65750(e: &mut Engine, this: Ptr, index: u32) {
    let this = this.addr();
    let lists = e.mem.u32(this + 0x30);
    let list = e.mem.u32(lists + 4 * index);
    let mut at = 0;
    while at < e.mem.i32(list + 8) {
        let data = e.mem.u32(list + 4);
        let item = e.mem.u32(data + 4 * at as u32);
        let first = e.mem.u32(item);
        let handler = e.mem.u32(this + 0x18);
        let process_id: u32 = e.global(PROCESS_ID);
        e.vcall(handler, 0x14, &args![item, process_id, first]);
        at += 1;
    }
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(list + 0xc) & 0x3fff_ffff) as i32;
    if capacity < 0 {
        let doubled = capacity.wrapping_mul(2);
        let new_capacity = if doubled <= 0 { 0 } else { doubled };
        e.call(
            ARRAY_RESIZE,
            &args![allocator, list + 4, new_capacity, 4u32],
        );
    }
    e.mem.set_u32(list + 8, 0);
}

// Translated from 00d657d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayViewer::hkpShapeDisplayViewer` (Xbox PDB), thiscall with one stack word
/// (`RET 4`): the list of process objects `processes` (data `+0`, count `+4`). Runs the viewer
/// base constructor (`00ddccb0`) and fills in the vtables (`+0`, `+8`, `+0x20`, and the
/// listeners at `+0x28` and `+0x2C`, the first of which is stored twice), the empty array at
/// `+0x30` and three empty maps at `+0x3C`, `+0x48`, `+0x54` (reserved through the
/// router's allocator), the option bytes `+0x60..0x66`, the `float` at `+0x68`, the words at
/// `+0x6C` and `+0x70`. When one of the process objects names itself
/// `ShapeDisplayViewerOptions` (virtual slot 4, `strCmp`), the byte at `+0x60` is taken from the
/// byte at `+0x58` of the object's start (the object pointer is a secondary base, so the
/// object starts 8 bytes earlier). Answers `this`.
pub fn hkp_shape_display_viewer_constructor(e: &mut Engine, this: Ptr, processes: Ptr) -> Ptr {
    let at = this.addr();
    let processes = processes.addr();
    e.call(VIEWER_BASE_CONSTRUCTOR, &args![at, processes]);
    e.mem.set_u32(at + 0x28, 0x0102_f13c);
    e.mem.set_u32(at + 0x2c, 0x010c_cf7c);
    e.mem.set_u32(at, 0x010d_1b04);
    e.mem.set_u32(at + 8, 0x010d_1ae4);
    e.mem.set_u32(at + 0x20, 0x010d_1ad4);
    e.mem.set_u32(at + 0x28, 0x010d_1ab8);
    e.mem.set_u32(at + 0x2c, 0x010d_1aa8);
    e.mem.set_u32(at + 0x30, 0);
    e.mem.set_u32(at + 0x34, 0);
    e.mem.set_u32(at + 0x38, 0x8000_0000);
    for map in [at + 0x3c, at + 0x48, at + 0x54] {
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let allocator = router_allocator(e);
        e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    }
    let start = x87_quiet_bits(e.global::<u32>(0x0101_2054));
    let hold_a: u32 = e.global(0x0126_8fe8);
    let hold_b: u32 = e.global(0x0126_8fe4);
    for (offset, value) in [
        (0x60, 1),
        (0x61, 0),
        (0x62, 0),
        (0x63, 1),
        (0x64, 1),
        (0x65, 1),
        (0x66, 0),
    ] {
        e.mem.set_u8(at + offset, value);
    }
    e.mem.set_u32(at + 0x68, start);
    e.mem.set_u32(at + 0x6c, hold_a);
    e.mem.set_u32(at + 0x70, hold_b);
    let count = e.mem.i32(processes + 4);
    let mut index = 0;
    if count > 0 {
        loop {
            let data = e.mem.u32(processes);
            let process = e.mem.u32(data + 4 * index as u32);
            let name = e.vcall(process, 4, &args![]).u32();
            let different = e.call(STR_CMP, &args![OPTIONS_NAME, name]).i32();
            if different == 0 {
                let data = e.mem.u32(processes);
                let process = e.mem.u32(data + 4 * index as u32);
                let start = if process != 0 {
                    process.wrapping_sub(8)
                } else {
                    0
                };
                let option = e.mem.u8(start.wrapping_add(0x58));
                e.mem.set_u8(at + 0x60, option);
                break;
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    this
}

// Translated from 00d65950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, no arguments: registers the process `Shapes` (creation function `00d65910`) with
/// the process factory at `01269cd0` and keeps the id at `01268fe0`. Answers the id.
pub fn fn_00d65950(e: &mut Engine) -> u32 {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, CREATE_VIEWER],
        )
        .u32();
    e.set_global(PROCESS_ID, id);
    id
}

// Translated from 00d65970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall with one stack word (`RET 4`): the `world`. Registers the viewer's entity
/// listener (`this + 0x28`, or 0 for a null viewer) and post-simulation listener
/// (`this + 0x2C`) with the world, appends a new 16-byte record (the world at `+0`, an empty
/// array at `+4`) to the array at `+0x30`, then announces every entity of the world's island
/// arrays (`+0x28`/`+0x2C`, `+0x34`/`+0x38`) and of its fixed island (`+0x20`) to the entity
/// listener (virtual slot 4). The record's allocation is not checked for failure.
pub fn fn_00d65970(e: &mut Engine, this: Ptr, world: Ptr) {
    let this = this.addr();
    let world = world.addr();
    let entity_listener = if this != 0 { this + 0x28 } else { 0 };
    e.call(ADD_ENTITY_LISTENER, &args![world, entity_listener]);
    let post_listener = if this != 0 { this + 0x2c } else { 0 };
    e.call(ADD_POST_SIMULATION_LISTENER, &args![world, post_listener]);
    let allocator = router_allocator(e);
    let allocated = e.vcall(allocator, 4, &args![0x10u32]).u32();
    let mut record = 0;
    if allocated != 0 {
        e.mem.set_u32(allocated + 4, 0);
        e.mem.set_u32(allocated + 8, 0);
        e.mem.set_u32(allocated + 0xc, 0x8000_0000);
        record = allocated;
    }
    e.mem.set_u32(record, world);
    push_back(e, this + 0x30, record);
    let mut index = 0;
    while index < e.mem.i32(world + 0x2c) {
        let islands = e.mem.u32(world + 0x28);
        let island = e.mem.u32(islands + 4 * index as u32);
        announce_island(e, this + 0x28, island);
        index += 1;
    }
    let mut index = 0;
    while index < e.mem.i32(world + 0x38) {
        let islands = e.mem.u32(world + 0x34);
        let island = e.mem.u32(islands + 4 * index as u32);
        announce_island(e, this + 0x28, island);
        index += 1;
    }
    let fixed = e.mem.u32(world + 0x20);
    if fixed != 0 {
        announce_island(e, this + 0x28, fixed);
    }
}

// Translated from 00d65ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayViewer::entityAddedCallback` (Xbox PDB), thiscall with one stack word
/// (`RET 4`): the `body`. `this` is the entity listener embedded at `+0x28` of the viewer; the
/// words before it are the viewer's (`-0x28` start, `-0x18`/`-0x14` the two option words, `-0x10`
/// the display handler). The exe keeps a 0x290-byte frame: the byte flag of `00d652c0` at
/// `+0x16`, "built here" at `+0x17`, the geometry array (hkInplaceArray of 8, data `+0x38`,
/// size `+0x3C`, capacity word `+0x40` = `0x80000008`, store `+0x44`), builder environment
/// `+0x20`, builder `+0x7C`, text object `+0x28`, text buffer `+0x90` (0x200 bytes), property
/// output pairs `+0x28`, `+0x68` and `+0x70`.
///
/// It reads the shape property value with `00d652c0` (`this - 0x18`/`this - 0x14` both 0 asks
/// for the first record), ends when that sets its byte or when the body has no list, notes the
/// shape's use count in the maps at `this + 0x2C` (option `+0x3A`) and `this + 0x14` (option
/// `+0x39`), and, when the shape is new and options `+0x3B` and `+0x3C` are set, builds the
/// geometries: with neither of the properties `0x1136` / `0x1131` the builder runs, geometries
/// of type 6 without a user pointer are reported and dropped, and the shape property (`0x1135`)
/// is released when it is the one read first; otherwise a display convex with an identity
/// transform is made from the geometry property. The display id (`00d66180`) goes into the
/// body's list; the display handler is told about the shape through slot 4 (new geometry,
/// `(array, body + 0xF0, id, process id, shape)`), slot 8 (a shape already seen:
/// `(count, ...)`) and slot `0xC` (the display colour/offset property `0x1130`).
/// Afterwards the geometries are released through their destructors and the array is freed.
pub fn hkp_shape_display_viewer_entity_added_callback(e: &mut Engine, this: Ptr, body: Ptr) {
    let this = this.addr();
    let body = body.addr();
    e.with_stack(0x300, |e, frame| {
        let s = frame.addr();
        let use_first = e.mem.u32(this - 0x18) == 0 && e.mem.u32(this - 0x14) == 0;
        let value = fn_00d652c0(e, Ptr::new(body), u8::from(use_first), Ptr::new(s + 0x16));
        if e.mem.u8(s + 0x16) != 0 {
            return;
        }
        e.mem.set_u32(s + 0x68, value);
        let id_key = e.mem.u32(body + 8);
        let index = fn_00d655e0(e, Ptr::new(this - 0x28), id_key);
        if index < 0 {
            return;
        }
        let lists = e.mem.u32(this + 8);
        let list = e.mem.u32(lists + 4 * index as u32);
        let mut shape = value;
        if value == 0 {
            shape = e.mem.u32(body + 0x10);
            if shape == 0 {
                return;
            }
        }
        e.mem.set_u8(s + 0x17, 0);
        if e.mem.u8(this + 0x3a) != 0 {
            let seen = e
                .call(MAP_GET_WITH_DEFAULT, &args![this + 0x2c, shape, 0u32])
                .u32();
            e.mem.set_u8(s + 0x17, u8::from(seen != 0));
            if seen == 0 {
                let allocator = router_allocator(e);
                e.call(MAP_INSERT, &args![this + 0x2c, allocator, shape, 1u32]);
            }
        }
        let mut count = 0;
        if e.mem.u8(this + 0x39) != 0 {
            count = e
                .call(MAP_GET_WITH_DEFAULT, &args![this + 0x14, shape, 0u32])
                .u32();
        }
        e.mem.set_u32(s + 0x38, s + 0x44);
        e.mem.set_u32(s + 0x3c, 0);
        e.mem.set_u32(s + 0x40, 0x8000_0008);
        e.mem.set_u8(s + 0x16, 1);
        if count == 0
            && e.mem.u8(s + 0x17) == 0
            && e.mem.u8(this + 0x3b) != 0
            && e.mem.u8(this + 0x3c) != 0
        {
            let found = e
                .call(GET_PROPERTY, &args![body, s + 0x70, 0x1136u32, 0u32])
                .u32();
            let first = e.mem.u32(found);
            let found = e
                .call(GET_PROPERTY, &args![body, s + 0x28, 0x1131u32, 0u32])
                .u32();
            let second = e.mem.u32(found);
            e.mem.set_u32(s + 0x20, second);
            if first == 0 && second == 0 {
                e.call(BUILDER_ENVIRONMENT, &args![s + 0x20]);
                e.call(BUILDER_CONSTRUCTOR, &args![s + 0x7c, s + 0x20]);
                e.call(BUILD_DISPLAY_GEOMETRIES, &args![s + 0x7c, shape, s + 0x38]);
                let mut at = e.mem.i32(s + 0x3c) - 1;
                while at >= 0 {
                    let data = e.mem.u32(s + 0x38);
                    let geometry = e.mem.u32(data + 4 * at as u32);
                    if e.mem.u32(geometry + 0x50) == 6 && e.mem.u32(geometry + 8) == 0 {
                        e.call(
                            STRING_BUFFER_CONSTRUCTOR,
                            &args![s + 0x28, s + 0x90, 0x200u32],
                        );
                        e.call(STRING_BUFFER_APPEND, &args![s + 0x28, BUILD_FAILED_TEXT]);
                        let sink: u32 = e.global(DIAGNOSTIC_SINK);
                        e.vcall(
                            sink,
                            0xc,
                            &args![0u32, 0xffff_ffffu32, s + 0x90, SOURCE_FILE_TEXT, 0x13cu32],
                        );
                        e.call(STRING_BUFFER_DESTRUCTOR, &args![s + 0x28]);
                        let last = e.mem.u32(s + 0x3c) - 1;
                        e.mem.set_u32(s + 0x3c, last);
                        if last != at as u32 {
                            let data = e.mem.u32(s + 0x38);
                            let moved = e.mem.u32(data + 4 * last);
                            e.mem.set_u32(data + 4 * at as u32, moved);
                        }
                    }
                    at -= 1;
                }
                let found = e
                    .call(GET_PROPERTY, &args![body, s + 0x28, KEY_1135, 0u32])
                    .u32();
                let current = e.mem.u32(found);
                if current != 0 && current == e.mem.u32(s + 0x68) {
                    e.call(REMOVE_REFERENCE, &args![current]);
                    e.call(REMOVE_PROPERTY, &args![body, s + 0x28, KEY_1135]);
                }
            } else {
                e.mem.set_u8(s + 0x16, u8::from(first == 0));
                if second != 0 {
                    e.call(REMOVE_PROPERTY, &args![body, s + 0x68, 0x1131u32]);
                }
                let allocator = router_allocator(e);
                let object = e.vcall(allocator, 4, &args![0x60u32]).u32();
                e.mem.set_u16(object + 4, 0x60);
                let source = if first == 0 {
                    e.mem.u32(s + 0x20)
                } else {
                    first
                };
                let convex = e
                    .call(DISPLAY_CONVEX_CONSTRUCTOR, &args![object, source])
                    .u32();
                store_bits(e, convex + 0x10, [0; 4]);
                store_bits(e, convex + 0x20, [0; 4]);
                store_bits(e, convex + 0x30, [0; 4]);
                e.mem.set_u32(convex + 0x10, 1.0f32.to_bits());
                e.mem.set_u32(convex + 0x24, 1.0f32.to_bits());
                e.mem.set_u32(convex + 0x38, 1.0f32.to_bits());
                store_bits(e, convex + 0x40, [0; 4]);
                let size = e.mem.u32(s + 0x3c);
                let data = e.mem.u32(s + 0x38);
                e.mem.set_u32(data + 4 * size, convex);
                e.mem.set_u32(s + 0x3c, size + 1);
            }
        }
        let id = fn_00d66180(e, Ptr::new(body));
        e.mem.set_u32(s + 0x20, id);
        push_back(e, list + 4, id);
        let process_id: u32 = e.global(PROCESS_ID);
        if count != 0 {
            let previous = e
                .call(MAP_GET_WITH_DEFAULT, &args![this + 0x20, shape, 1u32])
                .u32();
            let allocator = router_allocator(e);
            e.call(
                MAP_INSERT,
                &args![this + 0x20, allocator, shape, previous.wrapping_add(1)],
            );
            let display = e.mem.u32(this - 0x10);
            e.vcall(
                display,
                8,
                &args![count, body + 0xf0, id, process_id, shape],
            );
        } else if e.mem.i32(s + 0x3c) > 0 || e.mem.u8(s + 0x17) != 0 || e.mem.u8(this + 0x3c) == 0 {
            let display = e.mem.u32(this - 0x10);
            e.vcall(
                display,
                4,
                &args![s + 0x38, body + 0xf0, id, process_id, shape],
            );
            if e.mem.u8(this + 0x39) != 0 {
                let allocator = router_allocator(e);
                e.call(MAP_INSERT, &args![this + 0x14, allocator, shape, id]);
                let allocator = router_allocator(e);
                e.call(MAP_INSERT, &args![this + 0x20, allocator, shape, 1u32]);
            }
            let found = e
                .call(GET_PROPERTY, &args![body, s + 0x28, 0x1130u32, 0u32])
                .u32();
            let mut colour = e.mem.u32(found);
            if e.mem.u8(this + 0x3d) != 0 && (e.mem.u8(this + 0x3e) != 0 || colour == 0) {
                colour = if e.mem.u8(body + 0xe8) == 5 {
                    e.mem.u32(this + 0x44)
                } else {
                    e.mem.u32(this + 0x48)
                };
            }
            if colour != 0 {
                let display = e.mem.u32(this - 0x10);
                e.vcall(display, 0xc, &args![colour, id, process_id]);
            }
        }
        let mut at = 0;
        while at < e.mem.i32(s + 0x3c) {
            let data = e.mem.u32(s + 0x38);
            let geometry = e.mem.u32(data + 4 * at as u32);
            if e.mem.u8(s + 0x16) == 0 {
                e.mem.set_u32(geometry + 8, 0);
            }
            let data = e.mem.u32(s + 0x38);
            let geometry = e.mem.u32(data + 4 * at as u32);
            if geometry != 0 {
                let vtable = e.mem.u32(geometry);
                let destructor = e.mem.u32(vtable);
                e.call(destructor, &args![geometry, 1u32]);
            }
            at += 1;
        }
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(s + 0x40);
        e.mem.set_u32(s + 0x3c, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(s + 0x38);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
        }
    });
}

// Translated from 00d66180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, cdecl with one stack word: the display id of `body`. With a `0x1134` property
/// record the answer is its value, otherwise it is the address `body + 0x10`.
pub fn fn_00d66180(e: &mut Engine, body: Ptr) -> u32 {
    let body = body.addr();
    if find_property(e, body, KEY_DISPLAY_ID).is_some() {
        return e.with_stack(8, |e, out| {
            let found = e
                .call(GET_PROPERTY, &args![body, out.addr(), 0x1134u32, 0u32])
                .u32();
            e.mem.u32(found)
        });
    }
    body + 0x10
}

// Translated from 00d65910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x74 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d657d0` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d65956.
pub fn fn_00d65910(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x74]).u32();
    e.mem.set_u16(block + 4, 0x74);
    let object = e.call(0x00d6_57d0, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_52c0, fn_00d652c0(Ptr, u8, Ptr) -> u32),
        entry!(
            0x00d6_53f0,
            hkp_shape_display_viewer_synchronize_transforms(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_5520,
            hkp_shape_display_viewer_post_simulation_callback(Ptr, u32)
        ),
        entry!(0x00d6_55e0, fn_00d655e0(Ptr, u32) -> i32),
        entry!(0x00d6_5610, fn_00d65610(Ptr, Ptr)),
        entry!(0x00d6_5640, fn_00d65640(Ptr, Ptr)),
        entry!(0x00d6_5750, fn_00d65750(Ptr, u32)),
        entry!(
            0x00d6_57d0,
            hkp_shape_display_viewer_constructor(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d6_5950, fn_00d65950() -> u32),
        entry!(0x00d6_5970, fn_00d65970(Ptr, Ptr)),
        entry!(
            0x00d6_5ad0,
            hkp_shape_display_viewer_entity_added_callback(Ptr, Ptr)
        ),
        entry!(0x00d6_6180, fn_00d66180(Ptr) -> u32),
        entry!(0x00d6_5910, fn_00d65910(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, profile_labels, Log,
    };
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    const ALLOCATE: u32 = 0x0a20_0004;
    const DEALLOCATE: u32 = 0x0a20_0010;
    /// Base of the slot doubles of the display handler, of the entity listener and of the
    /// diagnostic sink.
    const HANDLER: u32 = 0x0a30_0000;
    const LISTENER: u32 = 0x0a40_0000;
    const SINK: u32 = 0x0a50_0000;
    const GEOMETRY_DESTRUCTOR: u32 = 0x0a80_0000;
    const PROCESS: u32 = 0x55;
    const WORLD: u32 = 0x4444;

    /// The state of the stand-in for the maps: `(map, key)` to value.
    type MapState = Rc<RefCell<HashMap<(u32, u32), u32>>>;

    /// The memory router and its allocator: slot 4 hands out zeroed blocks, slot `0x10` frees.
    /// Returns the allocator.
    fn memory(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        e.mem.set_u32(vtable + 0x10, DEALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, ROUTER, router);
        let allocations = Rc::clone(log);
        e.register_double(ALLOCATE, move |e, a| {
            allocations.borrow_mut().push((ALLOCATE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        log_double(e, log, DEALLOCATE, 0);
        allocator
    }

    /// `hkpWorldObject::getProperty` as the exe has it: copies the two value words of the
    /// first record with the key to the output pair (zeros without one) and answers the pair.
    fn properties(e: &mut Engine, log: &Log) {
        let log = Rc::clone(log);
        e.register_double(GET_PROPERTY, move |e, a| {
            log.borrow_mut().push((GET_PROPERTY, a.to_vec()));
            let records = e.mem.u32(a[0] + 0x78);
            let count = e.mem.i32(a[0] + 0x7c);
            let mut value = [0, 0];
            for index in 0..count.max(0) as u32 {
                if e.mem.u32(records + 16 * index) == a[2] {
                    value = [
                        e.mem.u32(records + 16 * index + 8),
                        e.mem.u32(records + 16 * index + 0xc),
                    ];
                    break;
                }
            }
            e.mem.set_u32(a[1], value[0]);
            e.mem.set_u32(a[1] + 4, value[1]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    /// The map methods over a table: `getWithDefault`, insert and remove.
    fn maps(e: &mut Engine, log: &Log) -> MapState {
        let state: MapState = Rc::new(RefCell::new(HashMap::new()));
        let get_state = Rc::clone(&state);
        let get_log = Rc::clone(log);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| {
            get_log
                .borrow_mut()
                .push((MAP_GET_WITH_DEFAULT, a.to_vec()));
            Ret {
                eax: get_state
                    .borrow()
                    .get(&(a[0], a[1]))
                    .copied()
                    .unwrap_or(a[2]),
                ..Ret::default()
            }
        });
        let insert_state = Rc::clone(&state);
        let insert_log = Rc::clone(log);
        e.register_double(MAP_INSERT, move |_, a| {
            insert_log.borrow_mut().push((MAP_INSERT, a.to_vec()));
            insert_state.borrow_mut().insert((a[0], a[2]), a[3]);
            Ret::default()
        });
        let remove_state = Rc::clone(&state);
        let remove_log = Rc::clone(log);
        e.register_double(MAP_REMOVE, move |_, a| {
            remove_log.borrow_mut().push((MAP_REMOVE, a.to_vec()));
            remove_state.borrow_mut().remove(&(a[0], a[1]));
            Ret::default()
        });
        state
    }

    /// An object (0x100 bytes) whose vtable has the given slots; each slot is a logging
    /// double at `base + slot`.
    fn object_with_slots(e: &mut Engine, log: &Log, base: u32, slots: &[u32]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for slot in slots {
            log_double(e, log, base + slot, 0);
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A body (0x100 bytes) with the given property records `(key, value)`.
    fn body_with(e: &mut Engine, props: &[(u32, u32)]) -> u32 {
        let body = e.mem.alloc(0x100);
        let records = e.mem.alloc(16 * props.len().max(1) as u32);
        for (index, (key, value)) in props.iter().enumerate() {
            e.mem.set_u32(records + 16 * index as u32, *key);
            e.mem.set_u32(records + 16 * index as u32 + 8, *value);
        }
        e.mem.set_u32(body + 0x78, records);
        e.mem.set_i32(body + 0x7c, props.len() as i32);
        body
    }

    /// A body of the world `WORLD` whose `+0x10` word is `shape`.
    fn added_body(e: &mut Engine, props: &[(u32, u32)], shape: u32) -> u32 {
        let body = body_with(e, props);
        e.mem.set_u32(body + 8, WORLD);
        e.mem.set_u32(body + 0x10, shape);
        body
    }

    /// A simulation island holding the given bodies (array at `+0x48`, count at `+0x4C`).
    fn island_with(e: &mut Engine, bodies: &[u32]) -> u32 {
        let island = e.mem.alloc(0x60);
        let data = e.mem.alloc(4 * bodies.len().max(1) as u32);
        for (index, body) in bodies.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *body);
        }
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_i32(island + 0x4c, bodies.len() as i32);
        island
    }

    /// Stores an array of islands (data pointer, then count) at `world + offset`.
    fn set_islands(e: &mut Engine, world: u32, offset: u32, islands: &[&[u32]]) {
        let data = e.mem.alloc(4 * islands.len().max(1) as u32);
        for (index, bodies) in islands.iter().enumerate() {
            let island = island_with(e, bodies);
            e.mem.set_u32(data + 4 * index as u32, island);
        }
        e.mem.set_u32(world + offset, data);
        e.mem.set_i32(world + offset + 4, islands.len() as i32);
    }

    /// The addresses of the logged calls, in order.
    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(address, _)| *address).collect()
    }

    /// The items of an array of words (data `+4`, size `+8` of `list`).
    fn items(e: &Engine, list: u32) -> Vec<u32> {
        let size = e.mem.u32(list + 8);
        let data = e.mem.u32(list + 4);
        (0..size).map(|index| e.mem.u32(data + 4 * index)).collect()
    }

    /// Stand-in for the array growth: moves the words to a new block with room for 16.
    fn growing_arrays(e: &mut Engine, log: &Log) {
        let log = Rc::clone(log);
        e.register_double(ARRAY_GROW, move |e, a| {
            log.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let size = e.mem.u32(a[1] + 4);
            let old = e.mem.u32(a[1]);
            let block = e.mem.alloc(64);
            for index in 0..size {
                let word = e.mem.u32(old + 4 * index);
                e.mem.set_u32(block + 4 * index, word);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
    }

    /// A viewer (0x100 bytes) with its display handler (slots 4, 8, `0xC`, `0x10`, `0x14`,
    /// `0x48` and `0x4C` are logging doubles at `HANDLER + slot`), one body list for the
    /// world `WORLD` (an array with room for 16 items), the router, the property and map
    /// doubles and the doubles of everything the entity callback calls. The option bytes at
    /// `listener + 0x39 ..= 0x3E` are `options`.
    struct Rig {
        viewer: u32,
        listener: u32,
        handler: u32,
        list: u32,
        allocator: u32,
        sink: u32,
        maps: MapState,
    }

    fn rig(e: &mut Engine, log: &Log, options: [u8; 6]) -> Rig {
        let allocator = memory(e, log);
        properties(e, log);
        let maps = maps(e, log);
        let handler = object_with_slots(e, log, HANDLER, &[4, 8, 0xc, 0x10, 0x14, 0x48, 0x4c]);
        let viewer = e.mem.alloc(0x100);
        e.mem.set_u32(viewer + 0x18, handler);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, WORLD);
        let data = e.mem.alloc(64);
        e.mem.set_u32(list + 4, data);
        e.mem.set_u32(list + 0xc, 16);
        let lists = e.mem.alloc(4);
        e.mem.set_u32(lists, list);
        e.mem.set_u32(viewer + 0x30, lists);
        e.mem.set_u32(viewer + 0x34, 1);
        let listener = viewer + 0x28;
        for (index, option) in options.iter().enumerate() {
            e.mem.set_u8(listener + 0x39 + index as u32, *option);
        }
        e.set_global(PROCESS_ID, PROCESS);
        let sink = object_with_slots(e, log, SINK, &[0xc]);
        e.set_global(DIAGNOSTIC_SINK, sink);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, GEOMETRY_DESTRUCTOR);
        log_double(e, log, GEOMETRY_DESTRUCTOR, 0);
        let constructions = Rc::clone(log);
        e.register_double(DISPLAY_CONVEX_CONSTRUCTOR, move |e, a| {
            constructions
                .borrow_mut()
                .push((DISPLAY_CONVEX_CONSTRUCTOR, a.to_vec()));
            e.mem.set_u32(a[0], vtable);
            e.mem.set_u32(a[0] + 8, 0x99);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for address in [
            BUILDER_ENVIRONMENT,
            BUILDER_CONSTRUCTOR,
            STRING_BUFFER_CONSTRUCTOR,
            STRING_BUFFER_APPEND,
            STRING_BUFFER_DESTRUCTOR,
            REMOVE_PROPERTY,
            REMOVE_REFERENCE,
        ] {
            log_double(e, log, address, 0);
        }
        Rig {
            viewer,
            listener,
            handler,
            list,
            allocator,
            sink,
            maps,
        }
    }

    /// A display geometry of the given kind (`+0x50`) and user word (`+8`) whose destructor is
    /// the logging double.
    fn geometry(e: &mut Engine, kind: u32, user: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, GEOMETRY_DESTRUCTOR);
        let geometry = e.mem.alloc(0x60);
        e.mem.set_u32(geometry, vtable);
        e.mem.set_u32(geometry + 8, user);
        e.mem.set_u32(geometry + 0x50, kind);
        geometry
    }

    /// The builder stand-in appends `geometries` to the array it is given.
    fn builder_adds(e: &mut Engine, log: &Log, geometries: &[u32]) {
        let log = Rc::clone(log);
        let geometries = geometries.to_vec();
        e.register_double(BUILD_DISPLAY_GEOMETRIES, move |e, a| {
            log.borrow_mut()
                .push((BUILD_DISPLAY_GEOMETRIES, a.to_vec()));
            for geometry in &geometries {
                let size = e.mem.u32(a[2] + 4);
                let data = e.mem.u32(a[2]);
                e.mem.set_u32(data + 4 * size, *geometry);
                e.mem.set_u32(a[2] + 4, size + 1);
            }
            Ret::default()
        });
    }

    /// Slot 4 of the display handler logs its call and records the geometries in the array
    /// it was given.
    fn snapshots(e: &mut Engine, log: &Log) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(HANDLER + 4, move |e, a| {
            log.borrow_mut().push((HANDLER + 4, a.to_vec()));
            let size = e.mem.u32(a[1] + 4);
            let data = e.mem.u32(a[1]);
            record
                .borrow_mut()
                .push((0..size).map(|index| e.mem.u32(data + 4 * index)).collect());
            Ret::default()
        });
        seen
    }

    #[test]
    fn display_id_00d66180_is_the_property_value_or_an_address_in_the_body() {
        let mut e = engine();
        let log = new_log();
        properties(&mut e, &log);
        let plain = body_with(&mut e, &[(0x1135, 5)]);
        assert_eq!(e.call(0x00d6_6180, &args![plain]).u32(), plain + 0x10);
        assert!(calls(&log, GET_PROPERTY).is_empty());
        let named = body_with(&mut e, &[(0x1135, 5), (0x1134, 0xabc)]);
        assert_eq!(e.call(0x00d6_6180, &args![named]).u32(), 0xabc);
        let asked = calls(&log, GET_PROPERTY);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], named);
        assert_eq!(asked[0][2..].to_vec(), vec![0x1134, 0]);
        let empty = body_with(&mut e, &[]);
        assert_eq!(e.call(0x00d6_6180, &args![empty]).u32(), empty + 0x10);
    }

    #[test]
    fn property_choice_00d652c0_takes_the_first_record_that_is_present() {
        // (records, use_first, answer, out byte)
        let cases: Vec<(Vec<(u32, u32)>, u8, u32, u8)> = vec![
            (vec![(0x1234, 7), (0x1235, 8), (0x1135, 9)], 1, 7, 0),
            (vec![(0x1234, 7), (0x1235, 8), (0x1135, 9)], 0, 8, 0),
            (vec![(0x1234, 0), (0x1235, 8)], 1, 0, 1),
            (vec![(0x1234, 5)], 0, 0, 0),
            (vec![(0x1235, 0), (0x1135, 9)], 0, 0, 1),
            (vec![(0x1135, 9)], 1, 9, 0),
            (vec![(0x1135, 0)], 0, 0, 1),
            (vec![], 1, 0, 0),
            (vec![(0x1234, 3), (0x1135, 9)], 1, 3, 0),
            (vec![(0x1136, 3)], 1, 0, 0),
        ];
        for (records, use_first, answer, out_byte) in cases {
            let mut e = engine();
            let log = new_log();
            properties(&mut e, &log);
            let object = body_with(&mut e, &records);
            let out = e.mem.alloc(4);
            e.mem.set_u8(out, 0xee);
            let got = e.call(0x00d6_52c0, &args![object, use_first, out]).u32();
            assert_eq!(got, answer, "{records:?} {use_first}");
            assert_eq!(e.mem.u8(out), out_byte, "{records:?} {use_first}");
            let keys: Vec<u32> = calls(&log, GET_PROPERTY)
                .iter()
                .map(|call| call[2])
                .collect();
            assert_eq!(keys, vec![0x1135, 0x1235, 0x1234]);
        }
    }

    #[test]
    fn synchronize_transforms_00d653f0_updates_every_body_at_the_stored_time() {
        let mut e = engine();
        let log = new_log();
        properties(&mut e, &log);
        e.set_global(PROCESS_ID, PROCESS);
        let handler = object_with_slots(&mut e, &log, HANDLER, &[0x10, 0x48, 0x4c]);
        log_double(&mut e, &log, APPROX_TRANSFORM_AT, 0);
        log_double(&mut e, &log, GET_CURRENT_TIME, 0);
        let plain = body_with(&mut e, &[]);
        let named = body_with(&mut e, &[(0x1134, 0x777)]);
        let world = e.mem.alloc(0x80);
        set_islands(&mut e, world, 0x28, &[&[plain, named], &[]]);
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x68, 2.5);
        e.call(0x00d6_53f0, &args![this, handler, world]);
        assert!(calls(&log, GET_CURRENT_TIME).is_empty());
        let transforms = calls(&log, APPROX_TRANSFORM_AT);
        assert_eq!(transforms.len(), 2);
        assert_eq!(
            transforms[0][..2].to_vec(),
            vec![plain + 0xe0, 2.5f32.to_bits()]
        );
        assert_eq!(
            transforms[1][..2].to_vec(),
            vec![named + 0xe0, 2.5f32.to_bits()]
        );
        assert_eq!(
            calls(&log, HANDLER + 0x10),
            vec![
                vec![handler, transforms[0][2], plain + 0x10, PROCESS],
                vec![handler, transforms[1][2], 0x777, PROCESS]
            ]
        );
        assert_eq!(
            order(&log),
            vec![
                HANDLER + 0x48,
                APPROX_TRANSFORM_AT,
                HANDLER + 0x10,
                APPROX_TRANSFORM_AT,
                HANDLER + 0x10,
                HANDLER + 0x4c
            ]
        );
    }

    #[test]
    fn synchronize_transforms_00d653f0_asks_the_world_unless_the_stored_time_is_positive() {
        let mut e = engine();
        let log = new_log();
        e.set_global(PROCESS_ID, PROCESS);
        let handler = object_with_slots(&mut e, &log, HANDLER, &[0x10, 0x48, 0x4c]);
        log_double(&mut e, &log, APPROX_TRANSFORM_AT, 0);
        let clock = Rc::clone(&log);
        e.register_double(GET_CURRENT_TIME, move |_, a| {
            clock.borrow_mut().push((GET_CURRENT_TIME, a.to_vec()));
            Ret {
                st0: 0.75,
                ..Ret::default()
            }
        });
        let body = body_with(&mut e, &[]);
        let world = e.mem.alloc(0x80);
        set_islands(&mut e, world, 0x28, &[&[body]]);
        let this = e.mem.alloc(0x80);
        for stored in [0.0f32, -1.0, f32::NAN] {
            log.borrow_mut().clear();
            e.mem.set_f32(this + 0x68, stored);
            e.call(0x00d6_53f0, &args![this, handler, world]);
            assert_eq!(calls(&log, GET_CURRENT_TIME), vec![vec![world]]);
            let transforms = calls(&log, APPROX_TRANSFORM_AT);
            assert_eq!(transforms.len(), 1);
            assert_eq!(transforms[0][1], 0.75f32.to_bits());
        }
        // No island at all: only the begin and the end calls.
        let empty = e.mem.alloc(0x80);
        log.borrow_mut().clear();
        e.call(0x00d6_53f0, &args![this, handler, empty]);
        assert_eq!(
            order(&log),
            vec![GET_CURRENT_TIME, HANDLER + 0x48, HANDLER + 0x4c]
        );
    }

    #[test]
    fn post_simulation_00d65520_calls_the_viewer_when_its_flag_is_set() {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let viewer = object_with_slots(&mut e, &log, HANDLER, &[0x10]);
        let listener = viewer + 0x2c;
        e.mem.set_u32(listener - 0x14, 0x1234);
        e.mem.set_u8(listener + 0x34, 1);
        e.call(0x00d6_5520, &args![listener, 0x99u32]);
        assert_eq!(
            calls(&log, HANDLER + 0x10),
            vec![vec![viewer, 0x1234, 0x99]]
        );
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_VIEWER, PROFILE_END]
        );
        e.mem.set_u8(listener + 0x34, 0);
        e.call(0x00d6_5520, &args![listener, 0x98u32]);
        assert_eq!(calls(&log, HANDLER + 0x10).len(), 1);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_VIEWER, PROFILE_END, LABEL_VIEWER, PROFILE_END]
        );
    }

    #[test]
    fn list_index_00d655e0_finds_the_entry_with_the_key() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(12);
        for (index, key) in [10u32, 20, 30].iter().enumerate() {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, *key);
            e.mem.set_u32(data + 4 * index as u32, entry);
        }
        e.mem.set_u32(this + 0x30, data);
        e.mem.set_i32(this + 0x34, 3);
        assert_eq!(e.call(0x00d6_55e0, &args![this, 20u32]).i32(), 1);
        assert_eq!(e.call(0x00d6_55e0, &args![this, 10u32]).i32(), 0);
        assert_eq!(e.call(0x00d6_55e0, &args![this, 30u32]).i32(), 2);
        assert_eq!(e.call(0x00d6_55e0, &args![this, 99u32]).i32(), -1);
        e.mem.set_i32(this + 0x34, 0);
        assert_eq!(e.call(0x00d6_55e0, &args![this, 10u32]).i32(), -1);
    }

    #[test]
    fn display_update_00d65610_hands_the_transform_words_to_the_handler() {
        let mut e = engine();
        let log = new_log();
        properties(&mut e, &log);
        e.set_global(PROCESS_ID, PROCESS);
        let handler = object_with_slots(&mut e, &log, HANDLER, &[0x10]);
        let this = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u32(this - 0x14, handler);
        let named = body_with(&mut e, &[(0x1134, 0x777)]);
        e.call(0x00d6_5610, &args![this, named]);
        let plain = body_with(&mut e, &[]);
        e.call(0x00d6_5610, &args![this, plain]);
        assert_eq!(
            calls(&log, HANDLER + 0x10),
            vec![
                vec![handler, named + 0xf0, 0x777, PROCESS],
                vec![handler, plain + 0xf0, plain + 0x10, PROCESS]
            ]
        );
    }

    #[test]
    fn entity_removed_00d65640_drops_the_display_id_from_the_list() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0; 6]);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        let id = body + 0x10;
        let data = e.mem.u32(rig.list + 4);
        for (index, item) in [1u32, id, 3, 4].iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *item);
        }
        e.mem.set_u32(rig.list + 8, 4);
        e.call(0x00d6_5640, &args![rig.listener, body]);
        assert_eq!(
            calls(&log, HANDLER + 0x14),
            vec![vec![rig.handler, id, PROCESS, 0x300]]
        );
        assert_eq!(items(&e, rig.list), vec![1, 4, 3]);
        assert!(calls(&log, MAP_GET_WITH_DEFAULT).is_empty());
        // An id that is not in the list changes nothing.
        let other = added_body(&mut e, &[], 0x321);
        e.call(0x00d6_5640, &args![rig.listener, other]);
        assert_eq!(items(&e, rig.list), vec![1, 4, 3]);
    }

    #[test]
    fn entity_removed_00d65640_ends_without_a_list_for_the_world() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0; 6]);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        e.mem.set_u32(body + 8, 0x9999);
        e.mem.set_u32(rig.list + 8, 1);
        e.call(0x00d6_5640, &args![rig.listener, body]);
        assert!(calls(&log, HANDLER + 0x14).is_empty());
        assert_eq!(e.mem.u32(rig.list + 8), 1);
    }

    #[test]
    fn entity_removed_00d65640_uses_the_body_shape_when_the_properties_give_none() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0; 6]);
        // The viewer's option word asks for the second record kind, so 0x1234 is skipped.
        e.mem.set_u32(rig.viewer + 0x10, 1);
        let body = added_body(&mut e, &[(0x1234, 0x900)], 0x321);
        e.call(0x00d6_5640, &args![rig.listener, body]);
        assert_eq!(
            calls(&log, HANDLER + 0x14),
            vec![vec![rig.handler, body + 0x10, PROCESS, 0x321]]
        );
        assert!(items(&e, rig.list).is_empty());
    }

    #[test]
    fn entity_removed_00d65640_releases_the_shape_use_counts() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [1, 0, 0, 0, 0, 0]);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        rig.maps
            .borrow_mut()
            .insert((rig.listener + 0x20, 0x300), 1);
        rig.maps
            .borrow_mut()
            .insert((rig.listener + 0x14, 0x300), 7);
        e.call(0x00d6_5640, &args![rig.listener, body]);
        assert_eq!(
            calls(&log, MAP_GET_WITH_DEFAULT),
            vec![vec![rig.listener + 0x20, 0x300, 0]]
        );
        assert_eq!(
            calls(&log, MAP_REMOVE),
            vec![
                vec![rig.listener + 0x14, 0x300],
                vec![rig.listener + 0x20, 0x300]
            ]
        );
        assert!(calls(&log, MAP_INSERT).is_empty());
        // Three users: the count drops by one and the shape stays.
        rig.maps
            .borrow_mut()
            .insert((rig.listener + 0x20, 0x300), 3);
        log.borrow_mut().clear();
        e.call(0x00d6_5640, &args![rig.listener, body]);
        assert_eq!(
            calls(&log, MAP_INSERT),
            vec![vec![rig.listener + 0x20, rig.allocator, 0x300, 2]]
        );
        assert!(calls(&log, MAP_REMOVE).is_empty());
        // A zero shape is not counted at all.
        let bare = added_body(&mut e, &[], 0);
        log.borrow_mut().clear();
        e.call(0x00d6_5640, &args![rig.listener, bare]);
        assert!(calls(&log, MAP_GET_WITH_DEFAULT).is_empty());
        assert_eq!(calls(&log, HANDLER + 0x14).len(), 1);
    }

    #[test]
    fn list_clear_00d65750_hands_every_item_to_the_handler_and_empties_the_list() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0; 6]);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x11);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x22);
        let data = e.mem.u32(rig.list + 4);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, second);
        e.mem.set_u32(rig.list + 8, 2);
        e.call(0x00d6_5750, &args![rig.viewer, 0u32]);
        assert_eq!(
            calls(&log, HANDLER + 0x14),
            vec![
                vec![rig.handler, first, PROCESS, 0x11],
                vec![rig.handler, second, PROCESS, 0x22]
            ]
        );
        assert_eq!(e.mem.u32(rig.list + 8), 0);
        assert_eq!(calls(&log, ROUTER).len(), 1);
        e.call(0x00d6_5750, &args![rig.viewer, 0u32]);
        assert_eq!(calls(&log, HANDLER + 0x14).len(), 2);
        assert_eq!(calls(&log, ROUTER).len(), 2);
        assert!(calls(&log, ARRAY_RESIZE).is_empty());
    }

    #[test]
    fn constructor_00d657d0_sets_up_the_viewer_with_default_options() {
        let mut e = engine();
        let log = new_log();
        let allocator = memory(&mut e, &log);
        log_double(&mut e, &log, VIEWER_BASE_CONSTRUCTOR, 0);
        log_double(&mut e, &log, MAP_RESERVE, 0);
        let strings = Rc::clone(&log);
        e.register_double(STR_CMP, move |_, a| {
            strings.borrow_mut().push((STR_CMP, a.to_vec()));
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.set_global(0x0101_2054, 0x7f80_0001u32);
        e.set_global(0x0126_8fe8, 0x1111u32);
        e.set_global(0x0126_8fe4, 0x2222u32);
        let other = object_with_slots(&mut e, &log, 0x0a60_0000, &[4]);
        e.register_double(0x0a60_0004, |_, _| Ret {
            eax: 0x0a70_0000,
            ..Ret::default()
        });
        let processes = e.mem.alloc(8);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, other);
        e.mem.set_u32(processes, data);
        e.mem.set_u32(processes + 4, 1);
        let at = e.mem.alloc(0x100);
        let got = e.call(0x00d6_57d0, &args![at, processes]).u32();
        assert_eq!(got, at);
        assert_eq!(
            calls(&log, VIEWER_BASE_CONSTRUCTOR),
            vec![vec![at, processes]]
        );
        for (offset, value) in [
            (0, 0x010d_1b04),
            (8, 0x010d_1ae4),
            (0x20, 0x010d_1ad4),
            (0x28, 0x010d_1ab8),
            (0x2c, 0x010d_1aa8),
            (0x30, 0),
            (0x34, 0),
            (0x38, 0x8000_0000),
            (0x68, 0x7fc0_0001),
            (0x6c, 0x1111),
            (0x70, 0x2222),
        ] {
            assert_eq!(e.mem.u32(at + offset), value, "{offset:#x}");
        }
        for map in [0x3c, 0x48, 0x54] {
            assert_eq!(e.mem.u32(at + map), 0);
            assert_eq!(e.mem.u32(at + map + 4), 0);
            assert_eq!(e.mem.u32(at + map + 8), 0xffff_ffff);
        }
        assert_eq!(
            calls(&log, MAP_RESERVE),
            vec![
                vec![at + 0x3c, allocator, 0],
                vec![at + 0x48, allocator, 0],
                vec![at + 0x54, allocator, 0]
            ]
        );
        let bytes: Vec<u8> = (0x60..=0x66).map(|offset| e.mem.u8(at + offset)).collect();
        assert_eq!(bytes, vec![1, 0, 0, 1, 1, 1, 0]);
        assert_eq!(calls(&log, STR_CMP), vec![vec![OPTIONS_NAME, 0x0a70_0000]]);
        // No process objects: nothing is compared.
        let none = e.mem.alloc(8);
        let again = e.mem.alloc(0x100);
        log.borrow_mut().clear();
        e.call(0x00d6_57d0, &args![again, none]);
        assert!(calls(&log, STR_CMP).is_empty());
        assert_eq!(e.mem.u8(again + 0x60), 1);
    }

    #[test]
    fn constructor_00d657d0_takes_the_option_byte_from_the_options_object() {
        let mut e = engine();
        let log = new_log();
        memory(&mut e, &log);
        log_double(&mut e, &log, VIEWER_BASE_CONSTRUCTOR, 0);
        log_double(&mut e, &log, MAP_RESERVE, 0);
        e.register_double(STR_CMP, move |_, a| Ret {
            eax: u32::from(a[1] != 0x0a70_0100),
            ..Ret::default()
        });
        let other = object_with_slots(&mut e, &log, 0x0a60_0000, &[4]);
        e.register_double(0x0a60_0004, |_, _| Ret {
            eax: 0x0a70_0000,
            ..Ret::default()
        });
        let wanted = object_with_slots(&mut e, &log, 0x0a61_0000, &[4]);
        e.register_double(0x0a61_0004, |_, _| Ret {
            eax: 0x0a70_0100,
            ..Ret::default()
        });
        // The option object pointer is a secondary base, 8 bytes into the object.
        e.mem.set_u8(wanted + 0x50, 7);
        let processes = e.mem.alloc(8);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, other);
        e.mem.set_u32(data + 4, wanted);
        e.mem.set_u32(processes, data);
        e.mem.set_u32(processes + 4, 2);
        let at = e.mem.alloc(0x100);
        e.call(0x00d6_57d0, &args![at, processes]);
        assert_eq!(e.mem.u8(at + 0x60), 7);
        assert_eq!(e.mem.u8(at + 0x61), 0);
    }

    #[test]
    fn register_00d65950_keeps_the_process_id() {
        let mut e = engine();
        let log = new_log();
        e.set_global(PROCESS_FACTORY, 0x7000u32);
        log_double(&mut e, &log, REGISTER_PROCESS, 9);
        assert_eq!(e.call(0x00d6_5950, &args![]).u32(), 9);
        assert_eq!(
            calls(&log, REGISTER_PROCESS),
            vec![vec![0x7000, PROCESS_NAME, CREATE_VIEWER]]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 9);
    }

    /// A world for the registration test: active islands `[[a, b], [c]]`, inactive islands
    /// `[[d]]` and the fixed island `[f]`; the bodies are the numbers 1 to 5.
    fn registration_world(e: &mut Engine) -> u32 {
        let world = e.mem.alloc(0x80);
        set_islands(e, world, 0x28, &[&[1, 2], &[3]]);
        set_islands(e, world, 0x34, &[&[4]]);
        let fixed = island_with(e, &[5]);
        e.mem.set_u32(world + 0x20, fixed);
        world
    }

    #[test]
    fn registration_00d65970_registers_the_listeners_and_announces_the_bodies() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0; 6]);
        growing_arrays(&mut e, &log);
        log_double(&mut e, &log, ADD_ENTITY_LISTENER, 0);
        log_double(&mut e, &log, ADD_POST_SIMULATION_LISTENER, 0);
        let listener_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(listener_vtable + 4, LISTENER + 4);
        e.mem.set_u32(rig.listener, listener_vtable);
        log_double(&mut e, &log, LISTENER + 4, 0);
        let world = registration_world(&mut e);
        // Nothing is registered yet: the array is full and has no storage.
        e.mem.set_u32(rig.viewer + 0x30, 0);
        e.mem.set_u32(rig.viewer + 0x34, 0);
        e.mem.set_u32(rig.viewer + 0x38, 0x8000_0000);
        e.call(0x00d6_5970, &args![rig.viewer, world]);
        assert_eq!(
            calls(&log, ADD_ENTITY_LISTENER),
            vec![vec![world, rig.viewer + 0x28]]
        );
        assert_eq!(
            calls(&log, ADD_POST_SIMULATION_LISTENER),
            vec![vec![world, rig.viewer + 0x2c]]
        );
        assert_eq!(calls(&log, ALLOCATE), vec![vec![rig.allocator, 0x10]]);
        assert_eq!(
            calls(&log, ARRAY_GROW),
            vec![vec![rig.allocator, rig.viewer + 0x30, 4]]
        );
        assert_eq!(e.mem.u32(rig.viewer + 0x34), 1);
        let record = e.mem.u32(e.mem.u32(rig.viewer + 0x30));
        assert_eq!(e.mem.u32(record), world);
        assert_eq!(e.mem.u32(record + 4), 0);
        assert_eq!(e.mem.u32(record + 8), 0);
        assert_eq!(e.mem.u32(record + 0xc), 0x8000_0000);
        assert_eq!(
            calls(&log, LISTENER + 4),
            (1..=5)
                .map(|body| vec![rig.viewer + 0x28, body])
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn registration_00d65970_without_growth_or_fixed_island() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0; 6]);
        growing_arrays(&mut e, &log);
        log_double(&mut e, &log, ADD_ENTITY_LISTENER, 0);
        log_double(&mut e, &log, ADD_POST_SIMULATION_LISTENER, 0);
        let listener_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(listener_vtable + 4, LISTENER + 4);
        e.mem.set_u32(rig.listener, listener_vtable);
        log_double(&mut e, &log, LISTENER + 4, 0);
        let world = e.mem.alloc(0x80);
        // The viewer already holds one record; the array has room.
        let storage = e.mem.alloc(16);
        e.mem.set_u32(storage, 0xabcd);
        e.mem.set_u32(rig.viewer + 0x30, storage);
        e.mem.set_u32(rig.viewer + 0x34, 1);
        e.mem.set_u32(rig.viewer + 0x38, 4);
        e.call(0x00d6_5970, &args![rig.viewer, world]);
        assert!(calls(&log, ARRAY_GROW).is_empty());
        assert_eq!(e.mem.u32(rig.viewer + 0x34), 2);
        assert_eq!(e.mem.u32(storage), 0xabcd);
        let record = e.mem.u32(storage + 4);
        assert_eq!(e.mem.u32(record), world);
        assert!(calls(&log, LISTENER + 4).is_empty());
    }

    #[test]
    fn entity_added_00d65ad0_ends_early_when_there_is_nothing_to_show() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [1, 1, 1, 1, 0, 0]);
        // The record 0x1235 with value 0 sets the byte.
        let flagged = added_body(&mut e, &[(0x1235, 0)], 0x300);
        e.call(0x00d6_5ad0, &args![rig.listener, flagged]);
        assert!(calls(&log, MAP_GET_WITH_DEFAULT).is_empty());
        // A world without a list.
        let elsewhere = added_body(&mut e, &[(0x1135, 0x300)], 0);
        e.mem.set_u32(elsewhere + 8, 0x9999);
        e.call(0x00d6_5ad0, &args![rig.listener, elsewhere]);
        assert!(calls(&log, MAP_GET_WITH_DEFAULT).is_empty());
        // No shape at all.
        let bare = added_body(&mut e, &[], 0);
        e.call(0x00d6_5ad0, &args![rig.listener, bare]);
        assert!(calls(&log, MAP_GET_WITH_DEFAULT).is_empty());
        assert!(calls(&log, HANDLER + 4).is_empty());
        assert!(items(&e, rig.list).is_empty());
        assert_eq!(calls(&log, GET_PROPERTY).len(), 9);
    }

    #[test]
    fn entity_added_00d65ad0_counts_a_shape_that_is_already_shown() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [1, 0, 0, 0, 0, 0]);
        rig.maps
            .borrow_mut()
            .insert((rig.listener + 0x14, 0x300), 2);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        assert_eq!(
            calls(&log, MAP_GET_WITH_DEFAULT),
            vec![
                vec![rig.listener + 0x14, 0x300, 0],
                vec![rig.listener + 0x20, 0x300, 1]
            ]
        );
        assert_eq!(
            calls(&log, MAP_INSERT),
            vec![vec![rig.listener + 0x20, rig.allocator, 0x300, 2]]
        );
        assert_eq!(
            calls(&log, HANDLER + 8),
            vec![vec![
                rig.handler,
                2,
                body + 0xf0,
                body + 0x10,
                PROCESS,
                0x300
            ]]
        );
        assert!(calls(&log, HANDLER + 4).is_empty());
        assert!(calls(&log, BUILDER_ENVIRONMENT).is_empty());
        assert_eq!(items(&e, rig.list), vec![body + 0x10]);
    }

    #[test]
    fn entity_added_00d65ad0_builds_the_geometries_of_a_new_shape() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0, 0, 1, 1, 0, 0]);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        let dropped = geometry(&mut e, 6, 0);
        let kept = geometry(&mut e, 1, 0);
        let user = geometry(&mut e, 6, 5);
        builder_adds(&mut e, &log, &[dropped, kept, user]);
        let arrays = snapshots(&mut e, &log);
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        let build = calls(&log, BUILD_DISPLAY_GEOMETRIES);
        assert_eq!(build.len(), 1);
        assert_eq!(build[0][1], 0x300);
        let environment = calls(&log, BUILDER_ENVIRONMENT);
        assert_eq!(environment.len(), 1);
        assert_eq!(
            calls(&log, BUILDER_CONSTRUCTOR),
            vec![vec![build[0][0], environment[0][0]]]
        );
        // The geometry without user data is reported and replaced by the last one.
        let text = calls(&log, STRING_BUFFER_CONSTRUCTOR);
        assert_eq!(text.len(), 1);
        assert_eq!(text[0][2], 0x200);
        assert_eq!(
            calls(&log, STRING_BUFFER_APPEND),
            vec![vec![text[0][0], BUILD_FAILED_TEXT]]
        );
        assert_eq!(
            calls(&log, SINK + 0xc),
            vec![vec![
                rig.sink,
                0,
                0xffff_ffff,
                text[0][1],
                SOURCE_FILE_TEXT,
                0x13c
            ]]
        );
        assert_eq!(
            calls(&log, STRING_BUFFER_DESTRUCTOR),
            vec![vec![text[0][0]]]
        );
        assert_eq!(*arrays.borrow(), vec![vec![user, kept]]);
        let shown = calls(&log, HANDLER + 4);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][0], rig.handler);
        assert_eq!(
            shown[0][2..].to_vec(),
            vec![body + 0xf0, body + 0x10, PROCESS, 0x300]
        );
        // The shape property is released.
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x300]]);
        let removed = calls(&log, REMOVE_PROPERTY);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], body);
        assert_eq!(removed[0][2], 0x1135);
        // The kept geometries are destroyed with the delete flag; the user words stay.
        assert_eq!(
            calls(&log, GEOMETRY_DESTRUCTOR),
            vec![vec![user, 1], vec![kept, 1]]
        );
        assert_eq!(e.mem.u32(user + 8), 5);
        assert_eq!(items(&e, rig.list), vec![body + 0x10]);
        assert!(calls(&log, DEALLOCATE).is_empty());
    }

    #[test]
    fn entity_added_00d65ad0_keeps_the_shape_property_when_it_is_not_the_one_read() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0, 0, 1, 1, 0, 0]);
        // No 0x1135 record: the shape comes from the body.
        let body = added_body(&mut e, &[], 0x321);
        builder_adds(&mut e, &log, &[]);
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert!(calls(&log, REMOVE_PROPERTY).is_empty());
        let build = calls(&log, BUILD_DISPLAY_GEOMETRIES);
        assert_eq!(build[0][1], 0x321);
    }

    #[test]
    fn entity_added_00d65ad0_frees_an_array_the_builder_moved_to_the_heap() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0, 0, 1, 1, 0, 0]);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        let moved = e.mem.alloc(64);
        let builds = Rc::clone(&log);
        e.register_double(BUILD_DISPLAY_GEOMETRIES, move |e, a| {
            builds
                .borrow_mut()
                .push((BUILD_DISPLAY_GEOMETRIES, a.to_vec()));
            e.mem.set_u32(a[2], moved);
            e.mem.set_u32(a[2] + 8, 16);
            Ret::default()
        });
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        assert_eq!(
            calls(&log, DEALLOCATE),
            vec![vec![rig.allocator, moved, 64]]
        );
        // The empty array of a new shape is not shown.
        assert!(calls(&log, HANDLER + 4).is_empty());
    }

    #[test]
    fn entity_added_00d65ad0_makes_a_convex_from_the_geometry_property() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0, 0, 1, 1, 0, 0]);
        let body = added_body(&mut e, &[(0x1136, 0x111), (0x1135, 0x300)], 0);
        let arrays = snapshots(&mut e, &log);
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![rig.allocator, 0x60]]);
        let made = calls(&log, DISPLAY_CONVEX_CONSTRUCTOR);
        assert_eq!(made.len(), 1);
        let convex = made[0][0];
        assert_eq!(made[0][1], 0x111);
        assert_eq!(e.mem.u16(convex + 4), 0x60);
        assert!(calls(&log, BUILD_DISPLAY_GEOMETRIES).is_empty());
        assert!(calls(&log, REMOVE_PROPERTY).is_empty());
        assert_eq!(*arrays.borrow(), vec![vec![convex]]);
        // Identity rows, no translation.
        let matrix: Vec<u32> = (0..16)
            .map(|word| e.mem.u32(convex + 0x10 + 4 * word))
            .collect();
        let one = 1.0f32.to_bits();
        let mut expected = vec![0u32; 16];
        expected[0] = one;
        expected[5] = one;
        expected[10] = one;
        assert_eq!(matrix, expected);
        // The user word is cleared before the geometry is destroyed.
        assert_eq!(e.mem.u32(convex + 8), 0);
        assert_eq!(calls(&log, GEOMETRY_DESTRUCTOR), vec![vec![convex, 1]]);
    }

    #[test]
    fn entity_added_00d65ad0_makes_a_convex_from_the_second_property_and_drops_it() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0, 0, 1, 1, 0, 0]);
        let body = added_body(&mut e, &[(0x1131, 0x222), (0x1135, 0x300)], 0);
        snapshots(&mut e, &log);
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        let made = calls(&log, DISPLAY_CONVEX_CONSTRUCTOR);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1], 0x222);
        let removed = calls(&log, REMOVE_PROPERTY);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], body);
        assert_eq!(removed[0][2], 0x1131);
        // This time the user word is left alone.
        assert_eq!(e.mem.u32(made[0][0] + 8), 0x99);
        assert_eq!(calls(&log, GEOMETRY_DESTRUCTOR), vec![vec![made[0][0], 1]]);
    }

    #[test]
    fn entity_added_00d65ad0_shows_nothing_for_a_shape_that_builds_no_geometry() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [0, 1, 1, 1, 0, 0]);
        builder_adds(&mut e, &log, &[]);
        let arrays = snapshots(&mut e, &log);
        let first = added_body(&mut e, &[(0x1135, 0x300)], 0);
        e.call(0x00d6_5ad0, &args![rig.listener, first]);
        assert!(calls(&log, HANDLER + 4).is_empty());
        assert_eq!(
            calls(&log, MAP_INSERT),
            vec![vec![rig.listener + 0x2c, rig.allocator, 0x300, 1]]
        );
        assert_eq!(items(&e, rig.list), vec![first + 0x10]);
        // The shape has been seen: no build, and the (empty) array is shown.
        let second = added_body(&mut e, &[(0x1135, 0x300)], 0);
        e.call(0x00d6_5ad0, &args![rig.listener, second]);
        assert_eq!(calls(&log, BUILD_DISPLAY_GEOMETRIES).len(), 1);
        assert_eq!(calls(&log, HANDLER + 4).len(), 1);
        assert_eq!(*arrays.borrow(), vec![Vec::<u32>::new()]);
        assert_eq!(calls(&log, MAP_INSERT).len(), 1);
        assert_eq!(items(&e, rig.list), vec![first + 0x10, second + 0x10]);
    }

    #[test]
    fn entity_added_00d65ad0_shows_an_empty_array_when_building_is_off() {
        let mut e = engine();
        let log = new_log();
        let rig = rig(&mut e, &log, [1, 0, 1, 0, 0, 0]);
        let arrays = snapshots(&mut e, &log);
        let body = added_body(&mut e, &[(0x1135, 0x300)], 0);
        e.call(0x00d6_5ad0, &args![rig.listener, body]);
        assert!(calls(&log, BUILDER_ENVIRONMENT).is_empty());
        assert_eq!(*arrays.borrow(), vec![Vec::<u32>::new()]);
        // The maps learn the shape and its display id.
        assert_eq!(
            calls(&log, MAP_INSERT),
            vec![
                vec![rig.listener + 0x14, rig.allocator, 0x300, body + 0x10],
                vec![rig.listener + 0x20, rig.allocator, 0x300, 1]
            ]
        );
        assert_eq!(
            rig.maps
                .borrow()
                .get(&(rig.listener + 0x14, 0x300))
                .copied(),
            Some(body + 0x10)
        );
    }

    #[test]
    fn entity_added_00d65ad0_tells_the_handler_the_colour() {
        // (option 0x3D, option 0x3E, colour property, body kind, colour the handler gets)
        let cases: [(u8, u8, u32, u8, Option<u32>); 7] = [
            (0, 0, 0, 5, None),
            (0, 0, 7, 3, Some(7)),
            (1, 0, 7, 3, Some(7)),
            (1, 0, 0, 5, Some(0xa1)),
            (1, 0, 0, 3, Some(0xa2)),
            (1, 1, 7, 5, Some(0xa1)),
            (1, 1, 7, 3, Some(0xa2)),
        ];
        for (flag_d, flag_e, property, kind, expected) in cases {
            let mut e = engine();
            let log = new_log();
            let rig = rig(&mut e, &log, [0, 0, 0, 0, flag_d, flag_e]);
            e.mem.set_u32(rig.listener + 0x44, 0xa1);
            e.mem.set_u32(rig.listener + 0x48, 0xa2);
            let body = added_body(&mut e, &[(0x1130, property), (0x1135, 0x300)], 0);
            e.mem.set_u8(body + 0xe8, kind);
            e.call(0x00d6_5ad0, &args![rig.listener, body]);
            let told = calls(&log, HANDLER + 0xc);
            match expected {
                Some(colour) => assert_eq!(
                    told,
                    vec![vec![rig.handler, colour, body + 0x10, PROCESS]],
                    "{flag_d} {flag_e} {property} {kind}"
                ),
                None => assert!(told.is_empty(), "{flag_d} {flag_e} {property} {kind}"),
            }
        }
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
    fn fn_00d65910_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_57d0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_5910, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x74]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x74);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d65910_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_57d0, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_5910, &args![0x1234u32]).u32(), 0);
    }
}
