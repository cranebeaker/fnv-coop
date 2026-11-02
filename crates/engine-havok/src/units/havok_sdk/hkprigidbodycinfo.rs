//! `Havok SDK/hkprigidbodycinfo.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The one function has no /GS cookie and no exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_bits, store_bits};

/// The 16-byte constant stored at `+0x20` (`0, 0, 0, 1.0` in the exe).
const ROTATION_VECTOR: u32 = 0x010c_71b0;
/// The floats the constructor loads from the exe through the x87 (`FLD dword` / `FSTP dword`):
/// stored to `+0x98`, `+0xA0`, `+0xA4`, `+0xA8` and `+0xAC`, `+0xB0`.
const FLOAT_98: u32 = 0x0101_81bc;
const FLOAT_A0: u32 = 0x0101_6248;
const FLOAT_A4: u32 = 0x0102_31a8;
const FLOAT_A8_AC: u32 = 0x0102_226c;
const FLOAT_B0: u32 = 0x0101_2054;

// Translated from 00cf5ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBodyCinfo::hkpRigidBodyCinfo` (Xbox PDB name), fastcall; answers `this`. Stores the
/// default construction info: zero vectors at `+0x10`, `+0x30`, `+0x40`, `+0x50`, `+0x60`,
/// `+0x70` and `+0x80`, the vector `010c71b0` at `+0x20`, `1.0` at `+0x50`, `+0x64`, `+0x78`,
/// `+0x90` and `+0x9C`, `0` at `+0x94`, the exe's floats through the x87 (a signalling NaN comes
/// out quiet), the bytes `+0xB4`, `+0xB5` and `+0xC` = 1, `+0xE` (16 bits) = `0xFFFF`, `+0xB6` =
/// 2, `+0xB7` = `0xFF`, and zero in the first three words and in `+0xB8` to `+0xBB`.
pub fn hkp_rigid_body_cinfo_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    store_bits(e, me + 0x10, [0; 4]);
    let vector = load_bits(e, ROTATION_VECTOR);
    store_bits(e, me + 0x20, vector);
    for offset in [0x30, 0x40, 0x50, 0x60, 0x70] {
        store_bits(e, me + offset, [0; 4]);
    }
    e.mem.set_f32(me + 0x50, 1.0);
    e.mem.set_f32(me + 0x64, 1.0);
    e.mem.set_f32(me + 0x78, 1.0);
    e.mem.set_u8(me + 0xb4, 1);
    e.mem.set_f32(me + 0x90, 1.0);
    store_bits(e, me + 0x80, [0; 4]);
    e.mem.set_f32(me + 0x94, 0.0);
    let bits = x87_quiet_bits(e.global::<u32>(FLOAT_98));
    e.mem.set_u32(me + 0x98, bits);
    e.mem.set_f32(me + 0x9c, 1.0);
    let bits = x87_quiet_bits(e.global::<u32>(FLOAT_A0));
    e.mem.set_u32(me + 0xa0, bits);
    let bits = x87_quiet_bits(e.global::<u32>(FLOAT_A4));
    e.mem.set_u32(me + 0xa4, bits);
    let bits = x87_quiet_bits(e.global::<u32>(FLOAT_A8_AC));
    e.mem.set_u32(me + 0xa8, bits);
    e.mem.set_u32(me + 0xac, bits);
    let bits = x87_quiet_bits(e.global::<u32>(FLOAT_B0));
    e.mem.set_u32(me + 0xb0, bits);
    e.mem.set_u8(me + 0xb5, 1);
    e.mem.set_u8(me + 0xc, 1);
    e.mem.set_u16(me + 0xe, 0xffff);
    e.mem.set_u8(me + 0xb6, 2);
    e.mem.set_u8(me + 0xb7, 0xff);
    for offset in [0, 4, 8] {
        e.mem.set_u32(me + offset, 0);
    }
    for offset in [0xb8, 0xb9, 0xba, 0xbb] {
        e.mem.set_u8(me + offset, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf_5ea0, hkp_rigid_body_cinfo_construct(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;

    fn exe_floats(e: &mut Engine) {
        e.set_global(ROTATION_VECTOR, 0.0f32);
        e.set_global(ROTATION_VECTOR + 4, 0.0f32);
        e.set_global(ROTATION_VECTOR + 8, 0.0f32);
        e.set_global(ROTATION_VECTOR + 12, 1.0f32);
        e.set_global(FLOAT_98, 0.05f32);
        e.set_global(FLOAT_A0, 0.5f32);
        e.set_global(FLOAT_A4, 0.4f32);
        e.set_global(FLOAT_A8_AC, 200.0f32);
        e.set_global(FLOAT_B0, -1.0f32);
    }

    #[test]
    fn the_defaults_are_stored_00cf5ea0() {
        let mut e = engine();
        exe_floats(&mut e);
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xcd; 0xc0]);
        assert_eq!(e.call(0x00cf_5ea0, &args![this]).u32(), this);
        let word = |offset: u32| e.mem.u32(this + offset);
        // A byte the function does not store keeps its fill (the sub-word fields at +0xC and
        // +0xE are stored by size).
        assert_eq!([word(0), word(4), word(8)], [0, 0, 0]);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u8(this + 0xd), 0xcd);
        assert_eq!(e.mem.u16(this + 0xe), 0xffff);
        for offset in [0x10, 0x14, 0x18, 0x1c] {
            assert_eq!(word(offset), 0);
        }
        assert_eq!(
            [word(0x20), word(0x24), word(0x28), word(0x2c)],
            [0, 0, 0, 0x3f80_0000]
        );
        for offset in (0x30..0x80).step_by(4) {
            let expected = if [0x50, 0x64, 0x78].contains(&offset) {
                0x3f80_0000
            } else {
                0
            };
            assert_eq!(word(offset), expected, "offset {offset:#x}");
        }
        for offset in [0x80, 0x84, 0x88, 0x8c, 0x94] {
            assert_eq!(word(offset), 0, "offset {offset:#x}");
        }
        assert_eq!(word(0x90), 0x3f80_0000);
        assert_eq!(word(0x98), 0.05f32.to_bits());
        assert_eq!(word(0x9c), 0x3f80_0000);
        assert_eq!(word(0xa0), 0.5f32.to_bits());
        assert_eq!(word(0xa4), 0.4f32.to_bits());
        assert_eq!(word(0xa8), 200.0f32.to_bits());
        assert_eq!(word(0xac), 200.0f32.to_bits());
        assert_eq!(word(0xb0), (-1.0f32).to_bits());
        assert_eq!(
            [
                e.mem.u8(this + 0xb4),
                e.mem.u8(this + 0xb5),
                e.mem.u8(this + 0xb6),
                e.mem.u8(this + 0xb7),
                e.mem.u8(this + 0xb8),
                e.mem.u8(this + 0xb9),
                e.mem.u8(this + 0xba),
                e.mem.u8(this + 0xbb),
            ],
            [1, 1, 2, 0xff, 0, 0, 0, 0]
        );
    }

    #[test]
    fn a_signalling_nan_constant_comes_out_quiet_00cf5ea0() {
        let mut e = engine();
        exe_floats(&mut e);
        e.set_global(FLOAT_98, 0x7f80_0001u32);
        e.set_global(FLOAT_A8_AC, 0x7fa0_0000u32);
        e.set_global(FLOAT_B0, 0xff80_0123u32);
        let this = e.mem.alloc(0xc0);
        e.call(0x00cf_5ea0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x98), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0xa8), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0xac), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0xb0), 0xffc0_0123);
        // The next field is not touched by the one before it.
        assert_eq!(e.mem.u32(this + 0x9c), 0x3f80_0000);
    }
}
