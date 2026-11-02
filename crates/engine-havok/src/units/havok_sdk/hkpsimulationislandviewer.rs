//! `Havok SDK/hkpsimulationislandviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_vec, maxps, minps, profile_record, store_vec, zip_with, PROFILE_END,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d65220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_4ae0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d65220(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_4ae0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d651f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d65220`. `00d65220` (`RET 4`, which the Xbox PDB names `hkpSimulationIslandViewer::_vector_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d651f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_5220, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d65200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d65220`. `00d65220` (`RET 4`, which the Xbox PDB names `hkpSimulationIslandViewer::_vector_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d65200(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_5220, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d65210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x28 (`SUB ECX,0x28`) and jumps to `00d65220`. `00d65220` (`RET 4`, which the Xbox PDB names `hkpSimulationIslandViewer::_vector_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d65210(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_5220, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

/// `hkProcessFactory::registerProcess` (`00dd9380`, thiscall on the factory at the global
/// `01269cd0`: process name, creator), its result being the viewer's process tag in the global
/// `01268fdc`, and the name and creator `registerViewer` passes.
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const VIEWER_PROCESS_TAG: u32 = 0x0126_8fdc;
const VIEWER_PROCESS_NAME: u32 = 0x010d_19d0;
const VIEWER_PROCESS_CREATOR: u32 = 0x00d6_5180;
/// The vtables the destructor stores (at `+0`, `+8`, `+0x20` and `+0x28`) and the base class's
/// vtable it ends with (at `+0x28`), and the base destructor body it jumps to (`00ddcc60`).
const VIEWER_VTABLE: u32 = 0x010d_1a28;
const VIEWER_VTABLE_AT_8: u32 = 0x010d_1a08;
const VIEWER_VTABLE_AT_0X20: u32 = 0x010d_19f8;
const VIEWER_VTABLE_AT_0X28: u32 = 0x010d_19e8;
const WORLD_LISTENER_VTABLE: u32 = 0x010c_cf7c;
const VIEWER_BASE_DESTRUCT: u32 = 0x00dd_cc60;
/// `00c918e0` (thiscall on a listener of the world: the viewer's sub-object at `+0x28`).
const WORLD_LISTENER_DETACH: u32 = 0x00c9_18e0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkDisplayAABB`'s constructor (`00de1210`, thiscall) and `setExtents` (`00de1170`, thiscall
/// on the box: minimum, maximum).
const DISPLAY_AABB_CONSTRUCT: u32 = 0x00de_1210;
const DISPLAY_AABB_SET_EXTENTS: u32 = 0x00de_1170;
/// The floats `-3.4028233e38` (`0104e148`) and `3.4028233e38` (`0104da48`) the bounds start
/// from.
const LARGEST_NEGATIVE: u32 = 0x0104_e148;
const LARGEST_POSITIVE: u32 = 0x0104_da48;
/// The profile label `postSimulationCallback` begins with.
const PROFILE_ISLAND_VIEWER: u32 = 0x010d_1a34;

// Translated from 00d64ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (the destructor of the island viewer, `ECX` = `this`): stores the vtables
/// (`010d1a28` at `+0`, `010d1a08` at `+8`, `010d19f8` at `+0x20`, `010d19e8` at `+0x28`), and
/// when the world at `+0x24` is set, calls `00c918e0` with every listener of the world's array
/// at `+0x6c` (count at `+0x70`, re-read each time) on the sub-object at `+0x28`. Then frees two
/// arrays of 0x80-byte display boxes (data `+0x3c`, size `+0x40`, capacity word `+0x44`, and
/// data `+0x30`, size `+0x34`, capacity word `+0x38`): each element's virtual slot 0 is called
/// with `0`, and the block (capacity `<< 7` bytes) is returned to the allocator (slot `0x10` of
/// `[router + 0x10]`) when the capacity word is not negative. Finally stores the vtable
/// `010ccf7c` at `+0x28` and jumps to `00ddcc60`.
pub fn fn_00d64ae0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VIEWER_VTABLE);
    e.mem.set_u32(base + 8, VIEWER_VTABLE_AT_8);
    e.mem.set_u32(base + 0x20, VIEWER_VTABLE_AT_0X20);
    e.mem.set_u32(base + 0x28, VIEWER_VTABLE_AT_0X28);
    if e.mem.u32(base + 0x24) != 0 {
        let mut index = 0u32;
        while e.mem.i32(e.mem.u32(base + 0x24) + 0x70) > index as i32 {
            let world = e.mem.u32(base + 0x24);
            let listener = e.mem.u32(e.mem.u32(world + 0x6c) + 4 * index);
            e.call(WORLD_LISTENER_DETACH, &args![listener, base + 0x28]);
            index += 1;
        }
    }
    free_display_box_array(e, base + 0x3c);
    free_display_box_array(e, base + 0x30);
    e.mem.set_u32(base + 0x28, WORLD_LISTENER_VTABLE);
    e.call(VIEWER_BASE_DESTRUCT, &args![this]);
}

/// The destructor's array release: `array` holds data (`+0`), size (`+4`) and a capacity word
/// (`+8`) of 0x80-byte display boxes.
fn free_display_box_array(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.i32(array + 4);
    let mut element = e.mem.u32(array);
    for _ in 0..size.max(0) {
        e.vcall(element, 0, &args![0u32]);
        element = element.wrapping_add(0x80);
    }
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 7]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00d651c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIslandViewer::registerViewer` (Xbox PDB): registers the creator `00d65180`
/// under the name `"Simulation Islands"` (`010d19d0`) with the process factory at the global
/// `01269cd0` (`00dd9380`, thiscall) and stores the answer in the global `01268fdc`.
pub fn hkp_simulation_island_viewer_register_viewer(e: &mut Engine) {
    let factory = e.global::<u32>(PROCESS_FACTORY);
    let tag = e
        .call(
            REGISTER_PROCESS,
            &args![factory, VIEWER_PROCESS_NAME, VIEWER_PROCESS_CREATOR],
        )
        .u32();
    e.set_global(VIEWER_PROCESS_TAG, tag);
}

// Translated from 00d651e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: answers the viewer's process tag, the global at `01268fdc`.
pub fn fn_00d651e0(e: &mut Engine) -> u32 {
    e.global::<u32>(VIEWER_PROCESS_TAG)
}

/// The inline `reserve` of the exe's `hkArray`: asks the router for the allocator stored at
/// `router + allocator_field` and, when the array's capacity (the capacity word `+8` masked
/// with `0x3fffffff`) is below `wanted`, calls `hkArrayUtil::_reserve` with the larger of
/// `wanted` and twice the capacity.
fn reserve_array(e: &mut Engine, allocator_field: u32, array: u32, wanted: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_field);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// The inline resize that grows the viewer's array of display boxes (data `array`, size `+4`,
/// capacity word `+8`; 0x80-byte `hkDisplayAABB` elements, allocator `[router + 0x10]`) to
/// `wanted` elements, when `wanted` is larger than its size: reserve, destroy the elements
/// `wanted ..` (virtual slot 0 with `0`; unreachable here, because `wanted` is larger than the
/// size, so the count is never positive) and construct the new ones.
fn grow_display_boxes(e: &mut Engine, array: u32, wanted: i32) {
    if wanted <= e.mem.i32(array + 4) {
        return;
    }
    reserve_array(e, 0x10, array, wanted, 0x80);
    let surplus = e.mem.i32(array + 4).wrapping_sub(wanted);
    let mut element = (wanted as u32)
        .wrapping_shl(7)
        .wrapping_add(e.mem.u32(array));
    for _ in 0..surplus.max(0) {
        e.vcall(element, 0, &args![0u32]);
        element = element.wrapping_add(0x80);
    }
    let size = e.mem.u32(array + 4);
    let missing = wanted.wrapping_sub(size as i32);
    let mut element = size.wrapping_shl(7).wrapping_add(e.mem.u32(array));
    for _ in 0..missing.max(0) {
        if element != 0 {
            e.call(DISPLAY_AABB_CONSTRUCT, &args![element]);
        }
        element = element.wrapping_add(0x80);
    }
    e.mem.set_i32(array + 4, wanted);
}

/// Starts an empty local array (`data 0`, `size 0`, capacity word `0x80000000`).
fn init_frame_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Releases a local array the way the exe's inline destructor does: size cleared, and when the
/// capacity word is not negative the block (`capacity << shift` bytes) goes back to the
/// allocator at `router + allocator_field` (slot `0x10`).
fn release_frame_array(e: &mut Engine, allocator_field: u32, array: u32, shift: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_field);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff) << shift],
        );
    }
}

/// One island's bounding box into the display box at `boxes + 0x80 * index`: the minimum and
/// maximum start at the largest floats; for each entity of the island's array (`island + 0x48`,
/// size `+4`) whose word at `+0x10` is set, slot `0x3c` of the object at `world + 0x58` writes
/// the entity's AABB (from `entity + 0x24`, minimum then maximum) into `scratch`, and the lanes
/// are folded with `MAXPS` / `MINPS`. The result goes to `hkDisplayAABB::setExtents`.
fn set_island_box(e: &mut Engine, world: u32, island: u32, display_box: u32, scratch: u32) {
    // Layout of `scratch`: minimum at +0, maximum at +0x10, the entity's AABB at +0x20.
    let largest_positive = e.global::<f32>(LARGEST_POSITIVE);
    let largest_negative = e.global::<f32>(LARGEST_NEGATIVE);
    let mut minimum = [largest_positive; 4];
    let mut maximum = [largest_negative; 4];
    let entities = island + 0x48;
    let mut index = 0u32;
    while e.mem.i32(entities + 4) > index as i32 {
        let entity = e.mem.u32(e.mem.u32(entities) + 4 * index);
        if e.mem.u32(entity + 0x10) != 0 {
            let object = e.mem.u32(world + 0x58);
            e.vcall(object, 0x3c, &args![entity + 0x24, scratch + 0x20]);
            let entity_max = load_vec(e, scratch + 0x30);
            maximum = zip_with(maximum, entity_max, maxps);
            let entity_min = load_vec(e, scratch + 0x20);
            minimum = zip_with(minimum, entity_min, minps);
        }
        index += 1;
    }
    store_vec(e, scratch, minimum);
    store_vec(e, scratch + 0x10, maximum);
    e.call(
        DISPLAY_AABB_SET_EXTENTS,
        &args![display_box, scratch, scratch + 0x10],
    );
}

// Translated from 00d64c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIslandViewer::postSimulationCallback` (Xbox PDB), `(this, world)`, `RET 4`
/// (`this` = the listener sub-object, 0x10 above the process object). Between profile records
/// (`010d1a34` and the end label): for the active islands when the byte at `+4` is set, and then
/// for the inactive islands when the byte at `+5` is set, it grows the viewer's array of display
/// boxes (`+0x14` / `+8`) to the number of islands (`world + 0x2c`, `world + 0x38`; island arrays
/// at `+0x28` / `+0x34`), fits one box around each island (`set_island_box`), collects the box
/// pointers in a local array and hands it to slot `0x38` of the process object at `this - 0x10`
/// with the colour (`0xff0000ff` for the active islands, `0xff008000` for the inactive ones), a
/// zero word and the process tag (`01268fdc`). The active-island pass also fills a second local
/// array of 32-byte entries with the starting bounds, which nothing reads. Local arrays are
/// released through the allocator in `router + 0xc` (active pass) and `router + 0x10`
/// (inactive pass).
pub fn hkp_simulation_island_viewer_post_simulation_callback(
    e: &mut Engine,
    this: Ptr,
    world: u32,
) {
    let base = this.addr();
    profile_record(e, PROFILE_ISLAND_VIEWER);
    // Frame: box pointer array at +0, starting-bounds array at +0x10, bounds scratch at +0x20
    // (0x20 bytes) followed by the entity's AABB at +0x40.
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        if e.mem.u8(base + 4) != 0 {
            let islands = e.mem.i32(world + 0x2c);
            grow_display_boxes(e, base + 0x14, islands);
            let count = e.mem.i32(world + 0x2c);
            init_frame_array(e, frame);
            reserve_array(e, 0xc, frame, count, 4);
            e.mem.set_i32(frame + 4, count);
            let bounds_count = e.mem.i32(world + 0x2c);
            init_frame_array(e, frame + 0x10);
            reserve_array(e, 0xc, frame + 0x10, bounds_count, 0x20);
            e.mem.set_i32(frame + 0x14, bounds_count);
            let largest_positive = e.global::<f32>(LARGEST_POSITIVE);
            let largest_negative = e.global::<f32>(LARGEST_NEGATIVE);
            let mut index = 0u32;
            while e.mem.i32(world + 0x2c) > index as i32 {
                let island = e.mem.u32(e.mem.u32(world + 0x28) + 4 * index);
                let entry = e.mem.u32(frame + 0x10) + 0x20 * index;
                store_vec(e, entry + 0x10, [largest_negative; 4]);
                store_vec(e, entry, [largest_positive; 4]);
                let display_box = e.mem.u32(base + 0x14) + 0x80 * index;
                set_island_box(e, world, island, display_box, frame + 0x20);
                let display_box = e.mem.u32(base + 0x14) + 0x80 * index;
                e.mem.set_u32(e.mem.u32(frame) + 4 * index, display_box);
                index += 1;
            }
            let process = e.mem.u32(base.wrapping_sub(0x10));
            let tag = e.global::<u32>(VIEWER_PROCESS_TAG);
            e.vcall(process, 0x38, &args![frame, 0xff00_00ffu32, 0u32, tag]);
            release_frame_array(e, 0xc, frame + 0x10, 5);
            e.mem.set_u32(frame + 0x10, 0);
            e.mem.set_u32(frame + 0x18, 0x8000_0000);
            release_frame_array(e, 0xc, frame, 2);
        }
        if e.mem.u8(base + 5) != 0 {
            let islands = e.mem.i32(world + 0x38);
            grow_display_boxes(e, base + 8, islands);
            let count = e.mem.i32(world + 0x38);
            init_frame_array(e, frame);
            reserve_array(e, 0x10, frame, count, 4);
            e.mem.set_i32(frame + 4, count);
            let mut index = 0u32;
            while e.mem.i32(world + 0x38) > index as i32 {
                let island = e.mem.u32(e.mem.u32(world + 0x34) + 4 * index);
                let display_box = e.mem.u32(base + 8) + 0x80 * index;
                set_island_box(e, world, island, display_box, frame + 0x20);
                let display_box = e.mem.u32(base + 8) + 0x80 * index;
                e.mem.set_u32(e.mem.u32(frame) + 4 * index, display_box);
                index += 1;
            }
            let process = e.mem.u32(base.wrapping_sub(0x10));
            let tag = e.global::<u32>(VIEWER_PROCESS_TAG);
            e.vcall(process, 0x38, &args![frame, 0xff00_8000u32, 0u32, tag]);
            release_frame_array(e, 0x10, frame, 2);
        }
    });
    profile_record(e, PROFILE_END);
}
// Translated from 00d65180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x48 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d64a50` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d651c6.
pub fn fn_00d65180(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x48]).u32();
    e.mem.set_u16(block + 4, 0x48);
    let object = e.call(0x00d6_4a50, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_4ae0, fn_00d64ae0(Ptr)),
        entry!(
            0x00d6_4c00,
            hkp_simulation_island_viewer_post_simulation_callback(Ptr, u32)
        ),
        entry!(0x00d6_51c0, hkp_simulation_island_viewer_register_viewer()),
        entry!(0x00d6_51e0, fn_00d651e0() -> u32),
        entry!(0x00d6_5220, deleting_destructor_00d65220(Ptr, u8) -> Ptr),
        entry!(0x00d6_51f0, fn_00d651f0(Ptr, u32) -> u32),
        entry!(0x00d6_5200, fn_00d65200(Ptr, u32) -> u32),
        entry!(0x00d6_5210, fn_00d65210(Ptr, u32) -> u32),
        entry!(0x00d6_5180, fn_00d65180(u32) -> u32),
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
    fn deleting_destructor_00d65220_frees_only_when_bit_zero_is_set() {
        check(0x00d6_5220, 0x00d6_4ae0);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d651f0() {
        assert_eq!(
            check(0x00d6_51f0, 0x00d6_5220, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d65200() {
        assert_eq!(
            check(0x00d6_5200, 0x00d6_5220, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d65210() {
        assert_eq!(
            check(0x00d6_5210, 0x00d6_5220, 0x28, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The double that stands for the allocator's free slot (`0x10`).
    const FREE_DOUBLE: u32 = 0x0900_4010;
    /// The doubles for the viewer process object's slot `0x38`, the world's slot `0x3c`
    /// (entity AABB) and an element's destructor.
    const DRAW_DOUBLE: u32 = 0x0900_4038;
    const ENTITY_AABB_DOUBLE: u32 = 0x0900_403c;
    const ELEMENT_DESTRUCTOR_DOUBLE: u32 = 0x0900_4000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addr: u32) {
        e.register_double(addr, |_, _| Ret::default());
    }

    /// A router whose allocators at `+0xc` and `+0x10` are distinct objects with the free
    /// double in slot `0x10`; returns both allocators.
    fn router(e: &mut Engine) -> (u32, u32) {
        let table = 0x0900_4100;
        e.put_vtable(table, &[0, 0, 0, 0, FREE_DOUBLE]);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first, table);
        e.mem.set_u32(second, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, first);
        e.mem.set_u32(router + 0x10, second);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        noop(e, FREE_DOUBLE);
        (first, second)
    }

    #[test]
    fn register_viewer_stores_the_process_tag() {
        let mut e = engine();
        e.set_global(PROCESS_FACTORY, 0x1234u32);
        e.register_double(REGISTER_PROCESS, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.call(0x00d651c0, &args![]);
        assert_eq!(
            calls(&e, REGISTER_PROCESS),
            vec![vec![0x1234, 0x010d_19d0, 0x00d6_5180]]
        );
        assert_eq!(e.global::<u32>(VIEWER_PROCESS_TAG), 0x77);
        assert_eq!(e.call(0x00d651e0, &args![]).u32(), 0x77);
    }

    #[test]
    fn destructor_detaches_listeners_and_frees_the_box_arrays() {
        let mut e = engine();
        let (_, allocator) = router(&mut e);
        e.put_vtable(0x0900_4200, &[ELEMENT_DESTRUCTOR_DOUBLE]);
        noop(&mut e, ELEMENT_DESTRUCTOR_DOUBLE);
        noop(&mut e, WORLD_LISTENER_DETACH);
        noop(&mut e, VIEWER_BASE_DESTRUCT);
        let world = e.mem.alloc(0x80);
        let listeners = e.mem.alloc(0x10);
        e.mem.set_u32(listeners, 0xa1);
        e.mem.set_u32(listeners + 4, 0xa2);
        e.mem.set_u32(world + 0x6c, listeners);
        e.mem.set_u32(world + 0x70, 2);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x24, world);
        // Two boxes in the array at +0x3c (capacity word 3), none in the one at +0x30 (a
        // negative capacity word: not owned, so not freed).
        let boxes = e.mem.alloc(0x180);
        for index in 0..2 {
            e.mem.set_u32(boxes + 0x80 * index, 0x0900_4200);
        }
        e.mem.set_u32(this + 0x3c, boxes);
        e.mem.set_u32(this + 0x40, 2);
        e.mem.set_u32(this + 0x44, 3);
        e.mem.set_u32(this + 0x38, 0x8000_0000);
        e.call(0x00d64ae0, &args![this]);
        assert_eq!(
            calls(&e, WORLD_LISTENER_DETACH),
            vec![vec![0xa1, this + 0x28], vec![0xa2, this + 0x28]]
        );
        assert_eq!(
            calls(&e, ELEMENT_DESTRUCTOR_DOUBLE),
            vec![vec![boxes, 0], vec![boxes + 0x80, 0]]
        );
        assert_eq!(calls(&e, FREE_DOUBLE), vec![vec![allocator, boxes, 3 << 7]]);
        assert_eq!(calls(&e, VIEWER_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x010d_1a28);
        assert_eq!(e.mem.u32(this + 8), 0x010d_1a08);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_19f8);
        assert_eq!(e.mem.u32(this + 0x28), 0x010c_cf7c);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn destructor_without_a_world_skips_the_detach() {
        let mut e = engine();
        router(&mut e);
        noop(&mut e, VIEWER_BASE_DESTRUCT);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x44, 0x8000_0000);
        e.mem.set_u32(this + 0x38, 0x8000_0000);
        e.call(0x00d64ae0, &args![this]);
        assert!(calls(&e, WORLD_LISTENER_DETACH).is_empty());
        assert_eq!(calls(&e, VIEWER_BASE_DESTRUCT), vec![vec![this]]);
    }

    /// An entity whose word at `+0x10` is `active` and whose `+0x24` holds the float `x`.
    fn entity(e: &mut Engine, active: bool, x: f32) -> u32 {
        let entity = e.mem.alloc(0x40);
        e.mem.set_u32(entity + 0x10, u32::from(active));
        e.mem.set_f32(entity + 0x24, x);
        entity
    }

    fn island(e: &mut Engine, entities: &[u32]) -> u32 {
        let island = e.mem.alloc(0x60);
        let data = e.mem.alloc(0x10);
        for (index, entity) in entities.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entity);
        }
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_u32(island + 0x4c, entities.len() as u32);
        island
    }

    fn island_array(e: &mut Engine, islands: &[u32]) -> u32 {
        let data = e.mem.alloc(0x10);
        for (index, island) in islands.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *island);
        }
        data
    }

    #[test]
    fn post_simulation_callback_draws_a_box_per_island() {
        let mut e = engine();
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.set_global(LARGEST_NEGATIVE, (-3.0e38f32).to_bits());
        e.set_global(LARGEST_POSITIVE, 3.0e38f32.to_bits());
        e.set_global(VIEWER_PROCESS_TAG, 0x55u32);
        let (wide_allocator, narrow_allocator) = router(&mut e);
        // The reserve double hands out a zeroed block and records the capacity.
        e.register_double(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        noop(&mut e, DISPLAY_AABB_CONSTRUCT);
        let extents: EventLog = Rc::new(RefCell::new(Vec::new()));
        let extents_log = Rc::clone(&extents);
        e.register_double(DISPLAY_AABB_SET_EXTENTS, move |e, a| {
            let mut entry = vec![a[0]];
            entry.extend((0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)));
            entry.extend((0..4).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            extents_log.borrow_mut().push(entry);
            Ret::default()
        });
        // The entity AABB: minimum x in every lane, maximum x + 2.
        e.register_double(ENTITY_AABB_DOUBLE, |e, a| {
            let x = e.mem.f32(a[1]);
            for lane in 0..4 {
                e.mem.set_f32(a[2] + 4 * lane, x);
                e.mem.set_f32(a[2] + 0x10 + 4 * lane, x + 2.0);
            }
            Ret::default()
        });
        let bounds_object = e.mem.alloc(0x10);
        e.put_vtable(0x0900_4300, &[0; 16]);
        e.mem.set_u32(0x0900_4300 + 0x3c, ENTITY_AABB_DOUBLE);
        e.mem.set_u32(bounds_object, 0x0900_4300);
        let drawn: EventLog = Rc::new(RefCell::new(Vec::new()));
        let drawn_log = Rc::clone(&drawn);
        e.register_double(DRAW_DOUBLE, move |e, a| {
            let mut entry = vec![a[0], a[2], a[3], a[4]];
            let (data, size) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            entry.extend((0..size).map(|index| e.mem.u32(data + 4 * index)));
            drawn_log.borrow_mut().push(entry);
            Ret::default()
        });
        let process = e.mem.alloc(0x10);
        e.put_vtable(0x0900_4400, &[0; 16]);
        e.mem.set_u32(0x0900_4400 + 0x38, DRAW_DOUBLE);
        e.mem.set_u32(process, 0x0900_4400);
        let block = e.mem.alloc(0x40);
        let this = block + 0x20;
        e.mem.set_u32(this - 0x10, process);
        e.mem.set_u8(this + 4, 1);
        e.mem.set_u8(this + 5, 1);
        let world = e.mem.alloc(0x80);
        e.mem.set_u32(world + 0x58, bounds_object);
        let low = entity(&mut e, true, 1.0);
        let high = entity(&mut e, true, 5.0);
        let first = [low, high];
        let ignored = entity(&mut e, false, 100.0);
        let negative = entity(&mut e, true, -2.0);
        let active = [
            island(&mut e, &[first[0], ignored, first[1]]),
            island(&mut e, &[]),
        ];
        let inactive = [island(&mut e, &[negative])];
        let active_data = island_array(&mut e, &active);
        e.mem.set_u32(world + 0x28, active_data);
        e.mem.set_u32(world + 0x2c, 2);
        let inactive_data = island_array(&mut e, &inactive);
        e.mem.set_u32(world + 0x34, inactive_data);
        e.mem.set_u32(world + 0x38, 1);
        e.call(0x00d64c00, &args![this, world]);
        // Boxes: two for the active islands (array at +0x14), one for the inactive (at +8).
        let active_boxes = e.mem.u32(this + 0x14);
        let inactive_boxes = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(
            calls(&e, DISPLAY_AABB_CONSTRUCT),
            vec![
                vec![active_boxes],
                vec![active_boxes + 0x80],
                vec![inactive_boxes]
            ]
        );
        let lanes = |value: f32| [value.to_bits(); 4];
        let entry = |display_box: u32, minimum: f32, maximum: f32| {
            let mut entry = vec![display_box];
            entry.extend(lanes(minimum));
            entry.extend(lanes(maximum));
            entry
        };
        assert_eq!(
            *extents.borrow(),
            vec![
                entry(active_boxes, 1.0, 7.0),
                entry(active_boxes + 0x80, 3.0e38, -3.0e38),
                entry(inactive_boxes, -2.0, 0.0),
            ]
        );
        assert_eq!(
            *drawn.borrow(),
            vec![
                vec![
                    process,
                    0xff00_00ff,
                    0,
                    0x55,
                    active_boxes,
                    active_boxes + 0x80
                ],
                vec![process, 0xff00_8000, 0, 0x55, inactive_boxes],
            ]
        );
        // The array growth and both local arrays reserve through the right allocators.
        let reserves: Vec<(u32, u32, u32)> = calls(&e, ARRAY_RESERVE)
            .iter()
            .map(|a| (a[0], a[2], a[3]))
            .collect();
        assert_eq!(
            reserves,
            vec![
                (narrow_allocator, 2, 0x80),
                (wide_allocator, 2, 4),
                (wide_allocator, 2, 0x20),
                (narrow_allocator, 1, 0x80),
                (narrow_allocator, 1, 4),
            ]
        );
        // The local arrays go back: bounds (2 << 5) and pointers (2 << 2) through the first
        // allocator, then the inactive pointers (1 << 2) through the second.
        let frees: Vec<(u32, u32)> = calls(&e, FREE_DOUBLE)
            .iter()
            .map(|a| (a[0], a[2]))
            .collect();
        assert_eq!(
            frees,
            vec![
                (wide_allocator, 0x40),
                (wide_allocator, 8),
                (narrow_allocator, 4)
            ]
        );
    }

    #[test]
    fn post_simulation_callback_does_nothing_when_both_flags_are_clear() {
        let mut e = engine();
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40) + 0x20;
        e.call(0x00d64c00, &args![this, 0u32]);
        // Only the two profile records were written (begin and end).
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_ISLAND_VIEWER);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
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
    fn fn_00d65180_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_4a50, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_5180, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x48]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x48);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d65180_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_4a50, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_5180, &args![0x1234u32]).u32(), 0);
    }
}
