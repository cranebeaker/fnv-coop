//! `SpeedTree/run_00b0dce0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The default constructor, `push_back` and single-element erase of the vector of 4-byte
//! elements (the instance whose insert, `_Buy` and tidy are `00b0e600`, `00b0e730` and
//! `00b0e7f0`), identified from the code; the engine map gives them no names.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{checked_iterator, StlVector, INVALID_PARAMETER};

/// The container proxy allocation (`0044d670`, `cdecl`: a count and a hint).
const PROXY_ALLOCATE: u32 = 0x0044_d670;
/// The storage set-up (`00b0e730`, `thiscall`: a count), the construction of one element at the
/// end (`00b36b10`, `cdecl`: dest, count, value pointer), the single-element insert
/// (`00b0e600`, `thiscall`: a result iterator slot, the proxy, the position and the value's
/// address) and the word range copy (`00b42e50`, `cdecl`: first, last, dest).
const BUY_ELEMENTS: u32 = 0x00b0_e730;
const CONSTRUCT_ELEMENT: u32 = 0x00b3_6b10;
const INSERT_ELEMENT: u32 = 0x00b0_e600;
const COPY_WORDS: u32 = 0x00b4_2e50;

// Translated from 00b0dce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0dce0 - the FS:[0] exception frame is not translated.
/// The default constructor of the vector (identified from the code; `fastcall`): makes the
/// container proxy (`0044d670`; its first word is the vector) and stores it at the vector's
/// start (a null block stays null), then sets up empty storage (`00b0e730` with a count of 0).
/// Returns `this`.
pub fn fn_00b0dce0(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    let proxy = e.call(PROXY_ALLOCATE, &args![1u32, 0u32]).u32();
    let stored = if proxy == 0 {
        0
    } else {
        e.mem.set_u32(proxy, this.addr());
        proxy
    };
    e.mem.set_u32(this.addr(), stored);
    e.call(BUY_ELEMENTS, &args![this, 0u32]);
    this
}

// Translated from 00b0ddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back(const int &)` of the vector (identified from the code): with room left (size below
/// the capacity; a vector without a buffer has capacity 0) the element at `value` is
/// constructed at the end (`00b36b10`) and the end advanced by 4; otherwise the single-element
/// insert at the end (`00b0e600`) reallocates.
pub fn fn_00b0ddc0(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    let vector = this.addr();
    let capacity = if e.mem.u32(vector + 0x0C) == 0 {
        0
    } else {
        ((e.mem
            .u32(vector + 0x14)
            .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
            >> 2) as u32
    };
    let size = ((e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
        >> 2) as u32;
    if size < capacity {
        let last = e.mem.u32(vector + 0x10);
        e.call(CONSTRUCT_ELEMENT, &args![last, 1u32, value]);
        e.mem.set_u32(vector + 0x10, last.wrapping_add(4));
    } else {
        let last = e.mem.u32(vector + 0x10);
        let proxy = checked_iterator(e, vector, last);
        e.with_stack(8, |e, out| {
            e.call(INSERT_ELEMENT, &args![this, out, proxy, last, value]);
        });
    }
}

// Translated from 00b0df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `erase(where)` of one element of the vector (identified from the code; `thiscall`: a result
/// iterator slot, the iterator's proxy, which is not read, and its position): the elements after
/// `position` are copied down by one (`00b42e50`) and the last pointer lowered by 4; the
/// position must then be inside `[first, last]` (the invalid-parameter path otherwise). Writes
/// the iterator (the vector's proxy and `position`) to the result slot and returns it.
pub fn fn_00b0df00(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_1: u32,
    position: u32,
) -> Ptr {
    let vector = this.addr();
    let last = e.mem.u32(vector + 0x10);
    e.call(COPY_WORDS, &args![position.wrapping_add(4), last, position]);
    let lowered = e.mem.u32(vector + 0x10).wrapping_sub(4);
    e.mem.set_u32(vector + 0x10, lowered);
    e.mem.set_u32(out.addr(), 0);
    if vector == 0 || e.mem.u32(vector + 0x0C) > position || position > e.mem.u32(vector + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(vector);
    e.mem.set_u32(out.addr(), proxy);
    e.mem.set_u32(out.addr() + 4, position);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0dce0, fn_00b0dce0(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b0ddc0, fn_00b0ddc0(Ptr<StlVector>, Ptr)),
        entry!(
            0x00b0df00,
            fn_00b0df00(Ptr<StlVector>, Ptr, u32, u32) -> Ptr
        ),
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

    fn spy(e: &mut Engine, addr: u32, result: u32) -> Words {
        let log: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(result)
        });
        log
    }

    fn vector(e: &mut Engine, count: u32, capacity: u32) -> (u32, u32) {
        let at = e.mem.alloc(0x18);
        let data = e.mem.alloc((4 * capacity).max(1));
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + 4 * count);
        e.mem.set_u32(at + 0x14, data + 4 * capacity);
        (at, data)
    }

    #[test]
    fn default_constructor_makes_the_proxy_and_empty_storage() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        let proxy_block = e.mem.alloc(4);
        let proxies = spy(&mut e, PROXY_ALLOCATE, proxy_block);
        let buys = spy(&mut e, BUY_ELEMENTS, 0);
        let r = e.call(0x00b0_dce0, &args![Ptr::<()>::new(this)]);
        assert_eq!(r.u32(), this);
        assert_eq!(*proxies.borrow(), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(proxy_block), this);
        assert_eq!(e.mem.u32(this), proxy_block);
        assert_eq!(*buys.borrow(), vec![vec![this, 0]]);
        // A failed proxy allocation leaves a null proxy.
        spy(&mut e, PROXY_ALLOCATE, 0);
        e.call(0x00b0_dce0, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn push_back_with_room_constructs_at_the_end() {
        let mut e = Engine::new();
        let constructs = spy(&mut e, CONSTRUCT_ELEMENT, 0);
        let inserts = spy(&mut e, INSERT_ELEMENT, 0);
        let (v, data) = vector(&mut e, 2, 4);
        let value = e.mem.alloc(4);
        e.call(
            0x00b0_ddc0,
            &args![Ptr::<StlVector>::new(v), Ptr::<()>::new(value)],
        );
        assert_eq!(*constructs.borrow(), vec![vec![data + 8, 1, value]]);
        assert!(inserts.borrow().is_empty());
        assert_eq!(e.mem.u32(v + 0x10), data + 12);
    }

    #[test]
    fn push_back_without_room_inserts_at_the_end() {
        let mut e = Engine::new();
        let constructs = spy(&mut e, CONSTRUCT_ELEMENT, 0);
        let inserts = spy(&mut e, INSERT_ELEMENT, 0);
        let (v, data) = vector(&mut e, 4, 4);
        let value = e.mem.alloc(4);
        e.call(
            0x00b0_ddc0,
            &args![Ptr::<StlVector>::new(v), Ptr::<()>::new(value)],
        );
        assert!(constructs.borrow().is_empty());
        let log = inserts.borrow().clone();
        assert_eq!(log.len(), 1);
        // (this, result slot, proxy, position, value)
        assert_eq!(log[0][0], v);
        assert_eq!(log[0][2..], [PROXY, data + 16, value]);
        // A vector without a buffer has capacity 0: the insert reallocates too.
        let (v2, _) = vector(&mut e, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        e.call(
            0x00b0_ddc0,
            &args![Ptr::<StlVector>::new(v2), Ptr::<()>::new(value)],
        );
        assert_eq!(inserts.borrow().len(), 2);
    }

    #[test]
    fn erase_one_copies_the_tail_down() {
        let mut e = Engine::new();
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
        let copies = spy(&mut e, COPY_WORDS, 0);
        let (v, data) = vector(&mut e, 4, 4);
        let out = e.mem.alloc(8);
        let r = e.call(
            0x00b0_df00,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                0u32,
                data + 4
            ],
        );
        assert_eq!(r.u32(), out);
        assert_eq!(*copies.borrow(), vec![vec![data + 8, data + 16, data + 4]]);
        assert_eq!(e.mem.u32(v + 0x10), data + 12);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, data + 4));
        // The position of the last element stays valid; one past the lowered last does not.
        e.call(
            0x00b0_df00,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                0u32,
                data + 8
            ],
        );
        assert_eq!(e.mem.u32(v + 0x10), data + 8);
        let failed = catch_unwind(AssertUnwindSafe(|| {
            fn_00b0df00(&mut e, Ptr::new(v), Ptr::new(out), 0, data + 16);
        }));
        assert!(failed.is_err());
        let (v2, d2) = vector(&mut e, 4, 4);
        let failed = catch_unwind(AssertUnwindSafe(|| {
            fn_00b0df00(&mut e, Ptr::new(v2), Ptr::new(out), 0, d2 - 4);
        }));
        assert!(failed.is_err());
    }
}
