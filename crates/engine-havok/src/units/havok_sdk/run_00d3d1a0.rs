//! `Havok SDK/run_00d3d1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d3d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first and third are never read (`_unused_0`, `_unused_2`), the second is the second collidable (`body_b`) and the last is the contact manager, `manager`. Allocates 0x20 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffff` as a 16-bit word at +0xc and the vtable word `010cf060` at +0. Then calls `00cd7160` with the data at +0x20 of the shape (the word at +0 of the collidable) and the address of the block + 0x10. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d3dbe6.
pub fn fn_00d3d1a0(
    e: &mut Engine,
    _unused_0: u32,
    body_b: u32,
    _unused_2: u32,
    manager: u32,
) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x20u32]).u32();
    e.mem.set_u16(block + 4, 0x20);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block + 8, manager);
    e.mem.set_u16(block + 0xc, 0xffff);
    e.mem.set_u32(block, 0x010c_f060);
    let shape = e.mem.u32(body_b);
    e.call(0x00cd_7160, &args![shape + 0x20, block + 0x10]);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d3_d1a0, fn_00d3d1a0(u32, u32, u32, u32) -> u32)]
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
    fn fn_00d3d1a0_fills_the_block_and_copies_the_shape_data() {
        let (mut e, sizes) = allocator_engine();
        let shape = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cd_7160, move |e, a| {
            // The block already holds the first vtable word while the data is copied.
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[1] - 0x10)));
            Ret::default()
        });
        let block = e
            .call(0x00d3_d1a0, &args![0x1000u32, body, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x20]);
        assert_eq!(
            *seen.borrow(),
            vec![(vec![shape + 0x20, block + 0x10], 0x010c_f060)]
        );
        assert_eq!(e.mem.u16(block + 4), 0x20);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_f060);
        assert_eq!(e.mem.u16(block + 0xc), 0xffff);
    }
}
