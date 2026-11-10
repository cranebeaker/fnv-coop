//! Sleeping and waiting in a session (docs/coop/SPECS.md S17): coop-core's vote, asked and
//! answered through the game's own sleep and wait menu.
//!
//! A player who presses rest in the menu proposes the rest to the party (`VoteRequest`: sleep in
//! a bed, wait otherwise, the hours as the parameter). The other players are told on the HUD;
//! each agrees by pressing rest in the own menu while the vote is open (a `VoteCast` yes), and
//! the vote fails when one does not within coop-core's time limit. When all agreed, every game
//! passes the hours itself through the game's own hour step, and the shared clock then holds
//! them on the host's time.

use coop_core::rules::{Notice, Prompt, RulesClient, RulesMsg, VoteKind};
use coop_engine::{rest, talk};
use engine_core::Engine;

fn is_rest(kind: VoteKind) -> bool {
    matches!(kind, VoteKind::Sleep | VoteKind::Wait)
}

fn verb(kind: VoteKind) -> &'static str {
    if kind == VoteKind::Sleep {
        "sleep"
    } else {
        "wait"
    }
}

/// One machine's part in the rest vote.
#[derive(Debug, Default)]
pub struct Rester {
    /// The vote this player was last told about.
    told: Option<u32>,
    /// Rests this player proposed.
    pub proposed: u32,
    /// Votes this player agreed to.
    pub agreed: u32,
    /// Rests the party agreed on and this game passed.
    pub passed: u32,
    /// Hours passed that way.
    pub hours: u32,
    /// Rests the party did not agree on.
    pub refused: u32,
}

impl Rester {
    /// One step: this game's rest clicks become proposals or agreement, an open vote is
    /// announced once, and a vote's outcome is carried out. Returns the rules messages to send
    /// to the host's table, and whether this game just passed hours.
    pub fn step(&mut self, e: &mut Engine, rules: &mut RulesClient) -> (Vec<RulesMsg>, bool) {
        let mut out = Vec::new();
        for asked in rest::take_asked(e) {
            let open = matches!(&rules.prompt, Some(Prompt::Vote { kind, .. }) if is_rest(*kind));
            if open {
                self.agreed += 1;
                out.extend(rules.answer(true));
            } else {
                self.proposed += 1;
                out.push(RulesMsg::VoteRequest {
                    kind: if asked.in_bed {
                        VoteKind::Sleep
                    } else {
                        VoteKind::Wait
                    },
                    param: asked.hours,
                });
            }
        }
        if let Some(Prompt::Vote {
            id,
            kind,
            param,
            proposer,
        }) = &rules.prompt
        {
            if is_rest(*kind) && self.told != Some(*id) {
                self.told = Some(*id);
                talk::show(
                    e,
                    &format!(
                        "Player {} wants to {} {param} hour(s). Rest yourself to agree.",
                        u32::from(proposer.0) + 1,
                        verb(*kind)
                    ),
                );
            }
        }
        let mut passed = false;
        let notices = std::mem::take(&mut rules.notices);
        for n in notices {
            match n {
                Notice::VoteDone {
                    kind,
                    param,
                    approved,
                } if is_rest(kind) => {
                    if approved && rest::pass(e, param, kind == VoteKind::Sleep) {
                        self.passed += 1;
                        self.hours += param.clamp(1, rest::MAX_HOURS);
                        passed = true;
                        talk::show(e, &format!("The party rests {param} hour(s)."));
                    } else {
                        self.refused += 1;
                        talk::show(e, "The party did not agree to rest.");
                    }
                }
                // Other notices are for the features they belong to.
                other => rules.notices.push(other),
            }
        }
        (out, passed)
    }
}
