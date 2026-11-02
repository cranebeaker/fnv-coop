//! `fallout shared/run_0060f4a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_0060bef0::TESQuest;
use super::tesquest::TESQuestStage;

/// `005dc960`: the byte at `this + 1` (a stage's done byte).
const READ_BYTE_1: u32 = 0x005d_c960;
/// `0060d5e0`: bit 3 of the quest data flags.
const QUEST_FLAG_BIT_3: u32 = 0x0060_d5e0;
/// `00717e50`: `this + 4` (the stage's item list).
const STAGE_ITEM_LIST: u32 = 0x0071_7e50;
/// `006815c0` returns the node (its first word is the item); `00726070`
/// gives the next node.
const RETURN_THIS: u32 = 0x0068_15c0;
const WORD_AT_4: u32 = 0x0072_6070;
/// The stage item's "set done" (`0060fb60`, one stack word: the flag).
const ITEM_SET_DONE: u32 = 0x0060_fb60;

// Translated from 0060f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks a stage done (`TESQuest::SetStageDone` calls it with the quest and
/// 0; `SetStage` reaches it through that): when the stage is not done yet,
/// or `quest` is given and its bit 3 (`0060d5e0`) is set, sets the done
/// byte (+1) and runs `0060fb60` with `force` on every non-null item of the
/// stage's item list. The walk stops at a null node (no empty-node test).
pub fn fn_0060f4a0(e: &mut Engine, this: Ptr<TESQuestStage>, quest: Ptr<TESQuest>, force: u8) {
    if e.call(READ_BYTE_1, &args![this]).bool()
        && (quest == Ptr::NULL || !e.call(QUEST_FLAG_BIT_3, &args![quest]).bool())
    {
        return;
    }
    e.set(this, TESQuestStage::bIsDone, 1);
    let mut node = e.call(STAGE_ITEM_LIST, &args![this]).u32();
    while node != 0 {
        let first = e.call(RETURN_THIS, &args![node]).u32();
        let item = e.mem.u32(first);
        node = e.call(WORD_AT_4, &args![node]).u32();
        if item != 0 {
            e.call(ITEM_SET_DONE, &args![item, u32::from(force)]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0060f4a0,
        fn_0060f4a0(Ptr<TESQuestStage>, Ptr<TESQuest>, u8)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A stage with done byte `done` and items `items` (nodes of two
    /// words, the first embedded at +4).
    fn world(done: u8, bit_3: bool, items: &[u32]) -> (Engine, Ptr<TESQuestStage>) {
        let mut e = Engine::new();
        e.register(READ_BYTE_1, |e, a| eax(u32::from(e.mem.u8(a[0] + 1))));
        e.register_double(QUEST_FLAG_BIT_3, move |_, _| eax(bit_3 as u32));
        e.register(STAGE_ITEM_LIST, |_, a| eax(a[0] + 4));
        e.register(RETURN_THIS, |_, a| eax(a[0]));
        e.register(WORD_AT_4, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(ITEM_SET_DONE, |_, _| eax(0));
        let stage: Ptr<TESQuestStage> = e.new_object();
        e.set(stage, TESQuestStage::bIsDone, done);
        let mut node = stage.addr() + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.call_log = Some(vec![]);
        (e, stage)
    }

    fn set_done_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == ITEM_SET_DONE)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn marks_a_stage_done_and_sets_every_item_done() {
        let (mut e, stage) = world(0, false, &[0x100, 0, 0x300]);
        fn_0060f4a0(&mut e, stage, Ptr::new(0x9000), 1);
        assert_eq!(e.get(stage, TESQuestStage::bIsDone), 1);
        assert_eq!(set_done_calls(&e), vec![vec![0x100, 1], vec![0x300, 1]]);
        // The quest is not asked about a stage that is not done.
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|(a, _)| *a != QUEST_FLAG_BIT_3));
    }

    #[test]
    fn a_done_stage_runs_again_only_with_the_quest_bit_3() {
        let (mut e, stage) = world(1, false, &[0x100]);
        fn_0060f4a0(&mut e, stage, Ptr::new(0x9000), 0);
        assert!(set_done_calls(&e).is_empty());
        let (mut e, stage) = world(1, true, &[0x100]);
        fn_0060f4a0(&mut e, stage, Ptr::NULL, 0);
        assert!(set_done_calls(&e).is_empty());
        let (mut e, stage) = world(1, true, &[0x100]);
        fn_0060f4a0(&mut e, stage, Ptr::new(0x9000), 0);
        assert_eq!(set_done_calls(&e), vec![vec![0x100, 0]]);
    }
}
