//! `fallout shared/pathfinding/run_006bab00` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Circle geometry for the path smoother: a segment against a circle (a
//! hit test and the crossing points), the tangent lines between two
//! circles, a random point in a triangle, and two segment helpers. Each
//! x87 operation is done in the code's order and operand order, `float`
//! stores rounded as the code stores them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `0.0` and `1.0` (`double`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// `0.001` (`float`): the tolerance of [`fn_006bac20`].
const EPSILON: u32 = 0x0101_7d00;
/// `00439ef0`: `this - other` into `out` (`thiscall`: out, other); returns
/// `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00439e90`: `this + other` into `out` (`thiscall`: out, other); returns
/// `out`.
const POINT_ADD: u32 = 0x0043_9e90;
/// `004a3760`: `vector * scale` into `out` (`cdecl`: out, scale, vector).
const POINT_SCALE: u32 = 0x004a_3760;
/// `0045bb20`: `this * scale` into `out` (`thiscall`: out, scale).
const POINT_SCALED: u32 = 0x0045_bb20;
/// `004a7290`: a point's squared length (`thiscall`, ST0).
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `NiPoint3::Dot` (engine map), `thiscall`, one word, ST0.
const POINT_DOT: u32 = 0x004b_6190;
/// `006815c0`: returns `this` (a point's trivial constructor).
const SELF: u32 = 0x0068_15c0;
/// `0063c8a0`: `this -= other` (`thiscall`, one word).
const POINT_SUBTRACT_IN_PLACE: u32 = 0x0063_c8a0;
/// `004941c0`: `this /= value` (`thiscall`, one float).
const POINT_DIVIDE: u32 = 0x0049_41c0;
/// `00595c80`: a 2D point's squared length (`thiscall`, ST0).
const POINT2_LENGTH_SQUARED: u32 = 0x0059_5c80;
/// `006985f0`: copies a 2D point (`thiscall`, one word: the source).
const POINT2_COPY: u32 = 0x0069_85f0;
/// `004019b0`: the square root of a float (`cdecl`, ST0).
const SQUARE_ROOT: u32 = 0x0040_19b0;
/// `00408840`: the absolute value of a float (`cdecl`, ST0).
const ABSOLUTE: u32 = 0x0040_8840;
/// `005c5420`: a random float (no arguments, ST0).
const RANDOM_VALUE_005C5420: u32 = 0x005c_5420;
/// `006e7d10`: a segment from `segment` (`thiscall`, one word).
const SEGMENT_CONSTRUCT: u32 = 0x006e_7d10;
/// `004a0bd0`: the circle's centre into a point (`thiscall`, one word);
/// returns it.
const CIRCLE_CENTER: u32 = 0x004a_0bd0;

fn ld(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn m32(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

fn st(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_u32(address, x87_store32(value));
}

/// The pushed copy of a float (`FLD; FSTP [ESP]`).
fn pushed(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

// Translated from 006bab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: whether segment `a`-`b` comes closer than
/// `radius` to `center` (the same test as
/// `NavMeshMath::SegmentCircleIntersection` with the arguments in another
/// order): the closest point is `a` when the projection is not past `a`,
/// `b` when it is at or past `b`, else `a + (b - a) * t`.
pub fn fn_006bab00(e: &mut Engine, center: Ptr, radius: f32, a: Ptr, b: Ptr) -> bool {
    let (center, a, b) = (center.addr(), a.addr(), b.addr());
    e.with_stack(0x58, |e, frame| {
        let l = |x: u32| frame.addr() + 0x58 - x;
        let (segment, to_center, closest) = (l(0x24), l(0xc), l(0x18));
        e.call(POINT_SUBTRACT, &args![b, segment, a]);
        e.call(POINT_SUBTRACT, &args![center, to_center, a]);
        e.call(SELF, &args![closest]);
        let along = e.call(POINT_DOT, &args![segment, to_center]).f64();
        st(e, l(0x28), along);
        let zero: f64 = e.global(ZERO);
        if ld(e, l(0x28)) <= zero {
            for i in 0..3 {
                let word = e.mem.u32(a + 4 * i);
                e.mem.set_u32(closest + 4 * i, word);
            }
        } else {
            let length = e.call(POINT_DOT, &args![segment, segment]).f64();
            st(e, l(0x2c), length);
            if ld(e, l(0x28)) >= ld(e, l(0x2c)) {
                for i in 0..3 {
                    let word = e.mem.u32(b + 4 * i);
                    e.mem.set_u32(closest + 4 * i, word);
                }
            } else {
                st(e, l(0x54), x87_div(ld(e, l(0x28)), m32(e, l(0x2c))));
                let t = pushed(e, l(0x54));
                let scaled = e.call(POINT_SCALE, &args![l(0x38), t, segment]).u32();
                let point = e.call(POINT_ADD, &args![a, l(0x44), scaled]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(point + 4 * i);
                    e.mem.set_u32(closest + 4 * i, word);
                }
            }
        }
        let offset = e
            .call(POINT_SUBTRACT, &args![center, l(0x50), closest])
            .u32();
        let distance = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
        let limit = x87_mul(x87_load32(radius.to_bits()), x87_m32(radius.to_bits()));
        limit > distance
    })
}

/// [`fn_006bac20`]'s range test: `-eps <= t <= 1 + eps` (`TEST AH,1` and
/// `TEST AH,0x41`, NaN failing).
fn in_range(e: &Engine, t: u32, eps: u32) -> bool {
    let t = x87_load32(e.mem.u32(t));
    let eps = x87_load32(e.mem.u32(eps));
    !(t < -eps || t.is_nan()) && t <= x87_add(eps, 1.0)
}

/// [`fn_006bac20`]'s end flag: 2 near an end (`t < eps` or `t > 1 - eps`),
/// else 1.
fn end_kind(e: &Engine, t: u32, eps: u32) -> u32 {
    let t = x87_load32(e.mem.u32(t));
    let eps = x87_load32(e.mem.u32(eps));
    if eps > t || t > x87_sub(1.0, eps) {
        2
    } else {
        1
    }
}

// Translated from 006bac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: where `segment` crosses the unit circle of
/// `circle` (both put in the circle's frame: its centre subtracted
/// (`004a0bd0`, [`fn_006bb510`]), divided by its radius `+0xc`). False for
/// a radius not above 0. The line is solved along `x` when `y` does not
/// move (or `|dx| > |dy|`), else along `y`; `out` gets the kind (0 no
/// crossing, 1/2 one end, 3 a tangent, 4 two crossings, 5 the segment
/// inside), and per crossing (`out+4`, `out+0x14`) its end flag (1, or 2
/// near an end), the point and its parameter `t` along the segment.
pub fn fn_006bac20(e: &mut Engine, circle: Ptr, segment: Ptr, out: Ptr) -> bool {
    let circle = circle.addr();
    let out = out.addr();
    e.with_stack(0xf0, |e, frame| {
        let l = |x: u32| frame.addr() + 0xf0 - x;
        let eps = l(0x4);
        let one_point = l(0x8);
        let one_squared = l(0xc);
        let start = l(0x38);
        let (start_x, start_y) = (l(0x38), l(0x34));
        let (dir_x, dir_y) = (l(0x20), l(0x1c));
        let eps_value = x87_quiet_bits(e.global::<u32>(EPSILON));
        e.mem.set_u32(eps, eps_value);
        let zero: f64 = e.global(ZERO);
        let one: f64 = e.global(ONE);
        if ld(e, circle + 0xc) <= zero {
            return false;
        }
        let first = out + 4;
        let second = out + 0x14;
        e.call(SEGMENT_CONSTRUCT, &args![start, segment]);
        let center = e.call(CIRCLE_CENTER, &args![circle, l(0xdc)]).ptr();
        fn_006bb510(e, Ptr::new(start), center);
        e.mem.set_u32(one_point, 1.0f32.to_bits());
        let radius = pushed(e, circle + 0xc);
        e.call(POINT_DIVIDE, &args![start, radius]);
        let radius = pushed(e, circle + 0xc);
        e.call(POINT_DIVIDE, &args![l(0x2c), radius]);
        fn_006bb540(e, Ptr::new(start), Ptr::new(l(0x20)));
        st(e, one_squared, x87_mul(ld(e, one_point), m32(e, one_point)));

        let dx = ld(e, dir_x);
        let dy = ld(e, dir_y);
        let along_x = if dy == zero && dx != zero {
            true
        } else if dy != zero && dx == zero {
            false
        } else {
            let fx = e.call(ABSOLUTE, &args![pushed(e, dir_x)]).f64();
            let fy = e.call(ABSOLUTE, &args![pushed(e, dir_y)]).f64();
            fy < fx
        };

        let t_of = |e: &mut Engine, entry: u32, coordinate: u32, start: u32, inverse: u32| {
            let value = x87_mul(
                x87_sub(
                    x87_mul(ld(e, one_point), m32(e, entry + coordinate)),
                    m32(e, start),
                ),
                m32(e, inverse),
            );
            st(e, entry + 0xc, value);
        };

        if along_x {
            let (inverse, slope, intercept) = (l(0x54), l(0x50), l(0x4c));
            let (k, intercept2, disc) = (l(0x48), l(0x44), l(0x58));
            st(e, inverse, x87_div(1.0, ld(e, dir_x)));
            st(e, slope, x87_mul(ld(e, dir_y), m32(e, inverse)));
            st(
                e,
                intercept,
                x87_sub(m32(e, start_y), x87_mul(ld(e, slope), m32(e, start_x))),
            );
            st(e, l(0x40), x87_mul(ld(e, slope), m32(e, slope)));
            st(e, k, x87_add(ld(e, l(0x40)), one));
            st(e, intercept2, x87_mul(ld(e, intercept), m32(e, intercept)));
            let value = x87_mul(
                x87_sub(x87_mul(ld(e, one_squared), m32(e, k)), m32(e, intercept2)),
                m32(e, one_squared),
            );
            st(e, disc, value);
            if ld(e, disc) < -ld(e, eps) {
                e.mem.set_u32(out, 0);
                return false;
            }
            if ld(e, eps) > ld(e, disc) {
                e.mem.set_u32(out, 3);
                st(
                    e,
                    l(0x5c),
                    x87_div(1.0, x87_mul(ld(e, one_squared), m32(e, k))),
                );
                st(e, l(0x68), x87_mul(ld(e, intercept), m32(e, one_point)));
                e.call(SELF, &args![l(0x64)]);
                let x = x87_mul(x87_mul(-ld(e, l(0x68)), m32(e, slope)), m32(e, l(0x5c)));
                st(e, l(0x64), x);
                st(e, l(0x60), x87_mul(ld(e, l(0x68)), m32(e, l(0x5c))));
                e.call(POINT2_COPY, &args![first + 4, l(0x64)]);
                t_of(e, first, 4, start_x, inverse);
            } else {
                e.mem.set_u32(out, 4);
                st(
                    e,
                    l(0x6c),
                    x87_div(1.0, x87_mul(ld(e, one_squared), m32(e, k))),
                );
                let root = e.call(SQUARE_ROOT, &args![pushed(e, disc)]).f64();
                st(e, l(0x74), root);
                st(e, l(0x70), x87_mul(ld(e, intercept), m32(e, one_point)));
                let (scale, root, offset) = (l(0x6c), l(0x74), l(0x70));
                let (p1, p2) = (l(0x7c), l(0x84));
                e.call(SELF, &args![p1]);
                let v = x87_add(x87_mul(-ld(e, offset), m32(e, slope)), m32(e, root));
                st(e, p1, x87_mul(v, m32(e, scale)));
                let v = x87_add(x87_mul(ld(e, slope), m32(e, root)), m32(e, offset));
                st(e, p1 + 4, x87_mul(v, m32(e, scale)));
                e.call(SELF, &args![p2]);
                let v = x87_sub(x87_mul(-ld(e, offset), m32(e, slope)), m32(e, root));
                st(e, p2, x87_mul(v, m32(e, scale)));
                let v = x87_sub(m32(e, offset), x87_mul(ld(e, slope), m32(e, root)));
                st(e, p2 + 4, x87_mul(v, m32(e, scale)));
                let (a, b) = if ld(e, dir_x) < zero {
                    (p1, p2)
                } else {
                    (p2, p1)
                };
                e.call(POINT2_COPY, &args![first + 4, a]);
                e.call(POINT2_COPY, &args![second + 4, b]);
                t_of(e, first, 4, start_x, inverse);
                t_of(e, second, 4, start_x, inverse);
            }
        } else {
            let (inverse, slope, intercept) = (l(0xa0), l(0x9c), l(0x98));
            let (k, intercept2, disc) = (l(0x90), l(0x8c), l(0x94));
            st(e, inverse, x87_div(1.0, ld(e, dir_y)));
            st(e, slope, x87_mul(ld(e, dir_x), m32(e, inverse)));
            st(
                e,
                intercept,
                x87_sub(m32(e, start_x), x87_mul(ld(e, slope), m32(e, start_y))),
            );
            st(e, l(0x88), x87_mul(ld(e, slope), m32(e, slope)));
            st(e, k, x87_add(ld(e, l(0x88)), one));
            st(e, intercept2, x87_mul(ld(e, intercept), m32(e, intercept)));
            let value = x87_mul(
                x87_sub(x87_mul(ld(e, one_squared), m32(e, k)), m32(e, intercept2)),
                m32(e, one_squared),
            );
            st(e, disc, value);
            if ld(e, disc) < -ld(e, eps) {
                e.mem.set_u32(out, 0);
                return false;
            }
            if ld(e, eps) > ld(e, disc) {
                e.mem.set_u32(out, 3);
                st(
                    e,
                    l(0xa4),
                    x87_div(1.0, x87_mul(ld(e, one_squared), m32(e, k))),
                );
                st(e, l(0xb0), x87_mul(ld(e, intercept), m32(e, one_point)));
                e.call(SELF, &args![l(0xac)]);
                st(e, l(0xac), x87_mul(ld(e, l(0xb0)), m32(e, l(0xa4))));
                let y = x87_mul(x87_mul(-ld(e, l(0xb0)), m32(e, slope)), m32(e, l(0xa4)));
                st(e, l(0xa8), y);
                e.call(POINT2_COPY, &args![first + 4, l(0xac)]);
                t_of(e, first, 8, start_y, inverse);
            } else {
                e.mem.set_u32(out, 4);
                st(
                    e,
                    l(0xb4),
                    x87_div(1.0, x87_mul(ld(e, one_squared), m32(e, k))),
                );
                let root = e.call(SQUARE_ROOT, &args![pushed(e, disc)]).f64();
                st(e, l(0xbc), root);
                st(e, l(0xb8), x87_mul(ld(e, intercept), m32(e, one_point)));
                let (scale, root, offset) = (l(0xb4), l(0xbc), l(0xb8));
                let (p1, p2) = (l(0xc4), l(0xcc));
                e.call(SELF, &args![p1]);
                let v = x87_add(x87_mul(ld(e, slope), m32(e, root)), m32(e, offset));
                st(e, p1, x87_mul(v, m32(e, scale)));
                let v = x87_add(x87_mul(-ld(e, offset), m32(e, slope)), m32(e, root));
                st(e, p1 + 4, x87_mul(v, m32(e, scale)));
                e.call(SELF, &args![p2]);
                let v = x87_sub(m32(e, offset), x87_mul(ld(e, slope), m32(e, root)));
                st(e, p2, x87_mul(v, m32(e, scale)));
                let v = x87_sub(x87_mul(-ld(e, offset), m32(e, slope)), m32(e, root));
                st(e, p2 + 4, x87_mul(v, m32(e, scale)));
                let (a, b) = if ld(e, dir_y) < zero {
                    (p1, p2)
                } else {
                    (p2, p1)
                };
                e.call(POINT2_COPY, &args![first + 4, a]);
                e.call(POINT2_COPY, &args![second + 4, b]);
                t_of(e, first, 8, start_y, inverse);
                t_of(e, second, 8, start_y, inverse);
            }
        }

        if e.mem.u32(out) == 3 {
            if in_range(e, first + 0xc, eps) {
                let kind = end_kind(e, first + 0xc, eps);
                e.mem.set_u32(out + 4, kind);
                return true;
            }
            e.mem.set_u32(out, 0);
            return false;
        }
        let first_in = in_range(e, first + 0xc, eps);
        let second_in = in_range(e, second + 0xc, eps);
        if first_in && second_in {
            e.mem.set_u32(out, 4);
            let kind = end_kind(e, first + 0xc, eps);
            e.mem.set_u32(out + 4, kind);
            let kind = end_kind(e, second + 0xc, eps);
            e.mem.set_u32(out + 0x14, kind);
            return true;
        }
        if first_in {
            e.mem.set_u32(out, 2);
            let kind = end_kind(e, first + 0xc, eps);
            e.mem.set_u32(out + 4, kind);
            return true;
        }
        if second_in {
            e.mem.set_u32(out, 1);
            e.call(POINT2_COPY, &args![first + 4, second + 4]);
            let t = pushed(e, second + 0xc);
            e.mem.set_u32(first + 0xc, t);
            let kind = end_kind(e, first + 0xc, eps);
            e.mem.set_u32(out + 4, kind);
            return true;
        }
        let length = e.call(POINT2_LENGTH_SQUARED, &args![start]).f64();
        let kind = if ld(e, one_squared) > length { 5 } else { 0 };
        e.mem.set_u32(out, kind);
        false
    })
}

// Translated from 006bb510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: subtracts `offset` from both points of the segment
/// (`+0` and `+0xc`, `0063c8a0`).
pub fn fn_006bb510(e: &mut Engine, this: Ptr, offset: Ptr) {
    e.call(POINT_SUBTRACT_IN_PLACE, &args![this, offset]);
    e.call(POINT_SUBTRACT_IN_PLACE, &args![this.addr() + 0xc, offset]);
}

// Translated from 006bb540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the segment's direction (end `+0xc` minus start) into
/// `out`. Returns `out`.
pub fn fn_006bb540(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.call(POINT_SUBTRACT, &args![this.addr() + 0xc, out, this]);
    out
}

/// One tangent entry of [`fn_006bb570`]: its kind, its point from the two
/// sums (`(r * dx ± dy * h) * inverse`, `(r * dy ∓ dx * h) * inverse`).
fn tangent_point(
    e: &mut Engine,
    entry: u32,
    kind: u32,
    radius: u32,
    delta: u32,
    root: u32,
    inverse: u32,
    plus_first: bool,
) {
    e.mem.set_u32(entry + 4, kind);
    let a = x87_mul(ld(e, radius), m32(e, delta));
    let b = x87_mul(ld(e, delta + 4), m32(e, root));
    let x = if plus_first {
        x87_add(a, b)
    } else {
        x87_sub(a, b)
    };
    st(e, entry + 8, x87_mul(x, m32(e, inverse)));
    let a = x87_mul(ld(e, radius), m32(e, delta + 4));
    let b = x87_mul(ld(e, delta), m32(e, root));
    let y = if plus_first {
        x87_sub(a, b)
    } else {
        x87_add(a, b)
    };
    st(e, entry + 0xc, x87_mul(y, m32(e, inverse)));
}

// Translated from 006bb570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: the tangent lines between circles `first` and
/// `second` (radius at `+0xc`) on side `side` (0 or not), scaled by the
/// larger radius: `out` gets their count and one 0x14-byte entry each
/// (kind, two direction floats, two more: the same for the outer tangent,
/// negated for the inner one). None when the centres coincide (length 0).
/// Returns the count.
pub fn fn_006bb570(e: &mut Engine, first: Ptr, second: Ptr, side: u32, out: Ptr) -> u32 {
    let (first, second, out) = (first.addr(), second.addr(), out.addr());
    e.with_stack(0x44, |e, frame| {
        let l = |x: u32| frame.addr() + 0x44 - x;
        let delta = l(0x14);
        let (first_radius, second_radius) = (l(0x2c), l(0x4));
        let mut count = 0u32;
        e.call(POINT_SUBTRACT, &args![second, delta, first]);
        let value = pushed(e, first + 0xc);
        e.mem.set_u32(first_radius, value);
        let value = pushed(e, second + 0xc);
        e.mem.set_u32(second_radius, value);
        let zero: f64 = e.global(ZERO);
        // `TEST AH,0x44; JNP`: taken when exactly 0.
        if ld(e, second + 0xc) != zero {
            e.mem.set_u32(second_radius, 1.0f32.to_bits());
            st(
                e,
                first_radius,
                x87_div(ld(e, first + 0xc), m32(e, second + 0xc)),
            );
            let radius = pushed(e, second + 0xc);
            e.call(POINT_DIVIDE, &args![delta, radius]);
        } else if ld(e, first + 0xc) != zero {
            e.mem.set_u32(first_radius, 1.0f32.to_bits());
            st(
                e,
                second_radius,
                x87_div(ld(e, second + 0xc), m32(e, first + 0xc)),
            );
            let radius = pushed(e, first + 0xc);
            e.call(POINT_DIVIDE, &args![delta, radius]);
        }
        let length = e.call(POINT2_LENGTH_SQUARED, &args![delta]).f64();
        st(e, l(0x20), length);
        let length = l(0x20);
        if ld(e, length) == zero {
            e.mem.set_u32(out, 0);
            return 0;
        }
        let difference = l(0x8);
        st(
            e,
            difference,
            x87_sub(ld(e, first_radius), m32(e, second_radius)),
        );
        st(e, l(0x1c), x87_mul(ld(e, difference), m32(e, difference)));
        // `TEST AH,0x41; JP`: on when the square is not above the length.
        if ld(e, l(0x1c)) <= ld(e, length) {
            st(e, l(0x30), x87_div(1.0, ld(e, length)));
            st(e, l(0x40), x87_sub(ld(e, length), m32(e, l(0x1c))));
            let root = e.call(SQUARE_ROOT, &args![pushed(e, l(0x40))]).f64();
            st(e, l(0x34), root);
            let entry = out + count * 0x14;
            if side == 0 {
                tangent_point(e, entry, 0, difference, delta, l(0x34), l(0x30), true);
            } else {
                tangent_point(e, entry, 1, difference, delta, l(0x34), l(0x30), false);
            }
            let x = pushed(e, entry + 8);
            e.mem.set_u32(entry + 0x10, x);
            let y = pushed(e, entry + 0xc);
            e.mem.set_u32(entry + 0x14, y);
            count += 1;
        }
        let sum = l(0x28);
        st(e, sum, x87_add(ld(e, first_radius), m32(e, second_radius)));
        st(e, l(0x24), x87_mul(ld(e, sum), m32(e, sum)));
        if ld(e, l(0x24)) <= ld(e, length) {
            st(e, l(0x38), x87_div(1.0, ld(e, length)));
            st(e, l(0x44), x87_sub(ld(e, length), m32(e, l(0x24))));
            let root = e.call(SQUARE_ROOT, &args![pushed(e, l(0x44))]).f64();
            st(e, l(0x3c), root);
            let entry = out + count * 0x14;
            if side == 0 {
                tangent_point(e, entry, 1, sum, delta, l(0x3c), l(0x38), true);
            } else {
                tangent_point(e, entry, 0, sum, delta, l(0x3c), l(0x38), false);
            }
            let x = pushed(e, entry + 8) ^ 0x8000_0000;
            e.mem.set_u32(entry + 0x10, x);
            let y = pushed(e, entry + 0xc) ^ 0x8000_0000;
            e.mem.set_u32(entry + 0x14, y);
            count += 1;
        }
        e.mem.set_u32(out, count);
        count
    })
}

// Translated from 006bbad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: a random point in triangle `a`, `b`, `c` into
/// `out`: two random floats (`005c5420`) drawn again while their sum is
/// above 1, then `a + (b - a) * s + (c - a) * t`. Returns `out`.
pub fn fn_006bbad0(e: &mut Engine, out: Ptr, a: Ptr, b: Ptr, c: Ptr) -> Ptr {
    let (a, b, c) = (a.addr(), b.addr(), c.addr());
    e.with_stack(0x44, |e, frame| {
        let l = |x: u32| frame.addr() + 0x44 - x;
        let (to_b, to_c, s, t) = (l(0x1c), l(0x10), l(0x4), l(0x20));
        e.call(POINT_SUBTRACT, &args![b, to_b, a]);
        e.call(POINT_SUBTRACT, &args![c, to_c, a]);
        let value = e.call(RANDOM_VALUE_005C5420, &args![]).f64();
        st(e, s, value);
        let value = e.call(RANDOM_VALUE_005C5420, &args![]).f64();
        st(e, t, value);
        let one: f64 = e.global(ONE);
        // `TEST AH,0x41; JNZ`: drawn again while the sum is above 1.
        while x87_add(ld(e, s), m32(e, t)) > one {
            let value = e.call(RANDOM_VALUE_005C5420, &args![]).f64();
            st(e, s, value);
            let value = e.call(RANDOM_VALUE_005C5420, &args![]).f64();
            st(e, t, value);
        }
        let scaled_c = e
            .call(POINT_SCALED, &args![to_c, l(0x44), pushed(e, t)])
            .u32();
        let scaled_b = e
            .call(POINT_SCALED, &args![to_b, l(0x2c), pushed(e, s)])
            .u32();
        let partial = e.call(POINT_ADD, &args![a, l(0x38), scaled_b]).u32();
        e.call(POINT_ADD, &args![partial, out, scaled_c]);
    });
    out
}

/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `006a0660` (`thiscall`, `RET 4`: a 2D point): a byte of the object
/// passed to [`fn_006bb8b0`] for one of the two intersection points.
const POINT_TEST_006A0660: u32 = 0x006a_0660;

// Translated from 006bb8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: a test between circles `first` and `second`
/// (centre at `+0`, radius at `+0xc`) and the object `other`. True when
/// either radius is exactly 0, false when the centres coincide in 2D
/// (`00595c80` of `second - first` is 0); true when the 2D or the 3D
/// squared distance (`00595c80`, `004a7290`) is above the square of the
/// radius sum or below the square of the radius difference. Otherwise the
/// two 2D intersection points of the circles, relative to `first` and
/// divided by its radius, are offered to `other`'s `006a0660`: false as
/// soon as one answers true, else true.
pub fn fn_006bb8b0(e: &mut Engine, first: Ptr, other: Ptr, second: Ptr) -> bool {
    let (first, other, second) = (first.addr(), other.addr(), second.addr());
    e.with_stack(0x50, |e, frame| {
        let l = |x: u32| frame.addr() + 0x50 - x;
        let zero: f64 = e.global(ZERO);
        // `TEST AH,0x44; JP`: on unless exactly equal.
        if ld(e, first + 0xc) == zero {
            return true;
        }
        if ld(e, second + 0xc) == zero {
            return true;
        }
        let delta = l(0x48);
        e.call(POINT_SUBTRACT, &args![second, delta, first]);
        let length = e.call(POINT_LENGTH_SQUARED, &args![delta]).f64();
        st(e, l(0x4), length);
        let flat = e.call(POINT2_LENGTH_SQUARED, &args![delta]).f64();
        st(e, l(0x3c), flat);
        if ld(e, l(0x3c)) == zero {
            return false;
        }
        st(e, l(0x8), x87_add(ld(e, first + 0xc), m32(e, second + 0xc)));
        // `FCOMPP; TEST AH,0x41; JNZ`: on when the distance is not above
        // the square (NaN included).
        let sum = x87_mul(ld(e, l(0x8)), m32(e, l(0x8)));
        if ld(e, l(0x3c)) > sum {
            return true;
        }
        let sum = x87_mul(ld(e, l(0x8)), m32(e, l(0x8)));
        if ld(e, l(0x4)) > sum {
            return true;
        }
        st(
            e,
            l(0x2c),
            x87_sub(ld(e, first + 0xc), m32(e, second + 0xc)),
        );
        // `FCOMPP; TEST AH,0x5; JP`: on when the distance is not below the
        // square (NaN included).
        let difference = x87_mul(ld(e, l(0x2c)), m32(e, l(0x2c)));
        if ld(e, l(0x3c)) < difference {
            return true;
        }
        let difference = x87_mul(ld(e, l(0x2c)), m32(e, l(0x2c)));
        if ld(e, l(0x4)) < difference {
            return true;
        }
        st(e, l(0x38), x87_mul(ld(e, first + 0xc), m32(e, first + 0xc)));
        st(
            e,
            l(0x28),
            x87_mul(ld(e, second + 0xc), m32(e, second + 0xc)),
        );
        let root = e.call(SQUARE_ROOT, &args![pushed(e, l(0x3c))]).f64();
        st(e, l(0x30), root);
        // `FLD; FLD1; FDIVRP`: 1 / the value.
        st(e, l(0x24), x87_div(1.0, ld(e, l(0x30))));
        st(e, l(0x20), x87_div(1.0, ld(e, first + 0xc)));
        let half: f64 = e.global(HALF);
        let along = x87_add(x87_sub(ld(e, l(0x38)), m32(e, l(0x28))), m32(e, l(0x3c)));
        st(e, l(0x14), x87_mul(x87_mul(along, half), m32(e, l(0x24))));
        st(e, l(0x4c), x87_mul(ld(e, l(0x14)), m32(e, l(0x14))));
        st(e, l(0x50), x87_sub(ld(e, l(0x38)), m32(e, l(0x4c))));
        let root = e.call(SQUARE_ROOT, &args![pushed(e, l(0x50))]).f64();
        st(e, l(0x34), root);
        let (dx, dy) = (l(0x48), l(0x44));
        let (along, across) = (l(0x14), l(0x34));
        let (inverse_radius, inverse_length) = (l(0x20), l(0x24));
        let scale = |e: &Engine, value: f64| {
            x87_mul(
                x87_mul(value, m32(e, inverse_radius)),
                m32(e, inverse_length),
            )
        };

        e.call(SELF, &args![l(0x1c)]);
        let x = x87_add(
            x87_mul(ld(e, along), m32(e, dx)),
            x87_mul(ld(e, across), m32(e, dy)),
        );
        st(e, l(0x1c), scale(e, x));
        let y = x87_sub(
            x87_mul(ld(e, along), m32(e, dy)),
            x87_mul(ld(e, across), m32(e, dx)),
        );
        st(e, l(0x18), scale(e, y));

        e.call(SELF, &args![l(0x10)]);
        let x = x87_sub(
            x87_mul(ld(e, along), m32(e, dx)),
            x87_mul(ld(e, across), m32(e, dy)),
        );
        st(e, l(0x10), scale(e, x));
        let y = x87_add(
            x87_mul(ld(e, along), m32(e, dy)),
            x87_mul(ld(e, across), m32(e, dx)),
        );
        st(e, l(0xc), scale(e, y));

        if e.call(POINT_TEST_006A0660, &args![other, l(0x1c)]).bool() {
            return false;
        }
        if e.call(POINT_TEST_006A0660, &args![other, l(0x10)]).bool() {
            return false;
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_ab00, fn_006bab00(Ptr, f32, Ptr, Ptr) -> bool),
        entry!(0x006b_ac20, fn_006bac20(Ptr, Ptr, Ptr) -> bool),
        entry!(0x006b_b510, fn_006bb510(Ptr, Ptr)),
        entry!(0x006b_b540, fn_006bb540(Ptr, Ptr) -> Ptr),
        entry!(0x006b_b570, fn_006bb570(Ptr, Ptr, u32, Ptr) -> u32),
        entry!(0x006b_bad0, fn_006bbad0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006b_b8b0, fn_006bb8b0(Ptr, Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(e: &Engine, p: u32, n: u32) -> Vec<f64> {
        (0..n).map(|i| f64::from(e.mem.f32(p + 4 * i))).collect()
    }

    fn write(e: &mut Engine, p: u32, v: &[f64]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x as f32);
        }
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn close(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() < 1e-5
    }

    /// Point doubles over plain floats.
    fn points(e: &mut Engine) {
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_f32(EPSILON, 0.001);
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[2], 3));
            let r: Vec<f64> = p.iter().zip(q).map(|(x, y)| x - y).collect();
            write(e, a[1], &r);
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[2], 3));
            let r: Vec<f64> = p.iter().zip(q).map(|(x, y)| x + y).collect();
            write(e, a[1], &r);
            a[1].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let s = f64::from(f32::from_bits(a[1]));
            let r: Vec<f64> = read(e, a[2], 3).iter().map(|x| x * s).collect();
            write(e, a[0], &r);
            a[0].into_ret()
        });
        e.register(POINT_SCALED, |e, a| {
            let s = f64::from(f32::from_bits(a[2]));
            let r: Vec<f64> = read(e, a[0], 3).iter().map(|x| x * s).collect();
            write(e, a[1], &r);
            a[1].into_ret()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            st0(read(e, a[0], 3).iter().map(|x| x * x).sum())
        });
        e.register(POINT2_LENGTH_SQUARED, |e, a| {
            st0(read(e, a[0], 2).iter().map(|x| x * x).sum())
        });
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[1], 3));
            st0(p.iter().zip(q).map(|(x, y)| x * y).sum())
        });
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(POINT_SUBTRACT_IN_PLACE, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[1], 3));
            let r: Vec<f64> = p.iter().zip(q).map(|(x, y)| x - y).collect();
            write(e, a[0], &r);
            Ret::default()
        });
        e.register(POINT_DIVIDE, |e, a| {
            let s = f64::from(f32::from_bits(a[1]));
            let r: Vec<f64> = read(e, a[0], 3).iter().map(|x| x / s).collect();
            write(e, a[0], &r);
            Ret::default()
        });
        e.register(POINT2_COPY, |e, a| {
            let r = read(e, a[1], 2);
            write(e, a[0], &r);
            Ret::default()
        });
        e.register(SQUARE_ROOT, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).sqrt())
        });
        e.register(ABSOLUTE, |_, a| st0(f64::from(f32::from_bits(a[0])).abs()));
        e.register(SEGMENT_CONSTRUCT, |e, a| {
            let r = read(e, a[1], 6);
            write(e, a[0], &r);
            Ret::default()
        });
        e.register(CIRCLE_CENTER, |e, a| {
            let r = read(e, a[0], 3);
            write(e, a[1], &r);
            a[1].into_ret()
        });
    }

    #[test]
    fn segment_against_circle() {
        let mut e = Engine::new();
        points(&mut e);
        let a = floats(&mut e, &[0.0, 0.0, 0.0]);
        let b = floats(&mut e, &[10.0, 0.0, 0.0]);
        let hit = |e: &mut Engine, c: &[f32], r: f32| {
            let center = floats(e, c);
            e.call(0x006b_ab00, &args![center, r, a, b]).bool()
        };
        assert!(hit(&mut e, &[5.0, 1.0, 0.0], 2.0));
        assert!(!hit(&mut e, &[5.0, 2.0, 0.0], 2.0));
        assert!(hit(&mut e, &[-1.0, 0.0, 0.0], 1.5));
        assert!(!hit(&mut e, &[-2.0, 0.0, 0.0], 1.5));
        assert!(hit(&mut e, &[11.0, 0.0, 0.0], 1.5));
        assert!(!hit(&mut e, &[12.0, 0.0, 0.0], 1.5));
    }

    #[test]
    fn segment_helpers() {
        let mut e = Engine::new();
        points(&mut e);
        let segment = floats(&mut e, &[1.0, 2.0, 3.0, 5.0, 7.0, 9.0]);
        let offset = floats(&mut e, &[1.0, 1.0, 1.0]);
        e.call(0x006b_b510, &args![segment, offset]);
        assert_eq!(read(&e, segment, 6), vec![0.0, 1.0, 2.0, 4.0, 6.0, 8.0]);
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x006b_b540, &args![segment, out]).u32(), out);
        assert_eq!(read(&e, out, 3), vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn tangents_between_two_circles() {
        let mut e = Engine::new();
        points(&mut e);
        let first = floats(&mut e, &[0.0, 0.0, 0.0, 1.0]);
        let second = floats(&mut e, &[4.0, 0.0, 0.0, 1.0]);
        let out = e.mem.alloc(0x2c);
        assert_eq!(
            e.call(0x006b_b570, &args![first, second, 0u32, out]).u32(),
            2
        );
        assert_eq!(e.mem.u32(out), 2);
        // The outer tangent (radii equal): kind 0, direction (0, -1).
        assert_eq!(e.mem.u32(out + 4), 0);
        let outer = read(&e, out + 8, 4);
        assert!(close(outer[0], 0.0) && close(outer[1], -1.0));
        assert_eq!(outer[2..], outer[..2]);
        // The inner tangent: kind 1, (0.5, -sqrt(12)/4), then negated.
        assert_eq!(e.mem.u32(out + 0x18), 1);
        let inner = read(&e, out + 0x1c, 4);
        assert!(close(inner[0], 0.5) && close(inner[1], -(12f64.sqrt()) / 4.0));
        assert!(close(inner[2], -0.5) && close(inner[3], 12f64.sqrt() / 4.0));
        // The other side mirrors the kinds.
        e.call(0x006b_b570, &args![first, second, 1u32, out]);
        assert_eq!(e.mem.u32(out + 4), 1);
        let outer = read(&e, out + 8, 2);
        assert!(close(outer[0], 0.0) && close(outer[1], 1.0));
        assert_eq!(e.mem.u32(out + 0x18), 0);
        // The same centre: none.
        let same = floats(&mut e, &[0.0, 0.0, 0.0, 2.0]);
        assert_eq!(e.call(0x006b_b570, &args![first, same, 0u32, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn random_point_in_a_triangle() {
        let mut e = Engine::new();
        points(&mut e);
        let draws = std::cell::Cell::new(0usize);
        e.register_double(RANDOM_VALUE_005C5420, move |_, _| {
            let values = [0.8, 0.7, 0.25, 0.5];
            let value = values[draws.get()];
            draws.set(draws.get() + 1);
            st0(value)
        });
        let a = floats(&mut e, &[1.0, 1.0, 0.0]);
        let b = floats(&mut e, &[5.0, 1.0, 0.0]);
        let c = floats(&mut e, &[1.0, 9.0, 2.0]);
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_bad0, &args![out, a, b, c]).u32(), out);
        assert_eq!(read(&e, out, 3), vec![2.0, 5.0, 1.0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|c| c.0 == RANDOM_VALUE_005C5420).count(),
            4
        );
    }

    struct Crossing {
        kind: u32,
        first: (u32, f64, f64, f64),
        second: Option<(u32, f64, f64, f64)>,
    }

    fn cross(e: &mut Engine, radius: f32, from: [f32; 2], to: [f32; 2]) -> (bool, Crossing) {
        let circle = floats(e, &[0.0, 0.0, 0.0, radius]);
        let segment = floats(e, &[from[0], from[1], 0.0, to[0], to[1], 0.0]);
        let out = e.mem.alloc(0x24);
        let hit = e.call(0x006b_ac20, &args![circle, segment, out]).bool();
        let entry = |e: &Engine, at: u32| {
            let v = read(e, at + 4, 3);
            (e.mem.u32(at), v[0], v[1], v[2])
        };
        let crossing = Crossing {
            kind: e.mem.u32(out),
            first: entry(e, out + 4),
            second: Some(entry(e, out + 0x14)),
        };
        (hit, crossing)
    }

    #[test]
    fn crossings_along_x_and_y() {
        let mut e = Engine::new();
        points(&mut e);
        let root = 0.75f64.sqrt();
        // Horizontal: solved along x; dx >= 0 puts the smaller x first.
        let (hit, c) = cross(&mut e, 2.0, [-4.0, 1.0], [4.0, 1.0]);
        assert!(hit);
        assert_eq!(c.kind, 4);
        let (k1, x1, y1, t1) = c.first;
        assert_eq!(k1, 1);
        assert!(close(x1, -root) && close(y1, 0.5) && close(t1, (2.0 - root) / 4.0));
        let (k2, x2, _, t2) = c.second.unwrap();
        assert_eq!(k2, 1);
        assert!(close(x2, root) && close(t2, (2.0 + root) / 4.0));
        // Vertical: solved along y.
        let (hit, c) = cross(&mut e, 2.0, [1.0, -4.0], [1.0, 4.0]);
        assert!(hit);
        assert_eq!(c.kind, 4);
        let (_, x1, y1, t1) = c.first;
        assert!(close(x1, 0.5) && close(y1, -root) && close(t1, (2.0 - root) / 4.0));
    }

    #[test]
    fn tangent_inside_missing_and_degenerate() {
        let mut e = Engine::new();
        points(&mut e);
        // A tangent at y = 2: one point at t = 0.5.
        let (hit, c) = cross(&mut e, 2.0, [-4.0, 2.0], [4.0, 2.0]);
        assert!(hit);
        assert_eq!(c.kind, 3);
        let (k, x, y, t) = c.first;
        assert_eq!(k, 1);
        assert!(close(x, 0.0) && close(y, 1.0) && close(t, 0.5));
        // Wholly inside: kind 5, false.
        let (hit, c) = cross(&mut e, 2.0, [-1.0, 0.0], [1.0, 0.0]);
        assert!(!hit);
        assert_eq!(c.kind, 5);
        // Apart: kind 0.
        let (hit, c) = cross(&mut e, 2.0, [-4.0, 3.0], [4.0, 3.0]);
        assert!(!hit);
        assert_eq!(c.kind, 0);
        // Only the far crossing on the segment: it becomes the first.
        let (hit, c) = cross(&mut e, 2.0, [0.0, 1.0], [4.0, 1.0]);
        assert!(hit);
        assert_eq!(c.kind, 1);
        let (_, x, _, _) = c.first;
        assert!(close(x, 0.75f64.sqrt()));
        // Only the near one: kind 2.
        let (hit, c) = cross(&mut e, 2.0, [-4.0, 1.0], [0.0, 1.0]);
        assert!(hit);
        assert_eq!(c.kind, 2);
        // A radius of 0: false, nothing written.
        let circle = floats(&mut e, &[0.0, 0.0, 0.0, 0.0]);
        let segment = floats(&mut e, &[0.0; 6]);
        let out = e.mem.alloc(0x24);
        e.mem.set_u32(out, 9);
        assert!(!e.call(0x006b_ac20, &args![circle, segment, out]).bool());
        assert_eq!(e.mem.u32(out), 9);
    }

    /// Runs `006bb8b0` on two circles with `006a0660` answering `answers`
    /// in turn; returns the result and the points it was offered.
    fn circle_test(first: [f32; 4], second: [f32; 4], answers: [bool; 2]) -> (bool, Vec<[f32; 2]>) {
        let mut e = Engine::new();
        points(&mut e);
        e.map(HALF & !0xfff, 0x1000);
        e.set_global(HALF, 0.5f64);
        let mut turn = 0usize;
        e.register_double(POINT_TEST_006A0660, move |_, _| {
            let answer = answers[turn.min(1)];
            turn += 1;
            u32::from(answer).into_ret()
        });
        let first = floats(&mut e, &first);
        let second = floats(&mut e, &second);
        let other = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let result = e.call(0x006b_b8b0, &args![first, other, second]).bool();
        let offered = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == POINT_TEST_006A0660)
            .map(|(_, w)| {
                assert_eq!(w[0], other);
                [e.mem.f32(w[1]), e.mem.f32(w[1] + 4)]
            })
            .collect();
        (result, offered)
    }

    #[test]
    fn circles_rejected_before_the_intersection_points() {
        let no = [false, false];
        // A zero radius on either side: true, nothing offered.
        assert_eq!(
            circle_test([0.0; 4], [3.0, 0.0, 0.0, 2.0], no),
            (true, vec![])
        );
        assert_eq!(
            circle_test([0.0, 0.0, 0.0, 2.0], [3.0, 0.0, 0.0, 0.0], no),
            (true, vec![])
        );
        // Same centre in 2D: false.
        assert_eq!(
            circle_test([0.0, 0.0, 0.0, 2.0], [0.0, 0.0, 5.0, 1.0], no),
            (false, vec![])
        );
        // Too far apart in 2D, or in 3D only.
        assert_eq!(
            circle_test([0.0, 0.0, 0.0, 2.0], [5.0, 0.0, 0.0, 2.0], no),
            (true, vec![])
        );
        assert_eq!(
            circle_test([0.0, 0.0, 0.0, 2.0], [3.0, 0.0, 4.0, 2.0], no),
            (true, vec![])
        );
        // One inside the other.
        assert_eq!(
            circle_test([0.0, 0.0, 0.0, 5.0], [1.0, 0.0, 0.0, 1.0], no),
            (true, vec![])
        );
    }

    #[test]
    fn circles_offer_both_intersection_points() {
        let first = [0.0, 0.0, 0.0, 2.0];
        let second = [3.0, 0.0, 0.0, 2.0];
        let (result, offered) = circle_test(first, second, [false, false]);
        assert!(result);
        // Intersections at (1.5, -+sqrt(1.75)) relative to the first
        // centre, divided by its radius 2.
        let y = (1.75f64.sqrt() / 2.0) as f32;
        assert_eq!(offered.len(), 2);
        assert!(close(f64::from(offered[0][0]), 0.75));
        assert!(close(f64::from(offered[0][1]), -f64::from(y)));
        assert!(close(f64::from(offered[1][0]), 0.75));
        assert!(close(f64::from(offered[1][1]), f64::from(y)));
        // The first point accepted: false at once.
        let (result, offered) = circle_test(first, second, [true, false]);
        assert!(!result);
        assert_eq!(offered.len(), 1);
        // The second accepted.
        let (result, offered) = circle_test(first, second, [false, true]);
        assert!(!result);
        assert_eq!(offered.len(), 2);
    }
}
