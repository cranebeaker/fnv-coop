//! `Havok SDK/hkpserializeddisplaymarkerlistclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dff080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_f010`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dff080(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f010, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall, no further words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The vtable the destructor sets first (`010e66fc`) and the base vtable it ends with (`0102e388`).
const DERIVED_VTABLE: u32 = 0x010e_66fc;
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00dff010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the serialized display marker list (`ECX` is `this`): sets the derived vtable,
/// removes a reference from each of the `[this + 0xC]` objects of the array at `[this + 8]`,
/// then empties the array: its size becomes 0 and, when the capacity word at `+0x10` is not
/// negative (the array owns its block), the block is freed through slot `0x10` of the memory
/// router's allocator with `capacity * 4` bytes. The array is left as data 0, capacity
/// `0x80000000` and the base vtable is set.
pub fn fn_00dff010(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, DERIVED_VTABLE);
    if e.mem.i32(this + 0xc) > 0 {
        let mut index: u32 = 0;
        loop {
            let object = e
                .mem
                .u32(e.mem.u32(this + 8).wrapping_add(index.wrapping_mul(4)));
            e.call(REMOVE_REFERENCE, &args![object]);
            index = index.wrapping_add(1);
            if index as i32 >= e.mem.i32(this + 0xc) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(this + 0x10);
    e.mem.set_u32(this + 0xc, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(this + 8);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0x10, 0x8000_0000);
    e.mem.set_u32(this, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_f010, fn_00dff010(Ptr)),
        entry!(0x00df_f080, deleting_destructor_00dff080(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, new_log, Log};
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
    fn deleting_destructor_00dff080_frees_only_when_bit_zero_is_set() {
        check(0x00df_f080, 0x00df_f010);
    }

    const ARRAY_FREE_DOUBLE: u32 = 0x0900_0014;

    /// A memory router whose allocator frees arrays through a logging double (slot `0x10`);
    /// returns the allocator.
    fn array_router(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ARRAY_FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(e, log, ARRAY_FREE_DOUBLE, 0);
        allocator
    }

    #[test]
    fn destructor_00dff010_releases_the_objects_and_frees_an_owned_array() {
        let mut e = engine();
        let log = new_log();
        let allocator = array_router(&mut e, &log);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 4, 0x2222);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 3);
        e.call(0x00df_f010, &args![this]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![0x1111], vec![0x2222]]
        );
        assert_eq!(
            calls(&log, ARRAY_FREE_DOUBLE),
            vec![vec![allocator, data, 12]]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(
            [
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc),
                e.mem.u32(this + 0x10)
            ],
            [0, 0, 0x8000_0000]
        );
    }

    #[test]
    fn destructor_00dff010_keeps_a_block_the_array_does_not_own() {
        let mut e = engine();
        let log = new_log();
        array_router(&mut e, &log);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x5000);
        e.mem.set_u32(this + 0x10, 0x8000_0003);
        e.call(0x00df_f010, &args![this]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert!(calls(&log, ARRAY_FREE_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
    }
}
