//! `Havok SDK/hkxvertexvectordatachannelclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator blocks are freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable word every destructor ends with (`hkBaseObject`, `0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00e025a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexVectorDataChannel::_scalar_deleting_destructor_` (Xbox PDB): empties the `hkArray`
/// of 16-byte elements at `+0x8` (data, size `+0xc`, capacity word `+0x10`): the size becomes 0
/// and, unless the capacity word's top bit says the memory is not owned, the data goes back to
/// the memory router's allocator (slot `0x10`, size `(capacity & 0x3fffffff) << 4` bytes). The
/// data pointer is cleared, the capacity word becomes `0x80000000`, the vtable word `0102e388`
/// is written, and when bit 0 of `flags` is set the block itself is freed through the allocator
/// (slot `8`) with the 16-bit size kept at `this + 4`. Returns `this`.
pub fn hkx_vertex_vector_data_channel_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(base + 8);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
    }
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0x8000_0000);
    e.mem.set_u32(base, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(base + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e0_25a0,
        hkx_vertex_vector_data_channel_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0011;
    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0021;

    /// Builds a router whose allocator frees through two doubles (slot 8: a block, slot
    /// `0x10`: array data); both log tagged arguments (1: array data, 2: block).
    fn engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_BLOCK_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let array_log = Rc::clone(&log);
        e.register_double(FREE_ARRAY_DOUBLE, move |_, a| {
            array_log.borrow_mut().extend([1, a[1], a[2]]);
            Ret::default()
        });
        let block_log = Rc::clone(&log);
        e.register_double(FREE_BLOCK_DOUBLE, move |_, a| {
            block_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        (e, log)
    }

    /// An owned array is freed with 16 bytes per element of capacity and the block only with
    /// bit 0 of the flags; an unowned array is reset without a free.
    #[test]
    fn destructor_frees_owned_array_and_block_with_bit_zero() {
        let (mut e, log) = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x20);
        e.mem.set_u32(this + 8, 0x0222_0000);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x10, 5);
        let got = e.call(0x00e0_25a0, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, 0x0222_0000, 0x50]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        log.borrow_mut().clear();
        // The array is now unowned (capacity word 0x80000000): only the block is freed.
        let got = e.call(0x00e0_25a0, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![2, this, 0x20]);
    }
}
