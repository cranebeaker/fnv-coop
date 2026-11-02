//! `Havok SDK/run_00d87bd0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d87bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty `thiscall` with one stack word (`RET 4`): does nothing. The class is unconfirmed.
pub fn fn_00d87bd0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d87be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty `thiscall` with one stack word (`RET 4`): does nothing, as `00d87bd0`. The class is
/// unconfirmed.
pub fn fn_00d87be0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d87b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x1c bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, the vtable word `010d3cc0` at +0, zero words at +8, +0xc, +0x10 and +0x14 and `0x80000000` at +0x18 (an empty array that owns no storage). Returns the block. Ghidra made no function here; the address is stored in the data word at 011b6ca4.
pub fn fn_00d87b90(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, 0x010d_3cc0);
    e.mem.set_u32(block + 0x10, 0);
    e.mem.set_u32(block + 0x14, 0);
    e.mem.set_u32(block + 0x18, 0x8000_0000);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_7bd0, fn_00d87bd0(Ptr, u32)),
        entry!(0x00d8_7be0, fn_00d87be0(Ptr, u32)),
        entry!(0x00d8_7b90, fn_00d87b90() -> u32),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_empty_functions_change_nothing_and_call_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00d8_7bd0, &args![this, 0x55u32]);
        e.call(0x00d8_7be0, &args![this, 0x66u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
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
    fn fn_00d87b90_allocates_and_initialises_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e.call(0x00d8_7b90, &args![]).u32();
        assert_eq!(*sizes.borrow(), vec![0x1c]);
        assert_eq!(e.mem.u16(block + 4), 0x1c);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), 0x010d_3cc0);
        assert_eq!(e.mem.u32(block + 0x18), 0x8000_0000);
        for offset in [8, 0xc, 0x10, 0x14] {
            assert_eq!(e.mem.u32(block + offset), 0);
        }
    }
}
