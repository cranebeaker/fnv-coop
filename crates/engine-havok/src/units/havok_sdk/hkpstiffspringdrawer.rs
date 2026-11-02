//! `Havok SDK/hkpstiffspringdrawer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`, through `inverse_length`), so the
//! tests allow for the estimate's error.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    inverse_length, load_vec, store_vec, sum3, zip_with, HALF_F32,
};

/// `hkpPrimitiveDrawer::setDisplayHandler` (`00e10a10`, thiscall on the primitive drawer at
/// `this + 0xc0`, one stack word).
const SET_DISPLAY_HANDLER: u32 = 0x00e1_0a10;
/// `hkpConstraintDrawer::updateCommonParameters` (`00de47e0`, thiscall, three stack words).
const UPDATE_COMMON_PARAMETERS: u32 = 0x00de_47e0;
/// `hkVector4::setTransformedPos` (`00c7f640`, thiscall on the output vector, two stack words:
/// the transform and the position).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkpConstraintDrawer::drawPivots` and `drawBodyFrames` (`00de44c0`, `00de4530`, thiscall,
/// two stack words).
const DRAW_PIVOTS: u32 = 0x00de_44c0;
const DRAW_BODY_FRAMES: u32 = 0x00de_4530;
/// `hkColor::rgbFromFloats` (`00cabb30`, cdecl: red, green, blue, alpha as floats).
const RGB_FROM_FLOATS: u32 = 0x00ca_bb30;

// Translated from 00de7580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringDrawer::drawConstraint` (Xbox PDB), `(this, constraint, display_handler, id,
/// tag)`, `RET 0x10`. With `data` the pointer at `constraint + 0xc`: hands the display handler to
/// the primitive drawer at `this + 0xc0`, lets `updateCommonParameters(constraint, A, B)` fill
/// two 64-byte transforms on the stack, sets the pivot vectors `this + 0x50` (transform A of
/// the position `data + 0x30`) and `this + 0x40` (transform B of the position `data + 0x20`),
/// draws the pivots and the body frames with `(id, tag)`, then draws two lines through slot
/// `0x24` of the display handler (the exe passes five stack words, `id` and `tag` last): from
/// the vector at `this + 0x40` to the point `stop`, in the colour
/// `rgbFromFloats(0.0, 0.5, 1.0, 1.0)`, and from `stop` to the vector at `this + 0x50`, in
/// `0xffff0000`. With `delta` the difference of those two vectors,
/// `stop = float at data + 0x44 * normalised(delta) + vector at this + 0x40` in all four
/// lanes, the normalisation being the `RSQRTSS` refinement of the squared length of the first
/// three lanes (zero when that is zero). (Ghidra shows three words for the first line and an
/// uninitialised first argument for the second; the disassembly passes `id` and `tag` pushed
/// before `rgbFromFloats` and `stop` as the second line's start.)
pub fn hkp_stiff_spring_drawer_draw_constraint(
    e: &mut Engine,
    this: Ptr,
    constraint: Ptr,
    display_handler: Ptr,
    id: u32,
    tag: u32,
) {
    let base = this.addr();
    let display = display_handler.addr();
    let data = e.mem.u32(constraint.addr() + 0xc);
    e.call(SET_DISPLAY_HANDLER, &args![base + 0xc0, display]);
    e.with_stack(0x90, |e, frame| {
        let transform_a = frame.addr();
        let transform_b = frame.addr() + 0x40;
        let stop_point = frame.addr() + 0x80;
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![base, constraint, transform_a, transform_b],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![base + 0x50, transform_a, data + 0x30],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![base + 0x40, transform_b, data + 0x20],
        );
        e.call(DRAW_PIVOTS, &args![base, id, tag]);
        e.call(DRAW_BODY_FRAMES, &args![base, id, tag]);
        let length = x87_quiet(e.mem.f32(data + 0x44));
        let start = load_vec(e, base + 0x40);
        let end = load_vec(e, base + 0x50);
        let delta = zip_with(end, start, |end, start| end - start);
        let squares = delta.map(|lane| lane * lane);
        let inverse = inverse_length(e, sum3(squares));
        let direction = delta.map(|lane| inverse * lane);
        let stop = zip_with(direction, start, |direction, start| {
            length * direction + start
        });
        let vtable = e.mem.u32(display);
        let half = x87_quiet(e.global::<f32>(HALF_F32));
        let color = e
            .call(RGB_FROM_FLOATS, &args![0.0f32, half, 1.0f32, 1.0f32])
            .u32();
        store_vec(e, stop_point, stop);
        let draw_line = e.mem.u32(vtable + 0x24);
        e.call(
            draw_line,
            &args![display, base + 0x40, stop_point, color, id, tag],
        );
        e.vcall(
            display,
            0x24,
            &args![stop_point, base + 0x50, 0xffff_0000u32, id, tag],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00de_7580,
        hkp_stiff_spring_drawer_draw_constraint(Ptr, Ptr, Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{THREE_F32, ZERO_F32};
    use std::cell::RefCell;
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const DRAW_LINE_DOUBLE: u32 = 0x0900_0024;
    const DISPLAY_VTABLE: u32 = 0x0900_3000;
    const COLOR: u32 = 0x1234_abcd;

    /// Doubles for every callee, all logging `[address, arguments...]`; the transform
    /// callee copies the position it is given into the output vector; the line drawer logs
    /// the vectors it is given as bits too.
    fn setup(e: &mut Engine) -> EventLog {
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        for address in [
            SET_DISPLAY_HANDLER,
            UPDATE_COMMON_PARAMETERS,
            DRAW_PIVOTS,
            DRAW_BODY_FRAMES,
            RGB_FROM_FLOATS,
        ] {
            let double_log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                let mut entry = vec![address];
                entry.extend_from_slice(a);
                double_log.borrow_mut().push(entry);
                Ret {
                    eax: COLOR,
                    ..Ret::default()
                }
            });
        }
        let transform_log = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            transform_log
                .borrow_mut()
                .push(vec![SET_TRANSFORMED_POS, a[0], a[2]]);
            let position = load_vec(e, a[2]);
            store_vec(e, a[0], position);
            Ret::default()
        });
        e.put_vtable(
            DISPLAY_VTABLE,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, DRAW_LINE_DOUBLE],
        );
        let line_log = Rc::clone(&log);
        e.register_double(DRAW_LINE_DOUBLE, move |e, a| {
            let mut entry = vec![DRAW_LINE_DOUBLE, a[0]];
            for vector in [a[1], a[2]] {
                entry.extend((0..4).map(|lane| e.mem.u32(vector + 4 * lane)));
            }
            entry.extend_from_slice(&a[3..]);
            line_log.borrow_mut().push(entry);
            Ret::default()
        });
        log
    }

    /// The drawer, a constraint whose data holds the two pivot positions and `length`, and
    /// a display handler.
    fn scene(e: &mut Engine, length: f32, a: [f32; 4], b: [f32; 4]) -> (u32, u32, u32) {
        let drawer = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x60);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_f32(data + 0x44, length);
        store_vec(e, data + 0x30, b);
        store_vec(e, data + 0x20, a);
        let display = e.mem.alloc(0x10);
        e.mem.set_u32(display, DISPLAY_VTABLE);
        (drawer, constraint, display)
    }

    fn close(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= 1e-3 * expected.abs().max(1.0)
    }

    fn floats(bits: &[u32]) -> Vec<f32> {
        bits.iter().map(|word| f32::from_bits(*word)).collect()
    }

    #[test]
    fn the_spring_is_drawn_as_two_lines_split_at_the_rest_length() {
        let mut e = Engine::new();
        let log = setup(&mut e);
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [4.0, 6.0, 15.0, 4.5];
        let (drawer, constraint, display) = scene(&mut e, 2.6, a, b);
        e.call(
            0x00de_7580,
            &args![drawer, constraint, display, 77u32, 88u32],
        );
        let log = log.borrow();
        let entries = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|entry| entry[0] == address)
                .cloned()
                .collect()
        };
        assert_eq!(
            entries(SET_DISPLAY_HANDLER),
            vec![vec![SET_DISPLAY_HANDLER, drawer + 0xc0, display]]
        );
        let common = &entries(UPDATE_COMMON_PARAMETERS)[0];
        assert_eq!(common[..3], [UPDATE_COMMON_PARAMETERS, drawer, constraint]);
        assert_eq!(common[4] - common[3], 0x40);
        assert_eq!(
            entries(DRAW_PIVOTS),
            vec![vec![DRAW_PIVOTS, drawer, 77, 88]]
        );
        assert_eq!(
            entries(DRAW_BODY_FRAMES),
            vec![vec![DRAW_BODY_FRAMES, drawer, 77, 88]]
        );
        assert_eq!(
            entries(RGB_FROM_FLOATS),
            vec![vec![
                RGB_FROM_FLOATS,
                0,
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                1.0f32.to_bits()
            ]]
        );
        // The pivot vectors are those the transform callee produced.
        assert_eq!(load_vec(&e, drawer + 0x40), a);
        assert_eq!(load_vec(&e, drawer + 0x50), b);
        // delta = (3, 4, 12, 0.5) has length 13, so the stop point is a + 0.2 * delta.
        let lines = entries(DRAW_LINE_DOUBLE);
        assert_eq!(lines.len(), 2);
        let first = &lines[0];
        assert_eq!(first[1], display);
        assert_eq!(floats(&first[2..6]), a);
        let stop = floats(&first[6..10]);
        let expected = [1.6, 2.8, 5.4, 4.1];
        for lane in 0..4 {
            assert!(
                close(stop[lane], expected[lane]),
                "lane {lane}: {}",
                stop[lane]
            );
        }
        assert_eq!(first[10..], [COLOR, 77, 88]);
        let second = &lines[1];
        assert_eq!(second[2..6], first[6..10]);
        assert_eq!(floats(&second[6..10]), b);
        assert_eq!(second[10..], [0xffff_0000, 77, 88]);
    }

    #[test]
    fn coincident_pivots_leave_the_stop_point_at_the_first_pivot() {
        let mut e = Engine::new();
        let log = setup(&mut e);
        let a = [1.0, 2.0, 3.0, 4.0];
        let (drawer, constraint, display) = scene(&mut e, 5.0, a, a);
        e.call(0x00de_7580, &args![drawer, constraint, display, 1u32, 2u32]);
        let log = log.borrow();
        let first_line = log
            .iter()
            .find(|entry| entry[0] == DRAW_LINE_DOUBLE)
            .expect("a line was drawn");
        assert_eq!(floats(&first_line[6..10]), a);
    }
}
