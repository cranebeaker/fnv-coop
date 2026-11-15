//! A guest's intents: from the gate's record to the session's commands for the host.
//!
//! A guest's game does not change a shared quest: the gate stops the change and notes it.
//! The host's game then makes the change with its own function, and each game gets the
//! result as state. A stage goes to the host as a stage command. Each other change goes as
//! the game's own script function for it, with form ids: the host's game has the same load
//! order (the handshake compares it), and it does not resolve each editor id.

use coop_core::ids::FormId;
use coop_engine::gate::Intent;
use coop_game::Names;
use coop_session::Command;

/// The bit of an objective's state that shows it, and the bit that completes it.
const DISPLAYED: u8 = 1;
const COMPLETED: u8 = 2;

/// The commands that make the host's game do what `intent` asks. `Err` tells why this
/// intent has no command: the caller logs it, and the change does not happen.
pub fn commands(intent: &Intent, names: &dyn Names) -> Result<Vec<Command>, String> {
    Ok(match *intent {
        Intent::Stage { quest, stage } => match names.name_of(FormId(quest)) {
            Some(name) => vec![Command::Stage(name, u16::from(stage))],
            // A quest with no editor id has no stage command: the script function takes
            // its form id.
            None => vec![Command::Line(format!("SetStage {quest:08X} {stage}"))],
        },
        Intent::Running { quest, on } => {
            let function = if on { "StartQuest" } else { "StopQuest" };
            vec![Command::Line(format!("{function} {quest:08X}"))]
        }
        Intent::Completed { quest, on: true } => {
            vec![Command::Line(format!("CompleteQuest {quest:08X}"))]
        }
        Intent::Failed { quest, on: true } => {
            vec![Command::Line(format!("FailQuest {quest:08X}"))]
        }
        // The game's script language has no function that takes "completed" or "failed"
        // away from a quest.
        Intent::Completed { quest, on: false } | Intent::Failed { quest, on: false } => {
            return Err(format!(
                "the game has no script function that makes the quest {quest:08X} not \
                 completed or not failed again"
            ))
        }
        Intent::Objective {
            quest,
            objective,
            state,
        } => vec![
            Command::Line(format!(
                "SetObjectiveDisplayed {quest:08X} {objective} {}",
                u8::from(state & DISPLAYED != 0)
            )),
            Command::Line(format!(
                "SetObjectiveCompleted {quest:08X} {objective} {}",
                u8::from(state & COMPLETED != 0)
            )),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_engine::game::NameTable;

    const QUEST: u32 = 0x0015_d912;

    fn names() -> NameTable {
        NameTable::parse("P 00 FalloutNV.esm\nF QUST 0015D912 VCG03\nF QUST 00104C1C\n").unwrap()
    }

    #[test]
    fn a_stage_goes_by_the_quests_name_and_each_other_change_by_form_id() {
        let names = names();
        assert_eq!(
            commands(
                &Intent::Stage {
                    quest: QUEST,
                    stage: 10
                },
                &names
            ),
            Ok(vec![Command::Stage("VCG03".into(), 10)])
        );
        assert_eq!(
            commands(
                &Intent::Running {
                    quest: QUEST,
                    on: false
                },
                &names
            ),
            Ok(vec![Command::Line("StopQuest 0015D912".into())])
        );
        assert_eq!(
            commands(
                &Intent::Objective {
                    quest: QUEST,
                    objective: 20,
                    state: 3
                },
                &names
            ),
            Ok(vec![
                Command::Line("SetObjectiveDisplayed 0015D912 20 1".into()),
                Command::Line("SetObjectiveCompleted 0015D912 20 1".into())
            ])
        );
        assert_eq!(
            commands(
                &Intent::Failed {
                    quest: QUEST,
                    on: true
                },
                &names
            ),
            Ok(vec![Command::Line("FailQuest 0015D912".into())])
        );
    }

    /// A quest with no name still gets its stage. An intent with no command is an error
    /// with its cause, not a silent drop.
    #[test]
    fn an_intent_with_no_command_names_its_cause() {
        let names = names();
        let no_name = commands(
            &Intent::Stage {
                quest: 0x0010_4c1c,
                stage: 5,
            },
            &names,
        );
        assert_eq!(
            no_name,
            Ok(vec![Command::Line("SetStage 00104C1C 5".into())]),
            "a quest with no editor id goes by its form id"
        );
        let back = commands(
            &Intent::Completed {
                quest: QUEST,
                on: false,
            },
            &names,
        );
        assert!(back.unwrap_err().contains("no script function"));
    }
}
