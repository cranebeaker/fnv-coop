//! Shared-state replication over the engine interface (`coop_game::Shared`).
//!
//! * [`HostMirror`] is the host's diff. It takes the game's export and writes each slot that
//!   changed into the replicated store.
//! * [`GuestMirror`] is the guest's reconcile. It makes the game's shared state equal to the
//!   replicated store.
//!
//! The two are the session's only change detector. They name no engine type: a game
//! adapter gives the export and takes the changes.
//!
//! One rule for a host's game: only [`HostMirror::collect`] may call
//! [`Shared::export_shared`]. The `world` adapter answers "nothing changed" from a queue
//! that each export empties (SPEC-02 F.2). A second caller takes changes from that queue,
//! and then the store gets them only at the next forced export.

use std::collections::BTreeMap;

use coop_core::delta::StateStore;
use coop_core::ids::{FormId, Rev};
use coop_core::schema::{Field, Key};
use coop_core::sync::HostSync;
use coop_game::{Applied, Change, Shared};

/// An export as a map from slot to value.
fn by_slot(list: Vec<(FormId, Field)>) -> BTreeMap<Key, Field> {
    list.into_iter()
        .map(|(form, field)| (field.key(form), field))
        .collect()
}

/// What one [`HostMirror::collect`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Collected {
    /// True when the game gave a list and the diff ran.
    pub ran: bool,
    /// The number of slots that got a new value in the store.
    pub written: u32,
    /// The number of slots that returned to their default in the store.
    pub cleared: u32,
}

/// Host side: turns the game's changes into store writes.
pub(crate) struct HostMirror {
    /// Each call with this number forces the export, also when the game is sure that
    /// nothing changed. It is the backstop for a writer that the game does not see.
    /// 0: no call forces the export.
    pub backstop_every: u32,
    calls: u32,
}

impl Default for HostMirror {
    fn default() -> Self {
        HostMirror {
            backstop_every: 20,
            calls: 0,
        }
    }
}

impl HostMirror {
    /// Mirrors the game's shared state into `host.store`. Call it one time for each frame,
    /// after the game ran its rules.
    ///
    /// The first export is not forced: the store of a new host fills at the first change,
    /// or at the first backstop call.
    pub fn collect<G: Shared + ?Sized>(&mut self, game: &mut G, host: &mut HostSync) -> Collected {
        self.calls = self.calls.wrapping_add(1);
        let due = self.backstop_every != 0 && self.calls % self.backstop_every == 0;
        let mut done = Collected::default();
        let Some(list) = game.export_shared(due) else {
            return done;
        };
        done.ran = true;
        let want = by_slot(list);
        for (key, field) in &want {
            if host.write(key.form, field.clone()).is_some() {
                done.written += 1;
            }
        }
        // A slot that left the export returned to its default.
        let stale: Vec<(Key, Field)> = host
            .store
            .iter()
            .filter(|(key, entry)| !want.contains_key(key) && !entry.field.is_tombstone())
            .map(|(key, entry)| (*key, entry.field.tombstone()))
            .collect();
        for (key, default) in stale {
            if host.write(key.form, default).is_some() {
                done.cleared += 1;
            }
        }
        done
    }
}

/// Guest side: makes the game follow the replicated store.
#[derive(Default)]
pub(crate) struct GuestMirror {
    /// The revision and the base of the store at the last apply.
    last: Option<(Rev, Rev)>,
    /// The number of applies.
    pub applies: u32,
    /// The number of applies that found the game different from a store that did not
    /// change (local drift, which the apply repaired).
    pub drift_repairs: u32,
}

impl GuestMirror {
    /// Applies the store to the game when the store changed since the last call.
    /// `recheck`: compare the game with the store also when the store did not change.
    /// `present`: the game shows the journal lines of the changes. The first apply of a
    /// store is the join snapshot, and the game shows nothing for it.
    pub fn sync<G: Shared + ?Sized>(
        &mut self,
        game: &mut G,
        store: &StateStore,
        recheck: bool,
        present: bool,
    ) -> Option<Applied> {
        let stamp = (store.rev(), store.base());
        if self.last == Some(stamp) && !recheck {
            return None;
        }
        let first = self.last.map_or(true, |(rev, _)| rev == Rev::ZERO);
        let applied = reconcile(game, store, present && !first);
        if self.last == Some(stamp) && applied.set + applied.cleared > 0 {
            self.drift_repairs += 1;
        }
        self.last = Some(stamp);
        self.applies += 1;
        Some(applied)
    }

    /// The store revision that the game holds. `None`: no apply came, or the store then
    /// had a gap before that revision.
    pub fn applied_rev(&self) -> Option<Rev> {
        self.last
            .filter(|(rev, base)| rev == base)
            .map(|(rev, _)| rev)
    }

    /// Forgets the last apply. Call it after the guest's game state was loaded again.
    pub fn reset(&mut self) {
        self.last = None;
    }
}

/// Makes the shared state of `game` equal to `store`: a slot that differs gets the store's
/// value, and a slot that the store does not have returns to its default.
fn reconcile<G: Shared + ?Sized>(game: &mut G, store: &StateStore, present: bool) -> Applied {
    let want: BTreeMap<Key, Field> = store
        .iter()
        .filter(|(_, entry)| !entry.field.is_tombstone())
        .map(|(key, entry)| (*key, entry.field.clone()))
        .collect();
    // A forced export always gives a list.
    let have = by_slot(game.export_shared(true).unwrap_or_default());
    let mut changes: Vec<Change> = Vec::new();
    for (key, field) in &want {
        if have.get(key) != Some(field) {
            changes.push((*key, Some(field.clone())));
        }
    }
    for key in have.keys() {
        if !want.contains_key(key) {
            changes.push((*key, None));
        }
    }
    // The game gets the list also when it is empty: an adapter can have work of its own
    // at each apply (the `world` adapter then gives invented variable names their real
    // names).
    game.apply_shared(&changes, present)
}

#[cfg(test)]
mod tests {
    use coop_core::proto::{Hello, PROTOCOL_VERSION};
    use coop_core::schema::flag;
    use coop_core::sync::HostConfig;
    use coop_game::fake::{Call, FakeGame};
    use coop_game::{Role, Setup};

    use super::*;

    const QUEST: FormId = FormId(0x0100_0A03);
    const OTHER: FormId = FormId(0x0100_0A04);

    fn game(role: Role) -> FakeGame {
        let mut g = FakeGame::new();
        let setup = Setup {
            role,
            me: 0,
            policy: None,
            keep_own_rows: false,
        };
        g.configure(&setup).unwrap();
        g
    }

    fn host_sync() -> HostSync {
        let hello = Hello {
            proto: PROTOCOL_VERSION,
            build_hash: 1,
            rules_hash: 1,
            mods_hash: 1,
            exe: String::new(),
            volatile_hash: 0,
            plugins: Vec::new(),
        };
        HostSync::new(HostConfig::new(hello, 7))
    }

    fn stage(stage: u16) -> Field {
        Field::QuestStage { stage }
    }

    fn running() -> Field {
        Field::QuestFlags {
            flags: flag::RUNNING,
        }
    }

    fn held(store: &StateStore, form: FormId, probe: &Field) -> Option<Field> {
        store.get(form, probe).map(|e| e.field.clone())
    }

    fn exports(g: &FakeGame) -> Vec<(bool, bool)> {
        g.calls()
            .iter()
            .filter_map(|c| match c {
                Call::Export { force, gave_list } => Some((*force, *gave_list)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_host_writes_each_changed_slot_and_an_idle_game_gives_no_write() {
        let mut g = game(Role::Host);
        let mut sync = host_sync();
        let mut mirror = HostMirror::default();
        g.set(QUEST, stage(10));
        g.set(QUEST, running());
        let first = mirror.collect(&mut g, &mut sync);
        assert_eq!((first.ran, first.written, first.cleared), (true, 2, 0));
        assert_eq!(held(&sync.store, QUEST, &stage(0)), Some(stage(10)));
        let rev = sync.store.rev();
        // the twin: no change in the game, so no list, no write and no new revision
        let idle = mirror.collect(&mut g, &mut sync);
        assert_eq!(idle, Collected::default());
        assert_eq!(sync.store.rev(), rev);
        // one slot changes: one write, and the other slot keeps its revision
        g.set(QUEST, stage(20));
        let second = mirror.collect(&mut g, &mut sync);
        assert_eq!((second.written, second.cleared), (1, 0));
        assert_eq!(held(&sync.store, QUEST, &stage(0)), Some(stage(20)));
        assert_eq!(sync.store.rev(), rev.next());
    }

    #[test]
    fn a_slot_that_leaves_the_export_gets_its_default_in_the_store() {
        let mut g = game(Role::Host);
        let mut sync = host_sync();
        let mut mirror = HostMirror::default();
        g.set(QUEST, running());
        g.set(OTHER, running());
        mirror.collect(&mut g, &mut sync);
        // the flags of one quest return to the default: the fake drops the slot
        g.set(QUEST, Field::QuestFlags { flags: 0 });
        let done = mirror.collect(&mut g, &mut sync);
        assert_eq!((done.written, done.cleared), (0, 1));
        let cleared = held(&sync.store, QUEST, &running()).expect("the store keeps the slot");
        assert!(cleared.is_tombstone());
        // the twin: the slot that did not leave the export keeps its value
        assert_eq!(held(&sync.store, OTHER, &running()), Some(running()));
        // a default in the store is not cleared again
        g.set(OTHER, stage(5));
        let again = mirror.collect(&mut g, &mut sync);
        assert_eq!((again.written, again.cleared), (1, 0));
    }

    #[test]
    fn the_host_forces_the_export_only_at_the_backstop_call() {
        let mut g = game(Role::Host);
        let mut sync = host_sync();
        let mut mirror = HostMirror {
            backstop_every: 3,
            ..HostMirror::default()
        };
        for _ in 0..6 {
            mirror.collect(&mut g, &mut sync);
        }
        // a new fake gives its first list with no force
        let forced: Vec<bool> = exports(&g).iter().map(|(force, _)| *force).collect();
        assert_eq!(forced, [false, false, true, false, false, true]);
        // the twin: with no backstop, no call forces the export
        let mut g = game(Role::Host);
        let mut mirror = HostMirror {
            backstop_every: 0,
            ..HostMirror::default()
        };
        for _ in 0..6 {
            mirror.collect(&mut g, &mut sync);
        }
        assert!(exports(&g).iter().all(|(force, _)| !force));
        assert_eq!(HostMirror::default().backstop_every, 20);
    }

    /// A store with the given fields, written in their order.
    fn store_of(fields: &[(FormId, Field)]) -> StateStore {
        let mut store = StateStore::new();
        for (form, field) in fields {
            store.write_local(*form, field.clone());
        }
        store
    }

    fn applies(g: &FakeGame) -> Vec<(usize, bool)> {
        g.calls()
            .iter()
            .filter_map(|c| match c {
                Call::Apply { changes, present } => Some((*changes, *present)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_guest_sets_what_differs_and_clears_what_the_store_does_not_have() {
        let mut g = game(Role::Guest);
        // local state that the host does not have
        g.set(OTHER, running());
        g.set(QUEST, stage(5));
        let store = store_of(&[(QUEST, stage(10)), (QUEST, running())]);
        let mut mirror = GuestMirror::default();
        let applied = mirror
            .sync(&mut g, &store, false, true)
            .expect("a new store");
        assert_eq!((applied.set, applied.cleared), (2, 1));
        assert_eq!(g.field(stage(0).key(QUEST)), Some(&stage(10)));
        assert_eq!(g.field(running().key(QUEST)), Some(&running()));
        assert_eq!(g.field(running().key(OTHER)), None);
        assert_eq!(g.intents_run(), 0);
        assert_eq!(mirror.applied_rev(), Some(store.rev()));
        // the twin: the same store again gives no apply
        assert_eq!(mirror.sync(&mut g, &store, false, true), None);
        assert_eq!((mirror.applies, mirror.drift_repairs), (1, 0));
    }

    #[test]
    fn the_guest_shows_nothing_for_the_join_snapshot_and_shows_later_changes() {
        let mut g = game(Role::Guest);
        let mut store = store_of(&[(QUEST, stage(10))]);
        let mut mirror = GuestMirror::default();
        mirror.sync(&mut g, &store, false, true);
        store.write_local(QUEST, stage(20));
        mirror.sync(&mut g, &store, false, true);
        // the twin: the caller asks for no presentation
        store.write_local(QUEST, stage(30));
        mirror.sync(&mut g, &store, false, false);
        assert_eq!(applies(&g), [(1, false), (1, true), (1, false)]);
        // after a reset the next apply is a join snapshot again
        mirror.reset();
        assert_eq!(mirror.applied_rev(), None);
        mirror.sync(&mut g, &store, false, true);
        assert_eq!(applies(&g).last(), Some(&(0, false)));
    }

    #[test]
    fn a_recheck_repairs_local_drift_and_counts_it() {
        let mut g = game(Role::Guest);
        let store = store_of(&[(QUEST, stage(10))]);
        let mut mirror = GuestMirror::default();
        mirror.sync(&mut g, &store, false, true);
        // a recheck of a game that equals the store applies an empty list
        let clean = mirror.sync(&mut g, &store, true, true).expect("a recheck");
        assert_eq!((clean.set, clean.cleared, mirror.drift_repairs), (0, 0, 0));
        assert_eq!(applies(&g).last(), Some(&(0, true)));
        // the game drifts: no store change, so only a recheck sees it
        g.set(QUEST, stage(11));
        assert_eq!(mirror.sync(&mut g, &store, false, true), None);
        assert_eq!(g.field(stage(0).key(QUEST)), Some(&stage(11)));
        let repaired = mirror.sync(&mut g, &store, true, true).expect("a recheck");
        assert_eq!((repaired.set, repaired.cleared), (1, 0));
        assert_eq!(g.field(stage(0).key(QUEST)), Some(&stage(10)));
        assert_eq!((mirror.applies, mirror.drift_repairs), (3, 1));
    }
}
