//! Hierarchical state hash and desync repair.
//!
//! Level 1: one 64-bit root. Level 2: 128 shard hashes (shard = FormID mix).
//! A shard hash is the wrapping sum of per-record xxh3 hashes, so it does not
//! depend on iteration order. Revisions are deliberately NOT hashed: two
//! replicas that hold the same values are equal however they got there.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

use crate::delta::{SnapEntry, StateStore};
use crate::ids::{shard_of, FormId, Rev, SHARD_COUNT};
use crate::schema::Field;

/// Hash of one record (subject form plus field value).
pub fn record_hash(form: FormId, field: &Field) -> u64 {
    // Encoding a plain enum cannot fail; fall back to 0 defensively.
    let bytes = postcard::to_allocvec(&(form, field)).unwrap_or_default();
    xxh3_64(&bytes)
}

/// The 128 shard hashes of a store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShardHashes {
    shards: Vec<u64>,
}

impl ShardHashes {
    /// Computes the shard hashes of `store`.
    pub fn compute(store: &StateStore) -> Self {
        let mut shards = vec![0u64; SHARD_COUNT];
        for (k, e) in store.iter() {
            let s = shard_of(k.form) as usize;
            shards[s] = shards[s].wrapping_add(record_hash(k.form, &e.field));
        }
        ShardHashes { shards }
    }

    /// Wraps hashes received from a peer; `None` if the count is wrong.
    pub fn from_vec(shards: Vec<u64>) -> Option<Self> {
        (shards.len() == SHARD_COUNT).then_some(ShardHashes { shards })
    }

    /// The raw shard hashes.
    pub fn shards(&self) -> &[u64] {
        &self.shards
    }

    /// Root hash over all shard hashes.
    pub fn root(&self) -> u64 {
        let mut buf = Vec::with_capacity(SHARD_COUNT * 8);
        for s in &self.shards {
            buf.extend_from_slice(&s.to_le_bytes());
        }
        xxh3_64(&buf)
    }

    /// Compares against `other` and lists the shards that differ.
    pub fn diff(&self, other: &ShardHashes) -> DesyncReport {
        let differing = (0..SHARD_COUNT)
            .filter(|&i| self.shards[i] != other.shards[i])
            .map(|i| i as u8)
            .collect();
        DesyncReport { differing }
    }
}

/// Which shards disagree between two replicas.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DesyncReport {
    /// Differing shard indices, ascending.
    pub differing: Vec<u8>,
}

impl DesyncReport {
    /// True when nothing differs.
    pub fn is_clean(&self) -> bool {
        self.differing.is_empty()
    }
}

/// A repair message: authoritative content of only the differing shards.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repair {
    /// Shards covered.
    pub shards: Vec<u8>,
    /// Host revision when the repair was built.
    pub rev: Rev,
    /// Full content of those shards.
    pub entries: Vec<SnapEntry>,
}

impl Repair {
    /// Builds the repair for `report` from the authoritative store.
    pub fn build(store: &StateStore, report: &DesyncReport) -> Repair {
        Repair {
            shards: report.differing.clone(),
            rev: store.rev(),
            entries: store.entries_in_shards(&report.differing),
        }
    }

    /// Applies the repair to a replica.
    pub fn apply(self, store: &mut StateStore) {
        store.replace_shards(&self.shards, self.entries);
    }
}

/// Host-side memory of recent shard hashes, so a guest's report taken at an
/// older revision can still be compared.
#[derive(Debug)]
pub struct HashHistory {
    cap: usize,
    items: VecDeque<(Rev, ShardHashes)>,
}

impl HashHistory {
    /// Remembers the last `cap` hash points.
    pub fn new(cap: usize) -> Self {
        HashHistory {
            cap: cap.max(1),
            items: VecDeque::new(),
        }
    }

    /// Records (or replaces) the hashes at `rev`.
    pub fn record(&mut self, rev: Rev, h: ShardHashes) {
        self.items.retain(|(r, _)| *r != rev);
        self.items.push_back((rev, h));
        while self.items.len() > self.cap {
            self.items.pop_front();
        }
    }

    /// Looks up the hashes recorded at `rev`.
    pub fn get(&self, rev: Rev) -> Option<&ShardHashes> {
        self.items.iter().find(|(r, _)| *r == rev).map(|(_, h)| h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn filled(n: u32) -> StateStore {
        let mut s = StateStore::new();
        for i in 0..n {
            s.write_local(
                FormId(0x0200_0000 + i),
                Field::QuestStage { stage: i as u16 },
            );
        }
        s
    }

    #[test]
    fn equal_stores_equal_hashes_regardless_of_revs() {
        let a = filled(300);
        // same content, different revision history
        let mut b = StateStore::new();
        for i in (0..300u32).rev() {
            b.write_local(FormId(0x0200_0000 + i), Field::QuestStage { stage: 0 });
            b.write_local(
                FormId(0x0200_0000 + i),
                Field::QuestStage { stage: i as u16 },
            );
        }
        assert_ne!(a.rev(), b.rev());
        let (ha, hb) = (ShardHashes::compute(&a), ShardHashes::compute(&b));
        assert_eq!(ha, hb);
        assert_eq!(ha.root(), hb.root());
        assert!(ha.diff(&hb).is_clean());
    }

    #[test]
    fn single_change_flags_exactly_one_shard() {
        let a = filled(300);
        let mut b = a.clone();
        let f = FormId(0x0200_0042);
        b.write_local(f, Field::QuestStage { stage: 9999 });
        let (ha, hb) = (ShardHashes::compute(&a), ShardHashes::compute(&b));
        assert_ne!(ha.root(), hb.root());
        let rep = ha.diff(&hb);
        assert_eq!(rep.differing, vec![shard_of(f)]);
    }

    #[test]
    fn repair_touches_only_differing_shards_and_fixes_them() {
        let host = filled(1000);
        let mut guest = host.clone();
        // damage three records in (likely) different shards, plus an extra key
        let mut bad = vec![];
        for i in [3u32, 400, 777] {
            let f = FormId(0x0200_0000 + i);
            guest.replace_shards(&[shard_of(f)], vec![]);
            bad.push(shard_of(f));
        }
        guest.write_local(FormId(0x0300_0001), Field::Karma { value: 5 });
        let rep = ShardHashes::compute(&host).diff(&ShardHashes::compute(&guest));
        assert!(!rep.is_clean());
        let repair = Repair::build(&host, &rep);
        assert!(
            repair.entries.len() < host.len() / 2,
            "repair must be partial"
        );
        assert!(repair.shards.len() <= 4 + bad.len());
        repair.apply(&mut guest);
        assert_eq!(ShardHashes::compute(&host), ShardHashes::compute(&guest));
    }

    #[test]
    fn history_lookup() {
        let mut h = HashHistory::new(2);
        h.record(Rev(1), ShardHashes::compute(&filled(1)));
        h.record(Rev(2), ShardHashes::compute(&filled(2)));
        h.record(Rev(3), ShardHashes::compute(&filled(3)));
        assert!(h.get(Rev(1)).is_none());
        assert!(h.get(Rev(3)).is_some());
    }

    #[test]
    fn from_vec_checks_length() {
        assert!(ShardHashes::from_vec(vec![0; 5]).is_none());
        assert!(ShardHashes::from_vec(vec![0; SHARD_COUNT]).is_some());
    }

    proptest! {
        #[test]
        fn hash_is_order_independent(vals in proptest::collection::vec((0u32..200, 0u16..50), 1..80), seed in any::<u64>()) {
            let mut a = StateStore::new();
            for (f, v) in &vals { a.write_local(FormId(*f), Field::QuestStage { stage: *v }); }
            // replay only the final value per form, in a different order
            let mut last = std::collections::BTreeMap::new();
            for (f, v) in &vals { last.insert(*f, *v); }
            let mut items: Vec<_> = last.into_iter().collect();
            let mut s = seed | 1;
            for i in (1..items.len()).rev() {
                s ^= s << 13; s ^= s >> 7; s ^= s << 17;
                items.swap(i, (s % (i as u64 + 1)) as usize);
            }
            let mut b = StateStore::new();
            for (f, v) in items { b.write_local(FormId(f), Field::QuestStage { stage: v }); }
            prop_assert_eq!(ShardHashes::compute(&a).root(), ShardHashes::compute(&b).root());
        }
    }
}
