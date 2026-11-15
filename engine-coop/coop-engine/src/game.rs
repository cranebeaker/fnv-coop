//! The engine adapter of the session's interface (`coop-game`).
//!
//! A session talks to a game only through the traits of `coop-game`. This module implements
//! them on the game program by address: the role with the guest gate, the shared state, the
//! story actions of a host, and the local player's place.
//!
//! [`EngineState`] is what the adapter keeps between frames. [`EngineCoop`] joins it with
//! the program for the calls of one gap between two frames.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::FormId;
use coop_core::schema::Field;
use coop_game::{
    Applied, Change, Intent, LocalPlayer, Names, Place, Players, Refused, Role, Setup, Shared,
    Slot, Story,
};
use coop_policy::Scope;

use crate::audit::Audit;
use crate::gate::Rules;
use crate::program::Program;
use crate::record::Pairs;
use crate::{addr, changes, console, shared};

/// The names of the game's data, from the plugin files of the load order. The program does
/// not resolve each editor id at run time, and a table from the files is the same on each
/// machine.
#[derive(Clone, Debug, Default)]
pub struct NameTable {
    by_name: BTreeMap<String, u32>,
    by_form: BTreeMap<u32, String>,
    plugins: BTreeMap<String, u8>,
    quests: Vec<u32>,
    globals: Vec<u32>,
}

impl NameTable {
    /// Adds a plugin file with its load index.
    pub fn add_plugin(&mut self, file: &str, index: u8) {
        self.plugins.insert(file.to_ascii_lowercase(), index);
    }

    /// Adds a form. `kind`: the record type of the form (`QUST`, `GLOB`, another type). An
    /// editor id that two forms have names the later form, as in the game.
    pub fn add_form(&mut self, kind: &str, form: u32, editor_id: &str) {
        if !editor_id.is_empty() {
            self.by_name.insert(editor_id.to_ascii_lowercase(), form);
            self.by_form.insert(form, editor_id.to_string());
        }
        match kind {
            "QUST" => self.quests.push(form),
            "GLOB" => self.globals.push(form),
            _ => {}
        }
    }

    /// Reads the table's text form: one line for each plugin (`P INDEX FILE`, the index in
    /// hex) and for each form (`F TYPE FORM EDITOR_ID`, the form id in hex). `Err` names the
    /// first bad line.
    pub fn parse(text: &str) -> Result<NameTable, String> {
        let mut table = NameTable::default();
        for (n, line) in text.lines().enumerate() {
            let bad = || format!("line {}: {line}", n + 1);
            let mut words = line.split_whitespace();
            match words.next() {
                None => {}
                Some("P") => {
                    let index = words
                        .next()
                        .and_then(|w| u8::from_str_radix(w, 16).ok())
                        .ok_or_else(bad)?;
                    table.add_plugin(words.next().ok_or_else(bad)?, index);
                }
                Some("F") => {
                    let kind = words.next().ok_or_else(bad)?;
                    let form = words
                        .next()
                        .and_then(|w| u32::from_str_radix(w, 16).ok())
                        .ok_or_else(bad)?;
                    table.add_form(kind, form, words.next().unwrap_or(""));
                }
                Some(_) => return Err(bad()),
            }
        }
        Ok(table)
    }

    /// The quests of the load order (form ids).
    pub fn quests(&self) -> &[u32] {
        &self.quests
    }

    /// The globals of the load order (form ids).
    pub fn globals(&self) -> &[u32] {
        &self.globals
    }

    /// The plugin files in load order (their names in lower case).
    pub fn plugin_files(&self) -> Vec<String> {
        let mut files: Vec<(u8, String)> = self
            .plugins
            .iter()
            .map(|(name, index)| (*index, name.clone()))
            .collect();
        files.sort();
        files.into_iter().map(|(_, name)| name).collect()
    }
}

impl Names for NameTable {
    fn form_of(&self, editor_id: &str) -> Option<FormId> {
        self.by_name
            .get(&editor_id.to_ascii_lowercase())
            .map(|f| FormId(*f))
    }

    fn name_of(&self, form: FormId) -> Option<String> {
        let name = self.by_form.get(&form.0)?;
        // The answer must name `form` again: an editor id that a later form took does not.
        (self.by_name.get(&name.to_ascii_lowercase()) == Some(&form.0)).then(|| name.clone())
    }

    fn load_index(&self, plugin: &str) -> Option<u8> {
        self.plugins.get(&plugin.to_ascii_lowercase()).copied()
    }
}

/// What the adapter keeps between frames.
#[derive(Default)]
pub struct EngineState {
    /// The names of the game's data.
    pub names: NameTable,
    role: Option<Role>,
    me: Slot,
    shared: shared::Shared,
    /// The host's last values, on a guest.
    pub audit: Audit,
    /// The host's index pairs that this guest's game has.
    pub pairs: Option<Pairs>,
    /// The adapter's own count of writes, a part of [`Shared::activity`].
    writes: u64,
}

impl EngineState {
    /// A state with these names. The game has no role yet.
    pub fn new(names: NameTable) -> EngineState {
        EngineState {
            names,
            ..EngineState::default()
        }
    }

    /// The role, after [`Shared::configure`].
    pub fn role(&self) -> Option<Role> {
        self.role
    }
}

/// The gate's rules from a campaign policy: the per-player quests, and the scripts of each
/// `guest_local` list by their form ids in this load order.
pub fn rules_of(setup: &Setup, names: &NameTable) -> Rules {
    let mut rules = Rules::default();
    let Some(policy) = &setup.policy else {
        return rules;
    };
    for row in policy.rows() {
        if row.scope == Scope::PerPlayer {
            rules.local_quests.insert(row.id);
        }
        for name in &row.guest_local {
            // `Plugin.esm:LOCALID`: the script's id in its own plugin.
            let Some((plugin, local)) = name.rsplit_once(':') else {
                continue;
            };
            let (Some(index), Ok(local)) =
                (names.load_index(plugin), u32::from_str_radix(local, 16))
            else {
                // A plugin that is not loaded has no script to run.
                continue;
            };
            rules
                .local_scripts
                .insert((u32::from(index) << 24) | (local & 0x00ff_ffff));
        }
    }
    rules
}

/// A campaign policy with each row keyed by its quest's form id in this load order. A row
/// of a plugin that is not loaded is dropped.
pub fn resolve_policy(
    policy: &coop_policy::PolicyTable,
    names: &NameTable,
) -> Result<coop_policy::PolicyTable, String> {
    policy
        .remap(|row| {
            let index = names.load_index(&row.plugin)?;
            Some((u32::from(index) << 24) | (row.local & 0x00ff_ffff))
        })
        .map(|(table, _)| table)
        .map_err(|e| format!("{e}"))
}

/// The adapter for one gap between two frames: the state and the program.
pub struct EngineCoop<'a> {
    state: &'a mut EngineState,
    p: &'a mut dyn Program,
}

impl<'a> EngineCoop<'a> {
    /// The adapter over this program, for the calls of one gap.
    pub fn over(p: &'a mut dyn Program, state: &'a mut EngineState) -> EngineCoop<'a> {
        EngineCoop { state, p }
    }

    /// Writes changes as plain state, and on a guest notes the host's values for the audit.
    fn apply_changes(&mut self, changes: &[Change], guest: bool) -> Applied {
        let mut done = Applied::default();
        for (key, value) in changes {
            match value {
                Some(field) if !field.is_tombstone() => {
                    if self.state.shared.set(self.p, key.form, field) {
                        done.set += 1;
                    }
                    if guest {
                        self.state.audit.delta(&[(key.form, field.clone())]);
                    }
                }
                // A default value and `None` each return the slot to "not set".
                other => {
                    let like = match other {
                        Some(field) => Some(field.clone()),
                        None => slot_field(key),
                    };
                    if let Some(like) = like {
                        if self.state.shared.clear(self.p, key.form, &like) {
                            done.cleared += 1;
                        }
                    }
                    if guest {
                        self.state.audit.forget(*key);
                    }
                }
            }
        }
        done
    }

    /// The shared fields of one catalog quest as they are now. `None`: the catalog has no such
    /// quest.
    pub fn fields_of(&self, quest: FormId) -> Option<Vec<Field>> {
        self.state.shared.fields_of(self.p, quest.0)
    }

    /// The quests of the shared catalog (form ids).
    pub fn quests(&self) -> Vec<u32> {
        self.state.shared.quests().map(|(id, _)| id).collect()
    }

    /// The objectives and the script variables of a catalog quest whose ids do not fit the
    /// shared fields (above 65535): they are not exported. Empty lists for a quest that has
    /// none, or that is not in the catalog.
    pub fn too_wide(&self, quest: FormId) -> (Vec<u32>, Vec<u32>) {
        let Some(at) = self.state.shared.address(quest.0) else {
            return (Vec::new(), Vec::new());
        };
        let s = crate::quest::read(self.p, at);
        let wide = |id: &u32| *id > u32::from(u16::MAX);
        (
            s.objectives.iter().map(|o| o.0).filter(wide).collect(),
            s.variables.iter().map(|v| v.0).filter(wide).collect(),
        )
    }

    /// Where the local player is, with its space (an interior cell, or a worldspace).
    pub fn local(&mut self) -> Option<crate::travel::Local> {
        crate::travel::local(self.p)
    }

    /// Puts each shown objective of a catalog quest that is missing from the player's
    /// objective list (the Pip-Boy's quest page) back into it. The count put back.
    pub fn relist(&mut self, quest: FormId) -> u32 {
        match self.state.shared.address(quest.0) {
            Some(at) => crate::quest::relist(self.p, at),
            None => 0,
        }
    }

    /// The address of a quest: from the catalog, or by the game's own lookup.
    fn quest_at(&mut self, quest: FormId) -> Option<u32> {
        self.state.shared.address(quest.0).or_else(|| {
            let at = self.p.call(addr::FORM_BY_ID, &[quest.0]).eax;
            (at != 0).then_some(at)
        })
    }
}

/// A field of a slot, by the slot's kind and sub-index: what a clear needs to name the slot.
fn slot_field(key: &coop_core::schema::Key) -> Option<Field> {
    let index = u16::try_from(key.sub).ok();
    Some(match key.kind {
        0 => Field::QuestStage { stage: 0 },
        1 => Field::QuestFlags { flags: 0 },
        2 => Field::Objective {
            index: index?,
            state: 0,
        },
        7 => Field::ScriptVar {
            index: index?,
            bits: 0,
        },
        255 => Field::Custom {
            tag: index?,
            data: Vec::new(),
        },
        // A global and the other kinds have no default.
        _ => return None,
    })
}

impl Names for EngineCoop<'_> {
    fn form_of(&self, editor_id: &str) -> Option<FormId> {
        self.state.names.form_of(editor_id)
    }

    fn name_of(&self, form: FormId) -> Option<String> {
        self.state.names.name_of(form)
    }

    fn load_index(&self, plugin: &str) -> Option<u8> {
        self.state.names.load_index(plugin)
    }
}

impl Shared for EngineCoop<'_> {
    fn configure(&mut self, setup: &Setup) -> Result<(), String> {
        let rules = rules_of(setup, &self.state.names);
        let local: BTreeSet<u32> = rules.local_quests.iter().copied().collect();
        let quests: Vec<u32> = self
            .state
            .names
            .quests()
            .iter()
            .copied()
            .filter(|q| !local.contains(q))
            .collect();
        let scripts: Vec<u32> = rules.local_scripts.iter().copied().collect();
        let globals = self.state.names.globals().to_vec();
        let catalog = shared::Shared::with_local_scripts(self.p, &quests, &globals, &scripts);
        if setup.role == Role::Guest {
            // A guest's game runs no script of a shared quest from now on.
            let shared_quest = catalog.quests().next().map(|(_, at)| at);
            self.p.install_gate(&rules, shared_quest)?;
            self.state.pairs = Pairs::join(self.p);
        }
        self.state.shared = catalog;
        self.state.role = Some(setup.role);
        self.state.me = setup.me;
        self.state.writes += 1;
        Ok(())
    }

    fn export_shared(&mut self, _force: bool) -> Option<Vec<(FormId, Field)>> {
        // The program gives no cheap proof that nothing changed: each call gives the list.
        Some(self.state.shared.export(self.p))
    }

    fn apply_shared(&mut self, changes: &[Change], _present: bool) -> Applied {
        let guest = self.state.role == Some(Role::Guest);
        // No script runs while the host's state goes in: the host ran them.
        self.p.applying(true);
        let done = self.apply_changes(changes, guest);
        self.p.applying(false);
        self.state.writes += 1;
        // The game's own journal lines for an applied change are not shown yet:
        // `announced` stays 0.
        done
    }

    fn quest(&self, quest: FormId) -> Option<(u16, u32)> {
        let fields = self.state.shared.fields_of(self.p, quest.0)?;
        let stage = fields.iter().find_map(|f| match f {
            Field::QuestStage { stage } => Some(*stage),
            _ => None,
        });
        let flags = fields
            .iter()
            .find_map(|f| match f {
                Field::QuestFlags { flags } => Some(*flags),
                _ => None,
            })
            .unwrap_or(0);
        if stage.is_none() && flags == 0 {
            return None;
        }
        Some((stage.unwrap_or(0), flags))
    }

    fn stage(&self, quest: FormId) -> Option<u16> {
        self.state
            .shared
            .fields_of(self.p, quest.0)?
            .into_iter()
            .find_map(|f| match f {
                Field::QuestStage { stage } => Some(stage),
                _ => None,
            })
    }

    fn activity(&self) -> u64 {
        // The game marks each changed form in its own map: a new form or a new flag moves the
        // number. The adapter's own writes move it too.
        let map = changes::walk(self.p).iter().fold(0u64, |sum, e| {
            sum.wrapping_mul(31)
                .wrapping_add(u64::from(e.form) ^ (u64::from(e.flags) << 32))
        });
        map ^ self.state.writes.wrapping_mul(0x9e37_79b9_7f4a_7c15)
    }
}

impl Story for EngineCoop<'_> {
    fn run_intent(&mut self, _slot: Slot, intent: &Intent) -> Result<bool, Refused> {
        if self.state.role != Some(Role::Host) {
            return Err(Refused::NotHost);
        }
        self.state.writes += 1;
        match intent {
            Intent::SetStage { quest, stage } => {
                let at = self.quest_at(*quest).ok_or(Refused::NoSuchForm)?;
                let stage = u32::from(u8::try_from(*stage).unwrap_or(u8::MAX));
                Ok(self.p.call(addr::QUEST_SET_STAGE, &[at, stage]).bool())
            }
            Intent::ScriptLine(line) => Ok(console::run_line(self.p, line, 0)),
            // The rows of each player are not on the engine yet: the game keeps one player.
            Intent::Character(_) => Ok(false),
        }
    }
}

impl Players for EngineCoop<'_> {
    fn add_player(&mut self, _slot: Slot) -> bool {
        // This game keeps one shared player until the per-player rows exist on the engine.
        false
    }

    fn local_player(&self) -> LocalPlayer {
        let at = self.p.u32(addr::PLAYER);
        if at == 0 {
            return LocalPlayer::default();
        }
        let cell = self.p.u32(at + addr::REF_CELL);
        LocalPlayer {
            place: Place {
                cell: (cell != 0).then(|| FormId(self.p.u32(cell + addr::FORM_ID))),
                // The worldspace needs a call into the program, and this call only reads:
                // `EngineCoop::local` gives the space.
                world: None,
                pos: Some([0, 4, 8].map(|o| self.p.f32(at + addr::REF_POSITION + o))),
                heading: self.p.f32(at + addr::REF_ANGLE + 8),
            },
            ..LocalPlayer::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const QUEST: u32 = 0x0015_d912;
    const OWN: u32 = 0x0010_4c1c;
    const HOUR: u32 = 0x38;

    fn names() -> NameTable {
        NameTable::parse(
            "P 00 FalloutNV.esm\nP 03 OldWorldBlues.esm\n\
             F QUST 0015D912 VCG03\nF QUST 00104C1C VCG01\nF GLOB 00000038 GameHour\n\
             F CELL 00103DF9 GSDocMitchellHouse\nF QUST 0015D912 vcg03\n",
        )
        .unwrap()
    }

    fn game() -> (FakeProgram, EngineState) {
        let mut p = FakeProgram::new();
        p.add_quest(QUEST, &[10, 20], &[10]);
        p.add_quest(OWN, &[5], &[]);
        p.add_global(HOUR, 12.0);
        (p, EngineState::new(names()))
    }

    fn setup(role: Role) -> Setup {
        Setup {
            role,
            me: 0,
            policy: None,
            keep_own_rows: false,
        }
    }

    #[test]
    fn the_name_table_answers_both_ways() {
        let table = names();
        assert_eq!(table.form_of("vcg03"), Some(FormId(QUEST)));
        assert_eq!(table.form_of("GameHour"), Some(FormId(HOUR)));
        assert_eq!(table.form_of("NoSuchName"), None);
        assert_eq!(table.name_of(FormId(QUEST)).as_deref(), Some("vcg03"));
        assert_eq!(table.name_of(FormId(0xbad)), None);
        assert_eq!(table.load_index("oldworldblues.ESM"), Some(3));
        assert_eq!(table.load_index("NoFile.esm"), None);
        assert_eq!(table.quests(), [QUEST, OWN, QUEST]);
        assert!(NameTable::parse("X 1 2").is_err());
        assert!(NameTable::parse("F QUST zz Name").is_err());
    }

    /// A host's game runs a stage with the game's own function, and its export shows it. A
    /// guest's game refuses each intent and changes nothing, and it takes the host's values
    /// as plain state: no stage function runs. Mutation twin: the guest's export differs
    /// before the apply.
    #[test]
    fn a_host_runs_the_story_and_a_guest_takes_its_state() {
        let (mut hp, mut hs) = game();
        let mut host = EngineCoop::over(&mut hp, &mut hs);
        host.configure(&setup(Role::Host)).unwrap();
        let stage = Intent::SetStage {
            quest: FormId(QUEST),
            stage: 10,
        };
        assert_eq!(host.stage(FormId(QUEST)), None);
        assert_eq!(host.run_intent(0, &stage), Ok(true));
        assert_eq!(host.stage(FormId(QUEST)), Some(10));
        assert!(matches!(host.quest(FormId(QUEST)), Some((10, flags)) if flags != 0));
        let missing = Intent::SetStage {
            quest: FormId(0xbad),
            stage: 1,
        };
        assert_eq!(host.run_intent(0, &missing), Err(Refused::NoSuchForm));
        let export = host.export_shared(true).unwrap();

        let (mut gp, mut gs) = game();
        let mut guest = EngineCoop::over(&mut gp, &mut gs);
        guest.configure(&setup(Role::Guest)).unwrap();
        assert_eq!(guest.run_intent(0, &stage), Err(Refused::NotHost));
        assert_eq!(guest.stage(FormId(QUEST)), None);
        assert_ne!(guest.export_shared(true).unwrap(), export, "the twin");
        let changes: Vec<Change> = export
            .iter()
            .map(|(form, field)| (field.key(*form), Some(field.clone())))
            .collect();
        let before = guest.activity();
        let applied = guest.apply_shared(&changes, false);
        assert!(applied.set >= 2, "{applied:?}");
        assert_ne!(guest.activity(), before);
        assert_eq!(guest.export_shared(false).unwrap(), export);
        assert_eq!(guest.stage(FormId(QUEST)), Some(10));
        // A cleared slot returns to its default, and it is not in the export after.
        let flags = Field::QuestFlags { flags: 0 }.key(FormId(QUEST));
        assert_eq!(guest.apply_shared(&[(flags, None)], false).cleared, 1);
        assert!(guest.quest(FormId(QUEST)).is_some_and(|(_, f)| f == 0));
        assert!(
            gp.stages_set.is_empty(),
            "no stage function ran on the guest"
        );
        assert_eq!(gp.gates.len(), 1, "the guest's game got its gate");
        assert!(hp.gates.is_empty(), "a host's game has no gate");
    }

    /// A guest takes the host's objectives inside the apply bracket, through the game's own
    /// setter: the objective is in the player's objective list (the Pip-Boy's quest page), and
    /// the relist finds nothing missing. Mutation twin: a status that the game itself wrote
    /// without its setter is put back into the list by the relist.
    #[test]
    fn a_guests_objectives_reach_its_pip_boy() {
        let (mut p, mut state) = game();
        p.add_player([0.0; 3]);
        let mut guest = EngineCoop::over(&mut p, &mut state);
        guest.configure(&setup(Role::Guest)).unwrap();
        let shown = Field::Objective {
            index: 10,
            state: 1,
        };
        let applied = guest.apply_shared(&[(shown.key(FormId(QUEST)), Some(shown.clone()))], false);
        assert_eq!(applied.set, 1);
        assert_eq!(guest.relist(FormId(QUEST)), 0);
        assert_eq!(guest.relist(FormId(0xbad)), 0);
        assert_eq!(p.applying, 0, "the bracket closed");
        assert_eq!(p.applied_calls, [addr::OBJECTIVE_SET_STATE]);
        assert_eq!(crate::quest::player_objectives(&p).len(), 1);

        let (mut twin, mut ts) = game();
        twin.add_player([0.0; 3]);
        let quest = twin.call(addr::FORM_BY_ID, &[QUEST]).eax;
        let item = crate::quest::objective_items(&twin, quest)[0].0;
        twin.set_u32(item + addr::OBJECTIVE_STATUS, 1);
        let mut guest = EngineCoop::over(&mut twin, &mut ts);
        guest.configure(&setup(Role::Guest)).unwrap();
        assert_eq!(guest.relist(FormId(QUEST)), 1, "the twin: only written");
        assert_eq!(crate::quest::player_objectives(&twin), [item]);
    }

    /// The gate's rules come from the policy: a per-player quest is local and not shared, and
    /// a `guest_local` script is local by its form id in this load order.
    #[test]
    fn the_policy_gives_the_gates_rules() {
        let mut policy = coop_policy::PolicyTable::bundled().expect("the bundled policy");
        let _ = &mut policy;
        let with = Setup {
            policy: Some(std::sync::Arc::new(policy)),
            ..setup(Role::Guest)
        };
        let rules = rules_of(&with, &names());
        assert!(
            rules.local_quests.contains(&OWN),
            "VCG01 is a per-player quest"
        );
        assert!(!rules.local_quests.contains(&QUEST));
        assert!(
            rules.local_scripts.contains(&0x0300_2fd0),
            "the add-on's start script, by its load index"
        );
        assert!(rules_of(&setup(Role::Guest), &names())
            .local_quests
            .is_empty());
        // The policy's rows get the ids of this load order: an add-on's quest moves from the
        // file's index 01 to its plugin's index, and a plugin that is not loaded has no row.
        let bundled = coop_policy::PolicyTable::bundled().unwrap();
        let here = resolve_policy(&bundled, &names()).unwrap();
        assert!(here.row(0x0300_2fca).is_some(), "NVDLC03MQ00 at index 03");
        assert!(here.row(0x0100_2fca).is_none());
        assert!(here.row(OWN).is_some());
        assert!(
            here.rows().len() < bundled.rows().len(),
            "other add-ons are not loaded"
        );

        let (mut p, mut state) = game();
        let mut guest = EngineCoop::over(&mut p, &mut state);
        guest.configure(&with).unwrap();
        // The per-player quest is not in the shared catalog.
        assert_eq!(guest.stage(FormId(OWN)), None);
        let export = guest.export_shared(true).unwrap();
        assert!(export.iter().all(|(form, _)| form.0 != OWN), "{export:?}");
    }

    /// Each player plays its own opening: the generic timer that ends the opening and the four
    /// pre-order pack quests are per-player rows, so a guest's game runs them and keeps them
    /// out of the shared catalog, and the packs' scripts are local by their form ids. Mutation
    /// twin: with no policy they are shared quests and their scripts stay closed.
    #[test]
    fn each_player_runs_its_own_opening_timer_and_pack_quests() {
        const TIMER: u32 = 0x0016_8a86;
        const PACKS: [u32; 4] = [0x0017_2a1d, 0x0017_2a1e, 0x0017_2a1f, 0x0016_74d4];
        const PACK_SCRIPTS: [u32; 4] = [0x0017_2a24, 0x0017_2a25, 0x0017_2a26, 0x0017_2a27];
        let policy = coop_policy::PolicyTable::bundled().expect("the bundled policy");
        let with = Setup {
            policy: Some(std::sync::Arc::new(policy)),
            ..setup(Role::Guest)
        };
        let rules = rules_of(&with, &names());
        let twin = rules_of(&setup(Role::Guest), &names());
        for quest in std::iter::once(TIMER).chain(PACKS) {
            assert!(rules.quest_is_local(quest), "{quest:08x}");
            assert!(!twin.quest_is_local(quest), "the twin: {quest:08x}");
        }
        for script in PACK_SCRIPTS {
            assert!(rules.local_scripts.contains(&script), "{script:08x}");
            assert!(!twin.local_scripts.contains(&script));
        }

        let mut p = FakeProgram::new();
        p.add_quest(TIMER, &[10], &[]);
        p.add_quest(QUEST, &[10], &[]);
        let mut state = EngineState::new(
            NameTable::parse(
                "P 00 FalloutNV.esm\nF QUST 00168A86 VGenericTimer\nF QUST 0015D912 VCG03\n",
            )
            .unwrap(),
        );
        let mut guest = EngineCoop::over(&mut p, &mut state);
        guest.configure(&with).unwrap();
        assert_eq!(
            guest.stage(FormId(TIMER)),
            None,
            "not in the shared catalog"
        );
        assert!(guest
            .export_shared(true)
            .unwrap()
            .iter()
            .all(|(f, _)| f.0 != TIMER));
    }

    #[test]
    fn the_local_player_has_its_cell_and_place() {
        let (mut p, mut state) = game();
        assert_eq!(
            EngineCoop::over(&mut p, &mut state).local_player(),
            LocalPlayer::default()
        );
        let player = p.add_player([1.0, 2.0, 3.0]);
        let cell = p.add_cell(0x0010_3df9, addr::cell_state::ATTACHED);
        p.set_u32(player + addr::REF_CELL, cell);
        p.set_f32(player + addr::REF_ANGLE + 8, 1.5);
        let mut game = EngineCoop::over(&mut p, &mut state);
        let me = game.local_player();
        assert_eq!(me.place.cell, Some(FormId(0x0010_3df9)));
        assert_eq!(me.place.pos, Some([1.0, 2.0, 3.0]));
        assert_eq!(me.place.heading, 1.5);
        assert!(!game.add_player(1));

        let world = p.add_world(0x000d_a726);
        let outside = p.add_exterior(0x000d_e0f1, world, addr::cell_state::ATTACHED);
        p.set_u32(player + addr::REF_CELL, outside);
        let mut game = EngineCoop::over(&mut p, &mut state);
        assert_eq!(game.local_player().place.cell, Some(FormId(0x000d_e0f1)));
        let here = game.local().expect("a player in a cell");
        assert_eq!((here.space, here.cell), (0x000d_a726, 0x000d_e0f1));
    }
}
