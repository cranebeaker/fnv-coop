//! The dialogue topic index (`NV_TOPIC_INDEX`), a fix of the `fixes` preset ([`crate::fixes`],
//! docs/FIXES.md): **the same answers as FalloutNV.exe, found another way**; in the `vanilla`
//! preset nothing here runs.
//!
//! **What the original does.** `TESTopic::GetOwnerTopic` (`0061a310`) answers which topic holds a
//! topic info by walking the data handler's topic list (`listTopics`, `011c3f2c` +0x108) and, in
//! each topic, every per-quest entry and every info in it (`00619e10`, first to last), until a
//! topic whose first info with this info's form id is this very info. Loading a plugin that
//! changes an existing info clears it first (`TESTopicInfo::ClearData`, which asks for the owner
//! to take the info out of it), so a dialogue mod's load time grows with the infos it touches
//! times the topics and infos in the game (YUP and the add-ons: 5,837 searches). JIP LN
//! replaces the walk with a word it adds to every info, set where the load links the info into a
//! topic; that answers the last topic the info was linked into, which is not always the walk's
//! answer.
//!
//! **What this does**, with the walk's answer every time:
//! - **Which topics can hold the info.** The one place an info enters a topic's info arrays is
//!   `TESTopic::InsertInfo` (`00619820`; its only caller is `TESTopicInfo::Load`; the arrays'
//!   other writers are a temporary array of `0061a7d0`, entry making, resizing, sorting within an
//!   array and removals: Ghidra cross references, 2026-11-07). It runs here as its translation and
//!   then notes (info, topic). Nothing is ever taken out of the notes, so they hold every topic
//!   that holds the info; removals, deletions, overrides that move an info and blocks the heap
//!   reuses only leave extra topics, never missing ones.
//! - **Which of them hold it first by its id.** Each noted topic is asked `00619e10` (first to
//!   last) for this info's id, as the walk asks every topic; the ones that answer this very info
//!   are the walk's possible answers (a topic outside the notes can't answer it). None: 0.
//! - **Which of those the walk meets first.** The answer is the first of them in `listTopics`,
//!   ending at a null node or one with no item, as the walk ends. That order comes from a copy of
//!   the list (each topic's first place in it), made by walking the list once and kept while it
//!   stays right. Only three functions write `listTopics` (the translations name the field in
//!   these and read it elsewhere): the sorted insert `0061a380` (`00461780`, as each topic record
//!   loads), the sort `0061a5a0` (a null list meaning this one; the load's end sorts only the
//!   player's list) and the data handler's `ClearData` (`0045dfe0`, which empties it). The
//!   insert only adds a node, before a node (whose item moves into the new one after it) or at
//!   the end: every topic already in the list stays in it in the same order. So the copy stays
//!   right for every topic it holds; it is made again after the sort or `ClearData` (both
//!   registered here), after an insert of a topic it already holds (which can list that topic
//!   again, before its first place), when the list's head node no longer holds what it held
//!   (another writer), or when a possible answer isn't in it and a topic was inserted since it
//!   was made. A topic not in a fresh copy is not in the list.
//!
//! So the walk's tie-break (an info in several topics, a twin id earlier in a topic, the list's
//! order and end) is kept: a search reads one or two topics' infos and a table, not the list.
//!
//! Registering over `0061a310`, a member of the typed family `topic_search`, turns that family's
//! direct calls off for the run (a family something is registered over goes by address, so what
//! was registered runs; `engine_core::typed`): the walk's members are then called by address, and
//! the index itself calls only `00619e10`, once per noted topic of a search.
//!
//! `NV_TOPIC_INDEX_CHECK=1` with `NV_TOPIC_INDEX=1` (alone it is a test variable that makes the
//! run vanilla) runs the original's walk (the translation) beside every search and
//! stops the game at the first difference, naming the info and both answers; a line every
//! 1,000 searches says how many agreed and how often the list was copied. With or without it,
//! one line at the first sort after searches (the load's end) counts what the searches did: how
//! many topics held the info, the copies made and dropped by reason, and the time spent (YUP and
//! the add-ons, 2026-11-07: 5,837 searches, each one topic, 3 copies, 54 ms in all).

use crate::fixes::Fix;
use engine::prelude::*;
use engine::units::fallout_shared::tesdatahandler::tes_data_handler_clear_data;
use engine::units::fallout_shared::testopic::{
    fn_0061a380, fn_0061a5a0, tes_topic_get_owner_topic, tes_topic_insert_info,
};
use engine::Mem;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// `TESTopic::GetOwnerTopic` and `TESTopic::InsertInfo` (Xbox PDB).
pub const OWNER_TOPIC: u32 = 0x0061_a310;
pub const INSERT_INFO: u32 = 0x0061_9820;
/// The sorted insert into a topic list (cdecl: topic, list, by priority) and the list's sort
/// (cdecl: by priority, list; a null list is `listTopics`).
pub const SORTED_INSERT: u32 = 0x0061_a380;
pub const SORT: u32 = 0x0061_a5a0;
/// `TESDataHandler::ClearData` (Xbox PDB).
pub const HANDLER_CLEAR: u32 = 0x0045_dfe0;

/// The row in [`crate::fixes::FIXES`].
pub const FIX: Fix = Fix {
    name: "dialogue topic index",
    env: "NV_TOPIC_INDEX",
    addresses: &[HANDLER_CLEAR, INSERT_INFO, OWNER_TOPIC, SORTED_INSERT, SORT],
    summary: "the topic that holds a dialogue info is looked up, not walked for: the same answers",
    install,
};

/// The switch that runs the original's walk beside every search and stops on a difference. A
/// test variable ([`crate::fixes`]): set alone it makes the run vanilla, where nothing here runs,
/// so it goes with `NV_TOPIC_INDEX=1` (or `NV_PRESET=fixes`).
pub const CHECK_ENV: &str = "NV_TOPIC_INDEX_CHECK";

/// `00619e10` (`ECX` = topic; id, a byte): the first info with that id in the topic, 0 for none.
const FIND_INFO: u32 = 0x0061_9e10;
/// The word holding the data handler, whose topic list's first node is at +0x108.
const DATA_HANDLER: u32 = 0x011c_3f2c;
const TOPIC_LIST: u32 = 0x108;
/// A form's id.
const FORM_ID: u32 = 0xc;

/// A copy of `listTopics`' order.
struct Order {
    /// The data handler it was made from and its list's head node's two words then.
    handler: u32,
    head: [u32; 2],
    /// Each topic's first place in the list.
    place: HashMap<u32, u32>,
    /// No topic inserted since it was made: a topic it doesn't hold is not in the list.
    fresh: bool,
}

#[derive(Default)]
struct Index {
    /// Per info, every topic `InsertInfo` linked it into, oldest first.
    topics: HashMap<u32, Vec<u32>>,
    /// The list's order, while it is right.
    order: Option<Order>,
    /// Searches the check compared, and copies of the list made.
    checked: u64,
    copies: u64,
    /// What the searches did, logged once, at the first sort after searches (the load's end).
    stats: Stats,
    reported: bool,
}

/// Counters of the searches and the order's copies, by reason, with the time spent.
#[derive(Default, Debug)]
struct Stats {
    searches: u64,
    /// No topic noted for the info; no noted topic holding it; one holding it; more.
    unlinked: u64,
    none_holding: u64,
    one_holding: u64,
    several_holding: u64,
    /// Answers that were a topic (not 0).
    answered: u64,
    /// Copies made: none kept, another handler, a changed head node, a holder missing after an
    /// insert.
    copy_none: u64,
    copy_handler: u64,
    copy_head: u64,
    copy_missing: u64,
    /// Copies dropped: the sort of `listTopics`, `ClearData`, an insert of a topic the copy holds.
    drop_sort: u64,
    drop_clear: u64,
    drop_insert_held: u64,
    /// Sorted inserts into `listTopics`.
    inserts: u64,
    /// Nanoseconds in searches and in making copies.
    search_ns: u64,
    copy_ns: u64,
}

/// The run's index, by its guest memory ([`key`]), behind one lock for every thread.
fn indexes() -> std::sync::MutexGuard<'static, HashMap<usize, Index>> {
    static INDEXES: std::sync::OnceLock<Mutex<HashMap<usize, Index>>> = std::sync::OnceLock::new();
    INDEXES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The guest memory's address: what every game thread of a run shares (ADR-0015 gives each
/// thread an engine context of its own over the one guest memory), unlike the engine's address.
/// Through a `&Mem`, so if a context comes to hold the memory by reference this is still the
/// memory's own address. A run has one engine, boxed; a test's engine is boxed and removes its
/// entry when made and when dropped, so a later engine at the same address starts clean.
fn key(e: &Engine) -> usize {
    let mem: &Mem = &e.mem;
    mem as *const Mem as usize
}

/// Whether `list` is `listTopics` (the data handler's +0x108), or a null list, which the sort
/// takes as `listTopics`.
fn is_list_topics(e: &Engine, list: u32) -> bool {
    list == 0 || list == e.mem.u32(DATA_HANDLER).wrapping_add(TOPIC_LIST)
}

/// Registers the link, the list's three writers and the search (checked against the walk when
/// [`CHECK_ENV`] is 1).
pub fn install(e: &mut Engine) {
    let check = std::env::var(CHECK_ENV).is_ok_and(|v| v.trim() == "1");
    install_with(e, check);
}

/// [`install`] with the check given.
pub fn install_with(e: &mut Engine, check: bool) {
    e.register(INSERT_INFO, insert_info_entry);
    e.register(SORTED_INSERT, sorted_insert_entry);
    e.register(SORT, sort_entry);
    e.register(HANDLER_CLEAR, handler_clear_entry);
    if check {
        e.register(OWNER_TOPIC, checked_owner_topic_entry);
    } else {
        e.register(OWNER_TOPIC, owner_topic_entry);
    }
}

/// `TESTopic::InsertInfo` as translated, then the (info, topic) pair noted. The translation
/// returns false, linking nothing, for a zero quest or a null info.
fn insert_info_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let (topic, quest, info, previous) = (a[0], a[1], a[2], a[3]);
    let linked = tes_topic_insert_info(e, Ptr::new(topic), quest, Ptr::new(info), previous);
    if linked {
        let mut indexes = indexes();
        let topics = indexes
            .entry(key(e))
            .or_default()
            .topics
            .entry(info)
            .or_default();
        if !topics.contains(&topic) {
            topics.push(topic);
        }
    }
    Ret {
        eax: u32::from(linked),
        ..Ret::default()
    }
}

/// The sorted insert as translated. Into `listTopics` (a null list inserts nothing; other
/// lists, such as the player's topics, don't matter here): the copy of the order stays right for
/// what it holds, but a topic it doesn't hold may now be in the list. A topic it already holds may
/// now be listed a second time before its first place (the insert stops at the topic itself only
/// when it meets it before the insertion point): then the copy is made again.
fn sorted_insert_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let (topic, list) = (a[0], a[1]);
    let listed = list != 0 && is_list_topics(e, list);
    fn_0061a380(e, topic, list, a[2] as u8);
    if !listed {
        return Ret::default();
    }
    if let Some(index) = indexes().get_mut(&key(e)) {
        index.stats.inserts += 1;
        if let Some(order) = index.order.as_mut() {
            if order.place.contains_key(&topic) {
                index.order = None;
                index.stats.drop_insert_held += 1;
            } else {
                order.fresh = false;
            }
        }
    }
    Ret::default()
}

/// The sort as translated. Of `listTopics`: the order is made again at the next search. Any sort
/// after searches, the first time: the line that says what the load's searches did (the load's
/// last steps sort the player's list, not `listTopics`).
fn sort_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let listed = is_list_topics(e, a[1]);
    fn_0061a5a0(e, a[0] as u8, a[1]);
    let mut indexes = indexes();
    let engines = indexes.len();
    if let Some(index) = indexes.get_mut(&key(e)) {
        if listed && index.order.take().is_some() {
            index.stats.drop_sort += 1;
        }
        if !index.reported && index.stats.searches > 0 {
            index.reported = true;
            let epoch_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_millis());
            eprintln!(
                "topic index after the load (epoch_ms {epoch_ms}, {engines} engine(s), {} infos \
                 noted): {:?}",
                index.topics.len(),
                index.stats
            );
        }
    }
    Ret::default()
}

/// `TESDataHandler::ClearData` as translated: every topic and info is gone, so the notes and
/// the copy of the order go too.
fn handler_clear_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let cleared = tes_data_handler_clear_data(e, Ptr::new(a[0]));
    if let Some(index) = indexes().get_mut(&key(e)) {
        index.topics.clear();
        if index.order.take().is_some() {
            index.stats.drop_clear += 1;
        }
    }
    Ret {
        eax: u32::from(cleared),
        ..Ret::default()
    }
}

fn owner_topic_entry(e: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: indexed_owner(e, a[0]),
        ..Ret::default()
    }
}

/// [`owner_topic_entry`] with the original's walk (the translation) run beside it: a difference
/// stops the game.
fn checked_owner_topic_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let info = a[0];
    let found = indexed_owner(e, info);
    let walked = tes_topic_get_owner_topic(e, Ptr::new(info)).addr();
    if walked != found {
        let id = e.mem.u32(info.wrapping_add(FORM_ID));
        panic!(
            "topic index: the info {info:08x} (form {id:08x}) is in topic {walked:08x} by the \
             original's walk, {found:08x} by the index"
        );
    }
    let mut indexes = indexes();
    let index = indexes.entry(key(e)).or_default();
    index.checked += 1;
    if index.checked % 1000 == 0 {
        eprintln!(
            "topic index check: {} searches, index = walk in all; the list copied {} times",
            index.checked, index.copies
        );
    }
    Ret {
        eax: found,
        ..Ret::default()
    }
}

/// The walk's answer from the topics the info was linked into (module documentation), counted.
fn indexed_owner(e: &mut Engine, info: u32) -> u32 {
    let start = Instant::now();
    let (found, outcome) = search_index(e, info);
    let mut indexes = indexes();
    let stats = &mut indexes.entry(key(e)).or_default().stats;
    stats.searches += 1;
    stats.answered += u64::from(found != 0);
    match outcome {
        Outcome::Unlinked => stats.unlinked += 1,
        Outcome::NoneHolding => stats.none_holding += 1,
        Outcome::Holding(1) => stats.one_holding += 1,
        Outcome::Holding(_) => stats.several_holding += 1,
    }
    stats.search_ns += u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX);
    found
}

enum Outcome {
    Unlinked,
    NoneHolding,
    Holding(usize),
}

fn search_index(e: &mut Engine, info: u32) -> (u32, Outcome) {
    let candidates = match indexes().get(&key(e)).and_then(|i| i.topics.get(&info)) {
        Some(topics) => topics.clone(),
        None => return (0, Outcome::Unlinked),
    };
    let id = e.mem.u32(info.wrapping_add(FORM_ID));
    let holding: Vec<u32> = candidates
        .into_iter()
        .filter(|&topic| e.call(FIND_INFO, &args![topic, id, 0u32]).u32() == info)
        .collect();
    if holding.is_empty() {
        return (0, Outcome::NoneHolding);
    }
    let handler = e.mem.u32(DATA_HANDLER);
    let head_node = handler.wrapping_add(TOPIC_LIST);
    let head = [e.mem.u32(head_node), e.mem.u32(head_node.wrapping_add(4))];
    let mut indexes = indexes();
    let index = indexes.entry(key(e)).or_default();
    let stale = match index.order.as_ref() {
        None => Some(Stale::None),
        Some(order) if order.handler != handler => Some(Stale::Handler),
        Some(order) if order.head != head => Some(Stale::Head),
        Some(order) if !order.fresh && !holding.iter().all(|t| order.place.contains_key(t)) => {
            Some(Stale::Missing)
        }
        Some(_) => None,
    };
    if let Some(stale) = stale {
        let start = Instant::now();
        index.order = Some(copy_order(e, handler, head));
        index.copies += 1;
        let stats = &mut index.stats;
        match stale {
            Stale::None => stats.copy_none += 1,
            Stale::Handler => stats.copy_handler += 1,
            Stale::Head => stats.copy_head += 1,
            Stale::Missing => stats.copy_missing += 1,
        }
        stats.copy_ns += u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX);
    }
    let place = &index.order.as_ref().expect("made above").place;
    let found = holding
        .iter()
        .filter_map(|topic| place.get(topic).map(|at| (*at, *topic)))
        .min()
        .map_or(0, |(_, topic)| topic);
    (found, Outcome::Holding(holding.len()))
}

/// Why a copy of the order was made.
enum Stale {
    None,
    Handler,
    Head,
    Missing,
}

/// `listTopics`' order as the walk meets it: from the head node, ending at a null node or one
/// with no item.
fn copy_order(e: &Engine, handler: u32, head: [u32; 2]) -> Order {
    let mut place = HashMap::new();
    let mut node = handler.wrapping_add(TOPIC_LIST);
    let mut at = 0u32;
    while node != 0 {
        let topic = e.mem.u32(node);
        if topic == 0 {
            break;
        }
        place.entry(topic).or_insert(at);
        at += 1;
        node = e.mem.u32(node.wrapping_add(4));
    }
    Order {
        handler,
        head,
        place,
        fresh: true,
    }
}

#[cfg(test)]
mod tests {
    //! The index against the original's walk (the translation of `0061a310`), on topics, entries
    //! and infos laid out in guest memory as the exe's. Infos are linked by the translation of
    //! `InsertInfo` and topics listed and sorted by the translations of the sorted insert and the
    //! sort (by name). The primitives they call are doubles with the exe's layout and meaning: an
    //! info array (elements at +4, capacity at +8, count at +0xc; insert, set and grow, remove; an
    //! info's index a 16-bit word), the simple list's node operations, the allocation and the
    //! name compare. The searches, the entry lookup and making, the insert, the sort and both
    //! walks are the real code.

    use super::*;
    use std::cell::Cell;

    /// The allocations `InsertInfo` and the entry making call (the arrays' own primitives,
    /// `0061c120`, `0061bc10`, `0061bd20`, `00864240`, `00864280` and `0061bdc0`, are the
    /// translations), the link array's constructor and the helpers that keep an info's index.
    const ALLOC: u32 = 0x0040_1000;
    const MALLOC: u32 = 0x00aa_1070;
    const FREE: u32 = 0x00aa_10f0;
    const LINK_ARRAY: u32 = 0x006b_3f40;
    /// `NiTLargeArray::SetAt` (`0061bd20`): the word at an index, no shift.
    const SET_AT: u32 = 0x0061_bd20;
    const REINDEX: u32 = 0x0061_9fe0;
    const INFO_INDEX: u32 = 0x0061_9a70;
    const INFO_SET_INDEX: u32 = 0x0061_f340;
    const PRINT: u32 = 0x005b_5e40;
    /// The simple list's node operations (`ECX` = node; the address of an item) and the name
    /// compare (`strcmp`'s sign).
    const NODE_INSERT: u32 = 0x005a_e3d0;
    const NODE_APPEND: u32 = 0x0090_5820;
    const NODE_REMOVE: u32 = 0x0063_f7b0;
    const NODE_REMOVE_AFTER: u32 = 0x0090_5330;
    const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
    const LIST_DESTROY: u32 = 0x0046_ffb0;
    const COMPARE_NAMES: u32 = 0x0040_4dc0;
    /// Where the doubles keep an info's 16-bit index, the topics' vtable, its name slot's
    /// function and where a topic keeps its name.
    const INDEX_AT: u32 = 0x20;
    const TOPIC_VTABLE: u32 = 0x0150_0000;
    const TOPIC_NAME_FN: u32 = 0x0150_1000;
    const NAME_AT: u32 = 0x48;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn elements(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 0xc))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    /// Puts `values` into the array as its elements (`NiTLargeArray`: storage +4, capacity +8,
    /// used size +0xc, non-zero count +0x10), as an array the exe made would hold them.
    fn set_elements(e: &mut Engine, array: u32, values: &[u32]) {
        let n = u32::try_from(values.len()).unwrap();
        let present = values.iter().filter(|&&v| v != 0).count();
        e.mem.set_u32(array + 0x10, u32::try_from(present).unwrap());
        if n > e.mem.u32(array + 8) {
            let data = e.mem.alloc(4 * (n + 8));
            e.mem.set_u32(array + 4, data);
            e.mem.set_u32(array + 8, n + 8);
        }
        let data = e.mem.u32(array + 4);
        for (at, v) in (data..).step_by(4).zip(values) {
            e.mem.set_u32(at, *v);
        }
        e.mem.set_u32(array + 0xc, n);
    }

    fn c_string(e: &mut Engine, text: &str) -> u32 {
        let p = e.mem.alloc(u32::try_from(text.len()).unwrap() + 1);
        for (at, b) in (p..).zip(text.bytes().chain([0])) {
            e.mem.set_u8(at, b);
        }
        p
    }

    fn read_c_string(e: &Engine, mut p: u32) -> Vec<u8> {
        let mut out = Vec::new();
        while e.mem.u8(p) != 0 {
            out.push(e.mem.u8(p));
            p += 1;
        }
        out
    }

    /// A test's engine, boxed so that its guest memory's address (the index's [`key`]) holds; its
    /// index goes when it is made (what an earlier engine left at that address) and when dropped.
    struct Run(Box<Engine>);

    impl std::ops::Deref for Run {
        type Target = Engine;
        fn deref(&self) -> &Engine {
            &self.0
        }
    }

    impl std::ops::DerefMut for Run {
        fn deref_mut(&mut self) -> &mut Engine {
            &mut self.0
        }
    }

    impl Drop for Run {
        fn drop(&mut self) {
            indexes().remove(&key(&self.0));
        }
    }

    fn engine(check: bool) -> Run {
        engine_with(Some(check))
    }

    /// The engine with the doubles and an empty data handler; the fix installed when `check` is
    /// given (with or without the check).
    fn engine_with(check: Option<bool>) -> Run {
        let mut e = Run(Box::new(Engine::new()));
        indexes().remove(&key(&e));
        e.register(ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MALLOC, |e, a| ret(e.mem.alloc(a[0].max(4))));
        for nop in [FREE, LINK_ARRAY, PRINT, LIST_DESTROY] {
            e.register(nop, |_, _| Ret::default());
        }
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        e.register(NODE_INSERT, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) != 0 || e.mem.u32(a[0] + 4) != 0 {
                let moved = e.mem.alloc(8);
                e.mem.set_u32(moved, e.mem.u32(a[0]));
                e.mem.set_u32(moved + 4, e.mem.u32(a[0] + 4));
                e.mem.set_u32(a[0] + 4, moved);
            }
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(NODE_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
            } else {
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(fresh, item);
                e.mem.set_u32(node + 4, fresh);
            }
            Ret::default()
        });
        e.register(NODE_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                e.mem.set_u32(a[0], e.mem.u32(next));
                e.mem.set_u32(a[0] + 4, e.mem.u32(next + 4));
            }
            Ret::default()
        });
        e.register(NODE_REMOVE_AFTER, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut previous = a[0];
            let mut node = e.mem.u32(previous + 4);
            while node != 0 {
                if e.mem.u32(node) == item {
                    e.mem.set_u32(previous + 4, e.mem.u32(node + 4));
                    break;
                }
                previous = node;
                node = e.mem.u32(node + 4);
            }
            Ret::default()
        });
        e.register(COMPARE_NAMES, |e, a| {
            let order = read_c_string(e, a[0]).cmp(&read_c_string(e, a[1]));
            ret(order as i32 as u32)
        });
        e.map(TOPIC_VTABLE, 0x2000);
        e.mem.set_u32(TOPIC_VTABLE + 0x130, TOPIC_NAME_FN);
        e.register(TOPIC_NAME_FN, |e, a| ret(e.mem.u32(a[0] + NAME_AT)));
        e.register(REINDEX, |e, a| {
            for (i, info) in elements(e, a[0]).into_iter().enumerate() {
                if info != 0 {
                    e.mem
                        .set_u16(info + INDEX_AT, u16::try_from(i).unwrap_or(u16::MAX));
                }
            }
            Ret::default()
        });
        e.register(INFO_INDEX, |e, a| {
            ret(u32::from(e.mem.u16(a[0] + INDEX_AT)))
        });
        e.register(INFO_SET_INDEX, |e, a| {
            e.mem.set_u16(a[0] + INDEX_AT, a[1] as u16);
            Ret::default()
        });
        let handler = e.mem.alloc(0x110);
        e.map(DATA_HANDLER, 4);
        e.mem.set_u32(DATA_HANDLER, handler);
        if let Some(check) = check {
            install_with(&mut e, check);
        }
        e
    }

    fn head(e: &Engine) -> u32 {
        e.mem.u32(DATA_HANDLER) + TOPIC_LIST
    }

    /// The topics of the handler's list, in order, up to its end.
    fn topics_in_list(e: &Engine) -> Vec<u32> {
        let mut out = Vec::new();
        let mut node = head(e);
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// A topic named `name`, not listed.
    fn new_topic(e: &mut Engine, name: &str) -> u32 {
        let topic = e.mem.alloc(0x50);
        e.mem.set_u32(topic, TOPIC_VTABLE);
        let name = c_string(e, name);
        e.mem.set_u32(topic + NAME_AT, name);
        topic
    }

    /// A topic listed as a topic record's load lists it (`00461780`: the sorted insert into
    /// `listTopics`, by priority); here by name, the priorities being equal.
    fn listed_topic(e: &mut Engine, name: &str) -> u32 {
        let topic = new_topic(e, name);
        insert(e, topic);
        topic
    }

    fn insert(e: &mut Engine, topic: u32) {
        let list = head(e);
        e.call(SORTED_INSERT, &args![topic, list, 0u32]);
    }

    /// Renames a topic and sorts `listTopics` (a null list).
    fn rename_and_sort(e: &mut Engine, topic: u32, name: &str) {
        let name = c_string(e, name);
        e.mem.set_u32(topic + NAME_AT, name);
        e.call(SORT, &args![0u32, 0u32]);
    }

    fn new_info(e: &mut Engine, id: u32) -> u32 {
        let info = e.mem.alloc(0x54);
        e.mem.set_u32(info + FORM_ID, id);
        e.mem.set_u16(info + INDEX_AT, 0xffff);
        info
    }

    fn link(e: &mut Engine, topic: u32, quest: u32, info: u32, previous: u32) {
        e.call(INSERT_INFO, &args![topic, quest, info, previous]);
    }

    /// Every per-quest entry's info array of a topic.
    fn arrays(e: &Engine, topic: u32) -> Vec<u32> {
        let mut out = Vec::new();
        let mut node = topic + 0x2c;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node) + 4);
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// The info array of the topic's entry for `quest`, when it has one.
    fn entry_array(e: &Engine, topic: u32, quest: u32) -> Option<u32> {
        let mut node = topic + 0x2c;
        while node != 0 && e.mem.u32(node) != 0 {
            let entry = e.mem.u32(node);
            if e.mem.u32(entry) == quest {
                return Some(entry + 4);
            }
            node = e.mem.u32(node + 4);
        }
        None
    }

    /// Takes the info out of every array of the topic as the exe takes an info out: its slot set
    /// to zero (`0061bd20`, no shift); the array's next insert compacts it.
    fn unlink(e: &mut Engine, topic: u32, info: u32) {
        let zero = e.mem.alloc(4);
        e.mem.set_u32(zero, 0);
        for array in arrays(e, topic) {
            for (at, x) in elements(e, array).into_iter().enumerate() {
                if x == info {
                    e.call(SET_AT, &args![array, u32::try_from(at).unwrap(), zero]);
                }
            }
        }
    }

    fn walk(e: &mut Engine, info: u32) -> u32 {
        tes_topic_get_owner_topic(e, Ptr::new(info)).addr()
    }

    fn search(e: &mut Engine, info: u32) -> u32 {
        e.call(OWNER_TOPIC, &args![info]).u32()
    }

    fn copies(e: &Engine) -> u64 {
        indexes().get(&key(e)).map_or(0, |index| index.copies)
    }

    fn assert_all(e: &mut Engine, infos: &[u32], step: &str) -> usize {
        let mut held = 0;
        for &info in infos {
            let walked = walk(e, info);
            assert_eq!(search(e, info), walked, "{step}: info {info:08x}");
            held += usize::from(walked != 0);
        }
        held
    }

    /// The cases by name: the walk's tie-breaks, a mod's changes, deletions, the list's order and
    /// topics out of it.
    #[test]
    fn the_index_answers_as_the_walk() {
        let mut e = engine(false);
        let base = listed_topic(&mut e, "GREETING");
        let dlc = listed_topic(&mut e, "NVDLC01Topic");
        let other = listed_topic(&mut e, "Rumors");
        // The base game's infos, two quests; a DLC's infos (load-order index 01) in its own
        // topic and in the base topic.
        let a = new_info(&mut e, 0x0000_1000);
        let b = new_info(&mut e, 0x0000_1001);
        let c = new_info(&mut e, 0x0000_1002);
        link(&mut e, base, 0x0000_0100, a, 0);
        link(&mut e, base, 0x0000_0100, b, 0x0000_1000);
        link(&mut e, base, 0x0000_0200, c, 0);
        let d = new_info(&mut e, 0x0100_2000);
        let f = new_info(&mut e, 0x0100_2001);
        link(&mut e, dlc, 0x0100_0300, d, 0);
        link(&mut e, base, 0x0100_0300, f, u32::MAX);
        let all = [a, b, c, d, f];
        assert_eq!(assert_all(&mut e, &all, "linked"), 5);
        assert_eq!(search(&mut e, d), dlc);

        // The same info in two topics: the first in the list's order, whichever was linked first.
        link(&mut e, other, 0x0000_0100, a, 0);
        let first = |e: &Engine, x: u32, y: u32| {
            let list = topics_in_list(e);
            let at = |t| list.iter().position(|&l| l == t).unwrap();
            if at(x) < at(y) {
                x
            } else {
                y
            }
        };
        assert_eq!(search(&mut e, a), first(&e, base, other));
        // Renamed and sorted: the order changes, and so does the answer.
        // By the compare (`0061a450`: a lower first letter, or else a greater name) the three
        // inserts list Rumors, NVDLC01Topic, GREETING; renamed N and NZ, the sort lists GREETING's
        // topic first and Rumors' last.
        assert_eq!(topics_in_list(&e), [other, dlc, base]);
        assert_eq!(search(&mut e, a), other);
        let name = c_string(&mut e, "NZ");
        e.mem.set_u32(other + NAME_AT, name);
        rename_and_sort(&mut e, base, "N");
        assert_eq!(topics_in_list(&e), [base, dlc, other]);
        assert_eq!(search(&mut e, a), base);
        assert_all(&mut e, &all, "sorted");

        // A twin: another info with the same id earlier in a topic hides this one there.
        let twin = new_info(&mut e, 0x0100_2000);
        link(&mut e, dlc, 0x0100_0300, twin, 0);
        assert_eq!(walk(&mut e, d), 0);
        assert_all(&mut e, &[d, twin], "twin");

        // A mod's override moves an info to a mod's new topic: taken out, linked again; then
        // the topic listed.
        unlink(&mut e, base, c);
        let new = new_topic(&mut e, "ModTopic");
        link(&mut e, new, 0x0200_0400, c, 0);
        assert_eq!(search(&mut e, c), 0, "not listed yet");
        insert(&mut e, new);
        assert_eq!(search(&mut e, c), new);
        // A deleted info: out of every array.
        unlink(&mut e, base, b);
        assert_eq!(search(&mut e, b), 0);
        // An id changed under the index.
        e.mem.set_u32(f + FORM_ID, 0x0000_1000);
        assert_all(&mut e, &[a, b, c, d, f, twin], "end");
        // Never linked: none, without a walk.
        let lone = new_info(&mut e, 0x0300_0001);
        assert_eq!(search(&mut e, lone), 0);
    }

    /// A seeded series of loads, overrides, deletions, new topics (listed or not), sorts and id
    /// clashes; after each step the index answers as the walk for every info. The list is copied
    /// far fewer times than it is searched.
    #[test]
    fn a_long_series_of_changes_keeps_index_and_walk_the_same() {
        let mut e = engine(false);
        let seed = Cell::new(0x2545_f491u32);
        let next = |n: usize| -> usize {
            let mut x = seed.get();
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            seed.set(x);
            x as usize % n
        };
        let name = |n: usize| format!("T{:03}", n % 1000);
        let mut topics: Vec<u32> = (0..6)
            .map(|_| listed_topic(&mut e, &name(next(1000))))
            .collect();
        let quests = [0x0000_0100u32, 0x0000_0200, 0x0100_0300, 0x0200_0400];
        let mut infos: Vec<u32> = Vec::new();
        let (mut held_ever, mut searches) = (0, 0u64);
        for step in 0..600 {
            match next(11) {
                // A plugin's new info, or an override linking an existing one again (after the
                // previous info by id, at the start, or at the end).
                0..=3 => {
                    let info = if infos.is_empty() || next(3) == 0 {
                        let plugin = u32::try_from(next(4)).unwrap() << 24;
                        let id = plugin | u32::try_from(next(40)).unwrap();
                        let info = new_info(&mut e, id);
                        infos.push(info);
                        info
                    } else {
                        infos[next(infos.len())]
                    };
                    let topic = topics[next(topics.len())];
                    let quest = quests[next(quests.len())];
                    let previous = match next(3) {
                        0 => 0,
                        1 => u32::MAX,
                        _ => {
                            // Only where the exe goes on: an info with no index yet (one with an
                            // index is first taken out at it, of whatever array this is), and an
                            // id whose last match in the topic is in this quest's array (else its
                            // position is -1, `SetAtGrow(-1)`, a write below the array; not found
                            // at all, the message, which reads the last loaded topic).
                            let other = infos[next(infos.len())];
                            let id = e.mem.u32(other + FORM_ID);
                            let found = e.call(FIND_INFO, &args![topic, id, 1u32]).u32();
                            let in_entry = entry_array(&e, topic, quest)
                                .is_some_and(|array| elements(&e, array).contains(&found));
                            if found != 0 && in_entry && e.mem.u16(info + INDEX_AT) == 0xffff {
                                id
                            } else {
                                0
                            }
                        }
                    };
                    link(&mut e, topic, quest, info, previous);
                }
                // ClearData of an override, or a deletion: out of a topic.
                4 | 5 if !infos.is_empty() => {
                    let info = infos[next(infos.len())];
                    let topic = topics[next(topics.len())];
                    unlink(&mut e, topic, info);
                }
                // A mod's topic, listed (anywhere its name puts it) or not yet.
                6 if topics.len() < 16 => {
                    let topic = if next(4) == 0 {
                        new_topic(&mut e, &name(next(1000)))
                    } else {
                        listed_topic(&mut e, &name(next(1000)))
                    };
                    topics.push(topic);
                }
                // A topic listed (again: the insert may list it twice).
                7 => {
                    let topic = topics[next(topics.len())];
                    insert(&mut e, topic);
                }
                // A rename and the sort.
                8 => {
                    let topic = topics[next(topics.len())];
                    rename_and_sort(&mut e, topic, &name(next(1000)));
                }
                // A form id that clashes with another's.
                9 if infos.len() > 1 => {
                    let info = infos[next(infos.len())];
                    let other = infos[next(infos.len())];
                    let id = e.mem.u32(other + FORM_ID);
                    e.mem.set_u32(info + FORM_ID, id);
                }
                _ => {}
            }
            if step % 6 == 0 {
                held_ever = held_ever.max(assert_all(&mut e, &infos, &format!("step {step}")));
                searches += infos.len() as u64;
            }
        }
        assert_all(&mut e, &infos, "last");
        assert!(
            infos.len() > 40 && held_ever > 10,
            "{} {held_ever}",
            infos.len()
        );
        assert!(
            copies(&e) * 4 < searches,
            "{} copies, {searches} searches",
            copies(&e)
        );
    }

    /// A writer of the list the index doesn't know about: the head guard sees a changed head.
    #[test]
    fn a_changed_head_makes_the_copy_again() {
        let mut e = engine(false);
        let a = listed_topic(&mut e, "A");
        let b = listed_topic(&mut e, "B");
        let info = new_info(&mut e, 0x10);
        link(&mut e, a, 0x100, info, 0);
        link(&mut e, b, 0x100, info, 0);
        let first = topics_in_list(&e)[0];
        assert_eq!(search(&mut e, info), first);
        // Swap the two nodes' items behind the index's back.
        let node = head(&e);
        let second_node = e.mem.u32(node + 4);
        let second = e.mem.u32(second_node);
        e.mem.set_u32(node, second);
        e.mem.set_u32(second_node, first);
        assert_eq!(search(&mut e, info), second);
        assert_eq!(walk(&mut e, info), second);
    }

    /// A listed topic inserted again is listed a second time when the insert point comes before
    /// it (the insert stops at the topic itself only on the way there): its first place moves
    /// before a topic the copy put ahead of it, with the head node unchanged. By the compare
    /// (`0061a450`: a lower first letter, or else a greater name), "A2" then "Q" then "A1" list
    /// as A1, Q, A2, and A2 inserted again goes before Q.
    #[test]
    fn a_topic_listed_again_earlier_makes_the_copy_again() {
        let mut e = engine(false);
        let b = listed_topic(&mut e, "A2");
        let c = listed_topic(&mut e, "Q");
        let a = listed_topic(&mut e, "A1");
        assert_eq!(topics_in_list(&e), [a, c, b]);
        let info = new_info(&mut e, 0x10);
        link(&mut e, c, 0x100, info, 0);
        link(&mut e, b, 0x100, info, 0);
        assert_eq!(search(&mut e, info), c);
        insert(&mut e, b);
        assert_eq!(topics_in_list(&e), [a, b, c, b]);
        assert_eq!(walk(&mut e, info), b);
        assert_eq!(search(&mut e, info), b);
    }

    /// The check stops at an info put into a topic's array other than through `InsertInfo`, which
    /// the index can't see.
    #[test]
    fn the_check_stops_at_a_difference() {
        for check in [false, true] {
            let mut e = engine(check);
            let topic = listed_topic(&mut e, "T");
            let seen = new_info(&mut e, 0x10);
            link(&mut e, topic, 0x100, seen, 0);
            let hidden = new_info(&mut e, 0x11);
            let array = arrays(&e, topic)[0];
            let mut v = elements(&e, array);
            v.push(hidden);
            set_elements(&mut e, array, &v);
            assert_eq!(search(&mut e, seen), topic);
            let caught =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| search(&mut e, hidden)));
            if check {
                assert!(caught.is_err());
            } else {
                assert_eq!(caught.unwrap(), 0, "the index alone misses it");
                assert_eq!(walk(&mut e, hidden), topic);
            }
        }
    }

    /// The preset decides, through the fixes' own install: vanilla registers nothing, so the
    /// search is the walk, which finds an info put into an array other than through
    /// `InsertInfo`; with `NV_TOPIC_INDEX=1` the index answers, and it doesn't see that info.
    #[test]
    fn the_preset_switch_decides_who_answers() {
        use crate::fixes::Choice;
        let only_this =
            Choice::from_parts(None, |name| (name == FIX.env).then(|| "1".to_string())).unwrap();
        for (choice, indexed) in [(Choice::vanilla(), false), (only_this, true)] {
            assert_eq!(choice.is_on(&FIX), indexed);
            let mut e = engine_with(None);
            crate::fixes::install(&mut e, &choice);
            let topic = listed_topic(&mut e, "T");
            let seen = new_info(&mut e, 0x10);
            link(&mut e, topic, 0x100, seen, 0);
            let hidden = new_info(&mut e, 0x11);
            let array = arrays(&e, topic)[0];
            let mut v = elements(&e, array);
            v.push(hidden);
            set_elements(&mut e, array, &v);
            assert_eq!(search(&mut e, seen), topic);
            assert_eq!(search(&mut e, hidden), if indexed { 0 } else { topic });
            assert_eq!(walk(&mut e, hidden), topic);
        }
    }
}
