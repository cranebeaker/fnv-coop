//! `Havok SDK/hkppointtopathdrawer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::copy_vec;

/// `hkpPrimitiveDrawer::setDisplayHandler` (`00e10a10`, `this` = the drawer's primitive
/// drawer at `+0xc0`, `(display handler)`).
const SET_DISPLAY_HANDLER: u32 = 0x00e1_0a10;
/// `hkpConstraintDrawer::updateCommonParameters` (`00de47e0`, `this` = the drawer,
/// `(constraint, first block, second block)`), `drawPivots` (`00de44c0`) and `drawBodyFrames`
/// (`00de4530`) (both `this` = the drawer, `(id, tag)`).
const UPDATE_COMMON_PARAMETERS: u32 = 0x00de_47e0;
const DRAW_PIVOTS: u32 = 0x00de_44c0;
const DRAW_BODY_FRAMES: u32 = 0x00de_4530;
/// `hkpPointToPathConstraintData::calcPivot` (engine map, `00dee820`, `this` = the constraint
/// data, `(runtime, destination)`).
const CALC_PIVOT: u32 = 0x00de_e820;
/// `hkTransform::setMul` (`00cb2240`, `this` = the result, `(a, b)`) and
/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = the result, `(transform, point)`).
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkColor::rgbFromFloats` (`00cabb30`, cdecl, four floats; returns the colour).
const RGB_FROM_FLOATS: u32 = 0x00ca_bb30;
/// The `0.25` float in the exe image (`0101622c`).
const QUARTER: u32 = 0x0101_622c;
/// The memory router (`00c85750`): its field `+0x10` is the allocator, whose virtual function
/// at offset `0x10` frees `(data, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00de7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathDrawer::drawConstraint` (engine map): `this` is the drawer and the four stack
/// words are the constraint (`+0xc` is its data, `+0x14` its runtime), the display handler and
/// the two pass-through words `id` and `tag`.
///
/// Hands the display handler to the primitive drawer (`this + 0xc0`), lets
/// `updateCommonParameters` fill two 64-byte blocks, copies the translation vector of the
/// first block (`+0x30`) to `this + 0x50`, computes the pivot of the data from the runtime
/// (`runtime + 0xf0`, into `this + 0x40`) and draws the body frames and the pivots. If the data
/// has a path object (`+0x18`): multiplies the first block with the data's transform at
/// `+0x70`, lets the path object fill an array of points (virtual function `0x2c`, the array
/// header is `(pointer, size, capacity)` = `(0, 0, 0x80000000)`), moves each consecutive pair of
/// points by the product and draws a line through the display handler (virtual function `0x24`,
/// `(second point, first point, colour, id, tag)`, the colour from `rgbFromFloats(0, 0.25, 1,
/// 1)`), then frees the array through the allocator unless its capacity word has the top bit
/// set.
pub fn hkp_point_to_path_drawer_draw_constraint(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    id: u32,
    tag: u32,
) {
    let base = this.addr();
    let data = e.mem.u32(constraint + 0xc);
    e.call(SET_DISPLAY_HANDLER, &args![base + 0xc0, display_handler]);
    // The frame's slots are addressed from the stack pointer after the prologue; the block
    // starts 0xc bytes above it.
    e.with_stack(0x104, |e, frame| {
        let f = frame.addr().wrapping_sub(0xc);
        let first_block = f + 0x50;
        let product = f + 0x90;
        let second_block = f + 0xd0;
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![base, constraint, first_block, second_block],
        );
        copy_vec(e, first_block + 0x30, base + 0x50);
        let runtime = e.mem.u32(constraint + 0x14).wrapping_add(0xf0);
        e.call(CALC_PIVOT, &args![data, runtime, base + 0x40]);
        e.call(DRAW_BODY_FRAMES, &args![base, id, tag]);
        e.call(DRAW_PIVOTS, &args![base, id, tag]);
        if e.mem.u32(data + 0x18) == 0 {
            return;
        }
        e.call(TRANSFORM_SET_MUL, &args![product, first_block, data + 0x70]);
        let path = e.mem.u32(data + 0x18);
        if path == 0 {
            return;
        }
        e.mem.set_u32(f + 0x10, 0);
        e.mem.set_u32(f + 0x14, 0);
        e.mem.set_u32(f + 0x18, 0x8000_0000);
        e.vcall(path, 0x2c, &args![f + 0x10]);
        let count = e.mem.i32(f + 0x14);
        if count > 1 {
            let mut remaining = count - 1;
            let mut offset = 0x10u32;
            loop {
                let points = e.mem.u32(f + 0x10);
                copy_vec(e, points.wrapping_add(offset), f + 0x20);
                e.call(SET_TRANSFORMED_POS, &args![f + 0x40, product, f + 0x20]);
                let points = e.mem.u32(f + 0x10);
                copy_vec(e, points.wrapping_add(offset).wrapping_sub(0x10), f + 0x20);
                e.call(SET_TRANSFORMED_POS, &args![f + 0x30, product, f + 0x20]);
                let green = x87_quiet_bits(e.mem.u32(QUARTER));
                let colour = e
                    .call(
                        RGB_FROM_FLOATS,
                        &args![0u32, green, 0x3f80_0000u32, 0x3f80_0000u32],
                    )
                    .u32();
                e.vcall(
                    display_handler,
                    0x24,
                    &args![f + 0x40, f + 0x30, colour, id, tag],
                );
                offset = offset.wrapping_add(0x10);
                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(f + 0x18);
        e.mem.set_u32(f + 0x14, 0);
        if capacity & 0x8000_0000 == 0 {
            let points = e.mem.u32(f + 0x10);
            e.vcall(
                allocator,
                0x10,
                &args![points, (capacity & 0x3fff_ffff) << 4],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00de_7020,
        hkp_point_to_path_drawer_draw_constraint(Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, Vec4};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Stand-ins for the display handler's line function, the path object's fill function and
    /// the allocator's free function.
    const LINE_DOUBLE: u32 = 0x0900_0601;
    const FILL_DOUBLE: u32 = 0x0900_0602;
    const FREE_DOUBLE: u32 = 0x0900_0603;

    /// The two vectors of every drawn line, read when the line function runs.
    type Lines = Rc<RefCell<Vec<(Vec4, Vec4)>>>;

    struct Rig {
        e: Engine,
        drawer: u32,
        constraint: u32,
        data: u32,
        handler: u32,
        lines: Lines,
    }

    /// A drawer, a constraint with data and runtime, a display handler, and (when `points` is
    /// given) a path object that fills the array with `points` and the capacity word
    /// `capacity`. The transform double moves the first lane of a point by 100.
    fn rig(points: Option<(&[Vec4], u32)>) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(QUARTER, 0.25);
        for address in [
            SET_DISPLAY_HANDLER,
            CALC_PIVOT,
            DRAW_BODY_FRAMES,
            DRAW_PIVOTS,
            TRANSFORM_SET_MUL,
            FREE_DOUBLE,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(UPDATE_COMMON_PARAMETERS, |e, a| {
            store_vec(e, a[2] + 0x30, [4.0, 5.0, 6.0, 7.0]);
            Ret::default()
        });
        e.register_double(SET_TRANSFORMED_POS, |e, a| {
            let point = load_vec(e, a[2]);
            store_vec(e, a[0], [point[0] + 100.0, point[1], point[2], point[3]]);
            Ret::default()
        });
        e.register_double(RGB_FROM_FLOATS, |_, _| Ret {
            eax: 0xc0de_0001,
            ..Ret::default()
        });
        let lines: Lines = Rc::default();
        let sink = lines.clone();
        e.register_double(LINE_DOUBLE, move |e, a| {
            sink.borrow_mut()
                .push((load_vec(e, a[1]), load_vec(e, a[2])));
            Ret::default()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x24, LINE_DOUBLE);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(handler, table);
        let allocator_table = e.mem.alloc(0x40);
        e.mem.set_u32(allocator_table + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let data = e.mem.alloc(0x100);
        if let Some((points, capacity)) = points {
            let buffer = e.mem.alloc(0x10 * points.len() as u32 + 0x10);
            for (index, point) in points.iter().enumerate() {
                store_vec(&mut e, buffer + 0x10 * index as u32, *point);
            }
            let count = points.len() as u32;
            let path_table = e.mem.alloc(0x40);
            e.mem.set_u32(path_table + 0x2c, FILL_DOUBLE);
            let path = e.mem.alloc(0x10);
            e.mem.set_u32(path, path_table);
            e.mem.set_u32(data + 0x18, path);
            e.register_double(FILL_DOUBLE, move |e, a| {
                assert_eq!(a[0], path);
                assert_eq!(
                    (e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)),
                    (0, 0, 0x8000_0000)
                );
                e.mem.set_u32(a[1], buffer);
                e.mem.set_u32(a[1] + 4, count);
                e.mem.set_u32(a[1] + 8, capacity);
                Ret::default()
            });
        }
        let runtime = e.mem.alloc(0x200);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_u32(constraint + 0x14, runtime);
        let drawer = e.mem.alloc(0x100);
        Rig {
            e,
            drawer,
            constraint,
            data,
            handler,
            lines,
        }
    }

    impl Rig {
        fn draw(&mut self) -> Log {
            self.e.call_log = Some(vec![]);
            self.e.call(
                0x00de_7020,
                &args![self.drawer, self.constraint, self.handler, 11u32, 22u32],
            );
            self.e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .skip(1)
                .collect()
        }
    }

    fn calls(log: &Log, address: u32) -> Vec<&[u32]> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.as_slice())
            .collect()
    }

    #[test]
    fn without_a_path_only_the_frames_and_pivots_are_drawn() {
        let mut r = rig(None);
        let log = r.draw();
        let update = calls(&log, UPDATE_COMMON_PARAMETERS);
        assert_eq!(update.len(), 1);
        let (first, second) = (update[0][2], update[0][3]);
        assert_eq!(second, first + 0x80);
        let runtime = r.e.mem.u32(r.constraint + 0x14);
        let order: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            [
                SET_DISPLAY_HANDLER,
                UPDATE_COMMON_PARAMETERS,
                CALC_PIVOT,
                DRAW_BODY_FRAMES,
                DRAW_PIVOTS
            ]
        );
        assert_eq!(
            calls(&log, SET_DISPLAY_HANDLER),
            [[r.drawer + 0xc0, r.handler]]
        );
        assert_eq!(
            calls(&log, CALC_PIVOT),
            [[r.data, runtime + 0xf0, r.drawer + 0x40]]
        );
        assert_eq!(calls(&log, DRAW_BODY_FRAMES), [[r.drawer, 11, 22]]);
        assert_eq!(calls(&log, DRAW_PIVOTS), [[r.drawer, 11, 22]]);
        assert_eq!(load_vec(&r.e, r.drawer + 0x50), [4.0, 5.0, 6.0, 7.0]);
        assert!(r.lines.borrow().is_empty());
    }

    #[test]
    fn a_path_of_three_points_draws_two_lines_and_frees_the_array() {
        let points = [
            [1.0, 2.0, 3.0, 1.0],
            [4.0, 5.0, 6.0, 1.0],
            [7.0, 8.0, 9.0, 1.0],
        ];
        let mut r = rig(Some((&points[..], 5)));
        let log = r.draw();
        let update = calls(&log, UPDATE_COMMON_PARAMETERS);
        let first_block = update[0][2];
        let product = first_block + 0x40;
        let data = r.data;
        assert_eq!(
            calls(&log, TRANSFORM_SET_MUL),
            [[product, first_block, data + 0x70]]
        );
        let moved = calls(&log, SET_TRANSFORMED_POS);
        assert_eq!(moved.len(), 4);
        assert!(moved.iter().all(|call| call[1] == product));
        // Each line goes from the moved point i + 1 (this + 0x40 of the frame) to point i.
        let shifted = |point: Vec4| [point[0] + 100.0, point[1], point[2], point[3]];
        assert_eq!(
            *r.lines.borrow(),
            [
                (shifted(points[1]), shifted(points[0])),
                (shifted(points[2]), shifted(points[1]))
            ]
        );
        let lines = calls(&log, LINE_DOUBLE);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0][3..], [0xc0de_0001, 11, 22]);
        assert_eq!(lines[0][0], r.handler);
        let colours = calls(&log, RGB_FROM_FLOATS);
        assert_eq!(colours, [[0, 0x3e80_0000, 0x3f80_0000, 0x3f80_0000]; 2]);
        let freed = calls(&log, FREE_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 5 << 4);
        assert_eq!(load_vec(&r.e, freed[0][1]), points[0]);
    }

    #[test]
    fn a_static_array_is_not_freed_and_short_paths_draw_nothing() {
        let points = [[1.0, 2.0, 3.0, 1.0], [4.0, 5.0, 6.0, 1.0]];
        let mut r = rig(Some((&points[..], 0x8000_0002)));
        let log = r.draw();
        assert_eq!(r.lines.borrow().len(), 1);
        assert!(calls(&log, FREE_DOUBLE).is_empty());
        assert_eq!(calls(&log, MEMORY_ROUTER).len(), 1);
        for count in [0usize, 1] {
            let mut r = rig(Some((&points[..count], 3)));
            let log = r.draw();
            assert!(r.lines.borrow().is_empty());
            assert!(calls(&log, SET_TRANSFORMED_POS).is_empty());
            assert!(calls(&log, RGB_FROM_FLOATS).is_empty());
            let freed = calls(&log, FREE_DOUBLE);
            assert_eq!(freed.len(), 1);
            assert_eq!(freed[0][2], 3 << 4);
        }
    }

    #[test]
    fn the_capacity_word_keeps_only_thirty_bits_for_the_byte_count() {
        let points = [[0.0; 4]];
        let mut r = rig(Some((&points[..], 0x4000_0001)));
        let log = r.draw();
        let freed = calls(&log, FREE_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 0x10);
    }
}
