//! A quest in the game's memory, loaded and run by the game's own code.
//!
//! [`QuestForm::load`] makes a `TESQuest` (Xbox PDB) with the game's
//! constructor (`CreateFormOfType(0x47)`, `0060c4b0`), goes to its record in
//! its plugin's `TESFile` (`TESFile::SetOffset`, [`GameFile::supply`]), and
//! runs `TESQuest::Load` (`0060ceb0`, which reads the record's chunks
//! through `TESFile`: the quest data, its script, stages and their items
//! with result scripts, objectives and their targets, conditions) and the
//! quest's `InitItem` (`0060cba0`: the script resolved through the form
//! map, its `ScriptLocals`, stages, objectives, conditions). Only the file's
//! bytes are platform (`BSFile`, [`crate::game_file`]).
//! [`QuestForm::run`] runs its script once as the game runs a quest's
//! script (`0060d4d0`: `Script::Run` with the quest's locals).

use crate::game_file::GameFile;
use crate::script_form::ScriptForm;
use engine::units::fallout_shared::run_0060bef0::TESQuest;
use engine::{Engine, Ptr};
use esm::{FormId, LoadedPlugin, RecordEntry};

/// `TESDataHandler::CreateFormOfType` (Xbox PDB, `00465110`).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
/// Form type of `TESQuest` (`QUST`).
const FORM_TYPE_QUEST: u32 = 0x47;
/// `TESForm::iFormID` (Xbox PDB) +0xC.
const FORM_ID: u32 = 0x0c;
/// `TESQuest::Load` (Xbox PDB, `0060ceb0`) and the quest's `InitItem`
/// (`0060cba0`).
const QUEST_LOAD: u32 = 0x0060_ceb0;
const QUEST_INIT_ITEM: u32 = 0x0060_cba0;
/// The quest's `TESScriptableForm` (Xbox PDB) component's script (+4 of it,
/// at +0x18).
const SCRIPTABLE_SCRIPT: u32 = 0x18 + 4;
/// Runs the quest's script (`0060d4d0`).
const RUN_QUEST_SCRIPT: u32 = 0x0060_d4d0;
/// `ScriptLocals::GetVariable(id, script)` (Xbox PDB, `005a9140`).
const GET_VARIABLE: u32 = 0x005a_9140;
/// The quest's stage list as `TESQuest::SetStageDone` walks it: the list
/// (`005d43c0`), a node's item slot (`006815c0`), next node (`00726070`), empty node test
/// (`008256d0`), a stage's index byte (`+0`).
const STAGE_LIST: u32 = 0x005d_43c0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_IS_EMPTY: u32 = 0x0082_56d0;

/// A quest made in the game's memory.
#[derive(Debug, Clone, Copy)]
pub struct QuestForm {
    pub form: u32,
    pub id: FormId,
    /// Its script form, as `InitItem` resolved it (0: none).
    pub script: u32,
}

/// What a quest run can change: the current stage and the script's
/// variables (by name, in the script's order).
#[derive(Debug, Clone, PartialEq)]
pub struct QuestState {
    pub stage: u8,
    pub variables: Vec<(String, f64)>,
}

impl QuestForm {
    /// Makes the quest whose record is `entry` in `plugin` (form id `id` in
    /// the load order) from its record, as the game loads a form: the
    /// constructor, `TESFile::SetOffset`, `TESQuest::Load`, `InitItem`. Its
    /// script must already be in the game's form map ([`ScriptForm::load`]).
    pub fn load(
        e: &mut Engine,
        file: &GameFile,
        plugin: &LoadedPlugin,
        entry: &RecordEntry,
        id: FormId,
    ) -> Result<QuestForm, String> {
        let form = e.call(CREATE_FORM_OF_TYPE, &[FORM_TYPE_QUEST]).u32();
        if form == 0 {
            return Err(format!("quest {id}: CreateFormOfType gave no form"));
        }
        file.supply(e, plugin, entry)?;
        if !e.call(QUEST_LOAD, &[form, file.file]).bool() {
            return Err(format!("quest {id}: TESQuest::Load failed"));
        }
        let loaded = FormId(e.mem.u32(form + FORM_ID));
        if loaded != id {
            return Err(format!("quest {id}: loaded as {loaded}"));
        }
        e.call(QUEST_INIT_ITEM, &[form]);
        let script = e.mem.u32(form + SCRIPTABLE_SCRIPT);
        Ok(QuestForm { form, id, script })
    }

    /// The stage indices the quest has (its `INDX` stages, as loaded), in
    /// the list's order.
    pub fn stages(&self, e: &mut Engine) -> Vec<u8> {
        let mut out = Vec::new();
        let mut node = e.call(STAGE_LIST, &[self.form]).u32();
        while node != 0 && !e.call(NODE_IS_EMPTY, &[node]).bool() {
            let stage = e.call(NODE_ITEM, &[node]).u32();
            let stage = e.mem.u32(stage);
            out.push(e.mem.u8(stage));
            node = e.call(NODE_NEXT, &[node]).u32();
        }
        out
    }

    /// The quest's `ScriptLocals`.
    pub fn locals(&self, e: &mut Engine) -> u32 {
        e.get(Ptr::<TESQuest>::new(self.form), TESQuest::pScriptLocals)
            .addr()
    }

    /// Runs the quest's script once (`0060d4d0`).
    pub fn run(&self, e: &mut Engine) {
        e.call(RUN_QUEST_SCRIPT, &[self.form]);
    }

    /// The quest's current stage and its script's variables now.
    pub fn snapshot(&self, e: &mut Engine, script: &ScriptForm) -> QuestState {
        let locals = self.locals(e);
        let stage = e.get(Ptr::<TESQuest>::new(self.form), TESQuest::cCurrentStage);
        let variables = script
            .variables
            .iter()
            .map(|v| {
                let value = e.call(GET_VARIABLE, &[locals, v.id, self.script]).f64();
                (v.name.clone(), value)
            })
            .collect();
        QuestState { stage, variables }
    }
}
