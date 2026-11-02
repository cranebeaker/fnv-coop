//! `Havok SDK/hkpgskagentutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The function of this unit keeps its working data in a 0x1c4-byte frame; the translation
//! keeps that frame as a block of emulated stack (`with_stack`) and names its slots by their
//! offset from the stack pointer after the prologue (`f + offset`), because the exe's code
//! reads some slots through addresses that overlap others (see the first-group loop).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpgskmanifoldutil::{add, load_vec, mul, store_vec, Vec4};

/// `hkpGsk::getClosestPoint` (Xbox PDB, cdecl `(cache, simplex, normal, out)`; fills the
/// 16 bytes at `out` and answers 1 when the shapes do not touch).
const GET_CLOSEST_POINT: u32 = 0x00da_bca0;
/// `hkGskManifold_cleanup` (Xbox PDB, cdecl `(manifold, manager, flag)`).
const MANIFOLD_CLEANUP: u32 = 0x00da_d640;
/// `fn_00dad4d0` (no map name / unconfirmed, cdecl `(manifold, simplex)`): answers a byte, the
/// index of the manifold point that matches the simplex.
const FIND_POINT: u32 = 0x00da_d4d0;
/// `fn_00dae3a0` (no map name / unconfirmed, cdecl `(manifold, vertices, index, output,
/// manager)`).
const ADD_POINTS: u32 = 0x00da_e3a0;
/// `fn_00daf1e0` (no map name / unconfirmed, cdecl, ten arguments: `(manifold, body a, body b,
/// input, output, simplex, record, first record, manager, flag)`; answers the contact result).
const FIND_CONTACT: u32 = 0x00da_f1e0;
/// `hkpGskfAgent::removePoint` (Xbox PDB, cdecl `(manifold, index)`).
const REMOVE_POINT: u32 = 0x00da_e340;
/// The vector of sign bits for lanes 0 to 2 (`010c4c00`).
const NEGATE_XYZ_MASK: u32 = 0x010c_4c00;

/// XORs the first three lanes of the vector at `address` with the sign bits of the exe's mask.
fn negate_xyz(e: &mut Engine, address: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(address + 4 * lane) ^ e.mem.u32(NEGATE_XYZ_MASK + 4 * lane);
        e.mem.set_u32(address + 4 * lane, word);
    }
}

/// Transforms the vertices from `count - 1` down to 0 (the loop runs at least once, so a count
/// of 0 transforms the 16 bytes below the first vertex): `v.y * row1 + v.x * row0 + (v.z * row2
/// + row3)` with the four rows of the matrix at `matrix`, stored back in place. `top` is the
/// address of the vertex `count - 1`.
fn transform_vertices(e: &mut Engine, count: u32, top: u32, matrix: u32) {
    let rows: [Vec4; 4] = std::array::from_fn(|row| load_vec(e, matrix + 0x10 * row as u32));
    let mut index = count as i32 - 1;
    let mut address = top;
    loop {
        let v = load_vec(e, address);
        let first = add(mul([v[1]; 4], rows[1]), mul([v[0]; 4], rows[0]));
        let second = add(mul([v[2]; 4], rows[2]), rows[3]);
        store_vec(e, address, add(first, second));
        index -= 1;
        address = address.wrapping_sub(0x10);
        if index < 0 {
            break;
        }
    }
}

/// Copies the four words of a 16-byte vector bit for bit.
fn copy_words(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00db4180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGskAgentUtil_processCollisionNoTim` (Xbox PDB; cdecl, seven arguments).
///
/// `collision` is the pair: body a at `+0`, body b at `+4`, the collision input at `+0xc`, the
/// manager (an `hkpContactMgr`-like object with virtual functions at `0xc` and `0x10`) at
/// `+0x10` and the transform at `+0x20`; a body is `(shape, key, motion, parent)`.
/// `contact` has a flag byte at `+0xb` (bit 4 is cleared first), `simplex` is the cache
/// (vertex counts at `+8` and `+9`, vertex ids from `+8`), `manifold` is the point manifold
/// (`hkGskManifold`: counts at `+0` and `+1`, number of points at `+2`, points from `+4`),
/// `closest` is the normal vector (four floats; the fourth is the distance compared with the
/// limit) and `output` the collision output whose word `+0` is the record cursor (a record is
/// 0x30 bytes), `+4` is the flag the cleanup gets and `+0x3080` is the optional collector
/// (record cursor at `+0`, record array cursor at `+4`). `user_a` is stored in the
/// collector's records.
///
/// In order: clears bit 4 of the contact's flag byte; `getClosestPoint` over a five-word cache
/// block (transform, motion of body a, both shapes, the input's float at `+8`); an answer of 1
/// cleans up a non-empty manifold and ends. Otherwise the normal is copied; a shape of type 3
/// (body b's first, else body a's when its byte at `+0x16` is not 6, with the normal's first
/// three lanes negated for the call) is asked for its own closest result (virtual function
/// `0x40`): 0 cleans up and ends, 1 sets bit 4 of the contact flag. Then the manifold point
/// matching the simplex is looked up; when it is below the number of points, the shape
/// vertices of both groups are fetched (virtual function `0x3c`), moved by the bodies' motion
/// matrices (`v.y * row1 + v.x * row0 + (v.z * row2 + row3)`; the loops run at least once) and
/// the points are added. Then a record (the 16 bytes the closest point filled, then the
/// normal) is written at the output cursor: a found point (index not 0) stores the manifold's
/// id and advances the cursor; for index 0 the contact is searched when the limit chosen by the
/// simplex counts (`input + 0x60` array at `+4` for 4 vertices, else `+8`) exceeds the
/// distance, and its result either advances the cursor (4 with an id), asks the collector /
/// manager for an id (4 without), removes the point, restores the cursor (5, 6 with a retreat
/// of one record) or moves it back to a record (any other number, times 0x30). Finally, a
/// collector gets the record's address when it lies below the cursor.
pub fn hkgsk_agent_util_process_collision_no_tim(
    e: &mut Engine,
    collision: Ptr,
    user_a: u32,
    contact: Ptr,
    simplex: Ptr,
    manifold: Ptr,
    closest: Ptr,
    output: Ptr,
) {
    let me = collision.addr();
    let (contact, simplex) = (contact.addr(), simplex.addr());
    let (manifold, closest, output) = (manifold.addr(), closest.addr(), output.addr());
    e.with_stack(0x1c4, |e, frame| {
        // The frame's slots are addressed from the stack pointer after the prologue; the
        // block starts 0xc bytes above it.
        let f = frame.addr().wrapping_sub(0xc);
        let body_a = e.mem.u32(me);
        let body_b = e.mem.u32(me + 4);
        let shape_a = e.mem.u32(body_a);
        let shape_b = e.mem.u32(body_b);
        let input = e.mem.u32(me + 0xc);
        let manager = e.mem.u32(me + 0x10);
        let motion_a = e.mem.u32(body_a + 8);
        e.mem.set_u32(f + 0x94, shape_a);
        e.mem.set_u32(f + 0x98, shape_b);
        e.mem.set_u32(f + 0x8c, me + 0x20);
        e.mem.set_u32(f + 0x90, motion_a);
        let limit = x87_quiet_bits(e.mem.u32(input + 8));
        e.mem.set_u32(f + 0x9c, limit);
        let flags = e.mem.u8(contact + 0xb);
        e.mem.set_u8(contact + 0xb, flags & 0xef);
        let answer = e
            .call(
                GET_CLOSEST_POINT,
                &args![f + 0x8c, simplex, closest, f + 0xa0],
            )
            .u32();
        if answer == 1 {
            if e.mem.u8(manifold + 2) == 0 {
                return;
            }
            let flag = e.mem.u32(output + 4);
            e.call(MANIFOLD_CLEANUP, &args![manifold, manager, flag]);
            return;
        }
        copy_words(e, closest, f + 0x20);
        let mut verdict = None;
        if e.mem.u32(shape_b + 0xc) == 3 {
            let motion_b = e.mem.u32(e.mem.u32(me + 4) + 8);
            let motion_a = e.mem.u32(e.mem.u32(me) + 8);
            let vertices = simplex + u32::from(e.mem.u8(simplex + 8)) * 2;
            verdict = Some(
                e.vcall(
                    shape_b,
                    0x40,
                    &args![
                        vertices,
                        simplex + 9,
                        f + 0xa0,
                        motion_b,
                        shape_a,
                        motion_a,
                        f + 0x20
                    ],
                )
                .u32(),
            );
        } else if e.mem.u32(shape_a + 0xc) == 3 && e.mem.u8(shape_a + 0x16) != 6 {
            negate_xyz(e, f + 0x20);
            let motion_b = e.mem.u32(e.mem.u32(me + 4) + 8);
            let motion_a = e.mem.u32(e.mem.u32(me) + 8);
            verdict = Some(
                e.vcall(
                    shape_a,
                    0x40,
                    &args![
                        simplex,
                        simplex + 8,
                        f + 0xa0,
                        motion_a,
                        shape_b,
                        motion_b,
                        f + 0x20
                    ],
                )
                .u32(),
            );
            negate_xyz(e, f + 0x20);
        }
        if let Some(verdict) = verdict {
            if verdict == 0 {
                let flag = e.mem.u32(output + 4);
                e.call(MANIFOLD_CLEANUP, &args![manifold, manager, flag]);
                return;
            }
            if verdict == 1 {
                let flags = e.mem.u8(contact + 0xb);
                e.mem.set_u8(contact + 0xb, flags | 0x10);
            }
        }
        let mut index = e.call(FIND_POINT, &args![manifold, simplex]).u32() & 0xff;
        let first_record = e.mem.u32(output);
        e.mem.set_u32(f + 0x1c, first_record);
        let count = u32::from(e.mem.u8(manifold + 2));
        e.mem.set_u32(f + 0xc, index);
        e.mem.set_u32(f + 0x18, count);
        if count as i32 > index as i32 {
            let body_a = e.mem.u32(me);
            let body_b = e.mem.u32(me + 4);
            e.mem.set_u32(f + 0x3c, body_a);
            // The bounding radius sum and its square (locals the exe never reads again).
            let input = e.mem.u32(me + 0xc);
            let extra = x87_quiet_bits(e.mem.u32(input + 8));
            let shape_a = e.mem.u32(body_a);
            let radius_a = x87_quiet_bits(e.mem.u32(shape_a + 0x10));
            e.mem.set_u32(f + 0x14, body_b);
            let shape_b = e.mem.u32(body_b);
            e.mem.set_u32(f + 0x1c8, extra);
            e.mem.set_u32(f + 0x1c0, radius_a);
            let radius_b = x87_quiet_bits(e.mem.u32(shape_b + 0x10));
            e.mem.set_u32(f + 0x10, shape_b);
            e.mem.set_u32(f + 0x1c4, radius_b);
            copy_words(e, f + 0x20, f + 0x1b0);
            let sum = (f64::from(f32::from_bits(radius_a)) + f64::from(f32::from_bits(extra)))
                + f64::from(f32::from_bits(radius_b));
            let stored = sum as f32;
            e.mem.set_f32(f + 0x88, stored);
            let loaded = f64::from(stored);
            e.mem.set_f32(f + 0x1cc, (loaded * loaded) as f32);
            if count != 0 {
                let vertices = manifold + count * 8 + 4;
                let first_group = u32::from(e.mem.u8(manifold));
                e.mem.set_u32(f + 0x18, vertices);
                e.vcall(shape_a, 0x3c, &args![vertices, first_group, f + 0xb0]);
                let motion = e.mem.u32(e.mem.u32(f + 0x3c) + 8);
                let first_group = u32::from(e.mem.u8(manifold));
                let top = (f + 0xb0).wrapping_add(first_group.wrapping_sub(1) << 4);
                transform_vertices(e, first_group, top, motion);
                let second_buffer = (f + 0xb0).wrapping_add(first_group << 4);
                let second_group = u32::from(e.mem.u8(manifold + 1));
                let shape_b = e.mem.u32(f + 0x10);
                let vertices = e.mem.u32(f + 0x18);
                e.vcall(
                    shape_b,
                    0x3c,
                    &args![vertices + first_group * 2, second_group, second_buffer],
                );
                let motion = e.mem.u32(e.mem.u32(f + 0x14) + 8);
                let second_group = u32::from(e.mem.u8(manifold + 1));
                let top = second_buffer.wrapping_add(second_group.wrapping_sub(1) << 4);
                transform_vertices(e, second_group, top, motion);
            }
            let index_now = e.mem.u32(f + 0xc);
            e.call(
                ADD_POINTS,
                &args![manifold, f + 0xb0, index_now, output, manager],
            );
            index = e.mem.u32(f + 0xc);
        }
        // The record: the closest point's 16 bytes, then the normal.
        let record = e.mem.u32(output);
        copy_words(e, f + 0xa0, record);
        copy_words(e, f + 0x20, record + 0x10);
        let mut cursor = record;
        if index != 0 {
            let id = u32::from(e.mem.u16(manifold + 6));
            e.mem.set_u32(record + 0x20, id);
            let end = e.mem.u32(output);
            e.mem.set_u32(output, end.wrapping_add(0x30));
        } else {
            let sum = u32::from(e.mem.u8(simplex + 9)) + u32::from(e.mem.u8(simplex + 8));
            let input = e.mem.u32(me + 0xc);
            let limits = e.mem.u32(input + 0x60);
            let limit = e.mem.f32(limits + if sum == 4 { 4 } else { 8 });
            let distance = e.mem.f32(closest + 0xc);
            if limit > distance {
                let flag = u32::from(e.mem.u8(input + 0x6c) != 0);
                let body_a = e.mem.u32(me);
                let body_b = e.mem.u32(me + 4);
                let manager = e.mem.u32(me + 0x10);
                let result = e
                    .call(
                        FIND_CONTACT,
                        &args![
                            manifold,
                            body_a,
                            body_b,
                            input,
                            output,
                            simplex,
                            record,
                            first_record,
                            manager,
                            flag
                        ],
                    )
                    .i32();
                if result == 4 {
                    if e.mem.u32(record + 0x20) != 0xffff {
                        let end = e.mem.u32(output);
                        e.mem.set_u32(output, end.wrapping_add(0x30));
                    } else {
                        let collector = e.mem.u32(output + 0x3080);
                        let mut remove = true;
                        if collector != 0 && user_a != 0 {
                            let manager = e.mem.u32(me + 0x10);
                            if e.vcall(manager, 0x10, &args![1u32]).u32() == 0 {
                                let collector = e.mem.u32(output + 0x3080);
                                let slot = e.mem.u32(collector);
                                e.mem.set_u32(collector, slot + 0xc);
                                e.mem.set_u32(slot + 4, user_a);
                                e.mem.set_u32(slot, record);
                                e.mem.set_u32(slot + 8, contact);
                                let end = e.mem.u32(output);
                                e.mem.set_u32(output, end.wrapping_add(0x30));
                                remove = false;
                            }
                        } else {
                            let body_a = e.mem.u32(me);
                            let body_b = e.mem.u32(me + 4);
                            let input = e.mem.u32(me + 0xc);
                            let manager = e.mem.u32(me + 0x10);
                            let id = e
                                .vcall(
                                    manager,
                                    0xc,
                                    &args![body_a, body_b, input, output, simplex, record],
                                )
                                .u32()
                                & 0xffff;
                            e.mem.set_u32(record + 0x20, id);
                            if id != 0xffff {
                                e.mem.set_u16(manifold + 6, id as u16);
                                let end = e.mem.u32(output);
                                e.mem.set_u32(output, end.wrapping_add(0x30));
                                remove = false;
                            }
                        }
                        if remove {
                            e.call(REMOVE_POINT, &args![manifold, 0u32]);
                            cursor = e.mem.u32(f + 0x1c);
                        }
                    }
                } else if result == 5 {
                    cursor = e.mem.u32(f + 0x1c);
                } else if result == 6 {
                    let end = e.mem.u32(output);
                    e.mem.set_u32(output, end.wrapping_add(0xffff_ffd0));
                    cursor = e.mem.u32(f + 0x1c);
                } else {
                    cursor = (result as u32)
                        .wrapping_mul(0x30)
                        .wrapping_add(e.mem.u32(f + 0x1c));
                }
            }
        }
        let collector = e.mem.u32(output + 0x3080);
        if collector != 0 && cursor < e.mem.u32(output) {
            let slot = e.mem.u32(collector + 4);
            e.mem.set_u32(slot, cursor);
            let collector = e.mem.u32(output + 0x3080);
            let next = e.mem.u32(collector + 4).wrapping_add(4);
            e.mem.set_u32(collector + 4, next);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00db_4180,
        hkgsk_agent_util_process_collision_no_tim(Ptr, u32, Ptr, Ptr, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine as base_engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const SHAPE_A: u32 = 0x00f1_0000;
    const SHAPE_B: u32 = 0x00f2_0000;
    const MANAGER: u32 = 0x00f3_0000;
    /// The four floats the double of `getClosestPoint` writes.
    const RESULT_WORDS: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    type Seen = Rc<RefCell<Vec<u32>>>;

    struct World {
        collision: u32,
        body_a: u32,
        body_b: u32,
        shape_a: u32,
        shape_b: u32,
        motion_a: u32,
        motion_b: u32,
        input: u32,
        manager: u32,
        simplex: u32,
        manifold: u32,
        closest: u32,
        output: u32,
        contact: u32,
        start: u32,
    }

    fn engine() -> Engine {
        let mut e = base_engine();
        for lane in 0..3 {
            e.mem.set_u32(NEGATE_XYZ_MASK + 4 * lane, 0x8000_0000);
        }
        e
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|word| e.mem.u32(address + 4 * word))
            .collect()
    }

    /// A double for `getClosestPoint`: logs, remembers the five words of the cache block,
    /// writes [`RESULT_WORDS`] to the output and answers `result`.
    fn closest_double(e: &mut Engine, log: &Log, result: u32) -> Seen {
        let seen: Seen = Rc::default();
        let sink = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(GET_CLOSEST_POINT, move |e, a| {
            log.borrow_mut().push((GET_CLOSEST_POINT, a.to_vec()));
            *sink.borrow_mut() = words(e, a[0], 5);
            for (lane, value) in RESULT_WORDS.into_iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * lane as u32, value);
            }
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
        seen
    }

    /// A double at `address` (a virtual function of the shapes taking the normal as its last
    /// argument) that logs, remembers the normal at the time of the call and answers `answer`.
    fn penetration_double(e: &mut Engine, log: &Log, address: u32, answer: u32) -> Seen {
        let seen: Seen = Rc::default();
        let sink = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            *sink.borrow_mut() = words(e, a[7], 4);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        seen
    }

    /// A double at `address` (a shape's vertex fetch) that logs and writes `vertices` to the
    /// buffer, its last argument.
    fn vertex_double(e: &mut Engine, log: &Log, address: u32, vertices: Vec<Vec4>) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            for (index, vertex) in vertices.iter().enumerate() {
                store_vec(e, a[3] + 16 * index as u32, *vertex);
            }
            Ret::default()
        });
    }

    /// A double for the point adder that logs and remembers the first `count` vertices of its
    /// buffer.
    fn add_double(e: &mut Engine, log: &Log, count: u32) -> Rc<RefCell<Vec<Vec4>>> {
        let seen: Rc<RefCell<Vec<Vec4>>> = Rc::default();
        let sink = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(ADD_POINTS, move |e, a| {
            log.borrow_mut().push((ADD_POINTS, a.to_vec()));
            *sink.borrow_mut() = (0..count).map(|n| load_vec(e, a[1] + 16 * n)).collect();
            Ret::default()
        });
        seen
    }

    /// A double for the contact search: logs, stores `id` (when given) in the record's word
    /// `+0x20` and answers `result`.
    fn contact_double(e: &mut Engine, log: &Log, result: i32, id: Option<u32>) {
        let log = Rc::clone(log);
        e.register_double(FIND_CONTACT, move |e, a| {
            log.borrow_mut().push((FIND_CONTACT, a.to_vec()));
            if let Some(id) = id {
                e.mem.set_u32(a[6] + 0x20, id);
            }
            Ret {
                eax: result as u32,
                ..Ret::default()
            }
        });
    }

    /// An object with logging doubles for the virtual functions at `slots`.
    fn object(e: &mut Engine, log: &Log, base: u32, slots: &[u32]) -> u32 {
        let vtable = e.mem.alloc(0x50);
        for slot in slots {
            log_double(e, log, base + slot, 0);
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    fn shape(e: &mut Engine, log: &Log, base: u32, radius: f32) -> u32 {
        let shape = object(e, log, base, &[0x3c, 0x40]);
        e.mem.set_u32(shape + 0xc, 1);
        e.mem.set_f32(shape + 0x10, radius);
        shape
    }

    fn body(e: &mut Engine, shape: u32, motion: u32) -> u32 {
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        body
    }

    fn matrix(e: &mut Engine, rows: [Vec4; 4]) -> u32 {
        let matrix = e.mem.alloc(0x40);
        for (row, vector) in rows.into_iter().enumerate() {
            store_vec(e, matrix + 0x10 * row as u32, vector);
        }
        matrix
    }

    /// A pair of bodies whose motions scale by two and move by (1, 2, 3) (body a) and move
    /// by (100, 0, 0) (body b), shapes of type 1, a manager, a 1-1 simplex, an empty manifold,
    /// a normal (0, 1, 0, 0.5), an output whose cursor is `start` and doubles for every callee
    /// (all answering 0; the closest point is not 1).
    fn world(e: &mut Engine, log: &Log) -> World {
        closest_double(e, log, 0);
        for address in [
            MANIFOLD_CLEANUP,
            FIND_POINT,
            ADD_POINTS,
            FIND_CONTACT,
            REMOVE_POINT,
        ] {
            log_double(e, log, address, 0);
        }
        let shape_a = shape(e, log, SHAPE_A, 0.5);
        let shape_b = shape(e, log, SHAPE_B, 0.25);
        let manager = object(e, log, MANAGER, &[0xc, 0x10]);
        let motion_a = matrix(
            e,
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [1.0, 2.0, 3.0, 0.0],
            ],
        );
        let motion_b = matrix(
            e,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [100.0, 0.0, 0.0, 0.0],
            ],
        );
        let body_a = body(e, shape_a, motion_a);
        let body_b = body(e, shape_b, motion_b);
        let limits = e.mem.alloc(0x10);
        e.mem.set_f32(limits + 4, 1.0);
        e.mem.set_f32(limits + 8, 2.0);
        let input = e.mem.alloc(0x80);
        e.mem.set_f32(input + 8, 0.125);
        e.mem.set_u32(input + 0x60, limits);
        let collision = e.mem.alloc(0x30);
        e.mem.set_u32(collision, body_a);
        e.mem.set_u32(collision + 4, body_b);
        e.mem.set_u32(collision + 0xc, input);
        e.mem.set_u32(collision + 0x10, manager);
        let simplex = e.mem.alloc(0x10);
        e.mem.set_u8(simplex + 8, 1);
        e.mem.set_u8(simplex + 9, 1);
        let manifold = e.mem.alloc(0x80);
        let closest = e.mem.alloc(0x10);
        store_vec(e, closest, [0.0, 1.0, 0.0, 0.5]);
        let output = e.mem.alloc(0x3100);
        let start = e.mem.alloc(0x200);
        e.mem.set_u32(output, start);
        e.mem.set_u32(output + 4, 7);
        let contact = e.mem.alloc(0x10);
        World {
            collision,
            body_a,
            body_b,
            shape_a,
            shape_b,
            motion_a,
            motion_b,
            input,
            manager,
            simplex,
            manifold,
            closest,
            output,
            contact,
            start,
        }
    }

    fn run(e: &mut Engine, w: &World, user_a: u32) {
        e.call(
            0x00db_4180,
            &args![
                w.collision,
                user_a,
                w.contact,
                w.simplex,
                w.manifold,
                w.closest,
                w.output
            ],
        );
    }

    /// Gives the output a collector: record cursor `list`, array cursor `array`.
    fn collector(e: &mut Engine, w: &World) -> (u32, u32, u32) {
        let collector = e.mem.alloc(0x10);
        let list = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(collector, list);
        e.mem.set_u32(collector + 4, array);
        e.mem.set_u32(w.output + 0x3080, collector);
        (collector, list, array)
    }

    #[test]
    fn an_answer_of_one_with_an_empty_manifold_ends_without_cleanup() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        let seen = closest_double(&mut e, &log, 1);
        e.mem.set_u8(w.contact + 0xb, 0xff);
        run(&mut e, &w, 5);
        assert_eq!(e.mem.u8(w.contact + 0xb), 0xef);
        assert_eq!(
            *seen.borrow(),
            vec![
                w.collision + 0x20,
                w.motion_a,
                w.shape_a,
                w.shape_b,
                0.125f32.to_bits()
            ]
        );
        assert!(calls(&log, MANIFOLD_CLEANUP).is_empty());
        assert!(calls(&log, FIND_POINT).is_empty());
    }

    #[test]
    fn an_answer_of_one_cleans_up_a_manifold_with_points() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        closest_double(&mut e, &log, 1);
        e.mem.set_u8(w.manifold + 2, 2);
        run(&mut e, &w, 5);
        assert_eq!(
            calls(&log, MANIFOLD_CLEANUP),
            vec![vec![w.manifold, w.manager, 7]]
        );
        assert!(calls(&log, FIND_POINT).is_empty());
    }

    #[test]
    fn a_type_three_shape_b_is_asked_for_its_own_closest_result() {
        for (answer, cleaned, flagged) in [(0u32, true, false), (1, false, true), (2, false, false)]
        {
            let mut e = engine();
            let log = new_log();
            let w = world(&mut e, &log);
            e.mem.set_u32(w.shape_b + 0xc, 3);
            let normal = penetration_double(&mut e, &log, SHAPE_B + 0x40, answer);
            run(&mut e, &w, 0);
            let seen = calls(&log, SHAPE_B + 0x40);
            assert_eq!(seen.len(), 1);
            let a = &seen[0];
            assert_eq!(a[0], w.shape_b);
            assert_eq!(a[1], w.simplex + 2);
            assert_eq!(a[2], w.simplex + 9);
            assert_eq!(a[4..7], [w.motion_b, w.shape_a, w.motion_a]);
            assert_eq!(
                *normal.borrow(),
                vec![0, 1.0f32.to_bits(), 0, 0.5f32.to_bits()]
            );
            if cleaned {
                assert_eq!(
                    calls(&log, MANIFOLD_CLEANUP),
                    vec![vec![w.manifold, w.manager, 7]]
                );
                assert!(calls(&log, FIND_POINT).is_empty());
            } else {
                assert!(calls(&log, MANIFOLD_CLEANUP).is_empty());
                assert_eq!(calls(&log, FIND_POINT).len(), 1);
            }
            assert_eq!(e.mem.u8(w.contact + 0xb), if flagged { 0x10 } else { 0 });
        }
    }

    #[test]
    fn a_type_three_shape_a_gets_the_negated_normal_and_the_normal_is_restored() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        e.mem.set_u32(w.shape_a + 0xc, 3);
        log_double(&mut e, &log, FIND_POINT, 1);
        let normal = penetration_double(&mut e, &log, SHAPE_A + 0x40, 2);
        run(&mut e, &w, 0);
        let seen = calls(&log, SHAPE_A + 0x40);
        assert_eq!(seen.len(), 1);
        let a = &seen[0];
        assert_eq!(a[0], w.shape_a);
        assert_eq!(a[1], w.simplex);
        assert_eq!(a[2], w.simplex + 8);
        assert_eq!(a[4..7], [w.motion_a, w.shape_b, w.motion_b]);
        assert_eq!(
            *normal.borrow(),
            vec![
                0x8000_0000,
                (-1.0f32).to_bits(),
                0x8000_0000,
                0.5f32.to_bits()
            ]
        );
        // The copy of the normal in the record is back to its original sign.
        assert_eq!(
            words(&e, w.start + 0x10, 4),
            vec![0, 1.0f32.to_bits(), 0, 0.5f32.to_bits()]
        );
    }

    #[test]
    fn a_type_three_shape_a_with_byte_six_is_left_alone_and_a_cleanup_answer_ends() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        e.mem.set_u32(w.shape_a + 0xc, 3);
        e.mem.set_u8(w.shape_a + 0x16, 6);
        penetration_double(&mut e, &log, SHAPE_A + 0x40, 0);
        run(&mut e, &w, 0);
        assert!(calls(&log, SHAPE_A + 0x40).is_empty());
        assert_eq!(calls(&log, FIND_POINT).len(), 1);

        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        e.mem.set_u32(w.shape_a + 0xc, 3);
        penetration_double(&mut e, &log, SHAPE_A + 0x40, 0);
        run(&mut e, &w, 0);
        assert_eq!(
            calls(&log, MANIFOLD_CLEANUP),
            vec![vec![w.manifold, w.manager, 7]]
        );
        assert!(calls(&log, FIND_POINT).is_empty());
    }

    #[test]
    fn the_vertices_of_both_groups_are_moved_by_the_motions_and_added() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        e.mem.set_u8(w.manifold, 2);
        e.mem.set_u8(w.manifold + 1, 1);
        e.mem.set_u8(w.manifold + 2, 2);
        e.mem.set_u16(w.manifold + 6, 0x77);
        log_double(&mut e, &log, FIND_POINT, 1);
        vertex_double(
            &mut e,
            &log,
            SHAPE_A + 0x3c,
            vec![[1.0, 1.0, 1.0, 0.0], [2.0, 2.0, 2.0, 0.0]],
        );
        vertex_double(&mut e, &log, SHAPE_B + 0x3c, vec![[1.0, 2.0, 3.0, 0.0]]);
        let added = add_double(&mut e, &log, 3);
        run(&mut e, &w, 0);
        let ids = w.manifold + 2 * 8 + 4;
        let first = calls(&log, SHAPE_A + 0x3c);
        assert_eq!(first.len(), 1);
        let buffer = first[0][3];
        assert_eq!(first[0][..3], [w.shape_a, ids, 2]);
        assert_eq!(
            calls(&log, SHAPE_B + 0x3c),
            vec![vec![w.shape_b, ids + 4, 1, buffer + 0x20]]
        );
        assert_eq!(
            *added.borrow(),
            vec![
                [3.0, 4.0, 5.0, 0.0],
                [5.0, 6.0, 7.0, 0.0],
                [101.0, 2.0, 3.0, 0.0]
            ]
        );
        assert_eq!(
            calls(&log, ADD_POINTS),
            vec![vec![w.manifold, buffer, 1, w.output, w.manager]]
        );
        let expected: Vec<u32> = RESULT_WORDS.iter().map(|value| value.to_bits()).collect();
        assert_eq!(words(&e, w.start, 4), expected);
        assert_eq!(
            words(&e, w.start + 0x10, 4),
            vec![0, 1.0f32.to_bits(), 0, 0.5f32.to_bits()]
        );
        assert_eq!(e.mem.u32(w.start + 0x20), 0x77);
        assert_eq!(e.mem.u32(w.output), w.start + 0x30);
    }

    #[test]
    fn an_empty_first_group_transforms_the_closest_point_in_place() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        e.mem.set_u8(w.manifold + 1, 1);
        e.mem.set_u8(w.manifold + 2, 2);
        log_double(&mut e, &log, FIND_POINT, 1);
        vertex_double(&mut e, &log, SHAPE_B + 0x3c, vec![[1.0, 2.0, 3.0, 0.0]]);
        let added = add_double(&mut e, &log, 1);
        run(&mut e, &w, 0);
        // The loop over zero vertices still runs once, on the 16 bytes below the buffer,
        // which hold the closest point's result.
        let expected = [3.0f32, 6.0, 9.0, 0.0].map(f32::to_bits);
        assert_eq!(words(&e, w.start, 4), expected.to_vec());
        assert_eq!(*added.borrow(), vec![[101.0, 2.0, 3.0, 0.0]]);
    }

    #[test]
    fn a_collector_gets_the_record_that_lies_below_the_cursor() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        let (collector, _, array) = collector(&mut e, &w);
        log_double(&mut e, &log, FIND_POINT, 1);
        run(&mut e, &w, 0);
        assert_eq!(e.mem.u32(array), w.start);
        assert_eq!(e.mem.u32(collector + 4), array + 4);
    }

    #[test]
    fn the_contact_search_needs_the_distance_below_the_limit() {
        // The limit is the float at +8 of the input's table for 2 vertices, at +4 for 4.
        let cases = [
            ((1u8, 1u8), 1.5f32, true),
            ((1, 1), 2.0, false),
            ((1, 1), f32::NAN, false),
            ((2, 2), 1.5, false),
            ((2, 2), 0.5, true),
        ];
        for ((first, second), distance, searched) in cases {
            let mut e = engine();
            let log = new_log();
            let w = world(&mut e, &log);
            e.mem.set_u8(w.simplex + 8, first);
            e.mem.set_u8(w.simplex + 9, second);
            e.mem.set_f32(w.closest + 0xc, distance);
            e.mem.set_u8(w.input + 0x6c, 5);
            run(&mut e, &w, 0);
            let seen = calls(&log, FIND_CONTACT);
            assert_eq!(
                seen.len(),
                usize::from(searched),
                "{first} {second} {distance}"
            );
            if searched {
                assert_eq!(
                    seen[0],
                    vec![
                        w.manifold, w.body_a, w.body_b, w.input, w.output, w.simplex, w.start,
                        w.start, w.manager, 1
                    ]
                );
            }
        }
    }

    #[test]
    fn a_found_contact_with_an_id_advances_the_output() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        contact_double(&mut e, &log, 4, Some(0x1234));
        run(&mut e, &w, 0);
        assert_eq!(e.mem.u32(w.output), w.start + 0x30);
        assert_eq!(e.mem.u32(w.start + 0x20), 0x1234);
        assert!(calls(&log, REMOVE_POINT).is_empty());
    }

    #[test]
    fn a_contact_without_an_id_is_recorded_for_the_collector_when_the_manager_agrees() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        let (collector, list, array) = collector(&mut e, &w);
        contact_double(&mut e, &log, 4, Some(0xffff));
        run(&mut e, &w, 9);
        assert_eq!(calls(&log, MANAGER + 0x10), vec![vec![w.manager, 1]]);
        assert_eq!(words(&e, list, 3), vec![w.start, 9, w.contact]);
        assert_eq!(e.mem.u32(collector), list + 0xc);
        assert_eq!(e.mem.u32(w.output), w.start + 0x30);
        // The record lies below the new cursor, so the collector's array gets it too.
        assert_eq!(e.mem.u32(array), w.start);
        assert_eq!(e.mem.u32(collector + 4), array + 4);
        assert!(calls(&log, REMOVE_POINT).is_empty());
    }

    #[test]
    fn a_contact_without_an_id_is_removed_when_the_manager_refuses() {
        let mut e = engine();
        let log = new_log();
        let w = world(&mut e, &log);
        let (collector, list, array) = collector(&mut e, &w);
        log_double(&mut e, &log, MANAGER + 0x10, 1);
        contact_double(&mut e, &log, 4, Some(0xffff));
        run(&mut e, &w, 9);
        assert_eq!(calls(&log, REMOVE_POINT), vec![vec![w.manifold, 0]]);
        assert_eq!(e.mem.u32(w.output), w.start);
        assert_eq!(e.mem.u32(collector), list);
        assert_eq!(e.mem.u32(collector + 4), array);
    }

    #[test]
    fn a_contact_without_an_id_asks_the_manager_for_one() {
        for (answer, removed) in [(0x55u32, false), (0xffff, true), (0x1_0055, false)] {
            let mut e = engine();
            let log = new_log();
            let w = world(&mut e, &log);
            // A collector is present but the user value is 0, so the manager's id function
            // (not the collector path) is used.
            collector(&mut e, &w);
            log_double(&mut e, &log, MANAGER + 0xc, answer);
            contact_double(&mut e, &log, 4, Some(0xffff));
            run(&mut e, &w, 0);
            assert!(calls(&log, MANAGER + 0x10).is_empty());
            assert_eq!(
                calls(&log, MANAGER + 0xc),
                vec![vec![
                    w.manager, w.body_a, w.body_b, w.input, w.output, w.simplex, w.start
                ]]
            );
            let id = answer & 0xffff;
            assert_eq!(e.mem.u32(w.start + 0x20), id);
            if removed {
                assert_eq!(calls(&log, REMOVE_POINT), vec![vec![w.manifold, 0]]);
                assert_eq!(e.mem.u32(w.output), w.start);
            } else {
                assert!(calls(&log, REMOVE_POINT).is_empty());
                assert_eq!(u32::from(e.mem.u16(w.manifold + 6)), id);
                assert_eq!(e.mem.u32(w.output), w.start + 0x30);
            }
        }
    }

    #[test]
    fn the_other_contact_results_restore_or_move_the_cursor() {
        // 5 keeps the first record; 6 retreats one record; any other number moves the
        // cursor by that many records (a negative one lands below the output cursor and is
        // handed to the collector).
        let cases = [(5, 0u32), (6, 0), (2, 0), (-1, 1)];
        for (result, registered) in cases {
            let mut e = engine();
            let log = new_log();
            let w = world(&mut e, &log);
            let (collector, _, array) = collector(&mut e, &w);
            contact_double(&mut e, &log, result, None);
            run(&mut e, &w, 0);
            let end = if result == 6 { w.start - 0x30 } else { w.start };
            assert_eq!(e.mem.u32(w.output), end, "{result}");
            assert_eq!(e.mem.u32(collector + 4), array + 4 * registered, "{result}");
            if registered == 1 {
                assert_eq!(e.mem.u32(array), w.start - 0x30);
            }
        }
    }
}
