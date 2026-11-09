//! A small story in engine memory, for tests of co-op without the game (feature `testkit`): quests
//! with stage lists, globals, and stand-ins for the game functions co-op calls, laid out as the
//! game lays them out (docs/coop/SPECS.md S2, S5). Each stand-in does what co-op relies on the real
//! one to do, and no more:
//!
//! - `GetFormByID` finds the story's forms by id.
//! - `SetStageDone` marks the stage done, raises the current stage, flags the quest's change through
//!   `BGSSaveLoadGame::AddChange` (so the change journal sees it), and runs the stage's result script
//!   through `ScriptRunner::Run`. The result script has no owner quest, so on a guest only the
//!   apply scope can keep it from running.
//! - `ScriptRunner::Run` counts the scripts that really ran.
//! - `AddChange` and the save's own test (`0084a620`) accept every change.
//!
//! Each [`Story`] is its own engine; two of them are two machines.

use engine_core::prelude::*;

use crate::{addr, forms};

/// Where the story lives.
const HEAP: u32 = 0x0200_0000;
const HEAP_LEN: u32 = 0x0010_0000;
/// The id-to-form table: a count, then (id, form) pairs.
const TABLE: u32 = 0x0180_0000;
const TABLE_LEN: u32 = 0x1_0000;
/// Counters: scripts run, then the last script run.
const RAN: u32 = 0x0170_0000;
/// The save/load game object the change calls pass.
const SAVE_LOAD: u32 = 0x0160_0000;
/// The quest script every quest has and the result script every stage has.
const QUEST_SCRIPT: u32 = 0x0150_0000;
const RESULT_SCRIPT: u32 = 0x0150_0100;
/// Bytes per form in the heap. A quest's slot: the form, stages at +0x80 (4 bytes each), stage list
/// nodes at +0x180, objectives at +0x400 (0x28 each), objective nodes at +0x700, its script's
/// `ScriptLocals` at +0x800, variable nodes at +0x840, variables at +0x900 (0x18 each).
const SLOT: u32 = 0x1000;

/// What a story holds: quests with their stage numbers, globals with their values.
#[derive(Debug, Clone, Default)]
pub struct StorySpec {
    /// (quest id, stage numbers in list order).
    pub quests: Vec<(u32, Vec<u8>)>,
    /// (global id, value).
    pub globals: Vec<(u32, f32)>,
    /// (quest id, objective numbers in list order).
    pub objectives: Vec<(u32, Vec<u32>)>,
    /// (quest id, its script's variable ids in list order); a quest listed here has a script.
    pub vars: Vec<(u32, Vec<u32>)>,
}

/// One machine's engine holding a [`StorySpec`].
pub struct Story {
    /// The engine; co-op is installed on it.
    pub e: Engine,
}

/// Writes a list of `n` items at the first node `head`, the later nodes from `nodes` on, 8 bytes
/// apart; `item(e, i)` makes item `i`. With no items the first node stays zero (an empty list).
fn link(
    e: &mut Engine,
    head: u32,
    nodes: u32,
    n: usize,
    mut item: impl FnMut(&mut Engine, usize) -> u32,
) {
    let mut node = head;
    for i in 0..n {
        let it = item(e, i);
        e.mem.set_u32(node, it);
        if i + 1 < n {
            let next = nodes + 8 * i as u32;
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }
}

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

fn lookup(e: &mut Engine, a: &[u32]) -> Ret {
    let n = e.mem.u32(TABLE);
    for i in 0..n {
        let at = TABLE + 4 + 8 * i;
        if e.mem.u32(at) == a[0] {
            return eax(e.mem.u32(at + 4));
        }
    }
    eax(0)
}

fn stage_of(e: &mut Engine, quest: u32, number: u8) -> Option<u32> {
    let mut node = quest + addr::QUEST_STAGE_LIST_OFFSET;
    while node != 0 {
        let stage = e.mem.u32(node);
        if stage != 0 && e.mem.u8(stage + addr::STAGE_INDEX_OFFSET) == number {
            return Some(stage);
        }
        node = e.mem.u32(node + 4);
    }
    None
}

fn set_stage_done(e: &mut Engine, a: &[u32]) -> Ret {
    let (quest, number) = (a[0], a[1] as u8);
    let Some(stage) = stage_of(e, quest, number) else {
        return eax(0);
    };
    e.mem.set_u8(stage + addr::STAGE_DONE_OFFSET, 1);
    let current = quest + addr::QUEST_CURRENT_STAGE_OFFSET;
    if e.mem.u8(current) < number {
        e.mem.set_u8(current, number);
    }
    e.call(
        addr::SAVE_LOAD_ADD_CHANGE,
        &args![SAVE_LOAD, quest, addr::CHANGE_QUEST_STAGES, 0u32],
    );
    run(e, RESULT_SCRIPT);
    eax(1)
}

fn run(e: &mut Engine, script: u32) {
    e.call(
        addr::SCRIPT_RUNNER_RUN,
        &args![0u32, script, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

fn runner(e: &mut Engine, a: &[u32]) -> Ret {
    let n = e.mem.u32(RAN);
    e.mem.set_u32(RAN, n + 1);
    e.mem.set_u32(RAN + 4, a[1]);
    eax(0)
}

impl Story {
    /// A machine holding `spec`, with co-op installed and nothing changed yet.
    pub fn new(spec: &StorySpec) -> Story {
        let mut e = Engine::new();
        e.map(HEAP, HEAP_LEN);
        e.map(TABLE, TABLE_LEN);
        e.map(RAN, 0x100);
        e.map(SAVE_LOAD, 0x100);
        e.map(QUEST_SCRIPT, 0x1000);
        e.mem.set_u8(QUEST_SCRIPT + addr::SCRIPT_IS_QUEST_OFFSET, 1);
        let mut slot = HEAP;
        let mut entries = Vec::new();
        for (id, stages) in &spec.quests {
            assert!(stages.len() <= 64, "a test quest has at most 64 stages");
            let q = slot;
            e.mem
                .set_u8(q + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_QUEST);
            e.mem.set_u32(q + addr::FORM_ID_OFFSET, *id);
            // Stages at +0x80, 4 bytes apart; list nodes after the first at +0x180, 8 apart.
            let mut node = q + addr::QUEST_STAGE_LIST_OFFSET;
            for (i, n) in stages.iter().enumerate() {
                let stage = q + 0x80 + 4 * i as u32;
                e.mem.set_u8(stage + addr::STAGE_INDEX_OFFSET, *n);
                e.mem.set_u32(node, stage);
                if i + 1 < stages.len() {
                    let next = q + 0x180 + 8 * i as u32;
                    e.mem.set_u32(node + 4, next);
                    node = next;
                }
            }
            let objs = spec
                .objectives
                .iter()
                .find(|o| o.0 == *id)
                .map_or(&[][..], |o| &o.1[..]);
            assert!(objs.len() <= 16, "a test quest has at most 16 objectives");
            link(
                &mut e,
                q + addr::QUEST_OBJECTIVE_LIST_OFFSET,
                q + 0x700,
                objs.len(),
                |e, i| {
                    let o = q + 0x400 + 0x28 * i as u32;
                    e.mem.set_u32(o + addr::OBJECTIVE_INDEX_OFFSET, objs[i]);
                    o
                },
            );
            if let Some((_, vars)) = spec.vars.iter().find(|v| v.0 == *id) {
                assert!(
                    vars.len() <= 16,
                    "a test quest script has at most 16 variables"
                );
                let locals = q + 0x800;
                e.mem.set_u32(q + addr::QUEST_SCRIPT_LOCALS_OFFSET, locals);
                // The list's first node lives outside the locals, as `m_pLocalList` points to it.
                let first = q + 0x840;
                e.mem.set_u32(locals + addr::LOCALS_LIST_OFFSET, first);
                link(&mut e, first, q + 0x848, vars.len(), |e, i| {
                    let v = q + 0x900 + 0x18 * i as u32;
                    e.mem.set_u32(v + addr::LOCAL_ID_OFFSET, vars[i]);
                    v
                });
            }
            entries.push((*id, q));
            slot += SLOT;
        }
        for (id, v) in &spec.globals {
            let g = slot;
            e.mem
                .set_u8(g + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_GLOBAL);
            e.mem.set_u32(g + addr::FORM_ID_OFFSET, *id);
            e.mem.set_f32(g + addr::GLOBAL_VALUE_OFFSET, *v);
            entries.push((*id, g));
            slot += SLOT;
        }
        assert!(slot <= HEAP + HEAP_LEN, "a test story fits its heap");
        e.mem.set_u32(TABLE, entries.len() as u32);
        for (i, (id, f)) in entries.into_iter().enumerate() {
            e.mem.set_u32(TABLE + 4 + 8 * i as u32, id);
            e.mem.set_u32(TABLE + 8 + 8 * i as u32, f);
        }
        e.register(addr::LOOKUP_FORM, lookup);
        e.register(addr::QUEST_SET_STAGE_DONE, set_stage_done);
        e.register(addr::QUEST_GET_CURRENT_STAGE, |e, a| {
            eax(u32::from(e.mem.u8(a[0] + addr::QUEST_CURRENT_STAGE_OFFSET)))
        });
        e.register(addr::SCRIPT_RUNNER_RUN, runner);
        e.register(addr::SAVE_LOAD_ADD_CHANGE, |_, _| Ret::default());
        e.register(addr::SAVE_LOAD_RECORDS_CHANGE, |_, _| eax(1));
        crate::install(&mut e);
        Story { e }
    }

    /// Sets a quest's stage as a script's `SetStage` does.
    pub fn set_stage(&mut self, quest: u32, stage: u8) -> bool {
        forms::set_quest_stage(&mut self.e, quest, stage)
    }

    /// Sets a quest's flags and flags its change, as completing or failing a quest does.
    pub fn set_quest_flags(&mut self, quest: u32, flags: u8) {
        let q = forms::quest(&mut self.e, quest).expect("a quest of the story");
        self.e.mem.set_u8(q + addr::QUEST_FLAGS_OFFSET, flags);
        self.e
            .call(addr::SAVE_LOAD_ADD_CHANGE, &args![SAVE_LOAD, q, 2u32, 0u32]);
    }

    /// Sets an objective's state as `SetObjectiveDisplayed` and `SetObjectiveCompleted` do, and
    /// flags the quest's change.
    pub fn set_objective(&mut self, quest: u32, objective: u32, state: u32) {
        let q = forms::quest(&mut self.e, quest).expect("a quest of the story");
        let o = crate::quest::list_items(&mut self.e, q + addr::QUEST_OBJECTIVE_LIST_OFFSET)
            .into_iter()
            .find(|o| self.e.mem.u32(o + addr::OBJECTIVE_INDEX_OFFSET) == objective)
            .expect("an objective of the quest");
        self.e.mem.set_u32(o + addr::OBJECTIVE_STATE_OFFSET, state);
        self.e.call(
            addr::SAVE_LOAD_ADD_CHANGE,
            &args![SAVE_LOAD, q, addr::CHANGE_QUEST_OBJECTIVES, 0u32],
        );
    }

    /// Sets a quest script variable as the quest script's `set` does, and flags the quest's
    /// change.
    pub fn set_quest_var(&mut self, quest: u32, var: u32, v: f64) {
        assert!(crate::vars::set_quest_var(
            &mut self.e,
            quest,
            var,
            v.to_bits()
        ));
        let q = forms::quest(&mut self.e, quest).expect("a quest of the story");
        self.e.call(
            addr::SAVE_LOAD_ADD_CHANGE,
            &args![SAVE_LOAD, q, addr::CHANGE_SCRIPT_LOCALS, 0u32],
        );
    }

    /// Sets a global as a script's `set` does.
    pub fn set_global(&mut self, global: u32, v: f32) {
        assert!(forms::set_global_value(&mut self.e, global, v));
    }

    /// Runs the quest script of the story (a story script): it runs on a host, not on a guest.
    pub fn run_quest_script(&mut self) {
        run(&mut self.e, QUEST_SCRIPT);
    }

    /// How many scripts really ran on this machine.
    pub fn scripts_run(&self) -> u32 {
        self.e.mem.u32(RAN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> StorySpec {
        StorySpec {
            quests: vec![(0x0010_4c0c, vec![5, 10, 20]), (0x0010_4c0d, vec![0, 100])],
            globals: vec![(0x0000_0039, 1.0), (0x0000_003a, 0.0)],
            objectives: vec![(0x0010_4c0c, vec![10, 20])],
            vars: vec![(0x0010_4c0c, vec![1, 2, 7])],
        }
    }

    #[test]
    fn a_stage_set_on_the_story_is_seen_by_the_journal_and_runs_its_result_script() {
        let mut s = Story::new(&spec());
        crate::arm(&mut s.e, true);
        assert!(s.set_stage(0x0010_4c0c, 10));
        assert!(!s.set_stage(0x0010_4c0c, 11));
        let changes = crate::drain(&mut s.e);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].form, 0x0010_4c0c);
        assert_eq!(changes[0].kind, addr::FORM_TYPE_QUEST);
        assert_eq!(s.scripts_run(), 1);
        let r = crate::quest::record(&mut s.e, 0x0010_4c0c).unwrap();
        assert_eq!((r.stage, r.done), (10, vec![10]));
    }

    #[test]
    fn globals_and_flags_are_where_co_op_reads_them() {
        let mut s = Story::new(&spec());
        assert_eq!(forms::global_value(&mut s.e, 0x39), Some(1.0));
        s.set_global(0x3a, 4.0);
        assert_eq!(forms::global_value(&mut s.e, 0x3a), Some(4.0));
        crate::arm(&mut s.e, true);
        s.set_quest_flags(0x0010_4c0d, 3);
        assert_eq!(crate::drain(&mut s.e)[0].flags, 2);
        assert_eq!(
            crate::quest::record(&mut s.e, 0x0010_4c0d).unwrap().flags,
            3
        );
    }

    #[test]
    fn objectives_and_variables_are_where_co_op_reads_them() {
        let mut s = Story::new(&spec());
        crate::arm(&mut s.e, true);
        s.set_objective(0x0010_4c0c, 20, 1);
        s.set_quest_var(0x0010_4c0c, 7, 2.5);
        let c = crate::drain(&mut s.e);
        assert_eq!(
            c[0].flags,
            addr::CHANGE_QUEST_OBJECTIVES | addr::CHANGE_SCRIPT_LOCALS
        );
        let r = crate::quest::record(&mut s.e, 0x0010_4c0c).unwrap();
        assert_eq!(r.objectives, vec![(10, 0), (20, 1)]);
        assert_eq!(
            crate::vars::quest_vars(&mut s.e, 0x0010_4c0c).unwrap(),
            vec![(1, 0), (2, 0), (7, 2.5f64.to_bits())]
        );
        // A quest without a script has no variables; a missing variable is refused.
        assert_eq!(crate::vars::quest_vars(&mut s.e, 0x0010_4c0d), Some(vec![]));
        assert!(!crate::vars::set_quest_var(&mut s.e, 0x0010_4c0c, 9, 0));
        assert_eq!(crate::vars::quest_vars(&mut s.e, 0x9999), None);
    }

    #[test]
    fn two_stories_are_two_machines() {
        let mut a = Story::new(&spec());
        let mut b = Story::new(&spec());
        crate::gate::set_guest(&mut b.e, true);
        a.run_quest_script();
        b.run_quest_script();
        assert_eq!((a.scripts_run(), b.scripts_run()), (1, 0));
        assert!(!crate::gate::is_guest(&mut a.e));
    }
}
