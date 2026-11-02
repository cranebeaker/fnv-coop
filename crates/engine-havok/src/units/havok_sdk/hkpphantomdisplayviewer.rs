//! `Havok SDK/hkpphantomdisplayviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The viewer is a world listener object; both callbacks get `ECX` = its listener sub-object,
//! whose `+4` / `+8` / `+0xC` hold the array of the phantoms being shown (data, size, capacity
//! word), whose word at `-0x10` (phantom added) or `-0x14` (post simulation) is the display
//! handler the callbacks talk to. The words at `01268ff0` and `01268ff4` are passed to the
//! handler as they are (the first looks like the viewer's process id, the second like a
//! colour; neither is confirmed).

use super::hkpsphereshape::{profile_record, PROFILE_END};
#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its first word is the top of the temporary stack, its field
/// `+0x10` the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`) and
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkpShapeDisplayBuilderEnvironment`'s constructor (`00ddfb70`, `this` only),
/// `hkpShapeDisplayBuilder`'s constructor (`00ddfb80`, `this`, the environment) and
/// `buildDisplayGeometries` (`00de0f90`, `this` = the builder, the shape, the array of
/// geometries to fill).
const ENVIRONMENT_CONSTRUCT: u32 = 0x00dd_fb70;
const BUILDER_CONSTRUCT: u32 = 0x00dd_fb80;
const BUILD_DISPLAY_GEOMETRIES: u32 = 0x00de_0f90;
/// The error stream constructor (`00cb1600`, `this`, buffer, size), its text writer
/// (`00cc01c0`, `this`, text) and its destructor (`00cc00e0`, `this`).
const ERROR_STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// `hkDisplayAABB`'s constructor (`00de1210`, `this`) and `setExtents` (`00de1170`, `this` =
/// the display box, the minimum and maximum vectors).
const DISPLAY_AABB_CONSTRUCT: u32 = 0x00de_1210;
const DISPLAY_AABB_SET_EXTENTS: u32 = 0x00de_1170;
/// The global pointer to the error reporter (`01268360`); its slot `0xc` reports a message
/// `(level, id, text, file, line)`.
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The global pointer to the array of phantom types the viewer shows besides types 1 and 2
/// (data at `+8`, size at `+0xc`) and the two words handed on to the display handler.
const SHOWN_TYPES: u32 = 0x0126_8fec;
const HANDLER_WORD_A: u32 = 0x0126_8ff0;
const HANDLER_WORD_B: u32 = 0x0126_8ff4;
/// The error message, the source file of the report and its line.
const GEOMETRY_ERROR_TEXT: u32 = 0x010d_196c;
const GEOMETRY_ERROR_FILE: u32 = 0x010d_1be8;
const GEOMETRY_ERROR_LINE: u32 = 0x91;
/// The profiling label `"TthkpPhantomDisplayViewer"`.
const PROFILE_BEGIN_VIEWER: u32 = 0x010d_1c38;
/// The size in bytes of a display box.
const DISPLAY_AABB_SIZE: u32 = 0x80;

/// The memory router's heap allocator.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The capacity an array grows to when it holds `capacity` and needs room for `wanted`: twice
/// the capacity, or `wanted` when that is not below twice the capacity.
fn grown_capacity(capacity: i32, wanted: i32) -> i32 {
    let doubled = capacity.wrapping_add(capacity);
    if wanted < doubled {
        doubled
    } else {
        wanted
    }
}

/// Takes `bytes` (already rounded) off the router's temporary stack and returns the old top.
fn take_temporary_stack(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(router, top.wrapping_add(bytes));
    top
}

/// Opens the array at `array` over the count in the word at `count_at`, with elements of
/// `element_size` bytes (`1 << shift`): takes the room (rounded up to 128 bytes) off the
/// router's temporary stack (data the stack top, size 0, capacity word `count | 0x80000000`,
/// the original top kept at `+0xc`) and then, as the exe re-reads the count in between, grows
/// the array with `hkArrayUtil::_reserve` when it holds fewer than the new count. Returns the
/// count read the second time.
fn open_stack_array(
    e: &mut Engine,
    array: u32,
    count_at: u32,
    element_size: u32,
    shift: u32,
) -> i32 {
    let wanted = e.mem.u32(count_at);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let rounded = (wanted << shift).wrapping_add(0x7f) & 0xffff_ff80;
    let top = take_temporary_stack(e, rounded);
    e.mem.set_u32(array + 8, wanted | 0x8000_0000);
    let second_count = e.mem.i32(count_at);
    e.mem.set_u32(array, top);
    e.mem.set_u32(array + 0xc, top);
    let heap = allocator(e);
    let have = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if have < second_count {
        let capacity = grown_capacity(have, second_count);
        e.call(ARRAY_RESERVE, &args![heap, array, capacity, element_size]);
    }
    second_count
}

/// Runs the destructor (slot 0, argument 0) of `count` display boxes from `first`.
fn destroy_display_boxes(e: &mut Engine, first: u32, count: i32) {
    for index in 0..count.max(0) as u32 {
        e.vcall(
            first.wrapping_add(index * DISPLAY_AABB_SIZE),
            0,
            &args![0u32],
        );
    }
}

/// Reports "Unable to build display geometry" through a stream over the 512-byte buffer at
/// `buffer`, whose control block is at `stream`.
fn report_geometry_error(e: &mut Engine, stream: u32, buffer: u32) {
    e.call(ERROR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    e.call(STREAM_WRITE_TEXT, &args![stream, GEOMETRY_ERROR_TEXT]);
    let reporter: u32 = e.global(ERROR_REPORTER);
    e.vcall(
        reporter,
        0xc,
        &args![
            0u32,
            0xffff_ffffu32,
            buffer,
            GEOMETRY_ERROR_FILE,
            GEOMETRY_ERROR_LINE
        ],
    );
    e.call(STREAM_DESTRUCT, &args![stream]);
}

// Translated from 00d66530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomDisplayViewer::phantomAddedCallback` (engine map): `ECX` = the listener
/// sub-object (4 bytes below the one `postSimulationCallback` gets) and `phantom` on the
/// stack. Asks the phantom for its type (slot `0x14`) and goes
/// on only when the type is 1 or 2 or is in the array behind `01268fec` (data `+8`, size
/// `+0xc`), and the phantom's shape (the word at `+0x10`, where the collidable starts) is not
/// null. Then builds the display geometries of the shape (`00ddfb70`, `00ddfb80`,
/// `00de0f90`) into a local array and, from the last to the first, drops those whose type word
/// (`+0x50`) is 6 and whose word `+8` is 0, reporting each as an error (stream `00cb1600`,
/// text `00cc01c0`, reporter slot `0xc`, `00cc00e0`) and filling the gap with the last
/// geometry. Appends the phantom to the listener's array (`+8` data, `+0xc` size, `+0x10`
/// capacity word, grown with `00c90940`), tells the display handler (at `-0x10`; slot 4 with
/// the geometries, `[phantom + 0x18]`, the collidable, `01268ff0` and the shape; slot `0xc` with
/// `01268ff4`, the collidable and `01268ff0`), and runs the destructor (slot 0, argument 1) of
/// every geometry that is not null. The geometry array is given back to the allocator (slot
/// `0x10`) unless its capacity word has bit 31 set.
pub fn hkp_phantom_display_viewer_phantom_added_callback(e: &mut Engine, this: Ptr, phantom: Ptr) {
    let listener = this.addr();
    let phantom = phantom.addr();
    e.with_stack(0x250, |e, frame| {
        let frame = frame.addr();
        let geometries = frame + 0x10;
        let stream = frame + 0x20;
        let environment = frame + 0x2c;
        let builder = frame + 0x34;
        let buffer = frame + 0x48;
        let collidable = phantom + 0x10;
        let shape = e.mem.u32(collidable);
        let kind = e.vcall(phantom, 0x14, &args![]).u32();
        e.mem.set_u32(geometries, 0);
        e.mem.set_u32(geometries + 4, 0);
        e.mem.set_u32(geometries + 8, 0x8000_0000);
        let mut shown = kind == 1 || kind == 2;
        if !shown {
            let table: u32 = e.global(SHOWN_TYPES);
            let count = e.mem.i32(table + 0xc);
            let data = e.mem.u32(table + 8);
            shown = (0..count.max(0) as u32).any(|index| e.mem.u32(data + index * 4) == kind);
        }
        if shown && shape != 0 {
            e.call(ENVIRONMENT_CONSTRUCT, &args![environment]);
            e.call(BUILDER_CONSTRUCT, &args![builder, environment]);
            e.call(BUILD_DISPLAY_GEOMETRIES, &args![builder, shape, geometries]);
            let mut index = e.mem.i32(geometries + 4).wrapping_sub(1);
            while index >= 0 {
                let data = e.mem.u32(geometries);
                let geometry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                if e.mem.u32(geometry + 0x50) == 6 && e.mem.u32(geometry + 8) == 0 {
                    report_geometry_error(e, stream, buffer);
                    let size = e.mem.i32(geometries + 4).wrapping_sub(1);
                    e.mem.set_i32(geometries + 4, size);
                    if size != index {
                        let data = e.mem.u32(geometries);
                        let last = e.mem.u32(data.wrapping_add((size as u32).wrapping_mul(4)));
                        e.mem
                            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), last);
                    }
                }
                index -= 1;
            }
            let heap = allocator(e);
            let list = listener + 8;
            if e.mem.u32(list + 4) == e.mem.u32(list + 8) & 0x3fff_ffff {
                e.call(ARRAY_RESERVE_MORE, &args![heap, list, 4u32]);
            }
            let size = e.mem.u32(list + 4);
            let data = e.mem.u32(list);
            e.mem
                .set_u32(data.wrapping_add(size.wrapping_mul(4)), phantom);
            e.mem.set_u32(list + 4, size.wrapping_add(1));
            let handler = e.mem.u32(listener.wrapping_sub(0x10));
            let word_a: u32 = e.global(HANDLER_WORD_A);
            let owner = e.mem.u32(collidable + 8);
            e.vcall(
                handler,
                4,
                &args![geometries, owner, collidable, word_a, shape],
            );
            let handler = e.mem.u32(listener.wrapping_sub(0x10));
            let word_a: u32 = e.global(HANDLER_WORD_A);
            let word_b: u32 = e.global(HANDLER_WORD_B);
            e.vcall(handler, 0xc, &args![word_b, collidable, word_a]);
            let count = e.mem.i32(geometries + 4);
            let mut index = 0;
            while index < count {
                let data = e.mem.u32(geometries);
                let geometry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                if geometry != 0 {
                    e.vcall(geometry, 0, &args![1u32]);
                }
                index += 1;
            }
        }
        let heap = allocator(e);
        e.mem.set_u32(geometries + 4, 0);
        let capacity = e.mem.u32(geometries + 8);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(geometries);
            e.vcall(
                heap,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
    });
}

// Translated from 00d667b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomDisplayViewer::postSimulationCallback` (engine map): `ECX` = the listener
/// sub-object, `step` on the stack (the array of phantoms: data `+0x110`, size `+0x114`).
/// Between the profiling records `"TthkpPhantomDisplayViewer"` and `"Et"`: takes room for one
/// display box (0x80 bytes, rounded up to 128) and one pointer per phantom off the router's
/// temporary stack as two arrays (growing them with `hkArrayUtil::_reserve` when the count has
/// grown in between), constructs the display boxes (`00de1210`), asks every phantom for its
/// box (slot `0x18`, into a local 32-byte box) and points the display box at it
/// (`setExtents`, `00de1170`) while storing a pointer to the display box in the second array,
/// hands the pointer array to the display handler (word at `-0x14`; slot `0x38` with the
/// array, `01268ff4`, 0 and `01268ff0`), and gives both arrays back: the router's stack top
/// returns to where it was, the display boxes are destroyed (slot 0, argument 0) and storage
/// that is not on the temporary stack (capacity word without bit 31) goes to the heap
/// allocator (slot `0x10`). Finally every phantom in the listener's array (`+4` data, `+8`
/// size) is shown (handler slot `0x10` with `[phantom + 0x18]`, `phantom + 0x10`, `01268ff0`).
/// The exe also has a loop destroying boxes beyond the new count; the array is empty when it
/// would run, so it cannot execute.
pub fn hkp_phantom_display_viewer_post_simulation_callback(e: &mut Engine, this: Ptr, step: Ptr) {
    let listener = this.addr();
    let step = step.addr();
    profile_record(e, PROFILE_BEGIN_VIEWER);
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        let boxes = frame + 0x20;
        let pointers = frame + 0x30;
        let extents = frame + 0x40;
        let count = open_stack_array(e, boxes, step + 0x114, DISPLAY_AABB_SIZE, 7);
        let old_size = e.mem.i32(boxes + 4);
        let mut display = e
            .mem
            .u32(boxes)
            .wrapping_add((old_size as u32).wrapping_mul(DISPLAY_AABB_SIZE));
        for _ in 0..count.wrapping_sub(old_size).max(0) {
            if display != 0 {
                e.call(DISPLAY_AABB_CONSTRUCT, &args![display]);
            }
            display = display.wrapping_add(DISPLAY_AABB_SIZE);
        }
        e.mem.set_i32(boxes + 4, count);
        let count = open_stack_array(e, pointers, step + 0x114, 4, 2);
        e.mem.set_i32(pointers + 4, count);
        let mut index = 0u32;
        let mut offset = 0u32;
        if e.mem.i32(step + 0x114) > 0 {
            loop {
                let phantoms = e.mem.u32(step + 0x110);
                let phantom = e.mem.u32(phantoms + index * 4);
                e.vcall(phantom, 0x18, &args![extents]);
                let data = e.mem.u32(boxes);
                e.call(
                    DISPLAY_AABB_SET_EXTENTS,
                    &args![data.wrapping_add(offset), extents, extents + 0x10],
                );
                let data = e.mem.u32(boxes);
                let list = e.mem.u32(pointers);
                e.mem.set_u32(list + index * 4, data.wrapping_add(offset));
                index += 1;
                offset = offset.wrapping_add(DISPLAY_AABB_SIZE);
                if index as i32 >= e.mem.i32(step + 0x114) {
                    break;
                }
            }
        }
        let handler = e.mem.u32(listener.wrapping_sub(0x14));
        let word_a: u32 = e.global(HANDLER_WORD_A);
        let word_b: u32 = e.global(HANDLER_WORD_B);
        e.vcall(handler, 0x38, &args![pointers, word_b, 0u32, word_a]);
        // The pointer array goes back.
        let saved = e.mem.u32(pointers + 0xc);
        if saved == e.mem.u32(pointers) {
            e.mem.set_u32(pointers + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, saved);
        let heap = allocator(e);
        let capacity = e.mem.u32(pointers + 8);
        e.mem.set_u32(pointers + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(pointers);
            e.vcall(
                heap,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        e.mem.set_u32(pointers, 0);
        e.mem.set_u32(pointers + 8, 0x8000_0000);
        // The display boxes go back.
        let first = e.mem.u32(boxes);
        if e.mem.u32(boxes + 0xc) == first {
            let size = e.mem.i32(boxes + 4);
            destroy_display_boxes(e, first, size);
            e.mem.set_u32(boxes + 4, 0);
        }
        let saved = e.mem.u32(boxes + 0xc);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, saved);
        let heap = allocator(e);
        let size = e.mem.i32(boxes + 4);
        let first = e.mem.u32(boxes);
        destroy_display_boxes(e, first, size);
        let capacity = e.mem.u32(boxes + 8);
        e.mem.set_u32(boxes + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(boxes);
            e.vcall(heap, 0x10, &args![data, (capacity & 0x3fff_ffff) << 7]);
        }
        // Everything the listener shows is brought up to date.
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(listener + 8) {
            let list = e.mem.u32(listener + 4);
            let shown = e.mem.u32(list + index * 4);
            let handler = e.mem.u32(listener.wrapping_sub(0x14));
            let word_a: u32 = e.global(HANDLER_WORD_A);
            let owner = e.mem.u32(shown + 0x18);
            e.vcall(handler, 0x10, &args![owner, shown + 0x10, word_a]);
            index += 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d66770 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x3c bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d66430` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d66af6.
pub fn fn_00d66770(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x3c]).u32();
    e.mem.set_u16(block + 4, 0x3c);
    let object = e.call(0x00d6_6430, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}
// Translated from 00d66730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x14 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, the vtable word `010d1c2c` at +0, two zero words at +8 and +0xc and `0x80000000` at +0x10 (an empty array that owns no storage). Returns the block. Ghidra made no function here; the address is stored in the data word at 011b5f44.
pub fn fn_00d66730(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(block + 4, 0x14);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, 0x010d_1c2c);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0x8000_0000);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d66530,
            hkp_phantom_display_viewer_phantom_added_callback(Ptr, Ptr)
        ),
        entry!(
            0x00d667b0,
            hkp_phantom_display_viewer_post_simulation_callback(Ptr, Ptr)
        ),
        entry!(0x00d6_6770, fn_00d66770(u32) -> u32),
        entry!(0x00d6_6730, fn_00d66730() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_GET_TYPE: u32 = 0x0900_0501;
    const SLOT_ADD_GEOMETRIES: u32 = 0x0900_0502;
    const SLOT_SET_COLOUR: u32 = 0x0900_0503;
    const SLOT_FREE: u32 = 0x0900_0504;
    const SLOT_GEOMETRY_DESTRUCT: u32 = 0x0900_0505;
    const SLOT_REPORT: u32 = 0x0900_0506;
    const SLOT_GET_AABB: u32 = 0x0900_0507;
    const SLOT_SHOW: u32 = 0x0900_0508;
    const SLOT_UPDATE_ALL: u32 = 0x0900_0509;
    const SLOT_BOX_DESTRUCT: u32 = 0x0900_050a;
    const WORD_A: u32 = 0x1234;
    const WORD_B: u32 = 0x5678;

    fn record(e: &mut Engine, log: &Log, address: u32, result: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(at, _)| *at).collect()
    }

    struct Scene {
        e: Engine,
        log: Log,
        listener: u32,
        handler: u32,
        heap: u32,
        router: u32,
    }

    /// An engine with the exe's data range, the handler words, an error reporter, a memory router
    /// (stack top `0x2000` bytes into a block; a heap allocator logging its frees) and a
    /// handler object at `listener - 0x14` and `listener - 0x10` (the same object).
    fn scene() -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        e.set_global(HANDLER_WORD_A, WORD_A);
        e.set_global(HANDLER_WORD_B, WORD_B);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, SLOT_FREE);
        let heap = e.mem.alloc(0x10);
        e.mem.set_u32(heap, table);
        let router = e.mem.alloc(0x20);
        let stack = e.mem.alloc(0x4000);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let handler_table = e.mem.alloc(0x80);
        e.mem.set_u32(handler_table + 4, SLOT_ADD_GEOMETRIES);
        e.mem.set_u32(handler_table + 0xc, SLOT_SET_COLOUR);
        e.mem.set_u32(handler_table + 0x10, SLOT_SHOW);
        e.mem.set_u32(handler_table + 0x38, SLOT_UPDATE_ALL);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(handler, handler_table);
        let listener = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u32(listener - 0x10, handler);
        e.mem.set_u32(listener - 0x14, handler);
        record(&mut e, &log, SLOT_FREE, 0);
        Scene {
            e,
            log,
            listener,
            handler,
            heap,
            router,
        }
    }

    /// A geometry object: type word `kind` at `+0x50`, word `+8` = `flag`, destructor logged.
    fn geometry(e: &mut Engine, kind: u32, flag: u32) -> u32 {
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, SLOT_GEOMETRY_DESTRUCT);
        let geometry = e.mem.alloc(0x60);
        e.mem.set_u32(geometry, table);
        e.mem.set_u32(geometry + 0x50, kind);
        e.mem.set_u32(geometry + 8, flag);
        geometry
    }

    /// A phantom of type `kind` with shape `shape` (the word at `+0x10`) and owner word 0x4242
    /// at `+0x18`.
    fn phantom(e: &mut Engine, log: &Log, kind: u32, shape: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x14, SLOT_GET_TYPE);
        e.mem.set_u32(table + 0x18, SLOT_GET_AABB);
        let phantom = e.mem.alloc(0x40);
        e.mem.set_u32(phantom, table);
        e.mem.set_u32(phantom + 0x10, shape);
        e.mem.set_u32(phantom + 0x18, 0x4242);
        record(e, log, SLOT_GET_TYPE, kind);
        phantom
    }

    /// Doubles for everything `phantomAddedCallback` calls outside the handler; the geometry
    /// builder fills the array with `geometries` (a block of its own, capacity = count).
    fn builder_doubles(s: &mut Scene, geometries: Vec<u32>) {
        for address in [
            ENVIRONMENT_CONSTRUCT,
            BUILDER_CONSTRUCT,
            ERROR_STREAM_CONSTRUCT,
            STREAM_WRITE_TEXT,
            STREAM_DESTRUCT,
            SLOT_ADD_GEOMETRIES,
            SLOT_SET_COLOUR,
            SLOT_GEOMETRY_DESTRUCT,
            SLOT_REPORT,
        ] {
            record(&mut s.e, &s.log, address, 0);
        }
        let sink = Rc::clone(&s.log);
        s.e.register_double(BUILD_DISPLAY_GEOMETRIES, move |e, a| {
            sink.borrow_mut()
                .push((BUILD_DISPLAY_GEOMETRIES, a.to_vec()));
            let block = e.mem.alloc(0x40);
            for (index, geometry) in geometries.iter().enumerate() {
                e.mem.set_u32(block + 4 * index as u32, *geometry);
            }
            e.mem.set_u32(a[2], block);
            e.mem.set_u32(a[2] + 4, geometries.len() as u32);
            e.mem.set_u32(a[2] + 8, geometries.len() as u32 + 4);
            Ret::default()
        });
        let table = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(table + 0xc, SLOT_REPORT);
        let reporter = s.e.mem.alloc(8);
        s.e.mem.set_u32(reporter, table);
        s.e.set_global(ERROR_REPORTER, reporter);
        let types = s.e.mem.alloc(0x20);
        let data = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(data, 7);
        s.e.mem.set_u32(data + 4, 9);
        s.e.mem.set_u32(types + 8, data);
        s.e.mem.set_u32(types + 0xc, 2);
        s.e.set_global(SHOWN_TYPES, types);
    }

    fn list_with(s: &mut Scene, entries: &[u32], capacity: u32) {
        let data = s.e.mem.alloc(0x40);
        for (index, entry) in entries.iter().enumerate() {
            s.e.mem.set_u32(data + 4 * index as u32, *entry);
        }
        s.e.mem.set_u32(s.listener + 4, data);
        s.e.mem.set_u32(s.listener + 8, entries.len() as u32);
        s.e.mem.set_u32(s.listener + 0xc, capacity);
    }

    #[test]
    fn a_plain_shape_is_built_added_shown_and_its_geometries_destroyed() {
        let mut s = scene();
        let first = geometry(&mut s.e, 3, 1);
        let second = geometry(&mut s.e, 6, 5);
        builder_doubles(&mut s, vec![first, second]);
        let shape = s.e.mem.alloc(0x10);
        let p = phantom(&mut s.e, &s.log, 1, shape);
        list_with(&mut s, &[], 4);
        s.e.call(0x00d6_6530, &args![s.listener - 4, p]);
        assert_eq!(
            order(&s.log),
            vec![
                SLOT_GET_TYPE,
                ENVIRONMENT_CONSTRUCT,
                BUILDER_CONSTRUCT,
                BUILD_DISPLAY_GEOMETRIES,
                SLOT_ADD_GEOMETRIES,
                SLOT_SET_COLOUR,
                SLOT_GEOMETRY_DESTRUCT,
                SLOT_GEOMETRY_DESTRUCT,
                SLOT_FREE,
            ]
        );
        let build = &calls_to(&s.log, BUILD_DISPLAY_GEOMETRIES)[0];
        let (builder, geometries) = (build[0], build[2]);
        assert_eq!(build[1], shape);
        assert_eq!(
            calls_to(&s.log, BUILDER_CONSTRUCT),
            vec![vec![builder, calls_to(&s.log, ENVIRONMENT_CONSTRUCT)[0][0]]]
        );
        assert_eq!(
            calls_to(&s.log, SLOT_ADD_GEOMETRIES),
            vec![vec![s.handler, geometries, 0x4242, p + 0x10, WORD_A, shape]]
        );
        assert_eq!(
            calls_to(&s.log, SLOT_SET_COLOUR),
            vec![vec![s.handler, WORD_B, p + 0x10, WORD_A]]
        );
        assert_eq!(
            calls_to(&s.log, SLOT_GEOMETRY_DESTRUCT),
            vec![vec![first, 1], vec![second, 1]]
        );
        // The phantom joined the listener's list; the geometry block (capacity 6) was freed.
        assert_eq!(s.e.mem.u32(s.listener + 8), 1);
        assert_eq!(s.e.mem.u32(s.e.mem.u32(s.listener + 4)), p);
        let freed = calls_to(&s.log, SLOT_FREE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][0], s.heap);
        assert_eq!(freed[0][2], 24);
    }

    #[test]
    fn other_types_need_to_be_in_the_table() {
        let mut s = scene();
        builder_doubles(&mut s, vec![]);
        let shape = s.e.mem.alloc(0x10);
        list_with(&mut s, &[], 4);
        let absent = phantom(&mut s.e, &s.log, 8, shape);
        s.e.call(0x00d6_6530, &args![s.listener - 4, absent]);
        assert_eq!(order(&s.log), vec![SLOT_GET_TYPE]);
        assert_eq!(s.e.mem.u32(s.listener + 8), 0);
        s.log.borrow_mut().clear();
        let present = phantom(&mut s.e, &s.log, 9, shape);
        s.e.call(0x00d6_6530, &args![s.listener - 4, present]);
        assert_eq!(calls_to(&s.log, BUILD_DISPLAY_GEOMETRIES).len(), 1);
        assert_eq!(s.e.mem.u32(s.listener + 8), 1);
        // An empty table finds nothing either.
        s.log.borrow_mut().clear();
        let types: u32 = s.e.global(SHOWN_TYPES);
        s.e.mem.set_u32(types + 0xc, 0);
        s.e.call(0x00d6_6530, &args![s.listener - 4, present]);
        assert_eq!(order(&s.log), vec![SLOT_GET_TYPE]);
    }

    #[test]
    fn a_phantom_without_a_shape_is_skipped_even_for_type_two() {
        let mut s = scene();
        builder_doubles(&mut s, vec![]);
        list_with(&mut s, &[], 4);
        let p = phantom(&mut s.e, &s.log, 2, 0);
        s.e.call(0x00d6_6530, &args![s.listener - 4, p]);
        assert_eq!(order(&s.log), vec![SLOT_GET_TYPE]);
        assert_eq!(s.e.mem.u32(s.listener + 8), 0);
    }

    #[test]
    fn unbuildable_geometries_are_reported_and_replaced_by_the_last_one() {
        let mut s = scene();
        let bad = geometry(&mut s.e, 6, 0);
        let good = geometry(&mut s.e, 3, 0);
        let also_bad = geometry(&mut s.e, 6, 0);
        // Type 6 with a nonzero word +8 stays.
        let kept = geometry(&mut s.e, 6, 2);
        builder_doubles(&mut s, vec![bad, good, kept, also_bad]);
        let shape = s.e.mem.alloc(0x10);
        let p = phantom(&mut s.e, &s.log, 2, shape);
        list_with(&mut s, &[], 4);
        s.e.call(0x00d6_6530, &args![s.listener - 4, p]);
        // From the last to the first: `also_bad` is dropped (it is last), then `bad` is
        // replaced by `kept`.
        assert_eq!(calls_to(&s.log, ERROR_STREAM_CONSTRUCT).len(), 2);
        let report = &calls_to(&s.log, SLOT_REPORT)[0];
        let reporter: u32 = s.e.global(ERROR_REPORTER);
        let buffer = calls_to(&s.log, ERROR_STREAM_CONSTRUCT)[0][1];
        assert_eq!(
            report[..],
            [reporter, 0, 0xffff_ffff, buffer, GEOMETRY_ERROR_FILE, 0x91]
        );
        assert_eq!(
            calls_to(&s.log, STREAM_WRITE_TEXT)[0][1],
            GEOMETRY_ERROR_TEXT
        );
        assert_eq!(calls_to(&s.log, ERROR_STREAM_CONSTRUCT)[0][2], 0x200);
        assert_eq!(
            calls_to(&s.log, STREAM_DESTRUCT)[0],
            calls_to(&s.log, STREAM_WRITE_TEXT)[0][..1].to_vec()
        );
        assert_eq!(
            calls_to(&s.log, SLOT_GEOMETRY_DESTRUCT),
            vec![vec![kept, 1], vec![good, 1]]
        );
    }

    #[test]
    fn a_full_listener_array_grows_first() {
        let mut s = scene();
        builder_doubles(&mut s, vec![]);
        let shape = s.e.mem.alloc(0x10);
        let p = phantom(&mut s.e, &s.log, 1, shape);
        list_with(&mut s, &[5, 6], 2);
        let grow_log = Rc::clone(&s.log);
        s.e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let room = e.mem.alloc(0x40);
            for index in 0..2 {
                let old = e.mem.u32(e.mem.u32(a[1]) + 4 * index);
                e.mem.set_u32(room + 4 * index, old);
            }
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        s.e.call(0x00d6_6530, &args![s.listener - 4, p]);
        assert_eq!(
            calls_to(&s.log, ARRAY_RESERVE_MORE),
            vec![vec![s.heap, s.listener + 4, 4]]
        );
        let data = s.e.mem.u32(s.listener + 4);
        assert_eq!(
            (0..3)
                .map(|i| s.e.mem.u32(data + 4 * i))
                .collect::<Vec<_>>(),
            vec![5, 6, p]
        );
        assert_eq!(s.e.mem.u32(s.listener + 8), 3);
        // The empty geometry block (capacity 4 from the double) is still given back.
        assert_eq!(calls_to(&s.log, SLOT_FREE).len(), 1);
    }

    struct StepScene {
        s: Scene,
        step: u32,
        records: u32,
        top: u32,
    }

    /// A step object with `phantoms` phantoms (slot `0x18` writes the box with all lanes of the
    /// minimum `n` and of the maximum `n + 1`, `n` being the word at `+8` of the phantom), a
    /// profiling stream, and doubles for the display box functions: the constructor gives the
    /// box a vtable whose destructor is logged, `setExtents` logs the box and the first lane of
    /// each extent.
    fn step_scene(phantoms: u32) -> StepScene {
        let mut s = scene();
        s.e.set_global(PROFILE_SLOT, 3u32);
        let records = s.e.mem.alloc(0x30);
        let stream = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(stream + 4, records);
        s.e.mem.set_u32(stream + 0xc, records + 0x30);
        s.e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let table = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(table + 0x18, SLOT_GET_AABB);
        let step = s.e.mem.alloc(0x200);
        let list = s.e.mem.alloc(0x40);
        for index in 0..phantoms {
            let object = s.e.mem.alloc(0x20);
            s.e.mem.set_u32(object, table);
            s.e.mem.set_u32(object + 8, index);
            s.e.mem.set_u32(list + 4 * index, object);
        }
        s.e.mem.set_u32(step + 0x110, list);
        s.e.mem.set_u32(step + 0x114, phantoms);
        let sink = Rc::clone(&s.log);
        s.e.register_double(SLOT_GET_AABB, move |e, a| {
            sink.borrow_mut().push((SLOT_GET_AABB, a.to_vec()));
            let index = e.mem.u32(a[0] + 8) as f32;
            for lane in 0..4 {
                e.mem.set_f32(a[1] + 4 * lane, index);
                e.mem.set_f32(a[1] + 0x10 + 4 * lane, index + 1.0);
            }
            Ret::default()
        });
        let box_table = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(box_table, SLOT_BOX_DESTRUCT);
        let sink = Rc::clone(&s.log);
        s.e.register_double(DISPLAY_AABB_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((DISPLAY_AABB_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0], box_table);
            Ret::default()
        });
        let sink = Rc::clone(&s.log);
        s.e.register_double(DISPLAY_AABB_SET_EXTENTS, move |e, a| {
            sink.borrow_mut().push((
                DISPLAY_AABB_SET_EXTENTS,
                vec![a[0], e.mem.f32(a[1]).to_bits(), e.mem.f32(a[2]).to_bits()],
            ));
            Ret::default()
        });
        let sink = Rc::clone(&s.log);
        s.e.register_double(SLOT_UPDATE_ALL, move |e, a| {
            sink.borrow_mut().push((
                SLOT_UPDATE_ALL,
                vec![a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 4), a[2], a[3], a[4]],
            ));
            Ret::default()
        });
        for address in [SLOT_SHOW, SLOT_BOX_DESTRUCT] {
            record(&mut s.e, &s.log, address, 0);
        }
        let top = s.e.mem.u32(s.router);
        StepScene {
            s,
            step,
            records,
            top,
        }
    }

    fn shown_item(e: &mut Engine, owner: u32) -> u32 {
        let item = e.mem.alloc(0x40);
        e.mem.set_u32(item + 0x18, owner);
        item
    }

    #[test]
    fn every_phantom_gets_a_display_box_and_the_handler_gets_the_pointers() {
        let mut t = step_scene(3);
        let first = shown_item(&mut t.s.e, 0x71);
        let second = shown_item(&mut t.s.e, 0x72);
        list_with(&mut t.s, &[first, second], 4);
        let (listener, top) = (t.s.listener, t.top);
        t.s.e.call(0x00d6_67b0, &args![listener, t.step]);
        let boxes: Vec<u32> = (0..3).map(|n| top + n * 0x80).collect();
        let pointers = top + 0x180;
        let mut expected = vec![DISPLAY_AABB_CONSTRUCT; 3];
        for _ in 0..3 {
            expected.push(SLOT_GET_AABB);
            expected.push(DISPLAY_AABB_SET_EXTENTS);
        }
        expected.push(SLOT_UPDATE_ALL);
        expected.extend([SLOT_BOX_DESTRUCT; 3]);
        expected.extend([SLOT_SHOW; 2]);
        assert_eq!(order(&t.s.log), expected);
        assert_eq!(
            calls_to(&t.s.log, DISPLAY_AABB_CONSTRUCT),
            boxes.iter().map(|b| vec![*b]).collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&t.s.log, DISPLAY_AABB_SET_EXTENTS),
            (0..3u32)
                .map(|n| vec![
                    boxes[n as usize],
                    (n as f32).to_bits(),
                    (n as f32 + 1.0).to_bits()
                ])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&t.s.log, SLOT_UPDATE_ALL),
            vec![vec![t.s.handler, pointers, 3, WORD_B, 0, WORD_A]]
        );
        for (n, display) in boxes.iter().enumerate() {
            assert_eq!(t.s.e.mem.u32(pointers + 4 * n as u32), *display);
        }
        assert_eq!(
            calls_to(&t.s.log, SLOT_BOX_DESTRUCT),
            boxes.iter().map(|b| vec![*b, 0]).collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&t.s.log, SLOT_SHOW),
            vec![
                vec![t.s.handler, 0x71, first + 0x10, WORD_A],
                vec![t.s.handler, 0x72, second + 0x10, WORD_A],
            ]
        );
        // The arrays were on the temporary stack: nothing was freed and the top is back.
        assert!(calls_to(&t.s.log, SLOT_FREE).is_empty());
        assert!(calls_to(&t.s.log, ARRAY_RESERVE).is_empty());
        assert_eq!(t.s.e.mem.u32(t.s.router), top);
        assert_eq!(t.s.e.mem.u32(t.records), PROFILE_BEGIN_VIEWER);
        assert_eq!(t.s.e.mem.u32(t.records + 12), PROFILE_END);
        assert_eq!(t.s.e.mem.u32(t.step + 0x114), 3);
    }

    #[test]
    fn an_empty_step_only_updates_the_handler_and_shows_the_listed_items() {
        let mut t = step_scene(0);
        list_with(&mut t.s, &[], 4);
        let (listener, top) = (t.s.listener, t.top);
        t.s.e.call(0x00d6_67b0, &args![listener, t.step]);
        assert_eq!(order(&t.s.log), vec![SLOT_UPDATE_ALL]);
        assert_eq!(
            calls_to(&t.s.log, SLOT_UPDATE_ALL),
            vec![vec![t.s.handler, top, 0, WORD_B, 0, WORD_A]]
        );
        assert_eq!(t.s.e.mem.u32(t.s.router), top);
        assert_eq!(t.s.e.mem.u32(t.records), PROFILE_BEGIN_VIEWER);
        assert_eq!(t.s.e.mem.u32(t.records + 12), PROFILE_END);
    }

    /// Makes the router double raise the step's count to `count` the `call`-th time it runs
    /// (so that the exe's second read of the count sees a larger one).
    fn grow_count_at_router_call(t: &mut StepScene, call: u32, count: u32) {
        let router = t.s.router;
        let step = t.step;
        let calls = Rc::new(RefCell::new(0u32));
        t.s.e.register_double(MEMORY_ROUTER, move |e, _| {
            *calls.borrow_mut() += 1;
            if *calls.borrow() == call {
                e.mem.set_u32(step + 0x114, count);
            }
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
    }

    /// Makes `_reserve` give the array a heap block of `capacity` elements of `size` bytes.
    fn heap_reserve(t: &mut StepScene) {
        let sink = Rc::clone(&t.s.log);
        t.s.e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let block = e.mem.alloc(a[2] * a[3] + 0x10);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    #[test]
    fn a_grown_count_moves_the_display_boxes_to_the_heap() {
        let mut t = step_scene(5);
        t.s.e.mem.set_u32(t.step + 0x114, 2);
        list_with(&mut t.s, &[], 4);
        grow_count_at_router_call(&mut t, 1, 5);
        heap_reserve(&mut t);
        let (listener, top) = (t.s.listener, t.top);
        t.s.e.call(0x00d6_67b0, &args![listener, t.step]);
        let reserve = calls_to(&t.s.log, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        // Capacity 2 grows to 5 (twice 2 is 4, below 5): 128-byte elements.
        assert_eq!(reserve[0][0], t.s.heap);
        assert_eq!(reserve[0][2..], [5, 0x80]);
        let block = t.s.e.mem.u32(reserve[0][1]);
        let constructed = calls_to(&t.s.log, DISPLAY_AABB_CONSTRUCT);
        assert_eq!(
            constructed,
            (0..5).map(|n| vec![block + n * 0x80]).collect::<Vec<_>>()
        );
        // The boxes were not on the temporary stack, so they are destroyed after the stack top
        // went back, and the block is freed (capacity 5, 0x80 bytes each).
        assert_eq!(calls_to(&t.s.log, SLOT_BOX_DESTRUCT).len(), 5);
        assert_eq!(
            calls_to(&t.s.log, SLOT_FREE),
            vec![vec![t.s.heap, block, 5 * 0x80]]
        );
        assert_eq!(t.s.e.mem.u32(t.s.router), top);
        assert_eq!(calls_to(&t.s.log, SLOT_GET_AABB).len(), 5);
    }

    #[test]
    fn a_grown_count_moves_the_pointer_array_to_the_heap() {
        let mut t = step_scene(5);
        t.s.e.mem.set_u32(t.step + 0x114, 2);
        list_with(&mut t.s, &[], 4);
        grow_count_at_router_call(&mut t, 3, 5);
        heap_reserve(&mut t);
        let (listener, top) = (t.s.listener, t.top);
        t.s.e.call(0x00d6_67b0, &args![listener, t.step]);
        let reserve = calls_to(&t.s.log, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][0], t.s.heap);
        assert_eq!(reserve[0][2..], [5, 4]);
        // The exe clears the array afterwards, so the block is taken from the free.
        let block = calls_to(&t.s.log, SLOT_FREE)[0][1];
        let update = calls_to(&t.s.log, SLOT_UPDATE_ALL);
        assert_eq!(update, vec![vec![t.s.handler, block, 5, WORD_B, 0, WORD_A]]);
        // The first two boxes (the count was 2 when they were made) are on the stack.
        assert_eq!(calls_to(&t.s.log, DISPLAY_AABB_CONSTRUCT).len(), 2);
        assert_eq!(t.s.e.mem.u32(block), top);
        assert_eq!(t.s.e.mem.u32(block + 4), top + 0x80);
        assert_eq!(
            calls_to(&t.s.log, SLOT_FREE),
            vec![vec![t.s.heap, block, 5 * 4]]
        );
        assert_eq!(t.s.e.mem.u32(t.s.router), top);
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
    fn fn_00d66770_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_6430, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_6770, &args![0x1234u32]).u32();
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
    fn fn_00d66770_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_6430, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_6770, &args![0x1234u32]).u32(), 0);
    }
    #[test]
    fn fn_00d66730_allocates_and_initialises_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e.call(0x00d6_6730, &args![]).u32();
        assert_eq!(*sizes.borrow(), vec![0x14]);
        assert_eq!(e.mem.u16(block + 4), 0x14);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), 0x010d_1c2c);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0x8000_0000);
    }
}
