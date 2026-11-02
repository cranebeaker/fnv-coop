//! `Havok SDK/hkpvehicledefaultsuspensionclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfbff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_bf70`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfbff0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_bf70, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The base object vtable the destructor leaves in `+0`.
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

/// Empties the array at `array` (data `+0`, size `+4`, capacity word `+8`) as the exe does:
/// the size is set to 0, the data freed through the router's allocator (slot `0x10`, data and
/// `capacity & 0x3FFFFFFF` times `element_size` bytes) when bit 31 of the capacity word is
/// clear, then the data pointer is cleared and the capacity word set to `0x80000000`.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00dfbf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of the class whose deleting destructor is `00dfbff0` (the class name is
/// unconfirmed: the map gives `00dfbf70` none), `this` in ECX. Empties two arrays, first
/// the one at `+0x14` (elements of 12 bytes), then the one at `+0x8` (elements of 48 bytes),
/// and puts the base object vtable `0x0102e388` into `+0`.
pub fn fn_00dfbf70(e: &mut Engine, this: Ptr) {
    release_array(e, this.addr() + 0x14, 12);
    release_array(e, this.addr() + 0x8, 48);
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_bff0, deleting_destructor_00dfbff0(Ptr, u8) -> Ptr),
        entry!(0x00df_bf70, fn_00dfbf70(Ptr)),
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
    fn deleting_destructor_00dfbff0_frees_only_when_bit_zero_is_set() {
        check(0x00df_bff0, 0x00df_bf70);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, logging_object, new_log, Log,
    };

    fn install_router(e: &mut Engine, log: &Log) -> u32 {
        let allocator = logging_object(e, log, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    #[test]
    fn both_arrays_are_freed_in_order_and_reset() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_router(&mut e, &log);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this, 0x0aaa_0000);
        e.mem.set_u32(this + 0x14, 0x0100_0100);
        e.mem.set_u32(this + 0x14 + 4, 3);
        e.mem.set_u32(this + 0x14 + 8, 0x4000_0004);
        e.mem.set_u32(this + 0x8, 0x0100_0200);
        e.mem.set_u32(this + 0x8 + 4, 2);
        e.mem.set_u32(this + 0x8 + 8, 0x0000_0007);
        e.call(0x00df_bf70, &args![this]);
        // The first array is freed before the second.
        assert_eq!(
            calls(&log, 0x0900_0010),
            vec![
                vec![allocator, 0x0100_0100, 4 * 12],
                vec![allocator, 0x0100_0200, 7 * 48]
            ]
        );
        for array in [this + 0x14, this + 0x8] {
            assert_eq!(e.mem.u32(array), 0);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }

    #[test]
    fn arrays_that_own_nothing_are_not_freed() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x14 + 8, 0x8000_0010);
        e.mem.set_u32(this + 0x8 + 8, 0x8000_0000);
        e.call(0x00df_bf70, &args![this]);
        assert!(calls(&log, 0x0900_0010).is_empty());
        assert_eq!(e.mem.u32(this + 0x14 + 8), 0x8000_0000);
    }
}
