//! `Havok SDK/hkpsimpleconstraintutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is a single-precision operation, bit for bit as the exe
//! does it. The scalar parts that go through the x87 stack compute in `double`, load through
//! `x87_quiet` and round to `float` where the exe stores.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, maxps, movhlps, movlhps, shufps, store_bits, store_vec, sum3,
    unpckhps, unpcklps, zip_with, Vec4, ONES, SIGN_MASK,
};

/// The `float` floor of a body's inverse mass (`010da2bc`), the `float` floor of the
/// determinant (`010da2b8`), the `double` limit under which the first diagonal element is
/// replaced by the mass floor (`010da2b0`), and the `double` limit (`010da2a8`) and `float`
/// floor (`010da2a0`) of the 2x2 minor of the last two rows.
const MASS_FLOOR: u32 = 0x010d_a2bc;
const DETERMINANT_FLOOR: u32 = 0x010d_a2b8;
const DIAGONAL_LIMIT: u32 = 0x010d_a2b0;
const MINOR_LIMIT: u32 = 0x010d_a2a8;
const MINOR_FLOOR: u32 = 0x010d_a2a0;
/// The mask that clears the fourth lane (`010cbfc0`).
const KEEP_XYZ_MASK: u32 = 0x010c_bfc0;
/// The one-time initialisation flag byte (`0126a42c`) and its initialiser (`00df22f0`, cdecl,
/// the flag's address).
const FLYING_COLORS_FLAG: u32 = 0x0126_a42c;
const INIT_FLYING_COLORS: u32 = 0x00df_22f0;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The `double` the friction term is compared with (`01012060`).
const FRICTION_LIMIT: u32 = 0x0101_2060;

pub(crate) fn splat(value: f32) -> Vec4 {
    [value; 4]
}

pub(crate) fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

pub(crate) fn sub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

pub(crate) fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// `a.yzx * b.zxy - a.zxy * b.yzx` (`SHUFPS` 0xC9 and 0xD2): the cross product `a x b` in
/// every lane, the fourth lane being `a.w * b.w - a.w * b.w`.
pub(crate) fn cross(a: Vec4, b: Vec4) -> Vec4 {
    let plus = mul(shufps(a, a, 0xc9), shufps(b, b, 0xd2));
    let minus = mul(shufps(a, a, 0xd2), shufps(b, b, 0xc9));
    sub(plus, minus)
}

/// `(v.y * second + v.x * first) + v.z * third`: the vector times the matrix of three rows.
pub(crate) fn rows_times(v: Vec4, first: Vec4, second: Vec4, third: Vec4) -> Vec4 {
    add(
        add(mul(splat(v[1]), second), mul(splat(v[0]), first)),
        mul(splat(v[2]), third),
    )
}

// Translated from 00df5950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleConstraintUtil_InitInfo` (engine map), cdecl `(body_a, body_b, rotation, info)`.
/// Builds the constraint info block at `info` from two bodies and the 3x3 `rotation` (three
/// vectors). `info + 0x100 ..` gets the rotation; `info + 0x130 ..` (three vectors) starts as
/// the diagonal matrix of `body_a[+0x4c] + body_b[+0x4c]` and collects, per body, the
/// matrix product of the body's tensor (`body[+0x10 ..]`) with the cross products of its
/// vector at `+0` with the rotation rows. Each body's section at `info` (resp. `info +
/// 0x80`) is: the cross rows at `+0 ..`, the tensor products at `+0x30 ..`, the vector
/// `body[+0x40]` at `+0x60`, `1 / max(body[+0x4c], mass floor)` at `+0x70` and the word
/// `body[+0x50]` at `+0x74`. The accumulated 3x3 matrix is then inverted (the adjugate over
/// `max(determinant, floor)`) into `info + 0x160 ..`, and the lanes `+0x13c` and `+0x14c`
/// (the fourth lanes of the first two accumulated rows) become the reciprocals of the first
/// diagonal element and of the 2x2 minor of the last two rows, each replaced by its floor
/// when under its limit.
pub fn hk_simple_constraint_util_init_info(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    rotation: u32,
    info: u32,
) {
    let sum = (f64::from(x87_quiet(e.mem.f32(body_a + 0x4c))) + f64::from(e.mem.f32(body_b + 0x4c)))
        as f32;
    for row in 0..3 {
        store_vec(e, info + 0x130 + 0x10 * row, [0.0; 4]);
    }
    for diagonal in [0x130, 0x144, 0x158] {
        e.mem.set_f32(info + diagonal, sum);
    }
    for row in 0..3 {
        copy_vec(e, rotation + 0x10 * row, info + 0x100 + 0x10 * row);
    }
    let mass_floor = x87_quiet(e.mem.f32(MASS_FLOOR));
    let mut body = body_a;
    for base in [info, info + 0x80] {
        let position = load_vec(e, body);
        for row in 0..3 {
            let direction = load_vec(e, rotation + 0x10 * row);
            store_vec(e, base + 0x10 * row, cross(position, direction));
        }
        let tensor = [
            load_vec(e, body + 0x10),
            load_vec(e, body + 0x20),
            load_vec(e, body + 0x30),
        ];
        for row in (0..3).rev() {
            let cross_row = load_vec(e, base + 0x10 * row);
            let product = rows_times(cross_row, tensor[0], tensor[1], tensor[2]);
            store_vec(e, base + 0x30 + 0x10 * row, product);
        }
        // The cross rows are transposed through the x87 (three swaps of lanes).
        for (first, second) in [(0x10, 0x04), (0x20, 0x08), (0x24, 0x18)] {
            let kept = x87_quiet_bits(e.mem.u32(base + first));
            let moved = x87_quiet_bits(e.mem.u32(base + second));
            e.mem.set_u32(base + first, moved);
            e.mem.set_u32(base + second, kept);
        }
        let transposed = [
            load_vec(e, base),
            load_vec(e, base + 0x10),
            load_vec(e, base + 0x20),
        ];
        let mut products = [[0.0f32; 4]; 3];
        for row in (0..3).rev() {
            let tensor_row = load_vec(e, base + 0x30 + 0x10 * row);
            products[row as usize] = add(
                add(
                    mul(splat(tensor_row[0]), transposed[0]),
                    mul(splat(tensor_row[1]), transposed[1]),
                ),
                mul(splat(tensor_row[2]), transposed[2]),
            );
        }
        for (row, product) in products.into_iter().enumerate() {
            let at = info + 0x130 + 0x10 * row as u32;
            let accumulated = add(load_vec(e, at), product);
            store_vec(e, at, accumulated);
        }
        copy_vec(e, body + 0x40, base + 0x60);
        let mass = x87_quiet(e.mem.f32(body + 0x4c));
        let limited = if f64::from(mass) > f64::from(mass_floor) {
            mass
        } else {
            mass_floor
        };
        e.mem
            .set_f32(base + 0x70, (1.0 / f64::from(limited)) as f32);
        let word = e.mem.u32(body + 0x50);
        e.mem.set_u32(base + 0x74, word);
        body = body_b;
    }
    let first = load_vec(e, info + 0x130);
    let second = load_vec(e, info + 0x140);
    let third = load_vec(e, info + 0x150);
    let determinant_floor = x87_quiet(e.mem.f32(DETERMINANT_FLOOR));
    let second_third = cross(second, third);
    let determinant = sum3(mul(second_third, first));
    let limited = maxps(determinant, determinant_floor);
    let one = e.mem.f32(ONES);
    let inverse = splat(one / limited);
    let third_first = cross(third, first);
    let first_second = cross(first, second);
    store_vec(e, info + 0x160, mul(second_third, inverse));
    store_vec(e, info + 0x170, mul(third_first, inverse));
    store_vec(e, info + 0x180, mul(first_second, inverse));
    let diagonal = x87_quiet(e.mem.f32(info + 0x130));
    let divisor = if f64::from(diagonal) < e.mem.f64(DIAGONAL_LIMIT) {
        x87_quiet(e.mem.f32(MASS_FLOOR))
    } else {
        diagonal
    };
    e.mem
        .set_f32(info + 0x13c, (1.0 / f64::from(divisor)) as f32);
    let last_diagonal = f64::from(x87_quiet(e.mem.f32(info + 0x158)));
    let middle_diagonal = f64::from(e.mem.f32(info + 0x144));
    let upper = f64::from(x87_quiet(e.mem.f32(info + 0x154)));
    let right = f64::from(e.mem.f32(info + 0x148));
    let minor = (last_diagonal * middle_diagonal - upper * right) as f32;
    let divisor = if f64::from(minor) < e.mem.f64(MINOR_LIMIT) {
        x87_quiet(e.mem.f32(MINOR_FLOOR))
    } else {
        minor
    };
    e.mem
        .set_f32(info + 0x14c, (1.0 / f64::from(divisor)) as f32);
}

// Translated from 00df5d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsimpleconstraintutil.cpp`), cdecl `(info, vector, first, second)`: applies an
/// impulse `vector` (three lanes used) to two velocity records `first` and `second` (the
/// linear vector at `+0`, the angular one at `+0x10`). With `world` the impulse turned by the
/// rotation at `info + 0x100`: `first[0] += info[+0x6c] * world`, `second[0] -= info[+0xec] *
/// world` (the fourth lanes of `info + 0x60` and `info + 0xe0`); `first[1] += ` the impulse
/// taken through the three vectors at `info + 0x30`, `second[1] -=` the same through
/// `info + 0xb0`. The fourth lanes of all four results are then cleared (mask `010cbfc0`).
pub fn fn_00df5d20(e: &mut Engine, info: u32, vector: u32, first: u32, second: u32) {
    let rotation = [
        load_vec(e, info + 0x100),
        load_vec(e, info + 0x110),
        load_vec(e, info + 0x120),
    ];
    let v = load_vec(e, vector);
    let world = rows_times(v, rotation[0], rotation[1], rotation[2]);
    let first_scale = load_vec(e, info + 0x60)[3];
    let moved = add(mul(splat(first_scale), world), load_vec(e, first));
    store_vec(e, first, moved);
    let second_scale = load_vec(e, info + 0xe0)[3];
    let moved = sub(load_vec(e, second), mul(splat(second_scale), world));
    store_vec(e, second, moved);
    let v = load_vec(e, vector);
    let rows = [
        load_vec(e, info + 0x30),
        load_vec(e, info + 0x40),
        load_vec(e, info + 0x50),
    ];
    let angular = rows_times(v, rows[0], rows[1], rows[2]);
    let moved = add(angular, load_vec(e, first + 0x10));
    store_vec(e, first + 0x10, moved);
    let v = load_vec(e, vector);
    let rows = [
        load_vec(e, info + 0xb0),
        load_vec(e, info + 0xc0),
        load_vec(e, info + 0xd0),
    ];
    let angular = rows_times(v, rows[0], rows[1], rows[2]);
    let moved = sub(load_vec(e, second + 0x10), angular);
    store_vec(e, second + 0x10, moved);
    let mask = load_bits(e, KEEP_XYZ_MASK);
    for address in [first + 0x10, second + 0x10, first, second] {
        let bits = load_bits(e, address);
        let kept: [u32; 4] = std::array::from_fn(|lane| bits[lane] & mask[lane]);
        store_bits(e, address, kept);
    }
}

// Translated from 00df5e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleConstraintUtil_getPointVelocity` (engine map), cdecl `(info, first, second,
/// out)`: the relative velocity of the point the info block describes. With the position
/// difference `d = first[0] - second[0]`, `transposed` the rotation at `info + 0x100`
/// transposed (the exe builds it with `UNPCKLPS` / `MOVLHPS` / `SHUFPS`, so its fourth
/// lanes hold the rotation's third row), `q(w, base) = (w.y * base[1] + w.x * base[0]) + w.z
/// * base[2]`: `out = (q(first[1], info) - q(second[1], info + 0x80)) + d.y * transposed[1] +
/// d.x * transposed[0] + d.z * transposed[2]`, grouped the way the exe adds them.
pub fn hk_simple_constraint_util_get_point_velocity(
    e: &mut Engine,
    info: u32,
    first: u32,
    second: u32,
    out: u32,
) {
    let difference = sub(load_vec(e, first), load_vec(e, second));
    let m0 = load_vec(e, info + 0x100);
    let m1 = load_vec(e, info + 0x110);
    let m2 = load_vec(e, info + 0x120);
    let low = unpcklps(m0, m1);
    let high = unpckhps(m0, m1);
    let transposed0 = movlhps(low, m2);
    let transposed1 = shufps(movhlps(transposed0, low), m2, 0xd4);
    let transposed2 = shufps(high, m2, 0xe4);
    let angular_first = load_vec(e, first + 0x10);
    let angular_second = load_vec(e, second + 0x10);
    let rows_first = [
        load_vec(e, info),
        load_vec(e, info + 0x10),
        load_vec(e, info + 0x20),
    ];
    let rows_second = [
        load_vec(e, info + 0x80),
        load_vec(e, info + 0x90),
        load_vec(e, info + 0xa0),
    ];
    let turned_first = rows_times(angular_first, rows_first[0], rows_first[1], rows_first[2]);
    let turned_second = rows_times(
        angular_second,
        rows_second[0],
        rows_second[1],
        rows_second[2],
    );
    let angular = sub(turned_first, turned_second);
    let linear = rows_times(difference, transposed0, transposed1, transposed2);
    store_vec(e, out, add(angular, linear));
}

// Translated from 00df5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleConstraintUtil_Collide` (engine map), cdecl `(info, params, first, second)`;
/// the result is the `float` the exe leaves in ST0. `params` is the friction/restitution
/// block: floats `p0` at `+0`, `p1` (`+4`), `p2` (`+8`, the largest impulse), `p3` (`+0xc`),
/// `p4` (`+0x10`), `p5` (`+0x14`), and the byte at `+0x18` this function sets to `1` when it
/// clamps an impulse (`0` otherwise). If the initialisation flag byte at `0126a42c` is zero
/// the initialiser `00df22f0` is called with its address; when the flag stays zero the
/// result is `0.0`.
///
/// With `v` the point velocity (`getPointVelocity`): when `v[0] > p3`, `v[0]` becomes `p3` and
/// `p4 <= p3` ends the call with `0.0`. Then `-v` is formed and its first lane replaced by
/// `n0 + (p4 + p1 * n0)` (`n0 = -v[0]`); that is turned by the matrix rows at `info + 0x160`
/// into the impulse `j`; a first lane above `p2` scales `j` by `p2 / j[0]` and sets the flag.
/// When the cone test `p0 * p0 * j[0]^2 < j[2]^2 + j[1]^2` fails, or `j[0] < 0`, the impulse is
/// rebuilt in two steps (a normal impulse limited by `p2`, then the friction impulse from the
/// 2x2 block at `info + 0x130`, scaled to the cone, a second normal part, and a residual
/// term using the CRT square root `00ec6040`); the first lane of the result never
/// exceeds `p2`. The impulse is then applied by `00df5d20` and its first lane returned.
pub fn hk_simple_constraint_util_collide(
    e: &mut Engine,
    info: u32,
    params: u32,
    first: u32,
    second: u32,
) -> f32 {
    if e.mem.u8(FLYING_COLORS_FLAG) == 0 {
        e.call(INIT_FLYING_COLORS, &args![FLYING_COLORS_FLAG]);
        if e.mem.u8(FLYING_COLORS_FLAG) == 0 {
            return 0.0;
        }
    }
    e.with_stack(0x60, |e, frame| {
        let velocity = frame.addr() + 0x30;
        let impulse = frame.addr() + 0x40;
        hk_simple_constraint_util_get_point_velocity(e, info, first, second, velocity);
        let p3 = x87_quiet(e.mem.f32(params + 0xc));
        if p3 < x87_quiet(e.mem.f32(velocity)) {
            e.mem.set_f32(velocity, p3);
            let p4 = x87_quiet(e.mem.f32(params + 0x10));
            if p4 <= p3 {
                return 0.0;
            }
        }
        // The velocity is negated and its first lane replaced.
        let signs = load_bits(e, SIGN_MASK);
        let original = load_vec(e, velocity);
        let negated: Vec4 =
            std::array::from_fn(|lane| f32::from_bits(original[lane].to_bits() ^ signs[lane]));
        store_vec(e, velocity, negated);
        let p1 = x87_quiet(e.mem.f32(params + 4));
        let negative_first = x87_quiet(e.mem.f32(velocity));
        let friction_term = (f64::from(p1) * f64::from(negative_first)) as f32;
        let p4 = x87_quiet(e.mem.f32(params + 0x10));
        let new_first =
            (f64::from(negative_first) + (f64::from(p4) + f64::from(friction_term))) as f32;
        e.mem.set_f32(velocity, new_first);
        let adjusted = load_vec(e, velocity);
        let rows = [
            load_vec(e, info + 0x160),
            load_vec(e, info + 0x170),
            load_vec(e, info + 0x180),
        ];
        let p2 = x87_quiet(e.mem.f32(params + 8));
        let mut jacobian = rows_times(adjusted, rows[0], rows[1], rows[2]);
        store_vec(e, impulse, jacobian);
        let mut first_lane = x87_quiet(e.mem.f32(impulse));
        e.mem.set_u8(params + 0x18, 0);
        if p2 < first_lane {
            let ratio = (f64::from(p2) / f64::from(first_lane)) as f32;
            jacobian = mul(splat(ratio), jacobian);
            store_vec(e, impulse, jacobian);
            first_lane = x87_quiet(e.mem.f32(impulse));
            e.mem.set_u8(params + 0x18, 1);
        }
        let p0 = x87_quiet(e.mem.f32(params));
        let squares = mul(jacobian, jacobian);
        let tangent_squared = (f64::from(squares[2]) + f64::from(squares[1])) as f32;
        let cone = ((f64::from(p0) * f64::from(p0)) * f64::from(squares[0])) as f32;
        if cone < tangent_squared || first_lane < 0.0 {
            correct_impulse(e, info, params, frame.addr(), p2, adjusted, friction_term);
        }
        // The value left on the x87 stack is the impulse's first lane.
        let result = x87_quiet(e.mem.f32(impulse));
        fn_00df5d20(e, info, impulse, first, second);
        result
    })
}

/// The rebuild of the impulse `hkSimpleConstraintUtil_Collide` does when the cone test fails
/// (`frame + 0x30` is the adjusted velocity vector, `frame + 0x40` the impulse being built,
/// `adjusted` the velocity vector as the first step left it and `friction_term` the product
/// of `p1` and the negated first lane). Writes the impulse's four lanes at `frame + 0x40`
/// and may set the clamp flag at `params + 0x18`.
fn correct_impulse(
    e: &mut Engine,
    info: u32,
    params: u32,
    frame: u32,
    p2: f32,
    adjusted: Vec4,
    friction_term: f32,
) {
    let velocity = frame + 0x30;
    let impulse = frame + 0x40;
    store_vec(e, impulse, [0.0; 4]);
    let lane_two = x87_quiet(e.mem.f32(velocity + 8));
    let lane_one = x87_quiet(e.mem.f32(velocity + 4));
    let tangent_squared = (f64::from(lane_two) * f64::from(lane_two)
        + f64::from(lane_one) * f64::from(lane_one)) as f32;
    let normal_first = x87_quiet(e.mem.f32(velocity));
    let mut normal = (f64::from(normal_first) * f64::from(e.mem.f32(info + 0x13c))) as f32;
    if normal > p2 {
        e.mem.set_u8(params + 0x18, 1);
        normal = p2;
    }
    let row_zero = load_vec(e, info + 0x130);
    let remaining = sub(adjusted, mul(splat(normal), row_zero));
    store_vec(e, velocity, remaining);
    let u1 = f64::from(x87_quiet(e.mem.f32(velocity + 4)));
    let u2 = f64::from(x87_quiet(e.mem.f32(velocity + 8)));
    let diagonal = f64::from(x87_quiet(e.mem.f32(info + 0x158)));
    let cross_term = f64::from(e.mem.f32(info + 0x154));
    let minor_inverse = f64::from(e.mem.f32(info + 0x14c));
    let mut slide_first = ((diagonal * u1 - u2 * cross_term) * minor_inverse) as f32;
    let middle = f64::from(x87_quiet(e.mem.f32(info + 0x144)));
    let mut slide_second = ((u2 * middle - u1 * cross_term) * minor_inverse) as f32;
    let friction_squared = (f64::from(slide_second) * f64::from(slide_second)
        + f64::from(slide_first) * f64::from(slide_first)) as f32;
    let p0 = x87_quiet(e.mem.f32(params));
    let limited = (f64::from(p0) * f64::from(normal)) as f32;
    let limit_squared = (f64::from(limited) * f64::from(limited)) as f32;
    // Past the cone the slide is scaled back onto it.
    if limit_squared < friction_squared {
        let ratio = (f64::from(limit_squared) / f64::from(friction_squared)) as f32;
        let scale = e.call(X87_SQRT, &args![f64::from(ratio)]).f64() as f32;
        slide_first = (f64::from(scale) * f64::from(slide_first)) as f32;
        slide_second = (f64::from(scale) * f64::from(slide_second)) as f32;
    }
    e.mem.set_f32(impulse + 4, x87_quiet(slide_first));
    e.mem.set_f32(impulse + 8, x87_quiet(slide_second));
    let row_one = load_vec(e, info + 0x140);
    let row_two = load_vec(e, info + 0x150);
    let remaining = load_vec(e, velocity);
    let remaining = sub(remaining, mul(splat(slide_first), row_one));
    let remaining = sub(remaining, mul(splat(slide_second), row_two));
    store_vec(e, velocity, remaining);
    let adjusted_first = f64::from(x87_quiet(e.mem.f32(velocity)));
    let mut second_normal =
        ((adjusted_first - f64::from(friction_term)) * f64::from(e.mem.f32(info + 0x13c))) as f32;
    if f64::from(second_normal) < e.mem.f64(FRICTION_LIMIT) {
        second_normal = 0.0;
    }
    if f64::from(second_normal) + f64::from(normal) > f64::from(p2) {
        e.mem.set_u8(params + 0x18, 1);
        second_normal = (f64::from(p2) - f64::from(normal)) as f32;
    }
    let total = (f64::from(normal) + f64::from(second_normal)) as f32;
    e.mem.set_f32(impulse, total);
    let remaining = sub(load_vec(e, velocity), mul(splat(second_normal), row_zero));
    store_vec(e, velocity, remaining);
    let u2 = f64::from(x87_quiet(e.mem.f32(velocity + 8)));
    let u1 = f64::from(x87_quiet(e.mem.f32(velocity + 4)));
    let rest_squared = (u2 * u2 + u1 * u1) as f32;
    let tangent = x87_quiet(tangent_squared);
    let chosen = if tangent > rest_squared {
        rest_squared
    } else {
        tangent
    };
    let root = e.call(X87_SQRT, &args![f64::from(chosen)]).f64() as f32;
    let extra =
        (f64::from(root) * f64::from(e.mem.f32(info + 0x13c)) * f64::from(e.mem.f32(params + 0x14))
            + f64::from(e.mem.f32(impulse))) as f32;
    e.mem.set_f32(impulse, extra);
    let reloaded = x87_quiet(e.mem.f32(impulse));
    if p2 < reloaded {
        e.mem.set_u8(params + 0x18, 1);
        e.mem.set_f32(impulse, x87_quiet(p2));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00df_5950,
            hk_simple_constraint_util_init_info(u32, u32, u32, u32)
        ),
        entry!(0x00df_5d20, fn_00df5d20(u32, u32, u32, u32)),
        entry!(
            0x00df_5e80,
            hk_simple_constraint_util_get_point_velocity(u32, u32, u32, u32)
        ),
        entry!(
            0x00df_5f80,
            hk_simple_constraint_util_collide(u32, u32, u32, u32) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ZERO: Vec4 = [0.0; 4];
    const ROW_X: Vec4 = [1.0, 0.0, 0.0, 0.0];
    const ROW_Y: Vec4 = [0.0, 1.0, 0.0, 0.0];
    const ROW_Z: Vec4 = [0.0, 0.0, 1.0, 0.0];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(KEEP_XYZ_MASK + 4 * lane, 0xffff_ffff);
        }
        e.mem.set_u32(KEEP_XYZ_MASK + 12, 0);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-5,
            "{actual} is not near {expected}"
        );
    }

    fn near_vec(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            near(actual[lane], expected[lane]);
        }
    }

    /// The floors and limits of the exe, with the values it ships.
    fn real_constants(e: &mut Engine) {
        e.mem.set_u32(MASS_FLOOR, 0x2d2f_ebff);
        e.mem.set_u32(DETERMINANT_FLOOR, 0x08a6_274c);
        e.mem.set_f64(DIAGONAL_LIMIT, 1.0e-11);
        e.mem.set_f64(MINOR_LIMIT, 1.0e-22);
        e.mem.set_u32(MINOR_FLOOR, 0x1af1_c900);
    }

    /// A body with the position `position`, a tensor of `scale` times the identity, the
    /// vector `(7, 8, 9, 10)` at `+0x40`, the inverse mass `mass` and the word `word`.
    fn body(e: &mut Engine, position: Vec4, mass: f32, word: u32) -> u32 {
        let body = e.mem.alloc(0x60);
        store_vec(e, body, position);
        store_vec(e, body + 0x10, ROW_X);
        store_vec(e, body + 0x20, ROW_Y);
        store_vec(e, body + 0x30, ROW_Z);
        store_vec(e, body + 0x40, [7.0, 8.0, 9.0, 10.0]);
        e.mem.set_f32(body + 0x4c, mass);
        e.mem.set_u32(body + 0x50, word);
        body
    }

    fn identity_rotation(e: &mut Engine) -> u32 {
        let rotation = e.mem.alloc(0x30);
        store_vec(e, rotation, ROW_X);
        store_vec(e, rotation + 0x10, ROW_Y);
        store_vec(e, rotation + 0x20, ROW_Z);
        rotation
    }

    #[test]
    fn init_info_with_bodies_at_the_origin_gives_a_scaled_identity() {
        let mut e = engine();
        real_constants(&mut e);
        let a = body(&mut e, ZERO, 1.0, 0xaaaa);
        let b = body(&mut e, ZERO, 1.0, 0xbbbb);
        let rotation = identity_rotation(&mut e);
        let info = e.mem.alloc(0x200);
        e.call(0x00df_5950, &args![a, b, rotation, info]);
        // The rotation is copied; the diagonal is the sum of the two inverse masses.
        assert_eq!(load_vec(&e, info + 0x100), ROW_X);
        assert_eq!(load_vec(&e, info + 0x120), ROW_Z);
        assert_eq!(e.mem.f32(info + 0x130), 2.0);
        assert_eq!(e.mem.f32(info + 0x158), 2.0);
        // The inverse matrix, the reciprocal of the diagonal and of the minor.
        assert_eq!(load_vec(&e, info + 0x160), [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, info + 0x170), [0.0, 0.5, 0.0, 0.0]);
        assert_eq!(load_vec(&e, info + 0x180), [0.0, 0.0, 0.5, 0.0]);
        assert_eq!(e.mem.f32(info + 0x13c), 0.5);
        assert_eq!(e.mem.f32(info + 0x14c), 0.25);
        // Each body's section: its vector at +0x60, 1 / mass at +0x70 and the word at +0x74.
        // The fourth lane of the copied vector is the inverse mass (`body[+0x4c]`).
        assert_eq!(load_vec(&e, info + 0x60), [7.0, 8.0, 9.0, 1.0]);
        assert_eq!(load_vec(&e, info + 0xe0), [7.0, 8.0, 9.0, 1.0]);
        assert_eq!(e.mem.f32(info + 0x70), 1.0);
        assert_eq!(e.mem.f32(info + 0xf0), 1.0);
        assert_eq!(e.mem.u32(info + 0x74), 0xaaaa);
        assert_eq!(e.mem.u32(info + 0xf4), 0xbbbb);
    }

    #[test]
    fn init_info_adds_the_cross_product_terms_of_an_offset_body() {
        let mut e = engine();
        real_constants(&mut e);
        let a = body(&mut e, [1.0, 0.0, 0.0, 0.0], 1.0, 1);
        let b = body(&mut e, ZERO, 1.0, 2);
        let rotation = identity_rotation(&mut e);
        let info = e.mem.alloc(0x200);
        e.call(0x00df_5950, &args![a, b, rotation, info]);
        // The first body's cross rows are (0, 0, 0), (0, 0, 1), (0, -1, 0); transposed they
        // turn the tensor products into the matrix diag(2, 3, 3).
        assert_eq!(load_vec(&e, info + 0x10)[2], -1.0);
        assert_eq!(load_vec(&e, info + 0x20)[1], 1.0);
        assert_eq!(e.mem.f32(info + 0x130), 2.0);
        assert_eq!(e.mem.f32(info + 0x144), 3.0);
        assert_eq!(e.mem.f32(info + 0x158), 3.0);
        near_vec(load_vec(&e, info + 0x160), [0.5, 0.0, 0.0, 0.0]);
        near_vec(load_vec(&e, info + 0x170), [0.0, 1.0 / 3.0, 0.0, 0.0]);
        near_vec(load_vec(&e, info + 0x180), [0.0, 0.0, 1.0 / 3.0, 0.0]);
        near(e.mem.f32(info + 0x13c), 0.5);
        near(e.mem.f32(info + 0x14c), 1.0 / 9.0);
    }

    #[test]
    fn init_info_falls_back_to_the_floors() {
        let mut e = engine();
        e.mem.set_f32(MASS_FLOOR, 0.25);
        e.mem.set_f32(DETERMINANT_FLOOR, 0.5);
        e.mem.set_f64(DIAGONAL_LIMIT, 0.125);
        e.mem.set_f64(MINOR_LIMIT, 0.5);
        e.mem.set_f32(MINOR_FLOOR, 0.2);
        let a = body(&mut e, ZERO, 0.0, 1);
        let b = body(&mut e, ZERO, 0.0, 2);
        let rotation = identity_rotation(&mut e);
        let info = e.mem.alloc(0x200);
        e.call(0x00df_5950, &args![a, b, rotation, info]);
        // Zero masses: the mass floor's reciprocal, the diagonal's replacement by the floor,
        // the determinant floor (the inverse matrix is zero) and the minor floor.
        assert_eq!(e.mem.f32(info + 0x70), 4.0);
        assert_eq!(e.mem.f32(info + 0xf0), 4.0);
        assert_eq!(load_vec(&e, info + 0x160), ZERO);
        assert_eq!(e.mem.f32(info + 0x13c), 4.0);
        near(e.mem.f32(info + 0x14c), 5.0);
    }

    #[test]
    fn apply_impulse_moves_both_records_and_clears_the_fourth_lanes() {
        let mut e = engine();
        let info = e.mem.alloc(0x200);
        store_vec(&mut e, info + 0x100, ROW_X);
        store_vec(&mut e, info + 0x110, ROW_Y);
        store_vec(&mut e, info + 0x120, ROW_Z);
        store_vec(&mut e, info + 0x60, [0.0, 0.0, 0.0, 2.0]);
        store_vec(&mut e, info + 0xe0, [0.0, 0.0, 0.0, 3.0]);
        for (base, scale) in [(info + 0x30, 2.0f32), (info + 0xb0, 3.0)] {
            store_vec(&mut e, base, ROW_X.map(|lane| lane * scale));
            store_vec(&mut e, base + 0x10, ROW_Y.map(|lane| lane * scale));
            store_vec(&mut e, base + 0x20, ROW_Z.map(|lane| lane * scale));
        }
        let vector = e.mem.alloc(0x10);
        store_vec(&mut e, vector, [1.0, 2.0, 3.0, 9.0]);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        for record in [first, second] {
            store_vec(&mut e, record, [10.0; 4]);
            store_vec(&mut e, record + 0x10, [1.0; 4]);
        }
        e.call(0x00df_5d20, &args![info, vector, first, second]);
        assert_eq!(load_vec(&e, first), [12.0, 14.0, 16.0, 0.0]);
        assert_eq!(load_vec(&e, second), [7.0, 4.0, 1.0, 0.0]);
        assert_eq!(load_vec(&e, first + 0x10), [3.0, 5.0, 7.0, 0.0]);
        assert_eq!(load_vec(&e, second + 0x10), [-2.0, -5.0, -8.0, 0.0]);
    }

    #[test]
    fn point_velocity_combines_the_linear_and_angular_parts() {
        let mut e = engine();
        let info = e.mem.alloc(0x200);
        store_vec(&mut e, info + 0x100, [1.0, 2.0, 3.0, 10.0]);
        store_vec(&mut e, info + 0x110, [4.0, 5.0, 6.0, 11.0]);
        store_vec(&mut e, info + 0x120, [7.0, 8.0, 9.0, 12.0]);
        store_vec(&mut e, info, ROW_X);
        store_vec(&mut e, info + 0x10, ROW_Y);
        store_vec(&mut e, info + 0x20, ROW_Z);
        store_vec(&mut e, info + 0x80, ROW_X.map(|lane| lane * 2.0));
        store_vec(&mut e, info + 0x90, ROW_Y.map(|lane| lane * 2.0));
        store_vec(&mut e, info + 0xa0, ROW_Z.map(|lane| lane * 2.0));
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        store_vec(&mut e, first, [1.0, 1.0, 1.0, 0.0]);
        store_vec(&mut e, first + 0x10, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, second + 0x10, [0.5, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00df_5e80, &args![info, first, second, out]);
        // The transposed rotation's columns (1, 4, 7, 8), (2, 5, 8, 12), (3, 6, 9, 12) times
        // the difference (1, 1, 1), plus (1, 2, 3) - (1, 0, 0).
        assert_eq!(load_vec(&e, out), [6.0, 17.0, 27.0, 32.0]);
    }

    /// The sphere / friction scene: identity rotation and inverse matrices, an angular part of
    /// zero, the flag byte set, and the parameter block `[p0, p1, p2, p3, p4, p5]`.
    fn collide_scene(
        e: &mut Engine,
        first_linear: Vec4,
        second_linear: Vec4,
        parameters: [f32; 6],
    ) -> (u32, u32, u32, u32) {
        e.mem.set_u8(FLYING_COLORS_FLAG, 1);
        e.register_double(X87_SQRT, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        let info = e.mem.alloc(0x200);
        store_vec(e, info + 0x100, ROW_X);
        store_vec(e, info + 0x110, ROW_Y);
        store_vec(e, info + 0x120, ROW_Z);
        store_vec(e, info + 0x160, ROW_X);
        store_vec(e, info + 0x170, ROW_Y);
        store_vec(e, info + 0x180, ROW_Z);
        // The accumulated matrix rows, their fourth lanes the reciprocals the exe keeps there.
        store_vec(e, info + 0x130, [1.0, 0.0, 0.0, 1.0]);
        store_vec(e, info + 0x140, [0.0, 1.0, 0.0, 1.0]);
        store_vec(e, info + 0x150, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, info + 0x60, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, info + 0xe0, [0.0, 0.0, 0.0, 1.0]);
        let params = e.mem.alloc(0x20);
        for (index, value) in parameters.into_iter().enumerate() {
            e.mem.set_f32(params + 4 * index as u32, value);
        }
        e.mem.set_u8(params + 0x18, 0xaa);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        store_vec(e, first, first_linear);
        store_vec(e, second, second_linear);
        (info, params, first, second)
    }

    #[test]
    fn collide_returns_zero_for_a_separating_pair_past_the_limit() {
        let mut e = engine();
        let (info, params, first, second) = collide_scene(
            &mut e,
            [5.0, 1.0, 2.0, 0.0],
            ZERO,
            [2.0, 0.5, 10.0, 3.0, 2.0, 1.0],
        );
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        assert_eq!(result.f32(), 0.0);
        assert_eq!(load_vec(&e, first), [5.0, 1.0, 2.0, 0.0]);
        assert_eq!(e.mem.u8(params + 0x18), 0xaa);
    }

    #[test]
    fn collide_applies_the_plain_impulse_inside_the_cone() {
        let mut e = engine();
        let (info, params, first, second) = collide_scene(
            &mut e,
            [1.5, 0.5, 0.25, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [2.0, 0.5, 10.0, 3.0, 2.0, 1.0],
        );
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        // v = (1, 0.5, 0.25): the first lane becomes -1 + (2 + 0.5 * -1) = 0.5, the impulse
        // (0.5, -0.5, -0.25).
        assert_eq!(result.f32(), 0.5);
        assert_eq!(load_vec(&e, first), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, second), [0.0, 0.5, 0.25, 0.0]);
        assert_eq!(e.mem.u8(params + 0x18), 0);
        assert_eq!(calls(&e, X87_SQRT).len(), 0);
    }

    #[test]
    fn collide_scales_an_impulse_above_the_limit_and_sets_the_flag() {
        let mut e = engine();
        let (info, params, first, second) = collide_scene(
            &mut e,
            [1.5, 0.5, 0.25, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [2.0, 0.5, 0.25, 3.0, 2.0, 1.0],
        );
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        assert_eq!(result.f32(), 0.25);
        assert_eq!(load_vec(&e, first), [1.75, 0.25, 0.125, 0.0]);
        assert_eq!(e.mem.u8(params + 0x18), 1);
    }

    #[test]
    fn collide_rebuilds_the_impulse_when_it_leaves_the_cone() {
        let mut e = engine();
        let (info, params, first, second) = collide_scene(
            &mut e,
            [-1.5, 1.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [0.1, 0.5, 100.0, 3.0, 0.0, 1.0],
        );
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        // Normal impulse 3, the sliding part (-1, 0) scaled to the cone 0.3 and the residual
        // sqrt(0.49) = 0.7 added to the first lane.
        near(result.f32(), 3.7);
        let first_velocity = load_vec(&e, first);
        near(first_velocity[0], 2.2);
        near(first_velocity[1], 0.7);
        near(first_velocity[2], 0.0);
        let second_velocity = load_vec(&e, second);
        near(second_velocity[0], -3.2);
        near(second_velocity[1], 0.3);
        assert_eq!(e.mem.u8(params + 0x18), 0);
        assert_eq!(calls(&e, X87_SQRT).len(), 2);
    }

    #[test]
    fn collide_limits_the_rebuilt_impulse_to_the_largest_one() {
        let mut e = engine();
        let (info, params, first, second) = collide_scene(
            &mut e,
            [-1.5, 1.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [0.1, 0.5, 3.5, 3.0, 0.0, 1.0],
        );
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        assert_eq!(result.f32(), 3.5);
        assert_eq!(e.mem.u8(params + 0x18), 1);
        assert_eq!(load_vec(&e, first)[0], -1.5 + 3.5);
    }

    #[test]
    fn collide_runs_the_initialiser_once_and_gives_up_if_it_does_not_help() {
        let mut e = engine();
        let (info, params, first, second) = collide_scene(
            &mut e,
            [1.5, 0.5, 0.25, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [2.0, 0.5, 10.0, 3.0, 2.0, 1.0],
        );
        e.mem.set_u8(FLYING_COLORS_FLAG, 0);
        let attempts = Rc::new(RefCell::new(0u32));
        let counted = Rc::clone(&attempts);
        e.register_double(INIT_FLYING_COLORS, move |_, _| {
            *counted.borrow_mut() += 1;
            Ret::default()
        });
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        assert_eq!(result.f32(), 0.0);
        assert_eq!(*attempts.borrow(), 1);
        assert_eq!(
            calls(&e, INIT_FLYING_COLORS),
            vec![vec![FLYING_COLORS_FLAG]]
        );
        assert_eq!(load_vec(&e, first), [1.5, 0.5, 0.25, 0.0]);
        // When the initialiser sets the flag the call goes on.
        e.register_double(INIT_FLYING_COLORS, |e, _| {
            e.mem.set_u8(FLYING_COLORS_FLAG, 1);
            Ret::default()
        });
        let result = e.call(0x00df_5f80, &args![info, params, first, second]);
        assert_eq!(result.f32(), 0.5);
    }
}
