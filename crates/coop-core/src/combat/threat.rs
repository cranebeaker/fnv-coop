//! The threat (attention) director: which player an enemy attends to.
//!
//! Score per player = damage dealt to this enemy (weighted by the party
//! focus) + nearness + seen, plus a stickiness bonus for the current target,
//! minus a spread penalty for every other enemy already on that player. A
//! challenger must beat the current target by a margin and the target must
//! have been held for a minimum time (hysteresis), so enemies do not flicker
//! between players. Downed and dead players score nothing; when everyone is
//! down there is no target.

use std::collections::BTreeMap;

use crate::ids::{FormId, PlayerId};

/// Minimum time to hold a target, ms.
pub const MIN_HOLD_MS: u64 = 3_000;
/// A challenger needs this many times the current target's score.
pub const SWITCH_MARGIN: f32 = 1.25;
/// Distance at which nearness reaches zero.
pub const NEAR_RANGE: f32 = 3_000.0;
/// Stickiness bonus for the current target.
pub const STICKY: f32 = 0.35;
/// Penalty for each other enemy already on a player.
pub const SPREAD: f32 = 0.12;
/// Damage that counts as a full point of score before focus.
pub const DAMAGE_UNIT: f32 = 100.0;

/// What the director knows about one player for one enemy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThreatInput {
    /// The player.
    pub player: PlayerId,
    /// Distance from the enemy (game units).
    pub dist: f32,
    /// The enemy detects this player now (that player's own sneak and noise
    /// were already folded in by the caller).
    pub seen: bool,
    /// Downed.
    pub downed: bool,
    /// Dead.
    pub dead: bool,
    /// In the enemy's cell.
    pub same_cell: bool,
}

/// The director's answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Threat {
    /// The chosen player.
    pub target: PlayerId,
    /// The target changed.
    pub switched: bool,
}

#[derive(Clone, Debug, Default)]
struct Mem {
    current: Option<PlayerId>,
    since_ms: u64,
    damage: BTreeMap<PlayerId, f32>,
}

/// Director state (kept for actors this machine owns).
#[derive(Clone, Debug, Default)]
pub struct ThreatDirector {
    mem: BTreeMap<FormId, Mem>,
    /// Target switches made.
    pub switches: u32,
}

impl ThreatDirector {
    /// Empty director.
    pub fn new() -> Self {
        Self::default()
    }

    /// `by` damaged `enemy`.
    pub fn damaged(&mut self, enemy: FormId, by: PlayerId, amount: f32) {
        *self
            .mem
            .entry(enemy)
            .or_default()
            .damage
            .entry(by)
            .or_insert(0.0) += amount.max(0.0);
    }

    /// The enemy died or was removed.
    pub fn forget(&mut self, enemy: FormId) {
        self.mem.remove(&enemy);
    }

    /// Enemies currently attending to `p`.
    pub fn load_on(&self, p: PlayerId) -> usize {
        self.mem.values().filter(|m| m.current == Some(p)).count()
    }

    /// The current target of an enemy.
    pub fn current(&self, enemy: FormId) -> Option<PlayerId> {
        self.mem.get(&enemy).and_then(|m| m.current)
    }

    fn score(
        &self,
        enemy: FormId,
        i: &ThreatInput,
        focus: f32,
        players: usize,
        is_current: bool,
    ) -> f32 {
        if i.downed || i.dead || !i.same_cell {
            return 0.0;
        }
        let dmg = self
            .mem
            .get(&enemy)
            .and_then(|m| m.damage.get(&i.player))
            .copied()
            .unwrap_or(0.0);
        let near = (1.0 - i.dist / NEAR_RANGE).clamp(0.0, 1.0);
        let weight = focus / players.max(1) as f32;
        let others = self.load_on(i.player) - usize::from(is_current);
        let s = dmg / DAMAGE_UNIT * weight + near + if i.seen { 0.5 } else { 0.0 };
        let s = s + if is_current { STICKY } else { 0.0 } - SPREAD * others as f32;
        s.max(0.0)
    }

    /// Picks the enemy's target among `players`. `focus` is the party focus
    /// (`Scaling::focus`).
    pub fn choose(
        &mut self,
        enemy: FormId,
        now_ms: u64,
        focus: f32,
        players: &[ThreatInput],
    ) -> Option<Threat> {
        let cur = self.mem.get(&enemy).and_then(|m| m.current);
        let since = self.mem.get(&enemy).map_or(0, |m| m.since_ms);
        let n = players.len();
        let mut best: Option<(PlayerId, f32)> = None;
        let mut cur_score = 0.0;
        for i in players {
            let is_cur = Some(i.player) == cur;
            let s = self.score(enemy, i, focus, n, is_cur);
            if is_cur {
                cur_score = s;
            }
            if s > 0.0 && best.map_or(true, |(_, b)| s > b) {
                best = Some((i.player, s));
            }
        }
        let (bp, bs) = best?;
        let keep = cur.is_some()
            && cur_score > 0.0
            && (Some(bp) == cur
                || now_ms.saturating_sub(since) < MIN_HOLD_MS
                || bs < cur_score * SWITCH_MARGIN);
        let pick = if keep { cur.expect("checked") } else { bp };
        let m = self.mem.entry(enemy).or_default();
        let switched = m.current != Some(pick);
        if switched {
            m.current = Some(pick);
            m.since_ms = now_ms;
            self.switches += 1;
        }
        Some(Threat {
            target: pick,
            switched,
        })
    }
}
