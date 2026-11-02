//! Whether the combat manager wants battle music (`00992d90`, on the
//! combat manager `[011f1958]`), read from the game's code:
//!
//! Every `fCombatMusicUpdateTime` (1 s), and not while a save loads, it
//! looks at the combat groups in turn, stopping at the first that wants
//! music (P is the player's strength):
//!
//! - a group whose targets include the player and that saw the player
//!   less than `fCombatDetectionLostTime` (15 s) ago: wants music if the
//!   music is on already, else if E (its members detecting the player now,
//!   their strengths summed) > 0 and P / E < `fCombatMusicPlayerTargetedThreatRatio`
//!   (100 in `FalloutNV.esm`; the exe's 2);
//! - a group fighting someone else: with the music on, if it's within
//!   `fCombatMusicNearCombatOuterRadius` (2000; exe 3500) of the player;
//!   off, within `fCombatMusicNearCombatInnerRadius` (500; exe 2500) its
//!   strength adds up, and once some such group has a member detecting the
//!   player, music when P × `fCombatMusicPlayerNearStrengthMult` (1) is
//!   below the sum.
//!
//! Wanted and off: on at once (the stop timer cleared). Not wanted and on:
//! the stop timer starts; once it has run `fCombatMusicStopTime` (3; exe
//! 5), off.
//!
//! **Guesses** (this engine has no combat groups, detection or strength
//! yet): each person fighting is a group of one, fighting the player when
//! their target is the player; someone "detects" the player within
//! `fSneakMaxDistance` (× `fSneakExteriorDistanceMult` outdoors), in 3D;
//! strength is their health now (the game's is A × health / (1 −
//! min(armour / 30, 0.99)), A a figure not traced); distances are 3D
//! (`00989880` isn't traced).

use std::collections::HashMap;

use esm::{FormId, LoadOrder};

use crate::scripting::{game_setting, GameState};

/// The combat music settings (`GMST`s, else the exe's defaults).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatMusicSettings {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub targeted_threat_ratio: f32,
    pub stop_time: f32,
    pub update_time: f32,
    pub detection_lost_time: f32,
    pub near_strength_mult: f32,
}

impl Default for CombatMusicSettings {
    /// The exe's defaults.
    fn default() -> Self {
        CombatMusicSettings {
            inner_radius: 2500.0,
            outer_radius: 3500.0,
            targeted_threat_ratio: 2.0,
            stop_time: 5.0,
            update_time: 1.0,
            detection_lost_time: 15.0,
            near_strength_mult: 1.0,
        }
    }
}

impl CombatMusicSettings {
    pub fn load(order: &LoadOrder) -> CombatMusicSettings {
        let d = CombatMusicSettings::default();
        let g = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        CombatMusicSettings {
            inner_radius: g("fCombatMusicNearCombatInnerRadius", d.inner_radius),
            outer_radius: g("fCombatMusicNearCombatOuterRadius", d.outer_radius),
            targeted_threat_ratio: g(
                "fCombatMusicPlayerTargetedThreatRatio",
                d.targeted_threat_ratio,
            ),
            stop_time: g("fCombatMusicStopTime", d.stop_time),
            update_time: g("fCombatMusicUpdateTime", d.update_time),
            detection_lost_time: g("fCombatDetectionLostTime", d.detection_lost_time),
            near_strength_mult: g("fCombatMusicPlayerNearStrengthMult", d.near_strength_mult),
        }
    }
}

/// One combat group, as the combat music looks at it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatGroup {
    /// The player is one of its targets.
    pub targets_player: bool,
    /// Seconds since it last saw the player.
    pub since_seen: f32,
    /// Its members detecting the player now: their strengths summed.
    pub detecting_strength: f32,
    /// It's fighting (someone).
    pub fighting: bool,
    /// How far it is from the player.
    pub distance: f32,
    /// Its strength.
    pub strength: f32,
    /// One of its members detects the player now.
    pub detects: bool,
}

/// The combat music's state: when it started (the game's start time,
/// −FLT_MAX for off), the stop timer, and when it last looked.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatMusic {
    started: Option<f32>,
    stop_timer: Option<f32>,
    looked: Option<f32>,
    /// When each person fighting last saw the player (the groups' "last
    /// seen" times, a person to a group here).
    seen: HashMap<FormId, f32>,
}

impl CombatMusic {
    /// Whether battle music is wanted now.
    pub fn on(&self) -> bool {
        self.started.is_some()
    }

    /// One look (`00992d90`); `now` is game time in seconds. Answers
    /// whether battle music is on.
    pub fn update(
        &mut self,
        now: f32,
        settings: &CombatMusicSettings,
        player_strength: f32,
        groups: &[CombatGroup],
    ) -> bool {
        if self.looked.is_some_and(|t| now - t < settings.update_time) {
            return self.on();
        }
        self.looked = Some(now);
        let on = self.on();
        let mut want = false;
        let mut near = 0.0;
        let mut seen = false;
        for g in groups {
            if g.targets_player {
                if g.since_seen < settings.detection_lost_time {
                    if on {
                        want = true;
                    } else {
                        let e = g.detecting_strength;
                        if e > 0.0 && player_strength / e < settings.targeted_threat_ratio {
                            want = true;
                        }
                    }
                }
            } else if g.fighting {
                if on {
                    if g.distance <= settings.outer_radius {
                        want = true;
                    }
                } else if g.distance <= settings.inner_radius {
                    seen |= g.detects;
                    near += g.strength;
                    if seen && player_strength * settings.near_strength_mult < near {
                        want = true;
                    }
                }
            }
            if want {
                break;
            }
        }
        if want && !on {
            self.started = Some(now);
            self.stop_timer = None;
        } else if !want && on {
            match self.stop_timer {
                None => self.stop_timer = Some(now),
                Some(t) if now - t >= settings.stop_time => {
                    self.started = None;
                    self.stop_timer = None;
                }
                Some(_) => {}
            }
        }
        self.on()
    }

    /// The combat groups as this engine has them (see the module notes):
    /// everyone alive whose fight (`GameState::combat`) has a target and
    /// whose position is known (`people`). Remembers when each last saw
    /// the player.
    pub fn groups(
        &mut self,
        order: &LoadOrder,
        state: &GameState,
        player: [f32; 3],
        outdoors: bool,
        people: &[(FormId, [f32; 3])],
        now: f32,
    ) -> Vec<CombatGroup> {
        let reach = game_setting(order, "fSneakMaxDistance").unwrap_or(1500.0)
            * if outdoors {
                game_setting(order, "fSneakExteriorDistanceMult").unwrap_or(2.0)
            } else {
                1.0
            };
        let player_ref = crate::dialogue::PLAYER_REF;
        let mut out = Vec::new();
        for &(who, at) in people {
            let Some(&target) = state.combat.get(&who) else {
                continue;
            };
            if who == player_ref || state.dead.contains(&who) {
                continue;
            }
            let distance = (0..3)
                .map(|i| (at[i] - player[i]).powi(2))
                .sum::<f32>()
                .sqrt();
            let detects = distance <= reach;
            if detects {
                self.seen.insert(who, now);
            }
            let strength = strength(order, state, who);
            out.push(CombatGroup {
                targets_player: target == player_ref,
                since_seen: self.seen.get(&who).map_or(f32::MAX, |t| now - t),
                detecting_strength: if detects { strength } else { 0.0 },
                fighting: true,
                distance,
                strength,
                detects,
            });
        }
        self.seen.retain(|who, _| state.combat.contains_key(who));
        out
    }
}

/// Someone's strength as the combat music weighs it: their health now
/// (a guess; see the module notes).
fn strength(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    crate::combat::health(order, state, who).map_or(1.0, |h| h.max(0.0) as f32)
}

/// The player's strength (`008acbe0` on the player; the same guess).
pub fn player_strength(order: &LoadOrder, state: &GameState) -> f32 {
    strength(order, state, crate::dialogue::PLAYER_REF)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `FalloutNV.esm`'s values.
    fn settings() -> CombatMusicSettings {
        CombatMusicSettings {
            inner_radius: 500.0,
            outer_radius: 2000.0,
            targeted_threat_ratio: 100.0,
            stop_time: 3.0,
            ..CombatMusicSettings::default()
        }
    }

    fn enemy(since_seen: f32, detecting: f32) -> CombatGroup {
        CombatGroup {
            targets_player: true,
            since_seen,
            detecting_strength: detecting,
            fighting: true,
            distance: 800.0,
            strength: 30.0,
            detects: detecting > 0.0,
        }
    }

    #[test]
    fn battle_music_starts_with_an_enemy_and_stops_after_the_stop_time() {
        let s = settings();
        let mut m = CombatMusic::default();
        // Nobody: off.
        assert!(!m.update(0.0, &s, 200.0, &[]));
        // An enemy who sees the player: on at the next look (1 s later),
        // not before.
        assert!(!m.update(0.5, &s, 200.0, &[enemy(0.0, 30.0)]));
        assert!(m.update(1.0, &s, 200.0, &[enemy(0.0, 30.0)]));
        // A player 100 times stronger than the enemy gets none.
        let mut calm = CombatMusic::default();
        assert!(!calm.update(0.0, &s, 3000.0, &[enemy(0.0, 30.0)]));
        // Out of sight for under 15 s still counts once on.
        assert!(m.update(2.0, &s, 200.0, &[enemy(10.0, 0.0)]));
        // The fight over: the stop timer starts, and 3 s on it's off.
        assert!(m.update(3.0, &s, 200.0, &[]));
        assert!(m.update(5.0, &s, 200.0, &[]));
        assert!(!m.update(6.0, &s, 200.0, &[]));
    }

    #[test]
    fn a_fight_nearby_brings_music_when_it_outweighs_the_player() {
        let s = settings();
        let near = |distance: f32, strength: f32| CombatGroup {
            targets_player: false,
            since_seen: f32::MAX,
            detecting_strength: 0.0,
            fighting: true,
            distance,
            strength,
            detects: true,
        };
        let mut m = CombatMusic::default();
        // Within 500 but weaker than the player: nothing.
        assert!(!m.update(0.0, &s, 100.0, &[near(400.0, 60.0)]));
        // Two such together outweigh them.
        assert!(m.update(1.0, &s, 100.0, &[near(400.0, 60.0), near(450.0, 60.0)]));
        // Once on, anything fighting within 2000 keeps it.
        assert!(m.update(2.0, &s, 100.0, &[near(1900.0, 1.0)]));
    }
}
