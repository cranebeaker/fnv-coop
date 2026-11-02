//! `SpeedTree/run_00b1e3e0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The default constructor and `push_back` of the vector of 0xC-byte elements (the instance whose
//! insert, `_Buy` and tidy are in `run_00b1fab0`) and the destructor of the vector of 0x18-byte
//! elements (whose tidy is `00b1fe80`), identified from the code; the engine map gives them no
//! names.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{checked_iterator, StlVector, OPERATOR_DELETE};

/// The container proxy allocation (`0044d670`, `cdecl`: a count and a hint).
const PROXY_ALLOCATE: u32 = 0x0044_d670;
/// The 0xC-byte vector's storage set-up (`00b1fbe0`, `thiscall`: a count), the construction of
/// one element at the end (`00b244a0`, `cdecl`: dest, count, value, allocator and two tag words)
/// and its single-element insert (`00b1fab0`, `thiscall`: a result iterator slot, the proxy, the
/// position and the value's address); and the tidy of the 0x18-byte vector (`00b1fe80`).
const BUY_ELEMENTS: u32 = 0x00b1_fbe0;
const CONSTRUCT_ELEMENT: u32 = 0x00b2_44a0;
const INSERT_ELEMENT: u32 = 0x00b1_fab0;
const TIDY_ELEMENTS: u32 = 0x00b1_fe80;

// Translated from 00b1e3e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1e3e0 - the FS:[0] exception frame is not translated.
/// The default constructor of the vector (identified from the code; `fastcall`): makes the
/// container proxy (`0044d670`; its first word is the vector) and stores it at the vector's
/// start (a null block stays null), then sets up empty storage (`00b1fbe0` with a count of 0).
/// Returns `this`.
pub fn fn_00b1e3e0(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
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

// Translated from 00b1e4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back(const T &)` of the vector (identified from the code): with room left (size below
/// the capacity; a vector without a buffer has capacity 0) the element at `value` is
/// constructed at the end (`00b244a0`, with the allocator at `+8` and two tag words, the first an
/// uninitialised stack byte in the exe and 0 here) and the end advanced by 0xC; otherwise the
/// single-element insert at the end (`00b1fab0`) reallocates.
pub fn fn_00b1e4c0(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    let vector = this.addr();
    let capacity = if e.mem.u32(vector + 0x0C) == 0 {
        0
    } else {
        ((e.mem
            .u32(vector + 0x14)
            .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
            / 0x0C) as u32
    };
    let size = ((e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
        / 0x0C) as u32;
    if size < capacity {
        let last = e.mem.u32(vector + 0x10);
        e.call(
            CONSTRUCT_ELEMENT,
            &args![last, 1u32, value, vector + 8, 0u32, 0u32],
        );
        e.mem.set_u32(vector + 0x10, last.wrapping_add(0x0C));
    } else {
        let last = e.mem.u32(vector + 0x10);
        let proxy = checked_iterator(e, vector, last);
        e.with_stack(8, |e, out| {
            e.call(INSERT_ELEMENT, &args![this, out, proxy, last, value]);
        });
    }
}

// Translated from 00b1e5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1e5d0 - the FS:[0] exception frame is not translated.
/// The destructor of the 0x18-byte element vector (identified from the code; `fastcall`): tidies it (`00b1fe80`)
/// and frees its container proxy.
pub fn fn_00b1e5d0(e: &mut Engine, this: Ptr<StlVector>) {
    e.call(TIDY_ELEMENTS, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b1e3e0, fn_00b1e3e0(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b1e4c0, fn_00b1e4c0(Ptr<StlVector>, Ptr)),
        entry!(0x00b1e5d0, fn_00b1e5d0(Ptr<StlVector>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

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
        let data = e.mem.alloc((0x0C * capacity).max(1));
        e.mem.set_u32(at, 0x5000_0001);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + 0x0C * count);
        e.mem.set_u32(at + 0x14, data + 0x0C * capacity);
        (at, data)
    }

    #[test]
    fn default_constructor_makes_the_proxy_and_empty_storage() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        let proxy_block = e.mem.alloc(4);
        let proxies = spy(&mut e, PROXY_ALLOCATE, proxy_block);
        let buys = spy(&mut e, BUY_ELEMENTS, 0);
        let r = e.call(0x00b1_e3e0, &args![Ptr::<()>::new(this)]);
        assert_eq!(r.u32(), this);
        assert_eq!(*proxies.borrow(), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(proxy_block), this);
        assert_eq!(e.mem.u32(this), proxy_block);
        assert_eq!(*buys.borrow(), vec![vec![this, 0]]);
        // A failed proxy allocation leaves a null proxy.
        spy(&mut e, PROXY_ALLOCATE, 0);
        e.call(0x00b1_e3e0, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn push_back_with_room_constructs_at_the_end() {
        let mut e = Engine::new();
        let constructs = spy(&mut e, CONSTRUCT_ELEMENT, 0);
        let inserts = spy(&mut e, INSERT_ELEMENT, 0);
        let (v, data) = vector(&mut e, 2, 4);
        let value = e.mem.alloc(0x0C);
        e.call(
            0x00b1_e4c0,
            &args![Ptr::<StlVector>::new(v), Ptr::<()>::new(value)],
        );
        assert_eq!(
            *constructs.borrow(),
            vec![vec![data + 0x18, 1, value, v + 8, 0, 0]]
        );
        assert!(inserts.borrow().is_empty());
        assert_eq!(e.mem.u32(v + 0x10), data + 0x24);
    }

    #[test]
    fn push_back_without_room_inserts_at_the_end() {
        let mut e = Engine::new();
        let constructs = spy(&mut e, CONSTRUCT_ELEMENT, 0);
        let inserts = spy(&mut e, INSERT_ELEMENT, 0);
        let (v, data) = vector(&mut e, 4, 4);
        let value = e.mem.alloc(0x0C);
        e.call(
            0x00b1_e4c0,
            &args![Ptr::<StlVector>::new(v), Ptr::<()>::new(value)],
        );
        assert!(constructs.borrow().is_empty());
        let log = inserts.borrow().clone();
        assert_eq!(log.len(), 1);
        // (this, result slot, proxy, position, value)
        assert_eq!(log[0][0], v);
        assert_eq!(log[0][2..], [0x5000_0001, data + 0x30, value]);
        // A vector without a buffer has capacity 0: the insert reallocates too.
        let (v2, _) = vector(&mut e, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        e.call(
            0x00b1_e4c0,
            &args![Ptr::<StlVector>::new(v2), Ptr::<()>::new(value)],
        );
        assert_eq!(inserts.borrow().len(), 2);
        // The checked iterator at the end of a vector whose last is below its first fails.
        e.register(super::super::branch::INVALID_PARAMETER, |_, _| {
            panic!("invalid parameter")
        });
        let (v3, d3) = vector(&mut e, 4, 4);
        e.mem.set_u32(v3 + 0x10, d3 - 0x0C);
        let failed = catch_unwind(AssertUnwindSafe(|| {
            fn_00b1e4c0(&mut e, Ptr::new(v3), Ptr::new(value));
        }));
        assert!(failed.is_err());
    }

    #[test]
    fn destructor_tidies_and_frees_the_proxy() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, 0x7777);
        let tidied = spy(&mut e, TIDY_ELEMENTS, 0);
        let freed = spy(&mut e, OPERATOR_DELETE, 0);
        e.call(0x00b1_e5d0, &args![Ptr::<StlVector>::new(this)]);
        assert_eq!(*tidied.borrow(), vec![vec![this]]);
        assert_eq!(*freed.borrow(), vec![vec![0x7777]]);
    }
}
