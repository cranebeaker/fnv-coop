//! `fallout shared/facegen/run_0064dc30` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0064de90` calls on the member (`0064e2d0`).
const CALLEE_0064E2D0: u32 = 0x0064_e2d0;

// Translated from 0064de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0064e2d0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0064de90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0064E2D0, &args![this.byte_add(0x8)]).u32()
}

// The functions below are one `std::vector` instance of the compiler's library (Visual C++
// 2008, checked iterators) for 16-byte elements, and its helpers. The vector keeps its
// allocator at `+8` and its first, last and end-of-storage pointers at `+0xc`, `+0x10` and
// `+0x14`; a checked iterator is two words (its owner and the element pointer).

/// The vector's pointers.
const FIRST: u32 = 0xc;
const LAST: u32 = 0x10;
const END: u32 = 0x14;
/// The allocator member.
const ALLOCATOR: u32 = 8;

/// The library helpers called by address: the iterator construction `0065ded0` (`thiscall`,
/// three words: the out iterator and the two words), the iterator comparison `0065dfd0`
/// (`thiscall` on an iterator, one word: the other; answers a byte), the two iterator getters
/// `0065f550` and `0044a170` (`thiscall`, one word: the out iterator; answer its address),
/// the iterator's owner test `005ca4f0` and owner `0060b980` (`thiscall`),
/// `_invalid_parameter_noinfo` (`00ec7c56`), the iterator base constructor `0044dee0`
/// (`thiscall`), the pointer category tag `0065e750` (`cdecl`, the addresses of one or two
/// iterators; answers the top byte of the caller's `ECX`, so it is registered with `ECX`
/// first and two stack words), the iterator category `0065fd70` (`cdecl`, three words: a
/// result byte's address and two iterators' addresses; answers the result's address), the
/// unchecked pointer `0065fe40` (`cdecl`, an iterator's address), the element destructor
/// `0065e9a0` (`thiscall` on the allocator, one word), `operator new` (`00401000`) and
/// `operator delete` (`00401030`, `cdecl`), the placement `operator new` `006e6da0`
/// (`cdecl`: size and place), the `std::bad_alloc` constructor `0044d6c0` (`thiscall`, one
/// word) and `_CxxThrowException` (`00ec7894`), the length error `0065f640`, the fill
/// `0064e4b0` and the backward copy `0064e4e0` (`cdecl`, three words each).
const MAKE_ITERATOR_0065DED0: u32 = 0x0065_ded0;
const ITERATORS_DIFFER_0065DFD0: u32 = 0x0065_dfd0;
const ITERATOR_0065F550: u32 = 0x0065_f550;
const ITERATOR_0044A170: u32 = 0x0044_a170;
const ITERATOR_HAS_OWNER_005CA4F0: u32 = 0x005c_a4f0;
const ITERATOR_OWNER_0060B980: u32 = 0x0060_b980;
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
const ITERATOR_BASE_0044DEE0: u32 = 0x0044_dee0;
const POINTER_CATEGORY_0065E750: u32 = 0x0065_e750;
const ITERATOR_CATEGORY_0065FD70: u32 = 0x0065_fd70;
const UNCHECKED_0065FE40: u32 = 0x0065_fe40;
const DESTROY_ELEMENT_0065E9A0: u32 = 0x0065_e9a0;
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const PLACEMENT_NEW_006E6DA0: u32 = 0x006e_6da0;
const BAD_ALLOC_CONSTRUCT: u32 = 0x0044_d6c0;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The throw information of `std::bad_alloc` (`01140ef8`).
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
const LENGTH_ERROR_0065F640: u32 = 0x0065_f640;
const FILL_0064E4B0: u32 = 0x0064_e4b0;
const COPY_BACKWARD_0064E4E0: u32 = 0x0064_e4e0;

/// The element count between two pointers (`SAR 4` of the byte distance).
fn elements(first: u32, last: u32) -> u32 {
    ((last.wrapping_sub(first) as i32) >> 4) as u32
}

/// Copies one 16-byte element.
fn copy_element(e: &mut Engine, dest: u32, source: u32) {
    for offset in [0u32, 4, 8, 0xc] {
        let word = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(dest.wrapping_add(offset), word);
    }
}

// Translated from 0064dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `size` (`(last - first) >> 4`, arithmetic shift).
pub fn fn_0064dc30(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.mem.u32(this.addr().wrapping_add(FIRST));
    let last = e.mem.u32(this.addr().wrapping_add(LAST));
    elements(first, last)
}

// Translated from 0064dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's range `erase` (`thiscall`, five words: the result
/// iterator's address and the two iterators). Both iterators are rebuilt (`0065ded0`); when
/// they differ (`0065dfd0`), the tail from the second is copied down to the first
/// ([`fn_0064e380`]), the elements after the new last destroyed ([`fn_0064dee0`]) and the
/// last pointer moved. The result is the first iterator; answers its address.
pub fn fn_0064dc50(
    e: &mut Engine,
    this: Ptr,
    result: u32,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
) -> u32 {
    let vector = this.addr();
    e.with_stack(0x10, |e, frame| {
        let from = frame.addr();
        let to = from + 8;
        e.call(
            MAKE_ITERATOR_0065DED0,
            &args![vector, from, first_owner, first],
        );
        e.call(MAKE_ITERATOR_0065DED0, &args![vector, to, last_owner, last]);
        if e.call(ITERATORS_DIFFER_0065DFD0, &args![from, to]).bool() {
            let old_last = e.mem.u32(vector.wrapping_add(LAST));
            let (source, dest) = (e.mem.u32(to + 4), e.mem.u32(from + 4));
            let new_last = fn_0064e380(e, source, old_last, dest);
            let old_last = e.mem.u32(vector.wrapping_add(LAST));
            fn_0064dee0(e, this, new_last, old_last);
            e.mem.set_u32(vector.wrapping_add(LAST), new_last);
        }
        let owner = e.mem.u32(from);
        e.mem.set_u32(result, owner);
        let pointer = e.mem.u32(from + 4);
        e.mem.set_u32(result.wrapping_add(4), pointer);
    });
    result
}

// Translated from 0064dcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `assign(count, value)` (`thiscall`, two words: the
/// count and the value's address). The value is copied first; the whole range is erased
/// ([`fn_0064dc50`] with the iterators from `0044a170` and `0065f550`) and `count` copies are
/// inserted at the iterator `0044a170` then answers ([`fn_0064deb0`]).
pub fn fn_0064dcf0(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    let vector = this.addr();
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let copy = frame;
        copy_element(e, copy, value);
        let end_slot = frame + 0x10;
        let at = e.call(ITERATOR_0065F550, &args![vector, end_slot]).u32();
        let (end_owner, end) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
        let begin_slot = frame + 0x18;
        let at = e.call(ITERATOR_0044A170, &args![vector, begin_slot]).u32();
        let (begin_owner, begin) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
        let erased = frame + 0x20;
        fn_0064dc50(e, this, erased, begin_owner, begin, end_owner, end);
        let where_slot = frame + 0x28;
        let at = e.call(ITERATOR_0044A170, &args![vector, where_slot]).u32();
        let (where_owner, place) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
        fn_0064deb0(e, this, where_owner, place, count, copy);
    });
}

/// The checked iterator's two tests: it has an owner, and its pointer is before the owner's
/// last element (`+0x10`); each failure calls `_invalid_parameter_noinfo` and goes on.
fn check_iterator(e: &mut Engine, this: Ptr) {
    if !e.call(ITERATOR_HAS_OWNER_005CA4F0, &args![this]).bool() {
        e.call(INVALID_PARAMETER, &[]);
    }
    let owner = e.call(ITERATOR_OWNER_0060B980, &args![this]).u32();
    let pointer = e.mem.u32(this.addr().wrapping_add(4));
    if pointer >= e.mem.u32(owner.wrapping_add(0x10)) {
        e.call(INVALID_PARAMETER, &[]);
    }
}

// Translated from 0064dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the checked iterator's dereference: the checks (an owner, the
/// pointer before its last element; `_invalid_parameter_noinfo` on failure), then answers the
/// element pointer (`+4`).
pub fn fn_0064dda0(e: &mut Engine, this: Ptr) -> u32 {
    check_iterator(e, this);
    e.mem.u32(this.addr().wrapping_add(4))
}

// Translated from 0064ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the checked iterator's pre-increment: the same checks as
/// [`fn_0064dda0`], then the element pointer moves on 16 bytes. Answers `this`.
pub fn fn_0064ddf0(e: &mut Engine, this: Ptr) -> Ptr {
    check_iterator(e, this);
    let pointer = e.mem.u32(this.addr().wrapping_add(4));
    e.mem
        .set_u32(this.addr().wrapping_add(4), pointer.wrapping_add(0x10));
    this
}

// Translated from 0064de50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor that runs [`fn_0064de70`] and answers `this`.
pub fn fn_0064de50(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0064de70(e, this);
    this
}

// Translated from 0064de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor that runs the base constructor `0044dee0` and
/// answers `this`.
pub fn fn_0064de70(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ITERATOR_BASE_0044DEE0, &args![this]);
    this
}

// Translated from 0064deb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `insert(where, count, value)` (`thiscall`, four
/// words: the iterator's two words, the count, the value's address), passed on to
/// [`fn_0064df50`].
pub fn fn_0064deb0(e: &mut Engine, this: Ptr, owner: u32, place: u32, count: u32, value: u32) {
    fn_0064df50(e, this, owner, place, count, value);
}

// Translated from 0064dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `_Destroy(first, last)`: [`fn_0064e3f0`] with the
/// allocator at `+8`.
pub fn fn_0064dee0(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_0064e3f0(e, first, last, this.addr().wrapping_add(ALLOCATOR));
}

// Translated from 0064df10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the allocator's `deallocate(pointer, count)`: `operator delete`
/// (`00401030`) on the pointer (the count is not read).
pub fn fn_0064df10(e: &mut Engine, _this: Ptr, pointer: u32, _unused_2: u32) {
    e.call(OPERATOR_DELETE, &args![pointer]);
}

// Translated from 0064df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the allocator's `allocate(count)`: [`fn_0064e430`] with the
/// count and 0.
pub fn fn_0064df30(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    fn_0064e430(e, count, 0)
}

// Translated from 0064df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `_Insert_n(where, count, value)` (`thiscall`, four
/// words: the iterator's owner, which is not read, its element pointer, the count and the
/// value's address). Nothing for a zero count; the length error (`0065f640`) when the count
/// does not fit under `max_size` ([`fn_0064de90`]). Without room, a new storage of
/// `max(capacity * 1.5 (0 when that would pass max_size), size + count)` elements gets the
/// head, `count` copies and the tail, and the old elements are destroyed and freed. With
/// room: when fewer elements follow `where` than are inserted, the tail moves up, the extra
/// copies go after the old last element and the old tail range is filled; otherwise the last
/// `count` elements are copied past the end, the rest moved back (`0064e4e0`) and the gap
/// filled (`0064e4b0`). The value is copied into a local first in both of those cases.
///
/// The exception frame (`FS:[0]`, the clean-up when a copy throws) is not translated.
// Translation incomplete: 0064df50 - the FS:[0] exception frame (the catch blocks that free the new storage or destroy the copies) is not translated
pub fn fn_0064df50(e: &mut Engine, this: Ptr, _unused_1: u32, place: u32, count: u32, value: u32) {
    let vector = this.addr();
    let field = |offset: u32| vector.wrapping_add(offset);
    let capacity = fn_0064e300(e, this);
    if count == 0 {
        return;
    }
    let size = fn_0064dc30(e, this);
    if fn_0064de90(e, this).wrapping_sub(size) < count {
        e.call(LENGTH_ERROR_0065F640, &[]);
        return;
    }
    if capacity < fn_0064dc30(e, this).wrapping_add(count) {
        let half = capacity >> 1;
        let mut new_capacity = if fn_0064de90(e, this).wrapping_sub(half) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if new_capacity < fn_0064dc30(e, this).wrapping_add(count) {
            new_capacity = fn_0064dc30(e, this).wrapping_add(count);
        }
        let storage = fn_0064df30(e, Ptr::new(field(ALLOCATOR)), new_capacity);
        let first = e.mem.u32(field(FIRST));
        let next = fn_0064e480(e, this, first, place, storage);
        let next = fn_0064e340(e, this, next, count, value);
        let last = e.mem.u32(field(LAST));
        fn_0064e480(e, this, place, last, next);
        let total = fn_0064dc30(e, this).wrapping_add(count);
        if e.mem.u32(field(FIRST)) != 0 {
            let (first, last) = (e.mem.u32(field(FIRST)), e.mem.u32(field(LAST)));
            fn_0064dee0(e, this, first, last);
            let (first, end) = (e.mem.u32(field(FIRST)), e.mem.u32(field(END)));
            fn_0064df10(e, Ptr::new(field(ALLOCATOR)), first, elements(first, end));
        }
        e.mem
            .set_u32(field(END), (new_capacity << 4).wrapping_add(storage));
        e.mem
            .set_u32(field(LAST), (total << 4).wrapping_add(storage));
        e.mem.set_u32(field(FIRST), storage);
        return;
    }
    e.with_stack(0x10, |e, copy| {
        let copy = copy.addr();
        let last = e.mem.u32(field(LAST));
        if elements(place, last) < count {
            copy_element(e, copy, value);
            let last = e.mem.u32(field(LAST));
            fn_0064e480(e, this, place, last, (count << 4).wrapping_add(place));
            let last = e.mem.u32(field(LAST));
            let extra = count.wrapping_sub(elements(place, last));
            fn_0064e340(e, this, last, extra, copy);
            let last = e.mem.u32(field(LAST));
            e.mem.set_u32(field(LAST), (count << 4).wrapping_add(last));
            let last = e.mem.u32(field(LAST));
            e.call(
                FILL_0064E4B0,
                &args![place, last.wrapping_sub(count << 4), copy],
            );
        } else {
            copy_element(e, copy, value);
            let old_last = e.mem.u32(field(LAST));
            let last = e.mem.u32(field(LAST));
            let moved = old_last.wrapping_sub(count << 4);
            let new_last = fn_0064e480(e, this, moved, old_last, last);
            e.mem.set_u32(field(LAST), new_last);
            e.call(COPY_BACKWARD_0064E4E0, &args![place, moved, old_last]);
            e.call(
                FILL_0064E4B0,
                &args![place, (count << 4).wrapping_add(place), copy],
            );
        }
    });
}

// Translated from 0064e2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the allocator's `max_size`: `0x0fffffff` (the compiler kept the
/// "at least 1" test, which never fails).
pub fn fn_0064e2d0(_e: &mut Engine, _this: Ptr) -> u32 {
    let count = 0x0fff_ffffu32;
    if count > 0 {
        count
    } else {
        1
    }
}

// Translated from 0064e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `capacity`: 0 without storage, else
/// `(end - first) >> 4`.
pub fn fn_0064e300(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.mem.u32(this.addr().wrapping_add(FIRST));
    if first == 0 {
        return 0;
    }
    let end = e.mem.u32(this.addr().wrapping_add(END));
    elements(first, end)
}

// Translated from 0064e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `_Ufill(first, count, value)` (`thiscall`, three
/// words): [`fn_0064e550`] with the allocator, answering `first + count * 16`.
pub fn fn_0064e340(e: &mut Engine, this: Ptr, first: u32, count: u32, value: u32) -> u32 {
    fn_0064e550(e, first, count, value, this.addr().wrapping_add(ALLOCATOR));
    (count << 4).wrapping_add(first)
}

// Translated from 0064e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `std::copy(first, last, dest)` of the checked library (`cdecl`,
/// three words). Takes the pointer category tag (`0065e750`) and the iterator category
/// (`0065fd70`) of the arguments, the unchecked pointers (`0065fe40`), and answers
/// [`fn_0064e5a0`] with them (the range-checked tag word 0).
pub fn fn_0064e380(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    e.with_stack(0x10, |e, frame| {
        let frame = frame.addr();
        let (first_slot, last_slot, dest_slot) = (frame, frame + 4, frame + 8);
        let (checked, tag, category) = (frame + 0xc, frame + 0xd, frame + 0xe);
        e.mem.set_u32(first_slot, first);
        e.mem.set_u32(last_slot, last);
        e.mem.set_u32(dest_slot, dest);
        e.mem.set_u8(checked, 0);
        // `ECX` is the address of the `dest` word; the pushes are those of `first` and `dest`.
        let pointer_tag = e
            .call(
                POINTER_CATEGORY_0065E750,
                &args![dest_slot, first_slot, dest_slot],
            )
            .u32() as u8;
        e.mem.set_u8(tag, pointer_tag);
        let at = e
            .call(
                ITERATOR_CATEGORY_0065FD70,
                &args![category, first_slot, dest_slot],
            )
            .u32();
        let iterator_tag = e.mem.u8(at);
        let dest = e.mem.u32(dest_slot);
        let last = e.call(UNCHECKED_0065FE40, &args![last_slot]).u32();
        let first = e.call(UNCHECKED_0065FE40, &args![first_slot]).u32();
        let checked = e.mem.u8(checked);
        fn_0064e5a0(
            e,
            first,
            last,
            dest,
            u32::from(iterator_tag),
            u32::from(pointer_tag),
            u32::from(checked),
        )
    })
}

// Translated from 0064e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `_Destroy_range(first, last, allocator)` (`cdecl`): the pointer
/// category tag (`0065e750` on both) and [`fn_0064e600`].
pub fn fn_0064e3f0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    e.with_stack(8, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame, first);
        e.mem.set_u32(frame + 4, last);
        // `ECX` is the address of the `first` word; the pushes are those of `first` and `last`.
        let tag = e
            .call(POINTER_CATEGORY_0065E750, &args![frame, frame, frame + 4])
            .u32() as u8;
        let (first, last) = (e.mem.u32(frame), e.mem.u32(frame + 4));
        fn_0064e600(e, first, last, allocator, u32::from(tag));
    });
}

// Translated from 0064e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `_Allocate(count, 0)` (`cdecl`, two words; the second is not
/// read): a count whose 16-byte size does not fit in 32 bits throws `std::bad_alloc`
/// (`0044d6c0`, `_CxxThrowException` with `01140ef8`); otherwise `operator new(count * 16)`
/// (a zero count allocates 0 bytes).
pub fn fn_0064e430(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    if count != 0 && 0xffff_ffff / count < 0x10 {
        e.with_stack(0xc, |e, exception| {
            e.call(BAD_ALLOC_CONSTRUCT, &args![exception, 0u32]);
            e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
        });
        unreachable!("_CxxThrowException does not return (00ec7894)");
    }
    e.call(OPERATOR_NEW, &args![count << 4]).u32()
}

// Translated from 0064e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vector's `_Ucopy(first, last, dest)` (`thiscall`, three
/// words): [`fn_0064e630`] with the allocator; answers the end of the copies.
pub fn fn_0064e480(e: &mut Engine, this: Ptr, first: u32, last: u32, dest: u32) -> u32 {
    fn_0064e630(e, first, last, dest, this.addr().wrapping_add(ALLOCATOR))
}

// Translated from 0064e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `uninitialized_fill_n(first, count, value, allocator)`
/// (`cdecl`, four words): the pointer category tag (`0065e750` on the first argument's slot,
/// twice) and [`fn_0064e710`].
pub fn fn_0064e550(e: &mut Engine, first: u32, count: u32, value: u32, allocator: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), first);
        // `ECX` and both pushes are the address of the `first` word.
        let tag = e
            .call(POINTER_CATEGORY_0065E750, &args![slot, slot, slot])
            .u32() as u8;
        let first = e.mem.u32(slot.addr());
        fn_0064e710(e, first, count, value, allocator, u32::from(tag), 0);
    });
}

// Translated from 0064e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the copy's random-access, non-scalar form (`cdecl`, six words:
/// first, last, dest and three tag words that are not read): copies the elements
/// ([`fn_0064e7f0`] with the destination's unchecked pointer, `0065fe40`) and answers
/// `dest + (last - first)`, rounded down to whole elements. The exe passes on a tag byte of
/// its own frame that it never sets; here the frame starts zeroed.
pub fn fn_0064e5a0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let end = (elements(first, last) << 4).wrapping_add(dest);
    e.with_stack(8, |e, frame| {
        let frame = frame.addr();
        let dest_slot = frame;
        let unset_tag = frame + 7;
        e.mem.set_u32(dest_slot, dest);
        let checked = 0u32;
        let scalar = 0u32;
        let unset = u32::from(e.mem.u8(unset_tag));
        let unchecked = e.call(UNCHECKED_0065FE40, &args![dest_slot]).u32();
        fn_0064e7f0(e, first, last, unchecked, unset, scalar, checked);
    });
    end
}

// Translated from 0064e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destroy loop (`cdecl`, four words; the tag is not read):
/// the allocator's element destructor (`0065e9a0`) on each 16-byte element from `first` up
/// to `last`.
pub fn fn_0064e600(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_3: u32) {
    let mut element = first;
    while element != last {
        e.call(DESTROY_ELEMENT_0065E9A0, &args![allocator, element]);
        element = element.wrapping_add(0x10);
    }
}

// Translated from 0064e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `uninitialized_copy(first, last, dest, allocator)` (`cdecl`,
/// four words): the pointer category tag of `dest` (`0065e750`), the unchecked pointers of
/// `last` and `first` (`0065fe40`), then [`fn_0064e840`]; answers its result.
pub fn fn_0064e630(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    e.with_stack(0x10, |e, frame| {
        // The local word at EBP-4 (its byte EBP-1 cleared, EBP-2 the tag); the argument
        // words follow it.
        let local = frame.addr();
        let (first_slot, last_slot, dest_slot) = (local + 4, local + 8, local + 0xc);
        e.mem.set_u32(first_slot, first);
        e.mem.set_u32(last_slot, last);
        e.mem.set_u32(dest_slot, dest);
        e.mem.set_u8(local + 3, 0);
        // `ECX` and the one push are the address of the `dest` word; the next stack word,
        // which the tag function does not read either, is the local word.
        let word = e.mem.u32(local);
        let tag = e
            .call(
                POINTER_CATEGORY_0065E750,
                &args![dest_slot, dest_slot, word],
            )
            .u32() as u8;
        e.mem.set_u8(local + 2, tag);
        let dest = e.mem.u32(dest_slot);
        let last = e.call(UNCHECKED_0065FE40, &args![last_slot]).u32();
        let first = e.call(UNCHECKED_0065FE40, &args![first_slot]).u32();
        fn_0064e840(e, first, last, dest, allocator, u32::from(tag), 0)
    })
}

// Translated from 0064e710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the fill loop (`cdecl`, six words; the two tags are not read):
/// constructs `count` copies of the value from `first` on ([`fn_0064e7d0`] on the
/// allocator).
///
/// The exception frame (`FS:[0]`, the clean-up when a copy throws) is not translated.
// Translation incomplete: 0064e710 - the FS:[0] exception frame (the catch block that destroys the copies made) is not translated
pub fn fn_0064e710(
    e: &mut Engine,
    first: u32,
    count: u32,
    value: u32,
    allocator: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let (mut element, mut left) = (first, count);
    while left != 0 {
        fn_0064e7d0(e, Ptr::new(allocator), element, value);
        left -= 1;
        element = element.wrapping_add(0x10);
    }
}

// Translated from 0064e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the allocator's `construct(pointer, value)` (`thiscall`, two
/// words): [`fn_0064e8c0`]; answers what it answers.
pub fn fn_0064e7d0(e: &mut Engine, _this: Ptr, pointer: u32, value: u32) -> u32 {
    fn_0064e8c0(e, pointer, value)
}

// Translated from 0064e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the element copy loop (`cdecl`, six words; the three tags are
/// not read): copies each 16-byte element from `first` up to `last` to `dest` on; answers the
/// end of the destination.
pub fn fn_0064e7f0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let (mut source, mut target) = (first, dest);
    while source != last {
        copy_element(e, target, source);
        target = target.wrapping_add(0x10);
        source = source.wrapping_add(0x10);
    }
    target
}

// Translated from 0064e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the uninitialized copy's dispatch (`cdecl`, six words; the two
/// tags are not read): [`fn_0064e920`] with the first four.
pub fn fn_0064e840(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    allocator: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    fn_0064e920(e, first, last, dest, allocator)
}

// Translated from 0064e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `_Construct(pointer, value)` (`cdecl`): placement `operator new`
/// (`006e6da0` with 16 and the pointer); when that answers a place, the 16-byte value is
/// copied there. Answers the place.
pub fn fn_0064e8c0(e: &mut Engine, pointer: u32, value: u32) -> u32 {
    let place = e
        .call(PLACEMENT_NEW_006E6DA0, &args![0x10u32, pointer])
        .u32();
    if place != 0 {
        copy_element(e, place, value);
    }
    place
}

// Translated from 0064e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `uninitialized_copy` of the checked library (`cdecl`, four
/// words): the pointer category tag of `first` and `dest` (`0065e750`), the unchecked
/// pointers of `last` and `first` (`0065fe40`), then [`fn_0064e9c0`] (range-checked tag 0).
pub fn fn_0064e920(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    e.with_stack(0xc, |e, frame| {
        let frame = frame.addr();
        let (first_slot, last_slot, dest_slot) = (frame, frame + 4, frame + 8);
        e.mem.set_u32(first_slot, first);
        e.mem.set_u32(last_slot, last);
        e.mem.set_u32(dest_slot, dest);
        // `ECX` is the address of the `dest` word; the pushes are those of `first` and `dest`.
        let tag = e
            .call(
                POINTER_CATEGORY_0065E750,
                &args![dest_slot, first_slot, dest_slot],
            )
            .u32() as u8;
        let dest = e.mem.u32(dest_slot);
        let last = e.call(UNCHECKED_0065FE40, &args![last_slot]).u32();
        let first = e.call(UNCHECKED_0065FE40, &args![first_slot]).u32();
        fn_0064e9c0(e, first, last, dest, allocator, u32::from(tag), 0)
    })
}

// Translated from 0064e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the uninitialized copy loop (`cdecl`, six words; the two tags
/// are not read): constructs a copy of each element from `first` up to `last` at `dest` on
/// ([`fn_0064e7d0`] on the allocator); answers the end of the destination.
///
/// The exception frame (`FS:[0]`, the clean-up when a copy throws) is not translated.
// Translation incomplete: 0064e9c0 - the FS:[0] exception frame (the catch block that destroys the copies made) is not translated
pub fn fn_0064e9c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    allocator: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let (mut source, mut target) = (first, dest);
    while source != last {
        fn_0064e7d0(e, Ptr::new(allocator), target, source);
        target = target.wrapping_add(0x10);
        source = source.wrapping_add(0x10);
    }
    target
}

// Translated from 0064e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `std::fill(first, last, value)` of the checked library
/// (`cdecl`, three words: the range and the value's address): the unchecked pointers of
/// `last` and `first` (`0065fe40` on the argument slots), then [`fn_0064e680`].
pub fn fn_0064e4b0(e: &mut Engine, first: u32, last: u32, value: u32) {
    e.with_stack(8, |e, frame| {
        let frame = frame.addr();
        let (first_slot, last_slot) = (frame, frame + 4);
        e.mem.set_u32(first_slot, first);
        e.mem.set_u32(last_slot, last);
        let last = e.call(UNCHECKED_0065FE40, &args![last_slot]).u32();
        let first = e.call(UNCHECKED_0065FE40, &args![first_slot]).u32();
        fn_0064e680(e, first, last, value);
    });
}

// Translated from 0064e4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `std::copy_backward(first, last, dest)` of the checked
/// library (`cdecl`, three words). Takes the pointer category tag (`0065e750`, registered
/// with the caller's `ECX` first: here the address of the `dest` word, which is also the one
/// word pushed; the next stack word is the frame's local word) and the iterator category
/// (`0065fd70`) of `first` and `dest`, the unchecked pointers of `last` and `first`
/// (`0065fe40`), and passes them with `dest`, the category, the tag and the range-checked
/// tag 0 to [`fn_0064e6c0`].
pub fn fn_0064e4e0(e: &mut Engine, first: u32, last: u32, dest: u32) {
    e.with_stack(0x10, |e, frame| {
        // The local word at EBP-4 holds the bytes at EBP-3 (category), EBP-2 (tag) and EBP-1
        // (the range-checked tag); the argument words follow it.
        let local = frame.addr();
        let (category, tag, checked) = (local + 1, local + 2, local + 3);
        let (first_slot, last_slot, dest_slot) = (local + 4, local + 8, local + 0xc);
        e.mem.set_u32(first_slot, first);
        e.mem.set_u32(last_slot, last);
        e.mem.set_u32(dest_slot, dest);
        e.mem.set_u8(checked, 0);
        let word = e.mem.u32(local);
        let pointer_tag = e
            .call(
                POINTER_CATEGORY_0065E750,
                &args![dest_slot, dest_slot, word],
            )
            .u32() as u8;
        e.mem.set_u8(tag, pointer_tag);
        let at = e
            .call(
                ITERATOR_CATEGORY_0065FD70,
                &args![category, first_slot, dest_slot],
            )
            .u32();
        let iterator_tag = e.mem.u8(at);
        let dest = e.mem.u32(dest_slot);
        let last = e.call(UNCHECKED_0065FE40, &args![last_slot]).u32();
        let first = e.call(UNCHECKED_0065FE40, &args![first_slot]).u32();
        let checked = e.mem.u8(checked);
        fn_0064e6c0(
            e,
            first,
            last,
            dest,
            u32::from(iterator_tag),
            u32::from(pointer_tag),
            u32::from(checked),
        );
    });
}

// Translated from 0064e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the fill loop (`cdecl`, three words): copies the 16-byte value
/// at `value` into each element from `first` up to `last`.
pub fn fn_0064e680(e: &mut Engine, first: u32, last: u32, value: u32) {
    let mut at = first;
    while at != last {
        copy_element(e, at, value);
        at = at.wrapping_add(0x10);
    }
}

// Translated from 0064e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the backward copy's dispatch (`cdecl`, six words: first, last,
/// dest, the iterator category byte and two tag words that are not read): the pointer
/// category tag (`0065e750` with `ECX` the address of the `dest` word and the addresses of the
/// `first` and `dest` words pushed), then
/// [`fn_0064e860`] with the first three, the category byte, the tag and the range-checked
/// tag 0.
pub fn fn_0064e6c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    category: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    e.with_stack(0xc, |e, frame| {
        let frame = frame.addr();
        let (first_slot, dest_slot) = (frame, frame + 4);
        e.mem.set_u32(first_slot, first);
        e.mem.set_u32(dest_slot, dest);
        let tag = e
            .call(
                POINTER_CATEGORY_0065E750,
                &args![dest_slot, first_slot, dest_slot],
            )
            .u32() as u8;
        let (first, dest) = (e.mem.u32(first_slot), e.mem.u32(dest_slot));
        fn_0064e860(e, first, last, dest, category & 0xff, u32::from(tag), 0);
    });
}

// Translated from 0064e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the backward copy's random-access, non-scalar form (`cdecl`,
/// six words: first, last, dest and three tag words that are not read): copies the
/// elements backward ([`fn_0064e980`] with the destination's unchecked pointer, `0065fe40`)
/// and answers `dest - (last - first)`, rounded down to whole elements. The exe passes on a
/// tag byte of its own frame that it never sets (not read by the copy); here the frame
/// starts zeroed.
pub fn fn_0064e860(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = dest.wrapping_sub(elements(first, last) << 4);
    e.with_stack(8, |e, frame| {
        let frame = frame.addr();
        let dest_slot = frame;
        let unset_tag = frame + 4;
        e.mem.set_u32(dest_slot, dest);
        let unset = u32::from(e.mem.u8(unset_tag));
        let target = e.call(UNCHECKED_0065FE40, &args![dest_slot]).u32();
        fn_0064e980(e, first, last, target, unset, 0, 0);
    });
    start
}

// Translated from 0064e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the backward element copy loop (`cdecl`, six words; the three
/// tags are not read): copies each 16-byte element from `last` down to `first` to before
/// `dest`; answers the start of the destination.
pub fn fn_0064e980(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let (mut source, mut target) = (last, dest);
    while first != source {
        source = source.wrapping_sub(0x10);
        target = target.wrapping_sub(0x10);
        copy_element(e, target, source);
    }
    target
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_de90, fn_0064de90(Ptr) -> u32),
        entry!(0x0064_dc30, fn_0064dc30(Ptr) -> u32),
        entry!(
            0x0064_dc50,
            fn_0064dc50(Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0064_dcf0, fn_0064dcf0(Ptr, u32, u32)),
        entry!(0x0064_dda0, fn_0064dda0(Ptr) -> u32),
        entry!(0x0064_ddf0, fn_0064ddf0(Ptr) -> Ptr),
        entry!(0x0064_de50, fn_0064de50(Ptr) -> Ptr),
        entry!(0x0064_de70, fn_0064de70(Ptr) -> Ptr),
        entry!(0x0064_deb0, fn_0064deb0(Ptr, u32, u32, u32, u32)),
        entry!(0x0064_dee0, fn_0064dee0(Ptr, u32, u32)),
        entry!(0x0064_df10, fn_0064df10(Ptr, u32, u32)),
        entry!(0x0064_df30, fn_0064df30(Ptr, u32) -> u32),
        entry!(0x0064_df50, fn_0064df50(Ptr, u32, u32, u32, u32)),
        entry!(0x0064_e2d0, fn_0064e2d0(Ptr) -> u32),
        entry!(0x0064_e300, fn_0064e300(Ptr) -> u32),
        entry!(0x0064_e340, fn_0064e340(Ptr, u32, u32, u32) -> u32),
        entry!(0x0064_e380, fn_0064e380(u32, u32, u32) -> u32),
        entry!(0x0064_e3f0, fn_0064e3f0(u32, u32, u32)),
        entry!(0x0064_e430, fn_0064e430(u32, u32) -> u32),
        entry!(0x0064_e480, fn_0064e480(Ptr, u32, u32, u32) -> u32),
        entry!(0x0064_e550, fn_0064e550(u32, u32, u32, u32)),
        entry!(
            0x0064_e5a0,
            fn_0064e5a0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0064_e600, fn_0064e600(u32, u32, u32, u32)),
        entry!(0x0064_e4b0, fn_0064e4b0(u32, u32, u32)),
        entry!(0x0064_e4e0, fn_0064e4e0(u32, u32, u32)),
        entry!(0x0064_e680, fn_0064e680(u32, u32, u32)),
        entry!(0x0064_e6c0, fn_0064e6c0(u32, u32, u32, u32, u32, u32)),
        entry!(0x0064_e860, fn_0064e860(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0064_e980, fn_0064e980(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0064_e630, fn_0064e630(u32, u32, u32, u32) -> u32),
        entry!(0x0064_e710, fn_0064e710(u32, u32, u32, u32, u32, u32)),
        entry!(0x0064_e7d0, fn_0064e7d0(Ptr, u32, u32) -> u32),
        entry!(
            0x0064_e7f0,
            fn_0064e7f0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0064_e840,
            fn_0064e840(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0064_e8c0, fn_0064e8c0(u32, u32) -> u32),
        entry!(0x0064_e920, fn_0064e920(u32, u32, u32, u32) -> u32),
        entry!(
            0x0064_e9c0,
            fn_0064e9c0(u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0064de90_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0064E2D0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_de90, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0064E2D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The library helpers as plain pointer operations: tags answer 0x100, the unchecked
    /// pointer reads the slot, placement new answers the place, `operator new` the heap,
    /// iterators are copied words, and the length error, delete, destroy, fill and backward
    /// copy are recorded (the last two do their work on 16-byte elements).
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(POINTER_CATEGORY_0065E750, |_, _| 0x100u32.into_ret());
        e.register(ITERATOR_CATEGORY_0065FD70, |e, a| {
            e.mem.set_u8(a[0], 7);
            a[0].into_ret()
        });
        e.register(UNCHECKED_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(MAKE_ITERATOR_0065DED0, |e, a| {
            e.mem.set_u32(a[1], a[2]);
            e.mem.set_u32(a[1] + 4, a[3]);
            Ret::default()
        });
        e.register(ITERATORS_DIFFER_0065DFD0, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        // Begin and end iterators of the vector.
        e.register(ITERATOR_0044A170, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            let first = e.mem.u32(a[0] + FIRST);
            e.mem.set_u32(a[1] + 4, first);
            a[1].into_ret()
        });
        e.register(ITERATOR_0065F550, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            let last = e.mem.u32(a[0] + LAST);
            e.mem.set_u32(a[1] + 4, last);
            a[1].into_ret()
        });
        e.register(FILL_0064E4B0, |e, a| {
            let mut at = a[0];
            while at != a[1] {
                copy_element(e, at, a[2]);
                at += 0x10;
            }
            Ret::default()
        });
        e.register(COPY_BACKWARD_0064E4E0, |e, a| {
            let (mut source, mut dest) = (a[1], a[2]);
            while source != a[0] {
                source -= 0x10;
                dest -= 0x10;
                copy_element(e, dest, source);
            }
            dest.into_ret()
        });
        for addr in [
            OPERATOR_DELETE,
            DESTROY_ELEMENT_0065E9A0,
            LENGTH_ERROR_0065F640,
            INVALID_PARAMETER,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CALLEE_0064E2D0, |_, _| 0x0fff_ffffu32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A vector holding elements `1..=n` (word 0 of each), with `capacity` slots.
    fn vector(e: &mut Engine, n: u32, capacity: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        let storage = e.mem.alloc(16 * capacity.max(1));
        for i in 0..n {
            e.mem.set_u32(storage + 16 * i, i + 1);
            e.mem.set_u32(storage + 16 * i + 12, 0xe0 + i);
        }
        e.mem.set_u32(this + FIRST, storage);
        e.mem.set_u32(this + LAST, storage + 16 * n);
        e.mem.set_u32(this + END, storage + 16 * capacity);
        this
    }

    fn contents(e: &Engine, this: u32) -> Vec<u32> {
        let (first, last) = (e.mem.u32(this + FIRST), e.mem.u32(this + LAST));
        (0..elements(first, last))
            .map(|i| e.mem.u32(first + 16 * i))
            .collect()
    }

    fn element_value(e: &mut Engine, word: u32) -> u32 {
        let value = e.mem.alloc(16);
        e.mem.set_u32(value, word);
        e.mem.set_u32(value + 12, 0xcc);
        value
    }

    #[test]
    fn size_capacity_and_max_size() {
        let mut e = rig();
        let this = vector(&mut e, 3, 5);
        assert_eq!(e.call(0x0064_dc30, &args![this]).u32(), 3);
        assert_eq!(e.call(0x0064_e300, &args![this]).u32(), 5);
        e.mem.set_u32(this + FIRST, 0);
        assert_eq!(e.call(0x0064_e300, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0064_e2d0, &args![this]).u32(), 0x0fff_ffff);
    }

    #[test]
    fn erase_range_moves_the_tail_down() {
        let mut e = rig();
        let this = vector(&mut e, 5, 5);
        let first = e.mem.u32(this + FIRST);
        let result = e.mem.alloc(8);
        let back = e
            .call(
                0x0064_dc50,
                &args![this, result, this, first + 16, this, first + 48],
            )
            .u32();
        assert_eq!(back, result);
        assert_eq!(contents(&e, this), vec![1, 4, 5]);
        assert_eq!(
            (e.mem.u32(result), e.mem.u32(result + 4)),
            (this, first + 16)
        );
        // The two elements past the new last are destroyed.
        assert_eq!(
            calls(&e, DESTROY_ELEMENT_0065E9A0),
            vec![vec![this + 8, first + 48], vec![this + 8, first + 64]]
        );
        // An empty range changes nothing.
        e.call(0x0064_dc50, &args![this, result, this, first, this, first]);
        assert_eq!(contents(&e, this), vec![1, 4, 5]);
    }

    #[test]
    fn assign_replaces_everything_with_copies() {
        let mut e = rig();
        let this = vector(&mut e, 2, 4);
        let value = element_value(&mut e, 9);
        e.call(0x0064_dcf0, &args![this, 3u32, value]);
        assert_eq!(contents(&e, this), vec![9, 9, 9]);
        let first = e.mem.u32(this + FIRST);
        assert_eq!(e.mem.u32(first + 12), 0xcc);
    }

    #[test]
    fn insert_with_room_after_a_short_tail() {
        let mut e = rig();
        let this = vector(&mut e, 3, 8);
        let first = e.mem.u32(this + FIRST);
        let value = element_value(&mut e, 9);
        // Two elements follow the place; three are inserted.
        e.call(0x0064_deb0, &args![this, 0u32, first + 16, 3u32, value]);
        assert_eq!(contents(&e, this), vec![1, 9, 9, 9, 2, 3]);
        assert_eq!(e.mem.u32(this + FIRST), first);
    }

    #[test]
    fn insert_with_room_before_a_long_tail() {
        let mut e = rig();
        let this = vector(&mut e, 4, 8);
        let first = e.mem.u32(this + FIRST);
        let value = element_value(&mut e, 9);
        e.call(0x0064_df50, &args![this, 0u32, first + 16, 2u32, value]);
        assert_eq!(contents(&e, this), vec![1, 9, 9, 2, 3, 4]);
        assert_eq!(calls(&e, COPY_BACKWARD_0064E4E0).len(), 1);
    }

    #[test]
    fn insert_without_room_grows_the_storage() {
        let mut e = rig();
        let this = vector(&mut e, 2, 2);
        let old = e.mem.u32(this + FIRST);
        let value = element_value(&mut e, 9);
        e.call(0x0064_df50, &args![this, 0u32, old + 16, 2u32, value]);
        assert_eq!(contents(&e, this), vec![1, 9, 9, 2]);
        let first = e.mem.u32(this + FIRST);
        assert_ne!(first, old);
        // Capacity 2 * 1.5 = 3 is below 4: the new storage holds 4.
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x40]]);
        assert_eq!(e.mem.u32(this + END), first + 0x40);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![old]]);
        assert_eq!(calls(&e, DESTROY_ELEMENT_0065E9A0).len(), 2);
    }

    #[test]
    fn insert_of_nothing_or_too_much() {
        let mut e = rig();
        let this = vector(&mut e, 2, 2);
        let first = e.mem.u32(this + FIRST);
        let value = element_value(&mut e, 9);
        e.call(0x0064_df50, &args![this, 0u32, first, 0u32, value]);
        assert!(calls(&e, CALLEE_0064E2D0).is_empty());
        e.call(
            0x0064_df50,
            &args![this, 0u32, first, 0x0fff_fffeu32, value],
        );
        assert_eq!(calls(&e, LENGTH_ERROR_0065F640).len(), 1);
        assert_eq!(contents(&e, this), vec![1, 2]);
    }

    #[test]
    fn iterator_dereference_and_increment_check_the_iterator() {
        let mut e = rig();
        let owner = e.mem.alloc(0x14);
        e.mem.set_u32(owner + 0x10, 0x5020);
        e.map(0x0f00_0000, 0x1000);
        e.register(ITERATOR_HAS_OWNER_005CA4F0, |e, _| {
            e.mem.u32(0x0f00_0000).into_ret()
        });
        e.register_double(ITERATOR_OWNER_0060B980, move |_, _| owner.into_ret());
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator + 4, 0x5000);
        e.mem.set_u32(0x0f00_0000, 1);
        assert_eq!(e.call(0x0064_dda0, &args![iterator]).u32(), 0x5000);
        assert_eq!(e.call(0x0064_ddf0, &args![iterator]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), 0x5010);
        e.call(0x0064_ddf0, &args![iterator]);
        assert!(calls(&e, INVALID_PARAMETER).is_empty());
        // At the last element of the owner the check fails, and the step still happens.
        e.call(0x0064_ddf0, &args![iterator]);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 1);
        assert_eq!(e.mem.u32(iterator + 4), 0x5030);
        // No owner (low byte 0) and past the end: both checks fail.
        e.mem.set_u32(0x0f00_0000, 0x100);
        e.call(0x0064_dda0, &args![iterator]);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 3);
    }

    #[test]
    fn constructors_and_allocator_helpers() {
        let mut e = rig();
        e.register(ITERATOR_BASE_0044DEE0, |_, _| Ret::default());
        assert_eq!(e.call(0x0064_de50, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(e.call(0x0064_de70, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(calls(&e, ITERATOR_BASE_0044DEE0), vec![vec![0x5000]; 2]);
        e.call(0x0064_df10, &args![0x5008u32, 0x7000u32, 3u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x7000]]);
        let block = e.call(0x0064_df30, &args![0x5008u32, 3u32]).u32();
        assert_ne!(block, 0);
        e.call(0x0064_e430, &args![0u32, 0u32]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x30], vec![0]]);
    }

    #[test]
    fn allocate_throws_bad_alloc_for_a_huge_count() {
        let mut e = rig();
        e.register(BAD_ALLOC_CONSTRUCT, |_, _| Ret::default());
        e.register(CXX_THROW_EXCEPTION, |_, a| -> Ret {
            panic!("thrown {:#x}", a[1])
        });
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x0064_e430, &args![0x1000_0000u32, 0u32]);
        }));
        assert!(thrown.is_err());
    }

    #[test]
    fn copy_fill_and_destroy_helpers() {
        let mut e = rig();
        let source = vector(&mut e, 3, 3);
        let first = e.mem.u32(source + FIRST);
        let dest = e.mem.alloc(0x40);
        // std::copy answers the end of the copies.
        let end = e.call(0x0064_e380, &args![first, first + 0x30, dest]).u32();
        assert_eq!(end, dest + 0x30);
        assert_eq!(e.mem.u32(dest + 0x20), 3);
        assert_eq!(e.mem.u32(dest + 0x2c), 0xe2);
        let other = e.mem.alloc(0x40);
        let end = e
            .call(0x0064_e480, &args![source, first, first + 0x20, other])
            .u32();
        assert_eq!(end, other + 0x20);
        assert_eq!(e.mem.u32(other + 0x10), 2);
        let value = element_value(&mut e, 9);
        let end = e
            .call(0x0064_e340, &args![source, other, 3u32, value])
            .u32();
        assert_eq!(end, other + 0x30);
        assert_eq!(e.mem.u32(other + 0x20), 9);
        e.call(0x0064_e3f0, &args![other, other + 0x20, 0x5008u32]);
        assert_eq!(
            calls(&e, DESTROY_ELEMENT_0065E9A0),
            vec![vec![0x5008, other], vec![0x5008, other + 0x10]]
        );
        // The placement construction skips a null place.
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        assert_eq!(e.call(0x0064_e8c0, &args![other, value]).u32(), 0);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// `count` 16-byte elements whose words are `base + 4 * index`.
    fn block(e: &mut Engine, count: u32, base: u32) -> u32 {
        let at = e.mem.alloc(0x10 * count);
        for word in 0..4 * count {
            e.mem.set_u32(at + 4 * word, base + word);
        }
        at
    }

    /// The tags answer 0x100 (byte 0), the category writes 7, the unchecked pointer reads
    /// its slot.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(POINTER_CATEGORY_0065E750, |_, _| 0x100u32.into_ret());
        e.register(ITERATOR_CATEGORY_0065FD70, |e, a| {
            e.mem.set_u8(a[0], 7);
            a[0].into_ret()
        });
        e.register(UNCHECKED_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    #[test]
    fn fill_0064e4b0_fills_through_the_unchecked_pointers() {
        let mut e = rig();
        let range = block(&mut e, 3, 0);
        let value = block(&mut e, 1, 0x70);
        e.call_log = Some(vec![]);
        e.call(0x0064_e4b0, &args![range, range + 0x20, value]);
        let unchecked = calls(&e, UNCHECKED_0065FE40);
        assert_eq!(unchecked.len(), 2);
        // `last` first, then `first`, each through its argument slot.
        assert_eq!(e.mem.u32(unchecked[0][0]), range + 0x20);
        assert_eq!(e.mem.u32(unchecked[1][0]), range);
        for element in 0..2 {
            for word in 0..4 {
                assert_eq!(e.mem.u32(range + 0x10 * element + 4 * word), 0x70 + word);
            }
        }
        assert_eq!(e.mem.u32(range + 0x20), 8);
    }

    #[test]
    fn fill_loop_0064e680_copies_the_value_until_last() {
        let mut e = rig();
        let range = block(&mut e, 2, 0);
        let value = block(&mut e, 1, 0x50);
        e.call(0x0064_e680, &args![range, range + 0x20, value]);
        assert_eq!(e.mem.u32(range + 0x1c), 0x53);
        // An empty range writes nothing.
        let other = block(&mut e, 1, 0x90);
        e.call(0x0064_e680, &args![range, range, other]);
        assert_eq!(e.mem.u32(range), 0x50);
    }

    #[test]
    fn copy_backward_0064e4e0_moves_the_range_up() {
        let mut e = rig();
        // Elements 0..2 move up by one element (overlapping).
        let range = block(&mut e, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x0064_e4e0, &args![range, range + 0x20, range + 0x30]);
        let expected = [0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7];
        for (word, value) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(range + 4 * word as u32), *value);
        }
        // The tags: ECX and the one push the destination's slot, then (in the dispatch)
        // ECX the destination's slot with the first's and the destination's pushed.
        let tags = calls(&e, POINTER_CATEGORY_0065E750);
        assert_eq!(tags[0].len(), 3);
        assert_eq!(e.mem.u32(tags[0][0]), range + 0x30);
        assert_eq!(tags[0][1], tags[0][0]);
        assert_eq!(e.mem.u32(tags[1][1]), range);
        assert_eq!(calls(&e, ITERATOR_CATEGORY_0065FD70).len(), 1);
    }

    #[test]
    fn copy_backward_dispatch_0064e6c0_passes_the_category_byte() {
        let mut e = rig();
        let range = block(&mut e, 2, 0x20);
        let dest = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(
            0x0064_e6c0,
            &args![range, range + 0x20, dest + 0x20, 0x1207u32, 9u32, 9u32],
        );
        assert_eq!(e.mem.u32(dest + 0x1c), 0x27);
        let tags = calls(&e, POINTER_CATEGORY_0065E750);
        assert_eq!(e.mem.u32(tags[0][0]), dest + 0x20);
        assert_eq!(e.mem.u32(tags[0][1]), range);
        assert_eq!(e.mem.u32(tags[0][2]), dest + 0x20);
    }

    #[test]
    fn copy_backward_0064e860_answers_the_start_and_0064e980_copies() {
        let mut e = rig();
        let range = block(&mut e, 2, 0x40);
        let dest = e.mem.alloc(0x40);
        let start = e
            .call(
                0x0064_e860,
                &args![range, range + 0x20, dest + 0x40, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(start, dest + 0x20);
        assert_eq!(e.mem.u32(dest + 0x20), 0x40);
        assert_eq!(e.mem.u32(dest + 0x3c), 0x47);
        // The loop alone answers the destination's start; an empty range answers dest.
        let other = e.mem.alloc(0x10);
        let answer = e
            .call(
                0x0064_e980,
                &args![range, range + 0x10, other + 0x10, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(answer, other);
        assert_eq!(e.mem.u32(other + 8), 0x42);
        let answer = e
            .call(0x0064_e980, &args![range, range, other, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(answer, other);
    }
}

#[cfg(test)]
mod pointer_tag_tests {
    use super::*;

    /// The tag's words per call: the tags answer 0x100, the category writes 7, the unchecked
    /// pointer reads its slot.
    fn tag_calls(run: impl FnOnce(&mut Engine)) -> (Engine, Vec<Vec<u32>>) {
        let mut e = Engine::new();
        e.register(POINTER_CATEGORY_0065E750, |_, _| 0x100u32.into_ret());
        e.register(ITERATOR_CATEGORY_0065FD70, |e, a| {
            e.mem.set_u8(a[0], 7);
            a[0].into_ret()
        });
        e.register(UNCHECKED_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        e.call_log = Some(vec![]);
        run(&mut e);
        let tags = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == POINTER_CATEGORY_0065E750)
            .map(|(_, w)| w.clone())
            .collect();
        (e, tags)
    }

    #[test]
    fn the_tag_gets_ecx_and_the_pushed_words() {
        // copy (0064e380): ECX the dest word's address, pushes first and dest.
        let (e, tags) = tag_calls(|e| {
            e.call(0x0064_e380, &args![0x4000u32, 0x4000u32, 0x5000u32]);
        });
        assert_eq!(tags.len(), 1);
        assert_eq!(e.mem.u32(tags[0][0]), 0x5000);
        assert_eq!(e.mem.u32(tags[0][1]), 0x4000);
        assert_eq!(tags[0][2], tags[0][0]);
        // destroy (0064e3f0): ECX the first word's address, pushes first and last.
        let (e, tags) = tag_calls(|e| {
            e.call(0x0064_e3f0, &args![0x4000u32, 0x4000u32, 0x5008u32]);
        });
        assert_eq!(tags[0][0], tags[0][1]);
        assert_eq!(e.mem.u32(tags[0][1]), 0x4000);
        assert_eq!(tags[0][2], tags[0][1] + 4);
        // uninitialized copy (0064e630): ECX and the one push the dest word's address, then
        // the local word (its byte EBP-1 cleared).
        let (e, tags) = tag_calls(|e| {
            e.call(
                0x0064_e630,
                &args![0x4000u32, 0x4000u32, 0x5000u32, 0x6000u32],
            );
        });
        assert_eq!(tags[0][0], tags[0][1]);
        assert_eq!(e.mem.u32(tags[0][0]), 0x5000);
        assert_eq!(tags[0][2] >> 24, 0);
        // checked uninitialized copy (0064e920): as copy.
        let (e, tags) = tag_calls(|e| {
            e.call(
                0x0064_e920,
                &args![0x4000u32, 0x4000u32, 0x5000u32, 0x6000u32],
            );
        });
        assert_eq!(e.mem.u32(tags[0][0]), 0x5000);
        assert_eq!(e.mem.u32(tags[0][1]), 0x4000);
        assert_eq!(tags[0][2], tags[0][0]);
    }
}
