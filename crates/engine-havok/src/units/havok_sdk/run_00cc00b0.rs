//! `Havok SDK/run_00cc00b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vtable `010c9314` is `hkOstream`'s (RTTI `.?AVhkOstream@@` confirmed through its
//! complete object locator). The class is 0xC bytes: an `hkReferencedObject` header and the
//! `hkStreamWriter*` it writes through at +8.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkOstream`'s vtable.
const OSTREAM_VTABLE: u32 = 0x010c_9314;
/// The `hkReferencedObject` base vtable that the destructor leaves behind.
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;
/// `hkReferencedObject::addReferenceLockUnchecked` and `removeReferenceLockUnchecked`
/// (Xbox PDB names, `this` = the object).
const ADD_REFERENCE: u32 = 0x00c9_03a0;
const REMOVE_REFERENCE: u32 = 0x00c9_03b0;

layout! {
    /// `hkOstream` (Xbox PDB), 0xC bytes.
    pub struct HkOstream: 0xC {
        /// Vtable (`010c9314`).
        0x00 vftable: u32,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_writer` (Xbox PDB): `hkStreamWriter*`.
        0x08 m_writer: Ptr,
    }
}

// Translated from 00cc00b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::hkOstream(hkStreamWriter*)` (Xbox PDB name of the constructor, body
/// confirmed): reference count 1, the vtable, and the writer, which gets a reference.
pub fn hk_ostream_construct(e: &mut Engine, this: Ptr<HkOstream>, writer: Ptr) -> Ptr<HkOstream> {
    e.set(this, HkOstream::m_referenceCount, 1);
    e.set(this, HkOstream::vftable, OSTREAM_VTABLE);
    e.set(this, HkOstream::m_writer, writer);
    if writer.addr() != 0 {
        e.call(ADD_REFERENCE, &args![writer]);
    }
    this
}

// Translated from 00cc00e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::~hkOstream` (Xbox PDB name, body confirmed): drops the writer's reference
/// and leaves the `hkReferencedObject` vtable.
pub fn hk_ostream_destruct(e: &mut Engine, this: Ptr<HkOstream>) {
    e.set(this, HkOstream::vftable, OSTREAM_VTABLE);
    let writer = e.get(this, HkOstream::m_writer);
    if writer.addr() != 0 {
        e.call(REMOVE_REFERENCE, &args![writer]);
    }
    e.set(this, HkOstream::vftable, REFERENCED_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cc_00b0,
            hk_ostream_construct(Ptr<HkOstream>, Ptr) -> Ptr<HkOstream>
        ),
        entry!(0x00cc_00e0, hk_ostream_destruct(Ptr<HkOstream>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, u32)>>>;

    fn rig() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a[0]));
                Ret::default()
            });
        }
        (e, log)
    }

    #[test]
    fn construct_references_a_writer_only_when_there_is_one() {
        let (mut e, log) = rig();
        let this = e.new_object::<HkOstream>();
        let writer = e.mem.alloc(0x10);
        let got = e.call(0x00cc_00b0, &args![this, writer]).u32();
        assert_eq!(got, this.addr());
        assert_eq!(e.get(this, HkOstream::m_referenceCount), 1);
        assert_eq!(e.get(this, HkOstream::vftable), OSTREAM_VTABLE);
        assert_eq!(e.get(this, HkOstream::m_writer).addr(), writer);
        assert_eq!(*log.borrow(), vec![(ADD_REFERENCE, writer)]);
        log.borrow_mut().clear();
        e.call(0x00cc_00b0, &args![this, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.get(this, HkOstream::m_writer).addr(), 0);
    }

    #[test]
    fn destruct_releases_the_writer_and_restores_the_base_vtable() {
        let (mut e, log) = rig();
        let this = e.new_object::<HkOstream>();
        let writer = e.mem.alloc(0x10);
        e.set(this, HkOstream::m_writer, Ptr::new(writer));
        e.call(0x00cc_00e0, &args![this]);
        assert_eq!(*log.borrow(), vec![(REMOVE_REFERENCE, writer)]);
        assert_eq!(e.get(this, HkOstream::vftable), REFERENCED_OBJECT_VTABLE);
        log.borrow_mut().clear();
        e.set(this, HkOstream::m_writer, Ptr::new(0));
        e.call(0x00cc_00e0, &args![this]);
        assert!(log.borrow().is_empty());
    }
}
