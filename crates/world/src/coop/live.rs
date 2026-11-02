//! The live layer on `GameState`: actors, combat, loot, dialogue and rules.
//!
//! No Bevy. The viewer calls, once per frame:
//!
//! * host: [`LiveHostAdapter::frame`] (send the returned messages), plus
//!   [`LiveHostAdapter::on_local`] for messages the host player produces;
//! * guest: [`LiveGuestAdapter::frame`] (send the returned messages to the
//!   host) after the network pump fed `GuestLive`.
//!
//! Single player never constructs an adapter and pays nothing.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::actors::{ActorClient, ActorMsg, ActorState, MAX_HIT};
use coop_core::authority::AuthorityTable;
use coop_core::avatar::{dequant_heading, dequant_pos, quant_heading, quant_pos, AvatarState};
use coop_core::avatar::{FLAG_DEAD, FLAG_DOWNED};
use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use coop_core::sync::{GuestLive, HostLive};
use esm::FormId;

use super::apply::Ctx;
use super::combat_live::CombatBridge;
use super::dialogue::DialogueBridge;
use super::facts::{puppet_of, PendingHit};
use super::interact::{GuestInteract, HostInteract};
use super::loot::{GuestLoot, HostLoot};
use super::players::PlayerRegistry;
use super::rules::RulesBridge;
use super::scene::SceneBridge;
use super::Authority;
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

/// Messages for peers.
pub type Outs = Vec<(PlayerId, Msg)>;

/// Publish rate of owned actors.
const PUBLISH_MS: u64 = 50;

fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

fn wf(f: CForm) -> FormId {
    FormId(f.0)
}

/// The place the live layer files the local player under: the interior cell,
/// or outdoors the worldspace (not the grid square: everyone in the open
/// world is in view of the same actors, and the nearest player owns each).
pub fn space_of_player(gs: &GameState) -> u32 {
    gs.player_world.or(gs.player_cell).map_or(0, |c| c.0)
}

/// The local player's avatar sample from `GameState`.
pub fn avatar_of(gs: &GameState, cx: Ctx) -> AvatarState {
    let p = gs.player_position.unwrap_or([0.0; 3]);
    let health = crate::combat::health(cx.order, gs, PLAYER_REF).unwrap_or(0.0).clamp(0.0, 65535.0);
    let mut flags = gs.coop.live.local_flags & !(FLAG_DOWNED | FLAG_DEAD);
    if gs.coop.live.downed {
        flags |= FLAG_DOWNED;
    }
    if gs.dead.contains(&PLAYER_REF) {
        flags |= FLAG_DEAD;
    }
    AvatarState {
        cell: CellId(space_of_player(gs)),
        pos: [quant_pos(p[0]), quant_pos(p[1]), quant_pos(p[2])],
        heading: quant_heading(gs.player_heading),
        health: health as u16,
        flags,
        anim: gs.coop.live.local_anim,
        weapon: gs.coop.live.local_weapon,
        ..Default::default()
    }
}

/// What the owner publishes about an actor.
pub fn export_actor(gs: &GameState, cx: Ctx, a: FormId) -> ActorState {
    let (cell, pos, heading) = gs.place(cx.order, a).map_or((0, [0.0; 3], 0.0), |p| (p.0 .0, p.2, p.3));
    ActorState {
        cell: CellId(cell),
        pos: [quant_pos(pos[0]), quant_pos(pos[1]), quant_pos(pos[2])],
        heading: quant_heading(heading),
        anim: gs.coop.live.anim.get(&a).copied().unwrap_or(0),
        health: crate::combat::health(cx.order, gs, a).unwrap_or(0.0).clamp(0.0, 65535.0).round() as u16,
        target: gs.combat.get(&a).map_or(0, |t| t.0),
        flags: u16::from(gs.dead.contains(&a)),
    }
}

/// Damage to the ledger's unit: whole points, at least one.
fn hit_points(amount: f64) -> u16 {
    amount.ceil().clamp(1.0, f64::from(MAX_HIT)) as u16
}

/// Sums the hits of one frame per victim (one request each).
fn coalesce(hits: Vec<PendingHit>) -> Vec<(FormId, f64)> {
    let mut by: BTreeMap<FormId, f64> = BTreeMap::new();
    for h in hits {
        *by.entry(h.who).or_insert(0.0) += h.amount;
    }
    by.into_iter().collect()
}

/// Applies a host verdict to this machine's world without hooks.
fn replay(cx: Ctx, gs: &mut GameState, who: FormId, amount: f64, by: FormId) {
    gs.coop.live.replaying = true;
    crate::combat::hurt(cx.order, gs, who, amount, by);
    gs.coop.live.replaying = false;
}

/// Death is final on every peer, exactly once.
fn replay_death(cx: Ctx, gs: &mut GameState, who: FormId) {
    if gs.dead.contains(&who) {
        return;
    }
    let left = crate::combat::health(cx.order, gs, who).unwrap_or(1.0).max(1.0);
    replay(cx, gs, who, left, FormId(0));
    if gs.dead.insert(who) {
        gs.combat.remove(&who);
        gs.events.push(crate::scripting::Event::Died { who, by: FormId(0) });
    }
}

/// Publishes owned actors and applies everyone else's.
#[derive(Default)]
pub struct ActorPump {
    next_ms: u64,
    /// Remote state samples applied.
    pub applied: u64,
}

impl ActorPump {
    /// Owned actors' states at the fixed rate.
    pub fn publish(&mut self, cx: Ctx, gs: &GameState, client: &mut ActorClient, now_ms: u64) -> Vec<ActorMsg> {
        if now_ms < self.next_ms {
            return vec![];
        }
        self.next_ms = now_ms + PUBLISH_MS;
        client.owned().into_iter().filter_map(|a| client.publish(a, export_actor(gs, cx, wf(a)))).collect()
    }

    /// Writes other owners' states into the world, and every actor's damage
    /// and death (the host ledger is the truth for those, for owners too).
    pub fn apply(&mut self, cx: Ctx, gs: &mut GameState, client: &ActorClient, me: PlayerId) {
        for (id, rec) in client.iter() {
            let a = wf(*id);
            if let Some(st) = rec.state {
                // dormant actors (nobody shares their place) keep running here
                if rec.owner.is_some() && rec.owner != Some(me) {
                    let pos = [dequant_pos(st.pos[0]), dequant_pos(st.pos[1]), dequant_pos(st.pos[2])];
                    gs.positions.insert(a, (pos, dequant_heading(st.heading)));
                    gs.coop.live.remote_anim.insert(a, st.anim);
                    match st.target {
                        0 => {
                            gs.combat.remove(&a);
                        }
                        t => {
                            gs.combat.insert(a, FormId(t));
                        }
                    }
                    self.applied += 1;
                }
                if rec.alive && !gs.dead.contains(&a) {
                    if let Some(h) = crate::combat::health(cx.order, gs, a) {
                        if h > f64::from(st.health) + 0.5 {
                            replay(cx, gs, a, h - f64::from(st.health), FormId(0));
                        }
                    }
                }
            }
            if !rec.alive {
                replay_death(cx, gs, a);
            }
        }
    }
}

/// Counters for tests and diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveStats {
    /// Hit requests sent.
    pub hits_sent: u32,
    /// Frames run.
    pub frames: u64,
}

/// Host side of the live layer on `GameState`.
pub struct LiveHostAdapter {
    /// The host player's own views (actors, dialogue, loot, rules).
    pub own: GuestLive,
    /// Players, puppets and per-player records.
    pub registry: PlayerRegistry,
    /// Container bridge.
    pub loot: HostLoot,
    /// Dialogue bridge.
    pub dialogue: DialogueBridge,
    /// Rules bridge.
    pub rules: RulesBridge,
    /// Scenes, movies, control locks and time.
    pub scene: SceneBridge,
    /// Player-world interaction bridge.
    pub interact: HostInteract,
    /// Counters.
    pub stats: LiveStats,
    /// Combat depth (remote players as targets and victims, VATS, blasts).
    pub combat: CombatBridge,
    pump: ActorPump,
    tracked: BTreeSet<FormId>,
    announced: BTreeSet<FormId>,
    next_loot_ms: u64,
}

impl Default for LiveHostAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveHostAdapter {
    /// A fresh adapter for the host player.
    pub fn new() -> Self {
        LiveHostAdapter {
            own: GuestLive::new(PlayerId::HOST),
            registry: PlayerRegistry::new(),
            loot: HostLoot::new(),
            dialogue: DialogueBridge::new(),
            rules: RulesBridge::new(),
            scene: SceneBridge::new(),
            interact: HostInteract::new(),
            stats: LiveStats::default(),
            combat: CombatBridge::new(),
            pump: ActorPump::default(),
            tracked: BTreeSet::new(),
            announced: BTreeSet::new(),
            next_loot_ms: 0,
        }
    }

    /// Starts replicating actors (call for the actors of every loaded cell).
    /// Essential and story actors cannot be killed by players.
    pub fn track(&mut self, cx: Ctx, gs: &GameState, live: &mut HostLive, actors: &[FormId]) {
        for &a in actors {
            // without a place there is nothing to publish: try again later
            if gs.place(cx.order, a).is_none() {
                continue;
            }
            if self.tracked.insert(a) {
                live.actors.register(cf(a), export_actor(gs, cx, a));
                let essential = crate::more_functions::is_essential(cx.order, gs, a);
                live.actors.set_protected(cf(a), essential);
            }
        }
    }

    /// Actors tracked so far (placed ones, made ones, those a guest reported).
    pub fn tracked_count(&self) -> usize {
        self.tracked.len()
    }

    /// Actors a guest reported are protected like the host's own when they
    /// are essential; every tracked actor is known here once.
    fn adopt_reported(&mut self, cx: Ctx, gs: &GameState, live: &mut HostLive) {
        let new: Vec<CForm> = live.actors.own.iter().map(|(id, _)| *id).filter(|id| !self.tracked.contains(&wf(*id))).collect();
        for id in new {
            self.tracked.insert(wf(id));
            let essential = crate::more_functions::is_essential(cx.order, gs, wf(id));
            live.actors.set_protected(id, essential);
        }
    }

    /// The host decides, once, that a person or creature a script made
    /// (`PlaceAtMe`) exists: every peer is told its id and place.
    fn announce_made(&mut self, cx: Ctx, gs: &GameState, live: &mut HostLive) -> Outs {
        let mut outs = Vec::new();
        let fresh: Vec<(FormId, FormId)> = gs
            .more
            .placed
            .refs
            .iter()
            .filter(|(r, _)| !self.announced.contains(*r))
            .filter(|(_, m)| {
                cx.order
                    .get(m.base)
                    .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"NPC_" | b"CREA"))
            })
            .map(|(r, m)| (*r, m.base))
            .collect();
        for (r, base) in fresh {
            if gs.place(cx.order, r).is_none() {
                continue;
            }
            self.announced.insert(r);
            self.tracked.insert(r);
            let st = export_actor(gs, cx, r);
            let essential = crate::more_functions::is_essential(cx.order, gs, r);
            for o in live.actors.announce_made(cf(r), cf(base), cf(r), st) {
                outs.extend(live.route_extern(o.aud, Msg::Actor(o.msg)));
            }
            live.actors.set_protected(cf(r), essential);
        }
        outs
    }

    /// Feeds one message from the host player's own game.
    pub fn on_local(&mut self, live: &mut HostLive, auth: &mut AuthorityTable, now_ms: u64, msg: Msg) -> Outs {
        live.on_msg(PlayerId::HOST, msg, now_ms, auth)
    }

    /// The per-frame driver. Send the returned messages.
    pub fn frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        auth: &mut AuthorityTable,
        now_ms: u64,
    ) -> Outs {
        let mut outs = Vec::new();
        if gs.coop.authority != Authority::Host {
            return outs;
        }
        self.stats.frames += 1;
        super::campaign::host_frame(cx, gs, now_ms);
        self.registry.update(live);
        self.own.local = avatar_of(gs, cx);
        if let Some(m) = self.own.sender.tick(now_ms, &self.own.local) {
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Avatar(m), now_ms, auth));
        }
        for (kind, arg) in std::mem::take(&mut gs.coop.live.local_actions) {
            let m = self.own.action(kind, arg);
            outs.extend(live.on_msg(PlayerId::HOST, m, now_ms, auth));
        }
        gs.coop.live.scaling = Some(live.rules.scaling(now_ms));
        // damage the host dealt itself goes through the one ledger
        let hurts = std::mem::take(&mut gs.coop.live.hurts);
        for (who, amount) in coalesce(hurts) {
            let m = self.own.actors.hit(cf(who), hit_points(amount));
            self.stats.hits_sent += 1;
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Actor(m), now_ms, auth));
        }
        self.adopt_reported(cx, gs, live);
        outs.extend(self.announce_made(cx, gs, live));
        for m in self.pump.publish(cx, gs, &mut self.own.actors, now_ms) {
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Actor(m), now_ms, auth));
        }
        outs.extend(self.dialogue.host_frame(cx, gs, live));
        outs.extend(self.rules.host_frame(cx, gs, live, now_ms));
        outs.extend(self.scene.host_frame(cx, gs, live, now_ms));
        if now_ms >= self.next_loot_ms {
            self.next_loot_ms = now_ms + 100;
            outs.extend(self.loot.reconcile(gs, live, &mut self.registry));
        }
        // what the live layer addressed to the host player
        for m in live.take_local() {
            // who dealt the damage and who killed: paid by slot
            super::campaign::attribution::observe(cx, gs, &m);
            self.own.on_msg(m, now_ms);
        }
        self.pump.apply(cx, gs, &self.own.actors, PlayerId::HOST);
        self.dialogue.poll_lines(gs, &self.own.dialogue);
        for m in self.rules.local_frame(cx, gs, &mut self.own.rules) {
            outs.extend(live.on_msg(PlayerId::HOST, m, now_ms, auth));
        }
        for m in self.scene.local_frame(cx, gs, &mut self.own.scene, true, now_ms) {
            outs.extend(live.on_msg(PlayerId::HOST, m, now_ms, auth));
        }
        outs.extend(self.interact.frame(cx, gs, live, &mut self.own, &self.rules.events, now_ms));
        outs.extend(self.combat.host_frame(cx, gs, live, &mut self.own, auth, &self.rules.events, now_ms));
        outs
    }
}

/// Guest side of the live layer on `GameState`.
pub struct LiveGuestAdapter {
    me: PlayerId,
    /// Container bridge.
    pub loot: GuestLoot,
    /// Dialogue bridge.
    pub dialogue: DialogueBridge,
    /// Rules bridge.
    pub rules: RulesBridge,
    /// Scenes, movies, control locks and time.
    pub scene: SceneBridge,
    /// Player-world interaction bridge.
    pub interact: GuestInteract,
    /// Counters.
    pub stats: LiveStats,
    /// Combat depth.
    pub combat: CombatBridge,
    pump: ActorPump,
    tracked: BTreeSet<FormId>,
    report: Vec<(CForm, ActorState)>,
    realized: BTreeSet<FormId>,
}

impl LiveGuestAdapter {
    /// A fresh adapter for player `me`.
    pub fn new(me: PlayerId) -> Self {
        LiveGuestAdapter {
            me,
            loot: GuestLoot::new(),
            dialogue: DialogueBridge::new(),
            rules: RulesBridge::new(),
            scene: SceneBridge::new(),
            interact: GuestInteract::new(),
            stats: LiveStats::default(),
            combat: CombatBridge::new(),
            pump: ActorPump::default(),
            tracked: BTreeSet::new(),
            report: Vec::new(),
            realized: BTreeSet::new(),
        }
    }

    /// Actors this guest has reported to the host.
    pub fn tracked_count(&self) -> usize {
        self.tracked.len()
    }

    /// Reports the actors loaded here (once each): the host tracks them
    /// too, so a creature found where only this guest has loaded is shared.
    pub fn track(&mut self, cx: Ctx, gs: &GameState, actors: &[FormId]) {
        for &a in actors {
            if puppet_of(self.me) == a || super::facts::player_of(a).is_some() || gs.place(cx.order, a).is_none() {
                continue;
            }
            if self.tracked.insert(a) {
                self.report.push((cf(a), export_actor(gs, cx, a)));
            }
        }
    }

    /// Makes the host's announced refs exist here (the viewer shows them as
    /// it shows any made reference), and drops those the host removed.
    fn realize_spawned(&mut self, gs: &mut GameState, live: &GuestLive) {
        for (id, (cell, pos)) in &live.actors.spawn_at {
            let Some(base) = live.actors.spawned.get(id) else { continue };
            let r = wf(*id);
            if !self.realized.insert(r) {
                continue;
            }
            let at = FormId(cell.0);
            let position = [dequant_pos(pos[0]), dequant_pos(pos[1]), dequant_pos(pos[2])];
            gs.more.placed.refs.entry(r).or_insert(crate::more_functions::placed::Made {
                base: wf(*base),
                space: at,
                cell: at,
                position,
                rotation: [0.0; 3],
                count: 1,
            });
        }
        let gone: Vec<FormId> = self.realized.iter().copied().filter(|r| !live.actors.spawn_at.contains_key(&cf(*r))).collect();
        for r in gone {
            self.realized.remove(&r);
            gs.more.placed.refs.remove(&r);
            gs.disabled.insert(r, true);
        }
    }

    /// The per-frame driver. Send the returned messages to the host.
    pub fn frame(&mut self, cx: Ctx, gs: &mut GameState, live: &mut GuestLive, now_ms: u64) -> Vec<Msg> {
        let mut out = Vec::new();
        if gs.coop.authority != Authority::Guest {
            return out;
        }
        self.stats.frames += 1;
        gs.configure_campaign(cx.order);
        live.local = avatar_of(gs, cx);
        out.extend(live.tick(now_ms));
        for (kind, arg) in std::mem::take(&mut gs.coop.live.local_actions) {
            out.push(live.action(kind, arg));
        }
        while !self.report.is_empty() {
            let n = self.report.len().min(coop_core::actors::MAX_TRACK);
            let batch: Vec<_> = self.report.drain(..n).collect();
            out.extend(live.actors.report(batch).map(Msg::Actor));
        }
        self.realize_spawned(gs, live);
        // damage dealt here is a request; the host resolves it
        let pending = std::mem::take(&mut gs.coop.live.pending_hits);
        for (who, amount) in coalesce(pending) {
            if who == PLAYER_REF || puppet_of(self.me) == who {
                continue;
            }
            self.stats.hits_sent += 1;
            out.push(Msg::Actor(live.actors.hit(cf(who), hit_points(amount))));
        }
        out.extend(self.pump.publish(cx, gs, &mut live.actors, now_ms).into_iter().map(Msg::Actor));
        self.pump.apply(cx, gs, &live.actors, self.me);
        self.dialogue.poll_lines(gs, &live.dialogue);
        self.loot.apply(gs, &live.loot);
        out.extend(self.rules.local_frame(cx, gs, &mut live.rules));
        out.extend(self.scene.local_frame(cx, gs, &mut live.scene, false, now_ms));
        out.extend(self.interact.frame(cx, gs, live));
        out.extend(self.combat.guest_frame(cx, gs, live, self.me, now_ms));
        out
    }
}
