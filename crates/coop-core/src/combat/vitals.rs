//! Host-side vitals of every player: the shadow health and ailments the host
//! uses to resolve damage that NPCs, explosions and other players deal to a
//! player who is not the host.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::{CombatMsg, DamageKind, EffectKind, LIMB_COUNT, MAX_PLAYER_HIT, MAX_REQ_PER_SEC};
use crate::ids::{FormId, PlayerId};
use crate::rules::Scaling;

/// Damage to one limb that cripples it.
pub const LIMB_MAX_DAMAGE: u16 = 60;
/// After the host hurts a player, how long its own mirror wins over the
/// player's avatar health (the hurt message is still on its way).
const HURT_GUARD_MS: u64 = 1_000;
/// Ticking effects fire once a second.
const DOT_MS: u64 = 1_000;
const MEMORY: usize = 1024;

/// Player-versus-player damage policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FriendlyFire {
    /// Hits between players do nothing (default).
    Off,
    /// Hits between players deal this fraction.
    Scaled(f32),
    /// Full damage.
    On,
}

/// What the host knows about one player's body.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Vitals {
    /// Health now.
    pub hp: u16,
    /// Health at full.
    pub max: u16,
    /// Waiting for a revive.
    pub downed: bool,
    /// Dead for good.
    pub dead: bool,
    /// Radiation taken since the last cure.
    pub rads: u32,
    /// Poison: points a second.
    pub poison: u16,
    /// Poison: ms left.
    pub poison_ms: u32,
    /// Bleeding limbs, ms left each.
    pub bleed_ms: BTreeMap<u8, u32>,
    /// Damage taken per limb since healed.
    pub limb_damage: BTreeMap<u8, u16>,
    /// Crippled limbs (bit per limb).
    pub crippled: u8,
    /// Knocked down until this time.
    pub knocked_until: u64,
    hurt_at: u64,
    next_dot: u64,
}

/// Counters for dumps and tests.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VitalsStats {
    /// NPC and explosion hits applied.
    pub applied: u32,
    /// Hits refused (duplicate, rate, bad victim).
    pub refused: u32,
    /// Player versus player hits ignored by the friendly-fire rule.
    pub friendly_blocked: u32,
    /// Player versus player hits that landed.
    pub friendly_landed: u32,
    /// Hits ignored because the victim was down.
    pub while_downed: u32,
    /// Limbs crippled.
    pub crippled: u32,
    /// Ticks of poison, bleed.
    pub dot_ticks: u32,
}

/// Every player's vitals plus the rules that touch them.
#[derive(Clone, Debug)]
pub struct VitalsHost {
    players: BTreeMap<PlayerId, Vitals>,
    /// Player versus player policy.
    pub friendly: FriendlyFire,
    /// Counters.
    pub stats: VitalsStats,
    seq: u32,
    seen: BTreeSet<(PlayerId, u32)>,
    seen_order: VecDeque<(PlayerId, u32)>,
    times: BTreeMap<PlayerId, VecDeque<u64>>,
}

impl Default for VitalsHost {
    fn default() -> Self {
        Self::new()
    }
}

fn clamp16(v: f32) -> u16 {
    v.round().clamp(0.0, f32::from(u16::MAX)) as u16
}

impl VitalsHost {
    /// New table, friendly fire off.
    pub fn new() -> Self {
        VitalsHost {
            players: BTreeMap::new(),
            friendly: FriendlyFire::Off,
            stats: VitalsStats::default(),
            seq: 0,
            seen: BTreeSet::new(),
            seen_order: VecDeque::new(),
            times: BTreeMap::new(),
        }
    }

    /// A player's vitals.
    pub fn get(&self, p: PlayerId) -> Option<&Vitals> {
        self.players.get(&p)
    }

    /// Players tracked.
    pub fn players(&self) -> impl Iterator<Item = (&PlayerId, &Vitals)> {
        self.players.iter()
    }

    /// Starts tracking a player at full health.
    pub fn track(&mut self, p: PlayerId, hp: u16, max: u16) {
        let v = self.players.entry(p).or_default();
        if v.max == 0 {
            v.hp = hp;
            v.max = max.max(1);
        }
    }

    /// Stops tracking.
    pub fn forget(&mut self, p: PlayerId) {
        self.players.remove(&p);
        self.times.remove(&p);
    }

    /// The player's own report (avatar stream): health, downed, dead. The
    /// host's mirror wins for a moment after it hurt the player.
    pub fn report(&mut self, p: PlayerId, now_ms: u64, hp: u16, downed: bool, dead: bool) {
        let Some(v) = self.players.get_mut(&p) else {
            return;
        };
        if now_ms.saturating_sub(v.hurt_at) >= HURT_GUARD_MS || v.hurt_at == 0 {
            v.hp = hp;
        } else {
            v.hp = v.hp.min(hp);
        }
        v.max = v.max.max(v.hp);
        if downed && !v.downed {
            v.bleed_ms.clear();
            v.poison_ms = 0;
        }
        v.downed = downed;
        v.dead = dead;
    }

    fn fresh(&mut self, from: PlayerId, id: u32, now_ms: u64) -> bool {
        if id == 0 || !self.seen.insert((from, id)) {
            self.stats.refused += 1;
            return false;
        }
        self.seen_order.push_back((from, id));
        while self.seen_order.len() > MEMORY {
            if let Some(k) = self.seen_order.pop_front() {
                self.seen.remove(&k);
            }
        }
        let t = self.times.entry(from).or_default();
        while t.front().is_some_and(|x| now_ms.saturating_sub(*x) >= 1000) {
            t.pop_front();
        }
        if t.len() >= MAX_REQ_PER_SEC {
            self.stats.refused += 1;
            return false;
        }
        t.push_back(now_ms);
        true
    }

    fn next_seq(&mut self) -> u32 {
        self.seq = self.seq.wrapping_add(1).max(1);
        self.seq
    }

    /// Damage from the world (an NPC, a blast, a mine) to a player.
    /// `scaling` raises it for the party size. Returns the messages for the
    /// victim's machine (the caller routes them; for the host player they
    /// are local).
    #[allow(clippy::too_many_arguments)]
    pub fn hurt(
        &mut self,
        now_ms: u64,
        victim: PlayerId,
        attacker: FormId,
        amount: u16,
        kind: DamageKind,
        limb: u8,
        scaling: &Scaling,
    ) -> Vec<CombatMsg> {
        let Some(v) = self.players.get(&victim) else {
            self.stats.refused += 1;
            return vec![];
        };
        if v.dead {
            self.stats.refused += 1;
            return vec![];
        }
        if v.downed {
            // a downed player is out of the fight until revived or timed out
            self.stats.while_downed += 1;
            return vec![];
        }
        let scaled = if kind.is_direct() {
            clamp16(f32::from(amount) * scaling.damage_to_players)
        } else {
            amount
        };
        let scaled = scaled.clamp(1, MAX_PLAYER_HIT);
        let seq = self.next_seq();
        let v = self.players.get_mut(&victim).expect("checked");
        v.hp = v.hp.saturating_sub(scaled);
        v.hurt_at = now_ms.max(1);
        let hp = v.hp;
        self.stats.applied += 1;
        let mut out = vec![CombatMsg::PlayerHurt {
            victim,
            seq,
            amount: scaled,
            kind,
            limb,
            attacker,
            health: hp,
        }];
        if kind.is_direct() && limb < LIMB_COUNT && limb != 1 {
            let d = v.limb_damage.entry(limb).or_insert(0);
            *d = d.saturating_add(scaled);
            if *d >= LIMB_MAX_DAMAGE && v.crippled & (1 << limb) == 0 {
                v.crippled |= 1 << limb;
                self.stats.crippled += 1;
                let seq = self.next_seq();
                out.push(CombatMsg::PlayerEffect {
                    victim,
                    seq,
                    effect: EffectKind::Cripple,
                    amount: 0,
                    limb,
                    ms: 0,
                });
            }
        }
        out
    }

    /// An NPC hit request from a peer (`from` owns the NPC).
    pub fn on_npc_hit(
        &mut self,
        from: PlayerId,
        now_ms: u64,
        msg: &CombatMsg,
        scaling: &Scaling,
    ) -> Vec<CombatMsg> {
        let CombatMsg::NpcHit {
            id,
            attacker,
            victim,
            amount,
            kind,
            limb,
        } = *msg
        else {
            return vec![];
        };
        if !self.fresh(from, id, now_ms) {
            return vec![];
        }
        self.hurt(now_ms, victim, attacker, amount, kind, limb, scaling)
    }

    /// One player's hit on another: the friendly-fire rule. Self-damage
    /// (own grenade) always counts in full.
    pub fn player_vs_player(
        &mut self,
        now_ms: u64,
        attacker: PlayerId,
        victim: PlayerId,
        amount: u16,
        kind: DamageKind,
        limb: u8,
    ) -> Vec<CombatMsg> {
        let by = FormId(0xFE00_0000 + u32::from(attacker.0));
        let none = Scaling::NEUTRAL;
        if attacker == victim {
            return self.hurt(now_ms, victim, by, amount, kind, limb, &none);
        }
        let k = match self.friendly {
            FriendlyFire::Off => {
                self.stats.friendly_blocked += 1;
                return vec![];
            }
            FriendlyFire::Scaled(k) => k.clamp(0.0, 1.0),
            FriendlyFire::On => 1.0,
        };
        let a = clamp16(f32::from(amount) * k);
        if a == 0 {
            self.stats.friendly_blocked += 1;
            return vec![];
        }
        self.stats.friendly_landed += 1;
        self.hurt(now_ms, victim, by, a, kind, limb, &none)
    }

    /// A player-versus-player request from a peer.
    pub fn on_player_hit(
        &mut self,
        from: PlayerId,
        now_ms: u64,
        msg: &CombatMsg,
    ) -> Vec<CombatMsg> {
        let CombatMsg::PlayerHit {
            id,
            victim,
            amount,
            kind,
            limb,
        } = *msg
        else {
            return vec![];
        };
        if !self.fresh(from, id, now_ms) {
            return vec![];
        }
        self.player_vs_player(now_ms, from, victim, amount, kind, limb)
    }

    /// A lasting effect from the world (radiation, poison, bleeding).
    pub fn effect(
        &mut self,
        now_ms: u64,
        victim: PlayerId,
        effect: EffectKind,
        amount: u16,
        limb: u8,
        ms: u32,
    ) -> Vec<CombatMsg> {
        let Some(v) = self.players.get_mut(&victim) else {
            return vec![];
        };
        if v.dead || (v.downed && effect != EffectKind::Cure) {
            return vec![];
        }
        match effect {
            EffectKind::Radiation => v.rads = v.rads.saturating_add(u32::from(amount)),
            EffectKind::Poison => {
                v.poison = v.poison.max(amount);
                v.poison_ms = v.poison_ms.max(ms);
            }
            EffectKind::Bleed => {
                let e = v.bleed_ms.entry(limb.min(LIMB_COUNT - 1)).or_insert(0);
                *e = (*e).max(ms);
            }
            EffectKind::Cripple => v.crippled |= 1 << limb.min(LIMB_COUNT - 1),
            EffectKind::Knockdown => v.knocked_until = v.knocked_until.max(now_ms + u64::from(ms)),
            EffectKind::Cure => {
                v.rads = 0;
                v.poison = 0;
                v.poison_ms = 0;
                v.bleed_ms.clear();
                v.crippled = 0;
                v.limb_damage.clear();
            }
        }
        let seq = self.next_seq();
        vec![CombatMsg::PlayerEffect {
            victim,
            seq,
            effect,
            amount,
            limb,
            ms,
        }]
    }

    /// Revive: back up at `pct` percent health, ailments cured.
    pub fn revived(&mut self, now_ms: u64, p: PlayerId, pct: u8) -> Vec<CombatMsg> {
        if let Some(v) = self.players.get_mut(&p) {
            v.downed = false;
            v.dead = false;
            v.hp = ((u32::from(v.max) * u32::from(pct.min(100))) / 100).max(1) as u16;
            v.hurt_at = now_ms.max(1);
        }
        self.effect(now_ms, p, EffectKind::Cure, 0, 0, 0)
    }

    /// Poison and bleeding tick once a second. Returns hurt messages.
    pub fn tick(&mut self, now_ms: u64, scaling: &Scaling) -> Vec<CombatMsg> {
        let ids: Vec<PlayerId> = self.players.keys().copied().collect();
        let mut out = Vec::new();
        for p in ids {
            let Some(v) = self.players.get_mut(&p) else {
                continue;
            };
            if v.dead || v.downed {
                continue;
            }
            if now_ms < v.next_dot {
                continue;
            }
            v.next_dot = now_ms + DOT_MS;
            let mut dose: u16 = 0;
            let mut kind = DamageKind::Poison;
            if v.poison_ms > 0 {
                dose = dose.saturating_add(v.poison);
                v.poison_ms = v.poison_ms.saturating_sub(DOT_MS as u32);
                if v.poison_ms == 0 {
                    v.poison = 0;
                }
            }
            if !v.bleed_ms.is_empty() {
                dose = dose.saturating_add(v.bleed_ms.len() as u16);
                if v.poison_ms == 0 && dose as usize == v.bleed_ms.len() {
                    kind = DamageKind::Bleed;
                }
                v.bleed_ms.retain(|_, ms| {
                    *ms = ms.saturating_sub(DOT_MS as u32);
                    *ms > 0
                });
            }
            if dose > 0 {
                self.stats.dot_ticks += 1;
                out.extend(self.hurt(now_ms, p, FormId(0), dose, kind, LIMB_COUNT, scaling));
            }
        }
        out
    }
}
