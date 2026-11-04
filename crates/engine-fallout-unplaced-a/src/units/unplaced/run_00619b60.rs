//! `(unplaced)/run_00619b60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESTopic::GetTopic` (`0061a2d0`, cdecl: kind, index).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// The kind and index of the topic `00619d00` searches instead of a topic
/// whose flag bit 0 (`00619df0`) is set.
const REDIRECT_KIND: u32 = 1;
const REDIRECT_INDEX: u32 = 5;
/// `005d8a70` (`ECX` = topic): `this + 0x2c` (the list of the topic's
/// per-quest info lists).
const TOPIC_QUEST_LIST: u32 = 0x005d_8a70;
/// `006815c0` (`ECX` = node): the node itself (its first word is the item).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the word at +4 (the next node).
const NODE_NEXT: u32 = 0x0072_6070;
/// `00717e50` (`ECX` = object): `this + 4` (in a per-quest entry, its info
/// array).
const PLUS_FOUR: u32 = 0x0071_7e50;
/// `0084e3a0` (`ECX` = object): the word at +0xc (an array's count).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `00619f20` (cdecl: per-quest entry, index): the info at that index.
const ENTRY_INFO: u32 = 0x0061_9f20;

// Translated from 00619d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest under which the topic (`ECX`) holds the topic info (one stack
/// word; `RET 4`): 0 for a null info; for a topic with flag bit 0 set
/// (`00619df0`), the answer of the topic `GetTopic(1, 5)` instead; else the
/// first word (the quest) of the first per-quest entry (`005d8a70`'s list,
/// walked until a node with no item) whose infos (`00619f20`, below the
/// count `0084e3a0` of `00717e50`, unsigned) include the info. 0 if none.
/// The engine map gives it no name.
pub fn fn_00619d00(e: &mut Engine, this: Ptr, info: u32) -> u32 {
    if info == 0 {
        return 0;
    }
    if fn_00619df0(e, this) {
        let topic = e
            .call(GET_TOPIC, &args![REDIRECT_KIND, REDIRECT_INDEX])
            .ptr();
        return fn_00619d00(e, topic, info);
    }
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    loop {
        if node == 0 {
            return 0;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return 0;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let infos = e.call(PLUS_FOUR, &args![entry]).u32();
        let count = e.call(WORD_AT_0XC, &args![infos]).u32();
        let mut index = 0u32;
        while index < count {
            if e.call(ENTRY_INFO, &args![entry, index]).u32() == info {
                return e.mem.u32(entry);
            }
            index += 1;
        }
    }
}

// Translated from 00619df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the topic's (`ECX`) byte at +0x25 is set. The engine
/// map gives it no name.
pub fn fn_00619df0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x25)) & 1 != 0
}

// Translated from 00619c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic's (`ECX`) per-quest entry that holds the topic info (one stack word; `RET 4`):
/// walks `005d8a70`'s list until a node with no item, and returns the first entry (the
/// item itself, not its quest word as `00619d00` returns) whose infos (`00619f20`, below
/// the count `0084e3a0` of `00717e50`, unsigned) include `info`. 0 if none. Unlike
/// `00619d00` it neither refuses a null info nor follows the topic's flag bit 0. The
/// engine map gives it no name.
pub fn fn_00619c50(e: &mut Engine, this: Ptr, info: u32) -> u32 {
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    loop {
        if node == 0 {
            return 0;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return 0;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let infos = e.call(PLUS_FOUR, &args![entry]).u32();
        let count = e.call(WORD_AT_0XC, &args![infos]).u32();
        let mut index = 0u32;
        while index < count {
            if e.call(ENTRY_INFO, &args![entry, index]).u32() == info {
                return entry;
            }
            index += 1;
        }
    }
}

// Translated from 00619bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a topic, two words: a quest and a flag; `RET 8`;
/// the answer in `AL`): false for a null quest. Otherwise walks the topic's per-quest
/// entries (`005d8a70`'s list, stopping at a null node or a node without an item) to the
/// first whose first word is `quest`, and answers true when its info array (`00717e50`)
/// has a count (`0084e3a0`) above 0 or `flag` is clear, false when it is empty and `flag`
/// is set. False when no entry is the quest's.
pub fn fn_00619bc0(e: &mut Engine, this: Ptr, quest: u32, flag: u8) -> bool {
    if quest == 0 {
        return false;
    }
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    loop {
        if node == 0 {
            return false;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return false;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if e.mem.u32(entry) == quest {
            let infos = e.call(PLUS_FOUR, &args![entry]).u32();
            let count = e.call(WORD_AT_0XC, &args![infos]).u32();
            return count > 0 || flag == 0;
        }
    }
}

/// The `TESDataHandler` singleton pointer and `004612e0` (`ECX` = it): its topic list.
const DATA_HANDLER: u32 = 0x011c_3f2c;
const TOPIC_LIST_004612E0: u32 = 0x0046_12e0;
/// `008256d0` (`ECX` = node): whether the list is empty (`AL`).
const NODE_EMPTY: u32 = 0x0082_56d0;
/// `00619a90` (`ECX` = topic, one stack word: the quest).
const TOPIC_FOR_QUEST_00619A90: u32 = 0x0061_9a90;

// Translated from 00619b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word; called by `0060d720`): with a non-null
/// `quest`, `00619a90(quest)` on each topic of the data handler's topic list (`004612e0`),
/// up to an empty node.
pub fn fn_00619b60(e: &mut Engine, quest: u32) {
    if quest == 0 {
        return;
    }
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(TOPIC_LIST_004612E0, &args![handler]).u32();
    while node != 0 && e.call(NODE_EMPTY, &args![node]).u8() == 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let topic = e.mem.u32(item);
        e.call(TOPIC_FOR_QUEST_00619A90, &args![topic, quest]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9b60, fn_00619b60(u32)),
        entry!(0x0061_9bc0, fn_00619bc0(Ptr, u32, u8) -> bool),
        entry!(0x0061_9d00, fn_00619d00(Ptr, u32) -> u32),
        entry!(0x0061_9df0, fn_00619df0(Ptr) -> bool),
        entry!(0x0061_9c50, fn_00619c50(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles as the exe's bodies: a node's item is its first word, the
    /// next node its second; an entry's info count is at +0x10 and its
    /// infos in an array whose address is at +0x20.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ENTRY_INFO, |e, a| {
            let array = e.mem.u32(a[0] + 0x20);
            ret(e.mem.u32(array + 4 * a[1]))
        });
        e
    }

    /// A per-quest entry for `quest` with these infos.
    fn entry(e: &mut Engine, quest: u32, infos: &[u32]) -> u32 {
        let p = e.mem.alloc(0x24);
        e.mem.set_u32(p, quest);
        e.mem.set_u32(p + 0x10, infos.len() as u32);
        let array = e.mem.alloc(4 * infos.len().max(1) as u32);
        for (i, info) in infos.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *info);
        }
        e.mem.set_u32(p + 0x20, array);
        p
    }

    /// A topic whose per-quest list (at +0x2c) holds these entries.
    fn topic(e: &mut Engine, entries: &[u32]) -> u32 {
        let t = e.mem.alloc(0x40);
        let mut node = t + 0x2c;
        for (i, item) in entries.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if i + 1 < entries.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        t
    }

    #[test]
    fn flag_bit_zero() {
        let mut e = Engine::new();
        let t = e.mem.alloc(0x30);
        assert!(!e.call(0x0061_9df0, &args![t]).bool());
        e.mem.set_u8(t + 0x25, 0xfe);
        assert!(!e.call(0x0061_9df0, &args![t]).bool());
        e.mem.set_u8(t + 0x25, 0x81);
        assert!(e.call(0x0061_9df0, &args![t]).bool());
    }

    #[test]
    fn finds_the_quest_holding_the_info() {
        let mut e = setup();
        let first = entry(&mut e, 0x1111, &[0x10, 0x20]);
        let second = entry(&mut e, 0x2222, &[0x30, 0x40]);
        let t = topic(&mut e, &[first, second]);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0x40u32]).u32(), 0x2222);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0x10u32]).u32(), 0x1111);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0x50u32]).u32(), 0);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0u32]).u32(), 0);
    }

    #[test]
    fn a_flagged_topic_searches_topic_one_five() {
        let mut e = setup();
        let held = entry(&mut e, 0x3333, &[0x40]);
        let redirect = topic(&mut e, &[held]);
        e.register_double(GET_TOPIC, move |_, _| ret(redirect));
        let flagged = topic(&mut e, &[]);
        e.mem.set_u8(flagged + 0x25, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_9d00, &args![flagged, 0x40u32]).u32(), 0x3333);
        let log = e.call_log.take().unwrap();
        let topics: Vec<_> = log.iter().filter(|c| c.0 == GET_TOPIC).collect();
        assert_eq!(topics, [&(GET_TOPIC, vec![1, 5])]);
    }

    #[test]
    fn fn_00619c50_finds_the_entry_holding_the_info() {
        let mut e = setup();
        let first = entry(&mut e, 0x1111, &[0x10, 0x20]);
        let second = entry(&mut e, 0x2222, &[0x30, 0x40]);
        let t = topic(&mut e, &[first, second]);
        assert_eq!(e.call(0x0061_9c50, &args![t, 0x40u32]).u32(), second);
        assert_eq!(e.call(0x0061_9c50, &args![t, 0x10u32]).u32(), first);
        assert_eq!(e.call(0x0061_9c50, &args![t, 0x50u32]).u32(), 0);
        // An empty list (first node without an item) finds nothing.
        let empty = topic(&mut e, &[0]);
        assert_eq!(e.call(0x0061_9c50, &args![empty, 0x10u32]).u32(), 0);
    }

    #[test]
    fn fn_00619c50_ignores_the_redirect_flag_and_matches_a_null_info() {
        let mut e = setup();
        let held = entry(&mut e, 0x3333, &[0]);
        let t = topic(&mut e, &[held]);
        e.mem.set_u8(t + 0x25, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_9c50, &args![t, 0u32]).u32(), held);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != GET_TOPIC));
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// A topic whose per-quest list (at +0x2C, two-word nodes: item, next) holds entries
    /// for the quests 0x100 (two infos) and 0x200 (none).
    fn topic_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(TOPIC_QUEST_LIST, |_, a| (a[0] + 0x2c).into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PLUS_FOUR, |_, a| (a[0] + 4).into_ret());
        e.register(WORD_AT_0XC, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        let topic = e.mem.alloc(0x34);
        let second_node = e.mem.alloc(8);
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        e.mem.set_u32(first, 0x100);
        e.mem.set_u32(first + 4 + 0xc, 2);
        e.mem.set_u32(second, 0x200);
        e.mem.set_u32(topic + 0x2c, first);
        e.mem.set_u32(topic + 0x30, second_node);
        e.mem.set_u32(second_node, second);
        (e, topic)
    }

    #[test]
    fn fn_00619bc0_answers_by_the_quest_entry() {
        let (mut e, topic) = topic_engine();
        let ask = |e: &mut Engine, quest: u32, flag: u32| {
            e.call(0x0061_9bc0, &args![topic, quest, flag]).bool()
        };
        assert!(!ask(&mut e, 0, 0));
        // The quest with infos: true whatever the flag.
        assert!(ask(&mut e, 0x100, 1));
        assert!(ask(&mut e, 0x100, 0));
        // The empty entry: true only without the flag (its low byte).
        assert!(!ask(&mut e, 0x200, 1));
        assert!(ask(&mut e, 0x200, 0x100));
        // No entry for the quest: the walk ends at the null next node.
        assert!(!ask(&mut e, 0x300, 0));
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00619b60_visits_every_topic_for_the_quest() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x9000u32);
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x111);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x222);
        e.register_double(TOPIC_LIST_004612E0, move |_, _| first.into_ret());
        e.register(NODE_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TOPIC_FOR_QUEST_00619A90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_9b60, &args![0x77u32]);
        let log = e.call_log.take().unwrap();
        let visits: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == TOPIC_FOR_QUEST_00619A90)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(visits, vec![vec![0x111, 0x77], vec![0x222, 0x77]]);
        e.call_log = Some(vec![]);
        e.call(0x0061_9b60, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
