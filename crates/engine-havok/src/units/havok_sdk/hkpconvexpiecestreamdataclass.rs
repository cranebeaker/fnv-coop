//! `Havok SDK/hkpconvexpiecestreamdataclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dffd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_fca0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dffd50(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_fca0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dffca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the piece stream (fastcall, `this` in ECX; the class is
/// unconfirmed). Releases the three arrays at `+0x20`, `+0x14` and `+0x8` (each data pointer,
/// size, capacity word): the size is cleared, the block is returned to the memory router's
/// allocator (slot `0x10`: block, bytes) when the capacity word's sign bit is clear, then the
/// data pointer is cleared and the capacity word set to `0x80000000` (nothing owned). Ends by
/// storing the vtable `0102e388` at `this`.
pub fn destruct_00dffca0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for array in [0x20, 0x14, 0x8] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(this + array + 8);
        e.mem.set_u32(this + array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(this + array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        e.mem.set_u32(this + array, 0);
        e.mem.set_u32(this + array + 8, 0x8000_0000);
    }
    e.mem.set_u32(this, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_fca0, destruct_00dffca0(Ptr)),
        entry!(0x00df_fd50, deleting_destructor_00dffd50(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00dffd50_frees_only_when_bit_zero_is_set() {
        check(0x00df_fd50, 0x00df_fca0);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Frees = Rc<RefCell<Vec<(u32, u32)>>>;

    /// A memory router whose allocator logs every free as `(block, bytes)`.
    fn rig() -> (Engine, Frees) {
        let mut e = Engine::new();
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&frees);
        e.register_double(0x0900_0010, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, frees)
    }

    #[test]
    fn the_destructor_frees_owned_arrays_and_resets_all_three() {
        let (mut e, frees) = rig();
        let this = e.mem.alloc(0x30);
        // Owned arrays at +0x20 and +0x8, one not owned at +0x14.
        for (offset, data, size, capacity) in [
            (0x20, 0x1000, 3, 0x40),
            (0x14, 0x2000, 2, 0x8000_0010),
            (0x8, 0x3000, 1, 0x4000_0005),
        ] {
            e.mem.set_u32(this + offset, data);
            e.mem.set_u32(this + offset + 4, size);
            e.mem.set_u32(this + offset + 8, capacity);
        }
        e.call(0x00df_fca0, &args![this]);
        assert_eq!(*frees.borrow(), vec![(0x1000, 0x100), (0x3000, 0x14)]);
        for offset in [0x20, 0x14, 0x8] {
            assert_eq!(e.mem.u32(this + offset), 0);
            assert_eq!(e.mem.u32(this + offset + 4), 0);
            assert_eq!(e.mem.u32(this + offset + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}
