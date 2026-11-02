//! `Havok SDK/hkpvehicleviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{
    addps, mulps, normalising_scale, shufps, splat, subps, sum_of_three,
};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_vec, profile_record, store_vec, Vec4, HALF_F32, PROFILE_END, THREE_F32, ZERO_F32,
};

// hkpVehicleViewer has no type in the Xbox PDB, so its layout is described by the offsets the
// PC code uses (roles unconfirmed). `postSimulationCallback` is called with a sub-object of the
// viewer: relative to it `-0x14` is the display handler (its slot `0x24` draws a line), `+4`
// the array of vehicles (`+8` their count). The viewer itself holds the `hkpWorld` at `+0x24`
// and an `hkArray` (`+0x30` data, `+0x34` size, `+0x38` capacity word) it empties on exit.
// A vehicle (`hkpVehicleInstance`) has its data block at `+0x1c` (`+0x8c` the per-wheel
// parameter records, `0x28` bytes each, whose float at `+0` is a radius and whose byte at
// `+0x24` is the axle; `+0x98` a signed byte per axle, the number of wheels on it; `+0x9c` the
// number of axles), the wheel records at `+0x48` (`0xe0` bytes each) and their number at
// `+0x4c`. A wheel record has its position at `+0`, a contact object at `+0x24`, the ray start
// at `+0x50`, a direction at `+0x80`, an axis at `+0xa0` and an angle at `+0xc4`.

/// Memory router (`00c85750`): its field `+0` is the stack allocator's top, `+0x10` the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkQuaternion::setAxisAngle` (`ECX` the output, then the axis pointer and the angle).
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// The viewer's process identifier (the result of `registerProcess`, `.data`).
const PROCESS_ID: u32 = 0x0126_8fcc;
/// The process factory (`.data`) `registerViewer` registers with.
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
/// `hkProcessFactory::registerProcess` (`ECX` the factory, then the name and the creator).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
/// The name `registerViewer` registers (`"Vehicles"`) and the creator function it passes.
const VEHICLES_NAME: u32 = 0x010d_17c4;
const VIEWER_CREATOR: u32 = 0x00d6_3d60;
/// `hkpWorldViewerBase::hkpWorldViewerBase` (`ECX` the object, then the world).
const WORLD_VIEWER_BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
/// `hkpWorldViewerBase::~hkpWorldViewerBase` (`ECX` the object; reached by a tail jump).
const WORLD_VIEWER_BASE_DESTRUCT: u32 = 0x00dd_cc60;
/// Two `hkpWorld` functions of the world at `+0x24` of the viewer, each called with the entity
/// in `ECX` and a pointer into the viewer (roles unconfirmed).
const ENTITY_CALL_A: u32 = 0x00c9_1700;
const ENTITY_CALL_B: u32 = 0x00c9_18e0;
/// Profile label of the start record of `postSimulationCallback` ("TthkpVehicleViewer::step").
const STEP_LABEL: u32 = 0x010d_17e4;
/// Four `-0.5f` (`.rdata`), the vector the quaternion rotation adds to the squared `w`.
const NEGATIVE_HALVES: u32 = 0x010c_71e0;
/// The `float` angle (`.rdata`) the circle around a wheel is stepped by.
const CIRCLE_ANGLE: u32 = 0x0107_8a5c;

/// The vtables the constructor and destructor store (`.rdata`).
const VIEWER_VTABLE: u32 = 0x010d_1854;
const VIEWER_VTABLE_8: u32 = 0x010d_1834;
const VIEWER_VTABLE_20: u32 = 0x010d_1824;
const VIEWER_VTABLE_28: u32 = 0x010d_1814;
const VIEWER_VTABLE_2C: u32 = 0x010d_1804;
const BASE_VTABLE_28: u32 = 0x010c_b1d4;
const BASE_VTABLE_2C: u32 = 0x010c_cf7c;

// Offsets of the vectors in `postSimulationCallback`'s scratch block: the twelve points of the
// circle (`0x10` bytes each), the two quaternions, the two ends of the wheel's cross line and
// the previous and current points of the axle line.
const CIRCLE: u32 = 0;
const WHEEL_QUATERNION: u32 = 0xc0;
const CIRCLE_QUATERNION: u32 = 0xd0;
const LINE_START: u32 = 0xe0;
const LINE_END: u32 = 0xf0;
const PREVIOUS_POINT: u32 = 0x100;
const CURRENT_POINT: u32 = 0x110;

// Translated from 00d63480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall, one stack word (`RET 4`): `entity`. `this` is the viewer's sub-object at `+0x20`;
/// calls `00c91700` and `00c918e0` with `entity` in `ECX` and the pointers `this + 8` and
/// `this + 0xc` (zero when `this - 0x20` is null) as their stack word (the entity-listener
/// removals the destructor runs for every entity of the world; roles unconfirmed).
pub fn fn_00d63480(e: &mut Engine, this: Ptr, entity: u32) {
    let t = this.addr();
    let present = t.wrapping_sub(0x20) != 0;
    let first = if present { t.wrapping_add(8) } else { 0 };
    e.call(ENTITY_CALL_A, &args![entity, first]);
    let second = if present { t.wrapping_add(0xc) } else { 0 };
    e.call(ENTITY_CALL_B, &args![entity, second]);
}

/// The rotation of `vector` by the quaternion `q`:
/// `2 * (w * (q x v) + (w * w - 0.5) * v + (q . v) * q)` (the `-0.5` vector comes from memory).
fn rotate_by_quaternion(e: &Engine, q: Vec4, vector: Vec4) -> Vec4 {
    let halves = load_vec(e, NEGATIVE_HALVES);
    let dot = sum_of_three(mulps(vector, q));
    let cross = subps(
        mulps(shufps(vector, vector, 0xd2), shufps(q, q, 0xc9)),
        mulps(shufps(vector, vector, 0xc9), shufps(q, q, 0xd2)),
    );
    let w = splat(q[3]);
    let turned = mulps(cross, w);
    let diagonal = mulps(addps(mulps(w, w), halves), vector);
    let rest = addps(diagonal, mulps(splat(dot), q));
    let sum = addps(turned, rest);
    addps(sum, sum)
}

/// A line through slot `0x24` of the display handler (`this - 0x14`): `from`, `to`, `color`,
/// `tag` and the viewer's process id.
fn draw_line(e: &mut Engine, this: u32, from: u32, to: u32, color: u32, tag: u32) {
    let display = e.mem.u32(this.wrapping_sub(0x14));
    let process: u32 = e.global(PROCESS_ID);
    e.vcall(display, 0x24, &args![from, to, color, tag, process]);
}

/// The colour `(mask & bits) + 0xffbc8f8f`, `mask` all ones when the wheel has a contact object
/// (`NEG`/`SBB`).
fn wheel_color(ground: u32, bits: u32) -> u32 {
    let mask = if ground != 0 { 0xffff_ffff } else { 0 };
    (mask & bits).wrapping_add(0xffbc_8f8f)
}

/// Draws one wheel of `vehicle`: the line from its record (`+0`) to its ray start (`+0x50`), the
/// wheel's point (stored to `point`, `-radius` times the direction from the record position),
/// the line across the wheel (the point plus and minus the normalised rotated direction times
/// the radius), and the twelve-point circle of the wheel (the radius vector rotated step by step
/// by the quaternion for `CIRCLE_ANGLE` about the axis, each point offset by the wheel's
/// point) as twelve lines.
fn draw_wheel(
    e: &mut Engine,
    this: u32,
    scratch: u32,
    vehicle: u32,
    wheel: u32,
    point: u32,
    constants: [f32; 3],
) {
    let [zero, three, half] = constants;
    let info = e
        .mem
        .u32(vehicle + 0x48)
        .wrapping_add(wheel.wrapping_mul(0xe0));
    let parameters = e
        .mem
        .u32(e.mem.u32(vehicle + 0x1c) + 0x8c)
        .wrapping_add(wheel.wrapping_mul(0x28));
    let ground = e.mem.u32(info + 0x24);
    let tag = if ground != 0 {
        ground.wrapping_add(0x10)
    } else {
        0
    };
    draw_line(
        e,
        this,
        info,
        info + 0x50,
        wheel_color(ground, 0x0042_7071),
        tag,
    );
    let negative_radius = f32::from_bits(x87_quiet_bits(e.mem.u32(parameters)) ^ 0x8000_0000);
    let direction = load_vec(e, info + 0x80);
    let origin = load_vec(e, info);
    store_vec(
        e,
        point,
        addps(mulps(splat(negative_radius), direction), origin),
    );
    let angle = x87_quiet_bits(e.mem.u32(info + 0xc4)) ^ 0x8000_0000;
    e.call(
        SET_AXIS_ANGLE,
        &args![scratch + WHEEL_QUATERNION, info + 0xa0, angle],
    );
    let q = load_vec(e, scratch + WHEEL_QUATERNION);
    let rotated = rotate_by_quaternion(e, q, direction);
    let scale = normalising_scale(sum_of_three(mulps(rotated, rotated)), zero, half, three);
    let radius = e.mem.f32(parameters);
    let offset = mulps(mulps(splat(scale), rotated), splat(radius));
    let centre = load_vec(e, point);
    store_vec(e, scratch + LINE_START, addps(centre, offset));
    store_vec(e, scratch + LINE_END, subps(centre, offset));
    let ground = e.mem.u32(info + 0x24);
    draw_line(
        e,
        this,
        scratch + LINE_START,
        scratch + LINE_END,
        wheel_color(ground, 0x0039_6666),
        tag,
    );
    let circle_angle = x87_quiet_bits(e.mem.u32(CIRCLE_ANGLE));
    let direction = load_vec(e, info + 0x80);
    let radius_vector = mulps(splat(e.mem.f32(parameters)), direction);
    store_vec(e, scratch + CIRCLE, radius_vector);
    e.call(
        SET_AXIS_ANGLE,
        &args![scratch + CIRCLE_QUATERNION, info + 0xa0, circle_angle],
    );
    let mut q = load_vec(e, scratch + CIRCLE_QUATERNION);
    for step in 1..14u32 {
        let previous = scratch + CIRCLE + 0x10 * (step - 1);
        if step < 12 {
            let turned = rotate_by_quaternion(e, q, load_vec(e, previous));
            store_vec(e, previous + 0x10, turned);
        }
        if step < 13 {
            let moved = addps(load_vec(e, point), load_vec(e, previous));
            store_vec(e, previous, moved);
        }
        if step >= 2 {
            let from = scratch + CIRCLE + 0x10 * ((step - 2) % 12);
            let to = scratch + CIRCLE + 0x10 * ((step - 1) % 12);
            let ground = e.mem.u32(info + 0x24);
            draw_line(e, this, from, to, wheel_color(ground, 0x0039_6666), tag);
            q = load_vec(e, scratch + CIRCLE_QUATERNION);
        }
    }
}

/// Draws the axle lines of `vehicle`: for every axle the points (`points` is the array of wheel
/// points) of its wheels, in wheel order, joined by lines (colour `0xff9acd32`), stopping at
/// the axle's wheel count.
fn draw_axles(e: &mut Engine, this: u32, scratch: u32, vehicle: u32, wheels: i32, points: u32) {
    let axles = e.mem.i32(e.mem.u32(vehicle + 0x1c) + 0x9c);
    if axles <= 0 {
        return;
    }
    let mut axle = 0i32;
    loop {
        let expected = i32::from(
            e.mem.i8(e
                .mem
                .u32(e.mem.u32(vehicle + 0x1c) + 0x98)
                .wrapping_add(axle as u32)),
        );
        let mut first = true;
        let mut count = 0i32;
        if wheels > 0 {
            let mut wheel = 0i32;
            let mut info_offset = 0u32;
            let mut parameter_offset = 0u32;
            let mut point = points;
            loop {
                let info = e.mem.u32(vehicle + 0x48).wrapping_add(info_offset);
                let parameters = e
                    .mem
                    .u32(e.mem.u32(vehicle + 0x1c) + 0x8c)
                    .wrapping_add(parameter_offset);
                let ground = e.mem.u32(info + 0x24);
                let tag = if ground != 0 {
                    ground.wrapping_add(0x10)
                } else {
                    0
                };
                if i32::from(e.mem.i8(parameters + 0x24)) == axle {
                    count += 1;
                    copy_vec(e, scratch + CURRENT_POINT, scratch + PREVIOUS_POINT);
                    copy_vec(e, point, scratch + CURRENT_POINT);
                    if first {
                        first = false;
                    } else {
                        draw_line(
                            e,
                            this,
                            scratch + PREVIOUS_POINT,
                            scratch + CURRENT_POINT,
                            0xff9a_cd32,
                            tag,
                        );
                    }
                    if count == expected {
                        break;
                    }
                }
                info_offset = info_offset.wrapping_add(0xe0);
                point = point.wrapping_add(0x10);
                wheel += 1;
                parameter_offset = parameter_offset.wrapping_add(0x28);
                if wheel >= wheels {
                    break;
                }
            }
        }
        axle += 1;
        if axle >= axles {
            break;
        }
    }
}

// Translated from 00d63680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleViewer::postSimulationCallback` (Xbox PDB name, engine map). `ECX` is `this`; one
/// stack word is popped (`RET 4`) but never read: `_unused_1`. Between profile records (labels `010d17e4`
/// and `010c41f8`) it draws every vehicle of the viewer (the array at `this + 4`, count at
/// `this + 8`): `0x10` bytes per wheel are taken from the memory router's stack allocator
/// (rounded up to `0x80`), every wheel is drawn (`draw_wheel`; the constants `0`, `3` and `0.5`
/// are read from `.rdata` once per vehicle) and then the axle lines (`draw_axles`); the stack top
/// is restored afterwards.
pub fn hkp_vehicle_viewer_post_simulation_callback(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let t = this.addr();
    profile_record(e, STEP_LABEL);
    let vehicle_count = e.mem.i32(t + 8);
    if vehicle_count > 0 {
        e.with_stack(0x120, |e, frame| {
            let scratch = frame.addr();
            let mut index = 0i32;
            loop {
                let vehicles = e.mem.u32(t + 4);
                let vehicle = e
                    .mem
                    .u32(vehicles.wrapping_add((index as u32).wrapping_mul(4)));
                let wheels = e.mem.i32(vehicle + 0x4c);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let top = e.mem.u32(router);
                let rounded = ((wheels as u32) << 4).wrapping_add(0x7f) & 0xffff_ff80;
                e.mem.set_u32(router, rounded.wrapping_add(top));
                if wheels > 0 {
                    let constants = [
                        e.mem.f32(ZERO_F32),
                        e.mem.f32(THREE_F32),
                        e.mem.f32(HALF_F32),
                    ];
                    for wheel in 0..wheels as u32 {
                        let point = top.wrapping_add(wheel.wrapping_mul(0x10));
                        draw_wheel(e, t, scratch, vehicle, wheel, point, constants);
                    }
                }
                draw_axles(e, t, scratch, vehicle, wheels, top);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                e.mem.set_u32(router, top);
                index += 1;
                if index >= vehicle_count {
                    break;
                }
            }
        });
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00d63c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): `world`. The viewer's constructor (the engine map has no
/// name for it): runs the `hkpWorldViewerBase` constructor (`00ddccb0`) on `this` with the world,
/// then stores the vtables (`+0` `010d1854`, `+8` `010d1834`, `+0x20` `010d1824`, `+0x28`
/// `010d1814`, `+0x2c` `010d1804`; `+0x28` and `+0x2c` first get the base tables `010cb1d4` and
/// `010ccf7c`) and clears the array at `+0x30`, `+0x34` (data and size) with the capacity word
/// `+0x38` set to `0x80000000`. Returns `this`.
pub fn fn_00d63c50(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let t = this.addr();
    e.call(WORLD_VIEWER_BASE_CONSTRUCT, &args![t, world]);
    e.mem.set_u32(t + 0x28, BASE_VTABLE_28);
    e.mem.set_u32(t + 0x2c, BASE_VTABLE_2C);
    e.mem.set_u32(t, VIEWER_VTABLE);
    e.mem.set_u32(t + 8, VIEWER_VTABLE_8);
    e.mem.set_u32(t + 0x20, VIEWER_VTABLE_20);
    e.mem.set_u32(t + 0x28, VIEWER_VTABLE_28);
    e.mem.set_u32(t + 0x2c, VIEWER_VTABLE_2C);
    e.mem.set_u32(t + 0x30, 0);
    e.mem.set_u32(t + 0x34, 0);
    e.mem.set_u32(t + 0x38, 0x8000_0000);
    this
}

// Translated from 00d63cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's destructor (fastcall, no stack word; the engine map has no name for it): puts the
/// vtables back (`+0`, `+8`, `+0x20`, `+0x28`, `+0x2c`), then for every entity of the world at
/// `+0x24` (the array at world `+0x6c`, count at `+0x70`; both re-read each time) runs
/// `00d63480` with the sub-object at `+0x20`, frees the array at `+0x30` through the
/// allocator (slot `0x10`: the data and `4 * capacity` bytes) unless its capacity word is
/// negative, clears it (`+0x30` zero, `+0x34` zero, `+0x38` `0x80000000`), puts the base tables
/// `010ccf7c` and `010cb1d4` at `+0x2c` and `+0x28` and ends with a tail jump to the
/// `hkpWorldViewerBase` destructor (`00ddcc60`, `ECX` the object).
pub fn fn_00d63cb0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, VIEWER_VTABLE);
    e.mem.set_u32(t + 8, VIEWER_VTABLE_8);
    e.mem.set_u32(t + 0x20, VIEWER_VTABLE_20);
    e.mem.set_u32(t + 0x28, VIEWER_VTABLE_28);
    e.mem.set_u32(t + 0x2c, VIEWER_VTABLE_2C);
    let world = e.mem.u32(t + 0x24);
    if world != 0 && e.mem.i32(world + 0x70) > 0 {
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(t + 0x24);
            let entities = e.mem.u32(world + 0x6c);
            let entity = e.mem.u32(entities.wrapping_add(index.wrapping_mul(4)));
            fn_00d63480(e, Ptr::new(t + 0x20), entity);
            let world = e.mem.u32(t + 0x24);
            index += 1;
            if index as i32 >= e.mem.i32(world + 0x70) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(t + 0x38);
    e.mem.set_u32(t + 0x34, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(t + 0x30);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(t + 0x30, 0);
    e.mem.set_u32(t + 0x38, 0x8000_0000);
    e.mem.set_u32(t + 0x2c, BASE_VTABLE_2C);
    e.mem.set_u32(t + 0x28, BASE_VTABLE_28);
    e.call(WORLD_VIEWER_BASE_DESTRUCT, &args![t]);
}

// Translated from 00d63da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleViewer::registerViewer` (Xbox PDB name, engine map): registers the process
/// `"Vehicles"` (`010d17c4`) with the creator `00d63d60` in the process factory (the global
/// `01269cd0`, `ECX`; `00dd9380`) and keeps the result in the global `01268fcc`.
pub fn hkp_vehicle_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let process = e
        .call(
            REGISTER_PROCESS,
            &args![factory, VEHICLES_NAME, VIEWER_CREATOR],
        )
        .u32();
    e.set_global(PROCESS_ID, process);
}

// Translated from 00d63d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x3c bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d63c50` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d63da6.
pub fn fn_00d63d60(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x3c]).u32();
    e.mem.set_u16(block + 4, 0x3c);
    let object = e.call(0x00d6_3c50, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_3480, fn_00d63480(Ptr, u32)),
        entry!(
            0x00d6_3680,
            hkp_vehicle_viewer_post_simulation_callback(Ptr, u32)
        ),
        entry!(0x00d6_3c50, fn_00d63c50(Ptr, u32) -> Ptr),
        entry!(0x00d6_3cb0, fn_00d63cb0(Ptr)),
        entry!(0x00d6_3da0, hkp_vehicle_viewer_register_viewer()),
        entry!(0x00d6_3d60, fn_00d63d60(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{load_bits, PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base of the doubles that stand in for vtable slots (`SLOT_BASE + slot offset`).
    const SLOT_BASE: u32 = 0x0900_0000;

    /// An engine with the exe's data range mapped and a profiling stream with no room.
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let stream = e.mem.alloc(0x10);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    /// Doubles that only log their arguments (and return zero) for each address.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A memory router whose stack allocator top is `top` and whose allocator object has a
    /// logging slot `0x10` (the free).
    fn memory(e: &mut Engine, log: &Log, top: u32) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, top);
        let allocator = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(allocator, table);
        e.mem.set_u32(table + 0x10, SLOT_BASE + 0x10);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_callees(e, log, &[SLOT_BASE + 0x10]);
        (router, allocator)
    }

    #[test]
    fn fn_00d63480_passes_the_listener_pointers_for_the_entity() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[ENTITY_CALL_A, ENTITY_CALL_B]);
        e.call(0x00d6_3480, &args![0x1020u32, 0x4444u32]);
        assert_eq!(calls_to(&log, ENTITY_CALL_A), vec![vec![0x4444, 0x1028]]);
        assert_eq!(calls_to(&log, ENTITY_CALL_B), vec![vec![0x4444, 0x102c]]);
        log.borrow_mut().clear();
        e.call(0x00d6_3480, &args![0x20u32, 0x5555u32]);
        assert_eq!(calls_to(&log, ENTITY_CALL_A), vec![vec![0x5555, 0]]);
        assert_eq!(calls_to(&log, ENTITY_CALL_B), vec![vec![0x5555, 0]]);
    }

    /// The viewer test's world: one vehicle with two wheels on one axle (the second wheel has
    /// a contact object). Returns (viewer sub-object, wheel records, stack top, router).
    fn viewer_fixture(e: &mut Engine, log: &Log) -> (u32, u32, u32, u32) {
        e.set_global(PROCESS_ID, 7u32);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f32(CIRCLE_ANGLE, 0.5);
        store_vec(e, NEGATIVE_HALVES, [-0.5; 4]);
        let block = e.mem.alloc(0x80);
        let this = block + 0x20;
        let display = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(display, table);
        e.mem.set_u32(table + 0x24, SLOT_BASE + 0x24);
        e.mem.set_u32(this - 0x14, display);
        let sink = Rc::clone(log);
        e.register_double(SLOT_BASE + 0x24, move |e, a| {
            let mut logged = a.to_vec();
            for pointer in [a[1], a[2]] {
                logged.extend(load_bits(e, pointer));
            }
            sink.borrow_mut().push((SLOT_BASE + 0x24, logged));
            Ret::default()
        });
        let vehicles = e.mem.alloc(8);
        let vehicle = e.mem.alloc(0x80);
        e.mem.set_u32(vehicles, vehicle);
        e.mem.set_u32(this + 4, vehicles);
        e.mem.set_u32(this + 8, 1);
        e.mem.set_u32(vehicle + 0x4c, 2);
        let infos = e.mem.alloc(0x400);
        e.mem.set_u32(vehicle + 0x48, infos);
        let data = e.mem.alloc(0xc0);
        e.mem.set_u32(vehicle + 0x1c, data);
        let parameters = e.mem.alloc(0x80);
        e.mem.set_u32(data + 0x8c, parameters);
        let axles = e.mem.alloc(8);
        e.mem.set_u8(axles, 2);
        e.mem.set_u32(data + 0x98, axles);
        e.mem.set_u32(data + 0x9c, 1);
        for wheel in 0..2u32 {
            let info = infos + 0xe0 * wheel;
            store_vec(e, info, [1.0 + wheel as f32, 2.0, 3.0, 1.0]);
            store_vec(e, info + 0x80, [0.0, -1.0, 0.0, 0.0]);
            e.mem.set_f32(info + 0xc4, 0.5);
            e.mem.set_f32(parameters + 0x28 * wheel, 2.0);
        }
        e.mem.set_u32(infos + 0xe0 + 0x24, 0x9000);
        let sink = Rc::clone(log);
        e.register_double(SET_AXIS_ANGLE, move |e, a| {
            sink.borrow_mut().push((SET_AXIS_ANGLE, a.to_vec()));
            let q = if a[2] == 0.5f32.to_bits() {
                [0.0, 0.0, 1.0, 0.0]
            } else {
                [0.0, 0.0, 0.0, 1.0]
            };
            store_vec(e, a[0], q);
            Ret::default()
        });
        let top = e.mem.alloc(0x100);
        let (router, _) = memory(e, log, top);
        (this, infos, top, router)
    }

    #[test]
    fn post_simulation_callback_draws_the_wheels_and_the_axle() {
        let (mut e, log) = setup();
        let (this, infos, top, router) = viewer_fixture(&mut e, &log);
        e.call(0x00d6_3680, &args![this, 0u32]);
        let lines = calls_to(&log, SLOT_BASE + 0x24);
        assert_eq!(lines.len(), 29);
        let display = e.mem.u32(this - 0x14);
        let bits = |v: [f32; 4]| v.map(f32::to_bits);
        // Wheel 0, no contact object: the record line, the cross line, then the circle.
        assert_eq!(
            lines[0][..6],
            [display, infos, infos + 0x50, 0xffbc_8f8f, 0, 7]
        );
        assert_eq!(lines[1][3..6], [0xffbc_8f8f, 0, 7]);
        let start: Vec<f32> = lines[1][6..10].iter().map(|b| f32::from_bits(*b)).collect();
        let end: Vec<f32> = lines[1][10..14]
            .iter()
            .map(|b| f32::from_bits(*b))
            .collect();
        assert!((start[1] - 2.0).abs() < 1e-3 && (end[1] - 6.0).abs() < 1e-3);
        assert_eq!(lines[2][3..6], [0xffbc_8f8f, 0, 7]);
        assert_eq!(lines[2][6..10], bits([1.0, 2.0, 3.0, 1.0]));
        assert_eq!(lines[2][10..14], bits([1.0, 6.0, 3.0, 1.0]));
        assert_eq!(lines[3][6..10], bits([1.0, 6.0, 3.0, 1.0]));
        assert_eq!(lines[13][6..10], bits([1.0, 6.0, 3.0, 1.0]));
        assert_eq!(lines[13][10..14], bits([1.0, 2.0, 3.0, 1.0]));
        // Wheel 1 has a contact object: its colours and tag differ.
        assert_eq!(
            lines[14][..6],
            [
                display,
                infos + 0xe0,
                infos + 0xe0 + 0x50,
                0xffff_0000,
                0x9010,
                7
            ]
        );
        assert_eq!(lines[15][3..5], [0xfff5_f5f5, 0x9010]);
        assert_eq!(lines[16][3..5], [0xfff5_f5f5, 0x9010]);
        // The axle line joins the two wheel points.
        let axle = &lines[28];
        assert_eq!(axle[3..6], [0xff9a_cd32, 0x9010, 7]);
        assert_eq!(axle[6..10], bits([1.0, 4.0, 3.0, 1.0]));
        assert_eq!(axle[10..14], bits([2.0, 4.0, 3.0, 1.0]));
        assert_eq!(calls_to(&log, SET_AXIS_ANGLE).len(), 4);
        assert_eq!(
            calls_to(&log, SET_AXIS_ANGLE)[0][1..],
            [infos + 0xa0, (-0.5f32).to_bits()]
        );
        assert_eq!(e.mem.u32(router), top);
    }

    #[test]
    fn fn_00d63c50_stores_the_vtables_and_clears_the_array() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[WORLD_VIEWER_BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 0x1111);
        let got = e.call(0x00d6_3c50, &args![this, 0x7777u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls_to(&log, WORLD_VIEWER_BASE_CONSTRUCT),
            vec![vec![this, 0x7777]]
        );
        assert_eq!(e.mem.u32(this), 0x010d_1854);
        assert_eq!(e.mem.u32(this + 8), 0x010d_1834);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_1824);
        assert_eq!(e.mem.u32(this + 0x28), 0x010d_1814);
        assert_eq!(e.mem.u32(this + 0x2c), 0x010d_1804);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn fn_00d63cb0_releases_the_entities_and_the_array() {
        let (mut e, log) = setup();
        let (_, allocator) = memory(&mut e, &log, 0);
        log_callees(
            &mut e,
            &log,
            &[ENTITY_CALL_A, ENTITY_CALL_B, WORLD_VIEWER_BASE_DESTRUCT],
        );
        let this = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x80);
        let entities = e.mem.alloc(8);
        e.mem.set_u32(entities, 0xe1);
        e.mem.set_u32(entities + 4, 0xe2);
        e.mem.set_u32(world + 0x6c, entities);
        e.mem.set_u32(world + 0x70, 2);
        e.mem.set_u32(this + 0x24, world);
        e.mem.set_u32(this + 0x30, 0x5000);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 4);
        e.call(0x00d6_3cb0, &args![this]);
        assert_eq!(
            calls_to(&log, ENTITY_CALL_A),
            vec![vec![0xe1, this + 0x28], vec![0xe2, this + 0x28]]
        );
        assert_eq!(
            calls_to(&log, ENTITY_CALL_B),
            vec![vec![0xe1, this + 0x2c], vec![0xe2, this + 0x2c]]
        );
        assert_eq!(
            calls_to(&log, SLOT_BASE + 0x10),
            vec![vec![allocator, 0x5000, 16]]
        );
        assert_eq!(calls_to(&log, WORLD_VIEWER_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x010d_1854);
        assert_eq!(e.mem.u32(this + 0x28), 0x010c_b1d4);
        assert_eq!(e.mem.u32(this + 0x2c), 0x010c_cf7c);
        log.borrow_mut().clear();
        e.call(0x00d6_3cb0, &args![this]);
        assert!(calls_to(&log, SLOT_BASE + 0x10).is_empty());
        e.mem.set_u32(this + 0x24, 0);
        log.borrow_mut().clear();
        e.call(0x00d6_3cb0, &args![this]);
        assert!(calls_to(&log, ENTITY_CALL_A).is_empty());
        assert_eq!(calls_to(&log, WORLD_VIEWER_BASE_DESTRUCT).len(), 1);
    }

    #[test]
    fn register_viewer_keeps_the_process_id() {
        let (mut e, log) = setup();
        e.set_global(PROCESS_FACTORY, 0x1234u32);
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            sink.borrow_mut().push((REGISTER_PROCESS, a.to_vec()));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        e.call(0x00d6_3da0, &args![]);
        assert_eq!(
            calls_to(&log, REGISTER_PROCESS),
            vec![vec![0x1234, 0x010d_17c4, 0x00d6_3d60]]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x77);
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
    fn fn_00d63d60_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_3c50, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_3d60, &args![0x1234u32]).u32();
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
    fn fn_00d63d60_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_3c50, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_3d60, &args![0x1234u32]).u32(), 0);
    }
}
