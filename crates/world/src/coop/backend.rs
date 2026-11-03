//! The `WorldBackend` adapter: what coop-core needs from a game world.

use coop_core::delta::{Delta, SnapEntry, StateStore};
use coop_core::hash::{record_hash, ShardHashes};
use coop_core::ids::{shard_of, FormId as CForm, SHARD_COUNT};
use coop_core::schema::{Field, Key};

use super::apply::{self, ApplyReport, Change, Opts};
use super::export::{self, is_tombstone, Fields};
use crate::scripting::GameState;

/// Backend W: the `world` crate's `GameState`.
pub trait WorldBackend {
    /// Every shared field, sorted by key (the full snapshot content).
    fn export_fields(&self) -> Vec<(CForm, Field)>;
    /// Shard hashes over the shared domains only.
    fn shard_hashes(&self) -> ShardHashes;
    /// Applies a delta received from the host (no scripts run).
    fn apply_delta(&mut self, d: &Delta, opts: Opts) -> ApplyReport;
    /// Replaces the shared domains with a snapshot's content.
    fn apply_snapshot(&mut self, entries: &[SnapEntry], opts: Opts) -> ApplyReport;
    /// Makes the shared domains equal a replicated store (level triggered).
    fn apply_store(&mut self, store: &StateStore, opts: Opts) -> ApplyReport;
}

/// The store's content with default-valued (tombstone) entries dropped.
pub fn store_fields(store: &StateStore) -> Fields {
    store
        .iter()
        .filter(|(_, e)| !is_tombstone(&e.field))
        .map(|(k, e)| (*k, e.field.clone()))
        .collect()
}

/// Shard hashes of a store, ignoring tombstones (comparable with
/// [`WorldBackend::shard_hashes`]).
pub fn normalized_hashes(store: &StateStore) -> ShardHashes {
    hashes_of(&store_fields(store))
}

fn hashes_of(f: &Fields) -> ShardHashes {
    let mut shards = vec![0u64; SHARD_COUNT];
    for (k, field) in f {
        let s = shard_of(k.form) as usize;
        shards[s] = shards[s].wrapping_add(record_hash(k.form, field));
    }
    ShardHashes::from_vec(shards).expect("shard count")
}

impl WorldBackend for GameState {
    fn export_fields(&self) -> Vec<(CForm, Field)> {
        export::export(self)
            .into_iter()
            .map(|(k, f)| (k.form, f))
            .collect()
    }

    fn shard_hashes(&self) -> ShardHashes {
        hashes_of(&export::export(self))
    }

    fn apply_delta(&mut self, d: &Delta, opts: Opts) -> ApplyReport {
        let changes: Vec<Change> = d
            .changes
            .iter()
            .map(|c| (c.field.key(c.form), Some(c.field.clone())))
            .collect();
        apply::apply_changes(self, &changes, opts)
    }

    fn apply_snapshot(&mut self, entries: &[SnapEntry], opts: Opts) -> ApplyReport {
        let want: Fields = entries
            .iter()
            .filter(|e| !is_tombstone(&e.field))
            .map(|e| (e.field.key(e.form), e.field.clone()))
            .collect();
        apply::reconcile(self, &want, opts)
    }

    fn apply_store(&mut self, store: &StateStore, opts: Opts) -> ApplyReport {
        apply::reconcile(self, &store_fields(store), opts)
    }
}

/// The key a field lives under (re-exported helper for tests).
pub fn key_of(form: CForm, f: &Field) -> Key {
    f.key(form)
}
