//! What the HUD's quest text is told: a quest added, completed or failed,
//! and custom text such as a place discovered. Read from FalloutNV.exe
//! 1.4.0.525, with the Xbox 360 prototype's names (`QuestUpdateManager`,
//! `HUDMainMenu`; Xbox PDB). The HUD's side (the queue, the letters
//! appearing one by one, the objectives' lines) is `ui::quest_text`.
//!
//! - `0077a480` queues a quest's update (`QuestUpdateManager::QuestUpdate`,
//!   0x324 bytes: the quest at +0, its type at +4: `HQUT_ADD` 0,
//!   `HQUT_COMPLETE` 1, `HQUT_FAILED` 2, `HQUT_CUSTOM` 3). The type is the
//!   quest's state as it's queued: failed (the quest's flag 0x40), else
//!   completed (0x02), else added. A completed or failed quest stops being
//!   the active one; an added one becomes it when there's none.
//! - Its callers: an objective shown (`005ec5d0` with state 1) the first
//!   time for its quest (the quest's flag 0x20, set then and cleared by
//!   `ResetQuest`, `0060d720`); completing a quest (`0060ca30`: not
//!   completed before; `CompleteQuest`, `005c7280`, and a stage's
//!   "complete quest" entry, `0060fb60`); failing one (`0060caf0`: neither
//!   completed nor failed before; it sets both flags; a stage's "fail
//!   quest" entry).
//! - `SetCustomQuestText` (`0076b960`): a title, a subtitle, a queue
//!   priority (`HCQQP_NOW` 0 goes first, `HCQQP_NORMAL` 1 and
//!   `HCQQP_LAST` 2 after; the queue is sorted by priority, `0076bb00`), a
//!   justification (`HCQTJ_LEFT` 0, `CENTER` 1, `RIGHT` 2), the title's
//!   and subtitle's fonts (-1 the HUD's own; font 1, 0, isn't allowed and
//!   becomes -1) and a sound. Its one caller is the compass (`00779070`): a
//!   map marker found gives `sDiscoveredText` over the marker's name, left,
//!   normal priority, the HUD's fonts, `UIPopUpQuestNew`.

use esm::FormId;

use crate::scripting::{Event, GameState};

/// `QuestUpdateManager::HUD_QUEST_UPDATE_TYPE` (Xbox PDB) for a quest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Update {
    Added,
    Completed,
    Failed,
}

/// `SetCustomQuestText`'s text (`0076b960`).
#[derive(Debug, Clone, PartialEq)]
pub struct Custom {
    pub title: String,
    pub subtitle: String,
    /// `HUD_CUSTOM_QUEST_QUEUE_PRIORITY`: 0 now (first), 1 normal, 2 last.
    pub priority: i32,
    /// `HUD_CUSTOM_QUEST_TEXT_JUSTIFICATION`: 0 left, 1 centre, 2 right.
    pub justification: i32,
    /// The title's and subtitle's fonts (from 1); `None` the HUD's own.
    pub title_font: Option<i32>,
    pub subtitle_font: Option<i32>,
    /// The sound's editor ID.
    pub sound: String,
}

impl Custom {
    /// The game's checks (`0076b960`): font 1 (0) isn't allowed (none),
    /// a priority outside 0 to 2 is normal.
    pub fn new(
        title: &str,
        subtitle: &str,
        priority: i32,
        justification: i32,
        title_font: i32,
        subtitle_font: i32,
        sound: &str,
    ) -> Custom {
        let font = |f: i32| (f != 0 && f != -1).then_some(f);
        Custom {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            priority: if (0..=2).contains(&priority) {
                priority
            } else {
                1
            },
            justification,
            title_font: font(title_font),
            subtitle_font: font(subtitle_font),
            sound: sound.to_string(),
        }
    }
}

/// What the HUD's quest text is told.
#[derive(Debug, Clone, PartialEq)]
pub enum QuestText {
    Quest { quest: FormId, update: Update },
    Custom(Custom),
}

/// `0077a480`: the quest's update as its state stands, queued; a quest
/// completed or failed is no longer the active one, an added one becomes
/// it when none is.
pub fn announce(state: &mut GameState, quest: FormId) {
    state.emit(crate::coop::Domain::Flags, quest);
    let update = if state.failed.contains(&quest) {
        Update::Failed
    } else if state.completed.contains(&quest) {
        Update::Completed
    } else {
        Update::Added
    };
    match update {
        Update::Added => crate::quest_targets::objective_shown(state, quest),
        _ => crate::quest_targets::quest_ended(state, quest),
    }
    state
        .events
        .push(Event::QuestText(QuestText::Quest { quest, update }));
}

/// An objective shown (`005ec5d0`, state 1): the first for its quest
/// announces the quest (flag 0x20).
pub fn objective_shown(state: &mut GameState, quest: FormId) {
    if state.quests_announced.insert(quest) {
        announce(state, quest);
    }
}

/// Completing a quest (`0060ca30`): announced when it wasn't completed
/// already. Whether it was newly completed.
pub fn complete(state: &mut GameState, quest: FormId) -> bool {
    state.emit(crate::coop::Domain::Flags, quest);
    if !state.completed.insert(quest) {
        return false;
    }
    announce(state, quest);
    true
}

/// Failing a quest (`0060caf0`): nothing when it's completed or failed
/// already; else it's failed and completed both (flags 0x40 and 0x02) and
/// announced.
pub fn fail(state: &mut GameState, quest: FormId) -> bool {
    state.emit(crate::coop::Domain::Flags, quest);
    if state.completed.contains(&quest) || state.failed.contains(&quest) {
        return false;
    }
    state.failed.insert(quest);
    state.completed.insert(quest);
    announce(state, quest);
    true
}

/// A map marker found (`00779070`): `sDiscoveredText` over its name.
pub fn discovered(state: &mut GameState, discovered_text: &str, name: &str) {
    state
        .events
        .push(Event::QuestText(QuestText::Custom(Custom::new(
            discovered_text,
            name,
            1,
            0,
            -1,
            -1,
            "UIPopUpQuestNew",
        ))));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(state: &GameState) -> Vec<QuestText> {
        state
            .events
            .iter()
            .filter_map(|e| match e {
                Event::QuestText(t) => Some(t.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_quest_is_announced_once_when_its_first_objective_shows() {
        let mut state = GameState::default();
        let q = FormId(0x104c1c);
        objective_shown(&mut state, q);
        objective_shown(&mut state, q);
        assert_eq!(
            texts(&state),
            [QuestText::Quest {
                quest: q,
                update: Update::Added
            }]
        );
        // It becomes the active quest when there's none.
        assert_eq!(state.active_quest, Some(q));
    }

    #[test]
    fn completing_and_failing_announce_once() {
        let mut state = GameState::default();
        let q = FormId(0x104c1c);
        state.active_quest = Some(q);
        assert!(complete(&mut state, q));
        assert!(!complete(&mut state, q));
        // Failing a completed quest does nothing.
        assert!(!fail(&mut state, q));
        assert_eq!(state.active_quest, None);
        let r = FormId(0x104c1d);
        assert!(fail(&mut state, r));
        assert!(state.completed.contains(&r));
        assert_eq!(
            texts(&state),
            [
                QuestText::Quest {
                    quest: q,
                    update: Update::Completed
                },
                QuestText::Quest {
                    quest: r,
                    update: Update::Failed
                }
            ]
        );
    }

    #[test]
    fn custom_text_checks_its_fonts_and_priority() {
        let c = Custom::new("A", "B", 7, 2, 0, 3, "S");
        assert_eq!(
            (c.priority, c.title_font, c.subtitle_font),
            (1, None, Some(3))
        );
        let mut state = GameState::default();
        discovered(&mut state, "You have discovered", "Goodsprings");
        match &texts(&state)[0] {
            QuestText::Custom(c) => {
                assert_eq!(
                    (c.title.as_str(), c.subtitle.as_str(), c.sound.as_str()),
                    ("You have discovered", "Goodsprings", "UIPopUpQuestNew")
                );
                assert_eq!((c.priority, c.justification), (1, 0));
            }
            other => panic!("{other:?}"),
        }
    }
}
