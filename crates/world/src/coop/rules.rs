//! Session rules driven from the facts `GameState` has: the story tether
//! (party distance to an anchor), downed and revive (player health), votes
//! (fast travel, sleep, wait, fork), barriers, and difficulty scaling.
//!
//! The host runs `RulesHost`; every machine, the host's too, reacts to the
//! notices its `RulesClient` collects (warp, revive, travel approved).

use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::rules::{Notice, RulesClient, RulesEvent, RulesMsg, VoteKind};
use coop_core::sync::HostLive;
use esm::FormId;

use super::apply::Ctx;
use super::facts::LiveFacts;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Event, GameState};

/// How close to the anchor a player counts as inside the tether zone.
pub const TETHER_RADIUS: f32 = 4096.0;
/// How often the host re-reads the party's distance.
const ZONE_EVERY_MS: u64 = 250;
/// How often the host repeats the party scaling.
const SCALING_EVERY_MS: u64 = 2_000;

/// Rules bookkeeping for one machine.
#[derive(Default)]
pub struct RulesBridge {
    next_zone_ms: u64,
    next_scaling_ms: u64,
    anchor: Option<FormId>,
    /// Events the host rules produced (the host game may act on them).
    pub events: Vec<RulesEvent>,
    /// Warps this machine performed, by anchor.
    pub warps: Vec<u32>,
    /// Revives this machine received, by percent.
    pub revives: Vec<u8>,
}

/// Moves the local player to `place` (as a script's `MoveTo` does).
pub fn warp_player(cx: Ctx, gs: &mut GameState, place: FormId) -> bool {
    let Some((space, cell, position, heading)) = gs.place(cx.order, place) else {
        return false;
    };
    gs.player_world = (space != cell).then_some(space);
    gs.player_cell = Some(cell);
    gs.player_position = Some(position);
    gs.player_heading = heading;
    crate::script_functions::player_moved(gs);
    gs.events.push(Event::MoveTo {
        what: PLAYER_REF,
        to: place,
        at: None,
    });
    true
}

/// The party slots a warp spreads over (the host and up to three guests).
const PARTY_SLOTS: u8 = 4;

/// After a party warp the local player `me` moves from the anchor to its own
/// spot on the ring around it (`SceneModel::party_warp`). Warped players then
/// stand neither on the anchor (where the player who set it stands) nor on
/// each other.
fn spread_on_ring(gs: &mut GameState, me: PlayerId) {
    use coop_core::roam::{Placement, SceneModel};
    let (Some(cell), Some(pos)) = (gs.player_cell, gs.player_position) else {
        return;
    };
    let to = Placement {
        cell: coop_core::ids::CellId(cell.0),
        pos,
        heading: gs.player_heading,
    };
    let party: Vec<PlayerId> = (0..PARTY_SLOTS).map(PlayerId).collect();
    let spot = SceneModel::default()
        .party_warp(&party, to)
        .into_iter()
        .find(|(p, _)| *p == me);
    let Some((_, p)) = spot else {
        return;
    };
    gs.player_position = Some(p.pos);
    // the viewer moves the player on the warp's `MoveTo`: give it the spot
    for e in gs.events.iter_mut().rev() {
        if let Event::MoveTo { what, at, .. } = e {
            if *what == PLAYER_REF {
                *at = Some(p.pos);
                break;
            }
        }
    }
}

/// The local player's health to `pct` percent of full.
fn heal_to(cx: Ctx, gs: &mut GameState, pct: f64) {
    let full = crate::combat::max_health(cx.order, gs, PLAYER_REF).unwrap_or(100.0);
    gs.damage.insert(PLAYER_REF, full * (1.0 - pct / 100.0));
    gs.dead.remove(&PLAYER_REF);
}

impl RulesBridge {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Host: starts the story tether at `anchor` (a placed reference).
    pub fn start_tether(
        &mut self,
        cx: Ctx,
        gs: &GameState,
        live: &mut HostLive,
        anchor: FormId,
        now_ms: u64,
    ) {
        self.anchor = Some(anchor);
        // the zone is filed as the live layer files players: an interior
        // cell, or outdoors the worldspace (`live::space_of_player`)
        if let Some((space, _, pos, heading)) = gs.place(cx.order, anchor) {
            let zone = coop_core::roam::ZoneAnchor {
                cell: coop_core::ids::CellId(space.0),
                pos,
                radius: TETHER_RADIUS,
                heading,
            };
            live.roam.model.set_anchor(Some(zone));
        }
        let party = live.rules.party().to_vec();
        let inside: Vec<PlayerId> = party
            .iter()
            .copied()
            .filter(|p| self.inside(cx, gs, live, anchor, *p))
            .collect();
        live.rules.tether.start(now_ms, anchor.0, &party, &inside);
        live.rules.tether_votes_fast_travel = true;
    }

    /// Host: the story moment is over.
    pub fn end_tether(&mut self, live: &mut HostLive) {
        self.anchor = None;
        live.roam.model.set_anchor(None);
        live.rules.tether.end();
        live.rules.tether_votes_fast_travel = false;
    }

    fn inside(
        &self,
        cx: Ctx,
        gs: &GameState,
        live: &HostLive,
        anchor: FormId,
        p: PlayerId,
    ) -> bool {
        let Some(_) = live.avatars.state(p) else {
            return true;
        };
        // outdoors the live layer files a player under the worldspace, not
        // the grid cell: compare the anchor's place the same way, so every
        // player in the open world is measured by distance alone
        let Some((space, _, pos, _)) = gs.place(cx.order, anchor) else {
            return true;
        };
        // the scene model knows which cells each player has loaded: a grid
        // neighbour inside the radius is inside, another scene never is
        live.roam
            .model
            .in_zone(p, coop_core::ids::CellId(space.0), pos, TETHER_RADIUS)
    }

    /// Host, every frame: party distance to the tether anchor, combat flag,
    /// and whether the story stage is held.
    pub fn host_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        if now_ms >= self.next_scaling_ms {
            // join-time announcements can reach a guest still syncing: repeat
            self.next_scaling_ms = now_ms + SCALING_EVERY_MS;
            let o = live.rules.scaling_announcement();
            outs.extend(live.route_extern(o.aud, Msg::Rules(o.msg)));
        }
        live.rules.in_combat = !gs.combat.is_empty();
        // the host's policy rows lock fast travel for every player
        live.rules.fast_travel_locked = gs.coop_fast_travel_blocked();
        gs.coop.live.rules_choice = Some((live.rules.mode(), live.rules.friendly_fire));
        gs.coop.live.stage_held = live.rules.tether.stage_held();
        if let (Some(anchor), true) = (self.anchor, now_ms >= self.next_zone_ms) {
            self.next_zone_ms = now_ms + ZONE_EVERY_MS;
            let mut zone = Vec::new();
            for p in live.rules.party().to_vec() {
                let inside = self.inside(cx, gs, live, anchor, p);
                live.rules.tether.set_in_zone(p, inside, now_ms);
                zone.push((p.0, inside));
            }
            gs.coop.live.tether_zone = zone;
        } else if self.anchor.is_none() {
            gs.coop.live.tether_zone.clear();
        }
        self.events.extend(live.rules.take_events());
        outs
    }

    /// Host: opens a barrier for a game-defined transition; returns sends.
    pub fn open_barrier(
        &mut self,
        live: &mut HostLive,
        tag: u32,
        now_ms: u64,
    ) -> (u32, Vec<(PlayerId, Msg)>) {
        let (id, outs) = live.rules.open_barrier(tag, now_ms);
        let mut sends = Vec::new();
        for o in outs {
            sends.extend(live.route_extern(o.aud, Msg::Rules(o.msg)));
        }
        (id, sends)
    }

    /// Asks the party for a vote (sleep, wait, fork, fast travel).
    pub fn request_vote(&self, kind: VoteKind, param: u32) -> Msg {
        Msg::Rules(RulesMsg::VoteRequest { kind, param })
    }

    /// Asks to fast travel to a marker (the `map::travel` gate queued it).
    pub fn request_travel(&self, marker: FormId) -> Msg {
        self.request_vote(VoteKind::FastTravel, marker.0)
    }

    /// Helper: starts holding the revive key next to a downed player.
    pub fn revive_begin(&self, target: PlayerId) -> Msg {
        Msg::Rules(RulesMsg::ReviveBegin { target })
    }

    /// Helper: let go.
    pub fn revive_cancel(&self) -> Msg {
        Msg::Rules(RulesMsg::ReviveCancel)
    }

    /// Every machine, every frame: copies the scaling, reports the local
    /// player going down, and turns the client's notices into world changes.
    /// Returns messages for the host.
    pub fn local_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        client: &mut RulesClient,
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        gs.coop.live.scaling = Some(client.scaling);
        // the host reads its own rules in `host_frame`
        let host = gs.coop.authority == super::Authority::Host;
        if let (Some(mode), false) = (client.mode, host) {
            gs.coop.live.rules_choice = Some((mode, client.friendly_fire));
        }
        if gs.coop.live.down_pending {
            gs.coop.live.down_pending = false;
            if !gs.coop.live.downed {
                gs.coop.live.downed = true;
                // the campaign's rule for this down (host): a collar death is not revivable
                let revivable = super::campaign::downed::revivable(gs, gs.active_slot());
                out.push(Msg::Rules(client.report_down(revivable)));
            }
        }
        let me = client.me();
        for n in std::mem::take(&mut client.notices) {
            self.apply_notice(cx, gs, n, me);
        }
        out
    }

    fn apply_notice(&mut self, cx: Ctx, gs: &mut GameState, n: Notice, me: PlayerId) {
        match n {
            Notice::Warp(anchor) => {
                if warp_player(cx, gs, FormId(anchor)) {
                    spread_on_ring(gs, me);
                    self.warps.push(anchor);
                }
            }
            Notice::Revived(pct) => {
                heal_to(cx, gs, f64::from(pct));
                gs.coop.live.downed = false;
                self.revives.push(pct);
            }
            Notice::Respawn => {
                // a down the campaign holds until a checkpoint stays down here
                if super::campaign::downed::held(gs, gs.active_slot()) {
                    return;
                }
                heal_to(cx, gs, 100.0);
                gs.coop.live.downed = false;
                if let Some(a) = self.anchor {
                    if warp_player(cx, gs, a) {
                        spread_on_ring(gs, me);
                    }
                }
            }
            Notice::VoteDone {
                kind,
                param,
                approved,
            } => apply_vote(&mut gs.coop.live, kind, param, approved),
            Notice::Wipe => {
                gs.coop.live.wiped = true;
                gs.coop.live.downed = false;
            }
            Notice::Warn(_) | Notice::BarrierPassed(_) | Notice::BarrierCancelled(..) => {}
        }
    }
}

fn apply_vote(f: &mut LiveFacts, kind: VoteKind, param: u32, approved: bool) {
    if !approved {
        match kind {
            VoteKind::FastTravel => f.travel_requests.retain(|m| m.0 != param),
            VoteKind::Fork | VoteKind::Consent => f.fork_result = Some((param, false)),
            _ => {}
        }
        return;
    }
    match kind {
        VoteKind::FastTravel => f.travel_allowed = Some(FormId(param)),
        VoteKind::Sleep | VoteKind::Wait => {
            f.time_skip_ok = Some(param);
            f.scene.time_approved.push((kind == VoteKind::Sleep, param));
        }
        VoteKind::Fork | VoteKind::Consent => f.fork_result = Some((param, true)),
    }
}
