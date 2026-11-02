//! `Havok SDK/hkppositionconstraintmotor.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtables the copy goes through: the base object's (`010cb76c`), the motor's (`010cb780`)
/// and the position motor's final one (`010cb798`).
const BASE_VTABLE: u32 = 0x010c_b76c;
const MOTOR_VTABLE: u32 = 0x010c_b780;
const POSITION_MOTOR_VTABLE: u32 = 0x010c_b798;

// Translated from 00cdf5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPositionConstraintMotor::clone` (engine map, `__fastcall`): takes 0x24 bytes from the
/// allocator (slot 4), writes the vtables in the order the constructors chain (`010cb76c`,
/// then `010cb780`, then `010cb798`), the 16-bit size `0x24` at `+4`, the reference count 1 at
/// `+6`, copies the byte at `+8` and the floats at `+0xc` .. `+0x20` (each through the x87
/// stack, so signalling NaNs come out quiet) and returns the new object.
pub fn hkp_position_constraint_motor_clone(e: &mut Engine, this: Ptr) -> Ptr {
    let from = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let copy = e.vcall(allocator, 4, &args![0x24u32]).u32();
    e.mem.set_u32(copy, BASE_VTABLE);
    e.mem.set_u16(copy + 4, 0x24);
    e.mem.set_u16(copy + 6, 1);
    let flag = e.mem.u8(from + 8);
    e.mem.set_u32(copy, MOTOR_VTABLE);
    e.mem.set_u8(copy + 8, flag);
    let first = x87_quiet_bits(e.mem.u32(from + 0xc));
    e.mem.set_u32(copy + 0xc, first);
    let second = x87_quiet_bits(e.mem.u32(from + 0x10));
    e.mem.set_u32(copy, POSITION_MOTOR_VTABLE);
    e.mem.set_u32(copy + 0x10, second);
    for offset in [0x14, 0x18, 0x1c, 0x20] {
        let bits = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(copy + offset, bits);
    }
    Ptr::new(copy)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_f5b0,
        hkp_position_constraint_motor_clone(Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_ALLOCATE: u32 = 0x0900_0301;

    fn setup() -> Engine {
        let mut e = Engine::new();
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, SLOT_ALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(SLOT_ALLOCATE, |e, a| {
            assert_eq!(a[1], 0x24);
            Ret {
                eax: e.mem.alloc(0x24),
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn clone_copies_the_settings_and_quiets_signalling_nans() {
        let mut e = setup();
        let motor = e.mem.alloc(0x24);
        e.mem.set_u8(motor + 8, 1);
        let values: [u32; 6] = [
            1.5f32.to_bits(),
            (-2.25f32).to_bits(),
            0x7f80_0001,
            0xff80_1234,
            0x7fc0_0000,
            0.0f32.to_bits(),
        ];
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u32(motor + 0xc + 4 * index as u32, *value);
        }
        let copy = e.call(0x00cd_f5b0, &args![motor]).u32();
        assert_ne!(copy, motor);
        assert_eq!(e.mem.u32(copy), POSITION_MOTOR_VTABLE);
        assert_eq!(e.mem.u16(copy + 4), 0x24);
        assert_eq!(e.mem.u16(copy + 6), 1);
        assert_eq!(e.mem.u8(copy + 8), 1);
        let expected = [
            1.5f32.to_bits(),
            (-2.25f32).to_bits(),
            0x7fc0_0001,
            0xffc0_1234,
            0x7fc0_0000,
            0,
        ];
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(copy + 0xc + 4 * index as u32), *value);
        }
        // The vtable chain ends at the position motor's table; the motor itself is untouched.
        assert_eq!(e.mem.u32(motor + 0x14), 0x7f80_0001);
    }
}
