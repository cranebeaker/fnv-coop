//! Shared fights (docs/coop/SPECS.md S13; docs/coop/PLAN.md M6): the host's game is the
//! authority for actors that are not players.
//!
//! **State.** About 20 times a second the host reads the actors its game runs around its player
//! (`coop_engine::fight::near`) and tracks each placed one in coop-core's actor ledger
//! (`ActorHost`), pinned to the host as its owner, so the ledger's own rules announce it to the
//! guests in that space and relay its states (place, heading, health, life, combat flag, combat
//! target). A guest holds every such actor that is in its own space the way an avatar is held:
//! AI off, moved to the host's place, health set to the host's, killed through the game when the
//! host says it died. An actor the host no longer runs is given back to the guest's game.
//!
//! **Damage.** A guest's weapon hit on a held actor never touches the guest's copy: the hit wrap
//! keeps it, and it goes to the host as coop-core's `ActorMsg::Hit`. The ledger judges it (one
//! per id, a rate limit, the same space, not dead, an essential actor never below 1), and what
//! it accepts the host's game applies through its own damage applier, credited to the guest's
//! avatar; a kill the ledger decides is the game's `Actor::Kill`. The result comes back in the
//! actor's next state, and a death also through the story channel. A hit of the host's actors on
//! a guest's avatar is kept from the avatar and becomes coop-core's `PlayerHurt` through the
//! host's vitals; the guest's game applies it to its own player.
//!
//! **Threat.** Twice a second coop-core's `ThreatDirector` picks, for each of the host's actors
//! that is fighting a player, the player it attends to; when that is another player than the
//! game's own choice the host's game is asked to start that combat (the body of `StartCombat`),
//! against the guest's avatar or the host's player. An actor fighting someone who is not a
//! player is left alone. Avatars on the host stop being ghosts so that they can be fought.
//!
//! **Downed and revive.** In a session a player does not die at once: the game's kill of its
//! own player is held, the player is left with 1 health and no controls, and the machine tells
//! the host (`RulesMsg::ReportDown`). coop-core's `Reviver` runs the rest on the host: another
//! player standing next to the downed one (this machine asks for them, there is no key to hold
//! yet) revives them after its hold time at its share of health; nobody coming in its window is
//! a respawn (up again at full health where they lie); everyone down is a wipe, and then every
//! game lets its player die as the game does.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;

use coop_core::actors::{ActorMsg, ActorState};
use coop_core::avatar::{
    dequant_heading, dequant_pos, quant_heading, quant_pos, AvatarState, FLAG_DEAD as AVATAR_DEAD,
    FLAG_DOWNED as AVATAR_DOWNED,
};
use coop_core::combat::{DamageKind, ThreatInput, LIMB_COUNT};
use coop_core::ids::{CellId, FormId, PlayerId, Seq};
use coop_core::interest::Audience;
use coop_core::proto::Msg;
use coop_core::rules::revive::CLIENT_RANGE;
use coop_core::rules::{Notice, RulesMsg};
use coop_core::sync::{GuestLive, GuestSync, HostSync};
use coop_engine::addr::PLAYER_ALL_CONTROLS;
use coop_engine::fight::{self, Role, Snap};
use coop_engine::{actors, gate, live};
use engine_core::Engine;

/// How much of the fights a run shares (`NV_COOP_FIGHT`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cap {
    /// The host's actors are shown on the guests; every game still applies its own hits.
    State,
    /// Also: hits go through the host.
    Damage,
    /// Also: the host's actors can turn on a guest.
    Threat,
    /// Also: a player who would die is down and can be revived.
    Revive,
}

impl Cap {
    /// `state`, `damage` or `threat`; anything else, or nothing, is everything this build shares.
    pub fn parse(v: &str) -> Cap {
        match v.trim() {
            "state" => Cap::State,
            "damage" => Cap::Damage,
            "threat" => Cap::Threat,
            _ => Cap::Revive,
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
/// The host's threat director looks at the fights this often.
pub const THREAT_EVERY_MS: u64 = 500;

/// A helper whose request to revive got no result asks again after this long (longer than the
/// reviver's hold time).
pub const REVIVE_ASK_EVERY_MS: u64 = 4000;
/// After a wipe the player's death is held again this much later (the game was loaded again).
pub const SPARE_AGAIN_MS: u64 = 10_000;

/// The player `ActorState::target` names, if it names one.
pub fn target_player(target: u32) -> Option<PlayerId> {
    (target & 0xff00_0000 == PLAYER_TARGET_BASE).then_some(PlayerId(target as u8))
}

#[derive(Debug, Default)]
struct Tracked {
    seq: u32,
    last: Option<ActorState>,
    /// The actor as the game had it at the last read.
    snap: Option<Snap>,
}

/// The fights' state on one machine.
pub struct Fight {
    guest: bool,
    cap: Cap,
    emit: fn(&str),
    /// Every line written, in order.
    pub lines: Vec<String>,
    started: bool,
    /// The avatars the hit wrap knows.
    avatars: BTreeSet<u32>,
    /// Host: the actors in the ledger.
    tracked: BTreeMap<u32, Tracked>,
    published_ms: Option<u64>,
    threat_ms: Option<u64>,
    /// Guest: the actors held this frame.
    held: BTreeSet<u32>,
    /// Host: states published.
    pub published: u64,
    /// Host: guests' hits its game applied.
    pub hits_applied: u32,
    /// Host: kills the ledger decided, enacted in its game.
    pub kills: u32,
    /// Host: hits of its game on a guest's avatar, sent to that guest as damage.
    pub hurts_sent: u32,
    /// Host: combats its game was asked to start because the threat director chose a player.
    pub retargets: u32,
    /// Guest: holds of an actor (one per actor and frame).
    pub moves: u64,
    /// Guest: times the game had switched a held actor's AI back on.
    pub woke: u64,
    /// Guest: actors given back to this game.
    pub released: u32,
    /// Guest: deaths the host announced, enacted here.
    pub deaths: u32,
    /// Guest: this player's hits sent to the host.
    pub hits_sent: u32,
    /// Guest: damage from the host applied to this game's player.
    pub hurts_taken: u32,
    /// This game's player is down.
    down: bool,
    /// The player's disabled-controls byte from before it went down.
    saved_controls: u8,
    /// The highest health this game's player had in the session (its full health, for a
    /// revive's share).
    max_health: f32,
    /// The downed player this one is reviving, and when it last asked.
    helping: Option<(PlayerId, u64)>,
    wiped_ms: Option<u64>,
    /// Times this game's player went down.
    pub downs: u32,
    /// Times this game's player got up again (revived, or respawned).
    pub ups: u32,
    /// Times everyone was down and this game's player died as the game does.
    pub wipes: u32,
}

fn print_now(line: &str) {
    println!("{line}");
    let _ = std::io::stdout().flush();
}

fn health16(h: f32) -> u16 {
    h.ceil().clamp(0.0, f32::from(u16::MAX)) as u16
}

/// A hit's health damage as the points the wire carries; 0 for nothing.
fn damage16(amount: f32) -> u16 {
    if amount > 0.0 {
        amount.round().clamp(1.0, 5000.0) as u16
    } else {
        0
    }
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
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
            started: false,
            avatars: BTreeSet::new(),
            tracked: BTreeMap::new(),
            published_ms: None,
            threat_ms: None,
            held: BTreeSet::new(),
            published: 0,
            hits_applied: 0,
            kills: 0,
            hurts_sent: 0,
            retargets: 0,
            moves: 0,
            woke: 0,
            released: 0,
            deaths: 0,
            hits_sent: 0,
            hurts_taken: 0,
            down: false,
            saved_controls: 0,
            max_health: 0.0,
            helping: None,
            wiped_ms: None,
            downs: 0,
            ups: 0,
            wipes: 0,
        }
    }

    /// The same writing nowhere but [`Fight::lines`] (tests).
    pub fn quiet(mut self) -> Fight {
        self.emit = |_| {};
        self
    }

    fn say(&mut self, line: String) {
        (self.emit)(&line);
        self.lines.push(line);
    }

    /// The cap this run was given.
    pub fn cap(&self) -> Cap {
        self.cap
    }

    /// The first frame: the hit wrap learns what this machine is.
    fn start(&mut self, e: &mut Engine) {
        if self.started {
            return;
        }
        self.started = true;
        if self.cap < Cap::Damage {
            return;
        }
        if self.cap >= Cap::Revive {
            fight::set_spare(e, true);
        }
        if fight::hits_wrapped(e) {
            fight::set_role(e, if self.guest { Role::Guest } else { Role::Host });
        } else {
            self.say(
                "co-op fights: this game has no Actor::HitMe to wrap; hits are not shared".into(),
            );
        }
    }

    /// Tells the hit wrap which references are players' avatars, when that changed. On a host
    /// that lets its actors fight guests, a new avatar stops being a ghost.
    fn know_avatars(&mut self, e: &mut Engine, avatars: &[(PlayerId, u32)]) {
        let ids: BTreeSet<u32> = avatars.iter().map(|a| a.1).collect();
        if ids == self.avatars {
            return;
        }
        if !self.guest && self.cap >= Cap::Threat {
            for id in ids.difference(&self.avatars) {
                fight::set_ghost(e, *id, false);
            }
        }
        fight::set_avatars(e, ids.iter().copied());
        self.avatars = ids;
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

    /// The host's frame, between two of the game's frames and before coop-core's tick: guests'
    /// hits the ledger accepted are applied, hits on avatars become the guests' damage, the
    /// actors around the host's player go into the ledger and their states out, and the threat
    /// director has its say. `avatars` are the other players' avatars in this game (never
    /// tracked: they are players). Returns the sends.
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
        self.start(e);
        self.know_avatars(e, avatars);
        if self.cap >= Cap::Damage {
            self.host_damage(e, sync, avatars, now_ms, &mut out);
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
            // The ledger's health is the game's, and the game says who cannot die.
            sync.live.actors.set_protected(id, s.essential);
            if let Some(rec) = sync.live.actors.own.get_mut(id) {
                rec.state.health = state.health;
            }
            if s.dead() && !sync.live.actors.is_dead(id) {
                sync.live.combat.threat.forget(id);
                for o in sync.live.actors.kill_story(id, None) {
                    out.extend(sync.live.route_extern(o.aud, Msg::Actor(o.msg)));
                }
            }
            let t = self.tracked.entry(s.id).or_default();
            t.snap = Some(s);
            let Some(rec) = sync.live.actors.own.get(id).copied() else {
                continue;
            };
            if rec.owner != Some(PlayerId::HOST) {
                continue;
            }
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
            sync.live.combat.threat.forget(FormId(id));
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
        if self.cap >= Cap::Threat {
            self.host_threat(e, sync, avatars, now_ms);
        }
        out
    }

    /// The host's part of the damage: what the ledger accepted of the guests' hits goes into
    /// the game, and the game's hits on avatars go to their players through the vitals.
    fn host_damage(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
        out: &mut Vec<(PlayerId, Msg)>,
    ) {
        let avatar_of = |p: PlayerId| avatars.iter().find(|(q, _)| *q == p).map_or(0, |a| a.1);
        for at in std::mem::take(&mut sync.live.combat.attributions) {
            let by = avatar_of(at.by);
            if at.amount > 0 && fight::hurt_actor(e, at.actor.0, f32::from(at.amount), by).is_some()
            {
                self.hits_applied += 1;
                sync.live
                    .combat
                    .threat
                    .damaged(at.actor, at.by, f32::from(at.amount));
            }
            if at.killed && fight::kill_by(e, at.actor.0, by) {
                self.kills += 1;
            }
        }
        // The host knows each guest's health from the guest's own avatar stream.
        for p in sync.players() {
            if let Some(s) = sync.live.avatars.state(p).copied() {
                let vitals = &mut sync.live.combat.vitals;
                vitals.track(p, s.health.max(1), s.health.max(1));
                vitals.report(
                    p,
                    now_ms,
                    s.health,
                    s.flags & AVATAR_DOWNED != 0,
                    s.flags & AVATAR_DEAD != 0,
                );
            }
        }
        for h in fight::take_hits(e) {
            let Some((victim, _)) = avatars.iter().find(|(_, a)| *a == h.target) else {
                continue;
            };
            let amount = damage16(h.amount);
            if amount == 0 {
                continue;
            }
            // The kind and the limb are not read from the game yet: a direct hit, no limb.
            let msgs = if h.aggressor == actors::PLAYER_REF_ID {
                sync.live.combat.vitals.player_vs_player(
                    now_ms,
                    PlayerId::HOST,
                    *victim,
                    amount,
                    DamageKind::Bullet,
                    LIMB_COUNT,
                )
            } else {
                let scaling = sync.live.rules.scaling(now_ms);
                sync.live.combat.vitals.hurt(
                    now_ms,
                    *victim,
                    FormId(h.aggressor),
                    amount,
                    DamageKind::Bullet,
                    LIMB_COUNT,
                    &scaling,
                )
            };
            self.hurts_sent += msgs.len() as u32;
            out.extend(sync.live.route_combat(msgs));
        }
    }

    /// The threat director's turn: for each actor of the host's game that is fighting a player,
    /// which player it attends to; the game is asked to fight that one when it is another.
    fn host_threat(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) {
        if self
            .threat_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < THREAT_EVERY_MS)
        {
            return;
        }
        self.threat_ms = Some(now_ms);
        let Some(me) = live::local(e) else {
            return;
        };
        let focus = sync.live.rules.scaling(now_ms).focus;
        let fighting: Vec<Snap> = self
            .tracked
            .values()
            .filter_map(|t| t.snap)
            .filter(|s| s.in_combat && !s.dead())
            .collect();
        for s in fighting {
            // Only an actor whose own choice is a player is redirected.
            let at_player = s.target == actors::PLAYER_REF_ID
                || avatars.iter().any(|(_, a)| *a == s.target && s.target != 0);
            if !at_player {
                continue;
            }
            let mut inputs = vec![ThreatInput {
                player: PlayerId::HOST,
                dist: dist(me.pos, s.pos),
                seen: true,
                downed: self.down,
                dead: false,
                same_cell: me.cell == s.space,
            }];
            for (p, _) in avatars {
                let Some(a) = sync.live.avatars.state(*p) else {
                    continue;
                };
                inputs.push(ThreatInput {
                    player: *p,
                    dist: dist(a.pos.map(dequant_pos), s.pos),
                    seen: true,
                    downed: a.flags & AVATAR_DOWNED != 0,
                    dead: a.flags & AVATAR_DEAD != 0,
                    same_cell: a.cell.0 == s.space,
                });
            }
            let Some(choice) = sync
                .live
                .combat
                .threat
                .choose(FormId(s.id), now_ms, focus, &inputs)
            else {
                continue;
            };
            let want = if choice.target.is_host() {
                actors::PLAYER_REF_ID
            } else {
                match avatars.iter().find(|(p, _)| *p == choice.target) {
                    Some((_, a)) => *a,
                    None => continue,
                }
            };
            if want != s.target && fight::start_combat(e, s.id, want) {
                self.retargets += 1;
            }
        }
    }

    /// The guest's frame, between two of the game's frames: every actor the host runs that is in
    /// this player's space is held at the host's state, this player's hits go to the host, and
    /// the damage the host decided is taken. `avatars` are the other players' avatars in this
    /// game. Returns what to send to the host.
    pub fn guest_step(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        if !self.guest {
            return out;
        }
        self.start(e);
        self.know_avatars(e, avatars);
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
        if holding != self.held {
            fight::set_held(e, holding.iter().copied());
            self.held = holding;
        }
        if self.cap >= Cap::Damage {
            self.guest_damage(e, sync, avatars, &mut out);
        }
        if self.cap >= Cap::Revive {
            let me = sync.player;
            for m in self.body(e, &mut sync.live, me, now_ms) {
                out.push(Msg::Rules(m));
            }
        }
        self.stamp(e, &mut sync.live.local);
        out
    }

    /// The host's own player: what the other players' hits and the reviver decided for it.
    /// `view` is the host player's own view of the session (what the host sends to itself).
    /// Returns the sends.
    pub fn host_body(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        view: &mut GuestLive,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        if self.guest || self.cap < Cap::Damage {
            return out;
        }
        self.take_hurts(e, view);
        if self.cap >= Cap::Revive {
            for m in self.body(e, view, PlayerId::HOST, now_ms) {
                out.extend(sync.on_local(Msg::Rules(m)));
            }
        }
        out
    }

    /// Writes this game's player's health and fall into the state the other players get.
    pub fn stamp(&self, e: &mut Engine, state: &mut AvatarState) {
        if self.cap < Cap::Damage {
            return;
        }
        if let Some(h) = fight::player_health(e) {
            state.health = health16(h);
        }
        if self.down {
            state.flags |= AVATAR_DOWNED;
        }
    }

    /// The damage the host decided for this game's player is taken by it.
    fn take_hurts(&mut self, e: &mut Engine, glive: &mut GuestLive) {
        for n in std::mem::take(&mut glive.combat.hurts) {
            if let Some(left) = fight::hurt_player(e, f32::from(n.amount), n.attacker.0) {
                self.hurts_taken += 1;
                // Without revives a player at no health dies as the game has it.
                if left <= 0.0 && self.cap < Cap::Revive {
                    fight::kill_player(e);
                }
            }
        }
    }

    /// Whether player `p` stands next to this game's player.
    fn next_to(e: &mut Engine, glive: &mut GuestLive, p: PlayerId, now_ms: u64) -> bool {
        let (Some(me), Some(s)) = (live::local(e).filter(|l| l.loaded), glive.sample(p, now_ms))
        else {
            return false;
        };
        me.cell == s.cell.0 && dist(me.pos, s.pos) <= CLIENT_RANGE
    }

    /// This game's player's fall and rise, and its help for another player who is down.
    /// `glive` is this player's view of the session. Returns what to tell the host's rules.
    fn body(
        &mut self,
        e: &mut Engine,
        glive: &mut GuestLive,
        me: PlayerId,
        now_ms: u64,
    ) -> Vec<RulesMsg> {
        let mut out = Vec::new();
        let Some(health) = fight::player_health(e) else {
            return out;
        };
        self.max_health = self.max_health.max(health);
        if self
            .wiped_ms
            .is_some_and(|t| now_ms.saturating_sub(t) >= SPARE_AGAIN_MS)
        {
            self.wiped_ms = None;
            fight::set_spare(e, true);
        }
        let fell = fight::take_falls(e) > 0 || health <= 0.0;
        if fell && !self.down && self.wiped_ms.is_none() {
            self.down = true;
            self.downs += 1;
            self.saved_controls = fight::controls(e).unwrap_or(0);
            self.say(
                "co-op fights: this player is down; another player standing next to them revives them"
                    .into(),
            );
            out.push(RulesMsg::ReportDown { revivable: true });
        }
        // The reviver's word for this player; other notices are not the fights' to take.
        let (mut up, mut wipe) = (None, false);
        glive.rules.notices.retain(|n| match n {
            Notice::Revived(pct) => {
                up = Some(f32::from(*pct));
                false
            }
            Notice::Respawn => {
                up = Some(100.0);
                false
            }
            Notice::Wipe => {
                wipe = true;
                false
            }
            _ => true,
        });
        if self.down {
            self.helping = None;
            if wipe {
                self.down = false;
                self.wipes += 1;
                self.wiped_ms = Some(now_ms);
                fight::set_controls(e, self.saved_controls);
                fight::set_spare(e, false);
                self.say("co-op fights: everyone is down; this player dies".into());
                fight::kill_player(e);
            } else if let Some(pct) = up {
                self.down = false;
                self.ups += 1;
                fight::set_controls(e, self.saved_controls);
                fight::set_player_health(e, (self.max_health * pct / 100.0).max(1.0));
                self.say(format!(
                    "co-op fights: this player is up again at {pct:.0} percent health"
                ));
            } else {
                if health < 1.0 {
                    fight::set_player_health(e, 1.0);
                }
                fight::set_controls(e, self.saved_controls | PLAYER_ALL_CONTROLS);
            }
            return out;
        }
        // Up: stand next to a downed player and the host is asked to let this one revive them.
        let downed: Vec<PlayerId> = glive
            .rules
            .downed
            .iter()
            .copied()
            .filter(|p| *p != me)
            .collect();
        match self.helping {
            Some((t, asked)) => {
                if !downed.contains(&t) {
                    self.helping = None;
                } else if !Self::next_to(e, glive, t, now_ms) {
                    out.push(RulesMsg::ReviveCancel);
                    self.helping = None;
                } else if now_ms.saturating_sub(asked) >= REVIVE_ASK_EVERY_MS {
                    out.push(RulesMsg::ReviveBegin { target: t });
                    self.helping = Some((t, now_ms));
                }
            }
            None => {
                for t in downed {
                    if Self::next_to(e, glive, t, now_ms) {
                        out.push(RulesMsg::ReviveBegin { target: t });
                        self.helping = Some((t, now_ms));
                        break;
                    }
                }
            }
        }
        out
    }

    /// Whether this game's player is down.
    pub fn down(&self) -> bool {
        self.down
    }

    /// The guest's part of the damage: this player's kept hits go to the host, and what the host
    /// decided for this player is taken by this game's player.
    fn guest_damage(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        avatars: &[(PlayerId, u32)],
        out: &mut Vec<Msg>,
    ) {
        for h in fight::take_hits(e) {
            let amount = damage16(h.amount);
            if amount == 0 {
                continue;
            }
            if let Some((victim, _)) = avatars.iter().find(|(_, a)| *a == h.target) {
                // Another player: the host's friendly-fire rule decides.
                if h.aggressor == actors::PLAYER_REF_ID {
                    out.push(Msg::Combat(sync.live.combat.player_hit(
                        *victim,
                        amount,
                        DamageKind::Bullet,
                        LIMB_COUNT,
                    )));
                }
            } else {
                out.push(Msg::Actor(sync.live.actors.hit(FormId(h.target), amount)));
                self.hits_sent += 1;
            }
        }
        self.take_hurts(e, &mut sync.live);
    }

    /// The actors this guest holds now.
    pub fn held(&self) -> &BTreeSet<u32> {
        &self.held
    }

    /// Gives every held actor back to this game and takes this machine out of the hit wrap (the
    /// session ends).
    pub fn close(&mut self, e: &mut Engine) {
        for id in std::mem::take(&mut self.held) {
            fight::release(e, id);
            self.released += 1;
        }
        fight::set_held(e, []);
        fight::set_avatars(e, []);
        fight::set_role(e, Role::Off);
        fight::set_spare(e, false);
        if self.down {
            self.down = false;
            fight::set_controls(e, self.saved_controls);
        }
        self.avatars.clear();
        self.started = false;
    }

    /// One status phrase for the log.
    pub fn status(&self) -> String {
        if self.guest {
            format!(
                "fights: holding {} actors, moves {}, AI woke {}, released {}, deaths {}, hits sent {}, hurts taken {}, down {} up {}",
                self.held.len(),
                self.moves,
                self.woke,
                self.released,
                self.deaths,
                self.hits_sent,
                self.hurts_taken,
                self.downs,
                self.ups
            )
        } else {
            format!(
                "fights: tracking {} actors, states sent {}, guests' hits applied {}, kills {}, hurts sent {}, retargets {}, down {} up {}",
                self.tracked.len(),
                self.published,
                self.hits_applied,
                self.kills,
                self.hurts_sent,
                self.retargets,
                self.downs,
                self.ups
            )
        }
    }
}
