//! The guest's script gate (docs/coop/SPECS.md S5): on a guest, the story's scripts do not run,
//! because the host runs them and their effects arrive as the host's records. A wrap of
//! `ScriptRunner::Run` (`005e0d20`), as [`Owner::Coop`] after `nvse`, skips a run when:
//!
//! - the guest is applying the host's state ([`applying`]): `SetStageDone` and the like reach the
//!   stage items' result scripts, whose effects the host already sent; or
//! - the script is the story's: a quest script (`SCRIPT_HEADER::bIsQuestScript`) or one with an
//!   owner quest (`Script::pOwnerQuest`).
//!
//! The guest's own object and effect scripts keep running. A skipped run answers 0 (no
//! references changed), as a run of an empty script does. Every skip is counted, so a run on a
//! PC can show that no story script ran on the guest (M3's exit).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

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
    /// The distinct story scripts skipped (at most [`SEEN_MAX`]): the script's form id to whether
    /// it is a quest script, its owner quest's form id (0 for none) and how often it was skipped.
    seen: Mutex<BTreeMap<u32, SkippedScript>>,
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
    }
    e.call_next(addr::SCRIPT_RUNNER_RUN, a)
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
        crate::install(&mut e);
        e
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
