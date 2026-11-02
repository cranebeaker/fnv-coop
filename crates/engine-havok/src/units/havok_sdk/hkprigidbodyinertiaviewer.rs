//! `Havok SDK/hkprigidbodyinertiaviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Neither function has a /GS cookie or an exception frame. The float arithmetic is the x87's
//! (computed in `f64`, rounded to `f32` at every store, as the guide says); the vectors are SSE.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_vec, profile_record, store_vec, zip_with, PROFILE_END, ZERO_F32,
};

/// The memory router (`00c85750`): its `+0x10` is the allocator (slot `0x10` frees).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpMotion::getMass` (`00c86430`, thiscall on the motion, the mass in `ST0`).
const GET_MASS: u32 = 0x00c8_6430;
/// The CRT square root (`00ec6040`, argument and result in `ST0`).
const X87_SQRT: u32 = 0x00ec_6040;
/// The array reservation (`00c908c0`, cdecl: allocator, array, capacity, element size).
const RESERVE_ARRAY: u32 = 0x00c9_08c0;
/// The display box constructor (`00de1fc0`, thiscall, no stack words) and its set-up
/// (`00de2010`, thiscall: half extents, transform; `RET 8`).
const DISPLAY_BOX_CONSTRUCT: u32 = 0x00de_1fc0;
const DISPLAY_BOX_SET: u32 = 0x00de_2010;
/// `hkVector4::setRotatedDir` (`00c7f700`, thiscall on the result: rotation, vector; `RET 8`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkProcessFactory::registerProcess` (`00dd9380`, thiscall on the factory: name, creator;
/// `RET 8`), the factory (`01269cd0`), the id it answers (`01268ffc`), the creator
/// (`00d67880`) and the name `Inertia Tensors`.
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const PROCESS_ID: u32 = 0x0126_8ffc;
const CREATOR: u32 = 0x00d6_7880;
const PROCESS_NAME: u32 = 0x010d_1d10;
/// The doubles the box sizes are made of: `6.0` and `12.0` (`01034208`, `01035710`), and the
/// extent factors `1.01` (a float widened, `010d0f68`) and `0.5` (`01011588`).
const SIX: u32 = 0x0103_4208;
const TWELVE: u32 = 0x0103_5710;
const EXTENT_GROWTH: u32 = 0x010d_0f68;
const HALF: u32 = 0x0101_1588;

/// The profiling labels: the callback, collecting the properties and sending them (the end
/// label is `PROFILE_END`).
const LABEL_VIEWER: u32 = 0x010d_1da4;
const LABEL_GET_PROPS: u32 = 0x010d_1d98;
const LABEL_SEND_PROPS: u32 = 0x010d_1d8c;

/// Size of a display box element of the viewer's array.
const BOX_SIZE: u32 = 0x70;

/// The square root of `x` through the CRT helper, or `0` when `x` is below zero (a NaN takes
/// the root); the result passes the x87 store and load, so a signalling NaN comes out quiet.
fn clamped_root(e: &mut Engine, x: f32) -> f32 {
    if x < 0.0 {
        0.0
    } else {
        x87_quiet(e.call(X87_SQRT, &args![f64::from(x)]).f64() as f32)
    }
}

/// Resizes the array at `array` (data, size, capacity word) of 0x70-byte display boxes to
/// `count` entries: the capacity (low 30 bits of `+8`) is raised through `00c908c0` to the
/// larger of twice itself and `count` when it is below `count`; entries from `count` up to the
/// old size are destroyed through slot 0 of their vtable with the flag 0 (the element is not
/// freed), those from the old size up to `count` are constructed (`00de1fc0`, skipped for a null
/// address), and the size becomes `count`.
fn set_box_count(e: &mut Engine, array: u32, count: i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(
            RESERVE_ARRAY,
            &args![allocator, array, wanted as u32, BOX_SIZE],
        );
    }
    let size = e.mem.i32(array + 4);
    let mut element = (count as u32)
        .wrapping_mul(BOX_SIZE)
        .wrapping_add(e.mem.u32(array));
    let mut remaining = size.wrapping_sub(count);
    if remaining > 0 {
        loop {
            e.vcall(element, 0, &args![0u32]);
            element = element.wrapping_add(BOX_SIZE);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    let size = e.mem.i32(array + 4);
    let mut remaining = count.wrapping_sub(size);
    let mut element = (size as u32)
        .wrapping_mul(BOX_SIZE)
        .wrapping_add(e.mem.u32(array));
    if remaining > 0 {
        loop {
            if element != 0 {
                e.call(DISPLAY_BOX_CONSTRUCT, &args![element]);
            }
            element = element.wrapping_add(BOX_SIZE);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    e.mem.set_i32(array + 4, count);
}

// Translated from 00d67340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBodyInertiaViewer::postSimulationCallback` (Xbox PDB), fastcall on the
/// post-simulation listener sub-object (`this`: the body array's data at `+4`, its size at
/// `+8`, the display box array at `+0x10` / `+0x14` / `+0x18`; the display handler pointer is
/// the word at `this - 0x14`) with one unread stack word (`RET 4`).
///
/// The box array is first resized to the number of bodies. Every body with a mass that is not
/// zero (a NaN counts) asks its motion (`+0xE0`, slot `0x14`) for the inertia matrix (three
/// 16-byte columns), takes the diagonal `i0`, `i1`, `i2` and the mass `m`, and computes
/// `v = ((i0 - i1) + i2) / m * 6`, `b = (i0 / m) * 12 - v` and `c = (i2 / m) * 12 - v`, each
/// rounded to a float. The three square roots (CRT `00ec6040`; `0` for a negative value) of
/// `v`, `b` and `c` are scaled by `1.01 * 0.5` into the half extents `(root c, root v, root b,
/// 0)`. The box is placed with the body's rotation columns (`+0xF0`, `+0x100`, `+0x110`) and
/// the translation `+0x120` plus the body's `+0x170` vector rotated by them
/// (`hkVector4::setRotatedDir`), then set up (`00de2010`) as the next box. After the loop the
/// array is cut to the number of boxes made, a temporary array of pointers to them is built
/// back to front and handed to slot `0x38` of the display handler with `(array, 0xffff00ff, 0,
/// process id)`, and the temporary storage is freed through slot `0x10` of the allocator when
/// it was allocated.
///
/// The code keeps its working values in its stack frame; the frame here is a block of 0xE0
/// bytes with the same offsets: the temporary array at `+0x18`, the rotated vector `+0x30`,
/// the half extents `+0x40`, the inertia matrix `+0x50`, the body vector `+0x80` and the
/// transform copy `+0x90`.
pub fn hkp_rigid_body_inertia_viewer_post_simulation_callback(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) {
    let me = this.addr();
    profile_record(e, LABEL_VIEWER);
    profile_record(e, LABEL_GET_PROPS);
    e.with_stack(0xe0, |e, frame| {
        let frame = frame.addr();
        let boxes = me + 0x10;
        let bodies = e.mem.i32(me + 8);
        set_box_count(e, boxes, bodies);
        let mut drawn: i32 = 0;
        let mut box_offset: u32 = 0;
        let mut index: i32 = 0;
        while index < e.mem.i32(me + 8) {
            let body = e.mem.u32(
                e.mem
                    .u32(me + 4)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let mass = x87_quiet(e.call(GET_MASS, &args![body.wrapping_add(0xe0)]).f32());
            if mass != 0.0 {
                let body = e.mem.u32(
                    e.mem
                        .u32(me + 4)
                        .wrapping_add((index as u32).wrapping_mul(4)),
                );
                let motion = body.wrapping_add(0xe0);
                e.vcall(motion, 0x14, &args![frame + 0x50]);
                let first = f64::from(e.mem.f32(frame + 0x50));
                let second = f64::from(e.mem.f32(frame + 0x64));
                let third = f64::from(e.mem.f32(frame + 0x78));
                let mass = f64::from(mass);
                let six = e.mem.f64(SIX);
                let twelve = e.mem.f64(TWELVE);
                let rise = ((((first - second) + third) / mass) * six) as f32;
                let root_rise = clamped_root(e, rise);
                let rise = f64::from(rise);
                let wide = (((first / mass) * twelve) - rise) as f32;
                let root_wide = clamped_root(e, wide);
                let deep = (((third / mass) * twelve) - rise) as f32;
                let root_deep = clamped_root(e, deep);
                let growth = e.mem.f64(EXTENT_GROWTH);
                let half = e.mem.f64(HALF);
                let scaled_deep = ((f64::from(root_deep) * growth) * half) as f32;
                let scaled_rise = ((f64::from(root_rise) * growth) * half) as f32;
                let scaled_wide = (half * (growth * f64::from(root_wide))) as f32;
                let zero: f32 = e.global(ZERO_F32);
                store_vec(
                    e,
                    frame + 0x40,
                    [scaled_deep, scaled_rise, scaled_wide, zero],
                );
                // The transform copy: the three rotation columns and the translation.
                let columns = [
                    load_vec(e, body.wrapping_add(0xf0)),
                    load_vec(e, body.wrapping_add(0x100)),
                    load_vec(e, body.wrapping_add(0x110)),
                    load_vec(e, body.wrapping_add(0x120)),
                ];
                for (slot, column) in [(0x90, 0), (0xa0, 1), (0xb0, 2), (0xc0, 3)] {
                    store_vec(e, frame + slot, columns[column]);
                }
                let offset_vector = load_vec(e, body.wrapping_add(0x170));
                store_vec(e, frame + 0x80, offset_vector);
                e.call(
                    SET_ROTATED_DIR,
                    &args![frame + 0x30, frame + 0x90, frame + 0x80],
                );
                let translation = load_vec(e, frame + 0xc0);
                let rotated = load_vec(e, frame + 0x30);
                let moved = zip_with(translation, rotated, |a, b| a + b);
                drawn += 1;
                let box_address = e.mem.u32(boxes).wrapping_add(box_offset);
                store_vec(e, frame + 0x30, moved);
                store_vec(e, frame + 0xc0, moved);
                box_offset = box_offset.wrapping_add(BOX_SIZE);
                e.call(
                    DISPLAY_BOX_SET,
                    &args![box_address, frame + 0x40, frame + 0x90],
                );
            }
            index += 1;
        }
        profile_record(e, PROFILE_END);
        profile_record(e, LABEL_SEND_PROPS);
        set_box_count(e, boxes, drawn);
        // The temporary array of pointers to the boxes: data, size, capacity word.
        let count = e.mem.i32(me + 0x14);
        e.mem.set_u32(frame + 0x18, 0);
        e.mem.set_u32(frame + 0x1c, 0);
        e.mem.set_u32(frame + 0x20, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(frame + 0x20) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(
                RESERVE_ARRAY,
                &args![allocator, frame + 0x18, wanted as u32, 4u32],
            );
        }
        let mut slot = e.mem.i32(me + 0x14).wrapping_sub(1);
        e.mem.set_i32(frame + 0x1c, count);
        if slot >= 0 {
            let mut bytes = (slot as u32).wrapping_mul(BOX_SIZE);
            loop {
                let target = e.mem.u32(boxes).wrapping_add(bytes);
                let data = e.mem.u32(frame + 0x18);
                e.mem
                    .set_u32(data.wrapping_add((slot as u32).wrapping_mul(4)), target);
                slot = slot.wrapping_sub(1);
                bytes = bytes.wrapping_sub(BOX_SIZE);
                if slot < 0 {
                    break;
                }
            }
        }
        let process: u32 = e.global(PROCESS_ID);
        let handler = e.mem.u32(me.wrapping_sub(0x14));
        e.vcall(
            handler,
            0x38,
            &args![frame + 0x18, 0xffff_00ffu32, 0u32, process],
        );
        profile_record(e, PROFILE_END);
        profile_record(e, PROFILE_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(frame + 0x20);
        e.mem.set_u32(frame + 0x1c, 0);
        if (capacity as i32) >= 0 {
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            let data = e.mem.u32(frame + 0x18);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00d678c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBodyInertiaViewer::registerViewer` (Xbox PDB): registers the process
/// `Inertia Tensors` (creator `00d67880`) with the process factory (`01269cd0`) and keeps the
/// id in `01268ffc`.
pub fn hkp_rigid_body_inertia_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(REGISTER_PROCESS, &args![factory, PROCESS_NAME, CREATOR])
        .u32();
    e.set_global(PROCESS_ID, id);
}

// Translated from 00d67880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x48 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d671f0` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d678c6.
pub fn fn_00d67880(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x48]).u32();
    e.mem.set_u16(block + 4, 0x48);
    let object = e.call(0x00d6_71f0, &args![block, process_contexts]).u32();
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
            0x00d6_7340,
            hkp_rigid_body_inertia_viewer_post_simulation_callback(Ptr, u32)
        ),
        entry!(0x00d6_78c0, hkp_rigid_body_inertia_viewer_register_viewer()),
        entry!(0x00d6_7880, fn_00d67880(u32) -> u32),
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

    /// The motion's slot `0x14`, the display handler's slot `0x38`, the allocator's slot
    /// `0x10` and a box's slot 0 (the destructor).
    const MOTION_INERTIA: u32 = 0x0077_0014;
    const HANDLER_DRAW: u32 = 0x0078_0038;
    const ALLOCATOR_FREE: u32 = 0x0079_0010;
    const BOX_DESTRUCT: u32 = 0x007a_0000;

    /// What `00de2010` was given: (box, half extents, the four transform columns).
    type BoxSet = (u32, Vec4, [Vec4; 4]);
    /// What the display handler's slot `0x38` saw: (the array's data, its size, the pointers in
    /// it, colour, third word, process id).
    type Draw = (u32, u32, Vec<u32>, u32, u32, u32);

    struct Rig {
        e: Engine,
        log: Log,
        this: u32,
        stream: u32,
        records: u32,
        boxes: Rc<RefCell<Vec<BoxSet>>>,
        draws: Rc<RefCell<Vec<Draw>>>,
        bodies: Vec<u32>,
    }

    /// A viewer watching one body per `(mass, inertia diagonal)` entry; the boxes array starts
    /// empty and unowned. The doubles: the router, the reservation (a fresh zeroed buffer with
    /// the old entries copied, capacity set to the request), the mass, the motion's inertia,
    /// the square root, the display box constructor and set-up, `setRotatedDir` (twice the
    /// vector) and the display handler.
    fn rig(bodies: &[(f64, [f32; 3])]) -> Rig {
        let mut e = engine();
        e.set_global(SIX, 6.0f64);
        e.set_global(TWELVE, 12.0f64);
        e.set_global(EXTENT_GROWTH, f64::from(1.01f32));
        e.set_global(HALF, 0.5f64);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(PROCESS_ID, 0x66u32);
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        // The router and its allocator.
        let allocator_table = e.mem.alloc(0x40);
        e.mem.set_u32(allocator_table + 0x10, ALLOCATOR_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        log_double(&mut e, &log, ALLOCATOR_FREE, 0);
        // The reservation.
        let seen = Rc::clone(&log);
        e.register_double(RESERVE_ARRAY, move |e, a| {
            seen.borrow_mut().push((RESERVE_ARRAY, a.to_vec()));
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let new = e.mem.alloc((capacity * element).max(8));
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(new + byte, value);
            }
            e.mem.set_u32(array, new);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        // The bodies and their motions.
        let motion_table = e.mem.alloc(0x40);
        e.mem.set_u32(motion_table + 0x14, MOTION_INERTIA);
        let mut made = Vec::new();
        let mut masses: Vec<(u32, f64, [f32; 3])> = Vec::new();
        for (index, (mass, inertia)) in bodies.iter().enumerate() {
            let body = e.mem.alloc(0x180);
            e.mem.set_u32(body + 0xe0, motion_table);
            for column in 0..4u32 {
                let base = [1.0f32, 2.0, 3.0, 4.0].map(|lane| lane + column as f32 + index as f32);
                for (lane, value) in base.iter().enumerate() {
                    e.mem
                        .set_f32(body + 0xf0 + 0x10 * column + 4 * lane as u32, *value);
                }
            }
            for lane in 0..4u32 {
                e.mem
                    .set_f32(body + 0x170 + 4 * lane, (lane + 1) as f32 * 0.5);
            }
            masses.push((body + 0xe0, *mass, *inertia));
            made.push(body);
        }
        let masses = Rc::new(masses);
        let asked = Rc::clone(&masses);
        let seen = Rc::clone(&log);
        e.register_double(GET_MASS, move |_, a| {
            seen.borrow_mut().push((GET_MASS, a.to_vec()));
            let entry = asked.iter().find(|(motion, _, _)| *motion == a[0]).unwrap();
            Ret {
                st0: entry.1,
                ..Ret::default()
            }
        });
        let asked = Rc::clone(&masses);
        e.register_double(MOTION_INERTIA, move |e, a| {
            let entry = asked.iter().find(|(motion, _, _)| *motion == a[0]).unwrap();
            for (offset, value) in [0x00, 0x14, 0x28].into_iter().zip(entry.2) {
                e.mem.set_f32(a[1] + offset, value);
            }
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(X87_SQRT, move |_, a| {
            seen.borrow_mut().push((X87_SQRT, a.to_vec()));
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        // The display boxes.
        let box_table = e.mem.alloc(0x10);
        e.mem.set_u32(box_table, BOX_DESTRUCT);
        log_double(&mut e, &log, BOX_DESTRUCT, 0);
        let seen = Rc::clone(&log);
        e.register_double(DISPLAY_BOX_CONSTRUCT, move |e, a| {
            seen.borrow_mut().push((DISPLAY_BOX_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0], box_table);
            Ret::default()
        });
        let boxes: Rc<RefCell<Vec<BoxSet>>> = Rc::default();
        let seen = Rc::clone(&boxes);
        e.register_double(DISPLAY_BOX_SET, move |e, a| {
            let columns = [0, 0x10, 0x20, 0x30].map(|offset| load_vec(e, a[2] + offset));
            seen.borrow_mut().push((a[0], load_vec(e, a[1]), columns));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            seen.borrow_mut().push((SET_ROTATED_DIR, a.to_vec()));
            let vector = load_vec(e, a[2]);
            store_vec(e, a[0], vector.map(|lane| lane * 2.0));
            Ret::default()
        });
        // The display handler.
        let draws: Rc<RefCell<Vec<Draw>>> = Rc::default();
        let seen = Rc::clone(&draws);
        e.register_double(HANDLER_DRAW, move |e, a| {
            let (data, size) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            let pointers = (0..size).map(|i| e.mem.u32(data + 4 * i)).collect();
            seen.borrow_mut()
                .push((data, size, pointers, a[2], a[3], a[4]));
            Ret::default()
        });
        let handler_table = e.mem.alloc(0x40);
        e.mem.set_u32(handler_table + 0x38, HANDLER_DRAW);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(handler, handler_table);
        // The viewer.
        let block = e.mem.alloc(0x80);
        let this = block + 0x20;
        e.mem.set_u32(this - 0x14, handler);
        let array = e.mem.alloc(0x40);
        for (index, body) in made.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *body);
        }
        e.mem.set_u32(this + 4, array);
        e.mem.set_u32(this + 8, made.len() as u32);
        e.mem.set_u32(this + 0x18, 0x8000_0000);
        Rig {
            e,
            log,
            this,
            stream,
            records,
            boxes,
            draws,
            bodies: made,
        }
    }

    /// The box half extents for the three roots, as the code scales them.
    fn extents(deep: f64, rise: f64, wide: f64) -> Vec4 {
        let (deep, rise, wide) = (
            f64::from(deep as f32),
            f64::from(rise as f32),
            f64::from(wide as f32),
        );
        let growth = f64::from(1.01f32);
        [
            ((deep * growth) * 0.5) as f32,
            ((rise * growth) * 0.5) as f32,
            (0.5 * (growth * wide)) as f32,
            0.0,
        ]
    }

    #[test]
    fn two_massive_bodies_are_drawn_and_the_massless_one_is_skipped_00d67340() {
        // Inertia (6, 15, 15) with mass 1 and (12, 30, 30) with mass 2 both give the roots
        // 6, 6 and 12 (6 * 6 = 36, 12 * 6 - 36 = 36, 12 * 15 - 36 = 144).
        let mut r = rig(&[
            (1.0, [6.0, 15.0, 15.0]),
            (0.0, [1.0, 1.0, 1.0]),
            (2.0, [12.0, 30.0, 30.0]),
        ]);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        let expected_extents = extents(12.0, 6.0, 6.0);
        let boxes_data = r.e.mem.u32(r.this + 0x10);
        assert_eq!(r.e.mem.u32(r.this + 0x14), 2);
        // First the array grew to three entries, then it was cut to two.
        let log = &r.log;
        assert_eq!(calls(log, RESERVE_ARRAY).len(), 2);
        assert_eq!(calls(log, RESERVE_ARRAY)[0][2..], [3, 0x70]);
        assert_eq!(calls(log, RESERVE_ARRAY)[1][2..], [2, 4]);
        let first_data = calls(log, DISPLAY_BOX_CONSTRUCT)[0][0];
        assert_eq!(
            calls(log, DISPLAY_BOX_CONSTRUCT),
            vec![
                vec![first_data],
                vec![first_data + 0x70],
                vec![first_data + 0xe0]
            ]
        );
        assert_eq!(first_data, boxes_data);
        assert_eq!(calls(log, BOX_DESTRUCT), vec![vec![first_data + 0xe0, 0]]);
        // Both massive bodies have the same roots; the roots are the CRT's, three per body.
        assert_eq!(calls(log, X87_SQRT).len(), 6);
        let boxes = r.boxes.borrow();
        assert_eq!(boxes.len(), 2);
        for (index, set) in boxes.iter().enumerate() {
            assert_eq!(set.0, first_data + 0x70 * index as u32);
            assert_eq!(set.1, expected_extents);
            let base = [1.0f32, 2.0, 3.0, 4.0].map(|lane| lane + [0.0, 2.0][index]);
            for column in 0..3 {
                assert_eq!(set.2[column], base.map(|lane| lane + column as f32));
            }
            // The translation column plus twice the body's +0x170 vector.
            let offset = [1.0, 2.0, 3.0, 4.0];
            let translation = base.map(|lane| lane + 3.0);
            assert_eq!(set.2[3], zip_with(translation, offset, |a, b| a + b));
        }
        let motions: Vec<Vec<u32>> = r.bodies.iter().map(|body| vec![body + 0xe0]).collect();
        assert_eq!(calls(log, GET_MASS), motions);
        assert_eq!(calls(log, SET_ROTATED_DIR).len(), 2);
        // The handler got the pointers to the boxes in order.
        let draws = r.draws.borrow();
        assert_eq!(draws.len(), 1);
        assert_eq!(draws[0].1, 2);
        assert_eq!(draws[0].2, vec![first_data, first_data + 0x70]);
        assert_eq!((draws[0].3, draws[0].4, draws[0].5), (0xffff_00ff, 0, 0x66));
        // The temporary array was freed with its capacity in bytes.
        let frees = calls(log, ALLOCATOR_FREE);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [draws[0].0, 8]);
        assert_eq!(
            profile_labels(&r.e, r.stream, r.records),
            vec![
                LABEL_VIEWER,
                LABEL_GET_PROPS,
                PROFILE_END,
                LABEL_SEND_PROPS,
                PROFILE_END,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn a_negative_rise_has_no_root_00d67340() {
        // (1, 10, 1) with mass 1: rise = 6 * ((1 - 10) + 1) = -48, so no root and no scaling
        // of it; (1 * 12) + 48 = 60 for the other two.
        let mut r = rig(&[(1.0, [1.0, 10.0, 1.0])]);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        let boxes = r.boxes.borrow();
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].1, extents(60f64.sqrt(), 0.0, 60f64.sqrt()));
        assert_eq!(calls(&r.log, X87_SQRT).len(), 2);
    }

    #[test]
    fn a_negative_width_has_no_root_00d67340() {
        // (1, 0, 10) with mass 1: rise = 66, wide = 12 - 66 < 0, deep = 120 - 66 = 54.
        let mut r = rig(&[(1.0, [1.0, 0.0, 10.0])]);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        let boxes = r.boxes.borrow();
        assert_eq!(boxes[0].1, extents(54f64.sqrt(), 66f64.sqrt(), 0.0));
        assert_eq!(calls(&r.log, X87_SQRT).len(), 2);
    }

    #[test]
    fn a_negative_depth_has_no_root_00d67340() {
        // (10, 0, 1) with mass 1: rise = 66, wide = 120 - 66 = 54, deep = 12 - 66 < 0.
        let mut r = rig(&[(1.0, [10.0, 0.0, 1.0])]);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        let boxes = r.boxes.borrow();
        assert_eq!(boxes[0].1, extents(0.0, 66f64.sqrt(), 54f64.sqrt()));
        assert_eq!(calls(&r.log, X87_SQRT).len(), 2);
    }

    #[test]
    fn a_nan_mass_is_drawn_with_nan_extents_00d67340() {
        let mut r = rig(&[(f64::NAN, [6.0, 15.0, 15.0])]);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        let boxes = r.boxes.borrow();
        assert_eq!(boxes.len(), 1);
        assert!(boxes[0].1[..3].iter().all(|lane| lane.is_nan()));
        assert_eq!(boxes[0].1[3], 0.0);
        // A NaN is not below zero, so the roots are asked for.
        assert_eq!(calls(&r.log, X87_SQRT).len(), 3);
    }

    #[test]
    fn no_bodies_make_an_empty_draw_00d67340() {
        let mut r = rig(&[]);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        assert!(r.boxes.borrow().is_empty());
        assert!(calls(&r.log, RESERVE_ARRAY).is_empty());
        let draws = r.draws.borrow();
        assert_eq!(draws.len(), 1);
        assert_eq!(draws[0].1, 0);
        assert!(draws[0].2.is_empty());
        assert!(calls(&r.log, ALLOCATOR_FREE).is_empty());
        assert_eq!(r.e.mem.u32(r.this + 0x14), 0);
        assert_eq!(
            profile_labels(&r.e, r.stream, r.records),
            vec![
                LABEL_VIEWER,
                LABEL_GET_PROPS,
                PROFILE_END,
                LABEL_SEND_PROPS,
                PROFILE_END,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn an_array_with_room_is_not_reserved_again_00d67340() {
        let mut r = rig(&[(1.0, [6.0, 15.0, 15.0])]);
        // Two constructed boxes in a buffer owned by the array, room for four.
        let data = r.e.mem.alloc(4 * 0x70);
        let box_table = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(box_table, BOX_DESTRUCT);
        r.e.mem.set_u32(data, box_table);
        r.e.mem.set_u32(data + 0x70, box_table);
        r.e.mem.set_u32(r.this + 0x10, data);
        r.e.mem.set_u32(r.this + 0x14, 2);
        r.e.mem.set_u32(r.this + 0x18, 4);
        r.e.call(0x00d6_7340, &args![r.this, 0u32]);
        // Only the temporary pointer array needed storage.
        assert_eq!(calls(&r.log, RESERVE_ARRAY).len(), 1);
        assert_eq!(calls(&r.log, BOX_DESTRUCT), vec![vec![data + 0x70, 0]]);
        assert!(calls(&r.log, DISPLAY_BOX_CONSTRUCT).is_empty());
        assert_eq!(r.e.mem.u32(r.this + 0x14), 1);
        assert_eq!(r.boxes.borrow()[0].0, data);
    }

    #[test]
    fn registering_the_viewer_keeps_the_process_id_00d678c0() {
        let mut e = engine();
        let log = new_log();
        e.set_global(PROCESS_FACTORY, 0xfac0u32);
        log_double(&mut e, &log, REGISTER_PROCESS, 0x4b);
        e.call(0x00d6_78c0, &args![]);
        assert_eq!(
            calls(&log, REGISTER_PROCESS),
            vec![vec![0xfac0, 0x010d_1d10, 0x00d6_7880]]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x4b);
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
    fn fn_00d67880_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_71f0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_7880, &args![0x1234u32]).u32();
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
    fn fn_00d67880_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_71f0, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_7880, &args![0x1234u32]).u32(), 0);
    }
}
