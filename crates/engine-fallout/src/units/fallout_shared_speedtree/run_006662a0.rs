//! `fallout shared/speedtree/run_006662a0` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions sit inside `bstreemanager.cpp` after the manager's own
//! code. They belong to the lock-free hash map the manager keeps its
//! background-loaded trees in (`pBackgroundLoadedTreeMap`): a table of
//! bucket heads whose chains are singly linked nodes `{key, value, next}`,
//! the low bit of a `next` word marking a node deleted. The names of the
//! classes are not in the Xbox PDB map, so the functions carry their
//! addresses and their offsets are described, not named.
//!
//! The cursor object `006662a0` works on (`this`) holds, by offset:
//! `+0` the map, `+4`/`+8`/`+0xc` three published-word slots (pointers to the
//! words that mark what the cursor is using), `+0x10` the address of the
//! link word being examined, `+0x14` its value, `+0x18` the value of the next
//! link, `+0x1c` a list of retired nodes and `+0x20` its length.
//!
//! The helper calls are the exe's own: `0044dec0` (a word with its low bit
//! cleared), `0044df00` (the same), `0044dff0` (its low bit), `006ecba0` (a
//! tagged word: stores the value and sets or clears the low bit),
//! `004491c0` (a compare-and-swap of the destination, new value, expected
//! value), `00559450` (the word a pointer holds), `0044ee20` (the address of
//! a map's bucket for a key) and `0040fbe0` (an empty function the exe calls
//! after each published word).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The word a pointer holds (`00559450`, thiscall).
const READ_WORD: u32 = 0x0055_9450;
/// A word with its low bit cleared / tested (`0044dec0`, `0044df00` and
/// `0044dff0`, thiscall on the address of the word).
const UNMARKED: u32 = 0x0044_dec0;
const UNMARKED_POINTER: u32 = 0x0044_df00;
const IS_MARKED: u32 = 0x0044_dff0;
/// Tagged word constructor (`006ecba0`: the address, the mark, the value).
const TAGGED_WORD: u32 = 0x006e_cba0;
/// Compare-and-swap (`004491c0`: destination, new value, expected value).
const COMPARE_AND_SWAP: u32 = 0x0044_91c0;
/// Address of the bucket of `key` in the map (`0044ee20`).
const BUCKET_OF: u32 = 0x0044_ee20;
/// The empty function called after each published word (`0040fbe0`).
const AFTER_PUBLISH: u32 = 0x0040_fbe0;
/// Count of retired nodes after which the list is flushed (`0044edb0`
/// reads it from the map: the word at `+0x10`) and the flush (`0044f220`).
const RETIRE_LIMIT: u32 = 0x0044_edb0;
const FLUSH_RETIRED: u32 = 0x0044_f220;
/// Sets the link word of a retired node (`006ecd40`: the node, the next
/// retired node, stored at `+4`).
const SET_RETIRED_NEXT: u32 = 0x006e_cd40;
/// Frees the nodes of the table (`0044ed80`, flags) and the table's storage
/// (`0044e360`).
const DELETE_NODE: u32 = 0x0044_ed80;
const FREE_STORAGE: u32 = 0x0044_e360;
/// Virtual slots of the map used while searching: "this key matches" (`+0x34`)
/// and the action on the match (`+0x38`).
const SLOT_MATCHES: u32 = 0x34;
const SLOT_ON_MATCH: u32 = 0x38;

/// The word `address` holds, read through `00559450`.
fn read_word(e: &mut Engine, address: u32) -> u32 {
    e.call(READ_WORD, &args![address]).u32()
}

/// `0044dec0(address)`: the word at `address` with its low bit cleared.
fn unmarked(e: &mut Engine, address: u32) -> u32 {
    e.call(UNMARKED, &args![address]).u32()
}

/// Publishes `value` in the slot the cursor field `slot_field` points to
/// and calls the empty function after it.
fn publish(e: &mut Engine, this: u32, slot_field: u32, value: u32) {
    let slot = e.mem.u32(this + slot_field);
    e.mem.set_u32(slot, value);
    e.call(AFTER_PUBLISH, &args![]);
}

// Translated from 006662a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Searches the bucket of `key` of the lock-free map `this` is the cursor
/// of (`RET 8`: `key`, `arg`), restarting from the bucket head whenever
/// another thread changed a link it had read. Nodes whose next link is
/// marked are unlinked (compare-and-swap of the previous link) and handed
/// to `00666510`. At the first live node the map's virtual `+0x34` is asked
/// whether the node's key matches `key`/`arg`; on a match the result of
/// virtual `+0x38` for it is returned. An empty chain gives `0`.
///
/// The exe computes the bucket once, outside its retry loop. The three
/// tagged-word temporaries are locals of the exe.
pub fn fn_006662a0(e: &mut Engine, this: Ptr, key: u32, arg: u32) -> u32 {
    let cursor = this.addr();
    let map = e.mem.u32(cursor);
    let bucket = e.call(BUCKET_OF, &args![map, key]).u32();
    e.with_stack(0x0c, |e, scratch| {
        let first_word = scratch.addr();
        let second_word = first_word + 4;
        let third_word = first_word + 8;
        'restart: loop {
            e.mem.set_u32(cursor + 0x10, bucket);
            let head = e.mem.u32(bucket);
            e.mem.set_u32(cursor + 0x14, head);
            let current = unmarked(e, cursor + 0x14);
            publish(e, cursor, 0x08, current);
            let current = unmarked(e, cursor + 0x14);
            e.call(TAGGED_WORD, &args![first_word, 0u32, current]);
            let link = e.mem.u32(cursor + 0x10);
            let held = read_word(e, link);
            if held != read_word(e, first_word) {
                continue 'restart;
            }
            loop {
                if unmarked(e, cursor + 0x14) == 0 {
                    return 0;
                }
                let node = e.call(UNMARKED_POINTER, &args![cursor + 0x14]).u32();
                let next = e.mem.u32(node + 8);
                e.mem.set_u32(cursor + 0x18, next);
                let next_unmarked = unmarked(e, cursor + 0x18);
                publish(e, cursor, 0x04, next_unmarked);
                let seen = read_word(e, cursor + 0x18);
                let node = e.call(UNMARKED_POINTER, &args![cursor + 0x14]).u32();
                if seen != read_word(e, node + 8) {
                    continue 'restart;
                }
                let node_address = unmarked(e, cursor + 0x14);
                let node_key = e.mem.u32(node_address);
                let current = unmarked(e, cursor + 0x14);
                e.call(TAGGED_WORD, &args![second_word, 0u32, current]);
                let link = e.mem.u32(cursor + 0x10);
                let held = read_word(e, link);
                if held != read_word(e, second_word) {
                    continue 'restart;
                }
                if !e.call(IS_MARKED, &args![cursor + 0x18]).bool() {
                    if e.vcall(map, SLOT_MATCHES, &args![node_key, arg]).bool() {
                        return e.vcall(map, SLOT_ON_MATCH, &args![node_key, arg]).u32();
                    }
                    let node = e.call(UNMARKED_POINTER, &args![cursor + 0x14]).u32();
                    e.mem.set_u32(cursor + 0x10, node + 8);
                    let current = unmarked(e, cursor + 0x14);
                    publish(e, cursor, 0x0c, current);
                } else {
                    let next_unmarked = unmarked(e, cursor + 0x18);
                    e.call(TAGGED_WORD, &args![third_word, 0u32, next_unmarked]);
                    let expected = read_word(e, second_word);
                    let replacement = read_word(e, third_word);
                    let link = e.mem.u32(cursor + 0x10);
                    if !e
                        .call(COMPARE_AND_SWAP, &args![link, replacement, expected])
                        .bool()
                    {
                        continue 'restart;
                    }
                    let retired = unmarked(e, cursor + 0x14);
                    fn_00666510(e, this, retired);
                }
                let next = e.mem.u32(cursor + 0x18);
                e.mem.set_u32(cursor + 0x14, next);
                let next_unmarked = unmarked(e, cursor + 0x18);
                publish(e, cursor, 0x08, next_unmarked);
            }
        }
    })
}

// Translated from 00666510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Retires `node` onto the cursor's list (`RET 4`): clears the node's link
/// at `+4`, chains the previous list head (`+0x1c`) behind it (`006ecd40`),
/// makes `node` the head and counts it (`+0x20`); when the count reaches the
/// limit the map gives (`0044edb0` of the map) the list is flushed
/// (`0044f220`).
pub fn fn_00666510(e: &mut Engine, this: Ptr, node: u32) {
    let cursor = this.addr();
    e.mem.set_u32(node + 4, 0);
    let head = e.mem.u32(cursor + 0x1c);
    e.call(SET_RETIRED_NEXT, &args![node, head]);
    e.mem.set_u32(cursor + 0x1c, node);
    let count = e.mem.u32(cursor + 0x20).wrapping_add(1);
    e.mem.set_u32(cursor + 0x20, count);
    let map = e.mem.u32(cursor);
    let limit = e.call(RETIRE_LIMIT, &args![map]).u32();
    if e.mem.u32(cursor + 0x20) == limit {
        e.call(FLUSH_RETIRED, &args![this]);
    }
}

// Translated from 00666570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the table class `006665c0` destroys: runs
/// it and, with bit 0 of `flags`, frees the object. Returns `this`.
pub fn fn_00666570(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006665c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006665a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the first word of `this` to zero (`__fastcall`, no stack
/// arguments).
pub fn fn_006665a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 006665c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a table of nodes: for each of the `count` entries (the word
/// `00559450` reads at `+0`) of the entry array (`+8`, 8 bytes each) whose
/// word at `+4` is set, calls `0044ed80(entry, 1)` on it; then frees the
/// entry array (`00401030`) and the storage at `+4` (`0044e360`).
pub fn fn_006665c0(e: &mut Engine, this: Ptr) {
    let table = this.addr();
    let mut index = 0u32;
    while index < read_word(e, table) {
        let entries = e.mem.u32(table + 8);
        let entry = e.mem.u32(entries + index * 8 + 4);
        if entry != 0 {
            e.call(DELETE_NODE, &args![entry, 1u32]);
        }
        index += 1;
    }
    let entries = e.mem.u32(table + 8);
    e.call(OPERATOR_DELETE, &args![entries]);
    let storage = e.mem.u32(table + 4);
    e.call(FREE_STORAGE, &args![storage]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006662a0, fn_006662a0(Ptr, u32, u32) -> u32),
        entry!(0x00666510, fn_00666510(Ptr, u32)),
        entry!(0x00666570, fn_00666570(Ptr, u32) -> Ptr),
        entry!(0x006665a0, fn_006665a0(Ptr)),
        entry!(0x006665c0, fn_006665c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Doubles that record every call (address and words), answering zero.
    fn recording(e: &mut Engine, addrs: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for a in addrs {
            let a = *a;
            let log = log.clone();
            e.register_double(a, move |_, w| {
                log.borrow_mut().push((a, w.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The matching test of the map: true for node keys equal to 7.
    const MATCHES_DOUBLE: u32 = 0x7000_1000;
    /// The action on a match: returns `key + 0x100`.
    const ON_MATCH_DOUBLE: u32 = 0x7000_2000;
    /// The retired-list limit.
    const LIMIT: u32 = 99;

    struct Fixture {
        e: Engine,
        cursor: Ptr,
        bucket: u32,
        slots: u32,
        log: Log,
    }

    /// A map with 4 buckets, a cursor, and doubles with the helpers' real
    /// behaviour.
    fn fixture() -> Fixture {
        let mut e = Engine::new();
        let log = recording(&mut e, &[AFTER_PUBLISH, FLUSH_RETIRED]);
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(UNMARKED, |e, a| ret(e.mem.u32(a[0]) & !1));
        e.register(UNMARKED_POINTER, |e, a| ret(e.mem.u32(a[0]) & !1));
        e.register(IS_MARKED, |e, a| ret(e.mem.u32(a[0]) & 1));
        e.register(TAGGED_WORD, |e, a| {
            e.mem.set_u32(a[0], a[2] | (a[1] & 1));
            ret(a[0])
        });
        e.register(BUCKET_OF, |e, a| ret(e.mem.u32(a[0] + 0xc) + a[1] * 4));
        e.register(COMPARE_AND_SWAP, |e, a| {
            if e.mem.u32(a[0]) == a[2] {
                e.mem.set_u32(a[0], a[1]);
                ret(1)
            } else {
                ret(0)
            }
        });
        e.register(SET_RETIRED_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(RETIRE_LIMIT, |_, _| ret(LIMIT));
        e.register(MATCHES_DOUBLE, |_, a| ret((a[1] == 7) as u32));
        e.register(ON_MATCH_DOUBLE, |_, a| ret(a[1] + 0x100));
        let mut vtable_slots = vec![0u32; 15];
        vtable_slots[0x34 / 4] = MATCHES_DOUBLE;
        vtable_slots[0x38 / 4] = ON_MATCH_DOUBLE;
        let vtable = 0x7100_0000;
        e.put_vtable(vtable, &vtable_slots);
        let map = e.mem.alloc(0x20);
        e.mem.set_u32(map, vtable);
        let buckets = e.mem.alloc(16);
        e.mem.set_u32(map + 0xc, buckets);
        let slots = e.mem.alloc(16);
        let cursor: Ptr = Ptr::new(e.mem.alloc(0x28));
        e.mem.set_u32(cursor.addr(), map);
        for field in 1..4u32 {
            e.mem.set_u32(cursor.addr() + 4 * field, slots + 4 * field);
        }
        // Key 2 is in bucket 2: `BUCKET_OF` is `buckets + key * 4`.
        let bucket = buckets + 8;
        Fixture {
            e,
            cursor,
            bucket,
            slots,
            log,
        }
    }

    fn node(e: &mut Engine, key: u32, next: u32) -> u32 {
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, key);
        e.mem.set_u32(node + 8, next);
        node
    }

    #[test]
    fn an_empty_chain_gives_zero() {
        let mut f = fixture();
        let result = f.e.call(0x0066_62a0, &args![f.cursor, 2u32, 0x55u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x10), f.bucket);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x14), 0);
    }

    #[test]
    fn a_matching_first_node_returns_the_action_result() {
        let mut f = fixture();
        let found = node(&mut f.e, 7, 0);
        f.e.mem.set_u32(f.bucket, found);
        let result = f.e.call(0x0066_62a0, &args![f.cursor, 2u32, 0x55u32]).u32();
        assert_eq!(result, 0x107);
        // The cursor published the node it was looking at and its next link.
        assert_eq!(f.e.mem.u32(f.slots + 8), found);
        assert_eq!(f.e.mem.u32(f.slots + 4), 0);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x14), found);
        assert_eq!(calls_to(&f.log, AFTER_PUBLISH).len(), 2);
    }

    #[test]
    fn a_non_matching_node_is_skipped() {
        let mut f = fixture();
        let second = node(&mut f.e, 7, 0);
        let first = node(&mut f.e, 5, second);
        f.e.mem.set_u32(f.bucket, first);
        let result = f.e.call(0x0066_62a0, &args![f.cursor, 2u32, 0x55u32]).u32();
        assert_eq!(result, 0x107);
        // The link examined moved to the first node's next word, and the
        // cursor holds the second node.
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x10), first + 8);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x14), second);
        assert_eq!(f.e.mem.u32(f.slots + 12), first);
    }

    #[test]
    fn a_marked_node_is_unlinked_and_retired() {
        let mut f = fixture();
        let live = node(&mut f.e, 7, 0);
        let dead = node(&mut f.e, 5, live | 1);
        f.e.mem.set_u32(f.bucket, dead);
        let result = f.e.call(0x0066_62a0, &args![f.cursor, 2u32, 0x55u32]).u32();
        assert_eq!(result, 0x107);
        assert_eq!(f.e.mem.u32(f.bucket), live);
        // The dead node is the head of the retired list.
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x1c), dead);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x20), 1);
        assert!(calls_to(&f.log, FLUSH_RETIRED).is_empty());
    }

    #[test]
    fn a_lost_compare_and_swap_restarts_the_search() {
        let mut f = fixture();
        let live = node(&mut f.e, 7, 0);
        let dead = node(&mut f.e, 5, live | 1);
        f.e.mem.set_u32(f.bucket, dead);
        let attempts = Rc::new(RefCell::new(0u32));
        let counter = attempts.clone();
        f.e.register_double(COMPARE_AND_SWAP, move |e, a| {
            *counter.borrow_mut() += 1;
            if *counter.borrow() == 1 {
                return ret(0);
            }
            e.mem.set_u32(a[0], a[1]);
            ret(1)
        });
        let result = f.e.call(0x0066_62a0, &args![f.cursor, 2u32, 0x55u32]).u32();
        assert_eq!(result, 0x107);
        assert_eq!(*attempts.borrow(), 2);
        assert_eq!(f.e.mem.u32(f.bucket), live);
    }

    #[test]
    fn a_changed_link_restarts_the_search() {
        let mut f = fixture();
        let found = node(&mut f.e, 7, 0);
        f.e.mem.set_u32(f.bucket, found);
        // The first publication changes the bucket head under the cursor.
        let moved = Rc::new(RefCell::new(false));
        let flag = moved.clone();
        let other = node(&mut f.e, 7, 0);
        let bucket = f.bucket;
        f.e.register_double(AFTER_PUBLISH, move |e, _| {
            if !*flag.borrow() {
                *flag.borrow_mut() = true;
                e.mem.set_u32(bucket, other);
            }
            Ret::default()
        });
        let result = f.e.call(0x0066_62a0, &args![f.cursor, 2u32, 0x55u32]).u32();
        assert_eq!(result, 0x107);
        assert!(*moved.borrow());
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x14), other);
    }

    #[test]
    fn retiring_chains_the_list_and_flushes_at_the_limit() {
        let mut f = fixture();
        let first = f.e.mem.alloc(12);
        let second = f.e.mem.alloc(12);
        f.e.call(0x0066_6510, &args![f.cursor, first]);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x1c), first);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x20), 1);
        f.e.mem.set_u32(f.cursor.addr() + 0x20, LIMIT - 1);
        f.e.call(0x0066_6510, &args![f.cursor, second]);
        assert_eq!(f.e.mem.u32(second + 4), first);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x1c), second);
        assert_eq!(f.e.mem.u32(f.cursor.addr() + 0x20), LIMIT);
        assert_eq!(calls_to(&f.log, FLUSH_RETIRED), vec![vec![f.cursor.addr()]]);
    }

    #[test]
    fn the_first_word_is_cleared() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr(), 0x1234);
        e.mem.set_u32(this.addr() + 4, 0x5678);
        e.call(0x0066_65a0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x5678);
    }

    fn table(e: &mut Engine) -> (Ptr, u32, u32) {
        let table: Ptr = Ptr::new(e.mem.alloc(0x10));
        let entries = e.mem.alloc(0x18);
        let storage = e.mem.alloc(8);
        e.mem.set_u32(table.addr(), 2);
        e.mem.set_u32(table.addr() + 4, storage);
        e.mem.set_u32(table.addr() + 8, entries);
        e.mem.set_u32(entries + 4, 0x6000);
        e.mem.set_u32(entries + 12, 0);
        (table, entries, storage)
    }

    #[test]
    fn destroying_a_table_deletes_the_set_entries_then_the_storage() {
        let mut e = Engine::new();
        let log = recording(&mut e, &[DELETE_NODE, OPERATOR_DELETE, FREE_STORAGE]);
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        let (table, entries, storage) = table(&mut e);
        e.call(0x0066_65c0, &args![table]);
        assert_eq!(
            *log.borrow(),
            vec![
                (DELETE_NODE, vec![0x6000, 1]),
                (OPERATOR_DELETE, vec![entries]),
                (FREE_STORAGE, vec![storage]),
            ]
        );
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let log = recording(&mut e, &[DELETE_NODE, OPERATOR_DELETE, FREE_STORAGE]);
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        let (table, entries, _) = table(&mut e);
        let back = e.call(0x0066_6570, &args![table, 0u32]);
        assert_eq!(back.ptr::<()>(), table);
        assert!(!calls_to(&log, OPERATOR_DELETE).contains(&vec![table.addr()]));
        let back = e.call(0x0066_6570, &args![table, 1u32]);
        assert_eq!(back.ptr::<()>(), table);
        let deletes = calls_to(&log, OPERATOR_DELETE);
        assert_eq!(deletes.last().unwrap(), &vec![table.addr()]);
        assert!(deletes.contains(&vec![entries]));
    }
}
