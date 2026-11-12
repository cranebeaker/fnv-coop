//! A quest's story state as the host sends it and a guest applies it (docs/coop/SPECS.md S5): the
//! current stage, the quest data flags (enabled, completed, failed), which stages are done, and
//! each objective's state (displayed, completed). The quest script's variables are separate
//! ([`vars`]).
//!
//! The host reads a [`QuestRecord`] for each quest the change journal names with its stage flag.
//! The guest applies it inside [`crate::gate::applying`]: each stage the host has done and the guest
//! has not goes through the game's own `SetStageDone`, so the stage items, log entries and change
//! flags happen as in the game, while the result scripts it reaches do not run (their effects
//! arrive from the host). Then the current stage and the flags are made the host's, and a stage
//! done on the guest but not on the host is undone: the host is the authority.

use engine_core::prelude::*;

use crate::{addr, forms, gate};

/// One quest's story state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestRecord {
    /// The quest's form id.
    pub form: u32,
    /// `TESQuest::cCurrentStage`.
    pub stage: u8,
    /// `QuestData::ucFlags`.
    pub flags: u8,
    /// The numbers of the stages that are done, in list order.
    pub done: Vec<u8>,
    /// (objective number, `eState`) for every objective, in list order.
    pub objectives: Vec<(u32, u32)>,
}

/// The items of the list whose first node is at `node`, walked as the exe does: a first node
/// whose two words are zero is an empty list, null items are skipped.
pub(crate) fn list_items(e: &mut Engine, mut node: u32) -> Vec<u32> {
    let mut out = Vec::new();
    if node == 0 || (e.mem.u32(node) == 0 && e.mem.u32(node.wrapping_add(4)) == 0) {
        return out;
    }
    // A list is never longer than this; a cycle in a damaged list stops here.
    for _ in 0..4096 {
        if node == 0 {
            break;
        }
        let item = e.mem.u32(node);
        if item != 0 {
            out.push(item);
        }
        node = e.mem.u32(node.wrapping_add(4));
    }
    out
}

/// The stages of quest `q`: (stage object, its number, done), in list order.
fn stages(e: &mut Engine, q: u32) -> Vec<(u32, u8, bool)> {
    list_items(e, q.wrapping_add(addr::QUEST_STAGE_LIST_OFFSET))
        .into_iter()
        .map(|stage| {
            (
                stage,
                e.mem.u8(stage.wrapping_add(addr::STAGE_INDEX_OFFSET)),
                e.mem.u8(stage.wrapping_add(addr::STAGE_DONE_OFFSET)) != 0,
            )
        })
        .collect()
}

/// The objectives of quest `q`: (objective object, its number), in list order.
fn objectives(e: &mut Engine, q: u32) -> Vec<(u32, u32)> {
    list_items(e, q.wrapping_add(addr::QUEST_OBJECTIVE_LIST_OFFSET))
        .into_iter()
        .map(|o| (o, e.mem.u32(o.wrapping_add(addr::OBJECTIVE_INDEX_OFFSET))))
        .collect()
}

/// The record of quest `id`, or `None` when it is not a quest.
pub fn record(e: &mut Engine, id: u32) -> Option<QuestRecord> {
    let q = forms::quest(e, id)?;
    Some(QuestRecord {
        form: id,
        stage: e.mem.u8(q.wrapping_add(addr::QUEST_CURRENT_STAGE_OFFSET)),
        flags: e.mem.u8(q.wrapping_add(addr::QUEST_FLAGS_OFFSET)),
        done: stages(e, q)
            .into_iter()
            .filter(|s| s.2)
            .map(|s| s.1)
            .collect(),
        objectives: objectives(e, q)
            .into_iter()
            .map(|(o, n)| (n, e.mem.u32(o.wrapping_add(addr::OBJECTIVE_STATE_OFFSET))))
            .collect(),
    })
}

/// Makes the guest's quest the host's record. False when the form is not a quest, or a done stage
/// or an objective of the record is not one of the quest's (the two machines' data differ: nothing
/// is changed). An objective whose state differs is set through the game's own
/// `BGSQuestObjective::SetState`, as the script commands do, so it lands in the player's objective
/// list (the Pip-Boy's quest page) and the HUD says so. An objective the host completed before
/// this game ever showed it goes straight to its state: the game adds any objective leaving the
/// dormant state to the list, and says nothing on the HUD about one that is already done.
pub fn apply(e: &mut Engine, rec: &QuestRecord) -> bool {
    let Some(q) = forms::quest(e, rec.form) else {
        return false;
    };
    let local = stages(e, q);
    if rec.done.iter().any(|n| !local.iter().any(|s| s.1 == *n)) {
        return false;
    }
    let local_objectives = objectives(e, q);
    if rec
        .objectives
        .iter()
        .any(|(n, _)| !local_objectives.iter().any(|o| o.1 == *n))
    {
        return false;
    }
    gate::applying(e, |e| {
        for &(stage, number, done) in &local {
            let host_done = rec.done.contains(&number);
            if host_done && !done {
                e.call(addr::QUEST_SET_STAGE_DONE, &args![q, u32::from(number)]);
            } else if !host_done && done {
                e.mem.set_u8(stage.wrapping_add(addr::STAGE_DONE_OFFSET), 0);
            }
        }
        // Before the flags: showing an objective enables its quest and marks its name as shown,
        // and the host's flags are the last word.
        for &(o, n) in &local_objectives {
            let Some(&(_, state)) = rec.objectives.iter().find(|r| r.0 == n) else {
                continue;
            };
            let at = o.wrapping_add(addr::OBJECTIVE_STATE_OFFSET);
            let had = e.mem.u32(at);
            if had == state {
                continue;
            }
            e.call(addr::OBJECTIVE_SET_STATE, &args![o, state]);
            // A state the game's function would not store as given is still the host's.
            e.mem.set_u32(at, state);
        }
        e.mem
            .set_u8(q.wrapping_add(addr::QUEST_CURRENT_STAGE_OFFSET), rec.stage);
        e.mem
            .set_u8(q.wrapping_add(addr::QUEST_FLAGS_OFFSET), rec.flags);
    });
    true
}

/// Puts into the player's objective list (the Pip-Boy's quest page) every objective of quest
/// `id` that is not dormant and is missing from it, through the game's own function; returns how
/// many. The list is the player's, not the quest's: a game that empties it and keeps the
/// objectives' states (or states written by a build that did not go through the game's function)
/// would otherwise never show them.
pub fn relist(e: &mut Engine, id: u32) -> u32 {
    let Some(q) = forms::quest(e, id) else {
        return 0;
    };
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    if player == 0 {
        return 0;
    }
    let missing: Vec<u32> = {
        let listed = list_items(e, player.wrapping_add(addr::PLAYER_OBJECTIVE_LIST_OFFSET));
        objectives(e, q)
            .into_iter()
            .map(|(o, _)| o)
            .filter(|o| e.mem.u32(o.wrapping_add(addr::OBJECTIVE_STATE_OFFSET)) != 0)
            .filter(|o| !listed.contains(o))
            .collect()
    };
    gate::applying(e, |e| {
        for o in &missing {
            e.call(addr::PLAYER_UPDATE_OBJECTIVE, &args![player, *o]);
        }
    });
    missing.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAP: u32 = 0x0200_0000;
    const QUEST_ID: u32 = 0x0010_4c0c;
    /// A result script with no owner quest, which a stage's mark runs: only the apply scope keeps it
    /// from running on a guest.
    const RESULT_SCRIPT: u32 = 0x0300_0000;
    /// The stand-in runner's run count.
    const RAN: u32 = 0x0100_0000;

    /// One machine with the quest at a fixed place and stages 10, 20, 30 in its list.
    struct World {
        e: Engine,
        quest: u32,
    }

    fn lookup(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: if a[0] == QUEST_ID { HEAP } else { 0 },
            ..Ret::default()
        }
    }

    /// The game's `SetStageDone`, as far as these tests need it: the stage marked done, the
    /// current stage raised, and the stage's result script run (`0060f4a0`'s items).
    fn set_stage_done(e: &mut Engine, a: &[u32]) -> Ret {
        let (q, n) = (a[0], a[1] as u8);
        let Some(&(stage, _, _)) = stages(e, q).iter().find(|s| s.1 == n) else {
            return Ret::default();
        };
        e.mem.set_u8(stage + 1, 1);
        if e.mem.u8(q + 0x60) < n {
            e.mem.set_u8(q + 0x60, n);
        }
        e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![
                0u32,
                RESULT_SCRIPT,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        );
        Ret {
            eax: 1,
            ..Ret::default()
        }
    }

    fn runner(e: &mut Engine, _: &[u32]) -> Ret {
        let n = e.mem.u32(RAN);
        e.mem.set_u32(RAN, n + 1);
        Ret::default()
    }

    fn world() -> World {
        let mut e = Engine::new();
        e.map(HEAP, 0x1000);
        e.map(RAN, 0x10);
        e.map(RESULT_SCRIPT, 0x100);
        let quest = HEAP;
        e.mem
            .set_u8(quest + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_QUEST);
        // Stages at +0x100, +0x110, +0x120; list nodes after the first at +0x200, +0x208.
        let (s, nodes) = (
            [HEAP + 0x100, HEAP + 0x110, HEAP + 0x120],
            [HEAP + 0x200, HEAP + 0x208],
        );
        for (stage, n) in s.iter().zip([10u8, 20, 30]) {
            e.mem.set_u8(*stage, n);
        }
        let head = quest + addr::QUEST_STAGE_LIST_OFFSET;
        e.mem.set_u32(head, s[0]);
        e.mem.set_u32(head + 4, nodes[0]);
        e.mem.set_u32(nodes[0], s[1]);
        e.mem.set_u32(nodes[0] + 4, nodes[1]);
        e.mem.set_u32(nodes[1], s[2]);
        e.register(addr::LOOKUP_FORM, lookup);
        e.register(addr::QUEST_SET_STAGE_DONE, set_stage_done);
        e.register(addr::SCRIPT_RUNNER_RUN, runner);
        e.register(addr::SAVE_LOAD_ADD_CHANGE, |_, _| Ret::default());
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        World { e, quest }
    }

    #[test]
    fn the_record_reads_the_stage_the_flags_and_the_done_stages() {
        let mut w = world();
        assert_eq!(
            record(&mut w.e, QUEST_ID),
            Some(QuestRecord {
                form: QUEST_ID,
                stage: 0,
                flags: 0,
                done: vec![],
                objectives: vec![]
            })
        );
        forms::set_quest_stage(&mut w.e, QUEST_ID, 20);
        w.e.mem.set_u8(w.quest + addr::QUEST_FLAGS_OFFSET, 1);
        let r = record(&mut w.e, QUEST_ID).unwrap();
        assert_eq!((r.stage, r.flags, r.done), (20, 1, vec![20]));
        assert_eq!(record(&mut w.e, 0x9999), None);
    }

    #[test]
    fn a_guest_applies_the_host_record_and_ends_up_equal_without_running_story_scripts() {
        let mut host = world();
        forms::set_quest_stage(&mut host.e, QUEST_ID, 10);
        forms::set_quest_stage(&mut host.e, QUEST_ID, 30);
        host.e
            .mem
            .set_u8(host.quest + addr::QUEST_FLAGS_OFFSET, 0x03);
        // The host's two result scripts ran.
        assert_eq!(host.e.mem.u32(RAN), 2);
        let sent = record(&mut host.e, QUEST_ID).unwrap();

        let mut guest = world();
        gate::set_guest(&mut guest.e, true);
        // A stage done on the guest that the host never did (its done byte, as a drift).
        guest
            .e
            .mem
            .set_u8(HEAP + 0x110 + addr::STAGE_DONE_OFFSET, 1);
        assert!(apply(&mut guest.e, &sent));
        assert_eq!(record(&mut guest.e, QUEST_ID), Some(sent.clone()));
        // No result script ran on the guest.
        assert_eq!(guest.e.mem.u32(RAN), 0);
        // Applying the same record again changes nothing.
        assert!(apply(&mut guest.e, &sent));
        assert_eq!(record(&mut guest.e, QUEST_ID), Some(sent));
    }

    #[test]
    fn a_record_naming_a_stage_the_guest_lacks_is_refused_whole() {
        let mut guest = world();
        let bad = QuestRecord {
            form: QUEST_ID,
            stage: 40,
            flags: 1,
            done: vec![10, 40],
            objectives: vec![],
        };
        assert!(!apply(&mut guest.e, &bad));
        let r = record(&mut guest.e, QUEST_ID).unwrap();
        assert_eq!((r.stage, r.flags, r.done), (0, 0, vec![]));
        let not_a_quest = QuestRecord {
            form: 0x9999,
            ..bad
        };
        assert!(!apply(&mut guest.e, &not_a_quest));
    }

    #[test]
    fn an_empty_stage_list_is_no_stages() {
        let mut w = world();
        let head = w.quest + addr::QUEST_STAGE_LIST_OFFSET;
        w.e.mem.set_u32(head, 0);
        w.e.mem.set_u32(head + 4, 0);
        assert!(record(&mut w.e, QUEST_ID).unwrap().done.is_empty());
    }

    /// The twin: applied outside the gate (as a host would), the stage's result script runs, so
    /// the guest's silence comes from the gate.
    #[test]
    fn twin_on_a_machine_that_is_not_a_guest_the_result_script_runs() {
        let mut host = world();
        forms::set_quest_stage(&mut host.e, QUEST_ID, 10);
        let sent = record(&mut host.e, QUEST_ID).unwrap();
        let mut other = world();
        assert!(apply(&mut other.e, &sent));
        assert_eq!(other.e.mem.u32(RAN), 1);
    }
}
