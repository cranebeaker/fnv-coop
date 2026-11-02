//! `fallout shared/pathfinding/pathingavoidnode.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The cost an avoid node (a point or a segment with a radius and a cost)
//! adds to a point, and the sum over an array of them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `FLT_MAX` (`float`, `0x7f7fffff`).
const FLOAT_MAX: u32 = 0x0106_ccd0;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `00439ef0`: `this - other` into `out` (`thiscall` on a point: out,
/// other).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `004a7290`: a point's squared length (`thiscall`, ST0).
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `NiPoint3::Dot` (engine map), `thiscall`, one word, ST0.
const POINT_DOT: u32 = 0x004b_6190;
/// `0044ddc0`: the array's count (`thiscall`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `006a1400`: the array's element `index` (`thiscall`, one word).
const ARRAY_AT: u32 = 0x006a_1400;

// Translated from 006dc780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the avoid node's cost at `point`: with the radius
/// (`+0x18`) squared as `r2` and the node's squared distance `d` to the
/// point (type `+0x20` 0 or 2: from the node's position at `+0`; type 1:
/// from the segment `+0` to `+0xc`, `FLT_MAX` when the point projects
/// before its start or past its end; other types: `FLT_MAX`), the cost is
/// `(1 - d / r2) * cost` (`+0x1c`) when `d < r2`, else 0.
pub fn fn_006dc780(e: &mut Engine, this: Ptr, point: Ptr) -> f32 {
    let node = this.addr();
    let radius_squared = x87_store32(x87_mul(
        x87_load32(e.mem.u32(node + 0x18)),
        x87_m32(e.mem.u32(node + 0x18)),
    ));
    let mut distance = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
    let kind = e.mem.u32(node + 0x20);
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let offset = frame + 0x2c;
        let segment = frame + 0xc;
        let from_start = frame + 0x18;
        match kind {
            0 | 2 => {
                e.call(POINT_SUBTRACT, &args![node, offset, point]);
                let length = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
                distance = x87_store32(length);
            }
            1 => {
                e.call(POINT_SUBTRACT, &args![node + 0xc, segment, node]);
                e.call(POINT_SUBTRACT, &args![point, from_start, node]);
                let length = e.call(POINT_LENGTH_SQUARED, &args![segment]).f64();
                let segment_squared = x87_store32(length);
                let dot = e.call(POINT_DOT, &args![from_start, segment]).f64();
                let along = x87_store32(dot);
                let zero: f64 = e.global(ZERO);
                // `TEST AH,5; JNP`: below 0 goes to `FLT_MAX`; then
                // `TEST AH,5; JP`: past the end (not NaN) does too.
                let before = x87_load32(along) < zero;
                let past = x87_load32(segment_squared) < x87_load32(along);
                if before || past {
                    distance = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
                } else {
                    let total = e.call(POINT_LENGTH_SQUARED, &args![from_start]).f64();
                    let along_squared = x87_mul(x87_load32(along), x87_m32(along));
                    let projected = x87_div(along_squared, x87_m32(segment_squared));
                    distance = x87_store32(x87_sub(total, projected));
                }
            }
            _ => {}
        }
    });
    // `TEST AH,0x41; JNZ`: 0 unless `r2 > d` (ordered).
    if x87_load32(radius_squared) > x87_load32(distance) {
        let ratio = x87_div(x87_load32(distance), x87_m32(radius_squared));
        let scaled = x87_mul(x87_sub(1.0, ratio), x87_m32(e.mem.u32(node + 0x1c)));
        f32::from_bits(x87_store32(scaled))
    } else {
        0.0
    }
}

// Translated from 006dc990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingAvoidNodeArray::GetCostForPoint` (engine map): the sum of every
/// node's cost at `point` ([`fn_006dc780`]), kept as a `float`.
pub fn pathing_avoid_node_array_get_cost_for_point(e: &mut Engine, this: Ptr, point: Ptr) -> f32 {
    let mut sum = 0u32;
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    let mut index = 0u32;
    while index < count {
        let node = e.call(ARRAY_AT, &args![this, index]).ptr();
        let cost = fn_006dc780(e, node, point);
        sum = x87_store32(x87_add(f64::from(cost), x87_m32(sum)));
        index = index.wrapping_add(1);
    }
    f32::from_bits(sum)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006d_c780, fn_006dc780(Ptr, Ptr) -> f32),
        entry!(0x006d_c990, pathing_avoid_node_array_get_cost_for_point(Ptr, Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    fn read(e: &Engine, p: u32) -> [f64; 3] {
        [0, 4, 8].map(|o| f64::from(e.mem.f32(p + o)))
    }

    /// Vector doubles over plain floats.
    fn vectors(e: &mut Engine) {
        e.map(0x0106_c000, 0x1000);
        e.mem.set_u32(FLOAT_MAX, 0x7f7f_ffff);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[2]));
            for i in 0..3u32 {
                e.mem
                    .set_f32(a[1] + 4 * i, (p[i as usize] - q[i as usize]) as f32);
            }
            Ret::default()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let p = read(e, a[0]);
            Ret {
                st0: p.iter().map(|v| v * v).sum(),
                ..Ret::default()
            }
        });
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[1]));
            Ret {
                st0: p.iter().zip(q.iter()).map(|(a, b)| a * b).sum(),
                ..Ret::default()
            }
        });
    }

    fn node(e: &mut Engine, kind: u32, radius: f32, cost: f32) -> u32 {
        let n = e.mem.alloc(0x24);
        e.mem.set_f32(n + 0xc, 10.0);
        e.mem.set_f32(n + 0x18, radius);
        e.mem.set_f32(n + 0x1c, cost);
        e.mem.set_u32(n + 0x20, kind);
        n
    }

    #[test]
    fn point_node_cost_falls_off_with_distance() {
        let mut e = Engine::new();
        vectors(&mut e);
        let n = node(&mut e, 0, 4.0, 8.0);
        let p = point(&mut e, 2.0, 0.0, 0.0);
        // d = 4, r2 = 16: (1 - 0.25) * 8.
        assert_eq!(e.call(0x006d_c780, &args![n, p]).f32(), 6.0);
        let far = point(&mut e, 5.0, 0.0, 0.0);
        assert_eq!(e.call(0x006d_c780, &args![n, far]).f32(), 0.0);
        let edge = point(&mut e, 4.0, 0.0, 0.0);
        assert_eq!(e.call(0x006d_c780, &args![n, edge]).f32(), 0.0);
        // Another type: FLT_MAX, no cost.
        let other = node(&mut e, 3, 4.0, 8.0);
        assert_eq!(e.call(0x006d_c780, &args![other, p]).f32(), 0.0);
    }

    #[test]
    fn segment_node_measures_from_the_line() {
        let mut e = Engine::new();
        vectors(&mut e);
        // Segment from the origin to (10, 0, 0).
        let n = node(&mut e, 1, 4.0, 8.0);
        let beside = point(&mut e, 5.0, 2.0, 0.0);
        assert_eq!(e.call(0x006d_c780, &args![n, beside]).f32(), 6.0);
        let before = point(&mut e, -1.0, 0.0, 0.0);
        assert_eq!(e.call(0x006d_c780, &args![n, before]).f32(), 0.0);
        let past = point(&mut e, 11.0, 0.0, 0.0);
        assert_eq!(e.call(0x006d_c780, &args![n, past]).f32(), 0.0);
    }

    #[test]
    fn array_cost_sums_its_nodes() {
        let mut e = Engine::new();
        vectors(&mut e);
        let nodes = [node(&mut e, 0, 4.0, 8.0), node(&mut e, 2, 4.0, 4.0)];
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, nodes[0]);
        e.mem.set_u32(array + 4, nodes[1]);
        e.register(ARRAY_COUNT, |_, _| 2u32.into_ret());
        e.register(ARRAY_AT, |e, a| e.mem.u32(a[0] + 4 * a[1]).into_ret());
        let p = point(&mut e, 2.0, 0.0, 0.0);
        assert_eq!(e.call(0x006d_c990, &args![array, p]).f32(), 6.0 + 3.0);
        e.register(ARRAY_COUNT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x006d_c990, &args![array, p]).f32(), 0.0);
    }
}
