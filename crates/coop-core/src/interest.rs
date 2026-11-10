//! Cell-based interest management and a priority accumulator that spends a
//! byte budget per tick.

use std::collections::{BTreeMap, BTreeSet};

use crate::ids::{CellId, FormId, PlayerId};

/// Who a host-side output message goes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audience {
    /// One peer.
    To(PlayerId),
    /// Everyone who can see the cell.
    Cell(CellId),
    /// Everyone who can see the cell, except one peer.
    CellExcept(CellId, PlayerId),
    /// Every member of the session.
    All,
    /// Every member except one.
    AllExcept(PlayerId),
}

/// Which peers should receive updates for entities in which cells.
#[derive(Clone, Debug, Default)]
pub struct InterestMap {
    peer_cell: BTreeMap<PlayerId, CellId>,
    neighbors: BTreeMap<CellId, BTreeSet<CellId>>,
    /// Cells a peer has loaded besides its own (scene model).
    reach: BTreeMap<PlayerId, BTreeSet<CellId>>,
}

impl InterestMap {
    /// Empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records the cell a peer currently stands in. Returns the old cell.
    pub fn set_peer_cell(&mut self, p: PlayerId, cell: CellId) -> Option<CellId> {
        self.peer_cell.insert(p, cell)
    }

    /// The cell of a peer.
    pub fn peer_cell(&self, p: PlayerId) -> Option<CellId> {
        self.peer_cell.get(&p).copied()
    }

    /// Forgets a peer (disconnect).
    pub fn remove_peer(&mut self, p: PlayerId) {
        self.peer_cell.remove(&p);
        self.reach.remove(&p);
    }

    /// Records every cell a peer has loaded (its home and the cells around
    /// it): it sees things in all of them.
    pub fn set_peer_reach(&mut self, p: PlayerId, cells: BTreeSet<CellId>) {
        self.reach.insert(p, cells);
    }

    /// Declares `a` and `b` visible to each other (exterior grid neighbours,
    /// an interior seen through a door). Symmetric.
    pub fn link(&mut self, a: CellId, b: CellId) {
        self.neighbors.entry(a).or_default().insert(b);
        self.neighbors.entry(b).or_default().insert(a);
    }

    /// True when a peer standing in `peer_cell` should see things in `cell`.
    pub fn visible(&self, peer_cell: CellId, cell: CellId) -> bool {
        peer_cell == cell
            || self
                .neighbors
                .get(&peer_cell)
                .is_some_and(|n| n.contains(&cell))
    }

    /// Peers that should receive an update for an entity in `cell`.
    pub fn peers_seeing(&self, cell: CellId) -> Vec<PlayerId> {
        self.peer_cell
            .iter()
            .filter(|(p, c)| {
                self.visible(**c, cell) || self.reach.get(p).is_some_and(|r| r.contains(&cell))
            })
            .map(|(p, _)| *p)
            .collect()
    }

    /// Same as [`peers_seeing`](Self::peers_seeing) minus `except`.
    pub fn peers_seeing_except(&self, cell: CellId, except: PlayerId) -> Vec<PlayerId> {
        let mut v = self.peers_seeing(cell);
        v.retain(|p| *p != except);
        v
    }

    /// Resolves an audience to concrete recipients. `members` is the whole
    /// session (host included); cell audiences only reach members whose cell
    /// is known and visible.
    pub fn resolve(&self, aud: Audience, members: &[PlayerId]) -> Vec<PlayerId> {
        match aud {
            Audience::To(p) => members.iter().copied().filter(|m| *m == p).collect(),
            Audience::Cell(c) => self
                .peers_seeing(c)
                .into_iter()
                .filter(|p| members.contains(p))
                .collect(),
            Audience::CellExcept(c, x) => self
                .peers_seeing_except(c, x)
                .into_iter()
                .filter(|p| members.contains(p))
                .collect(),
            Audience::All => members.to_vec(),
            Audience::AllExcept(x) => members.iter().copied().filter(|m| *m != x).collect(),
        }
    }

    /// Every known peer.
    pub fn all_peers(&self) -> Vec<PlayerId> {
        self.peer_cell.keys().copied().collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct Item {
    weight: f32,
    acc: f32,
}

/// Per-recipient send scheduler. Entities that changed accumulate priority
/// every tick; each tick the highest are sent until the byte budget is spent,
/// and a sent entity's priority resets. Nothing starves: an entity left out
/// keeps growing until it wins.
#[derive(Clone, Debug, Default)]
pub struct PriorityAccumulator {
    items: BTreeMap<FormId, Item>,
}

impl PriorityAccumulator {
    /// Empty accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks `entity` as having news worth `weight` priority per tick
    /// (nearer or faster entities get larger weights). Re-marking updates the
    /// weight and keeps the accumulated priority.
    pub fn touch(&mut self, entity: FormId, weight: f32) {
        let w = weight.max(0.001);
        self.items
            .entry(entity)
            .and_modify(|i| i.weight = w)
            .or_insert(Item {
                weight: w,
                acc: 0.0,
            });
    }

    /// Stops tracking an entity (despawn, left interest).
    pub fn remove(&mut self, entity: FormId) {
        self.items.remove(&entity);
    }

    /// Entities waiting to be sent.
    pub fn pending(&self) -> usize {
        self.items.len()
    }

    /// Runs one tick: grows priorities, then returns the entities to send
    /// within `budget` bytes, highest priority first. `size_of` gives the
    /// encoded size of an entity's update. The first pick is always allowed so
    /// an oversized update cannot block forever.
    pub fn drain(
        &mut self,
        budget: usize,
        mut size_of: impl FnMut(FormId) -> usize,
    ) -> Vec<FormId> {
        for i in self.items.values_mut() {
            i.acc += i.weight;
        }
        let mut order: Vec<(FormId, f32)> = self.items.iter().map(|(f, i)| (*f, i.acc)).collect();
        order.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0.cmp(&b.0))
        });
        let mut spent = 0usize;
        let mut sent = Vec::new();
        for (f, _) in order {
            let sz = size_of(f);
            if !sent.is_empty() && spent + sz > budget {
                continue;
            }
            spent += sz;
            sent.push(f);
            self.items.remove(&f);
            if spent >= budget {
                break;
            }
        }
        sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn p(n: u8) -> PlayerId {
        PlayerId(n)
    }

    #[test]
    fn same_cell_and_neighbours_only() {
        let mut m = InterestMap::new();
        m.set_peer_cell(p(0), CellId(1));
        m.set_peer_cell(p(1), CellId(1));
        m.set_peer_cell(p(2), CellId(2));
        m.set_peer_cell(p(3), CellId(3));
        m.link(CellId(2), CellId(1));
        assert_eq!(m.peers_seeing(CellId(1)), vec![p(0), p(1), p(2)]);
        assert_eq!(m.peers_seeing(CellId(3)), vec![p(3)]);
        assert_eq!(m.peers_seeing(CellId(2)), vec![p(0), p(1), p(2)]);
        assert_eq!(m.peers_seeing_except(CellId(1), p(0)), vec![p(1), p(2)]);
        assert!(m.peers_seeing(CellId(99)).is_empty());
    }

    #[test]
    fn audiences_resolve_against_members() {
        let mut m = InterestMap::new();
        m.set_peer_cell(p(0), CellId(1));
        m.set_peer_cell(p(1), CellId(1));
        m.set_peer_cell(p(2), CellId(2));
        let members = [p(0), p(1), p(2)];
        assert_eq!(
            m.resolve(Audience::Cell(CellId(1)), &members),
            vec![p(0), p(1)]
        );
        assert_eq!(
            m.resolve(Audience::CellExcept(CellId(1), p(0)), &members),
            vec![p(1)]
        );
        assert_eq!(m.resolve(Audience::To(p(2)), &members), vec![p(2)]);
        assert!(m.resolve(Audience::To(p(9)), &members).is_empty());
        assert_eq!(
            m.resolve(Audience::AllExcept(p(1)), &members),
            vec![p(0), p(2)]
        );
        assert_eq!(m.resolve(Audience::All, &members).len(), 3);
        assert_eq!(m.resolve(Audience::Cell(CellId(1)), &[p(1)]), vec![p(1)]);
    }

    #[test]
    fn moving_cells_changes_audience() {
        let mut m = InterestMap::new();
        m.set_peer_cell(p(1), CellId(1));
        assert_eq!(m.set_peer_cell(p(1), CellId(2)), Some(CellId(1)));
        assert!(m.peers_seeing(CellId(1)).is_empty());
        m.remove_peer(p(1));
        assert!(m.all_peers().is_empty());
    }

    #[test]
    fn budget_is_respected_and_priority_wins() {
        let mut a = PriorityAccumulator::new();
        a.touch(FormId(1), 1.0);
        a.touch(FormId(2), 5.0);
        a.touch(FormId(3), 2.0);
        let sent = a.drain(200, |_| 100);
        assert_eq!(sent, vec![FormId(2), FormId(3)]);
        assert_eq!(a.pending(), 1);
    }

    #[test]
    fn oversized_item_still_goes_alone() {
        let mut a = PriorityAccumulator::new();
        a.touch(FormId(1), 1.0);
        assert_eq!(a.drain(10, |_| 500), vec![FormId(1)]);
        assert_eq!(a.pending(), 0);
    }

    #[test]
    fn low_weight_entity_is_not_starved() {
        let mut a = PriorityAccumulator::new();
        let mut got_low = None;
        for tick in 0..100 {
            // one busy high-priority entity every tick, room for exactly one
            a.touch(FormId(1000), 10.0);
            a.touch(FormId(1), 1.0);
            let sent = a.drain(100, |_| 100);
            if sent.contains(&FormId(1)) {
                got_low = Some(tick);
                break;
            }
        }
        assert!(got_low.is_some(), "entity 1 starved");
    }

    proptest! {
        #[test]
        fn drain_never_exceeds_budget_beyond_first(
            sizes in proptest::collection::vec(1usize..400, 1..30),
            budget in 1usize..1500,
        ) {
            let mut a = PriorityAccumulator::new();
            for (i, _) in sizes.iter().enumerate() {
                a.touch(FormId(i as u32), 1.0 + i as f32);
            }
            let sent = a.drain(budget, |f| sizes[f.0 as usize]);
            let total: usize = sent.iter().map(|f| sizes[f.0 as usize]).sum();
            prop_assert!(!sent.is_empty());
            prop_assert!(total <= budget || sent.len() == 1);
        }

        #[test]
        fn everything_eventually_sends(n in 1u32..40, budget in 100usize..600) {
            let mut a = PriorityAccumulator::new();
            for i in 0..n {
                a.touch(FormId(i), 1.0 + (i % 5) as f32);
            }
            let mut seen = BTreeSet::new();
            for _ in 0..(n as usize * 2 + 5) {
                seen.extend(a.drain(budget, |_| 100));
            }
            prop_assert_eq!(seen.len() as u32, n);
        }
    }
}
