//! `Havok SDK/hkpworldraycaster.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word +0 is the top of its stack.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtRayCstCached"`, `"TtRayCastGroup"`, `"TtRayCastFSP"` and the end record.
const LABEL_CAST_RAY: u32 = 0x010c_9268;
const LABEL_CAST_RAY_GROUP: u32 = 0x010c_9278;
const LABEL_CAST_RAYS_FROM_SINGLE_POINT: u32 = 0x010c_9288;
const LABEL_END: u32 = 0x010c_41f8;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// `MINPS dst, src` per lane: `dst` when it is less than `src`, else `src` (so a NaN in either
/// gives `src`).
fn min_ps(dst: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dst[lane] < src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// `MAXPS dst, src` per lane: `dst` when it is greater than `src`, else `src`.
fn max_ps(dst: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dst[lane] > src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// The stand-in for `RDTSC`'s low word: the host clock.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream (found with
/// `TlsGetValue` of the index at `0126835c`) while its cursor (+4) is below its limit (+0xC).
/// The engine has no model of `RDTSC`, so the host clock stands in for the time stamp.
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// A ray point in the local space of the collidable whose transform (three columns, then the
/// translation at +0x30) is at `transform`: the point minus the translation, then
/// `(y * Y + x * X) + z * Z` per lane with the `SHUFPS` selection the code uses
/// (`X = [c0.x, c1.x, c2.x, c2.y]`, `Y = [c0.y, c1.y, c2.y, c2.w]`,
/// `Z = [c0.z, c1.z, c2.z, c2.w]`).
fn to_local_space(e: &Engine, transform: u32, point: Vec4) -> Vec4 {
    let c0 = load_vec(e, transform);
    let c1 = load_vec(e, transform + 0x10);
    let c2 = load_vec(e, transform + 0x20);
    let translation = load_vec(e, transform + 0x30);
    let d: Vec4 = std::array::from_fn(|lane| point[lane] - translation[lane]);
    let x_lane = [c0[0], c1[0], c2[0], c2[1]];
    let y_lane = [c0[1], c1[1], c2[1], c2[3]];
    let z_lane = [c0[2], c1[2], c2[2], c2[3]];
    std::array::from_fn(|lane| (d[1] * y_lane[lane] + d[0] * x_lane[lane]) + z_lane[lane] * d[2])
}

/// Stores the ray caster's input pointer (+4), the filter interface (+8, the filter plus 0x14
/// or 0), the results base and stride (+0xC, +0x10) and, when the input's byte at +0x20 is
/// set, the filter plus 0x10 (or 0) at +0x44, else 0.
fn set_up_ray_caster(
    e: &mut Engine,
    caster: u32,
    input: u32,
    filter: u32,
    results: u32,
    stride: u32,
) {
    e.mem.set_u32(caster + 4, input);
    e.mem
        .set_u32(caster + 8, if filter != 0 { filter + 0x14 } else { 0 });
    e.mem.set_u32(caster + 0xc, results);
    e.mem.set_u32(caster + 0x10, stride);
    let shape_filter = if e.mem.u8(input + 0x20) != 0 && filter != 0 {
        filter + 0x10
    } else {
        0
    };
    e.mem.set_u32(caster + 0x44, shape_filter);
}

/// Builds the 0x20-byte cast input on the stack: a copy of the input's first vector, then
/// `count`, the pointer to the input's second vector (input + 0x10), `stride` and `extra`;
/// calls the world's virtual function at +0x5C on `world_like` with (the input, the caster, 0).
fn run_cast(
    e: &mut Engine,
    caster: u32,
    world_like: u32,
    input: u32,
    count: u32,
    stride: u32,
    extra: u32,
) {
    e.with_stack(0x20, |e, cast| {
        let at = cast.addr();
        let first = load_vec(e, input);
        store_vec(e, at, first);
        e.mem.set_u32(at + 0x10, count);
        e.mem.set_u32(at + 0x14, input + 0x10);
        e.mem.set_u32(at + 0x18, stride);
        e.mem.set_u32(at + 0x1c, extra);
        e.vcall(world_like, 0x5c, &args![cast, caster, 0u32]);
    });
}

// Translated from 00cbf710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldRayCaster::addBroadPhaseHandle` (Xbox PDB), thiscall `(handle, index)`, `RET 8`;
/// the answer is in `ST0`. The collidable is `handle` plus the signed byte at `handle + 5`;
/// its first word is the shape. The result record for ray `index` is at `[this + 0xC] +
/// [this + 0x10] * index`; its float at +4 is the answer in every case. Without a shape, or
/// when the filter interface (the object at `this + 8`, virtual function at +4 called with
/// (a byte slot, the ray at `[this + 4] + 0x30 * index`, the collidable)) answers a zero
/// byte, nothing else happens. Otherwise the first point of the input array (`[this + 4]`) and the
/// ray's second point (+0x10) are moved into the collidable's local space (its motion
/// state, the pointer at collidable +8) and stored at this +0x20 and +0x30, the ray's word at
/// +0x24 is copied to this +0x40, and the shape's virtual function at +0x24 is called with
/// (this + 0x20, the collidable, the result record).
pub fn hkp_world_ray_caster_add_broad_phase_handle(
    e: &mut Engine,
    this: Ptr,
    handle: Ptr,
    index: u32,
) -> f32 {
    let me = this.addr();
    let handle = handle.addr();
    let offset = i32::from(e.mem.i8(handle + 5));
    let collidable = handle.wrapping_add(offset as u32);
    let shape = e.mem.u32(collidable);
    let result = e
        .mem
        .u32(me + 0x10)
        .wrapping_mul(index)
        .wrapping_add(e.mem.u32(me + 0xc));
    if shape == 0 {
        return x87_quiet(e.mem.f32(result + 4));
    }
    let ray = index.wrapping_mul(0x30).wrapping_add(e.mem.u32(me + 4));
    let filter = e.mem.u32(me + 8);
    let accepted = e.with_stack(4, |e, slot| {
        let answer = e.vcall(filter, 4, &args![slot, ray, collidable]).u32();
        e.mem.u8(answer) != 0
    });
    if !accepted {
        return x87_quiet(e.mem.f32(result + 4));
    }
    let transform = e.mem.u32(collidable + 8);
    let inputs = e.mem.u32(me + 4);
    let from = to_local_space(e, transform, load_vec(e, inputs));
    store_vec(e, me + 0x20, from);
    let to = to_local_space(e, transform, load_vec(e, ray + 0x10));
    store_vec(e, me + 0x30, to);
    let filter_info = e.mem.u32(ray + 0x24);
    e.mem.set_u32(me + 0x40, filter_info);
    e.vcall(shape, 0x24, &args![me + 0x20, collidable, result]);
    x87_quiet(e.mem.f32(result + 4))
}

// Translated from 00cbf860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldRayCaster::castRay` (Xbox PDB), thiscall `(world, input, filter, buffer, results)`,
/// `RET 0x14`: writes a `"TtRayCstCached"` profiling record, stores the input (+4), the
/// filter interface (+8: the filter plus 0x14, or 0), the results base (+0xC) and a stride of 0
/// (+0x10), sets +0x44 (the filter plus 0x10, or 0, when the input's byte at +0x20 is set;
/// else 0), then calls the world's virtual function at +0x5C with (a 0x20-byte stack input:
/// the first vector of the input, 1, a pointer to its second vector, 0x10, `buffer`; this; 0)
/// and writes the end profiling record.
pub fn hkp_world_ray_caster_cast_ray(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    input: Ptr,
    filter: Ptr,
    buffer: u32,
    results: u32,
) {
    profile_record(e, LABEL_CAST_RAY);
    set_up_ray_caster(e, this.addr(), input.addr(), filter.addr(), results, 0);
    run_cast(e, this.addr(), world.addr(), input.addr(), 1, 0x10, buffer);
    profile_record(e, LABEL_END);
}

// Translated from 00cbf950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldRayCaster::castRayGroup` (Xbox PDB), thiscall `(broad phase, rays, count, filter,
/// results, stride)`, `RET 0x18`: writes a `"TtRayCastGroup"` profiling record; the box that
/// bounds all the rays (0x30 bytes each: two vectors at +0 and +0x10; `MINPS`/`MAXPS` over the
/// pairs) goes on the stack; the broad phase's virtual function at +0x60 answers a buffer
/// size, the memory router's stack is bumped by it rounded up to 0x80, the function at +0x68
/// gets (the box, the buffer), `castRay` runs once per ray (zero times for a count of 0 or
/// less) with (broad phase, ray, filter, buffer, results) and the results advance by `stride`
/// each time; the router's top is restored and the end record written.
pub fn hkp_world_ray_caster_cast_ray_group(
    e: &mut Engine,
    this: Ptr,
    broad_phase: Ptr,
    rays: Ptr,
    count: i32,
    filter: Ptr,
    results: u32,
    stride: u32,
) {
    profile_record(e, LABEL_CAST_RAY_GROUP);
    let first = load_vec(e, rays.addr());
    let second = load_vec(e, rays.addr() + 0x10);
    let mut low = min_ps(first, second);
    let mut high = max_ps(first, second);
    // `ADD ECX,-2` then a sign test: the later rays are only folded in for a count of 2 or more.
    let mut remaining = count.wrapping_sub(2);
    let mut cursor = rays.addr() + 0x30;
    if remaining >= 0 {
        loop {
            let to = load_vec(e, cursor + 0x10);
            let from = load_vec(e, cursor);
            cursor += 0x30;
            remaining -= 1;
            low = min_ps(low, to);
            high = max_ps(high, to);
            low = min_ps(low, from);
            high = max_ps(high, from);
            if remaining < 0 {
                break;
            }
        }
    }
    e.with_stack(0x20, |e, bounds| {
        store_vec(e, bounds.addr(), low);
        store_vec(e, bounds.addr() + 0x10, high);
        let size = e.vcall(broad_phase.addr(), 0x60, &args![]).u32();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let buffer = e.mem.u32(router);
        let rounded = size.wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, rounded.wrapping_add(buffer));
        e.vcall(broad_phase.addr(), 0x68, &args![bounds, buffer]);
        let mut results = results;
        let mut ray = rays;
        let mut left = count.wrapping_sub(1);
        while left >= 0 {
            hkp_world_ray_caster_cast_ray(e, this, broad_phase, ray, filter, buffer, results);
            results = results.wrapping_add(stride);
            ray = Ptr::new(ray.addr() + 0x30);
            left -= 1;
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, buffer);
    });
    profile_record(e, LABEL_END);
}

// Translated from 00cbfaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldRayCaster::castRaysFromSinglePoint` (Xbox PDB), thiscall `(world, input, count,
/// filter, buffer, results, stride)`, `RET 0x1C`: writes a `"TtRayCastFSP"` profiling record;
/// like `castRay` stores the input (+4), the filter interface (+8), `results` (+0xC) and
/// `stride` (+0x10) and sets +0x44, then calls the world's virtual function at +0x5C with (a
/// 0x20-byte stack input: the first vector of the input, `count`, a pointer to its second
/// vector, 0x30, `buffer`; this; 0) and writes the end profiling record.
pub fn hkp_world_ray_caster_cast_rays_from_single_point(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    input: Ptr,
    count: u32,
    filter: Ptr,
    buffer: u32,
    results: u32,
    stride: u32,
) {
    profile_record(e, LABEL_CAST_RAYS_FROM_SINGLE_POINT);
    set_up_ray_caster(e, this.addr(), input.addr(), filter.addr(), results, stride);
    run_cast(
        e,
        this.addr(),
        world.addr(),
        input.addr(),
        count,
        0x30,
        buffer,
    );
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_f710, hkp_world_ray_caster_add_broad_phase_handle(Ptr, Ptr, u32) -> f32),
        entry!(
            0x00cb_f860,
            hkp_world_ray_caster_cast_ray(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00cb_f950,
            hkp_world_ray_caster_cast_ray_group(Ptr, Ptr, Ptr, i32, Ptr, u32, u32)
        ),
        entry!(
            0x00cb_faa0,
            hkp_world_ray_caster_cast_rays_from_single_point(
                Ptr, Ptr, Ptr, u32, Ptr, u32, u32, u32
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The monitor stream (cursor at `+4`, limit at `+0xc`) and the TLS slot.
    fn monitor_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0xf0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    /// The labels of the profiling records written since the stream was made.
    fn record_labels(e: &Engine, stream: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        (stream + 0x40..end)
            .step_by(12)
            .map(|at| e.mem.u32(at))
            .collect()
    }

    /// An object with the given virtual function slots (byte offsets) routed to doubles that
    /// log their arguments with the slot's address.
    fn object_with(e: &mut Engine, log: &Log, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for &(slot, target) in slots {
            e.mem.set_u32(table + slot, target);
            let log = Rc::clone(log);
            e.register_double(target, move |_, a| {
                log.borrow_mut().push((target, a.to_vec()));
                Ret::default()
            });
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    fn set_vec(e: &mut Engine, at: u32, v: [f32; 4]) {
        store_vec(e, at, v);
    }

    #[test]
    fn min_and_max_pick_the_second_operand_when_the_compare_fails() {
        let a = [1.0, f32::NAN, 3.0, -0.0];
        let b = [2.0, 5.0, f32::NAN, 0.0];
        let low = min_ps(a, b);
        assert_eq!(low[0], 1.0);
        assert_eq!(low[1], 5.0);
        assert!(low[2].is_nan());
        assert_eq!(low[3].to_bits(), 0.0f32.to_bits());
        let high = max_ps(a, b);
        assert_eq!(high[0], 2.0);
        assert_eq!(high[1], 5.0);
        assert!(high[2].is_nan());
        assert_eq!(high[3].to_bits(), 0.0f32.to_bits());
    }

    #[test]
    fn handle_without_a_shape_or_rejected_by_the_filter_only_answers_the_result() {
        let mut e = engine();
        let log: Log = Rc::default();
        let (filter_hook, shape_hook) = (0x0900_0001, 0x0900_0002);
        let filter = object_with(&mut e, &log, &[(4, filter_hook)]);
        let shape = object_with(&mut e, &log, &[(0x24, shape_hook)]);
        // The filter answers through the byte slot it is given.
        e.register_double(filter_hook, {
            let log = Rc::clone(&log);
            move |e, a| {
                log.borrow_mut().push((filter_hook, a.to_vec()));
                let value = u8::from(e.mem.u32(a[2] + 0x24) == 1);
                e.mem.set_u8(a[1], value);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            }
        });
        let this = e.mem.alloc(0x80);
        let rays = e.mem.alloc(0x30 * 3);
        let results = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, rays);
        e.mem.set_u32(this + 8, filter);
        e.mem.set_u32(this + 0xc, results);
        e.mem.set_u32(this + 0x10, 0x10);
        e.mem.set_f32(results + 0x10 + 4, 0.25);
        // The collidable sits 0x10 before its handle.
        let block = e.mem.alloc(0x100);
        let collidable = block + 0x20;
        let handle = collidable + 0x10;
        e.mem.set_u8(handle + 5, 0xf0);
        // No shape: the answer is the result's float and nothing is called.
        let got = e.call(0x00cb_f710, &args![this, handle, 1u32]).f32();
        assert_eq!(got, 0.25);
        assert!(log.borrow().is_empty());
        // A shape, but ray 1 is rejected by the filter (its word at +0x24 is 0).
        e.mem.set_u32(collidable, shape);
        let got = e.call(0x00cb_f710, &args![this, handle, 1u32]).f32();
        assert_eq!(got, 0.25);
        let asked = calls(&log, filter_hook);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], filter);
        assert_eq!(asked[0][2..], [rays + 0x30, collidable]);
        assert!(calls(&log, shape_hook).is_empty());
    }

    #[test]
    fn handle_accepted_by_the_filter_casts_in_the_local_space_of_the_collidable() {
        let mut e = engine();
        let log: Log = Rc::default();
        let (filter_hook, shape_hook) = (0x0900_0001, 0x0900_0002);
        let filter = object_with(&mut e, &log, &[(4, filter_hook)]);
        let shape = object_with(&mut e, &log, &[(0x24, shape_hook)]);
        e.register_double(filter_hook, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // The shape writes the hit fraction into the result record.
        e.register_double(shape_hook, {
            let log = Rc::clone(&log);
            move |e, a| {
                log.borrow_mut().push((shape_hook, a.to_vec()));
                e.mem.set_f32(a[3] + 4, 0.5);
                Ret::default()
            }
        });
        let this = e.mem.alloc(0x80);
        let rays = e.mem.alloc(0x30 * 3);
        let results = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, rays);
        e.mem.set_u32(this + 8, filter);
        e.mem.set_u32(this + 0xc, results);
        e.mem.set_u32(this + 0x10, 0x10);
        set_vec(&mut e, rays, [2.0, 3.0, 4.0, 5.0]);
        set_vec(&mut e, rays + 0x30 + 0x10, [3.0, 3.0, 3.0, 1.0]);
        e.mem.set_u32(rays + 0x30 + 0x24, 0xabcd);
        let block = e.mem.alloc(0x100);
        let collidable = block + 0x20;
        let handle = collidable + 0x10;
        e.mem.set_u8(handle + 5, 0xf0);
        e.mem.set_u32(collidable, shape);
        let transform = e.mem.alloc(0x40);
        e.mem.set_u32(collidable + 8, transform);
        set_vec(&mut e, transform, [1.0, 2.0, 3.0, 0.0]);
        set_vec(&mut e, transform + 0x10, [4.0, 5.0, 6.0, 0.0]);
        set_vec(&mut e, transform + 0x20, [7.0, 8.0, 9.0, 0.0]);
        set_vec(&mut e, transform + 0x30, [1.0, 1.0, 1.0, 0.0]);
        let got = e.call(0x00cb_f710, &args![this, handle, 1u32]).f32();
        assert_eq!(got, 0.5);
        // Point (2, 3, 4, 5) minus the translation is (1, 2, 3, 4).
        assert_eq!(load_vec(&e, this + 0x20), [14.0, 32.0, 50.0, 8.0]);
        // Point (3, 3, 3, 1) minus the translation is (2, 2, 2, 1):
        // lane 0: (2 * 2 + 2 * 1) + 3 * 2, lane 1: (2 * 5 + 2 * 4) + 6 * 2,
        // lane 2: (2 * 8 + 2 * 7) + 9 * 2, lane 3: (2 * 0 + 2 * 8) + 0 * 2.
        assert_eq!(load_vec(&e, this + 0x30), [12.0, 30.0, 48.0, 16.0]);
        assert_eq!(e.mem.u32(this + 0x40), 0xabcd);
        assert_eq!(
            calls(&log, shape_hook),
            vec![vec![shape, this + 0x20, collidable, results + 0x10]]
        );
    }

    #[test]
    fn cast_ray_records_the_set_up_and_runs_the_world_function() {
        let mut e = engine();
        let log: Log = Rc::default();
        let stream = monitor_stream(&mut e);
        let hook = 0x0900_0003;
        let world = object_with(&mut e, &log, &[(0x5c, hook)]);
        let this = e.mem.alloc(0x80);
        let input = e.mem.alloc(0x40);
        set_vec(&mut e, input, [1.0, 2.0, 3.0, 4.0]);
        set_vec(&mut e, input + 0x10, [5.0, 6.0, 7.0, 8.0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(hook, move |e, a| {
            let words: Vec<u32> = (0..8).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            sink.borrow_mut().push((a.to_vec(), words));
            Ret::default()
        });
        // A filter and the input's byte at +0x20 set.
        e.mem.set_u8(input + 0x20, 1);
        e.call(
            0x00cb_f860,
            &args![this, world, input, 0x9000u32, 0x1234u32, 0x5678u32],
        );
        assert_eq!(e.mem.u32(this + 4), input);
        assert_eq!(e.mem.u32(this + 8), 0x9014);
        assert_eq!(e.mem.u32(this + 0xc), 0x5678);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0x9010);
        // Copy the record out: the borrow must end before the hook runs again.
        let (count, (args, words)) = {
            let seen = seen.borrow();
            (seen.len(), seen[0].clone())
        };
        assert_eq!(count, 1);
        assert_eq!(args[0], world);
        assert_eq!(args[2], this);
        assert_eq!(args[3], 0);
        let expected: Vec<u32> = [1.0f32, 2.0, 3.0, 4.0]
            .iter()
            .map(|f| f.to_bits())
            .chain([1, input + 0x10, 0x10, 0x1234])
            .collect();
        assert_eq!(words, expected);
        assert_eq!(record_labels(&e, stream), vec![LABEL_CAST_RAY, LABEL_END]);
        // Without a filter: +8 and +0x44 are 0; without the input byte +0x44 is 0 as well.
        e.mem.set_u8(input + 0x20, 0);
        e.call(
            0x00cb_f860,
            &args![this, world, input, 0x9000u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(this + 0x44), 0);
        e.mem.set_u8(input + 0x20, 1);
        e.call(0x00cb_f860, &args![this, world, input, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn single_point_cast_passes_the_count_and_the_stride_through() {
        let mut e = engine();
        let log: Log = Rc::default();
        let stream = monitor_stream(&mut e);
        let hook = 0x0900_0003;
        let world = object_with(&mut e, &log, &[(0x5c, hook)]);
        let this = e.mem.alloc(0x80);
        let input = e.mem.alloc(0x40);
        set_vec(&mut e, input, [1.0, 2.0, 3.0, 4.0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(hook, move |e, a| {
            let words: Vec<u32> = (4..8).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            sink.borrow_mut().push(words);
            Ret::default()
        });
        e.call(
            0x00cb_faa0,
            &args![this, world, input, 7u32, 0x9000u32, 0x1111u32, 0x2222u32, 0x30u32],
        );
        assert_eq!(*seen.borrow(), vec![vec![7, input + 0x10, 0x30, 0x1111]]);
        assert_eq!(e.mem.u32(this + 4), input);
        assert_eq!(e.mem.u32(this + 8), 0x9014);
        assert_eq!(e.mem.u32(this + 0xc), 0x2222);
        assert_eq!(e.mem.u32(this + 0x10), 0x30);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(
            record_labels(&e, stream),
            vec![LABEL_CAST_RAYS_FROM_SINGLE_POINT, LABEL_END]
        );
        // The profile stream stops growing when it is full: the cursor is at its limit.
        let end = e.mem.u32(stream + 4);
        e.mem.set_u32(stream + 0xc, end);
        e.call(
            0x00cb_faa0,
            &args![this, world, input, 7u32, 0u32, 0x1111u32, 0x2222u32, 0x30u32],
        );
        assert_eq!(e.mem.u32(stream + 4), end);
    }

    #[test]
    fn group_cast_bounds_the_rays_and_advances_the_results() {
        let mut e = engine();
        let log: Log = Rc::default();
        let stream = monitor_stream(&mut e);
        let (size_hook, box_hook, cast_hook) = (0x0900_0004, 0x0900_0005, 0x0900_0006);
        let world = object_with(
            &mut e,
            &log,
            &[(0x60, size_hook), (0x68, box_hook), (0x5c, cast_hook)],
        );
        e.register_double(size_hook, |_, _| Ret {
            eax: 0x30,
            ..Ret::default()
        });
        let boxes = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&boxes);
        e.register_double(box_hook, move |e, a| {
            let lanes: Vec<f32> = (0..8).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            sink.borrow_mut().push((lanes, a[2]));
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        let stack = e.mem.alloc(0x400);
        e.mem.set_u32(router, stack);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x80);
        let rays = e.mem.alloc(0x30 * 3);
        let ray_data = [
            ([1.0, 5.0, 2.0, 0.0], [3.0, 4.0, 9.0, 0.0]),
            ([0.0, 6.0, 3.0, 1.0], [2.0, 2.0, 2.0, 2.0]),
            ([-1.0, 7.0, 0.0, 0.0], [4.0, 3.0, 5.0, 0.0]),
        ];
        for (i, (from, to)) in ray_data.iter().enumerate() {
            set_vec(&mut e, rays + 0x30 * i as u32, *from);
            set_vec(&mut e, rays + 0x30 * i as u32 + 0x10, *to);
        }
        let router_top_during_cast = Rc::new(RefCell::new(Vec::new()));
        let tops = Rc::clone(&router_top_during_cast);
        let results_seen = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&results_seen);
        e.register_double(cast_hook, move |e, a| {
            tops.borrow_mut().push(e.mem.u32(router));
            // The caster's input pointer and results base at the time of the call.
            seen.borrow_mut().push((
                e.mem.u32(a[2] + 4),
                e.mem.u32(a[2] + 0xc),
                e.mem.u32(a[1] + 0x1c),
            ));
            Ret::default()
        });
        e.call(
            0x00cb_f950,
            &args![this, world, rays, 3i32, 0x9000u32, 0x7000u32, 0x20u32],
        );
        let boxes = boxes.borrow();
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].0, vec![-1.0, 2.0, 0.0, 0.0, 4.0, 7.0, 9.0, 2.0]);
        // The buffer is the old top of the router's stack; during the casts it is bumped by
        // 0x30 rounded up to 0x80.
        assert_eq!(boxes[0].1, stack);
        assert_eq!(*router_top_during_cast.borrow(), vec![stack + 0x80; 3]);
        assert_eq!(e.mem.u32(router), stack);
        assert_eq!(
            *results_seen.borrow(),
            vec![
                (rays, 0x7000, stack),
                (rays + 0x30, 0x7020, stack),
                (rays + 0x60, 0x7040, stack)
            ]
        );
        assert_eq!(
            record_labels(&e, stream),
            vec![
                LABEL_CAST_RAY_GROUP,
                LABEL_CAST_RAY,
                LABEL_END,
                LABEL_CAST_RAY,
                LABEL_END,
                LABEL_CAST_RAY,
                LABEL_END,
                LABEL_END
            ]
        );
    }
}
