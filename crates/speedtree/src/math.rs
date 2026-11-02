//! The small vector and matrix steps SpeedTreeRT grows trees with, each as
//! the game's code works it: in double precision within one statement,
//! stored as floats between statements (the library's code is compiled
//! without optimization, so every C statement ends in a store).
//!
//! Matrices are 3 × 3, row-major, nine floats.

pub type V3 = [f32; 3];
pub type M3 = [f32; 9];

pub const IDENTITY: M3 = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

/// `2π`, `π/2` as the library's constants hold them (π × 2.0 and π × 0.5
/// of the float π, `fb8830`, `fb87f0`).
pub const TWO_PI: f32 = (std::f32::consts::PI as f64 * 2.0) as f32;
pub const HALF_PI: f32 = (std::f32::consts::PI as f64 * 0.5) as f32;
/// Degrees per radian as the code divides by it (`57.29578`, a float).
pub const DEG: f32 = 57.295_78;

#[inline]
pub fn d(x: f32) -> f64 {
    x as f64
}

/// `00b12b50`: `M · v` (each row with `v`).
pub fn mat_vec(m: &M3, v: V3) -> V3 {
    [
        (d(v[0]) * d(m[0]) + d(v[1]) * d(m[1]) + d(v[2]) * d(m[2])) as f32,
        (d(v[0]) * d(m[3]) + d(v[1]) * d(m[4]) + d(v[2]) * d(m[5])) as f32,
        (d(v[0]) * d(m[6]) + d(v[1]) * d(m[7]) + d(v[2]) * d(m[8])) as f32,
    ]
}

/// `00b12890`: `vᵀ · M` (the rows weighted by `v`).
pub fn vec_mat(v: V3, m: &M3) -> V3 {
    [
        (d(m[0]) * d(v[0]) + d(m[3]) * d(v[1]) + d(m[6]) * d(v[2])) as f32,
        (d(m[1]) * d(v[0]) + d(m[4]) * d(v[1]) + d(m[7]) * d(v[2])) as f32,
        (d(m[2]) * d(v[0]) + d(m[5]) * d(v[1]) + d(m[8]) * d(v[2])) as f32,
    ]
}

/// `00b12940`: `A · B`.
pub fn mul(a: &M3, b: &M3) -> M3 {
    let mut out = [0.0f32; 9];
    for r in 0..3 {
        for c in 0..3 {
            out[r * 3 + c] = (d(a[r * 3 + 2]) * d(b[6 + c])
                + d(a[r * 3 + 1]) * d(b[3 + c])
                + d(a[r * 3]) * d(b[c])) as f32;
        }
    }
    out
}

/// `00b12ec0`: rotation by `degrees` about the unit axis `(x, y, z)`.
pub fn rotation(degrees: f32, x: f32, y: f32, z: f32) -> M3 {
    let a = (d(degrees) / d(DEG)) as f32 as f64;
    let s = a.sin() as f32;
    let c = a.cos() as f32;
    let t = 1.0 - c;
    let (s, c, t) = (d(s), d(c), d(t));
    let (x, y, z) = (d(x), d(y), d(z));
    [
        (t * x * x + c) as f32,
        (s * z + t * x * y) as f32,
        (t * x * z - s * y) as f32,
        (t * x * y - s * z) as f32,
        (t * y * y + c) as f32,
        (s * x + t * y * z) as f32,
        (s * y + t * x * z) as f32,
        (t * y * z - s * x) as f32,
        (t * z * z + c) as f32,
    ]
}

/// `00b12d50`: `M ← R(degrees, axis) · M`.
pub fn rotate_axis(m: &mut M3, degrees: f32, axis: V3) {
    let r = rotation(degrees, axis[0], axis[1], axis[2]);
    *m = mul(&r, m);
}

/// `00b12c00`: `M ← E(a, b) · M` with
/// `E = [ca·cb, ca·sb, −sa; −sb, cb, 0; sa·cb, sa·sb, ca]` (degrees).
pub fn rotate_two(m: &mut M3, a: f32, b: f32) {
    let ra = (d(a) / d(DEG)) as f32 as f64;
    let rb = (d(b) / d(DEG)) as f32 as f64;
    let ca = ra.cos() as f32;
    let sa = ra.sin() as f32;
    let cb = rb.cos() as f32;
    let sb = rb.sin() as f32;
    let e = [ca * cb, ca * sb, -sa, -sb, cb, 0.0, sa * cb, sa * sb, ca];
    *m = mul(&e, m);
}

/// `00b134d0`: `M ← P(degrees) · M` with `P = [c, 0, −s; 0, 1, 0; s, 0, c]`.
pub fn pitch(m: &mut M3, degrees: f32) {
    let r = (d(degrees) / d(DEG)) as f32 as f64;
    let c = r.cos() as f32;
    let s = r.sin() as f32;
    let p = [c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c];
    *m = mul(&p, m);
}

/// `00b2a180`.
pub fn dot(a: V3, b: V3) -> f32 {
    (d(a[2]) * d(b[2]) + d(a[1]) * d(b[1]) + d(a[0]) * d(b[0])) as f32
}

/// `00b12810`: `a × b`.
pub fn cross(a: V3, b: V3) -> V3 {
    [
        (d(b[2]) * d(a[1]) - d(b[1]) * d(a[2])) as f32,
        (d(b[0]) * d(a[2]) - d(b[2]) * d(a[0])) as f32,
        (d(b[1]) * d(a[0]) - d(b[0]) * d(a[1])) as f32,
    ]
}

/// `00b12790`: divided by its length (`1 / sqrt`, then each component ×
/// that). A zero vector becomes NaNs, as in the game.
pub fn normalize(v: &mut V3) {
    let len = (d(v[2]) * d(v[2]) + d(v[1]) * d(v[1]) + d(v[0]) * d(v[0])).sqrt() as f32;
    let inv = 1.0 / len;
    v[0] *= inv;
    v[1] *= inv;
    v[2] *= inv;
}

/// `00b12720`: the angle between two unit vectors (radians), the dot
/// product clamped to −1..1.
pub fn angle(a: V3, b: V3) -> f32 {
    let mut c = dot(a, b);
    c = c.clamp(-1.0, 1.0);
    (d(c)).acos() as f32
}

pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn scale(a: V3, s: f32) -> V3 {
    [s * a[0], s * a[1], s * a[2]]
}

/// `00b13000`: `a + (b − a) × t`.
pub fn lerp(a: V3, b: V3, t: f32) -> V3 {
    add(a, scale(sub(b, a), t))
}

/// The library's fast square root (`00b14230`, `00b04d40`): the float's
/// bits halved plus `0x1fc00000`, read back as a float.
pub fn fast_sqrt(x: f32) -> f32 {
    let bits = x.to_bits() as i32;
    f32::from_bits(((bits >> 1) + 0x1fc0_0000) as u32)
}

/// `00b14230`: a vector's length by the fast square root.
pub fn fast_length(v: V3) -> f32 {
    fast_sqrt((d(v[0]) * d(v[0]) + d(v[1]) * d(v[1]) + d(v[2]) * d(v[2])) as f32)
}

/// `00b04d40`: the distance between two points by the fast square root.
pub fn fast_distance(a: V3, b: V3) -> f32 {
    let dx = d(b[0]) - d(a[0]);
    let dy = d(b[1]) - d(a[1]);
    let dz = d(b[2]) - d(a[2]);
    fast_sqrt((dx * dx + dy * dy + dz * dz) as f32)
}

/// The library's `!(a < b)`: true also when either is not a number.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn not_less(a: f32, b: f32) -> bool {
    !(a < b)
}
