//! `BSMain/run_00c416a0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The search state the function runs on (offsets as the code uses them;
/// the class is not named in the code): the map +0x00 (its bucket array
/// at +0x0c, its slots `0x34` and `0x38` compare and hand back an entry),
/// three hazard slots (pointers to the words that publish the nodes in
/// use) +0x04, +0x08 and +0x0c, then the link being followed +0x10, the
/// current node word +0x14 and the next node word +0x18. Bit 0 of a node
/// word marks the node it leads from as deleted.
const MAP: u32 = 0x00;
const MAP_BUCKETS: u32 = 0x0c;
const HAZARD_NEXT: u32 = 0x04;
const HAZARD_CURRENT: u32 = 0x08;
const HAZARD_PREVIOUS: u32 = 0x0c;
const LINK: u32 = 0x10;
const CURRENT: u32 = 0x14;
const NEXT: u32 = 0x18;
/// A node: the key's two words +0x00 and +0x04, the next node word +0x0c.
const NODE_NEXT: u32 = 0x0c;
const MARK: u32 = 1;

/// `0044e010` (`thiscall` on a stack word holding a node, one word):
/// publishes the node (the hazard-pointer step) before it is re-checked.
const PUBLISH: u32 = 0x0044_e010;
/// `00c41f40` (unit `run_00c41f40`): puts an unlinked node on the free list.
const RETIRE_NODE: u32 = 0x00c4_1f40;
/// `InterlockedCompareExchange` (import slot; `(destination, exchange,
/// comparand)`, the old value).
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
/// The map's virtual slots `0x34` (whether a node's key matches) and `0x38`
/// (the result for a match), both `(node key low, node key high, key low,
/// key high)`.
const SLOT_KEY_MATCHES: u32 = 0x34;
const SLOT_FOUND: u32 = 0x38;

// Translated from 00c416a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c416a0`): the lock-free search of
/// bucket `bucket` of the map for the 64-bit key. From the bucket's head it
/// walks the list publishing each node it holds in the hazard slots and
/// re-checking that the links did not change (any change restarts from the
/// head). A live node whose key matches (slot `0x34`) gives the result of
/// slot `0x38`; a node marked deleted is unlinked with a compare-exchange
/// (a failure restarts) and retired (`00c41f40`). The end of the list
/// returns false.
///
/// The compiler's constant loop conditions never end the loops, and its
/// constant tests that would set the mark bit on the two copied node words
/// never do; both are kept as written. A restart flag the code keeps in a
/// local makes no difference: both of its ways restart.
pub fn fn_00c416a0(e: &mut Engine, this: Ptr, bucket: u32, key_low: u32, key_high: u32) -> bool {
    let this = this.addr();
    let field = |e: &Engine, offset: u32| e.mem.u32(this.wrapping_add(offset));
    let map = field(e, MAP);
    let head = e
        .mem
        .u32(map.wrapping_add(MAP_BUCKETS))
        .wrapping_add(bucket.wrapping_mul(4));
    e.with_stack(8, |e, frame| {
        let published = frame.addr();
        let expected = published + 4;
        'restart: loop {
            e.mem.set_u32(this.wrapping_add(LINK), head);
            let current = e.mem.u32(field(e, LINK));
            e.mem.set_u32(this.wrapping_add(CURRENT), current);
            e.mem.set_u32(field(e, HAZARD_CURRENT), current & !MARK);
            e.mem.set_u32(published, field(e, CURRENT) & !MARK);
            e.call(PUBLISH, &args![published, 0u32]);
            if e.mem.u32(field(e, LINK)) != e.mem.u32(published) {
                continue 'restart;
            }
            loop {
                let node = field(e, CURRENT) & !MARK;
                if node == 0 {
                    return false;
                }
                let next = e.mem.u32(node.wrapping_add(NODE_NEXT));
                e.mem.set_u32(this.wrapping_add(NEXT), next);
                e.mem.set_u32(field(e, HAZARD_NEXT), field(e, NEXT) & !MARK);
                let node = field(e, CURRENT) & !MARK;
                if field(e, NEXT) != e.mem.u32(node.wrapping_add(NODE_NEXT)) {
                    continue 'restart;
                }
                let node = field(e, CURRENT) & !MARK;
                let node_key_low = e.mem.u32(node);
                let node_key_high = e.mem.u32(node.wrapping_add(4));
                // The current node word without its mark (the constant
                // "marked" test is false).
                e.mem.set_u32(expected, field(e, CURRENT) & !MARK);
                let expected_word = e.mem.u32(expected) & !MARK;
                e.mem.set_u32(expected, expected_word);
                if e.mem.u32(field(e, LINK)) != e.mem.u32(expected) {
                    continue 'restart;
                }
                if field(e, NEXT) & MARK == 0 {
                    let map = field(e, MAP);
                    let matches = e
                        .vcall(
                            map,
                            SLOT_KEY_MATCHES,
                            &args![node_key_low, node_key_high, key_low, key_high],
                        )
                        .u8();
                    if matches != 0 {
                        let map = field(e, MAP);
                        return e
                            .vcall(
                                map,
                                SLOT_FOUND,
                                &args![node_key_low, node_key_high, key_low, key_high],
                            )
                            .u8()
                            != 0;
                    }
                    let link = (field(e, CURRENT) & !MARK).wrapping_add(NODE_NEXT);
                    e.mem.set_u32(this.wrapping_add(LINK), link);
                    e.mem
                        .set_u32(field(e, HAZARD_PREVIOUS), field(e, CURRENT) & !MARK);
                } else {
                    // The next node word without its mark (the constant
                    // "marked" test is false).
                    let unlinked = field(e, NEXT) & !MARK;
                    let comparand = e.mem.u32(expected);
                    let old = e
                        .call(
                            INTERLOCKED_COMPARE_EXCHANGE,
                            &args![field(e, LINK), unlinked, comparand],
                        )
                        .u32();
                    if old != comparand {
                        continue 'restart;
                    }
                    let node = field(e, CURRENT) & !MARK;
                    e.call(RETIRE_NODE, &args![this, node]);
                }
                let next = field(e, NEXT);
                e.mem.set_u32(this.wrapping_add(CURRENT), next);
                e.mem
                    .set_u32(field(e, HAZARD_CURRENT), field(e, NEXT) & !MARK);
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c4_16a0, fn_00c416a0(Ptr, u32, u32, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const MATCHES: u32 = 0x00ee_0100;
    const FOUND: u32 = 0x00ee_0200;

    /// A map with one bucket holding nodes with the given key words and
    /// next words (indices into the node list, or `None`), and a search
    /// state on it.
    fn search(nodes: &[(u32, Option<usize>, bool)]) -> (Engine, Log, u32, u32, Vec<u32>) {
        let mut e = Engine::new();
        let log = Log::default();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_KEY_MATCHES, MATCHES);
        e.mem.set_u32(vtable + SLOT_FOUND, FOUND);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, vtable);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + MAP_BUCKETS, buckets);
        let addresses: Vec<u32> = nodes.iter().map(|_| e.mem.alloc(0x10)).collect();
        for (i, (key, next, marked)) in nodes.iter().enumerate() {
            let next = next.map_or(0, |n| addresses[n]) | u32::from(*marked);
            e.mem.set_u32(addresses[i], *key);
            e.mem.set_u32(addresses[i] + NODE_NEXT, next);
        }
        if let Some(first) = addresses.first() {
            e.mem.set_u32(buckets + 4, *first);
        }
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + MAP, map);
        for hazard in [HAZARD_NEXT, HAZARD_CURRENT, HAZARD_PREVIOUS] {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(this + hazard, slot);
        }
        e.register(PUBLISH, |_, _| Ret::default());
        let l = log.clone();
        e.register_double(MATCHES, move |_, a| {
            l.borrow_mut().push((MATCHES, a.to_vec()));
            u32::from(a[1] == a[3]).into_ret()
        });
        let l = log.clone();
        e.register_double(FOUND, move |_, a| {
            l.borrow_mut().push((FOUND, a.to_vec()));
            0x101u32.into_ret()
        });
        let l = log.clone();
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, move |e, a| {
            l.borrow_mut()
                .push((INTERLOCKED_COMPARE_EXCHANGE, a.to_vec()));
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            old.into_ret()
        });
        let l = log.clone();
        e.register_double(RETIRE_NODE, move |_, a| {
            l.borrow_mut().push((RETIRE_NODE, a.to_vec()));
            Ret::default()
        });
        (e, log, this, buckets, addresses)
    }

    #[test]
    fn finds_a_matching_live_node() {
        let (mut e, log, this, buckets, nodes) = search(&[(1, Some(1), false), (2, None, false)]);
        assert!(e.call(0x00c4_16a0, &args![this, 1u32, 2u32, 0u32]).bool());
        assert_eq!(
            *log.borrow(),
            [
                (MATCHES, vec![e.mem.u32(this), 1, 0, 2, 0]),
                (MATCHES, vec![e.mem.u32(this), 2, 0, 2, 0]),
                (FOUND, vec![e.mem.u32(this), 2, 0, 2, 0]),
            ]
        );
        assert_eq!(e.mem.u32(this + LINK), nodes[0] + NODE_NEXT);
        assert_eq!(e.mem.u32(this + CURRENT), nodes[1]);
        assert_eq!(e.mem.u32(e.mem.u32(this + HAZARD_PREVIOUS)), nodes[0]);
        assert_eq!(e.mem.u32(buckets + 4), nodes[0]);
    }

    #[test]
    fn returns_false_at_the_end_of_the_list() {
        let (mut e, log, this, _, _) = search(&[]);
        assert!(!e.call(0x00c4_16a0, &args![this, 1u32, 2u32, 0u32]).bool());
        assert!(log.borrow().is_empty());
        let (mut e, log, this, _, _) = search(&[(1, None, false)]);
        assert!(!e.call(0x00c4_16a0, &args![this, 1u32, 2u32, 0u32]).bool());
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn unlinks_and_retires_a_deleted_node() {
        // Node 0 is marked deleted (its next word has bit 0 set).
        let (mut e, log, this, buckets, nodes) = search(&[(1, Some(1), true), (3, None, false)]);
        assert!(!e.call(0x00c4_16a0, &args![this, 1u32, 2u32, 0u32]).bool());
        let log = log.borrow();
        assert_eq!(
            log[0],
            (
                INTERLOCKED_COMPARE_EXCHANGE,
                vec![buckets + 4, nodes[1], nodes[0]]
            )
        );
        assert_eq!(log[1], (RETIRE_NODE, vec![this, nodes[0]]));
        assert_eq!(log[2].0, MATCHES);
        assert_eq!(e.mem.u32(buckets + 4), nodes[1]);
    }
}
