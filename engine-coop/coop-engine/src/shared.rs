//! The shared state of one game: export and apply over a catalog of forms.
//!
//! The catalog is the list of the quests and globals of the load order. It comes from the
//! plugin files, not from the program: the program does not resolve each editor id, and a
//! list from the files is the same on each machine.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::FormId;
use coop_core::schema::{Field, Key};

use crate::addr;
use crate::program::Program;
use crate::quest;

/// What kind of form a catalog entry is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Quest,
    Global,
}

/// The shared forms of one game, with their addresses in the program. A form of the load
/// order does not move, so each address is looked up once.
#[derive(Default)]
pub struct Shared {
    forms: BTreeMap<u32, (Kind, u32)>,
    /// Catalog forms that the program does not have.
    pub missing: Vec<u32>,
}

/// How many slots an apply set and cleared.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Applied {
    /// Slots written with a value.
    pub set: u32,
    /// Slots returned to their default.
    pub cleared: u32,
}

impl Shared {
    /// Looks the catalog's forms up in the program.
    pub fn new(p: &mut dyn Program, quests: &[u32], globals: &[u32]) -> Shared {
        let mut shared = Shared::default();
        let kinds = [(Kind::Quest, quests), (Kind::Global, globals)];
        for (kind, ids) in kinds {
            for id in ids {
                match p.call(addr::FORM_BY_ID, &[*id]).eax {
                    0 => shared.missing.push(*id),
                    at => {
                        shared.forms.insert(*id, (kind, at));
                    }
                }
            }
        }
        shared
    }

    /// The address of a catalog form.
    pub fn address(&self, id: u32) -> Option<u32> {
        self.forms.get(&id).map(|(_, at)| *at)
    }

    /// Every shared field as it is now, sorted by key. It reads only memory.
    pub fn export(&self, p: &dyn Program) -> Vec<(FormId, Field)> {
        let mut out: BTreeMap<Key, (FormId, Field)> = BTreeMap::new();
        for (id, (kind, at)) in &self.forms {
            let form = FormId(*id);
            let fields = match kind {
                Kind::Quest => quest::fields(&quest::read(p, *at)),
                Kind::Global => vec![Field::global(p.f32(at + addr::GLOBAL_VALUE))],
            };
            for f in fields {
                out.insert(f.key(form), (form, f));
            }
        }
        out.into_values().collect()
    }

    /// Writes one field as plain state. True when memory changed.
    fn apply_one(&self, p: &mut dyn Program, form: FormId, field: &Field) -> bool {
        match self.forms.get(&form.0) {
            Some((Kind::Quest, at)) => quest::apply(p, *at, field),
            Some((Kind::Global, at)) => match field {
                Field::Global { bits } => {
                    let changed = p.u32(at + addr::GLOBAL_VALUE) != *bits;
                    if changed {
                        p.set_u32(at + addr::GLOBAL_VALUE, *bits);
                    }
                    changed
                }
                _ => false,
            },
            None => false,
        }
    }

    /// Writes these fields as plain state. A field with its default value clears its slot.
    pub fn apply(&self, p: &mut dyn Program, changes: &[(FormId, Field)]) -> Applied {
        let mut applied = Applied::default();
        for (form, field) in changes {
            if self.apply_one(p, *form, field) {
                if field.is_tombstone() {
                    applied.cleared += 1;
                } else {
                    applied.set += 1;
                }
            }
        }
        applied
    }

    /// Makes the shared state equal to `entries`: writes each, and clears each slot that the
    /// program has and `entries` does not name. A stage that `entries` does not name returns
    /// to 0.
    pub fn apply_snapshot(&self, p: &mut dyn Program, entries: &[(FormId, Field)]) -> Applied {
        let named: BTreeSet<Key> = entries.iter().map(|(form, f)| f.key(*form)).collect();
        let mut applied = self.apply(p, entries);
        for (form, field) in self.export(p) {
            if named.contains(&field.key(form)) {
                continue;
            }
            let cleared = match &field {
                Field::QuestStage { .. } => Field::QuestStage { stage: 0 },
                other => other.tombstone(),
            };
            if cleared != field && self.apply_one(p, form, &cleared) {
                applied.cleared += 1;
            }
        }
        applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const QUEST: u32 = 0x0015_d912;
    const OTHER: u32 = 0x0010_4c1c;
    const HOUR: u32 = 0x38;

    fn game() -> (FakeProgram, Shared) {
        let mut p = FakeProgram::new();
        p.add_quest(QUEST, &[10, 20], &[10]);
        p.add_quest(OTHER, &[5], &[]);
        p.add_global(HOUR, 12.0);
        let shared = Shared::new(&mut p, &[QUEST, OTHER, 0xdead], &[HOUR]);
        (p, shared)
    }

    #[test]
    fn the_catalog_names_what_the_program_lacks() {
        let (_, shared) = game();
        assert_eq!(shared.missing, [0xdead]);
        assert!(shared.address(QUEST).is_some());
        assert!(shared.address(0xdead).is_none());
    }

    /// A snapshot of the host makes the guest's export equal, a second one changes nothing,
    /// and no stage function ran on the guest. Mutation twin: an empty snapshot leaves the
    /// guest's export different.
    #[test]
    fn a_snapshot_makes_the_guest_equal_and_is_idempotent() {
        let (mut host, hs) = game();
        let quest = hs.address(QUEST).unwrap();
        host.call(addr::QUEST_SET_STAGE, &[quest, 10]);
        host.set_f32(hs.address(HOUR).unwrap() + addr::GLOBAL_VALUE, 3.5);
        let snapshot = hs.export(&host);

        let (mut guest, gs) = game();
        assert_eq!(gs.apply_snapshot(&mut guest, &[]), Applied::default());
        assert_ne!(gs.export(&guest), snapshot, "the twin: an empty snapshot");
        let applied = gs.apply_snapshot(&mut guest, &snapshot);
        assert_eq!(applied.cleared, 0);
        assert!(applied.set >= 4, "{applied:?}");
        assert_eq!(gs.export(&guest), snapshot);
        assert_eq!(gs.apply_snapshot(&mut guest, &snapshot), Applied::default());
        assert!(guest.stages_set.is_empty());
    }

    /// A snapshot that does not name a slot clears it on the guest: local drift goes.
    #[test]
    fn a_snapshot_clears_what_it_does_not_name() {
        let (host, hs) = game();
        let snapshot = hs.export(&host);
        let (mut guest, gs) = game();
        let quest = gs.address(OTHER).unwrap();
        guest.call(addr::QUEST_SET_STAGE, &[quest, 5]);
        assert_ne!(gs.export(&guest), snapshot);
        let applied = gs.apply_snapshot(&mut guest, &snapshot);
        assert!(applied.cleared >= 3, "{applied:?}");
        assert_eq!(gs.export(&guest), snapshot);
    }

    #[test]
    fn a_field_of_an_unknown_form_changes_nothing() {
        let (mut p, shared) = game();
        let change = (FormId(0x999), Field::QuestStage { stage: 10 });
        assert_eq!(shared.apply(&mut p, &[change]), Applied::default());
    }
}
