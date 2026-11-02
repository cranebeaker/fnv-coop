//! `Havok SDK/run_00e3c240` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00e29250::{lane_add, lane_sub};

/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = the result vector, two stack words: the
/// transform and the point).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkpTriangleUtil::isDegenerate` (`00ceb8d0`, cdecl `(out, a, b, c, tolerance)`; answers a
/// pointer to the byte flag it wrote).
const IS_DEGENERATE: u32 = 0x00ce_b8d0;
/// The largest and the most negative float the bounds start from (`0x7f7fffee`, `0xff7fffee`),
/// zero, and the tolerance `isDegenerate` gets.
const BIG: u32 = 0x0104_da48;
const SMALL: u32 = 0x0104_e148;
const ZERO: u32 = 0x0101_1d78;
const DEGENERATE_TOLERANCE: u32 = 0x011b_2e90;

// Translated from 00e3c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: stores the 16-bit `value` at `+0xc` of record `index` (16 bytes
/// each) of the table at `[this + 0x24]`. The function takes two stack words and pops them
/// (`RET 8`). The class is unconfirmed.
pub fn fn_00e3c240(e: &mut Engine, this: Ptr, index: u32, value: u16) {
    let table = e.mem.u32(this.addr() + 0x24);
    e.mem.set_u16(
        index.wrapping_shl(4).wrapping_add(table).wrapping_add(0xc),
        value,
    );
}

// Translated from 00e3c260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: stores the byte `value` at `this + 0x40`. The function takes one
/// stack word and pops it (`RET 4`). The class is unconfirmed.
pub fn fn_00e3c260(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x40, value);
}

/// `MINPS dst, src` on one lane's bits: `dst` when it is smaller, else `src` (a NaN, or equal
/// values, give `src`).
fn lane_min(dst: u32, src: u32) -> u32 {
    if f32::from_bits(dst) < f32::from_bits(src) {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` on one lane's bits: `dst` when it is larger, else `src`.
fn lane_max(dst: u32, src: u32) -> u32 {
    if f32::from_bits(dst) > f32::from_bits(src) {
        dst
    } else {
        src
    }
}

// Translated from 00e3c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the bounding box of the points at `[this + 0x18]` (count at
/// `+0x1c`, 16 bytes each) after `transform`, grown by `margin` plus the float at `this + 0x3c`.
/// The box at `bounds` (two vectors, minimum then maximum) starts as `(big, big, big, 0)` and
/// `(-big, -big, -big, 0)` (floats `0x7f7fffee` and `0xff7fffee` from the exe's data and the
/// zero at `01011d78`); every point goes through `hkVector4::setTransformedPos` (`00c7f640`:
/// `this` = a stack vector, then `transform` and the point) and into `MINPS`/`MAXPS` lane by
/// lane. The margin is the x87 sum `[this + 0x3c] + margin` stored as a float and splat over
/// four lanes; the minimum gets it subtracted (`SUBPS`) and the maximum added (`ADDPS`). The
/// function takes three stack words and pops them (`RET 0xc`) and returns nothing. The class is
/// unconfirmed.
pub fn fn_00e3c270(e: &mut Engine, this: Ptr, transform: Ptr, margin: f32, bounds: Ptr) {
    let (base, bounds) = (this.addr(), bounds.addr());
    let big: u32 = e.global(BIG);
    let zero: u32 = e.global(ZERO);
    let small: u32 = e.global(SMALL);
    for (offset, lane) in [big, big, big, zero].into_iter().enumerate() {
        e.mem.set_u32(bounds + 4 * offset as u32, lane);
    }
    for (offset, lane) in [small, small, small, zero].into_iter().enumerate() {
        e.mem.set_u32(bounds + 0x10 + 4 * offset as u32, lane);
    }
    if (e.mem.u32(base + 0x1c) as i32) > 0 {
        let mut index: i32 = 0;
        let mut offset: u32 = 0;
        e.with_stack(0x10, |e, result| loop {
            let point = e.mem.u32(base + 0x18).wrapping_add(offset);
            e.call(SET_TRANSFORMED_POS, &args![result, transform, point]);
            for lane in 0..4u32 {
                let value = e.mem.u32(result.addr() + 4 * lane);
                let minimum = e.mem.u32(bounds + 4 * lane);
                e.mem.set_u32(bounds + 4 * lane, lane_min(minimum, value));
            }
            for lane in 0..4u32 {
                let value = e.mem.u32(result.addr() + 4 * lane);
                let maximum = e.mem.u32(bounds + 0x10 + 4 * lane);
                e.mem
                    .set_u32(bounds + 0x10 + 4 * lane, lane_max(maximum, value));
            }
            offset = offset.wrapping_add(0x10);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base + 0x1c) as i32 {
                break;
            }
        });
    }
    let radius = x87_load32(e.mem.u32(base + 0x3c));
    let grow = x87_store32(x87_add(radius, x87_m32(margin.to_bits())));
    for lane in 0..4u32 {
        let minimum = e.mem.u32(bounds + 4 * lane);
        e.mem.set_u32(bounds + 4 * lane, lane_sub(minimum, grow));
    }
    for lane in 0..4u32 {
        let maximum = e.mem.u32(bounds + 0x10 + 4 * lane);
        e.mem
            .set_u32(bounds + 0x10 + 4 * lane, lane_add(maximum, grow));
    }
}

/// The index of the first triangle at or after `start` that `hkpTriangleUtil::isDegenerate`
/// (`00ceb8d0`) does not call degenerate, or -1. The triangles are the 16-byte records at
/// `[this + 0x14]` (three vertex indices; count at `+0x18`) into the 16-byte vertices at
/// `[this + 8]`; the exe's tolerance is the float at `011b2e90` through the x87 stack.
/// `flag_offset` is where in a 4-byte stack temporary the flag's address points.
fn first_triangle_that_is_not_degenerate(
    e: &mut Engine,
    this: u32,
    start: i32,
    flag_offset: u32,
) -> i32 {
    let mut index = start;
    while index < e.mem.u32(this + 0x18) as i32 {
        let records = e.mem.u32(this + 0x14);
        let record = records.wrapping_add((index as u32).wrapping_shl(4));
        let vertices = e.mem.u32(this + 8);
        let corner = |e: &Engine, field: u32| {
            e.mem
                .u32(record + field)
                .wrapping_shl(4)
                .wrapping_add(vertices)
        };
        let (first, second, third) = (corner(e, 0), corner(e, 4), corner(e, 8));
        let tolerance = x87_quiet_bits(e.mem.u32(DEGENERATE_TOLERANCE));
        let flag = e.with_stack(4, |e, temporary| {
            e.call(
                IS_DEGENERATE,
                &args![
                    temporary.addr() + flag_offset,
                    first,
                    second,
                    third,
                    tolerance
                ],
            )
            .u32()
        });
        if e.mem.u8(flag) == 0 {
            return index;
        }
        index = index.wrapping_add(1);
    }
    -1
}

// Translated from 00e3c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the index of the first triangle that `hkpTriangleUtil::
/// isDegenerate` (`00ceb8d0`) does not call degenerate, or -1 (`this` in `ECX`; see
/// `first_triangle_that_is_not_degenerate`). The class is unconfirmed.
pub fn fn_00e3c340(e: &mut Engine, this: Ptr) -> i32 {
    first_triangle_that_is_not_degenerate(e, this.addr(), 0, 3)
}

// Translated from 00e3c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: like `00e3c340`, starting after triangle `index` (one stack word,
/// `RET 4`). The class is unconfirmed.
pub fn fn_00e3c3b0(e: &mut Engine, this: Ptr, index: i32) -> i32 {
    first_triangle_that_is_not_degenerate(e, this.addr(), index.wrapping_add(1), 0)
}

// Translated from 00e3c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: builds an object in the block at `out` from triangle `index` (16
/// bytes at `[this + 0x14]`) and returns `out` (`EAX`). When `out` is not null: the float at
/// `this + 0x2c` (through the x87 stack twice) goes to `+0x10`, the word 1 to `+6`, 0 to `+8`,
/// 3 to `+0xc`, the vtable word `010c749c` to `+0`, the triangle's 16-bit word at `+0xc` to
/// `+0x14`, the byte at `this + 0x30` to `+0x16`, 0 to `+0x17` and a zero vector to `+0x50`. Then
/// (also when `out` is null: the exe does not test it again, so it writes to `0x20` and faults)
/// the three vertices of the triangle (indices at `+0`, `+4`, `+8`; 16 bytes each at `[this +
/// 8]`) are copied to `out + 0x20`, `+0x30` and `+0x40`. The function takes two stack words and
/// pops them (`RET 8`). The class is unconfirmed.
pub fn fn_00e3c430(e: &mut Engine, this: Ptr, index: i32, out: Ptr) -> Ptr {
    let base = this.addr();
    let shifted = (index as u32).wrapping_shl(4);
    let target = out.addr();
    if target != 0 {
        let records = e.mem.u32(base + 0x14);
        let radius = x87_quiet_bits(x87_quiet_bits(e.mem.u32(base + 0x2c)));
        let flag = e.mem.u8(base + 0x30);
        let info = e.mem.u16(shifted.wrapping_add(records).wrapping_add(0xc));
        e.mem.set_u32(target + 0x10, radius);
        e.mem.set_u16(target + 6, 1);
        e.mem.set_u32(target + 8, 0);
        e.mem.set_u32(target + 0xc, 3);
        e.mem.set_u32(target, 0x010c_749c);
        e.mem.set_u16(target + 0x14, info);
        e.mem.set_u8(target + 0x16, flag);
        for word in 0..4 {
            e.mem.set_u32(target + 0x50 + 4 * word, 0);
        }
        e.mem.set_u8(target + 0x17, 0);
    }
    for (field, destination) in [(0u32, 0x20u32), (4, 0x30), (8, 0x40)] {
        let records = e.mem.u32(base + 0x14);
        let vertex = e
            .mem
            .u32(shifted.wrapping_add(records).wrapping_add(field))
            .wrapping_shl(4)
            .wrapping_add(e.mem.u32(base + 8));
        for word in 0..4u32 {
            let value = e.mem.u32(vertex + 4 * word);
            e.mem
                .set_u32(target.wrapping_add(destination + 4 * word), value);
        }
    }
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_c240, fn_00e3c240(Ptr, u32, u16)),
        entry!(0x00e3_c260, fn_00e3c260(Ptr, u8)),
        entry!(0x00e3_c270, fn_00e3c270(Ptr, Ptr, f32, Ptr)),
        entry!(0x00e3_c340, fn_00e3c340(Ptr) -> i32),
        entry!(0x00e3_c3b0, fn_00e3c3b0(Ptr, i32) -> i32),
        entry!(0x00e3_c430, fn_00e3c430(Ptr, i32, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(0x0104_da48, 0x7f7f_ffeeu32);
        e.set_global(0x0104_e148, 0xff7f_ffeeu32);
        e.set_global(0x0101_1d78, 0u32);
        e
    }

    /// The word lands at +0xc of the record, nothing else changes.
    #[test]
    fn fn_00e3c240_stores_the_word_in_the_record() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        e.mem.write(table, &[0x55; 0x40]);
        e.mem.set_u32(this + 0x24, table);
        e.call(0x00e3_c240, &args![this, 2u32, 0xbeefu32]);
        assert_eq!(e.mem.u16(table + 0x2c), 0xbeef);
        assert_eq!(e.mem.u16(table + 0x2e), 0x5555);
        assert_eq!(e.mem.u16(table + 0x1c), 0x5555);
    }

    /// The byte lands at +0x40.
    #[test]
    fn fn_00e3c260_stores_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.mem.write(this, &[0x55; 0x50]);
        e.call(0x00e3_c260, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u8(this + 0x40), 0x78);
        assert_eq!(e.mem.u8(this + 0x41), 0x55);
    }

    fn set_vec(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, value) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    fn get_vec(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    /// The transform double copies the point; the box covers the three points and grows by
    /// the margin (0.5 from the object plus 0.25).
    #[test]
    fn fn_00e3c270_builds_the_grown_bounds() {
        let mut e = engine();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let calls = Rc::clone(&log);
        e.register_double(0x00c7_f640, move |e, a| {
            calls.borrow_mut().push(a.to_vec());
            for lane in 0..4 {
                let word = e.mem.u32(a[2] + 4 * lane);
                e.mem.set_u32(a[0] + 4 * lane, word);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let points = e.mem.alloc(0x30);
        set_vec(&mut e, points, [1.0, 2.0, 3.0, 0.0]);
        set_vec(&mut e, points + 0x10, [-1.0, 5.0, 0.0, 0.0]);
        set_vec(&mut e, points + 0x20, [4.0, -2.0, 2.0, 0.0]);
        e.mem.set_u32(this + 0x18, points);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_f32(this + 0x3c, 0.5);
        let bounds = e.mem.alloc(0x20);
        e.call(0x00e3_c270, &args![this, 0x7000u32, 0.25f32, bounds]);
        assert_eq!(get_vec(&e, bounds), [-1.75, -2.75, -0.75, -0.75]);
        assert_eq!(get_vec(&e, bounds + 0x10), [4.75, 5.75, 3.75, 0.75]);
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        assert_eq!(log[1][1..], [0x7000, points + 0x10]);
    }

    /// No points: the box keeps its start values, grown by the margin.
    #[test]
    fn fn_00e3c270_without_points_grows_the_start_box() {
        let mut e = engine();
        let this = e.mem.alloc(0x50);
        e.mem.set_f32(this + 0x3c, 1.0);
        let bounds = e.mem.alloc(0x20);
        e.call(0x00e3_c270, &args![this, 0x7000u32, 1.0f32, bounds]);
        assert_eq!(e.mem.f32(bounds), f32::from_bits(0x7f7f_ffee));
        assert_eq!(e.mem.f32(bounds + 0xc), -2.0);
        assert_eq!(e.mem.f32(bounds + 0x10), f32::from_bits(0xff7f_ffee));
        assert_eq!(e.mem.f32(bounds + 0x1c), 2.0);
    }

    /// Three triangles over a vertex table; the degenerate double answers from a list.
    struct Mesh {
        e: Engine,
        this: u32,
        vertices: u32,
        log: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    fn mesh(answers: Vec<u8>) -> Mesh {
        let mut e = engine();
        e.set_global(0x011b_2e90, 0x3c23_d70au32);
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let calls = Rc::clone(&log);
        let remaining = Rc::new(RefCell::new(answers.into_iter()));
        e.register_double(0x00ce_b8d0, move |e, a| {
            calls.borrow_mut().push(a.to_vec());
            let answer = remaining.borrow_mut().next().unwrap_or(0);
            e.mem.set_u8(a[0], answer);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x80);
        let triangles = e.mem.alloc(0x40);
        for (index, corners) in [[0u32, 1, 2], [1, 2, 3], [2, 3, 4]].into_iter().enumerate() {
            for (field, corner) in corners.into_iter().enumerate() {
                e.mem
                    .set_u32(triangles + 16 * index as u32 + 4 * field as u32, corner);
            }
        }
        e.mem.set_u32(this + 8, vertices);
        e.mem.set_u32(this + 0x14, triangles);
        e.mem.set_u32(this + 0x18, 3);
        Mesh {
            e,
            this,
            vertices,
            log,
        }
    }

    /// The first two are degenerate: the third's index comes back; the calls get the vertex
    /// addresses and the tolerance.
    #[test]
    fn fn_00e3c340_finds_the_first_good_triangle() {
        let mut m = mesh(vec![1, 1, 0]);
        assert_eq!(m.e.call(0x00e3_c340, &args![m.this]).i32(), 2);
        let log = m.log.borrow();
        assert_eq!(log.len(), 3);
        let v = m.vertices;
        assert_eq!(log[2][1..], [v + 0x20, v + 0x30, v + 0x40, 0x3c23_d70a]);
        assert_eq!(log[0][1..4], [v, v + 0x10, v + 0x20]);
    }

    /// All degenerate: -1. A signalling NaN tolerance goes through the x87 stack and is quiet.
    #[test]
    fn fn_00e3c340_all_degenerate_gives_minus_one() {
        let mut m = mesh(vec![1, 1, 1]);
        m.e.set_global(0x011b_2e90, 0x7f80_0001u32);
        assert_eq!(m.e.call(0x00e3_c340, &args![m.this]).i32(), -1);
        assert_eq!(m.log.borrow().len(), 3);
        assert_eq!(m.log.borrow()[0][4], 0x7fc0_0001);
    }

    /// No triangles: -1 without a call.
    #[test]
    fn fn_00e3c340_without_triangles_gives_minus_one() {
        let mut m = mesh(vec![]);
        m.e.mem.set_u32(m.this + 0x18, 0);
        assert_eq!(m.e.call(0x00e3_c340, &args![m.this]).i32(), -1);
        assert!(m.log.borrow().is_empty());
    }

    /// The search starts after the given index; at the last index there is nothing left.
    #[test]
    fn fn_00e3c3b0_starts_after_the_index() {
        let mut m = mesh(vec![1, 0]);
        assert_eq!(m.e.call(0x00e3_c3b0, &args![m.this, 0i32]).i32(), 2);
        let log = m.log.borrow();
        assert_eq!(log.len(), 2);
        let v = m.vertices;
        assert_eq!(log[0][1..4], [v + 0x10, v + 0x20, v + 0x30]);
        drop(log);
        m.log.borrow_mut().clear();
        assert_eq!(m.e.call(0x00e3_c3b0, &args![m.this, 2i32]).i32(), -1);
        assert!(m.log.borrow().is_empty());
    }

    /// The object is built in the block, with the triangle's vertices copied in; the first
    /// vertex index is 1 for triangle 1.
    #[test]
    fn fn_00e3c430_builds_the_object() {
        let mut m = mesh(vec![]);
        let this = m.this;
        m.e.mem.set_u32(this + 0x2c, 0x7f80_0001);
        m.e.mem.set_u8(this + 0x30, 9);
        let triangles = m.e.mem.u32(this + 0x14);
        m.e.mem.set_u16(triangles + 16 + 0xc, 0x1357);
        for vertex in 0..5u32 {
            for lane in 0..4u32 {
                m.e.mem
                    .set_u32(m.vertices + 16 * vertex + 4 * lane, 0x100 * vertex + lane);
            }
        }
        let out = m.e.mem.alloc(0x60);
        m.e.mem.write(out, &[0x77; 0x60]);
        assert_eq!(m.e.call(0x00e3_c430, &args![this, 1i32, out]).u32(), out);
        assert_eq!(m.e.mem.u32(out + 0x10), 0x7fc0_0001);
        assert_eq!(m.e.mem.u16(out + 6), 1);
        assert_eq!(m.e.mem.u32(out + 8), 0);
        assert_eq!(m.e.mem.u32(out + 0xc), 3);
        assert_eq!(m.e.mem.u32(out), 0x010c_749c);
        assert_eq!(m.e.mem.u16(out + 0x14), 0x1357);
        assert_eq!(m.e.mem.u8(out + 0x16), 9);
        assert_eq!(m.e.mem.u8(out + 0x17), 0);
        assert_eq!(m.e.mem.bytes(out + 0x50, 16), vec![0; 16]);
        assert_eq!(m.e.mem.u16(out + 4), 0x7777);
        for (corner, destination) in [(1u32, 0x20u32), (2, 0x30), (3, 0x40)] {
            for lane in 0..4u32 {
                let expected = 0x100 * corner + lane;
                assert_eq!(m.e.mem.u32(out + destination + 4 * lane), expected);
            }
        }
    }

    /// A null block is not tested again by the exe: it writes to address 0x20 and faults.
    #[test]
    #[should_panic]
    fn fn_00e3c430_null_block_faults() {
        let mut m = mesh(vec![]);
        let this = m.this;
        m.e.call(0x00e3_c430, &args![this, 0i32, 0u32]);
    }
}
