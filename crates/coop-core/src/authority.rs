//! Authority table: who may write what.
//!
//! Three authorities: story state is the host's; an actor has one owner with
//! an epoch (handoffs bump the epoch, fencing late writes from the old
//! owner); a player owns their own avatar. The host may write anything.

use std::collections::BTreeMap;

use crate::ids::{Epoch, FormId, PlayerId};
use crate::schema::{Field, WriteClass};

/// Why a write was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthError {
    /// Story state (or an unowned form) is host-only.
    NotHost,
    /// The form is owned by someone else.
    NotOwner {
        /// Current owner.
        owner: PlayerId,
    },
    /// The writer owned the form once but the epoch moved on.
    StaleEpoch {
        /// The epoch that is current.
        current: Epoch,
    },
    /// The form is another player's avatar.
    NotAvatarOwner {
        /// Avatar owner.
        owner: PlayerId,
    },
}

/// Who holds authority over a form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authority {
    /// Host (story or unowned).
    Host,
    /// Actor owned by a player at an epoch.
    Actor {
        /// Owner.
        owner: PlayerId,
        /// Current epoch.
        epoch: Epoch,
    },
    /// A player's own avatar.
    Avatar(PlayerId),
}

/// The authority table.
#[derive(Clone, Debug, Default)]
pub struct AuthorityTable {
    actors: BTreeMap<FormId, (PlayerId, Epoch)>,
    last_epoch: BTreeMap<FormId, Epoch>,
    avatars: BTreeMap<FormId, PlayerId>,
}

impl AuthorityTable {
    /// Empty table: everything is host authority.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gives `owner` the actor, bumping its epoch. Returns the new epoch.
    pub fn claim_actor(&mut self, form: FormId, owner: PlayerId) -> Epoch {
        let epoch = self
            .last_epoch
            .get(&form)
            .copied()
            .unwrap_or_default()
            .next();
        self.last_epoch.insert(form, epoch);
        self.actors.insert(form, (owner, epoch));
        epoch
    }

    /// Returns the actor to the host (the epoch still advances on next claim).
    pub fn release_actor(&mut self, form: FormId) {
        self.actors.remove(&form);
    }

    /// Releases every actor owned by `player` (disconnect). Returns them.
    pub fn release_all_of(&mut self, player: PlayerId) -> Vec<FormId> {
        let gone: Vec<FormId> = self
            .actors
            .iter()
            .filter(|(_, (o, _))| *o == player)
            .map(|(f, _)| *f)
            .collect();
        for f in &gone {
            self.actors.remove(f);
        }
        gone
    }

    /// Declares `form` the avatar of `player`.
    pub fn register_avatar(&mut self, player: PlayerId, form: FormId) {
        self.avatars.insert(form, player);
    }

    /// Who holds authority over `form`.
    pub fn authority_of(&self, form: FormId) -> Authority {
        if let Some(p) = self.avatars.get(&form) {
            return Authority::Avatar(*p);
        }
        match self.actors.get(&form) {
            Some((owner, epoch)) => Authority::Actor {
                owner: *owner,
                epoch: *epoch,
            },
            None => Authority::Host,
        }
    }

    /// Checks whether `writer` (claiming `epoch` for actor writes) may write
    /// `field` on `form`.
    pub fn check_write(
        &self,
        writer: PlayerId,
        epoch: Epoch,
        form: FormId,
        field: &Field,
    ) -> Result<(), AuthError> {
        if writer.is_host() {
            return Ok(());
        }
        if field.class() == WriteClass::Story {
            return Err(AuthError::NotHost);
        }
        match self.authority_of(form) {
            Authority::Host => Err(AuthError::NotHost),
            Authority::Avatar(p) if p == writer => Ok(()),
            Authority::Avatar(p) => Err(AuthError::NotAvatarOwner { owner: p }),
            Authority::Actor { owner, epoch: cur } => {
                if owner != writer {
                    Err(AuthError::NotOwner { owner })
                } else if cur != epoch {
                    Err(AuthError::StaleEpoch { current: cur })
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: PlayerId = PlayerId(1);
    const B: PlayerId = PlayerId(2);
    fn pos() -> Field {
        Field::Position { x: 1, y: 2, z: 3 }
    }

    #[test]
    fn host_can_write_anything() {
        let t = AuthorityTable::new();
        assert!(t
            .check_write(
                PlayerId::HOST,
                Epoch(0),
                FormId(1),
                &Field::QuestStage { stage: 1 }
            )
            .is_ok());
        assert!(t
            .check_write(PlayerId::HOST, Epoch(0), FormId(1), &pos())
            .is_ok());
    }

    #[test]
    fn guests_cannot_write_story() {
        let mut t = AuthorityTable::new();
        t.register_avatar(A, FormId(7));
        for f in [
            Field::QuestStage { stage: 1 },
            Field::Global { bits: 0 },
            Field::Karma { value: 1 },
            Field::Topic { known: true },
        ] {
            assert_eq!(
                t.check_write(A, Epoch(0), FormId(7), &f),
                Err(AuthError::NotHost)
            );
        }
    }

    #[test]
    fn unowned_entity_is_host_only() {
        let t = AuthorityTable::new();
        assert_eq!(
            t.check_write(A, Epoch(0), FormId(9), &pos()),
            Err(AuthError::NotHost)
        );
    }

    #[test]
    fn avatar_rules() {
        let mut t = AuthorityTable::new();
        t.register_avatar(A, FormId(7));
        assert!(t.check_write(A, Epoch(0), FormId(7), &pos()).is_ok());
        assert_eq!(
            t.check_write(B, Epoch(0), FormId(7), &pos()),
            Err(AuthError::NotAvatarOwner { owner: A })
        );
    }

    #[test]
    fn actor_epochs_fence_old_owner() {
        let mut t = AuthorityTable::new();
        let e1 = t.claim_actor(FormId(20), A);
        assert!(t.check_write(A, e1, FormId(20), &pos()).is_ok());
        let e2 = t.claim_actor(FormId(20), B);
        assert!(e2 > e1);
        assert_eq!(
            t.check_write(A, e1, FormId(20), &pos()),
            Err(AuthError::NotOwner { owner: B })
        );
        assert!(t.check_write(B, e2, FormId(20), &pos()).is_ok());
        assert_eq!(
            t.check_write(B, e1, FormId(20), &pos()),
            Err(AuthError::StaleEpoch { current: e2 })
        );
        // hand back to A: epoch keeps growing, old A epoch stays dead
        let e3 = t.claim_actor(FormId(20), A);
        assert!(e3 > e2);
        assert_eq!(
            t.check_write(A, e1, FormId(20), &pos()),
            Err(AuthError::StaleEpoch { current: e3 })
        );
    }

    #[test]
    fn release_returns_to_host() {
        let mut t = AuthorityTable::new();
        t.claim_actor(FormId(20), A);
        t.claim_actor(FormId(21), A);
        t.claim_actor(FormId(22), B);
        let mut g = t.release_all_of(A);
        g.sort();
        assert_eq!(g, vec![FormId(20), FormId(21)]);
        assert_eq!(t.authority_of(FormId(20)), Authority::Host);
        assert!(matches!(
            t.authority_of(FormId(22)),
            Authority::Actor { .. }
        ));
        t.release_actor(FormId(22));
        assert_eq!(t.authority_of(FormId(22)), Authority::Host);
    }
}
