//! `Havok SDK/hkpconvexpenetrationutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkCalculatePenetrationDepth` (Xbox PDB) finds how deep two convex shapes overlap. It
//! builds a 0x70-byte search state (`00dfb1e0`), runs a simplex search on it (`00dfa940`) and,
//! when the search settles on the "needs the exact answer" outcome 3, asks `00e2db70` for it.
//! The state holds a 4x4 transform copy at +0x00 (four vectors), the radius splatted at
//! +0x40, its square at +0x50, an attempt counter at +0x5C and a first-attempt flag at +0x6C
//! (names by use; the Xbox PDB has no record of the state).
//!
//! The search (`00dfa940`) keeps its simplex on the stack: a header object at +0x20 of the
//! frame whose word at +8 counts the vertices, and up to 0x37 vertices of 0x40 bytes from
//! +0x40 on, in a frame of 0x2EC4 bytes. The callees (`00df9fe0`, `00dfaf80`, `00dfb0b0`,
//! `00df9b50`, `00df9c90`, `00df9d90`, `00df9ae0`) are not translated yet; they are called by
//! address.
//!
//! x87 note: `00dfa940` perturbs the state's vector at +0x30 after a failed attempt by
//! `(r - 0.0001) * 0.0002` (doubles, `r` the result of `00df9ae0` in ST0), rounded to
//! `float` and added to all four lanes with `ADDPS` against four frame words at +0x10, of
//! which the first three hold the perturbation and the fourth is whatever the frame held
//! (the exe never initialises it; the frame here starts zeroed, and the sum is stored back
//! into those four words, so later perturbations add the previous fourth lane again).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtPenetration"` and the end label.
const LABEL_PENETRATION: u32 = 0x010d_bb48;
const LABEL_END: u32 = 0x010c_41f8;
/// `double` 0.0001 (`01032980`) and 0.0002 (`010dbb40`) of the perturbation.
const PERTURBATION_OFFSET: u32 = 0x0103_2980;
const PERTURBATION_SCALE: u32 = 0x010d_bb40;
/// The search's callees. `this` is the state unless noted.
/// `(state, shapes, &outcome)` -> 1 when the search is decided.
const SEARCH_STEP: u32 = 0x00df_9fe0;
/// `this` = the simplex header: `(a, b, c)` taken from the state at +0x60, +0x64, +0x68.
const SIMPLEX_INIT: u32 = 0x00df_af80;
/// `this` = the simplex header: the next search direction, a pointer used as the vertex key.
const SIMPLEX_NEXT_DIRECTION: u32 = 0x00df_b0b0;
/// `(direction, vertex)`: fills a simplex vertex.
const FILL_VERTEX: u32 = 0x00df_9b50;
/// `(simplex header, direction, vertex, &outcome)` -> 1 when the simplex is decided.
const ADD_VERTEX: u32 = 0x00df_9c90;
/// `(simplex header, direction, shapes)`: the final computation after a decided search.
const FINISH: u32 = 0x00df_9d90;
/// Returns a `float` in ST0 from a seed in `EAX` (no stack word).
const RANDOM_VALUE: u32 = 0x00df_9ae0;
/// `(arg1, arg2, transform, arg5, arg6, arg9)`, cdecl: the exact answer for outcome 3.
const EXACT_DEPTH: u32 = 0x00e2_db70;
/// The memory router (`00c85750`, no arguments): its first word is the cursor of a stack-like
/// allocation area.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The float `0.0001` splatted into the radius lanes of every search state of the multi search.
const MULTI_RADIUS: u32 = 0x0101_41cc;
/// The float stored as the radius squared of every state of the multi search.
const MULTI_RADIUS_SQUARED: u32 = 0x010d_bb58;
/// The vector with the sign bit of every lane (for `XORPS`).
const SIGN_MASK: u32 = 0x010c_44d0;
/// `(first header, second header, &result vector, &first direction, &second direction)`,
/// cdecl (`00e2d6c0`): works on the headers of two simplexes and writes the vector.
const PAIR_SEARCH: u32 = 0x00e2_d6c0;
/// The bytes of one search record of the multi search (state, three points, header, vertices).
const RECORD_SIZE: u32 = 0x3000;

/// Vertices the simplex may hold before the search gives up with outcome 2.
const VERTEX_LIMIT: i32 = 0x37;
/// Attempts before the search gives up with outcome 3.
const ATTEMPT_LIMIT: i32 = 0x14;

layout! {
    /// The search state of `hkCalculatePenetrationDepth` (0x70 bytes), as built by `00dfb1e0`.
    pub struct PenetrationState: 0x70 {
        /// Fourth vector of the copied transform, perturbed by failed attempts.
        0x30 m_perturbed: Inline<[f32; 4]>,
        /// The radius splatted into four lanes.
        0x40 m_radius: Inline<[f32; 4]>,
        /// The radius squared.
        0x50 m_radiusSquared: f32,
        /// First of the two words set from the caller's arguments.
        0x54 m_field54: u32,
        /// Second of them.
        0x58 m_field58: u32,
        /// Attempt counter.
        0x5c m_attempts: i32,
        /// Words passed to the simplex start.
        0x60 m_field60: u32,
        0x64 m_field64: u32,
        0x68 m_field68: u32,
        /// 1 on the first attempt, 0 after a perturbation.
        0x6c m_firstAttempt: u32,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfa940, fn_00dfa940(Ptr<PenetrationState>, u32) -> i32),
        entry!(0x00dfaba0, hk_calc_multi_penetration_depth(Ptr, Ptr, Ptr, i32, Ptr, Ptr) -> u32),
        entry!(0x00dfaac0, hk_calculate_penetration_depth(u32, u32, Ptr, f32, u32, u32, u32, u32, u32) -> i32),
        entry!(
            0x00dfb1e0,
            fn_00dfb1e0(Ptr<PenetrationState>, u32, u32, Ptr, f32, u32, u32, u32, u32)
        ),
    ]
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
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

// Translated from 00dfb1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `RET 0x20`: builds the search state. Copies the 64 bytes of
/// `transform` to +0x00, splats `radius` into +0x40 (the argument's bits as pushed), stores
/// its square (computed on the x87 stack, so a signalling NaN is quieted) at +0x50,
/// `field_54` at +0x54, `field_58` at +0x58, 0 at +0x5C, and `field_60` to `field_6c` at
/// +0x60 to +0x6C.
#[allow(clippy::too_many_arguments)]
pub fn fn_00dfb1e0(
    e: &mut Engine,
    this: Ptr<PenetrationState>,
    field_54: u32,
    field_58: u32,
    transform: Ptr,
    radius: f32,
    field_60: u32,
    field_64: u32,
    field_68: u32,
    field_6c: u32,
) {
    let matrix = e.mem.bytes(transform.addr(), 0x40);
    e.mem.write(this.addr(), &matrix);
    let quiet = f64::from(x87_quiet(radius));
    e.set(
        this,
        PenetrationState::m_radiusSquared,
        (quiet * quiet) as f32,
    );
    e.set(this, PenetrationState::m_field54, field_54);
    e.set(this, PenetrationState::m_field58, field_58);
    for lane in 0..4 {
        e.mem
            .set_u32(this.addr() + 0x40 + 4 * lane, radius.to_bits());
    }
    e.set(this, PenetrationState::m_attempts, 0);
    e.set(this, PenetrationState::m_field60, field_60);
    e.set(this, PenetrationState::m_field64, field_64);
    e.set(this, PenetrationState::m_field68, field_68);
    e.set(this, PenetrationState::m_firstAttempt, field_6c);
}

/// The vertex key slot of vertex `index` in the search frame (vertices are 0x40 bytes from
/// +0x40; word +0x30 of each is cleared when it is taken).
fn vertex_at(frame: u32, index: i32) -> u32 {
    frame + 0x40 + 0x40 * index as u32
}

// Translated from 00dfa940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `RET 4`: the simplex search on a state, `(state, shapes)`.
/// Returns the outcome: whatever `00df9fe0` or the simplex reports when it is not 3, 2 when
/// the simplex needs more than 0x37 vertices (after the final computation `00df9d90`), and 3
/// when twenty attempts, each perturbing the state's vector at +0x30 (see the module
/// comment), still end in outcome 3.
pub fn fn_00dfa940(e: &mut Engine, this: Ptr<PenetrationState>, shapes: u32) -> i32 {
    e.with_stack(0x2ec4, |e, frame| {
        let frame = frame.addr();
        let outcome = frame + 0xc;
        let simplex = frame + 0x20;
        let vertex_count = frame + 0x28;
        loop {
            e.mem.set_i32(outcome, 0);
            let decided = e.call(SEARCH_STEP, &args![this, shapes, outcome]).u32();
            if decided == 1 {
                let result = e.mem.i32(outcome);
                if result != 3 {
                    return result;
                }
            } else {
                let first = e.get(this, PenetrationState::m_field60);
                let second = e.get(this, PenetrationState::m_field64);
                let third = e.get(this, PenetrationState::m_field68);
                e.call(SIMPLEX_INIT, &args![simplex, first, second, third]);
                let direction = loop {
                    let direction = e.call(SIMPLEX_NEXT_DIRECTION, &args![simplex]).u32();
                    let index = e.mem.i32(vertex_count);
                    let vertex = vertex_at(frame, index);
                    e.mem.set_i32(vertex_count, index.wrapping_add(1));
                    e.mem.set_u32(vertex + 0x30, 0);
                    e.call(FILL_VERTEX, &args![this, direction, vertex]);
                    let decided = e
                        .call(
                            ADD_VERTEX,
                            &args![this, simplex, direction, vertex, outcome],
                        )
                        .u32();
                    if decided == 1 {
                        break direction;
                    }
                    if e.mem.i32(vertex_count) >= VERTEX_LIMIT {
                        e.call(FINISH, &args![this, simplex, direction, shapes]);
                        return 2;
                    }
                };
                let result = e.mem.i32(outcome);
                if result != 3 {
                    e.call(FINISH, &args![this, simplex, direction, shapes]);
                    return result;
                }
            }
            // Outcome 3: try again, perturbed, up to the limit.
            let attempts = e.get(this, PenetrationState::m_attempts).wrapping_add(1);
            e.set(this, PenetrationState::m_attempts, attempts);
            if attempts == 1 {
                e.set(this, PenetrationState::m_firstAttempt, 1);
                continue;
            }
            if attempts >= ATTEMPT_LIMIT {
                return 3;
            }
            e.set(this, PenetrationState::m_firstAttempt, 0);
            // 00dfaa55: `00df9ae0` takes its seed in EAX, here the attempt count (loaded at
            // 00dfaa3a).
            let random = e.call(RANDOM_VALUE, &args![attempts]).f64();
            let offset = e.global::<f64>(PERTURBATION_OFFSET);
            let scale = e.global::<f64>(PERTURBATION_SCALE);
            let step = ((random - offset) * scale) as f32;
            let scratch = frame + 0x10;
            for lane in 0..3 {
                e.mem.set_f32(scratch + 4 * lane, step);
            }
            for lane in 0..4 {
                let at = this.addr() + 0x30 + 4 * lane;
                let sum = e.mem.f32(at) + e.mem.f32(scratch + 4 * lane);
                e.mem.set_f32(scratch + 4 * lane, sum);
                e.mem.set_f32(at, sum);
            }
        }
    })
}

// Translated from 00dfaac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCalculatePenetrationDepth` (Xbox PDB), cdecl with nine words. Builds the search state
/// from `arg1`, `arg2`, `transform`, `radius` (a `float` pushed through `FLD`/`FSTP`) and
/// `arg5` to `arg8`, runs the search with `arg9`, and for outcome 3 asks `00e2db70` for the
/// exact depth and answers 1. Between `"TtPenetration"` profiling records.
#[allow(clippy::too_many_arguments)]
pub fn hk_calculate_penetration_depth(
    e: &mut Engine,
    arg1: u32,
    arg2: u32,
    transform: Ptr,
    radius: f32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
    arg8: u32,
    arg9: u32,
) -> i32 {
    profile_record(e, LABEL_PENETRATION);
    let outcome = e.with_stack(0x70, |e, state| {
        let state = state.cast::<PenetrationState>();
        fn_00dfb1e0(
            e,
            state,
            arg1,
            arg2,
            transform,
            x87_quiet(radius),
            arg5,
            arg6,
            arg7,
            arg8,
        );
        let mut outcome = fn_00dfa940(e, state, arg9);
        if outcome == 3 {
            e.call(EXACT_DEPTH, &args![arg1, arg2, transform, arg5, arg6, arg9]);
            outcome = 1;
        }
        outcome
    });
    profile_record(e, LABEL_END);
    outcome
}

/// Four SSE lanes as bits.
type Lanes = [u32; 4];

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

fn lanes_op(dest: Lanes, source: Lanes, op: fn(f32, f32) -> f32) -> Lanes {
    [0, 1, 2, 3].map(|lane| sse(dest[lane], source[lane], op))
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

/// `SHUFPS` of a register with itself that copies one lane to all four.
fn broadcast(lanes: Lanes, lane: usize) -> Lanes {
    [lanes[lane]; 4]
}

/// A `MOVAPS` load of four lanes.
fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    [0, 1, 2, 3].map(|lane| e.mem.u32(addr + 4 * lane))
}

/// A `MOVAPS` store of four lanes.
fn store_lanes(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, value);
    }
}

// Translated from 00dfaba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCalcMultiPenetrationDepth` (Ghidra label): cdecl with six words, answers `1` when the
/// search is unusable and `0` when the results are written. For `count` shapes (the pointers
/// at `shapes`, the result records at `outputs`, both with a stride of four bytes) it takes a
/// search record of `0x3000` bytes each from the memory router's cursor (the area is rounded
/// up to 128 bytes in all). A record holds the search state in the layout of
/// [`fn_00dfb1e0`] (the `transform` copied to `+0`, the radius lanes `0.0001` at `+0x40`, a
/// constant radius squared at `+0x50`, `reference` at `+0x54`, the shape at `+0x58`, the
/// points `+0x70`, `+0xb0` and `+0xf0` named at `+0x60` to `+0x68`), then the shape at
/// `+0x130`, its result record at `+0x134` and the simplex header at `+0x140` (vertex count
/// at `+8`, the direction word at `+0x2ff0` is written by the pair search). A search step
/// that settles at once (answers `1`) drops the shape and the next one reuses the record.
/// With fewer than two records left the answer is `1`; with more only the first two are
/// used. The pair search (`00e2d6c0`) then runs, and every record whose outcome word is not
/// `2` and whose simplex holds `0x37` or more vertices takes one more vertex; rounds repeat
/// while one of those is undecided, and a decided one with outcome `3` gives up with `1`.
/// Finally the result vector of the pair search, negated, and the contact vector of the
/// final computation are rotated by the matrix `matrix` (rows at `+0`, `+0x10`, `+0x20`, the
/// translation at `+0x30`) into each result record: the rotated vector at `+0x10`, the
/// rotated and translated contact point at `+0`, and at `+0x1c` (over the rotated vector's
/// fourth lane) the depth: the negated fourth lane of the pair search's vector less
/// `reference[4]` less the shape's float at `+0x10`. The cursor is restored on every exit.
pub fn hk_calc_multi_penetration_depth(
    e: &mut Engine,
    matrix: Ptr,
    reference: Ptr,
    shapes: Ptr,
    count: i32,
    transform: Ptr,
    outputs: Ptr,
) -> u32 {
    e.with_stack(0xa0, |e, frame| {
        let frame = frame.addr();
        let (matrix, reference, shapes, transform, outputs) = (
            matrix.addr(),
            reference.addr(),
            shapes.addr(),
            transform.addr(),
            outputs.addr(),
        );
        let mut count = count;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let base = e.mem.u32(router);
        let area = (count.wrapping_mul(RECORD_SIZE as i32).wrapping_add(0x7f) as u32) & 0xffff_ff80;
        e.mem.set_u32(router, area.wrapping_add(base));
        let restore = |e: &mut Engine| {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            e.mem.set_u32(router, base);
        };
        if count > 0 {
            let radius = x87_quiet_bits(e.mem.u32(MULTI_RADIUS));
            let radius_squared = x87_quiet_bits(e.mem.u32(MULTI_RADIUS_SQUARED));
            let output_offset = outputs.wrapping_sub(shapes);
            let mut record = base;
            let mut shape_cursor = shapes;
            let mut index = 0;
            loop {
                let shape = e.mem.u32(shape_cursor);
                e.mem.set_u32(record + 0x130, shape);
                let output = e.mem.u32(shape_cursor.wrapping_add(output_offset));
                e.mem.set_u32(record + 0x134, output);
                for row in 0..4 {
                    let lanes = load_lanes(e, transform + 0x10 * row);
                    store_lanes(e, record + 0x10 * row, lanes);
                }
                e.mem.set_u32(record + 0x50, radius_squared);
                e.mem.set_u32(record + 0x58, shape);
                e.mem.set_u32(record + 0x54, reference);
                e.mem.set_u32(record + 0x60, record + 0x70);
                e.mem.set_u32(record + 0x5c, 0);
                e.mem.set_u32(record + 0x6c, 0);
                e.mem.set_u32(record + 0x64, record + 0xb0);
                e.mem.set_u32(record + 0x68, record + 0xf0);
                store_lanes(e, record + 0x40, [radius; 4]);
                let settled = e
                    .call(SEARCH_STEP, &args![record, frame + 0x70, frame + 0x28])
                    .u32();
                if settled == 1 {
                    count -= 1;
                } else {
                    e.call(
                        SIMPLEX_INIT,
                        &args![record + 0x140, record + 0x70, record + 0xb0, record + 0xf0],
                    );
                    record += RECORD_SIZE;
                }
                index += 1;
                shape_cursor = shape_cursor.wrapping_add(4);
                if index >= count {
                    break;
                }
            }
        }
        if count > 2 {
            count = 2;
        } else if count < 2 {
            restore(e);
            return 1;
        }
        e.mem.set_u32(frame + 0x28, 0);
        e.mem.set_u32(frame + 0x2c, 0);
        loop {
            e.call(
                PAIR_SEARCH,
                &args![
                    base + 0x140,
                    base + 0x140 + RECORD_SIZE,
                    frame + 0x40,
                    base + 0x2ff0,
                    base + 0x2ff0 + RECORD_SIZE
                ],
            );
            let mut undecided = false;
            let mut outcome = frame + 0x28;
            let mut header_count = base + 0x148;
            for _ in 0..count {
                let vertices = e.mem.i32(header_count);
                if e.mem.i32(outcome) != 2 && vertices >= 0x37 {
                    let state = header_count - 0x148;
                    let header = header_count - 8;
                    let vertex = header + 0x20 + ((vertices as u32) << 6);
                    e.mem.set_i32(header_count, vertices.wrapping_add(1));
                    e.mem.set_u32(vertex + 0x30, 0);
                    let direction = e.mem.u32(header_count + 0x2ea8);
                    e.call(FILL_VERTEX, &args![state, direction, vertex]);
                    let direction = e.mem.u32(header_count + 0x2ea8);
                    let decided = e
                        .call(
                            ADD_VERTEX,
                            &args![state, header, direction, vertex, outcome],
                        )
                        .u32();
                    if decided == 1 {
                        if e.mem.i32(outcome) == 3 {
                            restore(e);
                            return 1;
                        }
                    } else {
                        undecided = true;
                    }
                }
                outcome += 4;
                header_count += RECORD_SIZE;
            }
            if !undecided {
                break;
            }
        }
        // The pair search's result vector, negated, rotated into every result record.
        let depth = x87_quiet_bits(e.mem.u32(frame + 0x4c)) ^ 0x8000_0000;
        let mask = load_lanes(e, SIGN_MASK);
        let found = load_lanes(e, frame + 0x40);
        let vector: Lanes = std::array::from_fn(|lane| found[lane] ^ mask[lane]);
        let reference_float = x87_quiet_bits(e.mem.u32(reference + 0x10));
        if count <= 0 {
            restore(e);
            return 0;
        }
        let offset = x87_sub(x87_load32(depth), x87_load32(reference_float));
        e.mem.set_f64(frame + 0x28, offset);
        let mut state_cursor = base + 0x134;
        for _ in 0..count {
            let state = state_cursor - 0x134;
            let direction = e.mem.u32(state_cursor + 0x2ebc);
            e.call(
                FINISH,
                &args![state, state_cursor + 0xc, direction, frame + 0x70],
            );
            let rows = [0, 0x10, 0x20, 0x30].map(|row| load_lanes(e, matrix + row));
            let output = e.mem.u32(state_cursor);
            let turned = lanes_op(
                lanes_op(
                    lanes_op(broadcast(vector, 1), rows[1], mul),
                    lanes_op(rows[0], broadcast(vector, 0), mul),
                    add,
                ),
                lanes_op(broadcast(vector, 2), rows[2], mul),
                add,
            );
            store_lanes(e, output + 0x10, turned);
            let contact = load_lanes(e, frame + 0x80);
            let shape = e.mem.u32(state_cursor - 4);
            let shape_float = x87_quiet_bits(e.mem.u32(shape + 0x10));
            let lowered = x87_sub(e.mem.f64(frame + 0x28), x87_load32(shape_float));
            let sideways = lanes_op(
                lanes_op(broadcast(contact, 1), rows[1], mul),
                lanes_op(broadcast(contact, 0), rows[0], mul),
                add,
            );
            let forward = lanes_op(lanes_op(broadcast(contact, 2), rows[2], mul), rows[3], add);
            let point = lanes_op(sideways, forward, add);
            store_lanes(e, output, point);
            let output = e.mem.u32(state_cursor);
            e.mem.set_u32(output + 0x1c, (lowered as f32).to_bits());
            state_cursor += RECORD_SIZE;
        }
        restore(e);
        0
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// The monitor stream the profiling records go to.
    const STREAM: u32 = 0x0300_0000;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(PERTURBATION_OFFSET, 0.0001);
        e.mem.set_f64(PERTURBATION_SCALE, 0.0002);
        e.map(STREAM, 0x200);
        let stream = STREAM;
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0x1f0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, _| stream.into_ret());
        for addr in [SIMPLEX_INIT, FILL_VERTEX, FINISH, EXACT_DEPTH, RANDOM_VALUE] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A search step that answers "decided" with the outcomes of `script`, in turn.
    fn scripted_step(e: &mut Engine, script: Vec<(u32, i32)>) {
        let turn = Rc::new(Cell::new(0usize));
        e.register_double(SEARCH_STEP, move |e, a| {
            let (decided, outcome) = script[turn.get().min(script.len() - 1)];
            turn.set(turn.get() + 1);
            e.mem.set_i32(a[2], outcome);
            decided.into_ret()
        });
    }

    fn state(e: &mut Engine) -> Ptr<PenetrationState> {
        e.new_object::<PenetrationState>()
    }

    #[test]
    fn state_is_built_from_the_arguments() {
        let mut e = rig();
        let this = state(&mut e);
        let transform = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_f32(transform + 4 * i, i as f32 + 1.0);
        }
        e.call(
            0x00df_b1e0,
            &args![
                this,
                0xa1u32,
                0xa2u32,
                Ptr::<()>::new(transform),
                3.0f32,
                5u32,
                6u32,
                7u32,
                8u32
            ],
        );
        for i in 0..16 {
            assert_eq!(e.mem.f32(this.addr() + 4 * i), i as f32 + 1.0);
        }
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this.addr() + 0x40 + 4 * lane), 3.0);
        }
        assert_eq!(e.get(this, PenetrationState::m_radiusSquared), 9.0);
        assert_eq!(e.get(this, PenetrationState::m_field54), 0xa1);
        assert_eq!(e.get(this, PenetrationState::m_field58), 0xa2);
        assert_eq!(e.get(this, PenetrationState::m_attempts), 0);
        assert_eq!(e.get(this, PenetrationState::m_field60), 5);
        assert_eq!(e.get(this, PenetrationState::m_field64), 6);
        assert_eq!(e.get(this, PenetrationState::m_field68), 7);
        assert_eq!(e.get(this, PenetrationState::m_firstAttempt), 8);
    }

    #[test]
    fn a_decided_step_other_than_three_is_the_answer() {
        let mut e = rig();
        let this = state(&mut e);
        scripted_step(&mut e, vec![(1, 2)]);
        assert_eq!(e.call(0x00df_a940, &args![this, 0x77u32]).i32(), 2);
        assert!(calls(&e, SIMPLEX_INIT).is_empty());
        assert_eq!(calls(&e, SEARCH_STEP)[0][..2], [this.addr(), 0x77]);
    }

    #[test]
    fn the_simplex_runs_when_the_step_is_undecided() {
        let mut e = rig();
        let this = state(&mut e);
        e.set(this, PenetrationState::m_field60, 11);
        e.set(this, PenetrationState::m_field64, 12);
        e.set(this, PenetrationState::m_field68, 13);
        scripted_step(&mut e, vec![(0, 0)]);
        e.register(SIMPLEX_NEXT_DIRECTION, |_, _| 0xd00du32.into_ret());
        // The simplex is decided on the third vertex, with outcome 1.
        let added = Rc::new(Cell::new(0));
        let counter = Rc::clone(&added);
        let vertices = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&vertices);
        e.register_double(ADD_VERTEX, move |e, a| {
            counter.set(counter.get() + 1);
            seen.borrow_mut().push((a[1], a[3]));
            if counter.get() == 3 {
                e.mem.set_i32(a[4], 1);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        assert_eq!(e.call(0x00df_a940, &args![this, 0x77u32]).i32(), 1);
        let init = &calls(&e, SIMPLEX_INIT)[0];
        assert_eq!(init[1..], [11, 12, 13]);
        let vertices = vertices.borrow();
        assert_eq!(vertices.len(), 3);
        // Vertices are 0x40 bytes apart in the frame, after the header.
        assert_eq!(vertices[1].1 - vertices[0].1, 0x40);
        assert_eq!(vertices[0].1 - init[0], 0x20);
        // The final computation gets the simplex, the direction and the shapes.
        assert_eq!(
            calls(&e, FINISH),
            vec![vec![this.addr(), init[0], 0xd00d, 0x77]]
        );
    }

    #[test]
    fn too_many_vertices_answer_two_after_the_final_computation() {
        let mut e = rig();
        let this = state(&mut e);
        scripted_step(&mut e, vec![(0, 0)]);
        e.register(SIMPLEX_NEXT_DIRECTION, |_, _| 0xd00du32.into_ret());
        e.register(ADD_VERTEX, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00df_a940, &args![this, 1u32]).i32(), 2);
        assert_eq!(calls(&e, ADD_VERTEX).len(), VERTEX_LIMIT as usize);
        assert_eq!(calls(&e, FINISH).len(), 1);
    }

    #[test]
    fn outcome_three_is_retried_with_perturbations_and_then_given_up() {
        let mut e = rig();
        let this = state(&mut e);
        e.mem.set_f32(this.addr() + 0x30, 1.0);
        e.mem.set_f32(this.addr() + 0x3c, 1.0);
        scripted_step(&mut e, vec![(1, 3)]);
        // r = 0.5001 gives a perturbation of (0.5001 - 0.0001) * 0.0002 = 0.0001.
        e.register(RANDOM_VALUE, |_, _| Ret {
            st0: 0.5001,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00df_a940, &args![this, 1u32]).i32(), 3);
        assert_eq!(calls(&e, SEARCH_STEP).len(), ATTEMPT_LIMIT as usize);
        assert_eq!(e.get(this, PenetrationState::m_attempts), ATTEMPT_LIMIT);
        assert_eq!(e.get(this, PenetrationState::m_firstAttempt), 0);
        // Eighteen perturbations of 0.0001 (the first attempt only sets the flag).
        let moved = e.mem.f32(this.addr() + 0x30);
        assert!((moved - (1.0 + 18.0 * 0.0001)).abs() < 1e-4, "{moved}");
        // The fourth lane adds the frame's previous fourth lane each time: it doubles.
        assert_eq!(e.mem.f32(this.addr() + 0x3c), 1.0 * 2f32.powi(17));
    }

    #[test]
    fn penetration_depth_asks_for_the_exact_answer_on_three() {
        let mut e = rig();
        let transform = e.mem.alloc(0x40);
        scripted_step(&mut e, vec![(1, 3), (1, 3), (1, 3)]);
        // Make the search give up at once: outcome 3 for the whole run.
        let result = e
            .call(
                0x00df_aac0,
                &args![
                    1u32,
                    2u32,
                    Ptr::<()>::new(transform),
                    0.5f32,
                    5u32,
                    6u32,
                    7u32,
                    8u32,
                    9u32
                ],
            )
            .i32();
        assert_eq!(result, 1);
        assert_eq!(calls(&e, EXACT_DEPTH), vec![vec![1, 2, transform, 5, 6, 9]]);
        // The search ran with the ninth argument as its shapes.
        assert_eq!(calls(&e, SEARCH_STEP)[0][1], 9);
    }

    #[test]
    fn penetration_depth_passes_other_outcomes_through() {
        let mut e = rig();
        let transform = e.mem.alloc(0x40);
        scripted_step(&mut e, vec![(1, 2)]);
        let result = e
            .call(
                0x00df_aac0,
                &args![
                    1u32,
                    2u32,
                    Ptr::<()>::new(transform),
                    0.5f32,
                    5u32,
                    6u32,
                    7u32,
                    8u32,
                    9u32
                ],
            )
            .i32();
        assert_eq!(result, 2);
        assert!(calls(&e, EXACT_DEPTH).is_empty());
        // The start and end profiling records.
        assert_eq!(e.mem.u32(STREAM + 4), STREAM + 0x40 + 24);
        assert_eq!(e.mem.u32(STREAM + 0x40), LABEL_PENETRATION);
        assert_eq!(e.mem.u32(STREAM + 0x4c), LABEL_END);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// What the doubles answer: the search step per shape, the added vertices as
    /// `(decided, outcome written)`, and an outcome the pair search writes into the first
    /// outcome slot.
    struct Script {
        step: Vec<u32>,
        add: Vec<(u32, i32)>,
        pair_outcome: Option<i32>,
    }

    struct Rig {
        e: Engine,
        log: Log,
        base: u32,
        cursor: u32,
        matrix: u32,
        reference: u32,
        shapes: u32,
        outputs: u32,
        transform: u32,
    }

    fn put(e: &mut Engine, addr: u32, values: [f32; 4]) {
        for (lane, value) in values.into_iter().enumerate() {
            e.mem.set_f32(addr + 4 * lane as u32, value);
        }
    }

    fn floats(e: &Engine, addr: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|lane| e.mem.f32(addr + 4 * lane))
    }

    /// Doubles for the memory router and all the callees, two shapes whose floats at `+0x10`
    /// are `0.25` and `0.5`, two result records, a matrix with the translation
    /// `(10, 20, 30, 1)` and a reference float of `0.5`. The pair search writes
    /// `(1, 2, 3, 4)` and the final computation `(0.5, 0.25, 0.125, 9)` at `+0x10` of its
    /// output.
    fn rig(script: Script) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(MULTI_RADIUS, 0x38d1_b717);
        e.mem.set_u32(MULTI_RADIUS_SQUARED, 0x322b_cc76);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let arena = e.mem.alloc(0x8000);
        let cursor = e.mem.alloc(0x20);
        e.mem.set_u32(cursor, arena);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: cursor,
            ..Ret::default()
        });
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let turn = Rc::new(Cell::new(0usize));
        let (sink, steps, counter) = (Rc::clone(&log), script.step, Rc::clone(&turn));
        e.register_double(SEARCH_STEP, move |_, a| {
            sink.borrow_mut().push((SEARCH_STEP, a.to_vec()));
            let answer = steps[counter.get().min(steps.len() - 1)];
            counter.set(counter.get() + 1);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        for address in [SIMPLEX_INIT, FILL_VERTEX] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let (sink, pair_outcome) = (Rc::clone(&log), script.pair_outcome);
        e.register_double(PAIR_SEARCH, move |e, a| {
            sink.borrow_mut().push((PAIR_SEARCH, a.to_vec()));
            put(e, a[2], [1.0, 2.0, 3.0, 4.0]);
            if let Some(outcome) = pair_outcome {
                e.mem.set_i32(a[2] - 0x18, outcome);
            }
            Ret::default()
        });
        let (sink, adds, counter) = (Rc::clone(&log), script.add, Rc::new(Cell::new(0usize)));
        e.register_double(ADD_VERTEX, move |e, a| {
            sink.borrow_mut().push((ADD_VERTEX, a.to_vec()));
            let (decided, outcome) = adds[counter.get().min(adds.len() - 1)];
            counter.set(counter.get() + 1);
            e.mem.set_i32(a[4], outcome);
            Ret {
                eax: decided,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(FINISH, move |e, a| {
            sink.borrow_mut().push((FINISH, a.to_vec()));
            put(e, a[3] + 0x10, [0.5, 0.25, 0.125, 9.0]);
            Ret::default()
        });
        let matrix = e.mem.alloc(0x40);
        put(&mut e, matrix, [1.0, 0.0, 0.0, 0.0]);
        put(&mut e, matrix + 0x10, [0.0, 1.0, 0.0, 0.0]);
        put(&mut e, matrix + 0x20, [0.0, 0.0, 1.0, 0.0]);
        put(&mut e, matrix + 0x30, [10.0, 20.0, 30.0, 1.0]);
        let reference = e.mem.alloc(0x20);
        e.mem.set_f32(reference + 0x10, 0.5);
        let transform = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(transform + 4 * word, 0x4000_0000 + word);
        }
        let shapes = e.mem.alloc(0x10);
        let outputs = e.mem.alloc(0x10);
        for (index, height) in [0.25f32, 0.5].into_iter().enumerate() {
            let shape = e.mem.alloc(0x20);
            e.mem.set_f32(shape + 0x10, height);
            let output = e.mem.alloc(0x30);
            e.mem.set_u32(shapes + 4 * index as u32, shape);
            e.mem.set_u32(outputs + 4 * index as u32, output);
        }
        Rig {
            e,
            log,
            base: arena,
            cursor,
            matrix,
            reference,
            shapes,
            outputs,
            transform,
        }
    }

    impl Rig {
        fn run(&mut self, count: i32) -> u32 {
            let (matrix, reference, shapes, transform, outputs) = (
                self.matrix,
                self.reference,
                self.shapes,
                self.transform,
                self.outputs,
            );
            self.e
                .call(
                    0x00df_aba0,
                    &args![matrix, reference, shapes, count, transform, outputs],
                )
                .u32()
        }

        fn calls_to(&self, address: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(called, _)| *called == address)
                .map(|(_, words)| words.clone())
                .collect()
        }

        /// The result record of shape `index`.
        fn output(&self, index: u32) -> u32 {
            self.e.mem.u32(self.outputs + 4 * index)
        }

        fn shape(&self, index: u32) -> u32 {
            self.e.mem.u32(self.shapes + 4 * index)
        }
    }

    fn quiet() -> Script {
        Script {
            step: vec![0],
            add: vec![(1, 0)],
            pair_outcome: None,
        }
    }

    #[test]
    fn a_single_shape_lays_out_its_record_and_answers_one() {
        let mut r = rig(quiet());
        assert_eq!(r.run(1), 1);
        let base = r.base;
        // The state: the transform, the radius lanes, the constants and the pointers.
        for word in 0..16 {
            assert_eq!(r.e.mem.u32(base + 4 * word), 0x4000_0000 + word);
        }
        for lane in 0..4 {
            assert_eq!(r.e.mem.u32(base + 0x40 + 4 * lane), 0x38d1_b717);
        }
        assert_eq!(r.e.mem.u32(base + 0x50), 0x322b_cc76);
        assert_eq!(r.e.mem.u32(base + 0x54), r.reference);
        assert_eq!(r.e.mem.u32(base + 0x58), r.shape(0));
        assert_eq!(r.e.mem.u32(base + 0x5c), 0);
        assert_eq!(r.e.mem.u32(base + 0x60), base + 0x70);
        assert_eq!(r.e.mem.u32(base + 0x64), base + 0xb0);
        assert_eq!(r.e.mem.u32(base + 0x68), base + 0xf0);
        assert_eq!(r.e.mem.u32(base + 0x6c), 0);
        assert_eq!(r.e.mem.u32(base + 0x130), r.shape(0));
        assert_eq!(r.e.mem.u32(base + 0x134), r.output(0));
        let frame_pointers = &r.calls_to(SEARCH_STEP)[0];
        assert_eq!(frame_pointers[0], base);
        assert_eq!(frame_pointers[1] - frame_pointers[2], 0x70 - 0x28);
        assert_eq!(
            r.calls_to(SIMPLEX_INIT),
            vec![vec![base + 0x140, base + 0x70, base + 0xb0, base + 0xf0]]
        );
        // One shape is too few: the cursor is back where it was.
        assert_eq!(r.e.mem.u32(r.cursor), base);
        assert!(r.calls_to(PAIR_SEARCH).is_empty());
    }

    #[test]
    fn a_settled_shape_is_dropped_and_the_next_one_takes_its_record() {
        let mut r = rig(Script {
            step: vec![1, 0, 0],
            ..quiet()
        });
        // Records of the third shape would be read stale: the second record needs a shape.
        let base = r.base;
        r.e.mem.set_u32(base + RECORD_SIZE + 0x130, r.shape(1));
        r.e.mem.set_u32(base + RECORD_SIZE + 0x134, r.output(1));
        let shapes = r.e.mem.alloc(0x10);
        let outputs = r.e.mem.alloc(0x10);
        for (index, address) in [r.shape(0), r.shape(1), 0xdead_0000]
            .into_iter()
            .enumerate()
        {
            r.e.mem.set_u32(shapes + 4 * index as u32, address);
        }
        for index in 0..2 {
            let output = r.output(index);
            r.e.mem.set_u32(outputs + 4 * index, output);
        }
        r.shapes = shapes;
        r.outputs = outputs;
        assert_eq!(r.run(3), 0);
        // Two search steps: the second shape sits in the first record, and the loop ends
        // because only two shapes remain counted.
        assert_eq!(r.calls_to(SEARCH_STEP).len(), 2);
        assert_eq!(r.e.mem.u32(base + 0x130), r.e.mem.u32(shapes + 4));
        assert_eq!(r.calls_to(SIMPLEX_INIT).len(), 1);
        assert_eq!(r.calls_to(PAIR_SEARCH).len(), 1);
        assert_eq!(r.calls_to(FINISH).len(), 2);
        assert_eq!(r.e.mem.u32(r.cursor), base);
    }

    #[test]
    fn the_found_vector_is_rotated_into_the_result_records() {
        let mut r = rig(quiet());
        assert_eq!(r.run(2), 0);
        let base = r.base;
        assert_eq!(
            r.calls_to(PAIR_SEARCH),
            vec![vec![
                base + 0x140,
                base + 0x140 + RECORD_SIZE,
                r.calls_to(PAIR_SEARCH)[0][2],
                base + 0x2ff0,
                base + 0x2ff0 + RECORD_SIZE
            ]]
        );
        // The pair search wrote (1, 2, 3, 4) and the vector is negated: (-1, -2, -3).
        // The depth, -4 - 0.5 less the shape's float at +0x10, overwrites the fourth lane.
        assert_eq!(floats(&r.e, r.output(0) + 0x10), [-1.0, -2.0, -3.0, -4.75]);
        assert_eq!(floats(&r.e, r.output(1) + 0x10), [-1.0, -2.0, -3.0, -5.0]);
        // The point: (0.5, 0.25, 0.125) turned (the identity) and moved by (10, 20, 30);
        // the fourth lane is 1 (the translation row's).
        assert_eq!(floats(&r.e, r.output(0)), [10.5, 20.25, 30.125, 1.0]);
        assert_eq!(floats(&r.e, r.output(1)), [10.5, 20.25, 30.125, 1.0]);
        let finishes = r.calls_to(FINISH);
        assert_eq!(finishes.len(), 2);
        assert_eq!(finishes[0][..3], [base, base + 0x140, 0]);
        assert_eq!(
            finishes[1][..2],
            [base + RECORD_SIZE, base + RECORD_SIZE + 0x140]
        );
        assert_eq!(r.e.mem.u32(r.cursor), base);
    }

    #[test]
    fn a_full_simplex_takes_vertices_until_it_is_decided() {
        let mut r = rig(Script {
            add: vec![(0, 0), (1, 0)],
            ..quiet()
        });
        let base = r.base;
        r.e.mem.set_u32(base + 0x148, 0x37);
        r.e.mem.set_u32(base + 0x2ff0, 0xd1d1);
        assert_eq!(r.run(2), 0);
        let vertex = base + 0x140 + 0x20 + 0x37 * 0x40;
        let adds = r.calls_to(ADD_VERTEX);
        assert_eq!(adds.len(), 2);
        assert_eq!(adds[0][..4], [base, base + 0x140, 0xd1d1, vertex]);
        assert_eq!(adds[1][3], vertex + 0x40);
        assert_eq!(adds[0][4], adds[1][4]);
        assert_eq!(r.calls_to(FILL_VERTEX)[0], vec![base, 0xd1d1, vertex]);
        // The undecided vertex made another round of the pair search.
        assert_eq!(r.calls_to(PAIR_SEARCH).len(), 2);
        assert_eq!(r.e.mem.u32(base + 0x148), 0x39);
        assert_eq!(r.e.mem.u32(vertex + 0x30), 0);
    }

    #[test]
    fn a_record_with_outcome_two_takes_no_vertex() {
        let mut r = rig(Script {
            pair_outcome: Some(2),
            ..quiet()
        });
        let base = r.base;
        r.e.mem.set_u32(base + 0x148, 0x40);
        assert_eq!(r.run(2), 0);
        assert!(r.calls_to(ADD_VERTEX).is_empty());
        assert!(r.calls_to(FILL_VERTEX).is_empty());
        assert_eq!(r.e.mem.u32(base + 0x148), 0x40);
    }

    #[test]
    fn a_decided_simplex_with_outcome_three_gives_up() {
        let mut r = rig(Script {
            add: vec![(1, 3)],
            ..quiet()
        });
        let base = r.base;
        r.e.mem.set_u32(base + 0x148, 0x37);
        assert_eq!(r.run(2), 1);
        assert!(r.calls_to(FINISH).is_empty());
        assert_eq!(r.e.mem.u32(r.cursor), base);
    }
}
