//! `SpeedTree/run_00b0e1a0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! MSVC `std::vector` instantiations of the tree code, identified from the code (the engine map
//! gives them no names): for the vectors of bytes, ints and 0x1C-byte structs, the single-element
//! insert (`insert(where, value)`), the storage set-up (`_Buy`) and the tidy, plus the byte
//! vector's `resize` and range erase and a free of a vector's proxy block.
//!
//! The vector is the PC's 0x18-byte `std::vector` ([`StlVector`]): the proxy at `+0`, the
//! allocator tag at `+8`, first, last and end pointers at `+0x0C`, `+0x10` and `+0x14`. An
//! iterator is two words, a proxy and an element pointer.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{checked_iterator, StlVector, INVALID_PARAMETER};

/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the manager at `011f6238`, the block).
const DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The `_Insert_n(where, count, value)` of the byte vector (`00b0ecf0`), of the int vector
/// (`00b0f410`) and of the 0x1C-byte struct vector (`00b0fc00`); all `thiscall` on the vector
/// with (the iterator's proxy, the position, the count, the value's address).
const INSERT_BYTES: u32 = 0x00b0_ecf0;
const INSERT_INTS: u32 = 0x00b0_f410;
const INSERT_STRUCTS: u32 = 0x00b0_fc00;
/// The checked iterators' `operator+=` for 1-byte (`00b10670`), 4-byte (`00b105b0`) and
/// 0x1C-byte (`00b104f0`) elements (`thiscall` on the iterator, a count).
const ADVANCE_BYTES: u32 = 0x00b1_0670;
const ADVANCE_INTS: u32 = 0x00b1_05b0;
const ADVANCE_STRUCTS: u32 = 0x00b1_04f0;
/// The byte vector's range copy (`00b10730`, `cdecl`: first, last, dest) and the checked
/// iterator's `operator!=` (`00b10490`, `thiscall`: the other iterator; returns a byte).
const COPY_BYTES: u32 = 0x00b1_0730;
const ITERATOR_NOT_EQUAL: u32 = 0x00b1_0490;
/// The allocation of an int vector (`00b10830`) and of a 0x1C-byte struct vector (`00b108e0`),
/// both `cdecl` with a count and a hint, and the "vector<T> too long" throw (`00b0fb80`).
const ALLOCATE_INTS: u32 = 0x00b1_0830;
const ALLOCATE_STRUCTS: u32 = 0x00b1_08e0;
const VECTOR_TOO_LONG: u32 = 0x00b0_fb80;
/// The largest element counts of the int and struct vectors.
const INT_MAX_COUNT: u32 = 0x3FFF_FFFF;
const STRUCT_MAX_COUNT: u32 = 0x0924_9249;

/// The element counts and sizes the single-element insert differs in: how elements are
/// counted, its `_Insert_n` and its iterator advance.
struct ElementKind {
    count_between: fn(first: u32, last: u32) -> u32,
    insert: u32,
    advance: u32,
}

fn count_bytes(first: u32, last: u32) -> u32 {
    last.wrapping_sub(first)
}

fn count_ints(first: u32, last: u32) -> u32 {
    (last.wrapping_sub(first) as i32 >> 2) as u32
}

fn count_structs(first: u32, last: u32) -> u32 {
    (last.wrapping_sub(first) as i32 / 0x1C) as u32
}

const BYTES: ElementKind = ElementKind {
    count_between: count_bytes,
    insert: INSERT_BYTES,
    advance: ADVANCE_BYTES,
};
const INTS: ElementKind = ElementKind {
    count_between: count_ints,
    insert: INSERT_INTS,
    advance: ADVANCE_INTS,
};
const STRUCTS: ElementKind = ElementKind {
    count_between: count_structs,
    insert: INSERT_STRUCTS,
    advance: ADVANCE_STRUCTS,
};

/// `vector::insert(where, value)` of one element: remembers the offset of `position` from the
/// first element (0 for an empty vector; the iterators are checked), inserts one copy of `*value`
/// there (`_Insert_n`, which may reallocate), and writes to `out` the iterator for the same offset
/// in the new storage (an iterator at the first element advanced by the offset). Returns `out`.
fn insert_one(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
    kind: &ElementKind,
) -> Ptr {
    let vector = this.addr();
    let offset = if (kind.count_between)(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)) == 0 {
        0
    } else {
        let first = e.mem.u32(vector + 0x0C);
        let own_proxy = checked_iterator(e, vector, first);
        if proxy == 0 || proxy != own_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (kind.count_between)(first, position)
    };
    e.call(kind.insert, &args![this, proxy, position, 1u32, value]);
    let first = e.mem.u32(vector + 0x0C);
    let new_proxy = checked_iterator(e, vector, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), new_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        e.call(kind.advance, &args![iterator, offset]);
        let advanced_proxy = e.mem.u32(iterator.addr());
        let advanced = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), advanced_proxy);
        e.mem.set_u32(out.addr() + 4, advanced);
    });
    out
}

/// The storage set-up of a vector of ints or structs: first, last and end cleared, a count of 0
/// gives false; a count above `max` reports a length error (`00b0fb80`); otherwise the storage
/// (`allocate`, `cdecl` with the count and a hint) becomes first and last, `count * size` bytes
/// after it the end. Returns true after any nonzero count.
fn buy(
    e: &mut Engine,
    this: Ptr<StlVector>,
    count: u32,
    (max, size, allocate): (u32, u32, u32),
) -> bool {
    let vector = this.addr();
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
    if count == 0 {
        return false;
    }
    // `max > 0 ? max : 1` takes the maximum: the constant is never 0.
    if max < count {
        e.call(VECTOR_TOO_LONG, &args![]);
    } else {
        let storage = e.call(allocate, &args![count, 0u32]).u32();
        e.mem.set_u32(vector + 0x0C, storage);
        let first = e.mem.u32(vector + 0x0C);
        e.mem.set_u32(vector + 0x10, first);
        let end = e
            .mem
            .u32(vector + 0x0C)
            .wrapping_add(count.wrapping_mul(size));
        e.mem.set_u32(vector + 0x14, end);
    }
    true
}

// Translated from 00b0e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0e1a0 - the FS:[0] exception frame is not translated.
/// Frees the block the object's first word points at (identified from the code; `fastcall`:
/// `MemoryManager::Deallocate` of `*this`, the container proxy of a vector).
pub fn fn_00b0e1a0(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00b0e200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<char>::resize(new_size, value)` (identified from the code; the value is a byte
/// passed by value and handed on by address): a larger size inserts the missing bytes at the end
/// (`_Insert_n`, `00b0ecf0`); a smaller size erases the bytes from `first + new_size` to the end
/// (`00b0e510`); an equal size does nothing. The iterators are checked.
pub fn fn_00b0e200(e: &mut Engine, this: Ptr<StlVector>, new_size: u32, value: u8) {
    let vector = this.addr();
    let size = e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C));
    if size < new_size {
        let last = e.mem.u32(vector + 0x10);
        let proxy = checked_iterator(e, vector, last);
        e.with_stack(4, |e, slot| {
            e.mem.set_u8(slot.addr(), value);
            let missing = new_size.wrapping_sub(
                e.mem
                    .u32(vector + 0x10)
                    .wrapping_sub(e.mem.u32(vector + 0x0C)),
            );
            e.call(INSERT_BYTES, &args![this, proxy, last, missing, slot]);
        });
    } else if new_size
        < e.mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))
    {
        let last = e.mem.u32(vector + 0x10);
        let last_proxy = checked_iterator(e, vector, last);
        let first = e.mem.u32(vector + 0x0C);
        let first_proxy = checked_iterator(e, vector, first);
        e.with_stack(8, |e, iterator| {
            e.mem.set_u32(iterator.addr(), first_proxy);
            e.mem.set_u32(iterator.addr() + 4, first);
            e.call(ADVANCE_BYTES, &args![iterator, new_size]);
            let proxy = e.mem.u32(iterator.addr());
            let position = e.mem.u32(iterator.addr() + 4);
            e.with_stack(8, |e, result| {
                fn_00b0e510(e, this, result, proxy, position, last_proxy, last);
            });
        });
    }
}

// Translated from 00b0e3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<char>::insert(where, value)` (identified from the code; `thiscall`: the result
/// iterator slot, the iterator's proxy and position and the value's address): see
/// [`insert_one`], with `_Insert_n` `00b0ecf0` and the advance `00b10670`.
pub fn fn_00b0e3e0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(e, this, out, proxy, position, value, &BYTES)
}

// Translated from 00b0e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range erase of the byte vector (identified from the code; `thiscall`: a result slot, the
/// first iterator's proxy and position and the end iterator's proxy and position; the proxies
/// are not read): both positions must be inside `[first, last]` of the vector (the
/// invalid-parameter path otherwise); when the iterators differ (`00b10490`) the bytes from the
/// end position on are copied down to the first position (`00b10730`) and the last pointer
/// lowered. Writes the first iterator (the vector's proxy and the first position) to the
/// result slot and returns it.
pub fn fn_00b0e510(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_1: u32,
    first_position: u32,
    _unused_3: u32,
    last_position: u32,
) -> Ptr {
    let vector = this.addr();
    let first_proxy = checked_iterator(e, vector, first_position);
    let last_proxy = checked_iterator(e, vector, last_position);
    let moved = e.with_stack(0x10, |e, iterators| {
        // The first iterator at +0 and the end iterator at +8 (a proxy and a position each).
        e.mem.set_u32(iterators.addr(), first_proxy);
        e.mem.set_u32(iterators.addr() + 4, first_position);
        e.mem.set_u32(iterators.addr() + 8, last_proxy);
        e.mem.set_u32(iterators.addr() + 12, last_position);
        e.call(ITERATOR_NOT_EQUAL, &args![iterators, iterators.byte_add(8)])
            .bool()
    });
    if moved {
        let last = e.mem.u32(vector + 0x10);
        let new_last = e
            .call(COPY_BYTES, &args![last_position, last, first_position])
            .u32();
        e.mem.set_u32(vector + 0x10, new_last);
    }
    e.mem.set_u32(out.addr(), first_proxy);
    e.mem.set_u32(out.addr() + 4, first_position);
    out
}

// Translated from 00b0e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<int>::insert(where, value)` (identified from the code): see [`insert_one`], with
/// `_Insert_n` `00b0f410`, the advance `00b105b0` and element counts by `>> 2`.
pub fn fn_00b0e600(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(e, this, out, proxy, position, value, &INTS)
}

// Translated from 00b0e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The storage set-up (`_Buy(count)`) of the int vector (identified from the code): clears the
/// first, last and end pointers; false for a count of 0; a count of `0x40000000` or more reports
/// a length error (`00b0fb80`); otherwise allocates `count` ints (`00b10830`), sets first and
/// last to the block and the end `count * 4` bytes after it. Returns true.
pub fn fn_00b0e730(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    buy(e, this, count, (INT_MAX_COUNT, 4, ALLOCATE_INTS))
}

// Translated from 00b0e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of the int vector (identified from the code; `fastcall`): frees the storage when
/// there is some and clears the first, last and end pointers.
pub fn fn_00b0e7f0(e: &mut Engine, this: Ptr<StlVector>) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    if first != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, first]);
    }
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

// Translated from 00b0e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::insert(where, value)` for the 0x1C-byte element (identified from the code):
/// see [`insert_one`], with `_Insert_n` `00b0fc00`, the advance `00b104f0` and element counts by
/// division by 0x1C.
pub fn fn_00b0e880(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(e, this, out, proxy, position, value, &STRUCTS)
}

// Translated from 00b0e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The storage set-up (`_Buy(count)`) of the 0x1C-byte element vector (identified from the code):
/// the same as the int vector's (`00b0e730`) with at most 0x9249249 elements, the allocation
/// `00b108e0` and a `count * 0x1C` end. Returns true after a nonzero count.
pub fn fn_00b0e9c0(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    buy(e, this, count, (STRUCT_MAX_COUNT, 0x1C, ALLOCATE_STRUCTS))
}

// Translated from 00b0ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of the 0x1C-byte element vector (identified from the code; `fastcall`): when there is
/// storage, walks the elements (the element destructor is empty) and frees it; clears the first,
/// last and end pointers.
pub fn fn_00b0ea80(e: &mut Engine, this: Ptr<StlVector>) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    if first != 0 {
        let last = e.mem.u32(vector + 0x10);
        let mut element = e.mem.u32(vector + 0x0C);
        while element != last {
            element = element.wrapping_add(0x1C);
        }
        let storage = e.mem.u32(vector + 0x0C);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, storage]);
    }
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0e1a0, fn_00b0e1a0(Ptr)),
        entry!(0x00b0e200, fn_00b0e200(Ptr<StlVector>, u32, u8)),
        entry!(
            0x00b0e3e0,
            fn_00b0e3e0(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00b0e510,
            fn_00b0e510(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00b0e600,
            fn_00b0e600(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b0e730, fn_00b0e730(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b0e7f0, fn_00b0e7f0(Ptr<StlVector>)),
        entry!(
            0x00b0e880,
            fn_00b0e880(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b0e9c0, fn_00b0e9c0(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b0ea80, fn_00b0ea80(Ptr<StlVector>)),
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

    /// A double that records its argument words and returns `result`.
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

    #[test]
    fn proxy_block_is_freed() {
        let mut e = engine();
        let freed = spy(&mut e, DEALLOCATE, 0);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x3333);
        e.call(0x00b0_e1a0, &args![Ptr::<()>::new(object)]);
        assert_eq!(*freed.borrow(), vec![vec![MEMORY_MANAGER, 0x3333]]);
    }

    #[test]
    fn resize_grows_shrinks_or_keeps() {
        let mut e = engine();
        let inserted: Rc<RefCell<Vec<(Vec<u32>, u8)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = inserted.clone();
        e.register_double(INSERT_BYTES, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), e.mem.u8(a[4])));
            Ret::default()
        });
        e.register(ADVANCE_BYTES, |e, a| {
            let at = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, at + a[1]);
            Ret::default()
        });
        let erased = spy(&mut e, ITERATOR_NOT_EQUAL, 0);
        let (v, data) = vector(&mut e, 1, 4, 8);
        // Grow: the missing bytes are inserted at the end with the byte value.
        e.call(0x00b0_e200, &args![Ptr::<StlVector>::new(v), 7u32, 0x5Au8]);
        let seen = inserted.borrow().clone();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[..4], [v, PROXY, data + 4, 3]);
        assert_eq!(seen[0].1, 0x5A);
        // The same size: nothing.
        inserted.borrow_mut().clear();
        e.call(0x00b0_e200, &args![Ptr::<StlVector>::new(v), 4u32, 0u8]);
        assert!(inserted.borrow().is_empty() && erased.borrow().is_empty());
        // Shrink: the erase of the tail starts at first + new size.
        let tails = spy(&mut e, COPY_BYTES, data + 2);
        e.register_double(ITERATOR_NOT_EQUAL, |_, _| ret(1));
        e.call(0x00b0_e200, &args![Ptr::<StlVector>::new(v), 2u32, 0u8]);
        // The erase moved the (empty) tail: from the end position (the old last) to the last.
        assert_eq!(*tails.borrow(), vec![vec![data + 4, data + 4, data + 2]]);
        assert_eq!(e.mem.u32(v + 0x10), data + 2);
    }

    #[test]
    fn byte_erase_copies_the_tail_down() {
        let mut e = engine();
        e.register(ITERATOR_NOT_EQUAL, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)))
        });
        e.register(COPY_BYTES, |e, a| {
            let n = a[1] - a[0];
            let data = e.mem.bytes(a[0], n);
            e.mem.write(a[2], &data);
            ret(a[2] + n)
        });
        let (v, data) = vector(&mut e, 1, 6, 8);
        e.mem.write(data, b"ABCDEF");
        let out = e.mem.alloc(8);
        let r = e.call(
            0x00b0_e510,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                0u32,
                data + 1,
                0u32,
                data + 3
            ],
        );
        assert_eq!(r.u32(), out);
        assert_eq!(e.mem.bytes(data, 4), b"ADEF");
        assert_eq!(e.mem.u32(v + 0x10), data + 4);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, data + 1));
        // An empty range changes nothing.
        e.call(
            0x00b0_e510,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                0u32,
                data + 2,
                0u32,
                data + 2
            ],
        );
        assert_eq!(e.mem.u32(v + 0x10), data + 4);
        // Positions outside the vector are invalid.
        assert!(panics(|| {
            fn_00b0e510(&mut e, Ptr::new(v), Ptr::new(out), 0, data + 9, 0, data);
        }));
        assert!(panics(|| {
            fn_00b0e510(&mut e, Ptr::new(v), Ptr::new(out), 0, data, 0, data + 9);
        }));
    }

    #[test]
    fn single_insert_reports_the_offset_iterator() {
        for (addr, insert, advance, size) in [
            (0x00b0_e3e0u32, INSERT_BYTES, ADVANCE_BYTES, 1u32),
            (0x00b0_e600, INSERT_INTS, ADVANCE_INTS, 4),
            (0x00b0_e880, INSERT_STRUCTS, ADVANCE_STRUCTS, 0x1C),
        ] {
            let mut e = engine();
            let inserts = spy(&mut e, insert, 0);
            let advances: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
            let sink = advances.clone();
            e.register_double(advance, move |e, a| {
                sink.borrow_mut().push((e.mem.u32(a[0]), a[1]));
                let at = e.mem.u32(a[0] + 4);
                e.mem.set_u32(a[0] + 4, at + a[1] * size);
                Ret::default()
            });
            let (v, data) = vector(&mut e, size, 5, 8);
            let out = e.mem.alloc(8);
            let value = e.mem.alloc(0x1C);
            let call = |e: &mut Engine, position: u32| {
                e.call(
                    addr,
                    &args![
                        Ptr::<StlVector>::new(v),
                        Ptr::<()>::new(out),
                        PROXY,
                        position,
                        Ptr::<()>::new(value)
                    ],
                )
            };
            // Insertion before element 2: offset 2, the iterator at the first element advanced.
            let r = call(&mut e, data + 2 * size);
            assert_eq!(r.u32(), out);
            assert_eq!(
                *inserts.borrow(),
                vec![vec![v, PROXY, data + 2 * size, 1, value]]
            );
            assert_eq!(*advances.borrow(), vec![(PROXY, 2)]);
            assert_eq!(
                (e.mem.u32(out), e.mem.u32(out + 4)),
                (PROXY, data + 2 * size)
            );
            // An empty vector: offset 0, whatever the proxy.
            let (v2, d2) = vector(&mut e, size, 0, 8);
            advances.borrow_mut().clear();
            e.call(
                addr,
                &args![
                    Ptr::<StlVector>::new(v2),
                    Ptr::<()>::new(out),
                    0x77u32,
                    d2,
                    Ptr::<()>::new(value)
                ],
            );
            assert_eq!(*advances.borrow(), vec![(PROXY, 0)]);
            // A foreign or missing proxy, and a position outside the vector, are invalid.
            for proxy in [0u32, 0x1234] {
                assert!(panics(|| {
                    e.call(
                        addr,
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
    }

    #[test]
    fn storage_set_up_of_the_int_and_struct_vectors() {
        for (addr, allocate, size, largest) in [
            (0x00b0_e730u32, ALLOCATE_INTS, 4u32, 0x3FFF_FFFFu32),
            (0x00b0_e9c0, ALLOCATE_STRUCTS, 0x1C, 0x0924_9249),
        ] {
            let mut e = engine();
            let storage = e.mem.alloc(64);
            let allocated = spy(&mut e, allocate, storage);
            let long = spy(&mut e, VECTOR_TOO_LONG, 0);
            let v = e.mem.alloc(0x18);
            for off in [0x0C, 0x10, 0x14] {
                e.mem.set_u32(v + off, 0x99);
            }
            // A count of 0 clears the pointers and says false.
            assert!(!e.call(addr, &args![Ptr::<StlVector>::new(v), 0u32]).bool());
            assert_eq!(
                (
                    e.mem.u32(v + 0x0C),
                    e.mem.u32(v + 0x10),
                    e.mem.u32(v + 0x14)
                ),
                (0, 0, 0)
            );
            // A count of 3 allocates and sets first, last and end.
            assert!(e.call(addr, &args![Ptr::<StlVector>::new(v), 3u32]).bool());
            assert_eq!(*allocated.borrow(), vec![vec![3, 0]]);
            assert_eq!(
                (
                    e.mem.u32(v + 0x0C),
                    e.mem.u32(v + 0x10),
                    e.mem.u32(v + 0x14)
                ),
                (storage, storage, storage + 3 * size)
            );
            // The largest count is allowed; one more reports a length error and allocates
            // nothing (the double does not throw).
            allocated.borrow_mut().clear();
            assert!(e
                .call(addr, &args![Ptr::<StlVector>::new(v), largest])
                .bool());
            assert_eq!(allocated.borrow().len(), 1);
            assert!(long.borrow().is_empty());
            assert!(e
                .call(addr, &args![Ptr::<StlVector>::new(v), largest + 1])
                .bool());
            assert_eq!(allocated.borrow().len(), 1);
            assert_eq!(long.borrow().len(), 1);
            assert_eq!(e.mem.u32(v + 0x0C), 0);
        }
    }

    #[test]
    fn tidy_frees_the_storage() {
        for addr in [0x00b0_e7f0u32, 0x00b0_ea80] {
            let mut e = engine();
            let freed = spy(&mut e, DEALLOCATE, 0);
            let (v, data) = vector(&mut e, 0x1C, 3, 4);
            e.call(addr, &args![Ptr::<StlVector>::new(v)]);
            assert_eq!(*freed.borrow(), vec![vec![MEMORY_MANAGER, data]]);
            assert_eq!(
                (
                    e.mem.u32(v + 0x0C),
                    e.mem.u32(v + 0x10),
                    e.mem.u32(v + 0x14)
                ),
                (0, 0, 0)
            );
            // Without storage nothing is freed.
            freed.borrow_mut().clear();
            e.call(addr, &args![Ptr::<StlVector>::new(v)]);
            assert!(freed.borrow().is_empty());
        }
    }
}
