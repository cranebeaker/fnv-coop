//! `NiCollision/run_00eb3230` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// Unnamed in the engine map (`00eaff20`, cdecl: a segment, a block of a segment and its
/// displacement, three out floats): their squared distance, in ST0.
const SQR_DISTANCE_SEGMENT_MOVING_SEGMENT: u32 = 0x00ea_ff20;
/// `NiSqrDistance::Compute_ov3` (`00eb1d20`, Xbox PDB; cdecl: a segment, a triangle, three out
/// floats): their squared distance, in ST0.
const SQR_DISTANCE_SEGMENT_TRIANGLE: u32 = 0x00eb_1d20;

fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// `FLD dword [from]; FSTP dword [to]`.
fn move_float(e: &mut Engine, from: u32, to: u32) {
    let word = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, word);
}

// Translated from 00eb3230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: a triangle (origin, two edges), a block of a segment
/// (origin, direction) and its displacement (+0x18), four out floats: the triangle's two
/// parameters, the segment's parameter and the time): the squared distance between the
/// triangle and the segment swept along its displacement, the least of six (each stored as a
/// float): the triangle's three edges against the moving segment (`00eaff20`), then the segment
/// at its start, at its end (origin plus displacement) and the path of its far end (origin plus
/// direction, moving by the displacement) against the triangle (`00eb1d20`). The first fills the
/// outs (the second triangle parameter 0); each later one replaces them only when strictly
/// nearer: edge 1 gives `(0, s)`, edge 2 `(1 - s, s)`, the start time 0, the end time 1, and the
/// far end the segment parameter 1 with the time the word the end case left (1.0). Answers the
/// least distance.
pub fn fn_00eb3230(
    e: &mut Engine,
    triangle: Ptr,
    moving: Ptr,
    out_first: Ptr,
    out_second: Ptr,
    out_segment: Ptr,
    out_time: Ptr,
) -> f64 {
    let (tri, seg) = (triangle.addr(), moving.addr());
    let (s, t, p, time) = (
        out_first.addr(),
        out_second.addr(),
        out_segment.addr(),
        out_time.addr(),
    );
    // The exe's frame: offsets from ESP after its four register pushes.
    e.with_stack(0x48, |e, frame| {
        let f = frame.addr();
        let block = f + 0x2c;
        let (a, b, c, d) = (f + 0x10, f + 0x14, f + 0x18, f + 0x1c);
        // The first edge: the moving segment's outs go straight to the caller's.
        copy_words(e, tri, block, 6);
        let distance = e
            .call(
                SQR_DISTANCE_SEGMENT_MOVING_SEGMENT,
                &args![block, seg, s, p, time],
            )
            .f64();
        let mut best = x87_store32(distance);
        e.mem.set_u32(t, 0);
        // The second edge.
        copy_words(e, tri.wrapping_add(0x18), block + 0xc, 3);
        let distance = e
            .call(
                SQR_DISTANCE_SEGMENT_MOVING_SEGMENT,
                &args![block, seg, a, d, c],
            )
            .f64();
        let distance = x87_store32(distance);
        e.mem.set_u32(b, 0);
        if x87_load32(best) > x87_load32(distance) {
            best = distance;
            e.mem.set_u32(s, 0);
            move_float(e, a, t);
            move_float(e, d, p);
            move_float(e, c, time);
        }
        // The third edge, from the end of the first to the end of the second.
        for i in [0u32, 4, 8] {
            let value = x87_add(
                load_float(e, tri.wrapping_add(0xc + i)),
                float_operand(e, block + i),
            );
            e.mem.set_u32(block + i, x87_store32(value));
        }
        for i in [0u32, 4, 8] {
            let value = x87_sub(
                load_float(e, block + 0xc + i),
                float_operand(e, tri.wrapping_add(0xc + i)),
            );
            e.mem.set_u32(block + 0xc + i, x87_store32(value));
        }
        let distance = e
            .call(
                SQR_DISTANCE_SEGMENT_MOVING_SEGMENT,
                &args![block, seg, a, d, c],
            )
            .f64();
        let distance = x87_store32(distance);
        let along = load_float(e, a);
        e.mem.set_u32(b, x87_store32(x87_sub(1.0, along)));
        if x87_load32(best) > x87_load32(distance) {
            best = distance;
            move_float(e, b, s);
            e.mem.set_u32(t, x87_store32(along));
            move_float(e, d, p);
            move_float(e, c, time);
        }
        // The segment at its start.
        copy_words(e, seg, block, 6);
        let distance = e
            .call(SQR_DISTANCE_SEGMENT_TRIANGLE, &args![block, tri, d, b, a])
            .f64();
        let distance = x87_store32(distance);
        e.mem.set_u32(c, 0);
        if x87_load32(best) > x87_load32(distance) {
            best = distance;
            move_float(e, b, s);
            move_float(e, a, t);
            move_float(e, d, p);
            e.mem.set_u32(time, 0);
        }
        // The segment at its end.
        for i in [0u32, 4, 8] {
            let value = x87_add(
                load_float(e, seg.wrapping_add(0x18 + i)),
                float_operand(e, seg.wrapping_add(i)),
            );
            e.mem.set_u32(block + i, x87_store32(value));
        }
        copy_words(e, seg.wrapping_add(0xc), block + 0xc, 3);
        let distance = e
            .call(SQR_DISTANCE_SEGMENT_TRIANGLE, &args![block, tri, d, b, a])
            .f64();
        let distance = x87_store32(distance);
        let one = x87_store32(1.0);
        e.mem.set_u32(c, one);
        if x87_load32(best) > x87_load32(distance) {
            best = distance;
            move_float(e, b, s);
            move_float(e, a, t);
            move_float(e, d, p);
            e.mem.set_u32(time, one);
        }
        // The path of the far end.
        let x = x87_add(load_float(e, seg), float_operand(e, seg.wrapping_add(0xc)));
        e.mem.set_u32(block, x87_store32(x));
        let y = x87_add(
            load_float(e, seg.wrapping_add(0x10)),
            float_operand(e, seg.wrapping_add(4)),
        );
        e.mem.set_u32(block + 4, x87_store32(y));
        let z = x87_add(
            load_float(e, seg.wrapping_add(0x14)),
            float_operand(e, seg.wrapping_add(8)),
        );
        e.mem.set_u32(block + 8, x87_store32(z));
        copy_words(e, seg.wrapping_add(0x18), block + 0xc, 3);
        let distance = e
            .call(SQR_DISTANCE_SEGMENT_TRIANGLE, &args![block, tri, d, b, a])
            .f64();
        let distance = x87_store32(distance);
        if x87_load32(best) > x87_load32(distance) {
            best = distance;
            move_float(e, b, s);
            move_float(e, a, t);
            e.mem.set_u32(p, one);
            move_float(e, c, time);
        }
        x87_load32(best)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_3230,
        fn_00eb3230(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// The doubles answer the distance in the segment block's origin x (as a test value) and
    /// write 0.25, 0.5, 0.75 to their three outs; the log keeps the blocks they were given.
    fn doubles(e: &mut Engine) {
        for callee in [
            SQR_DISTANCE_SEGMENT_MOVING_SEGMENT,
            SQR_DISTANCE_SEGMENT_TRIANGLE,
        ] {
            e.register(callee, |e, a| {
                for (k, v) in (2..5).zip([0.25f32, 0.5, 0.75]) {
                    e.mem.set_f32(a[k], v);
                }
                Ret {
                    st0: f64::from(e.mem.f32(a[0])),
                    ..Ret::default()
                }
            });
        }
    }

    fn setup(e: &mut Engine, origin_x: f32, edge0_x: f32, segment_x: f32) -> [u32; 6] {
        let tri = e.mem.alloc(0x24);
        e.mem.set_f32(tri, origin_x);
        e.mem.set_f32(tri + 0xc, edge0_x);
        e.mem.set_f32(tri + 0x18, 1.0);
        let seg = e.mem.alloc(0x24);
        e.mem.set_f32(seg, segment_x);
        e.mem.set_f32(seg + 0xc, 1.0);
        e.mem.set_f32(seg + 0x18, 2.0);
        let outs = e.mem.alloc(16);
        [tri, seg, outs, outs + 4, outs + 8, outs + 12]
    }

    #[test]
    fn keeps_the_first_edge_when_nothing_is_nearer() {
        let mut e = Engine::new();
        doubles(&mut e);
        // Distances: edges 1 (origin x), 1, 1 + 3 = 4; segment start 5, end 7, far end 6.
        let [tri, seg, s, t, p, time] = setup(&mut e, 1.0, 3.0, 5.0);
        let d = e.call(0x00eb_3230, &args![tri, seg, s, t, p, time]).f64();
        assert_eq!(d, 1.0);
        assert_eq!(read(&e, s, 4), vec![0.25, 0.0, 0.5, 0.75]);
    }

    #[test]
    fn nearer_cases_replace_the_outs() {
        let mut e = Engine::new();
        doubles(&mut e);
        // Edge 3 starts at origin + edge0 = 5 - 6 = -1, the nearest: (1 - s, s).
        let [tri, seg, s, t, p, time] = setup(&mut e, 5.0, -6.0, 9.0);
        let d = e.call(0x00eb_3230, &args![tri, seg, s, t, p, time]).f64();
        assert_eq!(d, -1.0);
        assert_eq!(read(&e, s, 4), vec![0.75, 0.25, 0.5, 0.75]);
        // The segment's far end path (origin + direction = -4 + 1) is the nearest after its
        // start (-4): segment parameter 1, time 1.
        let [tri, seg, s, t, p, time] = setup(&mut e, 5.0, 1.0, -4.0);
        e.mem.set_f32(seg + 0xc, -1.0);
        let d = e.call(0x00eb_3230, &args![tri, seg, s, t, p, time]).f64();
        assert_eq!(d, -5.0);
        assert_eq!(read(&e, s, 4), vec![0.5, 0.75, 1.0, 1.0]);
    }
}
