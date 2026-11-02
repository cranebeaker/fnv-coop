//! `Havok SDK/hkrefvariant.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkRefVariant` (Xbox PDB, size 4) is one reference-counted object pointer at `+0`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkReferencedObject::addReference` and `removeReference` (the locked variants), `this` in
/// `ECX`.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The global (`0x01269b10`) holding the object that finds the class of an object: its virtual
/// slot `0x10` takes the object.
const CLASS_FINDER_GLOBAL: u32 = 0x0126_9b10;

// Translated from 00d47770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRefVariant::set` (Xbox PDB), `(object, _unused)`: adds a reference to `object` when it
/// is not null, removes one from the held object when there is one, and keeps `object`. The
/// second stack word (the exe's `RET 8`) is not read.
pub fn hk_ref_variant_set(e: &mut Engine, this: Ptr, object: Ptr, _unused_2: u32) {
    if object.addr() != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    let held = e.mem.u32(this.addr());
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(this.addr(), object.addr());
}

// Translated from 00d477a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRefVariant::getClass` (Xbox PDB): 0 when nothing is held, else the result of the class
/// finder's virtual slot `0x10` (the finder is the object at the global `0x01269b10`) called
/// with the held object.
pub fn hk_ref_variant_get_class(e: &mut Engine, this: Ptr) -> u32 {
    let held = e.mem.u32(this.addr());
    if held == 0 {
        return 0;
    }
    let finder = e.mem.u32(CLASS_FINDER_GLOBAL);
    e.vcall(finder, 0x10, &args![held]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_7770, hk_ref_variant_set(Ptr, Ptr, u32)),
        entry!(0x00d4_77a0, hk_ref_variant_get_class(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn set_adds_the_new_reference_then_removes_the_old_one() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(ADD_REFERENCE, move |_, a| {
            seen.borrow_mut().push(("add", a[0]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            seen.borrow_mut().push(("remove", a[0]));
            Ret::default()
        });
        let this = e.mem.alloc(4);
        e.call(0x00d4_7770, &args![this, 0x10u32, 0u32]);
        assert_eq!(*log.borrow(), vec![("add", 0x10)]);
        assert_eq!(e.mem.u32(this), 0x10);
        log.borrow_mut().clear();
        e.call(0x00d4_7770, &args![this, 0x20u32, 0u32]);
        assert_eq!(*log.borrow(), vec![("add", 0x20), ("remove", 0x10)]);
        assert_eq!(e.mem.u32(this), 0x20);
        log.borrow_mut().clear();
        e.call(0x00d4_7770, &args![this, 0u32, 0u32]);
        assert_eq!(*log.borrow(), vec![("remove", 0x20)]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn get_class_asks_the_finder_only_for_a_held_object() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let finder = e.mem.alloc(0x10);
        e.mem.set_u32(finder, vtable);
        e.mem.set_u32(CLASS_FINDER_GLOBAL, finder);
        e.register(0x0900_0010, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x00d4_77a0, &args![this]).u32(), 0);
        e.mem.set_u32(this, 0x500);
        assert_eq!(e.call(0x00d4_77a0, &args![this]).u32(), 0x501);
    }
}
