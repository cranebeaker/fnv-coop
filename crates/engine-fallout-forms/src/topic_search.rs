//! The typed family `topic_search` (docs/ENGINE_CRATE.md, "Typed code"): the search for the topic
//! that holds a topic info (`TESTopic::GetOwnerTopic`), and the functions its loops call.
//!
//! Clearing a topic info's data asks for its topic, and the exe finds it by walking every topic and,
//! in each, every per-quest info list, comparing ids: about 68,000 infos a search in a load with
//! YUP. A plugin that changes dialogue clears each info it changes (YUP about
//! 5,000), so the load runs that walk thousands of times. The translations make every step of it a
//! call by address; here the steps are Rust calls once the family is on.
//!
//! Same results, the same order of visiting and the same reads as the exe, and nothing here writes
//! guest memory. `NV_TYPED=topic_search:off` runs the translations. The callees outside this family
//! are the `leaves` family's (`engine_core::typed::leaf`); with that family off they go by address.

#![deny(clippy::cast_possible_truncation)]

use crate::{entry, AbiFn, Engine, Ptr};
use engine_core::abi::OneWord;
use engine_core::typed::leaf;
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::typed_call;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(5);

/// `NV_TYPED=topic_search`. Past its gate (2026-11-07: with YUP the plugin load, process start to the
/// first main frame, 8.0-8.4 s in all six runs against 9.2-20.9 s with the family off; the main menu
/// and Continue the same within the spread: the intro sets the menu time): on unless `NV_TYPED`
/// says otherwise (`NV_TYPED=off` or `topic_search:off`).
pub static TOPIC_SEARCH: Family = Family {
    name: "topic_search",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs,
};

/// The word at `011c3f2c`: the data handler, whose list at +0x108 holds the topics.
const DATA_HANDLER: u32 = 0x011c_3f2c;

// Typed from 004612e0
/// `this + 0x108`: on the data handler, the first node of its topic list. One body for several
/// classes.
#[inline(always)]
pub fn address_plus_108(_e: &mut Engine, this: impl OneWord) -> u32 {
    this.one_word().wrapping_add(0x108)
}

// Typed from 005d8a70
/// `this + 0x2c`: on a topic, the first node of its list of per-quest entries. One body for
/// several classes.
#[inline(always)]
pub fn address_plus_2c(_e: &mut Engine, this: impl OneWord) -> u32 {
    this.one_word().wrapping_add(0x2c)
}

// Typed from 00717e50
/// `this + 4`: on a per-quest entry, its info array. One body for many classes (a process's
/// running package among them, through its vtable).
#[inline(always)]
pub fn address_plus_4(_e: &mut Engine, this: impl OneWord) -> u32 {
    this.one_word().wrapping_add(4)
}

// Typed from 00619f20
/// Info `index` of a per-quest entry (cdecl: the entry, the index): 0 for a null entry, or for an
/// index not below its array's count (unsigned); else that element. The array (`00717e50`) is
/// asked for again before the element is read, as the exe does.
#[inline]
pub fn entry_info(e: &mut Engine, entry: u32, index: u32) -> u32 {
    if entry == 0 {
        return 0;
    }
    let array = typed_call!(FAMILY, 0x0071_7e50, address_plus_4(e, entry));
    let count = typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, array));
    if index >= count {
        return 0;
    }
    let array = typed_call!(FAMILY, 0x0071_7e50, address_plus_4(e, entry));
    let element = typed_call!(
        leaf::FAMILY,
        0x0087_7a30,
        leaf::array_element(e, array, index)
    );
    e.mem.u32(element)
}

// Typed from 00619e10
/// The info with id `id` in a topic's per-quest entries (thiscall: the topic; two stack words, the
/// second read as a byte; `RET 8`). The entries' list (`005d8a70`) is walked node by node, ending
/// at a null node or one whose item is 0; in each entry with a nonzero info count, the infos are
/// searched from the last down to index 0 (signed) when `last_first` is nonzero, else from the
/// first up (unsigned). The first non-null info with that id is the answer; 0 when there is none.
pub fn find_info(e: &mut Engine, topic: u32, id: u32, last_first: u8) -> u32 {
    let mut node = typed_call!(FAMILY, 0x005d_8a70, address_plus_2c(e, topic));
    loop {
        if node == 0 {
            return 0;
        }
        let item = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, node));
        if e.mem.u32(item) == 0 {
            return 0;
        }
        let item = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, node));
        let entry = e.mem.u32(item);
        node = typed_call!(leaf::FAMILY, 0x0072_6070, leaf::word_at_4(e, node));
        let infos = typed_call!(FAMILY, 0x0071_7e50, address_plus_4(e, entry));
        let count = typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, infos));
        if count == 0 {
            continue;
        }
        if last_first != 0 {
            // The exe counts down in a signed local and stops below 0.
            let mut index = i32::from_ne_bytes(count.wrapping_sub(1).to_ne_bytes());
            while index >= 0 {
                let at = u32::from_ne_bytes(index.to_ne_bytes());
                let info = typed_call!(FAMILY, 0x0061_9f20, entry_info(e, entry, at));
                if info != 0
                    && typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, info)) == id
                {
                    return info;
                }
                index -= 1;
            }
        } else {
            let mut index = 0u32;
            while index < count {
                let info = typed_call!(FAMILY, 0x0061_9f20, entry_info(e, entry, index));
                if info != 0
                    && typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, info)) == id
                {
                    return info;
                }
                index += 1;
            }
        }
    }
}

// Typed from 0061a310
/// `TESTopic::GetOwnerTopic` (cdecl: the topic info): walks the data handler's topic list node by
/// node, ending at a null node or one whose item is 0, and answers the first topic in which the
/// first info with this info's id (`0084e3a0`, read again for each topic; searched first to last)
/// is this very info. 0 when no topic holds it.
pub fn owner_topic(e: &mut Engine, info: u32) -> u32 {
    let handler = e.mem.u32(DATA_HANDLER);
    let mut node = typed_call!(FAMILY, 0x0046_12e0, address_plus_108(e, handler));
    loop {
        if node == 0 {
            return 0;
        }
        let item = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, node));
        if e.mem.u32(item) == 0 {
            return 0;
        }
        let item = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, node));
        let topic = e.mem.u32(item);
        node = typed_call!(leaf::FAMILY, 0x0072_6070, leaf::word_at_4(e, node));
        let id = typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, info));
        if typed_call!(FAMILY, 0x0061_9e10, find_info(e, topic, id, 0u8)) == info {
            return topic;
        }
    }
}

/// The family's functions at their exe addresses.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0046_12e0, address_plus_108(Ptr) -> u32),
        entry!(0x005d_8a70, address_plus_2c(Ptr) -> u32),
        entry!(0x0071_7e50, address_plus_4(Ptr) -> u32),
        entry!(0x0061_9f20, entry_info(u32, u32) -> u32),
        entry!(0x0061_9e10, find_info(u32, u32, u8) -> u32),
        entry!(0x0061_a310, owner_topic(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    //! Each function against its translation (`engine_core::typed::diff`), on real lists and arrays
    //! in guest memory laid out as the exe's: a node is (item, next); a per-quest entry's info array
    //! is at +4, with its base at +8 of the entry and its count at +0x10; an info's id is at +0xc; a
    //! topic's entries start at a node at +0x2c, the data handler's topics at a node at +0x108.

    use super::*;
    use engine_core::typed::diff::{Diff, Input};

    /// The ids the searches look for: one held twice, one once, one nowhere.
    const TWICE: u32 = 0x77;
    const ONCE: u32 = 0x55;
    const NOWHERE: u32 = 0x99;

    fn info(e: &mut Engine, id: u32) -> u32 {
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p + 0xc, id);
        p
    }

    /// A per-quest entry holding `infos` (0 for an empty slot).
    fn entry(e: &mut Engine, infos: &[u32]) -> u32 {
        let p = e.mem.alloc(0x14);
        let base = e.mem.alloc(4 * u32::try_from(infos.len().max(1)).unwrap());
        for (at, info) in (base..).step_by(4).zip(infos) {
            e.mem.set_u32(at, *info);
        }
        e.mem.set_u32(p + 8, base);
        e.mem.set_u32(p + 0x10, u32::try_from(infos.len()).unwrap());
        p
    }

    /// A list whose first node is embedded at `head`, holding `items` (an item 0 ends a walk).
    fn list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if i + 1 < items.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }

    fn topic(e: &mut Engine, entries: &[u32]) -> u32 {
        let t = e.mem.alloc(0x34);
        list(e, t + 0x2c, entries);
        t
    }

    /// One of four topics, by `shape`, and its infos in the order they were made: (topic, infos).
    /// 0: no entries; 1: an empty entry, then one with two infos of one id, a gap and another id;
    /// 2: an entry, an item-less node, then an entry the walk never reaches; 3: two entries, the
    /// second holding the twice-held id once more.
    fn topic_of_shape(e: &mut Engine, shape: u32) -> (u32, Vec<u32>) {
        let a = info(e, TWICE);
        let b = info(e, TWICE);
        let c = info(e, ONCE);
        let entries = match shape {
            0 => vec![],
            1 => {
                let empty = entry(e, &[]);
                let full = entry(e, &[c, 0, a, b]);
                vec![empty, full]
            }
            2 => {
                let first = entry(e, &[c]);
                let unreached = entry(e, &[a, b]);
                vec![first, 0, unreached]
            }
            _ => {
                let first = entry(e, &[0, a]);
                let second = entry(e, &[c, b]);
                vec![first, second]
            }
        };
        let t = topic(e, &entries);
        (t, vec![a, b, c])
    }

    #[test]
    fn address_plus_108_as_translated() {
        Diff::new(entry!(0x0046_12e0, address_plus_108(Ptr) -> u32))
            .family(&TOPIC_SEARCH)
            .inputs(&[Input::Of(&[0, 1, 0x0040_0000, 0x7fff_ffff, 0xffff_fef7])])
            .setup(|_, case| vec![case.word(0)])
            .run()
            .assert_same();
    }

    #[test]
    fn address_plus_2c_as_translated() {
        Diff::new(entry!(0x005d_8a70, address_plus_2c(Ptr) -> u32))
            .family(&TOPIC_SEARCH)
            .inputs(&[Input::Of(&[0, 1, 0x0040_0000, 0x7fff_ffff, 0xffff_ffd3])])
            .setup(|_, case| vec![case.word(0)])
            .run()
            .assert_same();
    }

    #[test]
    fn address_plus_4_as_translated() {
        Diff::new(entry!(0x0071_7e50, address_plus_4(Ptr) -> u32))
            .family(&TOPIC_SEARCH)
            .inputs(&[Input::Of(&[0, 1, 0x0040_0000, 0x7fff_ffff, 0xffff_fffb])])
            .setup(|_, case| vec![case.word(0)])
            .run()
            .assert_same();
    }

    #[test]
    fn entry_info_as_translated() {
        Diff::new(entry!(0x0061_9f20, entry_info(u32, u32) -> u32))
            .family(&TOPIC_SEARCH)
            .family(&leaf::LEAVES)
            // No entry, an empty one, one with a gap.
            .inputs(&[
                Input::Of(&[0, 1, 2]),
                Input::Of(&[0, 1, 2, 3, 0x7fff_ffff, 0x8000_0000, u32::MAX]),
            ])
            .setup(|e, case| {
                let a = info(e, TWICE);
                let b = info(e, ONCE);
                let object = match case.word(0) {
                    0 => 0,
                    1 => entry(e, &[]),
                    _ => entry(e, &[a, 0, b]),
                };
                vec![object, case.word(1)]
            })
            .run()
            .assert_same();
    }

    #[test]
    fn find_info_as_translated() {
        Diff::new(entry!(0x0061_9e10, find_info(u32, u32, u8) -> u32))
            .family(&TOPIC_SEARCH)
            .family(&leaf::LEAVES)
            .inputs(&[
                Input::Of(&[0, 1, 2, 3]),
                Input::Of(&[TWICE, ONCE, NOWHERE]),
                // Only the flag word's low byte counts.
                Input::Of(&[0, 1, 0x100]),
            ])
            .setup(|e, case| {
                let (t, _) = topic_of_shape(e, case.word(0));
                vec![t, case.word(1), case.word(2)]
            })
            .run()
            .assert_same();
    }

    #[test]
    fn owner_topic_as_translated() {
        Diff::new(entry!(0x0061_a310, owner_topic(u32) -> u32))
            .family(&TOPIC_SEARCH)
            .family(&leaf::LEAVES)
            .translation_calls_plainly(&[0x0061_9e10])
            .inputs(&[
                // Which info is asked about: one held behind its twin, the twin, one held once,
                // one in no topic, one behind an item-less node.
                Input::Of(&[0, 1, 2, 3, 4]),
                // The handler's list: empty, or the topics.
                Input::Of(&[0, 1]),
            ])
            .setup(|e, case| {
                let (first, infos) = topic_of_shape(e, 1);
                let (second, more) = topic_of_shape(e, 3);
                let (unreached, hidden) = topic_of_shape(e, 1);
                let lost = info(e, NOWHERE);
                let handler = e.mem.alloc(0x110);
                if case.word(1) != 0 {
                    list(e, handler + 0x108, &[first, second, 0, unreached]);
                }
                e.map(DATA_HANDLER, 4);
                e.mem.set_u32(DATA_HANDLER, handler);
                let asked = match case.word(0) {
                    0 => more[1],
                    1 => infos[0],
                    2 => infos[2],
                    3 => lost,
                    _ => hidden[1],
                };
                vec![asked]
            })
            .run()
            .assert_same();
    }
}
