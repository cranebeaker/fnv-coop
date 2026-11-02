//! `SpeedTree/run_00b169c0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `std::sort` machinery of SpeedTree runtime's branch pointer sort (an introsort of an
//! array of element pointers ordered by the `float` at offset `0x34` of the element: the sort,
//! the unguarded partition, the median of three, the insertion sort and the heap fall-back), the
//! helpers that copy ranges of 3-word, 6-word and 4-byte elements, and the 32-bit element
//! vector's allocator, buy, tidy and default constructor (see `run_00b32410` for the other
//! vector instantiations).
//!
//! The comparison the code builds with `FCOMPP` / `FNSTSW` / `TEST AH,5` / `JP` is true exactly
//! when `ST0 < ST1` and the two are ordered (a NaN gives false), which is Rust's `<` on `f32`.
//! Functions that carry an `FS:[0]` exception frame are marked `Translation incomplete` (the
//! unwinding handler is not translated; the throws are).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{StlVector, OPERATOR_DELETE};
use super::run_00b32410::{
    allocate_elements, clear_pointers, construct_proxy, first_of, move_bytes, LENGTH_ERROR,
};

/// `00b248c0`: the checked copy of a range (`cdecl`, six words: first, last, dest and three
/// words the exe leaves partly uninitialised).
const COPY_RANGE_CHECKED: u32 = 0x00b2_48c0;

/// The most 32-bit elements a vector can hold (`0x3fffffff`, `max_size`).
const WORD_LIMIT: u32 = 0x3fff_ffff;

/// The sort key of the element whose pointer is stored at `slot`: the `float` at offset
/// `0x34` of the element (`CIdvBranch::m_fFuzzyVolume` in the branch builder's layout).
fn key(e: &Engine, slot: u32) -> f32 {
    e.mem.f32(e.mem.u32(slot).wrapping_add(0x34))
}

/// The comparison of the element pointers stored at `lhs` and `rhs`: the key of `lhs` is less
/// than the key of `rhs` (the key of `rhs` is the first `FLD`, the key of `lhs` the second).
fn precedes(e: &Engine, lhs: u32, rhs: u32) -> bool {
    key(e, lhs) < key(e, rhs)
}

/// The key of an element pointer that is held by value rather than stored at an address.
fn key_of_element(e: &Engine, element: u32) -> f32 {
    e.mem.f32(element.wrapping_add(0x34))
}

/// Exchanges the element pointers stored at `x` and `y`.
fn swap_slots(e: &mut Engine, x: u32, y: u32) {
    let first = e.mem.u32(x);
    let second = e.mem.u32(y);
    e.mem.set_u32(x, second);
    e.mem.set_u32(y, first);
}

/// `std::iter_swap` as the exe inlines it: nothing when the two slots are the same address
/// (the code tests this twice in a row).
fn swap_if_different(e: &mut Engine, x: u32, y: u32) {
    if x != y {
        swap_slots(e, x, y);
    }
}

// Translated from 00b169c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (the engine map gives it no name): calls `00b248c0` with `first`,
/// `last`, `dest` and three words the exe leaves partly uninitialised (passed as zeros).
pub fn fn_00b169c0(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    e.call(
        COPY_RANGE_CHECKED,
        &args![first, last, dest, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b16a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (the engine map gives it no name): calls `00b16ec0` (the backward copy
/// of 6-word elements) with `first`, `last`, `dest_end` and three words the exe leaves partly
/// uninitialised (passed as zeros).
pub fn fn_00b16a10(e: &mut Engine, first: u32, last: u32, dest_end: u32) -> u32 {
    fn_00b16ec0(e, first, last, dest_end, 0, 0, 0)
}

// Translated from 00b16a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward copy of the 32-bit elements in `[first, last)` to end at `dest_end`
/// (`std::_Copy_backward_opt`, as the exe instantiates it; the engine map gives it no name):
/// with a positive element count (signed), `memmove_s` (`00ec7ce3`); answers the start of the
/// destination.
pub fn fn_00b16a60(e: &mut Engine, first: u32, last: u32, dest_end: u32) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 2;
    let start = dest_end.wrapping_sub((count as u32).wrapping_shl(2));
    if count > 0 {
        move_bytes(e, start, first, (count as u32).wrapping_shl(2));
    }
    start
}

// Translated from 00b16b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator of 32-bit elements (`std::allocator<int>::allocate`; the second word is not
/// read): a count of zero allocates zero bytes; a count above `0x3fffffff` throws
/// `std::bad_alloc`; otherwise `operator new(count * 4)`.
pub fn fn_00b16b00(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 4)
}

// Translated from 00b16b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Sort` (an introsort, as the exe instantiates it for the element pointers ordered by
/// their key; the engine map gives it no name): while the range holds more than 32 elements
/// and the allowed depth `ideal` is positive, partitions it (`00b16f70`), lets the depth
/// shrink (`ideal / 2 + ideal / 4` in signed arithmetic) and recurses into the smaller part
/// while looping on the larger; a range of more than 32 elements with the depth used up is heap
/// sorted (`00b174c0` makes the heap, then `00b17a60` pops the elements one by one); a range
/// of two to 32 elements is insertion sorted (`00b17630`).
pub fn fn_00b16b60(e: &mut Engine, first: u32, last: u32, ideal: i32, pred: u8) {
    let mut first = first;
    let mut last = last;
    let mut ideal = ideal;
    let mut count = last.wrapping_sub(first) as i32 >> 2;
    while count > 0x20 && ideal > 0 {
        let (middle_last, middle_first) = e.with_stack(8, |e, pair| {
            fn_00b16f70(e, pair, first, last, pred);
            (e.mem.u32(pair.addr()), e.mem.u32(pair.addr() + 4))
        });
        let half = ideal / 2;
        ideal = half / 2 + half;
        if (middle_last.wrapping_sub(first) as i32 >> 2)
            < (last.wrapping_sub(middle_first) as i32 >> 2)
        {
            fn_00b16b60(e, first, middle_last, ideal, pred);
            first = middle_first;
        } else {
            fn_00b16b60(e, middle_first, last, ideal, pred);
            last = middle_last;
        }
        count = last.wrapping_sub(first) as i32 >> 2;
    }
    if count > 0x20 {
        fn_00b174c0(e, first, last, pred);
        let mut cursor = last;
        while (cursor.wrapping_sub(first) as i32 >> 2) > 1 {
            fn_00b17a60(e, first, cursor, pred);
            cursor = cursor.wrapping_sub(4);
        }
    } else if count > 1 {
        fn_00b17630(e, first, last, u32::from(pred), 0);
    }
}

// Translated from 00b16cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b16cd0 - the FS:[0] exception frame is not translated.
/// The fill construction of `count` 6-word elements at `dest` from the element at `value`
/// (`std::_Uninit_fill_n`, as the exe instantiates it; the engine map gives it no name; the
/// three words after `value`, the allocator and two bytes, are not read): each non-null
/// destination gets the six words copied.
pub fn fn_00b16cd0(
    e: &mut Engine,
    dest: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut remaining = count;
    let mut target = dest;
    while remaining != 0 {
        if target != 0 {
            for word in 0..6 {
                let copied = e.mem.u32(value + 4 * word);
                e.mem.set_u32(target + 4 * word, copied);
            }
        }
        remaining -= 1;
        target = target.wrapping_add(0x18);
    }
}

// Translated from 00b16dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (the engine map gives it no name): calls `00b17b40` (the uninitialised
/// copy of 3-word elements) with `first`, `last`, `dest`, the allocator word and two bytes the
/// exe leaves partly uninitialised (passed as zeros). Answers what that function answers (the final destination).
pub fn fn_00b16dc0(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b17b40(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b16e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (the engine map gives it no name): calls `00b17c20` (the uninitialised
/// copy of 6-word elements) with `first`, `last`, `dest`, the allocator word and two bytes the
/// exe leaves partly uninitialised (passed as zeros). Answers what that function answers (the final destination).
pub fn fn_00b16e40(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b17c20(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b16ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward copy of the 6-word elements in `[first, last)` to end at `dest_end`
/// (`std::_Copy_backward_opt`, as the exe instantiates it; the engine map gives it no name; the
/// three words after `dest_end` are not read): the elements are copied last to first, word by
/// word. Answers the start of the destination, `dest_end` minus the element count (a signed
/// `IDIV` by 0x18) times 0x18, computed before the copying.
pub fn fn_00b16ec0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start =
        dest_end.wrapping_sub(((last.wrapping_sub(first) as i32 / 0x18) as u32).wrapping_mul(0x18));
    let mut source = last;
    let mut target = dest_end;
    while first != source {
        source = source.wrapping_sub(0x18);
        target = target.wrapping_sub(0x18);
        for word in 0..6 {
            let copied = e.mem.u32(source + 4 * word);
            e.mem.set_u32(target + 4 * word, copied);
        }
    }
    start
}

// Translated from 00b16f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Unguarded_partition` (as the exe instantiates it for the element pointers ordered by
/// their key; the engine map gives it no name; the comparison object, one byte, is not read):
/// picks the median of the range as the pivot (`00b17530` on the first, the middle and the
/// element before the end), widens the run of elements equal to the pivot around the middle,
/// then partitions with the equal elements swapped to the ends of the middle run. Writes the
/// start and the end of the run of pivot-equal elements to `out` (the pair of iterators the
/// C++ function returns) and returns `out`.
pub fn fn_00b16f70(e: &mut Engine, out: Ptr, first: u32, last: u32, pred: u8) -> Ptr {
    let count = last.wrapping_sub(first) as i32 >> 2;
    let middle = first.wrapping_add(((count / 2) as u32).wrapping_mul(4));
    fn_00b17530(e, first, middle, last.wrapping_sub(4), pred);
    let mut a = middle;
    let mut b = a.wrapping_add(4);
    while first < a && !precedes(e, a, a.wrapping_sub(4)) && !precedes(e, a.wrapping_sub(4), a) {
        a = a.wrapping_sub(4);
    }
    while b < last && !precedes(e, a, b) && !precedes(e, b, a) {
        b = b.wrapping_add(4);
    }
    let mut c = b;
    let mut d = a;
    loop {
        while c < last {
            if precedes(e, c, a) {
                c = c.wrapping_add(4);
                continue;
            }
            if precedes(e, a, c) {
                break;
            }
            let previous = b;
            b = b.wrapping_add(4);
            swap_if_different(e, previous, c);
            c = c.wrapping_add(4);
        }
        while first < d {
            if precedes(e, a, d.wrapping_sub(4)) {
                d = d.wrapping_sub(4);
                continue;
            }
            if precedes(e, d.wrapping_sub(4), a) {
                break;
            }
            a = a.wrapping_sub(4);
            swap_if_different(e, a, d.wrapping_sub(4));
            d = d.wrapping_sub(4);
        }
        if d == first && c == last {
            e.mem.set_u32(out.addr(), a);
            e.mem.set_u32(out.addr() + 4, b);
            return out;
        }
        if d == first {
            if b != c {
                swap_if_different(e, a, b);
            }
            b = b.wrapping_add(4);
            let old_a = a;
            let old_c = c;
            a = a.wrapping_add(4);
            c = c.wrapping_add(4);
            swap_if_different(e, old_a, old_c);
        } else if c == last {
            d = d.wrapping_sub(4);
            a = a.wrapping_sub(4);
            swap_if_different(e, d, a);
            b = b.wrapping_sub(4);
            swap_if_different(e, a, b);
        } else {
            d = d.wrapping_sub(4);
            swap_if_different(e, c, d);
            c = c.wrapping_add(4);
        }
    }
}

// Translated from 00b174c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Make_heap` (as the exe instantiates it for the element pointers ordered by their key;
/// the engine map gives it no name): for a range of more than one element, adjusts every
/// internal node from the last one (`count / 2 - 1`) down to the first with `00b17980`.
pub fn fn_00b174c0(e: &mut Engine, first: u32, last: u32, pred: u8) {
    if (last.wrapping_sub(first) as i32 >> 2) > 1 {
        let bottom = last.wrapping_sub(first) as i32 >> 2;
        let mut hole = bottom / 2;
        while hole > 0 {
            hole -= 1;
            let value = e.mem.u32(first.wrapping_add((hole as u32).wrapping_mul(4)));
            fn_00b17980(e, first, hole, bottom, value, pred);
        }
    }
}

// Translated from 00b17530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Med3`-style pivot selection (as the exe instantiates it for the element pointers
/// ordered by their key; the engine map gives it no name): for a range of up to 40 elements
/// orders the first, middle and last with `00b17850`; for a longer one first orders three
/// triples spaced `(count + 1) / 8` apart (rounded as the signed shift does) and then their
/// middles.
pub fn fn_00b17530(e: &mut Engine, first: u32, middle: u32, last: u32, pred: u8) {
    let count = last.wrapping_sub(first) as i32 >> 2;
    if count > 0x28 {
        let next = count.wrapping_add(1);
        let step = (next.wrapping_add((next >> 31) & 7) >> 3) as u32;
        let quarter = step.wrapping_mul(4);
        let eighth = step.wrapping_mul(8);
        fn_00b17850(
            e,
            first,
            first.wrapping_add(quarter),
            first.wrapping_add(eighth),
            u32::from(pred),
        );
        fn_00b17850(
            e,
            middle.wrapping_sub(quarter),
            middle,
            middle.wrapping_add(quarter),
            u32::from(pred),
        );
        fn_00b17850(
            e,
            last.wrapping_sub(eighth),
            last.wrapping_sub(quarter),
            last,
            u32::from(pred),
        );
        fn_00b17850(
            e,
            first.wrapping_add(quarter),
            middle,
            last.wrapping_sub(quarter),
            u32::from(pred),
        );
    } else {
        fn_00b17850(e, first, middle, last, u32::from(pred));
    }
}

// Translated from 00b17630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Insertion_sort` (as the exe instantiates it for the element pointers ordered by their
/// key; the engine map gives it no name; the last two words are not read): each element after
/// the first is compared with the first; one the first precedes goes to the front (the others
/// shifted up by the backward copy `00b17ac0`), any other is moved down past the elements that
/// precede it.
pub fn fn_00b17630(e: &mut Engine, first: u32, last: u32, _unused_2: u32, _unused_3: u32) {
    if first == last {
        return;
    }
    let mut cursor = first;
    loop {
        cursor = cursor.wrapping_add(4);
        if cursor == last {
            break;
        }
        let mut hole = cursor;
        let value = e.mem.u32(cursor);
        if key(e, first) < key_of_element(e, value) {
            hole = hole.wrapping_add(4);
            fn_00b17ac0(e, first, cursor, hole);
            e.mem.set_u32(first, value);
        } else {
            let mut previous = hole;
            loop {
                previous = previous.wrapping_sub(4);
                if key(e, previous) < key_of_element(e, value) {
                    let moved = e.mem.u32(previous);
                    e.mem.set_u32(hole, moved);
                    hole = previous;
                } else {
                    break;
                }
            }
            e.mem.set_u32(hole, value);
        }
    }
}

// Translated from 00b17740 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (a second instantiation of `00b16dc0`; the engine map gives it no
/// name): calls `00b17b40` with `first`, `last`, `dest`, the allocator word and two bytes the
/// exe leaves partly uninitialised (passed as zeros).
pub fn fn_00b17740(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b17b40(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b17790 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (a second instantiation of `00b16e40`; the engine map gives it no
/// name): calls `00b17c20` with `first`, `last`, `dest`, the allocator word and two bytes the
/// exe leaves partly uninitialised (passed as zeros).
pub fn fn_00b17790(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b17c20(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b177e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The uninitialised copy of 32-bit elements (`std::_Uninit_copy`, as the exe instantiates
/// it; the engine map gives it no name; the allocator word is not read): with an element count
/// (arithmetic shift) that is not zero, `memmove_s` (`00ec7ce3`) from `first` to `dest`;
/// answers `dest` plus the count.
pub fn fn_00b177e0(e: &mut Engine, first: u32, last: u32, dest: u32, _unused_3: u32) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 2;
    let end = dest.wrapping_add((count as u32).wrapping_mul(4));
    if count != 0 {
        move_bytes(e, dest, first, (count as u32).wrapping_shl(2));
    }
    end
}

// Translated from 00b17850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Orders three element pointers by their key (`std::_Med3`, as the exe instantiates it; the
/// engine map gives it no name; the comparison object word is not read): swaps the first two
/// when the first's key is less than the second's, the second and third when the second's is
/// less than the third's, then the first two again (each swap only for distinct slots).
pub fn fn_00b17850(e: &mut Engine, first: u32, second: u32, third: u32, _unused_3: u32) {
    if precedes(e, first, second) && second != first {
        swap_slots(e, second, first);
    }
    if precedes(e, second, third) && third != second {
        swap_slots(e, third, second);
    }
    if precedes(e, first, second) && second != first {
        swap_slots(e, second, first);
    }
}

// Translated from 00b17980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Adjust_heap` (as the exe instantiates it for the element pointers ordered by their
/// key; the engine map gives it no name): moves the hole at index `hole` down, each time
/// promoting the smaller-keyed child (of the two at `2 * hole + 1` and `2 * hole + 2`), until it
/// reaches the bottom (a lone last child is promoted too), then pushes `value` up from there
/// with `00b17d10` (stopping at the original hole `hole`).
pub fn fn_00b17980(e: &mut Engine, first: u32, hole: i32, bottom: i32, value: u32, pred: u8) {
    let top = hole;
    let mut hole = hole;
    let mut child = hole.wrapping_mul(2).wrapping_add(2);
    let slot = |index: i32| first.wrapping_add((index as u32).wrapping_mul(4));
    while child < bottom {
        if precedes(e, slot(child.wrapping_sub(1)), slot(child)) {
            child = child.wrapping_sub(1);
        }
        let moved = e.mem.u32(slot(child));
        e.mem.set_u32(slot(hole), moved);
        hole = child;
        child = child.wrapping_mul(2).wrapping_add(2);
    }
    if child == bottom {
        let moved = e.mem.u32(slot(bottom.wrapping_sub(1)));
        e.mem.set_u32(slot(hole), moved);
        hole = bottom.wrapping_sub(1);
    }
    fn_00b17d10(e, first, hole, top, value, pred);
}

// Translated from 00b17a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Pop_heap` (as the exe instantiates it for the element pointers ordered by their key;
/// the engine map gives it no name): for a range of more than one element, takes the last
/// element out, stores the first in its place and re-adjusts the heap of the remaining
/// elements from the root (`00b17980`).
pub fn fn_00b17a60(e: &mut Engine, first: u32, last: u32, pred: u8) {
    if (last.wrapping_sub(first) as i32 >> 2) > 1 {
        let value = e.mem.u32(last.wrapping_sub(4));
        let top = e.mem.u32(first);
        e.mem.set_u32(last.wrapping_sub(4), top);
        let bottom = last.wrapping_sub(4).wrapping_sub(first) as i32 >> 2;
        fn_00b17980(e, first, 0, bottom, value, pred);
    }
}

// Translated from 00b17ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward copy of the 32-bit elements in `[first, last)` to end at `dest_end`
/// (`std::_Copy_backward_opt`, as the exe instantiates it; the engine map gives it no name; a
/// second instantiation of `00b16a60`): with a positive element count (signed), `memmove_s`
/// (`00ec7ce3`); answers the start of the destination.
pub fn fn_00b17ac0(e: &mut Engine, first: u32, last: u32, dest_end: u32) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 2;
    let start = dest_end.wrapping_sub((count as u32).wrapping_shl(2));
    if count > 0 {
        move_bytes(e, start, first, (count as u32).wrapping_shl(2));
    }
    start
}

// Translated from 00b17b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b17b40 - the FS:[0] exception frame is not translated.
/// The uninitialised copy of the 3-word elements in `[first, last)` to `dest`
/// (`std::_Uninit_copy`, as the exe instantiates it; the engine map gives it no name; the
/// three words after `dest` are not read): each non-null destination gets the three words
/// copied. Answers the final destination.
pub fn fn_00b17b40(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut source = first;
    let mut target = dest;
    while source != last {
        if target != 0 {
            for word in 0..3 {
                let copied = e.mem.u32(source + 4 * word);
                e.mem.set_u32(target + 4 * word, copied);
            }
        }
        target = target.wrapping_add(0xC);
        source = source.wrapping_add(0xC);
    }
    target
}

// Translated from 00b17c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b17c20 - the FS:[0] exception frame is not translated.
/// The uninitialised copy of the 6-word elements in `[first, last)` to `dest`
/// (`std::_Uninit_copy`, as the exe instantiates it; the engine map gives it no name; the
/// three words after `dest` are not read): each non-null destination gets the six words
/// copied. Answers the final destination.
pub fn fn_00b17c20(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut source = first;
    let mut target = dest;
    while source != last {
        if target != 0 {
            for word in 0..6 {
                let copied = e.mem.u32(source + 4 * word);
                e.mem.set_u32(target + 4 * word, copied);
            }
        }
        target = target.wrapping_add(0x18);
        source = source.wrapping_add(0x18);
    }
    target
}

// Translated from 00b17d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Push_heap_0` (as the exe instantiates it for the element pointers ordered by their
/// key; the engine map gives it no name; the comparison object byte is not read): moves
/// `value` up from the hole at index `hole`, while the hole is above `top` and `value`'s key is
/// less than its parent's (index `(hole - 1) / 2`), copying the parent down; then stores
/// `value` in the hole.
pub fn fn_00b17d10(e: &mut Engine, first: u32, hole: i32, top: i32, value: u32, _unused_4: u8) {
    let slot = |index: i32| first.wrapping_add((index as u32).wrapping_mul(4));
    let mut hole = hole;
    loop {
        let parent = hole.wrapping_sub(1) / 2;
        if top >= hole {
            break;
        }
        if key_of_element(e, value) < key(e, slot(parent)) {
            let moved = e.mem.u32(slot(parent));
            e.mem.set_u32(slot(hole), moved);
            hole = parent;
        } else {
            break;
        }
    }
    e.mem.set_u32(slot(hole), value);
}

// Translated from 00b17db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b17db0 - the FS:[0] exception frame is not translated.
/// The default constructor of a 32-bit element vector (the `std::vector<unsigned short *>` of
/// `CIndexedGeometry`, `CONSTRUCT_POINTER_VECTOR`; the engine map gives it no name): the
/// container proxy, then the buy of zero elements (`00b17e90`). Returns `this`.
pub fn fn_00b17db0(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    construct_proxy(e, this);
    fn_00b17e90(e, this, 0);
    this
}

// Translated from 00b17e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The buy of a 32-bit element vector (the engine map gives it no name): clears the three
/// pointers; with a count of zero answers false; a count above `0x3fffffff` raises the length
/// error (`00b0fb80`); otherwise allocates the storage (`00b16b00`), points the first, last and
/// end at it and answers true.
pub fn fn_00b17e90(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let v = this.addr();
    clear_pointers(e, v);
    if count == 0 {
        return false;
    }
    let limit = WORD_LIMIT.max(1);
    if limit < count {
        e.call(LENGTH_ERROR, &args![]);
    } else {
        let storage = fn_00b16b00(e, count, 0);
        e.mem.set_u32(v + 0x0C, storage);
        e.mem.set_u32(v + 0x10, storage);
        e.mem
            .set_u32(v + 0x14, count.wrapping_mul(4).wrapping_add(storage));
    }
    true
}

// Translated from 00b17f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of a 32-bit element vector (the engine map gives it no name): frees the storage
/// (when there is any) and zeroes the first, last and end pointers.
pub fn fn_00b17f50(e: &mut Engine, this: Ptr<StlVector>) {
    let v = this.addr();
    if first_of(e, v) != 0 {
        let block = first_of(e, v);
        e.call(OPERATOR_DELETE, &args![block]);
    }
    clear_pointers(e, v);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b169c0, fn_00b169c0(u32, u32, u32) -> u32),
        entry!(0x00b16a10, fn_00b16a10(u32, u32, u32) -> u32),
        entry!(0x00b16a60, fn_00b16a60(u32, u32, u32) -> u32),
        entry!(0x00b16b00, fn_00b16b00(u32, u32) -> u32),
        entry!(0x00b16b60, fn_00b16b60(u32, u32, i32, u8)),
        entry!(0x00b16cd0, fn_00b16cd0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b16dc0, fn_00b16dc0(u32, u32, u32, u32) -> u32),
        entry!(0x00b16e40, fn_00b16e40(u32, u32, u32, u32) -> u32),
        entry!(
            0x00b16ec0,
            fn_00b16ec0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b16f70, fn_00b16f70(Ptr, u32, u32, u8) -> Ptr),
        entry!(0x00b174c0, fn_00b174c0(u32, u32, u8)),
        entry!(0x00b17530, fn_00b17530(u32, u32, u32, u8)),
        entry!(0x00b17630, fn_00b17630(u32, u32, u32, u32)),
        entry!(0x00b17740, fn_00b17740(u32, u32, u32, u32) -> u32),
        entry!(0x00b17790, fn_00b17790(u32, u32, u32, u32) -> u32),
        entry!(0x00b177e0, fn_00b177e0(u32, u32, u32, u32) -> u32),
        entry!(0x00b17850, fn_00b17850(u32, u32, u32, u32)),
        entry!(0x00b17980, fn_00b17980(u32, i32, i32, u32, u8)),
        entry!(0x00b17a60, fn_00b17a60(u32, u32, u8)),
        entry!(0x00b17ac0, fn_00b17ac0(u32, u32, u32) -> u32),
        entry!(
            0x00b17b40,
            fn_00b17b40(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00b17c20,
            fn_00b17c20(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b17d10, fn_00b17d10(u32, i32, i32, u32, u8)),
        entry!(0x00b17db0, fn_00b17db0(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b17e90, fn_00b17e90(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b17f50, fn_00b17f50(Ptr<StlVector>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::branch::OPERATOR_NEW;
    use super::super::run_00b32410::{ALLOCATE_PROXY, MEMMOVE_S};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the allocator, `memmove_s` and the throws doubled.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(ALLOCATE_PROXY, |e, a| ret(e.mem.alloc(4 * a[0])));
        e.register(LENGTH_ERROR, |_, _| -> Ret { panic!("vector too long") });
        e.register(0x00ec_78f1, |_, a| ret(a[0]));
        e.register(0x00ec_7894, |_, a| -> Ret { panic!("thrown {:#x}", a[1]) });
        e
    }

    fn recording(e: &mut Engine, address: u32, answer: u32) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(answer)
        });
        log
    }

    /// An element: a block whose float at `+0x34` is `key`.
    fn element(e: &mut Engine, key: f32) -> u32 {
        let at = e.mem.alloc(0x40);
        e.mem.set_f32(at + 0x34, key);
        at
    }

    /// An array of element pointers with the given keys (the pointers in order).
    fn array(e: &mut Engine, keys: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            let item = element(e, *key);
            e.mem.set_u32(at + 4 * i as u32, item);
        }
        at
    }

    /// The keys of the elements an array of pointers refers to, in order.
    fn keys(e: &Engine, array: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| e.mem.f32(e.mem.u32(array + 4 * i) + 0x34))
            .collect()
    }

    fn sorted_descending(mut values: Vec<f32>) -> Vec<f32> {
        values.sort_by(|a, b| b.partial_cmp(a).unwrap());
        values
    }

    /// Distinct pseudo-random keys.
    fn spread(count: u32) -> Vec<f32> {
        (0..count).map(|i| ((i * 37) % 101) as f32).collect()
    }

    // ---- the sort -------------------------------------------------------------------------

    #[test]
    fn a_short_range_is_insertion_sorted_in_descending_key_order() {
        let mut e = engine();
        let input = [3.0, 1.0, 4.0, 1.5, 9.0, 2.0, 6.0];
        let a = array(&mut e, &input);
        e.call(0x00b1_6b60, &args![a, a + 28, 100i32, 0u8]);
        assert_eq!(keys(&e, a, 7), sorted_descending(input.to_vec()));
    }

    #[test]
    fn a_long_range_is_partitioned_and_sorted() {
        let mut e = engine();
        let input = spread(60);
        let a = array(&mut e, &input);
        e.call(0x00b1_6b60, &args![a, a + 240, 100i32, 0u8]);
        assert_eq!(keys(&e, a, 60), sorted_descending(input));
    }

    #[test]
    fn a_long_range_with_the_depth_used_up_is_heap_sorted() {
        let mut e = engine();
        let input = spread(40);
        let a = array(&mut e, &input);
        e.call(0x00b1_6b60, &args![a, a + 160, 0i32, 0u8]);
        assert_eq!(keys(&e, a, 40), sorted_descending(input));
    }

    #[test]
    fn a_single_element_range_is_left_alone() {
        let mut e = engine();
        let a = array(&mut e, &[5.0]);
        e.call(0x00b1_6b60, &args![a, a + 4, 100i32, 0u8]);
        assert_eq!(keys(&e, a, 1), vec![5.0]);
    }

    #[test]
    fn the_partition_leaves_larger_keys_before_and_smaller_after_the_pivot() {
        let mut e = engine();
        let input = spread(45);
        let a = array(&mut e, &input);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00b1_6f70, &args![out, a, a + 180, 0u8]).u32(), out);
        let (low, high) = (e.mem.u32(out), e.mem.u32(out + 4));
        assert_eq!(high, low + 4);
        let pivot = e.mem.f32(e.mem.u32(low) + 0x34);
        for i in 0..45u32 {
            let key = e.mem.f32(e.mem.u32(a + 4 * i) + 0x34);
            let position = a + 4 * i;
            if position < low {
                assert!(key > pivot);
            } else if position >= high {
                assert!(key < pivot);
            }
        }
        let mut seen = keys(&e, a, 45);
        seen.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let mut expected = input;
        expected.sort_by(|x, y| x.partial_cmp(y).unwrap());
        assert_eq!(seen, expected);
    }

    #[test]
    fn the_median_orders_three_keys_descending_and_keeps_every_element() {
        let mut e = engine();
        let a = array(&mut e, &[1.0, 5.0, 3.0]);
        e.call(0x00b1_7530, &args![a, a + 4, a + 8, 0u8]);
        assert_eq!(keys(&e, a, 3), vec![5.0, 3.0, 1.0]);
        let input = spread(100);
        let b = array(&mut e, &input);
        e.call(0x00b1_7530, &args![b, b + 200, b + 396, 0u8]);
        let mut seen = keys(&e, b, 100);
        seen.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let mut expected = input;
        expected.sort_by(|x, y| x.partial_cmp(y).unwrap());
        assert_eq!(seen, expected);
    }

    #[test]
    fn the_insertion_sort_orders_descending_and_leaves_empty_and_single_ranges() {
        let mut e = engine();
        let input = [2.0, 8.0, 5.0, 7.0, 1.0];
        let a = array(&mut e, &input);
        e.call(0x00b1_7630, &args![a, a + 20, 0u32, 0u32]);
        assert_eq!(keys(&e, a, 5), vec![8.0, 7.0, 5.0, 2.0, 1.0]);
        e.call(0x00b1_7630, &args![a, a, 0u32, 0u32]);
        e.call(0x00b1_7630, &args![a, a + 4, 0u32, 0u32]);
        assert_eq!(keys(&e, a, 5), vec![8.0, 7.0, 5.0, 2.0, 1.0]);
    }

    #[test]
    fn three_keys_are_ordered_by_the_median_helper() {
        let mut e = engine();
        let a = array(&mut e, &[1.0, 5.0, 3.0]);
        e.call(0x00b1_7850, &args![a, a + 4, a + 8, 0u32]);
        assert_eq!(keys(&e, a, 3), vec![5.0, 3.0, 1.0]);
        let b = array(&mut e, &[2.0, 2.0, 2.0]);
        e.call(0x00b1_7850, &args![b, b + 4, b + 8, 0u32]);
        assert_eq!(keys(&e, b, 3), vec![2.0, 2.0, 2.0]);
        let c = array(&mut e, &[f32::NAN, 1.0, 2.0]);
        e.call(0x00b1_7850, &args![c, c + 4, c + 8, 0u32]);
        let after = keys(&e, c, 3);
        assert_eq!(after[1], 2.0);
        assert_eq!(after[2], 1.0);
    }

    // ---- the heap -------------------------------------------------------------------------

    #[test]
    fn make_heap_builds_a_heap_whose_parents_have_the_smaller_keys() {
        let mut e = engine();
        let input = [9.0, 4.0, 7.0, 1.0, 8.0, 2.0, 6.0];
        let a = array(&mut e, &input);
        e.call(0x00b1_74c0, &args![a, a + 28, 0u8]);
        let heap = keys(&e, a, 7);
        for child in 1..7usize {
            assert!(heap[(child - 1) / 2] <= heap[child]);
        }
        assert_eq!(sorted_descending(heap), sorted_descending(input.to_vec()));
        let single = array(&mut e, &[3.0]);
        e.call(0x00b1_74c0, &args![single, single + 4, 0u8]);
        assert_eq!(keys(&e, single, 1), vec![3.0]);
    }

    #[test]
    fn adjust_heap_moves_the_hole_down_and_the_value_back_up() {
        let mut e = engine();
        let a = array(&mut e, &[1.0, 3.0, 2.0, 7.0, 4.0, 5.0]);
        let value = element(&mut e, 6.0);
        e.call(0x00b1_7980, &args![a, 0i32, 6i32, value, 0u8]);
        assert_eq!(keys(&e, a, 6), vec![2.0, 3.0, 5.0, 7.0, 4.0, 6.0]);
    }

    #[test]
    fn pop_heap_moves_the_root_to_the_end_and_restores_the_heap() {
        let mut e = engine();
        let a = array(&mut e, &[1.0, 3.0, 2.0, 7.0, 4.0, 5.0]);
        e.call(0x00b1_7a60, &args![a, a + 24, 0u8]);
        assert_eq!(keys(&e, a, 6), vec![2.0, 3.0, 5.0, 7.0, 4.0, 1.0]);
        let one = array(&mut e, &[9.0]);
        e.call(0x00b1_7a60, &args![one, one + 4, 0u8]);
        assert_eq!(keys(&e, one, 1), vec![9.0]);
    }

    #[test]
    fn push_heap_moves_the_value_up_while_its_key_is_smaller() {
        let mut e = engine();
        let a = array(&mut e, &[1.0, 3.0, 2.0, 7.0, 4.0, 0.0]);
        let value = element(&mut e, 0.5);
        e.call(0x00b1_7d10, &args![a, 5i32, 0i32, value, 0u8]);
        assert_eq!(keys(&e, a, 6), vec![0.5, 3.0, 1.0, 7.0, 4.0, 2.0]);
        let b = array(&mut e, &[1.0, 3.0, 0.0]);
        let big = element(&mut e, 9.0);
        e.call(0x00b1_7d10, &args![b, 2i32, 0i32, big, 0u8]);
        assert_eq!(e.mem.u32(b + 8), big);
    }

    // ---- the range helpers ----------------------------------------------------------------

    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    fn numbered(e: &mut Engine, count: u32) -> u32 {
        let at = e.mem.alloc(4 * count);
        for i in 0..count {
            e.mem.set_u32(at + 4 * i, i + 1);
        }
        at
    }

    #[test]
    fn the_forwarder_passes_its_range_to_the_checked_copy() {
        let mut e = engine();
        let log = recording(&mut e, 0x00b2_48c0, 0x1234);
        assert_eq!(e.call(0x00b1_69c0, &args![1u32, 2u32, 3u32]).u32(), 0x1234);
        assert_eq!(*log.borrow(), vec![vec![1, 2, 3, 0, 0, 0]]);
    }

    #[test]
    fn the_six_word_backward_forwarder_copies_the_range() {
        let mut e = engine();
        let source = numbered(&mut e, 12);
        let destination = e.mem.alloc(0x30);
        let start = e
            .call(
                0x00b1_6a10,
                &args![source, source + 0x30, destination + 0x30],
            )
            .u32();
        assert_eq!(start, destination);
        assert_eq!(words(&e, destination, 12), words(&e, source, 12));
    }

    #[test]
    fn the_six_word_backward_copy_moves_up_overlapping_ranges() {
        let mut e = engine();
        let block = numbered(&mut e, 18);
        let start = e
            .call(
                0x00b1_6ec0,
                &args![block, block + 0x30, block + 0x48, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(start, block + 0x18);
        let moved = words(&e, block, 18);
        assert_eq!(moved[..6], [1, 2, 3, 4, 5, 6]);
        assert_eq!(moved[6..12], [1, 2, 3, 4, 5, 6]);
        assert_eq!(moved[12..], [7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn the_word_backward_copies_answer_the_start_and_skip_empty_ranges() {
        for address in [0x00b1_6a60u32, 0x00b1_7ac0] {
            let mut e = engine();
            let block = numbered(&mut e, 4);
            let start = e.call(address, &args![block, block + 12, block + 16]).u32();
            assert_eq!(start, block + 4);
            assert_eq!(words(&e, block, 4), vec![1, 1, 2, 3]);
            let log = recording(&mut e, MEMMOVE_S, 0);
            e.call(address, &args![block, block, block + 16]);
            e.call(address, &args![block + 12, block, block + 16]);
            assert!(log.borrow().is_empty());
        }
    }

    #[test]
    fn the_word_uninitialised_copy_answers_the_end_and_skips_empty_ranges() {
        let mut e = engine();
        let source = numbered(&mut e, 3);
        let destination = e.mem.alloc(16);
        let end = e
            .call(0x00b1_77e0, &args![source, source + 12, destination, 0u32])
            .u32();
        assert_eq!(end, destination + 12);
        assert_eq!(words(&e, destination, 3), vec![1, 2, 3]);
        let log = recording(&mut e, MEMMOVE_S, 0);
        e.call(0x00b1_77e0, &args![source, source, destination, 0u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_six_word_fill_copies_the_value_and_skips_a_null_destination() {
        let mut e = engine();
        let value = numbered(&mut e, 6);
        let destination = e.mem.alloc(0x30);
        e.call(
            0x00b1_6cd0,
            &args![destination, 2u32, value, 0u32, 0u32, 0u32],
        );
        assert_eq!(words(&e, destination, 6), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(words(&e, destination + 0x18, 6), vec![1, 2, 3, 4, 5, 6]);
        e.call(0x00b1_6cd0, &args![0u32, 1u32, value, 0u32, 0u32, 0u32]);
    }

    /// Checks an uninitialised copy of `size`-byte elements (`words` words each).
    fn check_uninit_copy(address: u32, word_count: u32) {
        let mut e = engine();
        let source = numbered(&mut e, 2 * word_count);
        let destination = e.mem.alloc(8 * word_count);
        let end = e
            .call(
                address,
                &args![
                    source,
                    source + 8 * word_count,
                    destination,
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        assert_eq!(end, destination + 8 * word_count);
        assert_eq!(
            words(&e, destination, 2 * word_count),
            words(&e, source, 2 * word_count)
        );
        // A null destination is skipped but still advanced (only the first one is null).
        let end = e
            .call(
                address,
                &args![source, source + 4 * word_count, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, 4 * word_count);
    }

    #[test]
    fn the_three_word_uninitialised_copy_copies_each_element() {
        check_uninit_copy(0x00b1_7b40, 3);
    }

    #[test]
    fn the_six_word_uninitialised_copy_copies_each_element() {
        check_uninit_copy(0x00b1_7c20, 6);
    }

    #[test]
    fn the_forwarders_to_the_uninitialised_copies_answer_their_result() {
        for (forwarder, size) in [
            (0x00b1_6dc0u32, 0xcu32),
            (0x00b1_7740, 0xc),
            (0x00b1_6e40, 0x18),
            (0x00b1_7790, 0x18),
        ] {
            let mut e = engine();
            let source = numbered(&mut e, 2 * size / 4);
            let destination = e.mem.alloc(8 * size / 4);
            let end = e
                .call(
                    forwarder,
                    &args![source, source + 2 * size, destination, 0u32],
                )
                .u32();
            assert_eq!(end, destination + 2 * size);
            assert_eq!(
                words(&e, destination, 2 * size / 4),
                words(&e, source, 2 * size / 4)
            );
        }
    }

    // ---- the 32-bit element vector --------------------------------------------------------

    #[test]
    fn the_allocator_allocates_and_throws_for_a_huge_count() {
        let mut e = engine();
        let block = e.call(0x00b1_6b00, &args![3u32, 0u32]).u32();
        assert_eq!(e.mem.block_size(block), Some(16));
        assert_ne!(e.call(0x00b1_6b00, &args![0u32, 0u32]).u32(), 0);
    }

    #[test]
    #[should_panic(expected = "thrown 0x1140ef8")]
    fn the_allocator_throws_bad_alloc_when_the_size_overflows() {
        let mut e = engine();
        e.call(0x00b1_6b00, &args![0x4000_0000u32, 0u32]);
    }

    #[test]
    fn the_default_constructor_makes_a_proxy_and_an_empty_vector() {
        let mut e = engine();
        let v = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00b1_7db0, &args![v]).u32(), v);
        let proxy = e.mem.u32(v);
        assert_ne!(proxy, 0);
        assert_eq!(e.mem.u32(proxy), v);
        assert_eq!(
            (e.mem.u32(v + 0xC), e.mem.u32(v + 0x10), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
    }

    #[test]
    fn the_buy_answers_false_for_zero_allocates_otherwise_and_checks_the_limit() {
        let mut e = engine();
        let v = e.mem.alloc(0x18);
        assert!(!e.call(0x00b1_7e90, &args![v, 0u32]).bool());
        assert!(e.call(0x00b1_7e90, &args![v, 3u32]).bool());
        let first = e.mem.u32(v + 0xC);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(v + 0x10), first);
        assert_eq!(e.mem.u32(v + 0x14), first + 12);
        let log = recording(&mut e, LENGTH_ERROR, 0);
        assert!(e.call(0x00b1_7e90, &args![v, 0x4000_0000u32]).bool());
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(v + 0xC), 0);
    }

    #[test]
    fn the_tidy_frees_the_storage_and_clears_the_pointers() {
        let mut e = engine();
        let v = e.mem.alloc(0x18);
        e.call(0x00b1_7e90, &args![v, 2u32]);
        let storage = e.mem.u32(v + 0xC);
        e.call(0x00b1_7f50, &args![v]);
        assert_eq!(
            (e.mem.u32(v + 0xC), e.mem.u32(v + 0x10), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
        assert!(e.mem.block_size(storage).is_none());
        e.call(0x00b1_7f50, &args![v]);
    }
}
