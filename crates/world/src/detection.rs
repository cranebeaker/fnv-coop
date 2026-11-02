//! Who notices whom: the game's detection value, read from its code
//! (`008a0d10` gathers the inputs, `00642ed0` adds them up; see
//! `%USERPROFILE%\nv-re\findings\combat_ai.md` §1).
//!
//! value = `fSneakBaseValue` (−35) + sound + light + skill, each scaled by
//! how near the target is, f = max((maxD − d) / maxD, 0) ^
//! `fSneakDistanceAttenuationExponent` (2), maxD = `fSneakMaxDistance`
//! (2500) × `fSneakExteriorDistanceMult` (2) outdoors:
//!
//! - sound = f (× `fSneakSoundLosMult` 0.1 out of sight) ×
//!   `fSneakSoundsMult` (1.6) × (moving × (`fSneakBootWeightBase` 12 +
//!   body armour weight × `fSneakBootWeightMult` 0.5) × (running ?
//!   `fSneakRunningMult` 1.5 : 1) + trunc(`fSneakActionMult` 2) × the
//!   target's last shot's noise);
//! - light, only in sight (and within the 190° view cone) = f × (1 + 0.01
//!   moving + 0.2 running) × (light level 0–100 + 0) × `fSneakLightMult`
//!   (1.4) × the detector's size factor;
//! - skill = (perception × mods × f − the target's sneak score × sneaking)
//!   × `fSneakSkillMult` (0.5), perception = 10 + 8 × Perception, mods 1 +
//!   0.2 in combat − 0.4 fighting someone else, sneak score = Sneak + (the
//!   target's level − the detector's) × 5 + max(50 − 10 × the target's
//!   level, 0) − 20 / 10 for heavy / medium body armour.
//!
//! Above 0: seen (`GetDetected`); above −20 (`fSneakNoticedMin`): noticed,
//! enough to start a fight; beyond maxD: −50. A value between 0 and 1 is
//! 1, otherwise it's cut to a whole number.

use crate::scripting::game_setting;
use esm::LoadOrder;

/// What a detection test knows about the two (the target's side and how
/// they stand to each other).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inputs {
    /// Straight-line distance.
    pub distance: f32,
    pub outdoors: bool,
    /// A clear line between them, and the target within the detector's
    /// view cone (`fDetectionViewCone` 190°).
    pub line_of_sight: bool,
    pub in_cone: bool,
    /// The target's movement.
    pub moving: bool,
    pub running: bool,
    pub sneaking: bool,
    /// The target's light level, 0–100.
    pub light: f32,
    /// The noise of the target's last attack (`world::noise`: 100 loud,
    /// 50 normal, 0 silent), 0 for none.
    pub shot_noise: f32,
    /// The weight of the target's body armour, and its stealth penalty
    /// (`iHeavyArmorStealthPenalty` 20, `iMediumArmorStealthPenalty` 10).
    pub armour_weight: f32,
    pub armour_penalty: f32,
    /// The detector's Perception and whether it's fighting (the target, or
    /// someone else).
    pub perception: f32,
    pub in_combat: bool,
    pub fighting_other: bool,
    /// The target's Sneak skill and both levels.
    pub sneak: f32,
    pub target_level: f32,
    pub detector_level: f32,
}

/// The value at which someone counts as seen, and as noticed.
pub const SEEN: i32 = 0;
pub const NOTICED: i32 = -20;

/// The detection value (see the module notes), with the load order's
/// settings (the exe's defaults where it has none).
pub fn value(order: &LoadOrder, i: &Inputs) -> i32 {
    value_with(i, |name, default| {
        game_setting(order, name).unwrap_or(default)
    })
}

/// The detection value with settings from `s` (a name and the exe's
/// default).
pub fn value_with(i: &Inputs, s: impl Fn(&str, f32) -> f32) -> i32 {
    let max = s("fSneakMaxDistance", 1500.0)
        * if i.outdoors {
            s("fSneakExteriorDistanceMult", 2.0)
        } else {
            1.0
        };
    if i.distance >= max {
        return s("iAICombatMinDetection", -50.0) as i32;
    }
    let f = ((max - i.distance) / max)
        .max(0.0)
        .powf(s("fSneakDistanceAttenuationExponent", 2.0));
    let moving = f32::from(u8::from(i.moving));
    let running = i.running;
    let sound =
        (f * if i.line_of_sight {
            1.0
        } else {
            s("fSneakSoundLosMult", 0.25)
        } * s("fSneakSoundsMult", 1.6)
            * (moving
                * (s("fSneakBootWeightBase", 14.0)
                    + i.armour_weight * s("fSneakBootWeightMult", 1.0))
                * if running {
                    s("fSneakRunningMult", 2.0)
                } else {
                    1.0
                }
                + s("fSneakActionMult", 1.0).trunc() * i.shot_noise))
            .max(0.0);
    let seen = i.line_of_sight && i.in_cone;
    let light = if seen {
        (f * (1.0
            + s("fSneakLightMoveMult", 0.01) * moving
            + s("fSneakLightRunMult", 0.2) * f32::from(u8::from(running)))
            * (i.light + s("fDetectionSneakLightMod", 0.0))
            * s("fSneakLightMult", 1.0))
        .max(0.0)
    } else {
        0.0
    };
    let perception = s("fSneakPerceptionSkillMin", 10.0)
        + i.perception / 10.0
            * (s("fSneakPerceptionSkillMax", 90.0) - s("fSneakPerceptionSkillMin", 10.0));
    let mut mods = 1.0;
    if i.in_combat {
        mods += s("fSneakAlertMod", 0.2);
    }
    if i.fighting_other {
        mods -= s("fSneakCombatMod", 0.4);
    }
    let sneak_score = i.sneak
        + (i.target_level - i.detector_level) * s("iSneakLevelBonus", 5.0)
        + (s("iSneakStartBonus", 50.0) - i.target_level * s("iSneakStartBonusLevelPenatly", 10.0))
            .max(0.0)
        - i.armour_penalty;
    let skill = (perception * mods * f - sneak_score * f32::from(u8::from(i.sneaking)))
        * s("fSneakSkillMult", 1.0);
    let v = s("fSneakBaseValue", -25.0) + sound + light + skill;
    if v > 0.0 && v < 1.0 {
        1
    } else {
        v.trunc() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `FalloutNV.esm`'s values for the settings it sets.
    fn data(name: &str, default: f32) -> f32 {
        match name {
            "fSneakMaxDistance" => 2500.0,
            "fSneakSoundLosMult" => 0.1,
            "fSneakBootWeightBase" => 12.0,
            "fSneakBootWeightMult" => 0.5,
            "fSneakRunningMult" => 1.5,
            "fSneakActionMult" => 2.0,
            "fSneakLightMult" => 1.4,
            "fSneakSkillMult" => 0.5,
            "fSneakBaseValue" => -35.0,
            _ => default,
        }
    }

    #[test]
    fn a_guard_sees_a_walking_player_as_the_game_works_it_out() {
        // The findings' worked example: Perception 5, the player walking,
        // not sneaking, indoors, lit at 50, no body armour.
        let at = |distance: f32| Inputs {
            distance,
            outdoors: false,
            line_of_sight: true,
            in_cone: true,
            moving: true,
            running: false,
            sneaking: false,
            light: 50.0,
            shot_noise: 0.0,
            armour_weight: 0.0,
            armour_penalty: 0.0,
            perception: 5.0,
            in_combat: false,
            fighting_other: false,
            sneak: 15.0,
            target_level: 1.0,
            detector_level: 1.0,
        };
        assert_eq!(value_with(&at(500.0), data), 38);
        let near_edge = value_with(&at(1500.0), data);
        assert!((NOTICED..SEEN).contains(&near_edge), "{near_edge}");
        assert!(value_with(&at(2000.0), data) <= NOTICED);
        // Beyond the most distance: −50.
        assert_eq!(value_with(&at(2600.0), data), -50);
        // Sneaking takes the sneak score (15 + 50 − 10 = 55) off the
        // perception term: (50 × 0.64 − 55) × 0.5 = −11.5, so −35 + 12.29
        // + 45.25 − 11.5 = 11.
        let sneaking = Inputs {
            sneaking: true,
            ..at(500.0)
        };
        assert_eq!(value_with(&sneaking, data), 11);
    }
}
