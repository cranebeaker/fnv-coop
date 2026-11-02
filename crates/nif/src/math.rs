//! Small vector and transform math, so the crate has no dependencies.

pub type Vec3 = [f32; 3];
/// Row-major 3x3 matrix, as stored in NIF files.
pub type Mat3 = [[f32; 3]; 3];

pub const IDENTITY3: Mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

pub fn mat_vec(m: &Mat3, v: Vec3) -> Vec3 {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

pub fn mat_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut out = [[0.0; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    out
}

pub fn normalize(v: Vec3) -> Vec3 {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 0.0 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        v
    }
}

/// A rigid transform with uniform scale, as used by Gamebryo:
/// `point' = rotation · (scale · point) + translation`, with column vectors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub rotation: Mat3,
    pub translation: Vec3,
    pub scale: f32,
}

impl Transform {
    pub const IDENTITY: Transform = Transform {
        rotation: IDENTITY3,
        translation: [0.0; 3],
        scale: 1.0,
    };

    pub fn apply_point(&self, p: Vec3) -> Vec3 {
        let r = mat_vec(
            &self.rotation,
            [p[0] * self.scale, p[1] * self.scale, p[2] * self.scale],
        );
        [
            r[0] + self.translation[0],
            r[1] + self.translation[1],
            r[2] + self.translation[2],
        ]
    }

    /// Rotates a direction (such as a normal); uniform scale doesn't change
    /// directions, so it's ignored.
    pub fn apply_direction(&self, d: Vec3) -> Vec3 {
        mat_vec(&self.rotation, d)
    }

    /// The transform that applies `child` first, then `self`: a child's
    /// local transform composed under its parent's world transform.
    pub fn then_child(&self, child: &Transform) -> Transform {
        Transform {
            rotation: mat_mul(&self.rotation, &child.rotation),
            translation: self.apply_point(child.translation),
            scale: self.scale * child.scale,
        }
    }

    /// The transform that undoes this one (the rotation taken as a pure
    /// rotation, as the files' are). A zero scale gives the identity.
    pub fn inverse(&self) -> Transform {
        if self.scale == 0.0 {
            return Transform::IDENTITY;
        }
        let r = &self.rotation;
        let rotation = [
            [r[0][0], r[1][0], r[2][0]],
            [r[0][1], r[1][1], r[2][1]],
            [r[0][2], r[1][2], r[2][2]],
        ];
        let scale = 1.0 / self.scale;
        let back = mat_vec(&rotation, self.translation);
        Transform {
            rotation,
            translation: [-back[0] * scale, -back[1] * scale, -back[2] * scale],
            scale,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    /// 90° counter-clockwise about Z, row-major.
    const ROT_Z90: Mat3 = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];

    #[test]
    fn applies_scale_then_rotation_then_translation() {
        let t = Transform {
            rotation: ROT_Z90,
            translation: [10.0, 0.0, 0.0],
            scale: 2.0,
        };
        assert!(close(t.apply_point([1.0, 0.0, 0.0]), [10.0, 2.0, 0.0]));
        assert!(close(t.apply_direction([1.0, 0.0, 0.0]), [0.0, 1.0, 0.0]));
    }

    #[test]
    fn composing_matches_applying_in_sequence() {
        let parent = Transform {
            rotation: ROT_Z90,
            translation: [10.0, 0.0, 0.0],
            scale: 2.0,
        };
        let child = Transform {
            rotation: ROT_Z90,
            translation: [0.0, 0.0, 1.0],
            scale: 0.5,
        };
        let p = [3.0, -4.0, 5.0];
        let world = parent.then_child(&child);
        assert!(close(
            world.apply_point(p),
            parent.apply_point(child.apply_point(p))
        ));
        // And undoing it gives the point back.
        assert!(close(world.inverse().apply_point(world.apply_point(p)), p));
        assert!(close(world.then_child(&world.inverse()).apply_point(p), p));
    }
}
