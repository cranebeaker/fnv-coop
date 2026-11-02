//! `NiCollision/run_00eb8dc0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// A value stored as a float and loaded back.
fn stored(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00eb8dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiOBBNode`, seven words: the centroids, the
/// first and last positions, the triangle numbers, the out left end, the out right start and the
/// out array; `00eb9fa0` calls it): splits the triangles `indices[first..=last]` by the side of
/// the box's plane their centroid lies on. For each axis in turn (the rows at +0x10, +0x1c,
/// +0x28, while a side is still empty): `*left_end = first - 1`, `*right_start = last + 1`, and
/// each position `i` (four at a time while at least four remain, then one by one) puts its
/// triangle `t` at `out[++*left_end]` when `(centroids[t] - center) . axis` (center +4, each
/// value stored) is above 0, at `out[--*right_start]` when below 0, and when 0 or unordered on
/// the left for `i <= (first + last) / 2`, else on the right. When all three axes leave a side
/// empty, the split is the middle (`*left_end = mid`, `*right_start = mid + 1`) and the triangle
/// numbers are copied to `out` unchanged.
#[allow(clippy::too_many_arguments)]
pub fn fn_00eb8dc0(
    e: &mut Engine,
    this: Ptr,
    centroids: Ptr,
    first: u32,
    last: u32,
    indices: Ptr,
    left_end: Ptr,
    right_start: Ptr,
    out: Ptr,
) {
    let (first_i, last_i) = (first as i32, last as i32);
    let mid = first_i.wrapping_add(last_i) / 2;
    let (l, r) = (left_end.addr(), right_start.addr());
    e.mem.set_u32(l, first.wrapping_sub(1));
    e.mem.set_u32(r, last.wrapping_add(1));
    let n = this.addr();
    let center = n.wrapping_add(4);
    let place = |e: &mut Engine, row: u32, position: i32| {
        let t = e.mem.u32(
            indices
                .addr()
                .wrapping_add((position as u32).wrapping_mul(4)),
        );
        let c = centroids.addr().wrapping_add(t.wrapping_mul(12));
        let [dx, dy, dz] = [0u32, 4, 8].map(|i| {
            stored(x87_sub(
                load_float(e, c.wrapping_add(i)),
                float_operand(e, center + i),
            ))
        });
        let s = x87_add(
            x87_mul(load_float(e, row + 4), dy),
            x87_mul(load_float(e, row), dx),
        );
        let dot = stored(x87_add(s, x87_mul(dz, float_operand(e, row + 8))));
        let left = if dot > 0.0 {
            true
        } else if dot < 0.0 {
            false
        } else {
            position <= mid
        };
        let slot = if left {
            let end = e.mem.u32(l).wrapping_add(1);
            e.mem.set_u32(l, end);
            end
        } else {
            let start = e.mem.u32(r).wrapping_sub(1);
            e.mem.set_u32(r, start);
            start
        };
        e.mem
            .set_u32(out.addr().wrapping_add(slot.wrapping_mul(4)), t);
    };
    let mut axis = 0u32;
    loop {
        if !((e.mem.u32(l) as i32) < first_i || (e.mem.u32(r) as i32) > last_i) {
            break;
        }
        e.mem.set_u32(l, first.wrapping_sub(1));
        e.mem.set_u32(r, last.wrapping_add(1));
        let row = n.wrapping_add(0x10 + 0xc * axis);
        let mut i = first_i;
        if last_i.wrapping_sub(first_i).wrapping_add(1) >= 4 {
            loop {
                for k in 0..4 {
                    place(e, row, i.wrapping_add(k));
                }
                i = i.wrapping_add(4);
                if i > last_i.wrapping_sub(3) {
                    break;
                }
            }
        }
        while i <= last_i {
            place(e, row, i);
            i = i.wrapping_add(1);
        }
        axis += 1;
        if axis >= 3 {
            break;
        }
    }
    if axis == 3 && ((e.mem.u32(l) as i32) < first_i || (e.mem.u32(r) as i32) > last_i) {
        e.mem.set_u32(l, mid as u32);
        e.mem.set_u32(r, mid.wrapping_add(1) as u32);
        if first_i <= last_i {
            let count = last.wrapping_sub(first).wrapping_add(1);
            for k in 0..count {
                let at = first.wrapping_add(k).wrapping_mul(4);
                let t = e.mem.u32(indices.addr().wrapping_add(at));
                e.mem.set_u32(out.addr().wrapping_add(at), t);
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_8dc0,
        fn_00eb8dc0(Ptr, Ptr, u32, u32, Ptr, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A node with identity axes and its center at the origin.
    fn node(e: &mut Engine) -> u32 {
        let n = e.mem.alloc(0x8c);
        for i in [0u32, 4, 8] {
            e.mem.set_f32(n + 0x10 + 4 * i, 1.0);
        }
        n
    }

    fn centroids(e: &mut Engine, points: &[[f32; 3]]) -> u32 {
        let at = e.mem.alloc(12 * points.len() as u32);
        for (i, p) in (0u32..).zip(points) {
            for (k, v) in (0u32..).zip(p) {
                e.mem.set_f32(at + 12 * i + 4 * k, *v);
            }
        }
        at
    }

    fn words(e: &mut Engine, values: &[u32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_u32(at + 4 * i, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    #[test]
    fn splits_by_the_first_axis_with_ties_by_position() {
        let mut e = Engine::new();
        let n = node(&mut e);
        let xs = [-1.0f32, 2.0, 0.0, -3.0, 5.0, 0.0];
        let points: Vec<[f32; 3]> = xs.iter().map(|x| [*x, 7.0, 7.0]).collect();
        let c = centroids(&mut e, &points);
        let indices = words(&mut e, &[0, 1, 2, 3, 4, 5]);
        let out = e.mem.alloc(24);
        let ends = e.mem.alloc(8);
        e.call(
            0x00eb_8dc0,
            &args![n, c, 0u32, 5u32, indices, ends, ends + 4, out],
        );
        assert_eq!(read(&e, ends, 2), vec![2, 3]);
        assert_eq!(read(&e, out, 6), vec![1, 2, 4, 5, 3, 0]);
    }

    #[test]
    fn falls_back_to_the_next_axis_then_to_the_middle() {
        let mut e = Engine::new();
        let n = node(&mut e);
        // All in front on x; y splits them.
        let c = centroids(
            &mut e,
            &[[1.0, 1.0, 0.0], [1.0, -1.0, 0.0], [1.0, 1.0, 0.0]],
        );
        let indices = words(&mut e, &[0, 1, 2]);
        let out = e.mem.alloc(12);
        let ends = e.mem.alloc(8);
        e.call(
            0x00eb_8dc0,
            &args![n, c, 0u32, 2u32, indices, ends, ends + 4, out],
        );
        assert_eq!(read(&e, ends, 2), vec![1, 2]);
        assert_eq!(read(&e, out, 3), vec![0, 2, 1]);
        // All on one side on every axis: the middle, numbers copied.
        let mut points = vec![[1.0f32, 1.0, 1.0]; 13];
        points[0] = [0.0; 3];
        let c = centroids(&mut e, &points);
        let indices = words(&mut e, &[0, 10, 11, 12]);
        let out = e.mem.alloc(16);
        e.call(
            0x00eb_8dc0,
            &args![n, c, 1u32, 3u32, indices, ends, ends + 4, out],
        );
        assert_eq!(read(&e, ends, 2), vec![2, 3]);
        assert_eq!(read(&e, out + 4, 3), vec![10, 11, 12]);
    }
}
