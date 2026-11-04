//! The scene model: where every peer is and what each has loaded.
//!
//! Every peer simulates the scene around its own player (see
//! `docs/COOP-SCENES.md`). The host keeps one [`SceneModel`] with the home
//! cell, position and derived loaded set of every peer. Interest, actor
//! ownership, the tether zone, party warps, door and travel follow rules and
//! late-join placement all read from it.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::graph::{square_of, CellGraph};
use crate::actors::Ownership;
use crate::ids::{CellId, PlayerId};
use crate::interest::InterestMap;

/// Players placed by one warp are spread on a ring of this radius.
pub const WARP_SPREAD: f32 = 96.0;
/// The party slots a warp spreads over: the host and up to three guests.
pub const PARTY_SLOTS: u8 = 4;
/// A follower must stand this close to the mover to come along through a door.
pub const FOLLOW_RADIUS: f32 = 600.0;

/// A place a player can be put: cell, position (world units), heading (radians).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    /// Cell.
    pub cell: CellId,
    /// Position.
    pub pos: [f32; 3],
    /// Heading in radians.
    pub heading: f32,
}

/// The tether zone: a disc in a cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneAnchor {
    /// Anchor cell.
    pub cell: CellId,
    /// Anchor position.
    pub pos: [f32; 3],
    /// Radius of the zone.
    pub radius: f32,
    /// Heading of the anchor, for warps.
    pub heading: f32,
}

/// One peer's scene.
#[derive(Clone, Debug, PartialEq)]
pub struct PeerScene {
    /// The cell the peer's player stands in.
    pub home: CellId,
    /// Position of the player.
    pub pos: [f32; 3],
    /// Every cell the peer has loaded (always contains `home`).
    pub loaded: BTreeSet<CellId>,
}

/// Cells whose load state changed for one peer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CellDelta {
    /// Newly loaded.
    pub entered: Vec<CellId>,
    /// No longer loaded.
    pub left: Vec<CellId>,
    /// True when the home cell changed (or the peer is new).
    pub moved_home: bool,
}

impl CellDelta {
    /// True when nothing changed.
    pub fn is_empty(&self) -> bool {
        self.entered.is_empty() && self.left.is_empty() && !self.moved_home
    }
}

/// Counters for dumps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SceneStats {
    /// Home-cell changes seen.
    pub home_changes: u32,
    /// Cell loads (a peer gaining a cell).
    pub cells_entered: u32,
    /// Cell unloads.
    pub cells_left: u32,
}

/// What a door or fast travel does to the party.
#[derive(Clone, Debug, PartialEq)]
pub struct Transit {
    /// Players that move, with where they end up (the mover first).
    pub moves: Vec<(PlayerId, Placement)>,
    /// True when the party must vote first (a tether holds the story).
    pub needs_vote: bool,
}

/// Where the host keeps every peer's scene.
#[derive(Clone, Debug, Default)]
pub struct SceneModel {
    /// The shared cell table.
    pub graph: CellGraph,
    peers: BTreeMap<PlayerId, PeerScene>,
    follow: BTreeMap<PlayerId, PlayerId>,
    anchor: Option<ZoneAnchor>,
    /// Counters.
    pub stats: SceneStats,
}

fn dist2(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

impl SceneModel {
    /// Empty model.
    pub fn new() -> Self {
        SceneModel {
            graph: CellGraph::new(),
            ..Default::default()
        }
    }

    /// Records where a peer's player stands (call on every avatar sample).
    /// Returns which cells that loaded or unloaded for the peer.
    pub fn update_peer(&mut self, p: PlayerId, home: CellId, pos: [f32; 3]) -> CellDelta {
        let known = self.peers.get(&p).map(|s| s.home);
        // outdoors the home is the worldspace: a new grid square loads other
        // cells, and so does a grid registered after the peer arrived
        let under = self.graph.cell_under(home, pos);
        let new_square = self.graph.is_world(home)
            && self.peers.get(&p).is_some_and(|s| {
                square_of(s.pos) != square_of(pos) || under.is_some_and(|c| !s.loaded.contains(&c))
            });
        let moved_home = known != Some(home);
        if !moved_home && !new_square {
            if let Some(s) = self.peers.get_mut(&p) {
                s.pos = pos;
            }
            return CellDelta::default();
        }
        let loaded = self.graph.loaded_at(home, pos);
        let old = self.peers.insert(
            p,
            PeerScene {
                home,
                pos,
                loaded: loaded.clone(),
            },
        );
        let before = old.map(|o| o.loaded).unwrap_or_default();
        let d = CellDelta {
            entered: loaded.difference(&before).copied().collect(),
            left: before.difference(&loaded).copied().collect(),
            moved_home,
        };
        self.stats.home_changes += u32::from(moved_home);
        self.stats.cells_entered += d.entered.len() as u32;
        self.stats.cells_left += d.left.len() as u32;
        d
    }

    /// Forgets a peer (disconnect).
    pub fn remove_peer(&mut self, p: PlayerId) -> CellDelta {
        self.follow.remove(&p);
        self.follow.retain(|_, leader| *leader != p);
        let left = self
            .peers
            .remove(&p)
            .map(|s| s.loaded.into_iter().collect())
            .unwrap_or_default();
        CellDelta {
            entered: vec![],
            left,
            moved_home: false,
        }
    }

    /// A peer's scene.
    pub fn peer(&self, p: PlayerId) -> Option<&PeerScene> {
        self.peers.get(&p)
    }

    /// Every peer with a known scene.
    pub fn peers(&self) -> impl Iterator<Item = (&PlayerId, &PeerScene)> {
        self.peers.iter()
    }

    /// The cells a peer has loaded.
    pub fn loaded_of(&self, p: PlayerId) -> BTreeSet<CellId> {
        self.peers
            .get(&p)
            .map(|s| s.loaded.clone())
            .unwrap_or_default()
    }

    /// Peers that have `cell` loaded, lowest slot first.
    pub fn peers_loading(&self, cell: CellId) -> Vec<PlayerId> {
        self.peers
            .iter()
            .filter(|(_, s)| s.loaded.contains(&cell))
            .map(|(p, _)| *p)
            .collect()
    }

    /// True when any peer has `cell` loaded.
    pub fn is_loaded_by_any(&self, cell: CellId) -> bool {
        self.peers.values().any(|s| s.loaded.contains(&cell))
    }

    /// True when `p` has `cell` loaded.
    pub fn is_loaded_by(&self, p: PlayerId, cell: CellId) -> bool {
        self.peers.get(&p).is_some_and(|s| s.loaded.contains(&cell))
    }

    /// True when `p` has loaded a thing filed under `cell` at `pos` (outdoors
    /// the grid square decides, see [`CellGraph::sees`]).
    pub fn sees(&self, p: PlayerId, cell: CellId, pos: [f32; 3]) -> bool {
        self.peers
            .get(&p)
            .is_some_and(|s| self.graph.sees(s.home, s.pos, cell, pos))
    }

    /// The grid square `p`'s player stands in, when it stands outdoors in a
    /// registered worldspace.
    pub fn square_of_peer(&self, p: PlayerId) -> Option<(i32, i32)> {
        let s = self.peers.get(&p)?;
        self.graph.is_world(s.home).then(|| square_of(s.pos))
    }

    /// Players standing in the same home cell as `p` (not counting `p`).
    pub fn cell_mates(&self, p: PlayerId) -> Vec<PlayerId> {
        let Some(me) = self.peers.get(&p) else {
            return vec![];
        };
        self.peers
            .iter()
            .filter(|(q, s)| **q != p && s.home == me.home)
            .map(|(q, _)| *q)
            .collect()
    }

    /// True when the two peers' loaded sets overlap: each is within the
    /// other's scene.
    pub fn share_scene(&self, a: PlayerId, b: PlayerId) -> bool {
        match (self.peers.get(&a), self.peers.get(&b)) {
            (Some(x), Some(y)) => !x.loaded.is_disjoint(&y.loaded),
            _ => false,
        }
    }

    /// Feeds the interest map: each peer sees exactly the cells it has loaded.
    pub fn apply_interest(&self, im: &mut InterestMap) {
        for (p, s) in &self.peers {
            im.set_peer_cell(*p, s.home);
            im.set_peer_reach(*p, s.loaded.clone());
        }
    }

    /// Feeds ownership: players also own actors in loaded cells other than
    /// their home when nobody stands there.
    pub fn apply_ownership(&self, own: &mut Ownership) {
        own.reach = self
            .peers
            .iter()
            .map(|(p, s)| (*p, s.loaded.clone()))
            .collect();
    }

    /// Where every peer is, for the party list and the compass: data only.
    pub fn places(&self) -> Vec<(PlayerId, CellId, [f32; 3])> {
        self.peers
            .iter()
            .map(|(p, s)| (*p, s.home, s.pos))
            .collect()
    }

    /// Sets (or clears) the tether zone.
    pub fn set_anchor(&mut self, a: Option<ZoneAnchor>) {
        self.anchor = a;
    }

    /// The tether zone.
    pub fn anchor(&self) -> Option<ZoneAnchor> {
        self.anchor
    }

    /// Is `p` inside a disc of `radius` around (`cell`, `pos`)? A player in
    /// another cell is outside, except in a loaded cell of the same
    /// worldspace, where coordinates share one frame. Unknown players count
    /// as inside (they cannot hold the party back).
    pub fn in_zone(&self, p: PlayerId, cell: CellId, pos: [f32; 3], radius: f32) -> bool {
        let Some(s) = self.peers.get(&p) else {
            return true;
        };
        let reachable =
            s.home == cell || (s.loaded.contains(&cell) && self.graph.same_space(s.home, cell));
        reachable && dist2(s.pos, pos) <= radius
    }

    /// [`in_zone`](Self::in_zone) against the tether anchor; true with no tether.
    pub fn in_tether_zone(&self, p: PlayerId) -> bool {
        match self.anchor {
            Some(a) => self.in_zone(p, a.cell, a.pos, a.radius),
            None => true,
        }
    }

    /// Players put on a ring around `to`, so a party warp does not stack them.
    pub fn spread(&self, players: &[PlayerId], to: Placement) -> Vec<(PlayerId, Placement)> {
        let n = players.len().max(1) as f32;
        players
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let a = std::f32::consts::TAU * (i as f32) / n;
                let off = if players.len() <= 1 { 0.0 } else { WARP_SPREAD };
                (
                    *p,
                    Placement {
                        cell: to.cell,
                        pos: [
                            to.pos[0] + a.cos() * off,
                            to.pos[1] + a.sin() * off,
                            to.pos[2],
                        ],
                        heading: to.heading,
                    },
                )
            })
            .collect()
    }

    /// Party warp: everyone to one place (a barrier released, a tether warp).
    pub fn party_warp(&self, members: &[PlayerId], to: Placement) -> Vec<(PlayerId, Placement)> {
        self.spread(members, to)
    }

    /// Player `who`'s own spot on the party ring around `to`: [`WARP_SPREAD`]
    /// from `to`, at the angle of its slot ([`PARTY_SLOTS`]). Players sent to
    /// one place then do not stand on each other. A slot out of the ring
    /// gets `to` itself.
    pub fn ring_spot(to: Placement, who: PlayerId) -> Placement {
        let party: Vec<PlayerId> = (0..PARTY_SLOTS).map(PlayerId).collect();
        Self::default()
            .spread(&party, to)
            .into_iter()
            .find(|(p, _)| *p == who)
            .map_or(to, |(_, s)| s)
    }

    /// Where a late joiner starts: its own spot on the party ring
    /// ([`ring_spot`](Self::ring_spot)) around the tether anchor when a tether
    /// holds the story, else around the host, else around any member in the
    /// scene. Before, the joiner stood on the exact spot, on the player there.
    pub fn join_placement(&self, joiner: PlayerId) -> Option<Placement> {
        let to = if let Some(a) = self.anchor {
            Placement {
                cell: a.cell,
                pos: a.pos,
                heading: a.heading,
            }
        } else {
            let pick = self
                .peers
                .iter()
                .filter(|(p, _)| **p != joiner)
                .min_by_key(|(p, _)| p.0)
                .map(|(_, s)| s)?;
            Placement {
                cell: pick.home,
                pos: pick.pos,
                heading: 0.0,
            }
        };
        Some(Self::ring_spot(to, joiner))
    }

    /// `p` follows `leader` through doors and fast travel (`None` stops).
    pub fn set_follow(&mut self, p: PlayerId, leader: Option<PlayerId>) {
        match leader {
            Some(l) if l != p => {
                self.follow.insert(p, l);
            }
            _ => {
                self.follow.remove(&p);
            }
        }
    }

    /// Who follows `leader` and is close enough to come along.
    pub fn followers_of(&self, leader: PlayerId) -> Vec<PlayerId> {
        let Some(l) = self.peers.get(&leader) else {
            return vec![];
        };
        self.follow
            .iter()
            .filter(|(_, ld)| **ld == leader)
            .filter_map(|(f, _)| self.peers.get(f).map(|s| (*f, s)))
            .filter(|(_, s)| s.home == l.home && dist2(s.pos, l.pos) <= FOLLOW_RADIUS)
            .map(|(f, _)| f)
            .collect()
    }

    /// A door or fast travel by `mover` to `to`. Free roam: only the mover
    /// goes (plus followers standing next to it). With a tether holding the
    /// story, a destination outside the zone needs a party vote.
    pub fn plan_transit(&self, mover: PlayerId, to: Placement) -> Transit {
        let mut who = vec![mover];
        who.extend(self.followers_of(mover));
        let needs_vote = self
            .anchor
            .is_some_and(|a| !(to.cell == a.cell && dist2(to.pos, a.pos) <= a.radius));
        Transit {
            moves: self.spread(&who, to),
            needs_vote,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(n: u8) -> PlayerId {
        PlayerId(n)
    }
    fn grid() -> SceneModel {
        let mut m = SceneModel::new();
        for x in 0..8 {
            for y in 0..8 {
                m.graph
                    .register_exterior(CellId(100 + (x * 8 + y) as u32), 0x3C, x, y);
            }
        }
        m
    }

    #[test]
    fn moving_home_reports_the_cells_entered_and_left() {
        let mut m = grid();
        let d = m.update_peer(p(1), CellId(100 + 9), [0.0; 3]);
        assert!(d.moved_home && d.left.is_empty());
        assert_eq!(
            d.entered.len(),
            16,
            "neighbourhood 4x4 around (1,1) clipped at the edge"
        );
        assert!(
            m.update_peer(p(1), CellId(100 + 9), [5.0, 0.0, 0.0])
                .is_empty(),
            "same cell only moves"
        );
        let d = m.update_peer(p(1), CellId(100 + 17), [0.0; 3]);
        assert_eq!(d.entered.len(), 4, "one row in");
        assert!(d.moved_home);
    }

    #[test]
    fn worldspace_peers_load_the_square_around_their_position() {
        let mut m = grid();
        let w = CellId(0x3C);
        let cell = |x: f32, y: f32| [x * 4096.0 + 10.0, y * 4096.0 + 10.0, 0.0];
        let d = m.update_peer(p(0), w, cell(1.0, 1.0));
        assert!(d.moved_home);
        assert_eq!(d.entered.len(), 17, "the worldspace and 16 grid cells");
        assert!(
            m.update_peer(p(0), w, cell(1.2, 1.0)).is_empty(),
            "same square"
        );
        let d = m.update_peer(p(0), w, cell(2.0, 1.0));
        assert!(!d.moved_home && d.entered.len() == 4, "one column in");
        m.update_peer(p(1), w, cell(7.0, 7.0));
        assert_eq!(m.square_of_peer(p(1)), Some((7, 7)));
        assert!(m.sees(p(0), w, cell(3.0, 2.0)));
        assert!(!m.sees(p(0), w, cell(7.0, 7.0)), "the far peer's square");
        assert!(m.sees(p(1), w, cell(6.0, 6.0)));
    }

    #[test]
    fn a_grid_registered_late_still_loads_around_the_peer() {
        let mut m = SceneModel::new();
        let w = CellId(0x3C);
        m.update_peer(p(0), w, [10.0, 10.0, 0.0]);
        assert_eq!(m.loaded_of(p(0)).len(), 1, "nothing registered yet");
        for x in 0..3 {
            for y in 0..3 {
                m.graph
                    .register_exterior(CellId(100 + (x * 3 + y) as u32), 0x3C, x, y);
            }
        }
        let d = m.update_peer(p(0), w, [20.0, 10.0, 0.0]);
        assert!(!d.moved_home);
        assert_eq!(d.entered.len(), 9, "the same square, now with its cells");
    }

    #[test]
    fn interior_and_exterior_peers_do_not_share_a_scene() {
        let mut m = grid();
        m.update_peer(p(0), CellId(100 + 27), [0.0; 3]);
        m.update_peer(p(1), CellId(5), [0.0; 3]);
        m.update_peer(p(2), CellId(100 + 28), [0.0; 3]);
        assert!(!m.share_scene(p(0), p(1)));
        assert!(m.share_scene(p(0), p(2)));
        assert_eq!(m.peers_loading(CellId(5)), vec![p(1)]);
        assert!(m.is_loaded_by_any(CellId(100 + 29)));
        assert!(!m.is_loaded_by_any(CellId(100 + 63)));
        m.remove_peer(p(2));
        assert_eq!(m.cell_mates(p(0)), Vec::<PlayerId>::new());
    }

    #[test]
    fn tether_zone_spans_loaded_grid_cells_but_not_interiors() {
        let mut m = grid();
        let anchor = ZoneAnchor {
            cell: CellId(100 + 27),
            pos: [1000.0, 0.0, 0.0],
            radius: 4096.0,
            heading: 0.0,
        };
        m.set_anchor(Some(anchor));
        m.update_peer(p(0), CellId(100 + 27), [0.0; 3]);
        m.update_peer(p(1), CellId(100 + 28), [3000.0, 0.0, 0.0]);
        m.update_peer(p(2), CellId(5), [1000.0, 0.0, 0.0]);
        m.update_peer(p(3), CellId(100 + 28), [90_000.0, 0.0, 0.0]);
        assert!(m.in_tether_zone(p(0)));
        assert!(m.in_tether_zone(p(1)), "next grid cell, close by");
        assert!(
            !m.in_tether_zone(p(2)),
            "an interior is another scene however close its numbers"
        );
        assert!(!m.in_tether_zone(p(3)), "too far");
        assert!(
            m.in_tether_zone(p(9)),
            "unknown players do not hold the party"
        );
    }

    #[test]
    fn join_goes_to_the_anchor_else_the_host() {
        let mut m = grid();
        m.update_peer(p(0), CellId(7), [10.0, 20.0, 0.0]);
        m.update_peer(p(1), CellId(8), [0.0; 3]);
        let j = m.join_placement(p(2)).unwrap();
        assert_eq!(j.cell, CellId(7));
        let d = (j.pos[0] - 10.0).hypot(j.pos[1] - 20.0);
        assert!(
            (d - WARP_SPREAD).abs() < 0.01,
            "beside the host, not on it: {d}"
        );
        assert_eq!(j.pos[2], 0.0);
        let k = m.join_placement(p(3)).unwrap();
        assert!(
            (k.pos[0] - j.pos[0]).hypot(k.pos[1] - j.pos[1]) > 1.0,
            "two joiners do not stand on each other"
        );
        m.set_anchor(Some(ZoneAnchor {
            cell: CellId(9),
            pos: [1.0, 2.0, 3.0],
            radius: 10.0,
            heading: 1.0,
        }));
        let j = m.join_placement(p(2)).unwrap();
        assert_eq!(j.cell, CellId(9));
        let d = (j.pos[0] - 1.0).hypot(j.pos[1] - 2.0);
        assert!((d - WARP_SPREAD).abs() < 0.01, "beside the anchor: {d}");
        assert!(SceneModel::new().join_placement(p(1)).is_none());
    }

    #[test]
    fn transit_takes_close_followers_and_a_tether_asks_for_a_vote() {
        let mut m = grid();
        m.update_peer(p(0), CellId(7), [0.0; 3]);
        m.update_peer(p(1), CellId(7), [100.0, 0.0, 0.0]);
        m.update_peer(p(2), CellId(7), [9000.0, 0.0, 0.0]);
        m.set_follow(p(1), Some(p(0)));
        m.set_follow(p(2), Some(p(0)));
        let to = Placement {
            cell: CellId(40),
            pos: [0.0; 3],
            heading: 0.0,
        };
        let t = m.plan_transit(p(0), to);
        assert_eq!(
            t.moves.iter().map(|x| x.0).collect::<Vec<_>>(),
            vec![p(0), p(1)]
        );
        assert!(!t.needs_vote);
        assert!(t.moves.iter().all(|(_, pl)| pl.cell == CellId(40)));
        assert_ne!(t.moves[0].1.pos, t.moves[1].1.pos, "spread, not stacked");
        m.set_anchor(Some(ZoneAnchor {
            cell: CellId(7),
            pos: [0.0; 3],
            radius: 100.0,
            heading: 0.0,
        }));
        assert!(m.plan_transit(p(0), to).needs_vote);
        m.remove_peer(p(0));
        assert!(m.followers_of(p(0)).is_empty());
    }
}
