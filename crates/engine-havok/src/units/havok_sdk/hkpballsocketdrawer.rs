//! `Havok SDK/hkpballsocketdrawer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpPrimitiveDrawer::setDisplayHandler` (`00e10a10`, `this` = the drawer's primitive
/// drawer at `+0xc0`, `(display handler)`).
const SET_DISPLAY_HANDLER: u32 = 0x00e1_0a10;
/// `hkpConstraintDrawer::updateCommonParameters` (`00de47e0`, `this` = the drawer,
/// `(constraint, first block, second block)`), `drawPivots` (`00de44c0`) and `drawBodyFrames`
/// (`00de4530`) (both `this` = the drawer, `(id, tag)`).
const UPDATE_COMMON_PARAMETERS: u32 = 0x00de_47e0;
const DRAW_PIVOTS: u32 = 0x00de_44c0;
const DRAW_BODY_FRAMES: u32 = 0x00de_4530;
/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = the result, `(transform, point)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;

// Translated from 00de8090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallSocketDrawer::drawConstraint` (named in the engine map): hands the display handler
/// to the primitive drawer (`this + 0xc0`), lets `updateCommonParameters` fill two 64-byte
/// stack blocks from the constraint, and sets the drawer's two points (`+0x50` from the
/// first block and the constraint data's `+0x30` vector, `+0x40` from the second block and the
/// `+0x20` vector; the constraint data is the pointer at `constraint + 0xc`, advanced by
/// `0x10`). Then the pivots and the body frames are drawn (`id`, `tag`) and the display
/// handler's slot `0x24` draws the line from `this + 0x40` to `this + 0x50` in the colour
/// `0xffff0000`.
pub fn hkp_ball_socket_drawer_draw_constraint(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    id: u32,
    tag: u32,
) {
    let base = this.addr();
    let data = e.mem.u32(constraint + 0xc).wrapping_add(0x10);
    e.call(SET_DISPLAY_HANDLER, &args![base + 0xc0, display_handler]);
    e.with_stack(0x80, |e, blocks| {
        let first = blocks.addr();
        let second = first + 0x40;
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![base, constraint, first, second],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![base + 0x50, first, data.wrapping_add(0x20)],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![base + 0x40, second, data.wrapping_add(0x10)],
        );
    });
    e.call(DRAW_PIVOTS, &args![base, id, tag]);
    e.call(DRAW_BODY_FRAMES, &args![base, id, tag]);
    e.vcall(
        display_handler,
        0x24,
        &args![base + 0x40, base + 0x50, 0xffff_0000u32, id, tag],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00de_8090,
        hkp_ball_socket_drawer_draw_constraint(Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE_DOUBLE: u32 = 0x0900_0010;

    #[test]
    fn draw_constraint_updates_the_points_then_draws_the_pivots_frames_and_a_line() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        for address in [
            SET_DISPLAY_HANDLER,
            UPDATE_COMMON_PARAMETERS,
            SET_TRANSFORMED_POS,
            DRAW_PIVOTS,
            DRAW_BODY_FRAMES,
            LINE_DOUBLE,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x24, LINE_DOUBLE);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(handler, table);
        let drawer = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x80);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, data);
        e.call(
            0x00de_8090,
            &args![drawer, constraint, handler, 11u32, 22u32],
        );
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        let update = &log[1].1;
        let (first, second) = (update[2], update[3]);
        assert_eq!(second, first + 0x40);
        assert_eq!(
            log,
            vec![
                (SET_DISPLAY_HANDLER, vec![drawer + 0xc0, handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![drawer, constraint, first, second]
                ),
                (SET_TRANSFORMED_POS, vec![drawer + 0x50, first, data + 0x30]),
                (
                    SET_TRANSFORMED_POS,
                    vec![drawer + 0x40, second, data + 0x20]
                ),
                (DRAW_PIVOTS, vec![drawer, 11, 22]),
                (DRAW_BODY_FRAMES, vec![drawer, 11, 22]),
                (
                    LINE_DOUBLE,
                    vec![handler, drawer + 0x40, drawer + 0x50, 0xffff_0000, 11, 22]
                ),
            ]
        );
    }
}
