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

// vats: the game's V.A.T.S. mode, menu stack and time multiplier.
#[path = "vats_kit.rs"]
pub mod vats_kit;

// companions: the teammate flag, the wheel's switches and the follow steps.
#[path = "companion_kit.rs"]
pub mod companion_kit;

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
/// An object script whose body sets a quest's stage (the quest and stage at [`SETTER`]).
const SETTER_SCRIPT: u32 = 0x0150_0300;
const SETTER: u32 = RAN + 0x40;
/// An object script whose body is `evp` on the reference it runs on.
const EVP_SCRIPT: u32 = 0x0150_0400;
/// An object script with a form id of its own (`VCG02TargetSCRIPT`'s) that sets a stage, counts
/// in a quest's variable and switches its reference off.
const BOTTLE_SCRIPT: u32 = 0x0150_0600;
/// An object script whose body is `Disable` on the reference it runs on (a trigger that spends
/// itself).
const SPEND_SCRIPT: u32 = 0x0150_0500;
/// A script whose body is `RewardXP`: a stage's result when it names an owner quest, an object
/// script when it does not.
const REWARD_SCRIPT: u32 = 0x0150_0700;
/// In any script's block: the experience its body gives the player (`RewardXP`), 0 for none.
pub(crate) const SCRIPT_REWARD_XP: u32 = 0x7c;
/// The experience the player was given in all.
const REWARD: u32 = RAN + 0x50;
/// Two cells (their addresses) this game does not have loaded; 0 for none.
const UNLOADED: u32 = RAN + 0x58;
/// In any script's block: an item its body gives (`AddItem`; its form id, 0 for none), how many,
/// and to whom (a reference's address; 0 for the player).
pub(crate) const SCRIPT_ADD_ITEM: u32 = 0x68;
/// In any script's block: a second item and its count in the same container (a form list's
/// elements, as `AddItem` gives them), 0 for none.
const SCRIPT_ADD_ITEM_TOO: u32 = 0x60;
/// Items: the next free byte of the arena the containers' lists take, item forms from
/// [`BAG_FORMS`], the player's inventory (item, count pairs) from [`BAG_INVENTORY`].
const BAG: u32 = 0x0172_0000;
const BAG_LEN: u32 = 0x4000;
const BAG_FORMS: u32 = BAG + 0x2000;
const BAG_INVENTORY: u32 = BAG + 0x3000;
/// The form id of the object script [`Story::run_object_script_setting_stage`] runs.
pub const SETTER_SCRIPT_ID: u32 = 0x0015_0001;
/// The form id of the target's script [`Story::shoot_target`] runs (`VCG02TargetSCRIPT`'s).
pub const BOTTLE_SCRIPT_ID: u32 = 0x0010_a1ef;
/// Where a reference's primitive is kept, in its own block (a trigger volume has one).
const PRIMITIVE_OFFSET: u32 = 0xeb0;
/// Where an actor's being in a conversation is kept, in its own block.
const DIALOGUE_OFFSET: u32 = 0xec0;
/// Where an actor's count of package evaluations is kept, in its own block.
const EVALUATIONS_OFFSET: u32 = 0xe80;
/// Where an actor's conversations are kept, in its own block: how many it started, then the
/// other's and the topic's addresses of the last.
const CONVERSATIONS_OFFSET: u32 = 0xe90;
/// A `StartConversation` command's parameters as the script data would hold them (target, topic,
/// two locations, two flags), and the offset into the script data.
/// Where an actor's packages are kept, in its own block: the one it runs, then the one this
/// game's own schedule and conditions would pick for it.
const PACKAGE_OFFSET: u32 = 0xea0;
/// In a package's own block: whether its procedure type is computed, and how many times it
/// began and ended for an actor.
const PROCEDURE_OFFSET: u32 = 0x100;
const BEGUN_OFFSET: u32 = 0x104;
const ENDED_OFFSET: u32 = 0x108;
/// A package's type, as `PACKAGE_TYPE` answers it (0 unless a test sets it).
const KIND_OFFSET: u32 = 0x10c;
/// A stand-in that answers 0, behind virtual slots whose effect no test reads.
pub(crate) const NOTHING_FN: u32 = 0x0170_0000;
const SCENE_ARGS: u32 = RAN + 0x60;
const SCENE_OFFSET: u32 = RAN + 0x80;
/// The stand-ins behind two of an actor's virtual slots.
const IS_MOBILE_FN: u32 = 0x0170_0100;
const START_CONVERSATION_FN: u32 = 0x0170_0280;
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
    /// Packages' form ids.
    pub packages: Vec<u32>,
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
    if a[1] == EVP_SCRIPT {
        e.call(addr::ACTOR_EVALUATE_PACKAGE, &args![a[2], 0u32, 1u32]);
    }
    if a[1] == SPEND_SCRIPT {
        e.call(addr::REF_DISABLE, &args![a[2]]);
    }
    if a[1] == BOTTLE_SCRIPT {
        // A target's script: the stage, the quest's first variable up by one, itself off.
        let (quest, stage) = (e.mem.u32(SETTER + 4), e.mem.u32(SETTER + 8) as u8);
        // (stage 0: only the count, as the first bottles do)
        if stage != 0 {
            forms::set_quest_stage(e, quest, stage);
        }
        if let Some((index, bits)) =
            crate::vars::quest_vars(e, quest).and_then(|v| v.first().copied())
        {
            let up = (f64::from_bits(bits) + 1.0).to_bits();
            crate::vars::set_quest_var(e, quest, index, up);
        }
        e.call(addr::REF_DISABLE, &args![a[2]]);
    }
    let item_at = a[1].wrapping_add(SCRIPT_ADD_ITEM);
    if a[1] != 0 && e.mem.is_mapped(item_at) && e.mem.u32(item_at) != 0 {
        let (id, count) = (e.mem.u32(item_at), e.mem.u32(item_at + 4));
        let player = match e.mem.u32(item_at + 8) {
            0 => e.mem.u32(addr::PLAYER_SINGLETON),
            to => to,
        };
        if let Some(item) = forms::form(e, id) {
            // As `AddItem` does: a container of its own, handed to the player.
            e.with_stack(addr::CONTAINER_SIZE, |e, c| {
                e.call(addr::CONTAINER_CONSTRUCT, &args![c]);
                e.call(addr::CONTAINER_ADD, &args![c, item, count, 0u32]);
                let too = a[1] + SCRIPT_ADD_ITEM_TOO;
                if let Some(second) = forms::form(e, e.mem.u32(too)) {
                    let n = e.mem.u32(too + 4);
                    e.call(addr::CONTAINER_ADD, &args![c, second, n, 0u32]);
                }
                e.call(addr::CONTAINER_GIVE, &args![c, player, 0u32]);
                e.call(addr::CONTAINER_DESTRUCT, &args![c]);
            });
        }
    }
    let xp_at = a[1].wrapping_add(SCRIPT_REWARD_XP);
    if a[1] != 0 && e.mem.is_mapped(xp_at) && e.mem.u32(xp_at) != 0 {
        let player = e.mem.u32(addr::PLAYER_SINGLETON);
        let xp = e.mem.u32(xp_at);
        e.call(addr::PLAYER_REWARD_EXPERIENCE, &args![player, xp]);
    }
    if a[1] == SETTER_SCRIPT {
        let (quest, stage) = (e.mem.u32(SETTER + 4), e.mem.u32(SETTER + 8) as u8);
        forms::set_quest_stage(e, quest, stage);
    }
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
                    // `pOwnerQuest`.
                    e.mem.set_u32(o + addr::OBJECTIVE_QUEST_OFFSET, q);
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
        for id in &spec.packages {
            e.mem
                .set_u8(slot + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_PACKAGE);
            e.mem.set_u32(slot + addr::FORM_ID_OFFSET, *id);
            entries.push((*id, slot));
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
        // Scenes: the command's body as the game has it (parse, then the actor's slot), the
        // parse (six values, the offset moved on by six), and the two virtual slots.
        e.register(addr::SCRIPT_PARSE_PARAMETERS, |e, a| {
            for i in 0..6 {
                let v = e.mem.u32(SCENE_ARGS + 4 * i);
                e.mem.set_u32(a[7 + i as usize], v);
            }
            let at = e.mem.u32(a[2]);
            e.mem.set_u32(a[2], at + 6);
            eax(1)
        });
        e.register(addr::CMD_START_CONVERSATION, |e, a| {
            let outs = SCENE_ARGS + 0x40;
            let mut words = args![a[0], a[1], a[7], a[2], a[3], a[4], a[5]];
            words.extend((0..6).map(|i| outs + 4 * i));
            e.call(addr::SCRIPT_PARSE_PARAMETERS, &words);
            let (target, topic) = (e.mem.u32(outs), e.mem.u32(outs + 4));
            if a[2] != 0 && target != 0 {
                e.vcall(
                    a[2],
                    addr::ACTOR_START_CONVERSATION_SLOT,
                    &args![target, 0u32, 0u32, 1u32, 0u32, 0u32, topic, 0u32, 0u32],
                );
            }
            eax(1)
        });
        // An actor is in a conversation while a test says so.
        e.register(addr::MOBILE_IS_IN_DIALOGUE, |e, a| {
            eax(e.mem.u32(a[0] + DIALOGUE_OFFSET))
        });
        // A cell is loaded unless a test let it go. The game reads the cell: none is a fault.
        e.register(addr::CELL_IS_LOADED, |e, a| {
            let cell = a[1];
            assert_ne!(cell, 0, "TES::IsCellLoaded of no cell");
            eax(u32::from(
                e.mem.u32(UNLOADED) != cell && e.mem.u32(UNLOADED + 4) != cell,
            ))
        });
        // The game's `MoveTo`: the reference goes to the target's cell, at its place plus the
        // offsets.
        e.register(addr::MOVE_TO_FUNCTION_BASE, |e, a| {
            let (r, target) = (a[0], a[1]);
            let cell = e.mem.u32(target + addr::REF_CELL_OFFSET);
            e.mem.set_u32(r + addr::REF_CELL_OFFSET, cell);
            for (i, o) in [0u32, 4, 8].into_iter().enumerate() {
                let v = e.mem.f32(target + addr::REF_POS_OFFSET + o) + f32::from_bits(a[2 + i]);
                e.mem.set_f32(r + addr::REF_POS_OFFSET + o, v);
            }
            Ret::default()
        });
        e.register(IS_MOBILE_FN, |_, _| eax(1));
        e.register(START_CONVERSATION_FN, |e, a| {
            let at = a[0] + CONVERSATIONS_OFFSET;
            bump(e, at);
            e.mem.set_u32(at + 4, a[1]);
            e.mem.set_u32(at + 8, a[7]);
            Ret::default()
        });
        for f in [
            addr::PACKAGE_LOCATION_CONSTRUCT,
            addr::PACKAGE_LOCATION_SET_REFERENCE,
            addr::PACKAGE_LOCATION_DESTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        // An evaluation picks the actor's package through the list's function, as the game's.
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |e, a| {
            let at = a[0] + EVALUATIONS_OFFSET;
            let n = e.mem.u32(at);
            e.mem.set_u32(at, n + 1);
            let picked = e
                .call(addr::PACKAGE_LIST_FIND_CURRENT, &args![0u32, a[0]])
                .u32();
            e.mem.set_u32(a[0] + PACKAGE_OFFSET, picked);
            Ret::default()
        });
        e.register(addr::PACKAGE_LIST_FIND_CURRENT, |e, a| {
            eax(e.mem.u32(a[1] + PACKAGE_OFFSET + 4))
        });
        e.register(addr::ACTOR_GET_CURRENT_EDITOR_PACKAGE, |e, a| {
            eax(e.mem.u32(a[0] + PACKAGE_OFFSET))
        });
        // A package whose procedure type is not computed yet answers -1 once.
        e.register(addr::PACKAGE_PROCEDURE_TYPE, |e, a| {
            eax(if e.mem.u32(a[0] + PROCEDURE_OFFSET) == 0 {
                u32::MAX
            } else {
                0
            })
        });
        e.register(addr::PACKAGE_CALCULATE_PROCEDURE, |e, a| {
            e.mem.set_u32(a[0] + PROCEDURE_OFFSET, 1);
            Ret::default()
        });
        e.register(addr::REF_PROCEDURE_VALUE, |_, _| eax(0));
        // What the game's own pick does besides: the flags a package's end and beginning set.
        e.register(addr::REF_GET_CURRENT_PACKAGE, |e, a| {
            eax(e.mem.u32(a[0] + PACKAGE_OFFSET))
        });
        e.register(addr::PACKAGE_TYPE, |e, a| {
            eax(e.mem.u32(a[0] + KIND_OFFSET))
        });
        e.register(addr::REF_EXTRA_LIST, |_, a| eax(a[0]));
        e.register(addr::EXTRA_REMOVE_FOLLOWER, |_, _| Ret::default());
        e.register(addr::EXTRA_GET_PRIMITIVE, |e, a| {
            eax(e.mem.u32(a[0] + PRIMITIVE_OFFSET))
        });
        e.register(addr::PACKAGE_SET_ACTION_FLAG, |e, a| {
            bump(
                e,
                a[0] + if a[2] == 0x200 {
                    BEGUN_OFFSET
                } else {
                    ENDED_OFFSET
                },
            );
            Ret::default()
        });
        e.register(addr::REF_IS_PENDING_FADE_DISABLED, |_, _| eax(0));
        e.register(addr::PLAYER_SET_CONVERSATION_PACKAGE, |_, _| Ret::default());
        e.register(NOTHING_FN, |_, _| eax(0));
        e.register(addr::PLAYER_UPDATE_OBJECTIVE, player_update_objective);
        e.register(addr::QUEST_GET_CURRENT_STAGE, |e, a| {
            eax(u32::from(e.mem.u8(a[0] + addr::QUEST_CURRENT_STAGE_OFFSET)))
        });
        e.register(addr::SCRIPT_RUNNER_RUN, runner);
        // A container: its list inline at +4 (`{item, next}` nodes, the items `{count, object,
        // extra}`) in the bag's arena; handed to the player, its items go into the inventory.
        e.map(BAG, BAG_LEN);
        e.mem.set_u32(BAG, BAG + 0x10);
        e.register(addr::CONTAINER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(addr::CONTAINER_DESTRUCT, |_, _| Ret::default());
        e.register(addr::CONTAINER_ADD, |e, a| {
            let (c, object, count) = (a[0], a[1], a[2]);
            let take = |e: &mut Engine, n: u32| {
                let at = e.mem.u32(BAG);
                assert!(at + n <= BAG_FORMS, "the test bag's arena is full");
                e.mem.set_u32(BAG, at + n);
                at
            };
            let item = take(e, 0xc);
            e.mem.set_u32(item, count);
            e.mem.set_u32(item + 4, object);
            e.mem.set_u32(item + 8, 0);
            let mut node = c + 4;
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let fresh = take(e, 8);
                e.mem.set_u32(fresh + 4, 0);
                e.mem.set_u32(node + 4, fresh);
                node = fresh;
            }
            e.mem.set_u32(node, item);
            eax(item)
        });
        e.register(addr::CONTAINER_GIVE, |e, a| {
            if a[1] != e.mem.u32(addr::PLAYER_SINGLETON) {
                return Ret::default();
            }
            let mut node = a[0] + 4;
            while node != 0 {
                let item = e.mem.u32(node);
                if item != 0 {
                    let (count, object) = (e.mem.u32(item), e.mem.u32(item + 4));
                    let mut slot = BAG_INVENTORY;
                    while e.mem.u32(slot) != 0 && e.mem.u32(slot) != object {
                        slot += 8;
                    }
                    e.mem.set_u32(slot, object);
                    let had = e.mem.u32(slot + 4);
                    e.mem.set_u32(slot + 4, had + count);
                }
                node = e.mem.u32(node + 4);
            }
            Ret::default()
        });
        // `PlayerCharacter::RewardExperience`: the experience given, counted.
        e.register(addr::PLAYER_REWARD_EXPERIENCE, |e, a| {
            let total = e.mem.u32(REWARD);
            e.mem.set_u32(REWARD, total + a[1]);
            Ret::default()
        });
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
            e.call(
                addr::SAVE_LOAD_ADD_CHANGE,
                &args![SAVE_LOAD, a[0], addr::CHANGE_FORM_FLAGS, 0u32],
            );
            Ret::default()
        });
        e.register(addr::REF_ENABLE, |e, a| {
            let flags = e.mem.u32(a[0] + addr::FORM_FLAGS_OFFSET);
            e.mem.set_u32(
                a[0] + addr::FORM_FLAGS_OFFSET,
                flags & !addr::FORM_FLAG_DISABLED,
            );
            e.call(
                addr::SAVE_LOAD_ADD_CHANGE,
                &args![SAVE_LOAD, a[0], addr::CHANGE_FORM_FLAGS, 0u32],
            );
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
        // social: the stand-ins of conversations, saves and rests.
        crate::social_kit::register(&mut e);
        // vats
        vats_kit::register(&mut e);
        // companions
        companion_kit::register(&mut e);
        crate::install(&mut e);
        Story { e }
    }

    // social
    /// Where the count of scripts that really ran is kept.
    pub(crate) const RAN_AT: u32 = RAN;

    // social
    /// Adds a form made outside the story's heap to its form table.
    pub fn add_form(&mut self, id: u32, form: u32) {
        let n = self.e.mem.u32(TABLE);
        self.e.mem.set_u32(TABLE + 4 + 8 * n, id);
        self.e.mem.set_u32(TABLE + 8 + 8 * n, form);
        self.e.mem.set_u32(TABLE, n + 1);
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

    /// Runs an object script on `reference` that sets `quest`'s stage, as a trigger's or an
    /// activator's script does (the stage's result script is tried too, as in the game).
    pub fn run_object_script_setting_stage(&mut self, reference: u32, quest: u32, stage: u8) {
        let r = forms::form(&mut self.e, reference).expect("a reference of the story");
        self.e.mem.set_u32(SETTER + 4, quest);
        self.e.mem.set_u32(SETTER + 8, u32::from(stage));
        self.e
            .mem
            .set_u32(SETTER_SCRIPT + addr::FORM_ID_OFFSET, SETTER_SCRIPT_ID);
        self.e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![0u32, SETTER_SCRIPT, r, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    /// A line's result script (no form id of its own) run on `speaker` sets `quest`'s stage, as
    /// a line of this player's dialogue whose result runs in this game does.
    pub fn run_dialogue_result_setting_stage(&mut self, speaker: u32, quest: u32, stage: u8) {
        let r = forms::form(&mut self.e, speaker).expect("a reference of the story");
        self.e.mem.set_u32(SETTER + 4, quest);
        self.e.mem.set_u32(SETTER + 8, u32::from(stage));
        self.e.mem.set_u32(SETTER_SCRIPT + addr::FORM_ID_OFFSET, 0);
        self.e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![0u32, SETTER_SCRIPT, r, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    /// A script of `quest` (a stage's or a line's result: no form id of its own, the quest its
    /// owner) gives the player `xp` experience (`RewardXP`); `quest` 0: an object script with no
    /// quest does.
    pub fn run_script_rewarding_xp(&mut self, quest: u32, xp: u32) {
        self.run_named_script_rewarding_xp(0, quest, xp);
    }

    /// As [`Story::run_script_rewarding_xp`], the script with form id `script` (a quest's script,
    /// or a script the rules let a guest run itself).
    pub fn run_named_script_rewarding_xp(&mut self, script: u32, quest: u32, xp: u32) {
        self.e
            .mem
            .set_u32(REWARD_SCRIPT + addr::FORM_ID_OFFSET, script);
        let owner = match quest {
            0 => 0,
            q => forms::quest(&mut self.e, q).expect("a quest of the story"),
        };
        self.e
            .mem
            .set_u32(REWARD_SCRIPT + addr::SCRIPT_OWNER_QUEST_OFFSET, owner);
        self.e.mem.set_u32(REWARD_SCRIPT + SCRIPT_REWARD_XP, xp);
        run(&mut self.e, REWARD_SCRIPT);
    }

    /// Makes an item form `id` of this story's data (an object a container can hold).
    pub fn add_item_form(&mut self, id: u32) {
        let n = self.e.mem.u32(BAG + 4);
        self.e.mem.set_u32(BAG + 4, n + 1);
        let form = BAG_FORMS + 0x20 * n;
        self.e.mem.set_u32(form + addr::FORM_ID_OFFSET, id);
        add_form(&mut self.e, id, form);
    }

    /// A script of `quest` (as [`Story::run_script_rewarding_xp`]) gives the player `count` of item
    /// `item` (`Player.AddItem`).
    pub fn run_script_giving_item(&mut self, quest: u32, item: u32, count: u32) {
        self.run_script_giving_item_to(quest, item, count, 0);
    }

    /// As [`Story::run_script_giving_item`], to the reference at `to` (0: the player).
    pub fn run_script_giving_item_to(&mut self, quest: u32, item: u32, count: u32, to: u32) {
        self.e.mem.set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM + 8, to);
        self.e.mem.set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM, item);
        self.e
            .mem
            .set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM + 4, count);
        self.run_named_script_rewarding_xp(0, quest, 0);
        self.e.mem.set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM, 0);
    }

    /// A script of `quest` gives the player two items at once, in one container (as `AddItem` of
    /// a form list does).
    pub fn run_script_giving_two_items(
        &mut self,
        quest: u32,
        first: (u32, u32),
        second: (u32, u32),
    ) {
        self.e
            .mem
            .set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM_TOO, second.0);
        self.e
            .mem
            .set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM_TOO + 4, second.1);
        self.run_script_giving_item(quest, first.0, first.1);
        self.e.mem.set_u32(REWARD_SCRIPT + SCRIPT_ADD_ITEM_TOO, 0);
    }

    /// How many of item `item` this game's player holds.
    pub fn items(&mut self, item: u32) -> u32 {
        let Some(form) = forms::form(&mut self.e, item) else {
            return 0;
        };
        let mut slot = BAG_INVENTORY;
        while self.e.mem.u32(slot) != 0 {
            if self.e.mem.u32(slot) == form {
                return self.e.mem.u32(slot + 4);
            }
            slot += 8;
        }
        0
    }

    /// The experience this game's player was given in all.
    pub fn xp(&self) -> u32 {
        self.e.mem.u32(REWARD)
    }

    /// A script tells an actor to evaluate its package (`evp`), as a stage's result script does.
    pub fn script_evp(&mut self, actor: u32) {
        let a = crate::actors::actor(&mut self.e, actor).expect("an actor of the story");
        self.e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![0u32, EVP_SCRIPT, a, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    /// The game's AI evaluates an actor's package by itself (no script).
    pub fn ai_evp(&mut self, actor: u32) {
        let a = crate::actors::actor(&mut self.e, actor).expect("an actor of the story");
        self.e
            .call(addr::ACTOR_EVALUATE_PACKAGE, &args![a, 0u32, 0u32]);
    }

    /// How many times an actor was told to evaluate its package.
    pub fn evaluations(&mut self, actor: u32) -> u32 {
        let a = crate::actors::actor(&mut self.e, actor).expect("an actor of the story");
        self.e.mem.u32(a + EVALUATIONS_OFFSET)
    }

    /// A script on `actor` says `StartConversation target topic`. Returns how far the script's
    /// offset moved (6 when the command's parameters were read).
    pub fn script_start_conversation(&mut self, actor: u32, target: u32, topic: u32) -> u32 {
        let form = |s: &mut Story, id: u32| forms::form(&mut s.e, id).expect("a form of the story");
        let (a, t, about) = (form(self, actor), form(self, target), form(self, topic));
        for (i, v) in [t, about, 0, 0, 1, 0].into_iter().enumerate() {
            self.e.mem.set_u32(SCENE_ARGS + 4 * i as u32, v);
        }
        self.e.mem.set_u32(SCENE_OFFSET, 0);
        self.e.call(
            addr::CMD_START_CONVERSATION,
            &args![0u32, 0u32, a, 0u32, 0u32, 0u32, 0u32, SCENE_OFFSET],
        );
        self.e.mem.u32(SCENE_OFFSET)
    }

    /// The conversations `actor` started: how many, and the ids of the other and the topic of
    /// the last.
    pub fn conversations(&mut self, actor: u32) -> (u32, u32, u32) {
        let a = forms::form(&mut self.e, actor).expect("an actor of the story");
        let at = a + CONVERSATIONS_OFFSET;
        let id = |s: &mut Story, form: u32| match form {
            0 => 0,
            f => s.e.mem.u32(f + addr::FORM_ID_OFFSET),
        };
        let (n, other, topic) = (
            self.e.mem.u32(at),
            self.e.mem.u32(at + 4),
            self.e.mem.u32(at + 8),
        );
        (n, id(self, other), id(self, topic))
    }

    /// This game's own schedule and conditions now pick `package` for `actor`, and the actor
    /// evaluates (as the game's AI does by itself).
    pub fn set_own_package(&mut self, actor: u32, package: u32) {
        let a = crate::actors::actor(&mut self.e, actor).expect("an actor of the story");
        let p = forms::form(&mut self.e, package).expect("a package of the story");
        self.e.mem.set_u32(a + PACKAGE_OFFSET + 4, p);
        self.e
            .call(addr::ACTOR_EVALUATE_PACKAGE, &args![a, 1u32, 1u32]);
    }

    /// Package `package`'s type, as the game's `PACKAGE_TYPE` answers it.
    pub fn set_package_type(&mut self, package: u32, kind: u32) {
        let p = forms::form(&mut self.e, package).expect("a package of the story");
        self.e.mem.set_u32(p + KIND_OFFSET, kind);
    }

    /// The form id of the package `actor` runs (0 for none).
    pub fn package(&mut self, actor: u32) -> u32 {
        crate::commands::current_package(&mut self.e, actor).expect("an actor of the story")
    }

    /// For a package: whether its procedure type was computed, and how many times it began and
    /// ended for an actor (the action flags the game's own pick sets).
    pub fn package_marks(&mut self, package: u32) -> (bool, u32, u32) {
        let p = forms::form(&mut self.e, package).expect("a package of the story");
        (
            self.e.mem.u32(p + PROCEDURE_OFFSET) != 0,
            self.e.mem.u32(p + BEGUN_OFFSET),
            self.e.mem.u32(p + ENDED_OFFSET),
        )
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

    /// Enables or disables a placed reference as a script's `Enable` and `Disable` do.
    pub fn set_disabled(&mut self, reference: u32, off: bool) {
        assert!(crate::actors::set_disabled(&mut self.e, reference, off));
    }

    /// Runs a target's own script on `reference` (a bottle shot): `quest`'s `stage` set, its
    /// first variable up by one, the reference switched off.
    pub fn shoot_target(&mut self, reference: u32, quest: u32, stage: u8) {
        let r = forms::form(&mut self.e, reference).expect("a reference of the story");
        self.e.mem.set_u32(SETTER + 4, quest);
        self.e.mem.set_u32(SETTER + 8, u32::from(stage));
        self.e
            .mem
            .set_u32(BOTTLE_SCRIPT + addr::FORM_ID_OFFSET, BOTTLE_SCRIPT_ID);
        self.e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![0u32, BOTTLE_SCRIPT, r, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    /// Makes a placed reference a trigger volume (it has a primitive).
    pub fn make_trigger(&mut self, reference: u32) {
        let r = forms::form(&mut self.e, reference).expect("a reference of the story");
        self.e.mem.set_u32(r + PRIMITIVE_OFFSET, 1);
    }

    /// The player walks into a trigger whose own script spends it (`Disable` on itself).
    pub fn enter_trigger(&mut self, reference: u32) {
        let r = forms::form(&mut self.e, reference).expect("a reference of the story");
        self.e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![0u32, SPEND_SCRIPT, r, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    /// Whether a placed reference is disabled (`None`: not in this game now).
    pub fn disabled(&mut self, reference: u32) -> Option<bool> {
        crate::actors::disabled(&mut self.e, reference)
    }

    /// Puts actor `id` in a conversation (its package a dialogue package), or takes it out.
    pub fn set_in_dialogue(&mut self, id: u32, talking: bool) {
        let a = crate::actors::actor(&mut self.e, id).expect("an actor of the story");
        self.e.mem.set_u32(a + DIALOGUE_OFFSET, u32::from(talking));
    }

    /// Makes this game let cell `cell` go (no longer loaded; what is in it is not drawn), or load
    /// it again. Two cells at most are let go at once.
    pub fn set_cell_loaded(&mut self, cell: u32, loaded: bool) {
        let c = forms::form(&mut self.e, cell).expect("a cell of the story");
        let slots = [UNLOADED, UNLOADED + 4];
        if loaded {
            for s in slots {
                if self.e.mem.u32(s) == c {
                    self.e.mem.set_u32(s, 0);
                }
            }
        } else if let Some(s) = slots
            .into_iter()
            .find(|s| self.e.mem.u32(*s) == 0 || self.e.mem.u32(*s) == c)
        {
            self.e.mem.set_u32(s, c);
        }
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
            packages: vec![],
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
