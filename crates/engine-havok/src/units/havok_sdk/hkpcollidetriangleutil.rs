//! `Havok SDK/hkpcollidetriangleutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vectors are SSE (`f32` lanes, three-component sums added as `z + (y + x)`, `RSQRTSS`
//! refined by one Newton step); the scalar solving runs on the x87 stack, approximated here in
//! `f64` with a rounding to `f32` wherever the code stores a `float`.
//!
//! The function is complete: it has no /GS cookie and no exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, zip_with, Vec4};
use crate::x87::rsqrt_estimate;

/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The closest point of a segment to a point (`00d230e0`, cdecl: the point, the segment's
/// start, its end, the vector the closest point is written to); answers a code: 4 when the
/// closest point is the start, 8 when it is the end.
const POINT_ON_SEGMENT: u32 = 0x00d2_30e0;
/// The signed bytes indexed by the three-bit mask of the barycentric lanes above 0: a
/// vertex index (0 to 2) or `region - 8` for an edge.
const REGIONS: u32 = 0x010c_b438;
/// The bytes of the edge table: edge `i` of a region runs from vertex `[010cb442 + i]` to
/// vertex `[010cb440 + i]`.
const SEGMENT_VERTICES: u32 = 0x010c_b440;
/// The doubles added to and subtracted from the barycentric values (`01016978`, `010cb448`)
/// and the float in the fourth lane (`01017d00`).
const BIAS_LOW: u32 = 0x0101_6978;
const BIAS_ONE: u32 = 0x010c_b448;
const FOURTH_LANE: u32 = 0x0101_7d00;
/// The zero vector (`01267e30`) the lanes are compared with and the sign mask (`010c44d0`).
const ZERO_VECTOR: u32 = 0x0126_7e30;
const SIGN_MASK: u32 = 0x010c_44d0;
/// The floats 0.0, 3.0 and 0.5 of the `RSQRTSS` refinement.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// The frame: the closest points of the two segments of a vertex region.
const FIRST_POINT: u32 = 0x20;
const SECOND_POINT: u32 = 0x30;
const FRAME_SIZE: u32 = 0x40;

fn sub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

/// The sum of the three lane products added as `z + (y + x)`.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    let p = [a[0] * b[0], a[1] * b[1], a[2] * b[2]];
    p[2] + (p[1] + p[0])
}

/// `zxy(b) * yzx(a) - yzx(b) * zxy(a)` (`SHUFPS` 0xD2 and 0xC9): `a x b` in lanes 0..2.
fn cross(a: Vec4, b: Vec4) -> Vec4 {
    let by_a = [b[2] * a[1], b[0] * a[2], b[1] * a[0], b[3] * a[3]];
    let by_b = [b[1] * a[2], b[2] * a[0], b[0] * a[1], b[3] * a[3]];
    zip_with(by_a, by_b, |x, y| x - y)
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let estimate = rsqrt_estimate(sum);
    if sum == e.mem.f32(ZERO) {
        0.0
    } else {
        (e.mem.f32(HALF) * estimate) * (e.mem.f32(THREE) - (sum * estimate) * estimate)
    }
}

/// The sign-extended table byte at `at`, as the 16-bit word the code stores.
fn word_of(e: &Engine, at: u32) -> u16 {
    e.mem.u8(at) as i8 as i16 as u16
}

/// The address of the vertex number `index` (a table byte) of the triangle.
fn vertex(e: &Engine, triangle: u32, at: u32) -> u32 {
    triangle.wrapping_add(((e.mem.u8(at) as i8 as i32) << 4) as u32)
}

/// Writes a feature description: `count` vertex or edge indices.
fn set_feature(e: &mut Engine, feature: u32, count: u8, indices: [u16; 2]) {
    if feature == 0 {
        return;
    }
    e.mem.set_u8(feature, count);
    e.mem.set_u16(feature + 2, indices[0]);
    if count == 2 {
        e.mem.set_u16(feature + 4, indices[1]);
    }
}

/// The feature of the segment function's answer `code` in a vertex region: 4 is the
/// segment's first point, 8 its second, anything else the edge between them.
fn segment_feature(e: &mut Engine, feature: u32, code: u32, own: u16, other: u16) {
    match code {
        4 => set_feature(e, feature, 1, [own, 0]),
        8 => set_feature(e, feature, 1, [other, 0]),
        _ => set_feature(e, feature, 2, [own, other]),
    }
}

// Translated from 00cd7850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollideTriangleUtil::closestPointTriangle` (name from the engine map, unconfirmed;
/// cdecl; `RSQRTSS` is the host CPU's estimate): the closest point of a triangle (`triangle`:
/// three vertices of 16 bytes) to `point`, with the triangle's `cache` (floats `+0`, `+4`,
/// `+8` and a scale at `+0xC`), written as the unit direction from the closest point to the
/// point at `out` (16 bytes) and the distance at `out + 0x10`. `feature` (may be 0) is told
/// what the closest point lies on: the byte `+0` counts the 16-bit vertex indices at `+2` and
/// `+4` (1 vertex, 2 edge, 3 the face with the indices 0, 1, 2; 0 before anything is found).
///
/// The point's offset from vertex 1 is taken along the edges `v0 - v1` and `v2 - v1` and
/// turned into three barycentric lanes with the cache (x87, floats stored in between, the
/// biases the doubles `01016978` and `010cb448`); when all three lanes are above 0 the point
/// projects onto the face: the normal is the cross product of the edges times the scale,
/// turned around when the point lies on its side (then `out + 0x10` is the dot product,
/// else its negation) and the answer is 0. Otherwise the lanes above 0 pick a region from the
/// table at `010cb438`: an edge, whose closest point `00d230e0` finds, or a vertex, where the
/// two edges at it are tried and the one with the smaller squared distance wins (the second
/// when the squared distances are equal or unordered); the distance there comes from the CRT
/// square root (`00ec6040`). Answers 1 for the edge and vertex regions.
pub fn hkp_collide_triangle_util_closest_point_triangle(
    e: &mut Engine,
    point: Ptr,
    triangle: Ptr,
    cache: Ptr,
    out: Ptr,
    feature: Ptr,
) -> u32 {
    let (point, triangle, cache, out, feature) = (
        point.addr(),
        triangle.addr(),
        cache.addr(),
        out.addr(),
        feature.addr(),
    );
    if feature != 0 {
        e.mem.set_u8(feature, 0);
    }
    let to_first = sub(load_vec(e, triangle + 0x10), load_vec(e, point));
    let first_edge = sub(load_vec(e, triangle), load_vec(e, triangle + 0x10));
    let along_first = dot3(first_edge, to_first);
    let second_edge = sub(load_vec(e, triangle + 0x20), load_vec(e, triangle + 0x10));
    let along_second = dot3(second_edge, to_first);
    let cached = |e: &Engine, offset: u32| f64::from(x87_quiet(e.mem.f32(cache + offset)));
    let (first, second) = (f64::from(along_first), f64::from(along_second));
    let x = (cached(e, 8) * second - cached(e, 4) * first) as f32;
    let y = (first * cached(e, 8) - second * cached(e, 0)) as f32;
    let bias = e.mem.f64(BIAS_LOW);
    let one = e.mem.f64(BIAS_ONE);
    let lanes: Vec4 = [
        (f64::from(x) + bias) as f32,
        ((one - f64::from(x)) - f64::from(y)) as f32,
        (f64::from(y) + bias) as f32,
        x87_quiet(e.mem.f32(FOURTH_LANE)),
    ];
    let mask: u32 = (0..4)
        .filter(|&lane| e.mem.f32(ZERO_VECTOR + 4 * lane as u32) < lanes[lane])
        .map(|lane| 1u32 << lane)
        .sum::<u32>()
        & 7;
    if mask == 7 {
        let normal = cross(first_edge, second_edge);
        store_vec(e, out, normal);
        let scale = e.mem.f32(cache + 0xc);
        let scaled = normal.map(|lane| scale * lane);
        let side = x87_quiet(dot3(to_first, scaled));
        e.mem.set_f32(out + 0x10, side);
        store_vec(e, out, scaled);
        if 0.0 < side {
            let mask = load_vec(e, SIGN_MASK);
            let turned = zip_with(scaled, mask, |lane, bits| {
                f32::from_bits(lane.to_bits() ^ bits.to_bits())
            });
            store_vec(e, out, turned);
        } else {
            e.mem.set_f32(out + 0x10, -side);
        }
        if feature != 0 {
            e.mem.set_u8(feature, 3);
            for (n, index) in [0u16, 1, 2].into_iter().enumerate() {
                e.mem.set_u16(feature + 2 + 2 * n as u32, index);
            }
        }
        return 0;
    }
    let region = e.mem.u8(REGIONS + mask) as i8 as i32;
    if region < 0 {
        let edge = (region + 8) as u32;
        let end_vertex = vertex(e, triangle, SEGMENT_VERTICES + edge);
        let start_vertex = vertex(e, triangle, SEGMENT_VERTICES + 2 + edge);
        let near = word_of(e, SEGMENT_VERTICES + edge);
        let far = word_of(e, SEGMENT_VERTICES + 2 + edge);
        return e.with_stack(FRAME_SIZE, |e, frame| {
            let closest_at = frame.addr() + FIRST_POINT;
            let code = e
                .call(
                    POINT_ON_SEGMENT,
                    &args![point, start_vertex, end_vertex, closest_at],
                )
                .u32();
            segment_feature(e, feature, code, near, far);
            let difference = sub(load_vec(e, point), load_vec(e, closest_at));
            let squared = dot3(difference, difference);
            let factor = inverse_length(e, squared);
            store_vec(e, out, difference.map(|lane| lane * factor));
            e.mem.set_f32(out + 0x10, x87_quiet(squared * factor));
            1
        });
    }
    let own = region as u32;
    let start = triangle + (own << 4);
    let first_end = vertex(e, triangle, SEGMENT_VERTICES + 2 + own);
    let second_end = vertex(e, triangle, SEGMENT_VERTICES + own);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let first_code = e
            .call(
                POINT_ON_SEGMENT,
                &args![point, start, first_end, frame + FIRST_POINT],
            )
            .u32();
        let second_code = e
            .call(
                POINT_ON_SEGMENT,
                &args![point, second_end, start, frame + SECOND_POINT],
            )
            .u32();
        let from_first = sub(load_vec(e, point), load_vec(e, frame + FIRST_POINT));
        let from_second = sub(load_vec(e, point), load_vec(e, frame + SECOND_POINT));
        let first_squared = x87_quiet(dot3(from_first, from_first));
        let second_squared = x87_quiet(dot3(from_second, from_second));
        let first_wins = second_squared > first_squared;
        let (squared, difference) = if first_wins {
            let other = word_of(e, SEGMENT_VERTICES + 2 + own);
            segment_feature(e, feature, first_code, own as u16, other);
            (first_squared, from_first)
        } else {
            let other = word_of(e, SEGMENT_VERTICES + own);
            segment_feature(e, feature, second_code, own as u16, other);
            (second_squared, from_second)
        };
        let root = e.call(X87_SQRT, &args![f64::from(squared)]).f32();
        let inverse = (1.0 / f64::from(root)) as f32;
        e.mem
            .set_f32(out + 0x10, (f64::from(squared) * f64::from(inverse)) as f32);
        store_vec(e, out, difference.map(|lane| inverse * lane));
        1
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_7850,
        hkp_collide_triangle_util_closest_point_triangle(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, new_log, Log};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The tables, constants and the square root the code reads from the exe.
    fn exe_data(e: &mut Engine) {
        for (offset, byte) in [0xffu8, 0, 1, 0xfa, 2, 0xf9, 0xf8, 0xff]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u8(REGIONS + offset as u32, byte);
        }
        for (offset, byte) in [2u8, 0, 1, 2, 0, 0, 0, 0].into_iter().enumerate() {
            e.mem.set_u8(SEGMENT_VERTICES + offset as u32, byte);
        }
        e.mem.set_u64(BIAS_LOW, 0x3f50_624d_e000_0000);
        e.mem.set_u64(BIAS_ONE, 0x3ff0_0418_9378_0000);
        e.mem.set_u32(FOURTH_LANE, 0x3a83_126f);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.register(X87_SQRT, |_, a| {
            let mut i = 0;
            Ret {
                st0: f64::take(a, &mut i).sqrt(),
                ..Ret::default()
            }
        });
    }

    /// The segment function answering with the given closest points and codes in turn.
    type Replies = Vec<(Vec4, u32)>;

    struct Scene {
        e: Engine,
        log: Log,
        point: u32,
        triangle: u32,
        cache: u32,
        out: u32,
        feature: u32,
    }

    /// The triangle (0, 0, 0), (1, 0, 0), (0, 1, 0), the point, the cache floats, an output
    /// block filled with 7.0 and a feature block filled with 0xee.
    fn scene(point: [f32; 3], cache: [f32; 4], replies: Replies) -> Scene {
        let mut e = engine();
        exe_data(&mut e);
        let log = new_log();
        let seen = Rc::clone(&log);
        let replies = RefCell::new(replies.into_iter());
        e.register_double(POINT_ON_SEGMENT, move |e, a| {
            seen.borrow_mut().push((POINT_ON_SEGMENT, a.to_vec()));
            let (closest, code) = replies.borrow_mut().next().expect("a reply per call");
            store_vec(e, a[3], closest);
            Ret {
                eax: code,
                ..Ret::default()
            }
        });
        let triangle = e.mem.alloc(0x30);
        for (n, vertex) in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
            .into_iter()
            .enumerate()
        {
            store_vec(
                &mut e,
                triangle + 0x10 * n as u32,
                [vertex[0], vertex[1], vertex[2], 0.0],
            );
        }
        let point_at = e.mem.alloc(0x10);
        store_vec(&mut e, point_at, [point[0], point[1], point[2], 0.0]);
        let cache_at = e.mem.alloc(0x20);
        for (n, value) in cache.into_iter().enumerate() {
            e.mem.set_f32(cache_at + 4 * n as u32, value);
        }
        let out = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_f32(out + 4 * word, 7.0);
        }
        let feature = e.mem.alloc(0x10);
        for byte in 0..8 {
            e.mem.set_u8(feature + byte, 0xee);
        }
        Scene {
            e,
            log,
            point: point_at,
            triangle,
            cache: cache_at,
            out,
            feature,
        }
    }

    impl Scene {
        fn run(&mut self, feature: u32) -> u32 {
            self.e
                .call(
                    0x00cd_7850,
                    &[self.point, self.triangle, self.cache, self.out, feature],
                )
                .u32()
        }

        fn out(&self) -> [f32; 5] {
            let word = |n: u32| self.e.mem.f32(self.out + 4 * n);
            [word(0), word(1), word(2), word(3), word(4)]
        }

        /// The feature block: the count byte and the three words after it.
        fn feature(&self) -> [u32; 4] {
            [
                u32::from(self.e.mem.u8(self.feature)),
                u32::from(self.e.mem.u16(self.feature + 2)),
                u32::from(self.e.mem.u16(self.feature + 4)),
                u32::from(self.e.mem.u16(self.feature + 6)),
            ]
        }
    }

    fn close(actual: &[f32], expected: &[f32]) {
        for (a, x) in actual.iter().zip(expected) {
            assert!((a - x).abs() < 1e-5, "{actual:?} vs {expected:?}");
        }
    }

    #[test]
    fn a_point_over_the_face_gets_the_scaled_normal_turned_towards_it_00cd7850() {
        // Barycentric lanes (0.3, 0.4, 0.3) from the cache (0.3, 0.4, 0, 5).
        let cache = [0.3, 0.4, 0.0, 5.0];
        let mut s = scene([0.25, 0.25, 1.0], cache, Vec::new());
        let feature = s.feature;
        assert_eq!(s.run(feature), 0);
        // The normal (0, 0, -1) times 5 is on the far side of the point: it is turned.
        close(&s.out(), &[0.0, 0.0, 5.0, 0.0, 5.0]);
        assert_eq!(s.feature(), [3, 0, 1, 2]);
        assert!(calls(&s.log, POINT_ON_SEGMENT).is_empty());

        let mut s = scene([0.25, 0.25, -1.0], cache, Vec::new());
        assert_eq!(s.run(0), 0);
        close(&s.out(), &[0.0, 0.0, -5.0, 0.0, 5.0]);
        // No feature block: nothing is written to it.
        assert_eq!(s.feature(), [0xee, 0xeeee, 0xeeee, 0xeeee]);
    }

    #[test]
    fn a_point_beside_an_edge_asks_for_the_closest_point_on_it_00cd7850() {
        // Lanes (0.301, 1.201, -0.499): the region of the edge between vertices 0 and 1.
        let cache = [-0.5, 0.4, 0.0, 5.0];
        // (the code of the segment function, the feature it leads to)
        let cases = [
            (0, [2, 1, 0, 0xeeee]),
            (4, [1, 1, 0xeeee, 0xeeee]),
            (8, [1, 0, 0xeeee, 0xeeee]),
        ];
        for (code, expected) in cases {
            let mut s = scene(
                [0.25, 0.25, 1.0],
                cache,
                vec![([0.25, 0.0, 0.0, 0.0], code)],
            );
            let feature = s.feature;
            assert_eq!(s.run(feature), 1);
            let call = &calls(&s.log, POINT_ON_SEGMENT)[0];
            assert_eq!(call[..3], [s.point, s.triangle, s.triangle + 0x10]);
            assert_eq!(s.feature()[..], expected[..], "code {code}");
            // From the closest point (0.25, 0, 0) to the point: (0, 0.25, 1).
            let length = 1.0625f32.sqrt();
            close(&s.out(), &[0.0, 0.25 / length, 1.0 / length, 0.0, length]);
        }
        // Without a feature block nothing is written.
        let mut s = scene([0.25, 0.25, 1.0], cache, vec![([0.25, 0.0, 0.0, 0.0], 0)]);
        assert_eq!(s.run(0), 1);
        assert_eq!(s.feature()[0], 0xee);
    }

    #[test]
    fn a_vertex_region_takes_the_nearer_of_its_two_edges_00cd7850() {
        // Lanes (-0.749, 2.251, -0.499): the region of vertex 1, whose edges run to
        // vertex 2 (asked first) and from vertex 0 (asked second).
        let cache = [-0.5, -1.0, 0.0, 5.0];
        let near: Vec4 = [0.5, 0.0, 0.0, 0.0];
        let far: Vec4 = [1.0, 0.5, 0.0, 0.0];
        // (first reply, second reply, feature). The second edge wins when it is nearer or as
        // near; its code 4 is vertex 1, 8 vertex 0, else it is the edge (1, 0); for the first
        // edge code 8 is vertex 2 and else it is the edge (1, 2).
        let cases: [((Vec4, u32), (Vec4, u32), [u32; 3]); 7] = [
            ((far, 0), (near, 4), [1, 1, 0xeeee]),
            ((far, 0), (near, 8), [1, 0, 0xeeee]),
            ((far, 0), (near, 0), [2, 1, 0]),
            ((near, 4), (far, 0), [1, 1, 0xeeee]),
            ((near, 8), (far, 0), [1, 2, 0xeeee]),
            ((near, 0), (far, 0), [2, 1, 2]),
            ((near, 0), (near, 8), [1, 0, 0xeeee]),
        ];
        for (n, (first, second, expected)) in cases.into_iter().enumerate() {
            let mut s = scene([0.25, 0.25, 1.0], cache, vec![first, second]);
            let feature = s.feature;
            assert_eq!(s.run(feature), 1, "case {n}");
            let segments = calls(&s.log, POINT_ON_SEGMENT);
            let (v0, v1, v2) = (s.triangle, s.triangle + 0x10, s.triangle + 0x20);
            assert_eq!(segments[0][..3], [s.point, v1, v2]);
            assert_eq!(segments[1][..3], [s.point, v0, v1]);
            assert_eq!(s.feature()[..3], expected[..], "case {n}");
            // From the nearer closest point (0.5, 0, 0) to the point: (-0.25, 0.25, 1).
            let length = 1.125f32.sqrt();
            close(
                &s.out(),
                &[-0.25 / length, 0.25 / length, 1.0 / length, 0.0, length],
            );
        }
    }
}
