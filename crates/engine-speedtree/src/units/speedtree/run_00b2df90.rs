//! `SpeedTree/run_00b2df90` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three more helpers of the vector of 0x54-byte elements (see `run_00b2e250`), identified from
//! the code (the engine map gives them no names): `clear()`, the storage set-up (`_Buy`) and the
//! destruction of a range of elements.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{StlVector, OPERATOR_DELETE};
use super::run_00b31070::clear_with_erase;

/// The range erase of the 0x54-byte element vector (`00b2e250`).
const ERASE_RANGE: u32 = 0x00b2_e250;
/// The element allocation (`00b2e4a0`, `cdecl`: a count and a hint), the "vector<T> too long"
/// throw (`00b0fb80`) and the string's `_Tidy(built, new_size)` (`0044a730`, `thiscall`).
const ALLOCATE_ELEMENTS: u32 = 0x00b2_e4a0;
const VECTOR_TOO_LONG: u32 = 0x00b0_fb80;
const STRING_TIDY: u32 = 0x0044_a730;
/// The largest element count of the vector (`0x30C30C3`, `0xFFFFFFFF / 0x54`).
const MAX_COUNT: u32 = 0x030C_30C3;
/// The scalar-deleting-destructor flag the exe folds to 0 where an inlined destructor call
/// could free its object.
const DESTRUCTOR_FREES: bool = false;

// Translated from 00b2df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `clear()` of the 0x54-byte element vector (identified from the code; `fastcall`): erases the
/// whole range `begin()..end()` through `00b2e250` after the iterator checks.
pub fn fn_00b2df90(e: &mut Engine, this: Ptr<StlVector>) {
    clear_with_erase(e, this, ERASE_RANGE);
}

// Translated from 00b2e080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The storage set-up (`_Buy(count)`) of the 0x54-byte element vector (identified from the code):
/// clears the first, last and end pointers; false for a count of 0; a count above `0x30C30C3`
/// reports a length error (`00b0fb80`); otherwise allocates `count` elements (`00b2e4a0`), sets
/// first and last to the block and the end `count * 0x54` bytes after it. Returns true after a
/// nonzero count.
pub fn fn_00b2e080(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let vector = this.addr();
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
    if count == 0 {
        return false;
    }
    if count > MAX_COUNT {
        e.call(VECTOR_TOO_LONG, &args![]);
    } else {
        let storage = e.call(ALLOCATE_ELEMENTS, &args![count, 0u32]).u32();
        e.mem.set_u32(vector + 0x0C, storage);
        let first = e.mem.u32(vector + 0x0C);
        e.mem.set_u32(vector + 0x10, first);
        let end = count
            .wrapping_mul(0x54)
            .wrapping_add(e.mem.u32(vector + 0x0C));
        e.mem.set_u32(vector + 0x14, end);
    }
    true
}

// Translated from 00b2e140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the 0x54-byte elements `first..last` (identified from the code; `thiscall`, `this`
/// is not read): the string at `+0x14` of each element is tidied (`0044a730` with built = 1 and
/// new size 0).
pub fn fn_00b2e140(e: &mut Engine, _this: Ptr, first: u32, last: u32) {
    let mut element = first;
    while element != last {
        e.call(STRING_TIDY, &args![element.wrapping_add(0x14), 1u32, 0u32]);
        if DESTRUCTOR_FREES {
            // The destructor's delete flag is the constant 0 (`XOR EDX, EDX`, `AND EDX, 1`), so
            // the element is never freed here.
            e.call(OPERATOR_DELETE, &args![element]);
        }
        element = element.wrapping_add(0x54);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b2df90, fn_00b2df90(Ptr<StlVector>)),
        entry!(0x00b2e080, fn_00b2e080(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b2e140, fn_00b2e140(Ptr, u32, u32)),
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

    #[test]
    fn clear_erases_the_whole_range() {
        let mut e = Engine::new();
        e.register(super::super::branch::INVALID_PARAMETER, |_, _| {
            panic!("invalid parameter")
        });
        let erased = spy(&mut e, ERASE_RANGE, 0);
        let v = e.mem.alloc(0x18);
        let data = e.mem.alloc(0x54 * 4);
        e.mem.set_u32(v, 0x5000_0001);
        e.mem.set_u32(v + 0x0C, data);
        e.mem.set_u32(v + 0x10, data + 0x54 * 3);
        e.mem.set_u32(v + 0x14, data + 0x54 * 4);
        e.call(0x00b2_df90, &args![Ptr::<()>::new(v)]);
        let log = erased.borrow().clone();
        assert_eq!(log.len(), 1);
        // (this, result slot, first proxy, first, last proxy, last)
        let proxy = 0x5000_0001;
        assert_eq!(log[0][0], v);
        assert_eq!(log[0][2..], [proxy, data, proxy, data + 0x54 * 3]);
        // A first pointer past the last is invalid.
        e.mem.set_u32(v + 0x10, data - 0x54);
        let failed = catch_unwind(AssertUnwindSafe(|| {
            e.call(0x00b2_df90, &args![Ptr::<()>::new(v)]);
        }));
        assert!(failed.is_err());
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
            .call(0x00b2_e080, &args![Ptr::<StlVector>::new(v), 0u32])
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
            .call(0x00b2_e080, &args![Ptr::<StlVector>::new(v), 3u32])
            .bool());
        assert_eq!(*allocated.borrow(), vec![vec![3, 0]]);
        assert_eq!(
            (
                e.mem.u32(v + 0x0C),
                e.mem.u32(v + 0x10),
                e.mem.u32(v + 0x14)
            ),
            (storage, storage, storage + 3 * 0x54)
        );
        // The largest count is allowed; one more reports a length error and allocates nothing.
        allocated.borrow_mut().clear();
        assert!(e
            .call(0x00b2_e080, &args![Ptr::<StlVector>::new(v), MAX_COUNT])
            .bool());
        assert_eq!(allocated.borrow().len(), 1);
        assert!(e
            .call(0x00b2_e080, &args![Ptr::<StlVector>::new(v), MAX_COUNT + 1])
            .bool());
        assert_eq!(allocated.borrow().len(), 1);
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(e.mem.u32(v + 0x0C), 0);
    }

    #[test]
    fn destroying_elements_tidies_their_strings() {
        let mut e = Engine::new();
        let tidied = spy(&mut e, STRING_TIDY, 0);
        let freed = spy(&mut e, OPERATOR_DELETE, 0);
        e.call(
            0x00b2_e140,
            &args![Ptr::<()>::NULL, 0x1000u32, 0x1000u32 + 0x54 * 2],
        );
        assert_eq!(
            *tidied.borrow(),
            vec![vec![0x1014, 1, 0], vec![0x1014 + 0x54, 1, 0]]
        );
        assert!(freed.borrow().is_empty());
        // An empty range does nothing.
        tidied.borrow_mut().clear();
        e.call(0x00b2_e140, &args![Ptr::<()>::NULL, 0x1000u32, 0x1000u32]);
        assert!(tidied.borrow().is_empty());
    }
}
