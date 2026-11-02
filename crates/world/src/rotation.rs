//! How a placed object's three stored angles become a rotation.
//!
//! References store position and rotation as six floats: X, Y, Z in game
//! units, then three angles in radians. The heading (Z angle) is known to
//! turn clockwise seen from above: 0 faces north (+Y), a quarter turn faces
//! east (+X). The order the engine combines the three angles in isn't
//! documented, and only matters for objects rotated on more than one axis
//! (a tilted plank, a toppled chair). [`RotationConvention::DEFAULT`] was
//! settled against the game's own data: the bathroom mirror in Doc
//! Mitchell's house (`RestroomMirror03`, angles 270°, 270°, 0°) sits flush
//! on the wall with X·Y·Z, and would float flat in mid-air with Z·Y·X. The
//! other conventions stay available for comparison.

use nif::math::{mat_mul, Mat3, Transform, Vec3};

/// Which order the three angles are combined in, and their direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotationConvention {
    pub order: AxisOrder,
    /// Positive angles turn clockwise when looking down the axis toward the
    /// origin (the known behaviour for the heading).
    pub clockwise: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisOrder {
    /// `R = X · Y · Z`: the object turns about the world Z axis first, then
    /// Y, then X. This is how Gamebryo builds a matrix from Euler angles and
    /// how OpenMW places Morrowind objects, which uses the same engine
    /// lineage.
    Xyz,
    /// `R = Z · Y · X`: about world X first, then Y, then Z.
    Zyx,
}

impl RotationConvention {
    /// X·Y·Z, clockwise: what the game does (see the module notes).
    pub const DEFAULT: Self = Self {
        order: AxisOrder::Xyz,
        clockwise: true,
    };

    /// Every convention, the default first.
    pub const ALL: [Self; 4] = [
        Self::DEFAULT,
        Self {
            order: AxisOrder::Zyx,
            clockwise: true,
        },
        Self {
            order: AxisOrder::Xyz,
            clockwise: false,
        },
        Self {
            order: AxisOrder::Zyx,
            clockwise: false,
        },
    ];

    /// Short name used on the command line and in file names.
    pub fn name(&self) -> &'static str {
        match (self.order, self.clockwise) {
            (AxisOrder::Xyz, true) => "xyz",
            (AxisOrder::Zyx, true) => "zyx",
            (AxisOrder::Xyz, false) => "xyz-ccw",
            (AxisOrder::Zyx, false) => "zyx-ccw",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|c| c.name().eq_ignore_ascii_case(name.trim()))
    }

    /// The rotation matrix (row-major, for column vectors) for stored angles
    /// `[x, y, z]` in radians.
    pub fn matrix(&self, angles: Vec3) -> Mat3 {
        let sign = if self.clockwise { -1.0 } else { 1.0 };
        let x = rot_x(sign * angles[0]);
        let y = rot_y(sign * angles[1]);
        let z = rot_z(sign * angles[2]);
        match self.order {
            AxisOrder::Xyz => mat_mul(&mat_mul(&x, &y), &z),
            AxisOrder::Zyx => mat_mul(&mat_mul(&z, &y), &x),
        }
    }

    /// The transform from an object's model space to the world.
    pub fn transform(&self, position: Vec3, angles: Vec3, scale: f32) -> Transform {
        Transform {
            rotation: self.matrix(angles),
            translation: position,
            scale,
        }
    }
}

impl Default for RotationConvention {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// True when an object is turned about more than one axis, the only case
/// where the conventions' orders disagree. Tiny angles don't count.
pub fn is_tilted(angles: Vec3) -> bool {
    const EPSILON: f32 = 0.01; // about half a degree
    let turned = |a: f32| {
        let a = a.rem_euclid(std::f32::consts::TAU);
        a > EPSILON && a < std::f32::consts::TAU - EPSILON
    };
    angles.iter().filter(|&&a| turned(a)).count() >= 2
}

// Counter-clockwise (right-handed) rotations about each axis.

fn rot_x(a: f32) -> Mat3 {
    let (s, c) = a.sin_cos();
    [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]]
}

fn rot_y(a: f32) -> Mat3 {
    let (s, c) = a.sin_cos();
    [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]]
}

fn rot_z(a: f32) -> Mat3 {
    let (s, c) = a.sin_cos();
    [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
}

#[cfg(test)]
mod tests {
    use super::*;
    use nif::math::mat_vec;
    use std::f32::consts::FRAC_PI_2;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    #[test]
    fn a_positive_heading_turns_north_toward_east() {
        for c in RotationConvention::ALL.iter().filter(|c| c.clockwise) {
            let m = c.matrix([0.0, 0.0, FRAC_PI_2]);
            assert!(close(mat_vec(&m, [0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]));
            assert!(close(mat_vec(&m, [0.0, 0.0, 1.0]), [0.0, 0.0, 1.0]));
        }
        let ccw = RotationConvention::parse("xyz-ccw").unwrap();
        let m = ccw.matrix([0.0, 0.0, FRAC_PI_2]);
        assert!(close(mat_vec(&m, [0.0, 1.0, 0.0]), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn orders_agree_on_single_axis_turns_and_differ_otherwise() {
        let [xyz, zyx, ..] = RotationConvention::ALL;
        for angles in [[0.3, 0.0, 0.0], [0.0, -1.2, 0.0], [0.0, 0.0, 2.5]] {
            assert_eq!(xyz.matrix(angles), zyx.matrix(angles));
            assert!(!is_tilted(angles));
        }
        let angles = [FRAC_PI_2, 0.0, FRAC_PI_2];
        assert!(is_tilted(angles));
        // A plank lying along +Y, tipped up about X, then turned to face east
        // (X·Y·Z turns it first, then tips it about world X).
        let tip = [0.0, 1.0, 0.0];
        assert!(close(mat_vec(&zyx.matrix(angles), tip), [0.0, 0.0, -1.0]));
        assert!(close(mat_vec(&xyz.matrix(angles), tip), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn matrices_are_rotations() {
        for c in RotationConvention::ALL {
            let m = c.matrix([0.4, -1.1, 2.9]);
            let mt = [
                [m[0][0], m[1][0], m[2][0]],
                [m[0][1], m[1][1], m[2][1]],
                [m[0][2], m[1][2], m[2][2]],
            ];
            let id = mat_mul(&m, &mt);
            for (i, row) in id.iter().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    let want = if i == j { 1.0 } else { 0.0 };
                    assert!((v - want).abs() < 1e-5);
                }
            }
        }
    }

    #[test]
    fn names_round_trip() {
        for c in RotationConvention::ALL {
            assert_eq!(RotationConvention::parse(c.name()), Some(c));
        }
        assert_eq!(
            RotationConvention::parse("ZYX"),
            Some(RotationConvention::ALL[1])
        );
        assert!(RotationConvention::parse("yxz").is_none());
        assert!(!is_tilted([0.0, 0.0, std::f32::consts::TAU - 0.001]));
    }
}
