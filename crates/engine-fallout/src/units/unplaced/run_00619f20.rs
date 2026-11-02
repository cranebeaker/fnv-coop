//! `(unplaced)/run_00619f20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A bounds-checked read of a word array held at +4 of an object (the
//! topic code uses it on a topic's per-quest entries for their infos).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00717e50` (`ECX` = object): `this + 4` (the array).
const PLUS_FOUR: u32 = 0x0071_7e50;
/// `0084e3a0` (`ECX` = array): the word at +0xc (its count).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `00877a30` (`ECX` = array, one word: the index; `RET 4`): the address of
/// that element (`m_pBase + index * 4`).
const ELEMENT_ADDRESS: u32 = 0x0087_7a30;

// Translated from 00619f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (object, index): 0 without an object; otherwise the element
/// `index` of the array at +4 (`00717e50`, count `0084e3a0`, element
/// `00877a30`) when `index` is below the count (unsigned), else 0. The
/// exe's unsigned `index < 0` test never holds and is left out. The
/// engine map gives it no name.
pub fn fn_00619f20(e: &mut Engine, object: Ptr, index: u32) -> u32 {
    if object.is_null() {
        return 0;
    }
    let array = e.call(PLUS_FOUR, &args![object]).u32();
    let count = e.call(WORD_AT_0XC, &args![array]).u32();
    if index >= count {
        return 0;
    }
    let array = e.call(PLUS_FOUR, &args![object]).u32();
    let element = e.call(ELEMENT_ADDRESS, &args![array, index]).u32();
    e.mem.u32(element)
}

/// The topic counts per kind (`0119a41c`, one signed word for each of the
/// eight kinds).
const TOPIC_COUNTS: u32 = 0x0119_a41c;
/// The default-topic tables per kind (`0119a43c`, one word per kind: the
/// address of 0xC bytes per topic: the topic, a word for vtable slot 0x128,
/// and the editor ID).
const TOPIC_ARRAYS: u32 = 0x0119_a43c;
/// The number of topic kinds the two loops cover.
const TOPIC_KINDS: u32 = 8;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESTopic.cpp"`.
const TESTOPIC_SOURCE_FILE: u32 = 0x0104_d398;

/// The address of entry `index` of kind `kind`'s table.
fn topic_entry(e: &Engine, kind: u32, index: u32) -> u32 {
    let table = e.mem.u32(TOPIC_ARRAYS + 4 * kind);
    index.wrapping_mul(0xc).wrapping_add(table)
}

/// Kind `kind`'s topic count (signed).
fn topic_count(e: &Engine, kind: u32) -> i32 {
    e.mem.i32(TOPIC_COUNTS + 4 * kind)
}

// Translated from 00619fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (array): for each index below the array's count (`0084e3a0`,
/// unsigned), the element (`*00877a30(array, index)`) when not null gets
/// `0061f340` with the index's low 16 bits. Nothing for a null array.
pub fn fn_00619fe0(e: &mut Engine, array: Ptr) {
    if array.is_null() {
        return;
    }
    let count = e.call(WORD_AT_0XC, &args![array]).u32();
    let mut index = 0u32;
    while index < count {
        let at = e.call(ELEMENT_ADDRESS, &args![array, index]).u32();
        let element = e.mem.u32(at);
        if element != 0 {
            e.call(0x0061_f340, &args![element, index & 0xffff]);
        }
        index += 1;
    }
}

// Translated from 0061a040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061a040 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `cdecl`, no arguments: makes the default topics. Inside a scope timer
/// (`00404eb0` with 0x35, 1, "...\TESTopic.cpp", line 0x1573; ended by
/// `00404ee0`), every entry of the eight kinds' tables (`0119a43c`, counts
/// `0119a41c`, signed) whose topic is still null gets a new topic: 0x48
/// bytes (`00401000`) constructed for the kind (`00618580`; null when the
/// allocation fails) and stored in the entry; then vtable slot 0x128 with
/// the entry's second word and 1, slot 0x134 (`SetEditorID`) with the
/// entry's name, `00489100` on the topic + 0x18 with `00464f30(name, 0)`,
/// `00461780` on the data handler (`011c3f2c`) with the topic, and
/// `00470200` on the object at `011c54c8` with the name and the topic.
pub fn fn_0061a040(e: &mut Engine) {
    e.with_stack(4, |e, timer| {
        e.call(
            0x0040_4eb0,
            &args![timer, 0x35u32, 1u32, TESTOPIC_SOURCE_FILE, 0x1573u32],
        );
        for kind in 0..TOPIC_KINDS {
            let mut index = 0i32;
            while index < topic_count(e, kind) {
                let entry = topic_entry(e, kind, index as u32);
                if e.mem.u32(entry) == 0 {
                    let block = e.call(0x0040_1000, &args![0x48u32]).u32();
                    let topic = if block != 0 {
                        e.call(0x0061_8580, &args![block, kind]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(topic_entry(e, kind, index as u32), topic);
                    let entry = topic_entry(e, kind, index as u32);
                    let word = e.mem.u32(entry + 4);
                    let topic = e.mem.u32(entry);
                    e.vcall(topic, 0x128, &args![word, 1u32]);
                    let name = e.mem.u32(entry + 8);
                    let topic = e.mem.u32(entry);
                    e.vcall(topic, 0x134, &args![name]);
                    let name = e.mem.u32(entry + 8);
                    let text = e.call(0x0046_4f30, &args![name, 0u32]).u32();
                    let topic = e.mem.u32(entry);
                    e.call(0x0048_9100, &args![topic + 0x18, text]);
                    let topic = e.mem.u32(entry);
                    let handler = e.mem.u32(0x011c_3f2c);
                    e.call(0x0046_1780, &args![handler, topic]);
                    let topic = e.mem.u32(entry);
                    let name = e.mem.u32(entry + 8);
                    let map = e.mem.u32(0x011c_54c8);
                    e.call(0x0047_0200, &args![map, name, topic]);
                }
                index += 1;
            }
        }
        e.call(0x0040_4ee0, &args![timer]);
    });
}

// Translated from 0061a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`, no arguments: clears the topic of every entry of the eight
/// kinds' default-topic tables.
pub fn fn_0061a270(e: &mut Engine) {
    for kind in 0..TOPIC_KINDS {
        let mut index = 0i32;
        while index < topic_count(e, kind) {
            let entry = topic_entry(e, kind, index as u32);
            e.mem.set_u32(entry, 0);
            index += 1;
        }
    }
}

/// `005d8a70` (`ECX` = topic): the list of its per-quest entries; `006815c0` (`ECX` =
/// node): the address of its item word; `00726070` (`ECX` = node): the next node.
const TOPIC_QUEST_LIST: u32 = 0x005d_8a70;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 00619f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The info array (`00717e50`, +4) of the topic's per-quest entry for `quest` (unnamed in
/// the engine map; `ECX` = the topic, one stack word, `RET 4`; called by `00619a90`): the
/// first entry (up to the first null item) whose first word is `quest`, where a null
/// `quest` also needs the entry's word at +0x2c to be 0; null when none.
pub fn fn_00619f70(e: &mut Engine, this: Ptr, quest: u32) -> u32 {
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if e.mem.u32(entry) == quest && (quest != 0 || e.mem.u32(entry + 0x2c) == 0) {
            return e.call(PLUS_FOUR, &args![entry]).u32();
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9f70, fn_00619f70(Ptr, u32) -> u32),
        entry!(0x00619f20, fn_00619f20(Ptr, u32) -> u32),
        entry!(0x0061_9fe0, fn_00619fe0(Ptr)),
        entry!(0x0061_a040, fn_0061a040()),
        entry!(0x0061_a270, fn_0061a270()),
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

    #[test]
    fn reads_an_element_below_the_count() {
        let mut e = Engine::new();
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ELEMENT_ADDRESS, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 4));
        e.call_log = Some(vec![]);
        let object = e.mem.alloc(0x14);
        let base = e.mem.alloc(8);
        e.mem.set_u32(object + 8, base);
        e.mem.set_u32(object + 0x10, 2);
        e.mem.set_u32(base, 0x111);
        e.mem.set_u32(base + 4, 0x222);
        assert_eq!(e.call(0x0061_9f20, &args![object, 1u32]).u32(), 0x222);
        assert_eq!(e.call(0x0061_9f20, &args![object, 0u32]).u32(), 0x111);
        assert_eq!(e.call(0x0061_9f20, &args![object, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0061_9f20, &args![object, u32::MAX]).u32(), 0);
        assert_eq!(e.call(0x0061_9f20, &args![0u32, 0u32]).u32(), 0);
        let elements: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ELEMENT_ADDRESS)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(elements, vec![vec![object + 4, 1], vec![object + 4, 0]]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn b020_00619fe0_visits_the_elements() {
        let mut e = Engine::new();
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ELEMENT_ADDRESS, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 4));
        e.register(0x0061_f340, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0xc);
        e.mem.set_u32(array + 4, elements);
        e.mem.set_u32(array + 0xc, 3);
        e.mem.set_u32(elements, 0x111);
        e.mem.set_u32(elements + 8, 0x333);
        e.call_log = Some(vec![]);
        e.call(0x0061_9fe0, &args![array]);
        assert_eq!(
            calls_to(&e, 0x0061_f340),
            vec![vec![0x111, 0], vec![0x333, 2]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0061_9fe0, &args![0u32]);
        assert!(calls_to(&e, WORD_AT_0XC).is_empty());
    }

    /// Kind 0 has two entries, kind 3 one; the others none.
    fn tables(e: &mut Engine) -> (u32, u32) {
        e.map(0x0119_a000, 0x1000);
        e.map(0x011c_3000, 0x3000);
        let kind0 = e.mem.alloc(0x18);
        let kind3 = e.mem.alloc(0xc);
        e.mem.set_i32(TOPIC_COUNTS, 2);
        e.mem.set_i32(TOPIC_COUNTS + 12, 1);
        e.mem.set_u32(TOPIC_ARRAYS, kind0);
        e.mem.set_u32(TOPIC_ARRAYS + 12, kind3);
        (kind0, kind3)
    }

    #[test]
    fn b020_0061a270_clears_the_topics() {
        let mut e = Engine::new();
        let (kind0, kind3) = tables(&mut e);
        for p in [kind0, kind0 + 0xc, kind3] {
            e.mem.set_u32(p, 0x55);
            e.mem.set_u32(p + 4, 0x66);
        }
        e.call(0x0061_a270, &[]);
        for p in [kind0, kind0 + 0xc, kind3] {
            assert_eq!(e.mem.u32(p), 0);
            assert_eq!(e.mem.u32(p + 4), 0x66);
        }
    }

    #[test]
    fn b020_0061a040_makes_the_missing_topics() {
        let mut e = Engine::new();
        let (kind0, kind3) = tables(&mut e);
        // Kind 0's first entry already has a topic.
        e.mem.set_u32(kind0, 0x0000_9999);
        e.mem.set_u32(kind0 + 0xc + 4, 0x44);
        e.mem.set_u32(kind0 + 0xc + 8, 0x0000_7000);
        e.mem.set_u32(kind3 + 4, 0x45);
        e.mem.set_u32(kind3 + 8, 0x0000_7100);
        e.mem.set_u32(0x011c_3f2c, 0x0000_3f2c);
        e.mem.set_u32(0x011c_54c8, 0x0000_54c8);
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0128, 0x0000_1128);
        e.mem.set_u32(0x0150_0134, 0x0000_1134);
        e.register(0x0040_1000, |e, _| ret(e.mem.alloc(0x48)));
        e.register(0x0061_8580, |e, a| {
            e.mem.set_u32(a[0], 0x0150_0000);
            ret(a[0])
        });
        e.register(0x0046_4f30, |_, a| ret(a[0] + 1));
        for a in [
            0x0040_4eb0,
            0x0040_4ee0,
            0x0000_1128,
            0x0000_1134,
            0x0048_9100,
            0x0046_1780,
            0x0047_0200,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0061_a040, &[]);
        let made = calls_to(&e, 0x0061_8580);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][1], 0);
        assert_eq!(made[1][1], 3);
        let first = e.mem.u32(kind0 + 0xc);
        let second = e.mem.u32(kind3);
        assert_eq!(first, made[0][0]);
        assert_eq!(e.mem.u32(kind0), 0x9999);
        assert_eq!(
            calls_to(&e, 0x0000_1128),
            vec![vec![first, 0x44, 1], vec![second, 0x45, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x0000_1134),
            vec![vec![first, 0x7000], vec![second, 0x7100]]
        );
        assert_eq!(calls_to(&e, 0x0046_4f30)[0], vec![0x7000, 0]);
        assert_eq!(calls_to(&e, 0x0048_9100)[0], vec![first + 0x18, 0x7001]);
        assert_eq!(calls_to(&e, 0x0046_1780)[1], vec![0x3f2c, second]);
        assert_eq!(calls_to(&e, 0x0047_0200)[1], vec![0x54c8, 0x7100, second]);
        let timer = calls_to(&e, 0x0040_4eb0);
        assert_eq!(
            timer,
            vec![vec![timer[0][0], 0x35, 1, TESTOPIC_SOURCE_FILE, 0x1573]]
        );
        assert_eq!(calls_to(&e, 0x0040_4ee0), vec![vec![timer[0][0]]]);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00619f70_finds_the_entry_of_the_quest() {
        let mut e = Engine::new();
        let entries: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x30)).collect();
        e.mem.set_u32(entries[0], 0x77);
        e.mem.set_u32(entries[1], 0);
        e.mem.set_u32(entries[1] + 0x2c, 1);
        e.mem.set_u32(entries[2], 0);
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        for i in 0..3 {
            e.mem.set_u32(nodes[i], entries[i]);
            e.mem
                .set_u32(nodes[i] + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        let head = nodes[0];
        e.register_double(TOPIC_QUEST_LIST, move |_, _| head.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PLUS_FOUR, |_, a| (a[0] + 4).into_ret());
        assert_eq!(
            e.call(0x0061_9f70, &args![0x1000u32, 0x77u32]).u32(),
            entries[0] + 4
        );
        // A null quest skips the entry whose +0x2c is set.
        assert_eq!(
            e.call(0x0061_9f70, &args![0x1000u32, 0u32]).u32(),
            entries[2] + 4
        );
        assert_eq!(e.call(0x0061_9f70, &args![0x1000u32, 0x78u32]).u32(), 0);
    }
}
