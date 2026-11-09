//! Forms, quests and globals as co-op reads and writes them (docs/coop/SPECS.md S2), through the
//! game's own functions where the game has one, so a stage set here does what `SetStage` does.

use std::collections::BTreeMap;

use engine_core::prelude::*;

use crate::addr;

/// The form with id `id`, or `None` (`TESForm::GetFormByID`).
pub fn form(e: &mut Engine, id: u32) -> Option<u32> {
    match e.call(addr::LOOKUP_FORM, &args![id]).u32() {
        0 => None,
        f => Some(f),
    }
}

/// The form with id `id` when it is of form type `kind`.
fn form_of(e: &mut Engine, id: u32, kind: u8) -> Option<u32> {
    let f = form(e, id)?;
    (e.mem.u8(f.wrapping_add(addr::FORM_TYPE_OFFSET)) == kind).then_some(f)
}

/// A quest's current stage (`TESQuest::GetCurrentStage`), or `None` when `id` is not a quest.
pub fn quest_stage(e: &mut Engine, id: u32) -> Option<u8> {
    let q = form_of(e, id, addr::FORM_TYPE_QUEST)?;
    Some(e.call(addr::QUEST_GET_CURRENT_STAGE, &args![q]).u8())
}

/// Sets a quest's stage as `SetStage` does (`TESQuest::SetStageDone`: the stage's result scripts,
/// the current stage raised, the quest enabled, the change flagged). False when `id` is not a
/// quest or the quest has no such stage.
pub fn set_quest_stage(e: &mut Engine, id: u32, stage: u8) -> bool {
    let Some(q) = form_of(e, id, addr::FORM_TYPE_QUEST) else {
        return false;
    };
    e.call(addr::QUEST_SET_STAGE_DONE, &args![q, u32::from(stage)])
        .bool()
}

/// A global's value (`TESGlobal::fValue`), or `None` when `id` is not a global.
pub fn global_value(e: &mut Engine, id: u32) -> Option<f32> {
    let g = form_of(e, id, addr::FORM_TYPE_GLOBAL)?;
    Some(e.mem.f32(g.wrapping_add(addr::GLOBAL_VALUE_OFFSET)))
}

/// Writes a global's value; false when `id` is not a global. No change is flagged: the save keeps
/// every global in its own block, whatever the change flags say.
pub fn set_global_value(e: &mut Engine, id: u32, v: f32) -> bool {
    let Some(g) = form_of(e, id, addr::FORM_TYPE_GLOBAL) else {
        return false;
    };
    e.mem.set_f32(g.wrapping_add(addr::GLOBAL_VALUE_OFFSET), v);
    true
}

/// The globals a session replicates and the bits of each one's value when last seen. Globals do
/// not pass through the change journal, so they are compared instead.
#[derive(Debug, Default, Clone)]
pub struct GlobalWatch {
    last: BTreeMap<u32, Option<u32>>,
}

impl GlobalWatch {
    /// Watches `ids`. The first [`GlobalWatch::diff`] reports every one that exists.
    pub fn new(ids: impl IntoIterator<Item = u32>) -> GlobalWatch {
        GlobalWatch {
            last: ids.into_iter().map(|id| (id, None)).collect(),
        }
    }

    /// `(id, value)` for each watched global whose value's bits changed since the last call, by
    /// id. Bits, not values, are compared: -0.0 after 0.0 is a change, and a NaN that stays the
    /// same NaN is not. A watched id that is not a global is skipped.
    pub fn diff(&mut self, e: &mut Engine) -> Vec<(u32, f32)> {
        let mut out = Vec::new();
        for (&id, last) in self.last.iter_mut() {
            let Some(v) = global_value(e, id) else {
                continue;
            };
            if *last != Some(v.to_bits()) {
                *last = Some(v.to_bits());
                out.push((id, v));
            }
        }
        out
    }

    /// Records `v` as `id`'s value seen, as after applying the host's value on a guest, so the
    /// guest does not send it back.
    pub fn note(&mut self, id: u32, v: f32) {
        if let Some(last) = self.last.get_mut(&id) {
            *last = Some(v.to_bits());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Forms laid out 0x80 bytes apart; the stand-in lookup maps id `0x1000 + i` to form `i`.
    const FORMS: u32 = 0x0200_0000;
    /// The stand-in `SetStageDone`'s last words.
    const SET_LOG: u32 = 0x0100_0000;
    const QUEST: u32 = 0x1000;
    const GLOBAL: u32 = 0x1001;
    const OTHER: u32 = 0x1002;

    fn at(id: u32) -> u32 {
        FORMS + 0x80 * (id - 0x1000)
    }

    fn lookup(_: &mut Engine, a: &[u32]) -> Ret {
        let eax = if (0x1000..0x1003).contains(&a[0]) {
            at(a[0])
        } else {
            0
        };
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The translation reads the byte at +0x60.
    fn current_stage(e: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: u32::from(e.mem.u8(a[0] + 0x60)),
            ..Ret::default()
        }
    }

    /// Accepts stages 10 and 20 only, and records its words.
    fn set_stage_done(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(SET_LOG, a[0]);
        e.mem.set_u32(SET_LOG + 4, a[1]);
        let ok = matches!(a[1], 10 | 20);
        if ok {
            e.mem.set_u8(a[0] + 0x60, a[1] as u8);
        }
        Ret {
            eax: u32::from(ok),
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FORMS, 0x1000);
        e.map(SET_LOG, 0x100);
        e.mem
            .set_u8(at(QUEST) + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_QUEST);
        e.mem
            .set_u8(at(GLOBAL) + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_GLOBAL);
        e.mem.set_u8(at(OTHER) + addr::FORM_TYPE_OFFSET, 0x2a);
        e.register(addr::LOOKUP_FORM, lookup);
        e.register(addr::QUEST_GET_CURRENT_STAGE, current_stage);
        e.register(addr::QUEST_SET_STAGE_DONE, set_stage_done);
        e
    }

    #[test]
    fn lookup_answers_none_for_an_unknown_id() {
        let mut e = engine();
        assert_eq!(form(&mut e, QUEST), Some(at(QUEST)));
        assert_eq!(form(&mut e, 0x9999), None);
    }

    #[test]
    fn a_quest_stage_is_set_through_set_stage_done_and_read_back() {
        let mut e = engine();
        assert_eq!(quest_stage(&mut e, QUEST), Some(0));
        assert!(set_quest_stage(&mut e, QUEST, 20));
        assert_eq!(
            (e.mem.u32(SET_LOG), e.mem.u32(SET_LOG + 4)),
            (at(QUEST), 20)
        );
        assert_eq!(quest_stage(&mut e, QUEST), Some(20));
        // A stage the quest does not have: refused by the game, nothing changes.
        assert!(!set_quest_stage(&mut e, QUEST, 15));
        assert_eq!(quest_stage(&mut e, QUEST), Some(20));
    }

    #[test]
    fn quest_calls_refuse_a_form_that_is_not_a_quest() {
        let mut e = engine();
        assert_eq!(quest_stage(&mut e, GLOBAL), None);
        assert!(!set_quest_stage(&mut e, OTHER, 10));
        assert!(!set_quest_stage(&mut e, 0x9999, 10));
        // SetStageDone was never reached.
        assert_eq!(e.mem.u32(SET_LOG), 0);
    }

    #[test]
    fn a_global_is_read_and_written_at_its_value_and_only_a_global() {
        let mut e = engine();
        assert!(set_global_value(&mut e, GLOBAL, 3.5));
        assert_eq!(e.mem.f32(at(GLOBAL) + addr::GLOBAL_VALUE_OFFSET), 3.5);
        assert_eq!(global_value(&mut e, GLOBAL), Some(3.5));
        assert!(!set_global_value(&mut e, QUEST, 1.0));
        assert_eq!(global_value(&mut e, OTHER), None);
        assert_eq!(e.mem.f32(at(QUEST) + addr::GLOBAL_VALUE_OFFSET), 0.0);
    }

    #[test]
    fn the_watch_reports_each_change_once_by_bits() {
        let mut e = engine();
        let mut w = GlobalWatch::new([GLOBAL, QUEST, 0x9999]);
        // First look: the one real global, at its value.
        assert_eq!(w.diff(&mut e), [(GLOBAL, 0.0)]);
        assert!(w.diff(&mut e).is_empty());
        set_global_value(&mut e, GLOBAL, -0.0);
        assert_eq!(w.diff(&mut e).len(), 1);
        set_global_value(&mut e, GLOBAL, f32::NAN);
        assert_eq!(w.diff(&mut e).len(), 1);
        assert!(w.diff(&mut e).is_empty());
        // A value applied from the host is noted and not reported back.
        set_global_value(&mut e, GLOBAL, 7.0);
        w.note(GLOBAL, 7.0);
        assert!(w.diff(&mut e).is_empty());
    }
}
