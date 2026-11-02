//! Havok's simulation islands (`hkpSimulationIsland`, Xbox PDB; B1 PR 5,
//! docs/PHYSICS.md "Simulation islands"): the bodies Havok steps, sleeps
//! and wakes together. Translated from FalloutNV.exe 1.4.0.525; names are
//! the Xbox prototype's (ADR-0002), addresses the PC's (ADR-0003).
//!
//! The rules, as the game's Havok has them (`iSimType` 1, so the world's
//! `m_minDesiredIslandSize` is 0 and no island is "sparse"):
//!
//! - A body is added in an island of its own, active or not
//!   (`hkpWorld::addEntity` `00c914d0`, `addEntityBatch` `00c94bd0`).
//! - Two islands merge when the broadphase pairs two of their bodies and
//!   an agent is made for the pair (`hkpWorldAgentUtil::addAgent`
//!   `00cc0f40` → `hkpWorldOperationUtil::mergeIslands` `00cb5570` →
//!   `internalMergeTwoIslands` `00cb4c60`), unless either body is fixed.
//!   Merging an active island with an inactive one first activates the
//!   inactive one (`00cb5100`): this is how a moving body wakes a sleeping
//!   one, by their bounding boxes meeting.
//! - When an agent goes (the pair's boxes part, `removeAgent` `00cc10b0`)
//!   and both bodies are in one island, the island asks for a split check;
//!   the world's maintenance (`hkpDefaultWorldMaintenanceMgr::
//!   performMaintenance` `00d0b280` → `splitSimulationIslands` `00cb6f30`
//!   → `00cb6e70` → `00cb6060`) splits it into its connected parts (bodies
//!   joined by agents with a non-fixed partner, by constraints and by
//!   actions; `00d074c0`) at the next step's start.
//! - Sleeping is per island: an island whose fewest passing deactivation
//!   checks is over 5 is marked inactive (`markIslandInactive` `00cb5420`:
//!   its active mark cleared, put on the world's dirty list); at the next
//!   step's start the dirty islands are cleaned up (`cleanupDirtyIslands`
//!   `00cb55d0`): one marked inactive goes to sleep when every body passes
//!   the last test (`00cb5310`), else its mark is set back; one marked
//!   active is woken (`00cb5100`).

/// An island (`hkpSimulationIsland`, 0x6c bytes).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Island {
    /// `m_entities` (`+0x48`).
    pub bodies: Vec<usize>,
    /// `m_isInActiveIslandsArray` (`+0x26`, bits 0–1 on the PC): it is
    /// stepped.
    pub active: bool,
    /// `m_activeMark` (`+0x26`, bits 2–3): whether it should be.
    pub active_mark: bool,
    /// `m_splitCheckRequested` (`+0x25`, bits 0–1).
    pub split_check_requested: bool,
    /// On the world's dirty list (`m_dirtyListIndex` `+0x22` not 0xffff).
    dirty: bool,
}

/// What a cleanup of the dirty islands did.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cleanup {
    /// Bodies of the islands put to sleep (their velocities are to be
    /// zeroed, `00cb5310`).
    pub deactivated: Vec<usize>,
    /// Bodies of the islands woken (their deactivation counts start
    /// again, `00cb5100`).
    pub activated: Vec<usize>,
}

/// The world's islands: each non-fixed body in exactly one.
#[derive(Debug, Clone, Default)]
pub struct Islands {
    islands: Vec<Option<Island>>,
    /// Each body's island; `None` for a fixed body (Havok's fixed island).
    island_of: Vec<Option<usize>>,
    /// `hkpWorld::m_dirtySimulationIslands` (`+0x40`).
    dirty: Vec<usize>,
}

impl Islands {
    pub fn new() -> Self {
        Self::default()
    }

    /// The island of body `body` (`hkpEntity::m_simulationIsland`, `+0xcc`).
    pub fn island_of(&self, body: usize) -> Option<usize> {
        self.island_of.get(body).copied().flatten()
    }

    pub fn island(&self, id: usize) -> Option<&Island> {
        self.islands.get(id).and_then(|i| i.as_ref())
    }

    /// Whether body `body`'s island is stepped.
    pub fn is_active(&self, body: usize) -> bool {
        self.island_of(body)
            .and_then(|i| self.island(i))
            .is_some_and(|i| i.active)
    }

    /// The ids of the active islands (`hkpWorld::m_activeSimulationIslands`).
    pub fn active(&self) -> Vec<usize> {
        (0..self.islands.len())
            .filter(|&i| self.island(i).is_some_and(|i| i.active))
            .collect()
    }

    /// Every island's id.
    pub fn all(&self) -> Vec<usize> {
        (0..self.islands.len())
            .filter(|&i| self.island(i).is_some())
            .collect()
    }

    fn new_island(&mut self, island: Island) -> usize {
        if let Some(i) = self.islands.iter().position(|i| i.is_none()) {
            self.islands[i] = Some(island);
            i
        } else {
            self.islands.push(Some(island));
            self.islands.len() - 1
        }
    }

    fn get_mut(&mut self, id: usize) -> &mut Island {
        self.islands[id].as_mut().expect("a live island")
    }

    /// Adds the next body: a fixed one in no island, any other in a new
    /// island of its own, active or not (`hkpWorld::addEntity` with its
    /// activation). Gives the body's index.
    pub fn add_body(&mut self, fixed: bool, active: bool) -> usize {
        let island = (!fixed).then(|| {
            let body = self.island_of.len();
            self.new_island(Island {
                bodies: vec![body],
                active,
                active_mark: active,
                ..Island::default()
            })
        });
        self.island_of.push(island);
        self.island_of.len() - 1
    }

    /// Takes body `body` out (later bodies' indices go down by one). Its
    /// island, if others remain in it, asks for a split check (its agents
    /// and constraints went with it, `00cc10b0`).
    pub fn remove_body(&mut self, body: usize) {
        if let Some(id) = self.island_of(body) {
            let island = self.get_mut(id);
            island.bodies.retain(|&b| b != body);
            if island.bodies.is_empty() {
                self.islands[id] = None;
                self.dirty.retain(|&d| d != id);
            } else {
                island.split_check_requested = true;
            }
        }
        self.island_of.remove(body);
        for island in self.islands.iter_mut().flatten() {
            for b in &mut island.bodies {
                if *b > body {
                    *b -= 1;
                }
            }
        }
    }

    fn put_on_dirty_list(&mut self, id: usize) {
        let island = self.get_mut(id);
        if !island.dirty {
            island.dirty = true;
            self.dirty.push(id);
        }
    }

    /// `hkpWorldOperationUtil::markIslandInactive` (`00cb5420`): the active
    /// mark cleared, the island on the dirty list.
    // Translated from 00cb5420 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn mark_inactive(&mut self, id: usize) {
        self.get_mut(id).active_mark = false;
        self.put_on_dirty_list(id);
    }

    /// `hkpWorldOperationUtil::markIslandActive` (Xbox PDB; what
    /// `hkpEntity::activate` does to a body's island): the mark set, the
    /// island on the dirty list (an island marked inactive and not yet
    /// asleep so stays awake).
    pub fn mark_active(&mut self, id: usize) {
        self.get_mut(id).active_mark = true;
        self.put_on_dirty_list(id);
    }

    /// Wakes an island at once (`internalActivateIsland` `00cb5100`):
    /// into the active array, marked active; gives its bodies.
    // Translated from 00cb5100 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn activate(&mut self, id: usize) -> Vec<usize> {
        let island = self.get_mut(id);
        island.active = true;
        island.active_mark = true;
        island.bodies.clone()
    }

    /// `hkpWorldOperationUtil::mergeIslands` (`00cb5570` →
    /// `internalMergeTwoIslands` `00cb4c60`) for a new agent between bodies
    /// `a` and `b`: nothing when either is fixed or they share an island.
    /// The island with more bodies is kept (on a tie, the one stored
    /// first); when either island is active the inactive one is activated
    /// first (`00cb5100`); the kept island's active mark and split request
    /// are the two's together, and it is on the dirty list when either
    /// was. Gives the bodies activated by the merge.
    // Translated from 00cb4c60 and 00cc0f40 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn merge(&mut self, a: usize, b: usize) -> Vec<usize> {
        let (Some(ia), Some(ib)) = (self.island_of(a), self.island_of(b)) else {
            return Vec::new();
        };
        if ia == ib {
            return Vec::new();
        }
        // `00cb4c60`: the second argument (b's island) is kept unless the
        // first has more bodies, or as many and is stored before it.
        let (na, nb) = (
            self.island(ia).map_or(0, |i| i.bodies.len()),
            self.island(ib).map_or(0, |i| i.bodies.len()),
        );
        let (keep, gone) = if na > nb || (na == nb && ia < ib) {
            (ia, ib)
        } else {
            (ib, ia)
        };
        let mut woken = Vec::new();
        let (keep_active, gone_active) = (
            self.island(keep).is_some_and(|i| i.active),
            self.island(gone).is_some_and(|i| i.active),
        );
        // The marks as they were before the activation below.
        let mark = self.island(keep).is_some_and(|i| i.active_mark)
            || self.island(gone).is_some_and(|i| i.active_mark);
        if keep_active != gone_active {
            let sleeper = if keep_active { gone } else { keep };
            woken = self.activate(sleeper);
        }
        let gone_island = self.islands[gone].take().expect("a live island");
        self.dirty.retain(|&d| d != gone);
        for &body in &gone_island.bodies {
            self.island_of[body] = Some(keep);
        }
        let kept = self.get_mut(keep);
        kept.bodies.extend(gone_island.bodies);
        kept.active_mark = mark;
        kept.split_check_requested |= gone_island.split_check_requested;
        if gone_island.dirty {
            self.put_on_dirty_list(keep);
        }
        woken
    }

    /// `hkpWorldAgentUtil::removeAgent` (`00cc10b0`) for an agent between
    /// bodies `a` and `b` in one island: the island asks for a split check.
    // Translated from 00cc10b0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn agent_removed(&mut self, a: usize, b: usize) {
        if let (Some(ia), Some(ib)) = (self.island_of(a), self.island_of(b)) {
            if ia == ib {
                self.get_mut(ia).split_check_requested = true;
            }
        }
    }

    /// `splitSimulationIslands` (`00cb6f30`, from the world's maintenance
    /// at each step's start): every island that asked is split into its
    /// connected parts; `joined(a, b)` says whether two of its bodies are
    /// joined directly (an agent between them, a constraint or an action;
    /// fixed bodies join nothing). The first part keeps the island; the
    /// others get new islands, active or not as it was, and marked
    /// inactive (on the dirty list) when it was active but marked inactive
    /// (`00cb6e70`). Gives the new islands' ids.
    // Translated from 00cb6f30, 00cb6e70, 00cb6060 and 00d074c0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn split(&mut self, mut joined: impl FnMut(usize, usize) -> bool) -> Vec<usize> {
        let mut made = Vec::new();
        for id in 0..self.islands.len() {
            let Some(island) = self.island(id) else {
                continue;
            };
            if !island.split_check_requested {
                continue;
            }
            let bodies = island.bodies.clone();
            let (active, mark) = (island.active, island.active_mark);
            self.get_mut(id).split_check_requested = false;
            // Union–find over the island's bodies.
            let n = bodies.len();
            let mut root: Vec<usize> = (0..n).collect();
            fn find(root: &mut [usize], mut i: usize) -> usize {
                while root[i] != i {
                    root[i] = root[root[i]];
                    i = root[i];
                }
                i
            }
            for x in 0..n {
                for y in x + 1..n {
                    if joined(bodies[x], bodies[y]) {
                        let (rx, ry) = (find(&mut root, x), find(&mut root, y));
                        if rx != ry {
                            root[ry] = rx;
                        }
                    }
                }
            }
            let mut groups: Vec<(usize, Vec<usize>)> = Vec::new();
            for (k, &body) in bodies.iter().enumerate() {
                let r = find(&mut root, k);
                match groups.iter_mut().find(|(g, _)| *g == r) {
                    Some((_, members)) => members.push(body),
                    None => groups.push((r, vec![body])),
                }
            }
            if groups.len() < 2 {
                continue;
            }
            let mut groups = groups.into_iter().map(|(_, m)| m);
            let first = groups.next().expect("a group");
            self.get_mut(id).bodies = first;
            for members in groups {
                let new = self.new_island(Island {
                    bodies: members.clone(),
                    active,
                    active_mark: active,
                    ..Island::default()
                });
                for &body in &members {
                    self.island_of[body] = Some(new);
                }
                if active && !mark {
                    self.mark_inactive(new);
                }
                made.push(new);
            }
        }
        made
    }

    /// `hkpWorldOperationUtil::cleanupDirtyIslands` (`00cb55d0`), at the
    /// start of a step's integration: each dirty island, last to first,
    /// whose mark differs from its state: marked inactive, it goes to
    /// sleep when `can_sleep` passes for its bodies (`00cb5310` with
    /// `00d28560`), else its mark is set back; marked active, it wakes
    /// (`00cb5100`).
    // Translated from 00cb55d0, 00cb5310 and 00cb5100 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn cleanup(&mut self, mut can_sleep: impl FnMut(&[usize]) -> bool) -> Cleanup {
        let mut out = Cleanup::default();
        while let Some(id) = self.dirty.pop() {
            let Some(island) = self.islands[id].as_mut() else {
                continue;
            };
            island.dirty = false;
            if island.active_mark == island.active {
                continue;
            }
            if !island.active_mark {
                if can_sleep(&island.bodies) {
                    island.active = false;
                    out.deactivated.extend(island.bodies.iter().copied());
                } else {
                    island.active_mark = true;
                }
            } else {
                out.activated.extend(self.activate(id));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bodies_start_alone_and_merge_on_a_pair() {
        let mut s = Islands::new();
        let a = s.add_body(false, false);
        let b = s.add_body(false, false);
        let ground = s.add_body(true, false);
        assert_ne!(s.island_of(a), s.island_of(b));
        assert_eq!(s.island_of(ground), None);
        // Two sleeping islands merge asleep; a fixed partner joins nothing.
        assert!(s.merge(a, b).is_empty());
        assert_eq!(s.island_of(a), s.island_of(b));
        assert!(!s.is_active(a));
        assert!(s.merge(a, ground).is_empty());
        assert_eq!(s.all().len(), 1);
    }

    #[test]
    fn an_active_island_wakes_the_one_it_merges_with() {
        let mut s = Islands::new();
        let a = s.add_body(false, true);
        let b = s.add_body(false, false);
        let c = s.add_body(false, false);
        s.merge(b, c);
        // `a` pairs with `b`: b's island (b and c) is activated first.
        let mut woken = s.merge(a, b);
        woken.sort();
        assert_eq!(woken, vec![b, c]);
        assert!(s.is_active(a) && s.is_active(b) && s.is_active(c));
        // The island with more bodies is kept.
        assert_eq!(s.island(s.island_of(a).unwrap()).unwrap().bodies.len(), 3);
    }

    #[test]
    fn a_removed_agent_splits_the_island_at_the_next_check() {
        let mut s = Islands::new();
        let a = s.add_body(false, true);
        let b = s.add_body(false, true);
        let c = s.add_body(false, true);
        s.merge(a, b);
        s.merge(b, c);
        // The agent between b and c goes: a split is asked for, done when
        // the maintenance runs with what still joins them (a–b only).
        s.agent_removed(b, c);
        let made = s.split(|x, y| (x, y) == (a, b) || (x, y) == (b, a));
        assert_eq!(made.len(), 1);
        assert_eq!(s.island_of(a), s.island_of(b));
        assert_ne!(s.island_of(a), s.island_of(c));
        assert!(s.is_active(c));
        // Nothing asked: nothing split.
        assert!(s.split(|_, _| false).is_empty());
    }

    #[test]
    fn islands_sleep_and_wake_through_the_dirty_list() {
        let mut s = Islands::new();
        let a = s.add_body(false, true);
        let b = s.add_body(false, true);
        s.merge(a, b);
        let id = s.island_of(a).unwrap();
        // Marked inactive; the last test fails: it stays awake, marked.
        s.mark_inactive(id);
        let c = s.cleanup(|_| false);
        assert!(c.deactivated.is_empty() && s.is_active(a));
        assert!(s.island(id).unwrap().active_mark);
        // Marked again and passing: both asleep together.
        s.mark_inactive(id);
        let c = s.cleanup(|bodies| bodies.len() == 2);
        assert_eq!(c.deactivated.len(), 2);
        assert!(!s.is_active(a) && !s.is_active(b));
        // Woken by a mark.
        s.mark_active(id);
        let c = s.cleanup(|_| true);
        assert_eq!(c.activated.len(), 2);
        assert!(s.is_active(b));
        // A wake between the mark and the cleanup cancels the sleep.
        s.mark_inactive(id);
        s.mark_active(id);
        let c = s.cleanup(|_| true);
        assert!(c.deactivated.is_empty() && s.is_active(a));
    }

    #[test]
    fn removing_a_body_renumbers_and_asks_for_a_split() {
        let mut s = Islands::new();
        let a = s.add_body(false, true);
        let b = s.add_body(false, true);
        let c = s.add_body(false, true);
        s.merge(a, c);
        s.remove_body(b);
        // c is now body 1, still with a.
        assert_eq!(s.island_of(0), s.island_of(1));
        assert!(s
            .island(s.island_of(0).unwrap())
            .unwrap()
            .bodies
            .contains(&1));
        s.remove_body(0);
        assert!(
            s.island(s.island_of(0).unwrap())
                .unwrap()
                .split_check_requested
        );
    }
}
