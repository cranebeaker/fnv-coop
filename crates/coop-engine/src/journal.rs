//! The change journal (docs/coop/SPECS.md S2): every form the game marks as changed for the save
//! game, seen at the one function all those marks go through, `BGSSaveLoadGame::AddChange`
//! (`0084a690`), wrapped as [`Owner::Coop`].
//!
//! The wrap runs the game's own call first and hands back its result unchanged, then records
//! the form's id, type and the flags it was given, but only when the game recorded the change
//! too (the test `0084a620` the call makes first: a change the save never gets is not news), and
//! only while the journal is armed: loading a save or starting a game marks thousands of forms
//! that are not news to anyone. The session drains the
//! journal once per network tick. A form marked twice in a tick is one entry with both flags.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::addr;

/// Co-op's place in front of `BGSSaveLoadGame::AddChange`.
pub(crate) const ADD_CHANGE: WrapSlot = WrapSlot::new(Owner::Coop, addr::SAVE_LOAD_ADD_CHANGE);

/// One form's changes since the last drain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    /// The form's id (`TESForm::iFormID`).
    pub form: u32,
    /// Its form type (`TESForm::cFormType`).
    pub kind: u8,
    /// The union of the change flags it was given.
    pub flags: u32,
    /// Whether any of them was forced (`TESForm::ForceChange`).
    pub forced: bool,
    /// The script that was running when the form was first changed, if one was and the host's
    /// state was not being applied ([`crate::gate::cause`]).
    pub cause: Option<crate::gate::Cause>,
}

/// The bridge's game-wide state: one value for every game thread's engine (engine-core's
/// lock-free shared slot, `Engine::ext_free`), so a change made on any thread lands here. The
/// slot itself takes no lock; the map has its own, held only while an entry is added or the map
/// taken, never while game code runs, so any thread, the viewer's included, may drain it
/// between frames.
#[derive(Debug, Default)]
pub struct Journal {
    /// `install` ran on this game.
    pub(crate) installed: AtomicBool,
    /// Changes are recorded only while this is set.
    armed: AtomicBool,
    /// Form id to (type, flags, forced), since the last drain.
    changes: Mutex<BTreeMap<u32, Entry>>,
}

/// One form's entry: type, flags, forced, cause.
type Entry = (u8, u32, bool, Option<crate::gate::Cause>);

impl Journal {
    fn map(&self) -> std::sync::MutexGuard<'_, BTreeMap<u32, Entry>> {
        // A thread that panicked while holding it left a whole map (each change is one insert).
        self.changes.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn record(
        &self,
        form: u32,
        kind: u8,
        flags: u32,
        forced: bool,
        cause: Option<crate::gate::Cause>,
    ) {
        let mut map = self.map();
        let entry = map.entry(form).or_insert((kind, 0, false, None));
        entry.1 |= flags;
        entry.2 |= forced;
        if entry.3.is_none() {
            entry.3 = cause;
        }
    }

    fn drain(&self) -> Vec<Change> {
        std::mem::take(&mut *self.map())
            .into_iter()
            .map(|(form, (kind, flags, forced, cause))| Change {
                form,
                kind,
                flags,
                forced,
                cause,
            })
            .collect()
    }
}

/// Starts or stops recording. Stopping keeps what was recorded until the next drain.
pub fn arm(e: &mut Engine, on: bool) {
    e.ext_free::<Journal>().armed.store(on, Ordering::SeqCst);
}

/// Whether the journal records.
pub fn armed(e: &mut Engine) -> bool {
    e.ext_free::<Journal>().armed.load(Ordering::SeqCst)
}

/// The changes since the last drain, by form id, and an empty journal.
pub fn drain(e: &mut Engine) -> Vec<Change> {
    e.ext_free::<Journal>().drain()
}

/// The wrap at `0084a690`. Its words are the call's: `this` (the save/load game), the form, the
/// flags and the force byte (`TESForm::AddChange` passes 0, `ForceChange` 1).
pub(crate) fn add_change(e: &mut Engine, a: &[u32]) -> Ret {
    let r = e.call_next(addr::SAVE_LOAD_ADD_CHANGE, a);
    let word = |i: usize| a.get(i).copied().unwrap_or(0);
    let (this, form, flags, force) = (word(0), word(1), word(2), word(3) & 0xff);
    if form != 0
        && armed(e)
        && e.call(addr::SAVE_LOAD_RECORDS_CHANGE, &args![this, form, force])
            .bool()
    {
        let id = e.mem.u32(form.wrapping_add(addr::FORM_ID_OFFSET));
        let kind = e.mem.u8(form.wrapping_add(addr::FORM_TYPE_OFFSET));
        let cause = crate::gate::cause(e);
        e.ext_free::<Journal>()
            .record(id, kind, flags, force != 0, cause);
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The game's save/load game object, as the words pass it.
    const SAVE_LOAD: u32 = 0x0300_0000;
    /// Three forms, 0x40 bytes apart, with ids at +0x0c.
    const FORMS: u32 = 0x0200_0000;
    /// The stand-in base's call count and its last flags.
    const BASE_LOG: u32 = 0x0100_0000;

    fn form(i: u32) -> u32 {
        FORMS + 0x40 * i
    }

    /// The stand-in for the translation at `0084a690`: counts its calls, keeps the last flags,
    /// and answers 0x5a, so the tests can see the wrap hands the game's answer back.
    fn base(e: &mut Engine, a: &[u32]) -> Ret {
        let n = e.mem.u32(BASE_LOG);
        e.mem.set_u32(BASE_LOG, n + 1);
        e.mem.set_u32(BASE_LOG + 4, a[2]);
        Ret {
            eax: 0x5a,
            ..Ret::default()
        }
    }

    /// The stand-in for the game's test `0084a620`: a form whose byte at +0x08 is set is not
    /// recorded unless forced (as the game's form flag 0x4000 or not recording at all).
    fn records(e: &mut Engine, a: &[u32]) -> Ret {
        let refused = e.mem.u8(a[1] + 8) != 0 && a[2] & 0xff == 0;
        Ret {
            eax: u32::from(!refused),
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(BASE_LOG, 0x100);
        e.map(FORMS, 0x1000);
        for (i, id) in [0x0010_4c0cu32, 0x0000_0014, 0x0100_2ad1]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(form(i as u32) + addr::FORM_ID_OFFSET, id);
        }
        for (i, kind) in [addr::FORM_TYPE_QUEST, 0x3b, 0x39].into_iter().enumerate() {
            e.mem.set_u8(form(i as u32) + addr::FORM_TYPE_OFFSET, kind);
        }
        e.register(addr::SAVE_LOAD_ADD_CHANGE, base);
        e.register(addr::SAVE_LOAD_RECORDS_CHANGE, records);
        e.register(addr::SCRIPT_RUNNER_RUN, |_, _| Ret::default());
        e
    }

    fn add(e: &mut Engine, f: u32, flags: u32, force: u32) -> Ret {
        e.call(
            addr::SAVE_LOAD_ADD_CHANGE,
            &args![SAVE_LOAD, f, flags, force],
        )
    }

    #[test]
    fn an_armed_journal_records_each_form_once_with_all_its_flags() {
        let mut e = engine();
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        arm(&mut e, true);
        add(&mut e, form(0), addr::CHANGE_QUEST_STAGES, 0);
        add(&mut e, form(1), 0x0000_0004, 0);
        add(&mut e, form(0), 0x0000_0002, 1);
        assert_eq!(
            drain(&mut e),
            [
                Change {
                    form: 0x0000_0014,
                    kind: 0x3b,
                    flags: 4,
                    forced: false,
                    cause: None,
                },
                Change {
                    form: 0x0010_4c0c,
                    kind: addr::FORM_TYPE_QUEST,
                    flags: addr::CHANGE_QUEST_STAGES | 2,
                    forced: true,
                    cause: None,
                },
            ]
        );
        // Drained: empty until the next change.
        assert!(drain(&mut e).is_empty());
        add(&mut e, form(2), 1, 0);
        assert_eq!(drain(&mut e)[0].form, 0x0100_2ad1);
    }

    #[test]
    fn the_game_call_runs_and_answers_as_without_the_wrap() {
        let mut e = engine();
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        for on in [false, true] {
            arm(&mut e, on);
            assert_eq!(add(&mut e, form(0), 0x20, 0).eax, 0x5a);
        }
        assert_eq!(e.mem.u32(BASE_LOG), 2);
        assert_eq!(e.mem.u32(BASE_LOG + 4), 0x20);
    }

    #[test]
    fn nothing_is_recorded_while_disarmed_or_for_a_null_form() {
        let mut e = engine();
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        assert!(!armed(&mut e));
        add(&mut e, form(0), 1, 0);
        arm(&mut e, true);
        add(&mut e, 0, 1, 0);
        assert!(drain(&mut e).is_empty());
        assert_eq!(e.mem.u32(BASE_LOG), 2);
    }

    #[test]
    fn a_change_the_game_does_not_record_is_not_recorded() {
        let mut e = engine();
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        arm(&mut e, true);
        e.mem.set_u8(form(2) + 8, 1);
        add(&mut e, form(2), 1, 0);
        assert!(drain(&mut e).is_empty());
        // Forced, the game records it, and so does the journal.
        add(&mut e, form(2), 1, 1);
        assert_eq!(drain(&mut e).len(), 1);
        // The game's own call ran both times.
        assert_eq!(e.mem.u32(BASE_LOG), 2);
    }

    #[test]
    fn disarming_keeps_what_was_recorded_until_the_drain() {
        let mut e = engine();
        e.register(addr::ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        crate::install(&mut e);
        arm(&mut e, true);
        add(&mut e, form(1), 8, 0);
        arm(&mut e, false);
        add(&mut e, form(2), 8, 0);
        assert_eq!(drain(&mut e).len(), 1);
    }

    /// The twin of the first test: without co-op installed the same calls record nothing, so the
    /// journal's entries come from the wrap and nowhere else.
    #[test]
    fn twin_without_the_wrap_the_journal_stays_empty() {
        let mut e = engine();
        arm(&mut e, true);
        add(&mut e, form(0), addr::CHANGE_QUEST_STAGES, 0);
        assert!(drain(&mut e).is_empty());
        assert_eq!(e.mem.u32(BASE_LOG), 1);
    }
}
