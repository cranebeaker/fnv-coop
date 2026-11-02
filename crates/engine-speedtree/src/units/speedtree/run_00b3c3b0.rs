//! `SpeedTree/run_00b3c3b0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two small pieces of the tree code, identified from the code (the engine map gives them no
//! names): the destructor, scalar deleting destructor and constructor of the exception class
//! the vector/map code throws for an invalid iterator (vtable `010a57fc`, a base class vtable
//! `010173e8`, the message string at `+0x0C`), and the two constructors of `CVec` (five floats at
//! `+0` and a small count at `+0x14`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::OPERATOR_DELETE;

/// The vtable of the exception class (`010a57fc`) and of its base class (`010173e8`).
const EXCEPTION_VTABLE: u32 = 0x010a_57fc;
const BASE_EXCEPTION_VTABLE: u32 = 0x0101_73e8;
/// The string's `_Tidy(built, new_size)` (`0044a730`, `thiscall`), the base constructor from a
/// string (`0044a450`, `thiscall`: the string) and the base destructor (`00ec79a1`, `thiscall`).
const STRING_TIDY: u32 = 0x0044_a730;
const BASE_FROM_STRING: u32 = 0x0044_a450;
const BASE_DESTRUCTOR: u32 = 0x00ec_79a1;

/// The destruction the destructors share: the base class's vtable is stored (after the class's
/// own, which the exe also stores first), the message string at `+0x0C` is tidied
/// (`0044a730` with built = 1 and new size 0) and the base destructor runs.
fn destroy_exception(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EXCEPTION_VTABLE);
    e.mem.set_u32(this.addr(), BASE_EXCEPTION_VTABLE);
    e.call(STRING_TIDY, &args![this.addr() + 0x0C, 1u32, 0u32]);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// Clears the five floats of a `CVec` (`+0x10` down to `+0`).
fn clear_cvec(e: &mut Engine, this: Ptr) {
    for offset in [0x10u32, 0x0C, 0x08, 0x04, 0x00] {
        e.mem.set_f32(this.addr() + offset, 0.0);
    }
}

// Translated from 00b3c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the invalid-iterator exception class (identified from the code;
/// `fastcall`): see [`destroy_exception`].
pub fn fn_00b3c3b0(e: &mut Engine, this: Ptr) {
    destroy_exception(e, this);
}

// Translated from 00b3c3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the invalid-iterator exception class (identified from the
/// code; `thiscall`, the flags word): the destruction of `00b3c3b0`, then `operator delete` of
/// the object when bit 0 of the flags is set. Returns `this`.
pub fn fn_00b3c3f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    destroy_exception(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00b3c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the invalid-iterator exception class from a message string (identified
/// from the code; `thiscall`): the base class constructor (`0044a450` with the string), then the
/// class's vtable. Returns `this`.
pub fn fn_00b3c440(e: &mut Engine, this: Ptr, message: Ptr) -> Ptr {
    e.call(BASE_FROM_STRING, &args![this, message]);
    e.mem.set_u32(this.addr(), EXCEPTION_VTABLE);
    this
}

// Translated from 00b3c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CVec` default constructor (identified from the code; `fastcall`): the five floats
/// cleared (stored `+0x10` down to `+0`) and the count at `+0x14` set to 3. Returns `this`.
pub fn fn_00b3c470(e: &mut Engine, this: Ptr) -> Ptr {
    clear_cvec(e, this);
    e.mem.set_u32(this.addr() + 0x14, 3);
    this
}

// Translated from 00b3c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CVec` constructor with a count (identified from the code; `thiscall`): the five floats
/// cleared and the count at `+0x14` set to the argument, limited to 5 (a signed compare).
/// Returns `this`.
pub fn fn_00b3c4b0(e: &mut Engine, this: Ptr, count: i32) -> Ptr {
    clear_cvec(e, this);
    let mut count = count;
    if count > 5 {
        count = 5;
    }
    e.mem.set_i32(this.addr() + 0x14, count);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b3c3b0, fn_00b3c3b0(Ptr)),
        entry!(0x00b3c3f0, fn_00b3c3f0(Ptr, u32) -> Ptr),
        entry!(0x00b3c440, fn_00b3c440(Ptr, Ptr) -> Ptr),
        entry!(0x00b3c470, fn_00b3c470(Ptr) -> Ptr),
        entry!(0x00b3c4b0, fn_00b3c4b0(Ptr, i32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

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
    fn destructor_stores_the_base_vtable_and_tidies() {
        let mut e = Engine::new();
        let log = spy_all(&mut e, &[STRING_TIDY, BASE_DESTRUCTOR, OPERATOR_DELETE]);
        let object = e.mem.alloc(0x28);
        e.call(0x00b3_c3b0, &args![Ptr::<()>::new(object)]);
        assert_eq!(e.mem.u32(object), BASE_EXCEPTION_VTABLE);
        assert_eq!(
            *log.borrow(),
            vec![
                (STRING_TIDY, vec![object + 0x0C, 1, 0]),
                (BASE_DESTRUCTOR, vec![object])
            ]
        );
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_zero() {
        let mut e = Engine::new();
        let log = spy_all(&mut e, &[STRING_TIDY, BASE_DESTRUCTOR, OPERATOR_DELETE]);
        let object = e.mem.alloc(0x28);
        let r = e.call(0x00b3_c3f0, &args![Ptr::<()>::new(object), 0u32]);
        assert_eq!(r.u32(), object);
        assert_eq!(log.borrow().len(), 2);
        let r = e.call(0x00b3_c3f0, &args![Ptr::<()>::new(object), 3u32]);
        assert_eq!(r.u32(), object);
        let calls = log.borrow().clone();
        assert_eq!(calls.len(), 5);
        assert_eq!(calls[4], (OPERATOR_DELETE, vec![object]));
        // Only bit 0 counts.
        let before = log.borrow().len();
        e.call(0x00b3_c3f0, &args![Ptr::<()>::new(object), 2u32]);
        assert_eq!(log.borrow().len(), before + 2);
    }

    #[test]
    fn constructor_builds_the_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        let log = spy_all(&mut e, &[BASE_FROM_STRING]);
        let object = e.mem.alloc(0x28);
        let message = e.mem.alloc(0x1C);
        let r = e.call(
            0x00b3_c440,
            &args![Ptr::<()>::new(object), Ptr::<()>::new(message)],
        );
        assert_eq!(r.u32(), object);
        assert_eq!(
            *log.borrow(),
            vec![(BASE_FROM_STRING, vec![object, message])]
        );
        assert_eq!(e.mem.u32(object), EXCEPTION_VTABLE);
    }

    #[test]
    fn cvec_constructors() {
        let mut e = Engine::new();
        let at = e.mem.alloc(0x18);
        for word in 0..6 {
            e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
        }
        let r = e.call(0x00b3_c470, &args![Ptr::<()>::new(at)]);
        assert_eq!(r.u32(), at);
        for offset in [0u32, 4, 8, 0x0C, 0x10] {
            assert_eq!(e.mem.u32(at + offset), 0);
        }
        assert_eq!(e.mem.u32(at + 0x14), 3);
        // The counted form limits the count to 5 (signed: a negative count stays).
        for (count, expected) in [(2i32, 2i32), (5, 5), (9, 5), (-4, -4)] {
            for word in 0..5 {
                e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
            }
            let r = e.call(0x00b3_c4b0, &args![Ptr::<()>::new(at), count]);
            assert_eq!(r.u32(), at);
            assert_eq!(e.mem.u32(at + 0x10), 0);
            assert_eq!(e.mem.u32(at), 0);
            assert_eq!(e.mem.i32(at + 0x14), expected);
        }
    }
}
