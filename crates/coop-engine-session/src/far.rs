//! Actors the host's game does not run (docs/coop/SPECS.md S21; coop-core's
//! `actors::ownership`): one authority for each of them too.
//!
//! The host's game runs up to 50 actors around its own player. An actor near a guest who is
//! somewhere else was until now run by that guest's game alone, and by every other guest's game
//! there on its own. The rule now:
//!
//! - The host's game is the authority for every actor it runs (`crate::fight`, as before).
//! - An actor the host's game does not run belongs to the guest whose game runs it and told the
//!   host first. A guest reports the actors its game runs in its player's space that nobody
//!   owns (coop-core's `ActorMsg::Track`); the host enters them in the ledger and names the
//!   reporter as their owner, the way the ledger keeps a companion with its recruiter. The
//!   owner's game goes on running the actor and sends its place, heading, health, life and
//!   combat flag (the ledger's state stream); every other guest there holds it at that state,
//!   as it holds the host's actors.
//! - When the host's game comes to run such an actor, the host takes it over (`crate::fight`
//!   names the host as its owner and the ledger hands it over); when the host stops running it,
//!   nobody owns it and the guests report it again.
//!
//! Damage follows the owner. Another player's hit on a guest's actor is judged by the host's
//! ledger as any hit is; the ledger's verdict reaches the owner, whose game applies it. The
//! host's own game does not apply damage to an actor it does not run. A death in the owner's
//! game is in the actor's state, and reaches the story the way any guest's death does
//! (docs/coop/DECISIONS.md D4).

use std::collections::BTreeMap;

use coop_core::actors::{ActorMsg, ActorState, MAX_TRACK};
use coop_core::ids::{FormId, PlayerId};
use coop_core::proto::Msg;
use coop_core::sync::{GuestPhase, GuestSync, HostSync};
use coop_engine::fight::{self, Snap};
use coop_engine::{actors, live};
use engine_core::Engine;

use crate::fight::{Fight, FLAG_DEAD, PUBLISH_EVERY_MS};

/// A guest reports the actors its game runs and nobody owns this often.
pub const TRACK_EVERY_MS: u64 = 1000;

/// One machine's part in the actors the host's game does not run.
#[derive(Debug, Default)]
pub struct Far {
    reported_ms: Option<u64>,
    published_ms: Option<u64>,
    /// Guest: the last state sent of each actor this game owns.
    last: BTreeMap<u32, ActorState>,
    /// Guest: actors reported to the host.
    pub reported: u32,
    /// Guest: states of its own actors sent.
    pub published: u64,
    /// Guest: other players' hits on its own actors, applied by this game.
    pub hurts_applied: u32,
    /// Guest: kills the ledger decided for its own actors, enacted by this game.
    pub kills: u32,
    /// Host: actors entered in the ledger for the guest that reported them.
    pub adopted: u32,
}

impl Far {
    /// The guest's frame: reports what nobody owns, sends the states of what this game owns,
    /// and applies what the ledger decided for those. `avatars` are the other players' avatars
    /// in this game. Returns what to send to the host.
    pub fn guest_step(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        let me = sync.player;
        // The ledger's verdicts on this game's own actors: another player's damage, and a kill.
        let avatar_of = |p: PlayerId| avatars.iter().find(|(q, _)| *q == p).map_or(0, |a| a.1);
        for (actor, by, amount) in std::mem::take(&mut sync.live.actors.damage_log) {
            if by != me
                && sync.live.actors.owns(actor)
                && fight::hurt_actor(e, actor.0, f32::from(amount), avatar_of(by)).is_some()
            {
                self.hurts_applied += 1;
            }
        }
        for id in sync.live.actors.owned() {
            let alive = sync.live.actors.get(id).is_some_and(|a| a.alive);
            if !alive && actors::dead(e, id.0) == Some(false) && actors::kill(e, id.0) {
                self.kills += 1;
            }
        }
        let Some(local) = live::local(e).filter(|l| l.loaded) else {
            return out;
        };
        if sync.phase() != GuestPhase::Active {
            return out;
        }
        let report = !self
            .reported_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < TRACK_EVERY_MS);
        let publish = !self
            .published_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < PUBLISH_EVERY_MS);
        if !report && !publish {
            return out;
        }
        // The actors this game runs in its player's space.
        let mut mine: Vec<Snap> = Vec::new();
        for a in fight::near(e) {
            let s = fight::snap(e, a);
            if actors::shareable(s.id)
                && s.loaded
                && s.space == local.cell
                && !avatars.iter().any(|(_, av)| *av == s.id)
                && actors::actor(e, s.id) == Some(a)
            {
                mine.push(s);
            }
        }
        if report {
            self.reported_ms = Some(now_ms);
            let unowned: Vec<(FormId, ActorState)> = mine
                .iter()
                .filter(|s| {
                    !s.dead()
                        && sync
                            .live
                            .actors
                            .get(FormId(s.id))
                            .map_or(true, |a| a.owner.is_none() && a.alive)
                })
                .take(MAX_TRACK)
                .map(|s| (FormId(s.id), Fight::state_of(s, avatars)))
                .collect();
            self.reported += unowned.len() as u32;
            out.extend(sync.live.actors.report(unowned).map(Msg::Actor));
        }
        if publish {
            self.published_ms = Some(now_ms);
            for s in &mine {
                let id = FormId(s.id);
                if !sync.live.actors.owns(id) {
                    self.last.remove(&s.id);
                    continue;
                }
                let state = Fight::state_of(s, avatars);
                if self.last.get(&s.id) == Some(&state) {
                    continue;
                }
                if let Some(m) = sync.live.actors.publish(id, state) {
                    self.last.insert(s.id, state);
                    self.published += 1;
                    out.push(Msg::Actor(m));
                }
            }
        }
        out
    }

    /// The host's look at a guest's message before coop-core's sync takes it. `runs` says
    /// whether the host's game runs an actor. A reported actor the host's game does not run
    /// and nobody owns is entered for its reporter; the health and the death in an owner's
    /// state are the ledger's from then on (the owner's game is the actor's authority).
    /// Returns the sends.
    pub fn host_pre(
        &mut self,
        sync: &mut HostSync,
        runs: &dyn Fn(u32) -> bool,
        from: PlayerId,
        msg: &Msg,
    ) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        match msg {
            Msg::Actor(ActorMsg::Track { actors: reported }) => {
                let Some(cell) = sync.live.interest.peer_cell(from) else {
                    return out;
                };
                for (id, state) in reported.iter().take(MAX_TRACK) {
                    if !actors::shareable(id.0)
                        || state.cell != cell
                        || runs(id.0)
                        || sync.live.actors.is_dead(*id)
                    {
                        continue;
                    }
                    match sync.live.actors.own.get(*id).map(|r| r.owner) {
                        None => {
                            sync.live.actors.register(*id, *state);
                            self.adopted += 1;
                        }
                        Some(None) => {}
                        Some(Some(_)) => continue,
                    }
                    sync.live.actors.own.set_follower(*id, Some(from));
                }
            }
            Msg::Actor(ActorMsg::State { actor, state, .. }) => {
                let owned = sync
                    .live
                    .actors
                    .own
                    .get(*actor)
                    .is_some_and(|r| r.owner == Some(from));
                if !owned || from.is_host() {
                    return out;
                }
                if let Some(rec) = sync.live.actors.own.get_mut(*actor) {
                    rec.state.health = state.health;
                }
                if state.flags & FLAG_DEAD != 0 && !sync.live.actors.is_dead(*actor) {
                    sync.live.combat.threat.forget(*actor);
                    for o in sync.live.actors.kill_story(*actor, None) {
                        out.extend(sync.live.route_extern(o.aud, Msg::Actor(o.msg)));
                    }
                }
            }
            _ => {}
        }
        out
    }

    /// One status phrase for the log.
    pub fn status(&self, guest: bool) -> String {
        if guest {
            format!(
                "own actors: reported {}, states sent {}, others' hits applied {}, kills {}",
                self.reported, self.published, self.hurts_applied, self.kills
            )
        } else {
            format!("guests' actors entered {}", self.adopted)
        }
    }
}
