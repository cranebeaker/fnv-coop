//! Shared fights (docs/coop/SPECS.md S12; docs/coop/PLAN.md M6): the host's game is the
//! authority for actors that are not players.
//!
//! **State.** About 20 times a second the host reads the actors its game runs around its player
//! (`coop_engine::fight::near`) and tracks each placed one in coop-core's actor ledger
//! (`ActorHost`), pinned to the host as its owner, so the ledger's own rules announce it to the
//! guests in that space and relay its states (place, heading, health, life, combat flag, combat
//! target). A guest holds every such actor that is in its own space the way an avatar is held:
//! AI off, moved to the host's place, health set to the host's, killed through the game when the
//! host says it died. An actor the host no longer runs is given back to the guest's game.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;

use coop_core::actors::{ActorMsg, ActorState};
use coop_core::avatar::{dequant_heading, dequant_pos, quant_heading, quant_pos};
use coop_core::ids::{CellId, FormId, PlayerId, Seq};
use coop_core::interest::Audience;
use coop_core::proto::Msg;
use coop_core::sync::{GuestSync, HostSync};
use coop_engine::fight::{self, Snap};
use coop_engine::{actors, gate, live};
use engine_core::Engine;

/// How much of the fights a run shares (`NV_COOP_FIGHT`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cap {
    /// The host's actors are shown on the guests; nothing else.
    State,
}

impl Cap {
    /// `state`; anything else, or nothing, is everything this build shares.
    pub fn parse(v: &str) -> Cap {
        match v.trim() {
            "state" => Cap::State,
            _ => Cap::State,
        }
    }
}

/// `ActorState::flags`: the actor is in combat.
pub const FLAG_COMBAT: u16 = 1;
/// `ActorState::flags`: the actor is dying or dead.
pub const FLAG_DEAD: u16 = 2;
/// `ActorState::target` for a target that is a player: this plus the player's slot (the id
/// coop-core's vitals use for a player as an attacker).
pub const PLAYER_TARGET_BASE: u32 = 0xfe00_0000;
/// The host reads and publishes its actors this often.
pub const PUBLISH_EVERY_MS: u64 = 50;

/// The player `ActorState::target` names, if it names one.
pub fn target_player(target: u32) -> Option<PlayerId> {
    (target & 0xff00_0000 == PLAYER_TARGET_BASE).then_some(PlayerId(target as u8))
}

#[derive(Debug, Default)]
struct Tracked {
    seq: u32,
    last: Option<ActorState>,
}

/// The fights' state on one machine.
pub struct Fight {
    guest: bool,
    cap: Cap,
    emit: fn(&str),
    /// Every line written, in order.
    pub lines: Vec<String>,
    /// Host: the actors in the ledger.
    tracked: BTreeMap<u32, Tracked>,
    published_ms: Option<u64>,
    /// Guest: the actors held this frame.
    held: BTreeSet<u32>,
    /// Host: states published.
    pub published: u64,
    /// Guest: holds of an actor (one per actor and frame).
    pub moves: u64,
    /// Guest: times the game had switched a held actor's AI back on.
    pub woke: u64,
    /// Guest: actors given back to this game.
    pub released: u32,
    /// Guest: deaths the host announced, enacted here.
    pub deaths: u32,
}

fn print_now(line: &str) {
    println!("{line}");
    let _ = std::io::stdout().flush();
}

fn health16(h: f32) -> u16 {
    h.ceil().clamp(0.0, f32::from(u16::MAX)) as u16
}

impl Fight {
    /// The fights of a guest or a host, up to `cap`. Lines are printed at once and kept in
    /// [`Fight::lines`].
    pub fn new(guest: bool, cap: Cap) -> Fight {
        Fight {
            guest,
            cap,
            emit: print_now,
            lines: Vec::new(),
            tracked: BTreeMap::new(),
            published_ms: None,
            held: BTreeSet::new(),
            published: 0,
            moves: 0,
            woke: 0,
            released: 0,
            deaths: 0,
        }
    }

    /// The same writing nowhere but [`Fight::lines`] (tests).
    pub fn quiet(mut self) -> Fight {
        self.emit = |_| {};
        self
    }

    #[allow(dead_code)]
    fn say(&mut self, line: String) {
        (self.emit)(&line);
        self.lines.push(line);
    }

    /// The cap this run was given.
    pub fn cap(&self) -> Cap {
        self.cap
    }

    /// An actor of the host's game as the state the ledger carries. A target that is a player
    /// (the host's own, or a guest's avatar) is named by the player.
    fn state_of(s: &Snap, avatars: &[(PlayerId, u32)]) -> ActorState {
        let target = if s.target == actors::PLAYER_REF_ID {
            PLAYER_TARGET_BASE
        } else if let Some((p, _)) = avatars
            .iter()
            .find(|(_, a)| *a == s.target && s.target != 0)
        {
            PLAYER_TARGET_BASE + u32::from(p.0)
        } else {
            s.target
        };
        ActorState {
            cell: CellId(s.space),
            pos: s.pos.map(quant_pos),
            heading: quant_heading(s.heading),
            anim: 0,
            health: health16(s.health),
            target,
            flags: if s.in_combat { FLAG_COMBAT } else { 0 } | if s.dead() { FLAG_DEAD } else { 0 },
        }
    }

    /// The host's frame, between two of the game's frames and before coop-core's tick: the
    /// actors around the host's player go into the ledger and their states out. `avatars` are the
    /// other players' avatars in this game (never tracked: they are players). Returns the sends.
    pub fn host_step(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        if self.guest {
            return out;
        }
        if self
            .published_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < PUBLISH_EVERY_MS)
        {
            return out;
        }
        self.published_ms = Some(now_ms);
        let mut seen = BTreeSet::new();
        for a in fight::near(e) {
            let s = fight::snap(e, a);
            if !actors::shareable(s.id)
                || s.space == 0
                || avatars.iter().any(|(_, av)| *av == s.id)
                || actors::actor(e, s.id) != Some(a)
            {
                continue;
            }
            let id = FormId(s.id);
            let state = Self::state_of(&s, avatars);
            if let std::collections::btree_map::Entry::Vacant(slot) = self.tracked.entry(s.id) {
                sync.live.actors.register(id, state);
                // Pinned to the host: its game runs every actor it has.
                sync.live.actors.own.set_follower(id, Some(PlayerId::HOST));
                slot.insert(Tracked::default());
            }
            seen.insert(s.id);
            // The ledger's health is the game's.
            if let Some(rec) = sync.live.actors.own.get_mut(id) {
                rec.state.health = state.health;
            }
            if s.dead() && !sync.live.actors.is_dead(id) {
                for o in sync.live.actors.kill_story(id, None) {
                    out.extend(sync.live.route_extern(o.aud, Msg::Actor(o.msg)));
                }
            }
            let Some(rec) = sync.live.actors.own.get(id).copied() else {
                continue;
            };
            if rec.owner != Some(PlayerId::HOST) {
                continue;
            }
            let t = self.tracked.entry(s.id).or_default();
            if t.last != Some(state) {
                t.seq += 1;
                t.last = Some(state);
                self.published += 1;
                out.extend(sync.live.on_msg(
                    PlayerId::HOST,
                    Msg::Actor(ActorMsg::State {
                        actor: id,
                        epoch: rec.epoch,
                        seq: Seq(t.seq),
                        state,
                    }),
                    now_ms,
                    &mut sync.auth,
                ));
            }
        }
        // An actor the host's game no longer runs near its player: out of the ledger, and the
        // guests told that nobody runs it (their own games take it back).
        let gone: Vec<u32> = self
            .tracked
            .keys()
            .copied()
            .filter(|id| !seen.contains(id))
            .collect();
        for id in gone {
            self.tracked.remove(&id);
            let rec = sync.live.actors.own.get(FormId(id)).copied();
            sync.live.actors.own.unregister(FormId(id), &mut sync.auth);
            if let Some(rec) = rec {
                out.extend(sync.live.route_extern(
                    Audience::Cell(rec.state.cell),
                    Msg::Actor(ActorMsg::Assign {
                        actor: FormId(id),
                        owner: None,
                        epoch: rec.epoch,
                        seed: Some(rec.state),
                    }),
                ));
            }
        }
        out
    }

    /// The guest's frame, between two of the game's frames: every actor the host runs that is in
    /// this player's space is held at the host's state. Returns what to send to the host.
    pub fn guest_step(&mut self, e: &mut Engine, sync: &mut GuestSync, _now_ms: u64) -> Vec<Msg> {
        let out = Vec::new();
        if !self.guest {
            return out;
        }
        let me = live::local(e).filter(|l| l.loaded);
        let mut holding = BTreeSet::new();
        if let Some(me) = me {
            let known: Vec<(u32, coop_core::actors::RemoteActor)> =
                sync.live.actors.iter().map(|(f, a)| (f.0, *a)).collect();
            for (id, a) in known {
                let Some(s) = a.state else {
                    continue;
                };
                if a.owner != Some(PlayerId::HOST) || s.cell.0 != me.cell || !actors::shareable(id)
                {
                    continue;
                }
                if !a.alive || s.flags & FLAG_DEAD != 0 {
                    // The death's scripts ran on the host.
                    if actors::dead(e, id) == Some(false) {
                        gate::applying(e, |e| actors::kill(e, id));
                        self.deaths += 1;
                    }
                    continue;
                }
                if let Some(h) = fight::hold(
                    e,
                    id,
                    s.pos.map(dequant_pos),
                    dequant_heading(s.heading),
                    f32::from(s.health),
                ) {
                    self.moves += 1;
                    self.woke += u64::from(h.woke && self.held.contains(&id));
                    holding.insert(id);
                }
            }
        }
        for id in self.held.difference(&holding) {
            fight::release(e, *id);
            self.released += 1;
        }
        self.held = holding;
        out
    }

    /// The actors this guest holds now.
    pub fn held(&self) -> &BTreeSet<u32> {
        &self.held
    }

    /// Gives every held actor back to this game (the session ends).
    pub fn close(&mut self, e: &mut Engine) {
        for id in std::mem::take(&mut self.held) {
            fight::release(e, id);
            self.released += 1;
        }
    }

    /// One status phrase for the log.
    pub fn status(&self) -> String {
        if self.guest {
            format!(
                "fights: holding {} actors, moves {}, AI woke {}, released {}, deaths {}",
                self.held.len(),
                self.moves,
                self.woke,
                self.released,
                self.deaths
            )
        } else {
            format!(
                "fights: tracking {} actors, states sent {}",
                self.tracked.len(),
                self.published
            )
        }
    }
}
