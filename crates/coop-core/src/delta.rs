//! The replicated state store and its deltas.
//!
//! Model: the host assigns every accepted write the next [`Rev`]. A [`Delta`]
//! is *state based*: it carries the current value of every key whose revision
//! is in `(base_rev, rev]`. Applying is last-writer-wins on the per-key
//! revision, so it is idempotent and tolerant of duplication and reordering.
//! Because the content is derived from the store (not from an event log), a
//! lost delta is repaired by simply sending a new delta from the last acked
//! baseline.

use std::collections::BTreeMap;
use std::ops::Bound;

use serde::{Deserialize, Serialize};

use crate::ids::{shard_of, FormId, Rev, Tick};
use crate::schema::{Field, Key};

/// One change inside a delta. The per-key revision is `base_rev + rev_off`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Change {
    /// Subject form.
    pub form: FormId,
    /// New value of the slot.
    pub field: Field,
    /// Revision offset from the delta's `base_rev` (at least 1).
    pub rev_off: u64,
}

/// A batch of changes against a baseline revision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delta {
    /// Host tick at which the delta was built.
    pub tick: Tick,
    /// Baseline: contains every key changed after this revision...
    pub base_rev: Rev,
    /// ...up to and including this revision.
    pub rev: Rev,
    /// The changes, ascending by revision.
    pub changes: Vec<Change>,
    /// False when the delta was cut short by a size limit. A complete delta
    /// leaves a replica exactly equal to the sender at `rev`; a partial one
    /// leaves it a consistent prefix that is not yet comparable by hash.
    pub complete: bool,
}

/// A key's full record, used by snapshots and repairs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapEntry {
    /// Subject form.
    pub form: FormId,
    /// Value.
    pub field: Field,
    /// Per-key revision.
    pub rev: Rev,
}

/// A stored value with its revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Revision of the last accepted write.
    pub rev: Rev,
    /// Current value.
    pub field: Field,
}

/// Outcome of applying one record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyOutcome {
    /// Newer than what we had; stored.
    Applied,
    /// Older than what we had; ignored.
    Stale,
    /// Same revision as what we had; ignored.
    Duplicate,
    /// Malformed (zero revision, oversize, revision collision).
    Rejected,
}

/// Counters from [`StateStore::apply_delta`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ApplyReport {
    /// Changes stored.
    pub applied: u32,
    /// Changes ignored as older.
    pub stale: u32,
    /// Changes ignored as already present.
    pub duplicate: u32,
    /// Malformed changes.
    pub rejected: u32,
}

/// The FormID-keyed state with per-key revisions.
///
/// `rev` is the highest revision seen. `base` is the contiguous baseline: the
/// revision up to which this replica is known to hold everything. On the host
/// they are always equal.
#[derive(Clone, Debug, Default)]
pub struct StateStore {
    map: BTreeMap<Key, Entry>,
    by_rev: BTreeMap<Rev, Key>,
    rev: Rev,
    base: Rev,
    /// Revision at which the content is known to equal the sender exactly.
    exact: Option<Rev>,
}

impl StateStore {
    /// Empty store at revision zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Highest revision seen.
    pub fn rev(&self) -> Rev {
        self.rev
    }

    /// Contiguous baseline revision (everything up to here is present).
    pub fn base(&self) -> Rev {
        self.base
    }

    /// True when `base == rev` and the content is exactly the sender's state
    /// at that revision (host stores always are). Only then may state hashes
    /// be compared.
    pub fn is_exact(&self) -> bool {
        self.base == self.rev && self.exact == Some(self.rev)
    }

    /// Number of keys.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// True when no keys are stored.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Looks up a slot.
    pub fn get(&self, form: FormId, probe: &Field) -> Option<&Entry> {
        self.map.get(&probe.key(form))
    }

    /// Looks up by key.
    pub fn get_key(&self, key: &Key) -> Option<&Entry> {
        self.map.get(key)
    }

    /// Iterates entries in key order (deterministic).
    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Entry)> {
        self.map.iter()
    }

    /// Host-side write: assigns the next revision. Returns `None` (and does
    /// not bump the revision) when the value is unchanged or malformed.
    pub fn write_local(&mut self, form: FormId, field: Field) -> Option<Rev> {
        if !field.is_sane() {
            return None;
        }
        let key = field.key(form);
        if self.map.get(&key).map_or(false, |e| e.field == field) {
            return None;
        }
        let contiguous = self.base == self.rev;
        let rev = self.rev.next();
        if let Some(old) = self.map.insert(key, Entry { rev, field }) {
            self.by_rev.remove(&old.rev);
        }
        self.by_rev.insert(rev, key);
        self.rev = rev;
        if contiguous {
            self.base = rev;
            self.exact = Some(rev);
        }
        Some(rev)
    }

    /// Applies one record carrying its host revision (last writer wins).
    pub fn apply_entry(&mut self, form: FormId, field: Field, rev: Rev) -> ApplyOutcome {
        if rev == Rev::ZERO || !field.is_sane() {
            return ApplyOutcome::Rejected;
        }
        let key = field.key(form);
        if let Some(e) = self.map.get(&key) {
            if e.rev == rev {
                return ApplyOutcome::Duplicate;
            }
            if e.rev > rev {
                return ApplyOutcome::Stale;
            }
        }
        match self.by_rev.get(&rev) {
            Some(k) if *k != key => return ApplyOutcome::Rejected,
            _ => {}
        }
        if let Some(old) = self.map.insert(key, Entry { rev, field }) {
            self.by_rev.remove(&old.rev);
        }
        self.by_rev.insert(rev, key);
        if rev > self.rev {
            self.rev = rev;
        }
        ApplyOutcome::Applied
    }

    /// Applies a delta. Advances the contiguous baseline only when the delta's
    /// own baseline is already covered and nothing in it was malformed.
    pub fn apply_delta(&mut self, d: &Delta) -> ApplyReport {
        let mut r = ApplyReport::default();
        for c in &d.changes {
            let rev = Rev(d.base_rev.0.saturating_add(c.rev_off));
            if c.rev_off == 0 || rev > d.rev {
                r.rejected += 1;
                continue;
            }
            match self.apply_entry(c.form, c.field.clone(), rev) {
                ApplyOutcome::Applied => r.applied += 1,
                ApplyOutcome::Stale => r.stale += 1,
                ApplyOutcome::Duplicate => r.duplicate += 1,
                ApplyOutcome::Rejected => r.rejected += 1,
            }
        }
        if r.rejected == 0 && d.base_rev <= self.base && d.rev > self.base {
            self.base = d.rev;
            self.exact = d.complete.then_some(d.rev);
            if self.rev < self.base {
                self.rev = self.base;
            }
        }
        r
    }

    /// Builds a delta with every key changed after `base`, at most
    /// `max_changes` of them (ascending by revision). If truncated, `rev` is
    /// the last included revision, so the delta is still a consistent prefix.
    pub fn delta_since(&self, base: Rev, tick: Tick, max_changes: usize) -> Delta {
        let mut changes = Vec::new();
        let mut last = base;
        let mut truncated = false;
        for (r, k) in self.by_rev.range((Bound::Excluded(base), Bound::Unbounded)) {
            if changes.len() >= max_changes {
                truncated = true;
                break;
            }
            let e = &self.map[k];
            changes.push(Change {
                form: k.form,
                field: e.field.clone(),
                rev_off: r.0 - base.0,
            });
            last = *r;
        }
        let rev = if truncated { last } else { self.rev.max(base) };
        Delta {
            tick,
            base_rev: base,
            rev,
            changes,
            complete: !truncated,
        }
    }

    /// All records, ascending by key.
    pub fn entries(&self) -> Vec<SnapEntry> {
        self.map
            .iter()
            .map(|(k, e)| SnapEntry {
                form: k.form,
                field: e.field.clone(),
                rev: e.rev,
            })
            .collect()
    }

    /// Replaces the whole content (join snapshot). Baseline becomes `rev`.
    pub fn load(&mut self, entries: Vec<SnapEntry>, rev: Rev) {
        *self = StateStore::new();
        for e in entries {
            self.apply_entry(e.form, e.field, e.rev);
        }
        self.rev = self.rev.max(rev);
        self.base = rev;
        self.exact = Some(rev);
    }

    /// Records whose form falls into one of `shards`.
    pub fn entries_in_shards(&self, shards: &[u8]) -> Vec<SnapEntry> {
        self.map
            .iter()
            .filter(|(k, _)| shards.contains(&shard_of(k.form)))
            .map(|(k, e)| SnapEntry {
                form: k.form,
                field: e.field.clone(),
                rev: e.rev,
            })
            .collect()
    }

    /// Replaces the content of `shards` with `entries` (desync repair).
    /// Entries outside the listed shards are ignored.
    pub fn replace_shards(&mut self, shards: &[u8], entries: Vec<SnapEntry>) {
        // `exact` is deliberately untouched: a repair only moves content toward the sender.
        let doomed: Vec<Key> = self
            .map
            .keys()
            .filter(|k| shards.contains(&shard_of(k.form)))
            .copied()
            .collect();
        for k in doomed {
            if let Some(e) = self.map.remove(&k) {
                self.by_rev.remove(&e.rev);
            }
        }
        for e in entries {
            if shards.contains(&shard_of(e.form)) {
                self.apply_entry(e.form, e.field, e.rev);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage(s: u16) -> Field {
        Field::QuestStage { stage: s }
    }

    #[test]
    fn write_local_assigns_increasing_revs_and_skips_noops() {
        let mut s = StateStore::new();
        assert_eq!(s.write_local(FormId(1), stage(10)), Some(Rev(1)));
        assert_eq!(s.write_local(FormId(1), stage(10)), None);
        assert_eq!(s.write_local(FormId(1), stage(20)), Some(Rev(2)));
        assert_eq!(s.write_local(FormId(2), stage(5)), Some(Rev(3)));
        assert_eq!(s.rev(), Rev(3));
        assert_eq!(s.base(), Rev(3));
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn apply_entry_lww() {
        let mut s = StateStore::new();
        assert_eq!(
            s.apply_entry(FormId(1), stage(2), Rev(5)),
            ApplyOutcome::Applied
        );
        assert_eq!(
            s.apply_entry(FormId(1), stage(1), Rev(3)),
            ApplyOutcome::Stale
        );
        assert_eq!(
            s.apply_entry(FormId(1), stage(2), Rev(5)),
            ApplyOutcome::Duplicate
        );
        assert_eq!(
            s.apply_entry(FormId(1), stage(9), Rev(7)),
            ApplyOutcome::Applied
        );
        assert_eq!(s.get(FormId(1), &stage(0)).unwrap().field, stage(9));
        assert_eq!(
            s.apply_entry(FormId(1), stage(9), Rev(0)),
            ApplyOutcome::Rejected
        );
        // a different key may not steal an existing revision
        assert_eq!(
            s.apply_entry(FormId(2), stage(1), Rev(7)),
            ApplyOutcome::Rejected
        );
    }

    #[test]
    fn delta_roundtrip_and_idempotent() {
        let mut h = StateStore::new();
        for i in 0..10u32 {
            h.write_local(FormId(i % 4), stage(i as u16 + 1));
        }
        let d = h.delta_since(Rev::ZERO, Tick(1), 100);
        let mut g = StateStore::new();
        let r1 = g.apply_delta(&d);
        assert_eq!(r1.applied as usize, d.changes.len());
        assert_eq!(g.base(), h.rev());
        let r2 = g.apply_delta(&d);
        assert_eq!(r2.applied, 0);
        assert_eq!(r2.duplicate as usize, d.changes.len());
        assert_eq!(g.entries(), h.entries());
    }

    #[test]
    fn delta_since_only_newer_and_truncates_consistently() {
        let mut h = StateStore::new();
        for i in 0..10u32 {
            h.write_local(FormId(i), stage(1));
        }
        let d = h.delta_since(Rev(6), Tick(0), 100);
        assert_eq!(d.changes.len(), 4);
        assert_eq!(d.rev, Rev(10));
        let d = h.delta_since(Rev(0), Tick(0), 3);
        assert_eq!(d.changes.len(), 3);
        assert_eq!(d.rev, Rev(3));
        let mut g = StateStore::new();
        g.apply_delta(&d);
        assert_eq!(g.base(), Rev(3));
        let d2 = h.delta_since(g.base(), Tick(0), 100);
        g.apply_delta(&d2);
        assert_eq!(g.entries(), h.entries());
        assert_eq!(g.base(), Rev(10));
    }

    #[test]
    fn gap_does_not_advance_base_but_fill_does() {
        let mut h = StateStore::new();
        for i in 0..6u32 {
            h.write_local(FormId(i), stage(1));
        }
        let first = h.delta_since(Rev(0), Tick(0), 3);
        let second = h.delta_since(Rev(3), Tick(0), 100);
        let mut g = StateStore::new();
        g.apply_delta(&second); // arrives first (reordered)
        assert_eq!(g.base(), Rev(0));
        assert_eq!(g.rev(), Rev(6));
        g.apply_delta(&first);
        assert_eq!(g.base(), Rev(3));
        // second again now covers base 3
        g.apply_delta(&second);
        assert_eq!(g.base(), Rev(6));
        assert_eq!(g.entries(), h.entries());
    }

    #[test]
    fn malformed_delta_rejected() {
        let mut g = StateStore::new();
        let d = Delta {
            tick: Tick(0),
            base_rev: Rev(0),
            rev: Rev(2),
            complete: true,
            changes: vec![
                Change {
                    form: FormId(1),
                    field: stage(1),
                    rev_off: 0,
                },
                Change {
                    form: FormId(2),
                    field: stage(1),
                    rev_off: 5,
                },
            ],
        };
        let r = g.apply_delta(&d);
        assert_eq!(r.rejected, 2);
        assert_eq!(g.base(), Rev(0));
        assert!(g.is_empty());
    }

    #[test]
    fn load_and_shard_replace() {
        let mut h = StateStore::new();
        for i in 0..300u32 {
            h.write_local(FormId(0x0100_0000 + i), stage(i as u16));
        }
        let mut g = StateStore::new();
        g.load(h.entries(), h.rev());
        assert_eq!(g.entries(), h.entries());
        assert_eq!(g.base(), h.rev());
        // corrupt one shard on g, repair from h
        let victim = FormId(0x0100_0007);
        let shard = shard_of(victim);
        g.replace_shards(
            &[shard],
            vec![SnapEntry {
                form: victim,
                field: stage(999),
                rev: Rev(9999),
            }],
        );
        assert_ne!(g.entries(), h.entries());
        let fix = h.entries_in_shards(&[shard]);
        g.replace_shards(&[shard], fix);
        assert_eq!(g.entries(), h.entries());
    }
}
