//! `Havok SDK/hkppulleyconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl `(atoms, size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;

// Translated from 00deef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPulleyConstraintData::getConstraintInfo`: the shared `getConstraintInfoUtil` on the
/// atoms at `this + 0x10`, `0x70` bytes of them, filling `info`.
pub fn pulley_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x70u32, info],
    );
}

// Translated from 00deef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPulleyConstraintData::getRuntimeInfo`: `(size, count)` is written to `info` as `(8, 1)`
/// when `flag` (the low byte of the first stack word) is non-zero and as `(0, 0)` otherwise.
pub fn pulley_get_runtime_info(e: &mut Engine, _this: Ptr, flag: u8, info: Ptr) {
    let (size, count) = if flag != 0 { (8, 1) } else { (0, 0) };
    e.mem.set_u32(info.addr() + 4, count);
    e.mem.set_u32(info.addr(), size);
}

// Translated from 00deef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPulleyConstraintData::isValid`: the byte at `valid` becomes 1 when both floats at `+0x70`
/// and `+0x74` are above zero (an unordered value, or zero, is not), else 0. Returns `valid`.
pub fn pulley_is_valid(e: &mut Engine, this: Ptr, valid: u32) -> u32 {
    let data = this.addr();
    let above_zero = |e: &Engine, offset: u32| 0.0 < f64::from(x87_quiet(e.mem.f32(data + offset)));
    let ok = above_zero(e, 0x70) && above_zero(e, 0x74);
    e.mem.set_u8(valid, u8::from(ok));
    valid
}

// Translated from 00deefc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPulleyConstraintData::getType`: the constraint type, 15.
pub fn pulley_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xf
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_ef40, pulley_get_constraint_info(Ptr, u32)),
        entry!(0x00de_ef60, pulley_get_runtime_info(Ptr, u8, Ptr)),
        entry!(0x00de_ef90, pulley_is_valid(Ptr, u32) -> u32),
        entry!(0x00de_efc0, pulley_get_type(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod a05_pulley_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn constraint_info_runtime_info_and_type() {
        let mut e = Engine::new();
        e.register(GET_CONSTRAINT_INFO_UTIL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let data = e.mem.alloc(0x80);
        let info = e.mem.alloc(0x10);
        e.call(0x00de_ef40, &args![data, info]);
        assert_eq!(
            calls_to(&e, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![data + 0x10, 0x70, info]]
        );
        e.call(0x00de_ef60, &args![data, 1u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (8, 1));
        // Only the low byte of the flag word counts.
        e.call(0x00de_ef60, &args![data, 0x100u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
        assert_eq!(e.call(0x00de_efc0, &args![data]).u32(), 0xf);
    }

    #[test]
    fn valid_needs_both_values_above_zero() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x80);
        let valid = e.mem.alloc(8);
        let cases = [
            (1.0, 2.0, 1),
            (0.0, 2.0, 0),
            (1.0, 0.0, 0),
            (-1.0, 2.0, 0),
            (1.0, -0.5, 0),
            (f32::NAN, 2.0, 0),
            (1.0, f32::NAN, 0),
            (f32::MIN_POSITIVE, f32::INFINITY, 1),
        ];
        for (first, second, expected) in cases {
            e.mem.set_f32(data + 0x70, first);
            e.mem.set_f32(data + 0x74, second);
            e.mem.set_u8(valid, 0xaa);
            let got = e.call(0x00de_ef90, &args![data, valid]).u32();
            assert_eq!(got, valid);
            assert_eq!(e.mem.u8(valid), expected, "{first} {second}");
        }
    }
}
