//! A game made of maps, for the tests of other crates.
//!
//! [`FakeGame`] has a field map, a name table, a table of scripted intents and a call log.
//! It runs no game code. A test fills the tables, hands the fake to the code under test, and
//! then reads the fields and the log.

use std::collections::BTreeMap;

use crate::{
    Applied, Change, Field, FormId, Intent, Key, Names, Refused, Role, Setup, Shared, Slot, Story,
};

/// One call that a [`FakeGame`] got through a trait of this crate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    /// [`Shared::configure`].
    Configure {
        /// The role that the call set.
        role: Role,
        /// The slot of the game's own player.
        me: Slot,
    },
    /// [`Shared::export_shared`].
    Export {
        /// The `force` argument.
        force: bool,
        /// True when the fake gave a list.
        gave_list: bool,
    },
    /// [`Shared::apply_shared`].
    Apply {
        /// The number of changes.
        changes: usize,
        /// The `present` argument.
        present: bool,
    },
    /// [`Story::run_intent`].
    Intent {
        /// The player whose rows were live.
        slot: Slot,
        /// The intent.
        intent: Intent,
        /// The answer of the fake.
        answer: Result<bool, Refused>,
    },
}

/// A game made of maps.
///
/// A new fake has no role. It refuses each intent until [`Shared::configure`] makes it a
/// host. As a guest it refuses each intent. [`Shared::apply_shared`] writes plain state and
/// never runs an intent.
#[derive(Debug, Default)]
pub struct FakeGame {
    fields: BTreeMap<Key, Field>,
    /// Editor ID in lower case, to its form.
    forms: BTreeMap<String, FormId>,
    /// Form, to its editor ID as the test wrote it.
    editor_ids: BTreeMap<FormId, String>,
    /// Plugin file names. The position is the load index.
    plugins: Vec<String>,
    /// The writes that the script of an intent makes.
    scripts: Vec<(Intent, Vec<Change>)>,
    calls: Vec<Call>,
    setup: Option<Setup>,
    activity: u64,
    /// True when a write came after the last export that gave a list.
    changed: bool,
}

impl FakeGame {
    /// A game with no state, no name and no role.
    pub fn new() -> FakeGame {
        FakeGame {
            // The first export gives a list: no earlier list exists.
            changed: true,
            ..FakeGame::default()
        }
    }

    /// Adds a plugin file to the end of the load order. Returns its load index.
    pub fn plugin(&mut self, file: &str) -> u8 {
        self.plugins.push(file.to_string());
        (self.plugins.len() - 1) as u8
    }

    /// Gives a form an editor ID.
    pub fn name(&mut self, editor_id: &str, form: FormId) {
        self.forms.insert(editor_id.to_ascii_lowercase(), form);
        self.editor_ids.insert(form, editor_id.to_string());
    }

    /// Gives an intent its script: the writes that a host's game makes when it runs `intent`.
    /// A host answers `Ok(false)` to an intent that has no script, and changes no state.
    pub fn script(&mut self, intent: Intent, writes: Vec<Change>) {
        self.scripts.retain(|(i, _)| *i != intent);
        self.scripts.push((intent, writes));
    }

    /// Writes one field as a rule of the game does: no call is logged.
    pub fn set(&mut self, form: FormId, field: Field) {
        self.write(field.key(form), Some(field));
    }

    /// The value of one slot. `None`: the slot has its default value.
    pub fn field(&self, key: Key) -> Option<&Field> {
        self.fields.get(&key)
    }

    /// Every slot that does not have its default value.
    pub fn fields(&self) -> &BTreeMap<Key, Field> {
        &self.fields
    }

    /// The calls that the fake got, in their order.
    pub fn calls(&self) -> &[Call] {
        &self.calls
    }

    /// The number of intents that the fake ran (answer `Ok(true)`).
    pub fn intents_run(&self) -> usize {
        self.calls
            .iter()
            .filter(|c| {
                matches!(
                    c,
                    Call::Intent {
                        answer: Ok(true),
                        ..
                    }
                )
            })
            .count()
    }

    /// The last setup. `None`: no [`Shared::configure`] call came.
    pub fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    /// The role. `None`: no [`Shared::configure`] call came.
    pub fn role(&self) -> Option<Role> {
        self.setup.as_ref().map(|s| s.role)
    }

    /// Writes one slot. A default value removes the slot. A field of another slot is ignored.
    fn write(&mut self, key: Key, field: Option<Field>) {
        match field {
            Some(f) if f.key(key.form) != key => return,
            Some(f) if !f.is_tombstone() => {
                self.fields.insert(key, f);
            }
            _ => {
                self.fields.remove(&key);
            }
        }
        self.activity += 1;
        self.changed = true;
    }

    fn answer(&mut self, intent: &Intent) -> Result<bool, Refused> {
        if self.role() != Some(Role::Host) {
            return Err(Refused::NotHost);
        }
        if let Intent::SetStage { quest, .. } = intent {
            if !self.editor_ids.contains_key(quest) {
                return Err(Refused::NoSuchForm);
            }
        }
        let script = self.scripts.iter().find(|(i, _)| i == intent);
        let Some(writes) = script.map(|(_, w)| w.clone()) else {
            return Ok(false);
        };
        for (key, field) in writes {
            self.write(key, field);
        }
        Ok(true)
    }
}

impl Names for FakeGame {
    fn form_of(&self, editor_id: &str) -> Option<FormId> {
        self.forms.get(&editor_id.to_ascii_lowercase()).copied()
    }

    fn name_of(&self, form: FormId) -> Option<String> {
        self.editor_ids.get(&form).cloned()
    }

    fn load_index(&self, plugin: &str) -> Option<u8> {
        self.plugins
            .iter()
            .position(|p| p.eq_ignore_ascii_case(plugin))
            .map(|i| i as u8)
    }
}

impl Shared for FakeGame {
    fn configure(&mut self, setup: &Setup) -> Result<(), String> {
        self.calls.push(Call::Configure {
            role: setup.role,
            me: setup.me,
        });
        self.setup = Some(setup.clone());
        Ok(())
    }

    fn export_shared(&mut self, force: bool) -> Option<Vec<(FormId, Field)>> {
        let gave_list = force || self.changed;
        self.calls.push(Call::Export { force, gave_list });
        if !gave_list {
            return None;
        }
        self.changed = false;
        Some(
            self.fields
                .iter()
                .map(|(k, f)| (k.form, f.clone()))
                .collect(),
        )
    }

    fn apply_shared(&mut self, changes: &[Change], present: bool) -> Applied {
        self.calls.push(Call::Apply {
            changes: changes.len(),
            present,
        });
        // The fake shows nothing to a player: `announced` stays 0.
        let mut applied = Applied::default();
        for (key, field) in changes {
            match field {
                Some(_) => applied.set += 1,
                None => applied.cleared += 1,
            }
            self.write(*key, field.clone());
        }
        applied
    }

    fn quest(&self, quest: FormId) -> Option<(u16, u32)> {
        let stage = match self.fields.get(&Field::QuestStage { stage: 0 }.key(quest)) {
            Some(Field::QuestStage { stage }) => Some(*stage),
            _ => None,
        };
        let flags = match self.fields.get(&Field::QuestFlags { flags: 0 }.key(quest)) {
            Some(Field::QuestFlags { flags }) => *flags,
            _ => 0,
        };
        if stage.is_none() && flags == 0 {
            return None;
        }
        Some((stage.unwrap_or(0), flags))
    }

    fn activity(&self) -> u64 {
        self.activity
    }
}

impl Story for FakeGame {
    fn run_intent(&mut self, slot: Slot, intent: &Intent) -> Result<bool, Refused> {
        let answer = self.answer(intent);
        self.calls.push(Call::Intent {
            slot,
            intent: intent.clone(),
            answer,
        });
        answer
    }
}

#[cfg(test)]
mod tests {
    use coop_core::schema::flag;

    use super::*;

    const QUEST: FormId = FormId(0x0100_0A03);

    fn setup(role: Role, me: Slot) -> Setup {
        Setup {
            role,
            me,
            policy: None,
            keep_own_rows: false,
        }
    }

    fn stage(stage: u16) -> Intent {
        Intent::SetStage {
            quest: QUEST,
            stage,
        }
    }

    /// A fake with a quest whose stage 10 has a script: the stage, and the running flag.
    fn game() -> FakeGame {
        let mut g = FakeGame::new();
        g.plugin("Base.esm");
        g.plugin("Story.esm");
        g.name("FakeQuest", QUEST);
        let writes = [
            Field::QuestStage { stage: 10 },
            Field::QuestFlags {
                flags: flag::RUNNING,
            },
        ];
        g.script(
            stage(10),
            writes
                .iter()
                .map(|f| (f.key(QUEST), Some(f.clone())))
                .collect(),
        );
        g
    }

    #[test]
    fn a_host_runs_a_scripted_intent_and_a_guest_refuses_it() {
        let mut host = game();
        host.configure(&setup(Role::Host, 0)).unwrap();
        assert_eq!(host.run_intent(0, &stage(10)), Ok(true));
        assert_eq!(host.quest(QUEST), Some((10, flag::RUNNING)));
        assert_eq!(host.intents_run(), 1);
        // an intent with no script does nothing
        assert_eq!(host.run_intent(0, &stage(20)), Ok(false));
        assert_eq!(host.quest(QUEST), Some((10, flag::RUNNING)));
        // a quest with no name is no form of this game
        let other = Intent::SetStage {
            quest: FormId(0x99),
            stage: 10,
        };
        assert_eq!(host.run_intent(0, &other), Err(Refused::NoSuchForm));

        // the twin: the same intent on a guest, and on a game with no role
        let mut guest = game();
        assert_eq!(guest.run_intent(1, &stage(10)), Err(Refused::NotHost));
        guest.configure(&setup(Role::Guest, 1)).unwrap();
        assert_eq!(guest.run_intent(1, &stage(10)), Err(Refused::NotHost));
        assert_eq!(guest.quest(QUEST), None);
        assert_eq!(guest.intents_run(), 0);
        assert!(guest.fields().is_empty());
    }

    #[test]
    fn an_apply_writes_plain_state_and_runs_no_intent() {
        let mut g = game();
        g.configure(&setup(Role::Guest, 1)).unwrap();
        let field = Field::QuestStage { stage: 10 };
        let key = field.key(QUEST);
        let applied = g.apply_shared(&[(key, Some(field.clone()))], true);
        assert_eq!((applied.set, applied.cleared, applied.announced), (1, 0, 0));
        // the stage is there, the script's flag is not
        assert_eq!(g.field(key), Some(&field));
        assert_eq!(g.quest(QUEST), Some((10, 0)));
        assert_eq!(g.intents_run(), 0);
        let applied = g.apply_shared(&[(key, None)], false);
        assert_eq!((applied.set, applied.cleared), (0, 1));
        assert!(g.fields().is_empty());
        // a field of another slot is not written
        let wrong = Field::QuestFlags { flags: 1 };
        g.apply_shared(&[(key, Some(wrong))], false);
        assert!(g.fields().is_empty());
    }

    #[test]
    fn an_unforced_export_gives_a_list_only_after_a_write() {
        let mut g = game();
        g.configure(&setup(Role::Host, 0)).unwrap();
        assert_eq!(g.export_shared(false), Some(Vec::new()));
        assert_eq!(g.export_shared(false), None);
        assert_eq!(g.export_shared(true), Some(Vec::new()));
        let before = g.activity();
        g.set(QUEST, Field::QuestStage { stage: 5 });
        assert!(g.activity() > before);
        let list = g.export_shared(false).expect("a write came");
        assert_eq!(list, vec![(QUEST, Field::QuestStage { stage: 5 })]);
        assert_eq!(g.export_shared(false), None);
        // a default value is not in the export
        g.set(QUEST, Field::QuestFlags { flags: 0 });
        assert_eq!(g.export_shared(true), Some(list));
    }

    #[test]
    fn names_and_the_call_log() {
        let mut g = game();
        assert_eq!(g.form_of("fakequest"), Some(QUEST));
        assert_eq!(g.name_of(QUEST).as_deref(), Some("FakeQuest"));
        assert_eq!(g.form_of("NoQuest"), None);
        assert_eq!(g.name_of(FormId(1)), None);
        assert_eq!(g.load_index("story.ESM"), Some(1));
        assert_eq!(g.load_index("None.esm"), None);
        assert_eq!(g.role(), None);
        g.configure(&setup(Role::Guest, 2)).unwrap();
        assert_eq!(g.setup().map(|s| (s.role, s.me)), Some((Role::Guest, 2)));
        g.export_shared(true);
        g.apply_shared(&[], true);
        let _ = g.run_intent(2, &stage(10));
        assert_eq!(
            g.calls(),
            [
                Call::Configure {
                    role: Role::Guest,
                    me: 2
                },
                Call::Export {
                    force: true,
                    gave_list: true
                },
                Call::Apply {
                    changes: 0,
                    present: true
                },
                Call::Intent {
                    slot: 2,
                    intent: stage(10),
                    answer: Err(Refused::NotHost)
                },
            ]
        );
    }
}
