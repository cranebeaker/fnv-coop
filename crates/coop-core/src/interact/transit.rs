//! Who may use a load door, and who goes along.
//!
//! * The leader (the host player by default) goes where they like; every
//!   member standing in the cell group they leave follows.
//! * Anyone else may take a door only to a cell in the leader's cell group
//!   (the leader's cell or one linked to it): they cannot walk away into a
//!   different part of the world alone.
//! * A one-way transition needs the whole party to agree (a barrier).

use super::Why;
use crate::ids::{CellId, PlayerId};
use crate::interest::InterestMap;

/// What the host does with a transition request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decision {
    /// Go now; these members load the destination too.
    Go {
        /// Followers (never the requester).
        followers: Vec<PlayerId>,
    },
    /// Open a party barrier; the move happens when everyone agrees.
    Barrier,
    /// Refused.
    Deny(Why),
}

/// Transition policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transit {
    /// The party leader.
    pub leader: PlayerId,
    /// Followers are the members in the cell group the leader leaves.
    pub follow_leader: bool,
}

impl Default for Transit {
    fn default() -> Self {
        Transit {
            leader: PlayerId::HOST,
            follow_leader: true,
        }
    }
}

impl Transit {
    /// Decides a request by `from` to go `from_cell` to `to_cell`.
    pub fn decide(
        &self,
        from: PlayerId,
        from_cell: CellId,
        to_cell: CellId,
        one_way: bool,
        members: &[PlayerId],
        interest: &InterestMap,
    ) -> Decision {
        if one_way && members.len() > 1 {
            return Decision::Barrier;
        }
        if from == self.leader {
            let followers = if self.follow_leader {
                members
                    .iter()
                    .copied()
                    .filter(|m| *m != from)
                    .filter(|m| {
                        interest
                            .peer_cell(*m)
                            .map_or(false, |c| interest.visible(from_cell, c))
                    })
                    .collect()
            } else {
                Vec::new()
            };
            return Decision::Go { followers };
        }
        let allowed = match interest.peer_cell(self.leader) {
            Some(lc) => interest.visible(lc, to_cell),
            None => true,
        };
        if allowed {
            Decision::Go {
                followers: Vec::new(),
            }
        } else {
            Decision::Deny(Why::Policy)
        }
    }

    /// Everyone who loads the destination when a barrier releases.
    pub fn everyone(&self, requester: PlayerId, members: &[PlayerId]) -> Vec<PlayerId> {
        members
            .iter()
            .copied()
            .filter(|m| *m != requester)
            .collect()
    }
}
