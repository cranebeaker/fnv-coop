//! The guest's script gate (docs/coop/SPECS.md S5): on a guest, the story's scripts do not run,
//! because the host runs them and their effects arrive as the host's records. A wrap of
//! `ScriptRunner::Run` (`005e0d20`), as [`Owner::Coop`] after `nvse`, skips a run when:
//!
//! - the guest is applying the host's state ([`applying`]): `SetStageDone` and the like reach the
//!   stage items' result scripts, whose effects the host already sent; or
//! - the script is the story's: a quest script (`SCRIPT_HEADER::bIsQuestScript`) or one with an
//!   owner quest (`Script::pOwnerQuest`), unless that quest is one of the player's own
//!   ([`set_own_quests`]): the quests each player runs for themself, the opening and character
//!   creation among them, are not the shared story (in the real game every quest script names its
//!   owner quest: PC run, 2026-11-10).
//!
//! The guest's own object and effect scripts keep running. A skipped run answers 0 (no
//! references changed), as a run of an empty script does. Every skip is counted, so a run on a
//! PC can show that no story script ran on the guest (M3's exit).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError, RwLock};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::addr;

/// Co-op's place in front of `ScriptRunner::Run`, after `nvse`'s.
pub(crate) const RUN: WrapSlot = WrapSlot::new(Owner::Coop, addr::SCRIPT_RUNNER_RUN);

/// The gate's game-wide state (a lock-free shared slot, as the journal's).
#[derive(Debug, Default)]
pub struct Gate {
    /// This machine is a guest: story scripts are skipped.
    guest: AtomicBool,
    /// How deep the guest is in applying the host's state (nested applies count).
    applying: AtomicU32,
    /// Runs skipped because the guest was applying the host's state.
    skipped_applying: AtomicU64,
    /// Story scripts skipped.
    skipped_story: AtomicU64,
    /// Runs of the player's own quests' scripts that went ahead.
    ran_own: AtomicU64,
    /// The form ids of the quests this player runs for themself.
    own: RwLock<BTreeSet<u32>>,
    /// The form ids of story scripts a guest runs itself whatever their quest (the rules'
    /// guest-local scripts).
    own_scripts: RwLock<BTreeSet<u32>>,
    /// The object scripts (not a quest's) that ran since [`drain_object_scripts`] last asked.
    object_ran: Mutex<BTreeSet<(u32, u32)>>,
    /// Story scripts run on a guest from inside its own doing (a stage's result).
    ran_nested: AtomicU64,
    /// The distinct story scripts skipped (at most [`SEEN_MAX`]): the script's form id to whether
    /// it is a quest script, its owner quest's form id (0 for none) and how often it was skipped.
    seen: Mutex<BTreeMap<u32, SkippedScript>>,
    /// The scripts running now, outermost first ([`cause`]).
    running: Mutex<Vec<Cause>>,
    /// Host: the actors a guest's game runs ([`set_puppets`]).
    puppets: RwLock<BTreeSet<u32>>,
    /// Host: the actors in a conversation this game started between two of them (a scene).
    scene_actors: RwLock<BTreeSet<u32>>,
    /// Host: scripts skipped because they ran on an actor a guest's game runs.
    skipped_puppet: AtomicU64,
}

/// Host: names the actors a guest's game runs. This game's copy of each is led to where that game
/// has it, its AI on so that it walks and goes through doors; the scripts it runs here (its
/// object script, its packages' scripts, its own conversations' results) are skipped while it
/// lives, because the guest's game runs them and what they do reaches the host as proposals (PC
/// run 25: Sunny's copy here, far from this player, finished her sneak package and her talk at
/// once and set the stages past them). Not skipped: a guest's line this game runs for it
/// ([`crate::talk::enact`]), the lines of a scene this game started ([`set_scene_actors`]), and
/// anything once the actor is dead (its death is the host's to run).
pub fn set_puppets(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    *e.ext_free::<Gate>()
        .puppets
        .write()
        .unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

/// Host: names the actors in a scene this game plays (see [`set_puppets`]).
pub fn set_scene_actors(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    *e.ext_free::<Gate>()
        .scene_actors
        .write()
        .unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

/// Host: how many scripts were skipped because they ran on an actor a guest's game runs.
pub fn puppet_skips(e: &mut Engine) -> u64 {
    e.ext_free::<Gate>().skipped_puppet.load(Ordering::SeqCst)
}

/// Host: whether a script run on `reference` (a pointer, 0 for none) is skipped as a puppet's.
fn puppets_own(e: &mut Engine, reference: u32) -> bool {
    if reference == 0 {
        return false;
    }
    let id = e.mem.u32(reference.wrapping_add(addr::FORM_ID_OFFSET));
    let g = e.ext_free::<Gate>();
    let puppet = g
        .puppets
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .contains(&id)
        && !g
            .scene_actors
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .contains(&id);
    puppet && !crate::talk::enacting(e) && crate::actors::dead(e, id) == Some(false)
}

/// A script that is running: what a change made meanwhile is put down to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cause {
    /// The script's form id (0 for a script without one, as a dialogue line's result).
    pub script: u32,
    /// Its owner quest's form id, 0 for none.
    pub owner_quest: u32,
    /// `SCRIPT_HEADER::bIsQuestScript`.
    pub quest_script: bool,
    /// The form id of the reference it runs on, 0 for none.
    pub reference: u32,
}

impl std::fmt::Display for Cause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match (self.quest_script, self.owner_quest != 0) {
            (true, _) => "quest script",
            (false, true) => "result script",
            (false, false) => "object script",
        };
        write!(
            f,
            "{kind} {:08x} (quest {:08x}) on reference {:08x}",
            self.script, self.owner_quest, self.reference
        )
    }
}

/// The innermost script running now, unless the host's state is being applied (a change made
/// then is the host's, not a script's of this game).
pub fn cause(e: &mut Engine) -> Option<Cause> {
    let g = e.ext_free::<Gate>();
    if g.applying.load(Ordering::SeqCst) > 0 {
        return None;
    }
    g.running
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .last()
        .copied()
}

/// Runs the script of the call `a` with itself named as the cause of what changes meanwhile.
fn run_named(e: &mut Engine, a: &[u32]) -> Ret {
    let script = a.get(1).copied().unwrap_or(0);
    let reference = a.get(2).copied().unwrap_or(0);
    let id_of = |e: &mut Engine, form: u32| match form {
        0 => 0,
        f => e.mem.u32(f.wrapping_add(addr::FORM_ID_OFFSET)),
    };
    let owner = match script {
        0 => 0,
        s => e.mem.u32(s.wrapping_add(addr::SCRIPT_OWNER_QUEST_OFFSET)),
    };
    let named = Cause {
        script: id_of(e, script),
        owner_quest: id_of(e, owner),
        quest_script: script != 0
            && e.mem.u8(script.wrapping_add(addr::SCRIPT_IS_QUEST_OFFSET)) != 0,
        reference: id_of(e, reference),
    };
    let depth = {
        let g = e.ext_free::<Gate>();
        if named.script != 0 && !named.quest_script {
            g.object_ran
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert((named.script, named.reference));
        }
        let mut s = g.running.lock().unwrap_or_else(PoisonError::into_inner);
        s.push(named);
        s.len() - 1
    };
    // Unwound or not, the stack goes back to where it was.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.call_next(addr::SCRIPT_RUNNER_RUN, a)
    }));
    e.ext_free::<Gate>()
        .running
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .truncate(depth);
    match result {
        Ok(r) => r,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// One story script a guest skipped ([`skipped_scripts`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkippedScript {
    /// `SCRIPT_HEADER::bIsQuestScript`.
    pub quest_script: bool,
    /// The owner quest's form id (`Script::pOwnerQuest`), 0 for none.
    pub owner_quest: u32,
    /// How often it was skipped.
    pub times: u64,
}

/// Most distinct scripts kept for [`skipped_scripts`].
pub const SEEN_MAX: usize = 512;

/// The distinct story scripts this guest skipped, by script form id: which quests' scripts the gate
/// holds back, for deciding which a guest should run itself (a player's own quests).
pub fn skipped_scripts(e: &mut Engine) -> Vec<(u32, SkippedScript)> {
    let g = e.ext_free::<Gate>();
    let seen = g.seen.lock().unwrap_or_else(PoisonError::into_inner);
    seen.iter().map(|(k, v)| (*k, *v)).collect()
}

/// Makes this machine a guest (story scripts gated) or not.
pub fn set_guest(e: &mut Engine, on: bool) {
    e.ext_free::<Gate>().guest.store(on, Ordering::SeqCst);
}

/// Whether this machine is a guest.
pub fn is_guest(e: &mut Engine) -> bool {
    e.ext_free::<Gate>().guest.load(Ordering::SeqCst)
}

/// Names the quests each player runs for themself (form ids): on a guest their scripts run. They
/// are not part of the shared story, so the host does not send them either.
pub fn set_own_quests(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    let g = e.ext_free::<Gate>();
    *g.own.write().unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

/// Names story scripts (form ids) a guest runs itself whatever quest owns them: the rules'
/// guest-local scripts, which act on the player whose game runs them.
pub fn set_own_scripts(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    let g = e.ext_free::<Gate>();
    *g.own_scripts
        .write()
        .unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

// social
/// The object scripts (form ids; not a quest's script, not a dialogue result) that ran in this
/// game since the last call, each with the reference it ran on.
pub fn drain_object_scripts(e: &mut Engine) -> BTreeSet<(u32, u32)> {
    std::mem::take(
        &mut *e
            .ext_free::<Gate>()
            .object_ran
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    )
}

/// Whether quest `id` is one the player runs for themself ([`set_own_quests`]).
pub fn is_own_quest(e: &mut Engine, id: u32) -> bool {
    let g = e.ext_free::<Gate>();
    g.own
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .contains(&id)
}

/// How many runs of the player's own quests' scripts went ahead on this guest.
pub fn ran_own(e: &mut Engine) -> u64 {
    e.ext_free::<Gate>().ran_own.load(Ordering::SeqCst)
}

/// Whether this game is applying another game's doing now ([`applying`]).
pub fn is_applying(e: &mut Engine) -> bool {
    e.ext_free::<Gate>().applying.load(Ordering::SeqCst) > 0
}

/// Runs `f` as the application of the host's state: no script runs on a guest until it returns.
pub fn applying<R>(e: &mut Engine, f: impl FnOnce(&mut Engine) -> R) -> R {
    e.ext_free::<Gate>().applying.fetch_add(1, Ordering::SeqCst);
    // Unwound or not, the count goes back down; an unwind (a translated C++ exception among
    // them) then goes on with its own payload.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut *e)));
    e.ext_free::<Gate>().applying.fetch_sub(1, Ordering::SeqCst);
    match result {
        Ok(r) => r,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// `(skipped while applying, story scripts skipped)` since the game started.
pub fn skipped(e: &mut Engine) -> (u64, u64) {
    let g = e.ext_free::<Gate>();
    (
        g.skipped_applying.load(Ordering::SeqCst),
        g.skipped_story.load(Ordering::SeqCst),
    )
}

/// Whether `script` is the story's: a quest script, or one with an owner quest.
fn is_story_script(e: &mut Engine, script: u32) -> bool {
    script != 0
        && (e.mem.u8(script.wrapping_add(addr::SCRIPT_IS_QUEST_OFFSET)) != 0
            || e.mem
                .u32(script.wrapping_add(addr::SCRIPT_OWNER_QUEST_OFFSET))
                != 0)
}

/// The wrap at `005e0d20`. Its words: the runner, the script, the reference, the locals, the
/// container, then the run's flags and the effect's elapsed time (`runner_run`).
pub(crate) fn run(e: &mut Engine, a: &[u32]) -> Ret {
    let g = e.ext_free::<Gate>();
    if g.guest.load(Ordering::SeqCst) {
        if g.applying.load(Ordering::SeqCst) > 0 {
            g.skipped_applying.fetch_add(1, Ordering::SeqCst);
            return Ret::default();
        }
        let script = a.get(1).copied().unwrap_or(0);
        if is_story_script(e, script) {
            let id = e.mem.u32(script.wrapping_add(addr::FORM_ID_OFFSET));
            let quest_script = e.mem.u8(script.wrapping_add(addr::SCRIPT_IS_QUEST_OFFSET)) != 0;
            let owner = e
                .mem
                .u32(script.wrapping_add(addr::SCRIPT_OWNER_QUEST_OFFSET));
            let owner_quest = match owner {
                0 => 0,
                q => e.mem.u32(q.wrapping_add(addr::FORM_ID_OFFSET)),
            };
            let g = e.ext_free::<Gate>();
            let own = (owner_quest != 0
                && g.own
                    .read()
                    .unwrap_or_else(PoisonError::into_inner)
                    .contains(&owner_quest))
                || g.own_scripts
                    .read()
                    .unwrap_or_else(PoisonError::into_inner)
                    .contains(&id);
            // Run from inside this game's own doing (an object script, a package's script, a
            // line of this player's dialogue setting a stage, whose result runs at once): the
            // stage is this player's doing, and so is its result, here, for this player (PC run
            // 23: Sunny's bark after the sneak, said by stage 40's result to the player, was the
            // host's player's, far away, and the guest's player was never told to go on). What it
            // does to the story reaches the host as proposals.
            let nested_in_own = g
                .running
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .last()
                .is_some_and(|c| !c.quest_script && c.owner_quest == 0);
            if own || nested_in_own {
                g.ran_own.fetch_add(1, Ordering::SeqCst);
                if nested_in_own && !own {
                    g.ran_nested.fetch_add(1, Ordering::SeqCst);
                }
                return run_named(e, a);
            }
            g.skipped_story.fetch_add(1, Ordering::SeqCst);
            let mut seen = g.seen.lock().unwrap_or_else(PoisonError::into_inner);
            if seen.len() < SEEN_MAX || seen.contains_key(&id) {
                seen.entry(id)
                    .or_insert(SkippedScript {
                        quest_script,
                        owner_quest,
                        times: 0,
                    })
                    .times += 1;
            }
            return Ret::default();
        }
    } else if puppets_own(e, a.get(2).copied().unwrap_or(0)) {
        e.ext_free::<Gate>()
            .skipped_puppet
            .fetch_add(1, Ordering::SeqCst);
        return Ret::default();
    }
    run_named(e, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Three scripts: an object script, a quest script, and a result script with an owner quest.
    const SCRIPTS: u32 = 0x0200_0000;
    const OBJECT: u32 = SCRIPTS;
    const QUEST: u32 = SCRIPTS + 0x100;
    const RESULT: u32 = SCRIPTS + 0x200;
    /// The stand-in runner's run count and the last script it ran.
    const RAN: u32 = 0x0100_0000;

    fn base(e: &mut Engine, a: &[u32]) -> Ret {
        let n = e.mem.u32(RAN);
        e.mem.set_u32(RAN, n + 1);
        e.mem.set_u32(RAN + 4, a[1]);
        Ret {
            eax: 1,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(SCRIPTS, 0x1000);
        e.map(RAN, 0x100);
        e.mem.set_u8(QUEST + addr::SCRIPT_IS_QUEST_OFFSET, 1);
        e.mem.set_u32(QUEST + addr::FORM_ID_OFFSET, 0x11);
        e.mem.set_u32(RESULT + addr::FORM_ID_OFFSET, 0x22);
        // The result script's owner quest: a form with id 0x77.
        e.map(0x0300_0000, 0x100);
        e.mem.set_u32(0x0300_0000 + addr::FORM_ID_OFFSET, 0x77);
        e.mem
            .set_u32(RESULT + addr::SCRIPT_OWNER_QUEST_OFFSET, 0x0300_0000);
        e.register(addr::SCRIPT_RUNNER_RUN, base);
        e.register(addr::SAVE_LOAD_ADD_CHANGE, |_, _| Ret::default());
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        e
    }

    /// The runner of [`a_guest_runs_a_story_script_its_own_object_script_called`]: counts, and
    /// the object script and the quest script each set a stage whose result runs at once.
    fn nesting(e: &mut Engine, a: &[u32]) -> Ret {
        let n = e.mem.u32(RAN);
        e.mem.set_u32(RAN, n + 1);
        if a[1] == OBJECT || a[1] == QUEST {
            run_script(e, RESULT);
        }
        Ret {
            eax: 1,
            ..Ret::default()
        }
    }

    /// PC run 23: a stage a guest's own object script sets runs its result there; the same
    /// result on its own (the host's story), or called by a quest's script, does not.
    #[test]
    fn a_guest_runs_a_story_script_its_own_object_script_called() {
        let mut e = Engine::new();
        e.map(SCRIPTS, 0x1000);
        e.map(RAN, 0x100);
        e.map(0x0300_0000, 0x200);
        e.mem.set_u8(QUEST + addr::SCRIPT_IS_QUEST_OFFSET, 1);
        e.mem.set_u32(QUEST + addr::FORM_ID_OFFSET, 0x11);
        e.mem.set_u32(RESULT + addr::FORM_ID_OFFSET, 0x22);
        e.mem.set_u32(0x0300_0000 + addr::FORM_ID_OFFSET, 0x77);
        e.mem.set_u32(0x0300_0100 + addr::FORM_ID_OFFSET, 0x11);
        e.mem
            .set_u32(RESULT + addr::SCRIPT_OWNER_QUEST_OFFSET, 0x0300_0000);
        e.mem
            .set_u32(QUEST + addr::SCRIPT_OWNER_QUEST_OFFSET, 0x0300_0100);
        e.register(addr::SCRIPT_RUNNER_RUN, nesting);
        e.register(addr::SAVE_LOAD_ADD_CHANGE, |_, _| Ret::default());
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        set_guest(&mut e, true);
        assert_eq!(run_script(&mut e, RESULT), 0);
        assert_eq!(runs(&e), 0);
        assert_eq!(run_script(&mut e, OBJECT), 1);
        assert_eq!(runs(&e), 2, "the object script and the result it called");
        // The quest's own script (the player's own quest) runs; the result it calls does not.
        set_own_quests(&mut e, [0x11]);
        run_script(&mut e, QUEST);
        assert_eq!(runs(&e), 3);
    }

    fn run_script(e: &mut Engine, script: u32) -> u32 {
        e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![
                0x0400_0000u32,
                script,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        )
        .u32()
    }

    fn runs(e: &Engine) -> u32 {
        e.mem.u32(RAN)
    }

    #[test]
    fn a_host_runs_every_script() {
        let mut e = engine();
        assert!(!is_guest(&mut e));
        for s in [OBJECT, QUEST, RESULT] {
            assert_eq!(run_script(&mut e, s), 1);
        }
        assert_eq!(runs(&e), 3);
        assert_eq!(skipped(&mut e), (0, 0));
    }

    #[test]
    fn a_guest_skips_story_scripts_and_runs_its_own() {
        let mut e = engine();
        set_guest(&mut e, true);
        assert_eq!(run_script(&mut e, QUEST), 0);
        assert_eq!(run_script(&mut e, RESULT), 0);
        assert_eq!(runs(&e), 0);
        assert_eq!(run_script(&mut e, OBJECT), 1);
        assert_eq!((runs(&e), e.mem.u32(RAN + 4)), (1, OBJECT));
        assert_eq!(skipped(&mut e), (0, 2));
        // Which scripts they were: the quest script, and the one with an owner quest.
        let seen = skipped_scripts(&mut e);
        assert_eq!(seen.len(), 2);
        assert!(seen[0].1.quest_script && seen[0].1.owner_quest == 0);
        assert!(!seen[1].1.quest_script && seen[1].1.owner_quest == 0x77);
        assert_eq!(seen[1].1.times, 1);
    }

    #[test]
    fn nothing_runs_on_a_guest_while_it_applies_the_host_state() {
        let mut e = engine();
        set_guest(&mut e, true);
        let answer = applying(&mut e, |e| {
            // Nested applies keep the gate shut until the outermost ends.
            applying(e, |e| run_script(e, OBJECT));
            run_script(e, OBJECT)
        });
        assert_eq!(answer, 0);
        assert_eq!(runs(&e), 0);
        assert_eq!(skipped(&mut e), (2, 0));
        // Applied: the guest's own scripts run again.
        assert_eq!(run_script(&mut e, OBJECT), 1);
    }

    #[test]
    fn the_gate_reopens_when_an_apply_unwinds() {
        let mut e = engine();
        set_guest(&mut e, true);
        let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            applying(&mut e, |_| panic!("an apply that fails"))
        }));
        assert!(unwound.is_err());
        assert_eq!(run_script(&mut e, OBJECT), 1);
    }

    #[test]
    fn a_guest_runs_the_scripts_of_its_own_quests_but_not_while_applying() {
        let mut e = engine();
        set_guest(&mut e, true);
        set_own_quests(&mut e, [0x77]);
        // The script whose owner quest is the player's own runs; the other quest's does not.
        assert_eq!(run_script(&mut e, RESULT), 1);
        assert_eq!(run_script(&mut e, QUEST), 0);
        assert_eq!((runs(&e), ran_own(&mut e)), (1, 1));
        assert_eq!(skipped_scripts(&mut e).len(), 1);
        // While the host's state is applied nothing runs, own or not.
        assert_eq!(applying(&mut e, |e| run_script(e, RESULT)), 0);
        assert_eq!(runs(&e), 1);
        // No longer the player's own: held back again.
        set_own_quests(&mut e, []);
        assert_eq!(run_script(&mut e, RESULT), 0);
    }

    #[test]
    fn a_guest_runs_a_script_named_guest_local_whatever_its_quest() {
        let mut e = engine();
        set_guest(&mut e, true);
        e.mem.set_u32(QUEST + addr::FORM_ID_OFFSET, 0x55);
        assert_eq!(run_script(&mut e, QUEST), 0);
        set_own_scripts(&mut e, [0x55]);
        assert_eq!(run_script(&mut e, QUEST), 1);
        assert_eq!(run_script(&mut e, RESULT), 0);
        assert_eq!(applying(&mut e, |e| run_script(e, QUEST)), 0);
    }

    /// The twin: without co-op installed a guest's story scripts run, so the skips come from the
    /// wrap.
    #[test]
    fn twin_without_the_wrap_a_guest_runs_story_scripts() {
        let mut e = Engine::new();
        e.map(SCRIPTS, 0x1000);
        e.map(RAN, 0x100);
        e.mem.set_u8(QUEST + addr::SCRIPT_IS_QUEST_OFFSET, 1);
        e.register(addr::SCRIPT_RUNNER_RUN, base);
        set_guest(&mut e, true);
        assert_eq!(run_script(&mut e, QUEST), 1);
        assert_eq!(runs(&e), 1);
    }
}
