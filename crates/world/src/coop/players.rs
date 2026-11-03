//! Host-side registry of the players: each gets a synthetic puppet form and a
//! record the rules and the viewer read (name, cell, position, health).

use std::collections::BTreeMap;

use coop_core::avatar::{dequant_heading, dequant_pos, AvatarState, FLAG_DOWNED};
use coop_core::ids::PlayerId;
use coop_core::sync::HostLive;
use esm::FormId;

use super::facts::{player_of, puppet_of};

/// What the host knows about one player.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlayerRecord {
    /// Display name.
    pub name: String,
    /// The synthetic reference that stands for them in the world.
    pub puppet: FormId,
    /// Cell form id (0 when unknown).
    pub cell: u32,
    /// Position.
    pub position: [f32; 3],
    /// Heading in radians.
    pub heading: f32,
    /// Health points.
    pub health: u16,
    /// Downed.
    pub downed: bool,
    /// Items gained from containers (net), by item.
    pub gained: BTreeMap<FormId, i64>,
    /// Newest avatar sample seen.
    pub seen: bool,
}

/// Players by id.
#[derive(Clone, Debug, Default)]
pub struct PlayerRegistry {
    players: BTreeMap<PlayerId, PlayerRecord>,
}

impl PlayerRegistry {
    /// An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a player (idempotent) and returns their record.
    pub fn join(&mut self, p: PlayerId, name: &str) -> &mut PlayerRecord {
        let r = self.players.entry(p).or_default();
        r.puppet = puppet_of(p);
        if r.name.is_empty() {
            r.name = name.to_string();
        }
        r
    }

    /// Removes a player.
    pub fn leave(&mut self, p: PlayerId) {
        self.players.remove(&p);
    }

    /// A player's record.
    pub fn get(&self, p: PlayerId) -> Option<&PlayerRecord> {
        self.players.get(&p)
    }

    /// The puppet form of a registered player.
    pub fn puppet(&self, p: PlayerId) -> Option<FormId> {
        self.players.get(&p).map(|r| r.puppet)
    }

    /// The player behind a puppet form.
    pub fn player(&self, puppet: FormId) -> Option<PlayerId> {
        player_of(puppet).filter(|p| self.players.contains_key(p))
    }

    /// All players, host first.
    pub fn iter(&self) -> impl Iterator<Item = (&PlayerId, &PlayerRecord)> {
        self.players.iter()
    }

    /// Refreshes cells, positions and health from the host's avatar table
    /// and makes sure every session member has a record.
    pub fn update(&mut self, live: &HostLive) {
        for p in live.members().to_vec() {
            self.join(p, "");
            if let Some(s) = live.avatars.state(p) {
                self.store(p, s);
            }
        }
        let gone: Vec<PlayerId> = self
            .players
            .keys()
            .copied()
            .filter(|p| !live.members().contains(p))
            .collect();
        for p in gone {
            self.leave(p);
        }
    }

    fn store(&mut self, p: PlayerId, s: &AvatarState) {
        let r = self.join(p, "");
        r.cell = s.cell.0;
        r.position = [
            dequant_pos(s.pos[0]),
            dequant_pos(s.pos[1]),
            dequant_pos(s.pos[2]),
        ];
        r.heading = dequant_heading(s.heading);
        r.health = s.health;
        r.downed = s.flags & FLAG_DOWNED != 0;
        r.seen = true;
    }

    /// Records items that reached (positive) or left (negative) a player.
    pub fn credit(&mut self, p: PlayerId, item: FormId, delta: i64) {
        *self.join(p, "").gained.entry(item).or_insert(0) += delta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_core::rules::Mode;

    #[test]
    fn registry_follows_the_session() {
        let mut live = HostLive::new(Mode::CoopBalanced);
        live.join(PlayerId(1), 0);
        let mut reg = PlayerRegistry::new();
        reg.update(&live);
        assert_eq!(reg.iter().count(), 2);
        assert_eq!(reg.puppet(PlayerId(1)), Some(puppet_of(PlayerId(1))));
        assert_eq!(reg.player(puppet_of(PlayerId(1))), Some(PlayerId(1)));
        live.leave(PlayerId(1), 5);
        reg.update(&live);
        assert_eq!(reg.iter().count(), 1);
    }

    #[test]
    fn credits_accumulate() {
        let mut reg = PlayerRegistry::new();
        reg.credit(PlayerId(1), FormId(9), 3);
        reg.credit(PlayerId(1), FormId(9), -1);
        assert_eq!(reg.get(PlayerId(1)).unwrap().gained[&FormId(9)], 2);
    }
}
