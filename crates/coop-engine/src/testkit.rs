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

// fights: actors with health, combat and a process, and the stand-ins fights call.
#[path = "fight_kit.rs"]
pub mod fight_kit;

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
/// The object script an actor's death runs (no quest: only the apply scope holds it back).
const DEATH_SCRIPT: u32 = 0x0150_0200;
/// Set on a form table entry's id while the form is not in the game (its cell is not loaded).
const AWAY: u32 = 0x4000_0000;
/// The player, and the arena placed references come from (0x100 bytes each; the count of placed
/// references is the word at `PLACED`).
const PLAYER: u32 = 0x0140_0000;
const PLACED: u32 = 0x0141_0000;
const ARENA: u32 = 0x0141_0100;
/// Counters at `RAN + 8..`: `CenterOnCell` calls, `Disable` calls, `EndMovement` calls, `SetGhost`
/// calls.
const WARPS: u32 = RAN + 8;
const DISABLES: u32 = RAN + 12;
const END_MOVES: u32 = RAN + 16;
const GHOSTS: u32 = RAN + 20;
/// The first id a placed reference gets, as the game's dynamic ids (`ff......`).
const FIRST_PLACED_ID: u32 = 0xff00_0001;
/// More counters: the player-cannot-move answer, and the next address `operator new` hands out
/// (from `ALLOCS`).
const BUSY: u32 = RAN + 24;
const NEXT_ALLOC: u32 = RAN + 28;
const ALLOCS: u32 = 0x0142_0000;
/// The player's vtable and the stand-in its slot `0x22c` points at.
const PLAYER_VTABLE: u32 = 0x0141_f000;
const CANNOT_MOVE_FN: u32 = 0x0130_0000;
/// `TESObjectCELL::pWorldSpace` (+0xc0, as the game), and where a test world space keeps a cell of
/// its own.
const CELL_WORLD_OFFSET: u32 = 0xc0;
const WORLD_CELL_OFFSET: u32 = 0x10;
/// The form type of a world space in the test data (anything but a cell's).
const FORM_TYPE_WORLD: u8 = 0x41;
use addr::FORM_TYPE_CELL;

/// Bytes per form in the heap. A quest's slot: the form, stages at +0x80 (4 bytes each), stage list
/// nodes at +0x180, objectives at +0x400 (0x28 each), objective nodes at +0x700, its script's
/// `ScriptLocals` at +0x800, variable nodes at +0x840, variables at +0x900 (0x18 each).
const SLOT: u32 = 0x1000;
/// The nodes of the player's objective list after the first (which is in the player): a count of
/// nodes used, then the nodes, 8 bytes each.
const PIPBOY: u32 = 0x0120_0000;

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
    /// Interior cell form ids. A story with cells also has a player (in no cell until
    /// [`Story::move_player`]) and the base form 7 an avatar is placed from.
    pub cells: Vec<u32>,
    /// Exterior cells: (cell form id, its world space's form id).
    pub exteriors: Vec<(u32, u32)>,
    /// Placed NPCs' reference ids: alive, each with a script that runs when it dies.
    pub actors: Vec<u32>,
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

/// The game's `00952a20`: the objective joins the player's list unless it is in it.
fn player_update_objective(e: &mut Engine, a: &[u32]) -> Ret {
    let (player, o) = (a[0], a[1]);
    let head = player + addr::PLAYER_OBJECTIVE_LIST_OFFSET;
    if crate::quest::list_items(e, head).contains(&o) {
        return Ret::default();
    }
    if e.mem.u32(head) == 0 && e.mem.u32(head + 4) == 0 {
        e.mem.set_u32(head, o);
        return Ret::default();
    }
    let mut last = head;
    while e.mem.u32(last + 4) != 0 {
        last = e.mem.u32(last + 4);
    }
    let n = e.mem.u32(PIPBOY);
    let node = PIPBOY + 8 + 8 * n;
    e.mem.set_u32(PIPBOY, n + 1);
    e.mem.set_u32(node, o);
    e.mem.set_u32(node + 4, 0);
    e.mem.set_u32(last + 4, node);
    Ret::default()
}

/// The game's `BGSQuestObjective::SetState` as far as co-op needs it: an objective shown (state
/// 1), or leaving the dormant state, joins the player's list; the state is stored.
fn objective_set_state(e: &mut Engine, a: &[u32]) -> Ret {
    let (o, state) = (a[0], a[1]);
    let at = o + addr::OBJECTIVE_STATE_OFFSET;
    if state == 1 || e.mem.u32(at) == 0 {
        e.call(addr::PLAYER_UPDATE_OBJECTIVE, &args![PLAYER, o]);
    }
    e.mem.set_u32(at, state);
    Ret::default()
}

/// The game's `Actor::Kill` as far as co-op needs it: a living actor's life state becomes dead,
/// its change is flagged, and its death script runs.
fn actor_kill(e: &mut Engine, a: &[u32]) -> Ret {
    let actor = a[0];
    let at = actor + addr::ACTOR_LIFE_STATE_OFFSET;
    if actor == 0 || e.mem.u32(at) != 0 {
        return Ret::default();
    }
    e.mem.set_u32(at, 2);
    e.call(
        addr::SAVE_LOAD_ADD_CHANGE,
        &args![SAVE_LOAD, actor, addr::CHANGE_ACTOR_LIFESTATE, 0u32],
    );
    run(e, DEATH_SCRIPT);
    Ret::default()
}

fn run(e: &mut Engine, script: u32) {
    e.call(
        addr::SCRIPT_RUNNER_RUN,
        &args![0u32, script, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

fn bump(e: &mut Engine, at: u32) {
    let n = e.mem.u32(at);
    e.mem.set_u32(at, n + 1);
}

/// The `PlaceAtMe` worker for a count of 1 and no distance: a new reference at the anchor's place,
/// in its cell, with its 3D, a dynamic id, and its AI on (as the game makes an actor).
fn place_at_me(e: &mut Engine, a: &[u32]) -> Ret {
    let (anchor, form) = (a[0], a[1]);
    if anchor == 0 || form == 0 {
        return eax(0);
    }
    let n = e.mem.u32(PLACED);
    let r = ARENA + 0x100 * n;
    e.mem.set_u32(PLACED, n + 1);
    e.mem.set_u32(r + addr::FORM_ID_OFFSET, FIRST_PLACED_ID + n);
    for o in (0..0x1c).step_by(4) {
        let w = e.mem.u32(anchor + 0x24 + o);
        e.mem.set_u32(r + 0x24 + o, w);
    }
    let cell = e.mem.u32(anchor + addr::REF_CELL_OFFSET);
    e.mem.set_u32(r + addr::REF_CELL_OFFSET, cell);
    e.mem.set_u32(r + addr::REF_LOADED_OFFSET, 1);
    e.mem.set_u8(r + addr::ACTOR_PROCESS_ME_OFFSET, 1);
    let count = e.mem.u32(TABLE);
    e.mem.set_u32(TABLE + 4 + 8 * count, FIRST_PLACED_ID + n);
    e.mem.set_u32(TABLE + 8 + 8 * count, r);
    e.mem.set_u32(TABLE, count + 1);
    eax(r)
}

// world: the world kit (`crate::worldkit`) adds its forms to the story's form table.
/// Enters `form` under `id` in the story's id-to-form table, as a form of the game's data.
pub(crate) fn add_form(e: &mut Engine, id: u32, form: u32) {
    let count = e.mem.u32(TABLE);
    assert!(
        8 * (count + 1) < TABLE_LEN,
        "a test story's form table is full"
    );
    e.mem.set_u32(TABLE + 4 + 8 * count, id);
    e.mem.set_u32(TABLE + 8 + 8 * count, form);
    e.mem.set_u32(TABLE, count + 1);
}
// world: end

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
        e.map(PIPBOY, 0x1000);
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
        for id in &spec.actors {
            let a = slot;
            e.mem
                .set_u8(a + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_ACHR);
            e.mem.set_u32(a + addr::FORM_ID_OFFSET, *id);
            entries.push((*id, a));
            slot += SLOT;
        }
        for id in &spec.cells {
            let c = slot;
            e.mem.set_u8(c + addr::FORM_TYPE_OFFSET, FORM_TYPE_CELL);
            e.mem.set_u32(c + addr::FORM_ID_OFFSET, *id);
            entries.push((*id, c));
            slot += SLOT;
        }
        for (cell, world) in &spec.exteriors {
            let (c, w) = (slot, slot + SLOT);
            e.mem.set_u8(c + addr::FORM_TYPE_OFFSET, FORM_TYPE_CELL);
            e.mem.set_u32(c + addr::FORM_ID_OFFSET, *cell);
            e.mem.set_u32(c + CELL_WORLD_OFFSET, w);
            e.mem.set_u8(w + addr::FORM_TYPE_OFFSET, FORM_TYPE_WORLD);
            e.mem.set_u32(w + addr::FORM_ID_OFFSET, *world);
            e.mem.set_u32(w + WORLD_CELL_OFFSET, c);
            entries.push((*cell, c));
            entries.push((*world, w));
            slot += 2 * SLOT;
        }
        {
            // The avatar's base form and the player, as the game has them from its start.
            e.mem.set_u32(slot + addr::FORM_ID_OFFSET, 7);
            entries.push((7, slot));
            slot += SLOT;
            e.map(PLAYER, 0x3_0000);
            e.map(addr::PLAYER_SINGLETON & !0xfff, 0x1000);
            e.mem.set_u32(PLAYER + addr::FORM_ID_OFFSET, 0x14);
            e.mem.set_u32(PLAYER, PLAYER_VTABLE);
            e.mem.set_u32(
                PLAYER_VTABLE + addr::PLAYER_CANNOT_MOVE_SLOT,
                CANNOT_MOVE_FN,
            );
            e.mem.set_u32(NEXT_ALLOC, ALLOCS);
            e.mem.set_u32(addr::PLAYER_SINGLETON, PLAYER);
            entries.push((0x14, PLAYER));
        }
        assert!(slot <= HEAP + HEAP_LEN, "a test story fits its heap");
        e.mem.set_u32(TABLE, entries.len() as u32);
        for (i, (id, f)) in entries.into_iter().enumerate() {
            e.mem.set_u32(TABLE + 4 + 8 * i as u32, id);
            e.mem.set_u32(TABLE + 8 + 8 * i as u32, f);
        }
        e.register(addr::LOOKUP_FORM, lookup);
        e.register(addr::QUEST_SET_STAGE_DONE, set_stage_done);
        e.register(addr::OBJECTIVE_SET_STATE, objective_set_state);
        e.register(addr::ACTOR_KILL, actor_kill);
        e.register(addr::PLAYER_UPDATE_OBJECTIVE, player_update_objective);
        e.register(addr::QUEST_GET_CURRENT_STAGE, |e, a| {
            eax(u32::from(e.mem.u8(a[0] + addr::QUEST_CURRENT_STAGE_OFFSET)))
        });
        e.register(addr::SCRIPT_RUNNER_RUN, runner);
        e.register(addr::SAVE_LOAD_ADD_CHANGE, |_, _| Ret::default());
        e.register(addr::SAVE_LOAD_RECORDS_CHANGE, |_, _| eax(1));
        e.register(addr::PLACE_AT_ME, place_at_me);
        e.register(addr::SET_POS_BASE, |e, a| {
            let axis = match a[1] as u8 {
                b'X' => 0,
                b'Y' => 4,
                _ => 8,
            };
            e.mem.set_u32(a[0] + addr::REF_POS_OFFSET + axis, a[2]);
            Ret::default()
        });
        e.register(addr::SET_ANGLE_BASE, |e, a| {
            let rad = f32::from_bits(a[2]).to_radians();
            e.mem.set_f32(a[0] + addr::REF_HEADING_OFFSET, rad);
            Ret::default()
        });
        e.register(addr::OPERATOR_NEW, |e, a| {
            let at = e.mem.u32(NEXT_ALLOC);
            e.mem.set_u32(NEXT_ALLOC, at + ((a[0] + 0xf) & !0xf));
            eax(at)
        });
        e.register(addr::POSITION_REQUEST_CONSTRUCT, |_, a| eax(a[0]));
        e.register(addr::REQUEST_POSITION_PLAYER, |e, a| {
            // The game's frame carries the request out: here at once.
            let (player, request) = (a[0], a[1]);
            let cell = match e.mem.u32(request) {
                0 => e.mem.u32(request + 4),
                world => e.mem.u32(world + WORLD_CELL_OFFSET),
            };
            e.mem.set_u32(player + addr::REF_CELL_OFFSET, cell);
            for o in [0, 4, 8] {
                let w = e.mem.u32(request + 8 + o);
                e.mem.set_u32(player + addr::REF_POS_OFFSET + o, w);
            }
            let heading = e.mem.u32(request + 0x1c);
            e.mem.set_u32(player + addr::REF_HEADING_OFFSET, heading);
            e.mem.set_u32(player + addr::REF_LOADED_OFFSET, 1);
            bump(e, WARPS);
            Ret::default()
        });
        e.register(CANNOT_MOVE_FN, |e, _| eax(e.mem.u32(BUSY)));
        e.register(addr::CELL_IS_INTERIOR, |e, a| {
            eax(u32::from(e.mem.u32(a[0] + CELL_WORLD_OFFSET) == 0))
        });
        e.register(addr::CELL_GET_WORLD_SPACE, |e, a| {
            eax(e.mem.u32(a[0] + CELL_WORLD_OFFSET))
        });
        e.register(addr::REF_DISABLE, |e, a| {
            let flags = e.mem.u32(a[0] + addr::FORM_FLAGS_OFFSET);
            e.mem.set_u32(
                a[0] + addr::FORM_FLAGS_OFFSET,
                flags | addr::FORM_FLAG_DISABLED,
            );
            bump(e, DISABLES);
            Ret::default()
        });
        e.register(addr::ACTOR_END_MOVEMENT, |e, _| {
            bump(e, END_MOVES);
            Ret::default()
        });
        e.register(addr::ACTOR_SET_GHOST, |e, _| {
            bump(e, GHOSTS);
            Ret::default()
        });
        // fights
        fight_kit::register(&mut e);
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
        self.e.call(addr::OBJECTIVE_SET_STATE, &args![o, state]);
        self.e.call(
            addr::SAVE_LOAD_ADD_CHANGE,
            &args![SAVE_LOAD, q, addr::CHANGE_QUEST_OBJECTIVES, 0u32],
        );
    }

    /// The player's objective list, the Pip-Boy's quest page: (quest id, objective number) in the
    /// order the game added them.
    pub fn pipboy(&mut self) -> Vec<(u32, u32)> {
        crate::quest::list_items(&mut self.e, PLAYER + addr::PLAYER_OBJECTIVE_LIST_OFFSET)
            .into_iter()
            .map(|o| {
                let q = HEAP + (o - HEAP) / SLOT * SLOT;
                (
                    self.e.mem.u32(q + addr::FORM_ID_OFFSET),
                    self.e.mem.u32(o + addr::OBJECTIVE_INDEX_OFFSET),
                )
            })
            .collect()
    }

    /// Empties the player's objective list and leaves the objectives' states as they are.
    pub fn empty_pipboy(&mut self) {
        let head = PLAYER + addr::PLAYER_OBJECTIVE_LIST_OFFSET;
        self.e.mem.set_u32(head, 0);
        self.e.mem.set_u32(head + 4, 0);
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

    /// Puts the player in `cell` at `pos`, facing `heading` (radians), with its 3D: in the world.
    pub fn move_player(&mut self, cell: u32, pos: [f32; 3], heading: f32) {
        let c = forms::form(&mut self.e, cell).expect("a cell of the story");
        self.e.mem.set_u32(PLAYER + addr::REF_CELL_OFFSET, c);
        for (i, v) in pos.into_iter().enumerate() {
            self.e
                .mem
                .set_f32(PLAYER + addr::REF_POS_OFFSET + 4 * i as u32, v);
        }
        self.e
            .mem
            .set_f32(PLAYER + addr::REF_HEADING_OFFSET, heading);
        self.e.mem.set_u32(PLAYER + addr::REF_LOADED_OFFSET, 1);
    }

    /// Makes the game say the player cannot be moved (or can again).
    pub fn set_busy(&mut self, busy: bool) {
        self.e.mem.set_u32(BUSY, u32::from(busy));
    }

    /// Makes `PlaceAtMe` fail from now on (the game refusing a base form).
    pub fn refuse_placing(&mut self) {
        self.e.register(addr::PLACE_AT_ME, |_, _| eax(0));
    }

    /// Switches every placed reference's AI back on, as loading a game does.
    pub fn wake_placed(&mut self) {
        for i in 0..self.e.mem.u32(PLACED) {
            self.e
                .mem
                .set_u8(ARENA + 0x100 * i + addr::ACTOR_PROCESS_ME_OFFSET, 1);
        }
    }

    /// The placed references: (form id, cell form id, position, AI on, disabled).
    pub fn placed(&mut self) -> Vec<(u32, u32, [f32; 3], bool, bool)> {
        (0..self.e.mem.u32(PLACED))
            .map(|i| {
                let r = ARENA + 0x100 * i;
                let cell = self.e.mem.u32(r + addr::REF_CELL_OFFSET);
                (
                    self.e.mem.u32(r + addr::FORM_ID_OFFSET),
                    self.e.mem.u32(cell + addr::FORM_ID_OFFSET),
                    [0, 4, 8].map(|o| self.e.mem.f32(r + addr::REF_POS_OFFSET + o)),
                    self.e.mem.u8(r + addr::ACTOR_PROCESS_ME_OFFSET) != 0,
                    self.e.mem.u32(r + addr::FORM_FLAGS_OFFSET) & addr::FORM_FLAG_DISABLED != 0,
                )
            })
            .collect()
    }

    /// How often the player was moved by a position request.
    pub fn warps(&self) -> u32 {
        self.e.mem.u32(WARPS)
    }

    /// Runs the script of quest `quest` (a quest script that names the quest as its owner, as every
    /// quest script in the game does): it runs on a host, and on a guest only for a quest of the
    /// player's own.
    pub fn run_script_of(&mut self, quest: u32) {
        let q = forms::quest(&mut self.e, quest).expect("a quest of the story");
        let script = q + 0xa00;
        self.e
            .mem
            .set_u32(script + addr::FORM_ID_OFFSET, quest ^ 0x00f0_0000);
        self.e.mem.set_u8(script + addr::SCRIPT_IS_QUEST_OFFSET, 1);
        self.e
            .mem
            .set_u32(script + addr::SCRIPT_OWNER_QUEST_OFFSET, q);
        run(&mut self.e, script);
    }

    /// Kills a placed actor as a lethal hit by this game's player does.
    pub fn kill(&mut self, actor: u32) {
        let a = crate::actors::actor(&mut self.e, actor).expect("an actor of the story");
        self.e
            .call(addr::ACTOR_KILL, &args![a, PLAYER, 0.0f32.to_bits()]);
    }

    /// Whether a placed actor is dead (`None`: not in this game now).
    pub fn dead(&mut self, actor: u32) -> Option<bool> {
        crate::actors::dead(&mut self.e, actor)
    }

    /// Takes a form out of the game (its cell is not loaded) or puts it back.
    pub fn set_present(&mut self, form: u32, present: bool) {
        let (from, to) = if present {
            (form | AWAY, form)
        } else {
            (form, form | AWAY)
        };
        for i in 0..self.e.mem.u32(TABLE) {
            let at = TABLE + 4 + 8 * i;
            if self.e.mem.u32(at) == from {
                self.e.mem.set_u32(at, to);
            }
        }
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
            cells: vec![0x0010_5a00, 0x0010_5a01],
            exteriors: vec![(0x0010_5b00, 0x0000_da7e)],
            actors: vec![0x0010_4f25],
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

    #[test]
    fn the_player_a_warp_and_an_avatar_are_where_co_op_reads_them() {
        use crate::live;
        let mut s = Story::new(&spec());
        // At the menu: a player in no cell.
        assert_eq!(live::local(&mut s.e), None);
        s.move_player(0x0010_5a00, [10.0, 20.0, 30.0], 1.5);
        let l = live::local(&mut s.e).unwrap();
        assert_eq!(
            (l.cell, l.pos, l.heading, l.loaded),
            (0x0010_5a00, [10.0, 20.0, 30.0], 1.5, true)
        );
        // The avatar: placed at the player, AI off, then held where it is told.
        let a = live::avatar_create(&mut s.e, 7).unwrap();
        assert_eq!(
            s.placed(),
            [(a.id, 0x0010_5a00, [10.0, 20.0, 30.0], false, false)]
        );
        s.wake_placed();
        let h = live::avatar_hold(&mut s.e, a, [1.0, 2.0, 3.0], 0.5).unwrap();
        assert_eq!((h.woke, h.pos), (true, [1.0, 2.0, 3.0]));
        assert!(
            !live::avatar_hold(&mut s.e, a, [1.0, 2.0, 3.0], 0.5)
                .unwrap()
                .woke
        );
        assert!(live::avatar_remove(&mut s.e, a));
        assert!(s.placed()[0].4);
        // The warp: to a place in a known interior; an unknown space is refused, a busy player is
        // left alone, and nothing moves.
        assert_eq!(
            live::warp(&mut s.e, 0x0010_5a01, [7.0, 8.0, 9.0], 2.0),
            Ok(())
        );
        let l = live::local(&mut s.e).unwrap();
        assert_eq!(
            (l.cell, l.pos, l.heading),
            (0x0010_5a01, [7.0, 8.0, 9.0], 2.0)
        );
        assert_eq!(
            live::warp(&mut s.e, 0x9999, [0.0; 3], 0.0),
            Err(live::Refusal::FormNotFound(0x9999))
        );
        s.set_busy(true);
        assert_eq!(
            live::warp(&mut s.e, 0x0010_5a00, [0.0; 3], 0.0),
            Err(live::Refusal::Busy)
        );
        s.set_busy(false);
        assert_eq!(s.warps(), 1);
        // An exterior: the space is its world space, which is what a warp there names.
        assert_eq!(
            live::warp(&mut s.e, 0x0000_da7e, [1.0, 1.0, 1.0], 0.0),
            Ok(())
        );
        assert_eq!(live::local(&mut s.e).unwrap().cell, 0x0000_da7e);
        s.move_player(0x0010_5b00, [3.0, 3.0, 3.0], 0.0);
        assert_eq!(live::local(&mut s.e).unwrap().cell, 0x0000_da7e);
        s.move_player(0x0010_5a00, [10.0, 20.0, 30.0], 1.5);
        // A base form the data lacks, and a game that refuses to place.
        assert_eq!(
            live::avatar_create(&mut s.e, 0x9999),
            Err(live::Refusal::FormNotFound(0x9999))
        );
        s.refuse_placing();
        assert_eq!(
            live::avatar_create(&mut s.e, 7),
            Err(live::Refusal::NotPlaced)
        );
    }
}
