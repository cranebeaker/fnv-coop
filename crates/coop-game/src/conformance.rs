//! The checks that each game adapter must pass.
//!
//! A row is a function. It takes a new game and a [`Sample`] of that game's data, and it
//! checks one behaviour. Each row has a mutation twin: with `mutate` set, the row breaks its
//! own scenario, and then it must fail. A twin that passes shows that the row checks nothing.
//!
//! [`ROWS`] lists the rows. [`check`] runs one row and its twin.
//! Give each call a new game: a row sets the role, and it changes state.

use std::collections::BTreeMap;
use std::sync::Arc;

use coop_core::schema::flag;
use coop_policy::PolicyTable;

use crate::{opening, Change, Field, FormId, Intent, Key, Refused, Role, Setup, Sheet, Slot};

/// A game under test: each trait that the rows use. It is [`crate::Game`] under the name
/// that the rows had first.
pub use crate::Game as Subject;

/// Data of the game under test that the rows use.
#[derive(Clone, Debug)]
pub struct Sample {
    /// The editor ID of [`Sample::quest`].
    pub editor_id: String,
    /// An editor ID that no form has. No plugin file has this name.
    pub no_editor_id: String,
    /// The plugin file that adds [`Sample::quest`].
    pub plugin: String,
    /// A quest that the party shares. A new game holds no stage for it.
    pub quest: FormId,
    /// A stage of the quest. Its script writes one other shared slot or more.
    pub stage: u16,
    /// An objective of the quest. A new game does not show it.
    pub objective: u16,
    /// A script line that a host's game can run.
    pub line: String,
    /// The policy that each row installs. `None`: the rows run with no policy.
    pub policy: Option<Arc<PolicyTable>>,
}

/// The signature of a row: the game, its sample, and the twin flag.
pub type RowFn = fn(&mut dyn Subject, &Sample, bool) -> Result<(), String>;

/// One conformance row.
pub struct Row {
    /// The name of the row.
    pub name: &'static str,
    /// The check. With the flag set it runs the twin.
    pub run: RowFn,
    /// Text that the twin's failure contains: the cause that the twin must fail for.
    pub twin_cause: &'static str,
}

/// Every row.
pub const ROWS: &[Row] = &[
    Row {
        name: "names_roundtrip",
        run: names_roundtrip,
        twin_cause: "no form has the editor ID",
    },
    Row {
        name: "apply_runs_no_script",
        run: apply_runs_no_script,
        twin_cause: "story code ran",
    },
    Row {
        name: "clear_returns_default",
        run: clear_returns_default,
        twin_cause: "did not return to the default",
    },
    Row {
        name: "guest_refuses_each_intent",
        run: guest_refuses_each_intent,
        twin_cause: "want Err(NotHost)",
    },
    Row {
        name: "idle_export",
        run: idle_export,
        twin_cause: "differ between two exports",
    },
    Row {
        name: "stage_tells_no_stage_from_stage_0",
        run: stage_tells_no_stage_from_stage_0,
        twin_cause: "after stage 0 is set",
    },
    Row {
        name: "character_reaches_the_local_player",
        run: character_reaches_the_local_player,
        twin_cause: "the local player has the character",
    },
    Row {
        name: "character_stays_with_its_player",
        run: character_stays_with_its_player,
        twin_cause: "changed the local player",
    },
];

/// Runs one row on `game` and its twin on `twin`. Each must be a new game.
/// `Ok`: the row passed, and the twin failed for its own cause.
pub fn check(
    row: &Row,
    game: &mut dyn Subject,
    twin: &mut dyn Subject,
    sample: &Sample,
) -> Result<(), String> {
    (row.run)(game, sample, false).map_err(|e| format!("{}: {e}", row.name))?;
    match (row.run)(twin, sample, true) {
        Ok(()) => Err(format!("{}: the twin passed", row.name)),
        Err(e) if e.contains(row.twin_cause) => Ok(()),
        Err(e) => Err(format!(
            "{}: the twin failed for another cause: {e}",
            row.name
        )),
    }
}

const HOST: Slot = 0;
const GUEST: Slot = 1;

fn configure(game: &mut dyn Subject, sample: &Sample, role: Role) -> Result<Slot, String> {
    let me = match role {
        Role::Host => HOST,
        Role::Guest => GUEST,
    };
    let setup = Setup {
        role,
        me,
        policy: sample.policy.clone(),
        keep_own_rows: false,
    };
    game.configure(&setup)
        .map_err(|e| format!("configure as {role:?}: {e}"))?;
    Ok(me)
}

/// A forced export as a map. It refuses a list that has a slot twice or a default value.
fn export(game: &mut dyn Subject) -> Result<BTreeMap<Key, Field>, String> {
    let list = game
        .export_shared(true)
        .ok_or("a forced export gave no list")?;
    let mut map = BTreeMap::new();
    for (form, field) in list {
        if field.is_tombstone() {
            return Err(format!(
                "the export has a default value: {form:?} {field:?}"
            ));
        }
        let key = field.key(form);
        if map.insert(key, field).is_some() {
            return Err(format!("the export has a slot twice: {key:?}"));
        }
    }
    Ok(map)
}

/// The changes that turn `before` into `after`.
fn diff(before: &BTreeMap<Key, Field>, after: &BTreeMap<Key, Field>) -> Vec<Change> {
    let mut changes = Vec::new();
    for (key, field) in after {
        if before.get(key) != Some(field) {
            changes.push((*key, Some(field.clone())));
        }
    }
    for key in before.keys() {
        if !after.contains_key(key) {
            changes.push((*key, None));
        }
    }
    changes
}

fn set_of(form: FormId, field: Field) -> Change {
    (field.key(form), Some(field))
}

/// Every kind of intent, made from the sample.
fn each_intent(sample: &Sample) -> Vec<Intent> {
    let all = vec![
        Intent::SetStage {
            quest: sample.quest,
            stage: sample.stage,
        },
        Intent::ScriptLine(sample.line.clone()),
        Intent::Character(another_character(&Sheet::default())),
    ];
    // A new kind of intent must come into the list. Without it this match does not compile.
    for intent in &all {
        match intent {
            Intent::SetStage { .. } | Intent::ScriptLine(_) | Intent::Character(_) => {}
        }
    }
    all
}

/// A character that differs from `now` in each part that a sheet sets. It keeps the perks:
/// a character only adds perks.
fn another_character(now: &Sheet) -> Sheet {
    Sheet {
        name: format!("{} the Second", now.name),
        female: Some(!now.female.unwrap_or(false)),
        special: now
            .special
            .map(|v| if (1..10).contains(&v) { v + 1 } else { 1 }),
        tags: if now.tags == [32, 33, 34] {
            vec![35, 36, 37]
        } else {
            vec![32, 33, 34]
        },
        perks: now.perks.clone(),
    }
}

/// An editor ID gives a form, the form gives the editor ID again, and the form's top byte is
/// the load index of its plugin. A name of nothing gives nothing.
///
/// Twin: the row asks for the editor ID that no form has.
pub fn names_roundtrip(game: &mut dyn Subject, s: &Sample, mutate: bool) -> Result<(), String> {
    let id = if mutate {
        &s.no_editor_id
    } else {
        &s.editor_id
    };
    let form = game
        .form_of(id)
        .ok_or_else(|| format!("no form has the editor ID {id}"))?;
    if form != s.quest {
        return Err(format!("{id} gave {form:?}, want {:?}", s.quest));
    }
    if game.form_of(&id.to_ascii_uppercase()) != Some(form)
        || game.form_of(&id.to_ascii_lowercase()) != Some(form)
    {
        return Err(format!("the lookup of {id} depends on the case of letters"));
    }
    let name = game
        .name_of(form)
        .ok_or_else(|| format!("{form:?} has no editor ID"))?;
    if !name.eq_ignore_ascii_case(id) {
        return Err(format!("{form:?} has the editor ID {name}, want {id}"));
    }
    if game.form_of(&name) != Some(form) {
        return Err(format!("{name} does not give {form:?} again"));
    }
    let index = game
        .load_index(&s.plugin)
        .ok_or_else(|| format!("the plugin {} has no load index", s.plugin))?;
    if (form.0 >> 24) as u8 != index {
        return Err(format!(
            "{form:?} is not in {}, which has the load index {index}",
            s.plugin
        ));
    }
    if let Some(found) = game.form_of(&s.no_editor_id) {
        return Err(format!("{} gave the form {found:?}", s.no_editor_id));
    }
    if let Some(found) = game.load_index(&s.no_editor_id) {
        return Err(format!("{} gave the load index {found}", s.no_editor_id));
    }
    Ok(())
}

/// An `apply_shared` of a stage on a guest changes the exported stage, and it changes no
/// other slot: no story code ran.
///
/// Twin: a host's game runs the stage as an intent, so the stage's script runs.
pub fn apply_runs_no_script(
    game: &mut dyn Subject,
    s: &Sample,
    mutate: bool,
) -> Result<(), String> {
    let role = if mutate { Role::Host } else { Role::Guest };
    let me = configure(game, s, role)?;
    let before = export(game)?;
    let want = Field::QuestStage { stage: s.stage };
    let key = want.key(s.quest);
    if before.get(&key) == Some(&want) {
        return Err(format!("the sample's stage {} is set already", s.stage));
    }
    if mutate {
        let intent = Intent::SetStage {
            quest: s.quest,
            stage: s.stage,
        };
        let answer = game.run_intent(me, &intent);
        if answer != Ok(true) {
            return Err(format!("the twin's host answered {answer:?} to {intent:?}"));
        }
    } else {
        let applied = game.apply_shared(&[(key, Some(want.clone()))], false);
        if (applied.set, applied.cleared) != (1, 0) {
            return Err(format!("the apply of one stage reported {applied:?}"));
        }
    }
    let after = export(game)?;
    if after.get(&key) != Some(&want) {
        return Err(format!(
            "the exported stage is {:?}, want {want:?}",
            after.get(&key)
        ));
    }
    match game.quest(s.quest) {
        Some((stage, _)) if stage == s.stage => {}
        other => return Err(format!("quest() gives {other:?}, want stage {}", s.stage)),
    }
    let others: Vec<Change> = diff(&before, &after)
        .into_iter()
        .filter(|(k, _)| *k != key)
        .collect();
    if !others.is_empty() {
        return Err(format!(
            "story code ran: {} other slots changed: {others:?}",
            others.len()
        ));
    }
    Ok(())
}

/// A change with `None` returns its slot to the default: the slot leaves the export, and no
/// other slot moves.
///
/// Twin: the row does not send the clear.
pub fn clear_returns_default(
    game: &mut dyn Subject,
    s: &Sample,
    mutate: bool,
) -> Result<(), String> {
    configure(game, s, Role::Guest)?;
    let before = export(game)?;
    // Two kinds that have a default: the quest's flags and an objective.
    let sets = [
        set_of(
            s.quest,
            Field::QuestFlags {
                flags: flag::RUNNING | flag::COMPLETED,
            },
        ),
        set_of(
            s.quest,
            Field::Objective {
                index: s.objective,
                state: 1,
            },
        ),
    ];
    for (key, field) in &sets {
        if before.get(key) == field.as_ref() {
            return Err(format!("the sample's slot {key:?} is set already"));
        }
    }
    let applied = game.apply_shared(&sets, false);
    if (applied.set, applied.cleared) != (2, 0) {
        return Err(format!("the apply of two fields reported {applied:?}"));
    }
    let held = export(game)?;
    for (key, field) in &sets {
        if held.get(key) != field.as_ref() {
            return Err(format!(
                "the slot {key:?} is {:?} after the apply, want {field:?}",
                held.get(key)
            ));
        }
    }
    if !mutate {
        let clears: Vec<Change> = sets.iter().map(|(key, _)| (*key, None)).collect();
        let applied = game.apply_shared(&clears, false);
        if (applied.set, applied.cleared) != (0, 2) {
            return Err(format!("the clear of two slots reported {applied:?}"));
        }
    }
    let after = export(game)?;
    let left: Vec<&Key> = sets
        .iter()
        .map(|(key, _)| key)
        .filter(|key| after.contains_key(key))
        .collect();
    if !left.is_empty() {
        return Err(format!(
            "{} slots did not return to the default: {left:?}",
            left.len()
        ));
    }
    if let Some((_, flags)) = game.quest(s.quest) {
        if flags != 0 {
            return Err(format!("quest() gives the flags {flags} after the clear"));
        }
    }
    let others: Vec<Change> = diff(&before, &after)
        .into_iter()
        .filter(|(k, _)| !sets.iter().any(|(key, _)| key == k))
        .collect();
    if !others.is_empty() {
        return Err(format!("the clear moved other slots: {others:?}"));
    }
    Ok(())
}

/// A guest's game answers `Err(NotHost)` to each kind of intent, and its shared state does
/// not change.
///
/// Twin: the game is a host, so it does not refuse.
pub fn guest_refuses_each_intent(
    game: &mut dyn Subject,
    s: &Sample,
    mutate: bool,
) -> Result<(), String> {
    let role = if mutate { Role::Host } else { Role::Guest };
    let me = configure(game, s, role)?;
    let before = export(game)?;
    for intent in each_intent(s) {
        let answer = game.run_intent(me, &intent);
        if answer != Err(Refused::NotHost) {
            return Err(format!(
                "the game answered {answer:?} to {intent:?}, want Err(NotHost)"
            ));
        }
    }
    let after = export(game)?;
    let changes = diff(&before, &after);
    if !changes.is_empty() {
        return Err(format!(
            "the refused intents changed shared state: {changes:?}"
        ));
    }
    Ok(())
}

/// Two exports with no change between them are equal. An export that is not forced then
/// gives no list, or the same list.
///
/// Twin: one stage changes between the two exports.
pub fn idle_export(game: &mut dyn Subject, s: &Sample, mutate: bool) -> Result<(), String> {
    configure(game, s, Role::Host)?;
    let first = export(game)?;
    if mutate {
        game.apply_shared(
            &[set_of(s.quest, Field::QuestStage { stage: s.stage })],
            false,
        );
    }
    let second = export(game)?;
    let changes = diff(&first, &second);
    if !changes.is_empty() {
        return Err(format!(
            "{} slots differ between two exports: {changes:?}",
            changes.len()
        ));
    }
    if let Some(list) = game.export_shared(false) {
        let third: BTreeMap<Key, Field> = list.into_iter().map(|(f, v)| (v.key(f), v)).collect();
        if third != first {
            return Err("an export that is not forced gave another list".into());
        }
    }
    Ok(())
}

/// `stage` gives `None` for a quest that has no stage, also when the quest has flags. It
/// gives `Some(0)` after stage 0 is set, and then each stage that is set.
///
/// Twin: the row does not set stage 0.
pub fn stage_tells_no_stage_from_stage_0(
    game: &mut dyn Subject,
    s: &Sample,
    mutate: bool,
) -> Result<(), String> {
    configure(game, s, Role::Guest)?;
    if let Some(stage) = game.stage(s.quest) {
        return Err(format!(
            "a new game holds the stage {stage} for the sample's quest"
        ));
    }
    let running = Field::QuestFlags {
        flags: flag::RUNNING,
    };
    game.apply_shared(&[set_of(s.quest, running)], false);
    match game.quest(s.quest) {
        Some((0, flags)) if flags & flag::RUNNING != 0 => {}
        other => {
            return Err(format!(
                "quest() gives {other:?} for a quest that has flags and no stage, want stage 0"
            ))
        }
    }
    if let Some(stage) = game.stage(s.quest) {
        return Err(format!(
            "stage() gives Some({stage}) for a quest that has flags and no stage, want None"
        ));
    }
    if !mutate {
        game.apply_shared(&[set_of(s.quest, Field::QuestStage { stage: 0 })], false);
    }
    if game.stage(s.quest) != Some(0) {
        return Err(format!(
            "stage() gives {:?} after stage 0 is set, want Some(0)",
            game.stage(s.quest)
        ));
    }
    game.apply_shared(
        &[set_of(s.quest, Field::QuestStage { stage: s.stage })],
        false,
    );
    if game.stage(s.quest) != Some(s.stage) {
        return Err(format!(
            "stage() gives {:?}, want Some({})",
            game.stage(s.quest),
            s.stage
        ));
    }
    Ok(())
}

/// A host's game gives a character to its own player. The local player then has the name,
/// the sex, the attributes and the tag skills of the sheet, and the opening is done for
/// that player.
///
/// Twin: the row does not send the character.
pub fn character_reaches_the_local_player(
    game: &mut dyn Subject,
    s: &Sample,
    mutate: bool,
) -> Result<(), String> {
    let me = configure(game, s, Role::Host)?;
    let want = another_character(&game.local_player().sheet);
    if !mutate {
        let answer = game.run_intent(me, &Intent::Character(want.clone()));
        if answer != Ok(true) {
            return Err(format!("the host answered {answer:?} to a character"));
        }
    }
    let got = game.local_player().sheet;
    let same = got.name == want.name
        && got.female == want.female
        && got.special == want.special
        && got.tags == want.tags
        && want.perks.iter().all(|p| got.perks.contains(p));
    if !same {
        return Err(format!(
            "the local player has the character {got:?}, want {want:?}"
        ));
    }
    for (quest, stage) in [
        (opening::VCG00, opening::VCG00_DONE),
        (opening::VCG01, opening::OPENING_OVER_STAGE),
    ] {
        match game.stage(quest) {
            Some(now) if now >= stage => {}
            other => {
                return Err(format!(
                    "the opening quest {quest:?} has the stage {other:?}, want {stage} or more"
                ))
            }
        }
    }
    Ok(())
}

/// A character for another player does not change the local player. A game that keeps one
/// shared player says so (`add_player` gives `false`), and the row then sends no character.
///
/// Twin: the row gives the character to the game's own player.
pub fn character_stays_with_its_player(
    game: &mut dyn Subject,
    s: &Sample,
    mutate: bool,
) -> Result<(), String> {
    let me = configure(game, s, Role::Host)?;
    let before = game.local_player().sheet;
    let other = another_character(&before);
    let kept = game.add_player(GUEST);
    let target = if mutate { me } else { GUEST };
    if kept || mutate {
        let answer = game.run_intent(target, &Intent::Character(other));
        if answer != Ok(true) {
            return Err(format!(
                "the host answered {answer:?} to the character of player {target}"
            ));
        }
    }
    let after = game.local_player().sheet;
    if after != before {
        return Err(format!(
            "the character of player {target} changed the local player: {after:?}, want {before:?}"
        ));
    }
    if game.add_player(GUEST) != kept {
        return Err("a second add of the same player gave another answer".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeGame;
    // The broken game of these tests implements each trait of a game.
    use crate::{Names, Players, Shared, Story};

    /// In the second plugin, so the load index is not 0.
    const QUEST: FormId = FormId(0x0100_0A03);
    const GLOBAL: FormId = FormId(0x0000_0A00);

    fn sample() -> Sample {
        Sample {
            editor_id: "FakeQuest".into(),
            no_editor_id: "NoSuchThing".into(),
            plugin: "Story.esm".into(),
            quest: QUEST,
            stage: 10,
            objective: 10,
            line: "set FakeGlobal to 7".into(),
            policy: None,
        }
    }

    /// A fake whose stage 10 has a script: it shows an objective. Its script line sets a
    /// global.
    fn game() -> FakeGame {
        let s = sample();
        let mut g = FakeGame::new();
        g.plugin("Base.esm");
        g.plugin(&s.plugin);
        g.name(&s.editor_id, QUEST);
        g.name("FakeGlobal", GLOBAL);
        g.set(GLOBAL, Field::global(5.0));
        g.script(
            Intent::SetStage {
                quest: QUEST,
                stage: s.stage,
            },
            vec![
                set_of(QUEST, Field::QuestStage { stage: s.stage }),
                set_of(
                    QUEST,
                    Field::Objective {
                        index: s.objective,
                        state: 1,
                    },
                ),
            ],
        );
        g.script(
            Intent::ScriptLine(s.line),
            vec![set_of(GLOBAL, Field::global(7.0))],
        );
        g
    }

    #[test]
    fn each_row_passes_on_the_fake() {
        for row in ROWS {
            let result = (row.run)(&mut game(), &sample(), false);
            assert_eq!(result, Ok(()), "{}", row.name);
        }
    }

    #[test]
    fn each_twin_fails_on_the_fake_for_its_own_cause() {
        for row in ROWS {
            let result = (row.run)(&mut game(), &sample(), true);
            let cause = result.expect_err(row.name);
            assert!(
                cause.contains(row.twin_cause),
                "{}: the twin failed for another cause: {cause}",
                row.name
            );
        }
    }

    #[test]
    fn check_takes_a_row_with_its_twin() {
        for row in ROWS {
            assert_eq!(
                check(row, &mut game(), &mut game(), &sample()),
                Ok(()),
                "{}",
                row.name
            );
        }
    }

    /// A game that keeps one shared player is a correct game too: each row passes on it,
    /// and each twin fails for its own cause.
    #[test]
    fn check_takes_each_row_on_a_fake_with_one_shared_player() {
        let shared = || {
            let mut g = game();
            g.one_shared_player();
            g
        };
        for row in ROWS {
            assert_eq!(
                check(row, &mut shared(), &mut shared(), &sample()),
                Ok(()),
                "{}",
                row.name
            );
        }
    }

    /// The twin of `check` itself: a twin that passes, and a twin that fails for another
    /// cause, are each a failure.
    #[test]
    fn check_refuses_a_twin_that_passes_or_fails_for_another_cause() {
        fn always(_: &mut dyn Subject, _: &Sample, _: bool) -> Result<(), String> {
            Ok(())
        }
        let blind = Row {
            name: "blind",
            run: always,
            twin_cause: "never",
        };
        let got = check(&blind, &mut game(), &mut game(), &sample());
        assert_eq!(got, Err("blind: the twin passed".into()));
        let wrong = Row {
            name: "wrong",
            run: idle_export,
            twin_cause: "story code ran",
        };
        let got = check(&wrong, &mut game(), &mut game(), &sample());
        assert!(got.unwrap_err().contains("for another cause"));
    }

    /// What a broken game does wrong. Each fault breaks the rule of one row.
    #[derive(Clone, Copy, PartialEq)]
    enum Fault {
        /// `name_of` knows no form.
        NoNames,
        /// `apply_shared` of a stage also writes what the stage's script writes.
        ApplyRunsScript,
        /// `apply_shared` ignores a change with `None`.
        ClearDoesNothing,
        /// A guest's game runs intents.
        GuestRuns,
        /// Each export carries a number that moves.
        ExportMoves,
        /// `stage` gives stage 0 for a quest that has flags and no stage, as `quest` does.
        StageFromQuest,
        /// The game answers `Ok(true)` to a character, and it writes nothing.
        CharacterDropped,
        /// The character of each player goes to the game's own player.
        CharacterToLocal,
        /// The game keeps one shared player, and `add_player` answers `true`.
        SharedPlayerSaysKept,
    }

    /// A fake with one fault.
    struct Broken {
        game: FakeGame,
        fault: Fault,
        exports: u32,
    }

    impl Names for Broken {
        fn form_of(&self, editor_id: &str) -> Option<FormId> {
            self.game.form_of(editor_id)
        }
        fn name_of(&self, form: FormId) -> Option<String> {
            if self.fault == Fault::NoNames {
                return None;
            }
            self.game.name_of(form)
        }
        fn load_index(&self, plugin: &str) -> Option<u8> {
            self.game.load_index(plugin)
        }
    }

    impl Shared for Broken {
        fn configure(&mut self, setup: &Setup) -> Result<(), String> {
            let mut setup = setup.clone();
            if self.fault == Fault::GuestRuns {
                setup.role = Role::Host;
            }
            self.game.configure(&setup)
        }
        fn export_shared(&mut self, force: bool) -> Option<Vec<(FormId, Field)>> {
            let mut list = self.game.export_shared(force)?;
            if self.fault == Fault::ExportMoves {
                self.exports += 1;
                let count = Field::Global { bits: self.exports };
                list.push((FormId(0x0000_0A01), count));
            }
            Some(list)
        }
        fn apply_shared(&mut self, changes: &[Change], present: bool) -> crate::Applied {
            let mut kept: Vec<Change> = changes.to_vec();
            if self.fault == Fault::ClearDoesNothing {
                kept.retain(|(_, field)| field.is_some());
            }
            let mut applied = self.game.apply_shared(&kept, present);
            // The broken game reports the clears that it did not make.
            applied.cleared += (changes.len() - kept.len()) as u32;
            if self.fault == Fault::ApplyRunsScript {
                for (key, field) in changes {
                    if let Some(Field::QuestStage { stage }) = field {
                        let shown = Field::Objective {
                            index: *stage,
                            state: 1,
                        };
                        self.game.set(key.form, shown);
                    }
                }
            }
            applied
        }
        fn quest(&self, quest: FormId) -> Option<(u16, u32)> {
            self.game.quest(quest)
        }
        fn stage(&self, quest: FormId) -> Option<u16> {
            if self.fault == Fault::StageFromQuest {
                return self.game.quest(quest).map(|(stage, _)| stage);
            }
            self.game.stage(quest)
        }
        fn activity(&self) -> u64 {
            self.game.activity()
        }
    }

    impl Story for Broken {
        fn run_intent(&mut self, slot: Slot, intent: &Intent) -> Result<bool, Refused> {
            if let Intent::Character(_) = intent {
                match self.fault {
                    Fault::CharacterDropped => return Ok(true),
                    Fault::CharacterToLocal => return self.game.run_intent(HOST, intent),
                    _ => {}
                }
            }
            self.game.run_intent(slot, intent)
        }
    }

    impl Players for Broken {
        fn add_player(&mut self, slot: Slot) -> bool {
            if self.fault == Fault::SharedPlayerSaysKept {
                self.game.one_shared_player();
                self.game.add_player(slot);
                return true;
            }
            self.game.add_player(slot)
        }
        fn local_player(&self) -> crate::LocalPlayer {
            self.game.local_player()
        }
    }

    /// Each row fails on the game that breaks its rule, and it names that rule.
    /// A row that passes here does not see the fault that it is for.
    #[test]
    fn each_row_fails_on_the_game_that_breaks_its_rule() {
        let cases = [
            ("names_roundtrip", Fault::NoNames, "has no editor ID"),
            (
                "apply_runs_no_script",
                Fault::ApplyRunsScript,
                "story code ran",
            ),
            (
                "clear_returns_default",
                Fault::ClearDoesNothing,
                "did not return to the default",
            ),
            (
                "guest_refuses_each_intent",
                Fault::GuestRuns,
                "want Err(NotHost)",
            ),
            (
                "idle_export",
                Fault::ExportMoves,
                "differ between two exports",
            ),
            (
                "stage_tells_no_stage_from_stage_0",
                Fault::StageFromQuest,
                "has flags and no stage, want None",
            ),
            (
                "character_reaches_the_local_player",
                Fault::CharacterDropped,
                "the local player has the character",
            ),
            (
                "character_stays_with_its_player",
                Fault::CharacterToLocal,
                "changed the local player",
            ),
            (
                "character_stays_with_its_player",
                Fault::SharedPlayerSaysKept,
                "changed the local player",
            ),
        ];
        for row in ROWS {
            assert!(
                cases.iter().any(|(name, _, _)| *name == row.name),
                "{}: no broken game for this row",
                row.name
            );
        }
        for (name, fault, cause) in cases {
            let row = ROWS.iter().find(|r| r.name == name).expect(name);
            let mut broken = Broken {
                game: game(),
                fault,
                exports: 0,
            };
            let got = (row.run)(&mut broken, &sample(), false).expect_err(name);
            assert!(got.contains(cause), "{name}: {got}");
        }
    }
}
