//! Companions in a party, the session's part (docs/coop/SPECS.md S32).
//!
//! - **Who keeps a companion.** The host's game notes each change of an actor's teammate flag
//!   (`coop_engine::companion::take_hires`). A hire while the host's game runs a guest's line is
//!   that guest's; one in the host player's own conversation, the host's; otherwise the player who
//!   last spoke with the actor, else the host. A companion let go has no keeper. The host writes
//!   [`KEEPER_TAG`] on the actor: its keeper's slot and the wheel's four switches.
//! - **Following.** Each game's copy of a companion another player keeps follows that player's
//!   avatar there (the bridge's follow wraps); in its keeper's game it follows that game's own
//!   player, as the game has it, and is that player's teammate (co-op sets the flag through the
//!   game's own `SetPlayerTeammate`). With hand-off on, a guest's companion is that guest's game's
//!   to run (`fight`'s pin).
//! - **Through doors and fast travel.** The keeper's game brings a following companion that is in
//!   another space than its player, or further than [`BRING_FAR`], to the player with the game's
//!   own `MoveTo` (every [`BRING_EVERY_MS`] at most). One waiting (the wheel's "Waiting") or let
//!   go stays where it is.
//! - **The wheel.** The wheel is the keeper's game's own menu; what it switches (aggressive, keep
//!   distance, ranged, waiting) is read there and proposed to the host ([`SWITCH_TAG`]), which sets
//!   them on its copy and has it evaluate its package, and every game gets them.
//! - **Party saves.** A guest's part lists the companions it keeps; a part given back is claimed of
//!   the host ([`KEEP_TAG`]), which takes it for an actor its game has as a teammate that no other
//!   player in the session keeps.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::{FormId, PlayerId};
use coop_core::schema::Field;
use coop_core::sync::{GuestPhase, GuestSync, HostSync};
use coop_engine::{actors, commands, companion, fight, live};
use engine_core::Engine;

/// Host to every game, on the companion's reference id: the keeper's slot (`NO_KEEPER` for none)
/// and the wheel's four switches (floats).
pub const KEEPER_TAG: u16 = 90;
/// A keeper's claim, on the companion's reference id: the wheel's four switches as its game has
/// them now.
pub const SWITCH_TAG: u16 = 91;
/// A guest's claim, on form 0: the companions it keeps (reference id and four switches each),
/// after a part was given back or a join.
pub const KEEP_TAG: u16 = 92;
/// The slot written for a companion nobody keeps.
pub const NO_KEEPER: u8 = 0xff;
/// A following companion further than this from its keeper's player (game units) is brought.
pub const BRING_FAR: f32 = 4096.0;
/// How often a companion may be brought, and how often a keeper reads its switches (ms).
pub const BRING_EVERY_MS: u64 = 3000;
/// See [`BRING_EVERY_MS`].
pub const SWITCHES_EVERY_MS: u64 = 500;
/// How often a guest's claim of its companions is sent again until the host agrees, and how
/// many times at most.
pub const KEEP_EVERY_MS: u64 = 2000;
/// See [`KEEP_EVERY_MS`].
pub const KEEP_TRIES: u32 = 5;
/// Where a brought companion is put: this far behind its player along the world's y axis.
const BRING_BEHIND: f32 = 64.0;

/// A companion's record as the host writes it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kept {
    /// The keeper's slot, [`NO_KEEPER`] for none.
    pub slot: u8,
    /// The wheel's switches.
    pub switches: [f32; 4],
}

/// The field of a companion's record.
pub fn keeper_field(k: Kept) -> Field {
    let mut data = vec![k.slot];
    for s in k.switches {
        data.extend(s.to_bits().to_le_bytes());
    }
    Field::Custom {
        tag: KEEPER_TAG,
        data,
    }
}

/// A companion's record from its field.
pub fn kept_of(field: &Field) -> Option<Kept> {
    match field {
        Field::Custom { tag, data } if *tag == KEEPER_TAG && data.len() == 17 => Some(Kept {
            slot: data[0],
            switches: switches_at(&data[1..])?,
        }),
        _ => None,
    }
}

fn switches_at(data: &[u8]) -> Option<[f32; 4]> {
    let mut out = [0.0; 4];
    for (i, v) in out.iter_mut().enumerate() {
        *v = f32::from_bits(u32::from_le_bytes(
            data.get(4 * i..4 * i + 4)?.try_into().ok()?,
        ));
    }
    Some(out)
}

fn switch_field(s: [f32; 4]) -> Field {
    Field::Custom {
        tag: SWITCH_TAG,
        data: s.iter().flat_map(|v| v.to_bits().to_le_bytes()).collect(),
    }
}

fn keep_field(list: &[(u32, [f32; 4])]) -> Field {
    let mut data = Vec::new();
    for (id, s) in list {
        data.extend(id.to_le_bytes());
        for v in s {
            data.extend(v.to_bits().to_le_bytes());
        }
    }
    Field::Custom {
        tag: KEEP_TAG,
        data,
    }
}

fn keep_list(data: &[u8]) -> Option<Vec<(u32, [f32; 4])>> {
    if data.len() % 20 != 0 {
        return None;
    }
    data.chunks(20)
        .map(|c| {
            Some((
                u32::from_le_bytes(c[..4].try_into().ok()?),
                switches_at(&c[4..])?,
            ))
        })
        .collect()
}

/// The host's side.
#[derive(Debug, Default)]
pub struct HostCompanions {
    kept: BTreeMap<u32, Kept>,
    written: BTreeMap<u32, Kept>,
    /// The player each actor last spoke with (a conversation's first line).
    talked: BTreeMap<u32, PlayerId>,
    loads_seen: u64,
    /// Hires and lets-go seen.
    pub hires: u32,
    /// Claims taken and refused.
    pub claims_taken: u32,
    /// See [`HostCompanions::claims_taken`].
    pub claims_refused: u32,
    /// What happened, for the log (taken by the caller).
    pub notes: Vec<String>,
}

impl HostCompanions {
    /// Actor `npc` began a conversation with player `p`.
    pub fn on_talk(&mut self, npc: u32, p: PlayerId) {
        self.talked.insert(npc, p);
    }

    /// The keeper of companion `actor`, if any.
    pub fn keeper(&self, actor: u32) -> Option<PlayerId> {
        self.kept
            .get(&actor)
            .filter(|k| k.slot != NO_KEEPER)
            .map(|k| PlayerId(k.slot))
    }

    /// The claim tags the host is to keep for this part.
    pub fn claim_tags() -> [u16; 2] {
        [SWITCH_TAG, KEEP_TAG]
    }

    /// One step, after the guests' messages were taken and the conversations' events handled.
    /// `avatars`: the other players' avatars in this game.
    pub fn step(&mut self, e: &mut Engine, sync: &mut HostSync, avatars: &[(PlayerId, u32)]) {
        for t in Self::claim_tags() {
            sync.claim_tags.insert(t);
        }
        let players = sync.players();
        companion::set_on(e, !players.is_empty());
        // A save loaded: who keeps what is the save's (its teammates), each guest claims its
        // own again from its part.
        let loads = coop_engine::saves::loads(e);
        if loads != self.loads_seen {
            self.loads_seen = loads;
            for k in self.kept.values_mut() {
                k.slot = 0;
            }
            self.talked.clear();
        }
        for h in companion::take_hires(e) {
            self.hires += 1;
            if h.on {
                let slot = match (h.by, h.own_menu) {
                    (Some(s), _) => s,
                    (None, true) => 0,
                    (None, false) => self.talked.get(&h.actor).map_or(0, |p| p.0),
                };
                let switches = companion::switches(e, h.actor).unwrap_or_default();
                self.kept.insert(h.actor, Kept { slot, switches });
                self.notes.push(format!(
                    "co-op companions: {:08x} hired by player {slot}",
                    h.actor
                ));
            } else if let Some(k) = self.kept.get_mut(&h.actor) {
                k.slot = NO_KEEPER;
                self.notes
                    .push(format!("co-op companions: {:08x} let go", h.actor));
            }
        }
        // The keepers' claims.
        let claims = std::mem::take(&mut sync.claims);
        for (from, form, field) in claims {
            match &field {
                Field::Custom { tag, data } if *tag == SWITCH_TAG => {
                    let ok = self.keeper(form.0) == Some(from);
                    match (ok, switches_at(data)) {
                        (true, Some(s)) => {
                            companion::set_switches(e, form.0, s);
                            commands::repick_package(e, form.0);
                            if let Some(k) = self.kept.get_mut(&form.0) {
                                k.switches = s;
                            }
                            self.claims_taken += 1;
                        }
                        _ => self.claims_refused += 1,
                    }
                }
                Field::Custom { tag, data } if *tag == KEEP_TAG => {
                    self.on_keep(e, from, data, &players);
                }
                _ => sync.claims.push((from, form, field)),
            }
        }
        // One the game let go some other way is nobody's.
        for (id, k) in self.kept.iter_mut() {
            if k.slot != NO_KEEPER && companion::teammate(e, *id) == Some(false) {
                k.slot = NO_KEEPER;
            }
        }
        // The follow wraps: a guest's companion here follows that guest's avatar.
        let mut mine = Vec::new();
        for (id, k) in &self.kept {
            let avatar = match k.slot {
                0 => {
                    mine.push(*id);
                    None
                }
                NO_KEEPER => None,
                s => avatars.iter().find(|(p, _)| p.0 == s).map(|(_, a)| *a),
            };
            companion::set_keeper(e, *id, avatar);
            companion::set_kept_by(e, *id, (k.slot != NO_KEEPER).then_some(k.slot));
        }
        companion::set_mine(e, mine);
        for (id, k) in &self.kept {
            if self.written.get(id) != Some(k) {
                sync.write(FormId(*id), keeper_field(*k));
                self.written.insert(*id, *k);
            }
        }
    }

    fn on_keep(&mut self, e: &mut Engine, from: PlayerId, data: &[u8], players: &[PlayerId]) {
        let Some(list) = keep_list(data) else {
            self.claims_refused += 1;
            return;
        };
        let wanted: BTreeSet<u32> = list.iter().map(|(id, _)| *id).collect();
        for (id, s) in list {
            // Another player in the session keeps it: not taken.
            let other = self
                .kept
                .get(&id)
                .is_some_and(|k| k.slot != 0 && k.slot != NO_KEEPER && k.slot != from.0)
                && self
                    .kept
                    .get(&id)
                    .is_some_and(|k| players.contains(&PlayerId(k.slot)));
            if other || companion::teammate(e, id) != Some(true) {
                self.claims_refused += 1;
                continue;
            }
            companion::set_switches(e, id, s);
            self.kept.insert(
                id,
                Kept {
                    slot: from.0,
                    switches: s,
                },
            );
            self.claims_taken += 1;
            self.notes.push(format!(
                "co-op companions: {id:08x} is player {}'s again (saved part or join)",
                from.0
            ));
        }
        // One this player no longer names is the host's.
        for (id, k) in self.kept.iter_mut() {
            if k.slot == from.0 && !wanted.contains(id) {
                k.slot = 0;
            }
        }
    }
}

/// A guest's side.
#[derive(Debug, Default)]
pub struct GuestCompanions {
    mine: BTreeSet<u32>,
    /// Companions whose teammate flag co-op set here.
    made_teammate: BTreeSet<u32>,
    /// The switches last proposed of each.
    proposed: BTreeMap<u32, [f32; 4]>,
    read_ms: Option<u64>,
    brought_ms: BTreeMap<u32, u64>,
    /// Companions to claim of the host, how often claimed and when last.
    claim: Vec<(u32, [f32; 4])>,
    claimed: u32,
    claimed_ms: Option<u64>,
    active: bool,
    /// Companions brought to this player.
    pub brought: u32,
    /// Switch proposals sent.
    pub proposals: u32,
    /// What happened, for the log (taken by the caller).
    pub notes: Vec<String>,
}

impl GuestCompanions {
    /// The companions this player keeps.
    pub fn mine(&self) -> &BTreeSet<u32> {
        &self.mine
    }

    /// One step, after the host's messages were applied. `avatars`: the other players' avatars in
    /// this game. Returns the claims for the host.
    pub fn step(
        &mut self,
        e: &mut Engine,
        sync: &GuestSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) -> Vec<(FormId, Field)> {
        let mut out = Vec::new();
        let active = sync.phase() == GuestPhase::Active;
        companion::set_on(e, active);
        if !active {
            self.active = false;
            return out;
        }
        let me = sync.player;
        let records: Vec<(u32, Kept)> = sync
            .store
            .iter()
            .filter_map(|(k, entry)| Some((k.form.0, kept_of(&entry.field)?)))
            .collect();
        // A join (or a rejoin): the companions this game keeps are claimed of the host again.
        if !self.active {
            self.active = true;
            if !self.mine.is_empty() && self.claim.is_empty() {
                self.claim = self
                    .mine
                    .iter()
                    .filter_map(|id| Some((*id, companion::switches(e, *id)?)))
                    .collect();
                self.claimed = 0;
            }
        }
        if let Some(list) = companion::take_restored(e) {
            self.claim = list;
            self.claimed = 0;
            self.claimed_ms = None;
        }
        for (id, k) in &records {
            let ours = k.slot == me.0;
            if ours && !self.mine.contains(id) {
                self.mine.insert(*id);
                if companion::teammate(e, *id) == Some(false)
                    && companion::set_teammate(e, *id, true)
                {
                    self.made_teammate.insert(*id);
                }
                companion::set_switches(e, *id, k.switches);
                self.proposed.insert(*id, k.switches);
                commands::repick_package(e, *id);
                self.notes.push(format!(
                    "co-op companions: {id:08x} is this player's companion"
                ));
            } else if !ours && self.mine.remove(id) {
                if self.made_teammate.remove(id) {
                    companion::set_teammate(e, *id, false);
                }
                self.proposed.remove(id);
                commands::repick_package(e, *id);
                self.notes.push(format!(
                    "co-op companions: {id:08x} is no longer this player's companion"
                ));
            }
            let avatar = match k.slot {
                s if s == me.0 || s == NO_KEEPER => None,
                s => avatars.iter().find(|(p, _)| p.0 == s).map(|(_, a)| *a),
            };
            companion::set_keeper(e, *id, avatar);
            companion::set_kept_by(e, *id, (k.slot != NO_KEEPER).then_some(k.slot));
        }
        companion::set_mine(e, self.mine.iter().copied());
        // The claim of a part given back, until the host agrees.
        if !self.claim.is_empty() {
            let agreed = self
                .claim
                .iter()
                .all(|(id, _)| records.iter().any(|(r, k)| r == id && k.slot == me.0));
            if agreed || self.claimed >= KEEP_TRIES {
                self.claim.clear();
            } else if self
                .claimed_ms
                .map_or(true, |t| now_ms.saturating_sub(t) >= KEEP_EVERY_MS)
            {
                self.claimed_ms = Some(now_ms);
                self.claimed += 1;
                out.push((FormId(0), keep_field(&self.claim)));
            }
        }
        // The wheel: what this game's player switched goes to the host.
        if self
            .read_ms
            .map_or(true, |t| now_ms.saturating_sub(t) >= SWITCHES_EVERY_MS)
        {
            self.read_ms = Some(now_ms);
            for id in self.mine.clone() {
                let Some(s) = companion::switches(e, id) else {
                    continue;
                };
                if self.proposed.get(&id) != Some(&s) {
                    self.proposed.insert(id, s);
                    self.proposals += 1;
                    out.push((FormId(id), switch_field(s)));
                }
            }
        }
        // Through doors and fast travel: a following companion comes to its player.
        if let Some(me_at) = live::local(e) {
            for id in self.mine.clone() {
                self.bring(e, id, &me_at, now_ms);
            }
        }
        out
    }

    fn bring(&mut self, e: &mut Engine, id: u32, me_at: &live::Local, now_ms: u64) {
        if actors::dead(e, id) != Some(false) || !companion::following(e, id) {
            return;
        }
        let Some(a) = actors::actor(e, id) else {
            return;
        };
        let space = fight::space_of(e, a);
        let pos = fight::place_of_ptr(e, a);
        let far = (0..3)
            .map(|i| (pos[i] - me_at.pos[i]).powi(2))
            .sum::<f32>()
            .sqrt()
            > BRING_FAR;
        if space == me_at.cell && !far {
            return;
        }
        if self
            .brought_ms
            .get(&id)
            .is_some_and(|t| now_ms.saturating_sub(*t) < BRING_EVERY_MS)
        {
            return;
        }
        self.brought_ms.insert(id, now_ms);
        let mut at = me_at.pos;
        at[1] -= BRING_BEHIND;
        if fight::bring(e, id, at) {
            self.brought += 1;
            self.notes.push(format!(
                "co-op companions: {id:08x} brought to this player (another space or far)"
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_and_claims_round_trip() {
        let k = Kept {
            slot: 2,
            switches: [1.0, 0.0, 1.0, 0.0],
        };
        assert_eq!(kept_of(&keeper_field(k)), Some(k));
        assert_eq!(
            kept_of(&Field::Custom {
                tag: KEEPER_TAG,
                data: vec![1, 2]
            }),
            None
        );
        let list = vec![(0x0004_0001, [0.0, 1.0, 0.0, 1.0]), (7, [0.0; 4])];
        let Field::Custom { data, .. } = keep_field(&list) else {
            unreachable!()
        };
        assert_eq!(keep_list(&data), Some(list));
        assert_eq!(keep_list(&[1, 2, 3]), None);
    }
}
