//! Shared map markers and pings, held by the host.

use std::collections::{BTreeMap, VecDeque};

use super::{Marker, SocialMsg, MAX_LABEL_BYTES};
use crate::ids::PlayerId;
use crate::interest::Audience;

/// Markers one player may own.
pub const MAX_PER_PLAYER: usize = 8;
/// Marker operations allowed per window.
pub const RATE_OPS: usize = 12;
/// Rate window.
pub const RATE_WINDOW_MS: u64 = 10_000;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkerOut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: SocialMsg,
}

#[derive(Clone, Debug, Default)]
struct Slot {
    marker: Option<Marker>,
    expires_ms: Option<u64>,
}

/// Host marker table.
#[derive(Clone, Debug, Default)]
pub struct MarkerTable {
    owners: BTreeMap<PlayerId, BTreeMap<u16, Slot>>,
    recent: BTreeMap<PlayerId, VecDeque<u64>>,
}

impl MarkerTable {
    /// Empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Markers owned by `p`.
    pub fn of(&self, p: PlayerId) -> Vec<&Marker> {
        self.owners
            .get(&p)
            .map(|m| m.values().filter_map(|s| s.marker.as_ref()).collect())
            .unwrap_or_default()
    }

    /// Total markers.
    pub fn len(&self) -> usize {
        self.owners.values().map(|m| m.len()).sum()
    }

    /// True when there are none.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn limited(&mut self, p: PlayerId, now_ms: u64) -> bool {
        let q = self.recent.entry(p).or_default();
        while q
            .front()
            .map_or(false, |t| now_ms.saturating_sub(*t) >= RATE_WINDOW_MS)
        {
            q.pop_front();
        }
        if q.len() >= RATE_OPS {
            return true;
        }
        q.push_back(now_ms);
        false
    }

    /// A player sets a marker.
    pub fn set(&mut self, owner: PlayerId, mut marker: Marker, now_ms: u64) -> Vec<MarkerOut> {
        if marker.label.len() > MAX_LABEL_BYTES {
            let mut cut = MAX_LABEL_BYTES;
            while !marker.label.is_char_boundary(cut) {
                cut -= 1;
            }
            marker.label.truncate(cut);
        }
        marker.label = super::sanitize(&marker.label);
        if self.limited(owner, now_ms) {
            return vec![];
        }
        let slots = self.owners.entry(owner).or_default();
        if !slots.contains_key(&marker.id) && slots.len() >= MAX_PER_PLAYER {
            return vec![];
        }
        let expires_ms = (marker.ttl_ms > 0).then(|| now_ms + marker.ttl_ms as u64);
        slots.insert(
            marker.id,
            Slot {
                marker: Some(marker.clone()),
                expires_ms,
            },
        );
        vec![MarkerOut {
            aud: Audience::AllExcept(owner),
            msg: SocialMsg::MarkerShown { owner, marker },
        }]
    }

    /// A player clears a marker.
    pub fn clear(&mut self, owner: PlayerId, id: u16, now_ms: u64) -> Vec<MarkerOut> {
        if self.limited(owner, now_ms) {
            return vec![];
        }
        let removed = self
            .owners
            .get_mut(&owner)
            .and_then(|m| m.remove(&id))
            .is_some();
        if removed {
            vec![MarkerOut {
                aud: Audience::AllExcept(owner),
                msg: SocialMsg::MarkerGone { owner, id },
            }]
        } else {
            vec![]
        }
    }

    /// Expires pings.
    pub fn tick(&mut self, now_ms: u64) -> Vec<MarkerOut> {
        let mut v = Vec::new();
        for (owner, slots) in self.owners.iter_mut() {
            let gone: Vec<u16> = slots
                .iter()
                .filter(|(_, s)| s.expires_ms.map_or(false, |e| now_ms >= e))
                .map(|(i, _)| *i)
                .collect();
            for id in gone {
                slots.remove(&id);
                v.push(MarkerOut {
                    aud: Audience::All,
                    msg: SocialMsg::MarkerGone { owner: *owner, id },
                });
            }
        }
        v
    }

    /// A player left: their markers vanish.
    pub fn remove_player(&mut self, p: PlayerId) -> Vec<MarkerOut> {
        self.recent.remove(&p);
        self.owners
            .remove(&p)
            .map(|m| {
                m.keys()
                    .map(|id| MarkerOut {
                        aud: Audience::All,
                        msg: SocialMsg::MarkerGone { owner: p, id: *id },
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Everything a joining peer needs to see.
    pub fn snapshot(&self) -> Vec<SocialMsg> {
        self.owners
            .iter()
            .flat_map(|(o, m)| {
                m.values()
                    .filter_map(|s| s.marker.clone())
                    .map(|marker| SocialMsg::MarkerShown { owner: *o, marker })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::CellId;

    const A: PlayerId = PlayerId(1);
    const B: PlayerId = PlayerId(2);

    fn mk(id: u16, ttl: u32) -> Marker {
        Marker {
            id,
            kind: 1,
            cell: CellId(1),
            pos: [0; 3],
            label: "camp".into(),
            ttl_ms: ttl,
        }
    }

    #[test]
    fn set_relays_to_others_and_replaces_by_id() {
        let mut t = MarkerTable::new();
        let o = t.set(A, mk(1, 0), 0);
        assert_eq!(o[0].aud, Audience::AllExcept(A));
        let mut m = mk(1, 0);
        m.label = "base".into();
        t.set(A, m, 1);
        assert_eq!(t.of(A).len(), 1);
        assert_eq!(t.of(A)[0].label, "base");
    }

    #[test]
    fn per_player_cap_and_rate() {
        let mut t = MarkerTable::new();
        for i in 0..MAX_PER_PLAYER as u16 {
            assert_eq!(t.set(A, mk(i, 0), 0).len(), 1);
        }
        assert!(t.set(A, mk(99, 0), 1).is_empty(), "ninth marker refused");
        assert_eq!(t.set(A, mk(0, 0), 1).len(), 1, "replacing is fine");
        for i in 0..20u16 {
            t.set(B, mk(i % 8, 0), 2);
        }
        assert!(t.set(B, mk(1, 0), 3).is_empty(), "rate limited");
        assert_eq!(t.set(B, mk(1, 0), RATE_WINDOW_MS + 3).len(), 1);
    }

    #[test]
    fn pings_expire() {
        let mut t = MarkerTable::new();
        t.set(A, mk(1, 5000), 100);
        assert!(t.tick(5099).is_empty());
        let o = t.tick(5100);
        assert!(matches!(o[0].msg, SocialMsg::MarkerGone { id: 1, .. }));
        assert!(t.is_empty());
    }

    #[test]
    fn clear_and_disconnect() {
        let mut t = MarkerTable::new();
        t.set(A, mk(1, 0), 0);
        t.set(A, mk(2, 0), 0);
        assert!(t.clear(B, 1, 0).is_empty(), "not yours");
        assert_eq!(t.clear(A, 1, 1).len(), 1);
        assert!(t.clear(A, 1, 2).is_empty());
        assert_eq!(t.remove_player(A).len(), 1);
        assert!(t.is_empty());
    }

    #[test]
    fn labels_are_sanitized_and_cut_and_snapshot_lists_all() {
        let mut t = MarkerTable::new();
        let mut m = mk(1, 0);
        m.label = format!("<{}>", "x".repeat(100));
        t.set(A, m, 0);
        let l = &t.of(A)[0].label;
        assert!(l.len() <= MAX_LABEL_BYTES && l.starts_with('?'));
        t.set(B, mk(1, 0), 0);
        assert_eq!(t.snapshot().len(), 2);
    }
}
