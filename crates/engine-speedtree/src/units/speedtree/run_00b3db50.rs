//! `SpeedTree/run_00b3db50` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `std::vector` instantiation of the 0x54-byte element (see `run_00b2e250`), identified from
//! the code (the engine map gives it no names): its tidy, `_Insert_n`, checked iterator `+=`,
//! the range copy-constructors, the backward assignment and the fill, and the constructor of the
//! element's smaller sibling class (three `CVec3`-like triples and a string).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{StlVector, OPERATOR_DELETE};
use super::run_00b0eca0::advance_checked_iterator;

/// The 0x54-byte element helpers of `run_00b2e250`: the allocation (`00b2e4a0`, `cdecl`: count
/// and hint), the assignment (`00b2e680`, `thiscall`: the other element), the copy construction
/// (`00b2e740`, `thiscall`: the other element), the destruction of a range (`00b2e140`,
/// `thiscall`: first, last) and the range copy-construct (`00b2e570`, `cdecl`: first, last,
/// dest and three words).
const ALLOCATE_ELEMENTS: u32 = 0x00b2_e4a0;
const ELEMENT_ASSIGN: u32 = 0x00b2_e680;
const ELEMENT_COPY_CONSTRUCT: u32 = 0x00b2_e740;
const DESTROY_ELEMENTS: u32 = 0x00b2_e140;
const UNINIT_COPY: u32 = 0x00b2_e570;
/// The "vector<T> too long" throw (`00b0fb80`) and the string's `_Tidy(built, new_size)`
/// (`0044a730`, `thiscall`).
const VECTOR_TOO_LONG: u32 = 0x00b0_fb80;
const STRING_TIDY: u32 = 0x0044_a730;
/// The largest element count of the vector (`0x30C30C3`, `0xFFFFFFFF / 0x54`).
const MAX_COUNT: u32 = 0x030C_30C3;

/// The number of 0x54-byte elements between two pointers (signed division).
fn elements_between(first: u32, last: u32) -> u32 {
    ((last.wrapping_sub(first)) as i32 / 0x54) as u32
}

/// The destruction of the copy-constructed local element: its string is tidied
/// (`0044a730` with built = 1, new size 0).
fn destroy_local(e: &mut Engine, local: u32) {
    e.call(STRING_TIDY, &args![local + 0x14, 1u32, 0u32]);
}

// Translated from 00b3db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of the 0x54-byte element vector (identified from the code; `fastcall`): when there is
/// storage, destroys the elements (`00b2e140`) and frees it with `operator delete`; clears the
/// first, last and end pointers.
pub fn fn_00b3db50(e: &mut Engine, this: Ptr<StlVector>) {
    let vector = this.addr();
    if e.mem.u32(vector + 0x0C) != 0 {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        e.call(DESTROY_ELEMENTS, &args![this, first, last]);
        let storage = e.mem.u32(vector + 0x0C);
        e.call(OPERATOR_DELETE, &args![storage]);
    }
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

// Translated from 00b3dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3dbd0 - the FS:[0] exception frame (destroying the local element
// and the new buffer if a copy throws) and the /GS cookie check (`00ec408c`) are not translated.
/// `std::vector<T>::_Insert_n(where, count, value)` for the 0x54-byte element (identified from
/// the code; the first argument is the iterator's proxy and is not read): nothing for a count
/// of 0; a count above the room left reports a length error (`00b0fb80`). Without room for the
/// new size a buffer of at least 1.5 times the capacity (`00b2e4a0`) takes the head
/// (`00b3e450`), `count` copies of `*value` (`00b3e5b0`) and the tail (`00b3e4a0`); the old
/// elements are destroyed (`00b2e140`) and the old buffer freed. With room, `value` is first
/// copy-constructed to a local (`00b2e740`); a tail shorter than `count` is copied up
/// (`00b3e450`), the gap beyond the old end filled with the local (`00b3e5b0`) and
/// `where..` assigned the local (`00b2e680`); otherwise the last `count` elements are
/// copy-constructed at the end (`00b3e4a0`), the rest assigned up backwards (`00b3e520`) and
/// `where..` assigned the local. The local's string is tidied last.
pub fn fn_00b3dbd0(
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
        elements_between(first, e.mem.u32(vector + 0x14))
    };
    if count == 0 {
        return;
    }
    let size = elements_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
    if MAX_COUNT.wrapping_sub(size) < count {
        e.call(VECTOR_TOO_LONG, &args![]);
        return;
    }
    if capacity < size.wrapping_add(count) {
        capacity = if MAX_COUNT.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let needed = elements_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10))
            .wrapping_add(count);
        if capacity < needed {
            capacity = needed;
        }
        let buffer = e.call(ALLOCATE_ELEMENTS, &args![capacity, 0u32]).u32();
        let first = e.mem.u32(vector + 0x0C);
        let mut end = fn_00b3e450(e, first, position, buffer, vector + 8);
        // The byte after the allocator is an uninitialised stack byte in the exe; zero here.
        fn_00b3e5b0(e, end, count, value, vector + 8, 0, 0);
        end = count.wrapping_mul(0x54).wrapping_add(end);
        let last = e.mem.u32(vector + 0x10);
        fn_00b3e4a0(e, position, last, end, vector + 8);
        let new_size = elements_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10))
            .wrapping_add(count);
        if e.mem.u32(vector + 0x0C) != 0 {
            let (old_first, old_last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
            e.call(DESTROY_ELEMENTS, &args![this, old_first, old_last]);
            e.call(OPERATOR_DELETE, &args![old_first]);
        }
        e.mem.set_u32(
            vector + 0x14,
            capacity.wrapping_mul(0x54).wrapping_add(buffer),
        );
        e.mem.set_u32(
            vector + 0x10,
            new_size.wrapping_mul(0x54).wrapping_add(buffer),
        );
        e.mem.set_u32(vector + 0x0C, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    let tail = elements_between(position, last);
    let span = count.wrapping_mul(0x54);
    e.with_stack(0x54, |e, local| {
        e.call(ELEMENT_COPY_CONSTRUCT, &args![local, value]);
        if tail < count {
            fn_00b3e450(e, position, last, position.wrapping_add(span), vector + 8);
            let last = e.mem.u32(vector + 0x10);
            let tail = elements_between(position, last);
            fn_00b3e5b0(e, last, count.wrapping_sub(tail), local, vector + 8, 0, 0);
            let grown = e.mem.u32(vector + 0x10).wrapping_add(span);
            e.mem.set_u32(vector + 0x10, grown);
            let end = e.mem.u32(vector + 0x10).wrapping_sub(span);
            let mut at = position;
            while at != end {
                e.call(ELEMENT_ASSIGN, &args![at, local]);
                at = at.wrapping_add(0x54);
            }
        } else {
            let moved_from = last.wrapping_sub(span);
            // The two tag bytes after the allocator: an uninitialised stack byte and a zero.
            let moved = fn_00b3e4a0(e, moved_from, last, e.mem.u32(vector + 0x10), vector + 8);
            e.mem.set_u32(vector + 0x10, moved);
            fn_00b3e520(e, position, moved_from, last, 0, 0, 0);
            let end = position.wrapping_add(span);
            let mut at = position;
            while at != end {
                e.call(ELEMENT_ASSIGN, &args![at, local]);
                at = at.wrapping_add(0x54);
            }
        }
        destroy_local(e, local.addr());
    });
}

// Translated from 00b3e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 0x54-byte elements (identified from the code; see
/// `00b104f0`): moves the element pointer by `n * 0x54` bytes; an iterator without a proxy, or a
/// target outside the container's `[first, last]`, takes the invalid-parameter path. Returns
/// `this`.
pub fn fn_00b3e390(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count.wrapping_mul(0x54));
    this
}

// Translated from 00b3e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a range copy-construct of 0x54-byte elements to `00b2e570` (identified from the code;
/// `cdecl`, `first`, `last`, `dest`, the allocator) with two tag bytes, uninitialised stack
/// bytes in the exe and zero here. Returns its result, the end of the copy.
pub fn fn_00b3e450(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    e.call(
        UNINIT_COPY,
        &args![first, last, dest, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b3e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same forwarder as `00b3e450`, built through one more layer of inlined helpers
/// (identified from the code; `cdecl`, `first`, `last`, `dest`, the allocator): the range
/// copy-construct `00b2e570`. Returns its result.
pub fn fn_00b3e4a0(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    e.call(
        UNINIT_COPY,
        &args![first, last, dest, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b3e520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward assignment of 0x54-byte elements (identified from the code; `cdecl`, `first`,
/// `last`, `dest_end`, three tag bytes not read): assigns the elements from the last one down to
/// the first (`00b2e680`) so that the range ends at `dest_end`. Returns the start of the
/// destination, `dest_end` minus the whole elements' size.
pub fn fn_00b3e520(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = dest_end.wrapping_sub(elements_between(first, last).wrapping_mul(0x54));
    let mut source = last;
    let mut dest = dest_end;
    while first != source {
        source = source.wrapping_sub(0x54);
        dest = dest.wrapping_sub(0x54);
        e.call(ELEMENT_ASSIGN, &args![dest, source]);
    }
    start
}

// Translated from 00b3e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3e5b0 - the FS:[0] exception frame is not translated.
/// `std::_Uninit_fill_n` for the 0x54-byte element (identified from the code; `cdecl`, `dest`,
/// `count`, `value`, the allocator and two tag bytes not read): copy-constructs `*value`
/// (`00b2e740`) at `count` places from `dest`, 0x54 bytes apart; a null destination is
/// skipped.
pub fn fn_00b3e5b0(
    e: &mut Engine,
    dest: u32,
    count: u32,
    value: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut dest = dest;
    let mut count = count;
    while count != 0 {
        if dest != 0 {
            e.call(ELEMENT_COPY_CONSTRUCT, &args![dest, value]);
        }
        count = count.wrapping_sub(1);
        dest = dest.wrapping_add(0x54);
    }
}

// Translated from 00b3e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a small class (identified from the code; class unconfirmed; `fastcall`):
/// three triples of floats at `+0`, `+0xC` and `+0x18` cleared (each stored `+8`, `+4`, `+0`)
/// and the string at `+0x24` set to the empty string (`0044a730` with 0 and 0). Returns
/// `this`.
pub fn fn_00b3e6c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for triple in [0u32, 0x0C, 0x18] {
        for offset in [8u32, 4, 0] {
            e.mem.set_f32(base + triple + offset, 0.0);
        }
    }
    e.call(STRING_TIDY, &args![base + 0x24, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b3db50, fn_00b3db50(Ptr<StlVector>)),
        entry!(0x00b3dbd0, fn_00b3dbd0(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b3e390, fn_00b3e390(Ptr, u32) -> Ptr),
        entry!(0x00b3e450, fn_00b3e450(u32, u32, u32, u32) -> u32),
        entry!(0x00b3e4a0, fn_00b3e4a0(u32, u32, u32, u32) -> u32),
        entry!(0x00b3e520, fn_00b3e520(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b3e5b0, fn_00b3e5b0(u32, u32, Ptr, u32, u32, u32)),
        entry!(0x00b3e6c0, fn_00b3e6c0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::speedtree::branch::INVALID_PARAMETER;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

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

    fn fill_element(e: &mut Engine, at: u32, tag: u32) {
        e.mem.set_u8(at, tag as u8);
        for word in 1..0x15 {
            e.mem.set_u32(at + 4 * word, tag);
        }
    }

    fn tags(e: &Engine, vector: u32) -> Vec<u8> {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        (0..(last - first) / 0x54)
            .map(|i| e.mem.u8(first + 0x54 * i))
            .collect()
    }

    /// Doubles for the element helpers: assignment and copy construction copy the 0x54 bytes
    /// (logged), the range copy-construct copies element by element, allocation hands out heap
    /// blocks, deletion and destruction are logged.
    fn element_doubles(e: &mut Engine) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in [ELEMENT_ASSIGN, ELEMENT_COPY_CONSTRUCT] {
            let sink = log.clone();
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                let data = e.mem.bytes(a[1], 0x54);
                e.mem.write(a[0], &data);
                ret(a[0])
            });
        }
        let sink = log.clone();
        e.register_double(UNINIT_COPY, move |e, a| {
            sink.borrow_mut().push((UNINIT_COPY, a.to_vec()));
            let n = (a[1] - a[0]) / 0x54;
            for i in 0..n {
                let data = e.mem.bytes(a[0] + 0x54 * i, 0x54);
                e.mem.write(a[2] + 0x54 * i, &data);
            }
            ret(a[2] + 0x54 * n)
        });
        e.register(ALLOCATE_ELEMENTS, |e, a| {
            ret(e.mem.alloc((a[0] * 0x54).max(1)))
        });
        for address in [DESTROY_ELEMENTS, OPERATOR_DELETE, STRING_TIDY] {
            let sink = log.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn insert(e: &mut Engine, v: u32, position: u32, count: u32, tag: u32) -> u32 {
        let value = e.mem.alloc(0x54);
        fill_element(e, value, tag);
        e.call(
            0x00b3_dbd0,
            &args![
                Ptr::<StlVector>::new(v),
                PROXY,
                position,
                count,
                Ptr::<()>::new(value)
            ],
        );
        value
    }

    fn filled_vector(e: &mut Engine, tags: &[u32], capacity: u32) -> (u32, u32) {
        let (v, data) = vector(e, 0x54, tags.len() as u32, capacity);
        for (i, tag) in tags.iter().enumerate() {
            fill_element(e, data + 0x54 * i as u32, *tag);
        }
        (v, data)
    }

    #[test]
    fn tidy_destroys_and_frees() {
        let mut e = engine();
        let log = element_doubles(&mut e);
        let (v, data) = filled_vector(&mut e, &[1, 2], 4);
        e.call(0x00b3_db50, &args![Ptr::<StlVector>::new(v)]);
        assert_eq!(
            *log.borrow(),
            vec![
                (DESTROY_ELEMENTS, vec![v, data, data + 0xA8]),
                (OPERATOR_DELETE, vec![data])
            ]
        );
        assert_eq!(
            (
                e.mem.u32(v + 0x0C),
                e.mem.u32(v + 0x10),
                e.mem.u32(v + 0x14)
            ),
            (0, 0, 0)
        );
        log.borrow_mut().clear();
        e.call(0x00b3_db50, &args![Ptr::<StlVector>::new(v)]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn iterator_advance_by_the_element_size() {
        let mut e = engine();
        let (v, data) = vector(&mut e, 0x54, 3, 3);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, v);
        let it = e.mem.alloc(8);
        e.mem.set_u32(it, proxy);
        e.mem.set_u32(it + 4, data);
        assert_eq!(
            e.call(0x00b3_e390, &args![Ptr::<()>::new(it), 3u32]).u32(),
            it
        );
        assert_eq!(e.mem.u32(it + 4), data + 0xFC);
        assert!(panics(|| {
            fn_00b3e390(&mut e, Ptr::new(it), 1);
        }));
    }

    #[test]
    fn forwarders_pass_the_allocator_and_zero_tags() {
        let mut e = engine();
        let copies = spy(&mut e, UNINIT_COPY, 0x4242);
        for addr in [0x00b3_e450u32, 0x00b3_e4a0] {
            assert_eq!(e.call(addr, &args![1u32, 2u32, 3u32, 4u32]).u32(), 0x4242);
        }
        assert_eq!(
            *copies.borrow(),
            vec![vec![1, 2, 3, 4, 0, 0], vec![1, 2, 3, 4, 0, 0]]
        );
    }

    #[test]
    fn backward_assignment_and_fill() {
        let mut e = engine();
        let log = element_doubles(&mut e);
        let (_, data) = filled_vector(&mut e, &[1, 2, 3], 4);
        // Backward: the three elements move up by one place (from the last one down).
        let r = e.call(
            0x00b3_e520,
            &args![data, data + 0xFC, data + 0x150, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), data + 0x54);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u8(data + 0x54 * i))
                .collect::<Vec<_>>(),
            [1, 1, 2, 3]
        );
        let order: Vec<u32> = log.borrow().iter().map(|(_, a)| a[0]).collect();
        assert_eq!(
            order,
            vec![
                data + 0x150 - 0x54,
                data + 0x150 - 0xA8,
                data + 0x150 - 0xFC
            ]
        );
        // The fill copy-constructs the value at each place and skips a null destination.
        log.borrow_mut().clear();
        let value = e.mem.alloc(0x54);
        fill_element(&mut e, value, 9);
        let dest = e.mem.alloc(0x54 * 2);
        e.call(
            0x00b3_e5b0,
            &args![dest, 2u32, Ptr::<()>::new(value), 0u32, 0u32, 0u32],
        );
        assert_eq!(
            *log.borrow(),
            vec![
                (ELEMENT_COPY_CONSTRUCT, vec![dest, value]),
                (ELEMENT_COPY_CONSTRUCT, vec![dest + 0x54, value])
            ]
        );
        log.borrow_mut().clear();
        e.call(
            0x00b3_e5b0,
            &args![0u32, 1u32, Ptr::<()>::new(value), 0u32, 0u32, 0u32],
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn small_class_constructor() {
        let mut e = engine();
        let log = element_doubles(&mut e);
        let at = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
        }
        let r = e.call(0x00b3_e6c0, &args![Ptr::<()>::new(at)]);
        assert_eq!(r.u32(), at);
        for offset in [0u32, 4, 8, 0x0C, 0x10, 0x14, 0x18, 0x1C, 0x20] {
            assert_eq!(e.mem.f32(at + offset), 0.0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(at + 0x24), 0xDEAD_BEEF);
        assert_eq!(*log.borrow(), vec![(STRING_TIDY, vec![at + 0x24, 0, 0])]);
    }

    #[test]
    fn insert_with_room_and_a_long_tail() {
        let mut e = engine();
        let log = element_doubles(&mut e);
        let (v, data) = filled_vector(&mut e, &[1, 2, 3, 4, 5], 10);
        insert(&mut e, v, data + 0x54, 2, 9);
        assert_eq!(tags(&e, v), [1, 9, 9, 2, 3, 4, 5]);
        // The local copy's string is tidied last (built = 1, new size 0).
        let calls = log.borrow().clone();
        let (address, words) = calls.last().unwrap();
        assert_eq!(*address, STRING_TIDY);
        assert_eq!(words[1..], [1, 0]);
        assert_ne!(words[0], 0);
    }

    #[test]
    fn insert_with_a_short_tail() {
        let mut e = engine();
        let log = element_doubles(&mut e);
        let (v, data) = filled_vector(&mut e, &[1, 2, 3], 10);
        insert(&mut e, v, data + 0xA8, 3, 9);
        assert_eq!(tags(&e, v), [1, 2, 9, 9, 9, 3]);
        // The gap beyond the old end is filled by copy construction (the exe passes two
        // zero tag words after the allocator; the copy-construct double ignores them).
        let constructs = log
            .borrow()
            .iter()
            .filter(|(a, _)| *a == ELEMENT_COPY_CONSTRUCT)
            .count();
        // One for the local, two for the gap.
        assert_eq!(constructs, 3);
    }

    #[test]
    fn insert_grows_the_buffer() {
        let mut e = engine();
        let log = element_doubles(&mut e);
        let (v, data) = filled_vector(&mut e, &[1, 2, 3], 3);
        insert(&mut e, v, data + 0x54, 2, 9);
        assert_eq!(tags(&e, v), [1, 9, 9, 2, 3]);
        let calls = log.borrow().clone();
        // The old elements are destroyed and the old buffer deleted.
        assert!(calls.contains(&(DESTROY_ELEMENTS, vec![v, data, data + 0xFC])));
        assert!(calls.contains(&(OPERATOR_DELETE, vec![data])));
        let first = e.mem.u32(v + 0x0C);
        assert_ne!(first, data);
        assert_eq!(e.mem.u32(v + 0x14), first + 5 * 0x54);
        // An empty vector has no storage to destroy or free.
        let (v2, _) = vector(&mut e, 0x54, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        log.borrow_mut().clear();
        insert(&mut e, v2, 0, 2, 4);
        assert_eq!(tags(&e, v2), [4, 4]);
        assert!(log
            .borrow()
            .iter()
            .all(|(a, _)| *a != DESTROY_ELEMENTS && *a != OPERATOR_DELETE));
    }

    #[test]
    fn insert_of_nothing_and_too_long() {
        let mut e = engine();
        element_doubles(&mut e);
        let long = spy(&mut e, VECTOR_TOO_LONG, 0);
        let (v, data) = filled_vector(&mut e, &[1, 2], 10);
        insert(&mut e, v, data, 0, 9);
        assert!(long.borrow().is_empty());
        insert(&mut e, v, data, MAX_COUNT, 9);
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(tags(&e, v), [1, 2]);
    }
}
