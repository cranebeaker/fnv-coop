//! A thread's interface to a map (`LockFreeMapInterface`, Xbox PDB): the lock-free bucket walks.
//! The interface keeps its place (the link to the current node, the current and the next node) in
//! its own fields, and publishes the nodes it is reading in its three slots of the map's
//! `pReferencedNodes`, so a thread deleting a node leaves it until no slot names it.
//!
//! The marked-pointer helpers the exe calls for each step (`0044dec0`, `0044df00`, `0044df20`,
//! `0044dff0`, `00537e90`, `0044e010`, `006ecba0`, `0044ee20`) and the release of the slots at the
//! end (`00529000`) are done in place here: they read and write the same words.

use super::layout::{
    BucketHead, HashListNode, Holder, InterfaceManager, LockFreeMap, MapInterface, Marked, NodeSlot,
};
use super::{
    holder, FAMILY, OPERATOR_DELETE, OPERATOR_NEW, SCOPE_BEGIN, SCOPE_CONTEXT, SCOPE_END,
    SCOPE_MODE, SOURCE_FILE,
};
use engine_core::ptr::{Field, Layout, Ptr};
use engine_core::{args, typed_call, Engine};

/// The map's virtual key copy: `thiscall(map, key, out)`, the key into `*out`.
const COPY_KEY_SLOT: u32 = 0x30;

/// What `00529000` does: the interface's three node slots cleared, in order.
#[inline(always)]
pub(crate) fn release_slots(e: &mut Engine, this: Ptr<MapInterface>) {
    let slot = e.get(this, MapInterface::next_slot);
    e.set(slot, NodeSlot::node, Ptr::NULL);
    let slot = e.get(this, MapInterface::current_slot);
    e.set(slot, NodeSlot::node, Ptr::NULL);
    let slot = e.get(this, MapInterface::previous_slot);
    e.set(slot, NodeSlot::node, Ptr::NULL);
}

// Typed from 0044e550
/// The first live node of bucket `bucket` (`thiscall(bucket, key out, value holder out)`, bool):
/// takes the bucket's head as the current node and publishes it in the current-node slot, until the
/// head read again is the same; with no node answers false. Otherwise copies the node's key through
/// the map's virtual slot +0x30 into `key_out`, assigns its value to `value_out`, and answers true,
/// unless the node was meanwhile marked deleted: then it starts over. The slots are cleared at the
/// end either way.
pub fn first_in_bucket(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key_out: Ptr<u32>,
    value_out: Ptr<Holder>,
) -> bool {
    first_node(e, this, bucket, key_out, |e, node| {
        let value = HashListNode::value_holder(node);
        typed_call!(FAMILY, 0x0092_c820, holder::assign(e, value_out, value));
    })
}

// Typed from 0044ea60
/// [`first_in_bucket`] for a map whose values are plain words: the node's value word goes to
/// `*value_out`.
pub fn first_word_in_bucket(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key_out: Ptr<u32>,
    value_out: Ptr<u32>,
) -> bool {
    first_node(e, this, bucket, key_out, |e, node| {
        let value = e.get(node, HashListNode::value);
        e.set(value_out, KEY_WORD, value);
    })
}

/// The walk of [`first_in_bucket`] and [`first_word_in_bucket`]; `take` stores the current
/// node's value.
#[inline(always)]
fn first_node(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key_out: Ptr<u32>,
    take: impl Fn(&mut Engine, Ptr<HashListNode>),
) -> bool {
    let found;
    loop {
        // A stable head: the one published is the one still there.
        loop {
            let map = e.get(this, MapInterface::map);
            let buckets = e.get(map, LockFreeMap::pBuckets);
            e.set(
                this,
                MapInterface::previous,
                LockFreeMap::bucket(buckets, bucket),
            );
            let link = e.get(this, MapInterface::previous);
            let head = e.get(link, BucketHead::head);
            e.set(this, MapInterface::current, head);
            let current = e.get(this, MapInterface::current).node();
            let slot = e.get(this, MapInterface::current_slot);
            e.set(slot, NodeSlot::node, current);
            let seen = e.get(this, MapInterface::current).node();
            let link = e.get(this, MapInterface::previous);
            if e.get(link, BucketHead::head).word() == seen.addr() {
                break;
            }
        }
        let node = e.get(this, MapInterface::current).node();
        if node.is_null() {
            found = false;
            break;
        }
        let key = e.get(node, HashListNode::key);
        let map = e.get(this, MapInterface::map);
        e.vcall(map.addr(), COPY_KEY_SLOT, &args![key, key_out]);
        let node = e.get(this, MapInterface::current).node();
        take(e, node);
        let node = e.get(this, MapInterface::current).node();
        if !e.get(node, HashListNode::next).is_marked() {
            found = true;
            break;
        }
    }
    release_slots(e, this);
    found
}

/// The compare-and-swap of a link (`cdecl(link, new, expected)`, bool: whether the link held
/// `expected` and now holds `new`).
const SWAP_LINK: u32 = 0x0044_91c0;
/// The map's virtual key compares: `+0x34` whether a node's key is not below the key looked for,
/// `+0x38` whether they are equal (`thiscall(map, node key, key)`).
const KEY_NOT_BELOW_SLOT: u32 = 0x34;
const KEY_EQUAL_SLOT: u32 = 0x38;
/// A holder set from a plain object pointer (`NiPointer<T>::operator=(T*)`, Xbox PDB name of the
/// template; `thiscall(holder, object)`).
const HOLDER_SET: u32 = 0x006f_74f0;
/// A key word where the caller keeps it.
const KEY_WORD: Field<u32, u32> = Field::new(0);

/// Makes the next node the current one, published in the current-node slot (`0052925a`).
#[inline(always)]
fn step_to_next(e: &mut Engine, this: Ptr<MapInterface>) {
    let next = e.get(this, MapInterface::next);
    e.set(this, MapInterface::current, next);
    let next = e.get(this, MapInterface::next).node();
    let slot = e.get(this, MapInterface::current_slot);
    e.set(slot, NodeSlot::node, next);
}

// Typed from 00529030
/// The search of bucket `bucket` for `key` (`thiscall(bucket, key)`; answers the map's equality
/// word, or 0 at the end of the chain). From the bucket's head it walks the chain, keeping the link
/// it came from, the current node and the next one in the interface and publishing them in its node
/// slots, and starts again from the head whenever a link it read has changed. A node whose own link
/// is marked deleted is unlinked (a compare-and-swap of the link it came from) and retired
/// (`005292a0`); a failed swap starts again. It stops at the first live node whose key is not below
/// `key` (the map's slot +0x34), answering whether the keys are equal (slot +0x38). The slots are
/// left as they are: the caller clears them.
pub fn search(e: &mut Engine, this: Ptr<MapInterface>, bucket: u32, key: u32) -> u32 {
    search_with::<0x0052_92a0>(e, this, bucket, key)
}

// Typed from 006662a0
/// [`search`]'s copy the string maps use (their keys compared by the map's slots as ever): the same
/// body, retiring with `00666510`.
pub fn search_string_map(e: &mut Engine, this: Ptr<MapInterface>, bucket: u32, key: u32) -> u32 {
    search_with::<RETIRE_OTHER>(e, this, bucket, key)
}

/// Retires `node` with `RETIRE`: [`retire`] (typed) or a copy's own, by address.
#[inline(always)]
fn retire_by<const RETIRE: u32>(e: &mut Engine, this: Ptr<MapInterface>, node: Ptr<HashListNode>) {
    if RETIRE == 0x0052_92a0 {
        typed_call!(FAMILY, 0x0052_92a0, retire(e, this, node));
    } else {
        e.call(RETIRE, &args![this, node]);
    }
}

/// [`search`]'s body, retiring with `RETIRE`.
#[inline(always)]
fn search_with<const RETIRE: u32>(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
) -> u32 {
    let map = e.get(this, MapInterface::map);
    let buckets = e.get(map, LockFreeMap::pBuckets);
    let head_link = LockFreeMap::bucket(buckets, bucket);
    'start: loop {
        e.set(this, MapInterface::previous, head_link);
        let link = e.get(this, MapInterface::previous);
        let head = e.get(link, BucketHead::head);
        e.set(this, MapInterface::current, head);
        let current = e.get(this, MapInterface::current).node();
        let slot = e.get(this, MapInterface::current_slot);
        e.set(slot, NodeSlot::node, current);
        let seen = e.get(this, MapInterface::current).node();
        let link = e.get(this, MapInterface::previous);
        if e.get(link, BucketHead::head).word() != seen.addr() {
            continue 'start;
        }
        loop {
            let node = e.get(this, MapInterface::current).node();
            if node.is_null() {
                return 0;
            }
            let next = e.get(node, HashListNode::next);
            e.set(this, MapInterface::next, next);
            let next = e.get(this, MapInterface::next).node();
            let slot = e.get(this, MapInterface::next_slot);
            e.set(slot, NodeSlot::node, next);
            let next = e.get(this, MapInterface::next);
            let node = e.get(this, MapInterface::current).node();
            if next != e.get(node, HashListNode::next) {
                continue 'start;
            }
            let node_key = e.get(node, HashListNode::key);
            let seen = e.get(this, MapInterface::current).node();
            let link = e.get(this, MapInterface::previous);
            if e.get(link, BucketHead::head).word() != seen.addr() {
                continue 'start;
            }
            if !e.get(this, MapInterface::next).is_marked() {
                let map = e.get(this, MapInterface::map);
                if e.vcall(map.addr(), KEY_NOT_BELOW_SLOT, &args![node_key, key])
                    .bool()
                {
                    let map = e.get(this, MapInterface::map);
                    return e
                        .vcall(map.addr(), KEY_EQUAL_SLOT, &args![node_key, key])
                        .u32();
                }
                let node = e.get(this, MapInterface::current).node();
                e.set(this, MapInterface::previous, HashListNode::next_link(node));
                let node = e.get(this, MapInterface::current).node();
                let slot = e.get(this, MapInterface::previous_slot);
                e.set(slot, NodeSlot::node, node);
            } else {
                let next = e.get(this, MapInterface::next).node();
                let link = e.get(this, MapInterface::previous);
                if !e.call(SWAP_LINK, &args![link, next, seen]).bool() {
                    continue 'start;
                }
                let node = e.get(this, MapInterface::current).node();
                retire_by::<RETIRE>(e, this, node);
            }
            step_to_next(e, this);
        }
    }
}

// Typed from 005292a0
/// Retires `node`, which this thread unlinked: its value released (`006f74f0` with null), the node
/// put first on the interface's list of nodes waiting to be freed (through its `value` word, as
/// `006ecd40` links it), and counted; when the count reaches the map's `iDeleteBatchSize` (as
/// `0044edb0` reads it), the waiting nodes are scanned (`0044f070`).
pub fn retire(e: &mut Engine, this: Ptr<MapInterface>, node: Ptr<HashListNode>) {
    e.call(HOLDER_SET, &args![HashListNode::value_holder(node), 0u32]);
    let waiting = e.get(this, MapInterface::deleted);
    e.set(node, HashListNode::value, waiting.addr());
    e.set(this, MapInterface::deleted, node);
    let count = e.get(this, MapInterface::deleted_count).wrapping_add(1);
    e.set(this, MapInterface::deleted_count, count);
    let map = e.get(this, MapInterface::map);
    let batch = e.get(map, LockFreeMap::iDeleteBatchSize);
    if e.get(this, MapInterface::deleted_count) == batch {
        typed_call!(FAMILY, 0x0044_f070, scan_deleted_releasing(e, this));
    }
}

// Typed from 0044e680
/// The node after `*key` in bucket `bucket` (`thiscall(bucket, key in and out, value holder out)`,
/// bool): searches for the key (`00529030`); found, the node after it is the answer, otherwise the
/// first node past where it would be. No such node answers false. A node marked deleted starts the
/// search again; so does one marked after its key and value were copied out (the key through the
/// map's slot +0x30 into `*key`, the value assigned to `value_out`), the value then released
/// (`006f74f0` with null) and the search made with the new key. The slots are cleared at the end.
pub fn next_in_bucket(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: Ptr<u32>,
    value_out: Ptr<Holder>,
) -> bool {
    next_node(
        e,
        this,
        key,
        |e, wanted| typed_call!(FAMILY, 0x0052_9030, search(e, this, bucket, wanted)),
        |e, node| {
            let value = HashListNode::value_holder(node);
            typed_call!(FAMILY, 0x0092_c820, holder::assign(e, value_out, value));
        },
        |e| {
            e.call(HOLDER_SET, &args![value_out, 0u32]);
        },
    )
}

// Typed from 0044eb80
/// [`next_in_bucket`] for a map whose values are plain words and whose search is `006662a0`: the
/// node's value word goes to `*value_out`, and a node marked after the copy clears it.
pub fn next_word_in_bucket(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: Ptr<u32>,
    value_out: Ptr<u32>,
) -> bool {
    next_node(
        e,
        this,
        key,
        |e, wanted| {
            typed_call!(
                FAMILY,
                0x0066_62a0,
                search_string_map(e, this, bucket, wanted)
            )
        },
        |e, node| {
            let value = e.get(node, HashListNode::value);
            e.set(value_out, KEY_WORD, value);
        },
        |e| e.set(value_out, KEY_WORD, 0),
    )
}

/// The walk of [`next_in_bucket`] and [`next_word_in_bucket`]: `search` looks the key up, `take`
/// stores the node's value, `let_go` lets it go when the node turns out deleted.
#[inline(always)]
fn next_node(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    key: Ptr<u32>,
    search: impl Fn(&mut Engine, u32) -> u32,
    take: impl Fn(&mut Engine, Ptr<HashListNode>),
    let_go: impl Fn(&mut Engine),
) -> bool {
    let found = loop {
        let wanted = e.get(key, KEY_WORD);
        let hit = search(e, wanted);
        // Found: the node after it; not found: the node it stopped at. The exe tests `AL`.
        let place = if hit & 0xff != 0 {
            MapInterface::next
        } else {
            MapInterface::current
        };
        let node = e.get(this, place).node();
        if node.is_null() {
            break false;
        }
        if e.get(node, HashListNode::next).is_marked() {
            continue;
        }
        let node_key = e.get(node, HashListNode::key);
        let map = e.get(this, MapInterface::map);
        e.vcall(map.addr(), COPY_KEY_SLOT, &args![node_key, key]);
        let node = e.get(this, place).node();
        take(e, node);
        let node = e.get(this, place).node();
        if !e.get(node, HashListNode::next).is_marked() {
            break true;
        }
        let_go(e);
    };
    release_slots(e, this);
    found
}

/// The map's virtual bucket of a key (`thiscall(map, key)`), and its `DecrementCount` (Xbox PDB).
const BUCKET_OF_SLOT: u32 = 0x24;
const DECREMENT_COUNT_SLOT: u32 = 0x40;

// Typed from 00528ce0
/// Removes `key` from bucket `bucket` (`thiscall(bucket, key)`, bool): searches for it
/// (`00529030`); not there answers false. Otherwise marks the found node deleted (a
/// compare-and-swap of its own link to the same pointer with the mark set; a failed swap searches
/// again), then unlinks it (a compare-and-swap of the link it came from to the next node): done, the
/// node is retired (`005292a0`); lost to another thread, a search unlinks it. The map's count goes
/// down (its slot +0x40) and the answer is true. The slots are cleared at the end.
pub fn remove_from_bucket(e: &mut Engine, this: Ptr<MapInterface>, bucket: u32, key: u32) -> bool {
    remove_from_bucket_with::<0x0052_92a0>(e, this, bucket, key)
}

// Typed from 00666080
/// [`remove_from_bucket`]'s copy the string maps use: the
/// same body, searching with their copy of the search (`006662a0`) and retiring with their
/// `00666510`.
pub fn remove_from_bucket_string_map(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
) -> bool {
    remove_from_bucket_with::<RETIRE_OTHER>(e, this, bucket, key)
}

/// The search that goes with retiring by `RETIRE`: [`search`] or [`search_string_map`].
#[inline(always)]
fn search_by<const RETIRE: u32>(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
) -> u32 {
    if RETIRE == 0x0052_92a0 {
        typed_call!(FAMILY, 0x0052_9030, search(e, this, bucket, key))
    } else {
        typed_call!(FAMILY, 0x0066_62a0, search_string_map(e, this, bucket, key))
    }
}

/// [`remove_from_bucket`]'s body, searching and retiring as `RETIRE` says.
#[inline(always)]
fn remove_from_bucket_with<const RETIRE: u32>(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
) -> bool {
    let removed = loop {
        let hit: u32 = search_by::<RETIRE>(e, this, bucket, key);
        if hit & 0xff == 0 {
            break false;
        }
        let next = e.get(this, MapInterface::next).node();
        let marked = next.addr() | 1;
        let current = e.get(this, MapInterface::current).node();
        let link = HashListNode::next_link(current);
        if !e.call(SWAP_LINK, &args![link, marked, next]).bool() {
            continue;
        }
        let current = e.get(this, MapInterface::current).node();
        let previous = e.get(this, MapInterface::previous);
        if e.call(SWAP_LINK, &args![previous, next, current]).bool() {
            let current = e.get(this, MapInterface::current).node();
            retire_by::<RETIRE>(e, this, current);
        } else {
            let _: u32 = search_by::<RETIRE>(e, this, bucket, key);
        }
        let map = e.get(this, MapInterface::map);
        e.vcall(map.addr(), DECREMENT_COUNT_SLOT, &[]);
        break true;
    };
    release_slots(e, this);
    removed
}

// Typed from 00528bb0
/// Removes `key` (`thiscall(key)`, bool): its bucket from the map's slot +0x24, then `00528ce0`.
pub fn remove(e: &mut Engine, this: Ptr<MapInterface>, key: u32) -> bool {
    let map = e.get(this, MapInterface::map);
    let bucket = e.vcall(map.addr(), BUCKET_OF_SLOT, &args![key]).u32();
    typed_call!(
        FAMILY,
        0x0052_8ce0,
        remove_from_bucket(e, this, bucket, key)
    )
}

/// The retirement of an unlinked node in the maps [`find_value`] serves (`thiscall(interface,
/// node)`), as `005292a0` is for the others.
const RETIRE_OTHER: u32 = 0x0066_6510;

// Typed from 0044e820
/// Whether bucket `bucket` holds a live node whose value word is `*value` (`thiscall(bucket,
/// value)`, bool): the walk of [`search`], with the value compared instead of the key (no virtual
/// call), unlinked nodes retired through `00666510`.
pub fn find_value(e: &mut Engine, this: Ptr<MapInterface>, bucket: u32, value: Ptr<u32>) -> bool {
    let map = e.get(this, MapInterface::map);
    let buckets = e.get(map, LockFreeMap::pBuckets);
    let head_link = LockFreeMap::bucket(buckets, bucket);
    'start: loop {
        e.set(this, MapInterface::previous, head_link);
        let link = e.get(this, MapInterface::previous);
        let head = e.get(link, BucketHead::head);
        e.set(this, MapInterface::current, head);
        let current = e.get(this, MapInterface::current).node();
        let slot = e.get(this, MapInterface::current_slot);
        e.set(slot, NodeSlot::node, current);
        let seen = e.get(this, MapInterface::current).node();
        let link = e.get(this, MapInterface::previous);
        if e.get(link, BucketHead::head).word() != seen.addr() {
            continue 'start;
        }
        loop {
            let node = e.get(this, MapInterface::current).node();
            if node.is_null() {
                return false;
            }
            let next = e.get(node, HashListNode::next);
            e.set(this, MapInterface::next, next);
            let next = e.get(this, MapInterface::next).node();
            let slot = e.get(this, MapInterface::next_slot);
            e.set(slot, NodeSlot::node, next);
            let next = e.get(this, MapInterface::next);
            let node = e.get(this, MapInterface::current).node();
            if next != e.get(node, HashListNode::next) {
                continue 'start;
            }
            let node_value = e.get(node, HashListNode::value);
            let seen = e.get(this, MapInterface::current).node();
            let link = e.get(this, MapInterface::previous);
            if e.get(link, BucketHead::head).word() != seen.addr() {
                continue 'start;
            }
            if !e.get(this, MapInterface::next).is_marked() {
                if node_value == e.get(value, KEY_WORD) {
                    return true;
                }
                let node = e.get(this, MapInterface::current).node();
                e.set(this, MapInterface::previous, HashListNode::next_link(node));
                let node = e.get(this, MapInterface::current).node();
                let slot = e.get(this, MapInterface::previous_slot);
                e.set(slot, NodeSlot::node, node);
            } else {
                let next = e.get(this, MapInterface::next).node();
                let link = e.get(this, MapInterface::previous);
                if !e.call(SWAP_LINK, &args![link, next, seen]).bool() {
                    continue 'start;
                }
                let node = e.get(this, MapInterface::current).node();
                e.call(RETIRE_OTHER, &args![this, node]);
            }
            step_to_next(e, this);
        }
    }
}

/// The interface's find of a key in a bucket (`thiscall(bucket, key, value holder out)`, bool).
const FIND_IN_BUCKET: u32 = 0x0044_d510;
/// The interface's insertion into a bucket (`thiscall(bucket, key, value, replace)`, bool).
const INSERT_IN_BUCKET: u32 = 0x0044_d070;

// Typed from 0044c8e0
/// Looks `key` up (`thiscall(key, value holder out)`, the `AL` of `0044d510`): its bucket from the
/// map's slot +0x24, then `0044d510` with the bucket, the key and the holder.
pub fn find(e: &mut Engine, this: Ptr<MapInterface>, key: u32, value_out: Ptr<Holder>) -> u8 {
    let map = e.get(this, MapInterface::map);
    let bucket = e.vcall(map.addr(), BUCKET_OF_SLOT, &args![key]).u32();
    e.call(FIND_IN_BUCKET, &args![this, bucket, key, value_out])
        .u8()
}

// Typed from 0044c000
/// Inserts `key` with `value` (`thiscall(key, value, replace)`, answering the `EAX` of `0044d070`):
/// its bucket from the map's slot +0x24, then `0044d070` with the bucket, the key, the value and the
/// flag byte.
pub fn insert(e: &mut Engine, this: Ptr<MapInterface>, key: u32, value: u32, replace: u8) -> u32 {
    let map = e.get(this, MapInterface::map);
    let bucket = e.vcall(map.addr(), BUCKET_OF_SLOT, &args![key]).u32();
    e.call(
        INSERT_IN_BUCKET,
        &args![this, bucket, key, value, u32::from(replace)],
    )
    .u32()
}

/// The line the insertions name in their memory-context scope.
const INSERT_LINE: u32 = 0x187;
/// The map's virtual key for a new node (`thiscall(map, key)`), and its `IncrementCount` (Xbox PDB).
const NODE_KEY_SLOT: u32 = 0x2c;
const INCREMENT_COUNT_SLOT: u32 = 0x3c;
/// The node constructors (`thiscall(node, key, value)`): with a counted value (`0044df40`) and with
/// a plain word (`006ecbd0`); and the counted node's scalar deleting destructor (`0044ed50`).
const NODE_CONSTRUCT_COUNTED: u32 = 0x0044_df40;
const NODE_CONSTRUCT_PLAIN: u32 = 0x006e_cbd0;
const NODE_DELETE_COUNTED: u32 = 0x0044_ed50;

/// The two kinds of map the insertion serves: counted values, searched by [`search`], and plain
/// words, searched by `006662a0`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Values {
    Counted,
    Plain,
}

/// The insertion of [`insert_in_bucket`] and [`insert_word_in_bucket`]: `value` is the address of
/// the value (a holder, or a word).
#[inline(always)]
fn insert_node(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
    value: u32,
    replace: u8,
    values: Values,
) -> bool {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_BEGIN,
            &args![scope, SCOPE_CONTEXT, SCOPE_MODE, SOURCE_FILE, INSERT_LINE],
        );
        let mut inserted = true;
        let mut node: Ptr<HashListNode> = Ptr::NULL;
        loop {
            let hit = match values {
                Values::Counted => typed_call!(FAMILY, 0x0052_9030, search(e, this, bucket, key)),
                Values::Plain => {
                    typed_call!(FAMILY, 0x0066_62a0, search_string_map(e, this, bucket, key))
                }
            };
            if hit & 0xff != 0 {
                // There already: the node made for it goes, and the value replaces the old one or not.
                match values {
                    Values::Counted => {
                        if !node.is_null() {
                            e.call(NODE_DELETE_COUNTED, &args![node, 1u32]);
                        }
                    }
                    Values::Plain => {
                        e.call(OPERATOR_DELETE, &args![node]);
                    }
                }
                if replace != 0 {
                    let current = e.get(this, MapInterface::current).node();
                    match values {
                        Values::Counted => {
                            let held = HashListNode::value_holder(current);
                            typed_call!(
                                FAMILY,
                                0x0092_c820,
                                holder::assign(e, held, Ptr::new(value))
                            );
                        }
                        Values::Plain => {
                            let word = e.get(Ptr::<u32>::new(value), KEY_WORD);
                            e.set(current, HashListNode::value, word);
                        }
                    }
                } else {
                    inserted = false;
                }
                break;
            }
            if node.is_null() {
                let block = e.call(OPERATOR_NEW, &args![HashListNode::SIZE]).u32();
                if block != 0 {
                    let map = e.get(this, MapInterface::map);
                    let node_key = e.vcall(map.addr(), NODE_KEY_SLOT, &args![key]).u32();
                    let construct = match values {
                        Values::Counted => NODE_CONSTRUCT_COUNTED,
                        Values::Plain => NODE_CONSTRUCT_PLAIN,
                    };
                    node = Ptr::new(e.call(construct, &args![block, node_key, value]).u32());
                }
            }
            // Linked before the current node, then swapped into the link that pointed at it.
            let current = e.get(this, MapInterface::current).node();
            e.set(node, HashListNode::next, Marked(current.addr()));
            let previous = e.get(this, MapInterface::previous);
            if e.call(SWAP_LINK, &args![previous, node, current]).bool() {
                let map = e.get(this, MapInterface::map);
                e.vcall(map.addr(), INCREMENT_COUNT_SLOT, &[]);
                break;
            }
        }
        release_slots(e, this);
        e.call(SCOPE_END, &args![scope]);
        inserted
    })
}

// Typed from 0044d070
/// Inserts `key` with the counted value in the holder `value` into bucket `bucket`
/// (`thiscall(bucket, key, value, replace)`, bool), in a memory-context scope: while the key isn't
/// there (`00529030`), makes the node once (`operator new(0xc)`, the map's key for it from slot
/// +0x2c, `0044df40`) and swaps it in before the node the search stopped at; on success the map
/// counts it (slot +0x3c). A key already there deletes the node made for it (`0044ed50`) and, with
/// `replace`, assigns the value to the node there; without, answers false.
pub fn insert_in_bucket(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
    value: Ptr<Holder>,
    replace: u8,
) -> bool {
    insert_node(e, this, bucket, key, value.addr(), replace, Values::Counted)
}

// Typed from 0044d240
/// [`insert_in_bucket`] for a map of plain words searched by `006662a0`: the node made with
/// `006ecbd0`, freed with `operator delete` when the key is there (null or not), the word at
/// `value` stored over the old one.
pub fn insert_word_in_bucket(
    e: &mut Engine,
    this: Ptr<MapInterface>,
    bucket: u32,
    key: u32,
    value: Ptr<u32>,
    replace: u8,
) -> bool {
    insert_node(e, this, bucket, key, value.addr(), replace, Values::Plain)
}

/// The line the scans name in their scope.
const SCAN_LINE: u32 = 0x362;
/// `memcpy` (`cdecl(to, from, bytes)`).
const MEMCPY: u32 = 0x0040_1460;
/// The map's deletion of a node (`thiscall(map, node)`).
const DELETE_NODE: u32 = 0x0044_dde0;
/// Referenced-node slots a thread.
const SLOTS_PER_THREAD: u32 = 3;

// Typed from 0044f220
/// The scan of the nodes this thread is waiting to free (`thiscall`), in a memory-context scope:
/// takes a copy of all threads' referenced-node slots (three a thread, the thread count read from
/// the interface manager as `0044f3e0` does: `operator new`, `memcpy`), then takes each node off
/// the waiting list: one a slot still names goes back on a new list, counted, and the others are
/// cut loose and deleted (`0044dde0`). The copy is freed, and the new list and its count replace the
/// old. The list links (`00726070`, `006ecd40`) are read and written in place.
pub fn scan_deleted(e: &mut Engine, this: Ptr<MapInterface>) {
    scan_deleted_with(e, this, DELETE_NODE);
}

/// The deletion of a node that first releases what it holds (`thiscall(map, node)`: the map's slot
/// +0x28 on the node's first word, then the node deleted).
const RELEASE_AND_DELETE_NODE: u32 = 0x0044_edd0;

// Typed from 0044f070
/// [`scan_deleted`]'s copy for maps whose nodes release what they hold: the same body, deleting
/// with `0044edd0`.
pub fn scan_deleted_releasing(e: &mut Engine, this: Ptr<MapInterface>) {
    scan_deleted_with(e, this, RELEASE_AND_DELETE_NODE);
}

/// [`scan_deleted`]'s body, deleting a node with `delete`.
#[inline(always)]
fn scan_deleted_with(e: &mut Engine, this: Ptr<MapInterface>, delete: u32) {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_BEGIN,
            &args![scope, SCOPE_CONTEXT, SCOPE_MODE, SOURCE_FILE, SCAN_LINE],
        );
        let map = e.get(this, MapInterface::map);
        let manager = e.get(map, LockFreeMap::pInterfaceManager);
        let threads = e.get(manager, InterfaceManager::max_threads);
        let slots = threads.wrapping_mul(SLOTS_PER_THREAD);
        let copy: Ptr<NodeSlot> = Ptr::new(
            e.call(OPERATOR_NEW, &args![slots.saturating_mul(NodeSlot::SIZE)])
                .u32(),
        );
        let map = e.get(this, MapInterface::map);
        let referenced = e.get(map, LockFreeMap::pReferencedNodes);
        e.call(MEMCPY, &args![copy, referenced, slots << 2]);
        let mut kept: Ptr<HashListNode> = Ptr::NULL;
        let mut kept_count = 0u32;
        while !e.get(this, MapInterface::deleted).is_null() {
            let node = e.get(this, MapInterface::deleted);
            let next = Ptr::new(e.get(node, HashListNode::value));
            e.set(this, MapInterface::deleted, next);
            let mut named = false;
            let mut i = 0u32;
            while i < slots {
                if node == e.get(copy.index(i, NodeSlot::SIZE), NodeSlot::node) {
                    named = true;
                    break;
                }
                i += 1;
            }
            if named {
                e.set(node, HashListNode::value, kept.addr());
                kept = node;
                kept_count = kept_count.wrapping_add(1);
            } else {
                e.set(node, HashListNode::value, 0);
                let map = e.get(this, MapInterface::map);
                e.call(delete, &args![map, node]);
            }
        }
        e.call(OPERATOR_DELETE, &args![copy]);
        e.set(this, MapInterface::deleted, kept);
        e.set(this, MapInterface::deleted_count, kept_count);
        e.call(SCOPE_END, &args![scope]);
    });
}
