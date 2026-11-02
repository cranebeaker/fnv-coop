//! `fallout shared/run_00528e10` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Members of the lock-free map template that `ExteriorCellLoader` keeps its
//! tasks in (`LockFreeMap<unsigned int, NiPointer<ExteriorCellLoaderTask>>`,
//! from the RTTI of its vtable `0102d084`; `ThreadSafeStructures.inl`), and
//! of the hazard-pointer bookkeeping it builds with `006c73b0`.
//!
//! The map (the words `005289e0` sets): +0x04 the node storage, +0x08 the
//! bucket count, +0x0C the bucket array (one tagged node pointer each),
//! +0x10 the item size, +0x14 the hazard-pointer record set, +0x20 a lock.
//! A node: its value at +0x00, a smart pointer at +0x04, the next node at
//! +0x08 with bit 0 marking the node as removed.
//!
//! The search cursor [`fn_00529030`] works on: +0x00 the map, +0x04,
//! +0x08 and +0x0C three hazard slots (pointers to words), +0x10 the link
//! being followed, +0x14 the current node, +0x18 the next node (tagged
//! words). The retire list [`fn_005292a0`] works on: +0x00 an object whose
//! word at +0x10 is the scan threshold, +0x1C the last retired node, +0x20
//! the retired count.

#[allow(unused_imports)]
use crate::prelude::*;

/// `LockFreeMap`'s destructor (`exteriorcellloader.rs`).
const MAP_DESTRUCT: u32 = 0x0052_8be0;
/// `operator delete` (cdecl, one argument).
const FREE: u32 = 0x0040_1030;
/// `operator new` (cdecl, one argument).
const ALLOCATE: u32 = 0x0040_1000;
/// A debug scope guard's constructor `(guard, 6, 1, file, line)` and
/// destructor `(guard)`.
const SCOPE_ENTER: u32 = 0x0040_4eb0;
const SCOPE_LEAVE: u32 = 0x0040_4ee0;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.inl"`.
const SOURCE_FILE: u32 = 0x0101_73f8;
/// The map's lock: enter `(lock, 0)`, leave `(lock)`.
const LOCK_ENTER: u32 = 0x0040_fbf0;
const LOCK_LEAVE: u32 = 0x0040_fba0;
/// The first word of an object (`(object)`).
const FIRST_WORD: u32 = 0x0055_9450;
/// A tagged pointer word without its mark bit (`*word & !1`).
const UNTAGGED: u32 = 0x0044_dec0;
/// The same through a second entry (`0044df00` calls `0044dec0`).
const UNTAGGED_NODE: u32 = 0x0044_df00;
/// Whether a tagged pointer word has its mark bit (`*word & 1`).
const IS_MARKED: u32 = 0x0044_dff0;
/// Clears a word (`*word = 0`).
const CLEAR_WORD: u32 = 0x0066_65a0;
/// `NiPointer<QueuedFile>::operator=` (Xbox PDB) on the node's smart
/// pointer, with null.
const SMART_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// The map's node release `(map, node)` (the map's slot 0x28 with the
/// node's value, then `0044ed50` on the node with 1).
const RELEASE_NODE: u32 = 0x0044_edd0;
/// The hazard-pointer record set's constructor `(set, count)`.
const RECORD_SET_CONSTRUCT: u32 = 0x006c_73b0;
/// The address of bucket `index` of a map (`[map + 0xc] + 4 * index`).
const BUCKET: u32 = 0x0044_ee20;
/// An empty function (the engine map's `Error`; the place of a memory
/// barrier).
const BARRIER: u32 = 0x0040_fbe0;
/// Stores a tagged word into a stack temporary `(temporary, 0, word)`.
const TAGGED_TEMPORARY: u32 = 0x006e_cba0;
/// Compare-and-swap of a link `(link, new, expected)` (cdecl, `bool`).
const COMPARE_EXCHANGE: u32 = 0x0044_91c0;
/// Stores `previous` at +4 of `node` (`(node, previous)`; the engine map's
/// folded name `CombatProcedureAttackMelee::Initialize`).
const LINK_RETIRED: u32 = 0x006e_cd40;
/// Reads the word at +0x10 of an object (the engine map's folded name
/// `BaseProcess::GetCurrentProcedureIndex`).
const RETIRE_THRESHOLD: u32 = 0x0044_edb0;
/// Frees the retired nodes no hazard slot holds (`(list)`).
const SCAN_RETIRED: u32 = 0x0044_f070;
/// `TlsFree` wrapper (cdecl, the TLS index).
const TLS_FREE: u32 = 0x0044_e360;
/// Drains a retire list (`0044f070` until its count is zero).
const DRAIN_RETIRED: u32 = 0x0044_f420;

fn word(e: &Engine, object: u32, offset: u32) -> u32 {
    e.mem.u32(object.wrapping_add(offset))
}

// Translated from 00528e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<unsigned int, NiPointer<ExteriorCellLoaderTask>>`'s scalar
/// deleting destructor (slot 0 of `0102d084`): the destructor (`00528be0`),
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00528e10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00528e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the map, under its lock and a debug scope guard (line 0x42C):
/// nothing when it has no record set (+0x14). Otherwise it keeps the
/// record set's first word (its count), destroys the record set
/// ([`fn_00529300`]), clears +0x14 and +0x18, and unlinks every bucket's
/// chain, releasing each node (its smart pointer set to null, then
/// `0044edd0`). Unless `keep_empty` is non-zero it then makes a new record
/// set of 0x10 bytes with the same count (null if the allocation fails).
// Translation incomplete: 00528e40 - the C++ exception-unwinding frame (FS:[0]) that frees the new record set and leaves the scope guard when a constructor throws is not translated.
pub fn fn_00528e40(e: &mut Engine, this: Ptr, keep_empty: u8) {
    let base = this.addr();
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_ENTER,
            &args![guard, 6u32, 1u32, SOURCE_FILE, 0x42cu32],
        );
        if word(e, base, 0x14) == 0 {
            e.call(SCOPE_LEAVE, &args![guard]);
            return;
        }
        e.call(LOCK_ENTER, &args![base + 0x20, 0u32]);
        let records = word(e, base, 0x14);
        let count = e.call(FIRST_WORD, &args![records]).u32();
        let records = word(e, base, 0x14);
        if records != 0 {
            fn_00529300(e, Ptr::new(records), 1);
        }
        e.mem.set_u32(base + 0x14, 0);
        e.mem.set_u32(base + 0x18, 0);
        let mut index = 0u32;
        while index < word(e, base, 8) {
            let slot = word(e, base, 0xc).wrapping_add(index.wrapping_mul(4));
            let mut node = e.call(UNTAGGED, &args![slot]).u32();
            let slot = word(e, base, 0xc).wrapping_add(index.wrapping_mul(4));
            e.call(CLEAR_WORD, &args![slot]);
            while node != 0 {
                let next = e.call(UNTAGGED, &args![node + 8]).u32();
                e.call(CLEAR_WORD, &args![node + 8]);
                e.call(SMART_POINTER_ASSIGN, &args![node + 4, 0u32]);
                e.call(RELEASE_NODE, &args![this, node]);
                node = next;
            }
            index += 1;
        }
        if keep_empty == 0 {
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            let records = if block == 0 {
                0
            } else {
                e.call(RECORD_SET_CONSTRUCT, &args![block, count]).u32()
            };
            e.mem.set_u32(base + 0x14, records);
        }
        e.call(LOCK_LEAVE, &args![base + 0x20]);
        e.call(SCOPE_LEAVE, &args![guard]);
    });
}

// Translated from 00529000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the cursor's three hazard slots (the words its pointers at +0x04,
/// +0x08 and +0x0C point to).
pub fn fn_00529000(e: &mut Engine, this: Ptr) {
    for offset in [4, 8, 0xc] {
        let slot = word(e, this.addr(), offset);
        e.mem.set_u32(slot, 0);
    }
}

/// Where [`fn_00529030`]'s inner walk goes next.
enum Step {
    /// Start again from the bucket.
    Restart,
    /// Move to the next node.
    Advance,
    /// The function returns this word.
    Return(u32),
}

// Translated from 00529030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Searches the chain of bucket `bucket_index` with the cursor `this`
/// (hazard-pointer list search). It follows the links, publishing each node
/// in the hazard slots and checking that the link it came from still holds
/// it (otherwise it starts again from the bucket). A node whose next link is
/// marked is unlinked by compare-and-swap (`004491c0`) and retired
/// ([`fn_005292a0`]); a failed swap starts again. For a live node it asks
/// the map's slot 0x34 with `(value, key)`; on a match it returns the map's
/// slot 0x38 with the same words, otherwise it moves on (+0x10 becomes the
/// node's link, its hazard slot at +0x0C the node). At the end of the chain
/// it returns 0.
pub fn fn_00529030(e: &mut Engine, this: Ptr, bucket_index: u32, key: u32) -> u32 {
    let base = this.addr();
    let map = word(e, base, 0);
    let bucket = e.call(BUCKET, &args![map, bucket_index]).u32();
    // The three stack temporaries 006ecba0 fills (EBP-0xC, -0x10, -0x18).
    e.with_stack(12, |e, temporaries| {
        let first = temporaries.addr();
        let current_copy = first + 4;
        let next_copy = first + 8;
        loop {
            // Start from the bucket.
            e.mem.set_u32(base + 0x10, bucket);
            let head = word(e, bucket, 0);
            e.mem.set_u32(base + 0x14, head);
            let current = e.call(UNTAGGED, &args![base + 0x14]).u32();
            let slot = word(e, base, 8);
            e.mem.set_u32(slot, current);
            e.call(BARRIER, &args![]);
            let current = e.call(UNTAGGED, &args![base + 0x14]).u32();
            e.call(TAGGED_TEMPORARY, &args![first, 0u32, current]);
            let link = word(e, base, 0x10);
            let linked = e.call(FIRST_WORD, &args![link]).u32();
            let published = e.call(FIRST_WORD, &args![first]).u32();
            if linked != published {
                continue;
            }
            loop {
                let step = walk_one(e, base, key, current_copy, next_copy);
                match step {
                    Step::Restart => break,
                    Step::Advance => {
                        let next = word(e, base, 0x18);
                        e.mem.set_u32(base + 0x14, next);
                        let next = e.call(UNTAGGED, &args![base + 0x18]).u32();
                        let slot = word(e, base, 8);
                        e.mem.set_u32(slot, next);
                        e.call(BARRIER, &args![]);
                    }
                    Step::Return(value) => return value,
                }
            }
        }
    })
}

/// One node of [`fn_00529030`]'s walk (`005290bc` to `0052925a`).
fn walk_one(e: &mut Engine, base: u32, key: u32, current_copy: u32, next_copy: u32) -> Step {
    if e.call(UNTAGGED, &args![base + 0x14]).u32() == 0 {
        return Step::Return(0);
    }
    let node = e.call(UNTAGGED_NODE, &args![base + 0x14]).u32();
    let next = word(e, node, 8);
    e.mem.set_u32(base + 0x18, next);
    let next = e.call(UNTAGGED, &args![base + 0x18]).u32();
    let slot = word(e, base, 4);
    e.mem.set_u32(slot, next);
    e.call(BARRIER, &args![]);
    let next_word = e.call(FIRST_WORD, &args![base + 0x18]).u32();
    let node = e.call(UNTAGGED_NODE, &args![base + 0x14]).u32();
    let node_link = e.call(FIRST_WORD, &args![node + 8]).u32();
    if next_word != node_link {
        return Step::Restart;
    }
    let node = e.call(UNTAGGED, &args![base + 0x14]).u32();
    let value = word(e, node, 0);
    let node = e.call(UNTAGGED, &args![base + 0x14]).u32();
    e.call(TAGGED_TEMPORARY, &args![current_copy, 0u32, node]);
    let link = word(e, base, 0x10);
    let linked = e.call(FIRST_WORD, &args![link]).u32();
    let published = e.call(FIRST_WORD, &args![current_copy]).u32();
    if linked != published {
        return Step::Restart;
    }
    if !e.call(IS_MARKED, &args![base + 0x18]).bool() {
        let map = word(e, base, 0);
        if e.vcall(map, 0x34, &args![value, key]).bool() {
            let map = word(e, base, 0);
            return Step::Return(e.vcall(map, 0x38, &args![value, key]).u32());
        }
        let node = e.call(UNTAGGED_NODE, &args![base + 0x14]).u32();
        e.mem.set_u32(base + 0x10, node + 8);
        let node = e.call(UNTAGGED, &args![base + 0x14]).u32();
        let slot = word(e, base, 0xc);
        e.mem.set_u32(slot, node);
        e.call(BARRIER, &args![]);
    } else {
        let next = e.call(UNTAGGED, &args![base + 0x18]).u32();
        e.call(TAGGED_TEMPORARY, &args![next_copy, 0u32, next]);
        let expected = e.call(FIRST_WORD, &args![current_copy]).u32();
        let replacement = e.call(FIRST_WORD, &args![next_copy]).u32();
        let link = word(e, base, 0x10);
        if !e
            .call(COMPARE_EXCHANGE, &args![link, replacement, expected])
            .bool()
        {
            return Step::Restart;
        }
        let node = e.call(UNTAGGED, &args![base + 0x14]).u32();
        fn_005292a0(e, Ptr::new(base), Ptr::new(node));
    }
    Step::Advance
}

// Translated from 005292a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Retires `node`: its smart pointer at +4 set to null, then the last
/// retired node (+0x1C) stored at its +4 (`006ecd40`), `node` made the
/// last, and the count at +0x20 raised; when the count equals the word at
/// +0x10 of the object the word at +0 points to (`0044edb0`), the retired
/// nodes are scanned (`0044f070`).
pub fn fn_005292a0(e: &mut Engine, this: Ptr, node: Ptr) {
    let base = this.addr();
    e.call(SMART_POINTER_ASSIGN, &args![node.addr() + 4, 0u32]);
    let previous = word(e, base, 0x1c);
    e.call(LINK_RETIRED, &args![node, previous]);
    e.mem.set_u32(base + 0x1c, node.addr());
    let count = word(e, base, 0x20).wrapping_add(1);
    e.mem.set_u32(base + 0x20, count);
    let records = word(e, base, 0);
    let threshold = e.call(RETIRE_THRESHOLD, &args![records]).u32();
    if word(e, base, 0x20) == threshold {
        e.call(SCAN_RETIRED, &args![this]);
    }
}

// Translated from 00529300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hazard-pointer record set's scalar deleting destructor: the
/// destructor ([`fn_00529330`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00529300(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00529330(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00529330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hazard-pointer record set's destructor: for each of its records
/// (count: the word at +0, re-read each time; 8-byte records from the
/// array at +8) whose second word is not null, that object's scalar
/// deleting destructor with 1 ([`fn_005293c0`]); then the array is freed
/// and the TLS index at +4 released (`0044e360`).
pub fn fn_00529330(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < e.call(FIRST_WORD, &args![this]).u32() {
        let records = word(e, base, 8);
        let list = word(e, records.wrapping_add(index.wrapping_mul(8)), 4);
        if list != 0 {
            fn_005293c0(e, Ptr::new(list), 1);
        }
        index += 1;
    }
    let records = word(e, base, 8);
    e.call(FREE, &args![records]);
    let tls_index = word(e, base, 4);
    e.call(TLS_FREE, &args![tls_index]);
}

// Translated from 005293c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A record's retire list scalar deleting destructor: drains the list
/// (`0044f420`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_005293c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DRAIN_RETIRED, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00528e10, fn_00528e10(Ptr, u32) -> Ptr),
        entry!(0x00528e40, fn_00528e40(Ptr, u8)),
        entry!(0x00529000, fn_00529000(Ptr)),
        entry!(0x00529030, fn_00529030(Ptr, u32, u32) -> u32),
        entry!(0x005292a0, fn_005292a0(Ptr, Ptr)),
        entry!(0x00529300, fn_00529300(Ptr, u32) -> Ptr),
        entry!(0x00529330, fn_00529330(Ptr)),
        entry!(0x005293c0, fn_005293c0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// Doubles of the word helpers, doing what their bodies do.
    fn double_word_helpers(e: &mut Engine) {
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(UNTAGGED, |e, a| (e.mem.u32(a[0]) & !1).into_ret());
        e.register(UNTAGGED_NODE, |e, a| (e.mem.u32(a[0]) & !1).into_ret());
        e.register(IS_MARKED, |e, a| (e.mem.u32(a[0]) & 1 != 0).into_ret());
        e.register(CLEAR_WORD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(BUCKET, |e, a| (e.mem.u32(a[0] + 0xc) + 4 * a[1]).into_ret());
        e.register(TAGGED_TEMPORARY, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            a[0].into_ret()
        });
        e.register(COMPARE_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            (old == a[2]).into_ret()
        });
        nothing(e, &[BARRIER]);
    }

    #[test]
    fn deleting_destructors_free_only_with_bit_zero() {
        let mut e = Engine::new();
        nothing(&mut e, &[MAP_DESTRUCT, FREE, DRAIN_RETIRED]);
        let object = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_8e10, &[object, 0]).u32(), object);
        assert_eq!(e.call(0x0052_93c0, &[object, 0]).u32(), object);
        assert!(calls_to(&e, FREE).is_empty());
        e.call(0x0052_8e10, &[object, 1]);
        e.call(0x0052_93c0, &[object, 1]);
        assert_eq!(calls_to(&e, FREE), vec![vec![object], vec![object]]);
        assert_eq!(calls_to(&e, MAP_DESTRUCT), vec![vec![object]; 2]);
        assert_eq!(calls_to(&e, DRAIN_RETIRED), vec![vec![object]; 2]);
    }

    #[test]
    fn record_set_destructor_destroys_live_lists_then_frees() {
        let mut e = Engine::new();
        nothing(&mut e, &[FREE, DRAIN_RETIRED, TLS_FREE]);
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        let set = e.mem.alloc(0x10);
        let records = e.mem.alloc(0x18);
        e.mem.set_u32(set, 3);
        e.mem.set_u32(set + 4, 0x2a);
        e.mem.set_u32(set + 8, records);
        e.mem.set_u32(records + 4, 0x0300_0000);
        e.mem.set_u32(records + 0x14, 0x0300_0100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_9300, &[set, 1]).u32(), set);
        assert_eq!(
            calls_to(&e, DRAIN_RETIRED),
            vec![vec![0x0300_0000], vec![0x0300_0100]]
        );
        assert_eq!(
            calls_to(&e, FREE),
            vec![
                vec![0x0300_0000],
                vec![0x0300_0100],
                vec![records],
                vec![set]
            ]
        );
        assert_eq!(calls_to(&e, TLS_FREE), vec![vec![0x2a]]);
        assert_eq!(calls_to(&e, FIRST_WORD).len(), 4);
    }

    #[test]
    fn clearing_releases_every_node_and_rebuilds_the_record_set() {
        let mut e = Engine::new();
        double_word_helpers(&mut e);
        nothing(
            &mut e,
            &[
                SCOPE_ENTER,
                SCOPE_LEAVE,
                LOCK_ENTER,
                LOCK_LEAVE,
                SMART_POINTER_ASSIGN,
                RELEASE_NODE,
                FREE,
                DRAIN_RETIRED,
                TLS_FREE,
            ],
        );
        e.register(ALLOCATE, |_, _| 0x0400_0000u32.into_ret());
        e.register(RECORD_SET_CONSTRUCT, |_, a| a[0].into_ret());
        let map = e.mem.alloc(0x24);
        let buckets = e.mem.alloc(8);
        let set = e.mem.alloc(0x10);
        let node_a = e.mem.alloc(0xc);
        let node_b = e.mem.alloc(0xc);
        e.mem.set_u32(map + 8, 2);
        e.mem.set_u32(map + 0xc, buckets);
        e.mem.set_u32(map + 0x14, set);
        e.mem.set_u32(map + 0x18, 9);
        e.mem.set_u32(set, 5);
        let records = e.mem.alloc(0x28);
        e.mem.set_u32(set + 8, records);
        e.mem.set_u32(buckets + 4, node_a | 1);
        e.mem.set_u32(node_a + 8, node_b);
        e.call_log = Some(vec![]);
        e.call(0x0052_8e40, &[map, 0]);
        assert_eq!(
            calls_to(&e, RELEASE_NODE),
            vec![vec![map, node_a], vec![map, node_b]]
        );
        assert_eq!(
            calls_to(&e, SMART_POINTER_ASSIGN),
            vec![vec![node_a + 4, 0], vec![node_b + 4, 0]]
        );
        assert_eq!(e.mem.u32(buckets + 4), 0);
        assert_eq!(e.mem.u32(node_a + 8), 0);
        assert_eq!(e.mem.u32(map + 0x18), 0);
        assert_eq!(
            calls_to(&e, RECORD_SET_CONSTRUCT),
            vec![vec![0x0400_0000, 5]]
        );
        assert_eq!(e.mem.u32(map + 0x14), 0x0400_0000);
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![map + 0x20, 0]]);
        assert_eq!(calls_to(&e, LOCK_LEAVE), vec![vec![map + 0x20]]);
        assert_eq!(
            calls_to(&e, SCOPE_ENTER)[0][1..],
            [6, 1, SOURCE_FILE, 0x42c]
        );
        assert_eq!(calls_to(&e, SCOPE_LEAVE).len(), 1);
    }

    #[test]
    fn clearing_without_records_or_with_keep_empty() {
        let mut e = Engine::new();
        double_word_helpers(&mut e);
        nothing(
            &mut e,
            &[
                SCOPE_ENTER,
                SCOPE_LEAVE,
                LOCK_ENTER,
                LOCK_LEAVE,
                FREE,
                TLS_FREE,
            ],
        );
        let map = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0052_8e40, &[map, 0]);
        assert!(calls_to(&e, LOCK_ENTER).is_empty());
        assert_eq!(calls_to(&e, SCOPE_LEAVE).len(), 1);

        let set = e.mem.alloc(0x10);
        let records = e.mem.alloc(8);
        e.mem.set_u32(set + 8, records);
        e.mem.set_u32(map + 0x14, set);
        e.call(0x0052_8e40, &[map, 1]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(map + 0x14), 0);
        assert_eq!(
            calls_to(&e, FREE),
            vec![vec![e.mem.u32(set + 8)], vec![set]]
        );
    }

    #[test]
    fn clearing_hazard_slots() {
        let mut e = Engine::new();
        let cursor = e.mem.alloc(0x1c);
        let slots = e.mem.alloc(0xc);
        e.mem.write(slots, &[0xff; 12]);
        for i in 0..3 {
            e.mem.set_u32(cursor + 4 + 4 * i, slots + 4 * i);
        }
        e.call(0x0052_9000, &[cursor]);
        assert_eq!(e.mem.bytes(slots, 12), vec![0; 12]);
    }

    /// A map whose slot 0x34 matches equal words and slot 0x38 returns
    /// `0x77`, a cursor with three hazard slots, and bucket 1 of two.
    fn search_world(e: &mut Engine) -> (u32, u32, u32) {
        double_word_helpers(e);
        let mut slots = vec![0u32; 13];
        slots.extend([0x0150_1000, 0x0150_1100]);
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1000, |_, a| (a[1] == a[2]).into_ret());
        e.register(0x0150_1100, |_, _| 0x77u32.into_ret());
        let map = e.mem.alloc(0x24);
        e.mem.set_u32(map, 0x0150_0000);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 0xc, buckets);
        let cursor = e.mem.alloc(0x24);
        let slots = e.mem.alloc(0xc);
        e.mem.set_u32(cursor, map);
        for i in 0..3 {
            e.mem.set_u32(cursor + 4 + 4 * i, slots + 4 * i);
        }
        (cursor, buckets, slots)
    }

    fn node(e: &mut Engine, value: u32, next: u32) -> u32 {
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node, value);
        e.mem.set_u32(node + 8, next);
        node
    }

    #[test]
    fn search_finds_a_later_node() {
        let mut e = Engine::new();
        let (cursor, buckets, slots) = search_world(&mut e);
        let second = node(&mut e, 7, 0);
        let first = node(&mut e, 5, second);
        e.mem.set_u32(buckets + 4, first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_9030, &[cursor, 1, 7]).u32(), 0x77);
        let map = e.mem.u32(cursor);
        assert_eq!(
            calls_to(&e, 0x0150_1000),
            vec![vec![map, 5, 7], vec![map, 7, 7]]
        );
        assert_eq!(e.mem.u32(cursor + 0x10), first + 8);
        assert_eq!(e.mem.u32(slots + 8), first);
        assert_eq!(e.mem.u32(slots + 4), second);
    }

    #[test]
    fn search_returns_zero_at_the_end() {
        let mut e = Engine::new();
        let (cursor, buckets, _) = search_world(&mut e);
        let first = node(&mut e, 5, 0);
        e.mem.set_u32(buckets + 4, first);
        assert_eq!(e.call(0x0052_9030, &[cursor, 1, 9]).u32(), 0);
        e.mem.set_u32(buckets, 0);
        assert_eq!(e.call(0x0052_9030, &[cursor, 0, 9]).u32(), 0);
    }

    #[test]
    fn search_unlinks_and_retires_a_marked_node() {
        let mut e = Engine::new();
        let (cursor, buckets, _) = search_world(&mut e);
        nothing(&mut e, &[SMART_POINTER_ASSIGN, LINK_RETIRED, SCAN_RETIRED]);
        e.register(RETIRE_THRESHOLD, |_, _| 100u32.into_ret());
        let second = node(&mut e, 7, 0);
        let first = node(&mut e, 5, second | 1);
        e.mem.set_u32(buckets + 4, first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_9030, &[cursor, 1, 7]).u32(), 0x77);
        assert_eq!(e.mem.u32(buckets + 4), second);
        assert_eq!(calls_to(&e, LINK_RETIRED), vec![vec![first, 0]]);
        assert_eq!(e.mem.u32(cursor + 0x1c), first);
        assert_eq!(e.mem.u32(cursor + 0x20), 1);
        // The marked node is never offered to slot 0x34.
        assert_eq!(calls_to(&e, 0x0150_1000).len(), 1);
    }

    #[test]
    fn search_restarts_when_the_swap_fails() {
        let mut e = Engine::new();
        let (cursor, buckets, _) = search_world(&mut e);
        nothing(&mut e, &[SMART_POINTER_ASSIGN, LINK_RETIRED, SCAN_RETIRED]);
        e.register(RETIRE_THRESHOLD, |_, _| 100u32.into_ret());
        let mut failures = 1;
        e.register_double(COMPARE_EXCHANGE, move |e, a| {
            if failures > 0 {
                failures -= 1;
                return false.into_ret();
            }
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            (old == a[2]).into_ret()
        });
        let first = node(&mut e, 5, 1);
        e.mem.set_u32(buckets + 4, first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_9030, &[cursor, 1, 7]).u32(), 0);
        assert_eq!(calls_to(&e, COMPARE_EXCHANGE).len(), 2);
        assert_eq!(calls_to(&e, BUCKET).len(), 1);
        assert_eq!(e.mem.u32(buckets + 4), 0);
    }

    #[test]
    fn retiring_scans_at_the_threshold() {
        let mut e = Engine::new();
        nothing(&mut e, &[SMART_POINTER_ASSIGN, LINK_RETIRED, SCAN_RETIRED]);
        e.register(RETIRE_THRESHOLD, |e, a| e.mem.u32(a[0]).into_ret());
        let records = e.mem.alloc(4);
        e.mem.set_u32(records, 2);
        let list = e.mem.alloc(0x24);
        e.mem.set_u32(list, records);
        e.mem.set_u32(list + 0x1c, 0x0500_0000);
        e.call_log = Some(vec![]);
        e.call(0x0052_92a0, &[list, 0x0600_0000]);
        assert!(calls_to(&e, SCAN_RETIRED).is_empty());
        assert_eq!(
            calls_to(&e, SMART_POINTER_ASSIGN),
            vec![vec![0x0600_0004, 0]]
        );
        assert_eq!(
            calls_to(&e, LINK_RETIRED),
            vec![vec![0x0600_0000, 0x0500_0000]]
        );
        e.call(0x0052_92a0, &[list, 0x0600_0100]);
        assert_eq!(calls_to(&e, SCAN_RETIRED), vec![vec![list]]);
        assert_eq!(e.mem.u32(list + 0x1c), 0x0600_0100);
        assert_eq!(e.mem.u32(list + 0x20), 2);
    }
}
