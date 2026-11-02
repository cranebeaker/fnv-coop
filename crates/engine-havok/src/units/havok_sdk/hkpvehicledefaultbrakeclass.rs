//! `Havok SDK/hkpvehicledefaultbrakeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the array and the block
/// are freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The base object vtable the destructor leaves in `+0`.
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00dfc080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultBrake::_scalar_deleting_destructor_` (Xbox PDB name), `(this, flags)`. The array at
/// `this + 8` (data `+8`, size `+8 + 4`, capacity word `+8 + 8`) holds the wheel braking properties.
/// Its size is set to 0 and, when bit 31 of the capacity word is clear, the data is freed
/// through the router's allocator (slot `0x10`, data and `capacity & 0x3FFFFFFF` times 12
/// bytes); then the data pointer is cleared and the capacity word set to `0x80000000`. `+0`
/// gets the base object vtable `0x0102e388`. When bit 0 of `flags` is set the block is also
/// freed (allocator slot `8`, with the 16-bit size kept at `this + 4`). Returns `this`.
pub fn hkp_vehicle_default_brake_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let array = this.addr() + 8;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(this.addr() + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00df_c080, hkp_vehicle_default_brake_scalar_deleting_destructor(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };

    /// A router whose allocator is a logging object (slots at `0x0900_0000 + slot`).
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

    /// An object with the array at `+8` holding `data` and the capacity word `capacity`.
    fn object(e: &mut Engine, data: u32, capacity: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        e.mem.set_u32(this, 0x0aaa_0000);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 8 + 4, 9);
        e.mem.set_u32(this + 8 + 8, capacity);
        this
    }

    #[test]
    fn the_array_is_freed_and_the_block_only_when_asked() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_router(&mut e, &log);
        let this = object(&mut e, 0x0123_4560, 0x4000_0005);
        // Bit 31 is clear, so the data is freed: capacity 0x40000005 & 0x3FFFFFFF = 5 elements.
        assert_eq!(
            e.call(0x00df_c080, &args![this, 0xffff_fffeu32]).u32(),
            this
        );
        assert_eq!(
            calls(&log, 0x0900_0010),
            vec![vec![allocator, 0x0123_4560, 5 * 12]]
        );
        assert!(calls(&log, 0x0900_0008).is_empty());
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 8 + 4), 0);
        assert_eq!(e.mem.u32(this + 8 + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        // With bit 0 the block is returned with its stored size.
        assert_eq!(e.call(0x00df_c080, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&log, 0x0900_0008),
            vec![vec![allocator, this, 0x1234]]
        );
    }

    #[test]
    fn an_array_that_owns_nothing_is_not_freed() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        log_double(&mut e, &log, 0x0900_0010, 0);
        let this = object(&mut e, 0x0123_4560, 0x8000_0010);
        e.call(0x00df_c080, &args![this, 0u32]);
        assert!(calls(&log, 0x0900_0010).is_empty());
        assert_eq!(e.mem.u32(this + 8 + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 8 + 4), 0);
    }
}
