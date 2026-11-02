//! `Havok SDK/hkpcallbackconstraintmotor.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpCallbackConstraintMotor`: a constraint motor of 0x28 bytes (the Xbox PDB has no record of
//! the Havok classes; the field names are unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator `clone` allocates from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable the copy constructor leaves in the new motor.
const MOTOR_VTABLE: u32 = 0x010e_83b4;
/// The two base-class vtables the copy constructor stores on the way.
const BASE_VTABLE: u32 = 0x010c_b76c;
const MOTOR_BASE_VTABLE: u32 = 0x010c_b780;

// Translated from 00e483d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCallbackConstraintMotor::clone` (engine map; fastcall, `this` the motor to copy):
/// allocates 0x28 bytes from the memory router's allocator (slot 4), stores the size 0x28 as a
/// `u16` at `+4` and builds the copy there with `fn_00e48400`. Returns the new motor.
pub fn hk_callback_constraint_motor_clone(e: &mut Engine, this: Ptr) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x28u32]).u32();
    e.mem.set_u16(block + 4, 0x28);
    fn_00e48400(e, Ptr::new(block), this)
}

// Translated from 00e48400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy constructor (no engine-map name; thiscall, `this` the new motor, one stack word,
/// the motor to copy). Stores the vtables `010cb76c`, `010cb780` and finally `010e83b4`,
/// the `u16` 1 at `+6`, the byte at `+8`, the floats at `+0xC` and `+0x10` (through the x87
/// stack, so signalling NaNs are quieted) and the words `+0x14` to `+0x24`. Returns `this`.
pub fn fn_00e48400(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    let (this, source) = (this.addr(), source.addr());
    e.mem.set_u32(this, BASE_VTABLE);
    e.mem.set_u16(this + 6, 1);
    let flag = e.mem.u8(source + 8);
    e.mem.set_u32(this, MOTOR_BASE_VTABLE);
    e.mem.set_u8(this + 8, flag);
    let first = x87_quiet_bits(e.mem.u32(source + 0xc));
    e.mem.set_u32(this + 0xc, first);
    let second = x87_quiet_bits(e.mem.u32(source + 0x10));
    e.mem.set_u32(this, MOTOR_VTABLE);
    e.mem.set_u32(this + 0x10, second);
    for offset in [0x14, 0x18, 0x1c, 0x20, 0x24] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(this + offset, word);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_83d0, hk_callback_constraint_motor_clone(Ptr) -> u32),
        entry!(0x00e4_8400, fn_00e48400(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    const ALLOCATE_SLOT: u32 = 0x0900_0601;

    fn engine_with_allocator() -> Engine {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_SLOT, |e, a| {
            assert_eq!(a[1], 0x28);
            Ret {
                eax: e.mem.alloc(0x28),
                ..Ret::default()
            }
        });
        e
    }

    fn source_motor(e: &mut Engine) -> u32 {
        let source = e.mem.alloc(0x28);
        e.mem.set_u8(source + 8, 0x5a);
        e.mem.set_u32(source + 0xc, 0x7f80_0001);
        e.mem.set_u32(source + 0x10, 0x3f80_0000);
        for (index, offset) in [0x14, 0x18, 0x1c, 0x20, 0x24].into_iter().enumerate() {
            e.mem.set_u32(source + offset, 0x1000 + index as u32);
        }
        source
    }

    #[test]
    fn the_copy_constructor_copies_the_fields_and_quiets_nans() {
        let mut e = engine_with_allocator();
        let source = source_motor(&mut e);
        let target = e.mem.alloc(0x28);
        assert_eq!(
            fn_00e48400(&mut e, Ptr::new(target), Ptr::new(source)),
            target
        );
        assert_eq!(e.mem.u32(target), MOTOR_VTABLE);
        assert_eq!(e.mem.u16(target + 6), 1);
        assert_eq!(e.mem.u8(target + 8), 0x5a);
        assert_eq!(e.mem.u32(target + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(target + 0x10), 0x3f80_0000);
        for index in 0..5u32 {
            assert_eq!(e.mem.u32(target + 0x14 + 4 * index), 0x1000 + index);
        }
    }

    #[test]
    fn clone_allocates_a_block_of_0x28_bytes_and_stores_its_size() {
        let mut e = engine_with_allocator();
        let source = source_motor(&mut e);
        let copy = hk_callback_constraint_motor_clone(&mut e, Ptr::new(source));
        assert_ne!(copy, source);
        assert_eq!(e.mem.u16(copy + 4), 0x28);
        assert_eq!(e.mem.u16(copy + 6), 1);
        assert_eq!(e.mem.u32(copy), MOTOR_VTABLE);
        assert_eq!(e.mem.u32(copy + 0x24), 0x1004);
    }
}
