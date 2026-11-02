//! `Havok SDK/hkprotationalconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl `(atoms, size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkRotation::isOrthonormal` (`00cb2e70`, `this` = the rotation, `(result byte, tolerance)`,
/// answers the pointer to the byte it wrote).
const IS_ORTHONORMAL: u32 = 0x00cb_2e70;
/// The `float` tolerance `isValid` passes (`FLD`, then `FSTP` to the stack).
const ORTHONORMAL_TOLERANCE: u32 = 0x0101_7d00;

// Translated from 00e3b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRotationalConstraintData::getConstraintInfo` (named in the engine map), thiscall with one
/// argument (`RET 4`): gives the atoms at `this + 0x10`, `0x74` bytes of them, and `info` to
/// `getConstraintInfoUtil`.
pub fn hkp_rotational_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr().wrapping_add(0x10), 0x74u32, info],
    );
}

// Translated from 00e3b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRotationalConstraintData::getRuntimeInfo` (named in the engine map), a virtual method:
/// stack words `(want_runtime, info)` (`RET 8`): with a nonzero first byte the info gets size `0x18` and `3`
/// solver results, without one both are zero. The first word is read as a byte.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_rotational_constraint_data_get_runtime_info(
    e: &mut Engine,
    _this: u32,
    want_runtime: u32,
    info: Ptr,
) {
    if want_runtime as u8 != 0 {
        e.mem.set_u32(info.addr() + 4, 3);
        e.mem.set_u32(info.addr(), 0x18);
    } else {
        e.mem.set_u32(info.addr() + 4, 0);
        e.mem.set_u32(info.addr(), 0);
    }
}

// Translated from 00e3b550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRotationalConstraintData::isValid` (named in the engine map), thiscall with one argument
/// (`RET 4`): writes into the byte at `valid` whether the rotation at `this + 0x20` and the one
/// at `this + 0x50` are both orthonormal within the tolerance. The second is only asked when the
/// first is. The answers are read through the pointers `isOrthonormal` returns.
/// Answers `valid`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_rotational_constraint_data_is_valid(e: &mut Engine, this: Ptr, valid: Ptr) -> Ptr {
    let tolerance = x87_quiet_bits(e.global::<u32>(ORTHONORMAL_TOLERANCE));
    let result = e.with_stack(2, |e, buffer| {
        let buffer = buffer.addr();
        let first = e
            .call(
                IS_ORTHONORMAL,
                &args![this.addr().wrapping_add(0x20), buffer, tolerance],
            )
            .u32();
        if e.mem.u8(first) != 0 {
            let second = e
                .call(
                    IS_ORTHONORMAL,
                    &args![this.addr().wrapping_add(0x50), buffer + 1, tolerance],
                )
                .u32();
            return e.mem.u8(second) != 0;
        }
        false
    });
    e.mem.set_u8(valid.addr(), u8::from(result));
    valid
}

// Translated from 00e3b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRotationalConstraintData::getType` (named in the engine map): the constraint type, `0x10`.
pub fn hkp_rotational_constraint_data_get_type(_e: &mut Engine) -> u32 {
    0x10
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e3_b500,
            hkp_rotational_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00e3_b520,
            hkp_rotational_constraint_data_get_runtime_info(u32, u32, Ptr)
        ),
        entry!(
            0x00e3_b550,
            hkp_rotational_constraint_data_is_valid(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00e3_b5b0, hkp_rotational_constraint_data_get_type() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    #[test]
    fn the_constraint_info_uses_the_atoms_at_0x10_00e3b500() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, GET_CONSTRAINT_INFO_UTIL, 0);
        e.call(0x00e3_b500, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls(&log, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![0x5010, 0x74, 0x6000]]
        );
    }

    #[test]
    fn the_runtime_info_depends_on_the_low_byte_00e3b520() {
        let mut e = engine();
        let info = e.mem.alloc(8);
        e.call(0x00e3_b520, &args![VCALL_THIS, 1u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x18, 3));
        // Only the low byte counts.
        e.call(0x00e3_b520, &args![VCALL_THIS, 0x100u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
        e.call(0x00e3_b520, &args![VCALL_THIS, 0xffu32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x18, 3));
        e.call(0x00e3_b520, &args![VCALL_THIS, 0u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
    }

    /// Answers from `isOrthonormal` doubles: each call writes the next of `answers` to its byte
    /// and answers the pointer to it; the log holds the (rotation, tolerance bits) per call.
    fn orthonormal_answers(e: &mut Engine, answers: Vec<u8>) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        let mut next = answers.into_iter();
        e.register_double(IS_ORTHONORMAL, move |e, a| {
            log.borrow_mut().push((a[0], a[2]));
            e.mem.set_u8(a[1], next.next().unwrap());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        seen
    }

    #[test]
    fn both_rotations_must_be_orthonormal_00e3b550() {
        for (answers, asked, valid) in [
            (vec![1u8, 1], 2, 1u8),
            (vec![1, 0], 2, 0),
            (vec![0], 1, 0),
            (vec![2, 0x80], 2, 1),
        ] {
            let mut e = engine();
            e.set_global(ORTHONORMAL_TOLERANCE, 0x3a83_126fu32);
            let seen = orthonormal_answers(&mut e, answers);
            let out = e.mem.alloc(4);
            e.mem.set_u32(out, 0xcdcd_cdcd);
            e.call(0x00e3_b550, &args![0x5000u32, out]);
            assert_eq!(e.mem.u8(out), valid);
            assert_eq!(e.mem.u8(out + 1), 0xcd);
            let seen = seen.borrow();
            assert_eq!(seen.len(), asked);
            assert_eq!(seen[0], (0x5020, 0x3a83_126f));
            if asked == 2 {
                assert_eq!(seen[1], (0x5050, 0x3a83_126f));
            }
        }
    }

    #[test]
    fn fn_00e3b550_answers_its_result_pointer() {
        let mut e = engine();
        e.set_global(ORTHONORMAL_TOLERANCE, 0x3a83_126fu32);
        orthonormal_answers(&mut e, vec![1u8, 1]);
        let out = e.mem.alloc(4);
        // EAX is `out` at both RETs (the hkBool result pointer the callers read through).
        assert_eq!(e.call(0x00e3_b550, &args![0x5000u32, out]).u32(), out);
    }

    #[test]
    fn the_tolerance_is_quieted_like_an_x87_load_00e3b550() {
        let mut e = engine();
        e.set_global(ORTHONORMAL_TOLERANCE, 0x7f80_0001u32);
        let seen = orthonormal_answers(&mut e, vec![0]);
        let out = e.mem.alloc(4);
        e.call(0x00e3_b550, &args![0x5000u32, out]);
        assert_eq!(seen.borrow()[0].1, 0x7fc0_0001);
    }

    #[test]
    fn the_type_is_0x10_00e3b5b0() {
        let mut e = engine();
        assert_eq!(e.call(0x00e3_b5b0, &args![]).u32(), 0x10);
    }
}
