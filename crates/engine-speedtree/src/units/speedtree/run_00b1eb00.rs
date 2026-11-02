//! `SpeedTree/run_00b1eb00` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The storage set-up (`_Buy`), the destruction of a range and the tidy of a vector of 0x38-byte
//! elements (each a `std::vector` at its start: destroyed by `00b1e970` and its container proxy
//! freed), identified from the code; the engine map gives them no names.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{StlVector, OPERATOR_DELETE};

/// The element allocation (`00b23980`, `cdecl`: a count and a hint), the "vector<T> too long"
/// throw (`00b0fb80`) and the element destructor (`00b1e970`, `thiscall`).
const ALLOCATE_ELEMENTS: u32 = 0x00b2_3980;
const VECTOR_TOO_LONG: u32 = 0x00b0_fb80;
const DESTROY_ELEMENT: u32 = 0x00b1_e970;
/// The largest element count of the vector (`0x4924924`, `0xFFFFFFFF / 0x38`).
const MAX_COUNT: u32 = 0x0492_4924;
/// The scalar-deleting-destructor flag the exe folds to 0 where an inlined destructor call
/// could free its object.
const DESTRUCTOR_FREES: bool = false;

// Translated from 00b1eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The storage set-up (`_Buy(count)`) of the 0x38-byte element vector (identified from the code):
/// clears the first, last and end pointers; false for a count of 0; a count above `0x4924924`
/// reports a length error (`00b0fb80`); otherwise allocates `count` elements (`00b23980`), sets
/// first and last to the block and the end `count * 0x38` bytes after it. Returns true after a
/// nonzero count.
pub fn fn_00b1eb00(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let vector = this.addr();
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
    if count == 0 {
        return false;
    }
    // `max > 0 ? max : 1` takes the maximum: the constant is never 0.
    if MAX_COUNT < count {
        e.call(VECTOR_TOO_LONG, &args![]);
    } else {
        let storage = e.call(ALLOCATE_ELEMENTS, &args![count, 0u32]).u32();
        e.mem.set_u32(vector + 0x0C, storage);
        let first = e.mem.u32(vector + 0x0C);
        e.mem.set_u32(vector + 0x10, first);
        let end = count
            .wrapping_mul(0x38)
            .wrapping_add(e.mem.u32(vector + 0x0C));
        e.mem.set_u32(vector + 0x14, end);
    }
    true
}

// Translated from 00b1ebc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1ebc0 - the FS:[0] exception frame is not translated.
/// Destroys the 0x38-byte elements `first..last` (identified from the code; `thiscall`, `this`
/// is not read): each element is destroyed by `00b1e970` and the block its first word points at
/// (the element's container proxy) freed with `operator delete`.
pub fn fn_00b1ebc0(e: &mut Engine, _this: Ptr, first: u32, last: u32) {
    let mut element = first;
    while element != last {
        e.call(DESTROY_ELEMENT, &args![element]);
        let proxy = e.mem.u32(element);
        e.call(OPERATOR_DELETE, &args![proxy]);
        if DESTRUCTOR_FREES {
            // The destructor's delete flag is the constant 0 (`XOR ECX, ECX`, `AND ECX, 1`), so
            // the element itself is never freed here.
            e.call(OPERATOR_DELETE, &args![element]);
        }
        element = element.wrapping_add(0x38);
    }
}

// Translated from 00b1ec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of the 0x38-byte element vector (identified from the code; `fastcall`): when there
/// is storage, destroys the elements (`00b1ebc0`) and frees it with `operator delete`; clears
/// the first, last and end pointers.
pub fn fn_00b1ec70(e: &mut Engine, this: Ptr<StlVector>) {
    let vector = this.addr();
    if e.mem.u32(vector + 0x0C) != 0 {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        fn_00b1ebc0(e, this.cast(), first, last);
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
        entry!(0x00b1eb00, fn_00b1eb00(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b1ebc0, fn_00b1ebc0(Ptr, u32, u32)),
        entry!(0x00b1ec70, fn_00b1ec70(Ptr<StlVector>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

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

    fn spy_all(e: &mut Engine, addrs: &[u32]) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addrs {
            let address = *address;
            let sink = log.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    #[test]
    fn storage_set_up() {
        let mut e = Engine::new();
        let storage = e.mem.alloc(64);
        let allocated = spy(&mut e, ALLOCATE_ELEMENTS, storage);
        let long = spy(&mut e, VECTOR_TOO_LONG, 0);
        let v = e.mem.alloc(0x18);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v + off, 0x99);
        }
        assert!(!e
            .call(0x00b1_eb00, &args![Ptr::<StlVector>::new(v), 0u32])
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
            .call(0x00b1_eb00, &args![Ptr::<StlVector>::new(v), 3u32])
            .bool());
        assert_eq!(*allocated.borrow(), vec![vec![3, 0]]);
        assert_eq!(
            (
                e.mem.u32(v + 0x0C),
                e.mem.u32(v + 0x10),
                e.mem.u32(v + 0x14)
            ),
            (storage, storage, storage + 3 * 0x38)
        );
        // The largest count is allowed; one more reports a length error and allocates nothing.
        allocated.borrow_mut().clear();
        assert!(e
            .call(0x00b1_eb00, &args![Ptr::<StlVector>::new(v), MAX_COUNT])
            .bool());
        assert_eq!(allocated.borrow().len(), 1);
        assert!(e
            .call(0x00b1_eb00, &args![Ptr::<StlVector>::new(v), MAX_COUNT + 1])
            .bool());
        assert_eq!(allocated.borrow().len(), 1);
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(e.mem.u32(v + 0x0C), 0);
    }

    #[test]
    fn range_destruction_destroys_each_element_and_frees_its_proxy() {
        let mut e = Engine::new();
        let log = spy_all(&mut e, &[DESTROY_ELEMENT, OPERATOR_DELETE]);
        let data = e.mem.alloc(0x38 * 2);
        e.mem.set_u32(data, 0x7001);
        e.mem.set_u32(data + 0x38, 0x7002);
        e.call(0x00b1_ebc0, &args![Ptr::<()>::NULL, data, data + 0x38 * 2]);
        assert_eq!(
            *log.borrow(),
            vec![
                (DESTROY_ELEMENT, vec![data]),
                (OPERATOR_DELETE, vec![0x7001]),
                (DESTROY_ELEMENT, vec![data + 0x38]),
                (OPERATOR_DELETE, vec![0x7002]),
            ]
        );
        // An empty range does nothing.
        log.borrow_mut().clear();
        e.call(0x00b1_ebc0, &args![Ptr::<()>::NULL, data, data]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn tidy_destroys_and_frees() {
        let mut e = Engine::new();
        let log = spy_all(&mut e, &[DESTROY_ELEMENT, OPERATOR_DELETE]);
        let v = e.mem.alloc(0x18);
        let data = e.mem.alloc(0x38 * 3);
        e.mem.set_u32(v + 0x0C, data);
        e.mem.set_u32(v + 0x10, data + 0x38);
        e.mem.set_u32(v + 0x14, data + 0x38 * 3);
        e.mem.set_u32(data, 0x7001);
        e.call(0x00b1_ec70, &args![Ptr::<StlVector>::new(v)]);
        assert_eq!(
            *log.borrow(),
            vec![
                (DESTROY_ELEMENT, vec![data]),
                (OPERATOR_DELETE, vec![0x7001]),
                (OPERATOR_DELETE, vec![data]),
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
        // Without storage nothing is freed.
        log.borrow_mut().clear();
        e.call(0x00b1_ec70, &args![Ptr::<StlVector>::new(v)]);
        assert!(log.borrow().is_empty());
    }
}
