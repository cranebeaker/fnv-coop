//! `Havok SDK/hkpswepttransformdisplayviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpSweptTransformDisplayViewer` (the class of the vtables these functions fill; names from
//! the engine map) is a world viewer with several bases, so its methods take different
//! interior pointers as `this`. With `B` the object's start: the display handler is the pointer
//! at `B + 0x18`; the entity listener base is at `B + 0x28` and the post-simulation listener
//! base at `B + 0x2c`; and `B + 0x30` is an array (data, size, capacity word) of per-body
//! records. A record is 16 bytes: the first word is the key (`[body + 8]`) and its own array of
//! pointers (data `+4`, size `+8`, capacity word `+0xc`) holds the display identifiers that were
//! handed to the handler for that body: `body + 0x11` and `body + 0x12`, the two ends of the
//! swept transform. A body keeps its two rotations as quaternions at `+0x150` and `+0x160`, the
//! two centres at `+0x130` and `+0x140` and the local offset at `+0x170`; a transform is built
//! from a rotation (`00cb2d90`, three columns and a fourth that is overwritten) with the
//! translation `centre - (offset.y * column1 + offset.x * column0 + offset.z * column2)`.
//! The viewer's id for the handler's tag is the word at `01268fd0`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, splat, sub};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, profile_record, store_vec, PROFILE_END};

/// The profile label of `postSimulationCallback` (the string `TthkpSweptTransformDisplayViewer`).
const LABEL_POST_SIMULATION: u32 = 0x010d_1894;
/// The tag (process id) word that the viewer's handler calls take (`01268fd0`) and the two
/// words (`01268fd4`, `01268fd8`) the attribute calls take.
const DISPLAY_TAG: u32 = 0x0126_8fd0;
const ATTRIBUTE_END_B: u32 = 0x0126_8fd4;
const ATTRIBUTE_END_A: u32 = 0x0126_8fd8;
/// `00cb2d90`: `this` = a 0x40-byte transform, `(quaternion)`; fills the three columns.
const RECORD_FROM_QUATERNION: u32 = 0x00cb_2d90;
/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90940` (cdecl `(allocator, array, element size)`): grows an array by one element.
const GROW_BY_ONE: u32 = 0x00c9_0940;
/// The record search (`00d63ff0`, `this` = the viewer base, `(key)`).
const FIND_RECORD: u32 = 0x00d6_3ff0;
/// `hkpShapeDisplayBuilder` pieces: the environment constructor (`00ddfb70`), the builder
/// constructor (`00ddfb80`, `this` = builder, `(environment)`) and `buildDisplayGeometries`
/// (`00de0f90`, `this` = builder, `(shape, geometries)`).
const ENVIRONMENT_CONSTRUCT: u32 = 0x00dd_fb70;
const BUILDER_CONSTRUCT: u32 = 0x00dd_fb80;
const BUILD_DISPLAY_GEOMETRIES: u32 = 0x00de_0f90;
/// The string buffer pieces of the warning: constructor (`00cb1600`, `this` = the buffer
/// object, `(storage, size)`), append (`00cc01c0`, `(text)`) and destructor (`00cc00e0`).
const STRING_BUFFER_CONSTRUCT: u32 = 0x00cb_1600;
const STRING_APPEND: u32 = 0x00cc_01c0;
const STRING_BUFFER_DESTRUCT: u32 = 0x00cc_00e0;
/// The warning text, its source file name and the logger object's pointer (`01268360`).
const WARNING_TEXT: u32 = 0x010d_196c;
const WARNING_FILE: u32 = 0x010d_1928;
const LOGGER: u32 = 0x0126_8360;
/// `hkpWorldViewerBase`'s constructor (`00ddccb0`, `this`, `(world)`).
const VIEWER_BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
/// `hkProcessFactory::registerProcess` (`00dd9380`, `this` = the factory, `(name, creator)`),
/// the factory pointer (`01269cd0`), the process name (`Swept Transforms`) and the creator.
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const PROCESS_NAME: u32 = 0x010d_1880;
const PROCESS_CREATOR: u32 = 0x00d6_45f0;

/// The address of element `index` of an array of words.
fn element(base: u32, index: i32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(4))
}

/// The transform of one pose of `body` in the 0x40 bytes at `buffer`: the rotation columns from
/// the quaternion at `body + rotation` (`00cb2d90`) and the translation
/// `[body + origin] - (offset.y * column1 + offset.x * column0 + offset.z * column2)` in the
/// fourth column.
fn pose_transform(e: &mut Engine, body: u32, rotation: u32, origin: u32, buffer: u32) {
    e.call(RECORD_FROM_QUATERNION, &args![buffer, body + rotation]);
    let offset = load_vec(e, body + 0x170);
    let column0 = load_vec(e, buffer);
    let column1 = load_vec(e, buffer + 0x10);
    let column2 = load_vec(e, buffer + 0x20);
    let rotated = add(
        add(
            mul(splat(offset, 1), column1),
            mul(splat(offset, 0), column0),
        ),
        mul(splat(offset, 2), column2),
    );
    let translation = sub(load_vec(e, body + origin), rotated);
    store_vec(e, buffer + 0x30, translation);
}

/// Hands both transforms of `body` to the display handler at `[this - 0x14]` (virtual `0x10`,
/// `(transform, id, tag)`), the first with the id `body + 0x11` and the second with `body +
/// 0x12`; one buffer serves for both.
fn show_swept_transforms(e: &mut Engine, this: u32, body: u32) {
    e.with_stack(0x40, |e, buffer| {
        let buffer = buffer.addr();
        pose_transform(e, body, 0x150, 0x130, buffer);
        let handler = e.mem.u32(this.wrapping_sub(0x14));
        let tag = e.mem.u32(DISPLAY_TAG);
        e.vcall(handler, 0x10, &args![buffer, body + 0x11, tag]);
        pose_transform(e, body, 0x160, 0x140, buffer);
        let handler = e.mem.u32(this.wrapping_sub(0x14));
        let tag = e.mem.u32(DISPLAY_TAG);
        e.vcall(handler, 0x10, &args![buffer, body + 0x12, tag]);
    });
}

// Translated from 00d63e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSweptTransformDisplayViewer::postSimulationCallback` (engine map; `this` = the
/// post-simulation listener base `B + 0x2c`, `(world)`): records the profile label, then for
/// every island (the pointers at `[world + 0x28]`, count at `world + 0x2c`, signed) and every
/// body in its array at `island + 0x48` (data, size at `+4`) hands the body's two swept
/// transforms to the display handler at `[this - 0x14]` (as [`fn_00d64020`] does) and finally
/// records the end label.
pub fn hkp_swept_transform_display_viewer_post_simulation_callback(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
) {
    let this = this.addr();
    let world = world.addr();
    profile_record(e, LABEL_POST_SIMULATION);
    let mut island_index = 0i32;
    if (e.mem.u32(world + 0x2c) as i32) > 0 {
        loop {
            let islands = e.mem.u32(world + 0x28);
            let island = e.mem.u32(element(islands, island_index));
            let bodies = island + 0x48;
            let mut body_index = 0i32;
            if (e.mem.u32(bodies + 4) as i32) > 0 {
                loop {
                    let list = e.mem.u32(bodies);
                    let body = e.mem.u32(element(list, body_index));
                    show_swept_transforms(e, this, body);
                    body_index = body_index.wrapping_add(1);
                    if body_index >= e.mem.u32(bodies + 4) as i32 {
                        break;
                    }
                }
            }
            island_index = island_index.wrapping_add(1);
            if island_index >= e.mem.u32(world + 0x2c) as i32 {
                break;
            }
        }
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00d63ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the record of the key in the viewer's record array (`this` = `B`; data `+0x30`, size
/// `+0x34`, signed): the index of the first record whose first word equals `key`, or -1.
pub fn fn_00d63ff0(e: &mut Engine, this: Ptr, key: u32) -> i32 {
    let object = this.addr();
    let count = e.mem.u32(object + 0x34) as i32;
    let mut index = 0i32;
    if count > 0 {
        let mut cursor = e.mem.u32(object + 0x30);
        loop {
            let record = e.mem.u32(cursor);
            if e.mem.u32(record) == key {
                return index;
            }
            index = index.wrapping_add(1);
            cursor = cursor.wrapping_add(4);
            if index >= count {
                break;
            }
        }
    }
    -1
}

// Translated from 00d64020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands the two swept transforms of `body` to the display handler at `[this - 0x14]` (`this` =
/// `B + 0x2c`): the transform from the quaternion at `+0x150` and the centre at `+0x130` with
/// the id `body + 0x11`, then the one from `+0x160` and `+0x140` with the id `body + 0x12`
/// (virtual `0x10`, `(transform, id, tag)`, the tag being the word at `01268fd0`).
pub fn fn_00d64020(e: &mut Engine, this: Ptr, body: Ptr) {
    show_swept_transforms(e, this.addr(), body.addr());
}

/// Removes the first occurrence of `value` from the pointer array of a record (data `+4`, size
/// `+8`) by moving the last element into its place.
fn remove_pointer(e: &mut Engine, record: u32, value: u32) {
    let count = e.mem.u32(record + 8) as i32;
    let mut index = 0i32;
    if count > 0 {
        let data = e.mem.u32(record + 4);
        loop {
            if e.mem.u32(element(data, index)) == value {
                // The exe also tests `index >= 0` here; the index only counts up from 0.
                let last = e.mem.u32(record + 8).wrapping_sub(1);
                e.mem.set_u32(record + 8, last);
                if last != index as u32 {
                    let data = e.mem.u32(record + 4);
                    let moved = e.mem.u32(data.wrapping_add(last.wrapping_mul(4)));
                    e.mem.set_u32(element(data, index), moved);
                }
                return;
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
}

// Translated from 00d64120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the display of `body` (`this` = `B + 0x28`, the entity listener base): when `[body +
/// 0x10]` is not zero and the record of the key `[body + 8]` is found (`00d63ff0`), the handler
/// at `[this - 0x10]` is told (virtual `0x14`, `(id, tag, 0)`) to drop the id `body + 0x11` and
/// the id is removed from the record's array (the last element takes its place); the same is
/// done for `body + 0x12`.
pub fn fn_00d64120(e: &mut Engine, this: Ptr, body: Ptr) {
    let this = this.addr();
    let body = body.addr();
    if e.mem.u32(body + 0x10) == 0 {
        return;
    }
    let key = e.mem.u32(body + 8);
    let index = e
        .call(FIND_RECORD, &args![this.wrapping_sub(0x28), key])
        .i32();
    if index < 0 {
        return;
    }
    let records = e.mem.u32(this + 8);
    let record = e.mem.u32(element(records, index));
    for id in [body + 0x11, body + 0x12] {
        let handler = e.mem.u32(this.wrapping_sub(0x10));
        let tag = e.mem.u32(DISPLAY_TAG);
        e.vcall(handler, 0x14, &args![id, tag, 0u32]);
        remove_pointer(e, record, id);
    }
}

// Translated from 00d641f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops every id of the record at `index` (`this` = `B`; the record array at `+0x30`): the
/// handler at `[B + 0x18]` is told (virtual `0x14`, `(id, tag, 0)`) for each pointer of the
/// record's array (data `+4`, size `+8`, signed), the memory router is asked for its allocator
/// and the record's size is set to 0. The exe's branch that would free the array's storage
/// follows `AND 0x3fffffff` with a test of the sign, so it can never run; the router's
/// allocator (`+0x10`) is only read there.
pub fn fn_00d641f0(e: &mut Engine, this: Ptr, index: u32) {
    let object = this.addr();
    let records = e.mem.u32(object + 0x30);
    let record = e.mem.u32(element(records, index as i32));
    let mut position = 0i32;
    if (e.mem.u32(record + 8) as i32) > 0 {
        loop {
            let tag = e.mem.u32(DISPLAY_TAG);
            let handler = e.mem.u32(object + 0x18);
            let ids = e.mem.u32(record + 4);
            let id = e.mem.u32(element(ids, position));
            e.vcall(handler, 0x14, &args![id, tag, 0u32]);
            position = position.wrapping_add(1);
            if position >= e.mem.u32(record + 8) as i32 {
                break;
            }
        }
    }
    e.call(MEMORY_ROUTER, &args![]);
    e.mem.set_u32(record + 8, 0);
}

// Translated from 00d64260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (no engine-map name), `(this = B, world)`: the base constructor (`00ddccb0`)
/// runs on `B` with the world, then the vtables are stored: first `0x0102f13c` at `+0x28` and
/// `0x010ccf7c` at `+0x2c` (both overwritten at once), then `0x010d1918` at `+0`, `0x010d18f8`
/// at `+8`, `0x010d18e8` at `+0x20`, `0x010d18cc` at `+0x28` and `0x010d18bc` at `+0x2c`; the
/// record array (`+0x30`, `+0x34`) is empty with the capacity word `0x80000000` at `+0x38`.
/// Returns `this`.
pub fn fn_00d64260(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let object = this.addr();
    e.call(VIEWER_BASE_CONSTRUCT, &args![this, world]);
    e.mem.set_u32(object + 0x28, 0x0102_f13c);
    e.mem.set_u32(object + 0x2c, 0x010c_cf7c);
    e.mem.set_u32(object, 0x010d_1918);
    e.mem.set_u32(object + 8, 0x010d_18f8);
    e.mem.set_u32(object + 0x20, 0x010d_18e8);
    e.mem.set_u32(object + 0x28, 0x010d_18cc);
    e.mem.set_u32(object + 0x2c, 0x010d_18bc);
    e.mem.set_u32(object + 0x30, 0);
    e.mem.set_u32(object + 0x34, 0);
    e.mem.set_u32(object + 0x38, 0x8000_0000);
    this
}

/// Appends `value` to a pointer array (data `+0`, size `+4`, capacity word `+8` whose low 30
/// bits are the capacity): the array is grown by one element (`00c90940`, with the router's
/// allocator) first when its size equals its capacity.
fn push_pointer(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(GROW_BY_ONE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The body of `entityAddedCallback` that runs in a frame of `0x2e0` bytes at `frame`: the
/// geometry array (data, size, capacity word `0x80000000`) is at `+0x14`, the string buffer
/// object at `+0x28`, the builder environment at `+0x34`, the builder at `+0x3c`, the two
/// transforms at `+0x50` and `+0x90` and the warning text storage (`0x200` bytes) at `+0xd0`.
fn show_new_body(e: &mut Engine, frame: u32, this: u32, body: u32, record: u32) {
    let geometries = frame + 0x14;
    let string_buffer = frame + 0x28;
    let environment = frame + 0x34;
    let builder = frame + 0x3c;
    let message = frame + 0xd0;
    e.mem.set_u32(geometries, 0);
    e.mem.set_u32(geometries + 4, 0);
    e.mem.set_u32(geometries + 8, 0x8000_0000);
    e.call(ENVIRONMENT_CONSTRUCT, &args![environment]);
    e.call(BUILDER_CONSTRUCT, &args![builder, environment]);
    let shape = e.mem.u32(body + 0x10);
    e.call(BUILD_DISPLAY_GEOMETRIES, &args![builder, shape, geometries]);
    let mut count = e.mem.u32(geometries + 4) as i32;
    let mut index = count.wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(geometries);
        let geometry = e.mem.u32(element(data, index));
        if e.mem.u32(geometry + 0x50) == 6 && e.mem.u32(geometry + 8) == 0 {
            e.call(
                STRING_BUFFER_CONSTRUCT,
                &args![string_buffer, message, 0x200u32],
            );
            e.call(STRING_APPEND, &args![string_buffer, WARNING_TEXT]);
            let logger = e.mem.u32(LOGGER);
            e.vcall(
                logger,
                0xc,
                &args![0u32, 0xffff_ffffu32, message, WARNING_FILE, 0xa8u32],
            );
            e.call(STRING_BUFFER_DESTRUCT, &args![string_buffer]);
            count = (e.mem.u32(geometries + 4) as i32).wrapping_sub(1);
            e.mem.set_u32(geometries + 4, count as u32);
            if count != index {
                let data = e.mem.u32(geometries);
                let moved = e.mem.u32(element(data, count));
                e.mem.set_u32(element(data, index), moved);
                count = e.mem.u32(geometries + 4) as i32;
            }
        }
        index = index.wrapping_sub(1);
    }
    if count > 0 && e.mem.u8(body + 0xe8) != 5 {
        let ids = record + 4;
        push_pointer(e, ids, body + 0x11);
        pose_transform(e, body, 0x150, 0x130, frame + 0x50);
        pose_transform(e, body, 0x160, 0x140, frame + 0x90);
        let handler = e.mem.u32(this.wrapping_sub(0x10));
        let tag = e.mem.u32(DISPLAY_TAG);
        e.vcall(
            handler,
            4,
            &args![geometries, frame + 0x50, body + 0x11, tag, 0u32],
        );
        let handler = e.mem.u32(this.wrapping_sub(0x10));
        let tag = e.mem.u32(DISPLAY_TAG);
        let end = e.mem.u32(ATTRIBUTE_END_A);
        e.vcall(handler, 0xc, &args![end, body + 0x11, tag]);
        push_pointer(e, ids, body + 0x12);
        let tag = e.mem.u32(DISPLAY_TAG);
        let handler = e.mem.u32(this.wrapping_sub(0x10));
        e.vcall(
            handler,
            4,
            &args![geometries, frame + 0x90, body + 0x12, tag, 0u32],
        );
        let tag = e.mem.u32(DISPLAY_TAG);
        let handler = e.mem.u32(this.wrapping_sub(0x10));
        let end = e.mem.u32(ATTRIBUTE_END_B);
        e.vcall(handler, 0xc, &args![end, body + 0x12, tag]);
        count = e.mem.u32(geometries + 4) as i32;
    }
    let mut position = 0i32;
    if count > 0 {
        loop {
            let data = e.mem.u32(geometries);
            let geometry = e.mem.u32(element(data, position));
            if geometry != 0 {
                e.vcall(geometry, 0, &args![1u32]);
                count = e.mem.u32(geometries + 4) as i32;
            }
            position = position.wrapping_add(1);
            if position >= count {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(geometries + 8);
    e.mem.set_u32(geometries + 4, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(geometries);
        let bytes = (flags & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00d642c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSweptTransformDisplayViewer::entityAddedCallback` (engine map; `this` = `B + 0x28`,
/// `(body)`): nothing happens when `[body + 0x10]` (the shape) is zero or when the record of the
/// key `[body + 8]` is not found (`00d63ff0` on `B`). Otherwise display geometries are built
/// for the shape into a local array (the builder environment `00ddfb70`, the builder
/// `00ddfb80`, `buildDisplayGeometries` `00de0f90`). Walking the array from the end, every
/// geometry whose type (`+0x50`) is 6 and whose word at `+8` is 0 is reported to the logger
/// (virtual `0xc` of `[01268360]`: `(0, -1, text, file, 0xa8)` with the text built in a string
/// buffer from the constant `010d196c`) and removed (the last element takes its place). When
/// geometries remain and the byte at `body + 0xe8` is not 5, the ids `body + 0x11` and
/// `body + 0x12` are appended to the record's array (grown by `00c90940` when full) and, with
/// the two transforms built as in [`fn_00d64020`], the handler at `[this - 0x10]` gets for each
/// id the geometries (virtual 4: `(geometries, transform, id, tag, 0)`) and an attribute call
/// (virtual `0xc`: `(word, id, tag)`, the word being `01268fd8` for the first id and
/// `01268fd4` for the second). Each remaining geometry is then deleted (its virtual 0 with
/// flag 1) and the local array's storage is returned to the router's allocator (virtual
/// `0x10`: `(data, capacity * 4)`) unless the capacity word marks it as not owned.
pub fn hkp_swept_transform_display_viewer_entity_added_callback(
    e: &mut Engine,
    this: Ptr,
    body: Ptr,
) {
    let this = this.addr();
    let body = body.addr();
    if e.mem.u32(body + 0x10) == 0 {
        return;
    }
    let key = e.mem.u32(body + 8);
    let index = e
        .call(FIND_RECORD, &args![this.wrapping_sub(0x28), key])
        .i32();
    if index < 0 {
        return;
    }
    let records = e.mem.u32(this + 8);
    let record = e.mem.u32(element(records, index));
    e.with_stack(0x2e0, |e, frame| {
        show_new_body(e, frame.addr(), this, body, record);
    });
}

// Translated from 00d64630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSweptTransformDisplayViewer::registerViewer` (engine map): registers the process
/// `Swept Transforms` with the factory (`01269cd0`, `registerProcess(name, creator
/// 00d645f0)`) and stores the id it returns in the word at `01268fd0`.
pub fn hkp_swept_transform_display_viewer_register_viewer(e: &mut Engine) {
    let factory = e.mem.u32(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATOR],
        )
        .u32();
    e.mem.set_u32(DISPLAY_TAG, id);
}

// Translated from 00d645f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x3c bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d64260` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d64636.
pub fn fn_00d645f0(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x3c]).u32();
    e.mem.set_u16(block + 4, 0x3c);
    let object = e.call(0x00d6_4260, &args![block, process_contexts]).u32();
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
            0x00d6_3e40,
            hkp_swept_transform_display_viewer_post_simulation_callback(Ptr, Ptr)
        ),
        entry!(0x00d6_3ff0, fn_00d63ff0(Ptr, u32) -> i32),
        entry!(0x00d6_4020, fn_00d64020(Ptr, Ptr)),
        entry!(0x00d6_4120, fn_00d64120(Ptr, Ptr)),
        entry!(0x00d6_41f0, fn_00d641f0(Ptr, u32)),
        entry!(0x00d6_4260, fn_00d64260(Ptr, u32) -> Ptr),
        entry!(
            0x00d6_42c0,
            hkp_swept_transform_display_viewer_entity_added_callback(Ptr, Ptr)
        ),
        entry!(
            0x00d6_4630,
            hkp_swept_transform_display_viewer_register_viewer()
        ),
        entry!(0x00d6_45f0, fn_00d645f0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The handler's vtable doubles sit at `HANDLER_BASE + slot`.
    const HANDLER_BASE: u32 = 0x0900_0000;
    const ALLOCATOR_BASE: u32 = 0x0900_1000;
    const LOGGER_BASE: u32 = 0x0900_2000;
    const GEOMETRY_BASE: u32 = 0x0900_3000;
    const TAG: u32 = 0x77;

    /// A viewer `B` with the handler installed at `+0x18` and the tag word set; the log of
    /// the handler's calls is returned too.
    fn make_viewer(e: &mut Engine) -> (u32, Log) {
        let log = new_log();
        let handler = logging_object(e, &log, HANDLER_BASE);
        let viewer = e.mem.alloc(0x80);
        e.mem.set_u32(viewer + 0x18, handler);
        e.set_global(DISPLAY_TAG, TAG);
        (viewer, log)
    }

    /// A record with the key and the ids; returns its address.
    fn make_record(e: &mut Engine, key: u32, ids: &[u32]) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, key);
        let data = e.mem.alloc(0x40);
        for (position, id) in ids.iter().enumerate() {
            e.mem.set_u32(data + 4 * position as u32, *id);
        }
        e.mem.set_u32(record + 4, data);
        e.mem.set_u32(record + 8, ids.len() as u32);
        e.mem.set_u32(record + 0xc, 0x10);
        record
    }

    /// Installs the records as the viewer's record array.
    fn install_records(e: &mut Engine, viewer: u32, records: &[u32]) {
        let data = e.mem.alloc(0x40);
        for (position, record) in records.iter().enumerate() {
            e.mem.set_u32(data + 4 * position as u32, *record);
        }
        e.mem.set_u32(viewer + 0x30, data);
        e.mem.set_u32(viewer + 0x34, records.len() as u32);
        e.mem.set_u32(viewer + 0x38, 0x10);
    }

    /// A body with the key and the shape, the quaternions at `+0x150` and `+0x160` (four
    /// floats each), the centres at `+0x130` and `+0x140` and the offset at `+0x170`.
    fn make_body(e: &mut Engine, key: u32, shape: u32) -> u32 {
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(body + 8, key);
        e.mem.set_u32(body + 0x10, shape);
        for lane in 0..4 {
            e.mem.set_f32(body + 0x130 + 4 * lane, 100.0 + lane as f32);
            e.mem.set_f32(body + 0x140 + 4 * lane, 200.0 + lane as f32);
            e.mem.set_f32(body + 0x150 + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(body + 0x160 + 4 * lane, 5.0 + lane as f32);
            e.mem.set_f32(body + 0x170 + 4 * lane, 2.0 + lane as f32);
        }
        body
    }

    /// A double for `00cb2d90`: the three columns are the quaternion times 1, 2 and 3 and the
    /// fourth column is filled with 99s, which the caller must overwrite.
    fn install_rotation_double(e: &mut Engine) {
        e.register_double(RECORD_FROM_QUATERNION, |e, a| {
            for lane in 0..4u32 {
                let quaternion = e.mem.f32(a[1] + 4 * lane);
                for column in 0..3u32 {
                    e.mem.set_f32(
                        a[0] + 0x10 * column + 4 * lane,
                        quaternion * (column + 1) as f32,
                    );
                }
                e.mem.set_f32(a[0] + 0x30 + 4 * lane, 99.0);
            }
            Ret::default()
        });
    }

    /// The transform the code must build for the rotation `quaternion` and the centre
    /// `origin` of a body whose offset lanes are `2, 3, 4` (`2 + lane`).
    fn expected_transform(quaternion: f32, origin: f32) -> [[f32; 4]; 4] {
        let offset = [2.0, 3.0, 4.0];
        let mut columns = [[0.0f32; 4]; 4];
        for lane in 0..4 {
            let q = quaternion + lane as f32;
            for (column, scale) in [(0usize, 1.0f32), (1, 2.0), (2, 3.0)] {
                columns[column][lane] = q * scale;
            }
            let rotated = offset[1] * columns[1][lane]
                + offset[0] * columns[0][lane]
                + offset[2] * columns[2][lane];
            columns[3][lane] = origin + lane as f32 - rotated;
        }
        columns
    }

    /// One call of the handler's virtual: (id, tag and the 16 floats of the transform).
    type Shown = (u32, u32, [f32; 16]);

    /// Replaces the handler's slot `0x10` with a double that keeps a copy of the transform.
    fn record_shown(e: &mut Engine) -> Rc<RefCell<Vec<Shown>>> {
        let shown = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&shown);
        e.register_double(HANDLER_BASE + 0x10, move |e, a| {
            let floats = std::array::from_fn(|lane| e.mem.f32(a[1] + 4 * lane as u32));
            sink.borrow_mut().push((a[2], a[3], floats));
            Ret::default()
        });
        shown
    }

    fn flatten(columns: [[f32; 4]; 4]) -> [f32; 16] {
        std::array::from_fn(|lane| columns[lane / 4][lane % 4])
    }

    #[test]
    fn find_record_returns_the_index_of_the_first_matching_key_or_minus_one() {
        let mut e = engine();
        let (viewer, _) = make_viewer(&mut e);
        let records = [
            make_record(&mut e, 10, &[]),
            make_record(&mut e, 20, &[]),
            make_record(&mut e, 20, &[]),
        ];
        install_records(&mut e, viewer, &records);
        assert_eq!(e.call(0x00d6_3ff0, &args![viewer, 20u32]).i32(), 1);
        assert_eq!(e.call(0x00d6_3ff0, &args![viewer, 10u32]).i32(), 0);
        assert_eq!(e.call(0x00d6_3ff0, &args![viewer, 30u32]).i32(), -1);
        e.mem.set_u32(viewer + 0x34, 0);
        assert_eq!(e.call(0x00d6_3ff0, &args![viewer, 10u32]).i32(), -1);
    }

    #[test]
    fn show_transforms_builds_both_poses_and_hands_them_to_the_handler() {
        let mut e = engine();
        let (viewer, _) = make_viewer(&mut e);
        install_rotation_double(&mut e);
        let shown = record_shown(&mut e);
        let body = make_body(&mut e, 1, 0);
        let this = viewer + 0x2c;
        e.call(0x00d6_4020, &args![this, body]);
        let shown = shown.borrow();
        assert_eq!(shown.len(), 2);
        assert_eq!((shown[0].0, shown[0].1), (body + 0x11, TAG));
        assert_eq!((shown[1].0, shown[1].1), (body + 0x12, TAG));
        assert_eq!(shown[0].2, flatten(expected_transform(1.0, 100.0)));
        assert_eq!(shown[1].2, flatten(expected_transform(5.0, 200.0)));
    }

    #[test]
    fn post_simulation_shows_every_body_of_every_island_between_profile_records() {
        let mut e = engine();
        let (viewer, _) = make_viewer(&mut e);
        install_rotation_double(&mut e);
        let shown = record_shown(&mut e);
        let (stream, records) = monitor(&mut e, 0x60);
        let bodies = [
            make_body(&mut e, 1, 0),
            make_body(&mut e, 2, 0),
            make_body(&mut e, 3, 0),
        ];
        let island = |e: &mut Engine, members: &[u32]| {
            let island = e.mem.alloc(0x80);
            let data = e.mem.alloc(0x20);
            for (position, member) in members.iter().enumerate() {
                e.mem.set_u32(data + 4 * position as u32, *member);
            }
            e.mem.set_u32(island + 0x48, data);
            e.mem.set_u32(island + 0x4c, members.len() as u32);
            island
        };
        let islands = [
            island(&mut e, &bodies[..1]),
            island(&mut e, &[]),
            island(&mut e, &bodies[1..]),
        ];
        let world = e.mem.alloc(0x40);
        let list = e.mem.alloc(0x10);
        for (position, island) in islands.iter().enumerate() {
            e.mem.set_u32(list + 4 * position as u32, *island);
        }
        e.mem.set_u32(world + 0x28, list);
        e.mem.set_u32(world + 0x2c, 3);
        e.call(0x00d6_3e40, &args![viewer + 0x2c, world]);
        let ids: Vec<u32> = shown.borrow().iter().map(|call| call.0).collect();
        let want: Vec<u32> = bodies
            .iter()
            .flat_map(|body| [body + 0x11, body + 0x12])
            .collect();
        assert_eq!(ids, want);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_POST_SIMULATION, PROFILE_END]
        );
    }

    #[test]
    fn post_simulation_without_islands_only_records_the_profile_labels() {
        let mut e = engine();
        let (viewer, log) = make_viewer(&mut e);
        let (stream, records) = monitor(&mut e, 0x60);
        let world = e.mem.alloc(0x40);
        e.call(0x00d6_3e40, &args![viewer + 0x2c, world]);
        assert!(log.borrow().is_empty());
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_POST_SIMULATION, PROFILE_END]
        );
    }

    #[test]
    fn removing_a_body_drops_both_ids_and_moves_the_last_id_into_the_hole() {
        let mut e = engine();
        let (viewer, log) = make_viewer(&mut e);
        let body = make_body(&mut e, 7, 0x1234);
        let (first, second) = (body + 0x11, body + 0x12);
        let other = make_record(&mut e, 99, &[]);
        let mine = make_record(&mut e, 7, &[first, 0x500, second]);
        install_records(&mut e, viewer, &[other, mine]);
        let this = viewer + 0x28;
        e.call(0x00d6_4120, &args![this, body]);
        let dropped = calls(&log, HANDLER_BASE + 0x14);
        let handler = e.mem.u32(viewer + 0x18);
        assert_eq!(
            dropped,
            vec![vec![handler, first, TAG, 0], vec![handler, second, TAG, 0]]
        );
        assert_eq!(e.mem.u32(mine + 8), 1);
        assert_eq!(e.mem.u32(e.mem.u32(mine + 4)), 0x500);
    }

    #[test]
    fn removing_a_body_without_a_shape_or_a_record_does_nothing() {
        let mut e = engine();
        let (viewer, log) = make_viewer(&mut e);
        let mine = make_record(&mut e, 7, &[1, 2]);
        install_records(&mut e, viewer, &[mine]);
        let no_shape = make_body(&mut e, 7, 0);
        e.call(0x00d6_4120, &args![viewer + 0x28, no_shape]);
        let unknown = make_body(&mut e, 8, 0x1234);
        e.call(0x00d6_4120, &args![viewer + 0x28, unknown]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(mine + 8), 2);
    }

    #[test]
    fn removing_a_body_still_tells_the_handler_for_ids_the_record_lacks() {
        let mut e = engine();
        let (viewer, log) = make_viewer(&mut e);
        let body = make_body(&mut e, 7, 0x1234);
        let mine = make_record(&mut e, 7, &[]);
        install_records(&mut e, viewer, &[mine]);
        e.call(0x00d6_4120, &args![viewer + 0x28, body]);
        assert_eq!(calls(&log, HANDLER_BASE + 0x14).len(), 2);
        assert_eq!(e.mem.u32(mine + 8), 0);
    }

    #[test]
    fn clearing_a_record_drops_each_id_and_empties_it() {
        let mut e = engine();
        let (viewer, log) = make_viewer(&mut e);
        let router = e.mem.alloc(0x20);
        let router_calls = new_log();
        log_double(&mut e, &router_calls, MEMORY_ROUTER, router);
        let first = make_record(&mut e, 1, &[0x10, 0x20, 0x30]);
        let second = make_record(&mut e, 2, &[0x40]);
        install_records(&mut e, viewer, &[first, second]);
        e.call(0x00d6_41f0, &args![viewer, 0u32]);
        let handler = e.mem.u32(viewer + 0x18);
        assert_eq!(
            calls(&log, HANDLER_BASE + 0x14),
            vec![
                vec![handler, 0x10, TAG, 0],
                vec![handler, 0x20, TAG, 0],
                vec![handler, 0x30, TAG, 0]
            ]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(second + 8), 1);
        assert_eq!(calls(&router_calls, MEMORY_ROUTER).len(), 1);
    }

    #[test]
    fn clearing_an_empty_record_only_asks_the_router() {
        let mut e = engine();
        let (viewer, log) = make_viewer(&mut e);
        let router_calls = new_log();
        log_double(&mut e, &router_calls, MEMORY_ROUTER, 0);
        let empty = make_record(&mut e, 1, &[]);
        install_records(&mut e, viewer, &[empty]);
        e.call(0x00d6_41f0, &args![viewer, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(calls(&router_calls, MEMORY_ROUTER).len(), 1);
    }

    #[test]
    fn the_constructor_runs_the_base_constructor_and_stores_the_vtables() {
        let mut e = engine();
        let base = new_log();
        log_double(&mut e, &base, VIEWER_BASE_CONSTRUCT, 0);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x30, 5);
        let got = e.call(0x00d6_4260, &args![this, 0x4242u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls(&base, VIEWER_BASE_CONSTRUCT),
            vec![vec![this, 0x4242]]
        );
        let words: Vec<u32> = [0, 8, 0x20, 0x28, 0x2c, 0x30, 0x34, 0x38]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![
                0x010d_1918,
                0x010d_18f8,
                0x010d_18e8,
                0x010d_18cc,
                0x010d_18bc,
                0,
                0,
                0x8000_0000
            ]
        );
    }

    #[test]
    fn register_viewer_stores_the_id_the_factory_returns() {
        let mut e = engine();
        let factory_calls = new_log();
        log_double(&mut e, &factory_calls, REGISTER_PROCESS, 0x4d);
        e.mem.set_u32(PROCESS_FACTORY, 0x5000);
        e.call(0x00d6_4630, &args![]);
        assert_eq!(
            calls(&factory_calls, REGISTER_PROCESS),
            vec![vec![0x5000, 0x010d_1880, 0x00d6_45f0]]
        );
        assert_eq!(e.mem.u32(DISPLAY_TAG), 0x4d);
    }

    /// Everything the entity-added callback talks to: the builder doubles (the build fills
    /// the geometry array with `geometries`, each a `(type, word at +8)` pair), the string
    /// buffer doubles, the logger, the router (whose allocator is a logging object) and the
    /// grow function. Returns the log of all of them and the geometry addresses.
    fn install_display_world(e: &mut Engine, geometry_fields: &[(u32, u32)]) -> (Log, Vec<u32>) {
        let log = new_log();
        for address in [
            ENVIRONMENT_CONSTRUCT,
            BUILDER_CONSTRUCT,
            STRING_BUFFER_CONSTRUCT,
            STRING_APPEND,
            STRING_BUFFER_DESTRUCT,
        ] {
            log_double(e, &log, address, 0);
        }
        let geometries: Vec<u32> = geometry_fields
            .iter()
            .enumerate()
            .map(|(position, (kind, word))| {
                let slot = GEOMETRY_BASE + 0x10 * position as u32;
                log_double(e, &log, slot, 0);
                let vtable = e.mem.alloc(8);
                e.mem.set_u32(vtable, slot);
                let geometry = e.mem.alloc(0x60);
                e.mem.set_u32(geometry, vtable);
                e.mem.set_u32(geometry + 8, *word);
                e.mem.set_u32(geometry + 0x50, *kind);
                geometry
            })
            .collect();
        let found = geometries.clone();
        let build_log = Rc::clone(&log);
        e.register_double(BUILD_DISPLAY_GEOMETRIES, move |e, a| {
            build_log
                .borrow_mut()
                .push((BUILD_DISPLAY_GEOMETRIES, a.to_vec()));
            let data = e.mem.alloc(0x40);
            for (position, geometry) in found.iter().enumerate() {
                e.mem.set_u32(data + 4 * position as u32, *geometry);
            }
            e.mem.set_u32(a[2], data);
            e.mem.set_u32(a[2] + 4, found.len() as u32);
            e.mem.set_u32(a[2] + 8, 0x10);
            Ret::default()
        });
        let logger = logging_object(e, &log, LOGGER_BASE);
        e.mem.set_u32(LOGGER, logger);
        let allocator = logging_object(e, &log, ALLOCATOR_BASE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, &log, MEMORY_ROUTER, router);
        e.mem.set_u32(ATTRIBUTE_END_A, 0xa0a0);
        e.mem.set_u32(ATTRIBUTE_END_B, 0xb0b0);
        (log, geometries)
    }

    #[test]
    fn a_new_body_gets_its_geometries_shown_and_the_bad_ones_reported_and_removed() {
        let mut e = engine();
        let (viewer, handler_log) = make_viewer(&mut e);
        install_rotation_double(&mut e);
        let shown_poses = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&shown_poses);
        e.register_double(HANDLER_BASE + 4, move |e, a| {
            let floats: [f32; 16] = std::array::from_fn(|lane| e.mem.f32(a[2] + 4 * lane as u32));
            sink.borrow_mut().push((a.to_vec(), floats));
            Ret::default()
        });
        // Geometries: 0 is a type 6 with no word (reported and removed), 1 is of another
        // type, 2 is a type 6 with a word (kept).
        let (log, geometries) = install_display_world(&mut e, &[(6, 0), (1, 0), (6, 5)]);
        let body = make_body(&mut e, 7, 0x1234);
        let mine = make_record(&mut e, 7, &[0x500]);
        install_records(&mut e, viewer, &[mine]);
        let this = viewer + 0x28;
        e.call(0x00d6_42c0, &args![this, body]);
        // The one bad geometry is reported to the logger with the text buffer's address.
        let reports = calls(&log, LOGGER_BASE + 0xc);
        assert_eq!(reports.len(), 1);
        assert_eq!(&reports[0][1..3], &[0, 0xffff_ffff]);
        assert_eq!(&reports[0][4..], &[WARNING_FILE, 0xa8]);
        assert_eq!(calls(&log, STRING_APPEND)[0][1], WARNING_TEXT);
        assert_eq!(calls(&log, STRING_BUFFER_CONSTRUCT)[0][2], 0x200);
        // The ids joined the record's array.
        assert_eq!(e.mem.u32(mine + 8), 3);
        let ids = e.mem.u32(mine + 4);
        assert_eq!(e.mem.u32(ids + 4), body + 0x11);
        assert_eq!(e.mem.u32(ids + 8), body + 0x12);
        // The handler got both poses with the remaining geometries.
        let shown = shown_poses.borrow();
        assert_eq!(shown.len(), 2);
        assert_eq!(shown[0].0[3], body + 0x11);
        assert_eq!(shown[0].0[4..], [TAG, 0]);
        assert_eq!(shown[1].0[3], body + 0x12);
        assert_eq!(shown[0].1, flatten(expected_transform(1.0, 100.0)));
        assert_eq!(shown[1].1, flatten(expected_transform(5.0, 200.0)));
        assert_eq!(shown[0].0[1], shown[1].0[1]);
        let handler = e.mem.u32(viewer + 0x18);
        assert_eq!(
            calls(&handler_log, HANDLER_BASE + 0xc),
            vec![
                vec![handler, 0xa0a0, body + 0x11, TAG],
                vec![handler, 0xb0b0, body + 0x12, TAG]
            ]
        );
        // The remaining geometries (the last one took the removed one's place) are deleted
        // with flag 1, and the array's storage is returned.
        assert!(calls(&log, GEOMETRY_BASE).is_empty());
        assert_eq!(
            calls(&log, GEOMETRY_BASE + 0x20),
            vec![vec![geometries[2], 1]]
        );
        assert_eq!(
            calls(&log, GEOMETRY_BASE + 0x10),
            vec![vec![geometries[1], 1]]
        );
        let freed = calls(&log, ALLOCATOR_BASE + 0x10);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 0x40);
    }

    #[test]
    fn a_body_with_the_byte_five_is_not_shown_but_its_geometries_are_deleted() {
        let mut e = engine();
        let (viewer, handler_log) = make_viewer(&mut e);
        install_rotation_double(&mut e);
        let (log, geometries) = install_display_world(&mut e, &[(1, 0)]);
        let body = make_body(&mut e, 7, 0x1234);
        e.mem.set_u8(body + 0xe8, 5);
        let mine = make_record(&mut e, 7, &[]);
        install_records(&mut e, viewer, &[mine]);
        e.call(0x00d6_42c0, &args![viewer + 0x28, body]);
        assert!(handler_log.borrow().is_empty());
        assert_eq!(e.mem.u32(mine + 8), 0);
        assert_eq!(calls(&log, GEOMETRY_BASE), vec![vec![geometries[0], 1]]);
    }

    #[test]
    fn a_full_record_array_is_grown_before_an_id_is_appended() {
        let mut e = engine();
        let (viewer, _) = make_viewer(&mut e);
        install_rotation_double(&mut e);
        let (log, _) = install_display_world(&mut e, &[(1, 0)]);
        let body = make_body(&mut e, 7, 0x1234);
        let mine = make_record(&mut e, 7, &[0x500]);
        e.mem.set_u32(mine + 0xc, 1);
        install_records(&mut e, viewer, &[mine]);
        let growth = Rc::clone(&log);
        e.register_double(GROW_BY_ONE, move |e, a| {
            growth.borrow_mut().push((GROW_BY_ONE, a.to_vec()));
            let larger = e.mem.alloc(0x40);
            let old = e.mem.u32(a[1]);
            let bytes = e.mem.bytes(old, 4);
            e.mem.write(larger, &bytes);
            e.mem.set_u32(a[1], larger);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00d6_42c0, &args![viewer + 0x28, body]);
        let grown = calls(&log, GROW_BY_ONE);
        assert_eq!(grown.len(), 1);
        assert_eq!(grown[0][1..], [mine + 4, 4]);
        assert_eq!(e.mem.u32(mine + 8), 3);
        let ids = e.mem.u32(mine + 4);
        assert_eq!(
            [e.mem.u32(ids), e.mem.u32(ids + 4), e.mem.u32(ids + 8)],
            [0x500, body + 0x11, body + 0x12]
        );
    }

    #[test]
    fn a_new_body_without_a_shape_or_a_record_builds_nothing() {
        let mut e = engine();
        let (viewer, _) = make_viewer(&mut e);
        let (log, _) = install_display_world(&mut e, &[(1, 0)]);
        let mine = make_record(&mut e, 7, &[]);
        install_records(&mut e, viewer, &[mine]);
        let no_shape = make_body(&mut e, 7, 0);
        e.call(0x00d6_42c0, &args![viewer + 0x28, no_shape]);
        let unknown = make_body(&mut e, 8, 0x1234);
        e.call(0x00d6_42c0, &args![viewer + 0x28, unknown]);
        assert!(log.borrow().is_empty());
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
    fn fn_00d645f0_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_4260, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_45f0, &args![0x1234u32]).u32();
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
    fn fn_00d645f0_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_4260, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_45f0, &args![0x1234u32]).u32(), 0);
    }
}
