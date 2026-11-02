//! The player's grab (the "Grab" control, Z by default): picking up a free
//! rigid body under the crosshair and carrying it on Havok's mouse spring
//! ([`crate::rigid::Spring`]), as `PlayerCharacter::HandlePhysicsGrab`,
//! `::CreateMouseSpring`, `::UpdateMouseSpring` and `::DestroyMouseSpring`
//! (Xbox PDB; `0095f6c0`, `0095f930`, `00960520`, `00961280`) do it.
//!
//! - The Grab control pressed with nothing held grabs the crosshair's
//!   reference when its body moves (not keyframed or fixed) and weighs at
//!   most `fGrabMaxWeightWalking` (100); pressed again, it lets go. A
//!   released body keeps the velocity the spring gave it: the game has no
//!   throw (no setting or code path for one was found).
//! - The held point is where the crosshair's pick met the body, held at
//!   that distance from the eye, but no nearer than the player's
//!   controller radius + 5 (`00c6e280` + 5.0 at `01020998`).
//! - Each frame the target is the eye + the view × that distance, cut
//!   short where a cast along the view meets something else first
//!   (`00960520`'s pick loop).
//! - It lets go when the held point is more than 96 units from the target
//!   (`01072f98`), or more than `fZKeyMaxContactDistance` (10) while
//!   touching a body more than `fZKeyMaxContactMassRatio` (4) times as
//!   heavy.
//! - The spring: `fZKeySpringDamping` (0.5), `fZKeySpringElasticity`
//!   (0.2), `fZKeyObjectDamping` (0.75), `fZKeyMaxForce` (750); on the trap
//!   layer (14) the elasticity × 0.1 and the force × 0.5.
//!
//! Not reproduced (read, labelled): actors and ragdolls (the "complex"
//! spring, `fZKeyMaxForceScale…`/`WeightHigh/Low`, and the complex
//! helper's extra pull, `fZKeyComplexHelper…`); letting go when the player
//! stands on what they hold (`00961260`); the keep-out from the player's
//! own controller when the target is below the eye (`0063c8a0`); the hold
//! of the Activate control as a second way to grab (`00705a90`, its states
//! not traced); `fGrabMaxWeightRunning` (read outside any function).

use crate::vec::*;
use crate::Vec3;

/// The grab's game settings, with the executable's defaults
/// (`settings_all.txt`; none is set by `FalloutNV.esm`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrabSettings {
    /// `fZKeySpringDamping`.
    pub spring_damping: f32,
    /// `fZKeySpringElasticity`.
    pub spring_elasticity: f32,
    /// `fZKeyObjectDamping`.
    pub object_damping: f32,
    /// `fZKeyMaxForce`.
    pub max_force: f32,
    /// `fZKeyMaxContactMassRatio`.
    pub max_contact_mass_ratio: f32,
    /// `fZKeyMaxContactDistance`.
    pub max_contact_distance: f32,
    /// `fGrabMaxWeightWalking`.
    pub max_weight: f32,
}

impl Default for GrabSettings {
    fn default() -> Self {
        GrabSettings {
            spring_damping: 0.5,
            spring_elasticity: 0.2,
            object_damping: 0.75,
            max_force: 750.0,
            max_contact_mass_ratio: 4.0,
            max_contact_distance: 10.0,
            max_weight: 100.0,
        }
    }
}

impl GrabSettings {
    /// The settings by name, each from `get` or its default.
    pub fn read(get: impl Fn(&str) -> Option<f32>) -> Self {
        let d = Self::default();
        let f = |name: &str, default: f32| get(name).unwrap_or(default);
        GrabSettings {
            spring_damping: f("fZKeySpringDamping", d.spring_damping),
            spring_elasticity: f("fZKeySpringElasticity", d.spring_elasticity),
            object_damping: f("fZKeyObjectDamping", d.object_damping),
            max_force: f("fZKeyMaxForce", d.max_force),
            max_contact_mass_ratio: f("fZKeyMaxContactMassRatio", d.max_contact_mass_ratio),
            max_contact_distance: f("fZKeyMaxContactDistance", d.max_contact_distance),
            max_weight: f("fGrabMaxWeightWalking", d.max_weight),
        }
    }
}

/// How far past the player's controller radius the held point stays
/// (5.0, `01020998`).
pub const HOLD_PAST_RADIUS: f32 = 5.0;
/// The held point this far (game units) from its target lets go
/// (`01072f98`, 96).
pub const RELEASE_DISTANCE: f32 = 96.0;
/// The trap layer (14), whose bodies are held more loosely.
const TRAP: u8 = 14;

/// Whether a body of `mass` that moves can be grabbed: at most
/// `fGrabMaxWeightWalking`.
// Translated from 0095f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn may_grab(mass: f32, moves: bool, s: &GrabSettings) -> bool {
    // The x87 test `(m < max) != (m == max)` holds for m ≤ max.
    moves && mass <= s.max_weight
}

/// The distance the held point is kept from the eye: where it was picked,
/// at least the controller radius + 5.
pub fn hold_distance(picked: f32, controller_radius: f32) -> f32 {
    picked.max(controller_radius + HOLD_PAST_RADIUS)
}

/// The spring's damping, elasticity, object damping and most relative
/// force for a (non-actor) body on `layer`.
// Translated from 0095f930 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn spring_values(layer: u8, s: &GrabSettings) -> (f32, f32, f32, f32) {
    let (mut elastic, mut force) = (1.0f32, 1.0f32);
    if layer == TRAP {
        elastic *= 0.1;
        force *= 0.5;
    }
    (
        s.spring_damping,
        s.spring_elasticity * elastic,
        s.object_damping,
        s.max_force * force,
    )
}

/// The spring's target this frame: the eye + the view × the hold distance,
/// cut to the first thing a cast along the view meets (`blocked`: how far,
/// if anything else is in the way).
// Translated from 00960520 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn target(eye: Vec3, view: Vec3, distance: f32, blocked: Option<f32>) -> Vec3 {
    let d = match blocked {
        Some(b) if b < distance => b.max(0.0),
        _ => distance,
    };
    add(eye, scale(view, d))
}

/// Whether the grab lets go: the held point `gap` units from its target,
/// touching bodies whose heaviest weighs `heaviest_contact` against the
/// held body's `mass`.
// Translated from 00960520 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn lets_go(gap: f32, heaviest_contact: f32, mass: f32, s: &GrabSettings) -> bool {
    if gap > RELEASE_DISTANCE {
        return true;
    }
    let ratio = if mass > 0.0 {
        heaviest_contact / mass
    } else {
        0.0
    };
    gap > s.max_contact_distance && ratio > s.max_contact_mass_ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_can_be_grabbed_and_how_it_is_held() {
        let s = GrabSettings::default();
        assert!(may_grab(1.0, true, &s));
        assert!(may_grab(100.0, true, &s));
        assert!(!may_grab(100.5, true, &s));
        assert!(!may_grab(1.0, false, &s));
        // People's controller radius 20.25: held at least 25.25 away.
        assert_eq!(hold_distance(10.0, 20.25), 25.25);
        assert_eq!(hold_distance(80.0, 20.25), 80.0);
        assert_eq!(spring_values(10, &s), (0.5, 0.2, 0.75, 750.0));
        let (_, e, _, f) = spring_values(14, &s);
        assert!((e - 0.02).abs() < 1e-6 && (f - 375.0).abs() < 1e-3);
    }

    #[test]
    fn the_target_stops_short_of_walls_and_the_grab_lets_go() {
        let t = target([0.0; 3], [0.0, 1.0, 0.0], 80.0, Some(30.0));
        assert_eq!(t, [0.0, 30.0, 0.0]);
        assert_eq!(
            target([0.0; 3], [0.0, 1.0, 0.0], 80.0, None),
            [0.0, 80.0, 0.0]
        );
        let s = GrabSettings::default();
        assert!(lets_go(97.0, 0.0, 1.0, &s));
        assert!(!lets_go(50.0, 0.0, 1.0, &s));
        // Pinned against something five times heavier, 11 units off.
        assert!(lets_go(11.0, 5.0, 1.0, &s));
        assert!(!lets_go(9.0, 5.0, 1.0, &s));
    }
}
