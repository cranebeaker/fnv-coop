//! What every machine keeps of the combat layer: ids for its requests and
//! the host's verdicts waiting for the game to apply.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::{BlastEvent, CombatMsg, DamageKind, EffectKind, FireEvent, VatsResult, VatsShot};
use crate::ids::{FormId, PlayerId};

const MEMORY: usize = 256;

/// A hurt for the local player to take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HurtNote {
    /// Points.
    pub amount: u16,
    /// Kind.
    pub kind: DamageKind,
    /// Limb.
    pub limb: u8,
    /// Who.
    pub attacker: FormId,
    /// Host's health afterwards.
    pub health: u16,
}

/// An effect for the local player to take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EffectNote {
    /// What.
    pub effect: EffectKind,
    /// Amount.
    pub amount: u16,
    /// Limb.
    pub limb: u8,
    /// Duration.
    pub ms: u32,
}

/// Client side.
#[derive(Clone, Debug)]
pub struct CombatClient {
    me: PlayerId,
    next_id: u32,
    seen_seq: BTreeSet<u32>,
    seen_order: VecDeque<u32>,
    /// Hurts to apply to the local player, oldest first.
    pub hurts: Vec<HurtNote>,
    /// Effects to apply to the local player.
    pub effects: Vec<EffectNote>,
    /// Other players' shots to show.
    pub fires: Vec<FireEvent>,
    /// Explosions to show.
    pub blasts: Vec<BlastEvent>,
    /// VATS results by queue id.
    pub vats_done: BTreeMap<u32, (Vec<VatsResult>, u16)>,
    /// Players in VATS (presentation).
    pub vats_active: BTreeSet<PlayerId>,
    /// Other players' (sneak skill, level), as the host announced.
    pub stealth: BTreeMap<PlayerId, (u8, u16)>,
    /// Hurts applied (counter).
    pub hurts_seen: u32,
}

impl CombatClient {
    /// Client for `me`.
    pub fn new(me: PlayerId) -> Self {
        CombatClient {
            me,
            next_id: 1,
            seen_seq: BTreeSet::new(),
            seen_order: VecDeque::new(),
            hurts: Vec::new(),
            effects: Vec::new(),
            fires: Vec::new(),
            blasts: Vec::new(),
            vats_done: BTreeMap::new(),
            vats_active: BTreeSet::new(),
            stealth: BTreeMap::new(),
            hurts_seen: 0,
        }
    }

    fn id(&mut self) -> u32 {
        let i = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        i
    }

    /// My NPC hurt a player.
    pub fn npc_hit(
        &mut self,
        attacker: FormId,
        victim: PlayerId,
        amount: u16,
        kind: DamageKind,
        limb: u8,
    ) -> CombatMsg {
        CombatMsg::NpcHit {
            id: self.id(),
            attacker,
            victim,
            amount: amount.max(1),
            kind,
            limb,
        }
    }

    /// I hit another player.
    pub fn player_hit(
        &mut self,
        victim: PlayerId,
        amount: u16,
        kind: DamageKind,
        limb: u8,
    ) -> CombatMsg {
        CombatMsg::PlayerHit {
            id: self.id(),
            victim,
            amount: amount.max(1),
            kind,
            limb,
        }
    }

    /// I fired.
    pub fn fire(&mut self, mut e: FireEvent) -> CombatMsg {
        e.id = self.id();
        e.shooter = self.me;
        CombatMsg::Fire(e)
    }

    /// Something exploded because of me.
    pub fn blast(&mut self, mut e: BlastEvent) -> CombatMsg {
        e.id = self.id();
        e.source = self.me;
        CombatMsg::Blast(e)
    }

    /// I queued VATS shots; returns the queue id and the message.
    pub fn vats(&mut self, ap: u16, shots: Vec<VatsShot>) -> (u32, CombatMsg) {
        let id = self.id();
        (id, CombatMsg::VatsQueue { id, ap, shots })
    }

    /// I entered or left VATS.
    pub fn vats_mark(&self, active: bool) -> CombatMsg {
        CombatMsg::VatsMark {
            player: self.me,
            active,
        }
    }

    fn fresh(&mut self, seq: u32) -> bool {
        if !self.seen_seq.insert(seq) {
            return false;
        }
        self.seen_order.push_back(seq);
        while self.seen_order.len() > MEMORY {
            if let Some(s) = self.seen_order.pop_front() {
                self.seen_seq.remove(&s);
            }
        }
        true
    }

    /// Applies a host message.
    pub fn on_msg(&mut self, msg: CombatMsg) {
        match msg {
            CombatMsg::PlayerHurt {
                victim,
                seq,
                amount,
                kind,
                limb,
                attacker,
                health,
            } if victim == self.me => {
                if self.fresh(seq) {
                    self.hurts_seen += 1;
                    self.hurts.push(HurtNote {
                        amount,
                        kind,
                        limb,
                        attacker,
                        health,
                    });
                }
            }
            CombatMsg::PlayerEffect {
                victim,
                seq,
                effect,
                amount,
                limb,
                ms,
            } if victim == self.me => {
                if self.fresh(seq) {
                    self.effects.push(EffectNote {
                        effect,
                        amount,
                        limb,
                        ms,
                    });
                }
            }
            CombatMsg::Fire(e) if e.shooter != self.me => self.fires.push(e),
            CombatMsg::Blast(b) if b.source != self.me => self.blasts.push(b),
            CombatMsg::VatsDone { id, results, spent } => {
                self.vats_done.insert(id, (results, spent));
            }
            CombatMsg::Stealth {
                player,
                sneak,
                level,
            } => {
                self.stealth.insert(player, (sneak, level));
            }
            CombatMsg::VatsMark { player, active } if player != self.me => {
                if active {
                    self.vats_active.insert(player);
                } else {
                    self.vats_active.remove(&player);
                }
            }
            _ => {}
        }
    }
}
