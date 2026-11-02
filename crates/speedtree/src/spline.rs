//! SpeedTree's curves (`CIdvBezierSpline`): a text form inside the `.spt`
//! file, read and sampled as the game's SpeedTreeRT does.
//!
//! `BezierSpline <min> <max> <variance> { <n> <x y dx dy length> ... }`
//! (`00b37cc0`, words split at white space, numbers by `atof`). Each point
//! adds (`00b380e0`) itself, its tangent normalized and the tangent's
//! length; between consecutive points the cubic's four control points are
//! `P₀`, `P₀ + T₀·L₀`, `P₁ − T₁·L₁`, `P₁`. The curve at `u` (`00b38b50`):
//! `s = (n − 1) × clamp(u, 0, 1)`, segment `trunc(s)`, de Casteljau at
//! the fraction; the last point at the end.
//!
//! Once read, the curve is turned into a table (`00b38430`, 500 entries):
//! 500 curve points at `u = k/500`, then entry 0 = the first point,
//! entries 1–498 the curve's height at `x = k/500` by linear interpolation
//! between the samples around it (searched onward from the last segment
//! found), entry 499 the last point.
//!
//! A value at `x` (`00b37ad0`): `i = trunc(499·x)`, the table's height at
//! `i` blended toward `i + 1` by `(x − i·0.002004008) / 0.002004008`
//! (`1/499`, though the table was made with steps of `1/500`), or entry
//! 499's at `i = 499`; then `(max − min) × h + min` plus a uniform random
//! number in `[−variance, variance]` (drawn every time, even when the
//! variance is 0). The variance alone (`00b37fa0`): entry `round(499·x)`'s
//! height `h`, and a random number in `[−variance·h, variance·h]`.

use crate::random::Random;

/// The table's size (`00b38430` is called with 500).
const TABLE: usize = 500;
/// The step the lookup assumes (`010a57dc`).
const STEP: f32 = 0.002_004_008;

/// A 2D point as SpeedTree keeps it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct P2 {
    x: f32,
    y: f32,
}

/// One curve: its range and variance and the sampled table.
#[derive(Debug, Clone, PartialEq)]
pub struct Spline {
    pub min: f32,
    pub max: f32,
    pub variance: f32,
    /// Heights at the table's 500 entries.
    table: Vec<f32>,
}

impl Spline {
    /// Reads the text form. Anything that isn't a `BezierSpline` gives a
    /// curve with no points (as the game's: min 0, max 1, variance 0 and an
    /// empty table, which evaluates to 0).
    pub fn parse(text: &str) -> Spline {
        let mut words = text.split_ascii_whitespace();
        let mut empty = Spline {
            min: 0.0,
            max: 1.0,
            variance: 0.0,
            table: Vec::new(),
        };
        if words.next() != Some("BezierSpline") {
            return empty;
        }
        let mut num = || words.next().map_or(0.0, atof);
        let min = num() as f32;
        let max = num() as f32;
        let variance = num() as f32;
        empty.min = min;
        empty.max = max;
        empty.variance = variance;
        let mut words = text.split_ascii_whitespace().skip(4);
        if words.next() != Some("{") {
            return empty;
        }
        let count = words.next().map_or(0, atoi);
        let mut points = Vec::new();
        let mut tangents = Vec::new();
        let mut lengths = Vec::new();
        let mut control = Vec::new();
        for _ in 0..count.max(0) {
            let mut num = || words.next().map_or(0.0, atof) as f32;
            let x = num();
            let y = num();
            let dx = num();
            let dy = num();
            let len = num();
            let p = P2 { x, y };
            let t = normalized(P2 { x: dx, y: dy });
            if let (Some(&lp), Some(&lt), Some(&ll)) =
                (points.last(), tangents.last(), lengths.last())
            {
                let lp: P2 = lp;
                let lt: P2 = lt;
                let ll: f32 = ll;
                control.push(P2 {
                    x: lp.x + lt.x * ll,
                    y: lp.y + lt.y * ll,
                });
                control.push(P2 {
                    x: p.x - t.x * len,
                    y: p.y - t.y * len,
                });
            }
            points.push(p);
            control.push(p);
            tangents.push(t);
            lengths.push(len);
        }
        empty.table = make_table(&points, &control);
        empty
    }

    /// `00b37ad0`: the value at `x` (0..1).
    pub fn value(&self, x: f32, random: &mut Random) -> f32 {
        if self.table.len() != TABLE {
            return 0.0;
        }
        let i = ((x as f64 * 499.0) as i32).clamp(0, TABLE as i32 - 1) as usize;
        let h = if i == TABLE - 1 {
            self.table[TABLE - 1]
        } else {
            let a = self.table[i];
            let b = self.table[i + 1];
            let t = ((x as f64 - (i as f64 * STEP as f64)) / STEP as f64) as f32;
            ((b as f64 - a as f64) * t as f64 + a as f64) as f32
        };
        let v = ((self.max as f64 - self.min as f64) * h as f64 + self.min as f64) as f32;
        let r = random.uniform(-self.variance, self.variance);
        (r as f64 + v as f64) as f32
    }

    /// `00b37fa0`: only the variance part at `x`.
    pub fn variance_at(&self, x: f32, random: &mut Random) -> f32 {
        if self.table.len() != TABLE {
            return 0.0;
        }
        let i = ((x as f64 * 499.0 + 0.5) as i32).clamp(0, TABLE as i32 - 1) as usize;
        let h = self.table[i];
        let a = (-self.variance as f64 * h as f64) as f32;
        let b = (self.variance as f64 * h as f64) as f32;
        random.uniform(a, b)
    }

    /// The table's height at entry `i` (for tests and tools).
    pub fn table(&self) -> &[f32] {
        &self.table
    }
}

/// `00b3c600`: divided by its length (`00b3c5a0`: the squares summed into a
/// float one at a time, then the square root) unless that is 0.
fn normalized(p: P2) -> P2 {
    let mut sum = 0.0f32;
    for c in [p.x, p.y] {
        sum = (c as f64 * c as f64 + sum as f64) as f32;
    }
    let len = (sum as f64).sqrt() as f32;
    if len == 0.0 {
        p
    } else {
        P2 {
            x: p.x / len,
            y: p.y / len,
        }
    }
}

/// `00b38f40`: de Casteljau, in the code's order, each blend stored as a
/// float.
fn bezier(p0: P2, p1: P2, p2: P2, p3: P2, t: f32) -> P2 {
    let lerp = |a: f32, b: f32| -> f32 { lerp32(a, b, t) };
    let ax = lerp(p0.x, p1.x);
    let ay = lerp(p0.y, p1.y);
    let bx = lerp(p1.x, p2.x);
    let by = lerp(p1.y, p2.y);
    let cx = lerp(p2.x, p3.x);
    let cy = lerp(p2.y, p3.y);
    let abx = lerp(ax, bx);
    let aby = lerp(ay, by);
    let bcx = lerp(bx, cx);
    let bcy = lerp(by, cy);
    P2 {
        x: lerp(abx, bcx),
        y: lerp(aby, bcy),
    }
}

/// `(b − a) × t + a` worked in double precision and stored as a float, as
/// the game's code does each such blend.
pub(crate) fn lerp32(a: f32, b: f32, t: f32) -> f32 {
    ((b as f64 - a as f64) * t as f64 + a as f64) as f32
}

/// `00b38b50`: the curve at `u`.
fn curve(points: &[P2], control: &[P2], u: f32) -> P2 {
    let n = points.len();
    if n < 2 {
        return P2::default();
    }
    let u = u.clamp(0.0, 1.0);
    let s = ((n - 1) as f64 * u as f64) as f32;
    let whole = s as i32;
    let frac = s - whole as f32;
    for i in 0..n {
        if i == n - 1 {
            return points[i];
        }
        if (i as f32) <= s && s < (i + 1) as f32 {
            return bezier(
                control[3 * i],
                control[3 * i + 1],
                control[3 * i + 2],
                control[3 * i + 3],
                frac,
            );
        }
    }
    P2::default()
}

/// `00b38430`.
fn make_table(points: &[P2], control: &[P2]) -> Vec<f32> {
    if points.is_empty() {
        return Vec::new();
    }
    let samples: Vec<P2> = (0..TABLE)
        .map(|k| curve(points, control, (k as f64 / TABLE as f64) as f32))
        .collect();
    let mut table = vec![0.0f32; TABLE];
    table[0] = points[0].y;
    let mut seg = 0usize;
    for (k, entry) in table.iter_mut().enumerate().take(TABLE - 1).skip(1) {
        let x = (k as f64 / TABLE as f64) as f32;
        for j in seg..TABLE - 1 {
            if samples[j].x <= x && x < samples[j + 1].x {
                seg = j;
                break;
            }
        }
        let a = samples[seg];
        let b = samples[seg + 1];
        let t = ((x as f64 - a.x as f64) / (b.x as f64 - a.x as f64)) as f32;
        *entry = ((b.y as f64 - a.y as f64) * t as f64 + a.y as f64) as f32;
    }
    table[TABLE - 1] = points[points.len() - 1].y;
    table
}

/// C's `atof` on one word: the longest leading number, else 0.
fn atof(word: &str) -> f64 {
    let bytes = word.as_bytes();
    let mut end = 0;
    let mut seen_digit = false;
    let mut seen_dot = false;
    let mut seen_exp = false;
    while end < bytes.len() {
        let c = bytes[end];
        match c {
            b'+' | b'-' if end == 0 => {}
            b'+' | b'-' if seen_exp && matches!(bytes[end - 1], b'e' | b'E') => {}
            b'0'..=b'9' => seen_digit = true,
            b'.' if !seen_dot && !seen_exp => seen_dot = true,
            b'e' | b'E' if seen_digit && !seen_exp => seen_exp = true,
            _ => break,
        }
        end += 1;
    }
    let mut s = &word[..end];
    while !s.is_empty() && s.parse::<f64>().is_err() {
        s = &s[..s.len() - 1];
    }
    s.parse().unwrap_or(0.0)
}

/// C's `atoi`-like reading of the point count.
fn atoi(word: &str) -> i32 {
    atof(word) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_line_from_the_game() {
        // The shrub's "0 1 0" line rising from 0 to 1 with 45° tangents.
        let s = Spline::parse(
            "BezierSpline 0\t1\t0\n{\n\n\t2\n\t0 0 0.707107 0.707107 0.079604\n\t1 1 0.707107 0.707107 0.107006\n\n}\n",
        );
        assert_eq!((s.min, s.max, s.variance), (0.0, 1.0, 0.0));
        assert_eq!(s.table().len(), 500);
        let mut r = Random::new(5);
        for x in [0.0f32, 0.25, 0.5, 0.75] {
            let v = s.value(x, &mut r);
            assert!((v - x).abs() < 0.01, "{x} -> {v}");
        }
        assert_eq!(s.value(1.0, &mut r), 1.0);
    }

    #[test]
    fn variance_draws_every_time() {
        let s = Spline::parse("BezierSpline 2 2 0 { 2 0 1 1 0 0.3 1 1 1 0 0.3 }");
        let mut a = Random::new(7);
        let mut b = Random::new(7);
        assert_eq!(s.value(0.5, &mut a), 2.0);
        b.next_float();
        assert_eq!(a.next_float(), b.next_float());
    }

    #[test]
    fn atof_reads_like_c() {
        assert_eq!(atof("-6.24607e-008"), -6.24607e-8);
        assert_eq!(atof("0.5}"), 0.5);
        assert_eq!(atof("{"), 0.0);
    }
}
