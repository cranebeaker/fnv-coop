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
    let health = crate::combat::health(cx.order, gs, PLAYER_REF)
        .unwrap_or(0.0)
        .clamp(0.0, 65535.0);
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
        seat: gs.furniture.get(&PLAYER_REF).map_or(0, |f| f.0),
        ..Default::default()
    }
}

/// What the owner publishes about an actor.
pub fn export_actor(gs: &GameState, cx: Ctx, a: FormId) -> ActorState {
    let (cell, pos, heading) = gs
        .place(cx.order, a)
        .map_or((0, [0.0; 3], 0.0), |p| (p.0 .0, p.2, p.3));
    ActorState {
        cell: CellId(cell),
        pos: [quant_pos(pos[0]), quant_pos(pos[1]), quant_pos(pos[2])],
        heading: quant_heading(heading),
        anim: gs.coop.live.anim.get(&a).copied().unwrap_or(0),
        health: crate::combat::health(cx.order, gs, a)
            .unwrap_or(0.0)
            .clamp(0.0, 65535.0)
            .round() as u16,
        target: gs.combat.get(&a).map_or(0, |t| t.0),
        flags: u16::from(gs.dead.contains(&a)),
    }
}

/// An actor's combat target as it goes on the wire. `PLAYER_REF` means a
/// different player on each machine, so the owner sends its own player as
/// the puppet form of `me`.
fn target_out(t: u32, me: PlayerId) -> u32 {
    if FormId(t) == PLAYER_REF {
        puppet_of(me).0
    } else {
        t
    }
}

/// A combat target from the wire: this machine's own puppet form is the
/// local player.
fn target_in(t: u32, me: PlayerId) -> FormId {
    if FormId(t) == puppet_of(me) {
        PLAYER_REF
    } else {
        FormId(t)
    }
}

/// Damage to the ledger's unit: whole points, at least one.
fn hit_points(amount: f64) -> u16 {
    amount.ceil().clamp(1.0, f64::from(MAX_HIT)) as u16
}

/// The part of a point a guest's blows have dealt that no request has
/// carried yet, per victim. Rounding every frame's blow up to a whole point
/// (the ledger counts whole points) made a fast weapon or a fist do twice
/// its damage on some frames and the same on others.
#[derive(Debug, Default)]
struct HitCarry(BTreeMap<FormId, f64>);

impl HitCarry {
    /// Adds `amount` for `who`; the whole points to send now (`None`: less
    /// than one point so far). `left` is what the victim has left here: a
    /// blow that would finish it always counts as at least a point.
    fn take(&mut self, who: FormId, amount: f64, left: Option<f64>) -> Option<u16> {
        let total = self.0.get(&who).copied().unwrap_or(0.0) + amount;
        let mut pts = total.floor();
        if pts < 1.0 && left.is_some_and(|l| total + 0.5 >= l) {
            pts = 1.0;
        }
        if pts < 1.0 {
            self.0.insert(who, total);
            return None;
        }
        let pts = pts.min(f64::from(MAX_HIT));
        let rest = (total - pts).max(0.0);
        if rest > 0.0 {
            self.0.insert(who, rest);
        } else {
            self.0.remove(&who);
        }
        Some(pts as u16)
    }

    /// Forgets the part point kept for `who` (the host dropped a blow on it).
    fn forget(&mut self, who: FormId) {
        self.0.remove(&who);
    }
}

/// Sums the hits of one frame per victim (one request each).
fn coalesce(hits: Vec<PendingHit>) -> Vec<(FormId, f64)> {
    let mut by: BTreeMap<FormId, f64> = BTreeMap::new();
    for h in hits {
        *by.entry(h.who).or_insert(0.0) += h.amount;
    }
    by.into_iter().collect()
}

/// True when a guest sends a hit from its world to the host. The guest
/// sends the blows of its player and its followers, and the blows on or by
/// an actor that it owns. The host's world runs all other NPC fights. A
/// guest copy of such a fight (Doc Mitchell and a gecko) is not sent: the
/// host would book it as this player's hit, with an assault crime on Doc.
fn guest_sends(gs: &GameState, actors: &ActorClient, h: &PendingHit) -> bool {
    h.by == PLAYER_REF
        || gs.teammates.contains(&h.by)
        || actors.owns(cf(h.by))
        || actors.owns(cf(h.who))
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
    let left = crate::combat::health(cx.order, gs, who)
        .unwrap_or(1.0)
        .max(1.0);
    replay(cx, gs, who, left, FormId(0));
    if gs.dead.insert(who) {
        gs.combat.remove(&who);
        gs.events
            .push(crate::scripting::Event::Died { who, by: FormId(0) });
    }
}

/// Turns a place the live layer sends (an interior cell, or outdoors the
/// worldspace: [`space_of_player`]) back into a `GameState` place: (space,
/// cell), the cell being the worldspace's grid cell at the position.
/// Worldspace grids are read once and kept.
#[derive(Default)]
pub struct PlaceResolver {
    grids: BTreeMap<FormId, Option<crate::WorldGrid>>,
}

impl PlaceResolver {
    /// (space, cell) for `space` at `pos`; `None` for a form that is not a
    /// place or a position outside the worldspace's cells.
    pub fn resolve(
        &mut self,
        order: &esm::LoadOrder,
        space: FormId,
        pos: [f32; 3],
    ) -> Option<(FormId, FormId)> {
        if space.0 == 0 {
            return None;
        }
        // an id the load order does not know (a test's made-up cell) is a
        // place of its own, as the live layer files it
        let Some(rr) = order.get(space) else {
            return Some((space, space));
        };
        let kind = rr.entry.header.kind;
        if kind.as_bytes() == b"WRLD" {
            let grid = self
                .grids
                .entry(space)
                .or_insert_with(|| crate::WorldGrid::load(order, space).ok());
            let cell = grid.as_ref()?.cell_at(crate::square_of(pos))?;
            return Some((space, cell));
        }
        if kind.as_bytes() != b"CELL" {
            return None;
        }
        // an exterior grid cell sent as itself: its worldspace is the space
        match order.world_of(&rr) {
            Some(w) => Some((w, space)),
            None => Some((space, space)),
        }
    }

    /// The grid of a worldspace (read once), `None` when it cannot be read.
    pub fn grid(&mut self, order: &esm::LoadOrder, world: FormId) -> Option<&crate::WorldGrid> {
        self.grids
            .entry(world)
            .or_insert_with(|| crate::WorldGrid::load(order, world).ok())
            .as_ref()
    }
}

/// Publishes owned actors and applies everyone else's.
#[derive(Default)]
pub struct ActorPump {
    next_ms: u64,
    /// Remote state samples applied.
    pub applied: u64,
    places: PlaceResolver,
    /// The epoch at which this machine took each actor it owns.
    taken: BTreeMap<coop_core::ids::FormId, coop_core::ids::Epoch>,
}

impl ActorPump {
    /// Guest: an actor this machine now owns, but has in another place than
    /// the host's ledger (the `Assign` seed), moves to the ledger's place. A
    /// host script that moved it to another cell (`MoveTo`) did not move it
    /// here, and the owner must not publish the old place. The host does not
    /// call this: its world already has every place.
    pub fn take_over(&mut self, cx: Ctx, gs: &mut GameState, client: &ActorClient, me: PlayerId) {
        self.taken.retain(|id, _| client.owns(*id));
        for (id, rec) in client.iter() {
            if rec.owner != Some(me) || self.taken.get(id) == Some(&rec.epoch) {
                continue;
            }
            self.taken.insert(*id, rec.epoch);
            let Some(st) = rec.state.filter(|s| s.cell.0 != 0) else {
                continue;
            };
            let a = wf(*id);
            let pos = [
                dequant_pos(st.pos[0]),
                dequant_pos(st.pos[1]),
                dequant_pos(st.pos[2]),
            ];
            // in the same place the previous owner's stream already moved it
            let Some(to) = self.place_of(cx.order, gs, a, FormId(st.cell.0), pos) else {
                continue;
            };
            gs.spaces.insert(a, to);
            gs.positions.insert(a, (pos, dequant_heading(st.heading)));
        }
    }

    /// The (space, cell) a published place stands for, when it is not where
    /// this machine has the actor now.
    fn place_of(
        &mut self,
        order: &esm::LoadOrder,
        gs: &GameState,
        a: FormId,
        space: FormId,
        pos: [f32; 3],
    ) -> Option<(FormId, FormId)> {
        let to = self.places.resolve(order, space, pos)?;
        let now = gs.place(order, a).map(|p| (p.0, p.1));
        (now != Some(to)).then_some(to)
    }

    /// Owned actors' states at the fixed rate. `me` is this machine's player:
    /// a combat target on the local player goes out as `me`'s puppet form.
    pub fn publish(
        &mut self,
        cx: Ctx,
        gs: &GameState,
        client: &mut ActorClient,
        me: PlayerId,
        now_ms: u64,
    ) -> Vec<ActorMsg> {
        if now_ms < self.next_ms {
            return vec![];
        }
        self.next_ms = now_ms + PUBLISH_MS;
        client
            .owned()
            .into_iter()
            .filter_map(|a| {
                let mut st = export_actor(gs, cx, wf(a));
                st.target = target_out(st.target, me);
                client.publish(a, st)
            })
            .collect()
    }

    /// Writes other owners' states into the world, and every actor's damage
    /// and death (the host ledger is the truth for those, for owners too).
    pub fn apply(&mut self, cx: Ctx, gs: &mut GameState, client: &ActorClient, me: PlayerId) {
        for (id, rec) in client.iter() {
            let a = wf(*id);
            if let Some(st) = rec.state {
                // dormant actors (nobody shares their place) keep running here
                if rec.owner.is_some() && rec.owner != Some(me) {
                    let pos = [
                        dequant_pos(st.pos[0]),
                        dequant_pos(st.pos[1]),
                        dequant_pos(st.pos[2]),
                    ];
                    // the owner's place for it: another cell moves it here too
                    if let Some(sc) = self.place_of(cx.order, gs, a, FormId(st.cell.0), pos) {
                        gs.spaces.insert(a, sc);
                    }
                    gs.positions.insert(a, (pos, dequant_heading(st.heading)));
                    gs.coop.live.remote_anim.insert(a, st.anim);
                    match st.target {
                        0 => {
                            gs.combat.remove(&a);
                        }
                        t => {
                            gs.combat.insert(a, target_in(t, me));
                        }
                    }
                    self.applied += 1;
                }
                if rec.alive && !gs.dead.contains(&a) {
                    if let Some(h) = crate::combat::health(cx.order, gs, a) {
                        if h > f64::from(st.health) + 0.5 {
                            replay(cx, gs, a, h - f64::from(st.health), FormId(0));
                        } else if !me.is_host() && h + 0.5 < f64::from(st.health) {
                            // the host raised it (`ActorMsg::Healed`); the host's
                            // own world raised it first
                            let raise = f64::from(st.health) - h;
                            crate::magic::change(
                                cx.order,
                                gs,
                                a,
                                crate::combat::av::HEALTH,
                                raise,
                                a,
                            );
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
    /// Guest hit requests that wait for the friendly-fire guard.
    held_hits: Vec<(PlayerId, Msg)>,
    /// Each actor's world health after the last frame's verdicts.
    last_health: BTreeMap<FormId, f64>,
    /// Raised health the ledger has not taken yet (less than a point).
    heal_carry: BTreeMap<FormId, f64>,
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
            held_hits: Vec::new(),
            last_health: BTreeMap::new(),
            heal_carry: BTreeMap::new(),
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
                // the baseline for `raises`: a raise before the next frame still counts
                if let Some(h) = crate::combat::health(cx.order, gs, a) {
                    self.last_health.insert(a, h);
                }
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
        let new: Vec<CForm> = live
            .actors
            .own
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| !self.tracked.contains(&wf(*id)))
            .collect();
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
    pub fn on_local(
        &mut self,
        live: &mut HostLive,
        auth: &mut AuthorityTable,
        now_ms: u64,
        msg: Msg,
    ) -> Outs {
        live.on_msg(PlayerId::HOST, msg, now_ms, auth)
    }

    /// Takes a guest's hit request off the network before the ledger sees
    /// it: the next [`frame`](Self::frame) runs the friendly-fire guard on
    /// it first. Every other message comes back to the caller unchanged.
    pub fn hold_guest_hit(&mut self, from: PlayerId, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Actor(ActorMsg::Hit { .. }) if !from.is_host() => {
                self.held_hits.push((from, msg));
                None
            }
            // a guest's dialogue pick waits for the line's conditions check
            m => self.dialogue.hold_pick(from, m),
        }
    }

    /// The guest hit requests that pass the friendly-fire guard (G40). A
    /// guest's machine has no policy gate, and the host applies a guest's
    /// hit as a verdict without hooks, so the guard runs here, by slot.
    /// Health the host's world gained since the last frame goes to the
    /// ledger (a script's `ForceActorValue Health`, a heal). Without this the
    /// next verdict takes the raise back. A drop came with this frame's
    /// hurts (`dealt`), and the ledger has those already.
    fn raises(
        &mut self,
        cx: Ctx,
        gs: &GameState,
        live: &mut HostLive,
        dealt: &BTreeMap<FormId, f64>,
    ) -> Outs {
        let mut outs = Vec::new();
        // every tracked actor in the ledger: the host's copy gets a new actor
        // only with its first `Assign`, and a raise before that was lost
        for &a in &self.tracked {
            let id = cf(a);
            if live.actors.own.get(id).is_none()
                || live.actors.is_dead(id)
                || gs.dead.contains(&a)
                || super::facts::player_of(a).is_some()
            {
                continue;
            }
            let (Some(now), Some(&last)) = (
                crate::combat::health(cx.order, gs, a),
                self.last_health.get(&a),
            ) else {
                continue;
            };
            let gain = now - last + dealt.get(&a).copied().unwrap_or(0.0);
            if gain <= 0.0 {
                continue;
            }
            let carry = self.heal_carry.entry(a).or_insert(0.0);
            *carry += gain;
            let points = carry.round().min(f64::from(u16::MAX));
            if points < 1.0 {
                continue;
            }
            *carry -= points;
            for o in live.actors.heal(id, points as u16) {
                outs.extend(live.route_extern(o.aud, Msg::Actor(o.msg)));
            }
        }
        outs
    }

    /// Also returns a notice to the striker of each dropped blow
    /// ([`ActorMsg::HitBlocked`]), so that it forgets the part point it kept.
    fn guarded_hits(&mut self, cx: Ctx, gs: &mut GameState) -> (Outs, Outs) {
        let (mut pass, mut blocked) = (Vec::new(), Vec::new());
        for (from, m) in std::mem::take(&mut self.held_hits) {
            let Msg::Actor(ActorMsg::Hit { actor, npc, .. }) = &m else {
                continue;
            };
            if super::campaign::ffguard::allow_hit(cx.order, gs, from.0, wf(*actor)) {
                pass.push((from, m));
            } else {
                let (actor, npc) = (*actor, *npc);
                blocked.push((from, Msg::Actor(ActorMsg::HitBlocked { actor, npc })));
            }
        }
        (pass, blocked)
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
        super::campaign::party::refresh_positions(gs, &self.registry);
        // a guest's followers come along when the guest changes place
        let places = &mut self.pump.places;
        super::campaign::owners::follow_owners(cx.order, cx.scripts, gs, &mut |s, p| {
            places.resolve(cx.order, s, p)
        });
        self.own.local = avatar_of(gs, cx);
        if let Some(m) = self.own.sender.tick(now_ms, &self.own.local) {
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Avatar(m), now_ms, auth));
        }
        // the host's own conversation needs its heartbeat too, or the host
        // closes it (HeartbeatLost) after the last pick
        if let Some(m) = self.own.dialogue.heartbeat(now_ms) {
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Dialogue(m), now_ms, auth));
        }
        for (kind, arg) in std::mem::take(&mut gs.coop.live.local_actions) {
            let m = self.own.action(kind, arg);
            outs.extend(live.on_msg(PlayerId::HOST, m, now_ms, auth));
        }
        gs.coop.live.scaling = Some(live.rules.scaling(now_ms));
        // damage the host dealt itself goes through the one ledger
        let hurts = std::mem::take(&mut gs.coop.live.hurts);
        let mut dealt: BTreeMap<FormId, f64> = BTreeMap::new();
        for h in &hurts {
            *dealt.entry(h.who).or_insert(0.0) += h.amount;
        }
        let me = gs.active_slot();
        let (mine, npcs): (Vec<_>, Vec<_>) = hurts.into_iter().partition(|h| {
            super::campaign::attribution::slot_of(h.by, me) == Some(me)
                || super::campaign::owners::owner_of(gs, h.by) == Some(me)
        });
        // an NPC's blow (Doc Mitchell on a gecko) keeps the ledger's health but
        // credits no player: no XP and no threat for the host
        for (who, amount) in coalesce(npcs) {
            let m = self.own.actors.npc_hit(cf(who), hit_points(amount));
            self.stats.hits_sent += 1;
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Actor(m), now_ms, auth));
        }
        for (who, amount) in coalesce(mine) {
            let m = self.own.actors.hit(cf(who), hit_points(amount));
            self.stats.hits_sent += 1;
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Actor(m), now_ms, auth));
        }
        // a guest's hit requests: the friendly-fire guard, then the same ledger
        let (pass, blocked) = self.guarded_hits(cx, gs);
        for (from, m) in pass {
            outs.extend(live.on_msg(from, m, now_ms, auth));
        }
        // only a guest's blows are held, so each notice goes to a guest
        outs.extend(blocked);
        outs.extend(self.raises(cx, gs, live, &dealt));
        // a guest's dialogue picks: the line's conditions as the picker, then the sessions
        outs.extend(self.dialogue.check_picks(cx, gs, live, auth, now_ms));
        self.adopt_reported(cx, gs, live);
        outs.extend(self.announce_made(cx, gs, live));
        for m in self
            .pump
            .publish(cx, gs, &mut self.own.actors, PlayerId::HOST, now_ms)
        {
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Actor(m), now_ms, auth));
        }
        outs.extend(self.dialogue.host_frame(cx, gs, live));
        outs.extend(self.rules.host_frame(cx, gs, live, now_ms));
        // downs, revives and respawns: the live rules and the campaign tracker agree on every slot
        super::campaign::downed::mirror_rules(gs, &self.rules.events);
        for (slot, revivable) in std::mem::take(&mut gs.coop.campaign.down.to_live) {
            let m = Msg::Rules(coop_core::rules::RulesMsg::ReportDown { revivable });
            outs.extend(live.on_msg(PlayerId(slot), m, now_ms, auth));
        }
        // party message boxes: the question to the others, their casts to the gate
        outs.extend(super::campaign::consent::box_frame(
            gs,
            live,
            auth,
            &self.rules.events,
            now_ms,
        ));
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
        self.last_health = self
            .tracked
            .iter()
            .filter(|a| live.actors.own.get(cf(**a)).is_some())
            .filter_map(|a| Some((*a, crate::combat::health(cx.order, gs, *a)?)))
            .collect();
        self.dialogue.poll_lines(gs, &self.own.dialogue);
        self.dialogue
            .note_refusals(cx, gs, &self.own.dialogue, now_ms);
        // people host scripts made talk: a dialogue scene, or the line for everyone
        outs.extend(
            self.dialogue
                .forced_frame(cx, gs, live, auth, &mut self.own.dialogue, now_ms),
        );
        for m in self.rules.local_frame(cx, gs, &mut self.own.rules) {
            outs.extend(live.on_msg(PlayerId::HOST, m, now_ms, auth));
        }
        for m in self
            .scene
            .local_frame(cx, gs, &mut self.own.scene, true, now_ms)
        {
            outs.extend(live.on_msg(PlayerId::HOST, m, now_ms, auth));
        }
        outs.extend(
            self.interact
                .frame(cx, gs, live, &mut self.own, &self.rules.events, now_ms),
        );
        outs.extend(self.combat.host_frame(
            cx,
            gs,
            live,
            &mut self.own,
            auth,
            &self.rules.events,
            now_ms,
        ));
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
    carry: HitCarry,
    npc_carry: HitCarry,
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
            carry: HitCarry::default(),
            npc_carry: HitCarry::default(),
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
            if puppet_of(self.me) == a
                || super::facts::player_of(a).is_some()
                || gs.place(cx.order, a).is_none()
            {
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
            let Some(base) = live.actors.spawned.get(id) else {
                continue;
            };
            let r = wf(*id);
            if !self.realized.insert(r) {
                continue;
            }
            let at = FormId(cell.0);
            let position = [
                dequant_pos(pos[0]),
                dequant_pos(pos[1]),
                dequant_pos(pos[2]),
            ];
            gs.more
                .placed
                .refs
                .entry(r)
                .or_insert(crate::more_functions::placed::Made {
                    base: wf(*base),
                    space: at,
                    cell: at,
                    position,
                    rotation: [0.0; 3],
                    count: 1,
                });
        }
        let gone: Vec<FormId> = self
            .realized
            .iter()
            .copied()
            .filter(|r| !live.actors.spawn_at.contains_key(&cf(*r)))
            .collect();
        for r in gone {
            self.realized.remove(&r);
            gs.more.placed.refs.remove(&r);
            gs.disabled.insert(r, true);
        }
    }

    /// The per-frame driver. Send the returned messages to the host.
    pub fn frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut GuestLive,
        now_ms: u64,
    ) -> Vec<Msg> {
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
        // a blow the host's friendly-fire guard dropped leaves no part point
        for (actor, npc) in std::mem::take(&mut live.actors.blocked) {
            let carry = if npc {
                &mut self.npc_carry
            } else {
                &mut self.carry
            };
            carry.forget(wf(actor));
        }
        // damage dealt here is a request; the host resolves it
        let pending: Vec<PendingHit> = std::mem::take(&mut gs.coop.live.pending_hits)
            .into_iter()
            .filter(|h| guest_sends(gs, &live.actors, h))
            .collect();
        // the blows of its player and followers are this player's; the others
        // (by or on an actor this guest owns) are NPC blows that credit nobody.
        // NPC blows go first, as on the host: a player's blow in the same frame
        // then gets the kill
        let (mine, npcs): (Vec<_>, Vec<_>) = pending
            .into_iter()
            .partition(|h| h.by == PLAYER_REF || gs.teammates.contains(&h.by));
        for (npc, hits) in [(true, npcs), (false, mine)] {
            // the part points of a player's blows and of NPC blows are kept apart
            let carry = if npc {
                &mut self.npc_carry
            } else {
                &mut self.carry
            };
            for (who, amount) in coalesce(hits) {
                if who == PLAYER_REF || puppet_of(self.me) == who {
                    continue;
                }
                let left = crate::combat::health(cx.order, gs, who);
                let Some(pts) = carry.take(who, amount, left) else {
                    // a player's blow under a point still goes, with 0 points:
                    // the host's friendly-fire guard counts blows, not points
                    if !npc {
                        self.stats.hits_sent += 1;
                        out.push(Msg::Actor(live.actors.hit(cf(who), 0)));
                    }
                    continue;
                };
                self.stats.hits_sent += 1;
                let m = if npc {
                    live.actors.npc_hit(cf(who), pts)
                } else {
                    live.actors.hit(cf(who), pts)
                };
                out.push(Msg::Actor(m));
            }
        }
        self.pump.take_over(cx, gs, &live.actors, self.me);
        out.extend(
            self.pump
                .publish(cx, gs, &mut live.actors, self.me, now_ms)
                .into_iter()
                .map(Msg::Actor),
        );
        self.pump.apply(cx, gs, &live.actors, self.me);
        self.dialogue.poll_lines(gs, &live.dialogue);
        self.dialogue.note_refusals(cx, gs, &live.dialogue, now_ms);
        self.loot.apply(gs, &live.loot);
        out.extend(self.rules.local_frame(cx, gs, &mut live.rules));
        super::campaign::consent::box_prompts(gs, &live.rules);
        out.extend(
            self.scene
                .local_frame(cx, gs, &mut live.scene, false, now_ms),
        );
        out.extend(self.interact.frame(cx, gs, live));
        out.extend(self.combat.guest_frame(cx, gs, live, self.me, now_ms));
        out
    }
}

#[cfg(test)]
mod carry_tests {
    use super::*;

    #[test]
    fn small_blows_add_up_instead_of_each_rounding_to_a_point() {
        let mut c = HitCarry::default();
        let a = FormId(0x100);
        // three blows of 0.4: nothing, nothing, one point (1.2 dealt)
        assert_eq!(c.take(a, 0.4, Some(50.0)), None);
        assert_eq!(c.take(a, 0.4, Some(50.0)), None);
        assert_eq!(c.take(a, 0.4, Some(50.0)), Some(1));
        // 0.2 carried: 3.9 more makes 4.1, four points
        assert_eq!(c.take(a, 3.9, Some(50.0)), Some(4));
        // a blow that finishes the victim counts at once
        assert_eq!(c.take(a, 0.3, Some(0.4)), Some(1));
        // victims are counted apart
        assert_eq!(c.take(FormId(0x200), 2.5, None), Some(2));
    }
}
