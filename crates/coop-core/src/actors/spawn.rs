//! Spawn arbitration: the host allocates ids for dynamically created refs,
//! allows one owner per spawn site, and resolves leveled picks exactly once.

use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, FormId, PlayerId};
use crate::once::OnceMap;

/// First id of the host-allocated dynamic range.
pub const DYN_ID_BASE: u32 = 0xFF00_0000;
const REQ_MEMORY: usize = 512;

/// Why a spawn was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DenyReason {
    /// Another player already holds this site (turn your roll off).
    SiteTaken {
        /// Current holder.
        owner: PlayerId,
    },
    /// The site is cooling down until its respawn time.
    Cooling,
    /// Malformed request or id space exhausted.
    BadRequest,
}

/// Verdict on a spawn request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawnOutcome {
    /// Create the ref under this host-allocated id.
    Granted {
        /// The id.
        id: FormId,
    },
    /// Do not spawn.
    Denied(DenyReason),
}

#[derive(Clone, Copy, Debug)]
enum Site {
    Active { owner: PlayerId, id: FormId },
    Cooling { until_ms: u64 },
}

/// A live dynamic ref.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpawnRec {
    /// Base form.
    pub base: FormId,
    /// Site that produced it.
    pub site: FormId,
    /// Cell.
    pub cell: CellId,
    /// Position, 1/16 unit fixed point.
    pub pos: [i32; 3],
}

/// Host-side arbiter.
#[derive(Clone, Debug)]
pub struct SpawnArbiter {
    next: u32,
    sites: BTreeMap<FormId, Site>,
    live: BTreeMap<FormId, SpawnRec>,
    answered: BTreeMap<(PlayerId, u32), SpawnOutcome>,
    answered_order: VecDeque<(PlayerId, u32)>,
    leveled: OnceMap<(FormId, u32), FormId>,
}

impl Default for SpawnArbiter {
    fn default() -> Self {
        SpawnArbiter {
            next: DYN_ID_BASE,
            sites: BTreeMap::new(),
            live: BTreeMap::new(),
            answered: BTreeMap::new(),
            answered_order: VecDeque::new(),
            leveled: OnceMap::new(),
        }
    }
}

impl SpawnArbiter {
    /// New arbiter at the start of the dynamic id range.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles a spawn request. A repeated `(from, req)` returns the first
    /// answer and creates nothing. Returns the outcome and, when newly
    /// granted, the record to announce.
    #[allow(clippy::too_many_arguments)]
    pub fn request(
        &mut self,
        from: PlayerId,
        req: u32,
        site: FormId,
        base: FormId,
        cell: CellId,
        pos: [i32; 3],
        now_ms: u64,
    ) -> (SpawnOutcome, Option<(FormId, SpawnRec)>) {
        if let Some(o) = self.answered.get(&(from, req)) {
            return (*o, None);
        }
        let (outcome, rec) = self.decide(from, site, base, cell, pos, now_ms);
        self.answered.insert((from, req), outcome);
        self.answered_order.push_back((from, req));
        while self.answered_order.len() > REQ_MEMORY {
            if let Some(k) = self.answered_order.pop_front() {
                self.answered.remove(&k);
            }
        }
        (outcome, rec)
    }

    fn decide(
        &mut self,
        from: PlayerId,
        site: FormId,
        base: FormId,
        cell: CellId,
        pos: [i32; 3],
        now_ms: u64,
    ) -> (SpawnOutcome, Option<(FormId, SpawnRec)>) {
        match self.sites.get(&site) {
            Some(Site::Active { owner, .. }) => {
                return (
                    SpawnOutcome::Denied(DenyReason::SiteTaken { owner: *owner }),
                    None,
                )
            }
            Some(Site::Cooling { until_ms }) if now_ms < *until_ms => {
                return (SpawnOutcome::Denied(DenyReason::Cooling), None)
            }
            _ => {}
        }
        if self.next == u32::MAX {
            return (SpawnOutcome::Denied(DenyReason::BadRequest), None);
        }
        let id = FormId(self.next);
        self.next += 1;
        let rec = SpawnRec {
            base,
            site,
            cell,
            pos,
        };
        self.sites.insert(site, Site::Active { owner: from, id });
        self.live.insert(id, rec);
        (SpawnOutcome::Granted { id }, Some((id, rec)))
    }

    /// Records a ref the host itself made under an id of its own (the
    /// world's `PlaceAtMe` counts up from the same base): the host decided
    /// it, so it is announced once. False when the id is already known.
    /// Later grants skip the ids taken.
    pub fn adopt(&mut self, id: FormId, rec: SpawnRec) -> bool {
        if self.live.contains_key(&id) {
            return false;
        }
        self.live.insert(id, rec);
        if id.0 >= self.next && id.0 < u32::MAX {
            self.next = id.0 + 1;
        }
        true
    }

    /// True while the dynamic ref exists.
    pub fn is_live(&self, id: FormId) -> bool {
        self.live.contains_key(&id)
    }

    /// Removes a dynamic ref (dead and cleaned up, or despawned) and starts
    /// its site's respawn timer. Returns true when it existed.
    pub fn despawn(&mut self, id: FormId, now_ms: u64, respawn_ms: u64) -> bool {
        let Some(rec) = self.live.remove(&id) else {
            return false;
        };
        if matches!(self.sites.get(&rec.site), Some(Site::Active { id: i, .. }) if *i == id) {
            self.sites.insert(
                rec.site,
                Site::Cooling {
                    until_ms: now_ms + respawn_ms,
                },
            );
        }
        true
    }

    /// Who may despawn `id` on a player's say-so: the holder of its site.
    pub fn site_owner_of(&self, id: FormId) -> Option<PlayerId> {
        let rec = self.live.get(&id)?;
        match self.sites.get(&rec.site) {
            Some(Site::Active { owner, id: i }) if *i == id => Some(*owner),
            _ => None,
        }
    }

    /// Live dynamic refs.
    pub fn live(&self) -> impl Iterator<Item = (&FormId, &SpawnRec)> {
        self.live.iter()
    }

    /// Resolves a leveled pick: the first proposal per `(list, key)` wins.
    pub fn leveled(&mut self, list: FormId, key: u32, pick: FormId) -> FormId {
        self.leveled.resolve((list, key), pick).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SITE: FormId = FormId(0x500);
    const BASE: FormId = FormId(0x600);
    const C: CellId = CellId(1);

    #[test]
    fn first_requester_wins_the_site() {
        let mut a = SpawnArbiter::new();
        let (o1, r1) = a.request(PlayerId(1), 1, SITE, BASE, C, [0; 3], 0);
        let (o2, r2) = a.request(PlayerId(2), 1, SITE, BASE, C, [0; 3], 0);
        assert_eq!(
            o1,
            SpawnOutcome::Granted {
                id: FormId(DYN_ID_BASE)
            }
        );
        assert!(r1.is_some() && r2.is_none());
        assert_eq!(
            o2,
            SpawnOutcome::Denied(DenyReason::SiteTaken { owner: PlayerId(1) })
        );
        assert_eq!(a.live().count(), 1);
    }

    #[test]
    fn repeated_request_is_idempotent() {
        let mut a = SpawnArbiter::new();
        let (o1, _) = a.request(PlayerId(1), 7, SITE, BASE, C, [0; 3], 0);
        let (o1b, rec) = a.request(PlayerId(1), 7, SITE, BASE, C, [0; 3], 5);
        assert_eq!(o1, o1b);
        assert!(rec.is_none());
        assert_eq!(a.live().count(), 1);
    }

    #[test]
    fn ids_are_unique_and_in_the_dynamic_range() {
        let mut a = SpawnArbiter::new();
        let mut ids = std::collections::BTreeSet::new();
        for i in 0..50u32 {
            if let (SpawnOutcome::Granted { id }, _) =
                a.request(PlayerId(1), i, FormId(i), BASE, C, [0; 3], 0)
            {
                assert!(id.0 >= DYN_ID_BASE);
                assert!(ids.insert(id));
            }
        }
        assert_eq!(ids.len(), 50);
    }

    #[test]
    fn respawn_timer_is_host_authoritative() {
        let mut a = SpawnArbiter::new();
        let id = match a.request(PlayerId(1), 1, SITE, BASE, C, [0; 3], 0).0 {
            SpawnOutcome::Granted { id } => id,
            o => panic!("{o:?}"),
        };
        assert_eq!(a.site_owner_of(id), Some(PlayerId(1)));
        assert!(a.despawn(id, 1000, 5000));
        assert!(!a.despawn(id, 1000, 5000), "second despawn is a no-op");
        let denied = a.request(PlayerId(2), 1, SITE, BASE, C, [0; 3], 5999).0;
        assert_eq!(denied, SpawnOutcome::Denied(DenyReason::Cooling));
        let granted = a.request(PlayerId(2), 2, SITE, BASE, C, [0; 3], 6000).0;
        assert!(matches!(granted, SpawnOutcome::Granted { .. }));
    }

    #[test]
    fn leveled_pick_resolves_once() {
        let mut a = SpawnArbiter::new();
        assert_eq!(a.leveled(FormId(1), 9, FormId(100)), FormId(100));
        assert_eq!(a.leveled(FormId(1), 9, FormId(200)), FormId(100));
        assert_eq!(a.leveled(FormId(1), 10, FormId(200)), FormId(200));
    }

    #[test]
    fn request_memory_is_bounded() {
        let mut a = SpawnArbiter::new();
        for i in 0..(REQ_MEMORY as u32 + 100) {
            a.request(PlayerId(1), i, FormId(i), BASE, C, [0; 3], 0);
        }
        assert!(a.answered.len() <= REQ_MEMORY);
    }
}
