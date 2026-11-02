//! `Havok SDK/hkxtexturefileclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkStringPtr::~hkStringPtr` (named by Ghidra from the Xbox PDB; `this` only).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// The vtable word every destructor ends with (`hkBaseObject`, `0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00e01870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxTextureFile::_scalar_deleting_destructor_` (Xbox PDB): destroys the three `hkStringPtr`
/// members at `+0x10`, `+0xc` and `+0x8` (`00cb0c00`, in that order), writes the vtable word
/// `0102e388` at `this`, and when bit 0 of `flags` is set frees the block through the memory
/// router's allocator (slot `8`) with the 16-bit size kept at `this + 4`. Returns `this`.
pub fn hkx_texture_file_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x10]);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0xc]);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x8]);
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
        0x00e0_1870,
        hkx_texture_file_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
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

    /// Installs doubles for the string destructor and a memory router whose allocator frees
    /// through a double; they push tagged arguments on the log (1: string destructor, 2: free).
    fn install(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(STRING_PTR_DESTRUCT, move |_, a| {
            string_log.borrow_mut().extend([1, a[0]]);
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

    /// The three strings are destroyed in the exe's order and the vtable word is written;
    /// the block is freed only when bit 0 of the flags is set.
    #[test]
    fn destructor_destroys_strings_then_frees_only_with_bit_zero() {
        let mut e = engine();
        let log = install(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x2c);
        let got = e.call(0x00e0_1870, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(
            *log.borrow(),
            vec![1, this + 0x10, 1, this + 0xc, 1, this + 8]
        );
        log.borrow_mut().clear();
        let got = e.call(0x00e0_1870, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            *log.borrow(),
            vec![1, this + 0x10, 1, this + 0xc, 1, this + 8, 2, this, 0x2c]
        );
    }
}
