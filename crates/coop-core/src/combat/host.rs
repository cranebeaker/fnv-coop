//! The host's combat judge: one `on_msg` over vitals, fire relay, blasts and
//! VATS, plus the queues of work the game side must carry out (area damage
//! needs positions only the world knows).

use super::{
    BlastEvent, CombatMsg, ExtrasPlanner, FireEvent, FireHub, ThreatDirector, VatsHost, VitalsHost,
};
use crate::ids::{CellId, FormId, PlayerId};
use crate::interest::Audience;
use crate::rules::Scaling;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct COut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: CombatMsg,
}

/// Who damaged or killed an actor (read from the actor ledger's verdicts).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attribution {
    /// The actor.
    pub actor: FormId,
    /// The player.
    pub by: PlayerId,
    /// Points applied (0 for a bare kill notice).
    pub amount: u16,
    /// The hit killed it.
    pub killed: bool,
}

/// Damage a VATS shot must put on an actor through the actor ledger.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VatsHit {
    /// Shooter.
    pub from: PlayerId,
    /// Target.
    pub target: FormId,
    /// Points.
    pub amount: u16,
    /// Unique hit id for the ledger.
    pub hit: u32,
}

/// Host-side combat state.
#[derive(Clone, Debug, Default)]
pub struct CombatHost {
    /// Player vitals and the friendly-fire rule.
    pub vitals: VitalsHost,
    /// Shots and blasts validation.
    pub fire: FireHub,
    /// VATS.
    pub vats: VatsHost,
    /// Attention.
    pub threat: ThreatDirector,
    /// Encounter extras.
    pub extras: ExtrasPlanner,
    /// Hits and kills the ledger attributed to players (drained by the game
    /// for crime and reputation).
    pub attributions: Vec<Attribution>,
    /// Accepted blasts to resolve on the world (drained by the game).
    pub blasts: Vec<BlastEvent>,
    /// Accepted shots (drained by the game: noise, decals).
    pub fires: Vec<FireEvent>,
    /// VATS hits to put on the actor ledger (drained by the live layer).
    pub vats_hits: Vec<VatsHit>,
}

impl CombatHost {
    /// Fresh host state.
    pub fn new() -> Self {
        Self::default()
    }

    fn to_victims(out: &mut Vec<COut>, msgs: Vec<CombatMsg>) {
        for m in msgs {
            let victim = match &m {
                CombatMsg::PlayerHurt { victim, .. } | CombatMsg::PlayerEffect { victim, .. } => {
                    *victim
                }
                _ => continue,
            };
            out.push(COut {
                aud: Audience::To(victim),
                msg: m,
            });
        }
    }

    /// Routes vitals messages to their victims.
    pub fn deliver(msgs: Vec<CombatMsg>) -> Vec<COut> {
        let mut out = Vec::new();
        Self::to_victims(&mut out, msgs);
        out
    }

    /// Handles one combat message from `from`. `at` is where the host
    /// believes `from` stands, `cell` their cell.
    pub fn on_msg(
        &mut self,
        from: PlayerId,
        msg: CombatMsg,
        now_ms: u64,
        at: Option<[f32; 3]>,
        cell: Option<CellId>,
        scaling: &Scaling,
    ) -> Vec<COut> {
        let mut out = Vec::new();
        if !msg.is_sane() {
            return out;
        }
        match msg {
            CombatMsg::NpcHit { .. } => {
                let m = self.vitals.on_npc_hit(from, now_ms, &msg, scaling);
                Self::to_victims(&mut out, m);
            }
            CombatMsg::PlayerHit { .. } => {
                let m = self.vitals.on_player_hit(from, now_ms, &msg);
                Self::to_victims(&mut out, m);
            }
            CombatMsg::Fire(e) => {
                if let Some(e) = self.fire.on_fire(from, now_ms, at, e) {
                    out.push(COut {
                        aud: Audience::CellExcept(e.cell, from),
                        msg: CombatMsg::Fire(e),
                    });
                    self.fires.push(e);
                }
            }
            CombatMsg::Blast(b) => {
                if let Some(b) = self.fire.on_blast(from, now_ms, at, b) {
                    out.push(COut {
                        aud: Audience::CellExcept(b.cell, from),
                        msg: CombatMsg::Blast(b),
                    });
                    self.blasts.push(b);
                }
            }
            CombatMsg::VatsQueue { id, ap, shots } => {
                let (results, spent) = self.vats.resolve(from, id, ap, &shots, scaling);
                for (i, (r, s)) in results.iter().zip(shots.iter()).enumerate() {
                    if r.hit && r.damage > 0 {
                        self.vats_hits.push(VatsHit {
                            from,
                            target: s.target,
                            amount: r.damage,
                            hit: 0x8000_0000 | (id & 0x07FF_FFFF) << 4 | i as u32,
                        });
                    }
                }
                out.push(COut {
                    aud: Audience::To(from),
                    msg: CombatMsg::VatsDone { id, results, spent },
                });
            }
            CombatMsg::VatsMark { active, .. } => {
                self.vats.mark(from, active);
                let aud = cell.map_or(Audience::AllExcept(from), |c| Audience::CellExcept(c, from));
                out.push(COut {
                    aud,
                    msg: CombatMsg::VatsMark {
                        player: from,
                        active,
                    },
                });
            }
            _ => {}
        }
        out
    }
}
