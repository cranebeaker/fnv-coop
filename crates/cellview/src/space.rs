//! Converting between the game's space and a typical real-time renderer's.
//!
//! The game is Z-up and measures in its own units (64 units to the yard).
//! Renderers like Bevy are Y-up and measure in meters, with the camera
//! looking down -Z. The conversion is a proper rotation (no mirroring), so
//! triangle winding is kept:
//!
//! * game +X (east)  -> view +X
//! * game +Y (north) -> view -Z
//! * game +Z (up)    -> view +Y

/// Meters per game unit: 64 units to the yard (0.9144 m).
pub const METERS_PER_UNIT: f32 = 0.9144 / 64.0;

/// A position in game units to view space in meters.
pub fn point(p: [f32; 3]) -> [f32; 3] {
    let s = METERS_PER_UNIT;
    [p[0] * s, p[2] * s, -p[1] * s]
}

/// A direction (unit-free) from game to view space.
pub fn direction(d: [f32; 3]) -> [f32; 3] {
    [d[0], d[2], -d[1]]
}

/// A column-major 4x4 model matrix mapping into game space (units) turned
/// into one mapping into view space (meters).
pub fn matrix(m: &[f32; 16]) -> [f32; 16] {
    let s = METERS_PER_UNIT;
    let mut out = *m;
    for col in 0..4 {
        let c = &m[col * 4..col * 4 + 3];
        out[col * 4] = c[0] * s;
        out[col * 4 + 1] = c[2] * s;
        out[col * 4 + 2] = -c[1] * s;
    }
    out
}

/// The yaw (radians about view +Y, 0 looking down -Z) of a game heading
/// (radians clockwise from north).
pub fn heading_to_yaw(heading: f32) -> f32 {
    -heading
}

/// Applies a column-major 4x4 matrix to a point.
pub fn transform_point(m: &[f32; 16], p: [f32; 3]) -> [f32; 3] {
    [0, 1, 2].map(|r| m[r] * p[0] + m[4 + r] * p[1] + m[8 + r] * p[2] + m[12 + r])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: [f32; 3], b: [f32; 3]) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    #[test]
    fn maps_axes_and_units() {
        let s = METERS_PER_UNIT;
        assert!(close(point([64.0, 0.0, 0.0]), [0.9144, 0.0, 0.0]));
        assert!(close(point([0.0, 1.0, 0.0]), [0.0, 0.0, -s]));
        assert!(close(point([0.0, 0.0, 1.0]), [0.0, s, 0.0]));
        assert!(close(direction([0.0, 1.0, 0.0]), [0.0, 0.0, -1.0]));
    }

    #[test]
    fn converted_matrices_agree_with_converting_points() {
        // A quarter turn clockwise about Z, scaled by 2, moved to (10, 20, 30).
        let m = [
            0.0, -2.0, 0.0, 0.0, // column 0: +X -> -Y
            2.0, 0.0, 0.0, 0.0, // column 1: +Y -> +X
            0.0, 0.0, 2.0, 0.0, // column 2
            10.0, 20.0, 30.0, 1.0,
        ];
        let v = matrix(&m);
        for p in [[1.0, 0.0, 0.0], [0.0, 3.0, -1.0], [5.0, -2.0, 7.0]] {
            let direct = point(transform_point(&m, p));
            assert!(close(transform_point(&v, p), direct));
        }
    }

    #[test]
    fn headings_become_yaws_facing_the_same_way() {
        // Bevy-style: yaw rotates the -Z forward vector about +Y.
        let forward = |yaw: f32| [-yaw.sin(), 0.0, -yaw.cos()];
        for heading in [0.0f32, 0.5, 1.5707964, 3.0] {
            let game = [heading.sin(), heading.cos(), 0.0];
            assert!(close(forward(heading_to_yaw(heading)), direction(game)));
        }
    }
}
