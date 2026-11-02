//! DirectSound's software 3D stage, as the player's `dsound.dll` computes it for a 3D buffer on
//! a stereo device (`DS3DALG_NO_VIRTUALIZATION`, which is also what `DS3DALG_HRTF_LIGHT` falls
//! back to, measured: `DS_NO_VIRTUALIZATION`). Translated from the instructions of the 32-bit
//! `dsound.dll` the game loads (`SysWOW64`, Windows 11 10.0.26200; read in a private analysis
//! with the maintainer's leave, 2026-10-28; addresses below are that DLL's, image base
//! `0x10000000`), and checked against what the DLL sends to the device (`scripts/dsmix-measure`,
//! docs/SOUND.md). No bytes of the DLL are here: its tables are computed from the formulas they
//! were made with.
//!
//! What the stage does for one buffer (`C3dObject` and `CMultiPan3dObject` in the DLL's log
//! strings):
//! 1. The source's position in the listener's frame ([`to_listener`]): the offset from the
//!    listener turned by three rotations whose angles come from the listener's orientation
//!    ([`Angles`]). The rotations use a sine table in whole degrees (the angle is truncated), and
//!    the angles come from an arcsine table in steps of 0.01 (0.001 above 0.9).
//! 2. A distance factor `min / ((d - min) * rolloff + min)` past the minimum distance, with `d`
//!    held at the maximum distance ([`distance_factor`]). As millibels:
//!    `trunc(602.059991328 * log2(f)) - 500` ([`distance_millibels`]), so a source is always at
//!    least 5 dB down.
//! 3. A pan between the two nearest speakers (left at -pi/2, right at +pi/2, as `f32` values)
//!    by the tangent law, from the azimuth `atan2(x, z)`. Closer than 2.5 units (`|x|` and `|z|`
//!    under 2.5 and `sqrt(x^2 + z^2)` under 2.5, whatever the minimum distance) a part
//!    `0.5 + r / 5` is panned from the azimuth and the rest from the opposite direction. Height
//!    counts only in the distance. Each speaker's share becomes `trunc(301.03 * log2(share))`
//!    millibels ([`pan_millibels`]).
//! 4. The device mixes each channel at `gain(volume + distance) * gain(pan)`, two lookups in the
//!    volume table (`super::mixer::gain`) multiplied as `f32` ([`channel_gains`]).
//! 5. Doppler: with a velocity on the buffer or the listener, the frequency becomes
//!    `f * 359660 / (v + 359660)` for the radial speed `v` in mm/s ([`doppler_frequency`]).
//!
//! The angle stage's `atan2` and logarithms are the x87 unit's in the DLL; here they are the
//! `f64` ones. They agree to the last bit of an `f64`, so a truncation can differ only when a
//! product falls within about 1e-13 of a whole millibel.

use std::sync::OnceLock;

/// `f32` constants the DLL computes with (each the `f32` nearest the value).
const PI2: f32 = std::f32::consts::FRAC_PI_2;
const PI: f32 = std::f32::consts::PI;
const PI3_2: f32 = 3.0 * std::f32::consts::FRAC_PI_2;
const TAU: f32 = std::f32::consts::TAU;
/// `-180 / pi` as `f32` (`0xc2652ee1`): radians to (negated) degrees for the table index.
const DEGREES: f32 = (-180.0 / std::f64::consts::PI) as f32;

/// The DLL's trigonometric tables: `sin(k degrees)` for `k = 0..=90`, `asin(k / 100)` for
/// `k = 0..=90` and `asin(0.9 + k / 1000)` for `k = 0..=100`, each rounded to 6 decimals and
/// then to `f32` (the values match the DLL's tables entry for entry; checked privately).
struct Tables {
    sin: [f32; 91],
    asin_coarse: [f32; 91],
    asin_fine: [f32; 101],
}

/// `v` rounded to 6 decimals, as the `f32` nearest that decimal.
fn decimals6(v: f64) -> f32 {
    let n = (v * 1e6).round() as i64;
    let (sign, n) = if n < 0 { ("-", -n) } else { ("", n) };
    format!("{sign}{}.{:06}", n / 1_000_000, n % 1_000_000)
        .parse()
        .expect("a decimal")
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| Tables {
        sin: std::array::from_fn(|k| decimals6((k as f64).to_radians().sin())),
        asin_coarse: std::array::from_fn(|k| decimals6((k as f64 / 100.0).asin())),
        asin_fine: std::array::from_fn(|k| decimals6((0.9 + k as f64 / 1000.0).asin())),
    })
}

/// `cvttss2si`: truncation, `i32::MIN` when out of range or NaN.
fn truncate(v: f32) -> i32 {
    if v.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&v) {
        i32::MIN
    } else {
        v as i32
    }
}

/// The table entry for `a` radians (0 to pi/2): the sine table at `a` in whole degrees, the
/// fraction dropped (`-trunc(a * -57.29578)`).
fn sin_entry(a: f32, from_end: bool) -> f32 {
    let k = -truncate(a * DEGREES);
    let k = if from_end { 90 - k } else { k };
    tables().sin[k as usize]
}

/// The angle brought into 0..2pi by whole turns, as the DLL's loops do.
fn wrap(mut a: f32) -> f32 {
    while 0.0 > a {
        a += TAU;
    }
    while a > TAU {
        a -= TAU;
    }
    a
}

/// The DLL's table sine (`10026e61`).
pub fn sin(a: f32) -> f32 {
    let a = wrap(a);
    if a > PI3_2 {
        -sin_entry(PI2 - (a - PI3_2), false)
    } else if a > PI {
        -sin_entry(a - PI, false)
    } else if a > PI2 {
        sin_entry(PI2 - (a - PI2), false)
    } else {
        sin_entry(a, false)
    }
}

/// The DLL's table cosine (`10026d9a`).
pub fn cos(a: f32) -> f32 {
    let a = wrap(a);
    if a > PI3_2 {
        sin_entry(PI2 - (a - PI3_2), true)
    } else if a > PI {
        -sin_entry(a - PI, true)
    } else if a > PI2 {
        -sin_entry(PI2 - (a - PI2), true)
    } else {
        sin_entry(a, true)
    }
}

/// The DLL's table arcsine (`10026d06`): above 1 in size, pi/2 whatever the sign.
pub fn asin(x: f32) -> f32 {
    let (x, negative) = if 0.0 > x {
        (x * -1.0, true)
    } else {
        (x, false)
    };
    if x > 1.0 {
        return PI2;
    }
    let t = tables();
    let v = if 0.9f32 > x {
        let k = -(((f64::from(x) + 0.005) * -100.0) as i32);
        t.asin_coarse[k as usize]
    } else {
        let k = -((((f64::from(x) - 0.9) + 0.0005) * -1000.0) as i32);
        t.asin_fine[k as usize]
    };
    if negative {
        v * -1.0
    } else {
        v
    }
}

/// A vector's length (`10026c22`): the squares added in `f64` (y, x, then z), its root as `f32`.
pub fn length(v: [f32; 3]) -> f32 {
    let [x, y, z] = v.map(f64::from);
    ((y * y + x * x) + z * z).sqrt() as f32
}

fn is_zero(v: [f32; 3]) -> bool {
    v.iter().all(|&c| c == 0.0)
}

/// A vector longer than `f32::MAX` shortened to 0.99 of it (`10026876`), as the DLL does to
/// every position, velocity and orientation it is given.
pub fn clamp_length(v: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = v.map(f64::from);
    let n = ((y * y + x * x) + z * z).sqrt();
    if n > f64::from(f32::MAX) {
        let s = (3.368_795_231_721_435_6e38 / n) as f32;
        [s * v[0], s * v[1], s * v[2]]
    } else {
        v
    }
}

/// `v` made unit length by multiplying with `1 / length` (`10026cae`); `None` for a zero vector.
pub fn normalize(v: [f32; 3]) -> Option<[f32; 3]> {
    let n = length(v);
    if n == 0.0 {
        return None;
    }
    let s = 1.0f32 / n;
    Some([s * v[0], s * v[1], s * v[2]])
}

/// `a x b` in `f32` (`100268f9`).
pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - b[1] * a[2],
        a[2] * b[0] - a[0] * b[2],
        b[1] * a[0] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    (a[1] * b[1] + a[0] * b[0]) + a[2] * b[2]
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// The listener's orientation as `SetOrientation` and `SetAllParameters` store it: both vectors
/// made unit length, then the top replaced by `(front x top) x front` made unit length.
/// `None` when either is zero or they are parallel (the call answers `DSERR_INVALIDPARAM`).
pub fn orientation(front: [f32; 3], top: [f32; 3]) -> Option<([f32; 3], [f32; 3])> {
    let front = normalize(clamp_length(front))?;
    let top = normalize(clamp_length(top))?;
    let side = cross(front, top);
    if is_zero(side) {
        return None;
    }
    let up = cross(side, front);
    if is_zero(up) {
        return None;
    }
    Some((front, normalize(up).unwrap_or(up)))
}

/// Turns `v` by `a` about z (`10027028`).
fn turn_xy(v: [f32; 3], a: f32) -> [f32; 3] {
    let (s, c) = (sin(a), cos(a));
    [v[0] * c - v[1] * s, v[1] * c + v[0] * s, v[2]]
}

/// Turns `v` by `a` about x (`10026f77`).
fn turn_yz(v: [f32; 3], a: f32) -> [f32; 3] {
    let (s, c) = (sin(a), cos(a));
    [v[0], v[1] * c - v[2] * s, v[2] * c + v[1] * s]
}

/// Turns `v` by `a` about y (`10026fd1`).
fn turn_xz(v: [f32; 3], a: f32) -> [f32; 3] {
    let (s, c) = (sin(a), cos(a));
    [v[0] * c + v[2] * s, v[1], v[2] * c - v[0] * s]
}

/// A vector's length, azimuth `atan2(y, x)` (0..2pi; on the y axis exactly +-pi/2) and
/// elevation `asin(z / length)` from the table (`1002677b`).
fn spherical(v: [f32; 3]) -> (f32, f32, f32) {
    if is_zero(v) {
        return (0.0, 0.0, 0.0);
    }
    let r = length(v);
    if r == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let elevation = asin(v[2] / r);
    let mut azimuth = if v[0] == 0.0 {
        if v[1] < 0.0 {
            -PI2
        } else {
            PI2
        }
    } else {
        f64::from(v[1]).atan2(f64::from(v[0])) as f32
    };
    if 0.0 > azimuth {
        azimuth += TAU;
    }
    (r, azimuth, elevation)
}

/// The three angles the listener's orientation turns positions by (`10026b38`, kept per 3D
/// object at `+0xc8`, `+0xcc`, `+0xd0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angles {
    pub elevation: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Angles {
    /// From the listener's (stored) front and top vectors.
    pub fn of(front: [f32; 3], top: [f32; 3]) -> Angles {
        let (_, azimuth, elevation) = spherical(top);
        let roll = azimuth - PI2;
        let f = turn_xy(front, roll * -1.0);
        let f = turn_yz(f, elevation * -1.0);
        let yaw = if f[0] < 0.0 {
            asin(f[2]) - PI2
        } else {
            PI2 - asin(f[2])
        };
        Angles {
            elevation,
            yaw,
            roll,
        }
    }
}

/// A position in the listener's frame (`10026a0d`): the offset from the listener turned by the
/// three angles; an offset of zero stays zero.
pub fn to_listener(position: [f32; 3], listener: [f32; 3], angles: Angles) -> [f32; 3] {
    let v = sub(position, listener);
    if is_zero(v) {
        return v;
    }
    let v = turn_xy(v, angles.roll * -1.0);
    let v = turn_yz(v, angles.elevation * -1.0);
    turn_xz(v, angles.yaw * -1.0)
}

/// The distance factor (`10029f70`) for a source `distance` away: `d` held at `max`
/// (`beyond_max` then true), `min / ((d - min) * rolloff + min)` past `min` with a rolloff above
/// 0, else 1.
pub fn distance_factor(distance: f32, min: f32, max: f32, rolloff: f32) -> (f32, bool) {
    let (d, beyond) = if distance < max {
        (distance, false)
    } else {
        (max, true)
    };
    if d > min && rolloff > 0.0 {
        (min / ((d - min) * rolloff + min), beyond)
    } else {
        (1.0, beyond)
    }
}

/// The distance attenuation (`1002a3fb`): `trunc(602.059991328 * log2(f)) - 500` millibels for
/// the distance factor `f` (times the cone's, 1 without cones), -10000 for nothing left or when
/// the buffer mutes beyond its maximum distance.
pub fn distance_millibels(factor: f32, muted_beyond_max: bool) -> i32 {
    let f = factor * 1.0f32;
    if muted_beyond_max || !(f64::from(f) > 0.0) {
        return -10000;
    }
    (602.059_991_328f64 * f64::from(f).log2()) as i32 - 500
}

/// The two speakers' azimuths of a stereo device (`10007730`: -pi/2 and +pi/2 as `f32`).
const SPEAKERS: [f64; 2] = [-(PI2 as f64), PI2 as f64];

/// Adds `weight` to the two speakers nearest `azimuth` by the tangent law (`1002a60d`).
fn pan_to(shares: &mut [f64; 2], azimuth: f64, weight: f64) {
    let (pi, tau, half) = (f64::from(PI), f64::from(TAU), f64::from(PI2));
    let (mut below, mut below_at) = (-4.0f64, None);
    let (mut above, mut above_at) = (4.0f64, None);
    for (i, &s) in SPEAKERS.iter().enumerate() {
        let mut d = azimuth - s;
        if d > pi {
            d -= tau;
        } else if -pi > d {
            d += tau;
        }
        if 0.0 < d {
            if above > d {
                above = d;
                above_at = Some(i);
            }
        } else if d > below {
            below = d;
            below_at = Some(i);
        }
    }
    let (a, b) = (below.abs(), above.abs());
    let (ia, ib) = match (below_at, above_at) {
        (Some(ia), Some(ib)) => (ia, ib),
        _ => (0, 0),
    };
    if 1e-5 > a {
        shares[ia] += weight;
    } else if 1e-5 > b {
        shares[ib] += weight;
    } else {
        let t = (a * half / (b + a)).tan();
        let g = weight / (t + 1.0);
        shares[ia] += g;
        shares[ib] += t * g;
    }
}

/// Each channel's pan attenuation (millibels) for a source at `local` in the listener's frame
/// (`1002a18b`).
pub fn pan_millibels(local: [f32; 3]) -> [i32; 2] {
    let (x, z) = (local[0], local[2]);
    let azimuth = f64::from(x).atan2(f64::from(z));
    let (ax, az) = (x.abs(), z.abs());
    let mut shares = [0f64; 2];
    if 1e-5 > f64::from(ax) && 1e-5 > f64::from(az) {
        shares = [1.0 / 2.0; 2];
    } else {
        let (mut toward, mut weight) = (azimuth, 1.0);
        if 2.5f32 > ax && 2.5f32 > az {
            let r = f64::from(az * az + ax * ax).sqrt();
            if 2.5 > r {
                let near = r / 5.0 + 0.5;
                pan_to(&mut shares, azimuth, near);
                toward = azimuth + f64::from(PI);
                weight = 1.0 - near;
            }
        }
        pan_to(&mut shares, toward, weight);
    }
    shares.map(|s| {
        if s == 0.0 {
            -10000
        } else {
            (301.03f64 * s.log2()) as i32
        }
    })
}

/// The `f32` multipliers the device mixes the left and right channels with
/// (`10057be0`): `gain(volume) * gain(pan)` for each.
pub fn channel_gains(volume: i32, pan: [i32; 2]) -> (f32, f32) {
    let g = super::mixer::gain;
    let v = g(volume);
    (g(pan[0]) * v, g(pan[1]) * v)
}

/// What the 3D stage needs of the listener.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Listener {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub front: [f32; 3],
    pub top: [f32; 3],
    pub distance_factor: f32,
    pub rolloff: f32,
    pub doppler: f32,
}

/// What the 3D stage needs of a buffer (`DS3DMODE_NORMAL`, no cones).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Source {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub min: f32,
    pub max: f32,
    /// `DSBCAPS_MUTE3DATMAXDISTANCE`.
    pub mute_at_max: bool,
    /// The buffer's volume (millibels).
    pub volume: i32,
    /// The buffer's frequency (Hz).
    pub frequency: u32,
}

/// What the device is given for a 3D buffer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Heard {
    /// The left and right multipliers; (0, 0) when muted.
    pub gains: (f32, f32),
    /// The frequency after Doppler (Hz).
    pub frequency: u32,
}

/// The Doppler-shifted frequency (`10029c30`, `100269c9`, clamped to 100..200000 by the
/// device, `100581e0`); the buffer's own when neither moves or the factor is 0.
pub fn doppler_frequency(s: &Source, l: &Listener) -> u32 {
    if !(l.doppler > 0.0) || (is_zero(l.velocity) && is_zero(s.velocity)) {
        return s.frequency;
    }
    let d = sub(s.position, l.position);
    let speed = if is_zero(d) {
        0.0
    } else {
        dot(d, sub(s.velocity, l.velocity)) / length(d)
    };
    let v = f64::from(speed) * f64::from(l.distance_factor) * 1000.0 * f64::from(l.doppler);
    let v = if -f64::from(f32::MAX) > v {
        f32::MIN
    } else if v > f64::from(f32::MAX) {
        f32::MAX
    } else {
        v as f32
    };
    // MINSS / MAXSS: the second operand when either is NaN.
    let v = if 179_830.0f32 < v { 179_830.0 } else { v };
    let v = if -179_830.0f32 > v { -179_830.0 } else { v };
    let f = s.frequency as f64 as f32;
    let shifted = (f * 359_660.0) / (v + 359_660.0);
    let hz = if shifted.is_nan() || !(-9.2e18..9.2e18).contains(&shifted) {
        0
    } else {
        shifted as i64 as u32
    };
    hz.clamp(100, 200_000)
}

/// The 3D stage for one buffer: its channel multipliers and frequency.
pub fn hear(s: &Source, l: &Listener) -> Heard {
    let angles = Angles::of(l.front, l.top);
    let local = to_listener(s.position, l.position, angles);
    let distance = if is_zero(local) { 0.0 } else { length(local) };
    let (factor, beyond) = distance_factor(distance, s.min, s.max, l.rolloff);
    let volume = distance_millibels(factor, s.mute_at_max && beyond) + s.volume;
    let gains = if volume < -9999 {
        (0.0, 0.0)
    } else {
        channel_gains(volume, pan_millibels(local))
    };
    Heard {
        gains,
        frequency: doppler_frequency(s, l),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)] // Table values, not the constants.
    fn the_tables_follow_their_formulas() {
        let t = tables();
        assert_eq!(t.sin[0], 0.0);
        assert_eq!(t.sin[1].to_bits(), 0x3c8e_f77f); // 0.017452
        assert_eq!(t.sin[30], 0.5);
        assert_eq!(t.sin[90], 1.0);
        assert_eq!(t.asin_coarse[1], 0.01);
        assert_eq!(t.asin_coarse[90], t.asin_fine[0]);
        assert_eq!(t.asin_fine[100], 1.570_796);
        assert_eq!(DEGREES.to_bits(), 0xc265_2ee1);
    }

    #[test]
    #[allow(clippy::approx_constant)] // Table values, not the constants.
    fn table_trigonometry_truncates_to_whole_degrees() {
        // 10.9 degrees reads the 10 degree entry.
        assert_eq!(
            sin(10.9f32.to_radians()),
            decimals6(10f64.to_radians().sin())
        );
        assert_eq!(
            cos(10.9f32.to_radians()),
            decimals6(80f64.to_radians().sin())
        );
        // -0.5 rad is -28.6 degrees: the 28 degree entry, negated.
        assert_eq!(sin(-0.5), -decimals6(28f64.to_radians().sin()));
        assert_eq!(asin(0.5), 0.523_599);
        assert_eq!(asin(-0.5), -0.523_599);
        assert_eq!(asin(1.5), PI2);
        assert_eq!(asin(-1.5), PI2);
    }

    /// The default orientation (front +z, top +y) leaves positions as they are.
    #[test]
    fn the_default_orientation_turns_nothing() {
        let (front, top) = orientation([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]).unwrap();
        let a = Angles::of(front, top);
        for p in [[1.0f32, 2.0, 3.0], [-4.0, 0.5, -2.0]] {
            assert_eq!(to_listener(p, [0.0; 3], a), p);
        }
    }

    /// Facing +x (front +x, top +y): a source at +z is on the left.
    #[test]
    fn a_turned_listener_hears_the_source_on_its_side() {
        let (front, top) = orientation([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
        let a = Angles::of(front, top);
        let local = to_listener([0.0, 0.0, 4.0], [0.0; 3], a);
        assert!(local[0] < -3.9 && local[2].abs() < 0.01, "{local:?}");
    }

    /// Points measured by `dsmix-measure` (`dsmix-3d-2026-10-24`, minimum distance 1, rolloff
    /// 1, default listener): each channel's multiplier, bit for bit.
    #[test]
    fn measured_points_are_bit_exact() {
        let listener = Listener {
            position: [0.0; 3],
            velocity: [0.0; 3],
            front: [0.0, 0.0, 1.0],
            top: [0.0, 1.0, 0.0],
            distance_factor: 1.0,
            rolloff: 1.0,
            doppler: 1.0,
        };
        let cases: [([f32; 3], u32, u32); 6] = [
            ([0.0, 0.0, 1.0], 0x3ed0_9516, 0x3ed0_9516),
            ([0.0, 0.0, 4.0], 0x3dd1_912b, 0x3dd1_912b),
            ([-4.0, 0.0, 0.0], 0x3e12_5092, 0),
            ([-4.0, 0.0, -4.0], 0x3dae_4d47, 0x3d61_12e3),
            ([-1.0, 0.0, 0.0], 0x3ef7_e696, 0x3ea1_e8a5),
            ([4.0, 0.0, 4.0], 0x3d61_12e3, 0x3dae_4d47),
        ];
        for (position, l, r) in cases {
            let s = Source {
                position,
                velocity: [0.0; 3],
                min: 1.0,
                max: 1e9,
                mute_at_max: true,
                volume: 0,
                frequency: 48000,
            };
            let h = hear(&s, &listener);
            assert_eq!(
                (h.gains.0.to_bits(), h.gains.1.to_bits()),
                (l, r),
                "{position:?}"
            );
        }
    }

    #[test]
    fn a_moving_source_is_shifted_by_doppler() {
        let l = Listener {
            position: [0.0; 3],
            velocity: [0.0; 3],
            front: [0.0, 0.0, 1.0],
            top: [0.0, 1.0, 0.0],
            distance_factor: 1.0,
            rolloff: 1.0,
            doppler: 1.0,
        };
        let mut s = Source {
            position: [0.0, 0.0, 5.0],
            velocity: [0.0; 3],
            min: 1.0,
            max: 1e9,
            mute_at_max: false,
            volume: 0,
            frequency: 48000,
        };
        assert_eq!(doppler_frequency(&s, &l), 48000);
        // Coming closer at 50 units/s: 48000 * 359660 / (359660 - 50000).
        s.velocity = [0.0, 0.0, -50.0];
        assert_eq!(doppler_frequency(&s, &l), 55750);
    }
}
