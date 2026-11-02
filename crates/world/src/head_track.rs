//! Whom an actor looks at: the head-track target. Traced from
//! `FalloutNV.exe` 1.4.0.525; every address and the reasoning behind it are
//! in `docs/HEAD_TRACK_TARGET.md`. Turning the head toward the target is
//! [`crate::look_ik`].
//!
//! Ported from a contributor's branch (`playcon/claude/head-track-target`,
//! commit 9defa47); the chooser (008a3ed0), filter (008a4810) and score
//! (008a46c0) constants were re-checked in Ghidra on 2026-10-06.
//!
//! A high process keeps six target slots (`+0x3f8`), the highest one set
//! winning: what the actor chose itself, then what an action (`SayTo`), a
//! script (`Look`), combat and dialogue asked for. When a slot other than
//! the actor's own is released, its target is kept as the actor's own for
//! `fAIHoldDefaultHeadTrackTimer` seconds. Otherwise the actor picks
//! someone itself every second or so: the best-scored actor nearby that it
//! notices, in front of it.

use esm::FormId;

/// A target slot (`009016a0` names them), lowest priority first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    /// The actor's own choice (`DEFAULT`).
    Default = 0,
    /// `ACTION`: `SayTo` and its "Say To" package.
    Action = 1,
    /// `SCRIPT`: `Look`.
    Script = 2,
    /// `COMBAT`.
    Combat = 3,
    /// `DIALOG`: conversations.
    Dialog = 4,
    /// The sixth slot, whose setters aren't traced.
    Fifth = 5,
}

const SLOTS: usize = 6;

/// The settings head tracking reads, with the game's defaults.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `fAIHoldDefaultHeadTrackTimer`: how long a released or newly chosen
    /// target is kept before the actor chooses again, seconds.
    pub hold: f32,
    /// `fAIMaxHeadTrackDistance`: the farthest a candidate can be.
    pub max_distance: f32,
    /// `fAIBestHeadTrackDistance`: the distance that scores 1.
    pub best_distance: f32,
    /// `fAIMaxHeadTrackDistanceFromPC`: actors farther than this from the
    /// player don't update their target.
    pub max_distance_from_player: f32,
    /// `[HeadTracking] fUpdateDelaySecondsMin/Max`: how often an actor
    /// chooses.
    pub update_delay: (f32, f32),
    /// `[HeadTracking] fUpdateDelayNewTargetSecondsMin/Max`: the
    /// new-target timer that favours the current target.
    pub new_target_delay: (f32, f32),
    /// `[HeadTracking] iUpdateActorsPerFrame`.
    pub actors_per_frame: u32,
    /// `[HeadTracking] bDisableHeadTracking`.
    pub disabled: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            hold: 10.0,
            max_distance: 400.0,
            best_distance: 500.0,
            max_distance_from_player: 2000.0,
            update_delay: (1.0, 1.5),
            new_target_delay: (6.0, 10.0),
            actors_per_frame: 1,
            disabled: false,
        }
    }
}

impl Settings {
    /// The settings from the game settings (`game`, by name: `GMST`
    /// overrides) and the INI (`ini`, section and key), else the defaults.
    pub fn read(
        game: impl Fn(&str) -> Option<f32>,
        ini: impl Fn(&str, &str) -> Option<f32>,
    ) -> Settings {
        let d = Settings::default();
        let g = |name: &str, default: f32| game(name).unwrap_or(default);
        let i = |key: &str, default: f32| ini("HeadTracking", key).unwrap_or(default);
        Settings {
            hold: g("fAIHoldDefaultHeadTrackTimer", d.hold),
            max_distance: g("fAIMaxHeadTrackDistance", d.max_distance),
            best_distance: g("fAIBestHeadTrackDistance", d.best_distance),
            max_distance_from_player: g(
                "fAIMaxHeadTrackDistanceFromPC",
                d.max_distance_from_player,
            ),
            update_delay: (
                i("fUpdateDelaySecondsMin", d.update_delay.0),
                i("fUpdateDelaySecondsMax", d.update_delay.1),
            ),
            new_target_delay: (
                i("fUpdateDelayNewTargetSecondsMin", d.new_target_delay.0),
                i("fUpdateDelayNewTargetSecondsMax", d.new_target_delay.1),
            ),
            actors_per_frame: i("iUpdateActorsPerFrame", d.actors_per_frame as f32).max(0.0) as u32,
            disabled: i("bDisableHeadTracking", 0.0) != 0.0,
        }
    }
}

/// One actor's head-track state: the process's slots and hold timer
/// (`+0x3f8`, `+0x410`, `+0x418`) and the actor's two timers (`+0x74`,
/// `+0x158`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeadTrack {
    targets: [Option<FormId>; SLOTS],
    flags: [bool; SLOTS],
    /// `+0x418`: counts down; the actor may choose only once it is below 0.
    hold: f32,
    /// Actor `+0x74`: counts down to the next choice.
    update_timer: f32,
    /// Actor `+0x158`: favours the current target while it lasts.
    new_target_timer: f32,
}

impl HeadTrack {
    /// Puts `who` in `slot` (`+0x624`–`+0x638`). Slot 0's flag is always
    /// set; another's only when there is someone.
    pub fn set(&mut self, slot: Slot, who: Option<FormId>) {
        let s = slot as usize;
        self.targets[s] = who;
        self.flags[s] = slot == Slot::Default || who.is_some();
    }

    /// Empties `slot` (`+0x640`–`+0x654`). With `demote` (not for
    /// [`Slot::Combat`] or [`Slot::Default`], which have no such form) its
    /// target becomes the actor's own, held for [`Settings::hold`]
    /// seconds; slot 0's flag is left as it was.
    pub fn clear(&mut self, slot: Slot, demote: bool, settings: &Settings) {
        let s = slot as usize;
        let old = self.targets[s].take();
        self.flags[s] = false;
        if demote && !matches!(slot, Slot::Combat | Slot::Default) {
            self.targets[Slot::Default as usize] = old;
            self.hold = settings.hold;
        }
    }

    /// Empties every slot (`+0x660`).
    pub fn clear_all(&mut self) {
        self.targets = [None; SLOTS];
        self.flags = [false; SLOTS];
    }

    /// Empties every slot holding `who` (`+0x664`).
    pub fn forget(&mut self, who: FormId) {
        for s in 0..SLOTS {
            if self.targets[s] == Some(who) {
                self.targets[s] = None;
                self.flags[s] = false;
            }
        }
    }

    /// Who is in `slot`, set or not (`+0x674`).
    pub fn in_slot(&self, slot: Slot) -> Option<FormId> {
        self.targets[slot as usize]
    }

    /// The highest slot set (`+0x67c`).
    pub fn current_slot(&self) -> Option<Slot> {
        const ALL: [Slot; SLOTS] = [
            Slot::Default,
            Slot::Action,
            Slot::Script,
            Slot::Combat,
            Slot::Dialog,
            Slot::Fifth,
        ];
        (0..SLOTS).rev().find(|&s| self.flags[s]).map(|s| ALL[s])
    }

    /// Whom the actor looks at (`+0x678`): the highest slot set's target,
    /// which can be no one.
    pub fn current(&self) -> Option<FormId> {
        self.current_slot().and_then(|s| self.targets[s as usize])
    }

    /// Whether someone may ask the actor to look at someone
    /// (`CanSetActionHeadTrackTarget` (Xbox PDB), `+0x66c`, `00901460`):
    /// no flag of slots 1–5 set.
    // Translated from 00901460 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn action_free(&self) -> bool {
        !self.flags[1..].iter().any(|&f| f)
    }

    /// Whether the actor may choose its own target (`+0x668`): nothing
    /// asked of it and the hold over.
    pub fn may_choose(&self) -> bool {
        !self.flags[1..].iter().any(|&f| f) && self.hold < 0.0
    }

    /// One update of the actor (`008a3100` steps 3 and 5–8).
    ///
    /// `chosen_this_frame` is the frame's count of actors that chose
    /// (`011df674`, zeroed each frame); `random` gives a number in `[0, 1)`
    /// for the random delays (`00476b70`); `choose` is the chooser
    /// ([`choose`]), run only when the actor gets to choose, given the
    /// current target and the new-target timer. Returns the actor's new
    /// target when it changed it.
    pub fn update(
        &mut self,
        dt: f32,
        settings: &Settings,
        chosen_this_frame: &mut u32,
        random: &mut impl FnMut() -> f32,
        choose: impl FnOnce(Option<FormId>, f32) -> Option<FormId>,
    ) -> Option<FormId> {
        self.update_timer -= dt;
        self.hold -= dt;
        if *chosen_this_frame > settings.actors_per_frame {
            return None;
        }
        self.new_target_timer -= dt;
        if self.update_timer > 0.0 {
            return None;
        }
        *chosen_this_frame += 1;
        self.update_timer = between(settings.update_delay, random());
        if !self.may_choose() {
            return None;
        }
        self.new_target_timer = self.new_target_timer.max(0.0);
        let current = self.current();
        let pick = choose(current, self.new_target_timer);
        if pick == current {
            return None;
        }
        self.set(Slot::Default, pick);
        self.hold = settings.hold;
        self.new_target_timer = between(settings.new_target_delay, random());
        pick
    }
}

fn between((min, max): (f32, f32), unit: f32) -> f32 {
    min + (max - min) * unit
}

/// What the chooser needs to know about one candidate, from the choosing
/// actor's point of view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candidate {
    pub reference: FormId,
    pub is_player: bool,
    /// The distance between the two (`005723b0`).
    pub distance: f32,
    /// How far the direction to the candidate is off the actor's heading,
    /// radians, either side.
    pub off_heading: f32,
    /// The actor detects it (`008a0d10`'s level is at least 1).
    pub detected: bool,
    /// The actor has a line of sight to it (`0088b880`).
    pub in_sight: bool,
    /// It is dying, dead or unconscious (life state 1, 2 or 6).
    pub down: bool,
}

/// Whether `c` can be looked at (`008a4810`, the parts the caller can
/// supply: it is another actor with 3D, the chooser's caller only offers
/// those). `current` is the actor's current target (also what the game
/// caches at `+0x68c`).
pub fn accepts(c: &Candidate, current: Option<FormId>, settings: &Settings) -> bool {
    let is_current = current == Some(c.reference);
    if c.distance > settings.max_distance {
        return false;
    }
    if c.distance < 100.0 && !c.is_player && !is_current {
        return false;
    }
    let limit = if is_current {
        std::f32::consts::PI
    } else {
        120f32.to_radians()
    };
    c.off_heading.abs() <= limit && c.detected
}

/// How much the actor wants to look at `c` (`008a46c0`): 1 at
/// `fAIBestHeadTrackDistance`, more nearer, 0 at 1000; halved without a
/// line of sight; × (`new_target_timer` + 1) for the current target;
/// halved for someone down.
pub fn score(
    c: &Candidate,
    current: Option<FormId>,
    new_target_timer: f32,
    settings: &Settings,
) -> f32 {
    let mut s = (1000.0 - c.distance) / (1000.0 - settings.best_distance);
    if !c.in_sight {
        s *= 0.5;
    }
    if current == Some(c.reference) {
        s *= new_target_timer + 1.0;
    }
    if c.down {
        s *= 0.5;
    }
    s
}

/// The chooser (`008a3ed0`): of the candidates `accepts` lets through, the
/// one with the highest score above 0, the first on a tie (the game lists
/// the high-process actors, then the player); no one if none.
pub fn choose(
    candidates: &[Candidate],
    current: Option<FormId>,
    new_target_timer: f32,
    settings: &Settings,
) -> Option<FormId> {
    let mut best: Option<(FormId, f32)> = None;
    for c in candidates.iter().filter(|c| accepts(c, current, settings)) {
        let s = score(c, current, new_target_timer, settings);
        if s > best.map_or(0.0, |b| b.1) {
            best = Some((c.reference, s));
        }
    }
    best.map(|b| b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: FormId = FormId(0x104c0c);
    const PLAYER: FormId = FormId(0x14);
    const SUNNY: FormId = FormId(0x104e85);

    fn person(reference: FormId, distance: f32) -> Candidate {
        Candidate {
            reference,
            is_player: reference == PLAYER,
            distance,
            off_heading: 0.0,
            detected: true,
            in_sight: true,
            down: false,
        }
    }

    #[test]
    fn the_highest_slot_set_wins_and_releasing_one_holds_its_target() {
        let s = Settings::default();
        let mut h = HeadTrack::default();
        assert_eq!(h.current(), None);
        h.set(Slot::Default, Some(SUNNY));
        h.set(Slot::Action, Some(PLAYER));
        assert_eq!(h.current(), Some(PLAYER));
        assert_eq!(h.current_slot(), Some(Slot::Action));
        assert!(!h.may_choose());
        // A conversation outranks the action.
        h.set(Slot::Dialog, Some(DOC));
        assert_eq!(h.current(), Some(DOC));
        h.clear(Slot::Dialog, false, &s);
        assert_eq!(h.current(), Some(PLAYER));
        // Released with demote: the player is kept as its own choice, and
        // it can't choose for 10 s.
        h.clear(Slot::Action, true, &s);
        assert_eq!(h.current(), Some(PLAYER));
        assert_eq!(h.current_slot(), Some(Slot::Default));
        assert!(!h.may_choose());
        h.hold -= 10.01;
        assert!(h.may_choose());
    }

    #[test]
    fn demoting_needs_the_own_slot_set_and_setting_no_one_unsets_a_slot() {
        let s = Settings::default();
        let mut h = HeadTrack::default();
        // Never chosen: slot 0 isn't set, so the demoted target isn't
        // looked at (slot 0's flag is left as it was).
        h.set(Slot::Action, Some(PLAYER));
        h.clear(Slot::Action, true, &s);
        assert_eq!(h.in_slot(Slot::Default), Some(PLAYER));
        assert_eq!(h.current(), None);
        // Slot 0 set to no one is still the highest set: no one.
        h.set(Slot::Default, None);
        assert_eq!(h.current_slot(), Some(Slot::Default));
        h.set(Slot::Script, Some(DOC));
        h.set(Slot::Script, None);
        assert_eq!(h.current_slot(), Some(Slot::Default));
        // Combat has no demote.
        h.set(Slot::Combat, Some(SUNNY));
        h.clear(Slot::Combat, true, &s);
        assert_eq!(h.in_slot(Slot::Default), None);
        h.set(Slot::Default, Some(DOC));
        h.set(Slot::Dialog, Some(DOC));
        h.forget(DOC);
        assert_eq!(h.current_slot(), None);
    }

    #[test]
    fn the_filter_takes_distance_angle_and_detection() {
        let s = Settings::default();
        assert!(accepts(&person(SUNNY, 400.0), None, &s));
        assert!(!accepts(&person(SUNNY, 400.5), None, &s));
        // Too near, unless the player or the current target.
        assert!(!accepts(&person(SUNNY, 99.0), None, &s));
        assert!(accepts(&person(SUNNY, 99.0), Some(SUNNY), &s));
        assert!(accepts(&person(PLAYER, 20.0), None, &s));
        // Behind: 120° either side, all round for the current target.
        let behind = Candidate {
            off_heading: 2.2,
            ..person(SUNNY, 200.0)
        };
        assert!(!accepts(&behind, None, &s));
        assert!(accepts(&behind, Some(SUNNY), &s));
        let unnoticed = Candidate {
            detected: false,
            ..person(SUNNY, 200.0)
        };
        assert!(!accepts(&unnoticed, None, &s));
    }

    #[test]
    fn the_score_favours_near_seen_standing_and_current() {
        let s = Settings::default();
        assert_eq!(score(&person(SUNNY, 500.0), None, 0.0, &s), 1.0);
        assert_eq!(score(&person(SUNNY, 0.0), None, 0.0, &s), 2.0);
        let hidden = Candidate {
            in_sight: false,
            ..person(SUNNY, 500.0)
        };
        assert_eq!(score(&hidden, None, 0.0, &s), 0.5);
        let down = Candidate {
            down: true,
            ..person(SUNNY, 500.0)
        };
        assert_eq!(score(&down, None, 0.0, &s), 0.5);
        // The current target, 4 s into the new-target timer: × 5.
        assert_eq!(score(&person(SUNNY, 500.0), Some(SUNNY), 4.0, &s), 5.0);
    }

    #[test]
    fn the_chooser_picks_the_best_and_sticks_with_the_current_one() {
        let s = Settings::default();
        let near = person(SUNNY, 150.0);
        let player = person(PLAYER, 300.0);
        assert_eq!(choose(&[near, player], None, 0.0, &s), Some(SUNNY));
        // The player is the current target and the timer still runs.
        assert_eq!(choose(&[near, player], Some(PLAYER), 2.0, &s), Some(PLAYER));
        assert_eq!(choose(&[person(DOC, 900.0)], None, 0.0, &s), None);
        assert_eq!(choose(&[], None, 0.0, &s), None);
    }

    #[test]
    fn updates_choose_on_the_timers_and_the_frame_budget() {
        let s = Settings::default();
        let mut h = HeadTrack::default();
        let mut half = || 0.5;
        let mut frame = 0;
        // The hold starts at 0: not below it yet.
        let pick = h.update(0.0, &s, &mut frame, &mut half, |_, _| Some(PLAYER));
        assert_eq!(pick, None);
        assert_eq!(frame, 1);
        // 1.25 s later (the delay at 0.5 between 1 and 1.5) it chooses.
        let mut frame = 0;
        let pick = h.update(1.25, &s, &mut frame, &mut half, |_, _| Some(PLAYER));
        assert_eq!(pick, Some(PLAYER));
        assert_eq!(h.current(), Some(PLAYER));
        assert_eq!(h.new_target_timer, 8.0);
        // Then it holds that choice for 10 s, choosing nothing new.
        for _ in 0..9 {
            let mut frame = 0;
            let p = h.update(1.0, &s, &mut frame, &mut half, |_, _| Some(SUNNY));
            assert_eq!(p, None);
        }
        let mut frame = 0;
        let p = h.update(1.25, &s, &mut frame, &mut half, |current, timer| {
            assert_eq!(current, Some(PLAYER));
            assert_eq!(timer, 0.0);
            Some(SUNNY)
        });
        assert_eq!(p, Some(SUNNY));
        // Over the frame's budget (more than one already): nothing.
        let mut busy = 2;
        h.update_timer = 0.0;
        h.hold = -1.0;
        let p = h.update(0.1, &s, &mut busy, &mut half, |_, _| Some(DOC));
        assert_eq!(p, None);
        assert_eq!(busy, 2);
        // Something asked of it: no choosing.
        h.set(Slot::Action, Some(DOC));
        let mut frame = 0;
        h.update_timer = 0.0;
        let p = h.update(0.1, &s, &mut frame, &mut half, |_, _| Some(PLAYER));
        assert_eq!(p, None);
        assert_eq!(h.current(), Some(DOC));
    }

    #[test]
    fn settings_read_game_settings_and_the_ini() {
        let s = Settings::read(
            |name| (name == "fAIMaxHeadTrackDistance").then_some(600.0),
            |section, key| {
                (section == "HeadTracking" && key == "iUpdateActorsPerFrame").then_some(3.0)
            },
        );
        assert_eq!(s.max_distance, 600.0);
        assert_eq!(s.actors_per_frame, 3);
        assert_eq!(s.hold, 10.0);
        assert!(!s.disabled);
        assert_eq!(Settings::read(|_| None, |_, _| None), Settings::default());
    }
}
