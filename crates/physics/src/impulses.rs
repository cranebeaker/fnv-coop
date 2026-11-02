//! The pushes the game gives free rigid bodies ([`crate::rigid`]): a shot's
//! impact force and an explosion's push, scaled by the struck body's layer
//! and mass as `TESHavokUtilities::ScaleGameplayImpulseForce` (Xbox PDB)
//! scales them.
//!
//! Impulses here are in Havok's units (mass × Havok units a second; a Havok
//! unit is [`crate::HAVOK_UNIT`] game units), as the game hands them to
//! `hkpRigidBody::applyPointImpulse` and its relatives; [`crate::rigid`]
//! turns them into game units.
//!
//! Settings: the executable's defaults (`settings_all.txt`); none is set by
//! `FalloutNV.esm`'s `GMST`s as far as `nvinspect` shows, but callers read
//! the load order's values into [`ImpulseSettings`] in case a plugin does.

use crate::vec::*;
use crate::Vec3;

/// Havok collision layers the scaling tells apart (`nif::collision::layers`).
mod layer {
    pub const BIPED: u8 = 8;
    pub const PROPS: u8 = 10;
    pub const TRAP: u8 = 14;
    pub const DEBRIS_LARGE: u8 = 20;
    pub const DEAD_BIPED: u8 = 29;
}

/// The game settings the pushes use, with the executable's defaults.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImpulseSettings {
    /// `fGameplayImpulseScale` (150).
    pub scale: f32,
    /// `fGameplayImpulseMultTrap` (0.15), `…Biped` (0.2), `…Prop` (0.1),
    /// `…Clutter` (1), `…DebrisLarge` (1).
    pub mult_trap: f32,
    pub mult_biped: f32,
    pub mult_prop: f32,
    pub mult_clutter: f32,
    pub mult_debris_large: f32,
    /// `fGameplayImpulseMinMass` (5): lighter bodies get a share of the
    /// push, their mass over this.
    pub min_mass: f32,
    /// `fExplosionMaxImpulse` (8000).
    pub explosion_max: f32,
    /// `fExplosionForceClutterUpBias` (0.5).
    pub explosion_up_bias: f32,
    /// `fExplosionForceMultLinear` (1), `fExplosionForceMultAngular` (1).
    pub explosion_mult_linear: f32,
    pub explosion_mult_angular: f32,
    /// `fExplosionSourceRefMult` (1).
    pub explosion_source_ref_mult: f32,
}

impl Default for ImpulseSettings {
    fn default() -> Self {
        ImpulseSettings {
            scale: 150.0,
            mult_trap: 0.15,
            mult_biped: 0.2,
            mult_prop: 0.1,
            mult_clutter: 1.0,
            mult_debris_large: 1.0,
            min_mass: 5.0,
            explosion_max: 8000.0,
            explosion_up_bias: 0.5,
            explosion_mult_linear: 1.0,
            explosion_mult_angular: 1.0,
            explosion_source_ref_mult: 1.0,
        }
    }
}

impl ImpulseSettings {
    /// The settings by name (`fGameplayImpulseScale`…), each from `get` or
    /// its default.
    pub fn read(get: impl Fn(&str) -> Option<f32>) -> Self {
        let d = Self::default();
        let f = |name: &str, default: f32| get(name).unwrap_or(default);
        ImpulseSettings {
            scale: f("fGameplayImpulseScale", d.scale),
            mult_trap: f("fGameplayImpulseMultTrap", d.mult_trap),
            mult_biped: f("fGameplayImpulseMultBiped", d.mult_biped),
            mult_prop: f("fGameplayImpulseMultProp", d.mult_prop),
            mult_clutter: f("fGameplayImpulseMultClutter", d.mult_clutter),
            mult_debris_large: f("fGameplayImpulseMultDebrisLarge", d.mult_debris_large),
            min_mass: f("fGameplayImpulseMinMass", d.min_mass),
            explosion_max: f("fExplosionMaxImpulse", d.explosion_max),
            explosion_up_bias: f("fExplosionForceClutterUpBias", d.explosion_up_bias),
            explosion_mult_linear: f("fExplosionForceMultLinear", d.explosion_mult_linear),
            explosion_mult_angular: f("fExplosionForceMultAngular", d.explosion_mult_angular),
            explosion_source_ref_mult: f("fExplosionSourceRefMult", d.explosion_source_ref_mult),
        }
    }
}

/// `TESHavokUtilities::ScaleGameplayImpulseForce` (Xbox PDB): a push of
/// `force` on a body on `layer` weighing `mass`: × the layer's multiplier
/// (biped and dead biped, props, traps, large debris, else clutter's); a
/// body lighter than `fGameplayImpulseMinMass` gets mass ÷ that of it when
/// `by_mass`; then × `fGameplayImpulseScale`.
// Translated from 0062b520 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn scale_gameplay_impulse(
    force: f32,
    layer: u8,
    mass: f32,
    by_mass: bool,
    s: &ImpulseSettings,
) -> f32 {
    let mult = match layer {
        layer::BIPED | layer::DEAD_BIPED => s.mult_biped,
        layer::PROPS => s.mult_prop,
        layer::TRAP => s.mult_trap,
        layer::DEBRIS_LARGE => s.mult_debris_large,
        _ => s.mult_clutter,
    };
    let mut m = mult;
    if by_mass && mass < s.min_mass {
        m *= mass / s.min_mass;
    }
    m * s.scale * force
}

/// Whether a body's motion is one Havok moves (the game's motion types
/// below 4: dynamic, sphere and box inertia; `00517630` gives 5, fixed,
/// for a reference without a body). The NIF's motion systems: everything
/// but keyframed (6), fixed (7), character (9) and invalid (0).
pub fn moves(motion: u8) -> bool {
    !matches!(motion, 0 | 6 | 7 | 9)
}

/// A projectile's push on the body it strikes
/// (`Projectile::ApplyImpactForce` (Xbox PDB)): along its velocity
/// (`+0x104`, normalized when it has length), of its record's impact force
/// (`PROJ` `DATA` "impact force", the form's `+0x94`, `00644930`) scaled
/// by [`scale_gameplay_impulse`] with the mass; nothing for a force of 0.
/// The game applies it at the impact point to bodies that move
/// ([`moves`]), from `Projectile::ProcessImpacts` (`009c1b70`) for every
/// impact of a projectile that doesn't explode on impact.
// Translated from 009c2e80 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn projectile_impulse(
    impact_force: f32,
    velocity: Vec3,
    layer: u8,
    mass: f32,
    s: &ImpulseSettings,
) -> Option<Vec3> {
    let force = scale_gameplay_impulse(impact_force, layer, mass, true, s);
    if force == 0.0 {
        return None;
    }
    let l = length(velocity);
    let dir = if l > 0.0 {
        scale(velocity, 1.0 / l)
    } else {
        velocity
    };
    Some(scale(dir, force))
}

/// An explosion's push on one body in its sphere
/// (`Explosion::PushRigidBody` (Xbox PDB)): nothing unless the record's
/// force is positive; the direction from the explosion to the body,
/// normalized (zero when they coincide, `004a0c10`), its z raised by
/// `fExplosionForceClutterUpBias` on the biped and dead-biped layers
/// (`00624070` tests those two, whatever the setting's name says); the
/// force × `fExplosionSourceRefMult` when the body belongs to the
/// explosion's source reference (`+0xcc`), then scaled by
/// [`scale_gameplay_impulse`] with the mass, and held to
/// `fExplosionMaxImpulse` unless it was the source's. The linear impulse is
/// the direction × the force × `fExplosionForceMultLinear` (applied at the
/// centre: `hkpRigidBody::applyLinearImpulse`); the angular one a random
/// vector (each component from −1 to 1, `00476b70`) × the force ×
/// `fExplosionForceMultAngular`. `random` gives those components.
// Translated from 009b0920 (decompiled, FalloutNV.exe 1.4.0.525)
#[allow(clippy::too_many_arguments)]
pub fn explosion_push(
    force: f32,
    from: Vec3,
    body_at: Vec3,
    layer: u8,
    mass: f32,
    source_body: bool,
    s: &ImpulseSettings,
    mut random: impl FnMut() -> f32,
) -> Option<(Vec3, Vec3)> {
    if force <= 0.0 {
        return None;
    }
    let mut dir = sub(body_at, from);
    let l = length(dir);
    dir = if l > 1e-6 {
        scale(dir, 1.0 / l)
    } else {
        [0.0; 3]
    };
    if matches!(layer, layer::BIPED | layer::DEAD_BIPED) {
        dir[2] += s.explosion_up_bias;
    }
    let mut f = force;
    if source_body {
        f *= s.explosion_source_ref_mult;
    }
    f = scale_gameplay_impulse(f, layer, mass, true, s);
    if !source_body {
        f = f.min(s.explosion_max);
    }
    let linear = scale(dir, f * s.explosion_mult_linear);
    let spin = [random(), random(), random()];
    let angular = scale(spin, f * s.explosion_mult_angular);
    Some((linear, angular))
}

/// Which of an explosion's bodies it pushes (`Explosion::ApplyForces`
/// (Xbox PDB), `009afef0`): those that move ([`moves`]) and large debris,
/// unless the explosion pushes its source only (`EXPL` flag 0x20) and the
/// body isn't its source's. Dead bipeds take a separate path there
/// (ragdolls), not this one.
// Translated from 009afef0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn explosion_pushes(motion: u8, layer: u8, push_source_only: bool, source_body: bool) -> bool {
    if push_source_only && !source_body {
        return false;
    }
    layer != layer::DEAD_BIPED && (moves(motion) || layer == layer::DEBRIS_LARGE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gameplay_impulses_scale_by_layer_and_light_mass() {
        let s = ImpulseSettings::default();
        // Clutter of 10: × 1 × 150.
        assert_eq!(scale_gameplay_impulse(2.0, 4, 10.0, true, &s), 300.0);
        // A prop of mass 1 (the VCG02 bottle, layer 10): × 0.1 × 1/5 × 150.
        let f = scale_gameplay_impulse(2.0, 10, 1.0, true, &s);
        assert!((f - 6.0).abs() < 1e-4, "{f}");
        // Without the mass share.
        assert!((scale_gameplay_impulse(2.0, 10, 1.0, false, &s) - 30.0).abs() < 1e-4);
        // Bipeds, traps, large debris.
        assert!((scale_gameplay_impulse(1.0, 8, 50.0, true, &s) - 30.0).abs() < 1e-4);
        assert!((scale_gameplay_impulse(1.0, 29, 50.0, true, &s) - 30.0).abs() < 1e-4);
        assert!((scale_gameplay_impulse(1.0, 14, 50.0, true, &s) - 22.5).abs() < 1e-4);
        assert!((scale_gameplay_impulse(1.0, 20, 50.0, true, &s) - 150.0).abs() < 1e-4);
    }

    #[test]
    fn a_shot_pushes_along_its_flight() {
        let s = ImpulseSettings::default();
        let j = projectile_impulse(1.0, [0.0, 3000.0, 0.0], 4, 20.0, &s).unwrap();
        assert!(
            (j[1] - 150.0).abs() < 1e-3 && j[0] == 0.0 && j[2] == 0.0,
            "{j:?}"
        );
        assert!(projectile_impulse(0.0, [0.0, 1.0, 0.0], 4, 20.0, &s).is_none());
    }

    #[test]
    fn explosions_push_away_capped_and_spin() {
        let s = ImpulseSettings::default();
        let mut n = 0;
        let mut random = || {
            n += 1;
            [1.0, -1.0, 0.5][(n - 1) % 3]
        };
        // Force 90 (frag dynamite) on 20 kg of clutter 100 units east: 90 ×
        // 150 = 13500, held to 8000, pushing east, no up-bias.
        let (lin, ang) = explosion_push(
            90.0,
            [0.0; 3],
            [100.0, 0.0, 0.0],
            4,
            20.0,
            false,
            &s,
            &mut random,
        )
        .unwrap();
        assert!((lin[0] - 8000.0).abs() < 1e-2 && lin[2] == 0.0, "{lin:?}");
        assert_eq!(ang, [8000.0, -8000.0, 4000.0]);
        // A dead body gets the up-bias; the source's own body isn't capped.
        let (lin, _) = explosion_push(
            90.0,
            [0.0; 3],
            [100.0, 0.0, 0.0],
            29,
            20.0,
            true,
            &s,
            || 0.0,
        )
        .unwrap();
        assert!(
            (lin[0] - 2700.0).abs() < 1e-2 && (lin[2] - 1350.0).abs() < 1e-2,
            "{lin:?}"
        );
        assert!(
            explosion_push(0.0, [0.0; 3], [1.0, 0.0, 0.0], 4, 1.0, false, &s, || 0.0).is_none()
        );
        // Which bodies: moving ones and large debris; source-only blasts.
        assert!(explosion_pushes(4, 4, false, false));
        assert!(!explosion_pushes(7, 1, false, false));
        assert!(explosion_pushes(7, 20, false, false));
        assert!(!explosion_pushes(4, 29, false, false));
        assert!(!explosion_pushes(4, 4, true, false));
        assert!(explosion_pushes(4, 4, true, true));
    }
}
