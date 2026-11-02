//! `Havok SDK/hkmemorymeshshapeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e014d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_a460`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e014d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_a460, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e014c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::getClassType` (Xbox PDB): takes no arguments and returns the constant
/// `0x0126_e330`, the address of the class record.
pub fn hk_memory_mesh_shape_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_e330
}

// Translated from 00e01500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::getName` (Xbox PDB): the word at `+0x2c` of the shape with its low bit
/// cleared (a flag bit; its meaning is unconfirmed).
pub fn hk_memory_mesh_shape_get_name(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2c) & 0xffff_fffe
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_14c0, hk_memory_mesh_shape_get_class_type() -> u32),
        entry!(0x00e0_14d0, deleting_destructor_00e014d0(Ptr, u8) -> Ptr),
        entry!(0x00e0_1500, hk_memory_mesh_shape_get_name(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00e014d0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_14d0, 0x00e4_a460);
    }

    #[test]
    fn get_class_type_answers_the_class_record_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_14c0, &args![]).u32(), 0x0126_e330);
    }

    #[test]
    fn get_name_clears_the_low_bit_of_the_word_at_0x2c() {
        let mut e = Engine::new();
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0x2c, 0x0123_4567);
        assert_eq!(e.call(0x00e0_1500, &args![shape]).u32(), 0x0123_4566);
        e.mem.set_u32(shape + 0x2c, 0x0123_4566);
        assert_eq!(e.call(0x00e0_1500, &args![shape]).u32(), 0x0123_4566);
    }
}
