//! `Havok SDK/hkphingelimitsdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpConstraintData::getConstraintInfoUtil` (Xbox PDB, cdecl `(atoms, atom size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkRotation::isOrthonormal` (Xbox PDB; `this` = the rotation, then `(result byte,
/// tolerance)`; answers the result pointer).
const IS_ORTHONORMAL: u32 = 0x00cb_2e70;
/// The orthonormality tolerance (a float).
const ORTHONORMAL_TOLERANCE: u32 = 0x0101_7d00;
/// The vtable of `hkpHingeLimitsData` and the two angle limits (floats) its constructor
/// starts with.
const HINGE_LIMITS_VTABLE: u32 = 0x010d_0be4;
const MIN_ANGLE_LIMIT: u32 = 0x0106_b648;
const MAX_ANGLE_LIMIT: u32 = 0x0102_b3c8;

// Translated from 00d52f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeLimitsData::getConstraintInfo` (Xbox PDB): `getConstraintInfoUtil` over the atoms
/// at `this + 0x10` (size 0x84) filling `info`.
pub fn hkp_hinge_limits_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x84u32, info],
    );
}

// Translated from 00d52f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeLimitsData::getRuntimeInfo` (Xbox PDB; the first stack argument, the "want
/// runtime" flag, is ignored): the runtime size 0x1c at `info + 0` and the solver result
/// count 3 at `info + 4`.
pub fn hkp_hinge_limits_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    info: Ptr,
) {
    e.mem.set_u32(info.addr() + 4, 3);
    e.mem.set_u32(info.addr(), 0x1c);
}

// Translated from 00d52f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeLimitsData::isValid` (Xbox PDB): the byte at `out` is 1 when both rotations (`this
/// + 0x20` and `this + 0x50`) are orthonormal within the tolerance (`hkRotation::isOrthonormal`,
/// the second one only when the first is), else 0.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_hinge_limits_data_is_valid(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let tolerance = x87_quiet_bits(e.mem.u32(ORTHONORMAL_TOLERANCE));
    let valid = e.with_stack(8, |e, results| {
        let first = e
            .call(
                IS_ORTHONORMAL,
                &args![this.addr() + 0x20, results.addr(), tolerance],
            )
            .u32();
        if e.mem.u8(first) == 0 {
            return false;
        }
        let tolerance = x87_quiet_bits(e.mem.u32(ORTHONORMAL_TOLERANCE));
        let second = e
            .call(
                IS_ORTHONORMAL,
                &args![this.addr() + 0x50, results.addr() + 1, tolerance],
            )
            .u32();
        e.mem.u8(second) != 0
    });
    e.mem.set_u8(out.addr(), u8::from(valid));
    out
}

// Translated from 00d52fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeLimitsData::getType` (Xbox PDB): the constraint type, `0x12`.
pub fn hkp_hinge_limits_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x12
}

// Translated from 00d52fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeLimitsData::hkpHingeLimitsData` (Xbox PDB, `ECX` = `this`): installs the vtable and
/// the defaults (user data 0, atoms 0x10 / 0x80 / 0x90 of types 4, 0xe and 0xc, two identity
/// rotations at `+0x20` and `+0x50` (zero, with 1.0 on the diagonal), angle limits from
/// `0106b648` / `0102b3c8` at `+0x84` / `+0x88` and 1.0 at `+0x8c`). Returns `this`.
pub fn hkp_hinge_limits_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, HINGE_LIMITS_VTABLE);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u16(me + 0x10, 4);
    e.mem.set_u8(me + 0x82, 1);
    e.mem.set_u16(me + 0x80, 0xe);
    e.mem.set_u16(me + 0x90, 0xc);
    for word in 0..24 {
        e.mem.set_u32(me + 0x20 + 4 * word, 0);
    }
    let one = 1.0f32.to_bits();
    for offset in [0x20, 0x34, 0x48, 0x50, 0x64, 0x78] {
        e.mem.set_u32(me + offset, one);
    }
    e.mem.set_u8(me + 0x92, 0);
    let low = x87_quiet_bits(e.mem.u32(MIN_ANGLE_LIMIT));
    e.mem.set_u8(me + 0x83, 0);
    e.mem.set_u32(me + 0x84, low);
    let high = x87_quiet_bits(e.mem.u32(MAX_ANGLE_LIMIT));
    e.mem.set_u32(me + 0x88, high);
    e.mem.set_u32(me + 0x8c, one);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_2f10,
            hkp_hinge_limits_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00d5_2f30,
            hkp_hinge_limits_data_get_runtime_info(Ptr, u32, Ptr)
        ),
        entry!(0x00d5_2f50, hkp_hinge_limits_data_is_valid(Ptr, Ptr) -> Ptr),
        entry!(0x00d5_2fb0, hkp_hinge_limits_data_get_type(Ptr) -> u32),
        entry!(0x00d5_2fc0, hkp_hinge_limits_data_construct(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    #[test]
    fn get_constraint_info_forwards_the_atoms_and_the_size() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(GET_CONSTRAINT_INFO_UTIL, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00d5_2f10, &args![0x4000u32, 0x5000u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x4010, 0x84, 0x5000]]);
    }

    #[test]
    fn get_runtime_info_answers_the_runtime_size_and_the_result_count() {
        let mut e = engine();
        let info = e.mem.alloc(8);
        e.call(0x00d5_2f30, &args![0x4000u32, 1u32, info]);
        assert_eq!(e.mem.u32(info), 0x1c);
        assert_eq!(e.mem.u32(info + 4), 3);
    }

    #[test]
    fn is_valid_needs_both_rotations_orthonormal() {
        let mut e = engine();
        e.mem.set_u32(ORTHONORMAL_TOLERANCE, 0x7f80_0001);
        let this = 0x4000u32;
        let out = e.mem.alloc(4);
        for (answers, expected, calls) in [
            ([true, true], 1u8, 2usize),
            ([true, false], 0, 2),
            ([false, true], 0, 1),
        ] {
            let seen = Rc::new(RefCell::new(Vec::new()));
            let sink = Rc::clone(&seen);
            e.register_double(IS_ORTHONORMAL, move |e, a| {
                let call = sink.borrow().len();
                sink.borrow_mut().push(a.to_vec());
                e.mem.set_u8(a[1], u8::from(answers[call]));
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
            e.mem.set_u8(out, 9);
            e.call(0x00d5_2f50, &args![this, out]);
            assert_eq!(e.mem.u8(out), expected);
            let seen = seen.borrow();
            assert_eq!(seen.len(), calls);
            assert_eq!(seen[0][0], this + 0x20);
            assert_eq!(seen[0][2], 0x7fc0_0001);
            if calls == 2 {
                assert_eq!(seen[1][0], this + 0x50);
                assert_eq!(seen[1][2], 0x7fc0_0001);
                assert_ne!(seen[0][1], seen[1][1]);
            }
        }
    }

    #[test]
    fn fn_00d52f50_answers_its_result_pointer() {
        let mut e = engine();
        e.register_double(IS_ORTHONORMAL, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(4);
        // EAX is `out` at both RETs (the hkBool result pointer the callers read through).
        assert_eq!(e.call(0x00d5_2f50, &args![0x4000u32, out]).u32(), out);
    }

    #[test]
    fn get_type_is_0x12() {
        let mut e = engine();
        assert_eq!(e.call(0x00d5_2fb0, &args![0x4000u32]).u32(), 0x12);
    }

    #[test]
    fn construct_installs_the_vtable_and_the_defaults() {
        let mut e = engine();
        e.mem.set_u32(MIN_ANGLE_LIMIT, 0xc049_0fdb);
        e.mem.set_u32(MAX_ANGLE_LIMIT, 0x7f80_0001);
        let this = e.mem.alloc(0xa0);
        for word in 0..40 {
            e.mem.set_u32(this + 4 * word, 0x5a5a_5a5a);
        }
        assert_eq!(e.call(0x00d5_2fc0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), HINGE_LIMITS_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0x10), 4);
        assert_eq!(e.mem.u8(this + 0x82), 1);
        assert_eq!(e.mem.u16(this + 0x80), 0xe);
        assert_eq!(e.mem.u16(this + 0x90), 0xc);
        assert_eq!(e.mem.u8(this + 0x83), 0);
        assert_eq!(e.mem.u8(this + 0x92), 0);
        for word in 0..24u32 {
            let offset = 0x20 + 4 * word;
            let diagonal = matches!(offset, 0x20 | 0x34 | 0x48 | 0x50 | 0x64 | 0x78);
            let expected = if diagonal { 0x3f80_0000 } else { 0 };
            assert_eq!(e.mem.u32(this + offset), expected, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x84), 0xc049_0fdb);
        assert_eq!(e.mem.u32(this + 0x88), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x8c), 0x3f80_0000);
    }
}
