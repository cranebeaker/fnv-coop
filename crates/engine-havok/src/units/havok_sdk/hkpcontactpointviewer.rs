//! `Havok SDK/hkpcontactpointviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions bracket their work in two timer records of the thread's profiling stream
//! (`TlsGetValue` of the index at `0126835c`); the `RDTSC` time stamp comes from the host clock.

use crate::prelude::*;
use crate::x87::rsqrt_estimate;

type Lanes = [u32; 4];

/// `TlsGetValue` import slot and the TLS index of the profiling stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The label of the record that ends a timed section.
const LABEL_END: u32 = 0x010c_41f8;
/// The labels `"TthkContactPointViewer_displayArrow"` and
/// `"TthkpContactPointViewer::drawAllContactPointsInIsland"`.
const LABEL_DISPLAY_ARROW: u32 = 0x010d_8194;
const LABEL_DRAW_ALL: u32 = 0x010d_81b8;
/// The memory router (`00c85750`, no arguments): its first word is the stack-like allocation
/// cursor, `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Float constants of the arrow code: the epsilon below which a direction is not drawn, 0.0, 3.0
/// and 0.5 of the refined reciprocal square root, 0.85 (the arrow head's place along the
/// shaft), 0.15 and -0.15 (the head's width), and the vector of ones.
const FLOAT_EPSILON: u32 = 0x0104_da4c;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const HEAD_PLACE: u32 = 0x0101_9de0;
const HEAD_PLUS: u32 = 0x010c_69c4;
const HEAD_MINUS: u32 = 0x010c_69c0;
const VECTOR_ONES: u32 = 0x010e_c7f0;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte profiling record `(label, time stamp)` to the thread's stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

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

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

/// The lane sum `x2 + (x1 + x0)` the code builds with two `ADDSS` over shuffled copies.
fn lane_sum(lanes: Lanes) -> u32 {
    let low = sse(lanes[1], lanes[0], add);
    sse(lanes[2], low, add)
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

/// An `FLD dword` from `address` stored straight back with `FSTP dword` (or `MOVSS` of that
/// copy): the float's bits with a signalling NaN quieted.
fn x87_constant(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

// Translated from 00de2aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkContactPointViewer_displayArrow` (map name): draws an arrow for the vector `direction`
/// (passed in `EAX`) from the point `from`, with `this` the display handler. Does nothing but
/// the timer records when `x0*x0 + ... < epsilon` (a NaN length is drawn). Otherwise the
/// shaft goes from `from` to `end = from + direction`, and the head is two lines from `end` to
/// the points `base +- 0.15 * v`, where `base = 0.15 * from + 0.85 * end` and `v` is a
/// vector perpendicular to the direction: its zero lane is the direction's smallest lane (in
/// the order the code compares them), the other two are the direction's lanes swapped with
/// one sign flipped; it is scaled by the refined reciprocal square root of its own length
/// (the host's `RSQRTSS` estimate, one Newton step, zero when the length is 0) and by the
/// direction's length. The three lines are `displayLine` (vtable slot `0x24` of `this`: two
/// points, the colour, `id`, `tag`).
pub fn display_arrow(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    from: Ptr,
    color: u32,
    id: u32,
    tag: u32,
) {
    profile_record(e, LABEL_DISPLAY_ARROW);
    let dir = load_lanes(e, direction.addr());
    let squares = lanes_op(dir, dir, mul);
    let length_sq = lane_sum(squares);
    let epsilon = f32::from_bits(x87_constant(e, FLOAT_EPSILON));
    if epsilon > f32::from_bits(length_sq) {
        profile_record(e, LABEL_END);
        return;
    }
    let start = load_lanes(e, from.addr());
    let end = lanes_op(start, dir, add);
    // The three magnitudes through the x87 stack, and the two compares that pick the lanes.
    let magnitudes = [0, 1, 2].map(|lane| f32::from_bits(x87_quiet_bits(dir[lane])).abs());
    let (smaller, first_index, saved) = if magnitudes[0] > magnitudes[1] {
        (magnitudes[1], 0usize, 1usize)
    } else {
        (magnitudes[0], 1, 0)
    };
    let second_index = if smaller > magnitudes[2] { saved } else { 2 };
    let mut perpendicular = [0u32; 4];
    perpendicular[first_index] = x87_quiet_bits(dir[second_index]);
    perpendicular[second_index] = x87_quiet_bits(dir[first_index]) ^ 0x8000_0000;
    let perpendicular_sq = lane_sum(lanes_op(perpendicular, perpendicular, mul));
    let estimate = rsqrt_estimate(f32::from_bits(perpendicular_sq)).to_bits();
    let squared = sse(sse(perpendicular_sq, estimate, mul), estimate, mul);
    let correction = sse(e.mem.u32(THREE), squared, sub);
    let refined = sse(sse(e.mem.u32(HALF), estimate, mul), correction, mul);
    let scale = if f32::from_bits(perpendicular_sq) == f32::from_bits(e.mem.u32(ZERO)) {
        0
    } else {
        refined
    };
    let length = f32::from_bits(length_sq).sqrt().to_bits();
    let scaled = lanes_op([scale; 4], perpendicular, mul);
    let vector = lanes_op(scaled, [length; 4], mul);
    let start = load_lanes(e, from.addr());
    let place = [x87_constant(e, HEAD_PLACE); 4];
    let ones = load_lanes(e, VECTOR_ONES);
    let shaft = lanes_op(lanes_op(ones, place, sub), start, mul);
    let toward_end = lanes_op(place, end, mul);
    let base = lanes_op(shaft, toward_end, add);
    let plus = [x87_constant(e, HEAD_PLUS); 4];
    let minus = [x87_constant(e, HEAD_MINUS); 4];
    let head_a = lanes_op(lanes_op(plus, vector, mul), base, add);
    let head_b = lanes_op(lanes_op(minus, vector, mul), base, add);
    e.with_stack(0x30, |e, locals| {
        let end_at = locals.addr();
        let head_b_at = end_at + 0x10;
        let head_a_at = end_at + 0x20;
        store_lanes(e, end_at, end);
        store_lanes(e, head_b_at, head_b);
        store_lanes(e, head_a_at, head_a);
        let handler = this.addr();
        e.vcall(handler, 0x24, &args![from, end_at, color, id, tag]);
        e.vcall(handler, 0x24, &args![end_at, head_a_at, color, id, tag]);
        e.vcall(handler, 0x24, &args![end_at, head_b_at, color, id, tag]);
    });
    profile_record(e, LABEL_END);
}

/// The contacts of one agent: with a scratch array of `u16` indices (a `0x200`-byte block taken
/// from the memory router's stack-like cursor and given back afterwards) filled by the agent's
/// vtable slot `0x30`, every index whose value (slot `0x28`, a float pointer) is not below the
/// viewer's threshold at `+0x2c` gets an arrow (the index's point from slot `0x2c`, whose
/// vector at `+0x10` is the direction). Frees the array through the allocator when its
/// capacity word is not negative.
fn draw_agent_contacts(e: &mut Engine, viewer: u32, agent: u32) {
    e.with_stack(0x10, |e, array| {
        let at = array.addr();
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        e.mem.set_u32(router, top.wrapping_add(0x200));
        e.mem.set_u32(at, top);
        e.mem.set_u32(at + 0xc, top);
        e.mem.set_u32(at + 8, 0x8000_0100);
        e.vcall(agent, 0x30, &args![array]);
        if (e.mem.u32(at + 4) as i32) > 0 {
            let mut index = 0u32;
            loop {
                let data = e.mem.u32(at);
                let point_index = u32::from(e.mem.u16(data.wrapping_add(index.wrapping_mul(2))));
                let from = e.vcall(agent, 0x2c, &args![point_index]).u32();
                let data = e.mem.u32(at);
                let value_index = u32::from(e.mem.u16(data.wrapping_add(index.wrapping_mul(2))));
                let value_at = e.vcall(agent, 0x28, &args![value_index]).u32();
                let value = f32::from_bits(x87_quiet_bits(e.mem.u32(value_at)));
                let threshold = f32::from_bits(x87_quiet_bits(e.mem.u32(viewer + 0x2c)));
                let below = threshold > value;
                if !below {
                    let info = e.mem.u32(agent + 0x74);
                    let table = u32::from(e.mem.u8(info + 0x1a));
                    let record = e.mem.u32(info + table * 4 + 4);
                    let tag = e.vcall(viewer + 8, 4, &args![]).u32();
                    let color = e.mem.u32(viewer + 0x30);
                    let handler = e.mem.u32(viewer + 0x18);
                    display_arrow(
                        e,
                        Ptr::new(handler),
                        Ptr::new(from + 0x10),
                        Ptr::new(from),
                        color,
                        record + 0x10,
                        tag,
                    );
                }
                index += 1;
                if (index as i32) >= (e.mem.u32(at + 4) as i32) {
                    break;
                }
            }
        }
        let saved = e.mem.u32(at + 0xc);
        if saved == e.mem.u32(at) {
            e.mem.set_u32(at + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, saved);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(at + 8);
        e.mem.set_u32(at + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(at);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(2);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 8, 0x8000_0000);
    });
}

// Translated from 00de2da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContactPointViewer::drawAllContactPointsInIsland` (name from the exe's profile label):
/// walks the island's chunks (`[island + 0x5c]` pointers to blocks, `[island + 0x60]` of them,
/// signed): every block but the last holds `0x200` bytes of variable-sized records (the size is
/// the byte at `+3`), the last one `[island + 0x58]` bytes. For each record the agent at `+8`
/// whose vtable slot `0x34` answers zero gets its contacts drawn.
pub fn draw_all_contact_points_in_island(e: &mut Engine, this: Ptr, island: Ptr) {
    let viewer = this.addr();
    let island = island.addr();
    profile_record(e, LABEL_DRAW_ALL);
    if (e.mem.u32(island + 0x60) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let chunks = e.mem.u32(island + 0x5c);
            let chunk = e.mem.u32(chunks.wrapping_add(index.wrapping_mul(4)));
            index += 1;
            let end = if index == e.mem.u32(island + 0x60) {
                e.mem.u32(island + 0x58).wrapping_add(chunk)
            } else {
                chunk.wrapping_add(0x200)
            };
            let mut cursor = chunk;
            while cursor < end {
                let agent = e.mem.u32(cursor + 8);
                if e.vcall(agent, 0x34, &args![]).u32() == 0 {
                    draw_agent_contacts(e, viewer, agent);
                }
                cursor = cursor.wrapping_add(u32::from(e.mem.u8(cursor + 3)));
            }
            if (index as i32) >= (e.mem.u32(island + 0x60) as i32) {
                break;
            }
        }
    }
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_2aa0, display_arrow(Ptr, Ptr, Ptr, u32, u32, u32)),
        entry!(0x00de_2da0, draw_all_contact_points_in_island(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range mapped, the arrow constants, and a profiling
    /// stream with room for `room` records; answers the stream too.
    fn engine(room: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(FLOAT_EPSILON, 1.192_092_9e-7f32.to_bits());
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f32(HEAD_PLACE, 0.85);
        e.mem.set_f32(HEAD_PLUS, 0.15);
        e.mem.set_f32(HEAD_MINUS, -0.15);
        for lane in 0..4 {
            e.mem.set_f32(VECTOR_ONES + 4 * lane, 1.0);
        }
        e.set_global(PROFILE_SLOT, 5u32);
        let stream = e.mem.alloc(0x10);
        let records = e.mem.alloc(12 * room.max(1));
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 12 * room);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        (e, stream)
    }

    /// The labels of the records written to the stream since `first`.
    fn labels(e: &Engine, stream: u32, first: u32) -> Vec<u32> {
        let cursor = e.mem.u32(stream + 4);
        (0..(cursor - first) / 12)
            .map(|index| e.mem.u32(first + 12 * index))
            .collect()
    }

    fn floats(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|lane| e.mem.f32(at + 4 * lane))
    }

    /// A vector of four floats in a fresh block.
    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x20);
        for (lane, value) in lanes.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
        at
    }

    /// What a `displayLine` call saw: the two end points' lanes and the pass-through words.
    type Line = ([f32; 4], [f32; 4], u32, u32, u32);
    type Lines = Rc<RefCell<Vec<Line>>>;
    type WordPairs = Rc<RefCell<Vec<(u32, u32)>>>;

    /// A display handler object whose slot `0x24` logs the lines and the two pointers.
    fn handler(e: &mut Engine) -> (u32, Lines, WordPairs) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x24, 0x0900_0024);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let lines = Rc::new(RefCell::new(Vec::new()));
        let pointers = Rc::new(RefCell::new(Vec::new()));
        let (line_sink, pointer_sink) = (Rc::clone(&lines), Rc::clone(&pointers));
        e.register_double(0x0900_0024, move |e, a| {
            assert_eq!(a[0], object);
            line_sink
                .borrow_mut()
                .push((floats(e, a[1]), floats(e, a[2]), a[3], a[4], a[5]));
            pointer_sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        (object, lines, pointers)
    }

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn display_arrow_draws_the_shaft_and_a_head_perpendicular_to_the_direction() {
        // (direction, the lane of the perpendicular that is zero).
        for (direction, zero_lane) in [
            ([1.0, 2.0, 2.0, 0.0], 0),
            ([3.0, 2.0, 1.0, 0.0], 2),
            ([2.0, 3.0, 1.0, 0.0], 2),
            ([3.0, 1.0, 2.0, 0.0], 1),
        ] {
            let (mut e, stream) = engine(8);
            let first = e.mem.u32(stream + 4);
            let (object, lines, pointers) = handler(&mut e);
            let dir = vector(&mut e, direction);
            let from = vector(&mut e, [10.0, 20.0, 30.0, 1.0]);
            e.call(
                0x00de_2aa0,
                &args![object, dir, from, 0xff12_3456u32, 0x77u32, 0x88u32],
            );
            let lines = lines.borrow();
            assert_eq!(lines.len(), 3);
            let end = [
                10.0 + direction[0],
                20.0 + direction[1],
                30.0 + direction[2],
                1.0,
            ];
            assert_eq!(lines[0].0, [10.0, 20.0, 30.0, 1.0]);
            assert_eq!(lines[0].1, end);
            assert_eq!(lines[1].0, end);
            assert_eq!(lines[2].0, end);
            for line in lines.iter() {
                assert_eq!((line.2, line.3, line.4), (0xff12_3456, 0x77, 0x88));
            }
            // The end point is the same block in the three calls.
            let pointers = pointers.borrow();
            assert_eq!(pointers[1].0, pointers[0].1);
            assert_eq!(pointers[2].0, pointers[0].1);
            // The head points are base +- 0.15 * v with v perpendicular to the direction, of
            // the direction's length, and with the zero lane exactly zero.
            let (a, b) = (lines[1].1, lines[2].1);
            let length = (direction[0] * direction[0]
                + direction[1] * direction[1]
                + direction[2] * direction[2])
                .sqrt();
            let base = [
                0.15 * 10.0 + 0.85 * end[0],
                0.15 * 20.0 + 0.85 * end[1],
                0.15 * 30.0 + 0.85 * end[2],
            ];
            let v = [0, 1, 2].map(|lane| (a[lane] - b[lane]) / 0.3);
            assert_eq!(a[zero_lane], b[zero_lane]);
            assert!(near(
                v[0] * direction[0] + v[1] * direction[1] + v[2] * direction[2],
                0.0
            ));
            assert!(near(
                (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(),
                length
            ));
            for lane in 0..3 {
                assert!(near((a[lane] + b[lane]) / 2.0, base[lane]));
            }
            assert_eq!(a[3], b[3]);
            assert_eq!(labels(&e, stream, first), [LABEL_DISPLAY_ARROW, LABEL_END]);
        }
    }

    #[test]
    fn display_arrow_with_a_tiny_direction_draws_nothing() {
        let (mut e, stream) = engine(8);
        let first = e.mem.u32(stream + 4);
        let (object, lines, _) = handler(&mut e);
        let dir = vector(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let from = vector(&mut e, [1.0, 2.0, 3.0, 1.0]);
        e.call(0x00de_2aa0, &args![object, dir, from, 1u32, 2u32, 3u32]);
        assert!(lines.borrow().is_empty());
        assert_eq!(labels(&e, stream, first), [LABEL_DISPLAY_ARROW, LABEL_END]);
        // The same for a squared length just below the epsilon.
        let small = vector(&mut e, [3.0e-4, 0.0, 0.0, 0.0]);
        e.call(0x00de_2aa0, &args![object, small, from, 1u32, 2u32, 3u32]);
        assert!(lines.borrow().is_empty());
        // A length of exactly the epsilon is drawn.
        let exact = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(FLOAT_EPSILON, 1.0);
        e.call(0x00de_2aa0, &args![object, exact, from, 1u32, 2u32, 3u32]);
        assert_eq!(lines.borrow().len(), 3);
    }

    #[test]
    fn display_arrow_draws_a_nan_direction() {
        let (mut e, _) = engine(8);
        let (object, lines, _) = handler(&mut e);
        let dir = vector(&mut e, [f32::NAN, 0.0, 0.0, 0.0]);
        let from = vector(&mut e, [1.0, 2.0, 3.0, 1.0]);
        e.call(0x00de_2aa0, &args![object, dir, from, 1u32, 2u32, 3u32]);
        let lines = lines.borrow();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].1[0].is_nan());
        assert_eq!(lines[0].1[1], 2.0);
    }

    type AgentLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// An agent whose vtable logs `(slot, argument)` and answers: slot `0x34` the given
    /// answer, slot `0x30` fills the array with the indices `0, 1` (and, when `grow`, a
    /// positive capacity), slot `0x2c` the point of the index, slot `0x28` the value of it.
    fn agent(
        e: &mut Engine,
        skip: u32,
        grow: bool,
        points: [u32; 2],
        values: [u32; 2],
    ) -> (u32, AgentLog) {
        let log: AgentLog = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x40);
        // A double address of its own for every agent.
        let base = vtable | 0x8000_0000;
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        // The table the arrow's id comes from: byte 1 at +0x1a selects the second entry.
        let info = e.mem.alloc(0x40);
        e.mem.set_u8(info + 0x1a, 1);
        e.mem.set_u32(info + 8, 0x5000);
        e.mem.set_u32(object + 0x74, info);
        for (slot, double) in [
            (0x34u32, base + 0x34),
            (0x30, base + 0x30),
            (0x2c, base + 0x2c),
            (0x28, base + 0x28),
        ] {
            e.mem.set_u32(vtable + slot, double);
        }
        let sink = Rc::clone(&log);
        e.register_double(base + 0x34, move |_, a| {
            sink.borrow_mut().push((0x34, a[0]));
            Ret {
                eax: skip,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(base + 0x30, move |e, a| {
            sink.borrow_mut().push((0x30, a[1]));
            let data = e.mem.u32(a[1]);
            e.mem.set_u16(data, 0);
            e.mem.set_u16(data + 2, 1);
            e.mem.set_u32(a[1] + 4, 2);
            if grow {
                e.mem.set_u32(a[1] + 8, 0x100);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(base + 0x2c, move |_, a| {
            sink.borrow_mut().push((0x2c, a[1]));
            Ret {
                eax: points[a[1] as usize],
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(base + 0x28, move |_, a| {
            sink.borrow_mut().push((0x28, a[1]));
            Ret {
                eax: values[a[1] as usize],
                ..Ret::default()
            }
        });
        (object, log)
    }

    /// A viewer with threshold 0.5, colour `0xc0ffee`, a display handler at `+0x18` and an
    /// embedded object at `+8` whose slot 4 answers the tag `0x77`.
    fn viewer(e: &mut Engine) -> (u32, u32, Lines) {
        let (object, lines, _) = handler(e);
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x2c, 0.5);
        e.mem.set_u32(this + 0x30, 0xc0_ffee);
        e.mem.set_u32(this + 0x18, object);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0104);
        e.mem.set_u32(this + 8, vtable);
        e.register_double(0x0900_0104, move |_, a| {
            assert_eq!(a[0], this + 8);
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        (this, object, lines)
    }

    /// The memory router: a cursor word at `+0` into a block, an allocator at `+0x10` whose
    /// slot `0x10` logs `(data, bytes)`.
    fn router(e: &mut Engine) -> (u32, u32, WordPairs) {
        let block = e.mem.alloc(0x800);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, block);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(object + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&freed);
        e.register_double(0x0900_0010, move |_, a| {
            assert_eq!(a[0], allocator);
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        (object, block, freed)
    }

    #[test]
    fn draw_all_contact_points_draws_the_contacts_at_or_above_the_threshold() {
        let (mut e, stream) = engine(16);
        let first = e.mem.u32(stream + 4);
        let (this, _, lines) = viewer(&mut e);
        let (router_object, block, freed) = router(&mut e);
        // The second point (index 1) is drawn, the first one is below the threshold.
        let low_point = vector(&mut e, [0.0; 4]);
        let from = vector(&mut e, [1.0, 2.0, 3.0, 1.0]);
        e.mem.set_f32(from + 0x10, 1.0);
        e.mem.set_f32(from + 0x14, 2.0);
        e.mem.set_f32(from + 0x18, 2.0);
        let low = e.mem.alloc(4);
        e.mem.set_f32(low, 0.25);
        let high = e.mem.alloc(4);
        e.mem.set_f32(high, 0.75);
        let (drawing, drawing_log) = agent(&mut e, 0, true, [low_point, from], [low, high]);
        let (skipped, skipped_log) = agent(&mut e, 1, false, [0, 0], [0, 0]);
        // One chunk of two records: sizes 0x20 and 0x10, the island's last block of 0x30 bytes.
        let chunk = e.mem.alloc(0x100);
        e.mem.set_u8(chunk + 3, 0x20);
        e.mem.set_u32(chunk + 8, drawing);
        e.mem.set_u8(chunk + 0x20 + 3, 0x10);
        e.mem.set_u32(chunk + 0x20 + 8, skipped);
        let chunks = e.mem.alloc(4);
        e.mem.set_u32(chunks, chunk);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + 0x58, 0x30);
        e.mem.set_u32(island + 0x5c, chunks);
        e.mem.set_u32(island + 0x60, 1);
        e.call(0x00de_2da0, &args![this, island]);
        let drawing_log = drawing_log.borrow();
        assert_eq!(drawing_log[0], (0x34, drawing));
        assert_eq!(drawing_log[1].0, 0x30);
        assert_eq!(
            drawing_log[2..],
            [(0x2c, 0), (0x28, 0), (0x2c, 1), (0x28, 1)]
        );
        assert_eq!(*skipped_log.borrow(), [(0x34, skipped)]);
        let lines = lines.borrow();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].0, [1.0, 2.0, 3.0, 1.0]);
        assert_eq!(lines[0].1, [1.0 + 1.0, 2.0 + 2.0, 3.0 + 2.0, 1.0]);
        // Colour, the id (the table entry plus 0x10) and the tag from the viewer's objects.
        assert_eq!(
            (lines[0].2, lines[0].3, lines[0].4),
            (0xc0_ffee, 0x5010, 0x77)
        );
        // The scratch block is given back, and freed because the agent raised the capacity.
        assert_eq!(e.mem.u32(router_object), block);
        assert_eq!(*freed.borrow(), [(block, 0x200)]);
        assert_eq!(
            labels(&e, stream, first),
            [LABEL_DRAW_ALL, LABEL_DISPLAY_ARROW, LABEL_END, LABEL_END]
        );
    }

    #[test]
    fn draw_all_contact_points_walks_every_chunk_and_skips_empty_islands() {
        let (mut e, stream) = engine(16);
        let first = e.mem.u32(stream + 4);
        let (this, _, lines) = viewer(&mut e);
        let skipped: Vec<(u32, AgentLog)> = (0..6)
            .map(|_| agent(&mut e, 1, false, [0, 0], [0, 0]))
            .collect();
        // The first chunk holds four records of 0x80 bytes (ending at 0x200); the last one
        // two of 0x10 (the island's size 0x20).
        let chunks = [e.mem.alloc(0x300), e.mem.alloc(0x40)];
        let places: [(u32, u32, u8); 6] = [
            (chunks[0], 0, 0x80),
            (chunks[0], 0x80, 0x80),
            (chunks[0], 0x100, 0x80),
            (chunks[0], 0x180, 0x80),
            (chunks[1], 0, 0x10),
            (chunks[1], 0x10, 0x10),
        ];
        for ((chunk, offset, size), (agent, _)) in places.into_iter().zip(&skipped) {
            e.mem.set_u8(chunk + offset + 3, size);
            e.mem.set_u32(chunk + offset + 8, *agent);
        }
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, chunks[0]);
        e.mem.set_u32(table + 4, chunks[1]);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + 0x58, 0x20);
        e.mem.set_u32(island + 0x5c, table);
        e.mem.set_u32(island + 0x60, 2);
        e.call(0x00de_2da0, &args![this, island]);
        for (agent, log) in &skipped {
            assert_eq!(*log.borrow(), [(0x34, *agent)]);
        }
        assert!(lines.borrow().is_empty());
        assert_eq!(labels(&e, stream, first), [LABEL_DRAW_ALL, LABEL_END]);
        // Zero chunks: the two timer records only.
        e.mem.set_u32(island + 0x60, 0);
        e.call(0x00de_2da0, &args![this, island]);
        assert_eq!(
            labels(&e, stream, first),
            [LABEL_DRAW_ALL, LABEL_END, LABEL_DRAW_ALL, LABEL_END]
        );
    }
}
