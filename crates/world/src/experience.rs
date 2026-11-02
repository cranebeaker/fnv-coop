//! Experience and levels: the player's actor value 24 (`XP`), and what a
//! level brings. All read from the game's code (`FalloutNV.exe`; addresses
//! and details in `%USERPROFILE%\nv-re\findings\levelling.md`). Settings
//! missing from the data take the exe's built-in defaults, noted below.
//!
//! - XP to reach level L (`00648b50`): 0 below 2, else a step of `iXPBase`
//!   (200, exe default) growing by `iXPBumpBase` (150) each level: 25 (L −
//!   1)(3L + 2) with these values (200, 550, 1050, 1700 …).
//! - Every award (`008d5100`): nothing at `iMaxCharacterLevel` (30); the
//!   amount × the player's perks' "Adjust Experience Points" (entry point
//!   9: Swift Learner × 1.1), rounded up, cut so the total stops at the
//!   last level's. The "+N XP" notice; when the total reaches the next
//!   level's, a level-up waits (`008d55e0`) until the player is out of
//!   combat (and not in character generation or a menu), then comes one
//!   level at a time (`008d5210`, `008d5270`).
//! - A level-up gives skill points (`00648c10`): ⌊10 + 0.5 × I⌋ with I the
//!   permanent Intelligence (at most 10), + 1 when the new level is even
//!   and I odd, then the perks' "Adjust Gained Skill Points" (entry point
//!   10: Educated + 2). `iLevelUpSkillPoints…` aren't used by the game. A
//!   perk every `iLevelsPerPerk` (2, exe default) levels (`00784c80`).
//! - Kills (`0089d900`): the player and teammates' damage to the victim
//!   must be more than `iXPDeathRewardHealthThreshold` (40) percent of its
//!   full health; the XP is by the victim's own level: the first of five
//!   level limits it doesn't pass (`iXPLevelKillCreature…` 1, 3, 6, 9, 12
//!   → `iXPRewardKillOpponent…` 1, 5, 10, 25, 50 for creatures;
//!   `iXPLevelKillNPC…` 0, 1, 7, 10, 13 → `iXPRewardKillNPC…` 0, 10, 20,
//!   30, 50, exe defaults, for people); × the difficulty's `fDiffMultXP…`
//!   (1 at every difficulty in this data), rounded down. A teammate's
//!   death counts only when the player killed them.
//! - Map markers 10 (`iXPRewardDiscoverMapMarker`), picking a lock and
//!   hacking a terminal by difficulty (`iXPRewardPickLock…`,
//!   `iXPRewardHackComputer…`: 20 … 60), disarming a mine
//!   (`iMineDisarmExperience` 5), and quests' `RewardXP` (always the
//!   player). Only kills take the difficulty multiplier.
//!
//! Not traced: four interface checks that also hold the level-up back
//! (probably "no menu open"; the viewer waits for its menus to close).

use esm::{FormId, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::perks;
use crate::scripting::{game_setting, Event, Facts, GameState};

/// The experience points actor value.
pub const XP: u16 = 24;
/// Intelligence.
const INTELLIGENCE: u16 = 9;

/// The setting names' difficulty words, by difficulty (0 very easy … 4
/// very hard): lock brackets, terminal difficulties, kill tables.
pub const DIFFICULTIES: [&str; 5] = ["VeryEasy", "Easy", "Average", "Hard", "VeryHard"];

/// A setting, or the exe's built-in default when the data has none.
fn setting(order: &LoadOrder, name: &str, default: f32) -> f32 {
    game_setting(order, name).unwrap_or(default)
}

/// The player's experience points.
pub fn xp(state: &GameState) -> f64 {
    state
        .actor_values
        .get(&(PLAYER_REF, XP))
        .copied()
        .unwrap_or(0.0)
}

/// The highest level (`iMaxCharacterLevel`, 30).
pub fn max_level(order: &LoadOrder) -> u16 {
    setting(order, "iMaxCharacterLevel", 30.0).max(1.0) as u16
}

/// The experience points it takes to reach a level (`00648b50`).
pub fn xp_for_level(order: &LoadOrder, level: u16) -> f64 {
    if level < 2 {
        return 0.0;
    }
    let base = f64::from(setting(order, "iXPBase", 200.0));
    let bump = f64::from(setting(order, "iXPBumpBase", 100.0));
    let mut step = base;
    let mut total = base;
    for _ in 2..level {
        step += bump;
        total += step;
    }
    total
}

/// Gives the player experience (`008d5100`): the perks' multiplier,
/// rounded up, stopping at the last level; a notice, and a level-up
/// waiting when the next level is reached. What was given.
pub fn reward(order: &LoadOrder, state: &mut GameState, amount: f64) -> f64 {
    let max = max_level(order);
    if state.player_level >= max || amount == 0.0 {
        return 0.0;
    }
    let adjusted = perks::apply(
        order,
        state,
        perks::entry::ADJUST_EXPERIENCE_POINTS,
        amount as f32,
    );
    let mut value = f64::from(adjusted).ceil();
    let now = xp(state);
    let cap = xp_for_level(order, max);
    if now + value > cap {
        value = cap - now;
    }
    if value == 0.0 {
        return 0.0;
    }
    let total = now + value;
    state.actor_values.insert((PLAYER_REF, XP), total);
    state.emit(crate::coop::Domain::Experience, PLAYER_REF);
    if value > 0.0 {
        // The HUD's meter: `sStatsXP` and the gain (`0077c4e0`).
        let label =
            crate::scripting::game_setting_text(order, "sStatsXP").unwrap_or_else(|| "XP".into());
        state.events.push(Event::Message {
            title: None,
            text: format!("{label} +{value}"),
            buttons: Vec::new(),
            icon: None,
        });
        if total >= xp_for_level(order, state.player_level + 1) {
            state.level_up_pending = true;
        }
    }
    value
}

/// Gives the experience a setting names (nothing if it isn't there).
pub fn reward_setting(order: &LoadOrder, state: &mut GameState, setting: &str) {
    if let Some(amount) = game_setting(order, setting) {
        reward(order, state, f64::from(amount));
    }
}

/// The setting for something of a difficulty (0–4): `iXPRewardPickLock`
/// + "Average" for an average lock.
pub fn by_difficulty(base: &str, difficulty: u8) -> String {
    format!("{base}{}", DIFFICULTIES[usize::from(difficulty.min(4))])
}

/// What a level-up brings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelUp {
    pub level: u16,
    pub skill_points: u32,
    /// Whether a perk is picked at this level.
    pub perk: bool,
}

/// Skill points for reaching `new_level` with a permanent Intelligence
/// (`00648c10`), before perks.
pub fn base_skill_points(intelligence: f64, new_level: u16) -> f64 {
    let i = intelligence.min(10.0);
    let mut points = (10.0 + 0.5 * i).floor();
    if new_level % 2 == 0 && (i as i64) % 2 != 0 {
        points += 1.0;
    }
    points
}

/// Whether a level-up is waiting and may happen now: not in combat (the
/// caller also waits for menus to close).
pub fn level_up_ready(state: &GameState) -> bool {
    state.level_up_pending
        && !state.combat.contains_key(&PLAYER_REF)
        && !state.combat.values().any(|t| *t == PLAYER_REF)
}

/// The player goes up a level (`008d5210`): the new level and what it
/// brings. The waiting level-up stays set while the experience already
/// reaches the next one (`008d5270`).
pub fn level_up(order: &LoadOrder, state: &mut GameState) -> LevelUp {
    let level = (state.player_level + 1).min(max_level(order));
    state.player_level = level;
    state.emit(crate::coop::Domain::Level, PLAYER_REF);
    state.level_up_pending =
        level < max_level(order) && xp(state) >= xp_for_level(order, level + 1);
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let intelligence = facts
        .permanent_actor_value(PLAYER_REF, INTELLIGENCE)
        .unwrap_or(5.0);
    let points = perks::apply(
        order,
        state,
        perks::entry::ADJUST_GAINED_SKILL_POINTS,
        base_skill_points(intelligence, level) as f32,
    );
    let every = setting(order, "iLevelsPerPerk", 2.0).max(1.0) as u16;
    state.events.push(Event::Message {
        title: None,
        text: crate::scripting::game_setting_text(order, "sLevelUp")
            .unwrap_or_else(|| "LEVEL UP".into()),
        buttons: Vec::new(),
        icon: None,
    });
    LevelUp {
        level,
        skill_points: points.max(0.0) as u32,
        perk: level % every == 0,
    }
}

/// Experience for killing someone of a level (`006705b0`): the first of
/// the five level limits at or above it, else the last; creatures and
/// people have their own tables.
pub fn kill_xp(order: &LoadOrder, creature: bool, level: u16) -> f64 {
    let (limits, rewards, defaults): (&str, &str, [[f32; 5]; 2]) = if creature {
        (
            "iXPLevelKillCreature",
            "iXPRewardKillOpponent",
            [[1.0, 3.0, 6.0, 9.0, 12.0], [1.0, 5.0, 10.0, 25.0, 50.0]],
        )
    } else {
        (
            "iXPLevelKillNPC",
            "iXPRewardKillNPC",
            [[0.0, 1.0, 7.0, 10.0, 13.0], [0.0, 10.0, 20.0, 30.0, 50.0]],
        )
    };
    let level = f32::from(level);
    let mut pick = 4;
    for (i, word) in DIFFICULTIES.iter().enumerate() {
        if setting(order, &format!("{limits}{word}"), defaults[0][i]) >= level {
            pick = i;
            break;
        }
    }
    f64::from(setting(
        order,
        &format!("{rewards}{}", DIFFICULTIES[pick]),
        defaults[1][pick],
    ))
}

/// Damage counted toward the player's kill share: the player's and their
/// teammates' (`009134c0`).
pub fn count_damage(state: &mut GameState, victim: FormId, by: FormId, amount: f64) {
    if by == PLAYER_REF || state.teammates.contains(&by) {
        *state.kill_share.entry(victim).or_insert(0.0) += amount;
    }
}

/// Someone died: the player's experience for it (`0089e531`), given.
pub fn on_kill(order: &LoadOrder, state: &mut GameState, victim: FormId, killer: FormId) -> f64 {
    if victim == PLAYER_REF || (state.teammates.contains(&victim) && killer != PLAYER_REF) {
        return 0.0;
    }
    let share = state.kill_share.remove(&victim).unwrap_or(0.0);
    if share <= 0.0 {
        return 0.0;
    }
    let full = crate::combat::max_health(order, state, victim).unwrap_or(0.0);
    let percent = if full > 0.0 {
        (share / full * 100.0).trunc()
    } else {
        100.0
    };
    let threshold = f64::from(setting(order, "iXPDeathRewardHealthThreshold", 75.0));
    if percent <= threshold {
        return 0.0;
    }
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let level = facts.level_of(victim).unwrap_or(1);
    let creature = crate::combat::is_creature(order, victim);
    // The difficulty: Normal (`fDiffMultXPN`, 1), as the viewer has no
    // other.
    let mut amount =
        (kill_xp(order, creature, level) * f64::from(setting(order, "fDiffMultXPN", 1.0))).floor();
    if state.coop.authority != crate::coop::Authority::Single {
        amount = crate::coop::facts::xp_scale(state, amount);
    }
    reward(order, state, amount)
}
