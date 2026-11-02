//! People's gunfire and their next target, read from the game's code
//! (`FalloutNV.exe` 1.4.0.525; names marked (Xbox PDB) come from the Xbox
//! 360 prototype's symbols). Evidence: `docs/NPC_COMBAT.md`.
//!
//! A shot by someone in combat (`00523150`, the weapon's fire, the branch
//! for actors other than the player with a combat controller):
//!
//! - it leaves the actor's fire node (`Actor::GetFireNode`, Xbox PDB, vtable
//!   +0x20c on PC), else the point 0.75 of their height above their feet
//!   (`009a8050` → `009a7fa0`; [`fire_height`]);
//! - it is aimed straight at the point the ranged attack procedure targets
//!   (`CombatController::GetTargetedPoint`, Xbox PDB, `009807f0` → the action
//!   procedure's vtable +0x18): the target's feet raised by a share of its
//!   height chosen by which part of it is in view ([`aim_height`],
//!   `009a8600`/`009a8460`), led by the target's velocity only for
//!   projectiles that aren't hitscan (`009a8df0`);
//! - then turned at random within a cone: the weapon's cone (its min
//!   spread and the ammunition's spread effects, `world::combat::Weapon::
//!   shot`) + the shooter's gun wobble (`008b0dd0` mode 2 →
//!   `00646910`, `world::vats::wobble`) × `fNPCMaxGunWobbleAngle` (data 15)
//!   in degrees, + the process's aim offset (`+0x1d0`, decaying to 0,
//!   `009295c0`; taken as 0 here) ([`npc_cone`]); the turn is uniform in its
//!   angle off the aim and around it ([`deviate`]).
//!
//! The wobble's inputs for people: walking (movement flag 0x100) or
//! running (0x200), sneaking, and aiming down the sights, which the combat
//! AI turns on when the target is farther than `fCombatIronSightsDistance`
//! (512) × the weapon's sight usage (`DNAM` f32 at 124, the weapon's
//! `+0x170`; `008f74c0`, `009d0a30`; [`uses_iron_sights`]).
//!
//! Not read anywhere in the code (the settings inventory lists only their
//! initialisers): the `fGunSpread*` settings and the
//! `fWeaponConditionSpread1–10` table, so a weapon's condition doesn't
//! change its spread; `fAICombat…PriorityMult` and
//! `fCombatRangedStandoffTimer` likewise.
//!
//! After losing a target (killed or given up), the combat controller asks
//! its combat group for the best of the group's targets
//! (`CombatController::RemoveTarget`-like `0097f4d0` →
//! `CombatGroup::GetBestTarget`, Xbox PDB, `00986c60`; [`best_target`]); the
//! same choice is made again whenever the controller plans anew
//! (`CombatController::Update`, Xbox PDB, `0097da50`).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_f32;
use crate::combat_ai::Setting;

/// Which part of a target is in view, the index `009a8460` takes: 0 the
/// lower, 1 the middle (the default when there is no controller), 2 the
/// upper, 3 as 1. The controller's own value (`CombatController` → its
/// combat state's `+0x7c`, `00981920`) is set by its line-of-sight checks,
/// which aren't traced.
pub const SEGMENT_MIDDLE: u8 = 1;

/// Height above a target's feet that a shot aims at: a share of its
/// height by the part in view; 0.1 of it when the target is dead, knocked
/// out or paralysed; the upper part's share is 0.9 of the crouched height
/// when the target sneaks (`crouched`).
// Translated from 009a8460 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn aim_height(height: f32, segment: u8, down: bool, crouched: bool) -> f32 {
    if down {
        // `0101ffa0` (double 0.1).
        return height * 0.1;
    }
    match segment {
        // `010290b0` (double 0.25).
        0 => height * 0.25,
        // `01011588` (double 0.5).
        1 | 3 => height * 0.5,
        // `0101de30` (0.75), `0106b9e8` (0.9) for the crouched height.
        2 => height * if crouched { 0.9 } else { 0.75 },
        _ => 0.0,
    }
}

/// Height above an actor's feet that its shots leave from when it has no
/// fire node: 0.75 of its height (`0101de30`).
// Translated from 009a7fa0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn fire_height(height: f32) -> f32 {
    height * 0.75
}

/// An actor's height as the aim reads it (`008853a0`): its base's bounds
/// (`OBND` top − bottom) × its scale.
pub fn actor_height(order: &LoadOrder, base: FormId, scale: f32) -> Option<f32> {
    let [bottom, top] = bound_z(order, base)?;
    Some(((top - bottom) * scale).max(0.0)).filter(|h| *h > 0.0)
}

/// The bottom and top of an actor's bounds (vtable `+0x1d8` min and
/// `+0x1dc` max, z): its base's `OBND` z1 and z2, unscaled.
pub fn bound_z(order: &LoadOrder, base: FormId) -> Option<[f32; 2]> {
    let record = order.get(base)?.record().ok()?;
    let s = record
        .get(FourCC::new(b"OBND"))
        .filter(|s| s.data.len() >= 12)?;
    let z = |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
    Some([z(2), z(5)])
}

/// A weapon's sight usage (`DNAM` f32 at 124; the weapon's `+0x170`, read
/// by `008f7710`); 0 when the record has none.
pub fn sight_usage(order: &LoadOrder, weapon: FormId) -> f32 {
    order
        .get(weapon)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(FourCC::new(b"DNAM")).map(|s| s.data.clone()))
        .filter(|d| d.len() >= 128)
        .map_or(0.0, |d| le_f32(&d, 124))
}

/// Whether the combat AI aims down the sights at a target this far away
/// (the aim point less the actor's position, `004a7290` length squared):
/// farther than `fCombatIronSightsDistance` × the weapon's sight usage.
/// Melee weapons and fists never do (`008f74c0`: `006450c0`).
// Translated from 008f74c0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn uses_iron_sights(distance: f32, sight_usage: f32, melee: bool, s: Setting) -> bool {
    if melee {
        return false;
    }
    let reach = s("fCombatIronSightsDistance", 512.0) * sight_usage;
    reach * reach < distance * distance
}

/// The cone (radians, its half-angle as the game adds it) a person's shot
/// flies in: the weapon's own cone (`world::combat::Weapon::shot`) + the
/// shooter's wobble (`world::vats::wobble`) × `fNPCMaxGunWobbleAngle`
/// degrees. The process's aim offset (`+0x1d0`) is taken as 0: it is
/// written only by `008d8350` (from code not followed) and decays to 0 in
/// `009295c0`.
// Translated from 00523150 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn npc_cone(weapon_cone: f32, wobble: f32, max_wobble_degrees: f32) -> f32 {
    // The double at `01023128` is π/180.
    weapon_cone + wobble * max_wobble_degrees * std::f32::consts::PI / 180.0
}

/// A shot turned within `cone`: r = `u_r` × cone off the aim, at the angle
/// `u_turn` × 2π around it, added to heading and pitch (radians). Which of
/// sine and cosine goes to the pitch isn't identified (`00eca060`,
/// `00ec9f30`); the spread is the same either way.
// Translated from 00523150 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn deviate(heading: f32, pitch: f32, cone: f32, u_r: f32, u_turn: f32) -> (f32, f32) {
    let r = cone * u_r;
    let turn = std::f32::consts::TAU * u_turn;
    (heading + r * turn.cos(), pitch + r * turn.sin())
}

/// Heading (clockwise from north) and pitch (up positive) from `from` to
/// `to`, and the unit direction for a heading and pitch.
pub fn heading_pitch(from: [f32; 3], to: [f32; 3]) -> (f32, f32) {
    let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let flat = d[0].hypot(d[1]);
    (d[0].atan2(d[1]), d[2].atan2(flat))
}

pub fn direction(heading: f32, pitch: f32) -> [f32; 3] {
    [
        heading.sin() * pitch.cos(),
        heading.cos() * pitch.cos(),
        pitch.sin(),
    ]
}

/// One of a combat group's targets as `CombatGroup::GetBestTarget` weighs
/// it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetCandidate {
    pub reference: FormId,
    /// The group's detection level for it (`009887b0`: the highest of the
    /// members').
    pub detection: i32,
    /// The chooser's detection record: line of sight within its view
    /// (`+0x1e`, counted as `CombatTarget::cMemberLOSCount`, Xbox PDB) and
    /// all round (`+0x1c`, `cMember360LOSCount`).
    pub in_view: bool,
    pub in_sight: bool,
    /// It is the chooser's current target; and, for that one, it was seen
    /// within the last 2 s (`011a4d6c`; the controller's `+0x98` time).
    pub current: bool,
    pub seen_recently: bool,
    /// The chooser and it both fight in melee, or both don't (`009a9630`:
    /// no weapon or a melee one).
    pub same_kind: bool,
    /// Squared distance between the two.
    pub distance_sq: f32,
    /// Dead or knocked out (vtable +0x22c `IsDead`, +0x230 `IsKnockedOut`,
    /// Xbox PDB).
    pub down: bool,
    /// More attackers on it than the chooser (`CombatTarget::sAttackerCount`,
    /// Xbox PDB, above 1 for the current target, above 0 otherwise).
    pub attacked_by_others: bool,
    /// When it was last detected (seconds), for the choice among
    /// undetected targets.
    pub last_detected: f32,
}

/// The target a combat group member fights next: among those with a
/// detection level of at least 1, the highest score (when the group has
/// more than one target; else 0), adding 1000 in view, 100 in sight, 100
/// for the current target (1000 more when out of view but seen within
/// 2 s), 100 when both fight alike, (1 − min(d², 2048²) ÷ 2048²) × 1000,
/// 0 when others attack it too (`011f18b0` is 0 in the file and never
/// written), and taking 500 when it is down. With none detected, the most
/// recently detected (the timestamp comparison is inferred from the
/// decompiler's output), else the last one weighed. Not modelled: the
/// style's targeting field of view, which the caller applies; teammates'
/// rule (`00566950` → `008b06d0`), the cell checks (the flag `00408d60`
/// returns, `0084e3a0`), the unreachable-location penalty (2000,
/// `009a0f40`), the unloaded-area penalty (2500, `009a96f0`) and
/// `008a6650` (500).
// Translated from 00986c60 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn best_target(candidates: &[TargetCandidate]) -> Option<FormId> {
    // `0108d540` FLT_MAX; `011a4d50`–`011a4d78` the weights.
    let mut best_score = -f32::MAX;
    let mut best = None;
    let mut freshest: Option<(f32, FormId)> = None;
    let mut last = None;
    let several = candidates.len() > 1;
    for c in candidates {
        if c.detection < 1 {
            if freshest.map_or(true, |(t, _)| c.last_detected > t) {
                freshest = Some((c.last_detected, c.reference));
            }
        } else {
            let mut score = 0.0;
            if several {
                if c.in_view {
                    score += 1000.0;
                }
                if c.in_sight {
                    score += 100.0;
                }
                if c.current {
                    score += 100.0;
                    if !c.in_view && c.seen_recently {
                        score += 1000.0;
                    }
                }
                if c.same_kind {
                    score += 100.0;
                }
                // `0108d660` 2048² (float), `0108d658` 2048² (double).
                const FAR_SQ: f32 = 2048.0 * 2048.0;
                score += (1.0 - c.distance_sq.min(FAR_SQ) / FAR_SQ) * 1000.0;
                if c.attacked_by_others {
                    score += 0.0;
                }
                if c.down {
                    score -= 500.0;
                }
            }
            if best_score < score {
                best_score = score;
                best = Some(c.reference);
            }
        }
        last = Some(c.reference);
    }
    best.or(freshest.map(|f| f.1)).or(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(n: u32) -> TargetCandidate {
        TargetCandidate {
            reference: FormId(n),
            detection: 50,
            in_view: false,
            in_sight: false,
            current: false,
            seen_recently: false,
            same_kind: false,
            distance_sq: 0.0,
            down: false,
            attacked_by_others: false,
            last_detected: 0.0,
        }
    }

    #[test]
    fn aims_at_the_middle_of_a_standing_target_and_low_at_a_fallen_one() {
        assert_eq!(aim_height(128.0, SEGMENT_MIDDLE, false, false), 64.0);
        assert_eq!(aim_height(128.0, 0, false, false), 32.0);
        assert_eq!(aim_height(128.0, 2, false, false), 96.0);
        assert!((aim_height(100.0, 2, false, true) - 90.0).abs() < 1e-4);
        assert!((aim_height(128.0, 2, true, false) - 12.8).abs() < 1e-4);
        assert_eq!(aim_height(128.0, 4, false, false), 0.0);
        assert_eq!(fire_height(128.0), 96.0);
    }

    #[test]
    fn iron_sights_past_the_weapons_sight_distance() {
        let s = |_: &str, d: f32| d;
        // Sight usage 1: past 512 units.
        assert!(!uses_iron_sights(500.0, 1.0, false, &s));
        assert!(uses_iron_sights(600.0, 1.0, false, &s));
        // Sight usage 2: past 1024.
        assert!(!uses_iron_sights(1000.0, 2.0, false, &s));
        // Never with a melee weapon.
        assert!(!uses_iron_sights(5000.0, 1.0, true, &s));
    }

    #[test]
    fn the_cone_adds_the_wobble_in_degrees() {
        // Guns 50, standing and aiming: wobble 0.1 × 0.75 = 0.075; × 15° =
        // 1.125°, on a 0.7° weapon cone.
        let weapon = 0.7f32.to_radians();
        let cone = npc_cone(weapon, 0.075, 15.0);
        assert!(
            (cone.to_degrees() - 1.825).abs() < 1e-4,
            "{}",
            cone.to_degrees()
        );
        // r at the cone's edge, straight up or to the side.
        let (h, p) = deviate(0.0, 0.0, cone, 1.0, 0.25);
        assert!(h.abs() < 1e-6 && (p - cone).abs() < 1e-6);
        let (h, p) = deviate(1.0, 0.0, cone, 1.0, 0.0);
        assert!((h - 1.0 - cone).abs() < 1e-6 && p.abs() < 1e-6);
        let (h, p) = deviate(1.0, 0.2, cone, 0.0, 0.7);
        assert_eq!((h, p), (1.0, 0.2));
    }

    #[test]
    fn heading_and_direction_agree() {
        let (h, p) = heading_pitch([0.0; 3], [100.0, 0.0, 100.0]);
        assert!((h - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        assert!((p - std::f32::consts::FRAC_PI_4).abs() < 1e-5);
        let d = direction(h, p);
        let s = std::f32::consts::FRAC_1_SQRT_2;
        assert!((d[0] - s).abs() < 1e-5 && d[1].abs() < 1e-5 && (d[2] - s).abs() < 1e-5);
    }

    #[test]
    fn the_only_detected_target_is_taken() {
        let mut a = candidate(1);
        a.detection = 0;
        let b = candidate(2);
        assert_eq!(best_target(&[a, b]), Some(FormId(2)));
        assert_eq!(best_target(&[b]), Some(FormId(2)));
        assert_eq!(best_target(&[]), None);
    }

    #[test]
    fn in_view_and_near_beat_far() {
        let mut far = candidate(1);
        far.distance_sq = 3000.0 * 3000.0;
        far.in_sight = true;
        let mut near = candidate(2);
        near.distance_sq = 1024.0 * 1024.0;
        // Near: 750; far, in sight: 100.
        assert_eq!(best_target(&[far, near]), Some(FormId(2)));
        // In view outweighs the distance.
        far.in_view = true;
        assert_eq!(best_target(&[far, near]), Some(FormId(1)));
        // A downed target loses 500.
        let mut down = near;
        down.reference = FormId(3);
        down.down = true;
        down.distance_sq = 0.0;
        let mut up = near;
        up.reference = FormId(4);
        up.distance_sq = 1024.0 * 1024.0;
        // Down at 0: 1000 − 500 = 500; up at 1024: 750.
        assert_eq!(best_target(&[down, up]), Some(FormId(4)));
    }

    #[test]
    fn the_current_target_seen_lately_keeps_its_place() {
        let mut current = candidate(1);
        current.current = true;
        current.seen_recently = true;
        current.distance_sq = 2048.0 * 2048.0;
        let mut other = candidate(2);
        other.in_view = true;
        // Current: 100 + 1000 = 1100; other in view at 0: 2000.
        assert_eq!(best_target(&[current, other]), Some(FormId(2)));
        other.distance_sq = 2048.0 * 2048.0;
        // Other: 1000; current 1100.
        assert_eq!(best_target(&[current, other]), Some(FormId(1)));
    }

    #[test]
    fn with_none_detected_the_freshest_is_taken() {
        let mut a = candidate(1);
        a.detection = -20;
        a.last_detected = 5.0;
        let mut b = candidate(2);
        b.detection = 0;
        b.last_detected = 9.0;
        assert_eq!(best_target(&[a, b]), Some(FormId(2)));
    }
}
