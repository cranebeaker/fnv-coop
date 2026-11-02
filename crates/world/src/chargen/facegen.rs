//! FaceGen controls and statistics on a face's coordinates, as the face
//! menu reads and sets them (`FalloutNV.exe` 1.4.0.525). The control file
//! itself is `nif::Ctl`; see `docs/FACEGEN_CONTROLS.md`.
//!
//! * A control's value is its coefficients' dot product with the face's
//!   coordinates of that kind and symmetry (`00652230`). Setting it adds
//!   (target − value) × the coefficients to the coordinates (`00652320`),
//!   without dividing by their squared length: a control whose
//!   coefficients aren't of unit length reads back a different value, as
//!   in the game.
//! * Age and gender are the first statistic group's linear functions of
//!   the symmetric coordinates (`00652440` → `00aabdb0`: coefficients ·
//!   coordinates + offset). Setting them (`006524e0` → `00aac170`) clamps
//!   age to 15–65 and gender to −4–4, then moves the coordinates by the
//!   smallest change that reaches both: Cᵀ (C Cᵀ)⁻¹ d, where C's rows are
//!   the age and gender coefficients and d the two differences. The game
//!   inverts C Cᵀ when it loads the file (`00aaaf10`); here it is worked
//!   out on each call, in the same single precision.
//! * Setting one statistic (`00652470`) sets both, the other at its
//!   current value.

use nif::ctl::{Ctl, Kind, Stat, Symmetry};

/// The statistic group the game uses (`00652440` and `006524e0` pass 0).
pub const GROUP: usize = 0;

/// `00aac170`'s clamps: age, then gender.
pub const STAT_RANGES: [(f32, f32); 2] = [(15.0, 65.0), (-4.0, 4.0)];

/// A face's FaceGen coordinates, `[kind][symmetry]`: shape symmetric and
/// asymmetric (an NPC's `FGGS`, `FGGA`), texture symmetric (`FGTS`) and
/// asymmetric (none in the game's files).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Coords(pub [[Vec<f32>; 2]; 2]);

impl Coords {
    pub fn get(&self, kind: Kind, symmetry: Symmetry) -> &[f32] {
        &self.0[kind as usize][symmetry as usize]
    }

    fn get_mut(&mut self, kind: Kind, symmetry: Symmetry, len: usize) -> &mut Vec<f32> {
        let v = &mut self.0[kind as usize][symmetry as usize];
        if v.len() < len {
            v.resize(len, 0.0);
        }
        v
    }
}

/// Missing coordinates count as 0.
fn dot(coefficients: &[f32], coords: &[f32]) -> f32 {
    coefficients
        .iter()
        .zip(coords.iter().chain(std::iter::repeat(&0.0)))
        .fold(0.0, |sum, (c, x)| sum + c * x)
}

fn add_scaled(coords: &mut [f32], coefficients: &[f32], scale: f32) {
    for (x, c) in coords.iter_mut().zip(coefficients) {
        *x += c * scale;
    }
}

/// A control's value for a face; 0 for a control the file doesn't have
/// (`00652230`).
pub fn control(ctl: &Ctl, coords: &Coords, kind: Kind, symmetry: Symmetry, index: usize) -> f32 {
    ctl.controls(kind, symmetry)
        .get(index)
        .map_or(0.0, |c| dot(&c.coefficients, coords.get(kind, symmetry)))
}

/// Moves a face so that the control reads `value`, as the game does
/// (`00652320`; see the module notes). Nothing for a missing control.
pub fn set_control(
    ctl: &Ctl,
    coords: &mut Coords,
    kind: Kind,
    symmetry: Symmetry,
    index: usize,
    value: f32,
) {
    let Some(c) = ctl.controls(kind, symmetry).get(index) else {
        return;
    };
    let delta = value - control(ctl, coords, kind, symmetry, index);
    let x = coords.get_mut(kind, symmetry, c.coefficients.len());
    add_scaled(x, &c.coefficients, delta);
}

/// Age or gender as the shape (or texture) coordinates show it
/// (`00652440`); 0 without statistics.
pub fn stat(ctl: &Ctl, coords: &Coords, stat: Stat, kind: Kind) -> f32 {
    ctl.stat(GROUP, stat, kind).map_or(0.0, |s| {
        dot(&s.coefficients, coords.get(kind, Symmetry::Symmetric)) + s.offset
    })
}

/// Sets age and gender together (`006524e0` → `00aac170`).
pub fn set_stats(ctl: &Ctl, coords: &mut Coords, kind: Kind, values: [f32; 2]) {
    let (Some(age), Some(gender)) = (
        ctl.stat(GROUP, Stat::Age, kind),
        ctl.stat(GROUP, Stat::Gender, kind),
    ) else {
        return;
    };
    let rows = [&age.coefficients, &gender.coefficients];
    let mut d = [0.0; 2];
    for (a, s) in [Stat::Age, Stat::Gender].into_iter().enumerate() {
        let (low, high) = STAT_RANGES[a];
        let target = values[a].max(low).min(high);
        d[a] = target - stat(ctl, coords, s, kind);
    }
    // (C Cᵀ)⁻¹, as the loader stores it (`00aaaf10`, inverse `00aadd20`).
    let g = |a: usize, b: usize| dot(rows[a], rows[b]);
    let inv = 1.0 / (g(0, 0) * g(1, 1) - g(0, 1) * g(1, 0));
    let m = [g(1, 1) * inv, -g(0, 1) * inv, -g(1, 0) * inv, g(0, 0) * inv];
    // `00aaf2b0`.
    let y = [m[1] * d[1] + m[0] * d[0], m[3] * d[1] + m[2] * d[0]];
    let len = rows[0].len().max(rows[1].len());
    let x = coords.get_mut(kind, Symmetry::Symmetric, len);
    for a in 0..2 {
        add_scaled(x, rows[a], y[a]);
    }
}

/// Sets one statistic, keeping the other where it is (`00652470`).
pub fn set_stat(ctl: &Ctl, coords: &mut Coords, which: Stat, kind: Kind, value: f32) {
    let mut values = [0.0; 2];
    for (a, s) in [Stat::Age, Stat::Gender].into_iter().enumerate() {
        values[a] = if s == which {
            value
        } else {
            stat(ctl, coords, s, kind)
        };
    }
    set_stats(ctl, coords, kind, values);
}

#[cfg(test)]
mod tests {
    use super::*;
    use nif::ctl::{Control, Linear, GROUPS};

    fn linear(coefficients: Vec<f32>, offset: f32) -> Linear {
        Linear {
            coefficients,
            offset,
        }
    }

    fn file() -> Ctl {
        let control = |label: &str, coefficients: Vec<f32>| Control {
            label: label.into(),
            coefficients,
        };
        let stats = [
            [
                linear(vec![2.0, 0.0, 1.0], 30.0),
                linear(vec![0.5, 0.5], 40.0),
            ],
            [
                linear(vec![0.0, 1.0, 1.0], 0.0),
                linear(vec![1.0, -1.0], 0.0),
            ],
        ];
        Ctl {
            basis_ids: [0, 0],
            sizes: [[3, 1], [2, 0]],
            controls: [
                [
                    vec![
                        control("Unit", vec![0.0, 1.0, 0.0]),
                        control("Long", vec![2.0, 0.0, 0.0]),
                    ],
                    vec![control("Tilt", vec![1.0])],
                ],
                [vec![control("Tone", vec![1.0, 1.0])], vec![]],
            ],
            stats: vec![stats; GROUPS],
            differences: vec![],
            distributions: vec![],
        }
    }

    #[test]
    fn a_control_reads_its_dot_product_and_sets_by_adding_its_coefficients() {
        let ctl = file();
        let mut face = Coords::default();
        face.0[0][0] = vec![1.0, 2.0];
        let s = (Kind::Shape, Symmetry::Symmetric);
        assert_eq!(control(&ctl, &face, s.0, s.1, 0), 2.0);
        assert_eq!(control(&ctl, &face, s.0, s.1, 1), 2.0);
        assert_eq!(control(&ctl, &face, s.0, s.1, 9), 0.0);

        set_control(&ctl, &mut face, s.0, s.1, 0, 5.0);
        assert_eq!(face.get(s.0, s.1), [1.0, 5.0, 0.0]);
        assert_eq!(control(&ctl, &face, s.0, s.1, 0), 5.0);

        // Coefficients of length 2: the change is 4 × what was asked.
        set_control(&ctl, &mut face, s.0, s.1, 1, 3.0);
        assert_eq!(face.get(s.0, s.1), [3.0, 5.0, 0.0]);
        assert_eq!(control(&ctl, &face, s.0, s.1, 1), 6.0);

        set_control(&ctl, &mut face, s.0, s.1, 7, 1.0);
        assert_eq!(face.get(s.0, s.1), [3.0, 5.0, 0.0]);
    }

    #[test]
    fn age_and_gender_are_set_together_by_the_smallest_change() {
        let ctl = file();
        let mut face = Coords::default();
        assert_eq!(stat(&ctl, &face, Stat::Age, Kind::Shape), 30.0);
        assert_eq!(stat(&ctl, &face, Stat::Gender, Kind::Shape), 0.0);

        set_stats(&ctl, &mut face, Kind::Shape, [40.0, 2.0]);
        let close = |a: f32, b: f32| (a - b).abs() < 1e-5;
        assert!(close(stat(&ctl, &face, Stat::Age, Kind::Shape), 40.0));
        assert!(close(stat(&ctl, &face, Stat::Gender, Kind::Shape), 2.0));
        // The change lies in the span of the two coefficient rows.
        let x = face.get(Kind::Shape, Symmetry::Symmetric).to_vec();
        let (a, b) = (x[0] / 2.0, x[1]);
        assert!(close(x[2], a + b));

        // One at a time keeps the other.
        set_stat(&ctl, &mut face, Stat::Age, Kind::Shape, 20.0);
        assert!(close(stat(&ctl, &face, Stat::Age, Kind::Shape), 20.0));
        assert!(close(stat(&ctl, &face, Stat::Gender, Kind::Shape), 2.0));
    }

    #[test]
    fn age_and_gender_are_clamped_to_the_games_ranges() {
        let ctl = file();
        let mut face = Coords::default();
        set_stats(&ctl, &mut face, Kind::Texture, [90.0, -9.0]);
        let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
        assert!(close(stat(&ctl, &face, Stat::Age, Kind::Texture), 65.0));
        assert!(close(stat(&ctl, &face, Stat::Gender, Kind::Texture), -4.0));
        set_stat(&ctl, &mut face, Stat::Age, Kind::Texture, 3.0);
        assert!(close(stat(&ctl, &face, Stat::Age, Kind::Texture), 15.0));
    }
}
