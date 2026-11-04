//! `(unplaced)/run_006bec40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees: the navmesh's vertex point for a 16-bit index (`0068f0a0`, `thiscall` on the
/// navmesh, `(index)`, answers the point), an array's count (`0044ddc0`, `thiscall`, no
/// arguments) and element address (`00877a30`, `thiscall`, `(index)`), another array's
/// element address (`005eeda0`, `thiscall`, `(index)`), `NiPoint3::operator+` (`00439e90`,
/// `thiscall`, `(&result, &other)`, answers `&result`), the point scale `0045bb20`
/// (`thiscall`, `(&result, factor)`, answers `&result`), the 2-D point constructor `00452dc0`
/// (`thiscall`, `(x, y)`), and the tests `006b9cf0` (`cdecl`, seven words), `006c3570`
/// (`cdecl`, four words), `006ba010` (`cdecl`, four words) and `00697730` (`thiscall` on the
/// navmesh, five words).
const VERTEX_POINT: u32 = 0x0068_f0a0;
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
const OTHER_ARRAY_ELEMENT: u32 = 0x005e_eda0;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT2_CONSTRUCTOR: u32 = 0x0045_2dc0;
const TRIANGLE_TEST_006B9CF0: u32 = 0x006b_9cf0;
const POINT_TEST_006C3570: u32 = 0x006c_3570;
const SEGMENT_TEST_006BA010: u32 = 0x006b_a010;
const NAVMESH_TEST_00697730: u32 = 0x0069_7730;
/// Float constants: one third (`0106c2dc`, and the rounded-up `0106c2e0`), one half
/// (`01016248`), 128 (`0101e704`) and 180 (`0106b158`).
const ONE_THIRD_LOW: u32 = 0x0106_c2dc;
const ONE_THIRD: u32 = 0x0106_c2e0;
const ONE_HALF: u32 = 0x0101_6248;
const FLOAT_128: u32 = 0x0101_e704;
const FLOAT_180: u32 = 0x0106_b158;

/// A float constant as the code pushes it (`FLD`/`FSTP`, so quiet).
fn constant(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

/// The navmesh (`this + 0`)'s point for the 16-bit vertex index at `vertex`, copied to
/// `destination` (three words).
fn copy_vertex_point(e: &mut Engine, this: Ptr, vertex: u32, destination: u32) {
    let navmesh = e.mem.u32(this.addr());
    let index = u32::from(e.mem.u16(vertex));
    let point = e.call(VERTEX_POINT, &args![navmesh, index]).u32();
    for word in 0..3u32 {
        let value = e.mem.u32(point.wrapping_add(4 * word));
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// The navmesh's point for the 16-bit vertex index at `vertex` (its address).
fn vertex_point(e: &mut Engine, this: Ptr, vertex: u32) -> u32 {
    let navmesh = e.mem.u32(this.addr());
    let index = u32::from(e.mem.u16(vertex));
    e.call(VERTEX_POINT, &args![navmesh, index]).u32()
}

/// `(a + b + c) * factor` into `result` through the two temporaries, as the exe chains
/// `operator+`, `operator+` and the scale.
fn centroid(
    e: &mut Engine,
    a: u32,
    b: u32,
    c: u32,
    temporaries: [u32; 2],
    result: u32,
    factor: u32,
) {
    let sum = e.call(POINT_ADD, &args![a, temporaries[0], b]).u32();
    let sum = e.call(POINT_ADD, &args![sum, temporaries[1], c]).u32();
    e.call(POINT_SCALE, &args![sum, result, factor]);
}

/// The element word of the array at `this + offset` at `index` (through `00877a30`).
fn element(e: &mut Engine, this: Ptr, offset: u32, index: u32) -> u32 {
    let slot = e
        .call(ARRAY_ELEMENT, &args![this.addr() + offset, index])
        .u32();
    e.mem.u32(slot)
}

/// A 2-D point (`00452dc0`) at `destination` from the x and y of the point at `point`.
fn point2(e: &mut Engine, destination: u32, point: u32) {
    let y = x87_quiet_bits(e.mem.u32(point.wrapping_add(4)));
    let x = x87_quiet_bits(e.mem.u32(point));
    e.call(POINT2_CONSTRUCTOR, &args![destination, x, y]);
}

// Translated from 006bec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&a, &b, &c)`, three vertex records whose first word is a
/// navmesh vertex index): the three points, their centroid (times one third, `0106c2dc`);
/// for each vertex record of the array at `this + 0x1C` other than the three, the triangle
/// test `006b9cf0(a, b, c, point, &local, 128, 180)` answering true makes the answer false.
/// Otherwise the answer is whether `006c3570(&centroid, this + 4, 128, 180)` is false.
pub fn fn_006bec40(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr) -> bool {
    // The frame below EBP: temporaries at +0x04 and +0x10, the fourth point at +0x20, the
    // second at +0x30, the test's local at +0x3C, the third at +0x40, the first at +0x4C,
    // the centroid at +0x58.
    e.with_stack(0x64, |e, frame| {
        let frame = frame.addr();
        let (first, second, third) = (frame + 0x4c, frame + 0x30, frame + 0x40);
        copy_vertex_point(e, this, a.addr(), first);
        copy_vertex_point(e, this, b.addr(), second);
        copy_vertex_point(e, this, c.addr(), third);
        let factor = constant(e, ONE_THIRD_LOW);
        centroid(
            e,
            first,
            second,
            third,
            [frame + 0x10, frame + 0x04],
            frame + 0x58,
            factor,
        );
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![this.addr() + 0x1c]).u32() {
            let vertex = element(e, this, 0x1c, index);
            copy_vertex_point(e, this, vertex, frame + 0x20);
            if vertex != a.addr() && vertex != b.addr() && vertex != c.addr() {
                let (low, high) = (constant(e, FLOAT_128), constant(e, FLOAT_180));
                let hit = e
                    .call(
                        TRIANGLE_TEST_006B9CF0,
                        &args![first, second, third, frame + 0x20, frame + 0x3c, low, high],
                    )
                    .bool();
                if hit {
                    return false;
                }
            }
            index += 1;
        }
        let (low, high) = (constant(e, FLOAT_128), constant(e, FLOAT_180));
        !e.call(
            POINT_TEST_006C3570,
            &args![frame + 0x58, this.addr() + 4, low, high],
        )
        .bool()
    })
}

// Translated from 006bede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&a, &b, &c)`): the centroid of the three vertex
/// records' points (times one third, `0106c2e0`) tested with `fn_006bf220`; answers its
/// result.
pub fn fn_006bede0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr) -> bool {
    // The frame below EBP: temporaries at +0x04 and +0x10, the centroid at +0x1C, the
    // second point at +0x28, the third at +0x34, the first at +0x40.
    e.with_stack(0x4c, |e, frame| {
        let frame = frame.addr();
        let (first, second, third) = (frame + 0x40, frame + 0x28, frame + 0x34);
        copy_vertex_point(e, this, a.addr(), first);
        copy_vertex_point(e, this, b.addr(), second);
        copy_vertex_point(e, this, c.addr(), third);
        let factor = constant(e, ONE_THIRD);
        centroid(
            e,
            first,
            second,
            third,
            [frame + 0x10, frame + 0x04],
            frame + 0x1c,
            factor,
        );
        fn_006bf220(e, this, Ptr::new(frame + 0x1c))
    })
}

// Translated from 006beea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&a, &b)`, two vertex records): false when both have the
/// byte at +2 set, or either has +2 clear and +6 set. With the two points as 2-D points
/// (`00452dc0`): when the records' +2 bytes differ, the midpoint (times one half,
/// `01016248`) must pass `fn_006bf220`, else false. Then, from the last edge of the array at
/// `this + 0x30` down: an edge joining the two records (either way) gives false; an edge
/// touching either is skipped; any other edge whose 2-D segment `006ba010` finds crossing
/// the pair's gives false. True when no edge did.
pub fn fn_006beea0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) -> bool {
    let (a, b) = (a.addr(), b.addr());
    if e.mem.u8(a + 2) != 0 && e.mem.u8(b + 2) != 0 {
        return false;
    }
    if e.mem.u8(a + 2) == 0 && e.mem.u8(a + 6) != 0 {
        return false;
    }
    if e.mem.u8(b + 2) == 0 && e.mem.u8(b + 6) != 0 {
        return false;
    }
    // The frame below EBP: the sum at +0x04, the edge's 2-D points at +0x14 and +0x1C, the
    // midpoint at +0x2C, b's 2-D point at +0x3C, a's at +0x48.
    e.with_stack(0x50, |e, frame| {
        let frame = frame.addr();
        let point_a = vertex_point(e, this, a);
        let point_b = vertex_point(e, this, b);
        point2(e, frame + 0x48, point_a);
        point2(e, frame + 0x3c, point_b);
        if e.mem.u8(a + 2) != e.mem.u8(b + 2) {
            let sum = e
                .call(POINT_ADD, &args![point_a, frame + 0x04, point_b])
                .u32();
            let half = constant(e, ONE_HALF);
            e.call(POINT_SCALE, &args![sum, frame + 0x2c, half]);
            if !fn_006bf220(e, this, Ptr::new(frame + 0x2c)) {
                return false;
            }
        }
        let mut index = e
            .call(ARRAY_COUNT, &args![this.addr() + 0x30])
            .u32()
            .wrapping_sub(1);
        while index != u32::MAX {
            let edge_first = |e: &mut Engine| {
                let edge = element(e, this, 0x30, index);
                e.mem.u32(edge)
            };
            let edge_second = |e: &mut Engine| {
                let edge = element(e, this, 0x30, index);
                e.mem.u32(edge + 4)
            };
            if edge_first(e) == a && edge_second(e) == b {
                return false;
            }
            if edge_first(e) == b && edge_second(e) == a {
                return false;
            }
            let touches = edge_first(e) == a
                || edge_second(e) == a
                || edge_first(e) == b
                || edge_second(e) == b;
            if !touches {
                let start = edge_first(e);
                let point_start = vertex_point(e, this, start);
                let end = edge_second(e);
                let point_end = vertex_point(e, this, end);
                point2(e, frame + 0x14, point_start);
                point2(e, frame + 0x1c, point_end);
                let crossed = e
                    .call(
                        SEGMENT_TEST_006BA010,
                        &args![frame + 0x48, frame + 0x3c, frame + 0x14, frame + 0x1c],
                    )
                    .bool();
                if crossed {
                    return false;
                }
            }
            index = index.wrapping_sub(1);
        }
        true
    })
}

// Translated from 006bf170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(vertex index)`): the position in the array at
/// `this + 0x1C` of the first vertex record whose 16-bit index is `vertex`, or -1.
pub fn fn_006bf170(e: &mut Engine, this: Ptr, vertex: u16) -> u32 {
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![this.addr() + 0x1c]).u32() {
        let record = element(e, this, 0x1c, index);
        if e.mem.u16(record) == vertex {
            return index;
        }
        index += 1;
    }
    u32::MAX
}

// Translated from 006bf1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&record)`): true when the record's byte at +3 or +4 is
/// set; otherwise `fn_006bf220` on its vertex point.
pub fn fn_006bf1d0(e: &mut Engine, this: Ptr, record: Ptr) -> bool {
    let record = record.addr();
    if e.mem.u8(record + 3) != 0 || e.mem.u8(record + 4) != 0 {
        return true;
    }
    let point = vertex_point(e, this, record);
    fn_006bf220(e, this, Ptr::new(point))
}

// Translated from 006bf220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&point)`): whether `00697730` on the navmesh
/// (`this + 0`) answers true for `point` and any of the 16-bit indices in the array at
/// `[this + 8] + 0x20` (with a local and 180, 180).
pub fn fn_006bf220(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    e.with_stack(4, |e, local| {
        let mut index = 0u32;
        loop {
            let array = e.mem.u32(this.addr() + 8).wrapping_add(0x20);
            if index >= e.call(ARRAY_COUNT, &args![array]).u32() {
                return false;
            }
            let value = constant(e, FLOAT_180);
            let array = e.mem.u32(this.addr() + 8).wrapping_add(0x20);
            let slot = e.call(OTHER_ARRAY_ELEMENT, &args![array, index]).u32();
            let triangle = u32::from(e.mem.u16(slot));
            let navmesh = e.mem.u32(this.addr());
            let hit = e
                .call(
                    NAVMESH_TEST_00697730,
                    &args![navmesh, triangle, point, local, value, value],
                )
                .bool();
            if hit {
                return true;
            }
            index += 1;
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006bec40, fn_006bec40(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006bede0, fn_006bede0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006beea0, fn_006beea0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x006bf170, fn_006bf170(Ptr, u16) -> u32),
        entry!(0x006bf1d0, fn_006bf1d0(Ptr, Ptr) -> bool),
        entry!(0x006bf220, fn_006bf220(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The navmesh's points live at `POINTS + 12 * index`.
    const POINTS: u32 = 0x0050_0000;

    /// An engine with the point and array helpers doubled: an array object holds a
    /// descriptor pointer whose first word is the count and whose elements follow.
    fn cutter_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(POINTS, 0x1000);
        e.register(VERTEX_POINT, |_, a| answer(POINTS + 12 * a[1]));
        e.register(ARRAY_COUNT, |e, a| answer(e.mem.u32(e.mem.u32(a[0]))));
        e.register(ARRAY_ELEMENT, |e, a| answer(e.mem.u32(a[0]) + 4 + 4 * a[1]));
        e.register(OTHER_ARRAY_ELEMENT, |e, a| {
            answer(e.mem.u32(a[0]) + 4 + 4 * a[1])
        });
        e.register(POINT_ADD, |e, a| {
            for w in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * w) + e.mem.f32(a[2] + 4 * w);
                e.mem.set_f32(a[1] + 4 * w, sum);
            }
            answer(a[1])
        });
        e.register(POINT_SCALE, |e, a| {
            for w in 0..3 {
                let v = e.mem.f32(a[0] + 4 * w) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * w, v);
            }
            answer(a[1])
        });
        e.register(POINT2_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        for (address, value) in [
            (ONE_THIRD_LOW, 0.5f32),
            (ONE_THIRD, 0.5),
            (ONE_HALF, 0.5),
            (FLOAT_128, 128.0),
            (FLOAT_180, 180.0),
        ] {
            e.map(address, 4);
            e.set_global::<f32>(address, value);
        }
        e
    }

    fn set_point(e: &mut Engine, index: u32, point: [f32; 3]) {
        for (w, v) in point.into_iter().enumerate() {
            e.mem.set_f32(POINTS + 12 * index + 4 * w as u32, v);
        }
    }

    /// A vertex record for vertex `index` (16-bit index, then flag bytes).
    fn record(e: &mut Engine, index: u16) -> u32 {
        let r = e.mem.alloc(8);
        e.mem.set_u16(r, index);
        r
    }

    /// An array (descriptor pointer at `slot`) of the given element words.
    fn array(e: &mut Engine, slot: u32, elements: &[u32]) {
        let d = e.mem.alloc(4 + 4 * elements.len() as u32);
        e.mem.set_u32(d, elements.len() as u32);
        for (i, v) in elements.iter().enumerate() {
            e.mem.set_u32(d + 4 + 4 * i as u32, *v);
        }
        e.mem.set_u32(slot, d);
    }

    fn cutter(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x4000);
        Ptr::new(this)
    }

    #[test]
    fn fn_006bf170_finds_the_record_position() {
        let mut e = cutter_engine();
        let this = cutter(&mut e);
        let (r7, r9) = (record(&mut e, 7), record(&mut e, 9));
        array(&mut e, this.addr() + 0x1c, &[r7, r9]);
        assert_eq!(e.call(0x006bf170, &args![this, 9u32]).u32(), 1);
        assert_eq!(e.call(0x006bf170, &args![this, 3u32]).u32(), u32::MAX);
    }

    /// A cutter whose `[this + 8] + 0x20` array holds the triangles 4 and 6, with the navmesh
    /// test answering true for triangle 6.
    fn cutter_with_triangles(e: &mut Engine) -> Ptr {
        let this = cutter(e);
        let owner = e.mem.alloc(0x24);
        e.mem.set_u32(this.addr() + 8, owner);
        // The element helper answers d + 4 + 4i: the slots hold the 16-bit values.
        let d = e.mem.alloc(12);
        e.mem.set_u32(d, 2);
        e.mem.set_u32(d + 4, 4);
        e.mem.set_u32(d + 8, 6);
        e.mem.set_u32(owner + 0x20, d);
        this
    }

    #[test]
    fn fn_006bf220_tests_each_triangle() {
        let mut e = cutter_engine();
        let this = cutter_with_triangles(&mut e);
        e.register(NAVMESH_TEST_00697730, |_, a| answer(u32::from(a[1] == 6)));
        assert!(e.call(0x006bf220, &args![this, 0x7000u32]).bool());
        let tests = calls_at(&e, NAVMESH_TEST_00697730);
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0][..3], [0x4000, 4, 0x7000]);
        assert_eq!(tests[0][4..], [180.0f32.to_bits(), 180.0f32.to_bits()]);
        e.register(NAVMESH_TEST_00697730, |_, _| answer(0));
        assert!(!e.call(0x006bf220, &args![this, 0x7000u32]).bool());
    }

    #[test]
    fn fn_006bf1d0_tests_free_records_only() {
        let mut e = cutter_engine();
        let this = cutter_with_triangles(&mut e);
        e.register(NAVMESH_TEST_00697730, |_, _| answer(0));
        let r = record(&mut e, 3);
        e.mem.set_u8(r + 4, 1);
        assert!(e.call(0x006bf1d0, &args![this, r]).bool());
        assert!(calls_at(&e, NAVMESH_TEST_00697730).is_empty());
        e.mem.set_u8(r + 4, 0);
        assert!(!e.call(0x006bf1d0, &args![this, r]).bool());
        assert_eq!(calls_at(&e, NAVMESH_TEST_00697730)[0][2], POINTS + 36);
    }

    #[test]
    fn fn_006bede0_tests_the_centroid() {
        let mut e = cutter_engine();
        let this = cutter_with_triangles(&mut e);
        e.register(NAVMESH_TEST_00697730, |e, a| {
            answer(u32::from(
                e.mem.f32(a[2]) == 3.0 && e.mem.f32(a[2] + 8) == 1.5,
            ))
        });
        set_point(&mut e, 1, [1.0, 0.0, 1.0]);
        set_point(&mut e, 2, [2.0, 0.0, 1.0]);
        set_point(&mut e, 3, [3.0, 0.0, 1.0]);
        let (a, b, c) = (record(&mut e, 1), record(&mut e, 2), record(&mut e, 3));
        // The doubles scale by the constant (0.5 here): (6, 0, 3) * 0.5.
        assert!(e.call(0x006bede0, &args![this, a, b, c]).bool());
    }

    #[test]
    fn fn_006bec40_fails_on_a_point_inside_or_a_blocked_centroid() {
        let mut e = cutter_engine();
        let this = cutter(&mut e);
        set_point(&mut e, 1, [1.0, 0.0, 0.0]);
        set_point(&mut e, 2, [0.0, 1.0, 0.0]);
        set_point(&mut e, 3, [0.0, 0.0, 1.0]);
        set_point(&mut e, 4, [9.0, 9.0, 9.0]);
        let (a, b, c, d) = (
            record(&mut e, 1),
            record(&mut e, 2),
            record(&mut e, 3),
            record(&mut e, 4),
        );
        array(&mut e, this.addr() + 0x1c, &[a, d, b, c]);
        e.register(TRIANGLE_TEST_006B9CF0, |e, a| {
            answer(u32::from(e.mem.f32(a[3]) == 9.0))
        });
        e.register(POINT_TEST_006C3570, |_, _| answer(0));
        assert!(!e.call(0x006bec40, &args![this, a, b, c]).bool());
        // Only the fourth record is tested, with the three points.
        let tests = calls_at(&e, TRIANGLE_TEST_006B9CF0);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0][5..], [128.0f32.to_bits(), 180.0f32.to_bits()]);
        assert!(calls_at(&e, POINT_TEST_006C3570).is_empty());
        e.register(TRIANGLE_TEST_006B9CF0, |_, _| answer(0));
        assert!(e.call(0x006bec40, &args![this, a, b, c]).bool());
        let point_test = &calls_at(&e, POINT_TEST_006C3570)[0];
        assert_eq!(point_test[1], this.addr() + 4);
        assert_eq!(e.mem.f32(point_test[0] + 4), 0.5);
        e.register(POINT_TEST_006C3570, |_, _| answer(1));
        assert!(!e.call(0x006bec40, &args![this, a, b, c]).bool());
    }

    #[test]
    fn fn_006beea0_checks_flags_midpoint_and_edges() {
        let mut e = cutter_engine();
        let this = cutter_with_triangles(&mut e);
        e.register(NAVMESH_TEST_00697730, |_, _| answer(1));
        e.register(SEGMENT_TEST_006BA010, |e, a| {
            answer(u32::from(e.mem.f32(a[2]) == 5.0))
        });
        set_point(&mut e, 1, [0.0, 0.0, 0.0]);
        set_point(&mut e, 2, [2.0, 0.0, 0.0]);
        set_point(&mut e, 3, [1.0, -1.0, 0.0]);
        set_point(&mut e, 4, [1.0, 1.0, 0.0]);
        set_point(&mut e, 5, [5.0, -1.0, 0.0]);
        let (a, b, c, d, f) = (
            record(&mut e, 1),
            record(&mut e, 2),
            record(&mut e, 3),
            record(&mut e, 4),
            record(&mut e, 5),
        );
        // Flag checks.
        e.mem.set_u8(a + 2, 1);
        e.mem.set_u8(b + 2, 1);
        assert!(!e.call(0x006beea0, &args![this, a, b]).bool());
        e.mem.set_u8(b + 2, 0);
        e.mem.set_u8(b + 6, 1);
        assert!(!e.call(0x006beea0, &args![this, a, b]).bool());
        e.mem.set_u8(b + 6, 0);
        // Differing +2 bytes: the midpoint is tested (navmesh test true), then the edges.
        let edge = |e: &mut Engine, from: u32, to: u32| {
            let edge = e.mem.alloc(8);
            e.mem.set_u32(edge, from);
            e.mem.set_u32(edge + 4, to);
            edge
        };
        let touching = edge(&mut e, b, c);
        let crossing = edge(&mut e, c, d);
        array(&mut e, this.addr() + 0x30, &[crossing, touching]);
        assert!(e.call(0x006beea0, &args![this, a, b]).bool());
        let midpoint = calls_at(&e, NAVMESH_TEST_00697730)[0][2];
        assert_eq!(e.mem.f32(midpoint), 1.0);
        let segments = calls_at(&e, SEGMENT_TEST_006BA010);
        assert_eq!(segments.len(), 1);
        assert_eq!(e.mem.f32(segments[0][3] + 4), 1.0);
        // A crossing edge found by the segment test.
        let blocking = edge(&mut e, f, d);
        array(&mut e, this.addr() + 0x30, &[blocking]);
        assert!(!e.call(0x006beea0, &args![this, a, b]).bool());
        // The pair's own edge, either way round, gives false at once.
        let own = edge(&mut e, b, a);
        array(&mut e, this.addr() + 0x30, &[own]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006beea0, &args![this, a, b]).bool());
        assert!(calls_at(&e, SEGMENT_TEST_006BA010).is_empty());
        // A failed midpoint test gives false before the edges.
        e.register(NAVMESH_TEST_00697730, |_, _| answer(0));
        assert!(!e.call(0x006beea0, &args![this, a, b]).bool());
    }
}
