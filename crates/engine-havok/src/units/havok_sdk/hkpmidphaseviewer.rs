//! `Havok SDK/hkpmidphaseviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d6a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_a750`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d6a8b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_a750, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d6a880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d6a8b0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a880(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a8b0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d6a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d6a8b0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a890(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a8b0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d6a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d6a8b0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a8a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a8b0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

/// The list of 128-byte display objects, the elements of the array at `this + 4`.
const DISPLAY_SIZE: u32 = 0x80;
/// `hkpMidphaseViewer`'s process id (`01269024`), set by `registerViewer`.
const PROCESS_ID: u32 = 0x0126_9024;
/// The process factory (`01269cd0`) whose registration function (`00dd9380`, thiscall, `RET 8`,
/// `(name, creator)`) records a viewer by name; and the creator `00d6a810` of the viewer.
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const VIEWER_NAME: u32 = 0x010d_22e0;
const VIEWER_CREATOR: u32 = 0x00d6_a810;
/// Profile labels: the start (`TthkpMidphaseViewer`) and the end of the callback.
const LABEL_START: u32 = 0x010d_233c;
const LABEL_END: u32 = 0x010c_41f8;
/// `float` `0.0` (`01011d78`), the fourth lane of the box corners.
const ZERO: u32 = 0x0101_1d78;
/// `float` `4294967296.0` (`01099378`), added to a negative integer to read it as unsigned.
const UNSIGNED_BIAS: u32 = 0x0109_9378;
/// Four-lane constant (`010c7170`, all lanes 1.0) whose last lane completes the scale vector.
const ONES: u32 = 0x010c_7170;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkDisplayAABB` constructor (`00de1210`, thiscall, no arguments) and `setExtents`
/// (`00de1170`, thiscall, `RET 8`, `(minimum, maximum)`).
const DISPLAY_AABB_CONSTRUCTOR: u32 = 0x00de_1210;
const DISPLAY_AABB_SET_EXTENTS: u32 = 0x00de_1170;
/// The remover (`00c918e0`, thiscall, `RET 4`) that takes the viewer's listener out of a
/// world list.
const REMOVE_LISTENER: u32 = 0x00c9_18e0;
/// The base viewer's destructor (`00ddcc60`, thiscall, no arguments).
const BASE_VIEWER_DESTRUCTOR: u32 = 0x00dd_cc60;
/// Vtables the destructor writes back (`0x10d2330`, `0x10d2310`, `0x10d2300`, `0x10d22f0` and
/// finally `0x10ccf7c` for the listener part at `+0x28`).
const VTABLE_VIEWER: u32 = 0x010d_2330;
const VTABLE_VIEWER_B: u32 = 0x010d_2310;
const VTABLE_VIEWER_C: u32 = 0x010d_2300;
const VTABLE_LISTENER: u32 = 0x010d_22f0;
const VTABLE_LISTENER_BASE: u32 = 0x010c_cf7c;

type Lanes = [f32; 4];

fn load_lanes(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, value);
    }
}

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

/// `FILD` of a 32-bit integer stored to a float, with `4294967296.0` added first when it is
/// negative (the compiler's unsigned-to-float fix-up).
fn unsigned_to_float(e: &Engine, value: u32) -> f32 {
    let mut wide = f64::from(value as i32);
    if (value as i32) < 0 {
        wide += f64::from(e.global::<f32>(UNSIGNED_BIAS));
    }
    wide as f32
}

// Translated from 00d6a2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMidphaseViewer::postSimulationCallback` (Xbox PDB), `RET 4` with `this` the listener
/// part at `+0x28` of the viewer (the display list is at `this + 4`, the display handler at
/// `this - 0x10`) and one stack word, the world (name unconfirmed; the lists at `+0x28` and
/// `+0x34` hold entities, `+0x78` the transform data). Rebuilds the display list: destroys
/// the old 128-byte display objects, and then for both lists, for each entity (children at
/// `+0x48`), for each child with a compressed-node array at `+0x54` and a 16-bit count at
/// `+0x50`, for each 32-byte node, builds the box corners from the node's quantized
/// coordinates (an integer minus or plus a byte shifted by the shift byte at `+0xF`, read as
/// an unsigned float), scales them by the reciprocal of the scale vector (`RCPPS` and one
/// Newton step), subtracts the offset vector and appends a `hkDisplayAABB` with those
/// extents. Then hands the list of pointers to the display handler's slot `0x38` with
/// `(list, 0xFFFFA500, 0, process id)`. Profile records `TthkpMidphaseViewer` at the start and
/// another label at the end.
pub fn hkp_midphase_viewer_post_simulation_callback(e: &mut Engine, this: Ptr, world: Ptr) {
    let me = this.addr();
    let w = world.addr();
    profile_record(e, LABEL_START);
    let old_count = e.mem.u32(me + 8) as i32;
    let mut old = e.mem.u32(me + 4);
    if old_count > 0 {
        for _ in 0..old_count {
            e.vcall(old, 0, &args![0u32]);
            old = old.wrapping_add(DISPLAY_SIZE);
        }
    }
    e.mem.set_u32(me + 8, 0);
    e.with_stack(0x40, |e, frame| {
        let low = frame.addr();
        let high = frame.addr() + 0x10;
        let pointers = frame.addr() + 0x20;
        for list_offset in [0x28u32, 0x34] {
            let list = w + list_offset;
            let mut entity_index = 0i32;
            while entity_index < e.mem.u32(list + 4) as i32 {
                let entity = e.mem.u32(e.mem.u32(list) + 4 * entity_index as u32);
                let children = entity + 0x48;
                let mut child_index = 0i32;
                while child_index < e.mem.u32(children + 4) as i32 {
                    let child = e.mem.u32(e.mem.u32(children) + 4 * child_index as u32);
                    let mut node = e.mem.u32(child + 0x54);
                    if node != 0 {
                        let mut index = 0u32;
                        while index < u32::from(e.mem.u16(child + 0x50)) {
                            append_node_box(e, me, w, node, low, high);
                            node = node.wrapping_add(0x20);
                            index += 1;
                        }
                    }
                    child_index += 1;
                }
                entity_index += 1;
            }
        }
        let count = e.mem.u32(me + 8) as i32;
        e.mem.set_u32(pointers, 0);
        e.mem.set_u32(pointers + 4, 0);
        e.mem.set_u32(pointers + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        if count == 0 {
            e.mem.set_u32(pointers, 0);
            e.mem.set_u32(pointers + 4, count as u32);
            e.mem.set_u32(pointers + 8, 0x8000_0000);
        } else {
            // The allocator rounds the request and reports the bytes it granted.
            let request = frame.addr() + 0x30;
            e.mem.set_u32(request, (count as u32).wrapping_mul(4));
            let data = e.vcall(allocator, 0xc, &args![request]).u32();
            let granted = e.mem.u32(request) as i32;
            let words = granted.wrapping_add((granted >> 31) & 3) >> 2;
            let capacity = if words == 0 {
                0x8000_0000
            } else {
                words as u32
            };
            e.mem.set_u32(pointers, data);
            e.mem.set_u32(pointers + 4, words as u32);
            e.mem.set_u32(pointers + 8, capacity);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(pointers + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_mul(2);
            let wanted = if count < doubled { doubled } else { count };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, pointers, wanted as u32, 4u32],
            );
        }
        e.mem.set_u32(pointers + 4, count as u32);
        if count > 0 {
            let mut offset = 0u32;
            for index in 0..count as u32 {
                let data = e.mem.u32(pointers);
                let display = e.mem.u32(me + 4).wrapping_add(offset);
                e.mem.set_u32(data.wrapping_add(4 * index), display);
                offset = offset.wrapping_add(DISPLAY_SIZE);
            }
        }
        let handler = e.mem.u32(me.wrapping_sub(0x10));
        let process: u32 = e.global(PROCESS_ID);
        e.vcall(
            handler,
            0x38,
            &args![pointers, 0xffff_a500u32, 0u32, process],
        );
        profile_record(e, LABEL_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(pointers + 8);
        e.mem.set_u32(pointers + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(pointers);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
    });
}

/// The body of the innermost loop of the callback: the box of the compressed node at `node`
/// is written to `low` and `high` and appended as a new display object to the list at
/// `me + 4` (size `me + 8`, capacity word `me + 0xC`).
fn append_node_box(e: &mut Engine, me: u32, world: u32, node: u32, low: u32, high: u32) {
    let transform = e.mem.u32(world + 0x78);
    let shift = u32::from(e.mem.u8(node + 0xf)) & 31;
    let byte = |e: &Engine, offset: u32| u32::from(e.mem.u8(node + offset)) << shift;
    let x0 = e.mem.u32(node).wrapping_sub(byte(e, 0xc));
    let y0 = e.mem.u32(node + 4).wrapping_sub(byte(e, 0xd));
    let z0 = e.mem.u32(node + 8).wrapping_sub(byte(e, 0xe));
    let x1 = byte(e, 0x1c).wrapping_add(e.mem.u32(node + 0x10));
    let y1 = byte(e, 0x1d).wrapping_add(e.mem.u32(node + 0x14));
    let z1 = byte(e, 0x1e).wrapping_add(e.mem.u32(node + 0x18));
    let zero = e.global::<f32>(ZERO);
    let corner_low = [
        unsigned_to_float(e, x0),
        unsigned_to_float(e, y0),
        unsigned_to_float(e, z0),
        zero,
    ];
    let corner_high = [
        unsigned_to_float(e, x1),
        unsigned_to_float(e, y1),
        unsigned_to_float(e, z1),
        zero,
    ];
    // The scale vector with its last lane replaced by the constant's.
    let scale = load_lanes(e, transform + 0x40);
    let ones = load_lanes(e, ONES);
    let divisor = [scale[0], scale[1], scale[2], ones[3]];
    let offset = load_lanes(e, transform + 0x20);
    let estimate = divisor.map(rcp_estimate);
    // `(r + r) - ((r * d) * r)` per lane.
    let reciprocal: Lanes = std::array::from_fn(|lane| {
        (estimate[lane] + estimate[lane]) - (estimate[lane] * divisor[lane]) * estimate[lane]
    });
    let extent_low: Lanes =
        std::array::from_fn(|lane| corner_low[lane] * reciprocal[lane] - offset[lane]);
    let extent_high: Lanes =
        std::array::from_fn(|lane| corner_high[lane] * reciprocal[lane] - offset[lane]);
    store_lanes(e, low, extent_low);
    store_lanes(e, high, extent_high);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.u32(me + 8) as i32;
    let capacity = (e.mem.u32(me + 0xc) & 0x3fff_ffff) as i32;
    let wanted = size.wrapping_add(1);
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let count = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, me + 4, count as u32, DISPLAY_SIZE],
        );
    }
    let slot = e
        .mem
        .u32(me + 8)
        .wrapping_shl(7)
        .wrapping_add(e.mem.u32(me + 4));
    if slot != 0 {
        e.call(DISPLAY_AABB_CONSTRUCTOR, &args![slot]);
    }
    let grown = e.mem.u32(me + 8).wrapping_add(1);
    e.mem.set_u32(me + 8, grown);
    let display = (size as u32)
        .wrapping_shl(7)
        .wrapping_add(e.mem.u32(me + 4));
    e.call(DISPLAY_AABB_SET_EXTENTS, &args![display, low, high]);
}

// Translated from 00d6a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's destructor (name unconfirmed; the engine map has no name for it): writes the
/// vtables back, tells every list of the world at `+0x24` (count at `+0x70`, array at `+0x6C`)
/// to drop the listener at `this + 0x28` through `00c918e0`, destroys the display objects of
/// the list at `this + 0x2C` (slot 0 with 0, 128 bytes apart), frees its storage unless it
/// is not owned, resets it, and continues with the base destructor `00ddcc60`.
pub fn fn_00d6a750(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let listener = me + 0x28;
    e.mem.set_u32(me, VTABLE_VIEWER);
    e.mem.set_u32(me + 8, VTABLE_VIEWER_B);
    e.mem.set_u32(me + 0x20, VTABLE_VIEWER_C);
    e.mem.set_u32(listener, VTABLE_LISTENER);
    let world = e.mem.u32(me + 0x24);
    if world != 0 && (e.mem.u32(world + 0x70) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let lists = e.mem.u32(e.mem.u32(me + 0x24) + 0x6c);
            let list = e.mem.u32(lists + 4 * index);
            e.call(REMOVE_LISTENER, &args![list, listener]);
            index += 1;
            // The loop test reads the world pointer again.
            let again = e.mem.u32(me + 0x24);
            if (index as i32) >= e.mem.u32(again + 0x70) as i32 {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.u32(me + 0x30) as i32;
    let mut display = e.mem.u32(me + 0x2c);
    if count > 0 {
        for _ in 0..count {
            e.vcall(display, 0, &args![0u32]);
            display = display.wrapping_add(DISPLAY_SIZE);
        }
    }
    e.mem.set_u32(me + 0x30, 0);
    let capacity = e.mem.u32(me + 0x34);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x2c);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_shl(7)],
        );
    }
    e.mem.set_u32(me + 0x2c, 0);
    e.mem.set_u32(me + 0x34, 0x8000_0000);
    e.mem.set_u32(listener, VTABLE_LISTENER_BASE);
    e.call(BASE_VIEWER_DESTRUCTOR, &args![this]);
}

// Translated from 00d6a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMidphaseViewer::registerViewer` (Xbox PDB): registers the viewer under the name
/// `"Midphase"` with the process factory (`00dd9380(name, creator 00d6a810)`) and keeps the
/// returned process id at `01269024`.
pub fn hkp_midphase_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, VIEWER_NAME, VIEWER_CREATOR],
        )
        .u32();
    e.set_global(PROCESS_ID, id);
}

// Translated from 00d6a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's process id (`01269024`, set by `registerViewer`; the engine map has no name
/// for this function).
pub fn fn_00d6a870(e: &mut Engine) -> u32 {
    e.global(PROCESS_ID)
}

// Translated from 00d6a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x38 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d6a280` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d6a856.
pub fn fn_00d6a810(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x38]).u32();
    e.mem.set_u16(block + 4, 0x38);
    let object = e.call(0x00d6_a280, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_a8b0, deleting_destructor_00d6a8b0(Ptr, u8) -> Ptr),
        entry!(0x00d6_a880, fn_00d6a880(Ptr, u32) -> u32),
        entry!(0x00d6_a890, fn_00d6a890(Ptr, u32) -> u32),
        entry!(0x00d6_a8a0, fn_00d6a8a0(Ptr, u32) -> u32),
        entry!(
            0x00d6_a2f0,
            hkp_midphase_viewer_post_simulation_callback(Ptr, Ptr)
        ),
        entry!(0x00d6_a750, fn_00d6a750(Ptr)),
        entry!(0x00d6_a850, hkp_midphase_viewer_register_viewer()),
        entry!(0x00d6_a870, fn_00d6a870() -> u32),
        entry!(0x00d6_a810, fn_00d6a810(u32) -> u32),
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
    fn deleting_destructor_00d6a8b0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_a8b0, 0x00d6_a750);
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
    fn thunk_00d6a880() {
        assert_eq!(check(0x00d6_a880, 0x00d6_a8b0, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6a890() {
        assert_eq!(check(0x00d6_a890, 0x00d6_a8b0, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6a8a0() {
        assert_eq!(check(0x00d6_a8a0, 0x00d6_a8b0, 0x28), 0xdead_beef);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const DESTROY_SLOT: u32 = 0x0900_0101;
    const HANDLER_SLOT: u32 = 0x0900_0102;
    const ALLOCATE_SLOT: u32 = 0x0900_0103;
    const FREE_SLOT: u32 = 0x0900_0104;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn push(log: &Log, row: Vec<u32>) {
        log.borrow_mut().push(row);
    }

    /// The memory router, whose allocator allocates through a size pointer (slot `0xC`,
    /// granting 8 bytes more than asked) and frees (slot `0x10`); the calls are logged with
    /// the tags 90 and 91.
    fn install_allocator(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            let asked = e.mem.u32(a[1]);
            push(&sink, vec![90, asked]);
            e.mem.set_u32(a[1], asked + 8);
            Ret {
                eax: e.mem.alloc(asked + 8),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(log);
        e.register_double(FREE_SLOT, move |_, a| {
            push(&sink, vec![91, a[1], a[2]]);
            Ret::default()
        });
        allocator
    }

    fn log_double(e: &mut Engine, addr: u32, log: &Log, tag: u32) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            let mut row = vec![tag];
            row.extend_from_slice(a);
            push(&sink, row);
            Ret::default()
        });
    }

    /// An object whose first word points at a vtable with `slot` set to `target`.
    fn object_with_slot(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn register_viewer_keeps_the_process_id_the_factory_returns() {
        let mut e = engine();
        e.set_global(PROCESS_FACTORY, 0x1234u32);
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            push(&sink, a.to_vec());
            Ret {
                eax: 77,
                ..Ret::default()
            }
        });
        e.call(0x00d6a850, &args![]);
        assert_eq!(
            *log.borrow(),
            vec![vec![0x1234, VIEWER_NAME, VIEWER_CREATOR]]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 77);
        assert_eq!(e.call(0x00d6a870, &args![]).u32(), 77);
    }

    #[test]
    fn the_destructor_drops_the_listener_and_the_displays_and_resets_the_list() {
        let mut e = engine();
        let log: Log = Rc::default();
        install_allocator(&mut e, &log);
        log_double(&mut e, REMOVE_LISTENER, &log, 1);
        log_double(&mut e, BASE_VIEWER_DESTRUCTOR, &log, 2);
        log_double(&mut e, DESTROY_SLOT, &log, 3);
        let this = e.mem.alloc(0x80);
        let world = e.mem.alloc(0x80);
        let lists = e.mem.alloc(8);
        e.mem.set_u32(lists, 0x111);
        e.mem.set_u32(lists + 4, 0x222);
        e.mem.set_u32(world + 0x6c, lists);
        e.mem.set_u32(world + 0x70, 2);
        e.mem.set_u32(this + 0x24, world);
        let data = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, DESTROY_SLOT);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(data + 0x80, vtable);
        e.mem.set_u32(this + 0x2c, data);
        e.mem.set_u32(this + 0x30, 2);
        e.mem.set_u32(this + 0x34, 2);
        e.call(0x00d6a750, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![1, 0x111, this + 0x28],
                vec![1, 0x222, this + 0x28],
                vec![3, data, 0],
                vec![3, data + 0x80, 0],
                vec![91, data, 0x100],
                vec![2, this],
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_VIEWER_B);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_VIEWER_C);
        assert_eq!(e.mem.u32(this + 0x28), VTABLE_LISTENER_BASE);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0x8000_0000);
    }

    #[test]
    fn the_destructor_without_a_world_or_owned_storage_only_resets_and_continues() {
        let mut e = engine();
        let log: Log = Rc::default();
        install_allocator(&mut e, &log);
        log_double(&mut e, REMOVE_LISTENER, &log, 1);
        log_double(&mut e, BASE_VIEWER_DESTRUCTOR, &log, 2);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x2c, 0x5555);
        e.mem.set_u32(this + 0x34, 0x8000_0003);
        e.call(0x00d6a750, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![2, this]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0x8000_0000);
    }

    struct Callback {
        e: Engine,
        log: Log,
        this: u32,
        world: u32,
        stream: u32,
    }

    /// Node of the compressed array: a corner as integers, the byte offsets, the shift.
    fn put_node(
        e: &mut Engine,
        node: u32,
        low: [u32; 3],
        low_bytes: [u8; 3],
        shift: u8,
        high: [u32; 3],
        high_bytes: [u8; 3],
    ) {
        for i in 0..3u32 {
            e.mem.set_u32(node + 4 * i, low[i as usize]);
            e.mem.set_u8(node + 0xc + i, low_bytes[i as usize]);
            e.mem.set_u32(node + 0x10 + 4 * i, high[i as usize]);
            e.mem.set_u8(node + 0x1c + i, high_bytes[i as usize]);
        }
        e.mem.set_u8(node + 0xf, shift);
    }

    /// A viewer with one old display object, a world with one entity in its first list
    /// (two children: one with two nodes and one with a null node array) and one entity in
    /// the second list (a child with a node array whose count is 0).
    fn callback_rig(with_nodes: bool) -> Callback {
        let mut e = engine();
        let log: Log = Rc::default();
        install_allocator(&mut e, &log);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x80);
        e.mem.set_u32(PROFILE_SLOT, 2);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.set_global(ZERO, 0.0f32);
        e.set_global(UNSIGNED_BIAS, 4294967296.0f32);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
        }
        e.set_global(PROCESS_ID, 0x42u32);
        // Doubles for the display objects and the handler.
        e.register_double(ARRAY_RESERVE, {
            let sink = Rc::clone(&log);
            move |e, a| {
                push(&sink, vec![50, a[0], a[1], a[2], a[3]]);
                let fresh = e.mem.alloc(a[2] * a[3]);
                let used = e.mem.u32(a[1] + 4) * a[3];
                for byte in 0..used {
                    let value = e.mem.u8(e.mem.u32(a[1]) + byte);
                    e.mem.set_u8(fresh + byte, value);
                }
                e.mem.set_u32(a[1], fresh);
                e.mem.set_u32(a[1] + 8, a[2]);
                Ret::default()
            }
        });
        log_double(&mut e, DISPLAY_AABB_CONSTRUCTOR, &log, 60);
        e.register_double(DISPLAY_AABB_SET_EXTENTS, {
            let sink = Rc::clone(&log);
            move |e, a| {
                let mut row = vec![70, a[0]];
                row.extend((0..4).map(|i| e.mem.u32(a[1] + 4 * i)));
                row.extend((0..4).map(|i| e.mem.u32(a[2] + 4 * i)));
                push(&sink, row);
                Ret::default()
            }
        });
        e.register_double(HANDLER_SLOT, {
            let sink = Rc::clone(&log);
            move |e, a| {
                let size = e.mem.u32(a[1] + 4);
                let mut row = vec![
                    80,
                    e.mem.u32(a[1] + 4),
                    e.mem.u32(a[1] + 8),
                    a[2],
                    a[3],
                    a[4],
                ];
                row.extend((0..size).map(|i| e.mem.u32(e.mem.u32(a[1]) + 4 * i)));
                push(&sink, row);
                Ret::default()
            }
        });
        log_double(&mut e, DESTROY_SLOT, &log, 40);
        // The viewer: one old display object, capacity 1, and the handler at this - 0x10.
        let block = e.mem.alloc(0x80);
        let this = block + 0x20;
        let old_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(old_vtable, DESTROY_SLOT);
        let old = e.mem.alloc(0x80);
        e.mem.set_u32(old, old_vtable);
        e.mem.set_u32(this + 4, old);
        e.mem.set_u32(this + 8, 1);
        e.mem.set_u32(this + 0xc, 1);
        let handler = object_with_slot(&mut e, 0x38, HANDLER_SLOT);
        e.mem.set_u32(this - 0x10, handler);
        // The world.
        let world = e.mem.alloc(0x100);
        let transform = e.mem.alloc(0x80);
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(transform + 0x20 + 4 * lane as u32, value);
        }
        for (lane, value) in [2.0f32, 4.0, 8.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(transform + 0x40 + 4 * lane as u32, value);
        }
        e.mem.set_u32(world + 0x78, transform);
        if with_nodes {
            let nodes = e.mem.alloc(0x40);
            put_node(
                &mut e,
                nodes,
                [100, 50, 10],
                [3, 1, 2],
                2,
                [200, 100, 30],
                [1, 2, 3],
            );
            put_node(
                &mut e,
                nodes + 0x20,
                [0xffff_fffb, 0, 0],
                [0, 0, 0],
                0,
                [8, 8, 8],
                [0, 0, 0],
            );
            let child_a = e.mem.alloc(0x60);
            e.mem.set_u16(child_a + 0x50, 2);
            e.mem.set_u32(child_a + 0x54, nodes);
            let child_b = e.mem.alloc(0x60);
            e.mem.set_u16(child_b + 0x50, 3);
            let child_c = e.mem.alloc(0x60);
            e.mem.set_u32(child_c + 0x54, nodes);
            let children_a = e.mem.alloc(8);
            e.mem.set_u32(children_a, child_a);
            e.mem.set_u32(children_a + 4, child_b);
            let entity_a = e.mem.alloc(0x60);
            e.mem.set_u32(entity_a + 0x48, children_a);
            e.mem.set_u32(entity_a + 0x4c, 2);
            let children_c = e.mem.alloc(8);
            e.mem.set_u32(children_c, child_c);
            let entity_c = e.mem.alloc(0x60);
            e.mem.set_u32(entity_c + 0x48, children_c);
            e.mem.set_u32(entity_c + 0x4c, 1);
            let first = e.mem.alloc(8);
            e.mem.set_u32(first, entity_a);
            let second = e.mem.alloc(8);
            e.mem.set_u32(second, entity_c);
            e.mem.set_u32(world + 0x28, first);
            e.mem.set_u32(world + 0x2c, 1);
            e.mem.set_u32(world + 0x34, second);
            e.mem.set_u32(world + 0x38, 1);
        }
        Callback {
            e,
            log,
            this,
            world,
            stream,
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1e-5 * b.abs().max(1.0)
    }

    fn assert_row_close(row: &[u32], want: [f32; 4]) {
        for lane in 0..4 {
            let got = f32::from_bits(row[lane]);
            assert!(
                close(got, want[lane]),
                "lane {lane}: {got} vs {}",
                want[lane]
            );
        }
    }

    #[test]
    fn the_callback_builds_a_display_box_for_each_node() {
        let mut c = callback_rig(true);
        let old = c.e.mem.u32(c.this + 4);
        c.e.call(0x00d6a2f0, &args![c.this, c.world]);
        let fresh = c.e.mem.u32(c.this + 4);
        assert_ne!(fresh, old);
        assert_eq!(c.e.mem.u32(c.this + 8), 2);
        let log = c.log.borrow();
        // The old display object is destroyed first, flag 0.
        assert_eq!(log[0], vec![40, old, 0]);
        // The first box fits the old storage; the second makes the list grow from 1 to 2.
        assert_eq!(log[1], vec![60, old]);
        assert_eq!(log[2][..2], [70, old]);
        assert_row_close(&log[2][2..6], [43.0, 9.5, -2.75, 0.0]);
        assert_row_close(&log[2][6..10], [101.0, 25.0, 2.25, 0.0]);
        assert_eq!(log[3][0], 50);
        assert_eq!(log[3][2..], [c.this + 4, 2, 0x80]);
        assert_eq!(log[4], vec![60, fresh + 0x80]);
        assert_eq!(log[5][..2], [70, fresh + 0x80]);
        // A negative integer is read as unsigned: 2^32 - 5 is 2^32 as a float.
        assert_row_close(&log[5][2..6], [2147483647.0, -2.0, -3.0, 0.0]);
        assert_row_close(&log[5][6..10], [3.0, 0.0, -2.0, 0.0]);
        // The pointer list: 2 words asked for, 8 bytes more granted, so room for 4.
        assert_eq!(log[6], vec![90, 8]);
        assert_eq!(
            log[7],
            vec![80, 2, 4, 0xffff_a500, 0, 0x42, fresh, fresh + 0x80]
        );
        assert_eq!(log[8][0], 91);
        assert_eq!(log[8][2], 16);
        assert_eq!(log.len(), 9);
        // The profile stream got the start and end records.
        assert_eq!(c.e.mem.u32(c.stream + 0x20), LABEL_START);
        assert_eq!(c.e.mem.u32(c.stream + 0x2c), LABEL_END);
        assert_eq!(c.e.mem.u32(c.stream + 4), c.stream + 0x38);
    }

    #[test]
    fn the_callback_with_no_boxes_hands_over_an_empty_list() {
        let mut c = callback_rig(false);
        let old = c.e.mem.u32(c.this + 4);
        c.e.call(0x00d6a2f0, &args![c.this, c.world]);
        assert_eq!(c.e.mem.u32(c.this + 8), 0);
        let log = c.log.borrow();
        assert_eq!(
            *log,
            vec![
                vec![40, old, 0],
                vec![80, 0, 0x8000_0000, 0xffff_a500, 0, 0x42],
            ]
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
    fn fn_00d6a810_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_a280, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_a810, &args![0x1234u32]).u32();
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
    fn fn_00d6a810_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_a280, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_a810, &args![0x1234u32]).u32(), 0);
    }
}
