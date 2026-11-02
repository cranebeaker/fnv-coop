//! `Havok SDK/run_00d9ec10` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d9ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_b0d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d9ee20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_b0d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d9ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, one word (`RET 4`): swaps the three words at
/// `this` with the three words at `other`.
pub fn fn_00d9ec10(e: &mut Engine, this: Ptr, other: Ptr) {
    for offset in [0u32, 4, 8] {
        let mine = e.mem.u32(this.addr() + offset);
        let theirs = e.mem.u32(other.addr() + offset);
        e.mem.set_u32(this.addr() + offset, theirs);
        e.mem.set_u32(other.addr() + offset, mine);
    }
}

// Translated from 00d9ee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d9ee20`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d9ee10(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d9_ee20, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The sort key of an entry's object `q` (the first word of a 16-byte element): the word at
/// `+0xd4` of the objects `[q + 0x58]` (`first`) and `[q + 0x5c]` (`second`).
fn key_words(e: &Engine, object: u32) -> (u32, u32) {
    let first = e.mem.u32(e.mem.u32(object + 0x58) + 0xd4);
    let second = e.mem.u32(e.mem.u32(object + 0x5c) + 0xd4);
    (first, second)
}

/// The element at `address` is below the pivot: the `first` words (unsigned), then the `second`
/// words (unsigned), then the fourth words of the elements (signed), compared in that order.
fn element_below(e: &Engine, address: u32, pivot_keys: (u32, u32), pivot_word: u32) -> bool {
    let object = e.mem.u32(address);
    let (first, second) = key_words(e, object);
    if first != pivot_keys.0 {
        return first < pivot_keys.0;
    }
    if second != pivot_keys.1 {
        return second < pivot_keys.1;
    }
    (e.mem.u32(address + 0xc) as i32) < (pivot_word as i32)
}

/// The pivot is below the element at `address` (the same order the other way round).
fn pivot_below(e: &Engine, address: u32, pivot_keys: (u32, u32), pivot_word: u32) -> bool {
    let object = e.mem.u32(address);
    let (first, second) = key_words(e, object);
    if first != pivot_keys.0 {
        return pivot_keys.0 < first;
    }
    if second != pivot_keys.1 {
        return pivot_keys.1 < second;
    }
    (pivot_word as i32) < (e.mem.u32(address + 0xc) as i32)
}

// Translated from 00d9ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A recursive quicksort (Hoare partition around the middle element, `(first + last) >> 1`
/// with an arithmetic shift) of 16-byte elements: cdecl `(data, first, last, extra)`, where
/// `extra` is only handed to the recursive calls (`hkpendofstepcallbackutil.rs` calls it as
/// `ARRAY_SORT`). An element sorts by the word at `+0xd4` of the objects `[q + 0x58]` and `[q +
/// 0x5c]` (`q` the element's first word), in that order and unsigned, then by the element's fourth
/// word (signed). The left part is sorted by recursion, the right one by looping. The exe's
/// name for it is unknown (its Xbox PDB counterpart is an `hkAlgorithm` quick sort instance).
pub fn fn_00d9ec40(e: &mut Engine, data: u32, first: i32, last: i32, extra: u32) {
    let mut first = first;
    loop {
        let mut left = first;
        let mut right = last;
        let middle = (first.wrapping_add(last) >> 1).wrapping_shl(4) as u32;
        let pivot_at = middle.wrapping_add(data);
        let pivot_words: [u32; 4] = std::array::from_fn(|i| e.mem.u32(pivot_at + 4 * i as u32));
        loop {
            let pivot_keys = key_words(e, pivot_words[0]);
            let mut element = (left as u32).wrapping_shl(4).wrapping_add(data);
            while element_below(e, element, pivot_keys, pivot_words[3]) {
                left = left.wrapping_add(1);
                element = element.wrapping_add(0x10);
            }
            let mut element = (right as u32).wrapping_shl(4).wrapping_add(data);
            while pivot_below(e, element, pivot_keys, pivot_words[3]) {
                right = right.wrapping_sub(1);
                element = element.wrapping_sub(0x10);
            }
            if right < left {
                break;
            }
            if right != left {
                let high = (right as u32).wrapping_shl(4).wrapping_add(data);
                let low = (left as u32).wrapping_shl(4).wrapping_add(data);
                let held: [u32; 4] = std::array::from_fn(|i| e.mem.u32(high + 4 * i as u32));
                for i in 0..4u32 {
                    let word = e.mem.u32(low + 4 * i);
                    e.mem.set_u32(high + 4 * i, word);
                }
                for (i, word) in held.iter().enumerate() {
                    e.mem.set_u32(low + 4 * i as u32, *word);
                }
            }
            right = right.wrapping_sub(1);
            left = left.wrapping_add(1);
            if left > right {
                break;
            }
        }
        if first < right {
            fn_00d9ec40(e, data, first, right, extra);
        }
        if left < last {
            first = left;
        } else {
            return;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9ec10, fn_00d9ec10(Ptr, Ptr)),
        entry!(0x00d9_ee20, deleting_destructor_00d9ee20(Ptr, u8) -> Ptr),
        entry!(0x00d9_ee10, fn_00d9ee10(Ptr, u32) -> u32),
        entry!(0x00d9_ec40, fn_00d9ec40(u32, i32, i32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d9ee20_frees_only_when_bit_zero_is_set() {
        check(0x00d9_ee20, 0x00d0_b0d0);
    }

    #[test]
    fn fn_00d9ec10_swaps_three_words() {
        let mut e = Engine::new();
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(a + 4 * i, 10 + i);
            e.mem.set_u32(b + 4 * i, 20 + i);
        }
        e.call(0x00d9_ec10, &args![Ptr::<()>::new(a), Ptr::<()>::new(b)]);
        assert_eq!([0, 4, 8, 12].map(|o| e.mem.u32(a + o)), [20, 21, 22, 13]);
        assert_eq!([0, 4, 8, 12].map(|o| e.mem.u32(b + o)), [10, 11, 12, 23]);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d9ee10() {
        assert_eq!(check(0x00d9_ee10, 0x00d9_ee20, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// An element with the key words `first` and `second` and the fourth word `tail`; returns the
    /// object `q` it points to.
    fn object(e: &mut Engine, first: u32, second: u32) -> u32 {
        let q = e.mem.alloc(0x80);
        let a = e.mem.alloc(0xe0);
        let b = e.mem.alloc(0xe0);
        e.mem.set_u32(a + 0xd4, first);
        e.mem.set_u32(b + 0xd4, second);
        e.mem.set_u32(q + 0x58, a);
        e.mem.set_u32(q + 0x5c, b);
        q
    }

    fn array(e: &mut Engine, items: &[(u32, u32, u32)]) -> u32 {
        let data = e.mem.alloc(16 * items.len() as u32);
        for (i, (first, second, tail)) in items.iter().enumerate() {
            let q = object(e, *first, *second);
            e.mem.set_u32(data + 16 * i as u32, q);
            e.mem.set_u32(data + 16 * i as u32 + 4, 0xa000 + i as u32);
            e.mem.set_u32(data + 16 * i as u32 + 8, 0xb000 + i as u32);
            e.mem.set_u32(data + 16 * i as u32 + 0xc, *tail);
        }
        data
    }

    fn keys(e: &Engine, data: u32, count: u32) -> Vec<(u32, u32, i32)> {
        (0..count)
            .map(|i| {
                let q = e.mem.u32(data + 16 * i);
                let (first, second) = key_words(e, q);
                (first, second, e.mem.u32(data + 16 * i + 0xc) as i32)
            })
            .collect()
    }

    #[test]
    fn the_elements_end_up_ordered_by_first_second_and_signed_fourth_word() {
        let mut e = Engine::new();
        let items = [
            (5, 1, 0),
            (3, 9, 4),
            (5, 1, 0u32.wrapping_sub(2)),
            (3, 2, 7),
            (9, 0, 1),
            (3, 2, 5),
            (0xffff_fff0, 3, 0),
            (1, 0xffff_ffff, 2),
            (5, 0, 0),
            (3, 9, 1),
            (3, 9, 0),
        ];
        let data = array(&mut e, &items);
        let before = keys(&e, data, items.len() as u32);
        e.call(
            0x00d9_ec40,
            &args![data, 0u32, (items.len() - 1) as u32, 0x77u32],
        );
        let after = keys(&e, data, items.len() as u32);
        let mut want = before.clone();
        want.sort();
        assert_eq!(after, want);
        // The elements travel whole: the second and third words follow their first word.
        for i in 0..items.len() as u32 {
            let q = e.mem.u32(data + 16 * i);
            let slot = e.mem.u32(data + 16 * i + 4) - 0xa000;
            assert_eq!(e.mem.u32(data + 16 * i + 8), 0xb000 + slot);
            assert_eq!(key_words(&e, q).0, items[slot as usize].0);
        }
    }

    #[test]
    fn small_and_sorted_ranges_are_left_in_order() {
        let mut e = Engine::new();
        let data = array(&mut e, &[(1, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0)]);
        e.call(0x00d9_ec40, &args![data, 0u32, 3u32, 0u32]);
        assert_eq!(
            keys(&e, data, 4),
            [(1, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0)]
        );
        // A reverse range and a sub-range: only the asked part is sorted.
        let data = array(
            &mut e,
            &[(9, 0, 0), (4, 0, 0), (3, 0, 0), (2, 0, 0), (1, 0, 0)],
        );
        e.call(0x00d9_ec40, &args![data, 1u32, 3u32, 0u32]);
        assert_eq!(
            keys(&e, data, 5),
            [(9, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0), (1, 0, 0)]
        );
        // A single element and an empty range do nothing.
        e.call(0x00d9_ec40, &args![data, 2u32, 2u32, 0u32]);
        e.call(0x00d9_ec40, &args![data, 3u32, 1u32, 0u32]);
        assert_eq!(keys(&e, data, 5)[1], (2, 0, 0));
    }

    #[test]
    fn many_equal_keys_sort_without_running_away() {
        let mut e = Engine::new();
        let items: Vec<(u32, u32, u32)> = (0..20).map(|i| (i % 3, 7, 5)).collect();
        let data = array(&mut e, &items);
        e.call(0x00d9_ec40, &args![data, 0u32, 19u32, 0u32]);
        let after = keys(&e, data, 20);
        let mut want = after.clone();
        want.sort();
        assert_eq!(after, want);
    }
}
