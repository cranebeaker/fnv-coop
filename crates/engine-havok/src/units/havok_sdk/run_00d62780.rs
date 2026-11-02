//! `Havok SDK/run_00d62780` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::hkadeltacompressedanimation::profile_record;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{normalising_scale, sum_of_three};

type Lanes = [f32; 4];

/// The globals and constants of the display: the debug tag word (`01268fbc`), the table of
/// 24-byte entries of six floats (`01269210`, the one `00d10890` fills), the floats `0.0`
/// (`01011d78`), `3.0` (`01017718`), `0.5` (`01016248`), `1/3` (`0106c2e0`) and `0.48`
/// (`010d1648`), the vector of ones (`010ec7f0`), the sign vector of lanes 0 to 2 (`010c4c00`), the
/// doubles `-0.98` (`010d1640`) and `10.0` (`01020758`).
const DEBUG_TAG: u32 = 0x0126_8fbc;
const ANGLE_TABLE: u32 = 0x0126_9210;
const ZERO_FLOAT: u32 = 0x0101_1d78;
const THREE_FLOAT: u32 = 0x0101_7718;
const HALF_FLOAT: u32 = 0x0101_6248;
const THIRD_FLOAT: u32 = 0x0106_c2e0;
const EDGE_FRACTION: u32 = 0x010d_1648;
const ONES: u32 = 0x010e_c7f0;
const SIGN_MASK: u32 = 0x010c_4c00;
const DOT_LIMIT: u32 = 0x010d_1640;
const ARC_STEPS: u32 = 0x0102_0758;

fn lanes_00d62780(e: &Engine, at: u32) -> Lanes {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at.wrapping_add(lane * 4)))
}

fn store_00d62780(e: &mut Engine, at: u32, value: Lanes) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem
            .set_u32(at.wrapping_add(lane as u32 * 4), word.to_bits());
    }
}

fn mul_00d62780(a: Lanes, b: Lanes) -> Lanes {
    [0usize, 1, 2, 3].map(|lane| a[lane] * b[lane])
}

fn add_00d62780(a: Lanes, b: Lanes) -> Lanes {
    [0usize, 1, 2, 3].map(|lane| a[lane] + b[lane])
}

fn sub_00d62780(a: Lanes, b: Lanes) -> Lanes {
    [0usize, 1, 2, 3].map(|lane| a[lane] - b[lane])
}

/// `(y, z, x, w)`: `SHUFPS v, v, 0xc9`.
fn yzx_00d62780(v: Lanes) -> Lanes {
    [v[1], v[2], v[0], v[3]]
}

/// `(z, x, y, w)`: `SHUFPS v, v, 0xd2`.
fn zxy_00d62780(v: Lanes) -> Lanes {
    [v[2], v[0], v[1], v[3]]
}

/// A point through the four rows of an entity's transform: `(x * r0 + y * r1) + (z * r2 + r3)`.
fn transformed_00d62780(v: Lanes, rows: [Lanes; 4]) -> Lanes {
    let by_x = mul_00d62780([v[0]; 4], rows[0]);
    let by_y = mul_00d62780([v[1]; 4], rows[1]);
    let by_z = mul_00d62780([v[2]; 4], rows[2]);
    add_00d62780(add_00d62780(by_x, by_y), add_00d62780(by_z, rows[3]))
}

/// `v` scaled by the refined reciprocal square root of its squared length (`MULPS`, the
/// `ADDSS` sums, the host's `RSQRTSS` estimate, one Newton step, zero for a zero length).
fn normalised_00d62780(e: &Engine, v: Lanes) -> Lanes {
    let zero = e.mem.f32(ZERO_FLOAT);
    let three = e.mem.f32(THREE_FLOAT);
    let half = e.mem.f32(HALF_FLOAT);
    let scale = normalising_scale(sum_of_three(mul_00d62780(v, v)), zero, half, three);
    mul_00d62780([scale; 4], v)
}

/// Slot `0x24` of the display object at `this - 0x10`: `(from, to, color, id, tag)` addresses of
/// two points, an `ARGB` colour, the entity's `+0x10` and the debug tag word.
fn line_00d62780(e: &mut Engine, this: u32, from: u32, to: u32, color: u32, id: u32) {
    let tag = e.mem.u32(DEBUG_TAG);
    let display = e.mem.u32(this.wrapping_sub(0x10));
    e.vcall(display, 0x24, &args![from, to, color, id, tag]);
}

/// The debug lines of one triangle shape of an entity: the three edges of the transformed
/// triangle in red, and for each edge, when `dot(w, q) >= -0.98`, an orange normal of the edge
/// (`w`), an orange line to `q + m`, a green line to the centroid and ten orange arc segments from
/// `w` towards `q`. The frame `t` holds the points at `+0x50`, `+0x60`, `+0x70`, the edge midpoint
/// `m` at `+0x80`, the normal `q` at `+0x90`, `w` at `+0xd0`, the unit edge at `+0x100`, the blended
/// direction at `+0x110` and the centroid at `+0x140`, as the exe's frame does.
fn draw_triangle_00d62780(e: &mut Engine, this: u32, t: u32, entity: u32, shape: u32, kind: u8) {
    let id = entity + 0x10;
    let rows = [0xf0u32, 0x100, 0x110, 0x120].map(|offset| lanes_00d62780(e, entity + offset));
    let word = u32::from(e.mem.u16(shape + 0x14));
    let p0 = transformed_00d62780(lanes_00d62780(e, shape + 0x20), rows);
    store_00d62780(e, t + 0x50, p0);
    let p1 = transformed_00d62780(lanes_00d62780(e, shape + 0x30), rows);
    store_00d62780(e, t + 0x60, p1);
    let p2 = transformed_00d62780(lanes_00d62780(e, shape + 0x40), rows);
    e.mem.set_u32(t + 0x48, word);
    store_00d62780(e, t + 0x70, p2);
    line_00d62780(e, this, t + 0x50, t + 0x60, 0xffff_0000, id);
    line_00d62780(e, this, t + 0x60, t + 0x70, 0xffff_0000, id);
    line_00d62780(e, this, t + 0x70, t + 0x50, 0xffff_0000, id);
    let sum = add_00d62780(
        add_00d62780(lanes_00d62780(e, t + 0x60), lanes_00d62780(e, t + 0x50)),
        lanes_00d62780(e, t + 0x70),
    );
    let third = e.mem.f32(THIRD_FLOAT);
    store_00d62780(e, t + 0x140, mul_00d62780(sum, [third; 4]));
    e.call(0x00dd_d620, &args![t + 0x50, t + 0x60, t + 0x70, t + 0x90]);
    let dot_limit = e.mem.f64(DOT_LIMIT);
    let arc_steps = e.mem.f64(ARC_STEPS);
    for edge in 0..3u32 {
        let next = (edge + 1) % 3;
        let from = lanes_00d62780(e, t + 0x50 + edge * 0x10);
        let to = lanes_00d62780(e, t + 0x50 + next * 0x10);
        let delta = sub_00d62780(to, from);
        let direction = normalised_00d62780(e, delta);
        store_00d62780(e, t + 0x100, direction);
        let index = e.call(0x00dd_cdf0, &args![word, edge]).u32();
        let fraction = e.mem.f32(EDGE_FRACTION);
        let ones = lanes_00d62780(e, ONES);
        let from = lanes_00d62780(e, t + 0x50 + edge * 0x10);
        let to = lanes_00d62780(e, t + 0x50 + next * 0x10);
        let midpoint = add_00d62780(
            mul_00d62780(sub_00d62780(ones, [fraction; 4]), from),
            mul_00d62780([fraction; 4], to),
        );
        store_00d62780(e, t + 0x80, midpoint);
        let entry = ANGLE_TABLE + index * 24 + u32::from(kind) * 4;
        let sine = e.mem.f32(entry);
        let cosine = e.mem.f32(entry + 4);
        let normal = lanes_00d62780(e, t + 0x90);
        let unit = lanes_00d62780(e, t + 0x100);
        let cross = sub_00d62780(
            mul_00d62780(zxy_00d62780(normal), yzx_00d62780(unit)),
            mul_00d62780(yzx_00d62780(normal), zxy_00d62780(unit)),
        );
        let turned = add_00d62780(
            mul_00d62780(cross, [cosine; 4]),
            mul_00d62780([sine; 4], normal),
        );
        let w = normalised_00d62780(e, turned);
        store_00d62780(e, t + 0xd0, w);
        let mut normal = normal;
        if kind == 4 {
            let mask = [0u32, 1, 2, 3].map(|lane| e.mem.u32(SIGN_MASK + lane * 4));
            normal =
                [0usize, 1, 2, 3].map(|lane| f32::from_bits(normal[lane].to_bits() ^ mask[lane]));
            store_00d62780(e, t + 0x90, normal);
        }
        let dot = sum_of_three(mul_00d62780(w, normal));
        if x87_load32(dot.to_bits()) < dot_limit {
            continue;
        }
        store_00d62780(e, t + 0x130, add_00d62780(w, midpoint));
        store_00d62780(e, t + 0xe0, add_00d62780(normal, midpoint));
        line_00d62780(e, this, t + 0x80, t + 0x130, 0xffff_a500, id);
        line_00d62780(e, this, t + 0x80, t + 0xe0, 0xffff_a500, id);
        line_00d62780(e, this, t + 0x80, t + 0x140, 0xff00_8000, id);
        let mut previous = lanes_00d62780(e, t + 0xd0);
        for step in 1..=10u32 {
            let amount = f32::from_bits(x87_store32(x87_div(f64::from(step), arc_steps)));
            let ones = lanes_00d62780(e, ONES);
            let normal_now = lanes_00d62780(e, t + 0x90);
            let blend = add_00d62780(
                mul_00d62780(sub_00d62780(ones, [amount; 4]), lanes_00d62780(e, t + 0xd0)),
                mul_00d62780([amount; 4], normal_now),
            );
            let arc = normalised_00d62780(e, blend);
            store_00d62780(e, t + 0x110, arc);
            let middle = lanes_00d62780(e, t + 0x80);
            store_00d62780(e, t + 0xf0, add_00d62780(middle, previous));
            store_00d62780(e, t + 0x120, add_00d62780(arc, middle));
            line_00d62780(e, this, t + 0xf0, t + 0x120, 0xffff_a500, id);
            previous = lanes_00d62780(e, t + 0x110);
        }
    }
}

/// The triangle shapes of one entity: when the shape type word at `**(entity + 0x10) + 0xc` is 0xa
/// the shape container at `*(*(entity + 0x10) + 0x34) + 0x10` (slot 8 first key, `0xc` next key,
/// `0x14` child shape with a 0x200-byte buffer at frame `+0x150`) is walked; each child whose type
/// word at `+0xc` is 3 and whose byte at `+0x16` is not 5 or 6 is drawn.
fn draw_entity_00d62780(e: &mut Engine, this: u32, t: u32, entity: u32) {
    let body = e.mem.u32(entity + 0x10);
    if e.mem.u32(body + 0xc) != 0xa {
        return;
    }
    let owner = e.mem.u32(body + 0x34);
    let container = owner + 0x10;
    let mut key = e.vcall(container, 8, &args![]).u32();
    if key == 0xffff_ffff {
        return;
    }
    loop {
        let shape = e.vcall(container, 0x14, &args![key, t + 0x150]).u32();
        if e.mem.u32(shape + 0xc) == 3 {
            let kind = e.mem.u8(shape + 0x16);
            if kind != 5 && kind != 6 {
                draw_triangle_00d62780(e, this, t, entity, shape, kind);
            }
        }
        key = e.vcall(container, 0xc, &args![key]).u32();
        if key == 0xffff_ffff {
            break;
        }
    }
}

// Translated from 00d62780 (decompiled, FalloutNV.exe 1.4.0.525)
/// A debug display of the triangles of the entities of a world (thiscall, `RET 4`: one stack word
/// `source`; no map name, class unconfirmed; `this - 0x10` is the display object whose slot
/// `0x24` draws a line): between two profiling records (`010d164c` and the end record `010c41f8`;
/// the time stamps are the host clock) it calls `00c91180` on `source`, takes the entity list
/// `00c929e0(source)` (word `+0xc` count, `+8` the array of entity pointers), draws every
/// entity (see `draw_entity_00d62780`) and releases the list with `00c905b0` and `00c91190(source)`.
/// The 0x350-byte frame of the exe is reproduced for the points and vectors passed by address
/// to the display and to `00ddd620` (the plane normal of three points); the words the exe leaves
/// uninitialised in it (for example the fourth lane of the normal) are zero here.
pub fn fn_00d62780(e: &mut Engine, this: Ptr, source: u32) {
    profile_record(e, 0x010d_164c);
    e.call(0x00c9_1180, &args![source]);
    let list = e.call(0x00c9_29e0, &args![source]).u32();
    e.with_stack(0x350, |e, frame| {
        let t = frame.addr();
        if e.mem.i32(list + 0xc) > 0 {
            let mut index = 0i32;
            loop {
                let items = e.mem.u32(list + 8);
                let entity = e.mem.u32(items.wrapping_add(index as u32 * 4));
                draw_entity_00d62780(e, this.addr(), t, entity);
                index += 1;
                if index >= e.mem.i32(list + 0xc) {
                    break;
                }
            }
        }
    });
    e.call(0x00c9_05b0, &args![list]);
    e.call(0x00c9_1190, &args![source]);
    profile_record(e, 0x010c_41f8);
}

// Translated from 00d62e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x2c bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d626b0` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d62e66.
pub fn fn_00d62e20(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x2c]).u32();
    e.mem.set_u16(block + 4, 0x2c);
    let object = e.call(0x00d6_26b0, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_2780, fn_00d62780(Ptr, u32)),
        entry!(0x00d6_2e20, fn_00d62e20(u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// `(from, to, color, id, tag)` of every line drawn.
    type Lines = Rc<RefCell<Vec<(Lanes, Lanes, u32, u32, u32)>>>;

    struct Scene {
        this: u32,
        source: u32,
        entity: u32,
        shape: u32,
        lines: Lines,
    }

    /// One entity with one triangle shape `(0,0,0) (1,0,0) (0,1,0)` and an identity transform;
    /// the normal answered by `00ddd620` is `(0, 0, 1)`; the angle-table entry 0 holds `sine` and
    /// `cosine` in the columns of the shape's `kind`.
    fn scene(e: &mut Engine, kind: u8, sine: f32, cosine: f32) -> Scene {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(THREE_FLOAT, 3.0);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f32(THIRD_FLOAT, f32::from_bits(0x3eaa_aaab));
        e.mem.set_f32(EDGE_FRACTION, f32::from_bits(0x3ef5_c28f));
        for lane in 0..4 {
            e.mem.set_f32(ONES + lane * 4, 1.0);
        }
        for (lane, bits) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(SIGN_MASK + lane as u32 * 4, bits);
        }
        e.mem
            .set_f64(DOT_LIMIT, f64::from_bits(0xbfef_5c29_0000_0000));
        e.mem.set_f64(ARC_STEPS, 10.0);
        e.mem.set_u32(DEBUG_TAG, 0x7e57);
        e.mem.set_f32(ANGLE_TABLE + u32::from(kind) * 4, sine);
        e.mem.set_f32(ANGLE_TABLE + u32::from(kind) * 4 + 4, cosine);
        // The display: slot 0x24 logs every line with the points it is given.
        let lines: Lines = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&lines);
        e.register_double(0x0900_0171, move |e, a| {
            let from = lanes_00d62780(e, a[1]);
            let to = lanes_00d62780(e, a[2]);
            log.borrow_mut().push((from, to, a[3], a[4], a[5]));
            Ret::default()
        });
        let display_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(display_vtable + 0x24, 0x0900_0171);
        let display = e.mem.alloc(0x20);
        e.mem.set_u32(display, display_vtable);
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder, display);
        let this = holder + 0x10;
        // The shape container: one key.
        let container_vtable = e.mem.alloc(0x20);
        for (slot, double) in [
            (8u32, 0x0900_0172u32),
            (0xc, 0x0900_0173),
            (0x14, 0x0900_0174),
        ] {
            e.mem.set_u32(container_vtable + slot, double);
        }
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x10, container_vtable);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body + 0xc, 0xa);
        e.mem.set_u32(body + 0x34, owner);
        let entity = e.mem.alloc(0x140);
        e.mem.set_u32(entity + 0x10, body);
        for (row, lanes) in [
            (0xf0u32, [1.0f32, 0.0, 0.0, 0.0]),
            (0x100, [0.0, 1.0, 0.0, 0.0]),
            (0x110, [0.0, 0.0, 1.0, 0.0]),
            (0x120, [0.0, 0.0, 0.0, 1.0]),
        ] {
            store_00d62780(e, entity + row, lanes);
        }
        let shape = e.mem.alloc(0x60);
        e.mem.set_u32(shape + 0xc, 3);
        e.mem.set_u8(shape + 0x16, kind);
        e.mem.set_u16(shape + 0x14, 0x1234);
        for (offset, lanes) in [
            (0x20u32, [0.0f32, 0.0, 0.0, 0.0]),
            (0x30, [1.0, 0.0, 0.0, 0.0]),
            (0x40, [0.0, 1.0, 0.0, 0.0]),
        ] {
            store_00d62780(e, shape + offset, lanes);
        }
        e.register(0x0900_0172, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register(0x0900_0173, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.register_double(0x0900_0174, move |_, _| Ret {
            eax: shape,
            ..Ret::default()
        });
        // The list of entities and the other callees.
        let list = e.mem.alloc(0x10);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, entity);
        e.mem.set_u32(list + 8, items);
        e.mem.set_u32(list + 0xc, 1);
        e.register_double(0x00c9_29e0, move |_, _| Ret {
            eax: list,
            ..Ret::default()
        });
        for address in [0x00c9_1180, 0x00c9_05b0, 0x00c9_1190] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x00dd_d620, |e, a| {
            store_00d62780(e, a[3], [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        e.register(0x00dd_cdf0, |_, _| Ret::default());
        // The profile stream.
        e.set_global(0x0126_835c, 3u32);
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.register_double(0x00fd_f044, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        Scene {
            this,
            source: 0x5000,
            entity,
            shape,
            lines,
        }
    }

    fn run(e: &mut Engine, s: &Scene) {
        e.call(0x00d6_2780, &args![s.this, s.source]);
    }

    fn near(actual: Lanes, expected: Lanes) {
        for lane in 0..3 {
            assert!(
                (actual[lane] - expected[lane]).abs() <= 1e-5,
                "lane {lane}: {actual:?} vs {expected:?}"
            );
        }
    }

    #[test]
    fn fn_00d62780_draws_the_triangle_and_the_edge_normals() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0, 0.0, 1.0);
        run(&mut e, &s);
        let lines = s.lines.borrow();
        // 3 red edges, then for each of the 3 edges 3 + 10 lines.
        assert_eq!(lines.len(), 3 + 3 * 13);
        let id = s.entity + 0x10;
        let red = [
            ([0.0f32, 0.0, 0.0, 1.0], [1.0f32, 0.0, 0.0, 1.0]),
            ([1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]),
            ([0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0]),
        ];
        for (line, (from, to)) in lines.iter().zip(red) {
            assert_eq!(line.0, from);
            assert_eq!(line.1, to);
            assert_eq!(line.2, 0xffff_0000);
            assert_eq!(line.3, id);
            assert_eq!(line.4, 0x7e57);
        }
        // Edge 0: unit edge (1, 0, 0), normal (0, 0, 1): w = u x n = (0, -1, 0); the midpoint is
        // 0.48 of the way along.
        let midpoint = [0.48f32, 0.0, 0.0, 0.0];
        let orange = &lines[3];
        near(orange.0, midpoint);
        near(orange.1, [0.48, -1.0, 0.0, 0.0]);
        assert_eq!(orange.2, 0xffff_a500);
        near(lines[4].1, [0.48, 0.0, 1.0, 0.0]);
        assert_eq!(lines[4].2, 0xffff_a500);
        // The green line goes to the centroid (1/3, 1/3, 0).
        near(lines[5].1, [1.0 / 3.0, 1.0 / 3.0, 0.0, 0.0]);
        assert_eq!(lines[5].2, 0xff00_8000);
        // The first arc segment: from m + w to m + the blend of w and n by 0.1.
        near(lines[6].0, [0.48, -1.0, 0.0, 0.0]);
        let blend = [0.0f32, -0.9, 0.1, 0.0];
        let length = (blend[1] * blend[1] + blend[2] * blend[2]).sqrt();
        near(lines[6].1, [0.48, -0.9 / length, 0.1 / length, 0.0]);
        // The last segment of the arc ends at m + n (the blend by 1.0).
        near(lines[15].1, [0.48, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00d62780_negates_the_normal_for_kind_4() {
        let mut e = Engine::new();
        let s = scene(&mut e, 4, 0.0, 1.0);
        run(&mut e, &s);
        let lines = s.lines.borrow();
        assert_eq!(lines.len(), 3 + 3 * 13);
        // Edge 0: w comes from the normal (0, 0, 1) as before; only afterwards is the normal negated.
        near(lines[3].1, [0.48, -1.0, 0.0, 0.0]);
        near(lines[4].1, [0.48, 0.0, -1.0, 0.0]);
    }

    #[test]
    fn fn_00d62780_skips_the_edge_normals_when_w_opposes_the_normal() {
        let mut e = Engine::new();
        // sine -1, cosine 0: w = -n, dot = -1 < -0.98: only the triangle is drawn.
        let s = scene(&mut e, 0, -1.0, 0.0);
        run(&mut e, &s);
        assert_eq!(s.lines.borrow().len(), 3);
    }

    #[test]
    fn fn_00d62780_skips_other_shapes_and_bodies() {
        // Shape kinds 5 and 6 are skipped; so are other shape types and bodies of another type.
        for (kind, shape_type, body_type) in
            [(5u8, 3u32, 0xau32), (6, 3, 0xa), (0, 4, 0xa), (0, 3, 9)]
        {
            let mut e = Engine::new();
            let s = scene(&mut e, kind, 0.0, 1.0);
            e.mem.set_u32(s.shape + 0xc, shape_type);
            let body = e.mem.u32(s.entity + 0x10);
            e.mem.set_u32(body + 0xc, body_type);
            run(&mut e, &s);
            assert!(s.lines.borrow().is_empty());
        }
    }

    #[test]
    fn fn_00d62780_brackets_the_work_with_profile_records_and_releases_the_list() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0, -1.0, 0.0);
        e.call_log = Some(vec![]);
        run(&mut e, &s);
        let log = e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses[0], 0x00d6_2780);
        assert_eq!(addresses[1], 0x00fd_f044);
        assert_eq!(addresses[2..4], [0x00c9_1180, 0x00c9_29e0]);
        let tail = &addresses[addresses.len() - 3..];
        assert_eq!(tail, [0x00c9_05b0, 0x00c9_1190, 0x00fd_f044]);
        assert_eq!(log[2].1, vec![0x5000]);
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
    fn fn_00d62e20_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_26b0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_2e20, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x2c]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x2c);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d62e20_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_26b0, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_2e20, &args![0x1234u32]).u32(), 0);
    }
}
