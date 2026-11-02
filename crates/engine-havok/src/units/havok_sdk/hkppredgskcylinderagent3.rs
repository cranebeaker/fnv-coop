//! `Havok SDK/hkppredgskcylinderagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use std::cmp::Ordering;

use super::hkpsphereshape::{
    inverse_length, load_bits, load_vec, profile_record, store_bits, store_vec, sum3, zip_with,
    Vec4, PROFILE_END, ZERO_F32,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpCylinderShape::getCylinderRadius` (`00d571a0`, `this` = the shape, result in `ST0`).
const CYLINDER_RADIUS: u32 = 0x00d5_71a0;
/// `hkpCapsuleShape::hkpCapsuleShape` (`00d22590`, `this` = the block, `(vertex A, vertex B,
/// radius)`, `RET 0xC`, returns the block).
const CAPSULE_CONSTRUCT: u32 = 0x00d2_2590;
/// `hkPredGskAgent3::sepNormal` (`00d41930`, cdecl: the two body cells, the entry, the cache or
/// the vector it writes).
const SEPARATING_NORMAL: u32 = 0x00d4_1930;
/// `hkVector4::setRotatedDir` (`00c7f700`, `this` = the result, `(matrix, vector)`, `RET 8`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The manifold clean-up (`00dad640`, cdecl `(P3 + 0xC, agent word 4, output word 1)`).
const MANIFOLD_CLEANUP: u32 = 0x00da_d640;
/// The body cell builder (`00d33260`, `this` = the cells, `(agent)`, `RET 4`).
const BODY_CELLS: u32 = 0x00d3_3260;
/// `hkpGskCache::init` (`00dafcd0`, `this` = the cache, `(shape A, shape B, agent + 0x20)`,
/// `RET 0xC`).
const GSK_CACHE_INIT: u32 = 0x00da_fcd0;
/// The time-of-impact step (`00db60c0`, cdecl: the agent, three floats, the three buffers).
const TIME_OF_IMPACT: u32 = 0x00db_60c0;
/// The process step without a time of impact (`00db4180`, cdecl, seven words).
const PROCESS_NO_TIME_OF_IMPACT: u32 = 0x00db_4180;
/// The manifold finish (`00dae3a0`, cdecl `(P3 + 0xC, work points, 0, output, agent word 4)`).
const MANIFOLD_FINISH: u32 = 0x00da_e3a0;
/// `separatingDistanceUsingCapsuleRepresentations` (this unit, `00d40be0`).
const SEPARATING_DISTANCE: u32 = 0x00d4_0be0;
/// The reference count decrement (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The profiling label `process` starts with (`TtPredGskf3`); it ends with `PROFILE_END`.
const PROFILE_BEGIN: u32 = 0x010c_f220;
/// The margin added to a cylinder's radius for its capsule (a double, about `0.002`).
const CAPSULE_MARGIN: u32 = 0x010c_f218;
/// The `|dot|` above which a selected capsule representation is checked again (a double,
/// `0.1`).
const CHECK_AGAIN_LIMIT: u32 = 0x0101_ffa0;
/// The `|dot|` below which the capsule representation is selected at once (a double, `0.05`).
const SELECT_LIMIT: u32 = 0x0103_c7c8;
/// The separating distance above which the capsule representation is selected (a float,
/// `0.01`).
const SELECT_DISTANCE: u32 = 0x0101_3ea4;

/// Replaces the cylinder shape of one body cell by a capsule: allocates 0x40 bytes through the
/// allocator (slot 4), builds a capsule of the cylinder's axis end points and its radius plus
/// the cylinder radius and the margin, fills `record` (capsule, `-1`, the body's word 2 and
/// the body) when it is not null, and makes the cell point at `record`. Returns the body the
/// cell held.
fn replace_with_capsule(e: &mut Engine, cell: u32, record: u32) -> u32 {
    let shape = e.mem.u32(e.mem.u32(cell));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let radius = x87_quiet(e.mem.f32(shape + 0x10));
    let cylinder_radius = e.call(CYLINDER_RADIUS, &args![shape]).f64();
    let margin: f64 = e.global(CAPSULE_MARGIN);
    let capsule_radius = ((cylinder_radius + f64::from(radius)) + margin) as f32;
    let capsule = e
        .call(
            CAPSULE_CONSTRUCT,
            &args![block, shape + 0x20, shape + 0x30, capsule_radius],
        )
        .u32();
    if record != 0 {
        let body = e.mem.u32(cell);
        let word2 = e.mem.u32(body + 8);
        e.mem.set_u32(record + 0xc, body);
        e.mem.set_u32(record + 8, word2);
    }
    e.mem.set_u32(record, capsule);
    e.mem.set_u32(record + 4, 0xffff_ffff);
    let saved = e.mem.u32(cell);
    e.mem.set_u32(cell, record);
    saved
}

/// The loop the three functions share: for both cells in `slots` (addresses of body cells),
/// when the word of `flags` is set replace the cell's cylinder by a capsule (see
/// [`replace_with_capsule`]) using the next 0x10-byte record starting at `records`, and store
/// the body the cell held in `saved`; otherwise store `0` there.
fn replace_selected_cylinders(e: &mut Engine, flags: u32, saved: u32, slots: u32, records: u32) {
    for index in 0..2u32 {
        if e.mem.u32(flags + 4 * index) == 0 {
            e.mem.set_u32(saved + 4 * index, 0);
        } else {
            let cell = e.mem.u32(slots + 4 * index);
            let body = replace_with_capsule(e, cell, records + 0x10 * index);
            e.mem.set_u32(saved + 4 * index, body);
        }
    }
}

// Translated from 00d40a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `replaceCylindersWithCapsulesInCdBodies` (the name Ghidra shows). Register convention: the
/// flag array in `EAX`, the saved-body array in `ECX`, then two stack words, the cell address
/// array and the records; the words here are `(flags, saved, slots, records)`. The caller
/// cleans the stack. Each of the two flag words that is not zero makes the matching cell's
/// cylinder shape a capsule (see the shared loop).
pub fn replace_cylinders_with_capsules_in_cd_bodies(
    e: &mut Engine,
    flags: Ptr,
    saved: Ptr,
    slots: Ptr,
    records: Ptr,
) {
    replace_selected_cylinders(e, flags.addr(), saved.addr(), slots.addr(), records.addr());
}

/// The frame of `separatingDistanceUsingCapsuleRepresentations`, offsets from its base: the
/// bodies saved by the replacement, the cell address array, the flag words, the cache, the
/// separating vector (its fourth lane is the distance) and the two body cells.
const DISTANCE_SAVED: u32 = 0x0c;
const DISTANCE_SLOTS: u32 = 0x14;
const DISTANCE_FLAGS: u32 = 0x20;
const DISTANCE_CACHE: u32 = 0x28;
const DISTANCE_VECTOR: u32 = 0x34;
const DISTANCE_CELLS: u32 = 0x44;
const DISTANCE_RECORDS: u32 = 0xc4;
const DISTANCE_FRAME: u32 = 0xe4;

// Translated from 00d40be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `separatingDistanceUsingCapsuleRepresentations` (the name Ghidra shows), cdecl, the result
/// in `ST0`. Builds the body cells of `agent`, copies five of its words (`0x60`, `0x70` to
/// `0x7c`) into the cell block, replaces the cylinders of the bodies the low two bits of
/// `agent_data[0xB]` select, initialises a cache from the two shapes, asks the separating
/// normal for the vector and its distance (the fourth lane), puts the original bodies back
/// (dropping a reference on each capsule) and returns the distance.
pub fn separating_distance_using_capsule_representations(
    e: &mut Engine,
    agent: Ptr,
    entry: Ptr,
    agent_data: Ptr,
) -> f32 {
    e.with_stack(DISTANCE_FRAME, |e, frame| {
        let f = frame.addr();
        let agent = agent.addr();
        e.call(BODY_CELLS, &args![f + DISTANCE_CELLS, agent]);
        for (from, to) in [
            (0x60, 0xa4),
            (0x70, 0xb4),
            (0x74, 0xb8),
            (0x78, 0xbc),
            (0x7c, 0xc0),
        ] {
            let word = e.mem.u32(agent + from);
            e.mem.set_u32(f + to, word);
        }
        e.mem.set_u32(f + DISTANCE_SLOTS, f + DISTANCE_CELLS);
        e.mem
            .set_u32(f + DISTANCE_SLOTS + 4, f + DISTANCE_CELLS + 4);
        let flags = u32::from(e.mem.u8(agent_data.addr() + 0xb));
        e.mem.set_u32(f + DISTANCE_FLAGS, flags & 1);
        e.mem.set_u32(f + DISTANCE_FLAGS + 4, flags & 2);
        replace_selected_cylinders(
            e,
            f + DISTANCE_FLAGS,
            f + DISTANCE_SAVED,
            f + DISTANCE_SLOTS,
            f + DISTANCE_RECORDS,
        );
        let first = e.mem.u32(e.mem.u32(f + DISTANCE_CELLS));
        let second = e.mem.u32(e.mem.u32(f + DISTANCE_CELLS + 4));
        e.call(
            GSK_CACHE_INIT,
            &args![f + DISTANCE_CACHE, first, second, agent + 0x20],
        );
        e.call(
            SEPARATING_NORMAL,
            &args![
                f + DISTANCE_CELLS,
                entry,
                f + DISTANCE_CACHE,
                f + DISTANCE_VECTOR
            ],
        );
        for index in 0..2u32 {
            let saved = e.mem.u32(f + DISTANCE_SAVED + 4 * index);
            if saved != 0 {
                let cell = e.mem.u32(f + DISTANCE_SLOTS + 4 * index);
                let capsule = e.mem.u32(e.mem.u32(cell));
                e.call(REMOVE_REFERENCE, &args![capsule]);
                e.mem.set_u32(cell, saved);
            }
        }
        x87_quiet(e.mem.f32(f + DISTANCE_VECTOR + 0xc))
    })
}

/// The frame of `process`, offsets from its base: the quieted threshold, the two selection
/// words (the high two bits of the flags), the two request words (the low two bits), the
/// bodies saved by the replacement, the two body cell addresses, the direction, the
/// separating vector (distance in the fourth lane), the capsule records, the work points and
/// the temporaries of the contact path.
const PROCESS_THRESHOLD: u32 = 0x0c;
const PROCESS_CACHE: u32 = 0x14;
const PROCESS_SELECTED: u32 = 0x24;
const PROCESS_REQUESTED: u32 = 0x3c;
const PROCESS_SAVED: u32 = 0x44;
const PROCESS_SLOTS: u32 = 0x4c;
const PROCESS_DIRECTION: u32 = 0x54;
const PROCESS_VECTOR: u32 = 0x74;
const PROCESS_RECORDS: u32 = 0x84;
const PROCESS_WORK: u32 = 0xa4;
const PROCESS_NORMAL_COPY: u32 = 0x1a4;
const PROCESS_TOLERANCE: u32 = 0x1bc;
const PROCESS_RADIUS_A: u32 = 0x1b4;
const PROCESS_RADIUS_B: u32 = 0x1b8;
const PROCESS_SQUARE: u32 = 0x1c0;
const PROCESS_FRAME: u32 = 0x1c4;

/// The topmost parent (word `+0xC`, null at the top) of a body.
fn root_of(e: &Engine, mut body: u32) -> u32 {
    loop {
        let parent = e.mem.u32(body + 0xc);
        if parent == 0 {
            return body;
        }
        body = parent;
    }
}

/// Makes selection `index` (the flag `4 << index` of `agent_data[0xB]`) on or off, runs the
/// manifold clean-up and copies the contact count into the entry.
fn change_selection(
    e: &mut Engine,
    f: u32,
    index: u32,
    on: bool,
    (agent, entry, agent_data, output): (u32, u32, u32, u32),
) {
    let flags = e.mem.u8(agent_data + 0xb);
    let bit = 4u8 << index;
    e.mem
        .set_u32(f + PROCESS_SELECTED + 4 * index, u32::from(on));
    e.mem.set_u8(
        agent_data + 0xb,
        if on { flags | bit } else { flags & !bit },
    );
    let owner = e.mem.u32(agent + 0x10);
    let word = e.mem.u32(output + 4);
    e.call(MANIFOLD_CLEANUP, &args![agent_data + 0xc, owner, word]);
    let count = e.mem.u8(agent_data + 0xe);
    e.mem.set_u8(entry + 2, count);
}

/// Copies the four rows of `matrix` word by word to `to` (row `n` to `to[n]`) and loads them
/// back as vectors.
pub(crate) fn copy_rows(e: &mut Engine, matrix: u32, to: [u32; 4]) -> [Vec4; 4] {
    for (row, address) in to.iter().enumerate() {
        let bits = load_bits(e, matrix + 16 * row as u32);
        store_bits(e, *address, bits);
    }
    to.map(|address| load_vec(e, address))
}

/// Transforms the points `base + 16 * index` for `index` from `count - 1` down to `0` by the
/// rows (`y * row1 + x * row0 + (z * row2 + row3)`). The loop tests after the body, so a
/// count of `0` still transforms the point before `base`.
pub(crate) fn transform_points(e: &mut Engine, base: u32, count: u8, rows: &[Vec4; 4]) {
    let mut index = i32::from(count) - 1;
    let mut address = base.wrapping_add((index as u32) << 4);
    loop {
        let v = load_vec(e, address);
        let point: Vec4 = std::array::from_fn(|lane| {
            (v[1] * rows[1][lane] + v[0] * rows[0][lane]) + (v[2] * rows[2][lane] + rows[3][lane])
        });
        store_vec(e, address, point);
        index -= 1;
        address = address.wrapping_sub(0x10);
        if index < 0 {
            break;
        }
    }
}

/// Replaces the cylinders of the bodies whose selection word is set (see
/// [`replace_selected_cylinders`], on the frame's arrays).
fn replace_selected_in_frame(e: &mut Engine, f: u32) {
    replace_selected_cylinders(
        e,
        f + PROCESS_SELECTED,
        f + PROCESS_SAVED,
        f + PROCESS_SLOTS,
        f + PROCESS_RECORDS,
    );
}

/// The contact path of `process`: the capsule representations, the support points of both
/// shapes in world space and the manifold finish.
fn contact_points(
    e: &mut Engine,
    f: u32,
    agent: u32,
    agent_data: u32,
    output: u32,
    separating: u32,
) {
    replace_selected_in_frame(e, f);
    let data = e.mem.u32(agent + 0xc);
    let tolerance = x87_quiet(e.mem.f32(data + 8));
    e.mem.set_f32(f + PROCESS_TOLERANCE, tolerance);
    let body_a = e.mem.u32(agent);
    let body_b = e.mem.u32(agent + 4);
    let shape_a = e.mem.u32(body_a);
    let shape_b = e.mem.u32(body_b);
    let radius_a = x87_quiet(e.mem.f32(shape_a + 0x10));
    e.mem.set_f32(f + PROCESS_RADIUS_A, radius_a);
    let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
    e.mem.set_f32(f + PROCESS_RADIUS_B, radius_b);
    let normal = load_bits(e, separating);
    let count = e.mem.u8(agent_data + 0xe);
    store_bits(e, f + PROCESS_NORMAL_COPY, normal);
    let sum = ((f64::from(tolerance) + f64::from(radius_a)) + f64::from(radius_b)) as f32;
    e.mem.set_f32(f + PROCESS_THRESHOLD, sum);
    // The square of the sum is stored too, and nothing reads it back.
    e.mem
        .set_f32(f + PROCESS_SQUARE, (f64::from(sum) * f64::from(sum)) as f32);
    let work = f + PROCESS_WORK;
    let contacts = agent_data + 0xc;
    // The exe tests the count byte again here; it was just found not zero, so the other way
    // (straight to the manifold finish) is not taken unless a callee changed the byte.
    if count != 0 {
        let after_points = contacts + u32::from(count) * 8 + 4;
        let count_a = e.mem.u8(contacts);
        e.vcall(
            shape_a,
            0x3c,
            &args![after_points, u32::from(count_a), work],
        );
        let matrix_a = e.mem.u32(body_a + 8);
        let rows = copy_rows(e, matrix_a, [f + 0x14, f + 0x24, f + 0x64, f + 0x54]);
        let count_a = e.mem.u8(contacts);
        transform_points(e, work, count_a, &rows);
        let count_b = e.mem.u8(agent_data + 0xd);
        let second_work = work + (u32::from(count_a) << 4);
        e.vcall(
            shape_b,
            0x3c,
            &args![
                after_points + u32::from(count_a) * 2,
                u32::from(count_b),
                second_work
            ],
        );
        let matrix_b = e.mem.u32(body_b + 8);
        let rows = copy_rows(e, matrix_b, [f + 0x54, f + 0x64, f + 0x14, f + 0x24]);
        let count_b = e.mem.u8(agent_data + 0xd);
        transform_points(e, second_work, count_b, &rows);
    }
    let owner = e.mem.u32(agent + 0x10);
    e.call(MANIFOLD_FINISH, &args![contacts, work, 0u32, output, owner]);
    let count = e.mem.u8(contacts + 2);
    if count != 0 {
        let stack = e.mem.u32(output + 0x3080);
        if stack != 0 {
            let base = e.mem.u32(output);
            let value = base.wrapping_sub(u32::from(count) * 0x30);
            let cursor = e.mem.u32(stack + 4);
            e.mem.set_u32(cursor, value);
            e.mem.set_u32(stack + 4, cursor + 4);
        }
    }
}

/// Builds the cache for the contact path when a capsule representation is selected: the 12
/// bytes of `agent_data` are copied to the frame and, for each selected body, the flag word
/// for it is made `0x10` or `0` by bit 7 of its current value; a body with more than one
/// subtype entry gets `0x10` and `0` for the entry after.
fn selection_cache(e: &mut Engine, f: u32, agent_data: u32) -> u32 {
    let selected_a = e.mem.u32(f + PROCESS_SELECTED);
    let selected_b = e.mem.u32(f + PROCESS_SELECTED + 4);
    if selected_a == 0 && selected_b == 0 {
        return agent_data;
    }
    let cache = f + PROCESS_CACHE;
    for word in 0..3 {
        let value = e.mem.u32(agent_data + 4 * word);
        e.mem.set_u32(cache + 4 * word, value);
    }
    if selected_a != 0 {
        let bit = (e.mem.u32(cache) >> 7) & 1;
        e.mem.set_u16(cache, ((1 - bit) << 4) as u16);
        let count = e.mem.u8(cache + 8);
        if count > 1 {
            e.mem.set_u16(cache, 0x10);
            e.mem.set_u16(cache + 2, 0);
            if count == 3 {
                let word = e.mem.u16(cache + 6);
                e.mem.set_u8(cache + 8, 2);
                e.mem.set_u16(cache + 4, word);
            }
        }
    }
    if selected_b != 0 {
        let index = u32::from(e.mem.u8(cache + 8));
        let address = cache + index * 2;
        let bit = (e.mem.u16(address) >> 7) & 1;
        e.mem.set_u16(address, (1 - bit) << 4);
        if e.mem.u8(cache + 9) > 1 {
            let index = u32::from(e.mem.u8(cache + 8));
            e.mem.set_u16(cache + index * 2, 0x10);
            let index = u32::from(e.mem.u8(cache + 8));
            e.mem.set_u16(cache + index * 2 + 2, 0);
            if e.mem.u8(cache + 9) == 3 {
                e.mem.set_u8(cache + 9, 2);
            }
        }
    }
    cache
}

// Translated from 00d40d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPredGskCylinderAgent3::process` (the name Ghidra shows), cdecl. The parameter names
/// follow the Havok declaration of an agent3 process function and are not confirmed in the
/// exe: the process input (words `0`/`1` the two bodies, `3` the collision input, `4` the
/// owner, `0x60` a threshold float), the entry (byte `2` the contact count), the agent data
/// (byte `0xB` the flags, bytes `0xC` to `0xE` the counts), the separating normal output and
/// the process output. Each body's cylinder can be shown to the contact code as a capsule:
/// the flags' low two bits ask for it and bits 2 and 3 record that it is on, switched by the
/// direction of the cylinder axis against the separating vector. Returns the end of the agent
/// data (`agent_data + ((count A + 4 * count C + count B) * 2 + 0x1F & !0xF)`).
pub fn pred_gsk_cylinder_agent3_process(
    e: &mut Engine,
    input: Ptr,
    entry: Ptr,
    agent_data: Ptr,
    separating_normal: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN);
    let agent = input.addr();
    let entry = entry.addr();
    let agent_data = agent_data.addr();
    let separating = separating_normal.addr();
    let output = output.addr();
    e.with_stack(PROCESS_FRAME, |e, frame| {
        let f = frame.addr();
        let threshold = x87_quiet(e.mem.f32(agent + 0x60));
        e.mem.set_f32(f + PROCESS_THRESHOLD, threshold);
        e.mem.set_u32(f + PROCESS_SLOTS, agent);
        e.mem.set_u32(f + PROCESS_SLOTS + 4, agent + 4);
        let flags = u32::from(e.mem.u8(agent_data + 0xb));
        e.mem.set_u32(f + PROCESS_REQUESTED, flags & 1);
        e.mem.set_u32(f + PROCESS_REQUESTED + 4, flags & 2);
        e.mem.set_u32(f + PROCESS_SELECTED, flags & 4);
        e.mem.set_u32(f + PROCESS_SELECTED + 4, flags & 8);
        e.call(
            SEPARATING_NORMAL,
            &args![agent, entry, agent_data, f + PROCESS_VECTOR],
        );
        let context = (agent, entry, agent_data, output);
        for index in 0..2u32 {
            if e.mem.u32(f + PROCESS_REQUESTED + 4 * index) == 0 {
                continue;
            }
            let cell = e.mem.u32(f + PROCESS_SLOTS + 4 * index);
            let body = e.mem.u32(cell);
            let shape = e.mem.u32(body);
            let from = load_vec(e, shape + 0x20);
            let to = load_vec(e, shape + 0x30);
            let axis = zip_with(to, from, |a, b| a - b);
            let inverse = inverse_length(e, sum3(zip_with(axis, axis, |a, b| a * b)));
            let direction = axis.map(|lane| inverse * lane);
            store_vec(e, f + PROCESS_DIRECTION, direction);
            let body = e.mem.u32(e.mem.u32(f + PROCESS_SLOTS + 4 * index));
            let matrix = e.mem.u32(body + 8);
            e.call(
                SET_ROTATED_DIR,
                &args![f + PROCESS_DIRECTION, matrix, f + PROCESS_DIRECTION],
            );
            let selected = e.mem.u32(f + PROCESS_SELECTED + 4 * index);
            let direction = load_vec(e, f + PROCESS_DIRECTION);
            let vector = load_vec(e, f + PROCESS_VECTOR);
            let alignment = x87_quiet(sum3(zip_with(vector, direction, |a, b| a * b))).abs();
            if selected != 0 {
                let limit: f64 = e.global(CHECK_AGAIN_LIMIT);
                if f64::from(alignment) > limit {
                    let distance = e
                        .call(SEPARATING_DISTANCE, &args![agent, entry, agent_data])
                        .f64();
                    let zero: f32 = e.global(ZERO_F32);
                    if distance < f64::from(zero) {
                        change_selection(e, f, index, false, context);
                    }
                }
            } else {
                let limit: f64 = e.global(SELECT_LIMIT);
                let near = f64::from(alignment) < limit;
                let select = near || {
                    let distance = e
                        .call(SEPARATING_DISTANCE, &args![agent, entry, agent_data])
                        .f64();
                    let minimum: f32 = e.global(SELECT_DISTANCE);
                    distance > f64::from(minimum)
                };
                if select {
                    change_selection(e, f, index, true, context);
                }
            }
        }
        let data = e.mem.u32(agent + 0xc);
        let params = e.mem.u32(data + 0x60);
        let threshold_value = f64::from(threshold);
        let mut contact_path = false;
        let mut finished = false;
        if e.mem.u32(params + 0x10) != 0 {
            let first = root_of(e, e.mem.u32(agent));
            let first_scale = x87_quiet(e.mem.f32(first + 0x4c));
            let second = root_of(e, e.mem.u32(agent + 4));
            let second_scale = x87_quiet(e.mem.f32(second + 0x4c));
            let scale = if second_scale > first_scale {
                first_scale
            } else {
                second_scale
            };
            let offset = x87_quiet(e.mem.f32(separating + 0xc));
            let scale64 = f64::from(scale);
            let a = f64::from(x87_quiet(e.mem.f32(params + 0x18)));
            let b = f64::from(x87_quiet(e.mem.f32(params + 0x1c)));
            let low = (a * scale64) as f32;
            let high = ((b * scale64) + f64::from(offset)) as f32;
            let first_limit = if high > low { low } else { high };
            let fits = matches!(
                f64::from(first_limit).partial_cmp(&threshold_value),
                Some(Ordering::Less | Ordering::Equal)
            );
            if !fits {
                let c = f64::from(x87_quiet(e.mem.f32(params + 0x28)));
                let d = f64::from(x87_quiet(e.mem.f32(params + 0x2c)));
                let low = (c * scale64) as f32;
                let high = (f64::from(offset) + (d * scale64)) as f32;
                let second_limit = if high > low { low } else { high };
                e.call(
                    TIME_OF_IMPACT,
                    &args![
                        agent,
                        scale,
                        first_limit,
                        second_limit,
                        agent_data,
                        separating,
                        output
                    ],
                );
                finished = true;
            }
        }
        if !finished {
            let reach = f64::from(x87_quiet(e.mem.f32(params + 0xc)));
            let distance = f64::from(x87_quiet(e.mem.f32(f + PROCESS_VECTOR + 0xc)));
            let tolerance = f64::from(x87_quiet(e.mem.f32(data + 8)));
            contact_path = reach < threshold_value
                && e.mem.u8(agent_data + 0xb) & 0x10 == 0
                && tolerance > distance;
        }
        if contact_path {
            e.mem.set_f32(separating + 0xc, threshold);
            if e.mem.u8(agent_data + 0xe) != 0 {
                contact_points(e, f, agent, agent_data, output, separating);
            }
        } else {
            let data = e.mem.u32(agent + 0xc);
            let tolerance = f64::from(x87_quiet(e.mem.f32(data + 8)));
            let distance = f64::from(x87_quiet(e.mem.f32(f + PROCESS_VECTOR + 0xc)));
            if tolerance > distance {
                replace_selected_in_frame(e, f);
                let cache = selection_cache(e, f, agent_data);
                e.call(
                    PROCESS_NO_TIME_OF_IMPACT,
                    &args![
                        agent,
                        entry,
                        agent_data,
                        cache,
                        agent_data + 0xc,
                        separating,
                        output
                    ],
                );
            } else if e.mem.u8(agent_data + 0xe) != 0 {
                let owner = e.mem.u32(agent + 0x10);
                let word = e.mem.u32(output + 4);
                e.call(MANIFOLD_CLEANUP, &args![agent_data + 0xc, owner, word]);
            }
            let vector = load_bits(e, f + PROCESS_VECTOR);
            store_bits(e, separating, vector);
        }
        for index in 0..2u32 {
            let saved = e.mem.u32(f + PROCESS_SAVED + 4 * index);
            if saved != 0 {
                let cell = e.mem.u32(f + PROCESS_SLOTS + 4 * index);
                let capsule = e.mem.u32(e.mem.u32(cell));
                e.call(REMOVE_REFERENCE, &args![capsule]);
                e.mem.set_u32(cell, saved);
            }
        }
        let count = e.mem.u8(agent_data + 0xe);
        e.mem.set_u8(entry + 2, count);
    });
    profile_record(e, PROFILE_END);
    let count_a = u32::from(e.mem.u8(agent_data + 0xc));
    let count_b = u32::from(e.mem.u8(agent_data + 0xd));
    let count_c = u32::from(e.mem.u8(agent_data + 0xe));
    let words = count_b + count_c * 4 + count_a;
    Ptr::new(((words * 2 + 0x1f) & 0xffff_fff0).wrapping_add(agent_data))
}

// Translated from 00d40b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: three stack words (cdecl, nothing popped): `bodies` (a pair of
/// collidable pointers: the first at +0, the second at +4), `second` and `result`. Calls `00d418d0`
/// with all three, then keeps only the high nibble of the byte at `result + 0xb` (`AND 0xf0`) and
/// sets its bits 0 and 2 when the shape of the first collidable (the word at +0 of the body) has the
/// type word 2 (at +0xc), and its bits 1 and 3 when the shape of the second one has it. Returns
/// the answer of `00d418d0` (`EAX`). @EV@
pub fn fn_00d40b90(e: &mut Engine, bodies: u32, second: u32, result: u32) -> u32 {
    let answer = e.call(0x00d4_18d0, &args![bodies, second, result]).u32();
    let kept = e.mem.u8(result + 0xb) & 0xf0;
    e.mem.set_u8(result + 0xb, kept);
    let first_body = e.mem.u32(bodies);
    let first_shape = e.mem.u32(first_body);
    if e.mem.u32(first_shape + 0xc) == 2 {
        e.mem.set_u8(result + 0xb, kept | 0x5);
    }
    let second_body = e.mem.u32(bodies + 4);
    let second_shape = e.mem.u32(second_body);
    if e.mem.u32(second_shape + 0xc) == 2 {
        let flags = e.mem.u8(result + 0xb);
        e.mem.set_u8(result + 0xb, flags | 0xa);
    }
    answer
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_0a10,
            replace_cylinders_with_capsules_in_cd_bodies(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d4_0be0, separating_distance_using_capsule_representations(Ptr, Ptr, Ptr) -> f32),
        entry!(0x00d4_0d80, pred_gsk_cylinder_agent3_process(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d4_0b90, fn_00d40b90(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a05_cylinder_agent_tests {
    use super::super::hkpsphereshape::{HALF_F32, PROFILE_SLOT, THREE_F32, TLS_GET_VALUE};
    use super::*;

    const ALLOCATE: u32 = 0x0900_0100;
    const POINTS_A: u32 = 0x0900_0200;
    const POINTS_B: u32 = 0x0900_0210;
    const PROCESS: u32 = 0x00d4_0d80;

    /// An engine with the constants mapped, a memory router whose allocator hands out heap
    /// blocks, a cylinder radius of `0.25`, capsules that are their own block, and a profiling
    /// stream (the second value).
    fn world() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_3000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
            0x0103_c000,
            0x010c_f000,
            0x0126_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f32(SELECT_DISTANCE, 0.01);
        e.mem.set_f64(CHECK_AGAIN_LIMIT, 0.1);
        e.mem.set_f64(SELECT_LIMIT, 0.05);
        e.mem.set_f64(CAPSULE_MARGIN, f64::from(0.002f32));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(CYLINDER_RADIUS, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        e.register(CAPSULE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0xe0);
        e.mem.set_u32(PROFILE_SLOT, 3);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, stream)
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A cylinder shape: `radius` at `+0x10`, the axis from `(0, 0, 0)` to `(0, 0, 2)` at
    /// `+0x20` and `+0x30`.
    fn shape(e: &mut Engine, radius: f32) -> u32 {
        let shape = e.mem.alloc(0x60);
        e.mem.set_f32(shape + 0x10, radius);
        store_vec(e, shape + 0x30, [0.0, 0.0, 2.0, 0.0]);
        shape
    }

    /// A body whose shape is `shape` and whose word 2 is `word2`.
    fn body(e: &mut Engine, shape: u32, word2: u32) -> u32 {
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, word2);
        body
    }

    #[test]
    fn replace_cylinders_swaps_the_flagged_cell_for_a_capsule_record() {
        let (mut e, _) = world();
        let shapes = [shape(&mut e, 0.5), shape(&mut e, 0.75)];
        let bodies = [
            body(&mut e, shapes[0], 0x1111),
            body(&mut e, shapes[1], 0x2222),
        ];
        let cells = e.mem.alloc(8);
        e.mem.set_u32(cells, bodies[0]);
        e.mem.set_u32(cells + 4, bodies[1]);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, cells);
        e.mem.set_u32(slots + 4, cells + 4);
        let flags = e.mem.alloc(8);
        e.mem.set_u32(flags, 1);
        let saved = e.mem.alloc(8);
        e.mem.set_u32(saved, 0xdead);
        e.mem.set_u32(saved + 4, 0xdead);
        let records = e.mem.alloc(0x20);
        e.call(0x00d4_0a10, &args![flags, saved, slots, records]);
        let ctor = calls_to(&e, CAPSULE_CONSTRUCT);
        assert_eq!(ctor.len(), 1);
        let block = ctor[0][0];
        let radius = ((0.25f64 + 0.5f64) + f64::from(0.002f32)) as f32;
        assert_eq!(
            ctor[0],
            vec![block, shapes[0] + 0x20, shapes[0] + 0x30, radius.to_bits()]
        );
        assert_eq!(e.mem.u16(block + 4), 0x40);
        assert_eq!(e.mem.u32(cells), records);
        assert_eq!(e.mem.u32(cells + 4), bodies[1]);
        assert_eq!(e.mem.u32(saved), bodies[0]);
        assert_eq!(e.mem.u32(saved + 4), 0);
        assert_eq!(e.mem.u32(records), block);
        assert_eq!(e.mem.u32(records + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(records + 8), 0x1111);
        assert_eq!(e.mem.u32(records + 0xc), bodies[0]);
    }

    #[test]
    fn separating_distance_measures_with_the_capsules_and_restores_the_bodies() {
        let (mut e, _) = world();
        let shapes = [shape(&mut e, 0.5), shape(&mut e, 0.75)];
        let bodies = [
            body(&mut e, shapes[0], 0x1111),
            body(&mut e, shapes[1], 0x2222),
        ];
        let agent = e.mem.alloc(0x100);
        for (offset, value) in [(0x60, 1), (0x70, 2), (0x74, 3), (0x78, 4), (0x7c, 5)] {
            e.mem.set_u32(agent + offset, value);
        }
        let entry = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0xb, 1);
        let seen = e.mem.alloc(0x100);
        e.register_double(BODY_CELLS, move |e, a| {
            e.mem.set_u32(a[0], bodies[0]);
            e.mem.set_u32(a[0] + 4, bodies[1]);
            Ret::default()
        });
        e.register_double(SEPARATING_NORMAL, move |e, a| {
            let record = e.mem.u32(a[0]);
            e.mem.set_u32(seen, record);
            e.mem.set_u32(seen + 4, e.mem.u32(record));
            e.mem.set_u32(seen + 8, e.mem.u32(a[0] + 4));
            for (index, offset) in [0x60, 0x70, 0x74, 0x78, 0x7c].into_iter().enumerate() {
                e.mem
                    .set_u32(seen + 0x10 + 4 * index as u32, e.mem.u32(a[0] + offset));
            }
            e.mem.set_f32(a[3] + 0xc, 0.375);
            Ret::default()
        });
        e.register(GSK_CACHE_INIT, |_, _| Ret::default());
        let got = e
            .call(SEPARATING_DISTANCE, &args![agent, entry, data])
            .f64();
        assert_eq!(got, 0.375);
        let capsule = calls_to(&e, CAPSULE_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(seen + 4), capsule);
        assert_eq!(e.mem.u32(seen + 8), bodies[1]);
        let copied: Vec<u32> = (0..5).map(|n| e.mem.u32(seen + 0x10 + 4 * n)).collect();
        assert_eq!(copied, vec![1, 2, 3, 4, 5]);
        let init = calls_to(&e, GSK_CACHE_INIT);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1..], [capsule, shapes[1], agent + 0x20]);
        let normal = calls_to(&e, SEPARATING_NORMAL);
        assert_eq!(normal[0][1], entry);
        assert_eq!(normal[0][2], init[0][0]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![capsule]]);
    }

    /// A world for `process`: two cylinder bodies (the axis points along z, each with a matrix:
    /// body A translates by `(10, 20, 30)`, body B scales by two), a collision input whose
    /// tolerance is `0.1`, parameters with a zero time-of-impact flag, a threshold of `0.5`,
    /// and doubles for every function the process calls. The separating vector the
    /// `sepNormal` double writes is `(0, 0, 1, 0.25)`, the direction the rotated-direction
    /// double writes `(0, 0, 1, 0)`.
    struct Rig {
        e: Engine,
        stream: u32,
        agent: u32,
        entry: u32,
        data: u32,
        normal: u32,
        output: u32,
        params: u32,
        collision_input: u32,
        shapes: [u32; 2],
        bodies: [u32; 2],
        direction: u32,
        seen: u32,
    }

    impl Rig {
        fn new() -> Rig {
            let (mut e, stream) = world();
            let rows_a = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [10.0, 20.0, 30.0, 0.0],
            ];
            let rows_b = [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0; 4],
            ];
            let mut shapes = [0; 2];
            let mut bodies = [0; 2];
            for (index, (rows, points)) in [(rows_a, POINTS_A), (rows_b, POINTS_B)]
                .into_iter()
                .enumerate()
            {
                let vtable = e.mem.alloc(0x80);
                e.mem.set_u32(vtable + 0x3c, points);
                shapes[index] = shape(&mut e, 0.5);
                e.mem.set_u32(shapes[index], vtable);
                let matrix = e.mem.alloc(0x40);
                for (row, v) in rows.into_iter().enumerate() {
                    store_vec(&mut e, matrix + 16 * row as u32, v);
                }
                bodies[index] = body(&mut e, shapes[index], matrix);
            }
            e.register(POINTS_A, |e, a| {
                for k in 0..a[2] {
                    store_vec(e, a[3] + 16 * k, [k as f32 + 1.0, 2.0, 3.0, 0.0]);
                }
                Ret::default()
            });
            e.register(POINTS_B, |e, a| {
                for k in 0..a[2] {
                    store_vec(e, a[3] + 16 * k, [k as f32 + 10.0, 2.0, 3.0, 0.0]);
                }
                Ret::default()
            });
            let params = e.mem.alloc(0x40);
            e.mem.set_f32(params + 0xc, 1.0);
            e.mem.set_f32(params + 0x18, 1.0);
            e.mem.set_f32(params + 0x1c, 0.5);
            e.mem.set_f32(params + 0x28, 0.5);
            e.mem.set_f32(params + 0x2c, 2.0);
            let collision_input = e.mem.alloc(0x80);
            e.mem.set_f32(collision_input + 8, 0.1);
            e.mem.set_u32(collision_input + 0x60, params);
            let agent = e.mem.alloc(0x100);
            e.mem.set_u32(agent, bodies[0]);
            e.mem.set_u32(agent + 4, bodies[1]);
            e.mem.set_u32(agent + 0xc, collision_input);
            e.mem.set_u32(agent + 0x10, 0x7777);
            e.mem.set_f32(agent + 0x60, 0.5);
            let output = e.mem.alloc(0x3100);
            e.mem.set_u32(output, 0x1000);
            e.mem.set_u32(output + 4, 0x4444);
            let vector = e.mem.alloc(0x10);
            store_vec(&mut e, vector, [0.0, 0.0, 1.0, 0.25]);
            let direction = e.mem.alloc(0x10);
            store_vec(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
            let seen = e.mem.alloc(0x100);
            e.register_double(SEPARATING_NORMAL, move |e, a| {
                let v = load_bits(e, vector);
                store_bits(e, a[3], v);
                Ret::default()
            });
            e.register_double(SET_ROTATED_DIR, move |e, a| {
                let input = load_bits(e, a[2]);
                store_bits(e, seen, input);
                let v = load_bits(e, direction);
                store_bits(e, a[0], v);
                Ret::default()
            });
            e.register(MANIFOLD_CLEANUP, |_, _| Ret::default());
            e.register(TIME_OF_IMPACT, |_, _| Ret::default());
            e.register(MANIFOLD_FINISH, |_, _| Ret::default());
            e.register_double(PROCESS_NO_TIME_OF_IMPACT, move |e, a| {
                for word in 0..3 {
                    e.mem
                        .set_u32(seen + 0x10 + 4 * word, e.mem.u32(a[3] + 4 * word));
                }
                e.mem.set_u32(seen + 0x20, e.mem.u32(a[0]));
                e.mem.set_u32(seen + 0x24, e.mem.u32(e.mem.u32(a[0])));
                Ret::default()
            });
            Rig {
                stream,
                agent,
                entry: e.mem.alloc(0x10),
                data: e.mem.alloc(0x40),
                normal: e.mem.alloc(0x10),
                output,
                params,
                collision_input,
                shapes,
                bodies,
                direction,
                seen,
                e,
            }
        }

        fn run(&mut self) -> u32 {
            self.e
                .call(
                    PROCESS,
                    &args![self.agent, self.entry, self.data, self.normal, self.output],
                )
                .u32()
        }

        fn calls(&self, address: u32) -> Vec<Vec<u32>> {
            calls_to(&self.e, address)
        }

        fn set_counts(&mut self, counts: [u8; 3]) {
            for (index, count) in counts.into_iter().enumerate() {
                self.e.mem.set_u8(self.data + 0xc + index as u32, count);
            }
        }
    }

    #[test]
    fn process_without_a_contact_path_copies_the_separating_vector() {
        let mut r = Rig::new();
        r.set_counts([1, 2, 3]);
        let got = r.run();
        assert_eq!(got, r.data + 0x30);
        assert_eq!(load_vec(&r.e, r.normal), [0.0, 0.0, 1.0, 0.25]);
        assert_eq!(r.e.mem.u8(r.entry + 2), 3);
        assert_eq!(
            r.calls(MANIFOLD_CLEANUP),
            vec![vec![r.data + 0xc, 0x7777, 0x4444]]
        );
        assert!(r.calls(TIME_OF_IMPACT).is_empty());
        let sep = r.calls(SEPARATING_NORMAL);
        assert_eq!(sep.len(), 1);
        assert_eq!(sep[0][..3], [r.agent, r.entry, r.data]);
        assert_eq!(r.e.mem.u32(r.stream + 4), r.stream + 0x20 + 24);
        assert_eq!(r.e.mem.u32(r.stream + 0x20), PROFILE_BEGIN);
        assert_eq!(r.e.mem.u32(r.stream + 0x2c), PROFILE_END);
    }

    #[test]
    fn process_with_no_contacts_does_not_clean_up() {
        let mut r = Rig::new();
        let got = r.run();
        assert_eq!(got, r.data + 0x10);
        assert!(r.calls(MANIFOLD_CLEANUP).is_empty());
        assert_eq!(r.e.mem.u8(r.entry + 2), 0);
    }

    #[test]
    fn process_runs_the_time_of_impact_step_when_the_limit_is_above_the_threshold() {
        let mut r = Rig::new();
        r.e.mem.set_u32(r.params + 0x10, 1);
        let roots = [r.e.mem.alloc(0x60), r.e.mem.alloc(0x60)];
        r.e.mem.set_f32(roots[0] + 0x4c, 2.0);
        r.e.mem.set_f32(roots[1] + 0x4c, 3.0);
        r.e.mem.set_u32(r.bodies[0] + 0xc, roots[0]);
        r.e.mem.set_u32(r.bodies[1] + 0xc, roots[1]);
        r.e.mem.set_f32(r.normal + 0xc, 0.5);
        r.run();
        assert_eq!(
            r.calls(TIME_OF_IMPACT),
            vec![vec![
                r.agent,
                2.0f32.to_bits(),
                1.5f32.to_bits(),
                1.0f32.to_bits(),
                r.data,
                r.normal,
                r.output
            ]]
        );
        assert_eq!(load_vec(&r.e, r.normal), [0.0, 0.0, 1.0, 0.25]);
    }

    #[test]
    fn process_skips_the_time_of_impact_step_when_the_limit_is_within_the_threshold() {
        let mut r = Rig::new();
        r.e.mem.set_u32(r.params + 0x10, 1);
        r.e.mem.set_f32(r.params + 0x18, 0.1);
        r.e.mem.set_f32(r.normal + 0xc, 0.5);
        r.run();
        assert!(r.calls(TIME_OF_IMPACT).is_empty());
        assert_eq!(load_vec(&r.e, r.normal), [0.0, 0.0, 1.0, 0.25]);
    }

    #[test]
    fn process_builds_the_contact_points_in_world_space() {
        let mut r = Rig::new();
        r.e.mem.set_f32(r.params + 0xc, 0.1);
        r.e.mem.set_f32(r.collision_input + 8, 1.0);
        r.set_counts([2, 1, 1]);
        let stack = r.e.mem.alloc(8);
        let cursor = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(stack + 4, cursor);
        r.e.mem.set_u32(r.output + 0x3080, stack);
        let (seen, data) = (r.seen, r.data);
        r.e.register_double(MANIFOLD_FINISH, move |e, a| {
            for word in 0..12 {
                e.mem
                    .set_u32(seen + 0x30 + 4 * word, e.mem.u32(a[1] + 4 * word));
            }
            e.mem.set_u8(data + 0xe, 2);
            Ret::default()
        });
        let got = r.run();
        assert_eq!(got, r.data + 0x30);
        assert_eq!(r.e.mem.f32(r.normal + 0xc), 0.5);
        let after = r.data + 0xc + 8 + 4;
        let a = r.calls(POINTS_A);
        let b = r.calls(POINTS_B);
        assert_eq!(a[0][..4], [r.shapes[0], after, 2, a[0][3]]);
        assert_eq!(b[0][..3], [r.shapes[1], after + 4, 1]);
        assert_eq!(b[0][3], a[0][3] + 32);
        let points: Vec<f32> = (0..12)
            .map(|n| r.e.mem.f32(r.seen + 0x30 + 4 * n))
            .collect();
        assert_eq!(
            points,
            vec![11.0, 22.0, 33.0, 0.0, 12.0, 22.0, 33.0, 0.0, 20.0, 4.0, 6.0, 0.0]
        );
        let finish = r.calls(MANIFOLD_FINISH);
        assert_eq!(finish.len(), 1);
        assert_eq!(finish[0][..1], [r.data + 0xc]);
        assert_eq!(finish[0][2..], [0, r.output, 0x7777]);
        assert_eq!(r.e.mem.u8(r.entry + 2), 2);
        assert_eq!(r.e.mem.u32(cursor), 0x1000 - 2 * 0x30);
        assert_eq!(r.e.mem.u32(stack + 4), cursor + 4);
    }

    #[test]
    fn process_with_a_count_of_zero_finishes_without_points() {
        let mut r = Rig::new();
        r.e.mem.set_f32(r.params + 0xc, 0.1);
        r.e.mem.set_f32(r.collision_input + 8, 1.0);
        r.run();
        assert_eq!(r.e.mem.f32(r.normal + 0xc), 0.5);
        assert!(r.calls(POINTS_A).is_empty());
        assert!(r.calls(MANIFOLD_FINISH).is_empty());
    }

    #[test]
    fn process_selects_the_capsule_and_gives_the_cache_the_flags() {
        let mut r = Rig::new();
        r.e.register(SEPARATING_DISTANCE, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        r.e.mem.set_f32(r.collision_input + 8, 1.0);
        r.e.mem.set_u8(r.data + 0xb, 1);
        r.e.mem.set_u8(r.data + 8, 3);
        r.e.mem.set_u32(r.data + 4, 0xaaaa_bbbb);
        r.run();
        assert_eq!(r.e.mem.u8(r.data + 0xb), 5);
        assert_eq!(
            r.calls(SEPARATING_DISTANCE),
            vec![vec![r.agent, r.entry, r.data]]
        );
        assert_eq!(
            r.calls(MANIFOLD_CLEANUP),
            vec![vec![r.data + 0xc, 0x7777, 0x4444]]
        );
        let axis = load_vec(&r.e, r.seen);
        for (lane, want) in [0.0, 0.0, 1.0, 0.0].into_iter().enumerate() {
            assert!((axis[lane] - want).abs() < 1e-5);
        }
        let no_time = r.calls(PROCESS_NO_TIME_OF_IMPACT);
        assert_eq!(no_time.len(), 1);
        assert_eq!(no_time[0][..3], [r.agent, r.entry, r.data]);
        assert_ne!(no_time[0][3], r.data);
        assert_eq!(no_time[0][4..], [r.data + 0xc, r.normal, r.output]);
        let words: Vec<u32> = (0..3).map(|n| r.e.mem.u32(r.seen + 0x10 + 4 * n)).collect();
        assert_eq!(words, vec![0x10, 0xaaaa_aaaa, 0x0500_0002]);
        // The body was a capsule record while the step ran, and is back afterwards.
        let record = r.e.mem.u32(r.seen + 0x20);
        assert_ne!(record, r.bodies[0]);
        assert_eq!(r.e.mem.u32(r.agent), r.bodies[0]);
        assert_eq!(
            r.calls(REMOVE_REFERENCE),
            vec![vec![r.e.mem.u32(r.seen + 0x24)]]
        );
        assert_eq!(load_vec(&r.e, r.normal), [0.0, 0.0, 1.0, 0.25]);
    }

    #[test]
    fn process_gives_the_second_body_cache_entry_the_flags() {
        let mut r = Rig::new();
        r.e.register(SEPARATING_DISTANCE, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        r.e.mem.set_f32(r.collision_input + 8, 1.0);
        r.e.mem.set_u8(r.data + 0xb, 2);
        r.e.mem.set_u16(r.data, 0x0080);
        r.e.mem.set_u8(r.data + 8, 1);
        r.e.mem.set_u8(r.data + 9, 3);
        r.run();
        assert_eq!(r.e.mem.u8(r.data + 0xb), 0x0a);
        let words: Vec<u32> = (0..3).map(|n| r.e.mem.u32(r.seen + 0x10 + 4 * n)).collect();
        assert_eq!(words, vec![0x0010_0080, 0, 0x0a00_0201]);
        assert_eq!(r.e.mem.u32(r.agent + 4), r.bodies[1]);
    }

    #[test]
    fn process_clears_the_selection_when_the_distance_is_negative() {
        let mut r = Rig::new();
        r.e.register(SEPARATING_DISTANCE, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        r.e.mem.set_u8(r.data + 0xb, 5);
        r.run();
        assert_eq!(r.e.mem.u8(r.data + 0xb), 1);
        assert_eq!(r.calls(SEPARATING_DISTANCE).len(), 1);
        assert_eq!(r.calls(MANIFOLD_CLEANUP).len(), 1);
    }

    #[test]
    fn process_keeps_the_selection_when_the_distance_is_not_negative() {
        let mut r = Rig::new();
        r.e.register(SEPARATING_DISTANCE, |_, _| Ret {
            st0: 0.0,
            ..Ret::default()
        });
        r.e.mem.set_u8(r.data + 0xb, 5);
        r.run();
        assert_eq!(r.e.mem.u8(r.data + 0xb), 5);
        assert!(r.calls(MANIFOLD_CLEANUP).is_empty());
    }

    #[test]
    fn process_selects_a_nearly_perpendicular_axis_without_asking_the_distance() {
        let mut r = Rig::new();
        store_vec(&mut r.e, r.direction, [1.0, 0.0, 0.0, 0.0]);
        r.e.mem.set_u8(r.data + 0xb, 1);
        r.run();
        assert_eq!(r.e.mem.u8(r.data + 0xb), 5);
        assert!(r.calls(SEPARATING_DISTANCE).is_empty());
        assert_eq!(r.calls(MANIFOLD_CLEANUP).len(), 1);
    }

    #[test]
    fn process_does_not_select_when_the_distance_is_small() {
        let mut r = Rig::new();
        r.e.register(SEPARATING_DISTANCE, |_, _| Ret {
            st0: 0.005,
            ..Ret::default()
        });
        r.e.mem.set_u8(r.data + 0xb, 1);
        r.run();
        assert_eq!(r.e.mem.u8(r.data + 0xb), 1);
        assert_eq!(r.calls(SEPARATING_DISTANCE).len(), 1);
        assert!(r.calls(MANIFOLD_CLEANUP).is_empty());
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// A pair of collidables whose shapes have the given type words.
    fn pair_of_shapes(e: &mut Engine, first_type: u32, second_type: u32) -> u32 {
        let pair = e.mem.alloc(8);
        for (slot, type_word) in [(0, first_type), (4, second_type)] {
            let body = e.mem.alloc(0x20);
            let shape = e.mem.alloc(0x20);
            e.mem.set_u32(shape + 0xc, type_word);
            e.mem.set_u32(body, shape);
            e.mem.set_u32(pair + slot, body);
        }
        pair
    }

    #[test]
    fn fn_00d40b90_sets_the_flag_bits_for_type_two_shapes() {
        for (first, second, expected) in [(2, 2, 0xf), (2, 3, 0x5), (3, 2, 0xa), (3, 3, 0x0)] {
            let mut e = Engine::new();
            e.register_double(0x00d4_18d0, |_, _| Ret {
                eax: 0x31,
                ..Ret::default()
            });
            let pair = pair_of_shapes(&mut e, first, second);
            let result = e.mem.alloc(0x20);
            e.mem.set_u8(result + 0xb, 0xbf);
            assert_eq!(
                e.call(0x00d4_0b90, &args![pair, 0x77u32, result]).u32(),
                0x31
            );
            assert_eq!(e.mem.u8(result + 0xb), 0xb0 | expected, "{first} {second}");
        }
    }

    /// The call happens first (it may change the byte), with the words in order.
    #[test]
    fn fn_00d40b90_calls_the_helper_with_its_words_before_masking() {
        let mut e = Engine::new();
        e.register_double(0x00d4_18d0, |e, a| {
            e.mem.set_u8(a[2] + 0xb, 0xff);
            assert_eq!(a[1], 0x77);
            Ret::default()
        });
        let pair = pair_of_shapes(&mut e, 3, 3);
        let result = e.mem.alloc(0x20);
        e.call(0x00d4_0b90, &args![pair, 0x77u32, result]);
        assert_eq!(e.mem.u8(result + 0xb), 0xf0);
    }
}
