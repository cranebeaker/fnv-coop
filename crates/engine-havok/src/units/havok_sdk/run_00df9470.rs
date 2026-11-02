//! `Havok SDK/run_00df9470` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00df9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the class whose vtable `010dbaf8` has this in slot 0
/// (`hkCpuWorldRayCastCollector`'s vtable by the gap list; the class is otherwise unconfirmed):
/// `this` in `ECX` and one stack word, `flags` (`RET 4`; only bit 0 of its low byte is tested).
/// Rewrites the four vtable words (`010c3bb8` at +0, `010ccf8c` at +4, `010ccfa0` at +8,
/// `010ccfac` at +0xc) and, when bit 0 of `flags` is set, frees the block of 0x80 bytes through the
/// memory router's allocator (virtual function at +8 of the object at router + 0x10). Returns
/// `this`. Ghidra holds this code in the body of `00df8950`.
pub fn fn_00df9470(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0xc, 0x010c_cfac);
    e.mem.set_u32(me + 8, 0x010c_cfa0);
    e.mem.set_u32(me + 4, 0x010c_cf8c);
    e.mem.set_u32(me, 0x010c_3bb8);
    if flags & 1 != 0 {
        let router = e.call(0x00c8_5750, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![me, 0x80u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00df_9470, fn_00df9470(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A memory router whose allocator's free function (slot at +8) logs its words.
    fn freeing_engine() -> (Engine, Rc<RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0900_0006);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&frees);
        e.register_double(0x0900_0006, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, frees)
    }

    #[test]
    fn fn_00df9470_rewrites_the_vtables_and_frees_on_request() {
        let (mut e, frees) = freeing_engine();
        let this = e.mem.alloc(0x80);
        // Bit 0 clear: only the vtable words change.
        assert_eq!(e.call(0x00df_9470, &args![this, 0x2u8]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_3bb8);
        assert_eq!(e.mem.u32(this + 4), 0x010c_cf8c);
        assert_eq!(e.mem.u32(this + 8), 0x010c_cfa0);
        assert_eq!(e.mem.u32(this + 0xc), 0x010c_cfac);
        assert!(frees.borrow().is_empty());
        // Bit 0 set: the block is returned with its size.
        e.call(0x00df_9470, &args![this, 0x1u8]);
        let allocator_words = frees.borrow().clone();
        assert_eq!(allocator_words.len(), 1);
        assert_eq!(&allocator_words[0][1..], &[this, 0x80]);
    }
}
