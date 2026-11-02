//! `Havok SDK/run_00e3eec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkReferencedObject::addReference` (Xbox PDB, `00c90510`): thiscall, `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference` (Xbox PDB, `00c905b0`): thiscall, `this` only.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

/// The address of heap entry `index` (counted from 1) of 12 bytes at `base`: a reference
/// counted object pointer, then two signed keys.
fn heap_entry(base: u32, index: i32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(12))
        .wrapping_sub(12)
}

/// Takes a reference on the object when the pointer is not null.
fn add_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
}

/// Drops a reference on the object when the pointer is not null.
fn remove_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
}

/// Copies the object pointer of `source` over the one in `target`, taking a reference on the new
/// object and dropping the one on the old (`target` is unchanged when it is `source`).
fn assign_object(e: &mut Engine, target: u32, source: u32) {
    if target == source {
        return;
    }
    let object = e.mem.u32(source);
    add_reference(e, object);
    let old = e.mem.u32(target);
    remove_reference(e, old);
    e.mem.set_u32(target, object);
}

// Translated from 00e3eec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, four words (the last is not read): sifts the entry `index` down a max-heap of `limit`
/// entries at `base` (entries are 12 bytes, counted from 1: an object pointer and two signed
/// keys, ordered by the first key then the second). The entry is held in locals (with a
/// reference taken on its object); while it is smaller than the larger child (the right child
/// when its key is larger, comparing the second key on a tie) that child moves up
/// (`index` follows), and at the end the held entry is stored where it stopped, the reference
/// on its object dropped. The exe also compares the entry's address with the held copy's
/// stack address before the final store; the entry is in the heap array, so that test is never
/// true.
pub fn fn_00e3eec0(e: &mut Engine, base: u32, index: i32, limit: i32, _unused_4: u32) {
    let mut index = index;
    let held = heap_entry(base, index);
    let object = e.mem.u32(held);
    add_reference(e, object);
    let object = e.mem.u32(held);
    let first_key = e.mem.i32(held.wrapping_add(4));
    let second_key = e.mem.i32(held.wrapping_add(8));
    let half = limit / 2;
    while index <= half {
        let mut child = index.wrapping_mul(2);
        if child < limit {
            let left = heap_entry(base, child);
            let right = heap_entry(base, child.wrapping_add(1));
            let left_first = e.mem.i32(left.wrapping_add(4));
            let right_first = e.mem.i32(right.wrapping_add(4));
            let right_is_larger = if left_first < right_first {
                true
            } else if left_first != right_first {
                false
            } else {
                e.mem.i32(left.wrapping_add(8)) < e.mem.i32(right.wrapping_add(8))
            };
            if right_is_larger {
                child = child.wrapping_add(1);
            }
        }
        let child_entry = heap_entry(base, child);
        let child_first = e.mem.i32(child_entry.wrapping_add(4));
        let moves_up = if first_key < child_first {
            true
        } else if first_key != child_first {
            false
        } else {
            second_key < e.mem.i32(child_entry.wrapping_add(8))
        };
        if !moves_up {
            break;
        }
        let target = heap_entry(base, index);
        assign_object(e, target, child_entry);
        let value = e.mem.u32(child_entry.wrapping_add(4));
        e.mem.set_u32(target.wrapping_add(4), value);
        let value = e.mem.u32(child_entry.wrapping_add(8));
        e.mem.set_u32(target.wrapping_add(8), value);
        index = child;
    }
    let target = heap_entry(base, index);
    add_reference(e, object);
    let old = e.mem.u32(target);
    remove_reference(e, old);
    e.mem.set_u32(target, object);
    e.mem.set_i32(target.wrapping_add(4), first_key);
    e.mem.set_i32(target.wrapping_add(8), second_key);
    remove_reference(e, object);
}

// Translated from 00e3eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, three words: heap sort of the `count` entries at `base` (the layout of `00e3eec0`,
/// which gets `extra` as its unread fourth word). The heap is built by sifting down from
/// `count / 2` to 1; then the first entry (held, with a reference on its object) is swapped with
/// the last of the remaining entries, the count drops by one and entry 1 is sifted down again,
/// until nothing remains. The comparison of an entry's address with the held copy's stack address
/// in the swap is never true (the entries are in the heap array).
pub fn fn_00e3eff0(e: &mut Engine, base: u32, count: i32, extra: u32) {
    let mut index = count / 2;
    while index > 0 {
        fn_00e3eec0(e, base, index, count, extra);
        index -= 1;
    }
    if count < 1 {
        return;
    }
    let mut remaining = count;
    let mut last = heap_entry(base, remaining);
    loop {
        let root_object = e.mem.u32(base);
        add_reference(e, root_object);
        let first_key = e.mem.u32(base.wrapping_add(4));
        let second_key = e.mem.u32(base.wrapping_add(8));
        let object = e.mem.u32(base);
        assign_object(e, base, last);
        let value = e.mem.u32(last.wrapping_add(4));
        e.mem.set_u32(base.wrapping_add(4), value);
        let value = e.mem.u32(last.wrapping_add(8));
        e.mem.set_u32(base.wrapping_add(8), value);
        add_reference(e, object);
        let old = e.mem.u32(last);
        remove_reference(e, old);
        e.mem.set_u32(last, object);
        remaining -= 1;
        e.mem.set_u32(last.wrapping_add(4), first_key);
        e.mem.set_u32(last.wrapping_add(8), second_key);
        last = last.wrapping_sub(12);
        fn_00e3eec0(e, base, 1, remaining, extra);
        remove_reference(e, object);
        if remaining < 1 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_eec0, fn_00e3eec0(u32, i32, i32, u32)),
        entry!(0x00e3_eff0, fn_00e3eff0(u32, i32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    /// The reference count of each object the doubles saw, starting from 1.
    type Counts = Rc<RefCell<HashMap<u32, i32>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Doubles for the reference functions that count per object.
    fn count_references(e: &mut Engine) -> Counts {
        let counts: Counts = Rc::new(RefCell::new(HashMap::new()));
        for (address, delta) in [(ADD_REFERENCE, 1), (REMOVE_REFERENCE, -1)] {
            let log = Rc::clone(&counts);
            e.register_double(address, move |_, a| {
                *log.borrow_mut().entry(a[0]).or_insert(1) += delta;
                Ret::default()
            });
        }
        counts
    }

    /// An array of `(object, first key, second key)` entries.
    fn heap(e: &mut Engine, entries: &[(u32, i32, i32)]) -> u32 {
        let base = e.mem.alloc(12 * entries.len() as u32);
        for (index, (object, first, second)) in entries.iter().enumerate() {
            let at = base + 12 * index as u32;
            e.mem.set_u32(at, *object);
            e.mem.set_i32(at + 4, *first);
            e.mem.set_i32(at + 8, *second);
        }
        base
    }

    fn read(e: &Engine, base: u32, count: u32) -> Vec<(u32, i32, i32)> {
        (0..count)
            .map(|index| {
                let at = base + 12 * index;
                (e.mem.u32(at), e.mem.i32(at + 4), e.mem.i32(at + 8))
            })
            .collect()
    }

    #[test]
    fn sift_down_moves_the_larger_child_up() {
        let mut e = engine();
        let counts = count_references(&mut e);
        let base = heap(&mut e, &[(0x11, 1, 0), (0x22, 5, 0), (0x33, 3, 0)]);
        e.call(0x00e3_eec0, &args![base, 1i32, 3i32, 0u32]);
        assert_eq!(
            read(&e, base, 3),
            vec![(0x22, 5, 0), (0x11, 1, 0), (0x33, 3, 0)]
        );
        // The moved object has its count back to 1 after taking and dropping references.
        assert!(counts.borrow().values().all(|count| *count == 1));
    }

    #[test]
    fn sift_down_takes_the_right_child_on_a_second_key_tie() {
        let mut e = engine();
        count_references(&mut e);
        let base = heap(&mut e, &[(0x11, 1, 0), (0x22, 5, 1), (0x33, 5, 2)]);
        e.call(0x00e3_eec0, &args![base, 1i32, 3i32, 0u32]);
        assert_eq!(
            read(&e, base, 3),
            vec![(0x33, 5, 2), (0x22, 5, 1), (0x11, 1, 0)]
        );
    }

    #[test]
    fn sift_down_stops_when_the_entry_is_not_smaller() {
        let mut e = engine();
        count_references(&mut e);
        let entries = [(0x11, 9, 0), (0x22, 5, 0), (0x33, 3, 0)];
        let base = heap(&mut e, &entries);
        e.call(0x00e3_eec0, &args![base, 1i32, 3i32, 0u32]);
        assert_eq!(read(&e, base, 3), entries.to_vec());
        // Equal first keys: the second key decides, an equal entry stays.
        let tie = [(0x11, 5, 7), (0x22, 5, 7), (0x33, 3, 0)];
        let base = heap(&mut e, &tie);
        e.call(0x00e3_eec0, &args![base, 1i32, 3i32, 0u32]);
        assert_eq!(read(&e, base, 3), tie.to_vec());
    }

    #[test]
    fn sift_down_from_a_leaf_only_rewrites_the_entry() {
        let mut e = engine();
        let counts = count_references(&mut e);
        let entries = [(0x11, 1, 0), (0x22, 5, 0), (0x33, 3, 0)];
        let base = heap(&mut e, &entries);
        e.call(0x00e3_eec0, &args![base, 3i32, 3i32, 0u32]);
        assert_eq!(read(&e, base, 3), entries.to_vec());
        assert!(counts.borrow().values().all(|count| *count == 1));
    }

    #[test]
    fn heap_sort_orders_by_both_keys_and_keeps_the_reference_counts() {
        let mut e = engine();
        let counts = count_references(&mut e);
        let base = heap(
            &mut e,
            &[
                (0x11, 3, 0),
                (0x22, 1, 5),
                (0x33, 2, 2),
                (0x44, 1, 1),
                (0x55, 5, 0),
                (0, 4, 4),
            ],
        );
        e.call(0x00e3_eff0, &args![base, 6i32, 0u32]);
        assert_eq!(
            read(&e, base, 6),
            vec![
                (0x44, 1, 1),
                (0x22, 1, 5),
                (0x33, 2, 2),
                (0x11, 3, 0),
                (0, 4, 4),
                (0x55, 5, 0)
            ]
        );
        assert!(counts.borrow().values().all(|count| *count == 1));
    }

    #[test]
    fn heap_sort_of_nothing_or_one_entry_changes_nothing() {
        let mut e = engine();
        count_references(&mut e);
        let base = heap(&mut e, &[(0x11, 3, 0)]);
        e.call(0x00e3_eff0, &args![base, 0i32, 0u32]);
        assert_eq!(read(&e, base, 1), vec![(0x11, 3, 0)]);
        e.call(0x00e3_eff0, &args![base, 1i32, 0u32]);
        assert_eq!(read(&e, base, 1), vec![(0x11, 3, 0)]);
    }
}
