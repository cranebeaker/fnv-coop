//! `Havok SDK/run_00dd8fa0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dd8fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 8 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6 and the vtable word `010d7c58` at +0. Returns the block. Ghidra made no function here; the address is stored in the data word at 011b8334.
pub fn fn_00dd8fa0(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![8u32]).u32();
    e.mem.set_u16(block + 4, 8);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, 0x010d_7c58);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dd_8fa0, fn_00dd8fa0() -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    #[test]
    fn fn_00dd8fa0_allocates_and_initialises_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e.call(0x00dd_8fa0, &args![]).u32();
        assert_eq!(*sizes.borrow(), vec![8]);
        assert_eq!(e.mem.u16(block + 4), 8);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), 0x010d_7c58);
    }
}
