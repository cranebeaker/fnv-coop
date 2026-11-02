//! The scene model in the session: what the host reports to the script
//! layer (its loaded cells), the deferred-operation queue, and the plain
//! data the viewer and the state dump read (see `docs/COOP-SCENES.md`).

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::PlayerId;
use coop_core::roam::{DeferredOp, Placement};
use coop_core::sync::{GuestLive, HostLive};
use esm::FormId;
use world::coop::apply::Ctx;
use world::coop::roam_ops;
use world::scripting::{GameState, Runner};

use crate::live::LiveView;

/// Plain data about scenes for one frame.
#[derive(Clone, Debug, Default)]
pub struct RoamView {
    /// Where everyone known is: (player, cell form id, position). Players in
    /// cells this machine has not loaded are present as data only; the
    /// viewer draws a puppet only for those in its loaded cells.
    pub peers: Vec<(PlayerId, u32, [f32; 3])>,
    /// Actors owned per peer (host: all peers; guest: this machine).
    pub owned: Vec<(PlayerId, u32)>,
    /// Actors nobody has loaded (the host ledger holds them).
    pub dormant: u32,
    /// Actors other peers own that this machine mirrors.
    pub remote_actors: u32,
    /// Cells this machine has loaded (host only; a guest knows its own).
    pub loaded: Vec<u32>,
    /// Deferred operations waiting for a cell.
    pub deferred_pending: u32,
    /// Deferred operations queued so far.
    pub deferred_queued: u32,
    /// Deferred operations replayed so far (host) or received (guest).
    pub deferred_replayed: u32,
    /// Operations forwarded to guests.
    pub forwarded: u32,
    /// Forwarded operations confirmed.
    pub confirmed: u32,
    /// Operations a guest applies to its loaded copies this frame.
    pub ref_ops: Vec<DeferredOp>,
    /// The host asked this machine's player to start here (late join, party
    /// warp): the viewer moves the player once.
    pub place: Option<Placement>,
}

fn self_place(gs: &GameState, me: PlayerId) -> Option<(PlayerId, u32, [f32; 3])> {
    Some((me, gs.player_cell?.0, gs.player_position?))
}

/// Host side, once a frame after the live adapter ran.
pub(crate) fn host_frame(cx: Ctx, gs: &mut GameState, live: &mut HostLive, view: &mut LiveView, now_ms: u64) {
    let loaded: BTreeSet<u32> = live.roam.model.loaded_of(PlayerId::HOST).into_iter().map(|c| c.0).collect();
    gs.coop.live.host_loaded = if loaded.is_empty() { None } else { Some(loaded.clone()) };
    for (target, cell, func, args) in std::mem::take(&mut gs.coop.live.deferred_out) {
        live.roam.deferred.queue(coop_core::ids::FormId(target.0), roam_ops::cell_id(cell), &func, args, now_ms);
    }
    let ready = live.roam.take_ready_local();
    if !ready.is_empty() {
        let mut r = Runner::new(cx.order, cx.scripts, gs);
        for op in &ready {
            roam_ops::replay(&mut r, op);
        }
    }
    // only actors in cells this machine has loaded are mirrored
    if !loaded.is_empty() {
        view.remote_actors.retain(|a| loaded.contains(&a.cell));
    }
    let own = &live.actors.own;
    let mut owned: BTreeMap<PlayerId, u32> = BTreeMap::new();
    let mut dormant = 0;
    for (_, rec) in own.iter() {
        match rec.owner {
            Some(p) => *owned.entry(p).or_default() += 1,
            None => dormant += 1,
        }
    }
    let s = &live.roam;
    view.roam = RoamView {
        peers: s.model.places().into_iter().map(|(p, c, pos)| (p, c.0, pos)).collect(),
        owned: owned.into_iter().collect(),
        dormant,
        remote_actors: view.remote_actors.len() as u32,
        loaded: loaded.into_iter().collect(),
        deferred_pending: s.deferred.pending() as u32,
        deferred_queued: s.deferred.queued,
        deferred_replayed: s.deferred.replayed,
        forwarded: s.forwarded,
        confirmed: s.confirmed,
        ref_ops: vec![],
        place: None,
    };
}

/// Guest side, once a frame after the guest adapter ran and the remote
/// avatars were collected into `view`.
pub(crate) fn guest_frame(gs: &GameState, live: &mut GuestLive, view: &mut LiveView) {
    let me = view.me;
    let ops = std::mem::take(&mut live.roam.ref_ops);
    for op in &ops {
        live.roam.done(op.id);
    }
    if let Some(c) = gs.player_cell {
        let loaded = live.roam.graph.loaded_set(coop_core::ids::CellId(c.0));
        view.remote_actors.retain(|a| loaded.contains(&coop_core::ids::CellId(a.cell)));
    }
    let mut peers: BTreeMap<PlayerId, (u32, [f32; 3])> =
        live.roam.places.iter().map(|(p, r)| (*p, (r.cell.0, r.pos))).collect();
    for a in &view.avatars {
        peers.insert(a.player, (a.cell, a.pos));
    }
    peers.retain(|p, _| view.members.is_empty() || view.members.contains(p));
    if let Some((p, c, pos)) = self_place(gs, me) {
        peers.insert(p, (c, pos));
    }
    view.roam = RoamView {
        peers: peers.into_iter().map(|(p, (c, pos))| (p, c, pos)).collect(),
        owned: vec![(me, view.owned_actors.len() as u32)],
        remote_actors: view.remote_actors.len() as u32,
        deferred_replayed: ops.len() as u32,
        ref_ops: ops,
        place: live.roam.place.take(),
        ..RoamView::default()
    };
}

/// A reference's form id in the session's types.
pub fn form(f: coop_core::ids::FormId) -> FormId {
    FormId(f.0)
}

impl crate::Session {
    /// The shared cell table the scene model derives loaded sets from
    /// (register exterior grids and interior links here; see
    /// `world::coop::roam_ops::register_world_grid`).
    pub fn live_roam_graph(&mut self) -> Option<&mut coop_core::roam::CellGraph> {
        match (&mut self.host, &mut self.guest) {
            (Some(h), _) => Some(&mut h.sync.live.roam.model.graph),
            (_, Some(g)) => Some(&mut g.sync.live.roam.graph),
            _ => None,
        }
    }
}

impl crate::Session {
    /// Host: forward deferred script operations to the guest that has the
    /// cell loaded when the host has not. Turn on once the viewer applies
    /// [`RoamView::ref_ops`]; until then operations wait for the host.
    pub fn live_roam_forward_ops(&mut self, on: bool) {
        if let Some(h) = self.host.as_mut() {
            h.sync.live.roam.forward_to_guests = on;
        }
    }
}
