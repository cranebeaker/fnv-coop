//! `Havok SDK/hkptypedbroadphasedispatcher.cpp` (Xbox PDB source unit), subsystem
//! `Havok SDK`: its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The dispatcher keeps a table of 8 x 8 sub-dispatchers (word at `this + 4 * (type of the
//! second * 1 + type of the first * 8)`), the type being the signed byte at +4 of each
//! broad-phase handle. A pair is two handle pointers (8 bytes). An `hkArray` of pairs is
//! `{data, size, capacityAndFlags}` (the capacity is the low 30 bits of the third word).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`, no arguments): word +0xC is the "stack" allocator used for
/// the temporary hash map's storage, word +0x10 the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The number of map entries needed for a given count of pairs (`00cb1810`, cdecl `(count)`).
const MAP_CAPACITY_FOR: u32 = 0x00cb_1810;
/// The temporary `hkMap<u64, u64>` (12 bytes: elements pointer, element count, hash mask):
/// construct over a buffer (`00cb1b80`, `this`, `(buffer, capacity)`), `findKey` (`00cb1c60`,
/// `this`, `(key low, key high)`, returns the slot index, `hash mask + 1` when absent),
/// `insert` (`00cb1bc0`, `this`, `(allocator, key low, key high, value low, value high)`),
/// remove by index (`00cb1d20`, `this`, `(index)`), `clearAndDeallocate` (`00cb1e40`, `this`,
/// `(allocator)`) and its destructor (`00a29680`, `this`). A slot is 16 bytes: the key at +0,
/// the 64-bit value at +8.
const MAP_CONSTRUCT: u32 = 0x00cb_1b80;
const MAP_FIND_KEY: u32 = 0x00cb_1c60;
const MAP_INSERT: u32 = 0x00cb_1bc0;
const MAP_REMOVE: u32 = 0x00cb_1d20;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1e40;
const MAP_DESTRUCT: u32 = 0x00a2_9680;
/// The mask of the capacity bits in an `hkArray`'s third word, and the "not owned" size flag
/// the stack allocation keeps when the allocator granted nothing.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NO_ALLOCATION: u32 = 0x8000_0000;

/// The inlined `hkArray::reserve(needed)` of a pair array: when the capacity is below
/// `needed`, `_reserve(allocator, array, max(needed, 2 * capacity), 8)`. The memory router
/// is looked up first in every case.
fn reserve_pairs(e: &mut Engine, array: u32, needed: i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < needed {
        let doubled = capacity.wrapping_add(capacity);
        let count = if needed < doubled { doubled } else { needed };
        e.call(ARRAY_RESERVE, &args![allocator, array, count, 8u32]);
    }
}

/// The sub-dispatcher for a pair: `this[(type of second) + 8 * (type of first)]`.
fn sub_dispatcher(e: &Engine, this: u32, pair: u32) -> u32 {
    let first = e.mem.u32(pair);
    let second = e.mem.u32(pair + 4);
    let first_type = i32::from(e.mem.i8(first + 4));
    let second_type = i32::from(e.mem.i8(second + 4));
    let index = second_type.wrapping_add(first_type.wrapping_mul(8)) as u32;
    e.mem.u32(this.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00cf7080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTypedBroadPhaseDispatcher::addPairs` (Xbox PDB): for each of `count` pairs, asks the
/// filter (`filter` slot 4 with `(out hkBool, first + signed byte at first+5, second +
/// signed byte at second+5)`) and, when it answers true, adds the pair to its
/// sub-dispatcher (slot 4 with the pair).
pub fn hkp_typed_broad_phase_dispatcher_add_pairs(
    e: &mut Engine,
    this: Ptr,
    pairs: Ptr,
    count: i32,
    filter: Ptr,
) {
    e.with_stack(4, |e, out| {
        let mut pair = pairs.addr();
        for _ in 0..count.max(0) {
            let first = e.mem.u32(pair);
            let second = e.mem.u32(pair + 4);
            let first_owner = first.wrapping_add(i32::from(e.mem.i8(first + 5)) as u32);
            let second_owner = second.wrapping_add(i32::from(e.mem.i8(second + 5)) as u32);
            let answer = e
                .vcall(filter.addr(), 4, &args![out, first_owner, second_owner])
                .u32();
            if e.mem.u8(answer) != 0 {
                let dispatcher = sub_dispatcher(e, this.addr(), pair);
                e.vcall(dispatcher, 4, &args![pair]);
            }
            pair = pair.wrapping_add(8);
        }
    });
}

// Translated from 00cf70f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTypedBroadPhaseDispatcher::removePairs` (Xbox PDB): for each of `count` pairs, calls
/// slot 8 of its sub-dispatcher with the pair.
pub fn hkp_typed_broad_phase_dispatcher_remove_pairs(
    e: &mut Engine,
    this: Ptr,
    pairs: Ptr,
    count: i32,
) {
    let mut pair = pairs.addr();
    for _ in 0..count.max(0) {
        let dispatcher = sub_dispatcher(e, this.addr(), pair);
        e.vcall(dispatcher, 8, &args![pair]);
        pair = pair.wrapping_add(8);
    }
}

/// Whether the pair `(first, second)` of list `list` at `index` equals `(a, b)` in either
/// order.
fn pair_matches(e: &Engine, list_data: u32, index: u32, a: u32, b: u32) -> bool {
    let at = list_data.wrapping_add(8 * index);
    let (x, y) = (e.mem.u32(at), e.mem.u32(at + 4));
    (x == a && y == b) || (y == a && x == b)
}

/// Copies the pair at `from` onto the pair at `to` in `data` (two words).
fn copy_pair(e: &mut Engine, data: u32, to: u32, from: u32) {
    let first = e.mem.u32(data.wrapping_add(8 * from));
    let second = e.mem.u32(data.wrapping_add(8 * from + 4));
    e.mem.set_u32(data.wrapping_add(8 * to), first);
    e.mem.set_u32(data.wrapping_add(8 * to + 4), second);
}

/// The small-list path of `removeDuplicates`: every pair of `second` that also occurs
/// (either order) in `first` is dropped from `second`, and its first match is removed from
/// `first` (the entries behind it moved down).
fn remove_duplicates_small(e: &mut Engine, first: u32, second: u32) {
    let mut kept = 0i32;
    let mut index = 0i32;
    if e.mem.i32(second + 4) > 0 {
        loop {
            let mut found_at = 0i32;
            let first_count = e.mem.i32(first + 4);
            if first_count > 0 {
                let data = e.mem.u32(second);
                let a = e.mem.u32(data.wrapping_add(8 * index as u32));
                let b = e.mem.u32(data.wrapping_add(8 * index as u32 + 4));
                let first_data = e.mem.u32(first);
                while found_at < first_count {
                    if pair_matches(e, first_data, found_at as u32, a, b) {
                        break;
                    }
                    found_at += 1;
                }
            }
            let first_count = e.mem.i32(first + 4);
            if found_at == first_count {
                if kept != index {
                    let data = e.mem.u32(second);
                    copy_pair(e, data, kept as u32, index as u32);
                }
                kept += 1;
            } else {
                let remaining = first_count - 1;
                e.mem.set_i32(first + 4, remaining);
                let data = e.mem.u32(first);
                let bytes = remaining.wrapping_sub(found_at).wrapping_mul(8);
                let mut to = data.wrapping_add(8 * found_at as u32);
                if bytes > 0 {
                    let mut words = ((bytes - 1) as u32 >> 2) + 1;
                    while words != 0 {
                        let word = e.mem.u32(to + 8);
                        e.mem.set_u32(to, word);
                        to += 4;
                        words -= 1;
                    }
                }
            }
            index += 1;
            if index >= e.mem.i32(second + 4) {
                break;
            }
        }
    }
    reserve_pairs(e, second, kept);
    e.mem.set_i32(second + 4, kept);
}

/// The large-list path of `removeDuplicates`: the same result computed through a temporary
/// hash map from the (ordered) pair to `(index in first << 8) | occurrence count`.
fn remove_duplicates_large(e: &mut Engine, first: u32, second: u32) {
    let capacity = e.call(MAP_CAPACITY_FOR, &args![e.mem.i32(first + 4)]).i32();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let stack_allocator = e.mem.u32(router + 0xc);
    let (block, size_field) = if capacity == 0 {
        (0, NO_ALLOCATION)
    } else {
        e.with_stack(4, |e, size| {
            e.mem.set_i32(size.addr(), capacity);
            let block = e.vcall(stack_allocator, 0xc, &args![size]).u32();
            let granted = e.mem.u32(size.addr());
            (block, if granted == 0 { NO_ALLOCATION } else { granted })
        })
    };
    e.with_stack(0x10, |e, map| {
        let map = map.addr();
        e.call(MAP_CONSTRUCT, &args![map, block, capacity]);
        // First pass: register every pair of `first` (ordered key); a repeated key counts up
        // and the repeat is blanked out of `first`.
        let mut index = 0i32;
        if e.mem.i32(first + 4) > 0 {
            loop {
                let data = e.mem.u32(first);
                let x = e.mem.u32(data.wrapping_add(8 * index as u32));
                let y = e.mem.u32(data.wrapping_add(8 * index as u32 + 4));
                let (low, high) = if x > y { (y, x) } else { (x, y) };
                let slot = e.call(MAP_FIND_KEY, &args![map, low, high]).i32();
                if slot > e.mem.i32(map + 8) {
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let allocator = e.mem.u32(router + 0x10);
                    let value = ((index as u32) << 8) | 1;
                    let value_high = ((value as i32) >> 31) as u32;
                    e.call(
                        MAP_INSERT,
                        &args![map, allocator, low, high, value, value_high],
                    );
                } else {
                    let value_at = e.mem.u32(map) + (slot as u32).wrapping_mul(16) + 8;
                    let value = e.mem.u64(value_at).wrapping_add(1);
                    e.mem.set_u64(value_at, value);
                    let data = e.mem.u32(first);
                    e.mem.set_u32(data.wrapping_add(8 * index as u32), 0);
                }
                index += 1;
                if index >= e.mem.i32(first + 4) {
                    break;
                }
            }
        }
        // Second pass: a pair of `second` that is in the map is dropped; it uses up one
        // occurrence, and the last one removes the key and blanks its entry of `first`.
        let mut kept = 0i32;
        let mut index = 0i32;
        if e.mem.i32(second + 4) > 0 {
            loop {
                let data = e.mem.u32(second);
                let x = e.mem.u32(data.wrapping_add(8 * index as u32));
                let y = e.mem.u32(data.wrapping_add(8 * index as u32 + 4));
                let (low, high) = if x > y { (y, x) } else { (x, y) };
                let slot = e.call(MAP_FIND_KEY, &args![map, low, high]).i32();
                if slot > e.mem.i32(map + 8) {
                    copy_pair(e, data, kept as u32, index as u32);
                    kept += 1;
                } else {
                    let value_at = e.mem.u32(map) + (slot as u32).wrapping_mul(16) + 8;
                    let value = e.mem.u64(value_at);
                    if (value & 0xff) as i32 > 1 {
                        e.mem.set_u64(value_at, value.wrapping_sub(1));
                    } else {
                        e.call(MAP_REMOVE, &args![map, slot]);
                        let first_data = e.mem.u32(first);
                        let owner = (value as u32) >> 8;
                        e.mem.set_u32(first_data.wrapping_add(8 * owner), 0);
                    }
                }
                index += 1;
                if index >= e.mem.i32(second + 4) {
                    break;
                }
            }
        }
        reserve_pairs(e, second, kept);
        e.mem.set_i32(second + 4, kept);
        // Compact `first`, dropping the blanked entries.
        let mut kept = 0i32;
        let mut index = 0i32;
        while index < e.mem.i32(first + 4) {
            let data = e.mem.u32(first);
            if e.mem.u32(data.wrapping_add(8 * index as u32)) != 0 {
                copy_pair(e, data, kept as u32, index as u32);
                kept += 1;
            }
            index += 1;
        }
        reserve_pairs(e, first, kept);
        e.mem.set_i32(first + 4, kept);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(MAP_DESTRUCT, &args![map]);
    });
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let stack_allocator = e.mem.u32(router + 0xc);
    if (size_field as i32) >= 0 {
        e.vcall(
            stack_allocator,
            0x10,
            &args![block, size_field & CAPACITY_MASK],
        );
    }
}

// Translated from 00cf7130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTypedBroadPhaseDispatcher::removeDuplicates` (Xbox PDB, cdecl): removes the pairs
/// that occur in both `first` and `second` (a pair equals another with its handles swapped)
/// from both lists. When the shorter list has fewer than 32 pairs it compares directly
/// (each pair of `second` against `first`); otherwise it uses a temporary hash map from
/// the ordered pair to `(index in first << 8) | count`, taking its storage from the stack
/// allocator (`router + 0xC`): pairs repeated inside `first` are blanked out and counted,
/// matching pairs of `second` use the counts up, and the lists are compacted.
pub fn hkp_typed_broad_phase_dispatcher_remove_duplicates(e: &mut Engine, first: Ptr, second: Ptr) {
    let first_count = e.mem.i32(first.addr() + 4);
    let second_count = e.mem.i32(second.addr() + 4);
    let shorter = if first_count < second_count {
        first_count
    } else {
        second_count
    };
    if shorter < 0x20 {
        remove_duplicates_small(e, first.addr(), second.addr());
    } else {
        remove_duplicates_large(e, first.addr(), second.addr());
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf_7080,
            hkp_typed_broad_phase_dispatcher_add_pairs(Ptr, Ptr, i32, Ptr)
        ),
        entry!(
            0x00cf_70f0,
            hkp_typed_broad_phase_dispatcher_remove_pairs(Ptr, Ptr, i32)
        ),
        entry!(
            0x00cf_7130,
            hkp_typed_broad_phase_dispatcher_remove_duplicates(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Pairs = Vec<(u32, u32)>;
    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;
    type MapEntries = Rc<RefCell<Vec<Option<(u32, u32)>>>>;

    const ALLOCATOR_VTABLE: u32 = 0x0a00_0000;
    const ALLOCATE_SLOT: u32 = 0x0a10_0001;
    const FREE_SLOT: u32 = 0x0a10_0002;

    /// An `hkArray` of pairs with owned storage for `capacity` pairs.
    fn list(e: &mut Engine, pairs: &[(u32, u32)], capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(8 * capacity.max(pairs.len() as u32).max(1));
        for (i, (a, b)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *a);
            e.mem.set_u32(data + 8 * i as u32 + 4, *b);
        }
        e.mem.set_u32(array, data);
        e.mem.set_i32(array + 4, pairs.len() as i32);
        e.mem.set_u32(array + 8, capacity | 0x8000_0000);
        array
    }

    fn contents(e: &Engine, array: u32) -> Pairs {
        let data = e.mem.u32(array);
        (0..e.mem.i32(array + 4) as u32)
            .map(|i| (e.mem.u32(data + 8 * i), e.mem.u32(data + 8 * i + 4)))
            .collect()
    }

    /// Router and allocator doubles; returns the logs of `_reserve` and of the frees of
    /// the stack allocator, and that allocator.
    fn rig(e: &mut Engine) -> (CallLog, CallLog, u32) {
        let mut slots = vec![0u32; 8];
        slots[0xc / 4] = ALLOCATE_SLOT;
        slots[0x10 / 4] = FREE_SLOT;
        e.put_vtable(ALLOCATOR_VTABLE, &slots);
        let stack_allocator = e.mem.alloc(8);
        e.mem.set_u32(stack_allocator, ALLOCATOR_VTABLE);
        let heap_allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, stack_allocator);
        e.mem.set_u32(router + 0x10, heap_allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let reserves: CallLog = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&reserves);
        e.register_double(ARRAY_RESERVE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let frees: CallLog = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&frees);
        e.register_double(FREE_SLOT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // the stack allocator hands out 16 bytes per requested map entry
        e.register_double(ALLOCATE_SLOT, |e, a| {
            let entries = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(16 * entries),
                ..Ret::default()
            }
        });
        (reserves, frees, stack_allocator)
    }

    #[test]
    fn small_lists_drop_the_pairs_found_in_both() {
        let mut e = Engine::new();
        let (reserves, _, _) = rig(&mut e);
        let first = list(&mut e, &[(1, 2), (3, 4), (5, 6)], 8);
        let second = list(&mut e, &[(4, 3), (7, 8), (1, 9), (2, 1)], 8);
        e.call(0x00cf_7130, &args![first, second]);
        assert_eq!(contents(&e, first), vec![(5, 6)]);
        assert_eq!(contents(&e, second), vec![(7, 8), (1, 9)]);
        assert!(reserves.borrow().is_empty());
    }

    #[test]
    fn small_lists_grow_the_second_list_only_when_its_capacity_is_short() {
        let mut e = Engine::new();
        let (reserves, _, _) = rig(&mut e);
        let first = list(&mut e, &[(1, 2)], 8);
        let second = list(&mut e, &[(7, 8), (9, 10), (11, 12)], 2);
        e.call(0x00cf_7130, &args![first, second]);
        // capacity 2, three kept: max(3, 2 * 2) = 4
        let reserves = reserves.borrow();
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][1..], [second, 4, 8]);
        assert_eq!(contents(&e, second).len(), 3);
        assert_eq!(contents(&e, first), vec![(1, 2)]);
    }

    /// Doubles for the temporary hash map: a linear table over the block the map was built
    /// over (slot = 16 bytes, key at +0, value at +8). Returns the live-entries state and
    /// the log of the map calls made at the end.
    fn map_doubles(e: &mut Engine) -> (MapEntries, CallLog) {
        e.register(MAP_CAPACITY_FOR, |_, a| Ret {
            eax: a[0] * 2,
            ..Ret::default()
        });
        let entries: MapEntries = Rc::new(RefCell::new(Vec::new()));
        let state = Rc::clone(&entries);
        e.register_double(MAP_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, a[2] - 1);
            state.borrow_mut().clear();
            Ret::default()
        });
        let state = Rc::clone(&entries);
        e.register_double(MAP_FIND_KEY, move |e, a| {
            let hash_mask = e.mem.u32(a[0] + 8);
            let found = state
                .borrow()
                .iter()
                .position(|slot| *slot == Some((a[1], a[2])));
            Ret {
                eax: found.map_or(hash_mask + 1, |i| i as u32),
                ..Ret::default()
            }
        });
        let state = Rc::clone(&entries);
        e.register_double(MAP_INSERT, move |e, a| {
            let slot = state.borrow().len() as u32;
            let elements = e.mem.u32(a[0]);
            e.mem.set_u32(elements + 16 * slot, a[2]);
            e.mem.set_u32(elements + 16 * slot + 4, a[3]);
            e.mem.set_u32(elements + 16 * slot + 8, a[4]);
            e.mem.set_u32(elements + 16 * slot + 12, a[5]);
            state.borrow_mut().push(Some((a[2], a[3])));
            Ret::default()
        });
        let state = Rc::clone(&entries);
        e.register_double(MAP_REMOVE, move |_, a| {
            state.borrow_mut()[a[1] as usize] = None;
            Ret::default()
        });
        let ends: CallLog = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&ends);
        e.register_double(MAP_CLEAR_AND_DEALLOCATE, move |_, a| {
            log.borrow_mut().push(vec![1, a[0], a[1]]);
            Ret::default()
        });
        let log = Rc::clone(&ends);
        e.register_double(MAP_DESTRUCT, move |_, a| {
            log.borrow_mut().push(vec![2, a[0]]);
            Ret::default()
        });
        (entries, ends)
    }

    #[test]
    fn large_lists_go_through_the_hash_map() {
        let mut e = Engine::new();
        let (reserves, frees, stack_allocator) = rig(&mut e);
        let (entries, ends) = map_doubles(&mut e);
        let mut a_pairs: Pairs = (0..40).map(|i| (100 + i, 200 + i)).collect();
        a_pairs.push((200, 100));
        let first = list(&mut e, &a_pairs, 64);
        let mut b_pairs: Pairs = (0..10).map(|j| (200 + j, 100 + j)).collect();
        b_pairs.extend((10..36).map(|j| (300 + j, 400 + j)));
        let second = list(&mut e, &b_pairs, 64);
        e.call(0x00cf_7130, &args![first, second]);

        // the repeated pair of `first` is blanked and counted, so the first match of
        // `second` only uses a count up; the others remove their entries from `first`
        let mut want_first: Pairs = vec![(100, 200)];
        want_first.extend((10..40).map(|i| (100 + i, 200 + i)));
        assert_eq!(contents(&e, first), want_first);
        let want_second: Pairs = (10..36).map(|j| (300 + j, 400 + j)).collect();
        assert_eq!(contents(&e, second), want_second);
        assert!(reserves.borrow().is_empty());

        // 40 distinct keys were inserted, 9 of them removed again
        let live = entries.borrow().iter().flatten().count();
        assert_eq!(live, 40 - 9);
        // the map was cleared and destructed, then the stack block was returned
        let ends = ends.borrow();
        assert_eq!(ends.len(), 2);
        assert_eq!(ends[0][0], 1);
        assert_eq!(ends[1][0], 2);
        assert_eq!(ends[0][1], ends[1][1]);
        let frees = frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][0], stack_allocator);
        assert_eq!(frees[0][2], 82);
    }

    /// Two handles of types 1 and 2 (owner offsets +0x10 and 0) and a dispatcher table with
    /// one sub-dispatcher per ordered pair of types.
    struct Handles {
        this: u32,
        first: u32,
        second: u32,
        dispatchers: [u32; 2],
        log: CallLog,
    }

    fn handles(e: &mut Engine) -> Handles {
        const SUB_VTABLE: u32 = 0x0a00_1000;
        const SUB_ADD: u32 = 0x0a10_0011;
        const SUB_REMOVE: u32 = 0x0a10_0012;
        let mut slots = vec![0u32; 4];
        slots[1] = SUB_ADD;
        slots[2] = SUB_REMOVE;
        e.put_vtable(SUB_VTABLE, &slots);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let add_log = Rc::clone(&log);
        e.register_double(SUB_ADD, move |_, a| {
            add_log.borrow_mut().push(vec![1, a[0], a[1]]);
            Ret::default()
        });
        let remove_log = Rc::clone(&log);
        e.register_double(SUB_REMOVE, move |_, a| {
            remove_log.borrow_mut().push(vec![2, a[0], a[1]]);
            Ret::default()
        });
        let first = e.mem.alloc(0x20);
        e.mem.set_i8(first + 4, 1);
        e.mem.set_i8(first + 5, 0x10);
        let second = e.mem.alloc(0x20);
        e.mem.set_i8(second + 4, 2);
        e.mem.set_i8(second + 5, 0);
        let this = e.mem.alloc(4 * 64);
        let one_two = e.mem.alloc(8);
        e.mem.set_u32(one_two, SUB_VTABLE);
        let two_one = e.mem.alloc(8);
        e.mem.set_u32(two_one, SUB_VTABLE);
        // index = type of second + 8 * type of first
        e.mem.set_u32(this + 4 * (2 + 8), one_two);
        e.mem.set_u32(this + 4 * (1 + 16), two_one);
        Handles {
            this,
            first,
            second,
            dispatchers: [one_two, two_one],
            log,
        }
    }

    fn pair_block(e: &mut Engine, pairs: &[(u32, u32)]) -> u32 {
        let block = e.mem.alloc(8 * pairs.len() as u32);
        for (i, (a, b)) in pairs.iter().enumerate() {
            e.mem.set_u32(block + 8 * i as u32, *a);
            e.mem.set_u32(block + 8 * i as u32 + 4, *b);
        }
        block
    }

    #[test]
    fn remove_pairs_calls_each_pair_sub_dispatcher() {
        let mut e = Engine::new();
        let h = handles(&mut e);
        let pairs = pair_block(&mut e, &[(h.first, h.second), (h.second, h.first)]);
        e.call(0x00cf_70f0, &args![h.this, pairs, 2i32]);
        let want = vec![
            vec![2, h.dispatchers[0], pairs],
            vec![2, h.dispatchers[1], pairs + 8],
        ];
        assert_eq!(*h.log.borrow(), want);
        h.log.borrow_mut().clear();
        e.call(0x00cf_70f0, &args![h.this, pairs, 0i32]);
        e.call(0x00cf_70f0, &args![h.this, pairs, -3i32]);
        assert!(h.log.borrow().is_empty());
    }

    #[test]
    fn add_pairs_adds_only_the_pairs_the_filter_accepts() {
        let mut e = Engine::new();
        let h = handles(&mut e);
        const FILTER_VTABLE: u32 = 0x0a00_2000;
        const FILTER_SLOT: u32 = 0x0a10_0021;
        let mut slots = vec![0u32; 2];
        slots[1] = FILTER_SLOT;
        e.put_vtable(FILTER_VTABLE, &slots);
        let filter = e.mem.alloc(8);
        e.mem.set_u32(filter, FILTER_VTABLE);
        let asked = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&asked);
        let first_owner = h.first + 0x10;
        e.register_double(FILTER_SLOT, move |e, a| {
            log.borrow_mut().push((a[2], a[3]));
            // accept the pair only when the owners come in the order (first, second)
            e.mem.set_u8(a[1], u8::from(a[2] == first_owner));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let pairs = pair_block(&mut e, &[(h.first, h.second), (h.second, h.first)]);
        e.call(0x00cf_7080, &args![h.this, pairs, 2i32, filter]);
        assert_eq!(
            *asked.borrow(),
            vec![(first_owner, h.second), (h.second, first_owner)]
        );
        assert_eq!(*h.log.borrow(), vec![vec![1, h.dispatchers[0], pairs]]);
    }
}
