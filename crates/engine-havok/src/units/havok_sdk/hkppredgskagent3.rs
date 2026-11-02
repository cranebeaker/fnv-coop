//! `Havok SDK/hkppredgskagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{load_vec, profile_record, store_vec, Vec4, PROFILE_END};

/// The profiling label of `process`: the string `TtPredGskf3` (`010cf220`).
const PROFILE_BEGIN: u32 = 0x010c_f220;
/// Cleans up the GSK cache of an agent data block (cdecl `(cache, a, b)`).
const CACHE_CLEANUP: u32 = 0x00da_d640;
/// The two builders of the GSK cache (`00dafe70` when the second shape's type word at `+0xc`
/// is 3, otherwise `00dafcd0`; `this` = the cache, `(shape a, shape b, transform)`).
const GSK_INIT_TYPE_3: u32 = 0x00da_fe70;
const GSK_INIT_OTHER: u32 = 0x00da_fcd0;
/// The GSK solver (`this` = a local GSK object, `(shape a, shape b, transform, result)`) and
/// the cache refresh (`this` = the same object, `(cache)`).
const GSK_SOLVE: u32 = 0x00da_ad40;
const GSK_REFRESH_CACHE: u32 = 0x00da_9790;
/// Adds the manifold points of the cache (cdecl `(cache, points, 0, collector, manager)`).
const ADD_POINTS: u32 = 0x00da_e3a0;
/// `hk4dGskCollideCalcToi` (cdecl `(pair, float, float, float, data, input, collector)`).
const COLLIDE_CALC_TOI: u32 = 0x00db_60c0;
/// `hkGskAgentUtil::processCollisionNoTim` (cdecl, seven words).
const PROCESS_COLLISION_NO_TIM: u32 = 0x00db_4180;
/// Slot of the shape virtual function that writes the vertices of a cache (offset `0x3c`).
const SLOT_GET_VERTICES: u32 = 0x3c;

/// Offset (from the data block) of the cache: three counters (shape a vertices, shape b
/// vertices, entries) followed by flag bytes.
const CACHE: u32 = 0xc;

/// The address just behind the cache of a data block: the counters at `+0xc`, `+0xd` and
/// `+0xe` give `((b + 4 * c + a) * 2 + 0x1f) & !0xf` bytes from the block.
fn end_of_block(e: &Engine, block: u32) -> u32 {
    let first = u32::from(e.mem.u8(block + CACHE));
    let second = u32::from(e.mem.u8(block + CACHE + 1));
    let third = u32::from(e.mem.u8(block + CACHE + 2));
    (((second + third * 4 + first) * 2 + 0x1f) & 0xffff_fff0).wrapping_add(block)
}

// Translated from 00d41860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPredGskAgent3::cleanup` (engine map, cdecl, four words): cleans the cache at
/// `data + 0xc` (`00dad640(cache, a, b)`), clears the byte at `entry + 2` and returns the
/// address behind the cache (`end_of_block`).
pub fn hkp_pred_gsk_agent3_cleanup(e: &mut Engine, entry: Ptr, data: Ptr, a: u32, b: u32) -> u32 {
    let (entry, data) = (entry.addr(), data.addr());
    e.call(CACHE_CLEANUP, &args![data + CACHE, a, b]);
    let next = end_of_block(e, data);
    e.mem.set_u8(entry + 2, 0);
    next
}

// Translated from 00d418d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPredGskAgent3::create` (engine map, cdecl, three words): `pair` holds the two bodies
/// (`+0` and `+4`, whose first word is the shape) and the transform at `+0x20`. Builds the
/// GSK cache in `data` (`this` = `data`; `00dafe70` when the type word at `+0xc` of the second
/// shape is 3, otherwise `00dafcd0`; `(shape a, shape b, transform)`), clears the byte at
/// `entry + 2`, bit 4 of the byte at `data + 0xb` and the dword at `data + 0xc`, and returns
/// the address behind the cache.
pub fn hkp_pred_gsk_agent3_create(e: &mut Engine, pair: Ptr, entry: Ptr, data: Ptr) -> u32 {
    let (pair, entry, data) = (pair.addr(), entry.addr(), data.addr());
    let shape_a = e.mem.u32(e.mem.u32(pair));
    let shape_b = e.mem.u32(e.mem.u32(pair + 4));
    let builder = if e.mem.u32(shape_b + 0xc) == 3 {
        GSK_INIT_TYPE_3
    } else {
        GSK_INIT_OTHER
    };
    e.call(builder, &args![data, shape_a, shape_b, pair + 0x20]);
    e.mem.set_u8(entry + 2, 0);
    let flags = e.mem.u8(data + 0xb);
    e.mem.set_u8(data + 0xb, flags & 0xef);
    e.mem.set_u32(data + CACHE, 0);
    end_of_block(e, data)
}

// Translated from 00d41930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPredGskAgent3::sepNormal` (engine map, cdecl, four words; the second is not read):
/// takes the separating direction of the pair `pair` from the GSK cache `data` and writes it
/// to `out`. The vertices of the two shapes (counts at `data + 8` and `data + 9`, the low and
/// high nibble of the byte at `data + 0xa` go to the solver object) are fetched through the
/// shapes' virtual function at offset `0x3c` (`(source, count, destination)`), the solver
/// (`00daad40`, `this` = a local object of six dwords) runs with the transform at
/// `pair + 0x20` and, when it set the dword at `+0x14` of its object, the cache is refreshed
/// (`00da9790`). The solver's result vector is turned by the first body's transform
/// (`out = (y * column1 + x * column0) + z * column2`) and its fourth lane becomes the
/// result's fourth lane minus the two shape radii (floats at `+0x10`).
pub fn hkp_pred_gsk_agent3_sep_normal(
    e: &mut Engine,
    pair: Ptr,
    _unused: u32,
    data: Ptr,
    out: Ptr,
) {
    let (pair, data, out) = (pair.addr(), data.addr(), out.addr());
    // The frame's slots are addressed from the stack pointer after the prologue; the block
    // starts 0xc bytes above it.
    e.with_stack(0x194, |e, frame| {
        let f = frame.addr().wrapping_sub(0xc);
        let body_a = e.mem.u32(pair);
        let body_b = e.mem.u32(pair + 4);
        let shape_b = e.mem.u32(body_b);
        let shape_a = e.mem.u32(body_a);
        let count_a = u32::from(e.mem.u8(data + 8));
        let count_b = u32::from(e.mem.u8(data + 9));
        let flags = u32::from(e.mem.u8(data + 0xa));
        let solver = f + 0x20;
        e.mem.set_u32(solver, count_a);
        e.mem.set_u32(solver + 4, count_b);
        e.mem.set_u32(solver + 8, flags & 0xf);
        e.mem.set_u32(solver + 0xc, flags >> 4);
        e.mem.set_u32(solver + 0x10, 0);
        e.mem.set_u32(solver + 0x14, 0);
        e.vcall(shape_a, SLOT_GET_VERTICES, &args![data, count_a, f + 0x40]);
        e.vcall(
            shape_b,
            SLOT_GET_VERTICES,
            &args![data + count_a * 2, count_b, f + 0x100],
        );
        e.call(
            GSK_SOLVE,
            &args![solver, shape_a, shape_b, pair + 0x20, f + 0x10],
        );
        if e.mem.u32(solver + 0x14) != 0 {
            e.call(GSK_REFRESH_CACHE, &args![solver, data]);
        }
        let result = load_vec(e, f + 0x10);
        let fourth = x87_quiet(result[3]);
        let motion = e.mem.u32(e.mem.u32(pair) + 8);
        let column_0 = load_vec(e, motion);
        let column_1 = load_vec(e, motion + 0x10);
        let column_2 = load_vec(e, motion + 0x20);
        let turned: Vec4 = std::array::from_fn(|lane| {
            (result[1] * column_1[lane] + result[0] * column_0[lane]) + result[2] * column_2[lane]
        });
        store_vec(e, out, turned);
        let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
        let radius_a = x87_quiet(e.mem.f32(shape_a + 0x10));
        let gap = (f64::from(fourth) - (f64::from(radius_b) + f64::from(radius_a))) as f32;
        e.mem.set_u32(out + 0xc, gap.to_bits());
    });
}

/// The loop of `process` that moves the vertices `first` .. `first + 16 * count` by the
/// transform at `matrix` (four rows of four floats): every vertex becomes
/// `(y * row1 + x * row0) + (z * row2 + row3)`. Goes from the last vertex down to the first and,
/// as the exe does, runs the body once even for a count of 0 (then on the 16 bytes below
/// `first`).
fn transform_vertices(e: &mut Engine, matrix: u32, first: u32, count: u32) {
    let rows: [Vec4; 4] = std::array::from_fn(|row| load_vec(e, matrix + 0x10 * row as u32));
    let mut index = count as i32 - 1;
    let mut at = first.wrapping_add((index as u32) << 4);
    loop {
        let vertex = load_vec(e, at);
        let moved: Vec4 = std::array::from_fn(|lane| {
            (vertex[1] * rows[1][lane] + vertex[0] * rows[0][lane])
                + (vertex[2] * rows[2][lane] + rows[3][lane])
        });
        store_vec(e, at, moved);
        index -= 1;
        at = at.wrapping_sub(0x10);
        if index < 0 {
            break;
        }
    }
}

/// The root of the parent chain (the word at `+0xc`) that starts at `body`.
fn root_body(e: &Engine, body: u32) -> u32 {
    let mut root = body;
    loop {
        let parent = e.mem.u32(root + 0xc);
        if parent == 0 {
            return root;
        }
        root = parent;
    }
}

// Translated from 00d41a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPredGskAgent3::process` (engine map, cdecl `(pair, entry, data, input, collector)`).
/// Brackets its work with the profiling records `TtPredGskf3` and the end label.
///
/// `pair` holds the two bodies (`+0`, `+4`), the owner (`+0xc`; its word at `+0x60` is the
/// settings block), the manager (`+0x10`) and the time of impact (float at `+0x60`). With the
/// settings' dword at `+0x10` non-zero the smaller of the two root bodies' floats at `+0x4c`
/// (`m`) scales the settings' floats: `r = min(m * [+0x18], input[3] + [+0x1c] * m)`; a `r`
/// that is not at most the time of impact makes the exe call `hk4dGskCollideCalcToi` with
/// `(pair, m, r, min(m * [+0x28], input[3] + [+0x2c] * m), data, input, collector)` and then
/// `processCollisionNoTim`.
///
/// Otherwise, when the settings' float at `+0xc` is below the time of impact and bit 4 of the
/// byte at `data + 0xb` is clear, the time of impact becomes `input[3]` and, with a non-zero
/// entry counter at `data + 0xe`, the vertices of both shapes are fetched (shape virtual
/// function `0x3c`, `(source, count, destination)`) into a local array, moved by the two
/// bodies' transforms (`transform_vertices`), handed on with `00dae3a0` and the collector's
/// record `first - 0x30 * entries` is appended to the list at `collector + 0x3080`. In every
/// other case `processCollisionNoTim(pair, entry, data, data, data + 0xc, input, collector)`
/// runs. The entry's byte at `+2` gets the counter and the function returns the address behind
/// the cache. The squared radius sum and the copy of `input` that the exe keeps in locals are
/// never read and are left out.
pub fn hkp_pred_gsk_agent3_process(
    e: &mut Engine,
    pair: Ptr,
    entry: Ptr,
    data: Ptr,
    input: Ptr,
    collector: Ptr,
) -> u32 {
    let (pair, entry, data) = (pair.addr(), entry.addr(), data.addr());
    let (input, collector) = (input.addr(), collector.addr());
    profile_record(e, PROFILE_BEGIN);
    let cache = data + CACHE;
    let owner = e.mem.u32(pair + 0xc);
    let toi = x87_quiet(e.mem.f32(pair + 0x60));
    let settings = e.mem.u32(owner + 0x60);
    // Whether the vertex path is tried (it is unless the time-of-impact call was made).
    let vertex_path = if e.mem.u32(settings + 0x10) == 0 {
        true
    } else {
        let body_a = root_body(e, e.mem.u32(pair));
        let body_b = root_body(e, e.mem.u32(pair + 4));
        let first = x87_quiet(e.mem.f32(body_a + 0x4c));
        let second = x87_quiet(e.mem.f32(body_b + 0x4c));
        let scale = if second > first { first } else { second };
        let weight = x87_quiet(e.mem.f32(input + 0xc));
        let scaled = |factor: f32| f64::from(factor) * f64::from(scale);
        let upper = scaled(x87_quiet(e.mem.f32(settings + 0x18))) as f32;
        let lower = (scaled(x87_quiet(e.mem.f32(settings + 0x1c))) + f64::from(weight)) as f32;
        let bound = if lower > upper { upper } else { lower };
        if bound <= toi {
            true
        } else {
            let upper = scaled(x87_quiet(e.mem.f32(settings + 0x28))) as f32;
            let lower = (scaled(x87_quiet(e.mem.f32(settings + 0x2c))) + f64::from(weight)) as f32;
            let limit = if lower > upper { upper } else { lower };
            e.call(
                COLLIDE_CALC_TOI,
                &args![
                    pair,
                    scale.to_bits(),
                    bound.to_bits(),
                    limit.to_bits(),
                    data,
                    input,
                    collector
                ],
            );
            false
        }
    };
    let mut no_tim = true;
    if vertex_path {
        let threshold = x87_quiet(e.mem.f32(settings + 0xc));
        if threshold < toi && e.mem.u8(data + 0xb) & 0x10 == 0 {
            no_tim = false;
            e.mem.set_u32(input + 0xc, toi.to_bits());
            let entries = u32::from(e.mem.u8(data + CACHE + 2));
            if entries != 0 {
                collide_vertices(e, pair, data, entries, collector);
            }
        }
    }
    if no_tim {
        e.call(
            PROCESS_COLLISION_NO_TIM,
            &args![pair, entry, data, data, cache, input, collector],
        );
    }
    let entries = e.mem.u8(data + CACHE + 2);
    e.mem.set_u8(entry + 2, entries);
    let next = end_of_block(e, data);
    profile_record(e, PROFILE_END);
    next
}

/// The vertex path of `process` for a non-zero entry counter `entries`.
fn collide_vertices(e: &mut Engine, pair: u32, data: u32, entries: u32, collector: u32) {
    // The local array starts at 0x70 above the stack pointer after the prologue; the frame
    // block starts 0xc bytes above it.
    e.with_stack(0x184, |e, frame| {
        let f = frame.addr().wrapping_sub(0xc);
        let body_a = e.mem.u32(pair);
        let body_b = e.mem.u32(pair + 4);
        let shape_a = e.mem.u32(body_a);
        let shape_b = e.mem.u32(body_b);
        let count_a = u32::from(e.mem.u8(data + CACHE));
        let count_b = u32::from(e.mem.u8(data + CACHE + 1));
        let source = data + 0x10 + entries * 8;
        // The second body is kept in a frame slot that a count of 0 would overwrite.
        e.mem.set_u32(f + 0x6c, body_b);
        e.vcall(
            shape_a,
            SLOT_GET_VERTICES,
            &args![source, count_a, f + 0x70],
        );
        let motion_a = e.mem.u32(body_a + 8);
        transform_vertices(e, motion_a, f + 0x70, count_a);
        let second_array = f + 0x70 + count_a * 0x10;
        e.vcall(
            shape_b,
            SLOT_GET_VERTICES,
            &args![source + count_a * 2, count_b, second_array],
        );
        let body_b = e.mem.u32(f + 0x6c);
        let motion_b = e.mem.u32(body_b + 8);
        transform_vertices(e, motion_b, second_array, count_b);
        let manager = e.mem.u32(pair + 0x10);
        e.call(
            ADD_POINTS,
            &args![data + CACHE, f + 0x70, 0u32, collector, manager],
        );
    });
    let list = e.mem.u32(collector + 0x3080);
    if list != 0 {
        let record = e.mem.u32(collector).wrapping_sub(entries * 0x30);
        let cursor = e.mem.u32(list + 4);
        e.mem.set_u32(cursor, record);
        e.mem.set_u32(list + 4, cursor.wrapping_add(4));
    }
}

// Translated from 00d418b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: four stack words (cdecl, nothing popped), passed on in the same
/// order to `00d41860`, whose answer (`EAX`) it returns. Ghidra made no function here; the address is an immediate operand at 00d41776.
pub fn fn_00d418b0(e: &mut Engine, a: u32, b: u32, c: u32, d: u32) -> u32 {
    e.call(0x00d4_1860, &args![a, b, c, d]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_1860,
            hkp_pred_gsk_agent3_cleanup(Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00d4_18d0,
            hkp_pred_gsk_agent3_create(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00d4_1930,
            hkp_pred_gsk_agent3_sep_normal(Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00d4_1a30,
            hkp_pred_gsk_agent3_process(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x00d4_18b0, fn_00d418b0(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Seen<T> = Rc<RefCell<Vec<T>>>;

    /// Stand-ins for the shapes' vertex functions (slot `0x3c`).
    const FAKE_VERTICES_A: u32 = 0x0900_0501;
    const FAKE_VERTICES_B: u32 = 0x0900_0502;
    /// The manager word of the pair (`pair + 0x10`).
    const MANAGER: u32 = 0x1234_5678;

    const CLEANUP: u32 = 0x00d4_1860;
    const CREATE: u32 = 0x00d4_18d0;
    const SEP_NORMAL: u32 = 0x00d4_1930;
    const PROCESS: u32 = 0x00d4_1a30;

    fn calls(log: &Log, address: u32) -> Vec<&[u32]> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.as_slice())
            .collect()
    }

    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.map(PROFILE_SLOT & !0xfff, 0x1000);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    /// A shape whose vertex function is `slot`, a motion with the rows `rows` and a body that
    /// points at both. Returns the body.
    fn body(e: &mut Engine, slot: u32, rows: [Vec4; 4]) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + SLOT_GET_VERTICES, slot);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape, table);
        let motion = e.mem.alloc(0x40);
        for (index, row) in rows.into_iter().enumerate() {
            store_vec(e, motion + 0x10 * index as u32, row);
        }
        let body = e.mem.alloc(0x60);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        body
    }

    /// The vertex function writes `count` vertices `(seed + index, 0, 0, 9)`.
    fn vertex_source(e: &mut Engine, slot: u32, seed: f32) {
        e.register_double(slot, move |e, a| {
            for index in 0..a[2] {
                store_vec(e, a[3] + 0x10 * index, [seed + index as f32, 0.0, 0.0, 9.0]);
            }
            Ret::default()
        });
    }

    struct Rig {
        e: Engine,
        pair: u32,
        entry: u32,
        data: u32,
        input: u32,
        collector: u32,
        settings: u32,
        body_a: u32,
        body_b: u32,
        stream: u32,
        cursor: u32,
        added: Seen<Vec<Vec4>>,
    }

    /// A pair of bodies (A moves its vertices by `(10, 20, 30)`, B doubles them), vertex
    /// counts 2 and 1, 3 entries, time of impact 0.75, threshold 0.5 and the settings flag
    /// at `+0x10` clear.
    fn rig() -> Rig {
        let mut e = Engine::new();
        let stream = profile_stream(&mut e);
        let body_a = body(
            &mut e,
            FAKE_VERTICES_A,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [10.0, 20.0, 30.0, 1.0],
            ],
        );
        let body_b = body(
            &mut e,
            FAKE_VERTICES_B,
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        );
        vertex_source(&mut e, FAKE_VERTICES_A, 1.0);
        vertex_source(&mut e, FAKE_VERTICES_B, 5.0);
        let owner = e.mem.alloc(0x80);
        let settings = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x60, settings);
        e.mem.set_f32(settings + 0xc, 0.5);
        let pair = e.mem.alloc(0x80);
        e.mem.set_u32(pair, body_a);
        e.mem.set_u32(pair + 4, body_b);
        e.mem.set_u32(pair + 0xc, owner);
        e.mem.set_u32(pair + 0x10, MANAGER);
        e.mem.set_f32(pair + 0x60, 0.75);
        let entry = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x100);
        e.mem.set_u8(data + 0xc, 2);
        e.mem.set_u8(data + 0xd, 1);
        e.mem.set_u8(data + 0xe, 3);
        let input = e.mem.alloc(0x10);
        e.mem.set_f32(input + 0xc, 1.0);
        let collector = e.mem.alloc(0x3100);
        e.mem.set_u32(collector, 0x1000);
        let list = e.mem.alloc(0x10);
        let cursor = e.mem.alloc(0x40);
        e.mem.set_u32(list + 4, cursor);
        e.mem.set_u32(collector + 0x3080, list);
        let added: Seen<Vec<Vec4>> = Rc::default();
        let sink = added.clone();
        e.register_double(ADD_POINTS, move |e, a| {
            let total = u32::from(e.mem.u8(a[0])) + u32::from(e.mem.u8(a[0] + 1));
            let points = (0..total).map(|i| load_vec(e, a[1] + 0x10 * i)).collect();
            sink.borrow_mut().push(points);
            Ret::default()
        });
        e.register_double(COLLIDE_CALC_TOI, |_, _| Ret::default());
        e.register_double(PROCESS_COLLISION_NO_TIM, |_, _| Ret::default());
        Rig {
            e,
            pair,
            entry,
            data,
            input,
            collector,
            settings,
            body_a,
            body_b,
            stream,
            cursor,
            added,
        }
    }

    impl Rig {
        fn run(&mut self) -> (u32, Log) {
            self.e.call_log = Some(vec![]);
            let next = self
                .e
                .call(
                    PROCESS,
                    &args![self.pair, self.entry, self.data, self.input, self.collector],
                )
                .u32();
            (next, self.e.call_log.take().unwrap())
        }

        fn plain_call(&self) -> Vec<u32> {
            vec![
                self.pair,
                self.entry,
                self.data,
                self.data,
                self.data + 0xc,
                self.input,
                self.collector,
            ]
        }
    }

    #[test]
    fn the_vertex_path_moves_the_vertices_and_queues_the_record() {
        let mut r = rig();
        let (next, log) = r.run();
        assert_eq!(next, r.data + 0x30);
        assert_eq!(r.e.mem.f32(r.input + 0xc), 0.75);
        assert_eq!(r.e.mem.u8(r.entry + 2), 3);
        let shape_a = r.e.mem.u32(r.body_a);
        let shape_b = r.e.mem.u32(r.body_b);
        let first = calls(&log, FAKE_VERTICES_A);
        let second = calls(&log, FAKE_VERTICES_B);
        assert_eq!(first.len(), 1);
        assert_eq!(second.len(), 1);
        assert_eq!(first[0][..3], [shape_a, r.data + 0x28, 2]);
        assert_eq!(second[0][..3], [shape_b, r.data + 0x2c, 1]);
        assert_eq!(second[0][3], first[0][3] + 0x20);
        let added = calls(&log, ADD_POINTS);
        assert_eq!(
            added,
            [[r.data + 0xc, first[0][3], 0, r.collector, MANAGER]]
        );
        assert_eq!(
            *r.added.borrow(),
            [vec![
                [11.0, 20.0, 30.0, 1.0],
                [12.0, 20.0, 30.0, 1.0],
                [10.0, 0.0, 0.0, 1.0]
            ]]
        );
        assert!(calls(&log, PROCESS_COLLISION_NO_TIM).is_empty());
        assert!(calls(&log, COLLIDE_CALC_TOI).is_empty());
        assert_eq!(r.e.mem.u32(r.cursor), 0x1000 - 3 * 0x30);
        let list = r.e.mem.u32(r.collector + 0x3080);
        assert_eq!(r.e.mem.u32(list + 4), r.cursor + 4);
        assert_eq!(r.e.mem.u32(r.stream + 0x20), PROFILE_BEGIN);
        assert_eq!(r.e.mem.u32(r.stream + 0x2c), PROFILE_END);
        assert_eq!(r.e.mem.u32(r.stream + 4), r.stream + 0x38);
    }

    #[test]
    fn without_a_record_list_the_points_are_still_added() {
        let mut r = rig();
        r.e.mem.set_u32(r.collector + 0x3080, 0);
        let (next, log) = r.run();
        assert_eq!(next, r.data + 0x30);
        assert_eq!(calls(&log, ADD_POINTS).len(), 1);
        assert_eq!(r.e.mem.u32(r.cursor), 0);
    }

    #[test]
    fn a_threshold_that_is_not_below_the_time_of_impact_takes_the_plain_path() {
        let mut r = rig();
        r.e.mem.set_f32(r.settings + 0xc, 0.75);
        let (next, log) = r.run();
        assert_eq!(next, r.data + 0x30);
        assert_eq!(calls(&log, PROCESS_COLLISION_NO_TIM), [r.plain_call()]);
        assert!(calls(&log, ADD_POINTS).is_empty());
        assert!(calls(&log, FAKE_VERTICES_A).is_empty());
        assert_eq!(r.e.mem.f32(r.input + 0xc), 1.0);
        assert_eq!(r.e.mem.u8(r.entry + 2), 3);
        assert_eq!(r.e.mem.u32(r.stream + 0x2c), PROFILE_END);
    }

    #[test]
    fn bit_four_of_the_cache_flags_takes_the_plain_path() {
        let mut r = rig();
        r.e.mem.set_u8(r.data + 0xb, 0x10);
        let (_, log) = r.run();
        assert_eq!(calls(&log, PROCESS_COLLISION_NO_TIM), [r.plain_call()]);
        assert!(calls(&log, ADD_POINTS).is_empty());
        let mut r = rig();
        r.e.mem.set_u8(r.data + 0xb, 0xef);
        let (_, log) = r.run();
        assert!(calls(&log, PROCESS_COLLISION_NO_TIM).is_empty());
        assert_eq!(calls(&log, ADD_POINTS).len(), 1);
    }

    #[test]
    fn without_entries_only_the_time_of_impact_is_stored() {
        let mut r = rig();
        r.e.mem.set_u8(r.data + 0xe, 0);
        r.e.mem.set_u8(r.entry + 2, 7);
        let (next, log) = r.run();
        assert_eq!(next, r.data + 0x20);
        assert_eq!(r.e.mem.f32(r.input + 0xc), 0.75);
        assert_eq!(r.e.mem.u8(r.entry + 2), 0);
        assert!(calls(&log, FAKE_VERTICES_A).is_empty());
        assert!(calls(&log, ADD_POINTS).is_empty());
        assert!(calls(&log, PROCESS_COLLISION_NO_TIM).is_empty());
        assert_eq!(r.e.mem.u32(r.cursor), 0);
    }

    /// Scale factors 0.5, 0.25, 3.0 and 0.5 in the settings; the roots floats at `+0x4c`.
    fn timed_rig(first: f32, second: f32, toi: f32) -> Rig {
        let mut r = rig();
        r.e.mem.set_u32(r.settings + 0x10, 1);
        r.e.mem.set_f32(r.settings + 0x18, 0.5);
        r.e.mem.set_f32(r.settings + 0x1c, 0.25);
        r.e.mem.set_f32(r.settings + 0x28, 3.0);
        r.e.mem.set_f32(r.settings + 0x2c, 0.5);
        r.e.mem.set_f32(r.body_a + 0x4c, first);
        r.e.mem.set_f32(r.body_b + 0x4c, second);
        r.e.mem.set_f32(r.pair + 0x60, toi);
        r
    }

    fn bits(values: [f32; 3]) -> [u32; 3] {
        values.map(f32::to_bits)
    }

    #[test]
    fn a_late_impact_calls_the_time_of_impact_routine_then_the_plain_one() {
        for (first, second) in [(2.0, 4.0), (4.0, 2.0)] {
            let mut r = timed_rig(first, second, 0.5);
            let (next, log) = r.run();
            assert_eq!(next, r.data + 0x30);
            // m = 2: bound = min(0.5 * 2, 1 + 0.25 * 2) = 1, limit = min(3 * 2, 1 + 0.5 * 2) = 2.
            let words = bits([2.0, 1.0, 2.0]);
            let expected = vec![
                r.pair,
                words[0],
                words[1],
                words[2],
                r.data,
                r.input,
                r.collector,
            ];
            assert_eq!(calls(&log, COLLIDE_CALC_TOI), [expected]);
            assert_eq!(calls(&log, PROCESS_COLLISION_NO_TIM), [r.plain_call()]);
            let order: Vec<u32> = log
                .iter()
                .map(|call| call.0)
                .filter(|a| [COLLIDE_CALC_TOI, PROCESS_COLLISION_NO_TIM].contains(a))
                .collect();
            assert_eq!(order, [COLLIDE_CALC_TOI, PROCESS_COLLISION_NO_TIM]);
            assert!(calls(&log, ADD_POINTS).is_empty());
            assert_eq!(r.e.mem.f32(r.input + 0xc), 1.0);
            assert_eq!(r.e.mem.u8(r.entry + 2), 3);
        }
    }

    #[test]
    fn the_scale_comes_from_the_roots_of_the_parent_chains() {
        let mut r = timed_rig(9.0, 4.0, 0.1);
        let parent = r.e.mem.alloc(0x60);
        let root = r.e.mem.alloc(0x60);
        r.e.mem.set_u32(r.body_a + 0xc, parent);
        r.e.mem.set_u32(parent + 0xc, root);
        r.e.mem.set_f32(parent + 0x4c, 100.0);
        r.e.mem.set_f32(root + 0x4c, 1.0);
        let (_, log) = r.run();
        // m = 1: bound = min(0.5, 1.25) = 0.5, limit = min(3, 1.5) = 1.5.
        let words = bits([1.0, 0.5, 1.5]);
        let call = calls(&log, COLLIDE_CALC_TOI);
        assert_eq!(call.len(), 1);
        assert_eq!(call[0][1..4], words);
    }

    #[test]
    fn a_bound_within_the_time_of_impact_takes_the_vertex_path() {
        let mut r = timed_rig(2.0, 4.0, 1.0);
        let (next, log) = r.run();
        assert_eq!(next, r.data + 0x30);
        assert!(calls(&log, COLLIDE_CALC_TOI).is_empty());
        assert!(calls(&log, PROCESS_COLLISION_NO_TIM).is_empty());
        assert_eq!(calls(&log, ADD_POINTS).len(), 1);
        assert_eq!(r.e.mem.f32(r.input + 0xc), 1.0);
    }

    #[test]
    fn a_time_of_impact_that_is_not_a_number_is_never_stored() {
        let mut r = rig();
        r.e.mem.set_u32(r.pair + 0x60, 0x7f80_0001);
        let (_, log) = r.run();
        assert_eq!(calls(&log, PROCESS_COLLISION_NO_TIM), [r.plain_call()]);
        assert_eq!(r.e.mem.f32(r.input + 0xc), 1.0);
        let mut r = timed_rig(2.0, 4.0, f32::NAN);
        let (_, log) = r.run();
        assert_eq!(calls(&log, COLLIDE_CALC_TOI).len(), 1);
        assert_eq!(calls(&log, PROCESS_COLLISION_NO_TIM).len(), 1);
    }

    #[test]
    fn the_transform_loop_runs_once_for_a_count_of_zero() {
        let mut e = Engine::new();
        let matrix = e.mem.alloc(0x40);
        let rows: [Vec4; 4] = [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [1.0, 1.0, 1.0, 1.0],
        ];
        for (index, row) in rows.into_iter().enumerate() {
            store_vec(&mut e, matrix + 0x10 * index as u32, row);
        }
        let array = e.mem.alloc(0x30) + 0x10;
        store_vec(&mut e, array - 0x10, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, array, [5.0, 6.0, 7.0, 8.0]);
        transform_vertices(&mut e, matrix, array, 0);
        assert_eq!(load_vec(&e, array - 0x10), [3.0, 5.0, 7.0, 1.0]);
        assert_eq!(load_vec(&e, array), [5.0, 6.0, 7.0, 8.0]);
        transform_vertices(&mut e, matrix, array, 1);
        assert_eq!(load_vec(&e, array), [11.0, 13.0, 15.0, 1.0]);
    }

    #[test]
    fn cleanup_cleans_the_cache_and_returns_the_address_behind_it() {
        let mut e = Engine::new();
        let entry = e.mem.alloc(0x10);
        e.mem.set_u8(entry + 2, 9);
        let data = e.mem.alloc(0x100);
        e.register_double(CACHE_CLEANUP, move |e, a| {
            assert_eq!((a[0], a[1], a[2]), (data + 0xc, 0x11, 0x22));
            e.mem.set_u8(a[0], 2);
            e.mem.set_u8(a[0] + 1, 1);
            e.mem.set_u8(a[0] + 2, 3);
            Ret::default()
        });
        let next = e.call(CLEANUP, &args![entry, data, 0x11u32, 0x22u32]).u32();
        assert_eq!(next, data + 0x30);
        assert_eq!(e.mem.u8(entry + 2), 0);
    }

    /// A pair of bodies whose second shape has the type word `shape_type`; returns the pair
    /// and the two shapes.
    fn shape_pair(e: &mut Engine, shape_type: u32) -> (u32, u32, u32) {
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        e.mem.set_u32(shape_b + 0xc, shape_type);
        let body_a = e.mem.alloc(0x10);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_a, shape_a);
        e.mem.set_u32(body_b, shape_b);
        let pair = e.mem.alloc(0x40);
        e.mem.set_u32(pair, body_a);
        e.mem.set_u32(pair + 4, body_b);
        (pair, shape_a, shape_b)
    }

    #[test]
    fn create_picks_the_cache_builder_by_the_second_shape_type() {
        for (shape_type, used, unused) in [
            (3, GSK_INIT_TYPE_3, GSK_INIT_OTHER),
            (4, GSK_INIT_OTHER, GSK_INIT_TYPE_3),
        ] {
            let mut e = Engine::new();
            let (pair, shape_a, shape_b) = shape_pair(&mut e, shape_type);
            let entry = e.mem.alloc(0x10);
            e.mem.set_u8(entry + 2, 5);
            let data = e.mem.alloc(0x40);
            e.mem.set_u8(data + 0xb, 0xff);
            for builder in [used, unused] {
                e.register_double(builder, |e, a| {
                    e.mem.set_u32(a[0] + 0xc, 0x0707_0707);
                    Ret::default()
                });
            }
            e.call_log = Some(vec![]);
            let next = e.call(CREATE, &args![pair, entry, data]).u32();
            let log: Log = e.call_log.take().unwrap();
            assert_eq!(
                calls(&log, used),
                [[data, shape_a, shape_b, pair + 0x20]],
                "{shape_type}"
            );
            assert!(calls(&log, unused).is_empty());
            assert_eq!(next, data + 0x10);
            assert_eq!(e.mem.u8(entry + 2), 0);
            assert_eq!(e.mem.u8(data + 0xb), 0xef);
            assert_eq!(e.mem.u32(data + 0xc), 0);
        }
    }

    struct SepWorld {
        e: Engine,
        pair: u32,
        data: u32,
        out: u32,
        solver_words: Seen<u32>,
    }

    /// Bodies with the columns (1..4), (5..8), (9..12) for the first and radii 0.25 and 0.5;
    /// the solver writes `result` and sets the dword at `+0x14` of its object to `refresh`.
    fn sep_world(result: Vec4, refresh: u32) -> SepWorld {
        let mut e = Engine::new();
        let body_a = body(
            &mut e,
            FAKE_VERTICES_A,
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [0.0; 4],
            ],
        );
        let body_b = body(&mut e, FAKE_VERTICES_B, [[0.0; 4]; 4]);
        let shape_a = e.mem.u32(body_a);
        let shape_b = e.mem.u32(body_b);
        e.mem.set_f32(shape_a + 0x10, 0.25);
        e.mem.set_f32(shape_b + 0x10, 0.5);
        vertex_source(&mut e, FAKE_VERTICES_A, 1.0);
        vertex_source(&mut e, FAKE_VERTICES_B, 5.0);
        let pair = e.mem.alloc(0x40);
        e.mem.set_u32(pair, body_a);
        e.mem.set_u32(pair + 4, body_b);
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 8, 2);
        e.mem.set_u8(data + 9, 3);
        e.mem.set_u8(data + 0xa, 0xa5);
        let out = e.mem.alloc(0x10);
        let solver_words: Seen<u32> = Rc::default();
        let sink = solver_words.clone();
        e.register_double(GSK_SOLVE, move |e, a| {
            sink.borrow_mut()
                .extend((0..6).map(|i| e.mem.u32(a[0] + 4 * i)));
            store_vec(e, a[4], result);
            e.mem.set_u32(a[0] + 0x14, refresh);
            Ret::default()
        });
        e.register_double(GSK_REFRESH_CACHE, |_, _| Ret::default());
        SepWorld {
            e,
            pair,
            data,
            out,
            solver_words,
        }
    }

    #[test]
    fn sep_normal_turns_the_solver_result_and_subtracts_the_radii() {
        let mut w = sep_world([1.0, 2.0, 3.0, 10.0], 0);
        w.e.call_log = Some(vec![]);
        w.e.call(SEP_NORMAL, &args![w.pair, 0u32, w.data, w.out]);
        let log: Log = w.e.call_log.take().unwrap();
        let shape_a = w.e.mem.u32(w.e.mem.u32(w.pair));
        let shape_b = w.e.mem.u32(w.e.mem.u32(w.pair + 4));
        let first = calls(&log, FAKE_VERTICES_A);
        let second = calls(&log, FAKE_VERTICES_B);
        assert_eq!(first[0][..3], [shape_a, w.data, 2]);
        assert_eq!(second[0][..3], [shape_b, w.data + 4, 3]);
        assert_eq!(second[0][3], first[0][3] + 0xc0);
        // The solver object sits 0x20 above the stack pointer, its result 0x10.
        let solver = first[0][3] - 0x20;
        assert_eq!(
            calls(&log, GSK_SOLVE),
            [[solver, shape_a, shape_b, w.pair + 0x20, solver - 0x10]]
        );
        assert_eq!(*w.solver_words.borrow(), [2, 3, 5, 0xa, 0, 0]);
        assert!(calls(&log, GSK_REFRESH_CACHE).is_empty());
        assert_eq!(load_vec(&w.e, w.out), [38.0, 44.0, 50.0, 9.25]);
    }

    #[test]
    fn sep_normal_refreshes_the_cache_when_the_solver_asks() {
        let mut w = sep_world([0.0, 0.0, 0.0, 0.5], 1);
        w.e.call_log = Some(vec![]);
        w.e.call(SEP_NORMAL, &args![w.pair, 0u32, w.data, w.out]);
        let log: Log = w.e.call_log.take().unwrap();
        let solves = calls(&log, GSK_SOLVE);
        let refreshes = calls(&log, GSK_REFRESH_CACHE);
        assert_eq!(refreshes, [[solves[0][0], w.data]]);
        assert_eq!(load_vec(&w.e, w.out), [0.0, 0.0, 0.0, -0.25]);
    }

    #[test]
    fn sep_normal_quiets_a_signalling_fourth_lane() {
        let mut w = sep_world([1.0, 0.0, 0.0, f32::from_bits(0x7f80_0001)], 0);
        w.e.call(SEP_NORMAL, &args![w.pair, 0u32, w.data, w.out]);
        let lane = w.e.mem.u32(w.out + 0xc);
        assert_eq!(lane & 0x7fc0_0000, 0x7fc0_0000);
        assert_eq!(load_vec(&w.e, w.out)[0], 1.0);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00d418b0_forwards_its_words_in_order() {
        let mut e = Engine::new();
        e.register_double(0x00d4_1860, |_, a| {
            assert_eq!(a, [1, 2, 3, 4]);
            Ret {
                eax: 0x99,
                ..Ret::default()
            }
        });
        assert_eq!(
            e.call(0x00d4_18b0, &args![1u32, 2u32, 3u32, 4u32]).u32(),
            0x99
        );
    }
}
