//! `SpeedTree/run_00b156b0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `std::vector` instantiations of the tree code, identified from the code (the engine map gives
//! them no names): the single-element insert and `_Insert_n` of a vector of 4-byte elements (a
//! second instance of the int vector's, with its own allocator), the `_Insert_n` of the
//! 0x18-byte flare vector (`CIdvBranch::m_vFlares` elements, six words), the fill of the
//! 4-byte vector's tail, and the checked iterator's `!=` and its `+=` for 0x18-byte
//! elements.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{checked_iterator, StlVector, INVALID_PARAMETER, OPERATOR_DELETE};
use super::run_00b0eca0::advance_checked_iterator;

/// The "vector<T> too long" throw (`00b0fb80`, no arguments).
const VECTOR_TOO_LONG: u32 = 0x00b0_fb80;
/// The allocation of the 4-byte vector (`00b16b00`, `cdecl`: count and hint) and of the flare
/// vector (`00b23aa0`, `cdecl`: count and hint).
const ALLOCATE_WORDS: u32 = 0x00b1_6b00;
const ALLOCATE_FLARES: u32 = 0x00b2_3aa0;
/// The 4-byte vector's helpers outside this unit: the forward copy (`00b177e0`: first, last,
/// dest, allocator), the fill (`00b36b10`: dest, count, value pointer) and the backward copy
/// (`00b16a60`: first, last, dest end).
const WORD_COPY: u32 = 0x00b1_77e0;
const WORD_FILL: u32 = 0x00b3_6b10;
const WORD_COPY_BACKWARD: u32 = 0x00b1_6a60;
/// The flare vector's helpers outside this unit: the forward copy (`00b17790`: first, last,
/// dest, allocator), the fill (`00b16cd0`: dest, count, value pointer, allocator and two tag
/// words) and the copy-construct of the tail (`00b16e40`: first, last, dest, allocator) and the
/// backward copy (`00b16a10`: first, last, dest end).
const FLARE_COPY: u32 = 0x00b1_7790;
const FLARE_FILL: u32 = 0x00b1_6cd0;
const FLARE_COPY_CONSTRUCT: u32 = 0x00b1_6e40;
const FLARE_COPY_BACKWARD: u32 = 0x00b1_6a10;
/// The checked iterator's `operator+=` for 4-byte elements (`00b2e330`, `thiscall`).
const ADVANCE_WORDS: u32 = 0x00b2_e330;
/// The largest element counts of the 4-byte vector and of the flare vector.
const WORD_MAX_COUNT: u32 = 0x3FFF_FFFF;
const FLARE_MAX_COUNT: u32 = 0x0AAA_AAAA;

/// The number of 4-byte elements between two pointers (signed shift).
fn words_between(first: u32, last: u32) -> u32 {
    (last.wrapping_sub(first) as i32 >> 2) as u32
}

/// The number of 0x18-byte elements between two pointers (signed division).
fn flares_between(first: u32, last: u32) -> u32 {
    (last.wrapping_sub(first) as i32 / 0x18) as u32
}

/// Copies one 0x18-byte element as six words.
fn copy_flare(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..6u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 00b156b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<int>::insert(where, value)` of the second 4-byte instance (identified from the
/// code; `thiscall`: the result iterator slot, the iterator's proxy and position and the value's
/// address): remembers the offset of `position` from the first element (0 for an empty vector;
/// the iterators are checked), inserts one copy of `*value` there (`_Insert_n`, `00b16070`,
/// which may reallocate), and writes to `out` the iterator for the same offset in the new
/// storage (an iterator at the first element advanced by the offset, `00b2e330`). Returns `out`.
pub fn fn_00b156b0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    let vector = this.addr();
    let offset = if words_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)) == 0 {
        0
    } else {
        let first = e.mem.u32(vector + 0x0C);
        let own_proxy = checked_iterator(e, vector, first);
        if proxy == 0 || proxy != own_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        words_between(first, position)
    };
    fn_00b16070(e, this, proxy, position, 1, value);
    let first = e.mem.u32(vector + 0x0C);
    let new_proxy = checked_iterator(e, vector, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), new_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        e.call(ADVANCE_WORDS, &args![iterator, offset]);
        let advanced_proxy = e.mem.u32(iterator.addr());
        let advanced = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), advanced_proxy);
        e.mem.set_u32(out.addr() + 4, advanced);
    });
    out
}

// Translated from 00b157e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b157e0 - the FS:[0] exception frame (unwinding of the new buffer if
// a copy throws) is not translated.
/// `std::vector<T>::_Insert_n(where, count, value)` for the 0x18-byte flare element (identified
/// from the code; the first argument is the iterator's proxy and is not read): nothing for a
/// count of 0; a count above the room left reports a length error (`00b0fb80`). Without room for
/// the new size a buffer of at least 1.5 times the capacity (`00b23aa0`) takes the head
/// (`00b17790`), `count` copies of `*value` (`00b16cd0`) and the tail (`00b17790`); the old
/// elements are walked (the destructor is empty) and the old buffer freed. With room, `value` is
/// first copied to six locals; a tail shorter than `count` is copied up (`00b17790`), the gap
/// beyond the old end filled with the local (`00b16cd0`) and `where..` overwritten with it;
/// otherwise the last `count` elements are copy-constructed at the end (`00b16e40`), the rest
/// moved up backwards (`00b16a10`) and `where..` overwritten.
pub fn fn_00b157e0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    let mut capacity = if first == 0 {
        0
    } else {
        flares_between(first, e.mem.u32(vector + 0x14))
    };
    if count == 0 {
        return;
    }
    let size = flares_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
    if FLARE_MAX_COUNT.wrapping_sub(size) < count {
        e.call(VECTOR_TOO_LONG, &args![]);
        return;
    }
    if capacity < size.wrapping_add(count) {
        capacity = if FLARE_MAX_COUNT.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let needed =
            flares_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)).wrapping_add(count);
        if capacity < needed {
            capacity = needed;
        }
        let buffer = e.call(ALLOCATE_FLARES, &args![capacity, 0u32]).u32();
        let first = e.mem.u32(vector + 0x0C);
        let mut end = e
            .call(FLARE_COPY, &args![first, position, buffer, vector + 8])
            .u32();
        // The byte after the allocator is an uninitialised stack byte in the exe; zero here.
        e.call(
            FLARE_FILL,
            &args![end, count, value, vector + 8, 0u32, 0u32],
        );
        end = count.wrapping_mul(0x18).wrapping_add(end);
        let last = e.mem.u32(vector + 0x10);
        e.call(FLARE_COPY, &args![position, last, end, vector + 8]);
        let new_size =
            flares_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)).wrapping_add(count);
        if e.mem.u32(vector + 0x0C) != 0 {
            let mut at = e.mem.u32(vector + 0x0C);
            while at != e.mem.u32(vector + 0x10) {
                at = at.wrapping_add(0x18);
            }
            let old_first = e.mem.u32(vector + 0x0C);
            e.call(OPERATOR_DELETE, &args![old_first]);
        }
        e.mem.set_u32(
            vector + 0x14,
            capacity.wrapping_mul(0x18).wrapping_add(buffer),
        );
        e.mem.set_u32(
            vector + 0x10,
            new_size.wrapping_mul(0x18).wrapping_add(buffer),
        );
        e.mem.set_u32(vector + 0x0C, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    let tail = flares_between(position, last);
    let span = count.wrapping_mul(0x18);
    e.with_stack(0x18, |e, local| {
        copy_flare(e, local.addr(), value.addr());
        if tail < count {
            e.call(
                FLARE_COPY,
                &args![position, last, position.wrapping_add(span), vector + 8],
            );
            let last = e.mem.u32(vector + 0x10);
            let tail = flares_between(position, last);
            e.call(
                FLARE_FILL,
                &args![
                    last,
                    count.wrapping_sub(tail),
                    local,
                    vector + 8,
                    0u32,
                    0u32
                ],
            );
            let grown = e.mem.u32(vector + 0x10).wrapping_add(span);
            e.mem.set_u32(vector + 0x10, grown);
            let end = e.mem.u32(vector + 0x10).wrapping_sub(span);
            let mut at = position;
            while at != end {
                copy_flare(e, at, local.addr());
                at = at.wrapping_add(0x18);
            }
        } else {
            let moved_from = last.wrapping_sub(span);
            let moved = e
                .call(
                    FLARE_COPY_CONSTRUCT,
                    &args![moved_from, last, e.mem.u32(vector + 0x10), vector + 8],
                )
                .u32();
            e.mem.set_u32(vector + 0x10, moved);
            e.call(FLARE_COPY_BACKWARD, &args![position, moved_from, last]);
            let end = position.wrapping_add(span);
            let mut at = position;
            while at != end {
                copy_flare(e, at, local.addr());
                at = at.wrapping_add(0x18);
            }
        }
    });
}

// Translated from 00b16070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b16070 - the FS:[0] exception frame (unwinding of the new buffer if
// a copy throws) is not translated.
/// `std::vector<int>::_Insert_n(where, count, value)` of the second 4-byte instance (identified
/// from the code; the first argument is the iterator's proxy and is not read): the algorithm of
/// `00b0f410` with the allocation `00b16b00` and `operator delete` for the old buffer. A count
/// above the room left reports a length error (`00b0fb80`); growth copies with `00b177e0` and
/// fills with `00b36b10`; a shorter tail fills the gap with `00b167e0`; otherwise the last
/// elements are copied to the end and the rest moved up with `00b16a60`.
pub fn fn_00b16070(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    let mut capacity = if first == 0 {
        0
    } else {
        words_between(first, e.mem.u32(vector + 0x14))
    };
    if count == 0 {
        return;
    }
    let size = words_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
    if WORD_MAX_COUNT.wrapping_sub(size) < count {
        e.call(VECTOR_TOO_LONG, &args![]);
        return;
    }
    if capacity < size.wrapping_add(count) {
        capacity = if WORD_MAX_COUNT.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let needed =
            words_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)).wrapping_add(count);
        if capacity < needed {
            capacity = needed;
        }
        let buffer = e.call(ALLOCATE_WORDS, &args![capacity, 0u32]).u32();
        let first = e.mem.u32(vector + 0x0C);
        let mut end = e
            .call(WORD_COPY, &args![first, position, buffer, vector + 8])
            .u32();
        e.call(WORD_FILL, &args![end, count, value]);
        end = end.wrapping_add(count.wrapping_mul(4));
        let last = e.mem.u32(vector + 0x10);
        e.call(WORD_COPY, &args![position, last, end, vector + 8]);
        let new_size =
            words_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)).wrapping_add(count);
        let old_first = e.mem.u32(vector + 0x0C);
        if old_first != 0 {
            e.call(OPERATOR_DELETE, &args![old_first]);
        }
        e.mem
            .set_u32(vector + 0x14, buffer.wrapping_add(capacity.wrapping_mul(4)));
        e.mem
            .set_u32(vector + 0x10, buffer.wrapping_add(new_size.wrapping_mul(4)));
        e.mem.set_u32(vector + 0x0C, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    let tail = words_between(position, last);
    if tail < count {
        let fill = e.mem.u32(value.addr());
        let span = count.wrapping_mul(4);
        e.call(
            WORD_COPY,
            &args![position, last, position.wrapping_add(span), vector + 8],
        );
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), fill);
            let last = e.mem.u32(vector + 0x10);
            let tail = words_between(position, last);
            fn_00b167e0(e, this.cast(), last, count.wrapping_sub(tail), local);
        });
        let grown = e.mem.u32(vector + 0x10).wrapping_add(span);
        e.mem.set_u32(vector + 0x10, grown);
        let end = e.mem.u32(vector + 0x10).wrapping_sub(span);
        let mut at = position;
        while at != end {
            e.mem.set_u32(at, fill);
            at = at.wrapping_add(4);
        }
    } else {
        let fill = e.mem.u32(value.addr());
        let span = count.wrapping_mul(4);
        let moved_from = last.wrapping_sub(span);
        let moved = e
            .call(
                WORD_COPY,
                &args![moved_from, last, e.mem.u32(vector + 0x10), vector + 8],
            )
            .u32();
        e.mem.set_u32(vector + 0x10, moved);
        e.call(WORD_COPY_BACKWARD, &args![position, moved_from, last]);
        let end = position.wrapping_add(span);
        let mut at = position;
        while at != end {
            e.mem.set_u32(at, fill);
            at = at.wrapping_add(4);
        }
    }
}

// Translated from 00b167e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `count` 4-byte elements at `dest` with the one at `value` (`00b36b10`) and returns the
/// end, `dest + count * 4` (identified from the code: the `_Uninit_fill_n` of the 4-byte vector;
/// `thiscall`, `this` is not read).
pub fn fn_00b167e0(e: &mut Engine, _this: Ptr, dest: u32, count: u32, value: Ptr) -> u32 {
    e.call(WORD_FILL, &args![dest, count, value]);
    dest.wrapping_add(count.wrapping_mul(4))
}

// Translated from 00b16830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked vector iterator's `operator!=` (identified from the code): both must have the
/// same nonzero proxy, else the invalid-parameter path runs; true when the element pointers
/// differ.
pub fn fn_00b16830(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let proxy = e.mem.u32(this.addr());
    if proxy == 0 || proxy != e.mem.u32(other.addr()) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(this.addr() + 4) != e.mem.u32(other.addr() + 4)
}

// Translated from 00b16890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 0x18-byte elements (identified from the code; see
/// `00b104f0`): moves the element pointer by `n * 0x18` bytes; an iterator without a proxy, or a
/// target outside the container's `[first, last]`, takes the invalid-parameter path. Returns
/// `this`.
pub fn fn_00b16890(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count.wrapping_mul(0x18));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b156b0,
            fn_00b156b0(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b157e0, fn_00b157e0(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b16070, fn_00b16070(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b167e0, fn_00b167e0(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00b16830, fn_00b16830(Ptr, Ptr) -> bool),
        entry!(0x00b16890, fn_00b16890(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The proxy word every test vector carries at `+0`.
    const PROXY: u32 = 0x5000_0001;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
        e
    }

    fn panics(action: impl FnOnce()) -> bool {
        catch_unwind(AssertUnwindSafe(action)).is_err()
    }

    fn spy(e: &mut Engine, addr: u32, result: u32) -> Words {
        let log: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(result)
        });
        log
    }

    fn vector(e: &mut Engine, size: u32, count: u32, capacity: u32) -> (u32, u32) {
        let at = e.mem.alloc(0x18);
        let data = e.mem.alloc((size * capacity).max(1));
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + size * count);
        e.mem.set_u32(at + 0x14, data + size * capacity);
        (at, data)
    }

    fn words_of(e: &Engine, vector: u32, size: u32) -> Vec<u32> {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        (0..(last - first) / size)
            .map(|i| e.mem.u32(first + size * i))
            .collect()
    }

    /// The word vector's outside helpers with real behaviour.
    fn word_doubles(e: &mut Engine) -> Words {
        e.register(WORD_COPY, |e, a| {
            let n = (a[1].wrapping_sub(a[0])) / 4;
            let data = e.mem.bytes(a[0], n * 4);
            e.mem.write(a[2], &data);
            ret(a[2] + n * 4)
        });
        e.register(WORD_FILL, |e, a| {
            let value = e.mem.u32(a[2]);
            for i in 0..a[1] {
                e.mem.set_u32(a[0] + 4 * i, value);
            }
            Ret::default()
        });
        e.register(WORD_COPY_BACKWARD, |e, a| {
            let n = (a[1].wrapping_sub(a[0])) / 4;
            let data = e.mem.bytes(a[0], n * 4);
            e.mem.write(a[2] - n * 4, &data);
            ret(a[2] - n * 4)
        });
        e.register(ALLOCATE_WORDS, |e, a| ret(e.mem.alloc((4 * a[0]).max(1))));
        spy(e, OPERATOR_DELETE, 0)
    }

    fn insert_words(e: &mut Engine, v: u32, position: u32, count: u32, value: u32) {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, value);
        e.call(
            0x00b1_6070,
            &args![
                Ptr::<StlVector>::new(v),
                PROXY,
                position,
                count,
                Ptr::<()>::new(slot)
            ],
        );
    }

    fn word_vector(e: &mut Engine, values: &[u32], capacity: u32) -> (u32, u32) {
        let (v, data) = vector(e, 4, values.len() as u32, capacity);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *value);
        }
        (v, data)
    }

    #[test]
    fn word_insert_in_every_case() {
        let mut e = engine();
        let freed = word_doubles(&mut e);
        // Room and a long tail.
        let (v, data) = word_vector(&mut e, &[1, 2, 3, 4, 5], 10);
        insert_words(&mut e, v, data + 4, 2, 9);
        assert_eq!(words_of(&e, v, 4), [1, 9, 9, 2, 3, 4, 5]);
        // Room and a short tail: the gap is filled through `00b167e0`.
        let (v, data) = word_vector(&mut e, &[1, 2, 3], 10);
        insert_words(&mut e, v, data + 8, 3, 9);
        assert_eq!(words_of(&e, v, 4), [1, 2, 9, 9, 9, 3]);
        // Growth: the new size, the old block freed with `operator delete`.
        let (v, data) = word_vector(&mut e, &[1, 2, 3], 3);
        insert_words(&mut e, v, data + 4, 2, 9);
        assert_eq!(words_of(&e, v, 4), [1, 9, 9, 2, 3]);
        assert_eq!(*freed.borrow(), vec![vec![data]]);
        let first = e.mem.u32(v + 0x0C);
        assert_eq!(e.mem.u32(v + 0x14), first + 20);
        // An empty vector frees nothing; a count of 0 changes nothing.
        let (v2, _) = vector(&mut e, 4, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        freed.borrow_mut().clear();
        insert_words(&mut e, v2, 0, 2, 7);
        assert_eq!(words_of(&e, v2, 4), [7, 7]);
        assert!(freed.borrow().is_empty());
        insert_words(&mut e, v, first, 0, 7);
        assert_eq!(words_of(&e, v, 4), [1, 9, 9, 2, 3]);
    }

    #[test]
    fn word_insert_too_long_reports_the_error() {
        let mut e = engine();
        word_doubles(&mut e);
        let long = spy(&mut e, VECTOR_TOO_LONG, 0);
        let (v, data) = word_vector(&mut e, &[1, 2, 3], 10);
        insert_words(&mut e, v, data, 0x3FFF_FFFE, 7);
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(words_of(&e, v, 4), [1, 2, 3]);
    }

    #[test]
    fn single_word_insert_reports_the_offset_iterator() {
        let mut e = engine();
        word_doubles(&mut e);
        let advances: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = advances.clone();
        e.register_double(ADVANCE_WORDS, move |e, a| {
            sink.borrow_mut().push((e.mem.u32(a[0]), a[1]));
            let at = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, at + a[1] * 4);
            Ret::default()
        });
        let (v, data) = word_vector(&mut e, &[1, 2, 3, 4, 5], 8);
        let out = e.mem.alloc(8);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 42);
        let r = e.call(
            0x00b1_56b0,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                PROXY,
                data + 8,
                Ptr::<()>::new(value)
            ],
        );
        assert_eq!(r.u32(), out);
        assert_eq!(words_of(&e, v, 4), [1, 2, 42, 3, 4, 5]);
        assert_eq!(*advances.borrow(), vec![(PROXY, 2)]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, data + 8));
        // An empty vector: offset 0, whatever the proxy.
        let (v2, d2) = vector(&mut e, 4, 0, 8);
        advances.borrow_mut().clear();
        e.call(
            0x00b1_56b0,
            &args![
                Ptr::<StlVector>::new(v2),
                Ptr::<()>::new(out),
                0x77u32,
                d2,
                Ptr::<()>::new(value)
            ],
        );
        assert_eq!(*advances.borrow(), vec![(PROXY, 0)]);
        // A foreign or missing proxy is invalid.
        for proxy in [0u32, 0x1234] {
            assert!(panics(|| {
                e.call(
                    0x00b1_56b0,
                    &args![
                        Ptr::<StlVector>::new(v),
                        Ptr::<()>::new(out),
                        proxy,
                        data,
                        Ptr::<()>::new(value)
                    ],
                );
            }));
        }
    }

    #[test]
    fn tail_fill_returns_the_end() {
        let mut e = engine();
        let fills = spy(&mut e, WORD_FILL, 0);
        let r = e.call(
            0x00b1_67e0,
            &args![Ptr::<()>::NULL, 0x1000u32, 3u32, Ptr::<()>::new(0x2000)],
        );
        assert_eq!(r.u32(), 0x100C);
        assert_eq!(*fills.borrow(), vec![vec![0x1000, 3, 0x2000]]);
    }

    #[test]
    fn iterator_not_equal_and_advance() {
        let mut e = engine();
        let (v, data) = vector(&mut e, 0x18, 4, 4);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, v);
        let it = e.mem.alloc(8);
        e.mem.set_u32(it, proxy);
        e.mem.set_u32(it + 4, data);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, proxy);
        e.mem.set_u32(other + 4, data);
        let both = args![Ptr::<()>::new(it), Ptr::<()>::new(other)];
        assert!(!e.call(0x00b1_6830, &both).bool());
        assert_eq!(
            e.call(0x00b1_6890, &args![Ptr::<()>::new(it), 3u32]).u32(),
            it
        );
        assert_eq!(e.mem.u32(it + 4), data + 0x48);
        assert!(e.call(0x00b1_6830, &both).bool());
        // A different or missing proxy is invalid.
        e.mem.set_u32(other, 0);
        assert!(panics(|| {
            fn_00b16830(&mut e, Ptr::new(it), Ptr::new(other));
        }));
        assert!(panics(|| {
            fn_00b16890(&mut e, Ptr::new(it), 2);
        }));
    }

    /// The flare vector's outside helpers with real behaviour (six-word elements).
    fn flare_doubles(e: &mut Engine) -> Words {
        for address in [FLARE_COPY, FLARE_COPY_CONSTRUCT] {
            e.register(address, |e, a| {
                let n = (a[1].wrapping_sub(a[0])) / 0x18;
                let data = e.mem.bytes(a[0], n * 0x18);
                e.mem.write(a[2], &data);
                ret(a[2] + n * 0x18)
            });
        }
        e.register(FLARE_FILL, |e, a| {
            for i in 0..a[1] {
                let data = e.mem.bytes(a[2], 0x18);
                e.mem.write(a[0] + 0x18 * i, &data);
            }
            Ret::default()
        });
        e.register(FLARE_COPY_BACKWARD, |e, a| {
            let n = (a[1].wrapping_sub(a[0])) / 0x18;
            let data = e.mem.bytes(a[0], n * 0x18);
            e.mem.write(a[2] - n * 0x18, &data);
            ret(a[2] - n * 0x18)
        });
        e.register(ALLOCATE_FLARES, |e, a| {
            ret(e.mem.alloc((0x18 * a[0]).max(1)))
        });
        spy(e, OPERATOR_DELETE, 0)
    }

    fn flare_vector(e: &mut Engine, tags: &[u32], capacity: u32) -> (u32, u32) {
        let (v, data) = vector(e, 0x18, tags.len() as u32, capacity);
        for (i, tag) in tags.iter().enumerate() {
            for word in 0..6 {
                e.mem
                    .set_u32(data + 0x18 * i as u32 + 4 * word, *tag + word);
            }
        }
        (v, data)
    }

    fn insert_flares(e: &mut Engine, v: u32, position: u32, count: u32, tag: u32) {
        let value = e.mem.alloc(0x18);
        for word in 0..6 {
            e.mem.set_u32(value + 4 * word, tag + word);
        }
        e.call(
            0x00b1_57e0,
            &args![
                Ptr::<StlVector>::new(v),
                PROXY,
                position,
                count,
                Ptr::<()>::new(value)
            ],
        );
    }

    fn flare_tags(e: &Engine, v: u32) -> Vec<u32> {
        let (first, last) = (e.mem.u32(v + 0x0C), e.mem.u32(v + 0x10));
        (0..(last - first) / 0x18)
            .map(|i| e.mem.u32(first + 0x18 * i))
            .collect()
    }

    #[test]
    fn flare_insert_in_every_case() {
        let mut e = engine();
        let freed = flare_doubles(&mut e);
        // Room and a long tail.
        let (v, data) = flare_vector(&mut e, &[10, 20, 30, 40, 50], 10);
        insert_flares(&mut e, v, data + 0x18, 2, 90);
        assert_eq!(flare_tags(&e, v), [10, 90, 90, 20, 30, 40, 50]);
        // Every word of an inserted element is copied.
        assert_eq!(e.mem.u32(data + 0x18 + 0x14), 95);
        // Room and a short tail.
        let (v, data) = flare_vector(&mut e, &[10, 20, 30], 10);
        insert_flares(&mut e, v, data + 0x30, 3, 90);
        assert_eq!(flare_tags(&e, v), [10, 20, 90, 90, 90, 30]);
        assert_eq!(e.mem.u32(data + 0x30 + 0x14), 95);
        // Growth: the old block is freed.
        let (v, data) = flare_vector(&mut e, &[10, 20, 30], 3);
        insert_flares(&mut e, v, data + 0x18, 2, 90);
        assert_eq!(flare_tags(&e, v), [10, 90, 90, 20, 30]);
        assert_eq!(*freed.borrow(), vec![vec![data]]);
        let first = e.mem.u32(v + 0x0C);
        assert_eq!(e.mem.u32(v + 0x14), first + 5 * 0x18);
        // An empty vector frees nothing; nothing to insert changes nothing.
        let (v2, _) = vector(&mut e, 0x18, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        freed.borrow_mut().clear();
        insert_flares(&mut e, v2, 0, 2, 70);
        assert_eq!(flare_tags(&e, v2), [70, 70]);
        assert!(freed.borrow().is_empty());
        insert_flares(&mut e, v, first, 0, 70);
        assert_eq!(flare_tags(&e, v), [10, 90, 90, 20, 30]);
    }

    #[test]
    fn flare_insert_too_long_reports_the_error() {
        let mut e = engine();
        flare_doubles(&mut e);
        let long = spy(&mut e, VECTOR_TOO_LONG, 0);
        let (v, data) = flare_vector(&mut e, &[10, 20], 10);
        insert_flares(&mut e, v, data, 0x0AAA_AAA9, 70);
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(flare_tags(&e, v), [10, 20]);
    }
}
