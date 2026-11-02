//! `Havok SDK/hkpstiffspringconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpStiffSpringConstraintData` keeps its constraint atoms from `+0x10` (`0x38` bytes of
//! them: a bridge-less set whose first atom has type 3 at `+0x10`, another with type 6 at
//! `+0x40`), the spring's rest length in the `float` at `+0x44` and a zeroed pair of vectors
//! at `+0x20` and `+0x30` (the pivots in the two bodies' spaces).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl `(atoms, size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// The class's vtable.
const STIFF_SPRING_VTABLE: u32 = 0x010d_0aac;

// Translated from 00d4f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringConstraintData::getConstraintInfo` (named in the engine map): fills `info` from
/// the `0x38` bytes of atoms at `this + 0x10` (`getConstraintInfoUtil`).
pub fn hkp_stiff_spring_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x38u32, info],
    );
}

// Translated from 00d4f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringConstraintData::getRuntimeInfo` (named in the engine map; a virtual method,
/// `this` unused): `(size, count)` is written to `info` as `(8, 1)` when `flag` is not zero and
/// as `(0, 0)` otherwise.
pub fn hkp_stiff_spring_constraint_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    flag: u8,
    info: Ptr,
) {
    let (size, count) = if flag != 0 { (8, 1) } else { (0, 0) };
    e.mem.set_u32(info.addr() + 4, count);
    e.mem.set_u32(info.addr(), size);
}

// Translated from 00d4f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringConstraintData::isValid` (named in the engine map): the byte at `valid`
/// becomes 1 when the `float` at `+0x44` is above 0 (an x87 compare, so a NaN is not), else 0.
/// Returns `valid`.
pub fn hkp_stiff_spring_constraint_data_is_valid(e: &mut Engine, this: Ptr, valid: Ptr) -> u32 {
    let positive = 0.0 < e.mem.f32(this.addr() + 0x44);
    e.mem.set_u8(valid.addr(), u8::from(positive));
    valid.addr()
}

// Translated from 00d4f0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringConstraintData::getType` (named in the engine map): the constraint type, 8.
pub fn hkp_stiff_spring_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    8
}

// Translated from 00d4f0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringConstraintData::hkpStiffSpringConstraintData` (named in the engine map; the
/// register is `this`): the 16-bit count at `+6` is 1, the word at `+8` is 0, the vtable is
/// installed, the atom type words at `+0x10` and `+0x40` are 3 and 6, the rest length at `+0x44`
/// is 0.0 and the two vectors at `+0x20` and `+0x30` are zeroed. Returns `this`.
pub fn hkp_stiff_spring_constraint_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.mem.set_u16(object + 6, 1);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object, STIFF_SPRING_VTABLE);
    e.mem.set_u16(object + 0x10, 3);
    e.mem.set_u16(object + 0x40, 6);
    e.mem.set_f32(object + 0x44, 0.0);
    for lane in 0..8 {
        e.mem.set_u32(object + 0x20 + 4 * lane, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_f070,
            hkp_stiff_spring_constraint_data_get_constraint_info(Ptr, u32)
        ),
        entry!(
            0x00d4_f090,
            hkp_stiff_spring_constraint_data_get_runtime_info(Ptr, u8, Ptr)
        ),
        entry!(
            0x00d4_f0c0,
            hkp_stiff_spring_constraint_data_is_valid(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00d4_f0e0,
            hkp_stiff_spring_constraint_data_get_type(Ptr) -> u32
        ),
        entry!(
            0x00d4_f0f0,
            hkp_stiff_spring_constraint_data_construct(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_info_hands_the_atoms_to_the_info_util() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register_double(GET_CONSTRAINT_INFO_UTIL, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        let info = e.mem.alloc(0x20);
        e.call(0x00d4_f070, &args![this, info]);
        let log = e.call_log.clone().unwrap();
        let seen: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == GET_CONSTRAINT_INFO_UTIL)
            .collect();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, vec![this + 0x10, 0x38, info]);
    }

    #[test]
    fn runtime_info_is_eight_bytes_and_one_slot_only_with_the_flag() {
        let mut e = Engine::new();
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 99);
        e.mem.set_u32(info + 4, 99);
        e.call(0x00d4_f090, &args![0x1000u32, 1u8, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (8, 1));
        e.call(0x00d4_f090, &args![0x1000u32, 0u8, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
    }

    #[test]
    fn is_valid_needs_a_rest_length_above_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        let valid = e.mem.alloc(4);
        for (length, want) in [
            (1.5f32, 1u8),
            (0.0, 0),
            (-0.0, 0),
            (-2.0, 0),
            (f32::NAN, 0),
            (f32::INFINITY, 1),
        ] {
            e.mem.set_f32(this + 0x44, length);
            e.mem.set_u8(valid, 7);
            let got = e.call(0x00d4_f0c0, &args![this, valid]).u32();
            assert_eq!(got, valid);
            assert_eq!(e.mem.u8(valid), want, "length {length}");
        }
    }

    #[test]
    fn the_constraint_type_is_8() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_f0e0, &args![0x1000u32]).u32(), 8);
    }

    #[test]
    fn the_constructor_sets_the_header_atoms_and_clears_the_vectors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let got = e.call(0x00d4_f0f0, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), STIFF_SPRING_VTABLE);
        assert_eq!(e.mem.u16(this + 0x10), 3);
        assert_eq!(e.mem.u16(this + 0x40), 6);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        for lane in 0..8 {
            assert_eq!(e.mem.u32(this + 0x20 + 4 * lane), 0);
        }
        // The words around the written fields are left alone.
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x14), 0xdead_beef);
    }
}
