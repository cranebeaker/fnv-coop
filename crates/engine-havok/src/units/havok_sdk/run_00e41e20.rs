//! `Havok SDK/run_00e41e20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::release_array;

// Translated from 00e41e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`). It writes the vtable word
/// `010e58c8`, releases the `hkArray`-shaped triple of 4-byte elements at `+8` (size cleared,
/// storage returned through the allocator's slot `0x10` unless the capacity word is negative,
/// data cleared, capacity word `0x80000000`) and writes the vtable word `0102e388`. The class is
/// unconfirmed.
pub fn fn_00e41e20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010e_58c8);
    release_array(e, base + 8, 4);
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e4_1e20, fn_00e41e20(Ptr))]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An owned array is freed (capacity times 4 bytes) and cleared; the vtable word ends as
    /// the second value.
    #[test]
    fn fn_00e41e20_releases_the_array() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0011);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(0x0900_0011, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0xd00d);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x10, 6);
        e.call(0x00e4_1e20, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![allocator, 0xd00d, 24]]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        // Not owned any more: a second run frees nothing.
        e.call(0x00e4_1e20, &args![this]);
        assert_eq!(log.borrow().len(), 1);
    }
}
