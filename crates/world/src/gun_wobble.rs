//! The player's gun sway, as `FalloutNV.exe` 1.4.0.525 does it every
//! frame (`00962de0`, with the wobble models loaded by `008d66b0` and
//! sampled by `008d6970`).
//!
//! - The models: `Meshes\Characters\WeaponWobbles\<name>.nif`, one per
//!   wobble kind ([`FILES`]: 0 `ScopeWobble`, then the weapon kinds'
//!   names; the weapon's animation type picks one through the table at
//!   `0118a838`, [`wobble_kind`]). Each is a node named for the bone it
//!   turns (`2hr.nif`: `Bip01 Spine2`) with keyed X, Y, Z angles.
//! - The amount: with a gun out, the wobble (`008b0dd0(2)` →
//!   `00646910`, `world::vats::wobble`) × `fNonAttackGunWobbleMult` when
//!   not attacking (anim actions 2–6, `00894900`); the value kept
//!   (`011a3b2c`, 1 at the start) eases toward it by
//!   `fGunWobbleChaseDriftTime` × the frame's seconds, jumping up at once
//!   while attacking ([`chase`]).
//! - Unscoped, first person: the kind's model sampled at the first-person
//!   animation's time, relative to its controller's start (`008d6970`:
//!   R(t)·R(start)ᵀ), as X, Y, Z angles (`00a592c0`) × the amount, made a
//!   rotation again (`00a59540`) and set on an `AdditionalRotation`
//!   controller (`00c8ffd0`, vtable `010c6234`) on the first-person
//!   skeleton's bone of that name: its update (`00c90030`) puts the
//!   rotation after the bone's own ([`sway_rotation`]).
//! - Scoped (the sights up with a scope, below): the `ScopeWobble` model
//!   (kind 0) sampled without the start — so R(t)·R(t of the last frame)ᵀ,
//!   the frame's change — × the wobble (`008b0dd0(0)`) ×
//!   `fGunWobbleMultScope`, and its X angle added to the player's pitch
//!   (`00931e50`), its Z angle to the heading (`00931d30`, `SetHeading`
//!   (Xbox PDB)): the view itself sways.

/// The wobble models' names by kind (the table at `011977a4`); 0 is
/// `ScopeWobble`. Only `1HP`, `2HA`, `2HH`, `2HL`, `2HR` and the scope's
/// exist in the game's files; the others load nothing (no sway).
pub const FILES: [&str; 12] = [
    "ScopeWobble",
    "H2H",
    "1HM",
    "2HM",
    "1HP",
    "2HR",
    "2HA",
    "2HH",
    "2HL",
    "1GT",
    "1MD",
    "1LM",
];

/// The wobble kind for a weapon's animation type (`DNAM` u32 at 0; the
/// table at `0118a838`).
pub fn wobble_kind(animation: u32) -> usize {
    const TABLE: [usize; 14] = [1, 2, 3, 4, 4, 5, 6, 5, 7, 8, 9, 10, 11, 9];
    TABLE.get(animation as usize).copied().unwrap_or(1)
}

/// The model's path (`%s\Characters\WeaponWobbles\%s.nif`).
pub fn model_path(kind: usize) -> Option<String> {
    FILES
        .get(kind)
        .map(|n| format!("Meshes\\Characters\\WeaponWobbles\\{n}.nif"))
}

/// The settings the sway reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `fNonAttackGunWobbleMult` (exe 0.25, data 0.5).
    pub non_attack_mult: f32,
    /// `fGunWobbleMultScope` (exe 0.2, data 1).
    pub scope_mult: f32,
    /// `fGunWobbleChaseDriftTime` (0.75).
    pub chase_drift: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            non_attack_mult: 0.25,
            scope_mult: 0.2,
            chase_drift: 0.75,
        }
    }
}

impl Settings {
    pub fn read(order: &esm::LoadOrder) -> Settings {
        let d = Settings::default();
        let g = |n: &str, v: f32| crate::scripting::game_setting(order, n).unwrap_or(v);
        Settings {
            non_attack_mult: g("fNonAttackGunWobbleMult", d.non_attack_mult),
            scope_mult: g("fGunWobbleMultScope", d.scope_mult),
            chase_drift: g("fGunWobbleChaseDriftTime", d.chase_drift),
        }
    }
}

/// The kept amount (`011a3b2c`) one frame on toward `target`: by
/// `drift` × `dt` at most; reached at once when that's enough, or when
/// it's above and the player is attacking.
// Translated from 00962de0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn chase(current: f32, target: f32, attacking: bool, dt: f32, drift: f32) -> f32 {
    let diff = target - current;
    let step = dt * drift;
    if diff == 0.0 {
        current
    } else if diff.abs() <= step {
        target
    } else if diff <= 0.0 {
        current - step
    } else if attacking {
        target
    } else {
        current + step
    }
}

/// A 3 × 3 rotation as Gamebryo keeps it (rows).
pub type Matrix = [[f32; 3]; 3];

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut m = [[0.0; 3]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    m
}

fn transpose(a: &Matrix) -> Matrix {
    let mut m = [[0.0; 3]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = a[j][i];
        }
    }
    m
}

/// `NiMatrix3::FromEulerAnglesXYZ` (`00a59540`): X [1 0 0; 0 c s; 0 −s c]
/// · Y [c 0 −s; 0 1 0; s 0 c] · Z [c s 0; −s c 0; 0 0 1].
// Translated from 00a59540 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn from_euler(x: f32, y: f32, z: f32) -> Matrix {
    let (sx, cx) = x.sin_cos();
    let (sy, cy) = y.sin_cos();
    let (sz, cz) = z.sin_cos();
    let rx = [[1.0, 0.0, 0.0], [0.0, cx, sx], [0.0, -sx, cx]];
    let ry = [[cy, 0.0, -sy], [0.0, 1.0, 0.0], [sy, 0.0, cy]];
    let rz = [[cz, sz, 0.0], [-sz, cz, 0.0], [0.0, 0.0, 1.0]];
    mul(&rx, &mul(&ry, &rz))
}

/// `NiMatrix3::ToEulerAnglesXYZ` (`00a592c0`): y = −asin(m02) (±π/2 at
/// the ends), x = −atan2(−m12, m22), z = −atan2(−m01, m00) (the gimbal
/// cases: x 0 … as the exe, z from m10, m11).
// Translated from 00a592c0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn to_euler(m: &Matrix) -> [f32; 3] {
    let half = std::f32::consts::FRAC_PI_2;
    let s = m[0][2];
    let y = -(if s <= -1.0 {
        -half
    } else if s >= 1.0 {
        half
    } else {
        s.asin()
    });
    if y < half && y > -half {
        let x = -(-m[1][2]).atan2(m[2][2]);
        let z = -(-m[0][1]).atan2(m[0][0]);
        [x, y, z]
    } else if y >= half {
        [-m[1][0].atan2(m[1][1]), y, 0.0]
    } else {
        [m[1][0].atan2(m[1][1]) - std::f32::consts::PI, y, 0.0]
    }
}

/// The sway's rotation from the model's angles now and at the base time
/// (the controller's start, or for the scope the last frame's), scaled by
/// `amount`: R(now)·R(base)ᵀ as angles × amount, made a rotation again.
pub fn sway_angles(now: [f32; 3], base: [f32; 3], amount: f32) -> [f32; 3] {
    let r = mul(
        &from_euler(now[0], now[1], now[2]),
        &transpose(&from_euler(base[0], base[1], base[2])),
    );
    to_euler(&r).map(|a| a * amount)
}

/// [`sway_angles`] as the rotation set on the bone's controller.
pub fn sway_rotation(now: [f32; 3], base: [f32; 3], amount: f32) -> Matrix {
    let [x, y, z] = sway_angles(now, base, amount);
    from_euler(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_files_and_the_chase() {
        // Rifles (5) and automatic rifles (6), pistols (3, 4).
        assert_eq!(FILES[wobble_kind(5)], "2HR");
        assert_eq!(FILES[wobble_kind(6)], "2HA");
        assert_eq!(FILES[wobble_kind(3)], "1HP");
        assert_eq!(FILES[wobble_kind(0)], "H2H");
        assert_eq!(
            model_path(0).as_deref(),
            Some("Meshes\\Characters\\WeaponWobbles\\ScopeWobble.nif")
        );
        // 0.75 a second: up by 0.0125 in a 60th, down likewise; reached.
        let c = chase(1.0, 0.1, false, 1.0 / 60.0, 0.75);
        assert!((c - 0.9875).abs() < 1e-6);
        let c = chase(0.1, 0.5, false, 1.0 / 60.0, 0.75);
        assert!((c - 0.1125).abs() < 1e-6);
        assert_eq!(chase(0.1, 0.5, true, 1.0 / 60.0, 0.75), 0.5);
        assert_eq!(chase(0.1, 0.105, false, 1.0 / 60.0, 0.75), 0.105);
    }

    #[test]
    fn euler_angles_go_round_trip_and_scale() {
        let a = [0.1f32, -0.05, 0.2];
        let m = from_euler(a[0], a[1], a[2]);
        let b = to_euler(&m);
        for i in 0..3 {
            assert!((a[i] - b[i]).abs() < 1e-5, "{a:?} {b:?}");
        }
        // Relative to itself: nothing; half the amount of a small turn.
        assert!(sway_angles(a, a, 1.0).iter().all(|v| v.abs() < 1e-5));
        let half = sway_angles([0.0, 0.0, 0.1], [0.0; 3], 0.5);
        assert!((half[2] - 0.05).abs() < 1e-5 && half[0].abs() < 1e-6);
    }
}
