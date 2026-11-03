//! The scene model wired into the host and guest live layers. The hooks in
//! `sync::live` and `sync::glive` are one-line calls into this file.

use std::collections::BTreeMap;

use super::deferred::DeferredOp;
use super::model::{CellDelta, Placement, SceneModel};
use super::wire::RoamMsg;
use crate::actors::{ActorHost, ActorMsg};
use crate::avatar::{dequant_pos, quant_pos};
use crate::ids::{CellId, PlayerId, Seq};
use crate::interest::{Audience, InterestMap};
use crate::proto::Msg;

/// Milliseconds between presence broadcasts.
pub const PRESENCE_EVERY_MS: u64 = 500;

/// What the host adds to its live state for scenes.
#[derive(Clone, Debug, Default)]
pub struct HostRoam {
    /// Per-peer cells and loaded sets.
    pub model: SceneModel,
    /// Deferred script operations for cells nobody has loaded.
    pub deferred: super::deferred::DeferredQueue,
    next_presence_ms: u64,
    presence_seq: u32,
    /// Operations forwarded to guests, not yet confirmed: (op, owner).
    pub in_flight: BTreeMap<u32, PlayerId>,
    /// Operations ready for the host's own game to run (its cell is loaded).
    pub ready_local: Vec<DeferredOp>,
    /// Forward operations to the guest that has the cell loaded when the host
    /// has not (off by default: the viewer must apply `RoamView::ref_ops`
    /// first; until then they wait for the host to load the cell).
    pub forward_to_guests: bool,
    /// Operations forwarded to guests.
    pub forwarded: u32,
    /// Forwarded operations confirmed.
    pub confirmed: u32,
}

/// Messages a cell entry must carry to the entering peer: owners with their
/// last states, dynamic refs, and who is already dead.
pub fn cell_snapshot(actors: &ActorHost, cell: CellId) -> Vec<ActorMsg> {
    let mut v = actors.snapshot_for_cell(cell);
    for (id, rec) in actors.own.iter() {
        if rec.state.cell == cell && actors.is_dead(*id) {
            v.push(ActorMsg::Died {
                actor: *id,
                killer: None,
            });
        }
    }
    v
}

impl HostRoam {
    /// An avatar sample from `from` in `cell` at `pos`: updates the model,
    /// the interest map and, on a cell change, returns the snapshots of
    /// every cell the peer just loaded (not only its home).
    pub fn on_avatar(
        &mut self,
        interest: &mut InterestMap,
        actors: &ActorHost,
        from: PlayerId,
        cell: CellId,
        pos: [f32; 3],
    ) -> Vec<(Audience, Msg)> {
        let d: CellDelta = self.model.update_peer(from, cell, pos);
        // outdoors a new grid square changes the loaded set, not the home
        if d.is_empty() {
            return vec![];
        }
        self.model.apply_interest(interest);
        let mut out = Vec::new();
        for c in &d.entered {
            for a in cell_snapshot(actors, *c) {
                out.push((Audience::To(from), Msg::Actor(a)));
            }
        }
        out
    }

    /// A player left.
    pub fn on_leave(&mut self, interest: &mut InterestMap, p: PlayerId) {
        self.model.remove_peer(p);
        interest.remove_peer(p);
        self.in_flight.retain(|_, owner| *owner != p);
    }

    /// A player joined: tell it where to start.
    pub fn on_join(&self, p: PlayerId) -> Vec<(Audience, Msg)> {
        match self.model.join_placement(p) {
            Some(pl) => vec![(Audience::To(p), Msg::Roam(RoamMsg::place(pl)))],
            None => vec![],
        }
    }

    /// Moves players: one `Place` per player.
    pub fn place_msgs(&self, moves: &[(PlayerId, Placement)]) -> Vec<(Audience, Msg)> {
        moves
            .iter()
            .map(|(p, pl)| (Audience::To(*p), Msg::Roam(RoamMsg::place(*pl))))
            .collect()
    }

    /// Per tick: presence for players who share no scene, and the deferred
    /// operations whose cell is loaded now (to the host's own game, or
    /// forwarded to the guest that has the cell loaded).
    pub fn tick(&mut self, now_ms: u64) -> Vec<(Audience, Msg)> {
        let mut out = Vec::new();
        if now_ms >= self.next_presence_ms {
            self.next_presence_ms = now_ms + PRESENCE_EVERY_MS;
            self.presence_seq = self.presence_seq.wrapping_add(1);
            let places = self.model.places();
            for (p, cell, pos) in &places {
                for (q, _, _) in &places {
                    if q != p && !self.model.share_scene(*p, *q) {
                        out.push((
                            Audience::To(*q),
                            Msg::Roam(RoamMsg::Presence {
                                player: *p,
                                cell: *cell,
                                pos: pos.map(quant_pos),
                                seq: Seq(self.presence_seq),
                            }),
                        ));
                    }
                }
            }
        }
        let model = &self.model;
        let fwd = self.forward_to_guests;
        let ready = self.deferred.drain_loaded(|c| {
            if fwd {
                model.is_loaded_by_any(c)
            } else {
                model.is_loaded_by(PlayerId::HOST, c)
            }
        });
        for op in ready {
            let loaders = self.model.peers_loading(op.cell);
            if loaders.contains(&PlayerId::HOST) {
                self.ready_local.push(op);
            } else if let Some(owner) = loaders.first().copied() {
                self.in_flight.insert(op.id, owner);
                self.forwarded += 1;
                out.push((
                    Audience::To(owner),
                    Msg::Roam(RoamMsg::RefOp {
                        op: op.id,
                        target: op.target,
                        cell: op.cell,
                        func: op.func,
                        args: op.args,
                    }),
                ));
            }
        }
        out
    }

    /// A scene message from a guest.
    pub fn on_msg(&mut self, from: PlayerId, m: RoamMsg) {
        if let RoamMsg::RefOpDone { op } = m {
            if self.in_flight.get(&op) == Some(&from) {
                self.in_flight.remove(&op);
                self.confirmed += 1;
            }
        }
    }

    /// Operations the host's own game must run now.
    pub fn take_ready_local(&mut self) -> Vec<DeferredOp> {
        std::mem::take(&mut self.ready_local)
    }
}

/// A remote player's whereabouts as the guest last heard them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemotePlace {
    /// Cell.
    pub cell: CellId,
    /// Position.
    pub pos: [f32; 3],
    seq: Seq,
}

/// Guest side: presence data, the start placement, operations to apply.
#[derive(Clone, Debug, Default)]
pub struct GuestRoam {
    /// The cell table (the same one the host uses), to know which cells this
    /// machine has loaded around its own.
    pub graph: super::graph::CellGraph,
    /// Players outside this peer's scene, by data only.
    pub places: BTreeMap<PlayerId, RemotePlace>,
    /// A placement the host asked for; the game takes it once.
    pub place: Option<Placement>,
    /// Operations to apply to the loaded copies.
    pub ref_ops: Vec<DeferredOp>,
    outbox: Vec<Msg>,
}

impl GuestRoam {
    /// Applies one scene message from the host.
    pub fn on_msg(&mut self, me: PlayerId, m: RoamMsg) {
        match m {
            RoamMsg::Presence {
                player,
                cell,
                pos,
                seq,
            } if player != me => {
                let newer = self
                    .places
                    .get(&player)
                    .map_or(true, |o| seq.is_newer_than(o.seq));
                if newer {
                    self.places.insert(
                        player,
                        RemotePlace {
                            cell,
                            pos: pos.map(dequant_pos),
                            seq,
                        },
                    );
                }
            }
            RoamMsg::Place { .. } => self.place = m.as_placement(),
            RoamMsg::RefOp {
                op,
                target,
                cell,
                func,
                args,
            } => {
                self.ref_ops.push(DeferredOp {
                    id: op,
                    target,
                    cell,
                    func,
                    args,
                    queued_ms: 0,
                });
            }
            _ => {}
        }
    }

    /// Reports an applied operation to the host.
    pub fn done(&mut self, op: u32) {
        self.outbox.push(Msg::Roam(RoamMsg::RefOpDone { op }));
    }

    /// Messages for the host.
    pub fn take_out(&mut self) -> Vec<Msg> {
        std::mem::take(&mut self.outbox)
    }

    /// Forgets a player who left.
    pub fn forget(&mut self, p: PlayerId) {
        self.places.remove(&p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actors::ActorState;
    use crate::ids::FormId;

    fn p(n: u8) -> PlayerId {
        PlayerId(n)
    }

    #[test]
    fn presence_goes_only_to_players_outside_the_scene() {
        let mut hs = HostRoam::default();
        let mut im = InterestMap::new();
        let a = ActorHost::new();
        hs.on_avatar(&mut im, &a, p(0), CellId(1), [0.0; 3]);
        hs.on_avatar(&mut im, &a, p(1), CellId(1), [0.0; 3]);
        hs.on_avatar(&mut im, &a, p(2), CellId(9), [0.0; 3]);
        let out = hs.tick(0);
        let to_p2: Vec<_> = out
            .iter()
            .filter(|(au, _)| *au == Audience::To(p(2)))
            .collect();
        assert_eq!(to_p2.len(), 2, "p2 hears of p0 and p1");
        assert_eq!(
            out.iter()
                .filter(|(au, _)| *au == Audience::To(p(1)))
                .count(),
            1,
            "p1 shares a scene with p0 only"
        );
        assert!(hs.tick(100).is_empty(), "rate limited");
    }

    #[test]
    fn a_new_cell_sends_snapshots_for_every_cell_it_loads() {
        let mut hs = HostRoam::default();
        hs.model.graph.register_exterior(CellId(10), 1, 0, 0);
        hs.model.graph.register_exterior(CellId(11), 1, 1, 0);
        let mut im = InterestMap::new();
        let mut a = ActorHost::new();
        let st = ActorState {
            cell: CellId(11),
            health: 50,
            ..Default::default()
        };
        a.register(FormId(0x100), st);
        let mut auth = crate::authority::AuthorityTable::new();
        let locs: BTreeMap<_, _> = [(
            p(0),
            crate::actors::PlayerLoc {
                cell: CellId(11),
                pos: [0.0; 3],
            },
        )]
        .into();
        a.tick(0, &locs, &mut auth);
        let out = hs.on_avatar(&mut im, &a, p(1), CellId(10), [0.0; 3]);
        assert!(
            out.iter().any(|(_, m)| matches!(
                m,
                Msg::Actor(ActorMsg::Assign {
                    owner: Some(PlayerId(0)),
                    ..
                })
            )),
            "the neighbour cell's owner is announced although the peer stands in cell 10"
        );
        assert!(hs
            .on_avatar(&mut im, &a, p(1), CellId(10), [5.0; 3])
            .is_empty());
    }

    #[test]
    fn deferred_ops_wait_then_go_to_the_host_or_the_loading_guest() {
        let mut hs = HostRoam {
            forward_to_guests: true,
            ..Default::default()
        };
        let mut im = InterestMap::new();
        let a = ActorHost::new();
        hs.deferred
            .queue(FormId(5), CellId(7), "Activate", vec![], 0);
        hs.deferred
            .queue(FormId(6), CellId(8), "Activate", vec![], 0);
        hs.on_avatar(&mut im, &a, p(0), CellId(1), [0.0; 3]);
        assert!(hs
            .tick(0)
            .iter()
            .all(|(_, m)| !matches!(m, Msg::Roam(RoamMsg::RefOp { .. }))));
        hs.on_avatar(&mut im, &a, p(0), CellId(7), [0.0; 3]);
        hs.on_avatar(&mut im, &a, p(1), CellId(8), [0.0; 3]);
        let out = hs.tick(1000);
        assert_eq!(hs.take_ready_local().len(), 1, "the host loaded cell 7");
        let ops: Vec<_> = out
            .iter()
            .filter(|(_, m)| matches!(m, Msg::Roam(RoamMsg::RefOp { .. })))
            .collect();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].0, Audience::To(p(1)));
        hs.on_msg(p(1), RoamMsg::RefOpDone { op: 2 });
        assert_eq!((hs.confirmed, hs.in_flight.len()), (1, 0));
    }

    #[test]
    fn guest_keeps_the_newest_presence_and_a_placement() {
        let mut g = GuestRoam::default();
        let pr = |seq, x| RoamMsg::Presence {
            player: p(2),
            cell: CellId(3),
            pos: [x, 0, 0],
            seq: Seq(seq),
        };
        g.on_msg(p(1), pr(5, 16));
        g.on_msg(p(1), pr(4, 32));
        assert_eq!(g.places[&p(2)].pos[0], 1.0);
        g.on_msg(
            p(1),
            RoamMsg::Presence {
                player: p(1),
                cell: CellId(3),
                pos: [0; 3],
                seq: Seq(9),
            },
        );
        assert_eq!(g.places.len(), 1, "own presence ignored");
        g.on_msg(
            p(1),
            RoamMsg::place(Placement {
                cell: CellId(4),
                pos: [1.0, 2.0, 3.0],
                heading: 0.0,
            }),
        );
        assert_eq!(g.place.unwrap().cell, CellId(4));
    }
}

#[cfg(test)]
mod forward_tests {
    use super::*;
    use crate::ids::FormId;

    #[test]
    fn without_forwarding_ops_wait_for_the_host() {
        let mut hs = HostRoam::default();
        let mut im = InterestMap::new();
        let a = ActorHost::new();
        hs.deferred
            .queue(FormId(5), CellId(8), "Activate", vec![], 0);
        hs.on_avatar(&mut im, &a, PlayerId(0), CellId(1), [0.0; 3]);
        hs.on_avatar(&mut im, &a, PlayerId(1), CellId(8), [0.0; 3]);
        assert!(hs
            .tick(1000)
            .iter()
            .all(|(_, m)| !matches!(m, Msg::Roam(RoamMsg::RefOp { .. }))));
        assert_eq!(hs.deferred.pending(), 1, "still waiting");
        hs.on_avatar(&mut im, &a, PlayerId(0), CellId(8), [0.0; 3]);
        hs.tick(2000);
        assert_eq!(hs.take_ready_local().len(), 1);
    }
}
