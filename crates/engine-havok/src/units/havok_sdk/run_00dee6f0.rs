//! `Havok SDK/run_00dee6f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), thiscall
/// on the object, no stack words.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00dee6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the reference-counted pointer at `this + 0x18` (the engine map has no name; the
/// shape of an `hkRefPtr` assignment), thiscall with one word `object`: adds a reference to
/// `object` when it is not null, removes one from the old pointer when that is not null and
/// stores `object`.
pub fn fn_00dee6f0(e: &mut Engine, this: Ptr, object: u32) {
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    let old = e.mem.u32(this.addr().wrapping_add(0x18));
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(this.addr().wrapping_add(0x18), object);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00de_e6f0, fn_00dee6f0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn rig() -> (Engine, Rc<RefCell<Vec<(u32, Vec<u32>)>>>, u32) {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::default();
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let log = log.clone();
            e.register_double(address, move |_, words| {
                log.borrow_mut().push((address, words.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x20);
        (e, log, this)
    }

    #[test]
    fn a_new_pointer_gains_a_reference_and_the_old_one_loses_one() {
        let (mut e, log, this) = rig();
        e.mem.set_u32(this + 0x18, 0x2000);
        e.call(0x00de_e6f0, &args![this, 0x3000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (ADD_REFERENCE, vec![0x3000]),
                (REMOVE_REFERENCE, vec![0x2000])
            ]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0x3000);
    }

    #[test]
    fn a_null_pointer_only_releases_the_old_one() {
        let (mut e, log, this) = rig();
        e.mem.set_u32(this + 0x18, 0x2000);
        e.call(0x00de_e6f0, &args![this, 0u32]);
        assert_eq!(*log.borrow(), vec![(REMOVE_REFERENCE, vec![0x2000])]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }

    #[test]
    fn nothing_is_released_when_there_was_no_pointer() {
        let (mut e, log, this) = rig();
        e.call(0x00de_e6f0, &args![this, 0x3000u32]);
        assert_eq!(*log.borrow(), vec![(ADD_REFERENCE, vec![0x3000])]);
        assert_eq!(e.mem.u32(this + 0x18), 0x3000);
    }
}
