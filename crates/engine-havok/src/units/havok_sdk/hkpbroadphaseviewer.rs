//! `Havok SDK/hkpbroadphaseviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::hkpsphereshape::{profile_record, PROFILE_END};
#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d6ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_ac40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d6ada0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_ac40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d6ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d6ada0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6ad70(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_ada0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d6ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d6ada0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6ad80(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_ada0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d6ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d6ada0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6ad90(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_ada0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size), the
/// `hkDisplayAABB` constructor (`00de1210`, `this` only) and `hkDisplayAABB::setExtents`
/// (`00de1170`, `this` the display box, the minimum and maximum vectors), the removal of a
/// listener (`00c918e0`, `this` an entry of the world's array at `+0x6C`, one word) and the
/// base class destructor (`00ddcc60`, `this` only).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const DISPLAY_AABB_CONSTRUCT: u32 = 0x00de_1210;
const DISPLAY_AABB_SET_EXTENTS: u32 = 0x00de_1170;
const REMOVE_LISTENER: u32 = 0x00c9_18e0;
const BASE_DESTRUCT: u32 = 0x00dd_cc60;
/// The profiling label `"TthkpBroadphaseViewer"`.
const PROFILE_BEGIN_VIEWER: u32 = 0x010d_23c0;
/// The process id `registerViewer` stores (`01269028`), the process factory object
/// (`01269cd0`), `hkProcessFactory::registerProcess` (`00dd9380`, `this` the factory, the
/// name and the creation function) and its two arguments: the string `"Broadphase"` at
/// `010c65b0` and the creation function at `00d6ad00`.
const PROCESS_ID: u32 = 0x0126_9028;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_NAME: u32 = 0x010c_65b0;
const PROCESS_CREATE: u32 = 0x00d6_ad00;
/// The vtables the destructor stores in the viewer (`+0`, `+8`, `+0x20` and the listener
/// sub-object at `+0x28`) and the one it ends with at `+0x28`.
const VTABLE_VIEWER: u32 = 0x010d_23b4;
const VTABLE_PROCESS: u32 = 0x010d_2394;
const VTABLE_CONTEXT_OWNER: u32 = 0x010d_2384;
const VTABLE_LISTENER: u32 = 0x010d_2374;
const VTABLE_LISTENER_DESTROYED: u32 = 0x010c_cf7c;
/// The size in bytes of a display box.
const DISPLAY_AABB_SIZE: u32 = 0x80;
/// The colour handed to the display handler with the boxes.
const DISPLAY_COLOUR: u32 = 0xffff_0000;

/// The allocator behind the memory router.
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

// Translated from 00d6a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadphaseViewer::postSimulationCallback` (engine map): `ECX` = the listener
/// sub-object (`viewer + 0x28`; its `+4`, `+8` and `+0xC` are the data, size and capacity word
/// of the array of display boxes, 0x80 bytes each; the display handler is the word at `-0x10`),
/// the stepping object on the stack. Between the profiling records `"TthkpBroadphaseViewer"`
/// and `"Et"`: takes a chunk of 0x8000 bytes from the allocator (slot `0xC`, which writes the
/// size it gave back) as an array of `hkAabb`s (0x20 bytes each; capacity the size divided by
/// 32, or `0x80000000` when that is 0; length 0x400), has the object at `[step + 0x58]` fill it
/// (slot `0x38`, which sets the length to the number of boxes), grows the display boxes to
/// that number (reserving capacity, constructing the new boxes), points each display box at
/// its `hkAabb` (`setExtents`, from the last to the first) while collecting pointers to the
/// boxes in a second array, passes that array with the colour `0xffff0000`, 0 and the process
/// id `01269028` to the display handler (slot `0x38`), and frees both arrays (slot `0x10`,
/// unless their capacity word has bit 31 set). The exe also destroys boxes from the new
/// number up to the old one, which cannot happen since it only grows.
pub fn hkp_broadphase_viewer_post_simulation_callback(e: &mut Engine, this: Ptr, step: Ptr) {
    let listener = this.addr();
    profile_record(e, PROFILE_BEGIN_VIEWER);
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        let aabbs = frame + 0x18;
        let pointers = frame + 0x24;
        e.mem.set_u32(aabbs, 0);
        e.mem.set_u32(aabbs + 4, 0);
        e.mem.set_u32(aabbs + 8, 0x8000_0000);
        let chunks = allocator(e);
        e.mem.set_u32(frame + 0x14, 0x8000);
        let block = e.vcall(chunks, 0xc, &args![frame + 0x14]).u32();
        let bytes = e.mem.u32(frame + 0x14) as i32;
        let mut capacity = (bytes + ((bytes >> 31) & 0x1f)) >> 5;
        if capacity == 0 {
            capacity = i32::MIN;
        }
        e.mem.set_u32(aabbs, block);
        let filler = e.mem.u32(step.addr() + 0x58);
        e.mem.set_u32(aabbs + 8, capacity as u32);
        e.mem.set_u32(aabbs + 4, 0x400);
        e.vcall(filler, 0x38, &args![aabbs]);
        let count = e.mem.u32(aabbs + 4) as i32;
        if count > e.mem.u32(listener + 8) as i32 {
            let boxes = allocator(e);
            let have = (e.mem.u32(listener + 0xc) & 0x3fff_ffff) as i32;
            if have < count {
                let wanted = grown_capacity(have, count);
                e.call(
                    ARRAY_RESERVE,
                    &args![boxes, listener + 4, wanted, DISPLAY_AABB_SIZE],
                );
            }
            let size = e.mem.u32(listener + 8);
            let first_new = e.mem.u32(listener + 4).wrapping_add(size << 7);
            for index in 0..count.wrapping_sub(size as i32).max(0) as u32 {
                let display = first_new.wrapping_add(index * DISPLAY_AABB_SIZE);
                if display != 0 {
                    e.call(DISPLAY_AABB_CONSTRUCT, &args![display]);
                }
            }
            e.mem.set_u32(listener + 8, count as u32);
        }
        let count = e.mem.u32(aabbs + 4) as i32;
        e.mem.set_u32(pointers, 0);
        e.mem.set_u32(pointers + 4, 0);
        e.mem.set_u32(pointers + 8, 0x8000_0000);
        let arrays = allocator(e);
        let have = (e.mem.u32(pointers + 8) & 0x3fff_ffff) as i32;
        if have < count {
            let wanted = grown_capacity(have, count);
            e.call(ARRAY_RESERVE, &args![arrays, pointers, wanted, 4u32]);
        }
        e.mem.set_u32(pointers + 4, count as u32);
        let mut index = e.mem.u32(aabbs + 4).wrapping_sub(1) as i32;
        while index >= 0 {
            let minimum = e.mem.u32(aabbs).wrapping_add((index as u32) << 5);
            let display = e.mem.u32(listener + 4).wrapping_add((index as u32) << 7);
            e.call(
                DISPLAY_AABB_SET_EXTENTS,
                &args![display, minimum, minimum + 0x10],
            );
            let display = e.mem.u32(listener + 4).wrapping_add((index as u32) << 7);
            let list = e.mem.u32(pointers);
            e.mem
                .set_u32(list.wrapping_add((index as u32).wrapping_mul(4)), display);
            index -= 1;
        }
        let process: u32 = e.global(PROCESS_ID);
        let handler = e.mem.u32(listener - 0x10);
        e.vcall(
            handler,
            0x38,
            &args![pointers, DISPLAY_COLOUR, 0u32, process],
        );
        profile_record(e, PROFILE_END);
        let freeing = allocator(e);
        let flags = e.mem.u32(pointers + 8);
        e.mem.set_u32(pointers + 4, 0);
        if (flags as i32) >= 0 {
            let data = e.mem.u32(pointers);
            e.vcall(
                freeing,
                0x10,
                &args![data, (flags & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        e.mem.set_u32(pointers, 0);
        e.mem.set_u32(pointers + 8, 0x8000_0000);
        let freeing = allocator(e);
        let flags = e.mem.u32(aabbs + 8);
        e.mem.set_u32(aabbs + 4, 0);
        if (flags as i32) >= 0 {
            let data = e.mem.u32(aabbs);
            e.vcall(freeing, 0x10, &args![data, (flags & 0x3fff_ffff) << 5]);
        }
    });
}

// Translated from 00d6ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the viewer (no engine-map name; `ECX` = viewer): stores the four
/// vtables (`+0`, `+8`, `+0x20`, `+0x28`), has every entry of the array at `[viewer + 0x24] +
/// 0x6C` (counted at `+0x70`, read again after each call; none when the word at `+0x24` is
/// 0) remove the listener sub-object (`viewer + 0x28`), destroys the display boxes (array
/// at `+0x2C`, length `+0x30`: slot 0 of each with 0, 0x80 bytes apart), frees the array
/// (slot `0x10`, unless bit 31 of `+0x34` is set), clears it, stores the last vtable at
/// `+0x28` and jumps to the base destructor `00ddcc60`.
pub fn fn_00d6ac40(e: &mut Engine, this: Ptr) {
    let viewer = this.addr();
    let listener = viewer + 0x28;
    e.mem.set_u32(viewer, VTABLE_VIEWER);
    e.mem.set_u32(viewer + 8, VTABLE_PROCESS);
    e.mem.set_u32(viewer + 0x20, VTABLE_CONTEXT_OWNER);
    e.mem.set_u32(listener, VTABLE_LISTENER);
    let world = e.mem.u32(viewer + 0x24);
    if world != 0 && (e.mem.u32(world + 0x70) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(viewer + 0x24);
            let list = e.mem.u32(world + 0x6c);
            let entry = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
            e.call(REMOVE_LISTENER, &args![entry, listener]);
            let world = e.mem.u32(viewer + 0x24);
            index += 1;
            if (index as i32) >= (e.mem.u32(world + 0x70) as i32) {
                break;
            }
        }
    }
    let freeing = allocator(e);
    let size = e.mem.u32(viewer + 0x30) as i32;
    let mut display = e.mem.u32(viewer + 0x2c);
    if size > 0 {
        for _ in 0..size {
            e.vcall(display, 0, &args![0u32]);
            display = display.wrapping_add(DISPLAY_AABB_SIZE);
        }
    }
    let flags = e.mem.u32(viewer + 0x34);
    e.mem.set_u32(viewer + 0x30, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(viewer + 0x2c);
        e.vcall(freeing, 0x10, &args![data, (flags & 0x3fff_ffff) << 7]);
    }
    e.mem.set_u32(viewer + 0x2c, 0);
    e.mem.set_u32(viewer + 0x34, 0x8000_0000);
    e.mem.set_u32(listener, VTABLE_LISTENER_DESTROYED);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00d6ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadphaseViewer::registerViewer` (engine map): registers the process `"Broadphase"`
/// with the process factory at `01269cd0` (creation function `00d6ad00`) and stores its id
/// at `01269028`.
pub fn hkp_broadphase_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATE],
        )
        .u32();
    e.set_global(PROCESS_ID, id);
}

// Translated from 00d6ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the process id stored by `registerViewer` (`01269028`).
pub fn fn_00d6ad60(e: &mut Engine) -> u32 {
    e.global(PROCESS_ID)
}

// Translated from 00d6ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x38 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d6a940` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d6ad46.
pub fn fn_00d6ad00(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x38]).u32();
    e.mem.set_u16(block + 4, 0x38);
    let object = e.call(0x00d6_a940, &args![block, process_contexts]).u32();
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
            0x00d6_a9b0,
            hkp_broadphase_viewer_post_simulation_callback(Ptr, Ptr)
        ),
        entry!(0x00d6_ac40, fn_00d6ac40(Ptr)),
        entry!(0x00d6_ad40, hkp_broadphase_viewer_register_viewer()),
        entry!(0x00d6_ad60, fn_00d6ad60() -> u32),
        entry!(0x00d6_ada0, deleting_destructor_00d6ada0(Ptr, u8) -> Ptr),
        entry!(0x00d6_ad70, fn_00d6ad70(Ptr, u32) -> u32),
        entry!(0x00d6_ad80, fn_00d6ad80(Ptr, u32) -> u32),
        entry!(0x00d6_ad90, fn_00d6ad90(Ptr, u32) -> u32),
        entry!(0x00d6_ad00, fn_00d6ad00(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d6ada0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_ada0, 0x00d6_ac40);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d6ad70() {
        assert_eq!(check(0x00d6_ad70, 0x00d6_ada0, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6ad80() {
        assert_eq!(check(0x00d6_ad80, 0x00d6_ada0, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6ad90() {
        assert_eq!(check(0x00d6_ad90, 0x00d6_ada0, 0x28), 0xdead_beef);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_CHUNK: u32 = 0x0900_0401;
    const SLOT_FREE: u32 = 0x0900_0402;
    const SLOT_FILL: u32 = 0x0900_0403;
    const SLOT_DISPLAY: u32 = 0x0900_0404;
    const SLOT_DESTRUCT: u32 = 0x0900_0405;

    struct Fixture {
        e: Engine,
        log: Log,
        stream: u32,
        allocator: u32,
        listener: u32,
        step: u32,
    }

    /// An engine with the exe's data range, a profiling stream, a memory router whose allocator
    /// hands out chunks of `chunk` bytes (slot `0xC`) and logs frees (slot `0x10`), a step
    /// object whose slot `0x38` reports `count` boxes, a display handler at `listener - 0x10`
    /// and doubles for the array and display box helpers.
    fn fixture(chunk: u32, count: u32) -> Fixture {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let records = e.mem.alloc(0x30);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT, 5u32);
        e.set_global(PROCESS_ID, 0x77u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.register_double(SLOT_CHUNK, move |e, a| {
            e.mem.set_u32(a[1], chunk);
            Ret {
                eax: e.mem.alloc(0x2000),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_FREE, move |_, a| {
            sink.borrow_mut().push((SLOT_FREE, a.to_vec()));
            Ret::default()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0xc, SLOT_CHUNK);
        e.mem.set_u32(table + 0x10, SLOT_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The step object's filler: box `i` is (i, i + 1, i + 2, 0) to (i + 10, i + 11, i + 12, 0).
        e.register_double(SLOT_FILL, move |e, a| {
            let data = e.mem.u32(a[1]);
            for i in 0..count {
                let at = data + 0x20 * i;
                for lane in 0..3 {
                    e.mem.set_f32(at + 4 * lane, (i + lane) as f32);
                    e.mem.set_f32(at + 0x10 + 4 * lane, (i + lane + 10) as f32);
                }
            }
            e.mem.set_u32(a[1] + 4, count);
            Ret::default()
        });
        let fill_table = e.mem.alloc(0x80);
        e.mem.set_u32(fill_table + 0x38, SLOT_FILL);
        let filler = e.mem.alloc(0x10);
        e.mem.set_u32(filler, fill_table);
        let step = e.mem.alloc(0x80);
        e.mem.set_u32(step + 0x58, filler);
        // The display handler logs the pointer list it is given.
        let sink = Rc::clone(&log);
        e.register_double(SLOT_DISPLAY, move |e, a| {
            let size = e.mem.u32(a[1] + 4);
            let list = e.mem.u32(a[1]);
            let mut words = vec![a[2], a[3], a[4], size];
            words.extend((0..size).map(|i| e.mem.u32(list + 4 * i)));
            sink.borrow_mut().push((SLOT_DISPLAY, words));
            Ret::default()
        });
        let handler_table = e.mem.alloc(0x80);
        e.mem.set_u32(handler_table + 0x38, SLOT_DISPLAY);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(handler, handler_table);
        let viewer = e.mem.alloc(0x100);
        let listener = viewer + 0x40;
        e.mem.set_u32(listener - 0x10, handler);
        // `_reserve` gives the array a fresh block of the capacity asked for.
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let block = e.mem.alloc(a[2] * a[3] + 4);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        for address in [DISPLAY_AABB_CONSTRUCT, DISPLAY_AABB_SET_EXTENTS] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                if address == DISPLAY_AABB_SET_EXTENTS {
                    words.extend((0..3).map(|lane| e.mem.u32(a[1] + 4 * lane)));
                    words.extend((0..3).map(|lane| e.mem.u32(a[2] + 4 * lane)));
                }
                sink.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        Fixture {
            e,
            log,
            stream,
            allocator,
            listener,
            step,
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The labels of the profiling records written so far.
    fn labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0x30;
        let cursor = e.mem.u32(stream + 4);
        (first..cursor).step_by(12).map(|a| e.mem.u32(a)).collect()
    }

    #[test]
    fn the_first_step_reserves_constructs_and_shows_every_box() {
        let mut f = fixture(0x8000, 3);
        f.e.mem.set_u32(f.listener + 0xc, 0x8000_0000);
        f.e.call(0x00d6_a9b0, &args![f.listener, f.step]);
        let data = f.e.mem.u32(f.listener + 4);
        assert_eq!(f.e.mem.u32(f.listener + 8), 3);
        let reserves = calls_to(&f.log, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[0], vec![f.allocator, f.listener + 4, 3, 0x80]);
        assert_eq!(reserves[1][0], f.allocator);
        assert_eq!(reserves[1][2..], [3, 4]);
        assert_eq!(
            calls_to(&f.log, DISPLAY_AABB_CONSTRUCT),
            (0..3).map(|i| vec![data + 0x80 * i]).collect::<Vec<_>>()
        );
        // Boxes are given their extents from the last to the first.
        let extents = calls_to(&f.log, DISPLAY_AABB_SET_EXTENTS);
        assert_eq!(extents.len(), 3);
        for (call, index) in extents.iter().zip([2u32, 1, 0]) {
            assert_eq!(call[0], data + 0x80 * index);
            let low: Vec<u32> = (0..3)
                .map(|lane| ((index + lane) as f32).to_bits())
                .collect();
            let high: Vec<u32> = (0..3)
                .map(|lane| ((index + lane + 10) as f32).to_bits())
                .collect();
            assert_eq!(call[3..6], low[..]);
            assert_eq!(call[6..9], high[..]);
            assert_eq!(call[2], call[1] + 0x10);
        }
        let shown = calls_to(&f.log, SLOT_DISPLAY);
        assert_eq!(
            shown,
            vec![vec![
                0xffff_0000,
                0,
                0x77,
                3,
                data,
                data + 0x80,
                data + 0x100
            ]]
        );
        assert_eq!(labels(&f.e, f.stream), vec![0x010d_23c0, 0x010c_41f8]);
    }

    #[test]
    fn both_arrays_are_freed_by_capacity_after_the_boxes_are_shown() {
        let mut f = fixture(0x8000, 3);
        f.e.mem.set_u32(f.listener + 0xc, 0x8000_0000);
        f.e.call(0x00d6_a9b0, &args![f.listener, f.step]);
        let frees = calls_to(&f.log, SLOT_FREE);
        assert_eq!(frees.len(), 2);
        // The pointer list: capacity 3 words of 4 bytes; the aabbs: 0x8000 / 32 entries.
        assert_eq!(frees[0][0], f.allocator);
        assert_eq!(frees[0][2], 12);
        assert_eq!(frees[1][0], f.allocator);
        assert_eq!(frees[1][2], 0x8000);
        let order: Vec<u32> = f.log.borrow().iter().map(|(a, _)| *a).collect();
        let shown = order.iter().position(|a| *a == SLOT_DISPLAY).unwrap();
        let first_free = order.iter().position(|a| *a == SLOT_FREE).unwrap();
        assert!(shown < first_free);
    }

    #[test]
    fn existing_boxes_are_reused_and_not_reconstructed() {
        let mut f = fixture(0x8000, 2);
        let existing = f.e.mem.alloc(0x400);
        f.e.mem.set_u32(f.listener + 4, existing);
        f.e.mem.set_u32(f.listener + 8, 5);
        f.e.mem.set_u32(f.listener + 0xc, 8);
        f.e.call(0x00d6_a9b0, &args![f.listener, f.step]);
        assert_eq!(f.e.mem.u32(f.listener + 8), 5);
        assert!(calls_to(&f.log, DISPLAY_AABB_CONSTRUCT).is_empty());
        let reserves = calls_to(&f.log, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_ne!(reserves[0][1], f.listener + 4);
        let shown = calls_to(&f.log, SLOT_DISPLAY);
        assert_eq!(shown[0][3..], [2, existing, existing + 0x80]);
    }

    #[test]
    fn a_larger_count_doubles_the_capacity_or_takes_the_count() {
        let mut f = fixture(0x8000, 5);
        let existing = f.e.mem.alloc(0x400);
        f.e.mem.set_u32(f.listener + 4, existing);
        f.e.mem.set_u32(f.listener + 8, 2);
        f.e.mem.set_u32(f.listener + 0xc, 4);
        f.e.call(0x00d6_a9b0, &args![f.listener, f.step]);
        let reserves = calls_to(&f.log, ARRAY_RESERVE);
        assert_eq!(reserves[0], vec![f.allocator, f.listener + 4, 8, 0x80]);
        let data = f.e.mem.u32(f.listener + 4);
        assert_eq!(
            calls_to(&f.log, DISPLAY_AABB_CONSTRUCT),
            (2..5).map(|i| vec![data + 0x80 * i]).collect::<Vec<_>>()
        );
        assert_eq!(f.e.mem.u32(f.listener + 8), 5);
    }

    #[test]
    fn an_empty_step_shows_nothing_and_frees_only_the_chunk() {
        let mut f = fixture(0x8000, 0);
        f.e.mem.set_u32(f.listener + 0xc, 0x8000_0000);
        f.e.call(0x00d6_a9b0, &args![f.listener, f.step]);
        assert!(calls_to(&f.log, DISPLAY_AABB_SET_EXTENTS).is_empty());
        assert!(calls_to(&f.log, ARRAY_RESERVE).is_empty());
        assert_eq!(
            calls_to(&f.log, SLOT_DISPLAY),
            vec![vec![0xffff_0000, 0, 0x77, 0]]
        );
        let frees = calls_to(&f.log, SLOT_FREE);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][2], 0x8000);
    }

    #[test]
    fn a_chunk_smaller_than_one_box_is_not_freed() {
        let mut f = fixture(0x10, 0);
        f.e.call(0x00d6_a9b0, &args![f.listener, f.step]);
        assert!(calls_to(&f.log, SLOT_FREE).is_empty());
    }

    #[test]
    fn the_destructor_unregisters_destroys_and_frees() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        for address in [REMOVE_LISTENER, BASE_DESTRUCT, SLOT_DESTRUCT, SLOT_FREE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let free_table = e.mem.alloc(0x40);
        e.mem.set_u32(free_table + 0x10, SLOT_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, free_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let world = e.mem.alloc(0x80);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, 0xa000);
        e.mem.set_u32(list + 4, 0xa001);
        e.mem.set_u32(world + 0x6c, list);
        e.mem.set_u32(world + 0x70, 2);
        let element_table = e.mem.alloc(0x10);
        e.mem.set_u32(element_table, SLOT_DESTRUCT);
        let boxes = e.mem.alloc(0x200);
        for i in 0..3 {
            e.mem.set_u32(boxes + 0x80 * i, element_table);
        }
        let viewer = e.mem.alloc(0x80);
        e.mem.set_u32(viewer + 0x24, world);
        e.mem.set_u32(viewer + 0x2c, boxes);
        e.mem.set_u32(viewer + 0x30, 3);
        e.mem.set_u32(viewer + 0x34, 5);
        e.call(0x00d6_ac40, &args![viewer]);
        assert_eq!(
            *log.borrow(),
            vec![
                (REMOVE_LISTENER, vec![0xa000, viewer + 0x28]),
                (REMOVE_LISTENER, vec![0xa001, viewer + 0x28]),
                (SLOT_DESTRUCT, vec![boxes, 0]),
                (SLOT_DESTRUCT, vec![boxes + 0x80, 0]),
                (SLOT_DESTRUCT, vec![boxes + 0x100, 0]),
                (SLOT_FREE, vec![allocator, boxes, 5 * 0x80]),
                (BASE_DESTRUCT, vec![viewer]),
            ]
        );
        assert_eq!(e.mem.u32(viewer), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(viewer + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(viewer + 0x20), VTABLE_CONTEXT_OWNER);
        assert_eq!(e.mem.u32(viewer + 0x28), VTABLE_LISTENER_DESTROYED);
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
        assert_eq!(e.mem.u32(viewer + 0x30), 0);
        assert_eq!(e.mem.u32(viewer + 0x34), 0x8000_0000);
    }

    #[test]
    fn the_destructor_of_a_viewer_without_world_or_boxes_only_resets_it() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        for address in [REMOVE_LISTENER, BASE_DESTRUCT, SLOT_FREE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, SLOT_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let viewer = e.mem.alloc(0x80);
        e.mem.set_u32(viewer + 0x34, 0x8000_0000);
        e.call(0x00d6_ac40, &args![viewer]);
        assert_eq!(*log.borrow(), vec![(BASE_DESTRUCT, vec![viewer])]);
    }

    #[test]
    fn register_viewer_stores_the_process_id_and_it_can_be_read_back() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            sink.borrow_mut().push((REGISTER_PROCESS, a.to_vec()));
            Ret {
                eax: 0x4321,
                ..Ret::default()
            }
        });
        e.set_global(PROCESS_FACTORY, 0x5555u32);
        e.call(0x00d6_ad40, &args![]);
        assert_eq!(
            *log.borrow(),
            vec![(REGISTER_PROCESS, vec![0x5555, 0x010c_65b0, 0x00d6_ad00])]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x4321);
        assert_eq!(e.call(0x00d6_ad60, &args![]).u32(), 0x4321);
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
    fn fn_00d6ad00_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_a940, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_ad00, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x38]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x38);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d6ad00_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_a940, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_ad00, &args![0x1234u32]).u32(), 0);
    }
}
