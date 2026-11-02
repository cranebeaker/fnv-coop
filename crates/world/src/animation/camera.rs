//! First-person camera placement from the animated skeleton.
//!
//! The executable reads the `Camera1st` world transform at `0094ae40`
//! (rotation at node +0x68, position at +0x8c). `00952290` pitches the
//! `Bip01 Looking` local transform by `fFirstPersonHandFollowMult` (default
//! 0.85, initialized at `00f5b5e0`); `0094ae40` then applies the remaining
//! pitch to the camera rotation on the right. The player root stays at the
//! actor's feet (`00888b50` / `00889812`) and faces `Rz(-heading)`. These
//! operations use the row-major, column-vector convention confirmed for
//! `004a0c90`, `00524ac0`, and `0043f8d0`.

use nif::math::{mat_mul, mat_vec, Mat3};

/// Hand-follow interpolation in `00952290`: advance a fraction of the
/// remaining difference, clamping at the target. Chase times are the GMSTs
/// fFirstPersonHandChaseSeconds (2, 00f5b610) and its Attack variant
/// (0.05, 00f5b640). The caller selects the target and matching chase time.
pub fn chase_follow(current: f32, target: f32, seconds: f32, chase: f32) -> f32 {
    if current == target {
        return current;
    }
    let step = seconds / chase * (target - current);
    if step < 0.0 {
        (current + step).max(target)
    } else {
        (current + step).min(target)
    }
}

/// Places an animated first-person camera in the world.
///
/// `camera` is the camera node's animated skeletal transform, `pivot` is the
/// `Bip01 Looking` pivot in the same skeleton space, and `feet` / `heading`
/// place the actor root. Pitch is positive upward in this API; the game
/// applies the opposite sign because its pitch input is positive downward.
/// `follow` is the traced hand-follow factor (normally
/// `fFirstPersonHandFollowMult`, 0.85). When `animated_rotation` is false,
/// retain the ordinary heading/pitch view, as `0094ae40` does when its
/// `00933840` guard is true. Position still comes from the animated node.
pub fn first_person_camera(
    camera: &nif::Transform,
    pivot: [f32; 3],
    feet: [f32; 3],
    heading: f32,
    pitch: f32,
    follow: f32,
    animated_rotation: bool,
) -> nif::Transform {
    let followed_pitch = pitch * follow;
    let follow_rotation = rotation_x(followed_pitch);
    let camera_rotation = mat_mul(&follow_rotation, &camera.rotation);

    let relative_position = subtract(camera.translation, pivot);
    let camera_position = add(pivot, mat_vec(&follow_rotation, relative_position));

    let actor_rotation = rotation_z(-heading);
    let mut world_rotation = mat_mul(&actor_rotation, &camera_rotation);
    if animated_rotation {
        let residual_rotation = rotation_x(pitch * (1.0 - follow));
        world_rotation = mat_mul(&world_rotation, &residual_rotation);
    } else {
        world_rotation = mat_mul(&actor_rotation, &rotation_x(pitch));
    }

    nif::Transform {
        rotation: world_rotation,
        translation: add(feet, mat_vec(&actor_rotation, camera_position)),
        scale: camera.scale,
    }
}

fn rotation_x(angle: f32) -> Mat3 {
    let (s, c) = angle.sin_cos();
    [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]]
}

fn rotation_z(angle: f32) -> Mat3 {
    let (s, c) = angle.sin_cos();
    [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_follow_chases_remaining_difference_and_clamps_large_steps() {
        assert!((chase_follow(1.0, 0.85, 0.5, 2.0) - 0.9625).abs() < 1e-6);
        assert_eq!(chase_follow(1.0, 0.85, 3.0, 2.0), 0.85);
        assert_eq!(chase_follow(0.85, 1.0, 0.1, 0.05), 1.0);
        assert_eq!(chase_follow(0.85, 0.85, 0.0, 0.0), 0.85);
    }

    #[test]
    fn guard_keeps_input_rotation_but_animated_position() {
        let camera = nif::Transform {
            rotation: rotation_y(0.7),
            translation: [1.0, 2.0, 3.0],
            ..nif::Transform::IDENTITY
        };
        let guarded = first_person_camera(&camera, [0.0; 3], [0.0; 3], 0.3, 0.4, 0.85, false);
        let animated = first_person_camera(&camera, [0.0; 3], [0.0; 3], 0.3, 0.4, 0.85, true);
        close_rotation(
            guarded.rotation,
            mat_mul(&rotation_z(-0.3), &rotation_x(0.4)),
        );
        close3(guarded.translation, animated.translation);
    }

    fn close3(actual: [f32; 3], expected: [f32; 3]) {
        for (a, e) in actual.into_iter().zip(expected) {
            assert!((a - e).abs() < 1e-5, "{actual:?} != {expected:?}");
        }
    }

    fn close_rotation(actual: Mat3, expected: Mat3) {
        for (actual_row, expected_row) in actual.into_iter().zip(expected) {
            for (a, e) in actual_row.into_iter().zip(expected_row) {
                assert!((a - e).abs() < 1e-5, "{actual:?} != {expected:?}");
            }
        }
    }

    fn rotation_y(angle: f32) -> Mat3 {
        let (s, c) = angle.sin_cos();
        [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]]
    }

    #[test]
    fn camera_offset_is_placed_from_actor_feet_and_heading() {
        let camera = nif::Transform {
            translation: [1.0, 2.0, 3.0],
            ..nif::Transform::IDENTITY
        };
        let result = first_person_camera(
            &camera,
            [0.0; 3],
            [10.0, 20.0, 0.0],
            std::f32::consts::FRAC_PI_2,
            0.0,
            0.85,
            true,
        );
        close3(result.translation, [12.0, 19.0, 3.0]);
    }

    #[test]
    fn neutral_camera_receives_full_pitch_across_follow_and_compensation() {
        let pitch = 0.4;
        let result = first_person_camera(
            &nif::Transform::IDENTITY,
            [0.0; 3],
            [0.0; 3],
            0.0,
            pitch,
            0.85,
            true,
        );
        close_rotation(result.rotation, rotation_x(pitch));
    }

    #[test]
    fn residual_pitch_postmultiplies_the_animated_camera_rotation() {
        let pitch = 0.6;
        let follow = 0.85;
        let animated = rotation_y(0.7);
        let camera = nif::Transform {
            rotation: animated,
            ..nif::Transform::IDENTITY
        };
        let result = first_person_camera(&camera, [0.0; 3], [0.0; 3], 0.0, pitch, follow, true);
        let expected = mat_mul(
            &mat_mul(&rotation_x(pitch * follow), &animated),
            &rotation_x(pitch * (1.0 - follow)),
        );
        let premultiplied = mat_mul(
            &mat_mul(
                &rotation_x(pitch * follow),
                &rotation_x(pitch * (1.0 - follow)),
            ),
            &animated,
        );
        close_rotation(result.rotation, expected);
        assert!(result
            .rotation
            .iter()
            .flatten()
            .zip(premultiplied.iter().flatten())
            .any(|(a, b)| (a - b).abs() > 1e-3));
    }

    #[test]
    fn translation_rotates_only_by_follow_pitch_about_looking_pivot() {
        let camera = nif::Transform {
            translation: [0.0, 2.0, 0.0],
            ..nif::Transform::IDENTITY
        };
        let pitch = 0.4;
        let follow = 0.85;
        let result =
            first_person_camera(&camera, [0.0, 1.0, 0.0], [0.0; 3], 0.0, pitch, follow, true);
        let followed = pitch * follow;
        close3(
            result.translation,
            [0.0, 1.0 + followed.cos(), followed.sin()],
        );
    }
}
