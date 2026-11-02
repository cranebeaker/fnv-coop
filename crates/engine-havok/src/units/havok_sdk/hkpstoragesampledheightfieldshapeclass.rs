//! `Havok SDK/hkpstoragesampledheightfieldshapeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d49630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_96d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d49630(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_96d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d49600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getHeightAtImpl` of the storage height field (virtual, slot `0x34`; `this` first, then the
/// column `x` and the row `z`, `stdcall`): the sample at index `((rows - z) * columns + x) * 3`
/// (32-bit wrapping) of the `float` array at `+0x60`, read at an offset of 8 bytes (the third
/// `float` of the entry), plus the `float` at `+0x6c`. The sum is formed in double precision,
/// rounded to `float` and returned in `ST0`. The class is unconfirmed.
pub fn get_height_at_impl_00d49600(e: &mut Engine, this: Ptr, x: u32, z: u32) -> f32 {
    let object = this.addr();
    let rows = e.mem.u32(object + 0x68);
    let samples = e.mem.u32(object + 0x60);
    let columns = e.mem.u32(object + 0x10);
    let index = rows.wrapping_sub(z).wrapping_mul(columns).wrapping_add(x);
    let entry = index.wrapping_add(index.wrapping_mul(2));
    let at = samples.wrapping_add(entry.wrapping_mul(4)).wrapping_add(8);
    let sum = f64::from(e.mem.f32(at)) + f64::from(e.mem.f32(object + 0x6c));
    sum as f32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_9600, get_height_at_impl_00d49600(Ptr, u32, u32) -> f32),
        entry!(0x00d4_9630, deleting_destructor_00d49630(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d49630_frees_only_when_bit_zero_is_set() {
        check(0x00d4_9630, 0x00d4_96d0);
    }
}

#[cfg(test)]
mod height_tests {
    use super::*;

    #[test]
    fn get_height_at_impl_00d49600_indexes_the_third_float_of_the_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let samples = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x60, samples);
        e.mem.set_u32(this + 0x10, 5);
        e.mem.set_u32(this + 0x68, 3);
        e.mem.set_f32(this + 0x6c, 0.5);
        // Index (3 - 2) * 5 + 1 = 6, entry 18, its third float is float 20.
        e.mem.set_f32(samples + 4 * 20, 2.25);
        let height = e.call(0x00d4_9600, &args![this, 1u32, 2u32]).f32();
        assert_eq!(height, 2.75);
        // z = 3 gives index 1: entry 3, its third float is float 5.
        e.mem.set_f32(samples + 4 * 5, -1.5);
        let height = e.call(0x00d4_9600, &args![this, 1u32, 3u32]).f32();
        assert_eq!(height, -1.0);
    }
}
