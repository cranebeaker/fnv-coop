//! Guest side of the rules: mirrors what the host announced so the UI and
//! game can react (warp, warning banner, vote prompt, downed state).

use std::collections::BTreeSet;

use super::difficulty::{Mode, Scaling};
use super::{CancelReason, Gift, RulesMsg, VoteKind};
use crate::ids::PlayerId;

/// A prompt waiting for this player's answer.
#[derive(Clone, Debug, PartialEq)]
pub enum Prompt {
    /// A barrier needs a yes or no.
    Barrier {
        /// Barrier id.
        id: u32,
        /// Transition tag.
        tag: u32,
    },
    /// A vote needs a yes or no.
    Vote {
        /// Vote id.
        id: u32,
        /// What.
        kind: VoteKind,
        /// Parameter.
        param: u32,
        /// Proposer.
        proposer: PlayerId,
    },
}

/// Something the local game must do.
#[derive(Clone, Debug, PartialEq)]
pub enum Notice {
    /// Show "you will be warped in N ms".
    Warn(u32),
    /// Warp to the anchor.
    Warp(u32),
    /// Barrier passed: warp for `tag`.
    BarrierPassed(u32),
    /// Barrier cancelled.
    BarrierCancelled(u32, CancelReason),
    /// Vote finished.
    VoteDone {
        /// What.
        kind: VoteKind,
        /// Parameter.
        param: u32,
        /// Approved or not.
        approved: bool,
    },
    /// Local player was revived: restore this health percent.
    Revived(u8),
    /// Local player must respawn at the checkpoint.
    Respawn,
    /// Everyone is down.
    Wipe,
    /// The shared story rewarded the party: give the local player the same.
    Reward(Gift),
}

/// Rules state for one machine.
#[derive(Clone, Debug)]
pub struct RulesClient {
    me: PlayerId,
    /// Open prompt, if any.
    pub prompt: Option<Prompt>,
    /// Players currently down.
    pub downed: BTreeSet<PlayerId>,
    /// Who is reviving whom (target, helper).
    pub reviving: Vec<(PlayerId, PlayerId)>,
    /// Latest host scaling.
    pub scaling: Scaling,
    /// The host's campaign mode (`None` until the first scaling announcement).
    pub mode: Option<Mode>,
    /// The host's friendly-fire choice, from the same announcement.
    pub friendly_fire: bool,
    /// Pending notices for the game.
    pub notices: Vec<Notice>,
}

impl RulesClient {
    /// The player this client belongs to.
    pub fn me(&self) -> PlayerId {
        self.me
    }

    /// A client for player `me`.
    pub fn new(me: PlayerId) -> Self {
        RulesClient {
            me,
            prompt: None,
            downed: BTreeSet::new(),
            reviving: Vec::new(),
            scaling: Scaling::NEUTRAL,
            mode: None,
            friendly_fire: false,
            notices: Vec::new(),
        }
    }

    /// Answers the open prompt.
    pub fn answer(&mut self, yes: bool) -> Option<RulesMsg> {
        match self.prompt.take()? {
            Prompt::Barrier { id, .. } => Some(if yes {
                RulesMsg::BarrierReady { id }
            } else {
                RulesMsg::BarrierDecline { id }
            }),
            Prompt::Vote { id, .. } => Some(RulesMsg::VoteCast { id, yes }),
        }
    }

    /// Reports the local player's health reached zero.
    pub fn report_down(&self, revivable: bool) -> RulesMsg {
        RulesMsg::ReportDown { revivable }
    }

    /// Applies a host message.
    pub fn on_msg(&mut self, msg: RulesMsg) {
        match msg {
            RulesMsg::TetherWarn { in_ms } => self.notices.push(Notice::Warn(in_ms)),
            RulesMsg::TetherWarp { anchor } => self.notices.push(Notice::Warp(anchor)),
            RulesMsg::BarrierOpen { id, tag, required } => {
                if required.contains(&self.me) {
                    self.prompt = Some(Prompt::Barrier { id, tag });
                }
            }
            RulesMsg::BarrierResult {
                id,
                tag,
                released,
                reason,
            } => {
                if matches!(self.prompt, Some(Prompt::Barrier { id: i, .. }) if i == id) {
                    self.prompt = None;
                }
                self.notices.push(if released {
                    Notice::BarrierPassed(tag)
                } else {
                    Notice::BarrierCancelled(tag, reason.unwrap_or(CancelReason::Timeout))
                });
            }
            RulesMsg::VoteOpen {
                id,
                kind,
                param,
                proposer,
            } => {
                if proposer != self.me {
                    self.prompt = Some(Prompt::Vote {
                        id,
                        kind,
                        param,
                        proposer,
                    });
                }
            }
            RulesMsg::VoteResult {
                id,
                kind,
                param,
                approved,
            } => {
                if matches!(self.prompt, Some(Prompt::Vote { id: i, .. }) if i == id) {
                    self.prompt = None;
                }
                self.notices.push(Notice::VoteDone {
                    kind,
                    param,
                    approved,
                });
            }
            RulesMsg::Downed { player } => {
                self.downed.insert(player);
            }
            RulesMsg::Revived {
                player, health_pct, ..
            } => {
                self.downed.remove(&player);
                if player == self.me {
                    self.notices.push(Notice::Revived(health_pct));
                }
            }
            RulesMsg::Respawn { player } => {
                self.downed.remove(&player);
                self.reviving.retain(|(t, _)| *t != player);
                if player == self.me {
                    self.notices.push(Notice::Respawn);
                }
            }
            RulesMsg::ReviveProgress { target, helper } => {
                self.reviving.retain(|(t, _)| *t != target);
                if let Some(h) = helper {
                    self.reviving.push((target, h));
                }
            }
            RulesMsg::Wipe => self.notices.push(Notice::Wipe),
            RulesMsg::Reward { gift, .. } => self.notices.push(Notice::Reward(gift)),
            RulesMsg::ScalingSet {
                players,
                mode,
                friendly_fire,
            } => {
                self.scaling = Scaling::for_party(players as u32, mode);
                self.mode = Some(mode);
                self.friendly_fire = friendly_fire;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barrier_prompt_and_answer() {
        let mut c = RulesClient::new(PlayerId(1));
        c.on_msg(RulesMsg::BarrierOpen {
            id: 3,
            tag: 9,
            required: vec![PlayerId(0), PlayerId(1)],
        });
        assert_eq!(c.answer(true), Some(RulesMsg::BarrierReady { id: 3 }));
        assert!(c.answer(true).is_none());
        c.on_msg(RulesMsg::BarrierOpen {
            id: 4,
            tag: 9,
            required: vec![PlayerId(1)],
        });
        assert_eq!(c.answer(false), Some(RulesMsg::BarrierDecline { id: 4 }));
    }

    #[test]
    fn not_required_gets_no_prompt_and_result_clears_it() {
        let mut c = RulesClient::new(PlayerId(5));
        c.on_msg(RulesMsg::BarrierOpen {
            id: 1,
            tag: 1,
            required: vec![PlayerId(0)],
        });
        assert!(c.prompt.is_none());
        let mut c = RulesClient::new(PlayerId(0));
        c.on_msg(RulesMsg::BarrierOpen {
            id: 1,
            tag: 1,
            required: vec![PlayerId(0)],
        });
        c.on_msg(RulesMsg::BarrierResult {
            id: 1,
            tag: 1,
            released: true,
            reason: None,
        });
        assert!(c.prompt.is_none());
        assert_eq!(c.notices, vec![Notice::BarrierPassed(1)]);
    }

    #[test]
    fn proposer_is_not_prompted() {
        let mut c = RulesClient::new(PlayerId(1));
        c.on_msg(RulesMsg::VoteOpen {
            id: 1,
            kind: VoteKind::Sleep,
            param: 8,
            proposer: PlayerId(1),
        });
        assert!(c.prompt.is_none());
        c.on_msg(RulesMsg::VoteOpen {
            id: 2,
            kind: VoteKind::Sleep,
            param: 8,
            proposer: PlayerId(0),
        });
        assert_eq!(
            c.answer(true),
            Some(RulesMsg::VoteCast { id: 2, yes: true })
        );
    }

    #[test]
    fn downed_revive_respawn_tracking() {
        let mut c = RulesClient::new(PlayerId(1));
        c.on_msg(RulesMsg::Downed {
            player: PlayerId(1),
        });
        c.on_msg(RulesMsg::ReviveProgress {
            target: PlayerId(1),
            helper: Some(PlayerId(0)),
        });
        assert_eq!(c.reviving, vec![(PlayerId(1), PlayerId(0))]);
        c.on_msg(RulesMsg::Revived {
            player: PlayerId(1),
            helper: PlayerId(0),
            health_pct: 30,
        });
        assert!(c.downed.is_empty());
        assert_eq!(c.notices, vec![Notice::Revived(30)]);
        c.on_msg(RulesMsg::Downed {
            player: PlayerId(0),
        });
        c.on_msg(RulesMsg::Respawn {
            player: PlayerId(0),
        });
        assert!(c.downed.is_empty());
        assert_eq!(c.notices.len(), 1, "only our own respawn notifies");
    }
}
