//! A game made of maps, for the tests of other crates.
//!
//! [`FakeGame`] has a field map, a name table, a table of scripted intents, the rows of its
//! players and a call log. It runs no game code. A test fills the tables, hands the fake to
//! the code under test, and then reads the fields and the log.

use std::collections::BTreeMap;

use crate::{
    opening, Applied, Change, Field, FormId, Intent, Key, LocalPlayer, Names, Place, Players,
    Refused, Role, Setup, Shared, Sheet, Slot, Story,
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
    /// [`Players::add_player`].
    AddPlayer {
        /// The player.
        slot: Slot,
        /// The answer of the fake: true when it keeps rows for each player.
        kept: bool,
    },
}

/// The rows that the fake keeps for a player who is not its own.
#[derive(Clone, Debug, Default, PartialEq)]
struct PlayerRows {
    sheet: Sheet,
    /// The stages of the opening quests.
    stages: BTreeMap<FormId, u16>,
}

/// The stages that end the opening for a player.
const OPENING_DONE: [(FormId, u16); 2] = [
    (opening::VCG00, opening::VCG00_DONE),
    (opening::VCG01, opening::OPENING_OVER_STAGE),
];

/// Writes `sheet` over `to` as [`Intent::Character`] tells: the name and the sex only when
/// the sheet has them, and the perks added.
fn write_sheet(to: &mut Sheet, sheet: &Sheet) {
    if !sheet.name.is_empty() {
        to.name = sheet.name.clone();
    }
    if sheet.female.is_some() {
        to.female = sheet.female;
    }
    to.special = sheet.special;
    to.tags = sheet.tags.clone();
    to.perks.extend(sheet.perks.iter().copied());
    to.perks.sort_unstable();
    to.perks.dedup();
}

/// A game made of maps.
///
/// A new fake has no role. It refuses each intent until [`Shared::configure`] makes it a
/// host. As a guest it refuses each intent. [`Shared::apply_shared`] writes plain state and
/// never runs an intent. The fake keeps rows for each player, until
/// [`FakeGame::one_shared_player`] makes it a game with one shared player.
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
    /// True: the fake keeps one shared player, and it has no rows for a player.
    one_player: bool,
    /// The rows of each player who is not the fake's own.
    players: BTreeMap<Slot, PlayerRows>,
    /// The player of the fake's own slot.
    local: LocalPlayer,
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

    /// Makes the fake a game with one shared player: [`Players::add_player`] answers
    /// `false`, and each character goes to that player.
    pub fn one_shared_player(&mut self) {
        self.one_player = true;
    }

    /// Puts the local player at a place, as a game does when its player moves.
    pub fn set_place(&mut self, place: Place) {
        self.local.place = place;
    }

    /// The character of a player who is not the fake's own. `None`: the fake has no rows
    /// for that player.
    pub fn sheet_of(&self, slot: Slot) -> Option<&Sheet> {
        self.players.get(&slot).map(|rows| &rows.sheet)
    }

    /// The slot of the fake's own player. It is 0 before a [`Shared::configure`] call.
    fn me(&self) -> Slot {
        self.setup.as_ref().map_or(0, |s| s.me)
    }

    /// Gives `slot` a character, and ends the opening for that player. No stage goes down.
    fn give_character(&mut self, slot: Slot, sheet: &Sheet) {
        if self.one_player || slot == self.me() {
            write_sheet(&mut self.local.sheet, sheet);
            for (quest, stage) in OPENING_DONE {
                if self.stage(quest).unwrap_or(0) < stage {
                    self.set(quest, Field::QuestStage { stage });
                }
            }
            return;
        }
        let rows = self.players.entry(slot).or_default();
        write_sheet(&mut rows.sheet, sheet);
        for (quest, stage) in OPENING_DONE {
            let now = rows.stages.entry(quest).or_insert(0);
            *now = (*now).max(stage);
        }
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

    fn answer(&mut self, slot: Slot, intent: &Intent) -> Result<bool, Refused> {
        if self.role() != Some(Role::Host) {
            return Err(Refused::NotHost);
        }
        if let Intent::Character(sheet) = intent {
            self.give_character(slot, sheet);
            return Ok(true);
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
        let stage = self.stage(quest);
        let flags = match self.fields.get(&Field::QuestFlags { flags: 0 }.key(quest)) {
            Some(Field::QuestFlags { flags }) => *flags,
            _ => 0,
        };
        if stage.is_none() && flags == 0 {
            return None;
        }
        Some((stage.unwrap_or(0), flags))
    }

    fn stage(&self, quest: FormId) -> Option<u16> {
        match self.fields.get(&Field::QuestStage { stage: 0 }.key(quest)) {
            Some(Field::QuestStage { stage }) => Some(*stage),
            _ => None,
        }
    }

    fn activity(&self) -> u64 {
        self.activity
    }
}

impl Story for FakeGame {
    fn run_intent(&mut self, slot: Slot, intent: &Intent) -> Result<bool, Refused> {
        let answer = self.answer(slot, intent);
        self.calls.push(Call::Intent {
            slot,
            intent: intent.clone(),
            answer,
        });
        answer
    }
}

impl Players for FakeGame {
    fn add_player(&mut self, slot: Slot) -> bool {
        let kept = !self.one_player;
        if kept && slot != self.me() {
            self.players.entry(slot).or_default();
        }
        self.calls.push(Call::AddPlayer { slot, kept });
        kept
    }

    fn local_player(&self) -> LocalPlayer {
        self.local.clone()
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

    fn sheet(name: &str) -> Sheet {
        Sheet {
            name: name.into(),
            female: Some(true),
            special: [3, 4, 5, 6, 7, 8, 9],
            tags: vec![32, 40],
            perks: vec![FormId(0x1234)],
        }
    }

    #[test]
    fn a_character_goes_to_the_rows_of_its_player() {
        let mut g = game();
        g.configure(&setup(Role::Host, 0)).unwrap();
        assert!(g.add_player(1));
        assert_eq!(g.sheet_of(1), Some(&Sheet::default()));
        let bo = Intent::Character(sheet("Bo"));
        assert_eq!(g.run_intent(1, &bo), Ok(true));
        assert_eq!(g.sheet_of(1), Some(&sheet("Bo")));
        // the local player and the shared state did not change
        assert_eq!(g.local_player(), LocalPlayer::default());
        assert!(g.fields().is_empty());
        // the fake's own player: the opening is done, and a later stage stays
        g.set(opening::VCG01, Field::QuestStage { stage: 150 });
        assert_eq!(g.run_intent(0, &Intent::Character(sheet("Al"))), Ok(true));
        assert_eq!(g.local_player().sheet, sheet("Al"));
        assert_eq!(g.stage(opening::VCG00), Some(opening::VCG00_DONE));
        assert_eq!(g.stage(opening::VCG01), Some(150));
        // an empty name and an unknown sex keep what the player has, and the perks add
        let more = Sheet {
            name: String::new(),
            female: None,
            perks: vec![FormId(0x0100)],
            ..sheet("")
        };
        assert_eq!(g.run_intent(0, &Intent::Character(more)), Ok(true));
        let now = g.local_player().sheet;
        assert_eq!((now.name.as_str(), now.female), ("Al", Some(true)));
        assert_eq!(now.perks, [FormId(0x0100), FormId(0x1234)]);

        // the twin: a guest's game takes no character
        let mut guest = game();
        guest.configure(&setup(Role::Guest, 1)).unwrap();
        assert_eq!(guest.run_intent(1, &bo), Err(Refused::NotHost));
        assert_eq!(guest.local_player(), LocalPlayer::default());
    }

    #[test]
    fn a_fake_with_one_shared_player_has_no_rows_for_a_player() {
        let mut g = game();
        g.configure(&setup(Role::Host, 0)).unwrap();
        g.one_shared_player();
        assert!(!g.add_player(1));
        assert_eq!(g.sheet_of(1), None);
        // a character for another player replaces the shared one
        assert_eq!(g.run_intent(1, &Intent::Character(sheet("Bo"))), Ok(true));
        assert_eq!(g.local_player().sheet, sheet("Bo"));
        // the twin: a fake with rows for each player keeps the two apart
        let mut g = game();
        g.configure(&setup(Role::Host, 0)).unwrap();
        assert!(g.add_player(1));
        assert_eq!(g.run_intent(1, &Intent::Character(sheet("Bo"))), Ok(true));
        assert_eq!(g.local_player().sheet, Sheet::default());
        let added = Call::AddPlayer {
            slot: 1,
            kept: true,
        };
        assert!(g.calls().contains(&added));
    }

    #[test]
    fn the_local_player_is_where_the_test_put_it() {
        let mut g = game();
        assert_eq!(g.local_player().place, Place::default());
        let place = Place {
            cell: Some(FormId(0x0010_4C0F)),
            world: None,
            pos: Some([1.0, 2.0, 3.0]),
            heading: 0.5,
        };
        g.set_place(place);
        assert_eq!(g.local_player().place, place);
    }
}
