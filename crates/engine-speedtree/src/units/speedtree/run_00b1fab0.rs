//! `SpeedTree/run_00b1fab0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `std::vector` instantiations of the tree code, identified from the code (the engine map gives
//! them no names): for a vector of 0xC-byte elements the single-element insert, the storage
//! set-up (`_Buy`) and the tidy, and for a vector of 0x18-byte elements (with an element
//! destructor) the single-element insert and the tidy.
//!
//! The vector is the PC's 0x18-byte `std::vector` ([`StlVector`]); an iterator is two words, a
//! proxy and an element pointer.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{checked_iterator, StlVector, INVALID_PARAMETER, OPERATOR_DELETE};

/// The "vector<T> too long" throw (`00b0fb80`).
const VECTOR_TOO_LONG: u32 = 0x00b0_fb80;
/// For the 0xC-byte vector: `_Insert_n` (`00b21e00`, `thiscall`: the iterator's proxy, the
/// position, the count, the value's address), the checked iterator's `+=` (`00b235e0`) and the
/// allocation (`00b23bb0`, `cdecl`: count and hint).
const INSERT_TRIPLES: u32 = 0x00b2_1e00;
const ADVANCE_TRIPLES: u32 = 0x00b2_35e0;
const ALLOCATE_TRIPLES: u32 = 0x00b2_3bb0;
/// For the 0x18-byte vector: `_Insert_n` (`00b22700`), the checked iterator's `+=`
/// (`00b16890`) and the destruction of a range (`00b22650`, `thiscall`: first, last).
const INSERT_RECORDS: u32 = 0x00b2_2700;
const ADVANCE_RECORDS: u32 = 0x00b1_6890;
const DESTROY_RECORDS: u32 = 0x00b2_2650;
/// The largest element count of the 0xC-byte vector (`0xFFFFFFFF / 0xC`).
const TRIPLE_MAX_COUNT: u32 = 0x1555_5555;

/// What a single-element insert needs of its element type: its size and the callees.
struct ElementKind {
    size: i32,
    insert: u32,
    advance: u32,
}

const TRIPLES: ElementKind = ElementKind {
    size: 0x0C,
    insert: INSERT_TRIPLES,
    advance: ADVANCE_TRIPLES,
};
const RECORDS: ElementKind = ElementKind {
    size: 0x18,
    insert: INSERT_RECORDS,
    advance: ADVANCE_RECORDS,
};

/// The number of elements between two pointers (signed division by the element size).
fn elements_between(kind: &ElementKind, first: u32, last: u32) -> u32 {
    (last.wrapping_sub(first) as i32 / kind.size) as u32
}

/// `vector::insert(where, value)` of one element: remembers the offset of `position` from the
/// first element (0 for an empty vector; the iterators are checked), inserts one copy of `*value`
/// there (`_Insert_n`, which may reallocate), and writes to `out` the iterator for the same
/// offset in the new storage (an iterator at the first element advanced by the offset). Returns
/// `out`.
fn insert_one(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    (proxy, position, value): (u32, u32, Ptr),
    kind: &ElementKind,
) -> Ptr {
    let vector = this.addr();
    let offset = if elements_between(kind, e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10)) == 0
    {
        0
    } else {
        let first = e.mem.u32(vector + 0x0C);
        let own_proxy = checked_iterator(e, vector, first);
        if proxy == 0 || proxy != own_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        elements_between(kind, first, position)
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

// Translated from 00b1fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::insert(where, value)` for the 0xC-byte element (identified from the code;
/// `thiscall`: the result iterator slot, the iterator's proxy and position and the value's
/// address): see [`insert_one`], with `_Insert_n` `00b21e00` and the advance `00b235e0`.
pub fn fn_00b1fab0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(e, this, out, (proxy, position, value), &TRIPLES)
}

// Translated from 00b1fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The storage set-up (`_Buy(count)`) of the 0xC-byte element vector (identified from the code):
/// clears the first, last and end pointers; false for a count of 0; a count above `0x15555555`
/// reports a length error (`00b0fb80`); otherwise allocates `count` elements (`00b23bb0`), sets
/// first and last to the block and the end `count * 0xC` bytes after it. Returns true after a
/// nonzero count.
pub fn fn_00b1fbe0(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let vector = this.addr();
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
    if count == 0 {
        return false;
    }
    // `max > 0 ? max : 1` takes the maximum: the constant is never 0.
    if TRIPLE_MAX_COUNT < count {
        e.call(VECTOR_TOO_LONG, &args![]);
    } else {
        let storage = e.call(ALLOCATE_TRIPLES, &args![count, 0u32]).u32();
        e.mem.set_u32(vector + 0x0C, storage);
        let first = e.mem.u32(vector + 0x0C);
        e.mem.set_u32(vector + 0x10, first);
        let end = count
            .wrapping_mul(0x0C)
            .wrapping_add(e.mem.u32(vector + 0x0C));
        e.mem.set_u32(vector + 0x14, end);
    }
    true
}

// Translated from 00b1fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of the 0xC-byte element vector (identified from the code; `fastcall`): when there is
/// storage, walks the elements (the element destructor is empty) and frees it with `operator
/// delete`; clears the first, last and end pointers.
pub fn fn_00b1fca0(e: &mut Engine, this: Ptr<StlVector>) {
    let vector = this.addr();
    if e.mem.u32(vector + 0x0C) != 0 {
        let last = e.mem.u32(vector + 0x10);
        let mut element = e.mem.u32(vector + 0x0C);
        while element != last {
            element = element.wrapping_add(0x0C);
        }
        let storage = e.mem.u32(vector + 0x0C);
        e.call(OPERATOR_DELETE, &args![storage]);
    }
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

// Translated from 00b1fd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::insert(where, value)` for the 0x18-byte element (identified from the code;
/// `thiscall`): see [`insert_one`], with `_Insert_n` `00b22700` and the advance `00b16890`.
pub fn fn_00b1fd50(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(e, this, out, (proxy, position, value), &RECORDS)
}

// Translated from 00b1fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of the 0x18-byte element vector (identified from the code; `fastcall`): when there is
/// storage, destroys the elements (`00b22650`) and frees it with `operator delete`; clears the
/// first, last and end pointers.
pub fn fn_00b1fe80(e: &mut Engine, this: Ptr<StlVector>) {
    let vector = this.addr();
    if e.mem.u32(vector + 0x0C) != 0 {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        e.call(DESTROY_RECORDS, &args![this, first, last]);
        let storage = e.mem.u32(vector + 0x0C);
        e.call(OPERATOR_DELETE, &args![storage]);
    }
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b1fab0,
            fn_00b1fab0(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b1fbe0, fn_00b1fbe0(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b1fca0, fn_00b1fca0(Ptr<StlVector>)),
        entry!(
            0x00b1fd50,
            fn_00b1fd50(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b1fe80, fn_00b1fe80(Ptr<StlVector>)),
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

    #[test]
    fn single_insert_reports_the_offset_iterator() {
        for (addr, insert, advance, size) in [
            (0x00b1_fab0u32, INSERT_TRIPLES, ADVANCE_TRIPLES, 0x0Cu32),
            (0x00b1_fd50, INSERT_RECORDS, ADVANCE_RECORDS, 0x18),
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
            let value = e.mem.alloc(0x18);
            let r = e.call(
                addr,
                &args![
                    Ptr::<StlVector>::new(v),
                    Ptr::<()>::new(out),
                    PROXY,
                    data + 2 * size,
                    Ptr::<()>::new(value)
                ],
            );
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
            // A foreign or missing proxy is invalid.
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
    fn storage_set_up_of_the_triple_vector() {
        let mut e = engine();
        let storage = e.mem.alloc(64);
        let allocated = spy(&mut e, ALLOCATE_TRIPLES, storage);
        let long = spy(&mut e, VECTOR_TOO_LONG, 0);
        let v = e.mem.alloc(0x18);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v + off, 0x99);
        }
        assert!(!e
            .call(0x00b1_fbe0, &args![Ptr::<StlVector>::new(v), 0u32])
            .bool());
        assert_eq!(
            (
                e.mem.u32(v + 0x0C),
                e.mem.u32(v + 0x10),
                e.mem.u32(v + 0x14)
            ),
            (0, 0, 0)
        );
        assert!(e
            .call(0x00b1_fbe0, &args![Ptr::<StlVector>::new(v), 3u32])
            .bool());
        assert_eq!(*allocated.borrow(), vec![vec![3, 0]]);
        assert_eq!(
            (
                e.mem.u32(v + 0x0C),
                e.mem.u32(v + 0x10),
                e.mem.u32(v + 0x14)
            ),
            (storage, storage, storage + 0x24)
        );
        // The largest count is allowed; one more reports a length error and allocates nothing.
        allocated.borrow_mut().clear();
        assert!(e
            .call(
                0x00b1_fbe0,
                &args![Ptr::<StlVector>::new(v), 0x1555_5555u32]
            )
            .bool());
        assert_eq!(allocated.borrow().len(), 1);
        assert!(e
            .call(
                0x00b1_fbe0,
                &args![Ptr::<StlVector>::new(v), 0x1555_5556u32]
            )
            .bool());
        assert_eq!(allocated.borrow().len(), 1);
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(e.mem.u32(v + 0x0C), 0);
    }

    #[test]
    fn tidies_free_the_storage_with_operator_delete() {
        for (addr, size) in [(0x00b1_fca0u32, 0x0Cu32), (0x00b1_fe80, 0x18)] {
            let mut e = engine();
            let freed = spy(&mut e, OPERATOR_DELETE, 0);
            let destroyed = spy(&mut e, DESTROY_RECORDS, 0);
            let (v, data) = vector(&mut e, size, 3, 4);
            e.call(addr, &args![Ptr::<StlVector>::new(v)]);
            assert_eq!(*freed.borrow(), vec![vec![data]]);
            // Only the 0x18-byte element's tidy destroys the elements.
            let expected: Vec<Vec<u32>> = if size == 0x18 {
                vec![vec![v, data, data + 3 * size]]
            } else {
                vec![]
            };
            assert_eq!(*destroyed.borrow(), expected);
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
