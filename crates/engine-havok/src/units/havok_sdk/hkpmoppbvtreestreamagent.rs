//! `Havok SDK/hkpmoppbvtreestreamagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The collector's vtable (`0104e328`) the destructor writes back.
const VTABLE_COLLECTOR: u32 = 0x0104_e328;

// Translated from 00d2dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentFlipBodyCollector::_scalar_deleting_destructor_` (Xbox PDB), `RET 4`:
/// sets the vtable `0104e328` and, when bit 0 of `flags` is set, returns the 12-byte block to
/// the memory router's allocator (slot `8`). Returns `this`.
pub fn hkp_symmetric_agent_flip_body_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_COLLECTOR);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, 0xcu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d2_dc40,
        hkp_symmetric_agent_flip_body_collector_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT: u32 = 0x0900_0011;

    #[test]
    fn the_destructor_frees_twelve_bytes_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(FREE_SLOT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        let got = e.call(0x00d2dc40, &args![this, 0xfeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), VTABLE_COLLECTOR);
        assert!(log.borrow().is_empty());
        let got = e.call(0x00d2dc40, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![vec![allocator, this, 0xc]]);
    }
}
