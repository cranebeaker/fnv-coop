//! The shared state of one game: export and apply over a catalog of forms.
//!
//! The catalog is the list of the quests and globals of the load order. It comes from the
//! plugin files, not from the program: the program does not resolve each editor id, and a
//! list from the files is the same on each machine.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::FormId;
use coop_core::schema::{Field, Key};

use crate::addr;
use crate::audit::Leak;
use crate::program::Program;
use crate::quest;

/// What kind of form a catalog entry is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// A quest. `own_variables`: the quest's own script runs on each machine (the policy's
    /// `guest_local` lists), so its script variables are each machine's own state, and they
    /// are not shared.
    Quest {
        own_variables: bool,
    },
    Global,
}

/// The shared forms of one game, with their addresses in the program. A form of the load
/// order does not move, so each address is looked up once.
#[derive(Default)]
pub struct Shared {
    forms: BTreeMap<u32, (Kind, u32)>,
    /// Catalog forms that the program does not have.
    pub missing: Vec<u32>,
    /// The quests whose stage was set to 0 as plain state ([`Shared::set`]).
    zero_stage: BTreeSet<u32>,
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
        Shared::with_local_scripts(p, quests, globals, &[])
    }

    /// Looks the catalog's forms up in the program. `local_scripts`: the scripts that each
    /// machine runs itself (form ids). The variables of a quest whose own script is one of
    /// them are not shared.
    pub fn with_local_scripts(
        p: &mut dyn Program,
        quests: &[u32],
        globals: &[u32],
        local_scripts: &[u32],
    ) -> Shared {
        let mut shared = Shared::default();
        let kinds = [(true, quests), (false, globals)];
        for (quest, ids) in kinds {
            for id in ids {
                let at = p.call(addr::FORM_BY_ID, &[*id]).eax;
                if at == 0 {
                    shared.missing.push(*id);
                    continue;
                }
                let kind = if quest {
                    let script = p.u32(at + addr::QUEST_SCRIPT);
                    let own_variables =
                        script != 0 && local_scripts.contains(&p.u32(script + addr::FORM_ID));
                    Kind::Quest { own_variables }
                } else {
                    Kind::Global
                };
                shared.forms.insert(*id, (kind, at));
            }
        }
        shared
    }

    /// The address of a catalog form.
    pub fn address(&self, id: u32) -> Option<u32> {
        self.forms.get(&id).map(|(_, at)| *at)
    }

    /// The catalog's quests: form id and address.
    pub fn quests(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.forms
            .iter()
            .filter(|(_, (kind, _))| matches!(kind, Kind::Quest { .. }))
            .map(|(id, (_, at))| (*id, *at))
    }

    /// Gives each leaked slot the host's value again. A slot that the host does not have
    /// returns to its default. The count of the slots that changed.
    pub fn restore(&mut self, p: &mut dyn Program, leaks: &[Leak]) -> u32 {
        let mut restored = 0;
        for leak in leaks {
            let changed = match (&leak.host, &leak.here) {
                (Some(host), _) => self.set(p, leak.form, host),
                (None, Some(here)) => self.clear(p, leak.form, here),
                (None, None) => false,
            };
            if changed {
                restored += 1;
            }
        }
        restored
    }

    /// The shared fields of one catalog quest as they are now.
    fn quest_fields(&self, p: &dyn Program, id: u32, own_variables: bool, at: u32) -> Vec<Field> {
        let mut fields = quest::fields(&quest::read(p, at));
        if own_variables {
            fields.retain(|f| !matches!(f, Field::ScriptVar { .. }));
        }
        let has_stage = fields.iter().any(|f| matches!(f, Field::QuestStage { .. }));
        if !has_stage && self.zero_stage.contains(&id) {
            fields.insert(0, Field::QuestStage { stage: 0 });
        }
        fields
    }

    /// The shared fields of one catalog quest as they are now. `None`: the catalog has no
    /// such quest.
    pub fn fields_of(&self, p: &dyn Program, quest: u32) -> Option<Vec<Field>> {
        match self.forms.get(&quest) {
            Some((Kind::Quest { own_variables }, at)) => {
                Some(self.quest_fields(p, quest, *own_variables, *at))
            }
            _ => None,
        }
    }

    /// Every shared field as it is now, sorted by key. It reads only memory.
    pub fn export(&self, p: &dyn Program) -> Vec<(FormId, Field)> {
        let mut out: BTreeMap<Key, (FormId, Field)> = BTreeMap::new();
        for (id, (kind, at)) in &self.forms {
            let form = FormId(*id);
            let fields = match kind {
                Kind::Quest { own_variables } => self.quest_fields(p, *id, *own_variables, *at),
                Kind::Global => vec![Field::global(p.f32(at + addr::GLOBAL_VALUE))],
            };
            for f in fields {
                out.insert(f.key(form), (form, f));
            }
        }
        out.into_values().collect()
    }

    /// Writes one field into the program's memory. True when memory changed.
    fn write(&self, p: &mut dyn Program, form: FormId, field: &Field) -> bool {
        match self.forms.get(&form.0) {
            // A machine's own variables take no value from another machine.
            Some((
                Kind::Quest {
                    own_variables: true,
                },
                _,
            )) if matches!(field, Field::ScriptVar { .. }) => false,
            Some((Kind::Quest { .. }, at)) => quest::apply(p, *at, field),
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

    /// Sets one slot as plain state. True when the slot changed.
    ///
    /// The program has no word for "the stage is set, and it is 0": its own `SetStage 0`
    /// shows in the done stages. A stage 0 that comes alone as plain state is kept here, so
    /// that the export shows it.
    pub fn set(&mut self, p: &mut dyn Program, form: FormId, field: &Field) -> bool {
        let changed = self.write(p, form, field);
        let (Field::QuestStage { stage }, Some((Kind::Quest { .. }, at))) =
            (field, self.forms.get(&form.0))
        else {
            return changed;
        };
        if *stage != 0 {
            self.zero_stage.remove(&form.0);
            return changed;
        }
        let shown = quest::fields(&quest::read(p, *at))
            .iter()
            .any(|f| matches!(f, Field::QuestStage { .. }));
        if shown {
            changed
        } else {
            self.zero_stage.insert(form.0) || changed
        }
    }

    /// Returns one slot to "not set". `like`: a field of the slot (its kind and its
    /// sub-index). True when the slot changed. A kind with no default does not change.
    pub fn clear(&mut self, p: &mut dyn Program, form: FormId, like: &Field) -> bool {
        match like {
            Field::QuestStage { .. } => {
                let kept = self.zero_stage.remove(&form.0);
                self.write(p, form, &Field::QuestStage { stage: 0 }) || kept
            }
            other => {
                let default = other.tombstone();
                default.is_tombstone() && self.write(p, form, &default)
            }
        }
    }

    /// Writes these fields as plain state. A field with its default value clears its slot.
    pub fn apply(&mut self, p: &mut dyn Program, changes: &[(FormId, Field)]) -> Applied {
        let mut applied = Applied::default();
        for (form, field) in changes {
            if field.is_tombstone() {
                if self.clear(p, *form, field) {
                    applied.cleared += 1;
                }
            } else if self.set(p, *form, field) {
                applied.set += 1;
            }
        }
        applied
    }

    /// Makes the shared state equal to `entries`: writes each, and clears each slot that the
    /// program has and `entries` does not name.
    pub fn apply_snapshot(&mut self, p: &mut dyn Program, entries: &[(FormId, Field)]) -> Applied {
        let named: BTreeSet<Key> = entries.iter().map(|(form, f)| f.key(*form)).collect();
        let mut applied = self.apply(p, entries);
        for (form, field) in self.export(p) {
            if !named.contains(&field.key(form)) && self.clear(p, form, &field) {
                applied.cleared += 1;
            }
        }
        applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::Audit;
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

        let (mut guest, mut gs) = game();
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
        let (mut guest, mut gs) = game();
        let quest = gs.address(OTHER).unwrap();
        guest.call(addr::QUEST_SET_STAGE, &[quest, 5]);
        assert_ne!(gs.export(&guest), snapshot);
        let applied = gs.apply_snapshot(&mut guest, &snapshot);
        assert!(applied.cleared >= 3, "{applied:?}");
        assert_eq!(gs.export(&guest), snapshot);
    }

    /// A stage that a guest's own code set is one leak with the quest's form id, and the
    /// restore makes the guest equal to the host again. Mutation twin: with no restore the
    /// exports stay different.
    #[test]
    fn the_audit_names_a_local_change_and_restore_removes_it() {
        let (mut host, hs) = game();
        host.call(addr::QUEST_SET_STAGE, &[hs.address(QUEST).unwrap(), 10]);
        let snapshot = hs.export(&host);
        let (mut guest, mut gs) = game();
        gs.apply_snapshot(&mut guest, &snapshot);
        let mut audit = Audit::default();
        audit.snapshot(&snapshot);
        assert_eq!(audit.leaks(&gs.export(&guest)), []);

        guest.call(addr::QUEST_SET_STAGE, &[gs.address(OTHER).unwrap(), 5]);
        guest.set_f32(gs.address(HOUR).unwrap() + addr::GLOBAL_VALUE, 8.0);
        let leaks = audit.leaks(&gs.export(&guest));
        assert!(leaks.iter().any(|l| l.form == FormId(OTHER)), "{leaks:?}");
        assert!(leaks.iter().any(|l| l.form == FormId(HOUR)), "{leaks:?}");
        assert!(leaks.iter().all(|l| l.form != FormId(QUEST)), "{leaks:?}");
        assert_ne!(gs.export(&guest), snapshot, "the twin: no restore");
        assert!(gs.restore(&mut guest, &leaks) >= 2);
        assert_eq!(gs.export(&guest), snapshot);
        assert_eq!(audit.leaks(&gs.export(&guest)), []);
    }

    /// A quest whose own script runs on each machine keeps its script variables to the
    /// machine: they are not exported, and a value from another machine does not change
    /// them. Its stage is shared as before. Mutation twin: with the script not in the local
    /// list, the variable is exported and applied.
    #[test]
    fn a_local_scripts_quest_keeps_its_variables() {
        const SCRIPT: u32 = 0x0300_2fd0;
        let build = |local: &[u32]| {
            let mut p = FakeProgram::new();
            let quest = p.add_quest(QUEST, &[10], &[]);
            let vars = p.add_variables(quest, &[2]);
            p.add_script(quest, SCRIPT);
            let shared = Shared::with_local_scripts(&mut p, &[QUEST], &[], local);
            (p, shared, vars[0])
        };
        let var = |v: f64| (FormId(QUEST), Field::script_var(2, v));
        let has_var = |fields: &[(FormId, Field)]| {
            fields
                .iter()
                .any(|(_, f)| matches!(f, Field::ScriptVar { .. }))
        };

        let (mut p, mut shared, at) = build(&[SCRIPT]);
        p.write(at + addr::VAR_VALUE, &1.0f64.to_le_bytes());
        p.call(addr::QUEST_SET_STAGE, &[shared.address(QUEST).unwrap(), 10]);
        let export = shared.export(&p);
        assert!(!has_var(&export), "{export:?}");
        assert!(export.contains(&(FormId(QUEST), Field::QuestStage { stage: 10 })));
        assert_eq!(shared.apply(&mut p, &[var(5.0)]), Applied::default());
        assert_eq!(shared.apply_snapshot(&mut p, &export), Applied::default());
        let mut bytes = [0u8; 8];
        p.read(at + addr::VAR_VALUE, &mut bytes);
        assert_eq!(
            f64::from_le_bytes(bytes),
            1.0,
            "the machine's own value stays"
        );

        // The twin: the same quest with its script not local.
        let (mut p, mut shared, at) = build(&[]);
        p.write(at + addr::VAR_VALUE, &1.0f64.to_le_bytes());
        assert!(has_var(&shared.export(&p)));
        assert_eq!(shared.apply(&mut p, &[var(5.0)]).set, 1);
    }

    /// A stage 0 that comes alone as plain state is a set stage: the export shows it, a
    /// later stage replaces it, and a clear takes it away. Mutation twin: a quest that got
    /// no stage exports none.
    #[test]
    fn a_stage_zero_as_plain_state_is_a_set_stage() {
        let stage_of = |shared: &Shared, p: &FakeProgram| {
            shared
                .fields_of(p, QUEST)
                .unwrap()
                .into_iter()
                .find_map(|f| match f {
                    Field::QuestStage { stage } => Some(stage),
                    _ => None,
                })
        };
        let (mut p, mut shared) = game();
        let quest = FormId(QUEST);
        assert_eq!(stage_of(&shared, &p), None, "the twin: no stage was set");
        assert!(shared.set(&mut p, quest, &Field::QuestStage { stage: 0 }));
        assert_eq!(stage_of(&shared, &p), Some(0));
        assert!(shared
            .export(&p)
            .contains(&(quest, Field::QuestStage { stage: 0 })));
        assert!(!shared.set(&mut p, quest, &Field::QuestStage { stage: 0 }));
        assert!(shared.set(&mut p, quest, &Field::QuestStage { stage: 10 }));
        assert_eq!(stage_of(&shared, &p), Some(10));
        assert!(shared.clear(&mut p, quest, &Field::QuestStage { stage: 0 }));
        assert_eq!(stage_of(&shared, &p), None);
        // A set stage 0 leaves with a clear too, and a global has no default.
        shared.set(&mut p, quest, &Field::QuestStage { stage: 0 });
        assert!(shared.clear(&mut p, quest, &Field::QuestStage { stage: 0 }));
        assert_eq!(stage_of(&shared, &p), None);
        assert!(!shared.clear(&mut p, FormId(HOUR), &Field::global(1.0)));
        assert_eq!(shared.fields_of(&p, HOUR), None);
    }

    #[test]
    fn the_catalog_lists_its_quests() {
        let (_, shared) = game();
        let ids: Vec<u32> = shared.quests().map(|(id, _)| id).collect();
        assert_eq!(ids, [OTHER, QUEST]);
    }

    #[test]
    fn a_field_of_an_unknown_form_changes_nothing() {
        let (mut p, mut shared) = game();
        let change = (FormId(0x999), Field::QuestStage { stage: 10 });
        assert_eq!(shared.apply(&mut p, &[change]), Applied::default());
    }
}
