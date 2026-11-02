//! `Havok SDK/hkpsimpleworldraycaster.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The raycaster object keeps the state of the running cast: `+4` the input (a ray or an array of
//! rays of 0x30 bytes), `+8` the collision filter's ray filter (or null), `+0xc` the output,
//! `+0x10` / `+0x20` the ray of the collidable being tested in its local space, `+0x30` the
//! ray's filter info and `+0x34` the shape-collection filter (or null). Every cast entry point
//! brackets its work with profiling records (`profile_record`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, maxps, minps, movhlps, movlhps, profile_record, shufps, store_bits,
    store_vec, unpckhps, unpcklps, zip_with, Vec4, PROFILE_END,
};

/// The memory router (`00c85750`): its first word is the cursor of a stack-style allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The labels of the begin records of `castRay`, `castRay_ov2`, `castRayGroup` and
/// `castRaysFromSinglePoint`.
const PROFILE_BEGIN_CAST_RAY: u32 = 0x010c_c658;
const PROFILE_BEGIN_CAST_RAY_OV2: u32 = 0x010c_c668;
const PROFILE_BEGIN_CAST_RAY_GROUP: u32 = 0x010c_c678;
const PROFILE_BEGIN_CAST_RAYS_FROM_SINGLE_POINT: u32 = 0x010c_c688;

/// The ray state the entry points store (the input, the output and the two filters) and the
/// request they hand to the broad phase's `castRay` (slot `0x5c`), a record of the ray start
/// (16 bytes), the ray count, the pointer to the ray end, the stride of the rays and one more
/// word. The broad phase gets `(request, this, 0)`.
fn begin_cast(
    e: &mut Engine,
    this: Ptr,
    broad_phase: Ptr,
    input: Ptr,
    collision_filter: u32,
    output: u32,
    request: [u32; 3],
) {
    let base = this.addr();
    e.mem.set_u32(base + 4, input.addr());
    e.mem.set_u32(base + 0xc, output);
    let ray_filter = if collision_filter != 0 {
        collision_filter.wrapping_add(0x14)
    } else {
        0
    };
    e.mem.set_u32(base + 8, ray_filter);
    let collection_filter = if e.mem.u8(input.addr() + 0x20) != 0 && collision_filter != 0 {
        collision_filter.wrapping_add(0x10)
    } else {
        0
    };
    e.mem.set_u32(base + 0x34, collection_filter);
    let start = load_bits(e, input.addr());
    e.with_stack(0x20, |e, record| {
        let words = record.addr();
        store_bits(e, words, start);
        e.mem.set_u32(words + 0x10, request[0]);
        e.mem.set_u32(words + 0x14, input.addr().wrapping_add(0x10));
        e.mem.set_u32(words + 0x18, request[1]);
        e.mem.set_u32(words + 0x1c, request[2]);
        e.vcall(broad_phase.addr(), 0x5c, &args![record, this, 0u32]);
    });
}

// Translated from 00cfa890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleWorldRayCaster::addBroadPhaseHandle` (Xbox PDB), `(this, handle, index)`, `RET 8`,
/// answers a float (ST0). The collidable is `handle` plus the signed byte at `handle + 5`.
/// Nothing is tested when its shape (first word) is null. Otherwise the ray filter (`this + 8`,
/// slot 4) is asked `(result byte, ray, collidable)` with `ray = this->input + index * 0x30`
/// and must answer a pointer to a non-zero byte. Then the ray is moved into the collidable's
/// local space (the transform at `collidable + 8`: three rotation columns and a translation;
/// each end is `column . (end - translation)`): the start goes to `this + 0x10`, the end (at
/// `ray + 0x10`) to `this + 0x20`, the word at `ray + 0x24` to `this + 0x30`. The shape's
/// `castRay` (slot `0x20`, `(result byte, this + 0x10, output)`) is called; when its result
/// byte is set the output's collidable field (+0x50) is set to the collidable and the output's
/// normal (+0) is rotated back to world space. Every path answers the output's hit fraction
/// (the float at `output + 0x10`).
pub fn hkp_simple_world_ray_caster_add_broad_phase_handle(
    e: &mut Engine,
    this: Ptr,
    handle: Ptr,
    index: u32,
) -> f32 {
    let base = this.addr();
    let offset = i32::from(e.mem.i8(handle.addr() + 5));
    let collidable = handle.addr().wrapping_add(offset as u32);
    let shape = e.mem.u32(collidable);
    if shape != 0 {
        e.with_stack(4, |e, flag| {
            let ray_offset = index.wrapping_mul(0x30);
            let ray_filter = e.mem.u32(base + 8);
            let ray = e.mem.u32(base + 4).wrapping_add(ray_offset);
            let answer = e.vcall(ray_filter, 4, &args![flag, ray, collidable]).u32();
            if e.mem.u8(answer) == 0 {
                return;
            }
            let transform = e.mem.u32(collidable + 8);
            let ray = e.mem.u32(base + 4).wrapping_add(ray_offset);
            for (end, target) in [(0u32, 0x10u32), (0x10, 0x20)] {
                let local = inverse_transform_point(e, transform, load_vec(e, ray + end));
                store_vec(e, base + target, local);
            }
            let filter_info = e.mem.u32(ray + 0x24);
            e.mem.set_u32(base + 0x30, filter_info);
            let output = e.mem.u32(base + 0xc);
            e.vcall(shape, 0x20, &args![flag, base + 0x10, output]);
            if e.mem.u8(flag.addr()) != 0 {
                let output = e.mem.u32(base + 0xc);
                e.mem.set_u32(output + 0x50, collidable);
                let output = e.mem.u32(base + 0xc);
                let normal = load_vec(e, output);
                let rotated = rotate_vector(e, transform, normal);
                store_vec(e, output, rotated);
            }
        });
    }
    let output = e.mem.u32(base + 0xc);
    x87_quiet(e.mem.f32(output + 0x10))
}

/// The rotation columns and the translation of the transform at `transform`.
fn transform_parts(e: &Engine, transform: u32) -> ([Vec4; 3], Vec4) {
    (
        [
            load_vec(e, transform),
            load_vec(e, transform + 0x10),
            load_vec(e, transform + 0x20),
        ],
        load_vec(e, transform + 0x30),
    )
}

/// `rotation^T * (point - translation)` the way the exe computes it: the columns are shuffled
/// into rows (`UNPCKLPS`, `UNPCKHPS`, `MOVLHPS`, `MOVHLPS`, `SHUFPS`) and the result is
/// `y * row_y + x * row_x + z * row_z` over all four lanes.
fn inverse_transform_point(e: &Engine, transform: u32, point: Vec4) -> Vec4 {
    let (columns, translation) = transform_parts(e, transform);
    let relative = zip_with(point, translation, |point, translation| point - translation);
    let [column_0, column_1, column_2] = columns;
    let low = unpcklps(column_0, column_1);
    let high = unpckhps(column_0, column_1);
    let x_row = movlhps(low, column_2);
    let y_row = shufps(movhlps(x_row, low), column_2, 0xd4);
    let z_row = shufps(high, column_2, 0xe4);
    let scale = |row: Vec4, lane: u8| {
        let factor = shufps(relative, relative, lane * 0x55)[0];
        row.map(|value| factor * value)
    };
    let y_terms = scale(y_row, 1);
    let x_terms = scale(x_row, 0);
    let z_terms = scale(z_row, 2);
    let partial = zip_with(y_terms, x_terms, |y, x| y + x);
    zip_with(partial, z_terms, |partial, z| partial + z)
}

/// `x * column_0 + y * column_1 + z * column_2` of the transform at `transform`.
fn rotate_vector(e: &Engine, transform: u32, vector: Vec4) -> Vec4 {
    let (columns, _) = transform_parts(e, transform);
    let term = |column: Vec4, lane: usize| column.map(|value| vector[lane] * value);
    let partial = zip_with(term(columns[0], 0), term(columns[1], 1), |x, y| x + y);
    zip_with(partial, term(columns[2], 2), |partial, z| partial + z)
}

// Translated from 00cfaa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleWorldRayCaster::castRay` (engine map), `(this, broad_phase, input, collision_filter,
/// output)`, `RET 0x10`. Brackets the cast with profiling records (`010cc658` and the shared
/// end label), stores the input, the output and the filter pointers (`collision_filter + 0x14`
/// as the ray filter, `+0x10` as the shape-collection filter when the input's byte at +0x20 is
/// set; null when `collision_filter` is null) and calls the broad phase's slot `0x5c` with
/// `(request, this, 0)`, the request being `{ the 16 bytes of the input's ray start, 1, input +
/// 0x10, 0x10, 0 }`.
pub fn hkp_simple_world_ray_caster_cast_ray(
    e: &mut Engine,
    this: Ptr,
    broad_phase: Ptr,
    input: Ptr,
    collision_filter: u32,
    output: u32,
) {
    profile_record(e, PROFILE_BEGIN_CAST_RAY);
    begin_cast(
        e,
        this,
        broad_phase,
        input,
        collision_filter,
        output,
        [1, 0x10, 0],
    );
    profile_record(e, PROFILE_END);
}

// Translated from 00cfab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleWorldRayCaster::castRay_ov2` (engine map), `(this, broad_phase, input,
/// collision_filter, extra_word, output)`, `RET 0x14`. As `castRay` (profiling label
/// `010cc668`), with `extra_word` as the last word of the request.
pub fn hkp_simple_world_ray_caster_cast_ray_ov2(
    e: &mut Engine,
    this: Ptr,
    broad_phase: Ptr,
    input: Ptr,
    collision_filter: u32,
    extra_word: u32,
    output: u32,
) {
    profile_record(e, PROFILE_BEGIN_CAST_RAY_OV2);
    begin_cast(
        e,
        this,
        broad_phase,
        input,
        collision_filter,
        output,
        [1, 0x10, extra_word],
    );
    profile_record(e, PROFILE_END);
}

// Translated from 00cfac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleWorldRayCaster::castRayGroup` (engine map), `(this, broad_phase, rays, count,
/// collision_filter, outputs)`, `RET 0x14`. Profiling label `010cc678`. Builds the AABB of all
/// `count` rays (0x30 bytes each: start at +0, end at +0x10; per lane `MINPS`/`MAXPS` of the
/// first ray's start against its end, then of each further ray's end and start), asks the
/// broad phase for the size of its scratch area (slot `0x60`), takes it (rounded up to 128
/// bytes) from the memory router's cursor (its first word), lets the broad phase prepare the
/// cast (slot `0x68`, `(AABB, scratch)`), runs `castRay_ov2(broad_phase, ray, collision_filter,
/// scratch, output)` for each ray, with the outputs 0x60 bytes apart, and puts the cursor back.
pub fn hkp_simple_world_ray_caster_cast_ray_group(
    e: &mut Engine,
    this: Ptr,
    broad_phase: Ptr,
    rays: Ptr,
    count: i32,
    collision_filter: u32,
    outputs: u32,
) {
    profile_record(e, PROFILE_BEGIN_CAST_RAY_GROUP);
    e.with_stack(0x20, |e, aabb| {
        let first_start = load_vec(e, rays.addr());
        let first_end = load_vec(e, rays.addr() + 0x10);
        let mut lower = zip_with(first_start, first_end, minps);
        let mut upper = zip_with(first_start, first_end, maxps);
        store_vec(e, aabb.addr(), lower);
        store_vec(e, aabb.addr() + 0x10, upper);
        let mut remaining = count.wrapping_sub(2);
        if remaining >= 0 {
            let mut ray = rays.addr() + 0x30;
            loop {
                let end = load_vec(e, ray + 0x10);
                let start = load_vec(e, ray);
                ray = ray.wrapping_add(0x30);
                remaining = remaining.wrapping_sub(1);
                lower = zip_with(lower, end, minps);
                upper = zip_with(upper, end, maxps);
                lower = zip_with(lower, start, minps);
                upper = zip_with(upper, start, maxps);
                if remaining < 0 {
                    break;
                }
            }
            store_vec(e, aabb.addr() + 0x10, upper);
            store_vec(e, aabb.addr(), lower);
        }
        let scratch_size = e.vcall(broad_phase.addr(), 0x60, &args![]).u32();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let scratch = e.mem.u32(router);
        let rounded = scratch_size.wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, rounded.wrapping_add(scratch));
        e.vcall(broad_phase.addr(), 0x68, &args![aabb, scratch]);
        let mut output = outputs;
        let mut left = count.wrapping_sub(1);
        let mut ray = rays.addr();
        while left >= 0 {
            hkp_simple_world_ray_caster_cast_ray_ov2(
                e,
                this,
                broad_phase,
                Ptr::new(ray),
                collision_filter,
                scratch,
                output,
            );
            output = output.wrapping_add(0x60);
            ray = ray.wrapping_add(0x30);
            left = left.wrapping_sub(1);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, scratch);
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00cfad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleWorldRayCaster::castRaysFromSinglePoint` (engine map), `(this, broad_phase,
/// input, ray_count, collision_filter, extra_word, outputs)`, `RET 0x18` plus the register
/// `this`. As `castRay` (profiling label `010cc688`), with a request `{ the input's ray
/// start, ray_count, input + 0x10, 0x30, extra_word }`.
#[allow(clippy::too_many_arguments)]
pub fn hkp_simple_world_ray_caster_cast_rays_from_single_point(
    e: &mut Engine,
    this: Ptr,
    broad_phase: Ptr,
    input: Ptr,
    ray_count: u32,
    collision_filter: u32,
    extra_word: u32,
    outputs: u32,
) {
    profile_record(e, PROFILE_BEGIN_CAST_RAYS_FROM_SINGLE_POINT);
    begin_cast(
        e,
        this,
        broad_phase,
        input,
        collision_filter,
        outputs,
        [ray_count, 0x30, extra_word],
    );
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf_a890,
            hkp_simple_world_ray_caster_add_broad_phase_handle(Ptr, Ptr, u32) -> f32
        ),
        entry!(
            0x00cf_aa40,
            hkp_simple_world_ray_caster_cast_ray(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00cf_ab30,
            hkp_simple_world_ray_caster_cast_ray_ov2(Ptr, Ptr, Ptr, u32, u32, u32)
        ),
        entry!(
            0x00cf_ac20,
            hkp_simple_world_ray_caster_cast_ray_group(Ptr, Ptr, Ptr, i32, u32, u32)
        ),
        entry!(
            0x00cf_ad60,
            hkp_simple_world_ray_caster_cast_rays_from_single_point(
                Ptr, Ptr, Ptr, u32, u32, u32, u32
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// A profile stream with room for two records.
    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    /// A broad phase whose slots `0x5c`, `0x60` and `0x68` log `[slot, this, arguments...]`;
    /// slot `0x5c` also logs the 32 bytes of the request.
    fn broad_phase(e: &mut Engine, log: &EventLog, scratch_size: u32) -> u32 {
        let table = 0x0900_3000;
        e.put_vtable(
            table,
            &(0..0x1c)
                .map(|slot| 0x0900_3100 + 4 * slot)
                .collect::<Vec<u32>>(),
        );
        for slot in [0x5cu32, 0x60, 0x68] {
            let slot_log = Rc::clone(log);
            e.register_double(0x0900_3100 + slot, move |e, a| {
                let mut entry = vec![slot];
                entry.extend_from_slice(a);
                if slot == 0x5c {
                    entry.extend((0..8).map(|word| e.mem.u32(a[1] + 4 * word)));
                }
                if slot == 0x68 {
                    entry.extend(load_bits(e, a[1]));
                    entry.extend(load_bits(e, a[1] + 0x10));
                }
                slot_log.borrow_mut().push(entry);
                Ret {
                    eax: scratch_size,
                    ..Ret::default()
                }
            });
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn cast_ray_stores_the_state_and_asks_the_broad_phase() {
        let mut e = engine();
        let stream = profile_stream(&mut e);
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        let broad = broad_phase(&mut e, &log, 0);
        let this = e.mem.alloc(0x40);
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [1.0, 2.0, 3.0, 4.0]);
        let output = e.mem.alloc(0x60);
        let filter = 0x5000;
        e.call(0x00cf_aa40, &args![this, broad, input, filter, output]);
        assert_eq!(e.mem.u32(this + 4), input);
        assert_eq!(e.mem.u32(this + 0xc), output);
        assert_eq!(e.mem.u32(this + 8), filter + 0x14);
        // Without the input's shape-collection byte there is no second filter.
        assert_eq!(e.mem.u32(this + 0x34), 0);
        let entry = log.borrow()[0].clone();
        assert_eq!(entry[..3], [0x5c, broad, entry[2]]);
        assert_eq!(entry[3..5], [this, 0]);
        let request: Vec<u32> = entry[5..].to_vec();
        assert_eq!(
            request,
            vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4.0f32.to_bits(),
                1,
                input + 0x10,
                0x10,
                0
            ]
        );
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        // The byte at input + 0x20 selects the second filter; a null filter nulls both.
        e.mem.set_u8(input + 0x20, 1);
        e.call(0x00cf_aa40, &args![this, broad, input, filter, output]);
        assert_eq!(e.mem.u32(this + 0x34), filter + 0x10);
        e.call(0x00cf_aa40, &args![this, broad, input, 0u32, output]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
    }

    #[test]
    fn cast_ray_ov2_adds_its_extra_word_to_the_request() {
        let mut e = engine();
        profile_stream(&mut e);
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        let broad = broad_phase(&mut e, &log, 0);
        let this = e.mem.alloc(0x40);
        let input = e.mem.alloc(0x30);
        let output = e.mem.alloc(0x60);
        e.call(
            0x00cf_ab30,
            &args![this, broad, input, 0x6000u32, 0x1234u32, output],
        );
        assert_eq!(e.mem.u32(this + 8), 0x6014);
        assert_eq!(e.mem.u32(this + 0xc), output);
        let entry = log.borrow()[0].clone();
        assert_eq!(entry[5 + 4..], [1, input + 0x10, 0x10, 0x1234]);
    }

    #[test]
    fn rays_from_a_single_point_ask_for_every_ray() {
        let mut e = engine();
        let stream = profile_stream(&mut e);
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        let broad = broad_phase(&mut e, &log, 0);
        let this = e.mem.alloc(0x40);
        let input = e.mem.alloc(0x30);
        let output = e.mem.alloc(0x60);
        e.call(
            0x00cf_ad60,
            &args![this, broad, input, 7u32, 0x6000u32, 0x99u32, output],
        );
        assert_eq!(e.mem.u32(this + 0xc), output);
        let entry = log.borrow()[0].clone();
        assert_eq!(entry[5 + 4..], [7, input + 0x10, 0x30, 0x99]);
        assert_eq!(
            e.mem.u32(stream + 0x20),
            PROFILE_BEGIN_CAST_RAYS_FROM_SINGLE_POINT
        );
    }

    #[test]
    fn a_ray_group_is_cast_ray_by_ray_inside_its_bounding_box() {
        let mut e = engine();
        let stream = profile_stream(&mut e);
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        // 100 bytes of scratch space become 128.
        let broad = broad_phase(&mut e, &log, 100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, 0x7000);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let rays = e.mem.alloc(0x30 * 3);
        let ends = [
            ([1.0, 5.0, 0.0, 0.0], [2.0, -1.0, 3.0, 0.0]),
            ([-4.0, 0.0, 1.0, 0.0], [0.0, 9.0, 2.0, 0.0]),
            ([0.5, 0.5, -7.0, 0.0], [1.0, 1.0, 8.0, 0.0]),
        ];
        for (index, (start, end)) in ends.into_iter().enumerate() {
            store_vec(&mut e, rays + 0x30 * index as u32, start);
            store_vec(&mut e, rays + 0x30 * index as u32 + 0x10, end);
        }
        let outputs = e.mem.alloc(0x60 * 3);
        // The scratch area is taken from the cursor and handed back.
        e.call(
            0x00cf_ac20,
            &args![this, broad, rays, 3i32, 0x6000u32, outputs],
        );
        assert_eq!(e.mem.u32(router), 0x7000);
        let calls = log.borrow();
        let size_request = calls.iter().filter(|entry| entry[0] == 0x60).count();
        assert_eq!(size_request, 1);
        let prepare = calls.iter().find(|entry| entry[0] == 0x68).unwrap();
        assert_eq!(prepare[1..4], [broad, prepare[2], 0x7000]);
        let bounds: Vec<f32> = prepare[4..]
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect();
        assert_eq!(bounds, vec![-4.0, -1.0, -7.0, 0.0, 2.0, 9.0, 8.0, 0.0]);
        // Three casts, one per ray, each handing the scratch cursor on in the request, with
        // the outputs 0x60 bytes apart.
        let casts: Vec<&Vec<u32>> = calls.iter().filter(|entry| entry[0] == 0x5c).collect();
        assert_eq!(casts.len(), 3);
        for (index, cast) in casts.iter().enumerate() {
            assert_eq!(
                cast[5 + 4..],
                [1, rays + 0x30 * index as u32 + 0x10, 0x10, 0x7000]
            );
        }
        assert_eq!(e.mem.u32(this + 0xc), outputs + 0xc0);
        assert_eq!(e.mem.u32(this + 8), 0x6014);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY_GROUP);
    }

    /// A raycaster, the handle of a collidable and the output of the cast.
    struct RayScene {
        this: u32,
        handle: u32,
        collidable: u32,
        output: u32,
        log: EventLog,
    }

    /// A raycaster with a ray filter and a collidable with a shape and a transform (rotation
    /// columns `(0,1,0)`, `(-1,0,0)`, `(0,0,1)`, translation `(1,2,3)`), and a ray at index 1 of
    /// its input; every callee logs to the scene's log.
    fn ray_scene(e: &mut Engine, filter_answer: u8, shape_hit: u8) -> RayScene {
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x40);
        // The ray filter: slot 4 answers the pointer it was given.
        e.put_vtable(0x0900_3300, &[0, 0x0900_3304]);
        let ray_filter = e.mem.alloc(0x10);
        e.mem.set_u32(ray_filter, 0x0900_3300);
        e.mem.set_u32(this + 8, ray_filter);
        let filter_log = Rc::clone(&log);
        e.register_double(0x0900_3304, move |e, a| {
            filter_log.borrow_mut().push(vec![4, a[0], a[2], a[3]]);
            e.mem.set_u8(a[1], filter_answer);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // The shape: slot 0x20 sets the result byte.
        let shape = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 9];
        slots[8] = 0x0900_3408;
        e.put_vtable(0x0900_3400, &slots);
        e.mem.set_u32(shape, 0x0900_3400);
        let shape_log = Rc::clone(&log);
        e.register_double(0x0900_3408, move |e, a| {
            shape_log.borrow_mut().push(vec![0x20, a[0], a[2], a[3]]);
            e.mem.set_u8(a[1], shape_hit);
            Ret::default()
        });
        // The collidable: shape at +0, transform at +8; the handle sits 0x10 bytes above it
        // with the (signed) offset back at +5.
        let collidable = e.mem.alloc(0x40);
        e.mem.set_u32(collidable, shape);
        let transform = e.mem.alloc(0x40);
        e.mem.set_u32(collidable + 8, transform);
        store_vec(e, transform, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, transform + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, transform + 0x30, [1.0, 2.0, 3.0, 1.0]);
        let handle = collidable + 0x20;
        e.mem.set_i8(handle + 5, -0x20);
        // Two rays; the second one is used.
        let rays = e.mem.alloc(0x60);
        store_vec(e, rays + 0x30, [2.0, 2.0, 3.0, 0.0]);
        store_vec(e, rays + 0x40, [1.0, 5.0, 3.0, 0.0]);
        e.mem.set_u32(rays + 0x30 + 0x24, 0xf1f1);
        e.mem.set_u32(this + 4, rays);
        let output = e.mem.alloc(0x60);
        store_vec(e, output, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(output + 0x10, 0.75);
        e.mem.set_u32(this + 0xc, output);
        RayScene {
            this,
            handle,
            collidable,
            output,
            log,
        }
    }

    #[test]
    fn a_hit_moves_the_ray_into_local_space_and_the_normal_back() {
        let mut e = engine();
        let scene = ray_scene(&mut e, 1, 1);
        let hit_fraction = e
            .call(0x00cf_a890, &args![scene.this, scene.handle, 1u32])
            .f32();
        assert_eq!(hit_fraction, 0.75);
        // (2, 2, 3) - (1, 2, 3) = (1, 0, 0) -> (0, -1, 0); (1, 5, 3) - (1, 2, 3) = (0, 3, 0) -> (3, 0, 0).
        assert_eq!(load_vec(&e, scene.this + 0x10), [0.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, scene.this + 0x20), [3.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(scene.this + 0x30), 0xf1f1);
        let rays = e.mem.u32(scene.this + 4);
        let log = scene.log.borrow();
        assert_eq!(
            log[0],
            vec![4, e.mem.u32(scene.this + 8), rays + 0x30, scene.collidable]
        );
        assert_eq!(log[1][2..], [scene.this + 0x10, scene.output]);
        assert_eq!(e.mem.u32(scene.output + 0x50), scene.collidable);
        // The normal (1, 0, 0) turns into the first column.
        assert_eq!(load_vec(&e, scene.output), [0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn a_vetoed_or_missed_ray_leaves_the_output_alone() {
        let mut e = engine();
        let scene = ray_scene(&mut e, 0, 1);
        let hit_fraction = e
            .call(0x00cf_a890, &args![scene.this, scene.handle, 1u32])
            .f32();
        assert_eq!(hit_fraction, 0.75);
        assert_eq!(scene.log.borrow().len(), 1);
        assert_eq!(load_vec(&e, scene.output), [1.0, 0.0, 0.0, 0.0]);
        let mut e = engine();
        let scene = ray_scene(&mut e, 1, 0);
        e.call(0x00cf_a890, &args![scene.this, scene.handle, 1u32]);
        assert_eq!(scene.log.borrow().len(), 2);
        assert_eq!(e.mem.u32(scene.output + 0x50), 0);
        assert_eq!(load_vec(&e, scene.output), [1.0, 0.0, 0.0, 0.0]);
        // A collidable without a shape is skipped at once.
        e.mem.set_u32(scene.collidable, 0);
        scene.log.borrow_mut().clear();
        e.call(0x00cf_a890, &args![scene.this, scene.handle, 1u32]);
        assert!(scene.log.borrow().is_empty());
    }
}
