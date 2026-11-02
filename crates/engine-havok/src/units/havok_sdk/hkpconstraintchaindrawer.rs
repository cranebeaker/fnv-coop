//! `Havok SDK/hkpconstraintchaindrawer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::copy_vec;

/// `hkpPrimitiveDrawer::setDisplayHandler` (`RET 4`, `this` = the drawer at `+0xC0`).
const SET_DISPLAY_HANDLER: u32 = 0x00e1_0a10;
/// `hkVector4::setTransformedPos` (`RET 8`: transform, position; `this` = the result).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkpConstraintDrawer::drawPivots` and `drawBodyFrames` (`RET 8`: id, tag).
const DRAW_PIVOTS: u32 = 0x00de_44c0;
const DRAW_BODY_FRAMES: u32 = 0x00de_4530;

// Translated from 00de6390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00de6390 - a constraint whose type (slot 0x20) is not 0x64, 0x65 or 0x66 leaves the two pivot vectors unwritten; the exe then passes whatever its stack frame held (the previous iteration's pivots, or the frame's leftovers on the first); here that is the previous iteration's pivots, and zero before any.
/// `hkpConstraintChainDrawer::drawConstraint` (Xbox PDB): gives the primitive drawer at
/// `this + 0xC0` the display handler, then draws one line per link of the chain
/// (`[chain + 0x3C] - 1` of them; `[chain + 0x38]` is the array of body pointers and
/// `[chain + 0xC]` the constraint data). For link `i` with bodies `current = bodies[i]` and
/// `next = bodies[i + 1]`: copies the 4-vector transform at `+0xF0` of each (the rotation
/// rows to `this + 0x60` / `+ 0x90`, the translation to `+ 0x20` / `+ 0x30`), asks the
/// constraint data for its type (slot `0x20`), and for 100, 101 or 102 takes the two pivot
/// vectors from `[data + 0x18]` at the running byte offset of that type (strides `0x30`,
/// `0x20` and `0x50`, all three advanced on every link): the first from the offset, the
/// second from offset `+ 0x10`. The pivot of the current body is transformed by the current
/// body's transform into `this + 0x40` and the second pivot by the next body's transform
/// into `this + 0x50`; then `drawPivots` and `drawBodyFrames` (`id`, `tag`) run and the
/// display handler's slot `0x24` draws the line `this + 0x40` to `this + 0x50` in colour
/// `0xFFFF0000`.
pub fn hkp_constraint_chain_drawer_draw_constraint(
    e: &mut Engine,
    this: Ptr,
    chain: Ptr,
    display_handler: Ptr,
    id: u32,
    tag: u32,
) {
    let drawer = this.addr();
    let data = e.mem.u32(chain.addr() + 0xc);
    e.call(SET_DISPLAY_HANDLER, &args![drawer + 0xc0, display_handler]);
    let links = (e.mem.u32(chain.addr() + 0x3c) as i32).wrapping_sub(1);
    if links <= 0 {
        return;
    }
    // The next body's transform at `+0`, the current body's at `+0x40`, the pivots at
    // `+0x80` and `+0x90`.
    e.with_stack(0xa0, |e, frame| {
        let next_transform = frame.addr();
        let current_transform = frame.addr() + 0x40;
        let pivot = frame.addr() + 0x80;
        let pivot_next = frame.addr() + 0x90;
        let mut strides = [0u32; 3];
        let mut link = 0i32;
        loop {
            let bodies = e.mem.u32(chain.addr() + 0x38);
            let current = e
                .mem
                .u32(bodies.wrapping_add((link as u32).wrapping_mul(4)));
            let next = e
                .mem
                .u32(bodies.wrapping_add((link as u32).wrapping_mul(4)) + 4);
            for row in 0..4 {
                copy_vec(e, next + 0xf0 + 0x10 * row, next_transform + 0x10 * row);
                copy_vec(
                    e,
                    current + 0xf0 + 0x10 * row,
                    current_transform + 0x10 * row,
                );
            }
            for row in 0..3 {
                copy_vec(e, next_transform + 0x10 * row, drawer + 0x90 + 0x10 * row);
                copy_vec(
                    e,
                    current_transform + 0x10 * row,
                    drawer + 0x60 + 0x10 * row,
                );
            }
            copy_vec(e, next_transform + 0x30, drawer + 0x30);
            copy_vec(e, current_transform + 0x30, drawer + 0x20);
            let kind = e.vcall(data, 0x20, &args![]).u32();
            let stride = match kind {
                0x64 => Some(strides[0]),
                0x65 => Some(strides[1]),
                0x66 => Some(strides[2]),
                _ => None,
            };
            if let Some(stride) = stride {
                let points = e.mem.u32(data + 0x18).wrapping_add(stride);
                copy_vec(e, points + 0x10, pivot_next);
                copy_vec(e, points, pivot);
            }
            e.call(
                SET_TRANSFORMED_POS,
                &args![drawer + 0x50, next_transform, pivot_next],
            );
            e.call(
                SET_TRANSFORMED_POS,
                &args![drawer + 0x40, current_transform, pivot],
            );
            e.call(DRAW_PIVOTS, &args![this, id, tag]);
            e.call(DRAW_BODY_FRAMES, &args![this, id, tag]);
            e.vcall(
                display_handler.addr(),
                0x24,
                &args![drawer + 0x40, drawer + 0x50, 0xffff_0000u32, id, tag],
            );
            strides[2] = strides[2].wrapping_add(0x50);
            strides[1] = strides[1].wrapping_add(0x20);
            strides[0] = strides[0].wrapping_add(0x30);
            link += 1;
            if link >= links {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00de_6390,
        hkp_constraint_chain_drawer_draw_constraint(Ptr, Ptr, Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const TYPE_DOUBLE: u32 = 0x0900_0020;
    const LINE_DOUBLE: u32 = 0x0900_0124;

    /// The four words at `at`.
    fn quad(e: &Engine, at: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// The scene: a drawer, a chain of `bodies` bodies whose transforms hold distinctive
    /// words, constraint data answering the types in `kinds` one by one (the last repeats),
    /// pivots `0x1000 + n` in the data's point block, and a display handler.
    struct Scene {
        e: Engine,
        log: Log,
        drawer: u32,
        chain: u32,
        handler: u32,
        points: u32,
        bodies: Vec<u32>,
        transform_calls: Rc<RefCell<Vec<(u32, Vec<u32>, Vec<u32>)>>>,
    }

    fn scene(body_count: u32, kinds: Vec<u32>) -> Scene {
        let mut e = engine();
        let log = new_log();
        let drawer = e.mem.alloc(0x100);
        let points = e.mem.alloc(0x400);
        for word in 0..0x100 {
            e.mem.set_u32(points + 4 * word, 0x1000 + word);
        }
        let data_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(data_vtable + 0x20, TYPE_DOUBLE);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, data_vtable);
        e.mem.set_u32(data + 0x18, points);
        let asked = Rc::clone(&log);
        let mut sequence = kinds.into_iter();
        let mut last = 0;
        e.register_double(TYPE_DOUBLE, move |_, a| {
            asked.borrow_mut().push((TYPE_DOUBLE, a.to_vec()));
            if let Some(next) = sequence.next() {
                last = next;
            }
            Ret {
                eax: last,
                ..Ret::default()
            }
        });
        let handler_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(handler_vtable + 0x24, LINE_DOUBLE);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(handler, handler_vtable);
        log_double(&mut e, &log, LINE_DOUBLE, 0);
        let mut bodies = Vec::new();
        for body in 0..body_count {
            let address = e.mem.alloc(0x140);
            for word in 0..16 {
                e.mem
                    .set_u32(address + 0xf0 + 4 * word, (body + 1) * 0x100 + word);
            }
            bodies.push(address);
        }
        let list = e.mem.alloc(0x40);
        for (i, body) in bodies.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *body);
        }
        let chain = e.mem.alloc(0x50);
        e.mem.set_u32(chain + 0xc, data);
        e.mem.set_u32(chain + 0x38, list);
        e.mem.set_u32(chain + 0x3c, body_count);
        log_double(&mut e, &log, SET_DISPLAY_HANDLER, 0);
        log_double(&mut e, &log, DRAW_PIVOTS, 0);
        log_double(&mut e, &log, DRAW_BODY_FRAMES, 0);
        let transform_calls = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&transform_calls);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            seen.borrow_mut().push((
                a[0],
                (0..16).map(|i| e.mem.u32(a[1] + 4 * i)).collect(),
                (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect(),
            ));
            Ret::default()
        });
        Scene {
            e,
            log,
            drawer,
            chain,
            handler,
            points,
            bodies,
            transform_calls,
        }
    }

    fn draw(s: &mut Scene) {
        s.e.call(0x00de_6390, &[s.drawer, s.chain, s.handler, 0x77, 0x88]);
    }

    /// The 16 words of the transform at `+0xf0` of the body.
    fn transform_words(s: &Scene, body: usize) -> Vec<u32> {
        (0..16)
            .map(|i| s.e.mem.u32(s.bodies[body] + 0xf0 + 4 * i))
            .collect()
    }

    #[test]
    fn draws_a_line_per_link_with_each_types_stride_00de6390() {
        for (kind, stride) in [(0x64u32, 0x30u32), (0x65, 0x20), (0x66, 0x50)] {
            let mut s = scene(3, vec![kind]);
            draw(&mut s);
            assert_eq!(
                calls(&s.log, SET_DISPLAY_HANDLER),
                vec![vec![s.drawer + 0xc0, s.handler]]
            );
            let lines = calls(&s.log, LINE_DOUBLE);
            assert_eq!(lines.len(), 2);
            for line in &lines {
                assert_eq!(line[0], s.handler);
                assert_eq!(
                    line[1..],
                    [s.drawer + 0x40, s.drawer + 0x50, 0xffff_0000, 0x77, 0x88]
                );
            }
            assert_eq!(
                calls(&s.log, DRAW_PIVOTS),
                vec![vec![s.drawer, 0x77, 0x88]; 2]
            );
            assert_eq!(
                calls(&s.log, DRAW_BODY_FRAMES),
                vec![vec![s.drawer, 0x77, 0x88]; 2]
            );
            let transforms = s.transform_calls.borrow();
            assert_eq!(transforms.len(), 4);
            for link in 0..2usize {
                let base = s.points + stride * link as u32;
                let words =
                    |at: u32| -> Vec<u32> { (0..4).map(|i| s.e.mem.u32(at + 4 * i)).collect() };
                // The next body's transform takes the second pivot, the current body's the
                // first.
                assert_eq!(transforms[2 * link].0, s.drawer + 0x50);
                assert_eq!(transforms[2 * link].1, transform_words(&s, link + 1));
                assert_eq!(transforms[2 * link].2, words(base + 0x10));
                assert_eq!(transforms[2 * link + 1].0, s.drawer + 0x40);
                assert_eq!(transforms[2 * link + 1].1, transform_words(&s, link));
                assert_eq!(transforms[2 * link + 1].2, words(base));
            }
        }
    }

    #[test]
    fn copies_the_body_transforms_into_the_drawer_00de6390() {
        let mut s = scene(2, vec![0x64]);
        draw(&mut s);
        let current = transform_words(&s, 0);
        let next = transform_words(&s, 1);
        assert_eq!(quad(&s.e, s.drawer + 0x60), current[0..4]);
        assert_eq!(quad(&s.e, s.drawer + 0x70), current[4..8]);
        assert_eq!(quad(&s.e, s.drawer + 0x80), current[8..12]);
        assert_eq!(quad(&s.e, s.drawer + 0x20), current[12..16]);
        assert_eq!(quad(&s.e, s.drawer + 0x90), next[0..4]);
        assert_eq!(quad(&s.e, s.drawer + 0xa0), next[4..8]);
        assert_eq!(quad(&s.e, s.drawer + 0xb0), next[8..12]);
        assert_eq!(quad(&s.e, s.drawer + 0x30), next[12..16]);
    }

    #[test]
    fn a_chain_of_fewer_than_two_bodies_only_sets_the_handler_00de6390() {
        for count in [0u32, 1] {
            let mut s = scene(count, vec![0x64]);
            draw(&mut s);
            assert_eq!(calls(&s.log, SET_DISPLAY_HANDLER).len(), 1);
            assert!(calls(&s.log, LINE_DOUBLE).is_empty());
            assert!(calls(&s.log, TYPE_DOUBLE).is_empty());
            assert!(s.transform_calls.borrow().is_empty());
        }
    }

    #[test]
    fn an_unknown_type_keeps_the_previous_pivots_00de6390() {
        let mut s = scene(3, vec![0x65, 7]);
        draw(&mut s);
        let transforms = s.transform_calls.borrow();
        assert_eq!(transforms.len(), 4);
        // The second link's type is 7: no pivot is read, the first link's are passed again.
        assert_eq!(transforms[2].2, transforms[0].2);
        assert_eq!(transforms[3].2, transforms[1].2);
        assert_eq!(calls(&s.log, LINE_DOUBLE).len(), 2);
    }

    #[test]
    fn an_unknown_first_type_passes_zero_pivots_00de6390() {
        let mut s = scene(2, vec![7]);
        draw(&mut s);
        let transforms = s.transform_calls.borrow();
        assert_eq!(transforms[0].2, vec![0; 4]);
        assert_eq!(transforms[1].2, vec![0; 4]);
    }
}
